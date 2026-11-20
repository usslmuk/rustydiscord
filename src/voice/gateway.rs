use std::error::Error;

use crate::model::gateway::payload::UpdateVoiceState;

pub trait VoiceGateway: Send + Sync + 'static {
    fn send(
        &self,
        shard_id: u32,
        command: &UpdateVoiceState,
    ) -> Result<(), Box<dyn Error + Send + Sync>>;
}
