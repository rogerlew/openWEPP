#[path = "../../crates/openwepp-input-contract/src/parsers/watershed_structure.rs"]
mod watershed_structure;

use std::path::PathBuf;

use watershed_structure::{
    DatverSource, ParseMode, WatershedStructureParseError, WatershedStructureParseOptions,
    WatershedStructureWarningCode, parse_watershed_structure_from_path,
};

fn fixture_path(name: &str) -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("tests/fixtures/infile/watershed_structure")
        .join(name)
}

#[test]
fn strict_mode_parses_valid_watershed_structure() {
    let mut options = WatershedStructureParseOptions::strict(2, 2);
    options.expected_channel_count = Some(1);
    options.expected_impoundment_count = Some(1);

    let parsed =
        parse_watershed_structure_from_path(fixture_path("strict_valid_two_rows.str"), options)
            .expect("strict mode valid .str file should parse");

    assert_eq!(parsed.datver_source, DatverSource::ExplicitHeader);
    assert!((parsed.datver - 94.301).abs() < 1e-12);
    assert_eq!(parsed.rows.len(), 2);
    assert_eq!(parsed.summary.channel_count, 1);
    assert_eq!(parsed.summary.impoundment_count, 1);
    assert_eq!(parsed.summary.max_hillslope_ref, 2);
    assert_eq!(parsed.rows[0].element_id, 3);
    assert_eq!(parsed.rows[1].element_id, 4);
    assert!(parsed.warnings.is_empty());
}

#[test]
fn compatibility_mode_accepts_legacy_no_datver_and_warns() {
    let mut options = WatershedStructureParseOptions::compatibility(2, 2);
    options.expected_channel_count = Some(1);
    options.expected_impoundment_count = Some(1);

    let parsed =
        parse_watershed_structure_from_path(fixture_path("compat_no_datver_valid.str"), options)
            .expect("compat mode should accept no-datver legacy path");

    assert_eq!(parsed.datver_source, DatverSource::LegacyCompatNoDatver);
    assert_eq!(parsed.warnings.len(), 1);
    assert_eq!(
        parsed.warnings[0].code,
        WatershedStructureWarningCode::StrW001
    );
}

#[test]
fn strict_mode_rejects_no_datver_path() {
    let err = parse_watershed_structure_from_path(
        fixture_path("compat_no_datver_valid.str"),
        WatershedStructureParseOptions::strict(2, 2),
    )
    .expect_err("strict mode must reject no-datver path");

    assert!(matches!(
        err,
        WatershedStructureParseError::LegacyNoDatverDisallowed { .. }
    ));
    assert_eq!(err.contract_error_id(), "STR-E-003");
}

#[test]
fn strict_mode_rejects_unsupported_datver() {
    let err = parse_watershed_structure_from_path(
        fixture_path("strict_unsupported_datver_invalid.str"),
        WatershedStructureParseOptions::strict(2, 2),
    )
    .expect_err("unsupported datver must fail");

    assert!(matches!(
        err,
        WatershedStructureParseError::UnsupportedDatver { .. }
    ));
    assert_eq!(err.contract_error_id(), "STR-E-003");
}

#[test]
fn strict_mode_rejects_invalid_row_arity() {
    let err = parse_watershed_structure_from_path(
        fixture_path("strict_invalid_arity_invalid.str"),
        WatershedStructureParseOptions::strict(2, 2),
    )
    .expect_err("row arity mismatch must fail");

    assert!(matches!(
        err,
        WatershedStructureParseError::RecordArityError { .. }
    ));
    assert_eq!(err.contract_error_id(), "STR-E-002");
}

#[test]
fn strict_mode_rejects_invalid_element_type_domain() {
    let err = parse_watershed_structure_from_path(
        fixture_path("strict_invalid_element_type_invalid.str"),
        WatershedStructureParseOptions::strict(2, 2),
    )
    .expect_err("invalid element type must fail");

    assert!(matches!(
        err,
        WatershedStructureParseError::ElementTypeDomainError { .. }
    ));
    assert_eq!(err.contract_error_id(), "STR-E-004");
}

#[test]
fn strict_mode_rejects_disconnected_structure_row() {
    let err = parse_watershed_structure_from_path(
        fixture_path("strict_disconnected_invalid.str"),
        WatershedStructureParseOptions::strict(2, 2),
    )
    .expect_err("disconnected row must fail");

    assert!(matches!(
        err,
        WatershedStructureParseError::DisconnectedElementError { .. }
    ));
    assert_eq!(err.contract_error_id(), "STR-E-005");
}

#[test]
fn strict_mode_rejects_invalid_hillslope_domain() {
    let err = parse_watershed_structure_from_path(
        fixture_path("strict_invalid_hillslope_domain_invalid.str"),
        WatershedStructureParseOptions::strict(2, 2),
    )
    .expect_err("invalid hillslope id domain must fail");

    assert!(matches!(
        err,
        WatershedStructureParseError::ContributorDomainError { .. }
    ));
    assert_eq!(err.contract_error_id(), "STR-E-006");
}

#[test]
fn strict_mode_rejects_invalid_upstream_reference_domain() {
    let err = parse_watershed_structure_from_path(
        fixture_path("strict_invalid_upstream_reference_invalid.str"),
        WatershedStructureParseOptions::strict(2, 2),
    )
    .expect_err("invalid channel/impoundment contributor ref must fail");

    assert!(matches!(
        err,
        WatershedStructureParseError::ContributorDomainError { .. }
    ));
    assert_eq!(err.contract_error_id(), "STR-E-006");
}

#[test]
fn strict_mode_rejects_row_count_mismatch() {
    let err = parse_watershed_structure_from_path(
        fixture_path("strict_valid_two_rows.str"),
        WatershedStructureParseOptions::strict(2, 3),
    )
    .expect_err("row-count mismatch must fail");

    assert!(matches!(
        err,
        WatershedStructureParseError::RecordCountMismatch { .. }
    ));
    assert_eq!(err.contract_error_id(), "STR-E-011");
}

#[test]
fn strict_mode_rejects_channel_count_mismatch() {
    let mut options = WatershedStructureParseOptions::strict(2, 2);
    options.expected_channel_count = Some(2);

    let err =
        parse_watershed_structure_from_path(fixture_path("strict_valid_two_rows.str"), options)
            .expect_err("channel-count mismatch must fail");

    assert!(matches!(
        err,
        WatershedStructureParseError::ChannelCountMismatch { .. }
    ));
    assert_eq!(err.contract_error_id(), "STR-E-007");
}

#[test]
fn strict_mode_rejects_impoundment_count_mismatch() {
    let mut options = WatershedStructureParseOptions::strict(2, 2);
    options.expected_impoundment_count = Some(2);

    let err =
        parse_watershed_structure_from_path(fixture_path("strict_valid_two_rows.str"), options)
            .expect_err("impoundment-count mismatch must fail");

    assert!(matches!(
        err,
        WatershedStructureParseError::ImpoundmentCountMismatch { .. }
    ));
    assert_eq!(err.contract_error_id(), "STR-E-008");
}

#[test]
fn strict_mode_rejects_hillslope_coverage_mismatch() {
    let err = parse_watershed_structure_from_path(
        fixture_path("strict_valid_two_rows.str"),
        WatershedStructureParseOptions::strict(3, 2),
    )
    .expect_err("hillslope coverage mismatch must fail");

    assert!(matches!(
        err,
        WatershedStructureParseError::HillslopeCoverageMismatch { .. }
    ));
    assert_eq!(err.contract_error_id(), "STR-E-009");
}

#[test]
fn strict_mode_rejects_missing_file_with_typed_open_error() {
    let err = parse_watershed_structure_from_path(
        fixture_path("does_not_exist.str"),
        WatershedStructureParseOptions::strict(2, 2),
    )
    .expect_err("missing file must produce typed open error");

    assert!(matches!(
        err,
        WatershedStructureParseError::InputOpenError { .. }
    ));
    assert_eq!(err.contract_error_id(), "STR-E-000");
}

#[test]
fn strict_mode_rejects_invalid_nhill_context() {
    let err = parse_watershed_structure_from_path(
        fixture_path("strict_valid_two_rows.str"),
        WatershedStructureParseOptions::strict(0, 2),
    )
    .expect_err("nhill context must be positive");

    assert!(matches!(
        err,
        WatershedStructureParseError::NhillContextError { .. }
    ));
    assert_eq!(err.contract_error_id(), "STR-E-009");
}

#[test]
fn strict_mode_requires_expected_rows_context() {
    let err = parse_watershed_structure_from_path(
        fixture_path("strict_valid_two_rows.str"),
        WatershedStructureParseOptions {
            mode: ParseMode::Strict,
            nhill: 2,
            expected_rows: None,
            expected_channel_count: None,
            expected_impoundment_count: None,
        },
    )
    .expect_err("expected_rows context is required");

    assert!(matches!(
        err,
        WatershedStructureParseError::InvariantViolation { .. }
    ));
    assert_eq!(err.contract_error_id(), "STR-E-010");
}
