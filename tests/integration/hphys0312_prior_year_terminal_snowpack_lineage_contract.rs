use std::{fs::read_to_string, path::Path, process::Command};

use serde_json::Value;

const PACKAGE_DIR: &str =
    "docs/work-packages/20260605-hphys0312-prior-year-terminal-snowpack-lineage-closure-001";
const LEDGER_PATH: &str = "docs/work-packages/20260605-hphys0312-prior-year-terminal-snowpack-lineage-closure-001/artifacts/prior-year-terminal-snowpack-lineage-ledger.json";
const SOURCE_LINEAGE_PATH: &str = "docs/work-packages/20260605-hphys0312-prior-year-terminal-snowpack-lineage-closure-001/artifacts/prior-year-terminal-snowpack-lineage-source-lineage.md";

fn read(path: &str) -> String {
    read_to_string(path).unwrap_or_else(|error| panic!("failed to read {path}: {error}"))
}

fn python_interpreter() -> &'static str {
    let path = ".venv/bin/python";
    assert!(
        Path::new(path).is_file(),
        "required repo-local Python interpreter missing: {path}"
    );
    path
}

#[test]
fn hphys0312_contract_authority_is_registered() {
    let snowfreeze = read("docs/specifications/science-contracts/contracts/SC-SNOWFREEZE-001.md");
    assert!(
        snowfreeze.contains("INV-SNOWFREEZE-037")
            && snowfreeze.contains("prior calendar year")
            && snowfreeze.contains("year-start-inherited-state-hold")
            && snowfreeze.contains("settling-depth-update-hold"),
        "SC-SNOWFREEZE must define HPHYS0312 prior-year terminal lineage authority"
    );

    let watbal = read("docs/specifications/science-contracts/contracts/SC-WATBAL-001.md");
    assert!(
        watbal.contains("INV-WATBAL-085")
            && watbal.contains("prior-calendar-year lineage ledger")
            && watbal.contains("downstream compensation invalid"),
        "SC-WATBAL must define HPHYS0312 water-balance gate"
    );
}

#[test]
fn hphys0312_package_is_autonomous_and_continuation_scoped() {
    let package = read(&format!("{PACKAGE_DIR}/package.md"));
    let prompt = read(&format!(
        "{PACKAGE_DIR}/prompts/active/20260605-hphys0312-prior-year-terminal-snowpack-lineage-closure-001_kickoff_agent_prompt.md"
    ));
    for token in [
        "HPHYS0311 required continuation",
        "prior-year-terminal-state-hold",
        "No production Rust kernel edits unless source-line",
        "Contract-First Sequence",
        "Autonomy: execute package phases end-to-end",
    ] {
        assert!(
            package.contains(token) || prompt.contains(token),
            "HPHYS0312 package/prompt missing token {token}"
        );
    }
}

#[test]
fn hphys0312_runner_fails_closed_and_cites_required_sources() {
    let runner = read(&format!(
        "{PACKAGE_DIR}/artifacts/hphys0312_prior_year_terminal_snowpack_lineage.py"
    ));
    for token in [
        "SourceLineEvidenceError",
        "PairedEvidenceError",
        "--self-test-missing-source-line",
        "require_source_lineage",
        "MATERIAL_DEPTH_TOL_M = 0.0005",
        "MATERIAL_DENSITY_TOL_KG_M3 = 0.5",
        "year-start-inherited-state-hold",
        "settling-depth-update-hold",
        "snowd.for:61-65",
        "snowd.for:122-139",
        "snowd.for:145-173",
        "snowd.for:181-198",
        "snowd.for:240-278",
        "snowd.for:310-312",
        "03_kernel_support_00_support_helpers.rs:3872-4227",
    ] {
        assert!(
            runner.contains(token),
            "runner missing required token {token}"
        );
    }
    for forbidden in [".get(field, 0.0)", ".get(\"depth_after_m\", 0.0)", "or 0.0"] {
        assert!(
            !runner.contains(forbidden),
            "runner must not silently default missing evidence with {forbidden}"
        );
    }
}

#[test]
fn hphys0312_runner_negative_fixture_fails_closed_on_missing_source_line() {
    let status = Command::new(python_interpreter())
        .arg(format!(
            "{PACKAGE_DIR}/artifacts/hphys0312_prior_year_terminal_snowpack_lineage.py"
        ))
        .arg("--self-test-missing-source-line")
        .status()
        .expect("failed to run HPHYS0312 missing-source-line self-test");
    assert!(
        !status.success(),
        "missing source-line self-test must fail closed"
    );
}

#[test]
fn hphys0312_source_lineage_artifact_verifies_requirements() {
    let source_lineage = read(SOURCE_LINEAGE_PATH);
    for token in [
        "Verified Source Requirements",
        "snowd.for:61-65",
        "snowd.for:122-139",
        "snowd.for:145-173",
        "snowd.for:181-198",
        "snowd.for:215-246",
        "snowd.for:240-278",
        "snowd.for:310-312",
        "03_kernel_support_00_support_helpers.rs:3872-3920",
        "03_kernel_support_00_support_helpers.rs:3925-4057",
        "03_kernel_support_00_support_helpers.rs:4075-4109",
        "03_kernel_support_00_support_helpers.rs:4218-4227",
    ] {
        assert!(
            source_lineage.contains(token),
            "source-lineage artifact missing requirement {token}"
        );
    }
}

#[test]
fn hphys0312_executed_ledger_is_complete_and_hold_gated() {
    let ledger_text = read(LEDGER_PATH);
    let ledger: Value = serde_json::from_str(&ledger_text).expect("ledger must be valid JSON");
    let rows = ledger.as_array().expect("ledger must be an array");
    assert_eq!(
        rows.len(),
        6,
        "HPHYS0312 must cover all six HPHYS0311 inherited terminal groups"
    );

    let mut represented = 0_i64;
    let mut settling_depth_update = 0;
    let mut year_start_inherited = 0;
    let mut authorized = 0;
    for row in rows {
        represented += row["affected_hphys0309_rows"]
            .as_i64()
            .expect("affected row count must be integer");
        if row["production_edit_authorized"].as_bool() == Some(true) {
            authorized += 1;
        }
        assert_eq!(
            row["source_hphys0311_route"].as_str(),
            Some("prior-year-terminal-state-hold"),
            "HPHYS0312 rows must derive only from HPHYS0311 inherited terminal holds"
        );
        let first = row["first_material_divergence"]
            .as_object()
            .expect("row must publish first material divergence");
        let depth_delta = first
            .get("depth_delta_openwepp_minus_baseline_m")
            .and_then(Value::as_f64)
            .expect("first divergence must publish depth delta");
        let density_delta = first
            .get("density_delta_openwepp_minus_baseline_kg_m3")
            .and_then(Value::as_f64)
            .expect("first divergence must publish density delta");
        assert!(
            depth_delta.abs() > 0.0005 || density_delta.abs() > 0.5,
            "first divergence must be material by HPHYS0312 thresholds"
        );
        let terminal_continuity = row["terminal_continuity"]
            .as_object()
            .expect("row must publish terminal continuity checks");
        assert_eq!(
            terminal_continuity
                .get("depth_delta_matches_hphys0311")
                .and_then(Value::as_bool),
            Some(true),
            "terminal depth delta must match HPHYS0311"
        );
        assert_eq!(
            terminal_continuity
                .get("density_delta_matches_hphys0311")
                .and_then(Value::as_bool),
            Some(true),
            "terminal density delta must match HPHYS0311"
        );
        match row["route"].as_str() {
            Some("settling-depth-update-hold") => {
                settling_depth_update += 1;
                assert!(
                    row["last_within_tolerance_state_before_first_divergence"].is_object(),
                    "within-year settling holds must publish previous within-tolerance state"
                );
            }
            Some("year-start-inherited-state-hold") => {
                year_start_inherited += 1;
                assert_eq!(
                    first.get("julian").and_then(Value::as_i64),
                    Some(1),
                    "year-start inherited holds must start at day 1"
                );
                assert_eq!(
                    first.get("hour").and_then(Value::as_i64),
                    Some(1),
                    "year-start inherited holds must start at hour 1"
                );
                assert!(
                    row["last_within_tolerance_state_before_first_divergence"].is_null(),
                    "year-start inherited holds must not invent a prior in-year state"
                );
            }
            other => panic!("unexpected HPHYS0312 route {other:?}"),
        }
        assert!(
            row["prohibited_compensation_note"]
                .as_str()
                .is_some_and(|note| note.contains("downstream compensation")),
            "each row must prohibit downstream compensation"
        );
    }

    assert_eq!(represented, 57);
    assert_eq!(settling_depth_update, 3);
    assert_eq!(year_start_inherited, 3);
    assert_eq!(authorized, 0);
}
