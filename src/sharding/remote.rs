use std::sync::Arc;

use std::time::{Duration, Instant};

#[derive(Clone, Debug)]
pub struct TlsConfig {
    pub server_name: String,
}

impl TlsConfig {
    #[must_use]
    pub fn new(server_name: impl Into<String>) -> Self {
        Self {
            server_name: server_name.into(),
        }
    }
}

#[derive(Debug)]
pub struct CrossHostEndpoint {
    pub host: String,

    pub port: u16,

    pub tls: Option<TlsConfig>,
}

impl CrossHostEndpoint {
    #[must_use]
    pub fn new(host: impl Into<String>, port: u16) -> Self {
        Self {
            host: host.into(),
            port,
            tls: None,
        }
    }

    #[must_use]
    pub fn with_tls(mut self, tls: TlsConfig) -> Self {
        self.tls = Some(tls);

        self
    }

    #[must_use]
    pub fn addr(&self) -> String {
        format!("{}:{}", self.host, self.port)
    }

    #[must_use]
    pub fn uses_tls(&self) -> bool {
        self.tls.is_some()
    }
}

#[derive(Debug, Default)]
pub struct RedisIdentifyQueue {
    url: String,

    interval: Duration,
}

impl RedisIdentifyQueue {
    #[must_use]
    pub fn new() -> Self {
        Self {
            url: "redis://127.0.0.1/".to_string(),
            interval: Duration::from_millis(5_100),
        }
    }

    #[must_use]
    pub fn with_url(url: impl Into<String>) -> Self {
        Self {
            url: url.into(),
            interval: Duration::from_millis(5_100),
        }
    }

    #[must_use]
    pub fn url(&self) -> &str {
        &self.url
    }

    #[must_use]
    pub fn interval(&self) -> Duration {
        self.interval
    }

    #[must_use]
    pub fn bucket_key(shard: u32, max_concurrency: u32) -> u32 {
        shard % max_concurrency.max(1)
    }
}

#[derive(Clone, Debug)]
pub struct EvalLimits {
    pub max_input_bytes: usize,

    pub timeout: Duration,

    pub allowlist_only: bool,
}

impl Default for EvalLimits {
    fn default() -> Self {
        Self {
            max_input_bytes: 1_048_576,
            timeout: Duration::from_secs(5),
            allowlist_only: true,
        }
    }
}

#[derive(Clone, Debug)]
pub struct EvalRequest {
    pub command: String,

    pub input: Vec<u8>,

    pub limits: EvalLimits,
}

impl EvalRequest {
    #[must_use]
    pub fn new(command: impl Into<String>, input: Vec<u8>) -> Self {
        Self {
            command: command.into(),
            input,
            limits: EvalLimits::default(),
        }
    }

    pub fn validate(&self, allowlist: &[&str]) -> Result<(), EvalError> {
        if self.input.len() > self.limits.max_input_bytes {
            return Err(EvalError::TooLarge);
        }

        if self.command.is_empty() || self.command.len() > 128 {
            return Err(EvalError::UnknownCommand);
        }

        if self.limits.allowlist_only && !allowlist.contains(&self.command.as_str()) {
            return Err(EvalError::NotAllowlisted);
        }

        Ok(())
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EvalError {
    TooLarge,

    UnknownCommand,

    NotAllowlisted,

    Timeout,
}

impl std::fmt::Display for EvalError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::TooLarge => write!(f, "eval input too large"),

            Self::UnknownCommand => write!(f, "unknown eval command"),

            Self::NotAllowlisted => write!(f, "eval command not allowlisted"),

            Self::Timeout => write!(f, "eval timed out"),
        }
    }
}

impl std::error::Error for EvalError {}

#[must_use]
pub fn split_range(total: u32, parts: u32) -> Vec<std::ops::Range<u32>> {
    if parts == 0 || total == 0 {
        return vec![];
    }

    let per = total.div_ceil(parts);

    (0..parts)
        .map(|i| {
            let s = i * per;

            let e = (s + per).min(total);

            s..e
        })
        .filter(|r| !r.is_empty())
        .collect()
}

#[must_use]
pub fn reshard_plan(old_total: u32, new_total: u32) -> Vec<(u32, u32)> {
    (0..new_total)
        .map(|s| (s, new_total))
        .collect::<Vec<_>>()
        .into_iter()
        .filter(|_| old_total != new_total)
        .collect()
}

#[derive(Debug, Default)]
pub struct ReshardState {
    pub old_total: u32,

    pub new_total: u32,

    pub started: Option<Instant>,
}

impl ReshardState {
    #[must_use]
    pub fn new(old_total: u32, new_total: u32) -> Self {
        Self {
            old_total,
            new_total,
            started: Some(Instant::now()),
        }
    }

    #[must_use]
    pub fn moves_shard(&self, shard: u32) -> bool {
        shard < self.new_total
    }
}

pub type Allowlist = Arc<Vec<String>>;
