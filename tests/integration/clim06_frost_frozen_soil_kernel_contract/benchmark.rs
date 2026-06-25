use super::support::*;

// Source-pinned constants from
// crates/openwepp-hillslope-orchestrator/src/hydrology/03_kernel_support_00_support_helpers.rs.
const FROST_RUNTIME_KRES_BASE_W_M_K: f64 = 0.05;
const FROST_RUNTIME_LATENT_HEAT_WATER_J_M3: f64 = 3.35e8;
const FROST_RUNTIME_KFTILL_W_M_K: f64 = 1.75;
const FROST_RUNTIME_KFUTIL_W_M_K: f64 = 2.1;
const SHALLOW_FRONT_MINIMUM_CONDUCTION_PATH_M: f64 = 0.005;

fn tmpadj_snow_conductivity_w_m_k(snow_density_kg_m3: f64) -> f64 {
    let density_g_cm3 = snow_density_kg_m3 / 1000.0;
    if snow_density_kg_m3 < 156.0 {
        0.023 + 0.234 * density_g_cm3
    } else {
        0.138 - 1.01 * density_g_cm3 + 3.233 * density_g_cm3.powi(2)
    }
}

fn expected_series_resistance_m2_c_w(
    frozen_depth_m: f64,
    snow_depth_m: f64,
    snow_density_kg_m3: f64,
    residue_depth_m: f64,
) -> f64 {
    let mut resistance_m2_c_w = 0.0;
    if snow_depth_m > 0.0 {
        resistance_m2_c_w += snow_depth_m / tmpadj_snow_conductivity_w_m_k(snow_density_kg_m3);
    }
    if residue_depth_m > 0.0 {
        resistance_m2_c_w += residue_depth_m / FROST_RUNTIME_KRES_BASE_W_M_K;
    }

    let effective_depth_m = frozen_depth_m.max(SHALLOW_FRONT_MINIMUM_CONDUCTION_PATH_M);
    let tilled_depth_m = effective_depth_m.min(0.20);
    let untilled_depth_m = (effective_depth_m - tilled_depth_m).max(0.0);
    resistance_m2_c_w += tilled_depth_m / FROST_RUNTIME_KFTILL_W_M_K;
    resistance_m2_c_w += untilled_depth_m / FROST_RUNTIME_KFUTIL_W_M_K;
    resistance_m2_c_w
}

fn published_resistance_m2_c_w(response: &KernelRunResponse, hour: usize) -> f64 {
    let surface_temp_c =
        require_response_state_update(response, &format!("frost.hourly.surface_temp_c_{hour:04}"));
    let qsrf_w_m2 =
        require_response_state_update(response, &format!("frost.hourly.qsrf_w_m2_{hour:04}"));
    assert!(
        surface_temp_c < 0.0 && qsrf_w_m2 > 0.0,
        "freezing-hour resistance reconstruction requires negative surface temperature and positive Qsrf; surface_temp_c={surface_temp_c}, qsrf_w_m2={qsrf_w_m2}"
    );
    -surface_temp_c / qsrf_w_m2
}

fn daily_positive_qsrf_energy_j_m2(response: &KernelRunResponse) -> f64 {
    (1..=24)
        .map(|hour| {
            require_response_state_update(response, &format!("frost.hourly.qsrf_w_m2_{hour:04}"))
                .max(0.0)
                * 3600.0
        })
        .sum()
}

fn fine_ice_sum_m(response: &KernelRunResponse) -> f64 {
    response_fine_layer_sum(response, "frost.runtime_slsic_m", 1, 10)
        + response_fine_layer_sum(response, "frost.runtime_slsic_m", 2, 10)
}

fn stefan_latent_only_freezing_front_bound_m(
    initial_front_m: f64,
    conductivity_w_m_k: f64,
    temperature_drop_c: f64,
    elapsed_s: f64,
    liquid_theta: f64,
) -> f64 {
    let latent_per_depth_j_m3 = FROST_RUNTIME_LATENT_HEAT_WATER_J_M3 * liquid_theta;
    (initial_front_m.powi(2)
        + (2.0 * conductivity_w_m_k * temperature_drop_c * elapsed_s / latent_per_depth_j_m3))
        .sqrt()
}

#[test]
fn snowfrost_b_surface_series_resistance_matches_independent_formula() {
    let snow_depth_m = 0.20;
    let snow_density_kg_m3 = 300.0;
    let residue_depth_m = 0.03;
    let initial_frozen_depth_m = 0.0004;

    let mut surface = seeded_clim06_surface(true);
    seed_db_thin_front_frost(&mut surface);
    override_monthly_temperatures(&mut surface, -20.0);
    insert_state_scalar(&mut surface, "tmax", -12.0);
    insert_state_scalar(&mut surface, "tmin", -12.0);
    insert_state_scalar(&mut surface, "snow.runtime_depth_m", snow_depth_m);
    insert_state_scalar(
        &mut surface,
        "snow.runtime_density_kg_m3",
        snow_density_kg_m3,
    );
    insert_state_scalar(
        &mut surface,
        "frost.runtime_residue_depth_m",
        residue_depth_m,
    );
    set_winter_hourly_forcing(&mut surface, -12.0, 0.0, 1.0);

    let response = execute_clim06_runoff_phase(&surface);
    assert!(
        response.status.ok_flag(),
        "snow/residue heat-flow benchmark must execute successfully; status={:?}",
        response.status
    );

    let expected_resistance_m2_c_w = expected_series_resistance_m2_c_w(
        initial_frozen_depth_m,
        snow_depth_m,
        snow_density_kg_m3,
        residue_depth_m,
    );
    let actual_resistance_m2_c_w = published_resistance_m2_c_w(&response, 1);
    assert!(
        (actual_resistance_m2_c_w - expected_resistance_m2_c_w).abs() <= 1.0e-6,
        "published Qsrf must reconstruct the independent snow + residue + frozen-soil series resistance; actual={actual_resistance_m2_c_w}, expected={expected_resistance_m2_c_w}"
    );

    assert_close(
        require_response_state_update(&response, "frost.hourly.snow_depth_m_0001"),
        snow_depth_m,
        "benchmark must bind the conductive snow depth used by the resistance reconstruction",
    );
    assert_close(
        require_response_state_update(&response, "frost.hourly.residue_depth_m_0001"),
        residue_depth_m,
        "benchmark must bind the conductive residue depth used by the resistance reconstruction",
    );
}

#[test]
fn snowfrost_b_one_dimensional_freezing_front_stays_within_stefan_bound() {
    let mut surface = seeded_clim06_surface(true);
    seed_db_thin_front_frost(&mut surface);
    override_monthly_temperatures(&mut surface, -20.0);
    insert_state_scalar(&mut surface, "tmax", -18.0);
    insert_state_scalar(&mut surface, "tmin", -18.0);
    set_winter_hourly_forcing(&mut surface, -18.0, 0.0, 1.0);

    let response = execute_clim06_runoff_phase(&surface);
    assert!(
        response.status.ok_flag(),
        "Stefan-style one-dimensional freezing benchmark must execute successfully; status={:?}",
        response.status
    );

    let final_front_m = require_response_state_update(&response, "frost.runtime_frdp_m");
    let initial_effective_front_m = SHALLOW_FRONT_MINIMUM_CONDUCTION_PATH_M;
    let stefan_bound_m = stefan_latent_only_freezing_front_bound_m(
        initial_effective_front_m,
        FROST_RUNTIME_KFTILL_W_M_K,
        18.0,
        24.0 * 3600.0,
        0.2,
    );
    assert!(
        final_front_m > 0.0004 + CLIM06_TEST_TOLERANCE,
        "cold one-dimensional fixture must advance the freezing front; final_front_m={final_front_m}"
    );
    assert!(
        final_front_m <= stefan_bound_m + CLIM06_TEST_TOLERANCE,
        "legacy fine-layer front advance must stay within the latent-only Stefan upper bound; final_front_m={final_front_m}, stefan_bound_m={stefan_bound_m}"
    );
}

#[test]
fn snowfrost_b_snowpack_insulation_increases_resistance_and_reduces_freezing_flux() {
    let mut bare_surface = seeded_clim06_surface(true);
    seed_db_thin_front_frost(&mut bare_surface);
    override_monthly_temperatures(&mut bare_surface, -20.0);
    insert_state_scalar(&mut bare_surface, "tmax", -12.0);
    insert_state_scalar(&mut bare_surface, "tmin", -12.0);
    set_winter_hourly_forcing(&mut bare_surface, -12.0, 0.0, 1.0);

    let mut snow_surface = bare_surface.clone();
    insert_state_scalar(&mut snow_surface, "snow.runtime_depth_m", 0.25);
    insert_state_scalar(&mut snow_surface, "snow.runtime_density_kg_m3", 300.0);

    let bare = execute_clim06_runoff_phase(&bare_surface);
    let snow = execute_clim06_runoff_phase(&snow_surface);
    assert!(
        bare.status.ok_flag(),
        "bare benchmark failed: {:?}",
        bare.status
    );
    assert!(
        snow.status.ok_flag(),
        "snow benchmark failed: {:?}",
        snow.status
    );

    let bare_resistance_m2_c_w = published_resistance_m2_c_w(&bare, 1);
    let snow_resistance_m2_c_w = published_resistance_m2_c_w(&snow, 1);
    let expected_bare_resistance_m2_c_w = expected_series_resistance_m2_c_w(0.0004, 0.0, 0.0, 0.0);
    let expected_snow_resistance_m2_c_w =
        expected_series_resistance_m2_c_w(0.0004, 0.25, 300.0, 0.0);
    assert!(
        (bare_resistance_m2_c_w - expected_bare_resistance_m2_c_w).abs() <= 1.0e-6,
        "bare freezing path must reconstruct the shallow-front frozen-soil resistance; actual={bare_resistance_m2_c_w}, expected={expected_bare_resistance_m2_c_w}"
    );
    assert!(
        (snow_resistance_m2_c_w - expected_snow_resistance_m2_c_w).abs() <= 1.0e-6,
        "snow-covered freezing path must include snow conductivity in series; actual={snow_resistance_m2_c_w}, expected={expected_snow_resistance_m2_c_w}"
    );

    let bare_qsrf_w_m2 = require_response_state_update(&bare, "frost.hourly.qsrf_w_m2_0001");
    let snow_qsrf_w_m2 = require_response_state_update(&snow, "frost.hourly.qsrf_w_m2_0001");
    assert!(
        snow_resistance_m2_c_w > bare_resistance_m2_c_w * 100.0,
        "snowpack must materially raise conductive resistance; bare={bare_resistance_m2_c_w}, snow={snow_resistance_m2_c_w}"
    );
    assert!(
        snow_qsrf_w_m2 < bare_qsrf_w_m2 * 0.25,
        "snowpack insulation must suppress first-hour freezing flux in this controlled cold fixture; bare_qsrf={bare_qsrf_w_m2}, snow_qsrf={snow_qsrf_w_m2}"
    );
}

#[test]
fn snowfrost_b_lower_front_heat_offsets_marginal_freeze_without_migration_heat() {
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
        "lower-front no-migration benchmark must execute successfully; status={:?}",
        response.status
    );

    let qsrf_w_m2 = require_response_state_update(&response, "frost.hourly.qsrf_w_m2_0001");
    let quf_w_m2 = require_response_state_update(&response, "frost.hourly.quf_w_m2_0001");
    assert!(
        quf_w_m2 > qsrf_w_m2,
        "legacy dry lower-front heat must offset marginal surface freezing without a migration term; quf={quf_w_m2}, qsrf={qsrf_w_m2}"
    );
    assert_close(
        require_response_state_update(&response, "frost.hourly.frzflg_0001"),
        0.0,
        "net lower-front heat offset must leave the dispatch balanced",
    );
    assert_close(
        require_response_state_update(&response, "frost.runtime_frdp_m"),
        0.0,
        "marginal offset benchmark must not create frost depth",
    );
}

#[test]
fn snowfrost_b_fine_layer_freezing_is_bounded_by_latent_heat_energy() {
    let mut surface = seeded_clim06_surface(true);
    seed_increment_a_shadow_fine_state(&mut surface, 0.0);
    override_monthly_temperatures(&mut surface, -20.0);
    insert_state_scalar(&mut surface, "tmax", -12.0);
    insert_state_scalar(&mut surface, "tmin", -24.0);
    set_winter_hourly_forcing(&mut surface, -18.0, 0.0, 1.0);

    let response = execute_clim06_runoff_phase(&surface);
    assert!(
        response.status.ok_flag(),
        "fine-layer latent-energy benchmark must execute successfully; status={:?}",
        response.status
    );

    let initial_fine_ice_m = 0.012;
    let final_fine_ice_m = fine_ice_sum_m(&response);
    let fine_ice_delta_m = final_fine_ice_m - initial_fine_ice_m;
    let latent_energy_delta_j_m2 = fine_ice_delta_m * FROST_RUNTIME_LATENT_HEAT_WATER_J_M3;
    let positive_surface_energy_j_m2 = daily_positive_qsrf_energy_j_m2(&response);

    assert!(
        fine_ice_delta_m > CLIM06_TEST_TOLERANCE,
        "cold no-Qwet fixture must advance fine-layer ice before later physics tuning; delta_m={fine_ice_delta_m}"
    );
    assert!(
        latent_energy_delta_j_m2 <= positive_surface_energy_j_m2 + 1.0e-3,
        "fine-layer ice growth must be bounded by independently integrated positive surface freezing energy; latent={latent_energy_delta_j_m2}, available={positive_surface_energy_j_m2}"
    );
    assert!(
        require_response_state_update(&response, "frost.runtime_slfsd_m_0001_0004")
            > CLIM06_TEST_TOLERANCE,
        "energy benchmark must mutate the next fine-layer front state, not only aggregate storage",
    );
}
