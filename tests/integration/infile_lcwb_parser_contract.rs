use std::path::PathBuf;

use openwepp_input_contract::parsers::lcwb::{
    LcwbOfeRowSelectionPolicyMode, LcwbOpenResult, LcwbParseError, LcwbParserMode,
    LcwbParserOptions, LcwbRunContext, LcwbWarningCode, parse_lcwb_from_path,
};

fn fixture_path(name: &str) -> PathBuf {
    PathBuf::from("tests/fixtures/infile/lcwb").join(name)
}

#[test]
fn strict_watershed_open_success_with_empty_sentinel_enables_lcwb_mode() {
    let parsed = parse_lcwb_from_path(
        fixture_path("empty_lcwb.txt"),
        LcwbParserOptions {
            mode: LcwbParserMode::Strict,
            run_context: LcwbRunContext::Watershed,
            requested_channel_watbal_mode: true,
        },
    )
    .expect("strict watershed mode should accept empty sentinel");

    assert_eq!(parsed.lcwb_requested, 1);
    assert_eq!(parsed.lcwbflg, 1);
    assert_eq!(parsed.open_result, LcwbOpenResult::OpenSuccess);
    assert!(parsed.lcwb_file_present);
    assert_eq!(parsed.payload_bytes, 0);
    assert!(!parsed.payload_nonempty);
    assert!(!parsed.payload_nonwhitespace);
    assert!(!parsed.mode_divergence);
    assert_eq!(
        parsed.ofe_row_selection_policy_mode,
        LcwbOfeRowSelectionPolicyMode::LastOfeOnly
    );
    assert!(parsed.warnings.is_empty());
}

#[test]
fn strict_watershed_accepts_whitespace_only_payload() {
    let parsed = parse_lcwb_from_path(
        fixture_path("whitespace_lcwb.txt"),
        LcwbParserOptions {
            mode: LcwbParserMode::Strict,
            run_context: LcwbRunContext::Watershed,
            requested_channel_watbal_mode: true,
        },
    )
    .expect("strict sentinel policy should allow whitespace-only payload");

    assert!(parsed.lcwb_file_present);
    assert!(parsed.payload_nonempty);
    assert!(!parsed.payload_nonwhitespace);
    assert_eq!(parsed.lcwbflg, 1);
    assert!(parsed.warnings.is_empty());
}

#[test]
fn strict_nonwhitespace_payload_is_rejected() {
    let err = parse_lcwb_from_path(
        fixture_path("nonempty_payload_lcwb.txt"),
        LcwbParserOptions {
            mode: LcwbParserMode::Strict,
            run_context: LcwbRunContext::Watershed,
            requested_channel_watbal_mode: true,
        },
    )
    .expect_err("strict mode must reject non-whitespace payload");

    assert_eq!(err.contract_error_id(), "LCWB-E-001");
    assert!(matches!(
        err,
        LcwbParseError::SentinelPayloadNotEmpty { .. }
    ));
}

#[test]
fn compatibility_nonempty_payload_is_accepted_with_warning() {
    let parsed = parse_lcwb_from_path(
        fixture_path("nonempty_payload_lcwb.txt"),
        LcwbParserOptions {
            mode: LcwbParserMode::Compatibility,
            run_context: LcwbRunContext::Watershed,
            requested_channel_watbal_mode: true,
        },
    )
    .expect("compat mode should accept payload and ignore body");

    assert_eq!(parsed.open_result, LcwbOpenResult::OpenSuccess);
    assert_eq!(parsed.lcwbflg, 1);
    assert!(parsed.payload_nonempty);
    assert!(parsed.payload_nonwhitespace);
    assert!(parsed.payload_ignored_warning_emitted);
    assert!(
        parsed
            .warnings
            .iter()
            .any(|warning| warning.code == LcwbWarningCode::LcwbW002)
    );
}

#[test]
fn strict_missing_sentinel_without_request_is_valid_missing_branch() {
    let parsed = parse_lcwb_from_path(
        fixture_path("missing_lcwb.txt"),
        LcwbParserOptions {
            mode: LcwbParserMode::Strict,
            run_context: LcwbRunContext::Watershed,
            requested_channel_watbal_mode: false,
        },
    )
    .expect("strict mode should accept missing sentinel when not requested");

    assert_eq!(parsed.lcwb_requested, 0);
    assert_eq!(parsed.lcwbflg, 0);
    assert_eq!(parsed.open_result, LcwbOpenResult::Missing);
    assert!(!parsed.lcwb_file_present);
    assert!(!parsed.mode_divergence);
    assert_eq!(
        parsed.ofe_row_selection_policy_mode,
        LcwbOfeRowSelectionPolicyMode::AllOfe
    );
    assert!(parsed.warnings.is_empty());
}

#[test]
fn strict_missing_sentinel_with_requested_mode_errors() {
    let err = parse_lcwb_from_path(
        fixture_path("missing_lcwb.txt"),
        LcwbParserOptions {
            mode: LcwbParserMode::Strict,
            run_context: LcwbRunContext::Watershed,
            requested_channel_watbal_mode: true,
        },
    )
    .expect_err("strict mode must not silently collapse requested mode");

    assert_eq!(err.contract_error_id(), "LCWB-E-003");
    assert!(matches!(
        err,
        LcwbParseError::ModeClosureMismatch {
            lcwb_requested: 1,
            lcwbflg: 0,
            open_result: LcwbOpenResult::Missing,
        }
    ));
}

#[test]
fn compatibility_missing_sentinel_warns_and_records_divergence() {
    let parsed = parse_lcwb_from_path(
        fixture_path("missing_lcwb.txt"),
        LcwbParserOptions {
            mode: LcwbParserMode::Compatibility,
            run_context: LcwbRunContext::Watershed,
            requested_channel_watbal_mode: true,
        },
    )
    .expect("compat mode should collapse to missing default branch");

    assert_eq!(parsed.lcwb_requested, 1);
    assert_eq!(parsed.lcwbflg, 0);
    assert_eq!(parsed.open_result, LcwbOpenResult::Missing);
    assert!(parsed.mode_divergence);
    assert!(
        parsed
            .warnings
            .iter()
            .any(|warning| warning.code == LcwbWarningCode::LcwbW001)
    );
}

#[test]
fn strict_non_enoent_open_error_is_typed() {
    let err = parse_lcwb_from_path(
        fixture_path(""),
        LcwbParserOptions {
            mode: LcwbParserMode::Strict,
            run_context: LcwbRunContext::Watershed,
            requested_channel_watbal_mode: true,
        },
    )
    .expect_err("strict mode should type non-ENOENT open errors");

    assert_eq!(err.contract_error_id(), "LCWB-E-000");
    assert!(matches!(err, LcwbParseError::InputOpenError { .. }));
}

#[test]
fn compatibility_non_enoent_open_error_collapses_with_warning() {
    let parsed = parse_lcwb_from_path(
        fixture_path(""),
        LcwbParserOptions {
            mode: LcwbParserMode::Compatibility,
            run_context: LcwbRunContext::Watershed,
            requested_channel_watbal_mode: true,
        },
    )
    .expect("compat mode should collapse non-ENOENT open failures");

    assert_eq!(parsed.open_result, LcwbOpenResult::OpenErrorCollapsedCompat);
    assert_eq!(parsed.lcwbflg, 0);
    assert!(parsed.mode_divergence);
    assert!(
        parsed
            .warnings
            .iter()
            .any(|warning| warning.code == LcwbWarningCode::LcwbW003)
    );
    assert!(
        parsed
            .warnings
            .iter()
            .any(|warning| warning.code == LcwbWarningCode::LcwbW001)
    );
}

#[test]
fn strict_non_watershed_context_is_rejected() {
    let err = parse_lcwb_from_path(
        fixture_path("empty_lcwb.txt"),
        LcwbParserOptions {
            mode: LcwbParserMode::Strict,
            run_context: LcwbRunContext::Hillslope,
            requested_channel_watbal_mode: true,
        },
    )
    .expect_err("strict mode must reject non-watershed context");

    assert_eq!(err.contract_error_id(), "LCWB-E-002");
    assert!(matches!(err, LcwbParseError::UnsupportedRunContext { .. }));
}

#[test]
fn compatibility_non_watershed_context_is_not_applicable_branch() {
    let parsed = parse_lcwb_from_path(
        fixture_path("empty_lcwb.txt"),
        LcwbParserOptions {
            mode: LcwbParserMode::Compatibility,
            run_context: LcwbRunContext::Hillslope,
            requested_channel_watbal_mode: true,
        },
    )
    .expect("compat mode should expose non-watershed not-applicable branch");

    assert_eq!(parsed.open_result, LcwbOpenResult::NotApplicableCompat);
    assert_eq!(parsed.lcwbflg, 0);
    assert!(parsed.mode_divergence);
    assert!(
        parsed
            .warnings
            .iter()
            .any(|warning| warning.code == LcwbWarningCode::LcwbW004)
    );
}

#[test]
fn w4dr_001_legacy_source_authority_presence_controls_lcwbflg() {
    let missing = parse_lcwb_from_path(
        fixture_path("missing_lcwb.txt"),
        LcwbParserOptions {
            mode: LcwbParserMode::Compatibility,
            run_context: LcwbRunContext::Watershed,
            requested_channel_watbal_mode: false,
        },
    )
    .expect("missing branch should parse");
    assert_eq!(missing.lcwbflg, 0);

    let present = parse_lcwb_from_path(
        fixture_path("empty_lcwb.txt"),
        LcwbParserOptions {
            mode: LcwbParserMode::Compatibility,
            run_context: LcwbRunContext::Watershed,
            requested_channel_watbal_mode: false,
        },
    )
    .expect("present branch should parse");
    assert_eq!(present.lcwbflg, 1);
}

#[test]
fn w4dr_003_and_w4dr_011_policy_projection_is_derived_not_payload_parsed() {
    let from_empty = parse_lcwb_from_path(
        fixture_path("empty_lcwb.txt"),
        LcwbParserOptions {
            mode: LcwbParserMode::Compatibility,
            run_context: LcwbRunContext::Watershed,
            requested_channel_watbal_mode: true,
        },
    )
    .expect("empty sentinel should parse");

    let from_payload = parse_lcwb_from_path(
        fixture_path("nonempty_payload_lcwb.txt"),
        LcwbParserOptions {
            mode: LcwbParserMode::Compatibility,
            run_context: LcwbRunContext::Watershed,
            requested_channel_watbal_mode: true,
        },
    )
    .expect("non-empty payload should parse in compatibility");

    assert_eq!(from_empty.lcwbflg, 1);
    assert_eq!(from_payload.lcwbflg, 1);
    assert_eq!(
        from_empty.ofe_row_selection_policy_mode,
        LcwbOfeRowSelectionPolicyMode::LastOfeOnly
    );
    assert_eq!(
        from_payload.ofe_row_selection_policy_mode,
        LcwbOfeRowSelectionPolicyMode::LastOfeOnly
    );
}
