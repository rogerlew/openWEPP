use std::path::PathBuf;

use openwepp_input_contract::parsers::watershed_impoundment::{
    ImpWarningCode, ParseMode, WatershedImpoundmentParseError, WatershedImpoundmentParseOptions,
    parse_watershed_impoundment_from_path, parse_watershed_impoundment_from_str,
};

fn fixture_path(name: &str) -> PathBuf {
    PathBuf::from("tests/fixtures/infile/watershed_impoundment").join(name)
}

#[test]
fn strict_mode_parses_minimal_valid_impoundment() {
    let parsed = parse_watershed_impoundment_from_path(
        fixture_path("strict_valid_minimal.imp"),
        WatershedImpoundmentParseOptions::strict(),
    )
    .expect("strict parser should parse canonical impoundment file");

    assert_eq!(parsed.datver, Some(95.7));
    assert!(parsed.datver_explicit);
    assert_eq!(parsed.declared_count, 1);
    assert_eq!(parsed.parsed_count, 1);
    assert_eq!(parsed.surplus_ignored_count, 0);
    assert!(parsed.warnings.is_empty());

    let item = &parsed.items[0];
    assert_eq!(item.ids, 0);
    assert_eq!(item.culvert_icv, [0, 0]);
    assert_eq!(item.rockfill_code, 0);
    assert_eq!(item.emergency_code, 0);
    assert_eq!(item.filter_code, 0);
    assert_eq!(item.riser_code, 0);
    assert_eq!(item.nalpts, 3);
    assert_eq!(item.stage.len(), 3);
    assert_eq!(item.area.len(), 3);
    assert_eq!(item.length.len(), 3);
}

#[test]
fn strict_mode_rejects_legacy_no_datver_preamble() {
    let err = parse_watershed_impoundment_from_path(
        fixture_path("compat_legacy_no_datver.imp"),
        WatershedImpoundmentParseOptions::strict(),
    )
    .expect_err("strict mode must reject no-datver preamble");

    assert!(matches!(
        err,
        WatershedImpoundmentParseError::UnsupportedDatver { observed: None, .. }
    ));
    assert_eq!(err.contract_error_id(), "IMP-E-003");
}

#[test]
fn compatibility_mode_accepts_legacy_no_datver_with_warning() {
    let parsed = parse_watershed_impoundment_from_path(
        fixture_path("compat_legacy_no_datver.imp"),
        WatershedImpoundmentParseOptions::compatibility(),
    )
    .expect("compatibility mode should accept legacy no-datver preamble");

    assert_eq!(parsed.datver, None);
    assert!(!parsed.datver_explicit);
    assert_eq!(parsed.declared_count, 1);
    assert_eq!(parsed.parsed_count, 1);
    assert!(
        parsed
            .warnings
            .iter()
            .any(|warning| warning.code == ImpWarningCode::ImpW001)
    );
}

#[test]
fn strict_mode_rejects_datver_below_minimum() {
    let err = parse_watershed_impoundment_from_path(
        fixture_path("invalid_datver_below_min.imp"),
        WatershedImpoundmentParseOptions::strict(),
    )
    .expect_err("datver below 94.301 must be rejected");

    assert!(matches!(
        err,
        WatershedImpoundmentParseError::UnsupportedDatver {
            observed: Some(_),
            ..
        }
    ));
    assert_eq!(err.contract_error_id(), "IMP-E-003");
}

#[test]
fn strict_mode_rejects_structural_count_mismatch() {
    let options = WatershedImpoundmentParseOptions {
        mode: ParseMode::Strict,
        expected_structural_count: Some(2),
        max_impoundments: 25,
    };

    let err =
        parse_watershed_impoundment_from_path(fixture_path("strict_valid_minimal.imp"), options)
            .expect_err("strict mode requires jpond == npond when expected count provided");

    assert!(matches!(
        err,
        WatershedImpoundmentParseError::CountMismatch { .. }
    ));
    assert_eq!(err.contract_error_id(), "IMP-E-007");
}

#[test]
fn compatibility_mode_truncates_surplus_impoundments_with_warning() {
    let options = WatershedImpoundmentParseOptions {
        mode: ParseMode::Compatibility,
        expected_structural_count: Some(1),
        max_impoundments: 25,
    };

    let parsed =
        parse_watershed_impoundment_from_path(fixture_path("compat_surplus_jpond.imp"), options)
            .expect("compatibility mode should permit jpond > npond by deterministic truncation");

    assert_eq!(parsed.declared_count, 2);
    assert_eq!(parsed.parsed_count, 1);
    assert_eq!(parsed.surplus_ignored_count, 1);
    assert!(
        parsed
            .warnings
            .iter()
            .any(|warning| warning.code == ImpWarningCode::ImpW002)
    );
}

#[test]
fn ies_rating_branch_missing_qes_vector_is_typed_eof_error() {
    let err = parse_watershed_impoundment_from_path(
        fixture_path("invalid_ies2_missing_qes.imp"),
        WatershedImpoundmentParseOptions::strict(),
    )
    .expect_err("ies=2 branch must require both hest and qes vectors");

    assert!(matches!(
        err,
        WatershedImpoundmentParseError::UnexpectedEof { .. }
    ));
    assert_eq!(err.contract_error_id(), "IMP-E-002");
}

#[test]
fn invalid_drop_spillway_domain_is_typed() {
    let err = parse_watershed_impoundment_from_path(
        fixture_path("invalid_ids_domain.imp"),
        WatershedImpoundmentParseOptions::strict(),
    )
    .expect_err("ids outside 0|1|2|3 must be rejected");

    assert!(matches!(
        err,
        WatershedImpoundmentParseError::DomainError { .. }
    ));
    assert_eq!(err.contract_error_id(), "IMP-E-004");
}

#[test]
fn non_monotone_stage_curve_is_typed_invariant_error() {
    let err = parse_watershed_impoundment_from_path(
        fixture_path("invalid_non_monotone_stage.imp"),
        WatershedImpoundmentParseOptions::strict(),
    )
    .expect_err("stage array must be monotone non-decreasing");

    assert!(matches!(
        err,
        WatershedImpoundmentParseError::InvariantViolation { .. }
    ));
    assert_eq!(err.contract_error_id(), "IMP-E-008");
}

#[test]
fn negative_initial_timestep_is_typed_physical_domain_error() {
    let err = parse_watershed_impoundment_from_path(
        fixture_path("invalid_negative_deltat.imp"),
        WatershedImpoundmentParseOptions::strict(),
    )
    .expect_err("negative deltat must be rejected");

    assert!(matches!(
        err,
        WatershedImpoundmentParseError::PhysicalDomainError {
            field: "deltat",
            ..
        }
    ));
    assert_eq!(err.contract_error_id(), "IMP-E-006");
}

#[test]
fn oversupplied_curve_vector_is_typed_branch_arity_error() {
    let err = parse_watershed_impoundment_from_path(
        fixture_path("invalid_area_overshoot.imp"),
        WatershedImpoundmentParseOptions::strict(),
    )
    .expect_err("vector oversupply should trigger branch-arity closure");

    assert!(matches!(
        err,
        WatershedImpoundmentParseError::BranchArityError { .. }
    ));
    assert_eq!(err.contract_error_id(), "IMP-E-005");
}

#[test]
fn missing_file_is_typed_open_error() {
    let err = parse_watershed_impoundment_from_path(
        fixture_path("missing.imp"),
        WatershedImpoundmentParseOptions::strict(),
    )
    .expect_err("missing file should surface as typed open error");

    assert!(matches!(
        err,
        WatershedImpoundmentParseError::InputOpenError { .. }
    ));
    assert_eq!(err.contract_error_id(), "IMP-E-000");
}

#[test]
fn token_parse_error_maps_to_imp_e_001() {
    let content = "95.700\nX\n";
    let err =
        parse_watershed_impoundment_from_str(content, WatershedImpoundmentParseOptions::strict())
            .expect_err("non-numeric jpond token must be typed token-parse error");

    assert!(matches!(
        err,
        WatershedImpoundmentParseError::TokenParseError { .. }
    ));
    assert_eq!(err.contract_error_id(), "IMP-E-001");
}
