use serde::{Deserialize, Deserializer, Serialize, Serializer};
use thiserror::Error;

#[derive(Clone, Debug, Error, PartialEq, Eq)]
pub enum WirePrimitiveError {
    #[error("{kind} must be exactly {digits} lowercase hexadecimal digits after 0x")]
    Hex { kind: &'static str, digits: usize },
    #[error("SHA-256 must be exactly 64 lowercase hexadecimal digits")]
    Sha256,
    #[error("wire interval index {value} exceeds 47")]
    IntervalIndex { value: u8 },
    #[error("in-progress interval index {value} is outside 1..=47")]
    InProgressIntervalIndex { value: u8 },
    #[error("accepted interval count {value} exceeds the authority bound")]
    AcceptedIntervalCount { value: u64 },
    #[error("{kind} count must be nonzero")]
    ZeroCount { kind: &'static str },
}

fn validate_hex(value: &str, digits: usize, kind: &'static str) -> Result<(), WirePrimitiveError> {
    if value.len() != digits + 2
        || !value.starts_with("0x")
        || !value[2..]
            .bytes()
            .all(|byte| byte.is_ascii_digit() || (b'a'..=b'f').contains(&byte))
    {
        return Err(WirePrimitiveError::Hex { kind, digits });
    }
    Ok(())
}

macro_rules! strict_string_wire {
    ($name:ident, $validator:expr) => {
        impl Serialize for $name {
            fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
                serializer.serialize_str(&self.0)
            }
        }
        impl<'de> Deserialize<'de> for $name {
            fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
                let value = String::deserialize(deserializer)?;
                ($validator)(&value).map_err(serde::de::Error::custom)?;
                Ok(Self(value))
            }
        }
    };
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct HexF64(String);
impl HexF64 {
    #[must_use]
    pub fn from_f64(value: f64) -> Self {
        Self(format!("0x{:016x}", value.to_bits()))
    }
    pub fn try_new(value: String) -> Result<Self, WirePrimitiveError> {
        validate_hex(&value, 16, "HexF64")?;
        Ok(Self(value))
    }
    #[must_use]
    pub fn to_f64(&self) -> f64 {
        f64::from_bits(
            u64::from_str_radix(&self.0[2..], 16)
                .unwrap_or_else(|_| unreachable!("HexF64 invariant")),
        )
    }
    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.0
    }
}
strict_string_wire!(HexF64, |value: &str| validate_hex(value, 16, "HexF64"));

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct HexU128(String);
impl HexU128 {
    #[must_use]
    pub fn from_u128(value: u128) -> Self {
        Self(format!("0x{value:032x}"))
    }
    pub fn try_new(value: String) -> Result<Self, WirePrimitiveError> {
        validate_hex(&value, 32, "HexU128")?;
        Ok(Self(value))
    }
    #[must_use]
    pub fn to_u128(&self) -> u128 {
        u128::from_str_radix(&self.0[2..], 16).unwrap_or_else(|_| unreachable!("HexU128 invariant"))
    }
}
strict_string_wire!(HexU128, |value: &str| validate_hex(value, 32, "HexU128"));

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Sha256Hex(String);
impl Sha256Hex {
    pub fn try_new(value: String) -> Result<Self, WirePrimitiveError> {
        if value.len() != 64
            || !value
                .bytes()
                .all(|byte| byte.is_ascii_digit() || (b'a'..=b'f').contains(&byte))
        {
            return Err(WirePrimitiveError::Sha256);
        }
        Ok(Self(value))
    }
    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.0
    }
}
strict_string_wire!(Sha256Hex, |value: &str| Sha256Hex::try_new(
    value.to_owned()
)
.map(|_| ()));

macro_rules! integer_wire {
    ($name:ident, $inner:ty) => {
        #[derive(Clone, Copy, Debug, Deserialize, PartialEq, Eq, Serialize)]
        #[serde(transparent)]
        pub struct $name(pub $inner);
    };
}
integer_wire!(WireDayIndex, u64);
integer_wire!(WireLaneId, u32);

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize)]
#[serde(transparent)]
pub struct WireIntervalIndex(u8);
impl WireIntervalIndex {
    pub fn try_new(value: u8) -> Result<Self, WirePrimitiveError> {
        if value > 47 {
            return Err(WirePrimitiveError::IntervalIndex { value });
        }
        Ok(Self(value))
    }
    #[must_use]
    pub const fn get(self) -> u8 {
        self.0
    }
}
impl<'de> Deserialize<'de> for WireIntervalIndex {
    fn deserialize<D: Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        Self::try_new(u8::deserialize(d)?).map_err(serde::de::Error::custom)
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize)]
#[serde(transparent)]
pub struct InProgressIntervalIndex(u8);
impl InProgressIntervalIndex {
    pub fn try_new(value: u8) -> Result<Self, WirePrimitiveError> {
        if !(1..=47).contains(&value) {
            return Err(WirePrimitiveError::InProgressIntervalIndex { value });
        }
        Ok(Self(value))
    }
    #[must_use]
    pub const fn get(self) -> u8 {
        self.0
    }
}
impl<'de> Deserialize<'de> for InProgressIntervalIndex {
    fn deserialize<D: Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        Self::try_new(u8::deserialize(d)?).map_err(serde::de::Error::custom)
    }
}

const MAX_ACCEPTED_INTERVAL_COUNT: u64 = (u32::MAX as u64) * 48;
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize)]
#[serde(transparent)]
pub struct AcceptedIntervalCount(u64);
impl AcceptedIntervalCount {
    pub fn try_new(value: u64) -> Result<Self, WirePrimitiveError> {
        if value > MAX_ACCEPTED_INTERVAL_COUNT {
            return Err(WirePrimitiveError::AcceptedIntervalCount { value });
        }
        Ok(Self(value))
    }
    #[must_use]
    pub const fn get(self) -> u64 {
        self.0
    }
}
impl<'de> Deserialize<'de> for AcceptedIntervalCount {
    fn deserialize<D: Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        Self::try_new(u64::deserialize(d)?).map_err(serde::de::Error::custom)
    }
}

macro_rules! nonzero_count {
    ($name:ident, $kind:literal) => {
        #[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize)]
        #[serde(transparent)]
        pub struct $name(u32);
        impl $name {
            pub fn try_new(value: u32) -> Result<Self, WirePrimitiveError> {
                if value == 0 {
                    return Err(WirePrimitiveError::ZeroCount { kind: $kind });
                }
                Ok(Self(value))
            }
            #[must_use]
            pub const fn get(self) -> u32 {
                self.0
            }
        }
        impl<'de> Deserialize<'de> for $name {
            fn deserialize<D: Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
                Self::try_new(u32::deserialize(d)?).map_err(serde::de::Error::custom)
            }
        }
    };
}
nonzero_count!(DestinationCount, "destination");
nonzero_count!(LaneCount, "lane");

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn hex_f64_preserves_every_bit_including_signed_zero() {
        for value in [0.0, -0.0, 1.25, f64::from_bits(1), f64::INFINITY, f64::NAN] {
            let wire = HexF64::from_f64(value);
            assert_eq!(wire.to_f64().to_bits(), value.to_bits());
        }
        assert_eq!(HexF64::from_f64(0.0).as_str(), "0x0000000000000000");
        assert_eq!(HexF64::from_f64(-0.0).as_str(), "0x8000000000000000");
    }
    #[test]
    fn canonical_hex_and_sha_forms_reject_alternatives() {
        for invalid in [
            "0x0",
            "0000000000000000",
            "0x000000000000000A",
            "0x000000000000000g",
        ] {
            assert!(HexF64::try_new(invalid.into()).is_err());
        }
        assert!(HexU128::try_new(format!("0x{}", "0".repeat(32))).is_ok());
        assert!(HexU128::try_new(format!("0x{}", "0".repeat(31))).is_err());
        assert!(Sha256Hex::try_new("a".repeat(64)).is_ok());
        assert!(Sha256Hex::try_new("A".repeat(64)).is_err());
    }
    #[test]
    fn semantic_integer_wrappers_reject_wrong_domains() {
        assert_eq!(
            serde_json::from_str::<WireIntervalIndex>("0")
                .unwrap()
                .get(),
            0
        );
        assert!(serde_json::from_str::<InProgressIntervalIndex>("0").is_err());
        assert!(serde_json::from_str::<InProgressIntervalIndex>("48").is_err());
        assert_eq!(
            serde_json::from_str::<AcceptedIntervalCount>("48")
                .unwrap()
                .get(),
            48
        );
        assert!(AcceptedIntervalCount::try_new(MAX_ACCEPTED_INTERVAL_COUNT + 1).is_err());
        assert!(serde_json::from_str::<DestinationCount>("0").is_err());
        assert!(serde_json::from_str::<LaneCount>("4294967296").is_err());
        assert!(serde_json::from_str::<WireDayIndex>("-1").is_err());
    }
}
