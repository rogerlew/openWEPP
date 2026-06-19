//! Split test surface for hillslope orchestrator.

#![allow(unused_imports)]

use std::cell::Cell;
use std::collections::BTreeMap;

use openwepp_kernel_contract::{
    BoundarySymbol, BoundaryValue, HillslopeAnnualDecompositionAction,
    HillslopeAnnualDecompositionControl, HillslopeAnnualGrowthAction, HillslopeAnnualGrowthControl,
    HillslopeConsumerAdapter, HillslopeDecompositionKernelContext,
    HillslopeDecompositionManagementClass, HillslopeDecompositionTransitionControl,
    HillslopeDecompositionTransitionPayload, HillslopeGrowthKernelContext,
    HillslopeGrowthManagementClass, HillslopeGrowthStateSurface, HillslopeGrowthTransitionControl,
    HillslopeGrowthTransitionPayload, HillslopeKernel, HillslopeKernelPhaseClass,
    HillslopeKernelRequest, HillslopePerennialDecompositionAction,
    HillslopePerennialDecompositionControl, HillslopePerennialGrowthAction,
    HillslopePerennialGrowthControl, HillslopeProductionStateSymbol, IndexedKernelWritebackPayload,
    IndexedWritebackField, IndexedWritebackSurface, KernelRunResponse, KernelWritebackPayload,
    SymbolRegistry, WRITEBACK_REJECT_NON_FINITE_MESSAGE_ID, WritebackDecisionOutcome,
    WritebackField,
};
use openwepp_sim_contract::status::{BoundaryClass, SimulationPhase, StatusClassification};
use openwepp_topology::{parse_topology_fixture_str, validate_pre_execution_topology};

use crate::schedule_export::{
    ScheduleDiagnostic, ScheduleExport, diff_schedule_json, render_schedule_diff,
    validate_hillslope_schedule_graph,
};
use crate::{
    HillslopeDayFrame, HillslopeLaneDenseState, Wb11HydrologyKernel, Wb11HydrologyKernelGuardError,
    build_hillslope_hot_symbol_tables,
    consumer_boundary::{
        HillslopeConsumerBoundaryError, HillslopeDecompositionBoundaryError,
        HillslopeGrowthBoundaryError, HillslopePlActiveSlotResolutionError,
        hillslope_consumer_adapter_for_phase, required_hillslope_consumer_state_symbols,
        validate_hillslope_consumer_boundary,
    },
    phase::HillslopePhase,
    scheduler::{
        HillslopeKernelExecutionReport, HillslopeKernelPhaseReport, HillslopePhaseGraph,
        HillslopePhaseOutcome, HillslopePhaseScheduler, HillslopeSchedulerError,
        HillslopeSchedulerReport, HillslopeWritebackSurface, OfeLaneExecutionInput,
        OfeLanePersistentState, OfeLanePersistentStateSequence, OfeLaneSequenceError,
        SchedulerOutcomeClass,
    },
};
use crate::{
    decomposition_phase_dispatch_for_state, growth_phase_dispatch_for_state,
    hillslope_phase_class_for_phase, hydrology_phase_dispatch_for_phase, is_decomposition_phase,
    is_growth_phase,
};

mod boundaries;
mod day_frame;
mod fixtures;
mod growth;
mod hydrology;
mod phase;
mod schedule_export;
mod writeback;
