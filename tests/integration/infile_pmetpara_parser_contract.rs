use std::{io, path::PathBuf};

use openwepp_input_contract::parsers::pmetpara::{
    ParseMode, PmetWarningCode, PmetparaParseError, PmetparaParseOptions, parse_pmetpara_file,
};

fn fixture_path(name: &str) -> PathBuf {
    PathBuf::from("tests/fixtures/infile/pmetpara").join(name)
}

#[test]
#[allow(clippy::too_many_lines)]
fn pmetpara_error_display_preserves_all_contract_identities() {
    let cases = [
        (
            PmetparaParseError::Io {
                path: PathBuf::from("inputs/pmetpara.txt"),
                source: io::Error::new(io::ErrorKind::PermissionDenied, "permission denied"),
            },
            "PMET-E-000",
            "PMET-E-000 failed to read pmetpara sidecar 'inputs/pmetpara.txt': permission denied",
            true,
        ),
        (
            PmetparaParseError::RequiredSidecarMissingError {
                path: PathBuf::from("inputs/missing.txt"),
            },
            "PMET-E-007",
            "PMET-E-007 required pmetpara sidecar is missing: 'inputs/missing.txt'",
            false,
        ),
        (
            PmetparaParseError::TokenParseError {
                line: 2,
                field: "kcb",
                token: "bad".to_string(),
            },
            "PMET-E-001",
            "PMET-E-001 line 2: failed to parse field 'kcb' from token 'bad'",
            false,
        ),
        (
            PmetparaParseError::RecordArityError {
                line: 3,
                expected: 5,
                found: 4,
            },
            "PMET-E-001",
            "PMET-E-001 line 3: expected 5 row token(s), found 4",
            false,
        ),
        (
            PmetparaParseError::RecordCountError {
                declared: 2,
                parsed: 1,
            },
            "PMET-E-002",
            "PMET-E-002 row-count mismatch: declared=2, parsed=1",
            false,
        ),
        (
            PmetparaParseError::FieldRangeError {
                line: 4,
                field: "rawp",
                value: "-1".to_string(),
            },
            "PMET-E-003",
            "PMET-E-003 line 4: value '-1' violates domain for 'rawp'",
            false,
        ),
        (
            PmetparaParseError::DuplicateCropKeyError {
                line: 5,
                normalized_crop_key: "CORN".to_string(),
            },
            "PMET-E-003",
            "PMET-E-003 line 5: duplicate normalized crop key 'CORN'",
            false,
        ),
        (
            PmetparaParseError::UnsupportedHeaderVariant {
                line: 1,
                token: "datver".to_string(),
            },
            "PMET-E-004",
            "PMET-E-004 line 1: unsupported pmetpara header variant token 'datver'",
            false,
        ),
        (
            PmetparaParseError::CropNameMissingError {
                crop_name: "ALFALFA".to_string(),
                normalized_crop_key: "ALFALFA".to_string(),
            },
            "PMET-E-005",
            "PMET-E-005 no PMET record found for crop 'ALFALFA' (normalized 'ALFALFA')",
            false,
        ),
        (
            PmetparaParseError::ActlnamTokenizationError {
                line: 6,
                value: "two words".to_string(),
            },
            "PMET-E-008",
            "PMET-E-008 line 6: unsupported strict actlnam tokenization 'two words'",
            false,
        ),
        (
            PmetparaParseError::InvariantViolation {
                line: 0,
                context: "lookup without records",
            },
            "PMET-E-006",
            "PMET-E-006 line 0: invariant violation: lookup without records",
            false,
        ),
    ];

    for (error, expected_id, expected_display, has_source) in cases {
        assert_eq!(error.contract_error_id(), expected_id);
        assert_eq!(error.to_string(), expected_display);
        assert_eq!(std::error::Error::source(&error).is_some(), has_source);
    }
}

#[test]
fn strict_mode_accepts_canonical_count_and_rows() {
    let options = PmetparaParseOptions::default();
    let parsed = parse_pmetpara_file(fixture_path("strict_valid.txt"), options).unwrap();

    assert!(parsed.sidecar_present);
    assert_eq!(parsed.iflget, 2);
    assert_eq!(parsed.record_count, 2);
    assert_eq!(parsed.records.len(), 2);
    assert!(parsed.line_count_closed);
    assert_eq!(parsed.records[0].crop_name, "CORN");
    assert_eq!(parsed.records[0].normalized_crop_key, "CORN");
    assert!(parsed.warnings.is_empty());
}

#[test]
fn strict_mode_rejects_datver_prefixed_header_variant() {
    let err = parse_pmetpara_file(
        fixture_path("invalid_header_datver_variant.txt"),
        PmetparaParseOptions::default(),
    )
    .unwrap_err();

    assert!(matches!(
        err,
        PmetparaParseError::UnsupportedHeaderVariant { .. }
    ));
    assert_eq!(err.contract_error_id(), "PMET-E-004");
}

#[test]
fn strict_mode_rejects_record_count_mismatch() {
    let err = parse_pmetpara_file(
        fixture_path("invalid_record_count_mismatch.txt"),
        PmetparaParseOptions::default(),
    )
    .unwrap_err();

    assert!(matches!(err, PmetparaParseError::RecordCountError { .. }));
    assert_eq!(err.contract_error_id(), "PMET-E-002");
}

#[test]
fn strict_mode_rejects_duplicate_normalized_crop_keys() {
    let err = parse_pmetpara_file(
        fixture_path("invalid_duplicate_keys.txt"),
        PmetparaParseOptions::default(),
    )
    .unwrap_err();

    assert!(matches!(
        err,
        PmetparaParseError::DuplicateCropKeyError { .. }
    ));
    assert_eq!(err.contract_error_id(), "PMET-E-003");
}

#[test]
fn strict_mode_rejects_multitoken_actlnam_form() {
    let err = parse_pmetpara_file(
        fixture_path("strict_multitoken_actlnam.txt"),
        PmetparaParseOptions::default(),
    )
    .unwrap_err();

    assert!(matches!(
        err,
        PmetparaParseError::ActlnamTokenizationError { .. }
    ));
    assert_eq!(err.contract_error_id(), "PMET-E-008");
}

#[test]
fn strict_mode_rejects_row_arity_mismatch() {
    let err = parse_pmetpara_file(
        fixture_path("invalid_row_arity.txt"),
        PmetparaParseOptions::default(),
    )
    .unwrap_err();

    assert!(matches!(err, PmetparaParseError::RecordArityError { .. }));
    assert_eq!(err.contract_error_id(), "PMET-E-001");
}

#[test]
fn compatibility_mode_normalizes_multitoken_actlnam_with_warning() {
    let options = PmetparaParseOptions {
        mode: ParseMode::Compatibility,
        require_sidecar: false,
    };
    let parsed =
        parse_pmetpara_file(fixture_path("compat_multitoken_actlnam.txt"), options).unwrap();

    assert_eq!(parsed.records.len(), 1);
    assert_eq!(parsed.records[0].description, "default_description_text");
    assert!(
        parsed
            .warnings
            .iter()
            .any(|w| w.code == PmetWarningCode::PmetW004)
    );
}

#[test]
fn compatibility_mode_truncates_long_crop_key_with_warning() {
    let options = PmetparaParseOptions {
        mode: ParseMode::Compatibility,
        require_sidecar: false,
    };
    let parsed = parse_pmetpara_file(fixture_path("compat_long_crop_name.txt"), options).unwrap();

    assert_eq!(parsed.records[0].normalized_crop_key.len(), 8);
    assert!(
        parsed
            .warnings
            .iter()
            .any(|w| w.code == PmetWarningCode::PmetW002)
    );
}

#[test]
fn strict_mode_rejects_long_crop_key_width() {
    let err = parse_pmetpara_file(
        fixture_path("compat_long_crop_name.txt"),
        PmetparaParseOptions::default(),
    )
    .unwrap_err();

    assert!(matches!(err, PmetparaParseError::FieldRangeError { .. }));
    assert_eq!(err.contract_error_id(), "PMET-E-003");
}

#[test]
fn absent_sidecar_optional_sets_iflget_one_and_warning_in_compat() {
    let options = PmetparaParseOptions {
        mode: ParseMode::Compatibility,
        require_sidecar: false,
    };
    let parsed = parse_pmetpara_file(fixture_path("does_not_exist.txt"), options).unwrap();

    assert!(!parsed.sidecar_present);
    assert_eq!(parsed.iflget, 1);
    assert_eq!(parsed.record_count, 0);
    assert!(
        parsed
            .warnings
            .iter()
            .any(|w| w.code == PmetWarningCode::PmetW001)
    );
}

#[test]
fn strict_required_mode_rejects_missing_sidecar() {
    let options = PmetparaParseOptions {
        mode: ParseMode::Strict,
        require_sidecar: true,
    };
    let err = parse_pmetpara_file(fixture_path("does_not_exist.txt"), options).unwrap_err();

    assert!(matches!(
        err,
        PmetparaParseError::RequiredSidecarMissingError { .. }
    ));
    assert_eq!(err.contract_error_id(), "PMET-E-007");
}

#[test]
fn strict_lookup_rejects_crop_name_miss() {
    let options = PmetparaParseOptions::default();
    let mut parsed = parse_pmetpara_file(fixture_path("strict_valid.txt"), options).unwrap();

    let err = parsed
        .lookup_record("ALFALFA", ParseMode::Strict)
        .unwrap_err();
    assert!(matches!(
        err,
        PmetparaParseError::CropNameMissingError { .. }
    ));
    assert_eq!(err.contract_error_id(), "PMET-E-005");
}

#[test]
fn compatibility_lookup_uses_first_row_fallback_and_marks_state() {
    let options = PmetparaParseOptions {
        mode: ParseMode::Compatibility,
        require_sidecar: false,
    };
    let mut parsed = parse_pmetpara_file(fixture_path("strict_valid.txt"), options).unwrap();

    let record = parsed
        .lookup_record("ALFALFA", ParseMode::Compatibility)
        .unwrap();

    assert_eq!(record.crop_name, "CORN");
    assert!(parsed.lookup.fallback_first_row_used);
    assert!(
        parsed
            .warnings
            .iter()
            .any(|w| w.code == PmetWarningCode::PmetW003)
    );
}
