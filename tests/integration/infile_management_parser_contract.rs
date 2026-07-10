use std::fs;
use std::io;
use std::path::{Path, PathBuf};

use openwepp_input_contract::parsers::management::{
    InitialScenarioData, ManagementParseError, OperationScenarioData, ParseMode, PlantScenarioData,
    YearlyAnnualExtension, YearlyAnnualFallowData, YearlyCroplandBranch, YearlyScenarioData,
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

fn parse_annual_branch_from_fixture(name: &str, branch: &str) -> YearlyAnnualFallowData {
    let parsed = parse_strict_fixture_text(&fixture_with_yearly_branch(name, branch));
    let YearlyScenarioData::Cropland(cropland) = &parsed.registries.yearlies[0].data else {
        panic!("expected cropland yearly scenario");
    };
    match &cropland.branch {
        YearlyCroplandBranch::AnnualOrFallow(annual) => annual.clone(),
        YearlyCroplandBranch::Perennial(other) => {
            panic!("unexpected perennial branch: {other:?}");
        }
    }
}

fn parse_annual_branch(branch: &str) -> YearlyAnnualFallowData {
    parse_annual_branch_from_fixture("canonical_cropland_nonzero_98_4.man", branch)
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
fn display_includes_contract_ids_for_error_variants() {
    let errors = vec![
        ManagementParseError::InputOpenError {
            path: PathBuf::from("missing.man"),
            source: io::Error::new(io::ErrorKind::NotFound, "not found"),
        },
        ManagementParseError::MissingRecord { field: "field" },
        ManagementParseError::TokenParseError {
            field: "field",
            value: "abc".to_string(),
        },
        ManagementParseError::RecordArityError {
            field: "field",
            observed: 1,
            expected: "2",
        },
        ManagementParseError::UnsupportedDatver {
            datver: "99.9".to_string(),
        },
        ManagementParseError::InvalidCount {
            field: "count",
            value: -1,
        },
        ManagementParseError::InvalidOptionDomain {
            field: "option",
            value: 9,
            allowed: "1..2",
        },
        ManagementParseError::UnsupportedLanduse {
            section: "plant",
            landuse: 2,
        },
        ManagementParseError::ForestSectionNotApplicable { section: "drain" },
        ManagementParseError::DanglingScenarioReference {
            field: "ref",
            value: 3,
            max_allowed: 1,
        },
        ManagementParseError::TotalYearMismatch {
            declared_total_years: 3,
            derived_total_years: 2,
        },
        ManagementParseError::DateDomainError {
            field: "jdplt",
            value: 367,
            allowed: "1..366",
        },
        ManagementParseError::TrailingInput {
            first_unconsumed_line: 42,
        },
        ManagementParseError::YamlInputError {
            detail: "bad yaml".to_string(),
        },
    ];

    for error in &errors {
        let message = error.to_string();
        assert!(
            message.starts_with(error.contract_error_id()),
            "display message should start with contract id: {message}"
        );
    }
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
fn strict_mode_parses_operation_cltpos_and_extension_lines() {
    let cltpos_fixture = fixture_text("canonical_cropland_nonzero_98_4.man")
        .replace("4 # pcode - other", "3 2 # pcode - cultivator position");
    let parsed = parse_strict_fixture_text(&cltpos_fixture);
    let OperationScenarioData::Cropland(operation) = &parsed.registries.operations[0].data;
    assert_eq!(operation.pcode, 3);
    assert_eq!(operation.cltpos, Some(2));
    assert!(operation.extension_lines.is_empty());

    let extension_fixture = fixture_text("canonical_cropland_nonzero_98_4.man")
        .replace("4 # pcode - other", "10 # pcode - extension")
        .replace(
            "0.0250 0.7500 0.2500 0.1500 0.0120 0.1500 0.0000",
            "0.0250 0.7500 0.2500 0.1500 0.0120 0.1500 0.0000\n0.42\n1.0 2.0",
        );
    let parsed = parse_strict_fixture_text(&extension_fixture);
    let OperationScenarioData::Cropland(operation) = &parsed.registries.operations[0].data;
    assert_eq!(operation.pcode, 10);
    assert_eq!(
        operation.extension_lines,
        vec!["0.42".to_string(), "1.0 2.0".to_string()]
    );
}

#[test]
fn strict_mode_rejects_operation_cltpos_shape_and_domain_errors() {
    let missing_cltpos = fixture_text("canonical_cropland_nonzero_98_4.man")
        .replace("4 # pcode - other", "3 # pcode - missing cltpos");
    let err = parse_management_from_str(&missing_cltpos, ParseMode::Strict)
        .expect_err("pcode 3 must include cltpos");
    match err {
        ManagementParseError::RecordArityError {
            field,
            observed,
            expected,
        } => {
            assert_eq!(field, "op.cltpos");
            assert_eq!(observed, 1);
            assert_eq!(expected, "2");
            assert_eq!(err.contract_error_id(), "MAN-E-002");
        }
        other => panic!("unexpected error variant: {other:?}"),
    }

    let invalid_cltpos = fixture_text("canonical_cropland_nonzero_98_4.man")
        .replace("4 # pcode - other", "3 9 # pcode - invalid cltpos");
    let err = parse_management_from_str(&invalid_cltpos, ParseMode::Strict)
        .expect_err("cltpos must stay in its allowed domain");
    match err {
        ManagementParseError::InvalidOptionDomain {
            field,
            value,
            allowed,
        } => {
            assert_eq!(field, "cltpos");
            assert_eq!(value, 9);
            assert_eq!(allowed, "1 or 2");
            assert_eq!(err.contract_error_id(), "MAN-E-004");
        }
        other => panic!("unexpected error variant: {other:?}"),
    }
}

#[test]
fn strict_mode_parses_contour_scenarios_by_datver_shape() {
    let legacy_fixture = fixture_text("canonical_cropland_nonzero_98_4.man")
        .replace(
            "0  # Number of contour scenarios",
            "1 # Number of contour scenarios\n\nContour One\ndesc 1\ndesc 2\ndesc 3\n1 # landuse\n0.10 0.20 30.0 0.762",
        )
        .replace("0  # contour scenario", "1 # contour scenario");
    let parsed = parse_strict_fixture_text(&legacy_fixture);
    assert_eq!(parsed.section_counts.ncnt, 1);
    assert_f64_eq(parsed.registries.contours[0].cntslp, 0.10);
    assert_f64_eq(parsed.registries.contours[0].rowspc, 0.762);
    assert_eq!(parsed.registries.contours[0].contours_perm, None);

    let modern_fixture = fixture_text("canonical_cropland_nonzero_2016_3.man")
        .replace(
            "0  # Number of contour scenarios",
            "1 # Number of contour scenarios\n\nContour One\ndesc 1\ndesc 2\ndesc 3\n1 # landuse\n0.10 0.20 30.0 0.762 1",
        )
        .replace("0  # contour scenario", "1 # contour scenario");
    let parsed = parse_strict_fixture_text(&modern_fixture);
    assert_eq!(parsed.section_counts.ncnt, 1);
    assert_eq!(parsed.registries.contours[0].contours_perm, Some(1));
}

#[test]
fn strict_mode_rejects_legacy_contours_perm_extension() {
    let fixture = fixture_text("canonical_cropland_nonzero_98_4.man")
        .replace(
            "0  # Number of contour scenarios",
            "1 # Number of contour scenarios\n\nContour One\ndesc 1\ndesc 2\ndesc 3\n1 # landuse\n0.10 0.20 30.0 0.762 1",
        )
        .replace("0  # contour scenario", "1 # contour scenario");
    let err = parse_management_from_str(&fixture, ParseMode::Strict)
        .expect_err("legacy datver must reject the 2016+ contour extension");
    match err {
        ManagementParseError::InvalidOptionDomain {
            field,
            value,
            allowed,
        } => {
            assert_eq!(field, "contours_perm");
            assert_eq!(value, 1);
            assert_eq!(allowed, "2016.3+ datver only");
            assert_eq!(err.contract_error_id(), "MAN-E-004");
        }
        other => panic!("unexpected error variant: {other:?}"),
    }
}

#[test]
fn strict_mode_parses_drain_scenario_references() {
    let fixture = fixture_text("canonical_cropland_nonzero_98_4.man")
        .replace(
            "0  # Number of drainage scenarios",
            "1 # Number of drainage scenarios\n\nDrain One\ndesc 1\ndesc 2\ndesc 3\n1 # landuse\n1.0 2.0 3.0 4.0",
        )
        .replace("0  # drainage scenario", "1 # drainage scenario");

    let parsed = parse_strict_fixture_text(&fixture);
    assert_eq!(parsed.section_counts.ndrain, 1);
    assert_f64_eq(parsed.registries.drains[0].ddrain, 1.0);
    assert_f64_eq(parsed.registries.drains[0].drainc, 2.0);
    assert_f64_eq(parsed.registries.drains[0].drdiam, 3.0);
    assert_f64_eq(parsed.registries.drains[0].sdrain, 4.0);
}

#[test]
fn strict_mode_parses_initial_understory_and_rejects_invalid_options() {
    let understory_fixture = fixture_text("canonical_cropland_nonzero_98_4.man")
        .replace("0.50003 0.19997", "0.50003 0.19997 0.12 0.34");
    let parsed = parse_strict_fixture_text(&understory_fixture);
    let InitialScenarioData::Cropland(initial) = &parsed.registries.initials[0].data else {
        panic!("expected cropland initial scenario");
    };
    assert_eq!(initial.understory_line, Some([0.12, 0.34]));

    let invalid_imngmt = fixture_text("canonical_cropland_nonzero_98_4.man")
        .replace("1 # mang annual", "4 # mang invalid");
    let err = parse_management_from_str(&invalid_imngmt, ParseMode::Strict)
        .expect_err("imngmt must stay in its allowed domain");
    match err {
        ManagementParseError::InvalidOptionDomain { field, value, .. } => {
            assert_eq!(field, "imngmt");
            assert_eq!(value, 4);
            assert_eq!(err.contract_error_id(), "MAN-E-004");
        }
        other => panic!("unexpected error variant: {other:?}"),
    }

    let invalid_rtyp = fixture_text("canonical_cropland_nonzero_98_4.man")
        .replace("1  # rtyp - temporary", "3 # rtyp invalid");
    let err = parse_management_from_str(&invalid_rtyp, ParseMode::Strict)
        .expect_err("rtyp must stay in its allowed domain");
    match err {
        ManagementParseError::InvalidOptionDomain { field, value, .. } => {
            assert_eq!(field, "rtyp");
            assert_eq!(value, 3);
            assert_eq!(err.contract_error_id(), "MAN-E-004");
        }
        other => panic!("unexpected error variant: {other:?}"),
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
    let YearlyScenarioData::Cropland(cropland) = &parsed.registries.yearlies[0].data else {
        panic!("expected cropland yearly scenario");
    };
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
    let YearlyScenarioData::Cropland(cropland) = &parsed.registries.yearlies[0].data else {
        panic!("expected cropland yearly scenario");
    };
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
    let YearlyScenarioData::Cropland(cropland) = &parsed.registries.yearlies[0].data else {
        panic!("expected cropland yearly scenario");
    };
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
fn strict_mode_parses_annual_residue_management_extensions() {
    let herbicide = parse_annual_branch(
        "1 # management <annual>
   288  # harvest date --- 10 / 15
   130  # planting date --- 5 /10
   0.7620  # row width
   1   # residue man - <herbicide>
   200 # herbicide date",
    );
    assert_eq!(herbicide.jdharv, 288);
    assert_eq!(herbicide.jdplt, 130);
    assert_f64_eq(herbicide.rw, 0.7620);
    assert_eq!(herbicide.resmgt, 1);
    assert_eq!(
        herbicide.extension,
        Some(YearlyAnnualExtension::Herbicide { jdherb: 200 })
    );

    let burn = parse_annual_branch(
        "1 # management <annual>
   288  # harvest date --- 10 / 15
   130  # planting date --- 5 /10
   0.7620  # row width
   2   # residue man - <burn>
   250 # burn date
   0.30 # fbmag
   0.45 # fbrnog",
    );
    assert_eq!(
        burn.extension,
        Some(YearlyAnnualExtension::Burn {
            jdburn: 250,
            fbmag: 0.30,
            fbrnog: 0.45,
        })
    );

    let silage = parse_annual_branch(
        "1 # management <annual>
   288  # harvest date --- 10 / 15
   130  # planting date --- 5 /10
   0.7620  # row width
   3   # residue man - <silage>
   245 # silage date",
    );
    assert_eq!(
        silage.extension,
        Some(YearlyAnnualExtension::Silage { jdslge: 245 })
    );

    let cut = parse_annual_branch(
        "1 # management <annual>
   288  # harvest date --- 10 / 15
   130  # planting date --- 5 /10
   0.7620  # row width
   4   # residue man - <cut>
   180 # cut date
   0.55 # frcut",
    );
    assert_eq!(
        cut.extension,
        Some(YearlyAnnualExtension::Cut {
            jdcut: 180,
            frcut: 0.55,
        })
    );

    let remove = parse_annual_branch(
        "1 # management <annual>
   288  # harvest date --- 10 / 15
   130  # planting date --- 5 /10
   0.7620  # row width
   5   # residue man - <remove>
   190 # remove date
   0.65 # frmove",
    );
    assert_eq!(
        remove.extension,
        Some(YearlyAnnualExtension::Remove {
            jdmove: 190,
            frmove: 0.65,
        })
    );

    let none = parse_annual_branch(
        "1 # management <annual>
   288  # harvest date --- 10 / 15
   130  # planting date --- 5 /10
   0.7620  # row width
   6   # residue man - <none>",
    );
    assert_eq!(none.resmgt, 6);
    assert_eq!(none.extension, None);
}

#[test]
fn strict_mode_parses_2016_annual_cut_records() {
    let annual = parse_annual_branch_from_fixture(
        "canonical_cropland_nonzero_2016_3.man",
        "1 # management <annual>
   288  # harvest date --- 10 / 15
   130  # planting date --- 5 /10
   0.7620  # row width
   7   # residue man - <annual cut records>
   1   # annual cut flag
   2   # annual cut count
   120 0.50
   240 0.25",
    );
    assert_eq!(annual.resmgt, 7);
    assert_eq!(annual.extension, None);
}

#[test]
fn strict_mode_rejects_legacy_annual_residue_management_seven() {
    let fixture = fixture_with_yearly_branch(
        "canonical_cropland_nonzero_98_4.man",
        "1 # management <annual>
   288  # harvest date --- 10 / 15
   130  # planting date --- 5 /10
   0.7620  # row width
   7   # residue man - legacy-invalid",
    );

    let err = parse_management_from_str(&fixture, ParseMode::Strict)
        .expect_err("legacy datver must reject annual resmgt 7");

    match err {
        ManagementParseError::InvalidOptionDomain {
            field,
            value,
            allowed,
        } => {
            assert_eq!(field, "resmgt");
            assert_eq!(value, 7);
            assert_eq!(allowed, "1..6 (95.7/98.4) or 1..7 (2016.3+)");
            assert_eq!(err.contract_error_id(), "MAN-E-004");
        }
        other => panic!("unexpected error variant: {other:?}"),
    }
}

#[test]
fn strict_mode_rejects_annual_cut_zero_count() {
    let fixture = fixture_with_yearly_branch(
        "canonical_cropland_nonzero_2016_3.man",
        "1 # management <annual>
   288  # harvest date --- 10 / 15
   130  # planting date --- 5 /10
   0.7620  # row width
   7   # residue man - <annual cut records>
   1   # annual cut flag
   0   # annual cut count",
    );

    let err = parse_management_from_str(&fixture, ParseMode::Strict)
        .expect_err("annual cut branch must reject zero cut count");

    match err {
        ManagementParseError::InvalidCount { field, value } => {
            assert_eq!(field, "annual_cut.ncut");
            assert_eq!(value, 0);
            assert_eq!(err.contract_error_id(), "MAN-E-005");
        }
        other => panic!("unexpected error variant: {other:?}"),
    }
}

#[test]
fn strict_mode_rejects_annual_cut_short_entry() {
    let fixture = fixture_with_yearly_branch(
        "canonical_cropland_nonzero_2016_3.man",
        "1 # management <annual>
   288  # harvest date --- 10 / 15
   130  # planting date --- 5 /10
   0.7620  # row width
   7   # residue man - <annual cut records>
   1   # annual cut flag
   1   # annual cut count
   120",
    );

    let err = parse_management_from_str(&fixture, ParseMode::Strict)
        .expect_err("annual cut branch must reject short cut entries");

    match err {
        ManagementParseError::RecordArityError {
            field,
            observed,
            expected,
        } => {
            assert_eq!(field, "annual_cut.entry");
            assert_eq!(observed, 1);
            assert_eq!(expected, "2+");
            assert_eq!(err.contract_error_id(), "MAN-E-002");
        }
        other => panic!("unexpected error variant: {other:?}"),
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

    let YearlyScenarioData::Cropland(cropland) = &parsed.registries.yearlies[0].data else {
        panic!("expected cropland yearly scenario");
    };
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

// ---------------------------------------------------------------------------
// DFF-WS1 Increment-2: openWEPP-native forest `lanuse` mode (`ow-lanuse-1`).
// ---------------------------------------------------------------------------

const FOREST_FIXTURE: &str = "canonical_forest_nonzero_ow_lanuse_1.man";

fn assert_f64_close(actual: f64, expected: f64) {
    assert!(
        (actual - expected).abs() <= 1.0e-9,
        "expected {expected}, observed {actual}"
    );
}

#[test]
fn native_ow_lanuse_1_datver_parses_forest_scenarios() {
    let parsed = parse_management_from_path(fixture_path(FOREST_FIXTURE), ParseMode::Strict)
        .expect("native forest fixture should parse under the ow-lanuse-1 datver");

    assert_eq!(parsed.datver, "ow-lanuse-1");
    assert_eq!(parsed.registries.plants.len(), 1);
    assert_eq!(parsed.registries.initials.len(), 1);
    assert_eq!(parsed.registries.yearlies.len(), 1);

    // Plant: first-class forest block with Tier-A operands.
    let PlantScenarioData::Forest(plant) = &parsed.registries.plants[0].data else {
        panic!("expected forest plant scenario");
    };
    assert_eq!(plant.forest_class, "forest_high_sev_fire");
    assert_f64_close(plant.growth.bb, 14.0);
    assert_f64_close(plant.growth.bbb, 3.0);
    assert_f64_close(plant.growth.btemp, 2.0);
    assert_f64_close(plant.growth.extnct, 0.45);
    assert_f64_close(plant.growth.flivmx, 17.0);
    assert_f64_close(plant.growth.hmax, 0.2);
    assert_f64_close(plant.growth.hi, 0.42);
    assert_f64_close(plant.growth.dlai, 0.5);
    assert_f64_close(plant.growth.otemp, 20.0);
    assert_f64_close(plant.growth.spriod, 90.0);
    assert_f64_close(plant.growth.rsr, 0.33);
    assert_f64_close(plant.growth.rtmmax, 0.2);
    // Lookup-owned Tier-A operands (authoritative `forest high sev fire` row).
    assert_f64_close(plant.growth.xmxlai, 2.0);
    assert_f64_close(plant.growth.rdmax, 0.3);
    assert_f64_close(plant.growth.decfct, 1.0);
    assert_f64_close(plant.growth.dropfc, 1.0);
    assert_f64_close(plant.cf, 5.0);
    assert_f64_close(plant.diam, 0.005);
    assert_f64_close(plant.decomposition.oratea, 0.0);
    // Tier-B community structure (stored now; WS-4 consumes it).
    assert_f64_close(plant.community.tempmn, -5.0);
    assert_f64_close(plant.community.tree.pop, 500.0);

    // Initial: first-class forest cover/roughness.
    let InitialScenarioData::Forest(initial) = &parsed.registries.initials[0].data else {
        panic!("expected forest initial scenario");
    };
    assert_f64_close(initial.cancov, 0.4);
    assert_f64_close(initial.inrcov, 0.3);
    assert_f64_close(initial.rilcov, 0.3);
    assert_f64_close(initial.rrinit, 0.06);
    assert_eq!(initial.iresd, 1);
    assert_eq!(initial.imngmt, 2);

    // Yearly: established perennial forest slot.
    let YearlyScenarioData::Forest(yearly) = &parsed.registries.yearlies[0].data else {
        panic!("expected forest yearly scenario");
    };
    assert_eq!(yearly.itype, 1);
    assert_eq!(yearly.jdharv, 0);
    assert_eq!(yearly.jdplt, 0);
    assert_eq!(yearly.jdstop, 0);
    assert_f64_close(yearly.rw, 0.0);
}

#[test]
fn native_ow_lanuse_1_forest_parses_routing_coefficients_extension() {
    let native = fixture_text(FOREST_FIXTURE);
    let with_routing = native.replace(
        "0.02000 2.00000 8.00000 500.00000           # tree: coeff diam hgt pop",
        "0.02000 2.00000 8.00000 500.00000           # tree: coeff diam hgt pop\nrouting_coefficients\n500.00000 1.25000 0.06000 0.20000 0.70000",
    );

    let parsed = parse_management_from_str(&with_routing, ParseMode::Strict)
        .expect("forest routing coefficient extension should parse");
    let PlantScenarioData::Forest(plant) = &parsed.registries.plants[0].data else {
        panic!("expected forest plant scenario");
    };
    let routing = plant
        .routing
        .expect("forest routing coefficients should be present");
    assert_f64_close(routing.skin_friction_coefficient_ko, 500.0);
    assert_f64_close(routing.form_drag_coefficient, 1.25);
    assert_f64_close(routing.roughness_element_height_m, 0.06);
    assert_f64_close(routing.roughness_concentration, 0.2);
    assert_f64_close(routing.vegetation_drag_coefficient, 0.7);
}

#[test]
fn native_ow_lanuse_1_accepts_native_cropland_sentinel_with_routing_coefficients() {
    let native = fixture_text("canonical_cropland_nonzero_98_4.man")
        .replacen("98.4", "ow-lanuse-1", 1)
        .replace("1  #landuse", "4  #landuse")
        .replace(
            "1  # landuse  - cropland",
            "4  # landuse  - native cropland",
        )
        .replace(
            "1  # landuse <cropland>",
            "4  # landuse <native cropland>",
        )
        .replace(
            "0.00000 3.50000 0.00000",
            "0.00000 3.50000 0.00000\nrouting_coefficients\n650.00000 0.80000 0.03000 0.10000 0.40000",
        );

    let parsed = parse_management_from_str(&native, ParseMode::Strict)
        .expect("native cropland sentinel should parse under ow-lanuse-1");
    assert_eq!(parsed.registries.plants[0].meta.landuse, 4);
    assert_eq!(parsed.registries.operations[0].meta.landuse, 4);
    assert_eq!(parsed.registries.initials[0].meta.landuse, 4);
    assert_eq!(parsed.registries.surfaces[0].meta.landuse, 4);
    assert_eq!(parsed.registries.yearlies[0].meta.landuse, 4);

    let PlantScenarioData::Cropland(plant) = &parsed.registries.plants[0].data else {
        panic!("expected native cropland plant scenario");
    };
    let routing = plant
        .routing
        .expect("native cropland routing coefficients should be present");
    assert_f64_close(routing.skin_friction_coefficient_ko, 650.0);
    assert_f64_close(routing.form_drag_coefficient, 0.8);
    assert_f64_close(routing.roughness_element_height_m, 0.03);
    assert_f64_close(routing.roughness_concentration, 0.1);
    assert_f64_close(routing.vegetation_drag_coefficient, 0.4);
}

#[test]
fn forest_sentinel_rejected_under_legacy_datver() {
    // Same fixture but with a legacy datver: the forest sentinel (landuse 3)
    // stays rejected (compatibility quarantine, LANUSE-AUTH-4).
    let native = fixture_text(FOREST_FIXTURE);
    let legacy = native.replacen("ow-lanuse-1", "98.4", 1);
    let err = parse_management_from_str(&legacy, ParseMode::Strict)
        .expect_err("forest sentinel must be rejected under a legacy datver");
    match err {
        ManagementParseError::InvalidOptionDomain {
            field: "iplant", ..
        } => {
            assert_eq!(err.contract_error_id(), "MAN-E-004");
        }
        other => panic!("unexpected error variant: {other:?}"),
    }
}

#[test]
fn forest_scenario_in_operation_section_fails_closed() {
    // Inject a forest operation scenario (nop=1) into the native fixture; the
    // operation section has no forest payload and must fail closed.
    let native = fixture_text(FOREST_FIXTURE);
    let with_forest_op = native.replace(
        "0 # nop",
        "1 # nop\nForest_Op\n(null)\n(null)\n(null)\n3 # Landuse - <Forest>",
    );
    let err = parse_management_from_str(&with_forest_op, ParseMode::Strict)
        .expect_err("forest scenario in the operation section must fail closed");
    match err {
        ManagementParseError::ForestSectionNotApplicable {
            section: "operation",
        } => {
            assert_eq!(err.contract_error_id(), "MAN-E-004");
        }
        other => panic!("unexpected error variant: {other:?}"),
    }
}

#[test]
fn forest_yearly_rejects_nonzero_surface_effect_reference() {
    // A forest yearly slot must carry `tilseq = 0` (no surface effect).
    let native = fixture_text(FOREST_FIXTURE);
    let tampered = native.replace(
        "0     # Surface Effect Scenario index (tilseq)",
        "1     # Surface Effect Scenario index (tilseq)",
    );
    let err = parse_management_from_str(&tampered, ParseMode::Strict)
        .expect_err("forest yearly tilseq must be the 0 sentinel");
    match err {
        ManagementParseError::InvalidOptionDomain {
            field: "tilseq", ..
        } => {
            assert_eq!(err.contract_error_id(), "MAN-E-004");
        }
        other => panic!("unexpected error variant: {other:?}"),
    }
}

#[test]
fn forest_plant_block_missing_operand_fails_closed() {
    // Drop a value from the lookup-owned Tier-A line: parse must fail closed
    // (LANUSE-AUTH-2 typed presence), not substitute a default.
    let native = fixture_text(FOREST_FIXTURE);
    let tampered = native.replace(
        "2.00000 0.30000 1.00000 1.00000             # lookup-owned: xmxlai rdmax decfct dropfc",
        "2.00000 0.30000 1.00000                     # lookup-owned: missing dropfc",
    );
    let err = parse_management_from_str(&tampered, ParseMode::Strict)
        .expect_err("missing forest Tier-A operand must fail closed");
    match err {
        ManagementParseError::RecordArityError { .. } => {}
        other => panic!("unexpected error variant: {other:?}"),
    }
}

#[test]
fn forest_scenario_blank_description_slots_preserved() {
    // The `#landuse` blank-slot keep logic in `normalize_lines` is landuse
    // agnostic: a forest scenario authored with genuinely blank description
    // lines and a lowercase `#landuse` marker parses with its slots aligned.
    let native = fixture_text(FOREST_FIXTURE);
    let with_blank_desc = native.replace(
        "Forest_High_Severity_Fire
Native forest lanuse mode (disturbed class: forest high sev fire)
openWEPP DFF-WS1 native forest lanuse-v1
(null)
3 # Landuse - <Forest>",
        "Forest_High_Severity_Fire\n\n\n\n3 #landuse forest",
    );
    let parsed = parse_management_from_str(&with_blank_desc, ParseMode::Strict)
        .expect("forest scenario with blank description slots should parse");
    let PlantScenarioData::Forest(plant) = &parsed.registries.plants[0].data else {
        panic!("expected forest plant scenario");
    };
    assert_eq!(plant.forest_class, "forest_high_sev_fire");
    assert_eq!(
        parsed.registries.plants[0].meta.description,
        [String::new(), String::new(), String::new()]
    );
}
