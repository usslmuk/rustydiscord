use std::sync::Arc;

#[cfg(feature = "gateway")]
use rustydiscord_gateway::ReceiveError;

#[cfg(not(feature = "gateway"))]
type ReceiveError = core::convert::Infallible;
use rustydiscord_model::gateway::{CloseCode, DispatchEvent, Intents, ShardId, payload as gateway};

pub mod payload;

macro_rules! events {
    ($(
        $(#[$doc:meta])*
        $name:ident = $js:literal => $payload:ty [$($intent:ident)|*];
    )*) => {








        #[derive(Clone, Copy, Debug)]
        #[non_exhaustive]
        pub struct Events;

        #[expect(
            non_upper_case_globals,
            reason = "event names mirror discord.js' `Events.MessageCreate` style"
        )]
        impl Events {
            $(
                $(#[$doc])*
                pub const $name: $name = $name;
            )*
        }

        $(
            $(#[$doc])*
            #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
            #[non_exhaustive]
            pub struct $name;

            impl sealed::Sealed for $name {}

            impl Event for $name {
                type Payload = $payload;
                const NAME: &'static str = $js;
                const KIND: EventKind = EventKind::$name;
                const INTENTS: Intents = Intents::empty()$(.union(Intents::$intent))*;

                fn payload(dispatch: &Dispatch) -> Option<&Arc<$payload>> {
                    match dispatch {
                        Dispatch::$name(payload) => Some(payload),
                        _ => None,
                    }
                }
            }
        )*


        #[doc(hidden)]
        #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
        pub enum EventKind {
            $($name,)*
        }

        impl EventKind {

            pub(crate) const COUNT: usize = [$(EventKind::$name),*].len();


            pub(crate) const ALL: [Self; Self::COUNT] = [$(EventKind::$name),*];


            pub(crate) const fn intents(self) -> Intents {
                match self {
                    $(Self::$name => <$name as Event>::INTENTS,)*
                }
            }


            pub(crate) const fn name(self) -> &'static str {
                match self {
                    $(Self::$name => $js,)*
                }
            }
        }








        #[derive(Clone, Debug)]
        #[non_exhaustive]
        pub enum Dispatch {
            $(
                #[doc = concat!("The [`Events::", stringify!($name), "`] event.")]
                $name(Arc<$payload>),
            )*

            Unknown(Arc<str>),
        }

        impl Dispatch {


            #[must_use]
            pub fn name(&self) -> &str {
                match self {
                    $(Self::$name(_) => $js,)*
                    Self::Unknown(name) => name,
                }
            }


            pub(crate) const fn kind(&self) -> Option<EventKind> {
                match self {
                    $(Self::$name(_) => Some(EventKind::$name),)*
                    Self::Unknown(_) => None,
                }
            }
        }
    };
}

pub trait Event: Copy + Send + Sync + 'static + sealed::Sealed {
    type Payload: Send + Sync + 'static;

    const NAME: &'static str;

    #[doc(hidden)]
    const KIND: EventKind;

    const INTENTS: Intents;

    #[doc(hidden)]
    fn payload(dispatch: &Dispatch) -> Option<&Arc<Self::Payload>>;
}

mod sealed {
    pub trait Sealed {}
}

events! {










    ClientReady = "clientReady" => gateway::Ready [];








    ShardReady = "shardReady" => gateway::Ready [];





    ShardResume = "shardResume" => payload::ShardResume [];








    ShardReconnecting = "shardReconnecting" => payload::ShardReconnecting [];






    Invalidated = "invalidated" => payload::Invalidated [];





    ShardError = "shardError" => ReceiveError [];






    ShardDisconnect = "shardDisconnect" => CloseCode [];






    Raw = "raw" => DispatchEvent [];

    Debug = "debug" => String [];

    Warn = "warn" => String [];

    Error = "error" => String [];






    ApplicationCommandPermissionsUpdate = "applicationCommandPermissionsUpdate"
        => gateway::ApplicationCommandPermissionsUpdate [];





    AutoModerationActionExecution = "autoModerationActionExecution"
        => gateway::AutoModerationActionExecution [AUTO_MODERATION_EXECUTION];





    AutoModerationRuleCreate = "autoModerationRuleCreate" => gateway::AutoModerationRuleCreate [AUTO_MODERATION_CONFIGURATION];







    AutoModerationRuleUpdate = "autoModerationRuleUpdate" => gateway::AutoModerationRuleUpdate [AUTO_MODERATION_CONFIGURATION];





    AutoModerationRuleDelete = "autoModerationRuleDelete" => gateway::AutoModerationRuleDelete [AUTO_MODERATION_CONFIGURATION];




    EntitlementCreate = "entitlementCreate" => gateway::EntitlementCreate [];




    EntitlementUpdate = "entitlementUpdate" => gateway::EntitlementUpdate [];




    EntitlementDelete = "entitlementDelete" => gateway::EntitlementDelete [];




    SubscriptionCreate = "subscriptionCreate" => gateway::SubscriptionCreate [];




    SubscriptionUpdate = "subscriptionUpdate" => gateway::SubscriptionUpdate [];




    SubscriptionDelete = "subscriptionDelete" => gateway::SubscriptionDelete [];






    ChannelCreate = "channelCreate" => gateway::ChannelCreate [GUILDS];





    ChannelUpdate = "channelUpdate" => payload::ChannelUpdate [GUILDS];




    ChannelDelete = "channelDelete" => payload::ChannelDelete [GUILDS];





    ChannelPinsUpdate = "channelPinsUpdate" => gateway::ChannelPinsUpdate [GUILDS | DIRECT_MESSAGES];





    ChannelInfo = "channelInfo" => gateway::ChannelInfo [];




    ThreadCreate = "threadCreate" => gateway::ThreadCreate [GUILDS];





    ThreadUpdate = "threadUpdate" => payload::ThreadUpdate [GUILDS];




    ThreadDelete = "threadDelete" => payload::ThreadDelete [GUILDS];





    ThreadListSync = "threadListSync" => gateway::ThreadListSync [GUILDS];






    ThreadMemberUpdate = "threadMemberUpdate" => gateway::ThreadMemberUpdate [GUILDS];





    ThreadMembersUpdate = "threadMembersUpdate" => gateway::ThreadMembersUpdate [GUILDS];










    GuildCreate = "guildCreate" => gateway::GuildCreate [GUILDS];





    GuildAvailable = "guildAvailable" => gateway::GuildCreate [GUILDS];





    GuildUpdate = "guildUpdate" => payload::GuildUpdate [GUILDS];





    GuildDelete = "guildDelete" => payload::GuildDelete [GUILDS];





    GuildUnavailable = "guildUnavailable" => payload::GuildUnavailable [GUILDS];





    GuildAuditLogEntryCreate = "guildAuditLogEntryCreate" => gateway::GuildAuditLogEntryCreate [GUILD_MODERATION];




    GuildBanAdd = "guildBanAdd" => gateway::GuildBanAdd [GUILD_MODERATION];




    GuildBanRemove = "guildBanRemove" => gateway::GuildBanRemove [GUILD_MODERATION];








    GuildEmojisUpdate = "guildEmojisUpdate" => gateway::GuildEmojisUpdate [GUILD_EXPRESSIONS];




    GuildEmojiCreate = "emojiCreate" => payload::GuildEmojiCreate [GUILD_EXPRESSIONS];




    GuildEmojiUpdate = "emojiUpdate" => payload::GuildEmojiUpdate [GUILD_EXPRESSIONS];




    GuildEmojiDelete = "emojiDelete" => payload::GuildEmojiDelete [GUILD_EXPRESSIONS];








    GuildStickersUpdate = "guildStickersUpdate" => gateway::GuildStickersUpdate [GUILD_EXPRESSIONS];




    GuildStickerCreate = "stickerCreate" => payload::GuildStickerCreate [GUILD_EXPRESSIONS];




    GuildStickerUpdate = "stickerUpdate" => payload::GuildStickerUpdate [GUILD_EXPRESSIONS];




    GuildStickerDelete = "stickerDelete" => payload::GuildStickerDelete [GUILD_EXPRESSIONS];





    GuildIntegrationsUpdate = "guildIntegrationsUpdate" => gateway::GuildIntegrationsUpdate [GUILD_INTEGRATIONS];




    IntegrationCreate = "integrationCreate" => gateway::IntegrationCreate [GUILD_INTEGRATIONS];




    IntegrationUpdate = "integrationUpdate" => gateway::IntegrationUpdate [GUILD_INTEGRATIONS];




    IntegrationDelete = "integrationDelete" => gateway::IntegrationDelete [GUILD_INTEGRATIONS];




    GuildMemberAdd = "guildMemberAdd" => gateway::GuildMemberAdd [GUILD_MEMBERS];





    GuildMemberUpdate = "guildMemberUpdate" => payload::GuildMemberUpdate [GUILD_MEMBERS];





    GuildMemberRemove = "guildMemberRemove" => payload::GuildMemberRemove [GUILD_MEMBERS];

    GuildMemberAvailable = "guildMemberAvailable" => payload::GuildMemberAvailable [GUILD_MEMBERS];





    GuildMembersChunk = "guildMembersChunk" => gateway::GuildMembersChunk [];




    GuildRoleCreate = "roleCreate" => gateway::GuildRoleCreate [GUILDS];





    GuildRoleUpdate = "roleUpdate" => payload::GuildRoleUpdate [GUILDS];




    GuildRoleDelete = "roleDelete" => payload::GuildRoleDelete [GUILDS];





    GuildScheduledEventCreate = "guildScheduledEventCreate"
        => gateway::GuildScheduledEventCreate [GUILD_SCHEDULED_EVENTS];





    GuildScheduledEventUpdate = "guildScheduledEventUpdate"
        => payload::GuildScheduledEventUpdate [GUILD_SCHEDULED_EVENTS];





    GuildScheduledEventDelete = "guildScheduledEventDelete"
        => payload::GuildScheduledEventDelete [GUILD_SCHEDULED_EVENTS];





    GuildScheduledEventUserAdd = "guildScheduledEventUserAdd"
        => gateway::GuildScheduledEventUserAdd [GUILD_SCHEDULED_EVENTS];





    GuildScheduledEventUserRemove = "guildScheduledEventUserRemove"
        => gateway::GuildScheduledEventUserRemove [GUILD_SCHEDULED_EVENTS];





    GuildSoundboardSoundCreate = "guildSoundboardSoundCreate"
        => gateway::GuildSoundboardSoundCreate [GUILD_EXPRESSIONS];





    GuildSoundboardSoundUpdate = "guildSoundboardSoundUpdate"
        => payload::GuildSoundboardSoundUpdate [GUILD_EXPRESSIONS];





    GuildSoundboardSoundDelete = "guildSoundboardSoundDelete"
        => payload::GuildSoundboardSoundDelete [GUILD_EXPRESSIONS];





    GuildSoundboardSoundsUpdate = "guildSoundboardSoundsUpdate"
        => gateway::GuildSoundboardSoundsUpdate [GUILD_EXPRESSIONS];






    SoundboardSounds = "soundboardSounds" => gateway::SoundboardSounds [];




    StageInstanceCreate = "stageInstanceCreate" => gateway::StageInstanceCreate [GUILDS];





    StageInstanceUpdate = "stageInstanceUpdate" => payload::StageInstanceUpdate [GUILDS];





    StageInstanceDelete = "stageInstanceDelete" => payload::StageInstanceDelete [GUILDS];






    InteractionCreate = "interactionCreate" => gateway::InteractionCreate [];




    InviteCreate = "inviteCreate" => gateway::InviteCreate [GUILD_INVITES];




    InviteDelete = "inviteDelete" => gateway::InviteDelete [GUILD_INVITES];






    MessageCreate = "messageCreate" => gateway::MessageCreate [GUILD_MESSAGES | DIRECT_MESSAGES];





    MessageUpdate = "messageUpdate" => payload::MessageUpdate [GUILD_MESSAGES | DIRECT_MESSAGES];





    MessageDelete = "messageDelete" => payload::MessageDelete [GUILD_MESSAGES | DIRECT_MESSAGES];





    MessageBulkDelete = "messageDeleteBulk" => payload::MessageBulkDelete [GUILD_MESSAGES];





    MessageReactionAdd = "messageReactionAdd" => gateway::MessageReactionAdd [GUILD_MESSAGE_REACTIONS | DIRECT_MESSAGE_REACTIONS];





    MessageReactionRemove = "messageReactionRemove" => gateway::MessageReactionRemove [GUILD_MESSAGE_REACTIONS | DIRECT_MESSAGE_REACTIONS];





    MessageReactionRemoveAll = "messageReactionRemoveAll" => gateway::MessageReactionRemoveAll [GUILD_MESSAGE_REACTIONS | DIRECT_MESSAGE_REACTIONS];





    MessageReactionRemoveEmoji = "messageReactionRemoveEmoji"
        => gateway::MessageReactionRemoveEmoji [GUILD_MESSAGE_REACTIONS | DIRECT_MESSAGE_REACTIONS];





    MessagePollVoteAdd = "messagePollVoteAdd" => gateway::MessagePollVoteAdd [GUILD_MESSAGE_POLLS | DIRECT_MESSAGE_POLLS];





    MessagePollVoteRemove = "messagePollVoteRemove" => gateway::MessagePollVoteRemove [GUILD_MESSAGE_POLLS | DIRECT_MESSAGE_POLLS];





    TypingStart = "typingStart" => gateway::TypingStart [GUILD_MESSAGE_TYPING | DIRECT_MESSAGE_TYPING];







    PresenceUpdate = "presenceUpdate" => payload::PresenceUpdate [GUILD_PRESENCES];




    UserUpdate = "userUpdate" => payload::UserUpdate [];






    VoiceStateUpdate = "voiceStateUpdate" => payload::VoiceStateUpdate [GUILD_VOICE_STATES];




    VoiceServerUpdate = "voiceServerUpdate" => gateway::VoiceServerUpdate [];





    VoiceChannelEffectSend = "voiceChannelEffectSend" => gateway::VoiceChannelEffectSend [GUILD_VOICE_STATES];





    VoiceChannelStatusUpdate = "voiceChannelStatusUpdate" => gateway::VoiceChannelStatusUpdate [GUILDS];





    VoiceChannelStartTimeUpdate = "voiceChannelStartTimeUpdate"
        => gateway::VoiceChannelStartTimeUpdate [GUILDS];




    WebhooksUpdate = "webhooksUpdate" => gateway::WebhooksUpdate [GUILD_WEBHOOKS];





    RateLimited = "rateLimited" => gateway::RateLimited [];

    CacheSweep = "cacheSweep" => String [];
}

#[derive(Clone, Debug)]
#[non_exhaustive]
pub struct ShardEvent {
    pub shard: ShardId,

    pub event: Dispatch,
}
