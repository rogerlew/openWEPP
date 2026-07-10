use std::error::Error as _;
use std::path::PathBuf;

use openwepp_sim_contract::status::{BoundaryClass, SimulationPhase, StatusClassification};
use openwepp_topology::{
    TopologyParseError, TopologyValidationError, TopologyValidationReport,
    parse_topology_fixture_path, parse_topology_fixture_str, validate_pre_execution_topology,
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

fn parse_error(input: &str) -> TopologyParseError {
    parse_topology_fixture_str(input).expect_err("fixture text should fail")
}

fn report_for(input: &str) -> TopologyValidationReport {
    let graph = parse_topology_fixture_str(input).expect("fixture text should parse");
    validate_pre_execution_topology(&graph).expect("validation report should be constructed")
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
    assert!(
        error
            .to_string()
            .contains("failed reading topology fixture")
    );
    assert!(error.source().is_some());
    assert!(parse_error("").source().is_none());
}

#[test]
fn topology_validation_error_wraps_status_source() {
    let error =
        TopologyValidationError::from(openwepp_sim_contract::status::StatusError::MessageIdEmpty);

    assert_eq!(
        error.to_string(),
        "failed constructing topology status: message_id must not be empty"
    );
    assert!(error.source().is_some());
}

#[test]
fn parser_reports_required_header_and_header_format_errors() {
    let missing = parse_error(
        "\
CHANNELS 0
IMPOUNDMENTS 0
",
    );
    assert!(matches!(
        missing,
        TopologyParseError::MissingHeader { ref header } if header == "HILLSLOPES"
    ));
    assert_eq!(
        missing.to_string(),
        "missing required topology header HILLSLOPES"
    );

    let malformed = parse_error(
        "\
HILLSLOPES 1 extra
CHANNELS 0
IMPOUNDMENTS 0
",
    );
    assert!(matches!(
        malformed,
        TopologyParseError::HeaderFormat { line: 1, .. }
    ));
    assert_eq!(
        malformed.to_string(),
        "invalid topology header format at line 1: HILLSLOPES 1 extra"
    );

    let unknown = parse_error(
        "\
WATERSHEDS 1
HILLSLOPES 1
CHANNELS 0
IMPOUNDMENTS 0
",
    );
    assert!(matches!(
        unknown,
        TopologyParseError::HeaderFormat { line: 1, .. }
    ));
    assert_eq!(
        unknown.to_string(),
        "invalid topology header format at line 1: WATERSHEDS 1"
    );
}

#[test]
fn parser_reports_header_value_and_duplicate_header_errors() {
    let value = parse_error(
        "\
HILLSLOPES many
CHANNELS 0
IMPOUNDMENTS 0
",
    );
    assert!(matches!(
        value,
        TopologyParseError::HeaderValueParse {
            line: 1,
            ref header,
            ref value
        } if header == "HILLSLOPES" && value == "many"
    ));
    assert_eq!(
        value.to_string(),
        "failed parsing topology header value at line 1 for HILLSLOPES: many"
    );

    let duplicate = parse_error(
        "\
HILLSLOPES 1
HILLSLOPES 2
CHANNELS 0
IMPOUNDMENTS 0
",
    );
    assert!(matches!(
        duplicate,
        TopologyParseError::DuplicateHeader { line: 2, ref header } if header == "HILLSLOPES"
    ));
    assert_eq!(
        duplicate.to_string(),
        "duplicate topology header at line 2: HILLSLOPES"
    );
}

#[test]
fn parser_reports_node_record_and_kind_errors() {
    let short_record = parse_error(
        "\
HILLSLOPES 1
CHANNELS 1
IMPOUNDMENTS 0
NODE CHANNEL 1 H 1
",
    );
    assert!(matches!(
        short_record,
        TopologyParseError::NodeRecordFormat { line: 4, .. }
    ));
    assert_eq!(
        short_record.to_string(),
        "invalid topology node format at line 4: NODE CHANNEL 1 H 1"
    );

    let bad_markers = parse_error(
        "\
HILLSLOPES 1
CHANNELS 1
IMPOUNDMENTS 0
NODE CHANNEL 1 X 1 0 0 C 0 0 0 I 0 0 0
",
    );
    assert!(matches!(
        bad_markers,
        TopologyParseError::NodeRecordFormat { line: 4, .. }
    ));
    assert_eq!(
        bad_markers.to_string(),
        "invalid topology node format at line 4: NODE CHANNEL 1 X 1 0 0 C 0 0 0 I 0 0 0"
    );

    let unknown_kind = parse_error(
        "\
HILLSLOPES 1
CHANNELS 1
IMPOUNDMENTS 0
NODE RIDGE 1 H 1 0 0 C 0 0 0 I 0 0 0
",
    );
    assert!(matches!(
        unknown_kind,
        TopologyParseError::UnknownNodeKind { line: 4, ref value } if value == "RIDGE"
    ));
    assert_eq!(
        unknown_kind.to_string(),
        "unknown topology node kind at line 4: RIDGE"
    );

    let hillslope_node = parse_error(
        "\
HILLSLOPES 1
CHANNELS 1
IMPOUNDMENTS 0
NODE HILLSLOPE 1 H 1 0 0 C 0 0 0 I 0 0 0
",
    );
    assert!(matches!(
        hillslope_node,
        TopologyParseError::UnknownNodeKind { line: 4, ref value } if value == "HILLSLOPE"
    ));
    assert_eq!(
        hillslope_node.to_string(),
        "unknown topology node kind at line 4: HILLSLOPE"
    );
}

#[test]
fn parser_reports_node_value_and_duplicate_node_errors() {
    let id_error = parse_error(
        "\
HILLSLOPES 1
CHANNELS 1
IMPOUNDMENTS 0
NODE CHANNEL one H 1 0 0 C 0 0 0 I 0 0 0
",
    );
    assert!(matches!(
        id_error,
        TopologyParseError::NodeValueParse {
            line: 4,
            ref field,
            ref value
        } if field == "id" && value == "one"
    ));
    assert_eq!(
        id_error.to_string(),
        "invalid topology node value at line 4 for id: one"
    );

    let contributor_error = parse_error(
        "\
HILLSLOPES 1
CHANNELS 1
IMPOUNDMENTS 0
NODE CHANNEL 1 H 1 left 0 C 0 0 0 I 0 0 0
",
    );
    assert!(matches!(
        contributor_error,
        TopologyParseError::NodeValueParse {
            line: 4,
            ref field,
            ref value
        } if field == "hillslope_right" && value == "left"
    ));
    assert_eq!(
        contributor_error.to_string(),
        "invalid topology node value at line 4 for hillslope_right: left"
    );

    let duplicate = parse_error(
        "\
HILLSLOPES 1
CHANNELS 2
IMPOUNDMENTS 0
NODE CHANNEL 1 H 1 0 0 C 0 0 0 I 0 0 0
NODE CHANNEL 1 H 1 0 0 C 0 0 0 I 0 0 0
",
    );
    assert!(matches!(
        duplicate,
        TopologyParseError::DuplicateNode {
            line: 5,
            kind: openwepp_topology::TopologyNodeKind::Channel,
            id: 1
        }
    ));
    assert_eq!(
        duplicate.to_string(),
        "duplicate topology node at line 5: channel:1"
    );
}

#[test]
fn validation_reports_hillslope_and_impoundment_count_failures() {
    let hillslope_report = report_for(
        "\
HILLSLOPES 0
CHANNELS 0
IMPOUNDMENTS 0
",
    );
    assert!(has_violation(&hillslope_report, "TOPO-E-001"));
    assert!(!hillslope_report.is_valid());

    let impoundment_report = report_for(
        "\
HILLSLOPES 1
CHANNELS 0
IMPOUNDMENTS 1
",
    );
    assert!(has_violation(&impoundment_report, "TOPO-E-003"));
    assert!(!impoundment_report.is_valid());
}

#[test]
fn validation_reports_reference_domain_and_existence_failures() {
    let hillslope_domain = report_for(
        "\
HILLSLOPES 1
CHANNELS 1
IMPOUNDMENTS 0
NODE CHANNEL 1 H 2 0 0 C 0 0 0 I 0 0 0
",
    );
    assert!(has_violation(&hillslope_domain, "TOPO-E-006"));

    let channel_zero_upper_bound = report_for(
        "\
HILLSLOPES 1
CHANNELS 0
IMPOUNDMENTS 0
NODE CHANNEL 1 H 1 0 0 C 1 0 0 I 0 0 0
",
    );
    assert!(has_violation(&channel_zero_upper_bound, "TOPO-E-007"));

    let missing_channel = report_for(
        "\
HILLSLOPES 1
CHANNELS 2
IMPOUNDMENTS 0
NODE CHANNEL 1 H 1 0 0 C 2 0 0 I 0 0 0
",
    );
    assert!(has_violation(&missing_channel, "TOPO-E-009"));

    let missing_impoundment = report_for(
        "\
HILLSLOPES 1
CHANNELS 1
IMPOUNDMENTS 1
NODE CHANNEL 1 H 1 0 0 C 0 0 0 I 1 0 0
",
    );
    assert!(has_violation(&missing_impoundment, "TOPO-E-010"));
}
