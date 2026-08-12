use std::collections::{BTreeMap, BTreeSet};

use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};

use crate::{MODEL_SHA256, VegetationError};

#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum PhenologyType {
    Evergreen,
    SeasonalDeciduous,
}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct TopologyTile {
    pub tile_id: String,
    pub fraction: f64,
}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct RootLayer {
    pub layer_id: String,
    pub root_fraction: f64,
    pub mineral_n_root_fraction: f64,
    pub lateral_root_length_m: f64,
}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct StratumConfiguration {
    pub stratum_id: String,
    pub lifeform: String,
    pub phenology_type: PhenologyType,
    pub vertical_rank: u32,
    pub tile_ids: Vec<String>,
    pub height_m: f64,
    pub crown_base_m: f64,
    pub leaf_dimension_m: f64,
    pub stem_dimension_m: f64,
    pub wet_surface_dimension_m: f64,
    pub sla_m2_per_kg_c: f64,
    pub sai_relation: f64,
    pub leaf_angle_chi: f64,
    pub clumping_index: f64,
    pub leaf_rho_vis: f64,
    pub leaf_tau_vis: f64,
    pub leaf_rho_nir: f64,
    pub leaf_tau_nir: f64,
    pub stem_rho_vis: f64,
    pub stem_tau_vis: f64,
    pub stem_rho_nir: f64,
    pub stem_tau_nir: f64,
    pub g0_umol_h2o_m2_s: f64,
    pub g1_sqrt_kpa: f64,
    pub rubisco_n_efficiency: f64,
    pub electron_n_efficiency: f64,
    pub tp_vcmax_ratio: f64,
    pub rd_leaf_n_rate: f64,
    pub kc25_pa: f64,
    pub ko25_pa: f64,
    pub gamma25_pa: f64,
    pub ha_vcmax: f64,
    pub hd_vcmax: f64,
    pub entropy_vcmax: f64,
    pub ha_jmax: f64,
    pub hd_jmax: f64,
    pub entropy_jmax: f64,
    pub ha_kc: f64,
    pub ha_ko: f64,
    pub ha_gamma: f64,
    pub alpha_liq: f64,
    pub p_liq_kg_m2_plant: f64,
    pub stemflow_fraction: f64,
    pub z0m_m: f64,
    pub z0h_m: f64,
    pub z0q_m: f64,
    pub displacement_m: f64,
    pub leaf_emissivity: f64,
    pub stem_emissivity: f64,
    pub wet_surface_emissivity: f64,
    pub k1a_max_s1: f64,
    pub k1b_max_s1: f64,
    pub k2_max_m_s: f64,
    pub k3_max_m_s: f64,
    pub p50_leaf_mm: f64,
    pub p50_stem_mm: f64,
    pub p50_root_mm: f64,
    pub vulnerability_shape: f64,
    pub root_to_leaf_area: f64,
    pub root_layers: Vec<RootLayer>,
    pub atkin_intercept: f64,
    pub mr_base_kgc_per_kgn_s: f64,
    pub mr_q10: f64,
    pub xs_recovery_days: f64,
    pub growth_resp_ratio_g1: f64,
    pub alloc_froot_leaf_a1: f64,
    pub alloc_croot_stem_a2: f64,
    pub alloc_stem_leaf_a3: f64,
    pub livewood_fraction_a4: f64,
    pub current_growth_fraction: f64,
    pub cn_leaf: f64,
    pub cn_leaf_litter: f64,
    pub cn_froot: f64,
    pub cn_livewood: f64,
    pub cn_deadwood: f64,
    pub drymatter_carbon_fraction: f64,
    pub nh4_request_fraction: f64,
    pub litter_metabolic_fraction: BTreeMap<String, f64>,
    pub litter_cellulose_fraction: BTreeMap<String, f64>,
    pub litter_lignin_fraction: BTreeMap<String, f64>,
    pub onset_duration_s: Option<f64>,
    pub offset_duration_s: Option<f64>,
    pub gsi_on_threshold: Option<f64>,
    pub gsi_off_threshold: Option<f64>,
    pub gsi_hysteresis: Option<f64>,
    pub leaf_lifetime_s: f64,
    pub froot_lifetime_s: f64,
    pub livewood_turnover_s: f64,
    pub mortality_rate_s1: f64,
}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct VegetationConfiguration {
    pub model_definition_sha256: String,
    pub configuration_sha256: String,
    pub initial_state_sha256: String,
    pub area_m2: f64,
    pub timestamp: String,
    pub dt_s: f64,
    pub topology_tiles: Vec<TopologyTile>,
    pub strata: Vec<StratumConfiguration>,
}

impl VegetationConfiguration {
    pub fn parse_strict(bytes: &[u8]) -> Result<Self, VegetationError> {
        let value: Self =
            serde_json::from_slice(bytes).map_err(|e| VegetationError::Schema(e.to_string()))?;
        value.validate()?;
        Ok(value)
    }

    pub fn validate(&self) -> Result<(), VegetationError> {
        finite_positive(self.area_m2, "area_m2")?;
        finite_positive(self.dt_s, "dt_s")?;
        if self.model_definition_sha256 != MODEL_SHA256 {
            return Err(VegetationError::ModelDigestMismatch {
                expected: MODEL_SHA256.into(),
                found: self.model_definition_sha256.clone(),
            });
        }
        require_hex_digest(&self.configuration_sha256, "configuration_sha256")?;
        require_hex_digest(&self.initial_state_sha256, "initial_state_sha256")?;
        if self.configuration_sha256 != self.canonical_sha256()? {
            return Err(VegetationError::Receipt(
                "configuration digest does not match canonical bytes".into(),
            ));
        }
        if self.topology_tiles.is_empty() {
            return Err(VegetationError::Domain("empty topology"));
        }
        if self.topology_tiles.iter().any(|tile| {
            tile.tile_id.trim().is_empty() || !tile.fraction.is_finite() || tile.fraction <= 0.0
        }) {
            return Err(VegetationError::Domain("topology tile identity/fraction"));
        }
        let tile_sum: f64 = self.topology_tiles.iter().map(|v| v.fraction).sum();
        if (tile_sum - 1.0).abs() > 1e-12 {
            return Err(VegetationError::Domain("topology tile fractions"));
        }
        let tiles: BTreeSet<_> = self
            .topology_tiles
            .iter()
            .map(|v| v.tile_id.as_str())
            .collect();
        if tiles.len() != self.topology_tiles.len() {
            return Err(VegetationError::Domain("duplicate topology tile"));
        }
        let mut ids = BTreeSet::new();
        let mut tile_ranks = BTreeSet::new();
        for s in &self.strata {
            if !ids.insert(s.stratum_id.as_str()) || s.lifeform != "C3_WOODY" {
                return Err(VegetationError::Unsupported(
                    "lifeform or duplicate stratum",
                ));
            }
            validate_stratum(s, &tiles)?;
            for tile_id in &s.tile_ids {
                if !tile_ranks.insert((tile_id.as_str(), s.vertical_rank)) {
                    return Err(VegetationError::Domain("duplicate tile/rank occupancy"));
                }
            }
        }
        for tile in &self.topology_tiles {
            let mut column = self
                .strata
                .iter()
                .filter(|stratum| stratum.tile_ids.contains(&tile.tile_id))
                .collect::<Vec<_>>();
            column.sort_by_key(|stratum| stratum.vertical_rank);
            if column.windows(2).any(|pair| {
                pair[0].vertical_rank >= pair[1].vertical_rank
                    || pair[0].height_m <= pair[1].height_m
            }) {
                return Err(VegetationError::Domain("topology rank/height order"));
            }
        }
        Ok(())
    }

    pub fn canonical_sha256(&self) -> Result<String, VegetationError> {
        let mut canonical = self.clone();
        canonical.configuration_sha256.clear();
        canonical.initial_state_sha256.clear();
        let bytes =
            serde_json::to_vec(&canonical).map_err(|e| VegetationError::Schema(e.to_string()))?;
        Ok(format!("{:x}", Sha256::digest(bytes)))
    }
}

fn require_hex_digest(value: &str, field: &'static str) -> Result<(), VegetationError> {
    if value.len() != 64 || !value.bytes().all(|b| b.is_ascii_hexdigit()) {
        return Err(VegetationError::Domain(field));
    }
    Ok(())
}
fn finite_positive(value: f64, field: &'static str) -> Result<(), VegetationError> {
    if !value.is_finite() || value <= 0.0 {
        return Err(VegetationError::Domain(field));
    }
    Ok(())
}
fn fraction(value: f64, field: &'static str) -> Result<(), VegetationError> {
    if !value.is_finite() || !(0.0..=1.0).contains(&value) {
        return Err(VegetationError::Domain(field));
    }
    Ok(())
}
#[allow(clippy::too_many_lines)]
fn validate_stratum(
    s: &StratumConfiguration,
    tiles: &BTreeSet<&str>,
) -> Result<(), VegetationError> {
    for id in &s.tile_ids {
        if !tiles.contains(id.as_str()) {
            return Err(VegetationError::Domain("stratum tile membership"));
        }
    }
    if s.stratum_id.trim().is_empty() || s.tile_ids.is_empty() {
        return Err(VegetationError::Domain("stratum identity/tile membership"));
    }
    finite_positive(s.height_m, "height_m")?;
    finite_positive(s.leaf_dimension_m, "leaf_dimension_m")?;
    finite_positive(s.stem_dimension_m, "stem_dimension_m")?;
    finite_positive(s.wet_surface_dimension_m, "wet_surface_dimension_m")?;
    finite_positive(s.sla_m2_per_kg_c, "sla")?;
    finite_positive(s.clumping_index, "clumping_index")?;
    if !s.crown_base_m.is_finite() {
        return Err(VegetationError::Domain("crown_base_m"));
    }
    if s.clumping_index > 1.0
        || !(-0.4..=0.6).contains(&s.leaf_angle_chi)
        || s.crown_base_m < 0.0
        || s.crown_base_m >= s.height_m
    {
        return Err(VegetationError::Domain("geometry/leaf angle"));
    }
    for (rho, tau) in [
        (s.leaf_rho_vis, s.leaf_tau_vis),
        (s.leaf_rho_nir, s.leaf_tau_nir),
        (s.stem_rho_vis, s.stem_tau_vis),
        (s.stem_rho_nir, s.stem_tau_nir),
    ] {
        fraction(rho, "rho")?;
        fraction(tau, "tau")?;
        if rho + tau >= 1.0 {
            return Err(VegetationError::Domain("optical rho+tau"));
        }
    }
    fraction(s.alpha_liq, "alpha_liq")?;
    fraction(s.stemflow_fraction, "stemflow_fraction")?;
    if !s.p_liq_kg_m2_plant.is_finite() || s.p_liq_kg_m2_plant < 0.0 {
        return Err(VegetationError::Domain("p_liq_kg_m2_plant"));
    }
    fraction(s.livewood_fraction_a4, "a4")?;
    fraction(s.current_growth_fraction, "current_growth_fraction")?;
    fraction(s.nh4_request_fraction, "nh4_request_fraction")?;
    fraction(s.drymatter_carbon_fraction, "drymatter_carbon_fraction")?;
    if s.drymatter_carbon_fraction == 0.0
        || s.leaf_emissivity <= 0.0
        || s.stem_emissivity <= 0.0
        || s.wet_surface_emissivity <= 0.0
    {
        return Err(VegetationError::Domain(
            "positive material/emissivity fraction",
        ));
    }
    for (value, field) in [
        (s.g0_umol_h2o_m2_s, "g0"),
        (s.g1_sqrt_kpa, "g1"),
        (s.rd_leaf_n_rate, "rd_leaf_n_rate"),
        (s.sai_relation, "sai_relation"),
        (s.atkin_intercept, "atkin_intercept"),
        (s.mr_base_kgc_per_kgn_s, "mr_base"),
        (s.alloc_froot_leaf_a1, "a1"),
        (s.alloc_croot_stem_a2, "a2"),
        (s.alloc_stem_leaf_a3, "a3"),
        (s.mortality_rate_s1, "mortality_rate"),
    ] {
        if !value.is_finite() || value < 0.0 {
            return Err(VegetationError::Domain(field));
        }
    }
    for (value, field) in [
        (s.rubisco_n_efficiency, "rubisco_n_efficiency"),
        (s.electron_n_efficiency, "electron_n_efficiency"),
        (s.tp_vcmax_ratio, "tp_vcmax_ratio"),
        (s.kc25_pa, "kc25"),
        (s.ko25_pa, "ko25"),
        (s.gamma25_pa, "gamma25"),
        (s.z0m_m, "z0m"),
        (s.z0h_m, "z0h"),
        (s.z0q_m, "z0q"),
        (s.k1a_max_s1, "k1a"),
        (s.k1b_max_s1, "k1b"),
        (s.k2_max_m_s, "k2"),
        (s.k3_max_m_s, "k3"),
        (s.vulnerability_shape, "vulnerability_shape"),
        (s.root_to_leaf_area, "root_to_leaf_area"),
        (s.mr_q10, "mr_q10"),
        (s.xs_recovery_days, "xs_recovery_days"),
        (s.cn_leaf, "cn_leaf"),
        (s.cn_leaf_litter, "cn_leaf_litter"),
        (s.cn_froot, "cn_froot"),
        (s.cn_livewood, "cn_livewood"),
        (s.cn_deadwood, "cn_deadwood"),
        (s.leaf_lifetime_s, "leaf_lifetime"),
        (s.froot_lifetime_s, "froot_lifetime"),
        (s.livewood_turnover_s, "livewood_turnover"),
    ] {
        finite_positive(value, field)?;
    }
    for (value, field) in [
        (s.ha_vcmax, "ha_vcmax"),
        (s.hd_vcmax, "hd_vcmax"),
        (s.entropy_vcmax, "entropy_vcmax"),
        (s.ha_jmax, "ha_jmax"),
        (s.hd_jmax, "hd_jmax"),
        (s.entropy_jmax, "entropy_jmax"),
        (s.ha_kc, "ha_kc"),
        (s.ha_ko, "ha_ko"),
        (s.ha_gamma, "ha_gamma"),
    ] {
        finite_positive(value, field)?;
    }
    if !s.displacement_m.is_finite() || s.displacement_m < 0.0 {
        return Err(VegetationError::Domain("displacement"));
    }
    for (value, field) in [
        (s.leaf_emissivity, "leaf_emissivity"),
        (s.stem_emissivity, "stem_emissivity"),
        (s.wet_surface_emissivity, "wet_surface_emissivity"),
        (s.growth_resp_ratio_g1, "growth_respiration_ratio"),
        (s.livewood_fraction_a4, "livewood_fraction"),
        (s.current_growth_fraction, "current_growth_fraction"),
    ] {
        fraction(value, field)?;
    }
    for p50 in [s.p50_leaf_mm, s.p50_stem_mm, s.p50_root_mm] {
        if !p50.is_finite() || p50 >= 0.0 {
            return Err(VegetationError::Domain("p50"));
        }
    }
    let root_sum: f64 = s.root_layers.iter().map(|r| r.root_fraction).sum();
    let n_sum: f64 = s
        .root_layers
        .iter()
        .map(|r| r.mineral_n_root_fraction)
        .sum();
    if (root_sum - 1.0).abs() > 1e-12 || (n_sum - 1.0).abs() > 1e-12 {
        return Err(VegetationError::Domain("root fractions"));
    }
    let mut root_ids = BTreeSet::new();
    for root in &s.root_layers {
        if root.layer_id.trim().is_empty()
            || !root_ids.insert(root.layer_id.as_str())
            || !root.root_fraction.is_finite()
            || root.root_fraction < 0.0
            || !root.mineral_n_root_fraction.is_finite()
            || root.mineral_n_root_fraction < 0.0
            || !root.lateral_root_length_m.is_finite()
            || root.lateral_root_length_m <= 0.0
        {
            return Err(VegetationError::Domain("root layer"));
        }
    }
    for tissue in ["leaf", "fine_root"] {
        let litter = [
            &s.litter_metabolic_fraction,
            &s.litter_cellulose_fraction,
            &s.litter_lignin_fraction,
        ]
        .into_iter()
        .map(|map| map.get(tissue).copied().unwrap_or(f64::NAN))
        .collect::<Vec<_>>();
        if litter
            .iter()
            .any(|value| !value.is_finite() || !(0.0..=1.0).contains(value))
            || (litter.iter().sum::<f64>() - 1.0).abs() > 1e-12
        {
            return Err(VegetationError::Domain("litter partition"));
        }
    }
    if s.phenology_type == PhenologyType::SeasonalDeciduous {
        let on = s
            .gsi_on_threshold
            .ok_or_else(|| VegetationError::Schema("missing gsi_on_threshold".into()))?;
        let off = s
            .gsi_off_threshold
            .ok_or_else(|| VegetationError::Schema("missing gsi_off_threshold".into()))?;
        if !on.is_finite()
            || !off.is_finite()
            || !(0.0..=1.0).contains(&on)
            || !(0.0..=1.0).contains(&off)
        {
            return Err(VegetationError::Domain("GSI thresholds"));
        }
        if on <= off {
            return Err(VegetationError::Domain("GSI thresholds"));
        }
        let onset = s
            .onset_duration_s
            .ok_or_else(|| VegetationError::Schema("missing onset_duration_s".into()))?;
        let offset = s
            .offset_duration_s
            .ok_or_else(|| VegetationError::Schema("missing offset_duration_s".into()))?;
        let hysteresis = s
            .gsi_hysteresis
            .ok_or_else(|| VegetationError::Schema("missing gsi_hysteresis".into()))?;
        finite_positive(onset, "onset_duration_s")?;
        finite_positive(offset, "offset_duration_s")?;
        if !hysteresis.is_finite() || hysteresis < 0.0 || on - off < hysteresis {
            return Err(VegetationError::Domain("GSI hysteresis"));
        }
    } else if s.onset_duration_s.is_some()
        || s.offset_duration_s.is_some()
        || s.gsi_on_threshold.is_some()
        || s.gsi_off_threshold.is_some()
        || s.gsi_hysteresis.is_some()
    {
        return Err(VegetationError::Domain("evergreen phenology nulls"));
    }
    Ok(())
}
