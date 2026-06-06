use std::fs;
use std::process::Command;

use serde_json::Value;

const PACKAGE_DIR: &str =
    "docs/work-packages/20260605-hphys0310-prior-day-snow-carry-divergence-closure-001";
const LEDGER_PATH: &str = "docs/work-packages/20260605-hphys0310-prior-day-snow-carry-divergence-closure-001/artifacts/prior-day-snow-carry-divergence-ledger.json";

fn read(path: &str) -> String {
    fs::read_to_string(path).unwrap_or_else(|err| panic!("failed to read {path}: {err}"))
}

#[test]
fn hphys0310_contract_authority_is_registered() {
    let snowfreeze = read("docs/specifications/science-contracts/contracts/SC-SNOWFREEZE-001.md");
    let watbal = read("docs/specifications/science-contracts/contracts/SC-WATBAL-001.md");

    assert!(
        snowfreeze.contains("contract_version:")
            && snowfreeze.contains("INV-SNOWFREEZE-035")
            && snowfreeze.contains("first material paired snowpack divergence")
            && snowfreeze.contains("corrected negative-melt state-loss indicators"),
        "SC-SNOWFREEZE-001 must register HPHYS0310 prior-day carry divergence authority"
    );
    assert!(
        watbal.contains("contract_version:")
            && watbal.contains("INV-WATBAL-083")
            && watbal.contains("episode-level snow carry divergence ledger")
            && watbal.contains("downstream compensation invalid"),
        "SC-WATBAL-001 must register HPHYS0310 water-balance authority"
    );
}

#[test]
fn hphys0310_package_is_autonomous_and_compensation_prohibited() {
    let package = read(&format!("{PACKAGE_DIR}/package.md"));
    let prompt = read(&format!(
        "{PACKAGE_DIR}/prompts/active/20260605-hphys0310-prior-day-snow-carry-divergence-closure-001_kickoff_agent_prompt.md"
    ));

    for token in [
        "Contract-First Sequence",
        "first prior-day/day-start snowpack carry-state divergence",
        "No production Rust kernel edits",
        "WB13/WB17/WB18/WB19/WB12 compensation",
        "`snodpt`/`densgt`",
    ] {
        assert!(
            package.contains(token),
            "package must contain required HPHYS0310 token: {token}"
        );
    }
    assert!(
        prompt.contains("Execution mode: package-end-to-end")
            && prompt.contains("Autonomy:")
            && prompt.contains("no production edits before source-line proof"),
        "kickoff prompt must require end-to-end autonomous contract-first execution"
    );
}

#[test]
fn hphys0310_runner_uses_prior_ledgers_and_fails_closed() {
    let runner = read(&format!(
        "{PACKAGE_DIR}/artifacts/hphys0310_prior_day_snow_carry_divergence.py"
    ));

    for token in [
        "HPHYS0309_LEDGER",
        "baseline-observe-identity.json",
        "openwepp-trace-field-audit.json",
        "H305_S_OUT",
        "H305_M_POST",
        "snow_runtime_depth_before_m",
        "snow_hourly_depth_after_m",
        "initial-carry-state-projection-hold",
        "production_edit_authorized",
        "FileNotFoundError",
        "winter.for:434-453",
        "sum_record_field",
        "validate_required_paired_hourly_evidence",
        "PairedEvidenceError",
        "--self-test-missing-paired-evidence",
        "baseline_post_hrmlt_observed_hours",
    ] {
        assert!(
            runner.contains(token),
            "runner must include required HPHYS0310 token: {token}"
        );
    }
    assert!(
        !runner.contains("unwrap_or")
            && !runner.contains("or 0.0")
            && !runner.contains(".get(\"post_hrmlt_m\", 0.0)")
            && !runner.contains(".get(\"post_hrrain_m\", 0.0)"),
        "runner must not encode obvious silent zero-fill patterns"
    );
}

#[test]
fn hphys0310_runner_negative_fixture_fails_closed_on_missing_pair() {
    let output = Command::new("python")
        .arg(format!(
            "{PACKAGE_DIR}/artifacts/hphys0310_prior_day_snow_carry_divergence.py"
        ))
        .arg("--self-test-missing-paired-evidence")
        .output()
        .expect("failed to run HPHYS0310 missing-paired-evidence self-test");

    assert!(
        !output.status.success(),
        "negative paired-evidence fixture must fail closed"
    );
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(
        stderr.contains("missing paired hourly evidence fail-closed"),
        "negative fixture stderr must explain missing paired evidence, got: {stderr}"
    );
}

#[test]
fn hphys0310_source_lineage_cites_carry_and_runtime_surfaces() {
    let package = read(&format!("{PACKAGE_DIR}/package.md"));
    if package.contains("Status: queued") {
        return;
    }

    let lineage = read(&format!(
        "{PACKAGE_DIR}/artifacts/prior-day-snow-carry-divergence-source-lineage.md"
    ));

    for token in [
        "/workdir/wepp-forest_260430_baseline/src/snowd.for:50-53",
        "/workdir/wepp-forest_260430_baseline/src/snowd.for:303-312",
        "wepp_260430_negmeltfix_comparator_47ac4c32faee",
        "src/winter.for:434-453",
        "fixed-comparator-source-delta.patch",
        "03_kernel_support_00_support_helpers.rs:3879-4105",
        "03_kernel_support_00_support_helpers.rs:4111-4177",
        "03_kernel_support_00_support_helpers.rs:4231-4277",
    ] {
        assert!(
            lineage.contains(token),
            "source-lineage artifact must cite: {token}"
        );
    }
}

#[test]
fn hphys0310_executed_ledger_is_complete_and_hold_gated() {
    let package = read(&format!("{PACKAGE_DIR}/package.md"));
    if package.contains("Status: queued") {
        return;
    }

    let ledger_text = read(LEDGER_PATH);
    let rows: Vec<Value> =
        serde_json::from_str(&ledger_text).expect("HPHYS0310 ledger must be valid JSON array");

    assert_eq!(
        rows.len(),
        7,
        "executed HPHYS0310 ledger must cover seven affected hillslope/window/year groups"
    );

    let mut represented_rows = 0;
    let mut initial_carry_state_projection = 0;
    let mut density_settling_carry_state = 0;
    for row in rows {
        represented_rows += row
            .get("affected_hphys0309_rows")
            .and_then(Value::as_u64)
            .expect("row must carry affected HPHYS0309 row count");
        assert_eq!(
            row.get("production_edit_authorized")
                .and_then(Value::as_bool),
            Some(false),
            "HPHYS0310 diagnostics must not authorize production edits"
        );
        assert!(
            row.get("first_material_divergence")
                .and_then(Value::as_object)
                .and_then(|first| first.get("depth_delta_openwepp_minus_baseline_m"))
                .and_then(Value::as_f64)
                .is_some_and(|delta| delta.abs() > 0.0005),
            "row must preserve a material first paired depth divergence"
        );
        assert!(
            row.get("baseline_episode")
                .and_then(Value::as_object)
                .and_then(|episode| episode.get("baseline_observed_hour_count"))
                .and_then(Value::as_u64)
                .is_some_and(|count| count > 0),
            "row must include baseline episode evidence"
        );
        assert!(
            row.get("baseline_episode")
                .and_then(Value::as_object)
                .and_then(|episode| episode.get("baseline_post_hrmlt_observed_hours"))
                .and_then(Value::as_u64)
                .is_some_and(|count| count > 0),
            "row must include explicit baseline post-melt observed-hour evidence"
        );
        assert!(
            row.get("openwepp_episode")
                .and_then(Value::as_object)
                .and_then(|episode| episode.get("openwepp_observed_day_count"))
                .and_then(Value::as_u64)
                .is_some_and(|count| count > 0),
            "row must include openWEPP episode evidence"
        );
        match row.get("route").and_then(Value::as_str) {
            Some("initial-carry-state-projection-hold") => initial_carry_state_projection += 1,
            Some("density-settling-carry-state-hold") => density_settling_carry_state += 1,
            other => panic!("unexpected HPHYS0310 route: {other:?}"),
        }
    }

    assert_eq!(represented_rows, 58);
    assert_eq!(initial_carry_state_projection, 6);
    assert_eq!(density_settling_carry_state, 1);
}
