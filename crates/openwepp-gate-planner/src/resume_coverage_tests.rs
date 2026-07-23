use std::fs;
use std::path::PathBuf;
use std::process::Command;
use std::sync::atomic::{AtomicU64, Ordering};

use serde_json::{Value, json};

use super::{
    RecoveryArchive, load_accepted_receipt, load_candidate_internal, load_recovery_envelope,
    reuse_reason, verify_checkpoint, verify_native_attestation,
};
use crate::canonical::{canonical_bytes, derived_id, digest, sha256_bytes};
use crate::executor::ExecutionClaims;

static SCRATCH_SEQUENCE: AtomicU64 = AtomicU64::new(0);

struct CheckpointFixture {
    root: PathBuf,
    plan: Value,
    node: Value,
    checkpoint: Value,
    receipt: Value,
}

struct OwnedScratch(PathBuf);

impl OwnedScratch {
    fn new(label: &str) -> Self {
        let path =
            std::env::temp_dir().join(format!("openwepp-resume-{label}-{}", std::process::id()));
        fs::create_dir_all(&path).expect("scratch root");
        Self(path)
    }

    fn path(&self) -> &std::path::Path {
        &self.0
    }
}

impl Drop for OwnedScratch {
    fn drop(&mut self) {
        if self.0.is_dir() {
            fs::remove_dir_all(&self.0).expect("remove scratch root");
        }
    }
}

impl CheckpointFixture {
    fn new() -> Self {
        let sequence = SCRATCH_SEQUENCE.fetch_add(1, Ordering::Relaxed);
        let root = std::env::temp_dir().join(format!(
            "openwepp-resume-checkpoint-characterization-{}-{sequence}",
            std::process::id(),
        ));
        fs::create_dir_all(root.join("out")).expect("artifact root");
        fs::write(root.join("out/result"), b"pass").expect("artifact");
        let node = json!({
            "node_id": "1".repeat(64), "execution_cost_class": "HEAVY",
            "reuse_class": "HERMETIC_CONTENT", "output_paths": ["out/result"]
        });
        let plan = json!({
            "plan_id": "2".repeat(64), "execution_key": "3".repeat(64),
            "boundary": "INCREMENT",
            "environment_roots": {
                "execution_root": "e", "authority_root": "a", "assurance_root": "s"
            },
            "execution_context": {"tool": "fixed"}, "policy": {"generation": 1}
        });
        let attempt = json!({"node_id": node["node_id"], "attempt": 1, "result": "PASS"});
        let claims = json!({"workflow": "w", "job": "j", "runner": "r", "attempt": 1});
        let artifact = json!({"path": "out/result", "sha256": sha256_bytes(b"pass")});
        let mut checkpoint = json!({
            "schema_version": "openwepp-gate-node-checkpoint-v1",
            "checkpoint_id": "0".repeat(64), "node_id": node["node_id"],
            "node_sha256": digest(&node).expect("node digest"), "result": "PASS",
            "attempt": attempt, "artifacts": [artifact.clone()],
            "execution_binding": {
                "plan_id": plan["plan_id"], "execution_key": plan["execution_key"],
                "boundary": plan["boundary"], "roots": plan["environment_roots"],
                "execution_context": plan["execution_context"], "policy": plan["policy"],
                "claims": claims
            }
        });
        reseal_checkpoint(&mut checkpoint);
        let receipt = json!({
            "claims": {"workflow": "w", "job": "j", "runner": "r", "attempt": 1},
            "attempts": [checkpoint["attempt"].clone()], "artifacts": [artifact]
        });
        Self {
            root,
            plan,
            node,
            checkpoint,
            receipt,
        }
    }

    fn assert_error(
        &self,
        plan: &Value,
        node: &Value,
        checkpoint: &Value,
        receipt: &Value,
        code: &str,
    ) {
        let error = verify_checkpoint(plan, node, checkpoint, receipt, &self.root)
            .expect_err("mutated checkpoint must fail");
        assert_eq!(error.code, code);
    }
}

impl Drop for CheckpointFixture {
    fn drop(&mut self) {
        if self.root.is_dir() {
            fs::remove_dir_all(&self.root).expect("remove checkpoint fixture");
        }
    }
}

fn reseal_checkpoint(checkpoint: &mut Value) {
    checkpoint["checkpoint_id"] = json!("0".repeat(64));
    checkpoint["checkpoint_id"] =
        json!(derived_id(checkpoint, "checkpoint_id").expect("checkpoint ID"));
}

#[cfg(unix)]
const NATIVE_ATTESTATION_CHILD: &str = "resume::coverage_tests::native_attestation_scenario_child";

#[cfg(unix)]
#[test]
fn native_attestation_command_and_fail_closed_results_are_characterized() {
    use std::os::unix::fs::PermissionsExt;

    let scratch = std::env::temp_dir().join(format!(
        "openwepp-resume-attestation-{}",
        std::process::id()
    ));
    let bin = scratch.join("bin");
    let empty_bin = scratch.join("empty-bin");
    fs::create_dir_all(&bin).expect("fake gh directory");
    fs::create_dir_all(&empty_bin).expect("empty executable directory");
    let gh = bin.join("gh");
    fs::write(
        &gh,
        "#!/bin/sh\nprintf '%s\\n' \"$@\" > \"$OPENWEPP_ATTEST_ARGS\"\ncase \"$OPENWEPP_ATTEST_SCENARIO\" in\n  success) printf '[{}]\\n' ;;\n  empty) printf '[]\\n' ;;\n  malformed) printf 'not-json\\n' ;;\n  nonzero) printf '[{}]\\n'; printf 'denied\\n' >&2; exit 1 ;;\n  *) exit 2 ;;\nesac\n",
    )
    .expect("fake gh");
    let mut permissions = fs::metadata(&gh).expect("fake gh metadata").permissions();
    permissions.set_mode(0o755);
    fs::set_permissions(&gh, permissions).expect("executable fake gh");

    for scenario in ["spawn", "malformed", "nonzero", "empty", "success"] {
        let arguments = scratch.join(format!("{scenario}-arguments"));
        let path = if scenario == "spawn" {
            &empty_bin
        } else {
            &bin
        };
        let output = Command::new(std::env::current_exe().expect("current test executable"))
            .args([
                "--exact",
                NATIVE_ATTESTATION_CHILD,
                "--ignored",
                "--nocapture",
            ])
            .env("PATH", path)
            .env("OPENWEPP_ATTEST_SCENARIO", scenario)
            .env("OPENWEPP_ATTEST_ARGS", &arguments)
            .env("OPENWEPP_ATTEST_SCRATCH", &scratch)
            .output()
            .expect("run isolated attestation scenario");
        assert!(
            output.status.success(),
            "{scenario}: stdout={} stderr={}",
            String::from_utf8_lossy(&output.stdout),
            String::from_utf8_lossy(&output.stderr)
        );
        if scenario == "success" {
            let index = scratch.join("attempt-index.json");
            let bundle = scratch.join("attestation.jsonl");
            let actual = fs::read_to_string(&arguments).expect("captured gh arguments");
            let expected = [
                "attestation",
                "verify",
                index.to_str().expect("index argument"),
                "--repo",
                "owner/repo",
                "--signer-workflow",
                "owner/repo/.github/workflows/testgate-shadow.yml",
                "--source-ref",
                "refs/heads/main",
                "--source-digest",
                "0123456789012345678901234567890123456789",
                "--predicate-type",
                "https://openwepp.org/attestations/testgate-recovery/v1",
                "--deny-self-hosted-runners",
                "--bundle",
                bundle.to_str().expect("bundle argument"),
                "--format",
                "json",
            ]
            .join("\n")
                + "\n";
            assert_eq!(actual, expected);
        }
    }
    fs::remove_dir_all(scratch).expect("remove attestation fixture");
}

#[cfg(unix)]
#[test]
#[ignore = "isolated child invoked by native attestation characterization"]
fn native_attestation_scenario_child() {
    let scratch = std::env::var_os("OPENWEPP_ATTEST_SCRATCH")
        .map(PathBuf::from)
        .expect("scenario scratch");
    let scenario = std::env::var("OPENWEPP_ATTEST_SCENARIO").expect("scenario");
    let index = scratch.join("attempt-index.json");
    let bundle = scratch.join("attestation.jsonl");
    fs::write(&index, b"index").expect("index fixture");
    fs::write(&bundle, b"real bundle fixture").expect("bundle fixture");
    let predicate = json!({
        "source_ref": "refs/heads/main",
        "head_sha": "0123456789012345678901234567890123456789",
    });
    let result = verify_native_attestation(&index, &bundle, &predicate, "owner/repo");
    if scenario == "success" {
        result.expect("nonempty successful verification");
    } else {
        let error = result.expect_err("attestation scenario must fail closed");
        assert_eq!(error.code, "GATE-RESUME-PROVENANCE-VERIFY");
    }
}

#[test]
fn checkpoint_validation_preserves_guard_precedence() {
    let fixture = CheckpointFixture::new();
    verify_checkpoint(
        &fixture.plan,
        &fixture.node,
        &fixture.checkpoint,
        &fixture.receipt,
        &fixture.root,
    )
    .expect("valid checkpoint");

    for field in ["schema_version", "node_id", "node_sha256", "result"] {
        let mut checkpoint = fixture.checkpoint.clone();
        checkpoint[field] = json!("invalid");
        reseal_checkpoint(&mut checkpoint);
        fixture.assert_error(
            &fixture.plan,
            &fixture.node,
            &checkpoint,
            &fixture.receipt,
            "GATE-RESUME-CHECKPOINT-IDENTITY",
        );
    }
    let mut checkpoint = fixture.checkpoint.clone();
    checkpoint["checkpoint_id"] = json!("forged");
    checkpoint["execution_binding"]["boundary"] = json!("OTHER");
    fixture.assert_error(
        &fixture.plan,
        &fixture.node,
        &checkpoint,
        &fixture.receipt,
        "GATE-RESUME-CHECKPOINT-IDENTITY",
    );

    for pointer in [
        "/execution_binding/boundary",
        "/execution_binding/execution_context",
        "/execution_binding/policy",
        "/execution_binding/roots/execution_root",
        "/execution_binding/roots/authority_root",
        "/execution_binding/roots/assurance_root",
    ] {
        let mut checkpoint = fixture.checkpoint.clone();
        *checkpoint.pointer_mut(pointer).expect("binding field") = json!("drift");
        reseal_checkpoint(&mut checkpoint);
        fixture.assert_error(
            &fixture.plan,
            &fixture.node,
            &checkpoint,
            &fixture.receipt,
            "GATE-RESUME-CHECKPOINT-ROOT-DRIFT",
        );
    }

    for field in ["plan_id", "execution_key"] {
        let mut node = fixture.node.clone();
        node["reuse_class"] = json!("SAME_EXECUTION");
        let mut checkpoint = fixture.checkpoint.clone();
        checkpoint["node_sha256"] = json!(digest(&node).expect("same-execution node digest"));
        checkpoint["execution_binding"][field] = json!("drift");
        reseal_checkpoint(&mut checkpoint);
        fixture.assert_error(
            &fixture.plan,
            &node,
            &checkpoint,
            &fixture.receipt,
            "GATE-RESUME-CHECKPOINT-EXECUTION-DRIFT",
        );
    }

    for field in ["workflow", "job", "runner", "attempt"] {
        let mut checkpoint = fixture.checkpoint.clone();
        checkpoint["execution_binding"]["claims"][field] = json!("drift");
        reseal_checkpoint(&mut checkpoint);
        fixture.assert_error(
            &fixture.plan,
            &fixture.node,
            &checkpoint,
            &fixture.receipt,
            "GATE-RESUME-CHECKPOINT-RECEIPT-MISMATCH",
        );
    }
    for attempts in [json!([]), json!({}), json!([{"result": "FAIL"}])] {
        let mut receipt = fixture.receipt.clone();
        receipt["attempts"] = attempts;
        fixture.assert_error(
            &fixture.plan,
            &fixture.node,
            &fixture.checkpoint,
            &receipt,
            "GATE-RESUME-CHECKPOINT-RECEIPT-MISMATCH",
        );
    }
}

#[test]
fn checkpoint_artifact_validation_preserves_order() {
    let fixture = CheckpointFixture::new();

    let mut checkpoint = fixture.checkpoint.clone();
    checkpoint["artifacts"] = json!({});
    reseal_checkpoint(&mut checkpoint);
    fixture.assert_error(
        &fixture.plan,
        &fixture.node,
        &checkpoint,
        &fixture.receipt,
        "GATE-RESUME-CHECKPOINT-SHAPE",
    );

    let mut checkpoint = fixture.checkpoint.clone();
    checkpoint["artifacts"][0]["path"] = json!(1);
    reseal_checkpoint(&mut checkpoint);
    fixture.assert_error(
        &fixture.plan,
        &fixture.node,
        &checkpoint,
        &fixture.receipt,
        "GATE-RESUME-SHAPE",
    );

    fs::remove_file(fixture.root.join("out/result")).expect("remove source artifact");
    fixture.assert_error(
        &fixture.plan,
        &fixture.node,
        &fixture.checkpoint,
        &fixture.receipt,
        "GATE-RESUME-ARTIFACT",
    );
    fs::write(fixture.root.join("out/result"), b"changed").expect("changed artifact");
    fixture.assert_error(
        &fixture.plan,
        &fixture.node,
        &fixture.checkpoint,
        &fixture.receipt,
        "GATE-RESUME-ARTIFACT-DIGEST",
    );
    fs::write(fixture.root.join("out/result"), b"pass").expect("restore artifact");

    for artifacts in [json!([]), json!({})] {
        let mut receipt = fixture.receipt.clone();
        receipt["artifacts"] = artifacts;
        fixture.assert_error(
            &fixture.plan,
            &fixture.node,
            &fixture.checkpoint,
            &receipt,
            "GATE-RESUME-CHECKPOINT-RECEIPT-MISMATCH",
        );
    }

    let mut checkpoint = fixture.checkpoint.clone();
    checkpoint["artifacts"]
        .as_array_mut()
        .expect("checkpoint artifacts")
        .push(json!({"path": "out/missing", "sha256": sha256_bytes(b"missing")}));
    reseal_checkpoint(&mut checkpoint);
    fixture.assert_error(
        &fixture.plan,
        &fixture.node,
        &checkpoint,
        &fixture.receipt,
        "GATE-RESUME-ARTIFACT",
    );

    fs::write(fixture.root.join("out/second"), b"second").expect("second artifact");
    let mut checkpoint = fixture.checkpoint.clone();
    checkpoint["artifacts"]
        .as_array_mut()
        .expect("checkpoint artifacts")
        .push(json!({"path": "out/second", "sha256": sha256_bytes(b"wrong")}));
    reseal_checkpoint(&mut checkpoint);
    fixture.assert_error(
        &fixture.plan,
        &fixture.node,
        &checkpoint,
        &fixture.receipt,
        "GATE-RESUME-ARTIFACT-DIGEST",
    );
}

#[test]
fn candidate_discovery_preserves_initial_error_and_reverse_record_precedence() {
    let scratch = std::env::temp_dir().join(format!(
        "openwepp-resume-candidate-precedence-{}",
        std::process::id()
    ));
    let history = scratch.join("history");
    fs::create_dir_all(&history).expect("history");
    let ledger = history.join("attempts.jsonl");
    let malformed_plan = json!({"nodes": {}});
    let error = load_candidate_internal(
        &scratch,
        &malformed_plan,
        &ledger,
        &ExecutionClaims::default(),
        false,
        None,
    )
    .err()
    .expect("ledger read precedes plan shape");
    assert_eq!(error.code, "GATE-RESUME-LEDGER");

    fs::write(&ledger, "").expect("empty ledger");
    let error = load_candidate_internal(
        &scratch,
        &malformed_plan,
        &ledger,
        &ExecutionClaims::default(),
        false,
        None,
    )
    .err()
    .expect("plan shape follows ledger parsing");
    assert_eq!(error.code, "GATE-RESUME-PLAN-SHAPE");

    let older = json!({
        "record_type": "STAGE_ATTEMPT",
        "artifact_root": scratch.join("ordinary-current-attempt")
    });
    let newest_root = scratch.join("newest-explicit-invalid");
    let newest = json!({"record_type": "STAGE_ATTEMPT", "recovery_root": newest_root});
    fs::write(
        &ledger,
        format!(
            "{}\n{}\n",
            serde_json::to_string(&older).expect("older record"),
            serde_json::to_string(&newest).expect("newest record")
        ),
    )
    .expect("ordered ledger");
    let error = load_candidate_internal(
        &scratch,
        &json!({"nodes": []}),
        &ledger,
        &ExecutionClaims::default(),
        false,
        None,
    )
    .err()
    .expect("newest explicit recovery root fails closed first");
    assert_eq!(error.code, "GATE-RESUME-PROVENANCE-PATH");
    assert!(error.message.contains("newest-explicit-invalid"));
    fs::remove_dir_all(scratch).expect("remove candidate fixture");
}

#[test]
fn receipt_loading_and_envelope_branch_selection_fail_closed() {
    let owned_scratch = OwnedScratch::new("receipt-characterization");
    let scratch = owned_scratch.path();
    let artifact_root = scratch.join("execution");
    fs::create_dir_all(&artifact_root).expect("artifact root");
    let mut plan = json!({
        "plan_id": "0".repeat(64), "execution_key": "0".repeat(64),
        "boundary": "INCREMENT", "source": {"head_commit": "a".repeat(40)},
        "environment_roots": {
            "execution_root": "e", "authority_root": "a", "assurance_root": "s"
        },
        "execution_context": {"tool": "fixed"}, "policy": {"generation": 1},
        "nodes": [],
    });
    plan["plan_id"] = json!(crate::planner::derive_plan_id(&plan).expect("prior plan identity"));
    plan["execution_key"] =
        json!(crate::planner::derive_execution_key(&plan).expect("execution identity"));
    fs::write(
        scratch.join("plan.json"),
        canonical_bytes(&plan).expect("plan bytes"),
    )
    .expect("prior plan");
    let archive = RecoveryArchive {
        root: scratch.to_path_buf(),
        artifact_root,
        provenance: json!({
            "workflow": "testgate", "run_attempt": "1", "index_sha256": "f".repeat(64)
        }),
    };
    let receipt_path = scratch.join("receipt.json");

    let error = load_accepted_receipt(scratch, &plan, &archive, &plan, &receipt_path, false)
        .expect_err("missing receipt must fail");
    assert_eq!(error.code, "GATE-RESUME-RECEIPT");

    fs::write(&receipt_path, b"not-json").expect("malformed receipt");
    let error = load_accepted_receipt(scratch, &plan, &archive, &plan, &receipt_path, false)
        .expect_err("malformed receipt must fail strict parsing");
    assert_eq!(error.code, "GATE-JSON-INVALID");

    fs::write(&receipt_path, b"{}").expect("invalid receipt");
    for admitted in [false, true] {
        let error = load_accepted_receipt(scratch, &plan, &archive, &plan, &receipt_path, admitted)
            .expect_err("invalid receipt must fail after verifier selection");
        assert_eq!(error.code, "GATE-RESUME-RECEIPT-INVALID");
    }

    fs::remove_file(&receipt_path).expect("remove invalid receipt");
    let envelope =
        load_recovery_envelope(scratch, &plan, &archive, false).expect("receipt-absent envelope");
    assert!(envelope.accepted_receipt.is_none());
    fs::write(&receipt_path, b"{}").expect("restore invalid receipt");
    let error = load_recovery_envelope(scratch, &plan, &archive, true)
        .err()
        .expect("receipt-present envelope must invoke verifier");
    assert_eq!(error.code, "GATE-RESUME-RECEIPT-INVALID");
}

#[test]
fn reuse_decision_matrix_is_characterized() {
    let pass = json!({"result": "PASS"});
    let fail = json!({"result": "FAIL"});
    let claims = ExecutionClaims {
        workflow: "workflow".to_owned(),
        job: "job".to_owned(),
        runner: "runner".to_owned(),
        attempt: 1,
        ..ExecutionClaims::default()
    };
    assert_eq!(
        reuse_reason(&json!({}), None, &json!({}), &claims).expect("no prior receipt"),
        "NO_PRIOR_NODE_RECEIPT"
    );
    assert_eq!(
        reuse_reason(&json!({}), Some(&fail), &json!({}), &claims).expect("failed prior"),
        "PRIOR_NODE_NONPASS"
    );
    assert_eq!(
        reuse_reason(
            &json!({"reuse_class": "HERMETIC_CONTENT"}),
            Some(&pass),
            &json!({}),
            &claims,
        )
        .expect("hermetic reuse"),
        "IMPORTED_CURRENT_PASS"
    );
    for (prior, expected) in [
        (
            json!({"workflow": "other", "job": "job", "runner": "runner", "attempt": 1}),
            "SAME_EXECUTION_WORKFLOW_MISMATCH",
        ),
        (
            json!({"workflow": "workflow", "job": "other", "runner": "runner", "attempt": 1}),
            "SAME_EXECUTION_JOB_MISMATCH",
        ),
        (
            json!({"workflow": "workflow", "job": "job", "runner": "runner", "attempt": 1}),
            "IMPORTED_CURRENT_PASS",
        ),
    ] {
        assert_eq!(
            reuse_reason(
                &json!({"reuse_class": "SAME_EXECUTION"}),
                Some(&pass),
                &prior,
                &claims,
            )
            .expect("same-execution decision"),
            expected
        );
    }
    let error = reuse_reason(
        &json!({"reuse_class": "UNKNOWN"}),
        Some(&pass),
        &json!({}),
        &claims,
    )
    .expect_err("unknown reuse class must fail");
    assert_eq!(error.code, "GATE-RESUME-CLASS");
}
