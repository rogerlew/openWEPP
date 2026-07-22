use std::fs;
use std::path::{Path, PathBuf};

use serde_json::{Value, json};

use super::{ExecutionClaims, ExecutionRecord, execute_plan, execute_plan_stage};
use crate::canonical::{derived_id, digest, sha256_bytes};
use crate::pre_heavy::construct_audit;

struct DurableLedger(PathBuf);

impl DurableLedger {
    fn new(label: &str) -> Self {
        let repository = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../..");
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
fn public_stage_selection_preserves_light_final_and_rejection_shapes() {
    use super::tests::{TempDirectory, execution_fixture, gate_definition};

    let (repo, plan) = execution_fixture(
        "stage-selection-repo",
        &[gate_definition(
            "affected-adjudicated-crap-v1",
            &["./tools/pass.sh"],
            &[],
        )],
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
fn ready_audited_heavy_preserves_import_and_final_receipt_bindings() {
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
    for node in plan["nodes"].as_array().expect("nodes") {
        let node_id = node["node_id"].as_str().expect("node ID");
        assert!(
            attempts
                .iter()
                .any(|attempt| attempt["node_id"] == node_id && attempt["result"] == "PASS"),
            "missing PASS attempt for {node_id}"
        );
    }
    assert_eq!(attempts.len(), 3);
    assert_eq!(receipt["pre_heavy_audit"], *audit.as_value());
    assert_eq!(receipt["resume_decisions"], json!([]));
    assert_eq!(receipt["result"], "PASS");
}
