//! Exact V5 authorization-capped E11--E15 fixture acceptance.

use std::collections::BTreeMap;

use openwepp_kernel_contract::{OccupancyId, SoilLayerId, StratumId, TileId, TransactionId};
use serde_json::Value;

use super::constitutive::{
    ConstitutiveSolveContext, V3ConstitutiveEvaluator, V3PotentialCase, select_capped_flux,
};
use super::potential::{StageAEvaluator, StageASolveIdentity, StageAState};
use crate::VegetationError;
use crate::diagnostics::CoupledSolvePass;

const VECTORS: &str = include_str!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/../../docs/work-packages/20260812-c3-woody-potential-pass-authority-001/artifacts/openwepp_c3_woody_v5_vectors.json"
));

fn root() -> Value {
    serde_json::from_str(VECTORS).expect("released V5 fixture parses")
}

fn fixture() -> (V3PotentialCase, Value) {
    let root = root();
    let family = root["families"]["complete_coupled_capped_solve"].clone();
    let mut runtime = family["operands"].clone();
    let object = runtime.as_object_mut().expect("capped operands object");
    object.remove("emax");
    object.insert(
        "surface_dimensions".into(),
        serde_json::json!({"leaf_m": 0.045, "wet_surface_m": 0.16, "stem_m": 0.34}),
    );
    let parameters = runtime["parameters"]
        .as_object_mut()
        .expect("hydraulic parameters");
    let k1 = parameters.remove("k1_max").expect("k1 maximum");
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
        .expect("biochemistry");
    biochemistry.remove("oxygen_partial_pressure_pa");
    biochemistry.insert("oxygen_mole_fraction".into(), serde_json::json!(0.20));
    (
        serde_json::from_value(runtime).expect("V5 operands deserialize"),
        family,
    )
}

fn identity() -> StageASolveIdentity {
    StageASolveIdentity {
        transaction_id: TransactionId(53),
        occupancy_id: OccupancyId {
            stratum_id: StratumId::try_new("canopy").expect("stratum"),
            tile_id: TileId::try_new("tile-a").expect("tile"),
        },
    }
}

fn context() -> ConstitutiveSolveContext {
    let identity = identity();
    ConstitutiveSolveContext {
        transaction_id: identity.transaction_id,
        occupancy_id: identity.occupancy_id,
        pass: CoupledSolvePass::Capped,
    }
}

fn state(values: [f64; 6]) -> StageAState {
    StageAState {
        psi_sunleaf_mm: values[0],
        psi_shadeleaf_mm: values[1],
        psi_stem_mm: values[2],
        psi_root_mm: values[3],
        beta_sun: values[4],
        beta_shade: values[5],
    }
}

fn caps(expected: &Value) -> BTreeMap<SoilLayerId, f64> {
    expected["fluxes"]["q3"]
        .as_array()
        .expect("q3 array")
        .iter()
        .map(|layer| {
            (
                SoilLayerId::try_new(layer["layer_id"].as_str().expect("layer id"))
                    .expect("typed layer"),
                layer["cap_rate_kg_m2_tile_s"].as_f64().expect("cap rate"),
            )
        })
        .collect()
}

fn close(actual: f64, expected: f64, atol: f64, rtol: f64) {
    assert!(
        (actual - expected).abs() <= atol + rtol * expected.abs(),
        "actual={actual:.17e}, expected={expected:.17e}"
    );
}

#[test]
fn constrained_all_cap_matches_released_v5_endpoint_and_operands() {
    let (case, family) = fixture();
    let expected = &family["accepted_constrained_all_cap"];
    let evaluator =
        V3ConstitutiveEvaluator::new_capped(case, (-5_900.0, -5_450.0), context(), caps(expected))
            .expect("capped evaluator");
    let accepted = evaluator
        .solve_capped(
            &identity(),
            state([-5_900.0, -5_450.0, -4_300.0, -2_850.0, 0.68, 0.66]),
        )
        .expect("capped coupled solve");
    let solution = &expected["solution"];
    for (actual, field) in [
        (accepted.outer.state.psi_sunleaf_mm, "sun_leaf_potential_mm"),
        (
            accepted.outer.state.psi_shadeleaf_mm,
            "shade_leaf_potential_mm",
        ),
        (accepted.outer.state.psi_stem_mm, "stem_potential_mm"),
        (accepted.outer.state.psi_root_mm, "root_node_potential_mm"),
        (accepted.outer.state.beta_sun, "beta_hyd_sun"),
        (accepted.outer.state.beta_shade, "beta_hyd_shade"),
    ] {
        close(
            actual,
            solution[field].as_f64().expect("solution value"),
            1.0e-7,
            1.0e-10,
        );
    }
    assert_eq!(
        accepted.outer.iterations,
        u32::try_from(expected["iterations"].as_u64().expect("iterations"))
            .expect("iterations fit u32")
    );
    assert_eq!(
        accepted.outer.evaluation.active_water_caps,
        expected["active_water_caps"]
            .as_array()
            .expect("active caps")
            .iter()
            .map(|value| SoilLayerId::try_new(value.as_str().expect("layer")).expect("typed layer"))
            .collect::<Vec<_>>()
    );
    for (actual, fixture) in accepted
        .outer
        .evaluation
        .capped_layer_fluxes
        .iter()
        .zip(expected["fluxes"]["q3"].as_array().expect("q3"))
    {
        assert_eq!(
            actual.layer_id.as_str(),
            fixture["layer_id"].as_str().expect("layer id")
        );
        close(
            actual.q_law_kg_m2_s,
            fixture["q_law_kg_m2_tile_s"].as_f64().expect("q law"),
            1.0e-12,
            1.0e-9,
        );
        close(
            actual.cap_rate_kg_m2_s,
            fixture["cap_rate_kg_m2_tile_s"].as_f64().expect("cap"),
            0.0,
            0.0,
        );
        close(
            actual.q_final_kg_m2_s,
            fixture["q_final_kg_m2_tile_s"].as_f64().expect("q final"),
            1.0e-12,
            1.0e-9,
        );
        assert_eq!(
            actual.authorization_active_or_tie,
            fixture["branch"] == "authorization_active_or_tie"
        );
    }
}

#[test]
fn fully_authorized_reduces_to_v3_value_with_exact_recomputed_branches() {
    let (case, family) = fixture();
    let expected = &family["fully_authorized_value_reduction"]["capped_result"];
    let evaluator =
        V3ConstitutiveEvaluator::new_capped(case, (-5_900.0, -5_450.0), context(), caps(expected))
            .expect("fully authorized evaluator");
    let accepted = evaluator
        .solve_capped(
            &identity(),
            state([-5_900.0, -5_450.0, -4_300.0, -2_850.0, 0.68, 0.66]),
        )
        .expect("fully authorized solve");
    let solution = &expected["solution"];
    close(
        accepted.outer.state.psi_root_mm,
        solution["root_node_potential_mm"].as_f64().expect("root"),
        1.0e-7,
        0.0,
    );
    close(
        accepted.outer.persisted_beta_hyd,
        solution["beta_hyd"].as_f64().expect("beta"),
        1.0e-9,
        0.0,
    );
    assert_eq!(
        accepted.outer.evaluation.active_water_caps,
        ["soil-dry", "soil-frozen", "soil-zero-root"]
            .into_iter()
            .map(|value| SoilLayerId::try_new(value).expect("layer"))
            .collect::<Vec<_>>()
    );
}

#[test]
fn exact_tie_is_cap_active_and_finite_difference_pair_freezes_that_branch() {
    let (case, family) = fixture();
    let tie = &family["executed_coupled_exact_tie_jacobian"];
    let accepted = &family["fully_authorized_value_reduction"]["capped_result"];
    let mut cap_rates = caps(accepted);
    let soil_1 = SoilLayerId::try_new("soil-1").expect("layer");
    cap_rates.insert(
        soil_1.clone(),
        tie["unperturbed"]["cap_rate_kg_m2_tile_s"]
            .as_f64()
            .expect("tie cap"),
    );
    let evaluator =
        V3ConstitutiveEvaluator::new_capped(case, (-5_900.0, -5_450.0), context(), cap_rates)
            .expect("tie evaluator");
    let solution = &accepted["solution"];
    let base_state = StageAState {
        psi_sunleaf_mm: solution["sun_leaf_potential_mm"].as_f64().expect("sun"),
        psi_shadeleaf_mm: solution["shade_leaf_potential_mm"].as_f64().expect("shade"),
        psi_stem_mm: solution["stem_potential_mm"].as_f64().expect("stem"),
        psi_root_mm: solution["root_node_potential_mm"].as_f64().expect("root"),
        beta_sun: solution["beta_hyd_sun"].as_f64().expect("sun beta"),
        beta_shade: solution["beta_hyd_shade"].as_f64().expect("shade beta"),
    };
    let base = evaluator.evaluate(base_state).expect("tie base evaluation");
    let layer = base
        .capped_layer_fluxes
        .iter()
        .find(|layer| layer.layer_id == soil_1)
        .expect("tie layer");
    assert!(layer.authorization_active_or_tie);
    assert_eq!(
        layer.q_law_kg_m2_s.to_bits(),
        layer.cap_rate_kg_m2_s.to_bits()
    );

    let step = tie["root_step_mm"].as_f64().expect("root step");
    for (root, expected_key) in [
        (base_state.psi_root_mm + step, "frozen_plus"),
        (base_state.psi_root_mm - step, "frozen_minus"),
    ] {
        let mut trial = base_state;
        trial.psi_root_mm = root;
        let perturbed = evaluator
            .evaluate_jacobian_perturbation(trial, &base)
            .expect("frozen branch perturbation");
        let actual = perturbed
            .capped_layer_fluxes
            .iter()
            .find(|layer| layer.layer_id == soil_1)
            .expect("perturbed tie layer");
        assert!(actual.authorization_active_or_tie);
        assert_eq!(
            actual.q_final_kg_m2_s.to_bits(),
            layer.cap_rate_kg_m2_s.to_bits()
        );
        close(
            actual.q_law_kg_m2_s,
            tie[expected_key]["q_law_kg_m2_tile_s"]
                .as_f64()
                .expect("q law"),
            1.0e-15,
            1.0e-10,
        );
    }
}

#[test]
fn alternate_warm_start_converges_to_the_same_capped_endpoint() {
    let (case, family) = fixture();
    let expected = &family["accepted_constrained_all_cap"];
    let alternate = &family["alternate_warm_start"];
    let evaluator =
        V3ConstitutiveEvaluator::new_capped(case, (-5_900.0, -5_450.0), context(), caps(expected))
            .expect("capped evaluator");
    let start = alternate["start"]
        .as_array()
        .expect("alternate start")
        .iter()
        .map(|value| value.as_f64().expect("start number"))
        .collect::<Vec<_>>();
    let accepted = evaluator
        .solve_capped(
            &identity(),
            state([start[0], start[1], start[2], start[3], start[4], start[5]]),
        )
        .expect("alternate capped solve");
    let solution = &expected["solution"];
    close(
        accepted.outer.state.psi_root_mm,
        solution["root_node_potential_mm"].as_f64().expect("root"),
        1.0e-7,
        0.0,
    );
    close(
        accepted.outer.persisted_beta_hyd,
        solution["beta_hyd"].as_f64().expect("beta"),
        1.0e-9,
        0.0,
    );
}

#[test]
fn zero_iteration_failure_is_capped_typed_and_leaves_inputs_unchanged() {
    let (case, family) = fixture();
    let expected = &family["accepted_constrained_all_cap"];
    let evaluator =
        V3ConstitutiveEvaluator::new_capped(case, (-5_900.0, -5_450.0), context(), caps(expected))
            .expect("capped evaluator");
    let beginning = state([-5_900.0, -5_450.0, -4_300.0, -2_850.0, 0.68, 0.66]);
    let error = evaluator
        .solve_capped_with_limit(&identity(), beginning, 0)
        .expect_err("zero iteration limit rejects");
    let VegetationError::NumericalFailure(diagnostics) = error else {
        panic!("typed capped numerical failure required");
    };
    assert_eq!(diagnostics.pass, CoupledSolvePass::Capped);
    assert_eq!(diagnostics.iterations, 0);
    assert_eq!(diagnostics.backtracking_count, 0);
    assert_eq!(
        diagnostics.active_water_caps,
        ["soil-1", "soil-dry", "soil-frozen", "soil-zero-root"]
            .into_iter()
            .map(|value| SoilLayerId::try_new(value).expect("layer"))
            .collect::<Vec<_>>()
    );
    let operands = diagnostics
        .capped_operands
        .as_ref()
        .expect("failed capped iterate retains complete layer operands");
    assert_eq!(operands.layers.len(), 5);
    assert!(operands.water_residual_scale_kg_m2_tile_s.is_finite());
    assert_eq!(
        operands
            .layers
            .iter()
            .map(|layer| layer.layer_id.as_str())
            .collect::<Vec<_>>(),
        [
            "soil-1",
            "soil-2",
            "soil-dry",
            "soil-frozen",
            "soil-zero-root"
        ]
    );
    assert_eq!(
        operands
            .layers
            .iter()
            .filter(|layer| layer.authorization_active_or_tie)
            .map(|layer| layer.layer_id.clone())
            .collect::<Vec<_>>(),
        diagnostics.active_water_caps
    );
    assert_eq!(
        beginning,
        state([-5_900.0, -5_450.0, -4_300.0, -2_850.0, 0.68, 0.66])
    );
}

#[test]
fn exact_and_near_tie_bits_follow_the_released_v5_branch_rule() {
    let root = root();
    for case in root["families"]["exact_and_near_tie"]["cases"]
        .as_array()
        .expect("near-tie cases")
    {
        let q_law = case["q_law_kg_m2_tile_s"].as_f64().expect("q law");
        let cap = case["cap_rate_kg_m2_tile_s"].as_f64().expect("cap");
        let (selected, active) = select_capped_flux(q_law, cap, None);
        assert_eq!(
            active,
            case["branch"] == "authorization_active_or_tie",
            "{}",
            case["case"].as_str().expect("case")
        );
        assert_eq!(
            selected.to_bits(),
            case["q_final_kg_m2_tile_s"]
                .as_f64()
                .expect("final")
                .to_bits()
        );
    }
}

#[test]
fn capped_singular_failure_retains_configured_layer_operands_and_rolls_back() {
    let (case, family) = fixture();
    let expected = &family["accepted_constrained_all_cap"];
    let evaluator =
        V3ConstitutiveEvaluator::new_capped(case, (-5_900.0, -5_450.0), context(), caps(expected))
            .expect("capped evaluator")
            .with_released_singular_hydraulics();
    let beginning = state([-5_900.0, -5_450.0, -4_300.0, -2_850.0, 0.68, 0.66]);
    let error = evaluator
        .solve_capped(&identity(), beginning)
        .expect_err("singular capped hydraulics reject");
    let VegetationError::NumericalFailure(diagnostics) = error else {
        panic!("typed numerical failure required");
    };
    assert_eq!(diagnostics.pass, CoupledSolvePass::Capped);
    assert_eq!(diagnostics.iterations, 0);
    assert_eq!(diagnostics.backtracking_count, 0);
    assert_eq!(
        diagnostics.solve,
        crate::diagnostics::SolveIdentity::HydraulicSystem
    );
    assert_eq!(diagnostics.pivot_magnitude, Some(0.0));
    assert!(diagnostics.matrix_norm.is_some());
    assert_eq!(
        diagnostics
            .capped_operands
            .as_ref()
            .expect("capped operands")
            .layers
            .len(),
        5
    );
    assert_eq!(
        beginning,
        state([-5_900.0, -5_450.0, -4_300.0, -2_850.0, 0.68, 0.66])
    );
}

#[test]
fn zero_caps_execute_twenty_halving_exhaustion_with_complete_failed_iterate() {
    let (case, family) = fixture();
    let expected = &family["accepted_constrained_all_cap"];
    let zero_caps = caps(expected)
        .into_keys()
        .map(|layer| (layer, 0.0))
        .collect();
    let evaluator =
        V3ConstitutiveEvaluator::new_capped(case, (-5_900.0, -5_450.0), context(), zero_caps)
            .expect("zero-cap evaluator");
    let beginning = state([-5_900.0, -5_450.0, -4_300.0, -2_850.0, 0.68, 0.66]);
    let error = evaluator
        .solve_capped(&identity(), beginning)
        .expect_err("zero caps exhaust canonical backtracking");
    let VegetationError::NumericalFailure(diagnostics) = error else {
        panic!("typed numerical failure required");
    };
    assert_eq!(diagnostics.pass, CoupledSolvePass::Capped);
    assert_eq!(diagnostics.iterations, 7);
    assert_eq!(diagnostics.backtracking_count, 94);
    let frozen = family["executed_failures"]
        .as_array()
        .expect("failures")
        .iter()
        .find(|value| value["failure"] == "backtracking_limit")
        .expect("frozen backtracking failure");
    assert_eq!(
        diagnostics.active_water_caps,
        frozen["active_water_caps"]
            .as_array()
            .expect("active caps")
            .iter()
            .map(|value| SoilLayerId::try_new(value.as_str().expect("layer")).expect("typed"))
            .collect::<Vec<_>>()
    );
    close(
        diagnostics.step_norm.expect("step"),
        frozen["diagnostics"]["step_norm"].as_f64().expect("step"),
        1.0e-7,
        // Provisional portability observation only: this cannot pass Stage B
        // because authority freezes CPython failed-iterate rounding without
        // admitting a cross-runtime comparison tolerance.
        3.0e-6,
    );
    assert_eq!(
        diagnostics
            .capped_operands
            .as_ref()
            .expect("failed iterate")
            .layers
            .len(),
        5
    );
    // The exact owner/basis identity is bound by the validated authorization
    // boundary in `execute_capped_column_pass`, not synthesized by equations.
    assert!(diagnostics.fixed_authorization_identity.is_none());
    assert_eq!(
        beginning,
        state([-5_900.0, -5_450.0, -4_300.0, -2_850.0, 0.68, 0.66])
    );
}

#[test]
fn coupled_near_tie_sides_reselect_expected_branch_at_accepted_endpoint() {
    let (case, family) = fixture();
    let fully = &family["fully_authorized_value_reduction"]["capped_result"];
    let q_law = fully["fluxes"]["q3"][0]["q_law_kg_m2_tile_s"]
        .as_f64()
        .expect("law");
    for (cap, expected_active) in [(q_law * 0.99, true), (q_law * 1.01, false)] {
        let mut cap_rates = caps(fully);
        cap_rates.insert(SoilLayerId::try_new("soil-1").expect("layer"), cap);
        let evaluator = V3ConstitutiveEvaluator::new_capped(
            case.clone(),
            (-5_900.0, -5_450.0),
            context(),
            cap_rates,
        )
        .expect("near-tie evaluator");
        let accepted = evaluator
            .solve_capped(
                &identity(),
                state([-5_900.0, -5_450.0, -4_300.0, -2_850.0, 0.68, 0.66]),
            )
            .expect("coupled near-tie solve");
        let layer = accepted
            .outer
            .evaluation
            .capped_layer_fluxes
            .iter()
            .find(|layer| layer.layer_id.as_str() == "soil-1")
            .expect("soil-1");
        assert_eq!(layer.authorization_active_or_tie, expected_active);
    }
}
