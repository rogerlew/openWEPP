use std::collections::BTreeSet;
use std::error::Error;
use std::fmt;

use crate::policy::CompatibilityPolicy;

/// Typed HBP header compatibility contract.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct HbpHeaderContract {
    pub canonical_magic: [u8; 4],
    pub legacy_magic_aliases: Vec<[u8; 4]>,
    pub minimum_bytes: usize,
}

impl HbpHeaderContract {
    #[must_use]
    pub fn new(
        canonical_magic: [u8; 4],
        legacy_magic_aliases: Vec<[u8; 4]>,
        minimum_bytes: usize,
    ) -> Self {
        Self {
            canonical_magic,
            legacy_magic_aliases,
            minimum_bytes,
        }
    }
}

/// HBP bridge adapter request.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct HbpAdapterRequest<'a> {
    pub policy: CompatibilityPolicy,
    pub contract: HbpHeaderContract,
    pub shard_bytes: &'a [u8],
}

/// Source class for accepted HBP magic.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum HbpMagicSource {
    Canonical,
    LegacyAlias,
}

/// Compatibility warning codes for HBP edge adaptation.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum HbpWarningCode {
    LegacyMagicAliasApplied,
}

impl HbpWarningCode {
    #[must_use]
    pub const fn message_id(self) -> &'static str {
        match self {
            Self::LegacyMagicAliasApplied => "HBP-W-001",
        }
    }
}

/// One HBP bridge warning.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct HbpWarning {
    pub code: HbpWarningCode,
    pub observed_magic: [u8; 4],
    pub detail: String,
}

/// HBP bridge response with typed compatibility metadata.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct HbpAdapterResponse {
    pub observed_magic: [u8; 4],
    pub payload_len: usize,
    pub magic_source: HbpMagicSource,
    pub warnings: Vec<HbpWarning>,
}

/// HBP bridge typed error surface.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum HbpAdapterError {
    ContractMinimumBytesTooSmall {
        minimum_bytes: usize,
    },
    DuplicateLegacyMagicAlias {
        magic: [u8; 4],
    },
    CanonicalMagicListedAsLegacyAlias {
        magic: [u8; 4],
    },
    ShardTooShort {
        observed_bytes: usize,
        minimum_bytes: usize,
    },
    UnknownMagic {
        observed_magic: [u8; 4],
    },
    LegacyMagicDisallowed {
        observed_magic: [u8; 4],
    },
}

impl HbpAdapterError {
    #[must_use]
    pub const fn code(&self) -> &'static str {
        match self {
            Self::ContractMinimumBytesTooSmall { .. } => "HBP-E-001",
            Self::DuplicateLegacyMagicAlias { .. } => "HBP-E-002",
            Self::CanonicalMagicListedAsLegacyAlias { .. } => "HBP-E-003",
            Self::ShardTooShort { .. } => "HBP-E-004",
            Self::UnknownMagic { .. } => "HBP-E-005",
            Self::LegacyMagicDisallowed { .. } => "HBP-E-006",
        }
    }
}

impl fmt::Display for HbpAdapterError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::ContractMinimumBytesTooSmall { minimum_bytes } => {
                write!(
                    f,
                    "{} invalid contract minimum_bytes={} (must be >= 4)",
                    self.code(),
                    minimum_bytes
                )
            }
            Self::DuplicateLegacyMagicAlias { magic } => {
                write!(
                    f,
                    "{} duplicate legacy magic alias {}",
                    self.code(),
                    format_magic(*magic)
                )
            }
            Self::CanonicalMagicListedAsLegacyAlias { magic } => {
                write!(
                    f,
                    "{} canonical magic listed as legacy alias {}",
                    self.code(),
                    format_magic(*magic)
                )
            }
            Self::ShardTooShort {
                observed_bytes,
                minimum_bytes,
            } => {
                write!(
                    f,
                    "{} shard too short: observed={} minimum={}",
                    self.code(),
                    observed_bytes,
                    minimum_bytes
                )
            }
            Self::UnknownMagic { observed_magic } => {
                write!(
                    f,
                    "{} unknown magic {}",
                    self.code(),
                    format_magic(*observed_magic)
                )
            }
            Self::LegacyMagicDisallowed { observed_magic } => {
                write!(
                    f,
                    "{} strict policy disallows legacy magic {}",
                    self.code(),
                    format_magic(*observed_magic)
                )
            }
        }
    }
}

impl Error for HbpAdapterError {}

/// Validate HBP header compatibility under strict/compat policy.
///
/// # Errors
///
/// Returns [`HbpAdapterError`] when contract inputs are invalid, shard bytes do
/// not satisfy minimum requirements, or strict policy rejects legacy magic.
pub fn adapt_hbp_header(
    request: &HbpAdapterRequest<'_>,
) -> Result<HbpAdapterResponse, HbpAdapterError> {
    validate_contract(&request.contract)?;

    if request.shard_bytes.len() < request.contract.minimum_bytes {
        return Err(HbpAdapterError::ShardTooShort {
            observed_bytes: request.shard_bytes.len(),
            minimum_bytes: request.contract.minimum_bytes,
        });
    }

    let observed_magic = [
        request.shard_bytes[0],
        request.shard_bytes[1],
        request.shard_bytes[2],
        request.shard_bytes[3],
    ];

    if observed_magic == request.contract.canonical_magic {
        return Ok(HbpAdapterResponse {
            observed_magic,
            payload_len: request.shard_bytes.len(),
            magic_source: HbpMagicSource::Canonical,
            warnings: Vec::new(),
        });
    }

    let is_legacy_alias = request
        .contract
        .legacy_magic_aliases
        .contains(&observed_magic);

    if is_legacy_alias {
        if !request.policy.allows_legacy() {
            return Err(HbpAdapterError::LegacyMagicDisallowed { observed_magic });
        }

        return Ok(HbpAdapterResponse {
            observed_magic,
            payload_len: request.shard_bytes.len(),
            magic_source: HbpMagicSource::LegacyAlias,
            warnings: vec![HbpWarning {
                code: HbpWarningCode::LegacyMagicAliasApplied,
                observed_magic,
                detail: format!(
                    "accepted legacy HBP magic {} under compat policy",
                    format_magic(observed_magic)
                ),
            }],
        });
    }

    Err(HbpAdapterError::UnknownMagic { observed_magic })
}

fn validate_contract(contract: &HbpHeaderContract) -> Result<(), HbpAdapterError> {
    if contract.minimum_bytes < 4 {
        return Err(HbpAdapterError::ContractMinimumBytesTooSmall {
            minimum_bytes: contract.minimum_bytes,
        });
    }

    let mut seen_aliases = BTreeSet::new();
    for alias in &contract.legacy_magic_aliases {
        if *alias == contract.canonical_magic {
            return Err(HbpAdapterError::CanonicalMagicListedAsLegacyAlias { magic: *alias });
        }
        if !seen_aliases.insert(*alias) {
            return Err(HbpAdapterError::DuplicateLegacyMagicAlias { magic: *alias });
        }
    }

    Ok(())
}

fn format_magic(magic: [u8; 4]) -> String {
    format!(
        "0x{:02X}{:02X}{:02X}{:02X}",
        magic[0], magic[1], magic[2], magic[3]
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::policy::CompatibilityPolicy;

    fn contract() -> HbpHeaderContract {
        HbpHeaderContract::new(*b"HBP1", vec![*b"HBP0"], 8)
    }

    #[test]
    fn strict_mode_accepts_canonical_magic() {
        let request = HbpAdapterRequest {
            policy: CompatibilityPolicy::Strict,
            contract: contract(),
            shard_bytes: b"HBP1DATA",
        };

        let response = adapt_hbp_header(&request).expect("canonical strict pass");
        assert_eq!(response.magic_source, HbpMagicSource::Canonical);
        assert!(response.warnings.is_empty());
    }

    #[test]
    fn compat_mode_accepts_legacy_magic_alias_with_warning() {
        let request = HbpAdapterRequest {
            policy: CompatibilityPolicy::Compat,
            contract: contract(),
            shard_bytes: b"HBP0DATA",
        };

        let response = adapt_hbp_header(&request).expect("legacy compat pass");
        assert_eq!(response.magic_source, HbpMagicSource::LegacyAlias);
        assert_eq!(response.warnings.len(), 1);
        assert_eq!(
            response.warnings[0].code,
            HbpWarningCode::LegacyMagicAliasApplied
        );
        assert_eq!(response.warnings[0].code.message_id(), "HBP-W-001");
    }

    #[test]
    fn strict_mode_rejects_legacy_magic_alias() {
        let request = HbpAdapterRequest {
            policy: CompatibilityPolicy::Strict,
            contract: contract(),
            shard_bytes: b"HBP0DATA",
        };

        let error = adapt_hbp_header(&request).expect_err("strict legacy alias must fail");
        assert_eq!(error.code(), "HBP-E-006");
        assert!(matches!(
            error,
            HbpAdapterError::LegacyMagicDisallowed { .. }
        ));
    }

    #[test]
    fn reject_short_shard() {
        let request = HbpAdapterRequest {
            policy: CompatibilityPolicy::Compat,
            contract: contract(),
            shard_bytes: b"HBP1",
        };

        let error = adapt_hbp_header(&request).expect_err("short shard must fail");
        assert_eq!(error.code(), "HBP-E-004");
        assert!(matches!(error, HbpAdapterError::ShardTooShort { .. }));
    }

    #[test]
    fn reject_unknown_magic() {
        let request = HbpAdapterRequest {
            policy: CompatibilityPolicy::Compat,
            contract: contract(),
            shard_bytes: b"ABCDDATA",
        };

        let error = adapt_hbp_header(&request).expect_err("unknown magic must fail");
        assert_eq!(error.code(), "HBP-E-005");
        assert!(matches!(error, HbpAdapterError::UnknownMagic { .. }));
    }

    #[test]
    fn reject_contract_with_duplicate_alias() {
        let request = HbpAdapterRequest {
            policy: CompatibilityPolicy::Compat,
            contract: HbpHeaderContract::new(*b"HBP1", vec![*b"HBP0", *b"HBP0"], 8),
            shard_bytes: b"HBP1DATA",
        };

        let error = adapt_hbp_header(&request).expect_err("duplicate alias must fail");
        assert_eq!(error.code(), "HBP-E-002");
        assert!(matches!(
            error,
            HbpAdapterError::DuplicateLegacyMagicAlias { .. }
        ));
    }
}
