use crate::model::{
    id::{ApplicationId, InteractionId, MessageId},
    interaction::{InteractionCallbackResponse, InteractionResponse},
    message::Message,
    rest::{CreateFollowupMessage, EditWebhookMessage},
};

use super::webhook::WebhookQuery;
use crate::rest::client::boxed;
use crate::rest::{
    Error, Request, Rest, Route,
    query::{Query, QueryWriter},
};

#[derive(Debug)]
struct WithResponse;

impl Query for WithResponse {
    fn write(&self, query: &mut QueryWriter<'_>) {
        query.pair("with_response", &true);
    }
}

impl Rest {
    pub fn create_interaction_response<'a>(
        &'a self,
        interaction_id: InteractionId,
        interaction_token: &'a str,
        response: &'a InteractionResponse,
    ) -> impl Future<Output = Result<(), Error>> + Send + 'a {
        boxed(async move {
            self.call(callback(interaction_id, interaction_token, response)?)
                .await
        })
    }

    pub fn create_interaction_response_with_response<'a>(
        &'a self,
        interaction_id: InteractionId,
        interaction_token: &'a str,
        response: &'a InteractionResponse,
    ) -> impl Future<Output = Result<InteractionCallbackResponse, Error>> + Send + 'a {
        boxed(async move {
            self.call(callback(interaction_id, interaction_token, response)?.query(&WithResponse))
                .await
        })
    }

    pub fn get_original_interaction_response<'a>(
        &'a self,
        application_id: ApplicationId,
        interaction_token: &'a str,
    ) -> impl Future<Output = Result<Message, Error>> + Send + 'a {
        boxed(async move {
            self.call(Request::new(Route::GetOriginalInteractionResponse {
                application_id,
                interaction_token,
            }))
            .await
        })
    }

    pub fn edit_original_interaction_response<'a>(
        &'a self,
        application_id: ApplicationId,
        interaction_token: &'a str,
        body: &'a EditWebhookMessage,
    ) -> impl Future<Output = Result<Message, Error>> + Send + 'a {
        boxed(async move {
            let query = WebhookQuery::of_edit(body);
            self.call(
                Request::new(Route::EditOriginalInteractionResponse {
                    application_id,
                    interaction_token,
                })
                .query(&query)
                .json(body)?
                .files(&body.files),
            )
            .await
        })
    }

    pub fn delete_original_interaction_response<'a>(
        &'a self,
        application_id: ApplicationId,
        interaction_token: &'a str,
    ) -> impl Future<Output = Result<(), Error>> + Send + 'a {
        boxed(async move {
            self.call(Request::new(Route::DeleteOriginalInteractionResponse {
                application_id,
                interaction_token,
            }))
            .await
        })
    }

    pub fn create_followup_message<'a>(
        &'a self,
        application_id: ApplicationId,
        interaction_token: &'a str,
        body: &'a CreateFollowupMessage,
    ) -> impl Future<Output = Result<Message, Error>> + Send + 'a {
        boxed(async move {
            self.call(
                Request::new(Route::CreateFollowupMessage {
                    application_id,
                    interaction_token,
                })
                .json(body)?
                .files(&body.files),
            )
            .await
        })
    }

    pub fn get_followup_message<'a>(
        &'a self,
        application_id: ApplicationId,
        interaction_token: &'a str,
        message_id: MessageId,
    ) -> impl Future<Output = Result<Message, Error>> + Send + 'a {
        boxed(async move {
            self.call(Request::new(Route::GetFollowupMessage {
                application_id,
                interaction_token,
                message_id,
            }))
            .await
        })
    }

    pub fn edit_followup_message<'a>(
        &'a self,
        application_id: ApplicationId,
        interaction_token: &'a str,
        message_id: MessageId,
        body: &'a EditWebhookMessage,
    ) -> impl Future<Output = Result<Message, Error>> + Send + 'a {
        boxed(async move {
            let query = WebhookQuery::of_edit(body);
            self.call(
                Request::new(Route::EditFollowupMessage {
                    application_id,
                    interaction_token,
                    message_id,
                })
                .query(&query)
                .json(body)?
                .files(&body.files),
            )
            .await
        })
    }

    pub fn delete_followup_message<'a>(
        &'a self,
        application_id: ApplicationId,
        interaction_token: &'a str,
        message_id: MessageId,
    ) -> impl Future<Output = Result<(), Error>> + Send + 'a {
        boxed(async move {
            self.call(Request::new(Route::DeleteFollowupMessage {
                application_id,
                interaction_token,
                message_id,
            }))
            .await
        })
    }
}

fn callback<'a>(
    interaction_id: InteractionId,
    interaction_token: &'a str,
    response: &'a InteractionResponse,
) -> Result<Request<'a>, Error> {
    Ok(Request::new(Route::CreateInteractionResponse {
        interaction_id,
        interaction_token,
    })
    .json(response)?
    .files(response.files()))
}
