#![allow(clippy::missing_errors_doc)]

pub mod runtime_inputs;
pub mod schedule_export;

mod constants;
mod consumer_boundary;
mod hydrology;
mod phase;
mod scheduler;

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
    KernelRunResponse, KernelWritebackApplyResult, KernelWritebackPayload,
    MAX_CLIMATE_FORCING_SERIES_POINTS, WritebackDecisionOutcome, WritebackError, WritebackField,
    apply_kernel_writeback, evaluate_kernel_writeback,
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
pub use hydrology::{
    HillslopeHydrologyRoutingError, Wb11HydrologyKernel, Wb11HydrologyKernelGuardError,
};
pub use phase::HillslopePhase;
pub use scheduler::{
    HillslopeKernelExecutionReport, HillslopeKernelPhaseReport, HillslopePhaseGraph,
    HillslopePhaseOutcome, HillslopePhaseScheduler, HillslopeSchedulerError,
    HillslopeSchedulerReport, HillslopeWritebackSurface, MOFE_TRANSFER_HOUR_COUNT,
    OfeLaneExecutionInput, OfeLaneExecutionReport, OfeLanePersistentState,
    OfeLanePersistentStateSequence, OfeLaneSequenceError, OfeLaneSequenceExecutionReport,
    PerOfeDailyWaterBalanceCollection, PerOfeDailyWaterBalanceError, PerOfeDailyWaterBalanceRecord,
    PhaseDependency, SchedulerOutcomeClass, TransferInput, TransferOutput,
};

pub(crate) use hydrology::{
    DecompositionPhaseDispatch, GrowthPhaseDispatch, decomposition_phase_dispatch_for_state,
    growth_phase_dispatch_for_state, hillslope_phase_class_for_phase,
    hydrology_phase_dispatch_for_phase, is_decomposition_phase, is_growth_phase,
};
