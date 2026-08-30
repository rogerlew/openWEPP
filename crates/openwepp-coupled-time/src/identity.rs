use serde::{Deserialize, Deserializer, Serialize, Serializer, de};
use sha2::{Digest, Sha256};

use crate::CoupledTimeError;

/// Raw SHA-256 bytes; hexadecimal text is never used inside canonical preimages.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Digest32([u8; 32]);

impl Digest32 {
    #[must_use]
    pub const fn zero() -> Self {
        Self([0; 32])
    }
    #[must_use]
    pub const fn from_bytes(bytes: [u8; 32]) -> Self {
        Self(bytes)
    }
    #[must_use]
    pub const fn as_bytes(&self) -> &[u8; 32] {
        &self.0
    }
}
impl Serialize for Digest32 {
    fn serialize<S: Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        let mut text = String::with_capacity(64);
        for b in self.0 {
            use std::fmt::Write as _;
            write!(&mut text, "{b:02x}").map_err(serde::ser::Error::custom)?;
        }
        s.serialize_str(&text)
    }
}
impl<'de> Deserialize<'de> for Digest32 {
    fn deserialize<D: Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        let text = String::deserialize(d)?;
        if text.len() != 64
            || !text
                .bytes()
                .all(|b| b.is_ascii_hexdigit() && !b.is_ascii_uppercase())
        {
            return Err(de::Error::custom("noncanonical sha256"));
        }
        let mut out = [0; 32];
        for (i, chunk) in text.as_bytes().chunks_exact(2).enumerate() {
            let part = std::str::from_utf8(chunk).map_err(de::Error::custom)?;
            out[i] = u8::from_str_radix(part, 16).map_err(de::Error::custom)?;
        }
        Ok(Self(out))
    }
}

macro_rules! identity {
    ($name:ident) => {
        #[derive(
            Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize,
        )]
        pub struct $name(Digest32);
        impl $name {
            #[must_use]
            pub const fn from_digest(digest: Digest32) -> Self {
                Self(digest)
            }
            #[must_use]
            pub const fn digest(self) -> Digest32 {
                self.0
            }
        }
    };
}
identity!(ParentIntervalId);
identity!(ParentTransactionId);
identity!(SegmentId);
identity!(AcceptedSlabId);
identity!(AttemptId);
identity!(EventId);
identity!(ReceiptId);

/// A tagged canonical field for `OPENWEPP_CANONICAL_FRAMED_SHA256_V1`.
pub struct FramedField<'a> {
    pub tag: &'a str,
    pub value: &'a [u8],
}

/// Hash an ordered, length-framed domain and field sequence.
pub fn framed_sha256(
    domain: &str,
    fields: &[FramedField<'_>],
) -> Result<Digest32, CoupledTimeError> {
    let domain_len =
        u16::try_from(domain.len()).map_err(|_| CoupledTimeError::NonCanonicalIdentity)?;
    let mut hasher = Sha256::new();
    hasher.update(b"OPENWEPP\0");
    hasher.update(1_u16.to_be_bytes());
    hasher.update(domain_len.to_be_bytes());
    hasher.update(domain.as_bytes());
    for field in fields {
        let tag_len =
            u16::try_from(field.tag.len()).map_err(|_| CoupledTimeError::NonCanonicalIdentity)?;
        let value_len =
            u32::try_from(field.value.len()).map_err(|_| CoupledTimeError::NonCanonicalIdentity)?;
        hasher.update(tag_len.to_be_bytes());
        hasher.update(field.tag.as_bytes());
        hasher.update(value_len.to_be_bytes());
        hasher.update(field.value);
    }
    Ok(Digest32(hasher.finalize().into()))
}

#[must_use]
pub fn digest_bytes(bytes: &[u8]) -> Digest32 {
    Digest32(Sha256::digest(bytes).into())
}

fn id(domain: &str, fields: &[FramedField<'_>]) -> Result<Digest32, CoupledTimeError> {
    framed_sha256(domain, fields)
}

impl ParentIntervalId {
    pub fn derive(
        run: Digest32,
        calendar: Digest32,
        forcing: Digest32,
        support: crate::TimeSupport,
    ) -> Result<Self, CoupledTimeError> {
        let start = support.start_ns().get().to_be_bytes();
        let end = support.end_ns().get().to_be_bytes();
        Ok(Self(id(
            "parent-interval",
            &[
                FramedField {
                    tag: "run_id",
                    value: run.as_bytes(),
                },
                FramedField {
                    tag: "calendar_receipt",
                    value: calendar.as_bytes(),
                },
                FramedField {
                    tag: "forcing_receipt",
                    value: forcing.as_bytes(),
                },
                FramedField {
                    tag: "start_ns",
                    value: &start,
                },
                FramedField {
                    tag: "end_ns",
                    value: &end,
                },
            ],
        )?))
    }
}
impl ParentTransactionId {
    pub fn derive(
        run: Digest32,
        sequence: u128,
        interval: ParentIntervalId,
        owners: Digest32,
    ) -> Result<Self, CoupledTimeError> {
        let sequence = sequence.to_be_bytes();
        Ok(Self(id(
            "parent-transaction",
            &[
                FramedField {
                    tag: "run_id",
                    value: run.as_bytes(),
                },
                FramedField {
                    tag: "sequence",
                    value: &sequence,
                },
                FramedField {
                    tag: "parent_interval_id",
                    value: interval.0.as_bytes(),
                },
                FramedField {
                    tag: "begin_owner_set",
                    value: owners.as_bytes(),
                },
            ],
        )?))
    }
}
impl SegmentId {
    pub fn derive(
        parent: ParentTransactionId,
        ordinal: u32,
        support: crate::TimeSupport,
        regime: Digest32,
        participants: Digest32,
    ) -> Result<Self, CoupledTimeError> {
        let ordinal = ordinal.to_be_bytes();
        let start = support.start_ns().get().to_be_bytes();
        let end = support.end_ns().get().to_be_bytes();
        Ok(Self(id(
            "segment",
            &[
                FramedField {
                    tag: "parent_transaction_id",
                    value: parent.0.as_bytes(),
                },
                FramedField {
                    tag: "ordinal",
                    value: &ordinal,
                },
                FramedField {
                    tag: "start_ns",
                    value: &start,
                },
                FramedField {
                    tag: "end_ns",
                    value: &end,
                },
                FramedField {
                    tag: "regime_id",
                    value: regime.as_bytes(),
                },
                FramedField {
                    tag: "participant_set",
                    value: participants.as_bytes(),
                },
            ],
        )?))
    }

    /// Derive the typed boundary posture installed by a zero-duration event
    /// accepted exactly at the end of its parent support. This is not a slab
    /// segment and deliberately accepts no fabricated positive support.
    pub fn derive_terminal_event_boundary(
        parent: ParentTransactionId,
        parent_support: crate::TimeSupport,
        tick: crate::ModelTimeNs,
        event_ordinal: u32,
        predecessor_owner_set: Digest32,
    ) -> Result<Self, CoupledTimeError> {
        if tick != parent_support.end_ns() {
            return Err(CoupledTimeError::InvalidSupport);
        }
        let start = parent_support.start_ns().get().to_be_bytes();
        let end = parent_support.end_ns().get().to_be_bytes();
        let tick = tick.get().to_be_bytes();
        let event_ordinal = event_ordinal.to_be_bytes();
        Ok(Self(id(
            "terminal-event-boundary",
            &[
                FramedField {
                    tag: "parent_transaction_id",
                    value: parent.0.as_bytes(),
                },
                FramedField {
                    tag: "parent_start_ns",
                    value: &start,
                },
                FramedField {
                    tag: "parent_end_ns",
                    value: &end,
                },
                FramedField {
                    tag: "tick_ns",
                    value: &tick,
                },
                FramedField {
                    tag: "event_ordinal",
                    value: &event_ordinal,
                },
                FramedField {
                    tag: "predecessor_owner_set",
                    value: predecessor_owner_set.as_bytes(),
                },
            ],
        )?))
    }
}
impl AttemptId {
    #[allow(clippy::too_many_arguments)]
    pub fn derive(
        parent: ParentTransactionId,
        cursor: crate::ModelTimeNs,
        slab: u32,
        attempt: u32,
        support: crate::TimeSupport,
        constraint: Digest32,
        owners: Digest32,
    ) -> Result<Self, CoupledTimeError> {
        let cursor = cursor.get().to_be_bytes();
        let slab = slab.to_be_bytes();
        let attempt = attempt.to_be_bytes();
        let start = support.start_ns().get().to_be_bytes();
        let end = support.end_ns().get().to_be_bytes();
        Ok(Self(id(
            "attempt",
            &[
                FramedField {
                    tag: "parent_transaction_id",
                    value: parent.0.as_bytes(),
                },
                FramedField {
                    tag: "accepted_cursor_ns",
                    value: &cursor,
                },
                FramedField {
                    tag: "slab_ordinal",
                    value: &slab,
                },
                FramedField {
                    tag: "attempt_ordinal",
                    value: &attempt,
                },
                FramedField {
                    tag: "start_ns",
                    value: &start,
                },
                FramedField {
                    tag: "end_ns",
                    value: &end,
                },
                FramedField {
                    tag: "constraint_digest",
                    value: constraint.as_bytes(),
                },
                FramedField {
                    tag: "begin_owner_set",
                    value: owners.as_bytes(),
                },
            ],
        )?))
    }
}
