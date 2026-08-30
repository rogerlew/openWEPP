#[derive(Clone, Debug, PartialEq, serde::Deserialize, Serialize)]
pub struct DirectSnowStage3V11TerminalReceipt {
    pub lane_id: u32,
    pub support: TimeSupport,
    pub result: DirectSnowTerminalEventResult,
    pub candidate_ticks: Vec<ModelTimeNs>,
    pub accepted_event_tick: ModelTimeNs,
}

#[derive(Clone, Debug, PartialEq, serde::Deserialize, Serialize)]
pub struct DirectSnowStage3V11TerminalReceiverDestinationV1 {
    pub destination_ofe_id: String,
    pub destination_tile_id: String,
    pub destination_fraction: f64,
}

#[derive(Clone, Debug, PartialEq, serde::Deserialize, Serialize)]
pub struct DirectSnowStage3V11TerminalParcel {
    pub support: TimeSupport,
    pub source_lane_id: u32,
    pub parent_transaction_id: Digest32,
    pub event_ordinal: u32,
    pub terminal_event_proposal_core_id: Digest32,
    pub event_result_digest: Digest32,
    pub receiver_topology_sha256: Digest32,
    pub destination_ofe_id: String,
    /// Complete canonical receiver partition. One logical terminal parcel is
    /// owned per terminating lane; these rows project that one OFE-ground
    /// amount uniformly to every configured tile without multiplying custody.
    pub receiver_destinations: Vec<DirectSnowStage3V11TerminalReceiverDestinationV1>,
    pub mass_kg_m2_tile_ground: f64,
    pub temperature_k: f64,
    pub specific_liquid_enthalpy_j_kg: f64,
    pub posture: DirectSnowStage3V11TerminalParcelPosture,
    pub parcel_digest: Digest32,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, serde::Deserialize, Serialize)]
pub enum DirectSnowStage3V11TerminalParcelPosture {
    ProducedUnconsumed,
    Consumed,
}

fn validate_retained_terminal_receiver_custody_v1(
    group: &Stage3V11TerminalEventGroupV1,
) -> Result<(), DirectSnowStage3V11AttachmentError> {
    let accepted = group.accepted_event_receipt.as_ref().ok_or(
        DirectSnowStage3V11AttachmentError::Identity("terminal receiver custody accepted event"),
    )?;
    let mut digests = Vec::with_capacity(group.produced_unconsumed_parcels.len());
    for custody in &group.produced_unconsumed_parcels {
        let candidate = group
            .candidates
            .iter()
            .find(|candidate| candidate.lane_id == custody.source_lane_id)
            .ok_or(DirectSnowStage3V11AttachmentError::Identity(
                "terminal receiver custody candidate",
            ))?;
        let destinations = custody
            .receiver_destinations
            .iter()
            .map(
                |destination| DirectSnowStage3V11TerminalReceiverDestinationV1 {
                    destination_ofe_id: destination.destination_ofe_id.clone(),
                    destination_tile_id: destination.destination_tile_id.clone(),
                    destination_fraction: destination.destination_fraction,
                },
            )
            .collect::<Vec<_>>();
        let fraction_sum = destinations
            .iter()
            .map(|destination| destination.destination_fraction)
            .sum::<f64>();
        let parcel = DirectSnowStage3V11TerminalParcel {
            support: custody.support,
            source_lane_id: custody.source_lane_id,
            parent_transaction_id: custody.parent_transaction_id,
            event_ordinal: custody.event_ordinal,
            terminal_event_proposal_core_id: custody.terminal_event_proposal_core_id,
            event_result_digest: custody.event_result_digest,
            receiver_topology_sha256: custody.receiver_topology_sha256,
            destination_ofe_id: custody.destination_ofe_id.clone(),
            receiver_destinations: destinations,
            mass_kg_m2_tile_ground: custody.mass_kg_m2_tile_ground,
            temperature_k: custody.temperature_k,
            specific_liquid_enthalpy_j_kg: custody.specific_liquid_enthalpy_j_kg,
            posture: DirectSnowStage3V11TerminalParcelPosture::ProducedUnconsumed,
            parcel_digest: custody.parcel_digest,
        };
        let digest =
            crate::snow_owner_v4::canonical_terminal_parcel_digest(&parcel).map_err(|_| {
                DirectSnowStage3V11AttachmentError::Identity(
                    "terminal receiver custody parcel seal",
                )
            })?;
        if digest != custody.parcel_digest
            || custody.parent_transaction_id != accepted.parent_transaction_id().digest()
            || custody.event_ordinal != accepted.ordinal()
            || custody.terminal_event_proposal_core_id
                != group.proposal_core_sha256.ok_or(
                    DirectSnowStage3V11AttachmentError::Identity(
                        "terminal receiver custody proposal core",
                    ),
                )?
            || custody.event_result_digest != candidate.event_result_digest
            || custody.support != candidate.support
            || custody.mass_kg_m2_tile_ground.to_bits()
                != candidate.event.terminal_liquid_kg_m2.to_bits()
            || custody.receiver_destinations.is_empty()
            || !fraction_sum.is_finite()
            || (fraction_sum - 1.0).abs() > 1.0e-12
            || custody.receiver_destinations.windows(2).any(|pair| {
                (&pair[0].destination_ofe_id, &pair[0].destination_tile_id)
                    >= (&pair[1].destination_ofe_id, &pair[1].destination_tile_id)
            })
            || custody.receiver_destinations.iter().any(|destination| {
                destination.destination_ofe_id != custody.destination_ofe_id
                    || !destination.destination_fraction.is_finite()
                    || destination.destination_fraction <= 0.0
            })
        {
            return Err(DirectSnowStage3V11AttachmentError::Identity(
                "terminal receiver custody identity",
            ));
        }
        digests.push(digest);
    }
    digests.sort_unstable();
    if digests != group.produced_unconsumed_parcel_digests
        || digests.len() != group.candidates.len()
        || digests.windows(2).any(|pair| pair[0] == pair[1])
    {
        return Err(DirectSnowStage3V11AttachmentError::Identity(
            "terminal receiver custody parcel set",
        ));
    }
    Ok(())
}

fn advance_canonical_terminal_event_ordinal(
    next_by_parent: &mut BTreeMap<ParentTransactionId, u32>,
    parent: ParentTransactionId,
    accepted_ordinal: u32,
) -> Result<(), DirectSnowStage3V11AttachmentError> {
    let expected = next_by_parent.get(&parent).copied().unwrap_or(0);
    if accepted_ordinal != expected {
        return Err(DirectSnowStage3V11AttachmentError::Identity(
            "terminal accepted-event receipt ordinal sequence",
        ));
    }
    let next = expected
        .checked_add(1)
        .ok_or(DirectSnowStage3V11AttachmentError::Identity(
            "terminal accepted-event receipt ordinal overflow",
        ))?;
    next_by_parent.insert(parent, next);
    Ok(())
}

#[derive(Clone, Copy)]
struct AcceptedOwnerHandoffChronologyV1 {
    id: openwepp_coupled_time::ReceiptId,
    parent_transaction_id: ParentTransactionId,
    tick: ModelTimeNs,
    ordinal: u32,
    beginning_owner_sha256: Digest32,
    ending_owner_sha256: Digest32,
}

fn canonical_accepted_event_index_v1(
    handoffs: &[AcceptedOwnerHandoffChronologyV1],
) -> Result<
    BTreeMap<(ParentTransactionId, u32), openwepp_coupled_time::ReceiptId>,
    DirectSnowStage3V11AttachmentError,
> {
    let mut next_ordinal_by_parent = BTreeMap::new();
    let mut event_by_parent_ordinal = BTreeMap::new();
    let mut event_ids = BTreeSet::new();
    for event in handoffs {
        if !event_ids.insert(event.id) {
            return Err(DirectSnowStage3V11AttachmentError::Identity(
                "accepted publication event duplicate",
            ));
        }
        advance_canonical_terminal_event_ordinal(
            &mut next_ordinal_by_parent,
            event.parent_transaction_id,
            event.ordinal,
        )?;
        if event_by_parent_ordinal
            .insert((event.parent_transaction_id, event.ordinal), event.id)
            .is_some()
        {
            return Err(DirectSnowStage3V11AttachmentError::Identity(
                "accepted publication event parent/ordinal duplicate",
            ));
        }
    }
    Ok(event_by_parent_ordinal)
}

fn validate_subslab_owner_adjacency_v1(
    handoffs: &[AcceptedOwnerHandoffChronologyV1],
    tick: ModelTimeNs,
    preceding_parent_sha256: Digest32,
    following_parent_sha256: Digest32,
    preceding_ending_owner_sha256: Digest32,
    following_beginning_owner_sha256: Digest32,
) -> Result<(), DirectSnowStage3V11AttachmentError> {
    let mut traversed_owner = preceding_ending_owner_sha256;
    let mut accepted_ids = BTreeSet::new();
    for (index, handoff) in handoffs.iter().enumerate() {
        if handoff.tick != tick {
            continue;
        }
        let prior_same_parent = handoffs[..index]
            .iter()
            .rev()
            .find(|prior| prior.parent_transaction_id == handoff.parent_transaction_id);
        let ordinal_is_valid = prior_same_parent.map_or(handoff.ordinal == 0, |prior| {
            prior.ordinal.checked_add(1) == Some(handoff.ordinal)
        });
        if !accepted_ids.insert(handoff.id)
            || !ordinal_is_valid
            || (handoff.parent_transaction_id.digest() != preceding_parent_sha256
                && handoff.parent_transaction_id.digest() != following_parent_sha256)
            || handoff.beginning_owner_sha256 != traversed_owner
        {
            return Err(DirectSnowStage3V11AttachmentError::Identity(
                "parent subslab accepted-event owner adjacency",
            ));
        }
        traversed_owner = handoff.ending_owner_sha256;
    }
    if traversed_owner != following_beginning_owner_sha256 {
        return Err(DirectSnowStage3V11AttachmentError::Identity(
            "parent subslab chronology/owner adjacency",
        ));
    }
    Ok(())
}

#[allow(clippy::too_many_arguments)]
fn validate_retained_subslab_pair_after_complete_publication_v1(
    handoffs: &[AcceptedOwnerHandoffChronologyV1],
    preceding_support: TimeSupport,
    following_support: TimeSupport,
    preceding_parent_sha256: Digest32,
    following_parent_sha256: Digest32,
    preceding_ending_owner_sha256: Digest32,
    following_beginning_owner_sha256: Digest32,
) -> Result<(), DirectSnowStage3V11AttachmentError> {
    if preceding_support.end_ns() > following_support.start_ns() {
        return Err(DirectSnowStage3V11AttachmentError::Identity(
            "parent retained subslab chronology",
        ));
    }
    if preceding_support.end_ns() < following_support.start_ns() {
        // `accepted_publication_supports_for_day` has already proved that the
        // complete publication sequence fills this sparse Stage-3 gap and is
        // owner-adjacent through every accepted event handoff. The exact-union
        // join above additionally proves each intervening publication is a
        // sealed snow-free successor, so it must not be reclassified as a
        // retained Stage-3 subslab here.
        return Ok(());
    }
    validate_subslab_owner_adjacency_v1(
        handoffs,
        preceding_support.end_ns(),
        preceding_parent_sha256,
        following_parent_sha256,
        preceding_ending_owner_sha256,
        following_beginning_owner_sha256,
    )
}

fn bind_parent_receipt_snow_owner_bytes_v1(
    complete_owner_bytes: &mut BTreeMap<String, Vec<u8>>,
    coupled_owners: &[openwepp_coupled_time::OwnerState],
) -> Result<(), DirectSnowStage3V11AttachmentError> {
    let snow = coupled_owners
        .iter()
        .find(|owner| owner.owner_id() == "snow")
        .ok_or(DirectSnowStage3V11AttachmentError::Identity(
            "parent receipt coupled snow owner",
        ))?;
    complete_owner_bytes.insert("snow".to_owned(), snow.state_bytes().to_vec());
    Ok(())
}

include!("snow_stage3_v11_subslab_owner_adjacency_tests.rs");
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
struct QualificationAdaptivePublicationSupportV1 {
    parent_transaction_sha256: Digest32,
    support: TimeSupport,
    effective_ending_complete_owner_set_sha256: Digest32,
    event_posture: Stage3AdaptiveEventPostureV1,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
struct QualificationRetainedSubslabV1 {
    parent_transaction_sha256: Digest32,
    support: TimeSupport,
    physical_ending_complete_owner_set_sha256: Digest32,
    effective_ending_complete_owner_set_sha256: Digest32,
    terminal_event_at_support_end: bool,
}

#[derive(Clone, Debug, Eq, PartialEq)]
struct QualificationRetainedPublicationSupportV1 {
    support: TimeSupport,
    physical_ending_complete_owner_set_sha256: Digest32,
    ordered_owner_chain_sha256s: Vec<Digest32>,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
struct QualificationRetainedSnowFreeSuccessorV1 {
    parent_transaction_sha256: Digest32,
    support: TimeSupport,
}

fn qualification_supports_overlap_v1(left: TimeSupport, right: TimeSupport) -> bool {
    left.start_ns() < right.end_ns() && right.start_ns() < left.end_ns()
}

fn validate_qualification_adaptive_publication_crossjoin_v1(
    expected: &[QualificationAdaptivePublicationSupportV1],
    retained_subslabs: &[QualificationRetainedSubslabV1],
    retained_publication: &[QualificationRetainedPublicationSupportV1],
    retained_snow_free_successors: &[QualificationRetainedSnowFreeSuccessorV1],
) -> Result<(), DirectSnowStage3V11AttachmentError> {
    if retained_subslabs.len() != retained_publication.len() {
        return Err(DirectSnowStage3V11AttachmentError::Identity(
            "qualification subslab/publication support cardinality",
        ));
    }
    let mut retained_cursor = 0_usize;
    let mut consumed_successors = BTreeSet::new();
    let mut ending_authorities = Vec::with_capacity(expected.len());
    for expected_support in expected {
        let first_retained = retained_cursor;
        let mut covered_until = expected_support.support.start_ns();
        while let Some(retained) = retained_subslabs.get(retained_cursor) {
            if retained.parent_transaction_sha256
                != expected_support.parent_transaction_sha256
                || retained.support.start_ns() != covered_until
                || retained.support.end_ns() > expected_support.support.end_ns()
            {
                break;
            }
            if expected_support.event_posture == Stage3AdaptiveEventPostureV1::NoEvent
                && retained.terminal_event_at_support_end
            {
                return Err(DirectSnowStage3V11AttachmentError::Identity(
                    "qualification nonterminal adaptive physical partition",
                ));
            }
            covered_until = retained.support.end_ns();
            retained_cursor += 1;
            if retained.terminal_event_at_support_end
                || covered_until == expected_support.support.end_ns()
            {
                break;
            }
        }
        if retained_cursor == first_retained {
            return Err(DirectSnowStage3V11AttachmentError::Identity(
                "qualification adaptive/subslab support cardinality",
            ));
        }
        let final_subslab_index = retained_cursor - 1;
        let final_subslab = &retained_subslabs[final_subslab_index];
        let overlapping_successors = retained_snow_free_successors
            .iter()
            .enumerate()
            .filter(|(_, successor)| {
                qualification_supports_overlap_v1(successor.support, expected_support.support)
            })
            .collect::<Vec<_>>();
        if expected_support.event_posture == Stage3AdaptiveEventPostureV1::TerminalEvent {
            if !final_subslab.terminal_event_at_support_end {
                return Err(DirectSnowStage3V11AttachmentError::Identity(
                    "qualification terminal adaptive/subslab prefix chronology",
                ));
            }
            for (successor_index, successor) in overlapping_successors {
                if !consumed_successors.insert(successor_index)
                    || successor.parent_transaction_sha256
                        != expected_support.parent_transaction_sha256
                    || successor.support.start_ns() != covered_until
                    || successor.support.end_ns() > expected_support.support.end_ns()
                {
                    return Err(DirectSnowStage3V11AttachmentError::Identity(
                        "qualification terminal snow-free successor chronology",
                    ));
                }
                covered_until = successor.support.end_ns();
            }
        } else {
            if !overlapping_successors.is_empty() {
                return Err(DirectSnowStage3V11AttachmentError::Identity(
                    "qualification adaptive/subslab support chronology",
                ));
            }
        }
        if covered_until != expected_support.support.end_ns() {
            return Err(DirectSnowStage3V11AttachmentError::Identity(
                "qualification adaptive execution partition",
            ));
        }
        ending_authorities.push(final_subslab_index);
    }
    let relevant_successor_count = retained_snow_free_successors
        .iter()
        .filter(|successor| {
            expected.iter().any(|expected_support| {
                qualification_supports_overlap_v1(successor.support, expected_support.support)
            })
        })
        .count();
    if retained_cursor != retained_subslabs.len()
        || consumed_successors.len() != relevant_successor_count
    {
        return Err(DirectSnowStage3V11AttachmentError::Identity(
            "qualification adaptive/subslab support cardinality",
        ));
    }
    for (subslab, publication) in retained_subslabs.iter().zip(retained_publication) {
        if subslab.support != publication.support {
            return Err(DirectSnowStage3V11AttachmentError::Identity(
                "qualification subslab/publication support chronology",
            ));
        }
        if subslab.physical_ending_complete_owner_set_sha256
            != publication.physical_ending_complete_owner_set_sha256
        {
            return Err(DirectSnowStage3V11AttachmentError::Identity(
                "qualification subslab/publication physical owner join",
            ));
        }
        let subslab_effective_ordinal = publication
            .ordered_owner_chain_sha256s
            .iter()
            .position(|owner| owner == &subslab.effective_ending_complete_owner_set_sha256);
        if publication.ordered_owner_chain_sha256s.first()
            != Some(&publication.physical_ending_complete_owner_set_sha256)
            || subslab_effective_ordinal.is_none()
        {
            return Err(DirectSnowStage3V11AttachmentError::Identity(
                "qualification subslab/publication receiver owner lineage",
            ));
        }
    }
    for (expected_support, final_subslab_index) in
        expected.iter().zip(ending_authorities)
    {
        let subslab = &retained_subslabs[final_subslab_index];
        let publication = &retained_publication[final_subslab_index];
        let subslab_ordinal = publication
            .ordered_owner_chain_sha256s
            .iter()
            .position(|owner| owner == &subslab.effective_ending_complete_owner_set_sha256);
        let adaptive_ordinal = publication
            .ordered_owner_chain_sha256s
            .iter()
            .position(|owner| {
                owner == &expected_support.effective_ending_complete_owner_set_sha256
            });
        if adaptive_ordinal.is_none() || subslab_ordinal > adaptive_ordinal {
            return Err(DirectSnowStage3V11AttachmentError::Identity(
                "qualification adaptive ending owner lineage",
            ));
        }
    }
    Ok(())
}

fn qualification_retained_subslab_v1(
    subslab: &Stage3CoupledSubslabReceiptV1,
) -> Result<QualificationRetainedSubslabV1, DirectSnowStage3V11AttachmentError> {
    let terminal_event_at_support_end = if subslab.terminal_events.is_empty() {
        false
    } else {
        subslab
            .terminal_events
            .values()
            .try_fold(true, |at_end, event| {
                let event_offset = quantize_seconds_to_tick(
                    ModelTimeNs::new(0),
                    ModelTimeNs::new(subslab.support.duration_ns()),
                    event.hour_offset_seconds,
                )?;
                Ok::<_, DirectSnowStage3V11AttachmentError>(
                    at_end
                        && event.event_occurred
                        && (event.terminal_entry_offset_seconds + event.evaluated_seconds).to_bits()
                            == event.hour_offset_seconds.to_bits()
                        && event_offset.get() == subslab.support.duration_ns(),
                )
            })?
    };
    Ok(QualificationRetainedSubslabV1 {
        parent_transaction_sha256: subslab.owner_join.parent_transaction_sha256,
        support: subslab.support,
        physical_ending_complete_owner_set_sha256: subslab
            .owner_join
            .ending_complete_owner_set_sha256,
        effective_ending_complete_owner_set_sha256: subslab
            .effective_ending_complete_owner_set_sha256(),
        terminal_event_at_support_end,
    })
}

fn qualification_validated_snow_free_successors_v1(
    receipts: &[Stage3SnowFreeSuccessorReceiptV1],
    publication: &[&crate::v9_real_consumer_shadow::Stage3AcceptedPublicationSupportV1],
) -> Result<Vec<QualificationRetainedSnowFreeSuccessorV1>, DirectSnowStage3V11AttachmentError> {
    receipts
        .iter()
        .map(|receipt| {
            let matches = publication
                .iter()
                .copied()
                .filter(|support| receipt.validate_against_publication(support).is_ok())
                .collect::<Vec<_>>();
            if matches.len() != 1 {
                return Err(DirectSnowStage3V11AttachmentError::Identity(
                    "qualification snow-free successor/publication unique cross-join",
                ));
            }
            Ok(QualificationRetainedSnowFreeSuccessorV1 {
                parent_transaction_sha256: receipt.parent_transaction_id.digest(),
                support: receipt.support,
            })
        })
        .collect()
}

fn qualification_exact_adaptive_publication_subset(
    retained_subslabs: &[QualificationRetainedSubslabV1],
    same_parent_publication: &[QualificationRetainedPublicationSupportV1],
) -> Vec<QualificationRetainedPublicationSupportV1> {
    same_parent_publication
        .iter()
        .cloned()
        .filter(|publication| {
            retained_subslabs
                .iter()
                .any(|subslab| subslab.support == publication.support)
        })
        .collect()
}

fn qualification_expected_adaptive_publication_supports_v1(
    adaptive: &Stage3AdaptiveSupportReceiptV1,
) -> Result<Vec<QualificationAdaptivePublicationSupportV1>, DirectSnowStage3V11AttachmentError> {
    adaptive.validate()?;
    let mut expected = Vec::new();
    for accepted in &adaptive.accepted_microsteps {
        let comparison = adaptive
            .comparisons
            .iter()
            .find(|comparison| comparison.receipt_sha256 == accepted.comparison_receipt_sha256)
            .ok_or(DirectSnowStage3V11AttachmentError::Identity(
                "qualification accepted adaptive comparison",
            ))?;
        let request = adaptive
            .parent_requests
            .iter()
            .find(|request| request.context == accepted.context)
            .ok_or(DirectSnowStage3V11AttachmentError::Identity(
                "qualification accepted adaptive request",
            ))?;
        let direct = adaptive
            .direct_trials
            .iter()
            .find(|direct| direct.receipt_sha256 == comparison.direct_trial_sha256)
            .ok_or(DirectSnowStage3V11AttachmentError::Identity(
                "qualification accepted adaptive direct trial",
            ))?;
        accepted.validate_against(comparison)?;
        match accepted.decision {
            Stage3AdaptiveStepDecisionV1::FloorAccepted => {
                comparison.validate_floor_against(request, direct)?;
                expected.push(QualificationAdaptivePublicationSupportV1 {
                    parent_transaction_sha256: adaptive.parent_transaction_id.digest(),
                    support: accepted.context.step_support,
                    effective_ending_complete_owner_set_sha256: accepted
                        .ending_complete_owner_set_sha256,
                    event_posture: accepted.event_posture,
                });
            }
            Stage3AdaptiveStepDecisionV1::ComposedAccepted => {
                let child_1_sha256 = comparison.split_child_1_sha256.ok_or(
                    DirectSnowStage3V11AttachmentError::Identity(
                        "qualification composed adaptive child 1",
                    ),
                )?;
                let child_2_sha256 = comparison.split_child_2_sha256.ok_or(
                    DirectSnowStage3V11AttachmentError::Identity(
                        "qualification composed adaptive child 2",
                    ),
                )?;
                let child_1 = adaptive
                    .split_child_trials
                    .iter()
                    .find(|child| child.receipt_sha256 == child_1_sha256)
                    .ok_or(DirectSnowStage3V11AttachmentError::Identity(
                        "qualification composed adaptive child 1",
                    ))?;
                let child_2 = adaptive
                    .split_child_trials
                    .iter()
                    .find(|child| child.receipt_sha256 == child_2_sha256)
                    .ok_or(DirectSnowStage3V11AttachmentError::Identity(
                        "qualification composed adaptive child 2",
                    ))?;
                comparison.validate_composed_against(request, direct, child_1, child_2)?;
                expected.extend([
                    QualificationAdaptivePublicationSupportV1 {
                        parent_transaction_sha256: adaptive.parent_transaction_id.digest(),
                        support: child_1.child_support,
                        effective_ending_complete_owner_set_sha256: child_1
                            .ending_complete_owner_set_sha256,
                        event_posture: child_1.event_posture,
                    },
                    QualificationAdaptivePublicationSupportV1 {
                        parent_transaction_sha256: adaptive.parent_transaction_id.digest(),
                        support: child_2.child_support,
                        effective_ending_complete_owner_set_sha256: child_2
                            .ending_complete_owner_set_sha256,
                        event_posture: child_2.event_posture,
                    },
                ]);
            }
            Stage3AdaptiveStepDecisionV1::RefineRejected
            | Stage3AdaptiveStepDecisionV1::FloorRejected => {
                return Err(DirectSnowStage3V11AttachmentError::Identity(
                    "qualification accepted adaptive decision",
                ));
            }
        }
    }
    Ok(expected)
}

fn validate_adaptive_parent_publication_crossjoin_v1(
    adaptive: &Stage3AdaptiveSupportReceiptV1,
    retained_subslabs: &[Stage3CoupledSubslabReceiptV1],
    snow_free_successor_receipts: &[Stage3SnowFreeSuccessorReceiptV1],
    real_consumer: &DirectV10RealConsumerShadow,
    day_index: usize,
) -> Result<(), DirectSnowStage3V11AttachmentError> {
    let expected = qualification_expected_adaptive_publication_supports_v1(adaptive)?;
    let retained_subslabs = retained_subslabs
        .iter()
        .map(qualification_retained_subslab_v1)
        .collect::<Result<Vec<_>, _>>()?;
    let all_publication = real_consumer.accepted_publication_supports_for_day(day_index)?;
    let same_parent_publication = all_publication
        .iter()
        .copied()
        .into_iter()
        .filter(|support| support.parent_transaction_id() == adaptive.parent_transaction_id)
        .map(|support| {
            Ok(QualificationRetainedPublicationSupportV1 {
                support: support.support(),
                physical_ending_complete_owner_set_sha256: support
                    .ending_complete_owner_set_sha256(),
                ordered_owner_chain_sha256s: real_consumer
                    .accepted_publication_ordered_owner_chain(support)?,
            })
        })
        .collect::<Result<Vec<_>, DirectV11RealConsumerError>>()?;
    // One parent can transition from adaptive Stage-3 execution to a
    // snow-free successor. Parent identity alone therefore intentionally
    // selects a superset; the adaptive qualification join is the exact
    // temporal intersection with its retained coupled subslabs. The complete
    // parent receipt independently proves the adaptive/snow-free publication
    // union below.
    let retained_publication = qualification_exact_adaptive_publication_subset(
        &retained_subslabs,
        &same_parent_publication,
    );
    let retained_snow_free_successors = qualification_validated_snow_free_successors_v1(
        snow_free_successor_receipts,
        &all_publication,
    )?;
    validate_qualification_adaptive_publication_crossjoin_v1(
        &expected,
        &retained_subslabs,
        &retained_publication,
        &retained_snow_free_successors,
    )
}

#[derive(Clone, Debug, PartialEq)]
pub struct DirectSnowStage3V11ParentReceipt {
    pub day_index: usize,
    pub support_count: usize,
    pub terminal_events: Vec<DirectSnowStage3V11TerminalReceipt>,
    pub terminal_event_groups: Vec<Stage3V11TerminalEventGroupV1>,
    pub ending_stage3_state_digests: BTreeMap<u32, Digest32>,
    pub complete_owner_bytes: BTreeMap<String, Vec<u8>>,
    pub covered_owner_joins: Vec<CoveredParentOwnerJoinReceiptV1>,
    pub coupled_subslabs: Vec<Stage3CoupledSubslabReceiptV1>,
    pub adaptive_support_receipts: Vec<Stage3AdaptiveSupportReceiptV1>,
    pub snow_free_successor_receipts: Vec<Stage3SnowFreeSuccessorReceiptV1>,
    pub integrated_boundary_ledger: Stage3ParentIntegratedBoundaryLedgerV1,
    pub ending_coupled_owner_set_sha256: Digest32,
    pub ending_coupled_accepted_until_ns: ModelTimeNs,
    pub ending_next_parent_sequence: u128,
    pub ending_v11_parent_state: V11ParentTransaction,
    pub ending_last_v11_parent_candidate: Option<V11ParentCandidate>,
}

fn wb14_parent_finalization_placement_is_valid_v1(
    parent_support: TimeSupport,
    support: TimeSupport,
    parent_replay_is_present: bool,
) -> bool {
    parent_replay_is_present == (support.end_ns() == parent_support.end_ns())
}

impl DirectSnowStage3V11ParentReceipt {
    fn validate_adaptive_publication_crossjoin_v1(
        &self,
        real_consumer: &DirectV10RealConsumerShadow,
    ) -> Result<(), DirectSnowStage3V11AttachmentError> {
        let mut expected = Vec::new();
        for adaptive in &self.adaptive_support_receipts {
            expected.extend(qualification_expected_adaptive_publication_supports_v1(
                adaptive,
            )?);
        }
        let retained_subslabs = self
            .coupled_subslabs
            .iter()
            .map(qualification_retained_subslab_v1)
            .collect::<Result<Vec<_>, _>>()?;
        let all_publication =
            real_consumer.accepted_publication_supports_for_day(self.day_index)?;
        let retained_publication = all_publication
            .iter()
            .copied()
            .into_iter()
            .filter(|support| {
                retained_subslabs
                    .iter()
                    .any(|subslab| subslab.support == support.support())
            })
            .map(|support| {
                Ok(QualificationRetainedPublicationSupportV1 {
                    support: support.support(),
                    physical_ending_complete_owner_set_sha256: support
                        .ending_complete_owner_set_sha256(),
                    ordered_owner_chain_sha256s: real_consumer
                        .accepted_publication_ordered_owner_chain(support)?,
                })
            })
            .collect::<Result<Vec<_>, DirectV11RealConsumerError>>()?;
        let retained_snow_free_successors = qualification_validated_snow_free_successors_v1(
            &self.snow_free_successor_receipts,
            &all_publication,
        )?;
        validate_qualification_adaptive_publication_crossjoin_v1(
            &expected,
            &retained_subslabs,
            &retained_publication,
            &retained_snow_free_successors,
        )?;
        let mut matched_publication_receipts = BTreeSet::new();
        for receipt in &self.snow_free_successor_receipts {
            let matches = all_publication
                .iter()
                .copied()
                .filter(|support| receipt.validate_against_publication(support).is_ok())
                .collect::<Vec<_>>();
            if matches.len() != 1
                || !matched_publication_receipts.insert(matches[0].receipt_sha256())
            {
                return Err(DirectSnowStage3V11AttachmentError::Identity(
                    "snow-free successor/publication unique cross-join",
                ));
            }
        }
        for subslab in &retained_subslabs {
            let matches = all_publication
                .iter()
                .copied()
                .filter(|support| {
                    support.support() == subslab.support
                        && support.ending_complete_owner_set_sha256()
                            == subslab.physical_ending_complete_owner_set_sha256
                })
                .collect::<Vec<_>>();
            if matches.len() != 1
                || !matched_publication_receipts.insert(matches[0].receipt_sha256())
            {
                return Err(DirectSnowStage3V11AttachmentError::Identity(
                    "adaptive subslab/publication unique cross-join",
                ));
            }
        }
        if matched_publication_receipts.len() != all_publication.len() {
            return Err(DirectSnowStage3V11AttachmentError::Identity(
                "Stage-3 execution-class/publication exact union",
            ));
        }
        Ok(())
    }

    fn validate_against_ending(
        &self,
        ending: &DirectSnowStage3V11CommittedState,
    ) -> Result<(), DirectSnowStage3V11AttachmentError> {
        for subslab in &self.coupled_subslabs {
            subslab.validate()?;
        }
        for adaptive in &self.adaptive_support_receipts {
            adaptive.validate()?;
        }
        for receipt in &self.snow_free_successor_receipts {
            receipt.validate()?;
        }
        let execution_parent_ids = self
            .adaptive_support_receipts
            .iter()
            .map(|receipt| receipt.parent_transaction_id)
            .chain(
                self.snow_free_successor_receipts
                    .iter()
                    .map(|receipt| receipt.parent_transaction_id),
            )
            .collect::<BTreeSet<_>>();
        if execution_parent_ids.len() != self.support_count
            || self
                .snow_free_successor_receipts
                .windows(2)
                .any(|pair| pair[0].support.end_ns() > pair[1].support.start_ns())
        {
            return Err(DirectSnowStage3V11AttachmentError::Identity(
                "Stage-3 parent execution-class chronology",
            ));
        }
        self.validate_adaptive_publication_crossjoin_v1(&ending.real_consumer)?;
        if self.adaptive_support_receipts.windows(2).any(|pair| {
            pair[0].parent_support.end_ns() > pair[1].parent_support.start_ns()
                || pair[0].parent_transaction_id == pair[1].parent_transaction_id
        }) || self.adaptive_support_receipts.iter().any(|adaptive| {
            !self
                .coupled_subslabs
                .iter()
                .any(|subslab| subslab.parent_support == adaptive.parent_support)
        }) {
            return Err(DirectSnowStage3V11AttachmentError::Identity(
                "adaptive parent support receipt chronology",
            ));
        }
        let terminal_subslabs = self
            .coupled_subslabs
            .iter()
            .filter(|subslab| !subslab.terminal_events.is_empty())
            .collect::<Vec<_>>();
        if terminal_subslabs.len() != self.terminal_event_groups.len() {
            return Err(DirectSnowStage3V11AttachmentError::Identity(
                "terminal group/subslab cardinality",
            ));
        }
        let accepted_event_handoffs = ending
            .real_consumer
            .accepted_publication_event_handoffs_for_owner_adjacency()
            .iter()
            .map(|event| {
                event.validate().map_err(|_| {
                    DirectSnowStage3V11AttachmentError::Identity(
                        "parent subslab accepted-event seal",
                    )
                })?;
                Ok(AcceptedOwnerHandoffChronologyV1 {
                    id: event.id(),
                    parent_transaction_id: event.parent_transaction_id(),
                    tick: event.tick(),
                    ordinal: event.ordinal(),
                    beginning_owner_sha256: event.beginning_owner_set_digest(),
                    ending_owner_sha256: event.ending_owner_set_digest(),
                })
            })
            .collect::<Result<Vec<_>, DirectSnowStage3V11AttachmentError>>()?;
        let complete_event_by_parent_ordinal =
            canonical_accepted_event_index_v1(&accepted_event_handoffs)?;
        let mut accepted_ids = BTreeSet::new();
        let mut terminal_groups_for_ordinal = self.terminal_event_groups.iter();
        for subslab in &self.coupled_subslabs {
            if !subslab.terminal_events.is_empty() {
                let group = terminal_groups_for_ordinal.next().ok_or(
                    DirectSnowStage3V11AttachmentError::Identity("terminal ordinal group omission"),
                )?;
                let terminal = group.accepted_event_receipt.as_ref().ok_or(
                    DirectSnowStage3V11AttachmentError::Identity(
                        "terminal ordinal accepted-event omission",
                    ),
                )?;
                if !accepted_ids.insert(terminal.id()) {
                    return Err(DirectSnowStage3V11AttachmentError::Identity(
                        "terminal/support-liquid accepted-event duplicate",
                    ));
                }
                if complete_event_by_parent_ordinal
                    .get(&(terminal.parent_transaction_id(), terminal.ordinal()))
                    != Some(&terminal.id())
                {
                    return Err(DirectSnowStage3V11AttachmentError::Identity(
                        "terminal accepted-event publication chronology join",
                    ));
                }
            }
            if let Some(receiver) = &subslab.post_support_liquid_receiver_event {
                receiver.validate().map_err(|_| {
                    DirectSnowStage3V11AttachmentError::Identity(
                        "support-liquid receiver ordinal event seal",
                    )
                })?;
                if !accepted_ids.insert(receiver.id()) {
                    return Err(DirectSnowStage3V11AttachmentError::Identity(
                        "terminal/support-liquid accepted-event duplicate",
                    ));
                }
                if complete_event_by_parent_ordinal
                    .get(&(receiver.parent_transaction_id(), receiver.ordinal()))
                    != Some(&receiver.id())
                {
                    return Err(DirectSnowStage3V11AttachmentError::Identity(
                        "support-liquid accepted-event publication chronology join",
                    ));
                }
            }
        }
        if terminal_groups_for_ordinal.next().is_some() {
            return Err(DirectSnowStage3V11AttachmentError::Identity(
                "terminal ordinal group excess",
            ));
        }
        for (subslab, group) in terminal_subslabs
            .into_iter()
            .zip(&self.terminal_event_groups)
        {
            validate_retained_terminal_receiver_custody_v1(group)?;
            group.validate_terminal_receiver_custody_v2()?;
            let accepted = group.accepted_event_receipt.as_ref().ok_or(
                DirectSnowStage3V11AttachmentError::Identity(
                    "terminal group accepted-event receipt",
                ),
            )?;
            let ledger = group.terminal_physical_ledger.as_ref().ok_or(
                DirectSnowStage3V11AttachmentError::Identity("terminal group physical ledger"),
            )?;
            let group_parcel_digests = &group.produced_unconsumed_parcel_digests;
            let reconstructed_parcels = group
                .produced_unconsumed_parcels
                .iter()
                .map(|custody| DirectSnowStage3V11TerminalParcel {
                    support: custody.support,
                    source_lane_id: custody.source_lane_id,
                    parent_transaction_id: custody.parent_transaction_id,
                    event_ordinal: custody.event_ordinal,
                    terminal_event_proposal_core_id: custody.terminal_event_proposal_core_id,
                    event_result_digest: custody.event_result_digest,
                    receiver_topology_sha256: custody.receiver_topology_sha256,
                    destination_ofe_id: custody.destination_ofe_id.clone(),
                    receiver_destinations: custody
                        .receiver_destinations
                        .iter()
                        .map(
                            |destination| DirectSnowStage3V11TerminalReceiverDestinationV1 {
                                destination_ofe_id: destination.destination_ofe_id.clone(),
                                destination_tile_id: destination.destination_tile_id.clone(),
                                destination_fraction: destination.destination_fraction,
                            },
                        )
                        .collect(),
                    mass_kg_m2_tile_ground: custody.mass_kg_m2_tile_ground,
                    temperature_k: custody.temperature_k,
                    specific_liquid_enthalpy_j_kg: custody.specific_liquid_enthalpy_j_kg,
                    posture: DirectSnowStage3V11TerminalParcelPosture::ProducedUnconsumed,
                    parcel_digest: custody.parcel_digest,
                })
                .collect::<Vec<_>>();
            let mut reconstructed_parcel_digests = reconstructed_parcels
                .iter()
                .map(|parcel| crate::snow_owner_v4::canonical_terminal_parcel_digest(parcel))
                .collect::<Result<Vec<_>, _>>()
                .map_err(|_| {
                    DirectSnowStage3V11AttachmentError::Identity(
                        "terminal group retained parcel custody",
                    )
                })?;
            reconstructed_parcel_digests.sort_unstable();
            let group_parcel_fields = group_parcel_digests
                .iter()
                .map(|digest| FramedField {
                    tag: "parcel",
                    value: digest.as_bytes(),
                })
                .collect::<Vec<_>>();
            let reconstructed_parcel_set =
                framed_sha256("stage3-v11-terminal-parcel-set", &group_parcel_fields)?;
            if group.accepted_group_receipt_sha256 != Some(accepted_terminal_group_digest(group)?)
                || accepted.tick() != group.tick
                || u64::from(accepted.ordinal()) != group.ordinal
                || accepted.event_context_digest() != group.receipt_sha256
                || group_parcel_digests.len() != group.candidates.len()
                || reconstructed_parcel_digests != *group_parcel_digests
                || group_parcel_digests
                    .windows(2)
                    .any(|pair| pair[0] >= pair[1])
                || ledger.produced_unconsumed_parcel_set_sha256 != reconstructed_parcel_set
                || accepted.beginning_owner_set_digest()
                    != subslab.effective_ending_complete_owner_set_sha256()
                || group.candidates.len() != subslab.terminal_events.len()
                || group.candidates.iter().any(|candidate| {
                    subslab.terminal_events.get(&candidate.lane_id) != Some(&candidate.event)
                        || canonical_terminal_event_result_digest(&candidate.event).ok()
                            != Some(candidate.event_result_digest)
                })
            {
                return Err(DirectSnowStage3V11AttachmentError::Identity(
                    "terminal group accepted-event reconstruction",
                ));
            }
        }
        for pair in self.coupled_subslabs.windows(2) {
            validate_retained_subslab_pair_after_complete_publication_v1(
                &accepted_event_handoffs,
                pair[0].support,
                pair[1].support,
                pair[0].owner_join.parent_transaction_sha256,
                pair[1].owner_join.parent_transaction_sha256,
                pair[0].owner_join.ending_complete_owner_set_sha256,
                pair[1].owner_join.beginning_complete_owner_set_sha256,
            )?;
        }
        if self.coupled_subslabs.iter().any(|value| {
            !wb14_parent_finalization_placement_is_valid_v1(
                value.parent_support,
                value.support,
                value.wb14_parent_replay_bytes.is_some(),
            )
        }) {
            return Err(DirectSnowStage3V11AttachmentError::Identity(
                "parent WB14 finalization placement",
            ));
        }
        if self.covered_owner_joins
            != self
                .coupled_subslabs
                .iter()
                .map(|value| value.owner_join.clone())
                .collect::<Vec<_>>()
            || self.integrated_boundary_ledger
                != reconstruct_integrated_boundary_ledger(&self.coupled_subslabs)
        {
            return Err(DirectSnowStage3V11AttachmentError::Identity(
                "parent receipt reconstruction",
            ));
        }
        let stage3_digests = ending
            .stage3_by_lane
            .iter()
            .map(|(lane, state)| {
                let bytes = Wb11HydrologyKernel::serialize_stage3_persistent_state(state).map_err(
                    |_| DirectSnowStage3V11AttachmentError::Identity("Stage-3 restart bytes"),
                )?;
                Ok((*lane, digest_bytes(&bytes)))
            })
            .collect::<Result<BTreeMap<_, _>, DirectSnowStage3V11AttachmentError>>()?;
        let mut owner_bytes = ending
            .real_consumer
            .canonical_owner_state_bytes()
            .map_err(|_| {
                DirectSnowStage3V11AttachmentError::Identity("canonical V11 owner bytes")
            })?;
        bind_parent_receipt_snow_owner_bytes_v1(&mut owner_bytes, ending.coupled_clock.owners())?;
        if stage3_digests != self.ending_stage3_state_digests {
            return Err(DirectSnowStage3V11AttachmentError::Identity(
                "parent receipt ending Stage-3 digest join",
            ));
        }
        if owner_bytes != self.complete_owner_bytes {
            return Err(DirectSnowStage3V11AttachmentError::Identity(
                "parent receipt ending complete-owner bytes join",
            ));
        }
        if complete_owner_set_digest(ending.coupled_clock.owners())?
            != self.ending_coupled_owner_set_sha256
        {
            return Err(DirectSnowStage3V11AttachmentError::Identity(
                "parent receipt ending coupled-owner digest join",
            ));
        }
        if ending.coupled_clock.accepted_until() != self.ending_coupled_accepted_until_ns {
            return Err(DirectSnowStage3V11AttachmentError::Identity(
                "parent receipt ending accepted-cursor join",
            ));
        }
        if ending.next_parent_sequence != self.ending_next_parent_sequence {
            return Err(DirectSnowStage3V11AttachmentError::Identity(
                "parent receipt ending next-sequence join",
            ));
        }
        if ending.v11_parent_state.checkpoint() != self.ending_v11_parent_state.checkpoint() {
            return Err(DirectSnowStage3V11AttachmentError::Identity(
                "parent receipt ending V11 checkpoint join",
            ));
        }
        if !ending
            .last_v11_parent_candidate
            .as_ref()
            .zip(self.ending_last_v11_parent_candidate.as_ref())
            .map_or(
                ending.last_v11_parent_candidate.is_none()
                    && self.ending_last_v11_parent_candidate.is_none(),
                |(ending_candidate, receipt_candidate)| {
                    ending_candidate.has_same_checkpoint_authority(receipt_candidate)
                },
            )
        {
            return Err(DirectSnowStage3V11AttachmentError::Identity(
                "parent receipt ending V11 candidate authority join",
            ));
        }
        Ok(())
    }
}

#[cfg(test)]
mod wb14_parent_finalization_placement_tests {
    use super::*;

    fn support(start_ns: u128, end_ns: u128) -> TimeSupport {
        TimeSupport::new(ModelTimeNs::new(start_ns), ModelTimeNs::new(end_ns))
            .expect("positive support")
    }

    #[test]
    fn nonfinal_and_final_subslabs_bind_replay_to_exact_parent_end() {
        let parent = support(0, 1_800_000_000_000);
        let nonfinal = support(0, 60_000_000_000);
        let final_subslab = support(1_740_000_000_000, 1_800_000_000_000);

        assert!(wb14_parent_finalization_placement_is_valid_v1(
            parent, nonfinal, false,
        ));
        assert!(wb14_parent_finalization_placement_is_valid_v1(
            parent,
            final_subslab,
            true,
        ));
        assert!(
            !wb14_parent_finalization_placement_is_valid_v1(parent, nonfinal, true),
            "nonfinal subslab must not carry parent replay",
        );
        assert!(
            !wb14_parent_finalization_placement_is_valid_v1(parent, final_subslab, false),
            "parent-end subslab must carry parent replay",
        );
    }

    #[test]
    fn restarted_active_prefix_may_end_before_snow_free_parent_finalization() {
        let parent = support(5_400_000_000_000, 7_200_000_000_000);
        let restored_last_covered = support(6_240_000_000_000, 6_300_000_000_000);
        assert!(wb14_parent_finalization_placement_is_valid_v1(
            parent,
            restored_last_covered,
            false,
        ));
        assert!(
            !wb14_parent_finalization_placement_is_valid_v1(parent, restored_last_covered, true,),
            "restart must not move snow-free parent finalization onto the covered prefix",
        );
    }
}

#[cfg(test)]
#[path = "snow_stage3_v11_terminal_event_ordinal_tests.rs"]
mod terminal_event_ordinal_tests;
#[derive(Clone, Debug, PartialEq, serde::Deserialize, Serialize)]
pub struct Stage3SupportLiquidCustodyV2 {
    pub schema: String,
    pub subslab_receipt_sha256: Digest32,
    pub parent_transaction_sha256: Digest32,
    pub support: TimeSupport,
    pub accepted_slab_sha256: Digest32,
    pub output_set_sha256: Digest32,
    pub receiver_event: AcceptedEventReceiptV1,
    pub surface_beginning_state: crate::DirectSurfaceLiquidOwnedState,
    pub surface_ending_state: crate::DirectSurfaceLiquidOwnedState,
    pub lse_beginning_state: openwepp_land_surface_energy::LandSurfaceEnergyState,
    pub lse_ending_state: openwepp_land_surface_energy::LandSurfaceEnergyState,
    pub receiver_receipt_set_sha256: Digest32,
    pub receiver_receipts: Vec<crate::DirectZeroDurationSnowLiquidReceiptV1>,
    pub custody_sha256: Digest32,
}

impl Stage3SupportLiquidCustodyV2 {
    fn reconstructed_digest(&self) -> Result<Digest32, DirectSnowStage3V11AttachmentError> {
        let mut value = self.clone();
        value.custody_sha256 = Digest32::zero();
        let bytes = serde_json::to_vec(&value).map_err(|_| {
            DirectSnowStage3V11AttachmentError::Identity("support-liquid custody V2 serialization")
        })?;
        Ok(digest_bytes(&bytes))
    }

    pub fn seal(
        subslab: &Stage3CoupledSubslabReceiptV1,
        lse_beginning_state: openwepp_land_surface_energy::LandSurfaceEnergyState,
        lse_ending_state: openwepp_land_surface_energy::LandSurfaceEnergyState,
        receiver_receipt_set_sha256: Digest32,
        receiver_receipts: Vec<crate::DirectZeroDurationSnowLiquidReceiptV1>,
    ) -> Result<Self, DirectSnowStage3V11AttachmentError> {
        let mut value = Self {
            schema: "openwepp.stage3-support-liquid-custody.v2".to_owned(),
            subslab_receipt_sha256: subslab.receipt_sha256,
            parent_transaction_sha256: subslab.owner_join.parent_transaction_sha256,
            support: subslab.support,
            accepted_slab_sha256: subslab.accepted_slab_sha256,
            output_set_sha256: subslab.post_support_liquid_output_set_sha256.ok_or(
                DirectSnowStage3V11AttachmentError::Identity(
                    "support-liquid custody V2 output set",
                ),
            )?,
            receiver_event: subslab.post_support_liquid_receiver_event.clone().ok_or(
                DirectSnowStage3V11AttachmentError::Identity(
                    "support-liquid custody V2 receiver event",
                ),
            )?,
            surface_beginning_state: subslab
                .post_support_liquid_surface_beginning_state
                .clone()
                .ok_or(DirectSnowStage3V11AttachmentError::Identity(
                    "support-liquid custody V2 beginning surface",
                ))?,
            surface_ending_state: subslab
                .post_support_liquid_surface_ending_state
                .clone()
                .ok_or(DirectSnowStage3V11AttachmentError::Identity(
                    "support-liquid custody V2 ending surface",
                ))?,
            lse_beginning_state,
            lse_ending_state,
            receiver_receipt_set_sha256,
            receiver_receipts,
            custody_sha256: Digest32::zero(),
        };
        value.custody_sha256 = value.reconstructed_digest()?;
        value.validate(subslab)?;
        Ok(value)
    }

    pub fn validate(
        &self,
        subslab: &Stage3CoupledSubslabReceiptV1,
    ) -> Result<(), DirectSnowStage3V11AttachmentError> {
        self.receiver_event.validate().map_err(|_| {
            DirectSnowStage3V11AttachmentError::Identity("support-liquid custody V2 event seal")
        })?;
        let reconstructed_receipt_set =
            crate::zero_duration_snow_liquid_receipt_set_sha256(&self.receiver_receipts)
                .map(Digest32::from_bytes)
                .map_err(|_| {
                    DirectSnowStage3V11AttachmentError::Identity(
                        "support-liquid custody V2 typed receipt set",
                    )
                })?;
        let receiver_ledger = LedgerEntryV1::new(
            "positive-support-snow-liquid-receiver".to_owned(),
            "kg-m-2-and-j-m-2-ofe-ground".to_owned(),
            self.output_set_sha256,
            self.output_set_sha256,
            reconstructed_receipt_set,
        )?;
        self.receiver_event
            .validate_ledger_entries(&[receiver_ledger])
            .map_err(|_| {
                DirectSnowStage3V11AttachmentError::Identity(
                    "support-liquid custody V2 event/receipt-set ledger join",
                )
            })?;
        if self.schema != "openwepp.stage3-support-liquid-custody.v2"
            || self.custody_sha256 == Digest32::zero()
            || self.custody_sha256 != self.reconstructed_digest()?
            || self.subslab_receipt_sha256 != subslab.receipt_sha256
            || self.parent_transaction_sha256 != subslab.owner_join.parent_transaction_sha256
            || self.support != subslab.support
            || self.accepted_slab_sha256 != subslab.accepted_slab_sha256
            || Some(self.output_set_sha256) != subslab.post_support_liquid_output_set_sha256
            || Some(&self.receiver_event) != subslab.post_support_liquid_receiver_event.as_ref()
            || Some(&self.surface_beginning_state)
                != subslab.post_support_liquid_surface_beginning_state.as_ref()
            || Some(&self.surface_ending_state)
                != subslab.post_support_liquid_surface_ending_state.as_ref()
            || self.receiver_receipt_set_sha256 != reconstructed_receipt_set
            || self.receiver_event.beginning_owner_set_digest()
                != subslab.owner_join.ending_complete_owner_set_sha256
            || self.receiver_event.tick() != self.support.end_ns()
        {
            return Err(DirectSnowStage3V11AttachmentError::Identity(
                "support-liquid custody V2 identity join",
            ));
        }
        let event_ordinal = self.receiver_event.ordinal();
        let event_context = self.receiver_event.event_context_digest();
        if self.receiver_receipts.iter().any(|receipt| {
            receipt.output_set_sha256 != *self.output_set_sha256.as_bytes()
                || receipt.predecessor_owner_set_sha256
                    != *self.receiver_event.beginning_owner_set_digest().as_bytes()
                || receipt.receiver_context_sha256 != *event_context.as_bytes()
                || receipt.support_start_ns != self.support.start_ns().get()
                || receipt.support_end_ns != self.support.end_ns().get()
                || receipt.receiver_ordinal != event_ordinal
        }) {
            return Err(DirectSnowStage3V11AttachmentError::Identity(
                "support-liquid custody V2 receipt/event join",
            ));
        }
        let mut first_hop_mass = 0.0_f64;
        let mut first_hop_enthalpy = 0.0_f64;
        for receipt in self
            .receiver_receipts
            .iter()
            .filter(|receipt| receipt.basis_ofe_id == receipt.origin_ofe_id)
        {
            first_hop_mass += receipt.mass_kg_m2_basis_ofe_ground;
            first_hop_enthalpy += receipt.sensible_enthalpy_j_m2_basis_ofe_ground;
        }
        let expected_mass = f64::from_bits(subslab.post_support_liquid_mass_kg_m2_bits.ok_or(
            DirectSnowStage3V11AttachmentError::Identity(
                "support-liquid custody V2 output mass omission",
            ),
        )?);
        let expected_enthalpy =
            f64::from_bits(subslab.post_support_liquid_enthalpy_j_m2_bits.ok_or(
                DirectSnowStage3V11AttachmentError::Identity(
                    "support-liquid custody V2 output enthalpy omission",
                ),
            )?);
        if !first_hop_mass.is_finite()
            || !first_hop_enthalpy.is_finite()
            || first_hop_mass.to_bits() != expected_mass.to_bits()
            || first_hop_enthalpy.to_bits() != expected_enthalpy.to_bits()
        {
            return Err(DirectSnowStage3V11AttachmentError::Identity(
                "support-liquid custody V2 first-hop mass/enthalpy closure",
            ));
        }
        let mut retained_mass = BTreeMap::<(OfeId, TileId), f64>::new();
        let mut retained_enthalpy = BTreeMap::<(OfeId, TileId), f64>::new();
        for receipt in &self.receiver_receipts {
            if receipt.disposition
                == crate::direct_runtime::DirectZeroDurationSnowLiquidDispositionV1::RetainedSurface
            {
                let tile = receipt.recipient_tile_id.clone().ok_or(
                    DirectSnowStage3V11AttachmentError::Identity(
                        "support-liquid custody V2 retained recipient",
                    ),
                )?;
                let credited_mass = receipt.credited_mass_kg_m2_recipient_tile_ground.ok_or(
                    DirectSnowStage3V11AttachmentError::Identity(
                        "support-liquid custody V2 retained mass credit",
                    ),
                )?;
                let credited_enthalpy = receipt
                    .credited_enthalpy_j_m2_recipient_tile_ground
                    .ok_or(DirectSnowStage3V11AttachmentError::Identity(
                        "support-liquid custody V2 retained enthalpy credit",
                    ))?;
                *retained_mass
                    .entry((receipt.recipient_ofe_id.clone(), tile.clone()))
                    .or_default() += credited_mass;
                *retained_enthalpy
                    .entry((receipt.recipient_ofe_id.clone(), tile))
                    .or_default() += credited_enthalpy;
            }
        }
        for beginning_surface in &self.surface_beginning_state.records {
            let ending_surface = self
                .surface_ending_state
                .records
                .iter()
                .find(|record| record.key == beginning_surface.key)
                .ok_or(DirectSnowStage3V11AttachmentError::Identity(
                    "support-liquid custody V2 surface topology",
                ))?;
            let beginning_lse = self
                .lse_beginning_state
                .tiles
                .iter()
                .find(|tile| {
                    tile.ofe_id == beginning_surface.key.ofe_id
                        && tile.tile_id == beginning_surface.key.tile_id
                })
                .ok_or(DirectSnowStage3V11AttachmentError::Identity(
                    "support-liquid custody V2 beginning LSE topology",
                ))?;
            let ending_lse = self
                .lse_ending_state
                .tiles
                .iter()
                .find(|tile| {
                    tile.ofe_id == beginning_surface.key.ofe_id
                        && tile.tile_id == beginning_surface.key.tile_id
                })
                .ok_or(DirectSnowStage3V11AttachmentError::Identity(
                    "support-liquid custody V2 LSE topology",
                ))?;
            let mass_tile = retained_mass
                .get(&(
                    beginning_surface.key.ofe_id.clone(),
                    beginning_surface.key.tile_id.clone(),
                ))
                .copied()
                .unwrap_or(0.0);
            let enthalpy_tile = retained_enthalpy
                .get(&(
                    beginning_surface.key.ofe_id.clone(),
                    beginning_surface.key.tile_id.clone(),
                ))
                .copied()
                .unwrap_or(0.0);
            let expected_surface = beginning_surface.liquid_kg_m2_tile + mass_tile;
            let expected_lse = beginning_lse.surface_enthalpy_j_m2_tile_ground + enthalpy_tile;
            if !mass_tile.is_finite()
                || !enthalpy_tile.is_finite()
                || mass_tile < 0.0
                || enthalpy_tile < 0.0
                || expected_surface.to_bits() != ending_surface.liquid_kg_m2_tile.to_bits()
                || expected_lse.to_bits() != ending_lse.surface_enthalpy_j_m2_tile_ground.to_bits()
            {
                return Err(DirectSnowStage3V11AttachmentError::Identity(
                    "support-liquid custody V2 mass/enthalpy closure",
                ));
            }
        }
        Ok(())
    }
}

#[derive(Clone, Debug, PartialEq, serde::Deserialize, Serialize)]
pub struct Stage3CoupledSubslabReceiptV1 {
    pub parent_support: TimeSupport,
    pub support: TimeSupport,
    pub selected_upper_bound_s_bits: u64,
    pub accepted_slab_sha256: Digest32,
    /// Identity sealed into the unpublished WB14 trial replay. Adaptive
    /// acceptance retains that exact trial lineage while the coupled-time
    /// slab has its own publication identity.
    pub wb14_replay_trial_sha256: Digest32,
    pub wb14_replay_beginning_owner_set_sha256: Digest32,
    pub wb14_child_receipt_set_sha256: Digest32,
    pub wb14_parent_receipt_set_sha256: Option<Digest32>,
    pub wb14_child_replay_bytes: Vec<u8>,
    pub wb14_parent_replay_bytes: Option<Vec<u8>>,
    pub destination_receipts: BTreeMap<(OfeId, TileId), FinalStage3TileBoundaryReceiptV1>,
    pub lane_receipts: BTreeMap<u32, LaneStage3BoundaryReceiptV1>,
    pub physical_outcome_ledger_set_sha256: Digest32,
    pub terminal_events: BTreeMap<u32, DirectSnowTerminalEventResult>,
    /// Accepted same-tick bridge from the physical slab to the next owner.
    pub post_support_liquid_receiver_event: Option<AcceptedEventReceiptV1>,
    pub post_support_liquid_output_set_sha256: Option<Digest32>,
    pub post_support_liquid_mass_kg_m2_bits: Option<u64>,
    pub post_support_liquid_enthalpy_j_m2_bits: Option<u64>,
    pub post_support_liquid_surface_beginning_state: Option<crate::DirectSurfaceLiquidOwnedState>,
    pub post_support_liquid_surface_ending_state: Option<crate::DirectSurfaceLiquidOwnedState>,
    #[serde(skip)]
    pub post_support_liquid_custody_v2: Option<Stage3SupportLiquidCustodyV2>,
    pub owner_join: CoveredParentOwnerJoinReceiptV1,
    pub receipt_sha256: Digest32,
}

impl Stage3CoupledSubslabReceiptV1 {
    pub fn support_liquid_custody_v2(&self) -> Option<&Stage3SupportLiquidCustodyV2> {
        self.post_support_liquid_custody_v2.as_ref()
    }

    pub fn install_support_liquid_custody_v2(
        &mut self,
        custody: Stage3SupportLiquidCustodyV2,
    ) -> Result<(), DirectSnowStage3V11AttachmentError> {
        if self.post_support_liquid_custody_v2.is_some() {
            return Err(DirectSnowStage3V11AttachmentError::Identity(
                "duplicate support-liquid custody V2 install",
            ));
        }
        custody.validate(self)?;
        self.post_support_liquid_custody_v2 = Some(custody);
        Ok(())
    }

    pub fn validate_support_liquid_custody_v2(
        &self,
    ) -> Result<(), DirectSnowStage3V11AttachmentError> {
        match (
            self.post_support_liquid_receiver_event.is_some(),
            &self.post_support_liquid_custody_v2,
        ) {
            (true, Some(custody)) => custody.validate(self),
            (false, None) => Ok(()),
            _ => Err(DirectSnowStage3V11AttachmentError::Identity(
                "support-liquid custody V2 all-or-none",
            )),
        }
    }
    #[must_use]
    pub(crate) fn effective_ending_complete_owner_set_sha256(&self) -> Digest32 {
        self.post_support_liquid_receiver_event
            .as_ref()
            .map_or(self.owner_join.ending_complete_owner_set_sha256, |event| {
                event.ending_owner_set_digest()
            })
    }

    fn reconstructed_digest(&self) -> Result<Digest32, DirectSnowStage3V11AttachmentError> {
        self.owner_join.validate_seal().map_err(|_| {
            DirectSnowStage3V11AttachmentError::Identity("covered subslab owner-join seal")
        })?;
        self.owner_join
            .validate_retained_boundary_sets(&self.destination_receipts, &self.lane_receipts)
            .map_err(|_| {
                DirectSnowStage3V11AttachmentError::Identity(
                    "covered subslab retained boundary sets",
                )
            })?;
        let replay_binding = crate::direct_runtime::wb14_child_replay_binding(
            &self.wb14_child_replay_bytes,
        )
        .map_err(|_| {
            DirectSnowStage3V11AttachmentError::Identity(
                "covered subslab WB14 replay binding",
            )
        })?;
        if replay_binding.child_support_start_ns < self.support.start_ns().get()
            || replay_binding.child_support_end_ns != self.support.end_ns().get()
            || replay_binding.child_support_start_ns >= replay_binding.child_support_end_ns
        {
            return Err(DirectSnowStage3V11AttachmentError::Identity(
                "covered subslab WB14 physical-child support",
            ));
        }
        crate::direct_runtime::validate_wb14_child_replay_binding(
            &self.wb14_child_replay_bytes,
            crate::direct_runtime::DirectWb14CoupledChildBindingV1 {
                proposed_upper_bound_s_bits: self.selected_upper_bound_s_bits,
                coupled_parent_transaction_sha256: *self
                    .owner_join
                    .parent_transaction_sha256
                    .as_bytes(),
                accepted_slab_sha256: *self.wb14_replay_trial_sha256.as_bytes(),
                parent_beginning_complete_owner_set_sha256: *self
                    .wb14_replay_beginning_owner_set_sha256
                    .as_bytes(),
                parent_support_start_ns: self.parent_support.start_ns().get(),
                parent_support_end_ns: self.parent_support.end_ns().get(),
                child_support_start_ns: replay_binding.child_support_start_ns,
                child_support_end_ns: replay_binding.child_support_end_ns,
            },
        )
        .map_err(|_| {
            DirectSnowStage3V11AttachmentError::Identity("covered subslab WB14 replay/coupled join")
        })?;
        if let Some(parent_bytes) = &self.wb14_parent_replay_bytes {
            crate::direct_runtime::validate_wb14_parent_replay(
                &self.wb14_child_replay_bytes,
                parent_bytes,
            )
            .map_err(|_| {
                DirectSnowStage3V11AttachmentError::Identity(
                    "covered subslab WB14 parent finalization replay",
                )
            })?;
        }
        if self.accepted_slab_sha256 != self.owner_join.accepted_slab_sha256
            || self.wb14_child_receipt_set_sha256 != self.owner_join.wb14_child_receipt_set_sha256
            || self.wb14_parent_receipt_set_sha256 != self.owner_join.wb14_parent_receipt_set_sha256
            || self.support != self.owner_join.support
            || digest_bytes(&self.wb14_child_replay_bytes) != self.wb14_child_receipt_set_sha256
            || self
                .wb14_parent_replay_bytes
                .as_ref()
                .map(|bytes| digest_bytes(bytes))
                != self.wb14_parent_receipt_set_sha256
            || f64::from_bits(self.support.duration_s_bits())
                > f64::from_bits(self.selected_upper_bound_s_bits)
        {
            return Err(DirectSnowStage3V11AttachmentError::Identity(
                "covered subslab semantic join",
            ));
        }
        let mut bytes = Vec::new();
        bytes.extend_from_slice(b"openwepp.stage3-coupled-subslab-receipt.v1\0");
        bytes.extend_from_slice(&self.parent_support.start_ns().get().to_be_bytes());
        bytes.extend_from_slice(&self.parent_support.end_ns().get().to_be_bytes());
        bytes.extend_from_slice(&self.support.start_ns().get().to_be_bytes());
        bytes.extend_from_slice(&self.support.end_ns().get().to_be_bytes());
        bytes.extend_from_slice(&self.selected_upper_bound_s_bits.to_be_bytes());
        bytes.extend_from_slice(self.accepted_slab_sha256.as_bytes());
        bytes.extend_from_slice(self.wb14_replay_trial_sha256.as_bytes());
        bytes.extend_from_slice(self.wb14_replay_beginning_owner_set_sha256.as_bytes());
        bytes.extend_from_slice(self.wb14_child_receipt_set_sha256.as_bytes());
        match self.wb14_parent_receipt_set_sha256 {
            Some(digest) => {
                bytes.push(1);
                bytes.extend_from_slice(digest.as_bytes());
            }
            None => bytes.push(0),
        }
        bytes.extend_from_slice(self.owner_join.receipt_sha256.as_bytes());
        if self.physical_outcome_ledger_set_sha256 == Digest32::zero() {
            return Err(DirectSnowStage3V11AttachmentError::Identity(
                "physical outcome ledger set",
            ));
        }
        bytes.extend_from_slice(self.physical_outcome_ledger_set_sha256.as_bytes());
        let terminal_event_bytes = serde_json::to_vec(&self.terminal_events).map_err(|_| {
            DirectSnowStage3V11AttachmentError::Identity("terminal event receipt bytes")
        })?;
        bytes.extend_from_slice(digest_bytes(&terminal_event_bytes).as_bytes());
        match (
            &self.post_support_liquid_receiver_event,
            self.post_support_liquid_output_set_sha256,
            self.post_support_liquid_mass_kg_m2_bits,
            self.post_support_liquid_enthalpy_j_m2_bits,
            &self.post_support_liquid_surface_beginning_state,
            &self.post_support_liquid_surface_ending_state,
        ) {
            (
                Some(event),
                Some(output_set),
                Some(mass_bits),
                Some(enthalpy_bits),
                Some(surface_beginning),
                Some(surface_ending),
            ) => {
                event.validate().map_err(|_| {
                    DirectSnowStage3V11AttachmentError::Identity(
                        "post-support liquid receiver event seal",
                    )
                })?;
                if event.tick() != self.support.end_ns()
                    || event.parent_transaction_id().digest()
                        != self.owner_join.parent_transaction_sha256
                    || event.beginning_owner_set_digest()
                        != self.owner_join.ending_complete_owner_set_sha256
                {
                    return Err(DirectSnowStage3V11AttachmentError::Identity(
                        "post-support liquid receiver event chronology",
                    ));
                }
                let context = framed_sha256(
                    "stage3-v11-positive-support-liquid-receiver",
                    &[
                        FramedField {
                            tag: "parent_transaction",
                            value: self.owner_join.parent_transaction_sha256.as_bytes(),
                        },
                        FramedField {
                            tag: "support_start",
                            value: &self.support.start_ns().get().to_be_bytes(),
                        },
                        FramedField {
                            tag: "support_end",
                            value: &self.support.end_ns().get().to_be_bytes(),
                        },
                        FramedField {
                            tag: "support_ending_owner",
                            value: self.owner_join.ending_complete_owner_set_sha256.as_bytes(),
                        },
                        FramedField {
                            tag: "output_set",
                            value: output_set.as_bytes(),
                        },
                        FramedField {
                            tag: "mass_kg_m2",
                            value: &mass_bits.to_be_bytes(),
                        },
                        FramedField {
                            tag: "enthalpy_j_m2",
                            value: &enthalpy_bits.to_be_bytes(),
                        },
                    ],
                )?;
                if event.event_context_digest() != context
                    || !f64::from_bits(mass_bits).is_finite()
                    || f64::from_bits(mass_bits) <= 0.0
                    || !f64::from_bits(enthalpy_bits).is_finite()
                {
                    return Err(DirectSnowStage3V11AttachmentError::Identity(
                        "post-support liquid receiver custody",
                    ));
                }
                bytes.push(1);
                bytes.extend_from_slice(event.id().digest().as_bytes());
                bytes.extend_from_slice(event.ending_owner_set_digest().as_bytes());
                bytes.extend_from_slice(output_set.as_bytes());
                bytes.extend_from_slice(&mass_bits.to_be_bytes());
                bytes.extend_from_slice(&enthalpy_bits.to_be_bytes());
                let surface_states = serde_json::to_vec(&(surface_beginning, surface_ending))
                    .map_err(|_| {
                        DirectSnowStage3V11AttachmentError::Identity(
                            "post-support liquid receiver surface state serialization",
                        )
                    })?;
                bytes.extend_from_slice(digest_bytes(&surface_states).as_bytes());
            }
            (None, None, None, None, None, None) => bytes.push(0),
            _ => {
                return Err(DirectSnowStage3V11AttachmentError::Identity(
                    "post-support liquid receiver receipt completeness",
                ));
            }
        }
        Ok(digest_bytes(&bytes))
    }

    pub fn validate(&self) -> Result<(), DirectSnowStage3V11AttachmentError> {
        if self.receipt_sha256 != self.reconstructed_digest()? {
            return Err(DirectSnowStage3V11AttachmentError::Identity(
                "covered subslab receipt seal",
            ));
        }
        Ok(())
    }
}

include!("snow_stage3_v11_integrated_boundary_ledger.rs");
include!("snow_stage3_v11_adaptive_receipt_identity.rs");

/// Immutable identity shared by every receipt for one adaptive candidate.
#[derive(Clone, Copy, Debug, Eq, PartialEq, serde::Deserialize, Serialize)]
pub struct Stage3AdaptiveReceiptContextV1 {
    pub parent_transaction_id: ParentTransactionId,
    pub parent_support: TimeSupport,
    pub step_support: TimeSupport,
    pub step_ordinal: u32,
    pub attempt_ordinal: u32,
    pub beginning_complete_owner_set_sha256: Digest32,
    pub forcing_projection_sha256: Digest32,
    pub topology_sha256: Digest32,
    pub configuration_sha256: Digest32,
}

impl Stage3AdaptiveReceiptContextV1 {
    fn validate(self) -> Result<(), DirectSnowStage3V11AttachmentError> {
        let parent_start = self.parent_support.start_ns().get();
        let parent_end = self.parent_support.end_ns().get();
        let step_start = self.step_support.start_ns().get();
        let step_end = self.step_support.end_ns().get();
        if step_start < parent_start
            || step_end > parent_end
            || self.parent_support.duration_ns() % STAGE3_ADAPTIVE_MINIMUM_STEP_NS != 0
            || (step_start - parent_start) % STAGE3_ADAPTIVE_MINIMUM_STEP_NS != 0
            || self.step_support.duration_ns() % STAGE3_ADAPTIVE_MINIMUM_STEP_NS != 0
        {
            return Err(adaptive_receipt_identity_error(
                "adaptive receipt support grid",
            ));
        }
        require_adaptive_digest(
            self.parent_transaction_id.digest(),
            "adaptive receipt parent transaction",
        )?;
        require_adaptive_digest(
            self.beginning_complete_owner_set_sha256,
            "adaptive receipt beginning owner set",
        )?;
        require_adaptive_digest(
            self.forcing_projection_sha256,
            "adaptive receipt forcing projection",
        )?;
        require_adaptive_digest(self.topology_sha256, "adaptive receipt topology")?;
        require_adaptive_digest(self.configuration_sha256, "adaptive receipt configuration")
    }

    fn fields(self) -> Vec<(&'static str, Vec<u8>)> {
        vec![
            (
                "parent_transaction_id",
                self.parent_transaction_id.digest().as_bytes().to_vec(),
            ),
            (
                "parent_support_start_ns",
                self.parent_support.start_ns().get().to_be_bytes().to_vec(),
            ),
            (
                "parent_support_end_ns",
                self.parent_support.end_ns().get().to_be_bytes().to_vec(),
            ),
            (
                "step_support_start_ns",
                self.step_support.start_ns().get().to_be_bytes().to_vec(),
            ),
            (
                "step_support_end_ns",
                self.step_support.end_ns().get().to_be_bytes().to_vec(),
            ),
            ("step_ordinal", self.step_ordinal.to_be_bytes().to_vec()),
            (
                "attempt_ordinal",
                self.attempt_ordinal.to_be_bytes().to_vec(),
            ),
            (
                "beginning_complete_owner_set_sha256",
                self.beginning_complete_owner_set_sha256.as_bytes().to_vec(),
            ),
            (
                "forcing_projection_sha256",
                self.forcing_projection_sha256.as_bytes().to_vec(),
            ),
            ("topology_sha256", self.topology_sha256.as_bytes().to_vec()),
            (
                "configuration_sha256",
                self.configuration_sha256.as_bytes().to_vec(),
            ),
        ]
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, serde::Deserialize, Serialize)]
#[repr(u8)]
pub enum Stage3AdaptiveTrialDispositionV1 {
    Closed = 0,
    TypedRejected = 1,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, serde::Deserialize, Serialize)]
#[repr(u8)]
pub enum Stage3AdaptiveEventPostureV1 {
    NoEvent = 0,
    TerminalEvent = 1,
    PendingParcel = 2,
    ConsumedParcel = 3,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, serde::Deserialize, Serialize)]
#[repr(u8)]
pub enum Stage3AdaptiveStepDecisionV1 {
    ComposedAccepted = 0,
    RefineRejected = 1,
    FloorAccepted = 2,
    FloorRejected = 3,
}

impl Stage3AdaptiveStepDecisionV1 {
    const fn is_accepted(self) -> bool {
        matches!(self, Self::ComposedAccepted | Self::FloorAccepted)
    }

    const fn is_floor(self) -> bool {
        matches!(self, Self::FloorAccepted | Self::FloorRejected)
    }
}

#[derive(Clone, Debug, Eq, PartialEq, serde::Deserialize, Serialize)]
pub struct Stage3AdaptiveParentRequestReceiptV1 {
    pub context: Stage3AdaptiveReceiptContextV1,
    pub minimum_step_ns: u128,
    pub proposed_step_quanta: u128,
    pub receipt_sha256: Digest32,
}

impl Stage3AdaptiveParentRequestReceiptV1 {
    pub fn try_new(
        context: Stage3AdaptiveReceiptContextV1,
        proposed_step_quanta: u128,
    ) -> Result<Self, DirectSnowStage3V11AttachmentError> {
        let mut value = Self {
            context,
            minimum_step_ns: STAGE3_ADAPTIVE_MINIMUM_STEP_NS,
            proposed_step_quanta,
            receipt_sha256: Digest32::zero(),
        };
        value.receipt_sha256 = value.reconstructed_digest()?;
        Ok(value)
    }

    fn reconstructed_digest(&self) -> Result<Digest32, DirectSnowStage3V11AttachmentError> {
        self.context.validate()?;
        if self.minimum_step_ns != STAGE3_ADAPTIVE_MINIMUM_STEP_NS
            || self.proposed_step_quanta == 0
            || self.proposed_step_quanta.checked_mul(self.minimum_step_ns)
                != Some(self.context.step_support.duration_ns())
        {
            return Err(adaptive_receipt_identity_error(
                "adaptive parent request proposal",
            ));
        }
        let mut fields = self.context.fields();
        fields.extend([
            (
                "minimum_step_ns",
                self.minimum_step_ns.to_be_bytes().to_vec(),
            ),
            (
                "proposed_step_quanta",
                self.proposed_step_quanta.to_be_bytes().to_vec(),
            ),
        ]);
        adaptive_framed_sha256("stage3-adaptive-parent-request-v1", fields)
    }

    pub fn validate(&self) -> Result<(), DirectSnowStage3V11AttachmentError> {
        if self.receipt_sha256 != self.reconstructed_digest()? {
            return Err(adaptive_receipt_identity_error(
                "adaptive parent request seal",
            ));
        }
        Ok(())
    }
}

#[derive(Clone, Debug, Eq, PartialEq, serde::Deserialize, Serialize)]
pub struct Stage3AdaptiveDirectTrialReceiptV1 {
    pub context: Stage3AdaptiveReceiptContextV1,
    pub parent_request_sha256: Digest32,
    pub physical_ledger_sha256: Digest32,
    pub ending_complete_owner_set_sha256: Digest32,
    pub phase_result_sha256: Digest32,
    pub event_posture: Stage3AdaptiveEventPostureV1,
    pub disposition: Stage3AdaptiveTrialDispositionV1,
    pub receipt_sha256: Digest32,
}

impl Stage3AdaptiveDirectTrialReceiptV1 {
    #[allow(clippy::too_many_arguments)]
    pub fn try_new(
        request: &Stage3AdaptiveParentRequestReceiptV1,
        physical_ledger_sha256: Digest32,
        ending_complete_owner_set_sha256: Digest32,
        phase_result_sha256: Digest32,
        event_posture: Stage3AdaptiveEventPostureV1,
        disposition: Stage3AdaptiveTrialDispositionV1,
    ) -> Result<Self, DirectSnowStage3V11AttachmentError> {
        request.validate()?;
        let mut value = Self {
            context: request.context,
            parent_request_sha256: request.receipt_sha256,
            physical_ledger_sha256,
            ending_complete_owner_set_sha256,
            phase_result_sha256,
            event_posture,
            disposition,
            receipt_sha256: Digest32::zero(),
        };
        value.receipt_sha256 = value.reconstructed_digest()?;
        Ok(value)
    }

    fn reconstructed_digest(&self) -> Result<Digest32, DirectSnowStage3V11AttachmentError> {
        self.context.validate()?;
        for (digest, reason) in [
            (self.parent_request_sha256, "adaptive direct parent request"),
            (
                self.physical_ledger_sha256,
                "adaptive direct physical ledger",
            ),
            (
                self.ending_complete_owner_set_sha256,
                "adaptive direct ending owner set",
            ),
            (self.phase_result_sha256, "adaptive direct phase result"),
        ] {
            require_adaptive_digest(digest, reason)?;
        }
        let mut fields = self.context.fields();
        fields.extend([
            (
                "parent_request_sha256",
                self.parent_request_sha256.as_bytes().to_vec(),
            ),
            (
                "physical_ledger_sha256",
                self.physical_ledger_sha256.as_bytes().to_vec(),
            ),
            (
                "ending_complete_owner_set_sha256",
                self.ending_complete_owner_set_sha256.as_bytes().to_vec(),
            ),
            (
                "phase_result_sha256",
                self.phase_result_sha256.as_bytes().to_vec(),
            ),
            ("event_posture", vec![self.event_posture as u8]),
            ("trial_disposition", vec![self.disposition as u8]),
        ]);
        adaptive_framed_sha256("stage3-adaptive-direct-trial-v1", fields)
    }

    pub fn validate_against(
        &self,
        request: &Stage3AdaptiveParentRequestReceiptV1,
    ) -> Result<(), DirectSnowStage3V11AttachmentError> {
        request.validate()?;
        if self.receipt_sha256 != self.reconstructed_digest()?
            || self.context != request.context
            || self.parent_request_sha256 != request.receipt_sha256
        {
            return Err(adaptive_receipt_identity_error(
                "adaptive direct trial lineage",
            ));
        }
        Ok(())
    }
}

#[derive(Clone, Debug, Eq, PartialEq, serde::Deserialize, Serialize)]
pub struct Stage3AdaptiveSplitChildTrialReceiptV1 {
    pub context: Stage3AdaptiveReceiptContextV1,
    pub child_ordinal: u8,
    pub child_support: TimeSupport,
    pub predecessor_receipt_sha256: Digest32,
    pub trial_beginning_complete_owner_set_sha256: Digest32,
    pub physical_ledger_sha256: Digest32,
    pub ending_complete_owner_set_sha256: Digest32,
    pub phase_result_sha256: Digest32,
    pub event_posture: Stage3AdaptiveEventPostureV1,
    pub disposition: Stage3AdaptiveTrialDispositionV1,
    pub receipt_sha256: Digest32,
}

impl Stage3AdaptiveSplitChildTrialReceiptV1 {
    #[allow(clippy::too_many_arguments)]
    fn try_new(
        context: Stage3AdaptiveReceiptContextV1,
        child_ordinal: u8,
        child_support: TimeSupport,
        predecessor_receipt_sha256: Digest32,
        trial_beginning_complete_owner_set_sha256: Digest32,
        physical_ledger_sha256: Digest32,
        ending_complete_owner_set_sha256: Digest32,
        phase_result_sha256: Digest32,
        event_posture: Stage3AdaptiveEventPostureV1,
        disposition: Stage3AdaptiveTrialDispositionV1,
    ) -> Result<Self, DirectSnowStage3V11AttachmentError> {
        let mut value = Self {
            context,
            child_ordinal,
            child_support,
            predecessor_receipt_sha256,
            trial_beginning_complete_owner_set_sha256,
            physical_ledger_sha256,
            ending_complete_owner_set_sha256,
            phase_result_sha256,
            event_posture,
            disposition,
            receipt_sha256: Digest32::zero(),
        };
        value.receipt_sha256 = value.reconstructed_digest()?;
        Ok(value)
    }

    #[allow(clippy::too_many_arguments)]
    pub fn try_child_1(
        request: &Stage3AdaptiveParentRequestReceiptV1,
        direct: &Stage3AdaptiveDirectTrialReceiptV1,
        child_support: TimeSupport,
        physical_ledger_sha256: Digest32,
        ending_complete_owner_set_sha256: Digest32,
        phase_result_sha256: Digest32,
        event_posture: Stage3AdaptiveEventPostureV1,
        disposition: Stage3AdaptiveTrialDispositionV1,
    ) -> Result<Self, DirectSnowStage3V11AttachmentError> {
        direct.validate_against(request)?;
        Self::try_new(
            request.context,
            1,
            child_support,
            direct.receipt_sha256,
            request.context.beginning_complete_owner_set_sha256,
            physical_ledger_sha256,
            ending_complete_owner_set_sha256,
            phase_result_sha256,
            event_posture,
            disposition,
        )
    }

    #[allow(clippy::too_many_arguments)]
    pub fn try_child_2(
        request: &Stage3AdaptiveParentRequestReceiptV1,
        child_1: &Self,
        child_support: TimeSupport,
        physical_ledger_sha256: Digest32,
        ending_complete_owner_set_sha256: Digest32,
        phase_result_sha256: Digest32,
        event_posture: Stage3AdaptiveEventPostureV1,
        disposition: Stage3AdaptiveTrialDispositionV1,
    ) -> Result<Self, DirectSnowStage3V11AttachmentError> {
        request.validate()?;
        child_1.validate()?;
        if child_1.context != request.context || child_1.child_ordinal != 1 {
            return Err(adaptive_receipt_identity_error(
                "adaptive second child first-child lineage",
            ));
        }
        Self::try_new(
            request.context,
            2,
            child_support,
            child_1.receipt_sha256,
            child_1.ending_complete_owner_set_sha256,
            physical_ledger_sha256,
            ending_complete_owner_set_sha256,
            phase_result_sha256,
            event_posture,
            disposition,
        )
    }

    fn reconstructed_digest(&self) -> Result<Digest32, DirectSnowStage3V11AttachmentError> {
        self.context.validate()?;
        let candidate = self.context.step_support;
        if !matches!(self.child_ordinal, 1 | 2)
            || self.child_support.duration_ns() < STAGE3_ADAPTIVE_MINIMUM_STEP_NS
            || self.child_support.duration_ns() % STAGE3_ADAPTIVE_MINIMUM_STEP_NS != 0
            || self.child_support.start_ns() < candidate.start_ns()
            || self.child_support.end_ns() > candidate.end_ns()
        {
            return Err(adaptive_receipt_identity_error(
                "adaptive split child support",
            ));
        }
        for (digest, reason) in [
            (
                self.predecessor_receipt_sha256,
                "adaptive split child predecessor",
            ),
            (
                self.trial_beginning_complete_owner_set_sha256,
                "adaptive split child beginning owner set",
            ),
            (self.physical_ledger_sha256, "adaptive split child ledger"),
            (
                self.ending_complete_owner_set_sha256,
                "adaptive split child ending owner set",
            ),
            (self.phase_result_sha256, "adaptive split child phase"),
        ] {
            require_adaptive_digest(digest, reason)?;
        }
        let mut fields = self.context.fields();
        fields.extend([
            ("child_ordinal", vec![self.child_ordinal]),
            (
                "child_support_start_ns",
                self.child_support.start_ns().get().to_be_bytes().to_vec(),
            ),
            (
                "child_support_end_ns",
                self.child_support.end_ns().get().to_be_bytes().to_vec(),
            ),
            (
                "predecessor_receipt_sha256",
                self.predecessor_receipt_sha256.as_bytes().to_vec(),
            ),
            (
                "trial_beginning_complete_owner_set_sha256",
                self.trial_beginning_complete_owner_set_sha256
                    .as_bytes()
                    .to_vec(),
            ),
            (
                "physical_ledger_sha256",
                self.physical_ledger_sha256.as_bytes().to_vec(),
            ),
            (
                "ending_complete_owner_set_sha256",
                self.ending_complete_owner_set_sha256.as_bytes().to_vec(),
            ),
            (
                "phase_result_sha256",
                self.phase_result_sha256.as_bytes().to_vec(),
            ),
            ("event_posture", vec![self.event_posture as u8]),
            ("trial_disposition", vec![self.disposition as u8]),
        ]);
        adaptive_framed_sha256("stage3-adaptive-split-child-trial-v1", fields)
    }

    pub fn validate(&self) -> Result<(), DirectSnowStage3V11AttachmentError> {
        if self.receipt_sha256 != self.reconstructed_digest()? {
            return Err(adaptive_receipt_identity_error("adaptive split child seal"));
        }
        Ok(())
    }
}

#[derive(Clone, Debug, PartialEq, serde::Deserialize, Serialize)]
pub struct Stage3AdaptiveDiscreteSurfaceReceiptV1 {
    pub owner_id: String,
    pub path: String,
    pub kind: String,
    pub exact_value: String,
}

fn adaptive_discrete_surface_set_sha256_v1(
    surfaces: &[Stage3AdaptiveDiscreteSurfaceReceiptV1],
) -> Result<Digest32, DirectSnowStage3V11AttachmentError> {
    if surfaces.is_empty() {
        return Err(adaptive_receipt_identity_error(
            "adaptive discrete surface empty set",
        ));
    }
    if surfaces.windows(2).any(|pair| {
        (
            pair[0].owner_id.as_str(),
            pair[0].path.as_str(),
            pair[0].kind.as_str(),
            pair[0].exact_value.as_str(),
        ) >= (
            pair[1].owner_id.as_str(),
            pair[1].path.as_str(),
            pair[1].kind.as_str(),
            pair[1].exact_value.as_str(),
        )
    }) {
        return Err(adaptive_receipt_identity_error(
            "adaptive discrete surface ordering",
        ));
    }
    if surfaces.iter().any(|surface| {
        surface.owner_id.is_empty()
            || surface.kind.is_empty()
            || (surface.path.is_empty() && surface.kind != "schema")
    }) {
        return Err(adaptive_receipt_identity_error(
            "adaptive discrete surface identity",
        ));
    }
    for (owner_id, prefix) in [
        ("complete_owner", "adaptive_scalars"),
        ("snow", "pending_terminal_parcels"),
    ] {
        let cardinality_path = format!("{prefix}.cardinality");
        let cardinalities = surfaces
            .iter()
            .filter(|surface| {
                surface.owner_id == owner_id
                    && surface.path == cardinality_path
                    && surface.kind == "membership"
            })
            .collect::<Vec<_>>();
        if cardinalities.len() != 1 {
            return Err(adaptive_receipt_identity_error(
                "adaptive discrete cardinality authority",
            ));
        }
        let expected = cardinalities[0]
            .exact_value
            .parse::<usize>()
            .map_err(|_| adaptive_receipt_identity_error("adaptive discrete cardinality value"))?;
        let ordered_identity_path = format!("{prefix}.ordered_identity_set_sha256");
        let ordered_identity_digests = surfaces
            .iter()
            .filter(|surface| {
                surface.owner_id == owner_id
                    && surface.path == ordered_identity_path
                    && surface.kind == "ordering"
            })
            .collect::<Vec<_>>();
        if ordered_identity_digests.len() != 1 {
            return Err(adaptive_receipt_identity_error(
                "adaptive discrete ordered identity set authority",
            ));
        }
        let (committed_count, ordered_set_sha256) = ordered_identity_digests[0]
            .exact_value
            .split_once(':')
            .ok_or_else(|| {
                adaptive_receipt_identity_error("adaptive discrete ordered identity set encoding")
            })?;
        if committed_count.parse::<usize>().ok() != Some(expected)
            || parse_lower_hex_digest(ordered_set_sha256).is_err()
        {
            return Err(adaptive_receipt_identity_error(
                "adaptive discrete ordered identity set commitment",
            ));
        }
    }
    let mut fields = Vec::with_capacity(surfaces.len());
    for surface in surfaces {
        let receipt = adaptive_framed_sha256(
            "stage3-adaptive-discrete-surface-v1",
            vec![
                ("owner_id", surface.owner_id.as_bytes().to_vec()),
                ("path", surface.path.as_bytes().to_vec()),
                ("kind", surface.kind.as_bytes().to_vec()),
                ("exact_value", surface.exact_value.as_bytes().to_vec()),
            ],
        )?;
        fields.push(("surface", receipt.as_bytes().to_vec()));
    }
    adaptive_framed_sha256("stage3-adaptive-discrete-surface-set-v1", fields)
}

#[derive(Clone, Debug, PartialEq, serde::Deserialize, Serialize)]
pub struct Stage3AdaptiveStepComparisonReceiptV1 {
    pub context: Stage3AdaptiveReceiptContextV1,
    pub predecessor_receipt_sha256: Digest32,
    pub tolerance_policy_id: String,
    pub direct_trial_sha256: Digest32,
    pub split_child_1_sha256: Option<Digest32>,
    pub split_child_2_sha256: Option<Digest32>,
    pub maximum_scaled_error: f64,
    pub discrete_mismatch: bool,
    pub direct_exact_discrete_sha256: Digest32,
    pub composed_exact_discrete_sha256: Option<Digest32>,
    pub direct_exact_discrete_surfaces: Vec<Stage3AdaptiveDiscreteSurfaceReceiptV1>,
    pub composed_exact_discrete_surfaces: Vec<Stage3AdaptiveDiscreteSurfaceReceiptV1>,
    pub decision: Stage3AdaptiveStepDecisionV1,
    pub selected_physical_ledger_sha256: Digest32,
    pub selected_ending_complete_owner_set_sha256: Digest32,
    pub selected_phase_result_sha256: Digest32,
    pub selected_event_posture: Stage3AdaptiveEventPostureV1,
    pub receipt_sha256: Digest32,
}

impl Stage3AdaptiveStepComparisonReceiptV1 {
    pub const TOLERANCE_POLICY_ID: &'static str = "OPENWEPP_STAGE3_ADAPTIVE_OWNER_TOLERANCE_V1";

    pub fn try_composed(
        request: &Stage3AdaptiveParentRequestReceiptV1,
        direct: &Stage3AdaptiveDirectTrialReceiptV1,
        child_1: &Stage3AdaptiveSplitChildTrialReceiptV1,
        child_2: &Stage3AdaptiveSplitChildTrialReceiptV1,
        composed_physical_ledger_sha256: Digest32,
        direct_exact_discrete_sha256: Digest32,
        composed_exact_discrete_sha256: Digest32,
        direct_exact_discrete_surfaces: Vec<Stage3AdaptiveDiscreteSurfaceReceiptV1>,
        composed_exact_discrete_surfaces: Vec<Stage3AdaptiveDiscreteSurfaceReceiptV1>,
        maximum_scaled_error: f64,
        discrete_mismatch: bool,
        accepted: bool,
    ) -> Result<Self, DirectSnowStage3V11AttachmentError> {
        direct.validate_against(request)?;
        child_1.validate()?;
        child_2.validate()?;
        if child_1.context != request.context
            || child_2.context != request.context
            || child_1.child_ordinal != 1
            || child_2.child_ordinal != 2
            || child_1.predecessor_receipt_sha256 != direct.receipt_sha256
            || child_2.predecessor_receipt_sha256 != child_1.receipt_sha256
            || child_1.trial_beginning_complete_owner_set_sha256
                != request.context.beginning_complete_owner_set_sha256
            || child_2.trial_beginning_complete_owner_set_sha256
                != child_1.ending_complete_owner_set_sha256
            || child_1.child_support.start_ns() != request.context.step_support.start_ns()
            || child_1.child_support.end_ns() != child_2.child_support.start_ns()
            || child_2.child_support.end_ns() != request.context.step_support.end_ns()
        {
            return Err(adaptive_receipt_identity_error(
                "adaptive composed trial lineage",
            ));
        }
        let decision = if accepted {
            Stage3AdaptiveStepDecisionV1::ComposedAccepted
        } else {
            Stage3AdaptiveStepDecisionV1::RefineRejected
        };
        let value = Self::seal(Self {
            context: request.context,
            predecessor_receipt_sha256: child_2.receipt_sha256,
            tolerance_policy_id: Self::TOLERANCE_POLICY_ID.to_owned(),
            direct_trial_sha256: direct.receipt_sha256,
            split_child_1_sha256: Some(child_1.receipt_sha256),
            split_child_2_sha256: Some(child_2.receipt_sha256),
            maximum_scaled_error,
            discrete_mismatch,
            direct_exact_discrete_sha256,
            composed_exact_discrete_sha256: Some(composed_exact_discrete_sha256),
            direct_exact_discrete_surfaces,
            composed_exact_discrete_surfaces,
            decision,
            selected_physical_ledger_sha256: composed_physical_ledger_sha256,
            selected_ending_complete_owner_set_sha256: child_2.ending_complete_owner_set_sha256,
            selected_phase_result_sha256: child_2.phase_result_sha256,
            selected_event_posture: child_2.event_posture,
            receipt_sha256: Digest32::zero(),
        })?;
        value.validate_composed_against(request, direct, child_1, child_2)?;
        Ok(value)
    }

    pub fn try_floor(
        request: &Stage3AdaptiveParentRequestReceiptV1,
        direct: &Stage3AdaptiveDirectTrialReceiptV1,
        direct_exact_discrete_sha256: Digest32,
        direct_exact_discrete_surfaces: Vec<Stage3AdaptiveDiscreteSurfaceReceiptV1>,
        accepted: bool,
    ) -> Result<Self, DirectSnowStage3V11AttachmentError> {
        direct.validate_against(request)?;
        if request.context.step_support.duration_ns() != STAGE3_ADAPTIVE_MINIMUM_STEP_NS {
            return Err(adaptive_receipt_identity_error("adaptive floor support"));
        }
        let decision = if accepted {
            Stage3AdaptiveStepDecisionV1::FloorAccepted
        } else {
            Stage3AdaptiveStepDecisionV1::FloorRejected
        };
        let value = Self::seal(Self {
            context: request.context,
            predecessor_receipt_sha256: direct.receipt_sha256,
            tolerance_policy_id: Self::TOLERANCE_POLICY_ID.to_owned(),
            direct_trial_sha256: direct.receipt_sha256,
            split_child_1_sha256: None,
            split_child_2_sha256: None,
            maximum_scaled_error: 0.0,
            discrete_mismatch: false,
            direct_exact_discrete_sha256,
            composed_exact_discrete_sha256: None,
            direct_exact_discrete_surfaces,
            composed_exact_discrete_surfaces: Vec::new(),
            decision,
            selected_physical_ledger_sha256: direct.physical_ledger_sha256,
            selected_ending_complete_owner_set_sha256: direct.ending_complete_owner_set_sha256,
            selected_phase_result_sha256: direct.phase_result_sha256,
            selected_event_posture: direct.event_posture,
            receipt_sha256: Digest32::zero(),
        })?;
        value.validate_floor_against(request, direct)?;
        Ok(value)
    }

    fn seal(mut value: Self) -> Result<Self, DirectSnowStage3V11AttachmentError> {
        value.receipt_sha256 = value.reconstructed_digest()?;
        Ok(value)
    }

    fn reconstructed_digest(&self) -> Result<Digest32, DirectSnowStage3V11AttachmentError> {
        self.context.validate()?;
        if self.tolerance_policy_id != Self::TOLERANCE_POLICY_ID {
            return Err(adaptive_receipt_identity_error(
                "adaptive comparison tolerance policy identity",
            ));
        }
        if !self.maximum_scaled_error.is_finite() || self.maximum_scaled_error < 0.0 {
            return Err(adaptive_receipt_identity_error(
                "adaptive comparison scaled error",
            ));
        }
        require_adaptive_digest(
            self.direct_exact_discrete_sha256,
            "adaptive direct exact discrete",
        )?;
        let direct_surfaces_sha256 =
            adaptive_discrete_surface_set_sha256_v1(&self.direct_exact_discrete_surfaces)?;
        let composed_surfaces_sha256 = if self.decision.is_floor() {
            if self.composed_exact_discrete_sha256.is_some()
                || !self.composed_exact_discrete_surfaces.is_empty()
            {
                return Err(adaptive_receipt_identity_error(
                    "adaptive floor discrete composition",
                ));
            }
            None
        } else {
            let digest = self.composed_exact_discrete_sha256.ok_or_else(|| {
                adaptive_receipt_identity_error("adaptive composed exact discrete")
            })?;
            require_adaptive_digest(digest, "adaptive composed exact discrete")?;
            Some(adaptive_discrete_surface_set_sha256_v1(
                &self.composed_exact_discrete_surfaces,
            )?)
        };
        let floor_shape = self.split_child_1_sha256.is_none()
            && self.split_child_2_sha256.is_none()
            && self.predecessor_receipt_sha256 == self.direct_trial_sha256
            && self.context.step_support.duration_ns() == STAGE3_ADAPTIVE_MINIMUM_STEP_NS;
        let composed_shape = self.split_child_1_sha256.is_some()
            && self.split_child_2_sha256.is_some()
            && self.split_child_2_sha256 == Some(self.predecessor_receipt_sha256);
        if self.decision.is_floor() != floor_shape || (!self.decision.is_floor() && !composed_shape)
        {
            return Err(adaptive_receipt_identity_error(
                "adaptive comparison decision shape",
            ));
        }
        for (digest, reason) in [
            (
                self.predecessor_receipt_sha256,
                "adaptive comparison predecessor",
            ),
            (self.direct_trial_sha256, "adaptive comparison direct trial"),
            (
                self.selected_physical_ledger_sha256,
                "adaptive comparison selected ledger",
            ),
            (
                self.selected_ending_complete_owner_set_sha256,
                "adaptive comparison selected owner set",
            ),
            (
                self.selected_phase_result_sha256,
                "adaptive comparison selected phase",
            ),
        ] {
            require_adaptive_digest(digest, reason)?;
        }
        let optional_digest = |value: Option<Digest32>| {
            value.map_or_else(
                || vec![0],
                |digest| {
                    let mut bytes = vec![1];
                    bytes.extend_from_slice(digest.as_bytes());
                    bytes
                },
            )
        };
        let mut fields = self.context.fields();
        fields.extend([
            (
                "tolerance_policy_id",
                self.tolerance_policy_id.as_bytes().to_vec(),
            ),
            (
                "predecessor_receipt_sha256",
                self.predecessor_receipt_sha256.as_bytes().to_vec(),
            ),
            (
                "direct_trial_sha256",
                self.direct_trial_sha256.as_bytes().to_vec(),
            ),
            (
                "split_child_1_sha256",
                optional_digest(self.split_child_1_sha256),
            ),
            (
                "split_child_2_sha256",
                optional_digest(self.split_child_2_sha256),
            ),
            (
                "maximum_scaled_error_bits",
                self.maximum_scaled_error.to_bits().to_be_bytes().to_vec(),
            ),
            ("discrete_mismatch", vec![u8::from(self.discrete_mismatch)]),
            (
                "direct_exact_discrete_sha256",
                self.direct_exact_discrete_sha256.as_bytes().to_vec(),
            ),
            (
                "composed_exact_discrete_sha256",
                optional_digest(self.composed_exact_discrete_sha256),
            ),
            (
                "direct_exact_discrete_surface_set_sha256",
                direct_surfaces_sha256.as_bytes().to_vec(),
            ),
            (
                "composed_exact_discrete_surface_set_sha256",
                optional_digest(composed_surfaces_sha256),
            ),
            ("decision", vec![self.decision as u8]),
            (
                "selected_physical_ledger_sha256",
                self.selected_physical_ledger_sha256.as_bytes().to_vec(),
            ),
            (
                "selected_ending_complete_owner_set_sha256",
                self.selected_ending_complete_owner_set_sha256
                    .as_bytes()
                    .to_vec(),
            ),
            (
                "selected_phase_result_sha256",
                self.selected_phase_result_sha256.as_bytes().to_vec(),
            ),
            (
                "selected_event_posture",
                vec![self.selected_event_posture as u8],
            ),
        ]);
        adaptive_framed_sha256("stage3-adaptive-step-comparison-v1", fields)
    }

    pub fn validate(&self) -> Result<(), DirectSnowStage3V11AttachmentError> {
        if self.receipt_sha256 != self.reconstructed_digest()? {
            return Err(adaptive_receipt_identity_error("adaptive comparison seal"));
        }
        Ok(())
    }

    pub fn validate_composed_against(
        &self,
        request: &Stage3AdaptiveParentRequestReceiptV1,
        direct: &Stage3AdaptiveDirectTrialReceiptV1,
        child_1: &Stage3AdaptiveSplitChildTrialReceiptV1,
        child_2: &Stage3AdaptiveSplitChildTrialReceiptV1,
    ) -> Result<(), DirectSnowStage3V11AttachmentError> {
        self.validate()?;
        direct.validate_against(request)?;
        child_1.validate()?;
        child_2.validate()?;
        let accepted_shape = self.decision == Stage3AdaptiveStepDecisionV1::ComposedAccepted
            && direct.disposition == Stage3AdaptiveTrialDispositionV1::Closed
            && child_1.disposition == Stage3AdaptiveTrialDispositionV1::Closed
            && child_2.disposition == Stage3AdaptiveTrialDispositionV1::Closed
            && !self.discrete_mismatch
            && self.maximum_scaled_error <= 1.0;
        let rejected_shape = self.decision == Stage3AdaptiveStepDecisionV1::RefineRejected
            && (direct.disposition == Stage3AdaptiveTrialDispositionV1::TypedRejected
                || child_1.disposition == Stage3AdaptiveTrialDispositionV1::TypedRejected
                || child_2.disposition == Stage3AdaptiveTrialDispositionV1::TypedRejected
                || self.discrete_mismatch
                || self.maximum_scaled_error > 1.0);
        if self.context != request.context
            || child_1.context != request.context
            || child_2.context != request.context
            || self.direct_trial_sha256 != direct.receipt_sha256
            || self.split_child_1_sha256 != Some(child_1.receipt_sha256)
            || self.split_child_2_sha256 != Some(child_2.receipt_sha256)
            || self.predecessor_receipt_sha256 != child_2.receipt_sha256
            || child_1.child_ordinal != 1
            || child_2.child_ordinal != 2
            || child_1.predecessor_receipt_sha256 != direct.receipt_sha256
            || child_2.predecessor_receipt_sha256 != child_1.receipt_sha256
            || child_1.trial_beginning_complete_owner_set_sha256
                != request.context.beginning_complete_owner_set_sha256
            || child_2.trial_beginning_complete_owner_set_sha256
                != child_1.ending_complete_owner_set_sha256
            || child_1.child_support.start_ns() != request.context.step_support.start_ns()
            || child_1.child_support.end_ns() != child_2.child_support.start_ns()
            || child_2.child_support.end_ns() != request.context.step_support.end_ns()
            || self.selected_ending_complete_owner_set_sha256
                != child_2.ending_complete_owner_set_sha256
            || self.selected_phase_result_sha256 != child_2.phase_result_sha256
            || self.selected_event_posture != child_2.event_posture
            || !(accepted_shape || rejected_shape)
        {
            return Err(adaptive_receipt_identity_error(
                "adaptive composed comparison replay",
            ));
        }
        Ok(())
    }

    pub fn validate_floor_against(
        &self,
        request: &Stage3AdaptiveParentRequestReceiptV1,
        direct: &Stage3AdaptiveDirectTrialReceiptV1,
    ) -> Result<(), DirectSnowStage3V11AttachmentError> {
        self.validate()?;
        direct.validate_against(request)?;
        let accepted_shape = self.decision == Stage3AdaptiveStepDecisionV1::FloorAccepted
            && direct.disposition == Stage3AdaptiveTrialDispositionV1::Closed;
        let rejected_shape = self.decision == Stage3AdaptiveStepDecisionV1::FloorRejected
            && direct.disposition == Stage3AdaptiveTrialDispositionV1::TypedRejected;
        if self.context != request.context
            || self.predecessor_receipt_sha256 != direct.receipt_sha256
            || self.direct_trial_sha256 != direct.receipt_sha256
            || self.split_child_1_sha256.is_some()
            || self.split_child_2_sha256.is_some()
            || self.maximum_scaled_error.to_bits() != 0.0_f64.to_bits()
            || self.discrete_mismatch
            || self.selected_physical_ledger_sha256 != direct.physical_ledger_sha256
            || self.selected_ending_complete_owner_set_sha256
                != direct.ending_complete_owner_set_sha256
            || self.selected_phase_result_sha256 != direct.phase_result_sha256
            || self.selected_event_posture != direct.event_posture
            || !(accepted_shape || rejected_shape)
        {
            return Err(adaptive_receipt_identity_error(
                "adaptive floor comparison replay",
            ));
        }
        Ok(())
    }
}

#[derive(Clone, Debug, Eq, PartialEq, serde::Deserialize, Serialize)]
pub struct Stage3AdaptiveAcceptedMicrostepReceiptV1 {
    pub context: Stage3AdaptiveReceiptContextV1,
    pub comparison_receipt_sha256: Digest32,
    pub decision: Stage3AdaptiveStepDecisionV1,
    pub physical_ledger_sha256: Digest32,
    pub ending_complete_owner_set_sha256: Digest32,
    pub phase_result_sha256: Digest32,
    pub event_posture: Stage3AdaptiveEventPostureV1,
    pub receipt_sha256: Digest32,
}

impl Stage3AdaptiveAcceptedMicrostepReceiptV1 {
    pub fn try_new(
        comparison: &Stage3AdaptiveStepComparisonReceiptV1,
    ) -> Result<Self, DirectSnowStage3V11AttachmentError> {
        comparison.validate()?;
        if !comparison.decision.is_accepted() {
            return Err(adaptive_receipt_identity_error(
                "adaptive rejected comparison cannot be accepted",
            ));
        }
        let mut value = Self {
            context: comparison.context,
            comparison_receipt_sha256: comparison.receipt_sha256,
            decision: comparison.decision,
            physical_ledger_sha256: comparison.selected_physical_ledger_sha256,
            ending_complete_owner_set_sha256: comparison.selected_ending_complete_owner_set_sha256,
            phase_result_sha256: comparison.selected_phase_result_sha256,
            event_posture: comparison.selected_event_posture,
            receipt_sha256: Digest32::zero(),
        };
        value.receipt_sha256 = value.reconstructed_digest()?;
        Ok(value)
    }

    fn reconstructed_digest(&self) -> Result<Digest32, DirectSnowStage3V11AttachmentError> {
        self.context.validate()?;
        if !self.decision.is_accepted() {
            return Err(adaptive_receipt_identity_error(
                "adaptive accepted microstep decision",
            ));
        }
        for (digest, reason) in [
            (
                self.comparison_receipt_sha256,
                "adaptive accepted comparison",
            ),
            (self.physical_ledger_sha256, "adaptive accepted ledger"),
            (
                self.ending_complete_owner_set_sha256,
                "adaptive accepted ending owner set",
            ),
            (self.phase_result_sha256, "adaptive accepted phase"),
        ] {
            require_adaptive_digest(digest, reason)?;
        }
        let mut fields = self.context.fields();
        fields.extend([
            (
                "comparison_receipt_sha256",
                self.comparison_receipt_sha256.as_bytes().to_vec(),
            ),
            ("decision", vec![self.decision as u8]),
            (
                "physical_ledger_sha256",
                self.physical_ledger_sha256.as_bytes().to_vec(),
            ),
            (
                "ending_complete_owner_set_sha256",
                self.ending_complete_owner_set_sha256.as_bytes().to_vec(),
            ),
            (
                "phase_result_sha256",
                self.phase_result_sha256.as_bytes().to_vec(),
            ),
            ("event_posture", vec![self.event_posture as u8]),
        ]);
        adaptive_framed_sha256("stage3-adaptive-accepted-microstep-v1", fields)
    }

    pub fn validate_against(
        &self,
        comparison: &Stage3AdaptiveStepComparisonReceiptV1,
    ) -> Result<(), DirectSnowStage3V11AttachmentError> {
        comparison.validate()?;
        if self.receipt_sha256 != self.reconstructed_digest()?
            || self.context != comparison.context
            || self.comparison_receipt_sha256 != comparison.receipt_sha256
            || self.decision != comparison.decision
            || self.physical_ledger_sha256 != comparison.selected_physical_ledger_sha256
            || self.ending_complete_owner_set_sha256
                != comparison.selected_ending_complete_owner_set_sha256
            || self.phase_result_sha256 != comparison.selected_phase_result_sha256
            || self.event_posture != comparison.selected_event_posture
        {
            return Err(adaptive_receipt_identity_error(
                "adaptive accepted microstep lineage",
            ));
        }
        Ok(())
    }
}

pub fn stage3_adaptive_parent_request_set_sha256_v1(
    requests: &[Stage3AdaptiveParentRequestReceiptV1],
) -> Result<Digest32, DirectSnowStage3V11AttachmentError> {
    if requests.is_empty()
        || requests.windows(2).any(|pair| {
            pair[0].context.parent_transaction_id != pair[1].context.parent_transaction_id
                || (
                    pair[0].context.step_ordinal,
                    pair[0].context.attempt_ordinal,
                ) >= (
                    pair[1].context.step_ordinal,
                    pair[1].context.attempt_ordinal,
                )
        })
    {
        return Err(adaptive_receipt_identity_error(
            "adaptive parent request set ordering",
        ));
    }
    let mut fields = Vec::with_capacity(requests.len());
    for request in requests {
        request.validate()?;
        fields.push(("request", request.receipt_sha256.as_bytes().to_vec()));
    }
    adaptive_framed_sha256("stage3-adaptive-parent-request-set-v1", fields)
}

pub fn stage3_adaptive_accepted_microstep_set_sha256_v1(
    receipts: &[Stage3AdaptiveAcceptedMicrostepReceiptV1],
) -> Result<Digest32, DirectSnowStage3V11AttachmentError> {
    if receipts.windows(2).any(|pair| {
        pair[0].context.parent_transaction_id != pair[1].context.parent_transaction_id
            || pair[0].context.step_ordinal >= pair[1].context.step_ordinal
            || pair[0].context.step_support.end_ns() != pair[1].context.step_support.start_ns()
            || pair[0].ending_complete_owner_set_sha256
                != pair[1].context.beginning_complete_owner_set_sha256
    }) {
        return Err(adaptive_receipt_identity_error(
            "adaptive accepted microstep set chronology",
        ));
    }
    let mut fields = Vec::with_capacity(receipts.len());
    for receipt in receipts {
        if receipt.receipt_sha256 != receipt.reconstructed_digest()? {
            return Err(adaptive_receipt_identity_error(
                "adaptive accepted microstep set seal",
            ));
        }
        fields.push(("accepted", receipt.receipt_sha256.as_bytes().to_vec()));
    }
    adaptive_framed_sha256("stage3-adaptive-accepted-microstep-set-v1", fields)
}

/// Transient controller telemetry reconstructed from authoritative receipts.
///
/// This type deliberately has no serialization implementation and is never a
/// member of a receipt, restart, publication, or qualification schema.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct Stage3AdaptiveControllerTelemetryV1 {
    pub direct_trial_count: u64,
    pub split_child_trial_count: u64,
    pub accepted_microstep_count: u64,
    pub rejected_candidate_count: u64,
    pub minimum_accepted_step_ns: Option<u128>,
    pub maximum_accepted_step_ns: Option<u128>,
}

#[derive(Clone, Debug, PartialEq, serde::Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct Stage3AdaptiveSupportReceiptV1 {
    pub parent_transaction_id: ParentTransactionId,
    pub parent_support: TimeSupport,
    pub parent_requests: Vec<Stage3AdaptiveParentRequestReceiptV1>,
    pub direct_trials: Vec<Stage3AdaptiveDirectTrialReceiptV1>,
    pub split_child_trials: Vec<Stage3AdaptiveSplitChildTrialReceiptV1>,
    pub comparisons: Vec<Stage3AdaptiveStepComparisonReceiptV1>,
    pub accepted_microsteps: Vec<Stage3AdaptiveAcceptedMicrostepReceiptV1>,
    pub parent_request_set_sha256: Digest32,
    pub accepted_microstep_set_sha256: Digest32,
}

impl Stage3AdaptiveSupportReceiptV1 {
    fn count(
        value: usize,
        reason: &'static str,
    ) -> Result<u64, DirectSnowStage3V11AttachmentError> {
        u64::try_from(value).map_err(|_| adaptive_receipt_identity_error(reason))
    }

    /// Reconstruct controller counts only for an explicitly enabled telemetry
    /// consumer. No value returned here participates in receipt identity.
    pub fn transient_diagnostics(
        &self,
    ) -> Result<Stage3AdaptiveControllerTelemetryV1, DirectSnowStage3V11AttachmentError> {
        self.validate()?;
        let accepted_microstep_count = Self::count(
            self.accepted_microsteps.len(),
            "adaptive accepted telemetry width",
        )?;
        let accepted_decision_count = Self::count(
            self.comparisons
                .iter()
                .filter(|comparison| comparison.decision.is_accepted())
                .count(),
            "adaptive accepted-decision telemetry width",
        )?;
        let rejected_candidate_count = Self::count(
            self.comparisons
                .iter()
                .filter(|comparison| !comparison.decision.is_accepted())
                .count(),
            "adaptive rejected telemetry width",
        )?;
        if accepted_decision_count != accepted_microstep_count {
            return Err(adaptive_receipt_identity_error(
                "adaptive accepted telemetry reconstruction",
            ));
        }
        let minimum_accepted_step_ns = self
            .accepted_microsteps
            .iter()
            .map(|accepted| accepted.context.step_support.duration_ns())
            .min();
        let maximum_accepted_step_ns = self
            .accepted_microsteps
            .iter()
            .map(|accepted| accepted.context.step_support.duration_ns())
            .max();
        Ok(Stage3AdaptiveControllerTelemetryV1 {
            direct_trial_count: Self::count(
                self.direct_trials.len(),
                "adaptive direct telemetry width",
            )?,
            split_child_trial_count: Self::count(
                self.split_child_trials.len(),
                "adaptive split telemetry width",
            )?,
            accepted_microstep_count,
            rejected_candidate_count,
            minimum_accepted_step_ns,
            maximum_accepted_step_ns,
        })
    }

    pub fn validate(&self) -> Result<(), DirectSnowStage3V11AttachmentError> {
        let accepted_decision_count = self
            .comparisons
            .iter()
            .filter(|comparison| comparison.decision.is_accepted())
            .count();
        if self.parent_requests.is_empty()
            || self.direct_trials.len() != self.parent_requests.len()
            || self.comparisons.len() != self.parent_requests.len()
            || accepted_decision_count != self.accepted_microsteps.len()
            || self.parent_request_set_sha256
                != stage3_adaptive_parent_request_set_sha256_v1(&self.parent_requests)?
            || self.accepted_microstep_set_sha256
                != stage3_adaptive_accepted_microstep_set_sha256_v1(&self.accepted_microsteps)?
            || self.parent_requests.iter().any(|request| {
                request.context.parent_transaction_id != self.parent_transaction_id
                    || request.context.parent_support != self.parent_support
            })
        {
            return Err(adaptive_receipt_identity_error(
                "adaptive support receipt reconstruction",
            ));
        }
        for (request, direct) in self.parent_requests.iter().zip(&self.direct_trials) {
            direct.validate_against(request)?;
        }
        for child in &self.split_child_trials {
            child.validate()?;
        }
        for comparison in &self.comparisons {
            comparison.validate()?;
        }
        for accepted in &self.accepted_microsteps {
            let comparison = self
                .comparisons
                .iter()
                .find(|comparison| comparison.receipt_sha256 == accepted.comparison_receipt_sha256)
                .ok_or_else(|| {
                    adaptive_receipt_identity_error("adaptive accepted comparison membership")
                })?;
            accepted.validate_against(comparison)?;
        }
        Ok(())
    }
}

#[cfg(test)]
#[path = "snow_stage3_v11_attachment_receipt_tests.rs"]
mod adaptive_receipt_tests;
