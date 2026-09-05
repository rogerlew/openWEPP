struct ActualTerminalSubslabV1 {
    parent: V11ParentTransaction,
    consumer: DirectV10RealConsumerShadow,
    clock: CoupledClockStateV1,
    stage3: BTreeMap<u32, DirectSnowStage3PersistentState>,
    receipts: Vec<Stage3CoupledSubslabReceiptV1>,
    group: Option<Stage3V11TerminalEventGroupV1>,
    parcels: Vec<DirectSnowStage3V11TerminalParcel>,
    /// Exact native-V2 soil value retained only until the first snow-free
    /// successor.  This is deliberately outside every checkpoint/restart
    /// shape and is never an owner installation at this boundary.
    deferred_native_v2_soil_custody:
        Option<crate::v9_real_consumer_shadow::DeferredNativeV2SoilCustodyV1>,
}

fn deferred_native_v2_soil_custody_v1(
    authoritative: &DirectV10RealConsumerShadow,
    endpoint: &ExactCoveredTerminalEndpointV1,
) -> Result<
    Option<crate::v9_real_consumer_shadow::DeferredNativeV2SoilCustodyV1>,
    DirectSnowStage3V11AttachmentError,
> {
    if !terminal_provisional_publication_deferral_enabled() {
        return Ok(None);
    }
    if matches!(
        endpoint.carrier_phase.soil_candidate,
        crate::v9_real_consumer_shadow::DirectSoilThermalCandidate::V1(_)
    ) {
        return Ok(None);
    }
    let continuation = endpoint
        .carrier_phase
        .ending_candidates
        .soil_continuation()
        .cloned();
    crate::v9_real_consumer_shadow::DeferredNativeV2SoilCustodyV1::try_new(
        authoritative,
        endpoint.carrier_phase.soil_candidate.clone(),
        continuation,
    )
    .map(Some)
    .map_err(DirectSnowStage3V11AttachmentError::Owner)
}

enum CoveredTerminalProviderRetentionV1 {
    Initial(std::rc::Rc<crate::v9_real_consumer_shadow::CoveredCarrierEphemeralCandidatesV1>),
    Phase(std::rc::Rc<crate::v9_real_consumer_shadow::CoveredCarrierPhaseResultV1>),
}

/// Prove that the exact terminal trial and the retained installer image differ
/// only by the canonical parent-staged lineage projection. A reused V1
/// physical image may still carry the enclosing parent revision, while a
/// freshly assembled or already-normalized image carries the next accepted
/// support revision. These are the two exact representations admitted by the
/// covered owner finalizer. Apply the same typed normalization and then require
/// complete canonical-owner byte equality; no physical field, history-bearing
/// owner field, or unrelated owner may disappear behind this join.
fn validate_exact_terminal_installer_owner_relation_v1(
    beginning: &DirectV10RealConsumerShadow,
    exact: &DirectV10RealConsumerShadow,
    installed: &DirectV10RealConsumerShadow,
) -> Result<(), DirectSnowStage3V11AttachmentError> {
    fn vegetation_transaction_v1(
        consumer: &DirectV10RealConsumerShadow,
    ) -> Result<u128, DirectSnowStage3V11AttachmentError> {
        let owners = consumer.canonical_owner_state_bytes()?;
        let bytes =
            owners
                .get("vegetation")
                .ok_or(DirectSnowStage3V11AttachmentError::Terminal(
                    "exact terminal installer vegetation owner",
                ))?;
        let state: openwepp_vegetation::V10CoupledOwnedState = serde_json::from_slice(bytes)
            .map_err(|_| {
                DirectSnowStage3V11AttachmentError::Terminal(
                    "exact terminal installer vegetation decoding",
                )
            })?;
        Ok(state.0.last_transaction_id)
    }

    let parent_transaction = vegetation_transaction_v1(beginning)?;
    let exact_transaction = vegetation_transaction_v1(exact)?;
    let accepted_transaction =
        parent_transaction
            .checked_add(1)
            .ok_or(DirectSnowStage3V11AttachmentError::Terminal(
                "exact terminal installer lineage overflow",
            ))?;
    if exact_transaction != parent_transaction && exact_transaction != accepted_transaction {
        return Err(DirectSnowStage3V11AttachmentError::Terminal(
            "exact terminal installer accepted lineage",
        ));
    }

    let mut normalized_exact = exact.clone();
    crate::v9_real_consumer_shadow::normalize_v11_staged_parent_lineage(
        &mut normalized_exact,
        parent_transaction,
    )?;
    if normalized_exact.canonical_owner_state_bytes()? != installed.canonical_owner_state_bytes()? {
        return Err(DirectSnowStage3V11AttachmentError::Terminal(
            "exact terminal installer canonical owner relation",
        ));
    }
    Ok(())
}

impl CoveredTerminalProviderRetentionV1 {
    fn candidates(&self) -> &crate::v9_real_consumer_shadow::CoveredCarrierEphemeralCandidatesV1 {
        match self {
            Self::Initial(candidates) => candidates,
            Self::Phase(phase) => &phase.ending_candidates,
        }
    }

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

include!("snow_stage3_v11_terminal_execution_carrier_phase.rs");

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
    let terminal_trial_support =
        accepted_terminal_microstep.map_or(discovery.support, |step| step.support);
    let carrier_phase = select_exact_terminal_carrier_phase_v1(
        accepted_terminal_microstep,
        &ending,
        carrier_phases_by_joint,
    )?;
    let trial_chains_by_lane = prepare_exact_terminal_trial_chains_v1(
        discovery,
        exact_result,
        &carrier_phase,
        carrier_phases_by_joint,
    )?;
    let (carrier_phase, carrier_phase_chain, wb14_replay_trial_sha256) =
        prepare_exact_terminal_phase_chain_v1(
            exact_result,
            carrier_phase,
            carrier_phases_by_joint,
        )?;
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
    if trial_receipt.support != terminal_trial_support || trial_receipt.lane_id != discovery.lane_id
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
    let native_binding = carrier_phase
        .ending_candidates
        .shadow()
        .frozen_litter_v3_resident()
        .map(|resident| {
            crate::direct_runtime::stage3_covered_native_inactive_child_custody_binding(
                &carrier_phase.wb14_child_replay_bytes,
                &resident.surface_configuration().parent().ofe_topology,
            )
            .map_err(|_| {
                DirectSnowStage3V11AttachmentError::Terminal(
                    "exact endpoint native inactive child-custody binding",
                )
            })
        })
        .transpose()?
        .flatten();
    let wb14_binding =
        match native_binding {
            Some(binding) => binding,
            None => crate::direct_runtime::wb14_child_replay_binding(
                &carrier_phase.wb14_child_replay_bytes,
            )
            .map_err(|_| {
                DirectSnowStage3V11AttachmentError::Terminal("exact endpoint WB14 replay binding")
            })?,
        };
    let wb14_replay_beginning_owner_set_sha256 =
        Digest32::from_bytes(wb14_binding.parent_beginning_complete_owner_set_sha256);
    Ok(Box::new(ExactCoveredTerminalEndpointV1 {
        support: discovery.support,
        lane_id: discovery.lane_id,
        event,
        event_result_digest,
        forcing_sha256: exact_forcing_sha256,
        ending,
        carrier_phase,
        carrier_phase_chain,
        wb14_replay_trial_sha256,
        wb14_replay_beginning_owner_set_sha256,
        terminal_snow_soil_trial_receipt: trial_receipt,
        final_child_actual_vapor_to_canopy_air_kg_m2: accepted_terminal_microstep
            .map_or(event.sublimation_kg_m2 - event.deposition_kg_m2, |step| {
                step.sublimation_kg_m2 - step.deposition_kg_m2
            }),
        terminal_snow_soil_trial_receipt_chains_by_lane: trial_chains_by_lane,
        endpoint_receipt_sha256,
    }))
}

include!("snow_stage3_v11_terminal_execution_binding.rs");

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
        None,
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

struct TerminalCandidateProfileScopeV1 {
    phase: &'static str,
    started: Option<std::time::Instant>,
}

impl TerminalCandidateProfileScopeV1 {
    fn begin(phase: &'static str) -> Self {
        Self {
            phase,
            started:
                crate::snow_stage3_v11_attachment::begin_adaptive_parent_fixed_point_phase_v1(),
        }
    }
}

impl Drop for TerminalCandidateProfileScopeV1 {
    fn drop(&mut self) {
        crate::snow_stage3_v11_attachment::record_adaptive_parent_profile_detail_v1(
            self.phase,
            self.started.take(),
        );
    }
}

#[allow(clippy::too_many_arguments, clippy::too_many_lines)]
fn evaluate_covered_terminal_candidate_with_evidence_v1<
    M: crate::hydrology::TerminalEvidenceMode<Option<CoveredTerminalJointTrialStateV1>>,
>(
    beginning_consumer: &DirectV10RealConsumerShadow,
    deferred_native_v2_soil_custody: Option<
        &crate::v9_real_consumer_shadow::DeferredNativeV2SoilCustodyV1,
    >,
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
    let setup_profile = TerminalCandidateProfileScopeV1::begin("terminal candidate setup");
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
    let mut initial_owner_bytes = crate::v9_real_consumer_shadow::
        covered_carrier_initial_owner_bytes_with_deferred_native_v2_soil_custody_v1(
            beginning_consumer,
            deferred_native_v2_soil_custody,
        )?;
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
    let initial_candidates = match deferred_native_v2_soil_custody {
        Some(custody) => crate::v9_real_consumer_shadow::
            CoveredCarrierEphemeralCandidatesV1::try_new_with_deferred_native_v2_soil_custody(
                initial_joint.clone(),
                beginning_consumer.clone(),
                beginning_stage3.clone(),
                custody,
            )?,
        None => crate::v9_real_consumer_shadow::CoveredCarrierEphemeralCandidatesV1::try_new(
            initial_joint.clone(),
            beginning_consumer.clone(),
            beginning_stage3.clone(),
        )?,
    };
    let candidates_by_joint = std::cell::RefCell::new(BTreeMap::from([(
        initial_joint.receipt_sha256(),
        CoveredTerminalProviderRetentionV1::Initial(std::rc::Rc::new(initial_candidates)),
    )]));
    let parent_id = beginning_clock.parent_transaction_id();
    let carrier_failure = std::cell::RefCell::new(None);
    let carrier_phases_by_joint = std::cell::RefCell::new(BTreeMap::new());
    let mut provider_evidence = M::new_provider_state();
    drop(setup_profile);
    let mut provider = |request: crate::hydrology::CoveredTerminalTrialRequestV1| {
        let custody_profile = TerminalCandidateProfileScopeV1::begin("terminal provider custody");
        audit_terminal_provider_support(request.support);
        let carrier = if let Some(exact) = candidates_by_joint
            .borrow()
            .get(&request.beginning_joint.receipt_sha256())
        {
            match exact {
                CoveredTerminalProviderRetentionV1::Initial(candidates) => {
                    CoveredTerminalProviderRetentionV1::Initial(candidates.clone())
                }
                CoveredTerminalProviderRetentionV1::Phase(phase) => {
                    CoveredTerminalProviderRetentionV1::Phase(phase.clone())
                }
            }
        } else {
            let retained_by_joint = candidates_by_joint.borrow();
            let matching = retained_by_joint
                .values()
                .filter(|retained| {
                    retained
                        .candidates()
                        .try_with_selected_stage3_by_lane(
                            request.beginning_joint.clone(),
                            BTreeMap::new(),
                        )
                        .is_ok()
                })
                .collect::<Vec<_>>();
            if matching.len() != 1 {
                return Err(DirectSnowStage3EvaluationError::TerminalCustody(
                    "covered probe typed beginning joint",
                ));
            }
            match matching[0] {
                CoveredTerminalProviderRetentionV1::Initial(candidates) => {
                    CoveredTerminalProviderRetentionV1::Initial(candidates.clone())
                }
                CoveredTerminalProviderRetentionV1::Phase(phase) => {
                    CoveredTerminalProviderRetentionV1::Phase(phase.clone())
                }
            }
        };
        let carrier = carrier.candidates();
        let mut typed_stage3 = carrier.stage3_by_lane().clone();
        typed_stage3.insert(request.lane_id, (*request.beginning_stage3_state).clone());
        let beginning = stage3_boxed_execution_v1(|| {
            carrier
                .try_with_selected_stage3_by_lane(request.beginning_joint.clone(), typed_stage3)
                .map_err(|_| {
                    DirectSnowStage3EvaluationError::TerminalCustody(
                        "covered probe post-hydrology typed joint",
                    )
                })
        })?;
        drop(custody_profile);
        let projection_profile =
            TerminalCandidateProfileScopeV1::begin("terminal provider projection");
        let mut projected = stage3_boxed_execution_v1(|| {
            prepared
                .coupled_subslab(request.support, current_child_ordinal)
                .map_err(|_| {
                    DirectSnowStage3EvaluationError::TerminalCustody(
                        "covered probe exact support projection",
                    )
                })
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
        let child = stage3_boxed_execution_v1(|| {
            CoveredProbeChildIdentityV1::try_new(ProbeChildAuthorityV1 {
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
            })
            .map_err(DirectSnowStage3EvaluationError::from)
        })?;
        let covered_interval = projected.covered_v11_interval.as_ref().ok_or(
            DirectSnowStage3EvaluationError::TerminalCustody(
                "covered probe V11 interval projection",
            ),
        )?;
        let stack = stage3_boxed_execution_v1(|| {
            let stack = DirectV11SnowCoveredRealConsumerStack::new(
                beginning.shadow(),
                DirectV11SnowCoveredStackInputs {
                    interval: covered_interval,
                    stage3_inputs_by_lane: &projected.snow_inputs_by_lane,
                    stage3_forcing_by_lane: &projected.support_forcing_by_lane,
                    snow_surface_forcing_by_destination: &projected
                        .snow_surface_forcing_by_destination,
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
                            parent_beginning_complete_owner_set_sha256: *parent_owner_digest
                                .as_bytes(),
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
            match deferred_native_v2_soil_custody {
                Some(custody) => stack
                    .try_with_deferred_native_v2_soil_custody(custody.clone())
                    .map_err(|_| {
                        DirectSnowStage3EvaluationError::TerminalCustody(
                            "covered probe deferred native V2 soil custody",
                        )
                    }),
                None => Ok::<_, DirectSnowStage3EvaluationError>(stack),
            }
        })?;
        drop(projection_profile);
        let carrier_profile = TerminalCandidateProfileScopeV1::begin("terminal provider carrier");
        let provider_result = stage3_boxed_execution_v1(|| {
            stack.execute_covered_carrier_phase_v1(&beginning, &request, child.as_ref().clone())
        });
        drop(carrier_profile);
        let retention_profile =
            TerminalCandidateProfileScopeV1::begin("terminal provider retention");
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
        if request.support.end_ns() < beginning_clock.parent_support().end_ns() {
            let ending_parent = result
                .ending_candidates
                .shadow()
                .wb14_parent_working_state_v1();
            if beginning.shadow().frozen_litter_v3_resident().is_some() {
                if ending_parent != beginning.shadow().wb14_parent_working_state_v1() {
                    return Err(DirectSnowStage3EvaluationError::TerminalCustody(
                        "covered probe native child changed inactive WB14 parent working state",
                    ));
                }
            } else if ending_parent.is_none() {
                return Err(DirectSnowStage3EvaluationError::TerminalCustody(
                    if request.support.start_ns() == prepared.support.start_ns() {
                        "covered probe initial child lost WB14 parent working state"
                    } else {
                        "covered probe successor child lost WB14 parent working state"
                    },
                ));
            }
        }
        let result: std::rc::Rc<crate::v9_real_consumer_shadow::CoveredCarrierPhaseResultV1> =
            std::rc::Rc::from(result);
        let ending_joint_sha256 = result.ending_candidates.joint().receipt_sha256();
        candidates_by_joint.borrow_mut().insert(
            ending_joint_sha256,
            CoveredTerminalProviderRetentionV1::Phase(result.clone()),
        );
        carrier_phases_by_joint
            .borrow_mut()
            .insert(ending_joint_sha256, result.clone());
        drop(retention_profile);
        Ok(result.transition.clone())
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
            initial_joint.clone(),
            &mut provider,
            evidence,
        );
    drop(provider);
    let result_profile =
        TerminalCandidateProfileScopeV1::begin("terminal result finalization");
    M::merge_provider(evidence, provider_evidence);
    let result = match result {
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
            let candidates_by_joint = candidates_by_joint
                .into_inner()
                .into_iter()
                .map(|(digest, retained)| (digest, retained.candidates().clone()))
                .collect();
            let carrier_phases_by_joint = carrier_phases_by_joint
                .into_inner()
                .into_iter()
                .map(|(digest, phase)| (digest, phase.as_ref().clone()))
                .collect();
            Ok((result, candidates_by_joint, carrier_phases_by_joint))
        }
        Err(error) => {
            if let Some(carrier_error) = carrier_failure.into_inner() {
                Err(DirectSnowStage3V11AttachmentError::Owner(carrier_error))
            } else {
                Err(DirectSnowStage3V11AttachmentError::Stage3(error))
            }
        }
    };
    drop(result_profile);
    result
}

struct CoveredTerminalBatchCandidateV2 {
    phase: Box<crate::v9_real_consumer_shadow::CoveredCarrierPhaseResultV1>,
    result: Box<crate::hydrology::CoveredTerminalBatchTrialResultV2>,
    lane_results: BTreeMap<u32, Box<crate::hydrology::DirectSnowStage3PersistentDayResult>>,
    ending: Box<crate::v9_real_consumer_shadow::CoveredCarrierEphemeralCandidatesV1>,
}

include!("snow_stage3_v11_terminal_execution_batch.rs");

#[cfg(test)]
include!("snow_stage3_discrete_endpoint_evidence.rs");

include!("snow_stage3_v11_terminal_preterminal_disposition.rs");
include!("snow_stage3_v11_terminal_discovery_order.rs");

#[allow(clippy::too_many_arguments, clippy::too_many_lines)]
fn try_actual_terminal_subslab_with_evidence<M>(
    context: &DirectSnowStage3V11StaticContext,
    beginning_parent: &V11ParentTransaction,
    beginning_consumer: &DirectV10RealConsumerShadow,
    deferred_native_v2_soil_custody: Option<
        &crate::v9_real_consumer_shadow::DeferredNativeV2SoilCustodyV1,
    >,
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
    let terminal_lanes = beginning_stage3
        .iter()
        .filter_map(|(lane, state)| {
            crate::hydrology::stage3_is_terminal_event_domain(state).then_some(*lane)
        })
        .collect::<BTreeSet<_>>();
    if active_lanes.len() > 1 && !terminal_lanes.is_empty() {
        let outcome = stage3_boxed_execution_v1(|| {
            try_actual_terminal_batch_subslab_v2(
                context,
                beginning_parent,
                beginning_consumer,
                deferred_native_v2_soil_custody,
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
            )
        })?;
        return Ok(*outcome);
    }
    let outcome = stage3_boxed_execution_v1(|| {
        try_actual_terminal_single_subslab_with_evidence::<M>(
            context,
            beginning_parent,
            beginning_consumer,
            deferred_native_v2_soil_custody,
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
            evidence,
        )
    })?;
    Ok(*outcome)
}

include!("snow_stage3_v11_terminal_execution_preterminal_replay.rs");

/// Validate the two distinct authorities retained by a native-V2 terminal
/// replay. The accepted owner-join receipt authenticates the installed owner
/// set; the unpublished endpoint independently authenticates the physical
/// carrier value that selected that accepted support. Neither authority may
/// be substituted for, or used to rematerialize, the other.
fn validate_native_v2_preterminal_installation_v1(
    next_parent: &V11ParentTransaction,
    next_consumer: &DirectV10RealConsumerShadow,
    endpoint: &ExactCoveredTerminalEndpointV1,
    ending_stage3: &BTreeMap<u32, DirectSnowStage3PersistentState>,
    receipt: &Stage3CoupledSubslabReceiptV1,
    deferred_native_v2_soil_custody: Option<
        &crate::v9_real_consumer_shadow::DeferredNativeV2SoilCustodyV1,
    >,
) -> Result<(), DirectSnowStage3V11AttachmentError> {
    receipt.validate()?;
    receipt
        .owner_join
        .validate_retained_boundary_sets(&receipt.destination_receipts, &receipt.lane_receipts)
        .map_err(|_| {
            DirectSnowStage3V11AttachmentError::Identity(
                "adaptive preterminal V2 retained boundary seals",
            )
        })?;
    if receipt.support != endpoint.support
        || receipt.owner_join.support != endpoint.support
        || receipt.accepted_slab_sha256 != receipt.owner_join.accepted_slab_sha256
        || receipt.physical_outcome_ledger_set_sha256
            != receipt.owner_join.physical_outcome_ledger_set_sha256
        || receipt.wb14_replay_trial_sha256 != endpoint.wb14_replay_trial_sha256
        || receipt.wb14_replay_beginning_owner_set_sha256
            != endpoint.wb14_replay_beginning_owner_set_sha256
        || if endpoint.event.event_occurred {
            receipt.terminal_events.get(&endpoint.lane_id) != Some(&endpoint.event)
        } else {
            receipt.terminal_events.contains_key(&endpoint.lane_id)
        }
        || endpoint.event_result_digest != canonical_terminal_event_result_digest(&endpoint.event)?
    {
        return Err(DirectSnowStage3V11AttachmentError::Identity(
            "adaptive preterminal V2 support/WB14/event/ledger join",
        ));
    }

    let final_phase =
        endpoint
            .carrier_phase_chain
            .last()
            .ok_or(DirectSnowStage3V11AttachmentError::Identity(
                "adaptive preterminal V2 empty physical carrier chain",
            ))?;
    if !endpoint.carrier_phase_chain.first().is_some_and(|phase| {
        phase.transition.boundary.support.start_ns() == endpoint.support.start_ns()
    }) || final_phase.transition.boundary.support.end_ns() != endpoint.support.end_ns()
        || !endpoint.carrier_phase_chain.windows(2).all(|pair| {
            pair[0].transition.boundary.support.end_ns()
                == pair[1].transition.boundary.support.start_ns()
        })
    {
        return Err(DirectSnowStage3V11AttachmentError::Identity(
            "adaptive preterminal V2 physical-chain support coverage",
        ));
    }
    if endpoint
        .carrier_phase
        .transition
        .probe_child_identity
        .receipt_sha256
        != final_phase.transition.probe_child_identity.receipt_sha256
    {
        return Err(DirectSnowStage3V11AttachmentError::Identity(
            "adaptive preterminal V2 probe-child chain join",
        ));
    }
    if endpoint
        .carrier_phase
        .ending_candidates
        .joint()
        .receipt_sha256()
        != final_phase.ending_candidates.joint().receipt_sha256()
        || final_phase.ending_candidates.joint().receipt_sha256()
            != final_phase.transition.ending_joint.receipt_sha256()
    {
        return Err(DirectSnowStage3V11AttachmentError::Identity(
            "adaptive preterminal V2 carrier-joint chain join",
        ));
    }
    if endpoint.ending.joint().receipt_sha256() == Digest32::zero() {
        return Err(DirectSnowStage3V11AttachmentError::Identity(
            "adaptive preterminal V2 selected-joint seal",
        ));
    }
    if endpoint.ending.joint().authority().lane_id != endpoint.lane_id {
        return Err(DirectSnowStage3V11AttachmentError::Identity(
            "adaptive preterminal V2 selected-joint lane",
        ));
    }
    if endpoint.ending.joint().authority().state_support
        != final_phase
            .transition
            .ending_joint
            .authority()
            .state_support
        || endpoint
            .carrier_phase
            .transition
            .probe_child_identity
            .trial_support
            != endpoint.support
    {
        return Err(DirectSnowStage3V11AttachmentError::Identity(
            "adaptive preterminal V2 selected-joint support",
        ));
    }
    if endpoint.terminal_snow_soil_trial_receipt
        != *endpoint
            .carrier_phase
            .batch_terminal_snow_soil_trial_receipts_by_lane
            .get(&endpoint.lane_id)
            .ok_or(DirectSnowStage3V11AttachmentError::Identity(
                "adaptive preterminal V2 terminal soil-trial lane",
            ))?
    {
        return Err(DirectSnowStage3V11AttachmentError::Identity(
            "adaptive preterminal V2 terminal soil-trial value",
        ));
    }
    endpoint
        .terminal_snow_soil_trial_receipt
        .validate()
        .map_err(|_| {
            DirectSnowStage3V11AttachmentError::Identity(
                "adaptive preterminal V2 terminal soil-trial seal",
            )
        })?;
    if endpoint
        .terminal_snow_soil_trial_receipt_chains_by_lane
        .get(&endpoint.lane_id)
        .and_then(|chain| chain.last())
        != Some(&endpoint.terminal_snow_soil_trial_receipt)
    {
        return Err(DirectSnowStage3V11AttachmentError::Identity(
            "adaptive preterminal V2 selected-joint/physical-chain join",
        ));
    }

    let installed_soil = next_consumer.soil_thermal_resident();
    let endpoint_soil = &endpoint.carrier_phase.soil_candidate;
    let installed_soil_view = installed_soil.read_view();
    let endpoint_soil_view = endpoint_soil.read_view();
    if !matches!(
        (installed_soil_view, endpoint_soil_view),
        (
            crate::v9_real_consumer_shadow::DirectSoilThermalReadView::V2(_),
            crate::v9_real_consumer_shadow::DirectSoilThermalReadView::V2(_),
        )
    ) {
        return Err(DirectSnowStage3V11AttachmentError::Identity(
            "adaptive preterminal V2 soil posture",
        ));
    }
    installed_soil_view.validate().map_err(|_| {
        DirectSnowStage3V11AttachmentError::Identity(
            "adaptive preterminal V2 installed soil validation",
        )
    })?;
    endpoint_soil_view.validate().map_err(|_| {
        DirectSnowStage3V11AttachmentError::Identity(
            "adaptive preterminal V2 endpoint soil validation",
        )
    })?;
    let endpoint_continuation = endpoint.carrier_phase.ending_candidates.soil_continuation();
    let deferred_custody =
        deferred_native_v2_soil_custody.ok_or(DirectSnowStage3V11AttachmentError::Identity(
            "adaptive preterminal V2 deferred soil custody presence",
        ))?;
    let authenticated_custody =
        crate::v9_real_consumer_shadow::DeferredNativeV2SoilCustodyV1::try_new(
            next_consumer,
            endpoint_soil.clone(),
            endpoint_continuation.cloned(),
        )
        .map_err(|_| {
            DirectSnowStage3V11AttachmentError::Identity(
                "adaptive preterminal V2 deferred soil custody authentication",
            )
        })?;
    if deferred_custody != &authenticated_custody
        || deferred_custody.candidate() != endpoint_soil
        || deferred_custody.continuation() != endpoint_continuation
    {
        return Err(DirectSnowStage3V11AttachmentError::Identity(
            "adaptive preterminal V2 deferred physical soil custody join",
        ));
    }

    let installed_owner_envelopes = next_parent.staged_resource_owners();
    let expected_owner_ids = [
        "vegetation",
        "snow",
        "land_surface_energy",
        "hydrology",
        "bgc",
        "soil_thermal",
        "surface_liquid",
    ]
    .into_iter()
    .collect::<BTreeSet<_>>();
    if installed_owner_envelopes
        .keys()
        .map(String::as_str)
        .collect::<BTreeSet<_>>()
        != expected_owner_ids
    {
        return Err(DirectSnowStage3V11AttachmentError::Identity(
            "adaptive preterminal V2 accepted owner topology",
        ));
    }
    let accepted_vegetation_envelope =
        openwepp_vegetation::v11::v11_vegetation_owner_envelope(next_parent.staged_state())
            .map_err(|_| {
                DirectSnowStage3V11AttachmentError::Identity(
                    "adaptive preterminal V2 accepted vegetation envelope",
                )
            })?;
    if installed_owner_envelopes.get("vegetation") != Some(&accepted_vegetation_envelope) {
        return Err(DirectSnowStage3V11AttachmentError::Identity(
            "adaptive preterminal V2 accepted vegetation state/envelope join",
        ));
    }
    let expected_owner_digests = [
        (
            "vegetation",
            receipt.owner_join.vegetation_owner_sha256,
            "adaptive preterminal V2 accepted vegetation-owner digest",
        ),
        (
            "land_surface_energy",
            receipt.owner_join.land_surface_energy_owner_sha256,
            "adaptive preterminal V2 accepted LSE-owner digest",
        ),
        (
            "hydrology",
            receipt.owner_join.hydrology_owner_sha256,
            "adaptive preterminal V2 accepted hydrology-owner digest",
        ),
        (
            "bgc",
            receipt.owner_join.biogeochemistry_owner_sha256,
            "adaptive preterminal V2 accepted BGC-owner digest",
        ),
        (
            "soil_thermal",
            receipt.owner_join.soil_thermal_owner_sha256,
            "adaptive preterminal V2 accepted soil-owner digest",
        ),
        (
            "surface_liquid",
            receipt.owner_join.surface_liquid_owner_sha256,
            "adaptive preterminal V2 accepted surface-owner digest",
        ),
        (
            "snow",
            receipt.owner_join.snow_owner_sha256,
            "adaptive preterminal V2 accepted snow-owner digest",
        ),
    ];
    for (owner_id, expected_digest, mismatch) in expected_owner_digests {
        let envelope = installed_owner_envelopes.get(owner_id).ok_or(
            DirectSnowStage3V11AttachmentError::Identity(
                "adaptive preterminal V2 installed owner topology",
            ),
        )?;
        envelope.to_owner_state().map_err(|_| {
            DirectSnowStage3V11AttachmentError::Identity(
                "adaptive preterminal V2 installed owner seal",
            )
        })?;
        if envelope.state_sha256 != expected_digest {
            return Err(DirectSnowStage3V11AttachmentError::Identity(mismatch));
        }
    }

    if endpoint.ending.stage3_by_lane() != ending_stage3
        || digest_bytes(&canonical_stage3_snow_owner_bytes(ending_stage3)?)
            != receipt.owner_join.stage3_physical_state_sha256
        || receipt.owner_join.receipt_sha256 == Digest32::zero()
    {
        return Err(DirectSnowStage3V11AttachmentError::Identity(
            "adaptive preterminal V2 Stage3/owner-join digest",
        ));
    }
    Ok(())
}

#[allow(clippy::too_many_arguments)]
#[inline(never)]
fn finalize_preterminal_replay_terminal_v1(
    terminal: Box<PreterminalReplayTerminalStepV1>,
    context: &DirectSnowStage3V11StaticContext,
    beginning_clock: &CoupledClockStateV1,
    beginning_terminal_parcels: &BTreeMap<Digest32, DirectSnowStage3V11TerminalParcel>,
    active_lanes: &BTreeSet<u32>,
    lane_id: &u32,
    event_ordinal: u64,
) -> Result<Box<ActualTerminalSubslabV1>, DirectSnowStage3V11AttachmentError> {
    let PreterminalReplayTerminalStepV1 {
        state,
        endpoint,
        step_event,
        physical_child_ordinal,
        deferred_native_v2_soil_custody,
    } = *terminal;
    let candidates = vec![Stage3V11ActualTerminalCandidateV1 {
        lane_id: *lane_id,
        tick: endpoint.support.end_ns(),
        support: endpoint.support,
        event: step_event,
        event_result_digest: endpoint.event_result_digest,
        terminal_state_sha256: digest_bytes(
            &Wb11HydrologyKernel::serialize_stage3_persistent_state(
                state
                    .stage3
                    .get(lane_id)
                    .ok_or(DirectSnowStage3V11AttachmentError::Identity(
                        "adaptive terminal installed lane",
                    ))?,
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
        state.clock.event_ordinal(),
    )?;
    let mut group = select_common_earliest_actual_terminal_group_v1(
        beginning_clock.parent_support(),
        terminal_group_ordinal,
        active_lanes,
        candidates,
    )?
    .ok_or(DirectSnowStage3V11AttachmentError::Terminal(
        "adaptive terminal event group",
    ))?;
    let PreterminalReplayStateV1 {
        parent,
        mut consumer,
        clock,
        stage3,
        receipts,
        deferred_native_v2_soil_custody: _,
    } = *state;
    let installed_receipt = receipts
        .last()
        .ok_or(DirectSnowStage3V11AttachmentError::Identity(
            "adaptive terminal installed receipt",
        ))?;
    let (parent, clock, stage3, parcels, accepted_event_receipt) = apply_actual_terminal_group(
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
    consumer.retain_accepted_publication_zero_duration_event(&accepted_event_receipt)?;
    group.accepted_event_receipt = Some(accepted_event_receipt);
    group.accepted_group_receipt_sha256 = Some(accepted_terminal_group_digest(&group)?);
    Ok(Box::new(ActualTerminalSubslabV1 {
        parent,
        consumer,
        clock,
        stage3,
        receipts,
        group: Some(group),
        parcels,
        deferred_native_v2_soil_custody,
    }))
}

#[inline(never)]
fn finalize_preterminal_replay_without_event_v1(
    state: Box<PreterminalReplayStateV1>,
) -> Box<ActualTerminalSubslabV1> {
    Box::new(ActualTerminalSubslabV1 {
        parent: state.parent,
        consumer: state.consumer,
        clock: state.clock,
        stage3: state.stage3,
        receipts: state.receipts,
        group: None,
        parcels: Vec::new(),
        deferred_native_v2_soil_custody: state.deferred_native_v2_soil_custody,
    })
}

#[allow(clippy::too_many_arguments, clippy::too_many_lines)]
#[inline(never)]
fn execute_preterminal_replay_v1(
    context: &DirectSnowStage3V11StaticContext,
    beginning_parent: &V11ParentTransaction,
    beginning_consumer: &DirectV10RealConsumerShadow,
    deferred_native_v2_soil_custody: Option<
        &crate::v9_real_consumer_shadow::DeferredNativeV2SoilCustodyV1,
    >,
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
    active_lanes: &BTreeSet<u32>,
    lane_id: &u32,
    result: &crate::hydrology::DirectSnowStage3PersistentDayResult,
    carrier_phases_by_joint: &BTreeMap<
        Digest32,
        crate::v9_real_consumer_shadow::CoveredCarrierPhaseResultV1,
    >,
    event: DirectSnowTerminalEventResult,
) -> Result<Option<ActualTerminalSubslabV1>, DirectSnowStage3V11AttachmentError> {
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
            && step.beginning_cold_content_j_m2.to_bits() == event.start_cold_content_j_m2.to_bits()
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
                && step.ending_liquid_kg_m2.to_bits() == event.terminal_liquid_kg_m2.to_bits()
                && step.ending_cold_content_j_m2.to_bits() == event.end_cold_content_j_m2.to_bits()
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
                ending_is_supported_snow_domain: stage3_is_resolved_thermal_domain(&result.state)
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
    let mut state = Box::new(PreterminalReplayStateV1 {
        parent: beginning_parent.clone(),
        consumer: beginning_consumer.clone(),
        clock: beginning_clock.clone(),
        stage3: beginning_stage3.clone(),
        receipts: Vec::new(),
        deferred_native_v2_soil_custody: deferred_native_v2_soil_custody.cloned(),
    });
    for step in &result.covered_terminal_accepted_microsteps {
        match execute_preterminal_replay_step_v1(
            state,
            context,
            prepared,
            day_index,
            interval_index,
            forcing_receipt,
            beginning_terminal_parcels,
            selected_upper_bound_s,
            current_child_ordinal,
            active_lanes,
            lane_id,
            carrier_phases_by_joint,
            event,
            step,
        )? {
            PreterminalReplayStepOutcomeV1::Continue(next) => state = next,
            PreterminalReplayStepOutcomeV1::Terminal(terminal) => {
                let completed = finalize_preterminal_replay_terminal_v1(
                    terminal,
                    context,
                    beginning_clock,
                    beginning_terminal_parcels,
                    active_lanes,
                    lane_id,
                    event_ordinal,
                )?;
                return Ok(Some(*completed));
            }
        }
    }
    Ok(Some(*finalize_preterminal_replay_without_event_v1(state)))
}

#[allow(clippy::too_many_arguments, clippy::too_many_lines)]
#[inline(never)]
fn try_actual_terminal_single_subslab_with_evidence<M>(
    context: &DirectSnowStage3V11StaticContext,
    beginning_parent: &V11ParentTransaction,
    beginning_consumer: &DirectV10RealConsumerShadow,
    deferred_native_v2_soil_custody: Option<
        &crate::v9_real_consumer_shadow::DeferredNativeV2SoilCustodyV1,
    >,
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
    let terminal_lanes = beginning_stage3
        .iter()
        .filter_map(|(lane, state)| {
            crate::hydrology::stage3_is_terminal_event_domain(state).then_some(*lane)
        })
        .collect::<BTreeSet<_>>();
    let mut candidate_ticks = BTreeSet::new();
    let mut discovery_candidates = BTreeMap::new();
    for lane_id in &terminal_lanes {
        let discovery = stage3_boxed_execution_v1(|| {
            evaluate_covered_terminal_candidate_with_evidence_v1::<M>(
                beginning_consumer,
                deferred_native_v2_soil_custody,
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
            )
        })?;
        let result = &discovery.0;
        let carrier_phases_by_joint = &discovery.2;
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
            let replay_outcome = stage3_boxed_execution_v1(|| {
                execute_preterminal_replay_v1(
                    context,
                    beginning_parent,
                    beginning_consumer,
                    deferred_native_v2_soil_custody,
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
                    &active_lanes,
                    lane_id,
                    result,
                    carrier_phases_by_joint,
                    event,
                )
            })?;
            return Ok(*replay_outcome);
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
            let exact = stage3_boxed_execution_v1(|| {
                evaluate_covered_terminal_candidate_with_evidence_v1::<M>(
                    beginning_consumer,
                    deferred_native_v2_soil_custody,
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
                )
            })?;
            exact_endpoints.push(prepare_exact_terminal_endpoint_v1(
                discovery,
                &exact.0,
                canonical_stage3_support_forcing_digest(&projected.support_forcing_by_lane),
                &exact.1,
                &exact.2,
            )?);
        }
        let exact = exact_endpoints
            .first()
            .ok_or(DirectSnowStage3V11AttachmentError::Terminal(
                "missing exact endpoint value",
            ))?;
        let outcome = stage3_boxed_execution_v1(|| {
            execute_covered_real_v11_subslab(
                context,
                beginning_parent,
                beginning_consumer,
                deferred_native_v2_soil_custody,
                beginning_clock,
                &projected,
                day_index,
                interval_index,
                forcing_receipt,
                beginning_stage3.clone(),
                None,
                beginning_terminal_parcels,
                selected_upper_bound_s,
                Some(&exact_endpoints),
            )
        })?;
        let (
            parent,
            installed_consumer,
            clock,
            installed_stage3,
            receipt,
            deferred_native_v2_soil_custody,
            snow_enthalpy_material_owner,
        ) = *outcome;
        if snow_enthalpy_material_owner.is_some() {
            return Err(DirectSnowStage3V11AttachmentError::Identity(
                "V56 noncrossing compound snow owner entered exact terminal install",
            ));
        }
        if receipt.terminal_events.is_empty() {
            continue;
        }
        let expected_installed = precomputed_terminal_package_v1(
            &exact_endpoints,
            beginning_terminal_parcels,
            receipt.accepted_slab_sha256,
            complete_owner_set_digest(beginning_clock.owners())?,
        )?;
        validate_exact_terminal_installer_owner_relation_v1(
            beginning_consumer,
            exact.ending.shadow(),
            &installed_consumer,
        )?;
        if exact.support != support
            || exact.forcing_sha256
                != canonical_stage3_support_forcing_digest(&projected.support_forcing_by_lane)
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
            deferred_native_v2_soil_custody,
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

// Source-contract anchors remain discoverable in this host after the lexical split:
// terminal zero-event V4 owner installation
// terminal event creates positive subminimum support
include!("snow_stage3_v11_terminal_event_application.rs");
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

type CoveredV11ExecutorV1<'a> = crate::v11_vegetation_consumer::DirectV11VegetationExecutor<
    DirectV11SnowCoveredRealConsumerStack<'a>,
>;

#[allow(clippy::too_many_arguments)]
#[inline(never)]
fn prepare_provisional_covered_executor_v1<'a>(
    beginning_consumer: &DirectV10RealConsumerShadow,
    beginning_snow_enthalpy_material_owner: Option<
        &crate::snow_stage3_v11_snow_enthalpy_carry::AuthenticatedCoveredSnowMaterialOwnerV1,
    >,
    deferred_native_v2_soil_custody: Option<
        &crate::v9_real_consumer_shadow::DeferredNativeV2SoilCustodyV1,
    >,
    covered_interval: &'a DirectV11SnowCoveredSegmentInput,
    prepared: &'a DirectSnowStage3V11PreparedSupport,
    beginning_stage3: BTreeMap<u32, DirectSnowStage3PersistentState>,
    pending_terminal_parcels: &BTreeMap<Digest32, DirectSnowStage3V11TerminalParcel>,
    day_index: usize,
    interval_index: usize,
    finalize_wb14_parent_interval: bool,
    provisional_binding: crate::direct_runtime::DirectWb14CoupledChildBindingV1,
    terminal_endpoints: Option<&[Box<ExactCoveredTerminalEndpointV1>]>,
    provisional_slab_sha256: Digest32,
    ledger_digest: Digest32,
) -> Result<Box<CoveredV11ExecutorV1<'a>>, DirectSnowStage3V11AttachmentError> {
    if let Some(endpoints) = terminal_endpoints {
        prepare_provisional_terminal_executor_v1(
            beginning_consumer,
            beginning_snow_enthalpy_material_owner,
            deferred_native_v2_soil_custody,
            covered_interval,
            prepared,
            beginning_stage3,
            pending_terminal_parcels,
            day_index,
            interval_index,
            finalize_wb14_parent_interval,
            provisional_binding,
            endpoints,
            provisional_slab_sha256,
            ledger_digest,
        )
    } else {
        prepare_ordinary_replay_executor_v1(
            beginning_consumer,
            beginning_snow_enthalpy_material_owner,
            deferred_native_v2_soil_custody,
            covered_interval,
            prepared,
            beginning_stage3,
            pending_terminal_parcels,
            day_index,
            interval_index,
            finalize_wb14_parent_interval,
            provisional_binding,
        )
    }
}

#[allow(clippy::too_many_arguments)]
#[inline(never)]
fn prepare_provisional_terminal_executor_v1<'a>(
    beginning_consumer: &DirectV10RealConsumerShadow,
    beginning_snow_enthalpy_material_owner: Option<
        &crate::snow_stage3_v11_snow_enthalpy_carry::AuthenticatedCoveredSnowMaterialOwnerV1,
    >,
    deferred_native_v2_soil_custody: Option<
        &crate::v9_real_consumer_shadow::DeferredNativeV2SoilCustodyV1,
    >,
    covered_interval: &'a DirectV11SnowCoveredSegmentInput,
    prepared: &'a DirectSnowStage3V11PreparedSupport,
    beginning_stage3: BTreeMap<u32, DirectSnowStage3PersistentState>,
    pending_terminal_parcels: &BTreeMap<Digest32, DirectSnowStage3V11TerminalParcel>,
    day_index: usize,
    interval_index: usize,
    finalize_wb14_parent_interval: bool,
    provisional_binding: crate::direct_runtime::DirectWb14CoupledChildBindingV1,
    endpoints: &[Box<ExactCoveredTerminalEndpointV1>],
    provisional_slab_sha256: Digest32,
    ledger_digest: Digest32,
) -> Result<Box<CoveredV11ExecutorV1<'a>>, DirectSnowStage3V11AttachmentError> {
    let mut stack = DirectV11SnowCoveredRealConsumerStack::new(
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
            finalize_wb14_parent_interval,
            wb14_coupled_child_binding: provisional_binding,
        },
    );
    if let Some(custody) = deferred_native_v2_soil_custody {
        stack = stack.try_with_deferred_native_v2_soil_custody(custody.clone())?;
    }
    if let Some(owner) = beginning_snow_enthalpy_material_owner {
        stack = stack.try_with_beginning_snow_enthalpy_material_owner(owner.clone())?;
    }
    let endpoint = precomputed_terminal_package_v1(
        endpoints,
        pending_terminal_parcels,
        provisional_slab_sha256,
        ledger_digest,
    )?;
    stack = if terminal_provisional_publication_deferral_enabled() {
        stack.with_precomputed_terminal_provisional_endpoint(endpoint)?
    } else {
        stack.with_precomputed_terminal_accepted_endpoint(endpoint)
    };
    Ok(Box::new(
        crate::v11_vegetation_consumer::DirectV11VegetationExecutor { stack },
    ))
}

#[inline(never)]
fn prepare_terminal_reuse_executor_v1<'a>(
    provisional_executor: Box<CoveredV11ExecutorV1<'a>>,
    final_binding: crate::direct_runtime::DirectWb14CoupledChildBindingV1,
    endpoint: crate::v9_real_consumer_shadow::PrecomputedTerminalAcceptedEndpointV1,
) -> Result<Box<CoveredV11ExecutorV1<'a>>, DirectSnowStage3V11AttachmentError> {
    stage3_boxed_execution_v1(|| {
        let stack = provisional_executor
            .stack
            .prepare_terminal_physical_reuse(final_binding, endpoint)?;
        Ok(crate::v11_vegetation_consumer::DirectV11VegetationExecutor { stack })
    })
}

#[allow(clippy::too_many_arguments)]
#[inline(never)]
fn prepare_terminal_replay_executor_v1<'a>(
    beginning_consumer: &DirectV10RealConsumerShadow,
    beginning_snow_enthalpy_material_owner: Option<
        &crate::snow_stage3_v11_snow_enthalpy_carry::AuthenticatedCoveredSnowMaterialOwnerV1,
    >,
    deferred_native_v2_soil_custody: Option<
        &crate::v9_real_consumer_shadow::DeferredNativeV2SoilCustodyV1,
    >,
    covered_interval: &'a DirectV11SnowCoveredSegmentInput,
    prepared: &'a DirectSnowStage3V11PreparedSupport,
    beginning_stage3: BTreeMap<u32, DirectSnowStage3PersistentState>,
    pending_terminal_parcels: &BTreeMap<Digest32, DirectSnowStage3V11TerminalParcel>,
    day_index: usize,
    interval_index: usize,
    finalize_wb14_parent_interval: bool,
    final_binding: crate::direct_runtime::DirectWb14CoupledChildBindingV1,
    endpoint: crate::v9_real_consumer_shadow::PrecomputedTerminalAcceptedEndpointV1,
) -> Result<Box<CoveredV11ExecutorV1<'a>>, DirectSnowStage3V11AttachmentError> {
    stage3_boxed_execution_v1(|| {
        let stack = DirectV11SnowCoveredRealConsumerStack::new(
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
                finalize_wb14_parent_interval,
                wb14_coupled_child_binding: final_binding,
            },
        );
        let mut stack = match deferred_native_v2_soil_custody {
            Some(custody) => stack.try_with_deferred_native_v2_soil_custody(custody.clone())?,
            None => stack,
        };
        if let Some(owner) = beginning_snow_enthalpy_material_owner {
            stack = stack.try_with_beginning_snow_enthalpy_material_owner(owner.clone())?;
        }
        let stack = stack.with_precomputed_terminal_accepted_endpoint(endpoint);
        Ok(crate::v11_vegetation_consumer::DirectV11VegetationExecutor { stack })
    })
}

#[inline(never)]
fn prepare_ordinary_reuse_executor_v1<'a>(
    provisional_executor: Box<CoveredV11ExecutorV1<'a>>,
    final_binding: crate::direct_runtime::DirectWb14CoupledChildBindingV1,
) -> Result<Box<CoveredV11ExecutorV1<'a>>, DirectSnowStage3V11AttachmentError> {
    stage3_boxed_execution_v1(|| {
        let stack = provisional_executor
            .stack
            .prepare_ordinary_physical_reuse(final_binding)?;
        Ok(crate::v11_vegetation_consumer::DirectV11VegetationExecutor { stack })
    })
}

#[allow(clippy::too_many_arguments)]
#[inline(never)]
fn prepare_ordinary_replay_executor_v1<'a>(
    beginning_consumer: &DirectV10RealConsumerShadow,
    beginning_snow_enthalpy_material_owner: Option<
        &crate::snow_stage3_v11_snow_enthalpy_carry::AuthenticatedCoveredSnowMaterialOwnerV1,
    >,
    deferred_native_v2_soil_custody: Option<
        &crate::v9_real_consumer_shadow::DeferredNativeV2SoilCustodyV1,
    >,
    covered_interval: &'a DirectV11SnowCoveredSegmentInput,
    prepared: &'a DirectSnowStage3V11PreparedSupport,
    beginning_stage3: BTreeMap<u32, DirectSnowStage3PersistentState>,
    pending_terminal_parcels: &BTreeMap<Digest32, DirectSnowStage3V11TerminalParcel>,
    day_index: usize,
    interval_index: usize,
    finalize_wb14_parent_interval: bool,
    final_binding: crate::direct_runtime::DirectWb14CoupledChildBindingV1,
) -> Result<Box<CoveredV11ExecutorV1<'a>>, DirectSnowStage3V11AttachmentError> {
    stage3_boxed_execution_v1(|| {
        let stack = DirectV11SnowCoveredRealConsumerStack::new(
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
                finalize_wb14_parent_interval,
                wb14_coupled_child_binding: final_binding,
            },
        );
        let mut stack = match deferred_native_v2_soil_custody {
            Some(custody) => stack.try_with_deferred_native_v2_soil_custody(custody.clone())?,
            None => stack,
        };
        if let Some(owner) = beginning_snow_enthalpy_material_owner {
            stack = stack.try_with_beginning_snow_enthalpy_material_owner(owner.clone())?;
        }
        Ok(crate::v11_vegetation_consumer::DirectV11VegetationExecutor { stack })
    })
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
    beginning_deferred_native_v2_soil_custody: Option<
        &crate::v9_real_consumer_shadow::DeferredNativeV2SoilCustodyV1,
    >,
    beginning_clock: &CoupledClockStateV1,
    prepared: &DirectSnowStage3V11PreparedSupport,
    day_index: usize,
    interval_index: usize,
    forcing_receipt: Digest32,
    beginning_stage3: BTreeMap<u32, DirectSnowStage3PersistentState>,
    beginning_snow_enthalpy_material_owner: Option<
        &crate::snow_stage3_v11_snow_enthalpy_carry::AuthenticatedCoveredSnowMaterialOwnerV1,
    >,
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
        Option<crate::v9_real_consumer_shadow::DeferredNativeV2SoilCustodyV1>,
        Option<crate::snow_stage3_v11_snow_enthalpy_carry::AuthenticatedCoveredSnowMaterialOwnerV1>,
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
    let provisional_binding = crate::direct_runtime::DirectWb14CoupledChildBindingV1 {
        proposed_upper_bound_s_bits: selected_upper_bound_s.to_bits(),
        coupled_parent_transaction_sha256: *parent_id.digest().as_bytes(),
        accepted_slab_sha256: *provisional_receipt.slab_id().digest().as_bytes(),
        parent_beginning_complete_owner_set_sha256: *ledger_digest.as_bytes(),
        parent_support_start_ns: beginning_clock.parent_support().start_ns().get(),
        parent_support_end_ns: beginning_clock.parent_support().end_ns().get(),
        child_support_start_ns: support.start_ns().get() as u128,
        child_support_end_ns: support.end_ns().get() as u128,
    };
    let finalize_wb14_parent_interval =
        support.end_ns() == beginning_clock.parent_support().end_ns();
    let mut provisional_executor = prepare_provisional_covered_executor_v1(
        beginning_consumer,
        beginning_snow_enthalpy_material_owner,
        beginning_deferred_native_v2_soil_custody,
        covered_interval,
        prepared,
        beginning_stage3.clone(),
        pending_terminal_parcels,
        day_index,
        interval_index,
        finalize_wb14_parent_interval,
        provisional_binding,
        terminal_endpoints,
        provisional_receipt.slab_id().digest(),
        ledger_digest,
    )?;
    #[cfg(test)]
    let provisional_started = std::time::Instant::now();
    #[cfg(test)]
    let provisional_capture_regime =
        crate::snow_stage3_v11_attachment::enter_accepted_publication_capture_regime_v1(
            crate::snow_stage3_v11_attachment::AcceptedPublicationCaptureRegimeV1::ProvisionalClock,
        );
    let provisional_segment = stage3_boxed_execution_v1(|| {
        execute_direct_v11_segment(
            &context.vegetation_configuration,
            beginning_parent,
            &provisional_receipt,
            provisional_executor.as_mut(),
        )
    })?;
    let deferred_native_v2_soil_custody = if terminal_provisional_publication_deferral_enabled() {
        terminal_endpoints
            .and_then(|endpoints| endpoints.first())
            .map(|endpoint| deferred_native_v2_soil_custody_v1(beginning_consumer, endpoint))
            .transpose()?
            .flatten()
    } else {
        None
    };
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
    let mut final_executor = if let Some(endpoints) = terminal_endpoints {
        let endpoint = precomputed_terminal_package_v1(
            endpoints,
            pending_terminal_parcels,
            final_receipt.slab_id().digest(),
            ledger_digest,
        )?;
        if terminal_provisional_publication_deferral_enabled() {
            prepare_terminal_reuse_executor_v1(provisional_executor, final_binding, endpoint)?
        } else {
            drop(provisional_executor);
            prepare_terminal_replay_executor_v1(
                beginning_consumer,
                beginning_snow_enthalpy_material_owner,
                beginning_deferred_native_v2_soil_custody,
                covered_interval,
                prepared,
                beginning_stage3,
                pending_terminal_parcels,
                day_index,
                interval_index,
                finalize_wb14_parent_interval,
                final_binding,
                endpoint,
            )?
        }
    } else if ordinary_covered_physical_reuse_enabled() {
        prepare_ordinary_reuse_executor_v1(provisional_executor, final_binding)?
    } else {
        drop(provisional_executor);
        prepare_ordinary_replay_executor_v1(
            beginning_consumer,
            beginning_snow_enthalpy_material_owner,
            beginning_deferred_native_v2_soil_custody,
            covered_interval,
            prepared,
            beginning_stage3,
            pending_terminal_parcels,
            day_index,
            interval_index,
            finalize_wb14_parent_interval,
            final_binding,
        )?
    };
    #[cfg(test)]
    let final_started = std::time::Instant::now();
    #[cfg(test)]
    let final_capture_regime =
        crate::snow_stage3_v11_attachment::enter_accepted_publication_capture_regime_v1(
            crate::snow_stage3_v11_attachment::AcceptedPublicationCaptureRegimeV1::FinalClock,
        );
    let final_segment = stage3_boxed_execution_v1(|| {
        execute_direct_v11_segment(
            &context.vegetation_configuration,
            beginning_parent,
            &final_receipt,
            final_executor.as_mut(),
        )
    })?;
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
    let ending_snow_enthalpy_material_owner = final_executor
        .stack
        .last_snow_enthalpy_material_owner()
        .cloned();
    let ending_stage3 = final_executor.stack.take_staged_stage3().ok_or(
        DirectSnowStage3V11AttachmentError::Identity("missing staged covered Stage-3 ending"),
    )?;
    let mut consumer = final_executor.stack.take_staged_ending().ok_or(
        DirectSnowStage3V11AttachmentError::Identity("missing staged covered ending"),
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
            !adaptive_terminal_snow_soil_trial_receipts.contains_key(lane_id)
                && final_executor
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
    let installed_soil = consumer.soil_thermal_resident().read_view();
    installed_soil
        .validate()
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
            .ordered_ofes()
            .into_iter()
            .find(|value| value.ofe_id() == &receipt.ofe_id)
            .ok_or(DirectSnowStage3V11AttachmentError::Identity(
                "snow-soil installed soil OFE",
            ))?;
        let installed_soil_top = soil_ofe.ordered_layers().into_iter().next().ok_or(
            DirectSnowStage3V11AttachmentError::Identity("snow-soil installed top soil node"),
        )?;
        let installed_snow_identity = digest_bytes(&serde_json::to_vec(state).map_err(|_| {
            DirectSnowStage3V11AttachmentError::Identity("installed Stage-3 lane identity")
        })?);
        let installed_soil_identity = digest_bytes(
            &match soil_ofe {
                crate::v9_real_consumer_shadow::DirectSoilThermalOfeReadView::V1(value) => {
                    serde_json::to_vec(value)
                }
                crate::v9_real_consumer_shadow::DirectSoilThermalOfeReadView::V2(value) => {
                    serde_json::to_vec(value)
                }
            }
            .map_err(|_| {
                DirectSnowStage3V11AttachmentError::Identity("installed soil OFE identity")
            })?,
        );
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
            installed_soil_top.layer_id(),
            installed_snow_identity,
            installed_soil_identity,
        )?;
        if !close_temperature(
            installed_snow_bottom.temperature_k,
            receipt.ending_bottom_snow_temperature_k,
        ) || !close_temperature(
            installed_soil_top.temperature_k(),
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
            .ordered_ofes()
            .into_iter()
            .find(|value| value.ofe_id() == &receipt.ofe_id)
            .ok_or(DirectSnowStage3V11AttachmentError::Identity(
                "terminal snow-soil installed soil OFE",
            ))?;
        let installed_snow_identity = digest_bytes(
            &Wb11HydrologyKernel::serialize_stage3_persistent_state(state)?,
        );
        let installed_soil_identity = digest_bytes(
            &match soil_ofe {
                crate::v9_real_consumer_shadow::DirectSoilThermalOfeReadView::V1(value) => {
                    serde_json::to_vec(value)
                }
                crate::v9_real_consumer_shadow::DirectSoilThermalOfeReadView::V2(value) => {
                    serde_json::to_vec(value)
                }
            }
            .map_err(|_| {
                DirectSnowStage3V11AttachmentError::Identity("terminal installed soil OFE identity")
            })?,
        );
        // Native-V2 terminal publication deliberately keeps the beginning
        // resident installed until the first snow-free successor.  The
        // terminal heat receipt, however, owns the exact selected physical
        // soil candidate held in deferred transient custody.  Join the
        // receipt to that authenticated candidate when it exists; using the
        // resident here would substitute publication posture for physical
        // custody and reject every valid deferred terminal endpoint.
        let receipt_ending_soil_identity =
            if let Some(custody) = deferred_native_v2_soil_custody.as_ref() {
                let physical_soil = custody.candidate().read_view();
                let physical_soil_ofe = physical_soil
                    .ordered_ofes()
                    .into_iter()
                    .find(|value| value.ofe_id() == &receipt.ofe_id)
                    .ok_or(DirectSnowStage3V11AttachmentError::Identity(
                        "terminal snow-soil deferred physical soil OFE",
                    ))?;
                digest_bytes(
                    &match physical_soil_ofe {
                        crate::v9_real_consumer_shadow::DirectSoilThermalOfeReadView::V1(value) => {
                            serde_json::to_vec(value)
                        }
                        crate::v9_real_consumer_shadow::DirectSoilThermalOfeReadView::V2(value) => {
                            serde_json::to_vec(value)
                        }
                    }
                    .map_err(|_| {
                        DirectSnowStage3V11AttachmentError::Identity(
                            "terminal deferred physical soil OFE identity",
                        )
                    })?,
                )
            } else {
                installed_soil_identity
            };
        let event_lane = terminal_events.contains_key(lane_id);
        let invalid_phase_posture = if event_lane {
            crate::hydrology::stage3_has_represented_ice(state)
                || !state.layers.is_empty()
                || state.detached_retained_liquid_kg_m2.to_bits() != 0.0_f64.to_bits()
        } else {
            !crate::hydrology::stage3_is_terminal_event_domain(state)
        };
        if invalid_phase_posture {
            return Err(DirectSnowStage3V11AttachmentError::Identity(
                "terminal snow-soil dormant installed phase posture",
            ));
        }
        if receipt.ending_dormant_snow_owner_sha256 != installed_snow_identity {
            return Err(DirectSnowStage3V11AttachmentError::Identity(
                "terminal snow-soil dormant installed snow identity",
            ));
        }
        if receipt.ending_soil_owner_sha256 != receipt_ending_soil_identity {
            return Err(DirectSnowStage3V11AttachmentError::Identity(
                "terminal snow-soil dormant physical soil identity",
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
        *final_segment,
        beginning_consumer,
    )?;
    #[cfg(any(test, feature = "persisted-restart-v1"))]
    crate::v9_real_consumer_shadow::record_snow_free_outer_accepted_publication_v1();
    if terminal_endpoints.is_some()
        && final_executor.stack.last_publication_retained() != Some(true)
    {
        return Err(DirectSnowStage3V11AttachmentError::Identity(
            "missing final accepted publication append",
        ));
    }
    let (wb14_child_replay_bytes, wb14_parent_replay_bytes) =
        final_executor.stack.last_wb14_replay_bytes().ok_or(
            DirectSnowStage3V11AttachmentError::Identity("missing WB14 replay receipt payload"),
        )?;
    let wb14_child_replay_bytes = wb14_child_replay_bytes.to_vec();
    let wb14_parent_replay_bytes = wb14_parent_replay_bytes.map(ToOwned::to_owned);
    let stage3_covered_native_inactive =
        crate::direct_runtime::stage3_covered_native_inactive_child_custody_binding(
            &wb14_child_replay_bytes,
            &context.surface_liquid_configuration.ofe_topology,
        )
        .map_err(|error| {
            DirectSnowStage3V11AttachmentError::Owner(
                crate::v9_real_consumer_shadow::DirectV11RealConsumerError::SurfaceLiquidReplay(
                    error,
                ),
            )
        })?
        .is_some();
    let mut parent_after_segment = parent;
    let terminal_lanes = terminal_events.keys().copied().collect::<BTreeSet<_>>();
    let post_support_liquid_receiver = if stage3_covered_native_inactive {
        None
    } else {
        consume_positive_support_snow_liquid_v1(
            context,
            &mut parent_after_segment,
            &mut consumer,
            &mut final_clock,
            support,
            owner_join.ending_complete_owner_set_sha256,
            &terminal_lanes,
        )?
    };
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
        wb14_ofe_topology: context.surface_liquid_configuration.ofe_topology.clone(),
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
        deferred_native_v2_soil_custody,
        ending_snow_enthalpy_material_owner,
    ))
}

#[cfg(test)]
mod terminal_provider_retention_tests {
    #[test]
    fn provider_retains_exact_joint_phase_without_deep_clone_until_unwind() {
        let source = include_str!("snow_stage3_v11_terminal_execution.rs");
        let provider = source
            .split("fn evaluate_covered_terminal_candidate_with_evidence_v1")
            .nth(1)
            .expect("terminal provider source")
            .split("struct CoveredTerminalBatchCandidateV2")
            .next()
            .expect("terminal provider body");
        let unwind = provider.find("drop(provider);").expect("provider unwind");
        let deferred_clone = provider
            .find("retained.candidates().clone()")
            .expect("deferred candidate materialization");
        assert!(provider.contains("CoveredTerminalProviderRetentionV1::Phase(result.clone())"));
        assert!(provider.contains("request.beginning_joint.receipt_sha256()"));
        assert!(!provider[..unwind].contains("result.ending_candidates.clone()"));
        assert!(deferred_clone > unwind);
    }
}

#[cfg(test)]
include!("snow_stage3_v11_terminal_execution_tests.rs");

include!("snow_stage3_v11_attachment_helpers.rs");
