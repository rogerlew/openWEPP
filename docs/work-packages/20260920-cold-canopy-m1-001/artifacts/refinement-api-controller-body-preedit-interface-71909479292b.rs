//! Private 21-coordinate operand boundary for the deferred Stage 2 controller.

pub(crate) const M1_TRUST_REGION_CONTROLLER_DIMENSION: usize = 21;
pub(crate) const M1_TRUST_REGION_CORE_ENTRY_CAP: u16 = 6_500;

/// Current-base hydraulics are deferred until the selected tangent and its
/// representable probe have completed. Candidate evaluations are complete.
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub(crate) enum M1TrustRegionCurrentHydraulics {
    #[default]
    Pending,
    Complete([f64; 8]),
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) enum M1TrustRegionProposalDisposition {
    AcceptedUpdate,
    MeritRejectedAndShrunk,
    NoPredictedReductionAndShrunk,
    RootAdmitted,
}

#[derive(Clone, Debug)]
pub(crate) struct M1TrustRegionControllerEvaluation<P> {
    pub(super) coordinates: [f64; M1_TRUST_REGION_CONTROLLER_DIMENSION],
    pub(super) raw_residual: [f64; M1_TRUST_REGION_CONTROLLER_DIMENSION],
    pub(super) dynamic_normalizers: [f64; M1_TRUST_REGION_CONTROLLER_DIMENSION],
    // Complete governed-step operands are visible to the generic controller;
    // `payload` remains adapter-owned materialization state.
    pub(super) wet_temperature_k: [Option<f64>; 2],
    pub(super) current_hydraulics: M1TrustRegionCurrentHydraulics,
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
    /// Completes only a pending current base after selected-tangent work and
    /// before prediction/trial admission. Analytic fixtures are already
    /// complete and leave this unchanged.
    fn complete_current_hydraulics(
        &mut self,
        current: &mut M1TrustRegionControllerEvaluation<Self::Payload>,
    ) -> Result<(), Self::Error>;
    /// Resets an installed non-root base only after post-update admission has
    /// declined it. The next selected tangent must then complete its base
    /// exactly once; complete candidates remain usable by post-update checks.
    fn reset_current_hydraulics_after_nonroot_post_update(
        &mut self,
        current: &mut M1TrustRegionControllerEvaluation<Self::Payload>,
    );
    fn select_phase(
        &mut self,
        current: &M1TrustRegionControllerEvaluation<Self::Payload>,
        physical_direction: [f64; M1_TRUST_REGION_CONTROLLER_DIMENSION],
    ) -> Result<Self::Phase, Self::Error>;
    /// Constructs the adapter-owned failure for a disagreement between the
    /// selected predictor tangent and the final re-solved tangent.
    fn phase_active_set_inconsistent(
        &mut self,
        current: &M1TrustRegionControllerEvaluation<Self::Payload>,
        selected_phase: &Self::Phase,
        final_phase: &Self::Phase,
    ) -> Self::Error;
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
    pub(super) moving_merit: Option<f64>,
    pub(super) dynamic_max_normalized_residual: Option<f64>,
    pub(super) base_after: [f64; M1_TRUST_REGION_CONTROLLER_DIMENSION],
    pub(super) entered_materializations: u8,
}

#[derive(Clone, Debug)]
pub(crate) struct M1TrustRegionControllerRun {
    pub(super) initial_radius: f64,
    pub(crate) proposals: u16,
    pub(super) entered_svd_factorizations: u16,
    pub(crate) refinement_scalar_operations: u64,
    pub(crate) refinement_guard_events: u64,
    pub(crate) refinement_attempted_entries: u16,
    pub(crate) refinement_denied_entries: u16,
    pub(crate) accepted_updates: u8,
    pub(crate) entered_materializations: u8,
    pub(crate) final_coordinates: [f64; M1_TRUST_REGION_CONTROLLER_DIMENSION],
    pub(crate) materialized_coordinates: Option<[f64; M1_TRUST_REGION_CONTROLLER_DIMENSION]>,
    pub(crate) proposal_trace: Vec<M1TrustRegionControllerTransition>,
    pub(crate) terminal_refusal: Option<super::M1TrustRegionRefusalKind>,
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
