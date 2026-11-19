use opus2::{Application, Bitrate, Channels, SoftClip};

pub(crate) const FRAME_VALUES: usize = 960 * 2;

pub(crate) const FRAME_BYTES: usize = FRAME_VALUES * 2;

pub(crate) const MAX_PACKET_VALUES: usize = crate::opus::MAX_PACKET_SAMPLES as usize * 2;

pub(crate) const MAX_PACKET: usize = 4000;

pub(crate) const DEFAULT_BITRATE: u32 = 128_000;

const BITRATES: core::ops::RangeInclusive<u32> = 500..=512_000;
const SAMPLE_RATE: u32 = 48_000;

pub(crate) fn celt_only(packet: &[u8]) -> bool {
    packet.first().is_some_and(|toc| toc >> 3 >= 16)
}

pub(crate) fn clamp_bitrate(bits: u32) -> u32 {
    bits.clamp(*BITRATES.start(), *BITRATES.end())
}

pub(crate) struct Encoder {
    opus: opus2::Encoder,

    clip: SoftClip,

    independent: bool,
}

impl Encoder {
    pub(crate) fn new(bitrate: u32) -> Option<Self> {
        let mut opus = opus2::Encoder::new(SAMPLE_RATE, Channels::Stereo, Application::Audio)
            .inspect_err(|error| tracing::warn!(%error, "could not create an Opus encoder"))
            .ok()?;
        set_bitrate(&mut opus, bitrate);
        Some(Self {
            opus,
            clip: SoftClip::new(Channels::Stereo),
            independent: false,
        })
    }

    pub(crate) fn set_bitrate(&mut self, bitrate: u32) {
        set_bitrate(&mut self.opus, bitrate);
    }

    pub(crate) fn independent_next(&mut self) {
        self.independent = true;
    }

    pub(crate) fn lookahead(&mut self) -> usize {
        self.opus
            .get_lookahead()
            .ok()
            .and_then(|samples| usize::try_from(samples).ok())
            .unwrap_or(312)
            .min(FRAME_VALUES / 2 - 1)
    }

    pub(crate) fn encode(
        &mut self,
        pcm: &mut [f32],
        from: f32,
        to: f32,
        out: &mut [u8],
    ) -> Option<usize> {
        ramp(pcm, from, to);
        if from > 1.0 || to > 1.0 {
            self.clip.apply(pcm);
        }

        let independent = core::mem::take(&mut self.independent);
        if independent {
            let _ = self.opus.set_prediction_disabled(true);
        }
        let encoded = self
            .opus
            .encode_float(pcm, out)
            .inspect_err(|error| tracing::debug!(%error, len = pcm.len(), "Opus encoding failed"))
            .ok();
        if independent {
            let _ = self.opus.set_prediction_disabled(false);
        }
        encoded
    }
}

fn ramp(pcm: &mut [f32], from: f32, to: f32) {
    let unchanged = (from - 1.0).abs() <= f32::EPSILON && (to - 1.0).abs() <= f32::EPSILON;
    if unchanged {
        return;
    }
    let (pairs, _) = pcm.as_chunks_mut::<2>();
    #[expect(clippy::cast_precision_loss, reason = "at most 5 760 samples")]
    let step = (to - from) / pairs.len().max(1) as f32;
    let mut gain = from;
    for pair in pairs {
        gain += step;
        pair[0] *= gain;
        pair[1] *= gain;
    }
}

fn set_bitrate(opus: &mut opus2::Encoder, bitrate: u32) {
    let bits = i32::try_from(clamp_bitrate(bitrate)).unwrap_or(i32::MAX);
    if let Err(error) = opus.set_bitrate(Bitrate::Bits(bits)) {
        tracing::warn!(%error, bits, "could not set the Opus bitrate");
    }
}

pub(crate) struct Decoder {
    opus: opus2::Decoder,
}

impl Decoder {
    pub(crate) fn new() -> Option<Self> {
        let opus = opus2::Decoder::new(SAMPLE_RATE, Channels::Stereo)
            .inspect_err(|error| tracing::warn!(%error, "could not create an Opus decoder"))
            .ok()?;
        Some(Self { opus })
    }

    pub(crate) fn decode(&mut self, packet: &[u8], out: &mut [f32]) -> Option<usize> {
        self.opus
            .decode_float(packet, out, false)
            .inspect_err(|error| tracing::debug!(%error, "Opus decoding failed"))
            .ok()
    }

    pub(crate) fn decode_i16(
        &mut self,
        packet: &[u8],
        fec: bool,
        out: &mut [i16],
    ) -> Option<usize> {
        self.opus
            .decode(packet, out, fec)
            .inspect_err(|error| tracing::debug!(%error, "Opus decoding failed"))
            .ok()
    }
}

pub(crate) fn from_s16le(bytes: &[u8], out: &mut [f32]) {
    for (value, pair) in out.iter_mut().zip(bytes.as_chunks::<2>().0) {
        *value = f32::from(i16::from_le_bytes(*pair)) / 32_768.0;
    }
}

#[cfg(test)]
mod tests {
    use super::{
        Decoder, Encoder, FRAME_BYTES, FRAME_VALUES, MAX_PACKET, MAX_PACKET_VALUES, celt_only,
        clamp_bitrate, from_s16le,
    };
    use crate::opus::packet_samples;

    fn tone(samples: usize) -> Vec<f32> {
        (0..samples)
            .flat_map(|i| {
                #[expect(clippy::cast_precision_loss, reason = "small sample indices")]
                let value = (i as f32 * 440.0 * core::f32::consts::TAU / 48_000.0).sin() * 0.5;
                [value, value]
            })
            .collect()
    }

    fn rms(pcm: &[f32]) -> f32 {
        #[expect(clippy::cast_precision_loss, reason = "short buffers")]
        let len = pcm.len() as f32;
        (pcm.iter().map(|value| value * value).sum::<f32>() / len).sqrt()
    }

    #[test]
    fn frames_survive_encoding_and_decoding() {
        let mut encoder = Encoder::new(128_000).unwrap();
        let mut decoder = Decoder::new().unwrap();
        let mut packet = [0; MAX_PACKET];
        let mut output = vec![0.0; MAX_PACKET_VALUES];
        let mut last = 0.0;

        for _ in 0..5 {
            let mut pcm = tone(960);
            let len = encoder.encode(&mut pcm, 1.0, 1.0, &mut packet).unwrap();
            assert_eq!(packet_samples(&packet[..len]), Some(960));
            assert_eq!(decoder.decode(&packet[..len], &mut output), Some(960));
            last = rms(&output[..FRAME_VALUES]);
        }

        assert!((last - 0.354).abs() < 0.05, "rms {last}");
    }

    #[test]
    fn the_volume_scales_the_audio() {
        let mut decoder = Decoder::new().unwrap();
        let mut packet = [0; MAX_PACKET];
        let mut output = vec![0.0; MAX_PACKET_VALUES];
        let mut loudness = |volume| {
            let mut encoder = Encoder::new(128_000).unwrap();
            let mut last = 0.0;
            for _ in 0..5 {
                let mut pcm = tone(960);
                let len = encoder
                    .encode(&mut pcm, volume, volume, &mut packet)
                    .unwrap();
                decoder.decode(&packet[..len], &mut output).unwrap();
                last = rms(&output[..FRAME_VALUES]);
            }
            last
        };
        let half = loudness(0.5);
        assert!((half - 0.177).abs() < 0.03, "rms {half}");
        assert!(loudness(0.0) < 0.01);

        let double = loudness(2.0);
        assert!(double > 0.5 && double <= 1.0, "rms {double}");
    }

    #[test]
    fn longer_frames_keep_their_duration() {
        let mut encoder = Encoder::new(64_000).unwrap();
        let mut packet = [0; MAX_PACKET];
        for samples in [480, 1920, 2880, 5760] {
            let mut pcm = tone(samples);
            let len = encoder.encode(&mut pcm, 0.5, 0.5, &mut packet).unwrap();
            assert_eq!(packet_samples(&packet[..len]), u32::try_from(samples).ok());
        }

        assert_eq!(encoder.encode(&mut tone(360), 1.0, 1.0, &mut packet), None);
    }

    #[test]
    fn bitrates_stay_within_what_libopus_accepts() {
        assert_eq!(clamp_bitrate(0), 500);
        assert_eq!(clamp_bitrate(96_000), 96_000);
        assert_eq!(clamp_bitrate(u32::MAX), 512_000);
        let mut encoder = Encoder::new(u32::MAX).unwrap();
        encoder.set_bitrate(8_000);
        let mut packet = [0; MAX_PACKET];
        let len = encoder
            .encode(&mut tone(960), 1.0, 1.0, &mut packet)
            .unwrap();

        assert!(len < 60, "{len} bytes");
    }

    #[test]
    fn the_mode_of_a_packet_is_its_configuration() {
        assert!(!celt_only(&[9 << 3]));
        assert!(!celt_only(&[15 << 3]));
        assert!(celt_only(&[19 << 3]));
        assert!(celt_only(&[31 << 3 | 0b011]));
        assert!(!celt_only(&[]));
    }

    #[test]
    fn gains_ramp_across_a_frame() {
        let mut pcm = vec![1.0; 8];
        super::ramp(&mut pcm, 1.0, 0.0);
        assert_eq!(pcm, [0.75, 0.75, 0.5, 0.5, 0.25, 0.25, 0.0, 0.0]);
        let mut pcm = vec![1.0; 4];
        super::ramp(&mut pcm, 0.5, 0.5);
        assert_eq!(pcm, [0.5; 4]);
        let mut pcm = vec![0.3; 4];
        super::ramp(&mut pcm, 1.0, 1.0);
        assert_eq!(pcm, [0.3; 4], "unchanged at 1.0");
    }

    #[test]
    fn s16le_becomes_floats() {
        let mut bytes = [0; FRAME_BYTES];
        bytes[..6].copy_from_slice(&[0x00, 0x80, 0xFF, 0x7F, 0x00, 0x40]);
        let mut out = [1.0; FRAME_VALUES];
        from_s16le(&bytes, &mut out);
        assert_eq!(out[..4], [-1.0, 32_767.0 / 32_768.0, 0.5, 0.0]);
    }

    #[test]
    fn damaged_packets_are_refused() {
        let mut decoder = Decoder::new().unwrap();
        let mut output = vec![0.0; MAX_PACKET_VALUES];

        assert_eq!(decoder.decode(&[0xFB, 0x3F], &mut output), None);
    }

    #[test]
    #[ignore = "a measurement; run it in release"]
    #[expect(clippy::too_many_lines, reason = "one measurement after another")]
    fn throughput() {
        use std::time::Instant;

        let frames: u32 = 500;
        let mut rng = fastrand::Rng::with_seed(7);
        let audio: Vec<Vec<f32>> = (0..frames)
            .map(|frame| {
                (0..FRAME_VALUES)
                    .map(|value| {
                        #[expect(clippy::cast_precision_loss, reason = "small sample indices")]
                        let t = (frame as usize * FRAME_VALUES / 2 + value / 2) as f32 / 48_000.0;
                        let tones = (t * 220.0 * core::f32::consts::TAU).sin() * 0.3
                            + (t * 1_320.0 * core::f32::consts::TAU).sin() * 0.1;
                        tones + (rng.f32() - 0.5) * 0.1
                    })
                    .collect()
            })
            .collect();
        let best = |run: &mut dyn FnMut()| {
            let mut best = f64::MAX;
            for _ in 0..3 {
                let started = Instant::now();
                run();
                best = best.min(started.elapsed().as_secs_f64());
            }
            best / f64::from(frames)
        };
        let report = |name: &str, per_frame: f64| {
            println!(
                "{name}: {:.1} us per 20 ms frame, {:.0}x real time on one core",
                per_frame * 1e6,
                0.02 / per_frame
            );
        };
        let mut packet = [0; MAX_PACKET];
        let mut pcm = vec![0.0; MAX_PACKET_VALUES];
        for bitrate in [128_000, 64_000] {
            let mut encoder = Encoder::new(bitrate).unwrap();
            let per = best(&mut || {
                for frame in &audio {
                    pcm[..FRAME_VALUES].copy_from_slice(frame);
                    std::hint::black_box(encoder.encode(
                        &mut pcm[..FRAME_VALUES],
                        1.0,
                        1.0,
                        &mut packet,
                    ));
                }
            });
            report(&format!("encode at {} kb/s", bitrate / 1000), per);
        }

        let mut encoder = Encoder::new(128_000).unwrap();
        let packets: Vec<Vec<u8>> = audio
            .iter()
            .map(|frame| {
                pcm[..FRAME_VALUES].copy_from_slice(frame);
                let len = encoder
                    .encode(&mut pcm[..FRAME_VALUES], 1.0, 1.0, &mut packet)
                    .unwrap();
                packet[..len].to_vec()
            })
            .collect();
        let average = packets.iter().map(Vec::len).sum::<usize>() / packets.len();
        println!("({average} bytes per packet at 128 kb/s)");
        let mut decoder = Decoder::new().unwrap();
        let per = best(&mut || {
            for packet in &packets {
                std::hint::black_box(decoder.decode(packet, &mut pcm));
            }
        });
        report("decode to f32", per);
        let mut samples = vec![0_i16; MAX_PACKET_VALUES];
        let per = best(&mut || {
            for packet in &packets {
                std::hint::black_box(decoder.decode_i16(packet, false, &mut samples));
            }
        });
        report("decode to i16", per);
        let per = best(&mut || {
            for _ in &packets {
                std::hint::black_box(decoder.decode_i16(&[], false, &mut samples[..FRAME_VALUES]));
            }
        });
        report("conceal a lost packet", per);

        let mut out = [0; MAX_PACKET];
        let per = best(&mut || {
            for packet in &packets {
                let samples = decoder.decode(packet, &mut pcm).unwrap();
                let len = encoder.encode(&mut pcm[..samples * 2], 0.5, 0.5, &mut out);
                assert!(std::hint::black_box(len).is_some());
            }
        });
        report("decode + re-encode at 0.5", per);

        let starts: u32 = 200;
        let mut best_start = f64::MAX;
        for _ in 0..3 {
            let started = Instant::now();
            for start in 0..starts {
                let mut fresh = Decoder::new().unwrap();
                let from = usize::try_from(start).unwrap() % (packets.len() - 20);
                for packet in &packets[from..from + 20] {
                    fresh.decode(packet, &mut pcm).unwrap();
                }
                std::hint::black_box(encoder.encode(
                    &mut pcm[..FRAME_VALUES],
                    1.0,
                    1.0,
                    &mut packet,
                ));
            }
            best_start = best_start.min(started.elapsed().as_secs_f64());
        }
        println!(
            "start of re-encoding (400 ms decoded, one frame encoded): {:.0} us",
            best_start / f64::from(starts) * 1e6
        );
    }
}
