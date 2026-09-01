#[inline(never)]
fn select_exact_terminal_carrier_phase_v1(
    accepted_terminal_microstep: Option<&crate::hydrology::CoveredTerminalAcceptedMicrostepV1>,
    ending: &crate::v9_real_consumer_shadow::CoveredCarrierEphemeralCandidatesV1,
    carrier_phases_by_joint: &BTreeMap<
        Digest32,
        crate::v9_real_consumer_shadow::CoveredCarrierPhaseResultV1,
    >,
) -> Result<
    Box<crate::v9_real_consumer_shadow::CoveredCarrierPhaseResultV1>,
    DirectSnowStage3V11AttachmentError,
> {
    let carrier_phase = if let Some(step) = accepted_terminal_microstep {
        carrier_phases_by_joint
            .get(&step.carrier_ending_joint.receipt_sha256())
            .cloned()
            .ok_or(DirectSnowStage3V11AttachmentError::Terminal(
                "exact endpoint accepted carrier phase",
            ))?
    } else if let Some(exact) = carrier_phases_by_joint
        .get(&ending.joint().receipt_sha256())
        .cloned()
    {
        exact
    } else {
        let matching = carrier_phases_by_joint
            .values()
            .filter(|phase| {
                ending
                    .joint()
                    .owner_bytes()
                    .iter()
                    .all(|(owner_id, bytes)| {
                        owner_id == "snow"
                            || phase
                                .ending_candidates
                                .joint()
                                .owner_bytes()
                                .get(owner_id)
                                .is_some_and(|candidate| candidate == bytes)
                    })
            })
            .collect::<Vec<_>>();
        if matching.len() != 1 {
            return Err(DirectSnowStage3V11AttachmentError::Terminal(
                "exact endpoint converged carrier value evidence",
            ));
        }
        matching[0].clone()
    };
    Ok(Box::new(carrier_phase))
}

#[inline(never)]
fn prepare_exact_terminal_trial_chains_v1(
    discovery: &Stage3V11ActualTerminalCandidateV1,
    exact_result: &crate::hydrology::DirectSnowStage3PersistentDayResult,
    carrier_phase: &crate::v9_real_consumer_shadow::CoveredCarrierPhaseResultV1,
    carrier_phases_by_joint: &BTreeMap<
        Digest32,
        crate::v9_real_consumer_shadow::CoveredCarrierPhaseResultV1,
    >,
) -> Result<
    BTreeMap<u32, Vec<crate::v9_real_consumer_shadow::TerminalSnowSoilTrialReceiptV1>>,
    DirectSnowStage3V11AttachmentError,
> {
    let mut trial_chains_by_lane = BTreeMap::new();
    if exact_result.covered_terminal_accepted_microsteps.is_empty() {
        for (lane_id, receipt) in &carrier_phase.batch_terminal_snow_soil_trial_receipts_by_lane {
            trial_chains_by_lane.insert(*lane_id, vec![receipt.clone()]);
        }
    } else {
        for step in &exact_result.covered_terminal_accepted_microsteps {
            let phase = carrier_phases_by_joint
                .get(&step.carrier_ending_joint.receipt_sha256())
                .ok_or(DirectSnowStage3V11AttachmentError::Terminal(
                    "exact endpoint accepted carrier trial chain",
                ))?;
            for (lane_id, receipt) in &phase.batch_terminal_snow_soil_trial_receipts_by_lane {
                receipt.validate().map_err(|_| {
                    DirectSnowStage3V11AttachmentError::Terminal(
                        "exact endpoint accepted carrier trial chain seal",
                    )
                })?;
                if receipt.support != step.support || receipt.lane_id != *lane_id {
                    return Err(DirectSnowStage3V11AttachmentError::Terminal(
                        "exact endpoint accepted carrier trial chain support",
                    ));
                }
                trial_chains_by_lane
                    .entry(*lane_id)
                    .or_insert_with(Vec::new)
                    .push(receipt.clone());
            }
        }
    }
    for (lane_id, chain) in &trial_chains_by_lane {
        let final_receipt = carrier_phase
            .batch_terminal_snow_soil_trial_receipts_by_lane
            .get(lane_id)
            .ok_or(DirectSnowStage3V11AttachmentError::Terminal(
                "exact endpoint final carrier trial lane",
            ))?;
        let chain_covers_envelope = chain
            .first()
            .is_some_and(|receipt| receipt.support.start_ns() == discovery.support.start_ns())
            && chain.last().is_some_and(|receipt| {
                receipt.support.end_ns() == discovery.support.end_ns() && receipt == final_receipt
            })
            && chain
                .windows(2)
                .all(|pair| pair[0].support.end_ns() == pair[1].support.start_ns());
        if !chain_covers_envelope {
            return Err(DirectSnowStage3V11AttachmentError::Terminal(
                "exact endpoint accepted carrier trial chain coverage",
            ));
        }
    }
    Ok(trial_chains_by_lane)
}

#[inline(never)]
fn prepare_exact_terminal_phase_chain_v1(
    exact_result: &crate::hydrology::DirectSnowStage3PersistentDayResult,
    carrier_phase: Box<crate::v9_real_consumer_shadow::CoveredCarrierPhaseResultV1>,
    carrier_phases_by_joint: &BTreeMap<
        Digest32,
        crate::v9_real_consumer_shadow::CoveredCarrierPhaseResultV1,
    >,
) -> Result<
    (
        Box<crate::v9_real_consumer_shadow::CoveredCarrierPhaseResultV1>,
        Vec<crate::v9_real_consumer_shadow::CoveredCarrierPhaseResultV1>,
        Digest32,
    ),
    DirectSnowStage3V11AttachmentError,
> {
    let wb14_replay_trial_sha256 = carrier_phase.transition.probe_child_identity.receipt_sha256;
    let mut carrier_phase_chain = if exact_result.covered_terminal_accepted_microsteps.is_empty() {
        vec![(*carrier_phase).clone()]
    } else {
        exact_result
            .covered_terminal_accepted_microsteps
            .iter()
            .map(|step| {
                carrier_phases_by_joint
                    .get(&step.carrier_ending_joint.receipt_sha256())
                    .cloned()
                    .ok_or(DirectSnowStage3V11AttachmentError::Terminal(
                        "exact endpoint accepted carrier phase chain",
                    ))
            })
            .collect::<Result<Vec<_>, _>>()?
    };
    let accepted_child_count = u32::try_from(
        exact_result
            .covered_terminal_accepted_microsteps
            .len()
            .max(1),
    )
    .map_err(|_| {
        DirectSnowStage3V11AttachmentError::Identity("exact endpoint physical-child count width")
    })?;
    let first_physical_child_ordinal = carrier_phase
        .transition
        .probe_child_identity
        .physical_child_ordinal;
    for (index, phase) in carrier_phase_chain.iter_mut().enumerate() {
        let prior_child = &phase.transition.probe_child_identity;
        let physical_child_ordinal = first_physical_child_ordinal
            .checked_add(u32::try_from(index).map_err(|_| {
                DirectSnowStage3V11AttachmentError::Identity(
                    "exact endpoint physical-child ordinal width",
                )
            })?)
            .ok_or(DirectSnowStage3V11AttachmentError::Identity(
                "exact endpoint physical-child ordinal overflow",
            ))?;
        phase.transition.probe_child_identity =
            CoveredProbeChildIdentityV1::try_new(ProbeChildAuthorityV1 {
                parent_transaction_sha256: prior_child.parent_transaction_sha256,
                enclosing_parent_support: prior_child.enclosing_parent_support,
                trial_support: phase.transition.boundary.support,
                physical_child_ordinal,
                attempt_ordinal: prior_child.attempt_ordinal,
                role: prior_child.role,
                beginning_joint_sha256: prior_child.beginning_joint_sha256,
                beginning_owner_set_sha256: prior_child.beginning_owner_set_sha256,
                complete_forcing_sha256: prior_child.complete_forcing_sha256,
                topology_sha256: prior_child.topology_sha256,
            })?;
    }
    if u32::try_from(carrier_phase_chain.len()).ok() != Some(accepted_child_count) {
        return Err(DirectSnowStage3V11AttachmentError::Identity(
            "exact endpoint physical-child chain count",
        ));
    }
    let final_carrier_phase = Box::new(carrier_phase_chain.last().cloned().ok_or(
        DirectSnowStage3V11AttachmentError::Terminal("exact endpoint empty carrier phase chain"),
    )?);
    Ok((
        final_carrier_phase,
        carrier_phase_chain,
        wb14_replay_trial_sha256,
    ))
}
