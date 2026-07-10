use std::collections::HashSet;
use std::path::PathBuf;

use openwepp_input_contract::parsers::irrigation_depletion::{
    IrrigationDepletionParseError, IrrigationDepletionParserOptions,
    IrrigationDepletionTopologyContext, IrrigationDepletionWarningCode, IrrigationPeriodData,
    IrrigationSystemType, ParseMode, ZeroStartTransition, parse_irrigation_depletion_from_path,
    parse_irrigation_depletion_from_str,
};

fn fixture_path(name: &str) -> PathBuf {
    PathBuf::from("tests/fixtures/infile/irrigation_depletion").join(name)
}

fn assert_error_contract(
    error: &IrrigationDepletionParseError,
    expected_id: &str,
    expected_display: &str,
) {
    assert_eq!(error.contract_error_id(), expected_id);
    assert_eq!(error.to_string(), expected_display);
}

#[test]
fn typed_error_displays_preserve_every_public_contract_id() {
    assert_error_contract(
        &IrrigationDepletionParseError::InputOpenError {
            path: PathBuf::from("fixture.ird"),
            source: std::io::Error::other("synthetic open failure"),
        },
        "IRD-E-000",
        "IRD-E-000: could not open fixture.ird (synthetic open failure)",
    );
    assert_error_contract(
        &IrrigationDepletionParseError::MissingRecord {
            field: "static_line",
        },
        "IRD-E-002",
        "IRD-E-002: missing required record: static_line",
    );
    assert_error_contract(
        &IrrigationDepletionParseError::TokenParseError {
            line: 4,
            field: "irrate",
            token: "not-a-rate".to_string(),
        },
        "IRD-E-001",
        "IRD-E-001: line 4 could not parse field 'irrate' from token 'not-a-rate'",
    );
    assert_error_contract(
        &IrrigationDepletionParseError::RecordArityError {
            line: 3,
            context: "furrow_period",
            expected: "11",
            found: 10,
        },
        "IRD-E-002",
        "IRD-E-002: line 3 furrow_period expects 11 token(s), found 10",
    );
    assert_error_contract(
        &IrrigationDepletionParseError::UnsupportedDatver {
            line: 1,
            observed: Some(94.2),
            reason: "unsupported datver for selected mode/system",
        },
        "IRD-E-003",
        "IRD-E-003: line 1 datver 94.2 unsupported (unsupported datver for selected mode/system)",
    );
    assert_error_contract(
        &IrrigationDepletionParseError::UnsupportedDatver {
            line: 1,
            observed: None,
            reason: "strict mode requires explicit datver header",
        },
        "IRD-E-003",
        "IRD-E-003: line 1 unsupported datver/header branch (strict mode requires explicit datver header)",
    );
}

#[test]
fn remaining_typed_error_displays_preserve_every_public_contract_id() {
    assert_error_contract(
        &IrrigationDepletionParseError::InvalidHeaderDomain {
            line: 2,
            field: "jtemp",
            value: 3,
            allowed: "expected 1 (sprinkler) or 2 (furrow)",
        },
        "IRD-E-004",
        "IRD-E-004: line 2 invalid header field 'jtemp' value 3; expected expected 1 (sprinkler) or 2 (furrow)",
    );
    assert_error_contract(
        &IrrigationDepletionParseError::FieldRangeError {
            line: 4,
            field: "depsrg",
            value: 3.0,
            expected: "1, 2, 4, 5, or 6",
        },
        "IRD-E-005",
        "IRD-E-005: line 4 field 'depsrg' value 3 violates 1, 2, 4, 5, or 6",
    );
    assert_error_contract(
        &IrrigationDepletionParseError::CrossFileMismatch {
            line: 2,
            field: "itemp",
            expected: "2".to_string(),
            observed: "1".to_string(),
        },
        "IRD-E-006",
        "IRD-E-006: line 2 cross-file mismatch for 'itemp' (expected 2, observed 1)",
    );
    assert_error_contract(
        &IrrigationDepletionParseError::InvalidElementId {
            line: 4,
            field: "endpln",
            value: 0,
        },
        "IRD-E-007",
        "IRD-E-007: line 4 invalid element id for 'endpln': 0",
    );
    assert_error_contract(
        &IrrigationDepletionParseError::ContinuationOrderingError {
            line: 7,
            previous: (2001, 250, 1),
            current: (2001, 240, 2),
        },
        "IRD-E-008",
        "IRD-E-008: line 7 continuation ordering violation prev=(2001, 250, 1) current=(2001, 240, 2)",
    );
    assert_error_contract(
        &IrrigationDepletionParseError::FurrowDisallowed {
            line: 4,
            reason: "furrow irrigation disallowed under contour/non-cropland strict policy",
        },
        "IRD-E-009",
        "IRD-E-009: line 4 furrow irrigation disallowed (furrow irrigation disallowed under contour/non-cropland strict policy)",
    );
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
    assert_eq!(
        parsed
            .warnings
            .iter()
            .map(|warning| (
                warning.code.as_str(),
                warning.line,
                warning.message.as_str()
            ))
            .collect::<Vec<_>>(),
        vec![
            ("IRD-W-006", 2, "compatibility accepted legacy datver 95",),
            ("IRD-W-003", 4, "compatibility normalized depsrg 3 -> 4",),
            ("IRD-W-003", 5, "compatibility normalized depsrg 9 -> 6",),
        ]
    );

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
fn strict_rejects_legacy_furrow_datver_with_exact_policy_detail() {
    let error = parse_irrigation_depletion_from_path(
        fixture_path("compat_legacy_furrow_datver_normalization.txt"),
        IrrigationDepletionParserOptions::strict(),
        &IrrigationDepletionTopologyContext::default(),
    )
    .expect_err("strict mode must reject the legacy furrow datver before row normalization");

    assert_error_contract(
        &error,
        "IRD-E-003",
        "IRD-E-003: line 2 datver 95 unsupported (unsupported datver for selected mode/system)",
    );
}

#[test]
fn public_parser_preserves_furrow_guard_locations_and_details() {
    let arity_error = parse_irrigation_depletion_from_str(
        "95.7\n1 2 1\n0.010\n1 2 0.00035 7200 2 0.90 0.55 120 2001 180\n",
        IrrigationDepletionParserOptions::strict(),
        &IrrigationDepletionTopologyContext::default(),
    )
    .expect_err("furrow rows require all eleven fields in strict mode");
    assert_error_contract(
        &arity_error,
        "IRD-E-002",
        "IRD-E-002: line 4 furrow_period expects 11 token(s), found 10",
    );

    let surge_error = parse_irrigation_depletion_from_str(
        "95.7\n1 2 1\n0.010\n1 2 0.00035 7200 3 0.90 0.55 120 2001 180 2001\n",
        IrrigationDepletionParserOptions::strict(),
        &IrrigationDepletionTopologyContext::default(),
    )
    .expect_err("strict mode must preserve the depsrg domain instead of normalizing it");
    assert_error_contract(
        &surge_error,
        "IRD-E-005",
        "IRD-E-005: line 4 field 'depsrg' value 3 violates 1, 2, 4, 5, or 6",
    );

    let end_element_error = parse_irrigation_depletion_from_str(
        "95.7\n1 2 1\n0.010\n1 0 0.00035 7200 2 0.90 0.55 120 2001 180 2001\n",
        IrrigationDepletionParserOptions::strict(),
        &IrrigationDepletionTopologyContext::default(),
    )
    .expect_err("furrow end-element ids must remain positive");
    assert_error_contract(
        &end_element_error,
        "IRD-E-007",
        "IRD-E-007: line 4 invalid element id for 'endpln': 0",
    );

    let header_error = parse_irrigation_depletion_from_str(
        "95.7\n1 3 1\n",
        IrrigationDepletionParserOptions::strict(),
        &IrrigationDepletionTopologyContext::default(),
    )
    .expect_err("only sprinkler and furrow headers are valid");
    assert_error_contract(
        &header_error,
        "IRD-E-004",
        "IRD-E-004: line 2 invalid header field 'jtemp' value 3; expected expected 1 (sprinkler) or 2 (furrow)",
    );

    let token_error = parse_irrigation_depletion_from_str(
        "95.7\nitemp 1 1\n",
        IrrigationDepletionParserOptions::strict(),
        &IrrigationDepletionTopologyContext::default(),
    )
    .expect_err("header fields remain typed numeric tokens");
    assert_error_contract(
        &token_error,
        "IRD-E-001",
        "IRD-E-001: line 2 could not parse field 'itemp' from token 'itemp'",
    );

    let cross_file_error = parse_irrigation_depletion_from_str(
        "95.7\n1 2 1\n0.010\n1 2 0.00035 7200 2 0.90 0.55 120 2001 180 2001\n",
        IrrigationDepletionParserOptions {
            expected_system_type: Some(IrrigationSystemType::Sprinkler),
            expected_irrigation_option: Some(1),
            ..IrrigationDepletionParserOptions::strict()
        },
        &IrrigationDepletionTopologyContext::default(),
    )
    .expect_err("the first cross-file mismatch remains a typed jtemp error");
    assert_error_contract(
        &cross_file_error,
        "IRD-E-006",
        "IRD-E-006: line 2 cross-file mismatch for 'jtemp' (expected 1, observed 2)",
    );
}

#[test]
fn run_option_closure_preserves_cross_file_error_detail() {
    let error = parse_irrigation_depletion_from_path(
        fixture_path("strict_valid_sprinkler_95_7.txt"),
        IrrigationDepletionParserOptions {
            expected_irrigation_option: Some(1),
            ..IrrigationDepletionParserOptions::strict()
        },
        &IrrigationDepletionTopologyContext::default(),
    )
    .expect_err("non-depletion run options must remain rejected");

    assert_error_contract(
        &error,
        "IRD-E-006",
        "IRD-E-006: line 2 cross-file mismatch for 'irrigation_option' (expected 2|3|5|6 (depletion scheduling enabled), observed 1)",
    );
}

#[test]
fn compatibility_furrow_zero_start_preserves_transition_and_warning_details() {
    let parsed = parse_irrigation_depletion_from_str(
        "95.7\n1 2 1\n0.010\n1 1 0.00035 7200 2 0.90 0.55 0 0 180 2001\n",
        IrrigationDepletionParserOptions {
            mode: ParseMode::Compatibility,
            irschd_on_entry: Some(3),
            ..IrrigationDepletionParserOptions::compatibility()
        },
        &IrrigationDepletionTopologyContext::default(),
    )
    .expect("zero-start furrow period is an explicit compatibility transition");

    assert_eq!(
        parsed.periods[0].zero_start_transition,
        ZeroStartTransition::ThreeToTwo
    );
    assert_eq!(
        parsed.warnings,
        vec![
            openwepp_input_contract::parsers::irrigation_depletion::IrrigationDepletionWarning {
                code: IrrigationDepletionWarningCode::IrdW004,
                line: 4,
                message: "irbeg==0 transition branch encountered".to_string(),
            }
        ]
    );
}

#[test]
fn public_error_source_and_default_options_preserve_contract_semantics() {
    assert_eq!(
        IrrigationDepletionParserOptions::default(),
        IrrigationDepletionParserOptions::strict()
    );

    let input_open = IrrigationDepletionParseError::InputOpenError {
        path: PathBuf::from("fixture.ird"),
        source: std::io::Error::other("synthetic open failure"),
    };
    let source = std::error::Error::source(&input_open)
        .expect("input-open errors must retain their I/O source");
    assert_eq!(source.to_string(), "synthetic open failure");

    let missing = IrrigationDepletionParseError::MissingRecord {
        field: "static_line",
    };
    assert!(std::error::Error::source(&missing).is_none());
}

#[test]
fn sprinkler_rows_preserve_arity_and_nozzle_domain_guards() {
    let missing_nozzle = parse_irrigation_depletion_from_str(
        "95.7\n1 1 1\n0.010 0.020\n1 0.000002 1.0 0.5 120 2001 180 2001\n",
        IrrigationDepletionParserOptions::strict(),
        &IrrigationDepletionTopologyContext::default(),
    )
    .expect_err("strict mode must not silently default a missing sprinkler nozzle");
    assert_error_contract(
        &missing_nozzle,
        "IRD-E-002",
        "IRD-E-002: line 4 sprinkler_period expects 9 (or 8 in compatibility for legacy nozzle omission) token(s), found 8",
    );

    let zero_nozzle = parse_irrigation_depletion_from_str(
        "95.7\n1 1 1\n0.010 0.020\n1 0.000002 1.0 0.5 0.0 120 2001 180 2001\n",
        IrrigationDepletionParserOptions::strict(),
        &IrrigationDepletionTopologyContext::default(),
    )
    .expect_err("sprinkler nozzle factors must be positive");
    assert_error_contract(
        &zero_nozzle,
        "IRD-E-005",
        "IRD-E-005: line 4 field 'nozzle' value 0 violates finite and > 0",
    );

    let non_finite_nozzle = parse_irrigation_depletion_from_str(
        "95.7\n1 1 1\n0.010 0.020\n1 0.000002 1.0 0.5 NaN 120 2001 180 2001\n",
        IrrigationDepletionParserOptions::strict(),
        &IrrigationDepletionTopologyContext::default(),
    )
    .expect_err("sprinkler nozzle factors must be finite");
    assert_error_contract(
        &non_finite_nozzle,
        "IRD-E-005",
        "IRD-E-005: line 4 field 'nozzle' value NaN violates finite and > 0",
    );

    let negative_rate = parse_irrigation_depletion_from_str(
        "95.7\n1 1 1\n0.010 0.020\n1 -0.000002 1.0 0.5 1.0 120 2001 180 2001\n",
        IrrigationDepletionParserOptions::strict(),
        &IrrigationDepletionTopologyContext::default(),
    )
    .expect_err("sprinkler irrigation rates must remain nonnegative");
    assert_error_contract(
        &negative_rate,
        "IRD-E-005",
        "IRD-E-005: line 4 field 'irrate' value -0.000002 violates finite and >= 0",
    );
}

#[test]
fn sprinkler_rows_preserve_integer_and_day_year_contract_guards() {
    let parsed = parse_irrigation_depletion_from_str(
        "95.7\n1.0 1.0 1.0\n0.010 0.020\n1.0 0.000002 1.0 0.5 1.0 0.0 0.0 0.0 0.0\n",
        IrrigationDepletionParserOptions::default(),
        &IrrigationDepletionTopologyContext::default(),
    )
    .expect("integral floating-point legacy tokens and zero-date transition rows are valid");
    assert_eq!(parsed.element_count, 1);
    assert_eq!(parsed.periods[0].element_id, 1);
    assert_eq!(parsed.periods[0].start_doy, 0);
    assert_eq!(parsed.periods[0].end_doy, 0);

    let fractional_count = parse_irrigation_depletion_from_str(
        "95.7\n1.5 1 1\n",
        IrrigationDepletionParserOptions::strict(),
        &IrrigationDepletionTopologyContext::default(),
    )
    .expect_err("non-integral itemp tokens must fail instead of rounding silently");
    assert_error_contract(
        &fractional_count,
        "IRD-E-001",
        "IRD-E-001: line 2 could not parse field 'itemp' from token '1.5'",
    );

    let oversized_count = parse_irrigation_depletion_from_str(
        "95.7\n2147483648 1 1\n",
        IrrigationDepletionParserOptions::strict(),
        &IrrigationDepletionTopologyContext::default(),
    )
    .expect_err("out-of-range itemp tokens must fail instead of narrowing");
    assert_error_contract(
        &oversized_count,
        "IRD-E-001",
        "IRD-E-001: line 2 could not parse field 'itemp' from token '2147483648'",
    );

    let negative_count = parse_irrigation_depletion_from_str(
        "95.7\n-1 1 1\n",
        IrrigationDepletionParserOptions::strict(),
        &IrrigationDepletionTopologyContext::default(),
    )
    .expect_err("element counts must remain positive");
    assert_error_contract(
        &negative_count,
        "IRD-E-004",
        "IRD-E-004: line 2 invalid header field 'itemp' value -1; expected > 0",
    );

    let before_year = parse_irrigation_depletion_from_str(
        "95.7\n1 1 1\n0.010 0.020\n1 0.000002 1.0 0.5 1.0 -1 2001 180 2001\n",
        IrrigationDepletionParserOptions::strict(),
        &IrrigationDepletionTopologyContext::default(),
    )
    .expect_err("day-of-year values below zero must fail closed");
    assert_error_contract(
        &before_year,
        "IRD-E-005",
        "IRD-E-005: line 4 field 'irbeg/yrbeg' value -1 violates day in [0, 366]",
    );

    let after_year = parse_irrigation_depletion_from_str(
        "95.7\n1 1 1\n0.010 0.020\n1 0.000002 1.0 0.5 1.0 367 2001 180 2001\n",
        IrrigationDepletionParserOptions::strict(),
        &IrrigationDepletionTopologyContext::default(),
    )
    .expect_err("day-of-year values above 366 must fail closed");
    assert_error_contract(
        &after_year,
        "IRD-E-005",
        "IRD-E-005: line 4 field 'irbeg/yrbeg' value 367 violates day in [0, 366]",
    );

    let missing_year = parse_irrigation_depletion_from_str(
        "95.7\n1 1 1\n0.010 0.020\n1 0.000002 1.0 0.5 1.0 120 0 180 2001\n",
        IrrigationDepletionParserOptions::strict(),
        &IrrigationDepletionTopologyContext::default(),
    )
    .expect_err("nonzero period days require a positive year");
    assert_error_contract(
        &missing_year,
        "IRD-E-005",
        "IRD-E-005: line 4 field 'irbeg/yrbeg' value 0 violates year > 0 when day > 0",
    );
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
