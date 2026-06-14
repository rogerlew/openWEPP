use super::support::*;

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
    set_neutral_tmpadj_hourly_forcing(&mut mild);

    let mut severe = seeded_clim06_surface(true);
    severe
        .state_surface
        .insert(BoundarySymbol::from("tmax"), BoundaryValue::scalar(-2.0));
    severe
        .state_surface
        .insert(BoundarySymbol::from("tmin"), BoundaryValue::scalar(-12.0));
    set_winter_hourly_forcing(&mut severe, -18.0, 0.0, 1.0);

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
        "frost.hourly.surface_temp_c_0001",
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
