#![allow(clippy::missing_errors_doc)]

use std::collections::{BTreeMap, BTreeSet};
use std::error::Error;
use std::fmt;

use openwepp_sim_contract::closure::ClosureViolation;
use openwepp_sim_contract::status::{
    BoundaryClass, ClampClass, SimulationPhase, SimulationStatus, StatusClassification, StatusError,
};
use openwepp_topology::TopologyValidationReport;

const PHASE_COUNT: usize = 9;

/// Deterministic hillslope scheduler phases.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Ord, PartialOrd)]
pub enum HillslopePhase {
    Normalization,
    StorageBounds,
    Evapotranspiration,
    PercolationDeepSeepage,
    LateralTransfer,
    Drainage,
    RunoffReconciliation,
    StorageReconciliation,
    ClosureDiagnostics,
}

impl HillslopePhase {
    const ORDERED: [Self; PHASE_COUNT] = [
        Self::Normalization,
        Self::StorageBounds,
        Self::Evapotranspiration,
        Self::PercolationDeepSeepage,
        Self::LateralTransfer,
        Self::Drainage,
        Self::RunoffReconciliation,
        Self::StorageReconciliation,
        Self::ClosureDiagnostics,
    ];

    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Normalization => "normalization",
            Self::StorageBounds => "storage_bounds",
            Self::Evapotranspiration => "evapotranspiration",
            Self::PercolationDeepSeepage => "percolation_deep_seepage",
            Self::LateralTransfer => "lateral_transfer",
            Self::Drainage => "drainage",
            Self::RunoffReconciliation => "runoff_reconciliation",
            Self::StorageReconciliation => "storage_reconciliation",
            Self::ClosureDiagnostics => "closure_diagnostics",
        }
    }

    #[must_use]
    pub const fn rank(self) -> usize {
        match self {
            Self::Normalization => 0,
            Self::StorageBounds => 1,
            Self::Evapotranspiration => 2,
            Self::PercolationDeepSeepage => 3,
            Self::LateralTransfer => 4,
            Self::Drainage => 5,
            Self::RunoffReconciliation => 6,
            Self::StorageReconciliation => 7,
            Self::ClosureDiagnostics => 8,
        }
    }

    #[must_use]
    pub const fn ok_message_id(self) -> &'static str {
        match self {
            Self::Normalization => "HSCHED-PHASE-OK-001",
            Self::StorageBounds => "HSCHED-PHASE-OK-002",
            Self::Evapotranspiration => "HSCHED-PHASE-OK-003",
            Self::PercolationDeepSeepage => "HSCHED-PHASE-OK-004",
            Self::LateralTransfer => "HSCHED-PHASE-OK-005",
            Self::Drainage => "HSCHED-PHASE-OK-006",
            Self::RunoffReconciliation => "HSCHED-PHASE-OK-007",
            Self::StorageReconciliation => "HSCHED-PHASE-OK-008",
            Self::ClosureDiagnostics => "HSCHED-PHASE-OK-009",
        }
    }
}

/// Explicit scheduler dependency edge.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct PhaseDependency {
    pub phase: HillslopePhase,
    pub depends_on: HillslopePhase,
}

/// Deterministic dependency graph for hillslope phase ordering.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct HillslopePhaseGraph {
    dependencies: BTreeMap<HillslopePhase, Vec<HillslopePhase>>,
}

impl HillslopePhaseGraph {
    /// Build the canonical ARCH05 deterministic graph.
    #[must_use]
    pub fn canonical() -> Self {
        let mut dependencies: BTreeMap<HillslopePhase, Vec<HillslopePhase>> = BTreeMap::new();
        for phase in HillslopePhase::ORDERED {
            dependencies.insert(phase, Vec::new());
        }

        for edge in Self::canonical_dependencies() {
            dependencies
                .entry(edge.phase)
                .or_default()
                .push(edge.depends_on);
        }

        for deps in dependencies.values_mut() {
            deps.sort_by_key(|phase| phase.rank());
            deps.dedup();
        }

        Self { dependencies }
    }

    #[must_use]
    pub fn dependencies_for(&self, phase: HillslopePhase) -> &[HillslopePhase] {
        self.dependencies
            .get(&phase)
            .map_or(&[] as &[HillslopePhase], Vec::as_slice)
    }

    #[must_use]
    pub const fn canonical_order() -> [HillslopePhase; PHASE_COUNT] {
        HillslopePhase::ORDERED
    }

    #[must_use]
    pub fn dependency_edges(&self) -> Vec<PhaseDependency> {
        let mut edges = Vec::new();

        for phase in HillslopePhase::ORDERED {
            if let Some(deps) = self.dependencies.get(&phase) {
                for dependency in deps {
                    edges.push(PhaseDependency {
                        phase,
                        depends_on: *dependency,
                    });
                }
            }
        }

        edges
    }

    #[must_use]
    pub fn topological_order(&self) -> Option<Vec<HillslopePhase>> {
        let mut indegree: BTreeMap<HillslopePhase, usize> = BTreeMap::new();
        let mut adjacency: BTreeMap<HillslopePhase, BTreeSet<HillslopePhase>> = BTreeMap::new();

        for phase in HillslopePhase::ORDERED {
            indegree.insert(phase, 0);
            adjacency.insert(phase, BTreeSet::new());
        }

        for phase in HillslopePhase::ORDERED {
            for dependency in self.dependencies_for(phase) {
                let value = indegree.get_mut(&phase)?;
                *value += 1;

                adjacency.entry(*dependency).or_default().insert(phase);
            }
        }

        let mut ready: Vec<HillslopePhase> = HillslopePhase::ORDERED
            .iter()
            .copied()
            .filter(|phase| indegree.get(phase).copied().unwrap_or(0) == 0)
            .collect();
        let mut order = Vec::with_capacity(PHASE_COUNT);

        while !ready.is_empty() {
            ready.sort_by_key(|phase| phase.rank());
            let phase = ready.remove(0);
            order.push(phase);

            if let Some(neighbors) = adjacency.get(&phase) {
                for neighbor in neighbors {
                    let count = indegree.get_mut(neighbor)?;

                    if *count == 0 {
                        return None;
                    }

                    *count -= 1;
                    if *count == 0 {
                        ready.push(*neighbor);
                    }
                }
            }
        }

        if order.len() == PHASE_COUNT {
            Some(order)
        } else {
            None
        }
    }

    #[must_use]
    const fn canonical_dependencies() -> [PhaseDependency; PHASE_COUNT - 1] {
        [
            PhaseDependency {
                phase: HillslopePhase::StorageBounds,
                depends_on: HillslopePhase::Normalization,
            },
            PhaseDependency {
                phase: HillslopePhase::Evapotranspiration,
                depends_on: HillslopePhase::StorageBounds,
            },
            PhaseDependency {
                phase: HillslopePhase::PercolationDeepSeepage,
                depends_on: HillslopePhase::Evapotranspiration,
            },
            PhaseDependency {
                phase: HillslopePhase::LateralTransfer,
                depends_on: HillslopePhase::PercolationDeepSeepage,
            },
            PhaseDependency {
                phase: HillslopePhase::Drainage,
                depends_on: HillslopePhase::LateralTransfer,
            },
            PhaseDependency {
                phase: HillslopePhase::RunoffReconciliation,
                depends_on: HillslopePhase::Drainage,
            },
            PhaseDependency {
                phase: HillslopePhase::StorageReconciliation,
                depends_on: HillslopePhase::RunoffReconciliation,
            },
            PhaseDependency {
                phase: HillslopePhase::ClosureDiagnostics,
                depends_on: HillslopePhase::StorageReconciliation,
            },
        ]
    }
}

impl Default for HillslopePhaseGraph {
    fn default() -> Self {
        Self::canonical()
    }
}

/// One executed phase and its typed status surface.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct HillslopePhaseOutcome {
    pub phase: HillslopePhase,
    pub status: SimulationStatus,
}

/// Coarse scheduler completion class for deterministic decision routing.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SchedulerOutcomeClass {
    Completed,
    TopologyPreconditionFailed,
    PhaseFailure,
    SchedulerInvariantFailure,
}

/// Scheduler execution report.
#[derive(Debug, Clone)]
pub struct HillslopeSchedulerReport {
    pub outcome_class: SchedulerOutcomeClass,
    pub topology_precondition_status: SimulationStatus,
    pub scheduler_status: SimulationStatus,
    pub ordered_phases: Vec<HillslopePhase>,
    pub outcomes: Vec<HillslopePhaseOutcome>,
    pub precondition_violations: Vec<ClosureViolation>,
    pub halted_phase: Option<HillslopePhase>,
}

impl HillslopeSchedulerReport {
    #[must_use]
    pub fn is_success(&self) -> bool {
        self.outcome_class == SchedulerOutcomeClass::Completed
            && self.scheduler_status.classification() != StatusClassification::Failure
    }

    #[must_use]
    pub fn executed_phases(&self) -> Vec<HillslopePhase> {
        self.outcomes.iter().map(|outcome| outcome.phase).collect()
    }
}

/// Scheduler construction/operation error.
#[derive(Debug)]
pub enum HillslopeSchedulerError {
    Status(StatusError),
}

impl fmt::Display for HillslopeSchedulerError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Status(source) => write!(f, "status construction failed: {source}"),
        }
    }
}

impl Error for HillslopeSchedulerError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            Self::Status(source) => Some(source),
        }
    }
}

impl From<StatusError> for HillslopeSchedulerError {
    fn from(value: StatusError) -> Self {
        Self::Status(value)
    }
}

/// Deterministic hillslope scheduler.
#[derive(Debug, Clone)]
pub struct HillslopePhaseScheduler {
    graph: HillslopePhaseGraph,
}

impl HillslopePhaseScheduler {
    #[must_use]
    pub fn canonical() -> Self {
        Self {
            graph: HillslopePhaseGraph::canonical(),
        }
    }

    #[must_use]
    pub fn new(graph: HillslopePhaseGraph) -> Self {
        Self { graph }
    }

    #[must_use]
    pub fn graph(&self) -> &HillslopePhaseGraph {
        &self.graph
    }

    /// Build a nominal phase status for deterministic test/driver defaults.
    pub fn nominal_phase_status(phase: HillslopePhase) -> Result<SimulationStatus, StatusError> {
        SimulationStatus::ok(SimulationPhase::HillslopeKernel, phase.ok_message_id())
    }

    /// Execute deterministic phase scheduling with topology precondition gating.
    #[allow(clippy::too_many_lines)]
    pub fn execute_with<F>(
        &self,
        topology_report: &TopologyValidationReport,
        mut phase_executor: F,
    ) -> Result<HillslopeSchedulerReport, HillslopeSchedulerError>
    where
        F: FnMut(HillslopePhase) -> SimulationStatus,
    {
        if topology_report.status.classification() == StatusClassification::Failure {
            return Ok(HillslopeSchedulerReport {
                outcome_class: SchedulerOutcomeClass::TopologyPreconditionFailed,
                topology_precondition_status: topology_report.status.clone(),
                scheduler_status: topology_report.status.clone(),
                ordered_phases: Vec::new(),
                outcomes: Vec::new(),
                precondition_violations: topology_report.violations.clone(),
                halted_phase: None,
            });
        }

        if !topology_report.violations.is_empty() {
            let status = SimulationStatus::failure(
                SimulationPhase::PreExecutionValidation,
                true,
                false,
                BoundaryClass::TopologyInvalid,
                "HSCHED-E-TOPOLOGY-PRECONDITION",
            )?;

            return Ok(HillslopeSchedulerReport {
                outcome_class: SchedulerOutcomeClass::TopologyPreconditionFailed,
                topology_precondition_status: topology_report.status.clone(),
                scheduler_status: status,
                ordered_phases: Vec::new(),
                outcomes: Vec::new(),
                precondition_violations: topology_report.violations.clone(),
                halted_phase: None,
            });
        }

        let Some(order) = self.graph.topological_order() else {
            let status = SimulationStatus::failure(
                SimulationPhase::HillslopeKernel,
                true,
                false,
                BoundaryClass::ClosureViolation,
                "HSCHED-E-GRAPH-CYCLE",
            )?;

            return Ok(HillslopeSchedulerReport {
                outcome_class: SchedulerOutcomeClass::SchedulerInvariantFailure,
                topology_precondition_status: topology_report.status.clone(),
                scheduler_status: status,
                ordered_phases: Vec::new(),
                outcomes: Vec::new(),
                precondition_violations: Vec::new(),
                halted_phase: None,
            });
        };

        let mut outcomes = Vec::with_capacity(order.len());
        let mut completed: BTreeSet<HillslopePhase> = BTreeSet::new();

        for phase in order.clone() {
            let has_unsatisfied_dependency = self
                .graph
                .dependencies_for(phase)
                .iter()
                .any(|dependency| !completed.contains(dependency));

            if has_unsatisfied_dependency {
                let status = SimulationStatus::failure(
                    SimulationPhase::HillslopeKernel,
                    true,
                    false,
                    BoundaryClass::ClosureViolation,
                    "HSCHED-E-DEPENDENCY-CLOSURE",
                )?;

                return Ok(HillslopeSchedulerReport {
                    outcome_class: SchedulerOutcomeClass::SchedulerInvariantFailure,
                    topology_precondition_status: topology_report.status.clone(),
                    scheduler_status: status,
                    ordered_phases: order,
                    outcomes,
                    precondition_violations: Vec::new(),
                    halted_phase: Some(phase),
                });
            }

            let phase_status = phase_executor(phase);
            if phase_status.phase() != SimulationPhase::HillslopeKernel {
                let status = SimulationStatus::failure(
                    SimulationPhase::HillslopeKernel,
                    true,
                    false,
                    BoundaryClass::ModeMismatch,
                    "HSCHED-E-PHASE-STATUS-PHASE",
                )?;

                outcomes.push(HillslopePhaseOutcome {
                    phase,
                    status: status.clone(),
                });

                return Ok(HillslopeSchedulerReport {
                    outcome_class: SchedulerOutcomeClass::SchedulerInvariantFailure,
                    topology_precondition_status: topology_report.status.clone(),
                    scheduler_status: status,
                    ordered_phases: order,
                    outcomes,
                    precondition_violations: Vec::new(),
                    halted_phase: Some(phase),
                });
            }

            let is_failure = phase_status.classification() == StatusClassification::Failure;
            outcomes.push(HillslopePhaseOutcome {
                phase,
                status: phase_status.clone(),
            });
            completed.insert(phase);

            if is_failure {
                return Ok(HillslopeSchedulerReport {
                    outcome_class: SchedulerOutcomeClass::PhaseFailure,
                    topology_precondition_status: topology_report.status.clone(),
                    scheduler_status: phase_status,
                    ordered_phases: order,
                    outcomes,
                    precondition_violations: Vec::new(),
                    halted_phase: Some(phase),
                });
            }
        }

        let has_advisory = outcomes
            .iter()
            .any(|outcome| outcome.status.classification() == StatusClassification::Advisory);
        let scheduler_status = if has_advisory {
            SimulationStatus::advisory(
                SimulationPhase::HillslopeKernel,
                BoundaryClass::CapBinding,
                ClampClass::None,
                "HSCHED-W-ADVISORY",
            )?
        } else {
            SimulationStatus::ok(SimulationPhase::HillslopeKernel, "HSCHED-OK-001")?
        };

        Ok(HillslopeSchedulerReport {
            outcome_class: SchedulerOutcomeClass::Completed,
            topology_precondition_status: topology_report.status.clone(),
            scheduler_status,
            ordered_phases: order,
            outcomes,
            precondition_violations: Vec::new(),
            halted_phase: None,
        })
    }
}

impl Default for HillslopePhaseScheduler {
    fn default() -> Self {
        Self::canonical()
    }
}

#[cfg(test)]
mod tests {
    use std::cell::Cell;

    use openwepp_sim_contract::status::{BoundaryClass, SimulationPhase, StatusClassification};
    use openwepp_topology::{parse_topology_fixture_str, validate_pre_execution_topology};

    use super::{
        HillslopePhase, HillslopePhaseGraph, HillslopePhaseScheduler, SchedulerOutcomeClass,
    };

    const VALID_TOPOLOGY: &str = r"
HILLSLOPES 3
CHANNELS 2
IMPOUNDMENTS 1
NODE CHANNEL 1 H 1 2 0 C 0 0 0 I 0 0 0
NODE CHANNEL 2 H 3 0 0 C 1 0 0 I 0 0 0
NODE IMPOUNDMENT 1 H 0 0 0 C 2 0 0 I 0 0 0
";

    const INVALID_TOPOLOGY: &str = r"
HILLSLOPES 3
CHANNELS 2
IMPOUNDMENTS 1
NODE CHANNEL 1 H 0 0 0 C 0 0 0 I 0 0 0
NODE CHANNEL 2 H 3 0 0 C 1 0 0 I 0 0 0
NODE IMPOUNDMENT 1 H 0 0 0 C 2 0 0 I 0 0 0
";

    #[test]
    fn canonical_graph_order_is_deterministic() {
        let graph = HillslopePhaseGraph::canonical();
        let order = graph
            .topological_order()
            .expect("canonical graph should always topologically sort");

        assert_eq!(
            order,
            Vec::from(HillslopePhaseGraph::canonical_order()),
            "ARCH05 requires explicit deterministic scheduler order"
        );
        assert_eq!(graph.dependency_edges().len(), 8);
    }

    #[test]
    fn topology_precondition_failure_blocks_phase_execution() {
        let graph = parse_topology_fixture_str(INVALID_TOPOLOGY).expect("fixture should parse");
        let topology_report =
            validate_pre_execution_topology(&graph).expect("topology report should build");
        assert_eq!(
            topology_report.status.classification(),
            StatusClassification::Failure
        );

        let scheduler = HillslopePhaseScheduler::canonical();
        let call_count = Cell::new(0_usize);

        let report = scheduler
            .execute_with(&topology_report, |_| {
                call_count.set(call_count.get() + 1);
                HillslopePhaseScheduler::nominal_phase_status(HillslopePhase::Normalization)
                    .expect("nominal status should build")
            })
            .expect("scheduler should not error");

        assert_eq!(call_count.get(), 0);
        assert_eq!(
            report.outcome_class,
            SchedulerOutcomeClass::TopologyPreconditionFailed
        );
        assert_eq!(
            report.scheduler_status.classification(),
            StatusClassification::Failure
        );
        assert_eq!(
            report.scheduler_status.boundary_class(),
            BoundaryClass::TopologyInvalid
        );
    }

    #[test]
    fn phase_failure_is_typed_and_fail_fast() {
        let graph = parse_topology_fixture_str(VALID_TOPOLOGY).expect("fixture should parse");
        let topology_report =
            validate_pre_execution_topology(&graph).expect("topology report should build");
        let scheduler = HillslopePhaseScheduler::canonical();

        let report = scheduler
            .execute_with(&topology_report, |phase| {
                if phase == HillslopePhase::PercolationDeepSeepage {
                    return openwepp_sim_contract::status::SimulationStatus::failure(
                        SimulationPhase::HillslopeKernel,
                        true,
                        false,
                        BoundaryClass::DomainViolation,
                        "HSCHED-PHASE-E-004",
                    )
                    .expect("failure status should build");
                }

                HillslopePhaseScheduler::nominal_phase_status(phase)
                    .expect("nominal status should build")
            })
            .expect("scheduler should not error");

        assert_eq!(report.outcome_class, SchedulerOutcomeClass::PhaseFailure);
        assert_eq!(
            report.scheduler_status.classification(),
            StatusClassification::Failure
        );
        assert_eq!(
            report.scheduler_status.boundary_class(),
            BoundaryClass::DomainViolation
        );
        assert_eq!(
            report.executed_phases(),
            vec![
                HillslopePhase::Normalization,
                HillslopePhase::StorageBounds,
                HillslopePhase::Evapotranspiration,
                HillslopePhase::PercolationDeepSeepage,
            ]
        );
        assert_eq!(
            report.halted_phase,
            Some(HillslopePhase::PercolationDeepSeepage)
        );
    }

    #[test]
    fn phase_status_phase_mismatch_returns_mode_mismatch_failure() {
        let graph = parse_topology_fixture_str(VALID_TOPOLOGY).expect("fixture should parse");
        let topology_report =
            validate_pre_execution_topology(&graph).expect("topology report should build");
        let scheduler = HillslopePhaseScheduler::canonical();

        let report = scheduler
            .execute_with(&topology_report, |_| {
                openwepp_sim_contract::status::SimulationStatus::ok(
                    SimulationPhase::PreExecutionValidation,
                    "HSCHED-PHASE-INVALID-STATUS",
                )
                .expect("status should build")
            })
            .expect("scheduler should not error");

        assert_eq!(
            report.outcome_class,
            SchedulerOutcomeClass::SchedulerInvariantFailure
        );
        assert_eq!(
            report.scheduler_status.classification(),
            StatusClassification::Failure
        );
        assert_eq!(
            report.scheduler_status.boundary_class(),
            BoundaryClass::ModeMismatch
        );
        assert_eq!(report.halted_phase, Some(HillslopePhase::Normalization));
    }

    #[test]
    fn nominal_execution_completes_in_canonical_order() {
        let graph = parse_topology_fixture_str(VALID_TOPOLOGY).expect("fixture should parse");
        let topology_report =
            validate_pre_execution_topology(&graph).expect("topology report should build");
        let scheduler = HillslopePhaseScheduler::canonical();

        let report = scheduler
            .execute_with(&topology_report, |phase| {
                HillslopePhaseScheduler::nominal_phase_status(phase)
                    .expect("nominal status should build")
            })
            .expect("scheduler should not error");

        assert!(report.is_success());
        assert_eq!(report.outcome_class, SchedulerOutcomeClass::Completed);
        assert_eq!(report.halted_phase, None);
        assert_eq!(
            report.executed_phases(),
            Vec::from(HillslopePhaseGraph::canonical_order())
        );
        assert_eq!(
            report.scheduler_status.phase(),
            SimulationPhase::HillslopeKernel
        );
        assert_eq!(
            report.scheduler_status.classification(),
            StatusClassification::Nominal
        );
    }
}
