use std::env;
use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;
use std::time::{SystemTime, UNIX_EPOCH};

#[cfg(unix)]
use std::os::unix::fs::PermissionsExt;

const OWCMP_CLI: &str = include_str!("../../tools/owcmp/owcmp");
const OWCMP_SEMANTIC: &str = include_str!("../../tools/owcmp/semantic_wat.py");
const OWCMP_PL14S: &str = include_str!("../../tools/owcmp/pl14s_suite.py");
const OWCMP_SPEC: &str = include_str!("../../tools/owcmp/specification.md");
const OWCMP_README: &str = include_str!("../../tools/owcmp/README.md");

fn repo_root() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).to_path_buf()
}

fn fixture_temp_dir(prefix: &str) -> PathBuf {
    let stamp = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("system clock should be after unix epoch")
        .as_nanos();
    env::temp_dir().join(format!("{prefix}_{stamp}"))
}

fn dat_row(ofe: i32, julian: i32, year: i32, seed: i32) -> String {
    let tail = (0_i32..17_i32)
        .map(|offset| (seed + offset).to_string())
        .collect::<Vec<_>>()
        .join(" ");
    format!("{ofe} {julian} {year} {tail}")
}

fn contains_all(haystack: &str, needles: &[&str]) -> bool {
    needles.iter().all(|needle| haystack.contains(needle))
}

#[cfg(unix)]
fn write_executable(path: &Path, payload: &str) {
    fs::write(path, payload).expect("executable fixture should be writable");
    let mut permissions = fs::metadata(path)
        .expect("executable fixture metadata should be readable")
        .permissions();
    permissions.set_mode(0o755);
    fs::set_permissions(path, permissions).expect("executable fixture should be chmodded");
}

#[test]
fn owcmp_declares_pl14s_contract_markers_and_deferred_observe_boundary() {
    assert!(contains_all(
        OWCMP_CLI,
        &[
            "wat semantic",
            "pl14s run",
            "summarize",
            "manifest run",
            "owcmp observe normalize is deferred",
        ],
    ));
    assert!(contains_all(
        OWCMP_SEMANTIC,
        &[
            "REPORT_SCHEMA_VERSION = \"pl14s-semantic-wat-v2\"",
            "duplicate row key",
            "\"Total-Soil Water\": \"Total-Soil\"",
            "candidate_column_alias_sources",
            "row_key_fields",
            "--candidate-partition-value",
            "--candidate-partition-column",
            "--candidate-year-offset",
        ],
    ));
    assert!(contains_all(
        OWCMP_PL14S,
        &[
            "\"suite_schema_version\": \"pl14s-legacy-suite-v2\"",
            "tools/owcmp/configs/pl14s_wat_tolerances.json",
            "semantic_wat.py",
            "--baseline-year-policy",
            "--expected-common-row-count",
            "\"full_span_policy_ready\"",
            "\"conversion_source_row_consistency_ready\"",
            "/workdir/wepp-forest_260430_baseline/tools/compare_wepp_raw_outputs.py",
            "--abs-tol",
            "--rel-tol",
        ],
    ));
    assert!(contains_all(
        OWCMP_SPEC,
        &[
            "Do not implement `owcmp observe normalize` in this package.",
            "Focused `owcmp` contract tests pass",
            "`pl14s_tier_a_candidate_emission_and_replay_contract` remains intact",
        ],
    ));
    assert!(OWCMP_README.contains("tools/owcmp/owcmp wat semantic"));
}

#[test]
fn owcmp_wat_semantic_rejects_duplicate_row_keys() {
    let temp_dir = fixture_temp_dir("owcmp_duplicate_keys");
    fs::create_dir_all(&temp_dir).expect("temporary directory should be creatable");

    let baseline_wat = temp_dir.join("baseline.wat.dat");
    let candidate_wat = temp_dir.join("candidate.wat.dat");
    let report_path = temp_dir.join("semantic_report.json");

    let baseline_payload = format!("{}\n{}\n", dat_row(1, 1, 2008, 10), dat_row(1, 1, 2008, 20));
    let candidate_payload = format!("{}\n", dat_row(1, 1, 2008, 30));

    fs::write(&baseline_wat, baseline_payload).expect("baseline fixture should be writable");
    fs::write(&candidate_wat, candidate_payload).expect("candidate fixture should be writable");

    let output = Command::new("python3")
        .current_dir(repo_root())
        .arg(repo_root().join("tools").join("owcmp").join("owcmp"))
        .arg("wat")
        .arg("semantic")
        .arg("--baseline-wat")
        .arg(&baseline_wat)
        .arg("--candidate-wat")
        .arg(&candidate_wat)
        .arg("--report-json")
        .arg(&report_path)
        .output()
        .expect("python3 should run owcmp");

    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(
        !output.status.success(),
        "duplicate row keys must hard-fail owcmp"
    );
    assert!(
        stderr.contains("duplicate row key"),
        "error stream should mention duplicate row key; stderr={stderr}"
    );

    let _ = fs::remove_dir_all(&temp_dir);
}

#[test]
fn owcmp_summarize_emits_compact_json_and_markdown() {
    let temp_dir = fixture_temp_dir("owcmp_summary");
    fs::create_dir_all(&temp_dir).expect("temporary directory should be creatable");

    let baseline_wat = temp_dir.join("baseline.wat.dat");
    let candidate_wat = temp_dir.join("candidate.wat.dat");
    let report_path = temp_dir.join("semantic_report.json");
    let summary_root = temp_dir.join("summary");

    let payload = format!("{}\n{}\n", dat_row(1, 1, 2008, 10), dat_row(1, 2, 2008, 20));
    fs::write(&baseline_wat, &payload).expect("baseline fixture should be writable");
    fs::write(&candidate_wat, &payload).expect("candidate fixture should be writable");

    let semantic_output = Command::new("python3")
        .current_dir(repo_root())
        .arg(repo_root().join("tools").join("owcmp").join("owcmp"))
        .arg("wat")
        .arg("semantic")
        .arg("--baseline-wat")
        .arg(&baseline_wat)
        .arg("--candidate-wat")
        .arg(&candidate_wat)
        .arg("--report-json")
        .arg(&report_path)
        .output()
        .expect("python3 should run owcmp semantic comparator");
    assert!(
        semantic_output.status.success(),
        "semantic comparator should pass; stderr={}",
        String::from_utf8_lossy(&semantic_output.stderr)
    );

    let summary_output = Command::new("python3")
        .current_dir(repo_root())
        .arg(repo_root().join("tools").join("owcmp").join("owcmp"))
        .arg("summarize")
        .arg("--input")
        .arg(&report_path)
        .arg("--output-root")
        .arg(&summary_root)
        .output()
        .expect("python3 should run owcmp summarize");
    assert!(
        summary_output.status.success(),
        "summary should pass; stderr={}",
        String::from_utf8_lossy(&summary_output.stderr)
    );

    let summary_json =
        fs::read_to_string(summary_root.join("summary.json")).expect("summary json exists");
    assert!(summary_json.contains("\"source_type\": \"semantic_report\""));
    assert!(summary_json.contains("\"pass_count\": \"1/1\""));
    assert!(summary_json.contains("\"verdict\": \"PASS\""));

    let summary_md =
        fs::read_to_string(summary_root.join("summary.md")).expect("summary markdown exists");
    assert!(summary_md.contains("# owcmp Summary"));
    assert!(summary_md.contains("Verdict: `PASS`"));

    let _ = fs::remove_dir_all(&temp_dir);
}

#[test]
fn owcmp_summarize_reports_policy_skipped_commands_as_skipped() {
    let temp_dir = fixture_temp_dir("owcmp_provenance_summary");
    fs::create_dir_all(&temp_dir).expect("temporary directory should be creatable");

    let provenance_path = temp_dir.join("pl14s_provenance_manifest.json");
    let summary_root = temp_dir.join("summary");
    let provenance_payload = r#"{
  "suite_schema_version": "pl14s-legacy-suite-v2",
  "strict_lane_policy": {
    "strict_equivalent_blockers": [],
    "full_span_policy_blockers": [],
    "conversion_source_row_consistency_blockers": []
  },
  "executions": {
    "baseline_replay": {"returncode": 0, "cmd": ["baseline"]},
    "strict_compare": {"skipped": true, "reason": "strict raw comparator requires .dat input surfaces"},
    "semantic_compare": {"returncode": 0, "cmd": ["semantic"]}
  },
  "outputs": {
    "semantic_summary": {
      "semantic_pass": true,
      "common_row_count": 2,
      "only_baseline_count": 0,
      "only_candidate_count": 0
    }
  }
}"#;
    fs::write(&provenance_path, provenance_payload).expect("provenance fixture should be writable");

    let summary_output = Command::new("python3")
        .current_dir(repo_root())
        .arg(repo_root().join("tools").join("owcmp").join("owcmp"))
        .arg("summarize")
        .arg("--input")
        .arg(&provenance_path)
        .arg("--output-root")
        .arg(&summary_root)
        .output()
        .expect("python3 should run owcmp summarize");
    assert!(
        summary_output.status.success(),
        "provenance summary should pass; stderr={}",
        String::from_utf8_lossy(&summary_output.stderr)
    );

    let summary_json =
        fs::read_to_string(summary_root.join("summary.json")).expect("summary json exists");
    assert!(summary_json.contains("\"verdict\": \"PASS\""));
    assert!(summary_json.contains("\"name\": \"strict_compare\""));
    assert!(summary_json.contains("\"status\": \"SKIPPED\""));

    let _ = fs::remove_dir_all(&temp_dir);
}

#[test]
fn owcmp_summarize_reports_failed_commands_as_failed_verdict() {
    let temp_dir = fixture_temp_dir("owcmp_failed_command_summary");
    fs::create_dir_all(&temp_dir).expect("temporary directory should be creatable");

    let provenance_path = temp_dir.join("pl14s_provenance_manifest.json");
    let summary_root = temp_dir.join("summary");
    let provenance_payload = r#"{
  "suite_schema_version": "pl14s-legacy-suite-v2",
  "strict_lane_policy": {
    "strict_equivalent_blockers": [],
    "full_span_policy_blockers": [],
    "conversion_source_row_consistency_blockers": []
  },
  "executions": {
    "baseline_replay": {"returncode": 1, "cmd": ["baseline"]},
    "strict_compare": {"skipped": true, "reason": "strict raw comparator requires .dat input surfaces"},
    "semantic_compare": {"returncode": 0, "cmd": ["semantic"]}
  },
  "outputs": {
    "semantic_summary": {
      "semantic_pass": true,
      "common_row_count": 2,
      "only_baseline_count": 0,
      "only_candidate_count": 0
    }
  }
}"#;
    fs::write(&provenance_path, provenance_payload).expect("provenance fixture should be writable");

    let summary_output = Command::new("python3")
        .current_dir(repo_root())
        .arg(repo_root().join("tools").join("owcmp").join("owcmp"))
        .arg("summarize")
        .arg("--input")
        .arg(&provenance_path)
        .arg("--output-root")
        .arg(&summary_root)
        .output()
        .expect("python3 should run owcmp summarize");
    assert!(
        summary_output.status.success(),
        "failed-command summary should still emit an artifact; stderr={}",
        String::from_utf8_lossy(&summary_output.stderr)
    );

    let summary_json =
        fs::read_to_string(summary_root.join("summary.json")).expect("summary json exists");
    assert!(summary_json.contains("\"verdict\": \"FAIL\""));
    assert!(summary_json.contains("\"name\": \"baseline_replay\""));
    assert!(summary_json.contains("\"status\": \"FAIL\""));
    assert!(summary_json.contains("command failed: baseline_replay"));

    let _ = fs::remove_dir_all(&temp_dir);
}

#[cfg(unix)]
#[test]
fn owcmp_pl14s_run_emits_provenance_with_strict_and_semantic_lanes() {
    let temp_dir = fixture_temp_dir("owcmp_pl14s_run");
    let baseline_run_dir = temp_dir.join("baseline_fixture");
    let baseline_runs_dir = baseline_run_dir.join("runs");
    let candidate_wat = temp_dir.join("candidate.wat.dat");
    let baseline_binary = temp_dir.join("fake_baseline.sh");
    let strict_comparator = temp_dir.join("fake_strict_comparator.py");
    let output_root = temp_dir.join("suite_output");
    fs::create_dir_all(&baseline_runs_dir).expect("baseline runs directory should be creatable");

    let wat_payload = format!("{}\n{}\n", dat_row(1, 1, 2008, 10), dat_row(1, 2, 2008, 20));
    fs::write(baseline_runs_dir.join("p1.run"), "fake run input\n")
        .expect("baseline run file should be writable");
    fs::write(&candidate_wat, &wat_payload).expect("candidate fixture should be writable");
    write_executable(
        &baseline_binary,
        &format!(
            "#!/usr/bin/env bash\nset -euo pipefail\nmkdir -p ../output\ncat > ../output/H1.wat.dat <<'EOF'\n{wat_payload}EOF\n"
        ),
    );
    fs::write(
        &strict_comparator,
        r#"import json
import sys
out = sys.argv[sys.argv.index("--json-out") + 1]
with open(out, "w", encoding="utf-8") as handle:
    json.dump({"strict_pass": True, "argv": sys.argv[1:]}, handle)
    handle.write("\n")
"#,
    )
    .expect("strict comparator fixture should be writable");

    let suite_output = Command::new("python3")
        .current_dir(repo_root())
        .arg(repo_root().join("tools").join("owcmp").join("owcmp"))
        .arg("pl14s")
        .arg("run")
        .arg("--baseline-run-dir")
        .arg(&baseline_run_dir)
        .arg("--baseline-binary")
        .arg(&baseline_binary)
        .arg("--baseline-run-file")
        .arg("p1.run")
        .arg("--candidate-wat")
        .arg(&candidate_wat)
        .arg("--candidate-surface-source-class")
        .arg("native-runtime-dat")
        .arg("--legacy-comparator-tool")
        .arg(&strict_comparator)
        .arg("--output-root")
        .arg(&output_root)
        .output()
        .expect("python3 should run owcmp pl14s run");
    assert!(
        suite_output.status.success(),
        "pl14s run should pass; stderr={}",
        String::from_utf8_lossy(&suite_output.stderr)
    );

    let provenance_path = output_root
        .join("investigation")
        .join("pl14s_provenance_manifest.json");
    let provenance =
        fs::read_to_string(&provenance_path).expect("provenance manifest should exist");
    assert!(provenance.contains("\"suite_schema_version\": \"pl14s-legacy-suite-v2\""));
    assert!(provenance.contains("\"mode\": \"strict-required\""));
    assert!(provenance.contains("\"candidate_surface_source_class\": \"native-runtime-dat\""));
    assert!(provenance.contains("\"skipped\": false"));
    assert!(provenance.contains("\"semantic_pass\": true"));
    assert!(provenance.contains("tools/owcmp/configs/pl14s_wat_tolerances.json"));
    assert!(
        output_root
            .join("investigation")
            .join("h5_wat_strict_comparator.json")
            .is_file()
    );

    let _ = fs::remove_dir_all(&temp_dir);
}

#[test]
fn owcmp_observe_normalize_is_deferred() {
    let output = Command::new("python3")
        .current_dir(repo_root())
        .arg(repo_root().join("tools").join("owcmp").join("owcmp"))
        .arg("observe")
        .arg("normalize")
        .output()
        .expect("python3 should run owcmp");

    assert!(
        !output.status.success(),
        "observe normalize must remain deferred in OWCMP01"
    );
    assert!(
        String::from_utf8_lossy(&output.stderr).contains("deferred"),
        "deferred observe command should explain boundary"
    );
}
