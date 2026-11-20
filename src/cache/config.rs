use core::{fmt, time::Duration};

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[non_exhaustive]
pub enum Resource {
    Guild,

    Channel,

    Thread,

    Role,

    Emoji,

    Sticker,

    Member,

    User,

    Message,

    Presence,

    VoiceState,

    ScheduledEvent,

    StageInstance,

    SoundboardSound,
}

impl Resource {
    pub const ALL: &'static [Self] = &[
        Self::Guild,
        Self::Channel,
        Self::Thread,
        Self::Role,
        Self::Emoji,
        Self::Sticker,
        Self::Member,
        Self::User,
        Self::Message,
        Self::Presence,
        Self::VoiceState,
        Self::ScheduledEvent,
        Self::StageInstance,
        Self::SoundboardSound,
    ];

    const COUNT: usize = Self::ALL.len();

    pub(crate) const fn scope(self) -> Scope {
        match self {
            Self::Guild | Self::Channel | Self::User => Scope::Cache,
            Self::Message => Scope::Channel,
            Self::Thread
            | Self::Role
            | Self::Emoji
            | Self::Sticker
            | Self::Member
            | Self::Presence
            | Self::VoiceState
            | Self::ScheduledEvent
            | Self::StageInstance
            | Self::SoundboardSound => Scope::Guild,
        }
    }

    const fn index(self) -> usize {
        self as usize
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum Scope {
    Cache,

    Guild,

    Channel,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[non_exhaustive]
pub enum Limit {
    Disabled,

    Max(usize),

    Unlimited,
}

impl Limit {
    #[must_use]
    pub const fn is_enabled(self) -> bool {
        !matches!(self, Self::Disabled | Self::Max(0))
    }

    #[must_use]
    pub const fn max(self) -> Option<usize> {
        match self {
            Self::Max(0) | Self::Disabled | Self::Unlimited => None,
            Self::Max(max) => Some(max),
        }
    }

    const fn normalized(self) -> Self {
        match self {
            Self::Max(0) => Self::Disabled,
            other => other,
        }
    }
}

#[derive(Clone, PartialEq, Eq, Hash)]
pub struct CacheConfig {
    limits: [Limit; Resource::COUNT],
    sweepers: SweepConfig,
}

impl CacheConfig {
    pub fn builder() -> CacheConfigBuilder {
        CacheConfigBuilder {
            config: Self::default(),
        }
    }

    #[must_use]
    pub fn large() -> Self {
        let mut config = Self::uniform(Limit::Disabled);
        for resource in [Resource::Guild, Resource::Channel, Resource::Role] {
            config.set(resource, Limit::Unlimited);
        }
        config.set(Resource::Message, Limit::Max(50));
        config.sweepers = SweepConfig {
            messages: Some(Sweep::new(
                Duration::from_secs(600),
                Duration::from_mins(30),
            )),
            threads: Some(Sweep::new(
                Duration::from_secs(3_600),
                Duration::from_secs(14_400),
            )),
        };
        config
    }

    #[must_use]
    pub fn rustydiscord() -> Self {
        let mut config = Self::uniform(Limit::Unlimited);
        config.set(Resource::Message, Limit::Max(200));
        config
    }

    #[must_use]
    pub const fn limit(&self, resource: Resource) -> Limit {
        self.limits[resource.index()]
    }

    #[must_use]
    pub const fn is_enabled(&self, resource: Resource) -> bool {
        self.limit(resource).is_enabled()
    }

    #[must_use]
    pub const fn sweepers(&self) -> &SweepConfig {
        &self.sweepers
    }

    const fn uniform(limit: Limit) -> Self {
        Self {
            limits: [limit; Resource::COUNT],
            sweepers: SweepConfig::DEFAULT,
        }
    }

    const fn set(&mut self, resource: Resource, limit: Limit) {
        self.limits[resource.index()] = limit.normalized();
    }
}

impl Default for CacheConfig {
    fn default() -> Self {
        let mut config = Self::uniform(Limit::Disabled);
        for resource in [Resource::Guild, Resource::Channel, Resource::Role] {
            config.set(resource, Limit::Unlimited);
        }
        config
    }
}

impl fmt::Debug for CacheConfig {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        struct Limits<'a>(&'a CacheConfig);

        impl fmt::Debug for Limits<'_> {
            fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                f.debug_map()
                    .entries(
                        Resource::ALL
                            .iter()
                            .map(|&resource| (resource, self.0.limit(resource))),
                    )
                    .finish()
            }
        }

        f.debug_struct("CacheConfig")
            .field("limits", &Limits(self))
            .field("sweepers", &self.sweepers)
            .finish()
    }
}

#[derive(Clone, Debug)]
#[must_use = "a builder does nothing until `build` is called"]
pub struct CacheConfigBuilder {
    config: CacheConfig,
}

impl CacheConfigBuilder {
    pub const fn limit(mut self, resource: Resource, limit: Limit) -> Self {
        self.config.set(resource, limit);
        self
    }

    pub const fn guilds(self, limit: Limit) -> Self {
        self.limit(Resource::Guild, limit)
    }

    pub const fn channels(self, limit: Limit) -> Self {
        self.limit(Resource::Channel, limit)
    }

    pub const fn threads(self, limit: Limit) -> Self {
        self.limit(Resource::Thread, limit)
    }

    pub const fn roles(self, limit: Limit) -> Self {
        self.limit(Resource::Role, limit)
    }

    pub const fn emojis(self, limit: Limit) -> Self {
        self.limit(Resource::Emoji, limit)
    }

    pub const fn stickers(self, limit: Limit) -> Self {
        self.limit(Resource::Sticker, limit)
    }

    pub const fn members(self, limit: Limit) -> Self {
        self.limit(Resource::Member, limit)
    }

    pub const fn users(self, limit: Limit) -> Self {
        self.limit(Resource::User, limit)
    }

    pub const fn messages(self, limit: Limit) -> Self {
        self.limit(Resource::Message, limit)
    }

    pub const fn presences(self, limit: Limit) -> Self {
        self.limit(Resource::Presence, limit)
    }

    pub const fn voice_states(self, limit: Limit) -> Self {
        self.limit(Resource::VoiceState, limit)
    }

    pub const fn scheduled_events(self, limit: Limit) -> Self {
        self.limit(Resource::ScheduledEvent, limit)
    }

    pub const fn stage_instances(self, limit: Limit) -> Self {
        self.limit(Resource::StageInstance, limit)
    }

    pub const fn soundboard_sounds(self, limit: Limit) -> Self {
        self.limit(Resource::SoundboardSound, limit)
    }

    pub const fn sweep_messages(mut self, sweep: Option<Sweep>) -> Self {
        self.config.sweepers.messages = sweep;
        self
    }

    pub const fn sweep_threads(mut self, sweep: Option<Sweep>) -> Self {
        self.config.sweepers.threads = sweep;
        self
    }

    #[must_use]
    pub fn build(self) -> CacheConfig {
        self.config
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[non_exhaustive]
pub struct Sweep {
    pub interval: Duration,

    pub lifetime: Duration,
}

impl Sweep {
    #[must_use]
    pub const fn new(interval: Duration, lifetime: Duration) -> Self {
        Self { interval, lifetime }
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
#[non_exhaustive]
pub struct SweepConfig {
    pub messages: Option<Sweep>,

    pub threads: Option<Sweep>,
}

impl SweepConfig {
    const DEFAULT: Self = Self {
        messages: None,
        threads: Some(Sweep::new(Duration::from_hours(1), Duration::from_hours(4))),
    };
}

impl Default for SweepConfig {
    fn default() -> Self {
        Self::DEFAULT
    }
}

impl From<CacheConfig> for CacheConfigBuilder {
    fn from(config: CacheConfig) -> Self {
        Self { config }
    }
}

#[cfg(test)]
mod tests {
    use super::{CacheConfig, Limit, Resource, Scope};

    #[test]
    fn resource_indexes_match_all() {
        for (position, resource) in Resource::ALL.iter().enumerate() {
            assert_eq!(resource.index(), position, "{resource:?}");
        }
    }

    #[test]
    fn the_default_is_minimal() {
        let config = CacheConfig::default();
        for &resource in Resource::ALL {
            let expected = matches!(
                resource,
                Resource::Guild | Resource::Channel | Resource::Role
            );
            assert_eq!(config.is_enabled(resource), expected, "{resource:?}");
        }
        assert_eq!(CacheConfig::builder().build(), config);
    }

    #[test]
    fn the_rustydiscord_preset_limits_only_messages() {
        let config = CacheConfig::rustydiscord();
        for &resource in Resource::ALL {
            let expected = if resource == Resource::Message {
                Limit::Max(200)
            } else {
                Limit::Unlimited
            };
            assert_eq!(config.limit(resource), expected, "{resource:?}");
        }
    }

    #[test]
    fn a_zero_maximum_is_stored_as_disabled() {
        let config = CacheConfig::builder().guilds(Limit::Max(0)).build();
        assert_eq!(config.limit(Resource::Guild), Limit::Disabled);
        assert_eq!(
            config,
            CacheConfig::builder().guilds(Limit::Disabled).build()
        );
    }

    #[test]
    fn every_builder_method_sets_its_resource() {
        let config = CacheConfig::builder()
            .guilds(Limit::Max(1))
            .channels(Limit::Max(2))
            .threads(Limit::Max(3))
            .roles(Limit::Max(4))
            .emojis(Limit::Max(5))
            .stickers(Limit::Max(6))
            .members(Limit::Max(7))
            .users(Limit::Max(8))
            .messages(Limit::Max(9))
            .presences(Limit::Max(10))
            .voice_states(Limit::Max(11))
            .scheduled_events(Limit::Max(12))
            .stage_instances(Limit::Max(13))
            .soundboard_sounds(Limit::Max(14))
            .build();
        for (position, &resource) in Resource::ALL.iter().enumerate() {
            assert_eq!(
                config.limit(resource),
                Limit::Max(position + 1),
                "{resource:?}"
            );
        }
    }

    #[test]
    fn scopes_follow_the_rustydiscord_managers() {
        assert_eq!(Resource::Guild.scope(), Scope::Cache);
        assert_eq!(Resource::User.scope(), Scope::Cache);
        assert_eq!(Resource::Channel.scope(), Scope::Cache);
        assert_eq!(Resource::Message.scope(), Scope::Channel);
        assert_eq!(Resource::Member.scope(), Scope::Guild);
        assert_eq!(Resource::Presence.scope(), Scope::Guild);
    }

    #[test]
    fn debug_lists_every_resource() {
        let debug = format!("{:?}", CacheConfig::default());
        assert!(
            debug.starts_with(
                "CacheConfig { limits: {Guild: Unlimited, Channel: Unlimited, Thread: Disabled"
            ),
            "{debug}"
        );
        assert!(debug.contains("SoundboardSound: Disabled}, sweepers: SweepConfig {"));
    }

    #[test]
    fn sweeps_default_to_rustydiscord() {
        use core::time::Duration;

        use super::{Sweep, SweepConfig};

        let sweepers = CacheConfig::default().sweepers().clone();
        assert_eq!(sweepers, SweepConfig::default());
        assert_eq!(sweepers.messages, None);
        assert_eq!(
            sweepers.threads,
            Some(Sweep::new(
                Duration::from_secs(3_600),
                Duration::from_secs(14_400)
            ))
        );
        assert_eq!(CacheConfig::rustydiscord().sweepers(), &sweepers);
        let off = CacheConfig::builder().sweep_threads(None).build();
        assert_eq!(off.sweepers().threads, None);
    }
}
