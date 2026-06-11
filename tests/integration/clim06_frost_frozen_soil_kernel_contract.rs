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
const EXPECTED_DTHAW: f64 = 0.0;
const EXPECTED_NFT: f64 = 1.0;

#[allow(clippy::too_many_lines)]
fn seeded_clim06_surface(active_frost: bool) -> HillslopeWritebackSurface {
    let mut state_surface = std::collections::BTreeMap::new();

    state_surface.insert(BoundarySymbol::from("nsl"), BoundaryValue::scalar(2.0));
    state_surface.insert(BoundarySymbol::from("solthk"), BoundaryValue::scalar(0.3));
    state_surface.insert(
        BoundarySymbol::from("solwpv"),
        BoundaryValue::scalar(2006.0),
    );
    state_surface.insert(BoundarySymbol::from("dg"), BoundaryValue::scalar(2.0));
    state_surface.insert(BoundarySymbol::from("thetdr"), BoundaryValue::scalar(0.0));
    state_surface.insert(BoundarySymbol::from("thetfc"), BoundaryValue::scalar(1.0));
    state_surface.insert(BoundarySymbol::from("ssc"), BoundaryValue::scalar(0.5));
    state_surface.insert(BoundarySymbol::from("cancov"), BoundaryValue::scalar(0.0));
    state_surface.insert(BoundarySymbol::from("lai"), BoundaryValue::scalar(0.0));
    state_surface.insert(BoundarySymbol::from("vdmt"), BoundaryValue::scalar(0.0));
    state_surface.insert(BoundarySymbol::from("rtd"), BoundaryValue::scalar(0.0));
    state_surface.insert(BoundarySymbol::from("pltol"), BoundaryValue::scalar(0.25));

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
        BoundarySymbol::from("thetfc_0001"),
        BoundaryValue::scalar(50.0),
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
        BoundaryValue::scalar(5.0),
    );
    state_surface.insert(
        BoundarySymbol::from("wb18_perc_fc_0002"),
        BoundaryValue::scalar(4.0),
    );
    state_surface.insert(
        BoundarySymbol::from("thetfc_0002"),
        BoundaryValue::scalar(40.0),
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
    state_surface.insert(BoundarySymbol::from("por_0001"), BoundaryValue::scalar(0.8));
    state_surface.insert(BoundarySymbol::from("por_0002"), BoundaryValue::scalar(0.8));
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
        BoundaryValue::scalar(if active_frost { 1.0 } else { 0.0 }),
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
        BoundarySymbol::from("snow.runtime_swe"),
        BoundaryValue::scalar(0.0),
    );
    state_surface.insert(
        BoundarySymbol::from("snow.runtime_density_kg_m3"),
        BoundaryValue::scalar(0.0),
    );
    state_surface.insert(
        BoundarySymbol::from("snow.runtime_settle_day_count"),
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
    assert!(
        dfrost > CLIM06_TEST_TOLERANCE,
        "active cold frost coupling should publish positive Dfrost"
    );
    assert!(
        dfrost <= 0.3 + CLIM06_TEST_TOLERANCE,
        "Dfrost should remain bounded by the seeded physical profile depth"
    );

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
    assert!((ws_frz - frozen_layer_frzw_sum(&active_report)).abs() <= CLIM06_TEST_TOLERANCE);

    let infcap_frz = active_report
        .writeback_surface
        .state_surface
        .get(&BoundarySymbol::from("frost.runtime_infcap_frz"))
        .expect("frost.runtime_infcap_frz should be present")
        .as_f64();
    let ssc = active_report
        .writeback_surface
        .state_surface
        .get(&BoundarySymbol::from("ssc"))
        .expect("ssc should be present")
        .as_f64();
    assert!(
        infcap_frz + CLIM06_TEST_TOLERANCE < ssc,
        "active frost should reduce infiltration capacity"
    );

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
    let _ = require_state_scalar(&report, "frost.runtime_frwatc_soil_water_before_m");
    let _ = require_state_scalar(&report, "frost.runtime_frwatc_soil_water_after_m");
    let _ = require_state_scalar(&report, "frost.runtime_frwatc_frozen_water_before_m");
    let _ = require_state_scalar(&report, "frost.runtime_frwatc_frozen_water_after_m");
    let _ = require_state_scalar(&report, "frost.runtime_frwatc_freeze_debit_m");
    let _ = require_state_scalar(&report, "frost.runtime_frwatc_thaw_credit_m");
    let _ = require_state_scalar(&report, "frost.runtime_frwatc_net_liquid_delta_m");
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

fn insert_state_scalar(surface: &mut HillslopeWritebackSurface, symbol: &str, value: f64) {
    surface
        .state_surface
        .insert(BoundarySymbol::from(symbol), BoundaryValue::scalar(value));
}

fn frozen_layer_frzw_sum(
    report: &openwepp_hillslope_orchestrator::HillslopeKernelExecutionReport,
) -> f64 {
    require_state_scalar(report, "wb18_perc_frzw_0001")
        + require_state_scalar(report, "wb18_perc_frzw_0002")
}

fn layer_frozen_depth_sum(
    report: &openwepp_hillslope_orchestrator::HillslopeKernelExecutionReport,
) -> f64 {
    require_state_scalar(report, "wb18_perc_frozen_depth_0001")
        + require_state_scalar(report, "wb18_perc_frozen_depth_0002")
}

fn configure_fdhp01_deep_profile(surface: &mut HillslopeWritebackSurface) {
    insert_state_scalar(surface, "dg_0002", 0.20);
    insert_state_scalar(surface, "wb18_perc_fc_0002", 8.0);
    insert_state_scalar(surface, "wb19_drain_enabled", 0.0);
    insert_state_scalar(surface, "wb11_lateral_fraction", 0.0);
    insert_state_scalar(surface, "wb11_drainage_fraction", 0.0);
}

fn seed_prior_layered_frost(surface: &mut HillslopeWritebackSurface, depth_m: f64, frzw_m: f64) {
    insert_state_scalar(surface, "frost.runtime_frdp_m", depth_m);
    insert_state_scalar(surface, "frost.runtime_dfrost", depth_m);
    insert_state_scalar(surface, "frost.runtime_ws_frz", frzw_m);
    insert_state_scalar(surface, "wb18_perc_frozen_depth_0001", depth_m.min(0.10));
    insert_state_scalar(surface, "wb18_perc_frzw_0001", frzw_m.min(0.10));
    insert_state_scalar(
        surface,
        "wb18_perc_frozen_depth_0002",
        (depth_m - 0.10).max(0.0),
    );
    insert_state_scalar(surface, "wb18_perc_frzw_0002", (frzw_m - 0.10).max(0.0));
}

fn assert_close(actual: f64, expected: f64, context: &str) {
    assert!(
        (actual - expected).abs() <= CLIM06_TEST_TOLERANCE,
        "{context}: expected {expected}, got {actual}"
    );
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
fn fdhp01_contract_heat_flow_publishes_separate_surface_and_unfrozen_fluxes() {
    let active = execute_clim06_surface(seeded_clim06_surface(true));
    assert!(active.scheduler_report.is_success());

    let qsrf = require_state_scalar(&active, "frost.hourly.qsrf_w_m2_0001");
    let quf = require_state_scalar(&active, "frost.hourly.quf_w_m2_0001");

    assert!(
        qsrf > CLIM06_TEST_TOLERANCE,
        "cold-hour frost heat-flow must publish surface heat loss through the frozen path"
    );
    assert!(
        quf > CLIM06_TEST_TOLERANCE,
        "FDHP01 heat-flow must publish separate lower unfrozen-soil heat flow"
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
fn fdhp01_d2_contract_frwatc_freeze_exchange_diagnostics_reconcile_liquid_and_frozen_storage() {
    let report = execute_clim06_surface(seeded_clim06_surface(true));
    assert!(report.scheduler_report.is_success());

    let liquid_before = require_state_scalar(&report, "frost.runtime_frwatc_soil_water_before_m");
    let liquid_after = require_state_scalar(&report, "frost.runtime_frwatc_soil_water_after_m");
    let frozen_before = require_state_scalar(&report, "frost.runtime_frwatc_frozen_water_before_m");
    let frozen_after = require_state_scalar(&report, "frost.runtime_frwatc_frozen_water_after_m");
    let freeze_debit = require_state_scalar(&report, "frost.runtime_frwatc_freeze_debit_m");
    let thaw_credit = require_state_scalar(&report, "frost.runtime_frwatc_thaw_credit_m");
    let net_liquid_delta = require_state_scalar(&report, "frost.runtime_frwatc_net_liquid_delta_m");
    let final_liquid = require_state_scalar(&report, "wb11_soil_water");
    let ws_frz = require_state_scalar(&report, "frost.runtime_ws_frz");

    assert!(
        freeze_debit > CLIM06_TEST_TOLERANCE,
        "cold freeze-onset vector must debit liquid water into frozen storage"
    );
    assert_close(
        thaw_credit,
        0.0,
        "freeze-onset vector must not emit a thaw credit",
    );
    assert_close(
        freeze_debit,
        frozen_after - frozen_before,
        "freeze debit must equal frozen-storage growth",
    );
    assert_close(
        net_liquid_delta,
        -freeze_debit,
        "freeze net liquid delta must be the negative freeze debit",
    );
    assert_close(
        liquid_after,
        liquid_before + net_liquid_delta,
        "freeze liquid after must reconcile to liquid before plus net exchange",
    );
    assert_close(
        final_liquid,
        liquid_after,
        "WB11 liquid writeback must equal the diagnosed post-frwatc liquid state",
    );
    assert_close(
        ws_frz,
        frozen_after,
        "runtime ws_frz must equal the diagnosed post-frwatc frozen storage",
    );
}

#[test]
fn fdhp01_layered_store_contract_rejects_scalar_frdp_theta_frozen_water_authority() {
    let mut surface = seeded_clim06_surface(true);
    surface
        .state_surface
        .insert(BoundarySymbol::from("tmax"), BoundaryValue::scalar(8.0));
    surface
        .state_surface
        .insert(BoundarySymbol::from("tmin"), BoundaryValue::scalar(2.0));
    surface.state_surface.insert(
        BoundarySymbol::from("frost.runtime_frdp_m"),
        BoundaryValue::scalar(0.20),
    );
    surface.state_surface.insert(
        BoundarySymbol::from("frost.runtime_dfrost"),
        BoundaryValue::scalar(0.20),
    );
    surface.state_surface.insert(
        BoundarySymbol::from("frost.runtime_ws_frz"),
        BoundaryValue::scalar(0.001),
    );
    surface.state_surface.insert(
        BoundarySymbol::from("wb18_perc_frozen_depth_0001"),
        BoundaryValue::scalar(0.040),
    );
    surface.state_surface.insert(
        BoundarySymbol::from("wb18_perc_frzw_0001"),
        BoundaryValue::scalar(0.020),
    );
    surface.state_surface.insert(
        BoundarySymbol::from("wb18_perc_frozen_depth_0002"),
        BoundaryValue::scalar(0.010),
    );
    surface.state_surface.insert(
        BoundarySymbol::from("wb18_perc_frzw_0002"),
        BoundaryValue::scalar(0.004),
    );

    let report = execute_clim06_surface(surface);
    assert!(
        report.scheduler_report.is_success(),
        "expected deep frost vector to succeed, scheduler={:?}, phases={:?}",
        report.scheduler_report,
        report.phase_reports
    );

    let frozen_before = require_state_scalar(&report, "frost.runtime_frwatc_frozen_water_before_m");
    let frozen_after = require_state_scalar(&report, "frost.runtime_frwatc_frozen_water_after_m");
    let ws_frz = require_state_scalar(&report, "frost.runtime_ws_frz");
    let layer_frzw_sum = frozen_layer_frzw_sum(&report);

    assert_close(
        frozen_before,
        0.024,
        "prior frozen-store authority must be the seeded layer frzw sum",
    );
    assert_close(
        ws_frz,
        layer_frzw_sum,
        "runtime ws_frz must equal the layer frozen-water store sum",
    );
    assert_close(
        frozen_after,
        layer_frzw_sum,
        "frwatc after diagnostic must equal the layer frozen-water store sum",
    );
    assert!(
        (frozen_before - 0.001).abs() > CLIM06_TEST_TOLERANCE,
        "seeded runtime_ws_frz must not override the layer frozen-store authority"
    );
}

#[test]
fn fdhp01_layered_store_contract_freeze_updates_layer_depth_and_frzw_sum() {
    let report = execute_clim06_surface(seeded_clim06_surface(true));
    assert!(report.scheduler_report.is_success());

    let dfrost = require_state_scalar(&report, "frost.runtime_dfrost");
    let ws_frz = require_state_scalar(&report, "frost.runtime_ws_frz");
    let frozen_after = require_state_scalar(&report, "frost.runtime_frwatc_frozen_water_after_m");
    let layer_depth_sum = layer_frozen_depth_sum(&report);
    let layer_frzw_sum = frozen_layer_frzw_sum(&report);

    assert!(
        layer_depth_sum > CLIM06_TEST_TOLERANCE,
        "active freezing must write per-layer frozen depth"
    );
    assert!(
        layer_frzw_sum > CLIM06_TEST_TOLERANCE,
        "active freezing must write per-layer frozen water"
    );
    assert_close(
        layer_depth_sum,
        dfrost,
        "aggregate frost depth must reconcile to the layer frozen-depth state",
    );
    assert_close(
        layer_frzw_sum,
        ws_frz,
        "runtime ws_frz must be derived from the layer frozen-water store",
    );
    assert_close(
        layer_frzw_sum,
        frozen_after,
        "frwatc frozen-water diagnostic must be derived from the layer store",
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
fn fdhp01_contract_heat_flow_depth_can_exceed_retired_proxy_cap() {
    let mut surface = seeded_clim06_surface(true);
    surface
        .state_surface
        .insert(BoundarySymbol::from("tmax"), BoundaryValue::scalar(-8.0));
    surface
        .state_surface
        .insert(BoundarySymbol::from("tmin"), BoundaryValue::scalar(-16.0));
    surface.state_surface.insert(
        BoundarySymbol::from("frost.runtime_frdp_m"),
        BoundaryValue::scalar(0.19),
    );
    configure_fdhp01_deep_profile(&mut surface);

    let report = execute_clim06_surface(surface);
    assert!(
        report.scheduler_report.is_success(),
        "expected deep frost vector to succeed, scheduler={:?}, phases={:?}",
        report.scheduler_report,
        report.phase_reports
    );

    let dfrost = require_state_scalar(&report, "frost.runtime_dfrost");
    let frdp = require_state_scalar(&report, "frost.runtime_frdp_m");
    assert!(
        dfrost > 0.20 + CLIM06_TEST_TOLERANCE,
        "FDHP01 requires heat-flow frost progression beyond the retired 0.20 m proxy cap"
    );
    assert!(
        (frdp - dfrost).abs() <= CLIM06_TEST_TOLERANCE,
        "published runtime frdp must match the active frost-front depth"
    );
}

#[test]
fn fdhp01_contract_warm_heat_flow_thaws_prior_deep_frost() {
    let mut surface = seeded_clim06_surface(true);
    surface
        .state_surface
        .insert(BoundarySymbol::from("tmax"), BoundaryValue::scalar(8.0));
    surface
        .state_surface
        .insert(BoundarySymbol::from("tmin"), BoundaryValue::scalar(2.0));
    configure_fdhp01_deep_profile(&mut surface);
    seed_prior_layered_frost(&mut surface, 0.30, 0.30);
    let mut no_prior_storage_surface = surface.clone();
    seed_prior_layered_frost(&mut no_prior_storage_surface, 0.0, 0.0);

    let report = execute_clim06_surface(surface);
    let no_prior_storage_report = execute_clim06_surface(no_prior_storage_surface);
    assert!(
        report.scheduler_report.is_success(),
        "prior physical frost depth above 0.20 m must not be rejected by the retired proxy cap; scheduler={:?}, phases={:?}",
        report.scheduler_report,
        report.phase_reports
    );
    assert!(no_prior_storage_report.scheduler_report.is_success());

    let dfrost = require_state_scalar(&report, "frost.runtime_dfrost");
    let dthaw = require_state_scalar(&report, "frost.runtime_dthaw");
    let ws_frz = require_state_scalar(&report, "frost.runtime_ws_frz");
    let soil_water = require_state_scalar(&report, "wb11_soil_water");
    let liquid_before = require_state_scalar(&report, "frost.runtime_frwatc_soil_water_before_m");
    let liquid_after = require_state_scalar(&report, "frost.runtime_frwatc_soil_water_after_m");
    let frozen_before = require_state_scalar(&report, "frost.runtime_frwatc_frozen_water_before_m");
    let frozen_after = require_state_scalar(&report, "frost.runtime_frwatc_frozen_water_after_m");
    let freeze_debit = require_state_scalar(&report, "frost.runtime_frwatc_freeze_debit_m");
    let thaw_credit = require_state_scalar(&report, "frost.runtime_frwatc_thaw_credit_m");
    let net_liquid_delta = require_state_scalar(&report, "frost.runtime_frwatc_net_liquid_delta_m");
    assert!(
        dfrost < 0.30 - CLIM06_TEST_TOLERANCE,
        "warm heat flow should thaw prior deep frost"
    );
    assert!(
        dthaw > CLIM06_TEST_TOLERANCE,
        "warm heat flow should publish positive thaw depth"
    );
    assert!(
        ws_frz < 0.30 - CLIM06_TEST_TOLERANCE,
        "warm thaw should reduce frozen-water storage"
    );
    assert_close(
        freeze_debit,
        0.0,
        "warm thaw vector must not emit a freeze debit",
    );
    assert!(
        thaw_credit > CLIM06_TEST_TOLERANCE,
        "warm thaw vector must credit reduced frozen storage back to liquid water"
    );
    assert_close(
        thaw_credit,
        frozen_before - frozen_after,
        "thaw credit must equal frozen-storage reduction",
    );
    assert_close(
        net_liquid_delta,
        thaw_credit,
        "thaw net liquid delta must equal the thaw credit",
    );
    assert_close(
        liquid_after,
        liquid_before + net_liquid_delta,
        "thaw liquid after must reconcile to liquid before plus net exchange",
    );
    assert_close(
        soil_water,
        liquid_after,
        "WB11 liquid writeback must equal the diagnosed post-frwatc liquid state",
    );
    assert_close(
        ws_frz,
        frozen_after,
        "runtime ws_frz must equal the diagnosed post-frwatc frozen storage",
    );
}

#[test]
fn fdhp01_contract_frozen_water_exchange_hard_fails_on_liquid_overdraw() {
    let mut surface = seeded_clim06_surface(true);
    surface.state_surface.insert(
        BoundarySymbol::from("wb11_soil_water"),
        BoundaryValue::scalar(0.001),
    );
    for symbol in [
        "wb11_field_capacity",
        "wb11_et_demand",
        "wb12_rainfall_input",
        "wb12_runon_input",
    ] {
        surface
            .state_surface
            .insert(BoundarySymbol::from(symbol), BoundaryValue::scalar(0.0));
    }
    surface.state_surface.insert(
        BoundarySymbol::from("frost.runtime_frdp_m"),
        BoundaryValue::scalar(0.29),
    );
    surface
        .state_surface
        .insert(BoundarySymbol::from("thetfc"), BoundaryValue::scalar(30.0));
    surface
        .state_surface
        .insert(BoundarySymbol::from("tmax"), BoundaryValue::scalar(-20.0));
    surface
        .state_surface
        .insert(BoundarySymbol::from("tmin"), BoundaryValue::scalar(-40.0));

    let report = execute_clim06_surface(surface);

    assert!(
        !report.scheduler_report.is_success(),
        "FDHP01 must hard-fail instead of silently creating frozen-water storage beyond available liquid soil water"
    );
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
        runoff_phase.decision_status.boundary_class(),
        BoundaryClass::DomainViolation
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

#[test]
fn fq4_contract_default_frost_controls_activate_without_frost_sidecar_presence() {
    let mut surface = seeded_clim06_surface(false);
    surface.state_surface.insert(
        BoundarySymbol::from("frost.options.wintRed"),
        BoundaryValue::scalar(1.0),
    );

    let report = execute_clim06_surface(surface);
    assert!(report.scheduler_report.is_success());

    assert!(require_state_scalar(&report, "frost.runtime_dfrost") > CLIM06_TEST_TOLERANCE);
    assert!(require_state_scalar(&report, "frost.runtime_ws_frz") > CLIM06_TEST_TOLERANCE);
    let infcap = require_state_scalar(&report, "frost.runtime_infcap_frz");
    let ssc = require_state_scalar(&report, "ssc");
    assert!(
        infcap + CLIM06_TEST_TOLERANCE < ssc,
        "defaulted frost controls with wintRed=1 should reduce conductivity even when frost_file_present=0"
    );
}
