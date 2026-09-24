//! Expected-red controls for the detached Stage 2 trust-region controller.
//!
//! The controller body is intentionally absent. These controls define its one
//! private seam: a supplied model computes raw residuals, raw Jacobians,
//! dynamic normalizers, domain admission, and materialization. The controller
//! may not select an outcome or manufacture any of those values.

use super::{
    m1_trust_region_controller_interface::{
        M1_TRUST_REGION_CONTROLLER_DIMENSION as STAGE2_DIMENSION,
        M1TrustRegionControllerEvaluation, M1TrustRegionControllerEvaluator,
        M1TrustRegionStage1FaceEvent,
    },
    M1TrustRegionControllerFault, M1TrustRegionRefusalKind,
};

#[derive(Clone, Copy, Debug)]
enum Stage2FixtureKind {
    Ordinary { quadratic: f64 },
    FrozenRawJacobian,
    ConstrainedStationarity,
    FullNoUpdate,
}

#[derive(Clone, Copy, Debug)]
struct Stage2AnalyticFixture {
    kind: Stage2FixtureKind,
    base: [f64; STAGE2_DIMENSION],
    scales: [f64; STAGE2_DIMENSION],
    lower: [f64; STAGE2_DIMENSION],
    upper: [f64; STAGE2_DIMENSION],
    initial_radius: f64,
}

impl Stage2AnalyticFixture {
    fn ordinary(quadratic: f64) -> Self {
        Self::new(Stage2FixtureKind::Ordinary { quadratic })
    }

    fn frozen_raw_jacobian() -> Self {
        Self::new(Stage2FixtureKind::FrozenRawJacobian)
    }

    fn constrained_stationarity() -> Self {
        let mut fixture = Self::new(Stage2FixtureKind::ConstrainedStationarity);
        fixture.lower[5] = fixture.base[5];
        fixture
    }

    fn new(kind: Stage2FixtureKind) -> Self {
        let mut base = [300.0; STAGE2_DIMENSION];
        let mut scales = [1.0; STAGE2_DIMENSION];
        let mut lower = [f64::NEG_INFINITY; STAGE2_DIMENSION];
        let mut upper = [f64::INFINITY; STAGE2_DIMENSION];
        for index in [0, 1, 2, 6, 7, 8] {
            lower[index] = 263.15;
            upper[index] = 373.15;
        }
        lower[12] = 258.15;
        upper[12] = 373.15;
        lower[13] = 0.0;
        upper[13] = 0.1;
        base[3] = 0.01;
        base[9] = 0.01;
        base[4] = 1_000.0;
        base[10] = 1_000.0;
        base[5] = 0.0;
        base[11] = 0.0;
        base[13] = 0.01;
        scales[3] = 0.01;
        scales[9] = 0.01;
        scales[4] = 1_000.0;
        scales[10] = 1_000.0;
        scales[5] = 1e-5;
        scales[11] = 1e-5;
        scales[13] = 0.001;
        lower[5] = 0.0;
        lower[11] = 0.0;
        Self {
            kind,
            base,
            scales,
            lower,
            upper,
            initial_radius: match kind {
                Stage2FixtureKind::FullNoUpdate => 2_f64.powi(-60),
                _ => 1.0,
            },
        }
    }

    fn scaled(&self, coordinates: &[f64; STAGE2_DIMENSION], index: usize) -> f64 {
        (coordinates[index] - self.base[index]) / self.scales[index]
    }
}

impl Stage2AnalyticFixture {
    fn validate_coordinates(
        &self,
        coordinates: &[f64; STAGE2_DIMENSION],
    ) -> Result<(), &'static str> {
        for coordinate in 0..STAGE2_DIMENSION {
            if !coordinates[coordinate].is_finite()
                || coordinates[coordinate] < self.lower[coordinate]
                || coordinates[coordinate] > self.upper[coordinate]
            {
                return Err("analytic fixture domain");
            }
        }
        Ok(())
    }
}

impl M1TrustRegionControllerEvaluator for Stage2AnalyticFixture {
    fn initial_coordinates(&self) -> [f64; STAGE2_DIMENSION] {
        self.base
    }

    fn coordinate_scales(&self) -> [f64; STAGE2_DIMENSION] {
        self.scales
    }

    fn coordinate_bounds(&self) -> ([f64; STAGE2_DIMENSION], [f64; STAGE2_DIMENSION]) {
        (self.lower, self.upper)
    }

    fn initial_radius(&self) -> f64 {
        self.initial_radius
    }

    fn evaluate(
        &self,
        coordinates: [f64; STAGE2_DIMENSION],
    ) -> Result<M1TrustRegionControllerEvaluation, &'static str> {
        self.validate_coordinates(&coordinates)?;
        let mut raw_residual = [0.0; STAGE2_DIMENSION];
        let mut dynamic_normalizers = [1.0; STAGE2_DIMENSION];
        for coordinate in 0..STAGE2_DIMENSION {
            raw_residual[coordinate] = self.scaled(&coordinates, coordinate);
        }
        match self.kind {
            Stage2FixtureKind::Ordinary { quadratic } => {
                let z = self.scaled(&coordinates, 0);
                raw_residual[0] = -2.0 + z + quadratic * z * z;
            }
            Stage2FixtureKind::FrozenRawJacobian => {
                raw_residual[0] = -1.2 + self.scaled(&coordinates, 0);
                raw_residual[1] = -1.0 + self.scaled(&coordinates, 1);
                dynamic_normalizers[0] = 1.0 + self.scaled(&coordinates, 0) * 0.5;
                dynamic_normalizers[1] = 0.5;
            }
            Stage2FixtureKind::ConstrainedStationarity => {
                raw_residual[5] = 2.0 + self.scaled(&coordinates, 5);
            }
            Stage2FixtureKind::FullNoUpdate => {
                raw_residual[0] = self.scaled(&coordinates, 0) - 2_f64.powi(-28);
            }
        }
        Ok(M1TrustRegionControllerEvaluation {
            coordinates,
            raw_residual,
            dynamic_normalizers,
            wet_temperature_k: [300.0; 2],
            hydraulic_potential: [0.0; 4],
            beta: [0.5; 4],
            ci_pa: [20.0; 4],
        })
    }

    fn raw_jacobian(
        &self,
        coordinates: [f64; STAGE2_DIMENSION],
    ) -> Result<[[f64; STAGE2_DIMENSION]; STAGE2_DIMENSION], &'static str> {
        self.validate_coordinates(&coordinates)?;
        let mut jacobian = [[0.0; STAGE2_DIMENSION]; STAGE2_DIMENSION];
        for coordinate in 0..STAGE2_DIMENSION {
            jacobian[coordinate][coordinate] = 1.0 / self.scales[coordinate];
        }
        match self.kind {
            Stage2FixtureKind::Ordinary { quadratic } => {
                jacobian[0][0] =
                    (1.0 + 2.0 * quadratic * self.scaled(&coordinates, 0)) / self.scales[0];
            }
            Stage2FixtureKind::FrozenRawJacobian
            | Stage2FixtureKind::ConstrainedStationarity
            | Stage2FixtureKind::FullNoUpdate => {}
        }
        Ok(jacobian)
    }

    fn materialize(
        &self,
        evaluation: M1TrustRegionControllerEvaluation,
    ) -> Result<[f64; STAGE2_DIMENSION], &'static str> {
        Ok(evaluation.coordinates)
    }
}

#[test]
fn m1_stage2_ordinary_radius_transitions_use_actual_polynomial_evaluations() {
    for (quadratic, expected_trial, actual_reduction, rho, installed, next_radius) in [
        (0.0, -1.0, 1.5, 1.0, true, 2.0),
        (-0.5, -1.5, 0.875, 7.0 / 12.0, true, 1.0),
        (-0.875, -1.875, 31.0 / 128.0, 31.0 / 192.0, true, 0.25),
        (-1.0, -2.0, 0.0, 0.0, false, 0.25),
    ] as [(f64, f64, f64, f64, bool, f64); 4] {
        let fixture = Stage2AnalyticFixture::ordinary(quadratic);
        let mut candidate = fixture.base;
        candidate[0] += fixture.scales[0];
        assert_eq!(
            fixture.evaluate(candidate).expect("candidate").raw_residual[0],
            expected_trial
        );
        let transition = super::m1_trust_region_controller_transition_for_test(&fixture, 1.0);
        assert_eq!(transition.scaled_step[0].to_bits(), 1.0_f64.to_bits());
        assert_eq!(transition.lambda.to_bits(), 1.0_f64.to_bits());
        assert!((transition.predicted_reduction - 1.5).abs() < 1e-12);
        assert!((transition.trial_raw_residual[0] - expected_trial).abs() < 1e-12);
        assert!((transition.actual_reduction - actual_reduction).abs() < 1e-12);
        assert!((transition.rho - rho).abs() < 1e-12);
        assert_eq!(transition.installed, installed);
        assert_eq!(transition.next_radius.to_bits(), next_radius.to_bits());
        assert_eq!(transition.entered_materializations, 0);
    }
}

#[test]
fn m1_stage2_frozen_raw_jacobian_fixture_exposes_dynamic_normalizers() {
    let fixture = Stage2AnalyticFixture::frozen_raw_jacobian();
    let jacobian = fixture.raw_jacobian(fixture.base).expect("base Jacobian");
    assert_eq!(jacobian[0][0].to_bits(), 1.0_f64.to_bits());
    assert_eq!(jacobian[1][1].to_bits(), 1.0_f64.to_bits());
    let transition = super::m1_trust_region_controller_transition_for_test(&fixture, 1.0);
    assert_eq!(transition.scaled_step[0].to_bits(), 0.6_f64.to_bits());
    assert_eq!(transition.scaled_step[1].to_bits(), 0.8_f64.to_bits());
    assert_eq!(
        transition.weighted_normal_matrix[0][0].to_bits(),
        1.0_f64.to_bits()
    );
    assert_eq!(
        transition.weighted_normal_matrix[1][1].to_bits(),
        2.0_f64.to_bits()
    );
    assert_eq!(
        transition.predicted_reduction.to_bits(),
        (123.0_f64 / 50.0).to_bits()
    );
    assert_eq!(
        transition.actual_reduction.to_bits(),
        (123.0_f64 / 50.0).to_bits()
    );
    assert_eq!(transition.rho.to_bits(), 1.0_f64.to_bits());
    assert_eq!(
        transition.moving_merit.to_bits(),
        (788.0_f64 / 4225.0).to_bits()
    );
    assert_eq!(
        transition.dynamic_max_normalized_residual.to_bits(),
        (6.0_f64 / 13.0).to_bits()
    );
    assert_eq!(transition.next_radius.to_bits(), 2.0_f64.to_bits());
}

#[test]
fn m1_stage2_constrained_stationarity_uses_the_real_lower_face() {
    let fixture = Stage2AnalyticFixture::constrained_stationarity();
    let base = fixture.evaluate(fixture.base).expect("base evaluation");
    assert_eq!(base.raw_residual[5].to_bits(), 2.0_f64.to_bits());
    let transition = super::m1_trust_region_controller_transition_for_test(&fixture, 1.0);
    assert_eq!(transition.scaled_step[5].to_bits(), 0.0_f64.to_bits());
    assert_eq!(
        transition.weighted_normal_matrix[5][5].to_bits(),
        2.0_f64.to_bits()
    );
    assert_eq!(transition.predicted_reduction.to_bits(), 0.0_f64.to_bits());
    assert_eq!(transition.entered_materializations, 0);
    assert_eq!(
        transition.stage1_face_events,
        vec![
            M1TrustRegionStage1FaceEvent::Free,
            M1TrustRegionStage1FaceEvent::LowerActivated(5),
            M1TrustRegionStage1FaceEvent::FreeColumns(20),
        ]
    );
    assert_eq!(
        transition.refusal,
        Some(M1TrustRegionControllerFault::NoPredictedReductionAtProposal(1))
    );
}

#[test]
fn m1_stage2_stationary_rejection_counts_eleven_proposals_and_twenty_two_faces() {
    let fixture = Stage2AnalyticFixture::constrained_stationarity();
    let run = super::m1_trust_region_controller_run_for_test(&fixture);
    assert_eq!(run.proposals, 11);
    assert_eq!(run.entered_svd_factorizations, 22);
    assert_eq!(
        run.terminal_refusal,
        Some(M1TrustRegionRefusalKind::RadiusExhausted)
    );
    assert_eq!(run.accepted_updates, 0);
    assert_eq!(run.entered_materializations, 0);
    assert_eq!(
        run.final_coordinates.map(f64::to_bits),
        fixture.base.map(f64::to_bits)
    );
}

#[test]
fn m1_stage2_full_no_update_materializes_the_current_evaluation() {
    let fixture = Stage2AnalyticFixture::new(Stage2FixtureKind::FullNoUpdate);
    let run = super::m1_trust_region_controller_run_for_test(&fixture);
    let base = fixture.evaluate(fixture.base).expect("full witness base");
    assert_eq!(base.raw_residual[0].to_bits(), (-2_f64.powi(-28)).to_bits());
    assert_eq!(
        fixture.initial_radius().to_bits(),
        2_f64.powi(-60).to_bits()
    );
    assert_eq!(
        run.final_coordinates.map(f64::to_bits),
        fixture.base.map(f64::to_bits)
    );
    assert_eq!(run.accepted_updates, 0);
    assert_eq!(run.proposals, 1);
    assert_eq!(run.entered_materializations, 1);
}

#[test]
fn m1_stage2_seeded_entry_guards_observe_attempt_before_entry() {
    let fixture = Stage2AnalyticFixture::ordinary(0.0);
    let guard = super::m1_trust_region_seeded_entry_guard_for_test(&fixture, 6_500);
    assert_eq!(guard.attempted_delta, 1);
    assert_eq!(guard.entered_delta, 0);
    assert!(guard.sentinel_preserved);
    assert_eq!(
        guard.fault,
        Some(M1TrustRegionControllerFault::CoreCapBoundary)
    );
}

const ROOT_TOLERANCE: f64 = 1.0e-12;

fn affine_model(
    offset: [f64; 2],
    raw_jacobian: [[f64; 2]; 2],
    base_normalizers: [f64; 2],
    trial_normalizers: [f64; 2],
    lower: [f64; 2],
    upper: [f64; 2],
) -> M1TrustRegionControllerModel {
    M1TrustRegionControllerModel {
        initial_coordinates: [0.0, 0.0],
        raw_residual_offset: offset,
        raw_jacobian,
        base_normalizers,
        trial_normalizers,
        lower,
        upper,
        materialization_tolerance: ROOT_TOLERANCE,
    }
}

fn assert_monotonic_trace(result: &M1TrustRegionControllerResult) {
    let trace = &result.trace;
    assert_eq!(trace.base_entries[0].base_ordinal, 0);
    for pair in trace.base_entries.windows(2) {
        assert_eq!(pair[1].base_ordinal, pair[0].base_ordinal + 1);
        assert!(pair[1].accepted_updates > pair[0].accepted_updates);
        assert!(pair[1].core_entries >= pair[0].core_entries);
        assert!(pair[1].hydraulic_blocks >= pair[0].hydraulic_blocks);
        assert!(pair[1].svd_face_factorizations >= pair[0].svd_face_factorizations);
        assert!(pair[1].subproblem_entries >= pair[0].subproblem_entries);
    }
    for pair in trace.operation_entries.windows(2) {
        assert!(pair[1].core_entries >= pair[0].core_entries);
        assert!(pair[1].hydraulic_blocks >= pair[0].hydraulic_blocks);
        assert!(pair[1].svd_face_factorizations >= pair[0].svd_face_factorizations);
        assert!(pair[1].materialization_entries >= pair[0].materialization_entries);
    }
}

#[test]
fn m1_trust_controller_uses_raw_jacobian_and_frozen_base_weights() {
    // The unweighted/scalar-damped direction is (-1,-1). With the supplied
    // row weights, the actual weighted least-squares direction is (-1,0).
    // Trial normalizers differ deliberately; moving them changes ared/rho.
    let model = affine_model(
        [1.0, 1.0],
        [[1.0, 1.0], [1.0, 0.0]],
        [1.0, 10.0],
        [100.0, 0.1],
        [-4.0, -4.0],
        [4.0, 4.0],
    );
    let result = super::m1_trust_region_controller_run_for_test(&model, None)
        .expect("affine controller result");
    let first = &result.trace.proposals[0];
    assert_eq!(first.raw_residual, [1.0, 1.0]);
    assert_eq!(first.raw_jacobian, [[1.0, 1.0], [1.0, 0.0]]);
    assert_eq!(first.frozen_normalizers, [1.0, 10.0]);
    assert_eq!(first.scaled_step[0].to_bits(), (-1.0_f64).to_bits());
    assert_eq!(first.scaled_step[1].to_bits(), 0.0_f64.to_bits());
    assert_ne!(
        first.frozen_actual_reduction.to_bits(),
        first.moving_weight_actual_reduction.to_bits()
    );
    assert_monotonic_trace(&result);
}

#[test]
fn m1_trust_controller_nonzero_stationarity_never_admits_or_materializes() {
    // This reaches Stage 1 with a zero 2x2 raw J. Rank deficiency must not be
    // relabelled as a stationary root success.
    let model = affine_model(
        [1.0, -2.0],
        [[0.0, 0.0], [0.0, 0.0]],
        [1.0, 1.0],
        [1.0, 1.0],
        [-1.0, -1.0],
        [1.0, 1.0],
    );
    let refusal = super::m1_trust_region_controller_run_for_test(&model, None)
        .expect_err("rank-deficient nonroot is not a root");
    assert_eq!(
        refusal.refusal_kind,
        super::M1TrustRegionRefusalKind::RankDeficient
    );
    assert_eq!(refusal.trace.operation_entries[0].subproblem_entries, 1);
    assert_eq!(
        refusal.trace.operation_entries[0].svd_face_factorizations,
        1
    );
    assert_eq!(
        refusal.trace.operation_entries[0].materialization_entries,
        0
    );
    assert_eq!(refusal.trace.final_coordinates, [0.0, 0.0]);
}

#[test]
fn m1_trust_controller_records_domain_refusal_before_first_valid_replacement() {
    // At Delta=1 the Stage-1 direction is (3,1)/sqrt(10), inside this box.
    // The real trial-domain operation, rather than a box assertion, refuses it.
    let model = affine_model(
        [-3.0, -1.0],
        [[1.0, 0.0], [0.0, 1.0]],
        [1.0, 1.0],
        [1.0, 1.0],
        [0.0, 0.0],
        [4.0, 4.0],
    );
    let result = super::m1_trust_region_controller_run_for_test(
        &model,
        Some(M1TrustRegionControllerFault::TrialDomainAtProposal(1)),
    )
    .expect("later proposal after ordinary trial-domain refusal");
    let first = &result.trace.proposals[0];
    assert_eq!(
        first.disposition,
        M1TrustRegionProposalDisposition::DomainRefusedAndShrunk
    );
    assert_eq!(first.proposal_ordinal, 1);
    assert_eq!(
        first.radius_after.to_bits(),
        (first.radius_before * 0.25).to_bits()
    );
    assert_eq!(
        first.core_entries, 1,
        "domain refusal enters the real trial core"
    );
    assert_eq!(
        first.hydraulic_blocks, 0,
        "domain refusal precedes hydraulics"
    );
    assert_eq!(result.trace.proposals[1].proposal_ordinal, 2);
    assert_monotonic_trace(&result);
}

#[test]
fn m1_trust_controller_terminal_owner_error_does_not_shrink_or_retry() {
    let model = affine_model(
        [-1.0, 0.0],
        [[1.0, 0.0], [0.0, 1.0]],
        [1.0, 1.0],
        [1.0, 1.0],
        [-2.0, -2.0],
        [2.0, 2.0],
    );
    let refusal = super::m1_trust_region_controller_run_for_test(
        &model,
        Some(M1TrustRegionControllerFault::SelectedSideOwnerErrorAtProposal(1)),
    )
    .expect_err("selected-side owner error is terminal");
    let proposal = &refusal.trace.proposals[0];
    assert_eq!(
        proposal.disposition,
        M1TrustRegionProposalDisposition::TerminalOwnerError
    );
    assert_eq!(
        proposal.radius_after.to_bits(),
        proposal.radius_before.to_bits()
    );
    assert_eq!(refusal.trace.proposals.len(), 1);
    assert_eq!(refusal.trace.final_coordinates, [0.0, 0.0]);
    assert_eq!(
        refusal.trace.operation_entries.last().unwrap().core_entries,
        1
    );
}

#[test]
fn m1_trust_controller_no_predicted_reduction_is_nonterminal_and_radius_exhaustion_is_typed() {
    let model = affine_model(
        [-1.0, 0.0],
        [[1.0, 0.0], [0.0, 1.0]],
        [1.0, 1.0],
        [1.0, 1.0],
        [-2.0, -2.0],
        [2.0, 2.0],
    );
    let continued = super::m1_trust_region_controller_run_for_test(
        &model,
        Some(M1TrustRegionControllerFault::NoPredictedReductionAtProposal(1)),
    )
    .expect("NoPredictedReduction shrinks and keeps the base");
    let first = &continued.trace.proposals[0];
    assert_eq!(
        first.disposition,
        M1TrustRegionProposalDisposition::NoPredictedReductionAndShrunk
    );
    assert_eq!(
        first.radius_after.to_bits(),
        (first.radius_before * 0.25).to_bits()
    );
    assert_eq!(continued.trace.proposals[1].proposal_ordinal, 2);
    let exhausted = super::m1_trust_region_controller_run_for_test(
        &model,
        Some(M1TrustRegionControllerFault::RadiusExhaustion),
    )
    .expect_err("minimum radius is a typed refusal, never root admission");
    assert_eq!(
        exhausted.refusal_kind,
        super::M1TrustRegionRefusalKind::RadiusExhausted
    );
    assert_eq!(exhausted.trace.proposals.len(), 21);
    assert_eq!(exhausted.trace.final_coordinates, [0.0, 0.0]);
}

#[test]
fn m1_trust_controller_phase_reassembly_and_drainage_columns_are_actual_operands() {
    // This uses the existing original-input fixture and trial. It must run the
    // same M1 evaluator/materializer adapter that the detached harness will use.
    let (input, trial) = super::m1_coupled_tests::trust_controller_fixture();
    let result = super::m1_trust_region_controller_run_m1_for_test(
        &input,
        &trial,
        M1TrustRegionControllerAdmissionPath::InstalledRoot,
        None,
    )
    .expect("phase-selected controller result");
    let base = &result.trace.base_entries[0];
    assert_eq!(base.natural_assemblies, 1);
    assert_eq!(base.selected_reassemblies, 1);
    assert_eq!(base.predictor_selected_probe_entries, 1);
    assert_eq!(base.final_selected_probe_entries, 1);
    assert_ne!(base.raw_jacobian[0][5].to_bits(), 0);
    assert_ne!(base.raw_jacobian[0][11].to_bits(), 0);
    assert!(base.drainage_scaled_step[0] > -base.drainage_base[0]);
    assert!(base.drainage_scaled_step[1] > -base.drainage_base[1]);
    assert_monotonic_trace(&result);
}

#[test]
fn m1_trust_controller_root_and_no_update_paths_check_all_21_predicates() {
    let (input, trial) = super::m1_coupled_tests::trust_controller_fixture();
    let mut faults = Vec::new();
    for coordinate in 0..21 {
        faults.push(M1TrustRegionControllerFault::RawResidualAboveThreshold(
            coordinate,
        ));
        faults.push(M1TrustRegionControllerFault::NormalizedResidualAboveThreshold(coordinate));
    }
    // The unchanged root/no-update surface has temperature, q, reconstructed
    // hydraulic-potential, and exact-beta governed norms, in that order.
    for governed_norm in 0..4 {
        faults.push(M1TrustRegionControllerFault::GovernedStepAboveThreshold(
            governed_norm,
        ));
    }
    faults.push(M1TrustRegionControllerFault::MaterializerClosureError);
    for path in [
        M1TrustRegionControllerAdmissionPath::InstalledRoot,
        M1TrustRegionControllerAdmissionPath::FullNoUpdate,
        M1TrustRegionControllerAdmissionPath::FirstValidReplacementWitness,
    ] {
        for fault in faults.clone() {
            let refusal = super::m1_trust_region_controller_run_m1_for_test(
                &input,
                &trial,
                path,
                Some(fault),
            )
            .expect_err("poisoned root predicate refuses");
            assert_eq!(refusal.trace.final_coordinates, [267.0, 266.0]);
            assert_eq!(
                refusal
                    .trace
                    .operation_entries
                    .last()
                    .unwrap()
                    .materialization_entries,
                0
            );
            assert_eq!(
                refusal.trace.accepted_owner_bytes,
                refusal.trace.initial_owner_bytes
            );
        }
    }
}

#[test]
fn m1_trust_controller_first_domain_valid_witness_is_not_skipped_after_incomplete_entry() {
    let (input, trial) = super::m1_coupled_tests::trust_controller_fixture();
    let refusal = super::m1_trust_region_controller_run_m1_for_test(
        &input,
        &trial,
        M1TrustRegionControllerAdmissionPath::FirstValidReplacementWitness,
        Some(M1TrustRegionControllerFault::CoreIncompleteAtProposal(1)),
    )
    .expect_err("first domain-valid incomplete witness is terminal");
    assert_eq!(refusal.trace.proposals.len(), 1);
    assert_eq!(refusal.trace.proposals[0].proposal_ordinal, 1);
    assert_eq!(refusal.trace.proposals[0].core_entries, 1);
    assert_eq!(
        refusal
            .trace
            .operation_entries
            .last()
            .unwrap()
            .materialization_entries,
        0
    );
    assert_eq!(refusal.trace.final_coordinates, [267.0, 266.0]);
}

#[test]
fn m1_trust_controller_counts_attempted_work_at_real_boundaries_before_errors() {
    let (input, trial) = super::m1_coupled_tests::trust_controller_fixture();
    for (fault, entered, attempted) in [
        (M1TrustRegionControllerFault::CoreCapBoundary, 6500, 6501),
        (
            M1TrustRegionControllerFault::HydraulicCapBoundary,
            2202,
            2203,
        ),
        (
            M1TrustRegionControllerFault::SvdFaceCapBoundary,
            47300,
            47301,
        ),
        (
            M1TrustRegionControllerFault::MaterializationCapBoundary,
            1,
            2,
        ),
    ] {
        let refusal = super::m1_trust_region_controller_run_m1_for_test(
            &input,
            &trial,
            M1TrustRegionControllerAdmissionPath::InstalledRoot,
            Some(fault),
        )
        .expect_err("entered operation cap refuses before extra entry");
        let work = refusal.trace.operation_entries.last().unwrap();
        match fault {
            M1TrustRegionControllerFault::CoreCapBoundary => {
                assert_eq!(work.core_entries, entered);
                assert_eq!(work.attempted_core_entries, attempted);
            }
            M1TrustRegionControllerFault::HydraulicCapBoundary => {
                assert_eq!(work.hydraulic_blocks, entered);
                assert_eq!(work.attempted_hydraulic_blocks, attempted);
            }
            M1TrustRegionControllerFault::SvdFaceCapBoundary => {
                assert_eq!(work.svd_face_factorizations, entered);
                assert_eq!(work.attempted_svd_face_factorizations, attempted);
            }
            M1TrustRegionControllerFault::MaterializationCapBoundary => {
                assert_eq!(work.materialization_entries, entered);
                assert_eq!(work.attempted_materialization_entries, attempted);
            }
            _ => unreachable!("only cap-boundary faults are supplied"),
        }
        assert!(work.attempted_operation_is_recorded);
        assert_eq!(refusal.trace.final_coordinates, [267.0, 266.0]);
    }
}

#[test]
fn m1_trust_controller_reuses_terminal_current_evaluation_without_report_entry() {
    let (input, trial) = super::m1_coupled_tests::trust_controller_fixture();
    let result = super::m1_trust_region_controller_run_m1_for_test(
        &input,
        &trial,
        M1TrustRegionControllerAdmissionPath::InstalledRoot,
        None,
    )
    .expect("terminal admission trace");
    let terminal_entries = result
        .trace
        .operation_entries
        .iter()
        .filter(|entry| {
            entry.operation == super::M1TrustRegionControllerOperation::TerminalCurrentAdmission
        })
        .count();
    assert_eq!(terminal_entries, 1);
    assert!(result.trace.operation_entries.iter().all(|entry| {
        entry.operation != super::M1TrustRegionControllerOperation::PostUpdateCurrentAdmission
            || entry.core_entries < 6500
    }));
    assert!(
        result
            .trace
            .operation_entries
            .iter()
            .all(|entry| entry.core_entries <= 6500)
    );
}
