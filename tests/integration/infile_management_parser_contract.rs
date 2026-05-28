use std::path::{Path, PathBuf};

use openwepp_input_contract::parsers::management::{
    ManagementParseError, ParseMode, YearlyScenarioData, parse_management_from_path,
};

fn fixture_path(name: &str) -> PathBuf {
    Path::new(file!())
        .parent()
        .expect("test file parent exists")
        .parent()
        .expect("tests dir exists")
        .join("fixtures")
        .join("infile")
        .join("management")
        .join(name)
}

#[test]
fn strict_mode_parses_canonical_nonzero_cropland_fixture() {
    let parsed = parse_management_from_path(
        fixture_path("canonical_cropland_nonzero_98_4.man"),
        ParseMode::Strict,
    )
    .expect("strict parser should accept canonical non-zero 98.4 fixture");

    assert_eq!(parsed.datver, "98.4");
    assert_eq!(parsed.topology_count, 1);
    assert_eq!(parsed.declared_total_years, 1);
    assert_eq!(parsed.section_counts.ncrop, 1);
    assert_eq!(parsed.section_counts.nop, 1);
    assert_eq!(parsed.section_counts.nini, 1);
    assert_eq!(parsed.section_counts.nseq, 1);
    assert_eq!(parsed.section_counts.ncnt, 0);
    assert_eq!(parsed.section_counts.ndrain, 0);
    assert_eq!(parsed.section_counts.nscen, 1);

    assert_eq!(parsed.registries.plants.len(), 1);
    assert_eq!(parsed.registries.operations.len(), 1);
    assert_eq!(parsed.registries.initials.len(), 1);
    assert_eq!(parsed.registries.surfaces.len(), 1);
    assert_eq!(parsed.registries.yearlies.len(), 1);

    assert_eq!(parsed.schedule.rotation_repeats, 1);
    assert_eq!(parsed.schedule.rotation_years, 1);
    assert_eq!(parsed.schedule.slots.len(), 1);
    assert_eq!(parsed.schedule.slots[0].crop_slots, 1);
    assert_eq!(parsed.schedule.slots[0].yearly_refs, vec![1]);
}

#[test]
fn strict_mode_parses_rotation_fixture_with_schedule_expansion() {
    let parsed = parse_management_from_path(
        fixture_path("canonical_rotation_nonzero_98_4.man"),
        ParseMode::Strict,
    )
    .expect("strict parser should accept canonical multi-rotation fixture");

    assert_eq!(parsed.datver, "98.4");
    assert_eq!(parsed.topology_count, 3);
    assert_eq!(parsed.schedule.rotation_repeats, 10);
    assert_eq!(parsed.schedule.rotation_years, 3);
    assert_eq!(parsed.declared_total_years, 30);
    assert_eq!(parsed.schedule.slots.len(), 90);
}

#[test]
fn strict_mode_accepts_supported_datver_branches() {
    for fixture in [
        "canonical_cropland_nonzero_95_7.man",
        "canonical_cropland_nonzero_98_4.man",
        "canonical_cropland_nonzero_2016_3.man",
        "canonical_cropland_nonzero_2017_1.man",
    ] {
        parse_management_from_path(fixture_path(fixture), ParseMode::Strict)
            .unwrap_or_else(|err| panic!("fixture {fixture} should parse: {err}"));
    }
}

#[test]
fn compatibility_mode_accepts_trailing_tokens_on_control_records() {
    let parsed = parse_management_from_path(
        fixture_path("compat_trailing_tokens_ok.man"),
        ParseMode::Compatibility,
    )
    .expect("compat mode should accept first token from control records");

    assert_eq!(parsed.datver, "95.7");
    assert_eq!(parsed.declared_total_years, 1);
}

#[test]
fn strict_mode_rejects_tilseq_zero_when_nseq_nonzero() {
    let err = parse_management_from_path(
        fixture_path("compat_tilseq_zero_nonzero_nseq_98_4.man"),
        ParseMode::Strict,
    )
    .expect_err("strict mode must reject tilseq=0 when nseq>0");

    match err {
        ManagementParseError::DanglingScenarioReference {
            field,
            value,
            max_allowed,
        } => {
            assert_eq!(field, "tilseq");
            assert_eq!(value, 0);
            assert_eq!(max_allowed, 1);
            assert_eq!(err.contract_error_id(), "MAN-E-009");
        }
        other => panic!("unexpected error variant: {other:?}"),
    }
}

#[test]
fn compatibility_mode_accepts_tilseq_zero_when_nseq_nonzero() {
    let parsed = parse_management_from_path(
        fixture_path("compat_tilseq_zero_nonzero_nseq_98_4.man"),
        ParseMode::Compatibility,
    )
    .expect("compatibility mode should accept tilseq=0 sentinel");

    let YearlyScenarioData::Cropland(cropland) = &parsed.registries.yearlies[0].data;
    assert_eq!(cropland.tilseq, 0);
}

#[test]
fn strict_mode_rejects_trailing_tokens_on_control_records() {
    let err = parse_management_from_path(
        fixture_path("compat_trailing_tokens_ok.man"),
        ParseMode::Strict,
    )
    .expect_err("strict mode must reject trailing tokens");

    match err {
        ManagementParseError::TokenParseError { field, .. } => {
            assert_eq!(field, "datver");
            assert_eq!(err.contract_error_id(), "MAN-E-001");
        }
        other => panic!("unexpected error variant: {other:?}"),
    }
}

#[test]
fn unsupported_datver_rejected() {
    let err = parse_management_from_path(fixture_path("unsupported_datver.man"), ParseMode::Strict)
        .expect_err("unknown datver should be rejected");

    match err {
        ManagementParseError::UnsupportedDatver { ref datver } => {
            assert_eq!(datver, "99.9");
            assert_eq!(err.contract_error_id(), "MAN-E-003");
        }
        other => panic!("unexpected error variant: {other:?}"),
    }
}

#[test]
fn missing_required_line_rejected() {
    let err =
        parse_management_from_path(fixture_path("missing_required_line.man"), ParseMode::Strict)
            .expect_err("missing record must be rejected");

    match err {
        ManagementParseError::MissingRecord { .. } => {
            assert_eq!(err.contract_error_id(), "MAN-E-002");
        }
        other => panic!("unexpected error variant: {other:?}"),
    }
}

#[test]
fn negative_count_rejected() {
    let err = parse_management_from_path(fixture_path("negative_count.man"), ParseMode::Strict)
        .expect_err("negative section count must be rejected");

    match err {
        ManagementParseError::InvalidCount { field, value } => {
            assert_eq!(field, "ncrop");
            assert_eq!(value, -1);
            assert_eq!(err.contract_error_id(), "MAN-E-005");
        }
        other => panic!("unexpected error variant: {other:?}"),
    }
}

#[test]
fn total_year_closure_enforced() {
    let err = parse_management_from_path(
        fixture_path("malformed_total_year_mismatch.man"),
        ParseMode::Strict,
    )
    .expect_err("declared and derived total years must match");

    match err {
        ManagementParseError::TotalYearMismatch {
            declared_total_years,
            derived_total_years,
        } => {
            assert_eq!(declared_total_years, 2);
            assert_eq!(derived_total_years, 1);
            assert_eq!(err.contract_error_id(), "MAN-E-008");
        }
        other => panic!("unexpected error variant: {other:?}"),
    }
}

#[test]
fn dangling_yearly_reference_rejected() {
    let err = parse_management_from_path(
        fixture_path("malformed_dangling_yearly_ref.man"),
        ParseMode::Strict,
    )
    .expect_err("dangling manindx must be rejected");

    match err {
        ManagementParseError::DanglingScenarioReference {
            field,
            value,
            max_allowed,
        } => {
            assert_eq!(field, "manindx");
            assert_eq!(value, 99);
            assert_eq!(max_allowed, 1);
            assert_eq!(err.contract_error_id(), "MAN-E-009");
        }
        other => panic!("unexpected error variant: {other:?}"),
    }
}

#[test]
fn dangling_operation_reference_rejected() {
    let err = parse_management_from_path(
        fixture_path("malformed_dangling_op_ref.man"),
        ParseMode::Strict,
    )
    .expect_err("dangling operation reference must be rejected");

    match err {
        ManagementParseError::DanglingScenarioReference {
            field,
            value,
            max_allowed,
        } => {
            assert_eq!(field, "op");
            assert_eq!(value, 9);
            assert_eq!(max_allowed, 1);
            assert_eq!(err.contract_error_id(), "MAN-E-009");
        }
        other => panic!("unexpected error variant: {other:?}"),
    }
}

#[test]
fn invalid_surface_date_domain_rejected() {
    let err = parse_management_from_path(
        fixture_path("malformed_invalid_surface_date.man"),
        ParseMode::Strict,
    )
    .expect_err("invalid julian day domain must be rejected");

    match err {
        ManagementParseError::DateDomainError { field, value, .. } => {
            assert_eq!(field, "mdate");
            assert_eq!(value, 367);
            assert_eq!(err.contract_error_id(), "MAN-E-010");
        }
        other => panic!("unexpected error variant: {other:?}"),
    }
}

#[test]
fn rangeland_landuse_is_explicitly_unsupported() {
    let err = parse_management_from_path(
        fixture_path("malformed_rangeland_unsupported.man"),
        ParseMode::Strict,
    )
    .expect_err("rangeland path must be rejected with typed unsupported behavior");

    match err {
        ManagementParseError::UnsupportedLanduse { section, landuse } => {
            assert_eq!(section, "yearly");
            assert_eq!(landuse, 2);
            assert_eq!(err.contract_error_id(), "MAN-E-004");
        }
        other => panic!("unexpected error variant: {other:?}"),
    }
}

#[test]
fn open_failure_is_typed() {
    let err = parse_management_from_path(fixture_path("does_not_exist.man"), ParseMode::Strict)
        .expect_err("missing file should produce typed open error");

    match err {
        ManagementParseError::InputOpenError { .. } => {
            assert_eq!(err.contract_error_id(), "MAN-E-002");
        }
        other => panic!("unexpected error variant: {other:?}"),
    }
}
