#![allow(clippy::missing_errors_doc)]

pub mod runtime_inputs;
pub mod schedule_export;

mod constants;
mod consumer_boundary;
mod day_frame;
mod direct_runtime;
mod hydrology;
mod phase;
mod scheduler;
mod winter_column;

#[cfg(test)]
mod tests;

use std::collections::{BTreeMap, BTreeSet};
use std::error::Error;
use std::fmt;

use openwepp_kernel_contract::{
    BoundarySymbol, BoundaryValue, HillslopeActiveGrazingCycle, HillslopeAnnualDecompositionAction,
    HillslopeAnnualDecompositionControl, HillslopeAnnualGrowthAction, HillslopeAnnualGrowthControl,
    HillslopeConsumerAdapter, HillslopeDecompositionKernelContext,
    HillslopeDecompositionManagementClass, HillslopeDecompositionTransitionControl,
    HillslopeDecompositionTransitionPayload, HillslopeGrowthKernelContext,
    HillslopeGrowthManagementClass, HillslopeGrowthStateSurface, HillslopeGrowthTransitionControl,
    HillslopeGrowthTransitionPayload, HillslopeIrrigationDepletionPeriodField,
    HillslopeIrrigationFixedDateEventField, HillslopeKernel, HillslopeKernelPhaseClass,
    HillslopeKernelRequest, HillslopePerennialDecompositionAction,
    HillslopePerennialDecompositionControl, HillslopePerennialGrowthAction,
    HillslopePerennialGrowthControl, HillslopeProductionFluxSymbol, HillslopeProductionStateSymbol,
    HotSymbolTables, IndexedBoundarySymbol, IndexedKernelWritebackPayload, IndexedWritebackField,
    IndexedWritebackSurface, KernelRunResponse, KernelWritebackApplyResult, KernelWritebackPayload,
    MAX_CLIMATE_FORCING_SERIES_POINTS, SymbolId, SymbolRegistry, SymbolRegistryError,
    WRITEBACK_APPLY_MESSAGE_ID, WritebackDecisionOutcome, WritebackError, WritebackField,
    apply_indexed_kernel_writeback, apply_kernel_writeback, evaluate_indexed_kernel_writeback,
    evaluate_kernel_writeback,
};
use openwepp_sim_contract::closure::ClosureViolation;
use openwepp_sim_contract::status::{
    BoundaryClass, ClampClass, SimulationPhase, SimulationStatus, StatusClassification, StatusError,
};
use openwepp_topology::TopologyValidationReport;

pub use consumer_boundary::{
    HillslopeConsumerBoundaryError, HillslopeDecompositionBoundaryError,
    HillslopeGrowthBoundaryError, HillslopePlActiveSlotResolutionError,
    hillslope_consumer_adapter_for_phase, required_hillslope_consumer_state_symbols,
    validate_hillslope_consumer_boundary,
};
pub use day_frame::{
    HillslopeDayFrame, HillslopeDayFrameError, HillslopeDayFrameIoEdgeScalars,
    HillslopeDayFrameMismatch, HillslopeDayFrameShadowReport, HillslopeLaneDenseState,
};
pub use direct_runtime::{
    DIRECT_PHASE_COUNT, DIRECT_R3A_INPUT_ACCOUNTING_SPAN, DIRECT_R3A_PHASE_SPAN_COUNT,
    DIRECT_R3B_PHASE_SPAN_COUNT, DIRECT_R3B_WATER_LEDGER_SPAN, DIRECT_R3C_LANE_TRANSFER_SPAN,
    DIRECT_R3C_PHASE_SPAN_COUNT, DIRECT_R4A_PHASE_SPAN_COUNT, DIRECT_R4A_RUNOFF_PARTITION_SPAN,
    DIRECT_R4B_PHASE_SPAN_COUNT, DIRECT_R4B_STORAGE_RECONCILIATION_SPAN,
    DIRECT_R4C_PHASE_SPAN_COUNT, DIRECT_R4C_STORAGE_INPUT_SPAN, DIRECT_R4D_DEEP_SEEPAGE_SPAN,
    DIRECT_R4D_PHASE_SPAN_COUNT, DIRECT_R4E_PHASE_SPAN_COUNT, DIRECT_R4E_SUBSURFACE_LOSS_SPAN,
    DIRECT_R4F_EVAPOTRANSPIRATION_SPAN, DIRECT_R4F_PHASE_SPAN_COUNT, DIRECT_R4G_PHASE_SPAN_COUNT,
    DIRECT_R4G_SNOW_COUPLING_SPAN, DIRECT_R4I_LIQUID_INPUT_SPAN, DIRECT_R4I_PHASE_SPAN_COUNT,
    DIRECT_R4J_PHASE_SPAN_COUNT, DIRECT_R4J_RUNON_CARRY_SPAN,
    DIRECT_R4K_INFILTRATION_DEPRESSION_SPAN, DIRECT_R4K_PHASE_SPAN_COUNT,
    DIRECT_R4L_PHASE_SPAN_COUNT, DIRECT_R4L_SATURATION_ADDBACK_SPAN, DIRECT_R4M_PERCOLATION_SPAN,
    DIRECT_R4M_PHASE_SPAN_COUNT, DIRECT_R4N_PHASE_SPAN_COUNT, DIRECT_R4N_ROOT_PHASE_SPAN_COUNT,
    DIRECT_R4N_ROOT_UPTAKE_SPAN, DIRECT_R4N_SURFACE_ET_SPAN, DIRECT_R4N_SURFACE_PHASE_SPAN_COUNT,
    DIRECT_R4O_PHASE_SPAN_COUNT, DIRECT_R4O_SUBSURFACE_SPAN,
    DIRECT_R4PQZ_HYDROLOGY_PROJECTION_SPAN, DIRECT_R4PQZ_PHASE_SPAN_COUNT,
    DIRECT_R5B_NORMALIZATION_PHASE_SPAN_COUNT, DIRECT_R5B_NORMALIZATION_SPAN,
    DIRECT_R5B_STORAGE_BOUNDS_PHASE_SPAN_COUNT, DIRECT_R5B_STORAGE_BOUNDS_SPAN,
    DIRECT_R5C_DECOMPOSITION_PHASE_SPAN_COUNT, DIRECT_R5C_DECOMPOSITION_SPAN,
    DIRECT_R5C_RESIDUE_PARTITION_PHASE_SPAN_COUNT, DIRECT_R5C_RESIDUE_PARTITION_SPAN,
    DIRECT_R5D_ANNUAL_GROWTH_PHASE_SPAN_COUNT, DIRECT_R5D_ANNUAL_GROWTH_SPAN,
    DIRECT_R5D_PERENNIAL_GROWTH_PHASE_SPAN_COUNT, DIRECT_R5D_PERENNIAL_GROWTH_SPAN,
    DIRECT_R7D6_EROSION_PHASE_SPAN_COUNT, DIRECT_R7D6_EROSION_SPAN,
    DIRECT_R7D6_PEAK_RUNOFF_PHASE_SPAN_COUNT, DIRECT_R7D6_PEAK_RUNOFF_SPAN,
    DIRECT_TRANSFER_HOUR_COUNT, DirectCanopyInterceptionInputs, DirectCanopyInterceptionState,
    DirectDayConstructorInputs, DirectDayForcing, DirectDayFrame, DirectDecompositionAction,
    DirectDecompositionActiveContext, DirectDecompositionDownstreamOperands,
    DirectDecompositionInputs, DirectDecompositionShadowProjection, DirectDecompositionSpanReport,
    DirectDecompositionState, DirectDeepSeepageDownstreamOperands, DirectDeepSeepageInputs,
    DirectDeepSeepageShadowProjection, DirectDeepSeepageSpanReport, DirectDeepSeepageState,
    DirectDownstreamOperands, DirectErod13Inputs, DirectErod13State, DirectErod14ClassInputs,
    DirectErod14ClassState, DirectErod14Inputs, DirectErod14State, DirectErosionDownstreamOperands,
    DirectErosionInputs, DirectErosionShadowProjection, DirectErosionSpanReport,
    DirectErosionState, DirectEvapotranspirationComputeDownstreamOperands,
    DirectEvapotranspirationComputeInputs, DirectEvapotranspirationComputeShadowProjection,
    DirectEvapotranspirationComputeSpanReport, DirectEvapotranspirationComputeState,
    DirectEvapotranspirationDownstreamOperands, DirectEvapotranspirationInputs,
    DirectEvapotranspirationPmetInputs, DirectEvapotranspirationShadowProjection,
    DirectEvapotranspirationSpanReport, DirectEvapotranspirationStageState,
    DirectEvapotranspirationState, DirectEvapotranspirationSurfaceDownstreamOperands,
    DirectEvapotranspirationSurfaceShadowProjection, DirectEvapotranspirationSurfaceSpanReport,
    DirectEvapotranspirationSurfaceState, DirectExecutionReport, DirectExecutorMode,
    DirectFrameExecutor, DirectFrostFineLayerCarry, DirectFrostLayerCarryProjection,
    DirectFrostLayerShadowCarry, DirectFrostRuntimeCarry, DirectGrowthAction,
    DirectGrowthActiveContext, DirectGrowthDownstreamOperands, DirectGrowthInputs,
    DirectGrowthShadowProjection, DirectGrowthSpanReport, DirectGrowthState,
    DirectGrowthStateSurface, DirectHydrologyProjectionDownstreamOperands,
    DirectHydrologyProjectionInputs, DirectHydrologyProjectionShadowProjection,
    DirectHydrologyProjectionSpanReport, DirectHydrologyProjectionState,
    DirectInfiltrationDepressionDownstreamOperands, DirectInfiltrationDepressionInputs,
    DirectInfiltrationDepressionShadowProjection, DirectInfiltrationDepressionSpanReport,
    DirectInfiltrationDepressionState, DirectInputAccountingState, DirectLaneConstructorInputs,
    DirectLaneFrame, DirectLaneTransferLedger, DirectLedgerDownstreamOperands,
    DirectLedgerShadowProjection, DirectLedgerSpanReport, DirectLiquidInputDownstreamOperands,
    DirectLiquidInputInputs, DirectLiquidInputShadowProjection, DirectLiquidInputSpanReport,
    DirectLiquidInputState, DirectNormalizationDownstreamOperands, DirectNormalizationInputs,
    DirectNormalizationShadowProjection, DirectNormalizationSpanReport, DirectNormalizationState,
    DirectPeakRunoffDownstreamOperands, DirectPeakRunoffInputs, DirectPeakRunoffShadowProjection,
    DirectPeakRunoffSpanReport, DirectPeakRunoffState, DirectPercolationDownstreamOperands,
    DirectPercolationInputs, DirectPercolationShadowProjection, DirectPercolationSpanReport,
    DirectPercolationState, DirectPhaseKind, DirectPhaseLifecycleStatus, DirectPhasePlan,
    DirectPhaseSpanReport, DirectPhaseStatusCount, DirectPhaseView, DirectPublicationCalendarDay,
    DirectPublicationClimateOperands, DirectPublicationDayInput, DirectPublicationDayRow,
    DirectPublicationErosionOperands, DirectPublicationEvaporationOperands,
    DirectPublicationExecution, DirectPublicationFrame, DirectPublicationInterceptionOperands,
    DirectPublicationLiquidInputOperands, DirectPublicationProfileOperands,
    DirectPublicationRunMetadata, DirectPublicationRunoffOperands,
    DirectPublicationStorageOperands, DirectPublicationSubsurfaceOperands,
    DirectPublicationTransferOperands, DirectResiduePartitionDownstreamOperands,
    DirectResiduePartitionInputs, DirectResiduePartitionShadowProjection,
    DirectResiduePartitionSpanReport, DirectResiduePartitionState, DirectRunConstructorInputs,
    DirectRunFrame, DirectRunIdentity, DirectRunPublicationFrame,
    DirectRunTransferDownstreamOperands, DirectRunTransferShadowProjection,
    DirectRunTransferSpanReport, DirectRunoffDownstreamOperands, DirectRunoffPartitionInputs,
    DirectRunoffPartitionSpanReport, DirectRunoffPartitionState, DirectRunoffShadowProjection,
    DirectRunonCarryDownstreamOperands, DirectRunonCarryInputs, DirectRunonCarryShadowProjection,
    DirectRunonCarrySpanReport, DirectRunonCarryState, DirectRuntimeAuditSnapshot,
    DirectRuntimeError, DirectSaturationAddbackDownstreamOperands, DirectSaturationAddbackInputs,
    DirectSaturationAddbackShadowProjection, DirectSaturationAddbackSpanReport,
    DirectSaturationAddbackState, DirectShadowProjection, DirectSnowCouplingDownstreamOperands,
    DirectSnowCouplingInputs, DirectSnowCouplingShadowProjection, DirectSnowCouplingSpanReport,
    DirectSnowCouplingState, DirectSnowRuntimeCarry, DirectStorageBoundsDownstreamOperands,
    DirectStorageBoundsInputs, DirectStorageBoundsShadowProjection, DirectStorageBoundsSpanReport,
    DirectStorageBoundsState, DirectStorageDownstreamOperands,
    DirectStorageInputDownstreamOperands, DirectStorageInputInputs,
    DirectStorageInputShadowProjection, DirectStorageInputSpanReport, DirectStorageInputState,
    DirectStorageReconciliationInputs, DirectStorageReconciliationSpanReport,
    DirectStorageReconciliationState, DirectStorageShadowProjection,
    DirectSubsurfaceComputeDownstreamOperands, DirectSubsurfaceComputeInputs,
    DirectSubsurfaceComputeShadowProjection, DirectSubsurfaceComputeSpanReport,
    DirectSubsurfaceComputeState, DirectSubsurfaceLayerInputs, DirectSubsurfaceLayerState,
    DirectSubsurfaceLossDownstreamOperands, DirectSubsurfaceLossInputs,
    DirectSubsurfaceLossShadowProjection, DirectSubsurfaceLossSpanReport,
    DirectSubsurfaceLossState, DirectTransferBuffers, DirectWaterLedgerState, DirectWaterState,
    DirectWb14HyetographInterval, DirectWb14InfiltrationProducerInputs,
    compute_direct_canopy_interception, direct_runtime_audit_snapshot,
    record_direct_runtime_compatibility_edge_invocation, reset_direct_runtime_audit_counters,
};
pub use hydrology::{
    DirectActiveFrostPartitionInputs, DirectActiveSnowPartitionInputs, DirectFrostControlInputs,
    DirectFrostFineLayerProjection, DirectFrostHourlyForcing, DirectFrostLayerInput,
    DirectFrostLayerProjection, DirectFrostLayerShadowProjection, DirectFrostPriorStateInput,
    DirectFrostThermalInputs, DirectSnowHourlyForcing, DirectSnowLiquidPartition,
    DirectWinterFrostComputeInputs, DirectWinterFrostPartitionOutcome,
    HillslopeHydrologyRoutingError, Wb11HydrologyKernel, Wb11HydrologyKernelGuardError,
};
pub use phase::HillslopePhase;
pub use runtime_inputs::{
    DIRECT_WINTER_HOURLY_FORCING_COUNT, DirectWinterHourlyContext, DirectWinterHourlyForcing,
};
pub use scheduler::{
    HillslopeKernelExecutionReport, HillslopeKernelPhaseReport, HillslopePhaseGraph,
    HillslopePhaseOutcome, HillslopePhaseScheduler, HillslopeSchedulerError,
    HillslopeSchedulerReport, HillslopeWritebackSurface, MOFE_TRANSFER_HOUR_COUNT,
    OfeLaneExecutionInput, OfeLaneExecutionReport, OfeLanePersistentState,
    OfeLanePersistentStateSequence, OfeLaneSequenceError, OfeLaneSequenceExecutionReport,
    PerOfeDailyWaterBalanceCollection, PerOfeDailyWaterBalanceError, PerOfeDailyWaterBalanceRecord,
    PhaseDependency, SchedulerOutcomeClass, TransferInput, TransferOutput,
    build_hillslope_hot_symbol_tables,
};
pub use winter_column::{
    DIRECT_WINTER_HOURS_PER_DAY, DirectFrostFineLayerState, DirectFrostLaneState,
    DirectFrostLayerShadowState, DirectSnowLaneState, DirectWinterColumnState,
    DirectWinterDayForcing, DirectWinterDayOutcome, DirectWinterFrostOutcome,
    DirectWinterPublicationOutcome, DirectWinterSnowOutcome, DirectWinterStorageOutcome,
};

pub(crate) use hydrology::{
    DecompositionPhaseDispatch, GrowthPhaseDispatch,
    decomposition_phase_dispatch_for_state_indexed, growth_phase_dispatch_for_state_indexed,
    hillslope_phase_class_for_phase, hydrology_phase_dispatch_for_phase, is_decomposition_phase,
    is_growth_phase,
};

#[cfg(test)]
pub(crate) use hydrology::{
    decomposition_phase_dispatch_for_state, growth_phase_dispatch_for_state,
};
