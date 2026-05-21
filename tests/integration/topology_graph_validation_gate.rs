use std::path::PathBuf;

use openwepp_sim_contract::status::{BoundaryClass, SimulationPhase, StatusClassification};
use openwepp_topology::{
    TopologyParseError, TopologyValidationReport, parse_topology_fixture_path,
    validate_pre_execution_topology,
};

fn fixture_path(name: &str) -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("tests/fixtures/topology")
        .join(name)
}

fn has_violation(report: &TopologyValidationReport, message_id: &str) -> bool {
    report
        .violations
        .iter()
        .any(|violation| violation.message_id == message_id)
}

#[test]
fn canonical_topology_fixture_passes_pre_execution_validation() {
    let graph = parse_topology_fixture_path(fixture_path("canonical_valid.topo"))
        .expect("canonical fixture should parse");

    assert_eq!(graph.hillslope_count(), 3);
    assert_eq!(graph.declared_channel_count(), 2);
    assert_eq!(graph.declared_impoundment_count(), 1);
    assert_eq!(graph.observed_channel_count(), 2);
    assert_eq!(graph.observed_impoundment_count(), 1);
    assert_eq!(graph.edges().len(), 5);

    let report =
        validate_pre_execution_topology(&graph).expect("validation report should be constructed");

    assert!(report.is_valid());
    assert!(report.violations.is_empty());
    assert_eq!(
        report.status.phase(),
        SimulationPhase::PreExecutionValidation
    );
    assert_eq!(
        report.status.classification(),
        StatusClassification::Nominal
    );
    assert_eq!(report.status.boundary_class(), BoundaryClass::Ok);
}

#[test]
fn disconnected_node_fixture_fails_with_typed_violation() {
    let graph = parse_topology_fixture_path(fixture_path("invalid_disconnected.topo"))
        .expect("fixture should parse");

    let report =
        validate_pre_execution_topology(&graph).expect("validation report should be constructed");

    assert!(!report.is_valid());
    assert_eq!(
        report.status.classification(),
        StatusClassification::Failure
    );
    assert_eq!(
        report.status.boundary_class(),
        BoundaryClass::TopologyInvalid
    );
    assert!(has_violation(&report, "TOPO-E-004"));
}

#[test]
fn declared_count_mismatch_fixture_fails_with_typed_violation() {
    let graph = parse_topology_fixture_path(fixture_path("invalid_channel_count_mismatch.topo"))
        .expect("fixture should parse");

    let report =
        validate_pre_execution_topology(&graph).expect("validation report should be constructed");

    assert_eq!(
        report.status.classification(),
        StatusClassification::Failure
    );
    assert!(has_violation(&report, "TOPO-E-002"));
}

#[test]
fn out_of_domain_reference_fixture_fails_with_typed_violation() {
    let graph = parse_topology_fixture_path(fixture_path("invalid_reference_domain.topo"))
        .expect("fixture should parse");

    let report =
        validate_pre_execution_topology(&graph).expect("validation report should be constructed");

    assert_eq!(
        report.status.classification(),
        StatusClassification::Failure
    );
    assert!(has_violation(&report, "TOPO-E-007"));
}

#[test]
fn directed_cycle_fixture_fails_with_cycle_violation() {
    let graph = parse_topology_fixture_path(fixture_path("invalid_cycle.topo"))
        .expect("fixture should parse");

    let report =
        validate_pre_execution_topology(&graph).expect("validation report should be constructed");

    assert_eq!(
        report.status.classification(),
        StatusClassification::Failure
    );
    assert!(has_violation(&report, "TOPO-E-005"));
}

#[test]
fn missing_fixture_file_returns_typed_read_error() {
    let error = parse_topology_fixture_path(fixture_path("does_not_exist.topo"))
        .expect_err("missing fixture should fail");

    assert!(matches!(error, TopologyParseError::ReadError { .. }));
}
