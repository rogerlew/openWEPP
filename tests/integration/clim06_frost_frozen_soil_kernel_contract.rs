use openwepp_hillslope_orchestrator::{
    HillslopePhase, HillslopePhaseScheduler, HillslopeWritebackSurface, Wb11HydrologyKernel,
};
use openwepp_kernel_contract::{BoundarySymbol, BoundaryValue};
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

const CLIM06_TEST_TOLERANCE: f64 = 1.0e-6;
const EXPECTED_INF_CAP_FRZ: f64 = 0.1;
const EXPECTED_DFROST: f64 = 0.2;
const EXPECTED_DTHAW: f64 = 0.0;
const EXPECTED_NFT: f64 = 1.0;
const EXPECTED_WS_FRZ: f64 = 0.2;

#[allow(clippy::too_many_lines)]
fn seeded_clim06_surface(active_frost: bool) -> HillslopeWritebackSurface {
    let mut state_surface = std::collections::BTreeMap::new();

    state_surface.insert(BoundarySymbol::from("nsl"), BoundaryValue::scalar(2.0));
    state_surface.insert(BoundarySymbol::from("solthk"), BoundaryValue::scalar(0.3));
    state_surface.insert(BoundarySymbol::from("dg"), BoundaryValue::scalar(2.0));
    state_surface.insert(BoundarySymbol::from("thetdr"), BoundaryValue::scalar(0.0));
    state_surface.insert(BoundarySymbol::from("thetfc"), BoundaryValue::scalar(1.0));
    state_surface.insert(BoundarySymbol::from("ssc"), BoundaryValue::scalar(0.5));
    state_surface.insert(BoundarySymbol::from("cancov"), BoundaryValue::scalar(0.0));
    state_surface.insert(BoundarySymbol::from("lai"), BoundaryValue::scalar(0.0));
    state_surface.insert(BoundarySymbol::from("vdmt"), BoundaryValue::scalar(0.0));

    state_surface.insert(
        BoundarySymbol::from("wb11_soil_water"),
        BoundaryValue::scalar(12.0),
    );
    state_surface.insert(
        BoundarySymbol::from("wb11_et_demand"),
        BoundaryValue::scalar(2.0),
    );
    state_surface.insert(
        BoundarySymbol::from("wb17_residue_interception"),
        BoundaryValue::scalar(0.0),
    );
    state_surface.insert(
        BoundarySymbol::from("wb11_field_capacity"),
        BoundaryValue::scalar(8.0),
    );
    state_surface.insert(
        BoundarySymbol::from("wb11_perc_fraction"),
        BoundaryValue::scalar(0.5),
    );
    // WB18 per-layer percolation inputs (WB11 compatibility lane).
    state_surface.insert(
        BoundarySymbol::from("wb18_perc_theta_0001"),
        BoundaryValue::scalar(5.0),
    );
    state_surface.insert(
        BoundarySymbol::from("wb18_perc_fc_0001"),
        BoundaryValue::scalar(5.0),
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
        BoundaryValue::scalar(5.0),
    );
    state_surface.insert(
        BoundarySymbol::from("wb18_perc_fc_0002"),
        BoundaryValue::scalar(4.0),
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
    state_surface.insert(BoundarySymbol::from("cpm_0001"), BoundaryValue::scalar(1.0));
    state_surface.insert(BoundarySymbol::from("cpm_0002"), BoundaryValue::scalar(1.0));
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

    state_surface.insert(
        BoundarySymbol::from("wb12_rainfall_input"),
        BoundaryValue::scalar(3.0),
    );
    state_surface.insert(
        BoundarySymbol::from("wb12_runon_input"),
        BoundaryValue::scalar(0.4),
    );
    state_surface.insert(
        BoundarySymbol::from("wb12_infiltration"),
        BoundaryValue::scalar(0.0),
    );
    state_surface.insert(
        BoundarySymbol::from("wb12_depression_storage_delta"),
        BoundaryValue::scalar(0.2),
    );
    state_surface.insert(
        BoundarySymbol::from("wb12_runoff_observed"),
        BoundaryValue::scalar(0.0),
    );
    state_surface.insert(
        BoundarySymbol::from("wb12_runoff_closure_tolerance"),
        BoundaryValue::scalar(20.0),
    );

    state_surface.insert(
        BoundarySymbol::from("wb12_storage_initial"),
        BoundaryValue::scalar(12.0),
    );
    state_surface.insert(
        BoundarySymbol::from("wb12_storage_observed"),
        BoundaryValue::scalar(0.0),
    );
    state_surface.insert(
        BoundarySymbol::from("wb12_storage_closure_tolerance"),
        BoundaryValue::scalar(20.0),
    );
    state_surface.insert(
        BoundarySymbol::from("wb12_precip_input"),
        BoundaryValue::scalar(3.0),
    );

    state_surface.insert(BoundarySymbol::from("ninten"), BoundaryValue::scalar(4.0));
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
        BoundarySymbol::from("timem_0004"),
        BoundaryValue::scalar(3.0),
    );
    state_surface.insert(
        BoundarySymbol::from("intsty_0001"),
        BoundaryValue::scalar(1.0),
    );
    state_surface.insert(
        BoundarySymbol::from("intsty_0002"),
        BoundaryValue::scalar(1.0),
    );
    state_surface.insert(
        BoundarySymbol::from("intsty_0003"),
        BoundaryValue::scalar(1.0),
    );
    state_surface.insert(
        BoundarySymbol::from("intsty_0004"),
        BoundaryValue::scalar(0.0),
    );
    state_surface.insert(BoundarySymbol::from("timep"), BoundaryValue::scalar(0.25));
    state_surface.insert(BoundarySymbol::from("efflen"), BoundaryValue::scalar(2.0));
    state_surface.insert(BoundarySymbol::from("ealpha"), BoundaryValue::scalar(1.0));
    state_surface.insert(BoundarySymbol::from("m"), BoundaryValue::scalar(1.5));

    state_surface.insert(
        BoundarySymbol::from("frost.options.wintRed"),
        BoundaryValue::scalar(1.0),
    );
    state_surface.insert(
        BoundarySymbol::from("frost.options.fineTop"),
        BoundaryValue::scalar(10.0),
    );
    state_surface.insert(
        BoundarySymbol::from("frost.options.fineBot"),
        BoundaryValue::scalar(10.0),
    );
    state_surface.insert(
        BoundarySymbol::from("frost.options.ksnowf"),
        BoundaryValue::scalar(1.0),
    );
    state_surface.insert(
        BoundarySymbol::from("frost.options.kresf"),
        BoundaryValue::scalar(1.0),
    );
    state_surface.insert(
        BoundarySymbol::from("frost.options.ksoilf"),
        BoundaryValue::scalar(1.0),
    );
    state_surface.insert(
        BoundarySymbol::from("frost.options.kfactor1"),
        BoundaryValue::scalar(0.2),
    );
    state_surface.insert(
        BoundarySymbol::from("frost.options.kfactor2"),
        BoundaryValue::scalar(0.4),
    );
    state_surface.insert(
        BoundarySymbol::from("frost.options.kfactor3"),
        BoundaryValue::scalar(0.5),
    );
    state_surface.insert(
        BoundarySymbol::from("frost.options.frost_file_present"),
        BoundaryValue::scalar(if active_frost { 1.0 } else { 0.0 }),
    );
    state_surface.insert(
        BoundarySymbol::from("snow.runtime_depth_m"),
        BoundaryValue::scalar(0.0),
    );
    state_surface.insert(
        BoundarySymbol::from("frost.runtime_residue_depth_m"),
        BoundaryValue::scalar(0.0),
    );
    state_surface.insert(BoundarySymbol::from("tmax"), BoundaryValue::scalar(-2.0));
    state_surface.insert(BoundarySymbol::from("tmin"), BoundaryValue::scalar(-10.0));

    HillslopeWritebackSurface {
        state_surface,
        flux_surface: std::collections::BTreeMap::new(),
    }
}

#[test]
fn clim06_contract_conformance_couples_frost_controls_into_wb14_infiltration_capacity() {
    let graph = parse_topology_fixture_str(VALID_TOPOLOGY).expect("fixture should parse");
    let topology_report =
        validate_pre_execution_topology(&graph).expect("topology report should build");
    let scheduler = HillslopePhaseScheduler::canonical();
    let mut kernel = Wb11HydrologyKernel;

    let active_report = scheduler
        .execute_with_kernel(&topology_report, &mut kernel, seeded_clim06_surface(true))
        .expect("clim06 active-coupling execution should return typed report");
    let inactive_report = scheduler
        .execute_with_kernel(&topology_report, &mut kernel, seeded_clim06_surface(false))
        .expect("clim06 inactive-coupling execution should return typed report");

    assert!(active_report.scheduler_report.is_success());
    assert!(inactive_report.scheduler_report.is_success());

    let dfrost = active_report
        .writeback_surface
        .state_surface
        .get(&BoundarySymbol::from("frost.runtime_dfrost"))
        .expect("frost.runtime_dfrost should be present")
        .as_f64();
    assert!((dfrost - EXPECTED_DFROST).abs() <= CLIM06_TEST_TOLERANCE);

    let dthaw = active_report
        .writeback_surface
        .state_surface
        .get(&BoundarySymbol::from("frost.runtime_dthaw"))
        .expect("frost.runtime_dthaw should be present")
        .as_f64();
    assert!((dthaw - EXPECTED_DTHAW).abs() <= CLIM06_TEST_TOLERANCE);

    let nft = active_report
        .writeback_surface
        .state_surface
        .get(&BoundarySymbol::from("frost.runtime_nft"))
        .expect("frost.runtime_nft should be present")
        .as_f64();
    assert!((nft - EXPECTED_NFT).abs() <= CLIM06_TEST_TOLERANCE);

    let ws_frz = active_report
        .writeback_surface
        .state_surface
        .get(&BoundarySymbol::from("frost.runtime_ws_frz"))
        .expect("frost.runtime_ws_frz should be present")
        .as_f64();
    assert!((ws_frz - EXPECTED_WS_FRZ).abs() <= CLIM06_TEST_TOLERANCE);

    let infcap_frz = active_report
        .writeback_surface
        .state_surface
        .get(&BoundarySymbol::from("frost.runtime_infcap_frz"))
        .expect("frost.runtime_infcap_frz should be present")
        .as_f64();
    assert!((infcap_frz - EXPECTED_INF_CAP_FRZ).abs() <= CLIM06_TEST_TOLERANCE);

    let active_infiltration = active_report
        .writeback_surface
        .state_surface
        .get(&BoundarySymbol::from("wb12_infiltration"))
        .expect("active wb12_infiltration should be present")
        .as_f64();
    let inactive_infiltration = inactive_report
        .writeback_surface
        .state_surface
        .get(&BoundarySymbol::from("wb12_infiltration"))
        .expect("inactive wb12_infiltration should be present")
        .as_f64();
    assert!(active_infiltration < inactive_infiltration);

    let active_q = active_report
        .writeback_surface
        .flux_surface
        .get(&BoundarySymbol::from("Q"))
        .expect("active Q should be present")
        .as_f64();
    let inactive_q = inactive_report
        .writeback_surface
        .flux_surface
        .get(&BoundarySymbol::from("Q"))
        .expect("inactive Q should be present")
        .as_f64();
    assert!(active_q > inactive_q);
}

#[test]
fn clim06_contract_conformance_rejects_missing_active_frost_symbol() {
    let graph = parse_topology_fixture_str(VALID_TOPOLOGY).expect("fixture should parse");
    let topology_report =
        validate_pre_execution_topology(&graph).expect("topology report should build");
    let scheduler = HillslopePhaseScheduler::canonical();
    let mut kernel = Wb11HydrologyKernel;

    let mut surface = seeded_clim06_surface(true);
    surface
        .state_surface
        .remove(&BoundarySymbol::from("frost.options.kfactor3"));

    let report = scheduler
        .execute_with_kernel(&topology_report, &mut kernel, surface)
        .expect("clim06 missing-symbol failure should return typed report");

    assert_eq!(
        report.scheduler_report.halted_phase,
        Some(HillslopePhase::RunoffReconciliation)
    );

    let runoff_phase = report
        .phase_reports
        .iter()
        .find(|phase| phase.phase == HillslopePhase::RunoffReconciliation)
        .expect("runoff phase report should exist");
    assert_eq!(
        runoff_phase.decision_status.message_id(),
        "HKERNEL-WB14-RUNOFF-E-001"
    );
    assert_eq!(
        runoff_phase.decision_status.boundary_class(),
        BoundaryClass::MissingRequiredInput
    );
}

#[test]
fn clim06_contract_conformance_rejects_non_finite_active_frost_symbol() {
    let graph = parse_topology_fixture_str(VALID_TOPOLOGY).expect("fixture should parse");
    let topology_report =
        validate_pre_execution_topology(&graph).expect("topology report should build");
    let scheduler = HillslopePhaseScheduler::canonical();
    let mut kernel = Wb11HydrologyKernel;

    let mut surface = seeded_clim06_surface(true);
    surface.state_surface.insert(
        BoundarySymbol::from("frost.options.kfactor2"),
        BoundaryValue::scalar(f64::NAN),
    );

    let report = scheduler
        .execute_with_kernel(&topology_report, &mut kernel, surface)
        .expect("clim06 non-finite failure should return typed report");

    assert_eq!(
        report.scheduler_report.halted_phase,
        Some(HillslopePhase::RunoffReconciliation)
    );

    let runoff_phase = report
        .phase_reports
        .iter()
        .find(|phase| phase.phase == HillslopePhase::RunoffReconciliation)
        .expect("runoff phase report should exist");
    assert_eq!(
        runoff_phase.decision_status.message_id(),
        "HKERNEL-WB14-RUNOFF-E-002"
    );
    assert_eq!(
        runoff_phase.decision_status.boundary_class(),
        BoundaryClass::NonFinite
    );
}

#[test]
fn clim06_contract_conformance_rejects_out_of_domain_active_frost_symbol() {
    let graph = parse_topology_fixture_str(VALID_TOPOLOGY).expect("fixture should parse");
    let topology_report =
        validate_pre_execution_topology(&graph).expect("topology report should build");
    let scheduler = HillslopePhaseScheduler::canonical();
    let mut kernel = Wb11HydrologyKernel;

    let mut surface = seeded_clim06_surface(true);
    surface.state_surface.insert(
        BoundarySymbol::from("frost.options.kfactor1"),
        BoundaryValue::scalar(1.5),
    );

    let report = scheduler
        .execute_with_kernel(&topology_report, &mut kernel, surface)
        .expect("clim06 domain failure should return typed report");

    assert_eq!(
        report.scheduler_report.halted_phase,
        Some(HillslopePhase::RunoffReconciliation)
    );

    let runoff_phase = report
        .phase_reports
        .iter()
        .find(|phase| phase.phase == HillslopePhase::RunoffReconciliation)
        .expect("runoff phase report should exist");
    assert_eq!(
        runoff_phase.decision_status.message_id(),
        "HKERNEL-WB14-RUNOFF-E-003"
    );
    assert_eq!(
        runoff_phase.decision_status.boundary_class(),
        BoundaryClass::DomainViolation
    );
}

#[test]
fn simimpl33_contract_conformance_rejects_missing_frost_runtime_residue_depth_symbol() {
    let graph = parse_topology_fixture_str(VALID_TOPOLOGY).expect("fixture should parse");
    let topology_report =
        validate_pre_execution_topology(&graph).expect("topology report should build");
    let scheduler = HillslopePhaseScheduler::canonical();
    let mut kernel = Wb11HydrologyKernel;

    let mut surface = seeded_clim06_surface(true);
    surface
        .state_surface
        .remove(&BoundarySymbol::from("frost.runtime_residue_depth_m"));

    let report = scheduler
        .execute_with_kernel(&topology_report, &mut kernel, surface)
        .expect("clim06 missing frost-runtime residue depth should return typed report");

    assert_eq!(
        report.scheduler_report.halted_phase,
        Some(HillslopePhase::RunoffReconciliation)
    );

    let runoff_phase = report
        .phase_reports
        .iter()
        .find(|phase| phase.phase == HillslopePhase::RunoffReconciliation)
        .expect("runoff phase report should exist");
    assert_eq!(
        runoff_phase.decision_status.message_id(),
        "HKERNEL-WB14-RUNOFF-E-001"
    );
    assert_eq!(
        runoff_phase.decision_status.boundary_class(),
        BoundaryClass::MissingRequiredInput
    );
}

#[test]
fn simimpl33_contract_conformance_emits_runtime_topology_and_hourly_frost_seam_symbols() {
    let report = execute_clim06_surface(seeded_clim06_surface(true));
    assert!(report.scheduler_report.is_success());

    let total_fine_layers = require_state_scalar(&report, "frost.runtime_total_fine_layer_count");
    assert!(total_fine_layers >= 2.0);
    assert!(require_state_scalar(&report, "frost.runtime_nfine_0001") >= 1.0);
    assert!(require_state_scalar(&report, "frost.runtime_nfine_0002") >= 1.0);
    assert!(require_state_scalar(&report, "frost.runtime_fine_thickness_m_0001") > 0.0);
    assert!(require_state_scalar(&report, "frost.runtime_fine_thickness_m_0002") > 0.0);
    assert!(require_state_scalar(&report, "frost.runtime_kftill_w_m_k") > 0.0);
    assert!(require_state_scalar(&report, "frost.runtime_kfutil_w_m_k") > 0.0);
    assert!(require_state_scalar(&report, "frost.runtime_kres_w_m_k") > 0.0);
    let _ = require_state_scalar(&report, "frost.hourly.qsrf_w_m2_0001");
    let _ = require_state_scalar(&report, "frost.hourly.quf_w_m2_0001");
    let _ = require_state_scalar(&report, "frost.hourly.ksrf_w_m_k_0001");
    let _ = require_state_scalar(&report, "frost.hourly.snow_depth_m_0001");
    let _ = require_state_scalar(&report, "frost.hourly.residue_depth_m_0001");
    let _ = require_state_scalar(&report, "frost.hourly.tilled_frozen_depth_m_0001");
    let _ = require_state_scalar(&report, "frost.hourly.untilled_frozen_depth_m_0001");
}

fn execute_clim06_surface(
    surface: HillslopeWritebackSurface,
) -> openwepp_hillslope_orchestrator::HillslopeKernelExecutionReport {
    let graph = parse_topology_fixture_str(VALID_TOPOLOGY).expect("fixture should parse");
    let topology_report =
        validate_pre_execution_topology(&graph).expect("topology report should build");
    let scheduler = HillslopePhaseScheduler::canonical();
    let mut kernel = Wb11HydrologyKernel;
    scheduler
        .execute_with_kernel(&topology_report, &mut kernel, surface)
        .expect("CLIM06 execution should return typed report")
}

fn require_state_scalar(
    report: &openwepp_hillslope_orchestrator::HillslopeKernelExecutionReport,
    symbol: &str,
) -> f64 {
    report
        .writeback_surface
        .state_surface
        .get(&BoundarySymbol::from(symbol))
        .unwrap_or_else(|| panic!("missing expected state symbol {symbol}"))
        .as_f64()
}

#[test]
fn simimpl32_contract_dispatch_trigger_vector_requires_active_frost_hourly_emission() {
    let active = execute_clim06_surface(seeded_clim06_surface(true));
    let inactive = execute_clim06_surface(seeded_clim06_surface(false));
    assert!(active.scheduler_report.is_success());
    assert!(inactive.scheduler_report.is_success());

    let _active_qsrf = require_state_scalar(&active, "frost.hourly.qsrf_w_m2_0001");
    let _active_quf = require_state_scalar(&active, "frost.hourly.quf_w_m2_0001");

    assert!(
        !inactive
            .writeback_surface
            .state_surface
            .contains_key(&BoundarySymbol::from("frost.hourly.qsrf_w_m2_0001")),
        "inactive frost coupling should not emit frost.hourly.* branch payloads"
    );
}

#[test]
fn simimpl32_contract_handoff_direction_vector_requires_frozen_water_exchange_effect() {
    let active = execute_clim06_surface(seeded_clim06_surface(true));
    let inactive = execute_clim06_surface(seeded_clim06_surface(false));
    assert!(active.scheduler_report.is_success());
    assert!(inactive.scheduler_report.is_success());

    let active_ws_frz = require_state_scalar(&active, "frost.runtime_ws_frz");
    let inactive_ws_frz = inactive
        .writeback_surface
        .state_surface
        .get(&BoundarySymbol::from("frost.runtime_ws_frz"))
        .map_or(0.0, |value| value.as_f64());
    let active_soil_water = require_state_scalar(&active, "wb11_soil_water");
    let inactive_soil_water = require_state_scalar(&inactive, "wb11_soil_water");

    assert!(
        active_ws_frz > inactive_ws_frz + CLIM06_TEST_TOLERANCE,
        "active frost handoff should increase frozen-water ledger relative to inactive path"
    );
    assert!(
        active_soil_water + CLIM06_TEST_TOLERANCE < inactive_soil_water,
        "frwatc-style ingress/egress handoff should reduce liquid wb11 soil-water under active frost"
    );
}

#[test]
fn simimpl32_contract_freeze_lineage_vector_requires_temperature_sensitive_frost_progression() {
    let mut mild = seeded_clim06_surface(true);
    mild.state_surface
        .insert(BoundarySymbol::from("tmax"), BoundaryValue::scalar(0.1));
    mild.state_surface
        .insert(BoundarySymbol::from("tmin"), BoundaryValue::scalar(-0.1));

    let mut severe = seeded_clim06_surface(true);
    severe
        .state_surface
        .insert(BoundarySymbol::from("tmax"), BoundaryValue::scalar(-2.0));
    severe
        .state_surface
        .insert(BoundarySymbol::from("tmin"), BoundaryValue::scalar(-12.0));

    let mild_report = execute_clim06_surface(mild);
    let severe_report = execute_clim06_surface(severe);
    assert!(mild_report.scheduler_report.is_success());
    assert!(severe_report.scheduler_report.is_success());

    let mild_dfrost = require_state_scalar(&mild_report, "frost.runtime_dfrost");
    let severe_dfrost = require_state_scalar(&severe_report, "frost.runtime_dfrost");
    let mild_ws = require_state_scalar(&mild_report, "frost.runtime_ws_frz");
    let severe_ws = require_state_scalar(&severe_report, "frost.runtime_ws_frz");

    assert!(
        severe_dfrost > mild_dfrost + CLIM06_TEST_TOLERANCE,
        "freeze-lineage closure requires stronger cold forcing to deepen frost front"
    );
    assert!(
        severe_ws > mild_ws + CLIM06_TEST_TOLERANCE,
        "freeze-lineage closure requires stronger cold forcing to increase frozen-water accumulation"
    );
}

#[test]
fn simimpl32_contract_conductivity_lineage_vector_requires_land_use_dependent_kfactor_selection() {
    let mut forest_surface = seeded_clim06_surface(true);
    forest_surface.state_surface.insert(
        BoundarySymbol::from("landuse.class_proxy"),
        BoundaryValue::scalar(3.0),
    );
    forest_surface.state_surface.insert(
        BoundarySymbol::from("frost.options.kfactor1"),
        BoundaryValue::scalar(0.2),
    );
    forest_surface.state_surface.insert(
        BoundarySymbol::from("frost.options.kfactor2"),
        BoundaryValue::scalar(0.4),
    );
    forest_surface.state_surface.insert(
        BoundarySymbol::from("frost.options.kfactor3"),
        BoundaryValue::scalar(0.9),
    );

    let mut annual_surface = seeded_clim06_surface(true);
    annual_surface.state_surface.insert(
        BoundarySymbol::from("landuse.class_proxy"),
        BoundaryValue::scalar(1.0),
    );
    annual_surface.state_surface.insert(
        BoundarySymbol::from("frost.options.kfactor1"),
        BoundaryValue::scalar(0.2),
    );
    annual_surface.state_surface.insert(
        BoundarySymbol::from("frost.options.kfactor2"),
        BoundaryValue::scalar(0.4),
    );
    annual_surface.state_surface.insert(
        BoundarySymbol::from("frost.options.kfactor3"),
        BoundaryValue::scalar(0.9),
    );

    let forest_report = execute_clim06_surface(forest_surface);
    let annual_report = execute_clim06_surface(annual_surface);
    assert!(forest_report.scheduler_report.is_success());
    assert!(annual_report.scheduler_report.is_success());

    let forest_infcap = require_state_scalar(&forest_report, "frost.runtime_infcap_frz");
    let annual_infcap = require_state_scalar(&annual_report, "frost.runtime_infcap_frz");

    assert!(
        (forest_infcap - annual_infcap).abs() > CLIM06_TEST_TOLERANCE,
        "getFreezeCond lineage closure requires land-use-dependent conductivity divergence when kfactor set differs by class"
    );
}

#[test]
fn simimpl32_contract_cross_contract_seam_vector_requires_frost_hourly_payload_completeness() {
    let report = execute_clim06_surface(seeded_clim06_surface(true));
    assert!(report.scheduler_report.is_success());

    let _dfrost = require_state_scalar(&report, "frost.runtime_dfrost");
    let _dthaw = require_state_scalar(&report, "frost.runtime_dthaw");
    let _nft = require_state_scalar(&report, "frost.runtime_nft");
    let _ws_frz = require_state_scalar(&report, "frost.runtime_ws_frz");
    let _infcap = require_state_scalar(&report, "frost.runtime_infcap_frz");

    for symbol in [
        "frost.hourly.qsrf_w_m2_0001",
        "frost.hourly.quf_w_m2_0001",
        "frost.hourly.ksrf_w_m_k_0001",
        "frost.hourly.snow_depth_m_0001",
        "frost.hourly.residue_depth_m_0001",
        "frost.hourly.tilled_frozen_depth_m_0001",
        "frost.hourly.untilled_frozen_depth_m_0001",
    ] {
        let _ = require_state_scalar(&report, symbol);
    }
}
