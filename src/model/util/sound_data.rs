use core::fmt;

use bytes::Bytes;
use compact_str::CompactString;
use serde::{Serialize, Serializer};

use super::image_data::write_base64;

#[derive(Clone, PartialEq, Eq, Hash)]
pub struct SoundData {
    data: Bytes,
    content_type: CompactString,
}

impl SoundData {
    pub fn new(data: impl Into<Bytes>) -> Result<Self, SoundDataError> {
        let data = data.into();
        let content_type = detect(&data).ok_or(SoundDataError)?;
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

impl fmt::Display for SoundData {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("data:")?;
        f.write_str(&self.content_type)?;
        f.write_str(";base64,")?;
        write_base64(&self.data, f)
    }
}

impl fmt::Debug for SoundData {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("SoundData")
            .field("content_type", &self.content_type)
            .field("len", &self.data.len())
            .finish()
    }
}

impl Serialize for SoundData {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        serializer.collect_str(self)
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[non_exhaustive]
pub struct SoundDataError;

impl fmt::Display for SoundDataError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("unknown sound format (expected MP3 or Ogg)")
    }
}

impl core::error::Error for SoundDataError {}

fn detect(data: &[u8]) -> Option<&'static str> {
    match data {
        [b'O', b'g', b'g', b'S', ..] => Some("audio/ogg"),

        [b'I', b'D', b'3', ..] => Some("audio/mpeg"),
        [0xff, second, ..] if second & 0xe0 == 0xe0 => Some("audio/mpeg"),
        _ => None,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn detects_signatures() {
        let detected = |data: &[u8]| SoundData::new(data.to_vec()).map(|sound| sound.content_type);
        assert_eq!(detected(b"ID3\x04\0").unwrap(), "audio/mpeg");
        assert_eq!(detected(b"\xff\xfb\x90\x64").unwrap(), "audio/mpeg");
        assert_eq!(detected(b"OggS").unwrap(), "audio/ogg");
        assert_eq!(detected(b"\xff\xd8\xff"), Err(SoundDataError));
        assert_eq!(detected(b"RIFF"), Err(SoundDataError));
        assert_eq!(detected(b""), Err(SoundDataError));
    }

    #[test]
    fn serializes_and_hides_the_contents() {
        let sound = SoundData::with_content_type(&b"abc"[..], "audio/mpeg");
        assert_eq!(
            serde_json::to_string(&sound).unwrap(),
            r#""data:audio/mpeg;base64,YWJj""#
        );
        assert_eq!(
            format!("{sound:?}"),
            r#"SoundData { content_type: "audio/mpeg", len: 3 }"#
        );
    }
}
