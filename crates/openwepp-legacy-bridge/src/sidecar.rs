use std::collections::{BTreeMap, BTreeSet};
use std::error::Error;
use std::fmt;
use std::path::PathBuf;

use crate::policy::CompatibilityPolicy;

/// Typed sidecar identifier used by bridge contracts.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Ord, PartialOrd)]
pub struct SidecarId(String);

impl SidecarId {
    /// Build a typed sidecar id.
    ///
    /// # Errors
    ///
    /// Returns [`SidecarAdapterError::InvalidSidecarId`] when the id is empty
    /// or contains non-identifier characters.
    pub fn new(value: impl Into<String>) -> Result<Self, SidecarAdapterError> {
        let value = value.into();
        let trimmed = value.trim();
        let valid = !trimmed.is_empty()
            && trimmed
                .bytes()
                .all(|b| b.is_ascii_alphanumeric() || b == b'_' || b == b'-');

        if !valid {
            return Err(SidecarAdapterError::InvalidSidecarId { value });
        }

        Ok(Self(trimmed.to_owned()))
    }

    #[must_use]
    pub fn as_str(&self) -> &str {
        self.0.as_str()
    }
}

/// Required/optional policy for one sidecar contract surface.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SidecarRequirement {
    Required,
    Optional,
}

impl SidecarRequirement {
    #[must_use]
    pub const fn is_required(self) -> bool {
        matches!(self, Self::Required)
    }
}

/// One sidecar surface contract for adapter normalization.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SidecarContract {
    pub id: SidecarId,
    pub canonical_file_name: String,
    pub legacy_aliases: Vec<String>,
    pub requirement: SidecarRequirement,
}

impl SidecarContract {
    #[must_use]
    pub fn new(
        id: SidecarId,
        canonical_file_name: impl Into<String>,
        legacy_aliases: Vec<String>,
        requirement: SidecarRequirement,
    ) -> Self {
        Self {
            id,
            canonical_file_name: canonical_file_name.into(),
            legacy_aliases,
            requirement,
        }
    }
}

/// One sidecar discovered at the run-directory edge boundary.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SidecarDiscovery {
    pub file_name: String,
    pub path: PathBuf,
}

impl SidecarDiscovery {
    #[must_use]
    pub fn new(file_name: impl Into<String>, path: impl Into<PathBuf>) -> Self {
        Self {
            file_name: file_name.into(),
            path: path.into(),
        }
    }
}

/// Adapter request for sidecar normalization and compatibility policy handling.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SidecarAdapterRequest {
    pub policy: CompatibilityPolicy,
    pub contracts: Vec<SidecarContract>,
    pub discovered: Vec<SidecarDiscovery>,
}

/// Source class for an accepted binding.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SidecarBindingSource {
    Canonical,
    LegacyAlias,
}

/// Normalized sidecar binding output.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SidecarBinding {
    pub sidecar_id: SidecarId,
    pub resolved_file_name: String,
    pub resolved_path: PathBuf,
    pub source: SidecarBindingSource,
}

/// Typed warning class for compatibility-mode outcomes.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SidecarWarningCode {
    LegacyAliasApplied,
    UnknownSidecarIgnored,
}

impl SidecarWarningCode {
    #[must_use]
    pub const fn message_id(self) -> &'static str {
        match self {
            Self::LegacyAliasApplied => "LSB-W-001",
            Self::UnknownSidecarIgnored => "LSB-W-002",
        }
    }
}

/// Typed warning emitted by sidecar adaptation.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SidecarWarning {
    pub code: SidecarWarningCode,
    pub sidecar_id: Option<SidecarId>,
    pub file_name: String,
    pub detail: String,
}

/// Sidecar adapter response with deterministic bindings and warnings.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SidecarAdapterResponse {
    pub bindings: Vec<SidecarBinding>,
    pub warnings: Vec<SidecarWarning>,
}

/// Sidecar adapter typed failure surface.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SidecarAdapterError {
    InvalidSidecarId {
        value: String,
    },
    InvalidFileName {
        context: &'static str,
        value: String,
    },
    DuplicateContractId {
        id: SidecarId,
    },
    DuplicateCanonicalFileName {
        file_name: String,
    },
    DuplicateLegacyAlias {
        sidecar_id: SidecarId,
        alias_file_name: String,
    },
    DuplicateDiscoveredFileName {
        file_name: String,
    },
    MissingRequiredSidecar {
        sidecar_id: SidecarId,
        canonical_file_name: String,
    },
    LegacyAliasDisallowed {
        sidecar_id: SidecarId,
        alias_file_name: String,
    },
    UnknownSidecarDisallowed {
        file_name: String,
    },
}

impl SidecarAdapterError {
    #[must_use]
    pub const fn code(&self) -> &'static str {
        match self {
            Self::InvalidSidecarId { .. } => "LSB-E-001",
            Self::InvalidFileName { .. } => "LSB-E-002",
            Self::DuplicateContractId { .. } => "LSB-E-003",
            Self::DuplicateCanonicalFileName { .. } => "LSB-E-004",
            Self::DuplicateLegacyAlias { .. } => "LSB-E-005",
            Self::DuplicateDiscoveredFileName { .. } => "LSB-E-006",
            Self::MissingRequiredSidecar { .. } => "LSB-E-007",
            Self::LegacyAliasDisallowed { .. } => "LSB-E-008",
            Self::UnknownSidecarDisallowed { .. } => "LSB-E-009",
        }
    }
}

impl fmt::Display for SidecarAdapterError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InvalidSidecarId { value } => {
                write!(f, "{} invalid sidecar id: {value}", self.code())
            }
            Self::InvalidFileName { context, value } => {
                write!(f, "{} invalid file name ({context}): {value}", self.code())
            }
            Self::DuplicateContractId { id } => {
                write!(f, "{} duplicate contract id: {}", self.code(), id.as_str())
            }
            Self::DuplicateCanonicalFileName { file_name } => {
                write!(
                    f,
                    "{} duplicate canonical file name: {file_name}",
                    self.code()
                )
            }
            Self::DuplicateLegacyAlias {
                sidecar_id,
                alias_file_name,
            } => {
                write!(
                    f,
                    "{} duplicate/invalid alias for {}: {alias_file_name}",
                    self.code(),
                    sidecar_id.as_str()
                )
            }
            Self::DuplicateDiscoveredFileName { file_name } => {
                write!(
                    f,
                    "{} duplicate discovered file name: {file_name}",
                    self.code()
                )
            }
            Self::MissingRequiredSidecar {
                sidecar_id,
                canonical_file_name,
            } => {
                write!(
                    f,
                    "{} missing required sidecar {} ({canonical_file_name})",
                    self.code(),
                    sidecar_id.as_str()
                )
            }
            Self::LegacyAliasDisallowed {
                sidecar_id,
                alias_file_name,
            } => {
                write!(
                    f,
                    "{} strict policy disallows alias {alias_file_name} for {}",
                    self.code(),
                    sidecar_id.as_str()
                )
            }
            Self::UnknownSidecarDisallowed { file_name } => {
                write!(
                    f,
                    "{} strict policy disallows unknown sidecar {file_name}",
                    self.code()
                )
            }
        }
    }
}

impl Error for SidecarAdapterError {}

/// Normalize sidecar declarations under strict/compat bridge policy.
///
/// # Errors
///
/// Returns [`SidecarAdapterError`] when contract/discovery inputs are invalid,
/// required sidecars are missing, or strict policy disallows legacy behavior.
pub fn adapt_sidecar_bindings(
    request: &SidecarAdapterRequest,
) -> Result<SidecarAdapterResponse, SidecarAdapterError> {
    let prepared_contracts = prepare_contracts(&request.contracts)?;
    let discovered = map_discovered_sidecars(&request.discovered)?;
    let mut warnings = Vec::new();

    let (mut bindings, consumed) = resolve_bindings_for_contracts(
        request.policy,
        &prepared_contracts,
        &discovered,
        &mut warnings,
    )?;

    append_unknown_discovery_warnings(request.policy, discovered, &consumed, &mut warnings)?;

    bindings.sort_by(|left, right| left.sidecar_id.cmp(&right.sidecar_id));

    Ok(SidecarAdapterResponse { bindings, warnings })
}

fn prepare_contracts(
    contracts: &[SidecarContract],
) -> Result<Vec<PreparedContract<'_>>, SidecarAdapterError> {
    let mut prepared_contracts = Vec::with_capacity(contracts.len());
    let mut seen_ids = BTreeSet::new();
    let mut seen_canonical = BTreeSet::new();

    for contract in contracts {
        if !seen_ids.insert(contract.id.clone()) {
            return Err(SidecarAdapterError::DuplicateContractId {
                id: contract.id.clone(),
            });
        }

        let canonical = normalize_file_name(
            contract.canonical_file_name.as_str(),
            "contract canonical_file_name",
        )?;

        if !seen_canonical.insert(canonical.clone()) {
            return Err(SidecarAdapterError::DuplicateCanonicalFileName {
                file_name: canonical,
            });
        }

        let mut aliases = Vec::new();
        let mut seen_aliases = BTreeSet::new();
        for alias in &contract.legacy_aliases {
            let normalized = normalize_file_name(alias.as_str(), "contract legacy_aliases")?;
            if normalized == canonical || !seen_aliases.insert(normalized.clone()) {
                return Err(SidecarAdapterError::DuplicateLegacyAlias {
                    sidecar_id: contract.id.clone(),
                    alias_file_name: normalized,
                });
            }
            aliases.push(normalized);
        }

        prepared_contracts.push(PreparedContract {
            contract,
            canonical,
            aliases,
        });
    }

    Ok(prepared_contracts)
}

fn map_discovered_sidecars(
    discovered: &[SidecarDiscovery],
) -> Result<BTreeMap<String, &SidecarDiscovery>, SidecarAdapterError> {
    let mut discovered_by_name = BTreeMap::new();
    for item in discovered {
        let normalized = normalize_file_name(item.file_name.as_str(), "discovery file_name")?;
        if discovered_by_name
            .insert(normalized.clone(), item)
            .is_some()
        {
            return Err(SidecarAdapterError::DuplicateDiscoveredFileName {
                file_name: normalized,
            });
        }
    }
    Ok(discovered_by_name)
}

fn resolve_bindings_for_contracts(
    policy: CompatibilityPolicy,
    prepared_contracts: &[PreparedContract<'_>],
    discovered: &BTreeMap<String, &SidecarDiscovery>,
    warnings: &mut Vec<SidecarWarning>,
) -> Result<(Vec<SidecarBinding>, BTreeSet<String>), SidecarAdapterError> {
    let mut bindings = Vec::new();
    let mut consumed = BTreeSet::new();

    for prepared in prepared_contracts {
        if let Some(found) = discovered.get(&prepared.canonical) {
            consumed.insert(prepared.canonical.clone());
            bindings.push(SidecarBinding {
                sidecar_id: prepared.contract.id.clone(),
                resolved_file_name: found.file_name.clone(),
                resolved_path: found.path.clone(),
                source: SidecarBindingSource::Canonical,
            });
            continue;
        }

        let alias_match = prepared.aliases.iter().find_map(|alias| {
            discovered
                .get(alias)
                .map(|discovery| (alias.as_str(), *discovery))
        });

        if let Some((alias, found)) = alias_match {
            if !policy.allows_legacy() {
                return Err(SidecarAdapterError::LegacyAliasDisallowed {
                    sidecar_id: prepared.contract.id.clone(),
                    alias_file_name: alias.to_owned(),
                });
            }

            consumed.insert(alias.to_owned());
            bindings.push(SidecarBinding {
                sidecar_id: prepared.contract.id.clone(),
                resolved_file_name: found.file_name.clone(),
                resolved_path: found.path.clone(),
                source: SidecarBindingSource::LegacyAlias,
            });
            warnings.push(SidecarWarning {
                code: SidecarWarningCode::LegacyAliasApplied,
                sidecar_id: Some(prepared.contract.id.clone()),
                file_name: found.file_name.clone(),
                detail: format!(
                    "sidecar {} used legacy alias {}; canonical is {}",
                    prepared.contract.id.as_str(),
                    found.file_name,
                    prepared.contract.canonical_file_name
                ),
            });
            continue;
        }

        if prepared.contract.requirement.is_required() {
            return Err(SidecarAdapterError::MissingRequiredSidecar {
                sidecar_id: prepared.contract.id.clone(),
                canonical_file_name: prepared.contract.canonical_file_name.clone(),
            });
        }
    }

    Ok((bindings, consumed))
}

fn append_unknown_discovery_warnings(
    policy: CompatibilityPolicy,
    discovered: BTreeMap<String, &SidecarDiscovery>,
    consumed: &BTreeSet<String>,
    warnings: &mut Vec<SidecarWarning>,
) -> Result<(), SidecarAdapterError> {
    for (normalized_name, found) in discovered {
        if consumed.contains(&normalized_name) {
            continue;
        }

        if !policy.allows_legacy() {
            return Err(SidecarAdapterError::UnknownSidecarDisallowed {
                file_name: found.file_name.clone(),
            });
        }

        warnings.push(SidecarWarning {
            code: SidecarWarningCode::UnknownSidecarIgnored,
            sidecar_id: None,
            file_name: found.file_name.clone(),
            detail: format!(
                "ignored unknown sidecar {} at {}",
                found.file_name,
                found.path.display()
            ),
        });
    }

    Ok(())
}

struct PreparedContract<'a> {
    contract: &'a SidecarContract,
    canonical: String,
    aliases: Vec<String>,
}

fn normalize_file_name(value: &str, context: &'static str) -> Result<String, SidecarAdapterError> {
    let normalized = value.trim().to_ascii_lowercase();
    let has_separator = normalized.contains('/') || normalized.contains('\\');
    if normalized.is_empty() || has_separator {
        return Err(SidecarAdapterError::InvalidFileName {
            context,
            value: value.to_owned(),
        });
    }
    Ok(normalized)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::policy::CompatibilityPolicy;

    fn contract(
        id: &str,
        canonical: &str,
        aliases: &[&str],
        requirement: SidecarRequirement,
    ) -> SidecarContract {
        SidecarContract::new(
            SidecarId::new(id).expect("valid sidecar id"),
            canonical,
            aliases.iter().map(ToString::to_string).collect(),
            requirement,
        )
    }

    fn discovery(file_name: &str, path: &str) -> SidecarDiscovery {
        SidecarDiscovery::new(file_name, path)
    }

    #[test]
    fn strict_mode_accepts_canonical_required_and_optional_bindings() {
        let request = SidecarAdapterRequest {
            policy: CompatibilityPolicy::Strict,
            contracts: vec![
                contract(
                    "frost",
                    "frost.txt",
                    &["FROST_OLD.TXT"],
                    SidecarRequirement::Required,
                ),
                contract("snow", "snow.txt", &[], SidecarRequirement::Optional),
            ],
            discovered: vec![
                discovery("frost.txt", "/runs/x/frost.txt"),
                discovery("snow.txt", "/runs/x/snow.txt"),
            ],
        };

        let response = adapt_sidecar_bindings(&request).expect("strict canonical pass");
        assert_eq!(response.bindings.len(), 2);
        assert!(response.warnings.is_empty());

        assert_eq!(response.bindings[0].sidecar_id.as_str(), "frost");
        assert_eq!(response.bindings[0].source, SidecarBindingSource::Canonical);
        assert_eq!(response.bindings[1].sidecar_id.as_str(), "snow");
        assert_eq!(response.bindings[1].source, SidecarBindingSource::Canonical);
    }

    #[test]
    fn compat_mode_accepts_legacy_alias_with_warning() {
        let request = SidecarAdapterRequest {
            policy: CompatibilityPolicy::Compat,
            contracts: vec![contract(
                "frost",
                "frost.txt",
                &["FROST_OLD.TXT"],
                SidecarRequirement::Required,
            )],
            discovered: vec![discovery("FROST_OLD.TXT", "/runs/x/FROST_OLD.TXT")],
        };

        let response = adapt_sidecar_bindings(&request).expect("compat alias pass");
        assert_eq!(response.bindings.len(), 1);
        assert_eq!(
            response.bindings[0].source,
            SidecarBindingSource::LegacyAlias
        );
        assert_eq!(response.warnings.len(), 1);
        assert_eq!(
            response.warnings[0].code,
            SidecarWarningCode::LegacyAliasApplied
        );
        assert_eq!(response.warnings[0].code.message_id(), "LSB-W-001");
    }

    #[test]
    fn strict_mode_rejects_legacy_alias() {
        let request = SidecarAdapterRequest {
            policy: CompatibilityPolicy::Strict,
            contracts: vec![contract(
                "frost",
                "frost.txt",
                &["FROST_OLD.TXT"],
                SidecarRequirement::Required,
            )],
            discovered: vec![discovery("FROST_OLD.TXT", "/runs/x/FROST_OLD.TXT")],
        };

        let error = adapt_sidecar_bindings(&request).expect_err("strict alias must fail");
        assert_eq!(error.code(), "LSB-E-008");
        assert!(matches!(
            error,
            SidecarAdapterError::LegacyAliasDisallowed { .. }
        ));
    }

    #[test]
    fn strict_mode_rejects_missing_required_sidecar() {
        let request = SidecarAdapterRequest {
            policy: CompatibilityPolicy::Strict,
            contracts: vec![contract(
                "frost",
                "frost.txt",
                &[],
                SidecarRequirement::Required,
            )],
            discovered: Vec::new(),
        };

        let error = adapt_sidecar_bindings(&request).expect_err("missing required must fail");
        assert_eq!(error.code(), "LSB-E-007");
        assert!(matches!(
            error,
            SidecarAdapterError::MissingRequiredSidecar { .. }
        ));
    }

    #[test]
    fn strict_mode_rejects_unknown_sidecar() {
        let request = SidecarAdapterRequest {
            policy: CompatibilityPolicy::Strict,
            contracts: vec![contract(
                "frost",
                "frost.txt",
                &[],
                SidecarRequirement::Optional,
            )],
            discovered: vec![discovery("wepp_ui.txt", "/runs/x/wepp_ui.txt")],
        };

        let error = adapt_sidecar_bindings(&request).expect_err("unknown strict must fail");
        assert_eq!(error.code(), "LSB-E-009");
        assert!(matches!(
            error,
            SidecarAdapterError::UnknownSidecarDisallowed { .. }
        ));
    }

    #[test]
    fn compat_mode_ignores_unknown_sidecar_with_warning() {
        let request = SidecarAdapterRequest {
            policy: CompatibilityPolicy::Compat,
            contracts: vec![contract(
                "frost",
                "frost.txt",
                &[],
                SidecarRequirement::Optional,
            )],
            discovered: vec![discovery("wepp_ui.txt", "/runs/x/wepp_ui.txt")],
        };

        let response = adapt_sidecar_bindings(&request).expect("compat unknown should warn");
        assert!(response.bindings.is_empty());
        assert_eq!(response.warnings.len(), 1);
        assert_eq!(
            response.warnings[0].code,
            SidecarWarningCode::UnknownSidecarIgnored
        );
        assert_eq!(response.warnings[0].code.message_id(), "LSB-W-002");
    }

    #[test]
    fn duplicate_discovery_file_name_is_rejected() {
        let request = SidecarAdapterRequest {
            policy: CompatibilityPolicy::Compat,
            contracts: vec![contract(
                "frost",
                "frost.txt",
                &[],
                SidecarRequirement::Optional,
            )],
            discovered: vec![
                discovery("frost.txt", "/runs/a/frost.txt"),
                discovery("FROST.TXT", "/runs/b/FROST.TXT"),
            ],
        };

        let error = adapt_sidecar_bindings(&request).expect_err("duplicate discovery must fail");
        assert_eq!(error.code(), "LSB-E-006");
        assert!(matches!(
            error,
            SidecarAdapterError::DuplicateDiscoveredFileName { .. }
        ));
    }
}
