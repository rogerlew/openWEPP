//! Private validation-once custody for accepted Stage-3 publication supports.
//!
//! No type in this module has a wire, archive, checkpoint, restart, or public
//! representation. The owning capability deliberately does not implement
//! `Clone`; trusted append is its only consumer.

use std::sync::Arc;

use openwepp_coupled_time::{Digest32, ModelTimeNs, ParentTransactionId, TimeSupport};

use super::{
    AcceptedPublicationHistoryV1, AcceptedPublicationTailAuthorityV1, DirectV11RealConsumerError,
    PersistentCanonicalWb14ReplayV1, Stage3AcceptedPublicationSupportV1, digest_bytes,
};

#[cfg(test)]
use super::FORCE_FULL_SCAN_ACCEPTED_PUBLICATION_HISTORY_V1;

impl Stage3AcceptedPublicationSupportV1 {
    #[cfg(any(
        test,
        feature = "restart-authority-evidence",
        feature = "persisted-restart-v1"
    ))]
    pub(super) fn validate(&self) -> Result<(), DirectV11RealConsumerError> {
        #[cfg(test)]
        record_full_validation_attempt_v1();
        #[cfg(test)]
        record_append_time_full_validation_v1();
        let ingress_receipt_set = self.wb14_child_receipt_set_sha256;
        #[cfg(test)]
        record_operand_seal_v1();
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
        #[cfg(test)]
        record_receipt_seal_v1();
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
        self.validate_semantics()?;
        if self.operands_sha256 != operands_sha256 || self.receipt_sha256 != receipt_sha256 {
            return Err(DirectV11RealConsumerError::Identity(
                "accepted publication support identity",
            ));
        }
        #[cfg(test)]
        record_full_validation_success_v1();
        Ok(())
    }

    pub(super) fn validate_semantics(&self) -> Result<(), DirectV11RealConsumerError> {
        if self.support.duration_ns() == 0
            || self.accepted_slab_sha256 == Digest32::zero()
            || self.beginning_complete_owner_set_sha256 == Digest32::zero()
            || self.ending_complete_owner_set_sha256 == Digest32::zero()
            || self.wb14_child_receipt_set_sha256 != self.wb14_child_replay.canonical_sha256()
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
}

#[derive(Debug)]
pub(super) struct AcceptedPublicationHistoryIncarnationV1;

/// Complete cached process-local history identity used by trusted append.
///
/// Every field is bounded-size. In particular, the unbounded event-id and
/// ordinal maps remain authenticated by the incrementally maintained tail
/// digest rather than being cloned into this revision.
#[derive(Clone, Debug)]
pub(super) struct AcceptedPublicationHistoryLiveRevisionV1 {
    pub(super) incarnation: Arc<AcceptedPublicationHistoryIncarnationV1>,
    pub(super) sequence: u64,
    pub(super) cumulative_support_count: usize,
    pub(super) cumulative_event_count: usize,
    pub(super) resident_support_count: usize,
    pub(super) resident_event_count: usize,
    pub(super) last_day_index: Option<usize>,
    pub(super) last_interval_index: Option<usize>,
    pub(super) last_support: Option<TimeSupport>,
    pub(super) last_parent_transaction_id: Option<ParentTransactionId>,
    pub(super) last_accepted_slab_sha256: Option<Digest32>,
    pub(super) traversed_ending_owner_sha256: Option<Digest32>,
    pub(super) pending_pre_support_event: Option<(ParentTransactionId, ModelTimeNs)>,
    pub(super) event_id_count: usize,
    pub(super) current_event_ordinal: Option<u32>,
    pub(super) sealed_prefix_support_count: usize,
    pub(super) sealed_prefix_event_count: usize,
    pub(super) sealed_prefix_authority_sha256: Digest32,
    pub(super) wb14_checkpoint_sha256: Option<Digest32>,
    pub(super) last_wb14_replay_sha256: Option<Digest32>,
    pub(super) last_support_receipt_sha256: Option<Digest32>,
    pub(super) aggregate_tail_sha256: Digest32,
}

impl PartialEq for AcceptedPublicationHistoryLiveRevisionV1 {
    fn eq(&self, other: &Self) -> bool {
        Arc::ptr_eq(&self.incarnation, &other.incarnation)
            && self.sequence == other.sequence
            && self.cumulative_support_count == other.cumulative_support_count
            && self.cumulative_event_count == other.cumulative_event_count
            && self.resident_support_count == other.resident_support_count
            && self.resident_event_count == other.resident_event_count
            && self.last_day_index == other.last_day_index
            && self.last_interval_index == other.last_interval_index
            && self.last_support == other.last_support
            && self.last_parent_transaction_id == other.last_parent_transaction_id
            && self.last_accepted_slab_sha256 == other.last_accepted_slab_sha256
            && self.traversed_ending_owner_sha256 == other.traversed_ending_owner_sha256
            && self.pending_pre_support_event == other.pending_pre_support_event
            && self.event_id_count == other.event_id_count
            && self.current_event_ordinal == other.current_event_ordinal
            && self.sealed_prefix_support_count == other.sealed_prefix_support_count
            && self.sealed_prefix_event_count == other.sealed_prefix_event_count
            && self.sealed_prefix_authority_sha256 == other.sealed_prefix_authority_sha256
            && self.wb14_checkpoint_sha256 == other.wb14_checkpoint_sha256
            && self.last_wb14_replay_sha256 == other.last_wb14_replay_sha256
            && self.last_support_receipt_sha256 == other.last_support_receipt_sha256
            && self.aggregate_tail_sha256 == other.aggregate_tail_sha256
    }
}

impl Eq for AcceptedPublicationHistoryLiveRevisionV1 {}

impl AcceptedPublicationHistoryLiveRevisionV1 {
    pub(super) fn fresh(
        tail: &AcceptedPublicationTailAuthorityV1,
        sealed_prefix: &AcceptedPublicationTailAuthorityV1,
        resident_support_count: usize,
        resident_event_count: usize,
        wb14_checkpoint: Option<&PersistentCanonicalWb14ReplayV1>,
        last_support: Option<&Stage3AcceptedPublicationSupportV1>,
    ) -> Self {
        Self::from_parts(
            Arc::new(AcceptedPublicationHistoryIncarnationV1),
            0,
            tail,
            sealed_prefix,
            resident_support_count,
            resident_event_count,
            wb14_checkpoint,
            last_support,
        )
    }

    #[allow(clippy::too_many_arguments)]
    fn from_parts(
        incarnation: Arc<AcceptedPublicationHistoryIncarnationV1>,
        sequence: u64,
        tail: &AcceptedPublicationTailAuthorityV1,
        sealed_prefix: &AcceptedPublicationTailAuthorityV1,
        resident_support_count: usize,
        resident_event_count: usize,
        wb14_checkpoint: Option<&PersistentCanonicalWb14ReplayV1>,
        last_support: Option<&Stage3AcceptedPublicationSupportV1>,
    ) -> Self {
        let current_event_parent = tail
            .pending_pre_support_event
            .map(|(parent, _)| parent)
            .or(tail.last_parent_transaction_id);
        let current_event_ordinal = current_event_parent
            .and_then(|parent| tail.last_event_ordinal_by_parent.get(&parent).copied());
        Self {
            incarnation,
            sequence,
            cumulative_support_count: tail.support_count,
            cumulative_event_count: tail.event_count,
            resident_support_count,
            resident_event_count,
            last_day_index: tail.last_day_index,
            last_interval_index: tail.last_interval_index,
            last_support: tail.last_support,
            last_parent_transaction_id: tail.last_parent_transaction_id,
            last_accepted_slab_sha256: tail.last_accepted_slab_sha256,
            traversed_ending_owner_sha256: tail.traversed_ending_owner_sha256,
            pending_pre_support_event: tail.pending_pre_support_event,
            event_id_count: tail.event_ids.len(),
            current_event_ordinal,
            sealed_prefix_support_count: sealed_prefix.support_count,
            sealed_prefix_event_count: sealed_prefix.event_count,
            sealed_prefix_authority_sha256: sealed_prefix.aggregate_authority_sha256,
            wb14_checkpoint_sha256: wb14_checkpoint
                .map(PersistentCanonicalWb14ReplayV1::canonical_sha256),
            last_wb14_replay_sha256: last_support
                .map(|support| support.wb14_child_replay.canonical_sha256()),
            last_support_receipt_sha256: last_support.map(|support| support.receipt_sha256),
            aggregate_tail_sha256: tail.aggregate_authority_sha256,
        }
    }

    #[allow(clippy::too_many_arguments)]
    pub(super) fn successor(
        &self,
        tail: &AcceptedPublicationTailAuthorityV1,
        sealed_prefix: &AcceptedPublicationTailAuthorityV1,
        resident_support_count: usize,
        resident_event_count: usize,
        wb14_checkpoint: Option<&PersistentCanonicalWb14ReplayV1>,
        last_support: Option<&Stage3AcceptedPublicationSupportV1>,
    ) -> Result<Self, DirectV11RealConsumerError> {
        let sequence = self
            .sequence
            .checked_add(1)
            .ok_or(DirectV11RealConsumerError::Identity(
                "accepted publication live-revision overflow",
            ))?;
        Ok(Self::from_parts(
            Arc::clone(&self.incarnation),
            sequence,
            tail,
            sealed_prefix,
            resident_support_count,
            resident_event_count,
            wb14_checkpoint,
            last_support,
        ))
    }
}

/// Owning, private, move-only proof that one support completed its full
/// semantic and seal pass against one exact process-local history revision.
pub(crate) struct ValidatedStage3AcceptedPublicationSupportV1 {
    support: Stage3AcceptedPublicationSupportV1,
    target_revision: AcceptedPublicationHistoryLiveRevisionV1,
    support_receipt_sha256: Digest32,
}

/// Unsealed, move-only projection used only while a snow-free provisional
/// slab is awaiting its final accepted-slab identity. It is not appendable and
/// has no wire representation; only the final validation boundary can mint the
/// trusted capability.
pub(crate) struct PreparedStage3AcceptedPublicationSupportV1 {
    support: Stage3AcceptedPublicationSupportV1,
    #[cfg(test)]
    projection_elapsed: std::time::Duration,
}

impl PreparedStage3AcceptedPublicationSupportV1 {
    pub(super) fn new(support: Stage3AcceptedPublicationSupportV1) -> Self {
        Self {
            support,
            #[cfg(test)]
            projection_elapsed: std::time::Duration::ZERO,
        }
    }

    #[cfg(test)]
    pub(super) fn with_projection_elapsed(mut self, elapsed: std::time::Duration) -> Self {
        self.projection_elapsed = elapsed;
        self
    }

    pub(super) fn into_support_for_reseal(self) -> Stage3AcceptedPublicationSupportV1 {
        self.support
    }

    pub(super) fn validate_and_mint(
        mut self,
        target_revision: AcceptedPublicationHistoryLiveRevisionV1,
    ) -> Result<ValidatedStage3AcceptedPublicationSupportV1, DirectV11RealConsumerError> {
        #[cfg(test)]
        record_full_validation_attempt_v1();
        #[cfg(test)]
        let seal_started = std::time::Instant::now();
        #[cfg(test)]
        record_operand_seal_v1();
        self.support.operands_sha256 = Stage3AcceptedPublicationSupportV1::operands_sha256(
            &self.support.lse_support_receipt,
            &self.support.lse_forcing,
            &self.support.vegetation_forcing,
            &self.support.wb14_parameters,
            &self.support.resource_debits,
            &self.support.material_transfers,
            self.support.run_identity,
            &self.support.beginning_lane_carries,
            &self.support.beginning_subsurface_layers_by_lane,
            &self.support.ending_subsurface_layers_by_lane,
            &self.support.surface_beginning_state,
            &self.support.surface_ending_state,
            &self.support.open_ingress_parcels,
            &self.support.ingress_receipts,
            &self.support.ingress_ledgers,
            &self.support.accepted_snow_liquid_outputs,
            self.support.wb14_child_replay.canonical_sha256(),
            self.support
                .wb14_parent_replay_bytes
                .as_deref()
                .map_or_else(|| digest_bytes(b"no-parent-replay"), digest_bytes),
            &self.support.finalized_water_uses,
            &self.support.condensation_credits,
            self.support.receiver_operands_sha256,
            &self.support.rollback_hashes,
        )?;
        #[cfg(test)]
        record_receipt_seal_v1();
        let receipt_sha256 = Stage3AcceptedPublicationSupportV1::reconstructed_receipt_sha256(
            self.support.day_index,
            self.support.interval_index,
            self.support.parent_transaction_id,
            self.support.support,
            self.support.accepted_slab_sha256,
            self.support.beginning_complete_owner_set_sha256,
            self.support.ending_complete_owner_set_sha256,
            self.support.hydrology_transaction_id,
            self.support.wb14_child_receipt_set_sha256,
            self.support.operands_sha256,
        )?;
        self.support.receipt_sha256 = receipt_sha256;
        #[cfg(test)]
        crate::v9_real_consumer_shadow::v11_covered::canonical_covered_final_constructor_boundary_v1(
            crate::v9_real_consumer_shadow::v11_covered::CanonicalCoveredFinalConstructorStageV1::PublicationSupport,
        );
        #[cfg(test)]
        if matches!(
            crate::v9_real_consumer_shadow::v11_covered::canonical_covered_parity_poison_v1(),
            Some(
                crate::v9_real_consumer_shadow::v11_covered::CanonicalCoveredPhysicalParityPoisonV1::PublicationSupport,
            )
        ) {
            self.support.receipt_sha256 = Digest32::zero();
        }
        #[cfg(test)]
        let initial_seal_finished = std::time::Instant::now();
        self.support.validate_semantics()?;
        if self.support.receipt_sha256 != receipt_sha256 {
            return Err(DirectV11RealConsumerError::Identity(
                "accepted publication support identity",
            ));
        }
        #[cfg(test)]
        record_full_validation_success_v1();
        #[cfg(test)]
        {
            let validation_finished = std::time::Instant::now();
            crate::snow_stage3_v11_attachment::record_accepted_publication_capture_audit(
                crate::snow_stage3_v11_attachment::AcceptedPublicationCaptureAuditV1 {
                    support: self.support.support,
                    regime:
                        crate::snow_stage3_v11_attachment::accepted_publication_capture_regime_v1(),
                    projection_elapsed: self.projection_elapsed,
                    initial_seal_elapsed: initial_seal_finished.duration_since(seal_started),
                    validation_elapsed: validation_finished.duration_since(initial_seal_finished),
                    total_elapsed: self.projection_elapsed
                        + validation_finished.duration_since(seal_started),
                },
            );
        }
        Ok(ValidatedStage3AcceptedPublicationSupportV1::mint(
            self.support,
            target_revision,
        ))
    }
}

impl ValidatedStage3AcceptedPublicationSupportV1 {
    pub(super) fn mint(
        support: Stage3AcceptedPublicationSupportV1,
        target_revision: AcceptedPublicationHistoryLiveRevisionV1,
    ) -> Self {
        #[cfg(test)]
        record_capability_mint_v1();
        let support_receipt_sha256 = support.receipt_sha256;
        Self {
            support,
            target_revision,
            support_receipt_sha256,
        }
    }

    pub(super) fn into_parts(
        self,
    ) -> (
        Stage3AcceptedPublicationSupportV1,
        AcceptedPublicationHistoryLiveRevisionV1,
        Digest32,
    ) {
        (
            self.support,
            self.target_revision,
            self.support_receipt_sha256,
        )
    }

    #[cfg(test)]
    pub(super) fn poison_target_for_test(
        mut self,
        poison: AcceptedPublicationLiveRevisionPoisonV1,
    ) -> Self {
        use AcceptedPublicationLiveRevisionPoisonV1 as Poison;
        let revision = &mut self.target_revision;
        match poison {
            Poison::ForeignIncarnation => {
                revision.incarnation = Arc::new(AcceptedPublicationHistoryIncarnationV1);
            }
            Poison::Sequence => revision.sequence = revision.sequence.wrapping_add(1),
            Poison::CumulativeSupportCount => {
                revision.cumulative_support_count =
                    revision.cumulative_support_count.wrapping_add(1);
            }
            Poison::CumulativeEventCount => {
                revision.cumulative_event_count = revision.cumulative_event_count.wrapping_add(1);
            }
            Poison::ResidentSupportCount => {
                revision.resident_support_count = revision.resident_support_count.wrapping_add(1);
            }
            Poison::ResidentEventCount => {
                revision.resident_event_count = revision.resident_event_count.wrapping_add(1);
            }
            Poison::LastDayIndex => revision.last_day_index = Some(usize::MAX),
            Poison::LastIntervalIndex => revision.last_interval_index = Some(usize::MAX),
            Poison::LastSupport => {
                revision.last_support =
                    TimeSupport::new(ModelTimeNs::new(1), ModelTimeNs::new(2)).ok();
            }
            Poison::LastParentTransaction => {
                revision.last_parent_transaction_id = Some(ParentTransactionId::from_digest(
                    Digest32::from_bytes([201; 32]),
                ));
            }
            Poison::LastAcceptedSlab => {
                revision.last_accepted_slab_sha256 = Some(Digest32::from_bytes([202; 32]));
            }
            Poison::TraversedEndingOwner => {
                revision.traversed_ending_owner_sha256 = Some(Digest32::from_bytes([203; 32]));
            }
            Poison::PendingPreSupportEvent => {
                revision.pending_pre_support_event = Some((
                    ParentTransactionId::from_digest(Digest32::from_bytes([204; 32])),
                    ModelTimeNs::new(0),
                ));
            }
            Poison::EventIdCount => {
                revision.event_id_count = revision.event_id_count.wrapping_add(1)
            }
            Poison::CurrentEventOrdinal => revision.current_event_ordinal = Some(u32::MAX),
            Poison::SealedPrefixSupportCount => {
                revision.sealed_prefix_support_count =
                    revision.sealed_prefix_support_count.wrapping_add(1);
            }
            Poison::SealedPrefixEventCount => {
                revision.sealed_prefix_event_count =
                    revision.sealed_prefix_event_count.wrapping_add(1);
            }
            Poison::SealedPrefixAuthority => {
                revision.sealed_prefix_authority_sha256 = Digest32::from_bytes([205; 32]);
            }
            Poison::Wb14Checkpoint => {
                revision.wb14_checkpoint_sha256 = Some(Digest32::from_bytes([206; 32]));
            }
            Poison::LastWb14Replay => {
                revision.last_wb14_replay_sha256 = Some(Digest32::from_bytes([207; 32]));
            }
            Poison::LastSupportReceipt => {
                revision.last_support_receipt_sha256 = Some(Digest32::from_bytes([208; 32]));
            }
            Poison::AggregateTail => {
                revision.aggregate_tail_sha256 = Digest32::from_bytes([209; 32]);
            }
            Poison::SupportPayloadIdentity => {
                self.support_receipt_sha256 = Digest32::from_bytes([210; 32]);
            }
        }
        self
    }
}

impl AcceptedPublicationHistoryV1 {
    pub(super) fn install_validated_support(
        &mut self,
        mut support: Stage3AcceptedPublicationSupportV1,
        trusted_capability: bool,
    ) -> Result<(), DirectV11RealConsumerError> {
        let telemetry_started = (trusted_capability
            && crate::snow_stage3_v11_attachment::adaptive_parent_telemetry_enabled_v1())
        .then(std::time::Instant::now);
        let next_tail = self.inner.tail_authority.accept_support(&support)?;
        #[cfg(test)]
        if trusted_capability {
            record_chronology_owner_tail_join_v1();
        }
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
        let next_revision = self.inner.live_revision.successor(
            &next_tail,
            &self.inner.sealed_prefix_tail,
            self.inner.supports.len().checked_add(1).ok_or(
                DirectV11RealConsumerError::Identity(
                    "accepted publication resident support-count overflow",
                ),
            )?,
            self.inner.event_handoffs.len(),
            self.inner.wb14_replay_checkpoint.as_ref(),
            Some(&support),
        )?;
        #[cfg(test)]
        let forced_full_scan =
            FORCE_FULL_SCAN_ACCEPTED_PUBLICATION_HISTORY_V1.with(std::cell::Cell::get);
        #[cfg(not(test))]
        let forced_full_scan = false;
        let copied_on_write = forced_full_scan || Arc::strong_count(&self.inner) > 1;
        if forced_full_scan {
            let mut candidate = self.clone();
            let inner = candidate.make_mut();
            inner.supports.push(Arc::new(support));
            inner.last_child_replay_materialized = Some(current);
            inner.tail_authority = next_tail;
            inner.live_revision = next_revision;
            candidate.validate_cached_tail_against_full_scan()?;
            *self = candidate;
        } else {
            let inner = self.make_mut();
            inner.supports.push(Arc::new(support));
            inner.last_child_replay_materialized = Some(current);
            inner.tail_authority = next_tail;
            inner.live_revision = next_revision;
        }
        if trusted_capability {
            if let Some(accepted) = self.inner.supports.last() {
                crate::direct_runtime::record_snow_stage3_v11_accepted_history_append_v1(
                    accepted.receipt_sha256,
                );
            }
        }
        #[cfg(test)]
        if trusted_capability {
            if let Some(accepted) = self.inner.supports.last() {
                crate::v9_real_consumer_shadow::
                    record_canonical_covered_successful_history_append_v1(
                        accepted.receipt_sha256,
                    );
            }
        }
        if let Some(started) = telemetry_started {
            crate::snow_stage3_v11_attachment::record_adaptive_parent_publication_append_v1(
                started.elapsed(),
                copied_on_write,
            );
        }
        #[cfg(test)]
        if trusted_capability {
            record_successful_append_v1();
        }
        Ok(())
    }
}

#[cfg(test)]
#[derive(Clone, Copy, Debug)]
pub(crate) enum AcceptedPublicationLiveRevisionPoisonV1 {
    ForeignIncarnation,
    Sequence,
    CumulativeSupportCount,
    CumulativeEventCount,
    ResidentSupportCount,
    ResidentEventCount,
    LastDayIndex,
    LastIntervalIndex,
    LastSupport,
    LastParentTransaction,
    LastAcceptedSlab,
    TraversedEndingOwner,
    PendingPreSupportEvent,
    EventIdCount,
    CurrentEventOrdinal,
    SealedPrefixSupportCount,
    SealedPrefixEventCount,
    SealedPrefixAuthority,
    Wb14Checkpoint,
    LastWb14Replay,
    LastSupportReceipt,
    AggregateTail,
    SupportPayloadIdentity,
}

#[cfg(test)]
#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub(crate) struct AcceptedPublicationSupportCapabilityAuditV1 {
    pub(crate) full_validation_attempt_count: u64,
    pub(crate) full_validation_success_count: u64,
    pub(crate) operand_seal_count: u64,
    pub(crate) receipt_seal_count: u64,
    pub(crate) capability_mint_count: u64,
    pub(crate) trusted_append_attempt_count: u64,
    pub(crate) live_revision_join_count: u64,
    pub(crate) chronology_owner_tail_join_count: u64,
    pub(crate) successful_append_count: u64,
    pub(crate) append_time_full_validation_count: u64,
    pub(crate) append_time_operand_reconstruction_count: u64,
    pub(crate) append_time_receipt_reconstruction_count: u64,
    pub(crate) append_time_serialization_count: u64,
    pub(crate) append_time_full_prefix_scan_count: u64,
    pub(crate) support_payload_clone_count: u64,
}

#[cfg(test)]
std::thread_local! {
    static ACCEPTED_PUBLICATION_SUPPORT_CAPABILITY_AUDIT_V1:
        std::cell::RefCell<Option<AcceptedPublicationSupportCapabilityAuditV1>> =
            const { std::cell::RefCell::new(None) };
}

#[cfg(test)]
std::thread_local! {
    static TRUSTED_ACCEPTED_PUBLICATION_APPEND_DEPTH_V1: std::cell::Cell<u32> =
        const { std::cell::Cell::new(0) };
}

#[cfg(test)]
pub(super) struct TrustedAcceptedPublicationAppendAuditGuardV1;

#[cfg(test)]
impl Drop for TrustedAcceptedPublicationAppendAuditGuardV1 {
    fn drop(&mut self) {
        TRUSTED_ACCEPTED_PUBLICATION_APPEND_DEPTH_V1.with(|depth| {
            depth.set(
                depth
                    .get()
                    .checked_sub(1)
                    .expect("trusted append audit scope underflow"),
            );
        });
    }
}

#[cfg(test)]
pub(super) fn enter_trusted_accepted_publication_append_audit_scope_v1()
-> TrustedAcceptedPublicationAppendAuditGuardV1 {
    TRUSTED_ACCEPTED_PUBLICATION_APPEND_DEPTH_V1.with(|depth| {
        depth.set(
            depth
                .get()
                .checked_add(1)
                .expect("trusted append audit scope overflow"),
        );
    });
    TrustedAcceptedPublicationAppendAuditGuardV1
}

#[cfg(test)]
fn inside_trusted_accepted_publication_append_v1() -> bool {
    TRUSTED_ACCEPTED_PUBLICATION_APPEND_DEPTH_V1.with(|depth| depth.get() != 0)
}

#[cfg(test)]
fn increment(field: fn(&mut AcceptedPublicationSupportCapabilityAuditV1) -> &mut u64) {
    ACCEPTED_PUBLICATION_SUPPORT_CAPABILITY_AUDIT_V1.with(|slot| {
        let mut slot = slot.borrow_mut();
        if let Some(audit) = slot.as_mut() {
            let counter = field(audit);
            *counter = counter.checked_add(1).expect("test audit counter overflow");
        }
    });
}

#[cfg(test)]
pub(crate) fn begin_accepted_publication_support_capability_audit_v1() {
    ACCEPTED_PUBLICATION_SUPPORT_CAPABILITY_AUDIT_V1.with(|slot| {
        *slot.borrow_mut() = Some(AcceptedPublicationSupportCapabilityAuditV1::default());
    });
}

#[cfg(test)]
pub(crate) fn take_accepted_publication_support_capability_audit_v1()
-> AcceptedPublicationSupportCapabilityAuditV1 {
    ACCEPTED_PUBLICATION_SUPPORT_CAPABILITY_AUDIT_V1
        .with(|slot| slot.borrow_mut().take().unwrap_or_default())
}

#[cfg(test)]
pub(super) fn record_full_validation_attempt_v1() {
    increment(|audit| &mut audit.full_validation_attempt_count);
}

#[cfg(test)]
pub(super) fn record_full_validation_success_v1() {
    increment(|audit| &mut audit.full_validation_success_count);
}

#[cfg(test)]
pub(super) fn record_operand_seal_v1() {
    increment(|audit| &mut audit.operand_seal_count);
}

#[cfg(test)]
pub(super) fn record_receipt_seal_v1() {
    increment(|audit| &mut audit.receipt_seal_count);
}

#[cfg(test)]
fn record_capability_mint_v1() {
    increment(|audit| &mut audit.capability_mint_count);
}

#[cfg(test)]
pub(super) fn record_trusted_append_attempt_v1() {
    increment(|audit| &mut audit.trusted_append_attempt_count);
}

#[cfg(test)]
pub(super) fn record_live_revision_join_v1() {
    increment(|audit| &mut audit.live_revision_join_count);
}

#[cfg(test)]
pub(super) fn record_chronology_owner_tail_join_v1() {
    increment(|audit| &mut audit.chronology_owner_tail_join_count);
}

#[cfg(test)]
pub(super) fn record_successful_append_v1() {
    increment(|audit| &mut audit.successful_append_count);
}

#[cfg(test)]
pub(super) fn record_append_time_full_validation_v1() {
    if inside_trusted_accepted_publication_append_v1() {
        increment(|audit| &mut audit.append_time_full_validation_count);
    }
}

#[cfg(test)]
pub(super) fn record_append_time_operand_reconstruction_v1() {
    if inside_trusted_accepted_publication_append_v1() {
        increment(|audit| &mut audit.append_time_operand_reconstruction_count);
    }
}

#[cfg(test)]
pub(super) fn record_append_time_receipt_reconstruction_v1() {
    if inside_trusted_accepted_publication_append_v1() {
        increment(|audit| &mut audit.append_time_receipt_reconstruction_count);
    }
}

#[cfg(test)]
pub(super) fn record_append_time_serialization_v1() {
    if inside_trusted_accepted_publication_append_v1() {
        increment(|audit| &mut audit.append_time_serialization_count);
    }
}

#[cfg(test)]
pub(super) fn record_append_time_full_prefix_scan_v1() {
    if inside_trusted_accepted_publication_append_v1() {
        increment(|audit| &mut audit.append_time_full_prefix_scan_count);
    }
}

#[cfg(test)]
pub(super) fn record_support_payload_clone_v1() {
    if inside_trusted_accepted_publication_append_v1() {
        increment(|audit| &mut audit.support_payload_clone_count);
    }
}
