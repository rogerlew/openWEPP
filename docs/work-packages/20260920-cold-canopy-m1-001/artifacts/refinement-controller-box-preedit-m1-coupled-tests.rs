use crate::{
    BandDirectionalFluxes, CoveredColumnAuthority, CoveredColumnInputs, LandSurfaceEnergyError,
    M1ColumnTopologyBinding, M1CoupledColumnInput, M1CoupledColumnTrial, M1CoupledError,
    M1CoupledEvaluation, M1DiagnosticCaptureStatus, M1OccupancySupportCondition,
    M1PhaseReservoirInput, M1SupportConditions, NativeSnowExchangeV1, Sha256Digest,
    evaluate_m1_coupled_column, solve_m1_coupled_column,
};
use openwepp_vegetation::cold_canopy_m1::{M1CapacityBranch, M1Phase, M1PhaseReservoir};
use serde_json::Value;

const RADIATION: &str = include_str!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/../../docs/work-packages/20260920-cold-canopy-m1-001/artifacts/radiation-reference-m1.json"
));
const GAS: &str = include_str!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/../../docs/work-packages/20260920-cold-canopy-m1-001/artifacts/dark-gas-reference-m1.json"
));
const ORIGINAL: &str = include_str!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/../../docs/work-packages/20260920-cold-canopy-m1-001/artifacts/original-input-reference.json"
));
const STRUCTURAL_DRAINAGE: &str = include_str!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/../../docs/work-packages/20260920-cold-canopy-m1-001/artifacts/m1-original60-12-linear-reconstruction.json"
));
const STRUCTURAL_DRAINAGE_TRACE: &str = include_str!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/../../docs/work-packages/20260920-cold-canopy-m1-001/artifacts/m1-original60-12.stderr"
));

#[test]
fn m1_expected_red_latent_heat_nonfinite_precedes_finite_domain() {
    let original: Value = serde_json::from_str(ORIGINAL).expect("original input");
    let base = M1CoupledColumnInput::from_covered_column(
        covered_input(&original),
        [
            M1PhaseReservoirInput::new("stratum-z-upper::forest", 0.018, 0.0),
            M1PhaseReservoirInput::new("stratum-a-lower::forest", 0.018, 0.0),
        ],
        conditions(),
        topology_binding(),
    );
    let mut unexpected = Vec::new();
    for (latent_heat, expected) in [
        (f64::NAN, "VEG-E-141"),
        (f64::INFINITY, "VEG-E-141"),
        (f64::NEG_INFINITY, "VEG-E-141"),
        (0.0, "VEG-E-140"),
        (-1.0, "VEG-E-140"),
    ] {
        let mut input = base.clone();
        input.column.latent_heat_j_kg = latent_heat;
        match evaluate_m1_coupled_column(&input, &trial()) {
            Ok(_) => unexpected.push(format!("{latent_heat:?}: accepted, expected {expected}")),
            Err(error) if error.code() != expected => unexpected.push(format!(
                "{latent_heat:?}: {}, expected {expected}",
                error.code()
            )),
            Err(_) => {}
        }
    }
    assert!(
        unexpected.is_empty(),
        "latent heat precedence: {unexpected:?}"
    );
}

#[test]
fn m1_expected_red_first_domain_valid_halving_is_consumed_before_evaluation() {
    // A trial can pass the coordinate-domain predicate while a later core or
    // hydraulic reconstruction is incomplete.  That first valid factor is
    // still the sole no-update witness candidate; a later factor cannot
    // replace it.
    let original: Value = serde_json::from_str(ORIGINAL).expect("original input");
    let input = M1CoupledColumnInput::from_covered_column(
        covered_input(&original),
        [
            M1PhaseReservoirInput::new("stratum-z-upper::forest", 0.018, 0.0),
            M1PhaseReservoirInput::new("stratum-a-lower::forest", 0.018, 0.0),
        ],
        conditions(),
        topology_binding(),
    );
    let (_result, trace, outcomes) =
        super::m1_solve_with_halved_core_incomplete_for_test(&input, 1);
    let mut failures = Vec::new();
    if !trace.contains(&(1, false)) {
        failures.push(format!(
            "b=1 did not reach the real domain-valid seam: {trace:?}"
        ));
    }
    if trace
        .iter()
        .any(|(exponent, first)| *exponent == 2 && *first)
    {
        failures.push(format!(
            "b=2 replaced incomplete b=1 as the no-update witness: {trace:?}"
        ));
    }
    if outcomes
        .iter()
        .any(|(exponent, no_update)| *exponent == 2 && *no_update)
    {
        failures.push(format!(
            "b=2 installed a tempting no-update outcome: {outcomes:?}"
        ));
    }
    assert!(
        failures.is_empty(),
        "first-domain-valid controls: {failures:?}"
    );
}

#[test]
fn m1_first_domain_valid_halving_is_consumed_before_hydraulics() {
    let original: Value = serde_json::from_str(ORIGINAL).expect("original input");
    let input = M1CoupledColumnInput::from_covered_column(
        covered_input(&original),
        [
            M1PhaseReservoirInput::new("stratum-z-upper::forest", 0.018, 0.0),
            M1PhaseReservoirInput::new("stratum-a-lower::forest", 0.018, 0.0),
        ],
        conditions(),
        topology_binding(),
    );
    let (trace, outcomes) = super::m1_solve_with_halved_hydraulic_incomplete_for_test(&input, 1);
    assert!(trace.contains(&(1, false)), "b1 hydraulic seam: {trace:?}");
    assert!(
        !outcomes
            .iter()
            .any(|(exponent, no_update)| *exponent > 1 && *no_update),
        "b2 replaced a hydraulically incomplete b1 witness: {outcomes:?}"
    );
}

#[test]
fn m1_early_incomplete_first_witness_keeps_real_later_strict_decrease_installable() {
    let original: Value = serde_json::from_str(ORIGINAL).expect("original input");
    let input = M1CoupledColumnInput::from_covered_column(
        covered_input(&original),
        [
            M1PhaseReservoirInput::new("stratum-z-upper::forest", 0.018, 0.0),
            M1PhaseReservoirInput::new("stratum-a-lower::forest", 0.018, 0.0),
        ],
        conditions(),
        topology_binding(),
    );
    let (outcomes, activity) = super::m1_solve_with_early_first_witness_incomplete_for_test(&input);
    assert!(
        outcomes
            .iter()
            .any(|(exponent, no_update)| *exponent >= 2 && !*no_update),
        "later factor must install only on the real strict residual decrease branch: {outcomes:?}"
    );
    let installed_current_norm = activity
        .installed_current_norm
        .expect("installed current norm");
    let installed_candidate_norm = activity
        .installed_candidate_norm
        .expect("installed candidate norm");
    let installed_current_coordinates = activity
        .installed_current_coordinates
        .expect("installed current coordinates");
    let installed_candidate_coordinates = activity
        .installed_candidate_coordinates
        .expect("installed candidate coordinates");
    eprintln!(
        "strict-decrease installation: current_norm={installed_current_norm:?} candidate_norm={installed_candidate_norm:?} current={installed_current_coordinates:?} candidate={installed_candidate_coordinates:?}"
    );
    assert!(installed_candidate_norm < installed_current_norm);
    assert_ne!(
        installed_current_coordinates,
        installed_candidate_coordinates
    );
}

#[test]
fn m1_full_governed_excess_enables_only_first_complete_halved_witness() {
    let original: Value = serde_json::from_str(ORIGINAL).expect("original input");
    let input = M1CoupledColumnInput::from_covered_column(
        covered_input(&original),
        [
            M1PhaseReservoirInput::new("stratum-z-upper::forest", 0.018, 0.0),
            M1PhaseReservoirInput::new("stratum-a-lower::forest", 0.018, 0.0),
        ],
        conditions(),
        topology_binding(),
    );
    let (_result, outcomes) = super::m1_solve_with_full_governed_excess_for_test(&input);
    assert!(
        outcomes
            .iter()
            .any(|(exponent, no_update)| *exponent == 1 && *no_update),
        "a finite full governed-step refusal must admit the first complete b=1 witness: {outcomes:?}"
    );
    assert!(
        !outcomes
            .iter()
            .any(|(exponent, no_update)| *exponent > 1 && *no_update),
        "a later halved trial cannot replace the first complete witness: {outcomes:?}"
    );
}

#[test]
fn m1_full_evaluation_failure_cannot_enable_halved_witness() {
    let original: Value = serde_json::from_str(ORIGINAL).expect("original input");
    let input = M1CoupledColumnInput::from_covered_column(
        covered_input(&original),
        [
            M1PhaseReservoirInput::new("stratum-z-upper::forest", 0.018, 0.0),
            M1PhaseReservoirInput::new("stratum-a-lower::forest", 0.018, 0.0),
        ],
        conditions(),
        topology_binding(),
    );
    let (_result, outcomes) = super::m1_solve_with_full_core_incomplete_for_test(&input);
    assert!(
        !outcomes
            .iter()
            .any(|(exponent, no_update)| *exponent > 0 && *no_update),
        "an incomplete full evaluation is not a halved-witness eligibility refusal: {outcomes:?}"
    );
}

#[test]
fn m1_expected_red_nonfinite_current_residual_cannot_enable_no_update() {
    let original: Value = serde_json::from_str(ORIGINAL).expect("original input");
    let input = M1CoupledColumnInput::from_covered_column(
        covered_input(&original),
        [
            M1PhaseReservoirInput::new("stratum-z-upper::forest", 0.018, 0.0),
            M1PhaseReservoirInput::new("stratum-a-lower::forest", 0.018, 0.0),
        ],
        conditions(),
        topology_binding(),
    );
    assert_eq!(
        super::m1_solve_with_nonfinite_current_residual_for_test(&input)
            .expect_err("nonfinite current residual")
            .code(),
        "VEG-E-141",
        "a nonfinite current residual must fail before full or halved witness admission"
    );
}

#[test]
fn m1_first_halved_governed_and_ci_nonfinite_norms_refuse_no_update() {
    let original: Value = serde_json::from_str(ORIGINAL).expect("original input");
    let input = M1CoupledColumnInput::from_covered_column(
        covered_input(&original),
        [
            M1PhaseReservoirInput::new("stratum-z-upper::forest", 0.018, 0.0),
            M1PhaseReservoirInput::new("stratum-a-lower::forest", 0.018, 0.0),
        ],
        conditions(),
        topology_binding(),
    );
    for component in [
        0, 1, 2, 3, 4, 0x40, 0x41, 0x42, 0x43, 0x80, 0x81, 0x82, 0x83, 0x84,
    ] {
        let outcomes = super::m1_solve_with_first_halved_step_poison_for_test(&input, component);
        assert!(
            !outcomes
                .iter()
                .any(|(exponent, no_update)| *exponent == 1 && *no_update),
            "component {component:#x} admitted a nonfinite or governed-excess first witness: {outcomes:?}"
        );
    }
}

// COLD-CANOPY-M1-TR-SVD-BVLS-01 Stage 1 is intentionally module-local.  Its
// seam accepts only a complete numerical subproblem description and exposes
// its actual returned step/face/SVD observations.  It cannot be exported or
// selected by production.  Controller, phase, and materialization controls
// below remain a later Stage 2 obligation; no Stage 1 assertion claims them.
fn assert_exact_f64(actual: f64, expected: f64) {
    assert_eq!(
        actual.partial_cmp(&expected),
        Some(std::cmp::Ordering::Equal)
    );
}

fn assert_exact_f64_slice(actual: &[f64], expected: &[f64]) {
    assert_eq!(actual.len(), expected.len());
    for (&actual, &expected) in actual.iter().zip(expected) {
        assert_exact_f64(actual, expected);
    }
}

#[test]
fn m1_trust_region_subproblem_uses_raw_jacobian_not_scalar_damping() {
    let original: Value = serde_json::from_str(ORIGINAL).expect("original input");
    let input = M1CoupledColumnInput::from_covered_column(
        covered_input(&original),
        [
            M1PhaseReservoirInput::new("stratum-z-upper::forest", 0.018, 0.0),
            M1PhaseReservoirInput::new("stratum-a-lower::forest", 0.018, 0.0),
        ],
        conditions(),
        topology_binding(),
    );
    // Independent oracle at base (0,0): R=(-1,-1), J=[[1,2],[0,1]], x>=0.
    // The old Newton direction is (-1,1), so scalar damping has no positive
    // feasible factor.  On x=0 the frozen linear subproblem is
    // min_y (-1+2y)^2+(-1+y)^2, whose derivative gives y=3/5.  This is a
    // dyadic-free rational result, therefore use an absolute 16-epsilon
    // comparison after the prescribed ordered binary64 arithmetic.  The
    // first all-free face crosses x=0, then the x-active face is resolved;
    // the nonlinear trajectory/root is intentionally controller-stage work.
    let mut upper = [0.0; 21];
    upper[0] = f64::INFINITY;
    upper[1] = f64::INFINITY;
    let result = super::m1_trust_region_solve_subproblem_for_test(
        &input,
        &trial(),
        &super::M1TrustRegionSubproblemInput {
            raw_residual: [
                -1.0, -1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,
                0.0, 0.0, 0.0, 0.0, 0.0,
            ],
            raw_jacobian: [
                [
                    1.0, 2.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,
                    0.0, 0.0, 0.0, 0.0, 0.0,
                ],
                [
                    0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,
                    0.0, 0.0, 0.0, 0.0, 0.0,
                ],
                [0.0; 21],
                [0.0; 21],
                [0.0; 21],
                [0.0; 21],
                [0.0; 21],
                [0.0; 21],
                [0.0; 21],
                [0.0; 21],
                [0.0; 21],
                [0.0; 21],
                [0.0; 21],
                [0.0; 21],
                [0.0; 21],
                [0.0; 21],
                [0.0; 21],
                [0.0; 21],
                [0.0; 21],
                [0.0; 21],
                [0.0; 21],
            ],
            normalizers: [1.0; 21],
            scales: [1.0; 21],
            lower: [0.0; 21],
            upper,
            initial_radius: 1.0,
            base: [0.0; 21],
            test_work: super::M1TrustRegionTestWorkState::default(),
        },
    )
    .expect("Stage 1 bounded subproblem");
    assert!(result.active_lower[0]);
    assert!(!result.active_lower[1]);
    assert_eq!(result.entered_svd_factorizations, 2);
    assert!((result.scaled_step[0] - 0.0).abs() <= 16.0 * f64::EPSILON);
    assert!((result.scaled_step[1] - 3.0 / 5.0).abs() <= 16.0 * f64::EPSILON);
}

#[test]
fn m1_trust_region_frozen_merit_and_degenerate_faces_are_numeric() {
    let original: Value = serde_json::from_str(ORIGINAL).expect("original input");
    let input = M1CoupledColumnInput::from_covered_column(
        covered_input(&original),
        [
            M1PhaseReservoirInput::new("stratum-z-upper::forest", 0.018, 0.0),
            M1PhaseReservoirInput::new("stratum-a-lower::forest", 0.018, 0.0),
        ],
        conditions(),
        topology_binding(),
    );
    let before = input.clone();
    // R=(2,4), t=(2,4), Jraw=diag(6,8), S=(1/2,1/4), p=(-1/2,-1).
    // Frozen W gives Phi0=1, Phi1=5/32 and pred=ared=27/32.  A deliberately
    // different candidate normalizer (1,2) would give Phi1=5/8 and must not
    // be used for actual reduction or rho.
    let merit = super::m1_trust_region_frozen_merit_for_test(
        &input,
        &trial(),
        super::M1TrustRegionMeritInput {
            raw_residual: [2.0, 4.0],
            raw_jacobian: [[6.0, 0.0], [0.0, 8.0]],
            frozen_normalizers: [2.0, 4.0],
            coordinate_scales: [0.5, 0.25],
            scaled_step: [-0.5, -1.0],
            candidate_raw_residual: [0.5, 2.0],
            moving_candidate_normalizers: [1.0, 2.0],
        },
    )
    .expect("frozen merit assembly");
    assert_exact_f64(merit.predicted_reduction, 27.0 / 32.0);
    assert_exact_f64(merit.actual_reduction, 27.0 / 32.0);
    assert_exact_f64(merit.rho, 1.0);
    assert_exact_f64(merit.moving_weight_trial_merit, 5.0 / 8.0);

    let precedence = super::m1_trust_region_frozen_merit_for_test(
        &input,
        &trial(),
        super::M1TrustRegionMeritInput {
            raw_residual: [1.0, 0.0],
            raw_jacobian: [[1.0, 0.0], [0.0, 1.0]],
            frozen_normalizers: [1.0, 1.0],
            coordinate_scales: [1.0, 1.0],
            scaled_step: [1.0, 0.0],
            candidate_raw_residual: [1.0, 0.0],
            moving_candidate_normalizers: [f64::NAN, 1.0],
        },
    );
    assert_eq!(
        precedence.expect_err("dual-invalid precedence"),
        "invalid normalizer"
    );

    // Both coordinates are exactly fixed at their lower bounds; f0=2 exceeds
    // every normalized root tolerance.  Draft04 requires DegenerateFace
    // before SVD, never a stationary-root result.
    let mut closed_jacobian = [[0.0; 21]; 21];
    closed_jacobian[0][0] = 1.0;
    let face = super::m1_trust_region_solve_subproblem_for_test(
        &input,
        &trial(),
        &super::M1TrustRegionSubproblemInput {
            raw_residual: [
                2.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,
                0.0, 0.0, 0.0, 0.0, 0.0,
            ],
            raw_jacobian: closed_jacobian,
            normalizers: [1.0; 21],
            scales: [1.0; 21],
            lower: [0.0; 21],
            upper: [0.0; 21],
            initial_radius: 1.0,
            base: [0.0; 21],
            test_work: super::M1TrustRegionTestWorkState::default(),
        },
    )
    .expect_err("no-free face refuses");
    assert_eq!(face.kind, super::M1TrustRegionRefusalKind::DegenerateFace);
    assert_eq!(face.attempted_subproblems, 1);
    assert_eq!(face.entered_svd_factorizations, 0);
    assert_exact_f64(face.raw_residual_max, 2.0);
    assert_eq!(input, before);
}

#[test]
fn m1_trust_region_exact_frozen_merit_operation_controls() {
    // Independent full-operation oracle: trust-qa-body10-operation-oracles.json.
    let original: Value = serde_json::from_str(ORIGINAL).expect("original input");
    let input = M1CoupledColumnInput::from_covered_column(
        covered_input(&original),
        [
            M1PhaseReservoirInput::new("stratum-z-upper::forest", 0.018, 0.0),
            M1PhaseReservoirInput::new("stratum-a-lower::forest", 0.018, 0.0),
        ],
        conditions(),
        topology_binding(),
    );
    let run = |raw_residual, raw_jacobian, frozen_normalizers, candidate_raw_residual| {
        super::m1_trust_region_frozen_merit_for_test(
            &input,
            &trial(),
            super::M1TrustRegionMeritInput {
                raw_residual,
                raw_jacobian,
                frozen_normalizers,
                coordinate_scales: [1.0, 1.0],
                scaled_step: [1.0, 0.0],
                candidate_raw_residual,
                moving_candidate_normalizers: [1.0, 1.0],
            },
        )
        .expect("finite oracle operands")
    };
    let subnormal = run(
        [f64::from_bits(0x1fe3_3104_4ec3_864c), 0.0],
        [[f64::from_bits(0x201e_26d2_a141_08c2), 0.0], [0.0, 0.0]],
        [1.0, 1.0],
        [f64::from_bits(0x2020_4679_958c_bcc6), 0.0],
    );
    assert_eq!(
        subnormal.predicted_reduction.to_bits(),
        0x8040_7724_962c_8071
    );
    assert_eq!(subnormal.actual_reduction.to_bits(), 0x8040_7724_962c_8070);
    assert_eq!(subnormal.rho.to_bits(), 0x3fef_ffff_ffff_fffe);
    let weighting = run(
        [f64::from_bits(0x3ff5_28cb_464b_b40c), 0.0],
        [[f64::from_bits(0xbff5_28cb_464b_b40c), 0.0], [0.0, 0.0]],
        [f64::from_bits(0x3ffd_e6e5_fd29_f054), 1.0],
        [0.0, 0.0],
    );
    assert_eq!(
        weighting.predicted_reduction.to_bits(),
        0x3fd0_0600_2cf1_57a5
    );
    assert_eq!(weighting.actual_reduction.to_bits(), 0x3fd0_0600_2cf1_57a5);
    assert_eq!(weighting.rho.to_bits(), 0x3ff0_0000_0000_0000);
}

#[test]
fn m1_trust_region_drainage_bound_uses_current_plus_step_and_retains_both_columns() {
    let original: Value = serde_json::from_str(ORIGINAL).expect("original input");
    let input = M1CoupledColumnInput::from_covered_column(
        covered_input(&original),
        [
            M1PhaseReservoirInput::new("stratum-z-upper::forest", 0.018, 0.0),
            M1PhaseReservoirInput::new("stratum-a-lower::forest", 0.018, 0.0),
        ],
        conditions(),
        topology_binding(),
    );
    let before = input.clone();
    // D_current=(2e-5,3e-5).  With S_D=1e-5, A=J*S and
    // A^T f=1e-5*(10,7).  Here ||A^T A||_2<=trace(A^T A)=1.5e-9 and the
    // unit-ball KKT equation gives lambda>=sqrt(149)*1e-5-1.5e-9.  Thus the
    // independently solved KKT root is lambda=0.00012206425078825946 and
    // p=(-0.8192316209801462,-0.5734627722757967).  These values include
    // A^T A; the normalized-gradient direction differs by 3e-7/4.28e-7 and
    // therefore cannot satisfy the 1e-9 discriminant below.  Both negative
    // steps remain feasible under D_current+dD, so a dD>=0 predicate would
    // reject a valid interior solution.  Two residual rows retain both
    // drainage columns and their cross-couplings.
    let mut raw_jacobian = [[0.0; 21]; 21];
    raw_jacobian[0][5] = 1.0;
    raw_jacobian[0][11] = 2.0;
    raw_jacobian[1][5] = 3.0;
    raw_jacobian[1][11] = 1.0;
    let mut base = [0.0; 21];
    base[5] = 2.0e-5;
    base[11] = 3.0e-5;
    let mut scales = [1.0; 21];
    scales[5] = 1.0e-5;
    scales[11] = 1.0e-5;
    let mut upper = [0.0; 21];
    let lower = [0.0; 21];
    upper[5] = f64::INFINITY;
    upper[11] = f64::INFINITY;
    let original_case = super::M1TrustRegionSubproblemInput {
        raw_residual: [
            2.2, 2.6, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,
            0.0, 0.0, 0.0, 0.0,
        ],
        raw_jacobian,
        normalizers: [1.0; 21],
        scales,
        lower,
        upper,
        initial_radius: 1.0,
        base,
        test_work: super::M1TrustRegionTestWorkState::default(),
    };
    let refusal =
        super::m1_trust_region_solve_subproblem_for_test(&input, &trial(), &original_case.clone())
            .expect_err("natural frozen ball endpoint refuses");
    assert_eq!(refusal.kind, super::M1TrustRegionRefusalKind::BallAccuracy);
    assert_eq!(refusal.attempted_subproblems, 1);
    assert_eq!(refusal.entered_svd_factorizations, 1);
    let mut weighted_case = original_case;
    weighted_case.normalizers[0] = 1.0e-4;
    weighted_case.normalizers[1] = 1.0e-4;
    let result = super::m1_trust_region_solve_subproblem_for_test(&input, &trial(), &weighted_case)
        .expect("weighted radius-limited feasible drainage step");
    assert!((result.lambda * 1.0e-8 - 0.000_122_064_250_788_259_46).abs() <= 1.0e-15);
    assert!((result.scaled_step[5] + 0.819_231_620_980_146_2).abs() <= 1.0e-9);
    assert!((result.scaled_step[11] + 0.573_462_772_275_796_7).abs() <= 1.0e-9);
    let step_norm_squared = result.scaled_step[5] * result.scaled_step[5]
        + result.scaled_step[11] * result.scaled_step[11];
    assert!((step_norm_squared - 1.0).abs() <= 2.0e-10);
    assert!(result.candidate_coordinates[5] > 0.0);
    assert!(result.candidate_coordinates[11] > 0.0);
    assert_exact_f64_slice(&result.raw_jacobian_columns[5][0..2], &[1.0, 3.0]);
    assert_exact_f64_slice(&result.raw_jacobian_columns[11][0..2], &[2.0, 1.0]);
    assert_eq!(input, before);
}

#[test]
fn m1_trust_region_nonunit_weights_change_the_actual_subproblem() {
    let original: Value = serde_json::from_str(ORIGINAL).expect("original input");
    let input = M1CoupledColumnInput::from_covered_column(
        covered_input(&original),
        [
            M1PhaseReservoirInput::new("stratum-z-upper::forest", 0.018, 0.0),
            M1PhaseReservoirInput::new("stratum-a-lower::forest", 0.018, 0.0),
        ],
        conditions(),
        topology_binding(),
    );
    // With rows (x,y,x+y), R=(1,1,1), and t=(1,1,2), W gives
    // A^T A=((5,1),(1,5))/4 and A^T f=(5,5)/4.  Thus p=(-5/6,-5/6).
    // Unweighted least squares would instead give (-2/3,-2/3).
    let mut raw_jacobian = [[0.0; 21]; 21];
    raw_jacobian[0][0] = 1.0;
    raw_jacobian[1][1] = 1.0;
    raw_jacobian[2][0] = 1.0;
    raw_jacobian[2][1] = 1.0;
    let mut normalizers = [1.0; 21];
    normalizers[2] = 2.0;
    let mut lower = [0.0; 21];
    lower[0] = f64::NEG_INFINITY;
    lower[1] = f64::NEG_INFINITY;
    let mut upper = [0.0; 21];
    upper[0] = f64::INFINITY;
    upper[1] = f64::INFINITY;
    let result = super::m1_trust_region_solve_subproblem_for_test(
        &input,
        &trial(),
        &super::M1TrustRegionSubproblemInput {
            raw_residual: [
                1.0, 1.0, 1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,
                0.0, 0.0, 0.0, 0.0, 0.0,
            ],
            raw_jacobian,
            normalizers,
            scales: [1.0; 21],
            lower,
            upper,
            initial_radius: 8.0,
            base: [0.0; 21],
            test_work: super::M1TrustRegionTestWorkState::default(),
        },
    )
    .expect("weighted subproblem");
    assert!((result.scaled_step[0] + 5.0 / 6.0).abs() <= 16.0 * f64::EPSILON);
    assert!((result.scaled_step[1] + 5.0 / 6.0).abs() <= 16.0 * f64::EPSILON);
    assert!(result.entered_svd_factorizations >= 1);
}

fn m1_stage1_two_coordinate_subproblem() -> super::M1TrustRegionSubproblemInput {
    let mut raw_jacobian = [[0.0; 21]; 21];
    raw_jacobian[0][0] = 1.0;
    raw_jacobian[1][1] = 1.0;
    let mut lower = [0.0; 21];
    let mut upper = [0.0; 21];
    lower[0] = f64::NEG_INFINITY;
    lower[1] = f64::NEG_INFINITY;
    upper[0] = f64::INFINITY;
    upper[1] = f64::INFINITY;
    super::M1TrustRegionSubproblemInput {
        raw_residual: [
            1.0, 1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,
            0.0, 0.0, 0.0, 0.0,
        ],
        raw_jacobian,
        normalizers: [1.0; 21],
        scales: [1.0; 21],
        lower,
        upper,
        initial_radius: 1.0,
        base: [0.0; 21],
        test_work: super::M1TrustRegionTestWorkState::default(),
    }
}

fn m1_stage1_numeric_refusal_cases() -> Vec<(
    super::M1TrustRegionSubproblemInput,
    super::M1TrustRegionRefusalKind,
)> {
    let mut nonfinite = m1_stage1_two_coordinate_subproblem();
    nonfinite.raw_residual[0] = f64::NAN;
    let mut nonfinite_normalizer = m1_stage1_two_coordinate_subproblem();
    nonfinite_normalizer.normalizers[0] = f64::NAN;
    let mut nonfinite_scale = m1_stage1_two_coordinate_subproblem();
    nonfinite_scale.scales[0] = f64::INFINITY;
    let mut product_overflow = m1_stage1_two_coordinate_subproblem();
    product_overflow.raw_jacobian[0][0] = f64::MAX;
    product_overflow.scales[0] = f64::MAX;
    let mut rank_deficient = m1_stage1_two_coordinate_subproblem();
    rank_deficient.raw_jacobian[1][1] = 2_f64.powi(-41);
    let mut zero_radius = m1_stage1_two_coordinate_subproblem();
    zero_radius.initial_radius = 0.0;
    let mut no_free_face = m1_stage1_two_coordinate_subproblem();
    no_free_face.lower[0] = 0.0;
    no_free_face.upper[0] = 0.0;
    no_free_face.lower[1] = 0.0;
    no_free_face.upper[1] = 0.0;
    let mut lambda_bracket = m1_stage1_two_coordinate_subproblem();
    lambda_bracket.raw_residual[0] = 2_f64.powi(100);
    lambda_bracket.raw_residual[1] = 2_f64.powi(100);
    lambda_bracket.initial_radius = 2_f64.powi(-20);
    let mut svd_sweep_cap = m1_stage1_two_coordinate_subproblem();
    svd_sweep_cap.raw_jacobian[0][1] = 1.0;
    svd_sweep_cap.test_work.completed_jacobi_sweeps = 64;
    let mut face_pivot_cap = m1_stage1_two_coordinate_subproblem();
    face_pivot_cap.raw_residual[0] = 1.0;
    face_pivot_cap.raw_residual[1] = 1.0;
    face_pivot_cap.initial_radius = 8.0;
    face_pivot_cap.lower[0] = -0.25;
    face_pivot_cap.lower[1] = -0.25;
    face_pivot_cap.test_work.completed_face_pivots = 42;
    vec![
        (nonfinite, super::M1TrustRegionRefusalKind::SvdNonFinite),
        (
            nonfinite_normalizer,
            super::M1TrustRegionRefusalKind::SvdNonFinite,
        ),
        (
            nonfinite_scale,
            super::M1TrustRegionRefusalKind::SvdNonFinite,
        ),
        (
            product_overflow,
            super::M1TrustRegionRefusalKind::SvdNonFinite,
        ),
        (
            rank_deficient,
            super::M1TrustRegionRefusalKind::RankDeficient,
        ),
        (zero_radius, super::M1TrustRegionRefusalKind::DegenerateFace),
        (
            no_free_face,
            super::M1TrustRegionRefusalKind::DegenerateFace,
        ),
        (
            lambda_bracket,
            super::M1TrustRegionRefusalKind::LambdaBracket,
        ),
        (
            svd_sweep_cap,
            super::M1TrustRegionRefusalKind::SvdNoConvergence,
        ),
        (
            face_pivot_cap,
            super::M1TrustRegionRefusalKind::FacePivotLimit,
        ),
    ]
}

#[test]
fn m1_trust_region_stage1_numeric_refusals_are_input_derived() {
    let original: Value = serde_json::from_str(ORIGINAL).expect("original input");
    let input = M1CoupledColumnInput::from_covered_column(
        covered_input(&original),
        [
            M1PhaseReservoirInput::new("stratum-z-upper::forest", 0.018, 0.0),
            M1PhaseReservoirInput::new("stratum-a-lower::forest", 0.018, 0.0),
        ],
        conditions(),
        topology_binding(),
    );
    let before = input.clone();

    for (case, expected) in m1_stage1_numeric_refusal_cases() {
        let refusal = super::m1_trust_region_solve_subproblem_for_test(&input, &trial(), &case)
            .expect_err("input-derived Stage 1 refusal");
        assert_eq!(refusal.kind, expected);
        assert_eq!(refusal.attempted_subproblems, 1);
        assert!(refusal.completed_jacobi_sweeps <= 64);
        assert!(refusal.completed_face_pivots <= 42);
        if expected == super::M1TrustRegionRefusalKind::SvdNoConvergence {
            assert_eq!(refusal.completed_jacobi_sweeps, 64);
            assert_eq!(refusal.entered_svd_factorizations, 1);
        }
        if expected == super::M1TrustRegionRefusalKind::SvdNonFinite {
            assert_eq!(refusal.entered_svd_factorizations, 0);
        }
        if expected == super::M1TrustRegionRefusalKind::FacePivotLimit {
            assert_eq!(refusal.completed_face_pivots, 42);
            assert_eq!(refusal.entered_svd_factorizations, 1);
        }
        assert_eq!(input, before);
    }
}

#[test]
fn m1_trust_region_svd_overflow_controls_are_real_face_operations() {
    // Independent QA operation oracle: trust-qa-body10-operation-oracles.json.
    let original: Value = serde_json::from_str(ORIGINAL).expect("original input");
    let input = M1CoupledColumnInput::from_covered_column(
        covered_input(&original),
        [
            M1PhaseReservoirInput::new("stratum-z-upper::forest", 0.018, 0.0),
            M1PhaseReservoirInput::new("stratum-a-lower::forest", 0.018, 0.0),
        ],
        conditions(),
        topology_binding(),
    );
    let before = input.clone();
    for (free, diagonal, sweeps) in [
        ([0usize, 1usize], [1.0e154, 1.0e154], 0u8),
        ([0usize, 0usize], [1.0e155, 0.0], 1u8),
    ] {
        let mut lower = [0.0; 21];
        let mut upper = [0.0; 21];
        for coordinate in free {
            lower[coordinate] = f64::NEG_INFINITY;
            upper[coordinate] = f64::INFINITY;
        }
        let mut raw_jacobian = [[0.0; 21]; 21];
        raw_jacobian[0][free[0]] = diagonal[0];
        raw_jacobian[1][free[1]] = diagonal[1];
        let refusal = super::m1_trust_region_solve_subproblem_for_test(
            &input,
            &trial(),
            &super::M1TrustRegionSubproblemInput {
                raw_residual: [0.0; 21],
                raw_jacobian,
                normalizers: [1.0; 21],
                scales: [1.0; 21],
                lower,
                upper,
                initial_radius: 1.0,
                base: [0.0; 21],
                test_work: super::M1TrustRegionTestWorkState::default(),
            },
        )
        .expect_err("real SVD overflow refusal");
        assert_eq!(refusal.kind, super::M1TrustRegionRefusalKind::SvdNonFinite);
        assert_eq!(refusal.attempted_subproblems, 1);
        assert_eq!(refusal.entered_svd_factorizations, 1);
        assert_eq!(refusal.completed_jacobi_sweeps, sweeps);
        assert_eq!(input, before);
    }
}

#[test]
fn m1_trust_region_exact_bound_starts_move_only_inward_and_preserve_start() {
    let original: Value = serde_json::from_str(ORIGINAL).expect("original input");
    let input = M1CoupledColumnInput::from_covered_column(
        covered_input(&original),
        [
            M1PhaseReservoirInput::new("stratum-z-upper::forest", 0.018, 0.0),
            M1PhaseReservoirInput::new("stratum-a-lower::forest", 0.018, 0.0),
        ],
        conditions(),
        topology_binding(),
    );
    let before = input.clone();
    let mut lower = [0.0; 21];
    let mut upper = [0.0; 21];
    lower[0] = 0.0;
    upper[0] = 1.0;
    lower[1] = 0.0;
    upper[1] = 1.0;
    let mut lower_start = m1_stage1_two_coordinate_subproblem();
    lower_start.lower = lower;
    lower_start.upper = upper;
    lower_start.base[0] = 0.0;
    lower_start.raw_residual[0] = -1.0;
    let lower_result =
        super::m1_trust_region_solve_subproblem_for_test(&input, &trial(), &lower_start)
            .expect("exact lower start admits inward descent");
    assert_eq!(
        lower_result.initial_face_step.map(f64::to_bits),
        [0.0_f64.to_bits(); 21],
        "the lower-face start is exactly p=+0 in every coordinate"
    );
    assert_eq!(lower_result.initial_lower_mask, [false; 21]);
    assert_eq!(lower_result.initial_upper_mask, [false; 21]);
    assert_eq!(lower_result.initial_free_mask[0..2], [true, true]);
    assert!(lower_result.initial_free_mask[2..].iter().all(|free| !free));
    assert_exact_f64(lower_result.base_coordinates[0], 0.0);
    assert!(lower_result.candidate_coordinates[0] > 0.0);
    let mut upper_start = m1_stage1_two_coordinate_subproblem();
    upper_start.lower = lower;
    upper_start.upper = upper;
    upper_start.base[0] = 1.0;
    upper_start.raw_residual[0] = 1.0;
    let upper_result =
        super::m1_trust_region_solve_subproblem_for_test(&input, &trial(), &upper_start)
            .expect("exact upper start admits inward descent");
    assert_eq!(
        upper_result.initial_face_step.map(f64::to_bits),
        [0.0_f64.to_bits(); 21],
        "the upper-face start is exactly p=+0 in every coordinate"
    );
    assert_eq!(upper_result.initial_lower_mask, [false; 21]);
    assert_eq!(upper_result.initial_upper_mask, [false; 21]);
    assert_eq!(upper_result.initial_free_mask[0..2], [true, true]);
    assert!(upper_result.initial_free_mask[2..].iter().all(|free| !free));
    assert_exact_f64(upper_result.base_coordinates[0], 1.0);
    assert!(upper_result.candidate_coordinates[0] < 1.0);
    assert_eq!(input, before);
}

#[test]
fn m1_trust_region_real_post_bisection_ball_guard_has_boundary_pass_and_fail() {
    // The override is a coordinate-only post-bisection test seam.  The actual
    // factorization, bracket endpoints, 48 midpoint evaluations and final
    // upper endpoint precede it; shared validation computes the norm, gap and
    // tolerance after it.  It cannot supply a verdict or a work count.
    for (radius, endpoint, accepted) in [
        (1.0, None, true),
        (1.0, Some([1.0 - 2_f64.powi(-40)]), true),
        (1.0, Some([1.0 - 2_f64.powi(-40) - 2_f64.powi(-53)]), false),
        (1.0, Some([1.0 + 2_f64.powi(-52)]), false),
        (
            2_f64.powi(-10),
            Some([2_f64.powi(-10) - 2_f64.powi(-46)]),
            true,
        ),
        (
            2_f64.powi(-10),
            Some([2_f64.powi(-10) - 2_f64.powi(-46) - 2_f64.powi(-63)]),
            false,
        ),
    ] {
        let result = super::m1_trust_region_validate_ball_endpoint_for_test(
            super::M1TrustRegionBallEndpoint {
                a: 1.0,
                f: -2.0 * radius,
                radius,
                endpoint_override: endpoint,
            },
        );
        assert!(result.factorization_entered);
        assert_eq!(result.accepted, accepted);
        assert_eq!(result.endpoint_hook_observed, endpoint);
        assert_exact_f64_slice(&result.bracket_initial, &[0.0, 1.0]);
        assert_exact_f64_slice(&result.bracket_final, &[1.0 - 2_f64.powi(-48), 1.0]);
        assert_exact_f64(result.final_upper_lambda, 1.0);
        assert_eq!(result.bracket_endpoint_evaluations.len(), 2);
        assert_exact_f64(result.bracket_endpoint_evaluations[0].lambda, 0.0);
        assert_exact_f64(result.bracket_endpoint_evaluations[1].lambda, 1.0);
        assert!(!result.bracket_endpoint_evaluations[0].feasible);
        assert!(result.bracket_endpoint_evaluations[1].feasible);
        assert_eq!(result.bisection_evaluations.len(), 48);
        for (index, event) in result.bisection_evaluations.iter().enumerate() {
            let k = i32::try_from(index + 1).expect("bisection index fits i32");
            assert_exact_f64(event.lambda, 1.0 - 2_f64.powi(-k));
            assert_exact_f64(event.norm, 2.0 * radius / (1.0 + event.lambda));
            assert!(!event.feasible);
        }
        assert_exact_f64(result.natural_upper_endpoint, radius);
        assert_exact_f64(result.pre_hook_endpoint, radius);
        assert_exact_f64_slice(&result.post_hook_endpoint, &endpoint.unwrap_or([radius]));
        let expected_endpoint = endpoint.unwrap_or([radius])[0];
        assert_exact_f64(result.computed_norm, expected_endpoint.abs());
        assert_exact_f64(result.computed_gap, radius - expected_endpoint.abs());
        assert_exact_f64(
            result.computed_tolerance,
            (2_f64.powi(-40) * radius).max(64.0 * f64::EPSILON * radius.max(1.0)),
        );
        if accepted {
            assert_eq!(result.kind, None);
        } else {
            assert_eq!(
                result.kind,
                Some(super::M1TrustRegionRefusalKind::BallAccuracy)
            );
        }
    }
}

#[test]
fn m1_trust_region_jacobi_pair_order_and_singular_sort_are_observed() {
    assert_m1_trust_region_jacobi_pair_order();
    assert_m1_trust_region_singular_sort();
}

fn assert_m1_trust_region_jacobi_pair_order() {
    let rotation =
        super::m1_trust_region_jacobi_observation_for_test(super::M1TrustRegionJacobiInput {
            matrix: [[1.0, 1.0, 0.0], [1.0, 0.0, 1.0], [0.0, 1.0, 1.0]],
            free_ids: [0, 1, 2],
        });
    assert_eq!(rotation.completed_sweeps, 2);
    assert_eq!(rotation.rotations, 2);
    assert_eq!(
        rotation.pair_visits,
        [
            super::M1TrustRegionJacobiPair {
                left: 0,
                right: 1,
                rotated: true,
                sweep: 1
            },
            super::M1TrustRegionJacobiPair {
                left: 0,
                right: 2,
                rotated: false,
                sweep: 1
            },
            super::M1TrustRegionJacobiPair {
                left: 1,
                right: 2,
                rotated: true,
                sweep: 1
            },
            super::M1TrustRegionJacobiPair {
                left: 0,
                right: 1,
                rotated: false,
                sweep: 2
            },
            super::M1TrustRegionJacobiPair {
                left: 0,
                right: 2,
                rotated: false,
                sweep: 2
            },
            super::M1TrustRegionJacobiPair {
                left: 1,
                right: 2,
                rotated: false,
                sweep: 2
            },
        ]
    );
    let expected_b: [[f64; 3]; 3] = [
        [0.0, 2.0_f64.sqrt().recip(), -2.0_f64.sqrt().recip()],
        [
            2.0 / 3.0_f64.sqrt(),
            2.0 / 3.0_f64.sqrt(),
            2.0 / 3.0_f64.sqrt(),
        ],
        [
            -(2.0_f64 / 3.0).sqrt(),
            1.0 / 6.0_f64.sqrt(),
            1.0 / 6.0_f64.sqrt(),
        ],
    ];
    let expected_v: [[f64; 3]; 3] = [
        [2.0_f64.sqrt().recip(), -2.0_f64.sqrt().recip(), 0.0],
        [
            3.0_f64.sqrt().recip(),
            3.0_f64.sqrt().recip(),
            3.0_f64.sqrt().recip(),
        ],
        [
            -1.0 / 6.0_f64.sqrt(),
            -1.0 / 6.0_f64.sqrt(),
            (2.0_f64 / 3.0).sqrt(),
        ],
    ];
    for (actual, expected) in rotation.b_columns.iter().zip(expected_b) {
        for (actual, expected) in actual.iter().zip(expected) {
            assert!((*actual - expected).abs() <= 64.0 * f64::EPSILON);
        }
    }
    for (actual, expected) in rotation.v_columns.iter().zip(expected_v) {
        for (actual, expected) in actual.iter().zip(expected) {
            assert!((*actual - expected).abs() <= 64.0 * f64::EPSILON);
        }
    }
}

fn assert_m1_trust_region_singular_sort() {
    let sorted =
        super::m1_trust_region_jacobi_observation_for_test(super::M1TrustRegionJacobiInput {
            matrix: [[1.0, 0.0, 0.0], [0.0, 2.0, 0.0], [0.0, 0.0, 1.0]],
            free_ids: [2, 5, 11],
        });
    assert_eq!(sorted.completed_sweeps, 1);
    assert_eq!(sorted.pair_visits.len(), 3);
    assert!(sorted.pair_visits.iter().all(|pair| !pair.rotated));
    assert_exact_f64_slice(&sorted.unsorted_singular_values, &[1.0, 2.0, 1.0]);
    assert_exact_f64_slice(&sorted.sorted_singular_values, &[2.0, 1.0, 1.0]);
    assert_eq!(sorted.sorted_free_ids, [5, 2, 11]);
    assert_eq!(sorted.sorted_from_unsorted, [1, 0, 2]);
    for (sorted_index, unsorted_index) in sorted.sorted_from_unsorted.iter().enumerate() {
        assert_eq!(
            sorted.sorted_b_columns[sorted_index].map(f64::to_bits),
            sorted.b_columns[*unsorted_index].map(f64::to_bits)
        );
        assert_eq!(
            sorted.sorted_v_columns[sorted_index].map(f64::to_bits),
            sorted.v_columns[*unsorted_index].map(f64::to_bits)
        );
    }
}

#[test]
fn m1_trust_region_exact_jacobi_hypot_control() {
    // Independent QA oracle: trust-qa-body10-operation-oracles.json supplies
    // this operand matrix and binary64 result; it is not derived here from
    // the candidate factorization.
    let actual =
        super::m1_trust_region_jacobi_observation_for_test(super::M1TrustRegionJacobiInput {
            matrix: [
                [1.0, 1.0, 0.0],
                [0.0, 0.793_419_855_996_930_8, 0.0],
                [0.0, 0.0, 1.0],
            ],
            free_ids: [0, 1, 2],
        });
    assert_eq!(actual.v_columns[0][0].to_bits(), 0x3fe9_cd33_90e9_28b2);
    assert_eq!(actual.pair_visits.len(), 6);
}

fn m1_kkt_fixture() -> (M1CoupledColumnInput, [[f64; 21]; 21]) {
    let original: Value = serde_json::from_str(ORIGINAL).expect("original input");
    let input = M1CoupledColumnInput::from_covered_column(
        covered_input(&original),
        [
            M1PhaseReservoirInput::new("stratum-z-upper::forest", 0.018, 0.0),
            M1PhaseReservoirInput::new("stratum-a-lower::forest", 0.018, 0.0),
        ],
        conditions(),
        topology_binding(),
    );
    let mut jacobian = [[0.0; 21]; 21];
    jacobian[0][0] = 1.0;
    jacobian[0][2] = -1.0;
    jacobian[1][1] = 1.0;
    jacobian[1][3] = -1.0;
    jacobian[2][2] = 1.0;
    jacobian[3][3] = 1.0;
    jacobian[4][4] = 1.0;
    (input, jacobian)
}

fn m1_assert_close(actual: &[f64], expected: &[f64]) {
    for (actual, expected) in actual.iter().zip(expected) {
        assert!((actual - expected).abs() <= 128.0 * f64::EPSILON);
    }
}

fn m1_active_bound_release_result(
    input: &M1CoupledColumnInput,
    jacobian: &[[f64; 21]; 21],
) -> super::M1TrustRegionSubproblemObservation {
    let mut upper = [0.0; 21];
    for upper_bound in upper.iter_mut().take(5) {
        *upper_bound = f64::INFINITY;
    }
    super::m1_trust_region_solve_subproblem_for_test(
        input,
        &trial(),
        &super::M1TrustRegionSubproblemInput {
            raw_residual: [
                -1.0, -1.0, 2.0, 2.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,
                0.0, 0.0, 0.0, 0.0, 0.0,
            ],
            raw_jacobian: *jacobian,
            normalizers: [1.0; 21],
            scales: [1.0; 21],
            lower: [0.0; 21],
            upper,
            initial_radius: 8.0,
            base: [0.0; 21],
            test_work: super::M1TrustRegionTestWorkState::default(),
        },
    )
    .expect("KKT face release")
}

fn assert_active_bound_initial_and_activations(r: &super::M1TrustRegionSubproblemObservation) {
    m1_assert_close(&r.scaled_step[..5], &[1.0, 1.0, 0.0, 0.0, 0.0]);
    m1_assert_close(&r.final_residual[..5], &[0.0, 0.0, 2.0, 2.0, 0.0]);
    m1_assert_close(&r.final_g[..5], &[0.0, 0.0, 2.0, 2.0, 0.0]);
    m1_assert_close(&r.final_lambda_times_p[..5], &[0.0; 5]);
    m1_assert_close(&r.final_h[..5], &[0.0, 0.0, 2.0, 2.0, 0.0]);
    assert_exact_f64(r.lambda, 0.0);
    assert_eq!(r.initial_lower_mask, [false; 21]);
    assert_eq!(r.initial_upper_mask, [false; 21]);
    assert_eq!(r.initial_free_mask[..5], [true; 5]);
    assert!(r.initial_free_mask[5..].iter().all(|free| !free));
    assert_eq!(
        r.initial_face_step.map(f64::to_bits),
        [0.0_f64.to_bits(); 21]
    );
    let expected = [
        (super::M1TrustRegionKktAction::ActivateLower, 0),
        (super::M1TrustRegionKktAction::ActivateLower, 1),
        (super::M1TrustRegionKktAction::ActivateLower, 2),
        (super::M1TrustRegionKktAction::ActivateLower, 3),
        (super::M1TrustRegionKktAction::ReleaseLower, 0),
        (super::M1TrustRegionKktAction::ReleaseLower, 1),
        (super::M1TrustRegionKktAction::Return, usize::MAX),
    ];
    assert_eq!(r.event_trace.len(), expected.len());
    for (event, (action, coordinate)) in r.event_trace.iter().zip(expected) {
        assert_eq!(event.action, action);
        assert_eq!(event.selected_coordinate, coordinate);
    }
    for (index, event) in r.event_trace[..4].iter().enumerate() {
        let mut lower = [false; 21];
        lower[..=index].fill(true);
        let mut free = [false; 21];
        free[..5].fill(true);
        free[..=index].fill(false);
        assert_eq!(event.lower_mask, lower);
        assert_eq!(event.upper_mask, [false; 21]);
        assert_eq!(event.free_mask, free);
        assert_eq!(event.g, None);
        assert_eq!(event.lambda_times_p, None);
        assert_eq!(event.h, None);
    }
}

fn assert_active_bound_release_faces(r: &super::M1TrustRegionSubproblemObservation) {
    let a = &r.event_trace[4];
    let b = &r.event_trace[5];
    let c = &r.event_trace[6];
    assert_eq!(a.lower_mask[..5], [true, true, true, true, false]);
    assert_eq!(a.upper_mask[..5], [false; 5]);
    assert_eq!(a.free_mask[..5], [false, false, false, false, true]);
    assert!(a.lower_mask[5..].iter().all(|v| !v));
    assert!(a.upper_mask[5..].iter().all(|v| !v));
    assert!(a.free_mask[5..].iter().all(|v| !v));
    m1_assert_close(
        &a.g.as_ref().expect("box-feasible face computes g")[..5],
        &[-1.0, -1.0, 3.0, 3.0, 0.0],
    );
    m1_assert_close(
        &a.lambda_times_p
            .as_ref()
            .expect("box-feasible face computes lambda*p")[..5],
        &[0.0; 5],
    );
    m1_assert_close(
        &a.h.as_ref().expect("box-feasible face computes h")[..5],
        &[-1.0, -1.0, 3.0, 3.0, 0.0],
    );
    assert_eq!(b.lower_mask[..5], [false, true, true, true, false]);
    assert_eq!(b.upper_mask, [false; 21]);
    assert_eq!(b.free_mask[..5], [true, false, false, false, true]);
    assert!(b.lower_mask[5..].iter().all(|v| !v));
    assert!(b.free_mask[5..].iter().all(|v| !v));
    m1_assert_close(
        &b.g.as_ref().expect("box-feasible face computes g")[..5],
        &[0.0, -1.0, 2.0, 3.0, 0.0],
    );
    m1_assert_close(
        &b.lambda_times_p
            .as_ref()
            .expect("box-feasible face computes lambda*p")[..5],
        &[0.0; 5],
    );
    m1_assert_close(
        &b.h.as_ref().expect("box-feasible face computes h")[..5],
        &[0.0, -1.0, 2.0, 3.0, 0.0],
    );
    assert_eq!(c.lower_mask[..5], [false, false, true, true, false]);
    assert_eq!(c.upper_mask, [false; 21]);
    assert_eq!(c.free_mask[..5], [true, true, false, false, true]);
    assert!(c.lower_mask[5..].iter().all(|v| !v));
    assert!(c.free_mask[5..].iter().all(|v| !v));
    m1_assert_close(
        &c.g.as_ref().expect("return face computes g")[..5],
        &[0.0, 0.0, 2.0, 2.0, 0.0],
    );
    m1_assert_close(
        &c.lambda_times_p
            .as_ref()
            .expect("return face computes lambda*p")[..5],
        &[0.0; 5],
    );
    m1_assert_close(
        &c.h.as_ref().expect("return face computes h")[..5],
        &[0.0, 0.0, 2.0, 2.0, 0.0],
    );
    assert_eq!(r.entered_svd_factorizations, 7);
    assert_eq!(r.completed_face_pivots, 6);
}

fn m1_kkt_variant_result(
    input: &M1CoupledColumnInput,
    jacobian: &[[f64; 21]; 21],
    raw: [f64; 5],
    lo: f64,
    hi: f64,
) -> super::M1TrustRegionSubproblemObservation {
    let mut residual = [0.0; 21];
    residual[..5].copy_from_slice(&raw);
    let mut lower = [0.0; 21];
    let mut upper = [0.0; 21];
    for (l, u) in lower.iter_mut().zip(upper.iter_mut()).take(4) {
        *l = lo;
        *u = hi;
    }
    upper[4] = f64::INFINITY;
    lower[4] = f64::NEG_INFINITY;
    super::m1_trust_region_solve_subproblem_for_test(
        input,
        &trial(),
        &super::M1TrustRegionSubproblemInput {
            raw_residual: residual,
            raw_jacobian: *jacobian,
            normalizers: [1.0; 21],
            scales: [1.0; 21],
            lower,
            upper,
            initial_radius: 8.0,
            base: [0.0; 21],
            test_work: super::M1TrustRegionTestWorkState::default(),
        },
    )
    .expect("actual KKT face chronology")
}

fn assert_kkt_variant_setup(
    r: &super::M1TrustRegionSubproblemObservation,
    lo: f64,
    initial_h: [f64; 5],
    releases: [usize; 2],
) -> Vec<&super::M1TrustRegionKktEvent> {
    let mut active_upper = [false; 21];
    if !lo.is_finite() {
        active_upper[2] = true;
        active_upper[3] = true;
    }
    assert_eq!(r.active_upper, active_upper);
    let events = r
        .event_trace
        .iter()
        .filter(|e| {
            matches!(
                e.action,
                super::M1TrustRegionKktAction::ReleaseLower
                    | super::M1TrustRegionKktAction::ReleaseUpper
            )
        })
        .collect::<Vec<_>>();
    assert_eq!(events.len(), 2);
    let action = if lo.is_finite() {
        super::M1TrustRegionKktAction::ReleaseLower
    } else {
        super::M1TrustRegionKktAction::ReleaseUpper
    };
    assert_eq!(events[0].action, action);
    assert_eq!(events[1].action, action);
    assert_eq!(
        events
            .iter()
            .map(|e| e.selected_coordinate)
            .collect::<Vec<_>>(),
        releases
    );
    m1_assert_close(
        &events[0].g.as_ref().expect("box-feasible face computes g")[..5],
        &initial_h,
    );
    m1_assert_close(
        &events[0]
            .lambda_times_p
            .as_ref()
            .expect("box-feasible face computes lambda*p")[..5],
        &[0.0; 5],
    );
    m1_assert_close(
        &events[0].h.as_ref().expect("box-feasible face computes h")[..5],
        &initial_h,
    );
    assert_eq!(r.initial_lower_mask, [false; 21]);
    assert_eq!(r.initial_upper_mask, [false; 21]);
    assert_eq!(r.initial_free_mask[..5], [true; 5]);
    assert_eq!(
        r.initial_face_step.map(f64::to_bits),
        [0.0_f64.to_bits(); 21]
    );
    assert_eq!(
        r.event_trace[..4]
            .iter()
            .map(|e| e.selected_coordinate)
            .collect::<Vec<_>>(),
        [0, 1, 2, 3]
    );
    let activation = if lo.is_finite() {
        super::M1TrustRegionKktAction::ActivateLower
    } else {
        super::M1TrustRegionKktAction::ActivateUpper
    };
    for (i, e) in r.event_trace[..4].iter().enumerate() {
        assert_eq!(e.action, activation);
        let mut a = [false; 21];
        a[..=i].fill(true);
        let mut f = [false; 21];
        f[..5].fill(true);
        f[..=i].fill(false);
        if lo.is_finite() {
            assert_eq!(e.lower_mask, a);
            assert_eq!(e.upper_mask, [false; 21]);
        } else {
            assert_eq!(e.lower_mask, [false; 21]);
            assert_eq!(e.upper_mask, a);
        }
        assert_eq!(e.free_mask, f);
        assert_eq!(e.g, None);
        assert_eq!(e.lambda_times_p, None);
        assert_eq!(e.h, None);
    }
    events
}

fn assert_kkt_variant_release_and_return(
    r: &super::M1TrustRegionSubproblemObservation,
    lo: f64,
    step: [f64; 5],
    second_h: [f64; 5],
    releases: [usize; 2],
    final_h: [f64; 5],
    events: &[&super::M1TrustRegionKktEvent],
) {
    m1_assert_close(
        &events[1].g.as_ref().expect("box-feasible face computes g")[..5],
        &second_h,
    );
    m1_assert_close(
        &events[1]
            .lambda_times_p
            .as_ref()
            .expect("box-feasible face computes lambda*p")[..5],
        &[0.0; 5],
    );
    m1_assert_close(
        &events[1].h.as_ref().expect("box-feasible face computes h")[..5],
        &second_h,
    );
    if lo.is_finite() {
        assert_eq!(events[0].lower_mask[..5], [true, true, true, true, false]);
        assert_eq!(events[0].upper_mask[..5], [false; 5]);
    } else {
        assert_eq!(events[0].lower_mask[..5], [false; 5]);
        assert_eq!(events[0].upper_mask[..5], [true, true, true, true, false]);
    }
    assert_eq!(events[0].free_mask[..5], [false, false, false, false, true]);
    let mut a = [false; 21];
    a[..4].fill(true);
    a[releases[0]] = false;
    let mut f = [false; 21];
    f[..5].fill(true);
    for (i, v) in f.iter_mut().take(4).enumerate() {
        if i != releases[0] {
            *v = false;
        }
    }
    if lo.is_finite() {
        assert_eq!(events[1].lower_mask, a);
        assert_eq!(events[1].upper_mask, [false; 21]);
    } else {
        assert_eq!(events[1].lower_mask, [false; 21]);
        assert_eq!(events[1].upper_mask, a);
    }
    assert_eq!(events[1].free_mask, f);
    for e in events {
        assert!(e.lower_mask[5..].iter().all(|v| !v));
        assert!(e.upper_mask[5..].iter().all(|v| !v));
        assert!(e.free_mask[5..].iter().all(|v| !v));
    }
    m1_assert_close(&r.scaled_step[..5], &step);
    m1_assert_close(&r.final_h[..5], &final_h);
    let e = r.event_trace.last().expect("return event");
    assert_eq!(e.action, super::M1TrustRegionKktAction::Return);
    assert_eq!(e.selected_coordinate, usize::MAX);
    if lo.is_finite() {
        assert_eq!(e.lower_mask[..5], [false, false, true, true, false]);
        assert_eq!(e.upper_mask, [false; 21]);
    } else {
        assert_eq!(e.lower_mask, [false; 21]);
        assert_eq!(e.upper_mask[..5], [false, false, true, true, false]);
    }
    assert_eq!(e.free_mask[..5], [true, true, false, false, true]);
    assert!(e.lower_mask[5..].iter().all(|v| !v));
    assert!(e.upper_mask[5..].iter().all(|v| !v));
    assert!(e.free_mask[5..].iter().all(|v| !v));
    m1_assert_close(
        &e.g.as_ref().expect("return face computes g")[..5],
        &final_h,
    );
    m1_assert_close(
        &e.lambda_times_p
            .as_ref()
            .expect("return face computes lambda*p")[..5],
        &[0.0; 5],
    );
    m1_assert_close(
        &e.h.as_ref().expect("return face computes h")[..5],
        &final_h,
    );
    assert_exact_f64(r.lambda, 0.0);
    assert_eq!(r.completed_face_pivots, 6);
    assert_eq!(r.entered_svd_factorizations, 7);
}
#[test]
fn m1_trust_region_active_bound_release_and_tie_follow_real_kkt_faces() {
    let (input, jacobian) = m1_kkt_fixture();
    let result = m1_active_bound_release_result(&input, &jacobian);
    assert_active_bound_initial_and_activations(&result);
    assert_active_bound_release_faces(&result);
}

#[test]
fn m1_trust_region_kkt_upper_sign_and_largest_violation_variants_are_observed() {
    let (input, jacobian) = m1_kkt_fixture();
    for (raw, lo, hi, step, initial_h, second_h, releases, final_h) in [
        (
            [1.0, 1.0, -2.0, -2.0, 0.0],
            f64::NEG_INFINITY,
            0.0,
            [-1.0, -1.0, 0.0, 0.0, 0.0],
            [1.0, 1.0, -3.0, -3.0, 0.0],
            [0.0, 1.0, -2.0, -3.0, 0.0],
            [0, 1],
            [0.0, 0.0, -2.0, -2.0, 0.0],
        ),
        (
            [-1.0, -2.0, 2.0, 4.0, 0.0],
            0.0,
            f64::INFINITY,
            [1.0, 2.0, 0.0, 0.0, 0.0],
            [-1.0, -2.0, 3.0, 6.0, 0.0],
            [-1.0, 0.0, 3.0, 4.0, 0.0],
            [1, 0],
            [0.0, 0.0, 2.0, 4.0, 0.0],
        ),
    ] {
        let result = m1_kkt_variant_result(&input, &jacobian, raw, lo, hi);
        let events = assert_kkt_variant_setup(&result, lo, initial_h, releases);
        assert_kkt_variant_release_and_return(
            &result, lo, step, second_h, releases, final_h, &events,
        );
    }
}

// These retained controller controls are not compiled or credited in the
// Stage 1 cut.  Stage 2 must replace their unresolved placeholders with
// observable call/state/counter controls before enabling this private switch.
#[cfg(feature = "m1-trust-region-controller-stage")]
#[test]
fn m1_trust_region_proposal_and_root_paths_bind_real_state_and_all_thresholds() {
    let original: Value = serde_json::from_str(ORIGINAL).expect("original input");
    let input = M1CoupledColumnInput::from_covered_column(
        covered_input(&original),
        [
            M1PhaseReservoirInput::new("stratum-z-upper::forest", 0.018, 0.0),
            M1PhaseReservoirInput::new("stratum-a-lower::forest", 0.018, 0.0),
        ],
        conditions(),
        topology_binding(),
    );
    let before = input.clone();
    let domain = super::m1_trust_region_with_trial_domain_poison_for_test(&input, &trial());
    assert_eq!(domain.radius_before, 1.0);
    assert_eq!(domain.radius_after, 0.25);
    assert_eq!(
        domain.base_raw_jacobian_digest,
        domain.retained_raw_jacobian_digest
    );
    assert_eq!(domain.base_normalizer_bits, domain.retained_normalizer_bits);
    assert_eq!(domain.base_selection, domain.retained_selection);
    assert_eq!(input, before);

    let terminal = super::m1_trust_region_with_selected_side_error_for_test(&input, &trial());
    assert_eq!(terminal.error.code(), "VEG-E-142");
    assert_eq!(terminal.radius_before, terminal.radius_after);
    assert_eq!(input, before);

    for path in [
        super::M1TrustRegionWitnessPath::InstalledRoot,
        super::M1TrustRegionWitnessPath::FullNoUpdate,
        super::M1TrustRegionWitnessPath::FirstValidReplacement,
    ] {
        let witness = super::m1_trust_region_witness_thresholds_for_test(&input, &trial(), path);
        assert_eq!(witness.raw_checks.len(), 21);
        assert_eq!(witness.normalized_checks.len(), 21);
        assert_eq!(witness.governed_norms.len(), super::M1_GOVERNED_STEP_NORMS);
        assert!(witness.every_boundary_accepts);
        assert!(witness.each_governed_norm_poison_refuses);
        assert!(witness.each_raw_poison_refuses);
        assert!(witness.each_normalized_poison_refuses);
        assert!(witness.materialization_uses_exact_preinstall_snapshot);
        assert!(witness.owner_state_unchanged_on_refusal);
    }
}

#[cfg(feature = "m1-trust-region-controller-stage")]
#[test]
fn m1_trust_region_phase_counts_refusals_and_caps_are_exact() {
    let original: Value = serde_json::from_str(ORIGINAL).expect("original input");
    let input = M1CoupledColumnInput::from_covered_column(
        covered_input(&original),
        [
            M1PhaseReservoirInput::new("stratum-z-upper::forest", 0.018, 0.0),
            M1PhaseReservoirInput::new("stratum-a-lower::forest", 0.018, 0.0),
        ],
        conditions(),
        topology_binding(),
    );
    let phase = super::m1_trust_region_phase_capacity_tie_for_test(&input, &trial());
    assert_eq!(phase.natural_assemblies, 1);
    assert_eq!(phase.selected_assemblies, 1);
    assert_eq!(phase.predictor_selected_probe_entries, 1);
    assert_eq!(phase.final_selected_probe_entries, 1);
    assert_eq!(phase.drainage_columns.len(), 2);
    assert!(
        phase
            .drainage_columns
            .iter()
            .all(|column| column.iter().any(|x| *x != 0.0))
    );
    assert!(
        phase
            .radius_limited_drainage
            .iter()
            .all(|step| *step >= 0.0)
    );

    let natural = super::m1_trust_region_natural_tangent_for_test(&input, &trial());
    assert_eq!(natural.natural_assemblies, 1);
    assert_eq!(natural.selected_assemblies, 0);
    assert_eq!(natural.predictor_selected_probe_entries, 0);
    assert_eq!(natural.final_selected_probe_entries, 1);

    for (limit, attempted, entered) in [
        (21, 22, 21),
        (50, 51, 50),
        (22, 23, 22),
        (100, 101, 100),
        (6500, 6501, 6500),
        (2202, 2203, 2202),
        (47300, 47301, 47300),
        (1, 2, 1),
    ] {
        let refusal = super::m1_trust_region_cap_boundary_for_test(&input, &trial(), limit);
        assert_eq!(refusal.kind, super::M1TrustRegionRefusalKind::WorkCap);
        assert_eq!(refusal.attempted, attempted);
        assert_eq!(refusal.entered, entered);
        assert!(refusal.pre_state_equals_post_state);
    }
    let exhausted = super::m1_trust_region_radius_exhaustion_for_test(&input, &trial());
    assert_eq!(
        exhausted.kind,
        super::M1TrustRegionRefusalKind::RadiusExhausted
    );
    assert_eq!(exhausted.last_admitted_radius, 2_f64.powi(-20));
    assert_eq!(exhausted.next_radius, 2_f64.powi(-22));
}

#[cfg(feature = "m1-trust-region-controller-stage")]
#[test]
fn m1_trust_region_every_subproblem_refusal_has_an_input_derived_kind_and_count() {
    let original: Value = serde_json::from_str(ORIGINAL).expect("original input");
    let input = M1CoupledColumnInput::from_covered_column(
        covered_input(&original),
        [
            M1PhaseReservoirInput::new("stratum-z-upper::forest", 0.018, 0.0),
            M1PhaseReservoirInput::new("stratum-a-lower::forest", 0.018, 0.0),
        ],
        conditions(),
        topology_binding(),
    );
    // Each matrix/vector is the direct numerical cause of the indicated
    // refusal: NaN operand; 65 required rotations; sigma_min=0; no lambda
    // bracket within 48 x4 expansions; ball error above max(2^-40*r,64eps);
    // and 43 bound crossings.  These are inputs, never pre-labelled answers.
    for (assembly, expected) in [
        (
            super::M1TrustRegionTestAssembly::nonfinite_entry(),
            super::M1TrustRegionRefusalKind::SvdNonFinite,
        ),
        (
            super::M1TrustRegionTestAssembly::requires_65_jacobi_sweeps(),
            super::M1TrustRegionRefusalKind::SvdNoConvergence,
        ),
        (
            super::M1TrustRegionTestAssembly::singular_values([1.0, 0.0]),
            super::M1TrustRegionRefusalKind::RankDeficient,
        ),
        (
            super::M1TrustRegionTestAssembly::unbracketed_lambda(),
            super::M1TrustRegionRefusalKind::LambdaBracket,
        ),
        (
            super::M1TrustRegionTestAssembly::inaccurate_ball_endpoint(),
            super::M1TrustRegionRefusalKind::BallAccuracy,
        ),
        (
            super::M1TrustRegionTestAssembly::forty_three_crossing_face(),
            super::M1TrustRegionRefusalKind::FacePivotLimit,
        ),
    ] {
        let refusal =
            super::m1_trust_region_with_controlled_assembly_for_test(&input, &trial(), assembly)
                .expect_err("input-derived subproblem refusal");
        assert_eq!(refusal.kind, expected);
        assert_eq!(refusal.attempted_subproblems, 1);
        assert_eq!(refusal.entered_subproblems, 1);
        assert!(refusal.pre_state_equals_post_state);
    }
}

#[test]
fn m1_finite_unbounded_ci_remains_diagnostic_at_the_real_halved_seam() {
    let original: Value = serde_json::from_str(ORIGINAL).expect("original input");
    let input = M1CoupledColumnInput::from_covered_column(
        covered_input(&original),
        [
            M1PhaseReservoirInput::new("stratum-z-upper::forest", 0.018, 0.0),
            M1PhaseReservoirInput::new("stratum-a-lower::forest", 0.018, 0.0),
        ],
        conditions(),
        topology_binding(),
    );
    let outcomes = super::m1_solve_with_first_halved_step_poison_for_test(&input, 5);
    assert!(
        outcomes
            .iter()
            .any(|(exponent, no_update)| *exponent == 1 && *no_update),
        "finite ci is diagnostic and cannot add a threshold: {outcomes:?}"
    );
}

#[test]
fn m1_no_update_records_exact_b1_and_b2_cumulative_backtracking() {
    let original: Value = serde_json::from_str(ORIGINAL).expect("original input");
    let input = M1CoupledColumnInput::from_covered_column(
        covered_input(&original),
        [
            M1PhaseReservoirInput::new("stratum-z-upper::forest", 0.018, 0.0),
            M1PhaseReservoirInput::new("stratum-a-lower::forest", 0.018, 0.0),
        ],
        conditions(),
        topology_binding(),
    );
    for exponent in [1, 2] {
        let activity = super::m1_solve_with_no_update_exponent_for_test(&input, exponent);
        assert_eq!(
            activity.no_update_cumulative_backtracking,
            Some(exponent),
            "exact examined exponent is retained privately"
        );
        let steps = activity
            .last_step_norms
            .expect("actual b1/b2 candidate diagnostics");
        assert!(steps.beta.expect("beta").is_finite());
        assert!(steps.ci_pa.expect("ci").is_finite());
        let current = activity
            .last_current_coordinates
            .expect("current coordinates");
        let candidate = activity
            .last_candidate_coordinates
            .expect("candidate coordinates");
        assert_ne!(current[13].to_bits(), candidate[13].to_bits());
    }
}

fn m1_poisoned_installed_step_input() -> M1CoupledColumnInput {
    let original: Value = serde_json::from_str(ORIGINAL).expect("original input");
    M1CoupledColumnInput::from_covered_column(
        covered_input(&original),
        [
            M1PhaseReservoirInput::new("stratum-z-upper::forest", 0.018, 0.0),
            M1PhaseReservoirInput::new("stratum-a-lower::forest", 0.018, 0.0),
        ],
        conditions(),
        topology_binding(),
    )
}

#[test]
fn m1_postinstalled_beta_guard_prevents_materialization() {
    let (result, activity) = super::m1_solve_with_poisoned_installed_step_for_test(
        &m1_poisoned_installed_step_input(),
        0,
    );
    assert!(result.is_err());
    assert_eq!(activity.materialization_entries, 0);
}

#[test]
fn m1_postinstalled_ci_guard_prevents_materialization() {
    let (result, activity) = super::m1_solve_with_poisoned_installed_step_for_test(
        &m1_poisoned_installed_step_input(),
        1,
    );
    assert!(result.is_err());
    assert_eq!(activity.materialization_entries, 0);
}

#[test]
fn m1_postinstalled_passing_diagnostic_materializes() {
    let (result, activity) = super::m1_solve_with_poisoned_installed_step_for_test(
        &m1_poisoned_installed_step_input(),
        2,
    );
    assert!(
        result.is_ok(),
        "passing stored diagnostic must reach ordinary acceptance"
    );
    assert!(activity.materialization_entries > 0);
}

#[test]
fn m1_full_witness_has_priority_without_halved_admission() {
    let original: Value = serde_json::from_str(ORIGINAL).expect("original input");
    let input = M1CoupledColumnInput::from_covered_column(
        covered_input(&original),
        [
            M1PhaseReservoirInput::new("stratum-z-upper::forest", 0.018, 0.0),
            M1PhaseReservoirInput::new("stratum-a-lower::forest", 0.018, 0.0),
        ],
        conditions(),
        topology_binding(),
    );
    let outcomes = super::m1_solve_with_full_witness_for_test(&input);
    assert!(
        outcomes.contains(&(0, true)),
        "the complete full candidate must win before any halved witness: {outcomes:?}"
    );
    assert!(
        !outcomes.iter().any(|(exponent, _)| *exponent > 0),
        "full priority must not enter halved witness admission: {outcomes:?}"
    );
}

#[test]
fn m1_actual_candidate_current_norms_use_rounded_primitives_and_derived_beta_ci() {
    let original: Value = serde_json::from_str(ORIGINAL).expect("original input");
    let input = M1CoupledColumnInput::from_covered_column(
        covered_input(&original),
        [
            M1PhaseReservoirInput::new("stratum-z-upper::forest", 0.018, 0.0),
            M1PhaseReservoirInput::new("stratum-a-lower::forest", 0.018, 0.0),
        ],
        conditions(),
        topology_binding(),
    );
    let activity = super::m1_solve_with_no_update_exponent_for_test(&input, 2);
    let steps = activity.last_step_norms.expect("actual b2 step norms");
    let current = activity.last_current_coordinates.expect("actual current");
    let candidate = activity
        .last_candidate_coordinates
        .expect("actual candidate");
    let direction = activity.last_direction.expect("actual linear direction");
    let factor = activity.last_factor.expect("actual b2 factor");
    let actual_q = candidate[13] - current[13];
    let nominal_q = factor * direction[13];
    assert_ne!(
        actual_q.to_bits(),
        nominal_q.to_bits(),
        "binary64 candidate-current displacement must not be replaced by nominal factor*direction"
    );
    assert_eq!(
        steps.humidity_kg_kg.expect("q norm").to_bits(),
        actual_q.abs().to_bits()
    );

    let dry_current = activity
        .last_current_dry_temperatures
        .expect("current dry primitives");
    let dry_candidate = activity
        .last_candidate_dry_temperatures
        .expect("candidate dry primitives");
    let wet_current = activity
        .last_current_wet_temperatures
        .expect("current wet primitives");
    let wet_candidate = activity
        .last_candidate_wet_temperatures
        .expect("candidate wet primitives");
    let dry = dry_current
        .iter()
        .zip(dry_candidate)
        .map(|(before, after)| (after - before).abs())
        .fold(0.0_f64, f64::max);
    let wet = wet_current
        .iter()
        .zip(wet_candidate)
        .map(|(before, after)| match (before, after) {
            (Some(before), Some(after)) => (after - before).abs(),
            (None, None) => 0.0,
            _ => f64::NAN,
        })
        .fold(0.0_f64, f64::max);
    assert_eq!(
        steps.temperature_k.expect("T norm").to_bits(),
        dry.max(wet).to_bits()
    );

    for (name, current, candidate, observed) in [
        (
            "beta",
            activity.last_current_beta,
            activity.last_candidate_beta,
            steps.beta,
        ),
        (
            "ci",
            activity.last_current_ci_pa,
            activity.last_candidate_ci_pa,
            steps.ci_pa,
        ),
    ] {
        let expected = current
            .expect("current evaluator primitives")
            .iter()
            .zip(candidate.expect("candidate evaluator primitives"))
            .map(|(before, after)| (after - before).abs())
            .fold(0.0_f64, f64::max);
        assert_eq!(
            observed.expect("derived diagnostic").to_bits(),
            expected.to_bits(),
            "{name} must derive from actual candidate/current evaluator primitives"
        );
    }
}

#[test]
fn m1_runtime_observation_is_current_solve_only_without_external_reset() {
    let original: Value = serde_json::from_str(ORIGINAL).expect("original input");
    let input = M1CoupledColumnInput::from_covered_column(
        covered_input(&original),
        [
            M1PhaseReservoirInput::new("stratum-z-upper::forest", 0.018, 0.0),
            M1PhaseReservoirInput::new("stratum-a-lower::forest", 0.018, 0.0),
        ],
        conditions(),
        topology_binding(),
    );
    super::m1_reset_loop_activity_observation_for_test();
    solve_m1_coupled_column(&input).expect("first complete solve");
    let first = super::m1_loop_activity_observation_for_test();
    solve_m1_coupled_column(&input).expect("second complete solve");
    let second = super::m1_loop_activity_observation_for_test();
    assert_eq!(
        second.complete_evaluator_entries,
        first.complete_evaluator_entries
    );
    assert_eq!(second.hydraulic_entries, first.hydraulic_entries);
    assert_eq!(second.trial_entries, first.trial_entries);
    assert_eq!(
        second.materialization_entries,
        first.materialization_entries
    );
    assert_eq!(second.predictor_assemblies, first.predictor_assemblies);
    assert_eq!(second.final_assemblies, first.final_assemblies);
}

#[test]
fn m1_opt_in_diagnostic_observation_records_one_real_solve_and_then_drops() {
    let original: Value = serde_json::from_str(ORIGINAL).expect("original input");
    let input = M1CoupledColumnInput::from_covered_column(
        covered_input(&original),
        [
            M1PhaseReservoirInput::new("stratum-z-upper::forest", 0.018, 0.0),
            M1PhaseReservoirInput::new("stratum-a-lower::forest", 0.018, 0.0),
        ],
        conditions(),
        topology_binding(),
    );
    let unobserved = solve_m1_coupled_column(&input).expect("unobserved complete solve");
    let observation_guard = input.enable_one_m1_diagnostic_observation();
    let observed = solve_m1_coupled_column(&input).expect("observed complete solve");
    assert_eq!(observed, unobserved);
    let observation = input
        .take_m1_diagnostic_observation()
        .expect("one enabled solve has a bounded record");
    assert_eq!(observation.input, input);
    assert_eq!(observation.status, M1DiagnosticCaptureStatus::Complete);
    let seed = observation
        .seed_coordinates
        .expect("post-preflight canonical seed");
    assert_eq!(seed[5], 0.0);
    assert_eq!(seed[11], 0.0);
    assert_eq!(observation.terminal_error_code, None);
    assert!(!observation.bases.is_empty());
    assert!(!observation.trials.is_empty());
    assert!(!observation.linear_solves.is_empty());
    assert!(observation.bases.iter().all(|base| {
        base.raw_residuals.len() == 21
            && base.normalized_residuals.len() == 21
            && base.normalizers.len() == 21
            && base.normalized_jacobian.len() == 21
            && base.rhs.len() == 21
    }));
    assert!(observation.linear_solves.iter().all(|solve| {
        solve.full_matrix.len() == solve.full_rhs.len()
            && solve.reduced_matrix.len() == solve.reduced_rhs.len()
            && solve.retained_indices.len() == solve.reduced_rhs.len()
            && solve.full_direction.len() == solve.full_rhs.len()
            && solve.reduced_direction.len() == solve.reduced_rhs.len()
            && solve.retained_indices.len() + solve.eliminated_indices.len() == solve.full_rhs.len()
    }));
    let disabled = solve_m1_coupled_column(&input).expect("disabled subsequent solve");
    assert_eq!(disabled, unobserved);
    assert!(input.take_m1_diagnostic_observation().is_none());
    drop(observation_guard);
}

#[test]
fn m1_diagnostic_observation_cancels_mismatch_and_rearm_is_a_new_generation() {
    let original: Value = serde_json::from_str(ORIGINAL).expect("original input");
    let input = M1CoupledColumnInput::from_covered_column(
        covered_input(&original),
        [
            M1PhaseReservoirInput::new("stratum-z-upper::forest", 0.018, 0.0),
            M1PhaseReservoirInput::new("stratum-a-lower::forest", 0.018, 0.0),
        ],
        conditions(),
        topology_binding(),
    );
    let mut mismatch = input.clone();
    mismatch.reservoirs[0].mass_kg_m2 = 0.019;
    let mismatch_guard = input.enable_one_m1_diagnostic_observation();
    solve_m1_coupled_column(&mismatch).expect("mismatched physical solve");
    assert!(input.take_m1_diagnostic_observation().is_none());
    solve_m1_coupled_column(&input).expect("cancelled generation cannot rearm itself");
    assert!(input.take_m1_diagnostic_observation().is_none());
    drop(mismatch_guard);

    let rearmed_guard = input.enable_one_m1_diagnostic_observation();
    solve_m1_coupled_column(&input).expect("rearmed physical solve");
    assert!(input.take_m1_diagnostic_observation().is_some());
    drop(rearmed_guard);

    let dropped_guard = input.enable_one_m1_diagnostic_observation();
    solve_m1_coupled_column(&input).expect("drop-bound physical solve");
    drop(dropped_guard);
    assert!(input.take_m1_diagnostic_observation().is_none());
}

#[test]
fn m1_diagnostic_capture_fault_preserves_preflight_error() {
    let original: Value = serde_json::from_str(ORIGINAL).expect("original input");
    let mut input = M1CoupledColumnInput::from_covered_column(
        covered_input(&original),
        [
            M1PhaseReservoirInput::new("stratum-z-upper::forest", 0.018, 0.0),
            M1PhaseReservoirInput::new("stratum-a-lower::forest", 0.018, 0.0),
        ],
        conditions(),
        topology_binding(),
    );
    input.topology.occupancy_ids.swap(0, 1);
    let baseline = solve_m1_coupled_column(&input).expect_err("baseline preflight refusal");
    super::m1_set_diagnostic_capture_fault_for_test(true, false);
    let guard = input.enable_one_m1_diagnostic_observation();
    let observed = solve_m1_coupled_column(&input).expect_err("observer cannot replace refusal");
    assert_eq!(observed, baseline);
    assert_eq!(
        input
            .take_m1_diagnostic_observation()
            .expect("fault record")
            .status,
        M1DiagnosticCaptureStatus::Incomplete(super::M1DiagnosticCaptureReason::InjectedFault)
    );
    drop(guard);
    super::m1_set_diagnostic_capture_fault_for_test(false, true);
    let overflow_guard = input.enable_one_m1_diagnostic_observation();
    let overflow_error =
        solve_m1_coupled_column(&input).expect_err("overflow cannot replace refusal");
    assert_eq!(overflow_error, baseline);
    assert_eq!(
        input
            .take_m1_diagnostic_observation()
            .expect("overflow record")
            .status,
        M1DiagnosticCaptureStatus::Incomplete(super::M1DiagnosticCaptureReason::Capacity)
    );
    drop(overflow_guard);
    super::m1_set_diagnostic_capture_fault_for_test(false, false);
}

#[test]
fn m1_opt_in_diagnostic_observation_retains_physical_preflight_error() {
    let original: Value = serde_json::from_str(ORIGINAL).expect("original input");
    let mut input = M1CoupledColumnInput::from_covered_column(
        covered_input(&original),
        [
            M1PhaseReservoirInput::new("stratum-z-upper::forest", 0.018, 0.0),
            M1PhaseReservoirInput::new("stratum-a-lower::forest", 0.018, 0.0),
        ],
        conditions(),
        topology_binding(),
    );
    input.topology.occupancy_ids.swap(0, 1);
    let unobserved = solve_m1_coupled_column(&input).expect_err("unobserved preflight refusal");
    let observation_guard = input.enable_one_m1_diagnostic_observation();
    let error = solve_m1_coupled_column(&input).expect_err("physical preflight refusal");
    assert_eq!(error, unobserved);
    let observation = input
        .take_m1_diagnostic_observation()
        .expect("failure has the enabled record");
    assert_eq!(observation.status, M1DiagnosticCaptureStatus::Complete);
    assert_eq!(
        observation.terminal_error_code.as_deref(),
        Some(error.code())
    );
    assert!(observation.bases.is_empty());
    drop(observation_guard);
}

#[test]
fn m1_complete_success_observes_real_evaluator_hydraulic_trial_materialization_work() {
    let original: Value = serde_json::from_str(ORIGINAL).expect("original input");
    let input = M1CoupledColumnInput::from_covered_column(
        covered_input(&original),
        [
            M1PhaseReservoirInput::new("stratum-z-upper::forest", 0.018, 0.0),
            M1PhaseReservoirInput::new("stratum-a-lower::forest", 0.018, 0.0),
        ],
        conditions(),
        topology_binding(),
    );
    super::m1_reset_loop_activity_observation_for_test();
    solve_m1_coupled_column(&input).expect("actual complete M1 success");
    let activity = super::m1_loop_activity_observation_for_test();
    assert!(activity.complete_evaluator_entries > 0);
    assert!(activity.hydraulic_entries > 0);
    assert!(activity.actual_hydraulic_entries > activity.hydraulic_entries);
    assert!(activity.trial_entries > 0);
    assert!(activity.materialization_entries > 0);
    assert!(activity.predictor_assemblies > 0);
    assert!(
        activity
            .reduced_linear_dimension_counts
            .iter()
            .any(|count| *count > 0)
    );
}

fn decimal(value: &Value) -> f64 {
    value.as_str().expect("decimal").parse().expect("valid")
}

fn structural_identity_matrix() -> Vec<Vec<f64>> {
    (0..21)
        .map(|row| (0..21).map(|column| f64::from(row == column)).collect())
        .collect()
}

fn structural_metadata() -> super::M1DrainageMetadataTestView {
    super::M1DrainageMetadataTestView {
        rows: [5, 11],
        raw_row_identity: [5, 11],
        drainage_selected: [true, true],
        duration_s: [60.0, 60.0],
        support_s: [60.0, 60.0],
        row_scales: [60.0, 60.0],
        current_drainage: [0.0, 0.0],
        raw_drainage_residual: [0.0, 0.0],
        raw_drainage_diagonal: [60.0, 60.0],
    }
}

/// Independent test oracle for the valid, selected-affine boundary.  Each
/// poison case starts from this complete 21-coordinate system, so its expected
/// red is attributable to precisely the field it changes.
fn assert_structural_baseline(
    matrix: &[Vec<f64>],
    rhs: &[f64],
    metadata: &super::M1DrainageMetadataTestView,
) {
    assert_eq!(matrix.len(), 21);
    assert!(matrix.iter().all(|row| row.len() == 21));
    assert_eq!(rhs.len(), 21);
    for slot in 0..2 {
        let row = metadata.rows[slot];
        assert!(metadata.drainage_selected[slot]);
        assert!(row < 21);
        assert_eq!(metadata.raw_row_identity[slot], row);
        assert!(metadata.duration_s[slot].is_finite() && metadata.duration_s[slot] > 0.0);
        assert!(metadata.support_s[slot].is_finite() && metadata.support_s[slot] > 0.0);
        assert!(metadata.row_scales[slot].is_finite() && metadata.row_scales[slot] > 0.0);
        assert_eq!(
            metadata.raw_drainage_diagonal[slot].to_bits(),
            metadata.duration_s[slot].to_bits()
        );
        assert_eq!(
            metadata.raw_drainage_residual[slot].to_bits(),
            (metadata.duration_s[slot] * metadata.current_drainage[slot]).to_bits(),
        );
        assert_eq!(
            matrix[row][row].to_bits(),
            (metadata.raw_drainage_diagonal[slot] / metadata.row_scales[slot]).to_bits(),
        );
        let expected_rhs = -metadata.raw_drainage_residual[slot] / metadata.row_scales[slot];
        assert!(
            rhs[row].to_bits() == expected_rhs.to_bits()
                || (rhs[row] == 0.0 && expected_rhs == 0.0),
            "normalized RHS row {row}"
        );
    }
}

#[test]
fn m1_combined_support_and_raw_domain_guards_preserve_precedence() {
    let radiation: Value = serde_json::from_str(RADIATION).expect("radiation");
    let original: Value = serde_json::from_str(ORIGINAL).expect("original");
    let unsupported = radiation_input_with_conditions(
        &radiation,
        &original,
        M1SupportConditions {
            occupancy_conditions: vec![M1OccupancySupportCondition {
                occupancy_id: "stratum-z-upper::forest".to_owned(),
                liquid_conducting: true,
                full_supply: false,
            }],
            ..conditions()
        },
        f64::NAN,
    );
    let error = evaluate_m1_coupled_column(&unsupported, &trial()).expect_err("support first");
    assert_eq!(error.code(), "VEG-E-140");
    let wrong_model = radiation_input_with_conditions(
        &radiation,
        &original,
        M1SupportConditions {
            model_definition_id: "wrong-model".to_owned(),
            ..conditions()
        },
        decimal(&radiation["inputs"]["p"]),
    );
    let error = evaluate_m1_coupled_column(&wrong_model, &trial()).expect_err("model identity");
    assert_eq!(error.code(), "VEG-E-140");
    let raw_invalid =
        radiation_input_with_conditions(&radiation, &original, conditions(), f64::NAN);
    let error = evaluate_m1_coupled_column(&raw_invalid, &trial()).expect_err("raw input");
    assert_eq!(error.code(), "VEG-E-141");
    for support in [
        M1SupportConditions {
            occupancy_conditions: vec![M1OccupancySupportCondition {
                occupancy_id: "stratum-z-upper::forest".to_owned(),
                liquid_conducting: true,
                full_supply: true,
            }],
            ..conditions()
        },
        M1SupportConditions {
            occupancy_conditions: vec![
                M1OccupancySupportCondition {
                    occupancy_id: "stratum-z-upper::forest".to_owned(),
                    liquid_conducting: true,
                    full_supply: true,
                },
                M1OccupancySupportCondition {
                    occupancy_id: "stratum-z-upper::forest".to_owned(),
                    liquid_conducting: true,
                    full_supply: true,
                },
            ],
            ..conditions()
        },
        M1SupportConditions {
            occupancy_conditions: vec![
                M1OccupancySupportCondition {
                    occupancy_id: "stratum-z-upper::forest".to_owned(),
                    liquid_conducting: true,
                    full_supply: true,
                },
                M1OccupancySupportCondition {
                    occupancy_id: "unknown-occupancy".to_owned(),
                    liquid_conducting: true,
                    full_supply: true,
                },
            ],
            ..conditions()
        },
    ] {
        let invalid = radiation_input_with_conditions(&radiation, &original, support, f64::NAN);
        assert_eq!(
            evaluate_m1_coupled_column(&invalid, &trial())
                .expect_err("support precedence")
                .code(),
            "VEG-E-140"
        );
    }
}

#[test]
fn m1_inactive_sun_rows_freeze_current_anchor_in_newton_system() {
    let original: Value = serde_json::from_str(ORIGINAL).expect("original input");
    let column = covered_input(&original);
    let mut coordinates = [column.air_temperature_k; 21];
    coordinates[3] = 0.018;
    coordinates[4] = 0.0;
    coordinates[5] = 0.0;
    coordinates[9] = 0.018;
    coordinates[10] = 0.0;
    coordinates[11] = 0.0;
    coordinates[13] = column.air_specific_humidity_kg_kg;
    coordinates[14] = column
        .stage3_lower_boundary
        .as_ref()
        .expect("snow boundary")
        .snow_temperature_k;
    for (index, node) in column.ground.soil_nodes.iter().enumerate() {
        coordinates[15 + index] = node.beginning_temperature_k;
    }
    let input = M1CoupledColumnInput::from_covered_column(
        column,
        [
            M1PhaseReservoirInput::new("stratum-z-upper::forest", 0.018, 0.0),
            M1PhaseReservoirInput::new("stratum-a-lower::forest", 0.018, 0.0),
        ],
        conditions(),
        topology_binding(),
    );
    for canopy_temperature in [266.85496453757526, 273.15, 280.0] {
        coordinates[12] = canopy_temperature;
        let evaluation = evaluate_m1_coupled_column(
            &input,
            &M1CoupledColumnTrial::from_coordinates(coordinates),
        )
        .expect("valid inactive-anchor control");
        for column_index in 0..21 {
            let column = evaluation.jacobian_column(column_index).expect("column");
            for inactive_row in [0, 6] {
                assert_eq!(
                    column[inactive_row],
                    if column_index == inactive_row {
                        1.0
                    } else {
                        0.0
                    }
                );
            }
        }
    }
}

#[test]
fn m1_release_support_timing_observes_inner_solve_and_real_work() {
    let original: Value = serde_json::from_str(ORIGINAL).expect("original input");
    let input = M1CoupledColumnInput::from_covered_column(
        covered_input(&original),
        [
            M1PhaseReservoirInput::new("stratum-z-upper::forest", 0.018, 0.0),
            M1PhaseReservoirInput::new("stratum-a-lower::forest", 0.018, 0.0),
        ],
        conditions(),
        topology_binding(),
    );
    super::m1_reset_loop_activity_observation_for_test();
    let started = std::time::Instant::now();
    let result = solve_m1_coupled_column(&input).expect("admitted original60 solve");
    let inner_wall_s = started.elapsed().as_secs_f64();
    let activity = super::m1_loop_activity_observation_for_test();
    assert_eq!(result.support_duration_s(), 60.0);
    assert!(activity.complete_evaluator_entries > 0);
    assert!(activity.hydraulic_entries > 0);
    assert!(activity.predictor_assemblies > 0);
    assert!(
        activity
            .reduced_linear_dimension_counts
            .iter()
            .any(|count| *count > 0)
    );
    eprintln!(
        "m1-release-support-timing inner_wall_s={inner_wall_s:.9} evaluator={} hydraulics={} trials={} materialization={} predictor={} final={} reduced_dimensions={:?} cpu_unresolved=true cfg_test_trace_overhead=true",
        activity.complete_evaluator_entries,
        activity.hydraulic_entries,
        activity.trial_entries,
        activity.materialization_entries,
        activity.predictor_assemblies,
        activity.final_assemblies,
        activity.reduced_linear_dimension_counts,
    );
}

#[test]
fn m1_original_sixty_second_solve_has_original_geometry_active_cold_shade_and_real_closure() {
    let original: Value = serde_json::from_str(ORIGINAL).expect("original input");
    let solved = solve_m1_coupled_column(&M1CoupledColumnInput::from_covered_column(
        covered_input(&original),
        [
            M1PhaseReservoirInput::new("stratum-z-upper::forest", 0.018, 0.0),
            M1PhaseReservoirInput::new("stratum-a-lower::forest", 0.018, 0.0),
        ],
        conditions(),
        topology_binding(),
    ))
    .expect("actual solve");
    assert_eq!(solved.support_duration_s(), 60.);
    assert!(
        solved
            .occupancies()
            .iter()
            .all(|o| o.shade.gas_branch == "ExactZeroPar"
                && o.shade.net_assimilation < 0.
                && o.shade.vapor_kg_m2_tile_s > 0.)
    );
    for operand in solved.reservoir_operands() {
        assert_eq!(operand.beginning_mass_kg_m2, 0.018);
        assert_eq!(operand.beginning_enthalpy_j_m2, 0.0);
        assert!(operand.ending_mass_kg_m2.is_finite() && operand.ending_enthalpy_j_m2.is_finite());
        assert!(operand.vapor_kg_m2_s != 0.0 && operand.non_vapor_heat_w_m2 != 0.0);
        assert!(
            operand.ending_mass_kg_m2 != operand.beginning_mass_kg_m2
                || operand.ending_enthalpy_j_m2 != operand.beginning_enthalpy_j_m2
        );
    }
    independently_reconstruct_reservoirs(&solved, true);
}

#[test]
fn m1_guard_expected_red_accepted_materialization_does_not_rebuild_jacobian() {
    let original: Value = serde_json::from_str(ORIGINAL).expect("original input");
    let input = M1CoupledColumnInput::from_covered_column(
        covered_input(&original),
        [
            M1PhaseReservoirInput::new("stratum-z-upper::forest", 0.018, 0.0),
            M1PhaseReservoirInput::new("stratum-a-lower::forest", 0.018, 0.0),
        ],
        conditions(),
        topology_binding(),
    );
    super::m1_reset_work_observation_for_test();
    solve_m1_coupled_column(&input).expect("known accepted M1 control");
    let (jacobian_entries, controller_assemblies) = super::m1_work_observation_for_test();
    assert_eq!(controller_assemblies, 5, "known accepted controller work");
    assert_eq!(
        jacobian_entries, controller_assemblies,
        "expected-red: accepted materialization must reuse its accepted Core rather than rebuild FD Jacobian"
    );
}

#[test]
fn m1_guard_accepted_output_has_no_diagnostic_jacobian() {
    let original: Value = serde_json::from_str(ORIGINAL).expect("original input");
    let input = M1CoupledColumnInput::from_covered_column(
        covered_input(&original),
        [
            M1PhaseReservoirInput::new("stratum-z-upper::forest", 0.018, 0.0),
            M1PhaseReservoirInput::new("stratum-a-lower::forest", 0.018, 0.0),
        ],
        conditions(),
        topology_binding(),
    );
    let accepted = solve_m1_coupled_column(&input).expect("known accepted M1 control");
    assert_eq!(
        accepted
            .jacobian()
            .expect_err("accepted result has no diagnostic Jacobian")
            .code(),
        "LSEB-E-030"
    );
}

#[test]
fn m1_guard_expected_red_actual_materializer_rejects_published_and_accepted_state_poisons() {
    let original: Value = serde_json::from_str(ORIGINAL).expect("original input");
    let input = M1CoupledColumnInput::from_covered_column(
        covered_input(&original),
        [
            M1PhaseReservoirInput::new("stratum-z-upper::forest", 0.018, 0.0),
            M1PhaseReservoirInput::new("stratum-a-lower::forest", 0.018, 0.0),
        ],
        conditions(),
        topology_binding(),
    );
    super::m1_reset_accepted_capture_for_test();
    let accepted = solve_m1_coupled_column(&input).expect("actual accepted boundary capture");
    assert!(
        accepted.jacobian().is_err(),
        "captured boundary is actual None-Jacobian materialization"
    );
    let after_solve = super::m1_work_observation_for_test();
    super::m1_materialize_captured_accepted_for_test()
        .expect("unpoisoned captured accepted boundary");
    let (overcap_liquid, overcap_capacity, overcap_mass) =
        super::m1_captured_finite_overcapacity_core_for_test()
            .expect("finite one-ULP liquid overcapacity remains evaluable before acceptance");
    assert!(overcap_mass > 0.0 && overcap_liquid > overcap_capacity);
    let (
        beginning_mass,
        cached_mass_residual,
        cached_enthalpy_residual,
        poisoned_non_vapor,
        liquid,
        capacity,
    ) = super::m1_captured_accepted_guard_binding_for_test();
    assert_eq!(beginning_mass, 0.018, "captured accepted M0");
    assert!(
        cached_mass_residual.abs() < 1.0e-9,
        "accepted raw mass residual"
    );
    assert!(
        5.0e-10 < 1.0e-9 && 5.0e-10 / beginning_mass > 1.0e-8,
        "scaled mass poison binds while absolute tolerance passes"
    );
    assert!(cached_enthalpy_residual.is_finite());
    assert!(
        (poisoned_non_vapor - 1.0).is_finite(),
        "ledger poison changes the real non-vapor operand by one W m^-2"
    );
    assert!(
        liquid <= capacity,
        "captured accepted liquid remains within its unpoisoned capacity"
    );
    assert_eq!(
        super::m1_work_observation_for_test(),
        after_solve,
        "materializer seam must not add controller or Jacobian work"
    );
    let mut unexpected = Vec::new();
    for (poison, code) in [
        (
            super::M1AcceptedMaterializationPoison::NonfiniteResidual,
            "VEG-E-141",
        ),
        (
            super::M1AcceptedMaterializationPoison::NonfiniteScale,
            "VEG-E-141",
        ),
        (
            super::M1AcceptedMaterializationPoison::NonfiniteLeaf,
            "VEG-E-141",
        ),
        (
            super::M1AcceptedMaterializationPoison::NonfiniteReservoirWetFraction,
            "VEG-E-141",
        ),
        (
            super::M1AcceptedMaterializationPoison::NonfiniteReservoirWetTemperature,
            "VEG-E-141",
        ),
        (
            super::M1AcceptedMaterializationPoison::NonfiniteReservoirCapacity,
            "VEG-E-141",
        ),
        (
            super::M1AcceptedMaterializationPoison::NonfiniteHydraulic,
            "VEG-E-141",
        ),
        (
            super::M1AcceptedMaterializationPoison::NonfiniteHydraulicFlux,
            "VEG-E-141",
        ),
        (
            super::M1AcceptedMaterializationPoison::NonfiniteHydraulicContinuity,
            "VEG-E-141",
        ),
        (
            super::M1AcceptedMaterializationPoison::NonfiniteReservoirOperand,
            "VEG-E-141",
        ),
        (
            super::M1AcceptedMaterializationPoison::NonfiniteDuration,
            "VEG-E-141",
        ),
        (
            super::M1AcceptedMaterializationPoison::EmptyMass,
            "VEG-E-142",
        ),
        (
            super::M1AcceptedMaterializationPoison::NegativeDrainage,
            "VEG-E-142",
        ),
        (
            super::M1AcceptedMaterializationPoison::OverCapacityOneUlp,
            "VEG-E-142",
        ),
        (
            super::M1AcceptedMaterializationPoison::RawMassResidual,
            "VEG-E-142",
        ),
        (
            super::M1AcceptedMaterializationPoison::RawEnthalpyResidual,
            "VEG-E-142",
        ),
        (
            super::M1AcceptedMaterializationPoison::RawCapacity,
            "VEG-E-142",
        ),
        (
            super::M1AcceptedMaterializationPoison::ActualCapacityResidual,
            "VEG-E-142",
        ),
        (
            super::M1AcceptedMaterializationPoison::NonfiniteCapacity,
            "VEG-E-141",
        ),
        (
            super::M1AcceptedMaterializationPoison::ScaledMass,
            "VEG-E-142",
        ),
        (
            super::M1AcceptedMaterializationPoison::EnthalpyLedger,
            "VEG-E-142",
        ),
    ] {
        match super::m1_materialize_captured_accepted_poison_for_test(poison) {
            Ok(_) => unexpected.push(format!("{poison:?}: accepted, expected {code}")),
            Err(error) if error.code() != code => {
                unexpected.push(format!("{poison:?}: {}, expected {code}", error.code()))
            }
            Err(_) => {}
        }
    }
    assert!(
        unexpected.is_empty(),
        "expected-red actual accepted materializer guards: {unexpected:?}"
    );
}

#[test]
fn m1_guard_expected_red_empty_positive_components_reject_external_supersaturation() {
    let radiation: Value = serde_json::from_str(RADIATION).expect("radiation input");
    let original: Value = serde_json::from_str(ORIGINAL).expect("original input");
    let source = radiation_input(&radiation, &original);
    let mut coordinates = super::m1_trial_coordinates_for_test(&trial());
    coordinates[3] = 0.0;
    coordinates[4] = 0.0;
    let mut positive_outcomes = Vec::new();
    let mut zero_outcomes = Vec::new();
    for component in ["sun", "shade", "stem"] {
        let target_coordinate = match component {
            "sun" => 0,
            "shade" => 1,
            "stem" => 2,
            _ => unreachable!(),
        };
        for target_temperature in [263.15, 283.15] {
            let mut positive_column = source.column.clone();
            let occupancy = &mut positive_column.occupancies[0];
            occupancy.sun.leaf_area_m2_m2_tile = 0.01;
            occupancy.shade.leaf_area_m2_m2_tile = 0.01;
            occupancy.stem_area_m2_m2_tile = 0.01;
            occupancy.lai = 0.02;
            occupancy.sai = 0.01;
            let mut case_coordinates = coordinates;
            case_coordinates[0] = 293.15;
            case_coordinates[1] = 293.15;
            case_coordinates[2] = 293.15;
            case_coordinates[6] = 293.15;
            case_coordinates[7] = 293.15;
            case_coordinates[8] = 293.15;
            case_coordinates[target_coordinate] = target_temperature;
            let actual_phase = if target_temperature < 273.15 {
                openwepp_vegetation::cold_canopy_m1::M1ExternalPhase::Ice
            } else {
                openwepp_vegetation::cold_canopy_m1::M1ExternalPhase::Liquid
            };
            let q_actual = openwepp_vegetation::cold_canopy_m1::m1_qsat_external(
                target_temperature,
                positive_column.pressure_pa,
                actual_phase,
            )
            .expect("actual external-phase oracle")
            .specific_humidity_kg_kg;
            case_coordinates[13] = if matches!(
                actual_phase,
                openwepp_vegetation::cold_canopy_m1::M1ExternalPhase::Ice
            ) {
                let liquid_q = openwepp_vegetation::cold_canopy_m1::m1_qsat_external(
                    target_temperature,
                    positive_column.pressure_pa,
                    openwepp_vegetation::cold_canopy_m1::M1ExternalPhase::Liquid,
                )
                .expect("liquid midpoint oracle")
                .specific_humidity_kg_kg;
                (q_actual + liquid_q) * 0.5
            } else {
                q_actual * 1.000_001
            };
            assert!(
                case_coordinates[13] > q_actual,
                "target external supersaturation"
            );
            if matches!(
                actual_phase,
                openwepp_vegetation::cold_canopy_m1::M1ExternalPhase::Ice
            ) {
                let liquid_q = openwepp_vegetation::cold_canopy_m1::m1_qsat_external(
                    target_temperature,
                    positive_column.pressure_pa,
                    openwepp_vegetation::cold_canopy_m1::M1ExternalPhase::Liquid,
                )
                .expect("ice ordering oracle")
                .specific_humidity_kg_kg;
                assert!(
                    case_coordinates[13] < liquid_q,
                    "ice-only external supersaturation"
                );
            }
            let warm_q = openwepp_vegetation::cold_canopy_m1::m1_qsat_external(
                293.15,
                positive_column.pressure_pa,
                openwepp_vegetation::cold_canopy_m1::M1ExternalPhase::Liquid,
            )
            .expect("warm component oracle")
            .specific_humidity_kg_kg;
            assert!(
                case_coordinates[13] < warm_q,
                "warm non-target components remain undersaturated"
            );
            let input = M1CoupledColumnInput::from_covered_column(
                positive_column.clone(),
                [source.reservoirs[0].clone(), source.reservoirs[1].clone()],
                source.support.clone(),
                source.topology.clone(),
            );
            match super::m1_core_for_test(
                &input,
                &M1CoupledColumnTrial::from_coordinates(case_coordinates),
            ) {
                Err(error) if error.code() == "VEG-E-142" => {}
                Err(error) => positive_outcomes
                    .push(format!("{component}/{actual_phase:?}: {}", error.code())),
                Ok(()) => positive_outcomes.push(format!("{component}/{actual_phase:?}: accepted")),
            }

            let mut zero_column = positive_column;
            match component {
                "sun" => zero_column.occupancies[0].sun.leaf_area_m2_m2_tile = 0.0,
                "shade" => zero_column.occupancies[0].shade.leaf_area_m2_m2_tile = 0.0,
                "stem" => zero_column.occupancies[0].stem_area_m2_m2_tile = 0.0,
                _ => unreachable!(),
            }
            zero_column.occupancies[0].lai = zero_column.occupancies[0].sun.leaf_area_m2_m2_tile
                + zero_column.occupancies[0].shade.leaf_area_m2_m2_tile;
            zero_column.occupancies[0].sai = zero_column.occupancies[0].stem_area_m2_m2_tile;
            let zero_input = M1CoupledColumnInput::from_covered_column(
                zero_column,
                [source.reservoirs[0].clone(), source.reservoirs[1].clone()],
                source.support.clone(),
                source.topology.clone(),
            );
            if let Err(error) = super::m1_core_for_test(
                &zero_input,
                &M1CoupledColumnTrial::from_coordinates(case_coordinates),
            ) {
                zero_outcomes.push(format!("{component}/{actual_phase:?}: {}", error.code()));
            }
        }
    }
    assert!(
        zero_outcomes.is_empty(),
        "zero-area exemption: {zero_outcomes:?}"
    );
    assert!(
        positive_outcomes.is_empty(),
        "expected-red positive empty surfaces: {positive_outcomes:?}"
    );
}

#[test]
fn m1_structural_drainage_expected_red_current_lu_rejects_affine_zero_direction() {
    // Static iteration-three evidence is an analytic discriminator only. The
    // test invokes no physical solve and asks the test-only seam for no new
    // numerical algorithm: its stored direction is current full-LU behavior.
    let witness: Value = STRUCTURAL_DRAINAGE_TRACE
        .lines()
        .filter_map(|line| line.strip_prefix("m1-linear-witness "))
        .map(|json| serde_json::from_str(json).expect("witness JSON"))
        .find(|value: &Value| value["iteration"] == 3)
        .expect("preserved iteration-three witness");
    let matrix = witness["normalized_jacobian"]
        .as_array()
        .expect("matrix")
        .iter()
        .map(|row| {
            row.as_array()
                .expect("row")
                .iter()
                .map(|value| value.as_f64().expect("number"))
                .collect()
        })
        .collect::<Vec<Vec<f64>>>();
    let rhs = witness["rhs"]
        .as_array()
        .expect("rhs")
        .iter()
        .map(|value| value.as_f64().expect("number"))
        .collect::<Vec<_>>();
    let current = witness["coordinates"]
        .as_array()
        .expect("coordinates")
        .iter()
        .map(|value| value.as_f64().expect("number"))
        .collect::<Vec<_>>();
    let (old_direction, _, _) = super::m1_current_full_lu_ignoring_drainage_metadata_for_test(
        &matrix,
        &rhs,
        super::M1DrainageMetadataTestView {
            rows: [5, 11],
            raw_row_identity: [5, 11],
            drainage_selected: [true, true],
            duration_s: [60.0, 60.0],
            support_s: [60.0, 60.0],
            row_scales: [1e-9, 1e-9],
            current_drainage: [current[5], current[11]],
            raw_drainage_residual: [60.0 * current[5], 60.0 * current[11]],
            raw_drainage_diagonal: [60.0, 60.0],
        },
    )
    .expect("old full LU");
    for index in [5, 11] {
        assert_eq!(
            old_direction[index].to_bits(),
            (-current[index]).to_bits(),
            "expected-red: current full LU must be replaced by source-derived affine drainage direction"
        );
    }
}

#[test]
fn m1_structural_drainage_expected_red_current_candidate_cancels_positive_drainage() {
    // The candidate seam forwards the current x + alpha*d expression. Compare
    // all canonical factors to the independently derived (1-alpha)*D form.
    let witness: Value = serde_json::from_str(STRUCTURAL_DRAINAGE).expect("witness");
    let upper = &witness["drainage_rows"][0];
    let lower = &witness["drainage_rows"][1];
    let mut coordinates = [0.0; 21];
    let mut direction = [0.0; 21];
    coordinates[5] = upper["current"].as_f64().expect("upper current");
    coordinates[11] = lower["current"].as_f64().expect("lower current");
    direction[5] = upper["direction"].as_f64().expect("upper direction");
    direction[11] = lower["direction"].as_f64().expect("lower direction");
    let mut disagreements = Vec::new();
    for exponent in 0..=20 {
        let alpha = 2.0_f64.powi(-exponent);
        let old = super::m1_current_unreduced_candidate_for_test(
            coordinates,
            direction,
            alpha,
            [true, true],
        );
        for index in [5, 11] {
            if old[index].to_bits() != ((1.0 - alpha) * coordinates[index]).to_bits() {
                disagreements.push((index, exponent));
            }
        }
    }
    assert!(
        disagreements.is_empty(),
        "expected-red old candidate differs at {disagreements:?}"
    );
}

#[test]
fn m1_structural_drainage_valid_affine_baseline_reaches_current_lu() {
    let matrix = structural_identity_matrix();
    let rhs = [0.0; 21];
    let metadata = structural_metadata();
    assert_structural_baseline(&matrix, &rhs, &metadata);
    assert!(
        super::m1_current_full_lu_ignoring_drainage_metadata_for_test(&matrix, &rhs, metadata)
            .is_ok()
    );
}

#[test]
fn m1_structural_drainage_expected_red_valid_baseline_single_field_integrity_poison() {
    let matrix = structural_identity_matrix();
    let rhs = [0.0; 21];
    let metadata = structural_metadata();
    assert_structural_baseline(&matrix, &rhs, &metadata);
    let mut poisons = Vec::new();
    for name in [
        "row",
        "identity",
        "duration_positive",
        "support_finite",
        "scale_positive",
        "current_negative",
        "current_nonfinite",
        "raw_diagonal_wrong",
        "raw_diagonal_nonfinite",
        "raw_residual_wrong",
        "raw_residual_nonfinite",
    ] {
        let mut poisoned = structural_metadata();
        match name {
            "row" => poisoned.rows[1] = 21,
            "identity" => poisoned.raw_row_identity[1] = 99,
            "duration_positive" => poisoned.duration_s[0] = 0.0,
            "support_finite" => poisoned.support_s[0] = f64::NAN,
            "scale_positive" => poisoned.row_scales[0] = 0.0,
            "current_negative" => poisoned.current_drainage[0] = -1.0,
            "current_nonfinite" => poisoned.current_drainage[0] = f64::NAN,
            "raw_diagonal_wrong" => poisoned.raw_drainage_diagonal[0] = 61.0,
            "raw_diagonal_nonfinite" => poisoned.raw_drainage_diagonal[0] = f64::NAN,
            "raw_residual_wrong" => poisoned.raw_drainage_residual[0] = 1.0,
            "raw_residual_nonfinite" => poisoned.raw_drainage_residual[0] = f64::NAN,
            _ => unreachable!(),
        }
        match super::m1_current_full_lu_ignoring_drainage_metadata_for_test(&matrix, &rhs, poisoned)
        {
            Ok(_) => poisons.push(format!("{name}: accepted")),
            Err("drainage_elimination_integrity") => {}
            Err(other) => poisons.push(format!("{name}: wrong {other}")),
        }
    }
    assert!(
        poisons.is_empty(),
        "expected-red missing attributable integrity failures: {poisons:?}"
    );
}

#[test]
fn m1_structural_drainage_expected_red_valid_baseline_normalized_row_and_rhs_poison() {
    let matrix = structural_identity_matrix();
    let rhs = [0.0; 21];
    let metadata = structural_metadata();
    assert_structural_baseline(&matrix, &rhs, &metadata);
    let mut poisons = Vec::<String>::new();
    let mut wrong_normalized_row = structural_identity_matrix();
    wrong_normalized_row[5][5] = 2.0;
    match super::m1_current_full_lu_ignoring_drainage_metadata_for_test(
        &wrong_normalized_row,
        &rhs,
        structural_metadata(),
    ) {
        Ok(_) => poisons.push("normalized_row: accepted".to_owned()),
        Err("drainage_elimination_integrity") => {}
        Err(other) => poisons.push(format!("normalized_row: wrong {other}")),
    }
    let mut wrong_rhs = rhs;
    wrong_rhs[11] = 1.0;
    match super::m1_current_full_lu_ignoring_drainage_metadata_for_test(
        &matrix,
        &wrong_rhs,
        structural_metadata(),
    ) {
        Ok(_) => poisons.push("normalized_rhs: accepted".to_owned()),
        Err("drainage_elimination_integrity") => {}
        Err(other) => poisons.push(format!("normalized_rhs: wrong {other}")),
    }
    let mut coupled_selected_row = structural_identity_matrix();
    coupled_selected_row[5][4] = 1.0;
    match super::m1_current_full_lu_ignoring_drainage_metadata_for_test(
        &coupled_selected_row,
        &rhs,
        structural_metadata(),
    ) {
        Ok(_) => poisons.push("selected_row_offdiagonal: accepted".to_owned()),
        Err("drainage_elimination_integrity") => {}
        Err(other) => poisons.push(format!("selected_row_offdiagonal: wrong {other}")),
    }
    assert!(
        poisons.is_empty(),
        "expected-red missing affine row/RHS failures: {poisons:?}"
    );
}

#[test]
fn m1_structural_drainage_expected_red_affine_pivot_is_checked_before_lu() {
    let mut matrix = structural_identity_matrix();
    // The selected row remains source-coherent (dt / scale == 1).  A finite
    // unrelated norm makes its selected affine pivot fail the contract floor.
    matrix[0][0] = 1.0e20;
    let rhs = [0.0; 21];
    let metadata = structural_metadata();
    assert_structural_baseline(&matrix, &rhs, &metadata);
    let accepted =
        super::m1_current_full_lu_ignoring_drainage_metadata_for_test(&matrix, &rhs, metadata);
    assert_eq!(
        accepted.expect_err("expected-red integrity failure"),
        "drainage_elimination_integrity"
    );
}

#[test]
fn m1_structural_drainage_candidate_boundary_vectors_preserve_masked_current_expression() {
    let mut disagreements = Vec::new();
    for (name, drainage) in [
        ("negative_zero", -0.0),
        ("positive_subnormal", f64::from_bits(1)),
        ("max_finite", f64::MAX),
    ] {
        let mut coordinates = [0.0; 21];
        let mut direction = [0.0; 21];
        coordinates[5] = drainage;
        direction[5] = -drainage;
        for exponent in 0..=20 {
            let alpha = 2.0_f64.powi(-exponent);
            let eligible = super::m1_current_unreduced_candidate_for_test(
                coordinates,
                direction,
                alpha,
                [true, false],
            );
            let capacity = super::m1_current_unreduced_candidate_for_test(
                coordinates,
                direction,
                alpha,
                [false, false],
            );
            if eligible[5].to_bits() != ((1.0 - alpha) * drainage).to_bits() {
                disagreements.push((name, exponent));
            }
            assert_eq!(
                capacity[5].to_bits(),
                (coordinates[5] + alpha * direction[5]).to_bits(),
                "capacity-selected drainage keeps ordinary candidate expression",
            );
            assert!(eligible[5].is_finite());
        }
    }
    assert!(
        disagreements.is_empty(),
        "expected-red boundary candidate disagreements: {disagreements:?}"
    );
}

#[test]
fn m1_structural_drainage_current_lu_exposes_coupled_nonzero_columns_for_each_effective_set() {
    let mut matrix = structural_identity_matrix();
    matrix[0][5] = 0.25;
    matrix[1][11] = -0.5;
    let mut rhs = [0.0; 21];
    rhs[5] = -2.0;
    rhs[11] = -3.0;
    let mut metadata = structural_metadata();
    metadata.current_drainage = [2.0, 3.0];
    metadata.raw_drainage_residual = [120.0, 180.0];
    assert_structural_baseline(&matrix, &rhs, &metadata);
    assert_ne!(matrix[0][5], 0.0);
    assert_ne!(matrix[1][11], 0.0);
    for drainage_selected in [[true, false], [false, true], [true, true]] {
        let mut selected = structural_metadata();
        selected.drainage_selected = drainage_selected;
        selected.current_drainage = [2.0, 3.0];
        selected.raw_drainage_residual = [120.0, 180.0];
        let (direction, _, _) =
            super::m1_current_full_lu_ignoring_drainage_metadata_for_test(&matrix, &rhs, selected)
                .expect("current full LU control");
        assert_eq!(direction[0], 0.5);
        assert_eq!(direction[1], -1.5);
        assert_eq!(direction[5], -2.0);
        assert_eq!(direction[11], -3.0);
        for (index, value) in direction.iter().enumerate() {
            if ![0, 1, 5, 11].contains(&index) {
                assert_eq!(*value, 0.0, "closed-form row {index}");
            }
        }
        for row in 0..21 {
            let reconstructed = matrix[row]
                .iter()
                .zip(&direction)
                .map(|(a, b)| a * b)
                .sum::<f64>();
            assert_eq!(reconstructed, rhs[row], "reconstructed full row {row}");
        }
    }
}

#[test]
fn m1_structural_drainage_effective_set_keeps_capacity_rows_in_f_with_distinct_solution() {
    // Closed-form current-LU controls: a selected drainage row is affine while
    // a capacity row remains coupled and non-affine.  Each effective set must
    // therefore have a different retained system/result.
    for (selected_mask, expected_d5, expected_d11) in [
        ([true, false], -2.0, 7.0),
        ([false, true], 4.0, -3.0),
        ([true, true], -2.0, -3.0),
    ] {
        let mut matrix = structural_identity_matrix();
        let mut rhs = [0.0; 21];
        matrix[0][5] = 0.25;
        matrix[1][11] = -0.5;
        if selected_mask[0] {
            rhs[5] = -2.0;
        } else {
            matrix[5][3] = 0.5;
            rhs[5] = 4.0;
        }
        if selected_mask[1] {
            rhs[11] = -3.0;
        } else {
            matrix[11][4] = -0.25;
            rhs[11] = 7.0;
        }
        let mut metadata = structural_metadata();
        metadata.drainage_selected = selected_mask;
        metadata.current_drainage = [2.0, 3.0];
        metadata.raw_drainage_residual = [120.0, 180.0];
        let (direction, _, _) =
            super::m1_current_full_lu_ignoring_drainage_metadata_for_test(&matrix, &rhs, metadata)
                .expect("old full LU effective-set control");
        assert_eq!(direction[5], expected_d5, "E/F upper");
        assert_eq!(direction[11], expected_d11, "E/F lower");
        for row in 0..21 {
            assert_eq!(
                matrix[row]
                    .iter()
                    .zip(&direction)
                    .map(|(a, b)| a * b)
                    .sum::<f64>(),
                rhs[row],
                "E/F reconstructed row {row}"
            );
        }
    }
}

#[test]
fn m1_structural_drainage_noneligible_capacity_branch_retains_solved_nonzero_release() {
    let original: Value = serde_json::from_str(ORIGINAL).expect("original");
    let input = M1CoupledColumnInput::from_covered_column(
        covered_input(&original),
        [
            M1PhaseReservoirInput::new("stratum-z-upper::forest", 0.018, 0.0),
            M1PhaseReservoirInput::new("stratum-a-lower::forest", 0.018, 0.0),
        ],
        conditions(),
        topology_binding(),
    );
    let mut capacity_limited = trial();
    for (coordinate, offset) in [(3, -0.008), (4, 6395.68), (5, 0.005)] {
        capacity_limited = capacity_limited
            .with_coordinate_offset(coordinate, offset)
            .expect("capacity trial");
    }
    let current = super::m1_current_unreduced_linear_system_for_test(&input, &capacity_limited)
        .expect("real capacity controller");
    assert_eq!(current.capacity[0], M1CapacityBranch::Capacity);
    assert_ne!(
        current.direction[5], 0.0,
        "actual coupled LU supplies the release delta"
    );
    let coordinates = super::m1_trial_coordinates_for_test(&capacity_limited);
    let candidate = super::m1_current_unreduced_candidate_for_test(
        coordinates,
        current.direction,
        1.0,
        [false, false],
    );
    assert_ne!(
        candidate[5], coordinates[5],
        "capacity-selected D remains a solved nonzero release coordinate"
    );
}

#[test]
fn m1_structural_drainage_current_phase_controller_retains_real_tie_work_controls() {
    // Regression control: this is the real current controller, not the test
    // adapter. It binds one/two assembly work and keeps both tie selections in
    // scope while the two expected-red tests above remain red.
    let original: Value = serde_json::from_str(ORIGINAL).expect("original");
    let input = M1CoupledColumnInput::from_covered_column(
        covered_input(&original),
        [
            M1PhaseReservoirInput::new("stratum-z-upper::forest", 0.018, 0.0),
            M1PhaseReservoirInput::new("stratum-a-lower::forest", 0.018, 0.0),
        ],
        conditions(),
        topology_binding(),
    );
    let join = M1CoupledColumnTrial::from_coordinates([
        267., 266., 268., 0.018, 0., 0., 269., 267., 270., 0.018, 0., 0., 268., 0.0015, 265.15,
        291.5, 289.8, 293., 293., 293., 270.,
    ]);
    let current = super::m1_current_unreduced_linear_system_for_test(&input, &join)
        .expect("real current controller");
    assert_eq!(current.normalized_jacobian.len(), 21);
    assert_eq!(current.rhs.len(), 21);
    assert_eq!(current.activity.assemblies, 2);
    assert_eq!(current.activity.linear_solves, 2);
    assert_eq!(
        current.capacity,
        [M1CapacityBranch::Drainage, M1CapacityBranch::Drainage]
    );
    assert_eq!(current.drainage_selected, [true, true]);
    let join_coordinates = super::m1_trial_coordinates_for_test(&join);
    for row in [5, 11] {
        assert_eq!(
            current.raw_jacobian[row][row], 60.0,
            "reported selected raw affine diagonal"
        );
        assert_eq!(
            current.raw_residuals[row],
            60.0 * join_coordinates[row],
            "reported selected raw affine residual"
        );
        assert_eq!(
            current.rhs[row],
            -current.raw_residuals[row]
                / (current.raw_jacobian[row][row] / current.normalized_jacobian[row][row])
        );
    }
    // These are report-owned diagnostics from the exact matrix/RHS that the
    // controller supplied to LU.  In particular, RHS is not reconstructed as
    // `-normalized(base)`: inactive anchors have their raw residual RHS.
    assert!(current.direction.iter().all(|value| value.is_finite()));
    let report =
        super::m1_phase_join_controller_iteration(&input, &join).expect("real controller report");
    assert_eq!(report.assembled_system_residual().len(), 21);
    assert_eq!(report.original_system_residual().len(), 21);
    assert!(
        report
            .assembled_system_residual()
            .iter()
            .all(|value| value.is_finite())
    );
    assert!(
        report
            .original_system_residual()
            .iter()
            .all(|value| value.is_finite())
    );
    assert!(
        report
            .source_affine_residuals()
            .iter()
            .all(|value| value.is_some_and(f64::is_finite))
    );
    let assembled = report.assembled_system_residual();
    let original = report.original_system_residual();
    for row in 0..21 {
        let independently_assembled = current.normalized_jacobian[row]
            .iter()
            .zip(current.direction)
            .map(|(coefficient, direction)| coefficient * direction)
            .sum::<f64>()
            - current.rhs[row];
        let independently_original = current.raw_jacobian[row]
            .iter()
            .zip(current.direction)
            .map(|(coefficient, direction)| coefficient * direction)
            .sum::<f64>()
            + current.raw_residuals[row];
        assert_eq!(
            assembled[row].to_bits(),
            independently_assembled.to_bits(),
            "assembled RHS row {row}"
        );
        assert_eq!(
            original[row].to_bits(),
            independently_original.to_bits(),
            "original-system row {row}"
        );
    }
    for (slot, row) in [5, 11].into_iter().enumerate() {
        let independently_affine = current.direction[row] + current.current_drainage[slot];
        assert_eq!(
            report.source_affine_residuals()[slot]
                .expect("selected drainage affine diagnostic")
                .to_bits(),
            independently_affine.to_bits(),
            "source affine row {row}"
        );
    }

    // Keep the wet fraction interior and set positive D from the real
    // capacity identity; this is an exact capacity tie without changing the
    // frozen original60 fixture.  The real controller/report observes both
    // Capacity=>F and final Drainage=>E outcomes.
    let capacity_value = input.column.occupancies[0].liquid_capacity_kg_m2_plant
        * (input.column.occupancies[0].lai + input.column.occupancies[0].sai);
    let mut capacity_tie = join.clone();
    let exact_drainage = (capacity_value - 0.018) / 60.0;
    assert_eq!(60.0 * exact_drainage, capacity_value - 0.018);
    capacity_tie = capacity_tie
        .with_coordinate_offset(5, exact_drainage)
        .expect("interior exact capacity tie");
    let mut capacity_direction = [0.0; 21];
    capacity_direction[4] = 1.0;
    let capacity = super::m1_phase_join_controller_with_forced_predictor_direction(
        &input,
        &capacity_tie,
        capacity_direction,
    )
    .expect("actual capacity-selected controller report");
    assert_eq!(capacity.selected_capacity(0), M1CapacityBranch::Capacity);
    assert_eq!(
        capacity.source_affine_residuals()[0],
        None,
        "capacity tie remains in F"
    );
    assert!(
        !capacity.eliminated_coordinates()[0],
        "capacity tie is retained in F"
    );
    let mut drainage_direction = capacity_direction;
    drainage_direction[5] = -exact_drainage;
    let drainage = super::m1_phase_join_controller_with_forced_predictor_direction(
        &input,
        &capacity_tie,
        drainage_direction,
    )
    .expect("actual final drainage-selected controller report");
    assert_eq!(drainage.selected_capacity(0), M1CapacityBranch::Drainage);
    assert_eq!(
        drainage.direction()[5].to_bits(),
        (-exact_drainage).to_bits()
    );
    assert_eq!(drainage.source_affine_residuals()[0], Some(0.0));
    assert!(
        drainage.eliminated_coordinates()[0],
        "drainage tie is eliminated into E"
    );
    assert!(
        drainage.linear_dimension() < 21,
        "actual drainage solve is reduced"
    );
    assert_eq!(drainage.work_counts().assemblies, 2);
    assert_eq!(drainage.work_counts().linear_solves, 2);
}

#[test]
fn m1_structural_drainage_expected_red_selected_assembly_integrity_refuses_before_old_lu() {
    let original: Value = serde_json::from_str(ORIGINAL).expect("original input");
    let input = M1CoupledColumnInput::from_covered_column(
        covered_input(&original),
        [
            M1PhaseReservoirInput::new("stratum-z-upper::forest", 0.018, 0.0),
            M1PhaseReservoirInput::new("stratum-a-lower::forest", 0.018, 0.0),
        ],
        conditions(),
        topology_binding(),
    );
    let trial = M1CoupledColumnTrial::from_coordinates([
        267., 266., 268., 0.018, 0., 0., 269., 267., 270., 0.018, 0., 0., 268., 0.0015, 265.15,
        291.5, 289.8, 293., 293., 293., 270.,
    ]);
    let input_before = input.clone();
    let trial_before = trial.clone();
    let mut drainage_direction = [0.0; 21];
    drainage_direction[4] = -1.0;
    // This only poisons the actual selected raw assembly immediately
    // before its existing old LU.  The expected guard is deliberately absent,
    // so this test remains red until the released controller rejects it.
    let failure = super::m1_phase_join_controller_with_selected_assembly_poison(
        &input,
        &trial,
        drainage_direction,
        super::M1SelectedAffineAssemblyPoison {
            row: 5,
            column: 0,
            value: 1.0,
        },
    )
    .expect_err("LSEB-E-034 selected-affine integrity before LU");
    assert_eq!(failure.code(), "LSEB-E-034");
    assert!(
        matches!(failure, super::M1PhaseJoinFailure::Source { ref error, .. }
        if matches!(**error, M1CoupledError::Numerical(ref numerical)
            if format!("{:?}", numerical.kind) == "DrainageEliminationIntegrity"))
    );
    assert_eq!(failure.controller_activity().assemblies, 2);
    assert_eq!(failure.controller_activity().linear_solves, 1);
    assert_eq!(failure.controller_activity().candidate_attempts, 0);
    assert_eq!(failure.controller_activity().backtracking_attempts, 0);
    assert_eq!(failure.controller_activity().materialization_attempts, 0);
    assert_eq!(failure.controller_activity().receipt_attempts, 0);
    assert_eq!(failure.controller_activity().state_install_attempts, 0);
    assert_eq!(input, input_before);
    assert_eq!(trial, trial_before);
}

#[test]
fn m1_structural_drainage_reduced_singular_is_numerical_not_integrity() {
    // A valid selected E row is diagonal-only.  Therefore a singular F block
    // also makes the old full system singular; that remains a numerical pivot
    // refusal, never an affine-integrity failure.
    let mut singular = structural_identity_matrix();
    singular[0][0] = 0.0;
    let rhs = [0.0; 21];
    let metadata = structural_metadata();
    let singular =
        super::m1_current_full_lu_ignoring_drainage_metadata_for_test(&singular, &rhs, metadata);
    assert_eq!(
        singular.expect_err("reduced singular/pivot numerical refusal"),
        "singular_pivot"
    );
}

#[test]
fn m1_expected_red_controller_error_retains_actual_activity_through_source_conversion() {
    let original: Value = serde_json::from_str(ORIGINAL).expect("original input");
    let input = M1CoupledColumnInput::from_covered_column(
        covered_input(&original),
        [
            M1PhaseReservoirInput::new("stratum-z-upper::forest", 0.018, 0.0),
            M1PhaseReservoirInput::new("stratum-a-lower::forest", 0.018, 0.0),
        ],
        conditions(),
        topology_binding(),
    );
    let trial = M1CoupledColumnTrial::from_coordinates([
        267., 266., 268., 0.018, 0., 0., 269., 267., 270., 0.018, 0., 0., 268., 0.0015, 265.15,
        291.5, 289.8, 293., 293., 293., 270.,
    ]);
    let mut predictor = [0.0; 21];
    predictor[4] = -1.0;
    super::m1_reset_loop_activity_observation_for_test();
    let error = super::m1_controller_failure_activity_for_test(
        &input,
        &trial,
        predictor,
        super::M1SelectedAffineAssemblyPoison {
            row: 5,
            column: 0,
            value: 1.0,
        },
    );
    assert_eq!(error.code(), "LSEB-E-034");
    let activity = super::m1_last_controller_failure_activity_for_test()
        .expect("controller activity survives source conversion");
    assert_eq!(activity.assemblies, 2);
    assert_eq!(activity.linear_solves, 1);
    assert_eq!(activity.candidate_attempts, 0);
    assert_eq!(activity.backtracking_attempts, 0);
    let runtime = super::m1_loop_activity_observation_for_test();
    assert_eq!(runtime.predictor_assemblies, 1);
    assert_eq!(runtime.final_assemblies, 1);
    assert_eq!(runtime.controller_failure_activity, Some(activity));
    assert!(
        runtime
            .reduced_linear_dimension_counts
            .iter()
            .any(|count| *count > 0)
    );
}

#[test]
fn m1_structural_drainage_expected_red_reduced_norm_uses_f_not_full_a() {
    let mut norm_control = structural_identity_matrix();
    norm_control[0][0] = 1.0e-3;
    norm_control[5][5] = 1.0e12;
    norm_control[11][11] = 1.0e12;
    norm_control[0][5] = 1.0e12;
    let mut metadata = structural_metadata();
    metadata.row_scales = [60.0 / 1.0e12; 2];
    let reduced_floor = 64.0 * f64::EPSILON;
    assert!(1.0e-3 > reduced_floor);
    let (direction, _, norm) = super::m1_current_full_lu_ignoring_drainage_metadata_for_test(
        &norm_control,
        &[0.0; 21],
        metadata,
    )
    .expect("finite reduced-norm solve must not use inflated full-A norm");
    assert_eq!(norm, 1.0, "actual reduced F norm");
    for row in 0..21 {
        assert_eq!(
            norm_control[row]
                .iter()
                .zip(&direction)
                .map(|(coefficient, value)| coefficient * value)
                .sum::<f64>(),
            0.0,
            "reconstructed reduced row {row}"
        );
    }
    assert_eq!(direction[5], 0.0, "upper affine dD=-D");
    assert_eq!(direction[11], 0.0, "lower affine dD=-D");
}

#[test]
fn m1_structural_drainage_reduced_overflow_is_typed_not_integrity() {
    let scale = 60.0 / 1.0e295;
    let raw_residual = 60.0 * 2.0;
    let mut matrix = vec![vec![0.0; 21]; 21];
    for index in 0..21 {
        matrix[index][index] = 1.0e295;
    }
    matrix[0][5] = 1.0e308;
    let mut rhs = vec![0.0; 21];
    rhs[5] = -(raw_residual / scale);
    let mut metadata = structural_metadata();
    metadata.row_scales = [scale, scale];
    metadata.current_drainage = [2.0, 0.0];
    metadata.raw_drainage_residual = [raw_residual, 0.0];
    assert_eq!(
        matrix[5][5],
        metadata.duration_s[0] / metadata.row_scales[0]
    );
    assert_eq!(
        rhs[5],
        -(metadata.raw_drainage_residual[0] / metadata.row_scales[0])
    );
    assert_eq!(
        super::m1_current_full_lu_ignoring_drainage_metadata_for_test(&matrix, &rhs, metadata)
            .expect_err("typed reduced finite-result refusal"),
        "nonfinite_reduced_result"
    );
}

#[test]
fn m1_structural_drainage_empty_effective_set_nan_pivot_keeps_generic_singular_failure() {
    let mut matrix = structural_identity_matrix();
    matrix[0][0] = f64::NAN;
    let mut metadata = structural_metadata();
    metadata.drainage_selected = [false, false];
    let (result, attempts) =
        super::m1_current_full_lu_activity_for_test(&matrix, &[0.0; 21], metadata);
    assert_eq!(attempts, 1, "empty effective set reaches generic LU once");
    assert_eq!(
        result.expect_err("generic full LU must classify its NaN pivot"),
        "singular_pivot"
    );
}

#[test]
fn m1_structural_drainage_empty_effective_set_nan_rhs_keeps_nonfinite_result() {
    let mut rhs = [0.0; 21];
    rhs[0] = f64::NAN;
    let mut metadata = structural_metadata();
    metadata.drainage_selected = [false, false];
    let (result, attempts) =
        super::m1_current_full_lu_activity_for_test(&structural_identity_matrix(), &rhs, metadata);
    assert_eq!(
        attempts, 1,
        "empty effective set attempts generic LU before result check"
    );
    assert_eq!(
        result.expect_err("generic full LU result must retain nonfinite provenance"),
        "nonfinite_reduced_result"
    );
}

#[test]
fn m1_structural_drainage_selected_nan_coupling_or_retained_rhs_is_nonfinite_result() {
    let mut coupling = structural_identity_matrix();
    coupling[0][5] = f64::NAN;
    let mut metadata = structural_metadata();
    metadata.current_drainage = [2.0, 0.0];
    metadata.raw_drainage_residual = [120.0, 0.0];
    let mut affine_rhs = [0.0; 21];
    affine_rhs[5] = -2.0;
    let (result, attempts) =
        super::m1_current_full_lu_activity_for_test(&coupling, &affine_rhs, metadata);
    assert_eq!(attempts, 0, "nonempty NaN coupling rejects before LU");
    assert_eq!(
        result.expect_err("selected A_FE must make reduced RHS nonfinite"),
        "nonfinite_reduced_result"
    );
    let mut infinity_coupling = structural_identity_matrix();
    infinity_coupling[0][5] = f64::INFINITY;
    let mut metadata = structural_metadata();
    metadata.current_drainage = [2.0, 0.0];
    metadata.raw_drainage_residual = [120.0, 0.0];
    assert_eq!(
        super::m1_current_full_lu_ignoring_drainage_metadata_for_test(
            &infinity_coupling,
            &affine_rhs,
            metadata
        )
        .expect_err("infinite selected A_FE must make reduced RHS nonfinite"),
        "nonfinite_reduced_result"
    );
    let mut rhs = [0.0; 21];
    rhs[0] = f64::NAN;
    rhs[5] = -2.0;
    let mut metadata = structural_metadata();
    metadata.current_drainage = [2.0, 0.0];
    metadata.raw_drainage_residual = [120.0, 0.0];
    assert_eq!(
        super::m1_current_full_lu_ignoring_drainage_metadata_for_test(
            &structural_identity_matrix(),
            &rhs,
            metadata
        )
        .expect_err("retained RHS must retain nonfinite provenance"),
        "nonfinite_reduced_result"
    );
}

#[test]
fn m1_structural_drainage_selected_finite_full_norm_overflow_is_nonfinite_result() {
    let mut matrix = structural_identity_matrix();
    matrix[0][0] = f64::MAX;
    matrix[0][1] = f64::MAX;
    let (result, attempts) =
        super::m1_current_full_lu_activity_for_test(&matrix, &[0.0; 21], structural_metadata());
    assert_eq!(attempts, 0, "nonempty full-norm overflow rejects before LU");
    assert_eq!(
        result
            .expect_err("finite full norm sum overflow must not become an integrity pivot failure"),
        "nonfinite_reduced_result"
    );
}

#[test]
fn m1_structural_drainage_selected_infinite_normalized_diagonal_is_integrity_before_lu() {
    let mut matrix = structural_identity_matrix();
    let mut metadata = structural_metadata();
    metadata.row_scales[0] = f64::MIN_POSITIVE;
    matrix[5][5] = metadata.duration_s[0] / metadata.row_scales[0];
    assert!(matrix[5][5].is_infinite());
    let (result, attempts) =
        super::m1_current_full_lu_activity_for_test(&matrix, &[0.0; 21], metadata);
    assert_eq!(
        attempts, 0,
        "selected normalized infinity rejects before LU"
    );
    assert_eq!(
        result.expect_err("selected normalized diagonal must be finite"),
        "drainage_elimination_integrity"
    );
}

#[test]
fn m1_phase_join_expected_red_selects_directional_phase_and_capacity_controls() {
    // These controls deliberately use phase primitives and arithmetic, rather
    // than the Jacobian being specified. The API is absent until the reviewed
    // directional-active-set implementation is supplied.
    let selected = super::select_m1_phase_join_active_set(
        M1PhaseReservoir::from_mass_enthalpy(0.018, 0.0).expect("liquid join"),
        0.297,
        0.018 / 60.0,
        60.0,
        -0.000_276_901_700_229_317_9,
        -783.510_921_253_580_5,
        0.0,
    )
    .expect("finite cooling predictor");
    assert_eq!(selected.phase(), M1Phase::Mixed);
    assert_eq!(selected.capacity_branch(), M1CapacityBranch::Drainage);
    assert!(selected.requires_corrected_solve());

    let warming = super::select_m1_phase_join_active_set(
        M1PhaseReservoir::from_mass_enthalpy(0.018, 0.0).expect("liquid join"),
        0.297,
        0.0,
        60.0,
        0.0,
        1.0,
        0.0,
    )
    .expect("finite warming predictor");
    assert_eq!(warming.phase(), M1Phase::Liquid);
    assert!(!warming.requires_corrected_solve());

    assert_eq!(-333_700.0 / 8192.0 + 333_700.0 * (1.0 / 8192.0), 0.0);
    assert_eq!(1.0 / 8192.0 + (-333_700.0 / 8192.0) / 333_700.0, 0.0);
    let lower = -333_700.0 * 0.018;
    // The lower join is H=-Lf*M, not H=0.  Both probes have nonzero dM;
    // selection follows g=dH+Lf*dM and g=0 keeps the mixed tangent.
    for (d_m, d_h, phase) in [
        (1.0e-4, -333_700.0e-4 + 1.0, M1Phase::Mixed),
        (1.0e-4, -333_700.0e-4 - 1.0, M1Phase::Ice),
        (1.0 / 8192.0, -333_700.0 / 8192.0, M1Phase::Mixed),
    ] {
        let selected = super::select_m1_phase_join_active_set(
            M1PhaseReservoir::from_mass_enthalpy(0.018, lower).expect("lower join"),
            0.297,
            0.0,
            60.0,
            d_m,
            d_h,
            0.0,
        )
        .expect("finite lower-join predictor");
        assert_eq!(selected.phase(), phase);
    }

    let capacity_tie = super::select_m1_phase_join_active_set(
        M1PhaseReservoir::from_mass_enthalpy(0.018, 0.0).expect("capacity join"),
        0.018,
        0.0,
        60.0,
        0.0,
        -1.0,
        0.0,
    )
    .expect("cooling capacity tie");
    assert_eq!(capacity_tie.phase(), M1Phase::Mixed);
    assert_eq!(capacity_tie.capacity_branch(), M1CapacityBranch::Drainage);

    let empty_tie = super::select_m1_phase_join_active_set(
        M1PhaseReservoir::empty(),
        0.297,
        0.297 / 60.0,
        60.0,
        1.0,
        1.0,
        0.0,
    )
    .expect("empty capacity tie");
    assert_eq!(empty_tie.phase(), M1Phase::Empty);
    assert_eq!(empty_tie.capacity_branch(), M1CapacityBranch::Capacity);

    // The cap normal is dt*dD+dml.  Cover both signs and the exact-zero tie
    // in a nonempty liquid state; H signed zero is still the liquid join.
    for (d_drainage, branch) in [
        (-1.0, M1CapacityBranch::Drainage),
        (0.0, M1CapacityBranch::Capacity),
        (1.0, M1CapacityBranch::Capacity),
    ] {
        let cap = super::select_m1_phase_join_active_set(
            M1PhaseReservoir::from_mass_enthalpy(0.018, -0.0).expect("signed-zero liquid"),
            0.018,
            0.0,
            60.0,
            0.0,
            1.0,
            d_drainage,
        )
        .expect("finite capacity normal");
        assert_eq!(cap.phase(), M1Phase::Liquid);
        assert_eq!(cap.capacity_branch(), branch);
    }

    // At the lower join g=0 retains mixed, so dml=dM+dH/Lf=0 and its exact
    // capacity normal keeps Capacity too.
    let lower_cap = super::select_m1_phase_join_active_set(
        M1PhaseReservoir::from_mass_enthalpy(0.018, lower).expect("lower join"),
        0.018,
        0.018 / 60.0,
        60.0,
        1.0 / 8192.0,
        -333_700.0 / 8192.0,
        0.0,
    )
    .expect("lower g-zero");
    assert_eq!(lower_cap.phase(), M1Phase::Mixed);
    assert_eq!(lower_cap.capacity_branch(), M1CapacityBranch::Capacity);
}

#[test]
fn m1_phase_join_expected_red_rejects_nonfinite_and_final_set_mismatch_before_backtracking() {
    let join = M1PhaseReservoir::from_mass_enthalpy(0.018, 0.0).expect("liquid join");
    assert!(
        super::select_m1_phase_join_active_set(join, 0.297, 0.0, 60.0, 0.0, f64::NAN, 0.0).is_err()
    );
    let selected = super::select_m1_phase_join_active_set(join, 0.297, 0.0, 60.0, 0.0, -1.0, 0.0)
        .expect("cooling set");
    assert!(!super::m1_phase_join_set_matches_direction(
        &selected, 0.0, 1.0, 0.0
    ));

    // A real binary64 displacement at a valid large-M lower join can round
    // back to the base.  It is LSEB-E-034/Unrepresentable, never a relabel.
    let large_m: f64 = 1.0e14;
    let large_h: f64 = -333_700.0 * large_m;
    assert_eq!((large_m + 1.0e-3).to_bits(), large_m.to_bits());
    assert_eq!((large_h - 333.701).to_bits(), large_h.to_bits());
    let rounded = super::select_m1_phase_join_active_set(
        M1PhaseReservoir::from_mass_enthalpy(large_m, large_h).expect("valid large-M lower join"),
        large_m * 2.0,
        0.0,
        60.0,
        1.0e-3,
        -333.701,
        0.0,
    )
    .expect_err("selected-side probe rounds to the lower-join base");
    assert_eq!(rounded.code(), "LSEB-E-034");
    assert!(matches!(
        rounded,
        super::M1PhaseJoinFailure::UnrepresentableProbe { .. }
    ));
}

#[test]
fn m1_phase_join_expected_red_binds_all_primal_rows_and_bounded_work() {
    // This is a whole-direction primal secant, not an assertion derived from
    // the implemented Jacobian.  The input holds both reservoirs at H=0 and
    // retains the frozen cold-column forcing used by the original-60 case.
    let original: Value = serde_json::from_str(ORIGINAL).expect("original input");
    let input = M1CoupledColumnInput::from_covered_column(
        covered_input(&original),
        [
            M1PhaseReservoirInput::new("stratum-z-upper::forest", 0.018, 0.0),
            M1PhaseReservoirInput::new("stratum-a-lower::forest", 0.018, 0.0),
        ],
        conditions(),
        topology_binding(),
    );
    let join = M1CoupledColumnTrial::from_coordinates([
        267., 266., 268., 0.018, 0., 0., 269., 267., 270., 0.018, 0., 0., 268., 0.0015, 265.15,
        291.5, 289.8, 293., 293., 293., 270.,
    ]);
    let direction = [
        0.,
        0.,
        0.,
        -0.000_276_901_700_229_317_9,
        -783.510_921_253_580_5,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
    ];
    let base = evaluate_m1_coupled_column(&input, &join).expect("base primal evaluator");
    // 2^-12 keeps this same-side finite-difference well above cancellation
    // (including dM) while retaining the frozen cold-domain bounds.  It is
    // chosen before observing a matrix and is not the evaluator's FD step.
    let epsilon = 1.0 / 4096.0;
    let mut stepped = join.clone();
    for (coordinate, delta) in direction.iter().enumerate() {
        stepped = stepped
            .with_coordinate_offset(coordinate, epsilon * delta)
            .expect("finite same-side independent probe");
    }
    let probe = evaluate_m1_coupled_column(&input, &stepped).expect("mixed-side primal probe");
    // This test-only primal surface is the unchanged `core`/`m1_tolerances`
    // calculation before the phase-join controller sees a matrix.  It is kept
    // separate from the selected-matrix report so a controller normalizer bug
    // cannot make both sides agree.
    let base_normalizers = super::m1_phase_join_primal_normalizers(&input, &join)
        .expect("base canonical primal normalizers");
    let probe_normalizers = super::m1_phase_join_primal_normalizers(&input, &stepped)
        .expect("probe canonical primal normalizers");
    assert_eq!((base_normalizers.len(), probe_normalizers.len()), (21, 21));
    let controller = super::m1_phase_join_controller_iteration(&input, &join)
        .expect("actual phase-join controller iteration");
    let selected_raw_jacobian_times_direction = controller.selected_raw_jacobian_times(&direction);
    let selected_jacobian_times_direction =
        controller.selected_normalized_jacobian_times(&direction);
    assert_eq!(selected_raw_jacobian_times_direction.len(), 21);
    assert_eq!(selected_jacobian_times_direction.len(), 21);
    for row in 0..21 {
        let raw_secant = (probe.residuals()[row] - base.residuals()[row]) / epsilon;
        let normalized_secant = (probe.residuals()[row] / probe_normalizers[row]
            - base.residuals()[row] / base_normalizers[row])
            / epsilon;
        assert!(
            raw_secant.is_finite() && normalized_secant.is_finite(),
            "row {row}"
        );
        assert!(
            (selected_raw_jacobian_times_direction[row] - raw_secant).abs()
                <= 2.0e-3 * raw_secant.abs().max(1.0),
            "selected final raw matrix row {row}"
        );
        assert!(
            (selected_jacobian_times_direction[row] - normalized_secant).abs()
                <= 2.0e-3 * normalized_secant.abs().max(1.0),
            "selected final matrix row {row}: reciprocal-LW, dry, reservoir, and shared identities bind"
        );
    }
    assert_eq!(controller.work_counts().assemblies, 2);
    assert_eq!(controller.work_counts().linear_solves, 2);
    assert_eq!(controller.work_counts().backtracking_attempts, 0);

    // An exact H=0 join whose forced predictor warms retains the current liquid
    // set, so the real controller must reuse its one assembly/solve.
    let mut warm_predictor = [0.0; 21];
    warm_predictor[4] = 1.0;
    let ordinary = super::m1_phase_join_controller_with_forced_predictor_direction(
        &input,
        &join,
        warm_predictor,
    )
    .expect("exact-join identical-set controller iteration");
    assert_eq!(
        (
            ordinary.work_counts().assemblies,
            ordinary.work_counts().linear_solves
        ),
        (1, 1)
    );

    // Bind the lower join to the selected final matrix as well: nonzero dM
    // and g=dH+Lf*dM<0 select ice, and every row sees that selected J*d.
    let lower_join = join
        .with_coordinate_offset(10, -333_700.0 * 0.018)
        .expect("lower-join trial");
    let mut lower_direction = [0.0; 21];
    lower_direction[9] = 1.0e-4;
    lower_direction[10] = -333_700.0e-4 - 1.0;
    let lower_base = evaluate_m1_coupled_column(&input, &lower_join).expect("lower base");
    let mut lower_probe = lower_join.clone();
    for (coordinate, delta) in lower_direction.iter().enumerate() {
        lower_probe = lower_probe
            .with_coordinate_offset(coordinate, epsilon * delta)
            .expect("lower same-side probe");
    }
    let lower_primal = evaluate_m1_coupled_column(&input, &lower_probe).expect("lower probe");
    let lower_base_normalizers = super::m1_phase_join_primal_normalizers(&input, &lower_join)
        .expect("lower base canonical primal normalizers");
    let lower_probe_normalizers = super::m1_phase_join_primal_normalizers(&input, &lower_probe)
        .expect("lower probe canonical primal normalizers");
    let lower_controller = super::m1_phase_join_controller_with_forced_predictor_direction(
        &input,
        &lower_join,
        lower_direction,
    )
    .expect("lower final controller matrix");
    assert_eq!(lower_controller.selected_phase(1), M1Phase::Ice);
    let lower_selected_raw = lower_controller.selected_raw_jacobian_times(&lower_direction);
    let lower_selected_normalized =
        lower_controller.selected_normalized_jacobian_times(&lower_direction);
    assert_eq!(lower_selected_raw.len(), 21);
    assert_eq!(lower_selected_normalized.len(), 21);
    for row in 0..21 {
        let lower_secant = (lower_primal.residuals()[row] - lower_base.residuals()[row]) / epsilon;
        let lower_normalized_secant = (lower_primal.residuals()[row]
            / lower_probe_normalizers[row]
            - lower_base.residuals()[row] / lower_base_normalizers[row])
            / epsilon;
        assert!(
            (lower_selected_raw[row] - lower_secant).abs() <= 2.0e-3 * lower_secant.abs().max(1.0),
            "lower selected raw matrix row {row}"
        );
        assert!(
            (lower_selected_normalized[row] - lower_normalized_secant).abs()
                <= 2.0e-3 * lower_normalized_secant.abs().max(1.0),
            "lower selected normalized matrix row {row}"
        );
    }

    let nonjoin = super::m1_phase_join_controller_iteration(&input, &trial())
        .expect("ordinary nonjoin controller iteration");
    assert_eq!(
        (
            nonjoin.work_counts().assemblies,
            nonjoin.work_counts().linear_solves
        ),
        (1, 1)
    );
    // A non-tied capacity-limited primal branch must also reuse its sole solve.
    let mut capacity_limited = trial();
    for (coordinate, offset) in [(3, -0.008), (4, 6395.68), (5, 0.005)] {
        capacity_limited = capacity_limited
            .with_coordinate_offset(coordinate, offset)
            .expect("capacity-limited trial");
    }
    let capacity_control = super::m1_phase_join_controller_iteration(&input, &capacity_limited)
        .expect("away-from-tie capacity-limited controller");
    assert_eq!(
        (
            capacity_control.work_counts().assemblies,
            capacity_control.work_counts().linear_solves
        ),
        (1, 1)
    );
}

#[test]
fn m1_phase_join_expected_red_fails_closed_without_mutation_or_backtracking() {
    let original: Value = serde_json::from_str(ORIGINAL).expect("original input");
    let input = M1CoupledColumnInput::from_covered_column(
        covered_input(&original),
        [
            M1PhaseReservoirInput::new("stratum-z-upper::forest", 0.018, 0.0),
            M1PhaseReservoirInput::new("stratum-a-lower::forest", 0.018, 0.0),
        ],
        conditions(),
        topology_binding(),
    );
    let mut cooling_predictor = [0.0; 21];
    cooling_predictor[4] = -1.0;
    let mut warming_final = [0.0; 21];
    warming_final[4] = 1.0;
    let joined = M1CoupledColumnTrial::from_coordinates([
        267., 266., 268., 0.018, 0., 0., 269., 267., 270., 0.018, 0., 0., 268., 0.0015, 265.15,
        291.5, 289.8, 293., 293., 293., 270.,
    ]);
    let inconsistent = super::m1_phase_join_controller_with_forced_directions(
        &input,
        &joined,
        cooling_predictor,
        warming_final,
    )
    .expect_err("joined cooling predictor and warming final direction disagree");
    assert_eq!(inconsistent.code(), "LSEB-E-034");
    assert!(matches!(
        inconsistent,
        super::M1PhaseJoinFailure::InconsistentFinalSet { .. }
    ));
    let inconsistent_counts = inconsistent.controller_activity();
    assert_eq!(inconsistent_counts.assemblies, 2);
    assert_eq!(inconsistent_counts.linear_solves, 2);
    assert_eq!(inconsistent_counts.candidate_attempts, 0);
    assert_eq!(inconsistent_counts.backtracking_attempts, 0);
    assert_eq!(inconsistent_counts.materialization_attempts, 0);
    assert_eq!(inconsistent_counts.receipt_attempts, 0);
    assert_eq!(inconsistent_counts.state_install_attempts, 0);

    let unrepresentable = super::m1_phase_join_controller_unrepresentable_probe(&input)
        .expect_err("unrepresentable selected-side probe");
    assert_eq!(unrepresentable.code(), "LSEB-E-034");
    assert!(matches!(
        unrepresentable,
        super::M1PhaseJoinFailure::UnrepresentableProbe { .. }
    ));
    let unrepresentable_counts = unrepresentable.controller_activity();
    // The unrepresentable helper uses a changed frozen set: predictor
    // assembly/solve then corrected assembly, whose selected-side probe fails
    // before a corrected solve.  No third assembly is permitted.
    assert_eq!(unrepresentable_counts.assemblies, 2);
    assert_eq!(unrepresentable_counts.linear_solves, 1);
    assert_eq!(unrepresentable_counts.candidate_attempts, 0);
    assert_eq!(unrepresentable_counts.backtracking_attempts, 0);
    assert_eq!(unrepresentable_counts.materialization_attempts, 0);
    assert_eq!(unrepresentable_counts.receipt_attempts, 0);
    assert_eq!(unrepresentable_counts.state_install_attempts, 0);
}
fn assert_gas_reference(actual: f64, expected: &Value, gas: &Value, quantity: &str) {
    let expected = decimal(expected);
    let tolerance = &gas["comparisons"][quantity];
    assert!(actual.is_finite());
    assert!(
        (actual - expected).abs()
            <= decimal(&tolerance["absolute"])
                .max(decimal(&tolerance["relative"]) * expected.abs())
    );
}

fn vulnerability(psi_mm: f64, p50_mm: f64, exponent: f64) -> f64 {
    (-std::f64::consts::LN_2 * (psi_mm / p50_mm).powf(exponent)).exp()
}

fn independently_reconstruct_hydraulics(
    actual: &M1CoupledEvaluation,
    row: &Value,
    original: &Value,
    gas: &Value,
) {
    let occupancy = actual
        .occupancy(row["occupancy_id"].as_str().expect("occupancy"))
        .expect("occupancy");
    let source = covered_input(original)
        .occupancies
        .into_iter()
        .find(|item| item.occupancy_id == row["occupancy_id"].as_str().expect("occupancy"))
        .expect("original occupancy coefficients");
    let p = &occupancy.hydraulics.potentials_mm;
    let sun_flux = occupancy.sun.vapor_kg_m2_tile_s;
    let shade_flux = occupancy.shade.vapor_kg_m2_tile_s;
    let ksun = source.k1_sun_max_s1
        * source.sun.leaf_area_m2_m2_tile
        * vulnerability(p[2], source.p50_xylem_mm, source.vulnerability_exponent);
    let kshade = source.k1_shade_max_s1
        * source.shade.leaf_area_m2_m2_tile
        * vulnerability(p[2], source.p50_xylem_mm, source.vulnerability_exponent);
    let qsun = ksun * (p[2] - p[0]);
    let qshade = kshade * (p[2] - p[1]);
    assert_gas_reference(
        qsun,
        &Value::String(format!("{sun_flux}")),
        gas,
        "water_flux_kg_m2_tile_s",
    );
    assert_gas_reference(
        qshade,
        &Value::String(format!("{shade_flux}")),
        gas,
        "water_flux_kg_m2_tile_s",
    );
    let stem_k = source.k2_max / source.height_m
        * vulnerability(p[3], source.p50_xylem_mm, source.vulnerability_exponent)
        * source.sai;
    let qstem = stem_k * (p[3] - p[2] - 1000.0 * source.height_m);
    assert_gas_reference(qstem, &row["stem_flux"], gas, "water_flux_kg_m2_tile_s");
    let mut qroot_sum = 0.0;
    for (layer, expected) in source
        .root_layers
        .iter()
        .filter(|layer| layer.accessible && !layer.frozen && layer.root_fraction > 0.0)
        .zip(row["root_fluxes"].as_array().expect("root flows"))
    {
        let kr = source.k3_max_m_s / layer.z3_m
            * vulnerability(
                layer.soil_potential_mm,
                source.p50_root_mm,
                source.vulnerability_exponent,
            );
        let ks = layer.ksoil_m2_s / layer.dxroot_m;
        let coefficient = kr * ks / (kr + ks)
            * (source.lai + source.sai)
            * layer.root_fraction
            * source.root_to_leaf_area;
        let qroot = coefficient * (layer.soil_potential_mm - p[3] + layer.gravity_head_mm);
        qroot_sum += qroot;
        assert_gas_reference(qroot, expected, gas, "water_flux_kg_m2_tile_s");
    }
    let continuity = [
        qsun - sun_flux,
        qshade - shade_flux,
        qsun + qshade - qstem,
        qstem - qroot_sum,
    ];
    let expected = row["continuity_residuals"]
        .as_array()
        .expect("four frozen continuity residuals");
    assert_eq!(continuity.len(), expected.len());
    for (actual, expected) in continuity.iter().zip(expected) {
        assert_gas_reference(*actual, expected, gas, "continuity_residual_kg_m2_tile_s");
    }
}

// Fieldwise test-side source operands: production evaluation takes this and a
// trial vector, not an oracle JSON object or an expected residual callback.
fn covered_input(original: &Value) -> CoveredColumnInputs {
    serde_json::from_value(original["inputs"].clone()).expect("original CoveredColumnInputs")
}
fn conditions() -> M1SupportConditions {
    M1SupportConditions {
        model_definition_id: "OPENWEPP_C3_WOODY_COLD_M1_V1".to_owned(),
        occupancy_conditions: vec![
            M1OccupancySupportCondition {
                occupancy_id: "stratum-z-upper::forest".to_owned(),
                liquid_conducting: true,
                full_supply: true,
            },
            M1OccupancySupportCondition {
                occupancy_id: "stratum-a-lower::forest".to_owned(),
                liquid_conducting: true,
                full_supply: true,
            },
        ],
    }
}

fn topology_binding() -> M1ColumnTopologyBinding {
    M1ColumnTopologyBinding {
        occupancy_ids: [
            "stratum-z-upper::forest".to_owned(),
            "stratum-a-lower::forest".to_owned(),
        ],
        soil_layer_ids: [
            "thermal-1".to_owned(),
            "thermal-2".to_owned(),
            "soil-1".to_owned(),
            "soil-2".to_owned(),
            "soil-dry".to_owned(),
            "soil-frozen".to_owned(),
        ],
    }
}

fn raw_column_input(
    column: CoveredColumnInputs,
    support: M1SupportConditions,
) -> M1CoupledColumnInput {
    M1CoupledColumnInput::from_covered_column(
        column,
        [
            M1PhaseReservoirInput::new("stratum-z-upper::forest", 0.018, 0.0),
            M1PhaseReservoirInput::new("stratum-a-lower::forest", 0.018, 0.0),
        ],
        support,
        topology_binding(),
    )
}

fn assert_m1_diagnostic_envelope(error: M1CoupledError) {
    assert_eq!(error.code(), "LSEB-E-030");
    assert_eq!(
        error,
        M1CoupledError::LandSurfaceEnergy(LandSurfaceEnergyError::UnsupportedDomain(
            "m1_diagnostic_envelope"
        ))
    );
}

fn native_exchange_presence_poison(column: &CoveredColumnInputs) -> NativeSnowExchangeV1 {
    let optical = column
        .stage3_optical
        .as_ref()
        .expect("actual optical receipt");
    let mut exchange = NativeSnowExchangeV1 {
        policy_id: 14,
        ofe_id: optical.ofe_id.clone(),
        tile_id: optical.tile_id.clone(),
        start_ns: 0,
        end_ns: 60_000_000_000,
        snow_temperature_k: 265.15,
        snow_saturation_specific_humidity: 0.003,
        pressure_pa: column.pressure_pa,
        heat_conductance_m_s: 0.025,
        vapor_conductance_m_s: 0.02,
        forcing_sha256: Sha256Digest::try_new("aa".repeat(32)).expect("typed forcing digest"),
        exposure_sha256: Sha256Digest::try_new("bb".repeat(32)).expect("typed exposure digest"),
        payload_sha256: Sha256Digest::try_new("cc".repeat(32)).expect("typed payload digest"),
    };
    exchange.payload_sha256 = exchange
        .reconstructed_sha256()
        .expect("actual native exchange seal");
    exchange
}

#[test]
fn m1_diagnostic_envelope_binds_actual_topology_and_rejects_structural_poison_first() {
    let original: Value = serde_json::from_str(ORIGINAL).expect("original input");
    let binding = topology_binding();
    assert_eq!(
        binding.occupancy_ids,
        [
            "stratum-z-upper::forest".to_owned(),
            "stratum-a-lower::forest".to_owned(),
        ]
    );
    assert_eq!(
        binding.soil_layer_ids,
        [
            "thermal-1".to_owned(),
            "thermal-2".to_owned(),
            "soil-1".to_owned(),
            "soil-2".to_owned(),
            "soil-dry".to_owned(),
            "soil-frozen".to_owned(),
        ]
    );

    let mut wrong_occupancy_count = covered_input(&original);
    wrong_occupancy_count.occupancies.pop();
    assert_m1_diagnostic_envelope(
        evaluate_m1_coupled_column(
            &raw_column_input(wrong_occupancy_count, conditions()),
            &trial(),
        )
        .expect_err("one occupancy is outside the M1 diagnostic envelope"),
    );

    let mut wrong_occupancy_order = covered_input(&original);
    wrong_occupancy_order.occupancies.swap(0, 1);
    assert_m1_diagnostic_envelope(
        evaluate_m1_coupled_column(
            &raw_column_input(wrong_occupancy_order, conditions()),
            &trial(),
        )
        .expect_err("occupancy order is outside the M1 diagnostic envelope"),
    );

    let mut wrong_occupancy_identity = covered_input(&original);
    wrong_occupancy_identity.occupancies[1].occupancy_id = "wrong-occupancy".to_owned();
    assert_m1_diagnostic_envelope(
        evaluate_m1_coupled_column(
            &raw_column_input(wrong_occupancy_identity, conditions()),
            &trial(),
        )
        .expect_err("occupancy identity is outside the M1 diagnostic envelope"),
    );

    let mut wrong_soil_count = covered_input(&original);
    wrong_soil_count.ground.soil_nodes.pop();
    assert_m1_diagnostic_envelope(
        evaluate_m1_coupled_column(&raw_column_input(wrong_soil_count, conditions()), &trial())
            .expect_err("five soil nodes are outside the M1 diagnostic envelope"),
    );

    let mut wrong_soil_identity = covered_input(&original);
    wrong_soil_identity.ground.soil_nodes[5].layer_id = "wrong-soil".to_owned();
    assert_m1_diagnostic_envelope(
        evaluate_m1_coupled_column(
            &raw_column_input(wrong_soil_identity, conditions()),
            &trial(),
        )
        .expect_err("soil identity is outside the M1 diagnostic envelope"),
    );

    let mut wrong_soil_order = covered_input(&original);
    wrong_soil_order.ground.soil_nodes.swap(0, 1);
    assert_m1_diagnostic_envelope(
        evaluate_m1_coupled_column(&raw_column_input(wrong_soil_order, conditions()), &trial())
            .expect_err("soil order is outside the M1 diagnostic envelope"),
    );

    let mut wrong_boundary_mode = covered_input(&original);
    wrong_boundary_mode.authority = CoveredColumnAuthority::HistoricalV8;
    assert_m1_diagnostic_envelope(
        evaluate_m1_coupled_column(
            &raw_column_input(wrong_boundary_mode, conditions()),
            &trial(),
        )
        .expect_err("historical lower-boundary mode is outside the M1 diagnostic envelope"),
    );

    let mut native_exchange_present = covered_input(&original);
    let native_exchange = native_exchange_presence_poison(&native_exchange_present);
    native_exchange_present
        .stage3_lower_boundary
        .as_mut()
        .expect("actual V11 lower boundary")
        .native_snow_exchange = Some(native_exchange);
    assert_m1_diagnostic_envelope(
        evaluate_m1_coupled_column(
            &raw_column_input(native_exchange_present, conditions()),
            &trial(),
        )
        .expect_err("native exchange presence is outside the M1 diagnostic envelope"),
    );

    let mut structural_before_pressure = covered_input(&original);
    structural_before_pressure.occupancies.pop();
    structural_before_pressure.pressure_pa = f64::NAN;
    assert_m1_diagnostic_envelope(
        evaluate_m1_coupled_column(
            &raw_column_input(structural_before_pressure, conditions()),
            &trial(),
        )
        .expect_err("structural envelope precedes invalid pressure"),
    );

    let mut missing_condition_key = conditions();
    missing_condition_key.occupancy_conditions.pop();
    let error = evaluate_m1_coupled_column(
        &raw_column_input(covered_input(&original), missing_condition_key),
        &trial(),
    )
    .expect_err("valid topology with missing condition key remains VEG-E-140");
    assert_eq!(error.code(), "VEG-E-140");
}

#[test]
fn m1_positive_top_liquid_requires_explicit_finite_nonnegative_enthalpy() {
    let radiation: Value = serde_json::from_str(RADIATION).expect("radiation");
    let original: Value = serde_json::from_str(ORIGINAL).expect("original input");
    let positive_top_rain = |top_liquid_specific_enthalpy_j_kg: Option<f64>| {
        let input = radiation_input_with_conditions_and_top_rain(
            &radiation,
            &original,
            conditions(),
            decimal(&radiation["inputs"]["p"]),
            0.001,
        );
        match top_liquid_specific_enthalpy_j_kg {
            Some(value) => input.with_top_liquid_specific_enthalpy(value),
            None => input,
        }
    };

    for top_liquid_specific_enthalpy_j_kg in [None, Some(-1.0), Some(f64::NAN), Some(f64::INFINITY)]
    {
        let error = evaluate_m1_coupled_column(
            &positive_top_rain(top_liquid_specific_enthalpy_j_kg),
            &trial(),
        )
        .expect_err("positive top liquid without a finite nonnegative enthalpy is rejected");
        assert_eq!(error.code(), "VEG-E-142");
    }

    let evaluation = evaluate_m1_coupled_column(&positive_top_rain(Some(0.0)), &trial())
        .expect("explicit zero-enthalpy liquid input is admitted");
    assert_eq!(
        evaluation.reservoir_operands()[0].incident_enthalpy_j_kg,
        0.0
    );
}
fn radiation_input_with_conditions_and_top_rain(
    root: &Value,
    original: &Value,
    support: M1SupportConditions,
    pressure_pa: f64,
    top_rain_kg_m2_tile: f64,
) -> M1CoupledColumnInput {
    let x = &root["inputs"];
    let mut column = covered_input(original);
    column.pressure_pa = pressure_pa;
    column.air_temperature_k = decimal(&x["tcan"]);
    column.air_specific_humidity_kg_kg = decimal(&x["qcan"]);
    column.interval_s = decimal(&x["dt"]);
    column.top_rain_kg_m2_tile = top_rain_kg_m2_tile;
    column.atmospheric_downward_longwave_w_m2 = decimal(&x["lw_down"]);
    column.ground.air_pressure_pa = column.pressure_pa;
    column.ground.air_temperature_k = column.air_temperature_k;
    column.ground.air_specific_humidity_kg_kg = column.air_specific_humidity_kg_kg;
    column.ground.atmospheric_downward_longwave_w_m2 = column.atmospheric_downward_longwave_w_m2;
    for (index, occupancy) in column.occupancies.iter_mut().enumerate() {
        occupancy.sun.vcmax25 = occupancy.shade.vcmax25;
        occupancy.sun.jmax25 = occupancy.shade.jmax25;
        occupancy.sun.rd25 = occupancy.shade.rd25;
        occupancy.sun.leaf_area_m2_m2_tile = decimal(&x["sun_area"][index]);
        occupancy.shade.leaf_area_m2_m2_tile = decimal(&x["shade_area"][index]);
        occupancy.stem_area_m2_m2_tile = decimal(&x["stem_area"][index]);
        occupancy.lai = occupancy.sun.leaf_area_m2_m2_tile + occupancy.shade.leaf_area_m2_m2_tile;
        occupancy.sai = occupancy.stem_area_m2_m2_tile;
        occupancy.clumping_index = decimal(&x["clumping"][index]);
        occupancy.gb_wet_m_s = decimal(&x["gb"][index]);
        occupancy.liquid_capacity_kg_m2_plant =
            decimal(&x["capacity"][index]) / decimal(&x["plant_area"][index]);
        let sw = decimal(&x["shortwave_total"][index]);
        occupancy.sun.absorbed_shortwave_w_m2_tile = sw;
        occupancy.sun.absorbed_par_w_m2_leaf = 0.0;
        occupancy.shade.absorbed_shortwave_w_m2_tile = 0.0;
        occupancy.shade.absorbed_par_w_m2_leaf = 0.0;
        occupancy.stem_absorbed_shortwave_w_m2_tile = 0.0;
        let receipt = column
            .shortwave
            .occupancies
            .iter_mut()
            .find(|receipt| receipt.occupancy_id == occupancy.occupancy_id)
            .expect("shortwave identity");
        receipt.sun_leaf_absorbed_w_m2_tile = BandDirectionalFluxes {
            direct_nir: sw,
            ..BandDirectionalFluxes::default()
        };
        receipt.shade_leaf_absorbed_w_m2_tile = BandDirectionalFluxes::default();
        receipt.stem_absorbed_w_m2_tile = BandDirectionalFluxes::default();
    }
    // The explicit two occupancy NIR absorption is the whole prescribed
    // incident shortwave control: 14 + 9 W m-2, no reflected or ground term.
    column.shortwave.incident_w_m2_tile = BandDirectionalFluxes {
        direct_nir: decimal(&x["shortwave_total"][0]) + decimal(&x["shortwave_total"][1]),
        ..BandDirectionalFluxes::default()
    };
    column.shortwave.top_reflected_w_m2_tile = BandDirectionalFluxes::default();
    column.shortwave.ground_absorbed_by_incident_w_m2_tile = BandDirectionalFluxes::default();
    M1CoupledColumnInput::from_covered_column(
        column,
        [
            M1PhaseReservoirInput::new(
                "stratum-z-upper::forest",
                decimal(&x["state"][0]),
                decimal(&x["state"][1]),
            ),
            M1PhaseReservoirInput::new(
                "stratum-a-lower::forest",
                decimal(&x["state"][2]),
                decimal(&x["state"][3]),
            ),
        ],
        support,
        topology_binding(),
    )
}
fn radiation_input_with_conditions(
    root: &Value,
    original: &Value,
    support: M1SupportConditions,
    pressure_pa: f64,
) -> M1CoupledColumnInput {
    radiation_input_with_conditions_and_top_rain(root, original, support, pressure_pa, 0.0)
}
fn radiation_input(root: &Value, original: &Value) -> M1CoupledColumnInput {
    radiation_input_with_conditions(root, original, conditions(), decimal(&root["inputs"]["p"]))
}
fn trial() -> M1CoupledColumnTrial {
    M1CoupledColumnTrial::from_coordinates([
        267., 266., 268., 0.018, -6385.68, 0., 269., 267., 270., 0.023, -7917.29, 0., 268., 0.0015,
        265.15, 291.5, 289.8, 293., 293., 293., 270.,
    ])
}

// The Stage 2 private controls consume this existing original-input fixture;
// they must not reconstruct a controller input from an oracle trace.
#[cfg(feature = "m1-trust-region-controller-stage")]
pub(super) fn trust_controller_fixture() -> (M1CoupledColumnInput, M1CoupledColumnTrial) {
    let original: Value = serde_json::from_str(ORIGINAL).expect("original input");
    let input = M1CoupledColumnInput::from_covered_column(
        covered_input(&original),
        [
            M1PhaseReservoirInput::new("stratum-z-upper::forest", 0.018, 0.0),
            M1PhaseReservoirInput::new("stratum-a-lower::forest", 0.018, 0.0),
        ],
        conditions(),
        topology_binding(),
    );
    (input, trial())
}
fn gas_probe_input(
    _root: &Value,
    row: &Value,
    original: &Value,
) -> (M1CoupledColumnInput, M1CoupledColumnTrial) {
    let mut column = covered_input(original);
    column.pressure_pa = 90_000.0;
    column.air_temperature_k = 268.0;
    column.air_specific_humidity_kg_kg = 0.0015;
    column.ground.air_pressure_pa = column.pressure_pa;
    column.ground.air_temperature_k = column.air_temperature_k;
    column.ground.air_specific_humidity_kg_kg = column.air_specific_humidity_kg_kg;
    column.ground.atmospheric_downward_longwave_w_m2 = column.atmospheric_downward_longwave_w_m2;
    let mut masses = [0.0; 2];
    for (index, occupancy) in column.occupancies.iter_mut().enumerate() {
        occupancy.sun.leaf_area_m2_m2_tile = 0.0;
        occupancy.shade.leaf_area_m2_m2_tile = 1.0;
        occupancy.lai = 1.0;
        let capacity = occupancy.liquid_capacity_kg_m2_plant * (occupancy.lai + occupancy.sai);
        masses[index] = capacity * 0.25_f64.powf(1.5);
        occupancy.beginning_canopy_liquid_kg_m2_tile = masses[index];
    }
    let input = M1CoupledColumnInput::from_covered_column(
        column,
        [
            M1PhaseReservoirInput::new("stratum-z-upper::forest", masses[0], 0.0),
            M1PhaseReservoirInput::new("stratum-a-lower::forest", masses[1], 0.0),
        ],
        conditions(),
        topology_binding(),
    );
    let temperature = decimal(&row["leaf_temperature_k"]);
    let row_trial = M1CoupledColumnTrial::from_coordinates([
        temperature,
        temperature,
        temperature,
        masses[0],
        0.0,
        0.0,
        temperature,
        temperature,
        temperature,
        masses[1],
        0.0,
        0.0,
        268.0,
        0.0015,
        265.15,
        291.5,
        289.8,
        293.0,
        293.0,
        293.0,
        270.0,
    ]);
    (input, row_trial)
}
fn independently_reconstruct_reservoirs(value: &M1CoupledEvaluation, must_close: bool) {
    for o in value.reservoir_operands() {
        let rm = o.ending_mass_kg_m2
            - o.beginning_mass_kg_m2
            - o.dt_s * (o.incident_liquid_kg_m2_s - o.vapor_kg_m2_s - o.drainage_kg_m2_s);
        let t = o.wet_temperature_k.expect("nonempty reservoir");
        let rh = o.ending_enthalpy_j_m2
            - o.beginning_enthalpy_j_m2
            - o.dt_s
                * (o.non_vapor_heat_w_m2 + o.incident_liquid_kg_m2_s * o.incident_enthalpy_j_kg
                    - o.vapor_kg_m2_s * (2_501_000. + 1_849. * (t - 273.15))
                    - o.drainage_kg_m2_s * 4_218. * (t - 273.15));
        assert!((rm - o.mass_residual_kg_m2).abs() <= 1e-12);
        assert!((rh - o.enthalpy_residual_j_m2).abs() <= 1e-9);
        if must_close {
            assert!(rm.abs() <= 1e-9 && rh.abs() <= 1e-6);
        }
        assert!(
            o.diagnosed_liquid_mass_kg_m2 >= 0.
                && o.diagnosed_liquid_mass_kg_m2 <= o.liquid_capacity_kg_m2
        );
    }
}

#[test]
fn m1_prescribed_twenty_one_coordinate_physical_evaluator_has_all_fd_columns() {
    let root: Value = serde_json::from_str(RADIATION).expect("radiation");
    let original: Value = serde_json::from_str(ORIGINAL).expect("original");
    let input = radiation_input(&root, &original);
    let trial = trial();
    let base = evaluate_m1_coupled_column(&input, &trial).expect("actual evaluator");
    assert_eq!(base.residuals().len(), 21);
    assert!(
        base.jacobian()
            .expect("diagnostic jacobian")
            .iter()
            .all(|row| row.len() == 21)
    );
    for coordinate in 0..21 {
        let h = match coordinate {
            3 | 9 => 1e-8,
            4 | 10 => 1e-3,
            5 | 11 => 1e-10,
            13 => 1e-8,
            _ => 1e-4,
        };
        let high = evaluate_m1_coupled_column(
            &input,
            &trial
                .with_coordinate_offset(coordinate, h)
                .expect("valid coordinate offset"),
        )
        .expect("inward/high probe");
        let fd = if matches!(coordinate, 5 | 11) {
            high.residuals()
                .iter()
                .zip(base.residuals())
                .map(|(a, b)| (a - b) / h)
                .collect::<Vec<_>>()
        } else {
            let low = evaluate_m1_coupled_column(
                &input,
                &trial
                    .with_coordinate_offset(coordinate, -h)
                    .expect("valid coordinate offset"),
            )
            .expect("low probe");
            high.residuals()
                .iter()
                .zip(low.residuals())
                .map(|(a, b)| (a - b) / (2. * h))
                .collect::<Vec<_>>()
        };
        assert_eq!(fd.len(), 21);
        let column = base
            .jacobian_column(coordinate)
            .expect("valid Jacobian column");
        assert_eq!(column.len(), 21);
        for (actual, analytic) in fd.iter().zip(column) {
            assert!(actual.is_finite() && analytic.is_finite());
            assert!((actual - analytic).abs() <= 1e-6 * actual.abs().max(1.));
        }
    }
    independently_reconstruct_reservoirs(&base, false);
}

// This first non-target physical-adapter discriminator is initialization only.
// It deliberately does not enter a proposal, root, or materialization path.
#[cfg(feature = "m1-trust-region-physical-stage")]
#[test]
fn m1_trust_region_physical_budget_core_cap_control_reaches_actual_owner() {
    let radiation: Value = serde_json::from_str(RADIATION).expect("radiation");
    let original: Value = serde_json::from_str(ORIGINAL).expect("original");
    let input = radiation_input(&radiation, &original);
    let protected_input = input.clone();
    let protected_trial = trial();
    super::m1_reset_loop_activity_observation_for_test();
    let mut adapter = super::M1PhysicalTrustRegionAdapter::new(input, protected_trial.clone());
    let scope = super::m1_physical_budget_scope_seeded_for_test(super::M1PhysicalBudgetSnapshot {
        core_attempted: 6_499,
        core: 6_499,
        ..Default::default()
    });
    let error = super::m1_trust_region_physical_adapter_initialize_for_test(&mut adapter)
        .expect_err(
            "the #6,500 base core enters and the #6,501 first finite-difference core must refuse",
        );
    assert!(matches!(
        error,
        super::m1_trust_region_controller::M1TrustRegionControllerError::Own(
            super::M1PhysicalTrustRegionAdapterOwnError::Coupled(error)
        ) if matches!(*error, M1CoupledError::PhysicalBudget(super::M1PhysicalBudgetRefusal {
                operation: super::M1PhysicalBudgetOperation::Core,
                snapshot: super::M1PhysicalBudgetSnapshot {
                    core_attempted: 6_501, core: 6_500, raw_jacobian_attempted: 1,
                    raw_jacobian: 1, ..
                },
            }))
    ));
    let runtime = super::m1_loop_activity_observation_for_test();
    assert_eq!(runtime.complete_evaluator_entries, 1);
    assert_eq!(runtime.hydraulic_entries, 0);
    assert_eq!(runtime.actual_hydraulic_entries, 0);
    assert_eq!(runtime.materialization_entries, 0);
    assert_eq!(adapter.input, protected_input);
    assert_eq!(adapter.initial_trial, protected_trial);
    drop(scope);
    assert_eq!(super::m1_physical_budget_observation_for_test(), None);
}

#[cfg(feature = "m1-trust-region-physical-stage")]
#[test]
fn m1_trust_region_physical_budget_raw_jacobian_cap_control_reaches_actual_owner() {
    let radiation: Value = serde_json::from_str(RADIATION).expect("radiation");
    let original: Value = serde_json::from_str(ORIGINAL).expect("original");
    let input = radiation_input(&radiation, &original);
    super::m1_reset_loop_activity_observation_for_test();
    let scope = super::m1_physical_budget_scope_seeded_for_test(super::M1PhysicalBudgetSnapshot {
        raw_jacobian_attempted: 99,
        raw_jacobian: 99,
        ..Default::default()
    });
    let mut allowed = super::M1PhysicalTrustRegionAdapter::new(input.clone(), trial());
    super::m1_trust_region_physical_adapter_initialize_for_test(&mut allowed)
        .expect("the one hundredth raw-J assembly must enter");
    let mut denied = super::M1PhysicalTrustRegionAdapter::new(input, trial());
    let error = super::m1_trust_region_physical_adapter_initialize_for_test(&mut denied)
        .expect_err("the 101st raw-J assembly must refuse before a new core");
    assert!(matches!(
        error,
        super::m1_trust_region_controller::M1TrustRegionControllerError::Own(
            super::M1PhysicalTrustRegionAdapterOwnError::Coupled(error)
        ) if matches!(*error, M1CoupledError::PhysicalBudget(super::M1PhysicalBudgetRefusal {
                operation: super::M1PhysicalBudgetOperation::RawJacobian,
                snapshot: super::M1PhysicalBudgetSnapshot {
                    raw_jacobian_attempted: 101, raw_jacobian: 100, core_attempted: 41,
                    core: 41, ..
                },
            }))
    ));
    let runtime = super::m1_loop_activity_observation_for_test();
    assert_eq!(runtime.complete_evaluator_entries, 41);
    assert_eq!(runtime.hydraulic_entries, 0);
    assert_eq!(runtime.actual_hydraulic_entries, 0);
    assert_eq!(runtime.materialization_entries, 0);
    drop(scope);
    assert_eq!(super::m1_physical_budget_observation_for_test(), None);
}

#[cfg(feature = "m1-trust-region-physical-stage")]
#[test]
fn m1_trust_region_physical_budget_hydraulic_cap_control_reaches_actual_owner() {
    let radiation: Value = serde_json::from_str(RADIATION).expect("radiation");
    let original: Value = serde_json::from_str(ORIGINAL).expect("original");
    let input = radiation_input(&radiation, &original);
    let protected_trial = trial();
    super::m1_reset_loop_activity_observation_for_test();
    let scope = super::m1_physical_budget_scope_seeded_for_test(super::M1PhysicalBudgetSnapshot {
        hydraulic_attempted: 2_201,
        hydraulic: 2_201,
        ..Default::default()
    });
    let mut allowed =
        super::M1PhysicalTrustRegionAdapter::new(input.clone(), protected_trial.clone());
    let allowed_error =
        super::m1_trust_region_physical_adapter_hydraulic_height_poison_for_test(&mut allowed)
            .expect_err("the 2,202nd hydraulic entry reaches its own nonfinite-height guard");
    assert!(
        matches!(allowed_error, super::M1PhysicalTrustRegionAdapterOwnError::Coupled(error) if error.code() == "VEG-E-141")
    );
    assert_eq!(allowed.input, input);
    assert_eq!(allowed.initial_trial, protected_trial);
    let mut denied = super::M1PhysicalTrustRegionAdapter::new(input.clone(), trial());
    let error =
        super::m1_trust_region_physical_adapter_hydraulic_height_poison_for_test(&mut denied)
            .expect_err("the 2,203rd hydraulic entry must refuse before the owner guard");
    assert!(matches!(
        error,
        super::M1PhysicalTrustRegionAdapterOwnError::Coupled(error)
            if matches!(*error, M1CoupledError::PhysicalBudget(super::M1PhysicalBudgetRefusal {
                operation: super::M1PhysicalBudgetOperation::Hydraulic,
                snapshot: super::M1PhysicalBudgetSnapshot {
                    core_attempted: 82, core: 82,
                    raw_jacobian_attempted: 2, raw_jacobian: 2,
                    hydraulic_attempted: 2_203, hydraulic: 2_202,
                    materialization_attempted: 0, materialization: 0,
                    ..
                },
            }))
    ));
    let runtime = super::m1_loop_activity_observation_for_test();
    assert_eq!(runtime.complete_evaluator_entries, 82);
    assert_eq!(runtime.hydraulic_entries, 2);
    assert_eq!(runtime.actual_hydraulic_entries, 1);
    assert_eq!(runtime.materialization_entries, 0);
    assert_eq!(denied.input, input);
    drop(scope);
    assert_eq!(super::m1_physical_budget_observation_for_test(), None);
}

#[cfg(feature = "m1-trust-region-physical-stage")]
#[test]
fn m1_trust_region_physical_budget_stage1_factorization_cap_reaches_actual_face() {
    let original: Value = serde_json::from_str(ORIGINAL).expect("original");
    let input = M1CoupledColumnInput::from_covered_column(
        covered_input(&original),
        [
            M1PhaseReservoirInput::new("stratum-z-upper::forest", 0.018, 0.0),
            M1PhaseReservoirInput::new("stratum-a-lower::forest", 0.018, 0.0),
        ],
        conditions(),
        topology_binding(),
    );
    let mut identity = [[0.0; 21]; 21];
    for (index, row) in identity.iter_mut().enumerate() {
        row[index] = 1.0;
    }
    let subproblem = super::M1TrustRegionSubproblemInput {
        raw_residual: [0.25; 21],
        raw_jacobian: identity,
        normalizers: [1.0; 21],
        scales: [1.0; 21],
        lower: [f64::NEG_INFINITY; 21],
        upper: [f64::INFINITY; 21],
        initial_radius: 1.0,
        base: [0.0; 21],
        test_work: super::M1TrustRegionTestWorkState::default(),
    };
    let scope = super::m1_physical_budget_scope_seeded_for_test(super::M1PhysicalBudgetSnapshot {
        stage1_factorization_attempted: 47_299,
        stage1_factorization: 47_299,
        ..Default::default()
    });
    super::m1_trust_region_solve_subproblem_for_test(&input, &trial(), &subproblem)
        .expect("the 47,300th face factorization enters");
    let refusal = super::m1_trust_region_solve_subproblem_for_test(&input, &trial(), &subproblem)
        .expect_err("the 47,301st face factorization must refuse before local entry");
    assert_eq!(refusal.kind, super::M1TrustRegionRefusalKind::WorkCap);
    assert_eq!(refusal.entered_svd_factorizations, 0);
    assert!(matches!(
        refusal.budget_refusal,
        Some(super::M1PhysicalBudgetRefusal {
            operation: super::M1PhysicalBudgetOperation::Stage1Factorization,
            snapshot: super::M1PhysicalBudgetSnapshot {
                stage1_factorization_attempted: 47_301,
                stage1_factorization: 47_300,
                ..
            },
        })
    ));
    drop(scope);
    assert_eq!(super::m1_physical_budget_observation_for_test(), None);
}

#[cfg(feature = "m1-trust-region-physical-stage")]
#[test]
fn m1_trust_region_physical_budget_materialization_cap_reaches_actual_owner() {
    let original: Value = serde_json::from_str(ORIGINAL).expect("original");
    let input = M1CoupledColumnInput::from_covered_column(
        covered_input(&original),
        [
            M1PhaseReservoirInput::new("stratum-z-upper::forest", 0.018, 0.0),
            M1PhaseReservoirInput::new("stratum-a-lower::forest", 0.018, 0.0),
        ],
        conditions(),
        topology_binding(),
    );
    super::m1_reset_accepted_capture_for_test();
    let root = solve_m1_coupled_column(&input)
        .expect("original60 provides the accepted materializer operands");
    independently_reconstruct_reservoirs(&root, true);
    let protected_input = input.clone();
    super::m1_reset_loop_activity_observation_for_test();
    let scope = super::m1_physical_budget_scope_for_test();
    let first = super::m1_materialize_captured_accepted_for_test()
        .expect("the first materialization enters with its actual accepted core");
    independently_reconstruct_reservoirs(&first, true);
    assert!(first.is_bound_to(&protected_input));
    let error = super::m1_materialize_captured_accepted_for_test()
        .expect_err("the second materialization must refuse before hydraulic or materializer work");
    assert!(matches!(
        error,
        M1CoupledError::PhysicalBudget(super::M1PhysicalBudgetRefusal {
            operation: super::M1PhysicalBudgetOperation::Materialization,
            snapshot: super::M1PhysicalBudgetSnapshot {
                core_attempted: 0,
                core: 0,
                raw_jacobian_attempted: 0,
                raw_jacobian: 0,
                hydraulic_attempted: 2,
                hydraulic: 2,
                materialization_attempted: 2,
                materialization: 1,
                ..
            },
        })
    ));
    let runtime = super::m1_loop_activity_observation_for_test();
    assert_eq!(runtime.complete_evaluator_entries, 0);
    assert_eq!(runtime.actual_hydraulic_entries, 2);
    assert_eq!(runtime.materialization_entries, 1);
    assert_eq!(protected_input, input);
    drop(scope);
    assert_eq!(super::m1_physical_budget_observation_for_test(), None);
}

#[cfg(feature = "m1-trust-region-physical-stage")]
#[test]
fn m1_trust_region_physical_budget_scope_observes_actual_initialization_and_restores() {
    let radiation: Value = serde_json::from_str(RADIATION).expect("radiation");
    let original: Value = serde_json::from_str(ORIGINAL).expect("original");
    let mut adapter =
        super::M1PhysicalTrustRegionAdapter::new(radiation_input(&radiation, &original), trial());
    let scope = super::m1_physical_budget_scope_for_test();
    super::m1_trust_region_physical_adapter_initialize_for_test(&mut adapter)
        .expect("actual initialization under private budget scope");
    assert_eq!(
        super::m1_physical_budget_observation_for_test(),
        Some(super::M1PhysicalBudgetSnapshot {
            core_attempted: 41,
            core: 41,
            raw_jacobian_attempted: 1,
            raw_jacobian: 1,
            hydraulic_attempted: 0,
            hydraulic: 0,
            stage1_factorization_attempted: 0,
            stage1_factorization: 0,
            materialization_attempted: 0,
            materialization: 0,
        })
    );
    drop(scope);
    assert_eq!(super::m1_physical_budget_observation_for_test(), None);
}

#[cfg(feature = "m1-trust-region-physical-stage")]
#[test]
fn m1_trust_region_physical_adapter_initialization_binds_real_current_operands() {
    let radiation: Value = serde_json::from_str(RADIATION).expect("radiation");
    let original: Value = serde_json::from_str(ORIGINAL).expect("original");
    let input = radiation_input(&radiation, &original);
    let initial_trial = trial();
    let protected_input = input.clone();
    let protected_trial = initial_trial.clone();
    super::m1_reset_loop_activity_observation_for_test();
    super::m1_reset_work_observation_for_test();
    let mut adapter = super::M1PhysicalTrustRegionAdapter::new(input, initial_trial);
    super::m1_trust_region_physical_adapter_initialize_for_test(&mut adapter)
        .expect("physical adapter initialization must use the shared foundation state");
    let observation = adapter.observation();
    let runtime = super::m1_loop_activity_observation_for_test();
    let (jacobian_invocations, controller_assemblies) = super::m1_work_observation_for_test();

    assert_eq!(observation.current_evaluations, 1);
    assert_eq!(observation.raw_jacobian_assemblies, 1);
    assert_eq!(observation.natural_phase_selections, 1);
    assert_eq!(observation.trial_domain_checks, 0);
    assert_eq!(observation.selected_side_probes, 0);
    assert_eq!(observation.materialization_entries, 0);
    assert_eq!(
        observation.current_hydraulics,
        super::m1_trust_region_controller_interface::M1TrustRegionCurrentHydraulics::Pending
    );
    let raw_residual = observation
        .raw_residual
        .expect("actual current raw residual");
    let normalizers = observation
        .dynamic_normalizers
        .expect("actual current dynamic normalizers");
    let raw_jacobian = observation
        .raw_jacobian
        .expect("actual current raw Jacobian");
    assert_eq!(
        raw_residual[14].to_bits(),
        (265.15
            - protected_input
                .column
                .stage3_lower_boundary
                .as_ref()
                .expect("native Stage-3 lower boundary")
                .snow_temperature_k)
            .to_bits()
    );
    assert_eq!(normalizers[14].to_bits(), 1.0e-9_f64.to_bits());
    // The returned raw matrix preserves the owner’s ordered centered FD at
    // the 265.15 K boundary with h=1e-4; it is not reconstructed as 1.0.
    assert_eq!(raw_jacobian[14][14].to_bits(), 0x3fef_ffff_ffdd_8000);
    assert!(
        raw_jacobian[14]
            .iter()
            .enumerate()
            .all(|(column, value)| column == 14 || value.to_bits() == 0.0_f64.to_bits())
    );
    assert_eq!(runtime.complete_evaluator_entries, 41);
    assert_eq!(runtime.hydraulic_entries, 0);
    assert_eq!(runtime.actual_hydraulic_entries, 0);
    assert_eq!(runtime.materialization_entries, 0);
    assert_eq!(jacobian_invocations, 1);
    assert_eq!(controller_assemblies, 0);
    assert_eq!(adapter.input, protected_input);
    assert_eq!(adapter.initial_trial, protected_trial);
}

#[cfg(feature = "m1-trust-region-physical-stage")]
#[test]
fn m1_trust_region_physical_adapter_original60_setup_is_separate_from_deferred_proposal() {
    let original: Value = serde_json::from_str(ORIGINAL).expect("original input");
    let input = M1CoupledColumnInput::from_covered_column(
        covered_input(&original),
        [
            M1PhaseReservoirInput::new("stratum-z-upper::forest", 0.018, 0.0),
            M1PhaseReservoirInput::new("stratum-a-lower::forest", 0.018, 0.0),
        ],
        conditions(),
        topology_binding(),
    );
    assert_eq!(
        input.column.occupancies[0]
            .sun
            .leaf_area_m2_m2_tile
            .to_bits(),
        0.0_f64.to_bits()
    );
    assert_eq!(
        input.column.occupancies[1]
            .sun
            .leaf_area_m2_m2_tile
            .to_bits(),
        0.0_f64.to_bits()
    );
    let setup = solve_m1_coupled_column(&input).expect("canonical non-target original60 setup");
    independently_reconstruct_reservoirs(&setup, true);
    assert!(setup.is_bound_to(&input));
    let accepted_coordinates = *setup.accepted_coordinates();
    let accepted_trial = M1CoupledColumnTrial::from_coordinates(accepted_coordinates);
    super::m1_reset_accepted_capture_for_test();
    super::m1_reset_loop_activity_observation_for_test();
    super::m1_reset_work_observation_for_test();
    let mut adapter =
        super::M1PhysicalTrustRegionAdapter::new(input.clone(), accepted_trial.clone());
    let result = super::m1_trust_region_physical_adapter_proposal_for_test(&mut adapter)
        .expect("accepted local physical iterate must materialize through the adapter");
    independently_reconstruct_reservoirs(&result, true);
    assert_eq!(
        result.accepted_coordinates().map(f64::to_bits),
        accepted_coordinates.map(f64::to_bits)
    );
    assert!(result.is_bound_to(&input));
    assert_eq!(
        result
            .jacobian()
            .expect_err("accepted materialization must retain no Jacobian")
            .code(),
        "LSEB-E-030"
    );
    let (captured_input, captured_coordinates) =
        super::m1_captured_accepted_adapter_binding_for_test()
            .expect("adapter materialization must publish fresh accepted capture");
    assert_eq!(captured_input, input);
    assert_eq!(
        captured_coordinates.map(f64::to_bits),
        accepted_coordinates.map(f64::to_bits)
    );
    assert_eq!(adapter.input, input);
    assert_eq!(adapter.initial_trial, accepted_trial);
    assert_eq!(adapter.observation().current_evaluations, 2);
    assert_eq!(adapter.observation().raw_jacobian_assemblies, 1);
    assert_eq!(
        adapter
            .observation()
            .raw_jacobian
            .expect("candidate observation must retain the owner base raw Jacobian")[14][14]
            .to_bits(),
        0x3fef_ffff_ffdd_8000
    );
    assert_eq!(adapter.observation().natural_phase_selections, 1);
    assert_eq!(adapter.observation().selected_side_probes, 1);
    assert_eq!(adapter.observation().trial_domain_checks, 1);
    assert_eq!(adapter.observation().materialization_entries, 1);
    assert!(matches!(
        adapter.observation().current_hydraulics,
        super::m1_trust_region_controller_interface::M1TrustRegionCurrentHydraulics::Complete(_)
    ));
    let run = adapter
        .observation()
        .controller_run
        .expect("actual controller run");
    assert_eq!(run.entered_materializations, 1);
    assert_eq!(
        run.final_coordinates.map(f64::to_bits),
        captured_coordinates.map(f64::to_bits)
    );
    assert_eq!(
        run.materialized_coordinates
            .map(|coordinates| coordinates.map(f64::to_bits)),
        Some(captured_coordinates.map(f64::to_bits))
    );
    assert_eq!(run.terminal_refusal, None);
    let runtime = super::m1_loop_activity_observation_for_test();
    // Original60 has no phase/capacity tie, so the selected-side callback
    // returns before its core probe: natural raw-J is 41 plus candidate core.
    assert_eq!(runtime.complete_evaluator_entries, 42);
    assert_eq!(runtime.hydraulic_entries, 2);
    assert_eq!(runtime.actual_hydraulic_entries, 6);
    assert_eq!(runtime.materialization_entries, 1);
    assert_eq!(super::m1_work_observation_for_test(), (1, 0));
}

#[cfg(feature = "m1-trust-region-physical-stage")]
#[test]
fn m1_trust_region_physical_adapter_run_captures_and_restores_owned_budget_scope() {
    let original: Value = serde_json::from_str(ORIGINAL).expect("original input");
    let input = M1CoupledColumnInput::from_covered_column(
        covered_input(&original),
        [
            M1PhaseReservoirInput::new("stratum-z-upper::forest", 0.018, 0.0),
            M1PhaseReservoirInput::new("stratum-a-lower::forest", 0.018, 0.0),
        ],
        conditions(),
        topology_binding(),
    );
    let setup = solve_m1_coupled_column(&input).expect("canonical non-target original60 setup");
    let accepted_trial = M1CoupledColumnTrial::from_coordinates(*setup.accepted_coordinates());
    super::m1_reset_accepted_capture_for_test();
    super::m1_reset_loop_activity_observation_for_test();
    let mut adapter =
        super::M1PhysicalTrustRegionAdapter::new(input.clone(), accepted_trial.clone());
    let result = super::m1_trust_region_physical_adapter_proposal_for_test(&mut adapter)
        .expect("actual original60 root must materialize through the same controller");
    independently_reconstruct_reservoirs(&result, true);
    assert!(result.is_bound_to(&input));
    assert_eq!(adapter.input, input);
    assert_eq!(adapter.initial_trial, accepted_trial);
    let budget = adapter
        .observation()
        .budget_snapshot
        .expect("accepted run must retain its solve-owned budget snapshot");
    assert_eq!((budget.core_attempted, budget.core), (42, 42));
    assert_eq!((budget.raw_jacobian_attempted, budget.raw_jacobian), (1, 1));
    assert_eq!((budget.hydraulic_attempted, budget.hydraulic), (6, 6));
    assert_eq!(
        (budget.materialization_attempted, budget.materialization),
        (1, 1)
    );
    let run = adapter
        .observation()
        .controller_run
        .expect("actual controller run");
    assert_eq!(
        budget.stage1_factorization,
        u32::from(run.entered_svd_factorizations)
    );
    assert_eq!(
        budget.stage1_factorization_attempted,
        budget.stage1_factorization
    );
    assert_eq!(super::m1_physical_budget_observation_for_test(), None);
}

#[cfg(feature = "m1-trust-region-physical-stage")]
#[test]
fn m1_trust_region_physical_adapter_run_restores_nested_prior_budget_scope() {
    let original: Value = serde_json::from_str(ORIGINAL).expect("original input");
    let input = M1CoupledColumnInput::from_covered_column(
        covered_input(&original),
        [
            M1PhaseReservoirInput::new("stratum-z-upper::forest", 0.018, 0.0),
            M1PhaseReservoirInput::new("stratum-a-lower::forest", 0.018, 0.0),
        ],
        conditions(),
        topology_binding(),
    );
    let setup = solve_m1_coupled_column(&input).expect("canonical non-target original60 setup");
    let prior = super::M1PhysicalBudgetSnapshot {
        core: 17,
        core_attempted: 17,
        ..Default::default()
    };
    let outer = super::m1_physical_budget_scope_seeded_for_test(prior);
    let mut adapter = super::M1PhysicalTrustRegionAdapter::new(
        input,
        M1CoupledColumnTrial::from_coordinates(*setup.accepted_coordinates()),
    );
    super::m1_trust_region_physical_adapter_proposal_for_test(&mut adapter)
        .expect("same controller root under nested prior scope");
    assert_eq!(
        adapter
            .observation()
            .budget_snapshot
            .map(|snapshot| snapshot.core),
        Some(42)
    );
    assert_eq!(
        super::m1_physical_budget_observation_for_test(),
        Some(prior)
    );
    drop(outer);
    assert_eq!(super::m1_physical_budget_observation_for_test(), None);
}

#[cfg(feature = "m1-trust-region-physical-stage")]
#[test]
fn m1_trust_region_physical_adapter_invalid_drainage_operand_is_refused_before_work() {
    let mut coordinates = super::m1_trial_coordinates_for_test(&trial());
    coordinates[5] = -f64::MIN_POSITIVE;
    let invalid = M1CoupledColumnTrial::from_coordinates(coordinates);
    let radiation: Value = serde_json::from_str(RADIATION).expect("radiation");
    let original: Value = serde_json::from_str(ORIGINAL).expect("original");
    let input = radiation_input(&radiation, &original);
    let protected_input = input.clone();
    super::m1_reset_loop_activity_observation_for_test();
    super::m1_reset_work_observation_for_test();
    let mut adapter = super::M1PhysicalTrustRegionAdapter::new(input, invalid.clone());
    let domain = <super::M1PhysicalTrustRegionAdapter as super::m1_trust_region_controller_interface::M1TrustRegionControllerEvaluator>::pre_evaluator_trial_domain(
        &mut adapter,
        coordinates,
    )
    .expect("actual FD-domain guard has no owning error");
    assert!(!domain);
    assert_eq!(
        super::m1_trial_coordinates_for_test(&invalid).map(f64::to_bits),
        coordinates.map(f64::to_bits)
    );
    assert_eq!(adapter.input, protected_input);
    assert_eq!(adapter.observation().current_evaluations, 0);
    assert_eq!(adapter.observation().raw_jacobian_assemblies, 0);
    assert_eq!(adapter.observation().natural_phase_selections, 0);
    assert_eq!(adapter.observation().trial_domain_checks, 1);
    assert_eq!(adapter.observation().selected_side_probes, 0);
    assert_eq!(adapter.observation().materialization_entries, 0);
    let runtime = super::m1_loop_activity_observation_for_test();
    assert_eq!(runtime.complete_evaluator_entries, 0);
    assert_eq!(runtime.hydraulic_entries, 0);
    assert_eq!(runtime.actual_hydraulic_entries, 0);
    assert_eq!(runtime.materialization_entries, 0);
    assert_eq!(super::m1_work_observation_for_test(), (0, 0));
}

#[cfg(feature = "m1-trust-region-physical-stage")]
#[test]
fn m1_trust_region_physical_adapter_fd_guard_does_not_substitute_validate() {
    let mut coordinates = super::m1_trial_coordinates_for_test(&trial());
    coordinates[14] = f64::NAN;
    let trial = M1CoupledColumnTrial::from_coordinates(coordinates);
    let radiation: Value = serde_json::from_str(RADIATION).expect("radiation");
    let original: Value = serde_json::from_str(ORIGINAL).expect("original");
    let input = radiation_input(&radiation, &original);
    let mut adapter = super::M1PhysicalTrustRegionAdapter::new(input, trial);
    super::m1_reset_loop_activity_observation_for_test();
    super::m1_reset_work_observation_for_test();
    let admitted = <super::M1PhysicalTrustRegionAdapter as super::m1_trust_region_controller_interface::M1TrustRegionControllerEvaluator>::pre_evaluator_trial_domain(
        &mut adapter,
        coordinates,
    )
    .expect("FD-domain classification must not invoke the full evaluator");
    assert!(admitted, "ground NaN is outside FD admissibility ownership");
    assert_eq!(adapter.observation().trial_domain_checks, 1);
    assert_eq!(adapter.observation().current_evaluations, 0);
    let runtime = super::m1_loop_activity_observation_for_test();
    assert_eq!(runtime.complete_evaluator_entries, 0);
    assert_eq!(runtime.hydraulic_entries, 0);
    assert_eq!(runtime.actual_hydraulic_entries, 0);
    assert_eq!(runtime.materialization_entries, 0);
    assert_eq!(super::m1_work_observation_for_test(), (0, 0));
}

#[cfg(feature = "m1-trust-region-physical-stage")]
#[test]
fn m1_trust_region_physical_adapter_candidate_natural_jacobian_reassembles() {
    let original: Value = serde_json::from_str(ORIGINAL).expect("original input");
    let input = M1CoupledColumnInput::from_covered_column(
        covered_input(&original),
        [
            M1PhaseReservoirInput::new("stratum-z-upper::forest", 0.018, 0.0),
            M1PhaseReservoirInput::new("stratum-a-lower::forest", 0.018, 0.0),
        ],
        conditions(),
        topology_binding(),
    );
    let setup = solve_m1_coupled_column(&input).expect("canonical non-target original60 setup");
    let coordinates = *setup.accepted_coordinates();
    super::m1_reset_loop_activity_observation_for_test();
    super::m1_reset_work_observation_for_test();
    let mut adapter = super::M1PhysicalTrustRegionAdapter::new(
        input,
        M1CoupledColumnTrial::from_coordinates(coordinates),
    );
    let raw = super::m1_trust_region_physical_adapter_candidate_raw_jacobian_for_test(
        &mut adapter,
        coordinates,
    )
    .expect("candidate natural tangent must assemble through the owner");
    assert_eq!(raw[14][14].to_bits(), 0x3fef_ffff_ffdd_8000);
    assert!(
        raw[14]
            .iter()
            .enumerate()
            .all(|(column, value)| column == 14 || value.to_bits() == 0.0_f64.to_bits())
    );
    assert_eq!(adapter.observation().current_evaluations, 2);
    assert_eq!(adapter.observation().raw_jacobian_assemblies, 2);
    assert_eq!(adapter.observation().natural_phase_selections, 1);
    assert_eq!(adapter.observation().trial_domain_checks, 0);
    assert_eq!(adapter.observation().selected_side_probes, 0);
    assert_eq!(adapter.observation().materialization_entries, 0);
    assert!(matches!(
        adapter.observation().current_hydraulics,
        super::m1_trust_region_controller_interface::M1TrustRegionCurrentHydraulics::Complete(_)
    ));
    let runtime = super::m1_loop_activity_observation_for_test();
    assert_eq!(runtime.complete_evaluator_entries, 83);
    assert_eq!(runtime.hydraulic_entries, 1);
    assert_eq!(runtime.actual_hydraulic_entries, 2);
    assert_eq!(runtime.materialization_entries, 0);
    assert_eq!(super::m1_work_observation_for_test(), (2, 0));
}

#[cfg(feature = "m1-trust-region-physical-stage")]
#[test]
fn m1_face_pivot_capture_scope_is_opt_in_nested_and_error_preserving() {
    assert!(!super::m1_face_pivot_capture_enabled_for_test());
    let outer = super::m1_face_pivot_capture_begin_for_test();
    assert!(super::m1_face_pivot_capture_enabled_for_test());
    {
        let inner = super::m1_face_pivot_capture_begin_for_test();
        assert!(super::m1_face_pivot_capture_enabled_for_test());
        assert!(super::m1_face_pivot_capture_take_for_test().is_none());
        drop(inner);
    }
    assert!(super::m1_face_pivot_capture_enabled_for_test());
    let original_error: Result<(), &'static str> = Err("original typed error");
    drop(outer);
    assert!(!super::m1_face_pivot_capture_enabled_for_test());
    assert_eq!(original_error, Err("original typed error"));
}

#[cfg(feature = "m1-trust-region-physical-stage")]
#[test]
fn m1_trust_region_physical_adapter_ordinary_update_reassembles_before_root_materialization() {
    let original: Value = serde_json::from_str(ORIGINAL).expect("original input");
    let input = M1CoupledColumnInput::from_covered_column(
        covered_input(&original),
        [
            M1PhaseReservoirInput::new("stratum-z-upper::forest", 0.018, 0.0),
            M1PhaseReservoirInput::new("stratum-a-lower::forest", 0.018, 0.0),
        ],
        conditions(),
        topology_binding(),
    );
    let setup = solve_m1_coupled_column(&input).expect("canonical non-target original60 setup");
    let accepted_coordinates = *setup.accepted_coordinates();
    let mut perturbed_coordinates = accepted_coordinates;
    perturbed_coordinates[20] += 1.0e-4;
    let setup_coordinate_bits = accepted_coordinates.map(f64::to_bits);
    let perturbed_coordinate_bits = perturbed_coordinates.map(f64::to_bits);
    assert!(perturbed_coordinates[20] - accepted_coordinates[20] > 1.0e-8);
    assert!(perturbed_coordinates[20] - accepted_coordinates[20] < 1.0);
    let protected_input = input.clone();
    let protected_trial = M1CoupledColumnTrial::from_coordinates(perturbed_coordinates);
    super::m1_reset_accepted_capture_for_test();
    super::m1_reset_loop_activity_observation_for_test();
    super::m1_reset_work_observation_for_test();
    let prior_budget = super::m1_physical_budget_observation_for_test();
    let mut adapter = super::M1PhysicalTrustRegionAdapter::new(input, protected_trial.clone());
    let capture_scope = super::m1_face_pivot_capture_begin_for_test();
    let observed_result = super::m1_trust_region_physical_adapter_proposal_for_test(&mut adapter);
    let face_capture = super::m1_face_pivot_capture_take_for_test();
    let face_capture_json = super::m1_face_pivot_capture_json_for_test(face_capture.as_ref());
    let phase_events_json = adapter.observation().face_pivot_phase_json_for_test();
    let accepted_capture_is_none = super::m1_captured_accepted_adapter_binding_for_test().is_none();
    let restored_budget = super::m1_physical_budget_observation_for_test();
    let _ = std::io::Write::write_fmt(
        &mut std::io::stderr().lock(),
        format_args!(
            "FACE_PIVOT_SETUP accepted_bits={setup_coordinate_bits:?} perturbed_bits={perturbed_coordinate_bits:?}\nFACE_PIVOT_CAPTURE {face_capture_json}\nFACE_PIVOT_PHASES {phase_events_json}\nFACE_PIVOT_ADAPTER {:#?}\nFACE_PIVOT_RUNTIME {:#?}\nFACE_PIVOT_WORK {:?}\nFACE_PIVOT_BUDGET prior={prior_budget:#?} restored={restored_budget:#?}\nFACE_PIVOT_OUTER_BUDGET {:#?}\nFACE_PIVOT_PROTECTED input={} trial={} accepted_capture_none={}\n",
            adapter.observation(),
            super::m1_loop_activity_observation_for_test(),
            super::m1_work_observation_for_test(),
            super::m1_physical_budget_observation_for_test(),
            adapter.input == protected_input,
            adapter.initial_trial == protected_trial,
            accepted_capture_is_none,
        ),
    );
    drop(capture_scope);
    let result = observed_result
        .expect("ordinary accepted update must reassemble its nonroot base and materialize root");
    independently_reconstruct_reservoirs(&result, true);
    assert!(result.is_bound_to(&protected_input));
    let (captured_input, captured_coordinates) =
        super::m1_captured_accepted_adapter_binding_for_test()
            .expect("ordinary root materialization must publish fresh accepted capture");
    assert_eq!(captured_input, protected_input);
    assert_eq!(
        result.accepted_coordinates().map(f64::to_bits),
        captured_coordinates.map(f64::to_bits)
    );
    assert_eq!(
        result
            .jacobian()
            .expect_err("adapter materialization must retain no Jacobian")
            .code(),
        "LSEB-E-030"
    );
    assert_eq!(adapter.input, protected_input);
    assert_eq!(adapter.initial_trial, protected_trial);
    let run = adapter
        .observation()
        .controller_run
        .expect("actual ordinary controller run observation");
    assert_eq!(run.proposals, 2);
    assert_eq!(run.accepted_updates, 1);
    assert_eq!(run.installed_updates, 1);
    assert_eq!(run.entered_materializations, 1);
    assert_eq!(
        run.final_coordinates.map(f64::to_bits),
        captured_coordinates.map(f64::to_bits)
    );
    assert_eq!(
        run.materialized_coordinates
            .map(|coordinates| coordinates.map(f64::to_bits)),
        Some(captured_coordinates.map(f64::to_bits))
    );
    assert_ne!(
        run.final_coordinates.map(f64::to_bits),
        perturbed_coordinates.map(f64::to_bits)
    );
    assert_eq!(run.terminal_refusal, None);
    assert_eq!(adapter.observation().current_evaluations, 3);
    assert_eq!(adapter.observation().raw_jacobian_assemblies, 2);
    assert_eq!(adapter.observation().trial_domain_checks, 2);
    assert_eq!(adapter.observation().selected_side_probes, 2);
    assert_eq!(adapter.observation().materialization_entries, 1);
    let runtime = super::m1_loop_activity_observation_for_test();
    assert_eq!(runtime.complete_evaluator_entries, 84);
    assert_eq!(runtime.hydraulic_entries, 4);
    assert_eq!(runtime.actual_hydraulic_entries, 10);
    assert_eq!(runtime.materialization_entries, 1);
    assert_eq!(super::m1_work_observation_for_test(), (2, 0));
}

#[cfg(feature = "m1-trust-region-physical-stage")]
#[test]
fn m1_trust_region_physical_adapter_hydraulic_owner_poison_keeps_typed_cause() {
    let radiation: Value = serde_json::from_str(RADIATION).expect("radiation");
    let original: Value = serde_json::from_str(ORIGINAL).expect("original");
    let input = radiation_input(&radiation, &original);
    let protected_input = input.clone();
    let protected_trial = trial();
    super::m1_reset_accepted_capture_for_test();
    super::m1_reset_loop_activity_observation_for_test();
    super::m1_reset_work_observation_for_test();
    let mut adapter = super::M1PhysicalTrustRegionAdapter::new(input, protected_trial.clone());
    let error =
        super::m1_trust_region_physical_adapter_hydraulic_height_poison_for_test(&mut adapter)
            .expect_err("actual full-supply hydraulic owner must reject a nonfinite height");
    assert!(matches!(
        error,
        super::M1PhysicalTrustRegionAdapterOwnError::Coupled(error)
            if error.code() == "VEG-E-141"
    ));
    assert_eq!(adapter.input, protected_input);
    assert_eq!(adapter.initial_trial, protected_trial);
    assert!(super::m1_captured_accepted_adapter_binding_for_test().is_none());
    assert_eq!(adapter.observation().current_evaluations, 1);
    assert_eq!(adapter.observation().raw_jacobian_assemblies, 1);
    assert_eq!(adapter.observation().materialization_entries, 0);
    assert_eq!(
        adapter.observation().current_hydraulics,
        super::m1_trust_region_controller_interface::M1TrustRegionCurrentHydraulics::Pending
    );
    let runtime = super::m1_loop_activity_observation_for_test();
    assert_eq!(runtime.complete_evaluator_entries, 41);
    assert_eq!(runtime.hydraulic_entries, 1);
    assert_eq!(runtime.actual_hydraulic_entries, 1);
    assert_eq!(runtime.materialization_entries, 0);
    assert_eq!(super::m1_work_observation_for_test(), (1, 0));
}

#[cfg(feature = "m1-trust-region-physical-stage")]
#[test]
fn m1_trust_region_physical_adapter_phase_disagreement_is_not_generic_incomplete() {
    let radiation: Value = serde_json::from_str(RADIATION).expect("radiation");
    let original: Value = serde_json::from_str(ORIGINAL).expect("original");
    let input = radiation_input(&radiation, &original);
    let protected_input = input.clone();
    let protected_trial = trial();
    super::m1_reset_loop_activity_observation_for_test();
    super::m1_reset_work_observation_for_test();
    let mut adapter = super::M1PhysicalTrustRegionAdapter::new(input, protected_trial.clone());
    let error = super::m1_trust_region_physical_adapter_phase_disagreement_for_test(&mut adapter)
        .expect_err("distinct owner-derived phase selections require a typed controller cause");
    let error = match error {
        super::M1PhysicalTrustRegionAdapterOwnError::Coupled(error) => error,
        super::M1PhysicalTrustRegionAdapterOwnError::IncompleteScope => {
            panic!("phase disagreement must not collapse to incomplete scope")
        }
    };
    assert_eq!(error.code(), "LSEB-E-034");
    let M1CoupledError::Numerical(numerical) = *error else {
        panic!("phase disagreement must retain a numerical cause");
    };
    assert_eq!(
        numerical.kind,
        super::numerics::NumericalFailureKind::PhaseActiveSetInconsistent
    );
    let observation = adapter.observation();
    let current = observation
        .current_coordinates
        .expect("actual tied current coordinates");
    assert_eq!(
        numerical
            .failed_solution
            .iter()
            .map(|value| value.to_bits())
            .collect::<Vec<_>>(),
        current.map(f64::to_bits).to_vec()
    );
    let raw = observation.raw_residual.expect("actual tied raw residual");
    let tolerances = observation
        .dynamic_normalizers
        .expect("actual tied dynamic normalizers");
    assert_eq!(numerical.ordered_residuals.len(), 21);
    for (index, row) in numerical.ordered_residuals.iter().enumerate() {
        assert_eq!(row.identity, format!("m1_row_{index}"));
        assert_eq!(row.raw.to_bits(), raw[index].to_bits());
        assert_eq!(row.tolerance.to_bits(), tolerances[index].to_bits());
        assert_eq!(
            row.normalized.to_bits(),
            (raw[index] / tolerances[index]).to_bits()
        );
    }
    assert_eq!(adapter.input, protected_input);
    assert_eq!(adapter.initial_trial, protected_trial);
    assert_eq!(observation.current_evaluations, 1);
    assert_eq!(observation.raw_jacobian_assemblies, 1);
    assert_eq!(observation.materialization_entries, 0);
    let runtime = super::m1_loop_activity_observation_for_test();
    assert_eq!(runtime.complete_evaluator_entries, 41);
    assert_eq!(runtime.hydraulic_entries, 0);
    assert_eq!(runtime.actual_hydraulic_entries, 0);
    assert_eq!(runtime.materialization_entries, 0);
}

#[cfg(feature = "m1-trust-region-physical-stage")]
#[test]
fn m1_trust_region_physical_adapter_ordinary_perturbation_refuses_face_pivot_before_materialization()
 {
    let original: Value = serde_json::from_str(ORIGINAL).expect("original input");
    let input = M1CoupledColumnInput::from_covered_column(
        covered_input(&original),
        [
            M1PhaseReservoirInput::new("stratum-z-upper::forest", 0.018, 0.0),
            M1PhaseReservoirInput::new("stratum-a-lower::forest", 0.018, 0.0),
        ],
        conditions(),
        topology_binding(),
    );
    let setup = solve_m1_coupled_column(&input).expect("canonical non-target original60 setup");
    let mut coordinates = *setup.accepted_coordinates();
    coordinates[20] += 1.0e-4;
    let protected_input = input.clone();
    let protected_trial = M1CoupledColumnTrial::from_coordinates(coordinates);
    super::m1_reset_accepted_capture_for_test();
    super::m1_reset_loop_activity_observation_for_test();
    super::m1_reset_work_observation_for_test();
    let prior = super::M1PhysicalBudgetSnapshot {
        core: 29,
        core_attempted: 29,
        ..Default::default()
    };
    let outer = super::m1_physical_budget_scope_seeded_for_test(prior);
    let mut adapter = super::M1PhysicalTrustRegionAdapter::new(input, protected_trial.clone());
    let refusal = super::m1_trust_region_physical_adapter_run_outcome_for_test(&mut adapter)
        .expect_err("fixed ordinary perturbation must retain its actual Stage-1 refusal");
    assert!(matches!(
        refusal,
        super::m1_trust_region_controller::M1TrustRegionControllerError::Subproblem(
            super::M1TrustRegionSubproblemRefusal {
                kind: super::M1TrustRegionRefusalKind::FacePivotLimit,
                attempted_subproblems: 1,
                completed_jacobi_sweeps: 7,
                completed_face_pivots: 42,
                entered_svd_factorizations: 43,
                ..
            }
        )
    ));
    assert_eq!(adapter.input, protected_input);
    assert_eq!(adapter.initial_trial, protected_trial);
    assert!(super::m1_captured_accepted_adapter_binding_for_test().is_none());
    assert_eq!(adapter.observation().current_evaluations, 2);
    assert_eq!(adapter.observation().raw_jacobian_assemblies, 2);
    assert_eq!(adapter.observation().materialization_entries, 0);
    assert_eq!(
        adapter.observation().current_hydraulics,
        super::m1_trust_region_controller_interface::M1TrustRegionCurrentHydraulics::Pending
    );
    let runtime = super::m1_loop_activity_observation_for_test();
    assert_eq!(runtime.complete_evaluator_entries, 83);
    assert_eq!(runtime.hydraulic_entries, 2);
    assert_eq!(runtime.actual_hydraulic_entries, 4);
    assert_eq!(runtime.materialization_entries, 0);
    assert_eq!(super::m1_work_observation_for_test(), (2, 0));
    let budget = adapter
        .observation()
        .budget_snapshot
        .expect("ordinary refusal must retain its solve-owned budget snapshot");
    assert_eq!(budget.core, 83);
    assert_eq!(budget.raw_jacobian, 2);
    assert_eq!(budget.hydraulic, 4);
    assert_eq!(budget.materialization, 0);
    assert_eq!(budget.core_attempted, budget.core);
    assert_eq!(budget.raw_jacobian_attempted, budget.raw_jacobian);
    assert_eq!(budget.hydraulic_attempted, budget.hydraulic);
    assert_eq!(budget.materialization_attempted, budget.materialization);
    assert!(budget.stage1_factorization > 43);
    assert_eq!(
        budget.stage1_factorization_attempted,
        budget.stage1_factorization
    );
    assert_eq!(
        super::m1_physical_budget_observation_for_test(),
        Some(prior)
    );
    drop(outer);
    assert_eq!(super::m1_physical_budget_observation_for_test(), None);
}

#[cfg(all(
    feature = "m1-trust-region-physical-stage",
    feature = "m1-trust-region-analytic-stage"
))]
#[test]
fn m1_trust_region_physical_adapter_terminal_outcome_retains_analytic_run_kind() {
    let radiation: Value = serde_json::from_str(RADIATION).expect("radiation");
    let original: Value = serde_json::from_str(ORIGINAL).expect("original");
    let mut adapter =
        super::M1PhysicalTrustRegionAdapter::new(radiation_input(&radiation, &original), trial());
    let run =
        super::m1_trust_region_stage2_controls::m1_trust_region_analytic_work_cap_run_for_test();
    let outcome =
        super::m1_trust_region_physical_adapter_terminal_outcome_for_test(&mut adapter, &run)
            .expect("terminal refusal must retain its controller kind");
    assert!(matches!(
        outcome,
        super::M1PhysicalTrustRegionAdapterOutcome::Refused(
            super::M1TrustRegionRefusalKind::WorkCap
        )
    ));
    assert_eq!(adapter.observation().materialization_entries, 0);
}

#[test]
fn m1_frozen_radiation_columns_and_all_four_gas_rows_bind_physical_outputs() {
    let radiation: Value = serde_json::from_str(RADIATION).expect("radiation");
    let original: Value = serde_json::from_str(ORIGINAL).expect("original");
    let base = evaluate_m1_coupled_column(&radiation_input(&radiation, &original), &trial())
        .expect("evaluation");
    assert_eq!(base.residuals().len(), 21);
    assert_eq!(base.jacobian().expect("diagnostic jacobian").len(), 21);
    assert_eq!(base.reservoir_operands().len(), 2);
    for (index, operand) in base.reservoir_operands().iter().enumerate() {
        let expected = &radiation["expected"];
        assert!(
            (operand.wet_temperature_k.expect("nonempty reservoir")
                - decimal(&expected["temperature"][index]))
            .abs()
                <= 1e-9
        );
        assert!(
            (operand.non_vapor_heat_w_m2 - decimal(&expected["q_nonvapor"][index])).abs() <= 1e-6
        );
        assert!((operand.vapor_kg_m2_s - decimal(&expected["e"][index])).abs() <= 1e-12);
        assert!((operand.wet_fraction - decimal(&expected["wet_fraction"][index])).abs() <= 1e-12);
        assert!((operand.emission_w_m2 - decimal(&expected["emission"][index])).abs() <= 1e-9);
        assert!(
            (operand.wet_longwave_w_m2 - decimal(&expected["wet_longwave"][index])).abs() <= 1e-9
        );
        assert!(
            (operand.wet_sensible_w_m2 - decimal(&expected["wet_sensible"][index])).abs() <= 1e-9
        );
        assert!(
            (base.residuals()[6 * index + 3] - decimal(&expected["residual"][2 * index])).abs()
                <= 1e-9
        );
        assert!(
            (base.residuals()[6 * index + 4] - decimal(&expected["residual"][2 * index + 1])).abs()
                <= 1e-6
        );
    }
    for column in radiation["columns"].as_array().expect("four columns") {
        let index = match column["coordinate"].as_str().expect("coordinate") {
            "upper_M" => 3,
            "upper_H" => 4,
            "lower_M" => 9,
            "lower_H" => 10,
            _ => unreachable!(),
        };
        for (actual, expected) in [3, 4, 9, 10]
            .iter()
            .zip(column["residual"].as_array().expect("rows"))
        {
            let actual = base.jacobian().expect("diagnostic jacobian")[*actual][index];
            assert!((actual - decimal(expected)).abs() <= 1e-6 * decimal(expected).abs().max(1.));
        }
    }
    // Independent central probes bind each reservoir M/H sensitivity of the
    // exposed ordinary non-vapor heat operands, rather than a fixture-only
    // derivative API.
    for column in radiation["columns"].as_array().expect("four columns") {
        let coordinate = match column["coordinate"].as_str().expect("coordinate") {
            "upper_M" => 3,
            "upper_H" => 4,
            "lower_M" => 9,
            "lower_H" => 10,
            _ => unreachable!(),
        };
        let step = decimal(&column["step"]);
        let high = evaluate_m1_coupled_column(
            &radiation_input(&radiation, &original),
            &trial()
                .with_coordinate_offset(coordinate, step)
                .expect("valid coordinate offset"),
        )
        .expect("high heat probe");
        let low = evaluate_m1_coupled_column(
            &radiation_input(&radiation, &original),
            &trial()
                .with_coordinate_offset(coordinate, -step)
                .expect("valid coordinate offset"),
        )
        .expect("low heat probe");
        for index in 0..2 {
            let fd = (high.reservoir_operands()[index].non_vapor_heat_w_m2
                - low.reservoir_operands()[index].non_vapor_heat_w_m2)
                / (2.0 * step);
            assert!((fd - decimal(&column["q_nonvapor"][index])).abs() <= 1e-6 * fd.abs().max(1.0));
        }
    }
    let gas: Value = serde_json::from_str(GAS).expect("gas");
    assert_eq!(gas["rows"].as_array().expect("four gas controls").len(), 4);
    for row in gas["rows"].as_array().expect("rows") {
        let (gas_input, gas_trial) = gas_probe_input(&gas, row, &original);
        let gas_evaluation =
            evaluate_m1_coupled_column(&gas_input, &gas_trial).expect("frozen gas row");
        let actual = gas_evaluation
            .occupancy(row["occupancy_id"].as_str().expect("occupancy"))
            .expect("occupancy");
        let sun = &row["leaves"][0];
        let shade = &row["leaves"][1];
        assert_eq!(
            actual.sun.gas_branch,
            sun["gas_branch"].as_str().expect("sun branch")
        );
        assert_eq!(
            actual.shade.gas_branch,
            shade["gas_branch"].as_str().expect("shade branch")
        );
        assert_eq!(actual.sun.beta, decimal(&sun["beta"]));
        assert_eq!(actual.shade.beta, decimal(&shade["beta"]));
        assert_gas_reference(
            actual.shade.internal_specific_humidity_kg_kg,
            &row["internal_liquid_qsat"],
            &gas,
            "specific_humidity",
        );
        assert_gas_reference(
            actual.shade.dark_respiration,
            &shade["rd"],
            &gas,
            "respiration_assimilation",
        );
        assert_gas_reference(
            actual.shade.net_assimilation,
            &shade["an"],
            &gas,
            "respiration_assimilation",
        );
        assert_gas_reference(
            actual.shade.gross_assimilation,
            &shade["ag"],
            &gas,
            "respiration_assimilation",
        );
        assert_gas_reference(
            actual.shade.conductance_m_s,
            &shade["gs_m_s"],
            &gas,
            "conductance_m_s",
        );
        assert_gas_reference(actual.shade.ci_pa, &shade["ci_pa"], &gas, "ci_pa");
        assert_gas_reference(
            actual.shade.vapor_kg_m2_tile_s,
            &shade["vapor_kg_m2_tile_s"],
            &gas,
            "water_flux_kg_m2_tile_s",
        );
        assert_eq!(actual.sun.vapor_kg_m2_tile_s, 0.0);
        assert_eq!(actual.sun.net_assimilation, 0.0);
        assert_gas_reference(
            actual.hydraulics.stem_flux_kg_m2_tile_s,
            &row["stem_flux"],
            &gas,
            "water_flux_kg_m2_tile_s",
        );
        assert_gas_reference(
            actual.hydraulics.total_leaf_demand_kg_m2_tile_s,
            &row["total_leaf_demand"],
            &gas,
            "water_flux_kg_m2_tile_s",
        );
        assert_eq!(actual.hydraulics.potentials_mm.len(), 4);
        for (a, e) in actual
            .hydraulics
            .potentials_mm
            .iter()
            .zip(row["potentials_mm"].as_array().expect("potentials"))
        {
            assert_gas_reference(*a, e, &gas, "hydraulic_potential_mm");
        }
        assert_eq!(
            actual.hydraulics.root_layer_ids,
            row["root_layer_ids"]
                .as_array()
                .expect("layer order")
                .iter()
                .map(|value| value.as_str().expect("layer").to_owned())
                .collect::<Vec<_>>()
        );
        assert_eq!(
            actual.hydraulics.root_fluxes_kg_m2_tile_s.len(),
            row["root_fluxes"].as_array().expect("root flows").len()
        );
        for (actual_flow, expected_flow) in actual
            .hydraulics
            .root_fluxes_kg_m2_tile_s
            .iter()
            .zip(row["root_fluxes"].as_array().expect("root flows"))
        {
            assert_gas_reference(*actual_flow, expected_flow, &gas, "water_flux_kg_m2_tile_s");
        }
        independently_reconstruct_hydraulics(&gas_evaluation, row, &original, &gas);
        assert_eq!(actual.hydraulics.continuity_residuals_kg_m2_tile_s.len(), 4);
        for (actual_residual, expected_residual) in actual
            .hydraulics
            .continuity_residuals_kg_m2_tile_s
            .iter()
            .zip(row["continuity_residuals"].as_array().expect("continuity"))
        {
            assert_gas_reference(
                *actual_residual,
                expected_residual,
                &gas,
                "continuity_residual_kg_m2_tile_s",
            );
        }
    }
}

#[cfg(feature = "m1-trust-region-physical-stage")]
#[test]
fn m1_trust_region_physical_adapter_overcapacity_payload_rejects_without_publication() {
    let original: Value = serde_json::from_str(ORIGINAL).expect("original input");
    let input = M1CoupledColumnInput::from_covered_column(
        covered_input(&original),
        [
            M1PhaseReservoirInput::new("stratum-z-upper::forest", 0.018, 0.0),
            M1PhaseReservoirInput::new("stratum-a-lower::forest", 0.018, 0.0),
        ],
        conditions(),
        topology_binding(),
    );
    let setup = solve_m1_coupled_column(&input).expect("original60 accepted materializer input");
    let protected_input = input.clone();
    let protected_trial = M1CoupledColumnTrial::from_coordinates(*setup.accepted_coordinates());
    super::m1_reset_accepted_capture_for_test();
    super::m1_reset_loop_activity_observation_for_test();
    let mut adapter = super::M1PhysicalTrustRegionAdapter::new(input, protected_trial.clone());
    let error =
        super::m1_trust_region_physical_adapter_over_capacity_materialize_for_test(&mut adapter)
            .expect_err("actual retained adapter payload must reject one-ULP overcapacity");
    assert!(matches!(
        error,
        super::M1PhysicalTrustRegionAdapterOwnError::Coupled(error) if error.code() == "VEG-E-142"
    ));
    assert!(!adapter.has_materialized_result_for_test());
    assert_eq!(adapter.observation().materialization_entries, 0);
    assert!(super::m1_captured_accepted_adapter_binding_for_test().is_none());
    assert_eq!(adapter.input, protected_input);
    assert_eq!(adapter.initial_trial, protected_trial);
    let runtime = super::m1_loop_activity_observation_for_test();
    assert_eq!(runtime.materialization_entries, 1);
    assert_eq!(runtime.actual_hydraulic_entries, 2);
}
