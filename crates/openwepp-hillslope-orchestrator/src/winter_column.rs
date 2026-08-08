//! Typed winter-column state boundary ratified by ADR-0026.
//!
//! This module is intentionally separate from `direct_runtime` phase modules:
//! it owns the future snow/frost lane-state boundary, while direct-runtime
//! phases currently keep their existing behavior until consumer cutover.

use crate::hydrology::SnowAlbedoState;
use crate::runtime_inputs::{DIRECT_WINTER_HOURLY_FORCING_COUNT, DirectWinterHourlyForcing};

pub const DIRECT_WINTER_HOURS_PER_DAY: usize = DIRECT_WINTER_HOURLY_FORCING_COUNT;

#[derive(Debug, Clone, PartialEq)]
pub struct DirectWinterColumnState {
    pub snow: DirectSnowLaneState,
    pub frost: DirectFrostLaneState,
}

impl DirectWinterColumnState {
    #[must_use]
    pub const fn zero() -> Self {
        Self {
            snow: DirectSnowLaneState::zero(),
            frost: DirectFrostLaneState::zero(),
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct DirectSnowLaneState {
    pub runtime_swe_m: f64,
    pub runtime_depth_m: f64,
    pub runtime_density_kg_m3: f64,
    pub runtime_settle_day_count: f64,
    pub coe_boundary_depth_m: f64,
    pub coe_boundary_density_kg_m3: f64,
    pub coe_boundary_settle_day_count: f64,
    pub liquid_water_retained_m: f64,
    pub snow_albedo_state: Option<SnowAlbedoState>,
    pub layers: Vec<DirectSnowLayerState>,
}

#[derive(Debug, Clone, Copy, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct DirectSnowLayerState {
    pub mass_swe_m: f64,
    pub thickness_m: f64,
    pub density_kg_m3: f64,
    pub settle_day_count: f64,
    pub temperature_c: f64,
    pub liquid_water_m: f64,
    pub cold_content_j_m2: f64,
    pub refrozen_liquid_m: f64,
}

impl DirectSnowLayerState {
    #[must_use]
    pub const fn new(
        mass_swe_m: f64,
        thickness_m: f64,
        density_kg_m3: f64,
        settle_day_count: f64,
    ) -> Self {
        Self {
            mass_swe_m,
            thickness_m,
            density_kg_m3,
            settle_day_count,
            temperature_c: 0.0,
            liquid_water_m: 0.0,
            cold_content_j_m2: 0.0,
            refrozen_liquid_m: 0.0,
        }
    }

    #[must_use]
    pub const fn with_stage3_thermal_liquid_state(
        mut self,
        temperature_c: f64,
        liquid_water_m: f64,
        cold_content_j_m2: f64,
        refrozen_liquid_m: f64,
    ) -> Self {
        self.temperature_c = temperature_c;
        self.liquid_water_m = liquid_water_m;
        self.cold_content_j_m2 = cold_content_j_m2;
        self.refrozen_liquid_m = refrozen_liquid_m;
        self
    }
}

impl DirectSnowLaneState {
    #[must_use]
    pub const fn zero() -> Self {
        Self {
            runtime_swe_m: 0.0,
            runtime_depth_m: 0.0,
            runtime_density_kg_m3: 0.0,
            runtime_settle_day_count: 0.0,
            coe_boundary_depth_m: 0.0,
            coe_boundary_density_kg_m3: 0.0,
            coe_boundary_settle_day_count: 0.0,
            liquid_water_retained_m: 0.0,
            snow_albedo_state: None,
            layers: Vec::new(),
        }
    }

    #[must_use]
    pub const fn from_runtime_values(
        runtime_swe_m: f64,
        runtime_depth_m: f64,
        runtime_density_kg_m3: f64,
        runtime_settle_day_count: f64,
    ) -> Self {
        Self {
            runtime_swe_m,
            runtime_depth_m,
            runtime_density_kg_m3,
            runtime_settle_day_count,
            coe_boundary_depth_m: runtime_depth_m,
            coe_boundary_density_kg_m3: runtime_density_kg_m3,
            coe_boundary_settle_day_count: runtime_settle_day_count,
            liquid_water_retained_m: 0.0,
            snow_albedo_state: None,
            layers: Vec::new(),
        }
    }

    #[must_use]
    pub const fn from_runtime_values_and_albedo_state(
        runtime_swe_m: f64,
        runtime_depth_m: f64,
        runtime_density_kg_m3: f64,
        runtime_settle_day_count: f64,
        snow_albedo_state: Option<SnowAlbedoState>,
    ) -> Self {
        Self {
            runtime_swe_m,
            runtime_depth_m,
            runtime_density_kg_m3,
            runtime_settle_day_count,
            coe_boundary_depth_m: runtime_depth_m,
            coe_boundary_density_kg_m3: runtime_density_kg_m3,
            coe_boundary_settle_day_count: runtime_settle_day_count,
            liquid_water_retained_m: 0.0,
            snow_albedo_state,
            layers: Vec::new(),
        }
    }

    #[must_use]
    #[allow(clippy::too_many_arguments)]
    pub const fn from_runtime_values_boundary_and_albedo_state(
        runtime_swe_m: f64,
        runtime_depth_m: f64,
        runtime_density_kg_m3: f64,
        runtime_settle_day_count: f64,
        coe_boundary_depth_m: f64,
        coe_boundary_density_kg_m3: f64,
        coe_boundary_settle_day_count: f64,
        snow_albedo_state: Option<SnowAlbedoState>,
    ) -> Self {
        Self {
            runtime_swe_m,
            runtime_depth_m,
            runtime_density_kg_m3,
            runtime_settle_day_count,
            coe_boundary_depth_m,
            coe_boundary_density_kg_m3,
            coe_boundary_settle_day_count,
            liquid_water_retained_m: 0.0,
            snow_albedo_state,
            layers: Vec::new(),
        }
    }

    #[must_use]
    #[allow(clippy::too_many_arguments)]
    pub const fn from_runtime_values_boundary_liquid_and_albedo_state(
        runtime_swe_m: f64,
        runtime_depth_m: f64,
        runtime_density_kg_m3: f64,
        runtime_settle_day_count: f64,
        coe_boundary_depth_m: f64,
        coe_boundary_density_kg_m3: f64,
        coe_boundary_settle_day_count: f64,
        liquid_water_retained_m: f64,
        snow_albedo_state: Option<SnowAlbedoState>,
    ) -> Self {
        Self {
            runtime_swe_m,
            runtime_depth_m,
            runtime_density_kg_m3,
            runtime_settle_day_count,
            coe_boundary_depth_m,
            coe_boundary_density_kg_m3,
            coe_boundary_settle_day_count,
            liquid_water_retained_m,
            snow_albedo_state,
            layers: Vec::new(),
        }
    }

    #[must_use]
    #[allow(clippy::too_many_arguments)]
    pub fn from_runtime_values_boundary_liquid_albedo_and_layers(
        runtime_swe_m: f64,
        runtime_depth_m: f64,
        runtime_density_kg_m3: f64,
        runtime_settle_day_count: f64,
        coe_boundary_depth_m: f64,
        coe_boundary_density_kg_m3: f64,
        coe_boundary_settle_day_count: f64,
        liquid_water_retained_m: f64,
        snow_albedo_state: Option<SnowAlbedoState>,
        layers: Vec<DirectSnowLayerState>,
    ) -> Self {
        Self {
            runtime_swe_m,
            runtime_depth_m,
            runtime_density_kg_m3,
            runtime_settle_day_count,
            coe_boundary_depth_m,
            coe_boundary_density_kg_m3,
            coe_boundary_settle_day_count,
            liquid_water_retained_m,
            snow_albedo_state,
            layers,
        }
    }

    #[must_use]
    pub fn has_runtime_state(&self) -> bool {
        self.runtime_swe_m > 0.0
            || self.runtime_depth_m > 0.0
            || self.runtime_density_kg_m3 > 0.0
            || self.runtime_settle_day_count > 0.0
            || self.coe_boundary_depth_m > 0.0
            || self.coe_boundary_density_kg_m3 > 0.0
            || self.coe_boundary_settle_day_count > 0.0
            || self.snow_albedo_state.is_some()
            || !self.layers.is_empty()
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct DirectFrostLaneState {
    pub active_frost_coupling: bool,
    pub dfrost_m: f64,
    pub dthaw_m: f64,
    pub nft: f64,
    pub ws_frz_m: f64,
    pub infcap_frz_m_s: f64,
    pub frwatc_soil_water_before_m: f64,
    pub frwatc_soil_water_after_m: f64,
    pub frwatc_frozen_water_before_m: f64,
    pub frwatc_frozen_water_after_m: f64,
    pub frwatc_freeze_debit_m: f64,
    pub frwatc_thaw_credit_m: f64,
    pub frwatc_net_liquid_delta_m: f64,
    pub frdp_m: f64,
    pub thdp_m: f64,
    pub tfrdp_m: f64,
    pub tthawd_m: f64,
    pub fgthwd_flag: f64,
    pub total_fine_layer_count: f64,
    pub conductivity_tilled_w_m_k: f64,
    pub conductivity_untilled_w_m_k: f64,
    pub conductivity_residue_w_m_k: f64,
    pub shadow_total_water_before_m: f64,
    pub shadow_total_water_after_m: f64,
    pub shadow_wb_delta_m: f64,
    pub shadow_frwatc_residual_m: f64,
    pub watpdg_m: f64,
    pub watbtm_m: f64,
    pub layer_shadows: Vec<DirectFrostLayerShadowState>,
    pub fine_layers: Vec<DirectFrostFineLayerState>,
}

impl DirectFrostLaneState {
    #[must_use]
    pub const fn zero() -> Self {
        Self {
            active_frost_coupling: false,
            dfrost_m: 0.0,
            dthaw_m: 0.0,
            nft: 0.0,
            ws_frz_m: 0.0,
            infcap_frz_m_s: 0.0,
            frwatc_soil_water_before_m: 0.0,
            frwatc_soil_water_after_m: 0.0,
            frwatc_frozen_water_before_m: 0.0,
            frwatc_frozen_water_after_m: 0.0,
            frwatc_freeze_debit_m: 0.0,
            frwatc_thaw_credit_m: 0.0,
            frwatc_net_liquid_delta_m: 0.0,
            frdp_m: 0.0,
            thdp_m: 0.0,
            tfrdp_m: 0.0,
            tthawd_m: 0.0,
            fgthwd_flag: 0.0,
            total_fine_layer_count: 0.0,
            conductivity_tilled_w_m_k: 0.0,
            conductivity_untilled_w_m_k: 0.0,
            conductivity_residue_w_m_k: 0.0,
            shadow_total_water_before_m: 0.0,
            shadow_total_water_after_m: 0.0,
            shadow_wb_delta_m: 0.0,
            shadow_frwatc_residual_m: 0.0,
            watpdg_m: 0.0,
            watbtm_m: 0.0,
            layer_shadows: Vec::new(),
            fine_layers: Vec::new(),
        }
    }

    #[must_use]
    pub fn has_runtime_state(&self) -> bool {
        self.active_frost_coupling
            || self.dfrost_m != 0.0
            || self.dthaw_m != 0.0
            || self.nft != 0.0
            || self.ws_frz_m != 0.0
            || self.infcap_frz_m_s != 0.0
            || self.frwatc_soil_water_before_m != 0.0
            || self.frwatc_soil_water_after_m != 0.0
            || self.frwatc_frozen_water_before_m != 0.0
            || self.frwatc_frozen_water_after_m != 0.0
            || self.frwatc_freeze_debit_m != 0.0
            || self.frwatc_thaw_credit_m != 0.0
            || self.frwatc_net_liquid_delta_m != 0.0
            || self.frdp_m != 0.0
            || self.thdp_m != 0.0
            || self.tfrdp_m != 0.0
            || self.tthawd_m != 0.0
            || self.fgthwd_flag != 0.0
            || self.total_fine_layer_count != 0.0
            || self.conductivity_tilled_w_m_k != 0.0
            || self.conductivity_untilled_w_m_k != 0.0
            || self.conductivity_residue_w_m_k != 0.0
            || self.shadow_total_water_before_m != 0.0
            || self.shadow_total_water_after_m != 0.0
            || self.shadow_wb_delta_m != 0.0
            || self.shadow_frwatc_residual_m != 0.0
            || self.watpdg_m != 0.0
            || self.watbtm_m != 0.0
            || !self.layer_shadows.is_empty()
            || !self.fine_layers.is_empty()
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct DirectFrostLayerShadowState {
    pub layer_index: usize,
    pub st_m: f64,
    pub soil_water_m: f64,
    pub frozen_depth_m: f64,
    pub frozen_water_m: f64,
    pub soilf_m: f64,
    pub yst_m: f64,
    pub nwfrzz_m: f64,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct DirectFrostFineLayerState {
    pub layer_index: usize,
    pub fine_index: usize,
    pub fgfrst: f64,
    pub slfsd_m: f64,
    pub slsic_m: f64,
    pub slsw_theta: f64,
    pub sltime_s: f64,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct DirectWinterDayForcing {
    pub hourly: [DirectWinterHourlyForcing; DIRECT_WINTER_HOURS_PER_DAY],
}

impl DirectWinterDayForcing {
    #[must_use]
    pub const fn zero() -> Self {
        Self {
            hourly: [DirectWinterHourlyForcing::zero(); DIRECT_WINTER_HOURS_PER_DAY],
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct DirectWinterDayOutcome {
    pub snow: DirectWinterSnowOutcome,
    pub frost: DirectWinterFrostOutcome,
    pub storage: DirectWinterStorageOutcome,
    pub publication: DirectWinterPublicationOutcome,
}

impl DirectWinterDayOutcome {
    #[must_use]
    pub const fn zero() -> Self {
        Self {
            snow: DirectWinterSnowOutcome::zero(),
            frost: DirectWinterFrostOutcome::zero(),
            storage: DirectWinterStorageOutcome::zero(),
            publication: DirectWinterPublicationOutcome::zero(),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct DirectWinterSnowOutcome {
    pub active_snow_coupling: bool,
    pub snow_coupling_m: f64,
    pub routed_melt_m: f64,
    pub post_winter_rain_m: f64,
    pub runtime_swe_after_m: f64,
    pub runtime_depth_after_m: f64,
    pub runtime_density_after_kg_m3: f64,
    pub runtime_settle_day_count_after: f64,
}

impl DirectWinterSnowOutcome {
    #[must_use]
    pub const fn zero() -> Self {
        Self {
            active_snow_coupling: false,
            snow_coupling_m: 0.0,
            routed_melt_m: 0.0,
            post_winter_rain_m: 0.0,
            runtime_swe_after_m: 0.0,
            runtime_depth_after_m: 0.0,
            runtime_density_after_kg_m3: 0.0,
            runtime_settle_day_count_after: 0.0,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct DirectWinterFrostOutcome {
    pub active_frost_coupling: bool,
    pub frozen_infiltration_capacity_m_s: f64,
    pub frost_depth_after_m: f64,
    pub frozen_water_after_m: f64,
    pub front_dthaw_after_m: f64,
}

impl DirectWinterFrostOutcome {
    #[must_use]
    pub const fn zero() -> Self {
        Self {
            active_frost_coupling: false,
            frozen_infiltration_capacity_m_s: 0.0,
            frost_depth_after_m: 0.0,
            frozen_water_after_m: 0.0,
            front_dthaw_after_m: 0.0,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct DirectWinterStorageOutcome {
    pub frwatc_net_liquid_delta_m: f64,
    pub watpdg_m: f64,
    pub watbtm_m: f64,
    pub closure_residual_m: f64,
}

impl DirectWinterStorageOutcome {
    #[must_use]
    pub const fn zero() -> Self {
        Self {
            frwatc_net_liquid_delta_m: 0.0,
            watpdg_m: 0.0,
            watbtm_m: 0.0,
            closure_residual_m: 0.0,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct DirectWinterPublicationOutcome {
    pub snow_water_m: f64,
    pub frozen_soil_water_m: f64,
    pub frost_depth_m: f64,
    pub rm_m: f64,
}

impl DirectWinterPublicationOutcome {
    #[must_use]
    pub const fn zero() -> Self {
        Self {
            snow_water_m: 0.0,
            frozen_soil_water_m: 0.0,
            frost_depth_m: 0.0,
            rm_m: 0.0,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn zero_winter_column_state_is_inert() {
        let state = DirectWinterColumnState::zero();

        assert!(state.snow.runtime_swe_m.abs() <= f64::EPSILON);
        assert!(state.snow.runtime_depth_m.abs() <= f64::EPSILON);
        assert!(!state.frost.active_frost_coupling);
        assert!(state.frost.dfrost_m.abs() <= f64::EPSILON);
        assert!(state.frost.layer_shadows.is_empty());
        assert!(state.frost.fine_layers.is_empty());
    }

    #[test]
    fn zero_winter_day_outcome_is_inert() {
        let outcome = DirectWinterDayOutcome::zero();

        assert!(!outcome.snow.active_snow_coupling);
        assert!(!outcome.frost.active_frost_coupling);
        assert!(outcome.storage.closure_residual_m.abs() <= f64::EPSILON);
        assert!(outcome.publication.rm_m.abs() <= f64::EPSILON);
    }
}
