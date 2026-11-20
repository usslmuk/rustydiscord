use core::{fmt, ops::Range};

const CAPTURE: [u8; 4] = *b"OggS";

const HEADER: usize = 27;

const MAX_LEADING_GARBAGE: usize = 64 * 1024;

const CONTINUED: u8 = 0x01;

const FIRST: u8 = 0x02;

const LAST: u8 = 0x04;

const CRC_TABLES: [[u32; 256]; 8] = {
    let mut tables = [[0; 256]; 8];
    let mut index = 0;
    let mut value: u32 = 0;
    while index < 256 {
        let mut crc = value << 24;
        let mut bit = 0;
        while bit < 8 {
            crc = if crc & 0x8000_0000 == 0 {
                crc << 1
            } else {
                (crc << 1) ^ 0x04C1_1DB7
            };
            bit += 1;
        }
        tables[0][index] = crc;
        index += 1;
        value += 1;
    }
    let mut table = 1;
    while table < 8 {
        let mut index = 0;
        while index < 256 {
            let previous = tables[table - 1][index];
            tables[table][index] = (previous << 8) ^ tables[0][(previous >> 24) as usize];
            index += 1;
        }
        table += 1;
    }
    tables
};

fn crc(mut crc: u32, bytes: &[u8]) -> u32 {
    let [t0, t1, t2, t3, t4, t5, t6, t7] = &CRC_TABLES;
    let (blocks, rest) = bytes.as_chunks::<8>();
    for &[b0, b1, b2, b3, b4, b5, b6, b7] in blocks {
        let [x0, x1, x2, x3] = (crc ^ u32::from_be_bytes([b0, b1, b2, b3])).to_be_bytes();
        crc = t7[usize::from(x0)]
            ^ t6[usize::from(x1)]
            ^ t5[usize::from(x2)]
            ^ t4[usize::from(x3)]
            ^ t3[usize::from(b4)]
            ^ t2[usize::from(b5)]
            ^ t1[usize::from(b6)]
            ^ t0[usize::from(b7)];
    }
    for &byte in rest {
        crc = (crc << 8) ^ t0[usize::from(crc.to_be_bytes()[0] ^ byte)];
    }
    crc
}

fn le32(bytes: &[u8], at: usize) -> u32 {
    u32::from_le_bytes([bytes[at], bytes[at + 1], bytes[at + 2], bytes[at + 3]])
}

#[derive(Clone, Copy, Debug)]
struct Header {
    flags: u8,
    serial: u32,
    sequence: u32,

    segments: usize,

    len: usize,
}

#[derive(Debug)]
enum Scan {
    Page(Header),

    Incomplete,

    Invalid,
}

fn scan(bytes: &[u8]) -> Scan {
    let known = bytes.len().min(CAPTURE.len());
    if bytes[..known] != CAPTURE[..known] || bytes.get(4).is_some_and(|&version| version != 0) {
        return Scan::Invalid;
    }
    if bytes.len() < HEADER {
        return Scan::Incomplete;
    }
    let segments = usize::from(bytes[26]);
    let Some(lacing) = bytes.get(HEADER..HEADER + segments) else {
        return Scan::Incomplete;
    };
    let len = HEADER
        + segments
        + lacing
            .iter()
            .map(|&value| usize::from(value))
            .sum::<usize>();
    let Some(page) = bytes.get(..len) else {
        return Scan::Incomplete;
    };

    let computed = crc(crc(crc(0, &page[..22]), &[0; 4]), &page[26..]);
    if computed != le32(page, 22) {
        return Scan::Invalid;
    }
    Scan::Page(Header {
        flags: page[5],
        serial: le32(page, 14),
        sequence: le32(page, 18),
        segments,
        len,
    })
}

fn resync(bytes: &[u8]) -> usize {
    (1..bytes.len())
        .find(|&at| {
            let rest = &bytes[at..];
            let known = rest.len().min(CAPTURE.len());
            rest[..known] == CAPTURE[..known]
        })
        .unwrap_or(bytes.len())
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) struct OpusHead {
    pub(crate) channels: u8,

    pub(crate) pre_skip: u16,

    pub(crate) input_sample_rate: u32,

    pub(crate) output_gain: i16,

    pub(crate) mapping_family: u8,
}

impl OpusHead {
    const MAGIC: &'static [u8; 8] = b"OpusHead";

    fn parse(packet: &[u8]) -> Option<Self> {
        let rest = packet.strip_prefix(Self::MAGIC)?;
        let &[
            version,
            channels,
            s0,
            s1,
            r0,
            r1,
            r2,
            r3,
            g0,
            g1,
            mapping_family,
            ref table @ ..,
        ] = rest
        else {
            return None;
        };
        if version >> 4 != 0 || channels == 0 {
            return None;
        }
        if mapping_family == 0 {
            if channels > 2 {
                return None;
            }
        } else {
            let &[streams, coupled, ref mapping @ ..] = table else {
                return None;
            };
            if streams == 0 || coupled > streams || mapping.len() < usize::from(channels) {
                return None;
            }
        }
        Some(Self {
            channels,
            pre_skip: u16::from_le_bytes([s0, s1]),
            input_sample_rate: u32::from_le_bytes([r0, r1, r2, r3]),
            output_gain: i16::from_le_bytes([g0, g1]),
            mapping_family,
        })
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum OggError {
    NotOgg,

    NotOpus,

    BadHead,

    BadTags,
}

impl fmt::Display for OggError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(match self {
            Self::NotOgg => "the input is not an Ogg stream",
            Self::NotOpus => "the Ogg stream has no Opus audio from its start",
            Self::BadHead => "the Opus identification header is malformed or unsupported",
            Self::BadTags => "the Opus comment header is missing",
        })
    }
}

impl std::error::Error for OggError {}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Part {
    Head,
    Tags,
    Audio,
}

#[derive(Clone, Copy, Debug)]
enum Stream {
    Searching {
        first_link: bool,
    },

    Following {
        serial: u32,
        sequence: u32,
        part: Part,
    },
}

#[derive(Clone, Copy, Debug)]
struct Current {
    lacing: usize,
    lacing_end: usize,

    data: usize,

    end: usize,

    skip_first: bool,

    last: bool,
}

#[derive(Clone, Debug)]
enum Location {
    Page(Range<usize>),

    Assembled,
}

#[derive(Debug)]
pub(crate) struct OggOpus {
    buffer: Vec<u8>,
    start: usize,
    page: Option<Current>,

    assembled: Vec<u8>,

    continuing: bool,

    returned_assembled: bool,

    ended: bool,
    stream: Stream,
    head: Option<OpusHead>,

    leading: Option<usize>,
}

impl Default for OggOpus {
    fn default() -> Self {
        Self::new()
    }
}

impl OggOpus {
    pub(crate) const fn new() -> Self {
        Self {
            buffer: Vec::new(),
            start: 0,
            page: None,
            assembled: Vec::new(),
            continuing: false,
            returned_assembled: false,
            ended: false,
            stream: Stream::Searching { first_link: true },
            head: None,
            leading: Some(0),
        }
    }

    #[cfg_attr(
        not(test),
        expect(
            dead_code,
            reason = "for the player's decoding of step 2 (channels, gain)"
        )
    )]
    pub(crate) const fn head(&self) -> Option<&OpusHead> {
        self.head.as_ref()
    }

    pub(crate) fn push(&mut self, bytes: &[u8]) {
        if self.start > 0 && self.start >= self.buffer.len() - self.start {
            self.buffer.copy_within(self.start.., 0);
            self.buffer.truncate(self.buffer.len() - self.start);
            self.start = 0;
        }
        self.buffer.extend_from_slice(bytes);
    }

    pub(crate) const fn finish(&mut self) {
        self.ended = true;
    }

    pub(crate) fn packet(&mut self) -> Result<Option<&[u8]>, OggError> {
        Ok(match self.next_packet()? {
            None => None,
            Some(Location::Page(range)) => Some(&self.buffer[self.start..][range]),
            Some(Location::Assembled) => Some(&self.assembled),
        })
    }

    fn next_packet(&mut self) -> Result<Option<Location>, OggError> {
        loop {
            if self.page.is_none() && !self.next_page()? {
                return match (self.ended, self.head, self.leading) {
                    (true, None, None) => Err(OggError::NotOpus),
                    (true, None, Some(_)) => Err(OggError::NotOgg),
                    _ => Ok(None),
                };
            }
            match self.take_packet() {
                Some(location) => {
                    if let Some(audio) = self.route(location)? {
                        return Ok(Some(audio));
                    }
                }
                None => self.finish_page(),
            }
        }
    }

    fn next_page(&mut self) -> Result<bool, OggError> {
        loop {
            let unread = &self.buffer[self.start..];
            match scan(unread) {
                Scan::Incomplete if !self.ended || unread.is_empty() => return Ok(false),
                Scan::Incomplete | Scan::Invalid => {
                    let skip = resync(unread);
                    self.start += skip;
                    if let Some(leading) = &mut self.leading {
                        *leading += skip;
                        if *leading > MAX_LEADING_GARBAGE {
                            return Err(OggError::NotOgg);
                        }
                    }
                }
                Scan::Page(header) => {
                    self.leading = None;
                    if self.accept(header)? {
                        return Ok(true);
                    }
                    self.start += header.len;
                }
            }
        }
    }

    fn accept(&mut self, header: Header) -> Result<bool, OggError> {
        let page = &self.buffer[self.start..self.start + header.len];
        match &mut self.stream {
            Stream::Searching { first_link } => {
                if header.flags & FIRST == 0 {
                    return if *first_link {
                        Err(OggError::NotOpus)
                    } else {
                        Ok(false)
                    };
                }
                let data = HEADER + header.segments;
                if page.get(data..data + OpusHead::MAGIC.len()) != Some(OpusHead::MAGIC) {
                    return Ok(false);
                }
                self.stream = Stream::Following {
                    serial: header.serial,
                    sequence: header.sequence.wrapping_add(1),
                    part: Part::Head,
                };
                self.drop_assembled();
            }
            Stream::Following {
                serial, sequence, ..
            } => {
                if header.serial != *serial {
                    return Ok(false);
                }
                if header.sequence != *sequence {
                    self.continuing = false;
                    self.assembled.clear();
                }
                *sequence = header.sequence.wrapping_add(1);
            }
        }
        let continued = header.flags & CONTINUED != 0;
        if self.continuing && !continued {
            self.drop_assembled();
        }
        self.page = Some(Current {
            lacing: HEADER,
            lacing_end: HEADER + header.segments,
            data: HEADER + header.segments,
            end: header.len,
            skip_first: continued && !self.continuing,
            last: header.flags & LAST != 0,
        });
        Ok(true)
    }

    fn drop_assembled(&mut self) {
        self.continuing = false;
        self.assembled.clear();
    }

    fn take_packet(&mut self) -> Option<Location> {
        if self.returned_assembled {
            self.returned_assembled = false;
            self.assembled.clear();
        }
        let page = self.page.as_mut()?;
        let bytes = &self.buffer[self.start..self.start + page.end];
        if page.skip_first {
            page.skip_first = false;
            while page.lacing < page.lacing_end {
                let value = bytes[page.lacing];
                page.lacing += 1;
                page.data += usize::from(value);
                if value < 255 {
                    break;
                }
            }
        }
        let begin = page.data;
        let mut end = begin;
        let read_any = page.lacing < page.lacing_end;
        while page.lacing < page.lacing_end {
            let value = bytes[page.lacing];
            page.lacing += 1;
            end += usize::from(value);
            if value < 255 {
                page.data = end;
                if self.continuing {
                    self.continuing = false;
                    self.assembled.extend_from_slice(&bytes[begin..end]);
                    self.returned_assembled = true;
                    return Some(Location::Assembled);
                }
                return Some(Location::Page(begin..end));
            }
        }
        if read_any {
            self.assembled.extend_from_slice(&bytes[begin..end]);
            self.continuing = true;
            page.data = end;
        }
        None
    }

    fn finish_page(&mut self) {
        let Some(page) = self.page.take() else {
            return;
        };
        self.start += page.end;
        if page.last {
            self.stream = Stream::Searching { first_link: false };
            self.drop_assembled();
        }
    }

    fn route(&mut self, location: Location) -> Result<Option<Location>, OggError> {
        let Stream::Following { part, .. } = self.stream else {
            return Ok(None);
        };
        let packet = match &location {
            Location::Page(range) => &self.buffer[self.start..][range.clone()],
            Location::Assembled => &self.assembled[..],
        };
        let next = match part {
            Part::Head => {
                self.head = Some(OpusHead::parse(packet).ok_or(OggError::BadHead)?);
                Part::Tags
            }
            Part::Tags => {
                if !packet.starts_with(b"OpusTags") {
                    return Err(OggError::BadTags);
                }
                Part::Audio
            }
            Part::Audio => return Ok((!packet.is_empty()).then_some(location)),
        };
        if let Stream::Following { part, .. } = &mut self.stream {
            *part = next;
        }
        Ok(None)
    }
}

#[cfg(test)]
pub(crate) mod tests {
    use super::{
        CONTINUED, CRC_TABLES, FIRST, LAST, MAX_LEADING_GARBAGE, OggError, OggOpus, OpusHead, crc,
    };
    use crate::voice::opus::packet_samples;

    const SINE: &[u8] = include_bytes!("testdata/sine.opus");

    fn page(flags: u8, serial: u32, sequence: u32, lacing: &[u8], data: &[u8]) -> Vec<u8> {
        let mut page = b"OggS\0".to_vec();
        page.push(flags);
        page.extend_from_slice(&0_u64.to_le_bytes());
        page.extend_from_slice(&serial.to_le_bytes());
        page.extend_from_slice(&sequence.to_le_bytes());
        page.extend_from_slice(&[0; 4]);
        page.push(u8::try_from(lacing.len()).unwrap());
        page.extend_from_slice(lacing);
        page.extend_from_slice(data);
        let checksum = crc(0, &page);
        page[22..26].copy_from_slice(&checksum.to_le_bytes());
        page
    }

    pub(crate) fn stream(
        serial: u32,
        sequence: u32,
        packets: &[Vec<u8>],
        per_page: usize,
    ) -> Vec<u8> {
        let mut segments: Vec<(u8, &[u8])> = Vec::new();
        for packet in packets {
            let mut chunks = packet.chunks(255).peekable();
            if packet.len() % 255 == 0 {
                segments.extend(chunks.map(|chunk| (255, chunk)));
                segments.push((0, &[]));
            } else {
                while let Some(chunk) = chunks.next() {
                    let value = if chunks.peek().is_some() {
                        255
                    } else {
                        u8::try_from(chunk.len()).unwrap()
                    };
                    segments.push((value, chunk));
                }
            }
        }
        let pages: Vec<&[(u8, &[u8])]> = segments.chunks(per_page).collect();
        let mut out = Vec::new();
        let mut continued = false;
        for (index, segments) in pages.iter().enumerate() {
            let mut flags = 0;
            if continued {
                flags |= CONTINUED;
            }
            if index == 0 {
                flags |= FIRST;
            }
            if index == pages.len() - 1 {
                flags |= LAST;
            }
            let lacing: Vec<u8> = segments.iter().map(|(value, _)| *value).collect();
            let data: Vec<u8> = segments
                .iter()
                .flat_map(|(_, data)| *data)
                .copied()
                .collect();
            let number = sequence + u32::try_from(index).unwrap();
            out.extend(page(flags, serial, number, &lacing, &data));
            continued = lacing.last() == Some(&255);
        }
        out
    }

    fn head(channels: u8) -> Vec<u8> {
        let mut head = b"OpusHead".to_vec();
        head.extend_from_slice(&[1, channels]);
        head.extend_from_slice(&312_u16.to_le_bytes());
        head.extend_from_slice(&48_000_u32.to_le_bytes());
        head.extend_from_slice(&0_i16.to_le_bytes());
        head.push(0);
        head
    }

    fn tags() -> Vec<u8> {
        let mut tags = b"OpusTags".to_vec();
        tags.extend_from_slice(&4_u32.to_le_bytes());
        tags.extend_from_slice(b"test");
        tags.extend_from_slice(&0_u32.to_le_bytes());
        tags
    }

    pub(crate) fn audio(number: u8, len: usize) -> Vec<u8> {
        (0..len)
            .map(|at| number.wrapping_add(u8::try_from(at % 251).unwrap()))
            .collect()
    }

    pub(crate) fn opus(audio: &[Vec<u8>]) -> Vec<Vec<u8>> {
        let mut packets = vec![head(2), tags()];
        packets.extend_from_slice(audio);
        packets
    }

    fn read(input: &[u8], chunk: usize) -> Result<Vec<Vec<u8>>, OggError> {
        let mut reader = OggOpus::new();
        let mut packets = Vec::new();
        for bytes in input.chunks(chunk) {
            reader.push(bytes);
            while let Some(packet) = reader.packet()? {
                packets.push(packet.to_vec());
            }
        }
        reader.finish();
        while let Some(packet) = reader.packet()? {
            packets.push(packet.to_vec());
        }
        Ok(packets)
    }

    #[test]
    fn the_checksum_matches_the_reference() {
        assert_eq!(crc(0, b"123456789"), 0x89A1_897F);

        let bytes: Vec<u8> = (0..300_u32)
            .map(|at| u8::try_from(at.wrapping_mul(2_654_435_761) >> 24).unwrap())
            .collect();
        let one_at_a_time = |bytes: &[u8]| {
            bytes.iter().fold(0_u32, |crc, &byte| {
                (crc << 8) ^ CRC_TABLES[0][usize::from(crc.to_be_bytes()[0] ^ byte)]
            })
        };
        for start in 0..9 {
            for end in start..bytes.len() {
                assert_eq!(
                    crc(0, &bytes[start..end]),
                    one_at_a_time(&bytes[start..end])
                );
            }
        }

        assert_eq!(crc(crc(0, &bytes[..13]), &bytes[13..]), crc(0, &bytes));
    }

    #[test]
    fn a_file_from_ffmpeg_is_read_whole() {
        let mut reader = OggOpus::new();
        reader.push(SINE);
        reader.finish();
        let mut sizes = Vec::new();
        while let Some(packet) = reader.packet().unwrap() {
            assert_eq!(packet_samples(packet), Some(960), "20 ms frames");
            sizes.push(packet.len());
        }

        assert_eq!(sizes.len(), 51);
        assert_eq!(sizes.iter().sum::<usize>(), 4349);
        assert_eq!((sizes[0], sizes[50]), (78, 113));
        assert_eq!(
            reader.head(),
            Some(&OpusHead {
                channels: 2,
                pre_skip: 312,
                input_sample_rate: 48_000,
                output_gain: 0,
                mapping_family: 0,
            })
        );
    }

    #[test]
    fn any_chunking_gives_the_same_packets() {
        let whole = read(SINE, SINE.len()).unwrap();
        for chunk in [1, 2, 3, 7, 27, 28, 100, 4096] {
            assert_eq!(read(SINE, chunk).unwrap(), whole, "chunks of {chunk}");
        }
    }

    #[test]
    fn packets_cross_pages() {
        let audio = vec![
            audio(1, 10),
            audio(2, 255),
            audio(3, 510),
            audio(4, 70_000),
            audio(5, 1),
            audio(6, 600),
        ];
        for per_page in [1, 2, 3, 255] {
            let input = stream(7, 0, &opus(&audio), per_page);
            for chunk in [1, 13, input.len()] {
                assert_eq!(
                    read(&input, chunk).unwrap(),
                    audio,
                    "{per_page}/page, {chunk}"
                );
            }
        }
    }

    #[test]
    fn empty_audio_packets_are_skipped() {
        let input = stream(1, 0, &opus(&[audio(1, 3), Vec::new(), audio(2, 3)]), 255);
        assert_eq!(read(&input, 64).unwrap(), [audio(1, 3), audio(2, 3)]);
    }

    #[test]
    fn a_damaged_page_loses_only_its_packets() {
        let audio: Vec<Vec<u8>> = (0..12).map(|number| audio(number, 300)).collect();

        let mut input = stream(3, 0, &opus(&audio), 2);

        let headers = stream(3, 0, &opus(&[]), 2).len();
        let at = headers + 4 * (27 + 2 + 300) + 100;
        input[at] ^= 0xFF;
        let mut expected = audio.clone();
        expected.remove(4);
        assert_eq!(read(&input, 50).unwrap(), expected);
    }

    #[test]
    fn a_damaged_page_drops_the_packet_it_continues() {
        let audio: Vec<Vec<u8>> = (0..6).map(|number| audio(number, 300)).collect();
        let packets = opus(&audio);
        let mut input = stream(3, 0, &packets, 1);
        let headers = stream(3, 0, &opus(&[]), 1).len();

        let at = headers + 2 * (2 * 28 + 300) + 28 + 255 + 10;
        input[at] ^= 0xFF;
        let mut expected = audio.clone();
        expected.remove(2);
        assert_eq!(read(&input, 64).unwrap(), expected);

        let mut input = stream(3, 0, &packets, 1);
        input[headers + 2 * (2 * 28 + 300) + 28 + 10] ^= 0xFF;
        assert_eq!(read(&input, 64).unwrap(), expected);
    }

    #[test]
    fn garbage_is_skipped() {
        let audio = vec![audio(1, 40), audio(2, 40), audio(3, 40)];
        let pages = stream(9, 0, &opus(&audio), 1);
        let mut input = b"ID3 not an ogg page, OggS but no page".to_vec();
        input.extend_from_slice(&pages[..pages.len() / 2]);
        input.extend_from_slice(b"Og junk between pages OggS\0");
        input.extend_from_slice(&pages[pages.len() / 2..]);

        let read = read(&input, 5).unwrap();
        assert!(read.len() >= 2 && read.iter().all(|packet| audio.contains(packet)));
    }

    #[test]
    fn a_false_capture_pattern_does_not_hide_the_pages_after_it() {
        let audio = vec![audio(1, 40), audio(2, 40)];
        let pages = split_pages(&stream(9, 0, &opus(&audio), 1));

        let mut input = pages[..3].concat();
        input.extend_from_slice(b"OggS\0");
        input.extend_from_slice(&[0; 21]);
        input.push(255);
        input.extend_from_slice(&pages[3]);
        assert_eq!(read(&input, 16).unwrap(), audio);
    }

    #[test]
    fn inputs_that_are_not_ogg_fail() {
        assert_eq!(read(&[], 1), Err(OggError::NotOgg));
        assert_eq!(read(b"RIFF....WAVEfmt ", 4), Err(OggError::NotOgg));
        let zeros = vec![0; MAX_LEADING_GARBAGE + 1000];
        let mut reader = OggOpus::new();
        reader.push(&zeros);
        assert_eq!(reader.packet(), Err(OggError::NotOgg));
    }

    #[test]
    fn ogg_without_opus_fails() {
        let mut vorbis = b"\x01vorbis".to_vec();
        vorbis.resize(30, 0);
        let other = stream(5, 0, &[vorbis.clone(), vorbis], 1);
        assert_eq!(read(&other, 64), Err(OggError::NotOpus));

        let first = page(FIRST, 5, 0, &[7], b"\x80theora");
        assert_eq!(read(&first, 64), Err(OggError::NotOpus));

        let input = stream(5, 0, &opus(&[audio(1, 9)]), 1);
        let second_page = 27 + 1 + head(2).len();
        assert_eq!(read(&input[second_page..], 64), Err(OggError::NotOpus));
    }

    #[test]
    fn malformed_headers_fail() {
        let mut new_version = head(2);
        new_version[8] = 0x10;
        let input = stream(1, 0, &[new_version, tags()], 255);
        assert_eq!(read(&input, 64), Err(OggError::BadHead));
        let mut many_channels = head(2);
        many_channels[9] = 3;
        let input = stream(1, 0, &[many_channels, tags()], 255);
        assert_eq!(read(&input, 64), Err(OggError::BadHead));
        let input = stream(1, 0, &[head(2), audio(1, 20)], 255);
        assert_eq!(read(&input, 64), Err(OggError::BadTags));
    }

    #[test]
    fn surround_heads_are_read() {
        let mut surround = head(6);
        surround[18] = 1;
        surround.extend_from_slice(&[4, 2, 0, 4, 1, 2, 3, 5]);
        let mut reader = OggOpus::new();
        reader.push(&stream(1, 0, &[surround.clone(), tags()], 255));
        assert_eq!(reader.packet(), Ok(None));
        assert_eq!(reader.head().map(|head| head.mapping_family), Some(1));

        surround.truncate(surround.len() - 1);
        let input = stream(1, 0, &[surround, tags()], 255);
        assert_eq!(read(&input, 64), Err(OggError::BadHead));
    }

    #[test]
    fn interleaved_streams_give_only_opus() {
        let audio: Vec<Vec<u8>> = (0..4).map(|number| audio(number, 20)).collect();
        let opus_pages = split_pages(&stream(1, 0, &opus(&audio), 1));
        let mut vorbis = b"\x01vorbis".to_vec();
        vorbis.resize(30, 0);
        let other_pages = split_pages(&stream(2, 0, &vec![vorbis; 6], 1));

        let mut input = other_pages[0].clone();
        input.extend_from_slice(&opus_pages[0]);
        for at in 1..opus_pages.len().max(other_pages.len()) {
            for pages in [&other_pages, &opus_pages] {
                if let Some(page) = pages.get(at) {
                    input.extend_from_slice(page);
                }
            }
        }
        assert_eq!(read(&input, 33).unwrap(), audio);
    }

    #[test]
    fn chained_streams_are_read_in_turn() {
        let first = [audio(1, 30), audio(2, 30)];
        let second = [audio(3, 30)];
        let mut input = stream(1, 0, &opus(&first), 1);
        let mut packets = vec![head(1), tags()];
        packets.extend_from_slice(&second);
        input.extend(stream(2, 0, &packets, 1));
        let mut reader = OggOpus::new();
        reader.push(&input);
        let mut read = Vec::new();
        while let Some(packet) = reader.packet().unwrap() {
            read.push(packet.to_vec());
        }
        assert_eq!(
            read,
            [first[0].clone(), first[1].clone(), second[0].clone()]
        );
        assert_eq!(reader.head().map(|head| head.channels), Some(1));
    }

    #[test]
    fn a_lost_page_drops_the_packet_it_held() {
        let audio: Vec<Vec<u8>> = (0..5).map(|number| audio(number, 300)).collect();
        let pages = split_pages(&stream(4, 0, &opus(&audio), 1));

        let mut input = Vec::new();
        for (index, page) in pages.iter().enumerate() {
            if index != 4 {
                input.extend_from_slice(page);
            }
        }
        let mut expected = audio.clone();
        expected.remove(1);
        assert_eq!(read(&input, 64).unwrap(), expected);
    }

    #[test]
    fn a_lost_middle_page_drops_the_packet_around_it() {
        let audio: Vec<Vec<u8>> = (0..3).map(|number| audio(number, 555)).collect();
        let pages = split_pages(&stream(4, 0, &opus(&audio), 1));

        let mut input = Vec::new();
        for (index, page) in pages.iter().enumerate() {
            if index != 6 {
                input.extend_from_slice(page);
            }
        }
        let mut expected = audio.clone();
        expected.remove(1);
        assert_eq!(read(&input, 64).unwrap(), expected);
    }

    #[test]
    fn a_truncated_end_keeps_what_came_before() {
        let audio: Vec<Vec<u8>> = (0..3).map(|number| audio(number, 50)).collect();
        let input = stream(4, 0, &opus(&audio), 1);
        assert_eq!(read(&input[..input.len() - 10], 64).unwrap(), audio[..2]);
    }

    #[test]
    #[ignore = "a measurement; run it in release"]
    fn throughput() {
        use std::time::Instant;

        let packets = 3600 * 50;
        let audio: Vec<Vec<u8>> = (0..packets)
            .map(|number| audio(u8::try_from(number % 256).unwrap(), 160))
            .collect();
        let input = stream(1, 0, &opus(&audio), 50);
        let megabytes = f64::from(u32::try_from(input.len()).unwrap()) / f64::from(1 << 20);
        let measure = |name: &str, input: &[u8], chunk: usize| {
            let mut best = f64::MAX;
            let mut read = 0;
            let mut capacity = 0;
            for _ in 0..5 {
                let started = Instant::now();
                let mut reader = OggOpus::new();
                read = 0;
                for bytes in input.chunks(chunk) {
                    reader.push(bytes);
                    while let Some(packet) = reader.packet().unwrap() {
                        read += packet.len();
                    }
                }
                reader.finish();
                while let Some(packet) = reader.packet().unwrap() {
                    read += packet.len();
                }
                best = best.min(started.elapsed().as_secs_f64());
                capacity = reader.buffer.capacity();
            }
            println!(
                "{name}: {:.0} MiB/s, {:.1} M packets/s, {:.0}x real time; {read} audio bytes, \
                 buffer {capacity} bytes ({chunk}-byte chunks)",
                megabytes / best,
                f64::from(packets) / best / 1e6,
                3600.0 / best,
            );
        };
        println!("input: {megabytes:.1} MiB, {packets} packets, 1 hour");
        measure("whole input", &input, 64 << 10);
        measure("4 KiB reads", &input, 4 << 10);

        let mut damaged = input.clone();
        let page = 27 + 50 + 50 * 160;
        for at in (page * 100 + 200..damaged.len()).step_by(page * 100) {
            damaged[at] ^= 0xFF;
        }
        measure("1 % damaged", &damaged, 64 << 10);
        let started = Instant::now();
        let checksum = crc(0, &input);
        println!(
            "checksum alone: {:.0} MiB/s ({checksum:#x})",
            megabytes / started.elapsed().as_secs_f64()
        );
    }

    fn split_pages(mut input: &[u8]) -> Vec<Vec<u8>> {
        let mut pages = Vec::new();
        while !input.is_empty() {
            let segments = usize::from(input[26]);
            let len = 27
                + segments
                + input[27..27 + segments]
                    .iter()
                    .map(|&value| usize::from(value))
                    .sum::<usize>();
            pages.push(input[..len].to_vec());
            input = &input[len..];
        }
        pages
    }
}
