use core::fmt;

use bytes::Bytes;
use compact_str::CompactString;
use serde::{Serialize, Serializer};

#[derive(Clone, PartialEq, Eq, Hash)]
pub struct ImageData {
    data: Bytes,
    content_type: CompactString,
}

impl ImageData {
    pub fn new(data: impl Into<Bytes>) -> Result<Self, ImageDataError> {
        let data = data.into();
        let content_type = detect(&data).ok_or(ImageDataError)?;
        Ok(Self {
            data,
            content_type: CompactString::const_new(content_type),
        })
    }

    #[must_use]
    pub fn with_content_type(
        data: impl Into<Bytes>,
        content_type: impl Into<CompactString>,
    ) -> Self {
        Self {
            data: data.into(),
            content_type: content_type.into(),
        }
    }

    #[must_use]
    pub const fn data(&self) -> &Bytes {
        &self.data
    }

    #[must_use]
    pub fn content_type(&self) -> &str {
        &self.content_type
    }
}

impl fmt::Display for ImageData {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("data:")?;
        f.write_str(&self.content_type)?;
        f.write_str(";base64,")?;
        write_base64(&self.data, f)
    }
}

impl fmt::Debug for ImageData {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("ImageData")
            .field("content_type", &self.content_type)
            .field("len", &self.data.len())
            .finish()
    }
}

impl Serialize for ImageData {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        serializer.collect_str(self)
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[non_exhaustive]
pub struct ImageDataError;

impl fmt::Display for ImageDataError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("unknown image format (expected PNG, JPEG, GIF, WebP or AVIF)")
    }
}

impl core::error::Error for ImageDataError {}

fn detect(data: &[u8]) -> Option<&'static str> {
    if data.starts_with(b"\x89PNG\r\n\x1a\n") {
        Some("image/png")
    } else if data.starts_with(b"\xff\xd8\xff") {
        Some("image/jpeg")
    } else if data.starts_with(b"GIF87a") || data.starts_with(b"GIF89a") {
        Some("image/gif")
    } else if data.starts_with(b"RIFF") && data.get(8..12) == Some(b"WEBP") {
        Some("image/webp")
    } else if data.get(4..8) == Some(b"ftyp") && matches!(data.get(8..12), Some(b"avif" | b"avis"))
    {
        Some("image/avif")
    } else {
        None
    }
}

const ALPHABET: &[u8; 64] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/";

const CHUNK: usize = 768;

pub(super) fn write_base64(data: &[u8], out: &mut impl fmt::Write) -> fmt::Result {
    let mut buffer = [0_u8; CHUNK / 3 * 4];
    for chunk in data.chunks(CHUNK) {
        let (groups, rest) = chunk.as_chunks::<3>();
        let mut len = 0;
        for &[a, b, c] in groups {
            buffer[len..len + 4].copy_from_slice(&quad([a, b, c], 4));
            len += 4;
        }
        match *rest {
            [a] => buffer[len..len + 4].copy_from_slice(&quad([a, 0, 0], 2)),
            [a, b] => buffer[len..len + 4].copy_from_slice(&quad([a, b, 0], 3)),
            _ => {}
        }
        if !rest.is_empty() {
            len += 4;
        }

        let text = core::str::from_utf8(&buffer[..len]).map_err(|_| fmt::Error)?;
        out.write_str(text)?;
    }
    Ok(())
}

fn quad([a, b, c]: [u8; 3], significant: usize) -> [u8; 4] {
    let n = u32::from(a) << 16 | u32::from(b) << 8 | u32::from(c);
    let mut out = [b'='; 4];
    for (i, char) in out.iter_mut().enumerate().take(significant) {
        let shift = 18 - 6 * i;
        *char = ALPHABET[(n >> shift & 0x3f) as usize];
    }
    out
}

#[cfg(test)]
mod tests {
    use core::fmt::Write as _;

    use super::*;

    fn base64(data: &[u8]) -> String {
        let mut out = String::new();
        write_base64(data, &mut out).unwrap();
        out
    }

    fn reference(data: &[u8]) -> String {
        let bits = data.iter().fold(String::new(), |mut bits, byte| {
            write!(bits, "{byte:08b}").unwrap();
            bits
        });
        let mut out: String = bits
            .as_bytes()
            .chunks(6)
            .map(|six| {
                let six = format!("{:0<6}", core::str::from_utf8(six).unwrap());
                char::from(ALPHABET[usize::from_str_radix(&six, 2).unwrap()])
            })
            .collect();
        while !out.len().is_multiple_of(4) {
            out.push('=');
        }
        out
    }

    #[test]
    fn rfc_4648_vectors() {
        for (input, output) in [
            ("", ""),
            ("f", "Zg=="),
            ("fo", "Zm8="),
            ("foo", "Zm9v"),
            ("foob", "Zm9vYg=="),
            ("fooba", "Zm9vYmE="),
            ("foobar", "Zm9vYmFy"),
        ] {
            assert_eq!(base64(input.as_bytes()), output, "{input:?}");
        }
    }

    #[test]
    fn matches_the_reference_across_chunk_boundaries() {
        let data: Vec<u8> = (0..=u8::MAX).cycle().take(CHUNK * 3 + 2).collect();
        for len in [
            0,
            1,
            2,
            3,
            CHUNK - 1,
            CHUNK,
            CHUNK + 1,
            CHUNK * 2 + 2,
            data.len(),
        ] {
            assert_eq!(base64(&data[..len]), reference(&data[..len]), "len {len}");
        }
    }

    #[test]
    fn detects_signatures() {
        let detected = |data: &[u8]| ImageData::new(data.to_vec()).map(|image| image.content_type);
        assert_eq!(detected(b"\xff\xd8\xff\xe0").unwrap(), "image/jpeg");
        assert_eq!(detected(b"GIF89a...").unwrap(), "image/gif");
        assert_eq!(detected(b"RIFF\0\0\0\0WEBPVP8 ").unwrap(), "image/webp");
        assert_eq!(detected(b"\0\0\0\x1cftypavif").unwrap(), "image/avif");
        assert_eq!(detected(b"RIFF\0\0\0\0WAVE"), Err(ImageDataError));
        assert_eq!(detected(b""), Err(ImageDataError));
    }

    #[test]
    fn debug_hides_the_contents() {
        let image = ImageData::with_content_type(&b"secret pixels"[..], "image/png");
        assert_eq!(
            format!("{image:?}"),
            r#"ImageData { content_type: "image/png", len: 13 }"#
        );
    }
}
