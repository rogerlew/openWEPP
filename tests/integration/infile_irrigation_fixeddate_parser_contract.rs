use std::error::Error as _;
use std::io;
use std::path::PathBuf;

use openwepp_input_contract::parsers::irrigation_fixeddate::{
    DatverSource, FixedDateEvent, FixedDateIrrigationFile, FixedDateParseError, FixedDateWarning,
    FixedDateWarningCode, FurrowEvent, FurrowSurge, IryrInterpretationMode, Line3Record, ParseMode,
    SprinklerEvent, parse_fixeddate_file, parse_fixeddate_str,
};

fn fixture_path(name: &str) -> PathBuf {
    PathBuf::from("tests/fixtures/infile/irrigation_fixeddate").join(name)
}

fn line3(ofeflg: usize, irday: usize, iryr: usize) -> Line3Record {
    Line3Record {
        ofeflg,
        irday,
        iryr,
        schedule_termination_flag: irday == 0,
        legacy_ordering_warning_emitted: false,
    }
}

fn assert_non_finite_field_rejected(input: &str, mode: ParseMode, expected_field: &'static str) {
    let err = parse_fixeddate_str(input, mode)
        .expect_err("non-finite real tokens must not produce typed parser output");
    assert_eq!(err.contract_error_id(), "FDIR-E-005");
    match err {
        FixedDateParseError::FieldRangeError { field, value, .. } => {
            assert_eq!(field, expected_field);
            assert!(!value.is_finite());
        }
        other => panic!("expected FieldRangeError for {expected_field}, got {other:?}"),
    }
}

#[test]
fn every_real_field_rejects_nan_and_infinities_in_both_modes() {
    const NON_FINITE: [&str; 3] = ["NaN", "inf", "-inf"];
    const MODES: [ParseMode; 2] = [ParseMode::Strict, ParseMode::Compatibility];

    for mode in MODES {
        for token in NON_FINITE {
            let input = format!("{token}\n1 1 2\n1 120 1\n0.1 0.2 1.0\n1 0 0\n");
            assert_non_finite_field_rejected(&input, mode, "datver");

            for (field, row) in [
                ("irint", format!("{token} 0.2 1.0")),
                ("irdept", format!("0.1 {token} 1.0")),
                ("nozzle", format!("0.1 0.2 {token}")),
            ] {
                let input = format!("95.7\n1 1 2\n1 120 1\n{row}\n1 0 0\n");
                assert_non_finite_field_rejected(&input, mode, field);
            }

            for (field, row) in [
                ("qspply", format!("{token} 0.0 10.0 1.0")),
                ("tstart", format!("0.1 {token} 10.0 1.0")),
                ("tend", format!("0.1 0.0 {token} 1.0")),
                ("tdepl", format!("0.1 0.0 10.0 {token}")),
            ] {
                let input = format!("95.7\n1 2 2\n1 120 1\n1\n{row}\n1 0 0\n");
                assert_non_finite_field_rejected(&input, mode, field);
            }
        }
    }
}

#[test]
fn strict_mode_parses_valid_sprinkler_fixture() {
    let parsed = parse_fixeddate_file(
        fixture_path("strict_valid_sprinkler.ifd"),
        ParseMode::Strict,
    )
    .expect("strict parser should accept canonical sprinkler fixture");

    assert!((parsed.datver - 95.7).abs() < 1e-9);
    assert_eq!(parsed.datver_source, DatverSource::ExplicitHeader);
    assert_eq!(parsed.itemp, 2);
    assert_eq!(parsed.jtemp, 1);
    assert_eq!(parsed.ktemp, 2);
    assert_eq!(parsed.initial_records.len(), 2);
    assert_eq!(parsed.events.len(), 2);
    assert!(parsed.warnings.is_empty());
    assert_eq!(
        parsed.iryr_interpretation_mode,
        IryrInterpretationMode::UnresolvedRequiresRuntimePolicy
    );

    match &parsed.events[0] {
        FixedDateEvent::Sprinkler(event) => {
            assert!(!event.legacy_nozzle_default_applied);
            assert_eq!(event.next_record.ofeflg, 1);
            assert!(event.next_record.schedule_termination_flag);
        }
        FixedDateEvent::Furrow(_) => panic!("expected sprinkler event"),
    }

    assert_eq!(
        parsed,
        FixedDateIrrigationFile {
            datver: 95.7,
            datver_source: DatverSource::ExplicitHeader,
            itemp: 2,
            jtemp: 1,
            ktemp: 2,
            initial_records: vec![line3(1, 120, 1), line3(2, 130, 1)],
            events: vec![
                FixedDateEvent::Sprinkler(SprinklerEvent {
                    irint: 0.000_020,
                    irdept: 0.0120,
                    nozzle: 1.0,
                    legacy_nozzle_default_applied: false,
                    next_record: line3(1, 0, 0),
                }),
                FixedDateEvent::Sprinkler(SprinklerEvent {
                    irint: 0.000_018,
                    irdept: 0.0100,
                    nozzle: 1.1,
                    legacy_nozzle_default_applied: false,
                    next_record: line3(2, 0, 0),
                }),
            ],
            initial_dates_complete: true,
            event_stream_complete: true,
            iryr_interpretation_mode: IryrInterpretationMode::UnresolvedRequiresRuntimePolicy,
            warnings: vec![],
        }
    );
}

#[test]
fn strict_mode_parses_valid_furrow_fixture() {
    let parsed = parse_fixeddate_file(fixture_path("strict_valid_furrow.ifd"), ParseMode::Strict)
        .expect("strict parser should accept canonical furrow fixture");

    assert_eq!(parsed.jtemp, 2);
    assert_eq!(parsed.events.len(), 1);
    match &parsed.events[0] {
        FixedDateEvent::Furrow(event) => {
            assert_eq!(event.surges, 2);
            assert_eq!(event.rows.len(), 2);
            assert!(event.rows[0].tdepl.is_some());
            assert_eq!(event.rows[0].legacy_line5_arity, 4);
        }
        FixedDateEvent::Sprinkler(_) => panic!("expected furrow event"),
    }

    assert_eq!(
        parsed,
        FixedDateIrrigationFile {
            datver: 95.7,
            datver_source: DatverSource::ExplicitHeader,
            itemp: 1,
            jtemp: 2,
            ktemp: 2,
            initial_records: vec![line3(1, 100, 1)],
            events: vec![FixedDateEvent::Furrow(FurrowEvent {
                surges: 2,
                rows: vec![
                    FurrowSurge {
                        qspply: 0.000_30,
                        tstart: 0.0,
                        tend: 1800.0,
                        tdepl: Some(600.0),
                        legacy_line5_arity: 4,
                    },
                    FurrowSurge {
                        qspply: 0.000_15,
                        tstart: 1800.0,
                        tend: 3600.0,
                        tdepl: Some(300.0),
                        legacy_line5_arity: 4,
                    },
                ],
                next_record: line3(1, 0, 0),
            })],
            initial_dates_complete: true,
            event_stream_complete: true,
            iryr_interpretation_mode: IryrInterpretationMode::UnresolvedRequiresRuntimePolicy,
            warnings: vec![],
        }
    );
}

#[test]
fn compatibility_mode_accepts_no_datver_and_missing_nozzle() {
    let parsed = parse_fixeddate_file(
        fixture_path("compat_no_datver_missing_nozzle.ifd"),
        ParseMode::Compatibility,
    )
    .expect("compat mode should accept no-datver and missing nozzle branch");

    assert_eq!(parsed.datver_source, DatverSource::LegacyCompatNoDatver);
    assert_eq!(parsed.warnings.len(), 2);
    let warning_codes: Vec<&str> = parsed.warnings.iter().map(|w| w.code.as_str()).collect();
    assert!(warning_codes.contains(&FixedDateWarningCode::FdirW001.as_str()));
    assert!(warning_codes.contains(&FixedDateWarningCode::FdirW003.as_str()));

    match &parsed.events[0] {
        FixedDateEvent::Sprinkler(event) => {
            assert!(event.legacy_nozzle_default_applied);
            assert!((event.nozzle - 1.0).abs() < 1e-9);
        }
        FixedDateEvent::Furrow(_) => panic!("expected sprinkler event"),
    }

    assert_eq!(
        parsed,
        FixedDateIrrigationFile {
            datver: 95.7,
            datver_source: DatverSource::LegacyCompatNoDatver,
            itemp: 1,
            jtemp: 1,
            ktemp: 2,
            initial_records: vec![line3(1, 150, 1)],
            events: vec![FixedDateEvent::Sprinkler(SprinklerEvent {
                irint: 0.000_020,
                irdept: 0.0120,
                nozzle: 1.0,
                legacy_nozzle_default_applied: true,
                next_record: line3(1, 0, 0),
            })],
            initial_dates_complete: true,
            event_stream_complete: true,
            iryr_interpretation_mode: IryrInterpretationMode::UnresolvedRequiresRuntimePolicy,
            warnings: vec![
                FixedDateWarning {
                    code: FixedDateWarningCode::FdirW001,
                    line: 1,
                    message: "legacy no-datver branch accepted".to_string(),
                },
                FixedDateWarning {
                    code: FixedDateWarningCode::FdirW003,
                    line: 3,
                    message: "legacy sprinkler two-field row accepted; nozzle defaulted to 1.0"
                        .to_string(),
                },
            ],
        }
    );
}

#[test]
fn strict_mode_rejects_no_datver_compatibility_branch() {
    let err = parse_fixeddate_file(
        fixture_path("compat_no_datver_missing_nozzle.ifd"),
        ParseMode::Strict,
    )
    .expect_err("strict mode should reject legacy no-datver branch");

    assert!(matches!(
        err,
        FixedDateParseError::LegacyNoDatverDisallowed { .. }
    ));
    assert_eq!(err.contract_error_id(), "FDIR-E-003");
}

#[test]
fn strict_mode_rejects_legacy_explicit_datver() {
    let err = parse_fixeddate_file(fixture_path("compat_legacy_datver.ifd"), ParseMode::Strict)
        .expect_err("strict mode should reject pre-95.7 datver");

    assert!(matches!(err, FixedDateParseError::UnsupportedDatver { .. }));
    assert_eq!(err.contract_error_id(), "FDIR-E-003");
}

#[test]
fn compatibility_mode_accepts_legacy_explicit_datver_with_warning() {
    let parsed = parse_fixeddate_file(
        fixture_path("compat_legacy_datver.ifd"),
        ParseMode::Compatibility,
    )
    .expect("compat mode should accept legacy explicit datver");

    let warning_codes: Vec<&str> = parsed.warnings.iter().map(|w| w.code.as_str()).collect();
    assert!(warning_codes.contains(&FixedDateWarningCode::FdirW002.as_str()));
}

#[test]
fn strict_mode_rejects_legacy_furrow_line5_arity() {
    let err = parse_fixeddate_file(
        fixture_path("compat_furrow_legacy_line5_arity.ifd"),
        ParseMode::Strict,
    )
    .expect_err("strict mode should reject furrow line5 arity=3");

    assert!(matches!(err, FixedDateParseError::RecordArityError { .. }));
    assert_eq!(err.contract_error_id(), "FDIR-E-002");
}

#[test]
fn compatibility_mode_accepts_legacy_furrow_line5_arity_with_warning() {
    let parsed = parse_fixeddate_file(
        fixture_path("compat_furrow_legacy_line5_arity.ifd"),
        ParseMode::Compatibility,
    )
    .expect("compat mode should accept furrow line5 arity=3");

    let warning_codes: Vec<&str> = parsed.warnings.iter().map(|w| w.code.as_str()).collect();
    assert!(warning_codes.contains(&FixedDateWarningCode::FdirW004.as_str()));
    match &parsed.events[0] {
        FixedDateEvent::Furrow(event) => {
            assert_eq!(event.rows[0].legacy_line5_arity, 3);
            assert!(event.rows[0].tdepl.is_none());
        }
        FixedDateEvent::Sprinkler(_) => panic!("expected furrow event"),
    }
}

#[test]
fn header_domain_violation_is_typed() {
    let err = parse_fixeddate_file(
        fixture_path("malformed_header_ktemp.ifd"),
        ParseMode::Strict,
    )
    .expect_err("invalid header domains should be rejected");

    assert!(matches!(err, FixedDateParseError::HeaderDomainError { .. }));
    assert_eq!(err.contract_error_id(), "FDIR-E-004");
}

#[test]
fn furrow_surge_range_violation_is_typed() {
    let err = parse_fixeddate_file(
        fixture_path("malformed_furrow_surge_count.ifd"),
        ParseMode::Strict,
    )
    .expect_err("surges outside 1..20 should be rejected");

    assert!(matches!(err, FixedDateParseError::FieldRangeError { .. }));
    assert_eq!(err.contract_error_id(), "FDIR-E-005");
}

#[test]
fn strict_mode_rejects_initial_ordering_anomaly() {
    let err = parse_fixeddate_file(
        fixture_path("malformed_ordering_initial.ifd"),
        ParseMode::Strict,
    )
    .expect_err("strict mode must reject OFE ordering anomalies");

    assert!(matches!(
        err,
        FixedDateParseError::OrderingConstraintError { .. }
    ));
    assert_eq!(err.contract_error_id(), "FDIR-E-010");
}

#[test]
fn compatibility_mode_accepts_initial_ordering_anomaly_with_warning() {
    let parsed = parse_fixeddate_file(
        fixture_path("malformed_ordering_initial.ifd"),
        ParseMode::Compatibility,
    )
    .expect("compat mode should accept ordering anomaly with warning");

    let warning_codes: Vec<&str> = parsed.warnings.iter().map(|w| w.code.as_str()).collect();
    assert!(warning_codes.contains(&FixedDateWarningCode::FdirW006.as_str()));
    assert!(parsed.initial_records[0].legacy_ordering_warning_emitted);
}

#[test]
fn missing_event_successor_line3_is_typed_as_closure_error() {
    let err = parse_fixeddate_file(
        fixture_path("malformed_event_stream_missing_next_line3.ifd"),
        ParseMode::Strict,
    )
    .expect_err("event stream must include successor line3");

    assert!(matches!(
        err,
        FixedDateParseError::EventStreamClosureError { .. }
    ));
    assert_eq!(err.contract_error_id(), "FDIR-E-008");
}

#[test]
fn missing_file_is_typed_open_error() {
    let err = parse_fixeddate_file(fixture_path("does_not_exist.ifd"), ParseMode::Strict)
        .expect_err("missing input should return typed open error");

    assert!(matches!(err, FixedDateParseError::InputOpenError { .. }));
    assert_eq!(err.contract_error_id(), "FDIR-E-000");
}

#[test]
fn warning_codes_are_stable_and_complete() {
    let codes = [
        FixedDateWarningCode::FdirW001,
        FixedDateWarningCode::FdirW002,
        FixedDateWarningCode::FdirW003,
        FixedDateWarningCode::FdirW004,
        FixedDateWarningCode::FdirW005,
        FixedDateWarningCode::FdirW006,
    ];
    let rendered: Vec<&str> = codes
        .into_iter()
        .map(FixedDateWarningCode::as_str)
        .collect();
    assert_eq!(
        rendered,
        [
            "FDIR-W-001",
            "FDIR-W-002",
            "FDIR-W-003",
            "FDIR-W-004",
            "FDIR-W-005",
            "FDIR-W-006",
        ]
    );
}

#[test]
fn error_ids_display_and_sources_cover_the_public_error_contract() {
    let io_source = io::Error::new(io::ErrorKind::PermissionDenied, "denied");
    let errors = [
        FixedDateParseError::InputOpenError {
            path: PathBuf::from("blocked.ifd"),
            source: io_source,
        },
        FixedDateParseError::MissingRecord {
            line: 7,
            context: "header",
        },
        FixedDateParseError::TokenParseError {
            line: 8,
            field: "itemp",
            value: "x".to_string(),
        },
        FixedDateParseError::RecordArityError {
            line: 9,
            context: "line3",
            expected: "3",
            observed: 2,
        },
        FixedDateParseError::LegacyNoDatverDisallowed { line: 10 },
        FixedDateParseError::UnsupportedDatver {
            line: 11,
            datver: 90.0,
            jtemp: 1,
        },
        FixedDateParseError::HeaderDomainError {
            line: 12,
            field: "ktemp",
            value: 3,
            expected: "2",
        },
        FixedDateParseError::FieldRangeError {
            line: 13,
            field: "irday",
            value: 367.0,
            expected: "0..366",
        },
        FixedDateParseError::OrderingConstraintError {
            line: 14,
            phase: "event",
            expected_ofe: 1,
            observed_ofe: 2,
        },
        FixedDateParseError::EventStreamClosureError {
            line: 15,
            context: "successor",
        },
    ];

    let expected_ids = [
        "FDIR-E-000",
        "FDIR-E-002",
        "FDIR-E-001",
        "FDIR-E-002",
        "FDIR-E-003",
        "FDIR-E-003",
        "FDIR-E-004",
        "FDIR-E-005",
        "FDIR-E-010",
        "FDIR-E-008",
    ];
    for (error, expected_id) in errors.iter().zip(expected_ids) {
        assert_eq!(error.contract_error_id(), expected_id);
        let display = error.to_string();
        assert!(display.starts_with(expected_id));
        assert!(display.len() > expected_id.len());
    }
    assert!(errors[0].source().is_some());
    assert!(errors[1..].iter().all(|error| error.source().is_none()));
}

#[test]
fn comments_blank_lines_and_whitespace_are_normalized_with_physical_line_numbers() {
    let input = "\n # preamble comment\n 95.7 # version\n\n 1 1 2\n 1 120 1 # initial\n\t0.25 0.0 1.5\n 1 0 0 # terminator\n";
    let parsed = parse_fixeddate_str(input, ParseMode::Strict)
        .expect("comments, blank lines, and whitespace should normalize deterministically");
    assert_eq!(parsed.initial_records[0].irday, 120);
    assert_eq!(parsed.events.len(), 1);
    assert!(parsed.initial_dates_complete);
    assert!(parsed.event_stream_complete);

    let err = parse_fixeddate_str("\n# comment\n95.7\n1 1 2\n1 BAD 1\n", ParseMode::Strict)
        .expect_err("token error should retain the physical source line");
    assert!(matches!(
        err,
        FixedDateParseError::TokenParseError {
            line: 5,
            field: "irday",
            ..
        }
    ));
}

#[test]
fn empty_and_incomplete_preambles_have_typed_structural_errors() {
    for input in ["", " \n # only a comment\n"] {
        let err = parse_fixeddate_str(input, ParseMode::Strict)
            .expect_err("normalized empty input must be rejected");
        assert!(matches!(
            err,
            FixedDateParseError::MissingRecord {
                line: 1,
                context: "fixeddate preamble"
            }
        ));
    }

    let missing_header = parse_fixeddate_str("95.7\n", ParseMode::Strict)
        .expect_err("datver without a header must be rejected");
    assert!(matches!(
        missing_header,
        FixedDateParseError::MissingRecord {
            line: 2,
            context: "header line (itemp jtemp ktemp)"
        }
    ));

    for (mode, input, expected_id) in [
        (ParseMode::Strict, "1 1 2\n", "FDIR-E-003"),
        (ParseMode::Strict, "oops\n", "FDIR-E-001"),
        (ParseMode::Compatibility, "oops\n", "FDIR-E-001"),
    ] {
        let err = parse_fixeddate_str(input, mode).expect_err("invalid preamble must fail");
        assert_eq!(err.contract_error_id(), expected_id);
    }
}

#[test]
fn header_arity_tokens_and_each_domain_are_rejected() {
    for header in ["1 1", "1 1 2 3"] {
        let input = format!("95.7\n{header}\n");
        assert!(matches!(
            parse_fixeddate_str(&input, ParseMode::Strict),
            Err(FixedDateParseError::RecordArityError {
                context: "header line",
                ..
            })
        ));
    }

    for (header, field) in [("x 1 2", "itemp"), ("1 x 2", "jtemp"), ("1 1 x", "ktemp")] {
        let input = format!("95.7\n{header}\n");
        let err = parse_fixeddate_str(&input, ParseMode::Strict)
            .expect_err("non-integral header field must fail");
        assert!(matches!(
            err,
            FixedDateParseError::TokenParseError { field: actual, .. } if actual == field
        ));
    }

    for (header, field) in [
        ("0 1 2", "itemp"),
        ("1 0 2", "jtemp"),
        ("1 3 2", "jtemp"),
        ("1 1 1", "ktemp"),
    ] {
        let input = format!("95.7\n{header}\n");
        let err = parse_fixeddate_str(&input, ParseMode::Strict)
            .expect_err("header domain violation must fail");
        assert!(matches!(
            err,
            FixedDateParseError::HeaderDomainError { field: actual, .. } if actual == field
        ));
    }
}

#[test]
fn datver_policy_covers_epsilon_thresholds_and_both_irrigation_domains() {
    for datver in [95.7, 95.700_000_5, 95.699_999_5] {
        let input = format!("{datver}\n1 1 2\n1 120 1\n");
        parse_fixeddate_str(&input, ParseMode::Strict)
            .expect("canonical datver epsilon boundary should be accepted");
    }
    for datver in [95.701, 94.21] {
        let input = format!("{datver}\n1 1 2\n1 120 1\n");
        assert!(matches!(
            parse_fixeddate_str(&input, ParseMode::Strict),
            Err(FixedDateParseError::UnsupportedDatver { .. })
        ));
    }

    for (datver, jtemp) in [(94.21, 1), (95.699, 1), (91.5, 2), (95.699, 2)] {
        let input = format!("{datver}\n1 {jtemp} 2\n1 120 1\n");
        let parsed = parse_fixeddate_str(&input, ParseMode::Compatibility)
            .expect("legacy version at an allowed threshold should parse");
        assert_eq!(parsed.warnings[0].code, FixedDateWarningCode::FdirW002);
    }
    for (datver, jtemp) in [(94.209, 1), (91.499, 2), (95.701, 1), (95.701, 2)] {
        let input = format!("{datver}\n1 {jtemp} 2\n1 120 1\n");
        assert!(matches!(
            parse_fixeddate_str(&input, ParseMode::Compatibility),
            Err(FixedDateParseError::UnsupportedDatver { .. })
        ));
    }

    let no_datver = parse_fixeddate_str("1 1 2\n1 120 1\n", ParseMode::Compatibility)
        .expect("three-field legacy header should use the no-datver branch");
    assert_eq!(no_datver.datver_source, DatverSource::LegacyCompatNoDatver);
}

#[test]
fn initial_line3_records_cover_missing_arity_token_and_range_errors() {
    let missing = parse_fixeddate_str("95.7\n2 1 2\n1 120 1\n", ParseMode::Strict)
        .expect_err("itemp initial records are mandatory");
    assert!(matches!(
        missing,
        FixedDateParseError::MissingRecord {
            context: "initial line3 record",
            ..
        }
    ));

    let arity = parse_fixeddate_str("95.7\n1 1 2\n1 120\n", ParseMode::Strict)
        .expect_err("line3 arity is fixed");
    assert!(matches!(
        arity,
        FixedDateParseError::RecordArityError {
            context: "line3",
            ..
        }
    ));

    for (record, field) in [
        ("x 120 1", "ofeflg"),
        ("1 x 1", "irday"),
        ("1 120 x", "iryr"),
    ] {
        let input = format!("95.7\n1 1 2\n{record}\n");
        let err = parse_fixeddate_str(&input, ParseMode::Strict)
            .expect_err("non-integral line3 token must fail");
        assert!(matches!(
            err,
            FixedDateParseError::TokenParseError { field: actual, .. } if actual == field
        ));
    }

    for (record, field) in [
        ("0 120 1", "ofeflg"),
        ("2 120 1", "ofeflg"),
        ("1 -1 1", "irday"),
        ("1 367 1", "irday"),
        ("1 120 -1", "iryr"),
    ] {
        let input = format!("95.7\n1 1 2\n{record}\n");
        let err = parse_fixeddate_str(&input, ParseMode::Strict)
            .expect_err("line3 range violation must fail");
        assert!(matches!(
            err,
            FixedDateParseError::FieldRangeError { field: actual, .. } if actual == field
        ));
    }
}

#[test]
fn sprinkler_event_covers_arities_tokens_ranges_and_event_ordering() {
    for row in ["0.1", "0.1 0.2 1.0 9.0"] {
        let input = format!("95.7\n1 1 2\n1 120 1\n{row}\n1 0 0\n");
        assert!(matches!(
            parse_fixeddate_str(&input, ParseMode::Strict),
            Err(FixedDateParseError::RecordArityError {
                context: "sprinkler line4",
                ..
            })
        ));
    }
    for (row, field) in [
        ("x 0.2 1.0", "irint"),
        ("0.1 x 1.0", "irdept"),
        ("0.1 0.2 x", "nozzle"),
    ] {
        let input = format!("95.7\n1 1 2\n1 120 1\n{row}\n1 0 0\n");
        let err = parse_fixeddate_str(&input, ParseMode::Strict)
            .expect_err("non-real sprinkler token must fail");
        assert!(matches!(
            err,
            FixedDateParseError::TokenParseError { field: actual, .. } if actual == field
        ));
    }
    for (row, field) in [
        ("0 0.2 1.0", "irint"),
        ("0.1 -0.1 1.0", "irdept"),
        ("0.1 0.2 0", "nozzle"),
    ] {
        let input = format!("95.7\n1 1 2\n1 120 1\n{row}\n1 0 0\n");
        let err = parse_fixeddate_str(&input, ParseMode::Strict)
            .expect_err("sprinkler range violation must fail");
        assert!(matches!(
            err,
            FixedDateParseError::FieldRangeError { field: actual, .. } if actual == field
        ));
    }

    let strict_order = parse_fixeddate_str(
        "95.7\n2 1 2\n1 120 1\n2 121 1\n0.1 0.2 1\n2 0 0\n",
        ParseMode::Strict,
    )
    .expect_err("event OFE order starts at one");
    assert!(matches!(
        strict_order,
        FixedDateParseError::OrderingConstraintError { phase: "event", .. }
    ));
    let compat = parse_fixeddate_str(
        "95.7\n2 1 2\n1 120 1\n2 121 1\n0.1 0.2 1\n2 0 0\n",
        ParseMode::Compatibility,
    )
    .expect("compatibility mode records event-order anomalies");
    assert_eq!(compat.warnings[0].code, FixedDateWarningCode::FdirW006);
}

#[test]
fn event_ofe_expectation_cycles_across_multiple_sprinkler_events() {
    let input =
        "95.7\n2 1 2\n1 100 1\n2 101 1\n0.1 0.2 1\n1 150 1\n0.1 0.2 1\n2 151 1\n0.1 0.2 1\n1 0 0\n";
    let first = parse_fixeddate_str(input, ParseMode::Strict)
        .expect("event OFE sequence should wrap itemp to one");
    let second = parse_fixeddate_str(input, ParseMode::Strict)
        .expect("repeated parse should remain deterministic");
    assert_eq!(first, second);
    assert_eq!(first.events.len(), 3);
    assert_eq!(
        first
            .events
            .iter()
            .map(|event| match event {
                FixedDateEvent::Sprinkler(event) => event.next_record.ofeflg,
                FixedDateEvent::Furrow(_) => unreachable!("sprinkler fixture"),
            })
            .collect::<Vec<_>>(),
        [1, 2, 1]
    );
}

#[test]
fn furrow_surges_cover_arity_tokens_range_and_twenty_row_boundary() {
    for row in ["1 2", "1 2 3"] {
        let input = format!("95.7\n1 2 2\n1 120 1\n{row}\n1 0 1 0\n1 0 0\n");
        let err = parse_fixeddate_str(&input, ParseMode::Strict)
            .expect_err("furrow stream requires a valid surge record");
        assert!(matches!(
            err,
            FixedDateParseError::RecordArityError {
                context: "furrow line4",
                ..
            }
        ));
    }
    for surge in ["x", "0", "21"] {
        let input = format!("95.7\n1 2 2\n1 120 1\n{surge}\n");
        let err = parse_fixeddate_str(&input, ParseMode::Strict)
            .expect_err("invalid surge count must fail");
        if surge == "x" {
            assert!(matches!(
                err,
                FixedDateParseError::TokenParseError {
                    field: "surges",
                    ..
                }
            ));
        } else {
            assert!(matches!(
                err,
                FixedDateParseError::FieldRangeError {
                    field: "surges",
                    ..
                }
            ));
        }
    }

    let rows = std::iter::repeat_n("0.1 0 1 0", 20)
        .collect::<Vec<_>>()
        .join("\n");
    let input = format!("95.7\n1 2 2\n1 120 1\n20\n{rows}\n1 0 0\n");
    let parsed = parse_fixeddate_str(&input, ParseMode::Strict)
        .expect("twenty surges is the inclusive upper boundary");
    match &parsed.events[0] {
        FixedDateEvent::Furrow(event) => assert_eq!(event.rows.len(), 20),
        FixedDateEvent::Sprinkler(_) => panic!("expected furrow event"),
    }
}

#[test]
fn furrow_rows_cover_mode_arities_tokens_and_each_range_constraint() {
    for (mode, row) in [
        (ParseMode::Strict, "0.1 0 1"),
        (ParseMode::Strict, "0.1 0 1 0 9"),
        (ParseMode::Compatibility, "0.1 0"),
        (ParseMode::Compatibility, "0.1 0 1 0 9"),
    ] {
        let input = format!("95.7\n1 2 2\n1 120 1\n1\n{row}\n1 0 0\n");
        assert!(matches!(
            parse_fixeddate_str(&input, mode),
            Err(FixedDateParseError::RecordArityError {
                context: "furrow line5",
                ..
            })
        ));
    }
    for (row, field) in [
        ("x 0 1 0", "qspply"),
        ("0.1 x 1 0", "tstart"),
        ("0.1 0 x 0", "tend"),
        ("0.1 0 1 x", "tdepl"),
    ] {
        let input = format!("95.7\n1 2 2\n1 120 1\n1\n{row}\n1 0 0\n");
        let err = parse_fixeddate_str(&input, ParseMode::Strict)
            .expect_err("non-real furrow token must fail");
        assert!(matches!(
            err,
            FixedDateParseError::TokenParseError { field: actual, .. } if actual == field
        ));
    }
    for (row, field) in [
        ("-0.1 0 1 0", "qspply"),
        ("0.1 -0.1 1 0", "tstart"),
        ("0.1 2 1 0", "tend"),
        ("0.1 0 1 -0.1", "tdepl"),
    ] {
        let input = format!("95.7\n1 2 2\n1 120 1\n1\n{row}\n1 0 0\n");
        let err = parse_fixeddate_str(&input, ParseMode::Strict)
            .expect_err("furrow range violation must fail");
        assert!(matches!(
            err,
            FixedDateParseError::FieldRangeError { field: actual, .. } if actual == field
        ));
    }
}

#[test]
fn event_stream_closure_errors_cover_sprinkler_and_furrow_successors() {
    let sprinkler = parse_fixeddate_str("95.7\n1 1 2\n1 120 1\n0.1 0.2 1\n", ParseMode::Strict)
        .expect_err("sprinkler event requires successor line3");
    assert!(matches!(
        sprinkler,
        FixedDateParseError::EventStreamClosureError {
            context: "sprinkler successor line3",
            ..
        }
    ));

    for input in [
        "95.7\n1 2 2\n1 120 1\n1\n",
        "95.7\n1 2 2\n1 120 1\n2\n0.1 0 1 0\n",
    ] {
        let err = parse_fixeddate_str(input, ParseMode::Strict)
            .expect_err("furrow event requires every row and successor line3");
        assert!(matches!(
            err,
            FixedDateParseError::EventStreamClosureError {
                context: "furrow successor line3",
                ..
            }
        ));
    }
}
