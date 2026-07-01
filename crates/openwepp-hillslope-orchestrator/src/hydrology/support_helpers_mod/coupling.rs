#[allow(clippy::wildcard_imports)]
use super::super::*;

mod frost;
mod frost_entry;

#[derive(Debug, Clone)]
struct FrostLayerWaterState {
    layer_index: usize,
    fine_layer_count: usize,
    fine_layer_thickness_m: f64,
    dg_m: f64,
    bulk_density_kg_m3: f64,
    thetdr: f64,
    theta_m: f64,
    upper_limit_m: f64,
    frozen_depth_m: f64,
    frzw_m: f64,
}

#[derive(Debug, Clone)]
struct FrostFineLayerState {
    layer_index: usize,
    fine_index: usize,
    fine_layer_thickness_m: f64,
    fgfrst: f64,
    slfsd_m: f64,
    slsic_m: f64,
    slsw_theta: f64,
    sltime_s: f64,
}

#[derive(Debug, Clone)]
struct FrostLayerExchangeState {
    layer_index: usize,
    thetdr: f64,
    st_m: f64,
    yst_m: f64,
    nwfrzz_m: f64,
    frozen_m: f64,
    frzw_m: f64,
    soilf_m: f64,
    soil_water_m: f64,
}

#[derive(Debug, Clone)]
struct FrostFineShadowState {
    fine_layers: Vec<FrostFineLayerState>,
    layer_state: Vec<FrostLayerExchangeState>,
    total_water_before_m: f64,
    total_water_after_m: f64,
    wb_delta_m: f64,
    residual_m: f64,
    watpdg_m: f64,
    watbtm_m: f64,
}

#[derive(Debug, Clone, Copy)]
struct FrostDepthSummary {
    frdp: f64,
    thdp: f64,
    tfrdp: f64,
    tthawd: f64,
}

#[derive(Debug, Clone, Copy)]
struct FrostSeasonalTemperatureCurve {
    annual_mean_c: f64,
    amplitude_c: f64,
    phase_shift_days: f64,
}

#[derive(Debug, Clone, Copy)]
struct ActiveFrostTmpadjContext {
    wind_m_s: f64,
    albedo: f64,
    canopy_height_m: f64,
    random_roughness_m: f64,
}

impl Wb11HydrologyKernel {
}
