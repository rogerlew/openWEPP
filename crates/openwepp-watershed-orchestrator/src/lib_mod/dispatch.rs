use std::collections::{BTreeMap, BTreeSet};

use openwepp_kernel_contract::{
    WatershedKernel, WatershedKernelRequest, apply_kernel_writeback, evaluate_kernel_writeback,
};
use openwepp_sim_contract::status::{
    BoundaryClass, SimulationPhase, SimulationStatus, StatusClassification, StatusError,
};
use openwepp_topology::{
    TopologyGraph, TopologyNodeKey, TopologyNodeKind, TopologyValidationReport,
    validate_pre_execution_topology,
};

use super::types::{
    DispatchDiagnostic, DispatchDiagnosticCode, DispatchStep, MESSAGE_CYCLE_DETECTED,
    MESSAGE_DISPATCH_OK, MESSAGE_MISSING_DEPENDENCY, MESSAGE_PRECONDITION_FAILED,
    WatershedDispatchError, WatershedDispatchReport, WatershedKernelExecutionReport,
    WatershedKernelStepReport, WatershedWritebackSurface,
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

/// Execute watershed dispatch scheduling and invoke watershed kernels through
/// the typed ARCH07 boundary.
///
/// Kernel writeback proposals are accepted/rejected/applied by orchestrator
/// policy. Kernel code never mutates orchestrator state directly.
///
/// # Errors
///
/// Returns `WatershedDispatchError` when scheduler/status construction fails or
/// when writeback apply surfaces return typed errors.
pub fn execute_watershed_dispatch_with_kernel<K>(
    graph: &TopologyGraph,
    topology_validation: &TopologyValidationReport,
    kernel: &mut K,
    mut writeback_surface: WatershedWritebackSurface,
) -> Result<WatershedKernelExecutionReport, WatershedDispatchError>
where
    K: WatershedKernel,
{
    let mut dispatch_report = schedule_watershed_dispatch(graph, topology_validation)?;

    if !dispatch_report.is_success() {
        return Ok(WatershedKernelExecutionReport {
            dispatch_report,
            step_reports: Vec::new(),
            writeback_surface,
        });
    }

    let mode_mismatch_status = SimulationStatus::failure(
        SimulationPhase::WatershedKernel,
        true,
        false,
        BoundaryClass::ModeMismatch,
        "WKERNEL-E-STATUS-PHASE-MISMATCH",
    )?;

    let mut step_reports = Vec::new();

    for step in dispatch_report.steps.iter().cloned() {
        let response = {
            let request = WatershedKernelRequest::new(
                step.node.kind.as_str(),
                step.node.id,
                step.dependency_nodes
                    .iter()
                    .map(|node| format_node_key(*node))
                    .collect::<Vec<String>>(),
                &step.contributor_hillslopes,
                &writeback_surface.state_surface,
                &writeback_surface.flux_surface,
            );

            kernel.run_watershed_node(&request)
        };
        let kernel_status = response.status.clone();

        if kernel_status.phase() != SimulationPhase::WatershedKernel {
            step_reports.push(WatershedKernelStepReport {
                step,
                kernel_status,
                decision_outcome: openwepp_kernel_contract::WritebackDecisionOutcome::Reject,
                decision_status: mode_mismatch_status.clone(),
                apply_result: None,
            });
            dispatch_report.dispatch_status = mode_mismatch_status.clone();
            break;
        }

        if kernel_status.classification() == StatusClassification::Failure {
            step_reports.push(WatershedKernelStepReport {
                step,
                kernel_status: kernel_status.clone(),
                decision_outcome: openwepp_kernel_contract::WritebackDecisionOutcome::Reject,
                decision_status: kernel_status.clone(),
                apply_result: None,
            });
            dispatch_report.dispatch_status = kernel_status;
            break;
        }

        let decision =
            evaluate_kernel_writeback(SimulationPhase::WatershedKernel, &response.writeback)?;
        if decision.outcome == openwepp_kernel_contract::WritebackDecisionOutcome::Reject {
            step_reports.push(WatershedKernelStepReport {
                step,
                kernel_status,
                decision_outcome: openwepp_kernel_contract::WritebackDecisionOutcome::Reject,
                decision_status: decision.status.clone(),
                apply_result: None,
            });
            dispatch_report.dispatch_status = decision.status;
            break;
        }

        let apply_result = apply_kernel_writeback(
            SimulationPhase::WatershedKernel,
            &decision,
            &response.writeback,
            &mut writeback_surface.state_surface,
            &mut writeback_surface.flux_surface,
        )?;

        step_reports.push(WatershedKernelStepReport {
            step,
            kernel_status: kernel_status.clone(),
            decision_outcome: apply_result.outcome,
            decision_status: apply_result.status.clone(),
            apply_result: Some(apply_result),
        });

        if kernel_status.classification() == StatusClassification::Advisory {
            dispatch_report.dispatch_status = kernel_status;
        }
    }

    Ok(WatershedKernelExecutionReport {
        dispatch_report,
        step_reports,
        writeback_surface,
    })
}

/// Execute topology validation gate + watershed dispatch + kernel writeback
/// protocol in one helper surface.
///
/// # Errors
///
/// Returns `WatershedDispatchError` when topology validation, dispatch status
/// construction, or writeback apply surfaces return typed errors.
pub fn execute_watershed_dispatch_with_gate_and_kernel<K>(
    graph: &TopologyGraph,
    kernel: &mut K,
    writeback_surface: WatershedWritebackSurface,
) -> Result<WatershedKernelExecutionReport, WatershedDispatchError>
where
    K: WatershedKernel,
{
    let topology_validation = validate_pre_execution_topology(graph)?;
    execute_watershed_dispatch_with_kernel(graph, &topology_validation, kernel, writeback_surface)
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
