use super::support::*;

#[test]
fn fdhp01_fine_sublayer_frwatc_round_trip_conserves_mass() {
    let mut surface = seeded_clim06_surface(true);
    seed_increment_a_shadow_fine_state(&mut surface, 0.0);
    override_monthly_temperatures(&mut surface, -20.0);
    insert_state_scalar(&mut surface, "tmax", 0.0);
    insert_state_scalar(&mut surface, "tmin", 0.0);
    set_neutral_tmpadj_hourly_forcing(&mut surface);

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
    seed_c2_full_top_layer_frost(&mut sandwich_cold);
    insert_state_scalar(&mut sandwich_cold, "frost.runtime_fgfrst_0001_0001", 3.0);
    insert_state_scalar(&mut sandwich_cold, "frost.runtime_slfsd_m_0001_0001", 0.005);
    insert_state_scalar(&mut sandwich_cold, "frost.runtime_slsic_m_0001_0001", 0.001);
    insert_state_scalar(&mut sandwich_cold, "frost.runtime_fgfrst_0001_0002", 0.0);
    insert_state_scalar(&mut sandwich_cold, "frost.runtime_slfsd_m_0001_0002", 0.0);
    insert_state_scalar(&mut sandwich_cold, "frost.runtime_slsic_m_0001_0002", 0.0);
    set_winter_hourly_forcing(&mut sandwich_cold, -0.5, 0.0, 1.0);
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
    set_neutral_tmpadj_hourly_forcing(&mut balanced);
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
    set_winter_hourly_forcing(&mut surface, -18.0, 0.0, 1.0);

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
    set_winter_hourly_forcing(&mut surface, -40.0, 0.0, 1.0);

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
        hour_2_depth_m - hour_1_depth_m <= 0.060,
        "front advance must remain bounded while tmpadj feedback recomputes surface temperature: h1={hour_1_depth_m}, h2={hour_2_depth_m}"
    );

    assert!(
        hour_2_depth_m <= 0.060,
        "tmpadj-coupled resistance feedback must keep the second-hour thin-front state inside the tilled layer; h2={hour_2_depth_m}"
    );
}

#[test]
fn fdhp01_dg_shallow_front_minimum_limits_surface_flux_without_residue() {
    let mut surface = seeded_clim06_surface(true);
    seed_db_thin_front_frost(&mut surface);
    insert_state_scalar(&mut surface, "frost.runtime_residue_depth_m", 0.0);
    insert_state_scalar(&mut surface, "tmax", -8.086);
    insert_state_scalar(&mut surface, "tmin", -8.086);

    let response = execute_clim06_runoff_phase(&surface);
    assert!(
        response.status.ok_flag(),
        "Dg shallow-front vector should execute successfully; status={:?}",
        response.status
    );

    let qsrf_w_m2 = require_response_state_update(&response, "frost.hourly.qsrf_w_m2_0001");
    let dpfsfl_flux_w_m2 = 8.086 / (0.005 / 1.75);
    assert!(
        qsrf_w_m2 <= dpfsfl_flux_w_m2 + CLIM06_TEST_TOLERANCE,
        "below-freezing shallow-front heat flow must use the legacy dpfsfl minimum path even without residue; qsrf={qsrf_w_m2}, limit={dpfsfl_flux_w_m2}"
    );
}

#[test]
fn fdhp01_dg_residue_depth_adds_surface_resistance() {
    let mut bare_surface = seeded_clim06_surface(true);
    seed_db_thin_front_frost(&mut bare_surface);
    insert_state_scalar(&mut bare_surface, "frost.runtime_residue_depth_m", 0.0);
    insert_state_scalar(&mut bare_surface, "tmax", -8.086);
    insert_state_scalar(&mut bare_surface, "tmin", -8.086);

    let mut residue_surface = bare_surface.clone();
    insert_state_scalar(&mut residue_surface, "frost.runtime_residue_depth_m", 0.023);

    let bare_response = execute_clim06_runoff_phase(&bare_surface);
    let residue_response = execute_clim06_runoff_phase(&residue_surface);
    assert!(bare_response.status.ok_flag());
    assert!(residue_response.status.ok_flag());

    let bare_qsrf_w_m2 =
        require_response_state_update(&bare_response, "frost.hourly.qsrf_w_m2_0001");
    let residue_qsrf_w_m2 =
        require_response_state_update(&residue_response, "frost.hourly.qsrf_w_m2_0001");
    let published_residue_depth_m =
        require_response_state_update(&residue_response, "frost.hourly.residue_depth_m_0001");
    assert_close(
        published_residue_depth_m,
        0.023,
        "hourly frost seam must publish the residue depth consumed by Qsrf",
    );
    assert!(
        residue_qsrf_w_m2 < bare_qsrf_w_m2 * 0.75,
        "residue resistance must materially reduce the legacy tmpadj-adjusted shallow-front surface flux; bare={bare_qsrf_w_m2}, residue={residue_qsrf_w_m2}"
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
    set_winter_hourly_forcing(&mut surface, -18.0, 0.0, 1.0);

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
    set_winter_hourly_forcing(&mut surface, -18.0, 0.0, 1.0);

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
    set_neutral_tmpadj_hourly_forcing(&mut surface);

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
