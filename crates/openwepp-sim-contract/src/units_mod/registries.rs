use std::collections::{BTreeMap, BTreeSet};
use std::error::Error;
use std::fmt;

use super::boundary_catalog::canonical_boundary_unit_entries;
use super::output_catalog::canonical_output_unit_entries;
use super::types::{
    BoundaryUnitEntry, OutputUnitAuthority, OutputUnitEntry, TypedBoundaryRequirement,
};

/// Output-unit metadata registry validation and lookup errors.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum OutputUnitRegistryError {
    RegistryEmpty,
    EmptySchemaId {
        row: usize,
    },
    EmptyColumnName {
        row: usize,
        schema_id: Box<str>,
    },
    EmptyUnitLabel {
        row: usize,
        schema_id: Box<str>,
        column_name: Box<str>,
    },
    DuplicateOutputColumn {
        schema_id: Box<str>,
        column_name: Box<str>,
    },
    EmptyPublicationOnlyRationale {
        row: usize,
        schema_id: Box<str>,
        column_name: Box<str>,
    },
    EmptyPublicationOnlyContract {
        row: usize,
        schema_id: Box<str>,
        column_name: Box<str>,
    },
    BoundaryAliasNotFound {
        row: usize,
        schema_id: Box<str>,
        column_name: Box<str>,
        boundary_alias: Box<str>,
    },
    BoundaryUnitMismatch {
        row: usize,
        schema_id: Box<str>,
        column_name: Box<str>,
        boundary_alias: Box<str>,
        output_unit: Box<str>,
        boundary_unit: Box<str>,
    },
    OutputSchemaUnitMismatch {
        schema_id: Box<str>,
        column_name: Box<str>,
        schema_unit: Box<str>,
        registry_unit: Box<str>,
    },
    OutputColumnNotFound {
        schema_id: Box<str>,
        column_name: Box<str>,
    },
    BoundaryRegistry {
        detail: Box<str>,
    },
}

impl fmt::Display for OutputUnitRegistryError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::RegistryEmpty => f.write_str("output unit registry must contain entries"),
            Self::EmptySchemaId { row } => {
                write!(f, "output unit registry row {row} has empty schema_id")
            }
            Self::EmptyColumnName { row, schema_id } => write!(
                f,
                "output unit registry row {row} for schema {schema_id} has empty column_name"
            ),
            Self::EmptyUnitLabel {
                row,
                schema_id,
                column_name,
            } => write!(
                f,
                "output unit registry row {row} for {schema_id}.{column_name} has empty unit_label"
            ),
            Self::DuplicateOutputColumn {
                schema_id,
                column_name,
            } => write!(
                f,
                "duplicate output unit registry row for {schema_id}.{column_name}"
            ),
            Self::EmptyPublicationOnlyRationale {
                row,
                schema_id,
                column_name,
            } => write!(
                f,
                "publication-only output unit row {row} for {schema_id}.{column_name} lacks rationale"
            ),
            Self::EmptyPublicationOnlyContract {
                row,
                schema_id,
                column_name,
            } => write!(
                f,
                "publication-only output unit row {row} for {schema_id}.{column_name} lacks contract/invariant authority"
            ),
            Self::BoundaryAliasNotFound {
                row,
                schema_id,
                column_name,
                boundary_alias,
            } => write!(
                f,
                "output unit row {row} for {schema_id}.{column_name} references missing boundary alias {boundary_alias}"
            ),
            Self::BoundaryUnitMismatch {
                row,
                schema_id,
                column_name,
                boundary_alias,
                output_unit,
                boundary_unit,
            } => write!(
                f,
                "output unit row {row} for {schema_id}.{column_name} declares {output_unit}, but boundary alias {boundary_alias} declares {boundary_unit}"
            ),
            Self::OutputSchemaUnitMismatch {
                schema_id,
                column_name,
                schema_unit,
                registry_unit,
            } => write!(
                f,
                "output schema {schema_id}.{column_name} declares {schema_unit}, but output unit registry declares {registry_unit}"
            ),
            Self::OutputColumnNotFound {
                schema_id,
                column_name,
            } => write!(
                f,
                "output unit registry has no row for {schema_id}.{column_name}"
            ),
            Self::BoundaryRegistry { detail } => {
                write!(f, "boundary unit registry lookup failed: {detail}")
            }
        }
    }
}

/// Validate one fixed-unit output schema column against canonical output-unit
/// metadata and return the canonical registry label.
///
/// # Errors
///
/// Returns typed `OutputUnitRegistryError` when the canonical registry is
/// invalid, the schema/column pair is absent, or the schema unit differs from
/// registry authority.
pub fn validate_output_schema_unit(
    schema_id: &str,
    column_name: &str,
    schema_unit: &str,
) -> Result<&'static str, OutputUnitRegistryError> {
    let registry = OutputUnitRegistry::canonical_registry()?;
    let entry = registry.entry_for_output_column(schema_id, column_name)?;
    let registry_unit = entry.unit_label();
    if schema_unit != registry_unit {
        return Err(OutputUnitRegistryError::OutputSchemaUnitMismatch {
            schema_id: boxed_str(schema_id),
            column_name: boxed_str(column_name),
            schema_unit: boxed_str(schema_unit),
            registry_unit: boxed_str(registry_unit),
        });
    }
    Ok(registry_unit)
}

impl Error for OutputUnitRegistryError {}

/// Boundary-symbol unit registry validation and lookup errors.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum BoundaryUnitRegistryError {
    RegistryEmpty,
    EmptyCanonicalSymbol {
        row: usize,
    },
    EmptyBoundaryAlias {
        row: usize,
        canonical_symbol: String,
    },
    EmptyUnitLabel {
        row: usize,
        canonical_symbol: String,
    },
    EmptyProducerScope {
        row: usize,
        canonical_symbol: String,
    },
    EmptyConsumerScope {
        row: usize,
        canonical_symbol: String,
    },
    EmptyContractId {
        row: usize,
        canonical_symbol: String,
    },
    EmptyInvariantId {
        row: usize,
        canonical_symbol: String,
    },
    EmptyPublicationAlias {
        row: usize,
        canonical_symbol: String,
    },
    EmptyScalarException {
        row: usize,
        canonical_symbol: String,
    },
    DuplicateCanonicalSymbol {
        canonical_symbol: String,
    },
    DuplicateAliasMapping {
        canonical_symbol: String,
        boundary_alias: String,
    },
    DuplicatePublicationAlias {
        canonical_symbol: String,
        publication_alias: String,
    },
    AmbiguousBoundaryAlias {
        boundary_alias: String,
        canonical_a: String,
        canonical_b: String,
    },
    AmbiguousPublicationAlias {
        publication_alias: String,
        canonical_a: String,
        canonical_b: String,
    },
    InvalidBoundaryAliasTemplate {
        row: usize,
        canonical_symbol: String,
        boundary_alias: String,
        reason: String,
    },
    DimensionalSymbolMissingUnit {
        row: usize,
        canonical_symbol: String,
    },
    DimensionlessSymbolHasDimensionalUnit {
        row: usize,
        canonical_symbol: String,
        unit_label: String,
    },
    CanonicalSymbolNotFound {
        canonical_symbol: String,
    },
    BoundaryAliasNotFound {
        boundary_alias: String,
    },
    RequiredBoundaryAliasMissing {
        boundary_alias: String,
    },
}

impl fmt::Display for BoundaryUnitRegistryError {
    #[allow(clippy::too_many_lines)]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::RegistryEmpty => f.write_str("boundary unit registry must contain entries"),
            Self::EmptyCanonicalSymbol { row } => {
                write!(f, "canonical symbol is empty at row {row}")
            }
            Self::EmptyBoundaryAlias {
                row,
                canonical_symbol,
            } => write!(
                f,
                "boundary alias is empty at row {row} for canonical symbol {canonical_symbol}"
            ),
            Self::EmptyUnitLabel {
                row,
                canonical_symbol,
            } => write!(
                f,
                "unit label is empty at row {row} for canonical symbol {canonical_symbol}"
            ),
            Self::EmptyProducerScope {
                row,
                canonical_symbol,
            } => write!(
                f,
                "producer scope is empty at row {row} for canonical symbol {canonical_symbol}"
            ),
            Self::EmptyConsumerScope {
                row,
                canonical_symbol,
            } => write!(
                f,
                "consumer scope is empty at row {row} for canonical symbol {canonical_symbol}"
            ),
            Self::EmptyContractId {
                row,
                canonical_symbol,
            } => write!(
                f,
                "contract id is empty at row {row} for canonical symbol {canonical_symbol}"
            ),
            Self::EmptyInvariantId {
                row,
                canonical_symbol,
            } => write!(
                f,
                "invariant id is empty at row {row} for canonical symbol {canonical_symbol}"
            ),
            Self::EmptyPublicationAlias {
                row,
                canonical_symbol,
            } => write!(
                f,
                "publication alias is empty at row {row} for canonical symbol {canonical_symbol}"
            ),
            Self::EmptyScalarException {
                row,
                canonical_symbol,
            } => write!(
                f,
                "scalar exception is empty at row {row} for canonical symbol {canonical_symbol}"
            ),
            Self::DuplicateCanonicalSymbol { canonical_symbol } => {
                write!(f, "duplicate canonical symbol {canonical_symbol}")
            }
            Self::DuplicateAliasMapping {
                canonical_symbol,
                boundary_alias,
            } => write!(
                f,
                "duplicate boundary alias {boundary_alias} for canonical symbol {canonical_symbol}"
            ),
            Self::DuplicatePublicationAlias {
                canonical_symbol,
                publication_alias,
            } => write!(
                f,
                "duplicate publication alias {publication_alias} for canonical symbol {canonical_symbol}"
            ),
            Self::AmbiguousBoundaryAlias {
                boundary_alias,
                canonical_a,
                canonical_b,
            } => write!(
                f,
                "boundary alias {boundary_alias} is ambiguous between canonical symbols {canonical_a} and {canonical_b}"
            ),
            Self::AmbiguousPublicationAlias {
                publication_alias,
                canonical_a,
                canonical_b,
            } => write!(
                f,
                "publication alias {publication_alias} is ambiguous between canonical symbols {canonical_a} and {canonical_b}"
            ),
            Self::InvalidBoundaryAliasTemplate {
                row,
                canonical_symbol,
                boundary_alias,
                reason,
            } => write!(
                f,
                "invalid boundary alias template at row {row} for canonical symbol {canonical_symbol} and alias {boundary_alias}: {reason}"
            ),
            Self::DimensionalSymbolMissingUnit {
                row,
                canonical_symbol,
            } => write!(
                f,
                "dimensional canonical symbol {canonical_symbol} at row {row} has no unit"
            ),
            Self::DimensionlessSymbolHasDimensionalUnit {
                row,
                canonical_symbol,
                unit_label,
            } => write!(
                f,
                "dimensionless canonical symbol {canonical_symbol} at row {row} has dimensional unit {unit_label}"
            ),
            Self::CanonicalSymbolNotFound { canonical_symbol } => {
                write!(f, "canonical symbol {canonical_symbol} not found")
            }
            Self::BoundaryAliasNotFound { boundary_alias } => {
                write!(f, "boundary alias {boundary_alias} not found")
            }
            Self::RequiredBoundaryAliasMissing { boundary_alias } => {
                write!(
                    f,
                    "required boundary alias {boundary_alias} is missing from unit registry"
                )
            }
        }
    }
}

impl Error for BoundaryUnitRegistryError {}

#[derive(Debug, Clone, PartialEq, Eq)]
struct BoundaryUnitTemplateEntry {
    canonical_symbol: String,
    boundary_alias_template: String,
}

impl BoundaryUnitTemplateEntry {
    fn new(
        canonical_symbol: impl Into<String>,
        boundary_alias_template: impl Into<String>,
    ) -> Self {
        Self {
            canonical_symbol: canonical_symbol.into(),
            boundary_alias_template: boundary_alias_template.into(),
        }
    }
}

/// Validated machine-readable unit registry for runtime/publication symbols.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BoundaryUnitRegistry {
    canonical_to_entry: BTreeMap<String, BoundaryUnitEntry>,
    alias_to_canonical: BTreeMap<String, String>,
    template_aliases: Vec<BoundaryUnitTemplateEntry>,
}

impl BoundaryUnitRegistry {
    /// Build a validated unit registry.
    ///
    /// # Errors
    ///
    /// Returns typed `BoundaryUnitRegistryError` variants for empty required
    /// fields, missing dimensional units, duplicate aliases, ambiguous aliases,
    /// invalid templates, or an empty registry.
    pub fn new(
        entries: impl IntoIterator<Item = BoundaryUnitEntry>,
    ) -> Result<Self, BoundaryUnitRegistryError> {
        let mut canonical_to_entry: BTreeMap<String, BoundaryUnitEntry> = BTreeMap::new();
        let mut alias_to_canonical: BTreeMap<String, String> = BTreeMap::new();
        let mut publication_alias_to_canonical: BTreeMap<String, String> = BTreeMap::new();
        let mut template_to_canonical: BTreeMap<String, String> = BTreeMap::new();
        let mut template_aliases = Vec::new();
        let mut seen_pairs: BTreeSet<(String, String)> = BTreeSet::new();
        let mut seen_publication_pairs: BTreeSet<(String, String)> = BTreeSet::new();

        for (index, entry) in entries.into_iter().enumerate() {
            let row = index + 1;
            validate_entry(row, &entry)?;

            let canonical_key = entry.canonical_symbol().trim().to_string();
            if canonical_to_entry.contains_key(canonical_key.as_str()) {
                return Err(BoundaryUnitRegistryError::DuplicateCanonicalSymbol {
                    canonical_symbol: canonical_key,
                });
            }

            for boundary_alias in entry.boundary_aliases() {
                let alias_key = boundary_alias.trim().to_string();
                if !seen_pairs.insert((canonical_key.clone(), alias_key.clone())) {
                    return Err(BoundaryUnitRegistryError::DuplicateAliasMapping {
                        canonical_symbol: canonical_key,
                        boundary_alias: alias_key,
                    });
                }

                if alias_uses_template(alias_key.as_str()) {
                    validate_boundary_alias_template(
                        row,
                        canonical_key.as_str(),
                        alias_key.as_str(),
                    )?;
                    if let Some(existing_canonical) = template_to_canonical.get(alias_key.as_str())
                    {
                        if existing_canonical != &canonical_key {
                            return Err(BoundaryUnitRegistryError::AmbiguousBoundaryAlias {
                                boundary_alias: alias_key,
                                canonical_a: existing_canonical.clone(),
                                canonical_b: canonical_key,
                            });
                        }
                    } else {
                        template_to_canonical.insert(alias_key.clone(), canonical_key.clone());
                    }
                    template_aliases.push(BoundaryUnitTemplateEntry::new(
                        canonical_key.clone(),
                        alias_key,
                    ));
                } else if let Some(existing_canonical) = alias_to_canonical.get(alias_key.as_str())
                {
                    if existing_canonical != &canonical_key {
                        return Err(BoundaryUnitRegistryError::AmbiguousBoundaryAlias {
                            boundary_alias: alias_key,
                            canonical_a: existing_canonical.clone(),
                            canonical_b: canonical_key,
                        });
                    }
                } else {
                    alias_to_canonical.insert(alias_key, canonical_key.clone());
                }
            }

            for publication_alias in entry.publication_aliases() {
                let publication_key = publication_alias.trim().to_string();
                if !seen_publication_pairs.insert((canonical_key.clone(), publication_key.clone()))
                {
                    return Err(BoundaryUnitRegistryError::DuplicatePublicationAlias {
                        canonical_symbol: canonical_key,
                        publication_alias: publication_key,
                    });
                }
                if let Some(existing_canonical) =
                    publication_alias_to_canonical.get(publication_key.as_str())
                {
                    if existing_canonical != &canonical_key {
                        return Err(BoundaryUnitRegistryError::AmbiguousPublicationAlias {
                            publication_alias: publication_key,
                            canonical_a: existing_canonical.clone(),
                            canonical_b: canonical_key,
                        });
                    }
                } else {
                    publication_alias_to_canonical.insert(publication_key, canonical_key.clone());
                }
            }

            canonical_to_entry.insert(canonical_key, entry);
        }

        if canonical_to_entry.is_empty() {
            return Err(BoundaryUnitRegistryError::RegistryEmpty);
        }

        Ok(Self {
            canonical_to_entry,
            alias_to_canonical,
            template_aliases,
        })
    }

    /// Canonical boundary-symbol unit registry for HPHYS0274 touched scope.
    ///
    /// # Errors
    ///
    /// Returns typed `BoundaryUnitRegistryError` if embedded rows violate the
    /// unit registry schema.
    pub fn canonical_registry() -> Result<Self, BoundaryUnitRegistryError> {
        Self::new(canonical_boundary_unit_entries())
    }

    /// Return all canonical entries in deterministic canonical-symbol order.
    #[must_use]
    pub fn entries(&self) -> Vec<&BoundaryUnitEntry> {
        self.canonical_to_entry.values().collect()
    }

    /// Return one entry by canonical symbol.
    ///
    /// # Errors
    ///
    /// Returns `CanonicalSymbolNotFound` when the symbol is absent.
    pub fn entry_for_canonical(
        &self,
        canonical_symbol: &str,
    ) -> Result<&BoundaryUnitEntry, BoundaryUnitRegistryError> {
        self.canonical_to_entry
            .get(canonical_symbol)
            .ok_or_else(|| BoundaryUnitRegistryError::CanonicalSymbolNotFound {
                canonical_symbol: canonical_symbol.to_string(),
            })
    }

    /// Return one entry by runtime or publication boundary alias.
    ///
    /// # Errors
    ///
    /// Returns `BoundaryAliasNotFound` when no exact or template alias matches.
    pub fn entry_for_boundary_alias(
        &self,
        boundary_alias: &str,
    ) -> Result<&BoundaryUnitEntry, BoundaryUnitRegistryError> {
        if let Some(canonical_symbol) = self.alias_to_canonical.get(boundary_alias) {
            return self.entry_for_canonical(canonical_symbol);
        }

        let mut matching_canonicals: BTreeSet<&str> = BTreeSet::new();
        for template in &self.template_aliases {
            if template_matches_alias(template.boundary_alias_template.as_str(), boundary_alias) {
                matching_canonicals.insert(template.canonical_symbol.as_str());
            }
        }

        match (
            matching_canonicals.iter().next(),
            matching_canonicals.iter().nth(1),
        ) {
            (Some(canonical), None) => self.entry_for_canonical(canonical),
            (Some(canonical_a), Some(canonical_b)) => {
                Err(BoundaryUnitRegistryError::AmbiguousBoundaryAlias {
                    boundary_alias: boundary_alias.to_string(),
                    canonical_a: (*canonical_a).to_string(),
                    canonical_b: (*canonical_b).to_string(),
                })
            }
            _ => Err(BoundaryUnitRegistryError::BoundaryAliasNotFound {
                boundary_alias: boundary_alias.to_string(),
            }),
        }
    }

    /// Validate that all required aliases resolve to registry entries.
    ///
    /// # Errors
    ///
    /// Returns `RequiredBoundaryAliasMissing` for the first missing alias.
    pub fn require_boundary_aliases<'a>(
        &self,
        boundary_aliases: impl IntoIterator<Item = &'a str>,
    ) -> Result<(), BoundaryUnitRegistryError> {
        for boundary_alias in boundary_aliases {
            self.entry_for_boundary_alias(boundary_alias).map_err(|_| {
                BoundaryUnitRegistryError::RequiredBoundaryAliasMissing {
                    boundary_alias: boundary_alias.to_string(),
                }
            })?;
        }
        Ok(())
    }
}

/// Validated output-unit metadata registry keyed by `(schema_id, column_name)`.
#[derive(Debug, Clone)]
pub struct OutputUnitRegistry {
    output_to_entry: BTreeMap<(String, String), OutputUnitEntry>,
}

impl OutputUnitRegistry {
    /// Build a validated output-unit registry.
    ///
    /// # Errors
    ///
    /// Returns typed `OutputUnitRegistryError` variants for malformed rows,
    /// duplicate output columns, boundary-backed rows that do not resolve, or
    /// output/boundary unit mismatches.
    pub fn new(
        entries: impl IntoIterator<Item = OutputUnitEntry>,
    ) -> Result<Self, OutputUnitRegistryError> {
        let boundary_registry = BoundaryUnitRegistry::canonical_registry().map_err(|error| {
            OutputUnitRegistryError::BoundaryRegistry {
                detail: boxed_str(error.to_string()),
            }
        })?;
        let mut output_to_entry = BTreeMap::new();

        for (index, entry) in entries.into_iter().enumerate() {
            let row = index + 1;
            validate_output_unit_entry(row, &entry, &boundary_registry)?;

            let key = (
                entry.schema_id().trim().to_string(),
                entry.column_name().trim().to_string(),
            );
            if output_to_entry.contains_key(&key) {
                return Err(OutputUnitRegistryError::DuplicateOutputColumn {
                    schema_id: boxed_str(key.0),
                    column_name: boxed_str(key.1),
                });
            }
            output_to_entry.insert(key, entry);
        }

        if output_to_entry.is_empty() {
            return Err(OutputUnitRegistryError::RegistryEmpty);
        }

        Ok(Self { output_to_entry })
    }

    /// Canonical output unit registry for HPHYS0278 writer metadata.
    ///
    /// # Errors
    ///
    /// Returns typed `OutputUnitRegistryError` if embedded rows violate the
    /// output metadata registry schema.
    pub fn canonical_registry() -> Result<Self, OutputUnitRegistryError> {
        Self::new(canonical_output_unit_entries())
    }

    /// Return all canonical output entries in deterministic key order.
    #[must_use]
    pub fn entries(&self) -> Vec<&OutputUnitEntry> {
        self.output_to_entry.values().collect()
    }

    /// Return one output-unit entry by schema id and column name.
    ///
    /// # Errors
    ///
    /// Returns `OutputColumnNotFound` when the schema/column pair is absent.
    pub fn entry_for_output_column(
        &self,
        schema_id: &str,
        column_name: &str,
    ) -> Result<&OutputUnitEntry, OutputUnitRegistryError> {
        let schema_key = schema_id.trim().to_string();
        let column_key = column_name.trim().to_string();
        self.output_to_entry
            .get(&(schema_key.clone(), column_key.clone()))
            .ok_or(OutputUnitRegistryError::OutputColumnNotFound {
                schema_id: boxed_str(schema_key),
                column_name: boxed_str(column_key),
            })
    }
}

#[allow(clippy::too_many_lines)]
fn validate_entry(row: usize, entry: &BoundaryUnitEntry) -> Result<(), BoundaryUnitRegistryError> {
    let canonical_symbol = entry.canonical_symbol().trim();
    if canonical_symbol.is_empty() {
        return Err(BoundaryUnitRegistryError::EmptyCanonicalSymbol { row });
    }
    if entry.boundary_aliases().is_empty() {
        return Err(BoundaryUnitRegistryError::EmptyBoundaryAlias {
            row,
            canonical_symbol: canonical_symbol.to_string(),
        });
    }
    for boundary_alias in entry.boundary_aliases() {
        if boundary_alias.trim().is_empty() {
            return Err(BoundaryUnitRegistryError::EmptyBoundaryAlias {
                row,
                canonical_symbol: canonical_symbol.to_string(),
            });
        }
    }
    for publication_alias in entry.publication_aliases() {
        if publication_alias.trim().is_empty() {
            return Err(BoundaryUnitRegistryError::EmptyPublicationAlias {
                row,
                canonical_symbol: canonical_symbol.to_string(),
            });
        }
    }
    if entry.unit_label().trim().is_empty() {
        return Err(BoundaryUnitRegistryError::EmptyUnitLabel {
            row,
            canonical_symbol: canonical_symbol.to_string(),
        });
    }
    if entry.producer_scope().trim().is_empty() {
        return Err(BoundaryUnitRegistryError::EmptyProducerScope {
            row,
            canonical_symbol: canonical_symbol.to_string(),
        });
    }
    if entry.consumer_scope().trim().is_empty() {
        return Err(BoundaryUnitRegistryError::EmptyConsumerScope {
            row,
            canonical_symbol: canonical_symbol.to_string(),
        });
    }
    if entry.contract_id().trim().is_empty() {
        return Err(BoundaryUnitRegistryError::EmptyContractId {
            row,
            canonical_symbol: canonical_symbol.to_string(),
        });
    }
    if entry.invariant_id().trim().is_empty() {
        return Err(BoundaryUnitRegistryError::EmptyInvariantId {
            row,
            canonical_symbol: canonical_symbol.to_string(),
        });
    }
    if !entry.dimension_class().is_dimensionless() && entry.unit_label().trim() == "dimensionless" {
        return Err(BoundaryUnitRegistryError::DimensionalSymbolMissingUnit {
            row,
            canonical_symbol: canonical_symbol.to_string(),
        });
    }
    if entry.dimension_class().is_dimensionless()
        && !matches!(entry.unit_label().trim(), "dimensionless" | "count")
    {
        return Err(
            BoundaryUnitRegistryError::DimensionlessSymbolHasDimensionalUnit {
                row,
                canonical_symbol: canonical_symbol.to_string(),
                unit_label: entry.unit_label().to_string(),
            },
        );
    }
    if entry.typed_boundary() == TypedBoundaryRequirement::ScalarException {
        match entry.scalar_exception() {
            Some(reason) if !reason.trim().is_empty() => {}
            _ => {
                return Err(BoundaryUnitRegistryError::EmptyScalarException {
                    row,
                    canonical_symbol: canonical_symbol.to_string(),
                });
            }
        }
    }
    Ok(())
}

fn alias_uses_template(alias: &str) -> bool {
    alias.contains('{') || alias.contains('}')
}

fn boxed_str(value: impl Into<String>) -> Box<str> {
    value.into().into_boxed_str()
}

fn validate_output_unit_entry(
    row: usize,
    entry: &OutputUnitEntry,
    boundary_registry: &BoundaryUnitRegistry,
) -> Result<(), OutputUnitRegistryError> {
    let schema_id = entry.schema_id().trim();
    if schema_id.is_empty() {
        return Err(OutputUnitRegistryError::EmptySchemaId { row });
    }
    let column_name = entry.column_name().trim();
    if column_name.is_empty() {
        return Err(OutputUnitRegistryError::EmptyColumnName {
            row,
            schema_id: boxed_str(schema_id),
        });
    }
    let unit_label = entry.unit_label().trim();
    if unit_label.is_empty() {
        return Err(OutputUnitRegistryError::EmptyUnitLabel {
            row,
            schema_id: boxed_str(schema_id),
            column_name: boxed_str(column_name),
        });
    }

    match entry.authority() {
        OutputUnitAuthority::BoundaryRegistry { boundary_alias } => {
            let boundary_entry = boundary_registry
                .entry_for_boundary_alias(boundary_alias)
                .map_err(|_| OutputUnitRegistryError::BoundaryAliasNotFound {
                    row,
                    schema_id: boxed_str(schema_id),
                    column_name: boxed_str(column_name),
                    boundary_alias: boxed_str(boundary_alias),
                })?;
            let boundary_unit = boundary_entry.unit_label();
            if unit_label != boundary_unit {
                return Err(OutputUnitRegistryError::BoundaryUnitMismatch {
                    row,
                    schema_id: boxed_str(schema_id),
                    column_name: boxed_str(column_name),
                    boundary_alias: boxed_str(boundary_alias),
                    output_unit: boxed_str(unit_label),
                    boundary_unit: boxed_str(boundary_unit),
                });
            }
        }
        OutputUnitAuthority::PublicationOnly {
            rationale,
            contract_id,
            invariant_id,
        } => {
            if rationale.trim().is_empty() {
                return Err(OutputUnitRegistryError::EmptyPublicationOnlyRationale {
                    row,
                    schema_id: boxed_str(schema_id),
                    column_name: boxed_str(column_name),
                });
            }
            if contract_id.trim().is_empty() || invariant_id.trim().is_empty() {
                return Err(OutputUnitRegistryError::EmptyPublicationOnlyContract {
                    row,
                    schema_id: boxed_str(schema_id),
                    column_name: boxed_str(column_name),
                });
            }
        }
    }

    Ok(())
}

fn validate_boundary_alias_template(
    row: usize,
    canonical_symbol: &str,
    alias: &str,
) -> Result<(), BoundaryUnitRegistryError> {
    let mut cursor = alias;
    while let Some(open) = cursor.find('{') {
        let after_open = &cursor[open + 1..];
        let close = after_open.find('}').ok_or_else(|| {
            BoundaryUnitRegistryError::InvalidBoundaryAliasTemplate {
                row,
                canonical_symbol: canonical_symbol.to_string(),
                boundary_alias: alias.to_string(),
                reason: "missing closing '}'".to_string(),
            }
        })?;
        let token = &after_open[..close];
        if token != "idx4" && token != "ofe" {
            return Err(BoundaryUnitRegistryError::InvalidBoundaryAliasTemplate {
                row,
                canonical_symbol: canonical_symbol.to_string(),
                boundary_alias: alias.to_string(),
                reason: format!("unsupported token {{{token}}}"),
            });
        }
        cursor = &after_open[close + 1..];
    }
    if cursor.contains('}') {
        return Err(BoundaryUnitRegistryError::InvalidBoundaryAliasTemplate {
            row,
            canonical_symbol: canonical_symbol.to_string(),
            boundary_alias: alias.to_string(),
            reason: "unmatched closing '}'".to_string(),
        });
    }
    Ok(())
}

fn template_matches_alias(template: &str, alias: &str) -> bool {
    template_matches_alias_from(template.as_bytes(), alias.as_bytes())
}

fn template_matches_alias_from(template: &[u8], alias: &[u8]) -> bool {
    if template.is_empty() {
        return alias.is_empty();
    }
    if let Some(rest) = template.strip_prefix(b"{idx4}") {
        return alias.len() >= 4
            && alias[..4].iter().all(u8::is_ascii_digit)
            && template_matches_alias_from(rest, &alias[4..]);
    }
    if let Some(rest) = template.strip_prefix(b"{ofe}") {
        let consumed = alias
            .iter()
            .take_while(|byte| byte.is_ascii_digit())
            .count();
        return consumed > 0 && template_matches_alias_from(rest, &alias[consumed..]);
    }
    if alias.first() == template.first() {
        return template_matches_alias_from(&template[1..], &alias[1..]);
    }
    false
}
