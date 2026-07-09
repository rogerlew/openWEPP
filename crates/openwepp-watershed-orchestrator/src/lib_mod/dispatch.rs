use std::collections::{BTreeMap, BTreeSet};

use openwepp_sim_contract::status::{
    BoundaryClass, SimulationPhase, SimulationStatus, StatusClassification, StatusError,
};
use openwepp_topology::{
    TopologyGraph, TopologyNodeKey, TopologyNodeKind, TopologyValidationReport,
    validate_pre_execution_topology,
};

use super::kernel::{
    DirectWatershedKernelInput, DirectWatershedKernelOutput, Ws10ChannelImpoundmentKernel,
};
use super::network_frame::WatershedNetworkFrame;
use super::types::{
    DispatchDiagnostic, DispatchDiagnosticCode, DispatchStep, MESSAGE_CYCLE_DETECTED,
    MESSAGE_DISPATCH_OK, MESSAGE_MISSING_DEPENDENCY, MESSAGE_PRECONDITION_FAILED,
    WatershedDispatchError, WatershedDispatchReport, WatershedFrameExecutionReport,
    WatershedFrameStepReport,
};

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

/// Execute watershed dispatch directly against typed `WatershedNetworkFrame`
/// state.
///
/// This is the production frame-native path. It schedules the validated
/// topology, invokes typed WS10/WS11/WS12/WS18 routing, and writes routed
/// channel/impoundment state back to the frame.
///
/// # Errors
///
/// Returns `WatershedDispatchError` when scheduler/status construction fails.
pub fn execute_watershed_dispatch_with_frame(
    frame: &mut WatershedNetworkFrame,
    topology_validation: &TopologyValidationReport,
) -> Result<WatershedFrameExecutionReport, WatershedDispatchError> {
    let mut dispatch_report = schedule_watershed_dispatch(frame.topology(), topology_validation)?;

    if !dispatch_report.is_success() {
        return Ok(WatershedFrameExecutionReport {
            dispatch_report,
            step_reports: Vec::new(),
        });
    }

    let mut step_reports = Vec::new();

    for step in dispatch_report.steps.iter().cloned() {
        let response =
            Ws10ChannelImpoundmentKernel::run_direct_watershed_node(&DirectWatershedKernelInput {
                step: &step,
                frame: &*frame,
            });
        let kernel_status = response.status.clone();
        let mut routed_state_applied = false;

        if kernel_status.classification() == StatusClassification::Failure {
            step_reports.push(WatershedFrameStepReport {
                step,
                kernel_status: kernel_status.clone(),
                routed_state_applied,
            });
            dispatch_report.dispatch_status = kernel_status;
            break;
        }

        if let Some(output) = response.output {
            match output {
                DirectWatershedKernelOutput::Channel(state) => {
                    frame.record_routed_channel_state(*state);
                    routed_state_applied = true;
                }
                DirectWatershedKernelOutput::Impoundment(state) => {
                    frame.record_routed_impoundment_state(state);
                    routed_state_applied = true;
                }
            }
        }

        step_reports.push(WatershedFrameStepReport {
            step,
            kernel_status: kernel_status.clone(),
            routed_state_applied,
        });

        if kernel_status.classification() == StatusClassification::Advisory {
            dispatch_report.dispatch_status = kernel_status;
        }
    }

    Ok(WatershedFrameExecutionReport {
        dispatch_report,
        step_reports,
    })
}

type DependencyMap = BTreeMap<TopologyNodeKey, BTreeSet<TopologyNodeKey>>;
type HillslopeContributorMap = BTreeMap<TopologyNodeKey, BTreeSet<u32>>;
type IndegreeMap = BTreeMap<TopologyNodeKey, usize>;
type DependentMap = BTreeMap<TopologyNodeKey, BTreeSet<TopologyNodeKey>>;

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

    let (dependencies, hillslope_contributors) = collect_dependency_maps(graph, &dispatch_nodes)?;
    let (mut indegree, dependents) = build_indegree_and_dependents(&dependencies, &dispatch_nodes);

    let mut ready: BTreeSet<TopologyNodeKey> = indegree
        .iter()
        .filter_map(|(node, count)| if *count == 0 { Some(*node) } else { None })
        .collect();

    let mut steps: Vec<DispatchStep> = Vec::new();

    while let Some(node) = ready.pop_first() {
        steps.push(build_dispatch_step(
            node,
            steps.len(),
            &dependencies,
            &hillslope_contributors,
        )?);

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

fn collect_dependency_maps(
    graph: &TopologyGraph,
    dispatch_nodes: &BTreeSet<TopologyNodeKey>,
) -> Result<(DependencyMap, HillslopeContributorMap), DispatchPlanError> {
    let mut dependencies: DependencyMap = dispatch_nodes
        .iter()
        .copied()
        .map(|key| (key, BTreeSet::new()))
        .collect();
    let mut hillslope_contributors: HillslopeContributorMap = dispatch_nodes
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
            record_contributor(
                &mut dependencies,
                &mut hillslope_contributors,
                dispatch_nodes,
                node.key,
                kind,
                contributor_id,
            )?;
        }
    }

    Ok((dependencies, hillslope_contributors))
}

fn record_contributor(
    dependencies: &mut DependencyMap,
    hillslope_contributors: &mut HillslopeContributorMap,
    dispatch_nodes: &BTreeSet<TopologyNodeKey>,
    node: TopologyNodeKey,
    kind: TopologyNodeKind,
    contributor_id: u32,
) -> Result<(), DispatchPlanError> {
    match kind {
        TopologyNodeKind::Hillslope => {
            hillslope_contributors
                .entry(node)
                .or_default()
                .insert(contributor_id);
            Ok(())
        }
        TopologyNodeKind::Channel | TopologyNodeKind::Impoundment => {
            let dependency = TopologyNodeKey::new(kind, contributor_id);
            if !dispatch_nodes.contains(&dependency) {
                return Err(DispatchPlanError::MissingDependency { node, dependency });
            }

            dependencies.entry(node).or_default().insert(dependency);
            Ok(())
        }
    }
}

fn build_indegree_and_dependents(
    dependencies: &DependencyMap,
    dispatch_nodes: &BTreeSet<TopologyNodeKey>,
) -> (IndegreeMap, DependentMap) {
    let indegree: IndegreeMap = dependencies
        .iter()
        .map(|(node, parents)| (*node, parents.len()))
        .collect();
    let mut dependents: DependentMap = dispatch_nodes
        .iter()
        .copied()
        .map(|node| (node, BTreeSet::new()))
        .collect();

    for (node, parents) in dependencies {
        for parent in parents {
            dependents.entry(*parent).or_default().insert(*node);
        }
    }

    (indegree, dependents)
}

fn build_dispatch_step(
    node: TopologyNodeKey,
    sequence_index: usize,
    dependencies: &DependencyMap,
    hillslope_contributors: &HillslopeContributorMap,
) -> Result<DispatchStep, DispatchPlanError> {
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

    Ok(DispatchStep {
        sequence_index,
        node,
        dependency_nodes: parent_nodes,
        contributor_hillslopes: hillslope_nodes,
        status,
    })
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
