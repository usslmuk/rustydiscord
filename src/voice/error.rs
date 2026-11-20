use core::fmt;
use std::{error::Error, io};

use crate::voice::ogg::OggError;

#[derive(Debug)]
#[non_exhaustive]
pub enum JoinError {
    Gateway(Box<dyn Error + Send + Sync>),

    Timeout,

    Connection(Box<dyn Error + Send + Sync>),

    Cancelled,
}

impl fmt::Display for JoinError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(match self {
            Self::Gateway(_) => "the voice state update could not be sent to the gateway",
            Self::Timeout => "joining the voice channel timed out",
            Self::Connection(_) => "the voice connection failed",
            Self::Cancelled => "the join was cancelled by a leave or another join",
        })
    }
}

impl Error for JoinError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            Self::Gateway(error) | Self::Connection(error) => Some(&**error),
            Self::Timeout | Self::Cancelled => None,
        }
    }
}

#[derive(Debug)]
#[non_exhaustive]
pub enum InputError {
    Io(io::Error),

    NotOgg,

    NotOpus,

    FfmpegMissing,

    #[non_exhaustive]
    Ffmpeg {
        code: Option<i32>,

        message: String,
    },
}

impl fmt::Display for InputError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Io(error) => write!(f, "reading the input failed: {error}"),
            Self::NotOgg => f.write_str("the input is not an Ogg stream"),
            Self::NotOpus => f.write_str("the Ogg stream holds no well-formed Opus audio"),
            Self::FfmpegMissing => f.write_str("ffmpeg is not installed (not found on PATH)"),
            Self::Ffmpeg { code, message } => {
                f.write_str("ffmpeg failed")?;
                if let Some(code) = code {
                    write!(f, " with exit code {code}")?;
                }
                if !message.is_empty() {
                    write!(f, ": {message}")?;
                }
                Ok(())
            }
        }
    }
}

impl Error for InputError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            Self::Io(error) => Some(error),
            Self::NotOgg | Self::NotOpus | Self::FfmpegMissing | Self::Ffmpeg { .. } => None,
        }
    }
}

impl From<io::Error> for InputError {
    fn from(error: io::Error) -> Self {
        Self::Io(error)
    }
}

impl From<OggError> for InputError {
    fn from(error: OggError) -> Self {
        match error {
            OggError::NotOgg => Self::NotOgg,
            OggError::NotOpus | OggError::BadHead | OggError::BadTags => Self::NotOpus,
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[non_exhaustive]
pub enum TrackError {
    Ended,

    Unsupported,
}

impl fmt::Display for TrackError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(match self {
            Self::Ended => "the track has ended",
            Self::Unsupported => "the track's input cannot be opened again to loop or seek",
        })
    }
}

impl Error for TrackError {}

#[cfg(test)]
mod tests {
    use std::error::Error;

    use super::{InputError, JoinError, TrackError};
    use crate::voice::ogg::OggError;

    #[test]
    fn errors_describe_themselves_and_their_causes() {
        let cause: Box<dyn Error + Send + Sync> = "shard closed".into();
        let error = JoinError::Gateway(cause);
        assert_eq!(
            error.to_string(),
            "the voice state update could not be sent to the gateway"
        );
        assert_eq!(error.source().unwrap().to_string(), "shard closed");
        assert!(JoinError::Timeout.source().is_none());
        let io = InputError::from(std::io::Error::other("disk"));
        assert_eq!(io.to_string(), "reading the input failed: disk");
        assert!(io.source().is_some());
        assert_eq!(TrackError::Ended.to_string(), "the track has ended");
        assert_eq!(
            InputError::Ffmpeg {
                code: Some(1),
                message: "a.mp3: No such file or directory".into()
            }
            .to_string(),
            "ffmpeg failed with exit code 1: a.mp3: No such file or directory"
        );
        assert_eq!(
            InputError::Ffmpeg {
                code: None,
                message: String::new()
            }
            .to_string(),
            "ffmpeg failed"
        );
    }

    #[test]
    fn ogg_errors_become_input_errors() {
        assert!(matches!(
            InputError::from(OggError::NotOgg),
            InputError::NotOgg
        ));
        for error in [OggError::NotOpus, OggError::BadHead, OggError::BadTags] {
            assert!(matches!(InputError::from(error), InputError::NotOpus));
        }
    }
}
