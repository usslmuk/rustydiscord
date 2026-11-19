use compact_str::CompactString;
use serde::{Deserialize, Serialize};

use super::newtype_event;
use crate::{
    emoji::PartialEmoji,
    entitlement::Entitlement,
    gateway::Presence,
    guild::Member,
    id::{ChannelId, GuildId, MessageId, UserId},
    interaction::Interaction,
    message::{Message, ReactionType},
    subscription::Subscription,
    user::CurrentUser,
    util::Timestamp,
};

newtype_event! {




    MessageCreate(Message)
}

newtype_event! {




    MessageUpdate(Message)
}

#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[non_exhaustive]
pub struct MessageDelete {
    pub id: MessageId,

    pub channel_id: ChannelId,

    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub guild_id: Option<GuildId>,
}

#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[non_exhaustive]
pub struct MessageDeleteBulk {
    pub ids: Vec<MessageId>,

    pub channel_id: ChannelId,

    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub guild_id: Option<GuildId>,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct MessageReactionAdd {
    pub user_id: UserId,

    pub channel_id: ChannelId,

    pub message_id: MessageId,

    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub guild_id: Option<GuildId>,

    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub member: Option<Box<Member>>,

    pub emoji: PartialEmoji,

    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub message_author_id: Option<UserId>,

    #[serde(default)]
    pub burst: bool,

    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub burst_colors: Vec<CompactString>,

    #[serde(default, rename = "type")]
    pub kind: ReactionType,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct MessageReactionRemove {
    pub user_id: UserId,

    pub channel_id: ChannelId,

    pub message_id: MessageId,

    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub guild_id: Option<GuildId>,

    pub emoji: PartialEmoji,

    #[serde(default)]
    pub burst: bool,

    #[serde(default, rename = "type")]
    pub kind: ReactionType,
}

#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[non_exhaustive]
pub struct MessageReactionRemoveAll {
    pub channel_id: ChannelId,

    pub message_id: MessageId,

    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub guild_id: Option<GuildId>,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct MessageReactionRemoveEmoji {
    pub channel_id: ChannelId,

    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub guild_id: Option<GuildId>,

    pub message_id: MessageId,

    pub emoji: PartialEmoji,
}

#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[non_exhaustive]
pub struct MessagePollVoteAdd {
    pub user_id: UserId,

    pub channel_id: ChannelId,

    pub message_id: MessageId,

    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub guild_id: Option<GuildId>,

    pub answer_id: u32,
}

#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[non_exhaustive]
pub struct MessagePollVoteRemove {
    pub user_id: UserId,

    pub channel_id: ChannelId,

    pub message_id: MessageId,

    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub guild_id: Option<GuildId>,

    pub answer_id: u32,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct TypingStart {
    pub channel_id: ChannelId,

    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub guild_id: Option<GuildId>,

    pub user_id: UserId,

    #[serde(with = "crate::serde_util::unix_secs")]
    pub timestamp: Timestamp,

    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub member: Option<Box<Member>>,
}

newtype_event! {




    InteractionCreate(Interaction)
}

newtype_event! {




    PresenceUpdate(Presence)
}

newtype_event! {




    UserUpdate(CurrentUser)
}

newtype_event! {




    EntitlementCreate(Entitlement)
}

newtype_event! {




    EntitlementUpdate(Entitlement)
}

newtype_event! {




    EntitlementDelete(Entitlement)
}

newtype_event! {




    SubscriptionCreate(Subscription)
}

newtype_event! {




    SubscriptionUpdate(Subscription)
}

newtype_event! {




    SubscriptionDelete(Subscription)
}
