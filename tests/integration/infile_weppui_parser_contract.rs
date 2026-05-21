use std::path::PathBuf;

use openwepp_input_contract::parsers::wepp_ui::{
    WeppUiOpenResult, WeppUiParseError, WeppUiParserMode, WeppUiParserOptions,
    WeppUiSoilCompatibilityState, WeppUiWarningCode, parse_wepp_ui_from_path,
};

fn fixture_path(name: &str) -> PathBuf {
    PathBuf::from("tests/fixtures/infile/weppui").join(name)
}

#[test]
fn strict_requested_hourly_with_empty_sentinel_and_compatible_soil_enables_hourly_mode() {
    let parsed = parse_wepp_ui_from_path(
        fixture_path("empty_wepp_ui.txt"),
        WeppUiParserOptions {
            mode: WeppUiParserMode::Strict,
            requested_hourly_seepage: true,
            soil_versions: vec![7778.0, 9002.0],
        },
    )
    .expect("strict mode should accept empty sentinel with compatible soils");

    assert_eq!(parsed.ui_run_requested, 1);
    assert_eq!(parsed.ui_run, 1);
    assert_eq!(parsed.open_result, WeppUiOpenResult::OpenSuccess);
    assert!(parsed.wepp_ui_file_present);
    assert_eq!(parsed.payload_bytes, 0);
    assert!(!parsed.payload_nonempty);
    assert_eq!(parsed.solwpv_reduced_min, Some(7778.0));
    assert_eq!(
        parsed.soil_compatibility_state,
        WeppUiSoilCompatibilityState::Compatible7778OrNewer
    );
    assert!(!parsed.mode_divergence);
    assert!(parsed.warnings.is_empty());
}

#[test]
fn strict_missing_sentinel_with_daily_request_defaults_daily_without_error() {
    let parsed = parse_wepp_ui_from_path(
        fixture_path("missing_wepp_ui.txt"),
        WeppUiParserOptions {
            mode: WeppUiParserMode::Strict,
            requested_hourly_seepage: false,
            soil_versions: vec![],
        },
    )
    .expect("missing sentinel should be valid when hourly mode is not requested");

    assert_eq!(parsed.ui_run_requested, 0);
    assert_eq!(parsed.ui_run, 0);
    assert_eq!(parsed.open_result, WeppUiOpenResult::Missing);
    assert!(!parsed.mode_divergence);
    assert!(parsed.warnings.is_empty());
}

#[test]
fn strict_missing_sentinel_with_hourly_request_errors_on_mode_closure_mismatch() {
    let err = parse_wepp_ui_from_path(
        fixture_path("missing_wepp_ui.txt"),
        WeppUiParserOptions {
            mode: WeppUiParserMode::Strict,
            requested_hourly_seepage: true,
            soil_versions: vec![7778.0],
        },
    )
    .expect_err("strict mode must not silently degrade requested hourly mode");

    assert!(matches!(
        err,
        WeppUiParseError::ModeClosureMismatch {
            ui_run_requested: 1,
            ui_run: 0,
            open_result: WeppUiOpenResult::Missing,
        }
    ));
    assert_eq!(err.contract_error_id(), "WUI-E-003");
}

#[test]
fn compatibility_missing_sentinel_with_hourly_request_warns_and_diverges() {
    let parsed = parse_wepp_ui_from_path(
        fixture_path("missing_wepp_ui.txt"),
        WeppUiParserOptions {
            mode: WeppUiParserMode::Compatibility,
            requested_hourly_seepage: true,
            soil_versions: vec![],
        },
    )
    .expect("compat mode should accept missing sentinel with warning");

    assert_eq!(parsed.open_result, WeppUiOpenResult::Missing);
    assert_eq!(parsed.ui_run_requested, 1);
    assert_eq!(parsed.ui_run, 0);
    assert!(parsed.mode_divergence);
    assert!(
        parsed
            .warnings
            .iter()
            .any(|warning| warning.code == WeppUiWarningCode::WuiW001)
    );
}

#[test]
fn strict_nonempty_sentinel_payload_is_rejected() {
    let err = parse_wepp_ui_from_path(
        fixture_path("nonempty_wepp_ui.txt"),
        WeppUiParserOptions {
            mode: WeppUiParserMode::Strict,
            requested_hourly_seepage: true,
            soil_versions: vec![7778.0],
        },
    )
    .expect_err("strict mode should reject non-empty sentinel payloads");

    assert!(matches!(
        err,
        WeppUiParseError::SentinelPayloadNotEmpty { .. }
    ));
    assert_eq!(err.contract_error_id(), "WUI-E-001");
}

#[test]
fn compatibility_nonempty_payload_is_accepted_with_warning() {
    let parsed = parse_wepp_ui_from_path(
        fixture_path("nonempty_wepp_ui.txt"),
        WeppUiParserOptions {
            mode: WeppUiParserMode::Compatibility,
            requested_hourly_seepage: true,
            soil_versions: vec![7778.0],
        },
    )
    .expect("compat mode should accept non-empty payload as ignored content");

    assert_eq!(parsed.ui_run, 1);
    assert_eq!(parsed.open_result, WeppUiOpenResult::OpenSuccess);
    assert!(parsed.payload_nonempty);
    assert!(
        parsed
            .warnings
            .iter()
            .any(|warning| warning.code == WeppUiWarningCode::WuiW002)
    );
}

#[test]
fn strict_non_enoent_open_error_is_typed() {
    let err = parse_wepp_ui_from_path(
        fixture_path(""),
        WeppUiParserOptions {
            mode: WeppUiParserMode::Strict,
            requested_hourly_seepage: true,
            soil_versions: vec![7778.0],
        },
    )
    .expect_err("directory path should trigger strict typed open error");

    assert!(matches!(err, WeppUiParseError::InputOpenError { .. }));
    assert_eq!(err.contract_error_id(), "WUI-E-000");
}

#[test]
fn compatibility_non_enoent_open_error_collapses_with_warning() {
    let parsed = parse_wepp_ui_from_path(
        fixture_path(""),
        WeppUiParserOptions {
            mode: WeppUiParserMode::Compatibility,
            requested_hourly_seepage: true,
            soil_versions: vec![],
        },
    )
    .expect("compat mode should collapse non-ENOENT open failures");

    assert_eq!(parsed.ui_run_requested, 1);
    assert_eq!(parsed.ui_run, 0);
    assert_eq!(
        parsed.open_result,
        WeppUiOpenResult::OpenErrorCollapsedCompat
    );
    assert!(parsed.mode_divergence);
    assert!(
        parsed
            .warnings
            .iter()
            .any(|warning| warning.code == WeppUiWarningCode::WuiW004)
    );
}

#[test]
fn strict_hourly_mode_with_legacy_soil_versions_is_rejected() {
    let err = parse_wepp_ui_from_path(
        fixture_path("empty_wepp_ui.txt"),
        WeppUiParserOptions {
            mode: WeppUiParserMode::Strict,
            requested_hourly_seepage: true,
            soil_versions: vec![2006.2, 7777.0],
        },
    )
    .expect_err("strict mode should reject legacy soil versions for hourly mode");

    assert!(matches!(
        err,
        WeppUiParseError::SoilCompatibilityStrict { .. }
    ));
    assert_eq!(err.contract_error_id(), "WUI-E-002");
}

#[test]
fn compatibility_hourly_mode_with_legacy_soil_versions_warns() {
    let parsed = parse_wepp_ui_from_path(
        fixture_path("empty_wepp_ui.txt"),
        WeppUiParserOptions {
            mode: WeppUiParserMode::Compatibility,
            requested_hourly_seepage: true,
            soil_versions: vec![2006.2, 7777.0],
        },
    )
    .expect("compat mode should accept legacy soil versions with warning");

    assert_eq!(
        parsed.soil_compatibility_state,
        WeppUiSoilCompatibilityState::Legacy2006
    );
    assert_eq!(parsed.solwpv_reduced_min, Some(2006.2));
    assert!(
        parsed
            .warnings
            .iter()
            .any(|warning| warning.code == WeppUiWarningCode::WuiW003)
    );
}

#[test]
fn strict_hourly_mode_without_soil_versions_is_rejected() {
    let err = parse_wepp_ui_from_path(
        fixture_path("empty_wepp_ui.txt"),
        WeppUiParserOptions {
            mode: WeppUiParserMode::Strict,
            requested_hourly_seepage: true,
            soil_versions: vec![],
        },
    )
    .expect_err("strict mode requires soil-version surface for hourly mode policy");

    assert!(matches!(
        err,
        WeppUiParseError::MissingSoilVersionSurface { .. }
    ));
    assert_eq!(err.contract_error_id(), "WUI-E-004");
}
