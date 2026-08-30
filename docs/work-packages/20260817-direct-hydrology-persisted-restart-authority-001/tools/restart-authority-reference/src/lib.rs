//! Executable, package-local authority for persisted-restart wire semantics.
//!
//! This crate is evidence tooling. It is not linked by the openWEPP workspace
//! and exposes no production selector or runtime path.

mod canonical;
mod checkpoint;
mod continuation_template;
mod erosion;
mod evidence_fixture;
mod groundwater;
mod growth_et;
mod gsi_forcing;
mod hydrology_core;
mod hydrology_restart;
mod primitives;
mod scientific_owners;
mod subsurface;
mod surface_liquid;
mod transfer;
mod winter;

pub use canonical::{
    CanonicalJsonError, canonical_sha256, from_canonical_bytes, to_canonical_bytes,
};
pub use checkpoint::{
    CompleteCommittedOwnerStateV1, DirectV10CheckpointPhaseV1, DirectV10RealConsumerCheckpointV1,
    ExpectedRestartStaticContext, IsolatedRestoredCheckpointV1, RestartAdmissionFailureV1,
    ScientificOwnerStateSetV1, admit_checkpoint_v1,
};
pub use continuation_template::*;
pub use erosion::{
    DirectErosionDownstreamRestartV1, DirectErosionInflowIntakeRestartV1,
    DirectErosionRuntimeCarryRestartV1, ErosionRestartError,
};
pub use evidence_fixture::*;
pub use groundwater::{DirectGroundwaterRunStateRestartV1, GroundwaterRestartError};
pub use growth_et::{
    DirectEvapotranspirationStageRestartV1, DirectGrowthStateSurfaceRestartV1, GrowthEtRestartError,
};
pub use gsi_forcing::{
    DirectGsiDailyReceiptRestartV1, DirectGsiDateRestartV1, DirectGsiOwnerConfigurationRestartV1,
    DirectGsiOwnerStateRestartV1, GsiForcingRestartError, SnowFreeHalfHourDayReceiptRestartV1,
    SnowFreeHalfHourDestinationRestartV1, SnowFreeHalfHourProviderCursorRestartV1,
    SnowFreeHalfHourStaticConfigurationRestartV1, SnowFreePrecipitationParcelRestartV1,
    SnowFreeSolidPrecipitationParcelRestartV1,
};
pub use hydrology_core::{DirectHydrologyCoreError, DirectWaterStateRestartV1};
pub use hydrology_restart::{
    DirectHydrologyRestartV1, DirectLaneRestartV1, DirectRuntimePostureV1,
    ExpectedDirectHydrologyRestartContext, HydrologyRestartError,
};
pub use primitives::{
    AcceptedIntervalCount, DestinationCount, HexF64, HexU128, InProgressIntervalIndex, LaneCount,
    OptionalLaneLink, Sha256Hex, WireDayIndex, WireIntervalIndex, WireLaneId, WirePrimitiveError,
};
pub use scientific_owners::*;
pub use subsurface::{DirectSubsurfaceLayerRestartV1, SubsurfaceRestartError};
pub use surface_liquid::{
    DirectSurfaceLiquidConfigurationRestartV1, DirectSurfaceLiquidOwnedStateRestartV1,
    GroundIngressModeWireV1, SurfaceClassWireV1, SurfaceLiquidRestartError, WaterSourceWireV1,
};
pub use transfer::{
    DirectLaneTransferLedgerRestartV1, DirectRunTransferDownstreamOperandsRestartV1,
    DirectTransferBuffersRestartV1, TransferRestartError,
};
pub use winter::{DirectWinterColumnRestartV1, RestoredWinterCompatibility, WinterRestartError};
