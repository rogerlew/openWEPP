#[allow(
    clippy::large_types_passed_by_value,
    clippy::too_many_arguments,
    clippy::too_many_lines
)]
fn execute_real_v11_parent(
    context: &DirectSnowStage3V11StaticContext,
    beginning_parent: &V11ParentTransaction,
    beginning_consumer: &DirectV10RealConsumerShadow,
    beginning_clock: &CoupledClockStateV1,
    prepared: &DirectSnowStage3V11PreparedSupport,
    day_index: usize,
    interval_index: usize,
    forcing_receipt: Digest32,
    ending_snow_owner_bytes: Vec<u8>,
    native_inactive_wb14_prefix: Option<crate::direct_runtime::ValidatedNativeInactiveWb14PrefixV1>,
    deferred_native_v2_soil_custody: Option<
        crate::v9_real_consumer_shadow::DeferredNativeV2SoilCustodyV1,
    >,
    finalize_parent_at_endpoint: bool,
) -> Result<
    (
        V11ParentTransaction,
        DirectV10RealConsumerShadow,
        CoupledClockStateV1,
        Option<V11ParentCandidate>,
        RealV11AcceptedSupportIdentityV1,
    ),
    DirectSnowStage3V11AttachmentError,
> {
    if beginning_parent.parent_transaction_id() != beginning_clock.parent_transaction_id()
        || beginning_clock.accepted_until() != prepared.support.start_ns()
        || prepared.support.start_ns() < beginning_clock.parent_support().start_ns()
        || prepared.support.end_ns() > beginning_clock.parent_support().end_ns()
        || beginning_clock.owners().len()
            != openwepp_vegetation::v11::V11_COMPLETE_OWNER_MANIFEST.len()
    {
        return Err(DirectSnowStage3V11AttachmentError::Identity(
            "V11/coupled-time parent beginning",
        ));
    }

    // The current released DirectV11RealConsumerStack is a snow-free lower
    // boundary.  A snow-covered interval is rejected here until the released
    // covered-boundary executor is available; it is never silently routed
    // through the snow-free branch.
    if prepared.v11_interval.lse_forcing.snow_present_at_beginning
        || prepared.v11_interval.lse_forcing.snow_present_at_end
        || prepared
            .v11_interval
            .lse_forcing
            .snow_terminal_payload_present
    {
        return Err(DirectSnowStage3V11AttachmentError::Owner(
            DirectV11RealConsumerError::Identity(
                "snow-covered V11 lower-boundary executor is not released",
            ),
        ));
    }

    let parent_id = beginning_parent.parent_transaction_id();
    let beginning_complete_owner_set_sha256 = complete_owner_set_digest(beginning_clock.owners())?;
    let beginning_snow_owner_sha256 = stage3_snow_owner_sha256_v1(beginning_clock.owners())?;
    let support = prepared.support;
    let start = support.start_ns();
    let end = support.end_ns();
    let constraint = StepConstraintV1::new(
        parent_id,
        start,
        end,
        "v11-real-consumer".to_owned(),
        ConstraintClass::HardBoundary,
        context.controller_policy,
        context.calendar_receipt,
        forcing_receipt,
    )?;
    let reduction = reduce_constraints(&[constraint], parent_id, start, end, None)?;
    let ledger_digest = complete_owner_set_digest(beginning_clock.owners())?;
    let mut ledger_preimage = Vec::new();
    ledger_preimage.extend_from_slice(parent_id.digest().as_bytes());
    ledger_preimage.extend_from_slice(&support.start_ns().get().to_be_bytes());
    ledger_preimage.extend_from_slice(&support.end_ns().get().to_be_bytes());
    let ledger = LedgerEntryV1::new(
        "complete-owner-custody".to_owned(),
        "canonical-owner-state".to_owned(),
        ledger_digest,
        ledger_digest,
        digest_bytes(&ledger_preimage),
    )?;
    let segment = beginning_clock.active_segment_id();

    // The coupled-time receipt includes the ending owner digest.  Obtain the
    // actual V11 ending owners with a provisional identity receipt, then rerun
    // the real V11 stack against the final receipt before accepting anything.
    let provisional_slab = CoupledSlabCandidateV1::new(
        beginning_clock,
        segment,
        support,
        &reduction,
        beginning_clock.owners().to_vec(),
        vec![ledger.clone()],
    )?;
    let mut provisional_clock = beginning_clock.clone();
    let provisional_receipt = accept_slab(&mut provisional_clock, provisional_slab)?;
    let provisional_parent = beginning_parent.clone();
    let provisional_stack = DirectV11RealConsumerStack::new_parent_child_with_ending_snow_owner(
        beginning_consumer,
        &prepared.v11_interval,
        day_index,
        interval_index,
        support.end_ns() == beginning_clock.parent_support().end_ns(),
        crate::direct_runtime::DirectWb14CoupledChildBindingV1 {
            proposed_upper_bound_s_bits: support.duration_s_bits(),
            coupled_parent_transaction_sha256: *parent_id.digest().as_bytes(),
            accepted_slab_sha256: *provisional_receipt.slab_id().digest().as_bytes(),
            parent_beginning_complete_owner_set_sha256: *ledger_digest.as_bytes(),
            parent_support_start_ns: beginning_clock.parent_support().start_ns().get(),
            parent_support_end_ns: beginning_clock.parent_support().end_ns().get(),
            child_support_start_ns: support.start_ns().get(),
            child_support_end_ns: support.end_ns().get(),
        },
        ending_snow_owner_bytes.clone(),
    );
    let provisional_stack = match native_inactive_wb14_prefix {
        Some(prefix) => provisional_stack
            .try_with_native_inactive_wb14_prefix(prefix)
            .map_err(DirectSnowStage3V11AttachmentError::Owner)?,
        None => provisional_stack,
    };
    let provisional_stack = match deferred_native_v2_soil_custody.as_ref() {
        Some(custody) => provisional_stack
            .try_with_deferred_native_v2_soil_custody(custody.clone())
            .map_err(DirectSnowStage3V11AttachmentError::Owner)?,
        None => provisional_stack,
    };
    let mut provisional_executor = crate::v11_vegetation_consumer::DirectV11VegetationExecutor {
        stack: provisional_stack,
    };
    let provisional_segment = execute_direct_v11_segment(
        &context.vegetation_configuration,
        &provisional_parent,
        &provisional_receipt,
        &mut provisional_executor,
    )?;
    let ending_owners = owner_states_from_envelopes(&provisional_segment.ending_resource_owners)?;
    let final_slab = CoupledSlabCandidateV1::new(
        beginning_clock,
        segment,
        support,
        &reduction,
        ending_owners,
        vec![ledger],
    )?;
    let mut final_clock = beginning_clock.clone();
    let final_receipt = accept_slab(&mut final_clock, final_slab)?;
    let accepted_support_identity = RealV11AcceptedSupportIdentityV1 {
        accepted_slab_sha256: final_receipt.slab_id().digest(),
        beginning_complete_owner_set_sha256,
        ending_complete_owner_set_sha256: complete_owner_set_digest(final_clock.owners())?,
        beginning_snow_owner_sha256,
        ending_snow_owner_sha256: stage3_snow_owner_sha256_v1(final_clock.owners())?,
    };
    // `prepare_snow_free_physical_reuse` consumes the private
    // `SnowFreePhysicalReuseSeedV1`; its final executor independently rejects
    // a `snow-free physical reuse ending owners` mismatch before publication.
    let final_stack = crate::v9_real_consumer_shadow::prepare_snow_free_physical_reuse(
        provisional_executor.stack,
        crate::direct_runtime::DirectWb14CoupledChildBindingV1 {
            proposed_upper_bound_s_bits: support.duration_s_bits(),
            coupled_parent_transaction_sha256: *parent_id.digest().as_bytes(),
            accepted_slab_sha256: *final_receipt.slab_id().digest().as_bytes(),
            parent_beginning_complete_owner_set_sha256: *ledger_digest.as_bytes(),
            parent_support_start_ns: beginning_clock.parent_support().start_ns().get(),
            parent_support_end_ns: beginning_clock.parent_support().end_ns().get(),
            child_support_start_ns: support.start_ns().get(),
            child_support_end_ns: support.end_ns().get(),
        },
    )
    .map_err(DirectSnowStage3V11AttachmentError::Owner)?;
    let mut final_executor =
        crate::v11_vegetation_consumer::DirectV11VegetationExecutor { stack: final_stack };
    let final_segment = execute_direct_v11_segment(
        &context.vegetation_configuration,
        beginning_parent,
        &final_receipt,
        &mut final_executor,
    )?;
    if final_segment.ending_resource_owners != provisional_segment.ending_resource_owners {
        return Err(DirectSnowStage3V11AttachmentError::Identity(
            "V11 ending owner fixed point",
        ));
    }
    let mut parent = beginning_parent.clone();
    accept_direct_v11_segment(
        &mut parent,
        &context.vegetation_configuration,
        final_segment,
        beginning_consumer,
    )?;
    let mut consumer = final_executor
        .stack
        .commit_selected_publication_and_take_staged_ending()
        .map_err(DirectSnowStage3V11AttachmentError::Owner)?;
    let finalized = if finalize_parent_at_endpoint {
        if final_clock.accepted_until() != final_clock.parent_support().end_ns() {
            return Err(DirectSnowStage3V11AttachmentError::Identity(
                "snow-free V11 parent finalization before endpoint",
            ));
        }
        let (mut finalized, finalized_handoff) = parent
            .clone()
            .finalize_with_validated_handoff(&context.vegetation_configuration)?;
        install_v11_parent_finalization_owner_transition_with_validated_handoff(
            &mut final_clock,
            &mut consumer,
            &context.vegetation_configuration,
            &mut finalized,
            finalized_handoff,
        )?;
        Some(finalized)
    } else {
        None
    };
    Ok((
        parent,
        consumer,
        final_clock,
        finalized,
        accepted_support_identity,
    ))
}
