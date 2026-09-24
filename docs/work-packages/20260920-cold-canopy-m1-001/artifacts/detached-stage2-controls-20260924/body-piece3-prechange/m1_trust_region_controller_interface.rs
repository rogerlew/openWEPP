//! Private 21-coordinate operand boundary for the deferred Stage 2 controller.

pub(crate) const M1_TRUST_REGION_CONTROLLER_DIMENSION: usize = 21;
pub(crate) const M1_TRUST_REGION_CORE_ENTRY_CAP: u16 = 6_500;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) enum M1TrustRegionProposalDisposition {
    AcceptedUpdate,
    DomainRefusedAndShrunk,
    MeritRejectedAndShrunk,
    NoPredictedReductionAndShrunk,
    TerminalOwnerError,
    RootAdmitted,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) enum M1TrustRegionEvaluatorBoundary {
    OrdinaryPreDomainRefusal,
    TerminalOwningError,
}

#[derive(Clone, Debug)]
pub(crate) struct M1TrustRegionControllerEvaluation<P> {
    pub(super) coordinates: [f64; M1_TRUST_REGION_CONTROLLER_DIMENSION],
    pub(super) raw_residual: [f64; M1_TRUST_REGION_CONTROLLER_DIMENSION],
    pub(super) dynamic_normalizers: [f64; M1_TRUST_REGION_CONTROLLER_DIMENSION],
    // Complete governed-step operands are visible to the generic controller;
    // `payload` remains adapter-owned materialization state.
    pub(super) wet_temperature_k: [f64; 2],
    pub(super) hydraulic_potential: [f64; 4],
    pub(super) beta: [f64; 4],
    pub(super) ci_pa: [f64; 4],
    pub(super) payload: P,
}

/// Supplies inputs and owning failures; the controller alone selects transitions.
pub(crate) trait M1TrustRegionControllerEvaluator {
    type Error;
    type Payload: Clone;
    type Phase: Clone + PartialEq;
    fn initial_coordinates(&self) -> [f64; M1_TRUST_REGION_CONTROLLER_DIMENSION];
    fn coordinate_scales(&self) -> [f64; M1_TRUST_REGION_CONTROLLER_DIMENSION];
    fn coordinate_bounds(&self) -> [[f64; M1_TRUST_REGION_CONTROLLER_DIMENSION]; 2];
    fn evaluate(
        &mut self,
        coordinates: [f64; M1_TRUST_REGION_CONTROLLER_DIMENSION],
    ) -> Result<M1TrustRegionControllerEvaluation<Self::Payload>, Self::Error>;
    /// Classifies an evaluator refusal without granting the controller an outcome.
    fn evaluator_boundary(&self, error: &Self::Error) -> M1TrustRegionEvaluatorBoundary;
    fn pre_evaluator_trial_domain(
        &mut self,
        coordinates: [f64; M1_TRUST_REGION_CONTROLLER_DIMENSION],
    ) -> Result<bool, Self::Error>;
    fn raw_jacobian(
        &mut self,
        current: &M1TrustRegionControllerEvaluation<Self::Payload>,
        selected_phase: Option<&Self::Phase>,
        predictor_direction: [f64; M1_TRUST_REGION_CONTROLLER_DIMENSION],
    ) -> Result<
        [[f64; M1_TRUST_REGION_CONTROLLER_DIMENSION]; M1_TRUST_REGION_CONTROLLER_DIMENSION],
        Self::Error,
    >;
    fn select_phase(
        &mut self,
        current: &M1TrustRegionControllerEvaluation<Self::Payload>,
        physical_direction: [f64; M1_TRUST_REGION_CONTROLLER_DIMENSION],
    ) -> Result<Self::Phase, Self::Error>;
    fn selected_side_probe(
        &mut self,
        current: &M1TrustRegionControllerEvaluation<Self::Payload>,
        phase: &Self::Phase,
        direction: [f64; M1_TRUST_REGION_CONTROLLER_DIMENSION],
    ) -> Result<(), Self::Error>;
    fn materialize(
        &mut self,
        current: &M1TrustRegionControllerEvaluation<Self::Payload>,
    ) -> Result<Self::Payload, Self::Error>;
}

#[derive(Clone, Debug)]
pub(crate) struct M1TrustRegionControllerTransition {
    pub(super) scaled_step: [f64; M1_TRUST_REGION_CONTROLLER_DIMENSION],
    pub(super) lambda: f64,
    pub(super) predicted_reduction: f64,
    pub(super) actual_reduction: Option<f64>,
    pub(super) rho: Option<f64>,
    pub(super) installed: bool,
    pub(super) next_radius: f64,
    pub(super) weighted_jacobian:
        [[f64; M1_TRUST_REGION_CONTROLLER_DIMENSION]; M1_TRUST_REGION_CONTROLLER_DIMENSION],
    pub(super) disposition: M1TrustRegionProposalDisposition,
    pub(super) radius_before: f64,
    pub(super) base_before: [f64; M1_TRUST_REGION_CONTROLLER_DIMENSION],
    pub(super) stage1_observation: super::M1TrustRegionSubproblemObservation,
    pub(super) kkt_gradient: [f64; M1_TRUST_REGION_CONTROLLER_DIMENSION],
    pub(super) kkt_normal_gradient: [f64; M1_TRUST_REGION_CONTROLLER_DIMENSION],
    pub(super) moving_merit: Option<f64>,
    pub(super) dynamic_max_normalized_residual: Option<f64>,
    pub(super) base_after: [f64; M1_TRUST_REGION_CONTROLLER_DIMENSION],
    pub(super) entered_materializations: u8,
}

#[derive(Clone, Debug)]
pub(crate) struct M1TrustRegionControllerRun {
    pub(super) initial_radius: f64,
    pub(super) proposals: u8,
    pub(super) entered_svd_factorizations: u16,
    pub(super) accepted_updates: u8,
    pub(super) entered_materializations: u8,
    pub(super) final_coordinates: [f64; M1_TRUST_REGION_CONTROLLER_DIMENSION],
    pub(super) materialized_coordinates: Option<[f64; M1_TRUST_REGION_CONTROLLER_DIMENSION]>,
    pub(super) proposal_trace: Vec<M1TrustRegionControllerTransition>,
    pub(super) terminal_refusal: Option<super::M1TrustRegionRefusalKind>,
}

#[derive(Clone, Debug)]
pub(crate) struct M1TrustRegionSeededEntryGuard<E> {
    pub(super) incoming_core_entries: u16,
    pub(super) attempted_delta: u16,
    pub(super) entered_delta: u16,
    pub(super) refused_operation: super::M1TrustRegionControllerOperation,
    pub(super) numeric_refusal: Option<super::M1TrustRegionRefusalKind>,
    pub(super) entered_owner_error: Option<E>,
}
