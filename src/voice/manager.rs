use core::{
    fmt,
    sync::atomic::{AtomicU64, Ordering},
};
use std::{collections::HashMap, sync::Arc};

use crate::model::{
    gateway::{DispatchEvent, payload::UpdateVoiceState},
    id::{ChannelId, GuildId, UserId},
};
use tokio::sync::oneshot;

use crate::voice::{
    Call, JoinError, VoiceGateway,
    call::{CallCommand, Calls, Spawn, lock},
    codec::DEFAULT_BITRATE,
    connection::Config,
};

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[non_exhaustive]
pub struct JoinOptions {
    self_mute: bool,
    self_deaf: bool,
    bitrate: u32,
}

impl Default for JoinOptions {
    fn default() -> Self {
        Self::new()
    }
}

impl JoinOptions {
    #[must_use]
    pub const fn new() -> Self {
        Self {
            self_mute: false,
            self_deaf: true,
            bitrate: DEFAULT_BITRATE,
        }
    }

    #[must_use]
    pub const fn self_mute(mut self, self_mute: bool) -> Self {
        self.self_mute = self_mute;
        self
    }

    #[must_use]
    pub const fn self_deaf(mut self, self_deaf: bool) -> Self {
        self.self_deaf = self_deaf;
        self
    }

    #[must_use]
    pub const fn bitrate(mut self, bits_per_second: u32) -> Self {
        self.bitrate = bits_per_second;
        self
    }
}

#[derive(Clone)]
pub struct VoiceManager {
    inner: Arc<Inner>,
}

struct Inner {
    user_id: UserId,
    shard_count: u32,
    gateway: Arc<dyn VoiceGateway>,
    calls: Arc<Calls>,
    config: Config,
    next_call: AtomicU64,
}

impl fmt::Debug for VoiceManager {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("VoiceManager")
            .field("user_id", &self.inner.user_id)
            .field("shard_count", &self.inner.shard_count)
            .field("calls", &lock(&self.inner.calls).len())
            .finish_non_exhaustive()
    }
}

impl VoiceManager {
    #[must_use]
    pub fn new(bot_user_id: UserId, shard_count: u32, gateway: impl VoiceGateway) -> Self {
        Self::with_config(bot_user_id, shard_count, gateway, Config::default())
    }

    pub(crate) fn with_config(
        bot_user_id: UserId,
        shard_count: u32,
        gateway: impl VoiceGateway,
        config: Config,
    ) -> Self {
        Self {
            inner: Arc::new(Inner {
                user_id: bot_user_id,
                shard_count: shard_count.max(1),
                gateway: Arc::new(gateway),
                calls: Arc::new(Calls::new(HashMap::new())),
                config,
                next_call: AtomicU64::new(0),
            }),
        }
    }

    pub fn process(&self, event: &DispatchEvent) {
        match event {
            DispatchEvent::VoiceStateUpdate(state) => {
                if state.user_id != self.inner.user_id {
                    return;
                }
                if let Some(call) = state.guild_id.and_then(|guild_id| self.get(guild_id)) {
                    call.command(CallCommand::State {
                        session_id: state.session_id.to_string(),
                        channel_id: state.channel_id,
                    });
                }
            }
            DispatchEvent::VoiceServerUpdate(server) => {
                if let Some(call) = self.get(server.guild_id) {
                    call.command(CallCommand::Server {
                        token: server.token.to_string(),
                        endpoint: server.endpoint.as_ref().map(ToString::to_string),
                    });
                }
            }
            _ => {}
        }
    }

    pub async fn join(&self, guild_id: GuildId, channel_id: ChannelId) -> Result<Call, JoinError> {
        self.join_with(guild_id, channel_id, JoinOptions::default())
            .await
    }

    pub async fn join_with(
        &self,
        guild_id: GuildId,
        channel_id: ChannelId,
        options: JoinOptions,
    ) -> Result<Call, JoinError> {
        let mut attempts = 0;
        loop {
            attempts += 1;
            let call = self.call(guild_id);
            let (done, result) = oneshot::channel();
            let bitrate = options.bitrate;
            if !call.command(CallCommand::Join {
                channel_id,
                bitrate,
                done,
            }) {
                self.forget(&call);
                continue;
            }
            let command = UpdateVoiceState::new(
                guild_id,
                Some(channel_id),
                options.self_mute,
                options.self_deaf,
            );
            if let Err(error) = self.inner.gateway.send(self.shard_id(guild_id), &command) {
                drop(result);
                call.command(CallCommand::Abandoned);
                return Err(JoinError::Gateway(error));
            }
            match result.await {
                Ok(Ok(())) => return Ok(call),
                Ok(Err(error)) => return Err(error),
                Err(_) if attempts < 2 => self.forget(&call),
                Err(_) => return Err(JoinError::Cancelled),
            }
        }
    }

    #[must_use]
    pub fn get(&self, guild_id: GuildId) -> Option<Call> {
        lock(&self.inner.calls).get(&guild_id).cloned()
    }

    #[must_use]
    pub fn calls(&self) -> Vec<Call> {
        lock(&self.inner.calls).values().cloned().collect()
    }

    fn call(&self, guild_id: GuildId) -> Call {
        lock(&self.inner.calls)
            .entry(guild_id)
            .or_insert_with(|| {
                Call::spawn(Spawn {
                    id: self.inner.next_call.fetch_add(1, Ordering::Relaxed),
                    guild_id,
                    user_id: self.inner.user_id,
                    shard_id: self.shard_id(guild_id),
                    gateway: Arc::clone(&self.inner.gateway),
                    config: self.inner.config.clone(),
                    calls: Arc::downgrade(&self.inner.calls),
                })
            })
            .clone()
    }

    fn forget(&self, call: &Call) {
        let mut calls = lock(&self.inner.calls);
        if calls
            .get(&call.guild_id())
            .is_some_and(|current| current.inner.id == call.inner.id)
        {
            calls.remove(&call.guild_id());
        }
    }

    fn shard_id(&self, guild_id: GuildId) -> u32 {
        let shard = (guild_id.get() >> 22) % u64::from(self.inner.shard_count);

        u32::try_from(shard).unwrap_or_default()
    }
}

#[cfg(test)]
mod player_tests;
#[cfg(test)]
mod receive_tests;
#[cfg(test)]
mod stress_tests;
#[cfg(test)]
mod tests;
