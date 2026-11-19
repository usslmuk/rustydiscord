use rustydiscord_model::{
    id::{ChannelId, MessageId},
    message::Message,
    rest::GetAnswerVoters,
    user::User,
};
use serde::Deserialize;

use crate::client::boxed;
use crate::{Error, Request, Rest, Route};

#[derive(Deserialize)]
struct Voters {
    users: Vec<User>,
}

impl Rest {
    pub fn get_answer_voters<'a>(
        &'a self,
        channel_id: ChannelId,
        message_id: MessageId,
        answer_id: u32,
        query: &'a GetAnswerVoters,
    ) -> impl Future<Output = Result<Vec<User>, Error>> + Send + 'a {
        boxed(async move {
            let voters: Voters = self
                .request(
                    Request::new(Route::GetAnswerVoters {
                        channel_id,
                        message_id,
                        answer_id,
                    })
                    .query(query),
                )
                .await?;
            Ok(voters.users)
        })
    }

    pub fn end_poll(
        &self,
        channel_id: ChannelId,
        message_id: MessageId,
    ) -> impl Future<Output = Result<Message, Error>> + Send + '_ {
        boxed(async move {
            self.call(Request::new(Route::EndPoll {
                channel_id,
                message_id,
            }))
            .await
        })
    }
}
