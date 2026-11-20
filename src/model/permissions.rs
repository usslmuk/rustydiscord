use core::fmt;

use bitflags::bitflags;
use serde::{
    Deserialize, Deserializer, Serialize, Serializer,
    de::{Error as DeError, Unexpected, Visitor},
};

use crate::model::{
    channel::Overwrite,
    id::{GuildId, RoleId, UserId},
};

bitflags! {





























    #[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash, PartialOrd, Ord)]
    pub struct Permissions: u64 {

        const CREATE_INSTANT_INVITE = 1 << 0;

        const KICK_MEMBERS = 1 << 1;

        const BAN_MEMBERS = 1 << 2;

        const ADMINISTRATOR = 1 << 3;

        const MANAGE_CHANNELS = 1 << 4;

        const MANAGE_GUILD = 1 << 5;


        const ADD_REACTIONS = 1 << 6;

        const VIEW_AUDIT_LOG = 1 << 7;

        const PRIORITY_SPEAKER = 1 << 8;

        const STREAM = 1 << 9;


        const VIEW_CHANNEL = 1 << 10;



        const SEND_MESSAGES = 1 << 11;

        const SEND_TTS_MESSAGES = 1 << 12;

        const MANAGE_MESSAGES = 1 << 13;

        const EMBED_LINKS = 1 << 14;

        const ATTACH_FILES = 1 << 15;

        const READ_MESSAGE_HISTORY = 1 << 16;


        const MENTION_EVERYONE = 1 << 17;

        const USE_EXTERNAL_EMOJIS = 1 << 18;

        const VIEW_GUILD_INSIGHTS = 1 << 19;

        const CONNECT = 1 << 20;

        const SPEAK = 1 << 21;

        const MUTE_MEMBERS = 1 << 22;

        const DEAFEN_MEMBERS = 1 << 23;

        const MOVE_MEMBERS = 1 << 24;

        const USE_VAD = 1 << 25;

        const CHANGE_NICKNAME = 1 << 26;

        const MANAGE_NICKNAMES = 1 << 27;


        const MANAGE_ROLES = 1 << 28;

        const MANAGE_WEBHOOKS = 1 << 29;

        const MANAGE_GUILD_EXPRESSIONS = 1 << 30;


        const USE_APPLICATION_COMMANDS = 1 << 31;

        const REQUEST_TO_SPEAK = 1 << 32;

        const MANAGE_EVENTS = 1 << 33;

        const MANAGE_THREADS = 1 << 34;

        const CREATE_PUBLIC_THREADS = 1 << 35;

        const CREATE_PRIVATE_THREADS = 1 << 36;

        const USE_EXTERNAL_STICKERS = 1 << 37;

        const SEND_MESSAGES_IN_THREADS = 1 << 38;

        const USE_EMBEDDED_ACTIVITIES = 1 << 39;



        const MODERATE_MEMBERS = 1 << 40;

        const VIEW_CREATOR_MONETIZATION_ANALYTICS = 1 << 41;

        const USE_SOUNDBOARD = 1 << 42;


        const CREATE_GUILD_EXPRESSIONS = 1 << 43;


        const CREATE_EVENTS = 1 << 44;

        const USE_EXTERNAL_SOUNDS = 1 << 45;

        const SEND_VOICE_MESSAGES = 1 << 46;

        const SET_VOICE_CHANNEL_STATUS = 1 << 48;

        const SEND_POLLS = 1 << 49;



        const USE_EXTERNAL_APPS = 1 << 50;

        const PIN_MESSAGES = 1 << 51;

        const BYPASS_SLOWMODE = 1 << 52;
    }
}

impl Permissions {
    pub const NEEDS_SEND_MESSAGES: Self = Self::MENTION_EVERYONE
        .union(Self::SEND_TTS_MESSAGES)
        .union(Self::ATTACH_FILES)
        .union(Self::EMBED_LINKS);

    pub const DURING_TIMEOUT: Self = Self::VIEW_CHANNEL.union(Self::READ_MESSAGE_HISTORY);

    #[must_use]
    pub fn guild_level(owner: bool, everyone: Self, roles: impl IntoIterator<Item = Self>) -> Self {
        if owner {
            return Self::all();
        }
        let permissions = roles.into_iter().fold(everyone, Self::union);
        if permissions.contains(Self::ADMINISTRATOR) {
            Self::all()
        } else {
            permissions
        }
    }

    #[must_use]
    pub fn in_channel(
        self,
        guild_id: GuildId,
        user_id: UserId,
        role_ids: &[RoleId],
        overwrites: &[Overwrite],
    ) -> Self {
        if self.contains(Self::ADMINISTRATOR) {
            return Self::all();
        }
        let mut everyone = None;
        let mut member = None;
        let (mut allow, mut deny) = (Self::empty(), Self::empty());
        for overwrite in overwrites {
            if let Some(role_id) = overwrite.role_id() {
                if role_id.get() == guild_id.get() {
                    everyone = Some(overwrite);
                } else if role_ids.contains(&role_id) {
                    allow |= overwrite.allow;
                    deny |= overwrite.deny;
                }
            } else if overwrite.user_id() == Some(user_id) {
                member = Some(overwrite);
            }
        }
        let mut permissions = self;
        if let Some(everyone) = everyone {
            permissions = permissions.difference(everyone.deny).union(everyone.allow);
        }
        permissions = permissions.difference(deny).union(allow);
        if let Some(member) = member {
            permissions = permissions.difference(member.deny).union(member.allow);
        }

        permissions.remove(Self::ADMINISTRATOR);
        if !permissions.contains(Self::VIEW_CHANNEL) {
            return Self::empty();
        }
        if !permissions.contains(Self::SEND_MESSAGES) {
            permissions.remove(Self::NEEDS_SEND_MESSAGES);
        }
        permissions
    }

    #[must_use]
    pub const fn during_timeout(self) -> Self {
        if self.contains(Self::ADMINISTRATOR) {
            self
        } else {
            self.intersection(Self::DURING_TIMEOUT)
        }
    }
}

impl Serialize for Permissions {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        serializer.collect_str(&self.bits())
    }
}

impl<'de> Deserialize<'de> for Permissions {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        struct PermissionsVisitor;

        impl Visitor<'_> for PermissionsVisitor {
            type Value = Permissions;

            fn expecting(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                f.write_str("a permissions bitfield as a decimal string or integer")
            }

            fn visit_str<E: DeError>(self, value: &str) -> Result<Permissions, E> {
                value
                    .parse()
                    .map(Permissions::from_bits_retain)
                    .map_err(|_| E::invalid_value(Unexpected::Str(value), &self))
            }

            fn visit_u64<E: DeError>(self, value: u64) -> Result<Permissions, E> {
                Ok(Permissions::from_bits_retain(value))
            }

            fn visit_i64<E: DeError>(self, value: i64) -> Result<Permissions, E> {
                u64::try_from(value)
                    .map(Permissions::from_bits_retain)
                    .map_err(|_| E::invalid_value(Unexpected::Signed(value), &self))
            }
        }

        deserializer.deserialize_any(PermissionsVisitor)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn known_bits_match_the_docs() {
        assert_eq!(Permissions::all().bits(), 8_866_461_766_385_663);
        assert_eq!(Permissions::ADMINISTRATOR.bits(), 0x8);
        assert_eq!(Permissions::BYPASS_SLOWMODE.bits(), 0x0010_0000_0000_0000);
    }

    #[test]
    fn serializes_as_decimal_string() {
        let check = |permissions: Permissions, json: &str| {
            assert_eq!(serde_json::to_string(&permissions).unwrap(), json);
        };
        check(Permissions::empty(), r#""0""#);
        check(
            Permissions::VIEW_CHANNEL | Permissions::SEND_MESSAGES,
            r#""3072""#,
        );
        check(
            Permissions::from_bits_retain(u64::MAX),
            r#""18446744073709551615""#,
        );
    }

    #[test]
    fn deserializes_strings_and_integers_keeping_unknown_bits() {
        let unknown = (1 << 47) | (1 << 63) | (1 << 3);
        let permissions: Permissions = serde_json::from_str(r#""9223512774343131144""#).unwrap();
        assert_eq!(permissions.bits(), unknown);
        assert!(permissions.contains(Permissions::ADMINISTRATOR));
        assert_eq!(
            serde_json::to_string(&permissions).unwrap(),
            r#""9223512774343131144""#
        );

        let permissions: Permissions = serde_json::from_str("3072").unwrap();
        assert_eq!(
            permissions,
            Permissions::VIEW_CHANNEL | Permissions::SEND_MESSAGES
        );
    }

    #[test]
    fn rejects_invalid_values() {
        for json in [
            r#""""#,
            r#""-1""#,
            r#""12a""#,
            "-1",
            r#""18446744073709551616""#,
            "1.5",
        ] {
            assert!(
                serde_json::from_str::<Permissions>(json).is_err(),
                "{json} must be rejected"
            );
        }
    }
}
