//! Analytic expected-red controls for the deferred Stage 2 controller body.
use super::m1_trust_region_controller_interface::{
    M1_TRUST_REGION_CONTROLLER_DIMENSION as N, M1TrustRegionControllerEvaluation as Evaluation,
    M1TrustRegionControllerEvaluator as Evaluator,
};
const DERIVED_TOLERANCE: f64 = 1e-12;
#[derive(Clone, Copy)]
enum FixtureKind {
    Ordinary(f64),
    FrozenRawJacobian,
    ConstrainedStationarity,
    FullNoUpdate,
}
#[derive(Clone, Debug, Eq, PartialEq)]
struct Payload {
    materialized_coordinate_bits: [u64; N],
    marker: u64,
}
#[derive(Clone, Debug, PartialEq)]
enum FixtureOperation {
    Evaluate([f64; N]),
    PreEvaluatorTrialDomain([f64; N]),
    RawJacobian {
        base: [f64; N],
        selected_phase: Option<u8>,
        predictor_direction: [f64; N],
    },
    SelectPhase {
        base: [f64; N],
        step: [f64; N],
    },
    SelectedSideProbe {
        base: [f64; N],
        phase: u8,
        direction: [f64; N],
    },
    Materialize([f64; N]),
}
struct Fixture {
    kind: FixtureKind,
    base: [f64; N],
    scale: [f64; N],
    lower: [f64; N],
    upper: [f64; N],
    operation_log: Vec<FixtureOperation>,
    evaluator_entries: u16,
    owner_sentinel: u64,
    materializer_inputs: Vec<[f64; N]>,
    payload_marker: u64,
    injected_evaluate_error: Option<&'static str>,
}
impl Fixture {
    fn new(kind: FixtureKind) -> Self {
        let mut base = [300.0; N];
        let mut scale = [1.0; N];
        let mut lower = [f64::NEG_INFINITY; N];
        let mut upper = [f64::INFINITY; N];
        for i in [0, 1, 2, 6, 7, 8] {
            lower[i] = 263.15;
            upper[i] = 373.15;
        }
        lower[12] = 258.15;
        upper[12] = 373.15;
        lower[13] = 0.0;
        upper[13] = 0.1;
        base[3] = 0.01;
        base[9] = 0.01;
        base[4] = 1000.0;
        base[10] = 1000.0;
        base[5] = 0.0;
        base[11] = 0.0;
        base[13] = 0.01;
        scale[3] = 0.01;
        scale[9] = 0.01;
        scale[4] = 1000.0;
        scale[10] = 1000.0;
        scale[5] = 1e-5;
        scale[11] = 1e-5;
        scale[13] = 0.001;
        lower[5] = 0.0;
        lower[11] = 0.0;
        Self {
            kind,
            base,
            scale,
            lower,
            upper,
            operation_log: Vec::new(),
            evaluator_entries: 0,
            owner_sentinel: 0x4d31_5452_5553_5443,
            materializer_inputs: Vec::new(),
            payload_marker: 0,
            injected_evaluate_error: None,
        }
    }
    fn z(&self, x: &[f64; N], i: usize) -> f64 {
        (x[i] - self.base[i]) / self.scale[i]
    }
    fn normalizer(&self, x: &[f64; N], i: usize) -> f64 {
        match self.kind {
            FixtureKind::FrozenRawJacobian if i == 0 => 1.0 + self.z(x, 0) / 2.0,
            FixtureKind::FrozenRawJacobian if i == 1 => 0.5,
            FixtureKind::FullNoUpdate => 2f64.powi(-60),
            _ => 1.0,
        }
    }
    fn raw_multiplier(&self) -> f64 {
        if matches!(self.kind, FixtureKind::FullNoUpdate) {
            2f64.powi(-60)
        } else {
            1.0
        }
    }
    fn residual(&self, x: &[f64; N], i: usize) -> f64 {
        let z = self.z(x, i);
        match (self.kind, i) {
            (FixtureKind::Ordinary(c), 0) => -2.0 + z + c * z * z,
            (FixtureKind::FrozenRawJacobian, 0) => -1.2 + z,
            (FixtureKind::FrozenRawJacobian, 1) => -1.0 + z,
            (FixtureKind::ConstrainedStationarity, 5) => 2.0 + z,
            (FixtureKind::FullNoUpdate, 0) => z - 2f64.powi(-28),
            _ => z,
        }
    }
    fn record(&mut self, operation: FixtureOperation) {
        self.operation_log.push(operation);
    }
}
impl Evaluator for Fixture {
    type Error = &'static str;
    type Payload = Payload;
    type Phase = ();
    fn initial_coordinates(&self) -> [f64; N] {
        self.base
    }
    fn coordinate_scales(&self) -> [f64; N] {
        self.scale
    }
    fn coordinate_bounds(&self) -> [[f64; N]; 2] {
        [self.lower, self.upper]
    }
    fn evaluate(&mut self, x: [f64; N]) -> Result<Evaluation<Payload>, Self::Error> {
        self.evaluator_entries += 1;
        self.record(FixtureOperation::Evaluate(x));
        if let Some(error) = self.injected_evaluate_error {
            return Err(error);
        }
        for i in 0..N {
            if !x[i].is_finite()
                || x[i] < self.lower[i]
                || x[i] > self.upper[i]
                || !self.normalizer(&x, i).is_finite()
                || self.normalizer(&x, i) <= 0.0
            {
                return Err("analytic fixture domain");
            }
        }
        Ok(Evaluation {
            coordinates: x,
            raw_residual: std::array::from_fn(|i| self.raw_multiplier() * self.residual(&x, i)),
            dynamic_normalizers: std::array::from_fn(|i| self.normalizer(&x, i)),
            wet_temperature_k: [300.0; 2],
            hydraulic_potential: [0.0; 4],
            beta: [0.5; 4],
            ci_pa: [20.0; 4],
            payload: Payload {
                materialized_coordinate_bits: [0; N],
                marker: 0,
            },
        })
    }
    fn evaluator_boundary(
        &self,
        error: &&'static str,
    ) -> super::m1_trust_region_controller_interface::M1TrustRegionEvaluatorBoundary {
        match *error {
            "analytic fixture domain" => super::m1_trust_region_controller_interface::M1TrustRegionEvaluatorBoundary::OrdinaryPreDomainRefusal,
            _ => super::m1_trust_region_controller_interface::M1TrustRegionEvaluatorBoundary::TerminalOwningError,
        }
    }
    fn pre_evaluator_trial_domain(&mut self, coordinates: [f64; N]) -> Result<bool, Self::Error> {
        self.record(FixtureOperation::PreEvaluatorTrialDomain(coordinates));
        Ok(coordinates
            .iter()
            .enumerate()
            .all(|(i, value)| *value >= self.lower[i] && *value <= self.upper[i]))
    }
    fn raw_jacobian(
        &mut self,
        e: &Evaluation<Payload>,
        selected_phase: Option<&()>,
        predictor_direction: [f64; N],
    ) -> Result<[[f64; N]; N], Self::Error> {
        self.record(FixtureOperation::RawJacobian {
            base: e.coordinates,
            selected_phase: selected_phase.map(|_| 0),
            predictor_direction,
        });
        Ok(std::array::from_fn(|i| {
            std::array::from_fn(|j| {
                if i == j {
                    self.raw_multiplier()
                        * match self.kind {
                            FixtureKind::Ordinary(c) if i == 0 => {
                                (1.0 + 2.0 * c * self.z(&e.coordinates, 0)) / self.scale[0]
                            }
                            _ => 1.0 / self.scale[i],
                        }
                } else {
                    0.0
                }
            })
        }))
    }
    fn select_phase(
        &mut self,
        e: &Evaluation<Payload>,
        physical_direction: [f64; N],
    ) -> Result<(), Self::Error> {
        self.record(FixtureOperation::SelectPhase {
            base: e.coordinates,
            step: physical_direction,
        });
        Ok(())
    }
    fn selected_side_probe(
        &mut self,
        current: &Evaluation<Payload>,
        _: &(),
        direction: [f64; N],
    ) -> Result<(), Self::Error> {
        self.record(FixtureOperation::SelectedSideProbe {
            base: current.coordinates,
            phase: 0,
            direction,
        });
        Ok(())
    }
    fn materialize(&mut self, e: &Evaluation<Payload>) -> Result<Payload, Self::Error> {
        self.record(FixtureOperation::Materialize(e.coordinates));
        self.materializer_inputs.push(e.coordinates);
        self.payload_marker += 1;
        Ok(Payload {
            materialized_coordinate_bits: e.coordinates.map(f64::to_bits),
            marker: self.payload_marker,
        })
    }
}
fn close(a: f64, b: f64) {
    assert!((a - b).abs() <= DERIVED_TOLERANCE)
}

fn only_lower_five(mask: [bool; N]) -> bool {
    mask.iter()
        .enumerate()
        .all(|(index, active)| *active == (index == 5))
}

fn stationarity_log(base: [f64; N], proposals: usize) -> Vec<FixtureOperation> {
    let zero = [0.0; N];
    let mut log = vec![
        FixtureOperation::Evaluate(base),
        FixtureOperation::SelectPhase { base, step: zero },
        FixtureOperation::RawJacobian {
            base,
            selected_phase: None,
            predictor_direction: zero,
        },
    ];
    for _ in 0..proposals {
        log.push(FixtureOperation::SelectPhase { base, step: zero });
        log.push(FixtureOperation::SelectedSideProbe {
            base,
            phase: 0,
            direction: zero,
        });
    }
    log
}
#[test]
fn m1_stage2_ordinary_four_radius_cases_use_actual_state() {
    for (c, ared, rho, install, radius) in [
        (0.0, 1.5, 1.0, true, 2.0),
        (-0.5, 0.875, 7.0 / 12.0, true, 1.0),
        (-0.875, 31.0 / 128.0, 31.0 / 192.0, true, 0.25),
        (-1.0, 0.0, 0.0, false, 0.25),
    ] as [(f64, f64, f64, bool, f64); 4]
    {
        let mut f = Fixture::new(FixtureKind::Ordinary(c));
        let base = f.base;
        let t = super::m1_trust_region_controller_transition_for_test(&mut f).unwrap();
        let mut candidate = base;
        candidate[0] += 1.0;
        let physical_direction = std::array::from_fn(|i| f.scale[i] * t.scaled_step[i]);
        close(t.scaled_step[0], 1.0);
        close(t.lambda, 1.0);
        close(t.predicted_reduction, 1.5);
        close(t.actual_reduction.unwrap(), ared);
        close(t.rho.unwrap(), rho);
        assert_eq!(t.installed, install);
        assert_eq!(t.base_before.map(f64::to_bits), base.map(f64::to_bits));
        assert_eq!(t.radius_before.to_bits(), 1.0f64.to_bits());
        assert_eq!(
            t.disposition,
            if install {
                super::m1_trust_region_controller_interface::M1TrustRegionProposalDisposition::AcceptedUpdate
            } else {
                super::m1_trust_region_controller_interface::M1TrustRegionProposalDisposition::MeritRejectedAndShrunk
            }
        );
        assert_eq!(t.next_radius.to_bits(), radius.to_bits());
        assert_eq!(t.entered_materializations, 0);
        assert_eq!(
            t.stage1_observation.base_coordinates.map(f64::to_bits),
            base.map(f64::to_bits)
        );
        assert_eq!(
            t.stage1_observation.candidate_coordinates.map(f64::to_bits),
            candidate.map(f64::to_bits)
        );
        assert_eq!(
            t.stage1_observation.scaled_step.map(f64::to_bits),
            t.scaled_step.map(f64::to_bits)
        );
        assert_eq!(t.stage1_observation.lambda.to_bits(), t.lambda.to_bits());
        assert!(
            t.stage1_observation
                .initial_free_mask
                .iter()
                .all(|free| *free)
        );
        assert!(
            t.stage1_observation
                .active_lower
                .iter()
                .all(|active| !active)
        );
        assert!(
            t.stage1_observation
                .active_upper
                .iter()
                .all(|active| !active)
        );
        assert_eq!(t.stage1_observation.entered_svd_factorizations, 1);
        assert_eq!(
            f.operation_log,
            vec![
                FixtureOperation::Evaluate(base),
                FixtureOperation::SelectPhase {
                    base,
                    step: [0.0; N]
                },
                FixtureOperation::RawJacobian {
                    base,
                    selected_phase: None,
                    predictor_direction: [0.0; N],
                },
                FixtureOperation::SelectPhase {
                    base,
                    step: physical_direction
                },
                FixtureOperation::SelectedSideProbe {
                    base,
                    phase: 0,
                    direction: physical_direction
                },
                FixtureOperation::PreEvaluatorTrialDomain(candidate),
                FixtureOperation::Evaluate(candidate),
            ]
        );
        assert_eq!(f.evaluator_entries, 2);
        assert!(f.materializer_inputs.is_empty());
        assert_eq!(
            t.base_after.map(f64::to_bits),
            if install {
                candidate.map(f64::to_bits)
            } else {
                base.map(f64::to_bits)
            }
        );
    }
}
#[test]
fn m1_stage2_raw_jacobian_and_frozen_merit_are_actual() {
    let mut f = Fixture::new(FixtureKind::FrozenRawJacobian);
    let e = f.evaluate(f.base).unwrap();
    assert_eq!(e.raw_residual[0].to_bits(), (-1.2f64).to_bits());
    assert_eq!(
        f.raw_jacobian(&e, None, [0.0; N]).unwrap()[0][0].to_bits(),
        1.0f64.to_bits()
    );
    f.operation_log.clear();
    f.evaluator_entries = 0;
    let t = super::m1_trust_region_controller_transition_for_test(&mut f).unwrap();
    close(t.scaled_step[0], 0.6);
    close(t.scaled_step[1], 0.8);
    close(t.weighted_jacobian[0][0], 1.0);
    close(t.weighted_jacobian[1][1], 2.0);
    close(t.predicted_reduction, 123.0 / 50.0);
    close(t.actual_reduction.unwrap(), 123.0 / 50.0);
    close(t.rho.unwrap(), 1.0);
    close(t.moving_merit.unwrap(), 788.0 / 4225.0);
    close(t.dynamic_max_normalized_residual.unwrap(), 6.0 / 13.0);
    assert!(t.installed);
    assert_eq!(t.next_radius.to_bits(), 2.0f64.to_bits());
    let mut candidate = f.base;
    candidate[0] += 0.6;
    candidate[1] += 0.8;
    let physical_direction = std::array::from_fn(|i| f.scale[i] * t.scaled_step[i]);
    assert_eq!(f.evaluator_entries, 2);
    assert_eq!(
        f.operation_log,
        vec![
            FixtureOperation::Evaluate(f.base),
            FixtureOperation::SelectPhase {
                base: f.base,
                step: [0.0; N]
            },
            FixtureOperation::RawJacobian {
                base: f.base,
                selected_phase: None,
                predictor_direction: [0.0; N],
            },
            FixtureOperation::SelectPhase {
                base: f.base,
                step: physical_direction
            },
            FixtureOperation::SelectedSideProbe {
                base: f.base,
                phase: 0,
                direction: physical_direction
            },
            FixtureOperation::PreEvaluatorTrialDomain(candidate),
            FixtureOperation::Evaluate(candidate),
        ]
    );
    close(t.stage1_observation.scaled_step[0], 0.6);
    close(t.stage1_observation.scaled_step[1], 0.8);
    assert!(
        t.stage1_observation.scaled_step[2..]
            .iter()
            .all(|value| value.abs() <= DERIVED_TOLERANCE)
    );
    assert_eq!(
        t.stage1_observation.scaled_step.map(f64::to_bits),
        t.scaled_step.map(f64::to_bits)
    );
    assert_eq!(t.base_before.map(f64::to_bits), f.base.map(f64::to_bits));
    assert_eq!(t.base_after.map(f64::to_bits), candidate.map(f64::to_bits));
}
#[test]
fn m1_stage2_constrained_stationarity_is_nonterminal_rejection() {
    let mut f = Fixture::new(FixtureKind::ConstrainedStationarity);
    let sentinel = f.owner_sentinel;
    let t = super::m1_trust_region_controller_transition_for_test(&mut f).unwrap();
    assert_eq!(t.scaled_step.map(f64::to_bits), [0.0; N].map(f64::to_bits));
    close(t.weighted_jacobian[5][5], 1.0);
    close(t.stage1_observation.final_g[5], 2.0);
    close(t.stage1_observation.final_h[5], 2.0);
    close(t.predicted_reduction, 0.0);
    assert_eq!(t.actual_reduction, None);
    assert_eq!(t.rho, None);
    assert_eq!(t.moving_merit, None);
    assert_eq!(t.dynamic_max_normalized_residual, None);
    assert_eq!(t.disposition, super::m1_trust_region_controller_interface::M1TrustRegionProposalDisposition::NoPredictedReductionAndShrunk);
    assert_eq!(t.radius_before.to_bits(), 1.0f64.to_bits());
    assert_eq!(t.base_before.map(f64::to_bits), f.base.map(f64::to_bits));
    assert_eq!(t.base_after.map(f64::to_bits), f.base.map(f64::to_bits));
    assert_eq!(t.next_radius.to_bits(), 0.25f64.to_bits());
    assert!(!t.installed);
    assert_eq!(t.entered_materializations, 0);
    assert_eq!(t.stage1_observation.entered_svd_factorizations, 2);
    assert_eq!(t.stage1_observation.completed_face_pivots, 1);
    assert_eq!(t.stage1_observation.event_trace.len(), 2);
    assert!(
        t.stage1_observation
            .initial_lower_mask
            .iter()
            .all(|active| !active)
    );
    assert!(
        t.stage1_observation
            .initial_upper_mask
            .iter()
            .all(|active| !active)
    );
    assert!(
        t.stage1_observation
            .initial_free_mask
            .iter()
            .all(|active| *active)
    );
    assert_eq!(
        t.stage1_observation.event_trace[0].action,
        super::M1TrustRegionKktAction::ActivateLower
    );
    assert_eq!(t.stage1_observation.event_trace[0].selected_coordinate, 5);
    assert!(t.stage1_observation.event_trace[0].lower_mask[5]);
    assert!(only_lower_five(
        t.stage1_observation.event_trace[0].lower_mask
    ));
    assert!(
        t.stage1_observation.event_trace[0]
            .upper_mask
            .iter()
            .all(|active| !active)
    );
    assert!(!t.stage1_observation.event_trace[0].free_mask[5]);
    assert_eq!(
        t.stage1_observation.event_trace[1].action,
        super::M1TrustRegionKktAction::Return
    );
    assert!(only_lower_five(
        t.stage1_observation.event_trace[1].lower_mask
    ));
    assert!(
        t.stage1_observation.event_trace[1]
            .upper_mask
            .iter()
            .all(|active| !active)
    );
    assert_eq!(
        t.stage1_observation.event_trace[1].free_mask,
        std::array::from_fn(|index| index != 5)
    );
    assert!(
        t.stage1_observation.event_trace[1]
            .free_mask
            .iter()
            .enumerate()
            .filter(|(i, _)| *i != 5)
            .all(|(_, free)| *free)
    );
    assert!(only_lower_five(t.stage1_observation.active_lower));
    assert!(
        t.stage1_observation
            .active_upper
            .iter()
            .all(|active| !active)
    );
    assert_eq!(f.operation_log, stationarity_log(f.base, 1));
    assert_eq!(f.evaluator_entries, 1);
    assert_eq!(f.owner_sentinel, sentinel);
}
#[test]
fn m1_stage2_stationarity_exhausts_after_eleven_proposals_and_twenty_two_faces() {
    let mut f = Fixture::new(FixtureKind::ConstrainedStationarity);
    let base = f.base;
    let sentinel = f.owner_sentinel;
    let r = super::m1_trust_region_controller_run_for_test(&mut f).unwrap();
    assert_eq!(
        (
            r.proposals,
            r.entered_svd_factorizations,
            r.accepted_updates,
            r.entered_materializations
        ),
        (11, 22, 0, 0)
    );
    assert_eq!(
        r.terminal_refusal,
        Some(super::M1TrustRegionRefusalKind::RadiusExhausted)
    );
    assert_eq!(
        r.final_coordinates.map(f64::to_bits),
        base.map(f64::to_bits)
    );
    assert!(r.materialized_coordinates.is_none());
    assert_eq!(r.proposal_trace.len(), 11);
    for (j, proposal) in r.proposal_trace.iter().enumerate() {
        assert_eq!(
            proposal.radius_before.to_bits(),
            4f64.powi(-(j as i32)).to_bits()
        );
        assert_eq!(
            proposal.next_radius.to_bits(),
            4f64.powi(-((j + 1) as i32)).to_bits()
        );
        assert_eq!(proposal.disposition, super::m1_trust_region_controller_interface::M1TrustRegionProposalDisposition::NoPredictedReductionAndShrunk);
        assert_eq!(
            proposal.base_before.map(f64::to_bits),
            base.map(f64::to_bits)
        );
        assert_eq!(
            proposal.base_after.map(f64::to_bits),
            base.map(f64::to_bits)
        );
        assert_eq!(proposal.stage1_observation.entered_svd_factorizations, 2);
        assert!(
            proposal
                .stage1_observation
                .initial_lower_mask
                .iter()
                .all(|active| !active)
        );
        assert!(
            proposal
                .stage1_observation
                .initial_upper_mask
                .iter()
                .all(|active| !active)
        );
        assert!(
            proposal
                .stage1_observation
                .initial_free_mask
                .iter()
                .all(|active| *active)
        );
        assert_eq!(proposal.stage1_observation.event_trace.len(), 2);
        assert!(only_lower_five(
            proposal.stage1_observation.event_trace[0].lower_mask
        ));
        assert!(
            proposal.stage1_observation.event_trace[0]
                .upper_mask
                .iter()
                .all(|active| !active)
        );
        assert!(!proposal.stage1_observation.event_trace[0].free_mask[5]);
        assert_eq!(
            proposal.stage1_observation.event_trace[1].action,
            super::M1TrustRegionKktAction::Return
        );
        assert!(only_lower_five(
            proposal.stage1_observation.event_trace[1].lower_mask
        ));
        assert!(
            proposal.stage1_observation.event_trace[1]
                .upper_mask
                .iter()
                .all(|active| !active)
        );
        assert_eq!(
            proposal.stage1_observation.event_trace[1].free_mask,
            std::array::from_fn(|index| index != 5)
        );
        assert!(only_lower_five(proposal.stage1_observation.active_lower));
        assert!(
            proposal
                .stage1_observation
                .active_upper
                .iter()
                .all(|active| !active)
        );
    }
    assert_eq!(
        r.proposal_trace
            .iter()
            .map(|p| p.stage1_observation.entered_svd_factorizations as u16)
            .sum::<u16>(),
        22
    );
    assert_eq!(4f64.powi(-11).to_bits(), 2f64.powi(-22).to_bits());
    assert_eq!(
        f.operation_log
            .iter()
            .filter(|entry| matches!(entry, FixtureOperation::RawJacobian { .. }))
            .count(),
        1
    );
    assert_eq!(
        f.operation_log
            .iter()
            .filter(|entry| matches!(entry, FixtureOperation::Evaluate(_)))
            .count(),
        1
    );
    assert!(
        !f.operation_log
            .iter()
            .any(|entry| matches!(entry, FixtureOperation::Materialize(_)))
    );
    assert_eq!(f.owner_sentinel, sentinel);
    assert_eq!(f.operation_log, stationarity_log(base, 11));
    assert_eq!(f.evaluator_entries, 1);
}
#[test]
fn m1_stage2_full_no_update_materializes_current_base() {
    let mut f = Fixture::new(FixtureKind::FullNoUpdate);
    let base = f.base;
    let e = f.evaluate(base).unwrap();
    assert_eq!(e.raw_residual[0].to_bits(), (-2f64.powi(-88)).to_bits());
    assert_eq!(e.dynamic_normalizers[0].to_bits(), 2f64.powi(-60).to_bits());
    f.operation_log.clear();
    f.evaluator_entries = 0;
    let sentinel = f.owner_sentinel;
    let r = super::m1_trust_region_controller_run_for_test(&mut f).unwrap();
    assert_eq!(r.initial_radius.to_bits(), 1.0f64.to_bits());
    assert_eq!(
        (r.proposals, r.accepted_updates, r.entered_materializations),
        (1, 0, 1)
    );
    assert_eq!(
        r.materialized_coordinates.unwrap().map(f64::to_bits),
        base.map(f64::to_bits)
    );
    assert_eq!(f.materializer_inputs, vec![base]);
    assert_eq!(f.payload_marker, 1);
    assert_eq!(f.owner_sentinel, sentinel);
    let mut expected_candidate = base;
    expected_candidate[0] += 2f64.powi(-28);
    let direction = std::array::from_fn(|i| f.scale[i] * if i == 0 { 2f64.powi(-28) } else { 0.0 });
    assert_eq!(r.proposal_trace.len(), 1);
    let proposal = &r.proposal_trace[0];
    assert_eq!(
        proposal.disposition,
        super::m1_trust_region_controller_interface::M1TrustRegionProposalDisposition::RootAdmitted
    );
    assert!(!proposal.installed);
    assert_eq!(
        proposal.base_before.map(f64::to_bits),
        base.map(f64::to_bits)
    );
    assert_eq!(
        proposal.base_after.map(f64::to_bits),
        base.map(f64::to_bits)
    );
    assert_eq!(
        proposal
            .stage1_observation
            .base_coordinates
            .map(f64::to_bits),
        base.map(f64::to_bits)
    );
    assert_eq!(
        proposal
            .stage1_observation
            .candidate_coordinates
            .map(f64::to_bits),
        expected_candidate.map(f64::to_bits)
    );
    assert_eq!(
        proposal.stage1_observation.scaled_step.map(f64::to_bits),
        proposal.scaled_step.map(f64::to_bits)
    );
    assert_eq!(proposal.scaled_step[0].to_bits(), 2f64.powi(-28).to_bits());
    assert!(
        proposal.scaled_step[1..]
            .iter()
            .all(|value| value.to_bits() == 0)
    );
    assert_eq!(proposal.lambda.to_bits(), 0.0f64.to_bits());
    assert_eq!(proposal.stage1_observation.entered_svd_factorizations, 1);
    assert_eq!(
        proposal.predicted_reduction.to_bits(),
        2f64.powi(-57).to_bits()
    );
    assert_eq!(
        proposal.actual_reduction.unwrap().to_bits(),
        2f64.powi(-57).to_bits()
    );
    assert_eq!(proposal.rho.unwrap().to_bits(), 1.0f64.to_bits());
    assert_eq!(proposal.moving_merit.unwrap().to_bits(), 0.0f64.to_bits());
    assert_eq!(
        proposal.dynamic_max_normalized_residual.unwrap().to_bits(),
        0.0f64.to_bits()
    );
    assert_eq!(f.evaluator_entries, 2);
    assert_eq!(
        f.operation_log,
        vec![
            FixtureOperation::Evaluate(base),
            FixtureOperation::SelectPhase {
                base,
                step: [0.0; N]
            },
            FixtureOperation::RawJacobian {
                base,
                selected_phase: None,
                predictor_direction: [0.0; N]
            },
            FixtureOperation::SelectPhase {
                base,
                step: direction
            },
            FixtureOperation::SelectedSideProbe {
                base,
                phase: 0,
                direction
            },
            FixtureOperation::PreEvaluatorTrialDomain(expected_candidate),
            FixtureOperation::Evaluate(expected_candidate),
            FixtureOperation::Materialize(base)
        ]
    );
    assert_eq!(
        f.operation_log
            .iter()
            .filter(|entry| matches!(entry, FixtureOperation::Evaluate(candidate) if candidate == &expected_candidate))
            .count(),
        1
    );
    assert_eq!(
        f.operation_log
            .iter()
            .filter(
                |entry| matches!(entry, FixtureOperation::Materialize(current) if current == &base)
            )
            .count(),
        1
    );
}
#[test]
fn m1_stage2_seeded_work_guard_records_actual_pre_entry_refusal() {
    let mut f = Fixture::new(FixtureKind::Ordinary(0.0));
    let sentinel = f.owner_sentinel;
    let entries = f.evaluator_entries;
    let log = f.operation_log.clone();
    assert_eq!(
        super::m1_trust_region_controller_interface::M1_TRUST_REGION_CORE_ENTRY_CAP,
        6_500
    );
    let g = super::m1_trust_region_seeded_entry_guard_for_test(
        &mut f,
        super::m1_trust_region_controller_interface::M1_TRUST_REGION_CORE_ENTRY_CAP,
    )
    .unwrap();
    assert_eq!(
        (g.incoming_core_entries, g.attempted_delta, g.entered_delta),
        (
            super::m1_trust_region_controller_interface::M1_TRUST_REGION_CORE_ENTRY_CAP,
            1,
            0
        )
    );
    assert_eq!(
        g.refused_operation,
        super::M1TrustRegionControllerOperation::ProposalCore
    );
    assert_eq!(
        g.numeric_refusal,
        Some(super::M1TrustRegionRefusalKind::WorkCap)
    );
    assert_eq!(g.entered_owner_error, None);
    assert_eq!(f.evaluator_entries, entries);
    assert_eq!(f.operation_log, log);
    assert_eq!(f.owner_sentinel, sentinel);

    let mut entered = Fixture::new(FixtureKind::Ordinary(0.0));
    let entered_sentinel = entered.owner_sentinel;
    entered.injected_evaluate_error = Some("entered owning error");
    let entered_guard = super::m1_trust_region_seeded_entry_guard_for_test(
        &mut entered,
        super::m1_trust_region_controller_interface::M1_TRUST_REGION_CORE_ENTRY_CAP - 1,
    )
    .unwrap();
    assert_eq!(
        (
            entered_guard.incoming_core_entries,
            entered_guard.attempted_delta,
            entered_guard.entered_delta
        ),
        (6_499, 1, 1)
    );
    assert_eq!(
        entered_guard.refused_operation,
        super::M1TrustRegionControllerOperation::ProposalCore
    );
    assert_eq!(entered_guard.numeric_refusal, None);
    assert_eq!(
        entered_guard.entered_owner_error,
        Some("entered owning error")
    );
    assert_eq!(entered.evaluator_entries, 1);
    assert_eq!(
        entered.operation_log,
        vec![FixtureOperation::Evaluate(entered.base)]
    );
    assert_eq!(entered.owner_sentinel, entered_sentinel);
}
