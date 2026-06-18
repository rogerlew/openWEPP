#[allow(clippy::wildcard_imports)]
use crate::hydrology::*;

struct Wb19LateralInputs {
    soil_water_before: f64,
    q_drainage: Option<f64>,
    avgslp_symbol: BoundarySymbol,
    avgslp: f64,
    slplen: f64,
    anisotropy: f64,
    soldep: f64,
}

struct Wb19LaneConfig {
    solwpv_mode: i32,
    solwpv_mode_lt_2006: bool,
    mofe_hourly_carry_arrays_enabled: bool,
    lane_substeps: usize,
    lane_substeps_f64: f64,
    daily_lateral_lane: bool,
}

struct Wb19LateralLayerParameters {
    field_capacity_store: Vec<f64>,
    porosity: Vec<f64>,
    field_capacity_theta: Vec<f64>,
    coca: Vec<f64>,
}

struct Wb19LateralLayerState {
    theta: Vec<f64>,
    drain_threshold: Vec<f64>,
    conductivity: Vec<f64>,
    thickness: Vec<f64>,
    upper_limit: Vec<f64>,
    lateral_conductivity: Vec<f64>,
    lateral_withdrawal_threshold: Vec<f64>,
    frozen_water: Vec<f64>,
    top_effective_upper_limit: Option<f64>,
    parameters: Wb19LateralLayerParameters,
}

struct Wb19LateralActiveLayers {
    capacity_active_layer: Vec<bool>,
    conductivity_active_layer: Vec<bool>,
}

#[derive(Default)]
struct Wb19LateralSubstepMetrics {
    fcdep_before: f64,
    conductivity_depth_sum: f64,
    saturated_depth_sum: f64,
    avpora: f64,
    avfca: f64,
    avcoca: f64,
    lateral_capacity_tdv: f64,
    legacy_saturation_fraction: f64,
}

struct Wb19LateralDepths {
    watyld: f64,
    fcdep_after: f64,
    unsdep_after: f64,
}

struct Wb19LateralRunResult {
    theta: Vec<f64>,
    lateral_withdrawal_threshold: Vec<f64>,
    q_lateral: f64,
    q_lateral_potential_total: f64,
    q_lateral_target_total: f64,
    lateral_capacity_tdv_total: f64,
    watyld: f64,
    fcdep_after: f64,
    unsdep_after: f64,
    lateral_layer_withdrawal: Vec<f64>,
    lateral_capacity_active_count: Vec<f64>,
    lateral_conductivity_active_count: Vec<f64>,
    q_lateral_substeps: Vec<f64>,
    surface_saturation_substeps: Vec<f64>,
}

struct Wb19LateralRunAccumulator {
    q_lateral: f64,
    q_lateral_potential_total: f64,
    q_lateral_target_total: f64,
    lateral_capacity_tdv_total: f64,
    watyld: f64,
    fcdep_after: f64,
    unsdep_after: f64,
    lateral_layer_withdrawal: Vec<f64>,
    lateral_capacity_active_count: Vec<f64>,
    lateral_conductivity_active_count: Vec<f64>,
    q_lateral_substeps: Vec<f64>,
    surface_saturation_substeps: Vec<f64>,
}

struct Wb19DrainageInputs {
    soil_water_before: f64,
    drainage_capacity: f64,
    q_lateral: f64,
    drain_enabled: bool,
    lane_substeps: usize,
    lane_hour_fraction: f64,
}

struct Wb19DrainageGeometry {
    drain_depth_symbol: BoundarySymbol,
    drain_depth: f64,
    drain_spacing_symbol: BoundarySymbol,
    drain_spacing: f64,
    drain_diameter_symbol: BoundarySymbol,
    drain_diameter: f64,
    soldep_symbol: BoundarySymbol,
    soldep: f64,
}

struct Wb19DrainagePotential {
    q_drainage_potential: f64,
    tile_layer_index: usize,
}

struct Wb19DrainageRunResult {
    theta: Vec<f64>,
    q_drainage: f64,
    q_drainage_target_total: f64,
}

struct Wb19DrainageLayerSlices<'a> {
    theta: &'a [f64],
    drain_threshold: &'a [f64],
    conductivity: &'a [f64],
    thickness: &'a [f64],
}

#[derive(Default)]
struct Wb14KsatadjMetricSums {
    theta_sum: f64,
    ul_sum: f64,
    fc_sum: f64,
    thetfc_sum: f64,
    thetdr_sum: f64,
    dg_sum: f64,
    use_legacy_ksatadj_theta_derivation: bool,
}

struct Wb14KsatadjLayerMetrics {
    theta_symbol: BoundarySymbol,
    fc_symbol: BoundarySymbol,
    ul_symbol: BoundarySymbol,
    dg_symbol: BoundarySymbol,
    theta: f64,
    fc: f64,
    ul: f64,
    dg: f64,
    thetdr_optional: Option<(BoundarySymbol, f64)>,
}

include!("hydrology_phase_lateral_drainage/00_lateral_transfer.rs");
include!("hydrology_phase_lateral_drainage/01_tile_drainage.rs");
include!("hydrology_phase_lateral_drainage/02_ksat_adjustment.rs");
