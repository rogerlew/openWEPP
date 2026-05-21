use std::collections::HashSet;
use std::path::PathBuf;

use openwepp_input_contract::parsers::irrigation_depletion::{
    IrrigationDepletionParseError, IrrigationDepletionParserOptions,
    IrrigationDepletionTopologyContext, IrrigationDepletionWarningCode, IrrigationPeriodData,
    IrrigationSystemType, ParseMode, parse_irrigation_depletion_from_path,
};

fn fixture_path(name: &str) -> PathBuf {
    PathBuf::from("tests/fixtures/infile/irrigation_depletion").join(name)
}

#[test]
fn strict_parses_valid_sprinkler_with_continuation_rows() {
    let parsed = parse_irrigation_depletion_from_path(
        fixture_path("strict_valid_sprinkler_95_7.txt"),
        IrrigationDepletionParserOptions::strict(),
        &IrrigationDepletionTopologyContext::default(),
    )
    .expect("strict parser should accept canonical 95.7 sprinkler stream");

    assert_eq!(parsed.datver, Some(95.7));
    assert!(parsed.datver_explicit);
    assert_eq!(parsed.element_count, 2);
    assert_eq!(parsed.system_type, IrrigationSystemType::Sprinkler);
    assert_eq!(parsed.schedule_type, 1);
    assert_eq!(parsed.periods.len(), 4);
    assert_eq!(parsed.continuation_rows().len(), 2);
    assert!(parsed.warnings.is_empty());

    match &parsed.periods[0].data {
        IrrigationPeriodData::Sprinkler(record) => {
            assert!((record.nozzle_factor - 1.0).abs() < 1e-12);
        }
        IrrigationPeriodData::Furrow(_) => panic!("expected sprinkler row"),
    }
}

#[test]
fn strict_rejects_legacy_no_datver_branch() {
    let err = parse_irrigation_depletion_from_path(
        fixture_path("compat_legacy_no_datver_nozzle_missing.txt"),
        IrrigationDepletionParserOptions::strict(),
        &IrrigationDepletionTopologyContext::default(),
    )
    .expect_err("strict mode must reject no-datver branch");

    assert!(matches!(
        err,
        IrrigationDepletionParseError::UnsupportedDatver { .. }
    ));
    assert_eq!(err.contract_error_id(), "IRD-E-003");
}

#[test]
fn compat_accepts_legacy_no_datver_and_injects_nozzle_defaults() {
    let parsed = parse_irrigation_depletion_from_path(
        fixture_path("compat_legacy_no_datver_nozzle_missing.txt"),
        IrrigationDepletionParserOptions::compatibility(),
        &IrrigationDepletionTopologyContext::default(),
    )
    .expect("compatibility mode should accept no-datver legacy branch");

    assert_eq!(parsed.datver, None);
    assert!(!parsed.datver_explicit);
    assert_eq!(parsed.system_type, IrrigationSystemType::Sprinkler);
    assert_eq!(parsed.periods.len(), 2);

    let warning_codes: Vec<&str> = parsed.warnings.iter().map(|w| w.code.as_str()).collect();
    assert!(warning_codes.contains(&IrrigationDepletionWarningCode::IrdW001.as_str()));
    assert!(warning_codes.contains(&IrrigationDepletionWarningCode::IrdW002.as_str()));

    for period in &parsed.periods {
        match &period.data {
            IrrigationPeriodData::Sprinkler(record) => {
                assert!((record.nozzle_factor - 1.0).abs() < 1e-12);
            }
            IrrigationPeriodData::Furrow(_) => panic!("expected sprinkler period rows"),
        }
    }
}

#[test]
fn compat_accepts_legacy_furrow_datver_and_normalizes_depsrg() {
    let parsed = parse_irrigation_depletion_from_path(
        fixture_path("compat_legacy_furrow_datver_normalization.txt"),
        IrrigationDepletionParserOptions::compatibility(),
        &IrrigationDepletionTopologyContext::default(),
    )
    .expect("compatibility mode should accept supported legacy furrow datver window");

    assert_eq!(parsed.system_type, IrrigationSystemType::Furrow);

    let warning_codes: Vec<&str> = parsed.warnings.iter().map(|w| w.code.as_str()).collect();
    assert!(warning_codes.contains(&IrrigationDepletionWarningCode::IrdW006.as_str()));
    assert!(warning_codes.contains(&IrrigationDepletionWarningCode::IrdW003.as_str()));

    let mut surge_codes = Vec::new();
    for period in &parsed.periods {
        match &period.data {
            IrrigationPeriodData::Furrow(record) => surge_codes.push(record.surge_code),
            IrrigationPeriodData::Sprinkler(_) => panic!("expected furrow rows"),
        }
    }
    assert_eq!(surge_codes, vec![4, 6]);
}

#[test]
fn strict_rejects_invalid_ktemp_header_domain() {
    let err = parse_irrigation_depletion_from_path(
        fixture_path("invalid_ktemp.txt"),
        IrrigationDepletionParserOptions::strict(),
        &IrrigationDepletionTopologyContext::default(),
    )
    .expect_err("ktemp must be 1 for depletion scheduling");

    assert!(matches!(
        err,
        IrrigationDepletionParseError::InvalidHeaderDomain { field: "ktemp", .. }
    ));
    assert_eq!(err.contract_error_id(), "IRD-E-004");
}

#[test]
fn strict_rejects_invalid_initialization_order() {
    let err = parse_irrigation_depletion_from_path(
        fixture_path("invalid_initialization_order.txt"),
        IrrigationDepletionParserOptions::strict(),
        &IrrigationDepletionTopologyContext::default(),
    )
    .expect_err("first itemp rows must be element ids 1..itemp in order");

    assert!(matches!(
        err,
        IrrigationDepletionParseError::ContinuationOrderingError { .. }
    ));
    assert_eq!(err.contract_error_id(), "IRD-E-008");
}

#[test]
fn strict_rejects_non_monotone_continuation_order() {
    let err = parse_irrigation_depletion_from_path(
        fixture_path("invalid_continuation_order.txt"),
        IrrigationDepletionParserOptions::strict(),
        &IrrigationDepletionTopologyContext::default(),
    )
    .expect_err("strict mode must reject non-monotone continuation stream ordering");

    assert!(matches!(
        err,
        IrrigationDepletionParseError::ContinuationOrderingError { .. }
    ));
    assert_eq!(err.contract_error_id(), "IRD-E-008");
}

#[test]
fn strict_mode_errors_when_furrow_context_is_disallowed() {
    let options = IrrigationDepletionParserOptions {
        mode: ParseMode::Strict,
        furrow_disallowed_context: true,
        ..IrrigationDepletionParserOptions::strict()
    };

    let err = parse_irrigation_depletion_from_path(
        fixture_path("strict_furrow_disallowed_context.txt"),
        options,
        &IrrigationDepletionTopologyContext::default(),
    )
    .expect_err("strict mode should reject furrow in disallowed context");

    assert!(matches!(
        err,
        IrrigationDepletionParseError::FurrowDisallowed { .. }
    ));
    assert_eq!(err.contract_error_id(), "IRD-E-009");
}

#[test]
fn compat_mode_warns_and_marks_disabled_when_furrow_context_is_disallowed() {
    let options = IrrigationDepletionParserOptions {
        mode: ParseMode::Compatibility,
        furrow_disallowed_context: true,
        ..IrrigationDepletionParserOptions::compatibility()
    };

    let parsed = parse_irrigation_depletion_from_path(
        fixture_path("strict_furrow_disallowed_context.txt"),
        options,
        &IrrigationDepletionTopologyContext::default(),
    )
    .expect("compatibility mode should degrade to disabled furrow periods with warning");

    assert!(
        parsed
            .warnings
            .iter()
            .any(|warning| warning.code == IrrigationDepletionWarningCode::IrdW005)
    );
    assert!(
        parsed
            .periods
            .iter()
            .all(|period| period.furrow_disabled_by_landuse)
    );
}

#[test]
fn reports_cross_file_element_count_mismatch() {
    let options = IrrigationDepletionParserOptions {
        expected_element_count: Some(3),
        ..IrrigationDepletionParserOptions::strict()
    };

    let err = parse_irrigation_depletion_from_path(
        fixture_path("strict_valid_sprinkler_95_7.txt"),
        options,
        &IrrigationDepletionTopologyContext::default(),
    )
    .expect_err("cross-file mismatch should raise typed error");

    assert!(matches!(
        err,
        IrrigationDepletionParseError::CrossFileMismatch { field: "itemp", .. }
    ));
    assert_eq!(err.contract_error_id(), "IRD-E-006");
}

#[test]
fn rejects_unknown_element_ids_when_topology_context_is_provided() {
    let mut allowed = HashSet::new();
    let _ = allowed.insert(1usize);

    let err = parse_irrigation_depletion_from_path(
        fixture_path("strict_valid_sprinkler_95_7.txt"),
        IrrigationDepletionParserOptions::strict(),
        &IrrigationDepletionTopologyContext {
            allowed_element_ids: Some(allowed),
        },
    )
    .expect_err("element id 2 should fail topology closure");

    assert!(matches!(
        err,
        IrrigationDepletionParseError::InvalidElementId {
            field: "ofeflg",
            ..
        }
    ));
    assert_eq!(err.contract_error_id(), "IRD-E-007");
}

#[test]
fn missing_file_reports_typed_input_open_error() {
    let err = parse_irrigation_depletion_from_path(
        fixture_path("does_not_exist.txt"),
        IrrigationDepletionParserOptions::strict(),
        &IrrigationDepletionTopologyContext::default(),
    )
    .expect_err("missing file should be surfaced as typed open error");

    assert!(matches!(
        err,
        IrrigationDepletionParseError::InputOpenError { .. }
    ));
    assert_eq!(err.contract_error_id(), "IRD-E-000");
}
