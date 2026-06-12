use openwepp_hillslope_orchestrator::{
    HillslopePhase, HillslopePhaseScheduler, HillslopeWritebackSurface, Wb11HydrologyKernel,
};
use openwepp_kernel_contract::{
    BoundarySymbol, BoundaryValue, HillslopeConsumerAdapter, HillslopeKernel,
    HillslopeKernelPhaseClass, HillslopeKernelRequest, KernelRunResponse,
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

fn execute_clim06_runoff_phase(surface: &HillslopeWritebackSurface) -> KernelRunResponse {
    let request = HillslopeKernelRequest::with_phase_context(
        "runoff_reconciliation",
        HillslopeKernelPhaseClass::HydrologyRunoffReconciliation,
        HillslopeConsumerAdapter::Runoff,
        None,
        &surface.state_surface,
        &surface.flux_surface,
    );
    let mut kernel = Wb11HydrologyKernel;
    kernel.run_hillslope_phase(&request)
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

fn require_response_state_update(response: &KernelRunResponse, symbol: &str) -> f64 {
    response
        .writeback
        .state_updates
        .iter()
        .find(|field| field.symbol == BoundarySymbol::from(symbol))
        .unwrap_or_else(|| panic!("missing expected state update {symbol}"))
        .value
        .as_f64()
}

fn insert_state_scalar(surface: &mut HillslopeWritebackSurface, symbol: &str, value: f64) {
    surface
        .state_surface
        .insert(BoundarySymbol::from(symbol), BoundaryValue::scalar(value));
}

fn remove_state_prefixes(surface: &mut HillslopeWritebackSurface, prefixes: &[&str]) {
    surface.state_surface.retain(|symbol, _| {
        !prefixes
            .iter()
            .any(|prefix| symbol.as_str().starts_with(prefix))
    });
}

fn fine_frost_symbol(root: &str, layer_index: usize, fine_index: usize) -> String {
    format!("{root}_{layer_index:04}_{fine_index:04}")
}

#[allow(clippy::too_many_lines)]
fn seed_increment_a_shadow_fine_state(surface: &mut HillslopeWritebackSurface, yst_offset_m: f64) {
    insert_state_scalar(surface, "wb11_et_demand", 0.0);
    insert_state_scalar(surface, "wb11_perc_fraction", 0.5);
    insert_state_scalar(surface, "wb11_field_capacity", 12.0);
    insert_state_scalar(surface, "wb18_perc_fc_0001", 10.0);
    insert_state_scalar(surface, "wb18_perc_fc_0002", 10.0);
    insert_state_scalar(surface, "wb18_perc_ul_0001", 20.0);
    insert_state_scalar(surface, "wb18_perc_ul_0002", 20.0);
    insert_state_scalar(surface, "wb19_drain_enabled", 0.0);
    insert_state_scalar(surface, "wb11_lateral_fraction", 0.0);
    insert_state_scalar(surface, "wb11_drainage_fraction", 0.0);
    insert_state_scalar(surface, "wb12_rainfall_input", 0.0);
    insert_state_scalar(surface, "wb12_runon_input", 0.0);
    insert_state_scalar(surface, "wb12_precip_input", 0.0);
    insert_state_scalar(surface, "frost.runtime_ws_frz", 0.012);
    insert_state_scalar(surface, "frost.runtime_dfrost", 0.030);
    insert_state_scalar(surface, "frost.runtime_frdp_m", 0.030);
    insert_state_scalar(surface, "wb18_perc_frozen_depth_0001", 0.030);
    insert_state_scalar(surface, "wb18_perc_frzw_0001", 0.012);
    insert_state_scalar(surface, "wb18_perc_frozen_depth_0002", 0.0);
    insert_state_scalar(surface, "wb18_perc_frzw_0002", 0.0);
    insert_state_scalar(surface, "frost.runtime_yst_m_0001", 5.0 - yst_offset_m);
    insert_state_scalar(surface, "frost.runtime_yst_m_0002", 5.0);
    insert_state_scalar(surface, "frost.runtime_nwfrzz_m_0001", 0.0);
    insert_state_scalar(surface, "frost.runtime_nwfrzz_m_0002", 0.0);

    for fine_index in 1..=10 {
        let layer_1_frozen = fine_index <= 3;
        insert_state_scalar(
            surface,
            &fine_frost_symbol("frost.runtime_fgfrst", 1, fine_index),
            if layer_1_frozen { 1.0 } else { 0.0 },
        );
        insert_state_scalar(
            surface,
            &fine_frost_symbol("frost.runtime_slfsd_m", 1, fine_index),
            if layer_1_frozen { 0.010 } else { 0.0 },
        );
        insert_state_scalar(
            surface,
            &fine_frost_symbol("frost.runtime_slsic_m", 1, fine_index),
            if layer_1_frozen { 0.004 } else { 0.0 },
        );
        insert_state_scalar(
            surface,
            &fine_frost_symbol("frost.runtime_slsw_theta", 1, fine_index),
            if layer_1_frozen { 0.0 } else { 0.2 },
        );
        insert_state_scalar(
            surface,
            &fine_frost_symbol("frost.runtime_sltime_s", 1, fine_index),
            0.0,
        );

        insert_state_scalar(
            surface,
            &fine_frost_symbol("frost.runtime_fgfrst", 2, fine_index),
            0.0,
        );
        insert_state_scalar(
            surface,
            &fine_frost_symbol("frost.runtime_slfsd_m", 2, fine_index),
            0.0,
        );
        insert_state_scalar(
            surface,
            &fine_frost_symbol("frost.runtime_slsic_m", 2, fine_index),
            0.0,
        );
        insert_state_scalar(
            surface,
            &fine_frost_symbol("frost.runtime_slsw_theta", 2, fine_index),
            0.15,
        );
        insert_state_scalar(
            surface,
            &fine_frost_symbol("frost.runtime_sltime_s", 2, fine_index),
            0.0,
        );
    }
}

fn frozen_layer_frzw_sum(
    report: &openwepp_hillslope_orchestrator::HillslopeKernelExecutionReport,
) -> f64 {
    require_state_scalar(report, "wb18_perc_frzw_0001")
        + require_state_scalar(report, "wb18_perc_frzw_0002")
}

fn response_fine_layer_sum(
    response: &KernelRunResponse,
    root: &str,
    layer_index: usize,
    fine_count: usize,
) -> f64 {
    (1..=fine_count)
        .map(|fine_index| {
            require_response_state_update(
                response,
                &fine_frost_symbol(root, layer_index, fine_index),
            )
        })
        .sum()
}

fn response_fine_flag_count(response: &KernelRunResponse, expected_flag: f64) -> usize {
    (1..=2)
        .flat_map(|layer_index| {
            (1..=10).map(move |fine_index| {
                require_response_state_update(
                    response,
                    &fine_frost_symbol("frost.runtime_fgfrst", layer_index, fine_index),
                )
            })
        })
        .filter(|flag| (*flag - expected_flag).abs() <= CLIM06_TEST_TOLERANCE)
        .count()
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

fn configure_fdhp01_frost_only_no_flux(surface: &mut HillslopeWritebackSurface) {
    insert_state_scalar(surface, "wb11_et_demand", 0.0);
    insert_state_scalar(surface, "wb11_perc_fraction", 0.0);
    insert_state_scalar(surface, "wb19_drain_enabled", 0.0);
    insert_state_scalar(surface, "wb11_lateral_fraction", 0.0);
    insert_state_scalar(surface, "wb11_drainage_fraction", 0.0);
    insert_state_scalar(surface, "wb12_rainfall_input", 0.0);
    insert_state_scalar(surface, "wb12_runon_input", 0.0);
    insert_state_scalar(surface, "wb12_precip_input", 0.0);
    insert_state_scalar(surface, "wb12_runoff_closure_tolerance", 1000.0);
    insert_state_scalar(surface, "wb12_storage_closure_tolerance", 1000.0);
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

fn seed_db_thin_front_frost(surface: &mut HillslopeWritebackSurface) {
    configure_fdhp01_frost_only_no_flux(surface);
    let initial_depth_m = 0.0004;
    let initial_ice_m = 0.00008;
    let liquid_theta = 0.2;
    let top_liquid_m = liquid_theta * (0.100 - initial_depth_m);
    let bottom_liquid_m = liquid_theta * 0.100;
    insert_state_scalar(surface, "wb11_soil_water", top_liquid_m + bottom_liquid_m);
    insert_state_scalar(surface, "wb18_perc_theta_0001", top_liquid_m);
    insert_state_scalar(surface, "wb18_perc_theta_0002", bottom_liquid_m);
    insert_state_scalar(surface, "wb18_perc_ul_0001", 0.040);
    insert_state_scalar(surface, "wb18_perc_ul_0002", 0.040);
    insert_state_scalar(surface, "frost.runtime_ws_frz", initial_ice_m);
    insert_state_scalar(surface, "frost.runtime_dfrost", initial_depth_m);
    insert_state_scalar(surface, "frost.runtime_frdp_m", initial_depth_m);
    insert_state_scalar(surface, "wb18_perc_frozen_depth_0001", initial_depth_m);
    insert_state_scalar(surface, "wb18_perc_frzw_0001", initial_ice_m);
    insert_state_scalar(surface, "wb18_perc_frozen_depth_0002", 0.0);
    insert_state_scalar(surface, "wb18_perc_frzw_0002", 0.0);
    insert_state_scalar(surface, "frost.runtime_yst_m_0001", top_liquid_m);
    insert_state_scalar(surface, "frost.runtime_yst_m_0002", bottom_liquid_m);
    insert_state_scalar(surface, "frost.runtime_nwfrzz_m_0001", 0.0);
    insert_state_scalar(surface, "frost.runtime_nwfrzz_m_0002", 0.0);

    for fine_index in 1..=10 {
        insert_state_scalar(
            surface,
            &fine_frost_symbol("frost.runtime_fgfrst", 1, fine_index),
            if fine_index == 1 { 2.0 } else { 0.0 },
        );
        insert_state_scalar(
            surface,
            &fine_frost_symbol("frost.runtime_slfsd_m", 1, fine_index),
            if fine_index == 1 {
                initial_depth_m
            } else {
                0.0
            },
        );
        insert_state_scalar(
            surface,
            &fine_frost_symbol("frost.runtime_slsic_m", 1, fine_index),
            if fine_index == 1 { initial_ice_m } else { 0.0 },
        );
        insert_state_scalar(
            surface,
            &fine_frost_symbol("frost.runtime_slsw_theta", 1, fine_index),
            liquid_theta,
        );
        insert_state_scalar(
            surface,
            &fine_frost_symbol("frost.runtime_sltime_s", 1, fine_index),
            0.0,
        );

        insert_state_scalar(
            surface,
            &fine_frost_symbol("frost.runtime_fgfrst", 2, fine_index),
            0.0,
        );
        insert_state_scalar(
            surface,
            &fine_frost_symbol("frost.runtime_slfsd_m", 2, fine_index),
            0.0,
        );
        insert_state_scalar(
            surface,
            &fine_frost_symbol("frost.runtime_slsic_m", 2, fine_index),
            0.0,
        );
        insert_state_scalar(
            surface,
            &fine_frost_symbol("frost.runtime_slsw_theta", 2, fine_index),
            liquid_theta,
        );
        insert_state_scalar(
            surface,
            &fine_frost_symbol("frost.runtime_sltime_s", 2, fine_index),
            0.0,
        );
    }
}

fn seed_c2_full_top_layer_frost(surface: &mut HillslopeWritebackSurface) {
    configure_fdhp01_frost_only_no_flux(surface);
    insert_state_scalar(surface, "wb11_soil_water", 0.020);
    insert_state_scalar(surface, "wb18_perc_theta_0001", 0.0);
    insert_state_scalar(surface, "wb18_perc_theta_0002", 0.020);
    insert_state_scalar(surface, "wb18_perc_ul_0001", 0.020);
    insert_state_scalar(surface, "wb18_perc_ul_0002", 0.020);
    insert_state_scalar(surface, "frost.runtime_frdp_m", 0.100);
    insert_state_scalar(surface, "frost.runtime_dfrost", 0.100);
    insert_state_scalar(surface, "frost.runtime_ws_frz", 0.020);
    insert_state_scalar(surface, "wb18_perc_frozen_depth_0001", 0.100);
    insert_state_scalar(surface, "wb18_perc_frzw_0001", 0.020);
    insert_state_scalar(surface, "wb18_perc_frozen_depth_0002", 0.0);
    insert_state_scalar(surface, "wb18_perc_frzw_0002", 0.0);
    insert_state_scalar(surface, "frost.runtime_yst_m_0001", 0.0);
    insert_state_scalar(surface, "frost.runtime_yst_m_0002", 0.020);
    insert_state_scalar(surface, "frost.runtime_nwfrzz_m_0001", 0.0);
    insert_state_scalar(surface, "frost.runtime_nwfrzz_m_0002", 0.0);

    for fine_index in 1..=10 {
        insert_state_scalar(
            surface,
            &fine_frost_symbol("frost.runtime_fgfrst", 1, fine_index),
            1.0,
        );
        insert_state_scalar(
            surface,
            &fine_frost_symbol("frost.runtime_slfsd_m", 1, fine_index),
            0.010,
        );
        insert_state_scalar(
            surface,
            &fine_frost_symbol("frost.runtime_slsic_m", 1, fine_index),
            0.002,
        );
        insert_state_scalar(
            surface,
            &fine_frost_symbol("frost.runtime_slsw_theta", 1, fine_index),
            0.0,
        );
        insert_state_scalar(
            surface,
            &fine_frost_symbol("frost.runtime_sltime_s", 1, fine_index),
            0.0,
        );

        insert_state_scalar(
            surface,
            &fine_frost_symbol("frost.runtime_fgfrst", 2, fine_index),
            0.0,
        );
        insert_state_scalar(
            surface,
            &fine_frost_symbol("frost.runtime_slfsd_m", 2, fine_index),
            0.0,
        );
        insert_state_scalar(
            surface,
            &fine_frost_symbol("frost.runtime_slsic_m", 2, fine_index),
            0.0,
        );
        insert_state_scalar(
            surface,
            &fine_frost_symbol("frost.runtime_slsw_theta", 2, fine_index),
            0.2,
        );
        insert_state_scalar(
            surface,
            &fine_frost_symbol("frost.runtime_sltime_s", 2, fine_index),
            0.0,
        );
    }
}

fn apply_response_state_updates(
    surface: &mut HillslopeWritebackSurface,
    response: &KernelRunResponse,
) {
    assert!(
        response.status.ok_flag(),
        "cannot apply failed response: {:?}",
        response.status
    );
    for field in &response.writeback.state_updates {
        surface
            .state_surface
            .insert(field.symbol.clone(), field.value);
    }
    for field in &response.writeback.flux_updates {
        surface
            .flux_surface
            .insert(field.symbol.clone(), field.value);
    }
}

fn assert_close(actual: f64, expected: f64, context: &str) {
    assert!(
        (actual - expected).abs() <= CLIM06_TEST_TOLERANCE,
        "{context}: expected {expected}, got {actual}"
    );
}

#[test]
fn fdhp01_fine_sublayer_frwatc_round_trip_conserves_mass() {
    let mut surface = seeded_clim06_surface(true);
    seed_increment_a_shadow_fine_state(&mut surface, 0.0);
    insert_state_scalar(&mut surface, "tmax", -0.252);
    insert_state_scalar(&mut surface, "tmin", -0.252);

    let response = execute_clim06_runoff_phase(&surface);
    assert!(
        response.status.ok_flag(),
        "Increment A shadow fine-state vector should execute successfully; status={:?}",
        response.status
    );

    assert_close(
        require_response_state_update(&response, "frost.runtime_shadow_frwatc_residual_m"),
        0.0,
        "shadow frwatc round-trip residual must stay at numerical zero",
    );
    assert_close(
        require_response_state_update(&response, "frost.runtime_shadow_st_m_0001"),
        0.014,
        "shadow layer-1 active storage must round-trip",
    );
    assert_close(
        require_response_state_update(&response, "frost.runtime_shadow_soil_water_m_0001"),
        0.014,
        "shadow layer-1 liquid soil water must round-trip",
    );
    assert_close(
        require_response_state_update(&response, "frost.runtime_shadow_frozen_depth_m_0001"),
        0.030,
        "shadow layer-1 frozen depth must round-trip",
    );
    assert_close(
        require_response_state_update(&response, "frost.runtime_shadow_frzw_m_0001"),
        0.012,
        "shadow layer-1 frzw must round-trip",
    );
    assert_close(
        require_response_state_update(&response, "frost.runtime_shadow_soilf_m_0001"),
        0.012,
        "shadow layer-1 soilf must round-trip",
    );
    assert_close(
        require_response_state_update(&response, "frost.runtime_slsic_m_0001_0001"),
        0.004,
        "shadow fine-layer ice must remain present",
    );
    assert_close(
        require_response_state_update(&response, "frost.runtime_slsw_theta_0001_0004"),
        0.2,
        "shadow unfrozen fine-layer liquid theta must remain present",
    );
}

#[test]
fn fdhp01_fine_sublayer_shadow_seam_identity_tracks_wb_delta() {
    let mut surface = seeded_clim06_surface(true);
    seed_increment_a_shadow_fine_state(&mut surface, 0.003);

    let response = execute_clim06_runoff_phase(&surface);
    assert!(
        response.status.ok_flag(),
        "Increment A shadow seam identity vector should execute successfully; status={:?}",
        response.status
    );

    let total_before =
        require_response_state_update(&response, "frost.runtime_shadow_total_water_before_m");
    let total_after =
        require_response_state_update(&response, "frost.runtime_shadow_total_water_after_m");
    let wb_delta = require_response_state_update(&response, "frost.runtime_shadow_wb_delta_m");
    let residual =
        require_response_state_update(&response, "frost.runtime_shadow_frwatc_residual_m");

    assert_close(
        wb_delta,
        0.003,
        "shadow handoff must expose the daily st-yst delta",
    );
    assert_close(
        total_after - total_before,
        wb_delta,
        "shadow total fine water change must equal the water-balance delta",
    );
    assert_close(
        residual,
        0.0,
        "shadow seam conservation residual must close",
    );
}

#[test]
fn fdhp01_fine_sublayer_state_drives_active_depth_outputs() {
    let mut shadow_surface = seeded_clim06_surface(true);
    seed_increment_a_shadow_fine_state(&mut shadow_surface, 0.003);
    let mut active_only_surface = shadow_surface.clone();
    remove_state_prefixes(
        &mut active_only_surface,
        &[
            "frost.runtime_fgfrst_",
            "frost.runtime_slfsd_m_",
            "frost.runtime_slsic_m_",
            "frost.runtime_slsw_theta_",
            "frost.runtime_sltime_s_",
            "frost.runtime_yst_m_",
            "frost.runtime_nwfrzz_m_",
        ],
    );

    let shadow_response = execute_clim06_runoff_phase(&shadow_surface);
    let active_response = execute_clim06_runoff_phase(&active_only_surface);
    assert!(shadow_response.status.ok_flag());
    assert!(active_response.status.ok_flag());

    let fine_driven_depth = require_response_state_update(&shadow_response, "frost.runtime_frdp_m");
    let coarse_driven_depth =
        require_response_state_update(&active_response, "frost.runtime_frdp_m");
    assert!(
        (fine_driven_depth - coarse_driven_depth).abs() > CLIM06_TEST_TOLERANCE,
        "Increment B must bind active depth to the persisted fine-layer state"
    );
    assert_close(
        fine_driven_depth,
        response_fine_layer_sum(&shadow_response, "frost.runtime_slfsd_m", 1, 10)
            + response_fine_layer_sum(&shadow_response, "frost.runtime_slfsd_m", 2, 10),
        "runtime frdp must be derived from fine-layer frozen thickness",
    );
}

#[test]
fn fdhp01_frostn_dispatch_arms_match_inv_snowfreeze_012() {
    let mut cold_new_surface = seeded_clim06_surface(true);
    insert_state_scalar(
        &mut cold_new_surface,
        "wb12_runoff_closure_tolerance",
        1000.0,
    );
    let cold_new = execute_clim06_runoff_phase(&cold_new_surface);
    assert!(cold_new.status.ok_flag());
    assert_close(
        require_response_state_update(&cold_new, "frost.hourly.frzflg_0001"),
        1.0,
        "cold no-sandwich frost start must dispatch bottom/front freezing",
    );

    let mut sandwich_cold = seeded_clim06_surface(true);
    insert_state_scalar(&mut sandwich_cold, "wb12_runoff_closure_tolerance", 1000.0);
    seed_increment_a_shadow_fine_state(&mut sandwich_cold, 0.0);
    insert_state_scalar(&mut sandwich_cold, "frost.runtime_fgfrst_0001_0001", 3.0);
    insert_state_scalar(&mut sandwich_cold, "frost.runtime_slfsd_m_0001_0001", 0.005);
    insert_state_scalar(&mut sandwich_cold, "frost.runtime_slsic_m_0001_0001", 0.002);
    let sandwich_cold = execute_clim06_runoff_phase(&sandwich_cold);
    assert!(
        sandwich_cold.status.ok_flag(),
        "sandwich cold vector failed: status={:?}",
        sandwich_cold.status
    );
    assert_close(
        require_response_state_update(&sandwich_cold, "frost.hourly.frzflg_0001"),
        2.0,
        "cold sandwich state must dispatch top-freeze/bottom-thaw arm",
    );

    let mut balanced = seeded_clim06_surface(true);
    insert_state_scalar(&mut balanced, "wb12_runoff_closure_tolerance", 1000.0);
    insert_state_scalar(&mut balanced, "tmax", 0.0);
    insert_state_scalar(&mut balanced, "tmin", 0.0);
    let balanced = execute_clim06_runoff_phase(&balanced);
    assert!(balanced.status.ok_flag());
    assert_close(
        require_response_state_update(&balanced, "frost.hourly.frzflg_0001"),
        0.0,
        "balanced no-frost state must expose the no-dispatch arm",
    );
}

#[test]
fn fdhp01_fine_sublayer_freeze_front_steps_by_energy_and_resistance() {
    let mut surface = seeded_clim06_surface(true);
    insert_state_scalar(&mut surface, "wb12_runoff_closure_tolerance", 1000.0);
    seed_increment_a_shadow_fine_state(&mut surface, 0.0);
    insert_state_scalar(&mut surface, "tmax", -12.0);
    insert_state_scalar(&mut surface, "tmin", -24.0);

    let response = execute_clim06_runoff_phase(&surface);
    assert!(response.status.ok_flag());

    let front_fine_depth =
        require_response_state_update(&response, "frost.runtime_slfsd_m_0001_0004");
    let front_fine_ice =
        require_response_state_update(&response, "frost.runtime_slsic_m_0001_0004");
    assert!(
        front_fine_depth > CLIM06_TEST_TOLERANCE,
        "freeze energy must advance the next fine layer before aggregate depth changes"
    );
    assert!(
        front_fine_ice > CLIM06_TEST_TOLERANCE,
        "front advance must accumulate fine-layer ice mass"
    );

    let prior_depth = 0.030;
    let hour_1_depth =
        require_response_state_update(&response, "frost.hourly.tilled_frozen_depth_m_0001");
    let hour_2_depth =
        require_response_state_update(&response, "frost.hourly.tilled_frozen_depth_m_0002");
    assert!(
        hour_2_depth - hour_1_depth <= hour_1_depth - prior_depth + CLIM06_TEST_TOLERANCE,
        "increasing frozen-layer resistance should not accelerate the second hourly increment"
    );
}

#[test]
fn fdhp01_db_freeze_front_recomputes_resistance_within_hour() {
    let mut surface = seeded_clim06_surface(true);
    seed_db_thin_front_frost(&mut surface);
    insert_state_scalar(&mut surface, "tmax", -8.086);
    insert_state_scalar(&mut surface, "tmin", -8.086);

    let response = execute_clim06_runoff_phase(&surface);
    assert!(
        response.status.ok_flag(),
        "Db thin-front vector should execute successfully; status={:?}",
        response.status
    );

    let initial_depth_m = 0.0004;
    let hour_1_depth_m =
        require_response_state_update(&response, "frost.hourly.tilled_frozen_depth_m_0001");
    let hour_2_depth_m =
        require_response_state_update(&response, "frost.hourly.tilled_frozen_depth_m_0002");
    assert!(
        hour_1_depth_m > initial_depth_m + CLIM06_TEST_TOLERANCE,
        "sustained cooling must still advance the thin front"
    );
    assert!(
        hour_1_depth_m - initial_depth_m <= 0.060,
        "one cold hour must not spend start-hour thin-front resistance across the profile; advanced {} m",
        hour_1_depth_m - initial_depth_m
    );
    assert!(
        hour_2_depth_m - hour_1_depth_m <= hour_1_depth_m - initial_depth_m + 0.005,
        "front advance must remain resistance-limited after the first hour: h1={hour_1_depth_m}, h2={hour_2_depth_m}"
    );

    let initial_implied_qsrf = 8.086 / (initial_depth_m / 1.75);
    let hour_1_implied_qsrf = 8.086 / (hour_1_depth_m / 1.75);
    assert!(
        hour_1_implied_qsrf < initial_implied_qsrf * 0.05,
        "the frozen path grown inside hour 1 must materially reduce the next surface-flux slice"
    );
}

#[test]
fn fdhp01_fine_sublayer_frznw_refreezes_nwfrzz_once() {
    let mut surface = seeded_clim06_surface(true);
    insert_state_scalar(&mut surface, "wb12_runoff_closure_tolerance", 1000.0);
    seed_increment_a_shadow_fine_state(&mut surface, 0.0);
    insert_state_scalar(&mut surface, "frost.runtime_nwfrzz_m_0001", 0.002);
    insert_state_scalar(&mut surface, "tmax", -12.0);
    insert_state_scalar(&mut surface, "tmin", -24.0);

    let response = execute_clim06_runoff_phase(&surface);
    assert!(response.status.ok_flag());

    let nwfrzz_after = require_response_state_update(&response, "frost.runtime_nwfrzz_m_0001");
    let fine_ice_after = response_fine_layer_sum(&response, "frost.runtime_slsic_m", 1, 10);
    assert_close(
        nwfrzz_after,
        0.0,
        "frznw must consume frozen-zone liquid before ordinary front extension",
    );
    assert!(
        fine_ice_after >= 0.014 - CLIM06_TEST_TOLERANCE,
        "frozen-zone liquid must be added to fine-layer ice exactly once before front motion"
    );
    assert_close(
        require_response_state_update(&response, "frost.runtime_shadow_frwatc_residual_m"),
        0.0,
        "frznw refreeze must preserve the fine-layer handoff mass identity",
    );
}

#[test]
fn fdhp01_watdst_mode_flags_update_depths_and_sltime() {
    let mut surface = seeded_clim06_surface(true);
    insert_state_scalar(&mut surface, "wb12_runoff_closure_tolerance", 1000.0);
    seed_increment_a_shadow_fine_state(&mut surface, 0.0);
    insert_state_scalar(&mut surface, "frost.runtime_frdp_m", 0.20);
    insert_state_scalar(&mut surface, "frost.runtime_sltime_s_0001_0004", 1800.0);

    let response = execute_clim06_runoff_phase(&surface);
    assert!(response.status.ok_flag());

    let fine_depth = response_fine_layer_sum(&response, "frost.runtime_slfsd_m", 1, 10)
        + response_fine_layer_sum(&response, "frost.runtime_slfsd_m", 2, 10);
    assert_close(
        require_response_state_update(&response, "frost.runtime_frdp_m"),
        fine_depth,
        "watdst mode 2 must recompute global depth from fine flags instead of seeded scalar frdp",
    );
    assert_close(
        require_response_state_update(&response, "frost.runtime_sltime_s_0001_0004"),
        0.0,
        "hourly frost dispatch must reset sltime after watdst accounting",
    );
}

#[test]
fn fdhp01_c1b_rejects_persisted_fine_ice_above_capacity_without_clamping() {
    let mut surface = seeded_clim06_surface(true);
    seed_increment_a_shadow_fine_state(&mut surface, 0.0);
    insert_state_scalar(&mut surface, "wb18_perc_ul_0001", 0.020);

    let response = execute_clim06_runoff_phase(&surface);

    assert!(
        !response.status.ok_flag(),
        "C1b must fail closed on persisted slsic above ul/dg*slfsd instead of silently clamping; status={:?}",
        response.status
    );
}

#[test]
fn fdhp01_c1b_freeze_path_respects_fine_layer_pore_capacity() {
    let mut surface = seeded_clim06_surface(true);
    configure_fdhp01_frost_only_no_flux(&mut surface);
    insert_state_scalar(&mut surface, "wb11_soil_water", 0.040);
    insert_state_scalar(&mut surface, "wb18_perc_theta_0001", 0.020);
    insert_state_scalar(&mut surface, "wb18_perc_theta_0002", 0.020);
    insert_state_scalar(&mut surface, "wb18_perc_ul_0001", 0.030);
    insert_state_scalar(&mut surface, "wb18_perc_ul_0002", 0.030);
    insert_state_scalar(&mut surface, "tmax", -18.0);
    insert_state_scalar(&mut surface, "tmin", -24.0);

    let response = execute_clim06_runoff_phase(&surface);
    assert!(
        response.status.ok_flag(),
        "capacity-bound freeze vector should execute successfully; status={:?}",
        response.status
    );

    for layer_index in 1..=2 {
        let ul =
            require_response_state_update(&response, &format!("wb18_perc_frzw_{layer_index:04}"));
        assert!(
            ul <= 0.030 + CLIM06_TEST_TOLERANCE,
            "aggregate frzw for layer {layer_index} must stay within layer ul, observed {ul}"
        );
        for fine_index in 1..=10 {
            let slfsd = require_response_state_update(
                &response,
                &fine_frost_symbol("frost.runtime_slfsd_m", layer_index, fine_index),
            );
            let slsic = require_response_state_update(
                &response,
                &fine_frost_symbol("frost.runtime_slsic_m", layer_index, fine_index),
            );
            let fine_capacity = 0.030 / 0.100 * slfsd;
            assert!(
                slsic <= fine_capacity + CLIM06_TEST_TOLERANCE,
                "fine layer {layer_index}/{fine_index} ice must stay within pore capacity: slsic={slsic}, capacity={fine_capacity}"
            );
        }
    }
}

#[test]
fn fdhp01_c1b_capacity_uses_active_ul_above_residual() {
    let mut surface = seeded_clim06_surface(true);
    configure_fdhp01_frost_only_no_flux(&mut surface);
    insert_state_scalar(&mut surface, "wb11_soil_water", 0.070);
    insert_state_scalar(&mut surface, "thetdr_0001", 0.100);
    insert_state_scalar(&mut surface, "thetdr_0002", 0.100);
    insert_state_scalar(&mut surface, "wb18_perc_theta_0001", 0.025);
    insert_state_scalar(&mut surface, "wb18_perc_theta_0002", 0.025);
    insert_state_scalar(&mut surface, "wb18_perc_ul_0001", 0.030);
    insert_state_scalar(&mut surface, "wb18_perc_ul_0002", 0.030);

    let response = execute_clim06_runoff_phase(&surface);
    assert!(
        response.status.ok_flag(),
        "wb18_perc_ul is active storage above residual; capacity must include thetdr before rejecting fine-layer liquid, status={:?}",
        response.status
    );

    for layer_index in 1..=2 {
        let total_capacity_theta = 0.100 + 0.030 / 0.100;
        for fine_index in 1..=10 {
            let slsw = require_response_state_update(
                &response,
                &fine_frost_symbol("frost.runtime_slsw_theta", layer_index, fine_index),
            );
            assert!(
                slsw <= total_capacity_theta + CLIM06_TEST_TOLERANCE,
                "fine layer {layer_index}/{fine_index} liquid must use total pore capacity: slsw={slsw}, capacity={total_capacity_theta}"
            );
        }
    }
}

#[test]
fn fdhp01_c1b_overflow_routes_to_watbtm_and_closes_shadow_identity() {
    let mut surface = seeded_clim06_surface(true);
    configure_fdhp01_frost_only_no_flux(&mut surface);
    insert_state_scalar(&mut surface, "wb11_soil_water", 0.010);
    insert_state_scalar(&mut surface, "wb18_perc_theta_0001", 0.005);
    insert_state_scalar(&mut surface, "wb18_perc_theta_0002", 0.005);
    insert_state_scalar(&mut surface, "wb18_perc_ul_0001", 0.006);
    insert_state_scalar(&mut surface, "wb18_perc_ul_0002", 0.006);
    insert_state_scalar(&mut surface, "frost.runtime_yst_m_0001", 0.0);
    insert_state_scalar(&mut surface, "tmax", -0.25);
    insert_state_scalar(&mut surface, "tmin", -0.25);

    let response = execute_clim06_runoff_phase(&surface);
    assert!(
        response.status.ok_flag(),
        "overflow-routing vector should execute successfully; status={:?}",
        response.status
    );

    let watbtm = require_response_state_update(&response, "frost.runtime_watbtm_m");
    assert!(
        watbtm > CLIM06_TEST_TOLERANCE,
        "valid excess fine-layer liquid must route to watbtm instead of hidden storage"
    );
    assert_close(
        require_response_state_update(&response, "frost.runtime_shadow_frwatc_residual_m"),
        0.0,
        "shadow identity must include routed overflow",
    );
    assert_close(
        require_response_state_update(&response, "frost.runtime_watpdg_m"),
        0.0,
        "freeze-side lower overflow vector must not create surface ponding",
    );
    assert_close(
        require_response_state_update(&response, "frost.runtime_frwatc_soil_water_after_m"),
        require_response_state_update(&response, "frost.runtime_frwatc_soil_water_before_m")
            + require_response_state_update(&response, "frost.runtime_frwatc_net_liquid_delta_m"),
        "frwatc liquid after must include overflow in the net liquid delta",
    );
}

#[test]
fn fdhp01_c2_mltbtm_bottom_thaw_recedes_front_and_routes_overflow() {
    let mut surface = seeded_clim06_surface(true);
    seed_c2_full_top_layer_frost(&mut surface);
    insert_state_scalar(&mut surface, "frost.runtime_nwfrzz_m_0001", 0.004);
    insert_state_scalar(&mut surface, "tmax", 0.0);
    insert_state_scalar(&mut surface, "tmin", 0.0);

    let response = execute_clim06_runoff_phase(&surface);
    assert!(
        response.status.ok_flag(),
        "bottom-thaw vector should execute successfully; status={:?}",
        response.status
    );

    assert_close(
        require_response_state_update(&response, "frost.hourly.frzflg_0001"),
        4.0,
        "positive lower heat with neutral surface heat must dispatch bottom thaw",
    );
    assert!(
        require_response_state_update(&response, "frost.runtime_frdp_m")
            < 0.100 - CLIM06_TEST_TOLERANCE,
        "bottom thaw must retreat the lower frost front",
    );
    assert!(
        response_fine_flag_count(&response, 2.0) > 0,
        "partial bottom thaw must leave an active frost-at-top fine-layer front",
    );
    assert!(
        require_response_state_update(&response, "frost.runtime_nwfrzz_m_0001")
            < 0.004 - CLIM06_TEST_TOLERANCE,
        "bottom thaw must release liquid held in the frozen zone",
    );
    assert!(
        require_response_state_update(&response, "frost.runtime_watbtm_m") > CLIM06_TEST_TOLERANCE,
        "capacity-excess bottom thaw release must route to watbtm",
    );
    assert_close(
        require_response_state_update(&response, "frost.runtime_shadow_frwatc_residual_m"),
        0.0,
        "bottom thaw release and overflow must stay in the C1b identity",
    );
}

#[test]
fn fdhp01_c2_mlttp_top_thaw_sets_sandwich_geometry_and_fgthwd() {
    let mut surface = seeded_clim06_surface(true);
    seed_c2_full_top_layer_frost(&mut surface);
    insert_state_scalar(&mut surface, "tmax", 0.10);
    insert_state_scalar(&mut surface, "tmin", 0.10);

    let response = execute_clim06_runoff_phase(&surface);
    assert!(
        response.status.ok_flag(),
        "top-thaw vector should execute successfully; status={:?}",
        response.status
    );

    assert_close(
        require_response_state_update(&response, "frost.hourly.frzflg_0001"),
        3.0,
        "positive surface heat over existing frost must dispatch top thaw",
    );
    assert!(
        response_fine_flag_count(&response, 3.0) > 0,
        "partial top thaw must leave an active frost-at-bottom fine-layer front",
    );
    assert!(
        require_response_state_update(&response, "frost.runtime_thdp_m") > CLIM06_TEST_TOLERANCE,
        "top thaw must publish a positive thawed-from-surface depth",
    );
    assert!(
        require_response_state_update(&response, "frost.runtime_frdp_m")
            > require_response_state_update(&response, "frost.runtime_thdp_m"),
        "remaining frost below top thaw must keep bottom frost depth below the thawed surface layer",
    );
    assert_close(
        require_response_state_update(&response, "frost.runtime_fgthwd_flag"),
        0.0,
        "partial top thaw must not mark thaw-through complete",
    );

    insert_state_scalar(&mut surface, "tmax", 80.0);
    insert_state_scalar(&mut surface, "tmin", 60.0);
    let thaw_through = execute_clim06_runoff_phase(&surface);
    assert!(
        thaw_through.status.ok_flag(),
        "thaw-through vector should execute successfully; status={:?}",
        thaw_through.status
    );
    assert_close(
        require_response_state_update(&thaw_through, "frost.runtime_frdp_m"),
        0.0,
        "sufficient top-thaw energy must clear frost depth",
    );
    assert_close(
        require_response_state_update(&thaw_through, "frost.runtime_fgthwd_flag"),
        1.0,
        "thaw-through must set fgthwd for early frwatc(0) semantics",
    );
}

#[test]
fn fdhp01_c2_multicycle_freeze_thaw_does_not_amplify_storage_without_input() {
    let mut surface = seeded_clim06_surface(true);
    seed_c2_full_top_layer_frost(&mut surface);
    let initial_total = 0.020
        + require_response_state_update(
            &execute_clim06_runoff_phase(&surface),
            "frost.runtime_frwatc_frozen_water_before_m",
        );

    let mut max_total = initial_total;
    for cycle in 0..4 {
        insert_state_scalar(&mut surface, "tmax", -18.0);
        insert_state_scalar(&mut surface, "tmin", -24.0);
        let freeze = execute_clim06_runoff_phase(&surface);
        apply_response_state_updates(&mut surface, &freeze);

        insert_state_scalar(&mut surface, "tmax", 18.0);
        insert_state_scalar(&mut surface, "tmin", 12.0);
        let thaw = execute_clim06_runoff_phase(&surface);
        apply_response_state_updates(&mut surface, &thaw);

        let total = require_response_state_update(&thaw, "frost.runtime_frwatc_soil_water_after_m")
            + require_response_state_update(&thaw, "frost.runtime_frwatc_frozen_water_after_m")
            + require_response_state_update(&thaw, "frost.runtime_watpdg_m")
            + require_response_state_update(&thaw, "frost.runtime_watbtm_m");
        max_total = max_total.max(total);
        assert!(
            total <= initial_total + 1.0e-5,
            "cycle {cycle} must not amplify closed-system water storage: initial={initial_total}, total={total}"
        );
    }
    assert!(
        max_total <= initial_total + 1.0e-5,
        "multi-cycle thaw must not recreate the prior geometric-amplification signature"
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
    seed_increment_a_shadow_fine_state(&mut surface, 0.0);
    surface
        .state_surface
        .insert(BoundarySymbol::from("tmax"), BoundaryValue::scalar(-20.0));
    surface
        .state_surface
        .insert(BoundarySymbol::from("tmin"), BoundaryValue::scalar(-40.0));
    configure_fdhp01_deep_profile(&mut surface);

    let mut dfrost = 0.0;
    let mut frdp = 0.0;
    for _day in 0..5 {
        let response = execute_clim06_runoff_phase(&surface);
        assert!(
            response.status.ok_flag(),
            "expected deep frost vector to succeed, status={:?}",
            response.status
        );
        dfrost = require_response_state_update(&response, "frost.runtime_dfrost");
        frdp = require_response_state_update(&response, "frost.runtime_frdp_m");
        apply_response_state_updates(&mut surface, &response);
    }
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
        liquid_after - liquid_before,
        net_liquid_delta,
        "thaw net liquid delta must equal the actual frwatc liquid handoff",
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
