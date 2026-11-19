mod channel;
mod guild;
mod message;

use compact_str::CompactString;
use serde::{Deserialize, Serialize};

pub use self::{channel::*, guild::*, message::*};
use crate::{
    application::ApplicationFlags,
    gateway::{OpCode, ShardId},
    guild::UnavailableGuild,
    id::{ApplicationId, GuildId},
    user::CurrentUser,
};

macro_rules! newtype_event {
    ($(#[$meta:meta])* $name:ident($inner:ty)) => {
        $(#[$meta])*

        #[doc = concat!("Dereferences to the contained [`", stringify!($inner), "`].")]
        #[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
        #[serde(transparent)]
        pub struct $name(pub $inner);

        impl core::ops::Deref for $name {
            type Target = $inner;

            fn deref(&self) -> &$inner {
                &self.0
            }
        }

        impl core::ops::DerefMut for $name {
            fn deref_mut(&mut self) -> &mut $inner {
                &mut self.0
            }
        }

        impl From<$name> for $inner {
            fn from(event: $name) -> Self {
                event.0
            }
        }
    };
}

use newtype_event;

#[derive(Default)]
struct GuildIdExtra {
    guild_id: Option<GuildId>,
}

impl<'de> crate::serde_util::extras::Extras<'de> for GuildIdExtra {
    fn visit<A: serde::de::MapAccess<'de>>(
        &mut self,
        key: &str,
        map: &mut A,
    ) -> Result<crate::serde_util::extras::Visit, A::Error> {
        if key == "guild_id" {
            self.guild_id = Some(map.next_value()?);
            Ok(crate::serde_util::extras::Visit::Consumed)
        } else {
            Ok(crate::serde_util::extras::Visit::Pass)
        }
    }
}

macro_rules! guild_id_event {
    ($(#[$meta:meta])* $name:ident { $(#[$field_meta:meta])* $field:ident: $inner:ty }) => {
        $(#[$meta])*

        #[doc = concat!("Dereferences to the contained [`", stringify!($inner), "`].")]
        #[derive(Clone, Debug, PartialEq)]
        #[non_exhaustive]
        pub struct $name {

            pub guild_id: crate::id::GuildId,
            $(#[$field_meta])*
            pub $field: $inner,
        }

        impl core::ops::Deref for $name {
            type Target = $inner;

            fn deref(&self) -> &$inner {
                &self.$field
            }
        }

        impl core::ops::DerefMut for $name {
            fn deref_mut(&mut self) -> &mut $inner {
                &mut self.$field
            }
        }

        impl<'de> serde::Deserialize<'de> for $name {
            fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
                use serde::de::Error as _;

                let (base, extra) = crate::serde_util::extras::deserialize::<
                    $inner,
                    super::GuildIdExtra,
                    D,
                >(deserializer)?;
                Ok(Self {
                    guild_id: extra
                        .guild_id
                        .ok_or_else(|| D::Error::missing_field("guild_id"))?,
                    $field: base.ok_or_else(|| D::Error::custom("missing event data"))?,
                })
            }
        }

        impl serde::Serialize for $name {
            fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
                #[derive(serde::Serialize)]
                struct Flat<'a> {
                    guild_id: crate::id::GuildId,
                    #[serde(flatten)]
                    $field: &'a $inner,
                }

                Flat {
                    guild_id: self.guild_id,
                    $field: &self.$field,
                }
                .serialize(serializer)
            }
        }
    };
}

use guild_id_event;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[non_exhaustive]
pub struct Hello {
    pub heartbeat_interval: u64,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct Ready {
    #[serde(rename = "v")]
    pub version: u8,

    pub user: CurrentUser,

    pub guilds: Vec<UnavailableGuild>,

    pub session_id: CompactString,

    pub resume_gateway_url: CompactString,

    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub shard: Option<ShardId>,

    pub application: ReadyApplication,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[non_exhaustive]
pub struct ReadyApplication {
    pub id: ApplicationId,

    #[serde(default)]
    pub flags: ApplicationFlags,

    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        with = "crate::serde_util::option_flags_string"
    )]
    pub flags_new: Option<ApplicationFlags>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct RateLimited {
    pub opcode: OpCode,

    pub retry_after: f64,

    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub meta: Option<RateLimitMetadata>,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct RateLimitMetadata {
    pub guild_id: GuildId,

    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub nonce: Option<CompactString>,
}
