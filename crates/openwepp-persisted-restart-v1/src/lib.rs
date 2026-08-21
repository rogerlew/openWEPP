//! Production-owned, default-off persisted restart V1 implementation.
//!
//! The wire and admission semantics are promoted unchanged from the released
//! authority. This crate owns no selector, publication, output, or production
//! state mutation path; callers receive an isolated restored candidate.

// This module is a byte- and behavior-preserving promotion of the released
// restart authority. Keep its reviewed function shapes and numeric expressions
// stable; these selected pedantic style lints are intentionally not rewritten
// during the authority-to-production move.
#![allow(
    clippy::cast_lossless,
    clippy::cast_precision_loss,
    clippy::cast_sign_loss,
    clippy::implicit_clone,
    clippy::large_enum_variant,
    clippy::missing_errors_doc,
    clippy::missing_panics_doc,
    clippy::must_use_candidate,
    clippy::needless_pass_by_value,
    clippy::redundant_closure_for_method_calls,
    clippy::ref_option,
    clippy::semicolon_if_nothing_returned,
    clippy::too_many_lines,
    clippy::unreadable_literal
)]

mod canonical;
mod checkpoint;
mod continuation_template;
mod erosion;
#[cfg(feature = "fixtures")]
mod evidence_fixture;
mod groundwater;
mod growth_et;
mod gsi_forcing;
mod host;
mod hydrology_core;
mod hydrology_restart;
mod primitives;
mod projection;
mod scientific_owners;
mod subsurface;
mod surface_liquid;
mod transaction;
mod transfer;
mod vegetation_v11;
mod vegetation_v11_v3;
mod winter;

pub use canonical::{
    CanonicalJsonError, canonical_sha256, from_canonical_bytes, to_canonical_bytes,
};
pub use checkpoint::{
    CompleteCommittedOwnerStateV1, DirectV10CheckpointPhaseV1, DirectV10RealConsumerCheckpointV1,
    ExpectedRestartStaticContext, IsolatedRestoredCheckpointV1, RestartAdmissionFailureV1,
    RestoredCompleteCommittedOwnerStateV1, RestoredScientificOwnerStateSetV1,
    ScientificOwnerStateSetV1, admit_checkpoint_into_owner_store_v1, admit_checkpoint_v1,
};
pub use continuation_template::*;
pub use erosion::{
    DirectErosionDownstreamRestartV1, DirectErosionInflowIntakeRestartV1,
    DirectErosionRuntimeCarryRestartV1, ErosionRestartError,
};
#[cfg(feature = "fixtures")]
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
};
pub use host::{
    DirectV10RestartHost, RestartInstallError, admit_and_install_checkpoint_v1,
    install_restored_checkpoint,
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
pub use projection::{
    checkpoint_identities_v1, project_complete_owner_state_v1, project_scientific_owner_state_v1,
};
pub use scientific_owners::*;
pub use subsurface::{DirectSubsurfaceLayerRestartV1, SubsurfaceRestartError};
pub use surface_liquid::{
    DirectSurfaceLiquidConfigurationRestartV1, DirectSurfaceLiquidOwnedStateRestartV1,
    GroundIngressModeWireV1, SurfaceClassWireV1, SurfaceLiquidRestartError, WaterSourceWireV1,
};
pub use transaction::{DirectV10PreparedDayTransactionV1, RestartTransactionError};
pub use transfer::{
    DirectLaneTransferLedgerRestartV1, DirectRunTransferDownstreamOperandsRestartV1,
    DirectTransferBuffersRestartV1, TransferRestartError,
};
pub use vegetation_v11::*;
pub use vegetation_v11_v3::*;
pub use winter::{DirectWinterColumnRestartV1, RestoredWinterCompatibility, WinterRestartError};
