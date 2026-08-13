//! Exact V5 authorization-capped E11--E15 fixture acceptance.

use std::collections::BTreeMap;

use openwepp_kernel_contract::{
    OccupancyId, ResourceAmountBasis, ResourceOwnerId, SoilLayerId, StratumId, TileId,
    TransactionId,
};
use serde::Serialize;
use serde_json::Value;
use sha2::{Digest, Sha256};

use super::capped_pass::bind_fixed_authorization_failure;
use super::constitutive::{
    ConstitutiveSolveContext, V3ConstitutiveEvaluator, V3PotentialCase, select_capped_flux,
};
use super::potential::{
    StageAEvaluator, StageASolution, StageASolveIdentity, StageAState, capped_numerical_operands,
};
use crate::diagnostics::{
    CoupledSolvePass, FixedAuthorizationIdentity, NumericalFailureDiagnostics, SolveIdentity,
};
use crate::error::NumericalFailureCategory;
use crate::{CoupledOwnedState, VegetationConfiguration, VegetationError};

const VECTORS: &str = include_str!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/../../docs/work-packages/20260812-c3-woody-potential-pass-authority-001/artifacts/openwepp_c3_woody_v5_vectors.json"
));
const V6_PORTABILITY_VECTORS: &str = include_str!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/../../docs/work-packages/20260813-c3-woody-failure-diagnostic-portability-authority-001/artifacts/openwepp_c3_woody_v6_vectors.json"
));
const V6_DIAGNOSTIC_RTOL: f64 = 3.0e-7;
const V6_EXACT_PORTABILITY_FIELDS: [&str; 21] = [
    "model_definition_sha256",
    "configuration_sha256",
    "transaction_id",
    "occupancy_id",
    "pass",
    "solve",
    "field",
    "typed_failure",
    "candidate",
    "unit",
    "basis",
    "present",
    "iterations",
    "backtracking_count",
    "residual_cardinality",
    "active_bounds",
    "active_water_caps",
    "branches",
    "rollback_sha256_before",
    "rollback_sha256_after",
    "accepted_value",
];

fn root() -> Value {
    serde_json::from_str(VECTORS).expect("released V5 fixture parses")
}

fn v6_portability_root() -> Value {
    serde_json::from_str(V6_PORTABILITY_VECTORS).expect("released V6 portability fixture parses")
}

#[derive(Clone, Copy, Serialize)]
#[serde(rename_all = "snake_case")]
enum V6DiagnosticField {
    StepNorm,
}

#[derive(Clone, Copy, Serialize)]
#[serde(rename_all = "snake_case")]
enum V6DiagnosticUnit {
    MixedNativeUnknownUnits,
}

#[derive(Clone, Copy, Serialize)]
#[serde(rename_all = "snake_case")]
enum V6DiagnosticBasis {
    UnscaledSixUnknownNewtonCorrection,
}

/// Runtime context for evidence that is necessarily outside the numerical
/// diagnostics payload. This is constructed from the executed solve inputs,
/// never from the expected portability record.
struct V6RejectedExecutionContext {
    solve_context: ConstitutiveSolveContext,
    configuration_sha256: String,
    field: V6DiagnosticField,
    unit: V6DiagnosticUnit,
    basis: V6DiagnosticBasis,
}

#[derive(Serialize)]
struct V6CappedTransactionSnapshot<'a> {
    configuration: &'a VegetationConfiguration,
    beginning_owner: &'a CoupledOwnedState,
    constitutive_configuration: (&'a V3PotentialCase, &'a BTreeMap<SoilLayerId, f64>),
    beginning_stage_a: [f64; 6],
    attempted_transaction_id: TransactionId,
    attempted_occupancy_id: &'a OccupancyId,
    fixed_authorization_identity: &'a FixedAuthorizationIdentity,
    candidate: Option<&'static str>,
}

fn sha256(bytes: &[u8]) -> String {
    format!("{:x}", Sha256::digest(bytes))
}

fn rollback_snapshot_bytes(
    configuration: &VegetationConfiguration,
    beginning_owner: &CoupledOwnedState,
    case: &V3PotentialCase,
    caps: &BTreeMap<SoilLayerId, f64>,
    beginning: StageAState,
    attempted_identity: &StageASolveIdentity,
    fixed_authorization_identity: &FixedAuthorizationIdentity,
) -> Vec<u8> {
    serde_json::to_vec(&V6CappedTransactionSnapshot {
        configuration,
        beginning_owner,
        constitutive_configuration: (case, caps),
        beginning_stage_a: [
            beginning.psi_sunleaf_mm,
            beginning.psi_shadeleaf_mm,
            beginning.psi_stem_mm,
            beginning.psi_root_mm,
            beginning.beta_sun,
            beginning.beta_shade,
        ],
        attempted_transaction_id: attempted_identity.transaction_id,
        attempted_occupancy_id: &attempted_identity.occupancy_id,
        fixed_authorization_identity,
        candidate: None,
    })
    .expect("complete capped-transaction rollback snapshot serializes")
}

/// Applies the V6 evidence-only comparison after every categorical and domain
/// firewall has passed.
fn v6_portable_rejected_step_norm_equal(reference: &Value, actual: &Value) -> bool {
    if V6_EXACT_PORTABILITY_FIELDS
        .iter()
        .any(|field| reference[*field] != actual[*field])
        || reference["pass"] != "capped"
        || reference["solve"] != "hydraulic_system"
        || reference["field"] != "step_norm"
        || reference["typed_failure"] != "backtracking_limit"
        || reference["present"] != true
        || !reference["candidate"].is_null()
        || reference["accepted_value"] != false
        || reference["unit"] != "mixed_native_unknown_units"
        || reference["basis"] != "unscaled_six_unknown_newton_correction"
        || reference["rollback_sha256_before"] != reference["rollback_sha256_after"]
        || actual["rollback_sha256_before"] != actual["rollback_sha256_after"]
        || reference["scalar"]["class"] != "finite"
        || actual["scalar"]["class"] != "finite"
    {
        return false;
    }

    let (Some(a), Some(b)) = (
        reference["scalar"]["value"].as_f64(),
        actual["scalar"]["value"].as_f64(),
    ) else {
        return false;
    };
    if !a.is_finite() || !b.is_finite() || a < 0.0 || b < 0.0 {
        return false;
    }
    let a_zero = a.abs().to_bits() == 0;
    let b_zero = b.abs().to_bits() == 0;
    if a_zero != b_zero || (!a_zero && a.is_sign_negative() != b.is_sign_negative()) {
        return false;
    }

    (a - b).abs() <= V6_DIAGNOSTIC_RTOL * a.abs().max(b.abs())
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
        transaction_id: TransactionId(1),
        occupancy_id: OccupancyId {
            stratum_id: StratumId::try_new("tree-1").expect("stratum"),
            tile_id: TileId::try_new("tile-1").expect("tile"),
        },
    }
}

fn v6_beginning_owner(
    case: &V3PotentialCase,
) -> (VegetationConfiguration, CoupledOwnedState, StageAState) {
    let (mut configuration, mut beginning_owner) =
        crate::transaction::v6_identity_rebound_fixture();
    let template = configuration.strata[0].root_layers[0].clone();
    configuration.strata[0].root_layers = case
        .layers
        .iter()
        .map(|layer| crate::config::RootLayer {
            layer_id: SoilLayerId::try_new(layer.layer_id.clone()).expect("typed fixture layer"),
            root_fraction: layer.root_fraction,
            mineral_n_root_fraction: layer.root_fraction,
            lateral_root_length_m: template.lateral_root_length_m,
        })
        .collect();
    configuration.configuration_sha256.clear();
    configuration.configuration_sha256 = configuration
        .canonical_sha256()
        .expect("five-layer V6 configuration digest");
    beginning_owner
        .configuration_sha256
        .clone_from(&configuration.configuration_sha256);
    let attempted = identity();
    let lane = beginning_owner
        .occupancies
        .get_mut(&attempted.occupancy_id)
        .expect("attempted occupancy belongs to beginning owner");
    lane.sun_leaf_potential_mm = -5_900.0;
    lane.shade_leaf_potential_mm = -5_450.0;
    lane.stem_potential_mm = -4_300.0;
    lane.root_node_potential_mm = -2_850.0;
    lane.beta_hyd = 0.67;
    beginning_owner.state_sha256 = beginning_owner
        .canonical_sha256()
        .expect("five-layer beginning-owner digest");
    configuration
        .initial_state_sha256
        .clone_from(&beginning_owner.state_sha256);
    beginning_owner
        .validate(&configuration)
        .expect("five-layer beginning owner remains valid");
    (
        configuration,
        beginning_owner,
        state([-5_900.0, -5_450.0, -4_300.0, -2_850.0, 0.68, 0.66]),
    )
}

fn v6_fixed_authorization_identity() -> FixedAuthorizationIdentity {
    let attempted = identity();
    FixedAuthorizationIdentity {
        transaction_id: attempted.transaction_id,
        owner_id: ResourceOwnerId::try_new("v6-portability-water-owner")
            .expect("typed water owner"),
        occupancy_id: attempted.occupancy_id,
        basis: ResourceAmountBasis::WaterKgPerSquareMeterStandGroundInterval,
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

fn v6_runtime_rejected_step_norm_records() -> (Value, Value) {
    let (case, actual_family) = fixture();
    let expected = &actual_family["accepted_constrained_all_cap"];
    let zero_caps = caps(expected)
        .into_keys()
        .map(|layer| (layer, 0.0))
        .collect::<BTreeMap<_, _>>();
    let (configuration, beginning_owner, beginning) = v6_beginning_owner(&case);
    let execution = V6RejectedExecutionContext {
        solve_context: context(),
        configuration_sha256: configuration.configuration_sha256.clone(),
        field: V6DiagnosticField::StepNorm,
        unit: V6DiagnosticUnit::MixedNativeUnknownUnits,
        basis: V6DiagnosticBasis::UnscaledSixUnknownNewtonCorrection,
    };
    let evaluator = V3ConstitutiveEvaluator::new_capped(
        case.clone(),
        (-5_900.0, -5_450.0),
        execution.solve_context.clone(),
        zero_caps.clone(),
    )
    .expect("zero-cap evaluator");
    let attempted = identity();
    let fixed_authorization_identity = v6_fixed_authorization_identity();
    let rollback_before_bytes = rollback_snapshot_bytes(
        &configuration,
        &beginning_owner,
        &case,
        &zero_caps,
        beginning,
        &attempted,
        &fixed_authorization_identity,
    );
    let result = evaluator.solve_capped(&attempted, beginning);
    let accepted_value = result.is_ok();
    let candidate_present = result.as_ref().ok().is_some();
    let error = bind_fixed_authorization_failure(
        result.expect_err("zero caps exhaust canonical backtracking"),
        fixed_authorization_identity.clone(),
    );
    let VegetationError::NumericalFailure {
        category,
        diagnostics,
    } = error
    else {
        panic!("typed numerical failure required");
    };
    let rollback_after_bytes = rollback_snapshot_bytes(
        &configuration,
        &beginning_owner,
        &case,
        &zero_caps,
        beginning,
        &attempted,
        &fixed_authorization_identity,
    );

    assert_eq!(
        diagnostics.transaction_id,
        execution.solve_context.transaction_id
    );
    assert_eq!(
        diagnostics.occupancy_id,
        execution.solve_context.occupancy_id
    );
    assert_eq!(diagnostics.pass, execution.solve_context.pass);
    assert_eq!(
        diagnostics.fixed_authorization_identity.as_ref(),
        Some(&fixed_authorization_identity)
    );
    assert_eq!(category, NumericalFailureCategory::BacktrackingLimit);
    assert!(!accepted_value);
    assert!(!candidate_present);

    let actual = v6_actual_record(
        &diagnostics,
        category,
        &execution,
        accepted_value,
        candidate_present,
        &rollback_before_bytes,
        &rollback_after_bytes,
    );

    (v6_reference_record(), actual)
}

fn v6_reference_record() -> Value {
    // The reference retains V6's frozen comparator schema/scalar and V5's
    // frozen executed-failure caps/branches. Only the authority's explicit
    // identity/rollback sentinels are bound to independently derived runtime
    // identities and independently serialized reference beginning state.
    let (reference_case, reference_family) = fixture();
    let reference_zero_caps = caps(&reference_family["accepted_constrained_all_cap"])
        .into_keys()
        .map(|layer| (layer, 0.0))
        .collect::<BTreeMap<_, _>>();
    let (reference_configuration, reference_beginning_owner, reference_beginning) =
        v6_beginning_owner(&reference_case);
    let expected_configuration_sha256 = reference_configuration.configuration_sha256.clone();
    let frozen = reference_family["executed_failures"]
        .as_array()
        .expect("failures")
        .iter()
        .find(|value| value["failure"] == "backtracking_limit")
        .expect("frozen backtracking failure");
    let portability = v6_portability_root();
    let observed = portability["families"]["numeric_boundary_cases"]
        .as_array()
        .expect("V6 numeric cases")
        .iter()
        .find(|case| case["case"] == "observed_cpython_rust_step_norm")
        .expect("observed CPython/Rust portability case");
    assert_eq!(
        observed["reference"]["scalar"]["value"],
        frozen["diagnostics"]["step_norm"]
    );
    let expected_beginning_bytes = rollback_snapshot_bytes(
        &reference_configuration,
        &reference_beginning_owner,
        &reference_case,
        &reference_zero_caps,
        reference_beginning,
        &identity(),
        &v6_fixed_authorization_identity(),
    );
    let mut reference = observed["reference"].clone();
    reference["model_definition_sha256"] = Value::String(crate::MODEL_SHA256.into());
    reference["configuration_sha256"] = Value::String(expected_configuration_sha256);
    reference["transaction_id"] = serde_json::json!(identity().transaction_id);
    reference["occupancy_id"] = serde_json::json!(identity().occupancy_id);
    reference["active_water_caps"] = frozen["diagnostics"]["active_water_caps"].clone();
    reference["branches"] = Value::Array(
        frozen["diagnostics"]["layer_operands_in_configuration_order"]
            .as_array()
            .expect("frozen layer operands")
            .iter()
            .map(|layer| layer["branch"].clone())
            .collect(),
    );
    let expected_rollback_sha256 = sha256(&expected_beginning_bytes);
    reference["rollback_sha256_before"] = Value::String(expected_rollback_sha256.clone());
    reference["rollback_sha256_after"] = Value::String(expected_rollback_sha256);
    reference
}

fn v6_actual_record(
    diagnostics: &NumericalFailureDiagnostics,
    category: NumericalFailureCategory,
    execution: &V6RejectedExecutionContext,
    accepted_value: bool,
    candidate_present: bool,
    rollback_before_bytes: &[u8],
    rollback_after_bytes: &[u8],
) -> Value {
    let operands = diagnostics
        .capped_operands
        .as_ref()
        .expect("failed capped iterate retains operands");
    let branches = operands
        .layers
        .iter()
        .map(|layer| {
            if layer.authorization_active_or_tie {
                "authorization_active_or_tie"
            } else {
                "constitutive_law"
            }
        })
        .collect::<Vec<_>>();
    let step_norm = diagnostics.step_norm;
    serde_json::json!({
        "model_definition_sha256": diagnostics.model_definition_sha256,
        "configuration_sha256": execution.configuration_sha256,
        "transaction_id": diagnostics.transaction_id,
        "occupancy_id": diagnostics.occupancy_id,
        "pass": diagnostics.pass,
        "solve": diagnostics.solve,
        "field": execution.field,
        "typed_failure": category,
        "candidate": candidate_present.then_some("accepted_stage_a"),
        "unit": execution.unit,
        "basis": execution.basis,
        "present": step_norm.is_some(),
        "iterations": diagnostics.iterations,
        "backtracking_count": diagnostics.backtracking_count,
        "residual_cardinality": diagnostics.residual_norms.len(),
        "active_bounds": diagnostics.active_bounds,
        "active_water_caps": diagnostics.active_water_caps,
        "branches": branches,
        "rollback_sha256_before": sha256(rollback_before_bytes),
        "rollback_sha256_after": sha256(rollback_after_bytes),
        "accepted_value": accepted_value,
        "scalar": {
            "class": if step_norm.is_some_and(f64::is_finite) { "finite" } else { "nonfinite" },
            "value": step_norm,
        },
    })
}

fn close(actual: f64, expected: f64, atol: f64, rtol: f64) {
    assert!(
        (actual - expected).abs() <= atol + rtol * expected.abs(),
        "actual={actual:.17e}, expected={expected:.17e}"
    );
}

fn assert_accepted_state_and_diagnostics(actual: &StageASolution, expected: &Value) {
    let solution = &expected["solution"];
    for (value, field) in [
        (actual.state.psi_sunleaf_mm, "sun_leaf_potential_mm"),
        (actual.state.psi_shadeleaf_mm, "shade_leaf_potential_mm"),
        (actual.state.psi_stem_mm, "stem_potential_mm"),
        (actual.state.psi_root_mm, "root_node_potential_mm"),
        (actual.state.beta_sun, "beta_hyd_sun"),
        (actual.state.beta_shade, "beta_hyd_shade"),
    ] {
        close(
            value,
            solution[field].as_f64().expect("solution value"),
            1.0e-7,
            1.0e-10,
        );
    }
    assert_eq!(
        actual.iterations,
        u32::try_from(expected["iterations"].as_u64().expect("iterations"))
            .expect("u32 iterations")
    );
    assert_eq!(
        actual.backtracking_count,
        u32::try_from(
            expected["backtracking_count"]
                .as_u64()
                .expect("backtracking count")
        )
        .expect("u32 backtracks")
    );
    let operands = capped_numerical_operands(&actual.evaluation, &actual.state)
        .expect("accepted capped diagnostics");
    for ((residual, operand), frozen) in actual
        .normalized_residuals
        .iter()
        .zip(&operands.residuals)
        .zip(
            expected["normalized_residuals"]
                .as_array()
                .expect("residuals"),
        )
    {
        assert_eq!(
            residual.identity,
            frozen["identity"].as_str().expect("identity")
        );
        assert_eq!(operand.identity, residual.identity);
        assert_eq!(residual.value.to_bits(), operand.normalized.to_bits());
        assert_eq!(
            operand.normalized.to_bits(),
            (operand.raw_kg_m2_tile_s / operand.tolerance).to_bits()
        );
        assert!(residual.value.is_finite() && residual.value.abs() <= 1.0);
        assert!(
            frozen["normalized"]
                .as_f64()
                .expect("frozen normalized")
                .abs()
                <= 1.0
        );
        assert_eq!(
            operand.scale_kg_m2_tile_s.to_bits(),
            operands.water_residual_scale_kg_m2_tile_s.to_bits()
        );
    }
    let frozen_history = expected["residual_norm_history"]
        .as_array()
        .expect("history");
    assert_eq!(actual.residual_norm_history.len(), frozen_history.len());
    assert!(
        actual
            .residual_norm_history
            .iter()
            .all(|value| value.is_finite() && *value >= 0.0)
    );
    let terminal_norm = actual
        .normalized_residuals
        .iter()
        .map(|value| value.value.abs())
        .fold(0.0, f64::max);
    assert_eq!(
        actual
            .residual_norm_history
            .last()
            .expect("terminal norm")
            .to_bits(),
        terminal_norm.to_bits()
    );
    assert!(actual.potential_step_mm.is_finite() && actual.potential_step_mm <= 1.0e-7);
    assert!(actual.pivot_magnitude.is_finite() && actual.pivot_magnitude > 0.0);
    assert!(actual.matrix_norm.is_finite() && actual.matrix_norm > 0.0);
}

fn assert_capped_layer_amounts(actual: &StageASolution, expected: &Value, fraction: f64, dt: f64) {
    for (layer, frozen) in actual
        .evaluation
        .capped_layer_fluxes
        .iter()
        .zip(expected["fluxes"]["q3"].as_array().expect("q3"))
    {
        assert_eq!(
            layer.layer_id.as_str(),
            frozen["layer_id"].as_str().expect("layer")
        );
        close(
            layer.q_law_kg_m2_s,
            frozen["q_law_kg_m2_tile_s"].as_f64().expect("q law"),
            1.0e-12,
            1.0e-9,
        );
        assert_eq!(
            layer.cap_rate_kg_m2_s.to_bits(),
            frozen["cap_rate_kg_m2_tile_s"]
                .as_f64()
                .expect("cap")
                .to_bits()
        );
        close(
            layer.q_final_kg_m2_s,
            frozen["q_final_kg_m2_tile_s"].as_f64().expect("q final"),
            1.0e-12,
            1.0e-9,
        );
        assert_eq!(
            layer.authorization_active_or_tie,
            frozen["branch"] == "authorization_active_or_tie"
        );
        let demand = frozen["potential_request_kg_m2_stand_ground"]
            .as_f64()
            .expect("demand");
        let authorization = fraction * layer.cap_rate_kg_m2_s * dt;
        let finalized = fraction * layer.q_final_kg_m2_s * dt;
        close(
            authorization,
            frozen["authorization_kg_m2_stand_ground"]
                .as_f64()
                .expect("authorization"),
            1.0e-15,
            1.0e-12,
        );
        close(
            finalized,
            frozen["finalized_use_kg_m2_stand_ground"]
                .as_f64()
                .expect("finalized"),
            1.0e-15,
            1.0e-12,
        );
        assert!(finalized <= authorization && authorization <= demand);
    }
}

fn assert_coupled_fluxes(actual: &StageASolution, expected: &Value) {
    let fluxes = &expected["fluxes"];
    for (value, field) in [
        (
            actual.evaluation.gas_sun_kg_m2_s,
            "gas_energy_transpiration_sun",
        ),
        (
            actual.evaluation.gas_shade_kg_m2_s,
            "gas_energy_transpiration_shade",
        ),
        (actual.evaluation.q1_sun_kg_m2_s, "q1_sun"),
        (actual.evaluation.q1_shade_kg_m2_s, "q1_shade"),
        (actual.evaluation.q2_kg_m2_s, "q2"),
    ] {
        close(
            value,
            fluxes[field].as_f64().expect("coupled flux"),
            1.0e-15,
            1.0e-9,
        );
    }
}

#[test]
fn constrained_all_cap_matches_released_v5_endpoint_and_operands() {
    let (case, family) = fixture();
    let expected = &family["accepted_constrained_all_cap"];
    let tile_fraction = case.tile_fraction;
    let interval_s = case.dt_s;
    let evaluator =
        V3ConstitutiveEvaluator::new_capped(case, (-5_900.0, -5_450.0), context(), caps(expected))
            .expect("capped evaluator");
    let accepted = evaluator
        .solve_capped(
            &identity(),
            state([-5_900.0, -5_450.0, -4_300.0, -2_850.0, 0.68, 0.66]),
        )
        .expect("capped coupled solve");
    assert_accepted_state_and_diagnostics(&accepted.outer, expected);
    assert_eq!(
        accepted.outer.evaluation.active_water_caps,
        expected["active_water_caps"]
            .as_array()
            .expect("active caps")
            .iter()
            .map(|value| SoilLayerId::try_new(value.as_str().expect("layer")).expect("typed layer"))
            .collect::<Vec<_>>()
    );
    assert_capped_layer_amounts(&accepted.outer, expected, tile_fraction, interval_s);
    assert_coupled_fluxes(&accepted.outer, expected);
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
fn valid_authorization_demand_branch_sweep_executes_every_frozen_case() {
    let (case, family) = fixture();
    let demand = &family["accepted_constrained_all_cap"]["fluxes"]["q3"];
    let sweep = &family["valid_A_le_D_branch_sweep"];
    let mut found_positive_law_branch = false;
    for frozen in sweep["cases"].as_array().expect("branch sweep cases") {
        let fraction = frozen["uniform_authorization_fraction_of_D"]
            .as_f64()
            .expect("authorization fraction");
        let cap_rates = demand
            .as_array()
            .expect("potential requests")
            .iter()
            .map(|layer| {
                let id = SoilLayerId::try_new(layer["layer_id"].as_str().expect("layer"))
                    .expect("typed layer");
                let potential = layer["potential_request_kg_m2_stand_ground"]
                    .as_f64()
                    .expect("potential request");
                (id, fraction * potential / (case.tile_fraction * case.dt_s))
            })
            .collect::<BTreeMap<_, _>>();
        let evaluator = V3ConstitutiveEvaluator::new_capped(
            case.clone(),
            (-5_900.0, -5_450.0),
            context(),
            cap_rates,
        )
        .expect("sweep evaluator");
        let result = evaluator.solve_capped(
            &identity(),
            state([-5_900.0, -5_450.0, -4_300.0, -2_850.0, 0.68, 0.66]),
        );
        assert_eq!(result.is_ok(), frozen["accepted"] == true);
        let accepted = result.expect("frozen valid A<=D sweep accepts");
        let actual_positive = accepted
            .outer
            .evaluation
            .capped_layer_fluxes
            .iter()
            .filter(|layer| layer.q_final_kg_m2_s > 0.0)
            .map(|layer| {
                if layer.authorization_active_or_tie {
                    "authorization_active_or_tie"
                } else {
                    found_positive_law_branch = true;
                    "constitutive_law"
                }
            })
            .collect::<Vec<_>>();
        assert_eq!(
            actual_positive,
            frozen["positive_layer_branches"]
                .as_array()
                .expect("positive branches")
                .iter()
                .map(|value| value.as_str().expect("branch"))
                .collect::<Vec<_>>()
        );
    }
    assert_eq!(
        found_positive_law_branch,
        sweep["accepted_positive_law_branch_found"] == true
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
    let cap_rates = caps(expected);
    let (configuration, beginning_owner, beginning) = v6_beginning_owner(&case);
    let fixed = v6_fixed_authorization_identity();
    let before = rollback_snapshot_bytes(
        &configuration,
        &beginning_owner,
        &case,
        &cap_rates,
        beginning,
        &identity(),
        &fixed,
    );
    let evaluator = V3ConstitutiveEvaluator::new_capped(
        case.clone(),
        (-5_900.0, -5_450.0),
        context(),
        cap_rates.clone(),
    )
    .expect("capped evaluator");
    let result = evaluator.solve_capped_with_limit(&identity(), beginning, 0);
    assert!(result.is_err(), "iteration failure emitted a candidate");
    let error = bind_fixed_authorization_failure(
        result.expect_err("zero iteration limit rejects"),
        fixed.clone(),
    );
    let VegetationError::NumericalFailure {
        category,
        diagnostics,
    } = error
    else {
        panic!("typed capped numerical failure required");
    };
    assert_eq!(category, NumericalFailureCategory::IterationLimit);
    assert_eq!(
        diagnostics.fixed_authorization_identity,
        Some(fixed.clone())
    );
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
    let after = rollback_snapshot_bytes(
        &configuration,
        &beginning_owner,
        &case,
        &cap_rates,
        beginning,
        &identity(),
        &fixed,
    );
    assert_eq!(
        after, before,
        "iteration failure changed owner/transaction bytes"
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
    let cap_rates = caps(expected);
    let (configuration, beginning_owner, beginning) = v6_beginning_owner(&case);
    let fixed = v6_fixed_authorization_identity();
    let before = rollback_snapshot_bytes(
        &configuration,
        &beginning_owner,
        &case,
        &cap_rates,
        beginning,
        &identity(),
        &fixed,
    );
    let evaluator = V3ConstitutiveEvaluator::new_capped(
        case.clone(),
        (-5_900.0, -5_450.0),
        context(),
        cap_rates.clone(),
    )
    .expect("capped evaluator")
    .with_released_singular_hydraulics();
    let result = evaluator.solve_capped(&identity(), beginning);
    assert!(result.is_err(), "singular failure emitted a candidate");
    let error = bind_fixed_authorization_failure(
        result.expect_err("singular capped hydraulics reject"),
        fixed.clone(),
    );
    let VegetationError::NumericalFailure {
        category,
        diagnostics,
    } = error
    else {
        panic!("typed numerical failure required");
    };
    assert_eq!(category, NumericalFailureCategory::SingularPivot);
    assert_eq!(
        diagnostics.fixed_authorization_identity,
        Some(fixed.clone())
    );
    assert_eq!(diagnostics.pass, CoupledSolvePass::Capped);
    assert_eq!(diagnostics.iterations, 0);
    assert_eq!(diagnostics.backtracking_count, 0);
    assert_eq!(diagnostics.solve, SolveIdentity::HydraulicSystem);
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
    let after = rollback_snapshot_bytes(
        &configuration,
        &beginning_owner,
        &case,
        &cap_rates,
        beginning,
        &identity(),
        &fixed,
    );
    assert_eq!(
        after, before,
        "singular failure changed owner/transaction bytes"
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
    let VegetationError::NumericalFailure {
        category,
        diagnostics,
    } = error
    else {
        panic!("typed numerical failure required");
    };
    assert_eq!(category, NumericalFailureCategory::BacktrackingLimit);
    assert_eq!(diagnostics.model_definition_sha256, crate::MODEL_SHA256);
    assert_eq!(diagnostics.transaction_id, identity().transaction_id);
    assert_eq!(diagnostics.occupancy_id, identity().occupancy_id);
    assert_eq!(diagnostics.pass, CoupledSolvePass::Capped);
    assert_eq!(diagnostics.solve, SolveIdentity::HydraulicSystem);
    assert_eq!(diagnostics.iterations, 7);
    assert_eq!(diagnostics.backtracking_count, 94);
    assert!(diagnostics.step_norm.is_some());
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
    let failed_operands = diagnostics
        .capped_operands
        .as_ref()
        .expect("failed iterate");
    assert_eq!(diagnostics.residual_norms.len(), 6);
    assert_eq!(failed_operands.layers.len(), 5);
    let expected_branches = frozen["diagnostics"]["layer_operands_in_configuration_order"]
        .as_array()
        .expect("frozen layer operands")
        .iter()
        .map(|layer| layer["branch"].clone())
        .collect::<Vec<_>>();
    let actual_branches = failed_operands
        .layers
        .iter()
        .map(|layer| {
            Value::String(
                if layer.authorization_active_or_tie {
                    "authorization_active_or_tie"
                } else {
                    "constitutive_law"
                }
                .into(),
            )
        })
        .collect::<Vec<_>>();
    assert_eq!(actual_branches, expected_branches);

    let (reference_record, actual_record) = v6_runtime_rejected_step_norm_records();
    assert!(v6_portable_rejected_step_norm_equal(
        &reference_record,
        &actual_record,
    ));
    assert_eq!(
        beginning,
        state([-5_900.0, -5_450.0, -4_300.0, -2_850.0, 0.68, 0.66])
    );
}

#[test]
fn v6_rejected_step_norm_portability_enforces_boundaries_and_all_firewalls() {
    let root = v6_portability_root();
    assert_eq!(root["comparison"]["relative_tolerance"], V6_DIAGNOSTIC_RTOL);
    assert_eq!(
        root["comparison"]["formula"],
        "abs(a-b) <= rtol*max(abs(a),abs(b))"
    );
    let families = &root["families"];
    for (family, expected_count) in [
        ("numeric_boundary_cases", 10),
        ("eligibility_and_firewall_poisons", 20),
        ("nonfinite_rejections", 4),
    ] {
        let cases = families[family]
            .as_array()
            .unwrap_or_else(|| panic!("{family} is an array"));
        assert_eq!(cases.len(), expected_count, "complete {family} inventory");
        for case in cases {
            let expected = case["expected_equal"]
                .as_bool()
                .expect("comparison expectation");
            assert_eq!(
                v6_portable_rejected_step_norm_equal(&case["reference"], &case["actual"],),
                expected,
                "V6 portability case {}",
                case["case"].as_str().expect("case name"),
            );
        }
    }
}

#[test]
fn v6_runtime_record_rejects_poison_at_each_exact_firewall_seam() {
    let (reference, actual) = v6_runtime_rejected_step_norm_records();
    assert!(
        v6_portable_rejected_step_norm_equal(&reference, &actual),
        "runtime record differs from reference: reference={reference:#} actual={actual:#}"
    );
    assert_eq!(V6_EXACT_PORTABILITY_FIELDS.len(), 21);

    for field in V6_EXACT_PORTABILITY_FIELDS {
        let mut poisoned = actual.clone();
        poisoned[field] = match field {
            "model_definition_sha256" => Value::String("0".repeat(64)),
            "configuration_sha256" => Value::String("1".repeat(64)),
            "transaction_id" => serde_json::json!(54),
            "occupancy_id" => serde_json::json!({"stratum_id": "canopy", "tile_id": "tile-b"}),
            "pass" => serde_json::json!("potential"),
            "solve" => serde_json::json!("canopy_energy"),
            "field" => serde_json::json!("residual_norm"),
            "typed_failure" => serde_json::json!("iteration_limit"),
            "candidate" => serde_json::json!({"present": true}),
            "unit" => serde_json::json!("dimensionless"),
            "basis" => serde_json::json!("scaled_residual"),
            "present" => serde_json::json!(false),
            "iterations" => serde_json::json!(8),
            "backtracking_count" => serde_json::json!(95),
            "residual_cardinality" => serde_json::json!(5),
            "active_bounds" => serde_json::json!(["beta_sun_lower"]),
            "active_water_caps" => serde_json::json!(["soil-2", "soil-1"]),
            "branches" => serde_json::json!(["constitutive_law"]),
            "rollback_sha256_before" => Value::String("2".repeat(64)),
            "rollback_sha256_after" => Value::String("3".repeat(64)),
            "accepted_value" => serde_json::json!(true),
            _ => unreachable!("complete exact firewall inventory"),
        };
        assert!(
            !v6_portable_rejected_step_norm_equal(&reference, &poisoned),
            "runtime-derived {field} poison must reject"
        );
    }
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
