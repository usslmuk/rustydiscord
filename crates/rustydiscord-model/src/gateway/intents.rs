use crate::macros::int_flags;

int_flags! {
























    pub struct Intents: u64 {

        const GUILDS = 1 << 0;

        const GUILD_MEMBERS = 1 << 1;

        const GUILD_MODERATION = 1 << 2;

        const GUILD_EXPRESSIONS = 1 << 3;

        const GUILD_INTEGRATIONS = 1 << 4;

        const GUILD_WEBHOOKS = 1 << 5;

        const GUILD_INVITES = 1 << 6;

        const GUILD_VOICE_STATES = 1 << 7;

        const GUILD_PRESENCES = 1 << 8;

        const GUILD_MESSAGES = 1 << 9;

        const GUILD_MESSAGE_REACTIONS = 1 << 10;

        const GUILD_MESSAGE_TYPING = 1 << 11;

        const DIRECT_MESSAGES = 1 << 12;

        const DIRECT_MESSAGE_REACTIONS = 1 << 13;

        const DIRECT_MESSAGE_TYPING = 1 << 14;





        const MESSAGE_CONTENT = 1 << 15;

        const GUILD_SCHEDULED_EVENTS = 1 << 16;

        const AUTO_MODERATION_CONFIGURATION = 1 << 20;

        const AUTO_MODERATION_EXECUTION = 1 << 21;

        const GUILD_MESSAGE_POLLS = 1 << 24;

        const DIRECT_MESSAGE_POLLS = 1 << 25;
    }
}

impl Intents {
    pub const PRIVILEGED: Self = Self::GUILD_MEMBERS
        .union(Self::GUILD_PRESENCES)
        .union(Self::MESSAGE_CONTENT);

    pub const NON_PRIVILEGED: Self = Self::all().difference(Self::PRIVILEGED);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn privileged() {
        assert_eq!(Intents::PRIVILEGED.bits(), (1 << 1) | (1 << 8) | (1 << 15));
        assert!(!Intents::NON_PRIVILEGED.intersects(Intents::PRIVILEGED));
        assert_eq!(
            Intents::NON_PRIVILEGED | Intents::PRIVILEGED,
            Intents::all()
        );
    }

    #[test]
    fn serde_preserves_unknown_bits() {
        let intents: Intents = serde_json::from_str("1073741825").unwrap();
        assert!(intents.contains(Intents::GUILDS));
        assert_eq!(serde_json::to_string(&intents).unwrap(), "1073741825");
    }
}
