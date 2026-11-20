use crate::model::{
    guild_scheduled_event::{GuildScheduledEvent, GuildScheduledEventUser},
    id::{GuildId, ScheduledEventId},
    rest::{
        CreateGuildScheduledEvent, GetGuildScheduledEvent, GetGuildScheduledEventUsers,
        ListScheduledEventsForGuild, ModifyGuildScheduledEvent,
    },
};

use crate::rest::client::boxed;
use crate::rest::{Error, Request, Rest, Route};

impl Rest {
    pub fn list_scheduled_events_for_guild<'a>(
        &'a self,
        guild_id: GuildId,
        query: &'a ListScheduledEventsForGuild,
    ) -> impl Future<Output = Result<Vec<GuildScheduledEvent>, Error>> + Send + 'a {
        boxed(async move {
            self.call(Request::new(Route::ListScheduledEventsForGuild { guild_id }).query(query))
                .await
        })
    }

    pub fn create_guild_scheduled_event<'a>(
        &'a self,
        guild_id: GuildId,
        body: &'a CreateGuildScheduledEvent,
        reason: Option<&'a str>,
    ) -> impl Future<Output = Result<GuildScheduledEvent, Error>> + Send + 'a {
        boxed(async move {
            self.call(
                Request::new(Route::CreateGuildScheduledEvent { guild_id })
                    .json(body)?
                    .reason(reason),
            )
            .await
        })
    }

    pub fn get_guild_scheduled_event<'a>(
        &'a self,
        guild_id: GuildId,
        event_id: ScheduledEventId,
        query: &'a GetGuildScheduledEvent,
    ) -> impl Future<Output = Result<GuildScheduledEvent, Error>> + Send + 'a {
        boxed(async move {
            self.call(
                Request::new(Route::GetGuildScheduledEvent { guild_id, event_id }).query(query),
            )
            .await
        })
    }

    pub fn modify_guild_scheduled_event<'a>(
        &'a self,
        guild_id: GuildId,
        event_id: ScheduledEventId,
        body: &'a ModifyGuildScheduledEvent,
        reason: Option<&'a str>,
    ) -> impl Future<Output = Result<GuildScheduledEvent, Error>> + Send + 'a {
        boxed(async move {
            self.call(
                Request::new(Route::ModifyGuildScheduledEvent { guild_id, event_id })
                    .json(body)?
                    .reason(reason),
            )
            .await
        })
    }

    pub fn delete_guild_scheduled_event(
        &self,
        guild_id: GuildId,
        event_id: ScheduledEventId,
    ) -> impl Future<Output = Result<(), Error>> + Send + '_ {
        boxed(async move {
            self.call(Request::new(Route::DeleteGuildScheduledEvent {
                guild_id,
                event_id,
            }))
            .await
        })
    }

    pub fn get_guild_scheduled_event_users<'a>(
        &'a self,
        guild_id: GuildId,
        event_id: ScheduledEventId,
        query: &'a GetGuildScheduledEventUsers,
    ) -> impl Future<Output = Result<Vec<GuildScheduledEventUser>, Error>> + Send + 'a {
        boxed(async move {
            self.call(
                Request::new(Route::GetGuildScheduledEventUsers { guild_id, event_id })
                    .query(query),
            )
            .await
        })
    }
}
