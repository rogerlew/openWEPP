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
        for s in &self.strata {
            if !ids.insert(s.stratum_id.as_str()) || s.lifeform != "C3_WOODY" {
                return Err(VegetationError::Unsupported(
                    "lifeform or duplicate stratum",
                ));
            }
            validate_stratum(s, &tiles)?;
        }
        Ok(())
    }

    pub fn canonical_sha256(&self) -> Result<String, VegetationError> {
        let bytes = serde_json::to_vec(self).map_err(|e| VegetationError::Schema(e.to_string()))?;
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
fn validate_stratum(
    s: &StratumConfiguration,
    tiles: &BTreeSet<&str>,
) -> Result<(), VegetationError> {
    for id in &s.tile_ids {
        if !tiles.contains(id.as_str()) {
            return Err(VegetationError::Domain("stratum tile membership"));
        }
    }
    finite_positive(s.height_m, "height_m")?;
    finite_positive(s.leaf_dimension_m, "leaf_dimension_m")?;
    finite_positive(s.sla_m2_per_kg_c, "sla")?;
    finite_positive(s.clumping_index, "clumping_index")?;
    if !(-0.4..=0.6).contains(&s.leaf_angle_chi)
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
    fraction(s.livewood_fraction_a4, "a4")?;
    fraction(s.current_growth_fraction, "current_growth_fraction")?;
    fraction(s.nh4_request_fraction, "nh4_request_fraction")?;
    fraction(s.drymatter_carbon_fraction, "drymatter_carbon_fraction")?;
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
    if s.phenology_type == PhenologyType::SeasonalDeciduous {
        let on = s
            .gsi_on_threshold
            .ok_or_else(|| VegetationError::Schema("missing gsi_on_threshold".into()))?;
        let off = s
            .gsi_off_threshold
            .ok_or_else(|| VegetationError::Schema("missing gsi_off_threshold".into()))?;
        if on <= off {
            return Err(VegetationError::Domain("GSI thresholds"));
        }
    }
    Ok(())
}
