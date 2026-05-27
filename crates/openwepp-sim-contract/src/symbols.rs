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
    InvalidBoundaryAliasTemplate {
        row: usize,
        canonical_symbol: String,
        boundary_alias: String,
        reason: String,
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
            Self::InvalidBoundaryAliasTemplate {
                row,
                canonical_symbol,
                boundary_alias,
                reason,
            } => write!(
                f,
                "invalid boundary alias template at row {row} for canonical symbol {canonical_symbol} and alias {boundary_alias}: {reason}"
            ),
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

#[derive(Debug, Clone, PartialEq, Eq)]
struct SymbolAliasTemplateEntry {
    canonical_symbol: String,
    boundary_alias_template: String,
}

impl SymbolAliasTemplateEntry {
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

/// Canonical symbol alias registry with deterministic reverse lookups.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SymbolAliasRegistry {
    canonical_to_aliases: BTreeMap<String, Vec<String>>,
    alias_to_canonical: BTreeMap<String, String>,
    template_aliases: Vec<SymbolAliasTemplateEntry>,
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
        let mut template_to_canonical: BTreeMap<String, String> = BTreeMap::new();
        let mut template_aliases = Vec::new();
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

            if alias_uses_template(alias_key.as_str()) {
                validate_boundary_alias_template(row, canonical_key.as_str(), alias_key.as_str())?;

                if let Some(existing_canonical) = template_to_canonical.get(alias_key.as_str()) {
                    if existing_canonical != &canonical_key {
                        return Err(SymbolAliasRegistryError::AmbiguousBoundaryAlias {
                            boundary_alias: alias_key,
                            canonical_a: existing_canonical.clone(),
                            canonical_b: canonical_key,
                        });
                    }
                } else {
                    template_to_canonical.insert(alias_key.clone(), canonical_key.clone());
                }

                template_aliases.push(SymbolAliasTemplateEntry::new(
                    canonical_key.clone(),
                    alias_key.clone(),
                ));
            } else if let Some(existing_canonical) = alias_to_canonical.get(alias_key.as_str()) {
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
            template_aliases,
        })
    }

    /// Canonical WEPP/wepp-forest alias registry for ARCH03 baseline surfaces.
    ///
    /// # Errors
    ///
    /// Returns typed `SymbolAliasRegistryError` when the embedded baseline map
    /// fails registry validation.
    #[allow(clippy::too_many_lines)]
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
            SymbolAliasEntry::new("total_detachment_kg", "total_detachment_kg"),
            SymbolAliasEntry::new("total_detachment_kg", "hs{ofe}_total_detachment_kg"),
            SymbolAliasEntry::new("total_deposition_kg", "total_deposition_kg"),
            SymbolAliasEntry::new("total_deposition_kg", "hs{ofe}_total_deposition_kg"),
            SymbolAliasEntry::new("particle_class_count", "particle_class_count"),
            SymbolAliasEntry::new("particle_class_count", "hs{ofe}_particle_class_count"),
            SymbolAliasEntry::new(
                "sediment_concentration_kg_m3",
                "sediment_concentration_kg_m3_{idx4}",
            ),
            SymbolAliasEntry::new(
                "sediment_concentration_kg_m3",
                "hs{ofe}_sediment_concentration_kg_m3_{idx4}",
            ),
            SymbolAliasEntry::new("particle_diameter_m", "particle_diameter_m_{idx4}"),
            SymbolAliasEntry::new("particle_diameter_m", "hs{ofe}_particle_diameter_m_{idx4}"),
            SymbolAliasEntry::new("particle_flow_fraction", "particle_flow_fraction_{idx4}"),
            SymbolAliasEntry::new(
                "particle_flow_fraction",
                "hs{ofe}_particle_flow_fraction_{idx4}",
            ),
            SymbolAliasEntry::new("nelem", "nelem"),
            SymbolAliasEntry::new("nwsofe", "nwsofe"),
            SymbolAliasEntry::new("nslpts", "nslpts"),
            SymbolAliasEntry::new("nslpts", "ofe{ofe}_nslpts"),
            SymbolAliasEntry::new("slplen", "slplen"),
            SymbolAliasEntry::new("slplen", "ofe{ofe}_slplen"),
            SymbolAliasEntry::new("avgslp", "avgslp"),
            SymbolAliasEntry::new("avgslp", "ofe{ofe}_avgslp"),
            SymbolAliasEntry::new("xinput", "xinput_{idx4}"),
            SymbolAliasEntry::new("xinput", "ofe{ofe}_xinput_{idx4}"),
            SymbolAliasEntry::new("slpinp", "slpinp_{idx4}"),
            SymbolAliasEntry::new("slpinp", "ofe{ofe}_slpinp_{idx4}"),
            SymbolAliasEntry::new("ntemp", "ntemp"),
            SymbolAliasEntry::new("nsl", "nsl"),
            SymbolAliasEntry::new("nsl", "ofe{ofe}_nsl"),
            SymbolAliasEntry::new("solthk", "solthk"),
            SymbolAliasEntry::new("solthk", "solthk_{idx4}"),
            SymbolAliasEntry::new("solthk", "ofe{ofe}_solthk"),
            SymbolAliasEntry::new("solthk", "ofe{ofe}_solthk_{idx4}"),
            SymbolAliasEntry::new("dg", "dg"),
            SymbolAliasEntry::new("dg", "dg_{idx4}"),
            SymbolAliasEntry::new("dg", "ofe{ofe}_dg_{idx4}"),
            SymbolAliasEntry::new("thetdr", "thetdr"),
            SymbolAliasEntry::new("thetdr", "thetdr_{idx4}"),
            SymbolAliasEntry::new("thetdr", "ofe{ofe}_thetdr_{idx4}"),
            SymbolAliasEntry::new("thetfc", "thetfc"),
            SymbolAliasEntry::new("thetfc", "thetfc_{idx4}"),
            SymbolAliasEntry::new("thetfc", "ofe{ofe}_thetfc_{idx4}"),
            SymbolAliasEntry::new("ssc", "ssc"),
            SymbolAliasEntry::new("ssc", "ssc_{idx4}"),
            SymbolAliasEntry::new("ssc", "ofe{ofe}_ssc_{idx4}"),
            SymbolAliasEntry::new("lanuse", "lanuse"),
            SymbolAliasEntry::new("lanuse", "ofe{ofe}_lanuse"),
            SymbolAliasEntry::new("lanuse", "pl_schedule_ofe{ofe}_lanuse"),
            SymbolAliasEntry::new("lanuse", "pl_schedule_slot_{idx4}_crop_{idx4}_lanuse"),
            SymbolAliasEntry::new("nowcrp", "nowcrp"),
            SymbolAliasEntry::new("nowcrp", "ofe{ofe}_nowcrp"),
            SymbolAliasEntry::new("itype", "itype"),
            SymbolAliasEntry::new("itype", "itype_{idx4}"),
            SymbolAliasEntry::new("itype", "ofe{ofe}_itype_{idx4}"),
            SymbolAliasEntry::new("itype", "pl_schedule_slot_{idx4}_crop_{idx4}_itype"),
            SymbolAliasEntry::new("itype", "pl_growth_slot_{idx4}_crop_{idx4}_itype"),
            SymbolAliasEntry::new("imngmt", "imngmt"),
            SymbolAliasEntry::new("imngmt", "imngmt_{idx4}"),
            SymbolAliasEntry::new("imngmt", "ofe{ofe}_imngmt_{idx4}"),
            SymbolAliasEntry::new("imngmt", "pl_growth_ofe{ofe}_imngmt_seed"),
            SymbolAliasEntry::new("imngmt", "pl_schedule_slot_{idx4}_crop_{idx4}_imngmt"),
            SymbolAliasEntry::new("imngmt", "pl_growth_slot_{idx4}_crop_{idx4}_imngmt"),
            SymbolAliasEntry::new("tilseq", "tilseq"),
            SymbolAliasEntry::new("tilseq", "tilseq_{idx4}"),
            SymbolAliasEntry::new("tilseq", "ofe{ofe}_tilseq_{idx4}"),
            SymbolAliasEntry::new("tilseq", "pl_schedule_slot_{idx4}_crop_{idx4}_tilseq"),
            SymbolAliasEntry::new("conseq", "conseq"),
            SymbolAliasEntry::new("conseq", "conseq_{idx4}"),
            SymbolAliasEntry::new("conseq", "ofe{ofe}_conseq_{idx4}"),
            SymbolAliasEntry::new("conseq", "conset"),
            SymbolAliasEntry::new("conseq", "conset_{idx4}"),
            SymbolAliasEntry::new("conseq", "ofe{ofe}_conset_{idx4}"),
            SymbolAliasEntry::new("conseq", "pl_schedule_slot_{idx4}_crop_{idx4}_conset"),
            SymbolAliasEntry::new("drseq", "drseq"),
            SymbolAliasEntry::new("drseq", "drseq_{idx4}"),
            SymbolAliasEntry::new("drseq", "ofe{ofe}_drseq_{idx4}"),
            SymbolAliasEntry::new("drseq", "drset"),
            SymbolAliasEntry::new("drseq", "drset_{idx4}"),
            SymbolAliasEntry::new("drseq", "ofe{ofe}_drset_{idx4}"),
            SymbolAliasEntry::new("drseq", "pl_schedule_slot_{idx4}_crop_{idx4}_drset"),
            SymbolAliasEntry::new("jdplt", "jdplt"),
            SymbolAliasEntry::new("jdplt", "jdplt_{idx4}"),
            SymbolAliasEntry::new("jdplt", "ofe{ofe}_jdplt_{idx4}"),
            SymbolAliasEntry::new("jdplt", "pl_growth_slot_{idx4}_crop_{idx4}_jdplt"),
            SymbolAliasEntry::new("jdharv", "jdharv"),
            SymbolAliasEntry::new("jdharv", "jdharv_{idx4}"),
            SymbolAliasEntry::new("jdharv", "ofe{ofe}_jdharv_{idx4}"),
            SymbolAliasEntry::new("jdharv", "pl_growth_slot_{idx4}_crop_{idx4}_jdharv"),
            SymbolAliasEntry::new("jdstop", "jdstop"),
            SymbolAliasEntry::new("jdstop", "jdstop_{idx4}"),
            SymbolAliasEntry::new("jdstop", "ofe{ofe}_jdstop_{idx4}"),
            SymbolAliasEntry::new("jdstop", "pl_growth_slot_{idx4}_crop_{idx4}_jdstop"),
            SymbolAliasEntry::new("resmgt", "resmgt"),
            SymbolAliasEntry::new("resmgt", "resmgt_{idx4}"),
            SymbolAliasEntry::new("resmgt", "ofe{ofe}_resmgt_{idx4}"),
            SymbolAliasEntry::new("resmgt", "pl_decomp_slot_{idx4}_crop_{idx4}_resmgt"),
            SymbolAliasEntry::new("mgtopt", "mgtopt"),
            SymbolAliasEntry::new("mgtopt", "mgtopt_{idx4}"),
            SymbolAliasEntry::new("mgtopt", "ofe{ofe}_mgtopt_{idx4}"),
            SymbolAliasEntry::new("mgtopt", "pl_growth_slot_{idx4}_crop_{idx4}_mgtopt"),
            SymbolAliasEntry::new("mgtopt", "pl_decomp_slot_{idx4}_crop_{idx4}_mgtopt"),
            SymbolAliasEntry::new("rw", "rw"),
            SymbolAliasEntry::new("rw", "rw_{idx4}"),
            SymbolAliasEntry::new("rw", "ofe{ofe}_rw_{idx4}"),
            SymbolAliasEntry::new("rw", "pl_growth_slot_{idx4}_crop_{idx4}_rw"),
            SymbolAliasEntry::new("jdherb", "jdherb"),
            SymbolAliasEntry::new("jdherb", "jdherb_{idx4}"),
            SymbolAliasEntry::new("jdherb", "ofe{ofe}_jdherb_{idx4}"),
            SymbolAliasEntry::new("jdherb", "pl_decomp_slot_{idx4}_crop_{idx4}_jdherb"),
            SymbolAliasEntry::new("jdburn", "jdburn"),
            SymbolAliasEntry::new("jdburn", "jdburn_{idx4}"),
            SymbolAliasEntry::new("jdburn", "ofe{ofe}_jdburn_{idx4}"),
            SymbolAliasEntry::new("jdburn", "pl_decomp_slot_{idx4}_crop_{idx4}_jdburn"),
            SymbolAliasEntry::new("jdslge", "jdslge"),
            SymbolAliasEntry::new("jdslge", "jdslge_{idx4}"),
            SymbolAliasEntry::new("jdslge", "ofe{ofe}_jdslge_{idx4}"),
            SymbolAliasEntry::new("jdslge", "pl_decomp_slot_{idx4}_crop_{idx4}_jdslge"),
            SymbolAliasEntry::new("jdcut", "jdcut"),
            SymbolAliasEntry::new("jdcut", "jdcut_{idx4}"),
            SymbolAliasEntry::new("jdcut", "ofe{ofe}_jdcut_{idx4}"),
            SymbolAliasEntry::new("jdcut", "pl_decomp_slot_{idx4}_crop_{idx4}_jdcut"),
            SymbolAliasEntry::new("jdmove", "jdmove"),
            SymbolAliasEntry::new("jdmove", "jdmove_{idx4}"),
            SymbolAliasEntry::new("jdmove", "ofe{ofe}_jdmove_{idx4}"),
            SymbolAliasEntry::new("jdmove", "pl_decomp_slot_{idx4}_crop_{idx4}_jdmove"),
            SymbolAliasEntry::new("fbrnag", "fbrnag"),
            SymbolAliasEntry::new("fbrnag", "fbrnag_{idx4}"),
            SymbolAliasEntry::new("fbrnag", "ofe{ofe}_fbrnag_{idx4}"),
            SymbolAliasEntry::new("fbrnag", "pl_decomp_slot_{idx4}_crop_{idx4}_fbrnag"),
            SymbolAliasEntry::new("fbrnog", "fbrnog"),
            SymbolAliasEntry::new("fbrnog", "fbrnog_{idx4}"),
            SymbolAliasEntry::new("fbrnog", "ofe{ofe}_fbrnog_{idx4}"),
            SymbolAliasEntry::new("fbrnog", "pl_decomp_slot_{idx4}_crop_{idx4}_fbrnog"),
            SymbolAliasEntry::new("frcut", "frcut"),
            SymbolAliasEntry::new("frcut", "frcut_{idx4}"),
            SymbolAliasEntry::new("frcut", "ofe{ofe}_frcut_{idx4}"),
            SymbolAliasEntry::new("frcut", "pl_decomp_slot_{idx4}_crop_{idx4}_frcut"),
            SymbolAliasEntry::new("frmove", "frmove"),
            SymbolAliasEntry::new("frmove", "frmove_{idx4}"),
            SymbolAliasEntry::new("frmove", "ofe{ofe}_frmove_{idx4}"),
            SymbolAliasEntry::new("frmove", "pl_decomp_slot_{idx4}_crop_{idx4}_frmove"),
            SymbolAliasEntry::new("ncut", "ncut"),
            SymbolAliasEntry::new("ncut", "ncut_{idx4}"),
            SymbolAliasEntry::new("ncut", "ofe{ofe}_ncut_{idx4}"),
            SymbolAliasEntry::new("ncut", "pl_decomp_slot_{idx4}_crop_{idx4}_ncut"),
            SymbolAliasEntry::new("ncycle", "ncycle"),
            SymbolAliasEntry::new("ncycle", "ncycle_{idx4}"),
            SymbolAliasEntry::new("ncycle", "ofe{ofe}_ncycle_{idx4}"),
            SymbolAliasEntry::new("ncycle", "pl_decomp_slot_{idx4}_crop_{idx4}_ncycle"),
            SymbolAliasEntry::new("cutday", "cutday"),
            SymbolAliasEntry::new("cutday", "cutday_{idx4}"),
            SymbolAliasEntry::new("cutday", "ofe{ofe}_cutday_{idx4}"),
            SymbolAliasEntry::new("cutday", "pl_decomp_slot_{idx4}_crop_{idx4}_cutday_{idx4}"),
            SymbolAliasEntry::new("gday", "gday"),
            SymbolAliasEntry::new("gday", "gday_{idx4}"),
            SymbolAliasEntry::new("gday", "ofe{ofe}_gday_{idx4}"),
            SymbolAliasEntry::new("gday", "pl_decomp_slot_{idx4}_crop_{idx4}_gday_{idx4}"),
            SymbolAliasEntry::new("gend", "gend"),
            SymbolAliasEntry::new("gend", "gend_{idx4}"),
            SymbolAliasEntry::new("gend", "ofe{ofe}_gend_{idx4}"),
            SymbolAliasEntry::new("gend", "pl_decomp_slot_{idx4}_crop_{idx4}_gend_{idx4}"),
            SymbolAliasEntry::new("animal", "animal"),
            SymbolAliasEntry::new("animal", "animal_{idx4}"),
            SymbolAliasEntry::new("animal", "ofe{ofe}_animal_{idx4}"),
            SymbolAliasEntry::new("animal", "pl_decomp_slot_{idx4}_crop_{idx4}_animal_{idx4}"),
            SymbolAliasEntry::new("bodywt", "bodywt"),
            SymbolAliasEntry::new("bodywt", "bodywt_{idx4}"),
            SymbolAliasEntry::new("bodywt", "ofe{ofe}_bodywt_{idx4}"),
            SymbolAliasEntry::new("bodywt", "pl_decomp_slot_{idx4}_crop_{idx4}_bodywt_{idx4}"),
            SymbolAliasEntry::new("area", "area"),
            SymbolAliasEntry::new("area", "area_{idx4}"),
            SymbolAliasEntry::new("area", "ofe{ofe}_area_{idx4}"),
            SymbolAliasEntry::new("area", "pl_decomp_slot_{idx4}_crop_{idx4}_area_{idx4}"),
            SymbolAliasEntry::new("digest", "digest"),
            SymbolAliasEntry::new("digest", "digest_{idx4}"),
            SymbolAliasEntry::new("digest", "ofe{ofe}_digest_{idx4}"),
            SymbolAliasEntry::new("digest", "pl_decomp_slot_{idx4}_crop_{idx4}_digest_{idx4}"),
            SymbolAliasEntry::new("vdmt", "vdmt"),
            SymbolAliasEntry::new("vdmt", "ofe{ofe}_vdmt"),
            SymbolAliasEntry::new("tlive", "tlive"),
            SymbolAliasEntry::new("tlive", "ofe{ofe}_tlive"),
            SymbolAliasEntry::new("cancov", "cancov"),
            SymbolAliasEntry::new("cancov", "ofe{ofe}_cancov"),
            SymbolAliasEntry::new("canhgt", "canhgt"),
            SymbolAliasEntry::new("canhgt", "ofe{ofe}_canhgt"),
            SymbolAliasEntry::new("lai", "lai"),
            SymbolAliasEntry::new("lai", "ofe{ofe}_lai"),
            SymbolAliasEntry::new("rtmass", "rtmass"),
            SymbolAliasEntry::new("rtmass", "ofe{ofe}_rtmass"),
            SymbolAliasEntry::new("rtd", "rtd"),
            SymbolAliasEntry::new("rtd", "ofe{ofe}_rtd"),
            SymbolAliasEntry::new("sumgdd", "sumgdd"),
            SymbolAliasEntry::new("sumgdd", "ofe{ofe}_sumgdd"),
            SymbolAliasEntry::new("hia", "hia"),
            SymbolAliasEntry::new("hia", "ofe{ofe}_hia"),
            SymbolAliasEntry::new("vdmx", "vdmx"),
            SymbolAliasEntry::new("vdmx", "ofe{ofe}_vdmx"),
            SymbolAliasEntry::new("isenes", "isenes"),
            SymbolAliasEntry::new("isenes", "ofe{ofe}_isenes"),
            SymbolAliasEntry::new("ncount", "ncount"),
            SymbolAliasEntry::new("ncount", "ofe{ofe}_ncount"),
            SymbolAliasEntry::new("rmagt", "rmagt"),
            SymbolAliasEntry::new("rmagt", "ofe{ofe}_rmagt"),
            SymbolAliasEntry::new("rmogt", "rmogt_{idx4}"),
            SymbolAliasEntry::new("rmogt", "ofe{ofe}_rmogt_{idx4}"),
            SymbolAliasEntry::new("rilrm", "rilrm_{idx4}"),
            SymbolAliasEntry::new("rilrm", "ofe{ofe}_rilrm_{idx4}"),
            SymbolAliasEntry::new("rigrm", "rigrm_{idx4}"),
            SymbolAliasEntry::new("rigrm", "ofe{ofe}_rigrm_{idx4}"),
            SymbolAliasEntry::new("smrm", "smrm_{idx4}"),
            SymbolAliasEntry::new("smrm", "ofe{ofe}_smrm_{idx4}"),
            SymbolAliasEntry::new("rtm", "rtm_{idx4}"),
            SymbolAliasEntry::new("rtm", "ofe{ofe}_rtm_{idx4}"),
            SymbolAliasEntry::new("iresd", "iresd_{idx4}"),
            SymbolAliasEntry::new("iresd", "ofe{ofe}_iresd_{idx4}"),
            SymbolAliasEntry::new("iroot", "iroot_{idx4}"),
            SymbolAliasEntry::new("iroot", "ofe{ofe}_iroot_{idx4}"),
            SymbolAliasEntry::new("senvin", "senvin"),
            SymbolAliasEntry::new("senvin", "ofe{ofe}_senvin"),
            SymbolAliasEntry::new("fenvin", "fenvin_{idx4}"),
            SymbolAliasEntry::new("fenvin", "ofe{ofe}_fenvin_{idx4}"),
            SymbolAliasEntry::new("benvin", "benvin_{idx4}"),
            SymbolAliasEntry::new("benvin", "ofe{ofe}_benvin_{idx4}"),
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
        if let Some(canonical) = self.alias_to_canonical.get(boundary_alias) {
            return Ok(canonical.as_str());
        }

        let mut matching_canonicals: BTreeSet<&str> = BTreeSet::new();
        for template_entry in &self.template_aliases {
            if boundary_alias_matches_template(
                boundary_alias,
                template_entry.boundary_alias_template.as_str(),
            ) {
                matching_canonicals.insert(template_entry.canonical_symbol.as_str());
            }
        }

        match (
            matching_canonicals.iter().next(),
            matching_canonicals.iter().nth(1),
        ) {
            (Some(canonical), None) => Ok(*canonical),
            (Some(canonical_a), Some(canonical_b)) => {
                Err(SymbolAliasRegistryError::AmbiguousBoundaryAlias {
                    boundary_alias: boundary_alias.to_string(),
                    canonical_a: (*canonical_a).to_string(),
                    canonical_b: (*canonical_b).to_string(),
                })
            }
            _ => Err(SymbolAliasRegistryError::BoundaryAliasNotFound {
                boundary_alias: boundary_alias.to_string(),
            }),
        }
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

fn alias_uses_template(alias: &str) -> bool {
    alias.contains('{') || alias.contains('}')
}

fn validate_boundary_alias_template(
    row: usize,
    canonical_symbol: &str,
    boundary_alias: &str,
) -> Result<(), SymbolAliasRegistryError> {
    let mut cursor = 0usize;
    while let Some(offset) = boundary_alias[cursor..].find('{') {
        let start = cursor + offset;
        let Some(close_offset) = boundary_alias[start + 1..].find('}') else {
            return Err(SymbolAliasRegistryError::InvalidBoundaryAliasTemplate {
                row,
                canonical_symbol: canonical_symbol.to_string(),
                boundary_alias: boundary_alias.to_string(),
                reason: "missing closing brace".to_string(),
            });
        };
        let end = start + 1 + close_offset;
        let token = &boundary_alias[start + 1..end];
        if token != "ofe" && token != "idx4" {
            return Err(SymbolAliasRegistryError::InvalidBoundaryAliasTemplate {
                row,
                canonical_symbol: canonical_symbol.to_string(),
                boundary_alias: boundary_alias.to_string(),
                reason: format!("unsupported token {{{token}}}"),
            });
        }
        cursor = end + 1;
    }

    if boundary_alias[cursor..].contains('}') {
        return Err(SymbolAliasRegistryError::InvalidBoundaryAliasTemplate {
            row,
            canonical_symbol: canonical_symbol.to_string(),
            boundary_alias: boundary_alias.to_string(),
            reason: "closing brace without matching opening brace".to_string(),
        });
    }

    Ok(())
}

fn boundary_alias_matches_template(boundary_alias: &str, template: &str) -> bool {
    let template_bytes = template.as_bytes();
    let alias_bytes = boundary_alias.as_bytes();
    let mut template_index = 0usize;
    let mut alias_index = 0usize;

    while template_index < template_bytes.len() {
        if template_bytes[template_index] == b'{' {
            let mut end = template_index + 1;
            while end < template_bytes.len() && template_bytes[end] != b'}' {
                end += 1;
            }
            if end == template_bytes.len() {
                return false;
            }

            let token = &template[template_index + 1..end];
            match token {
                "ofe" => {
                    let start = alias_index;
                    while alias_index < alias_bytes.len()
                        && alias_bytes[alias_index].is_ascii_digit()
                    {
                        alias_index += 1;
                    }
                    if alias_index == start {
                        return false;
                    }

                    let parsed = boundary_alias[start..alias_index].parse::<u32>().ok();
                    if parsed.is_none_or(|value| value == 0) {
                        return false;
                    }
                }
                "idx4" => {
                    if alias_index + 4 > alias_bytes.len() {
                        return false;
                    }
                    if !alias_bytes[alias_index..alias_index + 4]
                        .iter()
                        .all(u8::is_ascii_digit)
                    {
                        return false;
                    }
                    alias_index += 4;
                }
                _ => return false,
            }

            template_index = end + 1;
            continue;
        }

        if alias_index >= alias_bytes.len()
            || template_bytes[template_index] != alias_bytes[alias_index]
        {
            return false;
        }
        template_index += 1;
        alias_index += 1;
    }

    alias_index == alias_bytes.len()
}
