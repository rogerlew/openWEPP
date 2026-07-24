use std::fs;
use std::path::{Path, PathBuf};

use serde_json::{Value, json};

use super::{ExecutionClaims, ExecutionRecord, execute_plan, execute_plan_stage};
use crate::canonical::{derived_id, digest, sha256_bytes};
use crate::pre_heavy::construct_audit;

#[test]
fn quality_deferral_is_exact_and_rejects_retired_nodes() {
    let disposition = json!({
        "status": "DEFERRED_TO_QUALITY_CI",
        "observations": ["COVERAGE", "CRAP"],
        "owner": "openwepp-quality-observatory",
        "trigger": "OPTIONAL_OPERATOR_DISPATCH",
        "closure_eligible": true,
        "prohibited_gate_definition_ids": [
            "affected-adjudicated-crap-v1",
            "adjudicated-crap-v1",
            "combined-workspace-quality-v1"
        ]
    });
    let plan = json!({"quality_disposition": disposition, "nodes": []});
    super::validate_quality_disposition(&plan).expect("exact quality deferral");

    let mut invalid = plan.clone();
    invalid["quality_disposition"]["status"] = json!("SKIPPED");
    assert_eq!(
        super::validate_quality_disposition(&invalid)
            .expect_err("invalid disposition")
            .code,
        "GATE-EXEC-QUALITY-DISPOSITION"
    );

    invalid = plan;
    invalid["nodes"] = json!([{
        "gate_definition_id": "adjudicated-crap-v1",
        "gate_family": "coverage-complexity",
        "artifact_contract": "adjudicated-crap-v1"
    }]);
    assert_eq!(
        super::validate_quality_disposition(&invalid)
            .expect_err("retired node")
            .code,
        "GATE-EXEC-QUALITY-NODE-PROHIBITED"
    );
}

struct DurableLedger(PathBuf);

impl DurableLedger {
    fn new(label: &str) -> Self {
        let repository = fs::canonicalize(PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../.."))
            .expect("canonical repository");
        let path = repository.join("target").join(format!(
            "{label}-{}-{}.jsonl",
            std::process::id(),
            std::thread::current().name().unwrap_or("test")
        ));
        fs::create_dir_all(path.parent().expect("ledger parent")).expect("ledger directory");
        fs::write(&path, "").expect("empty durable ledger");
        Self(path)
    }

    fn path(&self) -> &Path {
        &self.0
    }
}

impl Drop for DurableLedger {
    fn drop(&mut self) {
        fs::remove_file(&self.0).expect("remove durable ledger");
    }
}

#[test]
fn monolithic_executor_rejects_heavy_before_repository_access() {
    let plan = json!({
        "nodes": [{"execution_cost_class": "HEAVY"}]
    });
    let error = execute_plan(
        Path::new("/path/that/must/not/be-opened"),
        &plan,
        Path::new("/path/that/must/not/be-created"),
        &ExecutionClaims::default(),
    )
    .expect_err("heavy plan must require staged audit admission");
    assert_eq!(error.code, "GATE-EXEC-HEAVY-REQUIRES-AUDIT");
}

#[test]
fn heavy_handoff_accepts_only_checkpoint_bound_light_artifacts() {
    use super::tests::TempDirectory;

    let artifacts = TempDirectory::new("light-handoff");
    let node_id = "1".repeat(64);
    let node = json!({
        "node_id": node_id,
        "output_paths": ["target/light/result.json"]
    });
    let output = artifacts.path().join("target/light/result.json");
    fs::create_dir_all(output.parent().expect("output parent")).expect("output directory");
    fs::write(&output, b"bound\n").expect("light output");
    let checkpoint_dir = artifacts.path().join(".checkpoints");
    fs::create_dir(&checkpoint_dir).expect("checkpoint directory");
    fs::write(
        checkpoint_dir.join(format!("{node_id}.json")),
        serde_json::to_vec_pretty(&json!({
            "node_sha256": digest(&node).expect("node digest"),
            "result": "PASS",
            "artifacts": [{
                "path": "target/light/result.json",
                "sha256": sha256_bytes(b"bound\n")
            }]
        }))
        .expect("serialize checkpoint"),
    )
    .expect("write checkpoint");
    super::verify_checkpoint_artifact(artifacts.path(), &node, "target/light/result.json")
        .expect("bound light artifact");
    fs::write(&output, b"mutated\n").expect("mutate light output");
    let error =
        super::verify_checkpoint_artifact(artifacts.path(), &node, "target/light/result.json")
            .expect_err("mutated light artifact must fail");
    assert_eq!(error.code, "GATE-EXEC-CHECKPOINT-ARTIFACT-DRIFT");
}

#[test]
fn authority_report_proves_executed_suite_inventory() {
    use std::collections::BTreeSet;

    use super::tests::TempDirectory;

    let artifacts = TempDirectory::new("authority-report");
    let report = artifacts
        .path()
        .join(".work/target/gate-plan/required-authority-report.md");
    fs::create_dir_all(report.parent().expect("report parent")).expect("report directory");
    fs::write(
        &report,
        "- lane=required failure_class=hard-fail blocking=true test=one suites=suite_a,suite_b status=pass\n\
         - lane=required failure_class=investigation blocking=false test=two suites=ignored status=pass\n",
    )
    .expect("authority report");
    let node = json!({
        "gate_definition_id": "required-authority-v1",
        "output_paths": ["target/gate-plan/required-authority-report.md"]
    });
    let observed = super::observed_authority_inventory(artifacts.path(), &node, "PASS")
        .expect("observed authority inventory");
    assert_eq!(
        observed,
        BTreeSet::from(["suite_a".to_owned(), "suite_b".to_owned()])
    );
}

#[test]
fn authority_outcomes_encode_admission_science_and_truthful_nonpass() {
    use std::collections::BTreeMap;

    let nodes = vec![
        json!({"node_id": "a0", "gate_definition_id": "authority-admission-v1", "authority_class": "A0"}),
        json!({"node_id": "a3", "gate_definition_id": "required-authority-v1", "authority_class": "A3"}),
    ];
    let pass = BTreeMap::from([
        ("a0".to_owned(), "PASS".to_owned()),
        ("a3".to_owned(), "PASS".to_owned()),
    ]);
    let outcomes = super::authority_outcomes(&nodes, &pass).expect("authority pass outcomes");
    assert_eq!(outcomes[0]["admission_outcome"], "ADMITTED");
    assert_eq!(outcomes[1]["scientific_outcome"], "CONFORMS");

    let fail = BTreeMap::from([
        ("a0".to_owned(), "FAIL".to_owned()),
        ("a3".to_owned(), "BLOCKED".to_owned()),
    ]);
    let outcomes = super::authority_outcomes(&nodes, &fail).expect("authority nonpass outcomes");
    assert_eq!(outcomes[0]["admission_outcome"], "REJECTED");
    assert_eq!(outcomes[1]["scientific_outcome"], "NOT_EVALUATED");

    let unsupported = vec![
        json!({"node_id": "fake", "gate_definition_id": "generic-process", "authority_class": "A3"}),
    ];
    let result = BTreeMap::from([("fake".to_owned(), "PASS".to_owned())]);
    let error =
        super::authority_outcomes(&unsupported, &result).expect_err("generic authority claim");
    assert_eq!(error.code, "GATE-EXEC-AUTHORITY-UNSUPPORTED");
}

#[test]
fn source_mutation_and_checkout_precedence_are_preserved() {
    use std::collections::{BTreeMap, BTreeSet};

    use super::tests::{execution_fixture, gate_definition};

    let mut empty = ExecutionRecord::empty();
    assert_eq!(
        super::mark_source_mutation(&mut empty)
            .expect_err("mutation without an attempt must fail")
            .code,
        "GATE-EXEC-SOURCE-MUTATION"
    );
    let mut record = ExecutionRecord {
        final_results: BTreeMap::from([("node".to_owned(), "PASS".to_owned())]),
        attempts: vec![json!({"node_id": "node", "result": "PASS"})],
        executed_inventory: BTreeSet::new(),
        unavailable: BTreeMap::new(),
        resume_decisions: Vec::new(),
    };
    super::mark_source_mutation(&mut record).expect("attribute mutation to last attempt");
    assert_eq!(record.attempts[0]["result"], "INVALID");
    assert_eq!(record.final_results["node"], "INVALID");

    super::verify_execution_checkout(Path::new("/path/not-read"), &json!({"source": {}}))
        .expect("a plan without committed head has no checkout binding");
    let (repo, plan) = execution_fixture(
        "checkout-precedence-repo",
        &[gate_definition("fixture-checkout-v1", &["true"], &[])],
    );
    super::verify_execution_checkout(repo.path(), &plan).expect("matching clean checkout");
    let mut wrong_head = plan.clone();
    wrong_head["source"]["head_commit"] = json!("0".repeat(40));
    assert_eq!(
        super::verify_execution_checkout(repo.path(), &wrong_head)
            .expect_err("wrong committed head must fail first")
            .code,
        "GATE-EXEC-CHECKOUT-HEAD"
    );
    fs::write(repo.path().join("untracked-checkout-probe"), "dirty\n")
        .expect("write untracked probe");
    assert_eq!(
        super::verify_execution_checkout(repo.path(), &plan)
            .expect_err("dirty matching checkout must fail")
            .code,
        "GATE-EXEC-CHECKOUT-DIRTY"
    );
}

#[test]
fn environment_and_missing_observed_inventory_are_fail_closed() {
    use super::tests::TempDirectory;

    let absent_key = "OPENWEPP_EXECUTOR_COVERAGE_ABSENT_KEY_7A13";
    let environment = super::allowed_environment(&json!({"environment_allowlist": [absent_key]}))
        .expect("absent optional environment key");
    assert!(environment.is_empty());

    let artifacts = TempDirectory::new("missing-observed-inventory");
    let authority = json!({
        "output_paths": ["target/gate-plan/required-authority-report.md"]
    });
    assert!(
        super::observed_authority_inventory(artifacts.path(), &authority, "FAIL")
            .expect("missing nonpass authority report")
            .is_empty()
    );
    assert_eq!(
        super::observed_authority_inventory(artifacts.path(), &authority, "PASS")
            .expect_err("missing PASS authority report")
            .code,
        "GATE-EXEC-AUTHORITY-REPORT"
    );

    let junit = json!({"arguments": [], "artifact_contract": "nextest-junit-v1"});
    assert!(
        super::observed_junit_inventory(artifacts.path(), &junit, "FAIL")
            .expect("missing nonpass JUnit")
            .is_empty()
    );
    assert_eq!(
        super::observed_junit_inventory(artifacts.path(), &junit, "PASS")
            .expect_err("missing PASS JUnit")
            .code,
        "GATE-EXEC-JUNIT-READ"
    );
}

#[test]
fn junit_contract_dispatch_preserves_exact_inventory() {
    use super::tests::TempDirectory;

    let artifacts = TempDirectory::new("junit-contract-dispatch");
    let expected = sha256_bytes(b"rust-suites::suite\0case");
    let mut node = json!({
        "artifact_contract": "nextest-junit-v1",
        "arguments": [],
        "expected_inventory": {"ids": [expected]},
    });
    let junit = super::nextest_junit_path(artifacts.path(), &node).expect("JUnit path");
    fs::create_dir_all(junit.parent().expect("JUnit parent")).expect("JUnit directory");
    fs::write(
        &junit,
        "<testsuite>\n<testcase classname=\"suite\" name=\"case\"/>\n</testsuite>\n",
    )
    .expect("JUnit report");
    super::validate_success_artifacts(artifacts.path(), &node).expect("exact JUnit inventory");
    node["expected_inventory"]["ids"] = json!(["0".repeat(64)]);
    assert_eq!(
        super::validate_success_artifacts(artifacts.path(), &node)
            .expect_err("mismatched JUnit inventory")
            .code,
        "GATE-EXEC-JUNIT-INVENTORY"
    );
    super::validate_success_artifacts(
        artifacts.path(),
        &json!({"artifact_contract": "process-exit-v1"}),
    )
    .expect("ordinary process artifacts need no real-artifact validation");
}

#[test]
fn real_artifact_reset_and_source_selection_cover_every_contract() {
    use super::tests::TempDirectory;

    let artifacts = TempDirectory::new("real-artifact-reset-sources");
    let absent = artifacts.path().join("absent");
    super::reset_real_artifact(&absent).expect("absent artifact reset");
    let file = artifacts.path().join("file");
    fs::write(&file, "artifact\n").expect("regular artifact");
    super::reset_real_artifact(&file).expect("regular artifact removed");
    assert!(!file.exists());
    let directory = artifacts.path().join("directory");
    fs::create_dir(&directory).expect("artifact directory");
    assert_eq!(
        super::reset_real_artifact(&directory)
            .expect_err("directory is not an artifact file")
            .code,
        "GATE-EXEC-REAL-ARTIFACT-TYPE"
    );
    #[cfg(unix)]
    {
        use std::os::unix::fs::symlink;

        let link = artifacts.path().join("link");
        symlink(&directory, &link).expect("artifact symlink");
        assert_eq!(
            super::reset_real_artifact(&link)
                .expect_err("artifact symlink must fail")
                .code,
            "GATE-EXEC-REAL-ARTIFACT-SYMLINK"
        );
    }

    let nextest = json!({"artifact_contract": "nextest-junit-v1", "arguments": []});
    let work = artifacts.path().join(".work");
    assert_eq!(
        super::real_artifact_sources(artifacts.path(), &nextest).expect("Nextest sources"),
        [work.join("nextest/default/junit.xml")]
    );
    let crap_xml = json!({
        "artifact_contract": "adjudicated-crap-v1",
        "arguments": ["--output-dir", "target/custom-crap"],
        "output_paths": ["report.json", "junit.xml", "workspace.lcov"]
    });
    assert_eq!(
        super::real_artifact_sources(artifacts.path(), &crap_xml).expect("CRAP XML sources"),
        [
            work.join("target/custom-crap/adjudicated-crap-report.json"),
            work.join("target/custom-crap/run-status.json"),
            work.join("target/custom-crap/nextest/full/junit.xml"),
            work.join("target/custom-crap/workspace.lcov"),
        ]
    );
    let crap = json!({
        "artifact_contract": "adjudicated-crap-v1",
        "arguments": [],
        "output_paths": ["report.json"]
    });
    assert_eq!(
        super::real_artifact_sources(artifacts.path(), &crap).expect("CRAP control sources"),
        [
            work.join("target/adjudicated-crap/adjudicated-crap-report.json"),
            work.join("target/adjudicated-crap/run-status.json"),
        ]
    );
    let authority = json!({
        "artifact_contract": "authority-suite-report-v1",
        "output_paths": ["target/authority.md"]
    });
    assert_eq!(
        super::real_artifact_sources(artifacts.path(), &authority).expect("authority source"),
        [work.join("target/authority.md")]
    );
    assert!(
        super::real_artifact_sources(
            artifacts.path(),
            &json!({"artifact_contract": "process-exit-v1"})
        )
        .expect("process source set")
        .is_empty()
    );
}

#[test]
fn artifact_publication_selects_real_and_synthetic_sources() {
    use std::collections::BTreeSet;

    use super::tests::TempDirectory;

    let artifacts = TempDirectory::new("artifact-publication-sources");
    fs::create_dir_all(artifacts.path().join(".work")).expect("work root");
    let nextest = json!({
        "node_id": "nextest",
        "gate_definition_id": "nextest-v1",
        "artifact_contract": "nextest-junit-v1",
        "arguments": [],
        "output_paths": ["result.xml"]
    });
    let junit = super::nextest_junit_path(artifacts.path(), &nextest).expect("JUnit path");
    fs::create_dir_all(junit.parent().expect("JUnit parent")).expect("JUnit directory");
    fs::write(&junit, b"real junit\n").expect("real JUnit");
    let run = |result: &str| super::NodeRun {
        attempt: json!({}),
        result: result.to_owned(),
        log_path: artifacts.path().join("attempt.log"),
        executed_inventory: BTreeSet::new(),
        unavailable_reason: None,
    };
    assert_eq!(
        super::artifact_bytes(
            artifacts.path(),
            &nextest,
            &run("PASS"),
            "log",
            "result.xml"
        )
        .expect("published real JUnit"),
        b"real junit\n"
    );
    fs::remove_file(&junit).expect("remove real JUnit");
    assert_eq!(
        super::artifact_bytes(
            artifacts.path(),
            &nextest,
            &run("PASS"),
            "log",
            "result.xml"
        )
        .expect_err("missing PASS real artifact")
        .code,
        "GATE-EXEC-REAL-ARTIFACT-MISSING"
    );
    let synthetic_junit = super::artifact_bytes(
        artifacts.path(),
        &nextest,
        &run("FAIL"),
        "log",
        "result.xml",
    )
    .expect("synthetic failed JUnit");
    assert_eq!(
        synthetic_junit,
        b"<?xml version=\"1.0\" encoding=\"UTF-8\"?><testsuite name=\"openwepp-gate\" tests=\"0\" failures=\"1\"></testsuite>\n"
    );

    let crap = json!({
        "node_id": "crap",
        "gate_definition_id": "adjudicated-crap-v1",
        "artifact_contract": "adjudicated-crap-v1",
        "arguments": [],
        "output_paths": ["workspace.lcov", "report.json"]
    });
    assert_eq!(
        super::artifact_bytes(
            artifacts.path(),
            &crap,
            &run("FAIL"),
            "log",
            "workspace.lcov"
        )
        .expect("synthetic LCOV"),
        b"TN:\n"
    );
    let process = json!({
        "node_id": "process",
        "gate_definition_id": "process-v1",
        "artifact_contract": "process-exit-v1"
    });
    let process_json = super::artifact_bytes(
        artifacts.path(),
        &process,
        &run("FAIL"),
        "log",
        "result.json",
    )
    .expect("process fallback JSON");
    assert_eq!(
        serde_json::from_slice::<Value>(&process_json).expect("process JSON"),
        json!({
            "schema_version": "openwepp-gate-process-artifact-v1",
            "node_id": "process",
            "gate_definition_id": "process-v1",
            "result": "FAIL",
            "attempt_log_sha256": "log"
        })
    );
}

#[test]
fn real_output_source_selection_covers_every_contract() {
    use super::tests::TempDirectory;

    let artifacts = TempDirectory::new("real-output-source-selection");
    let nextest = json!({
        "artifact_contract": "nextest-junit-v1",
        "arguments": []
    });
    let crap = json!({
        "artifact_contract": "adjudicated-crap-v1",
        "arguments": [],
        "output_paths": ["workspace.lcov", "report.json"]
    });
    let authority = json!({
        "artifact_contract": "authority-suite-report-v1",
        "output_paths": ["target/authority.md"]
    });
    let process = json!({"artifact_contract": "process-exit-v1"});
    let work = artifacts.path().join(".work");
    for (node, output, expected) in [
        (
            &nextest,
            "result.xml",
            Some(work.join("nextest/default/junit.xml")),
        ),
        (
            &crap,
            "junit.xml",
            Some(work.join("target/adjudicated-crap/nextest/full/junit.xml")),
        ),
        (
            &crap,
            "workspace.lcov",
            Some(work.join("target/adjudicated-crap/workspace.lcov")),
        ),
        (
            &crap,
            "report.json",
            Some(work.join("target/adjudicated-crap/adjudicated-crap-report.json")),
        ),
        (
            &authority,
            "authority.md",
            Some(work.join("target/authority.md")),
        ),
        (&process, "result.json", None),
    ] {
        assert_eq!(
            super::real_source_for_output(artifacts.path(), node, output)
                .expect("real source selection"),
            expected
        );
    }
}

#[test]
fn canonical_directory_accepts_only_directories() {
    use super::tests::TempDirectory;

    let root = TempDirectory::new("canonical-directory");
    assert_eq!(
        super::canonical_directory(root.path(), "GATE-TEST-DIRECTORY")
            .expect("canonical directory"),
        fs::canonicalize(root.path()).expect("expected canonical directory")
    );
    let file = root.path().join("file");
    fs::write(&file, "not a directory\n").expect("regular file");
    assert_eq!(
        super::canonical_directory(&file, "GATE-TEST-DIRECTORY")
            .expect_err("regular file must fail")
            .code,
        "GATE-TEST-DIRECTORY"
    );
}

fn valid_stage_receipt() -> Value {
    json!({
        "final_results": {"node-a": "PASS", "node-b": "FAIL"},
        "attempts": [{"node_id": "node-a"}, {"node_id": "node-b"}],
        "executed_inventory": ["case-b", "case-a", "case-a"],
        "unavailable_items": [
            {"item_id": "missing", "reason_code": "FIRST"},
            {"item_id": "missing", "reason_code": "LAST"}
        ]
    })
}

fn assert_stage_receipt_error(receipt: &Value, code: &str, message: &str) {
    let error = ExecutionRecord::from_stage_receipt(receipt)
        .err()
        .expect("malformed stage receipt must fail");
    assert_eq!(error.code, code);
    assert_eq!(error.message, message);
}

#[test]
fn stage_receipt_reconstruction_preserves_field_order_and_collections() {
    let receipt = valid_stage_receipt();
    let record = ExecutionRecord::from_stage_receipt(&receipt).expect("valid stage receipt");
    assert_eq!(record.final_results["node-a"], "PASS");
    assert_eq!(record.final_results["node-b"], "FAIL");
    assert_eq!(
        record.attempts.as_slice(),
        receipt["attempts"].as_array().expect("attempt array")
    );
    assert_eq!(
        record.executed_inventory.into_iter().collect::<Vec<_>>(),
        ["case-a", "case-b"]
    );
    assert_eq!(record.unavailable["missing"], "LAST");
    assert!(record.resume_decisions.is_empty());

    let mut malformed = receipt.clone();
    malformed["final_results"] = json!([]);
    malformed["attempts"] = json!({});
    assert_stage_receipt_error(&malformed, "GATE-EXEC-STAGE-RECEIPT", "final_results");

    let mut malformed = receipt.clone();
    malformed["final_results"]["node-a"] = json!(1);
    malformed["attempts"] = json!({});
    assert_stage_receipt_error(&malformed, "GATE-EXEC-STAGE-RECEIPT", "non-string result");

    let mut malformed = receipt.clone();
    malformed["attempts"] = json!({});
    malformed["executed_inventory"] = json!({});
    assert_stage_receipt_error(&malformed, "GATE-EXEC-STAGE-RECEIPT", "attempts");

    let mut malformed = receipt.clone();
    malformed["executed_inventory"] = json!({});
    malformed["unavailable_items"] = json!({});
    assert_stage_receipt_error(&malformed, "GATE-EXEC-SHAPE", "stage executed inventory");

    let mut malformed = receipt.clone();
    malformed["executed_inventory"] = json!(["case", 1]);
    assert_stage_receipt_error(&malformed, "GATE-EXEC-SHAPE", "stage executed inventory");

    let mut malformed = receipt.clone();
    malformed["unavailable_items"] = json!({});
    assert_stage_receipt_error(&malformed, "GATE-EXEC-STAGE-RECEIPT", "unavailable_items");

    for (item, message) in [
        (json!({"reason_code": "reason"}), "item_id"),
        (json!({"item_id": 1, "reason_code": "reason"}), "item_id"),
        (json!({"item_id": "item"}), "reason_code"),
        (json!({"item_id": "item", "reason_code": 1}), "reason_code"),
    ] {
        let mut malformed = receipt.clone();
        malformed["unavailable_items"] = json!([item]);
        assert_stage_receipt_error(&malformed, "GATE-EXEC-SHAPE", message);
    }
}

#[test]
#[cfg_attr(
    not(coverage),
    ignore = "development-only: compiles public-stage reconstructed repositories"
)]
fn public_stage_selection_preserves_light_final_and_rejection_shapes() {
    use super::tests::{TempDirectory, execution_fixture, fixture_gate_definition};

    let (repo, plan) = execution_fixture(
        "stage-selection-repo",
        &[fixture_gate_definition(&["./tools/pass.sh"], &[])],
    );
    let light_artifacts = TempDirectory::new("stage-selection-light");
    let light = execute_plan_stage(
        repo.path(),
        &plan,
        light_artifacts.path(),
        &ExecutionClaims::default(),
        "LIGHT",
        None,
        None,
    )
    .expect("LIGHT stage receipt");
    assert_eq!(light["schema_version"], "openwepp-gate-stage-receipt-v1");
    assert_eq!(light["stage"], "LIGHT");
    assert!(light.get("counts").is_none());
    assert_eq!(
        light["stage_receipt_id"],
        derived_id(&light, "stage_receipt_id").expect("stage receipt identity")
    );

    let final_artifacts = TempDirectory::new("stage-selection-final");
    let final_receipt = execute_plan_stage(
        repo.path(),
        &plan,
        final_artifacts.path(),
        &ExecutionClaims::default(),
        "FINAL_LIGHT",
        None,
        None,
    )
    .expect("FINAL_LIGHT ordinary receipt");
    assert_eq!(final_receipt["schema_version"], "openwepp-gate-receipt-v1");
    assert!(final_receipt.get("counts").is_some());

    let invalid_artifacts = TempDirectory::new("stage-selection-invalid");
    let error = execute_plan_stage(
        repo.path(),
        &plan,
        invalid_artifacts.path(),
        &ExecutionClaims::default(),
        "UNKNOWN",
        None,
        None,
    )
    .expect_err("unknown stage must fail after ordinary admission");
    assert_eq!(error.code, "GATE-EXEC-STAGE");

    let heavy_artifacts = TempDirectory::new("stage-selection-heavy");
    let error = execute_plan_stage(
        repo.path(),
        &plan,
        heavy_artifacts.path(),
        &ExecutionClaims::default(),
        "HEAVY",
        None,
        None,
    )
    .expect_err("HEAVY requires READY audit");
    assert_eq!(error.code, "GATE-EXEC-AUDIT-REQUIRED");
}

#[test]
#[cfg_attr(
    not(coverage),
    ignore = "development-only: compiles a READY-audited reconstructed repository"
)]
fn ready_audited_heavy_preserves_import_and_final_receipt_bindings() {
    use std::collections::BTreeSet;

    use super::tests::{TempDirectory, execution_fixture, gate_definition};

    let documentation_definition =
        gate_definition("documentation-lint-v1", &["markdown-doc", "lint"], &[]);
    let light_definition = gate_definition("fixture-light-v1", &["./tools/pass.sh"], &[]);
    let mut heavy_definition = gate_definition(
        "adjudicated-crap-v1",
        &["./tools/pass.sh"],
        &["fixture-light-v1"],
    );
    heavy_definition["execution_cost_class"] = json!("HEAVY");
    let (repo, plan) = execution_fixture(
        "stage-selection-heavy-ready-repo",
        &[documentation_definition, light_definition, heavy_definition],
    );
    let artifacts = TempDirectory::new("stage-selection-heavy-ready-artifacts");
    let claims = ExecutionClaims::default();
    let light = execute_plan_stage(
        repo.path(),
        &plan,
        artifacts.path(),
        &claims,
        "LIGHT",
        None,
        None,
    )
    .expect("LIGHT stage receipt");
    let ledger = DurableLedger::new("executor-heavy-ready-ledger");
    let audit = construct_audit(repo.path(), &plan, &light, artifacts.path(), ledger.path())
        .expect("construct READY audit");
    assert_eq!(
        audit.as_value()["status"],
        "READY",
        "constructed audit: {}",
        audit.as_value()
    );

    let receipt = execute_plan_stage(
        repo.path(),
        &plan,
        artifacts.path(),
        &claims,
        "HEAVY",
        Some(&audit),
        None,
    )
    .expect("audited HEAVY receipt");
    let attempts = receipt["attempts"].as_array().expect("attempts");
    let light_attempts = light["attempts"].as_array().expect("LIGHT attempts");
    assert_eq!(&attempts[..light_attempts.len()], light_attempts);
    let heavy_node_ids = plan["nodes"]
        .as_array()
        .expect("nodes")
        .iter()
        .filter(|node| node["execution_cost_class"] == "HEAVY")
        .map(|node| node["node_id"].as_str().expect("node ID"))
        .collect::<Vec<_>>();
    let heavy_attempts = &attempts[light_attempts.len()..];
    assert_eq!(
        heavy_attempts
            .iter()
            .map(|attempt| attempt["node_id"].as_str().expect("attempt node ID"))
            .collect::<Vec<_>>(),
        heavy_node_ids
    );
    assert!(
        heavy_attempts
            .iter()
            .all(|attempt| attempt["result"] == "PASS")
    );
    assert_eq!(attempts.len(), 3);
    let expected_inventory = light["executed_inventory"]
        .as_array()
        .expect("LIGHT inventory")
        .iter()
        .chain(
            plan["nodes"]
                .as_array()
                .expect("nodes")
                .iter()
                .filter(|node| node["execution_cost_class"] == "HEAVY")
                .flat_map(|node| {
                    node["expected_inventory"]["ids"]
                        .as_array()
                        .expect("expected inventory")
                }),
        )
        .map(|item| item.as_str().expect("inventory ID"))
        .collect::<BTreeSet<_>>();
    assert_eq!(
        receipt["executed_inventory"]
            .as_array()
            .expect("executed inventory")
            .iter()
            .map(|item| item.as_str().expect("inventory ID"))
            .collect::<BTreeSet<_>>(),
        expected_inventory
    );
    assert_eq!(receipt["pre_heavy_audit"], *audit.as_value());
    assert_eq!(receipt["resume_decisions"], json!([]));
    assert_eq!(receipt["result"], "PASS");
}
