/// Join the exact hydrology-selected event root back to the unpublished typed
/// carrier candidate that produced its six non-snow owners.  Root search may
/// evaluate and discard many previews, so installation is keyed exclusively
/// by the selected joint returned by the solver; provider call order is not
/// authority.
fn bind_exact_terminal_endpoint_candidate_v1(
    lane_id: u32,
    result: &crate::hydrology::DirectSnowStage3PersistentDayResult,
    candidates_by_joint: &BTreeMap<
        Digest32,
        crate::v9_real_consumer_shadow::CoveredCarrierEphemeralCandidatesV1,
    >,
) -> Result<
    crate::v9_real_consumer_shadow::CoveredCarrierEphemeralCandidatesV1,
    DirectSnowStage3V11AttachmentError,
> {
    let event =
        result
            .terminal_event
            .as_ref()
            .ok_or(DirectSnowStage3V11AttachmentError::Terminal(
                "exact covered endpoint missing terminal event",
            ))?;
    if !event.event_occurred
        || event.unevaluated_seconds.abs() > 1.0e-6
        || !result.state.layers.is_empty()
        || result.state.lane_id != lane_id
    {
        return Err(DirectSnowStage3V11AttachmentError::Terminal(
            "exact covered endpoint physical disposition",
        ));
    }
    bind_selected_terminal_candidate_v1(lane_id, result, candidates_by_joint)
}

/// Bind any selected discrete terminal-domain endpoint back to the one typed
/// carrier candidate whose six non-snow owner bytes match the hydrology join.
/// This is usable for preterminal/invalid exploration candidates; the exact
/// accepted-event wrapper above retains the stronger dormant-event checks.
fn bind_selected_terminal_candidate_v1(
    lane_id: u32,
    result: &crate::hydrology::DirectSnowStage3PersistentDayResult,
    candidates_by_joint: &BTreeMap<
        Digest32,
        crate::v9_real_consumer_shadow::CoveredCarrierEphemeralCandidatesV1,
    >,
) -> Result<
    crate::v9_real_consumer_shadow::CoveredCarrierEphemeralCandidatesV1,
    DirectSnowStage3V11AttachmentError,
> {
    let selected_joint = result.covered_terminal_ending_joint.as_ref().ok_or(
        DirectSnowStage3V11AttachmentError::Terminal("exact covered endpoint selected joint"),
    )?;
    let carrier = if let Some(step) = result.covered_terminal_accepted_microsteps.last() {
        if &step.hydrology_ending_joint != selected_joint || step.ending_state != result.state {
            return Err(DirectSnowStage3V11AttachmentError::Terminal(
                "exact covered endpoint accepted hydrology joint",
            ));
        }
        let exact = candidates_by_joint
            .get(&step.carrier_ending_joint.receipt_sha256())
            .cloned()
            .ok_or(DirectSnowStage3V11AttachmentError::Terminal(
                "exact covered endpoint accepted typed carrier",
            ))?;
        if exact
            .terminal_snow_soil_trial_receipt()
            .is_none_or(|receipt| receipt.lane_id != lane_id || receipt.support != step.support)
        {
            return Err(DirectSnowStage3V11AttachmentError::Terminal(
                "exact covered endpoint accepted snow-soil receipt",
            ));
        }
        exact
    } else if let Some(exact) = candidates_by_joint
        .get(&selected_joint.receipt_sha256())
        .cloned()
    {
        exact
    } else {
        let matching = candidates_by_joint
            .values()
            .filter(|candidate| {
                selected_joint
                    .owner_bytes()
                    .iter()
                    .all(|(owner_id, bytes)| {
                        owner_id == "snow"
                            || candidate
                                .joint()
                                .owner_bytes()
                                .get(owner_id)
                                .is_some_and(|candidate_bytes| candidate_bytes == bytes)
                    })
            })
            .cloned()
            .collect::<Vec<_>>();
        if matching.len() != 1 {
            return Err(DirectSnowStage3V11AttachmentError::Terminal(
                "exact covered endpoint typed carrier selection",
            ));
        }
        matching[0].clone()
    };
    let mut stage3 = carrier.stage3_by_lane().clone();
    if stage3.insert(lane_id, result.state.clone()).is_none() {
        return Err(DirectSnowStage3V11AttachmentError::Identity(
            "exact covered endpoint lane candidate",
        ));
    }
    carrier
        .try_with_selected_stage3_by_lane(selected_joint.clone(), stage3)
        .map_err(DirectSnowStage3V11AttachmentError::Owner)
}
