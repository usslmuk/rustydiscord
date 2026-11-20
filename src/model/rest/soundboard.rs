use compact_str::CompactString;
use serde::Serialize;

use crate::model::{
    id::{EmojiId, GuildId, SoundboardSoundId},
    util::{Patch, SoundData},
};

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize)]
#[non_exhaustive]
pub struct SendSoundboardSound {
    pub sound_id: SoundboardSoundId,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_guild_id: Option<GuildId>,
}

impl SendSoundboardSound {
    #[must_use]
    pub const fn new(sound_id: SoundboardSoundId) -> Self {
        Self {
            sound_id,
            source_guild_id: None,
        }
    }
}

#[derive(Clone, Debug, PartialEq, Serialize)]
#[non_exhaustive]
pub struct CreateGuildSoundboardSound {
    pub name: CompactString,

    pub sound: SoundData,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub volume: Option<f64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub emoji_id: Option<EmojiId>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub emoji_name: Option<CompactString>,
}

impl CreateGuildSoundboardSound {
    #[must_use]
    pub fn new(name: impl Into<CompactString>, sound: SoundData) -> Self {
        Self {
            name: name.into(),
            sound,
            volume: None,
            emoji_id: None,
            emoji_name: None,
        }
    }
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[non_exhaustive]
pub struct ModifyGuildSoundboardSound {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<CompactString>,

    #[serde(skip_serializing_if = "Patch::is_keep")]
    pub volume: Patch<f64>,

    #[serde(skip_serializing_if = "Patch::is_keep")]
    pub emoji_id: Patch<EmojiId>,

    #[serde(skip_serializing_if = "Patch::is_keep")]
    pub emoji_name: Patch<CompactString>,
}
