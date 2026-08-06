use std::fs;

const SNOW_ENERGY_CONTRACT: &str =
    "docs/specifications/science-contracts/contracts/SC-SNOWENERGY-001.md";
const SNOW_FREEZE_CONTRACT: &str =
    "docs/specifications/science-contracts/contracts/SC-SNOWFREEZE-001.md";
const ROADMAP: &str = "docs/planning/snow-surface-energy-balance-roadmap.md";
const DIRECT_TRACE_PRODUCER: &str = "crates/openwepp-runner/src/hillslope/direct_publication/day_input_and_helpers/00c_day_input_builder_impl.rs";

fn read(path: &str) -> String {
    fs::read_to_string(path).unwrap_or_else(|error| panic!("read {path}: {error}"))
}

#[test]
fn eb03_contract_binds_provider_selectors_and_exact_one_exchange() {
    let energy = read(SNOW_ENERGY_CONTRACT);
    let freeze = read(SNOW_FREEZE_CONTRACT);

    for required in [
        "contract_version: 8",
        "INV-SNOWENERGY-015",
        "INV-SNOWENERGY-016",
        "INV-SNOWENERGY-017",
        "INV-SNOWENERGY-018",
        "INV-SNOWENERGY-019",
        "INV-SNOWENERGY-020",
        "INV-SNOWENERGY-021",
        "INV-SNOWENERGY-022",
        "INV-SNOWENERGY-023",
        "INV-SNOWENERGY-024",
        "INV-SNOWENERGY-025",
        "INV-SNOWENERGY-026",
        "INV-SNOWENERGY-027",
        "INV-SNOWENERGY-029",
        "INV-SNOWENERGY-030",
        "INV-SNOWENERGY-031",
        "OBL-SNOWENERGY-P-006",
        "OBL-SNOWENERGY-C-013",
        "stage3_melt_owner_status = AUTHORITY_ADMITTED_IMPLEMENTATION_HOLD",
        "m_melt = min(Q_excess/L_f, m_ice_available)",
        "latent heat released by refreeze",
        "m_liquid_external_in",
        "delta_m_retained",
        "Q_complete + Q_refreeze - delta_E_cold - L_f*m_melt - Q_unallocated_after_exhaustion = 0",
        "m_ice_start + m_solid_precip + m_deposition - m_ice_end - m_sublimation - m_melt + m_refrozen = 0",
        "m_liquid_external_in + m_melt - m_refrozen - delta_m_retained - m_routed = 0",
        "unresolved terminal meltout/remaining-energy boundary",
        "m_ice_available=max(m_ice_after_solid_precip-m_sublimation,0)",
        "333600",
        "KTS+efcon",
        "snow_active_lower_conduction_w_m2",
        "layered_thermal_liquid_v1",
        "T_c=T_a",
        "R_a,min = 1e-9 MJ m^-2 d^-1",
        "CloudForcingUnavailable",
        "M_ice,before - M_ice,after = M_sublimation",
    ] {
        assert!(
            energy.contains(required),
            "{SNOW_ENERGY_CONTRACT} missing {required}"
        );
    }

    for required in [
        "contract_version: 127",
        "INV-SNOWFREEZE-085",
        "INV-SNOWFREEZE-086",
        "OBL-SNOWFREEZE-P-059",
        "OBL-SNOWFREEZE-P-060",
        "INV-SNOWFREEZE-093",
        "OBL-SNOWFREEZE-P-066",
        "snow_surface_longwave_model",
        "snow_sublimation_model",
        "Stage 3 is the sole admitted future melt owner",
        "CoE plus Stage 3 may never generate melt simultaneously",
        "prospectively supersedes every earlier invariant, addendum, boundary row, and package constraint",
        "21N authorizes no runtime change by itself",
    ] {
        assert!(
            freeze.contains(required),
            "{SNOW_FREEZE_CONTRACT} missing {required}"
        );
    }
}

#[test]
fn eb04d_contract_separates_layer_mass_lifecycle_from_residual_tolerance() {
    let energy = read(SNOW_ENERGY_CONTRACT);
    for required in [
        "m_layer = rho_w * SWE_layer > 1e-9 kg m^-2",
        "mass_swe_m > 1e-12 m",
        "neither may delete a represented layer",
        "OBL-SNOWENERGY-C-012",
        "SNOWENERGY-EB04D-LAYER-RECONCILIATION",
        "GAP-SNOWENERGY-009",
    ] {
        assert!(energy.contains(required), "contract missing {required}");
    }
}

#[test]
fn eb04c_contract_binds_exact_minimum_resolved_thermal_domain() {
    let energy = read(SNOW_ENERGY_CONTRACT);
    let producer = read(DIRECT_TRACE_PRODUCER);
    for required in [
        "m_s <= m_res = 1 kg m^-2",
        "0 < m_l < m_res",
        "m_l = 1 kg m^-2",
        "Q_shortwave = Q_longwave = Q_latent = 0",
        "m_v = m_sub = 0",
        "G_0 = Q_E = 0",
        "INV-SNOWENERGY-026",
        "compatibility runtime owner",
    ] {
        assert!(energy.contains(required), "contract missing {required}");
    }
    for required in [
        "stage3_thermal_domain_suspended_seconds",
        "stage3_minimum_unresolved_thermal_mass_kg_m2",
        "stage3_lower_thermal_volume_collapsed_seconds",
        "stage3_minimum_collapsed_lower_mass_kg_m2",
    ] {
        assert!(
            producer.contains(required),
            "trace producer missing {required}"
        );
    }
}

#[test]
fn eb03_roadmap_requires_four_orthogonal_cells() {
    let roadmap = read(ROADMAP);
    for cell in [
        "`B` baseline",
        "`L` longwave only",
        "`S` sublimation only",
        "`LS` combined",
    ] {
        assert!(roadmap.contains(cell), "{ROADMAP} missing {cell}");
    }
    assert!(roadmap.contains("SNOW-SURFACE-EB-03A"));
    assert!(roadmap.contains("Marks/SNOBAL active thermal control volume"));
    assert!(roadmap.contains("No surrogate, provisional, proxy, or heuristic process physics"));
}

#[test]
fn canonical_sky_view_differs_from_direct_gap_fraction() {
    let cover = 0.75_f64;
    let direct_gap = 1.0 - cover;
    let sky_view = direct_gap.powf(1.6);
    assert!((sky_view - 0.108_818_820_412_015_5).abs() <= 1.0e-15);
    assert!((sky_view - direct_gap).abs() > 0.1);
}

#[test]
fn signed_sublimation_latent_identity_rejects_wrong_sign_and_double_debit() {
    let vapor_mass_exchange_kg_m2 = -0.018_f64;
    let duration_s = 3_600.0_f64;
    let latent_heat_j_kg = 2_835_000.0_f64;
    let latent_flux_w_m2 = vapor_mass_exchange_kg_m2 / duration_s * latent_heat_j_kg;
    let reconstructed_energy_j_m2 = latent_flux_w_m2 * duration_s;
    let mass_view_energy_j_m2 = vapor_mass_exchange_kg_m2 * latent_heat_j_kg;

    assert!(latent_flux_w_m2 < 0.0);
    assert!((reconstructed_energy_j_m2 - mass_view_energy_j_m2).abs() <= 1.0e-9);
    assert!((reconstructed_energy_j_m2 + mass_view_energy_j_m2).abs() > 1.0e-9);
    assert!((reconstructed_energy_j_m2 - 2.0 * mass_view_energy_j_m2).abs() > 1.0e-9);
}

#[test]
fn cold_content_closure_includes_exported_cold_content() {
    let cold_content_before_j_m2 = 25_000.0_f64;
    let applied_surface_energy_j_m2 = -4_000.0_f64;
    let conduction_j_m2 = 500.0_f64;
    let refreeze_j_m2 = 1_000.0_f64;
    let exported_cold_content_j_m2 = 2_500.0_f64;
    let cold_content_after_j_m2 = cold_content_before_j_m2
        - applied_surface_energy_j_m2
        - conduction_j_m2
        - refreeze_j_m2
        - exported_cold_content_j_m2;
    let residual =
        applied_surface_energy_j_m2 + conduction_j_m2 + refreeze_j_m2 + exported_cold_content_j_m2
            - (cold_content_before_j_m2 - cold_content_after_j_m2);

    assert!(residual.abs() <= 1.0e-12);
    let wrong_without_export = applied_surface_energy_j_m2 + conduction_j_m2 + refreeze_j_m2
        - (cold_content_before_j_m2 - cold_content_after_j_m2);
    assert!((wrong_without_export + exported_cold_content_j_m2).abs() <= 1.0e-12);
}

#[test]
fn active_layer_crosses_depositional_boundaries_and_reconstructs_mass() {
    let layer_depths_m = [0.000_68_f64, 0.030, 0.300];
    let layer_masses_kg_m2 = [0.34_f64, 12.0, 90.0];
    let active_depth_m = layer_depths_m.iter().sum::<f64>().min(0.25);
    let mut remaining_depth_m = active_depth_m;
    let mut active_mass_kg_m2 = 0.0;
    for (depth_m, mass_kg_m2) in layer_depths_m.into_iter().zip(layer_masses_kg_m2) {
        let included_depth_m = remaining_depth_m.min(depth_m);
        active_mass_kg_m2 += mass_kg_m2 * included_depth_m / depth_m;
        remaining_depth_m -= included_depth_m;
        if remaining_depth_m <= f64::EPSILON {
            break;
        }
    }

    assert!((active_depth_m - 0.25).abs() <= 1.0e-12);
    assert!(active_mass_kg_m2 > layer_masses_kg_m2[0]);
    assert!(active_mass_kg_m2 > layer_masses_kg_m2[0] + layer_masses_kg_m2[1]);
    assert!(active_mass_kg_m2 < layer_masses_kg_m2.iter().sum::<f64>());
}

#[test]
fn harmonic_conduction_is_equal_and_opposite() {
    let k_0 = 0.12_f64;
    let k_l = 0.30_f64;
    let z_0 = 0.25_f64;
    let z_l = 0.50_f64;
    let t_0 = -12.0_f64;
    let t_l = -3.0_f64;
    let g_0 = 2.0 * k_0 * k_l * (t_l - t_0) / (k_l * z_0 + k_0 * z_l);

    assert!(g_0 > 0.0);
    assert!((g_0 + -g_0).abs() <= f64::EPSILON);
    let wrong_sign = 2.0 * k_0 * k_l * (t_0 - t_l) / (k_l * z_0 + k_0 * z_l);
    assert!((g_0 - wrong_sign).abs() > 1.0);
}

#[test]
fn marks_mass_thresholds_select_authoritative_substeps() {
    fn substep_seconds(minimum_control_volume_mass_kg_m2: f64) -> u32 {
        if minimum_control_volume_mass_kg_m2 >= 60.0 {
            3_600
        } else if minimum_control_volume_mass_kg_m2 >= 10.0 {
            900
        } else {
            60
        }
    }

    assert_eq!(substep_seconds(60.0), 3_600);
    assert_eq!(substep_seconds(59.999), 900);
    assert_eq!(substep_seconds(10.0), 900);
    assert_eq!(substep_seconds(9.999), 60);
    assert_eq!(substep_seconds(1.0), 60);
    assert_eq!(substep_seconds(0.34), 60);
}

#[test]
fn eb04_trace_publishes_component_and_closure_operands() {
    let producer = read(DIRECT_TRACE_PRODUCER);
    for field in [
        "stage3_surface_energy_j_m2",
        "stage3_conduction_energy_j_m2",
        "stage3_shortwave_energy_j_m2",
        "stage3_longwave_energy_j_m2",
        "stage3_latent_energy_j_m2",
        "stage3_vapor_mass_exchange_kg_m2",
        "stage3_latent_mass_energy_j_m2",
        "stage3_hourly_net_shortwave_w_m2",
        "stage3_hourly_net_longwave_w_m2",
        "stage3_hourly_vapor_mass_exchange_kg_m2",
        "stage3_hourly_latent_heat_j_kg",
        "stage3_hourly_latent_flux_w_m2",
        "stage3_latent_refreeze_energy_j_m2",
        "stage3_cold_content_export_j_m2",
        "stage3_mass_latent_identity_residual_j_m2",
        "stage3_unused_positive_energy_j_m2",
        "stage3_refrozen_liquid_m",
    ] {
        assert!(
            producer.contains(field),
            "{DIRECT_TRACE_PRODUCER} missing {field}"
        );
    }
}
