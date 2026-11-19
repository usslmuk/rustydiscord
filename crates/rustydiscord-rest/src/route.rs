use core::fmt;

use http::Method;
use rustydiscord_model::{
    emoji::{Emoji, PartialEmoji},
    id::{
        ApplicationId, AutoModerationRuleId, ChannelId, CommandId, EmojiId, EntitlementId,
        GenericId, GuildId, Id, IntegrationId, InteractionId, LobbyId, MessageId, RoleId,
        ScheduledEventId, SkuId, SoundboardSoundId, StickerId, StickerPackId, SubscriptionId,
        UserId, WebhookId,
    },
    util::Timestamp,
};

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub(crate) struct Limits {
    pub(crate) global: bool,

    pub(crate) burst: bool,
}

impl Limits {
    pub(crate) const NORMAL: Self = Self {
        global: true,
        burst: false,
    };

    const INTERACTION: Self = Self {
        global: false,
        burst: false,
    };

    const INTERACTION_CALLBACK: Self = Self {
        global: false,
        burst: true,
    };
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub(crate) struct BucketRoute {
    pub(crate) template: &'static str,

    pub(crate) major: u64,

    pub(crate) limits: Limits,
}

impl BucketRoute {
    pub(crate) fn raw(method: &Method) -> Self {
        let template = if method == Method::GET {
            "RAW GET"
        } else if method == Method::POST {
            "RAW POST"
        } else if method == Method::PUT {
            "RAW PUT"
        } else if method == Method::PATCH {
            "RAW PATCH"
        } else if method == Method::DELETE {
            "RAW DELETE"
        } else {
            "RAW"
        };
        Self {
            template,
            major: 0,
            limits: Limits::NORMAL,
        }
    }
}

const OLD_MESSAGE_AGE_MS: u64 = 14 * 24 * 60 * 60 * 1000;

const OLD_MESSAGE_DELETE: &str =
    "DELETE /channels/{channel_id}/messages/{message_id} (older than two weeks)";

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum ReactionEmoji<'a> {
    Unicode(&'a str),

    Custom { name: &'a str, id: EmojiId },
}

impl<'a> From<&'a str> for ReactionEmoji<'a> {
    fn from(emoji: &'a str) -> Self {
        Self::Unicode(emoji)
    }
}

impl<'a> From<&'a Emoji> for ReactionEmoji<'a> {
    fn from(emoji: &'a Emoji) -> Self {
        Self::Custom {
            name: &emoji.name,
            id: emoji.id,
        }
    }
}

impl<'a> From<&'a PartialEmoji> for ReactionEmoji<'a> {
    fn from(emoji: &'a PartialEmoji) -> Self {
        match emoji {
            PartialEmoji::Custom { id, name, .. } => Self::Custom {
                name: name.as_deref().unwrap_or("_"),
                id: *id,
            },
            PartialEmoji::Unicode { name } => Self::Unicode(name),
        }
    }
}

#[derive(Clone, Copy)]
enum Segment<'a> {
    Id(u64),
    Text(&'a str),
    Emoji(ReactionEmoji<'a>),
}

trait Param: Copy {
    fn to_segment<'a>(self) -> Segment<'a>
    where
        Self: 'a;

    fn major(self) -> u64;

    fn debug(self, name: &str, out: &mut fmt::DebugStruct<'_, '_>);
}

impl<M> Param for Id<M> {
    fn to_segment<'a>(self) -> Segment<'a>
    where
        Self: 'a,
    {
        Segment::Id(self.get())
    }

    fn major(self) -> u64 {
        self.get()
    }

    fn debug(self, name: &str, out: &mut fmt::DebugStruct<'_, '_>) {
        out.field(name, &self.get());
    }
}

impl Param for u32 {
    fn to_segment<'a>(self) -> Segment<'a>
    where
        Self: 'a,
    {
        Segment::Id(u64::from(self))
    }

    fn major(self) -> u64 {
        u64::from(self)
    }

    fn debug(self, name: &str, out: &mut fmt::DebugStruct<'_, '_>) {
        out.field(name, &self);
    }
}

impl Param for &str {
    fn to_segment<'a>(self) -> Segment<'a>
    where
        Self: 'a,
    {
        Segment::Text(self)
    }

    fn major(self) -> u64 {
        self.bytes().fold(0xcbf2_9ce4_8422_2325, |hash, byte| {
            (hash ^ u64::from(byte)).wrapping_mul(0x0100_0000_01b3)
        })
    }

    fn debug(self, name: &str, out: &mut fmt::DebugStruct<'_, '_>) {
        if name.ends_with("token") {
            out.field(name, &"<redacted>");
        } else {
            out.field(name, &self);
        }
    }
}

impl Param for ReactionEmoji<'_> {
    fn to_segment<'a>(self) -> Segment<'a>
    where
        Self: 'a,
    {
        Segment::Emoji(self)
    }

    fn major(self) -> u64 {
        0
    }

    fn debug(self, name: &str, out: &mut fmt::DebugStruct<'_, '_>) {
        out.field(name, &self);
    }
}

const fn combine(hash: u64, part: u64) -> u64 {
    (hash.rotate_left(29) ^ part).wrapping_mul(0x9e37_79b9_7f4a_7c15)
}

pub(crate) fn write_text(out: &mut String, text: &str) {
    const HEX: &[u8; 16] = b"0123456789ABCDEF";
    for byte in text.bytes() {
        if byte.is_ascii_alphanumeric() || matches!(byte, b'-' | b'.' | b'_' | b'~') {
            out.push(char::from(byte));
        } else {
            out.push('%');
            out.push(char::from(HEX[usize::from(byte >> 4)]));
            out.push(char::from(HEX[usize::from(byte & 0xf)]));
        }
    }
}

fn write_template(out: &mut String, template: &str, params: &[(&str, Segment<'_>)]) {
    let mut rest = template;
    while let Some(open) = rest.find('{') {
        out.push_str(&rest[..open]);
        let after = &rest[open + 1..];
        let close = after.find('}').unwrap_or(after.len());
        let name = &after[..close];
        match params.iter().find(|(param, _)| *param == name) {
            Some((_, Segment::Id(id))) => {
                let _ = fmt::Write::write_fmt(out, format_args!("{id}"));
            }
            Some((_, Segment::Text(text))) => write_text(out, text),
            Some((_, Segment::Emoji(ReactionEmoji::Unicode(emoji)))) => write_text(out, emoji),
            Some((_, Segment::Emoji(ReactionEmoji::Custom { name, id }))) => {
                write_text(out, name);
                let _ = fmt::Write::write_fmt(out, format_args!("%3A{id}"));
            }
            None => debug_assert!(false, "unknown route parameter {{{name}}} in {template}"),
        }
        rest = after.get(close + 1..).unwrap_or("");
    }
    out.push_str(rest);
}

macro_rules! routes {
    ($(
        $(#[doc = $doc:literal])*
        $variant:ident $({ $( $(#[doc = $field_doc:literal])* $field:ident: $ty:ty ),* $(,)? })?
            => $method:ident $template:literal
            $(, major: $($major:ident)+)?
            $(, limits: $limits:ident)?;
    )*) => {






















        #[derive(Clone, Copy, PartialEq, Eq, Hash)]
        #[non_exhaustive]
        pub enum Route<'a> {
            $(
                $(#[doc = $doc])*
                #[doc = ""]
                #[doc = concat!("`", stringify!($method), " ", $template, "`")]
                $variant $({ $( $(#[doc = $field_doc])* $field: $ty ),* })?,
            )*
        }

        impl Route<'_> {



            #[must_use]
            pub fn method(self) -> Method {
                match self {
                    $(Self::$variant { .. } => Method::$method,)*
                }
            }





            #[must_use]
            pub fn path(self) -> String {
                let mut path = String::with_capacity(64);
                self.write_path(&mut path);
                path
            }


            pub(crate) fn write_path(self, out: &mut String) {
                match self {
                    $(Self::$variant $({ $($field),* })? => write_template(
                        out,
                        $template,
                        &[$($( (stringify!($field), Param::to_segment($field)) ),*)?],
                    ),)*
                }
            }



            fn has_token_parameter(self) -> bool {
                match self {
                    $(Self::$variant { .. } => [$($(stringify!($field)),*)?]
                        .iter()
                        .any(|name: &&str| name.ends_with("token")),)*
                }
            }


            fn base_bucket(self) -> BucketRoute {
                match self {
                    $(
                        #[allow(unused_variables, reason = "not every route has a major parameter")]
                        Self::$variant $({ $($field),* })? => BucketRoute {
                            template: concat!(stringify!($method), " ", $template),
                            major: 0 $($( ^ combine(0, Param::major($major)) )+)?,
                            limits: routes!(@limits $($limits)?),
                        },
                    )*
                }
            }
        }

        impl fmt::Debug for Route<'_> {
            fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                match *self {
                    $(
                        #[allow(unused_mut, reason = "unit routes have no fields")]
                        Self::$variant $({ $($field),* })? => {
                            let mut out = f.debug_struct(stringify!($variant));
                            $($( Param::debug($field, stringify!($field), &mut out); )*)?
                            out.finish()
                        }
                    )*
                }
            }
        }
    };
    (@limits) => { Limits::NORMAL };
    (@limits $limits:ident) => { Limits::$limits };
}

impl Route<'_> {
    pub(crate) fn authenticated(self) -> bool {
        !self.has_token_parameter()
            && !matches!(
                self,
                Self::OAuth2TokenExchange | Self::OAuth2TokenRevocation
            )
    }

    pub(crate) fn bucket(self) -> BucketRoute {
        let now_ms = u64::try_from(Timestamp::now().as_unix_millis()).unwrap_or(0);
        self.bucket_at(now_ms)
    }

    fn bucket_at(self, now_ms: u64) -> BucketRoute {
        let mut bucket = self.base_bucket();

        if let Self::DeleteMessage { message_id, .. } = self
            && now_ms.saturating_sub(message_id.timestamp_ms()) > OLD_MESSAGE_AGE_MS
        {
            bucket.template = OLD_MESSAGE_DELETE;
        }
        bucket
    }
}

routes! {

    GetGateway => GET "/gateway";

    GetGatewayBot => GET "/gateway/bot";




    GetGuild {

        guild_id: GuildId,
    } => GET "/guilds/{guild_id}", major: guild_id;


    GetGuildPreview {

        guild_id: GuildId,
    } => GET "/guilds/{guild_id}/preview", major: guild_id;


    ModifyGuild {

        guild_id: GuildId,
    } => PATCH "/guilds/{guild_id}", major: guild_id;


    GetGuildChannels {

        guild_id: GuildId,
    } => GET "/guilds/{guild_id}/channels", major: guild_id;


    CreateGuildChannel {

        guild_id: GuildId,
    } => POST "/guilds/{guild_id}/channels", major: guild_id;


    ModifyGuildChannelPositions {

        guild_id: GuildId,
    } => PATCH "/guilds/{guild_id}/channels", major: guild_id;


    ListActiveGuildThreads {

        guild_id: GuildId,
    } => GET "/guilds/{guild_id}/threads/active", major: guild_id;


    GetGuildMember {

        guild_id: GuildId,

        user_id: UserId,
    } => GET "/guilds/{guild_id}/members/{user_id}", major: guild_id;


    ListGuildMembers {

        guild_id: GuildId,
    } => GET "/guilds/{guild_id}/members", major: guild_id;


    SearchGuildMembers {

        guild_id: GuildId,
    } => GET "/guilds/{guild_id}/members/search", major: guild_id;


    GetGuildMemberPermissions {

        guild_id: GuildId,

        user_id: UserId,
    } => GET "/guilds/{guild_id}/permissions/member/{user_id}", major: guild_id;


    AddGuildMember {

        guild_id: GuildId,

        user_id: UserId,
    } => PUT "/guilds/{guild_id}/members/{user_id}", major: guild_id;


    ModifyGuildMember {

        guild_id: GuildId,

        user_id: UserId,
    } => PATCH "/guilds/{guild_id}/members/{user_id}", major: guild_id;


    ModifyCurrentMember {

        guild_id: GuildId,
    } => PATCH "/guilds/{guild_id}/members/@me", major: guild_id;


    AddGuildMemberRole {

        guild_id: GuildId,

        user_id: UserId,

        role_id: RoleId,
    } => PUT "/guilds/{guild_id}/members/{user_id}/roles/{role_id}", major: guild_id;


    RemoveGuildMemberRole {

        guild_id: GuildId,

        user_id: UserId,

        role_id: RoleId,
    } => DELETE "/guilds/{guild_id}/members/{user_id}/roles/{role_id}", major: guild_id;


    RemoveGuildMember {

        guild_id: GuildId,

        user_id: UserId,
    } => DELETE "/guilds/{guild_id}/members/{user_id}", major: guild_id;


    GetGuildBans {

        guild_id: GuildId,
    } => GET "/guilds/{guild_id}/bans", major: guild_id;


    GetGuildBan {

        guild_id: GuildId,

        user_id: UserId,
    } => GET "/guilds/{guild_id}/bans/{user_id}", major: guild_id;


    CreateGuildBan {

        guild_id: GuildId,

        user_id: UserId,
    } => PUT "/guilds/{guild_id}/bans/{user_id}", major: guild_id;


    RemoveGuildBan {

        guild_id: GuildId,

        user_id: UserId,
    } => DELETE "/guilds/{guild_id}/bans/{user_id}", major: guild_id;


    BulkGuildBan {

        guild_id: GuildId,
    } => POST "/guilds/{guild_id}/bulk-ban", major: guild_id;


    GetGuildRoles {

        guild_id: GuildId,
    } => GET "/guilds/{guild_id}/roles", major: guild_id;


    GetGuildRole {

        guild_id: GuildId,

        role_id: RoleId,
    } => GET "/guilds/{guild_id}/roles/{role_id}", major: guild_id;


    GetGuildRolePermissions {

        guild_id: GuildId,

        role_id: RoleId,
    } => GET "/guilds/{guild_id}/permissions/role/{role_id}", major: guild_id;


    GetGuildRoleMemberCounts {

        guild_id: GuildId,
    } => GET "/guilds/{guild_id}/roles/member-counts", major: guild_id;


    CreateGuildRole {

        guild_id: GuildId,
    } => POST "/guilds/{guild_id}/roles", major: guild_id;


    ModifyGuildRolePositions {

        guild_id: GuildId,
    } => PATCH "/guilds/{guild_id}/roles", major: guild_id;


    ModifyGuildRole {

        guild_id: GuildId,

        role_id: RoleId,
    } => PATCH "/guilds/{guild_id}/roles/{role_id}", major: guild_id;


    DeleteGuildRole {

        guild_id: GuildId,

        role_id: RoleId,
    } => DELETE "/guilds/{guild_id}/roles/{role_id}", major: guild_id;


    GetGuildPruneCount {

        guild_id: GuildId,
    } => GET "/guilds/{guild_id}/prune", major: guild_id;


    BeginGuildPrune {

        guild_id: GuildId,
    } => POST "/guilds/{guild_id}/prune", major: guild_id;


    GetGuildVoiceRegions {

        guild_id: GuildId,
    } => GET "/guilds/{guild_id}/regions", major: guild_id;


    GetGuildInvites {

        guild_id: GuildId,
    } => GET "/guilds/{guild_id}/invites", major: guild_id;


    GetGuildIntegrations {

        guild_id: GuildId,
    } => GET "/guilds/{guild_id}/integrations", major: guild_id;


    DeleteGuildIntegration {

        guild_id: GuildId,

        integration_id: IntegrationId,
    } => DELETE "/guilds/{guild_id}/integrations/{integration_id}", major: guild_id;


    GetGuildWidgetSettings {

        guild_id: GuildId,
    } => GET "/guilds/{guild_id}/widget", major: guild_id;


    ModifyGuildWidget {

        guild_id: GuildId,
    } => PATCH "/guilds/{guild_id}/widget", major: guild_id;


    GetGuildWidget {

        guild_id: GuildId,
    } => GET "/guilds/{guild_id}/widget.json", major: guild_id;


    GetGuildVanityUrl {

        guild_id: GuildId,
    } => GET "/guilds/{guild_id}/vanity-url", major: guild_id;


    GetGuildWelcomeScreen {

        guild_id: GuildId,
    } => GET "/guilds/{guild_id}/welcome-screen", major: guild_id;


    ModifyGuildWelcomeScreen {

        guild_id: GuildId,
    } => PATCH "/guilds/{guild_id}/welcome-screen", major: guild_id;


    GetGuildOnboarding {

        guild_id: GuildId,
    } => GET "/guilds/{guild_id}/onboarding", major: guild_id;


    ModifyGuildOnboarding {

        guild_id: GuildId,
    } => PUT "/guilds/{guild_id}/onboarding", major: guild_id;


    ModifyGuildIncidentActions {

        guild_id: GuildId,
    } => PUT "/guilds/{guild_id}/incident-actions", major: guild_id;




    GetCurrentUser => GET "/users/@me";


    GetUser {

        user_id: UserId,
    } => GET "/users/{user_id}";


    ModifyCurrentUser => PATCH "/users/@me";


    GetCurrentUserGuilds => GET "/users/@me/guilds";


    GetCurrentUserGuildMember {

        guild_id: GuildId,
    } => GET "/users/@me/guilds/{guild_id}/member";


    LeaveGuild {

        guild_id: GuildId,
    } => DELETE "/users/@me/guilds/{guild_id}";


    CreateDm => POST "/users/@me/channels";



    CreateGroupDm => POST "/users/@me/channels";


    GetCurrentUserConnections => GET "/users/@me/connections";


    GetCurrentUserApplicationRoleConnection {

        application_id: ApplicationId,
    } => GET "/users/@me/applications/{application_id}/role-connection";


    UpdateCurrentUserApplicationRoleConnection {

        application_id: ApplicationId,
    } => PUT "/users/@me/applications/{application_id}/role-connection";


    DeleteCurrentUserApplicationRoleConnection {

        application_id: ApplicationId,
    } => DELETE "/users/@me/applications/{application_id}/role-connection";




    CreateWebhook {

        channel_id: ChannelId,
    } => POST "/channels/{channel_id}/webhooks",
        major: channel_id;


    GetChannelWebhooks {

        channel_id: ChannelId,
    } => GET "/channels/{channel_id}/webhooks",
        major: channel_id;


    GetGuildWebhooks {

        guild_id: GuildId,
    } => GET "/guilds/{guild_id}/webhooks",
        major: guild_id;


    GetWebhook {

        webhook_id: WebhookId,
    } => GET "/webhooks/{webhook_id}",
        major: webhook_id;


    GetWebhookWithToken {

        webhook_id: WebhookId,

        webhook_token: &'a str,
    } => GET "/webhooks/{webhook_id}/{webhook_token}",
        major: webhook_id webhook_token;


    ModifyWebhook {

        webhook_id: WebhookId,
    } => PATCH "/webhooks/{webhook_id}",
        major: webhook_id;


    ModifyWebhookWithToken {

        webhook_id: WebhookId,

        webhook_token: &'a str,
    } => PATCH "/webhooks/{webhook_id}/{webhook_token}",
        major: webhook_id webhook_token;


    DeleteWebhook {

        webhook_id: WebhookId,
    } => DELETE "/webhooks/{webhook_id}",
        major: webhook_id;


    DeleteWebhookWithToken {

        webhook_id: WebhookId,

        webhook_token: &'a str,
    } => DELETE "/webhooks/{webhook_id}/{webhook_token}",
        major: webhook_id webhook_token;


    ExecuteWebhook {

        webhook_id: WebhookId,

        webhook_token: &'a str,
    } => POST "/webhooks/{webhook_id}/{webhook_token}",
        major: webhook_id webhook_token;


    GetWebhookMessage {

        webhook_id: WebhookId,

        webhook_token: &'a str,

        message_id: MessageId,
    } => GET "/webhooks/{webhook_id}/{webhook_token}/messages/{message_id}",
        major: webhook_id webhook_token;


    EditWebhookMessage {

        webhook_id: WebhookId,

        webhook_token: &'a str,

        message_id: MessageId,
    } => PATCH "/webhooks/{webhook_id}/{webhook_token}/messages/{message_id}",
        major: webhook_id webhook_token;


    DeleteWebhookMessage {

        webhook_id: WebhookId,

        webhook_token: &'a str,

        message_id: MessageId,
    } => DELETE "/webhooks/{webhook_id}/{webhook_token}/messages/{message_id}",
        major: webhook_id webhook_token;





    ListGuildEmojis {

        guild_id: GuildId,
    } => GET "/guilds/{guild_id}/emojis", major: guild_id;


    GetGuildEmoji {

        guild_id: GuildId,

        emoji_id: EmojiId,
    } => GET "/guilds/{guild_id}/emojis/{emoji_id}", major: guild_id;


    CreateGuildEmoji {

        guild_id: GuildId,
    } => POST "/guilds/{guild_id}/emojis", major: guild_id;


    ModifyGuildEmoji {

        guild_id: GuildId,

        emoji_id: EmojiId,
    } => PATCH "/guilds/{guild_id}/emojis/{emoji_id}", major: guild_id;


    DeleteGuildEmoji {

        guild_id: GuildId,

        emoji_id: EmojiId,
    } => DELETE "/guilds/{guild_id}/emojis/{emoji_id}", major: guild_id;


    ListApplicationEmojis {

        application_id: ApplicationId,
    } => GET "/applications/{application_id}/emojis";


    GetApplicationEmoji {

        application_id: ApplicationId,

        emoji_id: EmojiId,
    } => GET "/applications/{application_id}/emojis/{emoji_id}";


    CreateApplicationEmoji {

        application_id: ApplicationId,
    } => POST "/applications/{application_id}/emojis";


    ModifyApplicationEmoji {

        application_id: ApplicationId,

        emoji_id: EmojiId,
    } => PATCH "/applications/{application_id}/emojis/{emoji_id}";


    DeleteApplicationEmoji {

        application_id: ApplicationId,

        emoji_id: EmojiId,
    } => DELETE "/applications/{application_id}/emojis/{emoji_id}";




    GetSticker {

        sticker_id: StickerId,
    } => GET "/stickers/{sticker_id}";


    ListStickerPacks => GET "/sticker-packs";


    GetStickerPack {

        pack_id: StickerPackId,
    } => GET "/sticker-packs/{pack_id}";


    ListGuildStickers {

        guild_id: GuildId,
    } => GET "/guilds/{guild_id}/stickers", major: guild_id;


    GetGuildSticker {

        guild_id: GuildId,

        sticker_id: StickerId,
    } => GET "/guilds/{guild_id}/stickers/{sticker_id}", major: guild_id;


    CreateGuildSticker {

        guild_id: GuildId,
    } => POST "/guilds/{guild_id}/stickers", major: guild_id;


    ModifyGuildSticker {

        guild_id: GuildId,

        sticker_id: StickerId,
    } => PATCH "/guilds/{guild_id}/stickers/{sticker_id}", major: guild_id;


    DeleteGuildSticker {

        guild_id: GuildId,

        sticker_id: StickerId,
    } => DELETE "/guilds/{guild_id}/stickers/{sticker_id}", major: guild_id;




    SendSoundboardSound {

        channel_id: ChannelId,
    } => POST "/channels/{channel_id}/send-soundboard-sound", major: channel_id;


    ListDefaultSoundboardSounds => GET "/soundboard-default-sounds";


    ListGuildSoundboardSounds {

        guild_id: GuildId,
    } => GET "/guilds/{guild_id}/soundboard-sounds", major: guild_id;


    GetGuildSoundboardSound {

        guild_id: GuildId,

        sound_id: SoundboardSoundId,
    } => GET "/guilds/{guild_id}/soundboard-sounds/{sound_id}", major: guild_id;


    CreateGuildSoundboardSound {

        guild_id: GuildId,
    } => POST "/guilds/{guild_id}/soundboard-sounds", major: guild_id;


    ModifyGuildSoundboardSound {

        guild_id: GuildId,

        sound_id: SoundboardSoundId,
    } => PATCH "/guilds/{guild_id}/soundboard-sounds/{sound_id}", major: guild_id;


    DeleteGuildSoundboardSound {

        guild_id: GuildId,

        sound_id: SoundboardSoundId,
    } => DELETE "/guilds/{guild_id}/soundboard-sounds/{sound_id}", major: guild_id;




    ListScheduledEventsForGuild {

        guild_id: GuildId,
    } => GET "/guilds/{guild_id}/scheduled-events", major: guild_id;


    CreateGuildScheduledEvent {

        guild_id: GuildId,
    } => POST "/guilds/{guild_id}/scheduled-events", major: guild_id;


    GetGuildScheduledEvent {

        guild_id: GuildId,

        event_id: ScheduledEventId,
    } => GET "/guilds/{guild_id}/scheduled-events/{event_id}", major: guild_id;


    ModifyGuildScheduledEvent {

        guild_id: GuildId,

        event_id: ScheduledEventId,
    } => PATCH "/guilds/{guild_id}/scheduled-events/{event_id}", major: guild_id;


    DeleteGuildScheduledEvent {

        guild_id: GuildId,

        event_id: ScheduledEventId,
    } => DELETE "/guilds/{guild_id}/scheduled-events/{event_id}", major: guild_id;


    GetGuildScheduledEventUsers {

        guild_id: GuildId,

        event_id: ScheduledEventId,
    } => GET "/guilds/{guild_id}/scheduled-events/{event_id}/users", major: guild_id;




    CreateStageInstance => POST "/stage-instances";


    GetStageInstance {

        channel_id: ChannelId,
    } => GET "/stage-instances/{channel_id}";


    ModifyStageInstance {

        channel_id: ChannelId,
    } => PATCH "/stage-instances/{channel_id}";


    DeleteStageInstance {

        channel_id: ChannelId,
    } => DELETE "/stage-instances/{channel_id}";




    GetInvite {

        invite_code: &'a str,
    } => GET "/invites/{invite_code}";


    DeleteInvite {

        invite_code: &'a str,
    } => DELETE "/invites/{invite_code}";


    GetTargetUsers {

        invite_code: &'a str,
    } => GET "/invites/{invite_code}/target-users";


    AddTargetUser {

        invite_code: &'a str,

        user_id: UserId,
    } => PUT "/invites/{invite_code}/target-users/{user_id}";


    RemoveTargetUser {

        invite_code: &'a str,

        user_id: UserId,
    } => DELETE "/invites/{invite_code}/target-users/{user_id}";


    UpdateTargetUsers {

        invite_code: &'a str,
    } => PUT "/invites/{invite_code}/target-users";


    BulkAddTargetUsers {

        invite_code: &'a str,
    } => POST "/invites/{invite_code}/target-users/bulk-add";


    BulkDeleteTargetUsers {

        invite_code: &'a str,
    } => POST "/invites/{invite_code}/target-users/bulk-delete";


    GetTargetUsersJobStatus {

        invite_code: &'a str,
    } => GET "/invites/{invite_code}/target-users/job-status";





    GetGuildTemplate {

        template_code: &'a str,
    } => GET "/guilds/templates/{template_code}";


    GetGuildTemplates {

        guild_id: GuildId,
    } => GET "/guilds/{guild_id}/templates", major: guild_id;


    CreateGuildTemplate {

        guild_id: GuildId,
    } => POST "/guilds/{guild_id}/templates", major: guild_id;


    SyncGuildTemplate {

        guild_id: GuildId,

        template_code: &'a str,
    } => PUT "/guilds/{guild_id}/templates/{template_code}", major: guild_id;


    ModifyGuildTemplate {

        guild_id: GuildId,

        template_code: &'a str,
    } => PATCH "/guilds/{guild_id}/templates/{template_code}", major: guild_id;


    DeleteGuildTemplate {

        guild_id: GuildId,

        template_code: &'a str,
    } => DELETE "/guilds/{guild_id}/templates/{template_code}", major: guild_id;




    GetGuildAuditLog {

        guild_id: GuildId,
    } => GET "/guilds/{guild_id}/audit-logs", major: guild_id;




    GetAnswerVoters {

        channel_id: ChannelId,

        message_id: MessageId,

        answer_id: u32,
    } => GET "/channels/{channel_id}/polls/{message_id}/answers/{answer_id}",
        major: channel_id;

    EndPoll {

        channel_id: ChannelId,

        message_id: MessageId,
    } => POST "/channels/{channel_id}/polls/{message_id}/expire", major: channel_id;




    ListVoiceRegions => GET "/voice/regions";

    GetCurrentUserVoiceState {

        guild_id: GuildId,
    } => GET "/guilds/{guild_id}/voice-states/@me", major: guild_id;

    GetUserVoiceState {

        guild_id: GuildId,

        user_id: UserId,
    } => GET "/guilds/{guild_id}/voice-states/{user_id}", major: guild_id;

    ModifyCurrentUserVoiceState {

        guild_id: GuildId,
    } => PATCH "/guilds/{guild_id}/voice-states/@me", major: guild_id;

    ModifyUserVoiceState {

        guild_id: GuildId,

        user_id: UserId,
    } => PATCH "/guilds/{guild_id}/voice-states/{user_id}", major: guild_id;




    CreateLobby => POST "/lobbies";

    CreateOrJoinLobby => PUT "/lobbies";

    GetLobby {

        lobby_id: LobbyId,
    } => GET "/lobbies/{lobby_id}";

    ModifyLobby {

        lobby_id: LobbyId,
    } => PATCH "/lobbies/{lobby_id}";

    DeleteLobby {

        lobby_id: LobbyId,
    } => DELETE "/lobbies/{lobby_id}";

    AddLobbyMember {

        lobby_id: LobbyId,

        user_id: UserId,
    } => PUT "/lobbies/{lobby_id}/members/{user_id}";

    BulkUpdateLobbyMembers {

        lobby_id: LobbyId,
    } => POST "/lobbies/{lobby_id}/members/bulk";

    RemoveLobbyMember {

        lobby_id: LobbyId,

        user_id: UserId,
    } => DELETE "/lobbies/{lobby_id}/members/{user_id}";

    LeaveLobby {

        lobby_id: LobbyId,
    } => DELETE "/lobbies/{lobby_id}/members/@me";

    LinkChannelToLobby {

        lobby_id: LobbyId,
    } => PATCH "/lobbies/{lobby_id}/channel-linking";

    UnlinkChannelFromLobby {

        lobby_id: LobbyId,
    } => PATCH "/lobbies/{lobby_id}/channel-linking";

    SendLobbyMessage {

        lobby_id: LobbyId,
    } => POST "/lobbies/{lobby_id}/messages";

    GetLobbyMessages {

        lobby_id: LobbyId,
    } => GET "/lobbies/{lobby_id}/messages";

    UpdateLobbyMessageModerationMetadata {

        lobby_id: LobbyId,

        message_id: MessageId,
    } => PUT "/lobbies/{lobby_id}/messages/{message_id}/moderation-metadata";

    CreateLobbyChannelInviteForSelf {

        lobby_id: LobbyId,
    } => POST "/lobbies/{lobby_id}/members/@me/invites";

    CreateLobbyChannelInviteForUser {

        lobby_id: LobbyId,

        user_id: UserId,
    } => POST "/lobbies/{lobby_id}/members/{user_id}/invites";




    GetCurrentBotApplicationInformation => GET "/oauth2/applications/@me";

    GetCurrentAuthorizationInformation => GET "/oauth2/@me";



    OAuth2TokenExchange => POST "/oauth2/token";


    OAuth2TokenRevocation => POST "/oauth2/token/revoke";




    UpdateApplicationIdentityProfile {

        application_id: ApplicationId,

        user_id: UserId,

        provider_issued_user_id: &'a str,
    } => PATCH "/applications/{application_id}/users/{user_id}/identities/{provider_issued_user_id}/profile";

    GetApplicationIdentityProfile {

        application_id: ApplicationId,

        user_id: UserId,

        provider_issued_user_id: &'a str,
    } => GET "/applications/{application_id}/users/{user_id}/identities/{provider_issued_user_id}/profile";

    GetApplicationIdentitiesByUserId {

        user_id: UserId,

        application_id: ApplicationId,
    } => GET "/users/{user_id}/application-identities/{application_id}";

    GetApplicationIdentitiesByExternalId {

        application_id: ApplicationId,

        provider_type: &'a str,

        provider_issued_user_id: &'a str,
    } => GET "/applications/{application_id}/application-identities/{provider_type}/{provider_issued_user_id}";

    DeleteApplicationIdentity {

        user_id: UserId,

        application_id: ApplicationId,

        provider_type: &'a str,

        provider_issued_user_id: &'a str,
    } => POST "/users/{user_id}/application-identities/{application_id}/{provider_type}/{provider_issued_user_id}/delete";




    GetCurrentApplication => GET "/applications/@me";

    EditCurrentApplication => PATCH "/applications/@me";

    GetApplicationActivityInstance {

        application_id: ApplicationId,

        instance_id: &'a str,
    } => GET "/applications/{application_id}/activity-instances/{instance_id}";




    GetApplicationRoleConnectionMetadataRecords {

        application_id: ApplicationId,
    } => GET "/applications/{application_id}/role-connections/metadata";

    UpdateApplicationRoleConnectionMetadataRecords {

        application_id: ApplicationId,
    } => PUT "/applications/{application_id}/role-connections/metadata";




    ListSkus {

        application_id: ApplicationId,
    } => GET "/applications/{application_id}/skus";

    ListEntitlements {

        application_id: ApplicationId,
    } => GET "/applications/{application_id}/entitlements";

    GetEntitlement {

        application_id: ApplicationId,

        entitlement_id: EntitlementId,
    } => GET "/applications/{application_id}/entitlements/{entitlement_id}";

    ConsumeEntitlement {

        application_id: ApplicationId,

        entitlement_id: EntitlementId,
    } => POST "/applications/{application_id}/entitlements/{entitlement_id}/consume";

    CreateTestEntitlement {

        application_id: ApplicationId,
    } => POST "/applications/{application_id}/entitlements";

    DeleteTestEntitlement {

        application_id: ApplicationId,

        entitlement_id: EntitlementId,
    } => DELETE "/applications/{application_id}/entitlements/{entitlement_id}";

    ListSkuSubscriptions {

        sku_id: SkuId,
    } => GET "/skus/{sku_id}/subscriptions";

    GetSkuSubscription {

        sku_id: SkuId,

        subscription_id: SubscriptionId,
    } => GET "/skus/{sku_id}/subscriptions/{subscription_id}";




    ListAutoModerationRulesForGuild {

        guild_id: GuildId,
    } => GET "/guilds/{guild_id}/auto-moderation/rules", major: guild_id;

    GetAutoModerationRule {

        guild_id: GuildId,

        rule_id: AutoModerationRuleId,
    } => GET "/guilds/{guild_id}/auto-moderation/rules/{rule_id}", major: guild_id;

    CreateAutoModerationRule {

        guild_id: GuildId,
    } => POST "/guilds/{guild_id}/auto-moderation/rules", major: guild_id;

    ModifyAutoModerationRule {

        guild_id: GuildId,

        rule_id: AutoModerationRuleId,
    } => PATCH "/guilds/{guild_id}/auto-moderation/rules/{rule_id}", major: guild_id;

    DeleteAutoModerationRule {

        guild_id: GuildId,

        rule_id: AutoModerationRuleId,
    } => DELETE "/guilds/{guild_id}/auto-moderation/rules/{rule_id}", major: guild_id;




    GetChannel {

        channel_id: ChannelId,
    } => GET "/channels/{channel_id}", major: channel_id;


    ModifyChannel {

        channel_id: ChannelId,
    } => PATCH "/channels/{channel_id}", major: channel_id;


    DeleteChannel {

        channel_id: ChannelId,
    } => DELETE "/channels/{channel_id}", major: channel_id;


    GetChannelMemberPermissions {

        channel_id: ChannelId,

        user_id: UserId,
    } => GET "/channels/{channel_id}/permissions/member/{user_id}", major: channel_id;


    GetChannelRolePermissions {

        channel_id: ChannelId,

        role_id: RoleId,
    } => GET "/channels/{channel_id}/permissions/role/{role_id}", major: channel_id;


    SetVoiceChannelStatus {

        channel_id: ChannelId,
    } => PUT "/channels/{channel_id}/voice-status", major: channel_id;


    EditChannelPermissions {

        channel_id: ChannelId,

        overwrite_id: GenericId,
    } => PUT "/channels/{channel_id}/permissions/{overwrite_id}", major: channel_id;


    DeleteChannelPermission {

        channel_id: ChannelId,

        overwrite_id: GenericId,
    } => DELETE "/channels/{channel_id}/permissions/{overwrite_id}", major: channel_id;


    GetChannelInvites {

        channel_id: ChannelId,
    } => GET "/channels/{channel_id}/invites", major: channel_id;


    CreateChannelInvite {

        channel_id: ChannelId,
    } => POST "/channels/{channel_id}/invites", major: channel_id;


    FollowAnnouncementChannel {

        channel_id: ChannelId,
    } => POST "/channels/{channel_id}/followers", major: channel_id;


    TriggerTypingIndicator {

        channel_id: ChannelId,
    } => POST "/channels/{channel_id}/typing", major: channel_id;


    GroupDmAddRecipient {

        channel_id: ChannelId,

        user_id: UserId,
    } => PUT "/channels/{channel_id}/recipients/{user_id}", major: channel_id;


    GroupDmRemoveRecipient {

        channel_id: ChannelId,

        user_id: UserId,
    } => DELETE "/channels/{channel_id}/recipients/{user_id}", major: channel_id;


    StartThreadFromMessage {

        channel_id: ChannelId,

        message_id: MessageId,
    } => POST "/channels/{channel_id}/messages/{message_id}/threads", major: channel_id;


    StartThreadWithoutMessage {

        channel_id: ChannelId,
    } => POST "/channels/{channel_id}/threads", major: channel_id;




    StartThreadInForumOrMediaChannel {

        channel_id: ChannelId,
    } => POST "/channels/{channel_id}/threads", major: channel_id;


    JoinThread {

        channel_id: ChannelId,
    } => PUT "/channels/{channel_id}/thread-members/@me", major: channel_id;


    AddThreadMember {

        channel_id: ChannelId,

        user_id: UserId,
    } => PUT "/channels/{channel_id}/thread-members/{user_id}", major: channel_id;


    LeaveThread {

        channel_id: ChannelId,
    } => DELETE "/channels/{channel_id}/thread-members/@me", major: channel_id;


    RemoveThreadMember {

        channel_id: ChannelId,

        user_id: UserId,
    } => DELETE "/channels/{channel_id}/thread-members/{user_id}", major: channel_id;


    GetThreadMember {

        channel_id: ChannelId,

        user_id: UserId,
    } => GET "/channels/{channel_id}/thread-members/{user_id}", major: channel_id;


    ListThreadMembers {

        channel_id: ChannelId,
    } => GET "/channels/{channel_id}/thread-members", major: channel_id;


    ListPublicArchivedThreads {

        channel_id: ChannelId,
    } => GET "/channels/{channel_id}/threads/archived/public", major: channel_id;


    ListPrivateArchivedThreads {

        channel_id: ChannelId,
    } => GET "/channels/{channel_id}/threads/archived/private", major: channel_id;


    ListJoinedPrivateArchivedThreads {

        channel_id: ChannelId,
    } => GET "/channels/{channel_id}/users/@me/threads/archived/private", major: channel_id;




    GetChannelMessages {

        channel_id: ChannelId,
    } => GET "/channels/{channel_id}/messages", major: channel_id;


    SearchGuildMessages {

        guild_id: GuildId,
    } => GET "/guilds/{guild_id}/messages/search", major: guild_id;


    GetChannelMessage {

        channel_id: ChannelId,

        message_id: MessageId,
    } => GET "/channels/{channel_id}/messages/{message_id}", major: channel_id;


    CreateMessage {

        channel_id: ChannelId,
    } => POST "/channels/{channel_id}/messages", major: channel_id;


    CrosspostMessage {

        channel_id: ChannelId,

        message_id: MessageId,
    } => POST "/channels/{channel_id}/messages/{message_id}/crosspost", major: channel_id;


    CreateReaction {

        channel_id: ChannelId,

        message_id: MessageId,

        emoji: ReactionEmoji<'a>,
    } => PUT "/channels/{channel_id}/messages/{message_id}/reactions/{emoji}/@me",
        major: channel_id;


    DeleteOwnReaction {

        channel_id: ChannelId,

        message_id: MessageId,

        emoji: ReactionEmoji<'a>,
    } => DELETE "/channels/{channel_id}/messages/{message_id}/reactions/{emoji}/@me",
        major: channel_id;


    DeleteUserReaction {

        channel_id: ChannelId,

        message_id: MessageId,

        emoji: ReactionEmoji<'a>,

        user_id: UserId,
    } => DELETE "/channels/{channel_id}/messages/{message_id}/reactions/{emoji}/{user_id}",
        major: channel_id;


    GetReactions {

        channel_id: ChannelId,

        message_id: MessageId,

        emoji: ReactionEmoji<'a>,
    } => GET "/channels/{channel_id}/messages/{message_id}/reactions/{emoji}",
        major: channel_id;


    DeleteAllReactions {

        channel_id: ChannelId,

        message_id: MessageId,
    } => DELETE "/channels/{channel_id}/messages/{message_id}/reactions", major: channel_id;


    DeleteAllReactionsForEmoji {

        channel_id: ChannelId,

        message_id: MessageId,

        emoji: ReactionEmoji<'a>,
    } => DELETE "/channels/{channel_id}/messages/{message_id}/reactions/{emoji}",
        major: channel_id;


    EditMessage {

        channel_id: ChannelId,

        message_id: MessageId,
    } => PATCH "/channels/{channel_id}/messages/{message_id}", major: channel_id;



    DeleteMessage {

        channel_id: ChannelId,

        message_id: MessageId,
    } => DELETE "/channels/{channel_id}/messages/{message_id}", major: channel_id;


    BulkDeleteMessages {

        channel_id: ChannelId,
    } => POST "/channels/{channel_id}/messages/bulk-delete", major: channel_id;


    GetChannelPins {

        channel_id: ChannelId,
    } => GET "/channels/{channel_id}/messages/pins", major: channel_id;


    PinMessage {

        channel_id: ChannelId,

        message_id: MessageId,
    } => PUT "/channels/{channel_id}/messages/pins/{message_id}", major: channel_id;


    UnpinMessage {

        channel_id: ChannelId,

        message_id: MessageId,
    } => DELETE "/channels/{channel_id}/messages/pins/{message_id}", major: channel_id;




    CreateInteractionResponse {

        interaction_id: InteractionId,

        interaction_token: &'a str,
    } => POST "/interactions/{interaction_id}/{interaction_token}/callback",
        limits: INTERACTION_CALLBACK;


    GetOriginalInteractionResponse {

        application_id: ApplicationId,

        interaction_token: &'a str,
    } => GET "/webhooks/{application_id}/{interaction_token}/messages/@original",
        major: application_id interaction_token, limits: INTERACTION;


    EditOriginalInteractionResponse {

        application_id: ApplicationId,

        interaction_token: &'a str,
    } => PATCH "/webhooks/{application_id}/{interaction_token}/messages/@original",
        major: application_id interaction_token, limits: INTERACTION;


    DeleteOriginalInteractionResponse {

        application_id: ApplicationId,

        interaction_token: &'a str,
    } => DELETE "/webhooks/{application_id}/{interaction_token}/messages/@original",
        major: application_id interaction_token, limits: INTERACTION;


    CreateFollowupMessage {

        application_id: ApplicationId,

        interaction_token: &'a str,
    } => POST "/webhooks/{application_id}/{interaction_token}",
        major: application_id interaction_token, limits: INTERACTION;


    GetFollowupMessage {

        application_id: ApplicationId,

        interaction_token: &'a str,

        message_id: MessageId,
    } => GET "/webhooks/{application_id}/{interaction_token}/messages/{message_id}",
        major: application_id interaction_token, limits: INTERACTION;


    EditFollowupMessage {

        application_id: ApplicationId,

        interaction_token: &'a str,

        message_id: MessageId,
    } => PATCH "/webhooks/{application_id}/{interaction_token}/messages/{message_id}",
        major: application_id interaction_token, limits: INTERACTION;


    DeleteFollowupMessage {

        application_id: ApplicationId,

        interaction_token: &'a str,

        message_id: MessageId,
    } => DELETE "/webhooks/{application_id}/{interaction_token}/messages/{message_id}",
        major: application_id interaction_token, limits: INTERACTION;





    GetGlobalApplicationCommands {

        application_id: ApplicationId,
    } => GET "/applications/{application_id}/commands";


    CreateGlobalApplicationCommand {

        application_id: ApplicationId,
    } => POST "/applications/{application_id}/commands";


    GetGlobalApplicationCommand {

        application_id: ApplicationId,

        command_id: CommandId,
    } => GET "/applications/{application_id}/commands/{command_id}";


    EditGlobalApplicationCommand {

        application_id: ApplicationId,

        command_id: CommandId,
    } => PATCH "/applications/{application_id}/commands/{command_id}";


    DeleteGlobalApplicationCommand {

        application_id: ApplicationId,

        command_id: CommandId,
    } => DELETE "/applications/{application_id}/commands/{command_id}";


    BulkOverwriteGlobalApplicationCommands {

        application_id: ApplicationId,
    } => PUT "/applications/{application_id}/commands";


    GetGuildApplicationCommands {

        application_id: ApplicationId,

        guild_id: GuildId,
    } => GET "/applications/{application_id}/guilds/{guild_id}/commands";


    CreateGuildApplicationCommand {

        application_id: ApplicationId,

        guild_id: GuildId,
    } => POST "/applications/{application_id}/guilds/{guild_id}/commands";


    GetGuildApplicationCommand {

        application_id: ApplicationId,

        guild_id: GuildId,

        command_id: CommandId,
    } => GET "/applications/{application_id}/guilds/{guild_id}/commands/{command_id}";


    EditGuildApplicationCommand {

        application_id: ApplicationId,

        guild_id: GuildId,

        command_id: CommandId,
    } => PATCH "/applications/{application_id}/guilds/{guild_id}/commands/{command_id}";


    DeleteGuildApplicationCommand {

        application_id: ApplicationId,

        guild_id: GuildId,

        command_id: CommandId,
    } => DELETE "/applications/{application_id}/guilds/{guild_id}/commands/{command_id}";


    BulkOverwriteGuildApplicationCommands {

        application_id: ApplicationId,

        guild_id: GuildId,
    } => PUT "/applications/{application_id}/guilds/{guild_id}/commands";


    GetGuildApplicationCommandPermissions {

        application_id: ApplicationId,

        guild_id: GuildId,
    } => GET "/applications/{application_id}/guilds/{guild_id}/commands/permissions";


    GetApplicationCommandPermissions {

        application_id: ApplicationId,

        guild_id: GuildId,

        command_id: CommandId,
    } => GET "/applications/{application_id}/guilds/{guild_id}/commands/{command_id}/permissions";



    EditApplicationCommandPermissions {

        application_id: ApplicationId,

        guild_id: GuildId,

        command_id: CommandId,
    } => PUT "/applications/{application_id}/guilds/{guild_id}/commands/{command_id}/permissions";
}

#[cfg(test)]
mod tests {
    use super::*;

    const TOKEN: &str = "aW50ZXJhY3Rpb24";

    #[test]
    fn paths() {
        assert_eq!(Route::GetGatewayBot.path(), "/gateway/bot");
        assert_eq!(
            Route::CreateMessage {
                channel_id: ChannelId::new(1)
            }
            .path(),
            "/channels/1/messages"
        );
        assert_eq!(
            Route::CreateInteractionResponse {
                interaction_id: InteractionId::new(2),
                interaction_token: TOKEN,
            }
            .path(),
            "/interactions/2/aW50ZXJhY3Rpb24/callback"
        );
        assert_eq!(
            Route::EditFollowupMessage {
                application_id: ApplicationId::new(3),
                interaction_token: TOKEN,
                message_id: MessageId::new(4),
            }
            .path(),
            "/webhooks/3/aW50ZXJhY3Rpb24/messages/4"
        );
    }

    #[test]
    fn text_parameters_cannot_change_the_path() {
        let route = Route::CreateFollowupMessage {
            application_id: ApplicationId::new(3),
            interaction_token: "a/../b?c=d#e f%",
        };
        assert_eq!(route.path(), "/webhooks/3/a%2F..%2Fb%3Fc%3Dd%23e%20f%25");
    }

    #[test]
    fn debug_redacts_tokens() {
        let route = Route::GetFollowupMessage {
            application_id: ApplicationId::new(3),
            interaction_token: TOKEN,
            message_id: MessageId::new(4),
        };
        let debug = format!("{route:?}");
        assert_eq!(
            debug,
            r#"GetFollowupMessage { application_id: 3, interaction_token: "<redacted>", message_id: 4 }"#
        );
        assert_eq!(format!("{:?}", Route::GetGatewayBot), "GetGatewayBot");
    }

    #[test]
    fn token_routes_are_not_authenticated_with_the_client_token() {
        let webhook = WebhookId::new(1);
        assert!(
            Route::GetWebhook {
                webhook_id: webhook
            }
            .authenticated()
        );
        assert!(
            !Route::ExecuteWebhook {
                webhook_id: webhook,
                webhook_token: TOKEN
            }
            .authenticated()
        );
        assert!(
            !Route::CreateInteractionResponse {
                interaction_id: InteractionId::new(2),
                interaction_token: TOKEN
            }
            .authenticated()
        );
        assert!(Route::GetGatewayBot.authenticated());
        assert!(!Route::OAuth2TokenExchange.authenticated());
        assert!(!Route::OAuth2TokenRevocation.authenticated());
        assert!(Route::GetCurrentAuthorizationInformation.authenticated());
    }

    #[test]
    fn major_parameter() {
        let a = Route::CreateMessage {
            channel_id: ChannelId::new(1),
        }
        .bucket();
        let b = Route::CreateMessage {
            channel_id: ChannelId::new(2),
        }
        .bucket();
        assert_eq!(a.template, b.template);
        assert_ne!(a.major, b.major);
        assert_eq!(a.template, "POST /channels/{channel_id}/messages");
        assert_eq!(Route::GetGatewayBot.bucket().major, 0);

        let followup = |token| {
            Route::CreateFollowupMessage {
                application_id: ApplicationId::new(3),
                interaction_token: token,
            }
            .bucket()
        };
        assert_ne!(followup("a").major, followup("b").major);
        assert_eq!(followup("a").major, followup("a").major);

        let guild_command = |guild| {
            Route::CreateGuildApplicationCommand {
                application_id: ApplicationId::new(3),
                guild_id: GuildId::new(guild),
            }
            .bucket()
        };
        assert_eq!(guild_command(4), guild_command(5));
        assert_eq!(guild_command(4).major, 0);
    }

    #[test]
    fn old_message_deletes_have_their_own_bucket() {
        let created_ms = 1_794_398_400_000;
        let route = Route::DeleteMessage {
            channel_id: ChannelId::new(1),
            message_id: MessageId::from_timestamp_ms(created_ms).unwrap(),
        };
        assert_eq!(
            route.path(),
            format!(
                "/channels/1/messages/{}",
                MessageId::from_timestamp_ms(created_ms).unwrap()
            )
        );
        let new = route.bucket_at(created_ms + OLD_MESSAGE_AGE_MS);
        assert_eq!(
            new.template,
            "DELETE /channels/{channel_id}/messages/{message_id}"
        );
        let old = route.bucket_at(created_ms + OLD_MESSAGE_AGE_MS + 1);
        assert_eq!(old.template, OLD_MESSAGE_DELETE);

        assert_eq!((old.major, old.limits), (new.major, new.limits));

        assert_eq!(route.bucket_at(0).template, new.template);

        let ancient = Route::DeleteMessage {
            channel_id: ChannelId::new(1),
            message_id: MessageId::new(1 << 22),
        };
        assert_eq!(ancient.bucket().template, OLD_MESSAGE_DELETE);
    }

    #[test]
    fn interaction_routes_are_not_global() {
        let callback = Route::CreateInteractionResponse {
            interaction_id: InteractionId::new(2),
            interaction_token: TOKEN,
        }
        .bucket();
        assert_eq!(callback.limits, Limits::INTERACTION_CALLBACK);
        assert!(!callback.limits.global && callback.limits.burst);
        let original = Route::GetOriginalInteractionResponse {
            application_id: ApplicationId::new(3),
            interaction_token: TOKEN,
        }
        .bucket();
        assert!(!original.limits.global && !original.limits.burst);
        assert!(Route::GetGatewayBot.bucket().limits.global);
    }
}
