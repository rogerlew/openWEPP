//! Prepared-day V2 checkpoint transaction.

use crate::{
    AcceptedIntervalCount, CompleteCommittedOwnerStateV2, DirectGsiDailyReceiptRestartV1,
    DirectGsiOwnerStateRestartV1, DirectV10CheckpointPhaseV2,
    DirectV10ContinuationTemplateRestartV1, DirectV10NativeOwnerHostV2,
    DirectV10RealConsumerCheckpointV2, ExpectedRestartStaticContextV2, InProgressIntervalIndex,
    RestartAdmissionFailureV2, ScientificOwnerStateSetV2, Sha256Hex,
    SnowFreeHalfHourDayReceiptRestartV1, SnowFreeHalfHourProviderCursorRestartV1,
    SoilThermalOwnerStateRestartV2, SoilThermalRestartV2Error, WireDayIndex, admit_checkpoint_v2,
    to_canonical_bytes,
};
use openwepp_hillslope_orchestrator::v9_real_consumer_shadow::{
    SoilThermalAcceptedCandidateV2, SoilThermalOrchestratorSealsV2,
};
use openwepp_land_surface_energy::{LandSurfaceEnergyConfiguration, SoilThermalOwnerEnvelopeV2};
use thiserror::Error;

/// Already validated non-scientific prepared-day custody supplied by the V1
/// forcing/GSI authority. It is intentionally reused without schema changes.
#[derive(Clone)]
pub struct PreparedDayWireOwnersV2 {
    pub accepted_gsi_daily_receipt: DirectGsiDailyReceiptRestartV1,
    pub staged_gsi_ending_state: DirectGsiOwnerStateRestartV1,
    pub ending_provider_cursor: SnowFreeHalfHourProviderCursorRestartV1,
    pub validated_forcing_day_receipts: Vec<SnowFreeHalfHourDayReceiptRestartV1>,
    pub continuation_template: DirectV10ContinuationTemplateRestartV1,
}

#[derive(Debug, Error)]
pub enum RestartTransactionV2Error {
    #[error(transparent)]
    Admission(#[from] RestartAdmissionFailureV2),
    #[error(transparent)]
    SoilThermal(#[from] SoilThermalRestartV2Error),
    #[error("phase: {0}")]
    Phase(&'static str),
    #[error("canonical_checkpoint")]
    CanonicalCheckpoint,
}

/// Orchestrator-independent prepared transaction. Each accepted interval is
/// supplied as one fully sealed scientific successor and installed only after
/// V2 admission validation succeeds.
pub struct DirectV10PreparedDayTransactionV2 {
    committed: CompleteCommittedOwnerStateV2,
    staged: ScientificOwnerStateSetV2,
    native_soil_thermal: SoilThermalOwnerEnvelopeV2,
    prepared: PreparedDayWireOwnersV2,
    parent_v1_checkpoint_sha256: Sha256Hex,
    run: Sha256Hex,
    topology: Sha256Hex,
    day: u64,
    next: u8,
    accepted_at_beginning: u64,
}

impl DirectV10PreparedDayTransactionV2 {
    #[allow(clippy::too_many_arguments)]
    fn prepare(
        committed: CompleteCommittedOwnerStateV2,
        prepared: PreparedDayWireOwnersV2,
        parent_v1_checkpoint_sha256: Sha256Hex,
        run: Sha256Hex,
        topology: Sha256Hex,
        day: u64,
        accepted_at_beginning: u64,
    ) -> Result<Self, RestartTransactionV2Error> {
        if day.checked_mul(48) != Some(accepted_at_beginning)
            || parent_v1_checkpoint_sha256
                .as_str()
                .chars()
                .all(|value| value == '0')
        {
            return Err(RestartTransactionV2Error::Phase("beginning identity"));
        }
        let native_soil_thermal = committed
            .scientific
            .soil_thermal_v2
            .decode_native()?
            .owner_envelope;
        Ok(Self {
            staged: committed.scientific.clone(),
            native_soil_thermal,
            committed,
            prepared,
            parent_v1_checkpoint_sha256,
            run,
            topology,
            day,
            next: 0,
            accepted_at_beginning,
        })
    }

    #[allow(clippy::too_many_arguments)]
    pub fn prepare_from_native_host(
        host: &DirectV10NativeOwnerHostV2,
        prepared: PreparedDayWireOwnersV2,
        parent_v1_checkpoint_sha256: Sha256Hex,
        run: Sha256Hex,
        topology: Sha256Hex,
        day: u64,
        accepted_at_beginning: u64,
    ) -> Result<Self, RestartTransactionV2Error> {
        let transaction = Self::prepare(
            host.committed().clone(),
            prepared,
            parent_v1_checkpoint_sha256,
            run,
            topology,
            day,
            accepted_at_beginning,
        )?;
        if transaction.native_soil_thermal != *host.soil_thermal() {
            return Err(RestartTransactionV2Error::Phase("native host custody"));
        }
        Ok(transaction)
    }

    /// Validate one successor in isolation, then atomically replace staged state.
    fn accept_interval_successor(
        &mut self,
        successor: ScientificOwnerStateSetV2,
        context: &ExpectedRestartStaticContextV2<'_>,
    ) -> Result<(), RestartTransactionV2Error> {
        if self.next >= 48 {
            return Err(RestartTransactionV2Error::Phase("day complete"));
        }
        successor.validate_soil_owner(
            context.soil_thermal_owner_id,
            context.lse_configuration,
            context.native_seal_authority,
        )?;
        let current = self.staged.soil_thermal_v2.decode_native()?;
        let next = successor.soil_thermal_v2.decode_native()?;
        if current.owner_envelope != self.native_soil_thermal
            || current.owner_envelope.state.owner_id != next.owner_envelope.state.owner_id
            || current.owner_envelope.state.configuration_sha256
                != next.owner_envelope.state.configuration_sha256
            || current.owner_envelope.parent_v1_state_sha256
                != next.owner_envelope.parent_v1_state_sha256
            || next.owner_envelope.expected_predecessor_transaction_id
                != current.owner_envelope.state.last_accepted_transaction_id
        {
            return Err(RestartTransactionV2Error::Phase(
                "soil owner predecessor chain",
            ));
        }
        self.native_soil_thermal = next.owner_envelope;
        self.staged = successor;
        self.next = self
            .next
            .checked_add(1)
            .ok_or(RestartTransactionV2Error::Phase("interval overflow"))?;
        Ok(())
    }

    /// Consume one native accepted candidate and derive the persisted successor
    /// inside the transaction. Callers cannot substitute an independently
    /// fabricated soil-owner DTO at this boundary.
    pub fn accept_native_soil_candidate(
        &mut self,
        beginning: SoilThermalOwnerEnvelopeV2,
        candidate: SoilThermalAcceptedCandidateV2,
        seals: SoilThermalOrchestratorSealsV2,
        configuration: &LandSurfaceEnergyConfiguration,
        context: &ExpectedRestartStaticContextV2<'_>,
    ) -> Result<(), RestartTransactionV2Error> {
        if beginning != self.native_soil_thermal {
            return Err(RestartTransactionV2Error::Phase("native beginning custody"));
        }
        let parent_v1 = self.staged.soil_thermal_v2.parent_v1.clone();
        let successor_soil = SoilThermalOwnerStateRestartV2::from_accepted_candidate(
            parent_v1,
            beginning,
            candidate,
            seals,
            configuration,
        )?;
        let mut successor = self.staged.clone();
        successor.soil_thermal_v2 = successor_soil;
        self.accept_interval_successor(successor, context)
    }

    #[must_use]
    pub const fn native_soil_thermal(&self) -> &SoilThermalOwnerEnvelopeV2 {
        &self.native_soil_thermal
    }

    pub fn checkpoint(&self) -> Result<Vec<u8>, RestartTransactionV2Error> {
        let accepted = self
            .accepted_at_beginning
            .checked_add(u64::from(self.next))
            .ok_or(RestartTransactionV2Error::Phase("accepted count overflow"))?;
        let phase = if self.next == 0 {
            DirectV10CheckpointPhaseV2::BetweenDays {
                next_day_index: WireDayIndex(self.day),
                accepted_interval_count: AcceptedIntervalCount::try_new(accepted)
                    .map_err(|_| RestartTransactionV2Error::Phase("accepted count"))?,
                committed: self.committed.clone(),
            }
        } else {
            DirectV10CheckpointPhaseV2::InProgressDay {
                day_index: WireDayIndex(self.day),
                next_interval_index: InProgressIntervalIndex::try_new(self.next)
                    .map_err(|_| RestartTransactionV2Error::Phase("interval"))?,
                accepted_interval_count: AcceptedIntervalCount::try_new(accepted)
                    .map_err(|_| RestartTransactionV2Error::Phase("accepted count"))?,
                committed_day_beginning: self.committed.clone(),
                staged_scientific: self.staged.clone(),
                accepted_gsi_daily_receipt: self.prepared.accepted_gsi_daily_receipt.clone(),
                staged_gsi_ending_state: self.prepared.staged_gsi_ending_state.clone(),
                ending_provider_cursor: self.prepared.ending_provider_cursor.clone(),
                validated_forcing_day_receipts: self
                    .prepared
                    .validated_forcing_day_receipts
                    .clone(),
                continuation_template: self.prepared.continuation_template.clone(),
            }
        };
        let mut checkpoint = DirectV10RealConsumerCheckpointV2 {
            schema: crate::DIRECT_V10_CHECKPOINT_V2_SCHEMA.to_owned(),
            version: 2,
            parent_v1_checkpoint_sha256: self.parent_v1_checkpoint_sha256.clone(),
            run_identity_sha256: self.run.clone(),
            topology_sha256: self.topology.clone(),
            phase,
            payload_sha256: Sha256Hex::try_new("0".repeat(64))
                .map_err(|_| RestartTransactionV2Error::CanonicalCheckpoint)?,
        };
        checkpoint.seal()?;
        to_canonical_bytes(&checkpoint).map_err(|_| RestartTransactionV2Error::CanonicalCheckpoint)
    }

    pub fn restore(
        bytes: &[u8],
        context: &ExpectedRestartStaticContextV2<'_>,
    ) -> Result<crate::IsolatedRestoredCheckpointV2, RestartTransactionV2Error> {
        admit_checkpoint_v2(bytes, context).map_err(Into::into)
    }

    #[must_use]
    pub fn abort(self) -> CompleteCommittedOwnerStateV2 {
        self.committed
    }
}
