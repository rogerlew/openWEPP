use std::fs;
use std::path::Path;

const HPARITY01_ALWAYS_FAIL_COLUMNS: [&str; 12] = [
    "Dp",
    "Ep",
    "Es",
    "ProfileDepth",
    "ProfileFCStore",
    "ProfilePorosityCap",
    "ProfileWPStore",
    "RM",
    "Snow-Water",
    "SoilWaterTotal",
    "Total-Soil",
    "latqcc",
];

#[derive(Clone, Copy)]
struct BaselineColumnResidual {
    column: &'static str,
    fail_count: u64,
    hillslope_fail_count: u64,
    max_abs_diff: f64,
}

const HPARITY01_BASELINE_RESIDUALS: [BaselineColumnResidual; 12] = [
    BaselineColumnResidual {
        column: "Dp",
        fail_count: 44_447,
        hillslope_fail_count: 39,
        max_abs_diff: 0.24,
    },
    BaselineColumnResidual {
        column: "Ep",
        fail_count: 56_834,
        hillslope_fail_count: 39,
        max_abs_diff: 7.78,
    },
    BaselineColumnResidual {
        column: "Es",
        fail_count: 56_973,
        hillslope_fail_count: 39,
        max_abs_diff: 10.028_918_952_778_206,
    },
    BaselineColumnResidual {
        column: "ProfileDepth",
        fail_count: 56_979,
        hillslope_fail_count: 39,
        max_abs_diff: 200.0,
    },
    BaselineColumnResidual {
        column: "ProfileFCStore",
        fail_count: 56_979,
        hillslope_fail_count: 39,
        max_abs_diff: 216.797_000_000_000_03,
    },
    BaselineColumnResidual {
        column: "ProfilePorosityCap",
        fail_count: 56_979,
        hillslope_fail_count: 39,
        max_abs_diff: 752.384,
    },
    BaselineColumnResidual {
        column: "ProfileWPStore",
        fail_count: 56_979,
        hillslope_fail_count: 39,
        max_abs_diff: 87.180_900_000_000_01,
    },
    BaselineColumnResidual {
        column: "RM",
        fail_count: 20_732,
        hillslope_fail_count: 39,
        max_abs_diff: 45.739_999_999_999_995,
    },
    BaselineColumnResidual {
        column: "Snow-Water",
        fail_count: 27_358,
        hillslope_fail_count: 39,
        max_abs_diff: 562.47,
    },
    BaselineColumnResidual {
        column: "SoilWaterTotal",
        fail_count: 56_505,
        hillslope_fail_count: 39,
        max_abs_diff: 354.790_448_162_102_05,
    },
    BaselineColumnResidual {
        column: "Total-Soil",
        fail_count: 56_505,
        hillslope_fail_count: 39,
        max_abs_diff: 354.790_448_162_102_05,
    },
    BaselineColumnResidual {
        column: "latqcc",
        fail_count: 56_979,
        hillslope_fail_count: 39,
        max_abs_diff: 205.5085,
    },
];

fn repo_file(path: &str) -> String {
    let repo_root = env!("CARGO_MANIFEST_DIR");
    let full_path = Path::new(repo_root).join(path);
    fs::read_to_string(&full_path)
        .unwrap_or_else(|error| panic!("expected readable file {}: {error}", full_path.display()))
}

#[test]
fn hparity01_baseline_residual_snapshot_covers_expected_column_set() {
    assert_eq!(
        HPARITY01_BASELINE_RESIDUALS.len(),
        HPARITY01_ALWAYS_FAIL_COLUMNS.len(),
        "baseline snapshot must define one residual record per expected always-fail column"
    );

    for expected_column in HPARITY01_ALWAYS_FAIL_COLUMNS {
        let record = HPARITY01_BASELINE_RESIDUALS
            .iter()
            .find(|record| record.column == expected_column)
            .unwrap_or_else(|| panic!("missing baseline residual record for {expected_column}"));
        assert_eq!(
            record.hillslope_fail_count, 39,
            "{expected_column} should fail on all 39 hillslopes in baseline snapshot"
        );
        assert!(
            record.fail_count > 0,
            "{expected_column} baseline fail_count must remain > 0 pre-closure"
        );
        assert!(
            record.max_abs_diff > 0.0,
            "{expected_column} baseline max_abs_diff must remain > 0 pre-closure"
        );
    }
}

#[test]
fn hparity01_contract_authority_sections_exist() {
    let watbal = repo_file("docs/specifications/science-contracts/contracts/SC-WATBAL-001.md");
    let perc = repo_file("docs/specifications/science-contracts/contracts/SC-PERC-001.md");
    let climate = repo_file("docs/specifications/science-contracts/contracts/SC-CLIMATE-001.md");
    let package = repo_file(
        "docs/work-packages/20260529-hparity01-hillslope-wat-always-fail-gap-mapping-001/package.md",
    );

    assert!(
        watbal.contains("### HPARITY01 Always-Fail Column Lineage Register"),
        "SC-WATBAL-001 must include HPARITY01 lineage register section"
    );
    assert!(
        watbal.contains("Total-Soil Water") && watbal.contains("SoilWaterTotal"),
        "SC-WATBAL-001 must preserve explicit alias continuity for Total-Soil family"
    );
    assert!(
        perc.contains("distinct from") && perc.contains("climate time-to-peak descriptor `Dp`"),
        "SC-PERC-001 must explicitly disambiguate WB13 Dp from climate Dp"
    );
    assert!(
        climate.contains("must not be conflated with WB13") && climate.contains("deep percolation"),
        "SC-CLIMATE-001 must explicitly disambiguate climate Dp from WB13 Dp"
    );
    assert!(
        package.contains("MEASURE-HP01-001")
            && package.contains("MEASURE-HP01-004")
            && package.contains("Mandatory Contract-First Sequence"),
        "HPARITY01 package must encode closure measures and contract-first sequencing"
    );
}

#[test]
#[ignore = "HPARITY01 scaffold: expected to fail until HPARITY02-HPARITY05 production closure packages land and rerun evidence is refreshed"]
fn hparity01_closure_target_requires_zero_fail_counts() {
    for record in HPARITY01_BASELINE_RESIDUALS {
        assert_eq!(
            record.hillslope_fail_count, 0,
            "post-closure target unmet for {}: hillslope_fail_count should be zero",
            record.column
        );
        assert_eq!(
            record.fail_count, 0,
            "post-closure target unmet for {}: fail_count should be zero",
            record.column
        );
        assert!(
            record.max_abs_diff.abs() <= f64::EPSILON,
            "post-closure target unmet for {}: max_abs_diff should be zero",
            record.column
        );
    }
}
