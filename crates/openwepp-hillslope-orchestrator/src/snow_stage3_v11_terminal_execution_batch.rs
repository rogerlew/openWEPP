#[allow(clippy::too_many_arguments, clippy::too_many_lines)]
#[inline(never)]
fn finalize_terminal_batch_subslab_v2(
    context: &DirectSnowStage3V11StaticContext,
    beginning_parent: &V11ParentTransaction,
    beginning_consumer: &DirectV10RealConsumerShadow,
    deferred_native_v2_soil_custody: Option<
        &crate::v9_real_consumer_shadow::DeferredNativeV2SoilCustodyV1,
    >,
    beginning_clock: &CoupledClockStateV1,
    exact_prepared: &DirectSnowStage3V11PreparedSupport,
    exact: &CoveredTerminalBatchCandidateV2,
    terminal_lanes: &BTreeSet<u32>,
    day_index: usize,
    interval_index: usize,
    forcing_receipt: Digest32,
    beginning_stage3: &BTreeMap<u32, DirectSnowStage3PersistentState>,
    beginning_terminal_parcels: &BTreeMap<Digest32, DirectSnowStage3V11TerminalParcel>,
    selected_upper_bound_s: f64,
    current_child_ordinal: u32,
    event_ordinal: u64,
) -> Result<ActualTerminalSubslabV1, DirectSnowStage3V11AttachmentError> {
    let replay_trial_sha256 = exact.phase.transition.probe_child_identity.receipt_sha256;
    let wb14_ofe_topology = exact
        .phase
        .beginning_candidates
        .shadow()
        .surface_configuration()
        .ofe_topology
        .clone();
    let lower_boundary_ofes = exact
        .phase
        .complete_lower_boundaries
        .keys()
        .map(|(ofe_id, _)| ofe_id)
        .collect::<BTreeSet<_>>();
    if lower_boundary_ofes.is_empty()
        || lower_boundary_ofes
            .iter()
            .any(|ofe_id| !wb14_ofe_topology.contains(ofe_id))
    {
        return Err(DirectSnowStage3V11AttachmentError::Terminal(
            "terminal batch lower-boundary OFE topology",
        ));
    }
    let replay_binding =
        crate::direct_runtime::stage3_covered_native_inactive_child_custody_binding(
            &exact.phase.wb14_child_replay_bytes,
            &wb14_ofe_topology,
        )
        .map_err(|error| {
            DirectSnowStage3V11AttachmentError::Owner(
                crate::v9_real_consumer_shadow::DirectV11RealConsumerError::SurfaceLiquidReplay(
                    error,
                ),
            )
        })?
        .map_or_else(
            || {
                crate::direct_runtime::wb14_child_replay_binding(
                    &exact.phase.wb14_child_replay_bytes,
                )
                .map_err(|error| {
                    DirectSnowStage3V11AttachmentError::Owner(
                        crate::v9_real_consumer_shadow::DirectV11RealConsumerError::SurfaceLiquidReplay(
                            error,
                        ),
                    )
                })
            },
            Ok,
        )?;
    let replay_beginning_owner_sha256 =
        Digest32::from_bytes(replay_binding.parent_beginning_complete_owner_set_sha256);
    let forcing_sha256 =
        canonical_stage3_support_forcing_digest(&exact_prepared.support_forcing_by_lane);
    let mut endpoints = Vec::new();
    for lane_id in terminal_lanes {
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
            ending: exact.ending.as_ref().clone(),
            carrier_phase: Box::new(exact.phase.as_ref().clone()),
            carrier_phase_chain: vec![exact.phase.as_ref().clone()],
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
    let outcome = stage3_boxed_execution_v1(|| {
        execute_covered_real_v11_subslab(
            context,
            beginning_parent,
            beginning_consumer,
            deferred_native_v2_soil_custody,
            beginning_clock,
            exact_prepared,
            day_index,
            interval_index,
            forcing_receipt,
            beginning_stage3.clone(),
            None,
            beginning_terminal_parcels,
            selected_upper_bound_s,
            Some(&endpoints),
        )
    })?;
    let (
        parent,
        mut consumer,
        clock,
        stage3,
        receipt,
        deferred_native_v2_soil_custody,
        snow_enthalpy_material_owner,
    ) = *outcome;
    if snow_enthalpy_material_owner.is_some() {
        return Err(DirectSnowStage3V11AttachmentError::Identity(
            "V56 noncrossing compound snow owner entered terminal batch",
        ));
    }
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
        return Ok(ActualTerminalSubslabV1 {
            parent,
            consumer,
            clock,
            stage3,
            receipts: vec![receipt],
            group: None,
            parcels: Vec::new(),
            deferred_native_v2_soil_custody,
        });
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
    Ok(ActualTerminalSubslabV1 {
        parent,
        consumer,
        clock,
        stage3,
        receipts: vec![receipt],
        group: Some(group),
        parcels,
        deferred_native_v2_soil_custody,
    })
}

#[allow(clippy::too_many_arguments, clippy::too_many_lines)]
fn evaluate_covered_terminal_batch_candidate_v2(
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
) -> Result<CoveredTerminalBatchCandidateV2, DirectSnowStage3V11AttachmentError> {
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
        let snow_density_model = prepared
            .snow_inputs_by_lane
            .get(lane_id)
            .ok_or(DirectSnowStage3V11AttachmentError::Identity(
                "terminal batch density-model lane",
            ))?
            .snow_density_model;
        lanes.insert(
            *lane_id,
            crate::hydrology::CoveredTerminalLaneTrialStateV2 {
                lane_id: *lane_id,
                schema_version: state.schema_version,
                terminal_event_model: state.terminal_event_model,
                next_interval_index: state.next_interval_index,
                snow_density_model,
                ice_kg_m2,
                liquid_kg_m2,
                cold_content_j_m2,
                surface_temperature_c: surface.surface_temperature_k - 273.15,
                snow_depth_m,
                snow_density_kg_m3,
                layer_density_kg_m3: state
                    .layers
                    .iter()
                    .map(|layer| layer.density_kg_m3)
                    .collect(),
                layer_settle_day_count: state
                    .layers
                    .iter()
                    .map(|layer| layer.settle_day_count)
                    .collect(),
                represented_layers: state.layers.clone(),
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
    let stack = match deferred_native_v2_soil_custody {
        Some(custody) => stack.try_with_deferred_native_v2_soil_custody(custody.clone())?,
        None => stack,
    };
    let phase = stage3_boxed_execution_v1(|| {
        stack.execute_covered_carrier_batch_phase_v2(&initial_candidates, &request, child)
    })?;
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
        let mut result = stage3_boxed_execution_v1(|| {
            if crate::hydrology::stage3_is_terminal_event_domain(state) {
                Wb11HydrologyKernel::evaluate_stage3_terminal_batch_support_with_boundary_v2(
                    inputs,
                    state,
                    lane_id,
                    state.next_interval_index,
                    forcing,
                    boundary,
                )
            } else {
                Wb11HydrologyKernel::evaluate_stage3_persistent_support_with_boundary(
                    inputs,
                    state,
                    lane_id,
                    state.next_interval_index,
                    forcing,
                    boundary,
                )
            }
        })?;
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
    let batch_result = stage3_boxed_execution_v1(|| {
        Wb11HydrologyKernel::execute_covered_terminal_batch_trial_v2(
            &request,
            hydrology_endings.clone(),
            &mut provider,
            &mut join,
        )
    })?;
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
    let ending = stage3_boxed_execution_v1(|| {
        phase
            .ending_candidates
            .try_with_selected_stage3_by_lane(batch_result.ending_joint.clone(), hydrology_endings)
    })?;
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
) -> Result<Option<ActualTerminalSubslabV1>, DirectSnowStage3V11AttachmentError> {
    let discovery = stage3_boxed_execution_v1(|| {
        evaluate_covered_terminal_batch_candidate_v2(
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
        )
    })?;
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
        stage3_boxed_execution_v1(|| {
            evaluate_covered_terminal_batch_candidate_v2(
                beginning_consumer,
                deferred_native_v2_soil_custody,
                beginning_clock,
                &exact_prepared,
                day_index,
                interval_index,
                beginning_stage3,
                beginning_terminal_parcels,
                selected_upper_bound_s,
                current_child_ordinal,
            )
        })?
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
        let outcome = stage3_boxed_execution_v1(|| {
            execute_covered_real_v11_subslab(
                context,
                beginning_parent,
                beginning_consumer,
                deferred_native_v2_soil_custody,
                beginning_clock,
                &exact_prepared,
                day_index,
                interval_index,
                forcing_receipt,
                beginning_stage3.clone(),
                None,
                beginning_terminal_parcels,
                selected_upper_bound_s,
                None,
            )
        })?;
        let (
            parent,
            consumer,
            clock,
            stage3,
            receipt,
            deferred_native_v2_soil_custody,
            snow_enthalpy_material_owner,
        ) = *outcome;
        if snow_enthalpy_material_owner.is_some() {
            return Err(DirectSnowStage3V11AttachmentError::Identity(
                "V56 noncrossing compound snow owner entered terminal no-event batch",
            ));
        }
        return Ok(Some(ActualTerminalSubslabV1 {
            parent,
            consumer,
            clock,
            stage3,
            receipts: vec![receipt],
            group: None,
            parcels: Vec::new(),
            deferred_native_v2_soil_custody,
        }));
    }
    let finalized = finalize_terminal_batch_subslab_v2(
        context,
        beginning_parent,
        beginning_consumer,
        deferred_native_v2_soil_custody,
        beginning_clock,
        &exact_prepared,
        &exact,
        &terminal_lanes,
        day_index,
        interval_index,
        forcing_receipt,
        beginning_stage3,
        beginning_terminal_parcels,
        selected_upper_bound_s,
        current_child_ordinal,
        event_ordinal,
    )?;
    Ok(Some(finalized))
}

pub(crate) fn validate_accepted_terminal_group_for_native_prefix_v1(
    group: &Stage3V11TerminalEventGroupV1,
    parent: TimeSupport,
    configuration: &DirectSurfaceLiquidConfiguration,
    physical_child_ordinal: u32,
) -> Result<(), DirectSnowStage3V11AttachmentError> {
    if group.pre_active_lanes.is_empty()
        || group.terminating_lanes.is_empty()
        || !group.terminating_lanes.is_subset(&group.pre_active_lanes)
    {
        return Err(DirectSnowStage3V11AttachmentError::Identity(
            "native inactive-prefix terminal participant set",
        ));
    }
    let candidate_lanes = group
        .candidates
        .iter()
        .map(|candidate| candidate.lane_id)
        .collect::<BTreeSet<_>>();
    let expected_post = group
        .pre_active_lanes
        .difference(&group.terminating_lanes)
        .copied()
        .collect::<BTreeSet<_>>();
    if candidate_lanes.len() != group.candidates.len()
        || candidate_lanes != group.terminating_lanes
        || expected_post != group.post_active_lanes
        || group
            .candidates
            .iter()
            .any(|candidate| candidate.tick != group.tick)
    {
        return Err(DirectSnowStage3V11AttachmentError::Identity(
            "native inactive-prefix terminal participant transition",
        ));
    }
    for candidate in &group.candidates {
        candidate.validate(parent, &group.pre_active_lanes)?;
    }
    validate_retained_terminal_receiver_custody_v1(group)?;
    group.validate_terminal_receiver_custody_v2()?;

    let accepted = group.accepted_event_receipt.as_ref().ok_or(
        DirectSnowStage3V11AttachmentError::Identity(
            "native inactive-prefix terminal accepted event",
        ),
    )?;
    accepted.validate().map_err(|_| {
        DirectSnowStage3V11AttachmentError::Identity(
            "native inactive-prefix terminal accepted event seal",
        )
    })?;
    let ledger = group.terminal_physical_ledger.as_ref().ok_or(
        DirectSnowStage3V11AttachmentError::Identity(
            "native inactive-prefix terminal physical ledger",
        ),
    )?;
    ledger.validate()?;

    let discovery = terminal_event_group_digest(
        parent,
        group.tick,
        group.ordinal,
        &group.pre_active_lanes,
        &group.post_active_lanes,
        &group.candidates,
    )?;
    let proposal_core = terminal_event_proposal_core(
        configuration,
        group,
        accepted.parent_transaction_id().digest(),
        parent,
        physical_child_ordinal,
    )?;
    let parcel_fields = group
        .produced_unconsumed_parcel_digests
        .iter()
        .map(|digest| FramedField {
            tag: "parcel",
            value: digest.as_bytes(),
        })
        .collect::<Vec<_>>();
    let parcel_set = framed_sha256("stage3-v11-terminal-parcel-set", &parcel_fields)?;
    let topology = group
        .produced_unconsumed_parcels
        .first()
        .ok_or(DirectSnowStage3V11AttachmentError::Identity(
            "native inactive-prefix terminal topology",
        ))?
        .receiver_topology_sha256;
    let mut candidate_members = u32::try_from(group.candidates.len())
        .map_err(|_| {
            DirectSnowStage3V11AttachmentError::Identity(
                "native inactive-prefix terminal candidate count",
            )
        })?
        .to_be_bytes()
        .to_vec();
    for candidate in &group.candidates {
        let mut member = Vec::new();
        member.extend_from_slice(&candidate.lane_id.to_be_bytes());
        member.extend_from_slice(candidate.event_result_digest.as_bytes());
        member.extend_from_slice(candidate.terminal_state_sha256.as_bytes());
        member.extend_from_slice(&candidate.event.terminal_liquid_kg_m2.to_bits().to_be_bytes());
        member.extend_from_slice(
            &candidate
                .event
                .terminal_unallocated_energy_j_m2
                .to_bits()
                .to_be_bytes(),
        );
        member.extend_from_slice(parcel_set.as_bytes());
        candidate_members.extend_from_slice(
            &u32::try_from(member.len())
                .map_err(|_| {
                    DirectSnowStage3V11AttachmentError::Identity(
                        "native inactive-prefix terminal candidate width",
                    )
                })?
                .to_be_bytes(),
        );
        candidate_members.extend_from_slice(&member);
    }
    let search = group.candidates[0].support;
    let ordinal = u32::try_from(group.ordinal).map_err(|_| {
        DirectSnowStage3V11AttachmentError::Identity(
            "native inactive-prefix terminal ordinal width",
        )
    })?;
    let receipt = framed_sha256(
        "stage3-v11-terminal-group-preaccept",
        &[
            FramedField { tag: "schema", value: &1_u32.to_be_bytes() },
            FramedField { tag: "proposal_core", value: proposal_core.as_bytes() },
            FramedField { tag: "parent_transaction", value: accepted.parent_transaction_id().digest().as_bytes() },
            FramedField { tag: "enclosing_start", value: &parent.start_ns().get().to_be_bytes() },
            FramedField { tag: "enclosing_end", value: &parent.end_ns().get().to_be_bytes() },
            FramedField { tag: "search_start", value: &search.start_ns().get().to_be_bytes() },
            FramedField { tag: "search_end", value: &search.end_ns().get().to_be_bytes() },
            FramedField { tag: "event_tick", value: &group.tick.get().to_be_bytes() },
            FramedField { tag: "child_ordinal", value: &physical_child_ordinal.to_be_bytes() },
            FramedField { tag: "event_ordinal", value: &ordinal.to_be_bytes() },
            FramedField { tag: "forcing", value: group.candidates[0].shortened_forcing_sha256.as_bytes() },
            FramedField { tag: "topology", value: topology.as_bytes() },
            FramedField { tag: "begin_owner_set", value: ledger.beginning_owner_set_sha256.as_bytes() },
            FramedField { tag: "proposed_end_owner_set", value: ledger.ending_owner_set_sha256.as_bytes() },
            FramedField { tag: "mutations", value: b"\0\0\0\x04snow" },
            FramedField { tag: "candidates", value: &candidate_members },
        ],
    )?;
    let accepted_group = accepted_terminal_group_digest(group)?;
    if group.discovery_receipt_sha256 != discovery
        || group.proposal_core_sha256 != Some(proposal_core)
        || ledger.proposal_core_sha256 != proposal_core
        || ledger.produced_unconsumed_parcel_set_sha256 != parcel_set
        || group.receipt_sha256 != receipt
        || group.accepted_group_receipt_sha256 != Some(accepted_group)
    {
        return Err(DirectSnowStage3V11AttachmentError::Identity(
            "native inactive-prefix terminal group reconstruction",
        ));
    }
    Ok(())
}
