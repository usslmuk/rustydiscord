mod close_code;
mod event;
mod intents;
mod opcode;
pub mod payload;
mod presence;
mod shard_id;

pub use self::{
    close_code::CloseCode,
    event::{DispatchEvent, GatewayEvent, GatewayEventDeserializer},
    intents::Intents,
    opcode::OpCode,
    presence::{
        Activity, ActivityAssets, ActivityEmoji, ActivityFlags, ActivityParty, ActivitySecrets,
        ActivityTimestamps, ActivityType, ClientStatus, Presence, PresenceUser, Status,
        StatusDisplayType,
    },
    shard_id::ShardId,
};

pub const API_VERSION: u8 = 10;
