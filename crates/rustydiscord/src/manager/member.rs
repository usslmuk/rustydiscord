use std::{future::Future, sync::Arc};

use rustydiscord_cache::{Backend, CachedMember, GuildView};
use rustydiscord_collection::Collection;
use rustydiscord_model::{
    id::{GuildId, RoleId, UserId},
    rest::{
        BeginGuildPrune, CreateGuildBan, GetGuildPruneCount, ListGuildMembers, ModifyCurrentMember,
        ModifyGuildMember, SearchGuildMembers,
    },
    util::{Patch, Timestamp},
};
use rustydiscord_rest::Error;

use super::Fetch;
#[cfg(feature = "gateway")]
use super::FetchMembers;
use crate::Context;

#[derive(Debug)]
pub struct MemberManager<'a, B: Backend> {
    ctx: &'a Context<B>,
    guild_id: GuildId,
}

impl<B: Backend> Clone for MemberManager<'_, B> {
    fn clone(&self) -> Self {
        *self
    }
}

impl<B: Backend> Copy for MemberManager<'_, B> {}

impl<'a, B: Backend> MemberManager<'a, B> {
    pub(crate) const fn new(ctx: &'a Context<B>, guild_id: GuildId) -> Self {
        Self { ctx, guild_id }
    }

    #[must_use]
    pub const fn guild_id(self) -> GuildId {
        self.guild_id
    }

    #[must_use]
    pub fn cache(self) -> Option<GuildView<'a, UserId, CachedMember, B>> {
        self.ctx.cache().map(|cache| cache.members(self.guild_id))
    }

    pub fn fetch(
        self,
        user_id: UserId,
    ) -> Fetch<Arc<CachedMember>, impl Future<Output = Result<Arc<CachedMember>, Error>> + Send + 'a>
    {
        let cached = self.ctx.ops().member(self.guild_id, user_id);
        let Self { ctx, guild_id } = self;
        Fetch::new(cached, async move {
            let member = ctx.rest().get_guild_member(guild_id, user_id).await?;
            Ok(ctx.store_member(guild_id, &member))
        })
    }

    #[cfg(feature = "gateway")]
    #[cfg_attr(docsrs, doc(cfg(feature = "gateway")))]
    pub fn fetch_all(self) -> FetchMembers<'a, B> {
        FetchMembers::all(self.ctx, self.guild_id)
    }

    #[cfg(feature = "gateway")]
    #[cfg_attr(docsrs, doc(cfg(feature = "gateway")))]
    pub fn fetch_by_query(self, query: impl Into<String>) -> FetchMembers<'a, B> {
        FetchMembers::query(self.ctx, self.guild_id, query.into())
    }

    #[cfg(feature = "gateway")]
    #[cfg_attr(docsrs, doc(cfg(feature = "gateway")))]
    pub fn fetch_many(self, user_ids: impl IntoIterator<Item = UserId>) -> FetchMembers<'a, B> {
        FetchMembers::users(self.ctx, self.guild_id, user_ids.into_iter().collect())
    }

    pub async fn list(
        self,
        query: &ListGuildMembers,
    ) -> Result<Collection<UserId, Arc<CachedMember>>, Error> {
        let members = self
            .ctx
            .rest()
            .list_guild_members(self.guild_id, query)
            .await?;
        Ok(members
            .iter()
            .map(|member| {
                let member = self.ctx.store_member(self.guild_id, member);
                (member.user.id, member)
            })
            .collect())
    }

    pub async fn edit(
        self,
        user_id: UserId,
        body: &ModifyGuildMember,
        reason: Option<&str>,
    ) -> Result<Arc<CachedMember>, Error> {
        let member = self
            .ctx
            .rest()
            .modify_guild_member(self.guild_id, user_id, body, reason)
            .await?;
        Ok(self.ctx.store_member(self.guild_id, &member))
    }

    pub async fn timeout(
        self,
        user_id: UserId,
        until: Option<Timestamp>,
        reason: Option<&str>,
    ) -> Result<Arc<CachedMember>, Error> {
        let mut body = ModifyGuildMember::default();
        body.communication_disabled_until = until.into();
        self.edit(user_id, &body, reason).await
    }

    pub async fn set_nickname(
        self,
        user_id: UserId,
        nickname: Option<&str>,
        reason: Option<&str>,
    ) -> Result<Arc<CachedMember>, Error> {
        let mut body = ModifyGuildMember::default();
        body.nick = nickname.map_or(Patch::Clear, |nick| Patch::Set(nick.into()));
        self.edit(user_id, &body, reason).await
    }

    pub async fn add_role(
        self,
        user_id: UserId,
        role_id: RoleId,
        reason: Option<&str>,
    ) -> Result<(), Error> {
        self.ctx
            .rest()
            .add_guild_member_role(self.guild_id, user_id, role_id, reason)
            .await
    }

    pub async fn remove_role(
        self,
        user_id: UserId,
        role_id: RoleId,
        reason: Option<&str>,
    ) -> Result<(), Error> {
        self.ctx
            .rest()
            .remove_guild_member_role(self.guild_id, user_id, role_id, reason)
            .await
    }

    pub async fn kick(self, user_id: UserId, reason: Option<&str>) -> Result<(), Error> {
        self.ctx
            .rest()
            .remove_guild_member(self.guild_id, user_id, reason)
            .await
    }

    pub async fn ban(
        self,
        user_id: UserId,
        body: &CreateGuildBan,
        reason: Option<&str>,
    ) -> Result<(), Error> {
        self.ctx
            .bans(self.guild_id)
            .create(user_id, body, reason)
            .await
    }

    pub async fn unban(self, user_id: UserId, reason: Option<&str>) -> Result<(), Error> {
        self.ctx.bans(self.guild_id).remove(user_id, reason).await
    }

    pub async fn search(
        self,
        query: &SearchGuildMembers,
    ) -> Result<Collection<UserId, Arc<CachedMember>>, Error> {
        let members = self
            .ctx
            .rest()
            .search_guild_members(self.guild_id, query)
            .await?;
        Ok(members
            .iter()
            .map(|member| {
                let member = self.ctx.store_member(self.guild_id, member);
                (member.user.id, member)
            })
            .collect())
    }

    pub async fn edit_me(
        self,
        body: &ModifyCurrentMember,
        reason: Option<&str>,
    ) -> Result<Arc<CachedMember>, Error> {
        let member = self
            .ctx
            .rest()
            .modify_current_member(self.guild_id, body, reason)
            .await?;
        Ok(self.ctx.store_member(self.guild_id, &member))
    }

    pub async fn prune_count(self, query: &GetGuildPruneCount) -> Result<u32, Error> {
        self.ctx
            .rest()
            .get_guild_prune_count(self.guild_id, query)
            .await
    }

    pub async fn prune(
        self,
        body: &BeginGuildPrune,
        reason: Option<&str>,
    ) -> Result<Option<u32>, Error> {
        self.ctx
            .rest()
            .begin_guild_prune(self.guild_id, body, reason)
            .await
    }
}
