use rustydiscord_model::{
    channel::Channel,
    guild::Member,
    id::{ApplicationId, GuildId, UserId},
    rest::{CreateDm, CreateGroupDm, CurrentUserGuild, GetCurrentUserGuilds, ModifyCurrentUser},
    user::{ApplicationRoleConnection, Connection, CurrentUser, User},
};

use crate::client::boxed;
use crate::{Error, Request, Rest, Route};

impl Rest {
    pub fn get_current_user(&self) -> impl Future<Output = Result<CurrentUser, Error>> + Send + '_ {
        boxed(async move { self.call(Request::new(Route::GetCurrentUser)).await })
    }

    pub fn get_user(
        &self,
        user_id: UserId,
    ) -> impl Future<Output = Result<User, Error>> + Send + '_ {
        boxed(async move { self.call(Request::new(Route::GetUser { user_id })).await })
    }

    pub fn modify_current_user<'a>(
        &'a self,
        body: &'a ModifyCurrentUser,
    ) -> impl Future<Output = Result<CurrentUser, Error>> + Send + 'a {
        boxed(async move {
            self.call(Request::new(Route::ModifyCurrentUser).json(body)?)
                .await
        })
    }

    pub fn get_current_user_guilds<'a>(
        &'a self,
        query: &'a GetCurrentUserGuilds,
    ) -> impl Future<Output = Result<Vec<CurrentUserGuild>, Error>> + Send + 'a {
        boxed(async move {
            self.call(Request::new(Route::GetCurrentUserGuilds).query(query))
                .await
        })
    }

    pub fn get_current_user_guild_member(
        &self,
        guild_id: GuildId,
    ) -> impl Future<Output = Result<Member, Error>> + Send + '_ {
        boxed(async move {
            self.call(Request::new(Route::GetCurrentUserGuildMember { guild_id }))
                .await
        })
    }

    pub fn leave_guild(
        &self,
        guild_id: GuildId,
    ) -> impl Future<Output = Result<(), Error>> + Send + '_ {
        boxed(async move {
            self.call(Request::new(Route::LeaveGuild { guild_id }))
                .await
        })
    }

    pub fn create_dm<'a>(
        &'a self,
        body: &'a CreateDm,
    ) -> impl Future<Output = Result<Channel, Error>> + Send + 'a {
        boxed(async move { self.call(Request::new(Route::CreateDm).json(body)?).await })
    }

    pub fn create_group_dm<'a>(
        &'a self,
        body: &'a CreateGroupDm,
    ) -> impl Future<Output = Result<Channel, Error>> + Send + 'a {
        boxed(async move {
            self.call(Request::new(Route::CreateGroupDm).json(body)?)
                .await
        })
    }

    pub fn get_current_user_connections(
        &self,
    ) -> impl Future<Output = Result<Vec<Connection>, Error>> + Send + '_ {
        boxed(async move {
            self.call(Request::new(Route::GetCurrentUserConnections))
                .await
        })
    }

    pub fn get_current_user_application_role_connection(
        &self,
        application_id: ApplicationId,
    ) -> impl Future<Output = Result<ApplicationRoleConnection, Error>> + Send + '_ {
        boxed(async move {
            self.call(Request::new(
                Route::GetCurrentUserApplicationRoleConnection { application_id },
            ))
            .await
        })
    }

    pub fn update_current_user_application_role_connection<'a>(
        &'a self,
        application_id: ApplicationId,
        body: &'a ApplicationRoleConnection,
    ) -> impl Future<Output = Result<ApplicationRoleConnection, Error>> + Send + 'a {
        boxed(async move {
            self.call(
                Request::new(Route::UpdateCurrentUserApplicationRoleConnection { application_id })
                    .json(body)?,
            )
            .await
        })
    }

    pub fn delete_current_user_application_role_connection(
        &self,
        application_id: ApplicationId,
    ) -> impl Future<Output = Result<(), Error>> + Send + '_ {
        boxed(async move {
            self.request_ignoring_body(Request::new(
                Route::DeleteCurrentUserApplicationRoleConnection { application_id },
            ))
            .await
        })
    }
}
