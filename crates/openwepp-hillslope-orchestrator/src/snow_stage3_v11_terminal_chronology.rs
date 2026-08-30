/// One independently reproduced lane endpoint proposed to the terminal parent
/// chronology. Candidate discovery is outside this type; construction admits
/// only the exact result of a shortened covered solve.
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct Stage3V11ActualTerminalCandidateV1 {
    pub lane_id: u32,
    pub tick: ModelTimeNs,
    pub support: TimeSupport,
    pub event: DirectSnowTerminalEventResult,
    pub event_result_digest: Digest32,
    pub terminal_state_sha256: Digest32,
    pub shortened_forcing_sha256: Digest32,
    pub shortened_owner_set_sha256: Digest32,
    pub exact_endpoint_receipt_sha256: Option<Digest32>,
    pub terminal_snow_soil_trial_receipt_sha256: Option<Digest32>,
}

pub(crate) fn canonical_terminal_event_result_digest(
    event: &DirectSnowTerminalEventResult,
) -> Result<Digest32, DirectSnowStage3V11AttachmentError> {
    let mut bytes = b"OPENWEPP_STAGE3_TERMINAL_EVENT_RESULT_V1\0".to_vec();
    bytes.push(0); // EnthalpyEventV1
    bytes.push(u8::from(event.event_occurred));
    bytes.extend_from_slice(&u64::try_from(event.hour_index).map_err(|_| DirectSnowStage3V11AttachmentError::Identity("terminal event hour width"))?.to_be_bytes());
    macro_rules! floats { ($($field:ident),+ $(,)?) => { $(bytes.extend_from_slice(&event.$field.to_bits().to_be_bytes());)+ }; }
    floats!(terminal_entry_offset_seconds, requested_seconds, entry_solid_precipitation_kg_m2,
        hour_offset_seconds, evaluated_seconds, unevaluated_seconds, start_ice_kg_m2,
        start_liquid_kg_m2, start_cold_content_j_m2, end_ice_kg_m2, terminal_liquid_kg_m2,
        end_cold_content_j_m2, complete_energy_j_m2, shortwave_energy_j_m2, longwave_energy_j_m2,
        sensible_energy_j_m2, latent_energy_j_m2, advected_energy_j_m2, snow_soil_heat_energy_j_m2,
        external_liquid_kg_m2, cold_energy_change_j_m2, refrozen_kg_m2, deposition_kg_m2,
        sublimation_kg_m2, melt_kg_m2, terminal_unallocated_energy_j_m2,
        solid_mass_closure_residual_kg_m2, liquid_mass_closure_residual_kg_m2,
        energy_closure_residual_j_m2, event_bracket_width_seconds, event_bracket_lower_seconds,
        event_bracket_upper_seconds, event_bracket_lower_solid_kg_m2, event_bracket_upper_solid_kg_m2,
        lte_coarse_ice_kg_m2, lte_fine_ice_kg_m2, lte_coarse_liquid_kg_m2,
        lte_fine_liquid_kg_m2, lte_coarse_cold_content_j_m2, lte_fine_cold_content_j_m2,
        lte_coarse_complete_energy_j_m2, lte_fine_complete_energy_j_m2,
        lte_coarse_unallocated_energy_j_m2, lte_fine_unallocated_energy_j_m2);
    bytes.extend_from_slice(&event.accepted_trials.to_be_bytes());
    bytes.extend_from_slice(&event.rejected_trials.to_be_bytes());
    bytes.extend_from_slice(&event.maximum_scaled_error.to_bits().to_be_bytes());
    Ok(digest_bytes(&bytes))
}

impl Stage3V11ActualTerminalCandidateV1 {
    fn validate(
        &self,
        parent: TimeSupport,
        active_lanes: &BTreeSet<u32>,
    ) -> Result<(), DirectSnowStage3V11AttachmentError> {
        if !active_lanes.contains(&self.lane_id)
            || self.support.start_ns() < parent.start_ns()
            || self.support.end_ns() != self.tick
            || self.tick < parent.start_ns()
            || self.tick > parent.end_ns()
            || !self.event.event_occurred
            || !self.event.hour_offset_seconds.is_finite()
            || !self.event.evaluated_seconds.is_finite()
            || self.event.evaluated_seconds < 0.0
            || self.event.unevaluated_seconds < 0.0
            || canonical_terminal_event_result_digest(&self.event)? != self.event_result_digest
            || self.exact_endpoint_receipt_sha256.is_none()
            || self.terminal_snow_soil_trial_receipt_sha256.is_none()
        {
            return Err(DirectSnowStage3V11AttachmentError::Terminal(
                "actual terminal candidate identity/support",
            ));
        }
        if (self.event.terminal_entry_offset_seconds + self.event.evaluated_seconds).to_bits()
            != self.event.hour_offset_seconds.to_bits()
        {
            return Err(DirectSnowStage3V11AttachmentError::Terminal(
                "actual terminal endpoint relative chronology",
            ));
        }
        let endpoint_ns = quantize_seconds_to_tick(
            ModelTimeNs::new(0),
            ModelTimeNs::new(parent.duration_ns()),
            self.event.hour_offset_seconds,
        )?;
        if endpoint_ns.get() != self.support.duration_ns()
            || self.event.end_ice_kg_m2.abs() > 1.0e-12
        {
            return Err(DirectSnowStage3V11AttachmentError::Terminal(
                "actual terminal endpoint reproduction",
            ));
        }
        Ok(())
    }
}

/// Deterministic event group selected at one common earliest tick. Every lane
/// in this group has independently reproduced the same endpoint using its
/// shortened physical support; lanes with later candidates remain active.
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct Stage3V11TerminalEventGroupV1 {
    pub tick: ModelTimeNs,
    pub ordinal: u64,
    pub terminating_lanes: BTreeSet<u32>,
    pub pre_active_lanes: BTreeSet<u32>,
    pub post_active_lanes: BTreeSet<u32>,
    pub candidates: Vec<Stage3V11ActualTerminalCandidateV1>,
    pub discovery_receipt_sha256: Digest32,
    pub proposal_core_sha256: Option<Digest32>,
    /// Ordered identities of the exact ProducedUnconsumed parcel set sealed
    /// by the accepted terminal physical ledger. These identities remain as
    /// predecessor evidence even when the receiver consumes the parcels
    /// within the same atomic parent support.
    pub produced_unconsumed_parcel_digests: Vec<Digest32>,
    /// Exact typed custody retained even when a parent-end receiver consumes
    /// the parcel before any following positive support can retain it.
    #[serde(default)]
    pub produced_unconsumed_parcels: Vec<Stage3V11TerminalReceiverCustodyV1>,
    pub receipt_sha256: Digest32,
    pub accepted_event_receipt: Option<AcceptedEventReceiptV1>,
    pub accepted_group_receipt_sha256: Option<Digest32>,
    pub terminal_physical_ledger: Option<Stage3V11TerminalPhysicalLedgerV1>,
    /// V3-only durable capacity/routing custody for a parent-end terminal
    /// receiver. Legacy V1/V2 receipt bytes deliberately exclude this
    /// separately sealed supplement.
    #[serde(skip)]
    pub terminal_receiver_custody_v2: Option<Stage3TerminalLiquidCustodyV2>,
}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct Stage3V11TerminalReceiverDestinationCustodyV1 {
    pub destination_ofe_id: String,
    pub destination_tile_id: String,
    pub destination_fraction: f64,
}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct Stage3V11TerminalReceiverCustodyV1 {
    pub support: TimeSupport,
    pub source_lane_id: u32,
    pub parent_transaction_id: Digest32,
    pub event_ordinal: u32,
    pub terminal_event_proposal_core_id: Digest32,
    pub event_result_digest: Digest32,
    pub receiver_topology_sha256: Digest32,
    pub destination_ofe_id: String,
    pub receiver_destinations: Vec<Stage3V11TerminalReceiverDestinationCustodyV1>,
    pub mass_kg_m2_tile_ground: f64,
    pub temperature_k: f64,
    pub specific_liquid_enthalpy_j_kg: f64,
    pub parcel_digest: Digest32,
}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct Stage3V11TerminalPhysicalLedgerV1 {
    pub support: TimeSupport,
    pub event_result_set_sha256: Digest32,
    pub proposal_core_sha256: Digest32,
    pub accepted_event_receipt_sha256: Digest32,
    pub accepted_event_ledger_sha256: Digest32,
    pub produced_unconsumed_parcel_set_sha256: Digest32,
    pub beginning_owner_set_sha256: Digest32,
    pub ending_owner_set_sha256: Digest32,
    pub ending_snow_owner_sha256: Digest32,
    pub evaluated_seconds: f64,
    pub snow_soil_heat_j_m2: f64,
    pub receipt_sha256: Digest32,
}

impl Stage3V11TerminalPhysicalLedgerV1 {
    fn reconstructed_digest(&self) -> Result<Digest32, DirectSnowStage3V11AttachmentError> {
        if self.support.duration_ns() == 0
            || !self.evaluated_seconds.is_finite()
            || self.evaluated_seconds <= 0.0
            || !self.snow_soil_heat_j_m2.is_finite()
            || [
                self.event_result_set_sha256,
                self.proposal_core_sha256,
                self.accepted_event_receipt_sha256,
                self.accepted_event_ledger_sha256,
                self.produced_unconsumed_parcel_set_sha256,
                self.beginning_owner_set_sha256,
                self.ending_owner_set_sha256,
                self.ending_snow_owner_sha256,
            ]
            .contains(&Digest32::zero())
        {
            return Err(DirectSnowStage3V11AttachmentError::Identity(
                "terminal physical ledger domain",
            ));
        }
        Ok(framed_sha256(
            "stage3-v11-terminal-physical-ledger-v1",
            &[
                FramedField { tag: "support_start", value: &self.support.start_ns().get().to_be_bytes() },
                FramedField { tag: "support_end", value: &self.support.end_ns().get().to_be_bytes() },
                FramedField { tag: "event_results", value: self.event_result_set_sha256.as_bytes() },
                FramedField { tag: "proposal_core", value: self.proposal_core_sha256.as_bytes() },
                FramedField { tag: "accepted_event", value: self.accepted_event_receipt_sha256.as_bytes() },
                FramedField { tag: "accepted_event_ledger", value: self.accepted_event_ledger_sha256.as_bytes() },
                FramedField { tag: "parcel_set", value: self.produced_unconsumed_parcel_set_sha256.as_bytes() },
                FramedField { tag: "begin_owner_set", value: self.beginning_owner_set_sha256.as_bytes() },
                FramedField { tag: "end_owner_set", value: self.ending_owner_set_sha256.as_bytes() },
                FramedField { tag: "ending_snow_owner", value: self.ending_snow_owner_sha256.as_bytes() },
                FramedField { tag: "evaluated_seconds", value: &self.evaluated_seconds.to_bits().to_be_bytes() },
                FramedField { tag: "snow_soil_heat", value: &self.snow_soil_heat_j_m2.to_bits().to_be_bytes() },
            ],
        )?)
    }

    pub fn validate(&self) -> Result<(), DirectSnowStage3V11AttachmentError> {
        if self.receipt_sha256 != self.reconstructed_digest()? {
            return Err(DirectSnowStage3V11AttachmentError::Identity(
                "terminal physical ledger seal",
            ));
        }
        Ok(())
    }

    pub(crate) fn seal(mut self) -> Result<Self, DirectSnowStage3V11AttachmentError> {
        self.receipt_sha256 = self.reconstructed_digest()?;
        Ok(self)
    }
}

fn terminal_event_result_set_digest(
    candidates: &[Stage3V11ActualTerminalCandidateV1],
) -> Result<Digest32, DirectSnowStage3V11AttachmentError> {
    let fields = candidates
        .iter()
        .map(|candidate| FramedField {
            tag: "event_result",
            value: candidate.event_result_digest.as_bytes(),
        })
        .collect::<Vec<_>>();
    Ok(framed_sha256("stage3-v11-terminal-event-result-set", &fields)?)
}

pub(crate) fn accepted_terminal_group_digest(
    group: &Stage3V11TerminalEventGroupV1,
) -> Result<Digest32, DirectSnowStage3V11AttachmentError> {
    let accepted = group.accepted_event_receipt.as_ref().ok_or(
        DirectSnowStage3V11AttachmentError::Identity("terminal accepted event missing"),
    )?;
    let ledger = group.terminal_physical_ledger.as_ref().ok_or(
        DirectSnowStage3V11AttachmentError::Identity("terminal physical ledger missing"),
    )?;
    ledger.validate()?;
    if accepted.tick() != group.tick
        || u64::from(accepted.ordinal()) != group.ordinal
        || accepted.event_context_digest() != group.receipt_sha256
        || accepted.id().digest() != ledger.accepted_event_receipt_sha256
        || accepted.ledger_digest() != ledger.accepted_event_ledger_sha256
        || accepted.beginning_owner_set_digest() != ledger.beginning_owner_set_sha256
        || accepted.ending_owner_set_digest() != ledger.ending_owner_set_sha256
        || group.proposal_core_sha256 != Some(ledger.proposal_core_sha256)
        || terminal_event_result_set_digest(&group.candidates)?
            != ledger.event_result_set_sha256
    {
        return Err(DirectSnowStage3V11AttachmentError::Identity(
            "terminal accepted group cross-join",
        ));
    }
    Ok(framed_sha256(
        "stage3-v11-terminal-group-accepted-v2",
        &[
            FramedField { tag: "preaccept", value: group.receipt_sha256.as_bytes() },
            FramedField { tag: "proposal", value: ledger.proposal_core_sha256.as_bytes() },
            FramedField { tag: "accepted_event", value: accepted.id().digest().as_bytes() },
            FramedField { tag: "accepted_event_ledger", value: accepted.ledger_digest().as_bytes() },
            FramedField { tag: "event_results", value: ledger.event_result_set_sha256.as_bytes() },
            FramedField { tag: "parcels", value: ledger.produced_unconsumed_parcel_set_sha256.as_bytes() },
            FramedField { tag: "begin_owner_set", value: ledger.beginning_owner_set_sha256.as_bytes() },
            FramedField { tag: "end_owner_set", value: ledger.ending_owner_set_sha256.as_bytes() },
            FramedField { tag: "ending_snow_owner", value: ledger.ending_snow_owner_sha256.as_bytes() },
            FramedField { tag: "terminal_physical_ledger", value: ledger.receipt_sha256.as_bytes() },
        ],
    )?)
}

fn terminal_event_group_digest(
    parent: TimeSupport,
    tick: ModelTimeNs,
    ordinal: u64,
    pre: &BTreeSet<u32>,
    post: &BTreeSet<u32>,
    candidates: &[Stage3V11ActualTerminalCandidateV1],
) -> Result<Digest32, DirectSnowStage3V11AttachmentError> {
    let mut bytes = Vec::new();
    bytes.extend_from_slice(b"OPENWEPP_STAGE3_V11_TERMINAL_EVENT_GROUP_V1\0");
    bytes.extend_from_slice(&parent.start_ns().get().to_be_bytes());
    bytes.extend_from_slice(&parent.end_ns().get().to_be_bytes());
    bytes.extend_from_slice(&tick.get().to_be_bytes());
    bytes.extend_from_slice(&ordinal.to_be_bytes());
    for lanes in [pre, post] {
        let count = u32::try_from(lanes.len()).map_err(|_| {
            DirectSnowStage3V11AttachmentError::Identity("terminal participant count")
        })?;
        bytes.extend_from_slice(&count.to_be_bytes());
        for lane in lanes {
            bytes.extend_from_slice(&lane.to_be_bytes());
        }
    }
    for candidate in candidates {
        bytes.extend_from_slice(&candidate.lane_id.to_be_bytes());
        bytes.extend_from_slice(candidate.event_result_digest.as_bytes());
        bytes.extend_from_slice(candidate.terminal_state_sha256.as_bytes());
        bytes.extend_from_slice(candidate.shortened_forcing_sha256.as_bytes());
        bytes.extend_from_slice(candidate.shortened_owner_set_sha256.as_bytes());
        bytes.extend_from_slice(
            candidate
                .exact_endpoint_receipt_sha256
                .ok_or(DirectSnowStage3V11AttachmentError::Terminal(
                    "terminal group exact endpoint receipt",
                ))?
                .as_bytes(),
        );
        bytes.extend_from_slice(
            candidate
                .terminal_snow_soil_trial_receipt_sha256
                .ok_or(DirectSnowStage3V11AttachmentError::Terminal(
                    "terminal group snow-soil trial receipt",
                ))?
                .as_bytes(),
        );
    }
    Ok(openwepp_coupled_time::digest_bytes(&bytes))
}

/// Select one coalesced common-earliest group without changing owners. This is
/// deliberately a pure post-solve operation, so it cannot feed a candidate
/// tick or receipt back into any constitutive equation.
pub fn select_common_earliest_actual_terminal_group_v1(
    parent: TimeSupport,
    event_ordinal: u64,
    active_lanes: &BTreeSet<u32>,
    candidates: Vec<Stage3V11ActualTerminalCandidateV1>,
) -> Result<Option<Stage3V11TerminalEventGroupV1>, DirectSnowStage3V11AttachmentError> {
    if active_lanes.is_empty() {
        return Ok(None);
    }
    let mut by_lane = BTreeMap::new();
    for candidate in candidates {
        candidate.validate(parent, active_lanes)?;
        if by_lane.insert(candidate.lane_id, candidate).is_some() {
            return Err(DirectSnowStage3V11AttachmentError::Terminal(
                "duplicate terminal lane candidate",
            ));
        }
    }
    if by_lane
        .values()
        .map(|candidate| candidate.support.start_ns())
        .collect::<BTreeSet<_>>()
        .len()
        > 1
    {
        return Err(DirectSnowStage3V11AttachmentError::Terminal(
            "terminal candidates do not share the current search cursor",
        ));
    }
    let Some(earliest) = by_lane.values().map(|value| value.tick).min() else {
        return Ok(None);
    };
    let selected = by_lane
        .into_values()
        .filter(|value| value.tick == earliest)
        .collect::<Vec<_>>();
    let terminating_lanes = selected
        .iter()
        .map(|value| value.lane_id)
        .collect::<BTreeSet<_>>();
    let post_active_lanes = active_lanes
        .difference(&terminating_lanes)
        .copied()
        .collect::<BTreeSet<_>>();
    if terminating_lanes.is_empty() || !terminating_lanes.is_subset(active_lanes) {
        return Err(DirectSnowStage3V11AttachmentError::Terminal(
            "terminal participant transition",
        ));
    }
    let receipt_sha256 = terminal_event_group_digest(
        parent,
        earliest,
        event_ordinal,
        active_lanes,
        &post_active_lanes,
        &selected,
    )?;
    Ok(Some(Stage3V11TerminalEventGroupV1 {
        tick: earliest,
        ordinal: event_ordinal,
        terminating_lanes,
        pre_active_lanes: active_lanes.clone(),
        post_active_lanes,
        candidates: selected,
        discovery_receipt_sha256: receipt_sha256,
        proposal_core_sha256: None,
        produced_unconsumed_parcel_digests: Vec::new(),
        produced_unconsumed_parcels: Vec::new(),
        receipt_sha256,
        accepted_event_receipt: None,
        accepted_group_receipt_sha256: None,
        terminal_physical_ledger: None,
        terminal_receiver_custody_v2: None,
    }))
}
