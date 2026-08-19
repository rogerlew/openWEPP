use crate::{
    AcceptedIntervalCount, BiogeochemistryStateRestartV1, DirectGsiDailyReceiptRestartV1,
    DirectGsiOwnerConfigurationRestartV1, DirectGsiOwnerStateRestartV1, DirectHydrologyRestartV1,
    DirectSurfaceLiquidConfigurationRestartV1, ExpectedDirectHydrologyRestartContext,
    InProgressIntervalIndex, LseV2StateRestartV1, Sha256Hex, SnowFreeHalfHourDayReceiptRestartV1,
    SnowFreeHalfHourProviderCursorRestartV1, SnowFreeHalfHourStaticConfigurationRestartV1,
    SoilThermalStateRestartV1, VegetationV10StateRestartV1, WireDayIndex, canonical_sha256,
    from_canonical_bytes,
};
use openwepp_biogeochemistry::BiogeochemistryState;
use openwepp_hillslope_orchestrator::runtime_inputs::{
    DirectGsiOwnerConfigurationV1, DirectGsiOwnerStateV1, SnowFreeHalfHourProviderCursor,
    SnowFreeHalfHourStaticConfiguration,
};
use openwepp_hillslope_orchestrator::{
    DirectDayConstructorInputs, DirectPhasePlan, DirectRunFrame, DirectSurfaceLiquidConfiguration,
};
use openwepp_land_surface_energy::{
    LandSurfaceEnergyConfiguration, LandSurfaceEnergyV2State, SoilThermalSnapshot,
};
use openwepp_vegetation::{V10CoupledOwnedState, VegetationConfiguration};
use serde::{Deserialize, Serialize};
use thiserror::Error;

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct ScientificOwnerStateSetV1 {
    pub vegetation_v10: VegetationV10StateRestartV1,
    pub lse_v2: LseV2StateRestartV1,
    pub direct_hydrology: DirectHydrologyRestartV1,
    pub soil_thermal: SoilThermalStateRestartV1,
    pub biogeochemistry: BiogeochemistryStateRestartV1,
}
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct CompleteCommittedOwnerStateV1 {
    pub gsi_configuration: DirectGsiOwnerConfigurationRestartV1,
    pub gsi_state: DirectGsiOwnerStateRestartV1,
    pub static_forcing_configuration: SnowFreeHalfHourStaticConfigurationRestartV1,
    pub provider_cursor: SnowFreeHalfHourProviderCursorRestartV1,
    pub surface_liquid_configuration: DirectSurfaceLiquidConfigurationRestartV1,
    pub scientific: ScientificOwnerStateSetV1,
}
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(tag = "kind", rename_all = "snake_case", deny_unknown_fields)]
// The wire variants remain structurally explicit; Rust heap indirection is not
// checkpoint authority and must not leak into this evidence DTO.
#[allow(clippy::large_enum_variant)]
pub enum DirectV10CheckpointPhaseV1 {
    BetweenDays {
        next_day_index: WireDayIndex,
        accepted_interval_count: AcceptedIntervalCount,
        committed: CompleteCommittedOwnerStateV1,
    },
    InProgressDay {
        day_index: WireDayIndex,
        next_interval_index: InProgressIntervalIndex,
        accepted_interval_count: AcceptedIntervalCount,
        committed_day_beginning: CompleteCommittedOwnerStateV1,
        staged_scientific: ScientificOwnerStateSetV1,
        accepted_gsi_daily_receipt: DirectGsiDailyReceiptRestartV1,
        staged_gsi_ending_state: DirectGsiOwnerStateRestartV1,
        ending_provider_cursor: SnowFreeHalfHourProviderCursorRestartV1,
        validated_forcing_day_receipts: Vec<SnowFreeHalfHourDayReceiptRestartV1>,
    },
}
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct DirectV10RealConsumerCheckpointV1 {
    pub schema: String,
    pub version: u16,
    pub run_identity_sha256: Sha256Hex,
    pub topology_sha256: Sha256Hex,
    pub phase: DirectV10CheckpointPhaseV1,
    pub payload_sha256: Sha256Hex,
}
#[derive(Serialize)]
struct DigestInput<'a> {
    schema: &'a str,
    version: u16,
    run_identity_sha256: &'a Sha256Hex,
    topology_sha256: &'a Sha256Hex,
    phase: &'a DirectV10CheckpointPhaseV1,
}

#[derive(Debug, Error, Clone, Copy, PartialEq, Eq)]
pub enum RestartAdmissionFailureV1 {
    #[error("schema")]
    Schema,
    #[error("unsupported_version")]
    UnsupportedVersion,
    #[error("noncanonical_bytes")]
    NoncanonicalBytes,
    #[error("missing_field")]
    MissingField,
    #[error("extra_field")]
    ExtraField,
    #[error("reordered_field")]
    ReorderedField,
    #[error("duplicate_field")]
    DuplicateField,
    #[error("payload_digest")]
    PayloadDigest,
    #[error("run_identity")]
    RunIdentity,
    #[error("topology_identity")]
    TopologyIdentity,
    #[error("configuration_identity")]
    ConfigurationIdentity,
    #[error("owner_identity")]
    OwnerIdentity,
    #[error("owner_validation")]
    OwnerValidation,
    #[error("transaction_lineage")]
    TransactionLineage,
    #[error("scheduler_position")]
    SchedulerPosition,
    #[error("provider_cursor")]
    ProviderCursor,
    #[error("gsi_receipt")]
    GsiReceipt,
    #[error("heterogeneous_lane_gsi_receipt")]
    HeterogeneousLaneGsiReceipt,
    #[error("forcing_receipt_cardinality")]
    ForcingReceiptCardinality,
    #[error("forcing_receipt_order")]
    ForcingReceiptOrder,
    #[error("forcing_receipt_digest")]
    ForcingReceiptDigest,
    #[error("surface_liquid_configuration")]
    SurfaceLiquidConfiguration,
    #[error("v10_v9_projection")]
    V10V9Projection,
    #[error("lse_v2_v1_projection")]
    LseV2V1Projection,
    #[error("unsupported_laned_active")]
    UnsupportedLanedActive,
    #[error("canonical_order")]
    CanonicalOrder,
    #[error("owner_omission")]
    OwnerOmission,
    #[error("child4_retained_liquid")]
    Child4RetainedLiquid,
    #[error("groundwater_posture")]
    GroundwaterPosture,
    #[error("groundwater_total_area")]
    GroundwaterTotalArea,
    #[error("erosion_publication")]
    ErosionPublication,
}

pub struct ExpectedRestartStaticContext<'a> {
    pub run_identity_sha256: &'a Sha256Hex,
    pub topology_sha256: &'a Sha256Hex,
    pub vegetation_configuration: &'a VegetationConfiguration,
    pub vegetation_owner_id: &'a openwepp_kernel_contract::ResourceOwnerId,
    pub soil_thermal_owner_id: &'a openwepp_kernel_contract::ResourceOwnerId,
    pub soil_thermal_configuration_sha256: &'a openwepp_land_surface_energy::Sha256Digest,
    pub lse_configuration: &'a LandSurfaceEnergyConfiguration,
    pub surface_liquid_configuration: &'a DirectSurfaceLiquidConfiguration,
    pub gsi_configuration: &'a DirectGsiOwnerConfigurationV1,
    pub forcing_static_configuration: &'a SnowFreeHalfHourStaticConfiguration,
    pub phase_plan: &'a DirectPhasePlan,
    pub phase_plan_sha256: &'a Sha256Hex,
    pub day_inputs: &'a [Vec<DirectDayConstructorInputs>],
    pub day_input_digests: &'a [Sha256Hex],
}
pub struct RestoredScientificOwnerStateSetV1 {
    pub vegetation_v10: V10CoupledOwnedState,
    pub lse_v2: LandSurfaceEnergyV2State,
    pub direct_hydrology: DirectRunFrame,
    pub soil_thermal: SoilThermalSnapshot,
    pub biogeochemistry: BiogeochemistryState,
}
pub struct RestoredCompleteCommittedOwnerStateV1 {
    pub gsi_state: DirectGsiOwnerStateV1,
    pub provider_cursor: SnowFreeHalfHourProviderCursor,
    pub scientific: RestoredScientificOwnerStateSetV1,
}
// Admission returns complete isolated owners by value. This is evidence-path
// construction, not a latency-sensitive runtime API.
#[allow(clippy::large_enum_variant)]
pub enum IsolatedRestoredCheckpointV1 {
    BetweenDays {
        next_day_index: u64,
        committed: RestoredCompleteCommittedOwnerStateV1,
    },
    InProgressDay {
        day_index: u64,
        next_interval_index: u8,
        accepted_interval_count: u64,
        committed_day_beginning: RestoredCompleteCommittedOwnerStateV1,
        staged_scientific: RestoredScientificOwnerStateSetV1,
        staged_gsi_ending_state: DirectGsiOwnerStateV1,
        accepted_gsi_daily_receipt: openwepp_hillslope_orchestrator::runtime_inputs::DirectGsiDailyReceiptV1,
        validated_forcing_day_receipts:
            Vec<openwepp_hillslope_orchestrator::runtime_inputs::SnowFreeHalfHourDayReceipt>,
        ending_provider_cursor: SnowFreeHalfHourProviderCursor,
    },
}

impl DirectV10RealConsumerCheckpointV1 {
    pub fn compute_digest(&self) -> Result<Sha256Hex, RestartAdmissionFailureV1> {
        Sha256Hex::try_new(
            canonical_sha256(&DigestInput {
                schema: &self.schema,
                version: self.version,
                run_identity_sha256: &self.run_identity_sha256,
                topology_sha256: &self.topology_sha256,
                phase: &self.phase,
            })
            .map_err(|_| RestartAdmissionFailureV1::PayloadDigest)?,
        )
        .map_err(|_| RestartAdmissionFailureV1::PayloadDigest)
    }
    pub fn seal(&mut self) -> Result<(), RestartAdmissionFailureV1> {
        self.payload_sha256 = self.compute_digest()?;
        Ok(())
    }
    pub fn abort_to_day_beginning(&self) -> CompleteCommittedOwnerStateV1 {
        match &self.phase {
            DirectV10CheckpointPhaseV1::BetweenDays { committed, .. } => committed.clone(),
            DirectV10CheckpointPhaseV1::InProgressDay {
                committed_day_beginning,
                ..
            } => committed_day_beginning.clone(),
        }
    }
    pub fn abort_owner_store_to_day_beginning(
        &self,
        live: &mut CompleteCommittedOwnerStateV1,
    ) {
        *live = self.abort_to_day_beginning();
    }
}

#[cfg_attr(not(test), allow(dead_code))]
pub fn admit_checkpoint_into_owner_store_v1(
    bytes: &[u8],
    context: &ExpectedRestartStaticContext<'_>,
    live: &mut CompleteCommittedOwnerStateV1,
) -> Result<IsolatedRestoredCheckpointV1, RestartAdmissionFailureV1> {
    let admitted = admit_checkpoint_v1(bytes, context)?;
    let checkpoint: DirectV10RealConsumerCheckpointV1 =
        serde_json::from_slice(bytes).map_err(|_| RestartAdmissionFailureV1::Schema)?;
    let replacement = match checkpoint.phase {
        DirectV10CheckpointPhaseV1::BetweenDays { committed, .. } => committed,
        DirectV10CheckpointPhaseV1::InProgressDay {
            committed_day_beginning,
            staged_scientific,
            staged_gsi_ending_state,
            ending_provider_cursor,
            ..
        } => CompleteCommittedOwnerStateV1 {
            gsi_configuration: committed_day_beginning.gsi_configuration,
            static_forcing_configuration: committed_day_beginning.static_forcing_configuration,
            surface_liquid_configuration: committed_day_beginning.surface_liquid_configuration,
            gsi_state: staged_gsi_ending_state,
            provider_cursor: ending_provider_cursor,
            scientific: staged_scientific,
        },
    };
    *live = replacement;
    Ok(admitted)
}

pub fn admit_checkpoint_v1(
    bytes: &[u8],
    context: &ExpectedRestartStaticContext<'_>,
) -> Result<IsolatedRestoredCheckpointV1, RestartAdmissionFailureV1> {
    let checkpoint: DirectV10RealConsumerCheckpointV1 =
        from_canonical_bytes(bytes).map_err(|error| match error {
            crate::CanonicalJsonError::Parse(message) if message.contains("duplicate field") => {
                RestartAdmissionFailureV1::DuplicateField
            }
            crate::CanonicalJsonError::Typed(message) if message.contains("missing field") => {
                if [
                    "vegetation_v10",
                    "lse_v2",
                    "direct_hydrology",
                    "soil_thermal",
                    "biogeochemistry",
                    "gsi_configuration",
                    "gsi_state",
                    "static_forcing_configuration",
                    "provider_cursor",
                    "surface_liquid_configuration",
                ]
                .iter()
                .any(|field| message.contains(field))
                {
                    RestartAdmissionFailureV1::OwnerOmission
                } else {
                    RestartAdmissionFailureV1::MissingField
                }
            }
            crate::CanonicalJsonError::Typed(message) if message.contains("unknown field") => {
                RestartAdmissionFailureV1::ExtraField
            }
            crate::CanonicalJsonError::Typed(message) if message.contains("HexU128") => {
                RestartAdmissionFailureV1::TransactionLineage
            }
            crate::CanonicalJsonError::Typed(message)
                if message.contains("u64") || message.contains("day index") =>
            {
                RestartAdmissionFailureV1::SchedulerPosition
            }
            crate::CanonicalJsonError::NoncanonicalBytes
                if bytes.iter().any(u8::is_ascii_whitespace) =>
            {
                RestartAdmissionFailureV1::NoncanonicalBytes
            }
            crate::CanonicalJsonError::NoncanonicalBytes => {
                RestartAdmissionFailureV1::ReorderedField
            }
            _ => RestartAdmissionFailureV1::NoncanonicalBytes,
        })?;
    if checkpoint.schema != "OPENWEPP_DIRECT_V10_REAL_CONSUMER_CHECKPOINT_V1" {
        return Err(RestartAdmissionFailureV1::Schema);
    }
    if checkpoint.version != 1 {
        return Err(RestartAdmissionFailureV1::UnsupportedVersion);
    }
    if checkpoint.compute_digest()? != checkpoint.payload_sha256 {
        return Err(RestartAdmissionFailureV1::PayloadDigest);
    }
    if &checkpoint.run_identity_sha256 != context.run_identity_sha256 {
        return Err(RestartAdmissionFailureV1::RunIdentity);
    }
    if &checkpoint.topology_sha256 != context.topology_sha256 {
        return Err(RestartAdmissionFailureV1::TopologyIdentity);
    }
    match &checkpoint.phase {
        DirectV10CheckpointPhaseV1::BetweenDays {
            next_day_index,
            accepted_interval_count,
            committed,
        } => {
            if accepted_interval_count.get() % 48 != 0 {
                return Err(RestartAdmissionFailureV1::SchedulerPosition);
            }
            Ok(IsolatedRestoredCheckpointV1::BetweenDays {
                next_day_index: next_day_index.0,
                committed: restore_committed(committed, context, next_day_index.0)?,
            })
        }
        DirectV10CheckpointPhaseV1::InProgressDay {
            day_index,
            next_interval_index,
            accepted_interval_count,
            committed_day_beginning,
            staged_scientific,
            accepted_gsi_daily_receipt,
            staged_gsi_ending_state,
            ending_provider_cursor: ending_provider_cursor_dto,
            validated_forcing_day_receipts,
        } => {
            if accepted_interval_count.get() % 48 != u64::from(next_interval_index.get()) {
                return Err(RestartAdmissionFailureV1::SchedulerPosition);
            }
            let committed = restore_committed(committed_day_beginning, context, day_index.0)?;
            let receipt = accepted_gsi_daily_receipt
                .restore()
                .map_err(|_| RestartAdmissionFailureV1::GsiReceipt)?;
            if receipt.run_id != context.forcing_static_configuration.run_id
                || receipt.day_index != day_index.0
                || accepted_gsi_daily_receipt.beginning_state != committed_day_beginning.gsi_state
                || accepted_gsi_daily_receipt.ending_state != *staged_gsi_ending_state
                || receipt.configuration_sha256 != context.gsi_configuration.configuration_sha256
            {
                return Err(RestartAdmissionFailureV1::GsiReceipt);
            }
            let validated_forcing_day_receipts = validate_forcing(
                validated_forcing_day_receipts,
                context,
                day_index.0,
                accepted_gsi_daily_receipt.receipt_sha256.as_str(),
                &receipt.source_climate_sha256,
            )?;
            let outgoing_carry = validated_forcing_day_receipts
                .iter()
                .flat_map(|receipt| receipt.next_day_precipitation_carry.iter())
                .collect::<Vec<_>>();
            let ending_carry = ending_provider_cursor_dto
                .pending_carry
                .iter()
                .map(crate::SnowFreePrecipitationParcelRestartV1::restore)
                .collect::<Result<Vec<_>, _>>()
                .map_err(|_| RestartAdmissionFailureV1::ProviderCursor)?;
            if outgoing_carry != ending_carry.iter().collect::<Vec<_>>() {
                return Err(RestartAdmissionFailureV1::ProviderCursor);
            }
            for pending in &committed_day_beginning.provider_cursor.pending_carry {
                let native_pending = pending
                    .restore()
                    .map_err(|_| RestartAdmissionFailureV1::ProviderCursor)?;
                let found = validated_forcing_day_receipts
                    .iter()
                    .flat_map(|receipt| &receipt.intervals)
                    .flat_map(|interval| &interval.precipitation_parcels)
                    .filter(|parcel| *parcel == &native_pending)
                    .count();
                if found != 1 {
                    return Err(RestartAdmissionFailureV1::ProviderCursor);
                }
            }
            let ending_provider_cursor = ending_provider_cursor_dto
                .restore(
                    context.forcing_static_configuration,
                    usize::try_from(day_index.0 + 1)
                        .map_err(|_| RestartAdmissionFailureV1::ProviderCursor)?,
                )
                .map_err(|_| RestartAdmissionFailureV1::ProviderCursor)?;
            let expected_transaction = committed_day_beginning
                .scientific
                .vegetation_v10
                .last_transaction_id
                .to_u128()
                .checked_add(u128::from(next_interval_index.get()))
                .ok_or(RestartAdmissionFailureV1::TransactionLineage)?;
            require_lineage(staged_scientific, Some(expected_transaction))?;
            Ok(IsolatedRestoredCheckpointV1::InProgressDay {
                day_index: day_index.0,
                next_interval_index: next_interval_index.get(),
                accepted_interval_count: accepted_interval_count.get(),
                committed_day_beginning: committed,
                staged_scientific: restore_scientific(staged_scientific, context)?,
                staged_gsi_ending_state: staged_gsi_ending_state
                    .restore()
                    .map_err(|_| RestartAdmissionFailureV1::GsiReceipt)?,
                accepted_gsi_daily_receipt: receipt,
                validated_forcing_day_receipts,
                ending_provider_cursor,
            })
        }
    }
}

fn restore_committed(
    value: &CompleteCommittedOwnerStateV1,
    context: &ExpectedRestartStaticContext<'_>,
    next_day: u64,
) -> Result<RestoredCompleteCommittedOwnerStateV1, RestartAdmissionFailureV1> {
    if value.gsi_configuration.owner_id.trim().is_empty() {
        return Err(RestartAdmissionFailureV1::OwnerIdentity);
    }
    let gsi = value
        .gsi_configuration
        .restore()
        .map_err(|_| RestartAdmissionFailureV1::ConfigurationIdentity)?;
    if &gsi != context.gsi_configuration {
        return Err(RestartAdmissionFailureV1::ConfigurationIdentity);
    }
    let forcing = value
        .static_forcing_configuration
        .restore()
        .map_err(|_| RestartAdmissionFailureV1::ConfigurationIdentity)?;
    if &forcing != context.forcing_static_configuration {
        return Err(RestartAdmissionFailureV1::ConfigurationIdentity);
    }
    let surface = value
        .surface_liquid_configuration
        .restore()
        .map_err(|_| RestartAdmissionFailureV1::SurfaceLiquidConfiguration)?;
    if &surface != context.surface_liquid_configuration {
        return Err(RestartAdmissionFailureV1::SurfaceLiquidConfiguration);
    }
    require_lineage(&value.scientific, None)?;
    Ok(RestoredCompleteCommittedOwnerStateV1 {
        gsi_state: value
            .gsi_state
            .restore()
            .map_err(|_| RestartAdmissionFailureV1::OwnerValidation)?,
        provider_cursor: value
            .provider_cursor
            .restore(
                context.forcing_static_configuration,
                usize::try_from(next_day).map_err(|_| RestartAdmissionFailureV1::ProviderCursor)?,
            )
            .map_err(|_| RestartAdmissionFailureV1::ProviderCursor)?,
        scientific: restore_scientific(&value.scientific, context)?,
    })
}

fn restore_scientific(
    value: &ScientificOwnerStateSetV1,
    context: &ExpectedRestartStaticContext<'_>,
) -> Result<RestoredScientificOwnerStateSetV1, RestartAdmissionFailureV1> {
    if value.vegetation_v10.owner_id != context.vegetation_owner_id.as_str() {
        return Err(RestartAdmissionFailureV1::OwnerIdentity);
    }
    if value
        .direct_hydrology
        .surface_liquid_owned_state
        .as_deref()
        .is_some_and(|state| {
            state.configuration_sha256.as_str()
                != context.surface_liquid_configuration.configuration_sha256
        })
    {
        return Err(RestartAdmissionFailureV1::SurfaceLiquidConfiguration);
    }
    let hydrology_context = ExpectedDirectHydrologyRestartContext {
        phase_plan: context.phase_plan,
        phase_plan_sha256: context.phase_plan_sha256,
        day_inputs: context.day_inputs,
        day_input_digests: context.day_input_digests,
        surface_liquid_configuration: context.surface_liquid_configuration,
    };
    Ok(RestoredScientificOwnerStateSetV1 {
        vegetation_v10: value
            .vegetation_v10
            .restore(context.vegetation_configuration, context.vegetation_owner_id)
            .map_err(|_| RestartAdmissionFailureV1::V10V9Projection)?,
        lse_v2: value
            .lse_v2
            .restore(context.lse_configuration)
            .map_err(|_| RestartAdmissionFailureV1::LseV2V1Projection)?,
        direct_hydrology: value
            .direct_hydrology
            .restore(&hydrology_context)
            .map_err(classify_hydrology)?,
        soil_thermal: value
            .soil_thermal
            .restore(
                context.soil_thermal_owner_id,
                context.soil_thermal_configuration_sha256,
            )
            .map_err(|_| RestartAdmissionFailureV1::OwnerValidation)?,
        biogeochemistry: value.biogeochemistry.restore().map_err(|error| {
            if matches!(error, crate::ScientificOwnerRestartError::Ordering(_)) {
                RestartAdmissionFailureV1::CanonicalOrder
            } else {
                RestartAdmissionFailureV1::OwnerValidation
            }
        })?,
    })
}

fn classify_hydrology(error: crate::HydrologyRestartError) -> RestartAdmissionFailureV1 {
    let message = error.to_string();
    if message.contains("laned_active") {
        RestartAdmissionFailureV1::UnsupportedLanedActive
    } else if message.contains("retained") || message.contains("snow-free") {
        RestartAdmissionFailureV1::Child4RetainedLiquid
    } else if message.contains("canonical lane-area sum") {
        RestartAdmissionFailureV1::GroundwaterTotalArea
    } else if message.contains("groundwater") {
        RestartAdmissionFailureV1::GroundwaterPosture
    } else if message.contains("erosion") || message.contains("publication") {
        RestartAdmissionFailureV1::ErosionPublication
    } else if message.contains("order") {
        RestartAdmissionFailureV1::CanonicalOrder
    } else {
        RestartAdmissionFailureV1::OwnerValidation
    }
}

fn require_lineage(
    value: &ScientificOwnerStateSetV1,
    required: Option<u128>,
) -> Result<(), RestartAdmissionFailureV1> {
    let expected = value.vegetation_v10.last_transaction_id.to_u128();
    if required.is_some_and(|value| value != expected)
        || value
            .lse_v2
            .last_accepted_transaction_id
            .as_ref()
            .map(crate::HexU128::to_u128)
            != Some(expected)
        || value
            .soil_thermal
            .last_accepted_transaction_id
            .as_ref()
            .map(crate::HexU128::to_u128)
            != Some(expected)
        || value.biogeochemistry.last_transaction_id.to_u128() != expected
        || required.is_some_and(|_| {
            value
                .direct_hydrology
                .surface_liquid_owned_state
                .as_deref()
                .is_none_or(|state| {
                    state.continuations.is_empty()
                        || state.continuations.iter().any(|continuation| {
                            continuation
                                .last_accepted_transaction_id
                                .as_ref()
                                .map(crate::HexU128::to_u128)
                                != Some(expected)
                        })
                })
        })
    {
        return Err(RestartAdmissionFailureV1::TransactionLineage);
    }
    Ok(())
}

fn validate_forcing(
    receipts: &[SnowFreeHalfHourDayReceiptRestartV1],
    context: &ExpectedRestartStaticContext<'_>,
    day: u64,
    gsi_sha: &str,
    climate_sha: &str,
) -> Result<
    Vec<openwepp_hillslope_orchestrator::runtime_inputs::SnowFreeHalfHourDayReceipt>,
    RestartAdmissionFailureV1,
> {
    if receipts.len() != context.forcing_static_configuration.destinations.len() {
        return Err(RestartAdmissionFailureV1::ForcingReceiptCardinality);
    }
    let expected = context
        .forcing_static_configuration
        .destinations
        .iter()
        .map(|v| (&v.ofe_id, &v.tile_id))
        .collect::<Vec<_>>();
    let actual = receipts
        .iter()
        .map(|v| v.intervals.first().map(|i| (&i.ofe_id, &i.tile_id)))
        .collect::<Option<Vec<_>>>()
        .ok_or(RestartAdmissionFailureV1::ForcingReceiptCardinality)?;
    if actual != expected {
        return Err(RestartAdmissionFailureV1::ForcingReceiptOrder);
    }
    let mut restored = Vec::with_capacity(receipts.len());
    for (value, destination) in receipts
        .iter()
        .zip(&context.forcing_static_configuration.destinations)
    {
        if value.intervals.len() != 48 {
            return Err(RestartAdmissionFailureV1::ForcingReceiptCardinality);
        }
        let found = value
            .intervals
            .iter()
            .map(|interval| interval.gsi_receipt_sha256.as_str())
            .collect::<std::collections::BTreeSet<_>>();
        if found.len() != 1 || found.first().copied() != Some(gsi_sha) {
            return Err(RestartAdmissionFailureV1::HeterogeneousLaneGsiReceipt);
        }
        let receipt = value
            .restore()
            .map_err(|_| RestartAdmissionFailureV1::ForcingReceiptDigest)?;
        if receipt.day_index as u64 != day
            || receipt.run_id != context.forcing_static_configuration.run_id
            || receipt.source_climate_sha256 != climate_sha
            || receipt.intervals.iter().any(|interval| {
                interval.wb14_configuration_sha256
                    != destination.wb14_configuration_sha256
                    || interval.co2_pa.to_bits()
                        != context.forcing_static_configuration.co2_pa.to_bits()
                    || interval.reference_height_m.to_bits()
                        != context
                            .forcing_static_configuration
                            .reference_height_m
                            .to_bits()
            })
            || receipt
                .intervals
                .iter()
                .any(|interval| interval.gsi_receipt_sha256 != gsi_sha)
        {
            return Err(RestartAdmissionFailureV1::ForcingReceiptDigest);
        }
        restored.push(receipt);
    }
    Ok(restored)
}

#[cfg(test)]
mod poison_tests {
    use super::*;
    use crate::groundwater::GroundwaterAuthorityRestartV1;
    use crate::{
        DirectRuntimePostureV1, HexF64, HexU128,
        restart_authority_cross_midnight_carry_fixture,
        restart_authority_in_progress_checkpoint_fixture, to_canonical_bytes,
    };
    fn phase(
        value: &mut DirectV10RealConsumerCheckpointV1,
    ) -> (
        &mut CompleteCommittedOwnerStateV1,
        &mut ScientificOwnerStateSetV1,
        &mut DirectGsiDailyReceiptRestartV1,
        &mut SnowFreeHalfHourProviderCursorRestartV1,
        &mut Vec<SnowFreeHalfHourDayReceiptRestartV1>,
        &mut AcceptedIntervalCount,
    ) {
        let DirectV10CheckpointPhaseV1::InProgressDay {
            committed_day_beginning,
            staged_scientific,
            accepted_gsi_daily_receipt,
            ending_provider_cursor,
            validated_forcing_day_receipts,
            accepted_interval_count,
            ..
        } = &mut value.phase
        else {
            unreachable!()
        };
        (
            committed_day_beginning,
            staged_scientific,
            accepted_gsi_daily_receipt,
            ending_provider_cursor,
            validated_forcing_day_receipts,
            accepted_interval_count,
        )
    }
    fn sealed(mut value: DirectV10RealConsumerCheckpointV1) -> Vec<u8> {
        value.seal().unwrap();
        to_canonical_bytes(&value).unwrap()
    }
    #[test]
    fn complete_checkpoint_poison_matrix_is_typed_and_preserves_actual_live_bytes() {
        let (fixture, baseline, run, topology) =
            restart_authority_in_progress_checkpoint_fixture(24);
        let context = ExpectedRestartStaticContext {
            run_identity_sha256: &run,
            topology_sha256: &topology,
            vegetation_configuration: fixture
                .owners
                .runtime
                .shadow
                .restart_authority_vegetation_configuration(),
            vegetation_owner_id: fixture
                .owners
                .runtime
                .shadow
                .restart_authority_vegetation_owner_id(),
            soil_thermal_owner_id: &fixture
                .owners
                .runtime
                .shadow
                .restart_authority_soil_thermal()
                .owner_id,
            soil_thermal_configuration_sha256: &fixture
                .owners
                .runtime
                .shadow
                .restart_authority_soil_thermal()
                .configuration_sha256,
            lse_configuration: fixture
                .owners
                .runtime
                .shadow
                .restart_authority_lse_configuration(),
            surface_liquid_configuration: fixture
                .owners
                .runtime
                .shadow
                .restart_authority_surface_configuration(),
            gsi_configuration: fixture.owners.runtime.shadow.gsi_owner_configuration(),
            forcing_static_configuration: fixture
                .owners
                .runtime
                .shadow
                .provider_static_configuration(),
            phase_plan: &fixture
                .owners
                .runtime
                .shadow
                .restart_authority_hydrology_frame()
                .phase_plan,
            phase_plan_sha256: &fixture.owners.phase_plan_sha256,
            day_inputs: &fixture.owners.day_inputs,
            day_input_digests: &fixture.owners.day_input_digests,
        };
        let mut live = fixture.owners.committed.clone();
        let before = to_canonical_bytes(&live).unwrap();
        let mut check = |bytes: Vec<u8>, expected| {
            assert_eq!(
                admit_checkpoint_into_owner_store_v1(&bytes, &context, &mut live).err(),
                Some(expected)
            );
            assert_eq!(to_canonical_bytes(&live).unwrap(), before)
        };
        let mut p = baseline.clone();
        p.schema = "wrong".into();
        check(sealed(p), RestartAdmissionFailureV1::Schema);
        let mut p = baseline.clone();
        p.version = 2;
        check(sealed(p), RestartAdmissionFailureV1::UnsupportedVersion);
        let mut p = to_canonical_bytes(&baseline).unwrap();
        p.insert(1, b' ');
        check(p, RestartAdmissionFailureV1::NoncanonicalBytes);
        let mut p = baseline.clone();
        p.payload_sha256 = Sha256Hex::try_new("1".repeat(64)).unwrap();
        check(
            to_canonical_bytes(&p).unwrap(),
            RestartAdmissionFailureV1::PayloadDigest,
        );
        let value = serde_json::to_value(&baseline).unwrap();
        let mut p = value.clone();
        p.as_object_mut().unwrap().remove("phase");
        check(
            serde_json::to_vec(&p).unwrap(),
            RestartAdmissionFailureV1::MissingField,
        );
        let carry = restart_authority_cross_midnight_carry_fixture();
        let mut carry_checkpoint = baseline.clone();
        *phase(&mut carry_checkpoint).2 = carry.gsi_receipt;
        *phase(&mut carry_checkpoint).3 = carry.ending_cursor;
        *phase(&mut carry_checkpoint).4 = carry.forcing_receipts;
        if let DirectV10CheckpointPhaseV1::InProgressDay {
            staged_gsi_ending_state,
            ..
        } = &mut carry_checkpoint.phase
        {
            *staged_gsi_ending_state = carry.ending_gsi_state;
        }
        let carry_bytes = sealed(carry_checkpoint.clone());
        assert!(admit_checkpoint_v1(&carry_bytes, &context).is_ok());
        phase(&mut carry_checkpoint).3.pending_carry.pop().unwrap();
        phase(&mut carry_checkpoint).3.seal().unwrap();
        check(
            sealed(carry_checkpoint),
            RestartAdmissionFailureV1::ProviderCursor,
        );
        let mut p = value.clone();
        p["extra"] = serde_json::json!(true);
        check(
            serde_json::to_vec(&p).unwrap(),
            RestartAdmissionFailureV1::ExtraField,
        );
        check(
            serde_json::to_vec(&value).unwrap(),
            RestartAdmissionFailureV1::ReorderedField,
        );
        let raw = String::from_utf8(to_canonical_bytes(&baseline).unwrap()).unwrap();
        let p = raw.replacen(
            "\"schema\":",
            "\"schema\":\"OPENWEPP_DIRECT_V10_REAL_CONSUMER_CHECKPOINT_V1\",\"schema\":",
            1,
        );
        check(p.into_bytes(), RestartAdmissionFailureV1::DuplicateField);
        let mut p = baseline.clone();
        p.run_identity_sha256 = Sha256Hex::try_new("1".repeat(64)).unwrap();
        check(sealed(p), RestartAdmissionFailureV1::RunIdentity);
        let mut p = baseline.clone();
        p.topology_sha256 = Sha256Hex::try_new("1".repeat(64)).unwrap();
        check(sealed(p), RestartAdmissionFailureV1::TopologyIdentity);
        let mut p = baseline.clone();
        phase(&mut p).0.gsi_configuration.configuration_sha256 =
            Sha256Hex::try_new("1".repeat(64)).unwrap();
        check(sealed(p), RestartAdmissionFailureV1::ConfigurationIdentity);
        let mut p = baseline.clone();
        phase(&mut p).0.gsi_configuration.owner_id.clear();
        check(sealed(p), RestartAdmissionFailureV1::OwnerIdentity);
        let mut p = baseline.clone();
        phase(&mut p).1.vegetation_v10.owner_id = "wrong-vegetation-owner".into();
        check(sealed(p), RestartAdmissionFailureV1::OwnerIdentity);
        let mut p = baseline.clone();
        phase(&mut p).1.biogeochemistry.last_transaction_id = HexU128::from_u128(999);
        phase(&mut p).1.biogeochemistry.seal().unwrap();
        check(sealed(p), RestartAdmissionFailureV1::TransactionLineage);
        let mut p = String::from_utf8(to_canonical_bytes(&baseline).unwrap()).unwrap();
        p = p.replacen("0x00000000000000000000000000000040", "0x40", 1);
        check(
            p.into_bytes(),
            RestartAdmissionFailureV1::TransactionLineage,
        );
        let mut p = baseline.clone();
        *phase(&mut p).5 = AcceptedIntervalCount::try_new(25).unwrap();
        check(sealed(p), RestartAdmissionFailureV1::SchedulerPosition);
        let p = String::from_utf8(to_canonical_bytes(&baseline).unwrap())
            .unwrap()
            .replacen("\"day_index\":0", "\"day_index\":-1", 1);
        check(p.into_bytes(), RestartAdmissionFailureV1::SchedulerPosition);
        let mut p = baseline.clone();
        phase(&mut p).3.next_day_index = WireDayIndex(2);
        check(sealed(p), RestartAdmissionFailureV1::ProviderCursor);
        let mut p = baseline.clone();
        phase(&mut p).3.next_day_index = WireDayIndex(0);
        check(sealed(p), RestartAdmissionFailureV1::ProviderCursor);
        let mut p = baseline.clone();
        phase(&mut p).2.day_index = WireDayIndex(1);
        check(sealed(p), RestartAdmissionFailureV1::GsiReceipt);
        let mut p = baseline.clone();
        phase(&mut p)
            .2
            .ending_state
            .history_oldest_first
            .push(HexF64::from_f64(2.0));
        phase(&mut p).2.ending_state.history_oldest_first.swap(0, 1);
        check(sealed(p), RestartAdmissionFailureV1::GsiReceipt);
        let mut p = baseline.clone();
        phase(&mut p).4[0].intervals[1].gsi_receipt_sha256 =
            Sha256Hex::try_new("1".repeat(64)).unwrap();
        check(
            sealed(p),
            RestartAdmissionFailureV1::HeterogeneousLaneGsiReceipt,
        );
        let mut p = baseline.clone();
        phase(&mut p).4.pop();
        check(
            sealed(p),
            RestartAdmissionFailureV1::ForcingReceiptCardinality,
        );
        let mut p = baseline.clone();
        phase(&mut p).4[0].intervals.remove(0);
        check(
            sealed(p),
            RestartAdmissionFailureV1::ForcingReceiptCardinality,
        );
        let mut p = baseline.clone();
        let interval = phase(&mut p).4[0].intervals[0].clone();
        phase(&mut p).4[0].intervals.insert(0, interval);
        check(
            sealed(p),
            RestartAdmissionFailureV1::ForcingReceiptCardinality,
        );
        let mut p = baseline.clone();
        phase(&mut p).4.swap(0, 1);
        check(sealed(p), RestartAdmissionFailureV1::ForcingReceiptOrder);
        let mut p = baseline.clone();
        phase(&mut p).4[0].receipt_sha256 = Sha256Hex::try_new("1".repeat(64)).unwrap();
        check(sealed(p), RestartAdmissionFailureV1::ForcingReceiptDigest);
        let mut p = serde_json::to_value(&baseline).unwrap();
        p["phase"]["validated_forcing_day_receipts"][0]
            .as_object_mut()
            .unwrap()
            .remove("next_day_precipitation_carry");
        check(
            serde_json::to_vec(&p).unwrap(),
            RestartAdmissionFailureV1::MissingField,
        );
        let mut p = baseline.clone();
        phase(&mut p).1.vegetation_v10.state_sha256 = Sha256Hex::try_new("1".repeat(64)).unwrap();
        check(sealed(p), RestartAdmissionFailureV1::V10V9Projection);
        let mut p = baseline.clone();
        phase(&mut p).1.lse_v2.state_sha256 = Sha256Hex::try_new("1".repeat(64)).unwrap();
        check(sealed(p), RestartAdmissionFailureV1::LseV2V1Projection);
        let mut p = baseline.clone();
        phase(&mut p).1.soil_thermal.restart_payload_sha256 =
            Sha256Hex::try_new("1".repeat(64)).unwrap();
        check(sealed(p), RestartAdmissionFailureV1::OwnerValidation);
        let mut p = baseline.clone();
        phase(&mut p).1.biogeochemistry.state_sha256 =
            Sha256Hex::try_new("1".repeat(64)).unwrap();
        check(sealed(p), RestartAdmissionFailureV1::OwnerValidation);
        let mut p = baseline.clone();
        phase(&mut p).1.biogeochemistry.layers[0].ammonium_n = HexF64::from_f64(-1.0);
        check(sealed(p), RestartAdmissionFailureV1::OwnerValidation);
        let mut p = baseline.clone();
        phase(&mut p).1.direct_hydrology.runtime_posture =
            DirectRuntimePostureV1::UnsupportedLanedActive;
        check(sealed(p), RestartAdmissionFailureV1::UnsupportedLanedActive);
        let mut p = baseline.clone();
        phase(&mut p).1.direct_hydrology.lanes[0].area_m2 = HexF64::from_f64(-0.0);
        check(sealed(p), RestartAdmissionFailureV1::OwnerValidation);
        let mut p = baseline.clone();
        phase(&mut p).1.biogeochemistry.layers.reverse();
        phase(&mut p).1.biogeochemistry.seal().unwrap();
        check(sealed(p), RestartAdmissionFailureV1::CanonicalOrder);
        let mut p = serde_json::to_value(&baseline).unwrap();
        p["phase"]["staged_scientific"]
            .as_object_mut()
            .unwrap()
            .remove("direct_hydrology");
        check(
            serde_json::to_vec(&p).unwrap(),
            RestartAdmissionFailureV1::OwnerOmission,
        );
        for field in [
            "vegetation_v10",
            "lse_v2",
            "soil_thermal",
            "biogeochemistry",
        ] {
            let mut p = serde_json::to_value(&baseline).unwrap();
            p["phase"]["staged_scientific"]
                .as_object_mut()
                .unwrap()
                .remove(field);
            check(
                serde_json::to_vec(&p).unwrap(),
                RestartAdmissionFailureV1::OwnerOmission,
            );
        }
        for field in [
            "gsi_configuration",
            "gsi_state",
            "static_forcing_configuration",
            "provider_cursor",
            "surface_liquid_configuration",
        ] {
            let mut p = serde_json::to_value(&baseline).unwrap();
            p["phase"]["committed_day_beginning"]
                .as_object_mut()
                .unwrap()
                .remove(field);
            check(
                serde_json::to_vec(&p).unwrap(),
                RestartAdmissionFailureV1::OwnerOmission,
            );
        }
        let mut p = baseline.clone();
        phase(&mut p).1.direct_hydrology.lanes[0]
            .winter_column
            .snow
            .liquid_water_retained_m = HexF64::from_f64(1.0);
        check(sealed(p), RestartAdmissionFailureV1::Child4RetainedLiquid);
        let mut p = baseline.clone();
        phase(&mut p).1.direct_hydrology.groundwater.storage_m3 = HexF64::from_f64(1.0);
        check(sealed(p), RestartAdmissionFailureV1::GroundwaterPosture);
        let mut p = baseline.clone();
        let groundwater = &mut phase(&mut p).1.direct_hydrology.groundwater;
        groundwater.authority = GroundwaterAuthorityRestartV1::LinearReservoir {
            initial_storage_depth_m: HexF64::from_f64(0.0),
            baseflow_coeff_per_day: HexF64::from_f64(0.0),
            deep_seepage_coeff_per_day: HexF64::from_f64(0.0),
            baseflow_threshold_area_ha: HexF64::from_f64(0.0),
        };
        groundwater.initialized_area_m2 = Some(HexF64::from_f64(999.0));
        check(sealed(p), RestartAdmissionFailureV1::GroundwaterTotalArea);
        let mut p = baseline.clone();
        let committed = phase(&mut p).0.scientific.clone();
        *phase(&mut p).1 = committed;
        check(sealed(p), RestartAdmissionFailureV1::TransactionLineage);
        let mut p = baseline.clone();
        phase(&mut p).1.direct_hydrology.surface_liquid_owned_state = None;
        check(sealed(p), RestartAdmissionFailureV1::TransactionLineage);
        let mut p = baseline.clone();
        phase(&mut p).1.direct_hydrology.lanes[0]
            .erosion_downstream_operands
            .publication
            .peak_runoff_rate_m_s = Some(HexF64::from_f64(1.0));
        check(sealed(p), RestartAdmissionFailureV1::ErosionPublication);
        let mut p = baseline.clone();
        if let Some(state) = phase(&mut p)
            .1
            .direct_hydrology
            .surface_liquid_owned_state
            .as_deref_mut()
        {
            state.configuration_sha256 = Sha256Hex::try_new("1".repeat(64)).unwrap()
        }
        check(
            sealed(p),
            RestartAdmissionFailureV1::SurfaceLiquidConfiguration,
        );
        assert!(admit_checkpoint_into_owner_store_v1(
            &to_canonical_bytes(&baseline).unwrap(),
            &context,
            &mut live,
        )
        .is_ok());
        assert_ne!(to_canonical_bytes(&live).unwrap(), before);
    }
}
