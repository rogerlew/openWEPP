#[cfg(test)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) struct AdaptiveControllerTestPolicyV1 {
    pub initial_proposal_quanta: Option<u128>,
    pub growth_factor: u128,
}

#[cfg(test)]
#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub(crate) enum AcceptedPublicationCaptureRegimeV1 {
    Standalone,
    ProvisionalClock,
    FinalClock,
}

#[cfg(test)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) struct AcceptedPublicationCaptureAuditV1 {
    pub support: TimeSupport,
    pub regime: AcceptedPublicationCaptureRegimeV1,
    pub projection_elapsed: std::time::Duration,
    pub initial_seal_elapsed: std::time::Duration,
    pub validation_elapsed: std::time::Duration,
    pub total_elapsed: std::time::Duration,
}

#[derive(Clone, Debug, PartialEq)]
pub struct AdaptiveComparisonAuditV1 {
    pub support: TimeSupport,
    pub maximum_owner_id: Option<String>,
    pub maximum_path: Option<String>,
    pub maximum_tolerance_authority: Option<String>,
    pub maximum_direct_value: Option<f64>,
    pub maximum_composed_value: Option<f64>,
    pub maximum_tolerance_denominator: Option<f64>,
    pub maximum_scaled_error: f64,
    pub first_discrete_surface_kind: Option<String>,
    pub first_discrete_surface_delta: Option<(String, String, String, String)>,
}

#[cfg(test)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub(crate) struct AdaptivePerformanceSpanV1 {
    pub label: &'static str,
    pub support_duration_ns: u128,
    pub elapsed: std::time::Duration,
}

#[cfg(test)]
impl Default for AdaptiveControllerTestPolicyV1 {
    fn default() -> Self {
        Self {
            initial_proposal_quanta: None,
            growth_factor: 2,
        }
    }
}

#[cfg(test)]
std::thread_local! {
    static ADAPTIVE_CONTROLLER_TEST_POLICY: std::cell::RefCell<Option<AdaptiveControllerTestPolicyV1>> = const { std::cell::RefCell::new(None) };
    static ADAPTIVE_CONTROLLER_TEST_RECEIPTS: std::cell::RefCell<Option<Vec<Stage3AdaptiveSupportReceiptV1>>> = const { std::cell::RefCell::new(None) };
    static ACCEPTED_PUBLICATION_CAPTURE_AUDIT: std::cell::RefCell<Option<Vec<AcceptedPublicationCaptureAuditV1>>> = const { std::cell::RefCell::new(None) };
    static ACCEPTED_PUBLICATION_CAPTURE_REGIME: std::cell::Cell<AcceptedPublicationCaptureRegimeV1> = const { std::cell::Cell::new(AcceptedPublicationCaptureRegimeV1::Standalone) };
    static ADAPTIVE_PERFORMANCE_TEST_AUDIT: std::cell::RefCell<Option<Vec<AdaptivePerformanceSpanV1>>> = const { std::cell::RefCell::new(None) };
    static FORCE_ADAPTIVE_COVERED_CHILD_MEMO_DISABLED: std::cell::Cell<bool> = const { std::cell::Cell::new(false) };
}

std::thread_local! {
    // Explicitly opt-in, process-local diagnostic evidence. The production
    // execution path neither enables nor persists these comparison details.
    static ADAPTIVE_COMPARISON_TEST_AUDIT: std::cell::RefCell<Option<Vec<AdaptiveComparisonAuditV1>>> = const { std::cell::RefCell::new(None) };
}

#[cfg(test)]
pub(crate) struct AdaptiveCoveredChildMemoTestGuardV1;

#[cfg(test)]
impl Drop for AdaptiveCoveredChildMemoTestGuardV1 {
    fn drop(&mut self) {
        FORCE_ADAPTIVE_COVERED_CHILD_MEMO_DISABLED.with(|value| value.set(false));
    }
}

#[cfg(test)]
pub(crate) fn force_adaptive_covered_child_memo_disabled_for_test()
-> AdaptiveCoveredChildMemoTestGuardV1 {
    FORCE_ADAPTIVE_COVERED_CHILD_MEMO_DISABLED.with(|value| value.set(true));
    AdaptiveCoveredChildMemoTestGuardV1
}

#[cfg(test)]
fn adaptive_covered_child_memo_disabled_v1() -> bool {
    FORCE_ADAPTIVE_COVERED_CHILD_MEMO_DISABLED.with(std::cell::Cell::get)
}

#[cfg(not(test))]
const fn adaptive_covered_child_memo_disabled_v1() -> bool {
    false
}

#[cfg(test)]
pub(crate) struct AcceptedPublicationCaptureRegimeGuardV1 {
    previous: AcceptedPublicationCaptureRegimeV1,
}

#[cfg(test)]
impl Drop for AcceptedPublicationCaptureRegimeGuardV1 {
    fn drop(&mut self) {
        ACCEPTED_PUBLICATION_CAPTURE_REGIME.with(|value| value.set(self.previous));
    }
}

#[cfg(test)]
pub(crate) fn enter_accepted_publication_capture_regime_v1(
    regime: AcceptedPublicationCaptureRegimeV1,
) -> AcceptedPublicationCaptureRegimeGuardV1 {
    let previous = ACCEPTED_PUBLICATION_CAPTURE_REGIME.with(|value| value.replace(regime));
    AcceptedPublicationCaptureRegimeGuardV1 { previous }
}

#[cfg(test)]
pub(crate) fn accepted_publication_capture_regime_v1() -> AcceptedPublicationCaptureRegimeV1 {
    ACCEPTED_PUBLICATION_CAPTURE_REGIME.with(std::cell::Cell::get)
}

#[cfg(test)]
pub(crate) fn begin_adaptive_performance_test_audit() {
    ADAPTIVE_PERFORMANCE_TEST_AUDIT.with(|audit| *audit.borrow_mut() = Some(Vec::new()));
}

#[cfg(test)]
pub(crate) fn take_adaptive_performance_test_audit() -> Vec<AdaptivePerformanceSpanV1> {
    ADAPTIVE_PERFORMANCE_TEST_AUDIT.with(|audit| audit.borrow_mut().take().unwrap_or_default())
}

#[cfg(test)]
fn record_adaptive_performance_span_v1(
    label: &'static str,
    support_duration_ns: u128,
    started: std::time::Instant,
) {
    ADAPTIVE_PERFORMANCE_TEST_AUDIT.with(|audit| {
        if let Some(entries) = audit.borrow_mut().as_mut() {
            entries.push(AdaptivePerformanceSpanV1 {
                label,
                support_duration_ns,
                elapsed: started.elapsed(),
            });
        }
    });
}

#[cfg(test)]
pub(crate) fn begin_adaptive_controller_test_audit(policy: AdaptiveControllerTestPolicyV1) {
    assert!(policy.growth_factor > 0, "adaptive test growth factor");
    ADAPTIVE_CONTROLLER_TEST_POLICY.with(|value| *value.borrow_mut() = Some(policy));
    ADAPTIVE_CONTROLLER_TEST_RECEIPTS.with(|value| *value.borrow_mut() = Some(Vec::new()));
    ACCEPTED_PUBLICATION_CAPTURE_AUDIT.with(|value| *value.borrow_mut() = Some(Vec::new()));
}

#[cfg(test)]
pub(crate) fn take_adaptive_controller_test_audit() -> Vec<Stage3AdaptiveSupportReceiptV1> {
    ADAPTIVE_CONTROLLER_TEST_POLICY.with(|value| *value.borrow_mut() = None);
    ADAPTIVE_CONTROLLER_TEST_RECEIPTS.with(|value| value.borrow_mut().take().unwrap_or_default())
}

#[cfg(test)]
pub(crate) fn record_accepted_publication_capture_audit(audit: AcceptedPublicationCaptureAuditV1) {
    ACCEPTED_PUBLICATION_CAPTURE_AUDIT.with(|value| {
        if let Some(audits) = value.borrow_mut().as_mut() {
            audits.push(audit);
        }
    });
}

#[cfg(test)]
pub(crate) fn take_accepted_publication_capture_audit() -> Vec<AcceptedPublicationCaptureAuditV1> {
    ACCEPTED_PUBLICATION_CAPTURE_AUDIT.with(|value| value.borrow_mut().take().unwrap_or_default())
}

pub fn begin_adaptive_comparison_test_audit() {
    ADAPTIVE_COMPARISON_TEST_AUDIT.with(|audit| *audit.borrow_mut() = Some(Vec::new()));
}

pub fn take_adaptive_comparison_test_audit() -> Vec<AdaptiveComparisonAuditV1> {
    ADAPTIVE_COMPARISON_TEST_AUDIT.with(|audit| audit.borrow_mut().take().unwrap_or_default())
}

fn audit_adaptive_comparison_v1(value: AdaptiveComparisonAuditV1) {
    ADAPTIVE_COMPARISON_TEST_AUDIT.with(|audit| {
        if let Some(entries) = audit.borrow_mut().as_mut() {
            entries.push(value);
        }
    });
}

fn adaptive_comparison_test_audit_enabled_v1() -> bool {
    ADAPTIVE_COMPARISON_TEST_AUDIT.with(|audit| audit.borrow().is_some())
}

#[cfg(test)]
fn adaptive_test_initial_quanta(default: u128) -> u128 {
    ADAPTIVE_CONTROLLER_TEST_POLICY.with(|value| {
        value
            .borrow()
            .as_ref()
            .and_then(|policy| policy.initial_proposal_quanta)
            .unwrap_or(default)
    })
}

#[cfg(not(test))]
const fn adaptive_test_initial_quanta(default: u128) -> u128 {
    default
}

#[cfg(test)]
fn adaptive_test_growth_quanta(candidate: u128, remaining: u128) -> u128 {
    ADAPTIVE_CONTROLLER_TEST_POLICY.with(|value| {
        let factor = value
            .borrow()
            .as_ref()
            .map_or(2, |policy| policy.growth_factor);
        candidate.saturating_mul(factor).min(remaining)
    })
}

fn adaptive_refinable_lse_trial_failure_v1(
    error: &openwepp_land_surface_energy::LandSurfaceEnergyError,
) -> bool {
    matches!(
        error,
        openwepp_land_surface_energy::LandSurfaceEnergyError::NumericalSingular { .. }
            | openwepp_land_surface_energy::LandSurfaceEnergyError::NumericalBacktrackingLimit { .. }
            | openwepp_land_surface_energy::LandSurfaceEnergyError::NumericalIterationLimit { .. }
            | openwepp_land_surface_energy::LandSurfaceEnergyError::NumericalAcceptedResidual
            | openwepp_land_surface_energy::LandSurfaceEnergyError::ComponentClosure(
                "Stage-3 lower-boundary/column operand join"
            )
    )
}

fn adaptive_refinable_lse_shadow_trial_failure_v1(
    error: &crate::land_surface_energy_shadow::LandSurfaceEnergyShadowError,
) -> bool {
    match error {
        crate::land_surface_energy_shadow::LandSurfaceEnergyShadowError::LandSurface(error) => {
            adaptive_refinable_lse_trial_failure_v1(error)
        }
        _ => false,
    }
}

fn adaptive_refinable_v8_endpoint_trial_failure_v1(
    error: &crate::land_surface_energy_shadow::ExecuteV8LseRuntimeShadowError,
) -> bool {
    match error {
        crate::land_surface_energy_shadow::ExecuteV8LseRuntimeShadowError::Physical(error) => {
            adaptive_refinable_lse_shadow_trial_failure_v1(error)
        }
        _ => false,
    }
}

fn adaptive_refinable_v9_trial_failure_v1(
    error: &crate::v9_real_consumer_shadow::DirectV9RealConsumerError,
) -> bool {
    match error {
        crate::v9_real_consumer_shadow::DirectV9RealConsumerError::Physical(error) => {
            adaptive_refinable_v8_endpoint_trial_failure_v1(error)
        }
        crate::v9_real_consumer_shadow::DirectV9RealConsumerError::LandSurface(error) => {
            adaptive_refinable_lse_trial_failure_v1(error)
        }
        crate::v9_real_consumer_shadow::DirectV9RealConsumerError::LandSurfaceShadow(error) => {
            adaptive_refinable_lse_shadow_trial_failure_v1(error)
        }
        _ => false,
    }
}

fn adaptive_refinable_v10_trial_failure_v1(
    error: &crate::v9_real_consumer_shadow::DirectV10RealConsumerError,
) -> bool {
    match error {
        crate::v9_real_consumer_shadow::DirectV10RealConsumerError::LandSurface(error) => {
            adaptive_refinable_lse_trial_failure_v1(error)
        }
        crate::v9_real_consumer_shadow::DirectV10RealConsumerError::Runtime(error) => {
            adaptive_refinable_v9_trial_failure_v1(error)
        }
        _ => false,
    }
}

fn adaptive_refinable_cold_conductivity_failure_v1(
    error: &DirectSnowStage3EvaluationError,
) -> bool {
    let DirectSnowStage3EvaluationError::Kernel(source) = error else {
        return false;
    };
    let crate::hydrology::Wb11HydrologyKernelGuardError::SnowStage3Conductivity(snapshot) =
        source.as_ref()
    else {
        return false;
    };
    let openwepp_meteorology::MeteorologyError::BelowAbsoluteZero { quantity, value_c } =
        &snapshot.source
    else {
        return false;
    };
    let represented_swe_m = snapshot
        .control_volume_layers
        .iter()
        .map(|layer| layer.mass_swe_m)
        .sum::<f64>();
    *quantity == "temperature_c"
        && value_c.is_finite()
        && *value_c <= -273.15
        && snapshot.control_volume_temperature.as_celsius().to_bits() == value_c.to_bits()
        && snapshot.layer_index < snapshot.control_volume_layers.len()
        && snapshot.control_volume_layers[snapshot.layer_index] == snapshot.layer
        && represented_swe_m.is_finite()
        && represented_swe_m > 0.0
}

fn adaptive_refinable_v11_trial_failure_v1(error: &DirectV11RealConsumerError) -> bool {
    match error {
        DirectV11RealConsumerError::AdaptiveRefinement(_) => true,
        DirectV11RealConsumerError::CoveredBoundary(
            crate::snow_stage3_terminal_handoff::SnowStage3HandoffError::FixedPointIterationLimit,
        ) => true,
        DirectV11RealConsumerError::Runtime(error) => {
            adaptive_refinable_v10_trial_failure_v1(error)
        }
        DirectV11RealConsumerError::Stage3(error) => {
            adaptive_refinable_cold_conductivity_failure_v1(error)
        }
        DirectV11RealConsumerError::OpenSnowLowerBoundaryDomain {
            snow_temperature_k,
            latent_heat_j_kg,
            sensible_outward_w_m2,
            vapor_outward_kg_m2_s,
            net_longwave_w_m2,
            shortwave_absorbed_w_m2,
            albedo,
            beginning_stage3,
            forcing,
            exposure,
            optical,
            longwave,
            ..
        } => {
            snow_temperature_k.is_finite()
                && (0.0..200.0).contains(snow_temperature_k)
                && latent_heat_j_kg.is_finite()
                && *latent_heat_j_kg > 0.0
                && sensible_outward_w_m2.is_finite()
                && vapor_outward_kg_m2_s.is_finite()
                && net_longwave_w_m2.is_finite()
                && shortwave_absorbed_w_m2.is_finite()
                && albedo.is_finite()
                && (0.0..=1.0).contains(albedo)
                && [beginning_stage3, forcing, exposure, optical, longwave]
                    .into_iter()
                    .all(|digest| !digest.is_empty())
        }
        _ => false,
    }
}

fn adaptive_refinable_trial_failure_v1(error: &DirectSnowStage3V11AttachmentError) -> bool {
    if let DirectSnowStage3V11AttachmentError::AdaptiveTrial { source, .. } = error {
        return adaptive_refinable_trial_failure_v1(source);
    }
    matches!(
        error,
        DirectSnowStage3V11AttachmentError::AdaptiveRefinement(_)
    ) || matches!(
        error,
        DirectSnowStage3V11AttachmentError::Stage3(
            DirectSnowStage3EvaluationError::TerminalNumerics(
                crate::SnowTerminalNumericsFailure::BelowCarrierDomain
                    | crate::SnowTerminalNumericsFailure::StepUnderflow
            )
        )
    ) || matches!(
        error,
        DirectSnowStage3V11AttachmentError::Stage3(error)
            if adaptive_refinable_cold_conductivity_failure_v1(error)
    ) || matches!(
        error,
        DirectSnowStage3V11AttachmentError::V11(
            openwepp_vegetation::v11::V11ExecutionError::Executor(error)
        ) if adaptive_refinable_v11_trial_failure_v1(error)
    ) || matches!(
        error,
        DirectSnowStage3V11AttachmentError::Owner(error)
            if adaptive_refinable_v11_trial_failure_v1(error)
    )
}

fn contextualize_adaptive_trial_failure_v1<T>(
    phase: &'static str,
    supports: &[TimeSupport],
    result: Result<T, DirectSnowStage3V11AttachmentError>,
    started: Option<std::time::Instant>,
) -> Result<T, DirectSnowStage3V11AttachmentError> {
    if let Some(started) = started {
        crate::snow_stage3_v11_attachment::record_adaptive_parent_trial_phase_v1(
            phase,
            started.elapsed(),
        );
    }
    result.map_err(|source| {
        let start_ns = supports
            .first()
            .map_or(0, |support| support.start_ns().get());
        let end_ns = supports
            .last()
            .map_or(start_ns, |support| support.end_ns().get());
        DirectSnowStage3V11AttachmentError::AdaptiveTrial {
            phase,
            start_ns,
            end_ns,
            duration_ns: end_ns.saturating_sub(start_ns),
            source: Box::new(source),
        }
    })
}

enum AdaptiveTrialPairOutcomeV1<T> {
    Complete(T, T),
    Refinable {
        direct: Option<T>,
        composed: Option<T>,
    },
}

fn adaptive_propagate_non_refinable_trial_failure_v1<T>(
    direct: Result<T, DirectSnowStage3V11AttachmentError>,
    composed: Result<T, DirectSnowStage3V11AttachmentError>,
) -> Result<AdaptiveTrialPairOutcomeV1<T>, DirectSnowStage3V11AttachmentError> {
    match (direct, composed) {
        (Ok(direct), Ok(composed)) => Ok(AdaptiveTrialPairOutcomeV1::Complete(direct, composed)),
        (Err(direct), Err(composed)) => {
            if !adaptive_refinable_trial_failure_v1(&direct) {
                Err(direct)
            } else if !adaptive_refinable_trial_failure_v1(&composed) {
                Err(composed)
            } else {
                Ok(AdaptiveTrialPairOutcomeV1::Refinable {
                    direct: None,
                    composed: None,
                })
            }
        }
        (Err(direct), Ok(composed)) => {
            if adaptive_refinable_trial_failure_v1(&direct) {
                Ok(AdaptiveTrialPairOutcomeV1::Refinable {
                    direct: None,
                    composed: Some(composed),
                })
            } else {
                Err(direct)
            }
        }
        (Ok(direct), Err(composed)) => {
            if adaptive_refinable_trial_failure_v1(&composed) {
                Ok(AdaptiveTrialPairOutcomeV1::Refinable {
                    direct: Some(direct),
                    composed: None,
                })
            } else {
                Err(composed)
            }
        }
    }
}

fn accept_adaptive_floor_trial_v1<T>(
    direct: Result<T, DirectSnowStage3V11AttachmentError>,
) -> Result<T, DirectSnowStage3V11AttachmentError> {
    // The exact 60-second floor has no smaller admissible child. Candidate-
    // local failures therefore remain fail-closed even when the same failure
    // is refinable for a wider direct/composed comparison.
    direct
}

#[cfg(not(test))]
fn adaptive_test_growth_quanta(candidate: u128, remaining: u128) -> u128 {
    candidate.saturating_mul(2).min(remaining)
}

#[allow(
    clippy::too_many_arguments,
    clippy::too_many_lines,
    clippy::type_complexity
)]
#[cfg(test)]
pub(crate) fn execute_covered_real_v11_parent(
    context: &DirectSnowStage3V11StaticContext,
    beginning_parent: &V11ParentTransaction,
    beginning_consumer: &DirectV10RealConsumerShadow,
    beginning_clock: &CoupledClockStateV1,
    prepared: &DirectSnowStage3V11PreparedSupport,
    day_index: usize,
    interval_index: usize,
    forcing_receipt: Digest32,
    beginning_stage3: BTreeMap<u32, DirectSnowStage3PersistentState>,
    beginning_terminal_parcels: BTreeMap<Digest32, DirectSnowStage3V11TerminalParcel>,
    failure_injection: Option<Stage3V11FailureInjection>,
) -> Result<
    (
        V11ParentTransaction,
        DirectV10RealConsumerShadow,
        CoupledClockStateV1,
        V11ParentCandidate,
        BTreeMap<u32, DirectSnowStage3PersistentState>,
        Vec<Stage3CoupledSubslabReceiptV1>,
        Vec<Stage3V11TerminalEventGroupV1>,
        Vec<DirectSnowStage3V11TerminalParcel>,
    ),
    DirectSnowStage3V11AttachmentError,
> {
    let mut evidence = <crate::hydrology::NoEvidence as crate::hydrology::TerminalEvidenceMode<
        Option<CoveredTerminalJointTrialStateV1>,
    >>::new_state();
    let AdaptiveSupportExecutionOutcomeV2::Complete((
        parent,
        consumer,
        clock,
        finalized,
        stage3,
        _snow_enthalpy_material_owner,
        _snow_enthalpy_material_owner_chronology,
        receipts,
        groups,
        parcels,
        _adaptive_support_receipt,
        _snow_free_successor_receipts,
    )) = execute_covered_real_v11_parent_with_evidence::<crate::hydrology::NoEvidence>(
        context,
        beginning_parent,
        beginning_consumer,
        beginning_clock,
        prepared,
        day_index,
        interval_index,
        forcing_receipt,
        beginning_stage3,
        None,
        Vec::new(),
        beginning_terminal_parcels,
        failure_injection,
        &mut evidence,
        None,
        None,
    )?
    else {
        return Err(DirectSnowStage3V11AttachmentError::Identity(
            "unexpected adaptive restart interruption",
        ));
    };
    #[cfg(test)]
    ADAPTIVE_CONTROLLER_TEST_RECEIPTS.with(|value| {
        if let Some(receipts) = value.borrow_mut().as_mut() {
            receipts.push(_adaptive_support_receipt);
        }
    });
    Ok((
        parent, consumer, clock, finalized, stage3, receipts, groups, parcels,
    ))
}
