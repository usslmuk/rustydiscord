use compact_str::CompactString;
use serde::{Deserialize, Serialize};

use crate::{
    emoji::{Emoji, PartialEmoji},
    gateway::Status,
    id::{
        ApplicationId, ChannelId, EmojiId, GenericId, GuildId, IntegrationId, RoleId, SkuId, UserId,
    },
    macros::{int_enum, int_flags, str_enum},
    oauth2::Scope,
    permissions::Permissions,
    sticker::Sticker,
    user::{AvatarDecorationData, Collectibles, User},
    util::{ImageHash, Locale, Timestamp},
};

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[non_exhaustive]
pub struct UnavailableGuild {
    pub id: GuildId,

    #[serde(default)]
    pub unavailable: bool,
}

impl UnavailableGuild {
    #[must_use]
    pub const fn new(id: GuildId, unavailable: bool) -> Self {
        Self { id, unavailable }
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct Member {
    pub user: User,

    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub nick: Option<CompactString>,

    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub avatar: Option<ImageHash>,

    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub banner: Option<ImageHash>,

    pub roles: Vec<RoleId>,

    pub joined_at: Option<Timestamp>,

    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub premium_since: Option<Timestamp>,

    #[serde(default)]
    pub deaf: bool,

    #[serde(default)]
    pub mute: bool,

    #[serde(default)]
    pub flags: MemberFlags,

    #[serde(default, skip_serializing_if = "core::ops::Not::not")]
    pub pending: bool,

    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub communication_disabled_until: Option<Timestamp>,

    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub avatar_decoration_data: Option<Box<AvatarDecorationData>>,

    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub collectibles: Option<Box<Collectibles>>,
}

impl Member {
    #[must_use]
    pub fn display_name(&self) -> &str {
        self.nick
            .as_deref()
            .unwrap_or_else(|| self.user.display_name())
    }

    #[must_use]
    pub fn is_timed_out(&self, now: Timestamp) -> bool {
        self.communication_disabled_until
            .is_some_and(|until| until > now)
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct PartialMember {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub nick: Option<CompactString>,

    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub avatar: Option<ImageHash>,

    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub banner: Option<ImageHash>,

    pub roles: Vec<RoleId>,

    pub joined_at: Option<Timestamp>,

    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub premium_since: Option<Timestamp>,

    #[serde(default)]
    pub deaf: bool,

    #[serde(default)]
    pub mute: bool,

    #[serde(default)]
    pub flags: MemberFlags,

    #[serde(default, skip_serializing_if = "core::ops::Not::not")]
    pub pending: bool,

    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub communication_disabled_until: Option<Timestamp>,

    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub avatar_decoration_data: Option<Box<AvatarDecorationData>>,

    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub collectibles: Option<Box<Collectibles>>,
}

impl PartialMember {
    #[must_use]
    pub fn minimal() -> Self {
        Self {
            nick: None,
            avatar: None,
            banner: None,
            roles: Vec::new(),
            joined_at: None,
            premium_since: None,
            deaf: false,
            mute: false,
            flags: MemberFlags::empty(),
            pending: false,
            communication_disabled_until: None,
            avatar_decoration_data: None,
            collectibles: None,
        }
    }

    #[must_use]
    pub fn into_member(self, user: User) -> Member {
        Member {
            user,
            nick: self.nick,
            avatar: self.avatar,
            banner: self.banner,
            roles: self.roles,
            joined_at: self.joined_at,
            premium_since: self.premium_since,
            deaf: self.deaf,
            mute: self.mute,
            flags: self.flags,
            pending: self.pending,
            communication_disabled_until: self.communication_disabled_until,
            avatar_decoration_data: self.avatar_decoration_data,
            collectibles: self.collectibles,
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct Role {
    pub id: RoleId,

    pub name: CompactString,

    pub color: u32,

    pub colors: RoleColors,

    pub hoist: bool,

    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub icon: Option<ImageHash>,

    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub unicode_emoji: Option<CompactString>,

    pub position: i32,

    pub permissions: Permissions,

    pub managed: bool,

    pub mentionable: bool,

    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tags: Option<Box<RoleTags>>,

    #[serde(default)]
    pub flags: RoleFlags,
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct RoleColors {
    pub primary_color: u32,

    pub secondary_color: Option<u32>,

    pub tertiary_color: Option<u32>,
}

impl RoleColors {
    pub const HOLOGRAPHIC: Self = Self {
        primary_color: 11_127_295,
        secondary_color: Some(16_759_788),
        tertiary_color: Some(16_761_760),
    };

    #[must_use]
    pub const fn solid(color: u32) -> Self {
        Self {
            primary_color: color,
            secondary_color: None,
            tertiary_color: None,
        }
    }
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[non_exhaustive]
pub struct RoleTags {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub bot_id: Option<UserId>,

    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub integration_id: Option<IntegrationId>,

    #[serde(
        default,
        with = "null_flag",
        skip_serializing_if = "core::ops::Not::not"
    )]
    pub premium_subscriber: bool,

    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub subscription_listing_id: Option<SkuId>,

    #[serde(
        default,
        with = "null_flag",
        skip_serializing_if = "core::ops::Not::not"
    )]
    pub available_for_purchase: bool,

    #[serde(
        default,
        with = "null_flag",
        skip_serializing_if = "core::ops::Not::not"
    )]
    pub guild_connections: bool,
}

mod null_flag {
    use serde::{Deserialize, Deserializer, Serializer};

    #[expect(
        clippy::trivially_copy_pass_by_ref,
        reason = "signature required by serde(with)"
    )]
    pub(super) fn serialize<S: Serializer>(_: &bool, serializer: S) -> Result<S::Ok, S::Error> {
        serializer.serialize_unit()
    }

    pub(super) fn deserialize<'de, D: Deserializer<'de>>(
        deserializer: D,
    ) -> Result<bool, D::Error> {
        Option::<()>::deserialize(deserializer).map(|_| true)
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct Guild {
    pub id: GuildId,

    pub name: CompactString,

    pub icon: Option<ImageHash>,

    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub icon_hash: Option<ImageHash>,

    pub splash: Option<ImageHash>,

    pub discovery_splash: Option<ImageHash>,

    #[serde(default, skip_serializing_if = "core::ops::Not::not")]
    pub owner: bool,

    pub owner_id: UserId,

    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub permissions: Option<Permissions>,

    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub region: Option<CompactString>,

    pub afk_channel_id: Option<ChannelId>,

    pub afk_timeout: u32,

    #[serde(default, skip_serializing_if = "core::ops::Not::not")]
    pub widget_enabled: bool,

    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub widget_channel_id: Option<ChannelId>,

    pub verification_level: VerificationLevel,

    pub default_message_notifications: DefaultMessageNotificationLevel,

    pub explicit_content_filter: ExplicitContentFilterLevel,

    pub roles: Vec<Role>,

    pub emojis: Vec<Emoji>,

    pub features: Vec<Feature>,

    pub mfa_level: MfaLevel,

    pub application_id: Option<ApplicationId>,

    pub system_channel_id: Option<ChannelId>,

    pub system_channel_flags: SystemChannelFlags,

    pub rules_channel_id: Option<ChannelId>,

    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub max_presences: Option<u32>,

    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub max_members: Option<u32>,

    pub vanity_url_code: Option<CompactString>,

    pub description: Option<CompactString>,

    pub banner: Option<ImageHash>,

    pub premium_tier: PremiumTier,

    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub premium_subscription_count: Option<u32>,

    pub preferred_locale: Locale,

    pub public_updates_channel_id: Option<ChannelId>,

    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub max_video_channel_users: Option<u32>,

    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub max_stage_video_channel_users: Option<u32>,

    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub approximate_member_count: Option<u32>,

    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub approximate_presence_count: Option<u32>,

    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub welcome_screen: Option<Box<WelcomeScreen>>,

    #[serde(default)]
    pub nsfw_level: AgeRestrictionLevel,

    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub stickers: Vec<Sticker>,

    #[serde(default)]
    pub premium_progress_bar_enabled: bool,

    pub safety_alerts_channel_id: Option<ChannelId>,

    pub incidents_data: Option<Box<IncidentsData>>,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct PartialGuild {
    pub id: GuildId,

    pub name: CompactString,

    pub icon: Option<ImageHash>,

    #[serde(default)]
    pub splash: Option<ImageHash>,

    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub banner: Option<ImageHash>,

    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<CompactString>,

    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub features: Vec<Feature>,

    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub verification_level: Option<VerificationLevel>,

    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub vanity_url_code: Option<CompactString>,

    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub welcome_screen: Option<Box<WelcomeScreen>>,
}

int_enum! {




    pub enum DefaultMessageNotificationLevel: u8 {

        AllMessages = 0,

        OnlyMentions = 1,
    }
}

int_enum! {




    pub enum ExplicitContentFilterLevel: u8 {

        Disabled = 0,

        MembersWithoutRoles = 1,

        AllMembers = 2,
    }
}

int_enum! {




    pub enum MfaLevel: u8 {

        None = 0,

        Elevated = 1,
    }
}

int_enum! {




    pub enum VerificationLevel: u8 {

        None = 0,

        Low = 1,

        Medium = 2,

        High = 3,

        VeryHigh = 4,
    }
}

int_enum! {




    #[doc(alias = "NsfwLevel")]
    #[derive(Default)]
    pub enum AgeRestrictionLevel: u8 {

        #[default]
        Default = 0,

        Explicit = 1,

        Safe = 2,

        AgeRestricted = 3,
    }
}

int_enum! {




    pub enum PremiumTier: u8 {

        None = 0,

        Tier1 = 1,

        Tier2 = 2,

        Tier3 = 3,
    }
}

str_enum! {







    #[doc(alias = "GuildFeature")]
    pub enum Feature {

        AnimatedBanner = "ANIMATED_BANNER",

        AnimatedIcon = "ANIMATED_ICON",

        ApplicationCommandPermissionsV2 = "APPLICATION_COMMAND_PERMISSIONS_V2",

        AutoModeration = "AUTO_MODERATION",

        Banner = "BANNER",


        Community = "COMMUNITY",

        CreatorMonetizableProvisional = "CREATOR_MONETIZABLE_PROVISIONAL",

        CreatorStorePage = "CREATOR_STORE_PAGE",

        DeveloperSupportServer = "DEVELOPER_SUPPORT_SERVER",

        Discoverable = "DISCOVERABLE",

        EnhancedRoleColors = "ENHANCED_ROLE_COLORS",

        Featurable = "FEATURABLE",

        GuestsEnabled = "GUESTS_ENABLED",

        GuildTags = "GUILD_TAGS",




        HasDirectoryEntry = "HAS_DIRECTORY_ENTRY",




        Hub = "HUB",

        InvitesDisabled = "INVITES_DISABLED",

        InviteSplash = "INVITE_SPLASH",




        LinkedToHub = "LINKED_TO_HUB",

        MemberVerificationGateEnabled = "MEMBER_VERIFICATION_GATE_ENABLED",




        MonetizationEnabled = "MONETIZATION_ENABLED",

        MoreSoundboard = "MORE_SOUNDBOARD",

        MoreStickers = "MORE_STICKERS",

        News = "NEWS",

        Partnered = "PARTNERED",




        PinPermissionMigrationComplete = "PIN_PERMISSION_MIGRATION_COMPLETE",

        PreviewEnabled = "PREVIEW_ENABLED",




        PrivateThreads = "PRIVATE_THREADS",

        PruneRequiresAdmin = "PRUNE_REQUIRES_ADMIN",

        RaidAlertsDisabled = "RAID_ALERTS_DISABLED",




        RelayEnabled = "RELAY_ENABLED",

        RoleIcons = "ROLE_ICONS",

        RoleSubscriptionsAvailableForPurchase = "ROLE_SUBSCRIPTIONS_AVAILABLE_FOR_PURCHASE",

        RoleSubscriptionsEnabled = "ROLE_SUBSCRIPTIONS_ENABLED",

        Soundboard = "SOUNDBOARD",

        TicketedEventsEnabled = "TICKETED_EVENTS_ENABLED",

        VanityUrl = "VANITY_URL",

        Verified = "VERIFIED",

        VipRegions = "VIP_REGIONS",

        WelcomeScreenEnabled = "WELCOME_SCREEN_ENABLED",
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[non_exhaustive]
#[doc(alias = "GuildPreview")]
pub struct Preview {
    pub id: GuildId,

    pub name: CompactString,

    pub icon: Option<ImageHash>,

    pub splash: Option<ImageHash>,

    pub discovery_splash: Option<ImageHash>,

    pub emojis: Vec<Emoji>,

    pub features: Vec<Feature>,

    pub approximate_member_count: u32,

    pub approximate_presence_count: u32,

    pub description: Option<CompactString>,

    pub stickers: Vec<Sticker>,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[non_exhaustive]
pub struct WidgetSettings {
    pub enabled: bool,

    pub channel_id: Option<ChannelId>,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct Widget {
    pub id: GuildId,

    pub name: CompactString,

    pub instant_invite: Option<CompactString>,

    pub channels: Vec<WidgetChannel>,

    pub members: Vec<WidgetMember>,

    pub presence_count: u32,
}

#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[non_exhaustive]
pub struct WidgetChannel {
    pub id: ChannelId,

    pub name: CompactString,

    pub position: i32,
}

#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[non_exhaustive]
pub struct WidgetMember {
    pub id: CompactString,

    pub username: CompactString,

    pub discriminator: CompactString,

    pub avatar: Option<ImageHash>,

    pub status: Status,

    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub activity: Option<WidgetActivity>,

    pub avatar_url: CompactString,
}

#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[non_exhaustive]
pub struct WidgetActivity {
    pub name: CompactString,
}

str_enum! {




    pub enum WidgetStyle {

        Shield = "shield",

        Banner1 = "banner1",


        Banner2 = "banner2",

        Banner3 = "banner3",


        Banner4 = "banner4",
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct Integration {
    pub id: IntegrationId,

    pub name: CompactString,

    #[serde(rename = "type")]
    pub kind: IntegrationType,

    pub enabled: bool,

    #[serde(default, skip_serializing_if = "core::ops::Not::not")]
    pub syncing: bool,

    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub role_id: Option<RoleId>,

    #[serde(default, skip_serializing_if = "core::ops::Not::not")]
    pub enable_emoticons: bool,

    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub expire_behavior: Option<IntegrationExpireBehavior>,

    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub expire_grace_period: Option<u32>,

    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub user: Option<Box<User>>,

    pub account: IntegrationAccount,

    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub synced_at: Option<Timestamp>,

    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub subscriber_count: Option<u32>,

    #[serde(default, skip_serializing_if = "core::ops::Not::not")]
    pub revoked: bool,

    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub application: Option<Box<IntegrationApplication>>,

    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub scopes: Vec<Scope>,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct PartialIntegration {
    pub id: IntegrationId,

    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<CompactString>,

    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub kind: Option<IntegrationType>,

    #[serde(default, skip_serializing_if = "core::ops::Not::not")]
    pub enabled: bool,

    #[serde(default, skip_serializing_if = "core::ops::Not::not")]
    pub syncing: bool,

    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub role_id: Option<RoleId>,

    #[serde(default, skip_serializing_if = "core::ops::Not::not")]
    pub enable_emoticons: bool,

    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub expire_behavior: Option<IntegrationExpireBehavior>,

    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub expire_grace_period: Option<u32>,

    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub user: Option<Box<User>>,

    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub account: Option<IntegrationAccount>,

    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub synced_at: Option<Timestamp>,

    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub subscriber_count: Option<u32>,

    #[serde(default, skip_serializing_if = "core::ops::Not::not")]
    pub revoked: bool,

    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub application: Option<Box<IntegrationApplication>>,

    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub application_id: Option<ApplicationId>,

    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub scopes: Vec<Scope>,
}

str_enum! {



    pub enum IntegrationType {

        Twitch = "twitch",

        Youtube = "youtube",

        Discord = "discord",

        GuildSubscription = "guild_subscription",
    }
}

int_enum! {




    pub enum IntegrationExpireBehavior: u8 {

        RemoveRole = 0,

        Kick = 1,
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[non_exhaustive]
pub struct IntegrationAccount {
    pub id: CompactString,

    pub name: CompactString,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct IntegrationApplication {
    pub id: ApplicationId,

    pub name: CompactString,

    pub icon: Option<ImageHash>,

    pub description: CompactString,

    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub bot: Option<User>,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct Ban {
    pub reason: Option<CompactString>,

    pub user: User,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct WelcomeScreen {
    pub description: Option<CompactString>,

    pub welcome_channels: Vec<WelcomeScreenChannel>,
}

#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[non_exhaustive]
pub struct WelcomeScreenChannel {
    pub channel_id: ChannelId,

    pub description: CompactString,

    pub emoji_id: Option<EmojiId>,

    pub emoji_name: Option<CompactString>,
}

impl WelcomeScreenChannel {
    #[must_use]
    pub fn new(channel_id: ChannelId, description: impl Into<CompactString>) -> Self {
        Self {
            channel_id,
            description: description.into(),
            emoji_id: None,
            emoji_name: None,
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[non_exhaustive]
#[doc(alias = "GuildOnboarding")]
pub struct Onboarding {
    pub guild_id: GuildId,

    pub prompts: Vec<OnboardingPrompt>,

    pub default_channel_ids: Vec<ChannelId>,

    pub enabled: bool,

    #[serde(default)]
    pub mode: OnboardingMode,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct OnboardingPrompt {
    pub id: GenericId,

    #[serde(rename = "type")]
    pub kind: PromptType,

    pub options: Vec<PromptOption>,

    pub title: CompactString,

    pub single_select: bool,

    pub required: bool,

    pub in_onboarding: bool,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct PromptOption {
    pub id: GenericId,

    pub channel_ids: Vec<ChannelId>,

    pub role_ids: Vec<RoleId>,

    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub emoji: Option<PartialEmoji>,

    pub title: CompactString,

    pub description: Option<CompactString>,
}

int_enum! {




    #[derive(Default)]
    pub enum OnboardingMode: u8 {

        #[default]
        Default = 0,

        Advanced = 1,
    }
}

int_enum! {



    pub enum PromptType: u8 {

        MultipleChoice = 0,

        Dropdown = 1,
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[non_exhaustive]
pub struct IncidentsData {
    pub invites_disabled_until: Option<Timestamp>,

    pub dms_disabled_until: Option<Timestamp>,

    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub dm_spam_detected_at: Option<Timestamp>,

    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub raid_detected_at: Option<Timestamp>,
}

int_flags! {




    pub struct SystemChannelFlags: u64 {

        const SUPPRESS_JOIN_NOTIFICATIONS = 1 << 0;

        const SUPPRESS_PREMIUM_SUBSCRIPTIONS = 1 << 1;

        const SUPPRESS_GUILD_REMINDER_NOTIFICATIONS = 1 << 2;

        const SUPPRESS_JOIN_NOTIFICATION_REPLIES = 1 << 3;

        const SUPPRESS_ROLE_SUBSCRIPTION_PURCHASE_NOTIFICATIONS = 1 << 4;

        const SUPPRESS_ROLE_SUBSCRIPTION_PURCHASE_NOTIFICATION_REPLIES = 1 << 5;
    }
}

int_flags! {






    pub struct MemberFlags: u64 {

        const DID_REJOIN = 1 << 0;

        const COMPLETED_ONBOARDING = 1 << 1;

        const BYPASSES_VERIFICATION = 1 << 2;

        const STARTED_ONBOARDING = 1 << 3;

        const IS_GUEST = 1 << 4;

        const STARTED_HOME_ACTIONS = 1 << 5;

        const COMPLETED_HOME_ACTIONS = 1 << 6;

        const AUTOMOD_QUARANTINED_USERNAME = 1 << 7;






        const AUTOMOD_QUARANTINED_BIO = 1 << 8;

        const DM_SETTINGS_UPSELL_ACKNOWLEDGED = 1 << 9;

        const AUTOMOD_QUARANTINED_GUILD_TAG = 1 << 10;
    }
}

int_flags! {



    pub struct RoleFlags: u64 {

        const IN_PROMPT = 1 << 0;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const USER: &str = r#"{"id":"80351110224678912","username":"nelly","discriminator":"0","global_name":"Nelly","avatar":null}"#;

    #[test]
    fn member_from_documented_example() {
        let json = format!(
            r#"{{"user":{USER},"nick":"NOT API SUPPORT","avatar":null,"banner":null,"roles":[],
                "joined_at":"2015-04-26T06:26:56.936000+00:00","deaf":false,"mute":false}}"#
        );
        let member: Member = serde_json::from_str(&json).unwrap();
        assert_eq!(member.user.id.get(), 80_351_110_224_678_912);
        assert_eq!(member.nick.as_deref(), Some("NOT API SUPPORT"));
        assert_eq!(member.avatar, None);
        assert_eq!(
            member.joined_at.unwrap().to_string(),
            "2015-04-26T06:26:56.936000+00:00"
        );
        assert_eq!(member.flags, MemberFlags::empty());
        assert!(!member.pending);
        assert_eq!(member.display_name(), "NOT API SUPPORT");
    }

    #[test]
    fn partial_member_from_message_create() {
        let json = r#"{"roles":["1100000000000000001"],"premium_since":null,"pending":false,
            "nick":null,"mute":false,"joined_at":"2024-05-01T12:00:00.000000+00:00","flags":2,
            "deaf":false,"communication_disabled_until":"2026-11-10T20:00:00.000000+00:00",
            "banner":null,"avatar":null}"#;
        let partial: PartialMember = serde_json::from_str(json).unwrap();
        assert_eq!(partial.roles.len(), 1);
        assert_eq!(partial.flags, MemberFlags::COMPLETED_ONBOARDING);
        let user: User = serde_json::from_str(USER).unwrap();
        let member = partial.into_member(user);
        assert_eq!(member.display_name(), "Nelly");
        let before: Timestamp = "2026-11-10T19:00:00.000000+00:00".parse().unwrap();
        let after: Timestamp = "2026-11-10T21:00:00.000000+00:00".parse().unwrap();
        assert!(member.is_timed_out(before));
        assert!(!member.is_timed_out(after));
    }

    #[test]
    fn role_from_documented_example() {
        let json = r#"{"id":"41771983423143936","name":"WE DEM BOYZZ!!!!!!","color":3447003,
            "colors":{"primary_color":3447003,"secondary_color":null,"tertiary_color":null},
            "hoist":true,"icon":"cf3ced8600b777c9486c6d8d84fb4327","unicode_emoji":null,
            "position":1,"permissions":"66321471","managed":false,"mentionable":false,"flags":0}"#;
        let role: Role = serde_json::from_str(json).unwrap();
        assert_eq!(role.colors, RoleColors::solid(3_447_003));
        assert_eq!(role.permissions.bits(), 66_321_471);
        assert!(role.permissions.contains(Permissions::SEND_MESSAGES));
        assert_eq!(role.position, 1);
        assert!(role.tags.is_none());
        assert_eq!(
            role.icon.unwrap().to_string(),
            "cf3ced8600b777c9486c6d8d84fb4327"
        );
    }

    #[test]
    fn role_tags_null_flags() {
        let json = r#"{"bot_id":"1","premium_subscriber":null,"guild_connections":null}"#;
        let tags: RoleTags = serde_json::from_str(json).unwrap();
        assert_eq!(tags.bot_id.map(crate::id::Id::get), Some(1));
        assert!(tags.premium_subscriber);
        assert!(tags.guild_connections);
        assert!(!tags.available_for_purchase);

        assert_eq!(serde_json::to_string(&tags).unwrap(), json);
    }

    #[test]
    fn holographic_colors_serialize_like_the_docs() {
        assert_eq!(
            serde_json::to_string(&RoleColors::HOLOGRAPHIC).unwrap(),
            r#"{"primary_color":11127295,"secondary_color":16759788,"tertiary_color":16761760}"#
        );
    }

    #[test]
    fn hot_type_sizes() {
        assert!(
            size_of::<Member>() <= 304,
            "Member is {} bytes",
            size_of::<Member>()
        );
        assert!(
            size_of::<PartialMember>() <= 160,
            "PartialMember is {} bytes",
            size_of::<PartialMember>()
        );
        assert!(
            size_of::<Role>() <= 128,
            "Role is {} bytes",
            size_of::<Role>()
        );
    }
}
