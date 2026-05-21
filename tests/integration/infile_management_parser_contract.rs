#[path = "../../crates/openwepp-input-contract/src/parsers/management.rs"]
mod management;

use std::path::{Path, PathBuf};

use management::{ManagementParseError, ParseMode, parse_management_from_path};

fn fixture_path(name: &str) -> PathBuf {
    Path::new(file!())
        .parent()
        .expect("test file parent exists")
        .parent()
        .expect("tests dir exists")
        .join("fixtures")
        .join("infile")
        .join("management")
        .join(name)
}

#[test]
fn strict_mode_parses_minimal_management_fixture() {
    let parsed =
        parse_management_from_path(fixture_path("strict_minimal_ok.man"), ParseMode::Strict)
            .expect("strict minimal fixture should parse");

    assert_eq!(parsed.datver, "95.7");
    assert_eq!(parsed.topology_count, 1);
    assert_eq!(parsed.declared_total_years, 2);
    assert_eq!(parsed.schedule.rotation_repeats, 1);
    assert_eq!(parsed.schedule.rotation_years, 2);
    assert_eq!(parsed.schedule.slots.len(), 2);
}

#[test]
fn compatibility_mode_accepts_trailing_tokens_on_control_records() {
    let parsed = parse_management_from_path(
        fixture_path("compat_trailing_tokens_ok.man"),
        ParseMode::Compatibility,
    )
    .expect("compat mode should accept first token from control records");

    assert_eq!(parsed.datver, "95.7");
    assert_eq!(parsed.declared_total_years, 2);
}

#[test]
fn strict_mode_rejects_trailing_tokens_on_control_records() {
    let err = parse_management_from_path(
        fixture_path("compat_trailing_tokens_ok.man"),
        ParseMode::Strict,
    )
    .expect_err("strict mode must reject trailing tokens");

    match err {
        ManagementParseError::TokenParseError { field, .. } => {
            assert_eq!(field, "datver");
            assert_eq!(err.contract_error_id(), "MAN-E-001");
        }
        other => panic!("unexpected error variant: {other:?}"),
    }
}

#[test]
fn unsupported_datver_rejected() {
    let err = parse_management_from_path(fixture_path("unsupported_datver.man"), ParseMode::Strict)
        .expect_err("unknown datver should be rejected");

    match err {
        ManagementParseError::UnsupportedDatver { ref datver } => {
            assert_eq!(datver, "99.9");
            assert_eq!(err.contract_error_id(), "MAN-E-003");
        }
        other => panic!("unexpected error variant: {other:?}"),
    }
}

#[test]
fn missing_required_line_rejected() {
    let err =
        parse_management_from_path(fixture_path("missing_required_line.man"), ParseMode::Strict)
            .expect_err("missing record must be rejected");

    match err {
        ManagementParseError::MissingRecord { field } => {
            assert_eq!(field, "nycrop");
            assert_eq!(err.contract_error_id(), "MAN-E-002");
        }
        other => panic!("unexpected error variant: {other:?}"),
    }
}

#[test]
fn negative_count_rejected() {
    let err = parse_management_from_path(fixture_path("negative_count.man"), ParseMode::Strict)
        .expect_err("negative section count must be rejected");

    match err {
        ManagementParseError::InvalidCount { field, value } => {
            assert_eq!(field, "ncrop");
            assert_eq!(value, -1);
            assert_eq!(err.contract_error_id(), "MAN-E-005");
        }
        other => panic!("unexpected error variant: {other:?}"),
    }
}

#[test]
fn total_year_closure_enforced() {
    let err =
        parse_management_from_path(fixture_path("total_year_mismatch.man"), ParseMode::Strict)
            .expect_err("declared and derived total years must match");

    match err {
        ManagementParseError::TotalYearMismatch {
            declared_total_years,
            derived_total_years,
        } => {
            assert_eq!(declared_total_years, 3);
            assert_eq!(derived_total_years, 2);
            assert_eq!(err.contract_error_id(), "MAN-E-008");
        }
        other => panic!("unexpected error variant: {other:?}"),
    }
}

#[test]
fn nonzero_section_counts_are_currently_rejected() {
    let err = parse_management_from_path(
        fixture_path("nonzero_section_not_supported.man"),
        ParseMode::Strict,
    )
    .expect_err("this worker package currently rejects non-zero scenario sections");

    match err {
        ManagementParseError::NonZeroScenarioSectionUnsupported { section, count } => {
            assert_eq!(section, "ncrop");
            assert_eq!(count, 1);
            assert_eq!(err.contract_error_id(), "MAN-E-002");
        }
        other => panic!("unexpected error variant: {other:?}"),
    }
}

#[test]
fn open_failure_is_typed() {
    let err = parse_management_from_path(fixture_path("does_not_exist.man"), ParseMode::Strict)
        .expect_err("missing file should produce typed open error");

    match err {
        ManagementParseError::InputOpenError { .. } => {
            assert_eq!(err.contract_error_id(), "MAN-E-002");
        }
        other => panic!("unexpected error variant: {other:?}"),
    }
}
