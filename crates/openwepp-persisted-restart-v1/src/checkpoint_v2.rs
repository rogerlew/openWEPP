//! Direct V10 persisted checkpoint V2 schema and admission.

use openwepp_kernel_contract::ResourceOwnerId;
use openwepp_land_surface_energy::{LandSurfaceEnergyConfiguration, SoilThermalOwnerEnvelopeV2};
use serde::{Deserialize, Serialize};
use thiserror::Error;

use crate::{
    AcceptedIntervalCount, CompleteCommittedOwnerStateV2, DirectGsiDailyReceiptRestartV1,
    DirectGsiOwnerStateRestartV1, DirectV10ContinuationTemplateRestartV1, InProgressIntervalIndex,
    ScientificOwnerStateSetV2, Sha256Hex, SnowFreeHalfHourDayReceiptRestartV1,
    SnowFreeHalfHourProviderCursorRestartV1, SoilThermalNativeSealAuthorityV2,
    SoilThermalRestartV2Error, WireDayIndex, canonical_sha256, from_canonical_bytes,
};

pub const DIRECT_V10_CHECKPOINT_V2_SCHEMA: &str = "OPENWEPP_DIRECT_V10_REAL_CONSUMER_CHECKPOINT_V2";

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(tag = "kind", rename_all = "snake_case", deny_unknown_fields)]
#[allow(clippy::large_enum_variant)]
pub enum DirectV10CheckpointPhaseV2 {
    BetweenDays {
        next_day_index: WireDayIndex,
        accepted_interval_count: AcceptedIntervalCount,
        committed: CompleteCommittedOwnerStateV2,
    },
    InProgressDay {
        day_index: WireDayIndex,
        next_interval_index: InProgressIntervalIndex,
        accepted_interval_count: AcceptedIntervalCount,
        committed_day_beginning: CompleteCommittedOwnerStateV2,
        staged_scientific: ScientificOwnerStateSetV2,
        accepted_gsi_daily_receipt: DirectGsiDailyReceiptRestartV1,
        staged_gsi_ending_state: DirectGsiOwnerStateRestartV1,
        ending_provider_cursor: SnowFreeHalfHourProviderCursorRestartV1,
        validated_forcing_day_receipts: Vec<SnowFreeHalfHourDayReceiptRestartV1>,
        continuation_template: DirectV10ContinuationTemplateRestartV1,
    },
}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct DirectV10RealConsumerCheckpointV2 {
    pub schema: String,
    pub version: u16,
    pub parent_v1_checkpoint_sha256: Sha256Hex,
    pub run_identity_sha256: Sha256Hex,
    pub topology_sha256: Sha256Hex,
    pub phase: DirectV10CheckpointPhaseV2,
    pub payload_sha256: Sha256Hex,
}

#[derive(Serialize)]
struct CheckpointDigestBody<'a> {
    schema: &'a str,
    version: u16,
    parent_v1_checkpoint_sha256: &'a Sha256Hex,
    run_identity_sha256: &'a Sha256Hex,
    topology_sha256: &'a Sha256Hex,
    phase: &'a DirectV10CheckpointPhaseV2,
}

impl DirectV10RealConsumerCheckpointV2 {
    pub fn compute_digest(&self) -> Result<Sha256Hex, RestartAdmissionFailureV2> {
        Sha256Hex::try_new(
            canonical_sha256(&CheckpointDigestBody {
                schema: &self.schema,
                version: self.version,
                parent_v1_checkpoint_sha256: &self.parent_v1_checkpoint_sha256,
                run_identity_sha256: &self.run_identity_sha256,
                topology_sha256: &self.topology_sha256,
                phase: &self.phase,
            })
            .map_err(|_| RestartAdmissionFailureV2::PayloadDigest)?,
        )
        .map_err(|_| RestartAdmissionFailureV2::PayloadDigest)
    }

    pub fn seal(&mut self) -> Result<(), RestartAdmissionFailureV2> {
        self.payload_sha256 = self.compute_digest()?;
        Ok(())
    }

    #[must_use]
    pub fn abort_to_day_beginning(&self) -> CompleteCommittedOwnerStateV2 {
        match &self.phase {
            DirectV10CheckpointPhaseV2::BetweenDays { committed, .. } => committed.clone(),
            DirectV10CheckpointPhaseV2::InProgressDay {
                committed_day_beginning,
                ..
            } => committed_day_beginning.clone(),
        }
    }
}

pub struct ExpectedRestartStaticContextV2<'a> {
    pub run_identity_sha256: &'a Sha256Hex,
    pub topology_sha256: &'a Sha256Hex,
    pub soil_thermal_owner_id: &'a ResourceOwnerId,
    pub lse_configuration: &'a LandSurfaceEnergyConfiguration,
    pub native_seal_authority: &'a dyn SoilThermalNativeSealAuthorityV2,
}

#[derive(Clone, Debug, PartialEq)]
pub struct RestoredScientificOwnerStateSetV2 {
    pub persisted: ScientificOwnerStateSetV2,
    pub soil_thermal: SoilThermalOwnerEnvelopeV2,
}

#[derive(Clone, Debug, PartialEq)]
pub struct RestoredCompleteCommittedOwnerStateV2 {
    pub persisted: CompleteCommittedOwnerStateV2,
    pub scientific: RestoredScientificOwnerStateSetV2,
}

#[derive(Clone, Debug, PartialEq)]
pub enum IsolatedRestoredCheckpointV2 {
    BetweenDays {
        next_day_index: u64,
        accepted_interval_count: u64,
        committed: RestoredCompleteCommittedOwnerStateV2,
    },
    InProgressDay {
        day_index: u64,
        next_interval_index: u8,
        accepted_interval_count: u64,
        committed_day_beginning: RestoredCompleteCommittedOwnerStateV2,
        staged_scientific: RestoredScientificOwnerStateSetV2,
        accepted_gsi_daily_receipt: DirectGsiDailyReceiptRestartV1,
        staged_gsi_ending_state: DirectGsiOwnerStateRestartV1,
        ending_provider_cursor: SnowFreeHalfHourProviderCursorRestartV1,
        validated_forcing_day_receipts: Vec<SnowFreeHalfHourDayReceiptRestartV1>,
        continuation_template: DirectV10ContinuationTemplateRestartV1,
    },
}

#[derive(Clone, Copy, Debug, Error, Eq, PartialEq)]
pub enum RestartAdmissionFailureV2 {
    #[error("schema")]
    Schema,
    #[error("unsupported_version")]
    UnsupportedVersion,
    #[error("noncanonical_bytes")]
    NoncanonicalBytes,
    #[error("payload_digest")]
    PayloadDigest,
    #[error("run_identity")]
    RunIdentity,
    #[error("topology_identity")]
    TopologyIdentity,
    #[error("scheduler_position")]
    SchedulerPosition,
    #[error("parent_v1_checkpoint")]
    ParentV1Checkpoint,
    #[error("soil_thermal: {0}")]
    SoilThermal(#[from] SoilThermalRestartV2Error),
}

pub fn admit_checkpoint_v2(
    bytes: &[u8],
    context: &ExpectedRestartStaticContextV2<'_>,
) -> Result<IsolatedRestoredCheckpointV2, RestartAdmissionFailureV2> {
    let checkpoint: DirectV10RealConsumerCheckpointV2 =
        from_canonical_bytes(bytes).map_err(|_| RestartAdmissionFailureV2::NoncanonicalBytes)?;
    if checkpoint.compute_digest()? != checkpoint.payload_sha256 {
        return Err(RestartAdmissionFailureV2::PayloadDigest);
    }
    if checkpoint.schema != DIRECT_V10_CHECKPOINT_V2_SCHEMA {
        return Err(RestartAdmissionFailureV2::Schema);
    }
    if checkpoint.version != 2 {
        return Err(RestartAdmissionFailureV2::UnsupportedVersion);
    }
    if checkpoint
        .parent_v1_checkpoint_sha256
        .as_str()
        .chars()
        .all(|value| value == '0')
    {
        return Err(RestartAdmissionFailureV2::ParentV1Checkpoint);
    }
    if &checkpoint.run_identity_sha256 != context.run_identity_sha256 {
        return Err(RestartAdmissionFailureV2::RunIdentity);
    }
    if &checkpoint.topology_sha256 != context.topology_sha256 {
        return Err(RestartAdmissionFailureV2::TopologyIdentity);
    }
    match checkpoint.phase {
        DirectV10CheckpointPhaseV2::BetweenDays {
            next_day_index,
            accepted_interval_count,
            committed,
        } => {
            let soil_thermal = committed.scientific.validate_soil_owner(
                context.soil_thermal_owner_id,
                context.lse_configuration,
                context.native_seal_authority,
            )?;
            if next_day_index
                .0
                .checked_mul(48)
                .is_none_or(|expected| expected != accepted_interval_count.get())
            {
                return Err(RestartAdmissionFailureV2::SchedulerPosition);
            }
            Ok(IsolatedRestoredCheckpointV2::BetweenDays {
                next_day_index: next_day_index.0,
                accepted_interval_count: accepted_interval_count.get(),
                committed: RestoredCompleteCommittedOwnerStateV2 {
                    scientific: RestoredScientificOwnerStateSetV2 {
                        persisted: committed.scientific.clone(),
                        soil_thermal,
                    },
                    persisted: committed,
                },
            })
        }
        DirectV10CheckpointPhaseV2::InProgressDay {
            day_index,
            next_interval_index,
            accepted_interval_count,
            committed_day_beginning,
            staged_scientific,
            accepted_gsi_daily_receipt,
            staged_gsi_ending_state,
            ending_provider_cursor,
            validated_forcing_day_receipts,
            continuation_template,
        } => {
            let beginning = committed_day_beginning.scientific.validate_soil_owner(
                context.soil_thermal_owner_id,
                context.lse_configuration,
                context.native_seal_authority,
            )?;
            let staged = staged_scientific.validate_soil_owner(
                context.soil_thermal_owner_id,
                context.lse_configuration,
                context.native_seal_authority,
            )?;
            if beginning.parent_v1_state_sha256 != staged.parent_v1_state_sha256
                || beginning.state.owner_id != staged.state.owner_id
                || beginning.state.configuration_sha256 != staged.state.configuration_sha256
                || u64::from(next_interval_index.get()).checked_add(
                    day_index
                        .0
                        .checked_mul(48)
                        .ok_or(RestartAdmissionFailureV2::SchedulerPosition)?,
                ) != Some(accepted_interval_count.get())
            {
                return Err(RestartAdmissionFailureV2::SchedulerPosition);
            }
            Ok(IsolatedRestoredCheckpointV2::InProgressDay {
                day_index: day_index.0,
                next_interval_index: next_interval_index.get(),
                accepted_interval_count: accepted_interval_count.get(),
                committed_day_beginning: RestoredCompleteCommittedOwnerStateV2 {
                    scientific: RestoredScientificOwnerStateSetV2 {
                        persisted: committed_day_beginning.scientific.clone(),
                        soil_thermal: beginning,
                    },
                    persisted: committed_day_beginning,
                },
                staged_scientific: RestoredScientificOwnerStateSetV2 {
                    persisted: staged_scientific,
                    soil_thermal: staged,
                },
                accepted_gsi_daily_receipt,
                staged_gsi_ending_state,
                ending_provider_cursor,
                validated_forcing_day_receipts,
                continuation_template,
            })
        }
    }
}
