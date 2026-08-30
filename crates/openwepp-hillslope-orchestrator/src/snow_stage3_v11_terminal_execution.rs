struct ActualTerminalSubslabV1 {
    parent: V11ParentTransaction,
    consumer: DirectV10RealConsumerShadow,
    clock: CoupledClockStateV1,
    stage3: BTreeMap<u32, DirectSnowStage3PersistentState>,
    receipts: Vec<Stage3CoupledSubslabReceiptV1>,
    group: Option<Stage3V11TerminalEventGroupV1>,
    parcels: Vec<DirectSnowStage3V11TerminalParcel>,
}

#[cfg(test)]
include!("snow_stage3_v11_terminal_execution_test_controls.rs");

#[cfg(not(test))]
const fn terminal_provisional_publication_deferral_enabled() -> bool {
    true
}

#[cfg(not(test))]
const fn ordinary_covered_physical_reuse_enabled() -> bool {
    true
}

#[cfg(not(test))]
#[inline(always)]
fn audit_terminal_provider_support(_: TimeSupport) {}

include!("snow_stage3_v11_terminal_closure.rs");
include!("snow_stage3_v11_terminal_boundary_receiver.rs");

include!("snow_stage3_v11_terminal_precomputed_package.rs");
fn prepare_exact_terminal_endpoint_v1(
    discovery: &Stage3V11ActualTerminalCandidateV1,
    exact_result: &crate::hydrology::DirectSnowStage3PersistentDayResult,
    exact_forcing_sha256: Digest32,
    candidates_by_joint: &BTreeMap<
        Digest32,
        crate::v9_real_consumer_shadow::CoveredCarrierEphemeralCandidatesV1,
    >,
    carrier_phases_by_joint: &BTreeMap<
        Digest32,
        crate::v9_real_consumer_shadow::CoveredCarrierPhaseResultV1,
    >,
) -> Result<Box<ExactCoveredTerminalEndpointV1>, DirectSnowStage3V11AttachmentError> {
    let ending = bind_exact_terminal_endpoint_candidate_v1(
        discovery.lane_id,
        exact_result,
        candidates_by_joint,
    )?;
    let event = exact_result
        .terminal_event
        .ok_or(DirectSnowStage3V11AttachmentError::Terminal(
            "exact endpoint event-result custody",
        ))?;
    let accepted_terminal_microstep = exact_result.covered_terminal_accepted_microsteps.last();
    let terminal_trial_support = accepted_terminal_microstep
        .map_or(discovery.support, |step| step.support);
    let mut carrier_phase = if let Some(step) = accepted_terminal_microstep {
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
            .cloned()
            .collect::<Vec<_>>();
        if matching.len() != 1 {
            return Err(DirectSnowStage3V11AttachmentError::Terminal(
                "exact endpoint converged carrier value evidence",
            ));
        }
        matching[0].clone()
    };
    let mut trial_chains_by_lane = BTreeMap::<
        u32,
        Vec<crate::v9_real_consumer_shadow::TerminalSnowSoilTrialReceiptV1>,
    >::new();
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
                    .or_default()
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
        let chain_covers_envelope = chain.first().is_some_and(|receipt| {
            receipt.support.start_ns() == discovery.support.start_ns()
        }) && chain.last().is_some_and(|receipt| {
            receipt.support.end_ns() == discovery.support.end_ns() && receipt == final_receipt
        }) && chain
            .windows(2)
            .all(|pair| pair[0].support.end_ns() == pair[1].support.start_ns());
        if !chain_covers_envelope {
            return Err(DirectSnowStage3V11AttachmentError::Terminal(
                "exact endpoint accepted carrier trial chain coverage",
            ));
        }
    }
    let wb14_replay_trial_sha256 = carrier_phase.transition.probe_child_identity.receipt_sha256;
    let mut carrier_phase_chain = if exact_result.covered_terminal_accepted_microsteps.is_empty() {
        vec![carrier_phase.clone()]
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
        DirectSnowStage3V11AttachmentError::Identity(
            "exact endpoint physical-child count width",
        )
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
        phase.transition.probe_child_identity = CoveredProbeChildIdentityV1::try_new(
            ProbeChildAuthorityV1 {
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
            },
        )?;
    }
    if u32::try_from(carrier_phase_chain.len()).ok() != Some(accepted_child_count) {
        return Err(DirectSnowStage3V11AttachmentError::Identity(
            "exact endpoint physical-child chain count",
        ));
    }
    carrier_phase = carrier_phase_chain
        .last()
        .cloned()
        .ok_or(DirectSnowStage3V11AttachmentError::Terminal(
            "exact endpoint empty carrier phase chain",
        ))?;
    let event_result_digest = canonical_terminal_event_result_digest(&event)?;
    if canonical_terminal_event_result_digest(&discovery.event)? != discovery.event_result_digest
        || discovery.terminal_state_sha256 == Digest32::zero()
        || discovery.shortened_owner_set_sha256 == Digest32::zero()
    {
        return Err(DirectSnowStage3V11AttachmentError::Terminal(
            "terminal discovery ledger identity",
        ));
    }
    if exact_forcing_sha256 != discovery.shortened_forcing_sha256 {
        return Err(DirectSnowStage3V11AttachmentError::Terminal(
            "terminal discovery/exact forcing join",
        ));
    }
    if event.hour_offset_seconds.to_bits()
        != f64::from_bits(discovery.support.duration_s_bits()).to_bits()
        || (event.terminal_entry_offset_seconds + event.evaluated_seconds).to_bits()
            != f64::from_bits(discovery.support.duration_s_bits()).to_bits()
        || event.unevaluated_seconds.to_bits() != 0.0_f64.to_bits()
    {
        return Err(DirectSnowStage3V11AttachmentError::Terminal(
            "exact terminal localization chronology",
        ));
    }
    if !event.terminal_unallocated_energy_j_m2.is_finite()
        || event.terminal_unallocated_energy_j_m2 < 0.0
    {
        return Err(DirectSnowStage3V11AttachmentError::Terminal(
            "exact terminal localization sensible energy",
        ));
    }
    let trial_receipt = ending.terminal_snow_soil_trial_receipt().cloned().ok_or(
        DirectSnowStage3V11AttachmentError::Terminal(
            "exact endpoint terminal snow-soil trial receipt",
        ),
    )?;
    trial_receipt.validate().map_err(|_| {
        DirectSnowStage3V11AttachmentError::Terminal(
            "exact endpoint terminal snow-soil trial receipt seal",
        )
    })?;
    if trial_receipt.support != terminal_trial_support
        || trial_receipt.lane_id != discovery.lane_id
    {
        return Err(DirectSnowStage3V11AttachmentError::Terminal(
            "exact endpoint terminal snow-soil support join",
        ));
    }
    let endpoint_receipt_sha256 = framed_sha256(
        "stage3-v11-exact-terminal-endpoint-v1",
        &[
            FramedField {
                tag: "support_start",
                value: &discovery.support.start_ns().get().to_be_bytes(),
            },
            FramedField {
                tag: "support_end",
                value: &discovery.support.end_ns().get().to_be_bytes(),
            },
            FramedField {
                tag: "lane",
                value: &discovery.lane_id.to_be_bytes(),
            },
            FramedField {
                tag: "exact_event",
                value: event_result_digest.as_bytes(),
            },
            FramedField {
                tag: "discovery_event",
                value: discovery.event_result_digest.as_bytes(),
            },
            FramedField {
                tag: "discovery_state",
                value: discovery.terminal_state_sha256.as_bytes(),
            },
            FramedField {
                tag: "discovery_owner_set",
                value: discovery.shortened_owner_set_sha256.as_bytes(),
            },
            FramedField {
                tag: "forcing",
                value: exact_forcing_sha256.as_bytes(),
            },
            FramedField {
                tag: "ending_joint",
                value: ending.joint().receipt_sha256().as_bytes(),
            },
            FramedField {
                tag: "snow_soil_trial",
                value: trial_receipt.receipt_sha256.as_bytes(),
            },
            FramedField {
                tag: "final_physical_child",
                value: carrier_phase
                    .transition
                    .probe_child_identity
                    .receipt_sha256
                    .as_bytes(),
            },
        ],
    )?;
    let wb14_replay_beginning_owner_set_sha256 = Digest32::from_bytes(
        crate::direct_runtime::wb14_child_replay_binding(&carrier_phase.wb14_child_replay_bytes)
            .map_err(|_| {
                DirectSnowStage3V11AttachmentError::Terminal("exact endpoint WB14 replay binding")
            })?
            .parent_beginning_complete_owner_set_sha256,
    );
    Ok(Box::new(ExactCoveredTerminalEndpointV1 {
        support: discovery.support,
        lane_id: discovery.lane_id,
        event,
        event_result_digest,
        forcing_sha256: exact_forcing_sha256,
        ending,
        carrier_phase: Box::new(carrier_phase),
        carrier_phase_chain,
        wb14_replay_trial_sha256,
        wb14_replay_beginning_owner_set_sha256,
        terminal_snow_soil_trial_receipt: trial_receipt,
        final_child_actual_vapor_to_canopy_air_kg_m2: accepted_terminal_microstep.map_or(
            event.sublimation_kg_m2 - event.deposition_kg_m2,
            |step| step.sublimation_kg_m2 - step.deposition_kg_m2,
        ),
        terminal_snow_soil_trial_receipt_chains_by_lane: trial_chains_by_lane,
        endpoint_receipt_sha256,
    }))
}

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
    carrier.try_with_selected_stage3_by_lane(
        selected_joint.clone(),
        stage3,
    )
    .map_err(DirectSnowStage3V11AttachmentError::Owner)
}

#[cfg(test)]
#[allow(clippy::too_many_arguments, clippy::too_many_lines)]
fn evaluate_covered_terminal_candidate_v1(
    beginning_consumer: &DirectV10RealConsumerShadow,
    beginning_clock: &CoupledClockStateV1,
    prepared: &DirectSnowStage3V11PreparedSupport,
    day_index: usize,
    interval_index: usize,
    beginning_stage3: &BTreeMap<u32, DirectSnowStage3PersistentState>,
    beginning_terminal_parcels: &BTreeMap<Digest32, DirectSnowStage3V11TerminalParcel>,
    selected_upper_bound_s: f64,
    current_child_ordinal: u32,
    lane_id: u32,
    mode: CoveredTerminalExecutionMode,
) -> Result<
    (
        crate::hydrology::DirectSnowStage3PersistentDayResult,
        BTreeMap<Digest32, crate::v9_real_consumer_shadow::CoveredCarrierEphemeralCandidatesV1>,
        BTreeMap<Digest32, crate::v9_real_consumer_shadow::CoveredCarrierPhaseResultV1>,
    ),
    DirectSnowStage3V11AttachmentError,
> {
    let mut evidence = <crate::hydrology::NoEvidence as crate::hydrology::TerminalEvidenceMode<
        Option<CoveredTerminalJointTrialStateV1>,
    >>::new_state();
    evaluate_covered_terminal_candidate_with_evidence_v1::<crate::hydrology::NoEvidence>(
        beginning_consumer,
        beginning_clock,
        prepared,
        day_index,
        interval_index,
        beginning_stage3,
        beginning_terminal_parcels,
        selected_upper_bound_s,
        current_child_ordinal,
        lane_id,
        mode,
        &mut evidence,
    )
}

#[allow(clippy::too_many_arguments, clippy::too_many_lines)]
fn evaluate_covered_terminal_candidate_with_evidence_v1<
    M: crate::hydrology::TerminalEvidenceMode<Option<CoveredTerminalJointTrialStateV1>>,
>(
    beginning_consumer: &DirectV10RealConsumerShadow,
    beginning_clock: &CoupledClockStateV1,
    prepared: &DirectSnowStage3V11PreparedSupport,
    day_index: usize,
    interval_index: usize,
    beginning_stage3: &BTreeMap<u32, DirectSnowStage3PersistentState>,
    beginning_terminal_parcels: &BTreeMap<Digest32, DirectSnowStage3V11TerminalParcel>,
    selected_upper_bound_s: f64,
    current_child_ordinal: u32,
    lane_id: u32,
    mode: CoveredTerminalExecutionMode,
    evidence: &mut M::State,
) -> Result<
    (
        crate::hydrology::DirectSnowStage3PersistentDayResult,
        BTreeMap<Digest32, crate::v9_real_consumer_shadow::CoveredCarrierEphemeralCandidatesV1>,
        BTreeMap<Digest32, crate::v9_real_consumer_shadow::CoveredCarrierPhaseResultV1>,
    ),
    DirectSnowStage3V11AttachmentError,
> {
    let state =
        beginning_stage3
            .get(&lane_id)
            .ok_or(DirectSnowStage3V11AttachmentError::Identity(
                "terminal beginning lane",
            ))?;
    let inputs = prepared.snow_inputs_by_lane.get(&lane_id).ok_or(
        DirectSnowStage3V11AttachmentError::Identity("terminal input lane"),
    )?;
    let forcing = prepared
        .support_forcing_by_lane
        .get(&lane_id)
        .copied()
        .ok_or(DirectSnowStage3V11AttachmentError::Identity(
            "terminal forcing lane",
        ))?;
    let mut initial_owner_bytes = beginning_consumer.canonical_owner_state_bytes()?;
    let beginning_snow_bytes = if beginning_terminal_parcels.is_empty() {
        canonical_stage3_snow_owner_bytes_with_pending(
            beginning_stage3,
            beginning_terminal_parcels,
        )?
    } else {
        beginning_clock
            .owners()
            .iter()
            .find(|owner| owner.owner_id() == "snow")
            .ok_or(DirectSnowStage3V11AttachmentError::Identity(
                "terminal beginning V4 snow owner",
            ))?
            .state_bytes()
            .to_vec()
    };
    initial_owner_bytes.insert("snow".to_owned(), beginning_snow_bytes);
    let parent_owner_digest = complete_owner_set_digest(beginning_clock.owners())?;
    let source_snow_owner_sha256 = digest_bytes(initial_owner_bytes.get("snow").ok_or(
        DirectSnowStage3V11AttachmentError::Identity("terminal source snow owner"),
    )?);
    let initial_joint = CoveredTerminalJointTrialStateV1::try_new(
        JointTrialAuthorityV1 {
            source_owner_set_sha256: parent_owner_digest,
            lane_id,
            source_snow_owner_sha256,
            interval_index: u64::try_from(interval_index).map_err(|_| {
                DirectSnowStage3V11AttachmentError::Identity("terminal interval width")
            })?,
            state_support: prepared.support,
            accepted_predecessors: Vec::new(),
        },
        initial_owner_bytes,
    )?;
    let initial_candidates =
        crate::v9_real_consumer_shadow::CoveredCarrierEphemeralCandidatesV1::try_new(
            initial_joint.clone(),
            beginning_consumer.clone(),
            beginning_stage3.clone(),
        )?;
    let candidates_by_joint = std::cell::RefCell::new(BTreeMap::from([(
        initial_joint.receipt_sha256(),
        initial_candidates,
    )]));
    let parent_id = beginning_clock.parent_transaction_id();
    let carrier_failure = std::cell::RefCell::new(None);
    let carrier_phases_by_joint = std::cell::RefCell::new(BTreeMap::new());
    let mut provider_evidence = M::new_provider_state();
    let mut provider = |request: crate::hydrology::CoveredTerminalTrialRequestV1| {
        audit_terminal_provider_support(request.support);
        let carrier = if let Some(exact) = candidates_by_joint
            .borrow()
            .get(&request.beginning_joint.receipt_sha256())
            .cloned()
        {
            exact
        } else {
            let matching = candidates_by_joint
                .borrow()
                .values()
                .filter(|candidate| {
                    request
                        .beginning_joint
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
                return Err(DirectSnowStage3EvaluationError::TerminalCustody(
                    "covered probe typed beginning joint",
                ));
            }
            matching[0].clone()
        };
        let mut typed_stage3 = carrier.stage3_by_lane().clone();
        typed_stage3.insert(request.lane_id, (*request.beginning_stage3_state).clone());
        let beginning =
            crate::v9_real_consumer_shadow::CoveredCarrierEphemeralCandidatesV1::try_new(
                request.beginning_joint.clone(),
                carrier.shadow().clone(),
                typed_stage3,
            )
            .map_err(|_| {
                DirectSnowStage3EvaluationError::TerminalCustody(
                    "covered probe post-hydrology typed joint",
                )
            })?;
        let mut projected = prepared
            .coupled_subslab(request.support, current_child_ordinal)
            .map_err(|_| {
                DirectSnowStage3EvaluationError::TerminalCustody(
                    "covered probe exact support projection",
                )
            })?;
        if let Some(interval) = projected.covered_v11_interval.as_mut() {
            interval.lse_forcing.transaction_id =
                beginning.shadow().next_lse_transaction_id().map_err(|_| {
                    DirectSnowStage3EvaluationError::TerminalCustody(
                        "covered probe typed transaction projection",
                    )
                })?;
            interval.lse_forcing.forcing_sha256 =
                interval.lse_forcing.canonical_sha256().map_err(|_| {
                    DirectSnowStage3EvaluationError::TerminalCustody(
                        "covered probe typed forcing reseal",
                    )
                })?;
        }
        let beginning_owner_states = request
            .beginning_joint
            .owner_bytes()
            .iter()
            .map(|(id, bytes)| OwnerState::new(id.clone(), bytes.clone()))
            .collect::<Result<Vec<_>, _>>()
            .map_err(|_| {
                DirectSnowStage3EvaluationError::TerminalCustody(
                    "covered probe beginning owner reconstruction",
                )
            })?;
        let (_, _, complete_forcing_sha256, topology_sha256) = projected.forcing_projections();
        let child = CoveredProbeChildIdentityV1::try_new(ProbeChildAuthorityV1 {
            parent_transaction_sha256: beginning_clock.parent_transaction_id().digest(),
            enclosing_parent_support: beginning_clock.parent_support(),
            trial_support: request.support,
            physical_child_ordinal: current_child_ordinal,
            attempt_ordinal: request.attempt_ordinal,
            role: request.role,
            beginning_joint_sha256: request.beginning_joint.receipt_sha256(),
            beginning_owner_set_sha256: complete_owner_set_digest(&beginning_owner_states)
                .map_err(|_| {
                    DirectSnowStage3EvaluationError::TerminalCustody(
                        "covered probe beginning owner set",
                    )
                })?,
            complete_forcing_sha256,
            topology_sha256,
        })?;
        let covered_interval = projected.covered_v11_interval.as_ref().ok_or(
            DirectSnowStage3EvaluationError::TerminalCustody(
                "covered probe V11 interval projection",
            ),
        )?;
        let stack = DirectV11SnowCoveredRealConsumerStack::new(
            beginning.shadow(),
            DirectV11SnowCoveredStackInputs {
                interval: covered_interval,
                stage3_inputs_by_lane: &projected.snow_inputs_by_lane,
                stage3_forcing_by_lane: &projected.support_forcing_by_lane,
                snow_surface_forcing_by_destination: &projected.snow_surface_forcing_by_destination,
                stage3_beginning_by_lane: beginning.stage3_by_lane().clone(),
                pending_terminal_parcels: beginning_terminal_parcels.clone(),
                day_index,
                interval_index,
                finalize_wb14_parent_interval: request.support.end_ns()
                    == beginning_clock.parent_support().end_ns(),
                wb14_coupled_child_binding:
                    crate::direct_runtime::DirectWb14CoupledChildBindingV1 {
                        proposed_upper_bound_s_bits: selected_upper_bound_s.to_bits(),
                        coupled_parent_transaction_sha256: *parent_id.digest().as_bytes(),
                        accepted_slab_sha256: *child.receipt_sha256.as_bytes(),
                        parent_beginning_complete_owner_set_sha256: *parent_owner_digest.as_bytes(),
                        parent_support_start_ns: beginning_clock.parent_support().start_ns().get(),
                        parent_support_end_ns: beginning_clock.parent_support().end_ns().get(),
                        child_support_start_ns: request.support.start_ns().get(),
                        child_support_end_ns: request.support.end_ns().get(),
                    },
            },
        );
        let provider_result = stack.execute_covered_carrier_phase_v1(&beginning, &request, child);
        match provider_result.as_ref() {
            Ok(value) => {
                let projection = M::project_provider_success(&request, value);
                M::provider_success(&mut provider_evidence, &request, projection);
            }
            Err(error) => {
                let projection = M::project_provider_failure(error);
                M::provider_failure(&mut provider_evidence, &request, projection);
            }
        }
        let result = provider_result.map_err(|error| {
            if carrier_failure.borrow().is_none() {
                *carrier_failure.borrow_mut() = Some(error);
            }
            DirectSnowStage3EvaluationError::TerminalCustody("covered probe carrier fixed point")
        })?;
        let ending_joint_sha256 = result.ending_candidates.joint().receipt_sha256();
        candidates_by_joint
            .borrow_mut()
            .insert(ending_joint_sha256, result.ending_candidates.clone());
        carrier_phases_by_joint
            .borrow_mut()
            .insert(ending_joint_sha256, result.clone());
        Ok(result.transition)
    };
    let result =
        Wb11HydrologyKernel::evaluate_stage3_terminal_support_with_trial_provider_and_evidence_v1::<
            M,
        >(
            inputs,
            state,
            lane_id,
            state.next_interval_index,
            forcing,
            prepared.support,
            mode,
            initial_joint,
            &mut provider,
            evidence,
        );
    drop(provider);
    M::merge_provider(evidence, provider_evidence);
    match result {
        Ok(mut result) => {
            let parent_end = beginning_clock.parent_support().end_ns();
            for step in &mut result.covered_terminal_accepted_microsteps {
                Wb11HydrologyKernel::project_stage3_parent_cadence_state(
                    state,
                    &mut step.ending_state,
                    step.support.end_ns() == parent_end,
                )?;
            }
            Wb11HydrologyKernel::project_stage3_parent_cadence_result(
                state,
                &mut result,
                prepared.support.end_ns() == parent_end,
            )?;
            Ok((
                result,
                candidates_by_joint.into_inner(),
                carrier_phases_by_joint.into_inner(),
            ))
        }
        Err(error) => {
            if let Some(carrier_error) = carrier_failure.into_inner() {
                Err(DirectSnowStage3V11AttachmentError::Owner(carrier_error))
            } else {
                Err(DirectSnowStage3V11AttachmentError::Stage3(error))
            }
        }
    }
}

struct CoveredTerminalBatchCandidateV2 {
    phase: crate::v9_real_consumer_shadow::CoveredCarrierPhaseResultV1,
    result: crate::hydrology::CoveredTerminalBatchTrialResultV2,
    lane_results: BTreeMap<u32, crate::hydrology::DirectSnowStage3PersistentDayResult>,
    ending: crate::v9_real_consumer_shadow::CoveredCarrierEphemeralCandidatesV1,
}

#[allow(clippy::too_many_arguments, clippy::too_many_lines)]
fn evaluate_covered_terminal_batch_candidate_v2(
    beginning_consumer: &DirectV10RealConsumerShadow,
    beginning_clock: &CoupledClockStateV1,
    prepared: &DirectSnowStage3V11PreparedSupport,
    day_index: usize,
    interval_index: usize,
    beginning_stage3: &BTreeMap<u32, DirectSnowStage3PersistentState>,
    beginning_terminal_parcels: &BTreeMap<Digest32, DirectSnowStage3V11TerminalParcel>,
    selected_upper_bound_s: f64,
    current_child_ordinal: u32,
) -> Result<CoveredTerminalBatchCandidateV2, DirectSnowStage3V11AttachmentError> {
    let mut initial_owner_bytes = beginning_consumer.canonical_owner_state_bytes()?;
    let beginning_snow_bytes = if beginning_terminal_parcels.is_empty() {
        canonical_stage3_snow_owner_bytes_with_pending(
            beginning_stage3,
            beginning_terminal_parcels,
        )?
    } else {
        beginning_clock
            .owners()
            .iter()
            .find(|owner| owner.owner_id() == "snow")
            .ok_or(DirectSnowStage3V11AttachmentError::Identity(
                "terminal batch beginning V4 snow owner",
            ))?
            .state_bytes()
            .to_vec()
    };
    initial_owner_bytes.insert("snow".to_owned(), beginning_snow_bytes);
    let (&leader_id, _) =
        beginning_stage3
            .first_key_value()
            .ok_or(DirectSnowStage3V11AttachmentError::Identity(
                "terminal batch empty lane topology",
            ))?;
    let parent_owner_digest = complete_owner_set_digest(beginning_clock.owners())?;
    let source_snow_owner_sha256 = digest_bytes(initial_owner_bytes.get("snow").ok_or(
        DirectSnowStage3V11AttachmentError::Identity("terminal batch source snow owner"),
    )?);
    let initial_joint = CoveredTerminalJointTrialStateV1::try_new(
        JointTrialAuthorityV1 {
            source_owner_set_sha256: parent_owner_digest,
            lane_id: leader_id,
            source_snow_owner_sha256,
            interval_index: u64::try_from(interval_index).map_err(|_| {
                DirectSnowStage3V11AttachmentError::Identity("terminal batch interval width")
            })?,
            state_support: prepared.support,
            accepted_predecessors: Vec::new(),
        },
        initial_owner_bytes,
    )?;
    let initial_candidates =
        crate::v9_real_consumer_shadow::CoveredCarrierEphemeralCandidatesV1::try_new(
            initial_joint.clone(),
            beginning_consumer.clone(),
            beginning_stage3.clone(),
        )?;
    let mut lanes = BTreeMap::new();
    for (lane_id, state) in beginning_stage3 {
        if !stage3_is_resolved_thermal_domain(state)
            && !crate::hydrology::stage3_is_terminal_event_domain(state)
        {
            continue;
        }
        let surface = if crate::hydrology::stage3_is_terminal_event_domain(state) {
            Wb11HydrologyKernel::project_stage3_terminal_surface_state_v1(state)?
        } else {
            Wb11HydrologyKernel::project_stage3_surface_state_v1(state)?
        };
        let ice_kg_m2 = state
            .layers
            .iter()
            .map(|layer| layer.mass_swe_m * 1_000.0)
            .sum::<f64>();
        let snow_depth_m = state
            .layers
            .iter()
            .map(|layer| layer.thickness_m)
            .sum::<f64>();
        let liquid_kg_m2 = state
            .layers
            .iter()
            .map(|layer| layer.liquid_water_m * 1_000.0)
            .sum::<f64>()
            + state.detached_retained_liquid_kg_m2;
        let cold_content_j_m2 = state
            .layers
            .iter()
            .map(|layer| layer.cold_content_j_m2)
            .sum::<f64>();
        let snow_density_kg_m3 = if snow_depth_m > 0.0 {
            ice_kg_m2 / snow_depth_m
        } else {
            100.0
        };
        lanes.insert(
            *lane_id,
            crate::hydrology::CoveredTerminalLaneTrialStateV2 {
                lane_id: *lane_id,
                ice_kg_m2,
                liquid_kg_m2,
                cold_content_j_m2,
                surface_temperature_c: surface.surface_temperature_k - 273.15,
                snow_depth_m,
                snow_density_kg_m3,
                resolved_beginning: stage3_is_resolved_thermal_domain(state),
                candidate_event_tick: None,
            },
        );
    }
    let mut request = crate::hydrology::CoveredTerminalBatchTrialRequestV2 {
        support: prepared.support,
        role: crate::hydrology::CoveredTerminalTrialRoleV1::Root,
        attempt_ordinal: 0,
        lanes,
        beginning_joint: initial_joint.clone(),
    };
    let mut projected = prepared.clone();
    if let Some(interval) = projected.covered_v11_interval.as_mut() {
        interval.lse_forcing.transaction_id =
            beginning_consumer.next_lse_transaction_id().map_err(|_| {
                DirectSnowStage3V11AttachmentError::Identity(
                    "terminal batch typed transaction projection",
                )
            })?;
        interval.lse_forcing.forcing_sha256 =
            interval.lse_forcing.canonical_sha256().map_err(|_| {
                DirectSnowStage3V11AttachmentError::Identity("terminal batch typed forcing reseal")
            })?;
    }
    let beginning_owner_states = initial_joint
        .owner_bytes()
        .iter()
        .map(|(id, bytes)| OwnerState::new(id.clone(), bytes.clone()))
        .collect::<Result<Vec<_>, _>>()?;
    let (_, _, complete_forcing_sha256, topology_sha256) = projected.forcing_projections();
    let child = CoveredProbeChildIdentityV1::try_new(ProbeChildAuthorityV1 {
        parent_transaction_sha256: beginning_clock.parent_transaction_id().digest(),
        enclosing_parent_support: beginning_clock.parent_support(),
        trial_support: prepared.support,
        physical_child_ordinal: current_child_ordinal,
        attempt_ordinal: 0,
        role: crate::hydrology::CoveredTerminalTrialRoleV1::Root,
        beginning_joint_sha256: initial_joint.receipt_sha256(),
        beginning_owner_set_sha256: complete_owner_set_digest(&beginning_owner_states)?,
        complete_forcing_sha256,
        topology_sha256,
    })?;
    let covered_interval = projected.covered_v11_interval.as_ref().ok_or(
        DirectSnowStage3V11AttachmentError::Terminal("terminal batch covered interval"),
    )?;
    let stack = DirectV11SnowCoveredRealConsumerStack::new(
        beginning_consumer,
        DirectV11SnowCoveredStackInputs {
            interval: covered_interval,
            stage3_inputs_by_lane: &projected.snow_inputs_by_lane,
            stage3_forcing_by_lane: &projected.support_forcing_by_lane,
            snow_surface_forcing_by_destination: &projected.snow_surface_forcing_by_destination,
            stage3_beginning_by_lane: beginning_stage3.clone(),
            pending_terminal_parcels: beginning_terminal_parcels.clone(),
            day_index,
            interval_index,
            finalize_wb14_parent_interval: prepared.support.end_ns()
                == beginning_clock.parent_support().end_ns(),
            wb14_coupled_child_binding: crate::direct_runtime::DirectWb14CoupledChildBindingV1 {
                proposed_upper_bound_s_bits: selected_upper_bound_s.to_bits(),
                coupled_parent_transaction_sha256: *beginning_clock
                    .parent_transaction_id()
                    .digest()
                    .as_bytes(),
                accepted_slab_sha256: *child.receipt_sha256.as_bytes(),
                parent_beginning_complete_owner_set_sha256: *parent_owner_digest.as_bytes(),
                parent_support_start_ns: beginning_clock.parent_support().start_ns().get(),
                parent_support_end_ns: beginning_clock.parent_support().end_ns().get(),
                child_support_start_ns: prepared.support.start_ns().get(),
                child_support_end_ns: prepared.support.end_ns().get(),
            },
        },
    );
    let phase =
        stack.execute_covered_carrier_batch_phase_v2(&initial_candidates, &request, child)?;
    let mut lane_results = BTreeMap::new();
    let mut hydrology_endings = BTreeMap::new();
    let lane_ids = request.lanes.keys().copied().collect::<Vec<_>>();
    for lane_id in lane_ids {
        let inputs = projected.snow_inputs_by_lane.get(&lane_id).ok_or(
            DirectSnowStage3V11AttachmentError::Identity("terminal batch lane inputs"),
        )?;
        let state =
            beginning_stage3
                .get(&lane_id)
                .ok_or(DirectSnowStage3V11AttachmentError::Identity(
                    "terminal batch lane state",
                ))?;
        let forcing = projected
            .support_forcing_by_lane
            .get(&lane_id)
            .copied()
            .ok_or(DirectSnowStage3V11AttachmentError::Identity(
                "terminal batch lane forcing",
            ))?;
        let boundary = phase
            .batch_boundaries_by_lane
            .get(&lane_id)
            .copied()
            .ok_or(DirectSnowStage3V11AttachmentError::Terminal(
                "terminal batch lane boundary",
            ))?;
        let mut result = if crate::hydrology::stage3_is_terminal_event_domain(state) {
            Wb11HydrologyKernel::evaluate_stage3_terminal_batch_support_with_boundary_v2(
                inputs,
                state,
                lane_id,
                state.next_interval_index,
                forcing,
                boundary,
            )?
        } else {
            Wb11HydrologyKernel::evaluate_stage3_persistent_support_with_boundary(
                inputs,
                state,
                lane_id,
                state.next_interval_index,
                forcing,
                boundary,
            )?
        };
        Wb11HydrologyKernel::project_stage3_parent_cadence_result(
            state,
            &mut result,
            prepared.support.end_ns() == beginning_clock.parent_support().end_ns(),
        )?;
        if let Some(event) = result.terminal_event.filter(|event| event.event_occurred) {
            let relative = quantize_seconds_to_tick(
                ModelTimeNs::new(0),
                ModelTimeNs::new(prepared.support.duration_ns()),
                event.hour_offset_seconds,
            )?;
            request
                .lanes
                .get_mut(&lane_id)
                .ok_or(DirectSnowStage3V11AttachmentError::Identity(
                    "terminal batch event lane",
                ))?
                .candidate_event_tick = Some(ModelTimeNs::new(
                prepared.support.start_ns().get() + relative.get(),
            ));
        }
        hydrology_endings.insert(lane_id, result.state.clone());
        lane_results.insert(lane_id, result);
    }
    let carrier_candidates = phase.batch_carrier_candidates_v2();
    let mut provider =
        |_: &crate::hydrology::CoveredTerminalBatchTrialRequestV2| Ok(carrier_candidates.clone());
    let mut joined = carrier_candidates.carrier_joint.clone();
    for (lane_id, result) in &lane_results {
        let state = &result.state;
        let ice = state
            .layers
            .iter()
            .map(|layer| layer.mass_swe_m * 1_000.0)
            .sum();
        let liquid = state
            .layers
            .iter()
            .map(|layer| layer.liquid_water_m * 1_000.0)
            .sum::<f64>()
            + state.detached_retained_liquid_kg_m2;
        let cold = state
            .layers
            .iter()
            .map(|layer| layer.cold_content_j_m2)
            .sum();
        joined = joined.with_terminal_hydrology_state(*lane_id, ice, liquid, cold)?;
    }
    let mut join =
        |_: &crate::hydrology::CoveredTerminalBatchTrialRequestV2,
         _: &crate::hydrology::CoveredTerminalBatchCarrierCandidatesV2,
         _: &BTreeMap<u32, DirectSnowStage3PersistentState>| { Ok(joined.clone()) };
    let batch_result = Wb11HydrologyKernel::execute_covered_terminal_batch_trial_v2(
        &request,
        hydrology_endings.clone(),
        &mut provider,
        &mut join,
    )?;
    #[cfg(test)]
    TERMINAL_BATCH_PRODUCTION_AUDIT.with(|audit| {
        if let Some(entries) = audit.borrow_mut().as_mut() {
            entries.push(TerminalBatchProductionAuditV2 {
                support: request.support,
                lane_ids: request.lanes.keys().copied().collect(),
                event_ticks: request
                    .lanes
                    .iter()
                    .map(|(lane_id, lane)| (*lane_id, lane.candidate_event_tick))
                    .collect(),
                ending_terminal_lanes: hydrology_endings
                    .iter()
                    .filter_map(|(lane_id, state)| {
                        crate::hydrology::stage3_is_terminal_event_domain(state).then_some(*lane_id)
                    })
                    .collect(),
                ending_surviving_lanes: hydrology_endings
                    .iter()
                    .filter_map(|(lane_id, state)| {
                        crate::hydrology::stage3_is_resolved_thermal_domain(state)
                            .then_some(*lane_id)
                    })
                    .collect(),
                provider_call_count: 1,
                join_call_count: 1,
                beginning_joint_sha256: request.beginning_joint.receipt_sha256(),
                ending_joint_sha256: batch_result.ending_joint.receipt_sha256(),
            });
        }
    });
    let ending = crate::v9_real_consumer_shadow::CoveredCarrierEphemeralCandidatesV1::try_new(
        batch_result.ending_joint.clone(),
        phase.ending_candidates.shadow().clone(),
        hydrology_endings,
    )?;
    Ok(CoveredTerminalBatchCandidateV2 {
        phase,
        result: batch_result,
        lane_results,
        ending,
    })
}

#[allow(clippy::too_many_arguments, clippy::too_many_lines)]
fn try_actual_terminal_batch_subslab_v2(
    context: &DirectSnowStage3V11StaticContext,
    beginning_parent: &V11ParentTransaction,
    beginning_consumer: &DirectV10RealConsumerShadow,
    beginning_clock: &CoupledClockStateV1,
    prepared: &DirectSnowStage3V11PreparedSupport,
    day_index: usize,
    interval_index: usize,
    forcing_receipt: Digest32,
    beginning_stage3: &BTreeMap<u32, DirectSnowStage3PersistentState>,
    beginning_terminal_parcels: &BTreeMap<Digest32, DirectSnowStage3V11TerminalParcel>,
    selected_upper_bound_s: f64,
    current_child_ordinal: u32,
    event_ordinal: u64,
) -> Result<Option<ActualTerminalSubslabV1>, DirectSnowStage3V11AttachmentError> {
    let discovery = evaluate_covered_terminal_batch_candidate_v2(
        beginning_consumer,
        beginning_clock,
        prepared,
        day_index,
        interval_index,
        beginning_stage3,
        beginning_terminal_parcels,
        selected_upper_bound_s,
        current_child_ordinal,
    )?;
    let exact_prepared = if let Some(tick) = discovery.result.decision.event_tick {
        if tick < prepared.support.end_ns() {
            let pre = tick.get() - prepared.support.start_ns().get();
            let post = beginning_clock.parent_support().end_ns().get() - tick.get();
            if (pre != 0 && pre < context.minimum_support_ns)
                || (post != 0 && post < context.minimum_support_ns)
            {
                return Err(DirectSnowStage3V11AttachmentError::Support(
                    "terminal batch event creates positive subminimum support",
                ));
            }
            prepared.coupled_subslab(
                TimeSupport::new(prepared.support.start_ns(), tick)?,
                current_child_ordinal,
            )?
        } else {
            prepared.clone()
        }
    } else {
        prepared.clone()
    };
    let exact = if exact_prepared.support == prepared.support {
        discovery
    } else {
        evaluate_covered_terminal_batch_candidate_v2(
            beginning_consumer,
            beginning_clock,
            &exact_prepared,
            day_index,
            interval_index,
            beginning_stage3,
            beginning_terminal_parcels,
            selected_upper_bound_s,
            current_child_ordinal,
        )?
    };
    let terminal_lanes = exact
        .result
        .decision
        .terminating_lanes
        .iter()
        .copied()
        .collect::<BTreeSet<_>>();
    if terminal_lanes.len() != exact.result.decision.terminating_lanes.len()
        || terminal_lanes.is_empty() != exact.result.decision.event_tick.is_none()
    {
        return Err(DirectSnowStage3V11AttachmentError::Identity(
            "terminal batch exact decision lanes",
        ));
    }
    if terminal_lanes.is_empty() {
        let (parent, consumer, clock, stage3, receipt) = execute_covered_real_v11_subslab(
            context,
            beginning_parent,
            beginning_consumer,
            beginning_clock,
            &exact_prepared,
            day_index,
            interval_index,
            forcing_receipt,
            beginning_stage3.clone(),
            beginning_terminal_parcels,
            selected_upper_bound_s,
            None,
        )?;
        return Ok(Some(ActualTerminalSubslabV1 {
            parent,
            consumer,
            clock,
            stage3,
            receipts: vec![receipt],
            group: None,
            parcels: Vec::new(),
        }));
    }
    let replay_trial_sha256 = exact.phase.transition.probe_child_identity.receipt_sha256;
    let replay_beginning_owner_sha256 = Digest32::from_bytes(
        crate::direct_runtime::wb14_child_replay_binding(&exact.phase.wb14_child_replay_bytes)
            .map_err(|_| {
                DirectSnowStage3V11AttachmentError::Terminal("terminal batch WB14 replay binding")
            })?
            .parent_beginning_complete_owner_set_sha256,
    );
    let forcing_sha256 =
        canonical_stage3_support_forcing_digest(&exact_prepared.support_forcing_by_lane);
    let mut endpoints = Vec::new();
    for lane_id in &terminal_lanes {
        let result =
            exact
                .lane_results
                .get(lane_id)
                .ok_or(DirectSnowStage3V11AttachmentError::Identity(
                    "terminal batch exact lane result",
                ))?;
        let event = result
            .terminal_event
            .ok_or(DirectSnowStage3V11AttachmentError::Terminal(
                "terminal batch exact event-result custody",
            ))?;
        let trial = exact
            .phase
            .batch_terminal_snow_soil_trial_receipts_by_lane
            .get(lane_id)
            .cloned()
            .ok_or(DirectSnowStage3V11AttachmentError::Terminal(
                "terminal batch exact snow-soil receipt",
            ))?;
        let event_result_digest = canonical_terminal_event_result_digest(&event)?;
        let endpoint_receipt_sha256 = framed_sha256(
            "stage3-v11-joint-terminal-endpoint-v2",
            &[
                FramedField {
                    tag: "support_start",
                    value: &exact_prepared.support.start_ns().get().to_be_bytes(),
                },
                FramedField {
                    tag: "support_end",
                    value: &exact_prepared.support.end_ns().get().to_be_bytes(),
                },
                FramedField {
                    tag: "lane",
                    value: &lane_id.to_be_bytes(),
                },
                FramedField {
                    tag: "batch_decision",
                    value: exact.result.decision.receipt_sha256.as_bytes(),
                },
                FramedField {
                    tag: "event",
                    value: event_result_digest.as_bytes(),
                },
            ],
        )?;
        endpoints.push(Box::new(ExactCoveredTerminalEndpointV1 {
            support: exact_prepared.support,
            lane_id: *lane_id,
            event,
            event_result_digest,
            forcing_sha256,
            ending: exact.ending.clone(),
            carrier_phase: Box::new(exact.phase.clone()),
            carrier_phase_chain: vec![exact.phase.clone()],
            wb14_replay_trial_sha256: replay_trial_sha256,
            wb14_replay_beginning_owner_set_sha256: replay_beginning_owner_sha256,
            terminal_snow_soil_trial_receipt: trial,
            final_child_actual_vapor_to_canopy_air_kg_m2: result
                .covered_terminal_accepted_microsteps
                .last()
                .map_or(event.sublimation_kg_m2 - event.deposition_kg_m2, |step| {
                    step.sublimation_kg_m2 - step.deposition_kg_m2
                }),
            terminal_snow_soil_trial_receipt_chains_by_lane: exact
                .phase
                .batch_terminal_snow_soil_trial_receipts_by_lane
                .iter()
                .map(|(lane_id, receipt)| (*lane_id, vec![receipt.clone()]))
                .collect(),
            endpoint_receipt_sha256,
        }));
    }
    let (parent, mut consumer, clock, stage3, receipt) = execute_covered_real_v11_subslab(
        context,
        beginning_parent,
        beginning_consumer,
        beginning_clock,
        &exact_prepared,
        day_index,
        interval_index,
        forcing_receipt,
        beginning_stage3.clone(),
        beginning_terminal_parcels,
        selected_upper_bound_s,
        Some(&endpoints),
    )?;
    let candidates = endpoints
        .iter()
        .filter(|endpoint| endpoint.event.event_occurred)
        .map(|endpoint| {
            let state = stage3.get(&endpoint.lane_id).ok_or(
                DirectSnowStage3V11AttachmentError::Identity("terminal batch installed event lane"),
            )?;
            Ok(Stage3V11ActualTerminalCandidateV1 {
                lane_id: endpoint.lane_id,
                tick: exact_prepared.support.end_ns(),
                support: exact_prepared.support,
                event: endpoint.event,
                event_result_digest: endpoint.event_result_digest,
                terminal_state_sha256: digest_bytes(
                    &Wb11HydrologyKernel::serialize_stage3_persistent_state(state)?,
                ),
                shortened_forcing_sha256: endpoint.forcing_sha256,
                shortened_owner_set_sha256: exact.result.ending_joint.receipt_sha256(),
                exact_endpoint_receipt_sha256: Some(endpoint.endpoint_receipt_sha256),
                terminal_snow_soil_trial_receipt_sha256: Some(
                    endpoint.terminal_snow_soil_trial_receipt.receipt_sha256,
                ),
            })
        })
        .collect::<Result<Vec<_>, DirectSnowStage3V11AttachmentError>>()?;
    if candidates.is_empty() {
        return Ok(Some(ActualTerminalSubslabV1 {
            parent,
            consumer,
            clock,
            stage3,
            receipts: vec![receipt],
            group: None,
            parcels: Vec::new(),
        }));
    }
    let active_lanes = beginning_stage3
        .iter()
        .filter_map(|(lane_id, state)| {
            (stage3_is_resolved_thermal_domain(state)
                || crate::hydrology::stage3_is_terminal_event_domain(state))
            .then_some(*lane_id)
        })
        .collect::<BTreeSet<_>>();
    let terminal_group_ordinal =
        terminal_group_ordinal_after_physical_support_v1(event_ordinal, clock.event_ordinal())?;
    let mut group = select_common_earliest_actual_terminal_group_v1(
        beginning_clock.parent_support(),
        terminal_group_ordinal,
        &active_lanes,
        candidates,
    )?
    .ok_or(DirectSnowStage3V11AttachmentError::Terminal(
        "terminal batch event group",
    ))?;
    let (parent, clock, stage3, parcels, accepted_event_receipt) = apply_actual_terminal_group(
        context,
        parent,
        clock,
        stage3,
        beginning_terminal_parcels,
        current_child_ordinal,
        &mut group,
        &receipt.lane_receipts,
        &receipt.destination_receipts,
    )?;
    consumer.retain_accepted_publication_zero_duration_event(&accepted_event_receipt)?;
    group.accepted_event_receipt = Some(accepted_event_receipt);
    group.accepted_group_receipt_sha256 = Some(accepted_terminal_group_digest(&group)?);
    Ok(Some(ActualTerminalSubslabV1 {
        parent,
        consumer,
        clock,
        stage3,
        receipts: vec![receipt],
        group: Some(group),
        parcels,
    }))
}

#[cfg(test)]
include!("snow_stage3_discrete_endpoint_evidence.rs");

include!("snow_stage3_v11_terminal_preterminal_disposition.rs");
include!("snow_stage3_v11_terminal_discovery_order.rs");

#[allow(clippy::too_many_arguments, clippy::too_many_lines)]
fn try_actual_terminal_subslab_with_evidence<M>(
    context: &DirectSnowStage3V11StaticContext,
    beginning_parent: &V11ParentTransaction,
    beginning_consumer: &DirectV10RealConsumerShadow,
    beginning_clock: &CoupledClockStateV1,
    prepared: &DirectSnowStage3V11PreparedSupport,
    day_index: usize,
    interval_index: usize,
    forcing_receipt: Digest32,
    beginning_stage3: &BTreeMap<u32, DirectSnowStage3PersistentState>,
    beginning_terminal_parcels: &BTreeMap<Digest32, DirectSnowStage3V11TerminalParcel>,
    selected_upper_bound_s: f64,
    current_child_ordinal: u32,
    event_ordinal: u64,
    evidence: &mut M::State,
) -> Result<Option<ActualTerminalSubslabV1>, DirectSnowStage3V11AttachmentError>
where
    M: crate::hydrology::TerminalEvidenceMode<Option<CoveredTerminalJointTrialStateV1>>,
{
    let active_lanes = beginning_stage3
        .iter()
        .filter_map(|(lane, state)| {
            (stage3_is_resolved_thermal_domain(state)
                || crate::hydrology::stage3_is_terminal_event_domain(state))
            .then_some(*lane)
        })
        .collect::<BTreeSet<_>>();
    let mut candidate_ticks = BTreeSet::new();
    let mut discovery_candidates = BTreeMap::new();
    let terminal_lanes = beginning_stage3
        .iter()
        .filter_map(|(lane, state)| {
            crate::hydrology::stage3_is_terminal_event_domain(state).then_some(*lane)
        })
        .collect::<BTreeSet<_>>();
    if active_lanes.len() > 1 && !terminal_lanes.is_empty() {
        return try_actual_terminal_batch_subslab_v2(
            context,
            beginning_parent,
            beginning_consumer,
            beginning_clock,
            prepared,
            day_index,
            interval_index,
            forcing_receipt,
            beginning_stage3,
            beginning_terminal_parcels,
            selected_upper_bound_s,
            current_child_ordinal,
            event_ordinal,
        );
    }
    for lane_id in &terminal_lanes {
        let (result, _candidates_by_joint, carrier_phases_by_joint) =
            evaluate_covered_terminal_candidate_with_evidence_v1::<M>(
                beginning_consumer,
                beginning_clock,
                prepared,
                day_index,
                interval_index,
                beginning_stage3,
                beginning_terminal_parcels,
                selected_upper_bound_s,
                current_child_ordinal,
                *lane_id,
                CoveredTerminalExecutionMode::DiscoveryProbe,
                evidence,
            )?;
        let Some(event) = result.terminal_event else {
            continue;
        };
        let replay_preterminal_microsteps = if result
            .covered_terminal_accepted_microsteps
            .is_empty()
        {
            false
        } else {
            let forcing_snowfall_m = prepared
                .support_forcing_by_lane
                .get(lane_id)
                .ok_or(DirectSnowStage3V11AttachmentError::Identity(
                    "adaptive preterminal solid forcing lane",
                ))?
                .forcing
                .snowfall_m;
            let mut replay = true;
            for step in &result.covered_terminal_accepted_microsteps {
                let phase = carrier_phases_by_joint
                    .get(&step.carrier_ending_joint.receipt_sha256())
                    .ok_or(DirectSnowStage3V11AttachmentError::Terminal(
                        "adaptive preterminal solid parcel carrier phase",
                    ))?;
                let precipitation = phase.precipitation_sets.get(lane_id).ok_or(
                    DirectSnowStage3V11AttachmentError::Terminal(
                        "adaptive preterminal solid parcel lane",
                    ),
                )?;
                crate::snow_stage3_v11_attachment::validate_precipitation_phase_parcel_set(
                    precipitation,
                )?;
                if precipitation.support != step.support || precipitation.lane_id != *lane_id {
                    return Err(DirectSnowStage3V11AttachmentError::Identity(
                        "adaptive preterminal solid parcel support",
                    ));
                }
                let sealed_positive_solid_parcel = precipitation.parcels.iter().any(|parcel| {
                    parcel.phase
                        == crate::snow_stage3_v11_attachment::Stage3PrecipitationPhaseV1::Solid
                        && parcel.source
                            == crate::snow_stage3_v11_attachment::Stage3PrecipitationSourceV1::AtmosphericGroundSnow
                        && parcel.mass_kg_m2_tile_ground > 0.0
                });
                replay &= replay_preterminal_microsteps_before_terminal_localization_v1(
                    event.event_occurred,
                    result.covered_terminal_accepted_microsteps.len(),
                    forcing_snowfall_m,
                    sealed_positive_solid_parcel,
                )?;
            }
            replay
        };
        if replay_preterminal_microsteps {
            let microstep_supports = result
                .covered_terminal_accepted_microsteps
                .iter()
                .map(|step| step.support)
                .collect::<Vec<_>>();
            let first = result.covered_terminal_accepted_microsteps.first();
            let last = result.covered_terminal_accepted_microsteps.last();
            let microstep_states_are_exact = first.is_some_and(|step| {
                step.beginning_ice_kg_m2.to_bits() == event.start_ice_kg_m2.to_bits()
                    && step.beginning_liquid_kg_m2.to_bits() == event.start_liquid_kg_m2.to_bits()
                    && step.beginning_cold_content_j_m2.to_bits()
                        == event.start_cold_content_j_m2.to_bits()
            }) && result
                .covered_terminal_accepted_microsteps
                .windows(2)
                .all(|steps| {
                    steps[0].ending_ice_kg_m2.to_bits() == steps[1].beginning_ice_kg_m2.to_bits()
                        && steps[0].ending_liquid_kg_m2.to_bits()
                            == steps[1].beginning_liquid_kg_m2.to_bits()
                        && steps[0].ending_cold_content_j_m2.to_bits()
                            == steps[1].beginning_cold_content_j_m2.to_bits()
                })
                && last.is_some_and(|step| {
                    step.ending_ice_kg_m2 > 0.0
                        && step.ending_ice_kg_m2.to_bits() == event.end_ice_kg_m2.to_bits()
                        && step.ending_liquid_kg_m2.to_bits()
                            == event.terminal_liquid_kg_m2.to_bits()
                        && step.ending_cold_content_j_m2.to_bits()
                            == event.end_cold_content_j_m2.to_bits()
                        && step.ending_state == result.state
                })
                && result
                    .covered_terminal_accepted_microsteps
                    .iter()
                    .all(|step| step.beginning_ice_kg_m2 > 0.0 && step.ending_ice_kg_m2 > 0.0);
            if !event.event_occurred
                && !accepted_preterminal_non_event_disposition_v1(
                    &AcceptedPreterminalNonEventDispositionV1 {
                        support: prepared.support,
                        event_occurred: event.event_occurred,
                        terminal_entry_offset_seconds: event.terminal_entry_offset_seconds,
                        requested_seconds: event.requested_seconds,
                        evaluated_seconds: event.evaluated_seconds,
                        unevaluated_seconds: event.unevaluated_seconds,
                        hour_offset_seconds: event.hour_offset_seconds,
                        ending_is_supported_snow_domain: stage3_is_resolved_thermal_domain(
                            &result.state,
                        )
                            || crate::hydrology::stage3_is_terminal_event_domain(&result.state),
                        microstep_supports: &microstep_supports,
                        microstep_states_are_exact,
                    },
                )
            {
                return Err(DirectSnowStage3V11AttachmentError::Terminal(
                    "adaptive preterminal endpoint disposition",
                ));
            }
            let mut parent = beginning_parent.clone();
            let mut consumer = beginning_consumer.clone();
            let mut clock = beginning_clock.clone();
            let mut stage3 = beginning_stage3.clone();
            let mut receipts = Vec::new();
            for step in &result.covered_terminal_accepted_microsteps {
                let mut carrier_phase = carrier_phases_by_joint
                    .get(&step.carrier_ending_joint.receipt_sha256())
                    .cloned()
                    .ok_or(DirectSnowStage3V11AttachmentError::Terminal(
                        "adaptive preterminal accepted carrier phase",
                    ))?;
                let physical_child_ordinal = current_child_ordinal
                    .checked_add(u32::try_from(receipts.len()).map_err(|_| {
                        DirectSnowStage3V11AttachmentError::Identity(
                            "adaptive terminal receipt ordinal width",
                        )
                    })?)
                    .ok_or(DirectSnowStage3V11AttachmentError::Identity(
                        "adaptive terminal receipt ordinal overflow",
                    ))?;
                let prior_identity = &carrier_phase.transition.probe_child_identity;
                let wb14_replay_trial_sha256 = prior_identity.receipt_sha256;
                let wb14_replay_beginning_owner_set_sha256 = Digest32::from_bytes(
                    crate::direct_runtime::wb14_child_replay_binding(
                        &carrier_phase.wb14_child_replay_bytes,
                    )
                    .map_err(|_| {
                        DirectSnowStage3V11AttachmentError::Terminal(
                            "adaptive endpoint WB14 replay binding",
                        )
                    })?
                    .parent_beginning_complete_owner_set_sha256,
                );
                carrier_phase.transition.probe_child_identity =
                    CoveredProbeChildIdentityV1::try_new(ProbeChildAuthorityV1 {
                        parent_transaction_sha256: prior_identity.parent_transaction_sha256,
                        enclosing_parent_support: prior_identity.enclosing_parent_support,
                        trial_support: step.support,
                        physical_child_ordinal,
                        attempt_ordinal: prior_identity.attempt_ordinal,
                        role: prior_identity.role,
                        beginning_joint_sha256: prior_identity.beginning_joint_sha256,
                        beginning_owner_set_sha256: prior_identity.beginning_owner_set_sha256,
                        complete_forcing_sha256: prior_identity.complete_forcing_sha256,
                        topology_sha256: prior_identity.topology_sha256,
                    })?;
                let trial_receipt = carrier_phase
                    .ending_candidates
                    .terminal_snow_soil_trial_receipt()
                    .cloned()
                    .ok_or(DirectSnowStage3V11AttachmentError::Terminal(
                        "adaptive preterminal snow-soil trial receipt",
                    ))?;
                trial_receipt.validate().map_err(|_| {
                    DirectSnowStage3V11AttachmentError::Terminal(
                        "adaptive preterminal snow-soil trial seal",
                    )
                })?;
                let mut ending_stage3 = stage3.clone();
                ending_stage3.insert(*lane_id, step.ending_state.clone());
                let ending =
                    crate::v9_real_consumer_shadow::CoveredCarrierEphemeralCandidatesV1::try_new(
                        step.hydrology_ending_joint.clone(),
                        carrier_phase.ending_candidates.shadow().clone(),
                        ending_stage3.clone(),
                    )?;
                let projected = prepared
                    .coupled_subslab(step.support, physical_child_ordinal)?
                    .retain_active_snow_lanes(&active_lanes)?;
                let terminal_step = step.ending_ice_kg_m2 == 0.0;
                let mut step_event = event;
                step_event.event_occurred = terminal_step;
                step_event.terminal_entry_offset_seconds = 0.0;
                step_event.requested_seconds = f64::from_bits(step.support.duration_s_bits());
                step_event.hour_offset_seconds = step_event.requested_seconds;
                step_event.evaluated_seconds = step_event.requested_seconds;
                step_event.unevaluated_seconds = 0.0;
                step_event.start_ice_kg_m2 = step.beginning_ice_kg_m2;
                step_event.start_liquid_kg_m2 = step.beginning_liquid_kg_m2;
                step_event.start_cold_content_j_m2 = step.beginning_cold_content_j_m2;
                step_event.end_ice_kg_m2 = step.ending_ice_kg_m2;
                step_event.terminal_liquid_kg_m2 = step.ending_liquid_kg_m2;
                step_event.end_cold_content_j_m2 = step.ending_cold_content_j_m2;
                step_event.complete_energy_j_m2 = step.complete_energy_j_m2;
                step_event.shortwave_energy_j_m2 = step.shortwave_energy_j_m2;
                step_event.longwave_energy_j_m2 = step.longwave_energy_j_m2;
                step_event.sensible_energy_j_m2 = step.sensible_energy_j_m2;
                step_event.latent_energy_j_m2 = step.latent_energy_j_m2;
                step_event.advected_energy_j_m2 = step.advected_energy_j_m2;
                step_event.snow_soil_heat_energy_j_m2 = step.snow_soil_heat_energy_j_m2;
                step_event.cold_energy_change_j_m2 = step.cold_energy_change_j_m2;
                step_event.refrozen_kg_m2 = step.refrozen_kg_m2;
                step_event.deposition_kg_m2 = step.deposition_kg_m2;
                step_event.sublimation_kg_m2 = step.sublimation_kg_m2;
                step_event.melt_kg_m2 = step.melt_kg_m2;
                step_event.terminal_unallocated_energy_j_m2 = step.unallocated_energy_j_m2;
                step_event.external_liquid_kg_m2 = step.external_liquid_kg_m2;
                let [solid_residual, liquid_residual, energy_residual] =
                    reconstruct_terminal_closure_v1(TerminalClosureOperandsV1::from(&step_event))?;
                step_event.solid_mass_closure_residual_kg_m2 = solid_residual;
                step_event.liquid_mass_closure_residual_kg_m2 = liquid_residual;
                step_event.energy_closure_residual_j_m2 = energy_residual;
                step_event.event_bracket_width_seconds = 0.0;
                step_event.event_bracket_lower_seconds = step_event.requested_seconds;
                step_event.event_bracket_upper_seconds = step_event.requested_seconds;
                step_event.event_bracket_lower_solid_kg_m2 = step.ending_ice_kg_m2;
                step_event.event_bracket_upper_solid_kg_m2 = step.ending_ice_kg_m2;
                let endpoint = Box::new(ExactCoveredTerminalEndpointV1 {
                    support: step.support,
                    lane_id: *lane_id,
                    event: step_event,
                    event_result_digest: canonical_terminal_event_result_digest(&step_event)?,
                    forcing_sha256: canonical_stage3_support_forcing_digest(
                        &projected.support_forcing_by_lane,
                    ),
                    ending: ending.clone(),
                    carrier_phase: Box::new(carrier_phase.clone()),
                    carrier_phase_chain: vec![carrier_phase.clone()],
                    wb14_replay_trial_sha256,
                    wb14_replay_beginning_owner_set_sha256,
                    terminal_snow_soil_trial_receipt: trial_receipt,
                    final_child_actual_vapor_to_canopy_air_kg_m2:
                        step.sublimation_kg_m2 - step.deposition_kg_m2,
                    terminal_snow_soil_trial_receipt_chains_by_lane: carrier_phase
                        .batch_terminal_snow_soil_trial_receipts_by_lane
                        .iter()
                        .map(|(lane_id, receipt)| (*lane_id, vec![receipt.clone()]))
                        .collect(),
                    endpoint_receipt_sha256: framed_sha256(
                        "stage3-v11-adaptive-preterminal-endpoint-v1",
                        &[
                            FramedField {
                                tag: "support_start",
                                value: &step.support.start_ns().get().to_be_bytes(),
                            },
                            FramedField {
                                tag: "support_end",
                                value: &step.support.end_ns().get().to_be_bytes(),
                            },
                            FramedField {
                                tag: "lane",
                                value: &lane_id.to_be_bytes(),
                            },
                            FramedField {
                                tag: "ending_joint",
                                value: ending.joint().receipt_sha256().as_bytes(),
                            },
                        ],
                    )?,
                });
                let (next_parent, next_consumer, next_clock, next_stage3, receipt) =
                    execute_covered_real_v11_subslab(
                        context,
                        &parent,
                        &consumer,
                        &clock,
                        &projected,
                        day_index,
                        interval_index,
                        forcing_receipt,
                        stage3,
                        beginning_terminal_parcels,
                        selected_upper_bound_s,
                        Some(std::slice::from_ref(&endpoint)),
                    )?;
                if (!terminal_step && !receipt.terminal_events.is_empty())
                    || (terminal_step && receipt.terminal_events.get(lane_id) != Some(&step_event))
                {
                    return Err(DirectSnowStage3V11AttachmentError::Terminal(
                        "adaptive microstep installed event posture",
                    ));
                }
                let installed_owner_bytes = next_consumer.canonical_owner_state_bytes()?;
                let accepted_owner_bytes = ending.shadow().canonical_owner_state_bytes()?;
                if installed_owner_bytes != accepted_owner_bytes {
                    return Err(DirectSnowStage3V11AttachmentError::Terminal(
                        "adaptive preterminal carrier-owner installation divergence",
                    ));
                }
                if next_stage3 != ending_stage3 {
                    return Err(DirectSnowStage3V11AttachmentError::Terminal(
                        "adaptive preterminal snow-owner installation divergence",
                    ));
                }
                parent = next_parent;
                consumer = next_consumer;
                clock = next_clock;
                stage3 = next_stage3;
                receipts.push(receipt);
                if terminal_step {
                    let tick = step.support.end_ns();
                    let candidates = vec![Stage3V11ActualTerminalCandidateV1 {
                        lane_id: *lane_id,
                        tick,
                        support: step.support,
                        event: step_event,
                        event_result_digest: endpoint.event_result_digest,
                        terminal_state_sha256: digest_bytes(
                            &Wb11HydrologyKernel::serialize_stage3_persistent_state(
                                stage3.get(lane_id).ok_or(
                                    DirectSnowStage3V11AttachmentError::Identity(
                                        "adaptive terminal installed lane",
                                    ),
                                )?,
                            )?,
                        ),
                        shortened_forcing_sha256: endpoint.forcing_sha256,
                        shortened_owner_set_sha256: endpoint.ending.joint().receipt_sha256(),
                        exact_endpoint_receipt_sha256: Some(endpoint.endpoint_receipt_sha256),
                        terminal_snow_soil_trial_receipt_sha256: Some(
                            endpoint.terminal_snow_soil_trial_receipt.receipt_sha256,
                        ),
                    }];
                    let terminal_group_ordinal = terminal_group_ordinal_after_physical_support_v1(
                        event_ordinal,
                        clock.event_ordinal(),
                    )?;
                    let mut group = select_common_earliest_actual_terminal_group_v1(
                        beginning_clock.parent_support(),
                        terminal_group_ordinal,
                        &active_lanes,
                        candidates,
                    )?
                    .ok_or(DirectSnowStage3V11AttachmentError::Terminal(
                        "adaptive terminal event group",
                    ))?;
                    let installed_receipt =
                        receipts
                            .last()
                            .ok_or(DirectSnowStage3V11AttachmentError::Identity(
                                "adaptive terminal installed receipt",
                            ))?;
                    let (parent, clock, stage3, parcels, accepted_event_receipt) =
                        apply_actual_terminal_group(
                            context,
                            parent,
                            clock,
                            stage3,
                            beginning_terminal_parcels,
                            physical_child_ordinal,
                            &mut group,
                            &installed_receipt.lane_receipts,
                            &installed_receipt.destination_receipts,
                        )?;
                    consumer
                        .retain_accepted_publication_zero_duration_event(&accepted_event_receipt)?;
                    group.accepted_event_receipt = Some(accepted_event_receipt);
                    group.accepted_group_receipt_sha256 =
                        Some(accepted_terminal_group_digest(&group)?);
                    return Ok(Some(ActualTerminalSubslabV1 {
                        parent,
                        consumer,
                        clock,
                        stage3,
                        receipts,
                        group: Some(group),
                        parcels,
                    }));
                }
            }
            return Ok(Some(ActualTerminalSubslabV1 {
                parent,
                consumer,
                clock,
                stage3,
                receipts,
                group: None,
                parcels: Vec::new(),
            }));
        }
        // A non-event result has no exact terminal tick to localize. When a
        // positive solid source made its enclosing trace ineligible for
        // child reuse, let the caller execute the same projected support via
        // the ordinary terminal/reappearance path instead of fabricating an
        // ExactEndpoint request from the support end.
        if !event.event_occurred {
            continue;
        }
        let candidate_offsets = localized_terminal_candidate_offsets_v1(
            event.terminal_entry_offset_seconds,
            event.evaluated_seconds,
            event.hour_offset_seconds,
            event.event_bracket_lower_seconds,
            event.event_bracket_upper_seconds,
        )
        .ok_or(DirectSnowStage3V11AttachmentError::Terminal(
            "terminal discovery relative chronology",
        ))?;
        let event_relative = quantize_seconds_to_tick(
            ModelTimeNs::new(0),
            ModelTimeNs::new(prepared.support.duration_ns()),
            candidate_offsets[0],
        )?;
        let event_tick = ModelTimeNs::new(prepared.support.start_ns().get() + event_relative.get());
        let event_support = TimeSupport::new(prepared.support.start_ns(), event_tick)?;
        let event_projected = prepared.coupled_subslab(event_support, current_child_ordinal)?;
        discovery_candidates.insert(
            (event_tick, *lane_id),
            Stage3V11ActualTerminalCandidateV1 {
                lane_id: *lane_id,
                tick: event_tick,
                support: event_support,
                event,
                event_result_digest: canonical_terminal_event_result_digest(&event)?,
                terminal_state_sha256: digest_bytes(
                    &Wb11HydrologyKernel::serialize_stage3_persistent_state(&result.state)?,
                ),
                shortened_forcing_sha256: canonical_stage3_support_forcing_digest(
                    &event_projected.support_forcing_by_lane,
                ),
                shortened_owner_set_sha256: result
                    .covered_terminal_ending_joint
                    .as_ref()
                    .map(CoveredTerminalJointTrialStateV1::receipt_sha256)
                    .ok_or(DirectSnowStage3V11AttachmentError::Terminal(
                        "covered discovery selected joint",
                    ))?,
                exact_endpoint_receipt_sha256: None,
                terminal_snow_soil_trial_receipt_sha256: None,
            },
        );
        for seconds in candidate_offsets {
            if seconds.is_finite()
                && seconds > 0.0
                && seconds <= f64::from_bits(prepared.support.duration_s_bits())
            {
                let relative = quantize_seconds_to_tick(
                    ModelTimeNs::new(0),
                    ModelTimeNs::new(prepared.support.duration_ns()),
                    seconds,
                )?;
                candidate_ticks.insert(ModelTimeNs::new(
                    prepared.support.start_ns().get() + relative.get(),
                ));
            }
        }
    }
    for tick in candidate_ticks {
        let exact_discovery = discovery_candidates
            .iter()
            .filter(|((candidate_tick, _), _)| *candidate_tick == tick)
            .map(|(_, candidate)| candidate.clone())
            .collect::<Vec<_>>();
        if exact_discovery.is_empty() {
            continue;
        }
        let pre = tick.get() - prepared.support.start_ns().get();
        let post = beginning_clock.parent_support().end_ns().get() - tick.get();
        if (pre != 0 && pre < context.minimum_support_ns)
            || (post != 0 && post < context.minimum_support_ns)
        {
            return Err(DirectSnowStage3V11AttachmentError::Support(
                "terminal event creates positive subminimum support",
            ));
        }
        let support = TimeSupport::new(prepared.support.start_ns(), tick)?;
        let projected = prepared.coupled_subslab(support, current_child_ordinal)?;
        let mut exact_endpoints = Vec::new();
        for discovery in &exact_discovery {
            let (exact_result, exact_candidates, exact_carrier_phases) =
                evaluate_covered_terminal_candidate_with_evidence_v1::<M>(
                    beginning_consumer,
                    beginning_clock,
                    &projected,
                    day_index,
                    interval_index,
                    beginning_stage3,
                    beginning_terminal_parcels,
                    selected_upper_bound_s,
                    current_child_ordinal,
                    discovery.lane_id,
                    CoveredTerminalExecutionMode::ExactEndpoint {
                        expected_tick: tick,
                    },
                    evidence,
                )?;
            exact_endpoints.push(prepare_exact_terminal_endpoint_v1(
                discovery,
                &exact_result,
                canonical_stage3_support_forcing_digest(&projected.support_forcing_by_lane),
                &exact_candidates,
                &exact_carrier_phases,
            )?);
        }
        let exact = exact_endpoints
            .first()
            .ok_or(DirectSnowStage3V11AttachmentError::Terminal(
                "missing exact endpoint value",
            ))?;
        let (parent, installed_consumer, clock, installed_stage3, receipt) =
            execute_covered_real_v11_subslab(
                context,
                beginning_parent,
                beginning_consumer,
                beginning_clock,
                &projected,
                day_index,
                interval_index,
                forcing_receipt,
                beginning_stage3.clone(),
                beginning_terminal_parcels,
                selected_upper_bound_s,
                Some(&exact_endpoints),
            )?;
        if receipt.terminal_events.is_empty() {
            continue;
        }
        let expected_installed = precomputed_terminal_package_v1(
            &exact_endpoints,
            beginning_terminal_parcels,
            receipt.accepted_slab_sha256,
            complete_owner_set_digest(beginning_clock.owners())?,
        )?;
        if exact.support != support
            || exact.forcing_sha256
                != canonical_stage3_support_forcing_digest(&projected.support_forcing_by_lane)
            || exact.ending.shadow().canonical_owner_state_bytes()?
                != installed_consumer.canonical_owner_state_bytes()?
            || expected_installed.ending_stage3_by_lane != installed_stage3
            || expected_installed.terminal_events != receipt.terminal_events
        {
            return Err(DirectSnowStage3V11AttachmentError::Terminal(
                "exact endpoint/precomputed installer value divergence",
            ));
        }
        // The unpublished carrier shadow supplies exact physical value
        // evidence only. Preserve the installed consumer so its accepted
        // publication support/history cannot be replaced by the trial
        // candidate after the value-equality join above.
        let mut consumer = installed_consumer;
        let stage3 = expected_installed.ending_stage3_by_lane;
        let candidates = exact_endpoints
            .iter()
            .map(|endpoint| {
                let state = endpoint
                    .ending
                    .stage3_by_lane()
                    .get(&endpoint.lane_id)
                    .ok_or(DirectSnowStage3V11AttachmentError::Identity(
                        "exact endpoint installed snow lane",
                    ))?;
                Ok(Stage3V11ActualTerminalCandidateV1 {
                    lane_id: endpoint.lane_id,
                    tick,
                    support,
                    event: endpoint.event,
                    event_result_digest: endpoint.event_result_digest,
                    terminal_state_sha256: digest_bytes(
                        &Wb11HydrologyKernel::serialize_stage3_persistent_state(state)?,
                    ),
                    shortened_forcing_sha256: endpoint.forcing_sha256,
                    shortened_owner_set_sha256: endpoint.ending.joint().receipt_sha256(),
                    exact_endpoint_receipt_sha256: Some(endpoint.endpoint_receipt_sha256),
                    terminal_snow_soil_trial_receipt_sha256: Some(
                        endpoint.terminal_snow_soil_trial_receipt.receipt_sha256,
                    ),
                })
            })
            .collect::<Result<Vec<_>, DirectSnowStage3V11AttachmentError>>()?;
        let terminal_group_ordinal =
            terminal_group_ordinal_after_physical_support_v1(event_ordinal, clock.event_ordinal())?;
        let Some(mut group) = select_common_earliest_actual_terminal_group_v1(
            beginning_clock.parent_support(),
            terminal_group_ordinal,
            &active_lanes,
            candidates,
        )?
        else {
            continue;
        };
        if group.tick != tick
            || group.terminating_lanes
                != receipt
                    .terminal_events
                    .keys()
                    .copied()
                    .collect::<BTreeSet<_>>()
        {
            continue;
        }
        let (parent, clock, stage3, parcels, accepted_event_receipt) = apply_actual_terminal_group(
            context,
            parent,
            clock,
            stage3,
            beginning_terminal_parcels,
            current_child_ordinal,
            &mut group,
            &receipt.lane_receipts,
            &receipt.destination_receipts,
        )?;
        consumer.retain_accepted_publication_zero_duration_event(&accepted_event_receipt)?;
        group.accepted_event_receipt = Some(accepted_event_receipt.clone());
        group.accepted_group_receipt_sha256 = Some(accepted_terminal_group_digest(&group)?);
        return Ok(Some(ActualTerminalSubslabV1 {
            parent,
            consumer,
            clock,
            stage3,
            receipts: vec![receipt],
            group: Some(group),
            parcels,
        }));
    }
    Ok(None)
}

fn terminal_group_ordinal_after_physical_support_v1(
    beginning_event_ordinal: u64,
    ending_event_ordinal: u32,
) -> Result<u64, DirectSnowStage3V11AttachmentError> {
    let ending_event_ordinal = u64::from(ending_event_ordinal);
    let maximum_ending_ordinal = beginning_event_ordinal.checked_add(1).ok_or(
        DirectSnowStage3V11AttachmentError::Identity("terminal pre-support event ordinal overflow"),
    )?;
    if !(beginning_event_ordinal..=maximum_ending_ordinal).contains(&ending_event_ordinal) {
        return Err(DirectSnowStage3V11AttachmentError::Identity(
            "terminal physical-support event ordinal chronology",
        ));
    }
    Ok(ending_event_ordinal)
}

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

include!("snow_stage3_v11_terminal_receiver_topology.rs");

include!("snow_stage3_v11_terminal_proposal_core.rs");

fn terminal_parcels_for_event_group(
    configuration: &DirectSurfaceLiquidConfiguration,
    candidate: &Stage3V11ActualTerminalCandidateV1,
    group: &Stage3V11TerminalEventGroupV1,
    parent_transaction_id: Digest32,
    proposal_core: Digest32,
) -> Result<Vec<DirectSnowStage3V11TerminalParcel>, DirectSnowStage3V11AttachmentError> {
    let topology = terminal_receiver_topology(configuration, candidate.lane_id)?;
    let mass = candidate.event.terminal_liquid_kg_m2;
    let (temperature_k, specific_liquid_enthalpy_j_kg) =
        terminal_liquid_thermodynamics_v1(mass, candidate.event.terminal_unallocated_energy_j_m2)?;
    let mut parcel = DirectSnowStage3V11TerminalParcel {
        support: candidate.support,
        source_lane_id: candidate.lane_id,
        parent_transaction_id,
        event_ordinal: u32::try_from(group.ordinal).map_err(|_| {
            DirectSnowStage3V11AttachmentError::Identity("terminal event ordinal width")
        })?,
        terminal_event_proposal_core_id: proposal_core,
        event_result_digest: candidate.event_result_digest,
        receiver_topology_sha256: topology.digest,
        destination_ofe_id: topology.destination_ofe.to_string(),
        receiver_destinations: topology.receiver_destinations,
        mass_kg_m2_tile_ground: mass,
        temperature_k,
        specific_liquid_enthalpy_j_kg,
        posture: DirectSnowStage3V11TerminalParcelPosture::ProducedUnconsumed,
        parcel_digest: Digest32::zero(),
    };
    parcel.parcel_digest = crate::snow_owner_v4::canonical_terminal_parcel_digest(&parcel)
        .map_err(|_| DirectSnowStage3V11AttachmentError::Identity("terminal parcel digest"))?;
    Ok(vec![parcel])
}

#[allow(
    clippy::too_many_arguments,
    clippy::too_many_lines,
    clippy::type_complexity
)]
fn execute_covered_real_v11_subslab(
    context: &DirectSnowStage3V11StaticContext,
    beginning_parent: &V11ParentTransaction,
    beginning_consumer: &DirectV10RealConsumerShadow,
    beginning_clock: &CoupledClockStateV1,
    prepared: &DirectSnowStage3V11PreparedSupport,
    day_index: usize,
    interval_index: usize,
    forcing_receipt: Digest32,
    beginning_stage3: BTreeMap<u32, DirectSnowStage3PersistentState>,
    pending_terminal_parcels: &BTreeMap<Digest32, DirectSnowStage3V11TerminalParcel>,
    selected_upper_bound_s: f64,
    terminal_endpoints: Option<&[Box<ExactCoveredTerminalEndpointV1>]>,
) -> Result<
    (
        V11ParentTransaction,
        DirectV10RealConsumerShadow,
        CoupledClockStateV1,
        BTreeMap<u32, DirectSnowStage3PersistentState>,
        Stage3CoupledSubslabReceiptV1,
    ),
    DirectSnowStage3V11AttachmentError,
> {
    #[cfg(test)]
    let complete_owner_started = std::time::Instant::now();
    if beginning_parent.parent_transaction_id() != beginning_clock.parent_transaction_id() {
        return Err(DirectSnowStage3V11AttachmentError::Identity(
            "covered V11/coupled-time parent identity",
        ));
    }
    if beginning_clock.accepted_until() != prepared.support.start_ns()
        || prepared.support.start_ns() < beginning_clock.parent_support().start_ns()
        || prepared.support.end_ns() > beginning_clock.parent_support().end_ns()
    {
        return Err(DirectSnowStage3V11AttachmentError::Identity(
            "covered V11/coupled-time subslab support",
        ));
    }
    if beginning_clock.owners().len() != openwepp_vegetation::v11::V11_COMPLETE_OWNER_MANIFEST.len()
    {
        return Err(DirectSnowStage3V11AttachmentError::Identity(
            "covered V11/coupled-time complete owner set",
        ));
    }
    if !prepared.has_snow_surface_forcing() {
        return Err(DirectSnowStage3V11AttachmentError::Identity(
            "covered V11/coupled-time snow surface forcing",
        ));
    }
    let parent_id = beginning_parent.parent_transaction_id();
    let support = prepared.support;
    #[cfg(test)]
    crate::v9_real_consumer_shadow::audit_covered_carrier_support(support);
    let start = support.start_ns();
    let end = support.end_ns();
    let constraint = StepConstraintV1::new(
        parent_id,
        start,
        end,
        "v11-snow-covered-real-consumer".to_owned(),
        ConstraintClass::HardBoundary,
        context.controller_policy,
        context.calendar_receipt,
        forcing_receipt,
    )?;
    let reduction = reduce_constraints(&[constraint], parent_id, start, end, None)?;
    let ledger_digest = complete_owner_set_digest(beginning_clock.owners())?;
    let mut ledger_preimage = Vec::new();
    ledger_preimage.extend_from_slice(parent_id.digest().as_bytes());
    ledger_preimage.extend_from_slice(&start.get().to_be_bytes());
    ledger_preimage.extend_from_slice(&end.get().to_be_bytes());
    let ledger = LedgerEntryV1::new(
        "complete-owner-custody".to_owned(),
        "canonical-owner-state".to_owned(),
        ledger_digest,
        ledger_digest,
        digest_bytes(&ledger_preimage),
    )?;
    let segment = beginning_clock.active_segment_id();
    let covered_interval = prepared.covered_v11_interval.as_ref().ok_or(
        DirectSnowStage3V11AttachmentError::Support(
            "covered support missing covered V11 projection",
        ),
    )?;

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
    let mut provisional_stack = DirectV11SnowCoveredRealConsumerStack::new(
        beginning_consumer,
        DirectV11SnowCoveredStackInputs {
            interval: covered_interval,
            stage3_inputs_by_lane: &prepared.snow_inputs_by_lane,
            stage3_forcing_by_lane: &prepared.support_forcing_by_lane,
            snow_surface_forcing_by_destination: &prepared.snow_surface_forcing_by_destination,
            stage3_beginning_by_lane: beginning_stage3.clone(),
            pending_terminal_parcels: pending_terminal_parcels.clone(),
            day_index,
            interval_index,
            finalize_wb14_parent_interval: support.end_ns()
                == beginning_clock.parent_support().end_ns(),
            wb14_coupled_child_binding: crate::direct_runtime::DirectWb14CoupledChildBindingV1 {
                proposed_upper_bound_s_bits: selected_upper_bound_s.to_bits(),
                coupled_parent_transaction_sha256: *parent_id.digest().as_bytes(),
                accepted_slab_sha256: *provisional_receipt.slab_id().digest().as_bytes(),
                parent_beginning_complete_owner_set_sha256: *ledger_digest.as_bytes(),
                parent_support_start_ns: beginning_clock.parent_support().start_ns().get(),
                parent_support_end_ns: beginning_clock.parent_support().end_ns().get(),
                child_support_start_ns: support.start_ns().get() as u128,
                child_support_end_ns: support.end_ns().get() as u128,
            },
        },
    );
    if let Some(endpoints) = terminal_endpoints {
        let endpoint = precomputed_terminal_package_v1(
            endpoints,
            pending_terminal_parcels,
            provisional_receipt.slab_id().digest(),
            ledger_digest,
        )?;
        provisional_stack = if terminal_provisional_publication_deferral_enabled() {
            provisional_stack.with_precomputed_terminal_provisional_endpoint(endpoint)?
        } else {
            provisional_stack.with_precomputed_terminal_accepted_endpoint(endpoint)
        };
    }
    let mut provisional_executor = crate::v11_vegetation_consumer::DirectV11VegetationExecutor {
        stack: provisional_stack,
    };
    #[cfg(test)]
    let provisional_started = std::time::Instant::now();
    #[cfg(test)]
    let provisional_capture_regime =
        crate::snow_stage3_v11_attachment::enter_accepted_publication_capture_regime_v1(
            crate::snow_stage3_v11_attachment::AcceptedPublicationCaptureRegimeV1::ProvisionalClock,
        );
    let provisional_segment = execute_direct_v11_segment(
        &context.vegetation_configuration,
        beginning_parent,
        &provisional_receipt,
        &mut provisional_executor,
    )?;
    if terminal_endpoints.is_some()
        && provisional_executor.stack.last_publication_retained()
            != Some(!terminal_provisional_publication_deferral_enabled())
    {
        return Err(DirectSnowStage3V11AttachmentError::Identity(
            "terminal provisional publication posture",
        ));
    }
    #[cfg(test)]
    drop(provisional_capture_regime);
    #[cfg(test)]
    record_adaptive_performance_span_v1(
        "covered_v11_provisional_stack",
        support.duration_ns(),
        provisional_started,
    );
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
    let final_binding = crate::direct_runtime::DirectWb14CoupledChildBindingV1 {
        proposed_upper_bound_s_bits: selected_upper_bound_s.to_bits(),
        coupled_parent_transaction_sha256: *parent_id.digest().as_bytes(),
        accepted_slab_sha256: *final_receipt.slab_id().digest().as_bytes(),
        parent_beginning_complete_owner_set_sha256: *ledger_digest.as_bytes(),
        parent_support_start_ns: beginning_clock.parent_support().start_ns().get(),
        parent_support_end_ns: beginning_clock.parent_support().end_ns().get(),
        child_support_start_ns: support.start_ns().get() as u128,
        child_support_end_ns: support.end_ns().get() as u128,
    };
    let final_stack = if let Some(endpoints) = terminal_endpoints {
        let endpoint = precomputed_terminal_package_v1(
            endpoints,
            pending_terminal_parcels,
            final_receipt.slab_id().digest(),
            ledger_digest,
        )?;
        if terminal_provisional_publication_deferral_enabled() {
            provisional_executor
                .stack
                .prepare_terminal_physical_reuse(final_binding, endpoint)?
        } else {
            DirectV11SnowCoveredRealConsumerStack::new(
                beginning_consumer,
                DirectV11SnowCoveredStackInputs {
                    interval: covered_interval,
                    stage3_inputs_by_lane: &prepared.snow_inputs_by_lane,
                    stage3_forcing_by_lane: &prepared.support_forcing_by_lane,
                    snow_surface_forcing_by_destination: &prepared
                        .snow_surface_forcing_by_destination,
                    stage3_beginning_by_lane: beginning_stage3,
                    pending_terminal_parcels: pending_terminal_parcels.clone(),
                    day_index,
                    interval_index,
                    finalize_wb14_parent_interval: support.end_ns()
                        == beginning_clock.parent_support().end_ns(),
                    wb14_coupled_child_binding: final_binding,
                },
            )
            .with_precomputed_terminal_accepted_endpoint(endpoint)
        }
    } else if ordinary_covered_physical_reuse_enabled() {
        provisional_executor
            .stack
            .prepare_ordinary_physical_reuse(final_binding)?
    } else {
        DirectV11SnowCoveredRealConsumerStack::new(
            beginning_consumer,
            DirectV11SnowCoveredStackInputs {
                interval: covered_interval,
                stage3_inputs_by_lane: &prepared.snow_inputs_by_lane,
                stage3_forcing_by_lane: &prepared.support_forcing_by_lane,
                snow_surface_forcing_by_destination: &prepared.snow_surface_forcing_by_destination,
                stage3_beginning_by_lane: beginning_stage3,
                pending_terminal_parcels: pending_terminal_parcels.clone(),
                day_index,
                interval_index,
                finalize_wb14_parent_interval: support.end_ns()
                    == beginning_clock.parent_support().end_ns(),
                wb14_coupled_child_binding: final_binding,
            },
        )
    };
    let mut final_executor =
        crate::v11_vegetation_consumer::DirectV11VegetationExecutor { stack: final_stack };
    #[cfg(test)]
    let final_started = std::time::Instant::now();
    #[cfg(test)]
    let final_capture_regime =
        crate::snow_stage3_v11_attachment::enter_accepted_publication_capture_regime_v1(
            crate::snow_stage3_v11_attachment::AcceptedPublicationCaptureRegimeV1::FinalClock,
        );
    let final_segment = execute_direct_v11_segment(
        &context.vegetation_configuration,
        beginning_parent,
        &final_receipt,
        &mut final_executor,
    )?;
    #[cfg(test)]
    drop(final_capture_regime);
    #[cfg(test)]
    record_adaptive_performance_span_v1(
        "covered_v11_final_stack",
        support.duration_ns(),
        final_started,
    );
    if final_segment.ending_resource_owners != provisional_segment.ending_resource_owners {
        return Err(DirectSnowStage3V11AttachmentError::Identity(
            "covered V11 ending owner fixed point",
        ));
    }
    #[cfg(test)]
    let validation_started = std::time::Instant::now();
    let ending_stage3 = final_executor.stack.take_staged_stage3().ok_or(
        DirectSnowStage3V11AttachmentError::Identity("missing staged covered Stage-3 ending"),
    )?;
    let final_boundary_receipts = final_executor
        .stack
        .last_final_boundary_receipts()
        .ok_or(DirectSnowStage3V11AttachmentError::Identity(
            "missing final covered boundary receipt set",
        ))?
        .clone();
    let final_lane_receipts = final_executor
        .stack
        .last_lane_boundary_receipts()
        .ok_or(DirectSnowStage3V11AttachmentError::Identity(
            "missing final covered lane-boundary receipt set",
        ))?
        .clone();
    let (wb14_child_receipt_set, wb14_parent_receipt_set) = final_executor
        .stack
        .last_wb14_receipt_sets()
        .ok_or(DirectSnowStage3V11AttachmentError::Identity(
            "missing live WB14 receipt-set attachment",
        ))?;
    let wb14_child_receipt_set = parse_lower_hex_digest(wb14_child_receipt_set)?;
    let wb14_parent_receipt_set = wb14_parent_receipt_set
        .map(parse_lower_hex_digest)
        .transpose()?;
    let retained_snow_soil_heat_receipts =
        final_executor.stack.last_snow_soil_heat_receipts().ok_or(
            DirectSnowStage3V11AttachmentError::Identity("missing snow-soil heat receipt set"),
        )?;
    let terminal_snow_soil_heat_receipts = final_executor
        .stack
        .last_terminal_snow_soil_heat_receipts()
        .ok_or(DirectSnowStage3V11AttachmentError::Identity(
            "missing terminal snow-soil heat receipt set",
        ))?
        .clone();
    let adaptive_terminal_snow_soil_trial_receipts = final_executor
        .stack
        .last_adaptive_terminal_snow_soil_trial_receipts()
        .ok_or(DirectSnowStage3V11AttachmentError::Identity(
            "missing adaptive terminal snow-soil trial receipt set",
        ))?;
    let terminal_events = final_executor
        .stack
        .last_terminal_events()
        .ok_or(DirectSnowStage3V11AttachmentError::Identity(
            "missing terminal event receipt set",
        ))?
        .clone();
    let physical_outcome_ledgers = final_executor
        .stack
        .last_physical_outcome_ledgers()
        .ok_or(DirectSnowStage3V11AttachmentError::Identity(
            "missing physical outcome ledger set",
        ))?
        .clone();
    let snow_soil_heat_receipts = retained_snow_soil_heat_receipts
        .iter()
        .filter(|(lane_id, _)| {
            let Some(beginning) = final_executor.stack.stage3_beginning_by_lane.get(lane_id) else {
                return false;
            };
            let Some(ending) = ending_stage3.get(lane_id) else {
                return false;
            };
            crate::hydrology::stage3_is_resolved_thermal_domain(beginning)
                && (crate::hydrology::stage3_is_resolved_thermal_domain(ending)
                    || crate::hydrology::stage3_is_terminal_event_domain(ending))
        })
        .map(|(lane_id, receipt)| (*lane_id, receipt.clone()))
        .collect::<BTreeMap<_, _>>();
    let adaptive_terminal_snow_soil_heat_receipts = retained_snow_soil_heat_receipts
        .iter()
        .filter(|(lane_id, _)| {
            final_executor
                .stack
                .stage3_beginning_by_lane
                .get(lane_id)
                .is_some_and(crate::hydrology::stage3_is_terminal_event_domain)
                && ending_stage3
                    .get(lane_id)
                    .is_some_and(crate::hydrology::stage3_is_terminal_event_domain)
        })
        .map(|(lane_id, receipt)| (*lane_id, receipt.clone()))
        .collect::<BTreeMap<_, _>>();
    let adaptive_terminal_snow_soil_trial_receipts = adaptive_terminal_snow_soil_trial_receipts
        .iter()
        .filter(|(lane_id, _)| {
            final_executor
                .stack
                .stage3_beginning_by_lane
                .get(lane_id)
                .is_some_and(crate::hydrology::stage3_is_terminal_event_domain)
                && ending_stage3
                    .get(lane_id)
                    .is_some_and(crate::hydrology::stage3_is_terminal_event_domain)
        })
        .map(|(lane_id, receipt)| (*lane_id, receipt.clone()))
        .collect::<BTreeMap<_, _>>();
    let installed_soil: openwepp_land_surface_energy::SoilThermalSnapshot = serde_json::from_slice(
        &final_segment
            .ending_resource_owners
            .get("soil_thermal")
            .ok_or(DirectSnowStage3V11AttachmentError::Identity(
                "missing installed soil owner",
            ))?
            .state_bytes,
    )
    .map_err(|_| DirectSnowStage3V11AttachmentError::Identity("installed soil owner bytes"))?;
    for (lane_id, receipt) in &snow_soil_heat_receipts {
        let state =
            ending_stage3
                .get(lane_id)
                .ok_or(DirectSnowStage3V11AttachmentError::Identity(
                    "snow-soil installed snow lane",
                ))?;
        let inputs = final_executor
            .stack
            .stage3_inputs_by_lane
            .get(lane_id)
            .ok_or(DirectSnowStage3V11AttachmentError::Identity(
                "snow-soil installed lane inputs",
            ))?;
        let installed_snow_bottom = if crate::hydrology::stage3_is_terminal_event_domain(state) {
            Wb11HydrologyKernel::project_stage3_terminal_bottom_volume_v1(
                state,
                inputs.surface_energy_options.atmospheric_pressure_pa,
            )?
        } else {
            Wb11HydrologyKernel::project_stage3_bottom_volume_v1(
                state,
                inputs.surface_energy_options.atmospheric_pressure_pa,
            )?
        };
        let soil_ofe = installed_soil
            .ofes
            .iter()
            .find(|value| value.ofe_id == receipt.ofe_id)
            .ok_or(DirectSnowStage3V11AttachmentError::Identity(
                "snow-soil installed soil OFE",
            ))?;
        let installed_soil_top =
            soil_ofe
                .ordered_layers
                .first()
                .ok_or(DirectSnowStage3V11AttachmentError::Identity(
                    "snow-soil installed top soil node",
                ))?;
        let installed_snow_identity = digest_bytes(&serde_json::to_vec(state).map_err(|_| {
            DirectSnowStage3V11AttachmentError::Identity("installed Stage-3 lane identity")
        })?);
        let installed_soil_identity =
            digest_bytes(&serde_json::to_vec(soil_ofe).map_err(|_| {
                DirectSnowStage3V11AttachmentError::Identity("installed soil OFE identity")
            })?);
        let close_temperature = |left: f64, right: f64| {
            left.is_finite() && right.is_finite() && (left - right).abs() <= 1.0e-8
        };
        if receipt.snow_candidate_ending_identity_sha256 != installed_snow_identity {
            return Err(DirectSnowStage3V11AttachmentError::Identity(
                "snow-soil installed snow candidate identity",
            ));
        }
        if receipt.soil_candidate_ending_identity_sha256 != installed_soil_identity {
            return Err(DirectSnowStage3V11AttachmentError::Identity(
                "snow-soil installed soil candidate identity",
            ));
        }
        validate_snow_soil_heat_receipt_installed_join(
            receipt,
            &installed_soil_top.layer_id,
            installed_snow_identity,
            installed_soil_identity,
        )?;
        if !close_temperature(
            installed_snow_bottom.temperature_k,
            receipt.ending_bottom_snow_temperature_k,
        ) || !close_temperature(
            installed_soil_top.temperature_k,
            receipt.ending_top_soil_temperature_k,
        ) {
            return Err(DirectSnowStage3V11AttachmentError::Identity(
                "snow-soil receipt/installed ending join",
            ));
        }
    }
    for (lane_id, receipt) in &terminal_snow_soil_heat_receipts {
        receipt.validate().map_err(|_| {
            DirectSnowStage3V11AttachmentError::Identity(
                "terminal snow-soil installed receipt seal",
            )
        })?;
        let state =
            ending_stage3
                .get(lane_id)
                .ok_or(DirectSnowStage3V11AttachmentError::Identity(
                    "terminal snow-soil installed snow lane",
                ))?;
        let soil_ofe = installed_soil
            .ofes
            .iter()
            .find(|value| value.ofe_id == receipt.ofe_id)
            .ok_or(DirectSnowStage3V11AttachmentError::Identity(
                "terminal snow-soil installed soil OFE",
            ))?;
        let installed_snow_identity = digest_bytes(
            &Wb11HydrologyKernel::serialize_stage3_persistent_state(state)?,
        );
        let installed_soil_identity =
            digest_bytes(&serde_json::to_vec(soil_ofe).map_err(|_| {
                DirectSnowStage3V11AttachmentError::Identity("terminal installed soil OFE identity")
            })?);
        let event_lane = terminal_events.contains_key(lane_id);
        let invalid_phase_posture = if event_lane {
            crate::hydrology::stage3_has_represented_ice(state)
                || !state.layers.is_empty()
                || state.detached_retained_liquid_kg_m2.to_bits() != 0.0_f64.to_bits()
        } else {
            !crate::hydrology::stage3_is_terminal_event_domain(state)
        };
        if invalid_phase_posture
            || receipt.ending_dormant_snow_owner_sha256 != installed_snow_identity
            || receipt.ending_soil_owner_sha256 != installed_soil_identity
        {
            return Err(DirectSnowStage3V11AttachmentError::Identity(
                "terminal snow-soil dormant installed join",
            ));
        }
    }
    let physical_custody_join = CoveredPhysicalCustodyJoinInputs {
        snow_soil_heat_receipts: &snow_soil_heat_receipts,
        adaptive_terminal_snow_soil_heat_receipts: &adaptive_terminal_snow_soil_heat_receipts,
        adaptive_terminal_snow_soil_trial_receipts: &adaptive_terminal_snow_soil_trial_receipts,
        terminal_snow_soil_heat_receipts: &terminal_snow_soil_heat_receipts,
        terminal_events: &terminal_events,
        physical_outcome_ledgers: &physical_outcome_ledgers,
        beginning_stage3_states: &final_executor.stack.stage3_beginning_by_lane,
        ending_stage3_states: &ending_stage3,
        pending_terminal_parcels,
    };
    let owner_join = CoveredParentOwnerJoinReceiptV1::try_new(
        context.run_identity,
        ParentIntervalId::derive(
            context.run_identity,
            context.calendar_receipt,
            forcing_receipt,
            support,
        )?
        .digest(),
        parent_id.digest(),
        final_receipt.segment_id().digest(),
        final_receipt.slab_id().digest(),
        forcing_receipt,
        ledger_digest,
        wb14_child_receipt_set,
        wb14_parent_receipt_set,
        support,
        &final_boundary_receipts,
        &final_lane_receipts,
        final_executor
            .stack
            .last_component_carrier_receipts()
            .ok_or(DirectSnowStage3V11AttachmentError::Identity(
                "missing component-resolved carrier receipt set",
            ))?,
        &physical_custody_join,
        &final_segment.ending_resource_owners,
    )?;
    owner_join.validate(
        &final_boundary_receipts,
        &final_lane_receipts,
        final_executor
            .stack
            .last_component_carrier_receipts()
            .ok_or(DirectSnowStage3V11AttachmentError::Identity(
                "missing component-resolved carrier receipt set",
            ))?,
        &physical_custody_join,
        &final_segment.ending_resource_owners,
    )?;
    let mut parent = beginning_parent.clone();
    accept_direct_v11_segment(
        &mut parent,
        &context.vegetation_configuration,
        final_segment,
        beginning_consumer,
    )?;
    if terminal_endpoints.is_some()
        && final_executor.stack.last_publication_retained() != Some(true)
    {
        return Err(DirectSnowStage3V11AttachmentError::Identity(
            "missing final accepted publication append",
        ));
    }
    let mut consumer = final_executor.stack.take_staged_ending().ok_or(
        DirectSnowStage3V11AttachmentError::Identity("missing staged covered ending"),
    )?;
    let (wb14_child_replay_bytes, wb14_parent_replay_bytes) =
        final_executor.stack.last_wb14_replay_bytes().ok_or(
            DirectSnowStage3V11AttachmentError::Identity("missing WB14 replay receipt payload"),
        )?;
    let wb14_child_replay_bytes = wb14_child_replay_bytes.to_vec();
    let wb14_parent_replay_bytes = wb14_parent_replay_bytes.map(ToOwned::to_owned);
    let mut parent_after_segment = parent;
    let terminal_lanes = terminal_events.keys().copied().collect::<BTreeSet<_>>();
    let post_support_liquid_receiver = consume_positive_support_snow_liquid_v1(
        context,
        &mut parent_after_segment,
        &mut consumer,
        &mut final_clock,
        support,
        owner_join.ending_complete_owner_set_sha256,
        &terminal_lanes,
    )?;
    let wb14_replay_trial_sha256 = terminal_endpoints
        .and_then(|endpoints| endpoints.first())
        .map_or(final_receipt.slab_id().digest(), |endpoint| {
            endpoint.wb14_replay_trial_sha256
        });
    let wb14_replay_beginning_owner_set_sha256 = terminal_endpoints
        .and_then(|endpoints| endpoints.first())
        .map_or(ledger_digest, |endpoint| {
            endpoint.wb14_replay_beginning_owner_set_sha256
        });
    let mut subslab_receipt = Stage3CoupledSubslabReceiptV1 {
        parent_support: beginning_clock.parent_support(),
        support,
        selected_upper_bound_s_bits: selected_upper_bound_s.to_bits(),
        accepted_slab_sha256: final_receipt.slab_id().digest(),
        wb14_replay_trial_sha256,
        wb14_replay_beginning_owner_set_sha256,
        wb14_child_receipt_set_sha256: owner_join.wb14_child_receipt_set_sha256,
        wb14_parent_receipt_set_sha256: owner_join.wb14_parent_receipt_set_sha256,
        wb14_child_replay_bytes,
        wb14_parent_replay_bytes,
        destination_receipts: final_boundary_receipts,
        lane_receipts: final_lane_receipts,
        physical_outcome_ledger_set_sha256:
            crate::v9_real_consumer_shadow::stage3_physical_outcome_ledger_set_digest(
                &physical_outcome_ledgers,
            ),
        terminal_events: terminal_events.clone(),
        post_support_liquid_receiver_event: post_support_liquid_receiver
            .as_ref()
            .map(|receiver| receiver.event.clone()),
        post_support_liquid_output_set_sha256: post_support_liquid_receiver
            .as_ref()
            .map(|receiver| receiver.output_set_sha256),
        post_support_liquid_mass_kg_m2_bits: post_support_liquid_receiver
            .as_ref()
            .map(|receiver| receiver.mass_kg_m2_bits),
        post_support_liquid_enthalpy_j_m2_bits: post_support_liquid_receiver
            .as_ref()
            .map(|receiver| receiver.enthalpy_j_m2_bits),
        post_support_liquid_surface_beginning_state: post_support_liquid_receiver
            .as_ref()
            .map(|receiver| receiver.surface_beginning_state.clone()),
        post_support_liquid_surface_ending_state: post_support_liquid_receiver
            .as_ref()
            .map(|receiver| receiver.surface_ending_state.clone()),
        post_support_liquid_custody_v2: None,
        owner_join,
        receipt_sha256: Digest32::zero(),
    };
    subslab_receipt.receipt_sha256 = subslab_receipt.reconstructed_digest()?;
    subslab_receipt.validate()?;
    if let Some(receiver) = &post_support_liquid_receiver {
        let custody = Stage3SupportLiquidCustodyV2::seal(
            &subslab_receipt,
            receiver.lse_beginning_state.clone(),
            receiver.lse_ending_state.clone(),
            receiver.receiver_receipt_set_sha256,
            receiver.receiver_receipts.clone(),
        )?;
        subslab_receipt.install_support_liquid_custody_v2(custody)?;
    }
    subslab_receipt.validate_support_liquid_custody_v2()?;
    #[cfg(test)]
    {
        record_adaptive_performance_span_v1(
            "covered_publication_and_validation",
            support.duration_ns(),
            validation_started,
        );
        record_adaptive_performance_span_v1(
            "covered_complete_owner_subslab_total",
            support.duration_ns(),
            complete_owner_started,
        );
    }
    Ok((
        parent_after_segment,
        consumer,
        final_clock,
        ending_stage3,
        subslab_receipt,
    ))
}

#[cfg(test)]
include!("snow_stage3_v11_terminal_execution_tests.rs");

include!("snow_stage3_v11_attachment_helpers.rs");
