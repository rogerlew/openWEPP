#[derive(Clone, Debug, Eq, PartialEq)]
pub(crate) struct RealDiscreteCompleteEndpointEvidenceV1 {
    pub support: TimeSupport,
    pub selected_upper_bound_s_bits: u64,
    pub lane_id: u32,
    pub owner_count: usize,
    pub start_ice_bits: u64,
    pub start_liquid_bits: u64,
    pub start_cold_content_bits: u64,
    pub end_ice_bits: u64,
    pub end_liquid_bits: u64,
    pub event_occurred: bool,
    pub event_evaluated_seconds_bits: u64,
    pub event_hour_offset_seconds_bits: u64,
    pub event_unevaluated_seconds_bits: u64,
    pub deposition_bits: u64,
    pub sublimation_bits: u64,
    pub melt_bits: u64,
    pub refrozen_bits: u64,
    pub external_liquid_bits: u64,
    pub complete_energy_bits: u64,
    pub latent_energy_bits: u64,
    pub terminal_unallocated_energy_bits: u64,
    pub energy_closure_residual_bits: u64,
    pub ice_closure_residual_bits: u64,
    pub water_closure_residual_bits: u64,
    pub snow_soil_receipt_sha256: Digest32,
    pub ending_joint_sha256: Digest32,
    pub canonical_bytes: Vec<u8>,
}

#[cfg(test)]
impl RealDiscreteCompleteEndpointEvidenceV1 {
    pub(crate) fn validate_evidence(&self) -> Result<(), &'static str> {
        let finite_nonnegative = |bits| {
            let value = f64::from_bits(bits);
            value.is_finite() && value >= 0.0
        };
        let finite_within = |bits, tolerance: f64| {
            let value = f64::from_bits(bits);
            value.is_finite() && value.abs() <= tolerance
        };
        if self.support.duration_ns() < crate::discrete_terminal_support_root::MINIMUM_TERMINAL_SUPPORT_NS
            || !matches!(
                self.selected_upper_bound_s_bits,
                value if value == 60.0_f64.to_bits()
                    || value == 900.0_f64.to_bits()
                    || value == 1_800.0_f64.to_bits()
            )
            || f64::from_bits(self.support.duration_s_bits())
                > f64::from_bits(self.selected_upper_bound_s_bits)
            || self.owner_count != 7
            || self.snow_soil_receipt_sha256 == Digest32::zero()
            || self.ending_joint_sha256 == Digest32::zero()
            || !finite_nonnegative(self.start_ice_bits)
            || !finite_nonnegative(self.start_liquid_bits)
            || !finite_nonnegative(self.start_cold_content_bits)
            || !finite_nonnegative(self.end_ice_bits)
            || !finite_nonnegative(self.end_liquid_bits)
            || !finite_nonnegative(self.deposition_bits)
            || !finite_nonnegative(self.sublimation_bits)
            || !finite_nonnegative(self.melt_bits)
            || !finite_nonnegative(self.refrozen_bits)
            || !finite_nonnegative(self.external_liquid_bits)
            || !f64::from_bits(self.complete_energy_bits).is_finite()
            || !f64::from_bits(self.latent_energy_bits).is_finite()
            || !finite_nonnegative(self.event_evaluated_seconds_bits)
            || !finite_nonnegative(self.event_hour_offset_seconds_bits)
            || !finite_nonnegative(self.event_unevaluated_seconds_bits)
            || !finite_nonnegative(self.terminal_unallocated_energy_bits)
            || !finite_within(self.energy_closure_residual_bits, 1.0e-6)
            || !finite_within(self.ice_closure_residual_bits, 1.0e-9)
            || !finite_within(self.water_closure_residual_bits, 1.0e-9)
            || self.canonical_bytes.is_empty()
        {
            return Err("incomplete real discrete endpoint");
        }
        Ok(())
    }
}

#[cfg(test)]
impl crate::discrete_terminal_support_root::CompleteEndpointCandidate
    for RealDiscreteCompleteEndpointEvidenceV1
{
    fn validate_complete(&self) -> Result<(), &'static str> {
        self.validate_evidence()
    }

    fn canonical_bytes(&self) -> &[u8] {
        &self.canonical_bytes
    }
}

#[cfg(test)]
#[allow(clippy::too_many_arguments, clippy::too_many_lines)]
pub(crate) fn evaluate_real_discrete_complete_endpoint_v1(
    beginning_consumer: &DirectV10RealConsumerShadow,
    beginning_clock: &CoupledClockStateV1,
    prepared_parent: &DirectSnowStage3V11PreparedSupport,
    day_index: usize,
    interval_index: usize,
    beginning_stage3: &BTreeMap<u32, DirectSnowStage3PersistentState>,
    beginning_terminal_parcels: &BTreeMap<Digest32, DirectSnowStage3V11TerminalParcel>,
    selected_upper_bound_s: f64,
    current_child_ordinal: u32,
    lane_id: u32,
    endpoint_tick: ModelTimeNs,
) -> Result<RealDiscreteCompleteEndpointEvidenceV1, DirectSnowStage3V11AttachmentError> {
    evaluate_real_complete_endpoint_with_phase_mode_v1(
        beginning_consumer,
        beginning_clock,
        prepared_parent,
        day_index,
        interval_index,
        beginning_stage3,
        beginning_terminal_parcels,
        selected_upper_bound_s,
        current_child_ordinal,
        lane_id,
        endpoint_tick,
        CoveredTerminalExecutionMode::DirectStepTrial,
    )
}

#[allow(clippy::too_many_arguments, clippy::too_many_lines)]
pub(crate) fn evaluate_real_phase_complementarity_endpoint_v1(
    beginning_consumer: &DirectV10RealConsumerShadow,
    beginning_clock: &CoupledClockStateV1,
    prepared_parent: &DirectSnowStage3V11PreparedSupport,
    day_index: usize,
    interval_index: usize,
    beginning_stage3: &BTreeMap<u32, DirectSnowStage3PersistentState>,
    beginning_terminal_parcels: &BTreeMap<Digest32, DirectSnowStage3V11TerminalParcel>,
    selected_upper_bound_s: f64,
    current_child_ordinal: u32,
    lane_id: u32,
    endpoint_tick: ModelTimeNs,
) -> Result<RealDiscreteCompleteEndpointEvidenceV1, DirectSnowStage3V11AttachmentError> {
    evaluate_real_complete_endpoint_with_phase_mode_v1(
        beginning_consumer,
        beginning_clock,
        prepared_parent,
        day_index,
        interval_index,
        beginning_stage3,
        beginning_terminal_parcels,
        selected_upper_bound_s,
        current_child_ordinal,
        lane_id,
        endpoint_tick,
        CoveredTerminalExecutionMode::PhaseComplementarityEndpoint,
    )
}

#[allow(clippy::too_many_arguments, clippy::too_many_lines)]
fn evaluate_real_complete_endpoint_with_phase_mode_v1(
    beginning_consumer: &DirectV10RealConsumerShadow,
    beginning_clock: &CoupledClockStateV1,
    prepared_parent: &DirectSnowStage3V11PreparedSupport,
    day_index: usize,
    interval_index: usize,
    beginning_stage3: &BTreeMap<u32, DirectSnowStage3PersistentState>,
    beginning_terminal_parcels: &BTreeMap<Digest32, DirectSnowStage3V11TerminalParcel>,
    selected_upper_bound_s: f64,
    current_child_ordinal: u32,
    lane_id: u32,
    endpoint_tick: ModelTimeNs,
    mode: CoveredTerminalExecutionMode,
) -> Result<RealDiscreteCompleteEndpointEvidenceV1, DirectSnowStage3V11AttachmentError> {
    let support = TimeSupport::new(prepared_parent.support.start_ns(), endpoint_tick)?;
    let prepared = prepared_parent.coupled_subslab(support, current_child_ordinal)?;
    let (result, candidates_by_joint, carrier_phases_by_joint) =
        evaluate_covered_terminal_candidate_v1(
            beginning_consumer,
            beginning_clock,
            &prepared,
            day_index,
            interval_index,
            beginning_stage3,
            beginning_terminal_parcels,
            selected_upper_bound_s,
            current_child_ordinal,
            lane_id,
            mode,
        )?;
    if candidates_by_joint.is_empty() || carrier_phases_by_joint.len() != 1 {
        return Err(DirectSnowStage3V11AttachmentError::Terminal(
            "discrete endpoint carrier cardinality",
        ));
    }
    let phase = carrier_phases_by_joint
        .values()
        .next()
        .ok_or(DirectSnowStage3V11AttachmentError::Terminal(
            "discrete endpoint carrier candidate",
        ))?;
    let receipt = phase
        .ending_candidates
        .terminal_snow_soil_trial_receipt()
        .ok_or(DirectSnowStage3V11AttachmentError::Terminal(
            "discrete endpoint snow-soil receipt",
        ))?;
    receipt.validate().map_err(|_| {
        DirectSnowStage3V11AttachmentError::Terminal(
            "discrete endpoint snow-soil receipt validation",
        )
    })?;
    let ending = bind_selected_terminal_candidate_v1(lane_id, &result, &candidates_by_joint)?;
    let ending_stage3 = ending.stage3_by_lane();
    let mut owners = ending.shadow().canonical_owner_state_bytes()?;
    owners.insert(
        "snow".to_owned(),
        canonical_stage3_snow_owner_bytes_with_pending(ending_stage3, beginning_terminal_parcels)?,
    );
    if owners.len() != 7 {
        return Err(DirectSnowStage3V11AttachmentError::Terminal(
            "discrete endpoint complete owner cardinality",
        ));
    }
    let ending_joint_sha256 = ending.joint().receipt_sha256();
    let terminal_event = result.terminal_event.as_ref().ok_or(
        DirectSnowStage3V11AttachmentError::Terminal(
            "discrete endpoint terminal-event evidence",
        ),
    )?;
    let ending_liquid_kg_m2 = if terminal_event.event_occurred {
        terminal_event.terminal_liquid_kg_m2
    } else {
        result.end_retained_liquid_kg_m2
    };
    let mut canonical_bytes = b"OPENWEPP_REAL_DISCRETE_COMPLETE_ENDPOINT_V1".to_vec();
    canonical_bytes.push(match mode {
        CoveredTerminalExecutionMode::DirectStepTrial => 0,
        CoveredTerminalExecutionMode::PhaseComplementarityEndpoint => 1,
        _ => {
            return Err(DirectSnowStage3V11AttachmentError::Terminal(
                "discrete endpoint phase mode",
            ));
        }
    });
    canonical_bytes.extend_from_slice(&support.start_ns().get().to_be_bytes());
    canonical_bytes.extend_from_slice(&support.end_ns().get().to_be_bytes());
    let selected_upper_bound_s_bits = support.duration_s_bits();
    canonical_bytes.extend_from_slice(&selected_upper_bound_s_bits.to_be_bytes());
    canonical_bytes.extend_from_slice(&lane_id.to_be_bytes());
    for (owner_id, bytes) in &owners {
        canonical_bytes.extend_from_slice(&(owner_id.len() as u32).to_be_bytes());
        canonical_bytes.extend_from_slice(owner_id.as_bytes());
        canonical_bytes.extend_from_slice(&(bytes.len() as u32).to_be_bytes());
        canonical_bytes.extend_from_slice(bytes);
    }
    for value in [
        terminal_event.start_ice_kg_m2,
        terminal_event.start_liquid_kg_m2,
        terminal_event.start_cold_content_j_m2,
        result.end_ice_kg_m2,
        ending_liquid_kg_m2,
        terminal_event.deposition_kg_m2,
        terminal_event.sublimation_kg_m2,
        terminal_event.melt_kg_m2,
        terminal_event.refrozen_kg_m2,
        terminal_event.external_liquid_kg_m2,
        terminal_event.complete_energy_j_m2,
        terminal_event.latent_energy_j_m2,
        terminal_event.evaluated_seconds,
        terminal_event.hour_offset_seconds,
        terminal_event.unevaluated_seconds,
        result.terminal_unallocated_energy_j_m2,
        terminal_event.energy_closure_residual_j_m2,
        result.ice_mass_closure_residual_kg_m2,
        result.total_water_closure_residual_kg_m2,
    ] {
        canonical_bytes.extend_from_slice(&value.to_bits().to_be_bytes());
    }
    canonical_bytes.extend_from_slice(receipt.receipt_sha256.as_bytes());
    canonical_bytes.extend_from_slice(ending_joint_sha256.as_bytes());
    let evidence = RealDiscreteCompleteEndpointEvidenceV1 {
        support,
        selected_upper_bound_s_bits,
        lane_id,
        owner_count: owners.len(),
        start_ice_bits: terminal_event.start_ice_kg_m2.to_bits(),
        start_liquid_bits: terminal_event.start_liquid_kg_m2.to_bits(),
        start_cold_content_bits: terminal_event.start_cold_content_j_m2.to_bits(),
        end_ice_bits: result.end_ice_kg_m2.to_bits(),
        end_liquid_bits: ending_liquid_kg_m2.to_bits(),
        event_occurred: terminal_event.event_occurred,
        event_evaluated_seconds_bits: terminal_event.evaluated_seconds.to_bits(),
        event_hour_offset_seconds_bits: terminal_event.hour_offset_seconds.to_bits(),
        event_unevaluated_seconds_bits: terminal_event.unevaluated_seconds.to_bits(),
        deposition_bits: terminal_event.deposition_kg_m2.to_bits(),
        sublimation_bits: terminal_event.sublimation_kg_m2.to_bits(),
        melt_bits: terminal_event.melt_kg_m2.to_bits(),
        refrozen_bits: terminal_event.refrozen_kg_m2.to_bits(),
        external_liquid_bits: terminal_event.external_liquid_kg_m2.to_bits(),
        complete_energy_bits: terminal_event.complete_energy_j_m2.to_bits(),
        latent_energy_bits: terminal_event.latent_energy_j_m2.to_bits(),
        terminal_unallocated_energy_bits: result.terminal_unallocated_energy_j_m2.to_bits(),
        energy_closure_residual_bits: terminal_event.energy_closure_residual_j_m2.to_bits(),
        ice_closure_residual_bits: result.ice_mass_closure_residual_kg_m2.to_bits(),
        water_closure_residual_bits: result.total_water_closure_residual_kg_m2.to_bits(),
        snow_soil_receipt_sha256: receipt.receipt_sha256,
        ending_joint_sha256,
        canonical_bytes,
    };
    evidence.validate_evidence().map_err(|_| {
        DirectSnowStage3V11AttachmentError::Terminal(
            "discrete endpoint complete evidence validation",
        )
    })?;
    Ok(evidence)
}
