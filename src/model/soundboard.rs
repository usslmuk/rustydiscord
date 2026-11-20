use compact_str::CompactString;
use serde::{Deserialize, Serialize};

use crate::model::{
    id::{EmojiId, GuildId, SoundboardSoundId},
    user::User,
};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct SoundboardSound {
    pub name: CompactString,

    pub sound_id: SoundboardSoundId,

    pub volume: f64,

    pub emoji_id: Option<EmojiId>,

    pub emoji_name: Option<CompactString>,

    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub guild_id: Option<GuildId>,

    #[serde(default = "crate::model::serde_util::default_true")]
    pub available: bool,

    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub user: Option<Box<User>>,
}
