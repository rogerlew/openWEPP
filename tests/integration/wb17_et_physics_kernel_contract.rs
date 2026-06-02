use openwepp_hillslope_orchestrator::{
    HillslopePhase, HillslopePhaseGraph, HillslopePhaseScheduler, HillslopeWritebackSurface,
    Wb11HydrologyKernel, hillslope_consumer_adapter_for_phase,
};
use openwepp_kernel_contract::{
    BoundarySymbol, BoundaryValue, HillslopeKernel, HillslopeKernelPhaseClass,
    HillslopeKernelRequest, KernelRunResponse, WritebackField,
};
use openwepp_sim_contract::status::BoundaryClass;
use openwepp_topology::{parse_topology_fixture_str, validate_pre_execution_topology};

const VALID_TOPOLOGY: &str = r"
HILLSLOPES 3
CHANNELS 2
IMPOUNDMENTS 1
NODE CHANNEL 1 H 1 2 0 C 0 0 0 I 0 0 0
NODE CHANNEL 2 H 3 0 0 C 1 0 0 I 0 0 0
NODE IMPOUNDMENT 1 H 0 0 0 C 2 0 0 I 0 0 0
";

const TOL: f64 = 1.0e-12;

#[allow(clippy::too_many_lines)]
fn seeded_wb17_surface() -> HillslopeWritebackSurface {
    let mut state_surface = std::collections::BTreeMap::new();

    // Seed required consumer-boundary symbols.
    state_surface.insert(BoundarySymbol::from("nsl"), BoundaryValue::scalar(2.0));
    state_surface.insert(BoundarySymbol::from("solthk"), BoundaryValue::scalar(0.3));
    state_surface.insert(
        BoundarySymbol::from("solwpv"),
        BoundaryValue::scalar(2006.0),
    );
    state_surface.insert(BoundarySymbol::from("dg"), BoundaryValue::scalar(0.1));
    state_surface.insert(BoundarySymbol::from("thetdr"), BoundaryValue::scalar(0.1));
    state_surface.insert(BoundarySymbol::from("thetfc"), BoundaryValue::scalar(0.3));
    state_surface.insert(BoundarySymbol::from("ssc"), BoundaryValue::scalar(2.0));
    state_surface.insert(BoundarySymbol::from("cancov"), BoundaryValue::scalar(0.0));
    state_surface.insert(BoundarySymbol::from("lai"), BoundaryValue::scalar(0.3));
    state_surface.insert(BoundarySymbol::from("rtd"), BoundaryValue::scalar(0.0));
    state_surface.insert(BoundarySymbol::from("pltol"), BoundaryValue::scalar(0.25));
    state_surface.insert(BoundarySymbol::from("vdmt"), BoundaryValue::scalar(0.0));

    // WB17 ET runtime inputs.
    state_surface.insert(
        BoundarySymbol::from("wb11_soil_water"),
        BoundaryValue::scalar(0.2),
    );
    state_surface.insert(
        BoundarySymbol::from("wb11_et_demand"),
        BoundaryValue::scalar(0.5),
    );
    state_surface.insert(
        BoundarySymbol::from("wb17_residue_interception"),
        BoundaryValue::scalar(0.05),
    );

    // Keep downstream WB11 hydrology phases nominal while preserving ET results.
    state_surface.insert(
        BoundarySymbol::from("wb11_field_capacity"),
        BoundaryValue::scalar(2.0),
    );
    state_surface.insert(
        BoundarySymbol::from("wb11_perc_fraction"),
        BoundaryValue::scalar(0.5),
    );
    // Keep WB18 per-layer state aligned with the low-water WB17 ET vector.
    state_surface.insert(
        BoundarySymbol::from("wb18_perc_theta_0001"),
        BoundaryValue::scalar(0.1),
    );
    state_surface.insert(
        BoundarySymbol::from("wb18_perc_fc_0001"),
        BoundaryValue::scalar(0.1),
    );
    state_surface.insert(
        BoundarySymbol::from("thetfc_0001"),
        BoundaryValue::scalar(1.0),
    );
    state_surface.insert(
        BoundarySymbol::from("thetdr_0001"),
        BoundaryValue::scalar(0.0),
    );
    state_surface.insert(
        BoundarySymbol::from("wb18_perc_ul_0001"),
        BoundaryValue::scalar(8.0),
    );
    state_surface.insert(
        BoundarySymbol::from("wb18_perc_ssc_0001"),
        BoundaryValue::scalar(2.0e-6),
    );
    state_surface.insert(
        BoundarySymbol::from("wb18_perc_theta_0002"),
        BoundaryValue::scalar(0.1),
    );
    state_surface.insert(
        BoundarySymbol::from("wb18_perc_fc_0002"),
        BoundaryValue::scalar(0.1),
    );
    state_surface.insert(
        BoundarySymbol::from("thetfc_0002"),
        BoundaryValue::scalar(1.0),
    );
    state_surface.insert(
        BoundarySymbol::from("thetdr_0002"),
        BoundaryValue::scalar(0.0),
    );
    state_surface.insert(
        BoundarySymbol::from("wb18_perc_ul_0002"),
        BoundaryValue::scalar(8.0),
    );
    state_surface.insert(
        BoundarySymbol::from("wb18_perc_ssc_0002"),
        BoundaryValue::scalar(2.0e-5),
    );
    state_surface.insert(BoundarySymbol::from("dg_0001"), BoundaryValue::scalar(0.1));
    state_surface.insert(BoundarySymbol::from("dg_0002"), BoundaryValue::scalar(0.1));
    state_surface.insert(
        BoundarySymbol::from("por_0001"),
        BoundaryValue::scalar(0.55),
    );
    state_surface.insert(
        BoundarySymbol::from("por_0002"),
        BoundaryValue::scalar(0.55),
    );
    state_surface.insert(BoundarySymbol::from("cpm_0001"), BoundaryValue::scalar(1.0));
    state_surface.insert(
        BoundarySymbol::from("coca_0001"),
        BoundaryValue::scalar(1.0),
    );
    state_surface.insert(BoundarySymbol::from("cpm_0002"), BoundaryValue::scalar(1.0));
    state_surface.insert(
        BoundarySymbol::from("coca_0002"),
        BoundaryValue::scalar(1.0),
    );
    state_surface.insert(BoundarySymbol::from("avgslp"), BoundaryValue::scalar(0.1));
    state_surface.insert(BoundarySymbol::from("slplen"), BoundaryValue::scalar(1.0));
    state_surface.insert(BoundarySymbol::from("nslpts"), BoundaryValue::scalar(2.0));
    state_surface.insert(
        BoundarySymbol::from("xinput_0001"),
        BoundaryValue::scalar(0.0),
    );
    state_surface.insert(
        BoundarySymbol::from("slpinp_0001"),
        BoundaryValue::scalar(0.0),
    );
    state_surface.insert(
        BoundarySymbol::from("wb19_lateral_anisotropy_ratio"),
        BoundaryValue::scalar(39.653_865_297_983_295),
    );
    state_surface.insert(
        BoundarySymbol::from("wb19_drain_enabled"),
        BoundaryValue::scalar(1.0),
    );
    state_surface.insert(
        BoundarySymbol::from("wb19_drain_depth"),
        BoundaryValue::scalar(0.15),
    );
    state_surface.insert(
        BoundarySymbol::from("wb19_drain_spacing"),
        BoundaryValue::scalar(0.285),
    );
    state_surface.insert(
        BoundarySymbol::from("wb19_drain_diameter"),
        BoundaryValue::scalar(0.1),
    );
    state_surface.insert(
        BoundarySymbol::from("wb11_lateral_fraction"),
        BoundaryValue::scalar(0.25),
    );
    state_surface.insert(
        BoundarySymbol::from("wb11_drainage_fraction"),
        BoundaryValue::scalar(0.5),
    );
    state_surface.insert(
        BoundarySymbol::from("wb11_drainage_coefficient"),
        BoundaryValue::scalar(1.0),
    );
    state_surface.insert(
        BoundarySymbol::from("wb11_drainable_storage"),
        BoundaryValue::scalar(2.0),
    );

    // WB12/WB14/WB16 prerequisites for canonical scheduler completion.
    state_surface.insert(
        BoundarySymbol::from("wb12_rainfall_input"),
        BoundaryValue::scalar(4.0),
    );
    state_surface.insert(
        BoundarySymbol::from("wb12_runon_input"),
        BoundaryValue::scalar(1.0),
    );
    state_surface.insert(
        BoundarySymbol::from("wb12_infiltration"),
        BoundaryValue::scalar(0.0),
    );
    state_surface.insert(
        BoundarySymbol::from("wb12_depression_storage_delta"),
        BoundaryValue::scalar(0.5),
    );
    state_surface.insert(
        BoundarySymbol::from("wb12_runoff_observed"),
        BoundaryValue::scalar(0.5),
    );
    state_surface.insert(
        BoundarySymbol::from("wb12_runoff_closure_tolerance"),
        BoundaryValue::scalar(1.0e-6),
    );
    state_surface.insert(BoundarySymbol::from("ninten"), BoundaryValue::scalar(3.0));
    state_surface.insert(
        BoundarySymbol::from("timem_0001"),
        BoundaryValue::scalar(0.0),
    );
    state_surface.insert(
        BoundarySymbol::from("timem_0002"),
        BoundaryValue::scalar(1.0),
    );
    state_surface.insert(
        BoundarySymbol::from("timem_0003"),
        BoundaryValue::scalar(2.0),
    );
    state_surface.insert(
        BoundarySymbol::from("intsty_0001"),
        BoundaryValue::scalar(2.0),
    );
    state_surface.insert(
        BoundarySymbol::from("intsty_0002"),
        BoundaryValue::scalar(2.0),
    );
    state_surface.insert(
        BoundarySymbol::from("intsty_0003"),
        BoundaryValue::scalar(0.0),
    );
    state_surface.insert(BoundarySymbol::from("timep"), BoundaryValue::scalar(0.25));
    state_surface.insert(BoundarySymbol::from("efflen"), BoundaryValue::scalar(2.0));
    state_surface.insert(BoundarySymbol::from("ealpha"), BoundaryValue::scalar(1.0));
    state_surface.insert(BoundarySymbol::from("m"), BoundaryValue::scalar(1.5));
    state_surface.insert(
        BoundarySymbol::from("wb12_storage_initial"),
        BoundaryValue::scalar(12.0),
    );
    state_surface.insert(
        BoundarySymbol::from("wb12_storage_observed"),
        BoundaryValue::scalar(11.85),
    );
    state_surface.insert(
        BoundarySymbol::from("wb12_storage_closure_tolerance"),
        BoundaryValue::scalar(1.0e-6),
    );
    state_surface.insert(
        BoundarySymbol::from("wb12_precip_input"),
        BoundaryValue::scalar(4.0),
    );

    HillslopeWritebackSurface {
        state_surface,
        flux_surface: std::collections::BTreeMap::new(),
    }
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

fn run_wb17_root_uptake_phase(surface: &HillslopeWritebackSurface) -> KernelRunResponse {
    let request = HillslopeKernelRequest::with_phase_context(
        HillslopePhase::PlantRootUptake.as_str(),
        HillslopeKernelPhaseClass::HydrologyPlantRootUptake,
        hillslope_consumer_adapter_for_phase(HillslopePhase::PlantRootUptake),
        None,
        &surface.state_surface,
        &surface.flux_surface,
    );
    let mut kernel = Wb11HydrologyKernel;
    kernel.run_hillslope_phase(&request)
}

fn state_update_scalar(fields: &[WritebackField], symbol: &str) -> Option<f64> {
    let target = BoundarySymbol::from(symbol);
    fields.iter().find_map(|field| {
        if field.symbol == target {
            Some(field.value.as_f64())
        } else {
            None
        }
    })
}

fn flux_update_scalar(fields: &[WritebackField], symbol: &str) -> Option<f64> {
    state_update_scalar(fields, symbol)
}

fn hphys0251_root_uptake_surface(raw_pltol: f64) -> HillslopeWritebackSurface {
    let mut surface = seeded_wb17_surface();
    surface.state_surface.insert(
        BoundarySymbol::from("wb11_soil_water"),
        BoundaryValue::scalar(0.008),
    );
    surface
        .state_surface
        .insert(BoundarySymbol::from("rtd"), BoundaryValue::scalar(0.20));
    surface.state_surface.insert(
        BoundarySymbol::from("pltol"),
        BoundaryValue::scalar(raw_pltol),
    );
    for layer in 1..=2 {
        surface.state_surface.insert(
            BoundarySymbol::from(format!("wb18_perc_theta_{layer:04}")),
            BoundaryValue::scalar(0.004),
        );
        surface.state_surface.insert(
            BoundarySymbol::from(format!("wb18_perc_ul_{layer:04}")),
            BoundaryValue::scalar(0.02),
        );
        surface.state_surface.insert(
            BoundarySymbol::from(format!("dg_{layer:04}")),
            BoundaryValue::scalar(0.10),
        );
    }
    surface
        .flux_surface
        .insert(BoundarySymbol::from("ET"), BoundaryValue::scalar(0.0));
    surface
        .flux_surface
        .insert(BoundarySymbol::from("Etp"), BoundaryValue::scalar(0.006));
    surface
}

fn hphys0251_effective_pltol(raw_pltol: f64) -> f64 {
    if raw_pltol <= 0.0 {
        0.25
    } else {
        raw_pltol.clamp(0.1, 0.4)
    }
}

fn hphys0251_expected_swu_vectors(raw_pltol: f64) -> ([f64; 2], [f64; 2]) {
    let effective_pltol = hphys0251_effective_pltol(raw_pltol);
    let layer_depths = [0.10_f64, 0.10_f64];
    let layer_storage = [0.004_f64, 0.004_f64];
    let layer_upper_limit = [0.02_f64, 0.02_f64];
    let root_depth = 0.20_f64;
    let transpiration_demand = 0.006_f64;
    let ub = 3.065_f64;
    let uob = 0.953_346_f64;

    let mut potential = [0.0_f64; 2];
    let mut actual = [0.0_f64; 2];
    let mut cumulative_depth = 0.0_f64;
    let mut previous_cumulative_uptake = 0.0_f64;
    for (index, layer_depth) in layer_depths.iter().copied().enumerate() {
        cumulative_depth += layer_depth;
        let rooted_depth = cumulative_depth.min(root_depth);
        let relative_root_depth = rooted_depth / root_depth;
        let cumulative_uptake =
            transpiration_demand * (1.0 - (-ub * relative_root_depth).exp()) / uob;
        potential[index] = (cumulative_uptake - previous_cumulative_uptake).max(0.0);
        previous_cumulative_uptake = cumulative_uptake;

        let stress_threshold = effective_pltol * layer_upper_limit[index];
        actual[index] = potential[index];
        if stress_threshold > 0.0 && layer_storage[index] < stress_threshold {
            actual[index] *= layer_storage[index] / stress_threshold;
        }
        if actual[index] > layer_storage[index] {
            actual[index] = layer_storage[index];
        }
        if actual[index] < 1.0e-10 {
            actual[index] = 0.0;
        }
    }

    (potential, actual)
}

#[test]
fn wb17_contract_conformance_emits_partitioned_et_components() {
    let surface = seeded_wb17_surface();
    let response = run_wb17_phase(&surface);

    let et =
        flux_update_scalar(&response.writeback.flux_updates, "ET").expect("ET should be present");
    let ws =
        flux_update_scalar(&response.writeback.flux_updates, "Ws").expect("Ws should be present");
    let ep =
        flux_update_scalar(&response.writeback.flux_updates, "Ep").expect("Ep should be present");
    let es =
        flux_update_scalar(&response.writeback.flux_updates, "Es").expect("Es should be present");
    let er =
        flux_update_scalar(&response.writeback.flux_updates, "Er").expect("Er should be present");

    assert!((et - 0.15).abs() <= TOL);
    assert!((ws - 1.0).abs() <= TOL);
    assert!(ep.abs() <= TOL);
    assert!((es - 0.1).abs() <= TOL);
    assert!((er - 0.05).abs() <= TOL);

    let soil_water_after =
        state_update_scalar(&response.writeback.state_updates, "wb11_soil_water")
            .expect("wb11_soil_water should be present");
    assert!((soil_water_after - 0.1).abs() <= TOL);
}

#[test]
fn hphys0249_wb17_soil_evaporation_mutates_layer_storage_before_aggregate_writeback() {
    let mut surface = seeded_wb17_surface();
    surface.state_surface.insert(
        BoundarySymbol::from("wb11_soil_water"),
        BoundaryValue::scalar(0.11),
    );
    surface.state_surface.insert(
        BoundarySymbol::from("wb11_et_demand"),
        BoundaryValue::scalar(0.04),
    );
    surface
        .state_surface
        .insert(BoundarySymbol::from("lai"), BoundaryValue::scalar(0.0));
    surface
        .state_surface
        .insert(BoundarySymbol::from("cancov"), BoundaryValue::scalar(0.0));
    surface.state_surface.insert(
        BoundarySymbol::from("wb17_residue_interception"),
        BoundaryValue::scalar(0.0),
    );
    surface.state_surface.insert(
        BoundarySymbol::from("wb18_perc_theta_0001"),
        BoundaryValue::scalar(0.03),
    );
    surface.state_surface.insert(
        BoundarySymbol::from("wb18_perc_theta_0002"),
        BoundaryValue::scalar(0.08),
    );
    surface
        .state_surface
        .insert(BoundarySymbol::from("dg_0001"), BoundaryValue::scalar(0.05));
    surface
        .state_surface
        .insert(BoundarySymbol::from("dg_0002"), BoundaryValue::scalar(0.20));
    surface
        .state_surface
        .insert(BoundarySymbol::from("rtd"), BoundaryValue::scalar(0.0));

    let response = run_wb17_phase(&surface);

    assert_eq!(response.status.message_id(), "HKERNEL-WB11-ET-OK-001");
    let expected_soil_evaporation = 0.04 * (-0.5_f64 * 0.1).exp();
    let expected_layer_1 = 0.0;
    let expected_layer_2 = 0.08 - (expected_soil_evaporation - 0.03);
    let expected_soil_water = expected_layer_1 + expected_layer_2;

    let layer_1_after =
        state_update_scalar(&response.writeback.state_updates, "wb18_perc_theta_0001")
            .expect("WB17 must publish layer 1 storage after soil evaporation");
    let layer_2_after =
        state_update_scalar(&response.writeback.state_updates, "wb18_perc_theta_0002")
            .expect("WB17 must publish layer 2 storage after soil evaporation");
    let soil_water_after =
        state_update_scalar(&response.writeback.state_updates, "wb11_soil_water")
            .expect("WB17 must publish aggregate soil water after layer extraction");
    let es = flux_update_scalar(&response.writeback.flux_updates, "Es")
        .expect("WB17 must publish soil evaporation");

    assert!((layer_1_after - expected_layer_1).abs() <= TOL);
    assert!((layer_2_after - expected_layer_2).abs() <= TOL);
    assert!((soil_water_after - expected_soil_water).abs() <= TOL);
    assert!((es - expected_soil_evaporation).abs() <= TOL);
}

#[test]
fn hphys0249_wb17_soil_evaporation_depth_rationing_cap_limits_partial_layer_withdrawal() {
    let mut surface = seeded_wb17_surface();
    surface.state_surface.insert(
        BoundarySymbol::from("wb11_soil_water"),
        BoundaryValue::scalar(0.11),
    );
    surface.state_surface.insert(
        BoundarySymbol::from("wb11_et_demand"),
        BoundaryValue::scalar(0.20),
    );
    surface
        .state_surface
        .insert(BoundarySymbol::from("lai"), BoundaryValue::scalar(0.0));
    surface
        .state_surface
        .insert(BoundarySymbol::from("cancov"), BoundaryValue::scalar(0.0));
    surface.state_surface.insert(
        BoundarySymbol::from("wb17_residue_interception"),
        BoundaryValue::scalar(0.0),
    );
    surface.state_surface.insert(
        BoundarySymbol::from("wb18_perc_theta_0001"),
        BoundaryValue::scalar(0.03),
    );
    surface.state_surface.insert(
        BoundarySymbol::from("wb18_perc_theta_0002"),
        BoundaryValue::scalar(0.08),
    );
    surface
        .state_surface
        .insert(BoundarySymbol::from("dg_0001"), BoundaryValue::scalar(0.05));
    surface
        .state_surface
        .insert(BoundarySymbol::from("dg_0002"), BoundaryValue::scalar(0.20));
    surface
        .state_surface
        .insert(BoundarySymbol::from("rtd"), BoundaryValue::scalar(0.0));

    let response = run_wb17_phase(&surface);

    assert_eq!(response.status.message_id(), "HKERNEL-WB11-ET-OK-001");
    let potential_soil_evaporation = 0.20 * (-0.5_f64 * 0.1).exp();
    let expected_layer_1 = 0.0;
    let expected_layer_2 = 0.06;
    let expected_soil_evaporation = 0.05;
    let expected_soil_water = expected_layer_1 + expected_layer_2;

    let layer_1_after =
        state_update_scalar(&response.writeback.state_updates, "wb18_perc_theta_0001")
            .expect("WB17 must publish layer 1 storage after depth-capped evaporation");
    let layer_2_after =
        state_update_scalar(&response.writeback.state_updates, "wb18_perc_theta_0002")
            .expect("WB17 must publish layer 2 storage after depth-capped evaporation");
    let soil_water_after =
        state_update_scalar(&response.writeback.state_updates, "wb11_soil_water")
            .expect("WB17 must publish aggregate soil water after depth-capped evaporation");
    let es = flux_update_scalar(&response.writeback.flux_updates, "Es")
        .expect("WB17 must publish depth-capped soil evaporation");

    assert!(potential_soil_evaporation > expected_soil_evaporation);
    assert!((layer_1_after - expected_layer_1).abs() <= TOL);
    assert!((layer_2_after - expected_layer_2).abs() <= TOL);
    assert!((soil_water_after - expected_soil_water).abs() <= TOL);
    assert!((es - expected_soil_evaporation).abs() <= TOL);
}

#[test]
fn hphys0249_wb17_soil_evaporation_aggregate_includes_residual_and_frozen_depth_terms() {
    let mut surface = seeded_wb17_surface();
    surface.state_surface.insert(
        BoundarySymbol::from("wb11_soil_water"),
        BoundaryValue::scalar(0.1138),
    );
    surface.state_surface.insert(
        BoundarySymbol::from("wb11_et_demand"),
        BoundaryValue::scalar(0.04),
    );
    surface
        .state_surface
        .insert(BoundarySymbol::from("lai"), BoundaryValue::scalar(0.0));
    surface
        .state_surface
        .insert(BoundarySymbol::from("cancov"), BoundaryValue::scalar(0.0));
    surface.state_surface.insert(
        BoundarySymbol::from("wb17_residue_interception"),
        BoundaryValue::scalar(0.0),
    );
    surface.state_surface.insert(
        BoundarySymbol::from("wb18_perc_theta_0001"),
        BoundaryValue::scalar(0.03),
    );
    surface.state_surface.insert(
        BoundarySymbol::from("wb18_perc_theta_0002"),
        BoundaryValue::scalar(0.08),
    );
    surface
        .state_surface
        .insert(BoundarySymbol::from("dg_0001"), BoundaryValue::scalar(0.05));
    surface
        .state_surface
        .insert(BoundarySymbol::from("dg_0002"), BoundaryValue::scalar(0.20));
    surface.state_surface.insert(
        BoundarySymbol::from("thetdr_0001"),
        BoundaryValue::scalar(0.01),
    );
    surface.state_surface.insert(
        BoundarySymbol::from("thetdr_0002"),
        BoundaryValue::scalar(0.02),
    );
    surface.state_surface.insert(
        BoundarySymbol::from("wb18_perc_frozen_depth_0001"),
        BoundaryValue::scalar(0.01),
    );
    surface.state_surface.insert(
        BoundarySymbol::from("wb18_perc_frozen_depth_0002"),
        BoundaryValue::scalar(0.03),
    );

    let response = run_wb17_phase(&surface);

    assert_eq!(response.status.message_id(), "HKERNEL-WB11-ET-OK-001");
    let expected_soil_evaporation = 0.04 * (-0.5_f64 * 0.1).exp();
    let expected_layer_1 = 0.0;
    let expected_layer_2 = 0.08 - (expected_soil_evaporation - 0.03);
    let expected_soil_water =
        expected_layer_1 + 0.01 * (0.05 - 0.01) + expected_layer_2 + 0.02 * (0.20 - 0.03);
    let theta_only = expected_layer_1 + expected_layer_2;

    let soil_water_after =
        state_update_scalar(&response.writeback.state_updates, "wb11_soil_water")
            .expect("WB17 must publish aggregate soil water after layer extraction");

    assert!((soil_water_after - expected_soil_water).abs() <= TOL);
    assert!((soil_water_after - theta_only).abs() > TOL);
}

#[test]
fn hphys0249_wb17_residue_remainder_adds_back_to_top_layer_and_clears_interception() {
    let mut surface = seeded_wb17_surface();
    surface.state_surface.insert(
        BoundarySymbol::from("wb11_et_demand"),
        BoundaryValue::scalar(0.02),
    );
    surface
        .state_surface
        .insert(BoundarySymbol::from("lai"), BoundaryValue::scalar(3.0));
    surface.state_surface.insert(
        BoundarySymbol::from("wb17_residue_interception"),
        BoundaryValue::scalar(0.05),
    );
    surface.state_surface.insert(
        BoundarySymbol::from("wb18_perc_theta_0001"),
        BoundaryValue::scalar(0.01),
    );
    surface.state_surface.insert(
        BoundarySymbol::from("wb18_perc_theta_0002"),
        BoundaryValue::scalar(0.08),
    );

    let response = run_wb17_phase(&surface);

    assert_eq!(response.status.message_id(), "HKERNEL-WB11-ET-OK-001");
    let layer_1_after =
        state_update_scalar(&response.writeback.state_updates, "wb18_perc_theta_0001")
            .expect("WB17 must publish top-layer storage after residue add-back");
    let soil_water_after =
        state_update_scalar(&response.writeback.state_updates, "wb11_soil_water")
            .expect("WB17 must publish aggregate soil water after residue add-back");
    let residue_state_after = state_update_scalar(
        &response.writeback.state_updates,
        "wb17_residue_interception",
    )
    .expect("WB17 must clear residue interception after same-day handling");
    let et =
        flux_update_scalar(&response.writeback.flux_updates, "ET").expect("ET should be present");
    let es =
        flux_update_scalar(&response.writeback.flux_updates, "Es").expect("Es should be present");
    let er =
        flux_update_scalar(&response.writeback.flux_updates, "Er").expect("Er should be present");

    assert!((layer_1_after - 0.06).abs() <= TOL);
    assert!((soil_water_after - 0.14).abs() <= TOL);
    assert!(residue_state_after.abs() <= TOL);
    assert!(et.abs() <= TOL);
    assert!(es.abs() <= TOL);
    assert!(er.abs() <= TOL);
}

#[test]
fn hphys0249_wb17_root_uptake_mutates_layer_storage_and_stress_from_swu_lineage() {
    let surface = hphys0251_root_uptake_surface(0.25);

    let response = run_wb17_root_uptake_phase(&surface);

    assert_eq!(response.status.message_id(), "HKERNEL-WB17-SWU-OK-001");
    let expected_layer_1_uptake = 0.003_947_385_293_021_583_f64;
    let expected_layer_2_uptake = 0.000_852_615_503_174_695_3_f64;
    let expected_ep = expected_layer_1_uptake + expected_layer_2_uptake;
    let expected_ws = expected_ep / 0.006;
    let layer_1_after =
        state_update_scalar(&response.writeback.state_updates, "wb18_perc_theta_0001")
            .expect("WB17 must publish layer 1 storage after root uptake");
    let layer_2_after =
        state_update_scalar(&response.writeback.state_updates, "wb18_perc_theta_0002")
            .expect("WB17 must publish layer 2 storage after root uptake");
    let soil_water_after =
        state_update_scalar(&response.writeback.state_updates, "wb11_soil_water")
            .expect("WB17 must publish aggregate soil water after root uptake");
    let ep = flux_update_scalar(&response.writeback.flux_updates, "Ep")
        .expect("WB17 must publish plant transpiration");
    let ws = flux_update_scalar(&response.writeback.flux_updates, "Ws")
        .expect("WB17 must publish water stress");

    assert!((layer_1_after - (0.004 - expected_layer_1_uptake)).abs() <= TOL);
    assert!((layer_2_after - (0.004 - expected_layer_2_uptake)).abs() <= TOL);
    assert!((soil_water_after - (0.008 - expected_ep)).abs() <= TOL);
    assert!((ep - expected_ep).abs() <= TOL);
    assert!((ws - expected_ws).abs() <= TOL);
}

#[test]
fn hphys0251_wb17_root_uptake_normalizes_pltol_like_swu_for() {
    for (raw_pltol, effective_pltol) in [(0.0, 0.25), (0.05, 0.1), (0.45, 0.4)] {
        let surface = hphys0251_root_uptake_surface(raw_pltol);
        let response = run_wb17_root_uptake_phase(&surface);

        assert_eq!(response.status.message_id(), "HKERNEL-WB17-SWU-OK-001");
        let (_, actual) = hphys0251_expected_swu_vectors(raw_pltol);
        let expected_ep: f64 = actual.iter().sum();
        let expected_ws = expected_ep / 0.006;
        let effective_state = state_update_scalar(&response.writeback.state_updates, "pltol")
            .unwrap_or_else(|| panic!("WB17 must publish effective pltol for raw {raw_pltol}"));
        let layer_1_after =
            state_update_scalar(&response.writeback.state_updates, "wb18_perc_theta_0001")
                .unwrap_or_else(|| panic!("WB17 must publish layer 1 storage for raw {raw_pltol}"));
        let layer_2_after =
            state_update_scalar(&response.writeback.state_updates, "wb18_perc_theta_0002")
                .unwrap_or_else(|| panic!("WB17 must publish layer 2 storage for raw {raw_pltol}"));
        let ep = flux_update_scalar(&response.writeback.flux_updates, "Ep")
            .unwrap_or_else(|| panic!("WB17 must publish Ep for raw {raw_pltol}"));
        let ws = flux_update_scalar(&response.writeback.flux_updates, "Ws")
            .unwrap_or_else(|| panic!("WB17 must publish Ws for raw {raw_pltol}"));

        assert!((effective_state - effective_pltol).abs() <= TOL);
        assert!((layer_1_after - (0.004 - actual[0])).abs() <= TOL);
        assert!((layer_2_after - (0.004 - actual[1])).abs() <= TOL);
        assert!((ep - expected_ep).abs() <= TOL);
        assert!((ws - expected_ws).abs() <= TOL);
    }
}

#[test]
fn hphys0251_wb17_root_uptake_publishes_layer_upi_ui_trace() {
    let surface = hphys0251_root_uptake_surface(0.25);
    let response = run_wb17_root_uptake_phase(&surface);

    assert_eq!(response.status.message_id(), "HKERNEL-WB17-SWU-OK-001");
    let (potential, actual) = hphys0251_expected_swu_vectors(0.25);
    let upi = flux_update_scalar(&response.writeback.flux_updates, "UPi")
        .expect("WB17 must publish aggregate potential uptake");
    let ui = flux_update_scalar(&response.writeback.flux_updates, "Ui")
        .expect("WB17 must publish aggregate actual uptake");
    let ep = flux_update_scalar(&response.writeback.flux_updates, "Ep")
        .expect("WB17 must publish final Ep from actual uptake");
    let ws = flux_update_scalar(&response.writeback.flux_updates, "Ws")
        .expect("WB17 must publish final Ws from actual uptake");

    let mut layer_potential_sum = 0.0_f64;
    let mut layer_actual_sum = 0.0_f64;
    for layer in 1..=2 {
        let potential_symbol = format!("UPi_{layer:04}");
        let actual_symbol = format!("Ui_{layer:04}");
        let layer_potential =
            flux_update_scalar(&response.writeback.flux_updates, &potential_symbol)
                .unwrap_or_else(|| panic!("WB17 must publish {potential_symbol}"));
        let layer_actual = flux_update_scalar(&response.writeback.flux_updates, &actual_symbol)
            .unwrap_or_else(|| panic!("WB17 must publish {actual_symbol}"));
        assert!((layer_potential - potential[layer - 1]).abs() <= TOL);
        assert!((layer_actual - actual[layer - 1]).abs() <= TOL);
        assert!(layer_actual <= layer_potential + TOL);
        layer_potential_sum += layer_potential;
        layer_actual_sum += layer_actual;
    }

    assert!((upi - layer_potential_sum).abs() <= TOL);
    assert!((ui - layer_actual_sum).abs() <= TOL);
    assert!((ep - layer_actual_sum).abs() <= TOL);
    assert!((ws - (layer_actual_sum / 0.006)).abs() <= TOL);
}

#[test]
fn wb17_contract_conformance_rejects_missing_residue_interception_symbol() {
    let graph = parse_topology_fixture_str(VALID_TOPOLOGY).expect("fixture should parse");
    let topology_report =
        validate_pre_execution_topology(&graph).expect("topology report should build");
    let scheduler = HillslopePhaseScheduler::canonical();
    let mut kernel = Wb11HydrologyKernel;

    let mut surface = seeded_wb17_surface();
    surface
        .state_surface
        .remove(&BoundarySymbol::from("wb17_residue_interception"));

    let report = scheduler
        .execute_with_kernel(&topology_report, &mut kernel, surface)
        .expect("missing-symbol failure should return typed report");

    assert_eq!(
        report.scheduler_report.halted_phase,
        Some(HillslopePhase::Evapotranspiration)
    );
    let et_phase = report
        .phase_reports
        .iter()
        .find(|phase| phase.phase == HillslopePhase::Evapotranspiration)
        .expect("evapotranspiration phase report should exist");
    assert_eq!(
        et_phase.decision_status.message_id(),
        "HKERNEL-WB11-ET-E-001"
    );
    assert_eq!(
        et_phase.decision_status.boundary_class(),
        BoundaryClass::MissingRequiredInput
    );
}

#[test]
fn wb17_contract_conformance_rejects_non_finite_lai() {
    let graph = parse_topology_fixture_str(VALID_TOPOLOGY).expect("fixture should parse");
    let topology_report =
        validate_pre_execution_topology(&graph).expect("topology report should build");
    let scheduler = HillslopePhaseScheduler::canonical();
    let mut kernel = Wb11HydrologyKernel;

    let mut surface = seeded_wb17_surface();
    surface
        .state_surface
        .insert(BoundarySymbol::from("lai"), BoundaryValue::scalar(f64::NAN));

    let report = scheduler
        .execute_with_kernel(&topology_report, &mut kernel, surface)
        .expect("non-finite failure should return typed report");

    assert_eq!(
        report.scheduler_report.halted_phase,
        Some(HillslopePhase::Evapotranspiration)
    );
    let et_phase = report
        .phase_reports
        .iter()
        .find(|phase| phase.phase == HillslopePhase::Evapotranspiration)
        .expect("evapotranspiration phase report should exist");
    assert_eq!(
        et_phase.decision_status.message_id(),
        "HKERNEL-WB11-ET-E-002"
    );
    assert_eq!(
        et_phase.decision_status.boundary_class(),
        BoundaryClass::NonFinite
    );
}

#[test]
fn wb17_contract_conformance_rejects_domain_invalid_residue_interception() {
    let graph = parse_topology_fixture_str(VALID_TOPOLOGY).expect("fixture should parse");
    let topology_report =
        validate_pre_execution_topology(&graph).expect("topology report should build");
    let scheduler = HillslopePhaseScheduler::canonical();
    let mut kernel = Wb11HydrologyKernel;

    let mut surface = seeded_wb17_surface();
    surface.state_surface.insert(
        BoundarySymbol::from("wb17_residue_interception"),
        BoundaryValue::scalar(-0.01),
    );

    let report = scheduler
        .execute_with_kernel(&topology_report, &mut kernel, surface)
        .expect("domain failure should return typed report");

    assert_eq!(
        report.scheduler_report.halted_phase,
        Some(HillslopePhase::Evapotranspiration)
    );
    let et_phase = report
        .phase_reports
        .iter()
        .find(|phase| phase.phase == HillslopePhase::Evapotranspiration)
        .expect("evapotranspiration phase report should exist");
    assert_eq!(
        et_phase.decision_status.message_id(),
        "HKERNEL-WB11-ET-E-003"
    );
    assert_eq!(
        et_phase.decision_status.boundary_class(),
        BoundaryClass::DomainViolation
    );
}

#[test]
fn hphys0242_contract_wb17_et_executes_after_same_pass_percolation_before_wb19_tail() {
    let ordered = HillslopePhaseGraph::canonical_order();
    let phase_index = |phase| {
        ordered
            .iter()
            .position(|candidate| *candidate == phase)
            .unwrap_or_else(|| panic!("{phase:?} must exist in canonical order"))
    };

    assert!(
        phase_index(HillslopePhase::PercolationDeepSeepage)
            < phase_index(HillslopePhase::Evapotranspiration),
        "HPHYS0242 requires ET to consume same-pass percolation-mutated layer state"
    );
    assert!(
        phase_index(HillslopePhase::Evapotranspiration) < phase_index(HillslopePhase::Drainage),
        "HPHYS0242 requires ET before the hourly WB19 drainage/lateral tail"
    );
    assert!(
        phase_index(HillslopePhase::LateralTransfer) < phase_index(HillslopePhase::PlantRootUptake),
        "HPHYS0249 requires SWU/root uptake after the hourly WB19 drainage/lateral tail"
    );
}
