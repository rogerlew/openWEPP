use std::path::PathBuf;

use openwepp_input_contract::parsers::irrigation_fixeddate::{
    DatverSource, FixedDateEvent, FixedDateParseError, FixedDateWarningCode,
    IryrInterpretationMode, ParseMode, parse_fixeddate_file,
};

fn fixture_path(name: &str) -> PathBuf {
    PathBuf::from("tests/fixtures/infile/irrigation_fixeddate").join(name)
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
