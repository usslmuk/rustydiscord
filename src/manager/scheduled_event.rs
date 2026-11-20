use std::{future::Future, sync::Arc};

use crate::cache::{Backend, GuildView};
use crate::collection::Collection;
use crate::model::{
    guild_scheduled_event::{GuildScheduledEvent, GuildScheduledEventUser},
    id::{GuildId, ScheduledEventId, UserId},
    rest::{
        CreateGuildScheduledEvent, GetGuildScheduledEvent, GetGuildScheduledEventUsers,
        ListScheduledEventsForGuild, ModifyGuildScheduledEvent,
    },
};
use crate::rest::Error;

use super::Fetch;
use crate::Context;

#[derive(Debug)]
pub struct ScheduledEventManager<'a, B: Backend> {
    ctx: &'a Context<B>,
    guild_id: GuildId,
}

impl<B: Backend> Clone for ScheduledEventManager<'_, B> {
    fn clone(&self) -> Self {
        *self
    }
}

impl<B: Backend> Copy for ScheduledEventManager<'_, B> {}

impl<'a, B: Backend> ScheduledEventManager<'a, B> {
    pub(crate) const fn new(ctx: &'a Context<B>, guild_id: GuildId) -> Self {
        Self { ctx, guild_id }
    }

    #[must_use]
    pub const fn guild_id(self) -> GuildId {
        self.guild_id
    }

    #[must_use]
    pub fn cache(self) -> Option<GuildView<'a, ScheduledEventId, GuildScheduledEvent, B>> {
        self.ctx
            .cache()
            .map(|cache| cache.scheduled_events(self.guild_id))
    }

    pub fn fetch(
        self,
        event_id: ScheduledEventId,
    ) -> Fetch<
        Arc<GuildScheduledEvent>,
        impl Future<Output = Result<Arc<GuildScheduledEvent>, Error>> + Send + 'a,
    > {
        let cached = self.ctx.ops().scheduled_event(self.guild_id, event_id);
        let Self { ctx, guild_id } = self;
        Fetch::new(cached, async move {
            let query = GetGuildScheduledEvent {
                with_user_count: Some(true),
                ..Default::default()
            };
            let event = ctx
                .rest()
                .get_guild_scheduled_event(guild_id, event_id, &query)
                .await?;
            Ok(ctx.store_scheduled_event(guild_id, &event))
        })
    }

    pub async fn list(
        self,
    ) -> Result<Collection<ScheduledEventId, Arc<GuildScheduledEvent>>, Error> {
        let query = ListScheduledEventsForGuild {
            with_user_count: Some(true),
            ..Default::default()
        };
        let events = self
            .ctx
            .rest()
            .list_scheduled_events_for_guild(self.guild_id, &query)
            .await?;
        Ok(events
            .iter()
            .map(|event| {
                (
                    event.id,
                    self.ctx.store_scheduled_event(self.guild_id, event),
                )
            })
            .collect())
    }

    pub async fn create(
        self,
        body: &CreateGuildScheduledEvent,
        reason: Option<&str>,
    ) -> Result<Arc<GuildScheduledEvent>, Error> {
        let event = self
            .ctx
            .rest()
            .create_guild_scheduled_event(self.guild_id, body, reason)
            .await?;
        Ok(self.ctx.store_scheduled_event(self.guild_id, &event))
    }

    pub async fn edit(
        self,
        event_id: ScheduledEventId,
        body: &ModifyGuildScheduledEvent,
        reason: Option<&str>,
    ) -> Result<Arc<GuildScheduledEvent>, Error> {
        let event = self
            .ctx
            .rest()
            .modify_guild_scheduled_event(self.guild_id, event_id, body, reason)
            .await?;
        Ok(self.ctx.store_scheduled_event(self.guild_id, &event))
    }

    pub async fn delete(self, event_id: ScheduledEventId) -> Result<(), Error> {
        self.ctx
            .rest()
            .delete_guild_scheduled_event(self.guild_id, event_id)
            .await
    }

    pub async fn users(
        self,
        event_id: ScheduledEventId,
        query: &GetGuildScheduledEventUsers,
    ) -> Result<Collection<UserId, GuildScheduledEventUser>, Error> {
        let users = self
            .ctx
            .rest()
            .get_guild_scheduled_event_users(self.guild_id, event_id, query)
            .await?;
        Ok(users.into_iter().map(|user| (user.user.id, user)).collect())
    }
}
