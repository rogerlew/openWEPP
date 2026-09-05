//! Solver regressions for `SC-LANDSURFACEENERGY-001`.

use super::*;

#[test]
fn stage3_optical_boundary_receipt_is_band_directional_and_digest_bound() {
    let digest = || Sha256Digest::try_new("a".repeat(64)).expect("digest");
    let terminal = BandDirectionalFluxes {
        direct_vis: 100.0,
        diffuse_vis: 40.0,
        direct_nir: 80.0,
        diffuse_nir: 20.0,
    };
    let absorbed = BandDirectionalFluxes {
        direct_vis: 82.0,
        diffuse_vis: 32.8,
        direct_nir: 64.0,
        diffuse_nir: 16.0,
    };
    let reflected = BandDirectionalFluxes {
        direct_vis: 18.0,
        diffuse_vis: 7.2,
        direct_nir: 16.0,
        diffuse_nir: 4.0,
    };
    let receipt =
        Stage3SnowOpticalBoundaryReceiptV1::try_new(Stage3SnowOpticalBoundaryReceiptInputs {
            ofe_id: OfeId::try_new("ofe-1").expect("OFE"),
            tile_id: openwepp_kernel_contract::TileId::try_new("snow").expect("tile"),
            terminal_w_m2_tile: terminal,
            absorbed_w_m2_tile: absorbed,
            reflected_w_m2_tile: reflected,
            snow_vis_albedo: 0.18,
            snow_nir_albedo: 0.20,
            stage3_albedo_state_sha256: digest(),
            forcing_receipt_sha256: digest(),
        })
        .expect("optical receipt");
    receipt.validate().expect("optical receipt validates");

    let mut poisoned = receipt;
    poisoned.reflected_w_m2_tile.direct_vis += 1.0;
    assert!(poisoned.validate().is_err());
}

fn v10_vector_close(actual: f64, expected: f64) {
    assert!(
        (actual - expected).abs() <= 1.0e-12 * expected.abs().max(1.0),
        "actual={actual:?} expected={expected:?}"
    );
}

#[test]
fn v10_and_v11_leaf_gas_match_frozen_dark_low_light_and_positive_vectors() {
    let vectors: serde_json::Value = serde_json::from_str(include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../docs/work-packages/20260818-c3-nighttime-ci-hold-lift-001/artifacts/nighttime-ci-vectors.json"
    )))
    .expect("frozen V10 leaf-gas vectors");
    let temperature = 296.0;
    let pressure = 101_325.0;
    let vcmax_factor =
        peaked(temperature, 65_330.0, 200_000.0, 650.0).expect("finite Vcmax response");
    let jmax_factor =
        peaked(temperature, 43_540.0, 200_000.0, 650.0).expect("finite Jmax response");
    let rd_factor = peaked(temperature, 46_390.0, 150_650.0, 490.0).expect("finite Rd response");
    let biochemical = BiochemicalConstants {
        ha_vcmax_j_mol: 65_330.0,
        hd_vcmax_j_mol: 200_000.0,
        entropy_vcmax_j_mol_k: 650.0,
        ha_jmax_j_mol: 43_540.0,
        hd_jmax_j_mol: 200_000.0,
        entropy_jmax_j_mol_k: 650.0,
        kc25_pa: 40.0 / arrhenius(temperature, 79_430.0).expect("Kc response"),
        ha_kc_j_mol: 79_430.0,
        ko25_pa: 30_000.0 / arrhenius(temperature, 36_380.0).expect("Ko response"),
        ha_ko_j_mol: 36_380.0,
        gamma25_pa: 5.0 / arrhenius(temperature, 37_830.0).expect("gamma response"),
        ha_gamma_j_mol: 37_830.0,
        oxygen_partial_pressure_pa: 20_265.0,
        tp_vcmax_ratio: 0.1,
        electron_quantum_yield: 0.85,
        par_photon_umol_per_j: 4.6,
        electron_curvature: 0.7,
        ac_aj_curvature: 0.98,
        ag_ap_curvature: 0.95,
    };
    let qsurface = canopy_saturation_q(temperature, pressure).expect("leaf saturation");
    let es_leaf = qsurface * pressure / (0.622 + 0.378 * qsurface);
    let e_can = es_leaf - 1_200.0;
    let qcan = 0.622 * e_can / (pressure - 0.378 * e_can);
    let environment = LeafGasEnvironment {
        authority: CoveredColumnAuthority::V10NonpositiveAssimilation,
        pressure_pa: pressure,
        ca_pa: 42.0,
    };
    let expected_branches = [
        V10LeafGasBranch::ExactZeroPar,
        V10LeafGasBranch::ExactZeroPar,
        V10LeafGasBranch::RespirationDominated,
        V10LeafGasBranch::RespirationDominated,
        V10LeafGasBranch::PositiveAssimilation,
    ];
    let cases = vectors["cases"].as_array().expect("V10 vector cases");
    assert_eq!(cases.len(), expected_branches.len());
    for (case, expected_branch) in cases.iter().zip(expected_branches) {
        let par = case["par_abs"].as_f64().expect("PAR");
        let actual = leaf_trial_state(
            LeafBiochemicalInputs {
                leaf_area_m2_m2_tile: 1.0,
                absorbed_shortwave_w_m2_tile: par,
                absorbed_par_w_m2_leaf: par,
                vcmax25: 60.0 / vcmax_factor,
                jmax25: 100.0 / jmax_factor,
                rd25: 1.2 / rd_factor,
            },
            biochemical,
            temperature,
            qcan,
            1.0,
            environment,
            0.02,
            100.0,
            4.0,
        )
        .expect("actual Rust V10 leaf-gas path");
        let v11 = leaf_trial_state(
            LeafBiochemicalInputs {
                leaf_area_m2_m2_tile: 1.0,
                absorbed_shortwave_w_m2_tile: par,
                absorbed_par_w_m2_leaf: par,
                vcmax25: 60.0 / vcmax_factor,
                jmax25: 100.0 / jmax_factor,
                rd25: 1.2 / rd_factor,
            },
            biochemical,
            temperature,
            qcan,
            1.0,
            LeafGasEnvironment {
                authority: CoveredColumnAuthority::V11SnowCovered,
                ..environment
            },
            0.02,
            100.0,
            4.0,
        )
        .expect("actual Rust V11 inherited leaf-gas path");
        assert_eq!(v11, actual, "V11 inherited branch differs for PAR={par:?}");
        assert_eq!(actual.gas_branch, expected_branch);
        let expected = &case["result"];
        v10_vector_close(actual.ci_pa, expected["ci"].as_f64().expect("Ci"));
        v10_vector_close(
            actual.gross_assimilation_umol_co2_m2_leaf_s,
            expected["state"]["ag"].as_f64().expect("Ag"),
        );
        v10_vector_close(
            actual.net_assimilation_umol_co2_m2_leaf_s,
            expected["state"]["an"].as_f64().expect("An"),
        );
        v10_vector_close(
            actual.dark_respiration_umol_co2_m2_leaf_s,
            expected["state"]["ag"].as_f64().expect("Ag")
                - expected["state"]["an"].as_f64().expect("An"),
        );
        v10_vector_close(actual.rs_s_m, expected["state"]["rs"].as_f64().expect("rs"));
    }

    let inactive = leaf_trial_state(
        LeafBiochemicalInputs {
            leaf_area_m2_m2_tile: 0.0,
            absorbed_shortwave_w_m2_tile: 0.0,
            absorbed_par_w_m2_leaf: 0.0,
            vcmax25: 60.0 / vcmax_factor,
            jmax25: 100.0 / jmax_factor,
            rd25: 1.2 / rd_factor,
        },
        biochemical,
        temperature,
        qcan,
        1.0,
        environment,
        0.02,
        100.0,
        4.0,
    )
    .expect("zero-area V10 leaf class");
    assert_eq!(inactive.gas_branch, V10LeafGasBranch::Inactive);

    let historical_dark = leaf_trial_state(
        LeafBiochemicalInputs {
            leaf_area_m2_m2_tile: 1.0,
            absorbed_shortwave_w_m2_tile: 0.0,
            absorbed_par_w_m2_leaf: 0.0,
            vcmax25: 60.0 / vcmax_factor,
            jmax25: 100.0 / jmax_factor,
            rd25: 1.2 / rd_factor,
        },
        biochemical,
        temperature,
        qcan,
        1.0,
        LeafGasEnvironment {
            authority: CoveredColumnAuthority::HistoricalV8,
            ..environment
        },
        0.02,
        100.0,
        4.0,
    );
    assert_eq!(
        historical_dark,
        Err(LandSurfaceEnergyError::ConstitutiveDomain("ci_bracket"))
    );
    assert!(!CoveredColumnAuthority::HistoricalV8.admits_nonpositive_assimilation());
    assert!(CoveredColumnAuthority::V10NonpositiveAssimilation.admits_nonpositive_assimilation());
    assert!(CoveredColumnAuthority::V11SnowCovered.admits_nonpositive_assimilation());

    let v11_dark_at_saturation = leaf_trial_state(
        LeafBiochemicalInputs {
            leaf_area_m2_m2_tile: 1.0,
            absorbed_shortwave_w_m2_tile: 0.0,
            absorbed_par_w_m2_leaf: -0.0,
            vcmax25: 60.0 / vcmax_factor,
            jmax25: 100.0 / jmax_factor,
            rd25: 1.2 / rd_factor,
        },
        biochemical,
        temperature,
        qsurface,
        1.0,
        LeafGasEnvironment {
            authority: CoveredColumnAuthority::V11SnowCovered,
            ..environment
        },
        0.02,
        100.0,
        4.0,
    )
    .expect("exact zero PAR selects the analytic branch before the VPD gate");
    assert_eq!(
        v11_dark_at_saturation.gas_branch,
        V10LeafGasBranch::ExactZeroPar
    );

    let positive_inputs = LeafBiochemicalInputs {
        leaf_area_m2_m2_tile: 1.0,
        absorbed_shortwave_w_m2_tile: 300.0,
        absorbed_par_w_m2_leaf: 180.0,
        vcmax25: 60.0 / vcmax_factor,
        jmax25: 100.0 / jmax_factor,
        rd25: 1.2 / rd_factor,
    };
    begin_covered_leaf_trial_audit(false);
    let exact_beta_one = leaf_trial_state(
        positive_inputs,
        biochemical,
        temperature,
        qcan,
        1.0,
        environment,
        0.02,
        100.0,
        4.0,
    )
    .expect("exact-beta-one current leaf");
    let exact_beta_one_maximum = covered_maximum_leaf_trial_state(
        exact_beta_one,
        1.0,
        positive_inputs,
        biochemical,
        temperature,
        qcan,
        environment,
        0.02,
        100.0,
        4.0,
    )
    .expect("exact-beta-one maximum leaf");
    assert_eq!(exact_beta_one_maximum, exact_beta_one);
    assert_eq!(take_covered_leaf_trial_audit(), 1);

    let below_one = f64::from_bits(1.0_f64.to_bits() - 1);
    begin_covered_leaf_trial_audit(false);
    let positive_below_one = leaf_trial_state(
        positive_inputs,
        biochemical,
        temperature,
        qcan,
        below_one,
        environment,
        0.02,
        100.0,
        4.0,
    )
    .expect("positive-PAR current leaf below beta one");
    assert_eq!(
        positive_below_one.gas_branch,
        V10LeafGasBranch::PositiveAssimilation
    );
    let positive_below_one_maximum = covered_maximum_leaf_trial_state(
        positive_below_one,
        below_one,
        positive_inputs,
        biochemical,
        temperature,
        qcan,
        environment,
        0.02,
        100.0,
        4.0,
    )
    .expect("positive-PAR maximum leaf");
    assert_eq!(take_covered_leaf_trial_audit(), 2);
    let positive_below_one_oracle = leaf_trial_state(
        positive_inputs,
        biochemical,
        temperature,
        qcan,
        1.0,
        environment,
        0.02,
        100.0,
        4.0,
    )
    .expect("positive-PAR exhaustive maximum oracle");
    assert_eq!(positive_below_one_maximum, positive_below_one_oracle);

    for (leaf_area, expected_branch) in [
        (1.0, V10LeafGasBranch::ExactZeroPar),
        (0.0, V10LeafGasBranch::Inactive),
    ] {
        let beta_independent_inputs = LeafBiochemicalInputs {
            leaf_area_m2_m2_tile: leaf_area,
            absorbed_shortwave_w_m2_tile: 0.0,
            absorbed_par_w_m2_leaf: 0.0,
            ..positive_inputs
        };
        begin_covered_leaf_trial_audit(false);
        let current = leaf_trial_state(
            beta_independent_inputs,
            biochemical,
            temperature,
            qcan,
            0.25,
            environment,
            0.02,
            100.0,
            4.0,
        )
        .expect("beta-independent current leaf");
        assert_eq!(current.gas_branch, expected_branch);
        let maximum = covered_maximum_leaf_trial_state(
            current,
            0.25,
            beta_independent_inputs,
            biochemical,
            temperature,
            qcan,
            environment,
            0.02,
            100.0,
            4.0,
        )
        .expect("beta-independent maximum leaf");
        assert_eq!(maximum, current);
        assert_eq!(take_covered_leaf_trial_audit(), 1);
        let exhaustive = leaf_trial_state(
            beta_independent_inputs,
            biochemical,
            temperature,
            qcan,
            1.0,
            environment,
            0.02,
            100.0,
            4.0,
        )
        .expect("beta-independent exhaustive maximum oracle");
        assert_eq!(maximum, exhaustive);
    }
}

#[test]
fn numerical_failure_debug_excludes_failed_iterate() {
    let failure = NumericalFailure {
        kind: NumericalFailureKind::IterationLimit,
        iterations: 50,
        normalized_residuals: vec![2.0],
        ordered_residuals: Vec::new(),
        failed_solution: vec![12_345.678_901_234_5],
        occupancy_id: None,
        active_bounds: Vec::new(),
        backtracking_count: 0,
        step_norms: StepNorms {
            temperature_k: None,
            humidity_kg_kg: None,
            ci_pa: None,
            hydraulic_mm: None,
            beta: None,
        },
        pivot_magnitude: None,
        matrix_norm: None,
    };
    let rendered = format!("{failure:?}");
    assert!(!rendered.contains("failed_solution"));
    assert!(!rendered.contains("12345.6789012345"));
}

fn norm_below_one_non_decreasing_poison(trial_residual: f64) -> NumericalFailure {
    let outcome = solve_normalized_system(
        |trial: &[f64], frozen: Option<&bool>| {
            let residual = if frozen.is_some() {
                0.5 + 1.0e6 * trial[0]
            } else if trial[0] == 0.0 {
                0.5
            } else {
                trial_residual
            };
            Ok((vec![residual], ()))
        },
        vec![0.0],
        &[1.0],
        |_| true,
        |()| true,
    )
    .expect("bounded strict-decrease poison");
    match outcome {
        NormalizedSolveOutcome::Rejected(failure) => failure,
        NormalizedSolveOutcome::Accepted { .. } => {
            panic!("non-decreasing norm below one must not be accepted")
        }
    }
}

#[test]
fn normalized_solver_rejects_stagnation_below_one() {
    let failure = norm_below_one_non_decreasing_poison(0.5);
    assert_eq!(failure.kind, NumericalFailureKind::BacktrackingLimit);
    assert_eq!(failure.normalized_residuals, vec![0.5]);
    assert!(failure.step_norms.temperature_k.is_some());
}

#[test]
fn normalized_solver_rejects_increase_remaining_below_one() {
    let failure = norm_below_one_non_decreasing_poison(0.75);
    assert_eq!(failure.kind, NumericalFailureKind::BacktrackingLimit);
    assert_eq!(failure.normalized_residuals, vec![0.5]);
    assert!(failure.step_norms.temperature_k.is_some());
}

#[test]
fn normalized_solver_uses_inward_jacobian_probe_at_closed_lower_bound() {
    let outcome = solve_normalized_system(
        |trial: &[f64], _: Option<&()>| {
            if trial[0] < 0.0 {
                return Err(LandSurfaceEnergyError::ConstitutiveDomain(
                    "test closed lower bound",
                ));
            }
            Ok((vec![trial[0]], ()))
        },
        vec![0.0],
        &[1.0],
        |trial| trial[0] >= 0.0,
        |()| (),
    )
    .expect("inward one-sided Jacobian probe");
    assert!(matches!(outcome, NormalizedSolveOutcome::Accepted { .. }));
}

fn distinct_bands(total: f64, seed: f64) -> BandDirectionalFluxes {
    let direct_vis = total * (0.10 + seed);
    let diffuse_vis = total * (0.17 - seed / 2.0);
    let direct_nir = total * (0.29 + seed / 3.0);
    BandDirectionalFluxes {
        direct_vis,
        diffuse_vis,
        direct_nir,
        diffuse_nir: total - direct_vis - diffuse_vis - direct_nir,
    }
}

fn bound_shortwave(
    occupancies: &[CoveredOccupancyInputs],
    terminal: BandDirectionalFluxes,
    surface_vis_albedo: f64,
    surface_nir_albedo: f64,
) -> CoveredColumnShortwaveInputs {
    let rows = occupancies
        .iter()
        .enumerate()
        .map(|(index, occupancy)| {
            let index = f64::from(u32::try_from(index).expect("bounded occupancy count"));
            CoveredOccupancyShortwaveInputs {
                occupancy_id: occupancy.occupancy_id.clone(),
                sun_leaf_absorbed_w_m2_tile: distinct_bands(
                    occupancy.sun.absorbed_shortwave_w_m2_tile,
                    0.01 * index,
                ),
                shade_leaf_absorbed_w_m2_tile: distinct_bands(
                    occupancy.shade.absorbed_shortwave_w_m2_tile,
                    0.02 + 0.01 * index,
                ),
                stem_absorbed_w_m2_tile: distinct_bands(
                    occupancy.stem_absorbed_shortwave_w_m2_tile,
                    0.04 + 0.01 * index,
                ),
            }
        })
        .collect::<Vec<_>>();
    let top_reflected = BandDirectionalFluxes {
        direct_vis: 7.0,
        diffuse_vis: 11.0,
        direct_nir: 13.0,
        diffuse_nir: 17.0,
    };
    let ground_absorbed =
        crate::partition_ground_shortwave(terminal, surface_vis_albedo, surface_nir_albedo)
            .expect("ground shortwave partition")
            .absorbed;
    let mut incident = directional_values(top_reflected);
    let ground_absorbed_values = directional_values(ground_absorbed);
    for index in 0..4 {
        incident[index] += ground_absorbed_values[index]
            + rows
                .iter()
                .map(|row| {
                    directional_values(row.sun_leaf_absorbed_w_m2_tile)[index]
                        + directional_values(row.shade_leaf_absorbed_w_m2_tile)[index]
                        + directional_values(row.stem_absorbed_w_m2_tile)[index]
                })
                .sum::<f64>();
    }
    CoveredColumnShortwaveInputs {
        incident_w_m2_tile: BandDirectionalFluxes {
            direct_vis: incident[0],
            diffuse_vis: incident[1],
            direct_nir: incident[2],
            diffuse_nir: incident[3],
        },
        top_reflected_w_m2_tile: top_reflected,
        ground_absorbed_by_incident_w_m2_tile: ground_absorbed,
        occupancies: rows,
    }
}

fn four_layer_problem() -> OpenSurfaceProblem {
    OpenSurfaceProblem {
        interval_s: 1_800.0,
        tile_fraction: 1.0,
        class: SurfaceClassKind::BareMineralSoil,
        storage_branch: SurfaceStorageBranch::FiniteCapacity,
        terminal_shortwave_w_m2_tile: BandDirectionalFluxes {
            direct_vis: 91.0,
            diffuse_vis: 31.0,
            direct_nir: 117.0,
            diffuse_nir: 39.0,
        },
        surface_vis_albedo: 0.18,
        surface_nir_albedo: 0.31,
        surface_emissivity: 1.0,
        surface_depth_m: 0.02,
        surface_conductivity_w_m_k: 0.75,
        surface_dry_heat_capacity_j_m2_k: 42_000.0,
        litter_capacity_kg_m2_tile: None,
        open_geometry: OpenNeutralGeometry {
            reference_height_m: 20.0,
            roughness_momentum_m: 0.12,
            roughness_heat_m: 0.015,
            roughness_vapor_m: 0.010,
        },
        air_temperature_k: 294.0,
        air_specific_humidity_kg_kg: 0.0095,
        air_pressure_pa: 93_000.0,
        reference_wind_m_s: 2.4,
        atmospheric_downward_longwave_w_m2: 335.0,
        surface_liquid_kg_m2_tile: 0.0,
        surface_enthalpy_j_m2_tile: 42_000.0 * (295.0 - REFERENCE_TEMPERATURE_K),
        surface_temperature_warm_start_k: 295.0,
        bare_soil: Some(BareSoilParameters {
            top_layer_liquid_kg_m2: 26.0,
            top_layer_ice_kg_m2: 0.0,
            porosity: 0.46,
            saturated_matric_potential_mm: -120.0,
            clapp_hornberger_b: 4.05,
            theta_initial: 0.22,
        }),
        soil_nodes: (0..4)
            .map(|index| SoilThermalNodeOperands {
                layer_id: format!("thermal-{}", index + 1),
                depth_m: 0.08 + 0.05 * f64::from(index),
                conductivity_w_m_k: 1.1 + 0.12 * f64::from(index),
                heat_capacity_j_m2_k: 120_000.0 + 35_000.0 * f64::from(index),
                beginning_temperature_k: 291.5 - 1.1 * f64::from(index),
            })
            .collect(),
    }
}

#[test]
fn open_failed_solution_reports_every_temperature_bound_hit() {
    let mut problem = four_layer_problem();
    problem.air_specific_humidity_kg_kg = 0.0;
    problem.surface_liquid_kg_m2_tile = 0.0;
    let bounds = open_failure_bounds(&problem, &[200.0, 350.0, 200.0, 350.0, 200.0], Some(0.0));
    assert_eq!(
        bounds,
        vec![
            "ground_surface_temperature_k:lower",
            "soil_temperature_k:thermal-1:upper",
            "soil_temperature_k:thermal-2:lower",
            "soil_temperature_k:thermal-3:upper",
            "soil_temperature_k:thermal-4:lower",
            "air_specific_humidity_kg_kg:lower",
            "surface_liquid_kg_m2_tile:lower",
            "ground_water_authorization_cap_rate:lower",
        ]
    );
}

#[test]
fn fixed_cap_rebuild_matches_independent_four_layer_vector() {
    let problem = four_layer_problem();
    let cap = 0.000_053_040_160_893_323_02;
    let result = solve_open_surface(&problem, Some(cap), None).expect("valid solve");
    let OpenSurfaceSolveOutcome::Accepted(accepted) = result else {
        panic!("expected accepted vector");
    };
    let expected = [
        293.055_973_826_482_8,
        291.973_087_253_131_9,
        290.461_060_535_837_25,
        289.318_132_686_869_4,
        288.259_177_307_246_15,
    ];
    for (actual, expected) in accepted.solution.iter().zip(expected) {
        assert!(
            (actual - expected).abs() < 2.0e-10,
            "{actual} != {expected}"
        );
    }
    assert_eq!(
        accepted.evaluation.water.branch,
        WaterBranch::AuthorizationActiveOrTie
    );
    assert_eq!(
        accepted.evaluation.water.finalized_use_kg_m2_stand_ground,
        cap * problem.interval_s
    );
}

#[test]
fn alternate_warm_start_converges_to_same_accepted_state() {
    let problem = four_layer_problem();
    let cap = 0.000_053_040_160_893_323_02;
    let first = solve_open_surface(&problem, Some(cap), None).expect("first");
    let alternate = vec![298.0, 289.0, 288.0, 287.0, 286.0];
    let second = solve_open_surface(&problem, Some(cap), Some(alternate)).expect("second");
    let (OpenSurfaceSolveOutcome::Accepted(first), OpenSurfaceSolveOutcome::Accepted(second)) =
        (first, second)
    else {
        panic!("both starts must converge");
    };
    for (left, right) in first.solution.iter().zip(second.solution.iter()) {
        assert!((left - right).abs() < 2.0e-10);
    }
}

#[test]
fn potential_and_final_do_not_mutate_beginning_problem() {
    let problem = four_layer_problem();
    let beginning = problem.clone();
    let potential = solve_open_surface(&problem, None, None).expect("potential");
    assert!(matches!(potential, OpenSurfaceSolveOutcome::Accepted(_)));
    let final_pass =
        solve_open_surface(&problem, Some(0.000_053_040_160_893_323_02), None).expect("capped");
    assert!(matches!(final_pass, OpenSurfaceSolveOutcome::Accepted(_)));
    assert_eq!(problem, beginning);
}

#[test]
fn equality_is_cap_active() {
    let problem = four_layer_problem();
    let trial = problem.initial_trial();
    let potential = evaluate_open_surface(&problem, &trial, None, None).expect("potential");
    let capped = evaluate_open_surface(
        &problem,
        &trial,
        Some(potential.water.law_kg_m2_tile_s),
        None,
    )
    .expect("tie");
    assert_eq!(capped.water.branch, WaterBranch::AuthorizationActiveOrTie);
}

#[test]
fn covered_v8_block_matches_frozen_joint_solution() {
    let ground = OpenSurfaceProblem {
        interval_s: 1_800.0,
        tile_fraction: 0.38,
        class: SurfaceClassKind::ForestLitter,
        storage_branch: SurfaceStorageBranch::FiniteCapacity,
        terminal_shortwave_w_m2_tile: BandDirectionalFluxes {
            direct_vis: 47.412_973_012_352_3,
            diffuse_vis: 8.705_736_606_981_51,
            direct_nir: 41.052_696_144_841_63,
            diffuse_nir: 52.084_944_358_632_505,
        },
        surface_vis_albedo: 0.12,
        surface_nir_albedo: 0.24,
        surface_emissivity: 1.0,
        surface_depth_m: 0.04,
        surface_conductivity_w_m_k: 0.103,
        surface_dry_heat_capacity_j_m2_k: 3_235.68,
        litter_capacity_kg_m2_tile: Some(6.0),
        open_geometry: OpenNeutralGeometry {
            reference_height_m: 24.0,
            roughness_momentum_m: 1.25,
            roughness_heat_m: 0.12,
            roughness_vapor_m: 0.08,
        },
        air_temperature_k: 296.0,
        air_specific_humidity_kg_kg: 0.0102,
        air_pressure_pa: 101_325.0,
        reference_wind_m_s: 3.7,
        atmospheric_downward_longwave_w_m2: 395.0,
        surface_liquid_kg_m2_tile: 4.0,
        surface_enthalpy_j_m2_tile: 439_352.808_000_000_5,
        surface_temperature_warm_start_k: 295.0,
        bare_soil: None,
        soil_nodes: vec![
            SoilThermalNodeOperands {
                layer_id: "thermal-1".into(),
                depth_m: 0.08,
                conductivity_w_m_k: 1.1,
                heat_capacity_j_m2_k: 120_000.0,
                beginning_temperature_k: 291.5,
            },
            SoilThermalNodeOperands {
                layer_id: "thermal-2".into(),
                depth_m: 0.18,
                conductivity_w_m_k: 1.35,
                heat_capacity_j_m2_k: 180_000.0,
                beginning_temperature_k: 289.8,
            },
        ],
    };
    let mut column = CoveredColumnInputs {
        authority: CoveredColumnAuthority::HistoricalV8,
        interval_s: 1_800.0,
        tile_fraction: 0.38,
        pressure_pa: 101_325.0,
        air_temperature_k: 296.0,
        air_specific_humidity_kg_kg: 0.0102,
        reference_wind_m_s: 3.7,
        atmospheric_downward_longwave_w_m2: 395.0,
        ca_pa: 42.0,
        canopy_to_atmosphere_heat_resistance_s_m: 20.992_293_151_292_14,
        canopy_to_atmosphere_vapor_resistance_s_m: 22.734_132_598_127_985,
        latent_heat_j_kg: 2_501_000.0,
        top_rain_kg_m2_tile: 0.0,
        under_canopy_geometry: crate::physics::UnderCanopyGeometry {
            canopy_height_m: 12.5,
            canopy_roughness_m: 1.25,
            reference_height_m: 24.0,
            leaf_area_index: 2.708_333_333_333_333,
        },
        ground,
        occupancies: Vec::new(),
        shortwave: CoveredColumnShortwaveInputs {
            incident_w_m2_tile: BandDirectionalFluxes::default(),
            top_reflected_w_m2_tile: BandDirectionalFluxes::default(),
            ground_absorbed_by_incident_w_m2_tile: BandDirectionalFluxes::default(),
            occupancies: Vec::new(),
        },
        stage3_lower_boundary: None,
        stage3_optical: None,
    };
    let biochemical = BiochemicalConstants {
        ha_vcmax_j_mol: 65_330.0,
        hd_vcmax_j_mol: 200_000.0,
        entropy_vcmax_j_mol_k: 650.0,
        ha_jmax_j_mol: 43_540.0,
        hd_jmax_j_mol: 200_000.0,
        entropy_jmax_j_mol_k: 650.0,
        kc25_pa: 40.49,
        ha_kc_j_mol: 79_430.0,
        ko25_pa: 27_840.0,
        ha_ko_j_mol: 36_380.0,
        gamma25_pa: 4.275,
        ha_gamma_j_mol: 37_830.0,
        oxygen_partial_pressure_pa: 20_265.0,
        tp_vcmax_ratio: 0.167,
        electron_quantum_yield: 0.85,
        par_photon_umol_per_j: 4.6,
        electron_curvature: 0.7,
        ac_aj_curvature: 0.98,
        ag_ap_curvature: 0.95,
    };
    let occupancy = CoveredOccupancyInputs {
        occupancy_id: "canopy-rank-0".into(),
        medlyn_g1_kpa_sqrt: 3.5,
        g0_umol_m2_s: 25.0,
        sun: LeafBiochemicalInputs {
            leaf_area_m2_m2_tile: 1.110_267_869_704_946_6,
            absorbed_shortwave_w_m2_tile: 220.671_526_988_526_7,
            absorbed_par_w_m2_leaf: 136.733_826_525_724_48,
            vcmax25: 62.0,
            jmax25: 108.0,
            rd25: 1.15,
        },
        shade: LeafBiochemicalInputs {
            leaf_area_m2_m2_tile: 1.598_065_463_628_386_4,
            absorbed_shortwave_w_m2_tile: 300.708_550_892_603_5,
            absorbed_par_w_m2_leaf: 118.229_071_004_667_92,
            vcmax25: 41.0,
            jmax25: 74.0,
            rd25: 0.81,
        },
        biochemical,
        stem_area_m2_m2_tile: 0.72,
        stem_absorbed_shortwave_w_m2_tile: 185.377_620_426_979_95,
        beginning_canopy_liquid_kg_m2_tile: 0.018,
        liquid_interception_fraction: 0.35,
        liquid_capacity_kg_m2_plant: 0.023_328_503_368_824_437,
        stemflow_fraction: 0.08,
        gb_leaf_m_s: 0.035_961_386_715_575_215,
        gb_wet_m_s: 0.019_071_405_305_591_295,
        gb_stem_m_s: 0.013_082_876_106_352_972,
        lai: 2.708_333_333_333_333,
        sai: 0.72,
        clumping_index: 0.82,
        k1_sun_max_s1: 1.2e-6,
        k1_shade_max_s1: 1.2e-6,
        k2_max: 4.2e-6,
        k3_max_m_s: 5.0e-5,
        height_m: 12.5,
        root_to_leaf_area: 1.8,
        p50_leaf_mm: -9_800.0,
        p50_xylem_mm: -7_200.0,
        p50_root_mm: -14_000.0,
        vulnerability_exponent: 2.0,
        root_layers: vec![
            RootHydraulicLayer {
                layer_id: "soil-1".into(),
                accessible: true,
                frozen: false,
                root_fraction: 0.62,
                soil_potential_mm: 100.0,
                gravity_head_mm: 120.0,
                z3_m: 0.32,
                dxroot_m: 0.18,
                ksoil_m2_s: 6.0e-11,
            },
            RootHydraulicLayer {
                layer_id: "soil-2".into(),
                accessible: true,
                frozen: false,
                root_fraction: 0.38,
                soil_potential_mm: 100.0,
                gravity_head_mm: 360.0,
                z3_m: 0.55,
                dxroot_m: 0.24,
                ksoil_m2_s: 4.5e-11,
            },
            RootHydraulicLayer {
                layer_id: "soil-dry".into(),
                accessible: false,
                frozen: false,
                root_fraction: 0.0,
                soil_potential_mm: -9_000.0,
                gravity_head_mm: 600.0,
                z3_m: 0.8,
                dxroot_m: 0.31,
                ksoil_m2_s: 2.0e-7,
            },
            RootHydraulicLayer {
                layer_id: "soil-frozen".into(),
                accessible: true,
                frozen: true,
                root_fraction: 0.0,
                soil_potential_mm: -1_100.0,
                gravity_head_mm: 740.0,
                z3_m: 1.1,
                dxroot_m: 0.4,
                ksoil_m2_s: 1.0e-7,
            },
        ],
    };
    let block = [
        -17_238.603_315_214_05,
        -17_192.416_581_301_524,
        -17_085.265_386_264_447,
        -4_540.235_024_954_438,
        0.413_196_465_847_171_85,
        0.414_735_379_574_386_9,
        303.606_954_266_055_8,
        303.472_797_963_230_45,
        306.407_147_616_578_8,
        312.507_227_066_147_5,
    ];
    let residual = evaluate_covered_occupancy_block(
        &column,
        &occupancy,
        &block,
        300.094_986_089_824,
        0.013_930_509_809_668_333,
        [
            -24.324_096_100_383_827,
            -34.563_785_686_988_41,
            -56.025_633_011_908_55,
            -29.747_436_870_633_067,
        ],
    )
    .expect("joint block");
    assert!(residual.iter().all(|value| value.is_finite()));
    let mut redistribution = occupancy.clone();
    redistribution.root_layers[0].soil_potential_mm = -100_000.0;
    assert_eq!(
        evaluate_covered_occupancy_block(
            &column,
            &redistribution,
            &block,
            300.094_986_089_824,
            0.013_930_509_809_668_333,
            [
                -24.324_096_100_383_827,
                -34.563_785_686_988_41,
                -56.025_633_011_908_55,
                -29.747_436_870_633_067,
            ],
        ),
        Err(LandSurfaceEnergyError::UnsupportedDomain(
            "hydraulic_redistribution"
        )),
    );
    let mut fully_wet = occupancy.clone();
    fully_wet.beginning_canopy_liquid_kg_m2_tile =
        fully_wet.liquid_capacity_kg_m2_plant * (fully_wet.lai + fully_wet.sai);
    let fully_wet_residual = evaluate_covered_occupancy_block(
        &column,
        &fully_wet,
        &block,
        300.094_986_089_824,
        0.013_930_509_809_668_333,
        [
            -24.324_096_100_383_827,
            -34.563_785_686_988_41,
            -56.025_633_011_908_55,
            -29.747_436_870_633_067,
        ],
    )
    .expect("fully wet inactive dry components");
    for (index, temperature) in [(6, block[6]), (7, block[7]), (9, block[9])] {
        assert_eq!(
            fully_wet_residual[index].to_bits(),
            (temperature - 300.094_986_089_824).to_bits()
        );
    }
    column.occupancies = vec![occupancy];
    column.shortwave = bound_shortwave(
        &column.occupancies,
        column.ground.terminal_shortwave_w_m2_tile,
        column.ground.surface_vis_albedo,
        column.ground.surface_nir_albedo,
    );
    column.occupancies[0].sun.absorbed_shortwave_w_m2_tile = column.shortwave.occupancies[0]
        .sun_leaf_absorbed_w_m2_tile
        .total();
    column.occupancies[0].shade.absorbed_shortwave_w_m2_tile = column.shortwave.occupancies[0]
        .shade_leaf_absorbed_w_m2_tile
        .total();
    column.occupancies[0].stem_absorbed_shortwave_w_m2_tile = column.shortwave.occupancies[0]
        .stem_absorbed_w_m2_tile
        .total();
    let full_trial = [
        block.as_slice(),
        &[
            300.094_986_089_824,
            0.013_930_509_809_668_333,
            296.004_005_840_958_7,
            291.522_093_207_909_7,
            289.958_385_800_646_6,
        ],
    ]
    .concat();
    let full =
        evaluate_covered_column(&column, &full_trial, None, None).expect("full covered column");
    let mut exact_beta_one_trial = full_trial.clone();
    exact_beta_one_trial[4] = 1.0;
    exact_beta_one_trial[5] = 1.0;
    begin_covered_leaf_trial_audit(false);
    let reused_leaf_maximum = evaluate_covered_column(&column, &exact_beta_one_trial, None, None)
        .expect("covered column with exact leaf-maximum reuse");
    let reused_leaf_calls = take_covered_leaf_trial_audit();
    begin_covered_leaf_trial_audit(true);
    let exhaustive_leaf_maximum =
        evaluate_covered_column(&column, &exact_beta_one_trial, None, None)
            .expect("covered column with exhaustive leaf-maximum calls");
    let exhaustive_leaf_calls = take_covered_leaf_trial_audit();
    assert_eq!(
        reused_leaf_maximum, exhaustive_leaf_maximum,
        "OBL-LANDSURFACEENERGY-C-018 requires complete evaluation equality",
    );
    assert!(reused_leaf_calls < exhaustive_leaf_calls);
    assert_eq!(exhaustive_leaf_calls, 4);

    let mut zero_par_column = column.clone();
    zero_par_column.authority = CoveredColumnAuthority::V10NonpositiveAssimilation;
    zero_par_column.occupancies[0].sun.absorbed_par_w_m2_leaf = 0.0;
    zero_par_column.occupancies[0].shade.absorbed_par_w_m2_leaf = 0.0;
    begin_covered_leaf_trial_audit(false);
    let zero_par_reused = evaluate_covered_column(&zero_par_column, &full_trial, None, None)
        .expect("exact-zero-PAR complete evaluation with reuse");
    let zero_par_reused_calls = take_covered_leaf_trial_audit();
    begin_covered_leaf_trial_audit(true);
    let zero_par_exhaustive = evaluate_covered_column(&zero_par_column, &full_trial, None, None)
        .expect("exact-zero-PAR exhaustive complete evaluation");
    let zero_par_exhaustive_calls = take_covered_leaf_trial_audit();
    assert_eq!(zero_par_reused, zero_par_exhaustive);
    assert_eq!(zero_par_reused_calls, 2);
    assert_eq!(zero_par_exhaustive_calls, 4);
    begin_covered_leaf_trial_audit(false);
    let zero_par_reused_solve = solve_covered_column(&zero_par_column, None, full_trial.clone());
    let zero_par_reused_solve_calls = take_covered_leaf_trial_audit();
    begin_covered_leaf_trial_audit(true);
    let zero_par_exhaustive_solve =
        solve_covered_column(&zero_par_column, None, full_trial.clone());
    let zero_par_exhaustive_solve_calls = take_covered_leaf_trial_audit();
    assert_eq!(zero_par_reused_solve, zero_par_exhaustive_solve);
    assert!(zero_par_reused_solve_calls < zero_par_exhaustive_solve_calls);

    let mut inactive_leaf_column = column.clone();
    inactive_leaf_column.authority = CoveredColumnAuthority::V10NonpositiveAssimilation;
    inactive_leaf_column.occupancies[0].sun.leaf_area_m2_m2_tile = 0.0;
    inactive_leaf_column.occupancies[0]
        .shade
        .leaf_area_m2_m2_tile = 0.0;
    inactive_leaf_column.occupancies[0]
        .sun
        .absorbed_shortwave_w_m2_tile = 0.0;
    inactive_leaf_column.occupancies[0]
        .shade
        .absorbed_shortwave_w_m2_tile = 0.0;
    inactive_leaf_column.occupancies[0]
        .sun
        .absorbed_par_w_m2_leaf = 0.0;
    inactive_leaf_column.occupancies[0]
        .shade
        .absorbed_par_w_m2_leaf = 0.0;
    inactive_leaf_column.shortwave = bound_shortwave(
        &inactive_leaf_column.occupancies,
        inactive_leaf_column.ground.terminal_shortwave_w_m2_tile,
        inactive_leaf_column.ground.surface_vis_albedo,
        inactive_leaf_column.ground.surface_nir_albedo,
    );
    inactive_leaf_column.occupancies[0].stem_absorbed_shortwave_w_m2_tile =
        inactive_leaf_column.shortwave.occupancies[0]
            .stem_absorbed_w_m2_tile
            .total();
    begin_covered_leaf_trial_audit(false);
    let inactive_reused = evaluate_covered_column(&inactive_leaf_column, &full_trial, None, None)
        .expect("inactive-leaf complete evaluation with reuse");
    let inactive_reused_calls = take_covered_leaf_trial_audit();
    begin_covered_leaf_trial_audit(true);
    let inactive_exhaustive =
        evaluate_covered_column(&inactive_leaf_column, &full_trial, None, None)
            .expect("inactive-leaf exhaustive complete evaluation");
    let inactive_exhaustive_calls = take_covered_leaf_trial_audit();
    assert_eq!(inactive_reused, inactive_exhaustive);
    assert_eq!(inactive_reused_calls, 2);
    assert_eq!(inactive_exhaustive_calls, 4);
    assert_eq!(
        inactive_reused.occupancies[0].gas_branches,
        [V10LeafGasBranch::Inactive; 2]
    );
    begin_covered_leaf_trial_audit(false);
    let inactive_reused_solve =
        solve_covered_column(&inactive_leaf_column, None, full_trial.clone());
    let inactive_reused_solve_calls = take_covered_leaf_trial_audit();
    begin_covered_leaf_trial_audit(true);
    let inactive_exhaustive_solve =
        solve_covered_column(&inactive_leaf_column, None, full_trial.clone());
    let inactive_exhaustive_solve_calls = take_covered_leaf_trial_audit();
    assert_eq!(inactive_reused_solve, inactive_exhaustive_solve);
    assert!(inactive_reused_solve_calls < inactive_exhaustive_solve_calls);

    // Current sun then current shade must precede both maximum-demand calls.
    // A HistoricalV8 zero-PAR failure therefore occurs on call one for sun
    // and call two for shade, independent of the exhaustive-oracle switch.
    for (zero_sun, expected_calls) in [(true, 1), (false, 2)] {
        let mut ordered_error_column = column.clone();
        if zero_sun {
            ordered_error_column.occupancies[0]
                .sun
                .absorbed_par_w_m2_leaf = 0.0;
        } else {
            ordered_error_column.occupancies[0]
                .shade
                .absorbed_par_w_m2_leaf = 0.0;
        }
        begin_covered_leaf_trial_audit(false);
        let reused_error = evaluate_covered_column(&ordered_error_column, &full_trial, None, None);
        let reused_error_calls = take_covered_leaf_trial_audit();
        begin_covered_leaf_trial_audit(true);
        let exhaustive_error =
            evaluate_covered_column(&ordered_error_column, &full_trial, None, None);
        let exhaustive_error_calls = take_covered_leaf_trial_audit();
        assert_eq!(
            reused_error,
            Err(LandSurfaceEnergyError::ConstitutiveDomain("ci_bracket"))
        );
        assert_eq!(reused_error, exhaustive_error);
        assert_eq!(reused_error_calls, expected_calls);
        assert_eq!(exhaustive_error_calls, expected_calls);
    }
    let carbon = &full.occupancies[0];
    for class in 0..2 {
        assert!(carbon.gross_assimilation_umol_co2_m2_leaf_s[class].is_finite());
        assert!(carbon.dark_respiration_umol_co2_m2_leaf_s[class].is_finite());
        assert_eq!(
            carbon.net_assimilation_umol_co2_m2_leaf_s[class].to_bits(),
            (carbon.gross_assimilation_umol_co2_m2_leaf_s[class]
                - carbon.dark_respiration_umol_co2_m2_leaf_s[class])
                .to_bits()
        );
    }
    assert!(
        full.normalized_residuals
            .iter()
            .all(|value| value.is_finite())
    );

    // OBL-LANDSURFACEENERGY-C-017: represented-snow ground and soil
    // coordinates are exact identity anchors.  Their dependency-reused probe
    // vectors and dense Jacobian columns must remain bit-identical to the
    // complete evaluator for centered and both admitted inward stencils.
    let mut stage3_column = column.clone();
    stage3_column.authority = CoveredColumnAuthority::V11SnowCovered;
    let stage3_shortwave = partition_ground_shortwave(
        stage3_column.ground.terminal_shortwave_w_m2_tile,
        stage3_column.ground.surface_vis_albedo,
        stage3_column.ground.surface_nir_albedo,
    )
    .expect("Stage-3 ground optical partition");
    let stage3_digest = Sha256Digest::try_new("b".repeat(64)).expect("Stage-3 digest");
    let stage3_optical =
        Stage3SnowOpticalBoundaryReceiptV1::try_new(Stage3SnowOpticalBoundaryReceiptInputs {
            ofe_id: OfeId::try_new("ofe-stage3-anchor").expect("Stage-3 OFE"),
            tile_id: TileId::try_new("tile-stage3-anchor").expect("Stage-3 tile"),
            terminal_w_m2_tile: stage3_column.ground.terminal_shortwave_w_m2_tile,
            absorbed_w_m2_tile: stage3_shortwave.absorbed,
            reflected_w_m2_tile: stage3_shortwave.reflected,
            snow_vis_albedo: stage3_column.ground.surface_vis_albedo,
            snow_nir_albedo: stage3_column.ground.surface_nir_albedo,
            stage3_albedo_state_sha256: stage3_digest.clone(),
            forcing_receipt_sha256: stage3_digest.clone(),
        })
        .expect("Stage-3 optical receipt");
    let stage3_common = 10 * stage3_column.occupancies.len();
    stage3_column.stage3_lower_boundary = Some(Stage3SnowCoveredLowerBoundary {
        snow_temperature_k: full_trial[stage3_common + 2],
        latent_heat_j_kg: stage3_column.latent_heat_j_kg,
        sensible_to_canopy_air_w_m2: 0.0,
        vapor_to_canopy_air_kg_m2_s: 0.0,
        net_longwave_w_m2: 0.0,
        shortwave_absorbed_w_m2: stage3_shortwave.absorbed.total(),
        precipitation_advection_w_m2: 0.0,
        carrier_receipt_id: stage3_digest.clone(),
        snow_vis_albedo: stage3_column.ground.surface_vis_albedo,
        snow_nir_albedo: stage3_column.ground.surface_nir_albedo,
        stage3_albedo_state_sha256: stage3_digest.clone(),
        forcing_receipt_sha256: stage3_digest,
        optical_receipt_sha256: Some(stage3_optical.receipt_sha256.clone()),
        reciprocal_longwave_receipt_sha256: None,
        final_canopy_boundary_receipt_sha256: None,
    });
    stage3_column.stage3_optical = Some(stage3_optical);
    let validated_stage3 = ValidatedCoveredEvaluationInputs::try_new(&stage3_column, None)
        .expect("validated Stage-3 anchor fixture");
    begin_covered_leaf_trial_audit(false);
    let optimized_leaf_solve = solve_covered_column(&stage3_column, None, full_trial.clone());
    let optimized_leaf_calls = take_covered_leaf_trial_audit();
    begin_covered_leaf_trial_audit(true);
    let exhaustive_leaf_solve = solve_covered_column(&stage3_column, None, full_trial.clone());
    let exhaustive_leaf_calls = take_covered_leaf_trial_audit();
    assert_eq!(optimized_leaf_solve, exhaustive_leaf_solve);
    assert!(optimized_leaf_calls <= exhaustive_leaf_calls);

    begin_covered_jacobian_full_probe_audit();
    let optimized_probe_solve = solve_covered_column(&stage3_column, None, full_trial.clone());
    let optimized_probe_calls = take_covered_jacobian_full_probe_audit();
    begin_forced_complete_covered_jacobian_probe_audit();
    let complete_probe_solve = solve_covered_column(&stage3_column, None, full_trial.clone());
    let complete_probe_calls = take_covered_jacobian_full_probe_audit();
    assert_eq!(optimized_probe_solve, complete_probe_solve);
    assert!(optimized_probe_calls < complete_probe_calls);

    begin_covered_evaluation_input_validation_audit();
    evaluate_covered_column(&stage3_column, &full_trial, None, None)
        .expect("first public Stage-3 evaluation");
    evaluate_covered_column(&stage3_column, &full_trial, None, None)
        .expect("second public Stage-3 evaluation");
    assert_eq!(
        take_covered_evaluation_input_validation_audit(),
        2,
        "independent public evaluations must each admit immutable Stage-3 inputs"
    );
    let mut poisoned_stage3 = stage3_column.clone();
    poisoned_stage3
        .stage3_lower_boundary
        .as_mut()
        .expect("Stage-3 lower boundary")
        .forcing_receipt_sha256 = Sha256Digest::try_new("c".repeat(64)).expect("poison digest");
    assert_eq!(
        evaluate_covered_column(&poisoned_stage3, &full_trial, None, None),
        Err(LandSurfaceEnergyError::StateLineage(
            "Stage-3 snow optical/lower-boundary identity"
        ))
    );
    let mut invalid_initial_trial = full_trial.clone();
    invalid_initial_trial[0] = f64::NAN;
    assert_eq!(
        solve_covered_column(&poisoned_stage3, None, invalid_initial_trial),
        Err(LandSurfaceEnergyError::ConstitutiveDomain(
            "covered_initial_trial"
        )),
        "the existing initial-trial error precedes immutable Stage-3 admission"
    );
    let anchor_columns: Vec<usize> = (stage3_common + 2..full_trial.len()).collect();
    for anchor_column in anchor_columns {
        for anchor_current in [full_trial[anchor_column], 200.0, 350.0] {
            let mut current = full_trial.clone();
            current[anchor_column] = anchor_current;
            let base_proof = ValidatedCoveredJacobianBase::evaluate(&validated_stage3, &current)
                .expect("validated Stage-3 Jacobian base");
            let base = &base_proof.evaluation;
            let frozen = freeze_covered_branches(base);
            let perturbation = f64::EPSILON.sqrt() * current[anchor_column].abs().max(1.0);
            let mut minus = current.clone();
            let mut plus = current.clone();
            minus[anchor_column] -= perturbation;
            plus[anchor_column] += perturbation;
            let stencil = covered_finite_difference_stencil(
                &current,
                &minus,
                &plus,
                stage3_column.occupancies.len(),
                false,
            )
            .expect("Stage-3 anchor stencil");

            begin_covered_jacobian_full_probe_audit();
            let minus_is_valid =
                covered_trial_is_valid(&minus, stage3_column.occupancies.len(), false);
            let plus_is_valid =
                covered_trial_is_valid(&plus, stage3_column.occupancies.len(), false);
            let actual_minus = minus_is_valid
                .then(|| covered_jacobian_probe_residuals(&base_proof, &minus, anchor_column))
                .transpose()
                .expect("dependency-reused minus probe");
            let actual_plus = plus_is_valid
                .then(|| covered_jacobian_probe_residuals(&base_proof, &plus, anchor_column))
                .transpose()
                .expect("dependency-reused plus probe");
            assert_eq!(
                take_covered_jacobian_full_probe_audit(),
                0,
                "Stage-3 identity anchors must not invoke the complete evaluator"
            );

            let expected_minus = minus_is_valid.then(|| {
                evaluate_covered_column_validated(&validated_stage3, &minus, Some(&frozen), None)
                    .expect("complete minus oracle")
                    .normalized_residuals
            });
            let expected_plus = plus_is_valid.then(|| {
                evaluate_covered_column_validated(&validated_stage3, &plus, Some(&frozen), None)
                    .expect("complete plus oracle")
                    .normalized_residuals
            });
            for row in 0..current.len() {
                let actual_minus_value = actual_minus.as_ref().map(|values| values[row]);
                let actual_plus_value = actual_plus.as_ref().map(|values| values[row]);
                assert_eq!(
                    actual_minus_value.map(f64::to_bits),
                    expected_minus.as_ref().map(|values| values[row].to_bits()),
                    "minus residual row {row}, anchor column {anchor_column}"
                );
                assert_eq!(
                    actual_plus_value.map(f64::to_bits),
                    expected_plus.as_ref().map(|values| values[row].to_bits()),
                    "plus residual row {row}, anchor column {anchor_column}"
                );
                let actual_jacobian = covered_finite_difference_value(
                    stencil,
                    base.normalized_residuals[row],
                    actual_minus_value,
                    actual_plus_value,
                    perturbation,
                )
                .expect("dependency-reused Jacobian value");
                let expected_jacobian = covered_finite_difference_value(
                    stencil,
                    base.normalized_residuals[row],
                    expected_minus.as_ref().map(|values| values[row]),
                    expected_plus.as_ref().map(|values| values[row]),
                    perturbation,
                )
                .expect("complete-evaluator Jacobian value");
                assert_eq!(
                    actual_jacobian.to_bits(),
                    expected_jacobian.to_bits(),
                    "Jacobian row {row}, anchor column {anchor_column}"
                );
            }
        }
    }

    // The rejected V30 experiment must leave every hydraulic coordinate on
    // the complete evaluator.  Preserve its differential oracle as a guard
    // against accidentally reviving the unretained shortcut.
    let base_stage3_proof = ValidatedCoveredJacobianBase::evaluate(&validated_stage3, &full_trial)
        .expect("validated hydraulic Jacobian base");
    let base_stage3 = &base_stage3_proof.evaluation;
    let frozen_stage3 = freeze_covered_branches(base_stage3);
    for hydraulic_column in 0..4 {
        let perturbation = f64::EPSILON.sqrt() * full_trial[hydraulic_column].abs().max(1_000.0);
        let mut minus = full_trial.clone();
        let mut plus = full_trial.clone();
        minus[hydraulic_column] -= perturbation;
        plus[hydraulic_column] += perturbation;
        let stencil = covered_finite_difference_stencil(
            &full_trial,
            &minus,
            &plus,
            stage3_column.occupancies.len(),
            false,
        )
        .expect("hydraulic centered stencil");
        begin_covered_jacobian_full_probe_audit();
        let actual_minus =
            covered_jacobian_probe_residuals(&base_stage3_proof, &minus, hydraulic_column)
                .expect("dependency-recomputed hydraulic minus probe");
        let actual_plus =
            covered_jacobian_probe_residuals(&base_stage3_proof, &plus, hydraulic_column)
                .expect("dependency-recomputed hydraulic plus probe");
        assert_eq!(
            take_covered_jacobian_full_probe_audit(),
            2,
            "hydraulic probes must retain both complete evaluations"
        );
        let expected_minus = evaluate_covered_column_validated(
            &validated_stage3,
            &minus,
            Some(&frozen_stage3),
            None,
        )
        .expect("complete hydraulic minus oracle");
        let expected_plus =
            evaluate_covered_column_validated(&validated_stage3, &plus, Some(&frozen_stage3), None)
                .expect("complete hydraulic plus oracle");
        for row in 0..full_trial.len() {
            assert_eq!(
                actual_minus[row].to_bits(),
                expected_minus.normalized_residuals[row].to_bits(),
                "minus residual row {row}, hydraulic column {hydraulic_column}"
            );
            assert_eq!(
                actual_plus[row].to_bits(),
                expected_plus.normalized_residuals[row].to_bits(),
                "plus residual row {row}, hydraulic column {hydraulic_column}"
            );
            assert_eq!(
                covered_finite_difference_value(
                    stencil,
                    base_stage3.normalized_residuals[row],
                    Some(actual_minus[row]),
                    Some(actual_plus[row]),
                    perturbation,
                )
                .expect("recomputed hydraulic Jacobian")
                .to_bits(),
                covered_finite_difference_value(
                    stencil,
                    base_stage3.normalized_residuals[row],
                    Some(expected_minus.normalized_residuals[row]),
                    Some(expected_plus.normalized_residuals[row]),
                    perturbation,
                )
                .expect("complete hydraulic Jacobian")
                .to_bits(),
                "Jacobian row {row}, hydraulic column {hydraulic_column}"
            );
        }
    }

    let mut zero_area_column = stage3_column.clone();
    let zero_area_occupancy = &mut zero_area_column.occupancies[0];
    zero_area_occupancy.beginning_canopy_liquid_kg_m2_tile = zero_area_occupancy
        .liquid_capacity_kg_m2_plant
        * (zero_area_occupancy.lai + zero_area_occupancy.sai);
    let validated_zero_area = ValidatedCoveredEvaluationInputs::try_new(&zero_area_column, None)
        .expect("validated zero-area column");
    let zero_area_base_proof =
        ValidatedCoveredJacobianBase::evaluate(&validated_zero_area, &full_trial)
            .expect("evaluated zero-area fallback base");
    assert_eq!(
        zero_area_base_proof.evaluation.occupancies[0].component_areas_m2_m2_tile[0].to_bits(),
        0.0_f64.to_bits(),
    );
    let zero_area_frozen = freeze_covered_branches(&zero_area_base_proof.evaluation);
    let hydraulic_column = 0;
    let perturbation = f64::EPSILON.sqrt() * full_trial[hydraulic_column].abs().max(1_000.0);
    let mut zero_area_probe = full_trial.clone();
    zero_area_probe[hydraulic_column] -= perturbation;
    begin_covered_jacobian_full_probe_audit();
    let zero_area_fallback =
        covered_jacobian_probe_residuals(&zero_area_base_proof, &zero_area_probe, hydraulic_column)
            .expect("zero-area hydraulic proof falls back to complete evaluation");
    assert_eq!(take_covered_jacobian_full_probe_audit(), 1);

    // HistoricalV8 forbids hydraulic redistribution.  Place one active layer
    // exactly at zero flow in the complete base map so the canonical plus
    // psi_root probe crosses into the typed failure.  Ordinary and explicitly
    // forced complete evaluation must return the same first error, and the
    // complete solver outcomes must agree as well.
    let mut redistribution_column = column.clone();
    redistribution_column.occupancies[0].root_layers[0].soil_potential_mm = full_trial[3];
    redistribution_column.occupancies[0].root_layers[0].gravity_head_mm = 0.0;
    let validated_redistribution =
        ValidatedCoveredEvaluationInputs::try_new(&redistribution_column, None)
            .expect("validated redistribution-boundary fixture");
    let redistribution_base =
        ValidatedCoveredJacobianBase::evaluate(&validated_redistribution, &full_trial)
            .expect("zero-flow redistribution base");
    let redistribution_perturbation = f64::EPSILON.sqrt() * full_trial[3].abs().max(1_000.0);
    let mut redistribution_probe = full_trial.clone();
    redistribution_probe[3] += redistribution_perturbation;
    begin_covered_jacobian_full_probe_audit();
    let optimized_redistribution_error =
        covered_jacobian_probe_residuals(&redistribution_base, &redistribution_probe, 3);
    assert_eq!(take_covered_jacobian_full_probe_audit(), 1);
    begin_forced_complete_covered_jacobian_probe_audit();
    let complete_redistribution_error =
        covered_jacobian_probe_residuals(&redistribution_base, &redistribution_probe, 3);
    assert_eq!(take_covered_jacobian_full_probe_audit(), 1);
    assert_eq!(
        optimized_redistribution_error,
        Err(LandSurfaceEnergyError::UnsupportedDomain(
            "hydraulic_redistribution"
        ))
    );
    assert_eq!(
        optimized_redistribution_error,
        complete_redistribution_error
    );
    begin_covered_jacobian_full_probe_audit();
    let optimized_redistribution_solve =
        solve_covered_column(&redistribution_column, None, full_trial.clone());
    let optimized_redistribution_complete_calls = take_covered_jacobian_full_probe_audit();
    begin_forced_complete_covered_jacobian_probe_audit();
    let complete_redistribution_solve =
        solve_covered_column(&redistribution_column, None, full_trial.clone());
    let complete_redistribution_complete_calls = take_covered_jacobian_full_probe_audit();
    assert_eq!(
        optimized_redistribution_solve,
        complete_redistribution_solve
    );
    assert_eq!(
        optimized_redistribution_solve,
        Err(LandSurfaceEnergyError::UnsupportedDomain(
            "hydraulic_redistribution"
        ))
    );
    assert_eq!(
        optimized_redistribution_complete_calls,
        complete_redistribution_complete_calls
    );
    let zero_area_oracle = evaluate_covered_column_validated(
        &validated_zero_area,
        &zero_area_probe,
        Some(&zero_area_frozen),
        None,
    )
    .expect("zero-area complete oracle");
    for (actual, expected) in zero_area_fallback
        .iter()
        .zip(&zero_area_oracle.normalized_residuals)
    {
        assert_eq!(actual.to_bits(), expected.to_bits());
    }
    let mut multi_coordinate_probe = zero_area_probe.clone();
    multi_coordinate_probe[4] = f64::from_bits(multi_coordinate_probe[4].to_bits() + 1);
    begin_covered_jacobian_full_probe_audit();
    let multi_coordinate_fallback =
        covered_jacobian_probe_residuals(&base_stage3_proof, &multi_coordinate_probe, 0)
            .expect("multi-coordinate probe falls back to complete evaluation");
    assert_eq!(take_covered_jacobian_full_probe_audit(), 1);
    let multi_coordinate_oracle = evaluate_covered_column_validated(
        &validated_stage3,
        &multi_coordinate_probe,
        Some(&frozen_stage3),
        None,
    )
    .expect("multi-coordinate complete oracle");
    assert_eq!(
        multi_coordinate_fallback,
        multi_coordinate_oracle.normalized_residuals
    );
    begin_covered_jacobian_full_probe_audit();
    assert_eq!(
        covered_jacobian_probe_residuals(&base_stage3_proof, &full_trial[..5], 0,),
        Err(LandSurfaceEnergyError::ConstitutiveDomain(
            "covered_trial_shape_or_bounds"
        )),
    );
    assert_eq!(take_covered_jacobian_full_probe_audit(), 1);

    // A beta column must retain both complete centered evaluations.
    let non_anchor_column = 4;
    let non_anchor_perturbation =
        f64::EPSILON.sqrt() * full_trial[non_anchor_column].abs().max(1_000.0);
    let mut non_anchor_minus = full_trial.clone();
    let mut non_anchor_plus = full_trial.clone();
    non_anchor_minus[non_anchor_column] -= non_anchor_perturbation;
    non_anchor_plus[non_anchor_column] += non_anchor_perturbation;
    begin_covered_jacobian_full_probe_audit();
    covered_jacobian_probe_residuals(&base_stage3_proof, &non_anchor_minus, non_anchor_column)
        .expect("complete non-anchor minus probe");
    covered_jacobian_probe_residuals(&base_stage3_proof, &non_anchor_plus, non_anchor_column)
        .expect("complete non-anchor plus probe");
    assert_eq!(
        take_covered_jacobian_full_probe_audit(),
        2,
        "non-anchor probes must retain the complete evaluator"
    );

    let mut closed_bound_trial = full_trial.clone();
    closed_bound_trial[4] = 0.0;
    closed_bound_trial[5] = 1.0;
    begin_covered_leaf_trial_audit(false);
    let closed_bound_potential = solve_covered_column(&column, None, closed_bound_trial.clone());
    let closed_bound_reused_calls = take_covered_leaf_trial_audit();
    begin_covered_leaf_trial_audit(true);
    let closed_bound_exhaustive = solve_covered_column(&column, None, closed_bound_trial.clone());
    let closed_bound_exhaustive_calls = take_covered_leaf_trial_audit();
    assert_eq!(closed_bound_potential, closed_bound_exhaustive);
    assert!(closed_bound_reused_calls < closed_bound_exhaustive_calls);
    assert!(
        closed_bound_potential.is_ok(),
        "an admitted beta-bound potential iterate must construct its Jacobian: {closed_bound_potential:?}"
    );
    begin_covered_evaluation_input_validation_audit();
    let solved = solve_covered_column(&column, None, full_trial.clone()).expect("potential solve");
    assert_eq!(
        take_covered_evaluation_input_validation_audit(),
        1,
        "one immutable covered-column admission must feed every evaluation in one canonical solve"
    );
    assert_eq!(
        take_covered_caps_validation_audit(),
        1,
        "authorization caps must be admitted exactly once per canonical solve"
    );
    let CoveredColumnSolveOutcome::Accepted(potential) = solved else {
        panic!("potential must accept");
    };
    let mut root_caps = BTreeMap::new();
    for source in &potential.root_water {
        let request_rate =
            source.request_kg_m2_stand_ground / (column.tile_fraction * column.interval_s);
        let authorization_rate = match source.layer_id.as_str() {
            "soil-1" => 5.449_439_753_166_194e-6,
            "soil-2" => 2.003_239_473_339_757_3e-6,
            _ => 0.0,
        };
        root_caps.insert(
            (source.occupancy_id.clone(), source.layer_id.clone()),
            SourceWaterCap {
                request_rate_kg_m2_tile_s: request_rate,
                authorization_rate_kg_m2_tile_s: authorization_rate,
            },
        );
    }
    let caps = CoveredWaterCaps {
        root: root_caps,
        ground: SourceWaterCap {
            request_rate_kg_m2_tile_s: potential.ground_water.request_kg_m2_stand_ground
                / (column.tile_fraction * column.interval_s),
            authorization_rate_kg_m2_tile_s: 1.226_044_233_320_78e-4,
        },
    };
    let closed_bound_final = solve_covered_column(&column, Some(&caps), closed_bound_trial);
    assert!(
        closed_bound_final.is_ok(),
        "an admitted beta-bound fixed-final iterate must construct its Jacobian: {closed_bound_final:?}"
    );
    let capped_trial = vec![
        -16_824.779_647_297_01,
        -16_712.589_117_241_627,
        -16_664.596_249_631_624,
        -4_125.915_697_953_702,
        0.462_302_155_485_367_2,
        0.424_353_863_538_429,
        305.035_913_166_871_3,
        305.089_706_734_110_45,
        307.913_092_586_148_65,
        314.005_994_994_191_95,
        301.610_676_717_580_6,
        0.012_920_640_609_040_12,
        298.721_285_856_343_5,
        291.602_314_111_137_45,
        289.962_098_689_575_8,
    ];
    let validated_capped = ValidatedCoveredEvaluationInputs::try_new(&column, Some(&caps))
        .expect("validated capped hydraulic fixture");
    let capped_base_proof =
        ValidatedCoveredJacobianBase::evaluate(&validated_capped, &capped_trial)
            .expect("validated capped Jacobian base");
    let capped_frozen = freeze_covered_branches(&capped_base_proof.evaluation);
    for capped_hydraulic_column in 0..4 {
        let capped_perturbation =
            f64::EPSILON.sqrt() * capped_trial[capped_hydraulic_column].abs().max(1_000.0);
        let mut capped_minus = capped_trial.clone();
        let mut capped_plus = capped_trial.clone();
        capped_minus[capped_hydraulic_column] -= capped_perturbation;
        capped_plus[capped_hydraulic_column] += capped_perturbation;
        begin_covered_jacobian_full_probe_audit();
        let capped_recomputed_minus = covered_jacobian_probe_residuals(
            &capped_base_proof,
            &capped_minus,
            capped_hydraulic_column,
        )
        .expect("capped hydraulic minus recomputation");
        let capped_recomputed_plus = covered_jacobian_probe_residuals(
            &capped_base_proof,
            &capped_plus,
            capped_hydraulic_column,
        )
        .expect("capped hydraulic plus recomputation");
        assert_eq!(take_covered_jacobian_full_probe_audit(), 2);
        let capped_complete_minus = evaluate_covered_column_validated(
            &validated_capped,
            &capped_minus,
            Some(&capped_frozen),
            None,
        )
        .expect("capped complete minus probe oracle");
        let capped_complete_plus = evaluate_covered_column_validated(
            &validated_capped,
            &capped_plus,
            Some(&capped_frozen),
            None,
        )
        .expect("capped complete plus probe oracle");
        for row in 0..capped_trial.len() {
            assert_eq!(
                capped_recomputed_minus[row].to_bits(),
                capped_complete_minus.normalized_residuals[row].to_bits(),
                "capped minus row {row}, hydraulic column {capped_hydraulic_column}"
            );
            assert_eq!(
                capped_recomputed_plus[row].to_bits(),
                capped_complete_plus.normalized_residuals[row].to_bits(),
                "capped plus row {row}, hydraulic column {capped_hydraulic_column}"
            );
            assert_eq!(
                covered_finite_difference_value(
                    CoveredFiniteDifferenceStencil::Centered,
                    capped_base_proof.evaluation.normalized_residuals[row],
                    Some(capped_recomputed_minus[row]),
                    Some(capped_recomputed_plus[row]),
                    capped_perturbation,
                )
                .expect("capped recomputed Jacobian")
                .to_bits(),
                covered_finite_difference_value(
                    CoveredFiniteDifferenceStencil::Centered,
                    capped_base_proof.evaluation.normalized_residuals[row],
                    Some(capped_complete_minus.normalized_residuals[row]),
                    Some(capped_complete_plus.normalized_residuals[row]),
                    capped_perturbation,
                )
                .expect("capped complete Jacobian")
                .to_bits(),
                "capped Jacobian row {row}, hydraulic column {capped_hydraulic_column}"
            );
        }
    }
    let transaction = execute_covered_potential_final(&column, full_trial, &caps, capped_trial)
        .expect("immutable potential/final transaction");
    assert_ne!(
        transaction.potential.evaluation.occupancies[0].gross_assimilation_umol_co2_m2_leaf_s[0]
            .to_bits(),
        transaction.final_pass.evaluation.occupancies[0].gross_assimilation_umol_co2_m2_leaf_s[0]
            .to_bits(),
        "cap-active accepted carbon must come from the rebuilt final solve"
    );
    assert_eq!(
        transaction.final_pass.ground_water.branch,
        WaterBranch::AuthorizationActiveOrTie
    );
    assert!(
        transaction
            .final_pass
            .root_water
            .iter()
            .all(|source| source.finalized_use_kg_m2_stand_ground
                <= source.authorization_kg_m2_stand_ground.unwrap_or(0.0))
    );

    // The two-rank fixture exercises one shared canopy-air node and the
    // reciprocal longwave network across heterogeneous occupancies.
    let upper = &mut column.occupancies[0];
    upper.sun.absorbed_shortwave_w_m2_tile = 219.583_484_232_463_2;
    upper.sun.absorbed_par_w_m2_leaf = 136.097_574_782_013_34;
    upper.shade.absorbed_shortwave_w_m2_tile = 297.182_430_346_421_4;
    upper.shade.absorbed_par_w_m2_leaf = 116.714_147_486_897_5;
    upper.stem_absorbed_shortwave_w_m2_tile = 183.772_038_359_786_73;
    let mut lower = upper.clone();
    lower.occupancy_id = "canopy-rank-1".into();
    lower.sun.leaf_area_m2_m2_tile = 0.869_597_990_586_524_9;
    lower.sun.absorbed_shortwave_w_m2_tile = 36.606_943_691_269_41;
    lower.sun.absorbed_par_w_m2_leaf = 21.581_281_690_559_077;
    lower.shade.leaf_area_m2_m2_tile = 0.701_235_342_746_808_2;
    lower.shade.absorbed_shortwave_w_m2_tile = 29.370_268_258_774_185;
    lower.shade.absorbed_par_w_m2_leaf = 19.137_976_248_584_64;
    lower.stem_area_m2_m2_tile = 0.417_6;
    lower.stem_absorbed_shortwave_w_m2_tile = 23.961_096_147_421_54;
    lower.lai = 1.570_833_333_333_333;
    lower.sai = 0.417_6;
    lower.liquid_capacity_kg_m2_plant = 0.040_221_557_532_455_925;
    lower.clumping_index = 0.91;
    column.occupancies.push(lower);
    column.ground.terminal_shortwave_w_m2_tile = BandDirectionalFluxes {
        direct_vis: 12.572_362_927_904_654,
        diffuse_vis: 2.794_652_935_170_348_4,
        direct_nir: 10.885_826_437_575_982,
        diffuse_nir: 20.063_182_822_663_31,
    };
    column.shortwave = bound_shortwave(
        &column.occupancies,
        column.ground.terminal_shortwave_w_m2_tile,
        column.ground.surface_vis_albedo,
        column.ground.surface_nir_albedo,
    );
    for (occupancy, radiation) in column
        .occupancies
        .iter_mut()
        .zip(column.shortwave.occupancies.iter())
    {
        occupancy.sun.absorbed_shortwave_w_m2_tile = radiation.sun_leaf_absorbed_w_m2_tile.total();
        occupancy.shade.absorbed_shortwave_w_m2_tile =
            radiation.shade_leaf_absorbed_w_m2_tile.total();
        occupancy.stem_absorbed_shortwave_w_m2_tile = radiation.stem_absorbed_w_m2_tile.total();
    }
    let multirank_potential_trial = vec![
        -5_900.0, -5_450.0, -4_300.0, -2_850.0, 0.68, 0.66, 296.2, 295.4, 295.6, 295.2, -5_900.0,
        -5_450.0, -4_300.0, -2_850.0, 0.68, 0.66, 295.5, 295.0, 295.6, 295.2, 295.8, 0.011, 295.0,
        291.5, 289.8,
    ];
    let mut rain_column = column.clone();
    rain_column.top_rain_kg_m2_tile = 0.5;
    let rain = evaluate_covered_column(&rain_column, &multirank_potential_trial, None, None)
        .expect("rain routing evaluation");
    let upper_liquid = rain.occupancies[0].liquid;
    let lower_liquid = rain.occupancies[1].liquid;
    assert!(upper_liquid.throughfall_kg_m2_tile > 0.0);
    assert!(upper_liquid.initial_drainage_kg_m2_tile > 0.0);
    assert_eq!(
        lower_liquid.incident_rain_kg_m2_tile.to_bits(),
        (upper_liquid.throughfall_kg_m2_tile
            + upper_liquid.initial_drainage_kg_m2_tile
            + upper_liquid.second_drainage_kg_m2_tile)
            .to_bits()
    );
    assert_eq!(
        rain.ground_stemflow_kg_m2_tile.to_bits(),
        rain.occupancies
            .iter()
            .map(|value| value.liquid.stemflow_kg_m2_tile)
            .sum::<f64>()
            .to_bits()
    );
    for occupancy in &rain.occupancies {
        occupancy.liquid.validate().expect("rain liquid closure");
        assert_eq!(
            occupancy
                .liquid
                .wet_surface_specific_enthalpy_j_kg
                .to_bits(),
            (WATER_HEAT_CAPACITY_J_KG_K
                * (occupancy.liquid.wet_surface_temperature_k - REFERENCE_TEMPERATURE_K))
                .to_bits()
        );
    }

    let mut condensation_column = column.clone();
    condensation_column.occupancies[0].liquid_capacity_kg_m2_plant =
        0.018 / (condensation_column.occupancies[0].lai + condensation_column.occupancies[0].sai);
    let mut condensation_trial = multirank_potential_trial.clone();
    condensation_trial[8] = 280.0;
    let condensation =
        evaluate_covered_column(&condensation_column, &condensation_trial, None, None)
            .expect("condensation routing evaluation");
    assert!(condensation.occupancies[0].liquid.condensation_kg_m2_tile > 0.0);
    assert!(
        condensation.occupancies[0]
            .liquid
            .second_drainage_kg_m2_tile
            > 0.0
    );
    assert_eq!(
        condensation.occupancies[1]
            .liquid
            .incident_rain_kg_m2_tile
            .to_bits(),
        (condensation.occupancies[0].liquid.throughfall_kg_m2_tile
            + condensation.occupancies[0]
                .liquid
                .initial_drainage_kg_m2_tile
            + condensation.occupancies[0]
                .liquid
                .second_drainage_kg_m2_tile)
            .to_bits()
    );
    let multirank_potential =
        match solve_covered_column(&column, None, multirank_potential_trial.clone())
            .expect("multirank potential")
        {
            CoveredColumnSolveOutcome::Accepted(value) => value,
            CoveredColumnSolveOutcome::Rejected(failure) => {
                panic!("multirank potential rejected: {failure:?}")
            }
        };
    assert_eq!(multirank_potential.root_water.len(), 8);
    let mut multirank_root_caps = BTreeMap::new();
    for source in &multirank_potential.root_water {
        let request_rate =
            source.request_kg_m2_stand_ground / (column.tile_fraction * column.interval_s);
        let authorization_rate = if source.layer_id == "soil-1" {
            0.9 * request_rate
        } else {
            request_rate
        };
        multirank_root_caps.insert(
            (source.occupancy_id.clone(), source.layer_id.clone()),
            SourceWaterCap {
                request_rate_kg_m2_tile_s: request_rate,
                authorization_rate_kg_m2_tile_s: authorization_rate,
            },
        );
    }
    let multirank_caps = CoveredWaterCaps {
        root: multirank_root_caps,
        ground: SourceWaterCap {
            request_rate_kg_m2_tile_s: multirank_potential.ground_water.request_kg_m2_stand_ground
                / (column.tile_fraction * column.interval_s),
            authorization_rate_kg_m2_tile_s: 0.92
                * multirank_potential.ground_water.request_kg_m2_stand_ground
                / (column.tile_fraction * column.interval_s),
        },
    };
    let multirank_capped_trial = multirank_potential_trial.clone();
    let multirank = execute_covered_potential_final(
        &column,
        multirank_potential_trial,
        &multirank_caps,
        multirank_capped_trial,
    )
    .expect("multirank potential/final transaction");
    assert_eq!(multirank.final_pass.evaluation.occupancies.len(), 2);
    assert!(multirank.final_pass.root_water.iter().all(|source| {
        source.finalized_use_kg_m2_stand_ground
            <= source.authorization_kg_m2_stand_ground.unwrap_or(0.0)
    }));
}
