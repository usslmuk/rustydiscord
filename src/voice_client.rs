use core::sync::atomic::Ordering;
use std::{error::Error, sync::Arc};

use crate::{
    context::Shared,
    model::{
        gateway::payload::UpdateVoiceState,
        id::{GuildId, UserId},
    },
    voice::VoiceGateway,
};

pub(crate) struct ClientGateway {
    pub(crate) shared: Arc<Shared>,
}

impl VoiceGateway for ClientGateway {
    fn send(
        &self,
        _shard_id: u32,
        command: &UpdateVoiceState,
    ) -> Result<(), Box<dyn Error + Send + Sync>> {
        let total = self.shared.total_shards.load(Ordering::Acquire);
        if total == 0 {
            return Err("the client is not logged in".into());
        }
        let shard = shard_of(command.guild_id, total);
        let sender = self
            .shared
            .shard(shard)
            .ok_or_else(|| format!("the guild's shard {shard} does not run in this process"))?;
        sender.command(command)?;
        Ok(())
    }
}

pub(crate) fn shard_of(guild_id: GuildId, total: u32) -> u32 {
    let shard = (guild_id.get() >> 22) % u64::from(total.max(1));

    u32::try_from(shard).unwrap_or_default()
}

pub(crate) fn bot_id(token: &str) -> Option<UserId> {
    let encoded = token.split('.').next()?;
    let mut decoded = Vec::with_capacity(encoded.len() * 3 / 4);
    let mut bits = 0_u32;
    let mut count = 0;
    for byte in encoded.bytes().filter(|&byte| byte != b'=') {
        let value = match byte {
            b'A'..=b'Z' => byte - b'A',
            b'a'..=b'z' => byte - b'a' + 26,
            b'0'..=b'9' => byte - b'0' + 52,
            b'+' | b'-' => 62,
            b'/' | b'_' => 63,
            _ => return None,
        };
        bits = (bits << 6) | u32::from(value);
        count += 6;
        if count >= 8 {
            count -= 8;

            decoded.push(u8::try_from((bits >> count) & 0xFF).ok()?);
        }
    }
    let id = core::str::from_utf8(&decoded).ok()?.parse().ok()?;
    UserId::new_checked(id)
}

#[cfg(test)]
mod tests {
    use super::{bot_id, shard_of};
    use crate::model::id::{GuildId, UserId};

    #[test]
    fn the_bot_id_is_read_from_the_token() {
        assert_eq!(
            bot_id("MTIzNDU2Nzg5MDEyMzQ1Njc4.not.checked"),
            Some(UserId::new(123_456_789_012_345_678))
        );

        assert_eq!(bot_id("MTIzNDU=.x.y"), Some(UserId::new(12_345)));
        assert_eq!(bot_id("MTIzNDU.x.y"), Some(UserId::new(12_345)));
        assert_eq!(bot_id("test-token"), None, "not base64 of an ID");
        assert_eq!(bot_id(""), None);
        assert_eq!(bot_id("MA.x.y"), None, "the ID 0");
        assert_eq!(bot_id("!!!.x.y"), None);
    }

    #[test]
    fn guilds_map_to_their_shards() {
        assert_eq!(shard_of(GuildId::new(1 << 22), 2), 1);
        assert_eq!(shard_of(GuildId::new(3 << 22), 2), 1);
        assert_eq!(shard_of(GuildId::new(4 << 22), 2), 0);
        assert_eq!(shard_of(GuildId::new(41_771_983_423_143_937), 1), 0);
    }
}
