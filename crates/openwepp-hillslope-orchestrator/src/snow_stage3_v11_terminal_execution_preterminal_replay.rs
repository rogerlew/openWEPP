struct PreterminalReplayStateV1 {
    parent: V11ParentTransaction,
    consumer: DirectV10RealConsumerShadow,
    clock: CoupledClockStateV1,
    stage3: BTreeMap<u32, DirectSnowStage3PersistentState>,
    receipts: Vec<Stage3CoupledSubslabReceiptV1>,
    deferred_native_v2_soil_custody:
        Option<crate::v9_real_consumer_shadow::DeferredNativeV2SoilCustodyV1>,
}

enum PreterminalReplayStepOutcomeV1 {
    Continue(Box<PreterminalReplayStateV1>),
    Terminal(Box<PreterminalReplayTerminalStepV1>),
}

struct PreterminalReplayTerminalStepV1 {
    state: Box<PreterminalReplayStateV1>,
    endpoint: Box<ExactCoveredTerminalEndpointV1>,
    step_event: DirectSnowTerminalEventResult,
    physical_child_ordinal: u32,
    deferred_native_v2_soil_custody:
        Option<crate::v9_real_consumer_shadow::DeferredNativeV2SoilCustodyV1>,
}

struct PreparedPreterminalReplayStepV1 {
    projected: DirectSnowStage3V11PreparedSupport,
    endpoint: Box<ExactCoveredTerminalEndpointV1>,
    ending_stage3: BTreeMap<u32, DirectSnowStage3PersistentState>,
    terminal_step: bool,
    step_event: DirectSnowTerminalEventResult,
    physical_child_ordinal: u32,
}

#[allow(clippy::too_many_arguments, clippy::too_many_lines)]
#[inline(never)]
fn prepare_preterminal_replay_step_v1(
    state: &PreterminalReplayStateV1,
    prepared: &DirectSnowStage3V11PreparedSupport,
    current_child_ordinal: u32,
    active_lanes: &BTreeSet<u32>,
    lane_id: &u32,
    carrier_phases_by_joint: &BTreeMap<
        Digest32,
        crate::v9_real_consumer_shadow::CoveredCarrierPhaseResultV1,
    >,
    event: DirectSnowTerminalEventResult,
    step: &crate::hydrology::CoveredTerminalAcceptedMicrostepV1,
) -> Result<Box<PreparedPreterminalReplayStepV1>, DirectSnowStage3V11AttachmentError> {
    let mut carrier_phase = carrier_phases_by_joint
        .get(&step.carrier_ending_joint.receipt_sha256())
        .cloned()
        .ok_or(DirectSnowStage3V11AttachmentError::Terminal(
            "adaptive preterminal accepted carrier phase",
        ))?;
    let physical_child_ordinal = current_child_ordinal
        .checked_add(u32::try_from(state.receipts.len()).map_err(|_| {
            DirectSnowStage3V11AttachmentError::Identity("adaptive terminal receipt ordinal width")
        })?)
        .ok_or(DirectSnowStage3V11AttachmentError::Identity(
            "adaptive terminal receipt ordinal overflow",
        ))?;
    let prior_identity = &carrier_phase.transition.probe_child_identity;
    let wb14_replay_trial_sha256 = prior_identity.receipt_sha256;
    let wb14_replay_beginning_owner_set_sha256 = Digest32::from_bytes(
        crate::direct_runtime::wb14_child_replay_binding(&carrier_phase.wb14_child_replay_bytes)
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
        DirectSnowStage3V11AttachmentError::Terminal("adaptive preterminal snow-soil trial seal")
    })?;
    let mut ending_stage3 = state.stage3.clone();
    ending_stage3.insert(*lane_id, step.ending_state.clone());
    let ending = carrier_phase
        .ending_candidates
        .try_with_selected_stage3_by_lane(
            step.hydrology_ending_joint.clone(),
            ending_stage3.clone(),
        )?;
    let projected = prepared
        .coupled_subslab(step.support, physical_child_ordinal)?
        .retain_active_snow_lanes(active_lanes)?;
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
        forcing_sha256: canonical_stage3_support_forcing_digest(&projected.support_forcing_by_lane),
        ending: ending.clone(),
        carrier_phase: Box::new(carrier_phase.clone()),
        carrier_phase_chain: vec![carrier_phase.clone()],
        wb14_replay_trial_sha256,
        wb14_replay_beginning_owner_set_sha256,
        terminal_snow_soil_trial_receipt: trial_receipt,
        final_child_actual_vapor_to_canopy_air_kg_m2: step.sublimation_kg_m2
            - step.deposition_kg_m2,
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
    Ok(Box::new(PreparedPreterminalReplayStepV1 {
        projected,
        endpoint,
        ending_stage3,
        terminal_step,
        step_event,
        physical_child_ordinal,
    }))
}

#[allow(clippy::too_many_arguments, clippy::too_many_lines)]
#[inline(never)]
fn execute_preterminal_replay_step_v1(
    mut state: Box<PreterminalReplayStateV1>,
    context: &DirectSnowStage3V11StaticContext,
    prepared: &DirectSnowStage3V11PreparedSupport,
    day_index: usize,
    interval_index: usize,
    forcing_receipt: Digest32,
    beginning_terminal_parcels: &BTreeMap<Digest32, DirectSnowStage3V11TerminalParcel>,
    selected_upper_bound_s: f64,
    current_child_ordinal: u32,
    active_lanes: &BTreeSet<u32>,
    lane_id: &u32,
    carrier_phases_by_joint: &BTreeMap<
        Digest32,
        crate::v9_real_consumer_shadow::CoveredCarrierPhaseResultV1,
    >,
    event: DirectSnowTerminalEventResult,
    step: &crate::hydrology::CoveredTerminalAcceptedMicrostepV1,
) -> Result<PreterminalReplayStepOutcomeV1, DirectSnowStage3V11AttachmentError> {
    let prepared_step = prepare_preterminal_replay_step_v1(
        &state,
        prepared,
        current_child_ordinal,
        active_lanes,
        lane_id,
        carrier_phases_by_joint,
        event,
        step,
    )?;
    let PreparedPreterminalReplayStepV1 {
        projected,
        endpoint,
        ending_stage3,
        terminal_step,
        step_event,
        physical_child_ordinal,
    } = *prepared_step;
    let outcome = stage3_boxed_execution_v1(|| {
        execute_covered_real_v11_subslab(
            context,
            &state.parent,
            &state.consumer,
            state.deferred_native_v2_soil_custody.as_ref(),
            &state.clock,
            &projected,
            day_index,
            interval_index,
            forcing_receipt,
            std::mem::take(&mut state.stage3),
            None,
            beginning_terminal_parcels,
            selected_upper_bound_s,
            Some(std::slice::from_ref(&endpoint)),
        )
    })?;
    let (
        next_parent,
        next_consumer,
        next_clock,
        next_stage3,
        receipt,
        deferred_native_v2_soil_custody,
        snow_enthalpy_material_owner,
    ) = *outcome;
    if snow_enthalpy_material_owner.is_some() {
        return Err(DirectSnowStage3V11AttachmentError::Identity(
            "V56 noncrossing compound snow owner entered preterminal replay",
        ));
    }
    if (!terminal_step && !receipt.terminal_events.is_empty())
        || (terminal_step && receipt.terminal_events.get(lane_id) != Some(&step_event))
    {
        return Err(DirectSnowStage3V11AttachmentError::Terminal(
            "adaptive microstep installed event posture",
        ));
    }
    match (
        next_consumer.soil_thermal_resident().read_view(),
        endpoint.carrier_phase.soil_candidate.read_view(),
    ) {
        (
            crate::v9_real_consumer_shadow::DirectSoilThermalReadView::V1(_),
            crate::v9_real_consumer_shadow::DirectSoilThermalReadView::V1(_),
        ) => {
            // The historical V1 endpoint is itself the accepted-owner
            // authority. Preserve its exact whole-map comparison unchanged.
            if endpoint.ending.shadow().canonical_owner_state_bytes()?
                != next_consumer.canonical_owner_state_bytes()?
            {
                return Err(DirectSnowStage3V11AttachmentError::Terminal(
                    "adaptive preterminal carrier-owner installation divergence",
                ));
            }
            if next_stage3 != ending_stage3 {
                return Err(DirectSnowStage3V11AttachmentError::Terminal(
                    "adaptive preterminal snow-owner installation divergence",
                ));
            }
        }
        (
            crate::v9_real_consumer_shadow::DirectSoilThermalReadView::V2(_),
            crate::v9_real_consumer_shadow::DirectSoilThermalReadView::V2(_),
        ) => validate_native_v2_preterminal_installation_v1(
            &next_parent,
            &next_consumer,
            &endpoint,
            &ending_stage3,
            &receipt,
            deferred_native_v2_soil_custody.as_ref(),
        )?,
        _ => {
            return Err(DirectSnowStage3V11AttachmentError::Terminal(
                "adaptive preterminal soil-owner installation divergence",
            ));
        }
    }
    state.parent = next_parent;
    state.consumer = next_consumer;
    state.clock = next_clock;
    state.stage3 = next_stage3;
    state.receipts.push(receipt);
    if state.deferred_native_v2_soil_custody.is_some()
        && deferred_native_v2_soil_custody.is_none()
    {
        return Err(DirectSnowStage3V11AttachmentError::Identity(
            "deferred native V2 soil custody lost during preterminal replay",
        ));
    }
    state.deferred_native_v2_soil_custody = deferred_native_v2_soil_custody.clone();
    if terminal_step {
        Ok(PreterminalReplayStepOutcomeV1::Terminal(Box::new(
            PreterminalReplayTerminalStepV1 {
                state,
                endpoint,
                step_event,
                physical_child_ordinal,
                deferred_native_v2_soil_custody,
            },
        )))
    } else {
        Ok(PreterminalReplayStepOutcomeV1::Continue(state))
    }
}
