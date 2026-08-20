use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};

use crate::CoupledTimeError;

/// Raw SHA-256 bytes; hexadecimal text is never used inside canonical preimages.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub struct Digest32(pub [u8; 32]);

impl Digest32 {
    #[must_use]
    pub const fn zero() -> Self {
        Self([0; 32])
    }
}

macro_rules! identity {
    ($name:ident) => {
        #[derive(
            Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize,
        )]
        pub struct $name(pub Digest32);
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
