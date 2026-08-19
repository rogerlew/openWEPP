//! Executable, package-local authority for persisted-restart wire semantics.
//!
//! This crate is evidence tooling. It is not linked by the openWEPP workspace
//! and exposes no production selector or runtime path.

mod canonical;
mod checkpoint;
mod erosion;
mod groundwater;
mod growth_et;
mod hydrology_core;
mod hydrology_restart;
mod primitives;
mod subsurface;
mod surface_liquid;
mod transfer;
mod winter;

pub use canonical::{
    CanonicalJsonError, canonical_sha256, from_canonical_bytes, to_canonical_bytes,
};
pub use checkpoint::{
    CheckpointError, CheckpointPhaseV1, DirectV10RealConsumerCheckpointV1, OmissionConsequenceV1,
    OwnerKindV1, OwnerPoisonV1, OwnerSetV1, PersistedOwnerEnvelopeV1,
};
pub use erosion::{
    DirectErosionDownstreamRestartV1, DirectErosionInflowIntakeRestartV1,
    DirectErosionRuntimeCarryRestartV1, ErosionRestartError,
};
pub use groundwater::{DirectGroundwaterRunStateRestartV1, GroundwaterRestartError};
pub use growth_et::{
    DirectEvapotranspirationStageRestartV1, DirectGrowthStateSurfaceRestartV1, GrowthEtRestartError,
};
pub use hydrology_core::{DirectHydrologyCoreError, DirectWaterStateRestartV1};
pub use hydrology_restart::{DirectHydrologyRestartV1, DirectLaneRestartV1, HydrologyRestartError};
pub use primitives::{
    AcceptedIntervalCount, DestinationCount, HexF64, HexU128, InProgressIntervalIndex, LaneCount,
    OptionalLaneLink, Sha256Hex, WireDayIndex, WireIntervalIndex, WireLaneId, WirePrimitiveError,
};
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
