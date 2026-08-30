// Restart-only access to the constitutive attachment's atomic installation
// boundary. Wire ownership stays in `openwepp-persisted-restart-v1`; this seam
// exposes no raw selector and cannot install an unvalidated pending candidate.

/// Exact scheduler-visible interruption points supported by the adaptive
/// Stage-3/V11 restart state machine.
#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum DirectSnowStage3V11InterruptionPostureV2 {
    AdaptiveMicrostepBoundary,
    BeforeTerminalEvent,
    AfterTerminalEvent,
    BeforeTerminalReceiver,
    AfterTerminalReceiver,
    BeforeSnowReappearance,
    AfterSnowReappearance,
}

fn validate_restart_adaptive_trial_grid_v2(
    posture: DirectSnowStage3V11InterruptionPostureV2,
    adaptive_trial_quanta: u128,
    minimum_support_ns: u128,
    parent_support: TimeSupport,
    accepted_until: ModelTimeNs,
) -> Result<(), DirectSnowStage3V11AttachmentError> {
    if minimum_support_ns != STAGE3_V11_ADAPTIVE_MINIMUM_SUPPORT_NS || adaptive_trial_quanta == 0 {
        return Err(DirectSnowStage3V11AttachmentError::Identity(
            "restart adaptive trial minimum support or count",
        ));
    }
    let parent_start = parent_support.start_ns().get();
    let parent_end = parent_support.end_ns().get();
    let cursor = accepted_until.get();
    if cursor < parent_start || cursor > parent_end {
        return Err(DirectSnowStage3V11AttachmentError::Identity(
            "restart adaptive trial cursor range",
        ));
    }
    let parent_duration = parent_end - parent_start;
    let cursor_offset = cursor - parent_start;
    let parent_remainder = parent_end - cursor;
    if parent_duration % minimum_support_ns != 0
        || cursor_offset % minimum_support_ns != 0
        || parent_remainder % minimum_support_ns != 0
    {
        return Err(DirectSnowStage3V11AttachmentError::Identity(
            "restart adaptive trial support grid",
        ));
    }
    let trial_duration = adaptive_trial_quanta
        .checked_mul(minimum_support_ns)
        .ok_or(DirectSnowStage3V11AttachmentError::Identity(
            "restart adaptive trial duration overflow",
        ))?;
    if trial_duration > parent_duration {
        return Err(DirectSnowStage3V11AttachmentError::Identity(
            "restart adaptive trial parent range",
        ));
    }
    // Before a trial, and at an accepted adaptive boundary, the persisted
    // count is the next proposal. Event/receiver postures instead retain the
    // just-executed proposal, so a parent-end zero-duration handoff can have no
    // positive remainder while still carrying an authenticated nonzero count.
    if matches!(
        posture,
        DirectSnowStage3V11InterruptionPostureV2::AdaptiveMicrostepBoundary
            | DirectSnowStage3V11InterruptionPostureV2::BeforeTerminalEvent
            | DirectSnowStage3V11InterruptionPostureV2::BeforeSnowReappearance
            | DirectSnowStage3V11InterruptionPostureV2::AfterSnowReappearance
    ) && (parent_remainder == 0 || trial_duration > parent_remainder)
    {
        return Err(DirectSnowStage3V11AttachmentError::Identity(
            "restart adaptive trial parent remainder",
        ));
    }
    Ok(())
}

#[cfg(test)]
mod restart_adaptive_trial_grid_tests {
    use super::*;

    const MINIMUM_SUPPORT_NS: u128 = 60_000_000_000;
    const PARENT_END_NS: u128 = 1_800_000_000_000;

    fn parent_support() -> TimeSupport {
        TimeSupport::new(ModelTimeNs::new(0), ModelTimeNs::new(PARENT_END_NS)).unwrap()
    }

    #[test]
    fn exact_floor_count_and_cursor_cross_join_rejects_poisons() {
        let parent = parent_support();
        let cursor = ModelTimeNs::new(600_000_000_000);
        validate_restart_adaptive_trial_grid_v2(
            DirectSnowStage3V11InterruptionPostureV2::AdaptiveMicrostepBoundary,
            20,
            MINIMUM_SUPPORT_NS,
            parent,
            cursor,
        )
        .unwrap();

        // Count poison.
        assert!(
            validate_restart_adaptive_trial_grid_v2(
                DirectSnowStage3V11InterruptionPostureV2::AdaptiveMicrostepBoundary,
                0,
                MINIMUM_SUPPORT_NS,
                parent,
                cursor,
            )
            .is_err()
        );
        // Minimum-support substitution poison: the previous 600-ms authority
        // cannot be interpreted as a count on the exact 60-second grid.
        assert!(
            validate_restart_adaptive_trial_grid_v2(
                DirectSnowStage3V11InterruptionPostureV2::AdaptiveMicrostepBoundary,
                20,
                600_000_000,
                parent,
                cursor,
            )
            .is_err()
        );
        // Cursor divisibility poison.
        assert!(
            validate_restart_adaptive_trial_grid_v2(
                DirectSnowStage3V11InterruptionPostureV2::AdaptiveMicrostepBoundary,
                20,
                MINIMUM_SUPPORT_NS,
                parent,
                ModelTimeNs::new(cursor.get() + 1),
            )
            .is_err()
        );
        // Proposal range poison: 21 quanta do not fit the 20-quanta parent
        // remainder at this cursor.
        assert!(
            validate_restart_adaptive_trial_grid_v2(
                DirectSnowStage3V11InterruptionPostureV2::AdaptiveMicrostepBoundary,
                21,
                MINIMUM_SUPPORT_NS,
                parent,
                cursor,
            )
            .is_err()
        );
        // Parent range poison remains invalid even for a posture retaining the
        // just-executed proposal rather than a next proposal.
        assert!(
            validate_restart_adaptive_trial_grid_v2(
                DirectSnowStage3V11InterruptionPostureV2::AfterTerminalEvent,
                31,
                MINIMUM_SUPPORT_NS,
                parent,
                cursor,
            )
            .is_err()
        );
    }

    #[test]
    fn endpoint_event_posture_retains_nonzero_completed_trial_count() {
        validate_restart_adaptive_trial_grid_v2(
            DirectSnowStage3V11InterruptionPostureV2::AfterTerminalReceiver,
            30,
            MINIMUM_SUPPORT_NS,
            parent_support(),
            ModelTimeNs::new(PARENT_END_NS),
        )
        .unwrap();
    }
}

enum PreparedDayExecutionOutcomeV2 {
    Complete(DirectSnowStage3V11ParentCandidate),
    Paused(Box<DirectSnowStage3V11InProgressExecutionV2>),
}

/// Durable in-progress owner state. The immutable prepared day is supplied
/// again on resume and is admitted only after all support identities match.
#[derive(Clone, Debug, PartialEq)]
pub struct DirectSnowStage3V11InProgressExecutionV2 {
    day_candidate: DirectSnowStage3V11CommittedState,
    support_current: Option<DirectSnowStage3V11CommittedState>,
    day_index: usize,
    support_index: usize,
    prepared_supports: Vec<DirectSnowStage3V11PreparedSupportRestartV2>,
    terminal_events: Vec<DirectSnowStage3V11TerminalReceipt>,
    terminal_event_groups: Vec<Stage3V11TerminalEventGroupV1>,
    covered_owner_joins: Vec<CoveredParentOwnerJoinReceiptV1>,
    coupled_subslabs: Vec<Stage3CoupledSubslabReceiptV1>,
    adaptive_support_receipts: Vec<Stage3AdaptiveSupportReceiptV1>,
    snow_free_successor_receipts: Vec<Stage3SnowFreeSuccessorReceiptV1>,
    posture: DirectSnowStage3V11InterruptionPostureV2,
    support_owner_joins: Vec<Stage3CoupledSubslabReceiptV1>,
    support_event_groups: Vec<Stage3V11TerminalEventGroupV1>,
    support_terminal_parcels: Vec<DirectSnowStage3V11TerminalParcel>,
    expected_child_beginning: Digest32,
    pending_adaptive_request: Option<Stage3AdaptiveParentRequestReceiptV1>,
    adaptive_receipts: AdaptiveReceiptAccumulatorV1,
    support_snow_free_successor_receipts: Vec<Stage3SnowFreeSuccessorReceiptV1>,
    adaptive_trial_quanta: u128,
}

impl DirectSnowStage3V11InProgressExecutionV2 {
    #[must_use]
    pub const fn posture(&self) -> DirectSnowStage3V11InterruptionPostureV2 {
        self.posture
    }

    #[must_use]
    pub const fn day_candidate(&self) -> &DirectSnowStage3V11CommittedState {
        &self.day_candidate
    }

    #[must_use]
    pub const fn support_current(&self) -> Option<&DirectSnowStage3V11CommittedState> {
        self.support_current.as_ref()
    }

    /// Admit the one posture where the coupled clock has accepted the
    /// zero-duration solid-reappearance owner transition but publication must
    /// remain at its predecessor until the following positive support is
    /// accepted atomically with that event.
    #[cfg(feature = "persisted-restart-v1")]
    pub fn restart_authority_validate_after_snow_reappearance_publication_v2(
        &self,
    ) -> Result<(), DirectSnowStage3V11AttachmentError> {
        let current =
            self.support_current
                .as_ref()
                .ok_or(DirectSnowStage3V11AttachmentError::Identity(
                    "restart reappearance current support owner",
                ))?;
        let prepared = self.prepared_supports.get(self.support_index).ok_or(
            DirectSnowStage3V11AttachmentError::Identity("restart reappearance prepared support"),
        )?;
        let publication_predecessor = current
            .real_consumer
            .restart_authority_accepted_publication_traversed_ending_owner_sha256()
            .map_err(|_| {
                DirectSnowStage3V11AttachmentError::Identity(
                    "restart reappearance publication predecessor",
                )
            })?
            .ok_or(DirectSnowStage3V11AttachmentError::Identity(
                "restart reappearance missing publication predecessor",
            ))?;
        let events = current.coupled_clock.accepted_event_receipts();
        if self.posture != DirectSnowStage3V11InterruptionPostureV2::AfterSnowReappearance
            || self.pending_adaptive_request.is_some()
            || events.len() != 1
            || current.coupled_clock.parent_support() != prepared.support
            || current.coupled_clock.accepted_until() != prepared.support.start_ns()
        {
            return Err(DirectSnowStage3V11AttachmentError::Identity(
                "restart reappearance deferred publication posture",
            ));
        }
        let event = &events[0];
        event.validate()?;
        if event.ordinal() != 0
            || event.tick() != prepared.support.start_ns()
            || event.parent_transaction_id() != current.coupled_clock.parent_transaction_id()
            || event.beginning_owner_set_digest() != publication_predecessor
            || event.ending_owner_set_digest()
                != complete_owner_set_digest(current.coupled_clock.owners())?
        {
            return Err(DirectSnowStage3V11AttachmentError::Identity(
                "restart reappearance deferred publication event chain",
            ));
        }
        Ok(())
    }
}

#[cfg(feature = "persisted-restart-v1")]
#[derive(Clone, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
struct DirectSnowStage3V11InProgressMetadataWireV2 {
    schema: String,
    version: u16,
    day_index: usize,
    support_index: usize,
    prepared_supports: Vec<DirectSnowStage3V11PreparedSupportRestartV2>,
    terminal_events: Vec<DirectSnowStage3V11TerminalReceipt>,
    terminal_event_groups: Vec<Stage3V11TerminalEventGroupV1>,
    covered_owner_joins: Vec<CoveredParentOwnerJoinReceiptV1>,
    coupled_subslabs: Vec<Stage3CoupledSubslabReceiptWireV2>,
    adaptive_support_receipts: Vec<Stage3AdaptiveSupportReceiptV1>,
    snow_free_successor_receipts: Vec<Stage3SnowFreeSuccessorReceiptV1>,
    posture: DirectSnowStage3V11InterruptionPostureV2,
    support_owner_joins: Vec<Stage3CoupledSubslabReceiptWireV2>,
    support_event_groups: Vec<Stage3V11TerminalEventGroupV1>,
    support_terminal_parcels: Vec<DirectSnowStage3V11TerminalParcel>,
    expected_child_beginning: Digest32,
    pending_adaptive_request: Option<Stage3AdaptiveParentRequestReceiptV1>,
    adaptive_receipts: AdaptiveReceiptAccumulatorV1,
    support_snow_free_successor_receipts: Vec<Stage3SnowFreeSuccessorReceiptV1>,
    adaptive_trial_quanta: String,
}

#[cfg(feature = "persisted-restart-v1")]
#[derive(Clone, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
struct Stage3DestinationBoundaryReceiptWireV2 {
    ofe_id: OfeId,
    tile_id: TileId,
    receipt: FinalStage3TileBoundaryReceiptV1,
}

#[cfg(feature = "persisted-restart-v1")]
#[derive(Clone, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
struct Stage3CoupledSubslabReceiptWireV2 {
    parent_support: TimeSupport,
    support: TimeSupport,
    selected_upper_bound_s_bits: u64,
    accepted_slab_sha256: Digest32,
    wb14_replay_trial_sha256: Digest32,
    wb14_replay_beginning_owner_set_sha256: Digest32,
    wb14_child_receipt_set_sha256: Digest32,
    wb14_parent_receipt_set_sha256: Option<Digest32>,
    wb14_child_replay_bytes: Vec<u8>,
    wb14_parent_replay_bytes: Option<Vec<u8>>,
    destination_receipts: Vec<Stage3DestinationBoundaryReceiptWireV2>,
    lane_receipts: BTreeMap<u32, LaneStage3BoundaryReceiptV1>,
    physical_outcome_ledger_set_sha256: Digest32,
    terminal_events: BTreeMap<u32, DirectSnowTerminalEventResult>,
    post_support_liquid_receiver_event: Option<AcceptedEventReceiptV1>,
    post_support_liquid_output_set_sha256: Option<Digest32>,
    post_support_liquid_mass_kg_m2_bits: Option<u64>,
    post_support_liquid_enthalpy_j_m2_bits: Option<u64>,
    post_support_liquid_surface_beginning_state: Option<crate::DirectSurfaceLiquidOwnedState>,
    post_support_liquid_surface_ending_state: Option<crate::DirectSurfaceLiquidOwnedState>,
    owner_join: CoveredParentOwnerJoinReceiptV1,
    receipt_sha256: Digest32,
}

#[cfg(feature = "persisted-restart-v1")]
impl Stage3CoupledSubslabReceiptWireV2 {
    fn project(value: &Stage3CoupledSubslabReceiptV1) -> Self {
        Self {
            parent_support: value.parent_support,
            support: value.support,
            selected_upper_bound_s_bits: value.selected_upper_bound_s_bits,
            accepted_slab_sha256: value.accepted_slab_sha256,
            wb14_replay_trial_sha256: value.wb14_replay_trial_sha256,
            wb14_replay_beginning_owner_set_sha256: value.wb14_replay_beginning_owner_set_sha256,
            wb14_child_receipt_set_sha256: value.wb14_child_receipt_set_sha256,
            wb14_parent_receipt_set_sha256: value.wb14_parent_receipt_set_sha256,
            wb14_child_replay_bytes: value.wb14_child_replay_bytes.clone(),
            wb14_parent_replay_bytes: value.wb14_parent_replay_bytes.clone(),
            destination_receipts: value
                .destination_receipts
                .iter()
                .map(
                    |((ofe_id, tile_id), receipt)| Stage3DestinationBoundaryReceiptWireV2 {
                        ofe_id: ofe_id.clone(),
                        tile_id: tile_id.clone(),
                        receipt: receipt.clone(),
                    },
                )
                .collect(),
            lane_receipts: value.lane_receipts.clone(),
            physical_outcome_ledger_set_sha256: value.physical_outcome_ledger_set_sha256,
            terminal_events: value.terminal_events.clone(),
            post_support_liquid_receiver_event: value.post_support_liquid_receiver_event.clone(),
            post_support_liquid_output_set_sha256: value.post_support_liquid_output_set_sha256,
            post_support_liquid_mass_kg_m2_bits: value.post_support_liquid_mass_kg_m2_bits,
            post_support_liquid_enthalpy_j_m2_bits: value.post_support_liquid_enthalpy_j_m2_bits,
            post_support_liquid_surface_beginning_state: value
                .post_support_liquid_surface_beginning_state
                .clone(),
            post_support_liquid_surface_ending_state: value
                .post_support_liquid_surface_ending_state
                .clone(),
            owner_join: value.owner_join.clone(),
            receipt_sha256: value.receipt_sha256,
        }
    }

    #[cfg(feature = "persisted-restart-v1")]
    fn restore(self) -> Result<Stage3CoupledSubslabReceiptV1, DirectSnowStage3V11AttachmentError> {
        let mut destination_receipts = BTreeMap::new();
        let mut previous = None;
        for row in self.destination_receipts {
            let key = (row.ofe_id, row.tile_id);
            if previous.as_ref().is_some_and(|value| value >= &key)
                || destination_receipts
                    .insert(key.clone(), row.receipt)
                    .is_some()
            {
                return Err(DirectSnowStage3V11AttachmentError::Identity(
                    "restart subslab destination receipt order",
                ));
            }
            previous = Some(key);
        }
        Ok(Stage3CoupledSubslabReceiptV1 {
            parent_support: self.parent_support,
            support: self.support,
            selected_upper_bound_s_bits: self.selected_upper_bound_s_bits,
            accepted_slab_sha256: self.accepted_slab_sha256,
            wb14_replay_trial_sha256: self.wb14_replay_trial_sha256,
            wb14_replay_beginning_owner_set_sha256: self.wb14_replay_beginning_owner_set_sha256,
            wb14_child_receipt_set_sha256: self.wb14_child_receipt_set_sha256,
            wb14_parent_receipt_set_sha256: self.wb14_parent_receipt_set_sha256,
            wb14_child_replay_bytes: self.wb14_child_replay_bytes,
            wb14_parent_replay_bytes: self.wb14_parent_replay_bytes,
            destination_receipts,
            lane_receipts: self.lane_receipts,
            physical_outcome_ledger_set_sha256: self.physical_outcome_ledger_set_sha256,
            terminal_events: self.terminal_events,
            post_support_liquid_receiver_event: self.post_support_liquid_receiver_event,
            post_support_liquid_output_set_sha256: self.post_support_liquid_output_set_sha256,
            post_support_liquid_mass_kg_m2_bits: self.post_support_liquid_mass_kg_m2_bits,
            post_support_liquid_enthalpy_j_m2_bits: self.post_support_liquid_enthalpy_j_m2_bits,
            post_support_liquid_surface_beginning_state: self
                .post_support_liquid_surface_beginning_state,
            post_support_liquid_surface_ending_state: self.post_support_liquid_surface_ending_state,
            post_support_liquid_custody_v2: None,
            owner_join: self.owner_join,
            receipt_sha256: self.receipt_sha256,
        })
    }
}

#[cfg(feature = "persisted-restart-v1")]
pub fn restart_authority_encode_in_progress_metadata_v2(
    value: &DirectSnowStage3V11InProgressExecutionV2,
) -> Result<Vec<u8>, DirectSnowStage3V11AttachmentError> {
    if value
        .coupled_subslabs
        .iter()
        .chain(&value.support_owner_joins)
        .any(|receipt| receipt.support_liquid_custody_v2().is_some())
        || value
            .terminal_event_groups
            .iter()
            .chain(&value.support_event_groups)
            .any(|group| group.terminal_receiver_custody_v2().is_some())
    {
        return Err(DirectSnowStage3V11AttachmentError::Identity(
            "V2 cannot omit zero-duration liquid custody V2",
        ));
    }
    restart_authority_encode_in_progress_metadata_base_v3(value)
}

#[cfg(feature = "persisted-restart-v1")]
pub fn restart_authority_encode_in_progress_metadata_base_v3(
    value: &DirectSnowStage3V11InProgressExecutionV2,
) -> Result<Vec<u8>, DirectSnowStage3V11AttachmentError> {
    serde_json::to_vec(&DirectSnowStage3V11InProgressMetadataWireV2 {
        schema: "OPENWEPP_SNOW_STAGE3_V11_IN_PROGRESS_RESTART_V2".to_owned(),
        version: 2,
        day_index: value.day_index,
        support_index: value.support_index,
        prepared_supports: value.prepared_supports.clone(),
        terminal_events: value.terminal_events.clone(),
        terminal_event_groups: value.terminal_event_groups.clone(),
        covered_owner_joins: value.covered_owner_joins.clone(),
        coupled_subslabs: value
            .coupled_subslabs
            .iter()
            .map(Stage3CoupledSubslabReceiptWireV2::project)
            .collect(),
        adaptive_support_receipts: value.adaptive_support_receipts.clone(),
        snow_free_successor_receipts: value.snow_free_successor_receipts.clone(),
        posture: value.posture,
        support_owner_joins: value
            .support_owner_joins
            .iter()
            .map(Stage3CoupledSubslabReceiptWireV2::project)
            .collect(),
        support_event_groups: value.support_event_groups.clone(),
        support_terminal_parcels: value.support_terminal_parcels.clone(),
        expected_child_beginning: value.expected_child_beginning,
        pending_adaptive_request: value.pending_adaptive_request.clone(),
        adaptive_receipts: value.adaptive_receipts.clone(),
        support_snow_free_successor_receipts: value.support_snow_free_successor_receipts.clone(),
        adaptive_trial_quanta: value.adaptive_trial_quanta.to_string(),
    })
    .map_err(|_| DirectSnowStage3V11AttachmentError::Identity("restart in-progress serialization"))
}

#[cfg(feature = "persisted-restart-v1")]
pub fn restart_authority_decode_in_progress_metadata_v2(
    bytes: &[u8],
    day_candidate: DirectSnowStage3V11CommittedState,
    support_current: Option<DirectSnowStage3V11CommittedState>,
) -> Result<DirectSnowStage3V11InProgressExecutionV2, DirectSnowStage3V11AttachmentError> {
    let wire: DirectSnowStage3V11InProgressMetadataWireV2 =
        serde_json::from_slice(bytes).map_err(|_| {
            DirectSnowStage3V11AttachmentError::Identity("restart in-progress deserialization")
        })?;
    if wire.schema != "OPENWEPP_SNOW_STAGE3_V11_IN_PROGRESS_RESTART_V2"
        || wire.version != 2
        || serde_json::to_vec(&wire).map_err(|_| {
            DirectSnowStage3V11AttachmentError::Identity("restart in-progress serialization")
        })? != bytes
    {
        return Err(DirectSnowStage3V11AttachmentError::Identity(
            "restart in-progress schema or canonical bytes",
        ));
    }
    let adaptive_trial_quanta = wire.adaptive_trial_quanta.parse::<u128>().map_err(|_| {
        DirectSnowStage3V11AttachmentError::Identity("restart adaptive trial quanta")
    })?;
    if wire.support_index >= wire.prepared_supports.len()
        || wire.day_index != wire.prepared_supports[0].day_index
        || wire
            .prepared_supports
            .iter()
            .enumerate()
            .any(|(index, support)| {
                support.day_index != wire.day_index || support.support_index != index
            })
        || support_current.is_none()
        || adaptive_trial_quanta == 0
    {
        return Err(DirectSnowStage3V11AttachmentError::Identity(
            "restart in-progress support join",
        ));
    }
    let coupled_subslabs = wire
        .coupled_subslabs
        .into_iter()
        .map(Stage3CoupledSubslabReceiptWireV2::restore)
        .collect::<Result<Vec<_>, _>>()?;
    let support_owner_joins = wire
        .support_owner_joins
        .into_iter()
        .map(Stage3CoupledSubslabReceiptWireV2::restore)
        .collect::<Result<Vec<_>, _>>()?;
    Ok(DirectSnowStage3V11InProgressExecutionV2 {
        day_candidate,
        support_current,
        day_index: wire.day_index,
        support_index: wire.support_index,
        prepared_supports: wire.prepared_supports,
        terminal_events: wire.terminal_events,
        terminal_event_groups: wire.terminal_event_groups,
        covered_owner_joins: wire.covered_owner_joins,
        coupled_subslabs,
        adaptive_support_receipts: wire.adaptive_support_receipts,
        snow_free_successor_receipts: wire.snow_free_successor_receipts,
        posture: wire.posture,
        support_owner_joins,
        support_event_groups: wire.support_event_groups,
        support_terminal_parcels: wire.support_terminal_parcels,
        expected_child_beginning: wire.expected_child_beginning,
        pending_adaptive_request: wire.pending_adaptive_request,
        adaptive_receipts: wire.adaptive_receipts,
        support_snow_free_successor_receipts: wire.support_snow_free_successor_receipts,
        adaptive_trial_quanta,
    })
}

/// Canonical identity of the exact provider/GSI-bound support carried by an
/// in-progress restart. The immutable typed support is supplied again by the
/// scheduler on admission; every physical/configuration projection is bound
/// here so it cannot be substituted while resuming.
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct DirectSnowStage3V11PreparedSupportRestartV2 {
    pub day_index: usize,
    pub support_index: usize,
    pub support: TimeSupport,
    pub accepted_gsi_receipt: Digest32,
    pub beginning_provider_cursor_sha256: Digest32,
    pub ending_provider_cursor_sha256: Digest32,
    pub stage3_support_forcing_sha256: Digest32,
    pub stage3_configuration_sha256: Digest32,
    pub covered_v11_forcing_sha256: Digest32,
    pub carrier_configuration_sha256: Digest32,
    pub hard_boundaries: Vec<ModelTimeNs>,
    pub lane_ids: Vec<u32>,
}

impl DirectSnowStage3V11PreparedSupportRestartV2 {
    fn project(
        prepared: &ValidatedPreparedStage3V11DayV1,
        support_index: usize,
    ) -> Result<Self, DirectSnowStage3V11AttachmentError> {
        let support = prepared.inner.supports.get(support_index).ok_or(
            DirectSnowStage3V11AttachmentError::Support("restart prepared-support ordinal"),
        )?;
        let cursor_digest = |cursor: &SnowFreeHalfHourProviderCursor| {
            serde_json::to_vec(cursor)
                .map(|bytes| digest_bytes(&bytes))
                .map_err(|_| {
                    DirectSnowStage3V11AttachmentError::Identity(
                        "restart provider-cursor serialization",
                    )
                })
        };
        let (
            stage3_support_forcing_sha256,
            stage3_configuration_sha256,
            covered_v11_forcing_sha256,
            carrier_configuration_sha256,
        ) = support.forcing_projections();
        Ok(Self {
            day_index: prepared.inner.day_index,
            support_index,
            support: support.support,
            accepted_gsi_receipt: prepared.inner.accepted_gsi_receipt,
            beginning_provider_cursor_sha256: cursor_digest(
                &prepared.inner.beginning_provider_cursor,
            )?,
            ending_provider_cursor_sha256: cursor_digest(&prepared.inner.ending_provider_cursor)?,
            stage3_support_forcing_sha256,
            stage3_configuration_sha256,
            covered_v11_forcing_sha256,
            carrier_configuration_sha256,
            hard_boundaries: support.hard_boundaries.clone(),
            lane_ids: support.snow_inputs_by_lane.keys().copied().collect(),
        })
    }

    fn validate_against(
        &self,
        prepared: &ValidatedPreparedStage3V11DayV1,
    ) -> Result<(), DirectSnowStage3V11AttachmentError> {
        if Self::project(prepared, self.support_index)? != *self {
            return Err(DirectSnowStage3V11AttachmentError::Identity(
                "restart prepared-support projection",
            ));
        }
        Ok(())
    }
}

impl ValidatedPreparedStage3V11DayV1 {
    #[cfg(feature = "persisted-restart-v1")]
    pub fn restart_authority_support_checkpoint_v2(
        &self,
        support_index: usize,
    ) -> Result<DirectSnowStage3V11PreparedSupportRestartV2, DirectSnowStage3V11AttachmentError>
    {
        DirectSnowStage3V11PreparedSupportRestartV2::project(self, support_index)
    }

    #[cfg(feature = "persisted-restart-v1")]
    pub fn restart_authority_validate_support_checkpoint_v2(
        &self,
        checkpoint: &DirectSnowStage3V11PreparedSupportRestartV2,
    ) -> Result<(), DirectSnowStage3V11AttachmentError> {
        checkpoint.validate_against(self)
    }
}

#[cfg(feature = "persisted-restart-v1")]
fn select_active_execution_beginning_v3(
    beginnings: &mut [(u128, Digest32)],
) -> Result<(u128, Digest32), DirectSnowStage3V11AttachmentError> {
    beginnings.sort_unstable_by_key(|(tick, _)| *tick);
    let (beginning_tick, beginning_owner) =
        beginnings
            .first()
            .copied()
            .ok_or(DirectSnowStage3V11AttachmentError::Identity(
                "restart V3 active receipt beginning execution class",
            ))?;
    if beginnings
        .iter()
        .skip(1)
        .take_while(|(tick, _)| *tick == beginning_tick)
        .any(|(_, owner)| *owner != beginning_owner)
    {
        return Err(DirectSnowStage3V11AttachmentError::Identity(
            "restart V3 active receipt ambiguous beginning owner",
        ));
    }
    Ok((beginning_tick, beginning_owner))
}

#[cfg(all(test, feature = "persisted-restart-v1"))]
mod restart_v3_active_execution_beginning_tests {
    use super::*;

    #[test]
    fn snow_free_predecessor_wins_and_omission_or_owner_substitution_rejects() {
        let snow_free_owner = digest_bytes(b"snow-free-day-beginning");
        let covered_owner = digest_bytes(b"later-covered-beginning");
        let mut mixed = vec![(72_000, covered_owner), (0, snow_free_owner)];
        assert_eq!(
            select_active_execution_beginning_v3(&mut mixed).unwrap(),
            (0, snow_free_owner),
        );

        let mut omitted = Vec::new();
        assert!(select_active_execution_beginning_v3(&mut omitted).is_err());

        let mut substituted = vec![(0, snow_free_owner), (0, covered_owner)];
        assert!(select_active_execution_beginning_v3(&mut substituted).is_err());
    }
}

impl DirectSnowStage3V11ShadowAttachment {
    #[cfg(all(
        feature = "persisted-restart-v1",
        feature = "restart-authority-evidence"
    ))]
    pub fn restart_authority_complete_commit_and_stage_archive_day_v3(
        &mut self,
        publication_inputs: &[crate::direct_runtime::DirectPublicationDayInput],
    ) -> Result<(), DirectSnowStage3V11AttachmentError> {
        let day_index = self.committed.real_consumer.v11_next_day_index();
        let (
            mut completed_frame,
            supports,
            event_handoffs,
            terminal_event_groups,
            coupled_subslabs,
            beginning_stage3,
            ending_stage3,
            surface_configuration,
        ) = self.pending_publication_completion_inputs(day_index)?;
        let publication_day = crate::direct_runtime::Stage3AcceptedPublicationDayV1::try_complete(
            &mut completed_frame,
            day_index,
            publication_inputs,
            &supports,
            &event_handoffs,
            &terminal_event_groups,
            &coupled_subslabs,
            &beginning_stage3,
            &ending_stage3,
            &surface_configuration,
        )
        .map_err(|_| {
            DirectSnowStage3V11AttachmentError::Identity(
                "restart V3 evidence publication completion",
            )
        })?;
        self.complete_pending_publication_day(publication_day)?;
        self.commit_staged_day()?;
        let day_delta = self.build_qualification_day_delta_v1(day_index)?;
        self.stage_committed_day_archive_v1(day_delta)
    }

    #[cfg(feature = "restart-authority-evidence")]
    pub fn restart_authority_install_hydrology_continuation_inputs_v3(
        &mut self,
        continuation_inputs: &[Vec<crate::direct_runtime::DirectDayConstructorInputs>],
    ) -> Result<(), DirectSnowStage3V11AttachmentError> {
        self.committed
            .real_consumer
            .restart_authority_install_hydrology_continuation_inputs_v3(continuation_inputs)
            .map_err(|_| {
                DirectSnowStage3V11AttachmentError::Identity(
                    "restart V3 hydrology continuation input installation",
                )
            })
    }

    #[cfg(feature = "persisted-restart-v1")]
    pub fn restart_authority_archived_receipt_prefix_v3(
        &self,
    ) -> Result<Stage3ArchivedReceiptPrefixV1, DirectSnowStage3V11AttachmentError> {
        if self.pending_committed_day_evidence.is_some() {
            return Err(DirectSnowStage3V11AttachmentError::Identity(
                "restart V3 pending archive acknowledgement posture",
            ));
        }
        self.archived_receipt_prefix.validate()?;
        if self.archived_receipt_prefix.run_identity != self.static_context.run_identity
            || self.archived_receipt_prefix.topology_identity
                != self.static_context.topology_identity
        {
            return Err(DirectSnowStage3V11AttachmentError::Identity(
                "restart V3 archive prefix static context",
            ));
        }
        self.restart_authority_validate_archived_receipt_prefix_tail_v3(
            &self.archived_receipt_prefix,
        )?;
        Ok(self.archived_receipt_prefix.clone())
    }

    #[cfg(feature = "persisted-restart-v1")]
    pub fn restart_authority_install_archived_receipt_prefix_v3(
        &mut self,
        prefix: Stage3ArchivedReceiptPrefixV1,
    ) -> Result<(), DirectSnowStage3V11AttachmentError> {
        prefix.validate()?;
        if self.pending_committed_day_evidence.is_some()
            || prefix.run_identity != self.static_context.run_identity
            || prefix.topology_identity != self.static_context.topology_identity
        {
            return Err(DirectSnowStage3V11AttachmentError::Identity(
                "restart V3 archive prefix installation context or posture",
            ));
        }
        let validate_tail = |state: &DirectSnowStage3V11CommittedState| {
            if state.receipt_chain.len() > 1
                || state
                    .receipt_chain
                    .first()
                    .is_some_and(|receipt| receipt.day_index < prefix.archived_day_count)
            {
                return Err(DirectSnowStage3V11AttachmentError::Identity(
                    "restart V3 resident receipt tail bound or overlap",
                ));
            }
            Ok(())
        };
        validate_tail(&self.committed)?;
        if let Some(pending) = &self.pending_candidate {
            validate_tail(&pending.ending_state)?;
        }
        if let Some(execution) = self.in_progress_execution.as_deref() {
            validate_tail(&execution.day_candidate)?;
            validate_tail(execution.support_current.as_ref().ok_or(
                DirectSnowStage3V11AttachmentError::Identity(
                    "restart V3 in-progress support owner",
                ),
            )?)?;
        }
        self.restart_authority_validate_archived_receipt_prefix_tail_v3(&prefix)?;
        self.archived_receipt_prefix = prefix;
        Ok(())
    }

    #[cfg(feature = "persisted-restart-v1")]
    fn restart_authority_validate_archived_receipt_prefix_tail_v3(
        &self,
        prefix: &Stage3ArchivedReceiptPrefixV1,
    ) -> Result<(), DirectSnowStage3V11AttachmentError> {
        let current_owner = complete_owner_set_digest(self.committed.coupled_clock.owners())?;
        match self.committed.receipt_chain.as_slice() {
            [] => {
                if prefix.accepted_until_ns != self.committed.coupled_clock.accepted_until().get()
                    || prefix.next_parent_sequence != self.committed.next_parent_sequence
                    || prefix
                        .ending_owner_set_sha256
                        .is_some_and(|ending| ending != current_owner)
                {
                    return Err(DirectSnowStage3V11AttachmentError::Identity(
                        "restart V3 archive prefix/current committed owner-clock join",
                    ));
                }
            }
            [receipt] => {
                let mut beginnings = receipt
                    .coupled_subslabs
                    .iter()
                    .map(|subslab| {
                        (
                            subslab.support.start_ns().get(),
                            subslab.owner_join.beginning_complete_owner_set_sha256,
                        )
                    })
                    .chain(
                        receipt
                            .snow_free_successor_receipts
                            .iter()
                            .map(|successor| {
                                (
                                    successor.support.start_ns().get(),
                                    successor.beginning_complete_owner_set_sha256,
                                )
                            }),
                    )
                    .collect::<Vec<_>>();
                let (beginning_tick, beginning_owner) =
                    select_active_execution_beginning_v3(&mut beginnings)?;
                let require = |condition, detail| {
                    if condition {
                        Ok(())
                    } else {
                        Err(DirectSnowStage3V11AttachmentError::Identity(detail))
                    }
                };
                require(
                    receipt.day_index == prefix.archived_day_count,
                    "restart V3 active receipt day/prefix count join",
                )?;
                require(
                    beginning_tick == prefix.accepted_until_ns,
                    "restart V3 active receipt beginning tick/prefix cursor join",
                )?;
                require(
                    prefix.accepted_until_ns.checked_add(STAGE3_V11_DAY_NS)
                        == Some(receipt.ending_coupled_accepted_until_ns.get()),
                    "restart V3 active receipt ending tick/prefix day join",
                )?;
                require(
                    prefix.next_parent_sequence.checked_add(48_u128)
                        == Some(receipt.ending_next_parent_sequence),
                    "restart V3 active receipt ending sequence/prefix count join",
                )?;
                require(
                    prefix
                        .ending_owner_set_sha256
                        .is_none_or(|ending| ending == beginning_owner),
                    "restart V3 active receipt beginning owner/prefix owner join",
                )?;
                require(
                    receipt.ending_coupled_accepted_until_ns
                        == self.committed.coupled_clock.accepted_until(),
                    "restart V3 active receipt ending tick/committed clock join",
                )?;
                require(
                    receipt.ending_next_parent_sequence == self.committed.next_parent_sequence,
                    "restart V3 active receipt ending/committed sequence join",
                )?;
                require(
                    receipt.ending_coupled_owner_set_sha256 == current_owner,
                    "restart V3 active receipt ending/committed owner join",
                )?;
            }
            _ => {
                return Err(DirectSnowStage3V11AttachmentError::Identity(
                    "restart V3 resident committed receipt bound",
                ));
            }
        }
        Ok(())
    }

    /// Advance the prepared day without publication until the requested exact
    /// interruption posture is reached. `false` means an in-progress owner was
    /// durably retained; `true` means the day completed before that posture and
    /// is staged at the normal atomic installation boundary.
    pub fn stage_prepared_day_until_posture_v2(
        &mut self,
        prepared: &ValidatedPreparedStage3V11DayV1,
        posture: DirectSnowStage3V11InterruptionPostureV2,
    ) -> Result<bool, DirectSnowStage3V11AttachmentError> {
        if self.pending_candidate.is_some() {
            return Err(DirectSnowStage3V11AttachmentError::Identity(
                "duplicate uncommitted Stage-3/V11 parent",
            ));
        }
        let restart = self.in_progress_execution.take();
        let rollback = restart.clone();
        let outcome = self.execute_prepared_day_resumable_v2(prepared, restart, Some(posture));
        let outcome = match outcome {
            Ok(outcome) => outcome,
            Err(error) => {
                self.in_progress_execution = rollback;
                return Err(error);
            }
        };
        match outcome {
            PreparedDayExecutionOutcomeV2::Complete(candidate) => {
                self.pending_candidate = Some(candidate);
                Ok(true)
            }
            PreparedDayExecutionOutcomeV2::Paused(checkpoint) => {
                self.in_progress_execution = Some(checkpoint);
                Ok(false)
            }
        }
    }

    /// Continue an admitted in-progress owner through the end of the day and
    /// stage it for the existing atomic commit operation.
    pub fn finish_in_progress_prepared_day_v2(
        &mut self,
        prepared: &ValidatedPreparedStage3V11DayV1,
    ) -> Result<(), DirectSnowStage3V11AttachmentError> {
        if self.pending_candidate.is_some() {
            return Err(DirectSnowStage3V11AttachmentError::Identity(
                "duplicate uncommitted Stage-3/V11 parent",
            ));
        }
        let restart = self.in_progress_execution.take().ok_or(
            DirectSnowStage3V11AttachmentError::Identity("missing in-progress Stage-3/V11 parent"),
        )?;
        let rollback = restart.clone();
        let outcome = self.execute_prepared_day_resumable_v2(prepared, Some(restart), None);
        let outcome = match outcome {
            Ok(outcome) => outcome,
            Err(error) => {
                self.in_progress_execution = Some(rollback);
                return Err(error);
            }
        };
        match outcome {
            PreparedDayExecutionOutcomeV2::Complete(candidate) => {
                self.pending_candidate = Some(candidate);
                Ok(())
            }
            PreparedDayExecutionOutcomeV2::Paused(checkpoint) => {
                self.in_progress_execution = Some(checkpoint);
                Err(DirectSnowStage3V11AttachmentError::Identity(
                    "unexpected finish interruption",
                ))
            }
        }
    }

    #[cfg(feature = "persisted-restart-v1")]
    #[must_use]
    pub fn restart_authority_in_progress_execution_v2(
        &self,
    ) -> Option<&DirectSnowStage3V11InProgressExecutionV2> {
        self.in_progress_execution.as_deref()
    }

    /// Return the staged-but-uncommitted complete day candidate, when the
    /// scheduler has crossed `stage_prepared_day` but not `commit_staged_day`.
    #[cfg(feature = "persisted-restart-v1")]
    #[must_use]
    pub fn restart_authority_pending_candidate(
        &self,
    ) -> Option<&DirectSnowStage3V11ParentCandidate> {
        self.pending_candidate.as_ref()
    }

    /// Reconstruct an attachment only after the normal constructor and, for a
    /// staged candidate, the normal installation validator accept every owner,
    /// clock, receipt-chain, terminal-parcel, and topology join.
    #[cfg(feature = "persisted-restart-v1")]
    pub fn restart_authority_restore_parts(
        static_context: DirectSnowStage3V11StaticContext,
        committed: DirectSnowStage3V11CommittedState,
        pending_candidate: Option<DirectSnowStage3V11ParentCandidate>,
    ) -> Result<Self, DirectSnowStage3V11AttachmentError> {
        let mut restored = Self::new(static_context, committed)?;
        if let Some(last) = restored.committed.receipt_chain.last() {
            last.validate_against_ending(&restored.committed)?;
        }
        if let Some(candidate) = pending_candidate {
            let mut validator = restored.clone();
            validator.install_candidate(candidate.clone())?;
            restored.pending_candidate = Some(candidate);
        }
        Ok(restored)
    }

    /// Restore an in-progress execution only after its committed predecessor,
    /// current support owners, prepared-support identities, receipt joins, and
    /// posture have all been reconstructed by their canonical restart paths.
    #[cfg(feature = "persisted-restart-v1")]
    pub fn restart_authority_restore_parts_with_in_progress_v2(
        static_context: DirectSnowStage3V11StaticContext,
        committed: DirectSnowStage3V11CommittedState,
        pending_candidate: Option<DirectSnowStage3V11ParentCandidate>,
        in_progress: Option<DirectSnowStage3V11InProgressExecutionV2>,
    ) -> Result<Self, DirectSnowStage3V11AttachmentError> {
        let mut restored =
            Self::restart_authority_restore_parts(static_context, committed, pending_candidate)?;
        if restored.pending_candidate.is_some() && in_progress.is_some() {
            return Err(DirectSnowStage3V11AttachmentError::Identity(
                "restart pending/in-progress exclusivity",
            ));
        }
        if let Some(in_progress) = in_progress {
            in_progress.validate(&restored.static_context, &restored.committed)?;
            restored.in_progress_execution = Some(Box::new(in_progress));
        }
        Ok(restored)
    }
}

impl DirectSnowStage3V11InProgressExecutionV2 {
    fn validate(
        &self,
        context: &DirectSnowStage3V11StaticContext,
        committed: &DirectSnowStage3V11CommittedState,
    ) -> Result<(), DirectSnowStage3V11AttachmentError> {
        let current =
            self.support_current
                .as_ref()
                .ok_or(DirectSnowStage3V11AttachmentError::Identity(
                    "restart missing current support owner",
                ))?;
        if self.prepared_supports.is_empty()
            || self.support_index >= self.prepared_supports.len()
            || self.day_index != committed.real_consumer.v11_next_day_index()
            || self.day_index != self.prepared_supports[0].day_index
            || self.day_candidate.receipt_chain != committed.receipt_chain
            || self.day_candidate.next_parent_sequence
                != committed
                    .next_parent_sequence
                    .checked_add(u128::try_from(self.support_index).map_err(|_| {
                        DirectSnowStage3V11AttachmentError::Identity(
                            "restart in-progress support-index width",
                        )
                    })?)
                    .ok_or(DirectSnowStage3V11AttachmentError::Identity(
                        "restart in-progress sequence overflow",
                    ))?
            || current.coupled_clock.parent_support()
                != self.prepared_supports[self.support_index].support
            || current.coupled_clock.accepted_until()
                < self.prepared_supports[self.support_index]
                    .support
                    .start_ns()
            || current.coupled_clock.accepted_until()
                > self.prepared_supports[self.support_index].support.end_ns()
            || current.v11_parent_state.parent_transaction_id()
                != current.coupled_clock.parent_transaction_id()
            || current.stage3_by_lane.keys().copied().collect::<Vec<_>>() != context.lane_ids
            || complete_owner_set_digest(current.coupled_clock.owners())?
                != self.expected_child_beginning
        {
            return Err(DirectSnowStage3V11AttachmentError::Identity(
                "restart in-progress owner or chronology join",
            ));
        }
        validate_restart_adaptive_trial_grid_v2(
            self.posture,
            self.adaptive_trial_quanta,
            context.minimum_support_ns,
            self.prepared_supports[self.support_index].support,
            current.coupled_clock.accepted_until(),
        )?;
        for receipt in self
            .coupled_subslabs
            .iter()
            .chain(&self.support_owner_joins)
        {
            receipt.validate()?;
        }
        for receipt in &self.adaptive_support_receipts {
            receipt.validate()?;
        }
        for receipt in self
            .snow_free_successor_receipts
            .iter()
            .chain(&self.support_snow_free_successor_receipts)
        {
            receipt.validate()?;
        }
        if self.adaptive_receipts.parent_requests.len()
            != self.adaptive_receipts.direct_trials.len()
            || self.adaptive_receipts.parent_requests.len()
                != self.adaptive_receipts.comparisons.len()
        {
            return Err(DirectSnowStage3V11AttachmentError::Identity(
                "restart adaptive receipt cardinality",
            ));
        }
        for (request, direct) in self
            .adaptive_receipts
            .parent_requests
            .iter()
            .zip(&self.adaptive_receipts.direct_trials)
        {
            direct.validate_against(request)?;
        }
        for child in &self.adaptive_receipts.split_child_trials {
            child.validate()?;
        }
        for comparison in &self.adaptive_receipts.comparisons {
            comparison.validate()?;
        }
        for accepted in &self.adaptive_receipts.accepted_microsteps {
            let comparison = self
                .adaptive_receipts
                .comparisons
                .iter()
                .find(|comparison| comparison.receipt_sha256 == accepted.comparison_receipt_sha256)
                .ok_or(DirectSnowStage3V11AttachmentError::Identity(
                    "restart adaptive accepted comparison",
                ))?;
            accepted.validate_against(comparison)?;
        }
        stage3_adaptive_accepted_microstep_set_sha256_v1(
            &self.adaptive_receipts.accepted_microsteps,
        )?;
        if let Some(request) = &self.pending_adaptive_request {
            request.validate()?;
            if request.context.parent_transaction_id
                != current.coupled_clock.parent_transaction_id()
                || request.context.parent_support
                    != self.prepared_supports[self.support_index].support
                || request.context.beginning_complete_owner_set_sha256
                    != self.expected_child_beginning
            {
                return Err(DirectSnowStage3V11AttachmentError::Identity(
                    "restart pending adaptive request join",
                ));
            }
        }
        if let Some(first) = self.support_owner_joins.first() {
            let beginning_owner =
                complete_owner_set_digest(self.day_candidate.coupled_clock.owners())?;
            let last = self.support_owner_joins.last().ok_or(
                DirectSnowStage3V11AttachmentError::Identity("restart support owner receipt set"),
            )?;
            for group in &self.support_event_groups {
                if group.accepted_group_receipt_sha256
                    != Some(accepted_terminal_group_digest(group)?)
                {
                    return Err(DirectSnowStage3V11AttachmentError::Identity(
                        "restart adaptive terminal event group",
                    ));
                }
            }
            let mut ending_owner = last.effective_ending_complete_owner_set_sha256();
            if let Some(first_successor_event) = current
                .coupled_clock
                .accepted_event_receipts()
                .iter()
                .position(|event| event.beginning_owner_set_digest() == ending_owner)
            {
                for event in
                    &current.coupled_clock.accepted_event_receipts()[first_successor_event..]
                {
                    event.validate()?;
                    if event.beginning_owner_set_digest() != ending_owner {
                        return Err(DirectSnowStage3V11AttachmentError::Identity(
                            "restart adaptive event owner chronology",
                        ));
                    }
                    ending_owner = event.ending_owner_set_digest();
                }
            }
            if first.owner_join.beginning_complete_owner_set_sha256 != beginning_owner
                || ending_owner != self.expected_child_beginning
                || self.support_owner_joins.windows(2).any(|pair| {
                    pair[0].support.end_ns() != pair[1].support.start_ns()
                        || pair[0].effective_ending_complete_owner_set_sha256()
                            != pair[1].owner_join.beginning_complete_owner_set_sha256
                })
            {
                return Err(DirectSnowStage3V11AttachmentError::Identity(
                    "restart adaptive support owner chronology",
                ));
            }
        }
        Ok(())
    }

    fn validate_prepared_day(
        &self,
        prepared: &ValidatedPreparedStage3V11DayV1,
    ) -> Result<(), DirectSnowStage3V11AttachmentError> {
        if self.prepared_supports.len() != prepared.supports().len() {
            return Err(DirectSnowStage3V11AttachmentError::Support(
                "restart prepared-day support cardinality",
            ));
        }
        for checkpoint in &self.prepared_supports {
            checkpoint.validate_against(prepared)?;
        }
        Ok(())
    }
}

#[cfg(feature = "persisted-restart-v1")]
#[derive(Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
struct Stage3V11ReceiptStateRestartWireV2 {
    schema: String,
    version: u16,
    terminal_parcels: BTreeMap<Digest32, DirectSnowStage3V11TerminalParcel>,
    receipt_chain: Vec<Stage3V11ParentReceiptRestartWireV2>,
    payload_sha256: Digest32,
}

#[cfg(feature = "persisted-restart-v1")]
#[derive(Serialize)]
struct Stage3V11ReceiptStateDigestInput<'a> {
    schema: &'a str,
    version: u16,
    terminal_parcels: &'a BTreeMap<Digest32, DirectSnowStage3V11TerminalParcel>,
    receipt_chain: &'a [Stage3V11ParentReceiptRestartWireV2],
}

#[cfg(feature = "persisted-restart-v1")]
#[derive(Clone, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
struct Stage3V11ParentReceiptRestartWireV2 {
    day_index: usize,
    support_count: usize,
    terminal_events: Vec<DirectSnowStage3V11TerminalReceipt>,
    terminal_event_groups: Vec<Stage3V11TerminalEventGroupV1>,
    ending_stage3_state_digests: BTreeMap<u32, Digest32>,
    complete_owner_bytes: BTreeMap<String, Vec<u8>>,
    covered_owner_joins: Vec<CoveredParentOwnerJoinReceiptV1>,
    coupled_subslabs: Vec<Stage3CoupledSubslabReceiptWireV2>,
    adaptive_support_receipts: Vec<Stage3AdaptiveSupportReceiptV1>,
    snow_free_successor_receipts: Vec<Stage3SnowFreeSuccessorReceiptV1>,
    integrated_boundary_ledger: Stage3ParentIntegratedBoundaryLedgerV1,
    ending_coupled_owner_set_sha256: Digest32,
    ending_coupled_accepted_until_ns: ModelTimeNs,
    ending_next_parent_sequence: String,
    ending_v11_parent_checkpoint: openwepp_vegetation::V11ParentTransactionCheckpoint,
    has_ending_last_v11_parent_candidate: bool,
}

#[cfg(feature = "persisted-restart-v1")]
impl Stage3V11ParentReceiptRestartWireV2 {
    fn project(value: &DirectSnowStage3V11ParentReceipt) -> Self {
        Self {
            day_index: value.day_index,
            support_count: value.support_count,
            terminal_events: value.terminal_events.clone(),
            terminal_event_groups: value.terminal_event_groups.clone(),
            ending_stage3_state_digests: value.ending_stage3_state_digests.clone(),
            complete_owner_bytes: value.complete_owner_bytes.clone(),
            covered_owner_joins: value.covered_owner_joins.clone(),
            coupled_subslabs: value
                .coupled_subslabs
                .iter()
                .map(Stage3CoupledSubslabReceiptWireV2::project)
                .collect(),
            adaptive_support_receipts: value.adaptive_support_receipts.clone(),
            snow_free_successor_receipts: value.snow_free_successor_receipts.clone(),
            integrated_boundary_ledger: value.integrated_boundary_ledger.clone(),
            ending_coupled_owner_set_sha256: value.ending_coupled_owner_set_sha256,
            ending_coupled_accepted_until_ns: value.ending_coupled_accepted_until_ns,
            ending_next_parent_sequence: value.ending_next_parent_sequence.to_string(),
            ending_v11_parent_checkpoint: value.ending_v11_parent_state.checkpoint(),
            has_ending_last_v11_parent_candidate: value.ending_last_v11_parent_candidate.is_some(),
        }
    }

    #[cfg(feature = "persisted-restart-v1")]
    fn restore(
        self,
        configuration: &VegetationConfigurationV11,
    ) -> Result<DirectSnowStage3V11ParentReceipt, DirectSnowStage3V11AttachmentError> {
        let ending_v11_parent_state =
            V11ParentTransaction::restore(configuration, self.ending_v11_parent_checkpoint)?;
        let ending_last_v11_parent_candidate = self
            .has_ending_last_v11_parent_candidate
            .then(|| ending_v11_parent_state.clone().finalize(configuration))
            .transpose()?;
        let ending_next_parent_sequence = self
            .ending_next_parent_sequence
            .parse::<u128>()
            .map_err(|_| {
                DirectSnowStage3V11AttachmentError::Identity("restart parent-receipt sequence")
            })?;
        let coupled_subslabs = self
            .coupled_subslabs
            .into_iter()
            .map(Stage3CoupledSubslabReceiptWireV2::restore)
            .collect::<Result<Vec<_>, _>>()?;
        Ok(DirectSnowStage3V11ParentReceipt {
            day_index: self.day_index,
            support_count: self.support_count,
            terminal_events: self.terminal_events,
            terminal_event_groups: self.terminal_event_groups,
            ending_stage3_state_digests: self.ending_stage3_state_digests,
            complete_owner_bytes: self.complete_owner_bytes,
            covered_owner_joins: self.covered_owner_joins,
            coupled_subslabs,
            adaptive_support_receipts: self.adaptive_support_receipts,
            snow_free_successor_receipts: self.snow_free_successor_receipts,
            integrated_boundary_ledger: self.integrated_boundary_ledger,
            ending_coupled_owner_set_sha256: self.ending_coupled_owner_set_sha256,
            ending_coupled_accepted_until_ns: self.ending_coupled_accepted_until_ns,
            ending_next_parent_sequence,
            ending_v11_parent_state,
            ending_last_v11_parent_candidate,
        })
    }
}

#[derive(Serialize)]
struct Stage3DestinationBoundaryReceiptWireRefV2<'a> {
    ofe_id: &'a OfeId,
    tile_id: &'a TileId,
    receipt: &'a FinalStage3TileBoundaryReceiptV1,
}

#[derive(Clone, Copy)]
struct Stage3DestinationBoundaryReceiptsWireRefV2<'a>(
    &'a BTreeMap<(OfeId, TileId), FinalStage3TileBoundaryReceiptV1>,
);

impl Serialize for Stage3DestinationBoundaryReceiptsWireRefV2<'_> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeSeq as _;

        let mut sequence = serializer.serialize_seq(Some(self.0.len()))?;
        for ((ofe_id, tile_id), receipt) in self.0 {
            sequence.serialize_element(&Stage3DestinationBoundaryReceiptWireRefV2 {
                ofe_id,
                tile_id,
                receipt,
            })?;
        }
        sequence.end()
    }
}

#[derive(Serialize)]
struct Stage3CoupledSubslabReceiptWireRefV2<'a> {
    parent_support: &'a TimeSupport,
    support: &'a TimeSupport,
    selected_upper_bound_s_bits: u64,
    accepted_slab_sha256: &'a Digest32,
    wb14_replay_trial_sha256: &'a Digest32,
    wb14_replay_beginning_owner_set_sha256: &'a Digest32,
    wb14_child_receipt_set_sha256: &'a Digest32,
    wb14_parent_receipt_set_sha256: &'a Option<Digest32>,
    wb14_child_replay_bytes: &'a [u8],
    wb14_parent_replay_bytes: &'a Option<Vec<u8>>,
    destination_receipts: Stage3DestinationBoundaryReceiptsWireRefV2<'a>,
    lane_receipts: &'a BTreeMap<u32, LaneStage3BoundaryReceiptV1>,
    physical_outcome_ledger_set_sha256: &'a Digest32,
    terminal_events: &'a BTreeMap<u32, DirectSnowTerminalEventResult>,
    post_support_liquid_receiver_event: &'a Option<AcceptedEventReceiptV1>,
    post_support_liquid_output_set_sha256: &'a Option<Digest32>,
    post_support_liquid_mass_kg_m2_bits: &'a Option<u64>,
    post_support_liquid_enthalpy_j_m2_bits: &'a Option<u64>,
    post_support_liquid_surface_beginning_state:
        &'a Option<crate::DirectSurfaceLiquidOwnedState>,
    post_support_liquid_surface_ending_state: &'a Option<crate::DirectSurfaceLiquidOwnedState>,
    owner_join: &'a CoveredParentOwnerJoinReceiptV1,
    receipt_sha256: &'a Digest32,
}

impl<'a> From<&'a Stage3CoupledSubslabReceiptV1>
    for Stage3CoupledSubslabReceiptWireRefV2<'a>
{
    fn from(value: &'a Stage3CoupledSubslabReceiptV1) -> Self {
        Self {
            parent_support: &value.parent_support,
            support: &value.support,
            selected_upper_bound_s_bits: value.selected_upper_bound_s_bits,
            accepted_slab_sha256: &value.accepted_slab_sha256,
            wb14_replay_trial_sha256: &value.wb14_replay_trial_sha256,
            wb14_replay_beginning_owner_set_sha256: &value
                .wb14_replay_beginning_owner_set_sha256,
            wb14_child_receipt_set_sha256: &value.wb14_child_receipt_set_sha256,
            wb14_parent_receipt_set_sha256: &value.wb14_parent_receipt_set_sha256,
            wb14_child_replay_bytes: &value.wb14_child_replay_bytes,
            wb14_parent_replay_bytes: &value.wb14_parent_replay_bytes,
            destination_receipts: Stage3DestinationBoundaryReceiptsWireRefV2(
                &value.destination_receipts,
            ),
            lane_receipts: &value.lane_receipts,
            physical_outcome_ledger_set_sha256: &value.physical_outcome_ledger_set_sha256,
            terminal_events: &value.terminal_events,
            post_support_liquid_receiver_event: &value.post_support_liquid_receiver_event,
            post_support_liquid_output_set_sha256: &value.post_support_liquid_output_set_sha256,
            post_support_liquid_mass_kg_m2_bits: &value.post_support_liquid_mass_kg_m2_bits,
            post_support_liquid_enthalpy_j_m2_bits: &value
                .post_support_liquid_enthalpy_j_m2_bits,
            post_support_liquid_surface_beginning_state: &value
                .post_support_liquid_surface_beginning_state,
            post_support_liquid_surface_ending_state: &value
                .post_support_liquid_surface_ending_state,
            owner_join: &value.owner_join,
            receipt_sha256: &value.receipt_sha256,
        }
    }
}

#[derive(Clone, Copy)]
struct Stage3CoupledSubslabReceiptsWireRefV2<'a>(&'a [Stage3CoupledSubslabReceiptV1]);

impl Serialize for Stage3CoupledSubslabReceiptsWireRefV2<'_> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeSeq as _;

        let mut sequence = serializer.serialize_seq(Some(self.0.len()))?;
        for receipt in self.0 {
            sequence.serialize_element(&Stage3CoupledSubslabReceiptWireRefV2::from(receipt))?;
        }
        sequence.end()
    }
}

#[derive(Serialize)]
struct Stage3V11ParentReceiptRestartWireRefV2<'a> {
    day_index: usize,
    support_count: usize,
    terminal_events: &'a [DirectSnowStage3V11TerminalReceipt],
    terminal_event_groups: &'a [Stage3V11TerminalEventGroupV1],
    ending_stage3_state_digests: &'a BTreeMap<u32, Digest32>,
    complete_owner_bytes: &'a BTreeMap<String, Vec<u8>>,
    covered_owner_joins: &'a [CoveredParentOwnerJoinReceiptV1],
    coupled_subslabs: Stage3CoupledSubslabReceiptsWireRefV2<'a>,
    adaptive_support_receipts: &'a [Stage3AdaptiveSupportReceiptV1],
    snow_free_successor_receipts: &'a [Stage3SnowFreeSuccessorReceiptV1],
    integrated_boundary_ledger: &'a Stage3ParentIntegratedBoundaryLedgerV1,
    ending_coupled_owner_set_sha256: &'a Digest32,
    ending_coupled_accepted_until_ns: &'a ModelTimeNs,
    ending_next_parent_sequence: String,
    ending_v11_parent_checkpoint: openwepp_vegetation::V11ParentTransactionCheckpoint,
    has_ending_last_v11_parent_candidate: bool,
}

impl<'a> Stage3V11ParentReceiptRestartWireRefV2<'a> {
    fn project(value: &'a DirectSnowStage3V11ParentReceipt) -> Self {
        Self {
            day_index: value.day_index,
            support_count: value.support_count,
            terminal_events: &value.terminal_events,
            terminal_event_groups: &value.terminal_event_groups,
            ending_stage3_state_digests: &value.ending_stage3_state_digests,
            complete_owner_bytes: &value.complete_owner_bytes,
            covered_owner_joins: &value.covered_owner_joins,
            coupled_subslabs: Stage3CoupledSubslabReceiptsWireRefV2(&value.coupled_subslabs),
            adaptive_support_receipts: &value.adaptive_support_receipts,
            snow_free_successor_receipts: &value.snow_free_successor_receipts,
            integrated_boundary_ledger: &value.integrated_boundary_ledger,
            ending_coupled_owner_set_sha256: &value.ending_coupled_owner_set_sha256,
            ending_coupled_accepted_until_ns: &value.ending_coupled_accepted_until_ns,
            ending_next_parent_sequence: value.ending_next_parent_sequence.to_string(),
            ending_v11_parent_checkpoint: value.ending_v11_parent_state.checkpoint(),
            has_ending_last_v11_parent_candidate: value.ending_last_v11_parent_candidate.is_some(),
        }
    }
}

#[derive(Clone, Copy)]
struct Stage3SupportLiquidCustodyWireRefV3<'a>(&'a [Stage3CoupledSubslabReceiptV1]);

impl Serialize for Stage3SupportLiquidCustodyWireRefV3<'_> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeSeq as _;

        let mut sequence = serializer.serialize_seq(Some(self.0.len()))?;
        for receipt in self.0 {
            sequence.serialize_element(&receipt.support_liquid_custody_v2())?;
        }
        sequence.end()
    }
}

#[derive(Clone, Copy)]
struct Stage3TerminalLiquidCustodyWireRefV3<'a>(&'a [Stage3V11TerminalEventGroupV1]);

impl Serialize for Stage3TerminalLiquidCustodyWireRefV3<'_> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeSeq as _;

        let mut sequence = serializer.serialize_seq(Some(self.0.len()))?;
        for group in self.0 {
            sequence.serialize_element(&group.terminal_receiver_custody_v2())?;
        }
        sequence.end()
    }
}

#[cfg(feature = "persisted-restart-v1")]
#[derive(Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
struct Stage3V11ParentReceiptArchiveWireV3 {
    schema: String,
    version: u16,
    receipt: Stage3V11ParentReceiptRestartWireV2,
    support_liquid_custody_v2: Vec<Option<Stage3SupportLiquidCustodyV2>>,
    terminal_liquid_custody_v2: Vec<Option<Stage3TerminalLiquidCustodyV2>>,
    payload_sha256: Digest32,
}

#[cfg(feature = "persisted-restart-v1")]
#[derive(Serialize)]
struct Stage3V11ParentReceiptArchiveDigestInputV3<'a> {
    schema: &'a str,
    version: u16,
    receipt: &'a Stage3V11ParentReceiptRestartWireV2,
    support_liquid_custody_v2: &'a [Option<Stage3SupportLiquidCustodyV2>],
    terminal_liquid_custody_v2: &'a [Option<Stage3TerminalLiquidCustodyV2>],
}

#[derive(Serialize)]
struct Stage3V11ParentReceiptArchiveDigestInputRefV3<'a> {
    schema: &'a str,
    version: u16,
    receipt: &'a Stage3V11ParentReceiptRestartWireRefV2<'a>,
    support_liquid_custody_v2: Stage3SupportLiquidCustodyWireRefV3<'a>,
    terminal_liquid_custody_v2: Stage3TerminalLiquidCustodyWireRefV3<'a>,
}

#[derive(Serialize)]
struct Stage3V11ParentReceiptArchiveWireBorrowedV3<'a> {
    schema: &'a str,
    version: u16,
    receipt: &'a Stage3V11ParentReceiptRestartWireRefV2<'a>,
    support_liquid_custody_v2: Stage3SupportLiquidCustodyWireRefV3<'a>,
    terminal_liquid_custody_v2: Stage3TerminalLiquidCustodyWireRefV3<'a>,
    payload_sha256: Digest32,
}

#[cfg(feature = "persisted-restart-v1")]
fn stage3_v11_parent_receipt_archive_digest_v3(
    receipt: &Stage3V11ParentReceiptRestartWireV2,
    support_liquid_custody_v2: &[Option<Stage3SupportLiquidCustodyV2>],
    terminal_liquid_custody_v2: &[Option<Stage3TerminalLiquidCustodyV2>],
) -> Result<Digest32, DirectSnowStage3V11AttachmentError> {
    let mut digest = crate::snow_stage3_v11_attachment::ArchiveDigestCountWriter::default();
    serde_json::to_writer(
        &mut digest,
        &Stage3V11ParentReceiptArchiveDigestInputV3 {
            schema: "OPENWEPP_SNOW_STAGE3_V11_PARENT_RECEIPT_ARCHIVE_V3",
            version: 3,
            receipt,
            support_liquid_custody_v2,
            terminal_liquid_custody_v2,
        },
    )
    .map_err(|_| {
        DirectSnowStage3V11AttachmentError::Identity("archive parent-receipt digest serialization")
    })?;
    Ok(digest.finish().0)
}

fn stage3_v11_parent_receipt_archive_digest_borrowed_v3(
    receipt: &Stage3V11ParentReceiptRestartWireRefV2<'_>,
    coupled_subslabs: &[Stage3CoupledSubslabReceiptV1],
    terminal_event_groups: &[Stage3V11TerminalEventGroupV1],
) -> Result<Digest32, DirectSnowStage3V11AttachmentError> {
    let mut digest = crate::snow_stage3_v11_attachment::ArchiveDigestCountWriter::default();
    serde_json::to_writer(
        &mut digest,
        &Stage3V11ParentReceiptArchiveDigestInputRefV3 {
            schema: "OPENWEPP_SNOW_STAGE3_V11_PARENT_RECEIPT_ARCHIVE_V3",
            version: 3,
            receipt,
            support_liquid_custody_v2: Stage3SupportLiquidCustodyWireRefV3(coupled_subslabs),
            terminal_liquid_custody_v2: Stage3TerminalLiquidCustodyWireRefV3(
                terminal_event_groups,
            ),
        },
    )
    .map_err(|_| {
        DirectSnowStage3V11AttachmentError::Identity("archive parent-receipt digest serialization")
    })?;
    Ok(digest.finish().0)
}

/// Canonical, checkpoint-backed projection of one complete parent receipt for
/// the committed-day archive. This is default-compiled because archive
/// construction is part of ordinary execution, while admission remains behind
/// the persisted-restart feature.
pub(crate) fn write_stage3_v11_parent_receipt_canonical_v3(
    value: &DirectSnowStage3V11ParentReceipt,
    writer: &mut (impl std::io::Write + ?Sized),
) -> Result<(), DirectSnowStage3V11AttachmentError> {
    for subslab in &value.coupled_subslabs {
        subslab.validate_support_liquid_custody_v2()?;
    }
    for group in &value.terminal_event_groups {
        group.validate_terminal_receiver_custody_v2()?;
    }
    validate_terminal_liquid_custody_set_v3(
        &value.terminal_event_groups,
        &value.coupled_subslabs,
        false,
    )?;
    let receipt = Stage3V11ParentReceiptRestartWireRefV2::project(value);
    let wire = Stage3V11ParentReceiptArchiveWireBorrowedV3 {
        payload_sha256: stage3_v11_parent_receipt_archive_digest_borrowed_v3(
            &receipt,
            &value.coupled_subslabs,
            &value.terminal_event_groups,
        )?,
        schema: "OPENWEPP_SNOW_STAGE3_V11_PARENT_RECEIPT_ARCHIVE_V3",
        version: 3,
        receipt: &receipt,
        support_liquid_custody_v2: Stage3SupportLiquidCustodyWireRefV3(
            &value.coupled_subslabs,
        ),
        terminal_liquid_custody_v2: Stage3TerminalLiquidCustodyWireRefV3(
            &value.terminal_event_groups,
        ),
    };
    serde_json::to_writer(writer, &wire).map_err(|_| {
        DirectSnowStage3V11AttachmentError::Identity(
            "archive parent-receipt canonical serialization",
        )
    })?;
    Ok(())
}

#[cfg(feature = "persisted-restart-v1")]
pub fn restart_authority_decode_parent_receipt_archive_v3(
    bytes: &[u8],
    vegetation_configuration: &VegetationConfigurationV11,
) -> Result<DirectSnowStage3V11ParentReceipt, DirectSnowStage3V11AttachmentError> {
    let wire: Stage3V11ParentReceiptArchiveWireV3 =
        serde_json::from_slice(bytes).map_err(|_| {
            DirectSnowStage3V11AttachmentError::Identity(
                "archive parent-receipt canonical decoding",
            )
        })?;
    if wire.schema != "OPENWEPP_SNOW_STAGE3_V11_PARENT_RECEIPT_ARCHIVE_V3"
        || wire.version != 3
        || wire.payload_sha256
            != stage3_v11_parent_receipt_archive_digest_v3(
                &wire.receipt,
                &wire.support_liquid_custody_v2,
                &wire.terminal_liquid_custody_v2,
            )?
        || serde_json::to_vec(&wire).map_err(|_| {
            DirectSnowStage3V11AttachmentError::Identity(
                "archive parent-receipt canonical re-encoding",
            )
        })? != bytes
    {
        return Err(DirectSnowStage3V11AttachmentError::Identity(
            "archive parent-receipt schema, digest, or canonical bytes",
        ));
    }
    let mut receipt = wire.receipt.restore(vegetation_configuration)?;
    install_support_liquid_custody_v3(
        &mut receipt.coupled_subslabs,
        wire.support_liquid_custody_v2,
    )?;
    install_terminal_liquid_custody_v3(
        &mut receipt.terminal_event_groups,
        wire.terminal_liquid_custody_v2,
    )?;
    validate_support_liquid_event_ordinals_v3(
        &receipt.terminal_event_groups,
        &receipt.coupled_subslabs,
    )?;
    validate_terminal_liquid_custody_set_v3(
        &receipt.terminal_event_groups,
        &receipt.coupled_subslabs,
        false,
    )?;
    if receipt.support_count != STAGE3_V11_PARENT_SUPPORT_COUNT
        || receipt.covered_owner_joins
            != receipt
                .coupled_subslabs
                .iter()
                .map(|subslab| subslab.owner_join.clone())
                .collect::<Vec<_>>()
        || receipt.integrated_boundary_ledger
            != reconstruct_integrated_boundary_ledger(&receipt.coupled_subslabs)
    {
        return Err(DirectSnowStage3V11AttachmentError::Identity(
            "archive parent-receipt intrinsic reconstruction",
        ));
    }
    for subslab in &receipt.coupled_subslabs {
        subslab.validate()?;
    }
    for adaptive in &receipt.adaptive_support_receipts {
        adaptive.validate()?;
    }
    Ok(receipt)
}

#[cfg(feature = "persisted-restart-v1")]
fn project_support_liquid_custody_v3(
    receipts: &[Stage3CoupledSubslabReceiptV1],
) -> Result<Vec<Option<Stage3SupportLiquidCustodyV2>>, DirectSnowStage3V11AttachmentError> {
    receipts
        .iter()
        .map(|receipt| {
            receipt.validate_support_liquid_custody_v2()?;
            Ok(receipt.support_liquid_custody_v2().cloned())
        })
        .collect()
}

#[cfg(feature = "persisted-restart-v1")]
fn install_support_liquid_custody_v3(
    receipts: &mut [Stage3CoupledSubslabReceiptV1],
    custody: Vec<Option<Stage3SupportLiquidCustodyV2>>,
) -> Result<(), DirectSnowStage3V11AttachmentError> {
    if receipts.len() != custody.len() {
        return Err(DirectSnowStage3V11AttachmentError::Identity(
            "restart V3 support-liquid custody cardinality",
        ));
    }
    for (receipt, custody) in receipts.iter_mut().zip(custody) {
        if receipt.support_liquid_custody_v2().is_some() {
            return Err(DirectSnowStage3V11AttachmentError::Identity(
                "restart V3 support-liquid custody target posture",
            ));
        }
        if let Some(custody) = custody {
            receipt.install_support_liquid_custody_v2(custody)?;
        }
        receipt.validate_support_liquid_custody_v2()?;
    }
    Ok(())
}

#[cfg(feature = "persisted-restart-v1")]
fn project_terminal_liquid_custody_v3(
    groups: &[Stage3V11TerminalEventGroupV1],
) -> Result<Vec<Option<Stage3TerminalLiquidCustodyV2>>, DirectSnowStage3V11AttachmentError> {
    groups
        .iter()
        .map(|group| {
            group.validate_terminal_receiver_custody_v2()?;
            Ok(group.terminal_receiver_custody_v2().cloned())
        })
        .collect()
}

#[cfg(feature = "persisted-restart-v1")]
fn install_terminal_liquid_custody_v3(
    groups: &mut [Stage3V11TerminalEventGroupV1],
    custody: Vec<Option<Stage3TerminalLiquidCustodyV2>>,
) -> Result<(), DirectSnowStage3V11AttachmentError> {
    if groups.len() != custody.len() {
        return Err(DirectSnowStage3V11AttachmentError::Identity(
            "restart V3 terminal-liquid custody cardinality",
        ));
    }
    for (group, custody) in groups.iter_mut().zip(custody) {
        if group.terminal_receiver_custody_v2().is_some() {
            return Err(DirectSnowStage3V11AttachmentError::Identity(
                "restart V3 terminal-liquid custody target posture",
            ));
        }
        if let Some(custody) = custody {
            group.install_terminal_receiver_custody_v2(custody)?;
        }
        group.validate_terminal_receiver_custody_v2()?;
    }
    Ok(())
}

#[cfg(feature = "persisted-restart-v1")]
#[derive(Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
struct Stage3V11SupportLiquidCustodyStateWireV3 {
    schema: String,
    version: u16,
    committed: Vec<Vec<Option<Stage3SupportLiquidCustodyV2>>>,
    pending_candidate: Option<Vec<Vec<Option<Stage3SupportLiquidCustodyV2>>>>,
    in_progress_day_candidate: Option<Vec<Vec<Option<Stage3SupportLiquidCustodyV2>>>>,
    in_progress_support_current: Option<Vec<Vec<Option<Stage3SupportLiquidCustodyV2>>>>,
    in_progress_coupled_subslabs: Option<Vec<Option<Stage3SupportLiquidCustodyV2>>>,
    in_progress_support_owner_joins: Option<Vec<Option<Stage3SupportLiquidCustodyV2>>>,
    committed_terminal: Vec<Vec<Option<Stage3TerminalLiquidCustodyV2>>>,
    pending_candidate_terminal: Option<Vec<Vec<Option<Stage3TerminalLiquidCustodyV2>>>>,
    in_progress_day_candidate_terminal: Option<
        Vec<Vec<Option<Stage3TerminalLiquidCustodyV2>>>,
    >,
    in_progress_support_current_terminal: Option<
        Vec<Vec<Option<Stage3TerminalLiquidCustodyV2>>>,
    >,
    in_progress_terminal_event_groups: Option<Vec<Option<Stage3TerminalLiquidCustodyV2>>>,
    in_progress_support_event_groups: Option<Vec<Option<Stage3TerminalLiquidCustodyV2>>>,
    payload_sha256: Digest32,
}

#[cfg(feature = "persisted-restart-v1")]
fn support_liquid_custody_receipt_chain_v3(
    receipts: &[DirectSnowStage3V11ParentReceipt],
) -> Result<Vec<Vec<Option<Stage3SupportLiquidCustodyV2>>>, DirectSnowStage3V11AttachmentError> {
    receipts
        .iter()
        .map(|receipt| project_support_liquid_custody_v3(&receipt.coupled_subslabs))
        .collect()
}

#[cfg(feature = "persisted-restart-v1")]
fn install_support_liquid_custody_receipt_chain_v3(
    receipts: &mut [DirectSnowStage3V11ParentReceipt],
    custody: Vec<Vec<Option<Stage3SupportLiquidCustodyV2>>>,
) -> Result<(), DirectSnowStage3V11AttachmentError> {
    if receipts.len() != custody.len() {
        return Err(DirectSnowStage3V11AttachmentError::Identity(
            "restart V3 support-liquid parent-receipt cardinality",
        ));
    }
    for (receipt, custody) in receipts.iter_mut().zip(custody) {
        install_support_liquid_custody_v3(&mut receipt.coupled_subslabs, custody)?;
        validate_support_liquid_event_ordinals_v3(
            &receipt.terminal_event_groups,
            &receipt.coupled_subslabs,
        )?;
    }
    Ok(())
}

#[cfg(feature = "persisted-restart-v1")]
fn terminal_liquid_custody_receipt_chain_v3(
    receipts: &[DirectSnowStage3V11ParentReceipt],
) -> Result<Vec<Vec<Option<Stage3TerminalLiquidCustodyV2>>>, DirectSnowStage3V11AttachmentError> {
    receipts
        .iter()
        .map(|receipt| project_terminal_liquid_custody_v3(&receipt.terminal_event_groups))
        .collect()
}

#[cfg(feature = "persisted-restart-v1")]
fn install_terminal_liquid_custody_receipt_chain_v3(
    receipts: &mut [DirectSnowStage3V11ParentReceipt],
    custody: Vec<Vec<Option<Stage3TerminalLiquidCustodyV2>>>,
) -> Result<(), DirectSnowStage3V11AttachmentError> {
    if receipts.len() != custody.len() {
        return Err(DirectSnowStage3V11AttachmentError::Identity(
            "restart V3 terminal-liquid parent-receipt cardinality",
        ));
    }
    for (receipt, custody) in receipts.iter_mut().zip(custody) {
        install_terminal_liquid_custody_v3(&mut receipt.terminal_event_groups, custody)?;
        validate_terminal_liquid_custody_set_v3(
            &receipt.terminal_event_groups,
            &receipt.coupled_subslabs,
            false,
        )?;
    }
    Ok(())
}

fn validate_terminal_liquid_custody_set_v3(
    groups: &[Stage3V11TerminalEventGroupV1],
    subslabs: &[Stage3CoupledSubslabReceiptV1],
    allow_last_parent_end_omission: bool,
) -> Result<(), DirectSnowStage3V11AttachmentError> {
    for (index, group) in groups.iter().enumerate() {
        group.validate_terminal_receiver_custody_v2()?;
        let requires_custody = !group.produced_unconsumed_parcels.is_empty()
            && subslabs
                .iter()
                .any(|subslab| subslab.parent_support.end_ns() == group.tick);
        let allowed_pending = allow_last_parent_end_omission
            && index.checked_add(1) == Some(groups.len())
            && requires_custody
            && group.terminal_receiver_custody_v2().is_none();
        if group.terminal_receiver_custody_v2().is_some() != requires_custody
            && !allowed_pending
        {
            return Err(DirectSnowStage3V11AttachmentError::Identity(
                "restart V3 terminal-liquid custody required posture",
            ));
        }
    }
    Ok(())
}

#[cfg(feature = "persisted-restart-v1")]
fn validate_support_liquid_event_ordinals_v3(
    terminal_groups: &[Stage3V11TerminalEventGroupV1],
    receipts: &[Stage3CoupledSubslabReceiptV1],
) -> Result<(), DirectSnowStage3V11AttachmentError> {
    let mut ordinals = BTreeMap::<Digest32, Vec<u32>>::new();
    for accepted in terminal_groups
        .iter()
        .filter_map(|group| group.accepted_event_receipt.as_ref())
        .chain(
            receipts
                .iter()
                .filter_map(|receipt| receipt.post_support_liquid_receiver_event.as_ref()),
        )
    {
        accepted.validate()?;
        ordinals
            .entry(accepted.parent_transaction_id().digest())
            .or_default()
            .push(accepted.ordinal());
    }
    for values in ordinals.values_mut() {
        values.sort_unstable();
        if values
            .iter()
            .enumerate()
            .any(|(expected, actual)| u32::try_from(expected).ok() != Some(*actual))
        {
            return Err(DirectSnowStage3V11AttachmentError::Identity(
                "restart V3 support-liquid accepted-event ordinal",
            ));
        }
    }
    Ok(())
}

#[cfg(feature = "persisted-restart-v1")]
fn support_liquid_custody_state_digest_v3(
    wire: &Stage3V11SupportLiquidCustodyStateWireV3,
) -> Result<Digest32, DirectSnowStage3V11AttachmentError> {
    let mut projection = Stage3V11SupportLiquidCustodyStateWireV3 {
        schema: wire.schema.clone(),
        version: wire.version,
        committed: wire.committed.clone(),
        pending_candidate: wire.pending_candidate.clone(),
        in_progress_day_candidate: wire.in_progress_day_candidate.clone(),
        in_progress_support_current: wire.in_progress_support_current.clone(),
        in_progress_coupled_subslabs: wire.in_progress_coupled_subslabs.clone(),
        in_progress_support_owner_joins: wire.in_progress_support_owner_joins.clone(),
        committed_terminal: wire.committed_terminal.clone(),
        pending_candidate_terminal: wire.pending_candidate_terminal.clone(),
        in_progress_day_candidate_terminal: wire.in_progress_day_candidate_terminal.clone(),
        in_progress_support_current_terminal: wire.in_progress_support_current_terminal.clone(),
        in_progress_terminal_event_groups: wire.in_progress_terminal_event_groups.clone(),
        in_progress_support_event_groups: wire.in_progress_support_event_groups.clone(),
        payload_sha256: Digest32::zero(),
    };
    projection.payload_sha256 = Digest32::zero();
    let bytes = serde_json::to_vec(&projection).map_err(|_| {
        DirectSnowStage3V11AttachmentError::Identity(
            "restart V3 support-liquid custody serialization",
        )
    })?;
    Ok(digest_bytes(&bytes))
}

#[cfg(feature = "persisted-restart-v1")]
pub fn restart_authority_encode_support_liquid_custody_state_v3(
    value: &DirectSnowStage3V11ShadowAttachment,
) -> Result<Vec<u8>, DirectSnowStage3V11AttachmentError> {
    let in_progress = value.in_progress_execution.as_deref();
    let mut wire = Stage3V11SupportLiquidCustodyStateWireV3 {
        schema: "OPENWEPP_SNOW_STAGE3_V11_SUPPORT_LIQUID_CUSTODY_RESTART_V3".into(),
        version: 3,
        committed: support_liquid_custody_receipt_chain_v3(&value.committed.receipt_chain)?,
        pending_candidate: value
            .pending_candidate
            .as_ref()
            .map(|pending| {
                support_liquid_custody_receipt_chain_v3(&pending.ending_state.receipt_chain)
            })
            .transpose()?,
        in_progress_day_candidate: in_progress
            .map(|execution| {
                support_liquid_custody_receipt_chain_v3(&execution.day_candidate.receipt_chain)
            })
            .transpose()?,
        in_progress_support_current: in_progress
            .map(|execution| {
                support_liquid_custody_receipt_chain_v3(
                    &execution
                        .support_current
                        .as_ref()
                        .ok_or(DirectSnowStage3V11AttachmentError::Identity(
                            "restart V3 support-liquid current owner",
                        ))?
                        .receipt_chain,
                )
            })
            .transpose()?,
        in_progress_coupled_subslabs: in_progress
            .map(|execution| project_support_liquid_custody_v3(&execution.coupled_subslabs))
            .transpose()?,
        in_progress_support_owner_joins: in_progress
            .map(|execution| project_support_liquid_custody_v3(&execution.support_owner_joins))
            .transpose()?,
        committed_terminal: terminal_liquid_custody_receipt_chain_v3(
            &value.committed.receipt_chain,
        )?,
        pending_candidate_terminal: value
            .pending_candidate
            .as_ref()
            .map(|pending| {
                terminal_liquid_custody_receipt_chain_v3(&pending.ending_state.receipt_chain)
            })
            .transpose()?,
        in_progress_day_candidate_terminal: in_progress
            .map(|execution| {
                terminal_liquid_custody_receipt_chain_v3(&execution.day_candidate.receipt_chain)
            })
            .transpose()?,
        in_progress_support_current_terminal: in_progress
            .map(|execution| {
                terminal_liquid_custody_receipt_chain_v3(
                    &execution
                        .support_current
                        .as_ref()
                        .ok_or(DirectSnowStage3V11AttachmentError::Identity(
                            "restart V3 terminal-liquid current owner",
                        ))?
                        .receipt_chain,
                )
            })
            .transpose()?,
        in_progress_terminal_event_groups: in_progress
            .map(|execution| {
                project_terminal_liquid_custody_v3(&execution.terminal_event_groups)
            })
            .transpose()?,
        in_progress_support_event_groups: in_progress
            .map(|execution| project_terminal_liquid_custody_v3(&execution.support_event_groups))
            .transpose()?,
        payload_sha256: Digest32::zero(),
    };
    wire.payload_sha256 = support_liquid_custody_state_digest_v3(&wire)?;
    serde_json::to_vec(&wire).map_err(|_| {
        DirectSnowStage3V11AttachmentError::Identity(
            "restart V3 support-liquid custody serialization",
        )
    })
}

#[cfg(all(
    feature = "persisted-restart-v1",
    feature = "restart-authority-evidence"
))]
#[derive(Clone, Copy, Debug)]
pub enum RestartAuthoritySupportLiquidCustodyPoisonV3 {
    LseBeginningSubstitution,
    LseEndingSubstitution,
    RunoffRouteTopologySubstitution,
    RunoffDispositionSubstitution,
}

#[cfg(all(
    feature = "persisted-restart-v1",
    feature = "restart-authority-evidence"
))]
fn reseal_support_liquid_custody_poison_v3(
    custody: &mut Stage3SupportLiquidCustodyV2,
) -> Result<(), DirectSnowStage3V11AttachmentError> {
    use sha2::{Digest as _, Sha256};

    let mut predecessor = [0_u8; 32];
    for receipt in &mut custody.receiver_receipts {
        receipt.predecessor_receipt_sha256 = predecessor;
        receipt.receipt_sha256 = [0; 32];
        let canonical = serde_json::to_vec(receipt).map_err(|_| {
            DirectSnowStage3V11AttachmentError::Identity(
                "restart V3 support-liquid poison receipt serialization",
            )
        })?;
        let canonical_len = u64::try_from(canonical.len()).map_err(|_| {
            DirectSnowStage3V11AttachmentError::Identity(
                "restart V3 support-liquid poison receipt length",
            )
        })?;
        let mut hasher = Sha256::new();
        hasher.update(canonical_len.to_be_bytes());
        hasher.update(&canonical);
        receipt.receipt_sha256 = hasher.finalize().into();
        predecessor = receipt.receipt_sha256;
    }
    custody.receiver_receipt_set_sha256 = Digest32::from_bytes(
        crate::zero_duration_snow_liquid_receipt_set_sha256(&custody.receiver_receipts)
            .map_err(|_| {
                DirectSnowStage3V11AttachmentError::Identity(
                    "restart V3 support-liquid poison receipt-set reseal",
                )
            })?,
    );
    custody.custody_sha256 = Digest32::zero();
    custody.custody_sha256 = custody.reconstructed_digest()?;
    Ok(())
}

#[cfg(all(
    feature = "persisted-restart-v1",
    feature = "restart-authority-evidence"
))]
pub fn restart_authority_poison_support_liquid_custody_state_v3(
    bytes: &[u8],
    poison: RestartAuthoritySupportLiquidCustodyPoisonV3,
) -> Result<Vec<u8>, DirectSnowStage3V11AttachmentError> {
    let mut wire: Stage3V11SupportLiquidCustodyStateWireV3 =
        serde_json::from_slice(bytes).map_err(|_| {
            DirectSnowStage3V11AttachmentError::Identity(
                "restart V3 support-liquid custody poison decoding",
            )
        })?;
    let mut custody = wire
        .committed
        .iter_mut()
        .flatten()
        .find_map(Option::as_mut);
    if custody.is_none() {
        custody = wire
            .pending_candidate
            .iter_mut()
            .flatten()
            .flatten()
            .find_map(Option::as_mut);
    }
    if custody.is_none() {
        custody = wire
            .in_progress_day_candidate
            .iter_mut()
            .flatten()
            .flatten()
            .find_map(Option::as_mut);
    }
    if custody.is_none() {
        custody = wire
            .in_progress_support_current
            .iter_mut()
            .flatten()
            .flatten()
            .find_map(Option::as_mut);
    }
    if custody.is_none() {
        custody = wire
            .in_progress_coupled_subslabs
            .iter_mut()
            .flatten()
            .find_map(Option::as_mut);
    }
    if custody.is_none() {
        custody = wire
            .in_progress_support_owner_joins
            .iter_mut()
            .flatten()
            .find_map(Option::as_mut);
    }
    let custody = custody.ok_or(DirectSnowStage3V11AttachmentError::Identity(
        "restart V3 support-liquid custody poison target",
    ))?;
    let tiles = match poison {
        RestartAuthoritySupportLiquidCustodyPoisonV3::LseBeginningSubstitution => {
            Some(&mut custody.lse_beginning_state.tiles)
        }
        RestartAuthoritySupportLiquidCustodyPoisonV3::LseEndingSubstitution => {
            Some(&mut custody.lse_ending_state.tiles)
        }
        RestartAuthoritySupportLiquidCustodyPoisonV3::RunoffRouteTopologySubstitution
        | RestartAuthoritySupportLiquidCustodyPoisonV3::RunoffDispositionSubstitution => None,
    };
    if let Some(tiles) = tiles {
        let tile = tiles
            .first_mut()
            .ok_or(DirectSnowStage3V11AttachmentError::Identity(
                "restart V3 support-liquid custody poison LSE tile",
            ))?;
        tile.surface_enthalpy_j_m2_tile_ground = f64::from_bits(
            tile.surface_enthalpy_j_m2_tile_ground.to_bits() ^ 1,
        );
    } else {
        let receipt = custody
            .receiver_receipts
            .first_mut()
            .ok_or(DirectSnowStage3V11AttachmentError::Identity(
                "restart V3 support-liquid custody poison receipt",
            ))?;
        match poison {
            RestartAuthoritySupportLiquidCustodyPoisonV3::RunoffRouteTopologySubstitution => {
                receipt.recipient_ofe_id = OfeId::try_new(format!(
                    "{}-restart-poison",
                    receipt.recipient_ofe_id
                ))
                .map_err(|_| {
                    DirectSnowStage3V11AttachmentError::Identity(
                        "restart V3 support-liquid custody poison route identity",
                    )
                })?;
            }
            RestartAuthoritySupportLiquidCustodyPoisonV3::RunoffDispositionSubstitution => {
                match receipt.disposition {
                    crate::DirectZeroDurationSnowLiquidDispositionV1::RetainedSurface => {
                        receipt.disposition =
                            crate::DirectZeroDurationSnowLiquidDispositionV1::RoutedRunoff;
                        receipt.credited_mass_kg_m2_recipient_tile_ground = None;
                        receipt.credited_enthalpy_j_m2_recipient_tile_ground = None;
                    }
                    crate::DirectZeroDurationSnowLiquidDispositionV1::RoutedRunoff => {
                        receipt.disposition =
                            crate::DirectZeroDurationSnowLiquidDispositionV1::OutletRunoff;
                        receipt.recipient_tile_id = None;
                        receipt.recipient_tile_fraction = None;
                    }
                    crate::DirectZeroDurationSnowLiquidDispositionV1::OutletRunoff => {
                        receipt.disposition =
                            crate::DirectZeroDurationSnowLiquidDispositionV1::RoutedRunoff;
                        receipt.recipient_ofe_id = receipt.origin_ofe_id.clone();
                        receipt.recipient_tile_id = Some(receipt.origin_tile_id.clone());
                        receipt.recipient_tile_fraction = Some(1.0);
                    }
                }
            }
            RestartAuthoritySupportLiquidCustodyPoisonV3::LseBeginningSubstitution
            | RestartAuthoritySupportLiquidCustodyPoisonV3::LseEndingSubstitution => {
                return Err(DirectSnowStage3V11AttachmentError::Identity(
                    "restart V3 support-liquid custody poison dispatch",
                ));
            }
        }
    }
    reseal_support_liquid_custody_poison_v3(custody)?;
    wire.payload_sha256 = support_liquid_custody_state_digest_v3(&wire)?;
    serde_json::to_vec(&wire).map_err(|_| {
        DirectSnowStage3V11AttachmentError::Identity(
            "restart V3 support-liquid custody poison serialization",
        )
    })
}

#[cfg(all(
    feature = "persisted-restart-v1",
    feature = "restart-authority-evidence"
))]
#[derive(Clone, Copy, Debug)]
pub enum RestartAuthorityTerminalLiquidCustodyPoisonV3 {
    LseBeginningSubstitution,
    LseEndingSubstitution,
    RunoffRouteTopologySubstitution,
    RunoffDispositionSubstitution,
}

#[cfg(all(
    feature = "persisted-restart-v1",
    feature = "restart-authority-evidence"
))]
fn reseal_terminal_liquid_custody_poison_v3(
    custody: &mut Stage3TerminalLiquidCustodyV2,
) -> Result<(), DirectSnowStage3V11AttachmentError> {
    use sha2::{Digest as _, Sha256};

    let mut predecessor = [0_u8; 32];
    for receipt in &mut custody.receiver_receipts {
        receipt.predecessor_receipt_sha256 = predecessor;
        receipt.receipt_sha256 = [0; 32];
        let canonical = serde_json::to_vec(receipt).map_err(|_| {
            DirectSnowStage3V11AttachmentError::Identity(
                "restart V3 terminal-liquid poison receipt serialization",
            )
        })?;
        let canonical_len = u64::try_from(canonical.len()).map_err(|_| {
            DirectSnowStage3V11AttachmentError::Identity(
                "restart V3 terminal-liquid poison receipt length",
            )
        })?;
        let mut hasher = Sha256::new();
        hasher.update(canonical_len.to_be_bytes());
        hasher.update(&canonical);
        receipt.receipt_sha256 = hasher.finalize().into();
        predecessor = receipt.receipt_sha256;
    }
    custody.receiver_receipt_set_sha256 = Digest32::from_bytes(
        crate::zero_duration_snow_liquid_receipt_set_sha256(&custody.receiver_receipts)
            .map_err(|_| {
                DirectSnowStage3V11AttachmentError::Identity(
                    "restart V3 terminal-liquid poison receipt-set reseal",
                )
            })?,
    );
    custody.custody_sha256 = Digest32::zero();
    custody.custody_sha256 = custody.reconstructed_digest()?;
    Ok(())
}

#[cfg(all(
    feature = "persisted-restart-v1",
    feature = "restart-authority-evidence"
))]
pub fn restart_authority_poison_terminal_liquid_custody_state_v3(
    bytes: &[u8],
    poison: RestartAuthorityTerminalLiquidCustodyPoisonV3,
) -> Result<Vec<u8>, DirectSnowStage3V11AttachmentError> {
    let mut wire: Stage3V11SupportLiquidCustodyStateWireV3 =
        serde_json::from_slice(bytes).map_err(|_| {
            DirectSnowStage3V11AttachmentError::Identity(
                "restart V3 terminal-liquid custody poison decoding",
            )
        })?;
    let mut custody = wire
        .committed_terminal
        .iter_mut()
        .flatten()
        .find_map(Option::as_mut);
    for collection in [
        &mut wire.pending_candidate_terminal,
        &mut wire.in_progress_day_candidate_terminal,
        &mut wire.in_progress_support_current_terminal,
    ] {
        if custody.is_none() {
            custody = collection
                .iter_mut()
                .flatten()
                .flatten()
                .find_map(Option::as_mut);
        }
    }
    if custody.is_none() {
        custody = wire
            .in_progress_terminal_event_groups
            .iter_mut()
            .flatten()
            .find_map(Option::as_mut);
    }
    if custody.is_none() {
        custody = wire
            .in_progress_support_event_groups
            .iter_mut()
            .flatten()
            .find_map(Option::as_mut);
    }
    let custody = custody.ok_or(DirectSnowStage3V11AttachmentError::Identity(
        "restart V3 terminal-liquid custody poison target",
    ))?;
    match poison {
        RestartAuthorityTerminalLiquidCustodyPoisonV3::LseBeginningSubstitution
        | RestartAuthorityTerminalLiquidCustodyPoisonV3::LseEndingSubstitution => {
            let tiles = if matches!(
                poison,
                RestartAuthorityTerminalLiquidCustodyPoisonV3::LseBeginningSubstitution
            ) {
                &mut custody.lse_beginning_state.tiles
            } else {
                &mut custody.lse_ending_state.tiles
            };
            let tile = tiles
                .first_mut()
                .ok_or(DirectSnowStage3V11AttachmentError::Identity(
                    "restart V3 terminal-liquid custody poison LSE tile",
                ))?;
            tile.surface_enthalpy_j_m2_tile_ground =
                if tile.surface_enthalpy_j_m2_tile_ground == 0.0 {
                    1.0
                } else {
                    0.0
                };
        }
        RestartAuthorityTerminalLiquidCustodyPoisonV3::RunoffRouteTopologySubstitution
        | RestartAuthorityTerminalLiquidCustodyPoisonV3::RunoffDispositionSubstitution => {
            let receipt = custody
                .receiver_receipts
                .first_mut()
                .ok_or(DirectSnowStage3V11AttachmentError::Identity(
                    "restart V3 terminal-liquid custody poison receipt",
                ))?;
            if matches!(
                poison,
                RestartAuthorityTerminalLiquidCustodyPoisonV3::RunoffRouteTopologySubstitution
            ) {
                receipt.recipient_ofe_id = OfeId::try_new(format!(
                    "{}-restart-poison",
                    receipt.recipient_ofe_id
                ))
                .map_err(|_| {
                    DirectSnowStage3V11AttachmentError::Identity(
                        "restart V3 terminal-liquid custody poison route identity",
                    )
                })?;
            } else {
                receipt.disposition = match receipt.disposition {
                    crate::DirectZeroDurationSnowLiquidDispositionV1::RetainedSurface => {
                        receipt.credited_mass_kg_m2_recipient_tile_ground = None;
                        receipt.credited_enthalpy_j_m2_recipient_tile_ground = None;
                        crate::DirectZeroDurationSnowLiquidDispositionV1::RoutedRunoff
                    }
                    crate::DirectZeroDurationSnowLiquidDispositionV1::RoutedRunoff => {
                        receipt.recipient_tile_id = None;
                        receipt.recipient_tile_fraction = None;
                        crate::DirectZeroDurationSnowLiquidDispositionV1::OutletRunoff
                    }
                    crate::DirectZeroDurationSnowLiquidDispositionV1::OutletRunoff => {
                        receipt.recipient_tile_id = Some(receipt.origin_tile_id.clone());
                        receipt.recipient_tile_fraction = Some(1.0);
                        crate::DirectZeroDurationSnowLiquidDispositionV1::RoutedRunoff
                    }
                };
            }
        }
    }
    reseal_terminal_liquid_custody_poison_v3(custody)?;
    wire.payload_sha256 = support_liquid_custody_state_digest_v3(&wire)?;
    serde_json::to_vec(&wire).map_err(|_| {
        DirectSnowStage3V11AttachmentError::Identity(
            "restart V3 terminal-liquid custody poison serialization",
        )
    })
}

#[cfg(feature = "persisted-restart-v1")]
pub fn restart_authority_restore_support_liquid_custody_state_v3(
    value: &mut DirectSnowStage3V11ShadowAttachment,
    bytes: &[u8],
) -> Result<(), DirectSnowStage3V11AttachmentError> {
    let wire: Stage3V11SupportLiquidCustodyStateWireV3 =
        serde_json::from_slice(bytes).map_err(|_| {
            DirectSnowStage3V11AttachmentError::Identity(
                "restart V3 support-liquid custody decoding",
            )
        })?;
    if wire.schema != "OPENWEPP_SNOW_STAGE3_V11_SUPPORT_LIQUID_CUSTODY_RESTART_V3"
        || wire.version != 3
        || wire.payload_sha256 != support_liquid_custody_state_digest_v3(&wire)?
        || serde_json::to_vec(&wire).map_err(|_| {
            DirectSnowStage3V11AttachmentError::Identity(
                "restart V3 support-liquid custody re-encoding",
            )
        })? != bytes
    {
        return Err(DirectSnowStage3V11AttachmentError::Identity(
            "restart V3 support-liquid custody schema, seal, or canonical bytes",
        ));
    }
    let mut candidate = value.clone();
    install_support_liquid_custody_receipt_chain_v3(
        &mut candidate.committed.receipt_chain,
        wire.committed,
    )?;
    install_terminal_liquid_custody_receipt_chain_v3(
        &mut candidate.committed.receipt_chain,
        wire.committed_terminal,
    )?;
    match (
        &mut candidate.pending_candidate,
        wire.pending_candidate,
        wire.pending_candidate_terminal,
    ) {
        (Some(pending), Some(custody), Some(terminal_custody)) => {
            install_support_liquid_custody_receipt_chain_v3(
                &mut pending.ending_state.receipt_chain,
                custody,
            )?;
            install_terminal_liquid_custody_receipt_chain_v3(
                &mut pending.ending_state.receipt_chain,
                terminal_custody,
            )?;
        }
        (None, None, None) => {}
        _ => {
            return Err(DirectSnowStage3V11AttachmentError::Identity(
                "restart V3 support-liquid pending posture",
            ));
        }
    }
    match (
        candidate.in_progress_execution.as_deref_mut(),
        wire.in_progress_day_candidate,
        wire.in_progress_support_current,
        wire.in_progress_coupled_subslabs,
        wire.in_progress_support_owner_joins,
        wire.in_progress_day_candidate_terminal,
        wire.in_progress_support_current_terminal,
        wire.in_progress_terminal_event_groups,
        wire.in_progress_support_event_groups,
    ) {
        (
            Some(execution),
            Some(day),
            Some(current),
            Some(coupled),
            Some(owner_joins),
            Some(day_terminal),
            Some(current_terminal),
            Some(terminal_groups),
            Some(support_terminal_groups),
        ) => {
            install_support_liquid_custody_receipt_chain_v3(
                &mut execution.day_candidate.receipt_chain,
                day,
            )?;
            install_terminal_liquid_custody_receipt_chain_v3(
                &mut execution.day_candidate.receipt_chain,
                day_terminal,
            )?;
            install_support_liquid_custody_receipt_chain_v3(
                &mut execution
                    .support_current
                    .as_mut()
                    .ok_or(DirectSnowStage3V11AttachmentError::Identity(
                        "restart V3 support-liquid current posture",
                    ))?
                    .receipt_chain,
                current,
            )?;
            install_terminal_liquid_custody_receipt_chain_v3(
                &mut execution
                    .support_current
                    .as_mut()
                    .ok_or(DirectSnowStage3V11AttachmentError::Identity(
                        "restart V3 terminal-liquid current posture",
                    ))?
                    .receipt_chain,
                current_terminal,
            )?;
            install_support_liquid_custody_v3(&mut execution.coupled_subslabs, coupled)?;
            install_support_liquid_custody_v3(&mut execution.support_owner_joins, owner_joins)?;
            install_terminal_liquid_custody_v3(
                &mut execution.terminal_event_groups,
                terminal_groups,
            )?;
            install_terminal_liquid_custody_v3(
                &mut execution.support_event_groups,
                support_terminal_groups,
            )?;
            validate_support_liquid_event_ordinals_v3(
                &execution.terminal_event_groups,
                &execution.coupled_subslabs,
            )?;
            validate_terminal_liquid_custody_set_v3(
                &execution.terminal_event_groups,
                &execution.coupled_subslabs,
                matches!(
                    execution.posture,
                    DirectSnowStage3V11InterruptionPostureV2::AfterTerminalEvent
                        | DirectSnowStage3V11InterruptionPostureV2::BeforeTerminalReceiver
                ),
            )?;
            validate_support_liquid_event_ordinals_v3(
                &execution.support_event_groups,
                &execution.support_owner_joins,
            )?;
            validate_terminal_liquid_custody_set_v3(
                &execution.support_event_groups,
                &execution.support_owner_joins,
                matches!(
                    execution.posture,
                    DirectSnowStage3V11InterruptionPostureV2::AfterTerminalEvent
                        | DirectSnowStage3V11InterruptionPostureV2::BeforeTerminalReceiver
                ),
            )?;
        }
        (None, None, None, None, None, None, None, None, None) => {}
        _ => {
            return Err(DirectSnowStage3V11AttachmentError::Identity(
                "restart V3 support-liquid in-progress posture",
            ));
        }
    }
    *value = candidate;
    Ok(())
}

#[cfg(feature = "persisted-restart-v1")]
#[derive(Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
struct Stage3V11PublicationRotationStateWireV3 {
    schema: String,
    version: u16,
    committed: Vec<u8>,
    pending_candidate: Option<Vec<u8>>,
    in_progress_day_candidate: Option<Vec<u8>>,
    in_progress_support_current: Option<Vec<u8>>,
    payload_sha256: Digest32,
}

#[cfg(feature = "persisted-restart-v1")]
#[derive(Serialize)]
struct Stage3V11PublicationRotationStateDigestInputV3<'a> {
    schema: &'a str,
    version: u16,
    committed: &'a [u8],
    pending_candidate: &'a Option<Vec<u8>>,
    in_progress_day_candidate: &'a Option<Vec<u8>>,
    in_progress_support_current: &'a Option<Vec<u8>>,
}

#[cfg(feature = "persisted-restart-v1")]
fn publication_rotation_state_digest_v3(
    wire: &Stage3V11PublicationRotationStateWireV3,
) -> Result<Digest32, DirectSnowStage3V11AttachmentError> {
    let bytes = serde_json::to_vec(&Stage3V11PublicationRotationStateDigestInputV3 {
        schema: &wire.schema,
        version: wire.version,
        committed: &wire.committed,
        pending_candidate: &wire.pending_candidate,
        in_progress_day_candidate: &wire.in_progress_day_candidate,
        in_progress_support_current: &wire.in_progress_support_current,
    })
    .map_err(|_| {
        DirectSnowStage3V11AttachmentError::Identity(
            "restart V3 publication-rotation digest serialization",
        )
    })?;
    Ok(digest_bytes(&bytes))
}

#[cfg(feature = "persisted-restart-v1")]
pub fn restart_authority_encode_publication_rotation_state_v3(
    value: &DirectSnowStage3V11ShadowAttachment,
) -> Result<Vec<u8>, DirectSnowStage3V11AttachmentError> {
    let in_progress = value.in_progress_execution.as_deref();
    let mut wire = Stage3V11PublicationRotationStateWireV3 {
        schema: "OPENWEPP_SNOW_STAGE3_V11_PUBLICATION_ROTATION_RESTART_V3".into(),
        version: 3,
        committed: value
            .committed
            .real_consumer
            .restart_authority_accepted_publication_rotation_canonical_bytes_v3()
            .map_err(|_| {
                DirectSnowStage3V11AttachmentError::Identity(
                    "restart V3 committed publication rotation projection",
                )
            })?,
        pending_candidate: value
            .pending_candidate
            .as_ref()
            .map(|candidate| {
                candidate
                    .ending_state
                    .real_consumer
                    .restart_authority_accepted_publication_rotation_canonical_bytes_v3()
                    .map_err(|_| {
                        DirectSnowStage3V11AttachmentError::Identity(
                            "restart V3 pending publication rotation projection",
                        )
                    })
            })
            .transpose()?,
        in_progress_day_candidate: in_progress
            .map(|execution| {
                execution
                    .day_candidate
                    .real_consumer
                    .restart_authority_accepted_publication_rotation_canonical_bytes_v3()
                    .map_err(|_| {
                        DirectSnowStage3V11AttachmentError::Identity(
                            "restart V3 in-progress day publication rotation projection",
                        )
                    })
            })
            .transpose()?,
        in_progress_support_current: in_progress
            .and_then(|execution| execution.support_current.as_ref())
            .map(|state| {
                state
                    .real_consumer
                    .restart_authority_accepted_publication_rotation_canonical_bytes_v3()
                    .map_err(|_| {
                        DirectSnowStage3V11AttachmentError::Identity(
                            "restart V3 in-progress support publication rotation projection",
                        )
                    })
            })
            .transpose()?,
        payload_sha256: Digest32::zero(),
    };
    if wire.pending_candidate.is_some() && wire.in_progress_day_candidate.is_some()
        || wire.in_progress_day_candidate.is_some() != wire.in_progress_support_current.is_some()
    {
        return Err(DirectSnowStage3V11AttachmentError::Identity(
            "restart V3 publication rotation posture cardinality",
        ));
    }
    wire.payload_sha256 = publication_rotation_state_digest_v3(&wire)?;
    serde_json::to_vec(&wire).map_err(|_| {
        DirectSnowStage3V11AttachmentError::Identity(
            "restart V3 publication-rotation canonical serialization",
        )
    })
}

#[cfg(feature = "persisted-restart-v1")]
pub fn restart_authority_restore_publication_rotation_state_v3(
    value: &mut DirectSnowStage3V11ShadowAttachment,
    bytes: &[u8],
) -> Result<(), DirectSnowStage3V11AttachmentError> {
    let wire: Stage3V11PublicationRotationStateWireV3 =
        serde_json::from_slice(bytes).map_err(|_| {
            DirectSnowStage3V11AttachmentError::Identity(
                "restart V3 publication-rotation canonical decoding",
            )
        })?;
    if wire.schema != "OPENWEPP_SNOW_STAGE3_V11_PUBLICATION_ROTATION_RESTART_V3"
        || wire.version != 3
        || wire.payload_sha256 != publication_rotation_state_digest_v3(&wire)?
        || serde_json::to_vec(&wire).map_err(|_| {
            DirectSnowStage3V11AttachmentError::Identity(
                "restart V3 publication-rotation canonical re-encoding",
            )
        })? != bytes
        || wire.pending_candidate.is_some() != value.pending_candidate.is_some()
        || wire.in_progress_day_candidate.is_some() != value.in_progress_execution.is_some()
        || wire.in_progress_support_current.is_some() != value.in_progress_execution.is_some()
    {
        return Err(DirectSnowStage3V11AttachmentError::Identity(
            "restart V3 publication rotation schema, seal, posture, or canonical bytes",
        ));
    }
    let mut candidate = value.clone();
    candidate
        .committed
        .real_consumer
        .restart_authority_restore_accepted_publication_rotation_canonical_bytes_v3(&wire.committed)
        .map_err(|_| {
            DirectSnowStage3V11AttachmentError::Identity(
                "restart V3 committed publication rotation admission",
            )
        })?;
    if let (Some(state), Some(rotation)) =
        (candidate.pending_candidate.as_mut(), wire.pending_candidate)
    {
        state
            .ending_state
            .real_consumer
            .restart_authority_restore_accepted_publication_rotation_canonical_bytes_v3(&rotation)
            .map_err(|_| {
                DirectSnowStage3V11AttachmentError::Identity(
                    "restart V3 pending publication rotation admission",
                )
            })?;
    }
    if let (Some(execution), Some(day_rotation), Some(support_rotation)) = (
        candidate.in_progress_execution.as_deref_mut(),
        wire.in_progress_day_candidate,
        wire.in_progress_support_current,
    ) {
        execution
            .day_candidate
            .real_consumer
            .restart_authority_restore_accepted_publication_rotation_canonical_bytes_v3(
                &day_rotation,
            )
            .map_err(|_| {
                DirectSnowStage3V11AttachmentError::Identity(
                    "restart V3 in-progress day publication rotation admission",
                )
            })?;
        execution
            .support_current
            .as_mut()
            .ok_or(DirectSnowStage3V11AttachmentError::Identity(
                "restart V3 in-progress support owner",
            ))?
            .real_consumer
            .restart_authority_restore_accepted_publication_rotation_canonical_bytes_v3(
                &support_rotation,
            )
            .map_err(|_| {
                DirectSnowStage3V11AttachmentError::Identity(
                    "restart V3 in-progress support publication rotation admission",
                )
            })?;
    }
    let validate_publication_owner = |state: &DirectSnowStage3V11CommittedState,
                                      defer_pre_support_reappearance: bool|
     -> Result<(), DirectSnowStage3V11AttachmentError> {
        if let Some(publication_ending) = state
            .real_consumer
            .restart_authority_accepted_publication_traversed_ending_owner_sha256()
            .map_err(|_| {
                DirectSnowStage3V11AttachmentError::Identity(
                    "restart V3 publication ending owner traversal",
                )
            })?
            && !defer_pre_support_reappearance
            && publication_ending != complete_owner_set_digest(state.coupled_clock.owners())?
        {
            return Err(DirectSnowStage3V11AttachmentError::Identity(
                "restart V3 publication/current coupled owner join",
            ));
        }
        Ok(())
    };
    validate_publication_owner(&candidate.committed, false)?;
    if let Some(pending) = &candidate.pending_candidate {
        validate_publication_owner(&pending.ending_state, false)?;
    }
    if let Some(execution) = candidate.in_progress_execution.as_deref() {
        validate_publication_owner(&execution.day_candidate, false)?;
        validate_publication_owner(
            execution.support_current.as_ref().ok_or(
                DirectSnowStage3V11AttachmentError::Identity(
                    "restart V3 in-progress publication support owner",
                ),
            )?,
            execution.posture == DirectSnowStage3V11InterruptionPostureV2::AfterSnowReappearance,
        )?;
    }
    *value = candidate;
    Ok(())
}

#[cfg(feature = "persisted-restart-v1")]
fn restart_receipt_state_digest_v2(
    terminal_parcels: &BTreeMap<Digest32, DirectSnowStage3V11TerminalParcel>,
    receipt_chain: &[Stage3V11ParentReceiptRestartWireV2],
) -> Result<Digest32, DirectSnowStage3V11AttachmentError> {
    let bytes = serde_json::to_vec(&Stage3V11ReceiptStateDigestInput {
        schema: "OPENWEPP_SNOW_STAGE3_V11_RECEIPT_STATE_RESTART_V2",
        version: 2,
        terminal_parcels,
        receipt_chain,
    })
    .map_err(|_| {
        DirectSnowStage3V11AttachmentError::Identity("restart receipt-state serialization")
    })?;
    Ok(digest_bytes(&bytes))
}

#[cfg(feature = "persisted-restart-v1")]
pub fn restart_authority_encode_receipt_state_v2(
    terminal_parcels: &BTreeMap<Digest32, DirectSnowStage3V11TerminalParcel>,
    receipt_chain: &[DirectSnowStage3V11ParentReceipt],
) -> Result<Vec<u8>, DirectSnowStage3V11AttachmentError> {
    if receipt_chain.iter().any(|receipt| {
        receipt
            .coupled_subslabs
            .iter()
            .any(|subslab| subslab.support_liquid_custody_v2().is_some())
            || receipt
                .terminal_event_groups
                .iter()
                .any(|group| group.terminal_receiver_custody_v2().is_some())
    }) {
        return Err(DirectSnowStage3V11AttachmentError::Identity(
            "V2 cannot omit zero-duration liquid custody V2",
        ));
    }
    restart_authority_encode_receipt_state_base_v3(terminal_parcels, receipt_chain)
}

#[cfg(feature = "persisted-restart-v1")]
pub fn restart_authority_encode_receipt_state_base_v3(
    terminal_parcels: &BTreeMap<Digest32, DirectSnowStage3V11TerminalParcel>,
    receipt_chain: &[DirectSnowStage3V11ParentReceipt],
) -> Result<Vec<u8>, DirectSnowStage3V11AttachmentError> {
    let receipt_chain = receipt_chain
        .iter()
        .map(Stage3V11ParentReceiptRestartWireV2::project)
        .collect::<Vec<_>>();
    let wire = Stage3V11ReceiptStateRestartWireV2 {
        schema: "OPENWEPP_SNOW_STAGE3_V11_RECEIPT_STATE_RESTART_V2".into(),
        version: 2,
        terminal_parcels: terminal_parcels.clone(),
        receipt_chain,
        payload_sha256: Digest32::zero(),
    };
    let wire = Stage3V11ReceiptStateRestartWireV2 {
        payload_sha256: restart_receipt_state_digest_v2(
            &wire.terminal_parcels,
            &wire.receipt_chain,
        )?,
        ..wire
    };
    serde_json::to_vec(&wire).map_err(|_| {
        DirectSnowStage3V11AttachmentError::Identity("restart receipt-state serialization")
    })
}

#[cfg(feature = "persisted-restart-v1")]
pub fn restart_authority_decode_receipt_state_v2(
    bytes: &[u8],
    vegetation_configuration: &VegetationConfigurationV11,
) -> Result<
    (
        BTreeMap<Digest32, DirectSnowStage3V11TerminalParcel>,
        Vec<DirectSnowStage3V11ParentReceipt>,
    ),
    DirectSnowStage3V11AttachmentError,
> {
    let wire: Stage3V11ReceiptStateRestartWireV2 = serde_json::from_slice(bytes).map_err(|_| {
        DirectSnowStage3V11AttachmentError::Identity("restart receipt-state decoding")
    })?;
    if wire.schema != "OPENWEPP_SNOW_STAGE3_V11_RECEIPT_STATE_RESTART_V2"
        || wire.version != 2
        || wire.payload_sha256
            != restart_receipt_state_digest_v2(&wire.terminal_parcels, &wire.receipt_chain)?
        || wire
            .terminal_parcels
            .iter()
            .any(|(identity, parcel)| *identity != parcel.parcel_digest)
    {
        return Err(DirectSnowStage3V11AttachmentError::Identity(
            "restart receipt-state schema, digest, or parcel identity",
        ));
    }
    let canonical = serde_json::to_vec(&wire).map_err(|_| {
        DirectSnowStage3V11AttachmentError::Identity("restart receipt-state canonical encoding")
    })?;
    if canonical != bytes {
        return Err(DirectSnowStage3V11AttachmentError::Identity(
            "restart receipt-state noncanonical bytes",
        ));
    }
    let receipt_chain = wire
        .receipt_chain
        .into_iter()
        .map(|receipt| receipt.restore(vegetation_configuration))
        .collect::<Result<Vec<_>, _>>()?;
    if receipt_chain.windows(2).any(|pair| {
        pair[0]
            .day_index
            .checked_add(1)
            .is_none_or(|expected| pair[1].day_index != expected)
    }) {
        return Err(DirectSnowStage3V11AttachmentError::Identity(
            "restart parent-receipt day chronology",
        ));
    }
    for receipt in &receipt_chain {
        if receipt.support_count != STAGE3_V11_PARENT_SUPPORT_COUNT
            || receipt.covered_owner_joins
                != receipt
                    .coupled_subslabs
                    .iter()
                    .map(|subslab| subslab.owner_join.clone())
                    .collect::<Vec<_>>()
            || receipt.integrated_boundary_ledger
                != reconstruct_integrated_boundary_ledger(&receipt.coupled_subslabs)
        {
            return Err(DirectSnowStage3V11AttachmentError::Identity(
                "restart parent-receipt intrinsic reconstruction",
            ));
        }
        for subslab in &receipt.coupled_subslabs {
            subslab.validate()?;
        }
        for adaptive in &receipt.adaptive_support_receipts {
            adaptive.validate()?;
        }
    }
    Ok((wire.terminal_parcels, receipt_chain))
}
