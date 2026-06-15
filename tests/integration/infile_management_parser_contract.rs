use std::fs;
use std::path::{Path, PathBuf};

use openwepp_input_contract::parsers::management::{
    ManagementParseError, ParseMode, YearlyCroplandBranch, YearlyScenarioData,
    parse_management_from_path, parse_management_from_str,
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

fn fixture_text(name: &str) -> String {
    fs::read_to_string(fixture_path(name)).expect("fixture should be readable")
}

fn fixture_with_yearly_branch(name: &str, branch: &str) -> String {
    const ANNUAL_BRANCH: &str = "1 # management <annual>
   288  # harvest date --- 10 / 15
   130  # planting date --- 5 /10
   0.7620  # row width
   6   # residue man - <none>";

    let fixture = fixture_text(name);
    assert!(
        fixture.contains(ANNUAL_BRANCH),
        "fixture should contain canonical annual yearly branch"
    );
    fixture.replace(ANNUAL_BRANCH, branch)
}

fn parse_strict_fixture_text(
    text: &str,
) -> openwepp_input_contract::parsers::management::ManagementParseOutput {
    parse_management_from_str(text, ParseMode::Strict).expect("strict parser should accept fixture")
}

fn assert_f64_eq(actual: f64, expected: f64) {
    assert!(
        (actual - expected).abs() <= f64::EPSILON,
        "expected {expected}, got {actual}"
    );
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
fn strict_mode_parses_perennial_cutday_yearly_branch() {
    let fixture = fixture_with_yearly_branch(
        "canonical_cropland_nonzero_98_4.man",
        "2 # management <perennial>
   288  # harvest date --- 10 / 15
   130  # planting date --- 5 /10
   300  # stop date
   0.7620  # row width
   1   # mgtopt - cut days
   2   # ncut
   120
   240 0.50",
    );

    let parsed = parse_strict_fixture_text(&fixture);
    let YearlyScenarioData::Cropland(cropland) = &parsed.registries.yearlies[0].data;
    match &cropland.branch {
        YearlyCroplandBranch::Perennial(perennial) => {
            assert_eq!(perennial.jdharv, 288);
            assert_eq!(perennial.jdplt, 130);
            assert_eq!(perennial.jdstop, 300);
            assert_f64_eq(perennial.rw, 0.7620);
            assert_eq!(perennial.mgtopt, 1);
            assert_eq!(perennial.cut_days, vec![120, 240]);
            assert!(perennial.grazing_cycles.is_empty());
        }
        YearlyCroplandBranch::AnnualOrFallow(other) => {
            panic!("unexpected annual/fallow branch: {other:?}");
        }
    }
}

#[test]
fn strict_mode_parses_perennial_grazing_yearly_branch() {
    let fixture = fixture_with_yearly_branch(
        "canonical_cropland_nonzero_98_4.man",
        "2 # management <perennial>
   288  # harvest date --- 10 / 15
   130  # planting date --- 5 /10
   300  # stop date
   0.7620  # row width
   2   # mgtopt - grazing
   2   # ncycle
   1.0 2.0 3.0 4.0
   121
   150
   5.0 6.0 7.0 8.0
   200
   220",
    );

    let parsed = parse_strict_fixture_text(&fixture);
    let YearlyScenarioData::Cropland(cropland) = &parsed.registries.yearlies[0].data;
    match &cropland.branch {
        YearlyCroplandBranch::Perennial(perennial) => {
            assert_eq!(perennial.mgtopt, 2);
            assert!(perennial.cut_days.is_empty());
            assert_eq!(perennial.grazing_cycles.len(), 2);
            assert_f64_eq(perennial.grazing_cycles[0].animal, 1.0);
            assert_f64_eq(perennial.grazing_cycles[0].area, 2.0);
            assert_f64_eq(perennial.grazing_cycles[0].bodywt, 3.0);
            assert_f64_eq(perennial.grazing_cycles[0].digest, 4.0);
            assert_eq!(perennial.grazing_cycles[0].gday, 121);
            assert_eq!(perennial.grazing_cycles[0].gend, 150);
            assert_f64_eq(perennial.grazing_cycles[1].animal, 5.0);
            assert_f64_eq(perennial.grazing_cycles[1].area, 6.0);
            assert_f64_eq(perennial.grazing_cycles[1].bodywt, 7.0);
            assert_f64_eq(perennial.grazing_cycles[1].digest, 8.0);
            assert_eq!(perennial.grazing_cycles[1].gday, 200);
            assert_eq!(perennial.grazing_cycles[1].gend, 220);
        }
        YearlyCroplandBranch::AnnualOrFallow(other) => {
            panic!("unexpected annual/fallow branch: {other:?}");
        }
    }
}

#[test]
fn strict_mode_parses_perennial_no_action_yearly_branch() {
    let fixture = fixture_with_yearly_branch(
        "canonical_cropland_nonzero_98_4.man",
        "2 # management <perennial>
   288  # harvest date --- 10 / 15
   130  # planting date --- 5 /10
   300  # stop date
   0.7620  # row width
   3   # mgtopt - none",
    );

    let parsed = parse_strict_fixture_text(&fixture);
    let YearlyScenarioData::Cropland(cropland) = &parsed.registries.yearlies[0].data;
    match &cropland.branch {
        YearlyCroplandBranch::Perennial(perennial) => {
            assert_eq!(perennial.mgtopt, 3);
            assert!(perennial.cut_days.is_empty());
            assert!(perennial.grazing_cycles.is_empty());
        }
        YearlyCroplandBranch::AnnualOrFallow(other) => {
            panic!("unexpected annual/fallow branch: {other:?}");
        }
    }
}

#[test]
fn strict_mode_rejects_legacy_perennial_mgtopt_out_of_domain() {
    let fixture = fixture_with_yearly_branch(
        "canonical_cropland_nonzero_98_4.man",
        "2 # management <perennial>
   288  # harvest date --- 10 / 15
   130  # planting date --- 5 /10
   300  # stop date
   0.7620  # row width
   4   # mgtopt - datver-invalid",
    );

    let err = parse_management_from_str(&fixture, ParseMode::Strict)
        .expect_err("legacy datver must reject mgtopt outside 1..3");

    match err {
        ManagementParseError::InvalidOptionDomain {
            field,
            value,
            allowed,
        } => {
            assert_eq!(field, "mgtopt");
            assert_eq!(value, 4);
            assert_eq!(allowed, "1..3");
            assert_eq!(err.contract_error_id(), "MAN-E-004");
        }
        other => panic!("unexpected error variant: {other:?}"),
    }
}

#[test]
fn strict_mode_rejects_unimplemented_2016_perennial_mgtopt() {
    let fixture = fixture_with_yearly_branch(
        "canonical_cropland_nonzero_2016_3.man",
        "2 # management <perennial>
   288  # harvest date --- 10 / 15
   130  # planting date --- 5 /10
   300  # stop date
   0.7620  # row width
   4   # mgtopt - parser-unsupported",
    );

    let err = parse_management_from_str(&fixture, ParseMode::Strict)
        .expect_err("2016+ datver should reject currently unsupported mgtopt");

    match err {
        ManagementParseError::InvalidOptionDomain {
            field,
            value,
            allowed,
        } => {
            assert_eq!(field, "mgtopt");
            assert_eq!(value, 4);
            assert_eq!(
                allowed,
                "openWEPP parser currently supports perennial mgtopt 1..3"
            );
            assert_eq!(err.contract_error_id(), "MAN-E-004");
        }
        other => panic!("unexpected error variant: {other:?}"),
    }
}

#[test]
fn strict_mode_rejects_perennial_zero_cut_count() {
    let fixture = fixture_with_yearly_branch(
        "canonical_cropland_nonzero_98_4.man",
        "2 # management <perennial>
   288  # harvest date --- 10 / 15
   130  # planting date --- 5 /10
   300  # stop date
   0.7620  # row width
   1   # mgtopt - cut days
   0   # ncut",
    );

    let err = parse_management_from_str(&fixture, ParseMode::Strict)
        .expect_err("perennial cut branch must reject zero cut count");

    match err {
        ManagementParseError::InvalidCount { field, value } => {
            assert_eq!(field, "ncut");
            assert_eq!(value, 0);
            assert_eq!(err.contract_error_id(), "MAN-E-005");
        }
        other => panic!("unexpected error variant: {other:?}"),
    }
}

#[test]
fn strict_mode_rejects_perennial_cutday_arity() {
    let fixture = fixture_with_yearly_branch(
        "canonical_cropland_nonzero_98_4.man",
        "2 # management <perennial>
   288  # harvest date --- 10 / 15
   130  # planting date --- 5 /10
   300  # stop date
   0.7620  # row width
   1   # mgtopt - cut days
   1   # ncut
   120 0.50 0.25",
    );

    let err = parse_management_from_str(&fixture, ParseMode::Strict)
        .expect_err("cutday rows must reject more than two tokens");

    match err {
        ManagementParseError::RecordArityError {
            field,
            observed,
            expected,
        } => {
            assert_eq!(field, "cutday");
            assert_eq!(observed, 3);
            assert_eq!(expected, "1 or 2");
            assert_eq!(err.contract_error_id(), "MAN-E-002");
        }
        other => panic!("unexpected error variant: {other:?}"),
    }
}

#[test]
fn strict_mode_rejects_perennial_zero_grazing_cycle_count() {
    let fixture = fixture_with_yearly_branch(
        "canonical_cropland_nonzero_98_4.man",
        "2 # management <perennial>
   288  # harvest date --- 10 / 15
   130  # planting date --- 5 /10
   300  # stop date
   0.7620  # row width
   2   # mgtopt - grazing
   0   # ncycle",
    );

    let err = parse_management_from_str(&fixture, ParseMode::Strict)
        .expect_err("perennial grazing branch must reject zero cycle count");

    match err {
        ManagementParseError::InvalidCount { field, value } => {
            assert_eq!(field, "ncycle");
            assert_eq!(value, 0);
            assert_eq!(err.contract_error_id(), "MAN-E-005");
        }
        other => panic!("unexpected error variant: {other:?}"),
    }
}

#[test]
fn strict_mode_rejects_perennial_grazing_cycle_arity() {
    let fixture = fixture_with_yearly_branch(
        "canonical_cropland_nonzero_98_4.man",
        "2 # management <perennial>
   288  # harvest date --- 10 / 15
   130  # planting date --- 5 /10
   300  # stop date
   0.7620  # row width
   2   # mgtopt - grazing
   1   # ncycle
   1.0 2.0 3.0",
    );

    let err = parse_management_from_str(&fixture, ParseMode::Strict)
        .expect_err("graze cycle row must have four tokens");

    match err {
        ManagementParseError::RecordArityError {
            field,
            observed,
            expected,
        } => {
            assert_eq!(field, "graze_cycle");
            assert_eq!(observed, 3);
            assert_eq!(expected, "4");
            assert_eq!(err.contract_error_id(), "MAN-E-002");
        }
        other => panic!("unexpected error variant: {other:?}"),
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
