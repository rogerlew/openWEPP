fn apply_actual_terminal_group(
    context: &DirectSnowStage3V11StaticContext,
    mut parent: V11ParentTransaction,
    mut clock: CoupledClockStateV1,
    stage3: BTreeMap<u32, DirectSnowStage3PersistentState>,
    beginning_terminal_parcels: &BTreeMap<Digest32, DirectSnowStage3V11TerminalParcel>,
    physical_child_ordinal: u32,
    group: &mut Stage3V11TerminalEventGroupV1,
    endpoint_lane_receipts: &BTreeMap<u32, LaneStage3BoundaryReceiptV1>,
    endpoint_tile_receipts: &BTreeMap<(OfeId, TileId), FinalStage3TileBoundaryReceiptV1>,
) -> Result<
    (
        V11ParentTransaction,
        CoupledClockStateV1,
        BTreeMap<u32, DirectSnowStage3PersistentState>,
        Vec<DirectSnowStage3V11TerminalParcel>,
        AcceptedEventReceiptV1,
    ),
    DirectSnowStage3V11AttachmentError,
> {
    if clock.accepted_until() != group.tick || u64::from(clock.event_ordinal()) != group.ordinal {
        return Err(DirectSnowStage3V11AttachmentError::Terminal(
            "terminal event cursor or ordinal",
        ));
    }
    let mut parcels = Vec::new();
    let proposal_core = terminal_event_proposal_core(
        &context.surface_liquid_configuration,
        group,
        clock.parent_transaction_id().digest(),
        clock.parent_support(),
        physical_child_ordinal,
    )?;
    for candidate in &group.candidates {
        let terminal =
            stage3
                .get(&candidate.lane_id)
                .ok_or(DirectSnowStage3V11AttachmentError::Identity(
                    "terminal event lane owner",
                ))?;
        let candidate_parcels = terminal_parcels_for_event_group(
            &context.surface_liquid_configuration,
            candidate,
            group,
            clock.parent_transaction_id().digest(),
            proposal_core,
        )?;
        parcels.extend(candidate_parcels);
        if terminal.layers.iter().any(|layer| layer.thickness_m > 0.0)
            || terminal.detached_retained_liquid_kg_m2 != 0.0
        {
            return Err(DirectSnowStage3V11AttachmentError::Terminal(
                "terminal endpoint is not dormant",
            ));
        }
    }
    let mut ending_terminal_parcels = beginning_terminal_parcels.clone();
    for parcel in &parcels {
        if ending_terminal_parcels
            .insert(parcel.parcel_digest, parcel.clone())
            .is_some()
        {
            return Err(DirectSnowStage3V11AttachmentError::Terminal(
                "duplicate pending terminal parcel",
            ));
        }
    }
    let ending_snow_bytes = canonical_stage3_snow_owner_bytes_with_pending_and_receipts(
        &stage3,
        &ending_terminal_parcels,
        endpoint_lane_receipts,
        endpoint_tile_receipts,
    )?;
    let ending_owners = clock
        .owners()
        .iter()
        .map(|owner| {
            if owner.owner_id() == "snow" {
                OwnerState::new("snow".to_owned(), ending_snow_bytes.clone())
            } else {
                Ok(owner.clone())
            }
        })
        .collect::<Result<Vec<_>, _>>()?;
    group.proposal_core_sha256 = Some(proposal_core);
    let mut parcel_digests = parcels
        .iter()
        .map(|parcel| parcel.parcel_digest)
        .collect::<Vec<_>>();
    parcel_digests.sort_unstable();
    if parcel_digests.len() != group.candidates.len()
        || parcel_digests.windows(2).any(|pair| pair[0] == pair[1])
    {
        return Err(DirectSnowStage3V11AttachmentError::Terminal(
            "terminal produced parcel identity set",
        ));
    }
    group
        .produced_unconsumed_parcel_digests
        .clone_from(&parcel_digests);
    group.produced_unconsumed_parcels = parcels
        .iter()
        .map(|parcel| Stage3V11TerminalReceiverCustodyV1 {
            support: parcel.support,
            source_lane_id: parcel.source_lane_id,
            parent_transaction_id: parcel.parent_transaction_id,
            event_ordinal: parcel.event_ordinal,
            terminal_event_proposal_core_id: parcel.terminal_event_proposal_core_id,
            event_result_digest: parcel.event_result_digest,
            receiver_topology_sha256: parcel.receiver_topology_sha256,
            destination_ofe_id: parcel.destination_ofe_id.clone(),
            receiver_destinations: parcel
                .receiver_destinations
                .iter()
                .map(
                    |destination| Stage3V11TerminalReceiverDestinationCustodyV1 {
                        destination_ofe_id: destination.destination_ofe_id.clone(),
                        destination_tile_id: destination.destination_tile_id.clone(),
                        destination_fraction: destination.destination_fraction,
                    },
                )
                .collect(),
            mass_kg_m2_tile_ground: parcel.mass_kg_m2_tile_ground,
            temperature_k: parcel.temperature_k,
            specific_liquid_enthalpy_j_kg: parcel.specific_liquid_enthalpy_j_kg,
            parcel_digest: parcel.parcel_digest,
        })
        .collect();
    let parcel_fields = parcel_digests
        .iter()
        .map(|digest| FramedField {
            tag: "parcel",
            value: digest.as_bytes(),
        })
        .collect::<Vec<_>>();
    let parcel_set = framed_sha256("stage3-v11-terminal-parcel-set", &parcel_fields)?;
    let schema = 1_u32.to_be_bytes();
    let child = physical_child_ordinal.to_be_bytes();
    let ordinal = u32::try_from(group.ordinal)
        .map_err(|_| DirectSnowStage3V11AttachmentError::Identity("terminal event ordinal width"))?
        .to_be_bytes();
    let search = group.candidates[0].support;
    let mutations = b"\0\0\0\x04snow";
    let mut candidate_members = u32::try_from(group.candidates.len())
        .map_err(|_| DirectSnowStage3V11AttachmentError::Identity("terminal candidate count"))?
        .to_be_bytes()
        .to_vec();
    for candidate in &group.candidates {
        let mut member = Vec::new();
        member.extend_from_slice(&candidate.lane_id.to_be_bytes());
        member.extend_from_slice(candidate.event_result_digest.as_bytes());
        member.extend_from_slice(candidate.terminal_state_sha256.as_bytes());
        member.extend_from_slice(
            &candidate
                .event
                .terminal_liquid_kg_m2
                .to_bits()
                .to_be_bytes(),
        );
        member.extend_from_slice(
            &candidate
                .event
                .terminal_unallocated_energy_j_m2
                .to_bits()
                .to_be_bytes(),
        );
        member.extend_from_slice(parcel_set.as_bytes());
        candidate_members.extend_from_slice(&(member.len() as u32).to_be_bytes());
        candidate_members.extend_from_slice(&member);
    }
    group.receipt_sha256 = framed_sha256(
        "stage3-v11-terminal-group-preaccept",
        &[
            FramedField {
                tag: "schema",
                value: &schema,
            },
            FramedField {
                tag: "proposal_core",
                value: proposal_core.as_bytes(),
            },
            FramedField {
                tag: "parent_transaction",
                value: clock.parent_transaction_id().digest().as_bytes(),
            },
            FramedField {
                tag: "enclosing_start",
                value: &clock.parent_support().start_ns().get().to_be_bytes(),
            },
            FramedField {
                tag: "enclosing_end",
                value: &clock.parent_support().end_ns().get().to_be_bytes(),
            },
            FramedField {
                tag: "search_start",
                value: &search.start_ns().get().to_be_bytes(),
            },
            FramedField {
                tag: "search_end",
                value: &search.end_ns().get().to_be_bytes(),
            },
            FramedField {
                tag: "event_tick",
                value: &group.tick.get().to_be_bytes(),
            },
            FramedField {
                tag: "child_ordinal",
                value: &child,
            },
            FramedField {
                tag: "event_ordinal",
                value: &ordinal,
            },
            FramedField {
                tag: "forcing",
                value: group.candidates[0].shortened_forcing_sha256.as_bytes(),
            },
            FramedField {
                tag: "topology",
                value: parcels[0].receiver_topology_sha256.as_bytes(),
            },
            FramedField {
                tag: "begin_owner_set",
                value: complete_owner_set_digest(clock.owners())?.as_bytes(),
            },
            FramedField {
                tag: "proposed_end_owner_set",
                value: complete_owner_set_digest(&ending_owners)?.as_bytes(),
            },
            FramedField {
                tag: "mutations",
                value: mutations,
            },
            FramedField {
                tag: "candidates",
                value: &candidate_members,
            },
        ],
    )?;
    let ledger = LedgerEntryV1::new(
        "terminal-snow-liquid-custody".to_owned(),
        "kg-m-2-ofe-ground".to_owned(),
        parcel_set,
        parcel_set,
        group.receipt_sha256,
    )?;
    let mut participants = clock
        .active_participants()
        .iter()
        .filter(|value| !value.starts_with("stage3-lane-"))
        .cloned()
        .collect::<Vec<_>>();
    participants.extend(
        group
            .post_active_lanes
            .iter()
            .map(|lane| format!("stage3-lane-{lane}")),
    );
    participants.sort();
    participants.dedup();
    let event = EventProposalV1::new(
        EventClass::OwnershipTransfer,
        "snow".to_owned(),
        group.receipt_sha256,
        ending_owners.clone(),
        vec!["snow".to_owned()],
        if group.post_active_lanes.is_empty() {
            "snow-free".to_owned()
        } else {
            "snow-stage3-v11-mixed".to_owned()
        },
        participants,
        vec![ledger],
    )?;
    let mut queue = EventQueueV1::new(group.tick, vec![event])?;
    let accepted_event_receipt =
        queue
            .apply_next(&mut clock)?
            .ok_or(DirectSnowStage3V11AttachmentError::Terminal(
                "terminal event application",
            ))?;
    if queue.apply_next(&mut clock)?.is_some() {
        return Err(DirectSnowStage3V11AttachmentError::Terminal(
            "terminal event queue cardinality",
        ));
    }
    let installed_snow = clock
        .owners()
        .iter()
        .find(|owner| owner.owner_id() == "snow")
        .ok_or(DirectSnowStage3V11AttachmentError::Identity(
            "terminal installed V4 snow owner",
        ))?;
    if installed_snow.state_bytes() != ending_snow_bytes {
        return Err(DirectSnowStage3V11AttachmentError::Identity(
            "terminal zero-event V4 owner installation",
        ));
    }
    parent.accept_zero_duration_owner_transition(
        &context.vegetation_configuration,
        group.tick,
        owner_envelopes_from_states(&ending_owners)?,
        &["snow".to_owned()],
    )?;
    let evaluated_seconds = group
        .candidates
        .first()
        .ok_or(DirectSnowStage3V11AttachmentError::Identity(
            "terminal physical ledger candidate",
        ))?
        .event
        .evaluated_seconds;
    if group.candidates.iter().any(|candidate| {
        candidate.event.evaluated_seconds.to_bits() != evaluated_seconds.to_bits()
            || candidate.support != search
    }) {
        return Err(DirectSnowStage3V11AttachmentError::Identity(
            "terminal physical ledger evaluated support",
        ));
    }
    let snow_soil_heat_j_m2 = group
        .candidates
        .iter()
        .map(|candidate| candidate.event.snow_soil_heat_energy_j_m2)
        .sum::<f64>();
    group.terminal_physical_ledger = Some(
        Stage3V11TerminalPhysicalLedgerV1 {
            support: search,
            event_result_set_sha256: terminal_event_result_set_digest(&group.candidates)?,
            proposal_core_sha256: proposal_core,
            accepted_event_receipt_sha256: accepted_event_receipt.id().digest(),
            accepted_event_ledger_sha256: accepted_event_receipt.ledger_digest(),
            produced_unconsumed_parcel_set_sha256: parcel_set,
            beginning_owner_set_sha256: accepted_event_receipt.beginning_owner_set_digest(),
            ending_owner_set_sha256: accepted_event_receipt.ending_owner_set_digest(),
            ending_snow_owner_sha256: digest_bytes(&ending_snow_bytes),
            evaluated_seconds,
            snow_soil_heat_j_m2,
            receipt_sha256: Digest32::zero(),
        }
        .seal()?,
    );
    Ok((parent, clock, stage3, parcels, accepted_event_receipt))
}
