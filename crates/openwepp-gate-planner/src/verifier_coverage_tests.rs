use std::collections::BTreeSet;

use serde_json::{Value, json};

use super::{MemoryArtifacts, RootlessArtifacts, normalized_plan_and_receipt, refresh_receipt_id};
use crate::canonical::{derived_id, digest};
use crate::planner::{derive_execution_key, derive_plan_id};
use crate::verifier::{AttestationIdentity, EnvelopeVerdict, ReceiptVerdict};

fn refresh_plan_and_receipt(plan: &mut Value, receipt: &mut Value) {
    plan["plan_id"] = json!(derive_plan_id(plan).expect("plan ID"));
    plan["execution_key"] = json!(derive_execution_key(plan).expect("execution key"));
    receipt["plan_id"] = plan["plan_id"].clone();
    receipt["plan_sha256"] = json!(digest(plan).expect("plan digest"));
    receipt["execution_key"] = plan["execution_key"].clone();
    receipt["dag_sha256"] = json!(digest(&plan["nodes"]).expect("DAG digest"));
    receipt["dag_nodes"] = Value::Array(
        plan["nodes"]
            .as_array()
            .expect("plan nodes")
            .iter()
            .map(|node| {
                let mut snapshot = node.clone();
                snapshot.as_object_mut().expect("node object").insert(
                    "plan_node_sha256".to_owned(),
                    json!(digest(node).expect("node digest")),
                );
                snapshot
            })
            .collect(),
    );
    refresh_receipt_id(receipt);
}

fn replace_string(value: &mut Value, old: &str, new: &str) {
    match value {
        Value::String(text) if text == old => *text = new.to_owned(),
        Value::Array(values) => {
            for value in values {
                replace_string(value, old, new);
            }
        }
        Value::Object(object) => {
            for value in object.values_mut() {
                replace_string(value, old, new);
            }
        }
        _ => {}
    }
}

fn ready_admitted_fixture() -> (Value, Value, MemoryArtifacts) {
    let (mut plan, mut receipt, artifacts) = normalized_plan_and_receipt();
    let node = plan["nodes"]
        .as_array_mut()
        .expect("plan nodes")
        .last_mut()
        .expect("nonempty plan");
    let old_node_id = node["node_id"].as_str().expect("node ID").to_owned();
    node["execution_cost_class"] = json!("HEAVY");
    let new_node_id = derived_id(node, "node_id").expect("updated node ID");
    node["node_id"] = json!(new_node_id);
    replace_string(&mut plan, &old_node_id, &new_node_id);
    replace_string(&mut receipt, &old_node_id, &new_node_id);
    refresh_plan_and_receipt(&mut plan, &mut receipt);
    (plan, receipt, artifacts)
}

#[test]
fn ready_audit_verification_preserves_order_and_exact_verdict() {
    let root = super::repo();
    let (plan, receipt, artifacts) = ready_admitted_fixture();
    let verdict = crate::verifier::verify_receipt_after_ready_audit(
        &root, &plan, &receipt, &artifacts,
    )
    .expect("READY-admitted valid receipt");
    assert_eq!(verdict.receipt_id(), receipt["receipt_id"]);
    assert_eq!(verdict.result(), "PASS");
    assert_eq!(verdict.trust_class(), "LOCAL_UNTRUSTED");
    assert_eq!(
        verdict.claimed_trust_class(),
        receipt["claims"]["trust_class"]
    );

    let (plan, mut wrong_identity, artifacts) = ready_admitted_fixture();
    wrong_identity["plan_id"] = json!("0".repeat(64));
    let error = crate::verifier::verify_receipt_after_ready_audit(
        &root,
        &plan,
        &wrong_identity,
        &artifacts,
    )
    .expect_err("identity must be checked first");
    assert_eq!(error.code, "GATE-RECEIPT-ID");

    let (mut wrong_context, mut receipt, artifacts) = ready_admitted_fixture();
    wrong_context["execution_context"]["tool_manifest_sha256"] = json!("0".repeat(64));
    refresh_plan_and_receipt(&mut wrong_context, &mut receipt);
    let error = crate::verifier::verify_receipt_after_ready_audit(
        &root,
        &wrong_context,
        &receipt,
        &artifacts,
    )
    .expect_err("live execution context must be checked second");
    assert_eq!(error.code, "GATE-RECEIPT-EXECUTION-CONTEXT");
    assert_eq!(
        error.message,
        "execution context changed during the audit-admitted HEAVY transition"
    );

    let (light_plan, receipt, artifacts) = normalized_plan_and_receipt();
    assert!(
        light_plan["nodes"]
            .as_array()
            .expect("plan nodes")
            .iter()
            .all(|node| node["execution_cost_class"] != "HEAVY"),
        "the normalized increment fixture must remain light-only"
    );
    let error = crate::verifier::verify_receipt_after_ready_audit(
        &root,
        &light_plan,
        &receipt,
        &artifacts,
    )
    .expect_err("HEAVY admission must be checked third");
    assert_eq!(error.code, "GATE-RECEIPT-AUDIT-ADMISSION");
    assert_eq!(
        error.message,
        "READY-audit receipt verification requires a HEAVY terminal plan"
    );

    let (plan, mut downstream_invalid, artifacts) = ready_admitted_fixture();
    downstream_invalid["source"]["tree_sha256"] = json!("0".repeat(64));
    refresh_receipt_id(&mut downstream_invalid);
    let error = crate::verifier::verify_receipt_after_ready_audit(
        &root,
        &plan,
        &downstream_invalid,
        &artifacts,
    )
    .expect_err("valid admission must delegate to full receipt verification");
    assert_eq!(error.code, "GATE-RECEIPT-TREE");
}

#[test]
fn verdict_accessors_preserve_identity_and_trust_fields() {
    let receipt = ReceiptVerdict {
        receipt_id: "receipt".to_owned(),
        receipt_sha256: "receipt-sha".to_owned(),
        plan_id: "plan".to_owned(),
        plan_sha256: "plan-sha".to_owned(),
        execution_key: "execution".to_owned(),
        roots_sha256: "roots".to_owned(),
        boundary: "INCREMENT".to_owned(),
        result: "PASS_WITH_RETRY".to_owned(),
        trust_class: "LOCAL_UNTRUSTED".to_owned(),
        claimed_trust_class: "REPOSITORY_REVIEWED".to_owned(),
    };
    assert_eq!(receipt.receipt_id(), "receipt");
    assert_eq!(receipt.result(), "PASS_WITH_RETRY");
    assert_eq!(receipt.trust_class(), "LOCAL_UNTRUSTED");
    assert_eq!(receipt.claimed_trust_class(), "REPOSITORY_REVIEWED");

    let envelope = EnvelopeVerdict {
        envelope_id: "envelope".to_owned(),
        envelope_sha256: "envelope-sha".to_owned(),
        receipt_id: "receipt".to_owned(),
        receipt_sha256: "receipt-sha".to_owned(),
        trust_class: "PROTECTED_CI".to_owned(),
        policy_generation: 1,
        identity: AttestationIdentity {
            principal_id: "principal".to_owned(),
            trust_root_id: "root".to_owned(),
            repository: "repository".to_owned(),
            source_commit: "0".repeat(40),
            source_ref: "refs/heads/main".to_owned(),
            workflow: "workflow".to_owned(),
            workflow_sha256: "0".repeat(64),
            job: "job".to_owned(),
            runner_image_sha256: "0".repeat(64),
            attempt: 1,
            plan_id: "plan".to_owned(),
            execution_key: "execution".to_owned(),
            receipt_id: "receipt".to_owned(),
            receipt_sha256: "receipt-sha".to_owned(),
            artifacts: BTreeSet::new(),
        },
    };
    assert_eq!(envelope.envelope_id(), "envelope");
    assert_eq!(envelope.envelope_sha256(), "envelope-sha");
    assert_eq!(envelope.trust_class(), "PROTECTED_CI");
}

#[test]
fn local_verifier_guards_cover_retry_prerequisite_audit_and_binding_edges() {
    let node = json!({
        "node_id": "node",
        "arguments": ["test"],
        "acceptance": {"kind": "EXIT_CODE", "operator": "EQUALS", "expected": 0},
        "retry": {"maximum_attempts": 2, "permitted_reasons": ["FLAKY"]},
        "prerequisites": []
    });
    let first = json!({
        "node_id": "node", "attempt": 1, "arguments": ["test"],
        "exit_code": 0, "termination_signal": null, "result": "PASS",
        "retry_reason": null
    });
    super::super::verify_attempt("node", &node, &first, 0, false)
        .expect("first accepted attempt");
    let mut retry = first.clone();
    retry["attempt"] = json!(2);
    retry["retry_reason"] = json!("FLAKY");
    super::super::verify_attempt("node", &node, &retry, 1, false)
        .expect("permitted retry");
    retry["retry_reason"] = json!("UNPERMITTED");
    assert_eq!(
        super::super::verify_attempt("node", &node, &retry, 1, false)
            .expect_err("unpermitted retry")
            .code,
        "GATE-RECEIPT-RETRY-POLICY"
    );
    retry["arguments"] = json!(["different"]);
    assert_eq!(
        super::super::verify_attempt("node", &node, &retry, 1, false)
            .expect_err("attempt identity mismatch")
            .code,
        "GATE-RECEIPT-ATTEMPT"
    );

    let prerequisite = json!({"node_id": "first", "prerequisites": []});
    let dependent = json!({"node_id": "second", "prerequisites": ["first"]});
    let receipt = json!({"attempts": [
        {"node_id": "first", "result": "FAIL"},
        {"node_id": "second", "result": "PASS"}
    ]});
    assert_eq!(
        super::super::verify_prerequisite_results(&[prerequisite.clone(), dependent.clone()], &receipt)
            .expect_err("dependent must be blocked")
            .code,
        "GATE-RECEIPT-PREREQUISITE"
    );
    let blocked = json!({"attempts": [
        {"node_id": "first", "result": "FAIL"},
        {"node_id": "second", "result": "BLOCKED"}
    ]});
    super::super::verify_prerequisite_results(&[prerequisite, dependent], &blocked)
        .expect("blocked dependent is truthful");

    let terminal_light = json!({"planning_stage": "TERMINAL"});
    let audit_claim = json!({"pre_heavy_audit": {}});
    assert_eq!(
        super::super::verify_heavy_audit(
            &super::repo(),
            &terminal_light,
            &audit_claim,
            &RootlessArtifacts,
            &[json!({"execution_cost_class": "LIGHT"})],
        )
        .expect_err("light plan must reject an audit")
        .code,
        "GATE-RECEIPT-UNEXPECTED-AUDIT"
    );
    assert_eq!(
        super::super::verify_heavy_audit(
            &super::repo(),
            &terminal_light,
            &json!({"pre_heavy_audit": {}}),
            &RootlessArtifacts,
            &[json!({"execution_cost_class": "HEAVY"})],
        )
        .expect_err("heavy audit needs an attempt root")
        .code,
        "GATE-RECEIPT-AUDIT-ROOT"
    );

    let receipt_artifacts = json!({"artifacts": [
        {"artifact_id": "a", "sha256": "sha"}
    ]});
    super::super::verify_envelope_artifacts(&receipt_artifacts, &receipt_artifacts)
        .expect("identical artifact subjects");
    let different_artifacts = json!({"artifacts": [
        {"artifact_id": "a", "sha256": "different"}
    ]});
    assert_eq!(
        super::super::verify_envelope_artifacts(&receipt_artifacts, &different_artifacts)
            .expect_err("artifact mismatch")
            .code,
        "GATE-ENVELOPE-ARTIFACTS"
    );

    let left = json!({"nested": {"value": 1}});
    let right = json!({"nested": {"value": 1}, "other": 2});
    super::super::equal(&left, "/nested/value", &right, "/nested/value", "TEST-EQUAL")
        .expect("equal pointer values");
    assert_eq!(
        super::super::equal(&left, "/nested/value", &right, "/other", "TEST-EQUAL")
            .expect_err("different pointer values")
            .code,
        "TEST-EQUAL"
    );
}
