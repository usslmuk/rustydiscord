use std::{ffi::OsStr, path::Path};

use tokio::io::AsyncRead;

use crate::{Call, Input, InputError, PlayState, TrackError, TrackHandle};

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
#[non_exhaustive]
pub enum AudioPlayerStatus {
    #[default]
    Idle,
    Playing,
    Paused,
}

#[derive(Debug)]
pub struct AudioResource {
    inner: Input,
}

impl AudioResource {
    #[must_use]
    pub fn from_ffmpeg(source: impl AsRef<OsStr>) -> Self {
        Self {
            inner: Input::ffmpeg(source),
        }
    }

    pub async fn from_file(path: impl AsRef<Path>) -> Result<Self, InputError> {
        Ok(Self {
            inner: Input::ogg_file(path).await?,
        })
    }

    #[must_use]
    pub fn from_opus(reader: impl AsyncRead + Send + Unpin + 'static) -> Self {
        Self {
            inner: Input::ogg(reader),
        }
    }

    #[must_use]
    pub fn into_input(self) -> Input {
        self.inner
    }
}

#[must_use]
pub fn create_audio_resource(input: Input) -> AudioResource {
    AudioResource { inner: input }
}

#[derive(Clone, Debug)]
pub struct AudioPlayer {
    call: Call,
    current: Option<TrackHandle>,
    paused: bool,
}

impl AudioPlayer {
    #[must_use]
    pub fn new(call: Call) -> Self {
        Self {
            call,
            current: None,
            paused: false,
        }
    }

    #[must_use]
    pub fn call(&self) -> &Call {
        &self.call
    }

    #[must_use]
    pub fn current(&self) -> Option<TrackHandle> {
        self.current.clone()
    }

    #[must_use]
    pub fn status(&self) -> AudioPlayerStatus {
        match &self.current {
            None => AudioPlayerStatus::Idle,
            Some(handle) if handle.info().state == PlayState::Ended => AudioPlayerStatus::Idle,
            Some(_) if self.paused => AudioPlayerStatus::Paused,
            Some(_) => AudioPlayerStatus::Playing,
        }
    }

    #[allow(
        clippy::must_use_candidate,
        reason = "playing without keeping the handle is fine"
    )]
    pub fn play(&mut self, resource: AudioResource) -> TrackHandle {
        let handle = self.call.play(resource.into_input());
        self.current = Some(handle.clone());
        self.paused = false;
        handle
    }

    #[must_use]
    pub fn pause(&mut self) -> bool {
        let Some(handle) = &self.current else {
            return false;
        };
        match handle.pause() {
            Ok(()) => {
                self.paused = true;
                true
            }
            Err(TrackError::Ended | TrackError::Unsupported) => false,
        }
    }

    #[must_use]
    pub fn unpause(&mut self) -> bool {
        let Some(handle) = &self.current else {
            return false;
        };
        match handle.play() {
            Ok(()) => {
                self.paused = false;
                true
            }
            Err(TrackError::Ended | TrackError::Unsupported) => false,
        }
    }

    #[must_use]
    pub fn stop(&mut self) -> bool {
        let Some(handle) = self.current.take() else {
            return false;
        };
        self.paused = false;
        handle.stop().is_ok()
    }
}

#[cfg(test)]
mod tests {
    use std::{collections::HashMap, error::Error, sync::Arc};

    use rustydiscord_model::{
        gateway::payload::UpdateVoiceState,
        id::{GuildId, UserId},
    };

    use super::{AudioPlayer, AudioPlayerStatus, AudioResource, create_audio_resource};
    use crate::{
        Input,
        call::{Calls, Spawn},
    };

    #[derive(Debug)]
    struct Gateway;

    impl crate::VoiceGateway for Gateway {
        fn send(
            &self,
            _shard_id: u32,
            _command: &UpdateVoiceState,
        ) -> Result<(), Box<dyn Error + Send + Sync>> {
            Ok(())
        }
    }

    fn call() -> crate::Call {
        crate::Call::spawn(Spawn {
            id: 0,
            guild_id: GuildId::new(1),
            user_id: UserId::new(1),
            shard_id: 0,
            gateway: Arc::new(Gateway),
            config: crate::connection::Config::default(),
            calls: Arc::downgrade(&Arc::new(Calls::new(HashMap::new()))),
        })
    }

    #[test]
    fn status_defaults_to_idle() {
        assert_eq!(AudioPlayerStatus::default(), AudioPlayerStatus::Idle);
    }

    #[test]
    fn resources_wrap_inputs() {
        let from_ffmpeg = AudioResource::from_ffmpeg("song.mp3");
        assert_eq!(format!("{:?}", from_ffmpeg.into_input()), "Input::Ffmpeg");
        let from_opus = AudioResource::from_opus(&b""[..]);
        assert_eq!(format!("{:?}", from_opus.into_input()), "Input::Ogg");
        let wrapped = create_audio_resource(Input::ffmpeg("song.mp3"));
        assert_eq!(format!("{:?}", wrapped.into_input()), "Input::Ffmpeg");
    }

    #[tokio::test]
    async fn player_reports_construction_and_status() {
        let spawned = call();
        let mut player = AudioPlayer::new(spawned.clone());
        assert_eq!(player.status(), AudioPlayerStatus::Idle);
        assert!(player.current().is_none());
        assert_eq!(player.call().guild_id(), spawned.guild_id());

        let resource = AudioResource::from_opus(&b""[..]);
        let handle = player.play(resource);
        assert_eq!(player.status(), AudioPlayerStatus::Playing);
        assert_eq!(player.current(), Some(handle));

        assert!(player.pause());
        assert_eq!(player.status(), AudioPlayerStatus::Paused);

        assert!(player.unpause());
        assert_eq!(player.status(), AudioPlayerStatus::Playing);

        assert!(player.stop());
        assert_eq!(player.status(), AudioPlayerStatus::Idle);
        assert!(player.current().is_none());
        assert!(!player.stop());
        assert!(!player.pause());
        assert!(!player.unpause());
    }
}
