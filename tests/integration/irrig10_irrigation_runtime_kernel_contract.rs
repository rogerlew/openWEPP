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

#[allow(clippy::too_many_lines)]
fn seeded_irrig10_base_surface() -> HillslopeWritebackSurface {
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
        BoundaryValue::scalar(0.290_068_906_744_067),
    );
    state_surface.insert(
        BoundarySymbol::from("wb12_runoff_closure_tolerance"),
        BoundaryValue::scalar(1.0e-6),
    );

    state_surface.insert(
        BoundarySymbol::from("wb12_storage_initial"),
        BoundaryValue::scalar(12.0),
    );
    state_surface.insert(
        BoundarySymbol::from("wb12_storage_observed"),
        BoundaryValue::scalar(11.709_931_093_255_933),
    );
    state_surface.insert(
        BoundarySymbol::from("wb12_storage_closure_tolerance"),
        BoundaryValue::scalar(1.0e-6),
    );
    state_surface.insert(
        BoundarySymbol::from("wb12_precip_input"),
        BoundaryValue::scalar(3.0),
    );

    state_surface.insert(BoundarySymbol::from("day"), BoundaryValue::scalar(120.0));
    state_surface.insert(BoundarySymbol::from("year"), BoundaryValue::scalar(1.0));

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

    HillslopeWritebackSurface {
        state_surface,
        flux_surface: std::collections::BTreeMap::new(),
    }
}

fn run_surface(
    surface: HillslopeWritebackSurface,
) -> openwepp_hillslope_orchestrator::HillslopeKernelExecutionReport {
    let graph = parse_topology_fixture_str(VALID_TOPOLOGY).expect("fixture should parse");
    let topology_report =
        validate_pre_execution_topology(&graph).expect("topology report should build");
    let scheduler = HillslopePhaseScheduler::canonical();
    let mut kernel = Wb11HydrologyKernel;
    scheduler
        .execute_with_kernel(&topology_report, &mut kernel, surface)
        .expect("execution should return typed report")
}

#[test]
#[allow(clippy::too_many_lines)]
fn irrig10_fixeddate_contract_vector_couples_irrigation_depth_into_runoff_and_storage() {
    let mut baseline = seeded_irrig10_base_surface();
    baseline.state_surface.insert(
        BoundarySymbol::from("wb12_rainfall_input"),
        BoundaryValue::scalar(3.0),
    );

    let mut irrigated = seeded_irrig10_base_surface();
    irrigated.state_surface.insert(
        BoundarySymbol::from("irrigation.fixeddate.enabled"),
        BoundaryValue::scalar(1.0),
    );
    irrigated.state_surface.insert(
        BoundarySymbol::from("irrigation.fixeddate.event_count"),
        BoundaryValue::scalar(1.0),
    );
    irrigated.state_surface.insert(
        BoundarySymbol::from("irrigation.fixeddate.system_type"),
        BoundaryValue::scalar(1.0),
    );
    irrigated.state_surface.insert(
        BoundarySymbol::from("irrigation.fixeddate.event_0001.ofe_id"),
        BoundaryValue::scalar(1.0),
    );
    irrigated.state_surface.insert(
        BoundarySymbol::from("irrigation.fixeddate.event_0001.day"),
        BoundaryValue::scalar(120.0),
    );
    irrigated.state_surface.insert(
        BoundarySymbol::from("irrigation.fixeddate.event_0001.year"),
        BoundaryValue::scalar(1.0),
    );
    irrigated.state_surface.insert(
        BoundarySymbol::from("irrigation.fixeddate.event_0001.schedule_termination_flag"),
        BoundaryValue::scalar(0.0),
    );
    irrigated.state_surface.insert(
        BoundarySymbol::from("irrigation.fixeddate.event_0001.sprinkler_depth_m"),
        BoundaryValue::scalar(0.4),
    );
    irrigated.state_surface.insert(
        BoundarySymbol::from("irrigation.fixeddate.event_0001.sprinkler_rate_m_per_s"),
        BoundaryValue::scalar(0.2),
    );
    irrigated.state_surface.insert(
        BoundarySymbol::from("irrigation.fixeddate.event_0001.sprinkler_nozzle_factor"),
        BoundaryValue::scalar(1.0),
    );
    irrigated.state_surface.insert(
        BoundarySymbol::from("wb12_rainfall_input"),
        BoundaryValue::scalar(3.4),
    );
    irrigated.state_surface.insert(
        BoundarySymbol::from("wb12_runoff_closure_tolerance"),
        BoundaryValue::scalar(10.0),
    );
    irrigated.state_surface.insert(
        BoundarySymbol::from("wb12_storage_closure_tolerance"),
        BoundaryValue::scalar(10.0),
    );

    let baseline_report = run_surface(baseline);
    let irrigated_report = run_surface(irrigated);

    assert!(baseline_report.scheduler_report.is_success());
    assert!(irrigated_report.scheduler_report.is_success());

    let baseline_q = baseline_report
        .writeback_surface
        .flux_surface
        .get(&BoundarySymbol::from("Q"))
        .expect("baseline Q should exist")
        .as_f64();
    let irrigated_q = irrigated_report
        .writeback_surface
        .flux_surface
        .get(&BoundarySymbol::from("Q"))
        .expect("irrigated Q should exist")
        .as_f64();
    assert!(irrigated_q > baseline_q);

    let irrigation_depth = irrigated_report
        .writeback_surface
        .flux_surface
        .get(&BoundarySymbol::from("Irr"))
        .expect("Irr should exist for active irrigation")
        .as_f64();
    assert!((irrigation_depth - 0.4).abs() < 1.0e-12);
    let schedule_source = irrigated_report
        .writeback_surface
        .state_surface
        .get(&BoundarySymbol::from("irrigation.runtime_schedule_source"))
        .expect("runtime schedule source should be published")
        .as_f64();
    assert!((schedule_source - 2.0).abs() < 1.0e-12);

    let baseline_storage = baseline_report
        .writeback_surface
        .state_surface
        .get(&BoundarySymbol::from("wb12_storage_reconciled"))
        .expect("baseline storage should exist")
        .as_f64();
    let irrigated_storage = irrigated_report
        .writeback_surface
        .state_surface
        .get(&BoundarySymbol::from("wb12_storage_reconciled"))
        .expect("irrigated storage should exist")
        .as_f64();
    assert!(irrigated_storage > baseline_storage);
}

#[test]
#[allow(clippy::too_many_lines)]
fn irrig10_depletion_contract_vector_activates_period_trigger() {
    let mut surface = seeded_irrig10_base_surface();
    surface.state_surface.insert(
        BoundarySymbol::from("irrigation.depletion.enabled"),
        BoundaryValue::scalar(1.0),
    );
    surface.state_surface.insert(
        BoundarySymbol::from("irrigation.depletion.system_type"),
        BoundaryValue::scalar(1.0),
    );
    surface.state_surface.insert(
        BoundarySymbol::from("irrigation.depletion.min_depth_m"),
        BoundaryValue::scalar(0.05),
    );
    surface.state_surface.insert(
        BoundarySymbol::from("irrigation.depletion.max_depth_m"),
        BoundaryValue::scalar(0.3),
    );
    surface.state_surface.insert(
        BoundarySymbol::from("irrigation.depletion.period_count"),
        BoundaryValue::scalar(1.0),
    );
    surface.state_surface.insert(
        BoundarySymbol::from("wb11_soil_water"),
        BoundaryValue::scalar(7.2),
    );
    // Keep depletion-trigger vector consistent across WB11 scalar and WB18 layer state.
    surface.state_surface.insert(
        BoundarySymbol::from("wb18_perc_theta_0001"),
        BoundaryValue::scalar(0.0),
    );
    surface.state_surface.insert(
        BoundarySymbol::from("wb18_perc_fc_0001"),
        BoundaryValue::scalar(0.0),
    );
    surface.state_surface.insert(
        BoundarySymbol::from("thetfc_0001"),
        BoundaryValue::scalar(0.0),
    );
    surface.state_surface.insert(
        BoundarySymbol::from("thetdr_0001"),
        BoundaryValue::scalar(0.0),
    );
    surface.state_surface.insert(
        BoundarySymbol::from("wb18_perc_theta_0002"),
        BoundaryValue::scalar(0.0),
    );
    surface.state_surface.insert(
        BoundarySymbol::from("wb18_perc_fc_0002"),
        BoundaryValue::scalar(0.0),
    );
    surface.state_surface.insert(
        BoundarySymbol::from("thetfc_0002"),
        BoundaryValue::scalar(0.0),
    );
    surface.state_surface.insert(
        BoundarySymbol::from("thetdr_0002"),
        BoundaryValue::scalar(0.0),
    );
    surface.state_surface.insert(
        BoundarySymbol::from("irrigation.depletion.period_0001.element_id"),
        BoundaryValue::scalar(1.0),
    );
    surface.state_surface.insert(
        BoundarySymbol::from("irrigation.depletion.period_0001.start_doy"),
        BoundaryValue::scalar(100.0),
    );
    surface.state_surface.insert(
        BoundarySymbol::from("irrigation.depletion.period_0001.start_year"),
        BoundaryValue::scalar(1.0),
    );
    surface.state_surface.insert(
        BoundarySymbol::from("irrigation.depletion.period_0001.end_doy"),
        BoundaryValue::scalar(200.0),
    );
    surface.state_surface.insert(
        BoundarySymbol::from("irrigation.depletion.period_0001.end_year"),
        BoundaryValue::scalar(1.0),
    );
    surface.state_surface.insert(
        BoundarySymbol::from("irrigation.depletion.period_0001.depletion_trigger_ratio"),
        BoundaryValue::scalar(0.95),
    );
    surface.state_surface.insert(
        BoundarySymbol::from("irrigation.depletion.period_0001.sprinkler_depth_ratio"),
        BoundaryValue::scalar(0.5),
    );
    surface.state_surface.insert(
        BoundarySymbol::from("irrigation.depletion.period_0001.sprinkler_rate_m_per_s"),
        BoundaryValue::scalar(0.15),
    );
    surface.state_surface.insert(
        BoundarySymbol::from("irrigation.depletion.period_0001.sprinkler_nozzle_factor"),
        BoundaryValue::scalar(1.0),
    );
    surface.state_surface.insert(
        BoundarySymbol::from("wb12_rainfall_input"),
        BoundaryValue::scalar(3.15),
    );
    surface.state_surface.insert(
        BoundarySymbol::from("wb12_runoff_closure_tolerance"),
        BoundaryValue::scalar(10.0),
    );
    surface.state_surface.insert(
        BoundarySymbol::from("wb12_storage_closure_tolerance"),
        BoundaryValue::scalar(10.0),
    );

    let report = run_surface(surface);
    assert!(
        report.scheduler_report.is_success(),
        "scheduler halted at {:?}",
        report.scheduler_report.halted_phase
    );

    let irrigation_depth = report
        .writeback_surface
        .flux_surface
        .get(&BoundarySymbol::from("Irr"))
        .expect("Irr should exist for active depletion irrigation")
        .as_f64();
    assert!((irrigation_depth - 0.15).abs() < 1.0e-12);
    let schedule_source = report
        .writeback_surface
        .state_surface
        .get(&BoundarySymbol::from("irrigation.runtime_schedule_source"))
        .expect("runtime schedule source should be published")
        .as_f64();
    assert!((schedule_source - 1.0).abs() < 1.0e-12);
}

#[test]
fn irrig10_contract_vector_missing_schedule_day_symbol_is_typed() {
    let mut surface = seeded_irrig10_base_surface();
    surface.state_surface.remove(&BoundarySymbol::from("day"));
    surface.state_surface.insert(
        BoundarySymbol::from("irrigation.fixeddate.enabled"),
        BoundaryValue::scalar(1.0),
    );
    surface.state_surface.insert(
        BoundarySymbol::from("irrigation.fixeddate.event_count"),
        BoundaryValue::scalar(1.0),
    );
    surface.state_surface.insert(
        BoundarySymbol::from("irrigation.fixeddate.system_type"),
        BoundaryValue::scalar(1.0),
    );
    surface.state_surface.insert(
        BoundarySymbol::from("irrigation.fixeddate.event_0001.ofe_id"),
        BoundaryValue::scalar(1.0),
    );
    surface.state_surface.insert(
        BoundarySymbol::from("irrigation.fixeddate.event_0001.day"),
        BoundaryValue::scalar(120.0),
    );
    surface.state_surface.insert(
        BoundarySymbol::from("irrigation.fixeddate.event_0001.year"),
        BoundaryValue::scalar(1.0),
    );
    surface.state_surface.insert(
        BoundarySymbol::from("irrigation.fixeddate.event_0001.schedule_termination_flag"),
        BoundaryValue::scalar(0.0),
    );
    surface.state_surface.insert(
        BoundarySymbol::from("irrigation.fixeddate.event_0001.sprinkler_depth_m"),
        BoundaryValue::scalar(0.2),
    );
    surface.state_surface.insert(
        BoundarySymbol::from("irrigation.fixeddate.event_0001.sprinkler_rate_m_per_s"),
        BoundaryValue::scalar(0.1),
    );
    surface.state_surface.insert(
        BoundarySymbol::from("irrigation.fixeddate.event_0001.sprinkler_nozzle_factor"),
        BoundaryValue::scalar(1.0),
    );

    let report = run_surface(surface);

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
