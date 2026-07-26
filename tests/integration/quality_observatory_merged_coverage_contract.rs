use std::fs;
use std::path::PathBuf;
use std::process::Command;

use sha2::{Digest, Sha256};

fn root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
}

fn text(path: &str) -> String {
    fs::read_to_string(root().join(path)).expect("contract source must be readable")
}

#[test]
fn quality_observatory_self_test_passes() {
    let output = Command::new(root().join(".venv/bin/python"))
        .arg(root().join("tools/local_ci/quality_observatory.py"))
        .arg("self-test")
        .current_dir(root())
        .output()
        .expect("run quality observatory self-test");
    assert!(
        output.status.success(),
        "stdout:\n{}\nstderr:\n{}",
        String::from_utf8_lossy(&output.stdout),
        String::from_utf8_lossy(&output.stderr)
    );
}

#[test]
fn collector_source_encodes_sequential_profiles_and_merged_only_crap() {
    let source = text("tools/local_ci/quality_observatory.py");
    let full = source
        .find("\"full\",\n        config")
        .expect("full execution call");
    let science = source
        .find("\"science-manual\",\n        config")
        .expect("science-manual execution call");
    assert!(full < science, "full must execute before science-manual");
    assert!(source.contains("[full_raw, science_raw]"));
    assert!(source.contains("invoke_observational_crap("));
    assert!(source.contains("merged_crap,\n        merged_lcov,"));
    assert!(!source.contains("full_lcov,\n        source_manifest_path"));
    let runtime_prime = source
        .find("runtime_artifacts = prime_runtime_cargo_artifacts(")
        .expect("runtime Cargo artifacts are primed");
    let manifest_seal = source
        .find("\"artifacts\": instrumented_artifact_manifest(target)")
        .expect("instrumented artifact manifest");
    assert!(
        runtime_prime < manifest_seal,
        "runtime Cargo artifacts must be primed before manifest sealing"
    );
    assert!(source.contains("\"package\": \"openwepp-assurance\""));
    assert!(source.contains("\"binary\": \"openwepp-assurance\""));
    assert!(source.contains("\"runtime_cargo_artifacts\": payload[\"build_identity\"]"));
    assert!(source.contains("runtime Cargo artifact declaration changed"));
    let final_identity = source
        .rfind("\"during quality finalization\"")
        .expect("final execution identity check");
    let crap = source
        .rfind("invoke_observational_crap(")
        .expect("observational evaluator call");
    let run_status = source.find("run_status = {").expect("PASS run status");
    assert!(crap < final_identity && final_identity < run_status);
}

#[test]
fn nested_fixture_scratch_is_disk_backed_and_admission_bound() {
    let source = text("tools/local_ci/quality_observatory.py");
    assert!(source.contains("temporary_root = local / \"tmp\""));
    assert!(source.contains("base_env[\"TMPDIR\"] = str(temporary_root)"));
    assert!(source.contains("environment[\"TMPDIR\"] = str(temporary_root)"));
    assert!(source.contains("\"temporary_root_policy\": \"attempt-local-disk-backed-v1\""));
    assert!(source.contains("quality temporary-root policy changed after admission"));
}

#[test]
fn nested_cqr_fixture_is_globally_exclusive() {
    let config = text(".config/nextest.toml");
    let group = config
        .find("[test-groups.repository-snapshot]")
        .expect("repository snapshot group");
    let override_start = config[group..]
        .find("[[profile.default.overrides]]")
        .map(|offset| group + offset)
        .expect("repository snapshot override");
    let override_text = &config[override_start..];
    assert!(override_text.contains("binary(cqr_quality_evidence_handoff_contract)"));
    assert!(override_text.contains("test(cqr_quality_evidence_self_test_passes)"));
    assert!(override_text.contains("test-group = \"repository-snapshot\""));
    assert!(override_text.contains("threads-required = \"num-test-threads\""));
}

#[test]
fn independent_inventory_drops_recursive_coverage_wrappers() {
    let source = text("tools/local_ci/quality_observatory.py");
    assert!(source.contains("def without_llvm_cov_wrappers("));
    assert!(source.contains("key not in LLVM_COV_AMBIENT_KEYS"));
    assert!(source.contains("not key.startswith(\"__CARGO_LLVM_COV_\")"));
    assert!(source.contains("environment = without_llvm_cov_wrappers(dict(os.environ))"));
    assert!(!source.contains("QUALITY_CARGO_BUILD_JOBS"));
}

#[test]
fn collector_source_guards_identity_inventory_and_publication() {
    let source = text("tools/local_ci/quality_observatory.py");
    for required in [
        "profile inventories overlap",
        "profile union does not equal canonical inventory",
        "JUnit does not equal admitted inventory",
        "instrumented build identity changed after admission",
        "execution snapshot has test-incompatible directories",
        "execution snapshot .venv is not Git-clean",
        "execution snapshot changed during quality collection",
        "source checkout changed during quality collection",
        "published file set mismatch",
        "quality evidence ID does not match canonical payload",
        "quality payload contains its derived ID",
        "100 * 1024 * 1024",
        "RETAINED_OBSERVATIONAL_DEBT_REQUIRES_REVIEW",
    ] {
        assert!(source.contains(required), "missing guard: {required}");
    }
    assert!(
        !source.contains("path.chmod(") && !source.contains("snapshot.chmod("),
        "the observatory must not break valid repo-relative scratch writes"
    );
    assert!(source.contains("exclude_bound_venv_from_git(snapshot)"));
    assert!(
        source.contains("excluded .venv symlink-target drift did not change working-tree identity")
    );
    assert!(source.contains("broad pre-existing Git exclude policy survived"));
    assert!(source.contains("exact .venv exclude hid other untracked drift"));
    assert!(source.contains("Git exclude-policy drift was accepted"));
    assert!(source.contains("Git info-directory symlink was accepted"));
    assert!(source.contains("post-admission executable growth was accepted"));
}

#[test]
fn observational_crap_mode_preserves_debt_and_is_not_closure_eligible() {
    let source = text("tools/release/check_adjudicated_crap.py");
    assert!(source.contains("--observational"));
    assert!(source.contains("report[\"debt_status\"] = debt_status"));
    assert!(source.contains("report[\"closure_eligible\"] = False"));
    assert!(source.contains("report[\"status\"] = \"OBSERVATION-COMPLETE\""));
    assert!(source.contains("args.observational or report[\"debt_status\"] == \"PASS\""));
}

#[test]
fn historical_manual_science_row_ledger_is_exact_and_source_bound() {
    let ledger = text(
        "docs/work-packages/20260724-quality-observatory-merged-coverage-001/artifacts/snowbench-full-only-row-ledger.json",
    );
    assert_eq!(
        format!("{:x}", Sha256::digest(ledger.as_bytes())),
        "ff542b01772c39dd44d6d6c4d6fb6a376755d980f7faf7692df6b3566f62e257"
    );
    let payload: serde_json::Value = serde_json::from_str(&ledger).expect("snowbench ledger JSON");
    assert_eq!(payload["row_count"], 18);
    assert_eq!(payload["rows"].as_array().expect("rows").len(), 18);
    assert_eq!(payload["source"]["run_id"], "30113946779");
    assert_eq!(
        payload["source"]["head_commit"],
        "4b3e5435b1831c2a8a7d021c2dae879c18a6cd17"
    );
    let mut identities = payload["rows"]
        .as_array()
        .expect("rows")
        .iter()
        .map(|row| {
            (
                row["file"].as_str().expect("file"),
                row["function"].as_str().expect("function"),
            )
        })
        .collect::<Vec<_>>();
    identities.sort_unstable();
    identities.dedup();
    assert_eq!(identities.len(), 18);
}
