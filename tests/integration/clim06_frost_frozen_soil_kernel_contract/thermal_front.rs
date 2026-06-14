use super::support::*;

#[test]
fn fdhp01_c2_mlttp_top_thaw_sets_sandwich_geometry_and_fgthwd() {
    let mut surface = seeded_clim06_surface(true);
    seed_c2_full_top_layer_frost(&mut surface);
    override_monthly_temperatures(&mut surface, -20.0);
    insert_state_scalar(&mut surface, "tmax", 0.10);
    insert_state_scalar(&mut surface, "tmin", 0.10);
    set_winter_hourly_forcing(&mut surface, 6.0, 3.0, 0.0);

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
    set_winter_hourly_forcing(&mut surface, 35.0, 8.0, 0.0);
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
        set_winter_hourly_forcing(&mut surface, -18.0, 0.0, 1.0);
        let freeze = execute_clim06_runoff_phase(&surface);
        apply_response_state_updates(&mut surface, &freeze);

        insert_state_scalar(&mut surface, "tmax", 18.0);
        insert_state_scalar(&mut surface, "tmin", 12.0);
        set_winter_hourly_forcing(&mut surface, 12.0, 3.0, 0.0);
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
fn fdhp01_dc1_lower_front_heat_uses_seasonal_tmpbl_zero_gate() {
    let mut surface = seeded_clim06_surface(true);
    seed_c2_full_top_layer_frost(&mut surface);
    override_monthly_temperatures(&mut surface, -20.0);
    insert_state_scalar(&mut surface, "day", 32.0);
    insert_state_scalar(&mut surface, "mon", 2.0);
    insert_state_scalar(&mut surface, "tmax", 0.0);
    insert_state_scalar(&mut surface, "tmin", 0.0);
    set_neutral_tmpadj_hourly_forcing(&mut surface);

    let response = execute_clim06_runoff_phase(&surface);
    assert!(
        response.status.ok_flag(),
        "cold seasonal tmpbl vector should execute successfully; status={:?}",
        response.status
    );
    assert_close(
        require_response_state_update(&response, "frost.hourly.quf_w_m2_0001"),
        0.0,
        "tmpbl <= 0 must zero-gate lower-front qdry",
    );
    assert_close(
        require_response_state_update(&response, "frost.hourly.frzflg_0001"),
        0.0,
        "neutral surface heat plus zero qdry must be balanced",
    );
}

#[test]
fn fdhp01_de_lower_front_heat_uses_legacy_dry_fallback_only_when_no_positive_terms() {
    let mut surface = seeded_clim06_surface(true);
    seed_c2_full_top_layer_frost(&mut surface);
    insert_state_scalar(&mut surface, "wb19_bulk_density_kg_m3_0001", 700.0);
    insert_state_scalar(&mut surface, "wb19_bulk_density_kg_m3_0002", 700.0);
    override_monthly_temperatures(&mut surface, 15.0);
    insert_state_scalar(&mut surface, "day", 32.0);
    insert_state_scalar(&mut surface, "mon", 2.0);
    insert_state_scalar(&mut surface, "tmax", 0.0);
    insert_state_scalar(&mut surface, "tmin", 0.0);
    set_neutral_tmpadj_hourly_forcing(&mut surface);

    let response = execute_clim06_runoff_phase(&surface);
    assert!(
        response.status.ok_flag(),
        "warm seasonal tmpbl vector should execute successfully; status={:?}",
        response.status
    );
    assert_close(
        require_response_state_update(&response, "frost.hourly.quf_w_m2_0001"),
        3.0,
        "constant 15 degC monthly curve with no positive conductivity terms must retain legacy kufz=0.2 fallback",
    );
    assert_close(
        require_response_state_update(&response, "frost.hourly.frzflg_0001"),
        4.0,
        "neutral surface heat plus positive qdry over existing frost must dispatch bottom thaw",
    );
}

#[test]
fn fdhp01_de_lower_front_heat_uses_legacy_harmonic_unfrozen_conductivity() {
    let mut surface = seeded_clim06_surface(true);
    seed_de_full_meter_lower_front_profile(&mut surface, true);
    override_monthly_temperatures(&mut surface, 15.0);
    insert_state_scalar(&mut surface, "day", 32.0);
    insert_state_scalar(&mut surface, "mon", 2.0);
    insert_state_scalar(&mut surface, "tmax", 0.0);
    insert_state_scalar(&mut surface, "tmin", 0.0);
    set_neutral_tmpadj_hourly_forcing(&mut surface);

    let response = execute_clim06_runoff_phase(&surface);
    assert!(
        response.status.ok_flag(),
        "moist harmonic lower-front vector should execute successfully; status={:?}",
        response.status
    );
    let quf_w_m2 = require_response_state_update(&response, "frost.hourly.quf_w_m2_0001");
    let published_kufz_w_m_k = quf_w_m2 / 15.0;
    let theta = 0.25_f64;
    let expected_kufz_w_m_k =
        (0.5096 + 7.4493 * theta - 8.7484 * theta.powi(2)) * (0.001_413_9 * 1_300.0 - 1.0588);
    assert!(
        (published_kufz_w_m_k - expected_kufz_w_m_k).abs() <= 1.0e-6,
        "Quf must use the legacy frostn.for:430-458 harmonic conductivity path; actual_k={published_kufz_w_m_k}, expected_k={expected_kufz_w_m_k}"
    );
    assert!(
        published_kufz_w_m_k > 1.0 && published_kufz_w_m_k < 1.5,
        "moist De fixture should be in the expected legacy polynomial conductivity range; k={published_kufz_w_m_k}"
    );
    assert_close(
        require_response_state_update(&response, "frost.hourly.frzflg_0001"),
        4.0,
        "neutral surface heat plus positive harmonic qdry over existing frost must dispatch bottom thaw",
    );
    let final_frdp_m = require_response_state_update(&response, "frost.runtime_frdp_m");
    assert!(
        final_frdp_m < 0.2,
        "harmonic lower-front heat must spend bottom-thaw energy against the frozen front; frdp={final_frdp_m}"
    );
}

#[test]
fn fdhp01_de_lower_front_heat_suppresses_marginal_autumn_freeze_onset() {
    let mut surface = seeded_clim06_surface(true);
    seed_de_full_meter_lower_front_profile(&mut surface, false);
    override_monthly_temperatures(&mut surface, 15.0);
    insert_state_scalar(&mut surface, "day", 305.0);
    insert_state_scalar(&mut surface, "mon", 11.0);
    insert_state_scalar(&mut surface, "tmax", -0.01);
    insert_state_scalar(&mut surface, "tmin", -0.01);

    let response = execute_clim06_runoff_phase(&surface);
    assert!(
        response.status.ok_flag(),
        "marginal autumn onset vector should execute successfully; status={:?}",
        response.status
    );
    assert!(
        require_response_state_update(&response, "frost.hourly.quf_w_m2_0001") > 15.0,
        "moist one-meter lower-front path should materially exceed the retired 0.2 fallback"
    );
    assert_close(
        require_response_state_update(&response, "frost.hourly.frzflg_0001"),
        0.0,
        "warm lower-front heat must offset a marginal cold surface hour before frost onset",
    );
    assert_close(
        require_response_state_update(&response, "frost.runtime_frdp_m"),
        0.0,
        "marginal autumn vector must not create scalar frost depth when net heat flow is non-freezing",
    );
}

#[test]
fn fdhp01_dc1_top_thaw_recomputes_resistance_within_hour() {
    let mut surface = seeded_clim06_surface(true);
    seed_c2_full_top_layer_frost(&mut surface);
    override_monthly_temperatures(&mut surface, -20.0);
    insert_state_scalar(&mut surface, "day", 32.0);
    insert_state_scalar(&mut surface, "mon", 2.0);
    insert_state_scalar(&mut surface, "tmax", -20.0);
    insert_state_scalar(&mut surface, "tmin", -20.0);
    set_neutral_tmpadj_hourly_forcing(&mut surface);
    for hour in 1..=1 {
        insert_state_scalar(
            &mut surface,
            &format!("winter.hourly.air_temp_c_{hour:04}"),
            20.0,
        );
        insert_state_scalar(
            &mut surface,
            &format!("winter.hourly.rad_mj_m2_{hour:04}"),
            3.0,
        );
        insert_state_scalar(
            &mut surface,
            &format!("winter.hourly.cloud_fraction_{hour:04}"),
            0.0,
        );
    }

    let response = execute_clim06_runoff_phase(&surface);
    assert!(
        response.status.ok_flag(),
        "top-thaw resistance-feedback vector should execute successfully; status={:?}",
        response.status
    );
    assert_close(
        require_response_state_update(&response, "frost.hourly.frzflg_0001"),
        3.0,
        "positive surface heat over existing frost must dispatch top thaw",
    );
    let thdp = require_response_state_update(&response, "frost.runtime_thdp_m");
    assert!(
        thdp > CLIM06_TEST_TOLERANCE && thdp <= 0.060,
        "top thaw must recompute resistance within the hour instead of spending start-hour flux across the layer; thdp={thdp}"
    );
}

#[test]
fn fdhp01_dc1_persisted_fine_theta_roundoff_canonicalizes_at_lower_bound() {
    let mut surface = seeded_clim06_surface(true);
    seed_c2_full_top_layer_frost(&mut surface);
    override_monthly_temperatures(&mut surface, -20.0);
    insert_state_scalar(&mut surface, "day", 32.0);
    insert_state_scalar(&mut surface, "mon", 2.0);
    insert_state_scalar(&mut surface, "tmax", -5.0);
    insert_state_scalar(&mut surface, "tmin", -5.0);
    insert_state_scalar(&mut surface, "thetdr_0002", 0.175);
    insert_state_scalar(
        &mut surface,
        &fine_frost_symbol("frost.runtime_slsw_theta", 2, 6),
        0.174_999_999_971_669_4,
    );

    let response = execute_clim06_runoff_phase(&surface);
    assert!(
        response.status.ok_flag(),
        "p35-class lower-bound theta roundoff should canonicalize, status={:?}",
        response.status
    );
    assert_close(
        require_response_state_update(&response, "frost.runtime_slsw_theta_0002_0006"),
        0.175,
        "sub-residual lower-bound theta roundoff must publish at the residual bound",
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
    set_winter_hourly_forcing(&mut surface, -30.0, 0.0, 1.0);
    configure_fdhp01_deep_profile(&mut surface);
    override_monthly_temperatures(&mut surface, -20.0);

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
    set_winter_hourly_forcing(&mut surface, 8.0, 3.0, 0.0);
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
