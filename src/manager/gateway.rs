use core::{
    fmt,
    future::{Future, IntoFuture},
    pin::Pin,
    time::Duration,
};
use std::{
    collections::{HashMap, hash_map::Entry},
    sync::Arc,
};

use crate::cache::{Backend, CachedMember};
use crate::collection::Collection;
use crate::gateway::SendError;
use crate::model::{
    gateway::{
        ShardId,
        payload::{GuildMembersChunk, RequestGuildMembers, RequestSoundboardSounds},
    },
    id::{GuildId, SoundboardSoundId, UserId},
    soundboard::SoundboardSound,
};
use tokio::{sync::mpsc, time::Instant};

use crate::{
    Context,
    cache_ops::CacheOps,
    collector::{EndReason, Key, MemberReply, NONCE_PREFIX, Signal, Sink},
    context::Core,
};

#[derive(Debug, thiserror::Error)]
#[non_exhaustive]
pub enum GatewayRequestError {
    #[error("the shard of guild {0} does not run in this client")]
    ShardNotRunning(GuildId),

    #[error("the request could not be sent")]
    Send(#[from] SendError),

    #[error("the gateway did not answer in time")]
    Timeout,

    #[error("the request was rate limited; retry after {retry_after:?}")]
    RateLimited { retry_after: Duration },

    #[error("the request ended before its answer: {0}")]
    Ended(EndReason),
}

#[derive(Clone, Debug)]
enum Members {
    Query { query: String, limit: u32 },

    Users(Vec<UserId>),
}

#[must_use = "the request is only sent when awaited"]
pub struct FetchMembers<'a, B: Backend> {
    ctx: &'a Context<B>,
    guild_id: GuildId,
    members: Members,
    presences: bool,
    time: Duration,
}

impl<B: Backend> fmt::Debug for FetchMembers<'_, B> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("FetchMembers")
            .field("guild_id", &self.guild_id)
            .field("members", &self.members)
            .field("presences", &self.presences)
            .field("time", &self.time)
            .finish_non_exhaustive()
    }
}

const MEMBERS_TIME: Duration = Duration::from_secs(120);

const MAX_USER_IDS: usize = 100;

impl<'a, B: Backend> FetchMembers<'a, B> {
    pub(crate) fn all(ctx: &'a Context<B>, guild_id: GuildId) -> Self {
        Self::new(
            ctx,
            guild_id,
            Members::Query {
                query: String::new(),
                limit: 0,
            },
        )
    }

    pub(crate) fn query(ctx: &'a Context<B>, guild_id: GuildId, query: String) -> Self {
        Self::new(ctx, guild_id, Members::Query { query, limit: 100 })
    }

    pub(crate) fn users(ctx: &'a Context<B>, guild_id: GuildId, users: Vec<UserId>) -> Self {
        Self::new(ctx, guild_id, Members::Users(users))
    }

    const fn new(ctx: &'a Context<B>, guild_id: GuildId, members: Members) -> Self {
        Self {
            ctx,
            guild_id,
            members,
            presences: false,
            time: MEMBERS_TIME,
        }
    }

    pub const fn limit(mut self, limit: u32) -> Self {
        if let Members::Query { limit: current, .. } = &mut self.members {
            *current = limit;
        }
        self
    }

    pub const fn presences(mut self, presences: bool) -> Self {
        self.presences = presences;
        self
    }

    pub const fn time(mut self, time: Duration) -> Self {
        self.time = time;
        self
    }
}

type MembersFuture<'a> = Pin<
    Box<
        dyn Future<Output = Result<Collection<UserId, Arc<CachedMember>>, GatewayRequestError>>
            + Send
            + 'a,
    >,
>;

pub(crate) type SoundsFuture<'a> =
    Pin<Box<dyn Future<Output = Result<GuildSoundboardSounds, GatewayRequestError>> + Send + 'a>>;

fn fetch_members(
    ctx: &Core,
    guild_id: GuildId,
    members: Members,
    presences: bool,
    time: Duration,
) -> MembersFuture<'_> {
    Box::pin(async move {
        let sender = ctx
            .guild_shard(guild_id)
            .ok_or(GatewayRequestError::ShardNotRunning(guild_id))?;
        let deadline = Instant::now().checked_add(time);
        let requests: Vec<RequestGuildMembers> = match members {
            Members::Query { query, limit } => {
                vec![RequestGuildMembers::query(guild_id, query, limit)]
            }
            Members::Users(users) => users
                .chunks(MAX_USER_IDS)
                .map(|users| RequestGuildMembers::users(guild_id, users.iter().copied()))
                .collect(),
        };
        let mut fetched = Collection::new();
        for mut request in requests {
            let nonce = ctx.collectors().nonce();
            let (tx, mut rx) = mpsc::unbounded_channel();
            let _registration = ctx.collectors().register(
                Key::Nonce(nonce),
                Some(guild_id),
                None,
                Sink::MemberChunks(tx),
            );
            request.presences = presences;
            request.nonce = Some(format!("{NONCE_PREFIX}{nonce}").into());
            sender.command(&request)?;
            let mut received = 0;
            loop {
                let reply = match deadline {
                    Some(deadline) => tokio::time::timeout_at(deadline, rx.recv())
                        .await
                        .map_err(|_| GatewayRequestError::Timeout)?,
                    None => rx.recv().await,
                };
                match reply {
                    Some(Signal::Item(MemberReply::Chunk(chunk))) => {
                        received += 1;
                        add_members(ctx.ops(), &chunk, &mut fetched);
                        if received >= chunk.chunk_count {
                            break;
                        }
                    }
                    Some(Signal::Item(MemberReply::RateLimited(retry_after))) => {
                        return Err(GatewayRequestError::RateLimited { retry_after });
                    }
                    Some(Signal::End(reason)) => return Err(GatewayRequestError::Ended(reason)),
                    None => return Err(GatewayRequestError::Ended(EndReason::Shutdown)),
                }
            }
        }
        Ok(fetched)
    })
}

fn add_members(
    ops: &dyn CacheOps,
    chunk: &GuildMembersChunk,
    fetched: &mut Collection<UserId, Arc<CachedMember>>,
) {
    for member in &chunk.members {
        let stored = ops
            .member(chunk.guild_id, member.user.id)
            .unwrap_or_else(|| ops.store_member(chunk.guild_id, member));
        fetched.insert(member.user.id, stored);
    }
}

impl<'a, B: Backend> IntoFuture for FetchMembers<'a, B> {
    type Output = Result<Collection<UserId, Arc<CachedMember>>, GatewayRequestError>;
    type IntoFuture = MembersFuture<'a>;

    fn into_future(self) -> Self::IntoFuture {
        let Self {
            ctx,
            guild_id,
            members,
            presences,
            time,
        } = self;
        fetch_members(ctx.core(), guild_id, members, presences, time)
    }
}

pub type GuildSoundboardSounds =
    Collection<GuildId, Collection<SoundboardSoundId, Arc<SoundboardSound>>>;

pub(crate) fn fetch_soundboard_sounds(
    ctx: &Core,
    guild_ids: Vec<GuildId>,
    time: Duration,
) -> SoundsFuture<'_> {
    Box::pin(async move {
        let deadline = Instant::now().checked_add(time);
        let (tx, mut rx) = mpsc::unbounded_channel();
        let mut registrations = Vec::with_capacity(guild_ids.len());
        let mut by_shard: HashMap<u32, (crate::gateway::MessageSender, Vec<GuildId>)> =
            HashMap::new();
        let total = ctx.shard_id().total();
        for &guild_id in &guild_ids {
            let number = ShardId::for_guild(guild_id, total).number();
            let (_, guilds) = match by_shard.entry(number) {
                Entry::Occupied(entry) => entry.into_mut(),
                Entry::Vacant(entry) => {
                    let sender = ctx
                        .guild_shard(guild_id)
                        .ok_or(GatewayRequestError::ShardNotRunning(guild_id))?;
                    entry.insert((sender, Vec::new()))
                }
            };
            guilds.push(guild_id);
            registrations.push(ctx.collectors().register(
                Key::Sounds(guild_id),
                Some(guild_id),
                None,
                Sink::SoundboardSounds(tx.clone()),
            ));
        }
        drop(tx);
        for (sender, guilds) in by_shard.into_values() {
            sender.command(&RequestSoundboardSounds::new(guilds))?;
        }
        let mut fetched = GuildSoundboardSounds::new();
        while fetched.len() < guild_ids.len() {
            let reply = match deadline {
                Some(deadline) => tokio::time::timeout_at(deadline, rx.recv())
                    .await
                    .map_err(|_| GatewayRequestError::Timeout)?,
                None => rx.recv().await,
            };
            match reply {
                Some(Signal::Item(sounds)) => {
                    let guild_id = sounds.guild_id;
                    let stored = sounds
                        .soundboard_sounds
                        .iter()
                        .map(|sound| {
                            (
                                sound.sound_id,
                                ctx.ops().store_soundboard_sound(guild_id, sound),
                            )
                        })
                        .collect();
                    fetched.insert(guild_id, stored);
                }
                Some(Signal::End(reason)) => return Err(GatewayRequestError::Ended(reason)),
                None => return Err(GatewayRequestError::Ended(EndReason::Shutdown)),
            }
        }
        drop(registrations);
        Ok(fetched)
    })
}
