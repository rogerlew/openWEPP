use super::m1_trust_region_controller_interface::{
    M1_TRUST_REGION_CORE_ENTRY_CAP, M1TrustRegionControllerEvaluation,
    M1TrustRegionControllerEvaluator, M1TrustRegionSeededEntryGuard,
};
#[derive(Debug)]
pub(crate) enum M1TrustRegionControllerError<E> {
    Own(E),
    Subproblem(super::M1TrustRegionRefusalKind),
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
            return Err(M1TrustRegionControllerError::Subproblem(
                super::M1TrustRegionRefusalKind::WorkCap,
            ));
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
    scales: [f64; 21],
    bounds: [[f64; 21]; 2],
    accepted_updates: u8,
    materialization_count: u8,
    proposal_trace:
        Vec<super::m1_trust_region_controller_interface::M1TrustRegionControllerTransition>,
}
pub(crate) struct ProposalModel {
    pub(crate) observation: super::M1TrustRegionSubproblemObservation,
    pub(crate) weighted_residual: Vec<f64>,
    pub(crate) weighted_jacobian: Vec<Vec<f64>>,
    pub(crate) predicted_reduction: f64,
}
impl<P, Phase> FoundationState<P, Phase> {
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
            natural_phase,
            scales: evaluator.coordinate_scales(),
            bounds: evaluator.coordinate_bounds(),
            accepted_updates: 0,
            materialization_count: 0,
            proposal_trace: Vec::new(),
        })
    }
    pub(crate) fn solve_proposal(
        &self,
    ) -> Result<ProposalModel, M1TrustRegionControllerError<core::convert::Infallible>> {
        let rows: Vec<&[f64]> = self.raw_jacobian.iter().map(|row| row.as_slice()).collect();
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
            .map_err(|refusal| M1TrustRegionControllerError::Subproblem(refusal.kind))?;
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
            weighted_residual,
            weighted_jacobian,
            predicted_reduction,
        })
    }
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
        Err(M1TrustRegionControllerError::Subproblem(k)) => Ok(M1TrustRegionSeededEntryGuard {
            incoming_core_entries: incoming,
            attempted_delta: b.attempted,
            entered_delta: b.entered,
            refused_operation: super::M1TrustRegionControllerOperation::ProposalCore,
            numeric_refusal: Some(k),
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
