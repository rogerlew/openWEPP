use super::m1_trust_region_controller_interface::{
    M1_TRUST_REGION_CORE_ENTRY_CAP, M1TrustRegionControllerEvaluation,
    M1TrustRegionControllerEvaluator, M1TrustRegionSeededEntryGuard,
};
#[derive(Debug)]
pub(crate) enum M1TrustRegionControllerError<E> {
    Own(E),
    Subproblem(super::M1TrustRegionSubproblemRefusal),
    CoreWorkCap,
    Numerical(&'static str),
    IncompleteScope,
}
#[derive(Clone, Copy)]
struct CoreBudget {
    incoming: u16,
    attempted: u16,
    entered: u16,
}
impl CoreBudget {
    fn new(incoming: u16) -> Self {
        Self {
            incoming,
            attempted: 0,
            entered: 0,
        }
    }
    fn enter<E, P, V: M1TrustRegionControllerEvaluator<Error = E, Payload = P>>(
        &mut self,
        v: &mut V,
        x: [f64; 21],
    ) -> Result<M1TrustRegionControllerEvaluation<P>, M1TrustRegionControllerError<E>> {
        self.attempted += 1;
        if self.incoming + self.entered >= M1_TRUST_REGION_CORE_ENTRY_CAP {
            return Err(M1TrustRegionControllerError::CoreWorkCap);
        }
        self.entered += 1;
        v.evaluate(x).map_err(M1TrustRegionControllerError::Own)
    }
}
pub(crate) struct FoundationState<P, Phase> {
    pub(crate) current: M1TrustRegionControllerEvaluation<P>,
    pub(crate) radius: f64,
    budget: CoreBudget,
    raw_jacobian: [[f64; 21]; 21],
    natural_phase: Phase,
    selected_phase: Phase,
    phase_finalized: bool,
    scales: [f64; 21],
    bounds: [[f64; 21]; 2],
    accepted_updates: u8,
    materialization_count: u8,
    preceding: Option<M1TrustRegionControllerEvaluation<P>>,
    witness_eligibility: ReplacementWitnessEligibility,
    proposal_trace:
        Vec<super::m1_trust_region_controller_interface::M1TrustRegionControllerTransition>,
}
#[derive(Clone, Copy, Eq, PartialEq)]
enum ReplacementWitnessEligibility {
    FullProposal,
    ReplacementAvailable,
    ReplacementConsumed,
}
#[cfg(all(test, feature = "m1-trust-region-physical-stage"))]
fn capture_witness_eligibility_for_test(eligibility: ReplacementWitnessEligibility) {
    if !super::m1_face_pivot_capture_enabled_for_test() {
        return;
    }
    let event = match eligibility {
        ReplacementWitnessEligibility::FullProposal => "witness-full-proposal",
        ReplacementWitnessEligibility::ReplacementAvailable => "witness-replacement-available",
        ReplacementWitnessEligibility::ReplacementConsumed => "witness-replacement-consumed",
    };
    super::m1_trust_region_stage1::m1_face_pivot_capture_lifecycle_for_test(event);
}
pub(crate) struct ProposalModel {
    pub(crate) observation: super::M1TrustRegionSubproblemObservation,
    pub(crate) weighted_jacobian: Vec<Vec<f64>>,
    pub(crate) predicted_reduction: f64,
}
struct TransitionBasis<'a> {
    model: &'a ProposalModel,
    radius_before: f64,
    base_before: [f64; 21],
}
struct TransitionOutcome {
    disposition: super::m1_trust_region_controller_interface::M1TrustRegionProposalDisposition,
    actual_reduction: Option<f64>,
    rho: Option<f64>,
    moving_merit: Option<f64>,
    dynamic_max: Option<f64>,
    installed: bool,
    next_radius: f64,
    base_after: [f64; 21],
    entered_materializations: u8,
}
struct TrialMerit {
    actual_reduction: f64,
    rho: f64,
    moving_merit: f64,
    candidate_max: f64,
}
#[derive(Clone, Copy)]
enum GovernedStep {
    Complete {
        pass: bool,
        temperature: f64,
        humidity: f64,
        hydraulic: f64,
        beta: f64,
        ci: [f64; 4],
    },
    Incomplete,
}
fn finite_max(values: impl IntoIterator<Item = f64>) -> Option<f64> {
    values.into_iter().try_fold(0.0_f64, |maximum, value| {
        value.is_finite().then_some(maximum.max(value.abs()))
    })
}
impl<P: Clone, Phase: Clone + PartialEq> FoundationState<P, Phase> {
    pub(crate) fn initialize<
        E,
        V: M1TrustRegionControllerEvaluator<Error = E, Payload = P, Phase = Phase>,
    >(
        evaluator: &mut V,
    ) -> Result<Self, M1TrustRegionControllerError<E>> {
        let mut budget = CoreBudget::new(0);
        let current = budget.enter(evaluator, evaluator.initial_coordinates())?;
        let zero = [0.0; 21];
        let natural_phase = evaluator
            .select_phase(&current, zero)
            .map_err(M1TrustRegionControllerError::Own)?;
        let raw_jacobian = evaluator
            .raw_jacobian(&current, None, zero)
            .map_err(M1TrustRegionControllerError::Own)?;
        Ok(Self {
            current,
            radius: 1.0,
            budget,
            raw_jacobian,
            selected_phase: natural_phase.clone(),
            phase_finalized: false,
            natural_phase,
            scales: evaluator.coordinate_scales(),
            bounds: evaluator.coordinate_bounds(),
            accepted_updates: 0,
            materialization_count: 0,
            preceding: None,
            witness_eligibility: ReplacementWitnessEligibility::FullProposal,
            proposal_trace: Vec::new(),
        })
    }
    pub(crate) fn solve_proposal(
        &self,
    ) -> Result<ProposalModel, M1TrustRegionControllerError<core::convert::Infallible>> {
        let rows: Vec<&[f64]> = self
            .raw_jacobian
            .iter()
            .map(<[f64; 21]>::as_slice)
            .collect();
        let (weighted_residual, weighted_jacobian) =
            super::m1_trust_region_stage1::weighted_system(
                &self.current.raw_residual,
                &rows,
                &self.current.dynamic_normalizers,
                &self.scales,
            )
            .map_err(M1TrustRegionControllerError::Numerical)?;
        let input = super::M1TrustRegionSubproblemInput {
            raw_residual: self.current.raw_residual,
            raw_jacobian: self.raw_jacobian,
            normalizers: self.current.dynamic_normalizers,
            scales: self.scales,
            lower: self.bounds[0],
            upper: self.bounds[1],
            initial_radius: self.radius,
            base: self.current.coordinates,
            test_work: super::M1TrustRegionTestWorkState::default(),
        };
        let observation = super::m1_trust_region_stage1::solve_subproblem(&input)
            .map_err(M1TrustRegionControllerError::Subproblem)?;
        let f2 = super::m1_trust_region_stage1::checked_sum_squares(&weighted_residual)
            .map_err(M1TrustRegionControllerError::Numerical)?;
        let linear = observation.final_residual;
        let linear2 = super::m1_trust_region_stage1::checked_sum_squares(&linear)
            .map_err(M1TrustRegionControllerError::Numerical)?;
        let predicted_reduction =
            super::m1_trust_region_stage1::predicted_reduction_from_squares(f2, linear2);
        if !predicted_reduction.is_finite() {
            return Err(M1TrustRegionControllerError::Numerical(
                "nonfinite predicted reduction",
            ));
        }
        Ok(ProposalModel {
            observation,
            weighted_jacobian,
            predicted_reduction,
        })
    }

    fn weighted_array(model: &ProposalModel) -> [[f64; 21]; 21] {
        std::array::from_fn(|row| std::array::from_fn(|col| model.weighted_jacobian[row][col]))
    }

    fn physical_direction(&self, step: [f64; 21]) -> Result<[f64; 21], &'static str> {
        let direction = std::array::from_fn(|i| self.scales[i] * step[i]);
        if direction.iter().all(|value| value.is_finite()) {
            Ok(direction)
        } else {
            Err("nonfinite physical direction")
        }
    }

    fn dynamic_max(evaluation: &M1TrustRegionControllerEvaluation<P>) -> Result<f64, &'static str> {
        let mut maximum = 0.0_f64;
        for i in 0..21 {
            let raw = evaluation.raw_residual[i];
            let normalizer = evaluation.dynamic_normalizers[i];
            if !raw.is_finite() || !normalizer.is_finite() || normalizer <= 0.0 {
                return Err("invalid physical residual operand");
            }
            let ratio = (raw / normalizer).abs();
            if !ratio.is_finite() {
                return Err("nonfinite normalized residual");
            }
            maximum = maximum.max(ratio);
        }
        Ok(maximum)
    }

    fn governed_step_pass(
        current: &M1TrustRegionControllerEvaluation<P>,
        candidate: &M1TrustRegionControllerEvaluation<P>,
    ) -> Result<GovernedStep, &'static str> {
        for temperature in current
            .wet_temperature_k
            .iter()
            .chain(candidate.wet_temperature_k.iter())
            .flatten()
        {
            if !temperature.is_finite() {
                return Err("nonfinite governed step operand");
            }
        }
        let Some(temperature) = finite_max(
            [0, 1, 2, 6, 7, 8, 12, 14, 15, 16, 17, 18, 19, 20]
                .iter()
                .map(|&i| candidate.coordinates[i] - current.coordinates[i])
                .chain(
                    candidate
                        .wet_temperature_k
                        .iter()
                        .zip(current.wet_temperature_k.iter())
                        .map(|(candidate, current)| match (candidate, current) {
                            (Some(candidate), Some(current)) => candidate - current,
                            _ => 0.0,
                        }),
                ),
        ) else {
            return Err("nonfinite governed step operand");
        };
        let humidity = (candidate.coordinates[13] - current.coordinates[13]).abs();
        let (
            super::m1_trust_region_controller_interface::M1TrustRegionCurrentHydraulics::Complete(
                candidate_hydraulics,
            ),
            super::m1_trust_region_controller_interface::M1TrustRegionCurrentHydraulics::Complete(
                current_hydraulics,
            ),
        ) = (candidate.current_hydraulics, current.current_hydraulics)
        else {
            return Ok(GovernedStep::Incomplete);
        };
        let Some(hydraulic) = finite_max(
            candidate_hydraulics
                .iter()
                .zip(current_hydraulics.iter())
                .map(|(a, b)| a - b),
        ) else {
            return Err("nonfinite governed step operand");
        };
        let Some(beta) = finite_max(
            candidate
                .beta
                .iter()
                .zip(current.beta.iter())
                .map(|(a, b)| a - b),
        ) else {
            return Err("nonfinite governed step operand");
        };
        let ci = std::array::from_fn(|index| candidate.ci_pa[index] - current.ci_pa[index]);
        if !humidity.is_finite() || ci.iter().any(|value| !value.is_finite()) {
            return Err("nonfinite governed step operand");
        }
        if current
            .wet_temperature_k
            .iter()
            .zip(candidate.wet_temperature_k.iter())
            .any(|(current, candidate)| current.is_some() != candidate.is_some())
        {
            return Ok(GovernedStep::Incomplete);
        }
        Ok(GovernedStep::Complete {
            pass: super::solver::m1_governed_step_components_pass_for_trust_region(
                temperature,
                humidity,
                hydraulic,
                beta,
                &ci,
            ),
            temperature,
            humidity,
            hydraulic,
            beta,
            ci,
        })
    }

    fn transition_trace(
        basis: &TransitionBasis<'_>,
        outcome: &TransitionOutcome,
    ) -> super::m1_trust_region_controller_interface::M1TrustRegionControllerTransition {
        super::m1_trust_region_controller_interface::M1TrustRegionControllerTransition {
            scaled_step: basis.model.observation.scaled_step,
            lambda: basis.model.observation.lambda,
            predicted_reduction: basis.model.predicted_reduction,
            actual_reduction: outcome.actual_reduction,
            rho: outcome.rho,
            installed: outcome.installed,
            next_radius: outcome.next_radius,
            weighted_jacobian: Self::weighted_array(basis.model),
            disposition: outcome.disposition,
            radius_before: basis.radius_before,
            base_before: basis.base_before,
            stage1_observation: basis.model.observation.clone(),
            moving_merit: outcome.moving_merit,
            dynamic_max_normalized_residual: outcome.dynamic_max,
            base_after: outcome.base_after,
            entered_materializations: outcome.entered_materializations,
        }
    }

    fn frozen_trial_merit<E>(
        &self,
        model: &ProposalModel,
        candidate: &M1TrustRegionControllerEvaluation<P>,
    ) -> Result<TrialMerit, M1TrustRegionControllerError<E>> {
        let rows: Vec<&[f64]> = self
            .raw_jacobian
            .iter()
            .map(<[f64; 21]>::as_slice)
            .collect();
        let (_, actual_reduction, rho, moving_merit) =
            super::m1_trust_region_stage1::frozen_weighted_merit(
                &self.current.raw_residual,
                &rows,
                &self.current.dynamic_normalizers,
                &self.scales,
                &model.observation.scaled_step,
                &candidate.raw_residual,
                &candidate.dynamic_normalizers,
            )
            .map_err(M1TrustRegionControllerError::Numerical)?;
        Ok(TrialMerit {
            actual_reduction,
            rho,
            moving_merit,
            candidate_max: 0.0,
        })
    }

    fn materialize_current<E, V>(
        &mut self,
        evaluator: &mut V,
    ) -> Result<(), M1TrustRegionControllerError<E>>
    where
        V: M1TrustRegionControllerEvaluator<Error = E, Payload = P, Phase = Phase>,
    {
        self.materialization_count += 1;
        let payload = evaluator
            .materialize(&self.current)
            .map_err(M1TrustRegionControllerError::Own)?;
        self.current.payload = payload;
        Ok(())
    }

    fn install_candidate<E>(
        &mut self,
        basis: &TransitionBasis<'_>,
        candidate: M1TrustRegionControllerEvaluation<P>,
        merit: &TrialMerit,
        current_max: f64,
    ) -> Result<
        super::m1_trust_region_controller_interface::M1TrustRegionControllerTransition,
        M1TrustRegionControllerError<E>,
    > {
        use super::m1_trust_region_controller_interface::M1TrustRegionProposalDisposition;
        #[cfg(all(test, feature = "m1-trust-region-physical-stage"))]
        let recorder_current = if super::m1_face_pivot_capture_enabled_for_test() {
            Some((
                self.current.raw_residual,
                self.current.dynamic_normalizers,
                current_max,
            ))
        } else {
            None
        };
        self.preceding = Some(self.current.clone());
        self.current = candidate;
        self.accepted_updates += 1;
        self.witness_eligibility = ReplacementWitnessEligibility::FullProposal;
        #[cfg(all(test, feature = "m1-trust-region-physical-stage"))]
        super::m1_trust_region_stage1::m1_face_pivot_capture_lifecycle_for_test(
            "installed-witness-full-proposal",
        );
        if merit.rho < 0.25 {
            self.radius *= 0.25;
        } else if merit.rho > 0.75
            && super::m1_trust_region_stage1::checked_sum_squares(
                &basis.model.observation.scaled_step,
            )
            .map_err(M1TrustRegionControllerError::Numerical)?
            .sqrt()
                >= 0.99 * basis.radius_before
        {
            self.radius = (2.0 * self.radius).min(8.0);
        }
        #[cfg(all(test, feature = "m1-trust-region-physical-stage"))]
        if let Some(recorder_current) = recorder_current {
            super::m1_trust_region_stage1::m1_face_pivot_capture_accepted_install_for_test(
                super::m1_trust_region_stage1::M1FacePivotAcceptedInstall {
                    base_before: basis.base_before,
                    raw_before: recorder_current.0,
                    normalizers_before: recorder_current.1,
                    base_after: self.current.coordinates,
                    raw_after: self.current.raw_residual,
                    normalizers_after: self.current.dynamic_normalizers,
                    scaled_step: basis.model.observation.scaled_step,
                    rho: merit.rho,
                    predicted_reduction: basis.model.predicted_reduction,
                    actual_reduction: merit.actual_reduction,
                    dynamic_max_before: recorder_current.2,
                    dynamic_max_after: merit.candidate_max,
                    radius_before: basis.radius_before,
                    radius_after: self.radius,
                },
            );
        }
        Ok(Self::transition_trace(
            basis,
            &TransitionOutcome {
                disposition: M1TrustRegionProposalDisposition::AcceptedUpdate,
                actual_reduction: Some(merit.actual_reduction),
                rho: Some(merit.rho),
                moving_merit: Some(merit.moving_merit),
                dynamic_max: Some(merit.candidate_max),
                installed: true,
                next_radius: self.radius,
                base_after: self.current.coordinates,
                entered_materializations: 0,
            },
        ))
    }

    fn shrink_transition(
        &mut self,
        basis: &TransitionBasis<'_>,
        mut outcome: TransitionOutcome,
    ) -> super::m1_trust_region_controller_interface::M1TrustRegionControllerTransition {
        self.radius *= 0.25;
        outcome.next_radius = self.radius;
        Self::transition_trace(basis, &outcome)
    }

    fn trial_domain_refusal(
        &mut self,
        basis: &TransitionBasis<'_>,
    ) -> super::m1_trust_region_controller_interface::M1TrustRegionControllerTransition {
        if self.witness_eligibility == ReplacementWitnessEligibility::FullProposal {
            self.witness_eligibility = ReplacementWitnessEligibility::ReplacementAvailable;
        }
        #[cfg(all(test, feature = "m1-trust-region-physical-stage"))]
        capture_witness_eligibility_for_test(self.witness_eligibility);
        self.shrink_transition(
            basis,
            TransitionOutcome {
                disposition: super::m1_trust_region_controller_interface::M1TrustRegionProposalDisposition::MeritRejectedAndShrunk,
                actual_reduction: None,
                rho: None,
                moving_merit: None,
                dynamic_max: None,
                installed: false,
                next_radius: 0.0,
                base_after: basis.base_before,
                entered_materializations: 0,
            },
        )
    }

    fn no_predicted_reduction_transition(
        &mut self,
        basis: &TransitionBasis<'_>,
    ) -> super::m1_trust_region_controller_interface::M1TrustRegionControllerTransition {
        use super::m1_trust_region_controller_interface::M1TrustRegionProposalDisposition as Disposition;
        self.shrink_transition(
            basis,
            TransitionOutcome {
                disposition: Disposition::NoPredictedReductionAndShrunk,
                actual_reduction: None,
                rho: None,
                moving_merit: None,
                dynamic_max: None,
                installed: false,
                next_radius: 0.0,
                base_after: basis.base_before,
                entered_materializations: 0,
            },
        )
    }

    fn root_admit<E, V>(
        &mut self,
        evaluator: &mut V,
        basis: &TransitionBasis<'_>,
        merit: Option<&TrialMerit>,
        candidate_max: f64,
    ) -> Result<
        super::m1_trust_region_controller_interface::M1TrustRegionControllerTransition,
        M1TrustRegionControllerError<E>,
    >
    where
        V: M1TrustRegionControllerEvaluator<Error = E, Payload = P, Phase = Phase>,
    {
        use super::m1_trust_region_controller_interface::M1TrustRegionProposalDisposition as Disposition;
        self.materialize_current(evaluator)?;
        Ok(Self::transition_trace(
            basis,
            &TransitionOutcome {
                disposition: Disposition::RootAdmitted,
                actual_reduction: merit.map(|value| value.actual_reduction),
                rho: merit.map(|value| value.rho),
                moving_merit: merit.map(|value| value.moving_merit),
                dynamic_max: Some(candidate_max),
                installed: false,
                next_radius: self.radius,
                base_after: basis.base_before,
                entered_materializations: 1,
            },
        ))
    }

    fn complete_pending_current_hydraulics<E, V>(
        &mut self,
        evaluator: &mut V,
    ) -> Result<(), M1TrustRegionControllerError<E>>
    where
        V: M1TrustRegionControllerEvaluator<Error = E, Payload = P, Phase = Phase>,
    {
        evaluator
            .complete_current_hydraulics(&mut self.current)
            .map_err(M1TrustRegionControllerError::Own)?;
        matches!(
            self.current.current_hydraulics,
            super::m1_trust_region_controller_interface::M1TrustRegionCurrentHydraulics::Complete(
                _
            )
        )
        .then_some(())
        .ok_or(M1TrustRegionControllerError::IncompleteScope)
    }

    fn transition<E, V>(
        &mut self,
        evaluator: &mut V,
    ) -> Result<
        super::m1_trust_region_controller_interface::M1TrustRegionControllerTransition,
        M1TrustRegionControllerError<E>,
    >
    where
        V: M1TrustRegionControllerEvaluator<Error = E, Payload = P, Phase = Phase>,
    {
        use super::m1_trust_region_controller_interface::M1TrustRegionProposalDisposition as Disposition;
        let model = self.phase_model(evaluator)?;
        let radius_before = self.radius;
        let base_before = self.current.coordinates;
        let basis = TransitionBasis {
            model: &model,
            radius_before,
            base_before,
        };
        let candidate_coordinates = model.observation.candidate_coordinates;
        let current_max =
            Self::dynamic_max(&self.current).map_err(M1TrustRegionControllerError::Numerical)?;
        self.complete_pending_current_hydraulics(evaluator)?;
        if model.predicted_reduction <= 0.0 && current_max > 1.0 {
            return Ok(self.no_predicted_reduction_transition(&basis));
        }
        let trial_domain = evaluator
            .pre_evaluator_trial_domain(candidate_coordinates)
            .map_err(M1TrustRegionControllerError::Own)?;
        if !trial_domain {
            return Ok(self.trial_domain_refusal(&basis));
        }
        let root_witness = match self.witness_eligibility {
            ReplacementWitnessEligibility::FullProposal => true,
            ReplacementWitnessEligibility::ReplacementAvailable => {
                self.witness_eligibility = ReplacementWitnessEligibility::ReplacementConsumed;
                true
            }
            ReplacementWitnessEligibility::ReplacementConsumed => false,
        };
        let candidate = match self.budget.enter(evaluator, candidate_coordinates) {
            Ok(value) => value,
            Err(M1TrustRegionControllerError::Own(error)) => {
                return Err(M1TrustRegionControllerError::Own(error));
            }
            Err(error) => return Err(error),
        };
        let mut merit = if model.predicted_reduction > 0.0 {
            Some(self.frozen_trial_merit(&model, &candidate)?)
        } else {
            None
        };
        let candidate_max =
            Self::dynamic_max(&candidate).map_err(M1TrustRegionControllerError::Numerical)?;
        if !matches!(
            candidate.current_hydraulics,
            super::m1_trust_region_controller_interface::M1TrustRegionCurrentHydraulics::Complete(
                _
            )
        ) {
            return Err(M1TrustRegionControllerError::IncompleteScope);
        }
        let governed = Self::governed_step_pass(&self.current, &candidate)
            .map_err(M1TrustRegionControllerError::Numerical)?;
        if let Some(merit) = merit.as_mut() {
            merit.candidate_max = candidate_max;
        }
        if current_max <= 1.0 && root_witness {
            match governed {
                GovernedStep::Complete { pass: true, .. } => {
                    return self.root_admit(evaluator, &basis, merit.as_ref(), candidate_max);
                }
                GovernedStep::Complete { pass: false, .. } => {
                    if self.witness_eligibility == ReplacementWitnessEligibility::FullProposal {
                        self.witness_eligibility =
                            ReplacementWitnessEligibility::ReplacementAvailable;
                    }
                }
                GovernedStep::Incomplete => {
                    if self.witness_eligibility == ReplacementWitnessEligibility::FullProposal {
                        self.witness_eligibility =
                            ReplacementWitnessEligibility::ReplacementConsumed;
                    }
                }
            }
        }
        let ordinary_update_eligible = current_max > 1.0
            || self.witness_eligibility == ReplacementWitnessEligibility::ReplacementConsumed;
        if matches!(governed, GovernedStep::Complete { .. }) && ordinary_update_eligible {
            if let Some(merit) = merit.as_ref() {
                if merit.rho >= 0.1 && merit.candidate_max < current_max {
                    return self.install_candidate(&basis, candidate, merit, current_max);
                }
            }
        }
        Ok(self.shrink_transition(
            &basis,
            TransitionOutcome {
                disposition: Disposition::MeritRejectedAndShrunk,
                actual_reduction: merit.as_ref().map(|value| value.actual_reduction),
                rho: merit.as_ref().map(|value| value.rho),
                moving_merit: merit.as_ref().map(|value| value.moving_merit),
                dynamic_max: Some(candidate_max),
                installed: false,
                next_radius: 0.0,
                base_after: base_before,
                entered_materializations: 0,
            },
        ))
    }

    fn proposal_model<E>(&self) -> Result<ProposalModel, M1TrustRegionControllerError<E>> {
        match self.solve_proposal() {
            Ok(value) => Ok(value),
            Err(M1TrustRegionControllerError::Subproblem(refusal)) => {
                Err(M1TrustRegionControllerError::Subproblem(refusal))
            }
            Err(M1TrustRegionControllerError::CoreWorkCap) => {
                Err(M1TrustRegionControllerError::CoreWorkCap)
            }
            Err(M1TrustRegionControllerError::Numerical(message)) => {
                Err(M1TrustRegionControllerError::Numerical(message))
            }
            Err(_) => Err(M1TrustRegionControllerError::IncompleteScope),
        }
    }

    fn phase_model<E, V>(
        &mut self,
        evaluator: &mut V,
    ) -> Result<ProposalModel, M1TrustRegionControllerError<E>>
    where
        V: M1TrustRegionControllerEvaluator<Error = E, Payload = P, Phase = Phase>,
    {
        #[cfg(all(test, feature = "m1-trust-region-physical-stage"))]
        capture_witness_eligibility_for_test(self.witness_eligibility);
        let predictor = self.proposal_model()?;
        let predictor_direction = self
            .physical_direction(predictor.observation.scaled_step)
            .map_err(M1TrustRegionControllerError::Numerical)?;
        let phase = evaluator
            .select_phase(&self.current, predictor_direction)
            .map_err(M1TrustRegionControllerError::Own)?;
        if self.phase_finalized {
            if phase != self.selected_phase {
                return Err(M1TrustRegionControllerError::Own(
                    evaluator.phase_active_set_inconsistent(
                        &self.current,
                        &self.selected_phase,
                        &phase,
                    ),
                ));
            }
            evaluator
                .selected_side_probe(&self.current, &self.selected_phase, predictor_direction)
                .map_err(M1TrustRegionControllerError::Own)?;
            return Ok(predictor);
        }
        let reassembled = phase != self.selected_phase;
        let model = if reassembled {
            self.raw_jacobian = evaluator
                .raw_jacobian(&self.current, Some(&phase), predictor_direction)
                .map_err(M1TrustRegionControllerError::Own)?;
            self.selected_phase = phase.clone();
            self.proposal_model()?
        } else {
            predictor
        };
        let direction = self
            .physical_direction(model.observation.scaled_step)
            .map_err(M1TrustRegionControllerError::Numerical)?;
        if reassembled {
            let final_phase = evaluator
                .select_phase(&self.current, direction)
                .map_err(M1TrustRegionControllerError::Own)?;
            if final_phase != self.selected_phase {
                return Err(M1TrustRegionControllerError::Own(
                    evaluator.phase_active_set_inconsistent(
                        &self.current,
                        &self.selected_phase,
                        &final_phase,
                    ),
                ));
            }
        }
        self.phase_finalized = true;
        evaluator
            .selected_side_probe(&self.current, &self.selected_phase, direction)
            .map_err(M1TrustRegionControllerError::Own)?;
        Ok(model)
    }

    fn post_update_admit<E, V>(
        &mut self,
        evaluator: &mut V,
    ) -> Result<bool, M1TrustRegionControllerError<E>>
    where
        V: M1TrustRegionControllerEvaluator<Error = E, Payload = P, Phase = Phase>,
    {
        let Some(previous) = self.preceding.as_ref() else {
            return Ok(false);
        };
        let current_max =
            Self::dynamic_max(&self.current).map_err(M1TrustRegionControllerError::Numerical)?;
        let governed = (current_max <= 1.0)
            .then(|| Self::governed_step_pass(previous, &self.current))
            .transpose()
            .map_err(M1TrustRegionControllerError::Numerical)?;
        let admitted = matches!(governed, Some(GovernedStep::Complete { pass: true, .. }));
        #[cfg(all(test, feature = "m1-trust-region-physical-stage"))]
        if super::m1_face_pivot_capture_enabled_for_test() {
            super::m1_trust_region_stage1::m1_face_pivot_capture_post_update_admission_for_test(
                super::m1_trust_region_stage1::M1FacePivotPostUpdateAdmission {
                    previous_coordinates: previous.coordinates,
                    previous_raw: previous.raw_residual,
                    previous_normalizers: previous.dynamic_normalizers,
                    previous_wet_temperature_k: previous.wet_temperature_k,
                    previous_hydraulics: previous.current_hydraulics,
                    previous_beta: previous.beta,
                    previous_ci_pa: previous.ci_pa,
                    current_coordinates: self.current.coordinates,
                    current_raw: self.current.raw_residual,
                    current_normalizers: self.current.dynamic_normalizers,
                    current_wet_temperature_k: self.current.wet_temperature_k,
                    current_hydraulics: self.current.current_hydraulics,
                    current_beta: self.current.beta,
                    current_ci_pa: self.current.ci_pa,
                    current_max,
                    governed_temperature: match governed {
                        Some(GovernedStep::Complete { temperature, .. }) => Some(temperature),
                        _ => None,
                    },
                    governed_humidity: match governed {
                        Some(GovernedStep::Complete { humidity, .. }) => Some(humidity),
                        _ => None,
                    },
                    governed_hydraulic: match governed {
                        Some(GovernedStep::Complete { hydraulic, .. }) => Some(hydraulic),
                        _ => None,
                    },
                    governed_beta: match governed {
                        Some(GovernedStep::Complete { beta, .. }) => Some(beta),
                        _ => None,
                    },
                    governed_ci: match governed {
                        Some(GovernedStep::Complete { ci, .. }) => Some(ci),
                        _ => None,
                    },
                    governed_pass: match governed {
                        Some(GovernedStep::Complete { pass, .. }) => Some(pass),
                        _ => None,
                    },
                    governed_thresholds:
                        super::solver::m1_governed_step_thresholds_for_trust_region(),
                    admitted,
                },
            );
        }
        self.preceding = None;
        #[cfg(all(test, feature = "m1-trust-region-physical-stage"))]
        if !admitted {
            super::m1_trust_region_stage1::m1_face_pivot_capture_lifecycle_for_test(
                "post-update-not-admitted",
            );
        }
        if admitted {
            self.materialize_current(evaluator)?;
            return Ok(true);
        }
        Ok(false)
    }

    fn reassemble_current<E, V>(
        &mut self,
        evaluator: &mut V,
    ) -> Result<(), M1TrustRegionControllerError<E>>
    where
        V: M1TrustRegionControllerEvaluator<Error = E, Payload = P, Phase = Phase>,
    {
        let zero = [0.0; 21];
        self.natural_phase = evaluator
            .select_phase(&self.current, zero)
            .map_err(M1TrustRegionControllerError::Own)?;
        self.selected_phase = self.natural_phase.clone();
        self.phase_finalized = false;
        self.raw_jacobian = evaluator
            .raw_jacobian(&self.current, None, zero)
            .map_err(M1TrustRegionControllerError::Own)?;
        Ok(())
    }
}

pub(crate) fn m1_trust_region_controller_transition_for_test<E, P, Phase, V>(
    evaluator: &mut V,
) -> Result<
    super::m1_trust_region_controller_interface::M1TrustRegionControllerTransition,
    M1TrustRegionControllerError<E>,
>
where
    V: M1TrustRegionControllerEvaluator<Error = E, Payload = P, Phase = Phase>,
    P: Clone,
    Phase: Clone + PartialEq,
{
    let mut state = FoundationState::initialize(evaluator)?;
    state.transition(evaluator)
}

pub(crate) fn m1_trust_region_controller_run_for_test<E, P, Phase, V>(
    evaluator: &mut V,
) -> Result<
    super::m1_trust_region_controller_interface::M1TrustRegionControllerRun,
    M1TrustRegionControllerError<E>,
>
where
    V: M1TrustRegionControllerEvaluator<Error = E, Payload = P, Phase = Phase>,
    P: Clone,
    Phase: Clone + PartialEq,
{
    let mut state = FoundationState::initialize(evaluator)?;
    let initial_radius = state.radius;
    let mut terminal_refusal = None;
    let mut per_base_proposals = 0_u8;
    loop {
        let proposal = state.transition(evaluator)?;
        per_base_proposals += 1;
        let terminal = proposal.disposition == super::m1_trust_region_controller_interface::M1TrustRegionProposalDisposition::RootAdmitted;
        let accepted = proposal.disposition == super::m1_trust_region_controller_interface::M1TrustRegionProposalDisposition::AcceptedUpdate;
        state.proposal_trace.push(proposal);
        if terminal {
            break;
        }
        if accepted {
            if state.post_update_admit(evaluator)? {
                break;
            }
            if state.accepted_updates >= 50 {
                terminal_refusal = Some(super::M1TrustRegionRefusalKind::WorkCap);
                break;
            }
            evaluator.reset_current_hydraulics_after_nonroot_post_update(&mut state.current);
            #[cfg(all(test, feature = "m1-trust-region-physical-stage"))]
            super::m1_trust_region_stage1::m1_face_pivot_capture_lifecycle_for_test(
                "current-hydraulics-reset",
            );
            state.reassemble_current(evaluator)?;
            #[cfg(all(test, feature = "m1-trust-region-physical-stage"))]
            super::m1_trust_region_stage1::m1_face_pivot_capture_lifecycle_for_test(
                "fresh-base-reassembled",
            );
            per_base_proposals = 0;
            continue;
        }
        if state.radius < 2f64.powi(-20) || per_base_proposals >= 21 {
            terminal_refusal = Some(super::M1TrustRegionRefusalKind::RadiusExhausted);
            break;
        }
    }
    let entered_svd_factorizations = state
        .proposal_trace
        .iter()
        .map(|p| u16::from(p.stage1_observation.entered_svd_factorizations))
        .sum();
    let refinement_scalar_operations = state
        .proposal_trace
        .iter()
        .map(|p| p.stage1_observation.refinement_scalar_operations)
        .sum();
    let refinement_guard_events = state
        .proposal_trace
        .iter()
        .map(|p| p.stage1_observation.refinement_guard_events)
        .sum();
    let refinement_attempted_entries = state
        .proposal_trace
        .iter()
        .map(|p| p.stage1_observation.refinement_attempted_entries)
        .sum();
    let refinement_denied_entries = state
        .proposal_trace
        .iter()
        .map(|p| p.stage1_observation.refinement_denied_entries)
        .sum();
    let materialized_coordinates = if state.materialization_count == 1 {
        Some(state.current.coordinates)
    } else {
        None
    };
    Ok(
        super::m1_trust_region_controller_interface::M1TrustRegionControllerRun {
            initial_radius,
            proposals: u16::try_from(state.proposal_trace.len())
                .map_err(|_| M1TrustRegionControllerError::IncompleteScope)?,
            entered_svd_factorizations,
            refinement_scalar_operations,
            refinement_guard_events,
            refinement_attempted_entries,
            refinement_denied_entries,
            accepted_updates: state.accepted_updates,
            entered_materializations: state.materialization_count,
            final_coordinates: state.current.coordinates,
            materialized_coordinates,
            proposal_trace: state.proposal_trace,
            terminal_refusal,
        },
    )
}
pub(crate) fn m1_trust_region_seeded_entry_guard_for_test<
    E,
    P,
    V: M1TrustRegionControllerEvaluator<Error = E, Payload = P>,
>(
    v: &mut V,
    incoming: u16,
) -> Result<M1TrustRegionSeededEntryGuard<E>, M1TrustRegionControllerError<E>> {
    let mut b = CoreBudget::new(incoming);
    match b.enter(v, v.initial_coordinates()) {
        Err(M1TrustRegionControllerError::CoreWorkCap) => Ok(M1TrustRegionSeededEntryGuard {
            incoming_core_entries: incoming,
            attempted_delta: b.attempted,
            entered_delta: b.entered,
            refused_operation: super::M1TrustRegionControllerOperation::ProposalCore,
            numeric_refusal: Some(super::M1TrustRegionRefusalKind::WorkCap),
            entered_owner_error: None,
        }),
        Err(M1TrustRegionControllerError::Own(e)) => Ok(M1TrustRegionSeededEntryGuard {
            incoming_core_entries: incoming,
            attempted_delta: b.attempted,
            entered_delta: b.entered,
            refused_operation: super::M1TrustRegionControllerOperation::ProposalCore,
            numeric_refusal: None,
            entered_owner_error: Some(e),
        }),
        Err(e) => Err(e),
        Ok(_) => Err(M1TrustRegionControllerError::IncompleteScope),
    }
}
