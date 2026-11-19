use std::future::Future;

use rustydiscord_cache::Backend;
use rustydiscord_collection::Collection;
use rustydiscord_model::{
    auto_moderation::Rule,
    id::{AutoModerationRuleId, GuildId},
    rest::{CreateAutoModerationRule, ModifyAutoModerationRule},
};
use rustydiscord_rest::Error;

use super::Fetch;
use crate::Context;

#[derive(Debug)]
pub struct AutoModerationRuleManager<'a, B: Backend> {
    ctx: &'a Context<B>,
    guild_id: GuildId,
}

impl<B: Backend> Clone for AutoModerationRuleManager<'_, B> {
    fn clone(&self) -> Self {
        *self
    }
}

impl<B: Backend> Copy for AutoModerationRuleManager<'_, B> {}

impl<'a, B: Backend> AutoModerationRuleManager<'a, B> {
    pub(crate) const fn new(ctx: &'a Context<B>, guild_id: GuildId) -> Self {
        Self { ctx, guild_id }
    }

    #[must_use]
    pub const fn guild_id(self) -> GuildId {
        self.guild_id
    }

    pub fn fetch(
        self,
        rule_id: AutoModerationRuleId,
    ) -> Fetch<Rule, impl Future<Output = Result<Rule, Error>> + Send + 'a> {
        let Self { ctx, guild_id } = self;
        Fetch::new(None, async move {
            ctx.rest().get_auto_moderation_rule(guild_id, rule_id).await
        })
    }

    pub async fn list(self) -> Result<Collection<AutoModerationRuleId, Rule>, Error> {
        let rules = self
            .ctx
            .rest()
            .list_auto_moderation_rules_for_guild(self.guild_id)
            .await?;
        Ok(rules.into_iter().map(|rule| (rule.id, rule)).collect())
    }

    pub async fn create(
        self,
        body: &CreateAutoModerationRule,
        reason: Option<&str>,
    ) -> Result<Rule, Error> {
        self.ctx
            .rest()
            .create_auto_moderation_rule(self.guild_id, body, reason)
            .await
    }

    pub async fn edit(
        self,
        rule_id: AutoModerationRuleId,
        body: &ModifyAutoModerationRule,
        reason: Option<&str>,
    ) -> Result<Rule, Error> {
        self.ctx
            .rest()
            .modify_auto_moderation_rule(self.guild_id, rule_id, body, reason)
            .await
    }

    pub async fn delete(
        self,
        rule_id: AutoModerationRuleId,
        reason: Option<&str>,
    ) -> Result<(), Error> {
        self.ctx
            .rest()
            .delete_auto_moderation_rule(self.guild_id, rule_id, reason)
            .await
    }
}
