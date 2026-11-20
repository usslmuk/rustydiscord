use std::future::Future;

use crate::cache::Backend;
use crate::collection::Collection;
use crate::model::{
    id::{ChannelId, GuildId},
    invite::{Invite, InviteWithMetadata},
    rest::{CreateChannelInvite, GetInvite},
};
use crate::rest::Error;

use super::Fetch;
use crate::Context;

#[derive(Debug)]
pub struct InviteManager<'a, B: Backend> {
    ctx: &'a Context<B>,
}

impl<B: Backend> Clone for InviteManager<'_, B> {
    fn clone(&self) -> Self {
        *self
    }
}

impl<B: Backend> Copy for InviteManager<'_, B> {}

impl<'a, B: Backend> InviteManager<'a, B> {
    pub(crate) const fn new(ctx: &'a Context<B>) -> Self {
        Self { ctx }
    }

    pub fn fetch<'r>(
        self,
        code: &'r str,
        query: &'r GetInvite,
    ) -> Fetch<Invite, impl Future<Output = Result<Invite, Error>> + Send + 'r>
    where
        'a: 'r,
    {
        let ctx = self.ctx;
        Fetch::new(
            None,
            async move { ctx.rest().get_invite(code, query).await },
        )
    }

    pub async fn delete(self, code: &str, reason: Option<&str>) -> Result<Invite, Error> {
        self.ctx.rest().delete_invite(code, reason).await
    }

    pub async fn create(
        self,
        channel_id: ChannelId,
        body: &CreateChannelInvite,
        reason: Option<&str>,
    ) -> Result<InviteWithMetadata, Error> {
        self.ctx
            .rest()
            .create_channel_invite(channel_id, body, reason)
            .await
    }

    pub async fn list_channel(
        self,
        channel_id: ChannelId,
    ) -> Result<Collection<String, InviteWithMetadata>, Error> {
        let invites = self.ctx.rest().get_channel_invites(channel_id).await?;
        Ok(by_code(invites))
    }

    pub async fn list_guild(
        self,
        guild_id: GuildId,
    ) -> Result<Collection<String, InviteWithMetadata>, Error> {
        let invites = self.ctx.rest().get_guild_invites(guild_id).await?;
        Ok(by_code(invites))
    }
}

fn by_code(invites: Vec<InviteWithMetadata>) -> Collection<String, InviteWithMetadata> {
    invites
        .into_iter()
        .map(|invite| (invite.code.to_string(), invite))
        .collect()
}
