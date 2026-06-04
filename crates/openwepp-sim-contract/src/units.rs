use std::collections::{BTreeMap, BTreeSet};
use std::error::Error;
use std::fmt;

/// Boundary-symbol physical dimension class.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Ord, PartialOrd)]
pub enum DimensionClass {
    Area,
    Count,
    Density,
    Depth,
    Direction,
    Fraction,
    HydraulicConductivity,
    RadiationDaily,
    RadiationHourly,
    Rate,
    Temperature,
    Time,
    Unitless,
    VolumetricWaterContent,
    WindSpeed,
}

impl DimensionClass {
    #[must_use]
    pub const fn is_dimensionless(self) -> bool {
        matches!(self, Self::Count | Self::Fraction | Self::Unitless)
    }
}

/// Boundary-symbol domain class.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Ord, PartialOrd)]
pub enum DomainClass {
    AnyFinite,
    CountNonNegative,
    DirectionDegrees,
    NonNegativeFinite,
    PositiveFinite,
    SignedFinite,
    UnitInterval,
}

/// Required typed-boundary posture for a symbol.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Ord, PartialOrd)]
pub enum TypedBoundaryRequirement {
    TypedAvailable,
    TypedRequired,
    ScalarException,
    FollowUpRequired,
}

/// Canonical unit metadata for one runtime/publication boundary symbol family.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Ord, PartialOrd)]
pub struct BoundaryUnitEntry {
    canonical_symbol: &'static str,
    boundary_aliases: &'static [&'static str],
    unit_label: &'static str,
    dimension_class: DimensionClass,
    domain_class: DomainClass,
    producer_scope: &'static str,
    consumer_scope: &'static str,
    contract_id: &'static str,
    invariant_id: &'static str,
    typed_boundary: TypedBoundaryRequirement,
    scalar_exception: Option<&'static str>,
    publication_aliases: &'static [&'static str],
}

impl BoundaryUnitEntry {
    #[allow(clippy::too_many_arguments)]
    #[must_use]
    pub const fn new(
        canonical_symbol: &'static str,
        boundary_aliases: &'static [&'static str],
        unit_label: &'static str,
        dimension_class: DimensionClass,
        domain_class: DomainClass,
        producer_scope: &'static str,
        consumer_scope: &'static str,
        contract_id: &'static str,
        invariant_id: &'static str,
        typed_boundary: TypedBoundaryRequirement,
        scalar_exception: Option<&'static str>,
        publication_aliases: &'static [&'static str],
    ) -> Self {
        Self {
            canonical_symbol,
            boundary_aliases,
            unit_label,
            dimension_class,
            domain_class,
            producer_scope,
            consumer_scope,
            contract_id,
            invariant_id,
            typed_boundary,
            scalar_exception,
            publication_aliases,
        }
    }

    #[must_use]
    pub const fn canonical_symbol(&self) -> &'static str {
        self.canonical_symbol
    }

    #[must_use]
    pub const fn boundary_aliases(&self) -> &'static [&'static str] {
        self.boundary_aliases
    }

    #[must_use]
    pub const fn unit_label(&self) -> &'static str {
        self.unit_label
    }

    #[must_use]
    pub const fn dimension_class(&self) -> DimensionClass {
        self.dimension_class
    }

    #[must_use]
    pub const fn domain_class(&self) -> DomainClass {
        self.domain_class
    }

    #[must_use]
    pub const fn producer_scope(&self) -> &'static str {
        self.producer_scope
    }

    #[must_use]
    pub const fn consumer_scope(&self) -> &'static str {
        self.consumer_scope
    }

    #[must_use]
    pub const fn contract_id(&self) -> &'static str {
        self.contract_id
    }

    #[must_use]
    pub const fn invariant_id(&self) -> &'static str {
        self.invariant_id
    }

    #[must_use]
    pub const fn typed_boundary(&self) -> TypedBoundaryRequirement {
        self.typed_boundary
    }

    #[must_use]
    pub const fn scalar_exception(&self) -> Option<&'static str> {
        self.scalar_exception
    }

    #[must_use]
    pub const fn publication_aliases(&self) -> &'static [&'static str] {
        self.publication_aliases
    }
}

/// Authority class for output publication unit metadata.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Ord, PartialOrd)]
pub enum OutputUnitAuthority {
    BoundaryRegistry {
        boundary_alias: &'static str,
    },
    PublicationOnly {
        rationale: &'static str,
        contract_id: &'static str,
        invariant_id: &'static str,
    },
}

/// Canonical unit metadata for one output schema column.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Ord, PartialOrd)]
pub struct OutputUnitEntry {
    schema_id: &'static str,
    column_name: &'static str,
    unit_label: &'static str,
    authority: OutputUnitAuthority,
}

impl OutputUnitEntry {
    #[must_use]
    pub const fn boundary_registry(
        schema_id: &'static str,
        column_name: &'static str,
        unit_label: &'static str,
        boundary_alias: &'static str,
    ) -> Self {
        Self {
            schema_id,
            column_name,
            unit_label,
            authority: OutputUnitAuthority::BoundaryRegistry { boundary_alias },
        }
    }

    #[must_use]
    pub const fn publication_only(
        schema_id: &'static str,
        column_name: &'static str,
        unit_label: &'static str,
        rationale: &'static str,
        contract_id: &'static str,
        invariant_id: &'static str,
    ) -> Self {
        Self {
            schema_id,
            column_name,
            unit_label,
            authority: OutputUnitAuthority::PublicationOnly {
                rationale,
                contract_id,
                invariant_id,
            },
        }
    }

    #[must_use]
    pub const fn schema_id(&self) -> &'static str {
        self.schema_id
    }

    #[must_use]
    pub const fn column_name(&self) -> &'static str {
        self.column_name
    }

    #[must_use]
    pub const fn unit_label(&self) -> &'static str {
        self.unit_label
    }

    #[must_use]
    pub const fn authority(&self) -> OutputUnitAuthority {
        self.authority
    }
}

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

            let canonical_key = entry.canonical_symbol.trim().to_string();
            if canonical_to_entry.contains_key(canonical_key.as_str()) {
                return Err(BoundaryUnitRegistryError::DuplicateCanonicalSymbol {
                    canonical_symbol: canonical_key,
                });
            }

            for boundary_alias in entry.boundary_aliases {
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

            for publication_alias in entry.publication_aliases {
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
                entry.schema_id.trim().to_string(),
                entry.column_name.trim().to_string(),
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

/// HPHYS0274 touched-scope aliases that must resolve in the registry gate.
#[allow(clippy::too_many_lines)]
#[must_use]
pub fn hphys0274_required_boundary_aliases() -> &'static [&'static str] {
    &[
        "P",
        "hillslope_wat.P",
        "prcp",
        "RM",
        "hillslope_wat.RM",
        "Q",
        "hillslope_wat.Q",
        "Ep",
        "hillslope_wat.Ep",
        "Es",
        "hillslope_wat.Es",
        "Er",
        "hillslope_wat.Er",
        "pmet.es_storage_return_m",
        "Dp",
        "hillslope_wat.Dp",
        "Pe",
        "UpStrmQ",
        "hillslope_wat.UpStrmQ",
        "SubRIn",
        "hillslope_wat.SubRIn",
        "latqcc",
        "hillslope_wat.latqcc",
        "Total-Soil",
        "hillslope_wat.Total-Soil",
        "frozwt",
        "hillslope_wat.frozwt",
        "Snow-Water",
        "hillslope_wat.Snow-Water",
        "QOFE",
        "hillslope_wat.QOFE",
        "Tile",
        "hillslope_wat.Tile",
        "Irr",
        "hillslope_wat.Irr",
        "Area",
        "hillslope_wat.Area",
        "SoilWaterTotal",
        "hillslope_wat.SoilWaterTotal",
        "ProfileDepth",
        "hillslope_wat.ProfileDepth",
        "wb13_profile_depth_mm",
        "ProfilePorosityCap",
        "hillslope_wat.ProfilePorosityCap",
        "wb13_profile_porosity_cap_mm",
        "ProfileFCStore",
        "hillslope_wat.ProfileFCStore",
        "wb13_profile_fc_store_mm",
        "wb13_profile_fc_tail_mm",
        "ProfileWPStore",
        "hillslope_wat.ProfileWPStore",
        "wb13_profile_wp_store_mm",
        "InterceptionStorage",
        "hillslope_wat.InterceptionStorage",
        "rad",
        "tmax",
        "tmin",
        "tdpt",
        "wind",
        "vwind",
        "stmdur",
        "stmstr",
        "timem_0001",
        "mxint",
        "avrint",
        "intsty_0001",
        "hs21_prcp",
        "hs21_rad",
        "hs21_tmax",
        "hs21_tmin",
        "hs21_tdpt",
        "hs21_vwind",
        "hs21_wind",
        "hs21_stmdur",
        "hs21_stmstr",
        "hs21_timem_0001",
        "hs21_mxint",
        "hs21_avrint",
        "hs21_intsty_0001",
        "winter.hourly.rad_mj_m2_0001",
        "winter.hourly.air_temp_c_0001",
        "winter.hourly.dewpoint_c_0001",
        "winter.hourly.wind_m_s_0001",
        "winter.hourly.cloud_fraction_0001",
        "snow.runtime_swe",
        "snow.runtime_depth_m",
        "snow.runtime_density_kg_m3",
        "snow.runtime_settle_day_count",
        "snow.hourly.rain_m_0001",
        "snow.hourly.rain_retained_m_0001",
        "snow.hourly.snowfall_m_0001",
        "snow.hourly.depth_before_m_0001",
        "snow.hourly.depth_available_m_0001",
        "snow.hourly.depth_after_m_0001",
        "snow.hourly.density_before_kg_m3_0001",
        "snow.hourly.density_after_kg_m3_0001",
        "snow.hourly.melt_m_0001",
        "snow.hourly.melt_raw_m_0001",
        "snow.hourly.melt_branch_active_0001",
        "snow.hourly.melt_amelt_in_0001",
        "snow.hourly.melt_bmelt_in_0001",
        "snow.hourly.melt_cmelt_in_0001",
        "snow.hourly.melt_dmelt_in_0001",
        "dg",
        "dg_0001",
        "ofe2_dg_0001",
        "solthk",
        "solthk_0001",
        "ofe2_solthk",
        "ofe2_solthk_0001",
        "thetdr",
        "thetdr_0001",
        "ofe2_thetdr_0001",
        "thetfc",
        "thetfc_0001",
        "ofe2_thetfc_0001",
        "por",
        "por_0001",
        "ofe2_por_0001",
        "ssc",
        "ssc_0001",
        "ofe2_ssc_0001",
        "wb11_nsl",
        "wb19_nsl",
        "nsl",
        "ofe2_nsl",
        "sat",
        "ofe2_sat",
    ]
}

#[allow(clippy::too_many_lines)]
#[must_use]
pub fn canonical_boundary_unit_entries() -> Vec<BoundaryUnitEntry> {
    use DimensionClass::{
        Area, Count, Density, Depth, Direction, Fraction, HydraulicConductivity, RadiationDaily,
        RadiationHourly, Rate, Temperature, Time, Unitless, VolumetricWaterContent, WindSpeed,
    };
    use DomainClass::{
        AnyFinite, CountNonNegative, DirectionDegrees, NonNegativeFinite, PositiveFinite,
        SignedFinite, UnitInterval,
    };
    use TypedBoundaryRequirement::{FollowUpRequired, ScalarException, TypedRequired};

    vec![
        BoundaryUnitEntry::new(
            "P",
            &["P", "hillslope_wat.P"],
            "mm",
            Depth,
            NonNegativeFinite,
            "openwepp-hillslope-orchestrator",
            "openwepp-hillslope-output",
            "SC-WATBAL-001",
            "SC-WATBAL-001#INV-WATBAL-054",
            FollowUpRequired,
            None,
            &["hillslope_wat.P:mm"],
        ),
        BoundaryUnitEntry::new(
            "prcp",
            &["prcp"],
            "m",
            Depth,
            NonNegativeFinite,
            "openwepp-climate-runtime-adapter",
            "openwepp-hillslope-orchestrator",
            "SC-CLIMATE-001",
            "SC-CLIMATE-001#INV-CLIMATE-007",
            TypedRequired,
            None,
            &[],
        ),
        BoundaryUnitEntry::new(
            "watershed_prcp",
            &["hs{ofe}_prcp"],
            "m",
            Depth,
            NonNegativeFinite,
            "openwepp-watershed-orchestrator",
            "openwepp-watershed-orchestrator",
            "SC-CLIMATE-001",
            "SC-CLIMATE-001#INV-CLIMATE-007",
            TypedRequired,
            None,
            &[],
        ),
        BoundaryUnitEntry::new(
            "RM",
            &["RM", "hillslope_wat.RM"],
            "mm",
            Depth,
            NonNegativeFinite,
            "openwepp-hillslope-orchestrator",
            "openwepp-hillslope-output",
            "SC-WATBAL-001",
            "SC-WATBAL-001#INV-WATBAL-054",
            FollowUpRequired,
            None,
            &["hillslope_wat.RM:mm"],
        ),
        BoundaryUnitEntry::new(
            "Q",
            &["Q", "hillslope_wat.Q"],
            "mm",
            Depth,
            NonNegativeFinite,
            "openwepp-hillslope-orchestrator",
            "openwepp-hillslope-output",
            "SC-WATBAL-001",
            "SC-WATBAL-001#INV-WATBAL-054",
            FollowUpRequired,
            None,
            &["hillslope_wat.Q:mm"],
        ),
        BoundaryUnitEntry::new(
            "Ep",
            &["Ep", "hillslope_wat.Ep"],
            "mm",
            Depth,
            NonNegativeFinite,
            "openwepp-hillslope-orchestrator",
            "openwepp-hillslope-output",
            "SC-EVAP-001",
            "SC-EVAP-001#INV-EVAP-016",
            FollowUpRequired,
            None,
            &["hillslope_wat.Ep:mm"],
        ),
        BoundaryUnitEntry::new(
            "Es",
            &["Es", "hillslope_wat.Es"],
            "mm",
            Depth,
            NonNegativeFinite,
            "openwepp-hillslope-orchestrator",
            "openwepp-hillslope-output",
            "SC-EVAP-001",
            "SC-EVAP-001#INV-EVAP-016",
            FollowUpRequired,
            None,
            &["hillslope_wat.Es:mm"],
        ),
        BoundaryUnitEntry::new(
            "Er",
            &["Er", "hillslope_wat.Er"],
            "mm",
            Depth,
            NonNegativeFinite,
            "openwepp-hillslope-orchestrator",
            "openwepp-hillslope-output",
            "SC-EVAP-001",
            "SC-EVAP-001#INV-EVAP-016",
            FollowUpRequired,
            None,
            &["hillslope_wat.Er:mm"],
        ),
        BoundaryUnitEntry::new(
            "pmet.es_storage_return_m",
            &["pmet.es_storage_return_m"],
            "m",
            Depth,
            NonNegativeFinite,
            "openwepp-runner",
            "openwepp-hillslope-orchestrator",
            "SC-EVAP-001",
            "SC-EVAP-001#INV-EVAP-025",
            TypedRequired,
            None,
            &[],
        ),
        BoundaryUnitEntry::new(
            "Dp",
            &["Dp", "hillslope_wat.Dp"],
            "mm",
            Depth,
            NonNegativeFinite,
            "openwepp-hillslope-orchestrator",
            "openwepp-hillslope-output",
            "SC-PERC-001",
            "SC-PERC-001#INV-PERC-014",
            FollowUpRequired,
            None,
            &["hillslope_wat.Dp:mm"],
        ),
        BoundaryUnitEntry::new(
            "Pe",
            &["Pe"],
            "mm",
            Depth,
            NonNegativeFinite,
            "openwepp-hillslope-orchestrator",
            "openwepp-hillslope-orchestrator",
            "SC-PERC-001",
            "SC-PERC-001#INV-PERC-014",
            FollowUpRequired,
            None,
            &[],
        ),
        BoundaryUnitEntry::new(
            "UpStrmQ",
            &["UpStrmQ", "hillslope_wat.UpStrmQ"],
            "mm",
            Depth,
            NonNegativeFinite,
            "openwepp-hillslope-orchestrator",
            "openwepp-hillslope-output",
            "SC-WATBAL-001",
            "SC-WATBAL-001#INV-WATBAL-054",
            FollowUpRequired,
            None,
            &["hillslope_wat.UpStrmQ:mm"],
        ),
        BoundaryUnitEntry::new(
            "SubRIn",
            &["SubRIn", "hillslope_wat.SubRIn"],
            "mm",
            Depth,
            NonNegativeFinite,
            "openwepp-hillslope-orchestrator",
            "openwepp-hillslope-output",
            "SC-SUBHYD-001",
            "SC-SUBHYD-001#INV-SUBHYD-025",
            FollowUpRequired,
            None,
            &["hillslope_wat.SubRIn:mm"],
        ),
        BoundaryUnitEntry::new(
            "latqcc",
            &["latqcc", "hillslope_wat.latqcc"],
            "mm",
            Depth,
            NonNegativeFinite,
            "openwepp-hillslope-orchestrator",
            "openwepp-hillslope-output",
            "SC-SUBHYD-001",
            "SC-SUBHYD-001#INV-SUBHYD-025",
            FollowUpRequired,
            None,
            &["hillslope_wat.latqcc:mm"],
        ),
        BoundaryUnitEntry::new(
            "Total-Soil",
            &["Total-Soil", "hillslope_wat.Total-Soil"],
            "mm",
            Depth,
            NonNegativeFinite,
            "openwepp-hillslope-orchestrator",
            "openwepp-hillslope-output",
            "SC-WATBAL-001",
            "SC-WATBAL-001#INV-WATBAL-054",
            FollowUpRequired,
            None,
            &["hillslope_wat.Total-Soil:mm"],
        ),
        BoundaryUnitEntry::new(
            "frozwt",
            &["frozwt", "hillslope_wat.frozwt"],
            "mm",
            Depth,
            NonNegativeFinite,
            "openwepp-hillslope-orchestrator",
            "openwepp-hillslope-output",
            "SC-WATBAL-001",
            "SC-WATBAL-001#INV-WATBAL-054",
            FollowUpRequired,
            None,
            &["hillslope_wat.frozwt:mm"],
        ),
        BoundaryUnitEntry::new(
            "Snow-Water",
            &["Snow-Water", "hillslope_wat.Snow-Water"],
            "mm",
            Depth,
            NonNegativeFinite,
            "openwepp-hillslope-orchestrator",
            "openwepp-hillslope-output",
            "SC-SNOWFREEZE-001",
            "SC-SNOWFREEZE-001#INV-SNOWFREEZE-014",
            FollowUpRequired,
            None,
            &["hillslope_wat.Snow-Water:mm"],
        ),
        BoundaryUnitEntry::new(
            "QOFE",
            &["QOFE", "hillslope_wat.QOFE"],
            "mm",
            Depth,
            NonNegativeFinite,
            "openwepp-hillslope-orchestrator",
            "openwepp-hillslope-output",
            "SC-WATBAL-001",
            "SC-WATBAL-001#INV-WATBAL-054",
            FollowUpRequired,
            None,
            &["hillslope_wat.QOFE:mm"],
        ),
        BoundaryUnitEntry::new(
            "Tile",
            &["Tile", "hillslope_wat.Tile"],
            "mm",
            Depth,
            NonNegativeFinite,
            "openwepp-hillslope-orchestrator",
            "openwepp-hillslope-output",
            "SC-SUBHYD-001",
            "SC-SUBHYD-001#INV-SUBHYD-025",
            FollowUpRequired,
            None,
            &["hillslope_wat.Tile:mm"],
        ),
        BoundaryUnitEntry::new(
            "Irr",
            &["Irr", "hillslope_wat.Irr"],
            "mm",
            Depth,
            NonNegativeFinite,
            "openwepp-hillslope-orchestrator",
            "openwepp-hillslope-output",
            "SC-WATBAL-001",
            "SC-WATBAL-001#INV-WATBAL-054",
            FollowUpRequired,
            None,
            &["hillslope_wat.Irr:mm"],
        ),
        BoundaryUnitEntry::new(
            "Area",
            &["Area", "hillslope_wat.Area"],
            "m^2",
            Area,
            PositiveFinite,
            "openwepp-hillslope-orchestrator",
            "openwepp-hillslope-output",
            "SC-WATBAL-001",
            "SC-WATBAL-001#INV-WATBAL-054",
            FollowUpRequired,
            None,
            &["hillslope_wat.Area:m^2"],
        ),
        BoundaryUnitEntry::new(
            "SoilWaterTotal",
            &["SoilWaterTotal", "hillslope_wat.SoilWaterTotal"],
            "mm",
            Depth,
            NonNegativeFinite,
            "openwepp-hillslope-orchestrator",
            "openwepp-hillslope-output",
            "SC-WATBAL-001",
            "SC-WATBAL-001#INV-WATBAL-054",
            FollowUpRequired,
            None,
            &["hillslope_wat.SoilWaterTotal:mm"],
        ),
        BoundaryUnitEntry::new(
            "ProfileDepth",
            &[
                "ProfileDepth",
                "hillslope_wat.ProfileDepth",
                "wb13_profile_depth_mm",
            ],
            "mm",
            Depth,
            PositiveFinite,
            "openwepp-hillslope-orchestrator",
            "openwepp-hillslope-output",
            "SC-SOIL-001",
            "SC-SOIL-001#INV-SOIL-011",
            FollowUpRequired,
            None,
            &["hillslope_wat.ProfileDepth:mm"],
        ),
        BoundaryUnitEntry::new(
            "ProfilePorosityCap",
            &[
                "ProfilePorosityCap",
                "hillslope_wat.ProfilePorosityCap",
                "wb13_profile_porosity_cap_mm",
            ],
            "mm",
            Depth,
            NonNegativeFinite,
            "openwepp-hillslope-orchestrator",
            "openwepp-hillslope-output",
            "SC-WATBAL-001",
            "SC-WATBAL-001#INV-WATBAL-054",
            FollowUpRequired,
            None,
            &["hillslope_wat.ProfilePorosityCap:mm"],
        ),
        BoundaryUnitEntry::new(
            "ProfileFCStore",
            &[
                "ProfileFCStore",
                "hillslope_wat.ProfileFCStore",
                "wb13_profile_fc_store_mm",
            ],
            "mm",
            Depth,
            NonNegativeFinite,
            "openwepp-hillslope-orchestrator",
            "openwepp-hillslope-output",
            "SC-WATBAL-001",
            "SC-WATBAL-001#INV-WATBAL-054",
            FollowUpRequired,
            None,
            &["hillslope_wat.ProfileFCStore:mm"],
        ),
        BoundaryUnitEntry::new(
            "ProfileWPStore",
            &[
                "ProfileWPStore",
                "hillslope_wat.ProfileWPStore",
                "wb13_profile_wp_store_mm",
            ],
            "mm",
            Depth,
            NonNegativeFinite,
            "openwepp-hillslope-orchestrator",
            "openwepp-hillslope-output",
            "SC-WATBAL-001",
            "SC-WATBAL-001#INV-WATBAL-054",
            FollowUpRequired,
            None,
            &["hillslope_wat.ProfileWPStore:mm"],
        ),
        BoundaryUnitEntry::new(
            "wb13_profile_fc_tail_mm",
            &["wb13_profile_fc_tail_mm"],
            "mm",
            Depth,
            NonNegativeFinite,
            "openwepp-runner",
            "openwepp-runner",
            "SC-WATBAL-001",
            "SC-WATBAL-001#INV-WATBAL-054",
            FollowUpRequired,
            None,
            &[],
        ),
        BoundaryUnitEntry::new(
            "InterceptionStorage",
            &["InterceptionStorage", "hillslope_wat.InterceptionStorage"],
            "mm",
            Depth,
            NonNegativeFinite,
            "openwepp-hillslope-orchestrator",
            "openwepp-hillslope-output",
            "SC-WATBAL-001",
            "SC-WATBAL-001#INV-WATBAL-054",
            FollowUpRequired,
            None,
            &["hillslope_wat.InterceptionStorage:mm"],
        ),
        BoundaryUnitEntry::new(
            "radly",
            &["rad"],
            "Ly d^-1",
            RadiationDaily,
            NonNegativeFinite,
            "openwepp-climate-runtime-adapter",
            "openwepp-hillslope-orchestrator",
            "SC-CLIMATE-001",
            "SC-CLIMATE-001#INV-CLIMATE-007",
            TypedRequired,
            None,
            &[],
        ),
        BoundaryUnitEntry::new(
            "watershed_radly",
            &["hs{ofe}_rad"],
            "Ly d^-1",
            RadiationDaily,
            NonNegativeFinite,
            "openwepp-watershed-orchestrator",
            "openwepp-watershed-orchestrator",
            "SC-CLIMATE-001",
            "SC-CLIMATE-001#INV-CLIMATE-007",
            TypedRequired,
            None,
            &[],
        ),
        BoundaryUnitEntry::new(
            "tmax",
            &["tmax"],
            "degC",
            Temperature,
            SignedFinite,
            "openwepp-climate-runtime-adapter",
            "openwepp-hillslope-orchestrator",
            "SC-CLIMATE-001",
            "SC-CLIMATE-001#INV-CLIMATE-007",
            TypedRequired,
            None,
            &[],
        ),
        BoundaryUnitEntry::new(
            "tmin",
            &["tmin"],
            "degC",
            Temperature,
            SignedFinite,
            "openwepp-climate-runtime-adapter",
            "openwepp-hillslope-orchestrator",
            "SC-CLIMATE-001",
            "SC-CLIMATE-001#INV-CLIMATE-007",
            TypedRequired,
            None,
            &[],
        ),
        BoundaryUnitEntry::new(
            "tdpt",
            &["tdpt"],
            "degC",
            Temperature,
            SignedFinite,
            "openwepp-climate-runtime-adapter",
            "openwepp-hillslope-orchestrator",
            "SC-CLIMATE-001",
            "SC-CLIMATE-001#INV-CLIMATE-007",
            TypedRequired,
            None,
            &[],
        ),
        BoundaryUnitEntry::new(
            "watershed_temperature",
            &["hs{ofe}_tmax", "hs{ofe}_tmin", "hs{ofe}_tdpt"],
            "degC",
            Temperature,
            SignedFinite,
            "openwepp-watershed-orchestrator",
            "openwepp-watershed-orchestrator",
            "SC-CLIMATE-001",
            "SC-CLIMATE-001#INV-CLIMATE-007",
            TypedRequired,
            None,
            &[],
        ),
        BoundaryUnitEntry::new(
            "vwind",
            &["vwind"],
            "m s^-1",
            WindSpeed,
            NonNegativeFinite,
            "openwepp-climate-runtime-adapter",
            "openwepp-hillslope-orchestrator",
            "SC-CLIMATE-001",
            "SC-CLIMATE-001#INV-CLIMATE-007",
            TypedRequired,
            None,
            &[],
        ),
        BoundaryUnitEntry::new(
            "watershed_vwind",
            &["hs{ofe}_vwind"],
            "m s^-1",
            WindSpeed,
            NonNegativeFinite,
            "openwepp-watershed-orchestrator",
            "openwepp-watershed-orchestrator",
            "SC-CLIMATE-001",
            "SC-CLIMATE-001#INV-CLIMATE-007",
            TypedRequired,
            None,
            &[],
        ),
        BoundaryUnitEntry::new(
            "wind_direction",
            &["wind"],
            "deg",
            Direction,
            DirectionDegrees,
            "openwepp-climate-runtime-adapter",
            "openwepp-hillslope-orchestrator",
            "SC-CLIMATE-001",
            "SC-CLIMATE-001#INV-CLIMATE-007",
            TypedRequired,
            None,
            &[],
        ),
        BoundaryUnitEntry::new(
            "watershed_wind_direction",
            &["hs{ofe}_wind"],
            "deg",
            Direction,
            DirectionDegrees,
            "openwepp-watershed-orchestrator",
            "openwepp-watershed-orchestrator",
            "SC-CLIMATE-001",
            "SC-CLIMATE-001#INV-CLIMATE-007",
            TypedRequired,
            None,
            &[],
        ),
        BoundaryUnitEntry::new(
            "stmdur",
            &["stmdur"],
            "s",
            Time,
            NonNegativeFinite,
            "openwepp-climate-runtime-adapter",
            "openwepp-hillslope-orchestrator",
            "SC-CLIMATE-001",
            "SC-CLIMATE-001#INV-CLIMATE-007",
            TypedRequired,
            None,
            &[],
        ),
        BoundaryUnitEntry::new(
            "watershed_stmdur",
            &["hs{ofe}_stmdur"],
            "s",
            Time,
            NonNegativeFinite,
            "openwepp-watershed-orchestrator",
            "openwepp-watershed-orchestrator",
            "SC-CLIMATE-001",
            "SC-CLIMATE-001#INV-CLIMATE-007",
            TypedRequired,
            None,
            &[],
        ),
        BoundaryUnitEntry::new(
            "stmstr",
            &["stmstr"],
            "h",
            Time,
            NonNegativeFinite,
            "openwepp-climate-runtime-adapter",
            "openwepp-hillslope-orchestrator",
            "SC-CLIMATE-001",
            "SC-CLIMATE-001#INV-CLIMATE-012",
            TypedRequired,
            None,
            &[],
        ),
        BoundaryUnitEntry::new(
            "watershed_stmstr",
            &["hs{ofe}_stmstr"],
            "h",
            Time,
            NonNegativeFinite,
            "openwepp-watershed-orchestrator",
            "openwepp-watershed-orchestrator",
            "SC-CLIMATE-001",
            "SC-CLIMATE-001#INV-CLIMATE-012",
            TypedRequired,
            None,
            &[],
        ),
        BoundaryUnitEntry::new(
            "timem",
            &["timem_{idx4}"],
            "s",
            Time,
            NonNegativeFinite,
            "openwepp-climate-runtime-adapter",
            "openwepp-hillslope-orchestrator",
            "SC-CLIMATE-001",
            "SC-CLIMATE-001#INV-CLIMATE-012",
            TypedRequired,
            None,
            &[],
        ),
        BoundaryUnitEntry::new(
            "watershed_timem",
            &["hs{ofe}_timem_{idx4}"],
            "s",
            Time,
            NonNegativeFinite,
            "openwepp-watershed-orchestrator",
            "openwepp-watershed-orchestrator",
            "SC-CLIMATE-001",
            "SC-CLIMATE-001#INV-CLIMATE-012",
            TypedRequired,
            None,
            &[],
        ),
        BoundaryUnitEntry::new(
            "mxint",
            &["mxint", "avrint", "intsty_{idx4}"],
            "m s^-1",
            Rate,
            NonNegativeFinite,
            "openwepp-climate-runtime-adapter",
            "openwepp-hillslope-orchestrator",
            "SC-CLIMATE-001",
            "SC-CLIMATE-001#INV-CLIMATE-007",
            TypedRequired,
            None,
            &[],
        ),
        BoundaryUnitEntry::new(
            "watershed_mxint",
            &["hs{ofe}_mxint", "hs{ofe}_avrint", "hs{ofe}_intsty_{idx4}"],
            "m s^-1",
            Rate,
            NonNegativeFinite,
            "openwepp-watershed-orchestrator",
            "openwepp-watershed-orchestrator",
            "SC-CLIMATE-001",
            "SC-CLIMATE-001#INV-CLIMATE-007",
            TypedRequired,
            None,
            &[],
        ),
        BoundaryUnitEntry::new(
            "winter_rad_hourly",
            &["winter.hourly.rad_mj_m2_{idx4}"],
            "MJ m^-2 h^-1",
            RadiationHourly,
            NonNegativeFinite,
            "openwepp-hillslope-orchestrator",
            "openwepp-hillslope-orchestrator",
            "SC-SNOWFREEZE-001",
            "SC-SNOWFREEZE-001#INV-SNOWFREEZE-014",
            TypedRequired,
            None,
            &[],
        ),
        BoundaryUnitEntry::new(
            "winter_air_temp_hourly",
            &["winter.hourly.air_temp_c_{idx4}"],
            "degC",
            Temperature,
            SignedFinite,
            "openwepp-hillslope-orchestrator",
            "openwepp-hillslope-orchestrator",
            "SC-SNOWFREEZE-001",
            "SC-SNOWFREEZE-001#INV-SNOWFREEZE-014",
            TypedRequired,
            None,
            &[],
        ),
        BoundaryUnitEntry::new(
            "winter_dewpoint_hourly",
            &["winter.hourly.dewpoint_c_{idx4}"],
            "degC",
            Temperature,
            SignedFinite,
            "openwepp-hillslope-orchestrator",
            "openwepp-hillslope-orchestrator",
            "SC-SNOWFREEZE-001",
            "SC-SNOWFREEZE-001#INV-SNOWFREEZE-014",
            TypedRequired,
            None,
            &[],
        ),
        BoundaryUnitEntry::new(
            "winter_wind_hourly",
            &["winter.hourly.wind_m_s_{idx4}"],
            "m s^-1",
            WindSpeed,
            NonNegativeFinite,
            "openwepp-hillslope-orchestrator",
            "openwepp-hillslope-orchestrator",
            "SC-SNOWFREEZE-001",
            "SC-SNOWFREEZE-001#INV-SNOWFREEZE-014",
            TypedRequired,
            None,
            &[],
        ),
        BoundaryUnitEntry::new(
            "winter_cloud_fraction_hourly",
            &["winter.hourly.cloud_fraction_{idx4}"],
            "dimensionless",
            Fraction,
            UnitInterval,
            "openwepp-hillslope-orchestrator",
            "openwepp-hillslope-orchestrator",
            "SC-SNOWFREEZE-001",
            "SC-SNOWFREEZE-001#INV-SNOWFREEZE-014",
            TypedRequired,
            None,
            &[],
        ),
        BoundaryUnitEntry::new(
            "snow_runtime_swe",
            &["snow.runtime_swe"],
            "m",
            Depth,
            NonNegativeFinite,
            "openwepp-hillslope-orchestrator",
            "openwepp-hillslope-orchestrator",
            "SC-SNOWFREEZE-001",
            "SC-SNOWFREEZE-001#INV-SNOWFREEZE-014",
            TypedRequired,
            None,
            &[],
        ),
        BoundaryUnitEntry::new(
            "snow_runtime_depth",
            &["snow.runtime_depth_m"],
            "m",
            Depth,
            NonNegativeFinite,
            "openwepp-hillslope-orchestrator",
            "openwepp-hillslope-orchestrator",
            "SC-SNOWFREEZE-001",
            "SC-SNOWFREEZE-001#INV-SNOWFREEZE-014",
            TypedRequired,
            None,
            &[],
        ),
        BoundaryUnitEntry::new(
            "snow_runtime_density",
            &["snow.runtime_density_kg_m3"],
            "kg m^-3",
            Density,
            NonNegativeFinite,
            "openwepp-hillslope-orchestrator",
            "openwepp-hillslope-orchestrator",
            "SC-SNOWFREEZE-001",
            "SC-SNOWFREEZE-001#INV-SNOWFREEZE-014",
            TypedRequired,
            None,
            &[],
        ),
        BoundaryUnitEntry::new(
            "snow_runtime_settle_day_count",
            &["snow.runtime_settle_day_count"],
            "count",
            Count,
            CountNonNegative,
            "openwepp-hillslope-orchestrator",
            "openwepp-hillslope-orchestrator",
            "SC-SNOWFREEZE-001",
            "SC-SNOWFREEZE-001#INV-SNOWFREEZE-014",
            ScalarException,
            Some("counter-like diagnostic remains scalar by governance definition"),
            &[],
        ),
        BoundaryUnitEntry::new(
            "snow_hourly_rain",
            &["snow.hourly.rain_m_{idx4}"],
            "m",
            Depth,
            NonNegativeFinite,
            "openwepp-hillslope-orchestrator",
            "openwepp-hillslope-orchestrator",
            "SC-SNOWFREEZE-001",
            "SC-SNOWFREEZE-001#INV-SNOWFREEZE-014",
            TypedRequired,
            None,
            &[],
        ),
        BoundaryUnitEntry::new(
            "snow_hourly_rain_retained",
            &["snow.hourly.rain_retained_m_{idx4}"],
            "m",
            Depth,
            NonNegativeFinite,
            "openwepp-hillslope-orchestrator",
            "openwepp-hillslope-orchestrator",
            "SC-SNOWFREEZE-001",
            "SC-SNOWFREEZE-001#INV-SNOWFREEZE-014",
            TypedRequired,
            None,
            &[],
        ),
        BoundaryUnitEntry::new(
            "snow_hourly_snowfall",
            &["snow.hourly.snowfall_m_{idx4}"],
            "m",
            Depth,
            NonNegativeFinite,
            "openwepp-hillslope-orchestrator",
            "openwepp-hillslope-orchestrator",
            "SC-SNOWFREEZE-001",
            "SC-SNOWFREEZE-001#INV-SNOWFREEZE-014",
            TypedRequired,
            None,
            &[],
        ),
        BoundaryUnitEntry::new(
            "snow_hourly_depth",
            &[
                "snow.hourly.depth_before_m_{idx4}",
                "snow.hourly.depth_available_m_{idx4}",
                "snow.hourly.depth_after_m_{idx4}",
            ],
            "m",
            Depth,
            NonNegativeFinite,
            "openwepp-hillslope-orchestrator",
            "openwepp-hillslope-orchestrator",
            "SC-SNOWFREEZE-001",
            "SC-SNOWFREEZE-001#INV-SNOWFREEZE-014",
            TypedRequired,
            None,
            &[],
        ),
        BoundaryUnitEntry::new(
            "snow_hourly_density",
            &[
                "snow.hourly.density_before_kg_m3_{idx4}",
                "snow.hourly.density_after_kg_m3_{idx4}",
            ],
            "kg m^-3",
            Density,
            NonNegativeFinite,
            "openwepp-hillslope-orchestrator",
            "openwepp-hillslope-orchestrator",
            "SC-SNOWFREEZE-001",
            "SC-SNOWFREEZE-001#INV-SNOWFREEZE-014",
            TypedRequired,
            None,
            &[],
        ),
        BoundaryUnitEntry::new(
            "snow_hourly_melt",
            &["snow.hourly.melt_m_{idx4}"],
            "m",
            Depth,
            NonNegativeFinite,
            "openwepp-hillslope-orchestrator",
            "openwepp-hillslope-orchestrator",
            "SC-SNOWFREEZE-001",
            "SC-SNOWFREEZE-001#INV-SNOWFREEZE-014",
            TypedRequired,
            None,
            &[],
        ),
        BoundaryUnitEntry::new(
            "snow_hourly_melt_raw",
            &["snow.hourly.melt_raw_m_{idx4}"],
            "m",
            Depth,
            SignedFinite,
            "openwepp-hillslope-orchestrator",
            "openwepp-hillslope-orchestrator",
            "SC-SNOWFREEZE-001",
            "SC-SNOWFREEZE-001#INV-SNOWFREEZE-014",
            FollowUpRequired,
            None,
            &[],
        ),
        BoundaryUnitEntry::new(
            "snow_hourly_melt_branch_active",
            &["snow.hourly.melt_branch_active_{idx4}"],
            "dimensionless",
            Fraction,
            UnitInterval,
            "openwepp-hillslope-orchestrator",
            "openwepp-hillslope-orchestrator",
            "SC-SNOWFREEZE-001",
            "SC-SNOWFREEZE-001#INV-SNOWFREEZE-014",
            TypedRequired,
            None,
            &[],
        ),
        BoundaryUnitEntry::new(
            "snow_hourly_melt_terms",
            &[
                "snow.hourly.melt_amelt_in_{idx4}",
                "snow.hourly.melt_bmelt_in_{idx4}",
                "snow.hourly.melt_cmelt_in_{idx4}",
                "snow.hourly.melt_dmelt_in_{idx4}",
            ],
            "in",
            Depth,
            SignedFinite,
            "openwepp-hillslope-orchestrator",
            "openwepp-hillslope-orchestrator",
            "SC-SNOWFREEZE-001",
            "SC-SNOWFREEZE-001#INV-SNOWFREEZE-014",
            FollowUpRequired,
            None,
            &[],
        ),
        BoundaryUnitEntry::new(
            "dg",
            &["dg", "dg_{idx4}", "ofe{ofe}_dg_{idx4}"],
            "m",
            Depth,
            PositiveFinite,
            "openwepp-hillslope-orchestrator",
            "openwepp-hillslope-orchestrator",
            "SC-SOIL-001",
            "SC-SOIL-001#INV-SOIL-011",
            FollowUpRequired,
            None,
            &[],
        ),
        BoundaryUnitEntry::new(
            "solthk",
            &[
                "solthk",
                "solthk_{idx4}",
                "ofe{ofe}_solthk",
                "ofe{ofe}_solthk_{idx4}",
            ],
            "m",
            Depth,
            PositiveFinite,
            "openwepp-hillslope-orchestrator",
            "openwepp-hillslope-orchestrator",
            "SC-SOIL-001",
            "SC-SOIL-001#INV-SOIL-011",
            FollowUpRequired,
            None,
            &[],
        ),
        BoundaryUnitEntry::new(
            "thetdr",
            &["thetdr", "thetdr_{idx4}", "ofe{ofe}_thetdr_{idx4}"],
            "m^3 m^-3",
            VolumetricWaterContent,
            UnitInterval,
            "openwepp-hillslope-orchestrator",
            "openwepp-hillslope-orchestrator",
            "SC-SOIL-001",
            "SC-SOIL-001#INV-SOIL-011",
            FollowUpRequired,
            None,
            &[],
        ),
        BoundaryUnitEntry::new(
            "thetfc",
            &["thetfc", "thetfc_{idx4}", "ofe{ofe}_thetfc_{idx4}"],
            "m^3 m^-3",
            VolumetricWaterContent,
            UnitInterval,
            "openwepp-hillslope-orchestrator",
            "openwepp-hillslope-orchestrator",
            "SC-SOIL-001",
            "SC-SOIL-001#INV-SOIL-011",
            FollowUpRequired,
            None,
            &[],
        ),
        BoundaryUnitEntry::new(
            "por",
            &["por", "por_{idx4}", "ofe{ofe}_por_{idx4}"],
            "m^3 m^-3",
            VolumetricWaterContent,
            UnitInterval,
            "openwepp-hillslope-orchestrator",
            "openwepp-hillslope-orchestrator",
            "SC-SOIL-001",
            "SC-SOIL-001#INV-SOIL-011",
            FollowUpRequired,
            None,
            &[],
        ),
        BoundaryUnitEntry::new(
            "ssc",
            &["ssc", "ssc_{idx4}", "ofe{ofe}_ssc_{idx4}"],
            "m s^-1",
            HydraulicConductivity,
            PositiveFinite,
            "openwepp-hillslope-orchestrator",
            "openwepp-hillslope-orchestrator",
            "SC-SOIL-001",
            "SC-SOIL-001#INV-SOIL-011",
            FollowUpRequired,
            None,
            &[],
        ),
        BoundaryUnitEntry::new(
            "nsl",
            &["wb11_nsl", "wb19_nsl", "nsl", "ofe{ofe}_nsl"],
            "count",
            Count,
            CountNonNegative,
            "openwepp-hillslope-orchestrator",
            "openwepp-hillslope-orchestrator",
            "SC-SOIL-001",
            "SC-SOIL-001#INV-SOIL-011",
            ScalarException,
            Some("layer-count control symbol is dimensionless by governance definition"),
            &[],
        ),
        BoundaryUnitEntry::new(
            "sat",
            &["sat", "ofe{ofe}_sat"],
            "dimensionless",
            Unitless,
            AnyFinite,
            "openwepp-hillslope-orchestrator",
            "openwepp-hillslope-orchestrator",
            "SC-SOIL-001",
            "SC-SOIL-001#INV-SOIL-011",
            ScalarException,
            Some("legacy saturation flag/control scalar is not a dimensional unit surface"),
            &[],
        ),
    ]
}

#[allow(clippy::too_many_lines)]
#[must_use]
pub fn canonical_output_unit_entries() -> Vec<OutputUnitEntry> {
    const PUB_ONLY: &str = "publication-only output column with no runtime boundary symbol in the HPHYS0278 touched registry";
    const VOLUME_CONVERSION: &str =
        "publication output converts registered water-balance depth to volume using output area";
    const AGG_DEPTH: &str =
        "publication output derives an aggregate depth from registered water-balance components";
    const SEDIMENT: &str = "publication-only sediment, pollutant, ash, or loss summary column outside runtime hydrology boundary registry";
    const SOIL_OUTPUT: &str =
        "publication-only soil diagnostic column outside runtime hydrology boundary registry";
    const SYSTEM_OUTPUT: &str = "publication-only watershed routing/output diagnostic outside runtime hydrology boundary registry";

    vec![
        OutputUnitEntry::boundary_registry("hillslope_wat", "P", "mm", "hillslope_wat.P"),
        OutputUnitEntry::boundary_registry("hillslope_wat", "RM", "mm", "hillslope_wat.RM"),
        OutputUnitEntry::boundary_registry("hillslope_wat", "Q", "mm", "hillslope_wat.Q"),
        OutputUnitEntry::boundary_registry("hillslope_wat", "Ep", "mm", "hillslope_wat.Ep"),
        OutputUnitEntry::boundary_registry("hillslope_wat", "Es", "mm", "hillslope_wat.Es"),
        OutputUnitEntry::boundary_registry("hillslope_wat", "Er", "mm", "hillslope_wat.Er"),
        OutputUnitEntry::boundary_registry("hillslope_wat", "Dp", "mm", "hillslope_wat.Dp"),
        OutputUnitEntry::boundary_registry(
            "hillslope_wat",
            "UpStrmQ",
            "mm",
            "hillslope_wat.UpStrmQ",
        ),
        OutputUnitEntry::boundary_registry("hillslope_wat", "SubRIn", "mm", "hillslope_wat.SubRIn"),
        OutputUnitEntry::boundary_registry("hillslope_wat", "latqcc", "mm", "hillslope_wat.latqcc"),
        OutputUnitEntry::boundary_registry(
            "hillslope_wat",
            "Total-Soil",
            "mm",
            "hillslope_wat.Total-Soil",
        ),
        OutputUnitEntry::boundary_registry("hillslope_wat", "frozwt", "mm", "hillslope_wat.frozwt"),
        OutputUnitEntry::boundary_registry(
            "hillslope_wat",
            "Snow-Water",
            "mm",
            "hillslope_wat.Snow-Water",
        ),
        OutputUnitEntry::boundary_registry("hillslope_wat", "QOFE", "mm", "hillslope_wat.QOFE"),
        OutputUnitEntry::boundary_registry("hillslope_wat", "Tile", "mm", "hillslope_wat.Tile"),
        OutputUnitEntry::boundary_registry("hillslope_wat", "Irr", "mm", "hillslope_wat.Irr"),
        OutputUnitEntry::boundary_registry("hillslope_wat", "Area", "m^2", "hillslope_wat.Area"),
        OutputUnitEntry::boundary_registry(
            "hillslope_wat",
            "SoilWaterTotal",
            "mm",
            "hillslope_wat.SoilWaterTotal",
        ),
        OutputUnitEntry::boundary_registry(
            "hillslope_wat",
            "ProfileDepth",
            "mm",
            "hillslope_wat.ProfileDepth",
        ),
        OutputUnitEntry::boundary_registry(
            "hillslope_wat",
            "ProfilePorosityCap",
            "mm",
            "hillslope_wat.ProfilePorosityCap",
        ),
        OutputUnitEntry::boundary_registry(
            "hillslope_wat",
            "ProfileFCStore",
            "mm",
            "hillslope_wat.ProfileFCStore",
        ),
        OutputUnitEntry::boundary_registry(
            "hillslope_wat",
            "ProfileWPStore",
            "mm",
            "hillslope_wat.ProfileWPStore",
        ),
        OutputUnitEntry::boundary_registry(
            "hillslope_wat",
            "InterceptionStorage",
            "mm",
            "hillslope_wat.InterceptionStorage",
        ),
        OutputUnitEntry::boundary_registry("watershed_ebe", "precip", "mm", "hillslope_wat.P"),
        OutputUnitEntry::publication_only(
            "watershed_ebe",
            "runoff_volume",
            "m^3",
            VOLUME_CONVERSION,
            "SC-WATBAL-001",
            "SC-WATBAL-001#INV-WATBAL-054",
        ),
        OutputUnitEntry::publication_only(
            "watershed_ebe",
            "peak_runoff",
            "m^3/s",
            SYSTEM_OUTPUT,
            "SC-SYSTEM-001",
            "SC-SYSTEM-001#INV-SYSTEM-001",
        ),
        OutputUnitEntry::publication_only(
            "watershed_ebe",
            "sediment_yield",
            "kg",
            SEDIMENT,
            "SC-SYSTEM-001",
            "SC-SYSTEM-001#INV-SYSTEM-001",
        ),
        OutputUnitEntry::publication_only(
            "watershed_ebe",
            "soluble_pollutant",
            "kg",
            SEDIMENT,
            "SC-SYSTEM-001",
            "SC-SYSTEM-001#INV-SYSTEM-001",
        ),
        OutputUnitEntry::publication_only(
            "watershed_ebe",
            "particulate_pollutant",
            "kg",
            SEDIMENT,
            "SC-SYSTEM-001",
            "SC-SYSTEM-001#INV-SYSTEM-001",
        ),
        OutputUnitEntry::publication_only(
            "watershed_ebe",
            "total_pollutant",
            "kg",
            SEDIMENT,
            "SC-SYSTEM-001",
            "SC-SYSTEM-001#INV-SYSTEM-001",
        ),
        OutputUnitEntry::publication_only(
            "watershed_chan_peak",
            "Time (s)",
            "s",
            SYSTEM_OUTPUT,
            "SC-SYSTEM-001",
            "SC-SYSTEM-001#INV-SYSTEM-001",
        ),
        OutputUnitEntry::publication_only(
            "watershed_chan_peak",
            "Peak_Discharge (m^3/s)",
            "m^3/s",
            SYSTEM_OUTPUT,
            "SC-SYSTEM-001",
            "SC-SYSTEM-001#INV-SYSTEM-001",
        ),
        OutputUnitEntry::publication_only(
            "watershed_chanwb",
            "Inflow (m^3)",
            "m^3",
            SYSTEM_OUTPUT,
            "SC-SYSTEM-001",
            "SC-SYSTEM-001#INV-SYSTEM-001",
        ),
        OutputUnitEntry::publication_only(
            "watershed_chanwb",
            "Outflow (m^3)",
            "m^3",
            SYSTEM_OUTPUT,
            "SC-SYSTEM-001",
            "SC-SYSTEM-001#INV-SYSTEM-001",
        ),
        OutputUnitEntry::publication_only(
            "watershed_chanwb",
            "Storage (m^3)",
            "m^3",
            SYSTEM_OUTPUT,
            "SC-SYSTEM-001",
            "SC-SYSTEM-001#INV-SYSTEM-001",
        ),
        OutputUnitEntry::publication_only(
            "watershed_chanwb",
            "Baseflow (m^3)",
            "m^3",
            SYSTEM_OUTPUT,
            "SC-SYSTEM-001",
            "SC-SYSTEM-001#INV-SYSTEM-001",
        ),
        OutputUnitEntry::publication_only(
            "watershed_chanwb",
            "Loss (m^3)",
            "m^3",
            SYSTEM_OUTPUT,
            "SC-SYSTEM-001",
            "SC-SYSTEM-001#INV-SYSTEM-001",
        ),
        OutputUnitEntry::publication_only(
            "watershed_chanwb",
            "Balance (m^3)",
            "m^3",
            SYSTEM_OUTPUT,
            "SC-SYSTEM-001",
            "SC-SYSTEM-001#INV-SYSTEM-001",
        ),
        OutputUnitEntry::boundary_registry("watershed_chnwb", "P (mm)", "mm", "hillslope_wat.P"),
        OutputUnitEntry::boundary_registry("watershed_chnwb", "RM (mm)", "mm", "hillslope_wat.RM"),
        OutputUnitEntry::boundary_registry("watershed_chnwb", "Q (mm)", "mm", "hillslope_wat.Q"),
        OutputUnitEntry::boundary_registry("watershed_chnwb", "Ep (mm)", "mm", "hillslope_wat.Ep"),
        OutputUnitEntry::boundary_registry("watershed_chnwb", "Es (mm)", "mm", "hillslope_wat.Es"),
        OutputUnitEntry::boundary_registry("watershed_chnwb", "Er (mm)", "mm", "hillslope_wat.Er"),
        OutputUnitEntry::boundary_registry("watershed_chnwb", "Dp (mm)", "mm", "hillslope_wat.Dp"),
        OutputUnitEntry::boundary_registry(
            "watershed_chnwb",
            "UpStrmQ (mm)",
            "mm",
            "hillslope_wat.UpStrmQ",
        ),
        OutputUnitEntry::boundary_registry(
            "watershed_chnwb",
            "SubRIn (mm)",
            "mm",
            "hillslope_wat.SubRIn",
        ),
        OutputUnitEntry::boundary_registry(
            "watershed_chnwb",
            "latqcc (mm)",
            "mm",
            "hillslope_wat.latqcc",
        ),
        OutputUnitEntry::boundary_registry(
            "watershed_chnwb",
            "Total Soil Water (mm)",
            "mm",
            "hillslope_wat.Total-Soil",
        ),
        OutputUnitEntry::boundary_registry(
            "watershed_chnwb",
            "frozwt (mm)",
            "mm",
            "hillslope_wat.frozwt",
        ),
        OutputUnitEntry::boundary_registry(
            "watershed_chnwb",
            "Snow Water (mm)",
            "mm",
            "hillslope_wat.Snow-Water",
        ),
        OutputUnitEntry::boundary_registry(
            "watershed_chnwb",
            "QOFE (mm)",
            "mm",
            "hillslope_wat.QOFE",
        ),
        OutputUnitEntry::boundary_registry(
            "watershed_chnwb",
            "Tile (mm)",
            "mm",
            "hillslope_wat.Tile",
        ),
        OutputUnitEntry::boundary_registry(
            "watershed_chnwb",
            "Irr (mm)",
            "mm",
            "hillslope_wat.Irr",
        ),
        OutputUnitEntry::publication_only(
            "watershed_chnwb",
            "Surf (mm)",
            "mm",
            PUB_ONLY,
            "SC-WATBAL-001",
            "SC-WATBAL-001#INV-WATBAL-054",
        ),
        OutputUnitEntry::publication_only(
            "watershed_chnwb",
            "Base (mm)",
            "mm",
            PUB_ONLY,
            "SC-WATBAL-001",
            "SC-WATBAL-001#INV-WATBAL-054",
        ),
        OutputUnitEntry::boundary_registry(
            "watershed_chnwb",
            "Area (m^2)",
            "m^2",
            "hillslope_wat.Area",
        ),
        OutputUnitEntry::publication_only(
            "watershed_soil",
            "Poros",
            "%",
            SOIL_OUTPUT,
            "SC-SOIL-001",
            "SC-SOIL-001#INV-SOIL-011",
        ),
        OutputUnitEntry::publication_only(
            "watershed_soil",
            "Keff",
            "mm/hr",
            SOIL_OUTPUT,
            "SC-SOIL-001",
            "SC-SOIL-001#INV-SOIL-011",
        ),
        OutputUnitEntry::publication_only(
            "watershed_soil",
            "Suct",
            "mm",
            SOIL_OUTPUT,
            "SC-SOIL-001",
            "SC-SOIL-001#INV-SOIL-011",
        ),
        OutputUnitEntry::publication_only(
            "watershed_soil",
            "FC",
            "mm/mm",
            SOIL_OUTPUT,
            "SC-SOIL-001",
            "SC-SOIL-001#INV-SOIL-011",
        ),
        OutputUnitEntry::publication_only(
            "watershed_soil",
            "WP",
            "mm/mm",
            SOIL_OUTPUT,
            "SC-SOIL-001",
            "SC-SOIL-001#INV-SOIL-011",
        ),
        OutputUnitEntry::publication_only(
            "watershed_soil",
            "Rough",
            "mm",
            SOIL_OUTPUT,
            "SC-SOIL-001",
            "SC-SOIL-001#INV-SOIL-011",
        ),
        OutputUnitEntry::publication_only(
            "watershed_soil",
            "Ki",
            "adjsmt",
            SOIL_OUTPUT,
            "SC-SOIL-001",
            "SC-SOIL-001#INV-SOIL-011",
        ),
        OutputUnitEntry::publication_only(
            "watershed_soil",
            "Kr",
            "adjsmt",
            SOIL_OUTPUT,
            "SC-SOIL-001",
            "SC-SOIL-001#INV-SOIL-011",
        ),
        OutputUnitEntry::publication_only(
            "watershed_soil",
            "Tauc",
            "adjsmt",
            SOIL_OUTPUT,
            "SC-SOIL-001",
            "SC-SOIL-001#INV-SOIL-011",
        ),
        OutputUnitEntry::publication_only(
            "watershed_soil",
            "Saturation",
            "frac",
            SOIL_OUTPUT,
            "SC-SOIL-001",
            "SC-SOIL-001#INV-SOIL-011",
        ),
        OutputUnitEntry::publication_only(
            "watershed_soil",
            "TSW",
            "mm",
            SOIL_OUTPUT,
            "SC-SOIL-001",
            "SC-SOIL-001#INV-SOIL-011",
        ),
        OutputUnitEntry::publication_only(
            "watershed_soil",
            "TSMF",
            "frac",
            SOIL_OUTPUT,
            "SC-SOIL-001",
            "SC-SOIL-001#INV-SOIL-011",
        ),
        OutputUnitEntry::publication_only(
            "watershed_totalwatsed3",
            "runvol",
            "m^3",
            SYSTEM_OUTPUT,
            "SC-WATBAL-001",
            "SC-WATBAL-001#INV-WATBAL-054",
        ),
        OutputUnitEntry::publication_only(
            "watershed_totalwatsed3",
            "sbrunv",
            "m^3",
            SYSTEM_OUTPUT,
            "SC-SUBHYD-001",
            "SC-SUBHYD-001#INV-SUBHYD-025",
        ),
        OutputUnitEntry::publication_only(
            "watershed_totalwatsed3",
            "tdet",
            "kg",
            SEDIMENT,
            "SC-SYSTEM-001",
            "SC-SYSTEM-001#INV-SYSTEM-001",
        ),
        OutputUnitEntry::publication_only(
            "watershed_totalwatsed3",
            "tdep",
            "kg",
            SEDIMENT,
            "SC-SYSTEM-001",
            "SC-SYSTEM-001#INV-SYSTEM-001",
        ),
        OutputUnitEntry::publication_only(
            "watershed_totalwatsed3",
            "seddep_1",
            "kg",
            SEDIMENT,
            "SC-SYSTEM-001",
            "SC-SYSTEM-001#INV-SYSTEM-001",
        ),
        OutputUnitEntry::publication_only(
            "watershed_totalwatsed3",
            "seddep_2",
            "kg",
            SEDIMENT,
            "SC-SYSTEM-001",
            "SC-SYSTEM-001#INV-SYSTEM-001",
        ),
        OutputUnitEntry::publication_only(
            "watershed_totalwatsed3",
            "seddep_3",
            "kg",
            SEDIMENT,
            "SC-SYSTEM-001",
            "SC-SYSTEM-001#INV-SYSTEM-001",
        ),
        OutputUnitEntry::publication_only(
            "watershed_totalwatsed3",
            "seddep_4",
            "kg",
            SEDIMENT,
            "SC-SYSTEM-001",
            "SC-SYSTEM-001#INV-SYSTEM-001",
        ),
        OutputUnitEntry::publication_only(
            "watershed_totalwatsed3",
            "seddep_5",
            "kg",
            SEDIMENT,
            "SC-SYSTEM-001",
            "SC-SYSTEM-001#INV-SYSTEM-001",
        ),
        OutputUnitEntry::publication_only(
            "watershed_totalwatsed3",
            "sed_del",
            "kg",
            SEDIMENT,
            "SC-SYSTEM-001",
            "SC-SYSTEM-001#INV-SYSTEM-001",
        ),
        OutputUnitEntry::publication_only(
            "watershed_totalwatsed3",
            "sed_vol_conc",
            "m^3/m^3",
            SEDIMENT,
            "SC-SYSTEM-001",
            "SC-SYSTEM-001#INV-SYSTEM-001",
        ),
        OutputUnitEntry::boundary_registry(
            "watershed_totalwatsed3",
            "Area",
            "m^2",
            "hillslope_wat.Area",
        ),
        OutputUnitEntry::publication_only(
            "watershed_totalwatsed3",
            "P",
            "m^3",
            VOLUME_CONVERSION,
            "SC-WATBAL-001",
            "SC-WATBAL-001#INV-WATBAL-054",
        ),
        OutputUnitEntry::publication_only(
            "watershed_totalwatsed3",
            "RM",
            "m^3",
            VOLUME_CONVERSION,
            "SC-WATBAL-001",
            "SC-WATBAL-001#INV-WATBAL-054",
        ),
        OutputUnitEntry::publication_only(
            "watershed_totalwatsed3",
            "Q",
            "m^3",
            VOLUME_CONVERSION,
            "SC-WATBAL-001",
            "SC-WATBAL-001#INV-WATBAL-054",
        ),
        OutputUnitEntry::publication_only(
            "watershed_totalwatsed3",
            "Dp",
            "m^3",
            VOLUME_CONVERSION,
            "SC-PERC-001",
            "SC-PERC-001#INV-PERC-014",
        ),
        OutputUnitEntry::publication_only(
            "watershed_totalwatsed3",
            "latqcc",
            "m^3",
            VOLUME_CONVERSION,
            "SC-SUBHYD-001",
            "SC-SUBHYD-001#INV-SUBHYD-025",
        ),
        OutputUnitEntry::publication_only(
            "watershed_totalwatsed3",
            "QOFE",
            "m^3",
            VOLUME_CONVERSION,
            "SC-WATBAL-001",
            "SC-WATBAL-001#INV-WATBAL-054",
        ),
        OutputUnitEntry::publication_only(
            "watershed_totalwatsed3",
            "Ep",
            "m^3",
            VOLUME_CONVERSION,
            "SC-EVAP-001",
            "SC-EVAP-001#INV-EVAP-016",
        ),
        OutputUnitEntry::publication_only(
            "watershed_totalwatsed3",
            "Es",
            "m^3",
            VOLUME_CONVERSION,
            "SC-EVAP-001",
            "SC-EVAP-001#INV-EVAP-016",
        ),
        OutputUnitEntry::publication_only(
            "watershed_totalwatsed3",
            "Er",
            "m^3",
            VOLUME_CONVERSION,
            "SC-EVAP-001",
            "SC-EVAP-001#INV-EVAP-016",
        ),
        OutputUnitEntry::boundary_registry(
            "watershed_totalwatsed3",
            "UpStrmQ",
            "mm",
            "hillslope_wat.UpStrmQ",
        ),
        OutputUnitEntry::boundary_registry(
            "watershed_totalwatsed3",
            "SubRIn",
            "mm",
            "hillslope_wat.SubRIn",
        ),
        OutputUnitEntry::boundary_registry(
            "watershed_totalwatsed3",
            "Total-Soil Water",
            "mm",
            "hillslope_wat.Total-Soil",
        ),
        OutputUnitEntry::boundary_registry(
            "watershed_totalwatsed3",
            "SoilWaterTotal",
            "mm",
            "hillslope_wat.SoilWaterTotal",
        ),
        OutputUnitEntry::boundary_registry(
            "watershed_totalwatsed3",
            "ProfileDepth",
            "mm",
            "hillslope_wat.ProfileDepth",
        ),
        OutputUnitEntry::boundary_registry(
            "watershed_totalwatsed3",
            "ProfilePorosityCap",
            "mm",
            "hillslope_wat.ProfilePorosityCap",
        ),
        OutputUnitEntry::boundary_registry(
            "watershed_totalwatsed3",
            "ProfileFCStore",
            "mm",
            "hillslope_wat.ProfileFCStore",
        ),
        OutputUnitEntry::boundary_registry(
            "watershed_totalwatsed3",
            "ProfileWPStore",
            "mm",
            "hillslope_wat.ProfileWPStore",
        ),
        OutputUnitEntry::boundary_registry(
            "watershed_totalwatsed3",
            "InterceptionStorage",
            "mm",
            "hillslope_wat.InterceptionStorage",
        ),
        OutputUnitEntry::publication_only(
            "watershed_totalwatsed3",
            "TSMF",
            "frac",
            SOIL_OUTPUT,
            "SC-SOIL-001",
            "SC-SOIL-001#INV-SOIL-011",
        ),
        OutputUnitEntry::boundary_registry(
            "watershed_totalwatsed3",
            "frozwt",
            "mm",
            "hillslope_wat.frozwt",
        ),
        OutputUnitEntry::boundary_registry(
            "watershed_totalwatsed3",
            "Snow-Water",
            "mm",
            "hillslope_wat.Snow-Water",
        ),
        OutputUnitEntry::publication_only(
            "watershed_totalwatsed3",
            "QRain",
            "mm",
            AGG_DEPTH,
            "SC-WATBAL-001",
            "SC-WATBAL-001#INV-WATBAL-054",
        ),
        OutputUnitEntry::publication_only(
            "watershed_totalwatsed3",
            "QSnow",
            "mm",
            AGG_DEPTH,
            "SC-SNOWFREEZE-001",
            "SC-SNOWFREEZE-001#INV-SNOWFREEZE-014",
        ),
        OutputUnitEntry::boundary_registry(
            "watershed_totalwatsed3",
            "Tile",
            "mm",
            "hillslope_wat.Tile",
        ),
        OutputUnitEntry::boundary_registry(
            "watershed_totalwatsed3",
            "Irr",
            "mm",
            "hillslope_wat.Irr",
        ),
        OutputUnitEntry::boundary_registry(
            "watershed_totalwatsed3",
            "Precipitation",
            "mm",
            "hillslope_wat.P",
        ),
        OutputUnitEntry::boundary_registry(
            "watershed_totalwatsed3",
            "Rain+Melt",
            "mm",
            "hillslope_wat.RM",
        ),
        OutputUnitEntry::boundary_registry(
            "watershed_totalwatsed3",
            "Percolation",
            "mm",
            "hillslope_wat.Dp",
        ),
        OutputUnitEntry::boundary_registry(
            "watershed_totalwatsed3",
            "Lateral Flow",
            "mm",
            "hillslope_wat.latqcc",
        ),
        OutputUnitEntry::boundary_registry(
            "watershed_totalwatsed3",
            "Runoff",
            "mm",
            "hillslope_wat.Q",
        ),
        OutputUnitEntry::boundary_registry(
            "watershed_totalwatsed3",
            "Transpiration",
            "mm",
            "hillslope_wat.Ep",
        ),
        OutputUnitEntry::publication_only(
            "watershed_totalwatsed3",
            "Evaporation",
            "mm",
            AGG_DEPTH,
            "SC-EVAP-001",
            "SC-EVAP-001#INV-EVAP-016",
        ),
        OutputUnitEntry::publication_only(
            "watershed_totalwatsed3",
            "ET",
            "mm",
            AGG_DEPTH,
            "SC-EVAP-001",
            "SC-EVAP-001#INV-EVAP-016",
        ),
        OutputUnitEntry::publication_only(
            "watershed_totalwatsed3",
            "Baseflow",
            "mm",
            SYSTEM_OUTPUT,
            "SC-SUBHYD-001",
            "SC-SUBHYD-001#INV-SUBHYD-025",
        ),
        OutputUnitEntry::publication_only(
            "watershed_totalwatsed3",
            "Aquifer losses",
            "mm",
            SYSTEM_OUTPUT,
            "SC-SUBHYD-001",
            "SC-SUBHYD-001#INV-SUBHYD-025",
        ),
        OutputUnitEntry::publication_only(
            "watershed_totalwatsed3",
            "Reservoir Volume",
            "mm",
            SYSTEM_OUTPUT,
            "SC-SUBHYD-001",
            "SC-SUBHYD-001#INV-SUBHYD-025",
        ),
        OutputUnitEntry::publication_only(
            "watershed_totalwatsed3",
            "Streamflow",
            "mm",
            SYSTEM_OUTPUT,
            "SC-SUBHYD-001",
            "SC-SUBHYD-001#INV-SUBHYD-025",
        ),
        OutputUnitEntry::publication_only(
            "watershed_totalwatsed3",
            "wind_transport",
            "tonne",
            SEDIMENT,
            "SC-SYSTEM-001",
            "SC-SYSTEM-001#INV-SYSTEM-001",
        ),
        OutputUnitEntry::publication_only(
            "watershed_totalwatsed3",
            "wind_transport_per_ha",
            "tonne/ha",
            SEDIMENT,
            "SC-SYSTEM-001",
            "SC-SYSTEM-001#INV-SYSTEM-001",
        ),
        OutputUnitEntry::publication_only(
            "watershed_totalwatsed3",
            "wind_transport_black",
            "tonne",
            SEDIMENT,
            "SC-SYSTEM-001",
            "SC-SYSTEM-001#INV-SYSTEM-001",
        ),
        OutputUnitEntry::publication_only(
            "watershed_totalwatsed3",
            "wind_transport_black_per_ha",
            "tonne/ha",
            SEDIMENT,
            "SC-SYSTEM-001",
            "SC-SYSTEM-001#INV-SYSTEM-001",
        ),
        OutputUnitEntry::publication_only(
            "watershed_totalwatsed3",
            "wind_transport_white",
            "tonne",
            SEDIMENT,
            "SC-SYSTEM-001",
            "SC-SYSTEM-001#INV-SYSTEM-001",
        ),
        OutputUnitEntry::publication_only(
            "watershed_totalwatsed3",
            "wind_transport_white_per_ha",
            "tonne/ha",
            SEDIMENT,
            "SC-SYSTEM-001",
            "SC-SYSTEM-001#INV-SYSTEM-001",
        ),
        OutputUnitEntry::publication_only(
            "watershed_totalwatsed3",
            "water_transport",
            "tonne",
            SEDIMENT,
            "SC-SYSTEM-001",
            "SC-SYSTEM-001#INV-SYSTEM-001",
        ),
        OutputUnitEntry::publication_only(
            "watershed_totalwatsed3",
            "water_transport_per_ha",
            "tonne/ha",
            SEDIMENT,
            "SC-SYSTEM-001",
            "SC-SYSTEM-001#INV-SYSTEM-001",
        ),
        OutputUnitEntry::publication_only(
            "watershed_totalwatsed3",
            "water_transport_black",
            "tonne",
            SEDIMENT,
            "SC-SYSTEM-001",
            "SC-SYSTEM-001#INV-SYSTEM-001",
        ),
        OutputUnitEntry::publication_only(
            "watershed_totalwatsed3",
            "water_transport_black_per_ha",
            "tonne/ha",
            SEDIMENT,
            "SC-SYSTEM-001",
            "SC-SYSTEM-001#INV-SYSTEM-001",
        ),
        OutputUnitEntry::publication_only(
            "watershed_totalwatsed3",
            "water_transport_white",
            "tonne",
            SEDIMENT,
            "SC-SYSTEM-001",
            "SC-SYSTEM-001#INV-SYSTEM-001",
        ),
        OutputUnitEntry::publication_only(
            "watershed_totalwatsed3",
            "water_transport_white_per_ha",
            "tonne/ha",
            SEDIMENT,
            "SC-SYSTEM-001",
            "SC-SYSTEM-001#INV-SYSTEM-001",
        ),
        OutputUnitEntry::publication_only(
            "watershed_totalwatsed3",
            "ash_transport",
            "tonne",
            SEDIMENT,
            "SC-SYSTEM-001",
            "SC-SYSTEM-001#INV-SYSTEM-001",
        ),
        OutputUnitEntry::publication_only(
            "watershed_totalwatsed3",
            "ash_transport_per_ha",
            "tonne/ha",
            SEDIMENT,
            "SC-SYSTEM-001",
            "SC-SYSTEM-001#INV-SYSTEM-001",
        ),
        OutputUnitEntry::publication_only(
            "watershed_totalwatsed3",
            "ash_transport_black",
            "tonne",
            SEDIMENT,
            "SC-SYSTEM-001",
            "SC-SYSTEM-001#INV-SYSTEM-001",
        ),
        OutputUnitEntry::publication_only(
            "watershed_totalwatsed3",
            "ash_transport_black_per_ha",
            "tonne/ha",
            SEDIMENT,
            "SC-SYSTEM-001",
            "SC-SYSTEM-001#INV-SYSTEM-001",
        ),
        OutputUnitEntry::publication_only(
            "watershed_totalwatsed3",
            "ash_transport_white",
            "tonne",
            SEDIMENT,
            "SC-SYSTEM-001",
            "SC-SYSTEM-001#INV-SYSTEM-001",
        ),
        OutputUnitEntry::publication_only(
            "watershed_totalwatsed3",
            "ash_transport_white_per_ha",
            "tonne/ha",
            SEDIMENT,
            "SC-SYSTEM-001",
            "SC-SYSTEM-001#INV-SYSTEM-001",
        ),
        OutputUnitEntry::publication_only(
            "watershed_totalwatsed3",
            "transportable_ash",
            "tonne",
            SEDIMENT,
            "SC-SYSTEM-001",
            "SC-SYSTEM-001#INV-SYSTEM-001",
        ),
        OutputUnitEntry::publication_only(
            "watershed_totalwatsed3",
            "transportable_ash_per_ha",
            "tonne/ha",
            SEDIMENT,
            "SC-SYSTEM-001",
            "SC-SYSTEM-001#INV-SYSTEM-001",
        ),
        OutputUnitEntry::publication_only(
            "watershed_totalwatsed3",
            "ash_vol_conc",
            "m^3/m^3",
            SEDIMENT,
            "SC-SYSTEM-001",
            "SC-SYSTEM-001#INV-SYSTEM-001",
        ),
        OutputUnitEntry::publication_only(
            "watershed_totalwatsed3",
            "sed+ash_vol_conc",
            "m^3/m^3",
            SEDIMENT,
            "SC-SYSTEM-001",
            "SC-SYSTEM-001#INV-SYSTEM-001",
        ),
        OutputUnitEntry::publication_only(
            "watershed_totalwatsed3",
            "ash_black_pct_by_vol",
            "percent",
            SEDIMENT,
            "SC-SYSTEM-001",
            "SC-SYSTEM-001#INV-SYSTEM-001",
        ),
        OutputUnitEntry::publication_only(
            "watershed_loss_all_years_hill",
            "Runoff Volume",
            "m^3",
            SYSTEM_OUTPUT,
            "SC-WATBAL-001",
            "SC-WATBAL-001#INV-WATBAL-054",
        ),
        OutputUnitEntry::publication_only(
            "watershed_loss_all_years_hill",
            "Subrunoff Volume",
            "m^3",
            SYSTEM_OUTPUT,
            "SC-SUBHYD-001",
            "SC-SUBHYD-001#INV-SUBHYD-025",
        ),
        OutputUnitEntry::publication_only(
            "watershed_loss_all_years_hill",
            "Baseflow Volume",
            "m^3",
            SYSTEM_OUTPUT,
            "SC-SUBHYD-001",
            "SC-SUBHYD-001#INV-SUBHYD-025",
        ),
        OutputUnitEntry::publication_only(
            "watershed_loss_all_years_hill",
            "Soil Loss",
            "kg",
            SEDIMENT,
            "SC-SYSTEM-001",
            "SC-SYSTEM-001#INV-SYSTEM-001",
        ),
        OutputUnitEntry::publication_only(
            "watershed_loss_all_years_hill",
            "Sediment Deposition",
            "kg",
            SEDIMENT,
            "SC-SYSTEM-001",
            "SC-SYSTEM-001#INV-SYSTEM-001",
        ),
        OutputUnitEntry::publication_only(
            "watershed_loss_all_years_hill",
            "Sediment Yield",
            "kg",
            SEDIMENT,
            "SC-SYSTEM-001",
            "SC-SYSTEM-001#INV-SYSTEM-001",
        ),
        OutputUnitEntry::publication_only(
            "watershed_loss_all_years_hill",
            "Solub. React. Pollutant",
            "kg",
            SEDIMENT,
            "SC-SYSTEM-001",
            "SC-SYSTEM-001#INV-SYSTEM-001",
        ),
        OutputUnitEntry::publication_only(
            "watershed_loss_all_years_hill",
            "Particulate Pollutant",
            "kg",
            SEDIMENT,
            "SC-SYSTEM-001",
            "SC-SYSTEM-001#INV-SYSTEM-001",
        ),
        OutputUnitEntry::publication_only(
            "watershed_loss_all_years_hill",
            "Total Pollutant",
            "kg",
            SEDIMENT,
            "SC-SYSTEM-001",
            "SC-SYSTEM-001#INV-SYSTEM-001",
        ),
        OutputUnitEntry::publication_only(
            "watershed_loss_average_hill",
            "Runoff Volume",
            "m^3",
            SYSTEM_OUTPUT,
            "SC-WATBAL-001",
            "SC-WATBAL-001#INV-WATBAL-054",
        ),
        OutputUnitEntry::publication_only(
            "watershed_loss_average_hill",
            "Subrunoff Volume",
            "m^3",
            SYSTEM_OUTPUT,
            "SC-SUBHYD-001",
            "SC-SUBHYD-001#INV-SUBHYD-025",
        ),
        OutputUnitEntry::publication_only(
            "watershed_loss_average_hill",
            "Baseflow Volume",
            "m^3",
            SYSTEM_OUTPUT,
            "SC-SUBHYD-001",
            "SC-SUBHYD-001#INV-SUBHYD-025",
        ),
        OutputUnitEntry::publication_only(
            "watershed_loss_average_hill",
            "Soil Loss",
            "kg",
            SEDIMENT,
            "SC-SYSTEM-001",
            "SC-SYSTEM-001#INV-SYSTEM-001",
        ),
        OutputUnitEntry::publication_only(
            "watershed_loss_average_hill",
            "Sediment Deposition",
            "kg",
            SEDIMENT,
            "SC-SYSTEM-001",
            "SC-SYSTEM-001#INV-SYSTEM-001",
        ),
        OutputUnitEntry::publication_only(
            "watershed_loss_average_hill",
            "Sediment Yield",
            "kg",
            SEDIMENT,
            "SC-SYSTEM-001",
            "SC-SYSTEM-001#INV-SYSTEM-001",
        ),
        OutputUnitEntry::publication_only(
            "watershed_loss_average_hill",
            "Hillslope Area",
            "ha",
            SYSTEM_OUTPUT,
            "SC-WATBAL-001",
            "SC-WATBAL-001#INV-WATBAL-054",
        ),
        OutputUnitEntry::publication_only(
            "watershed_loss_average_hill",
            "Solub. React. Pollutant",
            "kg",
            SEDIMENT,
            "SC-SYSTEM-001",
            "SC-SYSTEM-001#INV-SYSTEM-001",
        ),
        OutputUnitEntry::publication_only(
            "watershed_loss_average_hill",
            "Particulate Pollutant",
            "kg",
            SEDIMENT,
            "SC-SYSTEM-001",
            "SC-SYSTEM-001#INV-SYSTEM-001",
        ),
        OutputUnitEntry::publication_only(
            "watershed_loss_average_hill",
            "Total Pollutant",
            "kg",
            SEDIMENT,
            "SC-SYSTEM-001",
            "SC-SYSTEM-001#INV-SYSTEM-001",
        ),
        OutputUnitEntry::publication_only(
            "watershed_loss_all_years_chn",
            "Discharge Volume",
            "m^3",
            SYSTEM_OUTPUT,
            "SC-WATBAL-001",
            "SC-WATBAL-001#INV-WATBAL-054",
        ),
        OutputUnitEntry::publication_only(
            "watershed_loss_all_years_chn",
            "Sediment Yield",
            "tonne",
            SEDIMENT,
            "SC-SYSTEM-001",
            "SC-SYSTEM-001#INV-SYSTEM-001",
        ),
        OutputUnitEntry::publication_only(
            "watershed_loss_all_years_chn",
            "Soil Loss",
            "kg",
            SEDIMENT,
            "SC-SYSTEM-001",
            "SC-SYSTEM-001#INV-SYSTEM-001",
        ),
        OutputUnitEntry::publication_only(
            "watershed_loss_all_years_chn",
            "Upland Charge",
            "m^3",
            SYSTEM_OUTPUT,
            "SC-SUBHYD-001",
            "SC-SUBHYD-001#INV-SUBHYD-025",
        ),
        OutputUnitEntry::publication_only(
            "watershed_loss_all_years_chn",
            "Subsuface Flow Volume",
            "m^3",
            SYSTEM_OUTPUT,
            "SC-SUBHYD-001",
            "SC-SUBHYD-001#INV-SUBHYD-025",
        ),
        OutputUnitEntry::publication_only(
            "watershed_loss_all_years_chn",
            "Solub. React. Pollutant",
            "kg",
            SEDIMENT,
            "SC-SYSTEM-001",
            "SC-SYSTEM-001#INV-SYSTEM-001",
        ),
        OutputUnitEntry::publication_only(
            "watershed_loss_all_years_chn",
            "Particulate Pollutant",
            "kg",
            SEDIMENT,
            "SC-SYSTEM-001",
            "SC-SYSTEM-001#INV-SYSTEM-001",
        ),
        OutputUnitEntry::publication_only(
            "watershed_loss_all_years_chn",
            "Total Pollutant",
            "kg",
            SEDIMENT,
            "SC-SYSTEM-001",
            "SC-SYSTEM-001#INV-SYSTEM-001",
        ),
        OutputUnitEntry::publication_only(
            "watershed_loss_average_chn",
            "Discharge Volume",
            "m^3",
            SYSTEM_OUTPUT,
            "SC-WATBAL-001",
            "SC-WATBAL-001#INV-WATBAL-054",
        ),
        OutputUnitEntry::publication_only(
            "watershed_loss_average_chn",
            "Sediment Yield",
            "tonne",
            SEDIMENT,
            "SC-SYSTEM-001",
            "SC-SYSTEM-001#INV-SYSTEM-001",
        ),
        OutputUnitEntry::publication_only(
            "watershed_loss_average_chn",
            "Soil Loss",
            "kg",
            SEDIMENT,
            "SC-SYSTEM-001",
            "SC-SYSTEM-001#INV-SYSTEM-001",
        ),
        OutputUnitEntry::publication_only(
            "watershed_loss_average_chn",
            "Upland Charge",
            "m^3",
            SYSTEM_OUTPUT,
            "SC-SUBHYD-001",
            "SC-SUBHYD-001#INV-SUBHYD-025",
        ),
        OutputUnitEntry::publication_only(
            "watershed_loss_average_chn",
            "Subsuface Flow Volume",
            "m^3",
            SYSTEM_OUTPUT,
            "SC-SUBHYD-001",
            "SC-SUBHYD-001#INV-SUBHYD-025",
        ),
        OutputUnitEntry::publication_only(
            "watershed_loss_average_chn",
            "Contributing Area",
            "ha",
            SYSTEM_OUTPUT,
            "SC-WATBAL-001",
            "SC-WATBAL-001#INV-WATBAL-054",
        ),
        OutputUnitEntry::publication_only(
            "watershed_loss_average_chn",
            "Solub. React. Pollutant",
            "kg",
            SEDIMENT,
            "SC-SYSTEM-001",
            "SC-SYSTEM-001#INV-SYSTEM-001",
        ),
        OutputUnitEntry::publication_only(
            "watershed_loss_average_chn",
            "Particulate Pollutant",
            "kg",
            SEDIMENT,
            "SC-SYSTEM-001",
            "SC-SYSTEM-001#INV-SYSTEM-001",
        ),
        OutputUnitEntry::publication_only(
            "watershed_loss_average_chn",
            "Total Pollutant",
            "kg",
            SEDIMENT,
            "SC-SYSTEM-001",
            "SC-SYSTEM-001#INV-SYSTEM-001",
        ),
        OutputUnitEntry::publication_only(
            "watershed_loss_all_years_out",
            "value",
            "row_field:units",
            "dynamic key/value publication column; physical unit is stored in the sibling row-level units column",
            "SC-SYSTEM-001",
            "SC-SYSTEM-001#INV-SYSTEM-001",
        ),
        OutputUnitEntry::publication_only(
            "watershed_loss_average_out",
            "value",
            "row_field:units",
            "dynamic key/value publication column; physical unit is stored in the sibling row-level units column",
            "SC-SYSTEM-001",
            "SC-SYSTEM-001#INV-SYSTEM-001",
        ),
        OutputUnitEntry::publication_only(
            "watershed_loss_all_years_class",
            "Diameter",
            "mm",
            SEDIMENT,
            "SC-SYSTEM-001",
            "SC-SYSTEM-001#INV-SYSTEM-001",
        ),
        OutputUnitEntry::publication_only(
            "watershed_loss_all_years_class",
            "Pct Sand",
            "%",
            SEDIMENT,
            "SC-SYSTEM-001",
            "SC-SYSTEM-001#INV-SYSTEM-001",
        ),
        OutputUnitEntry::publication_only(
            "watershed_loss_all_years_class",
            "Pct Silt",
            "%",
            SEDIMENT,
            "SC-SYSTEM-001",
            "SC-SYSTEM-001#INV-SYSTEM-001",
        ),
        OutputUnitEntry::publication_only(
            "watershed_loss_all_years_class",
            "Pct Clay",
            "%",
            SEDIMENT,
            "SC-SYSTEM-001",
            "SC-SYSTEM-001#INV-SYSTEM-001",
        ),
        OutputUnitEntry::publication_only(
            "watershed_loss_all_years_class",
            "Pct OM",
            "%",
            SEDIMENT,
            "SC-SYSTEM-001",
            "SC-SYSTEM-001#INV-SYSTEM-001",
        ),
        OutputUnitEntry::publication_only(
            "watershed_loss_all_years_class",
            "Fraction In Flow Exiting",
            "dimensionless",
            SEDIMENT,
            "SC-SYSTEM-001",
            "SC-SYSTEM-001#INV-SYSTEM-001",
        ),
        OutputUnitEntry::publication_only(
            "watershed_loss_average_class",
            "Diameter",
            "mm",
            SEDIMENT,
            "SC-SYSTEM-001",
            "SC-SYSTEM-001#INV-SYSTEM-001",
        ),
        OutputUnitEntry::publication_only(
            "watershed_loss_average_class",
            "Pct Sand",
            "%",
            SEDIMENT,
            "SC-SYSTEM-001",
            "SC-SYSTEM-001#INV-SYSTEM-001",
        ),
        OutputUnitEntry::publication_only(
            "watershed_loss_average_class",
            "Pct Silt",
            "%",
            SEDIMENT,
            "SC-SYSTEM-001",
            "SC-SYSTEM-001#INV-SYSTEM-001",
        ),
        OutputUnitEntry::publication_only(
            "watershed_loss_average_class",
            "Pct Clay",
            "%",
            SEDIMENT,
            "SC-SYSTEM-001",
            "SC-SYSTEM-001#INV-SYSTEM-001",
        ),
        OutputUnitEntry::publication_only(
            "watershed_loss_average_class",
            "Pct OM",
            "%",
            SEDIMENT,
            "SC-SYSTEM-001",
            "SC-SYSTEM-001#INV-SYSTEM-001",
        ),
        OutputUnitEntry::publication_only(
            "watershed_loss_average_class",
            "Fraction In Flow Exiting",
            "dimensionless",
            SEDIMENT,
            "SC-SYSTEM-001",
            "SC-SYSTEM-001#INV-SYSTEM-001",
        ),
    ]
}

fn validate_entry(row: usize, entry: &BoundaryUnitEntry) -> Result<(), BoundaryUnitRegistryError> {
    let canonical_symbol = entry.canonical_symbol.trim();
    if canonical_symbol.is_empty() {
        return Err(BoundaryUnitRegistryError::EmptyCanonicalSymbol { row });
    }
    if entry.boundary_aliases.is_empty() {
        return Err(BoundaryUnitRegistryError::EmptyBoundaryAlias {
            row,
            canonical_symbol: canonical_symbol.to_string(),
        });
    }
    for boundary_alias in entry.boundary_aliases {
        if boundary_alias.trim().is_empty() {
            return Err(BoundaryUnitRegistryError::EmptyBoundaryAlias {
                row,
                canonical_symbol: canonical_symbol.to_string(),
            });
        }
    }
    for publication_alias in entry.publication_aliases {
        if publication_alias.trim().is_empty() {
            return Err(BoundaryUnitRegistryError::EmptyPublicationAlias {
                row,
                canonical_symbol: canonical_symbol.to_string(),
            });
        }
    }
    if entry.unit_label.trim().is_empty() {
        return Err(BoundaryUnitRegistryError::EmptyUnitLabel {
            row,
            canonical_symbol: canonical_symbol.to_string(),
        });
    }
    if entry.producer_scope.trim().is_empty() {
        return Err(BoundaryUnitRegistryError::EmptyProducerScope {
            row,
            canonical_symbol: canonical_symbol.to_string(),
        });
    }
    if entry.consumer_scope.trim().is_empty() {
        return Err(BoundaryUnitRegistryError::EmptyConsumerScope {
            row,
            canonical_symbol: canonical_symbol.to_string(),
        });
    }
    if entry.contract_id.trim().is_empty() {
        return Err(BoundaryUnitRegistryError::EmptyContractId {
            row,
            canonical_symbol: canonical_symbol.to_string(),
        });
    }
    if entry.invariant_id.trim().is_empty() {
        return Err(BoundaryUnitRegistryError::EmptyInvariantId {
            row,
            canonical_symbol: canonical_symbol.to_string(),
        });
    }
    if !entry.dimension_class.is_dimensionless() && entry.unit_label.trim() == "dimensionless" {
        return Err(BoundaryUnitRegistryError::DimensionalSymbolMissingUnit {
            row,
            canonical_symbol: canonical_symbol.to_string(),
        });
    }
    if entry.dimension_class.is_dimensionless()
        && !matches!(entry.unit_label.trim(), "dimensionless" | "count")
    {
        return Err(
            BoundaryUnitRegistryError::DimensionlessSymbolHasDimensionalUnit {
                row,
                canonical_symbol: canonical_symbol.to_string(),
                unit_label: entry.unit_label.to_string(),
            },
        );
    }
    if entry.typed_boundary == TypedBoundaryRequirement::ScalarException {
        match entry.scalar_exception {
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
    let schema_id = entry.schema_id.trim();
    if schema_id.is_empty() {
        return Err(OutputUnitRegistryError::EmptySchemaId { row });
    }
    let column_name = entry.column_name.trim();
    if column_name.is_empty() {
        return Err(OutputUnitRegistryError::EmptyColumnName {
            row,
            schema_id: boxed_str(schema_id),
        });
    }
    let unit_label = entry.unit_label.trim();
    if unit_label.is_empty() {
        return Err(OutputUnitRegistryError::EmptyUnitLabel {
            row,
            schema_id: boxed_str(schema_id),
            column_name: boxed_str(column_name),
        });
    }

    match entry.authority {
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
