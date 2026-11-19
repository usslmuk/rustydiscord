use rustydiscord_model::{
    auto_moderation::Rule,
    id::{AutoModerationRuleId, GuildId},
    rest::{CreateAutoModerationRule, ModifyAutoModerationRule},
};

use crate::client::boxed;
use crate::{Error, Request, Rest, Route};

impl Rest {
    pub fn list_auto_moderation_rules_for_guild(
        &self,
        guild_id: GuildId,
    ) -> impl Future<Output = Result<Vec<Rule>, Error>> + Send + '_ {
        boxed(async move {
            self.call(Request::new(Route::ListAutoModerationRulesForGuild {
                guild_id,
            }))
            .await
        })
    }

    pub fn get_auto_moderation_rule(
        &self,
        guild_id: GuildId,
        rule_id: AutoModerationRuleId,
    ) -> impl Future<Output = Result<Rule, Error>> + Send + '_ {
        boxed(async move {
            self.call(Request::new(Route::GetAutoModerationRule {
                guild_id,
                rule_id,
            }))
            .await
        })
    }

    pub fn create_auto_moderation_rule<'a>(
        &'a self,
        guild_id: GuildId,
        body: &'a CreateAutoModerationRule,
        reason: Option<&'a str>,
    ) -> impl Future<Output = Result<Rule, Error>> + Send + 'a {
        boxed(async move {
            self.call(
                Request::new(Route::CreateAutoModerationRule { guild_id })
                    .json(body)?
                    .reason(reason),
            )
            .await
        })
    }

    pub fn modify_auto_moderation_rule<'a>(
        &'a self,
        guild_id: GuildId,
        rule_id: AutoModerationRuleId,
        body: &'a ModifyAutoModerationRule,
        reason: Option<&'a str>,
    ) -> impl Future<Output = Result<Rule, Error>> + Send + 'a {
        boxed(async move {
            self.call(
                Request::new(Route::ModifyAutoModerationRule { guild_id, rule_id })
                    .json(body)?
                    .reason(reason),
            )
            .await
        })
    }

    pub fn delete_auto_moderation_rule<'a>(
        &'a self,
        guild_id: GuildId,
        rule_id: AutoModerationRuleId,
        reason: Option<&'a str>,
    ) -> impl Future<Output = Result<(), Error>> + Send + 'a {
        boxed(async move {
            self.call(
                Request::new(Route::DeleteAutoModerationRule { guild_id, rule_id }).reason(reason),
            )
            .await
        })
    }
}
