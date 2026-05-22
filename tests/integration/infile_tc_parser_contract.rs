use std::path::PathBuf;

use openwepp_input_contract::parsers::tc::{
    TcOpenResult, TcParseError, TcParseMode, TcParseOptions, TcRunContext, TcWarningCode,
    parse_tc_from_path,
};

fn fixture_path(name: &str) -> PathBuf {
    PathBuf::from("tests/fixtures/infile/tc").join(name)
}

#[test]
fn strict_watershed_empty_sentinel_enables_tc_output() {
    let parsed = parse_tc_from_path(
        fixture_path("strict_empty.tc"),
        TcParseOptions {
            mode: TcParseMode::Strict,
            requested_tc_output: true,
            run_context: TcRunContext::Watershed,
        },
    )
    .expect("strict mode should accept present sentinel");

    assert_eq!(parsed.open_result, TcOpenResult::OpenSuccess);
    assert_eq!(parsed.luntc_requested, 1);
    assert_eq!(parsed.luntc, 1);
    assert!(parsed.tc_file_present);
    assert_eq!(parsed.payload_bytes, 0);
    assert!(!parsed.payload_nonempty);
    assert!(!parsed.payload_ignored_warning_emitted);
    assert!(!parsed.mode_divergence);
    assert!(parsed.tc_out_expected);
    assert!(parsed.warnings.is_empty());
}

#[test]
fn strict_nonempty_payload_is_content_insensitive_without_warning() {
    let parsed = parse_tc_from_path(
        fixture_path("nonempty_payload.tc"),
        TcParseOptions {
            mode: TcParseMode::Strict,
            requested_tc_output: true,
            run_context: TcRunContext::Watershed,
        },
    )
    .expect("strict mode should accept non-empty payload without parsing");

    assert_eq!(parsed.open_result, TcOpenResult::OpenSuccess);
    assert_eq!(parsed.luntc_requested, 1);
    assert_eq!(parsed.luntc, 1);
    assert!(parsed.tc_file_present);
    assert!(parsed.payload_nonempty);
    assert!(!parsed.payload_ignored_warning_emitted);
    assert!(!parsed.mode_divergence);
    assert!(parsed.tc_out_expected);
    assert!(parsed.warnings.is_empty());
}

#[test]
fn strict_missing_sentinel_is_normalized_without_warning() {
    let parsed = parse_tc_from_path(
        fixture_path("missing.tc"),
        TcParseOptions {
            mode: TcParseMode::Strict,
            requested_tc_output: true,
            run_context: TcRunContext::Watershed,
        },
    )
    .expect("strict mode should normalize optional-missing sentinel branch");

    assert_eq!(parsed.open_result, TcOpenResult::Missing);
    assert_eq!(parsed.luntc_requested, 1);
    assert_eq!(parsed.luntc, 0);
    assert!(!parsed.tc_file_present);
    assert_eq!(parsed.payload_bytes, 0);
    assert!(parsed.mode_divergence);
    assert!(!parsed.tc_out_expected);
    assert!(parsed.warnings.is_empty());
}

#[test]
fn compatibility_missing_sentinel_emits_tc_w_001() {
    let parsed = parse_tc_from_path(
        fixture_path("missing.tc"),
        TcParseOptions {
            mode: TcParseMode::Compatibility,
            requested_tc_output: true,
            run_context: TcRunContext::Watershed,
        },
    )
    .expect("compat mode should normalize missing sentinel with warning");

    assert_eq!(parsed.open_result, TcOpenResult::Missing);
    assert_eq!(parsed.luntc_requested, 1);
    assert_eq!(parsed.luntc, 0);
    assert!(parsed.mode_divergence);
    assert!(
        parsed
            .warnings
            .iter()
            .any(|warning| warning.code == TcWarningCode::TcW001)
    );
}

#[test]
fn strict_non_enoent_open_error_is_typed_tc_e_000() {
    let err = parse_tc_from_path(
        fixture_path(""),
        TcParseOptions {
            mode: TcParseMode::Strict,
            requested_tc_output: true,
            run_context: TcRunContext::Watershed,
        },
    )
    .expect_err("directory path should trigger strict typed open error");

    assert!(matches!(err, TcParseError::InputOpenError { .. }));
    assert_eq!(err.contract_error_id(), "TC-E-000");
}

#[test]
fn compatibility_non_enoent_open_error_collapses_with_tc_w_002() {
    let parsed = parse_tc_from_path(
        fixture_path(""),
        TcParseOptions {
            mode: TcParseMode::Compatibility,
            requested_tc_output: true,
            run_context: TcRunContext::Watershed,
        },
    )
    .expect("compat mode should collapse non-ENOENT open failures");

    assert_eq!(parsed.open_result, TcOpenResult::OpenErrorCollapsedCompat);
    assert_eq!(parsed.luntc_requested, 1);
    assert_eq!(parsed.luntc, 0);
    assert!(parsed.mode_divergence);
    assert!(!parsed.tc_out_expected);
    assert!(
        parsed
            .warnings
            .iter()
            .any(|warning| warning.code == TcWarningCode::TcW002)
    );
}

#[test]
fn compatibility_nonempty_payload_emits_tc_w_003_and_marker() {
    let parsed = parse_tc_from_path(
        fixture_path("nonempty_payload.tc"),
        TcParseOptions {
            mode: TcParseMode::Compatibility,
            requested_tc_output: true,
            run_context: TcRunContext::Watershed,
        },
    )
    .expect("compat mode should ignore non-empty payload body with warning");

    assert_eq!(parsed.open_result, TcOpenResult::OpenSuccess);
    assert_eq!(parsed.luntc_requested, 1);
    assert_eq!(parsed.luntc, 1);
    assert!(parsed.payload_nonempty);
    assert!(parsed.payload_ignored_warning_emitted);
    assert!(parsed.tc_out_expected);
    assert!(
        parsed
            .warnings
            .iter()
            .any(|warning| warning.code == TcWarningCode::TcW003)
    );
}

#[test]
fn hillslope_context_is_rejected_with_tc_e_001() {
    let err = parse_tc_from_path(
        fixture_path("strict_empty.tc"),
        TcParseOptions {
            mode: TcParseMode::Strict,
            requested_tc_output: true,
            run_context: TcRunContext::Hillslope,
        },
    )
    .expect_err("tc sidecar is watershed-only");

    assert!(matches!(err, TcParseError::UnsupportedRunContext { .. }));
    assert_eq!(err.contract_error_id(), "TC-E-001");
}
