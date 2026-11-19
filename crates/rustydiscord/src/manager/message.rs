use std::{future::Future, sync::Arc};

use rustydiscord_cache::{Backend, CachedMessage, CachedUser, ChannelView};
use rustydiscord_collection::Collection;
use rustydiscord_model::{
    channel::Channel,
    id::{ChannelId, MessageId, UserId},
    rest::{
        BulkDeleteMessages, CreateMessage, EditMessage, GetChannelMessages, GetReactions,
        StartThreadFromMessage,
    },
};
use rustydiscord_rest::{Error, ReactionEmoji};

use super::{Fetch, Paginator};
use crate::Context;

#[derive(Debug)]
pub struct MessageManager<'a, B: Backend> {
    ctx: &'a Context<B>,
    channel_id: ChannelId,
}

impl<B: Backend> Clone for MessageManager<'_, B> {
    fn clone(&self) -> Self {
        *self
    }
}

impl<B: Backend> Copy for MessageManager<'_, B> {}

impl<'a, B: Backend> MessageManager<'a, B> {
    pub(crate) const fn new(ctx: &'a Context<B>, channel_id: ChannelId) -> Self {
        Self { ctx, channel_id }
    }

    #[must_use]
    pub const fn channel_id(self) -> ChannelId {
        self.channel_id
    }

    #[must_use]
    pub fn cache(self) -> Option<ChannelView<'a, MessageId, CachedMessage, B>> {
        self.ctx
            .cache()
            .map(|cache| cache.messages(self.channel_id))
    }

    pub fn fetch(
        self,
        message_id: MessageId,
    ) -> Fetch<
        Arc<CachedMessage>,
        impl Future<Output = Result<Arc<CachedMessage>, Error>> + Send + 'a,
    > {
        let cached = self.ctx.ops().message(self.channel_id, message_id);
        let Self { ctx, channel_id } = self;
        Fetch::new(cached, async move {
            let message = ctx
                .rest()
                .get_channel_message(channel_id, message_id)
                .await?;
            Ok(ctx.store_message(&message))
        })
    }

    pub async fn list(
        self,
        query: &GetChannelMessages,
    ) -> Result<Collection<MessageId, Arc<CachedMessage>>, Error> {
        let messages = self
            .ctx
            .rest()
            .get_channel_messages(self.channel_id, query)
            .await?;
        Ok(messages
            .iter()
            .map(|message| (message.id, self.ctx.store_message(message)))
            .collect())
    }

    pub fn paginate(self, query: GetChannelMessages) -> Paginator<'a, B> {
        Paginator::new(self.ctx, self.channel_id, query)
    }

    pub fn send(
        self,
        body: impl Into<CreateMessage>,
    ) -> impl Future<Output = Result<Arc<CachedMessage>, Error>> + Send + 'a {
        let body = body.into();
        let Self { ctx, channel_id } = self;
        async move {
            let message = ctx.rest().create_message(channel_id, &body).await?;
            Ok(ctx.store_message(&message))
        }
    }

    pub async fn edit(
        self,
        message_id: MessageId,
        body: &EditMessage,
    ) -> Result<Arc<CachedMessage>, Error> {
        let message = self
            .ctx
            .rest()
            .edit_message(self.channel_id, message_id, body)
            .await?;
        Ok(self.ctx.store_message(&message))
    }

    pub async fn delete(self, message_id: MessageId) -> Result<(), Error> {
        self.ctx
            .rest()
            .delete_message(self.channel_id, message_id)
            .await
    }

    pub async fn bulk_delete(
        self,
        message_ids: Vec<MessageId>,
        reason: Option<&str>,
    ) -> Result<(), Error> {
        self.ctx
            .rest()
            .bulk_delete_messages(
                self.channel_id,
                &BulkDeleteMessages::new(message_ids),
                reason,
            )
            .await
    }

    pub fn react<'e>(
        self,
        message_id: MessageId,
        emoji: impl Into<ReactionEmoji<'e>>,
    ) -> impl Future<Output = Result<(), Error>> + Send {
        let emoji = emoji.into();
        let Self { ctx, channel_id } = self;
        async move {
            ctx.rest()
                .create_reaction(channel_id, message_id, emoji)
                .await
        }
    }

    pub async fn pin(self, message_id: MessageId, reason: Option<&str>) -> Result<(), Error> {
        self.ctx
            .rest()
            .pin_message(self.channel_id, message_id, reason)
            .await
    }

    pub async fn unpin(self, message_id: MessageId, reason: Option<&str>) -> Result<(), Error> {
        self.ctx
            .rest()
            .unpin_message(self.channel_id, message_id, reason)
            .await
    }

    pub async fn start_thread(
        self,
        message_id: MessageId,
        body: &StartThreadFromMessage,
        reason: Option<&str>,
    ) -> Result<Arc<Channel>, Error> {
        let thread = self
            .ctx
            .rest()
            .start_thread_from_message(self.channel_id, message_id, body, reason)
            .await?;
        Ok(self.ctx.store_channel(&thread))
    }

    pub async fn crosspost(self, message_id: MessageId) -> Result<Arc<CachedMessage>, Error> {
        let message = self
            .ctx
            .rest()
            .crosspost_message(self.channel_id, message_id)
            .await?;
        Ok(self.ctx.store_message(&message))
    }

    pub async fn reaction_users(
        self,
        message_id: MessageId,
        emoji: ReactionEmoji<'_>,
        query: &GetReactions,
    ) -> Result<Collection<UserId, Arc<CachedUser>>, Error> {
        let users = self
            .ctx
            .rest()
            .get_reactions(self.channel_id, message_id, emoji, query)
            .await?;
        Ok(users
            .iter()
            .map(|user| (user.id, self.ctx.store_user(user)))
            .collect())
    }

    pub fn remove_reaction<'e>(
        self,
        message_id: MessageId,
        emoji: impl Into<ReactionEmoji<'e>>,
        user: Option<UserId>,
    ) -> impl Future<Output = Result<(), Error>> + Send {
        let emoji = emoji.into();
        let Self { ctx, channel_id } = self;
        async move {
            match user {
                None => {
                    ctx.rest()
                        .delete_own_reaction(channel_id, message_id, emoji)
                        .await
                }
                Some(user_id) => {
                    ctx.rest()
                        .delete_user_reaction(channel_id, message_id, emoji, user_id)
                        .await
                }
            }
        }
    }

    pub fn remove_emoji_reactions<'e>(
        self,
        message_id: MessageId,
        emoji: impl Into<ReactionEmoji<'e>>,
    ) -> impl Future<Output = Result<(), Error>> + Send {
        let emoji = emoji.into();
        let Self { ctx, channel_id } = self;
        async move {
            ctx.rest()
                .delete_all_reactions_for_emoji(channel_id, message_id, emoji)
                .await
        }
    }

    pub async fn remove_all_reactions(self, message_id: MessageId) -> Result<(), Error> {
        self.ctx
            .rest()
            .delete_all_reactions(self.channel_id, message_id)
            .await
    }
}
