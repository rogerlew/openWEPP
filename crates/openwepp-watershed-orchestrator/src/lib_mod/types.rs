use std::collections::BTreeMap;
use std::error::Error;
use std::fmt;

use openwepp_kernel_contract::{
    BoundarySymbol, BoundaryValue, KernelWritebackApplyResult, WritebackDecisionOutcome,
    WritebackError,
};
use openwepp_sim_contract::status::{SimulationStatus, StatusClassification, StatusError};
use openwepp_topology::{TopologyNodeKey, TopologyValidationError};

/// Success message id for a complete watershed dispatch schedule.
pub const MESSAGE_DISPATCH_OK: &str = "WATERSHED-DISPATCH-OK-001";
/// Failure message id when topology preconditions are not satisfied.
pub const MESSAGE_PRECONDITION_FAILED: &str = "WATERSHED-DISPATCH-E-PRECONDITION-TOPOLOGY";
/// Failure message id when a dispatch dependency is missing from the graph.
pub const MESSAGE_MISSING_DEPENDENCY: &str = "WATERSHED-DISPATCH-E-MISSING-DEPENDENCY";
/// Failure message id when channel/impoundment dependencies contain a cycle.
pub const MESSAGE_CYCLE_DETECTED: &str = "WATERSHED-DISPATCH-E-CYCLE-DETECTED";

/// Diagnostic code for watershed dispatch failures.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DispatchDiagnosticCode {
    TopologyPreconditionFailed,
    MissingDependency,
    DependencyCycleDetected,
}

impl DispatchDiagnosticCode {
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::TopologyPreconditionFailed => "TOPOLOGY_PRECONDITION_FAILED",
            Self::MissingDependency => "MISSING_DEPENDENCY",
            Self::DependencyCycleDetected => "DEPENDENCY_CYCLE_DETECTED",
        }
    }
}

/// Typed diagnostic record emitted by scheduler execution.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DispatchDiagnostic {
    pub code: DispatchDiagnosticCode,
    pub message_id: String,
    pub detail: String,
}

impl DispatchDiagnostic {
    #[must_use]
    pub fn new(
        code: DispatchDiagnosticCode,
        message_id: impl Into<String>,
        detail: impl Into<String>,
    ) -> Self {
        Self {
            code,
            message_id: message_id.into(),
            detail: detail.into(),
        }
    }
}

/// One deterministic scheduler step for watershed dispatch execution.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DispatchStep {
    pub sequence_index: usize,
    pub node: TopologyNodeKey,
    pub dependency_nodes: Vec<TopologyNodeKey>,
    pub contributor_hillslopes: Vec<u32>,
    pub status: SimulationStatus,
}

/// Full scheduler report with precondition and dispatch outcomes.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct WatershedDispatchReport {
    pub precondition_status: SimulationStatus,
    pub dispatch_status: SimulationStatus,
    pub steps: Vec<DispatchStep>,
    pub diagnostics: Vec<DispatchDiagnostic>,
}

impl WatershedDispatchReport {
    #[must_use]
    pub fn is_success(&self) -> bool {
        self.precondition_status.classification() != StatusClassification::Failure
            && self.dispatch_status.classification() != StatusClassification::Failure
    }
}

/// Mutable state/flux maps owned by the watershed orchestrator.
#[derive(Debug, Clone, Default, PartialEq)]
pub struct WatershedWritebackSurface {
    pub state_surface: BTreeMap<BoundarySymbol, BoundaryValue>,
    pub flux_surface: BTreeMap<BoundarySymbol, BoundaryValue>,
}

/// Per-step watershed kernel/writeback execution evidence.
#[derive(Debug, Clone, PartialEq)]
pub struct WatershedKernelStepReport {
    pub step: DispatchStep,
    pub kernel_status: SimulationStatus,
    pub decision_outcome: WritebackDecisionOutcome,
    pub decision_status: SimulationStatus,
    pub apply_result: Option<KernelWritebackApplyResult>,
}

/// Kernel-integrated watershed execution report.
#[derive(Debug, Clone, PartialEq)]
pub struct WatershedKernelExecutionReport {
    pub dispatch_report: WatershedDispatchReport,
    pub step_reports: Vec<WatershedKernelStepReport>,
    pub writeback_surface: WatershedWritebackSurface,
}

/// Per-step frame-native watershed execution evidence.
#[derive(Debug, Clone, PartialEq)]
pub struct WatershedFrameStepReport {
    pub step: DispatchStep,
    pub kernel_status: SimulationStatus,
    pub routed_state_applied: bool,
}

/// Frame-native watershed execution report.
#[derive(Debug, Clone, PartialEq)]
pub struct WatershedFrameExecutionReport {
    pub dispatch_report: WatershedDispatchReport,
    pub step_reports: Vec<WatershedFrameStepReport>,
}

/// Error surface for watershed dispatch scheduler orchestration.
#[derive(Debug)]
pub enum WatershedDispatchError {
    Status(StatusError),
    TopologyValidation(TopologyValidationError),
    Writeback(WritebackError),
}

impl fmt::Display for WatershedDispatchError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Status(source) => write!(f, "failed constructing dispatch status: {source}"),
            Self::TopologyValidation(source) => {
                write!(
                    f,
                    "failed constructing topology validation gate report: {source}"
                )
            }
            Self::Writeback(source) => {
                write!(f, "failed applying watershed kernel writeback: {source}")
            }
        }
    }
}

impl Error for WatershedDispatchError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            Self::Status(source) => Some(source),
            Self::TopologyValidation(source) => Some(source),
            Self::Writeback(source) => Some(source),
        }
    }
}

impl From<StatusError> for WatershedDispatchError {
    fn from(value: StatusError) -> Self {
        Self::Status(value)
    }
}

impl From<TopologyValidationError> for WatershedDispatchError {
    fn from(value: TopologyValidationError) -> Self {
        Self::TopologyValidation(value)
    }
}

impl From<WritebackError> for WatershedDispatchError {
    fn from(value: WritebackError) -> Self {
        Self::Writeback(value)
    }
}
