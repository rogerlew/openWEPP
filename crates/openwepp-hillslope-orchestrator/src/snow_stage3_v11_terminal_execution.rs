struct ActualTerminalSubslabV1 {
    parent: V11ParentTransaction,
    consumer: DirectV10RealConsumerShadow,
    clock: CoupledClockStateV1,
    stage3: BTreeMap<u32, DirectSnowStage3PersistentState>,
    receipt: Stage3CoupledSubslabReceiptV1,
    group: Stage3V11TerminalEventGroupV1,
    parcels: Vec<DirectSnowStage3V11TerminalParcel>,
}

#[cfg(test)]
thread_local! {
    static TERMINAL_PROVIDER_SUPPORT_AUDIT: std::cell::RefCell<Option<Vec<TimeSupport>>> =
        const { std::cell::RefCell::new(None) };
}

#[cfg(test)]
pub(crate) fn begin_terminal_provider_support_audit() {
    TERMINAL_PROVIDER_SUPPORT_AUDIT.with(|audit| *audit.borrow_mut() = Some(Vec::new()));
}

#[cfg(test)]
pub(crate) fn take_terminal_provider_support_audit() -> Vec<TimeSupport> {
    TERMINAL_PROVIDER_SUPPORT_AUDIT.with(|audit| audit.borrow_mut().take().unwrap_or_default())
}

#[cfg(test)]
fn audit_terminal_provider_support(support: TimeSupport) {
    TERMINAL_PROVIDER_SUPPORT_AUDIT.with(|audit| {
        if let Some(entries) = audit.borrow_mut().as_mut() {
            entries.push(support);
        }
    });
}

#[cfg(not(test))]
#[inline(always)]
fn audit_terminal_provider_support(_: TimeSupport) {}

#[derive(Clone)]
struct ExactCoveredTerminalEndpointV1 {
    support: TimeSupport,
    lane_id: u32,
    event: DirectSnowTerminalEventResult,
    event_result_digest: Digest32,
    forcing_sha256: Digest32,
    ending: crate::v9_real_consumer_shadow::CoveredCarrierEphemeralCandidatesV1,
    carrier_phase: crate::v9_real_consumer_shadow::CoveredCarrierPhaseResultV1,
    terminal_snow_soil_trial_receipt:
        crate::v9_real_consumer_shadow::TerminalSnowSoilTrialReceiptV1,
    endpoint_receipt_sha256: Digest32,
}

fn precomputed_terminal_package_v1(
    endpoints: &[ExactCoveredTerminalEndpointV1],
    pending: &BTreeMap<Digest32, DirectSnowStage3V11TerminalParcel>,
    accepted_slab_sha256: Digest32,
) -> Result<crate::v9_real_consumer_shadow::PrecomputedTerminalAcceptedEndpointV1, DirectSnowStage3V11AttachmentError> {
    let first = endpoints.first().ok_or(DirectSnowStage3V11AttachmentError::Terminal(
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
    let mut trials = BTreeMap::new();
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
            || events.insert(endpoint.lane_id, endpoint.event).is_some()
            || trials
                .insert(endpoint.lane_id, endpoint.terminal_snow_soil_trial_receipt.clone())
                .is_some()
        {
            return Err(DirectSnowStage3V11AttachmentError::Terminal(
                "coalesced exact terminal carrier divergence",
            ));
        }
        let dormant = endpoint.ending.stage3_by_lane().get(&endpoint.lane_id).ok_or(
            DirectSnowStage3V11AttachmentError::Identity("coalesced dormant lane"),
        )?;
        ending_stage3.insert(endpoint.lane_id, dormant.clone());
    }
    Ok(crate::v9_real_consumer_shadow::PrecomputedTerminalAcceptedEndpointV1 {
        carrier_phase: first.carrier_phase.clone(),
        ending_stage3_by_lane: ending_stage3,
        terminal_events: events,
        terminal_snow_soil_trial_receipts: trials,
        beginning_pending_terminal_parcels: pending.clone(),
        accepted_slab_sha256,
        wb14_child_receipt_set_sha256: parse_lower_hex_digest(
            &first.carrier_phase.wb14_child_receipt_set_sha256,
        )?,
        wb14_parent_receipt_set_sha256: first
            .carrier_phase
            .wb14_parent_receipt_set_sha256
            .as_deref()
            .map(parse_lower_hex_digest)
            .transpose()?,
    })
}

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
) -> Result<ExactCoveredTerminalEndpointV1, DirectSnowStage3V11AttachmentError> {
    let ending = bind_exact_terminal_endpoint_candidate_v1(
        discovery.lane_id,
        exact_result,
        candidates_by_joint,
    )?;
    let event = exact_result.terminal_event.ok_or(
        DirectSnowStage3V11AttachmentError::Terminal(
            "exact endpoint event-result custody",
        ),
    )?;
    let carrier_phase = if let Some(exact) = carrier_phases_by_joint
        .get(&ending.joint().receipt_sha256())
        .cloned()
    {
        exact
    } else {
        let matching = carrier_phases_by_joint
            .values()
            .filter(|phase| {
                ending.joint().owner_bytes().iter().all(|(owner_id, bytes)| {
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
    let event_result_digest = canonical_terminal_event_result_digest(&event)?;
    let ending_state = ending.stage3_by_lane().get(&discovery.lane_id).ok_or(
        DirectSnowStage3V11AttachmentError::Identity(
            "exact endpoint ending snow lane",
        ),
    )?;
    let ending_state_sha256 = digest_bytes(
        &Wb11HydrologyKernel::serialize_stage3_persistent_state(ending_state)?,
    );
    if event_result_digest != discovery.event_result_digest
        || event != discovery.event
        || exact_forcing_sha256 != discovery.shortened_forcing_sha256
        || ending_state_sha256 != discovery.terminal_state_sha256
        || event.hour_offset_seconds.to_bits()
            != f64::from_bits(discovery.support.duration_s_bits()).to_bits()
        || event.evaluated_seconds.to_bits()
            != f64::from_bits(discovery.support.duration_s_bits()).to_bits()
        || event.unevaluated_seconds.to_bits() != 0.0_f64.to_bits()
        || event.terminal_unallocated_energy_j_m2 < 0.0
        || event.terminal_unallocated_energy_j_m2 > 1.0e-6
    {
        return Err(DirectSnowStage3V11AttachmentError::Terminal(
            "probe/exact terminal endpoint cross-join",
        ));
    }
    let trial_receipt = ending
        .terminal_snow_soil_trial_receipt()
        .cloned()
        .ok_or(DirectSnowStage3V11AttachmentError::Terminal(
            "exact endpoint terminal snow-soil trial receipt",
        ))?;
    trial_receipt.validate().map_err(|_| {
        DirectSnowStage3V11AttachmentError::Terminal(
            "exact endpoint terminal snow-soil trial receipt seal",
        )
    })?;
    if trial_receipt.support != discovery.support
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
                tag: "event",
                value: event_result_digest.as_bytes(),
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
        ],
    )?;
    Ok(ExactCoveredTerminalEndpointV1 {
        support: discovery.support,
        lane_id: discovery.lane_id,
        event,
        event_result_digest,
        forcing_sha256: exact_forcing_sha256,
        ending,
        carrier_phase,
        terminal_snow_soil_trial_receipt: trial_receipt,
        endpoint_receipt_sha256,
    })
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
    let event = result.terminal_event.as_ref().ok_or(
        DirectSnowStage3V11AttachmentError::Terminal(
            "exact covered endpoint missing terminal event",
        ),
    )?;
    if !event.event_occurred
        || event.unevaluated_seconds.abs() > 1.0e-6
        || !result.state.layers.is_empty()
        || result.state.lane_id != lane_id
    {
        return Err(DirectSnowStage3V11AttachmentError::Terminal(
            "exact covered endpoint physical disposition",
        ));
    }
    let selected_joint = result.covered_terminal_ending_joint.as_ref().ok_or(
        DirectSnowStage3V11AttachmentError::Terminal(
            "exact covered endpoint selected joint",
        ),
    )?;
    let carrier = if let Some(exact) = candidates_by_joint
        .get(&selected_joint.receipt_sha256())
        .cloned()
    {
        exact
    } else {
        let matching = candidates_by_joint
            .values()
            .filter(|candidate| {
                selected_joint.owner_bytes().iter().all(|(owner_id, bytes)| {
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
    crate::v9_real_consumer_shadow::CoveredCarrierEphemeralCandidatesV1::try_new(
        selected_joint.clone(),
        carrier.shadow().clone(),
        stage3,
    )
    .map_err(DirectSnowStage3V11AttachmentError::Owner)
}

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
    let mut evidence = <crate::hydrology::NoEvidence as crate::hydrology::TerminalEvidenceMode<Option<CoveredTerminalJointTrialStateV1>>>::new_state();
    evaluate_covered_terminal_candidate_with_evidence_v1::<crate::hydrology::NoEvidence>(
        beginning_consumer, beginning_clock, prepared, day_index, interval_index,
        beginning_stage3, beginning_terminal_parcels, selected_upper_bound_s,
        current_child_ordinal, lane_id, mode, &mut evidence,
    )
}

#[allow(clippy::too_many_arguments, clippy::too_many_lines)]
fn evaluate_covered_terminal_candidate_with_evidence_v1<M: crate::hydrology::TerminalEvidenceMode<Option<CoveredTerminalJointTrialStateV1>>>(
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
    let state = beginning_stage3.get(&lane_id).ok_or(
        DirectSnowStage3V11AttachmentError::Identity("terminal beginning lane"),
    )?;
    let inputs = prepared.snow_inputs_by_lane.get(&lane_id).ok_or(
        DirectSnowStage3V11AttachmentError::Identity("terminal input lane"),
    )?;
    let forcing = prepared.support_forcing_by_lane.get(&lane_id).copied().ok_or(
        DirectSnowStage3V11AttachmentError::Identity("terminal forcing lane"),
    )?;
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
            interval_index: u64::try_from(interval_index).map_err(|_| DirectSnowStage3V11AttachmentError::Identity("terminal interval width"))?,
            state_support: prepared.support,
            accepted_predecessors: Vec::new(),
        }, initial_owner_bytes)?;
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
        let beginning = if let Some(exact) = candidates_by_joint
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
                    request.beginning_joint.owner_bytes().iter().all(|(owner_id, bytes)| {
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
            let carrier = &matching[0];
            crate::v9_real_consumer_shadow::CoveredCarrierEphemeralCandidatesV1::try_new(
                request.beginning_joint.clone(),
                carrier.shadow().clone(),
                carrier.stage3_by_lane().clone(),
            )
            .map_err(|_| {
                DirectSnowStage3EvaluationError::TerminalCustody(
                    "covered probe post-hydrology typed joint",
                )
            })?
        };
        let mut projected = prepared
            .coupled_subslab(request.support, current_child_ordinal)
            .map_err(|_| {
                DirectSnowStage3EvaluationError::TerminalCustody(
                    "covered probe exact support projection",
                )
            })?;
        if let Some(interval) = projected.covered_v11_interval.as_mut() {
            interval.lse_forcing.transaction_id = beginning
                .shadow()
                .next_lse_transaction_id()
                .map_err(|_| {
                    DirectSnowStage3EvaluationError::TerminalCustody(
                        "covered probe typed transaction projection",
                    )
                })?;
            interval.lse_forcing.forcing_sha256 = interval
                .lse_forcing
                .canonical_sha256()
                .map_err(|_| {
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
                snow_surface_forcing_by_destination:
                    &projected.snow_surface_forcing_by_destination,
                stage3_beginning_by_lane: beginning.stage3_by_lane().clone(),
                pending_terminal_parcels: beginning_terminal_parcels.clone(),
                day_index,
                interval_index,
                finalize_wb14_parent_interval: false,
                wb14_coupled_child_binding:
                    crate::direct_runtime::DirectWb14CoupledChildBindingV1 {
                        proposed_upper_bound_s_bits: selected_upper_bound_s.to_bits(),
                        coupled_parent_transaction_sha256: *parent_id.digest().as_bytes(),
                        accepted_slab_sha256: *child.receipt_sha256.as_bytes(),
                        parent_beginning_complete_owner_set_sha256:
                            *parent_owner_digest.as_bytes(),
                        parent_support_start_ns: beginning_clock
                            .parent_support()
                            .start_ns()
                            .get(),
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
        let result = provider_result
            .map_err(|error| {
                if carrier_failure.borrow().is_none() {
                    *carrier_failure.borrow_mut() = Some(error);
                }
                DirectSnowStage3EvaluationError::TerminalCustody(
                    "covered probe carrier fixed point",
                )
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
    let result = Wb11HydrologyKernel::evaluate_stage3_terminal_support_with_trial_provider_and_evidence_v1::<M>(
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
        Ok(result) => Ok((
            result,
            candidates_by_joint.into_inner(),
            carrier_phases_by_joint.into_inner(),
        )),
        Err(error) => {
            if let Some(carrier_error) = carrier_failure.into_inner() {
                Err(DirectSnowStage3V11AttachmentError::Owner(carrier_error))
            } else {
                Err(DirectSnowStage3V11AttachmentError::Stage3(error))
            }
        }
    }
}

#[allow(clippy::too_many_arguments, clippy::too_many_lines)]
fn try_actual_terminal_subslab(
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
    let mut evidence = <crate::hydrology::NoEvidence as crate::hydrology::TerminalEvidenceMode<Option<CoveredTerminalJointTrialStateV1>>>::new_state();
    try_actual_terminal_subslab_with_evidence::<crate::hydrology::NoEvidence>(
        context, beginning_parent, beginning_consumer, beginning_clock, prepared,
        day_index, interval_index, forcing_receipt, beginning_stage3,
        beginning_terminal_parcels, selected_upper_bound_s, current_child_ordinal,
        event_ordinal, &mut evidence,
    )
}

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
    for lane_id in &active_lanes {
        let (result, _, _) = evaluate_covered_terminal_candidate_with_evidence_v1::<M>(
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
        let event_relative = quantize_seconds_to_tick(
            ModelTimeNs::new(0),
            ModelTimeNs::new(prepared.support.duration_ns()),
            event.hour_offset_seconds,
        )?;
        let event_tick = ModelTimeNs::new(
            prepared.support.start_ns().get() + event_relative.get(),
        );
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
        for seconds in [
            event.hour_offset_seconds,
            event.event_bracket_lower_seconds,
            event.event_bracket_upper_seconds,
        ] {
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
                canonical_stage3_support_forcing_digest(
                    &projected.support_forcing_by_lane,
                ),
                &exact_candidates,
                &exact_carrier_phases,
            )?);
        }
        let exact = exact_endpoints.first().ok_or(
            DirectSnowStage3V11AttachmentError::Terminal("missing exact endpoint value"),
        )?;
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
        let consumer = exact.ending.shadow().clone();
        let stage3 = expected_installed.ending_stage3_by_lane;
        let candidates = exact_endpoints
            .iter()
            .map(|endpoint| {
                let state = endpoint.ending.stage3_by_lane().get(&endpoint.lane_id).ok_or(
                    DirectSnowStage3V11AttachmentError::Identity(
                        "exact endpoint installed snow lane",
                    ),
                )?;
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
        let Some(mut group) = select_common_earliest_actual_terminal_group_v1(
            beginning_clock.parent_support(),
            event_ordinal,
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
        group.accepted_event_receipt = Some(accepted_event_receipt.clone());
        group.accepted_group_receipt_sha256 = Some(accepted_terminal_group_digest(&group)?);
        return Ok(Some(ActualTerminalSubslabV1 {
            parent,
            consumer,
            clock,
            stage3,
            receipt,
            group,
            parcels,
        }));
    }
    Ok(None)
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
    let mut proposal_core = None;
    for candidate in &group.candidates {
        let terminal =
            stage3
                .get(&candidate.lane_id)
                .ok_or(DirectSnowStage3V11AttachmentError::Identity(
                    "terminal event lane owner",
                ))?;
        let (candidate_parcels, candidate_core) = terminal_parcels_for_event_group(
            &context.surface_liquid_configuration,
            candidate,
            group,
            clock.parent_transaction_id().digest(),
            clock.parent_support(),
            physical_child_ordinal,
        )?;
        if proposal_core
            .replace(candidate_core)
            .is_some_and(|value| value != candidate_core)
        {
            return Err(DirectSnowStage3V11AttachmentError::Identity(
                "terminal proposal-core divergence",
            ));
        }
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
    let proposal_core = proposal_core.ok_or(DirectSnowStage3V11AttachmentError::Identity(
        "terminal proposal-core missing",
    ))?;
    group.proposal_core_sha256 = Some(proposal_core);
    let beginning_snow = clock
        .owners()
        .iter()
        .find(|owner| owner.owner_id() == "snow")
        .ok_or(DirectSnowStage3V11AttachmentError::Identity(
            "terminal beginning snow owner",
        ))?;
    let ending_snow = ending_owners
        .iter()
        .find(|owner| owner.owner_id() == "snow")
        .ok_or(DirectSnowStage3V11AttachmentError::Identity(
            "terminal ending snow owner",
        ))?;
    let mut parcel_digests = parcels
        .iter()
        .map(|parcel| parcel.parcel_digest)
        .collect::<Vec<_>>();
    parcel_digests.sort_unstable();
    let parcel_fields = parcel_digests
        .iter()
        .map(|digest| FramedField { tag: "parcel", value: digest.as_bytes() })
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
        beginning_snow.state_digest(),
        ending_snow.state_digest(),
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

fn terminal_parcels_for_event_group(
    configuration: &DirectSurfaceLiquidConfiguration,
    candidate: &Stage3V11ActualTerminalCandidateV1,
    group: &Stage3V11TerminalEventGroupV1,
    parent_transaction_id: Digest32,
    enclosing_support: TimeSupport,
    physical_child_ordinal: u32,
) -> Result<(Vec<DirectSnowStage3V11TerminalParcel>, Digest32), DirectSnowStage3V11AttachmentError>
{
    let destination_ofe = configuration
        .ofe_bindings
        .iter()
        .find(|binding| binding.production_lane_id == candidate.lane_id)
        .map(|binding| binding.ofe_id.clone())
        .ok_or(DirectSnowStage3V11AttachmentError::Identity(
            "terminal receiver lane binding",
        ))?;
    let records = configuration
        .records
        .iter()
        .filter(|record| record.key.ofe_id == destination_ofe)
        .collect::<Vec<_>>();
    let fraction_sum = records
        .iter()
        .map(|record| record.tile_fraction)
        .sum::<f64>();
    if records.is_empty() || (fraction_sum - 1.0).abs() > 1.0e-12 {
        return Err(DirectSnowStage3V11AttachmentError::Identity(
            "terminal receiver topology closure",
        ));
    }
    let mut topology_bytes = Vec::new();
    for record in &records {
        topology_bytes.extend_from_slice(record.key.ofe_id.as_str().as_bytes());
        topology_bytes.push(0);
        topology_bytes.extend_from_slice(record.key.tile_id.as_str().as_bytes());
        topology_bytes.extend_from_slice(&record.tile_fraction.to_bits().to_be_bytes());
    }
    let topology = digest_bytes(&topology_bytes);
    let schema = 1_u32.to_be_bytes();
    let child = physical_child_ordinal.to_be_bytes();
    let ordinal = u32::try_from(group.ordinal)
        .map_err(|_| DirectSnowStage3V11AttachmentError::Identity("terminal event ordinal width"))?
        .to_be_bytes();
    let mut candidates = u32::try_from(group.candidates.len())
        .map_err(|_| DirectSnowStage3V11AttachmentError::Identity("terminal candidate count"))?
        .to_be_bytes()
        .to_vec();
    for value in &group.candidates {
        let mut member = Vec::new();
        member.extend_from_slice(&value.lane_id.to_be_bytes());
        member.extend_from_slice(value.event_result_digest.as_bytes());
        member.extend_from_slice(value.terminal_state_sha256.as_bytes());
        member.extend_from_slice(&value.event.terminal_liquid_kg_m2.to_bits().to_be_bytes());
        member.extend_from_slice(
            &value
                .event
                .terminal_unallocated_energy_j_m2
                .to_bits()
                .to_be_bytes(),
        );
        candidates.extend_from_slice(&(member.len() as u32).to_be_bytes());
        candidates.extend_from_slice(&member);
    }
    let search = group
        .candidates
        .first()
        .ok_or(DirectSnowStage3V11AttachmentError::Identity(
            "terminal proposal-core candidates",
        ))?
        .support;
    let proposal_core = framed_sha256(
        "stage3-v11-terminal-event-proposal-core",
        &[
            FramedField {
                tag: "schema",
                value: &schema,
            },
            FramedField {
                tag: "parent_transaction",
                value: parent_transaction_id.as_bytes(),
            },
            FramedField {
                tag: "enclosing_start",
                value: &enclosing_support.start_ns().get().to_be_bytes(),
            },
            FramedField {
                tag: "enclosing_end",
                value: &enclosing_support.end_ns().get().to_be_bytes(),
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
                value: topology.as_bytes(),
            },
            FramedField {
                tag: "candidates",
                value: &candidates,
            },
        ],
    )?;
    let parcels = records
        .into_iter()
        .map(|record| {
            let mass = candidate.event.terminal_liquid_kg_m2;
            let mut parcel = DirectSnowStage3V11TerminalParcel {
                support: candidate.support,
                source_lane_id: candidate.lane_id,
                parent_transaction_id,
                event_ordinal: u32::try_from(group.ordinal).map_err(|_| {
                    DirectSnowStage3V11AttachmentError::Identity("terminal event ordinal width")
                })?,
                terminal_event_proposal_core_id: proposal_core,
                event_result_digest: candidate.event_result_digest,
                receiver_topology_sha256: topology,
                destination_ofe_id: record.key.ofe_id.to_string(),
                destination_tile_id: record.key.tile_id.as_str().to_owned(),
                destination_fraction: record.tile_fraction,
                mass_kg_m2_tile_ground: mass,
                temperature_k: 273.15,
                specific_liquid_enthalpy_j_kg: 0.0,
                posture: DirectSnowStage3V11TerminalParcelPosture::ProducedUnconsumed,
                parcel_digest: Digest32::zero(),
            };
            parcel.parcel_digest = crate::snow_owner_v4::canonical_terminal_parcel_digest(&parcel)
                .map_err(|_| {
                    DirectSnowStage3V11AttachmentError::Identity("terminal parcel digest")
                })?;
            Ok(parcel)
        })
        .collect::<Result<Vec<_>, DirectSnowStage3V11AttachmentError>>()?;
    Ok((parcels, proposal_core))
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
    terminal_endpoints: Option<&[ExactCoveredTerminalEndpointV1]>,
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
        provisional_stack = provisional_stack.with_precomputed_terminal_accepted_endpoint(
            precomputed_terminal_package_v1(
                endpoints,
                pending_terminal_parcels,
                provisional_receipt.slab_id().digest(),
            )?,
        );
    }
    let mut provisional_executor = crate::v11_vegetation_consumer::DirectV11VegetationExecutor {
        stack: provisional_stack,
    };
    let provisional_segment = execute_direct_v11_segment(
        &context.vegetation_configuration,
        beginning_parent,
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
    let mut final_stack = DirectV11SnowCoveredRealConsumerStack::new(
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
            wb14_coupled_child_binding: crate::direct_runtime::DirectWb14CoupledChildBindingV1 {
                proposed_upper_bound_s_bits: selected_upper_bound_s.to_bits(),
                coupled_parent_transaction_sha256: *parent_id.digest().as_bytes(),
                accepted_slab_sha256: *final_receipt.slab_id().digest().as_bytes(),
                parent_beginning_complete_owner_set_sha256: *ledger_digest.as_bytes(),
                parent_support_start_ns: beginning_clock.parent_support().start_ns().get(),
                parent_support_end_ns: beginning_clock.parent_support().end_ns().get(),
                child_support_start_ns: support.start_ns().get() as u128,
                child_support_end_ns: support.end_ns().get() as u128,
            },
        },
    );
    if let Some(endpoints) = terminal_endpoints {
        final_stack = final_stack.with_precomputed_terminal_accepted_endpoint(
            precomputed_terminal_package_v1(
                endpoints,
                pending_terminal_parcels,
                final_receipt.slab_id().digest(),
            )?,
        );
    }
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
            "covered V11 ending owner fixed point",
        ));
    }
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
        .filter(|(lane_id, _)| !terminal_events.contains_key(lane_id))
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
        let installed_snow_bottom = Wb11HydrologyKernel::project_stage3_bottom_volume_v1(
            state,
            inputs.surface_energy_options.atmospheric_pressure_pa,
        )?;
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
        if crate::hydrology::stage3_has_represented_ice(state)
            || !state.layers.is_empty()
            || state.detached_retained_liquid_kg_m2.to_bits() != 0.0_f64.to_bits()
            || receipt.ending_dormant_snow_owner_sha256 != installed_snow_identity
            || receipt.ending_soil_owner_sha256 != installed_soil_identity
            || !terminal_events.contains_key(lane_id)
        {
            return Err(DirectSnowStage3V11AttachmentError::Identity(
                "terminal snow-soil dormant installed join",
            ));
        }
    }
    let physical_custody_join = CoveredPhysicalCustodyJoinInputs {
        snow_soil_heat_receipts: &snow_soil_heat_receipts,
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
    let consumer = final_executor.stack.take_staged_ending().ok_or(
        DirectSnowStage3V11AttachmentError::Identity("missing staged covered ending"),
    )?;
    let (wb14_child_replay_bytes, wb14_parent_replay_bytes) =
        final_executor.stack.last_wb14_replay_bytes().ok_or(
            DirectSnowStage3V11AttachmentError::Identity("missing WB14 replay receipt payload"),
        )?;
    let wb14_child_replay_bytes = wb14_child_replay_bytes.to_vec();
    let wb14_parent_replay_bytes = wb14_parent_replay_bytes.map(ToOwned::to_owned);
    let parent_after_segment = parent;
    let mut subslab_receipt = Stage3CoupledSubslabReceiptV1 {
        parent_support: beginning_clock.parent_support(),
        support,
        selected_upper_bound_s_bits: selected_upper_bound_s.to_bits(),
        accepted_slab_sha256: final_receipt.slab_id().digest(),
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
        owner_join,
        receipt_sha256: Digest32::zero(),
    };
    subslab_receipt.receipt_sha256 = subslab_receipt.reconstructed_digest()?;
    subslab_receipt.validate()?;
    Ok((
        parent_after_segment,
        consumer,
        final_clock,
        ending_stage3,
        subslab_receipt,
    ))
}

#[cfg(test)]
mod terminal_exact_installation_source_guards {
    #[test]
    fn exact_terminal_installation_uses_only_precomputed_executor() {
        let source = include_str!("snow_stage3_v11_terminal_execution.rs");
        assert!(source.contains("with_precomputed_terminal_accepted_endpoint"));
        assert!(!source.contains(&["with_terminal", "_endpoint_mode"].concat()));
        assert!(!source.contains(&[
            "evaluate_stage3_",
            "persistent_support(",
        ]
        .concat()));
        assert!(!source.contains(&["let Ok((", "parent"].concat()));
    }

    #[test]
    fn obsolete_terminal_consumer_and_duplicate_ordinal_authority_are_absent() {
        let attachment = include_str!("snow_stage3_v11_attachment.rs");
        let receipts = include_str!("snow_stage3_v11_attachment_receipts.rs");
        let persistent = include_str!(
            "hydrology/support_helpers_mod/runoff_reconciliation/stage3_solver/persistent_state.rs"
        );
        assert!(!attachment.contains("accepted_event_ordinal"));
        assert!(!receipts.contains("ending_event_ordinal"));
        assert!(!persistent.contains("consume_stage3_terminal_liquid_v1"));
    }

    #[test]
    fn terminal_zero_event_and_subminimum_paths_are_fail_closed() {
        let source = include_str!("snow_stage3_v11_terminal_execution.rs");
        assert!(source.contains("terminal zero-event V4 owner installation"));
        assert!(source.contains("terminal event creates positive subminimum support"));
    }

    #[test]
    fn touched_real_consumer_host_remains_below_hard_source_ceiling() {
        let source = include_str!("v9_real_consumer_shadow.rs");
        assert!(source.lines().count() < 3_000);
        assert!(source.contains("include!(\"v9_real_consumer_shadow_v10_accessors.rs\")"));
    }
}

include!("snow_stage3_v11_attachment_helpers.rs");
