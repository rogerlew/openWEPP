enum AdaptiveCandidateSelectionV1<T> {
    Accepted {
        trial: Box<T>,
        maximum_scaled_error: f64,
    },
    Refine {
        next_trial_quanta: u128,
    },
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
