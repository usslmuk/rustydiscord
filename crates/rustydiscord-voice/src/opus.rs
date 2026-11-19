const SILK: [u32; 4] = [480, 960, 1920, 2880];
const HYBRID: [u32; 2] = [480, 960];
const CELT: [u32; 4] = [120, 240, 480, 960];

pub(crate) const MAX_PACKET_SAMPLES: u32 = 5760;

pub(crate) fn packet_samples(packet: &[u8]) -> Option<u32> {
    let &toc = packet.first()?;
    let config = toc >> 3;
    let frame = match config {
        0..=11 => SILK[usize::from(config & 0b11)],
        12..=15 => HYBRID[usize::from(config & 0b1)],
        _ => CELT[usize::from(config & 0b11)],
    };
    let frames = match toc & 0b11 {
        0 => 1,
        1 | 2 => 2,

        _ => u32::from(*packet.get(1)? & 0b11_1111),
    };
    let samples = frame * frames;
    (frames > 0 && samples <= MAX_PACKET_SAMPLES).then_some(samples)
}

#[cfg(test)]
mod tests {
    use super::packet_samples;

    fn toc(config: u8, stereo: bool, code: u8) -> u8 {
        (config << 3) | (u8::from(stereo) << 2) | code
    }

    #[test]
    fn durations_follow_the_table_of_contents() {
        let cases: [(u8, u8, Option<u32>); 16] = [
            (toc(0, false, 0), 0, Some(480)),
            (toc(1, false, 0), 0, Some(960)),
            (toc(2, false, 0), 0, Some(1920)),
            (toc(3, false, 0), 0, Some(2880)),
            (toc(12, false, 0), 0, Some(480)),
            (toc(13, false, 0), 0, Some(960)),
            (toc(16, false, 0), 0, Some(120)),
            (toc(17, false, 0), 0, Some(240)),
            (toc(18, false, 0), 0, Some(480)),
            (toc(31, true, 0), 0, Some(960)),
            (toc(31, true, 1), 0, Some(1920)),
            (toc(31, false, 2), 0, Some(1920)),
            (toc(31, true, 3), 6, Some(5760)),
            (toc(31, true, 3), 7, None),
            (toc(3, false, 3), 2, Some(5760)),
            (toc(16, false, 3), 0, None),
        ];
        for (toc, count, samples) in cases {
            assert_eq!(packet_samples(&[toc, count]), samples, "TOC {toc:#010b}");
        }
        assert_eq!(packet_samples(&[]), None);
        assert_eq!(
            packet_samples(&[toc(31, true, 3)]),
            None,
            "code 3 needs a second byte"
        );
        assert_eq!(
            packet_samples(&[toc(31, true, 0)]),
            Some(960),
            "codes 0 to 2 need no second byte"
        );
    }
}
