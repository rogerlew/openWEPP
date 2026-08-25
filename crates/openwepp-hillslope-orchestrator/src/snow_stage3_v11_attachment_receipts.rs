#[derive(Clone, Debug, PartialEq)]
pub struct DirectSnowStage3V11TerminalReceipt {
    pub lane_id: u32,
    pub support: TimeSupport,
    pub result: DirectSnowTerminalEventResult,
    pub candidate_ticks: Vec<ModelTimeNs>,
    pub accepted_event_tick: ModelTimeNs,
}

#[derive(Clone, Debug, PartialEq)]
pub struct DirectSnowStage3V11TerminalParcel {
    pub support: TimeSupport,
    pub source_lane_id: u32,
    pub parent_transaction_id: Digest32,
    pub event_ordinal: u32,
    pub terminal_event_proposal_core_id: Digest32,
    pub event_result_digest: Digest32,
    pub receiver_topology_sha256: Digest32,
    pub destination_ofe_id: String,
    pub destination_tile_id: String,
    pub destination_fraction: f64,
    pub mass_kg_m2_tile_ground: f64,
    pub temperature_k: f64,
    pub specific_liquid_enthalpy_j_kg: f64,
    pub posture: DirectSnowStage3V11TerminalParcelPosture,
    pub parcel_digest: Digest32,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum DirectSnowStage3V11TerminalParcelPosture {
    ProducedUnconsumed,
    Consumed,
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
    let next = expected.checked_add(1).ok_or(
        DirectSnowStage3V11AttachmentError::Identity(
            "terminal accepted-event receipt ordinal overflow",
        ),
    )?;
    next_by_parent.insert(parent, next);
    Ok(())
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
    pub integrated_boundary_ledger: Stage3ParentIntegratedBoundaryLedgerV1,
    pub ending_coupled_owner_set_sha256: Digest32,
    pub ending_coupled_accepted_until_ns: ModelTimeNs,
    pub ending_next_parent_sequence: u128,
    pub ending_v11_parent_state: V11ParentTransaction,
    pub ending_last_v11_parent_candidate: Option<V11ParentCandidate>,
}

impl DirectSnowStage3V11ParentReceipt {
    fn validate_against_ending(
        &self,
        ending: &DirectSnowStage3V11CommittedState,
    ) -> Result<(), DirectSnowStage3V11AttachmentError> {
        for subslab in &self.coupled_subslabs {
            subslab.validate()?;
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
        let mut accepted_ids = BTreeSet::new();
        let mut next_ordinal_by_parent = BTreeMap::new();
        for (subslab, group) in terminal_subslabs
            .into_iter()
            .zip(&self.terminal_event_groups)
        {
            let accepted = group.accepted_event_receipt.as_ref().ok_or(
                DirectSnowStage3V11AttachmentError::Identity(
                    "terminal group accepted-event receipt",
                ),
            )?;
            advance_canonical_terminal_event_ordinal(
                &mut next_ordinal_by_parent,
                accepted.parent_transaction_id(),
                accepted.ordinal(),
            )?;
            let ledger = group.terminal_physical_ledger.as_ref().ok_or(
                DirectSnowStage3V11AttachmentError::Identity(
                    "terminal group physical ledger",
                ),
            )?;
            let proposal_core = group.proposal_core_sha256.ok_or(
                DirectSnowStage3V11AttachmentError::Identity(
                    "terminal group proposal core",
                ),
            )?;
            let mut group_parcel_digests = ending
                .terminal_parcels
                .values()
                .filter(|parcel| {
                    parcel.event_ordinal == accepted.ordinal()
                        && parcel.terminal_event_proposal_core_id == proposal_core
                        && parcel.posture
                            == DirectSnowStage3V11TerminalParcelPosture::ProducedUnconsumed
                })
                .map(|parcel| parcel.parcel_digest)
                .collect::<Vec<_>>();
            group_parcel_digests.sort_unstable();
            let group_parcel_fields = group_parcel_digests
                .iter()
                .map(|digest| FramedField {
                    tag: "parcel",
                    value: digest.as_bytes(),
                })
                .collect::<Vec<_>>();
            let reconstructed_parcel_set =
                framed_sha256("stage3-v11-terminal-parcel-set", &group_parcel_fields)?;
            if group.accepted_group_receipt_sha256
                != Some(accepted_terminal_group_digest(group)?)
                || accepted.tick() != group.tick
                || u64::from(accepted.ordinal()) != group.ordinal
                || accepted.event_context_digest() != group.receipt_sha256
                || !accepted_ids.insert(accepted.id())
                || ledger.produced_unconsumed_parcel_set_sha256 != reconstructed_parcel_set
                || accepted.beginning_owner_set_digest()
                    != subslab.owner_join.ending_complete_owner_set_sha256
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
            let expected_beginning_owner = self
                .terminal_event_groups
                .iter()
                .find(|group| {
                    group.tick == pair[0].support.end_ns()
                        && !pair[0].terminal_events.is_empty()
                })
                .and_then(|group| group.accepted_event_receipt.as_ref())
                .map_or(pair[0].owner_join.ending_complete_owner_set_sha256, |accepted| {
                    accepted.ending_owner_set_digest()
                });
            if pair[0].support.end_ns() != pair[1].support.start_ns()
                || expected_beginning_owner
                    != pair[1].owner_join.beginning_complete_owner_set_sha256
            {
                return Err(DirectSnowStage3V11AttachmentError::Identity(
                    "parent subslab chronology/owner adjacency",
                ));
            }
        }
        if self
            .coupled_subslabs
            .iter()
            .enumerate()
            .any(|(index, value)| {
                value.wb14_parent_replay_bytes.is_some()
                    != (index + 1 == self.coupled_subslabs.len()
                        || self.coupled_subslabs[index + 1].parent_support != value.parent_support)
            })
        {
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
        owner_bytes.insert(
            "snow".to_owned(),
            if ending.terminal_parcels.is_empty() {
                canonical_stage3_snow_owner_bytes_with_pending(
                    &ending.stage3_by_lane,
                    &ending.terminal_parcels,
                )?
            } else {
                ending
                    .coupled_clock
                    .owners()
                    .iter()
                    .find(|owner| owner.owner_id() == "snow")
                    .ok_or(DirectSnowStage3V11AttachmentError::Identity(
                        "terminal ending coupled snow owner",
                    ))?
                    .state_bytes()
                    .to_vec()
            },
        );
        if stage3_digests != self.ending_stage3_state_digests
            || owner_bytes != self.complete_owner_bytes
            || complete_owner_set_digest(ending.coupled_clock.owners())?
                != self.ending_coupled_owner_set_sha256
            || ending.coupled_clock.accepted_until() != self.ending_coupled_accepted_until_ns
            || ending.next_parent_sequence != self.ending_next_parent_sequence
            || ending.v11_parent_state != self.ending_v11_parent_state
            || ending.last_v11_parent_candidate != self.ending_last_v11_parent_candidate
        {
            return Err(DirectSnowStage3V11AttachmentError::Identity(
                "parent receipt ending owner join",
            ));
        }
        Ok(())
    }
}

#[cfg(test)]
mod terminal_event_ordinal_tests {
    use super::*;

    fn parent(byte: u8) -> ParentTransactionId {
        ParentTransactionId::from_digest(Digest32::from_bytes([byte; 32]))
    }

    #[test]
    fn coupled_receipt_ordinals_are_parent_local_and_contiguous() {
        let mut next = BTreeMap::new();
        assert!(advance_canonical_terminal_event_ordinal(&mut next, parent(1), 0).is_ok());
        assert!(advance_canonical_terminal_event_ordinal(&mut next, parent(1), 1).is_ok());
        assert!(advance_canonical_terminal_event_ordinal(&mut next, parent(2), 0).is_ok());
    }

    #[test]
    fn coupled_receipt_ordinal_duplicate_gap_and_nonzero_start_fail() {
        let mut next = BTreeMap::new();
        let beginning = next.clone();
        assert!(advance_canonical_terminal_event_ordinal(&mut next, parent(1), 1).is_err());
        assert_eq!(next, beginning, "failed first receipt must retain no state");
        assert!(advance_canonical_terminal_event_ordinal(&mut next, parent(2), 0).is_ok());
        let accepted = next.clone();
        assert!(advance_canonical_terminal_event_ordinal(&mut next, parent(2), 0).is_err());
        assert_eq!(next, accepted, "duplicate rejection must roll back exactly");
        assert!(advance_canonical_terminal_event_ordinal(&mut next, parent(2), 2).is_err());
        assert_eq!(next, accepted, "gap rejection must roll back exactly");
    }
}
#[derive(Clone, Debug, PartialEq)]
pub struct Stage3CoupledSubslabReceiptV1 {
    pub parent_support: TimeSupport,
    pub support: TimeSupport,
    pub selected_upper_bound_s_bits: u64,
    pub accepted_slab_sha256: Digest32,
    pub wb14_child_receipt_set_sha256: Digest32,
    pub wb14_parent_receipt_set_sha256: Option<Digest32>,
    pub wb14_child_replay_bytes: Vec<u8>,
    pub wb14_parent_replay_bytes: Option<Vec<u8>>,
    pub destination_receipts: BTreeMap<(OfeId, TileId), FinalStage3TileBoundaryReceiptV1>,
    pub lane_receipts: BTreeMap<u32, LaneStage3BoundaryReceiptV1>,
    pub physical_outcome_ledger_set_sha256: Digest32,
    pub terminal_events: BTreeMap<u32, DirectSnowTerminalEventResult>,
    pub owner_join: CoveredParentOwnerJoinReceiptV1,
    pub receipt_sha256: Digest32,
}

impl Stage3CoupledSubslabReceiptV1 {
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
        crate::direct_runtime::validate_wb14_child_replay_binding(
            &self.wb14_child_replay_bytes,
            crate::direct_runtime::DirectWb14CoupledChildBindingV1 {
                proposed_upper_bound_s_bits: self.selected_upper_bound_s_bits,
                coupled_parent_transaction_sha256: *self
                    .owner_join
                    .parent_transaction_sha256
                    .as_bytes(),
                accepted_slab_sha256: *self.accepted_slab_sha256.as_bytes(),
                parent_beginning_complete_owner_set_sha256: *self
                    .owner_join
                    .beginning_complete_owner_set_sha256
                    .as_bytes(),
                parent_support_start_ns: self.parent_support.start_ns().get(),
                parent_support_end_ns: self.parent_support.end_ns().get(),
                child_support_start_ns: self.support.start_ns().get() as u128,
                child_support_end_ns: self.support.end_ns().get() as u128,
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

#[derive(Clone, Debug, Default, PartialEq)]
pub struct Stage3ParentIntegratedBoundaryLedgerV1 {
    pub by_lane: BTreeMap<u32, Stage3IntegratedBoundaryLedgerV1>,
}

#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct Stage3IntegratedBoundaryLedgerV1 {
    pub sensible_energy_into_snow_j_m2: f64,
    pub vapor_mass_into_snow_kg_m2: f64,
    pub latent_energy_into_snow_j_m2: f64,
    pub shortwave_energy_into_snow_j_m2: f64,
    pub net_longwave_energy_into_snow_j_m2: f64,
}

fn reconstruct_integrated_boundary_ledger(
    subslabs: &[Stage3CoupledSubslabReceiptV1],
) -> Stage3ParentIntegratedBoundaryLedgerV1 {
    let mut ledger = Stage3ParentIntegratedBoundaryLedgerV1::default();
    for subslab in subslabs {
        let duration_s = f64::from_bits(subslab.support.duration_s_bits());
        for (lane_id, receipt) in &subslab.lane_receipts {
            let lane = ledger.by_lane.entry(*lane_id).or_default();
            lane.sensible_energy_into_snow_j_m2 +=
                -receipt.aggregate_sensible_to_canopy_air_w_m2 * duration_s;
            lane.vapor_mass_into_snow_kg_m2 +=
                -receipt.aggregate_vapor_to_canopy_air_kg_m2_s * duration_s;
            lane.latent_energy_into_snow_j_m2 +=
                -receipt.aggregate_latent_energy_to_canopy_air_j_m2;
            lane.shortwave_energy_into_snow_j_m2 +=
                receipt.aggregate_snow_absorbed_shortwave_w_m2 * duration_s;
            lane.net_longwave_energy_into_snow_j_m2 +=
                receipt.aggregate_snow_net_longwave_w_m2 * duration_s;
        }
    }
    ledger
}
