use std::{fs::read_to_string, path::Path, process::Command};

use serde_json::Value;

const PACKAGE_DIR: &str =
    "docs/work-packages/20260605-hphys0311-snow-carry-source-line-parity-closure-001";
const LEDGER_PATH: &str = "docs/work-packages/20260605-hphys0311-snow-carry-source-line-parity-closure-001/artifacts/snow-carry-source-line-parity-ledger.json";
const SOURCE_LINEAGE_PATH: &str = "docs/work-packages/20260605-hphys0311-snow-carry-source-line-parity-closure-001/artifacts/snow-carry-source-line-parity-source-lineage.md";

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
fn hphys0311_contract_authority_is_registered() {
    let snowfreeze = read("docs/specifications/science-contracts/contracts/SC-SNOWFREEZE-001.md");
    assert!(
        snowfreeze.contains("INV-SNOWFREEZE-036")
            && snowfreeze.contains("prior-year terminal")
            && snowfreeze.contains("fixed-observe precision"),
        "SC-SNOWFREEZE must define HPHYS0311 source-line carry-state authority"
    );

    let watbal = read("docs/specifications/science-contracts/contracts/SC-WATBAL-001.md");
    assert!(
        watbal.contains("INV-WATBAL-084")
            && watbal.contains("source-line parity ledger")
            && watbal.contains("downstream compensation invalid"),
        "SC-WATBAL must define HPHYS0311 water-balance gate"
    );
}

#[test]
fn hphys0311_package_is_autonomous_and_source_line_scoped() {
    let package = read(&format!("{PACKAGE_DIR}/package.md"));
    let prompt = read(&format!(
        "{PACKAGE_DIR}/prompts/active/20260605-hphys0311-snow-carry-source-line-parity-closure-001_kickoff_agent_prompt.md"
    ));
    for token in [
        "source-line carry-state parity",
        "winter.for:193",
        "snowd.for",
        "No production Rust kernel edits unless source-line",
        "Contract-First Sequence",
    ] {
        assert!(
            package.contains(token) || prompt.contains(token),
            "HPHYS0311 package/prompt missing token {token}"
        );
    }
    assert!(
        prompt.contains("Execution mode: package-end-to-end")
            && prompt.contains("Autonomy: execute package phases end-to-end"),
        "kickoff prompt must encode autonomous end-to-end execution"
    );
}

#[test]
fn hphys0311_runner_fails_closed_and_cites_required_sources() {
    let runner = read(&format!(
        "{PACKAGE_DIR}/artifacts/hphys0311_snow_carry_source_line_parity.py"
    ));
    for token in [
        "SourceLineEvidenceError",
        "--self-test-missing-source-line",
        "require_source_lineage",
        "require_baseline_value",
        "require_trace_row",
        "prior-year-terminal-state-hold",
        "fixed-observe-precision-hold",
        "winter.for:193",
        "snowd.for:50-53",
        "snowd.for:122-139",
        "snowd.for:303-312",
        "infile.for:1361,1466",
        "inidat.for:383",
        "SC-INFILE-MANAGEMENT-001",
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
fn hphys0311_runner_negative_fixture_fails_closed_on_missing_source_line() {
    let status = Command::new(python_interpreter())
        .arg(format!(
            "{PACKAGE_DIR}/artifacts/hphys0311_snow_carry_source_line_parity.py"
        ))
        .arg("--self-test-missing-source-line")
        .status()
        .expect("failed to run HPHYS0311 missing-source-line self-test");
    assert!(
        !status.success(),
        "missing source-line self-test must fail closed"
    );
}

#[test]
fn hphys0311_source_lineage_artifact_verifies_expanded_requirements() {
    let source_lineage = read(SOURCE_LINEAGE_PATH);
    for token in [
        "Verified Source Requirements",
        "winter.for:193",
        "snowd.for:50-53",
        "snowd.for:122-139",
        "snowd.for:303-312",
        "infile.for:1361,1466",
        "inidat.for:383",
        "runtime_inputs/04_snow_frost_irrigation.rs:663-691",
        "03_kernel_support_00_support_helpers.rs:3690-3790",
        "03_kernel_support_00_support_helpers.rs:3872-3912",
        "03_kernel_support_00_support_helpers.rs:4218-4227",
        "03_kernel_support_01_kernel_phases.rs:4216-4235",
        "SC-INFILE-MANAGEMENT-001:201",
    ] {
        assert!(
            source_lineage.contains(token),
            "source-lineage artifact missing expanded requirement {token}"
        );
    }
}

#[test]
fn hphys0311_executed_ledger_is_complete_and_hold_gated() {
    let ledger_text = read(LEDGER_PATH);
    let ledger: Value = serde_json::from_str(&ledger_text).expect("ledger must be valid JSON");
    let rows = ledger.as_array().expect("ledger must be an array");
    assert_eq!(
        rows.len(),
        7,
        "HPHYS0311 must cover all seven HPHYS0310 groups"
    );

    let mut represented = 0_i64;
    let mut prior_year_terminal = 0;
    let mut fixed_observe_precision = 0;
    let mut authorized = 0;
    for row in rows {
        represented += row["affected_hphys0309_rows"]
            .as_i64()
            .expect("affected row count must be integer");
        if row["production_edit_authorized"].as_bool() == Some(true) {
            authorized += 1;
        }
        match row["route"].as_str() {
            Some("prior-year-terminal-state-hold") => {
                prior_year_terminal += 1;
                let checks = row["inheritance_checks"]
                    .as_object()
                    .expect("prior-year route must publish inheritance checks");
                assert_eq!(
                    checks.get("depth_delta_inherited").and_then(Value::as_bool),
                    Some(true),
                    "prior-year route must prove inherited depth delta"
                );
                assert_eq!(
                    checks
                        .get("density_delta_inherited")
                        .and_then(Value::as_bool),
                    Some(true),
                    "prior-year route must prove inherited density delta"
                );
                let depth_tolerance = checks
                    .get("depth_delta_tolerance_m")
                    .and_then(Value::as_f64)
                    .expect("prior-year route must publish depth tolerance");
                let density_tolerance = checks
                    .get("density_delta_tolerance_kg_m3")
                    .and_then(Value::as_f64)
                    .expect("prior-year route must publish density tolerance");
                assert!(
                    depth_tolerance <= 1.0e-12 && density_tolerance <= 1.0e-12,
                    "prior-year route tolerances must remain source-line strict"
                );
            }
            Some("fixed-observe-precision-hold") => {
                fixed_observe_precision += 1;
                let settling = row["settling_state"]
                    .as_object()
                    .expect("fixed-observe route must publish settling-state evidence");
                assert_eq!(
                    settling
                        .get("previous_hour_state_near_identical")
                        .and_then(Value::as_bool),
                    Some(true),
                    "fixed-observe route must prove previous-hour state is near-identical"
                );
                let depth_delta = settling
                    .get("previous_hour_depth_delta_openwepp_minus_baseline_m")
                    .and_then(Value::as_f64)
                    .expect("fixed-observe route must publish previous-hour depth delta");
                let depth_tolerance = settling
                    .get("previous_hour_depth_tolerance_m")
                    .and_then(Value::as_f64)
                    .expect("fixed-observe route must publish previous-hour depth tolerance");
                let density_delta = settling
                    .get("previous_hour_density_delta_openwepp_minus_baseline_kg_m3")
                    .and_then(Value::as_f64)
                    .expect("fixed-observe route must publish previous-hour density delta");
                let density_tolerance = settling
                    .get("previous_hour_density_tolerance_kg_m3")
                    .and_then(Value::as_f64)
                    .expect("fixed-observe route must publish previous-hour density tolerance");
                assert!(
                    depth_delta.abs() <= depth_tolerance
                        && density_delta.abs() <= density_tolerance,
                    "fixed-observe route must be backed by previous-hour threshold evidence"
                );
            }
            other => panic!("unexpected HPHYS0311 route {other:?}"),
        }
        assert!(
            row["prohibited_compensation_note"]
                .as_str()
                .is_some_and(|note| note.contains("downstream compensation")),
            "each row must prohibit downstream compensation"
        );
    }

    assert_eq!(represented, 58);
    assert_eq!(prior_year_terminal, 6);
    assert_eq!(fixed_observe_precision, 1);
    assert_eq!(authorized, 0);
}
