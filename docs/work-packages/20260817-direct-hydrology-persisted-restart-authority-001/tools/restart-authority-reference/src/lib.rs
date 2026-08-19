//! Executable, package-local authority for persisted-restart wire semantics.
//!
//! This crate is evidence tooling. It is not linked by the openWEPP workspace
//! and exposes no production selector or runtime path.

mod canonical;
mod growth_et;
mod hydrology_core;
mod primitives;
mod subsurface;
mod transfer;

pub use canonical::{CanonicalJsonError, from_canonical_bytes, to_canonical_bytes};
pub use growth_et::{
    DirectEvapotranspirationStageRestartV1, DirectGrowthStateSurfaceRestartV1, GrowthEtRestartError,
};
pub use hydrology_core::{DirectHydrologyCoreError, DirectWaterStateRestartV1};
pub use primitives::{
    AcceptedIntervalCount, DestinationCount, HexF64, HexU128, InProgressIntervalIndex, LaneCount,
    OptionalLaneLink, Sha256Hex, WireDayIndex, WireIntervalIndex, WireLaneId, WirePrimitiveError,
};
pub use subsurface::{DirectSubsurfaceLayerRestartV1, SubsurfaceRestartError};
pub use transfer::{
    DirectLaneTransferLedgerRestartV1, DirectRunTransferDownstreamOperandsRestartV1,
    DirectTransferBuffersRestartV1, TransferRestartError,
};
