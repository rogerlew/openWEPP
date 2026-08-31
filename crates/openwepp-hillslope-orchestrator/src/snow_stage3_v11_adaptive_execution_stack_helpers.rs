enum AdaptiveCandidateSelectionV1<T> {
    Accepted {
        trial: Box<T>,
        maximum_scaled_error: f64,
    },
    Refine {
        next_trial_quanta: u128,
    },
}

enum AdaptiveParentLoopOutcomeV1 {
    Paused(Box<AdaptiveSupportExecutionOutcomeV2>),
    Complete(Box<AdaptiveParentExecutionStateV1>),
}

struct AdaptiveSnowFreeSuccessorExecutionV1 {
    parent: V11ParentTransaction,
    consumer: DirectV10RealConsumerShadow,
    clock: CoupledClockStateV1,
    accepted_support: RealV11AcceptedSupportIdentityV1,
}

#[allow(clippy::too_many_arguments)]
#[inline(never)]
fn execute_adaptive_snow_free_successor_v1(
    context: &DirectSnowStage3V11StaticContext,
    parent: &V11ParentTransaction,
    consumer: &DirectV10RealConsumerShadow,
    clock: &CoupledClockStateV1,
    successor: &DirectSnowStage3V11PreparedSupport,
    day_index: usize,
    interval_index: usize,
    forcing_receipt: Digest32,
    ending_snow_owner_bytes: Vec<u8>,
) -> Result<Box<AdaptiveSnowFreeSuccessorExecutionV1>, DirectSnowStage3V11AttachmentError> {
    let (parent, consumer, clock, _, accepted_support) = execute_real_v11_parent(
        context,
        parent,
        consumer,
        clock,
        successor,
        day_index,
        interval_index,
        forcing_receipt,
        ending_snow_owner_bytes,
        false,
    )?;
    Ok(Box::new(AdaptiveSnowFreeSuccessorExecutionV1 {
        parent,
        consumer,
        clock,
        accepted_support,
    }))
}

#[inline(never)]
fn execute_adaptive_parent_loop_closure_v1<F>(
    execution: Box<AdaptiveParentExecutionStateV1>,
    execute: F,
) -> Result<AdaptiveParentLoopOutcomeV1, DirectSnowStage3V11AttachmentError>
where
    F: FnOnce(
        Box<AdaptiveParentExecutionStateV1>,
    ) -> Result<AdaptiveParentLoopOutcomeV1, DirectSnowStage3V11AttachmentError>,
{
    execute(execution)
}

#[allow(clippy::too_many_arguments, clippy::too_many_lines)]
#[inline(never)]
fn select_adaptive_covered_candidate_v1(
    context: &DirectSnowStage3V11StaticContext,
    parent: &V11ParentTransaction,
    consumer: &DirectV10RealConsumerShadow,
    clock: &CoupledClockStateV1,
    prepared: &DirectSnowStage3V11PreparedSupport,
    day_index: usize,
    interval_index: usize,
    forcing_receipt: Digest32,
    stage3: &BTreeMap<u32, DirectSnowStage3PersistentState>,
    pending_terminal_parcels: &BTreeMap<Digest32, DirectSnowStage3V11TerminalParcel>,
    covered_trial_memo: &mut Vec<AdaptiveCoveredTrialMemoEntryV1>,
    adaptive_receipts: &mut AdaptiveReceiptAccumulatorV1,
    adaptive_request: &Stage3AdaptiveParentRequestReceiptV1,
    support: TimeSupport,
    candidate_quanta: u128,
    child_ordinal: u32,
) -> Result<AdaptiveCandidateSelectionV1<AdaptiveCoveredTrialV1>, DirectSnowStage3V11AttachmentError>
{
    let direct_supports = [support];
    let direct_started = crate::snow_stage3_v11_attachment::adaptive_parent_telemetry_enabled_v1()
        .then(std::time::Instant::now);
    let direct = contextualize_adaptive_trial_failure_v1(
        "covered direct",
        &direct_supports,
        execute_adaptive_covered_trial_v1(
            context,
            parent,
            consumer,
            clock,
            prepared,
            day_index,
            interval_index,
            forcing_receipt,
            stage3,
            pending_terminal_parcels,
            Some(covered_trial_memo),
            &direct_supports,
            child_ordinal,
        ),
        direct_started,
    );
    if candidate_quanta == 1 {
        let direct = accept_adaptive_floor_trial_v1(direct)?;
        let direct_evidence = adaptive_direct_trial_receipt_v1(
            adaptive_request,
            &direct.consumer,
            &direct.clock,
            &direct.stage3,
            &direct.receipts,
            pending_terminal_parcels,
            false,
        )?;
        let comparison = Stage3AdaptiveStepComparisonReceiptV1::try_floor(
            adaptive_request,
            &direct_evidence.receipt,
            direct_evidence.exact_discrete_sha256,
            direct_evidence.exact_discrete_surfaces,
            true,
        )?;
        let accepted_receipt = Stage3AdaptiveAcceptedMicrostepReceiptV1::try_new(&comparison)?;
        adaptive_receipts
            .parent_requests
            .push(adaptive_request.clone());
        adaptive_receipts
            .direct_trials
            .push(direct_evidence.receipt);
        adaptive_receipts.comparisons.push(comparison);
        adaptive_receipts.accept(accepted_receipt);
        return Ok(AdaptiveCandidateSelectionV1::Accepted {
            trial: direct,
            maximum_scaled_error: 0.0,
        });
    }

    let first_quanta = candidate_quanta / 2;
    let first_end = ModelTimeNs::new(
        clock.accepted_until().get() + first_quanta * STAGE3_V11_ADAPTIVE_MINIMUM_SUPPORT_NS,
    );
    let first_support = TimeSupport::new(clock.accepted_until(), first_end)?;
    let second_support = TimeSupport::new(first_end, support.end_ns())?;
    let composed_supports = [first_support, second_support];
    let composed_started =
        crate::snow_stage3_v11_attachment::adaptive_parent_telemetry_enabled_v1()
            .then(std::time::Instant::now);
    let composed = contextualize_adaptive_trial_failure_v1(
        "covered composed",
        &composed_supports,
        execute_adaptive_covered_trial_v1(
            context,
            parent,
            consumer,
            clock,
            prepared,
            day_index,
            interval_index,
            forcing_receipt,
            stage3,
            pending_terminal_parcels,
            Some(covered_trial_memo),
            &composed_supports,
            child_ordinal,
        ),
        composed_started,
    );
    let (direct, mut composed) =
        match adaptive_propagate_non_refinable_trial_failure_v1(direct, composed)? {
            AdaptiveTrialPairOutcomeV1::Complete(direct, composed) => (direct, composed),
            AdaptiveTrialPairOutcomeV1::Refinable {
                direct,
                mut composed,
            } => {
                let direct_evidence = direct
                    .as_ref()
                    .map(|direct| {
                        adaptive_direct_trial_receipt_v1(
                            adaptive_request,
                            &direct.consumer,
                            &direct.clock,
                            &direct.stage3,
                            &direct.receipts,
                            pending_terminal_parcels,
                            false,
                        )
                    })
                    .transpose()?;
                adaptive_record_refinable_trial_failure_v1(
                    adaptive_receipts,
                    adaptive_request,
                    consumer,
                    stage3,
                    pending_terminal_parcels,
                    first_support,
                    second_support,
                    direct_evidence,
                    composed
                        .as_ref()
                        .map(|composed| composed.receipts.as_slice()),
                )?;
                if let Some(composed) = composed.as_mut() {
                    covered_trial_memo.append(&mut composed.composed_children);
                }
                return Ok(AdaptiveCandidateSelectionV1::Refine {
                    next_trial_quanta: first_quanta,
                });
            }
        };
    let direct_evidence = adaptive_direct_trial_receipt_v1(
        adaptive_request,
        &direct.consumer,
        &direct.clock,
        &direct.stage3,
        &direct.receipts,
        pending_terminal_parcels,
        false,
    )?;
    let (_, composed_owner_comparison, maximum_scaled_error, discrete_mismatch) =
        adaptive_complete_owner_error_v1(
            adaptive_request.context.step_support,
            &direct.consumer,
            &direct.stage3,
            &composed.consumer,
            &composed.stage3,
            pending_terminal_parcels,
            pending_terminal_parcels,
        )?;
    let child_1_receipts = composed
        .receipts
        .iter()
        .filter(|receipt| receipt.support == first_support)
        .cloned()
        .collect::<Vec<_>>();
    let child_2_receipts = composed
        .receipts
        .iter()
        .filter(|receipt| receipt.support == second_support)
        .cloned()
        .collect::<Vec<_>>();
    let child_1 = Stage3AdaptiveSplitChildTrialReceiptV1::try_child_1(
        adaptive_request,
        &direct_evidence.receipt,
        first_support,
        adaptive_trial_ledger_set_sha256_v1(&child_1_receipts)?,
        child_1_receipts
            .last()
            .ok_or(DirectSnowStage3V11AttachmentError::Identity(
                "adaptive covered first-child receipt",
            ))?
            .effective_ending_complete_owner_set_sha256(),
        adaptive_child_phase_result_sha256_v1(&child_1_receipts)?,
        Stage3AdaptiveEventPostureV1::NoEvent,
        Stage3AdaptiveTrialDispositionV1::Closed,
    )?;
    let child_2 = Stage3AdaptiveSplitChildTrialReceiptV1::try_child_2(
        adaptive_request,
        &child_1,
        second_support,
        adaptive_trial_ledger_set_sha256_v1(&child_2_receipts)?,
        child_2_receipts
            .last()
            .ok_or(DirectSnowStage3V11AttachmentError::Identity(
                "adaptive covered second-child receipt",
            ))?
            .effective_ending_complete_owner_set_sha256(),
        adaptive_child_phase_result_sha256_v1(&child_2_receipts)?,
        Stage3AdaptiveEventPostureV1::NoEvent,
        Stage3AdaptiveTrialDispositionV1::Closed,
    )?;
    let (composed_exact_discrete_sha256, composed_exact_discrete_surfaces) =
        adaptive_discrete_surface_receipts_v1(
            &composed_owner_comparison,
            pending_terminal_parcels,
        )?;
    let accepted_comparison = !discrete_mismatch && maximum_scaled_error <= 1.0;
    let comparison = Stage3AdaptiveStepComparisonReceiptV1::try_composed(
        adaptive_request,
        &direct_evidence.receipt,
        &child_1,
        &child_2,
        adaptive_trial_ledger_set_sha256_v1(&composed.receipts)?,
        direct_evidence.exact_discrete_sha256,
        composed_exact_discrete_sha256,
        direct_evidence.exact_discrete_surfaces,
        composed_exact_discrete_surfaces,
        maximum_scaled_error,
        discrete_mismatch,
        accepted_comparison,
    )?;
    adaptive_receipts
        .parent_requests
        .push(adaptive_request.clone());
    adaptive_receipts
        .direct_trials
        .push(direct_evidence.receipt);
    adaptive_receipts.split_child_trials.push(child_1);
    adaptive_receipts.split_child_trials.push(child_2);
    adaptive_receipts.comparisons.push(comparison.clone());
    if !accepted_comparison {
        adaptive_receipts.reject(discrete_mismatch, false)?;
        covered_trial_memo.append(&mut composed.composed_children);
        return Ok(AdaptiveCandidateSelectionV1::Refine {
            next_trial_quanta: first_quanta,
        });
    }
    adaptive_receipts.accept(Stage3AdaptiveAcceptedMicrostepReceiptV1::try_new(
        &comparison,
    )?);
    Ok(AdaptiveCandidateSelectionV1::Accepted {
        trial: composed,
        maximum_scaled_error,
    })
}

#[allow(clippy::too_many_arguments, clippy::too_many_lines)]
#[inline(never)]
fn select_adaptive_terminal_candidate_v1<M>(
    context: &DirectSnowStage3V11StaticContext,
    parent: &V11ParentTransaction,
    consumer: &DirectV10RealConsumerShadow,
    clock: &CoupledClockStateV1,
    prepared: &DirectSnowStage3V11PreparedSupport,
    day_index: usize,
    interval_index: usize,
    forcing_receipt: Digest32,
    stage3: &BTreeMap<u32, DirectSnowStage3PersistentState>,
    pending_terminal_parcels: &BTreeMap<Digest32, DirectSnowStage3V11TerminalParcel>,
    adaptive_receipts: &mut AdaptiveReceiptAccumulatorV1,
    adaptive_request: &Stage3AdaptiveParentRequestReceiptV1,
    support: TimeSupport,
    candidate_quanta: u128,
    child_ordinal: u32,
    event_ordinal: u64,
    evidence: &mut M::State,
) -> Result<AdaptiveCandidateSelectionV1<AdaptiveTerminalPathV1>, DirectSnowStage3V11AttachmentError>
where
    M: crate::hydrology::TerminalEvidenceMode<Option<CoveredTerminalJointTrialStateV1>>,
{
    let direct_supports = [support];
    let direct_started = crate::snow_stage3_v11_attachment::adaptive_parent_telemetry_enabled_v1()
        .then(std::time::Instant::now);
    let direct = contextualize_adaptive_trial_failure_v1(
        "terminal direct",
        &direct_supports,
        execute_adaptive_terminal_path_v1::<M>(
            context,
            parent,
            consumer,
            clock,
            prepared,
            day_index,
            interval_index,
            forcing_receipt,
            stage3,
            pending_terminal_parcels,
            &direct_supports,
            child_ordinal,
            event_ordinal,
            evidence,
        ),
        direct_started,
    );
    if candidate_quanta == 1 {
        let direct = accept_adaptive_floor_trial_v1(direct)?;
        let direct_evidence = adaptive_direct_trial_receipt_v1(
            adaptive_request,
            &direct.actual.consumer,
            &direct.actual.clock,
            &direct.actual.stage3,
            &direct.actual.receipts,
            &direct.ending_pending_terminal_parcels,
            direct.actual.group.is_some(),
        )?;
        let comparison = Stage3AdaptiveStepComparisonReceiptV1::try_floor(
            adaptive_request,
            &direct_evidence.receipt,
            direct_evidence.exact_discrete_sha256,
            direct_evidence.exact_discrete_surfaces,
            true,
        )?;
        let accepted_receipt = Stage3AdaptiveAcceptedMicrostepReceiptV1::try_new(&comparison)?;
        adaptive_receipts
            .parent_requests
            .push(adaptive_request.clone());
        adaptive_receipts
            .direct_trials
            .push(direct_evidence.receipt);
        adaptive_receipts.comparisons.push(comparison);
        adaptive_receipts.accept(accepted_receipt);
        return Ok(AdaptiveCandidateSelectionV1::Accepted {
            trial: Box::new(direct),
            maximum_scaled_error: 0.0,
        });
    }

    let first_quanta = candidate_quanta / 2;
    let first_end = ModelTimeNs::new(
        clock.accepted_until().get() + first_quanta * STAGE3_V11_ADAPTIVE_MINIMUM_SUPPORT_NS,
    );
    let first_support = TimeSupport::new(clock.accepted_until(), first_end)?;
    let second_support = TimeSupport::new(first_end, support.end_ns())?;
    let composed_supports = [first_support, second_support];
    let composed_started =
        crate::snow_stage3_v11_attachment::adaptive_parent_telemetry_enabled_v1()
            .then(std::time::Instant::now);
    let composed = contextualize_adaptive_trial_failure_v1(
        "terminal composed",
        &composed_supports,
        execute_adaptive_terminal_path_v1::<M>(
            context,
            parent,
            consumer,
            clock,
            prepared,
            day_index,
            interval_index,
            forcing_receipt,
            stage3,
            pending_terminal_parcels,
            &composed_supports,
            child_ordinal,
            event_ordinal,
            evidence,
        ),
        composed_started,
    );
    let (direct, composed) =
        match adaptive_propagate_non_refinable_trial_failure_v1(direct, composed)? {
            AdaptiveTrialPairOutcomeV1::Complete(direct, composed) => (direct, composed),
            AdaptiveTrialPairOutcomeV1::Refinable { direct, composed } => {
                let direct_evidence = direct
                    .as_ref()
                    .map(|direct| {
                        adaptive_direct_trial_receipt_v1(
                            adaptive_request,
                            &direct.actual.consumer,
                            &direct.actual.clock,
                            &direct.actual.stage3,
                            &direct.actual.receipts,
                            &direct.ending_pending_terminal_parcels,
                            direct.actual.group.is_some(),
                        )
                    })
                    .transpose()?;
                adaptive_record_refinable_trial_failure_v1(
                    adaptive_receipts,
                    adaptive_request,
                    consumer,
                    stage3,
                    pending_terminal_parcels,
                    first_support,
                    second_support,
                    direct_evidence,
                    composed
                        .as_ref()
                        .map(|composed| composed.actual.receipts.as_slice()),
                )?;
                return Ok(AdaptiveCandidateSelectionV1::Refine {
                    next_trial_quanta: first_quanta,
                });
            }
        };
    let direct_posture = direct.actual.group.as_ref().map(|group| {
        (
            group.tick,
            group.terminating_lanes.clone(),
            group.post_active_lanes.clone(),
        )
    });
    let composed_posture = composed.actual.group.as_ref().map(|group| {
        (
            group.tick,
            group.terminating_lanes.clone(),
            group.post_active_lanes.clone(),
        )
    });
    let direct_evidence = adaptive_direct_trial_receipt_v1(
        adaptive_request,
        &direct.actual.consumer,
        &direct.actual.clock,
        &direct.actual.stage3,
        &direct.actual.receipts,
        &direct.ending_pending_terminal_parcels,
        direct.actual.group.is_some(),
    )?;
    let (_, composed_owner_comparison, maximum_scaled_error, mut discrete_mismatch) =
        adaptive_complete_owner_error_v1(
            adaptive_request.context.step_support,
            &direct.actual.consumer,
            &direct.actual.stage3,
            &composed.actual.consumer,
            &composed.actual.stage3,
            &direct.ending_pending_terminal_parcels,
            &composed.ending_pending_terminal_parcels,
        )?;
    let event_mismatch = direct_posture != composed_posture;
    discrete_mismatch |= event_mismatch;
    let first_receipts = composed
        .actual
        .receipts
        .iter()
        .filter(|receipt| {
            receipt.support.start_ns() >= first_support.start_ns()
                && receipt.support.end_ns() <= first_support.end_ns()
        })
        .cloned()
        .collect::<Vec<_>>();
    let second_receipts = composed
        .actual
        .receipts
        .iter()
        .filter(|receipt| {
            receipt.support.start_ns() >= second_support.start_ns()
                && receipt.support.end_ns() <= second_support.end_ns()
        })
        .cloned()
        .collect::<Vec<_>>();
    let child_1 = Stage3AdaptiveSplitChildTrialReceiptV1::try_child_1(
        adaptive_request,
        &direct_evidence.receipt,
        first_support,
        adaptive_trial_ledger_set_sha256_v1(&first_receipts)?,
        first_receipts
            .last()
            .ok_or(DirectSnowStage3V11AttachmentError::Identity(
                "adaptive terminal first-child receipt",
            ))?
            .effective_ending_complete_owner_set_sha256(),
        adaptive_child_phase_result_sha256_v1(&first_receipts)?,
        adaptive_event_posture_v1(
            first_receipts
                .iter()
                .any(|receipt| !receipt.terminal_events.is_empty()),
            &BTreeMap::new(),
        ),
        Stage3AdaptiveTrialDispositionV1::Closed,
    )?;
    let child_2 = Stage3AdaptiveSplitChildTrialReceiptV1::try_child_2(
        adaptive_request,
        &child_1,
        second_support,
        adaptive_trial_ledger_set_sha256_v1(&second_receipts)?,
        second_receipts
            .last()
            .ok_or(DirectSnowStage3V11AttachmentError::Identity(
                "adaptive terminal second-child receipt",
            ))?
            .effective_ending_complete_owner_set_sha256(),
        adaptive_child_phase_result_sha256_v1(&second_receipts)?,
        adaptive_event_posture_v1(
            second_receipts
                .iter()
                .any(|receipt| !receipt.terminal_events.is_empty()),
            &composed.ending_pending_terminal_parcels,
        ),
        Stage3AdaptiveTrialDispositionV1::Closed,
    )?;
    let (composed_exact_discrete_sha256, composed_exact_discrete_surfaces) =
        adaptive_discrete_surface_receipts_v1(
            &composed_owner_comparison,
            &composed.ending_pending_terminal_parcels,
        )?;
    let accepted_comparison = !discrete_mismatch && maximum_scaled_error <= 1.0;
    let comparison = Stage3AdaptiveStepComparisonReceiptV1::try_composed(
        adaptive_request,
        &direct_evidence.receipt,
        &child_1,
        &child_2,
        adaptive_trial_ledger_set_sha256_v1(&composed.actual.receipts)?,
        direct_evidence.exact_discrete_sha256,
        composed_exact_discrete_sha256,
        direct_evidence.exact_discrete_surfaces,
        composed_exact_discrete_surfaces,
        maximum_scaled_error,
        discrete_mismatch,
        accepted_comparison,
    )?;
    adaptive_receipts
        .parent_requests
        .push(adaptive_request.clone());
    adaptive_receipts
        .direct_trials
        .push(direct_evidence.receipt);
    adaptive_receipts.split_child_trials.push(child_1);
    adaptive_receipts.split_child_trials.push(child_2);
    adaptive_receipts.comparisons.push(comparison.clone());
    if !accepted_comparison {
        adaptive_receipts.reject(discrete_mismatch, event_mismatch)?;
        return Ok(AdaptiveCandidateSelectionV1::Refine {
            next_trial_quanta: first_quanta,
        });
    }
    adaptive_receipts.accept(Stage3AdaptiveAcceptedMicrostepReceiptV1::try_new(
        &comparison,
    )?);
    Ok(AdaptiveCandidateSelectionV1::Accepted {
        trial: Box::new(composed),
        maximum_scaled_error,
    })
}

#[allow(clippy::too_many_arguments)]
#[inline(never)]
fn adaptive_interruption_outcome_v2(
    interrupt_at: Option<DirectSnowStage3V11InterruptionPostureV2>,
    posture: DirectSnowStage3V11InterruptionPostureV2,
    restart: &mut Option<Box<DirectSnowStage3V11InProgressExecutionV2>>,
    request: Option<&Stage3AdaptiveParentRequestReceiptV1>,
    parent: &V11ParentTransaction,
    consumer: &DirectV10RealConsumerShadow,
    clock: &CoupledClockStateV1,
    stage3: &BTreeMap<u32, DirectSnowStage3PersistentState>,
    pending_terminal_parcels: &BTreeMap<Digest32, DirectSnowStage3V11TerminalParcel>,
    owner_joins: &[Stage3CoupledSubslabReceiptV1],
    event_groups: &[Stage3V11TerminalEventGroupV1],
    terminal_parcels: &[DirectSnowStage3V11TerminalParcel],
    expected_child_beginning: Digest32,
    adaptive_receipts: &AdaptiveReceiptAccumulatorV1,
    snow_free_successor_receipts: &[Stage3SnowFreeSuccessorReceiptV1],
    adaptive_trial_quanta: u128,
) -> Result<Option<Box<AdaptiveSupportExecutionOutcomeV2>>, DirectSnowStage3V11AttachmentError> {
    if interrupt_at != Some(posture) {
        return Ok(None);
    }
    let checkpoint = restart
        .as_mut()
        .ok_or(DirectSnowStage3V11AttachmentError::Identity(
            "adaptive interruption requested without day checkpoint",
        ))?;
    checkpoint.posture = posture;
    checkpoint.support_current = Some(DirectSnowStage3V11CommittedState {
        stage3_by_lane: stage3.clone(),
        real_consumer: consumer.clone(),
        v11_parent_state: parent.clone(),
        coupled_clock: clock.clone(),
        next_parent_sequence: checkpoint.day_candidate.next_parent_sequence,
        last_v11_parent_candidate: checkpoint.day_candidate.last_v11_parent_candidate.clone(),
        terminal_parcels: pending_terminal_parcels.clone(),
        receipt_chain: checkpoint.day_candidate.receipt_chain.clone(),
    });
    checkpoint.support_owner_joins = owner_joins.to_vec();
    checkpoint.support_event_groups = event_groups.to_vec();
    checkpoint.support_terminal_parcels = terminal_parcels.to_vec();
    checkpoint.expected_child_beginning = expected_child_beginning;
    checkpoint.pending_adaptive_request = request.cloned();
    checkpoint.adaptive_receipts = adaptive_receipts.clone();
    checkpoint.support_snow_free_successor_receipts = snow_free_successor_receipts.to_vec();
    checkpoint.adaptive_trial_quanta = adaptive_trial_quanta;
    Ok(Some(Box::new(AdaptiveSupportExecutionOutcomeV2::Paused(
        restart
            .take()
            .ok_or(DirectSnowStage3V11AttachmentError::Identity(
                "adaptive interruption checkpoint",
            ))?,
    ))))
}

struct AdaptiveParentExecutionStateV1 {
    prepared: DirectSnowStage3V11PreparedSupport,
    restart: Option<Box<DirectSnowStage3V11InProgressExecutionV2>>,
    parent: V11ParentTransaction,
    consumer: DirectV10RealConsumerShadow,
    clock: CoupledClockStateV1,
    stage3: BTreeMap<u32, DirectSnowStage3PersistentState>,
    owner_joins: Vec<Stage3CoupledSubslabReceiptV1>,
    event_groups: Vec<Stage3V11TerminalEventGroupV1>,
    terminal_parcels: Vec<DirectSnowStage3V11TerminalParcel>,
    pending_terminal_parcels: BTreeMap<Digest32, DirectSnowStage3V11TerminalParcel>,
    expected_child_beginning: Digest32,
    adaptive_receipts: AdaptiveReceiptAccumulatorV1,
    snow_free_successor_receipts: Vec<Stage3SnowFreeSuccessorReceiptV1>,
    adaptive_trial_quanta: u128,
    covered_trial_memo: Vec<AdaptiveCoveredTrialMemoEntryV1>,
}

enum AdaptiveParentInitializationOutcomeV1 {
    Paused(Box<AdaptiveSupportExecutionOutcomeV2>),
    Ready(Box<AdaptiveParentExecutionStateV1>),
}

#[allow(clippy::too_many_arguments, clippy::too_many_lines)]
#[inline(never)]
fn initialize_adaptive_parent_execution_v1(
    context: &DirectSnowStage3V11StaticContext,
    beginning_parent: &V11ParentTransaction,
    beginning_consumer: &DirectV10RealConsumerShadow,
    beginning_clock: &CoupledClockStateV1,
    prepared: &DirectSnowStage3V11PreparedSupport,
    beginning_stage3: BTreeMap<u32, DirectSnowStage3PersistentState>,
    beginning_terminal_parcels: BTreeMap<Digest32, DirectSnowStage3V11TerminalParcel>,
    failure_injection: Option<Stage3V11FailureInjection>,
    restart: Option<Box<DirectSnowStage3V11InProgressExecutionV2>>,
    interrupt_at: Option<DirectSnowStage3V11InterruptionPostureV2>,
) -> Result<AdaptiveParentInitializationOutcomeV1, DirectSnowStage3V11AttachmentError> {
    let support_beginning_stage3 = beginning_stage3.clone();
    let support_beginning_terminal_parcels = beginning_terminal_parcels.clone();
    let solid_reappearance = canonical_solid_reappearance_transition_v1(
        context,
        beginning_parent,
        beginning_consumer,
        beginning_clock,
        prepared,
        &support_beginning_stage3,
        &support_beginning_terminal_parcels,
    )?;
    let solid_reappearance_event = solid_reappearance
        .as_ref()
        .map(|transition| transition.accepted_event.clone());
    let debited_prepared = solid_reappearance
        .as_ref()
        .map(|transition| prepared.after_solid_reappearance_debit(&transition.lanes))
        .transpose()?;
    let mut restart = restart;
    let (
        mut parent,
        mut consumer,
        mut clock,
        mut stage3,
        owner_joins,
        mut event_groups,
        mut terminal_parcels,
        mut pending_terminal_parcels,
        mut expected_child_beginning,
        adaptive_receipts,
        snow_free_successor_receipts,
    ) = if let Some(checkpoint) = restart.as_ref() {
        let current = checkpoint.support_current.as_ref().ok_or(
            DirectSnowStage3V11AttachmentError::Identity("restart current adaptive support"),
        )?;
        (
            current.v11_parent_state.clone(),
            current.real_consumer.clone(),
            current.coupled_clock.clone(),
            current.stage3_by_lane.clone(),
            checkpoint.support_owner_joins.clone(),
            checkpoint.support_event_groups.clone(),
            checkpoint.support_terminal_parcels.clone(),
            current.terminal_parcels.clone(),
            checkpoint.expected_child_beginning,
            checkpoint.adaptive_receipts.clone(),
            checkpoint.support_snow_free_successor_receipts.clone(),
        )
    } else {
        (
            beginning_parent.clone(),
            beginning_consumer.clone(),
            beginning_clock.clone(),
            beginning_stage3,
            Vec::new(),
            Vec::new(),
            Vec::new(),
            beginning_terminal_parcels,
            complete_owner_set_digest(beginning_clock.owners())?,
            AdaptiveReceiptAccumulatorV1::default(),
            Vec::new(),
        )
    };
    const ADAPTIVE_MIN_STEP_NS: u128 = STAGE3_V11_ADAPTIVE_MINIMUM_SUPPORT_NS;
    if prepared.support.duration_ns() % ADAPTIVE_MIN_STEP_NS != 0 {
        return Err(DirectSnowStage3V11AttachmentError::Support(
            "adaptive parent support is outside the 60-second grid",
        ));
    }
    let adaptive_trial_quanta = restart.as_ref().map_or_else(
        || adaptive_test_initial_quanta(prepared.support.duration_ns() / ADAPTIVE_MIN_STEP_NS),
        |checkpoint| checkpoint.adaptive_trial_quanta,
    );
    macro_rules! interrupt {
        ($posture:expr) => {
            if let Some(outcome) = adaptive_interruption_outcome_v2(
                interrupt_at,
                $posture,
                &mut restart,
                None,
                &parent,
                &consumer,
                &clock,
                &stage3,
                &pending_terminal_parcels,
                &owner_joins,
                &event_groups,
                &terminal_parcels,
                expected_child_beginning,
                &adaptive_receipts,
                &snow_free_successor_receipts,
                adaptive_trial_quanta,
            )? {
                return Ok(AdaptiveParentInitializationOutcomeV1::Paused(outcome));
            }
        };
    }
    if let Some(transition) = solid_reappearance {
        let already_applied = clock
            .accepted_event_receipts()
            .iter()
            .any(|receipt| receipt == &transition.accepted_event);
        if already_applied {
            let publication_retained =
                consumer.retains_accepted_publication_event_handoff(&transition.accepted_event);
            let publication_is_ordered_tail =
                consumer.accepted_publication_event_handoff_is_tail(&transition.accepted_event);
            let at_open_parent_beginning = clock.accepted_until() == prepared.support.start_ns();
            validate_solid_reappearance_publication_posture_v1(
                at_open_parent_beginning,
                stage3 == transition.stage3,
                publication_retained,
                publication_is_ordered_tail,
            )?;
        } else {
            if clock.accepted_until() != prepared.support.start_ns()
                || stage3 != support_beginning_stage3
                || complete_owner_set_digest(clock.owners())?
                    != complete_owner_set_digest(beginning_clock.owners())?
            {
                return Err(DirectSnowStage3V11AttachmentError::Identity(
                    "solid reappearance omitted event",
                ));
            }
            interrupt!(DirectSnowStage3V11InterruptionPostureV2::BeforeSnowReappearance);
            parent = transition.parent;
            consumer = transition.consumer;
            clock = transition.clock;
            stage3 = transition.stage3;
            expected_child_beginning = complete_owner_set_digest(clock.owners())?;
            interrupt!(DirectSnowStage3V11InterruptionPostureV2::AfterSnowReappearance);
        }
    }
    if let Some(event) = solid_reappearance_event.as_ref()
        && clock.accepted_until() == prepared.support.start_ns()
    {
        if consumer.retains_accepted_publication_event_handoff(event) {
            return Err(DirectSnowStage3V11AttachmentError::Identity(
                "solid reappearance orphan publication event",
            ));
        }
        consumer.retain_accepted_publication_zero_duration_event_for_following_support(
            event,
            complete_owner_set_digest(beginning_clock.owners())?,
            prepared.support,
        )?;
    }
    let prepared = debited_prepared.unwrap_or_else(|| prepared.clone());
    if clock.accepted_until() == prepared.support.end_ns() && !pending_terminal_parcels.is_empty() {
        let endpoint = owner_joins
            .last()
            .ok_or(DirectSnowStage3V11AttachmentError::Identity(
                "resumed parent-end terminal receiver endpoint support",
            ))?;
        let terminal_group_index = event_groups.len().checked_sub(1).ok_or(
            DirectSnowStage3V11AttachmentError::Identity(
                "resumed parent-end terminal receiver event group",
            ),
        )?;
        interrupt!(DirectSnowStage3V11InterruptionPostureV2::BeforeTerminalReceiver);
        let terminal_group = event_groups.get_mut(terminal_group_index).ok_or(
            DirectSnowStage3V11AttachmentError::Identity(
                "resumed parent-end terminal receiver event group",
            ),
        )?;
        consume_parent_end_terminal_parcels_v1(
            context,
            &mut parent,
            &mut consumer,
            &mut clock,
            &stage3,
            &mut terminal_parcels,
            &mut pending_terminal_parcels,
            endpoint,
            terminal_group,
        )?;
        expected_child_beginning = complete_owner_set_digest(clock.owners())?;
        interrupt!(DirectSnowStage3V11InterruptionPostureV2::AfterTerminalReceiver);
        if failure_injection == Some(Stage3V11FailureInjection::ParentEndTerminalReceiverCompleted)
        {
            return Err(DirectSnowStage3V11AttachmentError::Identity(
                "injected parent-end terminal receiver rollback",
            ));
        }
    }
    Ok(AdaptiveParentInitializationOutcomeV1::Ready(Box::new(
        AdaptiveParentExecutionStateV1 {
            prepared,
            restart,
            parent,
            consumer,
            clock,
            stage3,
            owner_joins,
            event_groups,
            terminal_parcels,
            pending_terminal_parcels,
            expected_child_beginning,
            adaptive_receipts,
            snow_free_successor_receipts,
            adaptive_trial_quanta,
            covered_trial_memo: Vec::new(),
        },
    )))
}

#[allow(clippy::too_many_arguments)]
#[inline(never)]
fn finalize_adaptive_parent_execution_state_v1(
    context: &DirectSnowStage3V11StaticContext,
    failure_injection: Option<Stage3V11FailureInjection>,
    day_index: usize,
    interval_index: usize,
    parent_telemetry_started: std::time::Instant,
    execution: Box<AdaptiveParentExecutionStateV1>,
) -> Result<Box<AdaptiveSupportExecutionOutcomeV2>, DirectSnowStage3V11AttachmentError> {
    let AdaptiveParentExecutionStateV1 {
        prepared,
        restart: _,
        parent,
        consumer,
        clock,
        stage3,
        owner_joins,
        event_groups,
        terminal_parcels,
        pending_terminal_parcels: _,
        expected_child_beginning: _,
        adaptive_receipts,
        snow_free_successor_receipts,
        adaptive_trial_quanta: _,
        covered_trial_memo: _,
    } = *execution;
    finalize_adaptive_parent_execution_v1(
        context,
        failure_injection,
        &prepared,
        day_index,
        interval_index,
        parent_telemetry_started,
        parent,
        consumer,
        clock,
        stage3,
        owner_joins,
        event_groups,
        terminal_parcels,
        adaptive_receipts,
        snow_free_successor_receipts,
    )
}

#[allow(clippy::too_many_arguments, clippy::too_many_lines)]
#[inline(never)]
fn finalize_adaptive_parent_execution_v1(
    context: &DirectSnowStage3V11StaticContext,
    failure_injection: Option<Stage3V11FailureInjection>,
    prepared: &DirectSnowStage3V11PreparedSupport,
    day_index: usize,
    interval_index: usize,
    parent_telemetry_started: std::time::Instant,
    parent: V11ParentTransaction,
    mut consumer: DirectV10RealConsumerShadow,
    mut clock: CoupledClockStateV1,
    stage3: BTreeMap<u32, DirectSnowStage3PersistentState>,
    owner_joins: Vec<Stage3CoupledSubslabReceiptV1>,
    event_groups: Vec<Stage3V11TerminalEventGroupV1>,
    terminal_parcels: Vec<DirectSnowStage3V11TerminalParcel>,
    adaptive_receipts: AdaptiveReceiptAccumulatorV1,
    snow_free_successor_receipts: Vec<Stage3SnowFreeSuccessorReceiptV1>,
) -> Result<Box<AdaptiveSupportExecutionOutcomeV2>, DirectSnowStage3V11AttachmentError> {
    if failure_injection == Some(Stage3V11FailureInjection::FinalOwnerJoinCompleted) {
        return Err(DirectSnowStage3V11AttachmentError::Identity(
            "injected post-owner-join rollback",
        ));
    }
    let mut finalized = parent.clone().finalize(&context.vegetation_configuration)?;
    install_v11_parent_finalization_owner_transition(
        &mut clock,
        &mut consumer,
        &context.vegetation_configuration,
        &mut finalized,
    )?;
    let adaptive_support_receipt = adaptive_receipts.finalize(&clock, prepared.support)?;
    validate_adaptive_parent_publication_crossjoin_v1(
        &adaptive_support_receipt,
        &owner_joins,
        &snow_free_successor_receipts,
        &consumer,
        day_index,
    )?;
    if crate::snow_stage3_v11_attachment::adaptive_parent_telemetry_enabled_v1() {
        let transient_diagnostics = adaptive_support_receipt.transient_diagnostics()?;
        let parent_elapsed = parent_telemetry_started.elapsed();
        let (publication_support_count, publication_event_count) =
            consumer.adaptive_parent_telemetry_publication_shape_v1();
        let retained_complete_owner_bytes = consumer
            .canonical_owner_state_bytes()
            .ok()
            .map(|owners| owners.values().map(Vec::len).sum());
        let adaptive_receipt_bytes = serde_json::to_vec(&adaptive_support_receipt)
            .ok()
            .map(|bytes| bytes.len());
        let coupled_receipt_inline_bytes = std::mem::size_of_val(owner_joins.as_slice())
            + std::mem::size_of_val(event_groups.as_slice())
            + std::mem::size_of_val(terminal_parcels.as_slice());
        let mut accepted_widths = BTreeMap::<u128, u64>::new();
        for accepted in &adaptive_support_receipt.accepted_microsteps {
            let count = accepted_widths
                .entry(accepted.context.step_support.duration_ns())
                .or_default();
            *count = count.saturating_add(1);
        }
        if crate::snow_stage3_v11_attachment::record_adaptive_parent_telemetry_v1(
            crate::snow_stage3_v11_attachment::AdaptiveParentTelemetryV1 {
                parent_ordinal: interval_index,
                support: prepared.support,
                direct_trial_count: transient_diagnostics.direct_trial_count,
                split_child_trial_count: transient_diagnostics.split_child_trial_count,
                accepted_microstep_count: transient_diagnostics.accepted_microstep_count,
                rejected_candidate_count: transient_diagnostics.rejected_candidate_count,
                owner_join_count: owner_joins.len(),
                event_group_count: event_groups.len(),
                terminal_parcel_count: terminal_parcels.len(),
                publication_support_count,
                publication_event_count,
                adaptive_receipt_bytes,
                coupled_receipt_inline_bytes,
                retained_complete_owner_bytes,
                accepted_width_histogram: accepted_widths.into_iter().collect(),
                phase_rejection_count: 0,
                event_rejection_count: 0,
                phase_and_event_rejection_count: 0,
                other_rejection_count: 0,
                covered_direct_trial_phase_count: 0,
                covered_direct_trial_phase_elapsed: std::time::Duration::ZERO,
                covered_composed_trial_phase_count: 0,
                covered_composed_trial_phase_elapsed: std::time::Duration::ZERO,
                terminal_direct_trial_phase_count: 0,
                terminal_direct_trial_phase_elapsed: std::time::Duration::ZERO,
                terminal_composed_trial_phase_count: 0,
                terminal_composed_trial_phase_elapsed: std::time::Duration::ZERO,
                fixed_point_evaluation_count: 0,
                fixed_point_iteration_total: 0,
                fixed_point_iteration_maximum: 0,
                fixed_point_operand_elapsed: std::time::Duration::ZERO,
                fixed_point_envelope_elapsed: std::time::Duration::ZERO,
                provisional_envelope_projection_elapsed: std::time::Duration::ZERO,
                provisional_envelope_solver_ready_elapsed: std::time::Duration::ZERO,
                provisional_envelope_physical_elapsed: std::time::Duration::ZERO,
                provisional_envelope_receipts_elapsed: std::time::Duration::ZERO,
                provisional_envelope_owner_elapsed: std::time::Duration::ZERO,
                profile_detail: Default::default(),
                fixed_point_stage3_elapsed: std::time::Duration::ZERO,
                fixed_point_soil_elapsed: std::time::Duration::ZERO,
                fixed_point_finalization_elapsed: std::time::Duration::ZERO,
                publication_append_count: 0,
                publication_append_elapsed: std::time::Duration::ZERO,
                publication_cow_count: 0,
                publication_full_validation_count: 0,
                publication_full_validation_elapsed: std::time::Duration::ZERO,
                reuse_validation_count: 0,
                reuse_validation_elapsed: std::time::Duration::ZERO,
                reuse_hit_count: 0,
                reuse_fallback_count: 0,
                covered_child_memo_hit_count: 0,
                covered_child_memo_fallback_count: 0,
                covered_child_memo_direct_hit_count: 0,
                covered_child_memo_composed_hit_count: 0,
                parent_elapsed,
                cumulative_elapsed: std::time::Duration::ZERO,
            },
        ) {
            return Err(DirectSnowStage3V11AttachmentError::AdaptiveTelemetryStop);
        }
    }
    Ok(Box::new(AdaptiveSupportExecutionOutcomeV2::Complete((
        parent,
        consumer,
        clock,
        finalized,
        stage3,
        owner_joins,
        event_groups,
        terminal_parcels,
        adaptive_support_receipt,
        snow_free_successor_receipts,
    ))))
}
