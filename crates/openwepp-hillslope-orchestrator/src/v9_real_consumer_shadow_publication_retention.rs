const WB14_REPLAY_DELTA_BLOCK_BYTES: usize = 32;

#[derive(Clone, Debug)]
enum PersistentCanonicalWb14ReplayChunkV1 {
    Copy { offset: usize, len: usize },
    Literal(std::sync::Arc<[u8]>),
}

#[derive(Clone, Debug)]
enum PersistentCanonicalWb14ReplayRepresentationV1 {
    Full(std::sync::Arc<[u8]>),
    Delta {
        previous: std::sync::Arc<PersistentCanonicalWb14ReplayNodeV1>,
        chunks: Vec<PersistentCanonicalWb14ReplayChunkV1>,
    },
}

#[derive(Clone, Debug)]
struct PersistentCanonicalWb14ReplayNodeV1 {
    len: usize,
    canonical_sha256: Digest32,
    representation: PersistentCanonicalWb14ReplayRepresentationV1,
}

#[derive(Clone, Debug)]
struct PersistentCanonicalWb14ReplayV1 {
    node: std::sync::Arc<PersistentCanonicalWb14ReplayNodeV1>,
}

impl PartialEq for PersistentCanonicalWb14ReplayV1 {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.node, &other.node)
            || (self.node.len == other.node.len
                && self.node.canonical_sha256 == other.node.canonical_sha256
                && self.materialize() == other.materialize())
    }
}

impl PersistentCanonicalWb14ReplayV1 {
    fn from_bytes(bytes: Vec<u8>) -> Self {
        let canonical_sha256 = digest_bytes(&bytes);
        let len = bytes.len();
        Self {
            node: std::sync::Arc::new(PersistentCanonicalWb14ReplayNodeV1 {
                len,
                canonical_sha256,
                representation: PersistentCanonicalWb14ReplayRepresentationV1::Full(bytes.into()),
            }),
        }
    }

    fn canonical_sha256(&self) -> Digest32 {
        self.node.canonical_sha256
    }

    fn materialize(&self) -> Vec<u8> {
        Self::materialize_node(&self.node)
    }

    fn materialize_arc(&self) -> std::sync::Arc<[u8]> {
        match &self.node.representation {
            PersistentCanonicalWb14ReplayRepresentationV1::Full(bytes) => {
                std::sync::Arc::clone(bytes)
            }
            PersistentCanonicalWb14ReplayRepresentationV1::Delta { .. } => {
                self.materialize().into()
            }
        }
    }

    /// Replace any predecessor-linked delta representation with one exact
    /// canonical full value. Day rotation uses this before releasing accepted
    /// supports so the live checkpoint cannot keep earlier replay nodes alive.
    fn rebase_materialized(&mut self) {
        if matches!(
            self.node.representation,
            PersistentCanonicalWb14ReplayRepresentationV1::Full(_)
        ) {
            return;
        }
        let bytes = self.materialize();
        *self = Self::from_bytes(bytes);
    }

    #[cfg(test)]
    fn validates_materialized_value(&self) -> bool {
        let bytes = self.materialize();
        bytes.len() == self.node.len && digest_bytes(&bytes) == self.node.canonical_sha256
    }

    #[cfg(test)]
    fn retained_literal_bytes(&self) -> usize {
        match &self.node.representation {
            PersistentCanonicalWb14ReplayRepresentationV1::Full(bytes) => bytes.len(),
            PersistentCanonicalWb14ReplayRepresentationV1::Delta { chunks, .. } => chunks
                .iter()
                .map(|chunk| match chunk {
                    PersistentCanonicalWb14ReplayChunkV1::Copy { .. } => 0,
                    PersistentCanonicalWb14ReplayChunkV1::Literal(bytes) => bytes.len(),
                })
                .sum(),
        }
    }

    fn materialize_node(node: &PersistentCanonicalWb14ReplayNodeV1) -> Vec<u8> {
        match &node.representation {
            PersistentCanonicalWb14ReplayRepresentationV1::Full(bytes) => bytes.to_vec(),
            PersistentCanonicalWb14ReplayRepresentationV1::Delta { previous, chunks } => {
                let previous = Self::materialize_node(previous);
                Self::apply_chunks(&previous, chunks, node.len)
            }
        }
    }

    fn apply_chunks(
        previous: &[u8],
        chunks: &[PersistentCanonicalWb14ReplayChunkV1],
        capacity: usize,
    ) -> Vec<u8> {
        let mut result = Vec::with_capacity(capacity);
        for chunk in chunks {
            match chunk {
                PersistentCanonicalWb14ReplayChunkV1::Copy { offset, len } => {
                    result.extend_from_slice(&previous[*offset..*offset + *len]);
                }
                PersistentCanonicalWb14ReplayChunkV1::Literal(bytes) => {
                    result.extend_from_slice(bytes);
                }
            }
        }
        result
    }

    fn compact_against(&mut self, previous: &Self, previous_bytes: &[u8], current: &[u8]) {
        let chunks = Self::delta_chunks(previous_bytes, current);
        let retained_bytes = chunks
            .iter()
            .map(|chunk| match chunk {
                PersistentCanonicalWb14ReplayChunkV1::Copy { .. } => {
                    2 * std::mem::size_of::<usize>()
                }
                PersistentCanonicalWb14ReplayChunkV1::Literal(bytes) => bytes.len(),
            })
            .sum::<usize>();
        if retained_bytes >= current.len() {
            return;
        }
        let reconstructed = Self::apply_chunks(previous_bytes, &chunks, current.len());
        if reconstructed != current {
            return;
        }
        let node = PersistentCanonicalWb14ReplayNodeV1 {
            len: current.len(),
            canonical_sha256: digest_bytes(current),
            representation: PersistentCanonicalWb14ReplayRepresentationV1::Delta {
                previous: std::sync::Arc::clone(&previous.node),
                chunks,
            },
        };
        let compacted = Self {
            node: std::sync::Arc::new(node),
        };
        *self = compacted;
    }

    fn delta_chunks(previous: &[u8], current: &[u8]) -> Vec<PersistentCanonicalWb14ReplayChunkV1> {
        let mut blocks =
            std::collections::HashMap::<[u8; WB14_REPLAY_DELTA_BLOCK_BYTES], Vec<usize>>::new();
        if previous.len() >= WB14_REPLAY_DELTA_BLOCK_BYTES {
            for offset in (0..=previous.len() - WB14_REPLAY_DELTA_BLOCK_BYTES)
                .step_by(WB14_REPLAY_DELTA_BLOCK_BYTES)
            {
                let mut key = [0_u8; WB14_REPLAY_DELTA_BLOCK_BYTES];
                key.copy_from_slice(&previous[offset..offset + WB14_REPLAY_DELTA_BLOCK_BYTES]);
                let offsets = blocks.entry(key).or_default();
                if offsets.len() < 16 {
                    offsets.push(offset);
                }
            }
        }
        let mut chunks = Vec::new();
        let mut literal = Vec::new();
        let mut index = 0;
        while index < current.len() {
            let matched = if index + WB14_REPLAY_DELTA_BLOCK_BYTES <= current.len() {
                let mut key = [0_u8; WB14_REPLAY_DELTA_BLOCK_BYTES];
                key.copy_from_slice(&current[index..index + WB14_REPLAY_DELTA_BLOCK_BYTES]);
                blocks.get(&key).and_then(|offsets| {
                    offsets
                        .iter()
                        .map(|offset| {
                            let mut len = WB14_REPLAY_DELTA_BLOCK_BYTES;
                            while *offset + len < previous.len()
                                && index + len < current.len()
                                && previous[*offset + len] == current[index + len]
                            {
                                len += 1;
                            }
                            (*offset, len)
                        })
                        .max_by_key(|(_, len)| *len)
                })
            } else {
                None
            };
            if let Some((offset, len)) = matched {
                if !literal.is_empty() {
                    chunks.push(PersistentCanonicalWb14ReplayChunkV1::Literal(
                        std::mem::take(&mut literal).into(),
                    ));
                }
                chunks.push(PersistentCanonicalWb14ReplayChunkV1::Copy { offset, len });
                index += len;
            } else {
                literal.push(current[index]);
                index += 1;
            }
        }
        if !literal.is_empty() {
            chunks.push(PersistentCanonicalWb14ReplayChunkV1::Literal(
                literal.into(),
            ));
        }
        chunks
    }
}

/// Exact real-consumer operands for one accepted V11 support. This is an
/// unpublished capability: downstream day owners may consume it only while
/// constructing the staged complete-day candidate.
#[derive(Clone, Debug, PartialEq)]
pub(crate) struct Stage3AcceptedPublicationSupportV1 {
    day_index: usize,
    interval_index: usize,
    parent_transaction_id: ParentTransactionId,
    support: TimeSupport,
    accepted_slab_sha256: Digest32,
    beginning_complete_owner_set_sha256: Digest32,
    ending_complete_owner_set_sha256: Digest32,
    lse_support_receipt: LseSupportAdmissibilityReceiptV1,
    lse_forcing: LandSurfaceForcing,
    vegetation_forcing: SnowFreeForcing,
    wb14_parameters: Vec<DirectOfeWb14Parameters>,
    resource_debits: Vec<V11ResourceDebit>,
    material_transfers: Vec<openwepp_vegetation::carbon_nitrogen::MaterialTransfer>,
    run_identity: crate::DirectRunIdentity,
    beginning_lane_carries: Vec<Stage3AcceptedBeginningLaneCarryV1>,
    beginning_subsurface_layers_by_lane: Vec<Vec<crate::DirectSubsurfaceLayerState>>,
    ending_subsurface_layers_by_lane: Vec<Vec<crate::DirectSubsurfaceLayerState>>,
    surface_beginning_state: crate::DirectSurfaceLiquidOwnedState,
    surface_ending_state: crate::DirectSurfaceLiquidOwnedState,
    open_ingress_parcels: Vec<crate::direct_runtime::DirectOpenLiquidIngressParcel>,
    ingress_receipts: Vec<crate::direct_runtime::DirectSurfaceLiquidParcelReceipt>,
    ingress_ledgers: Vec<crate::direct_runtime::DirectSurfaceLiquidIngressLedger>,
    accepted_snow_liquid_outputs: Vec<Stage3AcceptedSnowLiquidOutputV1>,
    wb14_child_replay: PersistentCanonicalWb14ReplayV1,
    wb14_parent_replay_bytes: Option<Vec<u8>>,
    finalized_water_uses: Vec<openwepp_land_surface_energy::WaterAmount>,
    condensation_credits: Vec<openwepp_land_surface_energy::CondensationCredit>,
    receiver_operands_sha256: Digest32,
    rollback_hashes: Vec<openwepp_land_surface_energy::OwnerRollbackHash>,
    hydrology_transaction_id: TransactionId,
    wb14_child_receipt_set_sha256: Digest32,
    operands_sha256: Digest32,
    receipt_sha256: Digest32,
}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub(crate) struct Stage3AcceptedSnowLiquidOutputV1 {
    pub support: TimeSupport,
    pub lane_id: u32,
    pub ofe_id: OfeId,
    pub topology_sha256: Digest32,
    pub beginning_snow_owner_sha256: Digest32,
    pub ending_snow_owner_sha256: Digest32,
    pub source_receipts_sha256: [Digest32; 6],
    pub mass_kg_m2_ofe_ground: f64,
    pub sensible_enthalpy_j_m2_ofe_ground: f64,
    pub destinations: Vec<v11_covered::physical_outcome_ledger::Stage3DestinationLiquidOutcomeV1>,
    pub refreeze_kg_m2_ofe_ground: f64,
    pub physical_ledger_receipt_sha256: Digest32,
    pub receipt_sha256: Digest32,
}

impl Stage3AcceptedSnowLiquidOutputV1 {
    fn digest(&self) -> Result<Digest32, DirectV11RealConsumerError> {
        let mut value = self.clone();
        value.receipt_sha256 = Digest32::zero();
        let bytes = serde_json::to_vec(&value).map_err(|_| {
            DirectV11RealConsumerError::Identity("accepted snow-liquid output serialization")
        })?;
        Ok(digest_bytes(&bytes))
    }

    fn seal(
        ledger: &v11_covered::physical_outcome_ledger::Stage3LanePhysicalOutcomeLedgerV1,
    ) -> Result<Self, DirectV11RealConsumerError> {
        let mut value = Self {
            support: ledger.support,
            lane_id: ledger.lane_id,
            ofe_id: ledger.ofe_id.clone(),
            topology_sha256: ledger.topology_sha256,
            beginning_snow_owner_sha256: ledger.beginning_snow_owner_sha256,
            ending_snow_owner_sha256: ledger.ending_snow_owner_sha256,
            source_receipts_sha256: ledger.source_receipts_sha256,
            mass_kg_m2_ofe_ground: ledger.terminal_liquid_kg_m2,
            sensible_enthalpy_j_m2_ofe_ground: ledger.terminal_liquid_sensible_enthalpy_j_m2,
            destinations: ledger.destination_liquid_outcomes.clone(),
            refreeze_kg_m2_ofe_ground: ledger.refreeze_kg_m2,
            physical_ledger_receipt_sha256: ledger.receipt_sha256,
            receipt_sha256: Digest32::zero(),
        };
        if value.trial_thermodynamics_require_refinement() {
            return Err(DirectV11RealConsumerError::AdaptiveRefinement(
                "accepted snow-liquid thermodynamic domain",
            ));
        }
        value.receipt_sha256 = value.digest()?;
        value.validate()?;
        Ok(value)
    }

    fn trial_thermodynamics_require_refinement(&self) -> bool {
        self.mass_kg_m2_ofe_ground.is_finite()
            && self.mass_kg_m2_ofe_ground > 0.0
            && self.sensible_enthalpy_j_m2_ofe_ground.is_finite()
            && crate::snow_stage3_v11_attachment::terminal_liquid_thermodynamics_v1(
                self.mass_kg_m2_ofe_ground,
                self.sensible_enthalpy_j_m2_ofe_ground,
            )
            .is_err()
    }

    pub(crate) fn validate(&self) -> Result<(), DirectV11RealConsumerError> {
        let source_receipts_are_unique = self
            .source_receipts_sha256
            .iter()
            .copied()
            .collect::<BTreeSet<_>>()
            .len()
            == self.source_receipts_sha256.len();
        let thermodynamics_are_exact = if self.mass_kg_m2_ofe_ground > 0.0 {
            crate::snow_stage3_v11_attachment::terminal_liquid_thermodynamics_v1(
                self.mass_kg_m2_ofe_ground,
                self.sensible_enthalpy_j_m2_ofe_ground,
            )
            .is_ok()
        } else {
            self.sensible_enthalpy_j_m2_ofe_ground.to_bits() == 0.0_f64.to_bits()
        };
        let destination_authority = self
            .destinations
            .first()
            .map(|first| {
                v11_covered::physical_outcome_ledger::seal_destination_liquid_outcomes_v1(
                    &self.ofe_id,
                    self.destinations
                        .iter()
                        .map(|value| (&value.tile_id, value.tile_fraction)),
                    first.mass_kg_m2_tile_ground,
                    first.sensible_enthalpy_j_m2_tile_ground,
                )
            })
            .transpose()
            .map_err(|_| {
                DirectV11RealConsumerError::Identity("accepted snow-liquid destination authority")
            })?;
        if self.support.duration_ns() == 0 || self.lane_id == 0 {
            return Err(DirectV11RealConsumerError::Identity(
                "accepted snow-liquid output support/lane identity",
            ));
        }
        if self.topology_sha256 == Digest32::zero()
            || self.beginning_snow_owner_sha256 == Digest32::zero()
            || self.ending_snow_owner_sha256 == Digest32::zero()
            || self.physical_ledger_receipt_sha256 == Digest32::zero()
        {
            return Err(DirectV11RealConsumerError::Identity(
                "accepted snow-liquid output authority identity",
            ));
        }
        if self.source_receipts_sha256.contains(&Digest32::zero()) || !source_receipts_are_unique {
            return Err(DirectV11RealConsumerError::Identity(
                "accepted snow-liquid output source receipt identity",
            ));
        }
        if !self.mass_kg_m2_ofe_ground.is_finite()
            || self.mass_kg_m2_ofe_ground < 0.0
            || !self.sensible_enthalpy_j_m2_ofe_ground.is_finite()
            || !self.refreeze_kg_m2_ofe_ground.is_finite()
            || self.refreeze_kg_m2_ofe_ground < 0.0
            || (self.mass_kg_m2_ofe_ground.to_bits() == 0.0_f64.to_bits()
                && self.sensible_enthalpy_j_m2_ofe_ground.to_bits() != 0.0_f64.to_bits())
            || !thermodynamics_are_exact
            || destination_authority
                .as_ref()
                .is_none_or(|(destinations, mass, enthalpy)| {
                    destinations != &self.destinations
                        || mass.to_bits() != self.mass_kg_m2_ofe_ground.to_bits()
                        || enthalpy.to_bits() != self.sensible_enthalpy_j_m2_ofe_ground.to_bits()
                })
        {
            return Err(DirectV11RealConsumerError::Identity(
                "accepted snow-liquid output thermodynamic identity",
            ));
        }
        if self.receipt_sha256 == Digest32::zero() || self.receipt_sha256 != self.digest()? {
            return Err(DirectV11RealConsumerError::Identity(
                "accepted snow-liquid output seal identity",
            ));
        }
        Ok(())
    }

    #[must_use]
    pub(crate) const fn mass_kg_m2_ofe_ground(&self) -> f64 {
        self.mass_kg_m2_ofe_ground
    }

    #[must_use]
    pub(crate) const fn sensible_enthalpy_j_m2_ofe_ground(&self) -> f64 {
        self.sensible_enthalpy_j_m2_ofe_ground
    }

    #[must_use]
    pub(crate) const fn lane_id(&self) -> u32 {
        self.lane_id
    }

    #[must_use]
    pub(crate) const fn receipt_sha256(&self) -> Digest32 {
        self.receipt_sha256
    }
}

/// Private persistent value history for committed publication authority.
///
/// Cloning a trial owner shares the immutable history and its already-sealed
/// entries. Appending uses copy-on-write for the small vector of entry handles
/// and installs one newly owned entry; neither operation aliases mutable
/// scientific state. Canonical projections always traverse entry values in
/// order, so allocation identity is never an authority input.
#[derive(Debug, PartialEq)]
struct AcceptedPublicationHistoryV1 {
    inner: std::sync::Arc<AcceptedPublicationHistoryInnerV1>,
}

#[derive(Clone, Debug)]
struct AcceptedPublicationHistoryInnerV1 {
    supports: Vec<std::sync::Arc<Stage3AcceptedPublicationSupportV1>>,
    event_handoffs: Vec<AcceptedEventReceiptV1>,
    sealed_prefix_tail: AcceptedPublicationTailAuthorityV1,
    wb14_replay_checkpoint: Option<PersistentCanonicalWb14ReplayV1>,
    last_child_replay_materialized: Option<std::sync::Arc<[u8]>>,
    tail_authority: AcceptedPublicationTailAuthorityV1,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
struct AcceptedPublicationTailAuthorityV1 {
    support_count: usize,
    event_count: usize,
    last_day_index: Option<usize>,
    last_interval_index: Option<usize>,
    last_support: Option<TimeSupport>,
    last_parent_transaction_id: Option<ParentTransactionId>,
    last_accepted_slab_sha256: Option<Digest32>,
    traversed_ending_owner_sha256: Option<Digest32>,
    pending_pre_support_event: Option<(ParentTransactionId, openwepp_coupled_time::ModelTimeNs)>,
    event_ids: BTreeSet<openwepp_coupled_time::ReceiptId>,
    last_event_ordinal_by_parent: BTreeMap<ParentTransactionId, u32>,
    aggregate_authority_sha256: Digest32,
}

struct RotationDayBoundaryV1 {
    day_index: usize,
    first_day_index: usize,
    last_day_index: usize,
    all_resident_supports_match_day: bool,
    first_support: TimeSupport,
    last_support: TimeSupport,
    has_pending_pre_support_event: bool,
    cached_last_day_index: Option<usize>,
}

fn rotation_day_boundary_is_complete_v1(value: RotationDayBoundaryV1) -> bool {
    let Some(day_start) = u128::try_from(value.day_index)
        .ok()
        .and_then(|day| day.checked_mul(crate::snow_stage3_v11_attachment::STAGE3_V11_DAY_NS))
    else {
        return false;
    };
    let Some(day_end) = day_start.checked_add(crate::snow_stage3_v11_attachment::STAGE3_V11_DAY_NS)
    else {
        return false;
    };
    value.first_day_index == value.day_index
        && value.last_day_index == value.day_index
        && value.all_resident_supports_match_day
        && value.first_support.start_ns().get() == day_start
        && value.last_support.end_ns().get() == day_end
        && !value.has_pending_pre_support_event
        && value.cached_last_day_index == Some(value.day_index)
}

fn bounded_sealed_prefix_tail_v1(
    mut tail: AcceptedPublicationTailAuthorityV1,
) -> AcceptedPublicationTailAuthorityV1 {
    tail.event_ids.clear();
    let current_parent = tail.last_parent_transaction_id;
    tail.last_event_ordinal_by_parent
        .retain(|parent, _| Some(*parent) == current_parent);
    tail
}

impl Default for AcceptedPublicationTailAuthorityV1 {
    fn default() -> Self {
        Self {
            support_count: 0,
            event_count: 0,
            last_day_index: None,
            last_interval_index: None,
            last_support: None,
            last_parent_transaction_id: None,
            last_accepted_slab_sha256: None,
            traversed_ending_owner_sha256: None,
            pending_pre_support_event: None,
            event_ids: BTreeSet::new(),
            last_event_ordinal_by_parent: BTreeMap::new(),
            aggregate_authority_sha256: digest_bytes(
                b"OPENWEPP_ACCEPTED_PUBLICATION_INCREMENTAL_AUTHORITY_V1\0",
            ),
        }
    }
}

impl PartialEq for AcceptedPublicationHistoryInnerV1 {
    fn eq(&self, other: &Self) -> bool {
        self.supports == other.supports
            && self.event_handoffs == other.event_handoffs
            && self.sealed_prefix_tail == other.sealed_prefix_tail
            && self.wb14_replay_checkpoint == other.wb14_replay_checkpoint
            && self.tail_authority == other.tail_authority
    }
}

impl Default for AcceptedPublicationHistoryV1 {
    fn default() -> Self {
        Self {
            inner: std::sync::Arc::new(AcceptedPublicationHistoryInnerV1 {
                supports: Vec::new(),
                event_handoffs: Vec::new(),
                sealed_prefix_tail: AcceptedPublicationTailAuthorityV1::default(),
                wb14_replay_checkpoint: None,
                last_child_replay_materialized: None,
                tail_authority: AcceptedPublicationTailAuthorityV1::default(),
            }),
        }
    }
}

#[cfg(test)]
std::thread_local! {
    static FORCE_DEEP_CLONE_ACCEPTED_PUBLICATION_HISTORY_V1: std::cell::Cell<bool> = const { std::cell::Cell::new(false) };
    static FORCE_FULL_SCAN_ACCEPTED_PUBLICATION_HISTORY_V1: std::cell::Cell<bool> = const { std::cell::Cell::new(false) };
}

#[cfg(test)]
pub(crate) struct ForceDeepCloneAcceptedPublicationHistoryGuardV1 {
    previous: bool,
}

#[cfg(test)]
impl Drop for ForceDeepCloneAcceptedPublicationHistoryGuardV1 {
    fn drop(&mut self) {
        FORCE_DEEP_CLONE_ACCEPTED_PUBLICATION_HISTORY_V1.with(|value| value.set(self.previous));
    }
}

#[cfg(test)]
pub(crate) fn force_deep_clone_accepted_publication_history_v1()
-> ForceDeepCloneAcceptedPublicationHistoryGuardV1 {
    let previous =
        FORCE_DEEP_CLONE_ACCEPTED_PUBLICATION_HISTORY_V1.with(|value| value.replace(true));
    ForceDeepCloneAcceptedPublicationHistoryGuardV1 { previous }
}

#[cfg(test)]
pub(crate) struct ForceFullScanAcceptedPublicationHistoryGuardV1 {
    previous: bool,
}

#[cfg(test)]
impl Drop for ForceFullScanAcceptedPublicationHistoryGuardV1 {
    fn drop(&mut self) {
        FORCE_FULL_SCAN_ACCEPTED_PUBLICATION_HISTORY_V1.with(|value| value.set(self.previous));
    }
}

#[cfg(test)]
pub(crate) fn force_full_scan_accepted_publication_history_v1()
-> ForceFullScanAcceptedPublicationHistoryGuardV1 {
    let previous =
        FORCE_FULL_SCAN_ACCEPTED_PUBLICATION_HISTORY_V1.with(|value| value.replace(true));
    ForceFullScanAcceptedPublicationHistoryGuardV1 { previous }
}

impl Clone for AcceptedPublicationHistoryV1 {
    fn clone(&self) -> Self {
        #[cfg(test)]
        if FORCE_DEEP_CLONE_ACCEPTED_PUBLICATION_HISTORY_V1.with(std::cell::Cell::get) {
            return Self {
                inner: std::sync::Arc::new(AcceptedPublicationHistoryInnerV1 {
                    supports: self
                        .inner
                        .supports
                        .iter()
                        .map(|support| {
                            let mut cloned = (**support).clone();
                            cloned.wb14_child_replay = PersistentCanonicalWb14ReplayV1::from_bytes(
                                support.wb14_child_replay.materialize(),
                            );
                            std::sync::Arc::new(cloned)
                        })
                        .collect(),
                    event_handoffs: self.inner.event_handoffs.clone(),
                    sealed_prefix_tail: self.inner.sealed_prefix_tail.clone(),
                    wb14_replay_checkpoint: self.inner.wb14_replay_checkpoint.clone(),
                    last_child_replay_materialized: self
                        .inner
                        .last_child_replay_materialized
                        .clone(),
                    tail_authority: self.inner.tail_authority.clone(),
                }),
            };
        }
        Self {
            inner: std::sync::Arc::clone(&self.inner),
        }
    }
}

impl AcceptedPublicationHistoryV1 {
    fn supports(&self) -> &[std::sync::Arc<Stage3AcceptedPublicationSupportV1>] {
        &self.inner.supports
    }

    fn event_handoffs(&self) -> &[AcceptedEventReceiptV1] {
        &self.inner.event_handoffs
    }

    fn push_support(
        &mut self,
        mut support: Stage3AcceptedPublicationSupportV1,
    ) -> Result<(), DirectV11RealConsumerError> {
        let telemetry_started =
            crate::snow_stage3_v11_attachment::adaptive_parent_telemetry_enabled_v1()
                .then(std::time::Instant::now);
        support.validate()?;
        let next_tail = self.inner.tail_authority.accept_support(&support)?;
        let current = support.wb14_child_replay.materialize_arc();
        let previous_replay = self
            .inner
            .supports
            .last()
            .map(|support| support.wb14_child_replay.clone())
            .or_else(|| self.inner.wb14_replay_checkpoint.clone());
        let previous_bytes = self.inner.last_child_replay_materialized.clone();
        if let (Some(previous_replay), Some(previous_bytes)) = (previous_replay, previous_bytes) {
            support
                .wb14_child_replay
                .compact_against(&previous_replay, &previous_bytes, &current);
        }
        let copied_on_write = std::sync::Arc::strong_count(&self.inner) > 1;
        let inner = self.make_mut();
        inner.supports.push(std::sync::Arc::new(support));
        inner.last_child_replay_materialized = Some(current);
        inner.tail_authority = next_tail;
        self.validate_full_scan_when_forced()?;
        if let Some(started) = telemetry_started {
            crate::snow_stage3_v11_attachment::record_adaptive_parent_publication_append_v1(
                started.elapsed(),
                copied_on_write,
            );
        }
        Ok(())
    }

    fn push_event_handoff(
        &mut self,
        event: AcceptedEventReceiptV1,
    ) -> Result<(), DirectV11RealConsumerError> {
        let telemetry_started =
            crate::snow_stage3_v11_attachment::adaptive_parent_telemetry_enabled_v1()
                .then(std::time::Instant::now);
        let next_tail = self.inner.tail_authority.accept_event(&event)?;
        let copied_on_write = std::sync::Arc::strong_count(&self.inner) > 1;
        let inner = self.make_mut();
        inner.event_handoffs.push(event);
        inner.tail_authority = next_tail;
        self.validate_full_scan_when_forced()?;
        if let Some(started) = telemetry_started {
            crate::snow_stage3_v11_attachment::record_adaptive_parent_publication_append_v1(
                started.elapsed(),
                copied_on_write,
            );
        }
        Ok(())
    }

    fn seal_day_evidence(
        &self,
        day_index: usize,
    ) -> Result<Stage3RotatedPublicationDayEvidenceV1, DirectV11RealConsumerError> {
        let traversed_ending = self.validate_cached_tail_against_full_scan()?.ok_or(
            DirectV11RealConsumerError::Identity("accepted publication rotation ending owner"),
        )?;
        let first = self
            .inner
            .supports
            .first()
            .ok_or(DirectV11RealConsumerError::Identity(
                "accepted publication rotation empty day",
            ))?;
        let last = self
            .inner
            .supports
            .last()
            .ok_or(DirectV11RealConsumerError::Identity(
                "accepted publication rotation empty tail",
            ))?;
        if !rotation_day_boundary_is_complete_v1(RotationDayBoundaryV1 {
            day_index,
            first_day_index: first.day_index,
            last_day_index: last.day_index,
            all_resident_supports_match_day: self
                .inner
                .supports
                .iter()
                .all(|support| support.day_index == day_index),
            first_support: first.support,
            last_support: last.support,
            has_pending_pre_support_event: self
                .inner
                .tail_authority
                .pending_pre_support_event
                .is_some(),
            cached_last_day_index: self.inner.tail_authority.last_day_index,
        }) {
            return Err(DirectV11RealConsumerError::Identity(
                "accepted publication premature or mixed-day rotation",
            ));
        }
        let beginning_owner_set_sha256 = self
            .inner
            .event_handoffs
            .first()
            .filter(|event| {
                event.tick() == first.support.start_ns()
                    && event.parent_transaction_id() == first.parent_transaction_id
            })
            .map_or(
                first.beginning_complete_owner_set_sha256,
                AcceptedEventReceiptV1::beginning_owner_set_digest,
            );
        let wire = Stage3AcceptedPublicationDayEvidenceWireV1 {
            schema_version: 1,
            day_index,
            prefix_support_count: self.inner.sealed_prefix_tail.support_count,
            prefix_event_count: self.inner.sealed_prefix_tail.event_count,
            prefix_authority_sha256: self.inner.sealed_prefix_tail.aggregate_authority_sha256,
            prefix_ending_owner_set_sha256: self
                .inner
                .sealed_prefix_tail
                .traversed_ending_owner_sha256,
            supports: self
                .inner
                .supports
                .iter()
                .map(|support| support.to_wire())
                .collect(),
            event_handoffs: self.inner.event_handoffs.clone(),
            cumulative_support_count: self.inner.tail_authority.support_count,
            cumulative_event_count: self.inner.tail_authority.event_count,
            cumulative_authority_sha256: self.inner.tail_authority.aggregate_authority_sha256,
            ending_owner_set_sha256: traversed_ending,
        };
        let canonical_support_event_bytes = serde_json::to_vec(&wire).map_err(|_| {
            DirectV11RealConsumerError::Identity(
                "accepted publication rotation evidence serialization",
            )
        })?;
        Ok(Stage3RotatedPublicationDayEvidenceV1 {
            day_index,
            canonical_uncompressed_sha256: digest_bytes(&canonical_support_event_bytes),
            canonical_support_event_bytes,
            support_count: self.inner.supports.len(),
            event_count: self.inner.event_handoffs.len(),
            beginning_owner_set_sha256,
            ending_owner_set_sha256: traversed_ending,
            last_support: last.support,
            last_parent_transaction_id: last.parent_transaction_id,
            last_accepted_slab_sha256: last.accepted_slab_sha256,
            tail_authority_sha256: self.inner.tail_authority.aggregate_authority_sha256,
        })
    }

    fn rotate_day(
        &mut self,
        evidence: &Stage3RotatedPublicationDayEvidenceV1,
    ) -> Result<(), DirectV11RealConsumerError> {
        if &self.seal_day_evidence(evidence.day_index)? != evidence {
            return Err(DirectV11RealConsumerError::Identity(
                "accepted publication rotation archive acknowledgement",
            ));
        }
        let mut checkpoint = self
            .inner
            .supports
            .last()
            .ok_or(DirectV11RealConsumerError::Identity(
                "accepted publication rotation WB14 checkpoint",
            ))?
            .wb14_child_replay
            .clone();
        checkpoint.rebase_materialized();
        let materialized = checkpoint.materialize_arc();
        if checkpoint.canonical_sha256() != digest_bytes(&materialized) {
            return Err(DirectV11RealConsumerError::Identity(
                "accepted publication rotation WB14 rebase",
            ));
        }
        let bounded_tail = bounded_sealed_prefix_tail_v1(self.inner.tail_authority.clone());
        let inner = self.make_mut();
        inner.supports.clear();
        inner.event_handoffs.clear();
        inner.sealed_prefix_tail = bounded_tail.clone();
        inner.wb14_replay_checkpoint = Some(checkpoint);
        inner.last_child_replay_materialized = Some(materialized);
        inner.tail_authority = bounded_tail;
        self.validate_cached_tail_against_full_scan()?;
        Ok(())
    }

    fn retention_state(&self) -> Stage3AcceptedPublicationRetentionStateV1 {
        let tail = &self.inner.tail_authority;
        let prefix = &self.inner.sealed_prefix_tail;
        let current_event_ordinal = tail
            .last_parent_transaction_id
            .and_then(|parent| tail.last_event_ordinal_by_parent.get(&parent).copied());
        Stage3AcceptedPublicationRetentionStateV1 {
            sealed_support_count: prefix.support_count,
            sealed_event_count: prefix.event_count,
            resident_support_count: self.inner.supports.len(),
            resident_event_count: self.inner.event_handoffs.len(),
            sealed_prefix_authority_sha256: prefix.aggregate_authority_sha256,
            cumulative_authority_sha256: tail.aggregate_authority_sha256,
            ending_owner_set_sha256: tail.traversed_ending_owner_sha256,
            last_day_index: tail.last_day_index,
            last_support: tail.last_support,
            last_parent_transaction_id: tail.last_parent_transaction_id,
            last_accepted_slab_sha256: tail.last_accepted_slab_sha256,
            current_event_ordinal,
            wb14_checkpoint_sha256: self
                .inner
                .wb14_replay_checkpoint
                .as_ref()
                .map(PersistentCanonicalWb14ReplayV1::canonical_sha256),
        }
    }

    #[cfg(any(
        feature = "restart-authority-evidence",
        feature = "persisted-restart-v1"
    ))]
    fn restore_rotation_v3(
        wire: Stage3AcceptedPublicationRotationWireV3,
    ) -> Result<Self, DirectV11RealConsumerError> {
        let prefix = &wire.sealed_prefix_tail;
        let bounded_ordinal_tail = prefix.last_event_ordinal_by_parent.len() <= 1
            && prefix
                .last_event_ordinal_by_parent
                .keys()
                .all(|parent| Some(*parent) == prefix.last_parent_transaction_id);
        let prefix_shape_is_valid = if prefix.support_count == 0 {
            prefix == &AcceptedPublicationTailAuthorityV1::default()
                && wire.wb14_replay_checkpoint_bytes.is_none()
        } else {
            prefix.last_day_index.is_some()
                && prefix.last_interval_index.is_some()
                && prefix.last_support.is_some()
                && prefix.last_parent_transaction_id.is_some()
                && prefix.last_accepted_slab_sha256.is_some()
                && prefix.traversed_ending_owner_sha256.is_some()
                && prefix.pending_pre_support_event.is_none()
                && prefix.event_ids.is_empty()
                && bounded_ordinal_tail
                && wire.wb14_replay_checkpoint_bytes.is_some()
        };
        if !prefix_shape_is_valid {
            return Err(DirectV11RealConsumerError::Identity(
                "accepted publication rotation restart prefix",
            ));
        }
        let supports = wire
            .resident_supports
            .into_iter()
            .map(Stage3AcceptedPublicationSupportV1::try_from_wire)
            .collect::<Result<Vec<_>, _>>()?;
        let checkpoint = wire
            .wb14_replay_checkpoint_bytes
            .map(PersistentCanonicalWb14ReplayV1::from_bytes);
        let checkpoint_bytes = checkpoint
            .as_ref()
            .map(PersistentCanonicalWb14ReplayV1::materialize_arc);
        let mut history = Self {
            inner: std::sync::Arc::new(AcceptedPublicationHistoryInnerV1 {
                supports: Vec::new(),
                event_handoffs: Vec::new(),
                sealed_prefix_tail: prefix.clone(),
                wb14_replay_checkpoint: checkpoint,
                last_child_replay_materialized: checkpoint_bytes,
                tail_authority: prefix.clone(),
            }),
        };
        let mut event_index = 0;
        for (support_index, support) in supports.into_iter().enumerate() {
            if support_index == 0 {
                while wire
                    .resident_event_handoffs
                    .get(event_index)
                    .is_some_and(|event| event.tick() == support.support.start_ns())
                {
                    history
                        .push_event_handoff(wire.resident_event_handoffs[event_index].clone())?;
                    event_index += 1;
                }
            }
            let support_end = support.support.end_ns();
            history.push_support(support)?;
            while wire
                .resident_event_handoffs
                .get(event_index)
                .is_some_and(|event| event.tick() == support_end)
            {
                history.push_event_handoff(wire.resident_event_handoffs[event_index].clone())?;
                event_index += 1;
            }
        }
        if event_index != wire.resident_event_handoffs.len()
            || history.inner.tail_authority != wire.cumulative_tail
            || history
                .inner
                .tail_authority
                .pending_pre_support_event
                .is_some()
        {
            return Err(DirectV11RealConsumerError::Identity(
                "accepted publication rotation restart chronology",
            ));
        }
        history.validate_cached_tail_against_full_scan()?;
        Ok(history)
    }

    #[cfg(any(
        test,
        feature = "restart-authority-evidence",
        feature = "persisted-restart-v1"
    ))]
    fn replace(
        &mut self,
        supports: Vec<Stage3AcceptedPublicationSupportV1>,
        event_handoffs: &[AcceptedEventReceiptV1],
    ) -> Result<(), DirectV11RealConsumerError> {
        *self = Self::default();
        let mut event_index = 0;
        for support in supports {
            if self.inner.supports.is_empty() {
                while event_handoffs
                    .get(event_index)
                    .is_some_and(|event| event.tick() == support.support.start_ns())
                {
                    self.push_event_handoff(event_handoffs[event_index].clone())?;
                    event_index += 1;
                }
            }
            let support_end = support.support.end_ns();
            self.push_support(support)?;
            while event_handoffs
                .get(event_index)
                .is_some_and(|event| event.tick() == support_end)
            {
                self.push_event_handoff(event_handoffs[event_index].clone())?;
                event_index += 1;
            }
        }
        if event_index != event_handoffs.len()
            || self
                .inner
                .tail_authority
                .pending_pre_support_event
                .is_some()
        {
            return Err(DirectV11RealConsumerError::Identity(
                "accepted publication incremental restore chronology",
            ));
        }
        self.validate_cached_tail_against_full_scan()?;
        Ok(())
    }

    fn make_mut(&mut self) -> &mut AcceptedPublicationHistoryInnerV1 {
        std::sync::Arc::make_mut(&mut self.inner)
    }

    fn validate_cached_tail_against_full_scan(
        &self,
    ) -> Result<Option<Digest32>, DirectV11RealConsumerError> {
        let telemetry_started =
            crate::snow_stage3_v11_attachment::adaptive_parent_telemetry_enabled_v1()
                .then(std::time::Instant::now);
        let result = self.validate_cached_tail_against_full_scan_inner();
        if let Some(started) = telemetry_started {
            crate::snow_stage3_v11_attachment::record_adaptive_parent_publication_validation_v1(
                started.elapsed(),
            );
        }
        result
    }

    fn validate_cached_tail_against_full_scan_inner(
        &self,
    ) -> Result<Option<Digest32>, DirectV11RealConsumerError> {
        let mut rebuilt = self.inner.sealed_prefix_tail.clone();
        let mut event_index = 0;
        for (support_index, support) in self.inner.supports.iter().enumerate() {
            if support_index == 0 {
                while self
                    .inner
                    .event_handoffs
                    .get(event_index)
                    .is_some_and(|event| event.tick() == support.support.start_ns())
                {
                    rebuilt = rebuilt.accept_event(&self.inner.event_handoffs[event_index])?;
                    event_index += 1;
                }
            }
            rebuilt = rebuilt.accept_support(support)?;
            while self
                .inner
                .event_handoffs
                .get(event_index)
                .is_some_and(|event| event.tick() == support.support.end_ns())
            {
                rebuilt = rebuilt.accept_event(&self.inner.event_handoffs[event_index])?;
                event_index += 1;
            }
        }
        if event_index != self.inner.event_handoffs.len()
            || rebuilt.pending_pre_support_event.is_some()
            || rebuilt != self.inner.tail_authority
        {
            return Err(DirectV11RealConsumerError::Identity(
                "accepted publication incremental tail authority",
            ));
        }
        Ok(rebuilt.traversed_ending_owner_sha256)
    }

    fn validate_full_scan_when_forced(&self) -> Result<(), DirectV11RealConsumerError> {
        #[cfg(test)]
        if FORCE_FULL_SCAN_ACCEPTED_PUBLICATION_HISTORY_V1.with(std::cell::Cell::get) {
            self.validate_cached_tail_against_full_scan()?;
        }
        Ok(())
    }
}

fn publication_rotation_v10_error(_: DirectV11RealConsumerError) -> DirectV10RealConsumerError {
    DirectV10RealConsumerError::Runtime(DirectV9RealConsumerError::Identity(
        "accepted publication history rotation",
    ))
}

impl DirectV10RealConsumerShadow {
    /// Seal one complete resident publication day without mutating retention.
    /// The returned value is the exact archive staging capability.
    pub(crate) fn seal_accepted_publication_day_evidence_v1(
        &self,
        day_index: usize,
    ) -> Result<Stage3RotatedPublicationDayEvidenceV1, DirectV10RealConsumerError> {
        self.accepted_publication_history
            .seal_day_evidence(day_index)
            .map_err(publication_rotation_v10_error)
    }

    /// Release a day only after the archive acknowledges the exact sealed
    /// capability. Any omission, substitution, reordering, or stale ack fails
    /// before mutation.
    pub(crate) fn rotate_accepted_publication_day_v1(
        &mut self,
        evidence: &Stage3RotatedPublicationDayEvidenceV1,
    ) -> Result<(), DirectV10RealConsumerError> {
        let mut candidate = self.accepted_publication_history.clone();
        candidate
            .rotate_day(evidence)
            .map_err(publication_rotation_v10_error)?;
        self.accepted_publication_history = candidate;
        Ok(())
    }

    #[must_use]
    pub fn accepted_publication_retention_state_v1(
        &self,
    ) -> Stage3AcceptedPublicationRetentionStateV1 {
        self.accepted_publication_history.retention_state()
    }

    #[cfg(any(
        feature = "restart-authority-evidence",
        feature = "persisted-restart-v1"
    ))]
    pub fn restart_authority_accepted_publication_rotation_canonical_bytes_v3(
        &self,
    ) -> Result<Vec<u8>, DirectV10RealConsumerError> {
        self.accepted_publication_history
            .validate_cached_tail_against_full_scan()
            .map_err(publication_rotation_v10_error)?;
        let mut wire = Stage3AcceptedPublicationRotationWireV3 {
            schema_version: 3,
            sealed_prefix_tail: self
                .accepted_publication_history
                .inner
                .sealed_prefix_tail
                .clone(),
            resident_supports: self
                .accepted_publication_history
                .inner
                .supports
                .iter()
                .map(|support| support.to_wire())
                .collect(),
            resident_event_handoffs: self
                .accepted_publication_history
                .inner
                .event_handoffs
                .clone(),
            wb14_replay_checkpoint_bytes: self
                .accepted_publication_history
                .inner
                .wb14_replay_checkpoint
                .as_ref()
                .map(PersistentCanonicalWb14ReplayV1::materialize),
            cumulative_tail: self
                .accepted_publication_history
                .inner
                .tail_authority
                .clone(),
            receipt_sha256: Digest32::zero(),
        };
        wire.receipt_sha256 = wire.digest()?;
        serde_json::to_vec(&wire).map_err(|_| {
            DirectV10RealConsumerError::Runtime(DirectV9RealConsumerError::Serialization(
                "accepted publication rotation V3 encoding".to_owned(),
            ))
        })
    }

    #[cfg(any(
        feature = "restart-authority-evidence",
        feature = "persisted-restart-v1"
    ))]
    pub fn restart_authority_restore_accepted_publication_rotation_canonical_bytes_v3(
        &mut self,
        bytes: &[u8],
    ) -> Result<(), DirectV10RealConsumerError> {
        let wire: Stage3AcceptedPublicationRotationWireV3 =
            serde_json::from_slice(bytes).map_err(|_| {
                DirectV10RealConsumerError::Runtime(DirectV9RealConsumerError::Serialization(
                    "accepted publication rotation V3 decoding".to_owned(),
                ))
            })?;
        let canonical = serde_json::to_vec(&wire).map_err(|_| {
            DirectV10RealConsumerError::Runtime(DirectV9RealConsumerError::Serialization(
                "accepted publication rotation V3 canonicalization".to_owned(),
            ))
        })?;
        if wire.schema_version != 3
            || canonical != bytes
            || wire.receipt_sha256 == Digest32::zero()
            || wire.receipt_sha256 != wire.digest()?
        {
            return Err(DirectV10RealConsumerError::Runtime(
                DirectV9RealConsumerError::Identity(
                    "accepted publication rotation V3 canonical seal",
                ),
            ));
        }
        let restored = AcceptedPublicationHistoryV1::restore_rotation_v3(wire)
            .map_err(publication_rotation_v10_error)?;
        self.accepted_publication_history = restored;
        Ok(())
    }
}

impl AcceptedPublicationTailAuthorityV1 {
    fn accept_support(
        &self,
        support: &Stage3AcceptedPublicationSupportV1,
    ) -> Result<Self, DirectV11RealConsumerError> {
        if self.support_count > 0 {
            let previous_support =
                self.last_support
                    .ok_or(DirectV11RealConsumerError::Identity(
                        "accepted publication cached support tail",
                    ))?;
            let previous_day = self
                .last_day_index
                .ok_or(DirectV11RealConsumerError::Identity(
                    "accepted publication cached day tail",
                ))?;
            let previous_interval =
                self.last_interval_index
                    .ok_or(DirectV11RealConsumerError::Identity(
                        "accepted publication cached interval tail",
                    ))?;
            let valid_interval = if previous_day == support.day_index {
                support.interval_index == previous_interval
                    || previous_interval.checked_add(1) == Some(support.interval_index)
            } else {
                previous_day.checked_add(1) == Some(support.day_index)
                    && support.interval_index == 0
            };
            if previous_support.end_ns() != support.support.start_ns() || !valid_interval {
                return Err(DirectV11RealConsumerError::Identity(
                    "accepted publication incremental support chronology",
                ));
            }
        }
        if self
            .traversed_ending_owner_sha256
            .is_some_and(|ending| ending != support.beginning_complete_owner_set_sha256)
        {
            return Err(DirectV11RealConsumerError::Identity(
                "accepted publication incremental owner handoff",
            ));
        }
        if let Some((parent, tick)) = self.pending_pre_support_event {
            if parent != support.parent_transaction_id || tick != support.support.start_ns() {
                return Err(DirectV11RealConsumerError::Identity(
                    "accepted publication incremental pre-support event",
                ));
            }
        }
        let mut next = self.clone();
        next.support_count += 1;
        next.last_day_index = Some(support.day_index);
        next.last_interval_index = Some(support.interval_index);
        next.last_support = Some(support.support);
        next.last_parent_transaction_id = Some(support.parent_transaction_id);
        next.last_accepted_slab_sha256 = Some(support.accepted_slab_sha256);
        next.traversed_ending_owner_sha256 = Some(support.ending_complete_owner_set_sha256);
        next.pending_pre_support_event = None;
        next.aggregate_authority_sha256 = digest_bytes(
            &[
                next.aggregate_authority_sha256.as_bytes().as_slice(),
                b"support",
                support.parent_transaction_id.digest().as_bytes().as_slice(),
                &support.support.start_ns().get().to_be_bytes(),
                &support.support.end_ns().get().to_be_bytes(),
                support.accepted_slab_sha256.as_bytes().as_slice(),
                support
                    .beginning_complete_owner_set_sha256
                    .as_bytes()
                    .as_slice(),
                support
                    .ending_complete_owner_set_sha256
                    .as_bytes()
                    .as_slice(),
                support
                    .wb14_child_replay
                    .canonical_sha256()
                    .as_bytes()
                    .as_slice(),
                support.wb14_child_receipt_set_sha256.as_bytes().as_slice(),
                support.operands_sha256.as_bytes().as_slice(),
                support.receipt_sha256.as_bytes().as_slice(),
            ]
            .concat(),
        );
        Ok(next)
    }

    fn accept_event(
        &self,
        event: &AcceptedEventReceiptV1,
    ) -> Result<Self, DirectV11RealConsumerError> {
        event.validate().map_err(|_| {
            DirectV11RealConsumerError::Identity("accepted publication incremental event seal")
        })?;
        if self.event_ids.contains(&event.id())
            || self
                .last_event_ordinal_by_parent
                .get(&event.parent_transaction_id())
                .map_or(event.ordinal() != 0, |prior| {
                    prior.checked_add(1) != Some(event.ordinal())
                })
            || self
                .traversed_ending_owner_sha256
                .is_some_and(|ending| ending != event.beginning_owner_set_digest())
        {
            return Err(DirectV11RealConsumerError::Identity(
                "accepted publication incremental event chronology",
            ));
        }
        if let Some(support) = self.last_support {
            if event.tick() != support.end_ns()
                || self.pending_pre_support_event.is_some()
                    && self.last_parent_transaction_id == Some(event.parent_transaction_id())
            {
                return Err(DirectV11RealConsumerError::Identity(
                    "accepted publication incremental event boundary",
                ));
            }
        } else if self.event_count > 0
            && self.pending_pre_support_event != Some((event.parent_transaction_id(), event.tick()))
        {
            return Err(DirectV11RealConsumerError::Identity(
                "accepted publication incremental genesis event boundary",
            ));
        }
        let mut next = self.clone();
        next.event_count += 1;
        next.event_ids.insert(event.id());
        next.last_event_ordinal_by_parent
            .insert(event.parent_transaction_id(), event.ordinal());
        next.traversed_ending_owner_sha256 = Some(event.ending_owner_set_digest());
        if self.last_parent_transaction_id != Some(event.parent_transaction_id()) {
            next.pending_pre_support_event = Some((event.parent_transaction_id(), event.tick()));
        }
        next.aggregate_authority_sha256 = digest_bytes(
            &[
                next.aggregate_authority_sha256.as_bytes().as_slice(),
                b"event",
                event.parent_transaction_id().digest().as_bytes().as_slice(),
                &event.tick().get().to_be_bytes(),
                &event.ordinal().to_be_bytes(),
                event.id().digest().as_bytes().as_slice(),
                event.beginning_owner_set_digest().as_bytes().as_slice(),
                event.ending_owner_set_digest().as_bytes().as_slice(),
            ]
            .concat(),
        );
        Ok(next)
    }
}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub(crate) struct Stage3AcceptedBeginningLaneCarryV1 {
    pub(crate) lane_id: u32,
    pub(crate) upstream_lane_id: u32,
    pub(crate) downstream_lane_id: u32,
    pub(crate) upstream_area_ratio: f64,
    pub(crate) surface_carry_m: [f64; 24],
    pub(crate) surface_hourly_weights: [f64; 24],
    pub(crate) lateral_carry_m: [f64; 24],
    pub(crate) upstream_flow_m: f64,
    pub(crate) subsurface_input_m: f64,
}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
struct Stage3PublicationRunIdentityWireV1 {
    run_id: u64,
    hillslope_id: u32,
    lane_count: usize,
    day_count: usize,
}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
struct Stage3PublicationSubsurfaceLayerWireV1 {
    theta_m: f64,
    field_capacity_m: f64,
    upper_limit_m: f64,
    conductivity_m_s: f64,
    depth_m: f64,
    residual_theta: f64,
    frozen_depth_m: f64,
    frozen_water_m: f64,
    porosity: f64,
    field_capacity_theta: f64,
    coca: f64,
    lateral_conductivity_m_s: f64,
}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
struct Stage3AcceptedPublicationSupportWireV1 {
    day_index: usize,
    interval_index: usize,
    parent_transaction_id: ParentTransactionId,
    support: TimeSupport,
    accepted_slab_sha256: Digest32,
    beginning_complete_owner_set_sha256: Digest32,
    ending_complete_owner_set_sha256: Digest32,
    lse_support_receipt: LseSupportAdmissibilityReceiptV1,
    lse_forcing: LandSurfaceForcing,
    vegetation_forcing: SnowFreeForcing,
    wb14_parameters: Vec<DirectOfeWb14Parameters>,
    resource_debits: Vec<V11ResourceDebit>,
    material_transfers: Vec<openwepp_vegetation::carbon_nitrogen::MaterialTransfer>,
    run_identity: Stage3PublicationRunIdentityWireV1,
    beginning_lane_carries: Vec<Stage3AcceptedBeginningLaneCarryV1>,
    beginning_subsurface_layers_by_lane: Vec<Vec<Stage3PublicationSubsurfaceLayerWireV1>>,
    ending_subsurface_layers_by_lane: Vec<Vec<Stage3PublicationSubsurfaceLayerWireV1>>,
    surface_beginning_state: crate::DirectSurfaceLiquidOwnedState,
    surface_ending_state: crate::DirectSurfaceLiquidOwnedState,
    open_ingress_parcels: Vec<crate::direct_runtime::DirectOpenLiquidIngressParcel>,
    ingress_receipts: Vec<crate::direct_runtime::DirectSurfaceLiquidParcelReceipt>,
    ingress_ledgers: Vec<crate::direct_runtime::DirectSurfaceLiquidIngressLedger>,
    accepted_snow_liquid_outputs: Vec<Stage3AcceptedSnowLiquidOutputV1>,
    wb14_child_replay_bytes: Vec<u8>,
    wb14_parent_replay_bytes: Option<Vec<u8>>,
    finalized_water_uses: Vec<openwepp_land_surface_energy::WaterAmount>,
    condensation_credits: Vec<openwepp_land_surface_energy::CondensationCredit>,
    receiver_operands_sha256: Digest32,
    rollback_hashes: Vec<openwepp_land_surface_energy::OwnerRollbackHash>,
    hydrology_transaction_id: TransactionId,
    wb14_child_receipt_set_sha256: Digest32,
    operands_sha256: Digest32,
    receipt_sha256: Digest32,
}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
struct Stage3AcceptedPublicationDayEvidenceWireV1 {
    schema_version: u32,
    day_index: usize,
    prefix_support_count: usize,
    prefix_event_count: usize,
    prefix_authority_sha256: Digest32,
    prefix_ending_owner_set_sha256: Option<Digest32>,
    supports: Vec<Stage3AcceptedPublicationSupportWireV1>,
    event_handoffs: Vec<AcceptedEventReceiptV1>,
    cumulative_support_count: usize,
    cumulative_event_count: usize,
    cumulative_authority_sha256: Digest32,
    ending_owner_set_sha256: Digest32,
}

#[cfg(any(
    feature = "restart-authority-evidence",
    feature = "persisted-restart-v1"
))]
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
struct Stage3AcceptedPublicationRotationWireV3 {
    schema_version: u32,
    sealed_prefix_tail: AcceptedPublicationTailAuthorityV1,
    resident_supports: Vec<Stage3AcceptedPublicationSupportWireV1>,
    resident_event_handoffs: Vec<AcceptedEventReceiptV1>,
    wb14_replay_checkpoint_bytes: Option<Vec<u8>>,
    cumulative_tail: AcceptedPublicationTailAuthorityV1,
    receipt_sha256: Digest32,
}

#[cfg(any(
    feature = "restart-authority-evidence",
    feature = "persisted-restart-v1"
))]
impl Stage3AcceptedPublicationRotationWireV3 {
    fn digest(&self) -> Result<Digest32, DirectV10RealConsumerError> {
        let mut value = self.clone();
        value.receipt_sha256 = Digest32::zero();
        let bytes = serde_json::to_vec(&value).map_err(|_| {
            DirectV10RealConsumerError::Runtime(DirectV9RealConsumerError::Serialization(
                "accepted publication rotation V3 digest".to_owned(),
            ))
        })?;
        Ok(digest_bytes(
            &[
                b"OPENWEPP_ACCEPTED_PUBLICATION_ROTATION_V3\0".as_slice(),
                bytes.as_slice(),
            ]
            .concat(),
        ))
    }
}

/// Exact, uncompressed archive handoff for one fully accepted publication day.
/// Sealing is nonmutating; rotation accepts this same value only after the
/// external day archive has durably acknowledged its canonical bytes.
#[derive(Clone, Debug, PartialEq)]
pub(crate) struct Stage3RotatedPublicationDayEvidenceV1 {
    pub(crate) day_index: usize,
    pub(crate) canonical_support_event_bytes: Vec<u8>,
    pub(crate) canonical_uncompressed_sha256: Digest32,
    pub(crate) support_count: usize,
    pub(crate) event_count: usize,
    pub(crate) beginning_owner_set_sha256: Digest32,
    pub(crate) ending_owner_set_sha256: Digest32,
    pub(crate) last_support: TimeSupport,
    pub(crate) last_parent_transaction_id: ParentTransactionId,
    pub(crate) last_accepted_slab_sha256: Digest32,
    pub(crate) tail_authority_sha256: Digest32,
}

/// Bounded live-retention projection used by qualification and persisted
/// restart. Counts and the prefix root are cumulative; resident counts cover
/// only the active, not-yet-rotated day.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Stage3AcceptedPublicationRetentionStateV1 {
    sealed_support_count: usize,
    sealed_event_count: usize,
    resident_support_count: usize,
    resident_event_count: usize,
    sealed_prefix_authority_sha256: Digest32,
    cumulative_authority_sha256: Digest32,
    ending_owner_set_sha256: Option<Digest32>,
    last_day_index: Option<usize>,
    last_support: Option<TimeSupport>,
    last_parent_transaction_id: Option<ParentTransactionId>,
    last_accepted_slab_sha256: Option<Digest32>,
    current_event_ordinal: Option<u32>,
    wb14_checkpoint_sha256: Option<Digest32>,
}

impl Stage3AcceptedPublicationRetentionStateV1 {
    #[must_use]
    pub const fn sealed_support_count(&self) -> usize {
        self.sealed_support_count
    }

    #[must_use]
    pub const fn sealed_event_count(&self) -> usize {
        self.sealed_event_count
    }

    #[must_use]
    pub const fn resident_support_count(&self) -> usize {
        self.resident_support_count
    }

    #[must_use]
    pub const fn resident_event_count(&self) -> usize {
        self.resident_event_count
    }

    #[must_use]
    pub const fn sealed_prefix_authority_sha256(&self) -> Digest32 {
        self.sealed_prefix_authority_sha256
    }

    #[must_use]
    pub const fn cumulative_authority_sha256(&self) -> Digest32 {
        self.cumulative_authority_sha256
    }

    #[must_use]
    pub const fn ending_owner_set_sha256(&self) -> Option<Digest32> {
        self.ending_owner_set_sha256
    }

    #[must_use]
    pub const fn last_day_index(&self) -> Option<usize> {
        self.last_day_index
    }

    #[must_use]
    pub const fn last_support(&self) -> Option<TimeSupport> {
        self.last_support
    }

    #[must_use]
    pub const fn last_parent_transaction_id(&self) -> Option<ParentTransactionId> {
        self.last_parent_transaction_id
    }

    #[must_use]
    pub const fn last_accepted_slab_sha256(&self) -> Option<Digest32> {
        self.last_accepted_slab_sha256
    }

    #[must_use]
    pub const fn current_event_ordinal(&self) -> Option<u32> {
        self.current_event_ordinal
    }

    #[must_use]
    pub const fn wb14_checkpoint_sha256(&self) -> Option<Digest32> {
        self.wb14_checkpoint_sha256
    }
}

#[cfg(any(
    feature = "restart-authority-evidence",
    feature = "persisted-restart-v1"
))]
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
struct Stage3AcceptedPublicationSupportSetWireV2 {
    schema_version: u32,
    supports: Vec<Stage3AcceptedPublicationSupportWireV1>,
    event_handoffs: Vec<AcceptedEventReceiptV1>,
    traversed_ending_complete_owner_set_sha256: Option<Digest32>,
    receipt_sha256: Digest32,
}

#[cfg(test)]
#[derive(Clone, Copy)]
struct AcceptedPublicationSupportChronologyV1 {
    day_index: usize,
    interval_index: usize,
    parent_transaction_sha256: Digest32,
    support_start_ns: u128,
    support_end_ns: u128,
    beginning_owner_sha256: Digest32,
    ending_owner_sha256: Digest32,
}

#[cfg(test)]
#[derive(Clone, Copy)]
struct AcceptedPublicationEventChronologyV1 {
    parent_transaction_sha256: Digest32,
    tick_ns: u128,
    ordinal: u32,
    beginning_owner_sha256: Digest32,
    ending_owner_sha256: Digest32,
}

#[cfg(test)]
fn validate_accepted_publication_chronology_v1(
    supports: &[AcceptedPublicationSupportChronologyV1],
    events: &[AcceptedPublicationEventChronologyV1],
) -> Result<Option<Digest32>, DirectV11RealConsumerError> {
    let mut event_index = 0;
    let mut traversed_ending_owner = None;
    if let Some(first_support) = supports.first() {
        let mut genesis_ending_owner = None;
        while let Some(event) = events
            .get(event_index)
            .filter(|event| event.tick_ns == first_support.support_start_ns)
        {
            let expected_beginning = genesis_ending_owner.unwrap_or(event.beginning_owner_sha256);
            if event.parent_transaction_sha256 != first_support.parent_transaction_sha256
                || event.ordinal
                    != u32::try_from(event_index).map_err(|_| {
                        DirectV11RealConsumerError::Identity(
                            "genesis accepted publication event ordinal width",
                        )
                    })?
                || event.beginning_owner_sha256 != expected_beginning
            {
                return Err(DirectV11RealConsumerError::Identity(
                    "genesis accepted publication event handoff",
                ));
            }
            genesis_ending_owner = Some(event.ending_owner_sha256);
            event_index += 1;
        }
        if let Some(ending) = genesis_ending_owner {
            if ending != first_support.beginning_owner_sha256 {
                return Err(DirectV11RealConsumerError::Identity(
                    "genesis accepted publication support handoff",
                ));
            }
            traversed_ending_owner = Some(ending);
        }
    }
    for (support_index, support) in supports.iter().enumerate() {
        if let Some(previous) = support_index
            .checked_sub(1)
            .and_then(|index| supports.get(index))
        {
            let same_day = previous.day_index == support.day_index;
            let valid_interval = if same_day {
                support.interval_index == previous.interval_index
                    || support.interval_index
                        == previous.interval_index.checked_add(1).ok_or(
                            DirectV11RealConsumerError::Identity(
                                "restored publication interval overflow",
                            ),
                        )?
            } else {
                support.day_index
                    == previous.day_index.checked_add(1).ok_or(
                        DirectV11RealConsumerError::Identity("restored publication day overflow"),
                    )?
                    && support.interval_index == 0
            };
            if previous.support_end_ns != support.support_start_ns || !valid_interval {
                return Err(DirectV11RealConsumerError::Identity(
                    "restored accepted publication support chronology",
                ));
            }
        }
        if traversed_ending_owner.is_some_and(|ending| ending != support.beginning_owner_sha256) {
            return Err(DirectV11RealConsumerError::Identity(
                "restored accepted publication complete-owner handoff",
            ));
        }

        let mut ending_owner = support.ending_owner_sha256;
        let mut crossed_into_next_parent = false;
        while let Some(event) = events
            .get(event_index)
            .filter(|event| event.tick_ns == support.support_end_ns)
        {
            let prior_same_parent = events[..event_index]
                .iter()
                .rev()
                .find(|prior| prior.parent_transaction_sha256 == event.parent_transaction_sha256);
            let ordinal_is_valid = prior_same_parent.map_or(event.ordinal == 0, |prior| {
                prior.ordinal.checked_add(1) == Some(event.ordinal)
            });
            if !ordinal_is_valid || event.beginning_owner_sha256 != ending_owner {
                return Err(DirectV11RealConsumerError::Identity(
                    "accepted publication event handoff chronology",
                ));
            }
            if event.parent_transaction_sha256 != support.parent_transaction_sha256 {
                let next =
                    supports
                        .get(support_index + 1)
                        .ok_or(DirectV11RealConsumerError::Identity(
                            "orphan pre-support accepted publication event handoff",
                        ))?;
                if event.parent_transaction_sha256 != next.parent_transaction_sha256
                    || event.tick_ns != next.support_start_ns
                {
                    return Err(DirectV11RealConsumerError::Identity(
                        "cross-parent accepted publication event handoff",
                    ));
                }
                crossed_into_next_parent = true;
            } else if crossed_into_next_parent {
                return Err(DirectV11RealConsumerError::Identity(
                    "reordered accepted publication event handoff",
                ));
            }
            ending_owner = event.ending_owner_sha256;
            event_index += 1;
        }
        traversed_ending_owner = Some(ending_owner);
    }
    if event_index != events.len() {
        return Err(DirectV11RealConsumerError::Identity(
            "orphan or out-of-order accepted publication event handoff",
        ));
    }
    Ok(traversed_ending_owner)
}

#[cfg(test)]
mod accepted_publication_chronology_tests {
    use super::*;

    fn support(
        interval_index: usize,
        parent: u8,
        start: u128,
        end: u128,
        beginning: u8,
        ending: u8,
    ) -> AcceptedPublicationSupportChronologyV1 {
        AcceptedPublicationSupportChronologyV1 {
            day_index: 0,
            interval_index,
            parent_transaction_sha256: Digest32::from_bytes([parent; 32]),
            support_start_ns: start,
            support_end_ns: end,
            beginning_owner_sha256: Digest32::from_bytes([beginning; 32]),
            ending_owner_sha256: Digest32::from_bytes([ending; 32]),
        }
    }

    fn event(
        parent: u8,
        tick: u128,
        ordinal: u32,
        beginning: u8,
        ending: u8,
    ) -> AcceptedPublicationEventChronologyV1 {
        AcceptedPublicationEventChronologyV1 {
            parent_transaction_sha256: Digest32::from_bytes([parent; 32]),
            tick_ns: tick,
            ordinal,
            beginning_owner_sha256: Digest32::from_bytes([beginning; 32]),
            ending_owner_sha256: Digest32::from_bytes([ending; 32]),
        }
    }

    #[test]
    fn pre_support_event_is_atomic_with_following_support_and_poisons_fail_closed() {
        let supports = [support(39, 1, 0, 10, 1, 2), support(40, 2, 10, 20, 4, 5)];
        let events = [event(2, 10, 0, 2, 4)];
        assert_eq!(
            validate_accepted_publication_chronology_v1(&supports, &events)
                .expect("event-before-support owner chain"),
            Some(Digest32::from_bytes([5; 32])),
        );
        assert!(validate_accepted_publication_chronology_v1(&supports, &[]).is_err());
        assert!(
            validate_accepted_publication_chronology_v1(&supports[..1], &events).is_err(),
            "pre-support event without its positive support must be orphaned",
        );
        assert!(
            validate_accepted_publication_chronology_v1(&supports, &[event(2, 10, 0, 3, 4)])
                .is_err(),
            "predecessor owner substitution",
        );
        assert!(
            validate_accepted_publication_chronology_v1(&supports, &[event(3, 10, 0, 2, 4)])
                .is_err(),
            "following-parent substitution",
        );
        let two_events = [event(2, 10, 0, 2, 3), event(2, 10, 1, 3, 4)];
        assert!(validate_accepted_publication_chronology_v1(&supports, &two_events).is_ok());
        assert!(
            validate_accepted_publication_chronology_v1(&supports, &[two_events[1], two_events[0]])
                .is_err(),
            "same-tick event order substitution",
        );

        let genesis_support = [support(0, 2, 10, 20, 4, 5)];
        let genesis_event = [event(2, 10, 0, 2, 4)];
        assert_eq!(
            validate_accepted_publication_chronology_v1(&genesis_support, &genesis_event)
                .expect("genesis event + following support"),
            Some(Digest32::from_bytes([5; 32])),
        );
        assert!(
            validate_accepted_publication_chronology_v1(
                &genesis_support,
                &[event(3, 10, 0, 2, 4)],
            )
            .is_err(),
            "genesis event parent substitution",
        );
        assert!(
            validate_accepted_publication_chronology_v1(
                &genesis_support,
                &[event(2, 10, 1, 2, 4)],
            )
            .is_err(),
            "genesis event ordinal substitution",
        );
    }

    #[test]
    fn persistent_wb14_replay_materializes_exact_bytes_and_delta_poisons_fail_closed() {
        let mut previous_bytes = b"header-v1:".to_vec();
        previous_bytes.extend(std::iter::repeat_n(b'a', 256));
        previous_bytes.extend_from_slice(b":receipt-boundary:");
        previous_bytes.extend(std::iter::repeat_n(b'b', 256));
        previous_bytes.extend_from_slice(b":tail");
        let mut current_bytes = b"header-v2:".to_vec();
        current_bytes.extend(std::iter::repeat_n(b'a', 256));
        current_bytes.extend_from_slice(b":receipt-boundary:new-child:");
        current_bytes.extend(std::iter::repeat_n(b'b', 256));
        current_bytes.extend_from_slice(b":tail");

        let previous = PersistentCanonicalWb14ReplayV1::from_bytes(previous_bytes.clone());
        let mut current = PersistentCanonicalWb14ReplayV1::from_bytes(current_bytes.clone());
        current.compact_against(&previous, &previous_bytes, &current_bytes);
        assert_eq!(current.materialize(), current_bytes);
        assert!(current.validates_materialized_value());
        assert!(current.retained_literal_bytes() < current_bytes.len() / 4);

        let PersistentCanonicalWb14ReplayRepresentationV1::Delta {
            previous: predecessor,
            chunks,
        } = &current.node.representation
        else {
            panic!("real replay-shaped bytes must compact to a delta");
        };
        assert!(chunks.len() >= 3);
        let poison =
            |chunks: Vec<PersistentCanonicalWb14ReplayChunkV1>| PersistentCanonicalWb14ReplayV1 {
                node: std::sync::Arc::new(PersistentCanonicalWb14ReplayNodeV1 {
                    len: current.node.len,
                    canonical_sha256: current.node.canonical_sha256,
                    representation: PersistentCanonicalWb14ReplayRepresentationV1::Delta {
                        previous: std::sync::Arc::clone(predecessor),
                        chunks,
                    },
                }),
            };

        let mut omitted = chunks.clone();
        omitted.remove(1);
        assert!(!poison(omitted).validates_materialized_value());

        let mut reordered = chunks.clone();
        reordered.swap(0, 1);
        assert!(!poison(reordered).validates_materialized_value());

        let mut substituted = chunks.clone();
        let literal = substituted
            .iter_mut()
            .find_map(|chunk| match chunk {
                PersistentCanonicalWb14ReplayChunkV1::Literal(bytes) => Some(bytes),
                PersistentCanonicalWb14ReplayChunkV1::Copy { .. } => None,
            })
            .expect("delta contains changed literal bytes");
        let mut bytes = literal.to_vec();
        bytes[0] ^= 1;
        *literal = bytes.into();
        assert!(!poison(substituted).validates_materialized_value());

        let canonical_before_rebase = current.materialize();
        let digest_before_rebase = current.canonical_sha256();
        current.rebase_materialized();
        assert_eq!(current.materialize(), canonical_before_rebase);
        assert_eq!(current.canonical_sha256(), digest_before_rebase);
        assert!(matches!(
            current.node.representation,
            PersistentCanonicalWb14ReplayRepresentationV1::Full(_)
        ));
    }
}

#[cfg(test)]
include!("v9_real_consumer_shadow_publication_rotation_tests.rs");

#[cfg(any(
    test,
    feature = "restart-authority-evidence",
    feature = "persisted-restart-v1"
))]
fn validate_accepted_publication_authority(
    supports: &[std::sync::Arc<Stage3AcceptedPublicationSupportV1>],
    events: &[AcceptedEventReceiptV1],
) -> Result<Option<Digest32>, DirectV11RealConsumerError> {
    let mut event_index = 0;
    let mut event_ids = BTreeSet::new();
    let mut traversed_ending_owner = None;
    if let Some(first_support) = supports.first() {
        let mut genesis_ending_owner = None;
        while let Some(event) = events
            .get(event_index)
            .filter(|event| event.tick() == first_support.support.start_ns())
        {
            event.validate().map_err(|_| {
                DirectV11RealConsumerError::Identity(
                    "genesis accepted publication event handoff seal",
                )
            })?;
            let expected_beginning =
                genesis_ending_owner.unwrap_or_else(|| event.beginning_owner_set_digest());
            if !event_ids.insert(event.id())
                || event.parent_transaction_id() != first_support.parent_transaction_id
                || event.ordinal()
                    != u32::try_from(event_index).map_err(|_| {
                        DirectV11RealConsumerError::Identity(
                            "genesis accepted publication event ordinal width",
                        )
                    })?
                || event.beginning_owner_set_digest() != expected_beginning
            {
                return Err(DirectV11RealConsumerError::Identity(
                    "genesis accepted publication event handoff",
                ));
            }
            genesis_ending_owner = Some(event.ending_owner_set_digest());
            event_index += 1;
        }
        if let Some(ending) = genesis_ending_owner {
            if ending != first_support.beginning_complete_owner_set_sha256 {
                return Err(DirectV11RealConsumerError::Identity(
                    "genesis accepted publication support handoff",
                ));
            }
            traversed_ending_owner = Some(ending);
        }
    }
    for (support_index, support) in supports.iter().enumerate() {
        support.validate()?;
        if let Some(previous) = support_index
            .checked_sub(1)
            .and_then(|index| supports.get(index))
        {
            let same_day = previous.day_index == support.day_index;
            let valid_interval = if same_day {
                support.interval_index == previous.interval_index
                    || support.interval_index
                        == previous.interval_index.checked_add(1).ok_or(
                            DirectV11RealConsumerError::Identity(
                                "restored publication interval overflow",
                            ),
                        )?
            } else {
                support.day_index
                    == previous.day_index.checked_add(1).ok_or(
                        DirectV11RealConsumerError::Identity("restored publication day overflow"),
                    )?
                    && support.interval_index == 0
            };
            if previous.support.end_ns() != support.support.start_ns() || !valid_interval {
                return Err(DirectV11RealConsumerError::Identity(
                    "restored accepted publication support chronology",
                ));
            }
        }

        if traversed_ending_owner
            .is_some_and(|ending| ending != support.beginning_complete_owner_set_sha256)
        {
            return Err(DirectV11RealConsumerError::Identity(
                "restored accepted publication complete-owner handoff",
            ));
        }

        let mut ending_owner = support.ending_complete_owner_set_sha256;
        let mut crossed_into_next_parent = false;
        while let Some(event) = events
            .get(event_index)
            .filter(|event| event.tick() == support.support.end_ns())
        {
            event.validate().map_err(|_| {
                DirectV11RealConsumerError::Identity("accepted publication event handoff seal")
            })?;
            let prior_same_parent = events[..event_index]
                .iter()
                .rev()
                .find(|prior| prior.parent_transaction_id() == event.parent_transaction_id());
            let ordinal_is_valid = prior_same_parent.map_or(event.ordinal() == 0, |prior| {
                prior.ordinal().checked_add(1) == Some(event.ordinal())
            });
            if !event_ids.insert(event.id())
                || !ordinal_is_valid
                || event.beginning_owner_set_digest() != ending_owner
            {
                return Err(DirectV11RealConsumerError::Identity(
                    "accepted publication event handoff chronology",
                ));
            }
            if event.parent_transaction_id() != support.parent_transaction_id {
                let next =
                    supports
                        .get(support_index + 1)
                        .ok_or(DirectV11RealConsumerError::Identity(
                            "orphan pre-support accepted publication event handoff",
                        ))?;
                if event.parent_transaction_id() != next.parent_transaction_id
                    || event.tick() != next.support.start_ns()
                {
                    return Err(DirectV11RealConsumerError::Identity(
                        "cross-parent accepted publication event handoff",
                    ));
                }
                crossed_into_next_parent = true;
            } else if crossed_into_next_parent {
                return Err(DirectV11RealConsumerError::Identity(
                    "reordered accepted publication event handoff",
                ));
            }
            ending_owner = event.ending_owner_set_digest();
            event_index += 1;
        }
        traversed_ending_owner = Some(ending_owner);
    }
    if event_index != events.len() {
        return Err(DirectV11RealConsumerError::Identity(
            "orphan or out-of-order accepted publication event handoff",
        ));
    }
    Ok(traversed_ending_owner)
}

impl From<crate::DirectRunIdentity> for Stage3PublicationRunIdentityWireV1 {
    fn from(value: crate::DirectRunIdentity) -> Self {
        Self {
            run_id: value.run_id,
            hillslope_id: value.hillslope_id,
            lane_count: value.lane_count,
            day_count: value.day_count,
        }
    }
}

impl From<&crate::DirectSubsurfaceLayerState> for Stage3PublicationSubsurfaceLayerWireV1 {
    fn from(value: &crate::DirectSubsurfaceLayerState) -> Self {
        Self {
            theta_m: value.theta_m,
            field_capacity_m: value.field_capacity_m,
            upper_limit_m: value.upper_limit_m,
            conductivity_m_s: value.conductivity_m_s,
            depth_m: value.depth_m,
            residual_theta: value.residual_theta,
            frozen_depth_m: value.frozen_depth_m,
            frozen_water_m: value.frozen_water_m,
            porosity: value.porosity,
            field_capacity_theta: value.field_capacity_theta,
            coca: value.coca,
            lateral_conductivity_m_s: value.lateral_conductivity_m_s,
        }
    }
}

#[cfg(any(
    feature = "restart-authority-evidence",
    feature = "persisted-restart-v1"
))]
impl From<Stage3PublicationSubsurfaceLayerWireV1> for crate::DirectSubsurfaceLayerState {
    fn from(value: Stage3PublicationSubsurfaceLayerWireV1) -> Self {
        Self {
            theta_m: value.theta_m,
            field_capacity_m: value.field_capacity_m,
            upper_limit_m: value.upper_limit_m,
            conductivity_m_s: value.conductivity_m_s,
            depth_m: value.depth_m,
            residual_theta: value.residual_theta,
            frozen_depth_m: value.frozen_depth_m,
            frozen_water_m: value.frozen_water_m,
            porosity: value.porosity,
            field_capacity_theta: value.field_capacity_theta,
            coca: value.coca,
            lateral_conductivity_m_s: value.lateral_conductivity_m_s,
        }
    }
}

impl Stage3AcceptedPublicationSupportV1 {
    fn to_wire(&self) -> Stage3AcceptedPublicationSupportWireV1 {
        let project_layers = |lanes: &[Vec<crate::DirectSubsurfaceLayerState>]| {
            lanes
                .iter()
                .map(|layers| layers.iter().map(Into::into).collect())
                .collect()
        };
        Stage3AcceptedPublicationSupportWireV1 {
            day_index: self.day_index,
            interval_index: self.interval_index,
            parent_transaction_id: self.parent_transaction_id,
            support: self.support,
            accepted_slab_sha256: self.accepted_slab_sha256,
            beginning_complete_owner_set_sha256: self.beginning_complete_owner_set_sha256,
            ending_complete_owner_set_sha256: self.ending_complete_owner_set_sha256,
            lse_support_receipt: self.lse_support_receipt.clone(),
            lse_forcing: self.lse_forcing.clone(),
            vegetation_forcing: self.vegetation_forcing.clone(),
            wb14_parameters: self.wb14_parameters.clone(),
            resource_debits: self.resource_debits.clone(),
            material_transfers: self.material_transfers.clone(),
            run_identity: self.run_identity.into(),
            beginning_lane_carries: self.beginning_lane_carries.clone(),
            beginning_subsurface_layers_by_lane: project_layers(
                &self.beginning_subsurface_layers_by_lane,
            ),
            ending_subsurface_layers_by_lane: project_layers(
                &self.ending_subsurface_layers_by_lane,
            ),
            surface_beginning_state: self.surface_beginning_state.clone(),
            surface_ending_state: self.surface_ending_state.clone(),
            open_ingress_parcels: self.open_ingress_parcels.clone(),
            ingress_receipts: self.ingress_receipts.clone(),
            ingress_ledgers: self.ingress_ledgers.clone(),
            accepted_snow_liquid_outputs: self.accepted_snow_liquid_outputs.clone(),
            wb14_child_replay_bytes: self.wb14_child_replay.materialize(),
            wb14_parent_replay_bytes: self.wb14_parent_replay_bytes.clone(),
            finalized_water_uses: self.finalized_water_uses.clone(),
            condensation_credits: self.condensation_credits.clone(),
            receiver_operands_sha256: self.receiver_operands_sha256,
            rollback_hashes: self.rollback_hashes.clone(),
            hydrology_transaction_id: self.hydrology_transaction_id,
            wb14_child_receipt_set_sha256: self.wb14_child_receipt_set_sha256,
            operands_sha256: self.operands_sha256,
            receipt_sha256: self.receipt_sha256,
        }
    }

    #[cfg(any(
        feature = "restart-authority-evidence",
        feature = "persisted-restart-v1"
    ))]
    fn try_from_wire(
        wire: Stage3AcceptedPublicationSupportWireV1,
    ) -> Result<Self, DirectV11RealConsumerError> {
        let run_identity = crate::DirectRunIdentity::new(
            wire.run_identity.run_id,
            wire.run_identity.hillslope_id,
            wire.run_identity.lane_count,
            wire.run_identity.day_count,
        )
        .map_err(|_| DirectV11RealConsumerError::Identity("publication run identity wire"))?;
        let restore_layers = |lanes: Vec<Vec<Stage3PublicationSubsurfaceLayerWireV1>>| {
            lanes
                .into_iter()
                .map(|layers| layers.into_iter().map(Into::into).collect())
                .collect()
        };
        let value = Self {
            day_index: wire.day_index,
            interval_index: wire.interval_index,
            parent_transaction_id: wire.parent_transaction_id,
            support: wire.support,
            accepted_slab_sha256: wire.accepted_slab_sha256,
            beginning_complete_owner_set_sha256: wire.beginning_complete_owner_set_sha256,
            ending_complete_owner_set_sha256: wire.ending_complete_owner_set_sha256,
            lse_support_receipt: wire.lse_support_receipt,
            lse_forcing: wire.lse_forcing,
            vegetation_forcing: wire.vegetation_forcing,
            wb14_parameters: wire.wb14_parameters,
            resource_debits: wire.resource_debits,
            material_transfers: wire.material_transfers,
            run_identity,
            beginning_lane_carries: wire.beginning_lane_carries,
            beginning_subsurface_layers_by_lane: restore_layers(
                wire.beginning_subsurface_layers_by_lane,
            ),
            ending_subsurface_layers_by_lane: restore_layers(wire.ending_subsurface_layers_by_lane),
            surface_beginning_state: wire.surface_beginning_state,
            surface_ending_state: wire.surface_ending_state,
            open_ingress_parcels: wire.open_ingress_parcels,
            ingress_receipts: wire.ingress_receipts,
            ingress_ledgers: wire.ingress_ledgers,
            accepted_snow_liquid_outputs: wire.accepted_snow_liquid_outputs,
            wb14_child_replay: PersistentCanonicalWb14ReplayV1::from_bytes(
                wire.wb14_child_replay_bytes,
            ),
            wb14_parent_replay_bytes: wire.wb14_parent_replay_bytes,
            finalized_water_uses: wire.finalized_water_uses,
            condensation_credits: wire.condensation_credits,
            receiver_operands_sha256: wire.receiver_operands_sha256,
            rollback_hashes: wire.rollback_hashes,
            hydrology_transaction_id: wire.hydrology_transaction_id,
            wb14_child_receipt_set_sha256: wire.wb14_child_receipt_set_sha256,
            operands_sha256: wire.operands_sha256,
            receipt_sha256: wire.receipt_sha256,
        };
        value.validate()?;
        Ok(value)
    }

    #[allow(clippy::too_many_arguments)]
    fn try_new(
        day_index: usize,
        interval_index: usize,
        input: &V11ImportedV10SegmentInput,
        ending_complete_owner_set_sha256: Digest32,
        lse_support_receipt: LseSupportAdmissibilityReceiptV1,
        lse_forcing: LandSurfaceForcing,
        vegetation_forcing: SnowFreeForcing,
        wb14_parameters: Vec<DirectOfeWb14Parameters>,
        resource_debits: Vec<V11ResourceDebit>,
        material_transfers: Vec<openwepp_vegetation::carbon_nitrogen::MaterialTransfer>,
        hydrology: &crate::land_surface_energy_shadow::UnifiedRealHydrologyCandidate,
        physical_outcome_ledgers: Option<
            &BTreeMap<u32, v11_covered::physical_outcome_ledger::Stage3LanePhysicalOutcomeLedgerV1>,
        >,
    ) -> Result<Self, DirectV11RealConsumerError> {
        #[cfg(test)]
        let capture_started = std::time::Instant::now();
        let beginning_states = input
            .staged_resource_owners
            .values()
            .map(V11OwnerEnvelope::to_owner_state)
            .collect::<Result<Vec<_>, _>>()?;
        let beginning_complete_owner_set_sha256 = complete_owner_set_digest(&beginning_states)
            .map_err(|_| {
                DirectV11RealConsumerError::Identity(
                    "accepted publication beginning complete-owner set",
                )
            })?;
        let ingress_receipt_set = digest32_from_lower_hex(
            hydrology
                .surface_ingress()
                .wb14_child_receipt_set_sha256()
                .as_str(),
        )?;
        let run_identity = hydrology.beginning_frame().identity;
        if hydrology.ending_frame().identity != run_identity {
            return Err(DirectV11RealConsumerError::Identity(
                "accepted publication hydrology run identity",
            ));
        }
        let beginning_lane_carries = hydrology
            .beginning_frame()
            .lanes
            .iter()
            .map(|lane| Stage3AcceptedBeginningLaneCarryV1 {
                lane_id: lane.lane_id,
                upstream_lane_id: lane.upstream_lane_id,
                downstream_lane_id: lane.downstream_lane_id,
                upstream_area_ratio: lane.upstream_area_ratio,
                surface_carry_m: lane.transfer.surface_carry_m,
                surface_hourly_weights: lane.transfer.surface_hourly_weights,
                lateral_carry_m: lane.transfer.lateral_carry_m,
                upstream_flow_m: lane.transfer.upstream_flow_m,
                subsurface_input_m: lane.transfer.subsurface_input_m,
            })
            .collect::<Vec<_>>();
        let beginning_subsurface_layers_by_lane = hydrology
            .beginning_frame()
            .lanes
            .iter()
            .map(|lane| lane.subsurface_layers.clone())
            .collect::<Vec<_>>();
        let ending_subsurface_layers_by_lane = hydrology
            .ending_frame()
            .lanes
            .iter()
            .map(|lane| lane.subsurface_layers.clone())
            .collect::<Vec<_>>();
        let ingress = hydrology.surface_ingress();
        let surface_beginning_state = ingress.beginning_state().clone();
        let surface_ending_state = ingress.ending_state().clone();
        let open_ingress_parcels = ingress.open_ingress_parcels().to_vec();
        let ingress_receipts = ingress.receipts().to_vec();
        let ingress_ledgers = ingress.ledgers().to_vec();
        let accepted_snow_liquid_outputs = physical_outcome_ledgers
            .map(|ledgers| {
                ledgers
                    .iter()
                    .map(|(lane_id, ledger)| {
                        if *lane_id != ledger.lane_id || ledger.support != input.support {
                            return Err(DirectV11RealConsumerError::Identity(
                                "accepted snow-liquid output support/lane",
                            ));
                        }
                        Stage3AcceptedSnowLiquidOutputV1::seal(ledger)
                    })
                    .collect::<Result<Vec<_>, _>>()
            })
            .transpose()?
            .unwrap_or_default();
        let wb14_child_replay_bytes = ingress.wb14_child_replay_bytes().to_vec();
        let wb14_child_replay =
            PersistentCanonicalWb14ReplayV1::from_bytes(wb14_child_replay_bytes);
        let wb14_parent_replay_bytes = ingress.wb14_parent_replay_bytes().map(<[u8]>::to_vec);
        let finalized_water_uses = hydrology.finalized_uses().to_vec();
        let condensation_credits = hydrology.condensation_credits().to_vec();
        let receiver_operands_sha256 =
            digest32_from_lower_hex(&hydrology.receiver_closure_operands().canonical_sha256())?;
        let rollback_hashes = hydrology.rollback_hashes().to_vec();
        let hydrology_transaction_id = hydrology.transaction_id();
        #[cfg(test)]
        let projection_finished = std::time::Instant::now();
        let operands_sha256 = Self::operands_sha256(
            &lse_support_receipt,
            &lse_forcing,
            &vegetation_forcing,
            &wb14_parameters,
            &resource_debits,
            &material_transfers,
            run_identity,
            &beginning_lane_carries,
            &beginning_subsurface_layers_by_lane,
            &ending_subsurface_layers_by_lane,
            &surface_beginning_state,
            &surface_ending_state,
            &open_ingress_parcels,
            &ingress_receipts,
            &ingress_ledgers,
            &accepted_snow_liquid_outputs,
            wb14_child_replay.canonical_sha256(),
            wb14_parent_replay_bytes
                .as_deref()
                .map_or_else(|| digest_bytes(b"no-parent-replay"), digest_bytes),
            &finalized_water_uses,
            &condensation_credits,
            receiver_operands_sha256,
            &rollback_hashes,
        )?;
        #[cfg(test)]
        let initial_seal_finished = std::time::Instant::now();
        let receipt_sha256 = Self::reconstructed_receipt_sha256(
            day_index,
            interval_index,
            input.parent_transaction_id,
            input.support,
            input.accepted_slab_receipt.slab_id().digest(),
            beginning_complete_owner_set_sha256,
            ending_complete_owner_set_sha256,
            hydrology_transaction_id,
            ingress_receipt_set,
            operands_sha256,
        )?;
        let value = Self {
            day_index,
            interval_index,
            parent_transaction_id: input.parent_transaction_id,
            support: input.support,
            accepted_slab_sha256: input.accepted_slab_receipt.slab_id().digest(),
            beginning_complete_owner_set_sha256,
            ending_complete_owner_set_sha256,
            lse_support_receipt,
            lse_forcing,
            vegetation_forcing,
            wb14_parameters,
            resource_debits,
            material_transfers,
            run_identity,
            beginning_lane_carries,
            beginning_subsurface_layers_by_lane,
            ending_subsurface_layers_by_lane,
            surface_beginning_state,
            surface_ending_state,
            open_ingress_parcels,
            ingress_receipts,
            ingress_ledgers,
            accepted_snow_liquid_outputs,
            wb14_child_replay,
            wb14_parent_replay_bytes,
            finalized_water_uses,
            condensation_credits,
            receiver_operands_sha256,
            rollback_hashes,
            hydrology_transaction_id,
            wb14_child_receipt_set_sha256: ingress_receipt_set,
            operands_sha256,
            receipt_sha256,
        };
        value.validate()?;
        #[cfg(test)]
        {
            let validation_finished = std::time::Instant::now();
            crate::snow_stage3_v11_attachment::record_accepted_publication_capture_audit(
                crate::snow_stage3_v11_attachment::AcceptedPublicationCaptureAuditV1 {
                    support: input.support,
                    regime:
                        crate::snow_stage3_v11_attachment::accepted_publication_capture_regime_v1(),
                    projection_elapsed: projection_finished.duration_since(capture_started),
                    initial_seal_elapsed: initial_seal_finished.duration_since(projection_finished),
                    validation_elapsed: validation_finished.duration_since(initial_seal_finished),
                    total_elapsed: validation_finished.duration_since(capture_started),
                },
            );
        }
        Ok(value)
    }

    #[allow(clippy::too_many_arguments)]
    fn reconstructed_receipt_sha256(
        day_index: usize,
        interval_index: usize,
        parent_transaction_id: ParentTransactionId,
        support: TimeSupport,
        accepted_slab_sha256: Digest32,
        beginning_complete_owner_set_sha256: Digest32,
        ending_complete_owner_set_sha256: Digest32,
        hydrology_transaction_id: TransactionId,
        ingress_receipt_set: Digest32,
        operands_sha256: Digest32,
    ) -> Result<Digest32, DirectV11RealConsumerError> {
        let day_index_bytes = u64::try_from(day_index)
            .map_err(|_| DirectV11RealConsumerError::Identity("publication day index width"))?
            .to_be_bytes();
        let interval_index_bytes = u64::try_from(interval_index)
            .map_err(|_| DirectV11RealConsumerError::Identity("publication interval index width"))?
            .to_be_bytes();
        let support_start = support.start_ns().get().to_be_bytes();
        let support_end = support.end_ns().get().to_be_bytes();
        let transaction_id = hydrology_transaction_id.0.to_be_bytes();
        framed_sha256(
            "stage3-accepted-publication-support-v1",
            &[
                FramedField {
                    tag: "day_index",
                    value: &day_index_bytes,
                },
                FramedField {
                    tag: "interval_index",
                    value: &interval_index_bytes,
                },
                FramedField {
                    tag: "parent_transaction_id",
                    value: parent_transaction_id.digest().as_bytes(),
                },
                FramedField {
                    tag: "support_start_ns",
                    value: &support_start,
                },
                FramedField {
                    tag: "support_end_ns",
                    value: &support_end,
                },
                FramedField {
                    tag: "accepted_slab_sha256",
                    value: accepted_slab_sha256.as_bytes(),
                },
                FramedField {
                    tag: "beginning_complete_owner_set_sha256",
                    value: beginning_complete_owner_set_sha256.as_bytes(),
                },
                FramedField {
                    tag: "ending_complete_owner_set_sha256",
                    value: ending_complete_owner_set_sha256.as_bytes(),
                },
                FramedField {
                    tag: "hydrology_transaction_id",
                    value: &transaction_id,
                },
                FramedField {
                    tag: "wb14_child_receipt_set_sha256",
                    value: ingress_receipt_set.as_bytes(),
                },
                FramedField {
                    tag: "accepted_operands_sha256",
                    value: operands_sha256.as_bytes(),
                },
            ],
        )
        .map_err(|_| DirectV11RealConsumerError::Identity("accepted publication support seal"))
    }

    fn json_digest<T: Serialize + ?Sized>(
        value: &T,
    ) -> Result<Digest32, DirectV11RealConsumerError> {
        let bytes = serde_json::to_vec(value).map_err(|_| {
            DirectV11RealConsumerError::Identity("accepted publication operand serialization")
        })?;
        Ok(digest_bytes(&bytes))
    }

    #[allow(clippy::too_many_arguments)]
    fn operands_sha256(
        lse_support_receipt: &LseSupportAdmissibilityReceiptV1,
        lse_forcing: &LandSurfaceForcing,
        vegetation_forcing: &SnowFreeForcing,
        wb14_parameters: &[DirectOfeWb14Parameters],
        resource_debits: &[V11ResourceDebit],
        material_transfers: &[openwepp_vegetation::carbon_nitrogen::MaterialTransfer],
        run_identity: crate::DirectRunIdentity,
        beginning_lane_carries: &[Stage3AcceptedBeginningLaneCarryV1],
        beginning_layers: &[Vec<crate::DirectSubsurfaceLayerState>],
        ending_layers: &[Vec<crate::DirectSubsurfaceLayerState>],
        surface_beginning_state: &crate::DirectSurfaceLiquidOwnedState,
        surface_ending_state: &crate::DirectSurfaceLiquidOwnedState,
        open_ingress_parcels: &[crate::direct_runtime::DirectOpenLiquidIngressParcel],
        ingress_receipts: &[crate::direct_runtime::DirectSurfaceLiquidParcelReceipt],
        ingress_ledgers: &[crate::direct_runtime::DirectSurfaceLiquidIngressLedger],
        accepted_snow_liquid_outputs: &[Stage3AcceptedSnowLiquidOutputV1],
        wb14_child_replay_sha256: Digest32,
        wb14_parent_replay_sha256: Digest32,
        finalized_water_uses: &[openwepp_land_surface_energy::WaterAmount],
        condensation_credits: &[openwepp_land_surface_energy::CondensationCredit],
        receiver_operands_sha256: Digest32,
        rollback_hashes: &[openwepp_land_surface_energy::OwnerRollbackHash],
    ) -> Result<Digest32, DirectV11RealConsumerError> {
        let lse_receipt = Self::json_digest(lse_support_receipt)?;
        let lse = Self::json_digest(lse_forcing)?;
        let vegetation = Self::json_digest(vegetation_forcing)?;
        let wb14 = Self::json_digest(wb14_parameters)?;
        let debits = Self::json_digest(resource_debits)?;
        let material = Self::json_digest(material_transfers)?;
        let lane_carries = Self::json_digest(beginning_lane_carries)?;
        let water_uses = Self::json_digest(finalized_water_uses)?;
        let condensation = Self::json_digest(condensation_credits)?;
        let surface_beginning = Self::json_digest(surface_beginning_state)?;
        let surface_ending = Self::json_digest(surface_ending_state)?;
        let open_ingress = Self::json_digest(open_ingress_parcels)?;
        let ingress_receipts = Self::json_digest(ingress_receipts)?;
        let ingress_ledgers = Self::json_digest(ingress_ledgers)?;
        let snow_liquid_outputs = Self::json_digest(accepted_snow_liquid_outputs)?;
        let rollback = Self::json_digest(rollback_hashes)?;
        let layers = Self::subsurface_layers_sha256(run_identity, beginning_layers, ending_layers)?;
        framed_sha256(
            "stage3-accepted-publication-operands-v1",
            &[
                FramedField {
                    tag: "lse_support_receipt",
                    value: lse_receipt.as_bytes(),
                },
                FramedField {
                    tag: "lse_forcing",
                    value: lse.as_bytes(),
                },
                FramedField {
                    tag: "vegetation_forcing",
                    value: vegetation.as_bytes(),
                },
                FramedField {
                    tag: "wb14_parameters",
                    value: wb14.as_bytes(),
                },
                FramedField {
                    tag: "resource_debits",
                    value: debits.as_bytes(),
                },
                FramedField {
                    tag: "material_transfers",
                    value: material.as_bytes(),
                },
                FramedField {
                    tag: "beginning_lane_carries",
                    value: lane_carries.as_bytes(),
                },
                FramedField {
                    tag: "water_uses",
                    value: water_uses.as_bytes(),
                },
                FramedField {
                    tag: "condensation_credits",
                    value: condensation.as_bytes(),
                },
                FramedField {
                    tag: "subsurface_layers",
                    value: layers.as_bytes(),
                },
                FramedField {
                    tag: "surface_beginning",
                    value: surface_beginning.as_bytes(),
                },
                FramedField {
                    tag: "surface_ending",
                    value: surface_ending.as_bytes(),
                },
                FramedField {
                    tag: "open_ingress_parcels",
                    value: open_ingress.as_bytes(),
                },
                FramedField {
                    tag: "ingress_receipts",
                    value: ingress_receipts.as_bytes(),
                },
                FramedField {
                    tag: "ingress_ledgers",
                    value: ingress_ledgers.as_bytes(),
                },
                FramedField {
                    tag: "accepted_snow_liquid_outputs",
                    value: snow_liquid_outputs.as_bytes(),
                },
                FramedField {
                    tag: "wb14_child_replay",
                    value: wb14_child_replay_sha256.as_bytes(),
                },
                FramedField {
                    tag: "wb14_parent_replay",
                    value: wb14_parent_replay_sha256.as_bytes(),
                },
                FramedField {
                    tag: "receiver_operands",
                    value: receiver_operands_sha256.as_bytes(),
                },
                FramedField {
                    tag: "rollback_hashes",
                    value: rollback.as_bytes(),
                },
            ],
        )
        .map_err(|_| DirectV11RealConsumerError::Identity("accepted publication operand seal"))
    }

    fn subsurface_layers_sha256(
        identity: crate::DirectRunIdentity,
        beginning: &[Vec<crate::DirectSubsurfaceLayerState>],
        ending: &[Vec<crate::DirectSubsurfaceLayerState>],
    ) -> Result<Digest32, DirectV11RealConsumerError> {
        if beginning.len() != identity.lane_count || ending.len() != identity.lane_count {
            return Err(DirectV11RealConsumerError::Identity(
                "accepted publication subsurface lane cardinality",
            ));
        }
        let lane_count = u64::try_from(identity.lane_count).map_err(|_| {
            DirectV11RealConsumerError::Identity("accepted publication lane-count width")
        })?;
        let day_count = u64::try_from(identity.day_count).map_err(|_| {
            DirectV11RealConsumerError::Identity("accepted publication day-count width")
        })?;
        let mut bytes = Vec::new();
        bytes.extend_from_slice(b"OPENWEPP_ACCEPTED_PUBLICATION_SUBSURFACE_V1\0");
        bytes.extend_from_slice(&identity.run_id.to_be_bytes());
        bytes.extend_from_slice(&identity.hillslope_id.to_be_bytes());
        bytes.extend_from_slice(&lane_count.to_be_bytes());
        bytes.extend_from_slice(&day_count.to_be_bytes());
        for lanes in [beginning, ending] {
            for layers in lanes {
                let layer_count = u64::try_from(layers.len()).map_err(|_| {
                    DirectV11RealConsumerError::Identity("accepted publication layer-count width")
                })?;
                bytes.extend_from_slice(&layer_count.to_be_bytes());
                for layer in layers {
                    for value in [
                        layer.theta_m,
                        layer.field_capacity_m,
                        layer.upper_limit_m,
                        layer.conductivity_m_s,
                        layer.depth_m,
                        layer.residual_theta,
                        layer.frozen_depth_m,
                        layer.frozen_water_m,
                        layer.porosity,
                        layer.field_capacity_theta,
                        layer.coca,
                        layer.lateral_conductivity_m_s,
                    ] {
                        bytes.extend_from_slice(&value.to_bits().to_be_bytes());
                    }
                }
            }
        }
        Ok(digest_bytes(&bytes))
    }

    fn validate(&self) -> Result<(), DirectV11RealConsumerError> {
        let ingress_receipt_set = self.wb14_child_receipt_set_sha256;
        let operands_sha256 = Self::operands_sha256(
            &self.lse_support_receipt,
            &self.lse_forcing,
            &self.vegetation_forcing,
            &self.wb14_parameters,
            &self.resource_debits,
            &self.material_transfers,
            self.run_identity,
            &self.beginning_lane_carries,
            &self.beginning_subsurface_layers_by_lane,
            &self.ending_subsurface_layers_by_lane,
            &self.surface_beginning_state,
            &self.surface_ending_state,
            &self.open_ingress_parcels,
            &self.ingress_receipts,
            &self.ingress_ledgers,
            &self.accepted_snow_liquid_outputs,
            self.wb14_child_replay.canonical_sha256(),
            self.wb14_parent_replay_bytes
                .as_deref()
                .map_or_else(|| digest_bytes(b"no-parent-replay"), digest_bytes),
            &self.finalized_water_uses,
            &self.condensation_credits,
            self.receiver_operands_sha256,
            &self.rollback_hashes,
        )?;
        let receipt_sha256 = Self::reconstructed_receipt_sha256(
            self.day_index,
            self.interval_index,
            self.parent_transaction_id,
            self.support,
            self.accepted_slab_sha256,
            self.beginning_complete_owner_set_sha256,
            self.ending_complete_owner_set_sha256,
            self.hydrology_transaction_id,
            ingress_receipt_set,
            operands_sha256,
        )?;
        if self.support.duration_ns() == 0
            || self.accepted_slab_sha256 == Digest32::zero()
            || self.beginning_complete_owner_set_sha256 == Digest32::zero()
            || self.ending_complete_owner_set_sha256 == Digest32::zero()
            || self.wb14_child_receipt_set_sha256 != self.wb14_child_replay.canonical_sha256()
            || self.operands_sha256 != operands_sha256
            || self.receipt_sha256 != receipt_sha256
            || self.receipt_sha256 == Digest32::zero()
            || self.lse_forcing.transaction_id != self.hydrology_transaction_id
            || self.lse_forcing.interval_s.to_bits() != self.support.duration_s_bits()
            || self.wb14_parameters.is_empty()
            || self.beginning_lane_carries.len() != self.run_identity.lane_count
            || self
                .beginning_lane_carries
                .iter()
                .enumerate()
                .any(|(index, lane)| {
                    u32::try_from(index + 1).ok() != Some(lane.lane_id)
                        || !lane.upstream_area_ratio.is_finite()
                        || lane.upstream_area_ratio < 0.0
                        || !lane.upstream_flow_m.is_finite()
                        || !lane.subsurface_input_m.is_finite()
                        || lane
                            .surface_carry_m
                            .iter()
                            .chain(&lane.surface_hourly_weights)
                            .chain(&lane.lateral_carry_m)
                            .any(|value| !value.is_finite() || *value < 0.0)
                })
            || self.resource_debits.iter().any(|debit| {
                debit.parent_transaction_id != self.parent_transaction_id
                    || debit.support != self.support
            })
            || self
                .accepted_snow_liquid_outputs
                .iter()
                .any(|output| output.support != self.support || output.validate().is_err())
            || self
                .accepted_snow_liquid_outputs
                .windows(2)
                .any(|pair| pair[0].lane_id >= pair[1].lane_id)
        {
            return Err(DirectV11RealConsumerError::Identity(
                "accepted publication support identity",
            ));
        }
        Ok(())
    }

    #[must_use]
    pub(crate) const fn day_index(&self) -> usize {
        self.day_index
    }

    #[must_use]
    pub(crate) const fn interval_index(&self) -> usize {
        self.interval_index
    }

    #[must_use]
    pub(crate) const fn parent_transaction_id(&self) -> ParentTransactionId {
        self.parent_transaction_id
    }

    #[must_use]
    pub(crate) const fn support(&self) -> TimeSupport {
        self.support
    }

    #[must_use]
    pub(crate) fn accepted_snow_liquid_outputs(&self) -> &[Stage3AcceptedSnowLiquidOutputV1] {
        &self.accepted_snow_liquid_outputs
    }

    #[must_use]
    pub(crate) const fn accepted_slab_sha256(&self) -> Digest32 {
        self.accepted_slab_sha256
    }

    #[must_use]
    pub(crate) const fn beginning_complete_owner_set_sha256(&self) -> Digest32 {
        self.beginning_complete_owner_set_sha256
    }

    #[must_use]
    pub(crate) const fn ending_complete_owner_set_sha256(&self) -> Digest32 {
        self.ending_complete_owner_set_sha256
    }

    #[must_use]
    pub(crate) const fn receipt_sha256(&self) -> Digest32 {
        self.receipt_sha256
    }

    #[must_use]
    pub(crate) const fn run_identity(&self) -> crate::DirectRunIdentity {
        self.run_identity
    }

    #[must_use]
    pub(crate) fn beginning_lane_carries(&self) -> &[Stage3AcceptedBeginningLaneCarryV1] {
        &self.beginning_lane_carries
    }

    #[must_use]
    pub(crate) fn beginning_subsurface_layers(
        &self,
        lane_index: usize,
    ) -> Option<&[crate::DirectSubsurfaceLayerState]> {
        self.beginning_subsurface_layers_by_lane
            .get(lane_index)
            .map(Vec::as_slice)
    }

    #[must_use]
    pub(crate) fn ending_subsurface_layers(
        &self,
        lane_index: usize,
    ) -> Option<&[crate::DirectSubsurfaceLayerState]> {
        self.ending_subsurface_layers_by_lane
            .get(lane_index)
            .map(Vec::as_slice)
    }

    #[must_use]
    pub(crate) const fn surface_beginning_state(&self) -> &crate::DirectSurfaceLiquidOwnedState {
        &self.surface_beginning_state
    }

    #[must_use]
    pub(crate) const fn surface_ending_state(&self) -> &crate::DirectSurfaceLiquidOwnedState {
        &self.surface_ending_state
    }

    #[must_use]
    pub(crate) fn ingress_receipts(
        &self,
    ) -> &[crate::direct_runtime::DirectSurfaceLiquidParcelReceipt] {
        &self.ingress_receipts
    }

    #[must_use]
    pub(crate) fn ingress_ledgers(
        &self,
    ) -> &[crate::direct_runtime::DirectSurfaceLiquidIngressLedger] {
        &self.ingress_ledgers
    }

    #[must_use]
    pub(crate) fn finalized_water_uses(&self) -> &[openwepp_land_surface_energy::WaterAmount] {
        &self.finalized_water_uses
    }

    #[must_use]
    pub(crate) const fn accepted_infiltration_is_installed(&self) -> bool {
        true
    }

    #[must_use]
    pub(crate) const fn lse_forcing(&self) -> &LandSurfaceForcing {
        &self.lse_forcing
    }

    #[must_use]
    #[cfg(test)]
    pub(crate) const fn vegetation_forcing(&self) -> &SnowFreeForcing {
        &self.vegetation_forcing
    }

    #[must_use]
    #[cfg(test)]
    pub(crate) fn wb14_parameters(&self) -> &[DirectOfeWb14Parameters] {
        &self.wb14_parameters
    }

    #[must_use]
    pub(crate) fn resource_debits(&self) -> &[V11ResourceDebit] {
        &self.resource_debits
    }

    #[must_use]
    pub(crate) fn material_transfers(
        &self,
    ) -> &[openwepp_vegetation::carbon_nitrogen::MaterialTransfer] {
        &self.material_transfers
    }
}

#[cfg(test)]
#[path = "v9_real_consumer_shadow_publication_retention/accepted_snow_liquid_output_tests.rs"]
mod accepted_snow_liquid_output_tests;
