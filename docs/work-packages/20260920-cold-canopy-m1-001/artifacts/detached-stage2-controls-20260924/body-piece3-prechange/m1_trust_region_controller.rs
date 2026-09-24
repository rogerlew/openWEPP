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
pub(crate) struct FoundationState<P> {
    pub(crate) current: M1TrustRegionControllerEvaluation<P>,
    pub(crate) radius: f64,
    budget: CoreBudget,
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
