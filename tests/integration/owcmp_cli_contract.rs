use std::env;
use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;
use std::time::{SystemTime, UNIX_EPOCH};

#[cfg(unix)]
use std::os::unix::fs::PermissionsExt;

const OWCMP_CLI: &str = include_str!("../../tools/owcmp/owcmp");
const OWCMP_BATCH_H1_H39: &str = include_str!("../../tools/owcmp/batch_h1_h39.py");
const OWCMP_SEMANTIC: &str = include_str!("../../tools/owcmp/semantic_wat.py");
const OWCMP_PL14S: &str = include_str!("../../tools/owcmp/pl14s_suite.py");
const OWCMP_SUITE_MANIFEST: &str = include_str!("../../tools/owcmp/suite_manifest.py");
const OWCMP_SPEC: &str = include_str!("../../tools/owcmp/specification.md");
const OWCMP_README: &str = include_str!("../../tools/owcmp/README.md");
const OWCMP_SUITES_README: &str = include_str!("../../tools/owcmp/suites/README.md");
const OWCMP_N_IDAHO_SUITE: &str =
    include_str!("../../tools/owcmp/suites/n-idaho-single-ofe-ksflag0.json");
const OWCMP_MINNESOTA_SUITE: &str =
    include_str!("../../tools/owcmp/suites/minnesota-corn-ksflag1.json");
const OWCMP_WA_SUITE: &str = include_str!("../../tools/owcmp/suites/wa-cascades-mofe-ksflag0.json");
const OWCMP_ARTIFACT_RETENTION: &str = include_str!("../../tools/owcmp/artifact-retention.md");
const OWCMP_RUNNER_GUIDANCE: &str =
    include_str!("../../docs/prompt_templates/owcmp-comparator-runner-guidance.md");

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
#[allow(clippy::too_many_lines)]
fn owcmp_declares_pl14s_contract_markers_and_deferred_observe_boundary() {
    assert!(contains_all(
        OWCMP_CLI,
        &[
            "wat semantic",
            "pl14s run",
            "batch h1-h39-semantic",
            "summarize",
            "usage: owcmp manifest <list|show|run>",
            "suite_manifest.list_main",
            "suite_manifest.show_main",
            "_run_manifest",
            "env",
            "suite_manifest",
            "VENV_PYTHON",
            "os.execv",
            "owcmp observe normalize is deferred",
        ],
    ));
    assert!(contains_all(
        OWCMP_BATCH_H1_H39,
        &[
            "h1_h39_semantic_batch",
            "--baseline-dir",
            "--candidate-dir",
            "--baseline-pattern",
            "--candidate-pattern",
            "semantic_pass_count",
            "execution_verdict",
            "command-log.json",
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
            "owcmp-env-v1",
            "owcmp manifest list",
            "owcmp env --manifest",
        ],
    ));
    assert!(OWCMP_README.contains("tools/owcmp/owcmp wat semantic"));
    assert!(OWCMP_README.contains("tools/owcmp/owcmp batch h1-h39-semantic"));
    assert!(OWCMP_README.contains("tools/owcmp/suites/n-idaho-single-ofe-ksflag0.json"));
    assert!(contains_all(
        OWCMP_SUITE_MANIFEST,
        &[
            "SCHEMA_VERSION = \"owcmp-suite-manifest-v1\"",
            "ENV_SCHEMA_VERSION = \"owcmp-env-v1\"",
            "pattern-range",
            "pyarrow",
        ],
    ));
    assert!(contains_all(
        OWCMP_SUITES_README,
        &[
            "tools/owcmp/owcmp manifest list",
            "tools/owcmp/owcmp env --manifest",
            "cohort-inventory",
        ],
    ));
    assert!(contains_all(
        OWCMP_N_IDAHO_SUITE,
        &[
            "\"suite_id\": \"n-idaho-single-ofe-ksflag0\"",
            "\"run_root\": \"/wc1/runs/un/unpalatable-rind\"",
            "\"path_pattern\": \"/wc1/runs/un/unpalatable-rind/wepp/output/H{h}.plot.dat\"",
            "\"end\": 39",
        ],
    ));
    assert!(contains_all(
        OWCMP_MINNESOTA_SUITE,
        &[
            "\"suite_id\": \"minnesota-corn-ksflag1\"",
            "\"run_root\": \"/wc1/runs/al/algebraic-radium\"",
            "\"path_pattern\": \"/wc1/runs/al/algebraic-radium/wepp/output/H{h}.wat.dat\"",
            "\"end\": 43",
        ],
    ));
    assert!(contains_all(
        OWCMP_WA_SUITE,
        &[
            "\"suite_id\": \"wa-cascades-mofe-ksflag0\"",
            "\"run_root\": \"/wc1/runs/ar/arboreal-dendrite\"",
            "\"path_pattern\": \"/wc1/runs/ar/arboreal-dendrite/wepp/output/H{h}.wat.dat\"",
            "\"end\": 36",
        ],
    ));
    assert!(contains_all(
        OWCMP_ARTIFACT_RETENTION,
        &[
            "summary.json",
            "summary.md",
            "command-log.json",
            "Local-Only by Default",
        ],
    ));
    assert!(contains_all(
        OWCMP_RUNNER_GUIDANCE,
        &[
            "comparator_suite_runner",
            "tools/owcmp/owcmp manifest list",
            "tools/owcmp/owcmp env --manifest",
            "Do not paste raw logs",
        ],
    ));
}

#[test]
fn owcmp_manifest_list_discovers_seeded_suites() {
    let output = Command::new("python3")
        .current_dir(repo_root())
        .arg(repo_root().join("tools").join("owcmp").join("owcmp"))
        .arg("manifest")
        .arg("list")
        .arg("--json")
        .output()
        .expect("python3 should run owcmp manifest list");

    assert!(
        output.status.success(),
        "manifest list should pass; stderr={}",
        String::from_utf8_lossy(&output.stderr)
    );
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("\"schema_version\": \"owcmp-suite-list-v1\""));
    assert!(stdout.contains("\"suite_id\": \"n-idaho-single-ofe-ksflag0\""));
    assert!(stdout.contains("\"suite_id\": \"minnesota-corn-ksflag1\""));
    assert!(stdout.contains("\"suite_id\": \"wa-cascades-mofe-ksflag0\""));
}

#[test]
fn owcmp_env_checks_temp_manifest_and_rejects_inventory_run() {
    let temp_dir = fixture_temp_dir("owcmp_manifest_env");
    let run_dir = temp_dir.join("run");
    fs::create_dir_all(&run_dir).expect("run fixture directory should be creatable");
    fs::write(run_dir.join("H1.wat.dat"), "fixture H1\n").expect("H1 fixture should be writable");
    fs::write(run_dir.join("H2.wat.dat"), "fixture H2\n").expect("H2 fixture should be writable");

    let manifest_path = temp_dir.join("suite.json");
    let manifest_payload = r#"{
  "schema_version": "owcmp-suite-manifest-v1",
  "suite_id": "fixture-suite",
  "title": "Fixture Suite",
  "lane": "cohort-inventory",
  "run_root": "run",
  "checks": [
    {"name": "run_root", "kind": "directory", "path": "run", "required": true},
    {"name": "wat_outputs", "kind": "pattern-range", "path_pattern": "run/H{h}.wat.dat", "start": 1, "end": 2, "required": true}
  ]
}"#;
    fs::write(&manifest_path, manifest_payload).expect("manifest fixture should be writable");

    let env_output = Command::new("python3")
        .current_dir(repo_root())
        .arg(repo_root().join("tools").join("owcmp").join("owcmp"))
        .arg("env")
        .arg("--manifest")
        .arg(&manifest_path)
        .arg("--json")
        .output()
        .expect("python3 should run owcmp env");
    assert!(
        env_output.status.success(),
        "env should pass for complete fixture manifest; stderr={}",
        String::from_utf8_lossy(&env_output.stderr)
    );
    let stdout = String::from_utf8_lossy(&env_output.stdout);
    assert!(stdout.contains("\"schema_version\": \"owcmp-env-v1\""));
    assert!(stdout.contains("\"suite_id\": \"fixture-suite\""));
    assert!(stdout.contains("\"status\": \"PASS\""));
    assert!(stdout.contains("\"name\": \"wat_outputs\""));

    let run_output = Command::new("python3")
        .current_dir(repo_root())
        .arg(repo_root().join("tools").join("owcmp").join("owcmp"))
        .arg("manifest")
        .arg("run")
        .arg("--manifest")
        .arg(&manifest_path)
        .output()
        .expect("python3 should run owcmp manifest run");
    assert!(
        !run_output.status.success(),
        "cohort-inventory manifests should not execute as comparisons"
    );
    assert!(
        String::from_utf8_lossy(&run_output.stderr)
            .contains("cohort inventory manifests are preflight declarations"),
        "inventory manifest rejection should explain preflight-only behavior"
    );

    let _ = fs::remove_dir_all(&temp_dir);
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
fn owcmp_batch_h1_h39_semantic_emits_compact_summary() {
    let temp_dir = fixture_temp_dir("owcmp_h1_h39_batch");
    let baseline_dir = temp_dir.join("baseline");
    let candidate_dir = temp_dir.join("candidate");
    let output_root = temp_dir.join("batch_output");
    fs::create_dir_all(&baseline_dir).expect("baseline directory should be creatable");
    fs::create_dir_all(&candidate_dir).expect("candidate directory should be creatable");

    let h1_payload = format!("{}\n{}\n", dat_row(1, 1, 2008, 10), dat_row(1, 2, 2008, 20));
    let h2_payload = format!("{}\n{}\n", dat_row(2, 1, 2008, 30), dat_row(2, 2, 2008, 40));
    fs::write(baseline_dir.join("baseline_H1.wat.dat"), &h1_payload)
        .expect("H1 baseline fixture should be writable");
    fs::write(candidate_dir.join("H1.wat.dat"), &h1_payload)
        .expect("H1 candidate fixture should be writable");
    fs::write(baseline_dir.join("baseline_H2.wat.dat"), &h2_payload)
        .expect("H2 baseline fixture should be writable");
    fs::write(candidate_dir.join("H2.wat.dat"), &h2_payload)
        .expect("H2 candidate fixture should be writable");

    let batch_output = Command::new("python3")
        .current_dir(repo_root())
        .arg(repo_root().join("tools").join("owcmp").join("owcmp"))
        .arg("batch")
        .arg("h1-h39-semantic")
        .arg("--baseline-dir")
        .arg(&baseline_dir)
        .arg("--candidate-dir")
        .arg(&candidate_dir)
        .arg("--output-root")
        .arg(&output_root)
        .arg("--start")
        .arg("1")
        .arg("--end")
        .arg("2")
        .arg("--baseline-pattern")
        .arg("baseline_H{h}.wat.dat")
        .arg("--candidate-pattern")
        .arg("H{h}.wat.dat")
        .arg("--candidate-year-offset")
        .arg("0")
        .output()
        .expect("python3 should run owcmp H1-H39 batch");
    assert!(
        batch_output.status.success(),
        "batch should pass; stderr={}",
        String::from_utf8_lossy(&batch_output.stderr)
    );

    let stdout = String::from_utf8_lossy(&batch_output.stdout);
    assert!(stdout.contains("\"semantic_pass_count\": \"2/2\""));
    assert!(stdout.contains("\"execution_verdict\": \"PASS\""));

    let summary_json =
        fs::read_to_string(output_root.join("summary.json")).expect("summary json exists");
    assert!(summary_json.contains("\"source_type\": \"h1_h39_semantic_batch\""));
    assert!(summary_json.contains("\"semantic_pass_count\": \"2/2\""));
    assert!(summary_json.contains("\"execution_verdict\": \"PASS\""));
    assert!(summary_json.contains("\"command_log\""));
    assert!(output_root.join("command-log.json").is_file());
    assert!(
        output_root
            .join("reports")
            .join("semantic")
            .join("H1.semantic.json")
            .is_file()
    );
    assert!(output_root.join("logs").join("H2.stderr.txt").is_file());

    let summary_md =
        fs::read_to_string(output_root.join("summary.md")).expect("summary markdown exists");
    assert!(summary_md.contains("# owcmp H1-H39 Semantic Batch"));
    assert!(summary_md.contains("Semantic pass count: `2/2`"));

    let _ = fs::remove_dir_all(&temp_dir);
}

#[test]
fn owcmp_batch_h1_h39_semantic_missing_inputs_emit_failure_artifacts() {
    let temp_dir = fixture_temp_dir("owcmp_h1_h39_batch_missing");
    let baseline_dir = temp_dir.join("baseline");
    let candidate_dir = temp_dir.join("candidate");
    let output_root = temp_dir.join("batch_output");
    fs::create_dir_all(&baseline_dir).expect("baseline directory should be creatable");
    fs::create_dir_all(&candidate_dir).expect("candidate directory should be creatable");

    let batch_output = Command::new("python3")
        .current_dir(repo_root())
        .arg(repo_root().join("tools").join("owcmp").join("owcmp"))
        .arg("batch")
        .arg("h1-h39-semantic")
        .arg("--baseline-dir")
        .arg(&baseline_dir)
        .arg("--candidate-dir")
        .arg(&candidate_dir)
        .arg("--output-root")
        .arg(&output_root)
        .arg("--start")
        .arg("1")
        .arg("--end")
        .arg("1")
        .arg("--baseline-pattern")
        .arg("baseline_H{h}.wat.dat")
        .arg("--candidate-pattern")
        .arg("H{h}.wat.dat")
        .output()
        .expect("python3 should run owcmp H1-H39 batch");
    assert!(
        !batch_output.status.success(),
        "missing input batch should fail closed"
    );

    let summary_json =
        fs::read_to_string(output_root.join("summary.json")).expect("summary json exists");
    assert!(summary_json.contains("\"execution_verdict\": \"FAIL\""));
    assert!(summary_json.contains("\"semantic_verdict\": \"NOT_RUN\""));
    assert!(summary_json.contains("\"summary_json\""));
    assert!(summary_json.contains("\"summary_md\""));
    assert!(summary_json.contains("\"command_log\""));
    assert!(summary_json.contains("\"pass_hillslopes\""));
    assert!(summary_json.contains("\"failed_hillslopes\""));
    assert!(summary_json.contains("\"focus_columns\""));
    assert!(summary_json.contains("\"first_divergent\""));
    assert!(summary_json.contains("\"missing_inputs\""));
    assert!(output_root.join("summary.md").is_file());
    assert!(output_root.join("command-log.json").is_file());

    let command_log =
        fs::read_to_string(output_root.join("command-log.json")).expect("command log exists");
    assert_eq!(command_log.trim(), "[]");

    let _ = fs::remove_dir_all(&temp_dir);
}

#[test]
fn owcmp_batch_h1_h39_semantic_command_failure_emits_failure_artifacts() {
    let temp_dir = fixture_temp_dir("owcmp_h1_h39_batch_command_failure");
    let baseline_dir = temp_dir.join("baseline");
    let candidate_dir = temp_dir.join("candidate");
    let output_root = temp_dir.join("batch_output");
    fs::create_dir_all(&baseline_dir).expect("baseline directory should be creatable");
    fs::create_dir_all(&candidate_dir).expect("candidate directory should be creatable");

    let duplicate_baseline = format!("{}\n{}\n", dat_row(1, 1, 2008, 10), dat_row(1, 1, 2008, 20));
    let candidate_payload = format!("{}\n", dat_row(1, 1, 2008, 10));
    fs::write(baseline_dir.join("baseline_H1.wat.dat"), duplicate_baseline)
        .expect("H1 baseline fixture should be writable");
    fs::write(candidate_dir.join("H1.wat.dat"), candidate_payload)
        .expect("H1 candidate fixture should be writable");

    let batch_output = Command::new("python3")
        .current_dir(repo_root())
        .arg(repo_root().join("tools").join("owcmp").join("owcmp"))
        .arg("batch")
        .arg("h1-h39-semantic")
        .arg("--baseline-dir")
        .arg(&baseline_dir)
        .arg("--candidate-dir")
        .arg(&candidate_dir)
        .arg("--output-root")
        .arg(&output_root)
        .arg("--start")
        .arg("1")
        .arg("--end")
        .arg("1")
        .arg("--baseline-pattern")
        .arg("baseline_H{h}.wat.dat")
        .arg("--candidate-pattern")
        .arg("H{h}.wat.dat")
        .arg("--candidate-year-offset")
        .arg("0")
        .output()
        .expect("python3 should run owcmp H1-H39 batch");
    assert!(
        !batch_output.status.success(),
        "duplicate row batch should fail closed"
    );

    let summary_json =
        fs::read_to_string(output_root.join("summary.json")).expect("summary json exists");
    assert!(summary_json.contains("\"execution_verdict\": \"FAIL\""));
    assert!(summary_json.contains("\"semantic_verdict\": \"NOT_RUN\""));
    assert!(summary_json.contains("\"summary_json\""));
    assert!(summary_json.contains("\"summary_md\""));
    assert!(summary_json.contains("\"command_log\""));
    assert!(summary_json.contains("\"pass_hillslopes\""));
    assert!(summary_json.contains("\"failed_hillslopes\""));
    assert!(summary_json.contains("\"focus_columns\""));
    assert!(summary_json.contains("\"first_divergent\""));
    assert!(summary_json.contains("\"failed_hillslope\": 1"));
    assert!(output_root.join("summary.md").is_file());
    assert!(output_root.join("command-log.json").is_file());

    let stderr =
        fs::read_to_string(output_root.join("logs").join("H1.stderr.txt")).expect("stderr exists");
    assert!(stderr.contains("duplicate row key"));

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
    let baseline_fixture_root = temp_dir.join("baseline_fixture");
    let baseline_runs_dir = baseline_fixture_root.join("runs");
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
        .arg(&baseline_fixture_root)
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
