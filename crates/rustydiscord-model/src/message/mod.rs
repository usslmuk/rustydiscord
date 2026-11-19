mod attachment;
mod embed;
mod interaction_metadata;

use core::{fmt, ops::Deref};

use compact_str::CompactString;
use serde::{
    Deserialize, Deserializer, Serialize, Serializer,
    de::{Error as DeError, Visitor},
};

pub use self::{
    attachment::{Attachment, AttachmentRequest, AttachmentRequestId},
    embed::{
        Embed, EmbedAuthor, EmbedField, EmbedFooter, EmbedImage, EmbedProvider, EmbedType,
        EmbedVideo,
    },
    interaction_metadata::{
        CommandMetadata, ComponentMetadata, InteractionMetadata, ModalSubmitMetadata,
        UnknownMetadata,
    },
};
use crate::{
    application::PartialApplication,
    channel::{Channel, ChannelType},
    component::Component,
    emoji::PartialEmoji,
    guild::PartialMember,
    id::{ApplicationId, ChannelId, GuildId, MessageId, RoleId, SkuId, UserId, WebhookId},
    interaction::{MessageInteraction, Resolved},
    macros::{int_enum, int_flags, str_enum},
    poll::Poll,
    sticker::{Sticker, StickerItem},
    user::{AvatarDecorationData, Collectibles, PrimaryGuild, User, UserFlags},
    util::{ImageHash, Timestamp},
};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct Message {
    pub id: MessageId,

    pub channel_id: ChannelId,

    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub guild_id: Option<GuildId>,

    pub author: User,

    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub member: Option<Box<PartialMember>>,

    pub content: CompactString,

    pub timestamp: Timestamp,

    pub edited_timestamp: Option<Timestamp>,

    pub tts: bool,

    pub mention_everyone: bool,

    #[serde(default)]
    pub mentions: Vec<Mention>,

    #[serde(default)]
    pub mention_roles: Vec<RoleId>,

    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub mention_channels: Vec<ChannelMention>,

    #[serde(default)]
    pub attachments: Vec<Attachment>,

    #[serde(default)]
    pub embeds: Vec<Embed>,

    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub reactions: Vec<Reaction>,

    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub nonce: Option<Nonce>,

    pub pinned: bool,

    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub webhook_id: Option<WebhookId>,

    #[serde(rename = "type")]
    pub kind: MessageType,

    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub activity: Option<Box<MessageActivity>>,

    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub application: Option<Box<PartialApplication>>,

    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub application_id: Option<ApplicationId>,

    #[serde(default)]
    pub flags: MessageFlags,

    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub message_reference: Option<MessageReference>,

    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub message_snapshots: Vec<MessageSnapshot>,

    #[serde(default, skip_serializing_if = "ReferencedMessage::is_not_fetched")]
    pub referenced_message: ReferencedMessage,

    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub interaction_metadata: Option<Box<InteractionMetadata>>,

    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub interaction: Option<Box<MessageInteraction>>,

    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub components: Vec<Component>,

    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub thread: Option<Box<Channel>>,

    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub resolved: Option<Box<Resolved>>,

    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub sticker_items: Vec<StickerItem>,

    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub stickers: Vec<Sticker>,

    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub position: Option<u32>,

    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub role_subscription_data: Option<Box<RoleSubscriptionData>>,

    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub poll: Option<Box<Poll>>,

    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub call: Option<Box<MessageCall>>,

    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub shared_client_theme: Option<Box<SharedClientTheme>>,

    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub channel_type: Option<ChannelType>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
#[non_exhaustive]
pub struct Mention {
    pub user: User,

    pub member: Option<Box<PartialMember>>,
}

impl Mention {
    #[must_use]
    pub const fn new(user: User) -> Self {
        Self { user, member: None }
    }
}

impl Deref for Mention {
    type Target = User;

    fn deref(&self) -> &User {
        &self.user
    }
}

impl From<User> for Mention {
    fn from(user: User) -> Self {
        Self::new(user)
    }
}

#[derive(Deserialize)]
struct RawMention {
    id: UserId,
    username: CompactString,
    #[serde(with = "crate::user::discriminator")]
    discriminator: u16,
    global_name: Option<CompactString>,
    avatar: Option<ImageHash>,
    #[serde(default)]
    bot: bool,
    #[serde(default)]
    system: bool,
    #[serde(default)]
    banner: Option<ImageHash>,
    #[serde(default)]
    accent_color: Option<u32>,
    #[serde(default)]
    public_flags: Option<UserFlags>,
    #[serde(default)]
    avatar_decoration_data: Option<Box<AvatarDecorationData>>,
    #[serde(default)]
    collectibles: Option<Box<Collectibles>>,
    #[serde(default)]
    primary_guild: Option<Box<PrimaryGuild>>,
    #[serde(default)]
    member: Option<Box<PartialMember>>,
}

impl<'de> Deserialize<'de> for Mention {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let raw = RawMention::deserialize(deserializer)?;
        Ok(Self {
            user: User {
                id: raw.id,
                username: raw.username,
                discriminator: raw.discriminator,
                global_name: raw.global_name,
                avatar: raw.avatar,
                bot: raw.bot,
                system: raw.system,
                banner: raw.banner,
                accent_color: raw.accent_color,
                public_flags: raw.public_flags,
                avatar_decoration_data: raw.avatar_decoration_data,
                collectibles: raw.collectibles,
                primary_guild: raw.primary_guild,
            },
            member: raw.member,
        })
    }
}

impl Serialize for Mention {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        #[derive(Serialize)]
        struct Flat<'a> {
            #[serde(flatten)]
            user: &'a User,
            #[serde(skip_serializing_if = "Option::is_none")]
            member: Option<&'a PartialMember>,
        }

        Flat {
            user: &self.user,
            member: self.member.as_deref(),
        }
        .serialize(serializer)
    }
}

#[derive(Clone, Debug, Default, PartialEq)]
#[non_exhaustive]
pub enum ReferencedMessage {
    #[default]
    NotFetched,

    Deleted,

    Message(Box<Message>),
}

impl ReferencedMessage {
    #[must_use]
    pub fn as_message(&self) -> Option<&Message> {
        match self {
            Self::Message(message) => Some(message),
            Self::NotFetched | Self::Deleted => None,
        }
    }

    #[must_use]
    pub fn into_message(self) -> Option<Box<Message>> {
        match self {
            Self::Message(message) => Some(message),
            Self::NotFetched | Self::Deleted => None,
        }
    }

    #[must_use]
    pub const fn is_not_fetched(&self) -> bool {
        matches!(self, Self::NotFetched)
    }

    #[must_use]
    pub const fn is_deleted(&self) -> bool {
        matches!(self, Self::Deleted)
    }
}

impl Serialize for ReferencedMessage {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::Message(message) => message.serialize(serializer),
            Self::NotFetched | Self::Deleted => serializer.serialize_none(),
        }
    }
}

impl<'de> Deserialize<'de> for ReferencedMessage {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        Ok(match Option::<Box<Message>>::deserialize(deserializer)? {
            Some(message) => Self::Message(message),
            None => Self::Deleted,
        })
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum Nonce {
    Integer(i64),

    String(CompactString),
}

impl Serialize for Nonce {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::Integer(value) => serializer.serialize_i64(*value),
            Self::String(value) => serializer.serialize_str(value),
        }
    }
}

impl<'de> Deserialize<'de> for Nonce {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        struct NonceVisitor;

        impl Visitor<'_> for NonceVisitor {
            type Value = Nonce;

            fn expecting(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                f.write_str("an integer or string nonce")
            }

            fn visit_i64<E: DeError>(self, value: i64) -> Result<Nonce, E> {
                Ok(Nonce::Integer(value))
            }

            fn visit_u64<E: DeError>(self, value: u64) -> Result<Nonce, E> {
                Ok(i64::try_from(value).map_or_else(
                    |_| Nonce::String(compact_str::format_compact!("{value}")),
                    Nonce::Integer,
                ))
            }

            fn visit_str<E: DeError>(self, value: &str) -> Result<Nonce, E> {
                Ok(Nonce::String(value.into()))
            }
        }

        deserializer.deserialize_any(NonceVisitor)
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[non_exhaustive]
pub struct MessageReference {
    #[serde(rename = "type", default)]
    pub kind: MessageReferenceType,

    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub message_id: Option<MessageId>,

    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub channel_id: Option<ChannelId>,

    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub guild_id: Option<GuildId>,

    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub fail_if_not_exists: Option<bool>,
}

impl MessageReference {
    #[must_use]
    pub const fn reply(message_id: MessageId) -> Self {
        Self {
            kind: MessageReferenceType::Default,
            message_id: Some(message_id),
            channel_id: None,
            guild_id: None,
            fail_if_not_exists: None,
        }
    }
}

int_enum! {



    #[derive(Default)]
    pub enum MessageReferenceType: u8 {

        #[default]
        Default = 0,


        Forward = 1,
    }
}

int_enum! {



    pub enum MessageType: u8 {

        Default = 0,

        RecipientAdd = 1,

        RecipientRemove = 2,

        Call = 3,

        ChannelNameChange = 4,

        ChannelIconChange = 5,

        ChannelPinnedMessage = 6,

        UserJoin = 7,

        GuildBoost = 8,

        GuildBoostTier1 = 9,

        GuildBoostTier2 = 10,

        GuildBoostTier3 = 11,

        ChannelFollowAdd = 12,

        GuildDiscoveryDisqualified = 14,

        GuildDiscoveryRequalified = 15,

        GuildDiscoveryGracePeriodInitialWarning = 16,

        GuildDiscoveryGracePeriodFinalWarning = 17,

        ThreadCreated = 18,

        Reply = 19,

        ChatInputCommand = 20,

        ThreadStarterMessage = 21,

        GuildInviteReminder = 22,

        ContextMenuCommand = 23,

        AutoModerationAction = 24,

        RoleSubscriptionPurchase = 25,

        InteractionPremiumUpsell = 26,

        StageStart = 27,

        StageEnd = 28,

        StageSpeaker = 29,



        StageRaiseHand = 30,

        StageTopic = 31,

        GuildApplicationPremiumSubscription = 32,

        GuildIncidentAlertModeEnabled = 36,

        GuildIncidentAlertModeDisabled = 37,

        GuildIncidentReportRaid = 38,

        GuildIncidentReportFalseAlarm = 39,

        PurchaseNotification = 44,

        PollResult = 46,
    }
}

impl MessageType {
    #[must_use]
    pub const fn is_deletable(self) -> bool {
        !matches!(
            self,
            Self::RecipientAdd
                | Self::RecipientRemove
                | Self::Call
                | Self::ChannelNameChange
                | Self::ChannelIconChange
                | Self::ThreadStarterMessage
        )
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[non_exhaustive]
pub struct MessageActivity {
    #[serde(rename = "type")]
    pub kind: MessageActivityType,

    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub party_id: Option<CompactString>,
}

int_enum! {



    pub enum MessageActivityType: u8 {

        Join = 1,

        Spectate = 2,

        Listen = 3,

        JoinRequest = 5,

        StreamRequest = 6,
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[non_exhaustive]
pub struct MessageCall {
    pub participants: Vec<UserId>,

    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ended_timestamp: Option<Timestamp>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct MessageSnapshot {
    pub message: SnapshotMessage,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
#[doc(alias = "MessageSnapshotFields")]
pub struct SnapshotMessage {
    #[serde(rename = "type")]
    pub kind: MessageType,

    #[serde(default)]
    pub content: CompactString,

    #[serde(default)]
    pub embeds: Vec<Embed>,

    #[serde(default)]
    pub attachments: Vec<Attachment>,

    pub timestamp: Timestamp,

    #[serde(default)]
    pub edited_timestamp: Option<Timestamp>,

    #[serde(default)]
    pub flags: MessageFlags,

    #[serde(default)]
    pub mentions: Vec<User>,

    #[serde(default)]
    pub mention_roles: Vec<RoleId>,

    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub stickers: Vec<Sticker>,

    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub sticker_items: Vec<StickerItem>,

    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub components: Vec<Component>,
}

int_enum! {




    #[derive(Default)]
    pub enum ReactionType: u8 {

        #[default]
        Normal = 0,

        Burst = 1,
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[non_exhaustive]
#[doc(alias = "MessageReaction")]
pub struct Reaction {
    pub count: u32,

    pub count_details: ReactionCountDetails,

    pub me: bool,

    pub me_burst: bool,

    pub emoji: PartialEmoji,

    #[serde(default)]
    pub burst_colors: Vec<CompactString>,
}

impl Reaction {
    #[must_use]
    pub fn new(emoji: PartialEmoji) -> Self {
        Self {
            count: 0,
            count_details: ReactionCountDetails::default(),
            me: false,
            me_burst: false,
            emoji,
            burst_colors: Vec::new(),
        }
    }
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[non_exhaustive]
pub struct ReactionCountDetails {
    pub burst: u32,

    pub normal: u32,
}

#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[non_exhaustive]
pub struct ChannelMention {
    pub id: ChannelId,

    pub guild_id: GuildId,

    #[serde(rename = "type")]
    pub kind: ChannelType,

    pub name: CompactString,
}

#[derive(Clone, Debug, Default, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[non_exhaustive]
pub struct AllowedMentions {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub parse: Vec<AllowedMentionType>,

    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub roles: Vec<RoleId>,

    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub users: Vec<UserId>,

    #[serde(default, skip_serializing_if = "core::ops::Not::not")]
    pub replied_user: bool,
}

str_enum! {



    #[doc(alias = "AllowedMentionsTypes")]
    pub enum AllowedMentionType {

        Roles = "roles",

        Users = "users",

        Everyone = "everyone",
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[non_exhaustive]
pub struct RoleSubscriptionData {
    pub role_subscription_listing_id: SkuId,

    pub tier_name: CompactString,

    pub total_months_subscribed: u32,

    pub is_renewal: bool,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct MessagePin {
    pub pinned_at: Timestamp,

    pub message: Message,
}

#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[non_exhaustive]
pub struct SharedClientTheme {
    pub colors: Vec<CompactString>,

    pub gradient_angle: u16,

    pub base_mix: u8,

    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub base_theme: Option<BaseTheme>,
}

int_enum! {



    pub enum BaseTheme: u8 {

        Unset = 0,

        Dark = 1,

        Light = 2,

        Darker = 3,

        Midnight = 4,
    }
}

int_flags! {

















    pub struct MessageFlags: u64 {

        const CROSSPOSTED = 1 << 0;

        const IS_CROSSPOST = 1 << 1;

        const SUPPRESS_EMBEDS = 1 << 2;

        const SOURCE_MESSAGE_DELETED = 1 << 3;

        const URGENT = 1 << 4;

        const HAS_THREAD = 1 << 5;

        const EPHEMERAL = 1 << 6;

        const LOADING = 1 << 7;


        const FAILED_TO_MENTION_SOME_ROLES_IN_THREAD = 1 << 8;




        const SHOULD_SHOW_LINK_NOT_DISCORD_WARNING = 1 << 10;

        const SUPPRESS_NOTIFICATIONS = 1 << 12;

        const IS_VOICE_MESSAGE = 1 << 13;

        const HAS_SNAPSHOT = 1 << 14;



        const IS_COMPONENTS_V2 = 1 << 15;
    }
}

int_flags! {



    pub struct AttachmentFlags: u64 {

        const IS_CLIP = 1 << 0;


        const IS_THUMBNAIL = 1 << 1;

        const IS_REMIX = 1 << 2;

        const IS_SPOILER = 1 << 3;

        const CONTAINS_EXPLICIT_CONTENT = 1 << 4;

        const IS_ANIMATED = 1 << 5;

        const CONTAINS_GORE_CONTENT = 1 << 6;

        const CONTAINS_SELF_HARM_CONTENT = 1 << 7;
    }
}

int_flags! {





    pub struct EmbedFlags: u64 {

        const CONTAINS_EXPLICIT_CONTENT = 1 << 4;

        const IS_CONTENT_INVENTORY_ENTRY = 1 << 5;

        const CONTAINS_GORE_CONTENT = 1 << 6;

        const CONTAINS_SELF_HARM_CONTENT = 1 << 7;
    }
}

int_flags! {



    pub struct EmbedMediaFlags: u64 {

        const IS_ANIMATED = 1 << 5;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sizes() {
        assert!(size_of::<Message>() <= 656, "{}", size_of::<Message>());
        assert!(size_of::<Mention>() <= 152, "{}", size_of::<Mention>());
        assert!(
            size_of::<Attachment>() <= 312,
            "{}",
            size_of::<Attachment>()
        );
        assert!(size_of::<Embed>() <= 232, "{}", size_of::<Embed>());
        assert_eq!(size_of::<ReferencedMessage>(), 16);
    }

    #[test]
    fn message_type_forward_compatible() {
        assert_eq!(MessageType::from(19), MessageType::Reply);
        assert_eq!(MessageType::from(250), MessageType::Unknown(250));
        assert!(MessageType::from(250).is_unknown());
        assert!(!MessageType::Reply.is_unknown());

        assert_eq!(MessageType::Unknown(0), MessageType::Default);
        assert!(!MessageType::Unknown(0).is_unknown());
        assert_eq!(serde_json::to_string(&MessageType::Reply).unwrap(), "19");
    }
}
