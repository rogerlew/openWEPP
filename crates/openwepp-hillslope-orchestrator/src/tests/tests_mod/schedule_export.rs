use super::fixtures::*;
use super::*;

fn canonical_dependency_map_for_test() -> BTreeMap<HillslopePhase, Vec<HillslopePhase>> {
    let order = HillslopePhaseGraph::canonical_order();
    let mut dependencies = BTreeMap::new();
    for phase in order {
        dependencies.insert(phase, Vec::new());
    }
    for pair in order.windows(2) {
        dependencies
            .entry(pair[1])
            .or_insert_with(Vec::new)
            .push(pair[0]);
    }
    dependencies
}

#[test]
fn schedule_export_formats_reflect_canonical_graph() {
    let export = ScheduleExport::from_graph(&HillslopePhaseGraph::canonical())
        .expect("canonical graph should export");
    let json = export.render_json();
    let mermaid = export.render_mermaid();
    let dot = export.render_dot();

    assert_eq!(
        export.nodes.len(),
        HillslopePhaseGraph::canonical_order().len()
    );
    assert_eq!(
        export.edges.len(),
        HillslopePhaseGraph::canonical_order().len() - 1
    );
    assert!(json.contains("\"phase\": \"normalization\""));
    assert!(json.contains("\"consumer_adapter\": \"soil\""));
    assert!(json.contains("\"topological_order\""));
    assert!(mermaid.starts_with("flowchart TD\n"));
    assert!(mermaid.contains("normalization --> storage_bounds"));
    assert!(dot.starts_with("digraph hillslope_phase_schedule {\n"));
    assert!(dot.contains("\"normalization\" -> \"storage_bounds\";"));
}

#[test]
fn schedule_export_validation_reports_cycle() {
    let mut dependencies = canonical_dependency_map_for_test();
    dependencies.insert(
        HillslopePhase::Normalization,
        vec![HillslopePhase::ClosureDiagnostics],
    );
    let graph = HillslopePhaseGraph::from_dependencies_for_test(dependencies);
    let report = validate_hillslope_schedule_graph(&graph);

    assert!(
        report
            .diagnostics
            .iter()
            .any(|diagnostic| diagnostic == &ScheduleDiagnostic::GraphCycle)
    );
    assert!(
        ScheduleExport::from_graph(&graph).is_err(),
        "malformed graph must not export silently"
    );
}

#[test]
fn schedule_export_validation_reports_disconnected_phase() {
    let mut dependencies = canonical_dependency_map_for_test();
    dependencies.insert(HillslopePhase::StorageBounds, Vec::new());
    let graph = HillslopePhaseGraph::from_dependencies_for_test(dependencies);
    let report = validate_hillslope_schedule_graph(&graph);

    assert!(report.diagnostics.iter().any(|diagnostic| {
        diagnostic
            == &ScheduleDiagnostic::UnreachableFromCanonicalRoot {
                phase: HillslopePhase::StorageBounds,
            }
    }));
}

#[test]
fn schedule_export_validation_reports_topological_order_drift() {
    let mut dependencies = canonical_dependency_map_for_test();
    dependencies.insert(
        HillslopePhase::Evapotranspiration,
        vec![HillslopePhase::PerennialGrowthTransition],
    );
    dependencies.insert(
        HillslopePhase::PercolationDeepSeepage,
        vec![HillslopePhase::Evapotranspiration],
    );
    let graph = HillslopePhaseGraph::from_dependencies_for_test(dependencies);
    let report = validate_hillslope_schedule_graph(&graph);

    assert!(report.diagnostics.iter().any(|diagnostic| matches!(
        diagnostic,
        ScheduleDiagnostic::CanonicalOrderMismatch { .. }
    )));
}

#[test]
fn schedule_diff_reports_added_and_removed_nodes_and_edges() {
    let base = r#"{
  "nodes": [
    {"phase": "normalization", "rank": 0, "consumer_adapter": "soil"}
  ],
  "edges": [
    {"from": "normalization", "to": "storage_bounds"}
  ],
  "topological_order": ["normalization"]
}"#;
    let head = r#"{
  "nodes": [
    {"phase": "storage_bounds", "rank": 1, "consumer_adapter": "soil"}
  ],
  "edges": [
    {"from": "storage_bounds", "to": "decomposition_transition"}
  ],
  "topological_order": ["storage_bounds"]
}"#;

    let diff = diff_schedule_json(base, head).expect("synthetic exports should diff");
    assert_eq!(diff.added_nodes, vec!["storage_bounds"]);
    assert_eq!(diff.removed_nodes, vec!["normalization"]);
    assert_eq!(diff.added_edges[0].from, "storage_bounds");
    assert_eq!(diff.added_edges[0].to, "decomposition_transition");
    assert_eq!(diff.removed_edges[0].from, "normalization");
    assert_eq!(diff.removed_edges[0].to, "storage_bounds");

    let rendered = render_schedule_diff(&diff);
    assert!(rendered.contains("Added nodes:"));
    assert!(rendered.contains("- storage_bounds"));
    assert!(rendered.contains("Removed edges:"));
    assert!(rendered.contains("- normalization -> storage_bounds"));
}
