use core::{
    fmt,
    pin::Pin,
    task::{Context, Poll},
};
use std::{
    ffi::{OsStr, OsString},
    io,
    path::{Path, PathBuf},
    process::Stdio,
};

use tokio::{
    io::{AsyncRead, AsyncReadExt, ReadBuf},
    process::Command,
    sync::mpsc,
};

use crate::voice::{InputError, codec::FRAME_BYTES, ogg::OggOpus, opus};

const CHUNK: usize = 16 * 1024;

const MAGIC: &[u8; 4] = b"OggS";

const STDERR_TAIL: usize = 4 * 1024;

const STDERR_LINES: usize = 4;

const FFMPEG: &str = "ffmpeg";

const PREROLL: u64 = 19_200;

pub struct Input {
    source: Source,
    again: Option<Again>,
}

pub(crate) enum Source {
    Ogg {
        reader: Box<dyn AsyncRead + Send + Unpin>,
        ogg: Box<OggOpus>,

        first: Option<Vec<u8>>,
    },
    OggFile(PathBuf),
    Pcm(Box<dyn AsyncRead + Send + Unpin>),
    Ffmpeg {
        program: OsString,
        source: OsString,
    },
}

#[derive(Clone)]
pub(crate) enum Again {
    OggFile(PathBuf),
    Ffmpeg { program: OsString, source: OsString },
}

impl Again {
    pub(crate) fn source(&self) -> Source {
        match self {
            Self::OggFile(path) => Source::OggFile(path.clone()),
            Self::Ffmpeg { program, source } => Source::Ffmpeg {
                program: program.clone(),
                source: source.clone(),
            },
        }
    }
}

impl fmt::Debug for Input {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(match (&self.source, &self.again) {
            (Source::Ogg { .. } | Source::OggFile(_), Some(_)) => "Input::OggFile",
            (Source::Ogg { .. } | Source::OggFile(_), None) => "Input::Ogg",
            (Source::Pcm(_), _) => "Input::Pcm",
            (Source::Ffmpeg { .. }, _) => "Input::Ffmpeg",
        })
    }
}

#[derive(Debug)]
pub(crate) enum Frame {
    Opus { data: Vec<u8>, samples: u32 },

    Pcm(Vec<u8>),
}

impl Frame {
    pub(crate) fn samples(&self) -> u32 {
        match self {
            Self::Opus { samples, .. } => *samples,
            Self::Pcm(_) => 960,
        }
    }
}

#[derive(Debug, Default)]
pub(crate) struct History {
    packets: std::collections::VecDeque<(Vec<u8>, u32)>,
    samples: u64,
}

impl History {
    pub(crate) fn push(&mut self, data: Vec<u8>, samples: u32) {
        self.samples += u64::from(samples);
        self.packets.push_back((data, samples));
        while let Some((_, oldest)) = self.packets.front()
            && self.samples - u64::from(*oldest) >= PREROLL
        {
            self.samples -= u64::from(*oldest);
            self.packets.pop_front();
        }
    }

    pub(crate) fn clear(&mut self) {
        self.packets.clear();
        self.samples = 0;
    }

    pub(crate) fn is_empty(&self) -> bool {
        self.packets.is_empty()
    }

    #[cfg(test)]
    pub(crate) fn len(&self) -> usize {
        self.packets.len()
    }

    pub(crate) fn packets(&self) -> impl Iterator<Item = &[u8]> {
        self.packets.iter().map(|(data, _)| &data[..])
    }
}

#[derive(Debug)]
pub(crate) enum Chunk {
    Preroll(History),

    Start(u64),
    Frame(Frame),
}

pub(crate) type Chunks = mpsc::Sender<Result<Chunk, InputError>>;

impl Input {
    #[must_use]
    pub fn ogg(reader: impl AsyncRead + Send + Unpin + 'static) -> Self {
        Self {
            source: Source::Ogg {
                reader: Box::new(reader),
                ogg: Box::new(OggOpus::new()),
                first: None,
            },
            again: None,
        }
    }

    pub async fn ogg_file(path: impl AsRef<Path>) -> Result<Self, InputError> {
        let path = path.as_ref();
        let mut reader = tokio::fs::File::open(path).await?;
        let mut ogg = Box::new(OggOpus::new());
        let mut chunk = vec![0; CHUNK];
        let first = loop {
            if let Some(packet) = ogg.packet()? {
                break Some(packet.to_vec());
            }
            let read = reader.read(&mut chunk).await?;
            if read == 0 {
                ogg.finish();
                break ogg.packet()?.map(<[u8]>::to_vec);
            }
            ogg.push(&chunk[..read]);
        };
        Ok(Self {
            source: Source::Ogg {
                reader: Box::new(reader),
                ogg,
                first,
            },
            again: Some(Again::OggFile(path.to_path_buf())),
        })
    }

    #[must_use]
    pub fn pcm(reader: impl AsyncRead + Send + Unpin + 'static) -> Self {
        Self {
            source: Source::Pcm(Box::new(reader)),
            again: None,
        }
    }

    #[must_use]
    pub fn ffmpeg(source: impl AsRef<OsStr>) -> Self {
        let program = OsString::from(FFMPEG);
        let source = source.as_ref().to_os_string();
        Self {
            source: Source::Ffmpeg {
                program: program.clone(),
                source: source.clone(),
            },
            again: Some(Again::Ffmpeg { program, source }),
        }
    }

    pub(crate) fn into_parts(self) -> (Source, Option<Again>) {
        (self.source, self.again)
    }

    pub(crate) fn seekable(&self) -> bool {
        self.again.is_some()
    }

    #[cfg(test)]
    pub(crate) fn ffmpeg_with(program: &str, source: impl AsRef<OsStr>) -> Self {
        let program = OsString::from(program);
        let source = source.as_ref().to_os_string();
        Self {
            source: Source::Ffmpeg {
                program: program.clone(),
                source: source.clone(),
            },
            again: Some(Again::Ffmpeg { program, source }),
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[non_exhaustive]
pub enum ProbedKind {
    Ogg,
    Raw,
}

struct Chained<R> {
    prefix: Vec<u8>,
    pos: usize,
    reader: R,
}

impl<R: AsyncRead + Unpin> AsyncRead for Chained<R> {
    fn poll_read(
        mut self: Pin<&mut Self>,
        cx: &mut Context<'_>,
        buf: &mut ReadBuf<'_>,
    ) -> Poll<io::Result<()>> {
        if self.pos < self.prefix.len() {
            let rest = &self.prefix[self.pos..];
            let take = rest.len().min(buf.remaining());
            buf.put_slice(&rest[..take]);
            self.pos += take;
            return Poll::Ready(Ok(()));
        }
        Pin::new(&mut self.reader).poll_read(cx, buf)
    }
}

pub async fn probe<R>(reader: R) -> Result<(ProbedKind, Input), InputError>
where
    R: AsyncRead + Send + Unpin + 'static,
{
    let mut reader = reader;
    let mut prefix = Vec::with_capacity(MAGIC.len());
    let mut byte = [0; 1];
    while prefix.len() < MAGIC.len() {
        match reader.read(&mut byte).await {
            Ok(0) => break,
            Ok(_) => prefix.push(byte[0]),
            Err(error) => return Err(InputError::Io(error)),
        }
    }
    let chained = Chained {
        prefix,
        pos: 0,
        reader,
    };
    if chained.prefix == MAGIC {
        Ok((ProbedKind::Ogg, Input::ogg(chained)))
    } else {
        Ok((ProbedKind::Raw, Input::pcm(chained)))
    }
}

pub(crate) async fn read(source: Source, start: u64, chunks: Chunks) {
    let result = match source {
        Source::Ogg { reader, ogg, first } => read_ogg(reader, ogg, first, start, &chunks).await,
        Source::OggFile(path) => match tokio::fs::File::open(path).await {
            Ok(file) => read_ogg(Box::new(file), Box::default(), None, start, &chunks).await,
            Err(error) => Err(error.into()),
        },
        Source::Pcm(reader) => read_pcm(reader, None, &chunks).await.map(drop),
        Source::Ffmpeg { program, source } => ffmpeg(&program, &source, start, &chunks).await,
    };
    if let Err(error) = result {
        let _ = chunks.send(Err(error)).await;
    }
}

struct Skip {
    start: u64,
    at: u64,
    preroll: History,
}

impl Skip {
    const fn new(start: u64) -> Self {
        Self {
            start,
            at: 0,
            preroll: History {
                packets: std::collections::VecDeque::new(),
                samples: 0,
            },
        }
    }

    async fn keep(&mut self, data: &[u8], samples: u32, chunks: &Chunks) -> Option<bool> {
        if self.start == 0 {
            return Some(true);
        }
        if self.at + u64::from(samples) <= self.start {
            self.at += u64::from(samples);
            self.preroll.push(data.to_vec(), samples);
            return Some(false);
        }
        let at = self.at;
        self.start = 0;
        let preroll = core::mem::take(&mut self.preroll);
        if !preroll.is_empty() {
            chunks.send(Ok(Chunk::Preroll(preroll))).await.ok()?;
        }
        chunks.send(Ok(Chunk::Start(at))).await.ok()?;
        Some(true)
    }
}

async fn read_ogg(
    mut reader: Box<dyn AsyncRead + Send + Unpin>,
    mut ogg: Box<OggOpus>,
    first: Option<Vec<u8>>,
    start: u64,
    chunks: &Chunks,
) -> Result<(), InputError> {
    let mut skip = Skip::new(start);
    if let Some(data) = first
        && !forward(data, &mut skip, chunks).await
    {
        return Ok(());
    }
    let mut chunk = vec![0; CHUNK];
    let mut ended = false;
    loop {
        while let Some(data) = ogg.packet()? {
            let data = data.to_vec();
            if !forward(data, &mut skip, chunks).await {
                return Ok(());
            }
        }
        if ended {
            return Ok(());
        }
        let read = reader.read(&mut chunk).await?;
        if read == 0 {
            ogg.finish();
            ended = true;
        } else {
            ogg.push(&chunk[..read]);
        }
    }
}

async fn forward(data: Vec<u8>, skip: &mut Skip, chunks: &Chunks) -> bool {
    let Some(samples) = opus::packet_samples(&data) else {
        tracing::debug!(len = data.len(), "skipping a packet that is not Opus");
        return true;
    };
    match skip.keep(&data, samples, chunks).await {
        Some(true) => chunks
            .send(Ok(Chunk::Frame(Frame::Opus { data, samples })))
            .await
            .is_ok(),
        Some(false) => true,
        None => false,
    }
}

async fn read_pcm(
    mut reader: impl AsyncRead + Unpin,
    mut start: Option<u64>,
    chunks: &Chunks,
) -> Result<bool, InputError> {
    let mut frame = vec![0; FRAME_BYTES];
    let mut filled = 0;
    loop {
        let read = reader.read(&mut frame[filled..]).await?;
        filled += read;
        if read != 0 && filled < FRAME_BYTES {
            continue;
        }
        if filled == 0 {
            return Ok(true);
        }

        frame[filled..].fill(0);
        if let Some(start) = start.take()
            && chunks.send(Ok(Chunk::Start(start))).await.is_err()
        {
            return Ok(false);
        }
        let full = core::mem::replace(&mut frame, vec![0; FRAME_BYTES]);
        if chunks
            .send(Ok(Chunk::Frame(Frame::Pcm(full))))
            .await
            .is_err()
        {
            return Ok(false);
        }
        if read == 0 {
            return Ok(true);
        }
        filled = 0;
    }
}

async fn ffmpeg(
    program: &OsStr,
    source: &OsStr,
    start: u64,
    chunks: &Chunks,
) -> Result<(), InputError> {
    let mut command = Command::new(program);
    command.args(["-nostdin", "-hide_banner", "-loglevel", "error"]);
    if start > 0 {
        command.arg("-ss").arg(seconds(start));
    }
    if is_http(source) {
        command.args([
            "-reconnect",
            "1",
            "-reconnect_streamed",
            "1",
            "-reconnect_delay_max",
            "5",
        ]);
    }
    command
        .arg("-i")
        .arg(source)
        .args(["-vn", "-f", "s16le", "-ar", "48000", "-ac", "2", "pipe:1"])
        .stdin(Stdio::null())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .kill_on_drop(true);
    let mut child = command.spawn().map_err(|error| match error.kind() {
        io::ErrorKind::NotFound => InputError::FfmpegMissing,
        _ => InputError::Io(error),
    })?;
    let stdout = child
        .stdout
        .take()
        .ok_or_else(|| io::Error::other("FFmpeg's standard output is not piped"))?;
    let stderr = child.stderr.take().map(|stderr| tokio::spawn(tail(stderr)));
    if !read_pcm(stdout, (start > 0).then_some(start), chunks).await? {
        return Ok(());
    }
    let status = child.wait().await?;
    if status.success() {
        return Ok(());
    }
    let message = match stderr {
        Some(task) => task.await.unwrap_or_default(),
        None => String::new(),
    };
    Err(InputError::Ffmpeg {
        code: status.code(),
        message,
    })
}

async fn tail(mut stderr: impl AsyncRead + Unpin) -> String {
    let mut kept = Vec::new();
    let mut chunk = [0; 1024];
    while let Ok(read @ 1..) = stderr.read(&mut chunk).await {
        kept.extend_from_slice(&chunk[..read]);
        if kept.len() > STDERR_TAIL {
            kept.drain(..kept.len() - STDERR_TAIL);
        }
    }
    let text = String::from_utf8_lossy(&kept);
    let lines: Vec<&str> = text
        .lines()
        .map(str::trim)
        .filter(|line| !line.is_empty())
        .collect();
    lines[lines.len().saturating_sub(STDERR_LINES)..].join("\n")
}

fn seconds(samples: u64) -> String {
    format!(
        "{}.{:06}",
        samples / 48_000,
        samples % 48_000 * 1_000_000 / 48_000
    )
}

fn is_http(source: &OsStr) -> bool {
    source.to_str().is_some_and(|source| {
        let source = source.trim_start().to_ascii_lowercase();
        source.starts_with("http://") || source.starts_with("https://")
    })
}

#[cfg(test)]
mod tests {
    use std::path::PathBuf;

    use tokio::sync::mpsc;

    use super::{Chunk, Frame, Input, Source, is_http, read, seconds};
    use crate::voice::{InputError, codec::FRAME_BYTES};

    const SINE: &[u8] = include_bytes!("testdata/sine.opus");

    async fn chunks(input: Input, start: u64) -> Vec<Result<Chunk, InputError>> {
        let (tx, mut rx) = mpsc::channel(4);
        let (source, _) = input.into_parts();
        let reader = tokio::spawn(read(source, start, tx));
        let mut chunks = Vec::new();
        while let Some(chunk) = rx.recv().await {
            chunks.push(chunk);
        }
        reader.await.unwrap();
        chunks
    }

    fn frames(chunks: &[Result<Chunk, InputError>]) -> Vec<&Frame> {
        chunks
            .iter()
            .filter_map(|chunk| match chunk {
                Ok(Chunk::Frame(frame)) => Some(frame),
                _ => None,
            })
            .collect()
    }

    fn dir(name: &str) -> PathBuf {
        let dir = std::env::temp_dir().join(format!("discord-voice-{name}-{}", std::process::id()));
        std::fs::create_dir_all(&dir).unwrap();
        dir
    }

    #[tokio::test]
    async fn probe_routes_by_magic() {
        use super::{ProbedKind, probe};

        const RAW: &[u8] = b"definitely not ogg, plain bytes follow....";

        assert_eq!(&SINE[..4], b"OggS");
        let (kind, input) = probe(SINE).await.unwrap();
        assert_eq!(kind, ProbedKind::Ogg);
        let read = chunks(input, 0).await;
        assert_eq!(frames(&read).len(), 51);

        let (kind, input) = probe(RAW).await.unwrap();
        assert_eq!(kind, ProbedKind::Raw);
        let read = chunks(input, 0).await;
        let mut heard = Vec::new();
        for chunk in &read {
            if let Ok(Chunk::Frame(Frame::Pcm(bytes))) = chunk {
                heard.extend_from_slice(bytes);
            }
        }
        assert!(heard.starts_with(RAW));
    }

    #[tokio::test]
    async fn reads_every_packet_of_a_stream() {
        let chunks = chunks(Input::ogg(SINE), 0).await;
        let frames = frames(&chunks);
        assert_eq!(frames.len(), 51);
        assert_eq!(chunks.len(), 51, "no start, no error");
        assert!(frames.iter().all(|frame| frame.samples() == 960));
        assert!(!Input::ogg(SINE).seekable());
    }

    #[tokio::test]
    async fn files_are_checked_when_opened_and_read_again_from_a_place() {
        let dir = dir("input");
        let good = dir.join("sine.opus");
        std::fs::write(&good, SINE).unwrap();
        let input = Input::ogg_file(&good).await.unwrap();
        assert!(input.seekable());
        assert_eq!(format!("{input:?}"), "Input::OggFile");
        let read = chunks(input, 0).await;
        assert_eq!(
            frames(&read).len(),
            51,
            "the packet read while checking is kept"
        );

        let (_, again) = Input::ogg_file(&good).await.unwrap().into_parts();
        for (start, at, left) in [(24_000, 24_000, 26), (24_010, 24_000, 26), (60_000, 0, 0)] {
            let read = chunks(
                Input {
                    source: again.as_ref().unwrap().source(),
                    again: None,
                },
                start,
            )
            .await;
            if left == 0 {
                assert!(read.is_empty(), "past the end");
                continue;
            }

            assert!(matches!(&read[0], Ok(Chunk::Preroll(packets)) if packets.len() == 20));
            assert!(matches!(read[1], Ok(Chunk::Start(position)) if position == at));
            assert_eq!(frames(&read).len(), left);
        }
        let text = dir.join("text.opus");
        std::fs::write(&text, b"not audio at all").unwrap();
        assert!(matches!(
            Input::ogg_file(&text).await,
            Err(InputError::NotOgg)
        ));
        assert!(matches!(
            Input::ogg_file(dir.join("missing.opus")).await,
            Err(InputError::Io(_))
        ));
        let read = chunks(
            Input {
                source: Source::OggFile(dir.join("missing.opus")),
                again: None,
            },
            0,
        )
        .await;
        assert!(matches!(read[..], [Err(InputError::Io(_))]));
        std::fs::remove_dir_all(&dir).unwrap();
    }

    #[tokio::test]
    async fn readers_report_inputs_that_are_not_ogg_opus() {
        let chunks = chunks(Input::ogg(&b"plain text"[..]), 0).await;
        assert!(matches!(chunks[..], [Err(InputError::NotOgg)]));
    }

    #[tokio::test]
    async fn reading_stops_when_the_player_does() {
        let (tx, mut rx) = mpsc::channel(1);
        let (source, _) = Input::ogg(SINE).into_parts();
        let reader = tokio::spawn(read(source, 0, tx));
        rx.recv().await.unwrap().unwrap();
        drop(rx);
        reader.await.unwrap();
    }

    #[tokio::test]
    async fn pcm_comes_in_frames_of_20_ms() {
        let bytes: Vec<u8> = (0..FRAME_BYTES * 5 / 2)
            .map(|i| u8::try_from(i % 251).unwrap())
            .collect();
        let (client, mut server) = tokio::io::duplex(64);
        let writer = {
            let bytes = bytes.clone();
            tokio::spawn(async move {
                use tokio::io::AsyncWriteExt;
                for piece in bytes.chunks(1000) {
                    server.write_all(piece).await.unwrap();
                }
            })
        };
        let read = chunks(Input::pcm(client), 0).await;
        writer.await.unwrap();
        let frames = frames(&read);
        assert_eq!(frames.len(), 3);
        let Frame::Pcm(last) = frames[2] else {
            panic!("{:?}", frames[2])
        };
        assert_eq!(last.len(), FRAME_BYTES);
        assert_eq!(last[..FRAME_BYTES / 2], bytes[FRAME_BYTES * 2..]);
        assert!(last[FRAME_BYTES / 2..].iter().all(|&byte| byte == 0));
        assert!(chunks(Input::pcm(&b""[..]), 0).await.is_empty());
        assert!(!Input::pcm(&b""[..]).seekable());
    }

    #[test]
    fn ffmpeg_arguments_are_formatted() {
        assert_eq!(seconds(0), "0.000000");
        assert_eq!(seconds(72_000), "1.500000");
        assert_eq!(seconds(48_001), "1.000020");
        assert!(is_http("https://example.com/a.mp3".as_ref()));
        assert!(is_http(" HTTP://example.com".as_ref()));
        assert!(!is_http("music/https.mp3".as_ref()));
    }

    #[tokio::test]
    async fn a_missing_ffmpeg_is_reported() {
        let input = Input::ffmpeg_with("discord-voice-no-such-program", "song.mp3");
        assert!(input.seekable());
        let read = chunks(input, 0).await;
        assert!(matches!(read[..], [Err(InputError::FfmpegMissing)]));
    }

    fn have_ffmpeg() -> bool {
        let found = std::process::Command::new("ffmpeg")
            .arg("-version")
            .output()
            .is_ok_and(|output| output.status.success());
        if !found {
            eprintln!("ffmpeg is not installed: skipped");
        }
        found
    }

    fn wav(path: &std::path::Path, seconds: u32) {
        let samples = 44_100 * seconds;
        let mut file = Vec::new();
        file.extend_from_slice(b"RIFF");
        file.extend_from_slice(&(36 + samples * 2).to_le_bytes());
        file.extend_from_slice(b"WAVEfmt ");
        file.extend_from_slice(&16_u32.to_le_bytes());
        file.extend_from_slice(&1_u16.to_le_bytes());
        file.extend_from_slice(&1_u16.to_le_bytes());
        file.extend_from_slice(&44_100_u32.to_le_bytes());
        file.extend_from_slice(&88_200_u32.to_le_bytes());
        file.extend_from_slice(&2_u16.to_le_bytes());
        file.extend_from_slice(&16_u16.to_le_bytes());
        file.extend_from_slice(b"data");
        file.extend_from_slice(&(samples * 2).to_le_bytes());
        for i in 0..samples {
            let value = (f64::from(i) * 440.0 * core::f64::consts::TAU / 44_100.0).sin() * 8000.0;
            #[expect(clippy::cast_possible_truncation, reason = "within ±8000")]
            file.extend_from_slice(&(value as i16).to_le_bytes());
        }
        std::fs::write(path, file).unwrap();
    }

    #[tokio::test]
    async fn ffmpeg_decodes_and_seeks() {
        if !have_ffmpeg() {
            return;
        }
        let dir = dir("ffmpeg");
        let path = dir.join("tone.wav");
        wav(&path, 1);

        let read = chunks(Input::ffmpeg(&path), 0).await;
        assert_eq!(frames(&read).len(), 50, "{read:?}");
        assert!(read.iter().all(Result::is_ok));

        let read = chunks(Input::ffmpeg(&path), 24_000).await;
        assert!(matches!(read[0], Ok(Chunk::Start(24_000))));
        assert_eq!(frames(&read).len(), 25);

        let read = chunks(Input::ffmpeg(dir.join("missing.mp3")), 0).await;
        match &read[..] {
            [Err(InputError::Ffmpeg { code, message })] => {
                assert_ne!(*code, Some(0));
                assert!(message.contains("missing.mp3"), "{message}");
            }
            other => panic!("{other:?}"),
        }
        std::fs::remove_dir_all(&dir).unwrap();
    }

    #[tokio::test(flavor = "multi_thread", worker_threads = 2)]
    #[ignore = "a measurement; run it in release"]
    async fn throughput() {
        use std::time::Instant;

        use crate::voice::ogg::tests::{audio, opus, stream};

        let packets: u32 = 3600 * 50;
        let audio: Vec<Vec<u8>> = (0..packets)
            .map(|number| {
                let mut packet = vec![0xFC];
                packet.extend(audio(u8::try_from(number % 256).unwrap(), 159));
                packet
            })
            .collect();
        let file = stream(1, 0, &opus(&audio), 50);
        let dir = std::env::temp_dir().join(format!("discord-voice-read-{}", std::process::id()));
        std::fs::create_dir_all(&dir).unwrap();
        let path = dir.join("hour.opus");
        std::fs::write(&path, &file).unwrap();
        let drain = |input: Input| async move {
            let (source, _) = input.into_parts();

            let (tx, mut rx) = mpsc::channel(50);
            tokio::spawn(read(source, 0, tx));
            let mut frames = 0_u32;
            while let Some(chunk) = rx.recv().await {
                if let Ok(Chunk::Frame(_)) = chunk {
                    frames += 1;
                }
            }
            frames
        };
        let mut best = f64::MAX;
        for _ in 0..3 {
            let started = Instant::now();
            let frames = drain(Input::ogg_file(&path).await.unwrap()).await;
            best = best.min(started.elapsed().as_secs_f64());
            assert_eq!(frames, packets);
        }
        println!(
            "an hour of Ogg/Opus from a file: {:.0} ms, {:.2} M packets/s, {:.0}x real time",
            best * 1e3,
            f64::from(packets) / best / 1e6,
            3600.0 / best
        );
        std::fs::remove_dir_all(&dir).unwrap();

        let minutes = 10;
        let pcm = vec![0_u8; minutes * 60 * 48_000 * 4];
        let frames = u32::try_from(minutes * 60 * 50).unwrap();
        let mut best = f64::MAX;
        for _ in 0..3 {
            let started = Instant::now();
            let read = drain(Input::pcm(std::io::Cursor::new(pcm.clone()))).await;
            best = best.min(started.elapsed().as_secs_f64());
            assert_eq!(read, frames);
        }
        println!(
            "ten minutes of PCM from memory: {:.0} ms, {:.2} M frames/s, {:.0}x real time",
            best * 1e3,
            f64::from(frames) / best / 1e6,
            600.0 / best
        );
    }
}
