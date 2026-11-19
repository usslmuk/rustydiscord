use core::time::Duration;

use rustydiscord_model::id::ChannelId;
use tokio::time::timeout;

use super::tests::{
    CHANNEL, SILENCE, SINE, audio, guild, joined, manager, silence_follows, sine_packets,
};
use crate::{
    Input, InputError, JoinOptions, Loop, PlayState, TrackEnd, TrackError, TrackEvent, TrackEvents,
    TrackHandle,
    codec::{Decoder, MAX_PACKET_VALUES},
    connection::tests::{KEY, Mock, STEP, recv_op},
    opus::packet_samples,
};

fn tone(frames: usize, level: f32) -> Vec<u8> {
    (0..frames * 960)
        .flat_map(|i| {
            #[expect(clippy::cast_precision_loss, reason = "small sample indices")]
            let angle = i as f32 * 440.0 * core::f32::consts::TAU / 48_000.0;
            #[expect(clippy::cast_possible_truncation, reason = "within the 16-bit range")]
            let value = (angle.sin() * level * 32_767.0) as i16;
            let [low, high] = value.to_le_bytes();
            [low, high, low, high]
        })
        .collect()
}

fn endless() -> Input {
    Input::pcm(tokio::io::repeat(0))
}

async fn next(events: &mut TrackEvents) -> TrackEvent {
    timeout(STEP, events.next())
        .await
        .expect("no track event")
        .expect("the track's events ended")
}

async fn ended(events: &mut TrackEvents) -> TrackEnd {
    loop {
        if let TrackEvent::Ended(reason) = next(events).await {
            assert!(events.next().await.is_none(), "the stream ends after Ended");
            return reason;
        }
    }
}

fn rms(pcm: &[f32]) -> f32 {
    #[expect(clippy::cast_precision_loss, reason = "short buffers")]
    let len = pcm.len() as f32;
    (pcm.iter().map(|value| value * value).sum::<f32>() / len).sqrt()
}

async fn loudness(mock: &Mock, skip: usize) -> f32 {
    let mut decoder = Decoder::new().unwrap();
    let mut pcm = vec![0.0; MAX_PACKET_VALUES];
    let mut last = Vec::new();
    for n in 0..skip + 5 {
        let (payload, _, _) = audio(mock, KEY).await;
        let samples = decoder.decode(&payload, &mut pcm).unwrap();
        if n >= skip {
            last.extend_from_slice(&pcm[..samples * 2]);
        }
    }
    rms(&last)
}

#[tokio::test]
async fn queued_tracks_play_in_turn() {
    let mock = Mock::new().await;
    let (manager, mut commands) = manager();
    let (call, _events, mut server) = joined(&manager, &mut commands, &mock).await;
    let first = call.enqueue(Input::pcm(std::io::Cursor::new(tone(5, 0.5))));
    let second = call.enqueue(Input::pcm(std::io::Cursor::new(tone(4, 0.5))));
    let (mut first_events, mut second_events) = (first.events(), second.events());
    let queue = call.queue();
    assert_eq!(queue.len(), 2);
    assert_eq!(queue.current(), Some(first.clone()));
    assert_eq!(second.info().state, PlayState::Queued);

    let (_, sequence, timestamp) = audio(&mock, KEY).await;
    assert_eq!(recv_op(&mut server, 5).await["d"]["speaking"], 1);
    for n in 1..9_u16 {
        let (payload, seq, ts) = audio(&mock, KEY).await;
        assert_ne!(payload, SILENCE);
        assert_eq!(packet_samples(&payload), Some(960));
        assert_eq!(seq, sequence.wrapping_add(n), "no gap between the tracks");
        assert_eq!(ts, timestamp.wrapping_add(960 * u32::from(n)));
    }
    silence_follows(&mock, &mut server).await;
    assert!(matches!(next(&mut first_events).await, TrackEvent::Playing));
    assert!(matches!(ended(&mut first_events).await, TrackEnd::Finished));
    assert!(matches!(
        next(&mut second_events).await,
        TrackEvent::Playing
    ));
    assert!(matches!(
        ended(&mut second_events).await,
        TrackEnd::Finished
    ));
    assert!(queue.is_empty());
    let info = first.info();
    assert_eq!(info.state, PlayState::Ended);
    assert_eq!(info.position, Duration::from_millis(100));
    assert_eq!(first.play(), Err(TrackError::Ended));
}

#[tokio::test]
async fn skip_clear_pause_and_stop_drive_the_queue() {
    let mock = Mock::new().await;
    let (manager, mut commands) = manager();
    let (call, _events, _server) = joined(&manager, &mut commands, &mock).await;
    let queue = call.queue();
    assert!(!queue.skip(), "nothing to skip");
    let tracks: Vec<TrackHandle> = (0..4).map(|_| call.enqueue(endless())).collect();
    let mut events: Vec<TrackEvents> = tracks.iter().map(TrackHandle::events).collect();
    assert!(matches!(next(&mut events[0]).await, TrackEvent::Playing));

    assert!(queue.skip());
    assert!(matches!(ended(&mut events[0]).await, TrackEnd::Skipped));
    assert_eq!(queue.current(), Some(tracks[1].clone()));
    assert!(matches!(next(&mut events[1]).await, TrackEvent::Playing));

    tracks[2].stop().unwrap();
    assert!(matches!(ended(&mut events[2]).await, TrackEnd::Stopped));
    assert_eq!(queue.len(), 2);

    tracks[1].pause().unwrap();
    assert!(matches!(next(&mut events[1]).await, TrackEvent::Paused));
    assert_eq!(tracks[1].info().state, PlayState::Paused);

    tokio::time::sleep(Duration::from_millis(60)).await;
    let position = tracks[1].info().position;
    tokio::time::sleep(Duration::from_millis(100)).await;
    assert_eq!(
        tracks[1].info().position,
        position,
        "a paused track keeps its place"
    );
    tracks[1].play().unwrap();
    assert!(matches!(next(&mut events[1]).await, TrackEvent::Playing));
    assert_eq!(tracks[1].info().state, PlayState::Playing);

    queue.clear();
    assert!(matches!(ended(&mut events[3]).await, TrackEnd::Stopped));
    assert_eq!(queue.len(), 1, "the current track goes on");
    tracks[1].stop().unwrap();
    assert!(matches!(ended(&mut events[1]).await, TrackEnd::Stopped));
    assert!(queue.is_empty());
    assert_eq!(tracks[1].stop(), Err(TrackError::Ended));

    let paused = call.enqueue(endless());
    paused.pause().unwrap();
    let mut paused_events = paused.events();
    let other = call.enqueue(endless());
    tokio::time::sleep(Duration::from_millis(60)).await;
    assert_eq!(paused.info().state, PlayState::Paused, "it started paused");
    assert_eq!(paused.info().position, Duration::ZERO);
    assert_eq!(other.info().state, PlayState::Queued);
    paused.play().unwrap();
    assert!(matches!(
        next(&mut paused_events).await,
        TrackEvent::Playing
    ));
}

#[tokio::test]
async fn play_replaces_the_current_track_and_the_queue_goes_on() {
    let mock = Mock::new().await;
    let (manager, mut commands) = manager();
    let (call, _events, _server) = joined(&manager, &mut commands, &mock).await;
    let first = call.enqueue(endless());
    let waiting = call.enqueue(Input::pcm(std::io::Cursor::new(tone(2, 0.5))));
    let mut first_events = first.events();
    let mut waiting_events = waiting.events();
    assert!(matches!(next(&mut first_events).await, TrackEvent::Playing));
    let now = call.play(Input::pcm(std::io::Cursor::new(tone(2, 0.5))));
    let mut now_events = now.events();
    assert!(matches!(ended(&mut first_events).await, TrackEnd::Replaced));
    assert_eq!(call.queue().current(), Some(now.clone()));
    assert_eq!(call.queue().len(), 2);
    assert!(matches!(ended(&mut now_events).await, TrackEnd::Finished));
    assert!(matches!(
        next(&mut waiting_events).await,
        TrackEvent::Playing
    ));
    assert!(matches!(
        ended(&mut waiting_events).await,
        TrackEnd::Finished
    ));
}

#[tokio::test]
async fn the_volume_scales_pcm_and_ogg_opus_goes_as_it_is_at_1() {
    let mock = Mock::new().await;
    let (manager, mut commands) = manager();
    let (call, _events, _server) = joined(&manager, &mut commands, &mock).await;
    let track = call.play(Input::pcm(std::io::Cursor::new(tone(200, 0.5))));

    let full = loudness(&mock, 3).await;
    assert!((full - 0.354).abs() < 0.05, "rms {full}");
    track.set_volume(0.5).unwrap();
    assert!((track.info().volume - 0.5).abs() < f32::EPSILON);
    let half = loudness(&mock, 5).await;
    assert!((half - 0.177).abs() < 0.03, "rms {half}");
    track.set_volume(f32::NAN).unwrap();
    assert!(
        track.info().volume.abs() < f32::EPSILON,
        "NaN counts as 0.0"
    );
    track.set_volume(7.0).unwrap();
    assert!((track.info().volume - 2.0).abs() < f32::EPSILON, "clamped");
    track.stop().unwrap();

    let expected = sine_packets();
    let track = call.play(Input::ogg(SINE));

    let mut in_flight = 0;
    while !expected.contains(&audio(&mock, KEY).await.0) {
        in_flight += 1;
        assert!(in_flight < 20, "not sent as it is at 1.0");
    }
    track.set_volume(0.5).unwrap();

    for _ in 0..3 {
        audio(&mock, KEY).await;
    }
    for _ in 0..5 {
        let (payload, _, _) = audio(&mock, KEY).await;
        assert!(!expected.contains(&payload), "re-encoded at 0.5");
        assert_eq!(packet_samples(&payload), Some(960));
    }
    track.set_volume(1.0).unwrap();

    let mut reencoded = 0;
    loop {
        let (payload, _, _) = audio(&mock, KEY).await;
        if let Some(n) = expected.iter().position(|packet| *packet == payload) {
            assert_eq!(n, 20, "as it is again at 1.0");
            break;
        }
        reencoded += 1;
        assert!(reencoded < 20, "not as it is again at 1.0");
    }
}

#[tokio::test]
async fn files_loop_and_seek_and_readers_cannot() {
    let dir = std::env::temp_dir().join(format!("discord-voice-player-{}", std::process::id()));
    std::fs::create_dir_all(&dir).unwrap();
    let path = dir.join("sine.opus");
    std::fs::write(&path, SINE).unwrap();
    let mock = Mock::new().await;
    let (manager, mut commands) = manager();
    let (call, _events, mut server) = joined(&manager, &mut commands, &mock).await;
    let expected = sine_packets();

    let track = call.play(Input::ogg_file(&path).await.unwrap());
    track.set_loop(Loop::Times(1)).unwrap();
    assert_eq!(track.info().looping, Loop::Times(1));
    let mut events = track.events();
    assert_eq!(recv_op(&mut server, 5).await["d"]["speaking"], 1);
    for n in 0..expected.len() * 2 {
        let (payload, _, _) = audio(&mock, KEY).await;
        assert_eq!(payload, expected[n % expected.len()], "packet {n}");
    }
    silence_follows(&mock, &mut server).await;
    assert!(matches!(next(&mut events).await, TrackEvent::Playing));
    assert!(matches!(next(&mut events).await, TrackEvent::Looped));
    assert!(matches!(ended(&mut events).await, TrackEnd::Finished));
    assert_eq!(track.info().looping, Loop::Times(0));

    let track = call.play(Input::ogg_file(&path).await.unwrap());
    audio(&mock, KEY).await;
    let position = track.seek(Duration::from_millis(500)).await.unwrap();
    assert_eq!(position, Duration::from_millis(500));
    let mut in_flight = 0;
    while audio(&mock, KEY).await.0 != expected[28] {
        in_flight += 1;
        assert!(in_flight < 7, "the new place does not play");
    }
    assert!(track.info().position >= Duration::from_millis(580));

    assert_eq!(track.seek(Duration::ZERO).await, Ok(Duration::ZERO));
    let mut events = track.events();
    assert_eq!(
        track.seek(Duration::from_secs(5)).await,
        Err(TrackError::Ended)
    );
    assert!(matches!(ended(&mut events).await, TrackEnd::Finished));

    let current = call.play(endless());
    let waiting = call.enqueue(Input::ogg_file(&path).await.unwrap());
    assert_eq!(
        waiting.seek(Duration::from_millis(800)).await,
        Ok(Duration::from_millis(800))
    );
    assert_eq!(waiting.info().position, Duration::from_millis(800));
    assert_eq!(waiting.info().state, PlayState::Queued);

    assert_eq!(
        current.set_loop(Loop::Forever),
        Err(TrackError::Unsupported)
    );
    assert_eq!(
        current.seek(Duration::from_secs(1)).await,
        Err(TrackError::Unsupported)
    );
    current.stop().unwrap();

    let mut in_flight = 0;
    while audio(&mock, KEY).await.0 != expected[40] {
        in_flight += 1;

        assert!(in_flight < 40, "the waiting track did not start at 0.8 s");
    }
    std::fs::remove_dir_all(&dir).unwrap();
}

#[tokio::test]
async fn a_failed_input_ends_its_track_and_the_next_plays() {
    let mock = Mock::new().await;
    let (manager, mut commands) = manager();
    let (call, _events, _server) = joined(&manager, &mut commands, &mock).await;
    let not_ogg = call.enqueue(Input::ogg(&b"plain text"[..]));
    let no_ffmpeg = call.enqueue(Input::ffmpeg_with("discord-voice-no-such-program", "a.mp3"));
    let good = call.enqueue(Input::pcm(std::io::Cursor::new(tone(2, 0.5))));
    let (mut a, mut b, mut c) = (not_ogg.events(), no_ffmpeg.events(), good.events());
    match ended(&mut a).await {
        TrackEnd::Failed(error) => assert!(matches!(*error, InputError::NotOgg)),
        other => panic!("{other:?}"),
    }
    match ended(&mut b).await {
        TrackEnd::Failed(error) => assert!(matches!(*error, InputError::FfmpegMissing)),
        other => panic!("{other:?}"),
    }
    assert!(matches!(next(&mut c).await, TrackEvent::Playing));
    assert!(matches!(ended(&mut c).await, TrackEnd::Finished));
}

#[tokio::test]
async fn the_end_of_the_call_ends_its_tracks() {
    let mock = Mock::new().await;
    let (manager, mut commands) = manager();
    let (call, _events, _server) = joined(&manager, &mut commands, &mock).await;
    let current = call.enqueue(endless());
    let waiting = call.enqueue(endless());
    let (mut a, mut b) = (current.events(), waiting.events());
    call.leave().await.unwrap();
    assert!(matches!(ended(&mut a).await, TrackEnd::CallEnded));
    assert!(matches!(ended(&mut b).await, TrackEnd::CallEnded));
    let late = call.play(endless());
    assert_eq!(late.info().state, PlayState::Ended);
    assert!(late.events().next().await.is_none());
    assert!(call.queue().is_empty());
}

#[tokio::test]
async fn joining_again_changes_the_bitrate() {
    let mock = Mock::new().await;
    let (manager, mut commands) = manager();
    let (call, _events, _server) = joined(&manager, &mut commands, &mock).await;
    call.play(Input::pcm(std::io::Cursor::new(tone(500, 0.5))));
    let mut size = 0;
    for _ in 0..10 {
        size += audio(&mock, KEY).await.0.len();
    }

    let options = JoinOptions::new().bitrate(8_000);
    let again = manager.join_with(guild(), ChannelId::new(CHANNEL), options);
    let (again, _) = tokio::join!(again, async { super::tests::command(&mut commands).await });
    again.unwrap();
    for _ in 0..3 {
        audio(&mock, KEY).await;
    }
    let mut small = 0;
    for _ in 0..10 {
        small += audio(&mock, KEY).await.0.len();
    }

    assert!(
        small * 4 < size,
        "{small} bytes at 8 kb/s, {size} at 128 kb/s"
    );
}

const GAP: u128 = 60;

struct Capture {
    left: Vec<f32>,
    arrivals: Vec<std::time::Instant>,
    decoder: Decoder,
}

impl Capture {
    fn new() -> Self {
        Self {
            left: Vec::new(),
            arrivals: Vec::new(),
            decoder: Decoder::new().unwrap(),
        }
    }

    async fn packet(&mut self, mock: &Mock) -> Vec<u8> {
        let (payload, _, _) = audio(mock, KEY).await;
        self.arrivals.push(std::time::Instant::now());
        let mut pcm = vec![0.0; MAX_PACKET_VALUES];
        let samples = self.decoder.decode(&payload, &mut pcm).unwrap();
        self.left.extend(pcm[..samples * 2].iter().step_by(2));
        payload
    }

    fn roughness(&self, from: usize, to: usize) -> f32 {
        let x = &self.left[from * 960..to * 960];
        x.windows(3)
            .map(|w| (w[2] - 2.0 * w[1] + w[0]).abs())
            .fold(0.0, f32::max)
    }

    fn level(&self, from: usize, to: usize) -> f32 {
        let x = &self.left[from * 960..to * 960];
        #[expect(clippy::cast_precision_loss, reason = "a few thousand samples")]
        let len = x.len() as f32;
        (x.iter().map(|v| v * v).sum::<f32>() / len).sqrt()
    }

    fn holds(&self, from: usize, to: usize, level: f32) -> bool {
        (from..to).all(|n| (self.level(n, n + 1) - level).abs() < level * 0.12)
    }

    fn below(&self, from: usize, level: f32) -> usize {
        (from..self.arrivals.len())
            .find(|&n| self.level(n, n + 1) < level)
            .expect("no quieter packet")
    }

    fn longest_gap(&self, from: usize) -> u128 {
        self.arrivals[from..]
            .windows(2)
            .map(|w| (w[1] - w[0]).as_millis())
            .max()
            .unwrap_or(0)
    }
}

#[tokio::test]
async fn volume_changes_pauses_and_seeks_make_no_click() {
    let mock = Mock::new().await;
    let (manager, mut commands) = manager();
    let (call, _events, _server) = joined(&manager, &mut commands, &mock).await;

    let track = call.play(Input::pcm(std::io::Cursor::new(tone(400, 0.5))));
    let mut capture = Capture::new();
    for _ in 0..30 {
        capture.packet(&mock).await;
    }
    let own = capture.roughness(5, 30);
    let level = capture.level(5, 30);
    track.set_volume(0.3).unwrap();
    for _ in 0..30 {
        capture.packet(&mock).await;
    }

    let lower = capture.below(30, level * 0.9);
    assert!(capture.holds(lower + 2, 60, level * 0.3));
    track.pause().unwrap();
    let mut silent = 0;
    while silent < 5 {
        if capture.packet(&mock).await == SILENCE {
            silent += 1;
        }
    }
    track.play().unwrap();
    let resumed = capture.arrivals.len();
    for _ in 0..30 {
        capture.packet(&mock).await;
    }
    let end = capture.arrivals.len();

    let changes = capture.roughness(30, end);
    assert!(changes < own * 2.0, "{changes} against its own {own}");
    let gap = capture.longest_gap(resumed);
    assert!(gap < GAP, "{gap} ms between two packets");
    track.stop().unwrap();

    let dir = std::env::temp_dir().join(format!("discord-voice-smooth-{}", std::process::id()));
    std::fs::create_dir_all(&dir).unwrap();
    let path = dir.join("sine.opus");
    std::fs::write(&path, SINE).unwrap();
    let expected = sine_packets();
    let track = call.play(Input::ogg_file(&path).await.unwrap());
    let mut capture = Capture::new();
    while capture.packet(&mock).await != expected[0] {}
    for _ in 0..15 {
        capture.packet(&mock).await;
    }
    let own = capture.roughness(3, 16);
    let level = capture.level(3, 16);
    track.seek(Duration::from_millis(300)).await.unwrap();
    for _ in 0..15 {
        capture.packet(&mock).await;
    }
    let end = capture.arrivals.len();
    let sought = capture.roughness(16, end);
    assert!(sought < own * 2.0, "{sought} against its own {own}");
    let gap = capture.longest_gap(1);
    assert!(gap < GAP, "{gap} ms between two packets");

    let faded = capture.below(16, level * 0.5);
    assert!(capture.holds(faded + 1, end, level));

    track.set_volume(0.5).unwrap();
    for _ in 0..15 {
        capture.packet(&mock).await;
    }
    let lower = capture.below(end, level * 0.9);
    let after = capture.arrivals.len();
    let halved = capture.roughness(end, after);
    assert!(halved < own * 2.0, "{halved} against its own {own}");
    assert!(capture.holds(lower + 2, after, level * 0.5));
    std::fs::remove_dir_all(&dir).unwrap();
}
