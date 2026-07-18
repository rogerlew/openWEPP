//! Strict I-JSON parsing and the integer-only RFC 8785 subset used by v1.

use std::collections::HashSet;
use std::fmt;

use serde::de::{DeserializeSeed, MapAccess, SeqAccess, Visitor};
use serde_json::{Map, Value};
use sha2::{Digest, Sha256};

use crate::error::{ErrorClass, GatePolicyError, Result};

struct StrictValue;

impl<'de> DeserializeSeed<'de> for StrictValue {
    type Value = Value;

    fn deserialize<D>(self, deserializer: D) -> std::result::Result<Value, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        deserializer.deserialize_any(StrictVisitor)
    }
}

struct StrictVisitor;

const MAX_SAFE_INTEGER: u64 = 9_007_199_254_740_991;

impl<'de> Visitor<'de> for StrictVisitor {
    type Value = Value;

    fn expecting(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str("I-JSON without duplicate keys or floating-point numbers")
    }

    fn visit_bool<E>(self, value: bool) -> std::result::Result<Value, E> {
        Ok(Value::Bool(value))
    }

    fn visit_i64<E>(self, value: i64) -> std::result::Result<Value, E>
    where
        E: serde::de::Error,
    {
        if value.unsigned_abs() <= MAX_SAFE_INTEGER {
            Ok(Value::Number(value.into()))
        } else {
            Err(E::custom("integer is outside the I-JSON safe range"))
        }
    }

    fn visit_u64<E>(self, value: u64) -> std::result::Result<Value, E>
    where
        E: serde::de::Error,
    {
        if value <= MAX_SAFE_INTEGER {
            Ok(Value::Number(value.into()))
        } else {
            Err(E::custom("integer is outside the I-JSON safe range"))
        }
    }

    fn visit_f64<E>(self, _value: f64) -> std::result::Result<Value, E>
    where
        E: serde::de::Error,
    {
        Err(E::custom("floating-point JSON is outside gate-policy/v1"))
    }

    fn visit_str<E>(self, value: &str) -> std::result::Result<Value, E> {
        Ok(Value::String(value.to_owned()))
    }

    fn visit_string<E>(self, value: String) -> std::result::Result<Value, E> {
        Ok(Value::String(value))
    }

    fn visit_none<E>(self) -> std::result::Result<Value, E> {
        Ok(Value::Null)
    }

    fn visit_unit<E>(self) -> std::result::Result<Value, E> {
        Ok(Value::Null)
    }

    fn visit_some<D>(self, deserializer: D) -> std::result::Result<Value, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        StrictValue.deserialize(deserializer)
    }

    fn visit_seq<A>(self, mut sequence: A) -> std::result::Result<Value, A::Error>
    where
        A: SeqAccess<'de>,
    {
        let mut values = Vec::new();
        while let Some(value) = sequence.next_element_seed(StrictValue)? {
            values.push(value);
        }
        Ok(Value::Array(values))
    }

    fn visit_map<A>(self, mut entries: A) -> std::result::Result<Value, A::Error>
    where
        A: MapAccess<'de>,
    {
        let mut keys = HashSet::new();
        let mut object = Map::new();
        while let Some(key) = entries.next_key::<String>()? {
            if !keys.insert(key.clone()) {
                return Err(serde::de::Error::custom(format!(
                    "duplicate JSON object key: {key}"
                )));
            }
            object.insert(key, entries.next_value_seed(StrictValue)?);
        }
        Ok(Value::Object(object))
    }
}

/// Parse one complete gate-policy JSON document while rejecting ambiguous JSON.
///
/// # Errors
///
/// Returns a typed JSON error for duplicate keys, floats, invalid encoding, or trailing data.
pub fn parse_strict(bytes: &[u8]) -> Result<Value> {
    let mut deserializer = serde_json::Deserializer::from_slice(bytes);
    let value = StrictValue
        .deserialize(&mut deserializer)
        .map_err(|error| {
            GatePolicyError::new(ErrorClass::Json, "GATE-JSON-INVALID", error.to_string())
        })?;
    deserializer.end().map_err(|error| {
        GatePolicyError::new(ErrorClass::Json, "GATE-JSON-TRAILING", error.to_string())
    })?;
    Ok(value)
}

/// Produce RFC 8785 canonical bytes for the integer-only v1 policy domain.
///
/// # Errors
///
/// Returns an identity error when the value contains an unsupported number.
pub fn canonical_bytes(value: &Value) -> Result<Vec<u8>> {
    let mut bytes = Vec::new();
    write_value(value, &mut bytes)?;
    Ok(bytes)
}

fn write_value(value: &Value, output: &mut Vec<u8>) -> Result<()> {
    match value {
        Value::Null => output.extend_from_slice(b"null"),
        Value::Bool(true) => output.extend_from_slice(b"true"),
        Value::Bool(false) => output.extend_from_slice(b"false"),
        Value::Number(number) => {
            let safe = number
                .as_i64()
                .is_some_and(|value| value.unsigned_abs() <= MAX_SAFE_INTEGER)
                || number
                    .as_u64()
                    .is_some_and(|value| value <= MAX_SAFE_INTEGER);
            if !safe {
                return Err(GatePolicyError::new(
                    ErrorClass::Identity,
                    "GATE-JCS-NUMBER",
                    "v1 admits only I-JSON safe integers",
                ));
            }
            output.extend_from_slice(number.to_string().as_bytes());
        }
        Value::String(text) => {
            let encoded = serde_json::to_string(text).map_err(|error| {
                GatePolicyError::new(ErrorClass::Json, "GATE-JCS-STRING", error.to_string())
            })?;
            output.extend_from_slice(encoded.as_bytes());
        }
        Value::Array(values) => {
            output.push(b'[');
            for (index, item) in values.iter().enumerate() {
                if index > 0 {
                    output.push(b',');
                }
                write_value(item, output)?;
            }
            output.push(b']');
        }
        Value::Object(entries) => {
            let mut keys = entries.keys().collect::<Vec<_>>();
            keys.sort_by(|left, right| left.encode_utf16().cmp(right.encode_utf16()));
            output.push(b'{');
            for (index, key) in keys.iter().enumerate() {
                if index > 0 {
                    output.push(b',');
                }
                write_value(&Value::String((*key).clone()), output)?;
                output.push(b':');
                write_value(&entries[*key], output)?;
            }
            output.push(b'}');
        }
    }
    Ok(())
}

#[must_use]
pub fn sha256_bytes(bytes: &[u8]) -> String {
    format!("{:x}", Sha256::digest(bytes))
}

/// Hash one canonical v1 value.
///
/// # Errors
///
/// Returns any canonicalization error from [`canonical_bytes`].
pub fn digest(value: &Value) -> Result<String> {
    Ok(sha256_bytes(&canonical_bytes(value)?))
}

/// Derive an object's identity after excluding its self-referential ID field.
///
/// # Errors
///
/// Returns an identity error for non-object or noncanonical input.
pub fn derived_id(value: &Value, field: &str) -> Result<String> {
    let mut payload = value.clone();
    let object = payload.as_object_mut().ok_or_else(|| {
        GatePolicyError::new(
            ErrorClass::Identity,
            "GATE-ID-NOT-OBJECT",
            "derived identity payload must be an object",
        )
    })?;
    object.remove(field);
    digest(&payload)
}

/// Validate an instance against a compiled Draft 2020-12 schema.
///
/// # Errors
///
/// Returns a schema error when compilation or validation fails.
pub fn validate_schema(schema: &Value, instance: &Value, label: &str) -> Result<()> {
    let validator = jsonschema::draft202012::new(schema).map_err(|error| {
        GatePolicyError::new(
            ErrorClass::Schema,
            "GATE-SCHEMA-COMPILE",
            format!("{label}: {error}"),
        )
    })?;
    let errors = validator
        .iter_errors(instance)
        .take(8)
        .map(|error| format!("{}: {error}", error.instance_path()))
        .collect::<Vec<_>>();
    if errors.is_empty() {
        Ok(())
    } else {
        Err(GatePolicyError::new(
            ErrorClass::Schema,
            "GATE-SCHEMA-REJECTED",
            format!("{label}: {}", errors.join("; ")),
        ))
    }
}

#[cfg(test)]
mod tests {
    use serde_json::json;

    use super::{canonical_bytes, parse_strict};

    #[test]
    fn canonicalization_sorts_utf16_keys_and_is_compact() {
        let value = json!({"z": 1, "a": [true, null], "é": "x"});
        assert_eq!(
            canonical_bytes(&value).expect("canonical bytes"),
            r#"{"a":[true,null],"z":1,"é":"x"}"#.as_bytes()
        );
    }

    #[test]
    fn strict_parser_rejects_duplicate_keys_and_floats() {
        assert!(parse_strict(br#"{"a":1,"a":2}"#).is_err());
        assert!(parse_strict(br#"{"a":1.5}"#).is_err());
    }
}
