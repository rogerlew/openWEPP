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
    state_surface.insert(
        BoundarySymbol::from("solwpv"),
        BoundaryValue::scalar(2006.0),
    );
    state_surface.insert(BoundarySymbol::from("dg"), BoundaryValue::scalar(0.1));
    state_surface.insert(BoundarySymbol::from("thetdr"), BoundaryValue::scalar(0.1));
    state_surface.insert(BoundarySymbol::from("thetfc"), BoundaryValue::scalar(0.3));
    state_surface.insert(BoundarySymbol::from("ssc"), BoundaryValue::scalar(2.0));
    state_surface.insert(BoundarySymbol::from("cancov"), BoundaryValue::scalar(0.0));
    state_surface.insert(BoundarySymbol::from("lai"), BoundaryValue::scalar(0.0));
    state_surface.insert(BoundarySymbol::from("rtd"), BoundaryValue::scalar(0.0));
    state_surface.insert(BoundarySymbol::from("pltol"), BoundaryValue::scalar(0.25));
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
        BoundarySymbol::from("wb20_forward_solver_lane_enabled"),
        BoundaryValue::scalar(1.0),
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

fn enable_mofe_hourly_carry_arrays(
    surface: &mut HillslopeWritebackSurface,
    upstream_saturation: &[(usize, f64)],
    upstream_lateral: &[(usize, f64)],
) {
    surface.state_surface.insert(
        BoundarySymbol::from("mofe_hourly_carry_arrays_enabled"),
        BoundaryValue::scalar(1.0),
    );
    surface.state_surface.insert(
        BoundarySymbol::from("mofe_hourly_upstream_area_ratio"),
        BoundaryValue::scalar(1.0),
    );
    for hour in 1..=24 {
        for root in ["ui_SUrunf", "ui_SCrunf", "ui_LfUrf", "ui_LfCrf"] {
            surface.state_surface.insert(
                BoundarySymbol::from(format!("{root}_{hour:04}")),
                BoundaryValue::scalar(0.0),
            );
        }
    }
    for (hour, value) in upstream_saturation {
        surface.state_surface.insert(
            BoundarySymbol::from(format!("ui_SUrunf_{hour:04}")),
            BoundaryValue::scalar(*value),
        );
    }
    for (hour, value) in upstream_lateral {
        surface.state_surface.insert(
            BoundarySymbol::from(format!("ui_LfUrf_{hour:04}")),
            BoundaryValue::scalar(*value),
        );
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

    let wb11_soil_water = report
        .writeback_surface
        .state_surface
        .get(&BoundarySymbol::from("wb11_soil_water"))
        .expect("wb11_soil_water should be present")
        .as_f64();
    assert!(
        (wb11_soil_water - 7.904_382_572_090_036).abs() <= 1.0e-12,
        "wb11_soil_water must reflect final post-WB19 aggregate storage, observed {wb11_soil_water}"
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
        Some(BoundaryValue::scalar(1.902_458_849_001_428))
    );
    assert_eq!(
        report
            .writeback_surface
            .flux_surface
            .get(&BoundarySymbol::from("Ws"))
            .copied(),
        Some(BoundaryValue::scalar(1.0))
    );
    let d_loss = report
        .writeback_surface
        .flux_surface
        .get(&BoundarySymbol::from("D"))
        .expect("D should be present")
        .as_f64();
    assert!(
        (d_loss - 0.027_369_245_807_551_727).abs() <= 1.0e-12,
        "D must reflect WB18 dynamic-Bi routing output, observed {d_loss}"
    );
    let pe_recharge = report
        .writeback_surface
        .flux_surface
        .get(&BoundarySymbol::from("Pe"))
        .expect("Pe should be present")
        .as_f64();
    assert!(
        (pe_recharge - 0.027_369_245_807_551_727).abs() <= 1.0e-12,
        "Pe must match WB18 deep-percolation loss, observed {pe_recharge}"
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
fn hphys0239_contract_wb11_hydrology_tail_order_requires_wb19_then_wb12_reconciliation() {
    let ordered = HillslopePhaseGraph::canonical_order();

    let percolation_index = ordered
        .iter()
        .position(|phase| *phase == HillslopePhase::PercolationDeepSeepage)
        .expect("percolation phase must exist");
    let evap_index = ordered
        .iter()
        .position(|phase| *phase == HillslopePhase::Evapotranspiration)
        .expect("evapotranspiration phase must exist");
    let lateral_index = ordered
        .iter()
        .position(|phase| *phase == HillslopePhase::LateralTransfer)
        .expect("lateral transfer phase must exist");
    let root_uptake_index = ordered
        .iter()
        .position(|phase| *phase == HillslopePhase::PlantRootUptake)
        .expect("plant root uptake phase must exist");
    let drainage_index = ordered
        .iter()
        .position(|phase| *phase == HillslopePhase::Drainage)
        .expect("drainage phase must exist");
    let runoff_index = ordered
        .iter()
        .position(|phase| *phase == HillslopePhase::RunoffReconciliation)
        .expect("runoff reconciliation phase must exist");
    let storage_index = ordered
        .iter()
        .position(|phase| *phase == HillslopePhase::StorageReconciliation)
        .expect("storage reconciliation phase must exist");

    assert!(
        percolation_index < evap_index
            && evap_index < drainage_index
            && drainage_index < lateral_index
            && lateral_index < root_uptake_index
            && root_uptake_index < runoff_index
            && runoff_index < storage_index,
        "canonical hydrology-tail ordering must remain Percolation -> ET -> Drainage -> Lateral -> PlantRootUptake -> RunoffReconciliation -> StorageReconciliation"
    );

    let graph = HillslopePhaseGraph::canonical();
    assert!(
        graph
            .dependencies_for(HillslopePhase::Evapotranspiration)
            .contains(&HillslopePhase::PercolationDeepSeepage),
        "evapotranspiration must depend on percolation"
    );
    assert!(
        graph
            .dependencies_for(HillslopePhase::Drainage)
            .contains(&HillslopePhase::Evapotranspiration),
        "drainage must depend on evapotranspiration"
    );
    assert!(
        graph
            .dependencies_for(HillslopePhase::LateralTransfer)
            .contains(&HillslopePhase::Drainage),
        "lateral transfer must depend on drainage"
    );
    assert!(
        graph
            .dependencies_for(HillslopePhase::RunoffReconciliation)
            .contains(&HillslopePhase::PlantRootUptake),
        "runoff reconciliation must depend on plant root uptake"
    );
    assert!(
        graph
            .dependencies_for(HillslopePhase::PlantRootUptake)
            .contains(&HillslopePhase::LateralTransfer),
        "plant root uptake must depend on lateral transfer"
    );
    assert!(
        graph
            .dependencies_for(HillslopePhase::StorageReconciliation)
            .contains(&HillslopePhase::RunoffReconciliation),
        "storage reconciliation must depend on runoff reconciliation"
    );
}

#[test]
fn hphys0240_contract_wb11_carryover_tail_requires_storage_after_runoff() {
    let graph = HillslopePhaseGraph::canonical();
    assert!(
        graph
            .dependencies_for(HillslopePhase::RunoffReconciliation)
            .contains(&HillslopePhase::PlantRootUptake),
        "carryover-producing tail must complete SWU after WB19 before runoff reconciliation"
    );
    assert!(
        graph
            .dependencies_for(HillslopePhase::StorageReconciliation)
            .contains(&HillslopePhase::RunoffReconciliation),
        "storage reconciliation must consume Q after runoff carryover resolution"
    );
}

#[test]
fn hphys0242_contract_wb11_hourly_tail_requires_drainage_before_lateral_and_same_pass_storage() {
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
        "HPHYS0242 requires final-hour ET after same-pass percolation"
    );
    assert!(
        phase_index(HillslopePhase::Evapotranspiration) < phase_index(HillslopePhase::Drainage)
            && phase_index(HillslopePhase::Drainage) < phase_index(HillslopePhase::LateralTransfer)
            && phase_index(HillslopePhase::LateralTransfer)
                < phase_index(HillslopePhase::PlantRootUptake)
            && phase_index(HillslopePhase::PlantRootUptake)
                < phase_index(HillslopePhase::RunoffReconciliation)
            && phase_index(HillslopePhase::RunoffReconciliation)
                < phase_index(HillslopePhase::StorageReconciliation),
        "HPHYS0242 hourly tail must be ET -> Drainage -> Lateral -> PlantRootUptake -> Runoff -> Storage"
    );
}

#[test]
fn hphys0241_contract_mofe_hourly_arrays_drive_runoff_carryover_and_copy_forward() {
    const TOL: f64 = 1.0e-12;

    let graph = parse_topology_fixture_str(VALID_TOPOLOGY).expect("fixture should parse");
    let topology_report =
        validate_pre_execution_topology(&graph).expect("topology report should build");
    let scheduler = HillslopePhaseScheduler::canonical();
    let mut kernel = Wb11HydrologyKernel;

    let mut surface = seeded_wb11_surface();
    enable_mofe_hourly_carry_arrays(&mut surface, &[(1, 0.10)], &[(2, 0.05)]);
    surface.state_surface.insert(
        BoundarySymbol::from("wb20_forward_solver_lane_enabled"),
        BoundaryValue::scalar(1.0),
    );
    surface.state_surface.insert(
        BoundarySymbol::from("wb12_runon_input"),
        BoundaryValue::scalar(0.80),
    );
    surface.state_surface.insert(
        BoundarySymbol::from("wb12_depression_storage_delta"),
        BoundaryValue::scalar(0.0),
    );

    let report = scheduler
        .execute_with_kernel(&topology_report, &mut kernel, surface)
        .expect("HPHYS0241 MOFE carry-array vector should return typed report");
    assert!(
        report.scheduler_report.is_success(),
        "scheduler halted at {:?}",
        report.scheduler_report.halted_phase
    );

    let carryover = require_flux_scalar(&report, "wb12_runoff_carryover");
    assert!(
        (carryover - 0.15).abs() <= TOL,
        "array-derived carryover must override compatibility runon state, observed {carryover}"
    );

    for hour in 1..=24 {
        let current_saturation = require_state_scalar(&report, &format!("ui_SCrunf_{hour:04}"));
        let upstream_saturation = require_state_scalar(&report, &format!("ui_SUrunf_{hour:04}"));
        let current_lateral = require_state_scalar(&report, &format!("ui_LfCrf_{hour:04}"));
        let upstream_lateral = require_state_scalar(&report, &format!("ui_LfUrf_{hour:04}"));
        assert!(
            current_saturation >= -TOL && current_lateral >= -TOL,
            "current carry arrays must remain non-negative"
        );
        assert!(
            (upstream_saturation - current_saturation).abs() <= TOL,
            "ui_SUrunf must copy-forward ui_SCrunf at hour {hour}"
        );
        assert!(
            (upstream_lateral - current_lateral).abs() <= TOL,
            "ui_LfUrf must copy-forward ui_LfCrf at hour {hour}"
        );
    }
}

#[test]
fn hphys0241_contract_mofe_hourly_arrays_reject_negative_upstream_payload() {
    let graph = parse_topology_fixture_str(VALID_TOPOLOGY).expect("fixture should parse");
    let topology_report =
        validate_pre_execution_topology(&graph).expect("topology report should build");
    let scheduler = HillslopePhaseScheduler::canonical();
    let mut kernel = Wb11HydrologyKernel;

    let mut surface = seeded_wb11_surface();
    enable_mofe_hourly_carry_arrays(&mut surface, &[], &[(7, -0.001)]);

    let report = scheduler
        .execute_with_kernel(&topology_report, &mut kernel, surface)
        .expect("HPHYS0241 malformed carry-array vector should return typed report");

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
