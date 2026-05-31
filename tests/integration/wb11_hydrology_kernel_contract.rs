use openwepp_hillslope_orchestrator::{
    HillslopePhase, HillslopePhaseGraph, HillslopePhaseScheduler, HillslopeWritebackSurface,
    Wb11HydrologyKernel,
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

#[allow(clippy::too_many_lines)]
fn seeded_wb11_surface() -> HillslopeWritebackSurface {
    let mut state_surface = std::collections::BTreeMap::new();

    // Seed soil family sentinels used by consumer-boundary guards.
    state_surface.insert(BoundarySymbol::from("nsl"), BoundaryValue::scalar(2.0));
    state_surface.insert(BoundarySymbol::from("solthk"), BoundaryValue::scalar(0.3));
    state_surface.insert(BoundarySymbol::from("dg"), BoundaryValue::scalar(0.1));
    state_surface.insert(BoundarySymbol::from("thetdr"), BoundaryValue::scalar(0.1));
    state_surface.insert(BoundarySymbol::from("thetfc"), BoundaryValue::scalar(0.3));
    state_surface.insert(BoundarySymbol::from("ssc"), BoundaryValue::scalar(2.0));
    state_surface.insert(BoundarySymbol::from("cancov"), BoundaryValue::scalar(0.0));
    state_surface.insert(BoundarySymbol::from("lai"), BoundaryValue::scalar(0.0));
    state_surface.insert(BoundarySymbol::from("vdmt"), BoundaryValue::scalar(0.0));

    // WB11 kernel state inputs.
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

    // WB12 reconciliation inputs so canonical scheduler completion can proceed
    // beyond WB11 hydrology phases in the nominal success vector.
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
        BoundaryValue::scalar(12.5),
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

#[test]
fn wb11_contract_conformance_kernel_updates_et_perc_lateral_drain_surfaces() {
    let graph = parse_topology_fixture_str(VALID_TOPOLOGY).expect("fixture should parse");
    let topology_report =
        validate_pre_execution_topology(&graph).expect("topology report should build");
    let scheduler = HillslopePhaseScheduler::canonical();
    let mut kernel = Wb11HydrologyKernel;

    let report = scheduler
        .execute_with_kernel(&topology_report, &mut kernel, seeded_wb11_surface())
        .expect("wb11 execution should return typed report");

    assert!(
        report.scheduler_report.is_success(),
        "scheduler halted at {:?}",
        report.scheduler_report.halted_phase,
    );

    assert_eq!(
        report
            .writeback_surface
            .state_surface
            .get(&BoundarySymbol::from("wb11_soil_water"))
            .copied(),
        Some(BoundaryValue::scalar(7.0))
    );
    let drainable_storage = report
        .writeback_surface
        .state_surface
        .get(&BoundarySymbol::from("wb11_drainable_storage"))
        .expect("wb11_drainable_storage should be present")
        .as_f64();
    assert!(
        (0.0..=2.0 + 1.0e-12).contains(&drainable_storage),
        "wb11_drainable_storage must remain bounded and non-negative, observed {drainable_storage}"
    );

    assert_eq!(
        report
            .writeback_surface
            .flux_surface
            .get(&BoundarySymbol::from("ET"))
            .copied(),
        Some(BoundaryValue::scalar(2.0))
    );
    assert_eq!(
        report
            .writeback_surface
            .flux_surface
            .get(&BoundarySymbol::from("Ws"))
            .copied(),
        Some(BoundaryValue::scalar(1.0))
    );
    assert_eq!(
        report
            .writeback_surface
            .flux_surface
            .get(&BoundarySymbol::from("D"))
            .copied(),
        Some(BoundaryValue::scalar(1.0))
    );
    assert_eq!(
        report
            .writeback_surface
            .flux_surface
            .get(&BoundarySymbol::from("Pe"))
            .copied(),
        Some(BoundaryValue::scalar(1.0))
    );
    let q_lateral = report
        .writeback_surface
        .flux_surface
        .get(&BoundarySymbol::from("q"))
        .expect("q should be present")
        .as_f64();
    let q_drainage = report
        .writeback_surface
        .flux_surface
        .get(&BoundarySymbol::from("Qdd"))
        .expect("Qdd should be present")
        .as_f64();
    let q_subhyd = report
        .writeback_surface
        .flux_surface
        .get(&BoundarySymbol::from("Qd"))
        .expect("Qd should be present")
        .as_f64();
    assert!(q_lateral >= 0.0, "q must remain non-negative");
    assert!(q_drainage >= 0.0, "Qdd must remain non-negative");
    assert!(
        (q_subhyd - (q_lateral + q_drainage)).abs() <= 1.0e-12,
        "Qd must satisfy coupled continuity Qd = q + Qdd"
    );
}

#[test]
fn wb11_contract_conformance_rejects_non_finite_et_demand() {
    let graph = parse_topology_fixture_str(VALID_TOPOLOGY).expect("fixture should parse");
    let topology_report =
        validate_pre_execution_topology(&graph).expect("topology report should build");
    let scheduler = HillslopePhaseScheduler::canonical();
    let mut kernel = Wb11HydrologyKernel;

    let mut surface = seeded_wb11_surface();
    surface.state_surface.insert(
        BoundarySymbol::from("wb11_et_demand"),
        BoundaryValue::scalar(f64::NAN),
    );

    let report = scheduler
        .execute_with_kernel(&topology_report, &mut kernel, surface)
        .expect("wb11 failure should return typed report");

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
fn wb11_contract_conformance_rejects_invalid_percolation_fraction() {
    let graph = parse_topology_fixture_str(VALID_TOPOLOGY).expect("fixture should parse");
    let topology_report =
        validate_pre_execution_topology(&graph).expect("topology report should build");
    let scheduler = HillslopePhaseScheduler::canonical();
    let mut kernel = Wb11HydrologyKernel;

    let mut surface = seeded_wb11_surface();
    surface.state_surface.insert(
        BoundarySymbol::from("wb11_perc_fraction"),
        BoundaryValue::scalar(1.2),
    );

    let report = scheduler
        .execute_with_kernel(&topology_report, &mut kernel, surface)
        .expect("wb11 failure should return typed report");

    assert_eq!(
        report.scheduler_report.halted_phase,
        Some(HillslopePhase::PercolationDeepSeepage)
    );

    let perc_phase = report
        .phase_reports
        .iter()
        .find(|phase| phase.phase == HillslopePhase::PercolationDeepSeepage)
        .expect("percolation phase report should exist");
    assert_eq!(
        perc_phase.decision_status.message_id(),
        "HKERNEL-WB11-PERC-E-003"
    );
    assert_eq!(
        perc_phase.decision_status.boundary_class(),
        BoundaryClass::DomainViolation
    );
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

fn require_flux_scalar(
    report: &openwepp_hillslope_orchestrator::HillslopeKernelExecutionReport,
    symbol: &str,
) -> f64 {
    report
        .writeback_surface
        .flux_surface
        .get(&BoundarySymbol::from(symbol))
        .unwrap_or_else(|| panic!("missing expected flux symbol {symbol}"))
        .as_f64()
}

#[test]
#[allow(clippy::similar_names)]
fn simimpl22_contract_stage_memory_vector_requires_transitioning_s1_s2_tu_tv() {
    const TOL: f64 = 1.0e-12;

    let graph = parse_topology_fixture_str(VALID_TOPOLOGY).expect("fixture should parse");
    let topology_report =
        validate_pre_execution_topology(&graph).expect("topology report should build");
    let scheduler = HillslopePhaseScheduler::canonical();
    let mut kernel = Wb11HydrologyKernel;
    let mut surface = seeded_wb11_surface();

    let stage_s1_before = 0.01;
    let stage_s2_before = 0.02;
    let stage_tu_before = 3.0;
    let stage_tv_before = 1.0;
    surface.state_surface.insert(
        BoundarySymbol::from("s1"),
        BoundaryValue::scalar(stage_s1_before),
    );
    surface.state_surface.insert(
        BoundarySymbol::from("s2"),
        BoundaryValue::scalar(stage_s2_before),
    );
    surface.state_surface.insert(
        BoundarySymbol::from("tu"),
        BoundaryValue::scalar(stage_tu_before),
    );
    surface.state_surface.insert(
        BoundarySymbol::from("tv"),
        BoundaryValue::scalar(stage_tv_before),
    );

    let report = scheduler
        .execute_with_kernel(&topology_report, &mut kernel, surface)
        .expect("SIMIMPL22 stage-memory vector should return typed report");
    assert!(report.scheduler_report.is_success());

    let stage_s1_after = require_state_scalar(&report, "s1");
    let stage_s2_after = require_state_scalar(&report, "s2");
    let stage_tu_after = require_state_scalar(&report, "tu");
    let stage_tv_after = require_state_scalar(&report, "tv");

    let transitioned = (stage_s1_after - stage_s1_before).abs() > TOL
        || (stage_s2_after - stage_s2_before).abs() > TOL
        || (stage_tu_after - stage_tu_before).abs() > TOL
        || (stage_tv_after - stage_tv_before).abs() > TOL;
    assert!(
        transitioned,
        "baseline-authoritative ET stage-memory transition was not observed"
    );
}

#[test]
fn simimpl22_contract_root_uptake_vector_requires_upi_ui_etp_and_ws_lineage() {
    const TOL: f64 = 1.0e-12;

    let graph = parse_topology_fixture_str(VALID_TOPOLOGY).expect("fixture should parse");
    let topology_report =
        validate_pre_execution_topology(&graph).expect("topology report should build");
    let scheduler = HillslopePhaseScheduler::canonical();
    let mut kernel = Wb11HydrologyKernel;
    let mut surface = seeded_wb11_surface();
    surface
        .state_surface
        .insert(BoundarySymbol::from("rtd"), BoundaryValue::scalar(0.4));
    surface
        .state_surface
        .insert(BoundarySymbol::from("lai"), BoundaryValue::scalar(1.5));

    let report = scheduler
        .execute_with_kernel(&topology_report, &mut kernel, surface)
        .expect("SIMIMPL22 uptake-lineage vector should return typed report");
    assert!(report.scheduler_report.is_success());

    let upi = require_flux_scalar(&report, "UPi");
    let ui = require_flux_scalar(&report, "Ui");
    let etp = require_flux_scalar(&report, "Etp");
    let ws = require_flux_scalar(&report, "Ws");

    assert!(upi >= -TOL, "UPi must be non-negative");
    assert!(ui >= -TOL, "Ui must be non-negative");
    assert!(ui <= upi + TOL, "Ui must not exceed UPi");
    assert!(etp > TOL, "Etp must be positive for lineage ratio checks");

    let expected_ws = ui / etp;
    assert!(
        (ws - expected_ws).abs() <= 1.0e-9,
        "Ws must follow canonical lineage Ws = ΣUi / Etp"
    );
}

#[test]
fn simimpl22_contract_wb11_ordering_vector_requires_purk_before_evap() {
    let ordered = HillslopePhaseGraph::canonical_order();

    let evap_index = ordered
        .iter()
        .position(|phase| *phase == HillslopePhase::Evapotranspiration)
        .expect("evapotranspiration phase must exist");
    let percolation_index = ordered
        .iter()
        .position(|phase| *phase == HillslopePhase::PercolationDeepSeepage)
        .expect("percolation phase must exist");

    assert!(
        percolation_index < evap_index,
        "baseline WB11 ordering requires purk/percolation before evap/evappm"
    );
}

#[test]
fn simimpl22_contract_wb13_publication_vector_requires_watcon_alias_lineage() {
    const TOL: f64 = 1.0e-12;

    let graph = parse_topology_fixture_str(VALID_TOPOLOGY).expect("fixture should parse");
    let topology_report =
        validate_pre_execution_topology(&graph).expect("topology report should build");
    let scheduler = HillslopePhaseScheduler::canonical();
    let mut kernel = Wb11HydrologyKernel;

    let report = scheduler
        .execute_with_kernel(&topology_report, &mut kernel, seeded_wb11_surface())
        .expect("SIMIMPL22 WB13 lineage vector should return typed report");
    assert!(report.scheduler_report.is_success());

    let _ep = require_flux_scalar(&report, "Ep");
    let _es = require_flux_scalar(&report, "Es");
    let _er = require_flux_scalar(&report, "Er");

    let watcon = require_state_scalar(&report, "watcon");
    let total_soil = require_state_scalar(&report, "Total-Soil");
    let soil_water_total = require_state_scalar(&report, "SoilWaterTotal");

    assert!(watcon >= 0.0, "watcon must remain non-negative");
    assert!(total_soil >= 0.0, "Total-Soil must remain non-negative");
    assert!(
        soil_water_total + TOL >= total_soil,
        "SoilWaterTotal must be >= Total-Soil"
    );
}
