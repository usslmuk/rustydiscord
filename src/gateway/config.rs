use core::{fmt, time::Duration};
use std::sync::Arc;

use crate::model::gateway::{
    Intents,
    payload::{IdentifyProperties, UpdatePresence},
};
use tokio_websockets::Connector;

use crate::gateway::{Compression, Encoding, IdentifyQueue, LocalIdentifyQueue};

pub const DEFAULT_GATEWAY_URL: &str = "wss://gateway.discord.gg";

#[derive(Clone)]
pub struct Config {
    pub(crate) token: Box<str>,
    pub(crate) intents: Intents,
    pub(crate) gateway_url: Box<str>,
    pub(crate) compression: Compression,
    pub(crate) encoding: Encoding,
    pub(crate) properties: IdentifyProperties,
    pub(crate) large_threshold: Option<u8>,
    pub(crate) connect_timeout: Duration,
    pub(crate) hello_timeout: Duration,
    pub(crate) tls: Arc<Connector>,
    pub(crate) identify_queue: Arc<dyn IdentifyQueue>,
    pub(crate) presence: Option<UpdatePresence>,
}

impl Config {
    #[must_use]
    pub fn new(token: impl Into<String>, intents: Intents) -> Self {
        Self::builder(token, intents).build()
    }

    pub fn builder(token: impl Into<String>, intents: Intents) -> ConfigBuilder {
        ConfigBuilder {
            config: Self {
                token: token.into().into_boxed_str(),
                intents,
                gateway_url: DEFAULT_GATEWAY_URL.into(),
                compression: Compression::default(),
                encoding: Encoding::default(),
                properties: IdentifyProperties::new("rustydiscord"),
                large_threshold: None,
                connect_timeout: Duration::from_secs(30),
                hello_timeout: Duration::from_secs(60),
                tls: Arc::new(crate::gateway::tls::connector()),
                identify_queue: Arc::new(LocalIdentifyQueue::default()),
                presence: None,
            },
        }
    }

    #[must_use]
    pub const fn intents(&self) -> Intents {
        self.intents
    }

    #[must_use]
    pub fn gateway_url(&self) -> &str {
        &self.gateway_url
    }

    #[must_use]
    pub const fn compression(&self) -> Compression {
        self.compression
    }

    #[must_use]
    pub const fn encoding(&self) -> Encoding {
        self.encoding
    }

    #[must_use]
    pub const fn presence(&self) -> Option<&UpdatePresence> {
        self.presence.as_ref()
    }

    #[must_use]
    pub fn identify_queue(&self) -> &Arc<dyn IdentifyQueue> {
        &self.identify_queue
    }
}

impl fmt::Debug for Config {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Config")
            .field("token", &"<redacted>")
            .field("intents", &self.intents)
            .field("gateway_url", &self.gateway_url)
            .field("compression", &self.compression)
            .field("encoding", &self.encoding)
            .field("properties", &self.properties)
            .field("large_threshold", &self.large_threshold)
            .field("connect_timeout", &self.connect_timeout)
            .field("hello_timeout", &self.hello_timeout)
            .field("identify_queue", &self.identify_queue)
            .field("presence", &self.presence)
            .finish_non_exhaustive()
    }
}

#[derive(Debug)]
#[must_use = "a builder does nothing until `build` is called"]
pub struct ConfigBuilder {
    config: Config,
}

impl ConfigBuilder {
    pub fn gateway_url(mut self, url: impl Into<String>) -> Self {
        self.config.gateway_url = url.into().into_boxed_str();
        self
    }

    pub const fn compression(mut self, compression: Compression) -> Self {
        self.config.compression = compression;
        self
    }

    pub const fn encoding(mut self, encoding: Encoding) -> Self {
        self.config.encoding = encoding;
        self
    }

    pub fn identify_properties(mut self, properties: IdentifyProperties) -> Self {
        self.config.properties = properties;
        self
    }

    pub const fn large_threshold(mut self, threshold: u8) -> Self {
        self.config.large_threshold = Some(threshold);
        self
    }

    pub const fn connect_timeout(mut self, timeout: Duration) -> Self {
        self.config.connect_timeout = timeout;
        self
    }

    pub const fn hello_timeout(mut self, timeout: Duration) -> Self {
        self.config.hello_timeout = timeout;
        self
    }

    pub fn presence(mut self, presence: UpdatePresence) -> Self {
        self.config.presence = Some(presence);
        self
    }

    pub fn identify_queue(mut self, queue: Arc<dyn IdentifyQueue>) -> Self {
        self.config.identify_queue = queue;
        self
    }

    #[must_use]
    pub fn build(self) -> Config {
        self.config
    }
}

#[cfg(test)]
mod tests {
    use core::time::Duration;
    use std::sync::Arc;

    use crate::model::gateway::{
        Intents, Status,
        payload::{IdentifyProperties, UpdatePresence},
    };

    use super::Config;
    use crate::gateway::{Compression, LocalIdentifyQueue};

    #[test]
    fn defaults() {
        let config = Config::new("a-token", Intents::GUILDS);
        assert_eq!(config.intents(), Intents::GUILDS);
        assert_eq!(config.gateway_url(), super::DEFAULT_GATEWAY_URL);
        assert_eq!(config.compression(), Compression::default());
        assert!(config.presence().is_none());
        assert_eq!(config.connect_timeout, Duration::from_secs(30));
        assert_eq!(config.hello_timeout, Duration::from_secs(60));
        assert_eq!(config.large_threshold, None);
    }

    #[test]
    fn every_setting_is_kept_and_the_token_is_never_shown() {
        let queue = Arc::new(LocalIdentifyQueue::default());
        let config = Config::builder("a-secret-token", Intents::GUILD_MESSAGES)
            .gateway_url("wss://gateway.example.com")
            .compression(Compression::None)
            .identify_properties(IdentifyProperties::new("my-bot"))
            .large_threshold(100)
            .connect_timeout(Duration::from_secs(5))
            .hello_timeout(Duration::from_secs(7))
            .presence(UpdatePresence::new(Status::Idle, []))
            .identify_queue(queue)
            .build();
        assert_eq!(config.gateway_url(), "wss://gateway.example.com");
        assert_eq!(config.compression(), Compression::None);
        assert_eq!(config.large_threshold, Some(100));
        assert_eq!(config.connect_timeout, Duration::from_secs(5));
        assert_eq!(config.hello_timeout, Duration::from_secs(7));
        assert!(config.presence().is_some());
        let debug = format!("{config:?}");
        assert!(!debug.contains("a-secret-token"), "{debug}");
        assert!(
            debug.contains("<redacted>") && debug.contains("my-bot"),
            "{debug}"
        );
        let builder = format!("{:?}", Config::builder("a-secret-token", Intents::GUILDS));
        assert!(!builder.contains("a-secret-token"), "{builder}");
    }
}
