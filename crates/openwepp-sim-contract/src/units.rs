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
