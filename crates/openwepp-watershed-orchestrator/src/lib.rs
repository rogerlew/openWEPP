//! Deterministic watershed dispatch scheduler graph for openWEPP.

use std::collections::{BTreeMap, BTreeSet};
use std::error::Error;
use std::fmt;

use openwepp_sim_contract::status::{
    BoundaryClass, SimulationPhase, SimulationStatus, StatusClassification, StatusError,
};
use openwepp_topology::{
    TopologyGraph, TopologyNodeKey, TopologyNodeKind, TopologyValidationError,
    TopologyValidationReport, validate_pre_execution_topology,
};

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

/// Error surface for watershed dispatch scheduler orchestration.
#[derive(Debug)]
pub enum WatershedDispatchError {
    Status(StatusError),
    TopologyValidation(TopologyValidationError),
}

impl fmt::Display for WatershedDispatchError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Status(source) => write!(f, "failed constructing dispatch status: {source}"),
            Self::TopologyValidation(source) => write!(
                f,
                "failed constructing topology validation gate report: {source}"
            ),
        }
    }
}

impl Error for WatershedDispatchError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            Self::Status(source) => Some(source),
            Self::TopologyValidation(source) => Some(source),
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

/// Schedule deterministic watershed dispatch order using an explicit topology
/// validation gate result as a hard precondition.
///
/// # Errors
///
/// Returns `WatershedDispatchError` when typed status construction fails.
pub fn schedule_watershed_dispatch(
    graph: &TopologyGraph,
    topology_validation: &TopologyValidationReport,
) -> Result<WatershedDispatchReport, WatershedDispatchError> {
    if !topology_validation.is_valid() {
        let dispatch_status = SimulationStatus::failure(
            SimulationPhase::WatershedKernel,
            true,
            false,
            BoundaryClass::TopologyInvalid,
            MESSAGE_PRECONDITION_FAILED,
        )?;

        let diagnostics = vec![DispatchDiagnostic::new(
            DispatchDiagnosticCode::TopologyPreconditionFailed,
            MESSAGE_PRECONDITION_FAILED,
            format!(
                "dispatch aborted: topology precondition failed with {} violation(s); gate_message_id={}",
                topology_validation.violations.len(),
                topology_validation.status.message_id()
            ),
        )];

        return Ok(WatershedDispatchReport {
            precondition_status: topology_validation.status.clone(),
            dispatch_status,
            steps: Vec::new(),
            diagnostics,
        });
    }

    match build_dispatch_steps(graph) {
        Ok(steps) => {
            let dispatch_status =
                SimulationStatus::ok(SimulationPhase::WatershedKernel, MESSAGE_DISPATCH_OK)?;

            Ok(WatershedDispatchReport {
                precondition_status: topology_validation.status.clone(),
                dispatch_status,
                steps,
                diagnostics: Vec::new(),
            })
        }
        Err(DispatchPlanError::MissingDependency { node, dependency }) => {
            let dispatch_status = SimulationStatus::failure(
                SimulationPhase::WatershedKernel,
                true,
                false,
                BoundaryClass::TopologyInvalid,
                MESSAGE_MISSING_DEPENDENCY,
            )?;

            let diagnostics = vec![DispatchDiagnostic::new(
                DispatchDiagnosticCode::MissingDependency,
                MESSAGE_MISSING_DEPENDENCY,
                format!(
                    "dispatch node {} depends on unresolved node {}",
                    format_node_key(node),
                    format_node_key(dependency)
                ),
            )];

            Ok(WatershedDispatchReport {
                precondition_status: topology_validation.status.clone(),
                dispatch_status,
                steps: Vec::new(),
                diagnostics,
            })
        }
        Err(DispatchPlanError::Status(source)) => Err(WatershedDispatchError::Status(source)),
        Err(DispatchPlanError::DependencyCycle { remaining_nodes }) => {
            let dispatch_status = SimulationStatus::failure(
                SimulationPhase::WatershedKernel,
                true,
                false,
                BoundaryClass::TopologyInvalid,
                MESSAGE_CYCLE_DETECTED,
            )?;

            let remaining_labels: Vec<String> = remaining_nodes
                .iter()
                .copied()
                .map(format_node_key)
                .collect();

            let diagnostics = vec![DispatchDiagnostic::new(
                DispatchDiagnosticCode::DependencyCycleDetected,
                MESSAGE_CYCLE_DETECTED,
                format!(
                    "dispatch dependency cycle detected; unresolved_nodes={}",
                    remaining_labels.join(",")
                ),
            )];

            Ok(WatershedDispatchReport {
                precondition_status: topology_validation.status.clone(),
                dispatch_status,
                steps: Vec::new(),
                diagnostics,
            })
        }
    }
}

/// Run the ARCH04 topology validation gate and execute deterministic dispatch
/// scheduling when preconditions pass.
///
/// # Errors
///
/// Returns `WatershedDispatchError` when topology validation or status
/// construction fails.
pub fn schedule_watershed_dispatch_with_gate(
    graph: &TopologyGraph,
) -> Result<WatershedDispatchReport, WatershedDispatchError> {
    let topology_validation = validate_pre_execution_topology(graph)?;
    schedule_watershed_dispatch(graph, &topology_validation)
}

#[derive(Debug)]
enum DispatchPlanError {
    Status(StatusError),
    MissingDependency {
        node: TopologyNodeKey,
        dependency: TopologyNodeKey,
    },
    DependencyCycle {
        remaining_nodes: Vec<TopologyNodeKey>,
    },
}

fn build_dispatch_steps(graph: &TopologyGraph) -> Result<Vec<DispatchStep>, DispatchPlanError> {
    let dispatch_nodes: BTreeSet<TopologyNodeKey> = graph
        .nodes()
        .iter()
        .map(|node| node.key)
        .filter(|key| key.kind != TopologyNodeKind::Hillslope)
        .collect();

    let mut dependencies: BTreeMap<TopologyNodeKey, BTreeSet<TopologyNodeKey>> = dispatch_nodes
        .iter()
        .copied()
        .map(|key| (key, BTreeSet::new()))
        .collect();

    let mut hillslope_contributors: BTreeMap<TopologyNodeKey, BTreeSet<u32>> = dispatch_nodes
        .iter()
        .copied()
        .map(|key| (key, BTreeSet::new()))
        .collect();

    for node in graph.nodes() {
        if node.key.kind == TopologyNodeKind::Hillslope {
            continue;
        }

        for (kind, _slot, contributor_id) in node.contributors.references() {
            if contributor_id == 0 {
                continue;
            }

            match kind {
                TopologyNodeKind::Hillslope => {
                    hillslope_contributors
                        .entry(node.key)
                        .or_default()
                        .insert(contributor_id);
                }
                TopologyNodeKind::Channel | TopologyNodeKind::Impoundment => {
                    let dependency = TopologyNodeKey::new(kind, contributor_id);
                    if !dispatch_nodes.contains(&dependency) {
                        return Err(DispatchPlanError::MissingDependency {
                            node: node.key,
                            dependency,
                        });
                    }

                    dependencies.entry(node.key).or_default().insert(dependency);
                }
            }
        }
    }

    let mut indegree: BTreeMap<TopologyNodeKey, usize> = dependencies
        .iter()
        .map(|(node, parents)| (*node, parents.len()))
        .collect();

    let mut dependents: BTreeMap<TopologyNodeKey, BTreeSet<TopologyNodeKey>> = dispatch_nodes
        .iter()
        .copied()
        .map(|node| (node, BTreeSet::new()))
        .collect();

    for (node, parents) in &dependencies {
        for parent in parents {
            dependents.entry(*parent).or_default().insert(*node);
        }
    }

    let mut ready: BTreeSet<TopologyNodeKey> = indegree
        .iter()
        .filter_map(|(node, count)| if *count == 0 { Some(*node) } else { None })
        .collect();

    let mut steps: Vec<DispatchStep> = Vec::new();

    while let Some(node) = ready.pop_first() {
        let parent_nodes: Vec<TopologyNodeKey> = dependencies
            .get(&node)
            .map(|parents| parents.iter().copied().collect())
            .unwrap_or_default();

        let hillslope_nodes: Vec<u32> = hillslope_contributors
            .get(&node)
            .map(|parents| parents.iter().copied().collect())
            .unwrap_or_default();

        let status = SimulationStatus::ok(
            SimulationPhase::WatershedKernel,
            format!(
                "WATERSHED-DISPATCH-STEP-{}-{}-OK",
                node_kind_message_token(node.kind),
                node.id
            ),
        )
        .map_err(DispatchPlanError::Status)?;

        steps.push(DispatchStep {
            sequence_index: steps.len(),
            node,
            dependency_nodes: parent_nodes,
            contributor_hillslopes: hillslope_nodes,
            status,
        });

        if let Some(children) = dependents.get(&node) {
            for child in children {
                if let Some(count) = indegree.get_mut(child) {
                    *count -= 1;
                    if *count == 0 {
                        ready.insert(*child);
                    }
                }
            }
        }
    }

    if steps.len() != dispatch_nodes.len() {
        let remaining_nodes: Vec<TopologyNodeKey> = indegree
            .into_iter()
            .filter_map(|(node, count)| if count > 0 { Some(node) } else { None })
            .collect();

        return Err(DispatchPlanError::DependencyCycle { remaining_nodes });
    }

    Ok(steps)
}

fn node_kind_message_token(kind: TopologyNodeKind) -> &'static str {
    match kind {
        TopologyNodeKind::Hillslope => "HILLSLOPE",
        TopologyNodeKind::Channel => "CHANNEL",
        TopologyNodeKind::Impoundment => "IMPOUNDMENT",
    }
}

fn format_node_key(key: TopologyNodeKey) -> String {
    format!("{}:{}", key.kind.as_str(), key.id)
}

#[cfg(test)]
mod tests {
    use openwepp_sim_contract::status::{BoundaryClass, SimulationPhase, StatusClassification};
    use openwepp_topology::{
        ContributorTriplet, TopologyContributors, TopologyNode, validate_pre_execution_topology,
    };

    use super::*;

    #[test]
    fn schedules_dispatch_in_deterministic_dependency_order() {
        let graph = TopologyGraph::new(
            4,
            3,
            2,
            vec![
                node(
                    TopologyNodeKind::Channel,
                    1,
                    [1, 0, 0],
                    [0, 0, 0],
                    [0, 0, 0],
                ),
                node(
                    TopologyNodeKind::Impoundment,
                    1,
                    [2, 0, 0],
                    [0, 0, 0],
                    [0, 0, 0],
                ),
                node(
                    TopologyNodeKind::Channel,
                    2,
                    [0, 0, 0],
                    [1, 0, 0],
                    [1, 0, 0],
                ),
                node(
                    TopologyNodeKind::Impoundment,
                    2,
                    [0, 0, 0],
                    [1, 0, 0],
                    [0, 0, 0],
                ),
                node(
                    TopologyNodeKind::Channel,
                    3,
                    [3, 0, 0],
                    [2, 0, 0],
                    [2, 0, 0],
                ),
            ],
        );

        let topology_validation =
            validate_pre_execution_topology(&graph).expect("topology validation should construct");
        assert!(topology_validation.is_valid());

        let report =
            schedule_watershed_dispatch(&graph, &topology_validation).expect("schedule should run");

        assert!(report.is_success());
        assert_eq!(
            report.dispatch_status.classification(),
            StatusClassification::Nominal
        );
        assert!(report.diagnostics.is_empty());

        let observed_order: Vec<TopologyNodeKey> =
            report.steps.iter().map(|step| step.node).collect();
        let expected_order = vec![
            key(TopologyNodeKind::Channel, 1),
            key(TopologyNodeKind::Impoundment, 1),
            key(TopologyNodeKind::Channel, 2),
            key(TopologyNodeKind::Impoundment, 2),
            key(TopologyNodeKind::Channel, 3),
        ];

        assert_eq!(observed_order, expected_order);

        let channel_two = &report.steps[2];
        assert_eq!(
            channel_two.dependency_nodes,
            vec![
                key(TopologyNodeKind::Channel, 1),
                key(TopologyNodeKind::Impoundment, 1),
            ]
        );

        let impoundment_two = &report.steps[3];
        assert_eq!(
            impoundment_two.dependency_nodes,
            vec![key(TopologyNodeKind::Channel, 1)]
        );

        for step in &report.steps {
            assert_eq!(step.status.phase(), SimulationPhase::WatershedKernel);
            assert_eq!(step.status.boundary_class(), BoundaryClass::Ok);
            assert_eq!(step.status.classification(), StatusClassification::Nominal);
        }
    }

    #[test]
    fn blocks_dispatch_when_topology_precondition_fails() {
        let graph = TopologyGraph::new(
            1,
            2,
            0,
            vec![node(
                TopologyNodeKind::Channel,
                1,
                [1, 0, 0],
                [0, 0, 0],
                [0, 0, 0],
            )],
        );

        let topology_validation =
            validate_pre_execution_topology(&graph).expect("topology validation should construct");
        assert!(!topology_validation.is_valid());

        let report =
            schedule_watershed_dispatch(&graph, &topology_validation).expect("schedule should run");

        assert!(!report.is_success());
        assert!(report.steps.is_empty());
        assert_eq!(
            report.precondition_status.classification(),
            StatusClassification::Failure
        );
        assert_eq!(
            report.dispatch_status.classification(),
            StatusClassification::Failure
        );
        assert_eq!(
            report.dispatch_status.boundary_class(),
            BoundaryClass::TopologyInvalid
        );
        assert_eq!(
            report.dispatch_status.message_id(),
            MESSAGE_PRECONDITION_FAILED
        );
        assert_eq!(report.diagnostics.len(), 1);
        assert_eq!(
            report.diagnostics[0].code,
            DispatchDiagnosticCode::TopologyPreconditionFailed
        );
    }

    #[test]
    fn classifies_cycle_as_typed_failure_class() {
        let graph = TopologyGraph::new(
            1,
            2,
            0,
            vec![
                node(
                    TopologyNodeKind::Channel,
                    1,
                    [1, 0, 0],
                    [2, 0, 0],
                    [0, 0, 0],
                ),
                node(
                    TopologyNodeKind::Channel,
                    2,
                    [1, 0, 0],
                    [1, 0, 0],
                    [0, 0, 0],
                ),
            ],
        );

        let forged_valid = TopologyValidationReport {
            status: SimulationStatus::ok(
                SimulationPhase::PreExecutionValidation,
                "TOPOLOGY-OK-001",
            )
            .expect("status should construct"),
            violations: Vec::new(),
        };

        let report =
            schedule_watershed_dispatch(&graph, &forged_valid).expect("schedule should run");

        assert!(!report.is_success());
        assert!(report.steps.is_empty());
        assert_eq!(
            report.dispatch_status.classification(),
            StatusClassification::Failure
        );
        assert_eq!(
            report.dispatch_status.boundary_class(),
            BoundaryClass::TopologyInvalid
        );
        assert_eq!(report.dispatch_status.message_id(), MESSAGE_CYCLE_DETECTED);
        assert_eq!(report.diagnostics.len(), 1);
        assert_eq!(
            report.diagnostics[0].code,
            DispatchDiagnosticCode::DependencyCycleDetected
        );
    }

    #[test]
    fn classifies_missing_dependency_as_typed_failure_class() {
        let graph = TopologyGraph::new(
            1,
            1,
            0,
            vec![node(
                TopologyNodeKind::Channel,
                1,
                [1, 0, 0],
                [2, 0, 0],
                [0, 0, 0],
            )],
        );

        let forged_valid = TopologyValidationReport {
            status: SimulationStatus::ok(
                SimulationPhase::PreExecutionValidation,
                "TOPOLOGY-OK-001",
            )
            .expect("status should construct"),
            violations: Vec::new(),
        };

        let report =
            schedule_watershed_dispatch(&graph, &forged_valid).expect("schedule should run");

        assert!(!report.is_success());
        assert_eq!(
            report.dispatch_status.classification(),
            StatusClassification::Failure
        );
        assert_eq!(
            report.dispatch_status.boundary_class(),
            BoundaryClass::TopologyInvalid
        );
        assert_eq!(
            report.dispatch_status.message_id(),
            MESSAGE_MISSING_DEPENDENCY
        );
        assert_eq!(report.diagnostics.len(), 1);
        assert_eq!(
            report.diagnostics[0].code,
            DispatchDiagnosticCode::MissingDependency
        );
    }

    fn key(kind: TopologyNodeKind, id: u32) -> TopologyNodeKey {
        TopologyNodeKey::new(kind, id)
    }

    fn node(
        kind: TopologyNodeKind,
        id: u32,
        hillslope: [u32; 3],
        channels: [u32; 3],
        impoundments: [u32; 3],
    ) -> TopologyNode {
        let contributors = TopologyContributors::new(
            ContributorTriplet::new(hillslope[0], hillslope[1], hillslope[2]),
            ContributorTriplet::new(channels[0], channels[1], channels[2]),
            ContributorTriplet::new(impoundments[0], impoundments[1], impoundments[2]),
        );

        TopologyNode::new(key(kind, id), contributors)
    }
}
