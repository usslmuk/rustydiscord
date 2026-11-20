use serde::Serialize;

use crate::model::{
    id::ChannelId,
    util::{Patch, Timestamp},
};

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash, Serialize)]
#[non_exhaustive]
pub struct ModifyCurrentUserVoiceState {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub channel_id: Option<ChannelId>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub suppress: Option<bool>,

    #[serde(skip_serializing_if = "Patch::is_keep")]
    pub request_to_speak_timestamp: Patch<Timestamp>,
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash, Serialize)]
#[non_exhaustive]
pub struct ModifyUserVoiceState {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub channel_id: Option<ChannelId>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub suppress: Option<bool>,
}
