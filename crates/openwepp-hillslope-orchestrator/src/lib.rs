#![allow(clippy::missing_errors_doc)]

#[cfg(test)]
extern crate self as openwepp_hillslope_orchestrator;

pub mod coupled_time_reference;
pub mod land_surface_energy_shadow;
pub mod runtime_inputs;
pub mod snow_stage3_open_boundary;
pub mod snow_stage3_terminal_handoff;
pub mod snow_stage3_v11_attachment;
pub mod v11_vegetation_consumer;
pub mod v9_real_consumer_shadow;
pub mod vegetation_diagnostic;
pub mod vegetation_energy_owner;
pub mod vegetation_real_hydrology_shadow;

mod constants;
mod direct_runtime;
mod hydrology;
pub mod ofe_routing;
mod winter_column;

#[cfg(test)]
mod tests;

use openwepp_kernel_contract::{HillslopeProductionFluxSymbol, HillslopeProductionStateSymbol};

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
    DIRECT_TRANSFER_HOUR_COUNT, DIRECT_WAVE1_GRID_POINTS, DirectCanopyInterceptionInputs,
    DirectCanopyInterceptionState, DirectDayConstructorInputs, DirectDayForcing, DirectDayFrame,
    DirectDecompositionAction, DirectDecompositionActiveContext,
    DirectDecompositionDownstreamOperands, DirectDecompositionInputs,
    DirectDecompositionShadowProjection, DirectDecompositionSpanReport, DirectDecompositionState,
    DirectDeepSeepageDownstreamOperands, DirectDeepSeepageInputs,
    DirectDeepSeepageShadowProjection, DirectDeepSeepageSpanReport, DirectDeepSeepageState,
    DirectDownstreamOperands, DirectErod13Inputs, DirectErod13State,
    DirectErosionDownstreamOperands, DirectErosionHydrographShapeAuthority, DirectErosionInputs,
    DirectErosionShadowProjection, DirectErosionSpanReport, DirectErosionState,
    DirectEvapotranspirationComputeDownstreamOperands, DirectEvapotranspirationComputeInputs,
    DirectEvapotranspirationComputeShadowProjection, DirectEvapotranspirationComputeSpanReport,
    DirectEvapotranspirationComputeState, DirectEvapotranspirationDownstreamOperands,
    DirectEvapotranspirationInputs, DirectEvapotranspirationPmetComputeInputs,
    DirectEvapotranspirationPmetInputs, DirectEvapotranspirationShadowProjection,
    DirectEvapotranspirationSpanReport, DirectEvapotranspirationStageState,
    DirectEvapotranspirationState, DirectEvapotranspirationSurfaceDownstreamOperands,
    DirectEvapotranspirationSurfaceShadowProjection, DirectEvapotranspirationSurfaceSpanReport,
    DirectEvapotranspirationSurfaceState, DirectEvapotranspirationTraceEvent,
    DirectExecutionReport, DirectExecutorMode, DirectFiveMinuteGenerationEvent,
    DirectFiveMinuteGenerationInterval, DirectFrameExecutor, DirectFrostFineLayerCarry,
    DirectFrostLayerCarryProjection, DirectFrostLayerShadowCarry, DirectFrostRuntimeCarry,
    DirectGroundwaterAuthority, DirectGroundwaterDayOutput, DirectGroundwaterRunState,
    DirectGrowthAction, DirectGrowthActiveContext, DirectGrowthDownstreamOperands,
    DirectGrowthInputs, DirectGrowthShadowProjection, DirectGrowthSpanReport, DirectGrowthState,
    DirectGrowthStateSurface, DirectHydrologyProjectionDownstreamOperands,
    DirectHydrologyProjectionInputs, DirectHydrologyProjectionShadowProjection,
    DirectHydrologyProjectionSpanReport, DirectHydrologyProjectionState,
    DirectInfiltrationDepressionDownstreamOperands, DirectInfiltrationDepressionInputs,
    DirectInfiltrationDepressionShadowProjection, DirectInfiltrationDepressionSpanReport,
    DirectInfiltrationDepressionState, DirectInputAccountingState, DirectLaneConstructorInputs,
    DirectLaneFrame, DirectLaneTransferLedger, DirectLanedActiveConfig,
    DirectLanedActiveDayRouting, DirectLanedActiveLaneConfig, DirectLanedActiveMeshPolicy,
    DirectLanedActiveMeshPolicySummary, DirectLanedActiveRunSummary,
    DirectLanedActiveStageLimiterTrace, DirectLanedActiveStepTraceRecord,
    DirectLanedActiveTraceDetail, DirectLanedActiveTraceDetailFilter, DirectLanedActiveTraceRecord,
    DirectLanedActiveTvdTrace, DirectLedgerDownstreamOperands, DirectLedgerShadowProjection,
    DirectLedgerSpanReport, DirectLiquidInputDownstreamOperands, DirectLiquidInputInputs,
    DirectLiquidInputShadowProjection, DirectLiquidInputSpanReport, DirectLiquidInputState,
    DirectNormalizationDownstreamOperands, DirectNormalizationInputs,
    DirectNormalizationShadowProjection, DirectNormalizationSpanReport, DirectNormalizationState,
    DirectPeakRunoffDownstreamOperands, DirectPeakRunoffShadowProjection,
    DirectPeakRunoffSpanReport, DirectPeakRunoffState, DirectPercolationDownstreamOperands,
    DirectPercolationInputs, DirectPercolationShadowProjection, DirectPercolationSpanReport,
    DirectPercolationState, DirectPercolationTraceEvent, DirectPhaseKind,
    DirectPhaseLifecycleStatus, DirectPhasePlan, DirectPhaseSpanReport, DirectPhaseStatusCount,
    DirectPhaseView, DirectPublicationBatchExecution, DirectPublicationCalendarDay,
    DirectPublicationClimateOperands, DirectPublicationDayInput, DirectPublicationDayRow,
    DirectPublicationErosionOperands, DirectPublicationEvaporationOperands,
    DirectPublicationExecution, DirectPublicationFrame, DirectPublicationInterceptionOperands,
    DirectPublicationLiquidInputOperands, DirectPublicationProfileOperands,
    DirectPublicationRunMetadata, DirectPublicationRunoffOperands,
    DirectPublicationStorageOperands, DirectPublicationSubsurfaceOperands,
    DirectPublicationTransferOperands, DirectPublicationWaterTemperatureOperands,
    DirectResiduePartitionDownstreamOperands, DirectResiduePartitionInputs,
    DirectResiduePartitionShadowProjection, DirectResiduePartitionSpanReport,
    DirectResiduePartitionState, DirectRunConstructorInputs, DirectRunFrame, DirectRunIdentity,
    DirectRunPublicationFrame, DirectRunTransferDownstreamOperands,
    DirectRunTransferShadowProjection, DirectRunTransferSpanReport, DirectRunoffDownstreamOperands,
    DirectRunoffPartitionInputs, DirectRunoffPartitionSpanReport, DirectRunoffPartitionState,
    DirectRunoffShadowProjection, DirectRunonCarryDownstreamOperands, DirectRunonCarryInputs,
    DirectRunonCarryShadowProjection, DirectRunonCarrySpanReport, DirectRunonCarryState,
    DirectRuntimeAuditSnapshot, DirectRuntimeError, DirectSaturationAddbackDownstreamOperands,
    DirectSaturationAddbackInputs, DirectSaturationAddbackShadowProjection,
    DirectSaturationAddbackSpanReport, DirectSaturationAddbackState, DirectShadowProjection,
    DirectSnowCouplingDownstreamOperands, DirectSnowCouplingInputs,
    DirectSnowCouplingShadowProjection, DirectSnowCouplingSpanReport, DirectSnowCouplingState,
    DirectSnowRuntimeCarry, DirectStorageBoundsDownstreamOperands, DirectStorageBoundsInputs,
    DirectStorageBoundsShadowProjection, DirectStorageBoundsSpanReport, DirectStorageBoundsState,
    DirectStorageDownstreamOperands, DirectStorageInputDownstreamOperands,
    DirectStorageInputInputs, DirectStorageInputShadowProjection, DirectStorageInputSpanReport,
    DirectStorageInputState, DirectStorageReconciliationInputs,
    DirectStorageReconciliationSpanReport, DirectStorageReconciliationState,
    DirectStorageShadowProjection, DirectStreamingPublicationExecution,
    DirectSubsurfaceComputeDownstreamOperands, DirectSubsurfaceComputeInputs,
    DirectSubsurfaceComputeShadowProjection, DirectSubsurfaceComputeSpanReport,
    DirectSubsurfaceComputeState, DirectSubsurfaceLayerInputs, DirectSubsurfaceLayerState,
    DirectSubsurfaceLossDownstreamOperands, DirectSubsurfaceLossInputs,
    DirectSubsurfaceLossShadowProjection, DirectSubsurfaceLossSpanReport,
    DirectSubsurfaceLossState, DirectSubsurfaceSaturationTraceEvent, DirectTransferBuffers,
    DirectWaterLedgerState, DirectWaterState, DirectWave1ContinuityInputs,
    DirectWave1ContinuityState, DirectWave1SlopeSegment, DirectWb14HyetographInterval,
    DirectWb14InfiltrationProducerInputs, LANED_ACTIVE_MAX_DT_S, Wave1InterOfeContinuity,
    Wave1ShearClassification, Wave1ShearRegime, compute_direct_canopy_interception,
    compute_direct_wave1_continuity, compute_direct_wave1_continuity_quantum,
    derive_wave1_slope_segments, direct_native_canopy_height_m, direct_runtime_audit_snapshot,
    erosion_sheart, record_direct_runtime_compatibility_edge_invocation,
    record_direct_runtime_ksatadj_effective_conductivity_evaluation,
    reset_direct_runtime_audit_counters, residue_ground_cover_fraction,
    validate_direct_native_canopy_height_parameters, wave1_classifier_shear,
    wave1_day_routes_sediment, wave1_depc, wave1_depend, wave1_depeqs,
    wave1_quantum_is_hydraulically_active, wave1_runge_step, wave1_xcrit,
    wp2_frost_pair_trace_path, write_wp2_frost_pair_trace,
};

pub use snow_stage3_v11_attachment::{
    DirectSnowStage3V11AttachmentError, DirectSnowStage3V11CommittedState,
    DirectSnowStage3V11ParentCandidate, DirectSnowStage3V11ParentReceipt,
    DirectSnowStage3V11PreparedDay, DirectSnowStage3V11PreparedSupport,
    DirectSnowStage3V11ShadowAttachment, DirectSnowStage3V11StaticContext,
    DirectSnowStage3V11TerminalParcel, DirectSnowStage3V11TerminalReceipt, PreparedStage3V11DayV1,
    PreparedStage3V11SupportIdentityV1, PreparedStage3V11SupportV1, STAGE3_V11_DAY_NS,
    STAGE3_V11_PARENT_SUPPORT_COUNT, STAGE3_V11_PARENT_SUPPORT_NS, Stage3LaneLifecycleV1,
    Stage3ParentAtmosphericReceiptV1, ValidatedPreparedStage3V11DayV1,
};

pub use direct_runtime::{
    DirectCanopyLiquidRelease, DirectIngressAmount, DirectOfeWb14Parameters,
    DirectOpenLiquidIngressParcel, DirectSurfaceLiquidClosureOperands,
    DirectSurfaceLiquidIngressCandidate, DirectSurfaceLiquidIngressInput,
    DirectSurfaceLiquidIngressLedger, DirectSurfaceLiquidParcelClosureOperands,
    DirectSurfaceLiquidParcelKind, DirectSurfaceLiquidParcelReceipt,
    DirectSurfaceLiquidReceiptDisposition, DirectSurfaceLiquidReceiptRecipient,
    DirectSurfaceLiquidStoreClosureOperands, DirectTileGroundIngress,
    DirectWb14CoupledChildBindingV1, execute_surface_liquid_ingress,
};

pub use direct_runtime::{
    DirectErosionConsolidationCarry, ErosionAdjustmentFactors, ErosionAdjustmentInputs,
    ErosionConsolidationBaselines, ErosionConsolidationInputs, ErosionFrostInputs,
    ErosionFrostRegime, ErosionIfrostCarry, ErosionRfcumInputs, advance_erosion_consolidation,
    erosion_adjustment_factors, erosion_consolidation_baselines, resolve_erosion_frost_regime,
};
pub use direct_runtime::{
    DirectErosionInflowIntake, DirectErosionRuntimeCarry, DirectWave1DailyState,
    DirectWave1OperandSeed, Wave1InflowOperands, assemble_wave1_continuity_inputs,
    assemble_wave1_continuity_inputs_quantum,
};
pub use direct_runtime::{
    DirectGroundIngressMode, DirectSurfaceLiquidArbitration, DirectSurfaceLiquidConfiguration,
    DirectSurfaceLiquidConfigurationRecord, DirectSurfaceLiquidContinuationState,
    DirectSurfaceLiquidError, DirectSurfaceLiquidErrorCode, DirectSurfaceLiquidErrorContext,
    DirectSurfaceLiquidFailure, DirectSurfaceLiquidOfeBinding, DirectSurfaceLiquidOwnedState,
    DirectSurfaceLiquidPhase, DirectSurfaceLiquidResourceCandidate,
    DirectSurfaceLiquidRollbackHashes, DirectSurfaceLiquidStateRecord, DirectSurfaceLiquidStoreKey,
    apply_surface_liquid_resource_phase, authorize_surface_liquid_withdrawals,
};
pub use direct_runtime::{
    DirectSnowStage3SealedForcing, DirectSnowStage3ShadowAttachment,
    DirectSnowStage3ShadowConfiguration, DirectSnowStage3ShadowRestartV1,
    DirectSnowStage3StagedSurfaceReceipt,
};
pub use direct_runtime::{
    EROSION_PARTICLE_CLASS_COUNT, ErosionEffectiveIntensity, ErosionExcessInterval,
    ErosionParticleClass, ErosionRillCoverInputs, ErosionRillHydraulics, ErosionShearSlopes,
    ErosionTextureInputs, ErosionTransportCoefficients, erosion_detinr,
    erosion_effective_intensity, erosion_effective_particle, erosion_falvel,
    erosion_interrill_delivery_ratio, erosion_particle_composition, erosion_rill_hydraulics,
    erosion_shield, erosion_surface_soil_ssa, erosion_transport_coefficients, erosion_trcoef,
    erosion_yalin, erosion_yalin_with_class_shares,
};
pub use direct_runtime::{
    Wave1EnrichmentInputs, Wave1EnrichmentRegionOperands, Wave1EnrichmentState,
};
pub use hydrology::{
    DirectActiveFrostPartitionInputs, DirectActiveSnowPartitionInputs, DirectFrostControlInputs,
    DirectFrostFineLayerProjection, DirectFrostHourlyForcing, DirectFrostLayerInput,
    DirectFrostLayerProjection, DirectFrostLayerShadowProjection, DirectFrostPriorStateInput,
    DirectFrostThermalInputs, DirectKsatadjEffectiveConductivityInputs,
    DirectKsatadjEffectiveConductivityOutcome, DirectKsatadjLayerInputs,
    DirectSnowAccumulationMeltDiagnostics, DirectSnowDiagnosticCapture, DirectSnowHourlyForcing,
    DirectSnowLiquidDispositionLedger, DirectSnowLiquidPartition,
    DirectSnowMassTransitionLedgerError, DirectSnowMassTransitionLedgers,
    DirectSnowMeltHourDiagnostics, DirectSnowSolidToLiquidLedger, DirectSnowStage3Diagnostics,
    DirectSnowStage3EvaluationDiagnostics, DirectSnowStage3EvaluationError,
    DirectSnowStage3EvaluationHourDiagnostics, DirectSnowStage3EvaluationResult,
    DirectSnowStage3EvaluationWithReconciliationResult, DirectSnowStage3OperatorReconciliation,
    DirectSnowStage3Outcome, DirectSnowStage3PersistentDayResult, DirectSnowStage3PersistentState,
    DirectSnowStage3ReconciliationHourStatus, DirectSnowStage3ReconciliationTuple,
    DirectSnowSurfaceEnergyHourDiagnostics, DirectSnowSurfaceEnergyOptions,
    DirectSnowTerminalEventModel, DirectSnowTerminalEventRequest, DirectSnowTerminalEventResult,
    DirectSnowTurbulentGeometry, DirectSnowVerboseDiagnostics, DirectWinterFrostComputeInputs,
    DirectWinterFrostPartitionOutcome, FrostSeasonalTemperatureCurve,
    STURM1995_CDM_CRITICAL_TEMPERATURE_C, STURM1995_EPHEMERAL_CDM_THRESHOLD_C_MONTH,
    STURM1995_HIGH_LOW_CDM_THRESHOLD_C_MONTH, STURM1995_HIGH_PRECIP_SPR_THRESHOLD_MM_DAY,
    STURM1995_HIGH_WIND_MIN_M_S, STURM1995_LOW_WIND_MAX_M_S, SnowAlbedoError, SnowAlbedoModel,
    SnowAlbedoState, SnowAlbedoUpdateInputs, SnowAlbedoUpdateOutcome, SnowClimateClass,
    SnowDensityCompactionConstants, SnowDensityError, SnowDensityModel,
    SnowDensityProcessDiagnostics, SnowDensityRuntimeInputs, SnowDensityRuntimeOutcome,
    SnowLayerAggregateMismatchError, SnowMeltModel, SnowStage3ConductivityError,
    SnowStage3EvaluationOperator, SnowStage3LiquidRoutingModel, SnowStage3TurbulentTransferError,
    SnowSurfaceLongwaveModel, SnowSurfaceSublimationModel, SnowTerminalNumericsFailure,
    Stage3SurfaceStateV1, Sturm1995ClimateClassAssignmentError, Sturm1995ClimateNormals,
    Sturm2010DensityParameters, Wb11HydrologyKernel, Wb11HydrologyKernelGuardError,
    snow_density_compaction_v1_constants, snow_density_shallow_guard_v1_constants,
    snow_density_spring_densification_v1_constants, sturm1995_climate_class_from_normals,
    sturm2010_bulk_density_kg_m3, sturm2010_density_parameters_for_class, update_snow_albedo_state,
    update_snow_density_runtime_state,
};
pub use runtime_inputs::{
    DIRECT_WINTER_HOURLY_FORCING_COUNT, DirectWinterHourlyContext, DirectWinterHourlyForcing,
    ForestLanuseReconciliationError, SnowPhasePartitionModel, reconcile_forest_lanuse_authority,
};
pub use winter_column::{
    DIRECT_WINTER_HOURS_PER_DAY, DirectFrostFineLayerState, DirectFrostLaneState,
    DirectFrostLayerShadowState, DirectSnowLaneState, DirectSnowLayerState,
    DirectWinterColumnState, DirectWinterDayForcing, DirectWinterDayOutcome,
    DirectWinterFrostOutcome, DirectWinterPublicationOutcome, DirectWinterSnowOutcome,
    DirectWinterStorageOutcome,
};
