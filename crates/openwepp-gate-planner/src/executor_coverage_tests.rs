use serde_json::{Value, json};

use super::{ExecutionClaims, ExecutionRecord, execute_plan_stage};
use crate::canonical::derived_id;

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
