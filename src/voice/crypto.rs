use aes_gcm::{
    Aes256Gcm,
    aead::{AeadInOut, KeyInit},
};
use chacha20poly1305::XChaCha20Poly1305;

pub(crate) const TAG: usize = 16;

pub(crate) const NONCE: usize = 4;

pub(crate) const OVERHEAD: usize = TAG + NONCE;

pub(crate) const RTP_HEADER: usize = 12;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub(crate) enum Mode {
    Aes256Gcm,

    XChaCha20Poly1305,
}

impl Mode {
    pub(crate) const fn name(self) -> &'static str {
        match self {
            Self::Aes256Gcm => "aead_aes256_gcm_rtpsize",
            Self::XChaCha20Poly1305 => "aead_xchacha20_poly1305_rtpsize",
        }
    }

    pub(crate) fn from_name(name: &str) -> Option<Self> {
        [Self::Aes256Gcm, Self::XChaCha20Poly1305]
            .into_iter()
            .find(|mode| mode.name() == name)
    }

    pub(crate) fn choose<'a>(offered: impl IntoIterator<Item = &'a str>) -> Option<Self> {
        offered
            .into_iter()
            .filter_map(Self::from_name)
            .min_by_key(|mode| match mode {
                Self::Aes256Gcm => 0,
                Self::XChaCha20Poly1305 => 1,
            })
    }
}

#[derive(Clone)]
pub(crate) enum Cipher {
    Aes256Gcm(Box<Aes256Gcm>),
    XChaCha20Poly1305(XChaCha20Poly1305),
}

impl core::fmt::Debug for Cipher {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("Cipher").field(&self.mode()).finish()
    }
}

impl Cipher {
    pub(crate) fn new(mode: Mode, key: &[u8; 32]) -> Self {
        match mode {
            Mode::Aes256Gcm => Self::Aes256Gcm(Box::new(Aes256Gcm::new(key.into()))),
            Mode::XChaCha20Poly1305 => Self::XChaCha20Poly1305(XChaCha20Poly1305::new(key.into())),
        }
    }

    pub(crate) const fn mode(&self) -> Mode {
        match self {
            Self::Aes256Gcm(_) => Mode::Aes256Gcm,
            Self::XChaCha20Poly1305(_) => Mode::XChaCha20Poly1305,
        }
    }

    pub(crate) fn seal(
        &self,
        packet: &mut [u8],
        header: usize,
        payload: usize,
        nonce: u32,
    ) -> Option<usize> {
        let len = header.checked_add(payload)?.checked_add(OVERHEAD)?;
        let packet = packet.get_mut(..len)?;
        let (aad, rest) = packet.split_at_mut(header);
        let (plaintext, suffix) = rest.split_at_mut(payload);
        let counter = nonce.to_be_bytes();
        let tag = match self {
            Self::Aes256Gcm(cipher) => cipher
                .encrypt_inout_detached(&nonce_12(counter).into(), aad, plaintext.into())
                .ok()?,
            Self::XChaCha20Poly1305(cipher) => cipher
                .encrypt_inout_detached(&nonce_24(counter).into(), aad, plaintext.into())
                .ok()?,
        };
        suffix[..TAG].copy_from_slice(&tag);
        suffix[TAG..].copy_from_slice(&counter);
        Some(len)
    }

    pub(crate) fn open(&self, packet: &mut [u8]) -> Option<core::ops::Range<usize>> {
        let &first = packet.first()?;
        let csrcs = usize::from(first & 0x0F);
        let extended = first & 0x10 != 0;
        let header = RTP_HEADER + 4 * csrcs + if extended { 4 } else { 0 };
        let end = packet.len().checked_sub(OVERHEAD)?;
        if end < header {
            return None;
        }
        let (aad, rest) = packet.split_at_mut(header);
        let (ciphertext, suffix) = rest.split_at_mut(end - header);
        let (tag, counter) = suffix.split_at(TAG);
        let tag: [u8; TAG] = tag.try_into().ok()?;
        let counter: [u8; NONCE] = counter.try_into().ok()?;
        let opened = match self {
            Self::Aes256Gcm(cipher) => cipher.decrypt_inout_detached(
                &nonce_12(counter).into(),
                aad,
                ciphertext.into(),
                &tag.into(),
            ),
            Self::XChaCha20Poly1305(cipher) => cipher.decrypt_inout_detached(
                &nonce_24(counter).into(),
                aad,
                ciphertext.into(),
                &tag.into(),
            ),
        };
        opened.ok()?;

        let start = if extended {
            let words = u16::from_be_bytes([aad[header - 2], aad[header - 1]]);
            header + 4 * usize::from(words)
        } else {
            header
        };
        (start <= end).then_some(start..end)
    }
}

fn nonce_12(counter: [u8; NONCE]) -> [u8; 12] {
    let mut nonce = [0; 12];
    nonce[..NONCE].copy_from_slice(&counter);
    nonce
}

fn nonce_24(counter: [u8; NONCE]) -> [u8; 24] {
    let mut nonce = [0; 24];
    nonce[..NONCE].copy_from_slice(&counter);
    nonce
}

#[cfg(test)]
mod tests {
    use super::{Cipher, Mode, OVERHEAD, RTP_HEADER};

    fn hex(text: &str) -> Vec<u8> {
        (0..text.len())
            .step_by(2)
            .map(|at| u8::from_str_radix(&text[at..at + 2], 16).unwrap())
            .collect()
    }

    fn key() -> [u8; 32] {
        core::array::from_fn(|at| 0x20 + u8::try_from(at).unwrap())
    }

    const HEADER: &str = "8078123489abcdef01020304";
    const PAYLOAD: &str = "fcfffe616e206f707573206672616d65206f6620736f6d65206279746573";

    const SEALED: [(Mode, u32, &str); 4] = [
        (
            Mode::XChaCha20Poly1305,
            7,
            "8078123489abcdef01020304052b93cb541c896dec3378feca25f36a31ac2f5dfeffbe27ac98ffb2ca09\
             44fb0fbe5fa17e94155338428981d7bf00000007",
        ),
        (
            Mode::Aes256Gcm,
            7,
            "8078123489abcdef010203044a6c8e8b6c4c00c6d4b774c76e5024c6fc1dff7b108f998ce3df3818a08b\
             01178ae1dfa0bb2389708f3383c10e7e00000007",
        ),
        (
            Mode::XChaCha20Poly1305,
            u32::MAX,
            "8078123489abcdef01020304a36dd7d7a464874ea54fca8373ee5e9869fc915b9c645c13eb8555077\
             7e3d67b10050a5317f8c8b9349a6157d801ffffffff",
        ),
        (
            Mode::Aes256Gcm,
            u32::MAX,
            "8078123489abcdef01020304fbfddbf9bbb96b5ebf24179f53ff603f9c1a670a8771c4c9e79b1ca82e8\
             4f27036267357cf7e50ecb5490b44c9aeffffffff",
        ),
    ];

    #[test]
    fn modes_are_named_and_chosen_as_rustydiscord_says() {
        assert_eq!(Mode::Aes256Gcm.name(), "aead_aes256_gcm_rtpsize");
        assert_eq!(
            Mode::from_name("aead_xchacha20_poly1305_rtpsize"),
            Some(Mode::XChaCha20Poly1305)
        );
        let offered = [
            "aead_aes256_gcm_rtpsize",
            "aead_xchacha20_poly1305_rtpsize",
            "xsalsa20_poly1305_lite_rtpsize",
        ];
        assert_eq!(Mode::choose(offered), Some(Mode::Aes256Gcm));
        assert_eq!(
            Mode::choose(["xsalsa20_poly1305", "aead_xchacha20_poly1305_rtpsize"]),
            Some(Mode::XChaCha20Poly1305)
        );
        assert_eq!(Mode::choose(["xsalsa20_poly1305_lite"]), None);
    }

    #[test]
    fn packets_match_libsodium() {
        let (header, payload) = (hex(HEADER), hex(PAYLOAD));
        for (mode, nonce, sealed) in SEALED {
            let cipher = Cipher::new(mode, &key());
            let mut packet = vec![0; 200];
            packet[..RTP_HEADER].copy_from_slice(&header);
            packet[RTP_HEADER..RTP_HEADER + payload.len()].copy_from_slice(&payload);
            let len = cipher
                .seal(&mut packet, RTP_HEADER, payload.len(), nonce)
                .unwrap();
            assert_eq!(packet[..len], hex(sealed), "{mode:?}, nonce {nonce}");
            let mut received = hex(sealed);
            let range = cipher.open(&mut received).unwrap();
            assert_eq!(received[range], payload, "{mode:?}, nonce {nonce}");
        }
    }

    #[test]
    fn received_extensions_are_decrypted_and_skipped() {
        let sealed = [
            (
                Mode::XChaCha20Poly1305,
                "91780102030405060708090a11121314bede0001209f59e2ac8718a80c2bd1b485e99b6053e699b4\
                 d6374501020304",
            ),
            (
                Mode::Aes256Gcm,
                "91780102030405060708090a11121314bede0001e2042d062780b85af6ca3c6bd9357d3b4e6611f7\
                 7a928a01020304",
            ),
        ];
        for (mode, sealed) in sealed {
            let mut packet = hex(sealed);
            let range = Cipher::new(mode, &key()).open(&mut packet).unwrap();
            assert_eq!(packet[range], [0xF8, 0xFF, 0xFE], "{mode:?}");
        }
    }

    #[test]
    fn damaged_or_foreign_packets_do_not_open() {
        for (mode, _, sealed) in SEALED {
            let cipher = Cipher::new(mode, &key());
            let packet = hex(sealed);

            for at in [
                1,
                5,
                RTP_HEADER,
                RTP_HEADER + 10,
                packet.len() - 10,
                packet.len() - 1,
            ] {
                let mut damaged = packet.clone();
                damaged[at] ^= 0x01;
                assert_eq!(cipher.open(&mut damaged), None, "{mode:?}, byte {at}");
            }
            let mut other_key = key();
            other_key[0] ^= 1;
            assert_eq!(
                Cipher::new(mode, &other_key).open(&mut packet.clone()),
                None
            );

            assert_eq!(
                cipher.open(&mut packet[..RTP_HEADER + OVERHEAD - 1].to_vec()),
                None
            );
            assert_eq!(cipher.open(&mut []), None);
        }
        let cipher = Cipher::new(Mode::Aes256Gcm, &key());

        let mut packet = vec![0; 64];
        packet[0] = 0x90;
        packet[RTP_HEADER..RTP_HEADER + 4].copy_from_slice(&[0xBE, 0xDE, 0x00, 0xFF]);
        let len = cipher.seal(&mut packet, RTP_HEADER + 4, 8, 1).unwrap();
        assert_eq!(cipher.open(&mut packet[..len]), None);
    }

    #[test]
    fn sealing_needs_room_for_the_overhead() {
        let cipher = Cipher::new(Mode::XChaCha20Poly1305, &key());
        let mut packet = [0; RTP_HEADER + 10 + OVERHEAD];
        assert_eq!(
            cipher.seal(&mut packet, RTP_HEADER, 10, 0),
            Some(packet.len())
        );
        assert_eq!(cipher.seal(&mut packet, RTP_HEADER, 11, 0), None);
        assert_eq!(cipher.seal(&mut packet, usize::MAX, 1, 0), None);
    }

    #[test]
    #[ignore = "a measurement; run it in release"]
    fn throughput() {
        use std::time::Instant;

        let packets: u32 = 1_000_000;
        for mode in [Mode::Aes256Gcm, Mode::XChaCha20Poly1305] {
            let cipher = Cipher::new(mode, &key());

            for payload in [160, 1275] {
                let mut packet = vec![0; RTP_HEADER + payload + OVERHEAD];
                let mut best = (f64::MAX, f64::MAX);
                for _ in 0..3 {
                    let started = Instant::now();
                    for nonce in 0..packets {
                        let len = cipher.seal(&mut packet, RTP_HEADER, payload, nonce);
                        std::hint::black_box(len);
                    }
                    let sealing = started.elapsed().as_secs_f64();
                    let sealed = packet.clone();
                    let started = Instant::now();
                    for _ in 0..packets {
                        packet.copy_from_slice(&sealed);
                        std::hint::black_box(cipher.open(&mut packet));
                    }
                    best = (
                        best.0.min(sealing),
                        best.1.min(started.elapsed().as_secs_f64()),
                    );
                }
                let per = |seconds: f64| seconds / f64::from(packets) * 1e9;
                println!(
                    "{mode:?}, {payload}-byte payload: seal {:.0} ns ({:.2} M packets/s), \
                     open {:.0} ns ({:.2} M packets/s)",
                    per(best.0),
                    f64::from(packets) / best.0 / 1e6,
                    per(best.1),
                    f64::from(packets) / best.1 / 1e6,
                );
            }
        }
    }

    #[test]
    fn the_key_stays_out_of_debug_output() {
        let cipher = Cipher::new(Mode::Aes256Gcm, &key());
        assert_eq!(format!("{cipher:?}"), "Cipher(Aes256Gcm)");
    }
}
