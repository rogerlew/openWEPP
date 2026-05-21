use std::collections::{BTreeMap, BTreeSet};
use std::error::Error;
use std::fmt;

/// Canonical symbol to openWEPP boundary alias mapping entry.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Ord, PartialOrd)]
pub struct SymbolAliasEntry {
    canonical_symbol: String,
    boundary_alias: String,
}

impl SymbolAliasEntry {
    #[must_use]
    pub fn new(canonical_symbol: impl Into<String>, boundary_alias: impl Into<String>) -> Self {
        Self {
            canonical_symbol: canonical_symbol.into(),
            boundary_alias: boundary_alias.into(),
        }
    }

    #[must_use]
    pub fn canonical_symbol(&self) -> &str {
        self.canonical_symbol.as_str()
    }

    #[must_use]
    pub fn boundary_alias(&self) -> &str {
        self.boundary_alias.as_str()
    }
}

/// Alias registry validation and lookup errors.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SymbolAliasRegistryError {
    RegistryEmpty,
    EmptyCanonicalSymbol {
        row: usize,
    },
    EmptyBoundaryAlias {
        row: usize,
        canonical_symbol: String,
    },
    DuplicateAliasMapping {
        canonical_symbol: String,
        boundary_alias: String,
    },
    AmbiguousBoundaryAlias {
        boundary_alias: String,
        canonical_a: String,
        canonical_b: String,
    },
    CanonicalSymbolNotFound {
        canonical_symbol: String,
    },
    BoundaryAliasNotFound {
        boundary_alias: String,
    },
}

impl fmt::Display for SymbolAliasRegistryError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::RegistryEmpty => {
                f.write_str("symbol alias registry must contain at least one mapping")
            }
            Self::EmptyCanonicalSymbol { row } => {
                write!(f, "canonical symbol is empty at row {row}")
            }
            Self::EmptyBoundaryAlias {
                row,
                canonical_symbol,
            } => {
                write!(
                    f,
                    "boundary alias is empty at row {row} for canonical symbol {canonical_symbol}"
                )
            }
            Self::DuplicateAliasMapping {
                canonical_symbol,
                boundary_alias,
            } => {
                write!(
                    f,
                    "duplicate alias mapping for canonical symbol {canonical_symbol} and alias {boundary_alias}"
                )
            }
            Self::AmbiguousBoundaryAlias {
                boundary_alias,
                canonical_a,
                canonical_b,
            } => {
                write!(
                    f,
                    "boundary alias {boundary_alias} is ambiguous between canonical symbols {canonical_a} and {canonical_b}"
                )
            }
            Self::CanonicalSymbolNotFound { canonical_symbol } => {
                write!(f, "canonical symbol {canonical_symbol} not found")
            }
            Self::BoundaryAliasNotFound { boundary_alias } => {
                write!(f, "boundary alias {boundary_alias} not found")
            }
        }
    }
}

impl Error for SymbolAliasRegistryError {}

/// Canonical symbol alias registry with deterministic reverse lookups.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SymbolAliasRegistry {
    canonical_to_aliases: BTreeMap<String, Vec<String>>,
    alias_to_canonical: BTreeMap<String, String>,
}

impl SymbolAliasRegistry {
    /// Build a validated alias registry.
    ///
    /// Validation rules:
    /// - canonical and alias names must be non-empty;
    /// - duplicate `(canonical, alias)` rows are rejected;
    /// - one boundary alias cannot map to multiple canonical symbols.
    ///
    /// # Errors
    ///
    /// Returns typed `SymbolAliasRegistryError` variants for empty fields,
    /// duplicate rows, ambiguous aliases, or an empty registry.
    pub fn new(
        entries: impl IntoIterator<Item = SymbolAliasEntry>,
    ) -> Result<Self, SymbolAliasRegistryError> {
        let mut canonical_to_aliases: BTreeMap<String, Vec<String>> = BTreeMap::new();
        let mut alias_to_canonical: BTreeMap<String, String> = BTreeMap::new();
        let mut seen_pairs: BTreeSet<(String, String)> = BTreeSet::new();

        for (index, entry) in entries.into_iter().enumerate() {
            let row = index + 1;
            let canonical_symbol = entry.canonical_symbol.trim();
            let boundary_alias = entry.boundary_alias.trim();

            if canonical_symbol.is_empty() {
                return Err(SymbolAliasRegistryError::EmptyCanonicalSymbol { row });
            }

            if boundary_alias.is_empty() {
                return Err(SymbolAliasRegistryError::EmptyBoundaryAlias {
                    row,
                    canonical_symbol: canonical_symbol.to_string(),
                });
            }

            let canonical_key = canonical_symbol.to_string();
            let alias_key = boundary_alias.to_string();

            if !seen_pairs.insert((canonical_key.clone(), alias_key.clone())) {
                return Err(SymbolAliasRegistryError::DuplicateAliasMapping {
                    canonical_symbol: canonical_key,
                    boundary_alias: alias_key,
                });
            }

            if let Some(existing_canonical) = alias_to_canonical.get(alias_key.as_str()) {
                if existing_canonical != &canonical_key {
                    return Err(SymbolAliasRegistryError::AmbiguousBoundaryAlias {
                        boundary_alias: alias_key,
                        canonical_a: existing_canonical.clone(),
                        canonical_b: canonical_key,
                    });
                }
            } else {
                alias_to_canonical.insert(alias_key.clone(), canonical_key.clone());
            }

            canonical_to_aliases
                .entry(canonical_key)
                .or_default()
                .push(alias_key);
        }

        if canonical_to_aliases.is_empty() {
            return Err(SymbolAliasRegistryError::RegistryEmpty);
        }

        for aliases in canonical_to_aliases.values_mut() {
            aliases.sort();
        }

        Ok(Self {
            canonical_to_aliases,
            alias_to_canonical,
        })
    }

    /// Canonical WEPP/wepp-forest alias registry for ARCH03 baseline surfaces.
    ///
    /// # Errors
    ///
    /// Returns typed `SymbolAliasRegistryError` when the embedded baseline map
    /// fails registry validation.
    pub fn canonical_wepp_registry() -> Result<Self, SymbolAliasRegistryError> {
        Self::new([
            SymbolAliasEntry::new("runoff", "runoff_depth_m"),
            SymbolAliasEntry::new("runvol", "runoff_volume_m3"),
            SymbolAliasEntry::new("sbrunf", "subsurface_runoff_depth_m"),
            SymbolAliasEntry::new("drainq", "tile_drain_flow_m"),
            SymbolAliasEntry::new("sep", "deep_seepage_depth_m"),
            SymbolAliasEntry::new("st", "layer_storage_m"),
            SymbolAliasEntry::new("frzw", "layer_frozen_water_m"),
            SymbolAliasEntry::new("frozen", "layer_frozen_fraction"),
            SymbolAliasEntry::new("thetdr", "layer_theta_residual"),
            SymbolAliasEntry::new("thetfc", "layer_theta_field_capacity"),
            SymbolAliasEntry::new("dg", "layer_thickness_m"),
            SymbolAliasEntry::new("solthk", "soil_profile_depth_m"),
            SymbolAliasEntry::new("peakro", "peak_runoff_rate_m3s"),
            SymbolAliasEntry::new("watdur", "runoff_duration_s"),
        ])
    }

    #[must_use]
    pub fn canonical_symbols(&self) -> Vec<&str> {
        self.canonical_to_aliases
            .keys()
            .map(String::as_str)
            .collect()
    }

    /// Resolve aliases for a canonical symbol.
    ///
    /// # Errors
    ///
    /// Returns `SymbolAliasRegistryError::CanonicalSymbolNotFound` when the
    /// canonical symbol does not exist in the registry.
    pub fn aliases_for_canonical(
        &self,
        canonical_symbol: &str,
    ) -> Result<&[String], SymbolAliasRegistryError> {
        self.canonical_to_aliases
            .get(canonical_symbol)
            .map(Vec::as_slice)
            .ok_or_else(|| SymbolAliasRegistryError::CanonicalSymbolNotFound {
                canonical_symbol: canonical_symbol.to_string(),
            })
    }

    /// Resolve canonical symbol for a boundary alias.
    ///
    /// # Errors
    ///
    /// Returns `SymbolAliasRegistryError::BoundaryAliasNotFound` when the
    /// alias does not exist in the registry.
    pub fn canonical_for_boundary_alias(
        &self,
        boundary_alias: &str,
    ) -> Result<&str, SymbolAliasRegistryError> {
        self.alias_to_canonical
            .get(boundary_alias)
            .map(String::as_str)
            .ok_or_else(|| SymbolAliasRegistryError::BoundaryAliasNotFound {
                boundary_alias: boundary_alias.to_string(),
            })
    }

    #[must_use]
    pub fn entries(&self) -> Vec<SymbolAliasEntry> {
        let mut entries = Vec::new();

        for (canonical_symbol, aliases) in &self.canonical_to_aliases {
            for alias in aliases {
                entries.push(SymbolAliasEntry::new(
                    canonical_symbol.clone(),
                    alias.clone(),
                ));
            }
        }

        entries
    }
}
