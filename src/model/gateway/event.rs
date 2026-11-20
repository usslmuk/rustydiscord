use core::fmt;

use compact_str::CompactString;
use serde::de::{DeserializeSeed, Deserializer, Error as DeError, IgnoredAny, MapAccess, Visitor};

use crate::model::gateway::{
    OpCode,
    payload::{self, Hello},
};

#[derive(Clone, Debug, PartialEq)]
#[non_exhaustive]
pub enum GatewayEvent {
    Dispatch(u64, DispatchEvent),

    Heartbeat,

    Reconnect,

    InvalidSession(bool),

    Hello(Hello),

    HeartbeatAck,

    Unknown(OpCode),
}

macro_rules! dispatch_events {
    ($( $(#[doc = $doc:literal])? $name:literal => $variant:ident($ty:ty),)*) => {








        #[derive(Clone, Debug, PartialEq)]
        #[non_exhaustive]
        pub enum DispatchEvent {
            $( $(#[doc = $doc])? $variant($ty),)*

            Resumed,

            Unknown(CompactString),
        }

        impl DispatchEvent {



            #[must_use]
            pub fn name(&self) -> &str {
                match self {
                    $(Self::$variant(_) => $name,)*
                    Self::Resumed => "RESUMED",
                    Self::Unknown(name) => name,
                }
            }


            fn decode<'de, A: MapAccess<'de>>(name: &str, map: &mut A) -> Result<Self, A::Error> {
                Ok(match name {
                    $($name => Self::$variant(map.next_value()?),)*
                    "RESUMED" => {
                        map.next_value::<IgnoredAny>()?;
                        Self::Resumed
                    }
                    other => {
                        map.next_value::<IgnoredAny>()?;
                        Self::Unknown(CompactString::from(other))
                    }
                })
            }
        }




        impl serde::Serialize for DispatchEvent {
            fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
                match self {
                    $(Self::$variant(data) => data.serialize(serializer),)*
                    Self::Resumed | Self::Unknown(_) => serializer.serialize_unit(),
                }
            }
        }
    };
}

dispatch_events! {

    "READY" => Ready(Box<payload::Ready>),

    "RATE_LIMITED" => RateLimited(Box<payload::RateLimited>),

    "APPLICATION_COMMAND_PERMISSIONS_UPDATE" => ApplicationCommandPermissionsUpdate(Box<payload::ApplicationCommandPermissionsUpdate>),

    "AUTO_MODERATION_RULE_CREATE" => AutoModerationRuleCreate(Box<payload::AutoModerationRuleCreate>),

    "AUTO_MODERATION_RULE_UPDATE" => AutoModerationRuleUpdate(Box<payload::AutoModerationRuleUpdate>),

    "AUTO_MODERATION_RULE_DELETE" => AutoModerationRuleDelete(Box<payload::AutoModerationRuleDelete>),

    "AUTO_MODERATION_ACTION_EXECUTION" => AutoModerationActionExecution(Box<payload::AutoModerationActionExecution>),

    "CHANNEL_CREATE" => ChannelCreate(Box<payload::ChannelCreate>),

    "CHANNEL_UPDATE" => ChannelUpdate(Box<payload::ChannelUpdate>),

    "CHANNEL_DELETE" => ChannelDelete(Box<payload::ChannelDelete>),

    "CHANNEL_INFO" => ChannelInfo(Box<payload::ChannelInfo>),

    "CHANNEL_PINS_UPDATE" => ChannelPinsUpdate(Box<payload::ChannelPinsUpdate>),

    "THREAD_CREATE" => ThreadCreate(Box<payload::ThreadCreate>),

    "THREAD_UPDATE" => ThreadUpdate(Box<payload::ThreadUpdate>),

    "THREAD_DELETE" => ThreadDelete(Box<payload::ThreadDelete>),

    "THREAD_LIST_SYNC" => ThreadListSync(Box<payload::ThreadListSync>),

    "THREAD_MEMBER_UPDATE" => ThreadMemberUpdate(Box<payload::ThreadMemberUpdate>),

    "THREAD_MEMBERS_UPDATE" => ThreadMembersUpdate(Box<payload::ThreadMembersUpdate>),

    "ENTITLEMENT_CREATE" => EntitlementCreate(Box<payload::EntitlementCreate>),

    "ENTITLEMENT_UPDATE" => EntitlementUpdate(Box<payload::EntitlementUpdate>),

    "ENTITLEMENT_DELETE" => EntitlementDelete(Box<payload::EntitlementDelete>),

    "GUILD_CREATE" => GuildCreate(Box<payload::GuildCreate>),

    "GUILD_UPDATE" => GuildUpdate(Box<payload::GuildUpdate>),

    "GUILD_DELETE" => GuildDelete(payload::GuildDelete),

    "GUILD_AUDIT_LOG_ENTRY_CREATE" => GuildAuditLogEntryCreate(Box<payload::GuildAuditLogEntryCreate>),

    "GUILD_BAN_ADD" => GuildBanAdd(Box<payload::GuildBanAdd>),

    "GUILD_BAN_REMOVE" => GuildBanRemove(Box<payload::GuildBanRemove>),

    "GUILD_EMOJIS_UPDATE" => GuildEmojisUpdate(Box<payload::GuildEmojisUpdate>),

    "GUILD_STICKERS_UPDATE" => GuildStickersUpdate(Box<payload::GuildStickersUpdate>),

    "GUILD_INTEGRATIONS_UPDATE" => GuildIntegrationsUpdate(payload::GuildIntegrationsUpdate),

    "GUILD_MEMBER_ADD" => GuildMemberAdd(Box<payload::GuildMemberAdd>),

    "GUILD_MEMBER_REMOVE" => GuildMemberRemove(Box<payload::GuildMemberRemove>),

    "GUILD_MEMBER_UPDATE" => GuildMemberUpdate(Box<payload::GuildMemberUpdate>),

    "GUILD_MEMBERS_CHUNK" => GuildMembersChunk(Box<payload::GuildMembersChunk>),

    "GUILD_ROLE_CREATE" => GuildRoleCreate(Box<payload::GuildRoleCreate>),

    "GUILD_ROLE_UPDATE" => GuildRoleUpdate(Box<payload::GuildRoleUpdate>),

    "GUILD_ROLE_DELETE" => GuildRoleDelete(payload::GuildRoleDelete),

    "GUILD_SCHEDULED_EVENT_CREATE" => GuildScheduledEventCreate(Box<payload::GuildScheduledEventCreate>),

    "GUILD_SCHEDULED_EVENT_UPDATE" => GuildScheduledEventUpdate(Box<payload::GuildScheduledEventUpdate>),

    "GUILD_SCHEDULED_EVENT_DELETE" => GuildScheduledEventDelete(Box<payload::GuildScheduledEventDelete>),

    "GUILD_SCHEDULED_EVENT_USER_ADD" => GuildScheduledEventUserAdd(payload::GuildScheduledEventUserAdd),

    "GUILD_SCHEDULED_EVENT_USER_REMOVE" => GuildScheduledEventUserRemove(payload::GuildScheduledEventUserRemove),

    "GUILD_SOUNDBOARD_SOUND_CREATE" => GuildSoundboardSoundCreate(Box<payload::GuildSoundboardSoundCreate>),

    "GUILD_SOUNDBOARD_SOUND_UPDATE" => GuildSoundboardSoundUpdate(Box<payload::GuildSoundboardSoundUpdate>),

    "GUILD_SOUNDBOARD_SOUND_DELETE" => GuildSoundboardSoundDelete(payload::GuildSoundboardSoundDelete),

    "GUILD_SOUNDBOARD_SOUNDS_UPDATE" => GuildSoundboardSoundsUpdate(Box<payload::GuildSoundboardSoundsUpdate>),

    "SOUNDBOARD_SOUNDS" => SoundboardSounds(Box<payload::SoundboardSounds>),

    "INTEGRATION_CREATE" => IntegrationCreate(Box<payload::IntegrationCreate>),

    "INTEGRATION_UPDATE" => IntegrationUpdate(Box<payload::IntegrationUpdate>),

    "INTEGRATION_DELETE" => IntegrationDelete(payload::IntegrationDelete),

    "INTERACTION_CREATE" => InteractionCreate(Box<payload::InteractionCreate>),

    "INVITE_CREATE" => InviteCreate(Box<payload::InviteCreate>),

    "INVITE_DELETE" => InviteDelete(Box<payload::InviteDelete>),

    "MESSAGE_CREATE" => MessageCreate(Box<payload::MessageCreate>),

    "MESSAGE_UPDATE" => MessageUpdate(Box<payload::MessageUpdate>),

    "MESSAGE_DELETE" => MessageDelete(Box<payload::MessageDelete>),

    "MESSAGE_DELETE_BULK" => MessageDeleteBulk(Box<payload::MessageDeleteBulk>),

    "MESSAGE_REACTION_ADD" => MessageReactionAdd(Box<payload::MessageReactionAdd>),

    "MESSAGE_REACTION_REMOVE" => MessageReactionRemove(Box<payload::MessageReactionRemove>),

    "MESSAGE_REACTION_REMOVE_ALL" => MessageReactionRemoveAll(payload::MessageReactionRemoveAll),

    "MESSAGE_REACTION_REMOVE_EMOJI" => MessageReactionRemoveEmoji(Box<payload::MessageReactionRemoveEmoji>),

    "MESSAGE_POLL_VOTE_ADD" => MessagePollVoteAdd(Box<payload::MessagePollVoteAdd>),

    "MESSAGE_POLL_VOTE_REMOVE" => MessagePollVoteRemove(Box<payload::MessagePollVoteRemove>),

    "PRESENCE_UPDATE" => PresenceUpdate(Box<payload::PresenceUpdate>),

    "STAGE_INSTANCE_CREATE" => StageInstanceCreate(Box<payload::StageInstanceCreate>),

    "STAGE_INSTANCE_UPDATE" => StageInstanceUpdate(Box<payload::StageInstanceUpdate>),

    "STAGE_INSTANCE_DELETE" => StageInstanceDelete(Box<payload::StageInstanceDelete>),

    "SUBSCRIPTION_CREATE" => SubscriptionCreate(Box<payload::SubscriptionCreate>),

    "SUBSCRIPTION_UPDATE" => SubscriptionUpdate(Box<payload::SubscriptionUpdate>),

    "SUBSCRIPTION_DELETE" => SubscriptionDelete(Box<payload::SubscriptionDelete>),

    "TYPING_START" => TypingStart(Box<payload::TypingStart>),

    "USER_UPDATE" => UserUpdate(Box<payload::UserUpdate>),

    "VOICE_CHANNEL_EFFECT_SEND" => VoiceChannelEffectSend(Box<payload::VoiceChannelEffectSend>),

    "VOICE_CHANNEL_START_TIME_UPDATE" => VoiceChannelStartTimeUpdate(Box<payload::VoiceChannelStartTimeUpdate>),

    "VOICE_CHANNEL_STATUS_UPDATE" => VoiceChannelStatusUpdate(Box<payload::VoiceChannelStatusUpdate>),

    "VOICE_STATE_UPDATE" => VoiceStateUpdate(Box<payload::VoiceStateUpdate>),

    "VOICE_SERVER_UPDATE" => VoiceServerUpdate(Box<payload::VoiceServerUpdate>),

    "WEBHOOKS_UPDATE" => WebhooksUpdate(payload::WebhooksUpdate),
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct GatewayEventDeserializer<'a> {
    op: OpCode,
    sequence: Option<u64>,
    event_type: Option<&'a str>,
}

impl<'a> GatewayEventDeserializer<'a> {
    #[must_use]
    pub const fn new(op: OpCode, sequence: Option<u64>, event_type: Option<&'a str>) -> Self {
        Self {
            op,
            sequence,
            event_type,
        }
    }

    #[must_use]
    pub fn from_json(json: &'a str) -> Option<Self> {
        scan::top_level(json)
    }

    #[must_use]
    pub const fn op(&self) -> OpCode {
        self.op
    }

    #[must_use]
    pub const fn sequence(&self) -> Option<u64> {
        self.sequence
    }

    #[must_use]
    pub const fn event_type(&self) -> Option<&'a str> {
        self.event_type
    }
}

impl<'de> DeserializeSeed<'de> for GatewayEventDeserializer<'_> {
    type Value = GatewayEvent;

    fn deserialize<D: Deserializer<'de>>(self, deserializer: D) -> Result<GatewayEvent, D::Error> {
        deserializer.deserialize_map(EnvelopeVisitor(self))
    }
}

struct EnvelopeVisitor<'a>(GatewayEventDeserializer<'a>);

enum Data {
    Dispatch(DispatchEvent),
    Event(GatewayEvent),
}

impl EnvelopeVisitor<'_> {
    fn data<'de, A: MapAccess<'de>>(&self, map: &mut A) -> Result<Data, A::Error> {
        let event = match self.0.op {
            OpCode::Dispatch => {
                let name = self.0.event_type.ok_or_else(|| {
                    A::Error::custom("dispatch payload without an event name (`t`)")
                })?;
                return Ok(Data::Dispatch(DispatchEvent::decode(name, map)?));
            }
            OpCode::InvalidSession => GatewayEvent::InvalidSession(map.next_value()?),
            OpCode::Hello => GatewayEvent::Hello(map.next_value()?),
            op => {
                map.next_value::<IgnoredAny>()?;
                Self::without_data(op)
            }
        };
        Ok(Data::Event(event))
    }

    fn without_data(op: OpCode) -> GatewayEvent {
        match op {
            OpCode::Heartbeat => GatewayEvent::Heartbeat,
            OpCode::Reconnect => GatewayEvent::Reconnect,
            OpCode::HeartbeatAck => GatewayEvent::HeartbeatAck,
            op => GatewayEvent::Unknown(op),
        }
    }
}

impl<'de> Visitor<'de> for EnvelopeVisitor<'_> {
    type Value = GatewayEvent;

    fn expecting(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("a gateway payload object")
    }

    fn visit_map<A: MapAccess<'de>>(self, mut map: A) -> Result<GatewayEvent, A::Error> {
        let mut data = None;
        let mut sequence = None;

        while let Some(field) = map.next_key::<Field>()? {
            match field {
                Field::Data => {
                    if data.is_some() {
                        return Err(A::Error::duplicate_field("d"));
                    }
                    data = Some(self.data(&mut map)?);
                }
                Field::Sequence => sequence = map.next_value::<Option<u64>>()?,
                Field::Other => {
                    map.next_value::<IgnoredAny>()?;
                }
            }
        }

        match (self.0.op, data) {
            (OpCode::Dispatch, Some(Data::Dispatch(event))) => {
                let sequence = sequence
                    .or(self.0.sequence)
                    .ok_or_else(|| A::Error::missing_field("s"))?;
                Ok(GatewayEvent::Dispatch(sequence, event))
            }
            (_, Some(Data::Event(event))) => Ok(event),
            (OpCode::Dispatch | OpCode::InvalidSession | OpCode::Hello, _) => {
                Err(A::Error::missing_field("d"))
            }
            (op, _) => Ok(Self::without_data(op)),
        }
    }
}

enum Field {
    Data,
    Sequence,
    Other,
}

impl<'de> serde::Deserialize<'de> for Field {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        struct FieldVisitor;

        impl Visitor<'_> for FieldVisitor {
            type Value = Field;

            fn expecting(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                f.write_str("a gateway payload key")
            }

            fn visit_str<E: DeError>(self, value: &str) -> Result<Field, E> {
                Ok(match value {
                    "d" => Field::Data,
                    "s" => Field::Sequence,
                    _ => Field::Other,
                })
            }
        }

        deserializer.deserialize_identifier(FieldVisitor)
    }
}

mod scan {
    use super::GatewayEventDeserializer;
    use crate::model::gateway::OpCode;

    struct Cursor<'a> {
        bytes: &'a [u8],
        at: usize,
    }

    impl<'a> Cursor<'a> {
        fn peek(&self) -> Option<u8> {
            self.bytes.get(self.at).copied()
        }

        fn skip_whitespace(&mut self) {
            while matches!(self.peek(), Some(b' ' | b'\t' | b'\n' | b'\r')) {
                self.at += 1;
            }
        }

        fn eat(&mut self, byte: u8) -> Option<()> {
            self.skip_whitespace();
            (self.peek()? == byte).then(|| self.at += 1)
        }

        fn string(&mut self) -> Option<(&'a [u8], bool)> {
            self.eat(b'"')?;
            let start = self.at;
            let mut escaped = false;
            loop {
                match *self.bytes.get(self.at)? {
                    b'"' => break,
                    b'\\' => {
                        escaped = true;
                        self.at += 2;
                    }
                    _ => self.at += 1,
                }
            }
            let contents = self.bytes.get(start..self.at)?;
            self.at += 1;
            Some((contents, escaped))
        }

        fn literal(&mut self, literal: &[u8]) -> bool {
            self.skip_whitespace();
            let matches = self
                .bytes
                .get(self.at..)
                .is_some_and(|rest| rest.starts_with(literal));
            if matches {
                self.at += literal.len();
            }
            matches
        }

        fn unsigned(&mut self) -> Option<u64> {
            self.skip_whitespace();
            let start = self.at;
            let mut value = 0_u64;
            while let Some(digit @ b'0'..=b'9') = self.peek() {
                value = value
                    .checked_mul(10)?
                    .checked_add(u64::from(digit - b'0'))?;
                self.at += 1;
            }
            (self.at > start).then_some(value)
        }

        fn skip_value(&mut self) -> Option<()> {
            self.skip_whitespace();
            match self.peek()? {
                b'"' => {
                    self.string()?;
                }
                b'{' | b'[' => {
                    let mut depth = 0_usize;
                    loop {
                        match *self.bytes.get(self.at)? {
                            b'{' | b'[' => {
                                depth += 1;
                                self.at += 1;
                            }
                            b'}' | b']' => {
                                depth -= 1;
                                self.at += 1;
                                if depth == 0 {
                                    break;
                                }
                            }
                            b'"' => {
                                self.string()?;
                            }
                            _ => self.at += 1,
                        }
                    }
                }
                _ => {
                    while !matches!(
                        self.peek()?,
                        b',' | b'}' | b']' | b' ' | b'\t' | b'\n' | b'\r'
                    ) {
                        self.at += 1;
                    }
                }
            }
            Some(())
        }
    }

    pub(super) fn top_level(json: &str) -> Option<GatewayEventDeserializer<'_>> {
        let mut cursor = Cursor {
            bytes: json.as_bytes(),
            at: 0,
        };
        let mut op = None;
        let mut sequence = None;
        let mut event_type = None;
        let (mut seen_sequence, mut seen_event_type) = (false, false);

        cursor.eat(b'{')?;
        cursor.skip_whitespace();
        if cursor.peek() != Some(b'}') {
            loop {
                let (key, _) = cursor.string()?;
                cursor.eat(b':')?;
                match key {
                    b"op" => {
                        op = Some(u8::try_from(cursor.unsigned()?).ok()?);
                    }
                    b"s" => {
                        seen_sequence = true;
                        if !cursor.literal(b"null") {
                            sequence = Some(cursor.unsigned()?);
                        }
                    }
                    b"t" => {
                        seen_event_type = true;
                        if !cursor.literal(b"null") {
                            let (name, escaped) = cursor.string()?;
                            if escaped {
                                return None;
                            }
                            event_type = Some(core::str::from_utf8(name).ok()?);
                        }
                    }
                    _ => cursor.skip_value()?,
                }

                if let Some(op) = op
                    && (op != OpCode::Dispatch.value() || (seen_sequence && seen_event_type))
                {
                    break;
                }

                cursor.skip_whitespace();
                match cursor.peek()? {
                    b',' => cursor.at += 1,
                    b'}' => break,
                    _ => return None,
                }
            }
        }

        Some(GatewayEventDeserializer::new(
            OpCode::from_value(op?),
            sequence,
            event_type,
        ))
    }
}

#[cfg(test)]
mod tests {
    use serde::de::DeserializeSeed;

    use super::*;

    fn decode(json: &str) -> GatewayEvent {
        let seed = GatewayEventDeserializer::from_json(json).expect("scan failed");
        let mut deserializer = serde_json::Deserializer::from_str(json);
        let event = seed
            .deserialize(&mut deserializer)
            .expect("deserialize failed");
        deserializer.end().expect("trailing data");
        event
    }

    #[test]
    fn scan_rustydiscord_order() {
        let seed = GatewayEventDeserializer::from_json(
            r#"{"t":"MESSAGE_CREATE","s":3,"op":0,"d":{"never":"read"}}"#,
        )
        .unwrap();
        assert_eq!(seed.op(), OpCode::Dispatch);
        assert_eq!(seed.sequence(), Some(3));
        assert_eq!(seed.event_type(), Some("MESSAGE_CREATE"));
    }

    #[test]
    fn scan_any_order_and_nested_decoys() {
        let json = r#"{ "d" : {"op": 9, "t": "DECOY", "s": 1, "x": [{"op":1}, "\"op\":2"]},
                        "op" : 0 , "s" : 77, "t" : "READY" }"#;
        let seed = GatewayEventDeserializer::from_json(json).unwrap();
        assert_eq!(seed.op(), OpCode::Dispatch);
        assert_eq!(seed.sequence(), Some(77));
        assert_eq!(seed.event_type(), Some("READY"));
    }

    #[test]
    fn scan_rejects_garbage() {
        for json in [
            "",
            "[]",
            "{}",
            r#"{"op":"1"}"#,
            r#"{"op":300}"#,
            "{\"t\":\"A\\u0042\",\"op\":0}",
        ] {
            assert!(
                GatewayEventDeserializer::from_json(json).is_none(),
                "{json}"
            );
        }
    }

    #[test]
    fn hello_and_control_events() {
        assert_eq!(
            decode(r#"{"t":null,"s":null,"op":10,"d":{"heartbeat_interval":45000}}"#),
            GatewayEvent::Hello(Hello {
                heartbeat_interval: 45000
            })
        );
        assert_eq!(decode(r#"{"op":11}"#), GatewayEvent::HeartbeatAck);
        assert_eq!(decode(r#"{"op":1,"d":null}"#), GatewayEvent::Heartbeat);
        assert_eq!(
            decode(r#"{"t":null,"s":null,"op":7,"d":null}"#),
            GatewayEvent::Reconnect
        );
        assert_eq!(
            decode(r#"{"op":9,"d":false}"#),
            GatewayEvent::InvalidSession(false)
        );
        assert_eq!(
            decode(r#"{"op":42,"d":{"x":1}}"#),
            GatewayEvent::Unknown(OpCode::Unknown(42))
        );
    }

    #[test]
    fn unknown_dispatch_is_skipped() {
        let event = decode(r#"{"t":"FUTURE_EVENT","s":2,"op":0,"d":{"id":"1","big":[1,2,3]}}"#);
        assert_eq!(
            event,
            GatewayEvent::Dispatch(2, DispatchEvent::Unknown("FUTURE_EVENT".into()))
        );
    }

    #[test]
    fn resumed() {
        let event = decode(r#"{"t":"RESUMED","s":10,"op":0,"d":{}}"#);
        assert_eq!(event, GatewayEvent::Dispatch(10, DispatchEvent::Resumed));
    }

    #[test]
    fn dispatch_without_sequence_fails() {
        let json = r#"{"t":"RESUMED","op":0,"d":{}}"#;
        let seed = GatewayEventDeserializer::from_json(json).unwrap();
        let mut deserializer = serde_json::Deserializer::from_str(json);
        assert!(seed.deserialize(&mut deserializer).is_err());
    }
}
