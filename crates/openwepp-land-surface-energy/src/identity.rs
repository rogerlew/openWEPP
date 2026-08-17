#![allow(clippy::missing_errors_doc)]

use core::fmt;
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};

use crate::LandSurfaceEnergyError;

macro_rules! string_identity {
    ($name:ident, $field:literal) => {
        #[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
        #[serde(transparent)]
        pub struct $name(String);

        impl $name {
            pub fn try_new(value: impl Into<String>) -> Result<Self, LandSurfaceEnergyError> {
                let value = value.into();
                if value.trim().is_empty() {
                    return Err(LandSurfaceEnergyError::topology_cardinality($field));
                }
                Ok(Self(value))
            }

            #[must_use]
            pub fn as_str(&self) -> &str {
                &self.0
            }
        }

        impl<'de> Deserialize<'de> for $name {
            fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
                let value = String::deserialize(deserializer)?;
                Self::try_new(value).map_err(serde::de::Error::custom)
            }
        }

        impl fmt::Display for $name {
            fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
                formatter.write_str(&self.0)
            }
        }
    };
}

string_identity!(OfeId, "ofe_id");
string_identity!(SurfaceId, "surface_id");
string_identity!(SourceId, "source_id");
string_identity!(ParcelId, "parcel_id");
string_identity!(ComponentId, "component_id");

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
#[serde(transparent)]
pub struct Sha256Digest(String);

impl Sha256Digest {
    pub fn try_new(value: impl Into<String>) -> Result<Self, LandSurfaceEnergyError> {
        let value = value.into();
        if value.len() != 64
            || !value
                .as_bytes()
                .iter()
                .all(|byte| byte.is_ascii_digit() || (b'a'..=b'f').contains(byte))
        {
            return Err(LandSurfaceEnergyError::Identity {
                field: "sha256",
                expected: "64 lowercase hexadecimal characters".into(),
                found: value,
            });
        }
        Ok(Self(value))
    }

    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl<'de> Deserialize<'de> for Sha256Digest {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        Self::try_new(value).map_err(serde::de::Error::custom)
    }
}

impl fmt::Display for Sha256Digest {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(&self.0)
    }
}

pub(crate) fn canonical_digest<T: Serialize>(
    value: &T,
) -> Result<Sha256Digest, LandSurfaceEnergyError> {
    let bytes = serde_json::to_vec(value)
        .map_err(|error| LandSurfaceEnergyError::MalformedSerialization(error.to_string()))?;
    Sha256Digest::try_new(format!(
        "{:x}",
        Sha256::digest(cpython_json_exponents(&bytes))
    ))
}

fn cpython_json_exponents(bytes: &[u8]) -> Vec<u8> {
    let mut output = Vec::with_capacity(bytes.len());
    let mut quoted = false;
    let mut escaped = false;
    let mut index = 0;
    while index < bytes.len() {
        let byte = bytes[index];
        if quoted {
            output.push(byte);
            if escaped {
                escaped = false;
            } else if byte == b'\\' {
                escaped = true;
            } else if byte == b'"' {
                quoted = false;
            }
            index += 1;
            continue;
        }
        if byte == b'"' {
            quoted = true;
            output.push(byte);
            index += 1;
            continue;
        }
        if byte == b'e'
            && index + 3 < bytes.len()
            && matches!(bytes[index + 1], b'+' | b'-')
            && bytes[index + 2].is_ascii_digit()
            && !bytes[index + 3].is_ascii_digit()
        {
            output.extend_from_slice(&[byte, bytes[index + 1], b'0', bytes[index + 2]]);
            index += 3;
            continue;
        }
        output.push(byte);
        index += 1;
    }
    output
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn digest_rejects_uppercase_and_wrong_length() {
        assert!(Sha256Digest::try_new("A".repeat(64)).is_err());
        assert!(Sha256Digest::try_new("a".repeat(63)).is_err());
        assert!(Sha256Digest::try_new("a".repeat(64)).is_ok());
    }

    #[test]
    fn identities_reject_empty_values_during_deserialization() {
        let result = serde_json::from_str::<OfeId>(r#""  ""#);
        assert!(result.is_err());
    }

    #[test]
    fn cpython_exponent_portability_never_rewrites_strings() {
        let input = br#"{"number":1e-7,"text":"1e-7","escaped":"x\\\"1e-7"}"#;
        let output = cpython_json_exponents(input);
        assert_eq!(
            output,
            br#"{"number":1e-07,"text":"1e-7","escaped":"x\\\"1e-7"}"#
        );
    }
}
