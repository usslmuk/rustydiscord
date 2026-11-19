mod data;
mod getters;
mod response;

use core::{fmt, ops::Deref};

use compact_str::CompactString;
use serde::{
    Deserialize, Deserializer, Serialize, Serializer,
    de::{Error as DeError, Unexpected, Visitor},
};

pub use self::{
    data::{
        CommandData, CommandDataOption, CommandOptionValue, ComponentData, InteractionData,
        ModalSubmitData, Resolved, SubmittedActionRow, SubmittedCheckbox, SubmittedComponent,
        SubmittedEntitySelect, SubmittedFileUpload, SubmittedLabel, SubmittedRadioGroup,
        SubmittedSelect, SubmittedTextDisplay, SubmittedTextInput, UnknownSubmittedComponent,
    },
    getters::Mentionable,
    response::{
        ActivityInstanceResource, AutocompleteCallbackData, AutocompleteChoices, CallbackResource,
        DeferredCallbackData, InteractionCallback, InteractionCallbackResponse,
        InteractionCallbackType, InteractionResponse, MessageCallbackData, ModalCallbackData,
    },
};
use crate::{
    channel::{ChannelFlags, ChannelType, ThreadMetadata},
    entitlement::Entitlement,
    guild::{Feature, MemberFlags, PartialMember},
    id::{ApplicationId, ChannelId, GuildId, InteractionId, MessageId, RoleId, UserId},
    macros::int_enum,
    message::Message,
    permissions::Permissions,
    user::{AvatarDecorationData, Collectibles, User},
    util::{ImageHash, Locale, Timestamp},
};

#[derive(Clone, PartialEq)]
#[non_exhaustive]
pub struct Interaction {
    pub id: InteractionId,

    pub application_id: ApplicationId,

    pub data: InteractionData,

    pub guild: Option<InteractionGuild>,

    pub guild_id: Option<GuildId>,

    pub channel: Option<Box<InteractionChannel>>,

    pub channel_id: Option<ChannelId>,

    pub member: Option<Box<InteractionMember>>,

    pub user: Option<Box<User>>,

    pub token: CompactString,

    pub version: u8,

    pub message: Option<Box<Message>>,

    pub app_permissions: Permissions,

    pub locale: Option<Locale>,

    pub guild_locale: Option<Locale>,

    pub entitlements: Vec<Entitlement>,

    pub authorizing_integration_owners: AuthorizingIntegrationOwners,

    pub context: Option<InteractionContextType>,

    pub attachment_size_limit: Option<u64>,
}

impl Interaction {
    #[must_use]
    pub const fn kind(&self) -> InteractionType {
        self.data.kind()
    }

    #[must_use]
    pub fn author(&self) -> Option<&User> {
        self.member
            .as_ref()
            .and_then(|member| member.user.as_ref())
            .or(self.user.as_deref())
    }

    #[must_use]
    pub fn command(&self) -> Option<&CommandData> {
        match &self.data {
            InteractionData::ApplicationCommand(data) => Some(data),
            _ => None,
        }
    }

    #[must_use]
    pub fn autocomplete(&self) -> Option<&CommandData> {
        match &self.data {
            InteractionData::Autocomplete(data) => Some(data),
            _ => None,
        }
    }

    #[must_use]
    pub fn component(&self) -> Option<&ComponentData> {
        match &self.data {
            InteractionData::MessageComponent(data) => Some(data),
            _ => None,
        }
    }

    #[must_use]
    pub fn modal(&self) -> Option<&ModalSubmitData> {
        match &self.data {
            InteractionData::ModalSubmit(data) => Some(data),
            _ => None,
        }
    }
}

impl fmt::Debug for Interaction {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Interaction")
            .field("id", &self.id)
            .field("application_id", &self.application_id)
            .field("data", &self.data)
            .field("guild", &self.guild)
            .field("guild_id", &self.guild_id)
            .field("channel", &self.channel)
            .field("channel_id", &self.channel_id)
            .field("member", &self.member)
            .field("user", &self.user)
            .field("token", &"<redacted>")
            .field("version", &self.version)
            .field("message", &self.message)
            .field("app_permissions", &self.app_permissions)
            .field("locale", &self.locale)
            .field("guild_locale", &self.guild_locale)
            .field("entitlements", &self.entitlements)
            .field(
                "authorizing_integration_owners",
                &self.authorizing_integration_owners,
            )
            .field("context", &self.context)
            .field("attachment_size_limit", &self.attachment_size_limit)
            .finish()
    }
}

#[derive(Deserialize)]
struct RawInteraction {
    id: InteractionId,
    application_id: ApplicationId,
    #[serde(rename = "type")]
    kind: InteractionType,
    #[serde(default)]
    data: Option<data::RawData>,
    #[serde(default)]
    guild: Option<InteractionGuild>,
    #[serde(default)]
    guild_id: Option<GuildId>,
    #[serde(default)]
    channel: Option<Box<InteractionChannel>>,
    #[serde(default)]
    channel_id: Option<ChannelId>,
    #[serde(default)]
    member: Option<Box<InteractionMember>>,
    #[serde(default)]
    user: Option<Box<User>>,
    token: CompactString,
    #[serde(default = "default_version")]
    version: u8,
    #[serde(default)]
    message: Option<Box<Message>>,
    #[serde(default)]
    app_permissions: Permissions,
    #[serde(default)]
    locale: Option<Locale>,
    #[serde(default)]
    guild_locale: Option<Locale>,
    #[serde(default)]
    entitlements: Vec<Entitlement>,
    #[serde(default)]
    authorizing_integration_owners: AuthorizingIntegrationOwners,
    #[serde(default)]
    context: Option<InteractionContextType>,
    #[serde(default)]
    attachment_size_limit: Option<u64>,
}

const fn default_version() -> u8 {
    1
}

impl<'de> Deserialize<'de> for Interaction {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let raw = RawInteraction::deserialize(deserializer)?;
        Ok(Self {
            id: raw.id,
            application_id: raw.application_id,
            data: data::RawData::into_data(raw.data, raw.kind)?,
            guild: raw.guild,
            guild_id: raw.guild_id,
            channel: raw.channel,
            channel_id: raw.channel_id,
            member: raw.member,
            user: raw.user,
            token: raw.token,
            version: raw.version,
            message: raw.message,
            app_permissions: raw.app_permissions,
            locale: raw.locale,
            guild_locale: raw.guild_locale,
            entitlements: raw.entitlements,
            authorizing_integration_owners: raw.authorizing_integration_owners,
            context: raw.context,
            attachment_size_limit: raw.attachment_size_limit,
        })
    }
}

impl Serialize for Interaction {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        #[derive(Serialize)]
        struct Flat<'a> {
            id: InteractionId,
            application_id: ApplicationId,
            #[serde(rename = "type")]
            kind: InteractionType,
            #[serde(skip_serializing_if = "Option::is_none")]
            data: Option<data::DataRef<'a>>,
            #[serde(skip_serializing_if = "Option::is_none")]
            guild: Option<&'a InteractionGuild>,
            #[serde(skip_serializing_if = "Option::is_none")]
            guild_id: Option<GuildId>,
            #[serde(skip_serializing_if = "Option::is_none")]
            channel: Option<&'a InteractionChannel>,
            #[serde(skip_serializing_if = "Option::is_none")]
            channel_id: Option<ChannelId>,
            #[serde(skip_serializing_if = "Option::is_none")]
            member: Option<&'a InteractionMember>,
            #[serde(skip_serializing_if = "Option::is_none")]
            user: Option<&'a User>,
            token: &'a str,
            version: u8,
            #[serde(skip_serializing_if = "Option::is_none")]
            message: Option<&'a Message>,
            app_permissions: Permissions,
            #[serde(skip_serializing_if = "Option::is_none")]
            locale: Option<&'a Locale>,
            #[serde(skip_serializing_if = "Option::is_none")]
            guild_locale: Option<&'a Locale>,
            entitlements: &'a [Entitlement],
            authorizing_integration_owners: &'a AuthorizingIntegrationOwners,
            #[serde(skip_serializing_if = "Option::is_none")]
            context: Option<InteractionContextType>,
            #[serde(skip_serializing_if = "Option::is_none")]
            attachment_size_limit: Option<u64>,
        }

        Flat {
            id: self.id,
            application_id: self.application_id,
            kind: self.kind(),
            data: self.data.as_ref(),
            guild: self.guild.as_ref(),
            guild_id: self.guild_id,
            channel: self.channel.as_deref(),
            channel_id: self.channel_id,
            member: self.member.as_deref(),
            user: self.user.as_deref(),
            token: &self.token,
            version: self.version,
            message: self.message.as_deref(),
            app_permissions: self.app_permissions,
            locale: self.locale.as_ref(),
            guild_locale: self.guild_locale.as_ref(),
            entitlements: &self.entitlements,
            authorizing_integration_owners: &self.authorizing_integration_owners,
            context: self.context,
            attachment_size_limit: self.attachment_size_limit,
        }
        .serialize(serializer)
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct InteractionGuild {
    pub id: GuildId,

    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub locale: Option<Locale>,

    #[serde(default)]
    pub features: Vec<Feature>,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct InteractionChannel {
    pub id: ChannelId,

    #[serde(rename = "type")]
    pub kind: ChannelType,

    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<CompactString>,

    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub guild_id: Option<GuildId>,

    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub permissions: Option<Permissions>,

    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub app_permissions: Option<Permissions>,

    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub parent_id: Option<ChannelId>,

    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub position: Option<i32>,

    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub topic: Option<CompactString>,

    #[serde(default, skip_serializing_if = "core::ops::Not::not")]
    pub nsfw: bool,

    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub last_message_id: Option<MessageId>,

    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub last_pin_timestamp: Option<Timestamp>,

    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub rate_limit_per_user: Option<u16>,

    #[serde(default)]
    pub flags: ChannelFlags,

    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub thread_metadata: Option<ThreadMetadata>,

    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub recipients: Vec<User>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
#[non_exhaustive]
pub struct InteractionMember {
    pub member: PartialMember,

    pub user: Option<User>,

    pub permissions: Permissions,
}

impl Deref for InteractionMember {
    type Target = PartialMember;

    fn deref(&self) -> &PartialMember {
        &self.member
    }
}

#[derive(Deserialize)]
struct RawInteractionMember {
    #[serde(default)]
    nick: Option<CompactString>,
    #[serde(default)]
    avatar: Option<ImageHash>,
    #[serde(default)]
    banner: Option<ImageHash>,
    roles: Vec<RoleId>,
    joined_at: Option<Timestamp>,
    #[serde(default)]
    premium_since: Option<Timestamp>,
    #[serde(default)]
    deaf: bool,
    #[serde(default)]
    mute: bool,
    #[serde(default)]
    flags: MemberFlags,
    #[serde(default)]
    pending: bool,
    #[serde(default)]
    communication_disabled_until: Option<Timestamp>,
    #[serde(default)]
    avatar_decoration_data: Option<Box<AvatarDecorationData>>,
    #[serde(default)]
    collectibles: Option<Box<Collectibles>>,
    #[serde(default)]
    user: Option<User>,
    #[serde(default)]
    permissions: Permissions,
}

impl<'de> Deserialize<'de> for InteractionMember {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let raw = RawInteractionMember::deserialize(deserializer)?;
        Ok(Self {
            member: PartialMember {
                nick: raw.nick,
                avatar: raw.avatar,
                banner: raw.banner,
                roles: raw.roles,
                joined_at: raw.joined_at,
                premium_since: raw.premium_since,
                deaf: raw.deaf,
                mute: raw.mute,
                flags: raw.flags,
                pending: raw.pending,
                communication_disabled_until: raw.communication_disabled_until,
                avatar_decoration_data: raw.avatar_decoration_data,
                collectibles: raw.collectibles,
            },
            user: raw.user,
            permissions: raw.permissions,
        })
    }
}

impl Serialize for InteractionMember {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        #[derive(Serialize)]
        struct Flat<'a> {
            #[serde(flatten)]
            member: &'a PartialMember,
            #[serde(skip_serializing_if = "Option::is_none")]
            user: Option<&'a User>,
            permissions: Permissions,
        }

        Flat {
            member: &self.member,
            user: self.user.as_ref(),
            permissions: self.permissions,
        }
        .serialize(serializer)
    }
}

int_enum! {



    pub enum InteractionType: u8 {

        Ping = 1,

        ApplicationCommand = 2,

        MessageComponent = 3,

        ApplicationCommandAutocomplete = 4,

        ModalSubmit = 5,
    }
}

int_enum! {




    pub enum InteractionContextType: u8 {

        Guild = 0,

        BotDm = 1,

        PrivateChannel = 2,
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct MessageInteraction {
    pub id: InteractionId,

    #[serde(rename = "type")]
    pub kind: InteractionType,

    pub name: CompactString,

    pub user: User,

    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub member: Option<Box<PartialMember>>,
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[non_exhaustive]
pub struct AuthorizingIntegrationOwners {
    #[serde(rename = "0", default, skip_serializing_if = "Option::is_none")]
    pub guild_install: Option<GuildInstallOwner>,

    #[serde(rename = "1", default, skip_serializing_if = "Option::is_none")]
    pub user_install: Option<UserId>,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[non_exhaustive]
pub enum GuildInstallOwner {
    Guild(GuildId),

    BotDm,
}

impl GuildInstallOwner {
    #[must_use]
    pub const fn guild_id(self) -> Option<GuildId> {
        match self {
            Self::Guild(id) => Some(id),
            Self::BotDm => None,
        }
    }
}

impl Serialize for GuildInstallOwner {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::Guild(id) => id.serialize(serializer),
            Self::BotDm => serializer.serialize_str("0"),
        }
    }
}

impl<'de> Deserialize<'de> for GuildInstallOwner {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        struct OwnerVisitor;

        impl Visitor<'_> for OwnerVisitor {
            type Value = GuildInstallOwner;

            fn expecting(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                f.write_str("a guild ID or \"0\"")
            }

            fn visit_str<E: DeError>(self, value: &str) -> Result<GuildInstallOwner, E> {
                if value == "0" {
                    return Ok(GuildInstallOwner::BotDm);
                }
                value
                    .parse()
                    .map(GuildInstallOwner::Guild)
                    .map_err(|_| E::invalid_value(Unexpected::Str(value), &self))
            }
        }

        deserializer.deserialize_str(OwnerVisitor)
    }
}

#[cfg(test)]
mod tests {
    use core::mem::size_of;

    use super::*;

    #[test]
    fn sizes() {
        assert!(
            size_of::<Interaction>() <= 288,
            "{}",
            size_of::<Interaction>()
        );
        assert_eq!(size_of::<InteractionData>(), 16);
        assert!(
            size_of::<CommandDataOption>() <= 56,
            "{}",
            size_of::<CommandDataOption>()
        );
        assert!(
            size_of::<SubmittedComponent>() <= 64,
            "{}",
            size_of::<SubmittedComponent>()
        );
        assert!(
            size_of::<InteractionResponse>() <= 32,
            "{}",
            size_of::<InteractionResponse>()
        );
    }

    #[test]
    fn debug_redacts_the_token() {
        let interaction: Interaction = serde_json::from_str(
            r#"{"id":"1","application_id":"2","type":1,"token":"secret-token","version":1}"#,
        )
        .unwrap();
        let debug = format!("{interaction:?}");
        assert!(!debug.contains("secret-token"), "{debug}");
        assert!(debug.contains("<redacted>"), "{debug}");
    }
}
