use crate::model::{
    id::{ChannelId, GuildId, MessageId, UserId},
    message::Message,
    rest::{
        BulkDeleteMessages, ChannelPins, CreateMessage, EditMessage, GetChannelMessages,
        GetChannelPins, GetReactions, MessageSearchResults, SearchGuildMessages,
    },
    user::User,
};
use http::StatusCode;

use crate::rest::{
    Error, ErrorKind, ReactionEmoji, Request, Rest, Route, client::boxed, error::not_yet_indexed,
};

impl Rest {
    pub fn get_channel_messages<'a>(
        &'a self,
        channel_id: ChannelId,
        query: &'a GetChannelMessages,
    ) -> impl Future<Output = Result<Vec<Message>, Error>> + Send + 'a {
        boxed(async move {
            self.call(Request::new(Route::GetChannelMessages { channel_id }).query(query))
                .await
        })
    }

    pub fn search_guild_messages<'a>(
        &'a self,
        guild_id: GuildId,
        query: &'a SearchGuildMessages,
    ) -> impl Future<Output = Result<MessageSearchResults, Error>> + Send + 'a {
        boxed(async move {
            let request = Request::new(Route::SearchGuildMessages { guild_id }).query(query);
            let (status, body) = self.execute(request).await?;
            if status == StatusCode::ACCEPTED {
                return Err(not_yet_indexed(&body));
            }
            serde_json::from_slice(&body)
                .map_err(|source| Error::with_source(ErrorKind::Deserialize, source))
        })
    }

    pub fn get_channel_message(
        &self,
        channel_id: ChannelId,
        message_id: MessageId,
    ) -> impl Future<Output = Result<Message, Error>> + Send + '_ {
        boxed(async move {
            self.call(Request::new(Route::GetChannelMessage {
                channel_id,
                message_id,
            }))
            .await
        })
    }

    pub fn create_message<'a>(
        &'a self,
        channel_id: ChannelId,
        body: &'a CreateMessage,
    ) -> impl Future<Output = Result<Message, Error>> + Send + 'a {
        boxed(async move {
            self.call(
                Request::new(Route::CreateMessage { channel_id })
                    .json(body)?
                    .files(&body.files),
            )
            .await
        })
    }

    pub fn crosspost_message(
        &self,
        channel_id: ChannelId,
        message_id: MessageId,
    ) -> impl Future<Output = Result<Message, Error>> + Send + '_ {
        boxed(async move {
            self.call(Request::new(Route::CrosspostMessage {
                channel_id,
                message_id,
            }))
            .await
        })
    }

    pub fn create_reaction<'a>(
        &'a self,
        channel_id: ChannelId,
        message_id: MessageId,
        emoji: ReactionEmoji<'a>,
    ) -> impl Future<Output = Result<(), Error>> + Send + 'a {
        boxed(async move {
            self.call(Request::new(Route::CreateReaction {
                channel_id,
                message_id,
                emoji,
            }))
            .await
        })
    }

    pub fn delete_own_reaction<'a>(
        &'a self,
        channel_id: ChannelId,
        message_id: MessageId,
        emoji: ReactionEmoji<'a>,
    ) -> impl Future<Output = Result<(), Error>> + Send + 'a {
        boxed(async move {
            self.call(Request::new(Route::DeleteOwnReaction {
                channel_id,
                message_id,
                emoji,
            }))
            .await
        })
    }

    pub fn delete_user_reaction<'a>(
        &'a self,
        channel_id: ChannelId,
        message_id: MessageId,
        emoji: ReactionEmoji<'a>,
        user_id: UserId,
    ) -> impl Future<Output = Result<(), Error>> + Send + 'a {
        boxed(async move {
            self.call(Request::new(Route::DeleteUserReaction {
                channel_id,
                message_id,
                emoji,
                user_id,
            }))
            .await
        })
    }

    pub fn get_reactions<'a>(
        &'a self,
        channel_id: ChannelId,
        message_id: MessageId,
        emoji: ReactionEmoji<'a>,
        query: &'a GetReactions,
    ) -> impl Future<Output = Result<Vec<User>, Error>> + Send + 'a {
        boxed(async move {
            self.call(
                Request::new(Route::GetReactions {
                    channel_id,
                    message_id,
                    emoji,
                })
                .query(query),
            )
            .await
        })
    }

    pub fn delete_all_reactions(
        &self,
        channel_id: ChannelId,
        message_id: MessageId,
    ) -> impl Future<Output = Result<(), Error>> + Send + '_ {
        boxed(async move {
            self.call(Request::new(Route::DeleteAllReactions {
                channel_id,
                message_id,
            }))
            .await
        })
    }

    pub fn delete_all_reactions_for_emoji<'a>(
        &'a self,
        channel_id: ChannelId,
        message_id: MessageId,
        emoji: ReactionEmoji<'a>,
    ) -> impl Future<Output = Result<(), Error>> + Send + 'a {
        boxed(async move {
            self.call(Request::new(Route::DeleteAllReactionsForEmoji {
                channel_id,
                message_id,
                emoji,
            }))
            .await
        })
    }

    pub fn edit_message<'a>(
        &'a self,
        channel_id: ChannelId,
        message_id: MessageId,
        body: &'a EditMessage,
    ) -> impl Future<Output = Result<Message, Error>> + Send + 'a {
        boxed(async move {
            self.call(
                Request::new(Route::EditMessage {
                    channel_id,
                    message_id,
                })
                .json(body)?
                .files(&body.files),
            )
            .await
        })
    }

    pub fn delete_message(
        &self,
        channel_id: ChannelId,
        message_id: MessageId,
    ) -> impl Future<Output = Result<(), Error>> + Send + '_ {
        boxed(async move {
            self.call(Request::new(Route::DeleteMessage {
                channel_id,
                message_id,
            }))
            .await
        })
    }

    pub fn bulk_delete_messages<'a>(
        &'a self,
        channel_id: ChannelId,
        body: &'a BulkDeleteMessages,
        reason: Option<&'a str>,
    ) -> impl Future<Output = Result<(), Error>> + Send + 'a {
        boxed(async move {
            self.call(
                Request::new(Route::BulkDeleteMessages { channel_id })
                    .json(body)?
                    .reason(reason),
            )
            .await
        })
    }

    pub fn get_channel_pins<'a>(
        &'a self,
        channel_id: ChannelId,
        query: &'a GetChannelPins,
    ) -> impl Future<Output = Result<ChannelPins, Error>> + Send + 'a {
        boxed(async move {
            self.call(Request::new(Route::GetChannelPins { channel_id }).query(query))
                .await
        })
    }

    pub fn pin_message<'a>(
        &'a self,
        channel_id: ChannelId,
        message_id: MessageId,
        reason: Option<&'a str>,
    ) -> impl Future<Output = Result<(), Error>> + Send + 'a {
        boxed(async move {
            self.call(
                Request::new(Route::PinMessage {
                    channel_id,
                    message_id,
                })
                .reason(reason),
            )
            .await
        })
    }

    pub fn unpin_message<'a>(
        &'a self,
        channel_id: ChannelId,
        message_id: MessageId,
        reason: Option<&'a str>,
    ) -> impl Future<Output = Result<(), Error>> + Send + 'a {
        boxed(async move {
            self.call(
                Request::new(Route::UnpinMessage {
                    channel_id,
                    message_id,
                })
                .reason(reason),
            )
            .await
        })
    }
}
