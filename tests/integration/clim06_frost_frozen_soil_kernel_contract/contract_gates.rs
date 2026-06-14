use super::support::*;

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
    let _ = require_state_scalar(&report, "frost.hourly.surface_temp_c_0001");
    let _ = require_state_scalar(&report, "frost.hourly.snow_depth_m_0001");
    let _ = require_state_scalar(&report, "frost.hourly.residue_depth_m_0001");
    let _ = require_state_scalar(&report, "frost.hourly.tilled_frozen_depth_m_0001");
    let _ = require_state_scalar(&report, "frost.hourly.untilled_frozen_depth_m_0001");
}

#[test]
fn fdhp01_dh_frozen_path_conductivity_uses_pinned_legacy_constants() {
    let mut surface = seeded_clim06_surface(true);
    insert_state_scalar(&mut surface, "frost.options.ksoilf", 10.0);
    insert_state_scalar(&mut surface, "wb18_perc_ssc_0001", 9.0e-6);
    insert_state_scalar(&mut surface, "wb18_perc_ssc_0002", 1.0e-7);

    let report = execute_clim06_surface(surface);
    assert!(report.scheduler_report.is_success());

    assert_close(
        require_state_scalar(&report, "frost.runtime_kftill_w_m_k"),
        1.75,
        "Dh source audit: frozen tilled conductivity is the pinned frostn kftill constant, not a ksoilf/soil-property function",
    );
    assert_close(
        require_state_scalar(&report, "frost.runtime_kfutil_w_m_k"),
        2.1,
        "Dh source audit: frozen untilled conductivity is the pinned frostn kfutil constant, not a ksoilf/soil-property function",
    );
}

#[test]
fn fdhp01_dj_snow_active_cold_hour_uses_tmpadj_surface_temperature_not_raw_air() {
    let mut surface = seeded_clim06_surface(true);
    insert_state_scalar(&mut surface, "snow.runtime_depth_m", 0.25);
    insert_state_scalar(&mut surface, "snow.runtime_density_kg_m3", 300.0);
    set_winter_hourly_forcing(&mut surface, -12.0, 0.0, 1.0);

    let response = execute_clim06_runoff_phase(&surface);
    assert!(
        response.status.ok_flag(),
        "Dj cold snow vector should execute successfully; status={:?}",
        response.status
    );

    let surface_temp_c =
        require_response_state_update(&response, "frost.hourly.surface_temp_c_0001");
    assert!(
        surface_temp_c <= 0.0,
        "snow-covered below-freezing surface temperature must remain non-positive; surface_temp_c={surface_temp_c}"
    );
    assert!(
        (surface_temp_c - -12.0).abs() > 0.1,
        "Dj must synthesize legacy tmpadj surtmp instead of passing raw hourly air temperature through; surface_temp_c={surface_temp_c}"
    );
}

#[test]
fn fdhp01_dj_positive_snow_covered_tmpadj_surface_temperature_caps_at_zero() {
    let mut surface = seeded_clim06_surface(true);
    insert_state_scalar(&mut surface, "snow.runtime_depth_m", 0.25);
    insert_state_scalar(&mut surface, "snow.runtime_density_kg_m3", 300.0);
    set_winter_hourly_forcing(&mut surface, 6.0, 3.0, 0.0);

    let response = execute_clim06_runoff_phase(&surface);
    assert!(
        response.status.ok_flag(),
        "Dj positive snow vector should execute successfully; status={:?}",
        response.status
    );

    assert_close(
        require_response_state_update(&response, "frost.hourly.surface_temp_c_0001"),
        0.0,
        "legacy tmpadj positive-under-snow cap must survive the full surface-temperature port",
    );
}

#[test]
fn fdhp01_dj_active_frost_requires_hourly_radiation_for_tmpadj_surface_temperature() {
    let mut surface = seeded_clim06_surface(true);
    surface
        .state_surface
        .remove(&BoundarySymbol::from("winter.hourly.rad_mj_m2_0001"));

    let response = execute_clim06_runoff_phase(&surface);
    assert!(
        !response.status.ok_flag(),
        "active frost must fail closed when tmpadj hourly radiation is absent"
    );
    assert_eq!(response.status.message_id(), "HKERNEL-WB14-RUNOFF-E-001");
    assert_eq!(
        response.status.boundary_class(),
        BoundaryClass::MissingRequiredInput
    );
}
