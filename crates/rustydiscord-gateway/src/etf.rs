use core::fmt;

use eetf::{Atom, BigInteger, Binary, FixInteger, Float, List, Map, Term};
use num_bigint::BigInt;
use num_traits::ToPrimitive;
use rustydiscord_model::gateway::{GatewayEvent, GatewayEventDeserializer, OpCode};
use serde::{Serialize, de::DeserializeSeed};
use serde_json::Value;

#[derive(Debug)]
#[non_exhaustive]
pub enum EtfError {
    Decode(Box<eetf::DecodeError>),
    Encode(eetf::EncodeError),
    Json(serde_json::Error),
    UnsupportedTerm(&'static str),
    InvalidUtf8,
    IntegerOutOfRange,
    TooDeep,
    MissingField(&'static str),
    InvalidOpcode,
    TrailingBytes,
}

impl fmt::Display for EtfError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Decode(error) => write!(f, "cannot decode ETF: {}", error.as_ref()),
            Self::Encode(error) => write!(f, "cannot encode ETF: {error}"),
            Self::Json(error) => write!(f, "JSON conversion failed: {error}"),
            Self::UnsupportedTerm(term) => write!(f, "unsupported ETF term: {term}"),
            Self::InvalidUtf8 => write!(f, "ETF string is not valid UTF-8"),
            Self::IntegerOutOfRange => write!(f, "ETF integer does not fit"),
            Self::TooDeep => write!(f, "ETF term is nested too deeply"),
            Self::MissingField(field) => write!(f, "ETF payload misses `{field}`"),
            Self::InvalidOpcode => write!(f, "ETF payload has an invalid opcode"),
            Self::TrailingBytes => write!(f, "trailing bytes after the ETF term"),
        }
    }
}

impl std::error::Error for EtfError {}

impl From<serde_json::Error> for EtfError {
    fn from(error: serde_json::Error) -> Self {
        Self::Json(error)
    }
}

const MAX_DEPTH: u32 = 128;

fn key_to_json(term: &Term) -> Result<String, EtfError> {
    match term {
        Term::Atom(atom) => Ok(atom.name.clone()),
        Term::Binary(binary) => {
            String::from_utf8(binary.bytes.clone()).map_err(|_| EtfError::InvalidUtf8)
        }
        _ => Err(EtfError::UnsupportedTerm("map key")),
    }
}

pub(crate) fn json_from_term(term: &Term) -> Result<Value, EtfError> {
    json_from_term_at(term, 0)
}

fn json_from_term_at(term: &Term, depth: u32) -> Result<Value, EtfError> {
    if depth > MAX_DEPTH {
        return Err(EtfError::TooDeep);
    }
    let deeper = depth + 1;
    match term {
        Term::FixInteger(fix) => Ok(Value::from(fix.value)),
        Term::BigInteger(big) => big
            .value
            .to_i64()
            .map(Value::from)
            .or_else(|| big.value.to_u64().map(Value::from))
            .ok_or(EtfError::IntegerOutOfRange),
        Term::Float(float) => serde_json::Number::from_f64(float.value)
            .map(Value::Number)
            .ok_or(EtfError::IntegerOutOfRange),
        Term::Atom(atom) => match atom.name.as_str() {
            "true" => Ok(Value::Bool(true)),
            "false" => Ok(Value::Bool(false)),
            "nil" | "null" | "undefined" => Ok(Value::Null),
            _ => Ok(Value::String(atom.name.clone())),
        },
        Term::Binary(binary) => String::from_utf8(binary.bytes.clone())
            .map(Value::String)
            .map_err(|_| EtfError::InvalidUtf8),
        Term::List(list) => list
            .elements
            .iter()
            .map(|element| json_from_term_at(element, deeper))
            .collect::<Result<Vec<_>, _>>()
            .map(Value::Array),
        Term::ByteList(list) => Ok(Value::Array(
            list.bytes
                .iter()
                .map(|byte| Value::from(*byte))
                .collect::<Vec<_>>(),
        )),
        Term::Tuple(tuple) => tuple
            .elements
            .iter()
            .map(|element| json_from_term_at(element, deeper))
            .collect::<Result<Vec<_>, _>>()
            .map(Value::Array),
        Term::Map(map) => map
            .map
            .iter()
            .map(|(key, value)| Ok((key_to_json(key)?, json_from_term_at(value, deeper)?)))
            .collect::<Result<serde_json::Map<_, _>, _>>()
            .map(Value::Object),
        _ => Err(EtfError::UnsupportedTerm("term")),
    }
}

fn term_from_json(value: &Value) -> Result<Term, EtfError> {
    term_from_json_at(value, 0)
}

fn term_from_json_at(value: &Value, depth: u32) -> Result<Term, EtfError> {
    if depth > MAX_DEPTH {
        return Err(EtfError::TooDeep);
    }
    let deeper = depth + 1;
    match value {
        Value::Null => Ok(Term::Atom(Atom {
            name: "nil".to_owned(),
        })),
        Value::Bool(true) => Ok(Term::Atom(Atom {
            name: "true".to_owned(),
        })),
        Value::Bool(false) => Ok(Term::Atom(Atom {
            name: "false".to_owned(),
        })),
        Value::Number(number) => {
            if let Some(int) = number.as_i64() {
                return Ok(match i32::try_from(int) {
                    Ok(value) => Term::FixInteger(FixInteger { value }),
                    Err(_) => Term::BigInteger(BigInteger {
                        value: BigInt::from(int),
                    }),
                });
            }
            if let Some(int) = number.as_u64() {
                return Ok(Term::BigInteger(BigInteger {
                    value: BigInt::from(int),
                }));
            }
            number
                .as_f64()
                .map(|value| Term::Float(Float { value }))
                .ok_or(EtfError::IntegerOutOfRange)
        }
        Value::String(text) => Ok(Term::Binary(Binary {
            bytes: text.as_bytes().to_vec(),
        })),
        Value::Array(items) => items
            .iter()
            .map(|item| term_from_json_at(item, deeper))
            .collect::<Result<Vec<_>, _>>()
            .map(|elements| Term::List(List { elements })),
        Value::Object(fields) => fields
            .iter()
            .map(|(key, value)| {
                Ok((
                    Term::Binary(Binary {
                        bytes: key.as_bytes().to_vec(),
                    }),
                    term_from_json_at(value, deeper)?,
                ))
            })
            .collect::<Result<std::collections::HashMap<_, _>, _>>()
            .map(|map| Term::Map(Map { map })),
    }
}

pub(crate) fn decode(bytes: &[u8]) -> Result<GatewayEvent, EtfError> {
    let mut slice = bytes;
    let term = Term::decode(&mut slice).map_err(|error| EtfError::Decode(Box::new(error)))?;
    if !slice.is_empty() {
        return Err(EtfError::TrailingBytes);
    }
    let value = json_from_term(&term)?;
    let op = value
        .get("op")
        .and_then(Value::as_u64)
        .ok_or(EtfError::MissingField("op"))?;
    let op = u8::try_from(op)
        .map(OpCode::from_value)
        .map_err(|_| EtfError::InvalidOpcode)?;
    let sequence = value.get("s").and_then(Value::as_u64);
    let name = value.get("t").and_then(Value::as_str).map(str::to_owned);
    let seed = GatewayEventDeserializer::new(op, sequence, name.as_deref());
    seed.deserialize(value).map_err(EtfError::Json)
}

pub(crate) fn encode<T: Serialize>(payload: &T) -> Result<Vec<u8>, EtfError> {
    encode_value(&serde_json::to_value(payload)?)
}

#[must_use = "encoding does nothing until the bytes are sent"]
pub fn encode_value(value: &Value) -> Result<Vec<u8>, EtfError> {
    let term = term_from_json(value)?;
    let mut out = Vec::new();
    term.encode(&mut out).map_err(EtfError::Encode)?;
    Ok(out)
}

pub(crate) fn parse(bytes: &[u8]) -> (Option<u64>, Result<GatewayEvent, crate::ReceiveError>) {
    use crate::{ReceiveError, ReceiveErrorKind};

    let event = decode(bytes)
        .map_err(|error| ReceiveError::with_source(ReceiveErrorKind::Deserialize, error));
    let sequence = event.as_ref().ok().and_then(|event| match event {
        GatewayEvent::Dispatch(sequence, _) => Some(*sequence),
        _ => None,
    });
    (sequence, event)
}

#[cfg(test)]
mod tests {
    use super::{decode, encode, json_from_term};
    use eetf::Term;

    fn envelope(op: u8) -> serde_json::Value {
        serde_json::json!({"op": op, "d": {"heartbeat_interval": 45000}, "s": null, "t": null})
    }

    #[test]
    fn terms_round_trip_through_json() {
        let values = [
            serde_json::json!(null),
            serde_json::json!(true),
            serde_json::json!(-5),
            serde_json::json!(127),
            serde_json::json!(u64::from(u32::MAX) + 1),
            serde_json::json!(1.5),
            serde_json::json!("hello"),
            serde_json::json!([1, "two", null]),
            serde_json::json!({"op": 10, "d": {"x": [true]}}),
        ];
        for value in values {
            let bytes = encode(&value).unwrap();
            let term = Term::decode(&mut &bytes[..]).unwrap();
            assert_eq!(json_from_term(&term).unwrap(), value);
        }
    }

    #[test]
    fn atoms_decode_like_rustydiscord_json() {
        for (term, expected) in [
            (
                Term::Atom(eetf::Atom {
                    name: "true".to_owned(),
                }),
                serde_json::json!(true),
            ),
            (
                Term::Atom(eetf::Atom {
                    name: "nil".to_owned(),
                }),
                serde_json::json!(null),
            ),
        ] {
            assert_eq!(json_from_term(&term).unwrap(), expected);
        }
    }

    #[test]
    fn hello_decodes() {
        let bytes = encode(&envelope(10)).unwrap();
        let event = decode(&bytes).unwrap();
        assert!(matches!(
            event,
            rustydiscord_model::gateway::GatewayEvent::Hello(_)
        ));
    }

    #[test]
    fn garbage_is_an_error() {
        assert!(decode(b"not etf").is_err());
        assert!(decode(&encode(&envelope(10)).unwrap()[..5]).is_err());
    }

    #[test]
    fn runaway_nesting_is_an_error_not_a_stack_overflow() {
        let mut value = serde_json::json!(1);
        for _ in 0..200 {
            value = serde_json::Value::Array(vec![value]);
        }
        assert!(matches!(encode(&value), Err(super::EtfError::TooDeep)));

        let mut term = Term::FixInteger(eetf::FixInteger { value: 1 });
        for _ in 0..200 {
            term = Term::List(eetf::List {
                elements: vec![term],
            });
        }
        assert!(matches!(
            json_from_term(&term),
            Err(super::EtfError::TooDeep)
        ));
    }
}
