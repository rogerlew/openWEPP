use openwepp_hillslope_orchestrator::{
    HillslopePhase, HillslopeWritebackSurface, Wb11HydrologyKernel,
    hillslope_consumer_adapter_for_phase,
};
use openwepp_kernel_contract::{
    BoundarySymbol, BoundaryValue, HillslopeKernel, HillslopeKernelPhaseClass,
    HillslopeKernelRequest, KernelRunResponse, WritebackField,
};

const TOL: f64 = 1.0e-12;

fn insert_state(surface: &mut HillslopeWritebackSurface, symbol: &str, value: f64) {
    surface
        .state_surface
        .insert(BoundarySymbol::from(symbol), BoundaryValue::scalar(value));
}

fn writeback_state(response: &KernelRunResponse, symbol: &str) -> f64 {
    let target = BoundarySymbol::from(symbol);
    response
        .writeback
        .state_updates
        .iter()
        .find_map(|field: &WritebackField| {
            if field.symbol == target {
                Some(field.value.as_f64())
            } else {
                None
            }
        })
        .unwrap_or_else(|| panic!("missing state writeback symbol {symbol}"))
}

fn run_wb17_phase(surface: &HillslopeWritebackSurface) -> KernelRunResponse {
    let request = HillslopeKernelRequest::with_phase_context(
        HillslopePhase::Evapotranspiration.as_str(),
        HillslopeKernelPhaseClass::HydrologyEvapotranspiration,
        hillslope_consumer_adapter_for_phase(HillslopePhase::Evapotranspiration),
        None,
        &surface.state_surface,
        &surface.flux_surface,
    );
    let mut kernel = Wb11HydrologyKernel;
    kernel.run_hillslope_phase(&request)
}

fn base_surface() -> HillslopeWritebackSurface {
    let mut surface = HillslopeWritebackSurface {
        state_surface: std::collections::BTreeMap::new(),
        flux_surface: std::collections::BTreeMap::new(),
    };

    for (symbol, value) in [
        ("nsl", 2.0),
        ("dg", 0.1),
        ("thetdr", 0.0),
        ("thetfc", 0.3),
        ("ssc", 0.10),
        ("wb11_soil_water", 0.16),
        ("wb11_et_demand", 0.0),
        ("cancov", 0.0),
        ("lai", 0.0),
        ("vdmt", 0.0),
        ("wb17_residue_interception", 0.0),
        ("wb18_perc_theta_0001", 0.04),
        ("wb18_perc_theta_0002", 0.12),
        ("wb18_perc_ul_0001", 0.08),
        ("wb18_perc_ul_0002", 0.08),
        ("dg_0001", 0.1),
        ("dg_0002", 0.1),
        ("thetdr_0001", 0.0),
        ("thetdr_0002", 0.0),
    ] {
        insert_state(&mut surface, symbol, value);
    }

    surface
}

fn add_same_pass_outside_water(surface: &mut HillslopeWritebackSurface) {
    for (symbol, value) in [
        ("s1", 0.0),
        ("s2", 0.0),
        ("tu", 0.006),
        ("tv", 0.0),
        ("wb12_rainfall_input", 0.002),
        ("wb12_infiltration", 0.0),
        ("ninten", 2.0),
        ("timem_0001", 0.0),
        ("timem_0002", 1.0),
        ("intsty_0001", 0.002),
        ("intsty_0002", 0.0),
        ("timep", 0.25),
        ("efflen", 1.0),
        ("ealpha", 1.0),
        ("m", 1.5),
        ("snow.options.snow_file_present", 0.0),
        ("snow.runtime_swe", 0.0),
        ("snow.runtime_depth_m", 0.0),
        ("snow.runtime_density_kg_m3", 0.0),
        ("snow.runtime_settle_day_count", 0.0),
        ("wb18_perc_frzw_0001", 0.0),
        ("wb18_perc_frzw_0002", 0.02),
    ] {
        insert_state(surface, symbol, value);
    }
}

#[test]
fn hphys0286_contract_post_et_lower_layer_excess_moves_upward() {
    let response = run_wb17_phase(&base_surface());

    assert_eq!(response.status.message_id(), "HKERNEL-WB11-ET-OK-001");

    let theta_1 = writeback_state(&response, "wb18_perc_theta_0001");
    let theta_2 = writeback_state(&response, "wb18_perc_theta_0002");
    let soil_water = writeback_state(&response, "wb11_soil_water");

    assert!(
        (theta_1 - 0.08).abs() <= TOL,
        "lower-layer excess above UL should move to layer 1; theta_1={theta_1}"
    );
    assert!(
        (theta_2 - 0.08).abs() <= TOL,
        "layer 2 should be capped at UL without outside water; theta_2={theta_2}"
    );
    assert!(
        (soil_water - 0.16).abs() <= TOL,
        "post-ET redistribution should conserve aggregate layer storage; soil_water={soil_water}"
    );
}

#[test]
fn hphys0286_contract_same_pass_water_uses_frozen_adjusted_upper_cap() {
    let mut surface = base_surface();
    add_same_pass_outside_water(&mut surface);

    let response = run_wb17_phase(&surface);

    assert_eq!(response.status.message_id(), "HKERNEL-WB11-ET-OK-001");

    let theta_1 = writeback_state(&response, "wb18_perc_theta_0001");
    let theta_2 = writeback_state(&response, "wb18_perc_theta_0002");
    let infiltration = writeback_state(&response, "wb12_infiltration");

    assert!(
        infiltration > 1.0e-6,
        "same-pass outside-water lineage must be active; infiltration={infiltration}"
    );
    assert!(
        (theta_2 - 0.06).abs() <= TOL,
        "outside-water branch should cap layer 2 at UL-frzw; theta_2={theta_2}"
    );
    assert!(
        (theta_1 - 0.10).abs() <= TOL,
        "outside-water branch should move frozen-adjusted excess upward; theta_1={theta_1}"
    );
}
