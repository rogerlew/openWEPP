fn terminal_carrier_phase_chain_identity_v1(
    phases: &[crate::v9_real_consumer_shadow::CoveredCarrierPhaseResultV1],
) -> Result<Digest32, DirectSnowStage3V11AttachmentError> {
    if phases.is_empty() {
        return Err(DirectSnowStage3V11AttachmentError::Terminal(
            "empty terminal carrier phase chain",
        ));
    }
    let mut bytes = Vec::new();
    bytes.extend_from_slice(b"OPENWEPP_STAGE3_V11_TERMINAL_CARRIER_PHASE_CHAIN_V1\0");
    for phase in phases {
        let support = phase.transition.boundary.support;
        bytes.extend_from_slice(&support.start_ns().get().to_be_bytes());
        bytes.extend_from_slice(&support.end_ns().get().to_be_bytes());
        bytes.extend_from_slice(
            phase
                .transition
                .probe_child_identity
                .receipt_sha256
                .as_bytes(),
        );
        bytes.extend_from_slice(phase.ending_candidates.joint().receipt_sha256().as_bytes());
        bytes.extend_from_slice(phase.wb14_child_receipt_set_sha256.as_bytes());
        for (lane_id, set) in &phase.precipitation_sets {
            bytes.extend_from_slice(&lane_id.to_be_bytes());
            bytes.extend_from_slice(set.receipt_sha256.as_bytes());
        }
    }
    Ok(digest_bytes(&bytes))
}

fn precomputed_terminal_package_v1(
    endpoints: &[Box<ExactCoveredTerminalEndpointV1>],
    pending: &BTreeMap<Digest32, DirectSnowStage3V11TerminalParcel>,
    accepted_slab_sha256: Digest32,
    beginning_owner_set_sha256: Digest32,
) -> Result<
    crate::v9_real_consumer_shadow::PrecomputedTerminalAcceptedEndpointV1,
    DirectSnowStage3V11AttachmentError,
> {
    let first = endpoints
        .first()
        .ok_or(DirectSnowStage3V11AttachmentError::Terminal(
            "empty exact terminal endpoint group",
        ))?;
    let first_non_snow = first
        .ending
        .joint()
        .owner_bytes()
        .iter()
        .filter(|(owner, _)| owner.as_str() != "snow")
        .map(|(owner, bytes)| (owner.clone(), bytes.clone()))
        .collect::<BTreeMap<_, _>>();
    let mut ending_stage3 = first.ending.stage3_by_lane().clone();
    let mut events = BTreeMap::new();
    let mut final_child_actual_vapor_by_lane = BTreeMap::new();
    let trials = first
        .carrier_phase
        .batch_terminal_snow_soil_trial_receipts_by_lane
        .clone();
    let trial_chains = first.terminal_snow_soil_trial_receipt_chains_by_lane.clone();
    if trials.is_empty() {
        return Err(DirectSnowStage3V11AttachmentError::Terminal(
            "coalesced terminal snow-soil trial set",
        ));
    }
    for endpoint in endpoints {
        let non_snow = endpoint
            .ending
            .joint()
            .owner_bytes()
            .iter()
            .filter(|(owner, _)| owner.as_str() != "snow")
            .map(|(owner, bytes)| (owner.clone(), bytes.clone()))
            .collect::<BTreeMap<_, _>>();
        if endpoint.support != first.support
            || non_snow != first_non_snow
            || terminal_carrier_phase_chain_identity_v1(&endpoint.carrier_phase_chain)?
                != terminal_carrier_phase_chain_identity_v1(&first.carrier_phase_chain)?
            || trials.get(&endpoint.lane_id) != Some(&endpoint.terminal_snow_soil_trial_receipt)
            || trial_chains.get(&endpoint.lane_id)
                != endpoint
                    .terminal_snow_soil_trial_receipt_chains_by_lane
                    .get(&endpoint.lane_id)
            || endpoint.wb14_replay_trial_sha256 != first.wb14_replay_trial_sha256
            || endpoint.wb14_replay_beginning_owner_set_sha256
                != first.wb14_replay_beginning_owner_set_sha256
        {
            return Err(DirectSnowStage3V11AttachmentError::Terminal(
                "coalesced exact terminal carrier divergence",
            ));
        }
        if endpoint.event.event_occurred
            && events.insert(endpoint.lane_id, endpoint.event).is_some()
        {
            return Err(DirectSnowStage3V11AttachmentError::Terminal(
                "coalesced exact terminal event duplication",
            ));
        }
        if endpoint.event.event_occurred
            && final_child_actual_vapor_by_lane
                .insert(
                    endpoint.lane_id,
                    endpoint.final_child_actual_vapor_to_canopy_air_kg_m2,
                )
                .is_some()
        {
            return Err(DirectSnowStage3V11AttachmentError::Terminal(
                "coalesced terminal final-child vapor duplication",
            ));
        }
        let dormant = endpoint
            .ending
            .stage3_by_lane()
            .get(&endpoint.lane_id)
            .ok_or(DirectSnowStage3V11AttachmentError::Identity(
                "coalesced dormant lane",
            ))?;
        ending_stage3.insert(endpoint.lane_id, dormant.clone());
    }
    let mut package = crate::v9_real_consumer_shadow::PrecomputedTerminalAcceptedEndpointV1 {
        carrier_phase: first.carrier_phase.clone(),
        carrier_phase_chain: first.carrier_phase_chain.clone(),
        ending_stage3_by_lane: ending_stage3,
        terminal_events: events,
        final_child_actual_vapor_to_canopy_air_kg_m2_by_lane:
            final_child_actual_vapor_by_lane,
        terminal_snow_soil_trial_receipts: trials,
        terminal_snow_soil_trial_receipt_chains_by_lane: trial_chains,
        beginning_pending_terminal_parcels: pending.clone(),
        accepted_envelope_support: first.support,
        accepted_slab_sha256,
        beginning_owner_set_sha256,
        wb14_replay_trial_sha256: first.wb14_replay_trial_sha256,
        wb14_replay_beginning_owner_set_sha256: first.wb14_replay_beginning_owner_set_sha256,
        wb14_child_receipt_set_sha256: parse_lower_hex_digest(
            &first.carrier_phase.wb14_child_receipt_set_sha256,
        )?,
        wb14_parent_receipt_set_sha256: first
            .carrier_phase
            .wb14_parent_receipt_set_sha256
            .as_deref()
            .map(parse_lower_hex_digest)
            .transpose()?,
        pre_event_authority_sha256: Digest32::zero(),
    };
    package.pre_event_authority_sha256 =
        crate::v9_real_consumer_shadow::precomputed_terminal_pre_event_authority_sha256_v1(
            &package,
        )?;
    #[cfg(test)]
    TERMINAL_PRE_EVENT_PACKAGE_AUDIT.with(|audit| {
        if let Some(packages) = audit.borrow_mut().as_mut() {
            packages.push(package.clone());
        }
    });
    Ok(package)
}
#[derive(Clone)]
struct ExactCoveredTerminalEndpointV1 {
    support: TimeSupport,
    lane_id: u32,
    event: DirectSnowTerminalEventResult,
    event_result_digest: Digest32,
    forcing_sha256: Digest32,
    ending: crate::v9_real_consumer_shadow::CoveredCarrierEphemeralCandidatesV1,
    carrier_phase: Box<crate::v9_real_consumer_shadow::CoveredCarrierPhaseResultV1>,
    carrier_phase_chain: Vec<crate::v9_real_consumer_shadow::CoveredCarrierPhaseResultV1>,
    wb14_replay_trial_sha256: Digest32,
    wb14_replay_beginning_owner_set_sha256: Digest32,
    terminal_snow_soil_trial_receipt:
        crate::v9_real_consumer_shadow::TerminalSnowSoilTrialReceiptV1,
    final_child_actual_vapor_to_canopy_air_kg_m2: f64,
    terminal_snow_soil_trial_receipt_chains_by_lane: BTreeMap<
        u32,
        Vec<crate::v9_real_consumer_shadow::TerminalSnowSoilTrialReceiptV1>,
    >,
    endpoint_receipt_sha256: Digest32,
}
