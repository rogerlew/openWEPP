//! Exact acceptance checks against the independently released V3 constitutive vectors.

use openwepp_kernel_contract::{OccupancyId, StratumId, TileId, TransactionId};
use serde_json::Value;

use super::constitutive::{
    ConstitutiveSolveContext, ENERGY_RESIDUAL_IDENTITIES, SolvedClass, V3AcceptedStageA,
    V3ConstitutiveEvaluator, V3PotentialCase, brent_dekker_class, solve_canopy_energy,
    solve_canopy_energy_with_limit, solve_linear,
};
use crate::VegetationError;
use crate::diagnostics::{CoupledSolvePass, NumericalFailureDiagnostics, SolveIdentity};
use crate::occupancy_solver::potential::{
    StageASolveIdentity, StageAState, solve_uncapped_stage_a_with_limit,
};

const VECTORS: &str = include_str!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/../../docs/work-packages/20260812-c3-woody-potential-pass-authority-001/artifacts/openwepp_c3_woody_v3_vectors.json"
));

const CI_TOLERANCE_PA: f64 = 1.0e-6;
const TEMPERATURE_TOLERANCE_K: f64 = 1.0e-8;
const POTENTIAL_TOLERANCE_MM: f64 = 1.0e-7;
const WATER_ATOL: f64 = 1.0e-12;
const WATER_RTOL: f64 = 1.0e-9;
const OUTER_RESIDUAL_IDENTITIES: [&str; 6] = [
    "sun_gas_minus_q1",
    "shade_gas_minus_q1",
    "sun_gas_minus_vulnerability_demand",
    "shade_gas_minus_vulnerability_demand",
    "q1_sum_minus_q2",
    "q3_sum_minus_q2",
];
fn vectors() -> Value {
    serde_json::from_str(VECTORS).expect("released V3 fixture parses")
}

fn fixture() -> (V3PotentialCase, Value) {
    let root = vectors();
    let family = root["families"]["hydraulic_potential_pass"].clone();
    let mut runtime = family["operands"].clone();
    let runtime_object = runtime.as_object_mut().expect("potential operands object");
    runtime_object.remove("emax");
    runtime_object.insert(
        "surface_dimensions".into(),
        serde_json::json!({
            "leaf_m": root["families"]["aerodynamics"]["operands"]["leaf_dimension_m"],
            "wet_surface_m": root["families"]["aerodynamics"]["operands"]["wet_surface_dimension_m"],
            "stem_m": root["families"]["aerodynamics"]["operands"]["stem_dimension_m"]
        }),
    );
    let parameters = runtime["parameters"]
        .as_object_mut()
        .expect("hydraulic parameters object");
    let k1 = parameters.remove("k1_max").expect("fixture k1 maximum");
    for duplicate in [
        "stem_to_leaf_path_m",
        "sun_leaf_area",
        "shade_leaf_area",
        "sai",
        "lai",
    ] {
        parameters.remove(duplicate);
    }
    parameters.insert("k1a_max_s1".into(), k1.clone());
    parameters.insert("k1b_max_s1".into(), k1);
    let biochemistry = runtime["biochemical_parameters"]
        .as_object_mut()
        .expect("biochemistry object");
    biochemistry.remove("oxygen_partial_pressure_pa");
    biochemistry.insert("oxygen_mole_fraction".into(), serde_json::json!(0.20));
    (
        serde_json::from_value(runtime).expect("exact V3 potential operands deserialize"),
        family,
    )
}

fn identity() -> StageASolveIdentity {
    StageASolveIdentity {
        transaction_id: TransactionId(17),
        occupancy_id: OccupancyId {
            stratum_id: StratumId::try_new("upper").expect("stratum"),
            tile_id: TileId::try_new("tile-a").expect("tile"),
        },
    }
}

fn context() -> ConstitutiveSolveContext {
    let identity = identity();
    ConstitutiveSolveContext {
        transaction_id: identity.transaction_id,
        occupancy_id: identity.occupancy_id,
        pass: CoupledSolvePass::Potential,
    }
}

fn state(values: &[f64]) -> StageAState {
    StageAState {
        psi_sunleaf_mm: values[0],
        psi_shadeleaf_mm: values[1],
        psi_stem_mm: values[2],
        psi_root_mm: values[3],
        beta_sun: values[4],
        beta_shade: values[5],
    }
}

fn number(value: &Value, field: &str) -> f64 {
    value[field]
        .as_f64()
        .unwrap_or_else(|| panic!("{field} is numeric"))
}

fn count(value: &Value, field: &str) -> u32 {
    u32::try_from(
        value[field]
            .as_u64()
            .unwrap_or_else(|| panic!("{field} is unsigned")),
    )
    .expect("fixture count fits u32")
}

fn close(actual: f64, expected: f64, atol: f64, rtol: f64) {
    assert!(actual.is_finite(), "actual is finite");
    assert!(
        (actual - expected).abs() <= atol + rtol * expected.abs(),
        "actual={actual:.17e}, expected={expected:.17e}, atol={atol:.1e}, rtol={rtol:.1e}"
    );
}

fn assert_failure_common(
    diagnostics: &NumericalFailureDiagnostics,
    expected: &Value,
    solve: SolveIdentity,
) {
    assert_eq!(diagnostics.model_definition_sha256, crate::MODEL_SHA256);
    assert_eq!(diagnostics.transaction_id, TransactionId(17));
    assert_eq!(diagnostics.occupancy_id, identity().occupancy_id);
    assert_eq!(diagnostics.pass, CoupledSolvePass::Potential);
    assert_eq!(diagnostics.solve, solve);
    assert_eq!(
        diagnostics.iterations,
        u32::try_from(expected["iterations"].as_u64().expect("iterations")).expect("u32")
    );
    assert_eq!(diagnostics.backtracking_count, 0);
    assert!(diagnostics.active_bounds.is_empty());
    assert!(diagnostics.active_water_caps.is_empty());
    assert_eq!(diagnostics.step_norm, None);
    assert_eq!(diagnostics.pivot_magnitude, None);
    assert_eq!(diagnostics.matrix_norm, None);
    diagnostics
        .validate()
        .expect("canonical finite diagnostics");
}

#[test]
fn beta_one_energy_diagnostics_match_exact_discrete_fixture() {
    let (case, family) = fixture();
    let actual = solve_canopy_energy(&case, (1.0, 1.0), (-5_900.0, -5_450.0), &context())
        .expect("beta-one canopy solve");
    let expected = &family["internal_maximum_evaluation"];
    let canopy = &expected["canopy_energy_state"];

    assert_eq!(actual.iterations, count(canopy, "iterations"));
    assert_eq!(
        actual.backtracking_count,
        count(canopy, "backtracking_count")
    );
    assert_eq!(
        actual.sun.ci_iterations,
        count(&expected["sun_gas_energy_state"], "ci_iterations")
    );
    assert_eq!(
        actual.shade.ci_iterations,
        count(&expected["shade_gas_energy_state"], "ci_iterations")
    );
    assert_eq!(
        actual.wet_store_cap_active,
        canopy["wet_store_cap_active"].as_bool().expect("cap")
    );
    assert!(actual.temperature_step_k.is_some());
    assert!(actual.temperature_step_k.expect("step") <= TEMPERATURE_TOLERANCE_K);

    for (actual_class, expected_class) in [
        (&actual.sun, &expected["sun_gas_energy_state"]),
        (&actual.shade, &expected["shade_gas_energy_state"]),
    ] {
        close(
            actual_class.ci_pa,
            number(expected_class, "ci_pa"),
            CI_TOLERANCE_PA,
            0.0,
        );
        let expected_bracket = expected_class["ci_bracket_pa"]
            .as_array()
            .expect("ci bracket");
        close(
            actual_class.ci_bracket_pa.0,
            expected_bracket[0].as_f64().expect("low"),
            CI_TOLERANCE_PA,
            0.0,
        );
        close(
            actual_class.ci_bracket_pa.1,
            expected_bracket[1].as_f64().expect("high"),
            CI_TOLERANCE_PA,
            0.0,
        );
        close(
            actual_class.leaf_temperature_k,
            number(expected_class, "leaf_temperature_k"),
            TEMPERATURE_TOLERANCE_K,
            0.0,
        );
        close(
            actual_class.transpiration_kg_m2_tile_s,
            number(expected_class, "transpiration_kg_m2_tile_s"),
            WATER_ATOL,
            WATER_RTOL,
        );
    }
    close(
        actual.canopy_air_temperature_k,
        number(canopy, "canopy_air_temperature_k"),
        TEMPERATURE_TOLERANCE_K,
        0.0,
    );
    close(
        actual.wet_surface_temperature_k,
        number(canopy, "wet_surface_temperature_k"),
        TEMPERATURE_TOLERANCE_K,
        0.0,
    );
    close(
        actual.dry_stem_temperature_k,
        number(canopy, "dry_stem_temperature_k"),
        TEMPERATURE_TOLERANCE_K,
        0.0,
    );
    assert_eq!(actual.normalized_residuals.len(), 6);
    for (index, residual) in actual.normalized_residuals.iter().enumerate() {
        assert_eq!(residual.identity, ENERGY_RESIDUAL_IDENTITIES[index]);
        assert!(residual.value.is_finite());
        assert!(residual.value.abs() <= 1.0);
        assert!(
            canopy["normalized_residuals"][index]
                .as_f64()
                .expect("released residual")
                .abs()
                <= 1.0
        );
    }
}

#[test]
fn accepted_and_alternate_outer_diagnostics_match_fixture() {
    let (case, family) = fixture();
    let evaluator = V3ConstitutiveEvaluator::new(case, (-5_900.0, -5_450.0), context())
        .expect("constitutive evaluator");
    let cases = [
        (
            state(&[-5_900.0, -5_450.0, -4_300.0, -2_850.0, 0.68, 0.66]),
            &family["accepted_uncapped_stage_a"],
        ),
        (
            state(
                &family["alternate_warm_start"]["start"]
                    .as_array()
                    .expect("start")
                    .iter()
                    .map(|v| v.as_f64().expect("number"))
                    .collect::<Vec<_>>(),
            ),
            &family["alternate_warm_start"]["result"],
        ),
    ];
    for (start, expected) in cases {
        let accepted = evaluator
            .solve_uncapped(&identity(), start)
            .expect("accepted Stage-A");
        assert_outer_diagnostics(&accepted, expected);
        assert_nested_diagnostics(&accepted, expected);
    }
}

fn assert_outer_diagnostics(accepted: &V3AcceptedStageA, expected: &Value) {
    let solution = &expected["solution"];
    for (actual, field) in [
        (accepted.outer.state.psi_sunleaf_mm, "sun_leaf_potential_mm"),
        (
            accepted.outer.state.psi_shadeleaf_mm,
            "shade_leaf_potential_mm",
        ),
        (accepted.outer.state.psi_stem_mm, "stem_potential_mm"),
        (accepted.outer.state.psi_root_mm, "root_node_potential_mm"),
    ] {
        close(actual, number(solution, field), POTENTIAL_TOLERANCE_MM, 0.0);
    }
    close(
        accepted.outer.state.beta_sun,
        number(solution, "beta_hyd_sun"),
        1.0e-9,
        0.0,
    );
    close(
        accepted.outer.state.beta_shade,
        number(solution, "beta_hyd_shade"),
        1.0e-9,
        0.0,
    );
    close(
        accepted.outer.persisted_beta_hyd,
        number(solution, "beta_hyd"),
        1.0e-9,
        0.0,
    );
    assert_eq!(accepted.outer.iterations, count(expected, "iterations"));
    assert_eq!(
        accepted.outer.backtracking_count,
        count(expected, "backtracking_count")
    );
    assert!(accepted.outer.potential_step_mm.is_finite());
    assert!(accepted.outer.potential_step_mm <= POTENTIAL_TOLERANCE_MM);
    close(
        accepted.outer.potential_step_mm,
        number(expected, "potential_step_mm"),
        POTENTIAL_TOLERANCE_MM,
        0.0,
    );
    let expected_residuals = expected["normalized_residuals"]
        .as_array()
        .expect("residuals");
    let identities = [
        "sun_gas_minus_q1",
        "shade_gas_minus_q1",
        "sun_gas_minus_vulnerability_demand",
        "shade_gas_minus_vulnerability_demand",
        "q1_sum_minus_q2",
        "q3_sum_minus_q2",
    ];
    assert_eq!(
        accepted.outer.normalized_residuals.len(),
        expected_residuals.len()
    );
    for (index, (actual, expected)) in accepted
        .outer
        .normalized_residuals
        .iter()
        .zip(expected_residuals)
        .enumerate()
    {
        assert_eq!(
            actual.identity,
            expected["identity"].as_str().expect("identity")
        );
        assert_eq!(actual.identity, identities[index]);
        assert!(actual.value.is_finite());
        assert!(actual.value.abs() <= 1.0);
        assert!(number(expected, "normalized").abs() <= 1.0);
    }
}

fn assert_nested_diagnostics(accepted: &V3AcceptedStageA, expected: &Value) {
    let canopy = &expected["fluxes"]["canopy_energy_state"];
    assert_eq!(accepted.canopy.iterations, count(canopy, "iterations"));
    assert_eq!(
        accepted.canopy.backtracking_count,
        count(canopy, "backtracking_count")
    );
    assert!(
        accepted
            .canopy
            .temperature_step_k
            .is_some_and(|step| step.is_finite() && step <= TEMPERATURE_TOLERANCE_K)
    );
    for (actual_class, expected_class) in [
        (
            &accepted.canopy.sun,
            &expected["fluxes"]["sun_gas_energy_state"],
        ),
        (
            &accepted.canopy.shade,
            &expected["fluxes"]["shade_gas_energy_state"],
        ),
    ] {
        assert_eq!(
            actual_class.ci_iterations,
            count(expected_class, "ci_iterations")
        );
        close(
            actual_class.ci_pa,
            number(expected_class, "ci_pa"),
            CI_TOLERANCE_PA,
            0.0,
        );
        let bracket = expected_class["ci_bracket_pa"].as_array().expect("bracket");
        close(
            actual_class.ci_bracket_pa.0,
            bracket[0].as_f64().expect("low"),
            CI_TOLERANCE_PA,
            0.0,
        );
        close(
            actual_class.ci_bracket_pa.1,
            bracket[1].as_f64().expect("high"),
            CI_TOLERANCE_PA,
            0.0,
        );
        close(
            actual_class.leaf_temperature_k,
            number(expected_class, "leaf_temperature_k"),
            TEMPERATURE_TOLERANCE_K,
            0.0,
        );
    }
}

#[test]
fn all_released_ci_failures_have_exact_payload_shape() {
    let root = vectors();
    let fixtures = root["families"]["executed_ci_failures"]
        .as_array()
        .expect("ci failures");
    assert_eq!(fixtures.len(), 6);
    for (index, fixture) in fixtures.iter().enumerate() {
        let expected = &fixture["diagnostics"];
        let solve = if index < 3 {
            SolveIdentity::SunCi
        } else {
            SolveIdentity::ShadeCi
        };
        let mode = index % 3;
        let (low, high) = if mode == 1 {
            (-1.0, 1.0)
        } else if mode == 2 {
            (0.0, 2.0)
        } else {
            (0.0, 1.0)
        };
        let result = brent_dekker_class(
            |value| {
                let residual = match mode {
                    0 => f64::NAN,
                    1 => value.mul_add(value, 1.0),
                    _ => 2.0 * (value - 1.0),
                };
                Ok((
                    residual,
                    SolvedClass {
                        ci_pa: value,
                        rs_s_m: 1.0,
                        iterations: 0,
                        bracket: (low, high),
                    },
                ))
            },
            low,
            high,
            2,
            &context(),
            solve,
        )
        .expect_err("released CI failure must fail");
        let VegetationError::NumericalFailure {
            diagnostics: actual,
            ..
        } = result
        else {
            panic!("typed CI diagnostics");
        };
        assert_failure_common(&actual, expected, solve);
        let expected_bracket = expected["bracket"].as_array().expect("bracket");
        assert_eq!(
            actual.bracket,
            Some((
                expected_bracket[0].as_f64().expect("low"),
                expected_bracket[1].as_f64().expect("high")
            ))
        );
        let expected_residuals = expected["residual_norms"].as_array().expect("residuals");
        assert_eq!(actual.residual_norms.len(), expected_residuals.len());
        for (actual, expected) in actual.residual_norms.iter().zip(expected_residuals) {
            assert_eq!(
                actual.identity,
                expected["identity"].as_str().expect("identity")
            );
            close(actual.value, number(expected, "normalized"), 0.0, 0.0);
        }
    }
}

#[test]
fn released_canopy_iteration_limit_and_singular_linear_payloads_are_exact() {
    let (case, family) = fixture();
    let failure =
        solve_canopy_energy_with_limit(&case, (0.6, 0.6), (-5_900.0, -5_450.0), &context(), 0)
            .expect_err("zero iteration cap");
    let VegetationError::NumericalFailure {
        diagnostics: actual,
        ..
    } = failure
    else {
        panic!("typed canopy failure");
    };
    let expected = &family["executed_canopy_energy_failures"][1]["diagnostics"];
    assert_failure_common(&actual, expected, SolveIdentity::CanopyEnergy);
    assert_eq!(actual.bracket, None);
    assert_eq!(actual.residual_norms.len(), 6);
    for (index, (actual, expected)) in actual
        .residual_norms
        .iter()
        .zip(expected["residual_norms"].as_array().expect("residuals"))
        .enumerate()
    {
        assert_eq!(actual.identity, ENERGY_RESIDUAL_IDENTITIES[index]);
        assert_eq!(
            actual.identity,
            expected["identity"].as_str().expect("identity")
        );
        let expected_value = number(expected, "normalized");
        assert_eq!(
            actual.value.is_sign_negative(),
            expected_value.is_sign_negative()
        );
        close(actual.value / expected_value, 1.0, 2.0e-12, 2.0e-12);
    }

    let zero_matrix = solve_linear([[0.0; 6]; 6], [0.0; 6]).expect_err("zero matrix");
    assert_eq!(zero_matrix.pivot_magnitude.to_bits(), 0.0_f64.to_bits());
    assert_eq!(zero_matrix.matrix_norm.to_bits(), 0.0_f64.to_bits());
}

#[test]
fn released_nonzero_singular_and_outer_iteration_limit_execute_exact_paths() {
    let (case, family) = fixture();
    let initial = state(&[-5_900.0, -5_450.0, -4_300.0, -2_850.0, 0.68, 0.66]);
    let singular = V3ConstitutiveEvaluator::new(case.clone(), (-5_900.0, -5_450.0), context())
        .expect("valid base evaluator")
        .with_released_singular_hydraulics();
    let failure = solve_uncapped_stage_a_with_limit(&identity(), initial, &singular, 50)
        .expect_err("released nonzero singular Jacobian must fail");
    let VegetationError::NumericalFailure {
        diagnostics: actual,
        ..
    } = failure
    else {
        panic!("typed singular diagnostics");
    };
    let expected = &family["singular_jacobian"]["diagnostics"];
    assert_outer_failure_payload(&actual, expected, SolveIdentity::HydraulicSystem);
    assert_eq!(
        actual.pivot_magnitude,
        Some(number(expected, "pivot_magnitude"))
    );
    close(
        actual.matrix_norm.expect("matrix norm"),
        number(expected, "matrix_norm"),
        2.0e-11,
        5.0e-7,
    );

    let alternate = state(
        &family["alternate_warm_start"]["start"]
            .as_array()
            .expect("alternate start")
            .iter()
            .map(|value| value.as_f64().expect("number"))
            .collect::<Vec<_>>(),
    );
    let evaluator = V3ConstitutiveEvaluator::new(case, (-5_900.0, -5_450.0), context())
        .expect("constitutive evaluator");
    let failure = solve_uncapped_stage_a_with_limit(&identity(), alternate, &evaluator, 1)
        .expect_err("released outer iteration cap must fail");
    let VegetationError::NumericalFailure {
        diagnostics: actual,
        ..
    } = failure
    else {
        panic!("typed outer diagnostics");
    };
    let expected = &family["iteration_limit"]["diagnostics"];
    assert_outer_failure_payload(
        &actual,
        expected,
        SolveIdentity::OuterGasEnergyHydraulicCoupling,
    );
    close(
        actual.step_norm.expect("last potential step"),
        number(expected, "step_norm"),
        1.0e-6,
        1.0e-6,
    );
    close(
        actual.pivot_magnitude.expect("pivot"),
        number(expected, "pivot_magnitude"),
        2.0e-14,
        1.0e-6,
    );
    close(
        actual.matrix_norm.expect("matrix norm"),
        number(expected, "matrix_norm"),
        2.0e-12,
        1.0e-6,
    );
}

fn assert_outer_failure_payload(
    actual: &NumericalFailureDiagnostics,
    expected: &Value,
    solve: SolveIdentity,
) {
    assert_eq!(actual.model_definition_sha256, crate::MODEL_SHA256);
    assert_eq!(actual.transaction_id, TransactionId(17));
    assert_eq!(actual.occupancy_id, identity().occupancy_id);
    assert_eq!(actual.pass, CoupledSolvePass::Potential);
    assert_eq!(actual.solve, solve);
    assert_eq!(actual.iterations, count(expected, "iterations"));
    assert_eq!(
        actual.backtracking_count,
        count(expected, "backtracking_count")
    );
    assert_eq!(actual.bracket, None);
    assert!(actual.active_bounds.is_empty());
    assert!(actual.active_water_caps.is_empty());
    let expected_residuals = expected["residual_norms"].as_array().expect("residuals");
    assert_eq!(actual.residual_norms.len(), expected_residuals.len());
    for (index, (actual, expected)) in actual
        .residual_norms
        .iter()
        .zip(expected_residuals)
        .enumerate()
    {
        let expected_identity = expected["identity"].as_str().expect("identity");
        if solve == SolveIdentity::OuterGasEnergyHydraulicCoupling {
            assert_eq!(expected_identity, format!("outer_{index}"));
            assert_eq!(actual.identity, OUTER_RESIDUAL_IDENTITIES[index]);
        } else {
            assert_eq!(actual.identity, expected_identity);
        }
        let expected_value = number(expected, "normalized");
        if solve == SolveIdentity::OuterGasEnergyHydraulicCoupling {
            // Failure residuals are finite-difference trajectory diagnostics,
            // not accepted endpoints. Preserve exact identity/order and use a
            // narrow cross-language arithmetic allowance; accepted states use
            // the canonical physical tolerances above.
            close(actual.value.abs(), expected_value.abs(), 2.0e-5, 1.0e-6);
        } else {
            close(actual.value, expected_value, 2.0e-5, 2.0e-9);
        }
    }
    actual.validate().expect("finite released failure payload");
}

#[test]
fn released_canopy_domain_case_executes_without_candidate() {
    let (mut case, family) = fixture();
    case.gas_energy.canopy_air_temperature_start_k = 250.0;
    let failure =
        solve_canopy_energy_with_limit(&case, (0.6, 0.6), (-5_900.0, -5_450.0), &context(), 50)
            .expect_err("released canopy domain case");
    let VegetationError::NumericalFailure {
        diagnostics: actual,
        ..
    } = failure
    else {
        panic!("typed canopy domain failure required");
    };
    assert_eq!(actual.solve, SolveIdentity::CanopyEnergy);
    assert_eq!(actual.iterations, 0);
    assert!(actual.residual_norms.is_empty());
    let released = &family["executed_canopy_energy_failures"][0];
    assert_eq!(released["failure_kind"].as_str(), Some("domain"));
    assert_eq!(released["candidate"], Value::Null);
    assert_eq!(released["last_iterate"], Value::Null);
    assert_eq!(released["diagnostics"]["iterations"].as_u64(), Some(0));
    assert_eq!(
        released["diagnostics"]["residual_norms"]
            .as_array()
            .expect("empty pre-solver residuals")
            .len(),
        0
    );
}
