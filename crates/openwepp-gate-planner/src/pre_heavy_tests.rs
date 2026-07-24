use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;
use std::sync::atomic::{AtomicU64, Ordering};

use serde_json::json;

use super::{
    AuditCheckInputs, AuditDocumentInputs, CHECK_IDS, ConstructedAudit, admit_attempt_ledger,
    append_attempt_record, audit_document, audit_reason_codes, audit_reconstruction_root,
    audit_status, build_audit_checks, build_failure_audit, check, close_tooling_defect,
    construct_audit, documentation_scope_is_exact, durable_ledger,
    enforce_authorized_rust_line_limit, execution_claims_match, execution_context_is_current,
    execution_identities, failure_check_index, file_digest, ledger_head, light_attempt_isolated,
    light_stage_passed, no_open_tooling_defect, no_open_tooling_defect_at_head, node_manifest,
    package_admission, package_admitted, path_digest, read_json, reconcile_orphaned_attempts,
    reconstructed_plan_is_exact, record_heavy_failure, reject_open_tooling_defects,
    require_bound_active_prompt, require_clean_diff, require_ready_audit_status, seal_audit,
    separated_roots, tooling_defect_statuses, valid_stage_order, validate_audit_artifact_fields,
    validate_audit_context_binding, validate_audit_core_binding, validate_audit_schema,
    validate_checkpoint_identity, validate_current_audit_inventory,
    validate_embedded_light_receipt_id, validate_exact_node_shapes, validate_quality_deferral,
    validate_ready_audit, validate_ready_check_set, validate_relocated_artifact_binding,
    validate_relocated_light_receipt, validate_stage_receipt,
    validate_stage_receipt_execution_binding, validate_stage_receipt_plan_binding,
    validate_started_successor, verify_ledger_chain, with_disposable_audit_reconstruction,
};
use crate::canonical::{derived_id, digest, parse_strict, validate_schema};
use crate::error::{ErrorClass, GatePolicyError};
use crate::executor::ExecutionClaims;

#[test]
fn audit_inventory_uses_a_disposable_target_distinct_from_execution() {
    let artifacts = std::env::temp_dir().join(format!(
        "openwepp-audit-disposable-{}-{}",
        std::process::id(),
        std::thread::current().name().unwrap_or("test")
    ));
    let audit_root = audit_reconstruction_root(&artifacts);
    assert_eq!(audit_root, artifacts.join(".work/audit-reconstruction"));
    assert_ne!(audit_root, artifacts.join(".work/cargo-target"));
    let value = with_disposable_audit_reconstruction(&artifacts, |root| {
        fs::create_dir_all(root).expect("create audit workspace");
        fs::write(root.join("compiled-test"), b"snapshot-bound").expect("write cache marker");
        Ok(7)
    })
    .expect("successful reconstruction");
    assert_eq!(value, 7);
    assert!(!audit_root.exists());

    let error = with_disposable_audit_reconstruction(&artifacts, |root| {
        fs::create_dir_all(root).expect("create failed audit workspace");
        fs::write(root.join("compiled-test"), b"snapshot-bound").expect("write cache marker");
        Err::<(), _>(GatePolicyError::new(
            ErrorClass::Execution,
            "GATE-AUDIT-TEST-FAILURE",
            "injected reconstruction failure",
        ))
    })
    .expect_err("reconstruction failure must be retained");
    assert_eq!(error.code, "GATE-AUDIT-TEST-FAILURE");
    assert!(!audit_root.exists());
    fs::remove_dir_all(artifacts).expect("remove fixture");
}

#[test]
fn audit_schema_rejects_duplicate_canonical_check_ids() {
    let root = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../..");
    let schema = read_json(&root.join("gate-policy/v1/schemas/pre-heavy-audit.schema.json"))
        .expect("audit schema");
    let mut audit = read_json(&root.join("gate-policy/v1/fixtures/valid/pre-heavy-audit.json"))
        .expect("valid audit fixture");
    validate_schema(&schema, &audit, "valid audit").expect("valid audit must pass schema");
    audit["checks"][1]["check_id"] = audit["checks"][0]["check_id"].clone();
    assert!(validate_schema(&schema, &audit, "duplicate audit").is_err());
}

#[test]
fn rust_verifies_python_jcs_with_adversarial_unicode_keys() {
    let root = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../..");
    verify_ledger_chain(&root.join("tests/fixtures/testgate/python-ledger-unicode.jsonl"))
        .expect("Python-produced ledger must share Rust RFC 8785 ordering");
}

#[test]
fn recurring_cause_opens_a_blocking_tooling_defect() {
    let path = std::env::temp_dir().join(format!(
        "openwepp-gate-recurrence-{}-{}.jsonl",
        std::process::id(),
        std::thread::current().name().unwrap_or("test")
    ));
    fs::write(&path, "").expect("empty ledger");
    for _ in 0..2 {
        record_heavy_failure(
            &path,
            json!({
                "record_type": "STAGE_ATTEMPT",
                "status": "FAILED",
                "cause_key": "GATE-EXEC-SPAWN",
            }),
            "GATE-EXEC-SPAWN",
        )
        .expect("record failure");
    }
    let text = fs::read_to_string(&path).expect("ledger");
    assert!(text.contains("SAME_CAUSE_RECURRED_AFTER_ONE_RETRY"));
    assert!(text.contains("\"status\":\"OPEN\""));
    fs::remove_file(path).expect("remove ledger");
}

#[test]
fn documentation_scope_is_exact_sorted_and_excludes_deletions() {
    let plan = json!({
        "changed_objects": [
            {"path": "docs/a.md", "change_kind": "MODIFY"},
            {"path": "docs/deleted.md", "change_kind": "DELETE"},
            {"path": "README.MD", "change_kind": "ADD"},
            {"path": "docs/schema.json", "change_kind": "MODIFY"}
        ],
        "nodes": [{
            "gate_definition_id": "documentation-lint-v1",
            "arguments": [
                "markdown-doc", "lint", "--path", "README.MD", "--path", "docs/a.md"
            ]
        }]
    });
    documentation_scope_is_exact(&plan).expect("exact changed Markdown scope");
    for arguments in [
        json!(["markdown-doc", "lint", "--path", "docs/a.md"]),
        json!([
            "markdown-doc",
            "lint",
            "--path",
            "docs/a.md",
            "--path",
            "README.MD"
        ]),
        json!([
            "markdown-doc",
            "lint",
            "--path",
            "README.MD",
            "--path",
            "docs/deleted.md"
        ]),
    ] {
        let mut drifted = plan.clone();
        drifted["nodes"][0]["arguments"] = arguments;
        assert!(documentation_scope_is_exact(&drifted).is_err());
    }
}

#[test]
fn independently_reconstructed_plan_must_match_all_identity_fields() {
    let plan = json!({
        "execution_context": {"configuration_sha256": "original"},
        "nodes": [{
        "node_id": "a", "arguments": ["cargo", "nextest", "run"],
        "expected_inventory": {"mode": "EXACT", "ids": ["one"]}
    }]});
    reconstructed_plan_is_exact(&plan, &plan).expect("exact reconstruction");
    for pointer in [
        "/execution_context/configuration_sha256",
        "/nodes/0/arguments/2",
        "/nodes/0/expected_inventory/ids/0",
    ] {
        let mut drifted = plan.clone();
        *drifted.pointer_mut(pointer).expect("mutation pointer") = json!("drift");
        assert!(reconstructed_plan_is_exact(&plan, &drifted).is_err());
    }
}

#[test]
fn heavy_admission_rejects_every_execution_context_identity_breaker() {
    let context = json!({
        "environment_manifest_sha256": "environment",
        "runner_host_class": "runner",
        "runner_image_sha256": "image",
        "fixture_manifest_sha256": "fixtures",
        "tool_manifest_sha256": "tools",
        "configuration_sha256": "configuration"
    });
    let plan = json!({"execution_context": context});
    execution_context_is_current(&plan, &plan["execution_context"]).expect("unchanged context");
    for field in [
        "environment_manifest_sha256",
        "runner_host_class",
        "runner_image_sha256",
        "fixture_manifest_sha256",
        "tool_manifest_sha256",
        "configuration_sha256",
    ] {
        let mut drifted = plan["execution_context"].clone();
        drifted[field] = json!("drift");
        assert!(execution_context_is_current(&plan, &drifted).is_err());
    }
}

#[test]
fn heavy_started_must_be_the_exact_successor_of_the_audited_ledger_head() {
    let path = std::env::temp_dir().join(format!(
        "openwepp-gate-started-successor-{}-{}.jsonl",
        std::process::id(),
        std::thread::current().name().unwrap_or("test")
    ));
    fs::write(&path, "").expect("empty ledger");
    let head = append_attempt_record(
        &path,
        json!({"record_type": "STAGE_ATTEMPT", "status": "CLOSED", "stage": "LIGHT"}),
    )
    .expect("audited ledger head");
    let plan = json!({"plan_id": "1".repeat(64)});
    let audit = json!({"audit_id": "2".repeat(64), "ledger_head_sha256": head});
    let artifacts = PathBuf::from("/external/evidence");
    let claims = ExecutionClaims {
        workflow: "workflow".to_owned(),
        job: "job".to_owned(),
        runner: "runner".to_owned(),
        attempt: 1,
        ..ExecutionClaims::default()
    };
    let started = append_attempt_record(
        &path,
        json!({
            "record_type": "STAGE_ATTEMPT", "status": "STARTED", "stage": "HEAVY",
            "phase": "ADMISSION", "plan_id": plan["plan_id"], "audit_id": audit["audit_id"],
            "artifact_root": artifacts.display().to_string(), "workflow": claims.workflow,
            "job": claims.job, "runner": claims.runner, "attempt": claims.attempt,
        }),
    )
    .expect("started successor");
    validate_started_successor(&plan, &audit, &artifacts, &path, &started, &claims)
        .expect("exact successor");
    append_attempt_record(&path, json!({"record_type": "ATTEMPT", "status": "CLOSED"}))
        .expect("intervening record");
    assert!(
        validate_started_successor(&plan, &audit, &artifacts, &path, &started, &claims).is_err()
    );
    fs::remove_file(path).expect("remove ledger");
}

#[test]
fn orphaned_admission_is_closed_once_and_recurrence_opens_defect() {
    let path = std::env::temp_dir().join(format!(
        "openwepp-gate-orphan-{}-{}.jsonl",
        std::process::id(),
        std::thread::current().name().unwrap_or("test")
    ));
    fs::write(&path, "").expect("empty ledger");
    for attempt in 1..=2 {
        append_attempt_record(
            &path,
            json!({
                "record_type": "STAGE_ATTEMPT", "status": "STARTED",
                "stage": "HEAVY", "phase": "ADMISSION", "attempt": attempt,
                "plan_id": "1".repeat(64), "audit_id": "2".repeat(64),
                "artifact_root": "/external/e", "recovery_root": "/history/recovery/r",
                "workflow": "w", "job": "j", "runner": "r",
            }),
        )
        .expect("started");
        assert_eq!(reconcile_orphaned_attempts(&path).expect("reconcile"), 1);
        assert_eq!(reconcile_orphaned_attempts(&path).expect("idempotent"), 0);
    }
    let text = fs::read_to_string(&path).expect("ledger");
    assert_eq!(
        text.lines()
            .map(|line| parse_strict(line.as_bytes()).expect("record"))
            .filter(|item| item["status"] == "FAILED")
            .count(),
        2
    );
    assert!(text.contains("SAME_CAUSE_RECURRED_AFTER_ONE_RETRY"));
    fs::remove_file(path).expect("remove ledger");
}

#[test]
fn representable_early_failure_emits_ten_check_invalid_audit() {
    let root = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../..");
    let failure = GatePolicyError::new(
        ErrorClass::Identity,
        "GATE-PLAN-IDENTITY",
        "injected identity failure",
    );
    let audit = build_failure_audit(
        &root,
        &json!({}),
        &json!({}),
        &root,
        &root.join("target/test-ledger.jsonl"),
        &failure,
    )
    .expect("invalid audit");
    assert_eq!(audit["status"], "INVALID");
    assert_eq!(audit["checks"].as_array().map(Vec::len), Some(10));
    assert_eq!(audit["reason_codes"], json!(["GATE-PLAN-IDENTITY"]));
    assert_eq!(audit["checks"][2]["status"], "INVALID");

    let blocked = GatePolicyError::new(
        ErrorClass::Io,
        "GATE-AUDIT-LEDGER-MISSING",
        "durable ledger unavailable",
    );
    let audit = build_failure_audit(
        &root,
        &json!({}),
        &json!({}),
        &root,
        &root.join("target/test-ledger.jsonl"),
        &blocked,
    )
    .expect("blocked audit");
    assert_eq!(audit["status"], "BLOCKED");
    assert_eq!(audit["checks"][8]["status"], "BLOCKED");
    assert_eq!(
        audit["checks"][8]["reason_codes"],
        json!(["GATE-AUDIT-LEDGER-MISSING"])
    );

    let malformed = build_failure_audit(
        &root,
        &json!({"plan_id": "z".repeat(64), "execution_key": "Z".repeat(64)}),
        &json!({"stage_receipt_id": "g".repeat(64)}),
        &root,
        &root.join("target/test-ledger.jsonl"),
        &failure,
    )
    .expect("malformed identities still yield schema-valid audit");
    for field in ["plan_id", "execution_key", "light_stage_receipt_id"] {
        let value = malformed[field].as_str().expect("digest field");
        assert_eq!(value.len(), 64);
        assert!(value.bytes().all(|byte| byte.is_ascii_hexdigit()));
        assert_eq!(value, value.to_ascii_lowercase());
    }
}

#[test]
fn rejected_package_admission_is_an_identity_failure() {
    let error = package_admitted(
        &json!({
            "authorized_paths": ["docs/work-packages/p/package.md"],
            "source": {"base_commit": "base"},
        }),
        &json!({
            "status": "INVALID", "changed_paths": [], "base_commit": "base",
            "reason_codes": ["PACKAGE-UNDECLARED-PATH"],
        }),
    )
    .expect_err("authority substitution is invalid");
    assert_eq!(error.class, ErrorClass::Identity);
    assert_eq!(error.code, "GATE-AUDIT-PACKAGE-ADMISSION");
}

#[test]
fn missing_ledger_is_reported_by_both_owning_checks_without_escape() {
    let path = std::env::temp_dir().join(format!(
        "openwepp-gate-missing-ledger-{}-{}.jsonl",
        std::process::id(),
        std::thread::current().name().unwrap_or("test")
    ));
    let durable = check(
        "DURABLE_ATTEMPT_LEDGER",
        durable_ledger(&path),
        json!({"ledger_path": path.display().to_string()}),
    )
    .expect("durable check artifact");
    let defects = check(
        "OPEN_TOOLING_DEFECTS",
        no_open_tooling_defect(&path),
        json!({"ledger_path": path.display().to_string(), "ledger_sha256": null}),
    )
    .expect("defect check artifact");
    assert_eq!(durable["status"], "BLOCKED");
    assert_eq!(
        durable["reason_codes"],
        json!(["GATE-AUDIT-LEDGER-MISSING"])
    );
    assert_eq!(defects["status"], "BLOCKED");
    assert_eq!(defects["reason_codes"], json!(["GATE-AUDIT-LEDGER-READ"]));
}

#[test]
fn tooling_defect_ledger_uses_the_last_status_for_each_defect() {
    let path = std::env::temp_dir().join(format!(
        "openwepp-gate-tooling-defect-{}-{}.jsonl",
        std::process::id(),
        std::thread::current().name().unwrap_or("test")
    ));
    fs::write(&path, "").expect("empty ledger");
    append_attempt_record(
        &path,
        json!({"record_type": "TOOLING_DEFECT", "defect_id": "RTR-OPEN", "status": "OPEN"}),
    )
    .expect("open defect");
    assert_eq!(
        no_open_tooling_defect(&path)
            .expect_err("open defect blocks")
            .code,
        "GATE-AUDIT-OPEN-TOOLING-DEFECT"
    );
    append_attempt_record(
        &path,
        json!({"record_type": "TOOLING_DEFECT", "defect_id": "RTR-OPEN", "status": "CLOSED"}),
    )
    .expect("close defect");
    no_open_tooling_defect(&path).expect("latest CLOSED status admits");
    append_attempt_record(
        &path,
        json!({"record_type": "TOOLING_DEFECT", "defect_id": "RTR-OPEN", "status": "CLOZED"}),
    )
    .expect("malformed status");
    assert_eq!(
        no_open_tooling_defect(&path)
            .expect_err("malformed status must fail closed")
            .code,
        "GATE-AUDIT-TOOLING-DEFECT-SHAPE"
    );
    fs::remove_file(path).expect("remove ledger");
}

fn assert_invalid_closures_fail(ledger: &Path, history: &Path, recovery: &Path) {
    assert_eq!(
        close_tooling_defect(
            ledger,
            "AUTO-example",
            &"a".repeat(40),
            "dual review passed",
            Some(&history.join("recovery/..")),
        )
        .expect_err("dot-dot recovery root must fail")
        .code,
        "GATE-AUDIT-DEFECT-CLOSURE-PATH"
    );
    assert_eq!(
        close_tooling_defect(
            ledger,
            "AUTO-example",
            &"a".repeat(40),
            "dual review passed",
            Some(&history.join("outside")),
        )
        .expect_err("outside recovery root must fail")
        .code,
        "GATE-AUDIT-DEFECT-CLOSURE-PATH"
    );
    assert_eq!(
        close_tooling_defect(
            ledger,
            "AUTO-example",
            &"a".repeat(40),
            "dual review passed",
            Some(&history.join("recovery/unassociated")),
        )
        .expect_err("unassociated recovery root must fail")
        .code,
        "GATE-AUDIT-DEFECT-CLOSURE-UNASSOCIATED"
    );
    assert_eq!(
        close_tooling_defect(
            ledger,
            "AUTO-example",
            &"a".repeat(40),
            " \t ",
            Some(recovery),
        )
        .expect_err("blank review evidence must fail")
        .code,
        "GATE-AUDIT-DEFECT-CLOSURE-SHAPE"
    );
}

#[test]
fn tooling_defect_closure_command_binds_review_and_exact_recovery_root() {
    let repo = fs::canonicalize(
        PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .parent()
            .and_then(Path::parent)
            .expect("repository root"),
    )
    .expect("canonical repository");
    let root = repo.join(format!(
        "target/openwepp-gate-tooling-closure-{}-{}",
        std::process::id(),
        std::thread::current().name().unwrap_or("test")
    ));
    let history = root.join("history");
    fs::create_dir_all(history.join("recovery")).expect("history");
    let ledger = history.join("attempts.jsonl");
    fs::write(&ledger, "").expect("empty ledger");
    let recovery = history.join("recovery/failed");
    append_attempt_record(
        &ledger,
        json!({
            "record_type": "STAGE_ATTEMPT",
            "stage": "HEAVY",
            "status": "FAILED",
            "cause_key": "GATE-EXAMPLE",
            "recovery_root": recovery,
        }),
    )
    .expect("failed attempt");
    append_attempt_record(
        &ledger,
        json!({
            "record_type": "TOOLING_DEFECT",
            "defect_id": "AUTO-example",
            "status": "OPEN",
            "cause_key": "GATE-EXAMPLE",
        }),
    )
    .expect("open defect");
    assert_invalid_closures_fail(&ledger, &history, &recovery);
    let entry = close_tooling_defect(
        &ledger,
        "AUTO-example",
        &"a".repeat(40),
        "dual review passed",
        Some(&recovery),
    )
    .expect("close exact defect");
    assert_eq!(entry.len(), 64);
    no_open_tooling_defect(&ledger).expect("closure admits audit");
    let closed = fs::read_to_string(&ledger)
        .expect("closed ledger")
        .lines()
        .map(|line| parse_strict(line.as_bytes()).expect("canonical record"))
        .collect::<Vec<_>>();
    assert_eq!(
        closed.last().expect("closure")["invalidated_recovery_root"],
        recovery.display().to_string()
    );
    assert_eq!(
        close_tooling_defect(&ledger, "AUTO-example", &"a".repeat(40), "duplicate", None,)
            .expect_err("duplicate closure must fail")
            .code,
        "GATE-AUDIT-DEFECT-CLOSURE-NOT-OPEN"
    );
    fs::remove_dir_all(root).expect("remove scratch");
}

#[cfg(unix)]
#[test]
fn tooling_defect_closure_rejects_symlinked_ledger_without_mutation() {
    use std::os::unix::fs::symlink;

    let repo = fs::canonicalize(
        PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .parent()
            .and_then(Path::parent)
            .expect("repository root"),
    )
    .expect("canonical repository");
    let root = repo.join(format!(
        "target/openwepp-gate-tooling-symlink-{}",
        std::process::id()
    ));
    let history = root.join("history");
    let outside = root.join("outside");
    fs::create_dir_all(history.join("recovery")).expect("history");
    fs::create_dir_all(&outside).expect("outside");
    let target = outside.join("attempts.jsonl");
    fs::write(&target, "").expect("outside ledger");
    append_attempt_record(
        &target,
        json!({
            "record_type": "TOOLING_DEFECT",
            "defect_id": "AUTO-link",
            "status": "OPEN",
        }),
    )
    .expect("open defect");
    let before = fs::read(&target).expect("before");
    let link = history.join("attempts.jsonl");
    symlink(&target, &link).expect("ledger symlink");
    assert_eq!(
        close_tooling_defect(&link, "AUTO-link", &"a".repeat(40), "reviewed", None)
            .expect_err("symlinked ledger must fail")
            .code,
        "GATE-AUDIT-LEDGER-PATH"
    );
    assert_eq!(fs::read(&target).expect("after"), before);
    let ancestor = root.join("history-link");
    symlink(&outside, &ancestor).expect("ancestor symlink");
    assert_eq!(
        close_tooling_defect(
            &ancestor.join("attempts.jsonl"),
            "AUTO-link",
            &"a".repeat(40),
            "reviewed",
            None,
        )
        .expect_err("symlinked ancestor must fail")
        .code,
        "GATE-AUDIT-LEDGER-PATH"
    );
    assert_eq!(fs::read(&target).expect("ancestor after"), before);
    fs::remove_dir_all(root).expect("remove scratch");
}

#[test]
fn every_light_heavy_execution_claim_must_match() {
    let light = json!({"workflow": "w", "job": "j", "runner": "r", "attempt": 1});
    let baseline = ExecutionClaims {
        workflow: "w".to_owned(),
        job: "j".to_owned(),
        runner: "r".to_owned(),
        attempt: 1,
        ..ExecutionClaims::default()
    };
    assert!(execution_claims_match(&light, &baseline));
    for field in ["workflow", "job", "runner", "attempt"] {
        let mut mutated = light.clone();
        mutated[field] = if field == "attempt" {
            json!(2)
        } else {
            json!("other")
        };
        assert!(!execution_claims_match(&mutated, &baseline), "{field}");
    }
}

#[test]
fn failure_check_index_preserves_first_matching_token_precedence() {
    for (code, expected) in [
        ("GATE-PACKAGE-LIGHT", 0),
        ("GATE-LIGHT-DOC", 1),
        ("GATE-INVENTORY-PLAN", 2),
        ("GATE-EXECUTION-CLAIM", 3),
        ("GATE-ARTIFACT-CHECKPOINT", 4),
        ("GATE-ROOT-CACHE", 5),
        ("GATE-QUALITY-DISPOSITION", 6),
        ("GATE-ORDER-RETRY", 7),
        ("GATE-LEDGER-UNKNOWN", 8),
        ("GATE-UNKNOWN", 9),
    ] {
        assert_eq!(failure_check_index(code), expected, "{code}");
    }
}

#[test]
fn light_stage_and_stage_order_reject_nonpass_or_forward_dependency() {
    let mut plan = json!({
        "nodes": [
            {"node_id": "light", "execution_cost_class": "LIGHT", "prerequisites": []},
            {"node_id": "heavy", "execution_cost_class": "HEAVY", "prerequisites": ["light"]}
        ]
    });
    let receipt = json!({"final_results": {"light": "PASS"}});
    light_stage_passed(&plan, &receipt).expect("passing LIGHT receipt");
    valid_stage_order(&plan).expect("ordered LIGHT then HEAVY");

    plan["nodes"][1]["prerequisites"] = json!(["missing"]);
    assert_eq!(
        valid_stage_order(&plan)
            .expect_err("forward dependency")
            .code,
        "GATE-AUDIT-PREREQUISITE-ORDER"
    );
    assert_eq!(
        light_stage_passed(&plan, &json!({"final_results": {"light": "FAIL"}}))
            .expect_err("LIGHT failure")
            .code,
        "GATE-AUDIT-LIGHT-NONPASS"
    );
}

#[test]
fn quality_deferral_requires_its_exact_dag_shape() {
    let plan = json!({
        "nodes": [{"gate_definition_id": "workspace-full-nextest-v1"}],
        "quality_disposition": super::expected_quality_disposition()
    });
    validate_quality_deferral(&plan).expect("deferred quality DAG");

    let mut drifted = plan.clone();
    drifted["nodes"][0]["gate_definition_id"] = json!("adjudicated-crap-v1");
    assert_eq!(
        validate_quality_deferral(&drifted)
            .expect_err("retired quality node")
            .code,
        "GATE-AUDIT-QUALITY-NODE-PROHIBITED"
    );
}

#[test]
fn execution_identity_requires_all_bound_digests_and_claims() {
    let digest = "a".repeat(64);
    let plan = json!({"execution_context": {
        "configuration_sha256": digest,
        "environment_manifest_sha256": "b".repeat(64),
        "fixture_manifest_sha256": "c".repeat(64),
        "tool_manifest_sha256": "d".repeat(64)
    }});
    let receipt = json!({
        "claims": {
            "principal": "principal", "repository": "repository",
            "source_event": "event", "source_ref": "ref", "workflow": "workflow",
            "job": "job", "runner": "runner", "attempt": 1
        },
        "executor_binary_sha256": "e".repeat(64)
    });
    execution_identities(&plan, &receipt).expect("complete execution identity");

    let mut malformed = receipt.clone();
    malformed["claims"]["attempt"] = json!(0);
    assert_eq!(
        execution_identities(&plan, &malformed)
            .expect_err("zero attempt")
            .code,
        "GATE-AUDIT-EXECUTION-CLAIM"
    );
}

#[test]
fn extracted_audit_bindings_preserve_exact_identity_checks() {
    let plan = json!({
        "plan_id": "plan", "execution_key": "execution",
        "quality_disposition": super::expected_quality_disposition(), "nodes": []
    });
    let admission = json!({"status": "READY"});
    let artifact_root = PathBuf::from("/external/audit-root");
    let mut audit = json!({
        "audit_id": "0".repeat(64), "plan_id": plan["plan_id"],
        "plan_sha256": digest(&plan).expect("plan digest"),
        "execution_key": plan["execution_key"],
        "artifact_root_sha256": path_digest(&artifact_root),
        "node_manifest": node_manifest(&plan).expect("node manifest"),
        "quality_disposition": plan["quality_disposition"],
        "package_admission": admission
    });
    audit["audit_id"] = json!(derived_id(&audit, "audit_id").expect("audit ID"));
    validate_audit_core_binding(&plan, &audit).expect("core binding");
    validate_audit_context_binding(&plan, &audit, &artifact_root, &admission)
        .expect("context binding");
    let mut drifted = audit;
    drifted["execution_key"] = json!("drift");
    assert_eq!(
        validate_audit_core_binding(&plan, &drifted)
            .expect_err("binding drift")
            .code,
        "GATE-AUDIT-IDENTITY"
    );
}

#[test]
fn relocated_audit_binds_sealed_attempt_root_instead_of_extraction_path() {
    let root = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../..");
    let original_root = PathBuf::from("/forest1/runtime/e");
    let extracted_root = PathBuf::from("/hosted/runner/unsigned/execution");
    let plan = json!({
        "plan_id": "1".repeat(64), "execution_key": "2".repeat(64),
        "quality_disposition": super::expected_quality_disposition(), "nodes": []
    });
    let mut receipt = read_json(&root.join("gate-policy/v1/fixtures/valid/stage-receipt.json"))
        .expect("stage receipt fixture");
    receipt["plan_id"] = plan["plan_id"].clone();
    receipt["plan_sha256"] = json!(digest(&plan).expect("plan digest"));
    receipt["execution_key"] = plan["execution_key"].clone();
    receipt["artifact_root_sha256"] = json!(path_digest(&original_root));
    receipt["stage_receipt_id"] = json!("0".repeat(64));
    receipt["stage_receipt_id"] =
        json!(derived_id(&receipt, "stage_receipt_id").expect("receipt ID"));
    let mut audit = json!({
        "audit_id": "0".repeat(64), "plan_id": plan["plan_id"],
        "plan_sha256": digest(&plan).expect("plan digest"),
        "execution_key": plan["execution_key"],
        "artifact_root_sha256": path_digest(&original_root),
        "node_manifest": node_manifest(&plan).expect("node manifest"),
        "quality_disposition": plan["quality_disposition"],
        "package_admission": {"status": "READY"},
        "light_stage_receipt_id": receipt["stage_receipt_id"],
        "light_receipt": receipt
    });
    audit["audit_id"] = json!(derived_id(&audit, "audit_id").expect("audit ID"));

    validate_relocated_artifact_binding(&plan, &audit).expect("sealed original root is portable");
    assert_eq!(
        validate_audit_artifact_fields(&plan, &audit, &extracted_root)
            .expect_err("extraction pathname must differ")
            .code,
        "GATE-AUDIT-IDENTITY"
    );
    validate_relocated_light_receipt(&root, &plan, &audit).expect("relocated LIGHT receipt");

    audit["light_receipt"]["artifact_root_sha256"] = json!(path_digest(&extracted_root));
    assert_eq!(
        validate_relocated_artifact_binding(&plan, &audit)
            .expect_err("substituted root identity")
            .code,
        "GATE-AUDIT-IDENTITY"
    );
}

#[test]
fn extracted_ready_checks_preserve_status_and_reason_rules() {
    let checks = CHECK_IDS
        .iter()
        .map(|id| json!({"check_id": id, "status": "PASS", "reason_codes": []}))
        .collect::<Vec<_>>();
    let audit = json!({"checks": checks, "reason_codes": []});
    validate_ready_check_set(&audit).expect("canonical READY checks");
    let ready = audit["checks"].as_array().expect("checks");
    assert_eq!(audit_status(ready), "READY");
    assert!(audit_reason_codes(ready).is_empty());

    let mut invalid = audit;
    invalid["checks"][0]["status"] = json!("INVALID");
    invalid["checks"][0]["reason_codes"] = json!(["GATE-INVALID"]);
    let failed = invalid["checks"].as_array().expect("checks");
    assert!(validate_ready_check_set(&invalid).is_err());
    assert_eq!(audit_status(failed), "INVALID");
    assert_eq!(audit_reason_codes(failed), vec!["GATE-INVALID"]);
}

#[test]
fn extracted_receipt_and_checkpoint_bindings_remain_fail_closed() {
    let plan = json!({"plan_id": "plan", "execution_key": "key"});
    let root = PathBuf::from("/external/light-root");
    let mut receipt = json!({
        "stage_receipt_id": "0".repeat(64), "plan_id": plan["plan_id"],
        "plan_sha256": digest(&plan).expect("plan digest"),
        "execution_key": plan["execution_key"],
        "artifact_root_sha256": path_digest(&root), "stage": "LIGHT"
    });
    receipt["stage_receipt_id"] =
        json!(derived_id(&receipt, "stage_receipt_id").expect("receipt ID"));
    validate_stage_receipt_plan_binding(&plan, &receipt).expect("plan binding");
    validate_stage_receipt_execution_binding(&plan, &receipt, &root).expect("execution binding");

    let node = json!({"node_id": "node"});
    let mut checkpoint = json!({
        "node_sha256": digest(&node).expect("node digest"), "result": "PASS"
    });
    validate_checkpoint_identity(&node, &checkpoint, "node").expect("checkpoint identity");
    checkpoint["result"] = json!("FAIL");
    assert_eq!(
        validate_checkpoint_identity(&node, &checkpoint, "node")
            .expect_err("checkpoint failure")
            .code,
        "GATE-AUDIT-CHECKPOINT-DRIFT"
    );
}

#[test]
fn canonical_audit_document_seals_and_ready_helpers_fail_closed() {
    let root = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../..");
    let plan = json!({
        "plan_id": "2".repeat(64), "execution_key": "4".repeat(64), "nodes": [],
        "quality_disposition": super::expected_quality_disposition()
    });
    let receipt = json!({
        "stage_receipt_id": "5".repeat(64),
        "executor_binary_sha256": "a".repeat(64)
    });
    let checks = CHECK_IDS
        .iter()
        .map(|id| check(id, Ok(()), json!({"id": id})).expect("check"))
        .collect();
    let artifact_root = PathBuf::from("/external/audit-artifacts");
    let ledger = PathBuf::from("/external/audit-ledger.jsonl");
    let audit = audit_document(AuditDocumentInputs {
        plan: &plan,
        light_receipt: &receipt,
        artifact_root: &artifact_root,
        ledger: &ledger,
        ledger_head_sha256: None,
        package_admission: json!({"status": "READY"}),
        checks,
        quality_disposition: plan["quality_disposition"].clone(),
        status: "READY",
        reason_codes: Vec::new(),
    })
    .and_then(|audit| seal_audit(&root, audit))
    .expect("canonical sealed audit");
    assert_eq!(ConstructedAudit(audit.clone()).as_value(), &audit);
    validate_audit_schema(&root, &audit).expect("schema-valid audit");
    validate_ready_audit(&audit).expect("READY audit");
    validate_current_audit_inventory(&plan, &audit).expect("current inventory");

    let mut drifted = audit.clone();
    drifted["status"] = json!("BLOCKED");
    assert_eq!(
        require_ready_audit_status(&drifted)
            .expect_err("blocked audit")
            .code,
        "GATE-AUDIT-NOT-READY"
    );
    drifted = audit;
    drifted["light_receipt"]["stage_receipt_id"] = json!("6".repeat(64));
    assert_eq!(
        validate_embedded_light_receipt_id(&drifted)
            .expect_err("receipt substitution")
            .code,
        "GATE-AUDIT-LIGHT-RECEIPT"
    );
}

#[test]
fn stage_receipt_schema_and_binary_binding_are_enforced_together() {
    let root = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../..");
    let artifact_root = PathBuf::from("/external/light-artifacts");
    let plan = json!({"plan_id": "2".repeat(64), "execution_key": "4".repeat(64)});
    let mut receipt = read_json(&root.join("gate-policy/v1/fixtures/valid/stage-receipt.json"))
        .expect("stage receipt fixture");
    receipt["plan_id"] = plan["plan_id"].clone();
    receipt["plan_sha256"] = json!(digest(&plan).expect("plan digest"));
    receipt["execution_key"] = plan["execution_key"].clone();
    receipt["artifact_root_sha256"] = json!(path_digest(&artifact_root));
    receipt["stage_receipt_id"] = json!("0".repeat(64));
    receipt["stage_receipt_id"] =
        json!(derived_id(&receipt, "stage_receipt_id").expect("receipt ID"));
    validate_stage_receipt(&root, &plan, &receipt, &artifact_root, false)
        .expect("cross-runner receipt validation");
    assert_eq!(
        validate_stage_receipt(&root, &plan, &receipt, &artifact_root, true)
            .expect_err("binary drift")
            .code,
        "GATE-AUDIT-EXECUTOR-BINARY-DRIFT"
    );
    let mut malformed = receipt;
    malformed["execution_key"] = json!("9".repeat(64));
    malformed["stage_receipt_id"] = json!("0".repeat(64));
    malformed["stage_receipt_id"] =
        json!(derived_id(&malformed, "stage_receipt_id").expect("drifted receipt ID"));
    assert_eq!(
        validate_stage_receipt(&root, &plan, &malformed, &artifact_root, false)
            .expect_err("wrong stage")
            .code,
        "GATE-AUDIT-STAGE-RECEIPT-IDENTITY"
    );
}

#[test]
fn light_checkpoint_artifacts_are_content_and_attempt_bound() {
    let root = std::env::temp_dir().join(format!(
        "openwepp-light-checkpoint-{}-{}",
        std::process::id(),
        std::thread::current().name().unwrap_or("test")
    ));
    fs::create_dir_all(root.join(".checkpoints")).expect("checkpoint directory");
    fs::write(root.join("result.txt"), "bound\n").expect("output artifact");
    let node = json!({
        "node_id": "a".repeat(64), "execution_cost_class": "LIGHT",
        "output_paths": ["result.txt"]
    });
    let checkpoint = json!({
        "node_sha256": digest(&node).expect("node digest"), "result": "PASS",
        "artifacts": [{"path": "result.txt", "sha256": file_digest(&root.join("result.txt")).expect("artifact digest")}]
    });
    fs::write(
        root.join(".checkpoints").join(format!(
            "{}.json",
            node["node_id"].as_str().expect("node ID")
        )),
        serde_json::to_vec(&checkpoint).expect("checkpoint JSON"),
    )
    .expect("checkpoint");
    let plan = json!({"nodes": [node]});
    let receipt = json!({"artifact_root_sha256": path_digest(&root)});
    light_attempt_isolated(&plan, &receipt, &root).expect("isolated LIGHT artifacts");
    fs::write(root.join("result.txt"), "drift\n").expect("drift output");
    assert_eq!(
        light_attempt_isolated(&plan, &receipt, &root)
            .expect_err("artifact drift")
            .code,
        "GATE-AUDIT-CHECKPOINT-ARTIFACT-DRIFT"
    );
    fs::remove_dir_all(root).expect("remove checkpoint fixture");
}

#[test]
fn cheap_file_shape_root_and_ledger_guards_cover_success_and_failure() {
    let fixture = PackageFixture::new(true, false);
    require_clean_diff(&fixture.root, &fixture.plan()).expect("clean diff hygiene");
    enforce_authorized_rust_line_limit(&fixture.root, &fixture.plan()).expect("line limit");
    let mut malformed = json!({"nodes": [
        {"node_id": "n", "execution_cost_class": "LIGHT", "arguments": [],
         "expected_inventory": {"mode": "EXACT"}, "prerequisites": []},
        {"node_id": "n", "execution_cost_class": "SIDE", "arguments": [],
         "expected_inventory": {"mode": "EXACT"}, "prerequisites": []}
    ]});
    assert_eq!(
        validate_exact_node_shapes(&malformed)
            .expect_err("duplicate node")
            .code,
        "GATE-AUDIT-INVENTORY-INVALID"
    );
    assert_eq!(
        valid_stage_order(&malformed).expect_err("cost class").code,
        "GATE-AUDIT-COST-CLASS"
    );
    malformed["environment_roots"] = json!({
        "execution_root": "/same", "authority_root": "/same", "documentation_root": "/docs"
    });
    assert_eq!(
        separated_roots(&malformed).expect_err("aliased roots").code,
        "GATE-AUDIT-ROOT-ALIAS"
    );

    let ledger = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("target")
        .join(format!("pre-heavy-ledger-{}.jsonl", std::process::id()));
    fs::create_dir_all(ledger.parent().expect("ledger parent")).expect("target directory");
    fs::write(&ledger, "").expect("empty ledger");
    admit_attempt_ledger(&ledger).expect("durable empty ledger");
    assert_eq!(ledger_head(&ledger).expect("ledger head"), None);
    let entry = append_attempt_record(&ledger, json!({"record_type": "NOTE"}))
        .expect("append ledger entry");
    assert_eq!(
        ledger_head(&ledger).expect("ledger head"),
        Some(entry.clone())
    );
    no_open_tooling_defect_at_head(&ledger, Some(&entry)).expect("stable closed ledger");
    assert_eq!(
        no_open_tooling_defect_at_head(&ledger, None)
            .expect_err("ledger drift")
            .code,
        "GATE-AUDIT-LEDGER-DRIFT"
    );
    let defects = tooling_defect_statuses(
        "{\"record_type\":\"TOOLING_DEFECT\",\"defect_id\":\"RTR-X\",\"status\":\"OPEN\"}\n",
    )
    .expect("defect statuses");
    assert_eq!(
        reject_open_tooling_defects(defects)
            .expect_err("open defect")
            .code,
        "GATE-AUDIT-OPEN-TOOLING-DEFECT"
    );
    fs::remove_file(ledger).expect("remove ledger fixture");
}

#[test]
fn line_limit_skips_only_typed_rust_deletions() {
    let root = std::env::temp_dir().join(format!(
        "openwepp-line-limit-deletion-{}",
        std::process::id()
    ));
    fs::create_dir_all(&root).expect("line-limit root");
    let deleted = json!({
        "authorized_paths": ["src/deleted.rs"],
        "changed_objects": [{"path": "src/deleted.rs", "change_kind": "DELETE"}]
    });
    enforce_authorized_rust_line_limit(&root, &deleted).expect("typed deletion is absent");

    let missing = json!({
        "authorized_paths": ["src/missing.rs"],
        "changed_objects": [{"path": "src/missing.rs", "change_kind": "MODIFY"}]
    });
    assert_eq!(
        enforce_authorized_rust_line_limit(&root, &missing)
            .expect_err("non-deleted Rust path remains required")
            .code,
        "GATE-AUDIT-LINE-COUNT"
    );
    fs::remove_dir_all(root).expect("remove line-limit root");
}

#[test]
fn unsealed_audit_assembles_all_ten_checks_and_fallback_is_representable() {
    let fixture = PackageFixture::new(true, false);
    for package in ["owner", "contender"] {
        let active = fixture
            .root
            .join(format!("docs/work-packages/{package}/prompts/active"));
        fs::create_dir_all(&active).expect("active prompt directory");
        fs::write(active.join("kickoff.md"), "# Kickoff\n").expect("active prompt");
    }
    let artifact_root = fixture.root.join("target/audit-artifacts");
    fs::create_dir_all(&artifact_root).expect("artifact root");
    let ledger = fixture.root.join("target/attempts.jsonl");
    fs::write(&ledger, "").expect("durable ledger");
    let mut plan = fixture.plan();
    plan["plan_id"] = json!("2".repeat(64));
    plan["execution_key"] = json!("4".repeat(64));
    plan["nodes"] = json!([]);
    plan["changed_objects"] = json!([]);
    plan["execution_context"] = json!({
        "configuration_sha256": "a".repeat(64),
        "environment_manifest_sha256": "b".repeat(64),
        "fixture_manifest_sha256": "c".repeat(64),
        "tool_manifest_sha256": "d".repeat(64)
    });
    plan["environment_roots"] = json!({
        "execution_root": "/execution", "authority_root": "/authority",
        "documentation_root": "/documentation"
    });
    plan["quality_disposition"] = super::expected_quality_disposition();
    let receipt = json!({
        "executor_binary_sha256": "e".repeat(64),
        "artifact_root_sha256": path_digest(&artifact_root), "final_results": {},
        "claims": {
            "principal": "p", "repository": "r", "source_event": "e",
            "source_ref": "s", "workflow": "w", "job": "j",
            "runner": "r", "attempt": 1
        }
    });
    let admission = package_admission(&fixture.root, &plan).expect("package admission");
    let quality_disposition = plan["quality_disposition"].clone();
    let checks = build_audit_checks(&AuditCheckInputs {
        repo: &fixture.root,
        plan: &plan,
        light_receipt: &receipt,
        artifact_root: &artifact_root,
        ledger: &ledger,
        package_admission: &admission,
        quality_disposition: &quality_disposition,
        ledger_head_sha256: None,
    })
    .expect("ten-check audit assembly");
    assert_eq!(checks.len(), CHECK_IDS.len());
    assert!(
        checks
            .iter()
            .zip(CHECK_IDS)
            .all(|(item, id)| item["check_id"] == id)
    );
    assert_eq!(audit_status(&checks), "BLOCKED");

    let fallback = construct_audit(
        &PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../.."),
        &json!({"nodes": []}),
        &json!({}),
        &artifact_root,
        &ledger,
    )
    .expect("representable fallback audit");
    assert_eq!(
        fallback.as_value()["checks"].as_array().map(Vec::len),
        Some(10)
    );
}

#[test]
fn active_package_prompt_must_match_bound_digest() {
    let root = std::env::temp_dir().join(format!(
        "openwepp-active-prompt-{}-{}",
        std::process::id(),
        std::thread::current().name().unwrap_or("test")
    ));
    let active = root.join("docs/work-packages/prompt/prompts/active");
    fs::create_dir_all(&active).expect("active prompt directory");
    let prompt = active.join("kickoff.md");
    fs::write(&prompt, "# Kickoff\n").expect("active prompt");
    let admission = json!({
        "prompt_owner": {
            "prompt_path": "docs/work-packages/prompt/prompts/active/kickoff.md",
            "prompt_sha256": crate::canonical::sha256_bytes(b"# Kickoff\n")
        }
    });
    require_bound_active_prompt(&root, &admission).expect("bound active prompt");
    fs::write(&prompt, "# Mutated\n").expect("mutated prompt");
    assert!(require_bound_active_prompt(&root, &admission).is_err());
    fs::write(&prompt, "# Kickoff\n").expect("restore prompt");
    fs::write(active.join("extra.md"), "# Extra\n").expect("extra prompt");
    assert!(require_bound_active_prompt(&root, &admission).is_err());
    fs::remove_file(active.join("extra.md")).expect("remove extra prompt");
    #[cfg(unix)]
    {
        use std::os::unix::fs::symlink;

        fs::remove_file(&prompt).expect("remove regular prompt");
        symlink("../../../../README.md", &prompt).expect("symlink prompt");
        assert!(require_bound_active_prompt(&root, &admission).is_err());
    }
    fs::remove_dir_all(root).expect("remove prompt fixture");
}

#[test]
fn package_admission_requires_exact_chain_identity() {
    let plan = json!({
        "source": {"base_commit": "a", "head_commit": "b"},
        "authorized_paths": ["src/lib.rs"],
        "package_authority": {
            "chain_id": "c".repeat(64),
            "intent_package_path": "docs/work-packages/one/package.md"
        }
    });
    let ready = json!({
        "status": "READY",
        "base_commit": "a",
        "head_commit": "b",
        "package_authority_chain_id": "c".repeat(64),
        "intent_package_path": "docs/work-packages/one/package.md",
        "changed_paths": plan["authorized_paths"],
        "reason_codes": []
    });
    package_admitted(&plan, &ready).expect("exact chain identity");
    let mut tampered = ready;
    tampered["package_authority_chain_id"] = json!("d".repeat(64));
    assert!(package_admitted(&plan, &tampered).is_err());
}

static PACKAGE_FIXTURE_SEQUENCE: AtomicU64 = AtomicU64::new(0);

struct PackageFixture {
    root: PathBuf,
    base: String,
    head: String,
    paths: Vec<String>,
}

impl PackageFixture {
    fn new(owner_ready: bool, contender_ready: bool) -> Self {
        let sequence = PACKAGE_FIXTURE_SEQUENCE.fetch_add(1, Ordering::Relaxed);
        let root = std::env::temp_dir().join(format!(
            "openwepp-package-admission-{}-{sequence}",
            std::process::id()
        ));
        fs::create_dir_all(root.join("gate-policy/v1/schemas")).expect("schema directory");
        fs::create_dir_all(root.join("docs/work-packages/owner")).expect("owner directory");
        fs::create_dir_all(root.join("docs/work-packages/contender")).expect("contender directory");
        fs::create_dir_all(root.join("src")).expect("source directory");
        let repository = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../..");
        fs::copy(
            repository.join("gate-policy/v1/schemas/package-authority-chain.schema.json"),
            root.join("gate-policy/v1/schemas/package-authority-chain.schema.json"),
        )
        .expect("copy package schema");
        Self::write_package(&root, "owner", owner_ready, "base owner");
        Self::write_package(&root, "contender", contender_ready, "base contender");
        let active = root.join("docs/work-packages/owner/prompts/active");
        fs::create_dir_all(&active).expect("owner active prompt directory");
        fs::write(active.join("kickoff.md"), "# Kickoff\n").expect("owner active prompt");
        fs::write(root.join("src/lib.rs"), "pub fn base() {}\n").expect("base source");
        Self::git(&root, &["init", "-q"]);
        Self::git(&root, &["config", "user.email", "test@example.invalid"]);
        Self::git(&root, &["config", "user.name", "Test"]);
        Self::git(&root, &["add", "."]);
        Self::git(&root, &["commit", "-qm", "base"]);
        let base = String::from_utf8(Self::git_output(&root, &["rev-parse", "HEAD"]))
            .expect("UTF-8 base")
            .trim()
            .to_owned();
        Self::write_package(&root, "owner", owner_ready, "changed owner");
        Self::write_package(&root, "contender", contender_ready, "changed contender");
        fs::write(root.join("src/lib.rs"), "pub fn changed() {}\n").expect("changed source");
        Self::git(&root, &["add", "."]);
        Self::git(&root, &["commit", "-qm", "change"]);
        let head = String::from_utf8(Self::git_output(&root, &["rev-parse", "HEAD"]))
            .expect("UTF-8 head")
            .trim()
            .to_owned();
        Self {
            root,
            base,
            head,
            paths: vec![
                "docs/work-packages/contender/package.md".to_owned(),
                "docs/work-packages/owner/package.md".to_owned(),
                "src/lib.rs".to_owned(),
            ],
        }
    }

    fn write_package(root: &std::path::Path, name: &str, ready: bool, note: &str) {
        let write_set = if ready {
            "- `docs/work-packages/**`\n- `src/**`".to_owned()
        } else {
            format!("- `docs/work-packages/{name}/**`")
        };
        fs::write(
            root.join(format!("docs/work-packages/{name}/package.md")),
            format!(
                "# {name}\n\nStatus: ACTIVE\n\n{note}\n\n## Declared Write Set\n\n{write_set}\n"
            ),
        )
        .expect("write package");
    }

    fn plan(&self) -> serde_json::Value {
        let chain = crate::package_validation::validate_package_chain(
            &self.root,
            &self.base,
            Some(&self.head),
            std::path::Path::new("docs/work-packages/owner/package.md"),
        )
        .expect("fixture package chain");
        json!({
            "source": {"base_commit": self.base, "head_commit": self.head},
            "authorized_paths": self.paths,
            "package_authority": {
                "chain_id": chain["package_authority_chain_id"],
                "intent_package_path": "docs/work-packages/owner/package.md"
            }
        })
    }

    fn git(root: &std::path::Path, arguments: &[&str]) {
        assert!(
            Command::new("git")
                .args(arguments)
                .current_dir(root)
                .status()
                .expect("run git")
                .success(),
            "git {arguments:?}"
        );
    }

    fn git_output(root: &std::path::Path, arguments: &[&str]) -> Vec<u8> {
        let output = Command::new("git")
            .args(arguments)
            .current_dir(root)
            .output()
            .expect("run git");
        assert!(output.status.success(), "git {arguments:?}");
        output.stdout
    }
}

impl Drop for PackageFixture {
    fn drop(&mut self) {
        fs::remove_dir_all(&self.root).expect("remove package fixture");
    }
}
