use std::{future::Future, sync::Arc};

use crate::cache::Backend;
use crate::collection::Collection;
use crate::model::{
    channel::{Channel, ThreadMember},
    id::{ChannelId, UserId},
    rest::{
        GetThreadMember, ListArchivedThreads, ListJoinedPrivateArchivedThreads, ListThreadMembers,
        StartThreadInForumOrMediaChannel, StartThreadWithoutMessage,
    },
};
use crate::rest::Error;

use super::Fetch;
use crate::Context;

#[derive(Clone, Debug)]
#[non_exhaustive]
pub struct FetchedThreads {
    pub threads: Collection<ChannelId, Arc<Channel>>,

    pub members: Vec<ThreadMember>,

    pub has_more: bool,
}

impl<B: Backend> Context<B> {
    pub(crate) fn fetched_threads(
        &self,
        threads: &[Channel],
        members: Vec<ThreadMember>,
        has_more: bool,
    ) -> FetchedThreads {
        FetchedThreads {
            threads: threads
                .iter()
                .map(|thread| (thread.id(), self.store_channel(thread)))
                .collect(),
            members,
            has_more,
        }
    }
}

#[derive(Debug)]
pub struct ThreadManager<'a, B: Backend> {
    ctx: &'a Context<B>,
    channel_id: ChannelId,
}

impl<B: Backend> Clone for ThreadManager<'_, B> {
    fn clone(&self) -> Self {
        *self
    }
}

impl<B: Backend> Copy for ThreadManager<'_, B> {}

impl<'a, B: Backend> ThreadManager<'a, B> {
    pub(crate) const fn new(ctx: &'a Context<B>, channel_id: ChannelId) -> Self {
        Self { ctx, channel_id }
    }

    #[must_use]
    pub const fn channel_id(self) -> ChannelId {
        self.channel_id
    }

    pub async fn create(
        self,
        body: &StartThreadWithoutMessage,
        reason: Option<&str>,
    ) -> Result<Arc<Channel>, Error> {
        let thread = self
            .ctx
            .rest()
            .start_thread_without_message(self.channel_id, body, reason)
            .await?;
        Ok(self.ctx.store_channel(&thread))
    }

    pub async fn create_in_forum(
        self,
        body: &StartThreadInForumOrMediaChannel,
        reason: Option<&str>,
    ) -> Result<Arc<Channel>, Error> {
        let post = self
            .ctx
            .rest()
            .start_thread_in_forum_or_media_channel(self.channel_id, body, reason)
            .await?;
        if let Some(message) = &post.message {
            self.ctx.store_message(message);
        }
        Ok(self.ctx.store_channel(&post.channel))
    }

    pub async fn list_public_archived(
        self,
        query: &ListArchivedThreads,
    ) -> Result<FetchedThreads, Error> {
        let list = self
            .ctx
            .rest()
            .list_public_archived_threads(self.channel_id, query)
            .await?;
        Ok(self
            .ctx
            .fetched_threads(&list.threads, list.members, list.has_more))
    }

    pub async fn list_private_archived(
        self,
        query: &ListArchivedThreads,
    ) -> Result<FetchedThreads, Error> {
        let list = self
            .ctx
            .rest()
            .list_private_archived_threads(self.channel_id, query)
            .await?;
        Ok(self
            .ctx
            .fetched_threads(&list.threads, list.members, list.has_more))
    }

    pub async fn list_joined_private_archived(
        self,
        query: &ListJoinedPrivateArchivedThreads,
    ) -> Result<FetchedThreads, Error> {
        let list = self
            .ctx
            .rest()
            .list_joined_private_archived_threads(self.channel_id, query)
            .await?;
        Ok(self
            .ctx
            .fetched_threads(&list.threads, list.members, list.has_more))
    }
}

#[derive(Debug)]
pub struct ThreadMemberManager<'a, B: Backend> {
    ctx: &'a Context<B>,
    thread_id: ChannelId,
}

impl<B: Backend> Clone for ThreadMemberManager<'_, B> {
    fn clone(&self) -> Self {
        *self
    }
}

impl<B: Backend> Copy for ThreadMemberManager<'_, B> {}

impl<'a, B: Backend> ThreadMemberManager<'a, B> {
    pub(crate) const fn new(ctx: &'a Context<B>, thread_id: ChannelId) -> Self {
        Self { ctx, thread_id }
    }

    #[must_use]
    pub const fn thread_id(self) -> ChannelId {
        self.thread_id
    }

    pub fn fetch<'r>(
        self,
        user_id: UserId,
        query: &'r GetThreadMember,
    ) -> Fetch<ThreadMember, impl Future<Output = Result<ThreadMember, Error>> + Send + 'r>
    where
        'a: 'r,
    {
        let Self { ctx, thread_id } = self;
        Fetch::new(None, async move {
            ctx.rest()
                .get_thread_member(thread_id, user_id, query)
                .await
        })
    }

    pub async fn list(
        self,
        query: &ListThreadMembers,
    ) -> Result<Collection<UserId, ThreadMember>, Error> {
        let members = self
            .ctx
            .rest()
            .list_thread_members(self.thread_id, query)
            .await?;
        Ok(members
            .into_iter()
            .filter_map(|member| Some((member.user_id?, member)))
            .collect())
    }

    pub async fn add(self, user_id: UserId) -> Result<(), Error> {
        self.ctx
            .rest()
            .add_thread_member(self.thread_id, user_id)
            .await
    }

    pub async fn remove(self, user_id: UserId) -> Result<(), Error> {
        self.ctx
            .rest()
            .remove_thread_member(self.thread_id, user_id)
            .await
    }

    pub async fn join(self) -> Result<(), Error> {
        self.ctx.rest().join_thread(self.thread_id).await
    }

    pub async fn leave(self) -> Result<(), Error> {
        self.ctx.rest().leave_thread(self.thread_id).await
    }
}
