use crate::model::{
    guild::Guild,
    guild_template::GuildTemplate,
    id::GuildId,
    rest::{CreateGuildFromTemplate, CreateGuildTemplate, ModifyGuildTemplate},
};

use crate::rest::client::boxed;
use crate::rest::{Error, Request, Rest, Route};

impl Rest {
    pub fn get_guild_template<'a>(
        &'a self,
        template_code: &'a str,
    ) -> impl Future<Output = Result<GuildTemplate, Error>> + Send + 'a {
        boxed(async move {
            self.call(Request::new(Route::GetGuildTemplate { template_code }))
                .await
        })
    }

    pub fn get_guild_templates(
        &self,
        guild_id: GuildId,
    ) -> impl Future<Output = Result<Vec<GuildTemplate>, Error>> + Send + '_ {
        boxed(async move {
            self.call(Request::new(Route::GetGuildTemplates { guild_id }))
                .await
        })
    }

    pub fn create_guild_template<'a>(
        &'a self,
        guild_id: GuildId,
        body: &'a CreateGuildTemplate,
    ) -> impl Future<Output = Result<GuildTemplate, Error>> + Send + 'a {
        boxed(async move {
            self.call(Request::new(Route::CreateGuildTemplate { guild_id }).json(body)?)
                .await
        })
    }

    pub fn create_guild_from_template<'a>(
        &'a self,
        template_code: &'a str,
        body: &'a CreateGuildFromTemplate,
    ) -> impl Future<Output = Result<Guild, Error>> + Send + 'a {
        boxed(async move {
            self.call(Request::new(Route::CreateGuildFromTemplate { template_code }).json(body)?)
                .await
        })
    }

    pub fn sync_guild_template<'a>(
        &'a self,
        guild_id: GuildId,
        template_code: &'a str,
    ) -> impl Future<Output = Result<GuildTemplate, Error>> + Send + 'a {
        boxed(async move {
            self.call(Request::new(Route::SyncGuildTemplate {
                guild_id,
                template_code,
            }))
            .await
        })
    }

    pub fn modify_guild_template<'a>(
        &'a self,
        guild_id: GuildId,
        template_code: &'a str,
        body: &'a ModifyGuildTemplate,
    ) -> impl Future<Output = Result<GuildTemplate, Error>> + Send + 'a {
        boxed(async move {
            self.call(
                Request::new(Route::ModifyGuildTemplate {
                    guild_id,
                    template_code,
                })
                .json(body)?,
            )
            .await
        })
    }

    pub fn delete_guild_template<'a>(
        &'a self,
        guild_id: GuildId,
        template_code: &'a str,
    ) -> impl Future<Output = Result<GuildTemplate, Error>> + Send + 'a {
        boxed(async move {
            self.call(Request::new(Route::DeleteGuildTemplate {
                guild_id,
                template_code,
            }))
            .await
        })
    }
}
