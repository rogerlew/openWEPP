use std::error::Error;
use std::fmt;
use std::sync::atomic::{AtomicU64, Ordering};

use crate::constants::{WB11_ZERO_THRESHOLD, WB16_PEAKRO_FLOOR, WB16_RUNOFF_NEAR_ZERO_THRESHOLD};
use crate::hydrology::{DirectSnowStage3Diagnostics, SnowAlbedoState};
use crate::winter_column::{
    DirectFrostFineLayerState, DirectFrostLaneState, DirectFrostLayerShadowState,
    DirectSnowLaneState, DirectSnowLayerState, DirectWinterColumnState,
};

pub const DIRECT_TRANSFER_HOUR_COUNT: usize = 24;
pub const DIRECT_PHASE_COUNT: usize = 14;
pub const DIRECT_R3A_PHASE_SPAN_COUNT: usize = 2;
pub const DIRECT_R3A_INPUT_ACCOUNTING_SPAN: [DirectPhaseKind; DIRECT_R3A_PHASE_SPAN_COUNT] = [
    DirectPhaseKind::Normalization,
    DirectPhaseKind::LateralTransfer,
];
pub const DIRECT_R3B_PHASE_SPAN_COUNT: usize = 3;
pub const DIRECT_R3B_WATER_LEDGER_SPAN: [DirectPhaseKind; DIRECT_R3B_PHASE_SPAN_COUNT] = [
    DirectPhaseKind::RunoffReconciliation,
    DirectPhaseKind::StorageReconciliation,
    DirectPhaseKind::ClosureDiagnostics,
];
pub const DIRECT_R3C_PHASE_SPAN_COUNT: usize = 3;
pub const DIRECT_R3C_LANE_TRANSFER_SPAN: [DirectPhaseKind; DIRECT_R3C_PHASE_SPAN_COUNT] = [
    DirectPhaseKind::LateralTransfer,
    DirectPhaseKind::RunoffReconciliation,
    DirectPhaseKind::ClosureDiagnostics,
];
pub const DIRECT_R4A_PHASE_SPAN_COUNT: usize = 3;
pub const DIRECT_R4A_RUNOFF_PARTITION_SPAN: [DirectPhaseKind; DIRECT_R4A_PHASE_SPAN_COUNT] = [
    DirectPhaseKind::RunoffReconciliation,
    DirectPhaseKind::StorageReconciliation,
    DirectPhaseKind::ClosureDiagnostics,
];
pub const DIRECT_R4B_PHASE_SPAN_COUNT: usize = 2;
pub const DIRECT_R4B_STORAGE_RECONCILIATION_SPAN: [DirectPhaseKind; DIRECT_R4B_PHASE_SPAN_COUNT] = [
    DirectPhaseKind::StorageReconciliation,
    DirectPhaseKind::ClosureDiagnostics,
];
pub const DIRECT_R4C_PHASE_SPAN_COUNT: usize = 2;
pub const DIRECT_R4C_STORAGE_INPUT_SPAN: [DirectPhaseKind; DIRECT_R4C_PHASE_SPAN_COUNT] = [
    DirectPhaseKind::Normalization,
    DirectPhaseKind::StorageReconciliation,
];
pub const DIRECT_R4D_PHASE_SPAN_COUNT: usize = 2;
pub const DIRECT_R4D_DEEP_SEEPAGE_SPAN: [DirectPhaseKind; DIRECT_R4D_PHASE_SPAN_COUNT] = [
    DirectPhaseKind::PercolationDeepSeepage,
    DirectPhaseKind::StorageReconciliation,
];
pub const DIRECT_R4E_PHASE_SPAN_COUNT: usize = 3;
pub const DIRECT_R4E_SUBSURFACE_LOSS_SPAN: [DirectPhaseKind; DIRECT_R4E_PHASE_SPAN_COUNT] = [
    DirectPhaseKind::Drainage,
    DirectPhaseKind::LateralTransfer,
    DirectPhaseKind::StorageReconciliation,
];
pub const DIRECT_R4F_PHASE_SPAN_COUNT: usize = 2;
pub const DIRECT_R4F_EVAPOTRANSPIRATION_SPAN: [DirectPhaseKind; DIRECT_R4F_PHASE_SPAN_COUNT] = [
    DirectPhaseKind::Evapotranspiration,
    DirectPhaseKind::StorageReconciliation,
];
pub const DIRECT_R4G_PHASE_SPAN_COUNT: usize = 2;
pub const DIRECT_R4G_SNOW_COUPLING_SPAN: [DirectPhaseKind; DIRECT_R4G_PHASE_SPAN_COUNT] = [
    DirectPhaseKind::Normalization,
    DirectPhaseKind::StorageReconciliation,
];
pub const DIRECT_R4I_PHASE_SPAN_COUNT: usize = 2;
pub const DIRECT_R4I_LIQUID_INPUT_SPAN: [DirectPhaseKind; DIRECT_R4I_PHASE_SPAN_COUNT] = [
    DirectPhaseKind::Normalization,
    DirectPhaseKind::RunoffReconciliation,
];
pub const DIRECT_R4J_PHASE_SPAN_COUNT: usize = 2;
pub const DIRECT_R4J_RUNON_CARRY_SPAN: [DirectPhaseKind; DIRECT_R4J_PHASE_SPAN_COUNT] = [
    DirectPhaseKind::LateralTransfer,
    DirectPhaseKind::RunoffReconciliation,
];
pub const DIRECT_R4K_PHASE_SPAN_COUNT: usize = 2;
pub const DIRECT_R4K_INFILTRATION_DEPRESSION_SPAN: [DirectPhaseKind; DIRECT_R4K_PHASE_SPAN_COUNT] = [
    DirectPhaseKind::RunoffReconciliation,
    DirectPhaseKind::StorageReconciliation,
];
pub const DIRECT_R4L_PHASE_SPAN_COUNT: usize = 2;
pub const DIRECT_R4L_SATURATION_ADDBACK_SPAN: [DirectPhaseKind; DIRECT_R4L_PHASE_SPAN_COUNT] = [
    DirectPhaseKind::RunoffReconciliation,
    DirectPhaseKind::StorageReconciliation,
];
pub const DIRECT_R4M_PHASE_SPAN_COUNT: usize = 2;
pub const DIRECT_R4M_PERCOLATION_SPAN: [DirectPhaseKind; DIRECT_R4M_PHASE_SPAN_COUNT] = [
    DirectPhaseKind::PercolationDeepSeepage,
    DirectPhaseKind::StorageReconciliation,
];
pub const DIRECT_R4O_PHASE_SPAN_COUNT: usize = 3;
pub const DIRECT_R4O_SUBSURFACE_SPAN: [DirectPhaseKind; DIRECT_R4O_PHASE_SPAN_COUNT] = [
    DirectPhaseKind::Drainage,
    DirectPhaseKind::LateralTransfer,
    DirectPhaseKind::StorageReconciliation,
];
pub const DIRECT_R4N_SURFACE_PHASE_SPAN_COUNT: usize = 1;
pub const DIRECT_R4N_SURFACE_ET_SPAN: [DirectPhaseKind; DIRECT_R4N_SURFACE_PHASE_SPAN_COUNT] =
    [DirectPhaseKind::Evapotranspiration];
pub const DIRECT_R4N_ROOT_PHASE_SPAN_COUNT: usize = 2;
pub const DIRECT_R4N_ROOT_UPTAKE_SPAN: [DirectPhaseKind; DIRECT_R4N_ROOT_PHASE_SPAN_COUNT] = [
    DirectPhaseKind::PlantRootUptake,
    DirectPhaseKind::StorageReconciliation,
];
pub const DIRECT_R4N_PHASE_SPAN_COUNT: usize =
    DIRECT_R4N_SURFACE_PHASE_SPAN_COUNT + DIRECT_R4N_ROOT_PHASE_SPAN_COUNT;
pub const DIRECT_R4PQZ_PHASE_SPAN_COUNT: usize = 2;
pub const DIRECT_R4PQZ_HYDROLOGY_PROJECTION_SPAN: [DirectPhaseKind; DIRECT_R4PQZ_PHASE_SPAN_COUNT] = [
    DirectPhaseKind::StorageReconciliation,
    DirectPhaseKind::ClosureDiagnostics,
];
pub const DIRECT_R7D6_PEAK_RUNOFF_PHASE_SPAN_COUNT: usize = 2;
pub const DIRECT_R7D6_PEAK_RUNOFF_SPAN: [DirectPhaseKind;
    DIRECT_R7D6_PEAK_RUNOFF_PHASE_SPAN_COUNT] = [
    DirectPhaseKind::RunoffReconciliation,
    DirectPhaseKind::ClosureDiagnostics,
];
pub const DIRECT_R5B_NORMALIZATION_PHASE_SPAN_COUNT: usize = 1;
pub const DIRECT_R5B_NORMALIZATION_SPAN: [DirectPhaseKind;
    DIRECT_R5B_NORMALIZATION_PHASE_SPAN_COUNT] = [DirectPhaseKind::Normalization];
pub const DIRECT_R5B_STORAGE_BOUNDS_PHASE_SPAN_COUNT: usize = 1;
pub const DIRECT_R5B_STORAGE_BOUNDS_SPAN: [DirectPhaseKind;
    DIRECT_R5B_STORAGE_BOUNDS_PHASE_SPAN_COUNT] = [DirectPhaseKind::StorageBounds];
pub const DIRECT_R5C_DECOMPOSITION_PHASE_SPAN_COUNT: usize = 1;
pub const DIRECT_R5C_DECOMPOSITION_SPAN: [DirectPhaseKind;
    DIRECT_R5C_DECOMPOSITION_PHASE_SPAN_COUNT] = [DirectPhaseKind::DecompositionTransition];
pub const DIRECT_R5C_RESIDUE_PARTITION_PHASE_SPAN_COUNT: usize = 1;
pub const DIRECT_R5C_RESIDUE_PARTITION_SPAN: [DirectPhaseKind;
    DIRECT_R5C_RESIDUE_PARTITION_PHASE_SPAN_COUNT] = [DirectPhaseKind::ResiduePartitionTransition];
pub const DIRECT_R5D_ANNUAL_GROWTH_PHASE_SPAN_COUNT: usize = 1;
pub const DIRECT_R5D_ANNUAL_GROWTH_SPAN: [DirectPhaseKind;
    DIRECT_R5D_ANNUAL_GROWTH_PHASE_SPAN_COUNT] = [DirectPhaseKind::AnnualGrowthTransition];
pub const DIRECT_R5D_PERENNIAL_GROWTH_PHASE_SPAN_COUNT: usize = 1;
pub const DIRECT_R5D_PERENNIAL_GROWTH_SPAN: [DirectPhaseKind;
    DIRECT_R5D_PERENNIAL_GROWTH_PHASE_SPAN_COUNT] = [DirectPhaseKind::PerennialGrowthTransition];
pub const DIRECT_R7D6_EROSION_PHASE_SPAN_COUNT: usize = 2;
pub const DIRECT_R7D6_EROSION_SPAN: [DirectPhaseKind; DIRECT_R7D6_EROSION_PHASE_SPAN_COUNT] = [
    DirectPhaseKind::RunoffReconciliation,
    DirectPhaseKind::ClosureDiagnostics,
];

static DIRECT_AUDIT: DirectRuntimeAuditCounters = DirectRuntimeAuditCounters::new();

mod decomposition;
mod erosion;
mod evapotranspiration;
mod growth;
mod normalization;
mod projection;
mod runoff;
mod storage;
mod subsurface;

pub use decomposition::{
    DirectDecompositionAction, DirectDecompositionActiveContext,
    DirectDecompositionDownstreamOperands, DirectDecompositionInputs,
    DirectDecompositionShadowProjection, DirectDecompositionSpanReport, DirectDecompositionState,
    DirectResiduePartitionDownstreamOperands, DirectResiduePartitionInputs,
    DirectResiduePartitionShadowProjection, DirectResiduePartitionSpanReport,
    DirectResiduePartitionState,
};
pub use erosion::{
    DirectErod13Inputs, DirectErod13State, DirectErod14ClassInputs, DirectErod14ClassState,
    DirectErod14Inputs, DirectErod14State, DirectErosionDownstreamOperands, DirectErosionInputs,
    DirectErosionShadowProjection, DirectErosionSpanReport, DirectErosionState,
};
pub use evapotranspiration::{
    DirectEvapotranspirationComputeDownstreamOperands, DirectEvapotranspirationComputeInputs,
    DirectEvapotranspirationComputeShadowProjection, DirectEvapotranspirationComputeSpanReport,
    DirectEvapotranspirationComputeState, DirectEvapotranspirationPmetComputeInputs,
    DirectEvapotranspirationPmetInputs, DirectEvapotranspirationStageState,
    DirectEvapotranspirationSurfaceDownstreamOperands,
    DirectEvapotranspirationSurfaceShadowProjection, DirectEvapotranspirationSurfaceSpanReport,
    DirectEvapotranspirationSurfaceState,
};
pub use growth::{
    DirectGrowthAction, DirectGrowthActiveContext, DirectGrowthDownstreamOperands,
    DirectGrowthInputs, DirectGrowthShadowProjection, DirectGrowthSpanReport, DirectGrowthState,
    DirectGrowthStateSurface,
};
pub use normalization::{
    DirectNormalizationDownstreamOperands, DirectNormalizationInputs,
    DirectNormalizationShadowProjection, DirectNormalizationSpanReport, DirectNormalizationState,
    DirectStorageBoundsDownstreamOperands, DirectStorageBoundsInputs,
    DirectStorageBoundsShadowProjection, DirectStorageBoundsSpanReport, DirectStorageBoundsState,
};
pub use projection::{
    DirectHydrologyProjectionDownstreamOperands, DirectHydrologyProjectionInputs,
    DirectHydrologyProjectionShadowProjection, DirectHydrologyProjectionSpanReport,
    DirectHydrologyProjectionState,
};
pub use runoff::{
    DirectCanopyInterceptionInputs, DirectCanopyInterceptionState,
    DirectInfiltrationDepressionDownstreamOperands, DirectInfiltrationDepressionInputs,
    DirectInfiltrationDepressionShadowProjection, DirectInfiltrationDepressionSpanReport,
    DirectInfiltrationDepressionState, DirectLiquidInputDownstreamOperands,
    DirectLiquidInputInputs, DirectLiquidInputShadowProjection, DirectLiquidInputSpanReport,
    DirectLiquidInputState, DirectPeakRunoffDownstreamOperands, DirectPeakRunoffInputs,
    DirectPeakRunoffShadowProjection, DirectPeakRunoffSpanReport, DirectPeakRunoffState,
    DirectRunoffDownstreamOperands, DirectRunoffPartitionInputs, DirectRunoffPartitionSpanReport,
    DirectRunoffPartitionState, DirectRunoffShadowProjection, DirectRunonCarryDownstreamOperands,
    DirectRunonCarryInputs, DirectRunonCarryShadowProjection, DirectRunonCarrySpanReport,
    DirectRunonCarryState, DirectSaturationAddbackDownstreamOperands,
    DirectSaturationAddbackInputs, DirectSaturationAddbackShadowProjection,
    DirectSaturationAddbackSpanReport, DirectSaturationAddbackState, DirectWb14HyetographInterval,
    DirectWb14InfiltrationProducerInputs, compute_direct_canopy_interception,
};
pub use storage::{
    DirectDeepSeepageDownstreamOperands, DirectDeepSeepageInputs,
    DirectDeepSeepageShadowProjection, DirectDeepSeepageSpanReport, DirectDeepSeepageState,
    DirectEvapotranspirationDownstreamOperands, DirectEvapotranspirationInputs,
    DirectEvapotranspirationShadowProjection, DirectEvapotranspirationSpanReport,
    DirectEvapotranspirationState, DirectSnowCouplingDownstreamOperands, DirectSnowCouplingInputs,
    DirectSnowCouplingShadowProjection, DirectSnowCouplingSpanReport, DirectSnowCouplingState,
    DirectStorageDownstreamOperands, DirectStorageInputDownstreamOperands,
    DirectStorageInputInputs, DirectStorageInputShadowProjection, DirectStorageInputSpanReport,
    DirectStorageInputState, DirectStorageReconciliationInputs,
    DirectStorageReconciliationSpanReport, DirectStorageReconciliationState,
    DirectStorageShadowProjection, DirectSubsurfaceLossDownstreamOperands,
    DirectSubsurfaceLossInputs, DirectSubsurfaceLossShadowProjection,
    DirectSubsurfaceLossSpanReport, DirectSubsurfaceLossState,
};
pub use subsurface::{
    DirectPercolationDownstreamOperands, DirectPercolationInputs,
    DirectPercolationShadowProjection, DirectPercolationSpanReport, DirectPercolationState,
    DirectSubsurfaceComputeDownstreamOperands, DirectSubsurfaceComputeInputs,
    DirectSubsurfaceComputeShadowProjection, DirectSubsurfaceComputeSpanReport,
    DirectSubsurfaceComputeState, DirectSubsurfaceLayerInputs, DirectSubsurfaceLayerState,
};

include!("direct_runtime/00_core_frames.rs");
include!("direct_runtime/01_publication.rs");
include!("direct_runtime/02_state_reports.rs");
include!("direct_runtime/03_executor.rs");
include!("direct_runtime/04_audit_error_helpers.rs");
