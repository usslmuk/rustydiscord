use rustydiscord_model::{
    interaction::MessageCallbackData,
    message::Embed,
    rest::{CreateFollowupMessage, CreateMessage, EditWebhookMessage},
};

use crate::EmbedBuilder;

fn unchecked(builder: EmbedBuilder) -> Embed {
    builder.build_unchecked()
}

impl From<EmbedBuilder> for Embed {
    fn from(builder: EmbedBuilder) -> Self {
        unchecked(builder)
    }
}

impl From<EmbedBuilder> for MessageCallbackData {
    fn from(builder: EmbedBuilder) -> Self {
        Self::from(unchecked(builder))
    }
}

impl From<EmbedBuilder> for CreateMessage {
    fn from(builder: EmbedBuilder) -> Self {
        Self::from(unchecked(builder))
    }
}

impl From<EmbedBuilder> for CreateFollowupMessage {
    fn from(builder: EmbedBuilder) -> Self {
        Self::from(unchecked(builder))
    }
}

impl From<EmbedBuilder> for EditWebhookMessage {
    fn from(builder: EmbedBuilder) -> Self {
        Self::from(unchecked(builder))
    }
}

impl From<&EmbedBuilder> for Embed {
    fn from(builder: &EmbedBuilder) -> Self {
        builder.clone().build_unchecked()
    }
}
