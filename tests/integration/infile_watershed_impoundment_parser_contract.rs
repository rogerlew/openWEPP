use std::path::PathBuf;

use openwepp_input_contract::parsers::watershed_impoundment::{
    DropSpillwayPayload, EmergencySpillwayPayload, ImpWarningCode, ParseMode,
    WatershedImpoundmentParseError, WatershedImpoundmentParseOptions,
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
fn strict_mode_parses_active_structure_payload_exports() {
    let parsed = parse_watershed_impoundment_from_path(
        fixture_path("strict_valid_active_payloads.imp"),
        WatershedImpoundmentParseOptions::strict(),
    )
    .expect("strict parser should parse active-structure impoundment payloads");

    let item = &parsed.items[0];
    assert!(item.structure_flags.has_drop_spillway);
    assert!(item.structure_flags.has_culvert_1);
    assert!(item.structure_flags.has_culvert_2);
    assert!(item.structure_flags.has_rockfill);
    assert!(item.structure_flags.has_emergency_spillway);
    assert!(item.structure_flags.has_filter_barrier);
    assert!(item.structure_flags.has_perforated_riser);

    match &item.drop_spillway {
        DropSpillwayPayload::Ids1 { payload, .. } => {
            assert!((payload.diars - 0.60).abs() < 1.0e-12);
            assert!((payload.coefw - 3.20).abs() < 1.0e-12);
        }
        other => panic!("unexpected drop spillway payload variant: {other:?}"),
    }

    let culvert_1 = &item.culverts[0];
    assert_eq!(culvert_1.icv, 1);
    assert_eq!(culvert_1.ncv, 2);
    let culvert_1_params = culvert_1
        .parameters
        .as_ref()
        .expect("culvert 1 parameters must be exported");
    assert!((culvert_1_params.arcv - 0.15).abs() < 1.0e-12);
    assert!((culvert_1_params.kus - 0.90).abs() < 1.0e-12);

    let rockfill = item
        .rockfill
        .as_ref()
        .expect("rockfill payload must be exported");
    assert!((rockfill.lnrf - 1.20).abs() < 1.0e-12);

    match &item.emergency_spillway {
        EmergencySpillwayPayload::RatingCurve { payload, .. } => {
            assert!((payload.hes - 0.50).abs() < 1.0e-12);
            assert_eq!(payload.hest.len(), 3);
            assert_eq!(payload.qes.len(), 3);
            assert!((payload.qes[2] - 0.05).abs() < 1.0e-12);
        }
        other => panic!("unexpected emergency spillway variant: {other:?}"),
    }

    let filter = item
        .filter_barrier
        .as_ref()
        .expect("filter barrier payload must be exported");
    assert!((filter.vsl - 0.02).abs() < 1.0e-12);

    let riser = item
        .perforated_riser
        .as_ref()
        .expect("perforated riser payload must be exported");
    assert!((riser.hr - 0.80).abs() < 1.0e-12);
    assert!((riser.cs - 0.80).abs() < 1.0e-12);
}

#[test]
fn strict_mode_parses_ids2_drop_spillway_and_open_channel_emergency_branch() {
    let content = r"95.700
1
ids2 impoundment desc line 1
ids2 impoundment desc line 2
ids2 impoundment desc line 3
2
drop spillway ids2 structure
1.10 1.20 0.50 3.20 0.60
0.45 0.30 2.00 0.01 0.10
0.50 0.20 0.30
0 0
0 0
0
1
emergency open channel
1.00 2.00 0.04 0.50 1.10
0.01 3.00 0.02 4.00 0.03
0
0
1.20 1.00 0.80 0.50 0.01
1 5
2
0.80 120.0 24.0
0.85 0.95
130.0 150.0
26.0 30.0
";

    let parsed =
        parse_watershed_impoundment_from_str(content, WatershedImpoundmentParseOptions::strict())
            .expect("strict parser should parse ids=2 and ies=1 branch payloads");
    let item = &parsed.items[0];

    match &item.drop_spillway {
        DropSpillwayPayload::Ids2 { comment, payload } => {
            assert_eq!(comment, "drop spillway ids2 structure");
            assert!((payload.lenrs - 1.10).abs() < 1.0e-12);
            assert!((payload.widrs - 1.20).abs() < 1.0e-12);
            assert!((payload.hblot - 0.10).abs() < 1.0e-12);
        }
        other => panic!("unexpected drop spillway payload variant: {other:?}"),
    }

    match &item.emergency_spillway {
        EmergencySpillwayPayload::OpenChannel { comment, payload } => {
            assert_eq!(comment, "emergency open channel");
            assert!((payload.bwes - 1.00).abs() < 1.0e-12);
            assert!((payload.ses3 - 0.03).abs() < 1.0e-12);
        }
        other => panic!("unexpected emergency spillway variant: {other:?}"),
    }
}

#[test]
fn strict_mode_parses_ids3_drop_spillway_without_optional_branches() {
    let content = r"95.700
1
ids3 impoundment desc line 1
ids3 impoundment desc line 2
ids3 impoundment desc line 3
3
drop spillway ids3 structure
1.10 1.20 0.50 3.20 0.60
0.45 0.30 2.00 0.01 0.10 0.11
0.50 0.20 0.30
0 0
0 0
0
0
0
0
1.20 1.00 0.80 0.50 0.01
1 5
2
0.80 120.0 24.0
0.85 0.95
130.0 150.0
26.0 30.0
";

    let parsed =
        parse_watershed_impoundment_from_str(content, WatershedImpoundmentParseOptions::strict())
            .expect("strict parser should parse ids=3 branch payload");
    let item = &parsed.items[0];

    match &item.drop_spillway {
        DropSpillwayPayload::Ids3 { comment, payload } => {
            assert_eq!(comment, "drop spillway ids3 structure");
            assert!((payload.lenrs - 1.10).abs() < 1.0e-12);
            assert!((payload.hitbl - 0.45).abs() < 1.0e-12);
            assert!((payload.wdbl - 0.30).abs() < 1.0e-12);
            assert!((payload.hblot - 0.11).abs() < 1.0e-12);
        }
        other => panic!("unexpected drop spillway payload variant: {other:?}"),
    }

    assert!(matches!(
        item.emergency_spillway,
        EmergencySpillwayPayload::None
    ));
    assert_eq!(item.branch_comments, ["drop spillway ids3 structure"]);
}

#[test]
fn watershed_impoundment_parse_error_display_strings_are_stable() {
    let cases = vec![
        (
            WatershedImpoundmentParseError::InputOpenError {
                path: PathBuf::from("missing.imp"),
                source: std::io::Error::new(
                    std::io::ErrorKind::NotFound,
                    "missing impoundment input",
                ),
            },
            "IMP-E-000: could not open/read 'missing.imp': missing impoundment input",
        ),
        (
            WatershedImpoundmentParseError::TokenParseError {
                line: 7,
                field: "ids",
                token: "x".to_string(),
            },
            "IMP-E-001: line 7 token parse error for ids from 'x'",
        ),
        (
            WatershedImpoundmentParseError::UnexpectedEof {
                context: "drop_spillway.strdes",
            },
            "IMP-E-002: unexpected end of file while parsing drop_spillway.strdes",
        ),
        (
            WatershedImpoundmentParseError::UnsupportedDatver {
                line: 1,
                observed: Some(94.0),
                reason: "below supported watershed impoundment datver",
            },
            "IMP-E-003: line 1 unsupported datver 94 (below supported watershed impoundment datver)",
        ),
        (
            WatershedImpoundmentParseError::UnsupportedDatver {
                line: 1,
                observed: None,
                reason: "strict mode requires explicit datver",
            },
            "IMP-E-003: line 1 unsupported legacy no-datver preamble (strict mode requires explicit datver)",
        ),
        (
            WatershedImpoundmentParseError::DomainError {
                line: 12,
                field: "ies",
                value: "4".to_string(),
                allowed: "0|1|2",
            },
            "IMP-E-004: line 12 invalid domain value '4' for ies; expected 0|1|2",
        ),
        (
            WatershedImpoundmentParseError::BranchArityError {
                line: 13,
                context: "misc",
                expected: 5,
                found: 4,
            },
            "IMP-E-005: line 13 arity mismatch in misc; expected 5, found 4",
        ),
        (
            WatershedImpoundmentParseError::PhysicalDomainError {
                line: 14,
                field: "deltat",
                value: 0.0,
                expected: "> 0",
            },
            "IMP-E-006: line 14 physical-domain violation for deltat: 0 (> 0)",
        ),
        (
            WatershedImpoundmentParseError::CountMismatch {
                line: 2,
                declared_jpond: 3,
                expected_npond: 2,
                reason: "strict structural count mismatch",
            },
            "IMP-E-007: line 2 count mismatch jpond=3 vs npond=2 (strict structural count mismatch)",
        ),
        (
            WatershedImpoundmentParseError::InvariantViolation {
                line: 20,
                context: "stage array must be monotone non-decreasing",
            },
            "IMP-E-008: line 20 invariant violation (stage array must be monotone non-decreasing)",
        ),
        (
            WatershedImpoundmentParseError::OrderingMismatch {
                line: 21,
                context: "impoundment ordering",
            },
            "IMP-E-009: line 21 ordering mismatch (impoundment ordering)",
        ),
    ];

    for (err, expected) in cases {
        assert_eq!(err.to_string(), expected);
    }
}

#[test]
fn watershed_impoundment_parse_error_source_is_only_input_open_source() {
    let input_open = WatershedImpoundmentParseError::InputOpenError {
        path: PathBuf::from("missing.imp"),
        source: std::io::Error::new(std::io::ErrorKind::NotFound, "missing"),
    };
    assert!(std::error::Error::source(&input_open).is_some());

    let domain = WatershedImpoundmentParseError::DomainError {
        line: 1,
        field: "ids",
        value: "5".to_string(),
        allowed: "0|1|2|3",
    };
    assert!(std::error::Error::source(&domain).is_none());
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
fn strict_mode_accepts_zero_impoundments_when_structure_declares_none() {
    let options = WatershedImpoundmentParseOptions {
        mode: ParseMode::Strict,
        expected_structural_count: Some(0),
        max_impoundments: 25,
    };

    let parsed = parse_watershed_impoundment_from_path(
        fixture_path("strict_zero_impoundments.imp"),
        options,
    )
    .expect("strict parser should accept typed empty impoundment set for npond=0");

    assert_eq!(parsed.datver, Some(99.1));
    assert!(parsed.datver_explicit);
    assert_eq!(parsed.declared_count, 0);
    assert_eq!(parsed.expected_structural_count, Some(0));
    assert_eq!(parsed.parsed_count, 0);
    assert_eq!(parsed.surplus_ignored_count, 0);
    assert!(parsed.items.is_empty());
    assert!(parsed.warnings.is_empty());
}

#[test]
fn compatibility_mode_accepts_zero_impoundments_when_structure_declares_none() {
    let options = WatershedImpoundmentParseOptions {
        mode: ParseMode::Compatibility,
        expected_structural_count: Some(0),
        max_impoundments: 25,
    };

    let parsed = parse_watershed_impoundment_from_path(
        fixture_path("strict_zero_impoundments.imp"),
        options,
    )
    .expect("compatibility parser should accept typed empty impoundment set for npond=0");

    assert_eq!(parsed.declared_count, 0);
    assert_eq!(parsed.expected_structural_count, Some(0));
    assert_eq!(parsed.parsed_count, 0);
    assert!(parsed.items.is_empty());
    assert!(parsed.warnings.is_empty());
}

#[test]
fn zero_impoundments_without_structure_context_remains_domain_error() {
    let err = parse_watershed_impoundment_from_path(
        fixture_path("strict_zero_impoundments.imp"),
        WatershedImpoundmentParseOptions::strict(),
    )
    .expect_err("bare strict parse should not relax jpond=0 without npond context");

    assert!(matches!(
        err,
        WatershedImpoundmentParseError::DomainError { field: "jpond", .. }
    ));
    assert_eq!(err.contract_error_id(), "IMP-E-004");
}

#[test]
fn zero_impoundments_rejects_positive_structural_count_as_mismatch() {
    for mode in [ParseMode::Strict, ParseMode::Compatibility] {
        let options = WatershedImpoundmentParseOptions {
            mode,
            expected_structural_count: Some(1),
            max_impoundments: 25,
        };

        let err = parse_watershed_impoundment_from_path(
            fixture_path("strict_zero_impoundments.imp"),
            options,
        )
        .expect_err("jpond=0 must fail when the structure declares impoundments");

        assert!(matches!(
            err,
            WatershedImpoundmentParseError::CountMismatch {
                declared_jpond: 0,
                expected_npond: 1,
                ..
            }
        ));
        assert_eq!(err.contract_error_id(), "IMP-E-007");
    }
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
