use std::collections::BTreeSet;

use serde::de::{MapAccess, SeqAccess, Visitor};
use serde::ser::SerializeMap;
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use thiserror::Error;

#[derive(Debug, Error, PartialEq, Eq)]
pub enum CanonicalJsonError {
    #[error("strict JSON parse failed: {0}")]
    Parse(String),
    #[error("input bytes are not the unique canonical serialization")]
    NoncanonicalBytes,
    #[error("typed JSON parse failed: {0}")]
    Typed(String),
    #[error("canonical JSON serialization failed: {0}")]
    Serialize(String),
}

#[derive(Clone, Debug)]
enum StrictJson {
    Null,
    Bool(bool),
    Number(serde_json::Number),
    String(String),
    Array(Vec<Self>),
    Object(Vec<(String, Self)>),
}

impl Serialize for StrictJson {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::Null => serializer.serialize_none(),
            Self::Bool(value) => serializer.serialize_bool(*value),
            Self::Number(value) => value.serialize(serializer),
            Self::String(value) => serializer.serialize_str(value),
            Self::Array(values) => values.serialize(serializer),
            Self::Object(entries) => {
                let mut map = serializer.serialize_map(Some(entries.len()))?;
                for (key, value) in entries {
                    map.serialize_entry(key, value)?;
                }
                map.end()
            }
        }
    }
}

struct StrictJsonVisitor;

impl<'de> Visitor<'de> for StrictJsonVisitor {
    type Value = StrictJson;

    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter.write_str("duplicate-free canonical JSON")
    }
    fn visit_unit<E>(self) -> Result<Self::Value, E> {
        Ok(StrictJson::Null)
    }
    fn visit_none<E>(self) -> Result<Self::Value, E> {
        Ok(StrictJson::Null)
    }
    fn visit_bool<E>(self, value: bool) -> Result<Self::Value, E> {
        Ok(StrictJson::Bool(value))
    }
    fn visit_i64<E>(self, value: i64) -> Result<Self::Value, E> {
        Ok(StrictJson::Number(value.into()))
    }
    fn visit_u64<E>(self, value: u64) -> Result<Self::Value, E> {
        Ok(StrictJson::Number(value.into()))
    }
    fn visit_f64<E: serde::de::Error>(self, value: f64) -> Result<Self::Value, E> {
        serde_json::Number::from_f64(value)
            .map(StrictJson::Number)
            .ok_or_else(|| E::custom("non-finite JSON number"))
    }
    fn visit_str<E>(self, value: &str) -> Result<Self::Value, E> {
        Ok(StrictJson::String(value.into()))
    }
    fn visit_string<E>(self, value: String) -> Result<Self::Value, E> {
        Ok(StrictJson::String(value))
    }
    fn visit_seq<A: SeqAccess<'de>>(self, mut sequence: A) -> Result<Self::Value, A::Error> {
        let mut values = Vec::new();
        while let Some(value) = sequence.next_element()? {
            values.push(value);
        }
        Ok(StrictJson::Array(values))
    }
    fn visit_map<A: MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
        let mut entries = Vec::new();
        let mut keys = BTreeSet::new();
        while let Some((key, value)) = map.next_entry::<String, StrictJson>()? {
            if !keys.insert(key.clone()) {
                return Err(serde::de::Error::custom(format!("duplicate field {key}")));
            }
            entries.push((key, value));
        }
        Ok(StrictJson::Object(entries))
    }
}

impl<'de> Deserialize<'de> for StrictJson {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        deserializer.deserialize_any(StrictJsonVisitor)
    }
}

pub fn to_canonical_bytes<T: Serialize>(value: &T) -> Result<Vec<u8>, CanonicalJsonError> {
    let mut bytes = serde_json::to_vec(value)
        .map_err(|error| CanonicalJsonError::Serialize(error.to_string()))?;
    bytes.push(b'\n');
    Ok(bytes)
}

pub fn from_canonical_bytes<T>(bytes: &[u8]) -> Result<T, CanonicalJsonError>
where
    T: for<'de> Deserialize<'de> + Serialize,
{
    if bytes.last() != Some(&b'\n') || bytes[..bytes.len().saturating_sub(1)].contains(&b'\n') {
        return Err(CanonicalJsonError::NoncanonicalBytes);
    }
    let strict: StrictJson = serde_json::from_slice(&bytes[..bytes.len() - 1])
        .map_err(|error| CanonicalJsonError::Parse(error.to_string()))?;
    if to_canonical_bytes(&strict)? != bytes {
        return Err(CanonicalJsonError::NoncanonicalBytes);
    }
    let typed: T = serde_json::from_slice(&bytes[..bytes.len() - 1])
        .map_err(|error| CanonicalJsonError::Typed(error.to_string()))?;
    if to_canonical_bytes(&typed)? != bytes {
        return Err(CanonicalJsonError::NoncanonicalBytes);
    }
    Ok(typed)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[derive(Debug, Deserialize, PartialEq, Serialize)]
    #[serde(deny_unknown_fields)]
    struct Example {
        first: u32,
        second: String,
    }

    #[test]
    fn typed_round_trip_enforces_exact_order_and_bytes() {
        let value = Example {
            first: 1,
            second: "x".into(),
        };
        let bytes = to_canonical_bytes(&value).unwrap();
        assert_eq!(bytes, b"{\"first\":1,\"second\":\"x\"}\n");
        assert_eq!(from_canonical_bytes::<Example>(&bytes).unwrap(), value);
        assert!(from_canonical_bytes::<Example>(b"{\"second\":\"x\",\"first\":1}\n").is_err());
        assert!(from_canonical_bytes::<Example>(b"{ \"first\":1,\"second\":\"x\"}\n").is_err());
    }

    #[test]
    fn duplicate_and_unknown_members_reject_before_admission() {
        assert!(
            from_canonical_bytes::<Example>(b"{\"first\":1,\"first\":1,\"second\":\"x\"}\n")
                .is_err()
        );
        assert!(
            from_canonical_bytes::<Example>(b"{\"first\":1,\"second\":\"x\",\"third\":0}\n")
                .is_err()
        );
    }
}
