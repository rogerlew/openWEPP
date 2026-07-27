use super::*;
use std::sync::atomic::{AtomicU64, Ordering};

static TEMP_SEQUENCE: AtomicU64 = AtomicU64::new(0);

#[test]
fn audit_failure_records_started_then_one_terminal() {
    let fixture = TransitionFixture::new();
    let package = "docs/work-packages/integration/package.md";
    fixture.write(
        package,
        "# Integration\n\nStatus: `ACTIVE`\n\n## Intended Write Set\n\n- `docs/**`\n- `evidence/**`\n- `plan/**`\n- `tools/**`\n",
    );
    fixture.copy_policy_schema("package-audit.schema.json");
    let base = fixture.commit("scaffold external transition package");

    fixture.write("evidence/cheap.txt", "focused gates passed\n");
    fixture.write_executable(
        "tools/light.sh",
        "#!/bin/sh\nset -eu\nmkdir -p \"$(dirname \"$1\")\"\nprintf 'light-output\\n' > \"$1\"\n",
    );
    fixture.write_executable(
        "tools/heavy.sh",
        "#!/bin/sh\nset -eu\ntest \"$(cat \"$1\")\" = light-output\nmkdir -p \"$(dirname \"$2\")\"\nprintf 'heavy-from-%s\\n' \"$(cat \"$1\")\" > \"$2\"\n",
    );
    fixture.write(
        "plan/source.csv",
        &format!(
            "order,command_id,source_path,argv,environment,working_directory,inputs,outputs,harvard_access,cost_class\n\
             1,light,tools/light.sh,tools/light.sh ${{OBJECTS_ROOT}}/light.txt,default,{},seed,light.txt,FORBIDDEN,QUICK\n\
             2,heavy,tools/heavy.sh,tools/heavy.sh ${{OBJECTS_ROOT}}/light.txt ${{OBJECTS_ROOT}}/heavy.txt,default,{},light output,heavy.txt,FORBIDDEN,HEAVY\n",
            fixture.root.path.display(),
            fixture.root.path.display(),
        ),
    );
    fixture.write(
        "plan/contract.csv",
        concat!(
            "command_id,prerequisites,receipt_outputs\n",
            "light,-,objects/light.txt\n",
            "heavy,light,objects/heavy.txt\n",
        ),
    );

    let mut plan = json!({
        "schema": EXTERNAL_PLAN_SCHEMA,
        "plan_id": "0".repeat(64),
        "generation": "A",
        "parent_plan": null,
        "source_identity": null,
        "source_plan": fixture.binding("plan/source.csv"),
        "source_contract": fixture.binding("plan/contract.csv"),
        "authority": {
            "package_path": package,
            "base_commit": base,
            "cheap_gate_evidence": [fixture.binding("evidence/cheap.txt")],
        },
        "transactions": [{
            "transaction_id": "real-transition",
            "light": [{
                "order": 1,
                "command_id": "light",
                "argv": ["${REPO}/tools/light.sh", "${OBJECTS_ROOT}/light.txt"],
                "env": {},
                "cwd": fixture.root.path.display().to_string(),
                "source_working_directory": fixture.root.path.display().to_string(),
                "source_inputs": ["seed"],
                "prerequisites": [],
                "cost_class": "QUICK",
                "source_path": "tools/light.sh",
                "declared_outputs": ["objects/light.txt"],
                "timeout_seconds": 10,
                "max_attempts": 1,
                "handoff": "READY audit",
                "harvard_access": "NONE"
            }],
            "heavy": [{
                "order": 2,
                "command_id": "heavy",
                "argv": [
                    "${REPO}/tools/heavy.sh",
                    "${OBJECTS_ROOT}/light.txt",
                    "${OBJECTS_ROOT}/heavy.txt"
                ],
                "env": {},
                "cwd": fixture.root.path.display().to_string(),
                "source_working_directory": fixture.root.path.display().to_string(),
                "source_inputs": ["light output"],
                "prerequisites": ["light"],
                "cost_class": "HEAVY",
                "source_path": "tools/heavy.sh",
                "declared_outputs": ["objects/heavy.txt"],
                "timeout_seconds": 10,
                "max_attempts": 1,
                "handoff": "terminal receipt",
                "harvard_access": "NONE"
            }],
            "custody_prerequisites": [],
            "custody_receipts": []
        }],
        "custody_commands": []
    });
    plan["plan_id"] =
        Value::String(derived_id(&plan, "plan_id").expect("derive Generation-A plan identity"));
    fixture.write_json("plan/external.json", &plan);
    fixture.commit("bind Generation-A transition plan");
    let committed_plan =
        load_plan(&fixture.root.path.join("plan/external.json")).expect("committed plan");
    let light_node = &committed_plan.transactions[0].light[0];
    assert!(
        command_projection_matches(
            &fixture.root.path,
            "tools/light.sh ${OBJECTS_ROOT}/light.txt",
            light_node,
        )
        .expect("LIGHT command projection")
    );
    assert!(
        environment_projection_matches(
            "tools/light.sh ${OBJECTS_ROOT}/light.txt",
            "default",
            light_node,
        )
        .expect("LIGHT environment projection")
    );
    assert!(output_projection_matches("light.txt", light_node));

    let blocked_external = TempDirectory::new("openwepp-external-blocked-audit");
    let blocked_ledger = blocked_external.path.join("ledger.jsonl");
    fs::write(&blocked_ledger, []).expect("create blocked durable ledger");
    append_attempt_record(
        &blocked_ledger,
        json!({
            "record_type": "TOOLING_DEFECT",
            "status": "OPEN",
            "defect_id": "fixture-open-defect",
        }),
    )
    .expect("record open tooling defect");
    let blocked_options = ExternalTransitionOptions {
        repo: fixture.root.path.clone(),
        plan_path: fixture.root.path.join("plan/external.json"),
        transaction_id: "real-transition".to_owned(),
        attempt_root: blocked_external.path.join("attempt"),
        ledger: blocked_ledger.clone(),
        receipt_path: blocked_external.path.join("receipt.json"),
        custody_root: None,
        opening_token: None,
        claims: ExecutionClaims {
            principal: "integration-test".to_owned(),
            repository: "local/external-transition".to_owned(),
            source_event: "test".to_owned(),
            source_ref: "refs/heads/main".to_owned(),
            workflow: "external-dag-integration".to_owned(),
            job: "blocked-audit".to_owned(),
            runner: "local-test".to_owned(),
            attempt: 1,
        },
    };
    let blocked_error = run_external_transition(&blocked_options)
        .expect_err("failed audit must still enter and balance HEAVY lifecycle");
    assert_eq!(blocked_error.code, "GATE-EXTERNAL-AUDIT-INVALID");
    let blocked_audit: Value = serde_json::from_slice(
        &fs::read(blocked_options.receipt_path.with_extension("audit.json"))
            .expect("persisted blocked audit"),
    )
    .expect("blocked audit JSON");
    assert_eq!(blocked_audit["status"], "BLOCKED");
    assert_eq!(
        blocked_audit["checks"]
            .as_array()
            .expect("evaluated checks")
            .len(),
        10
    );
    let blocked_records = fs::read_to_string(&blocked_ledger).expect("blocked ledger");
    let blocked_records = blocked_records
        .lines()
        .map(|line| serde_json::from_str::<Value>(line).expect("blocked ledger record"))
        .collect::<Vec<_>>();
    assert_eq!(
        blocked_records[blocked_records.len() - 2]["status"],
        "STARTED"
    );
    assert_eq!(
        blocked_records[blocked_records.len() - 1]["status"],
        "FAILED"
    );
    let blocked_started = blocked_records
        .iter()
        .filter(|record| {
            record["record_type"] == "EXTERNAL_TRANSACTION"
                && record["stage"] == "HEAVY"
                && record["status"] == "STARTED"
        })
        .count();
    let blocked_terminal = blocked_records
        .iter()
        .filter(|record| {
            record["record_type"] == "EXTERNAL_TRANSACTION"
                && record["stage"] == "HEAVY"
                && matches!(record["status"].as_str(), Some("FAILED" | "CLOSED"))
        })
        .count();
    // RED at the retained clean anchor: audit evaluation preceded STARTED, so
    // an audit failure had neither half of this balanced lifecycle.
    assert_eq!((blocked_started, blocked_terminal), (1, 1));

    let external = TempDirectory::new("openwepp-external-transition");
    let attempt_root = external.path.join("attempt");
    let ledger = external.path.join("ledger.jsonl");
    let receipt_path = external.path.join("receipt.json");
    fs::write(&ledger, []).expect("create external durable ledger");
    let options = ExternalTransitionOptions {
        repo: fixture.root.path.clone(),
        plan_path: fixture.root.path.join("plan/external.json"),
        transaction_id: "real-transition".to_owned(),
        attempt_root: attempt_root.clone(),
        ledger: ledger.clone(),
        receipt_path: receipt_path.clone(),
        custody_root: None,
        opening_token: None,
        claims: ExecutionClaims {
            principal: "integration-test".to_owned(),
            repository: "local/external-transition".to_owned(),
            source_event: "test".to_owned(),
            source_ref: "refs/heads/main".to_owned(),
            workflow: "external-dag-integration".to_owned(),
            job: "light-ready-heavy".to_owned(),
            runner: "local-test".to_owned(),
            attempt: 1,
        },
    };

    let receipt = run_external_transition(&options).expect("real transition must pass");

    assert_eq!(
        fs::read_to_string(attempt_root.join("objects/light.txt")).expect("light output"),
        "light-output\n"
    );
    assert_eq!(
        fs::read_to_string(attempt_root.join("objects/heavy.txt")).expect("heavy output"),
        "heavy-from-light-output\n"
    );
    let persisted_light: Value = serde_json::from_slice(
        &fs::read(receipt_path.with_extension("light.json")).expect("persisted LIGHT receipt"),
    )
    .expect("LIGHT receipt JSON");
    let persisted_audit: Value = serde_json::from_slice(
        &fs::read(receipt_path.with_extension("audit.json")).expect("persisted READY audit"),
    )
    .expect("READY audit JSON");
    assert_eq!(persisted_light, receipt["light"]);
    assert_eq!(persisted_audit["status"], "READY");
    assert_eq!(persisted_audit, receipt["audit"]);
    assert_eq!(receipt["result"], "PASS");
    let persisted_receipt: Value =
        serde_json::from_slice(&fs::read(&receipt_path).expect("final receipt"))
            .expect("final receipt JSON");
    assert_eq!(persisted_receipt, receipt);

    let records = fs::read_to_string(&ledger)
        .expect("durable ledger")
        .lines()
        .map(|line| serde_json::from_str::<Value>(line).expect("ledger JSONL record"))
        .collect::<Vec<_>>();
    assert_eq!(records.len(), 3);
    assert_eq!(
        records
            .iter()
            .map(|record| (
                record["stage"].as_str().expect("stage"),
                record["status"].as_str().expect("status")
            ))
            .collect::<Vec<_>>(),
        vec![
            ("LIGHT", "CLOSED"),
            ("HEAVY", "STARTED"),
            ("HEAVY", "CLOSED")
        ]
    );
    assert_eq!(
        records[2]["started_entry_sha256"],
        records[1]["entry_sha256"]
    );
    verify_external_transaction(&options.plan_path, &receipt)
        .expect("independent external transaction verification must pass");

    let mut missing_prerequisites = BTreeMap::new();
    let error = verify_receipt_stage(
        &committed_plan.transactions[0].heavy,
        &receipt["heavy"],
        "HEAVY",
        &mut vec![PathBuf::from("objects/light.txt")],
        &options,
        &mut missing_prerequisites,
    )
    .expect_err("missing prerequisite receipt must fail closed");
    assert_eq!(error.code, "GATE-EXTERNAL-RECEIPT-PREREQUISITES");

    let mut forged = receipt.clone();
    forged["claims"]["principal"] = Value::String("forged-principal".to_owned());
    forged["audit"]["claims"] = forged["claims"].clone();
    forged["audit"]["audit_id"] =
        Value::String(derived_id(&forged["audit"], "audit_id").expect("rederive forged audit"));
    forged["receipt_id"] =
        Value::String(derived_id(&forged, "receipt_id").expect("rederive forged receipt"));
    let error = verify_external_transaction(&options.plan_path, &forged)
        .expect_err("self-consistent receipt claims must not override the durable ledger");
    assert!(
        matches!(
            error.code,
            "GATE-EXTERNAL-RECEIPT-AUDIT"
                | "GATE-EXTERNAL-STARTED-MISSING"
                | "GATE-EXTERNAL-LEDGER-TERMINAL"
        ),
        "unexpected error: {error:?}"
    );
}

#[test]
#[allow(
    clippy::too_many_lines,
    reason = "the Generation-B consumer-path fixture keeps parent authority, custody dispatch, execution, and independent verification contiguous"
)]
fn generation_b_run_external_transition_consumes_once_and_verifies_final_receipt() {
    let fixture = TransitionFixture::new();
    let package = "docs/work-packages/integration/package.md";
    fixture.write(
        package,
        "# Integration\n\nStatus: `ACTIVE`\n\n## Intended Write Set\n\n- `docs/**`\n- `evidence/**`\n- `plan/**`\n- `tools/**`\n",
    );
    fixture.copy_policy_schema("package-audit.schema.json");
    let base = fixture.commit("scaffold Generation-B integration package");
    fixture.write("evidence/cheap.txt", "focused gates passed\n");
    fixture.write_executable(
        "tools/light.sh",
        "#!/bin/sh\nset -eu\nmkdir -p \"$(dirname \"$1\")\"\nprintf 'light-output\\n' > \"$1\"\n",
    );
    fixture.write_executable(
        "tools/heavy.sh",
        "#!/bin/sh\nset -eu\ntest \"$(cat \"$1\")\" = light-output\nprintf 'heavy-output\\n' > \"$2\"\n",
    );
    fixture.write_executable(
        "docs/work-packages/20260727-canopy-cal-04b-calibration-readiness-and-ensemble-execution-001/tools/freeze-verify.py",
        "#!/usr/bin/env python3\n",
    );
    fixture.write(
        "plan/source.csv",
        &format!(
            "order,command_id,source_path,argv,environment,working_directory,inputs,outputs,harvard_access,cost_class\n\
             1,light,tools/light.sh,tools/light.sh ${{OBJECTS_ROOT}}/light.txt,default,{},seed,light.txt,FORBIDDEN,QUICK\n\
             2,heavy,tools/heavy.sh,tools/heavy.sh ${{OBJECTS_ROOT}}/light.txt ${{OBJECTS_ROOT}}/heavy.txt,default,{},light output,heavy.txt,FORBIDDEN,HEAVY\n",
            fixture.root.path.display(),
            fixture.root.path.display(),
        ),
    );
    fixture.write(
        "plan/contract.csv",
        concat!(
            "command_id,prerequisites,receipt_outputs\n",
            "light,-,objects/light.txt\n",
            "heavy,light,objects/heavy.txt\n",
        ),
    );
    let transaction = json!({
        "transaction_id": "generation-b-transition",
        "light": [integration_node(&fixture.root.path, "light", 1, "QUICK")],
        "heavy": [integration_node(&fixture.root.path, "heavy", 2, "HEAVY")],
        "custody_prerequisites": [],
        "custody_receipts": [],
    });
    let mut parent = json!({
        "schema": EXTERNAL_PLAN_SCHEMA,
        "plan_id": "0".repeat(64),
        "generation": "A",
        "parent_plan": null,
        "source_identity": null,
        "source_plan": fixture.binding("plan/source.csv"),
        "source_contract": fixture.binding("plan/contract.csv"),
        "authority": {
            "package_path": package,
            "base_commit": base,
            "cheap_gate_evidence": [fixture.binding("evidence/cheap.txt")],
        },
        "transactions": [transaction],
        "custody_commands": [],
    });
    parent["plan_id"] =
        json!(derived_id(&parent, "plan_id").expect("derive Generation-A parent identity"));
    fixture.write_json("plan/parent.json", &parent);
    fixture.commit("bind Generation-A parent authority");

    let external = TempDirectory::new("openwepp-generation-b-orchestration");
    let plan_path = external.path.join("generation-b.json");
    let mut child = parent.clone();
    child["generation"] = json!("B");
    child["parent_plan"] = json!({
        "path": fixture.root.path.join("plan/parent.json").display().to_string(),
        "sha256": file_sha256(&fixture.root.path.join("plan/parent.json"))
            .expect("parent plan digest"),
        "plan_id": parent["plan_id"],
    });
    child["source_identity"] =
        serde_json::to_value(repository_identity(&fixture.root.path).expect("source identity"))
            .expect("serialize source identity");
    child["transactions"][0]["custody_prerequisites"] =
        json!(["freeze_verify_a.json", "freeze_verify_b.json"]);
    child["plan_id"] = json!("0".repeat(64));
    child["plan_id"] = json!(derived_id(&child, "plan_id").expect("derive Generation-B identity"));
    fs::write(
        &plan_path,
        canonical_bytes(&child).expect("canonical Generation-B plan"),
    )
    .expect("write Generation-B plan");

    let custody_root = external.path.join("custody");
    let first_options = integration_options(
        &fixture.root.path,
        &plan_path,
        &external.path.join("attempt-1"),
        &external.path.join("ledger-1.jsonl"),
        &external.path.join("receipt-1.json"),
        &custody_root,
        1,
    );
    let stale_options = integration_options(
        &fixture.root.path,
        &plan_path,
        &external.path.join("attempt-stale"),
        &external.path.join("ledger-stale.jsonl"),
        &external.path.join("receipt-stale.json"),
        &custody_root,
        4,
    );
    fs::write(&stale_options.ledger, []).expect("stale ledger");
    install_custody_dispatch(&stale_options, "dispatch-stale", b"stale");
    for name in ["freeze_verify_a.json", "freeze_verify_b.json"] {
        let path = custody_root.join(name);
        let mut stale = parse_strict(&fs::read(&path).expect("fresh attestation"))
            .expect("fresh attestation JSON");
        stale["created_at"] = json!("2000-01-01T00:00:00Z");
        stale["attestation_id"] =
            json!(derived_id(&stale, "attestation_id").expect("stale attestation identity"));
        fs::write(
            path,
            canonical_bytes(&stale).expect("canonical stale attestation"),
        )
        .expect("write stale attestation");
    }
    let stale_error = run_external_transition(&stale_options)
        .expect_err("orchestration must reject an old same-dispatch attestation");
    assert_eq!(
        stale_error.code, "GATE-EXTERNAL-ATTESTATION-FRESHNESS",
        "{stale_error:?}"
    );
    assert_eq!(stale_error.class, ErrorClass::Trust);
    assert!(
        fs::read(&stale_options.ledger)
            .expect("stale ledger")
            .is_empty()
    );
    fs::remove_dir_all(custody_root.join("capabilities")).expect("remove stale capabilities");
    fs::write(&first_options.ledger, []).expect("first ledger");
    install_custody_dispatch(&first_options, "dispatch-one", b"first");
    let before = capability_tree_at(&custody_root);
    reset_inventory_reconstruction_count();
    crate::pre_heavy::reset_ledger_admission_count();

    let receipt = run_external_transition(&first_options).unwrap_or_else(|error| {
        let audit = fs::read_to_string(first_options.receipt_path.with_extension("audit.json"))
            .unwrap_or_else(|_| "<no audit>".to_owned());
        panic!("Generation-B orchestration must pass: {error:?}; audit={audit}");
    });
    let after = capability_tree_at(&custody_root);
    assert_eq!(before.active.len(), 2);
    assert!(
        after.active.is_empty(),
        "original capabilities are not reusable"
    );
    assert_eq!(after.consumed.len(), 2);
    assert_eq!(
        receipt["audit"]["consumed_custody_proof"]["entries"]
            .as_array()
            .map(Vec::len),
        Some(2)
    );
    assert_eq!(inventory_reconstruction_count(), 1);
    assert_eq!(crate::pre_heavy::ledger_admission_count(), 1);

    let ledger_records = fs::read_to_string(&first_options.ledger)
        .expect("first ledger")
        .lines()
        .map(|line| serde_json::from_str::<Value>(line).expect("ledger record"))
        .collect::<Vec<_>>();
    assert_eq!(
        ledger_records
            .iter()
            .map(|record| (record["stage"].as_str(), record["status"].as_str()))
            .collect::<Vec<_>>(),
        vec![
            (Some("LIGHT"), Some("CLOSED")),
            (Some("HEAVY"), Some("STARTED")),
            (Some("HEAVY"), Some("CLOSED")),
        ]
    );
    verify_external_transaction(&plan_path, &receipt)
        .expect("independent final receipt verification");
    assert_eq!(
        crate::pre_heavy::ledger_admission_count(),
        1,
        "independent proof verification must not re-admit the ledger"
    );
    let mut malformed_receipt = receipt.clone();
    malformed_receipt["light"] = Value::Null;
    malformed_receipt["receipt_id"] =
        json!(derived_id(&malformed_receipt, "receipt_id").expect("malformed receipt identity"));
    let receipt_error = verify_external_transaction(&plan_path, &malformed_receipt)
        .expect_err("actual verifier must type receipt shape failures");
    assert_eq!(receipt_error.class, ErrorClass::Receipt);

    let mut forged_custody = receipt.clone();
    forged_custody["audit"]["consumed_custody_proof"]["entries"]
        .as_array_mut()
        .expect("consumed entries")
        .pop();
    forged_custody["audit"]["consumed_custody_proof"]["proof_id"] = json!(
        derived_id(
            &forged_custody["audit"]["consumed_custody_proof"],
            "proof_id"
        )
        .expect("forged custody proof identity")
    );
    forged_custody["audit"]["audit_id"] =
        json!(derived_id(&forged_custody["audit"], "audit_id").expect("forged audit identity"));
    forged_custody["receipt_id"] =
        json!(derived_id(&forged_custody, "receipt_id").expect("forged receipt identity"));
    let custody_error = verify_external_transaction(&plan_path, &forged_custody)
        .expect_err("actual verifier must type custody inventory failures");
    assert_eq!(custody_error.class, ErrorClass::Trust);

    let original_ledger = fs::read(&first_options.ledger).expect("original ledger");
    let mut forged_records = ledger_records.clone();
    forged_records[2]["previous_entry_sha256"] = Value::Null;
    let mut forged_ledger = Vec::new();
    for record in &forged_records {
        forged_ledger.extend(canonical_bytes(record).expect("canonical forged ledger record"));
        forged_ledger.push(b'\n');
    }
    fs::write(&first_options.ledger, forged_ledger).expect("write forged ledger");
    let ledger_error = verify_external_transaction(&plan_path, &receipt)
        .expect_err("actual verifier must type ledger chain failures");
    assert_eq!(
        ledger_error.class,
        ErrorClass::Ledger,
        "unexpected ledger verifier error: {ledger_error:?}"
    );
    fs::write(&first_options.ledger, original_ledger).expect("restore ledger");

    let retry_options = integration_options(
        &fixture.root.path,
        &plan_path,
        &external.path.join("attempt-retry"),
        &external.path.join("ledger-retry.jsonl"),
        &external.path.join("receipt-retry.json"),
        &custody_root,
        2,
    );
    fs::write(&retry_options.ledger, []).expect("retry ledger");
    let retry_error =
        run_external_transition(&retry_options).expect_err("consumed dispatch cannot be restarted");
    assert_eq!(retry_error.code, "GATE-EXTERNAL-CAPABILITY-MISSING");
    assert!(
        fs::read(&retry_options.ledger)
            .expect("retry ledger")
            .is_empty()
    );

    let second_options = integration_options(
        &fixture.root.path,
        &plan_path,
        &external.path.join("attempt-2"),
        &external.path.join("ledger-2.jsonl"),
        &external.path.join("receipt-2.json"),
        &custody_root,
        3,
    );
    fs::write(&second_options.ledger, []).expect("second ledger");
    install_custody_dispatch(&second_options, "dispatch-two", b"second");
    let second_receipt = run_external_transition(&second_options)
        .expect("new dispatch in the same custody root must pass");
    verify_external_transaction(&plan_path, &second_receipt)
        .expect("independently verify second dispatch");
    verify_external_transaction(&plan_path, &receipt).expect(
        "dispatch-one receipt must remain independently verifiable after live attestations change",
    );
    let after_second = capability_tree_at(&custody_root);
    assert!(after_second.active.is_empty());
    assert_eq!(after_second.consumed.len(), 4);
    assert_ne!(
        receipt["audit"]["consumed_custody_proof"]["consumed_root"],
        second_receipt["audit"]["consumed_custody_proof"]["consumed_root"]
    );
}

#[test]
fn capability_consumption_destination_collision_is_atomic_and_fail_closed() {
    let fixture = CustodyFixture::new("holdout-v1");
    let before = fixture.capability_tree();
    let scope = sha256_bytes(b"dispatch\0holdout-v1");
    fs::create_dir_all(
        fixture
            .custody
            .path
            .join("consumed-capabilities")
            .join(scope),
    )
    .expect("pre-existing dispatch scope");
    let error = consume_custody_capabilities(&fixture.options, &fixture.transaction)
        .expect_err("scope collision must fail before the atomic directory rename");
    assert_eq!(error.code, "GATE-EXTERNAL-CAPABILITY-ALREADY-CONSUMED");
    let after = fixture.capability_tree();
    assert_eq!(after.active, before.active);
    assert!(after.consumed.is_empty());
}

#[test]
fn external_orphan_started_is_durably_balanced_once() {
    let external = TempDirectory::new("openwepp-external-orphan-ledger");
    let ledger = external.path.join("ledger.jsonl");
    fs::write(&ledger, []).expect("create orphan ledger");
    let started = append_attempt_record(
        &ledger,
        json!({
            "record_type": "EXTERNAL_TRANSACTION",
            "status": "STARTED",
            "stage": "HEAVY",
            "phase": "ADMISSION",
            "plan_id": "1".repeat(64),
            "audit_id": "2".repeat(64),
            "artifact_root": null,
            "attempt_root": external.path.join("attempt").display().to_string(),
            "transaction_id": "orphan-transition",
            "recovery_root": null,
            "workflow": "external-dag-integration",
            "job": "orphan-reconciliation",
            "runner": "terminated-runner",
            "attempt": 1
        }),
    )
    .expect("persist external STARTED before simulated process loss");

    assert_eq!(
        reconcile_orphaned_attempts(&ledger).expect("reconcile external orphan"),
        1
    );
    assert_eq!(
        reconcile_orphaned_attempts(&ledger).expect("reconciliation must be idempotent"),
        0
    );
    let records = fs::read_to_string(&ledger)
        .expect("reconciled ledger")
        .lines()
        .map(|line| serde_json::from_str::<Value>(line).expect("ledger record"))
        .collect::<Vec<_>>();
    assert_eq!(records.len(), 2);
    assert_eq!(records[0]["entry_sha256"], started);
    assert_eq!(records[1]["record_type"], "EXTERNAL_TRANSACTION");
    assert_eq!(records[1]["status"], "FAILED");
    assert_eq!(records[1]["stage"], "HEAVY");
    assert_eq!(records[1]["transaction_id"], "orphan-transition");
    assert_eq!(records[1]["started_entry_sha256"], started);
    assert_eq!(
        records[1]["cause_key"],
        "GATE-ATTEMPT-PREVIOUS-PROCESS-TERMINATED"
    );
    assert_eq!(records[1]["failure_class"], "INFRASTRUCTURE");
    assert_eq!(
        records[1]["previous_entry_sha256"],
        records[0]["entry_sha256"]
    );
    let mut terminal_unsigned = records[1].clone();
    terminal_unsigned
        .as_object_mut()
        .expect("terminal object")
        .remove("entry_sha256");
    assert_eq!(
        records[1]["entry_sha256"],
        digest(&terminal_unsigned).expect("terminal chain digest")
    );
}

#[test]
#[allow(
    clippy::too_many_lines,
    reason = "the real dual-Cargo integration fixture keeps both build and consumer custody in one transaction"
)]
fn real_cal_cargo_build_patterns_use_isolated_authenticated_caches_and_staged_consumers() {
    let fixture = TransitionFixture::new();
    fixture.write(
        "Cargo.toml",
        "[workspace]\nmembers = [\"runner\"]\nresolver = \"2\"\n",
    );
    fixture.write(
        "runner/Cargo.toml",
        "[package]\nname = \"runner\"\nversion = \"0.1.0\"\nedition = \"2024\"\n\n[[bin]]\nname = \"runner-bin\"\npath = \"src/main.rs\"\n",
    );
    fixture.write(
        "runner/src/main.rs",
        "fn main() { let path = std::path::PathBuf::from(std::env::args().nth(1).expect(\"output\")); std::fs::create_dir_all(path.parent().expect(\"parent\")).expect(\"mkdir\"); std::fs::write(path, b\"runner-consumed\\n\").expect(\"write\"); }\n",
    );
    fixture.write(
        "tools/executor/Cargo.toml",
        "[package]\nname = \"executor\"\nversion = \"0.1.0\"\nedition = \"2024\"\n\n[workspace]\n\n[[bin]]\nname = \"executor-bin\"\npath = \"src/main.rs\"\n",
    );
    fixture.write(
        "tools/executor/src/main.rs",
        "fn main() { let path = std::path::PathBuf::from(std::env::args().nth(1).expect(\"output\")); std::fs::create_dir_all(path.parent().expect(\"parent\")).expect(\"mkdir\"); std::fs::write(path, b\"executor-consumed\\n\").expect(\"write\"); }\n",
    );
    fixture.write(
        "Cargo.lock",
        "# This file is automatically @generated by Cargo.\n# It is not intended for manual editing.\nversion = 4\n\n[[package]]\nname = \"runner\"\nversion = \"0.1.0\"\n",
    );
    fixture.write(
        "tools/executor/Cargo.lock",
        "# This file is automatically @generated by Cargo.\n# It is not intended for manual editing.\nversion = 4\n\n[[package]]\nname = \"executor\"\nversion = \"0.1.0\"\n",
    );
    fixture.commit("minimal CAL Cargo build consumers");

    let external = TempDirectory::new("openwepp-cargo-build-cache");
    let attempt_root = external.path.join("attempt");
    prepare_attempt_root(&attempt_root).expect("fresh Cargo attempt root");
    let ledger = external.path.join("ledger.jsonl");
    fs::write(&ledger, []).expect("Cargo fixture ledger");
    let options = ExternalTransitionOptions {
        repo: fixture.root.path.clone(),
        plan_path: fixture.root.path.join("unused-plan.json"),
        transaction_id: "cargo-build-consumers".to_owned(),
        attempt_root: attempt_root.clone(),
        ledger,
        receipt_path: external.path.join("receipt.json"),
        custody_root: None,
        opening_token: None,
        claims: ExecutionClaims::default(),
    };
    let source = repository_identity(&fixture.root.path).expect("clean Cargo source identity");
    let attempt_identity = directory_identity(&attempt_root).expect("Cargo attempt identity");
    let build_executor = ExternalNode {
        order: 1,
        command_id: "build_executor".to_owned(),
        argv: vec![
            "cargo".to_owned(),
            "build".to_owned(),
            "--release".to_owned(),
            "--manifest-path".to_owned(),
            "${REPO}/tools/executor/Cargo.toml".to_owned(),
        ],
        env: BTreeMap::from([(
            "CARGO_TARGET_DIR".to_owned(),
            "${CARGO_TARGET_DIR}".to_owned(),
        )]),
        cwd: fixture.root.path.display().to_string(),
        source_working_directory: fixture.root.path.display().to_string(),
        source_inputs: vec!["executor sources".to_owned()],
        prerequisites: vec![],
        cost_class: "QUICK".to_owned(),
        source_path: "tools/executor/Cargo.toml".to_owned(),
        declared_outputs: vec!["cargo-target/release/executor-bin".to_owned()],
        timeout_seconds: 60,
        max_attempts: 1,
        handoff: "executor consumer".to_owned(),
        harvard_access: "NONE".to_owned(),
    };
    let executor_declarations = vec![PathBuf::from("cargo-target/release/executor-bin")];
    let executor_receipt = execute_node(
        &options,
        &build_executor,
        "LIGHT",
        vec![],
        &executor_declarations,
        &source,
        &attempt_identity,
    )
    .expect("real standalone --manifest-path Cargo build");

    let build_runner = ExternalNode {
        order: 2,
        command_id: "build_production_runner".to_owned(),
        argv: vec![
            "cargo".to_owned(),
            "build".to_owned(),
            "-p".to_owned(),
            "runner".to_owned(),
            "--bin".to_owned(),
            "runner-bin".to_owned(),
        ],
        env: BTreeMap::from([(
            "CARGO_TARGET_DIR".to_owned(),
            "${CARGO_TARGET_DIR}".to_owned(),
        )]),
        cwd: fixture.root.path.display().to_string(),
        source_working_directory: fixture.root.path.display().to_string(),
        source_inputs: vec!["runner sources".to_owned()],
        prerequisites: vec!["build_executor".to_owned()],
        cost_class: "HEAVY".to_owned(),
        source_path: "Cargo.toml".to_owned(),
        declared_outputs: vec!["cargo-target/debug/runner-bin".to_owned()],
        timeout_seconds: 60,
        max_attempts: 1,
        handoff: "runner consumer".to_owned(),
        harvard_access: "NONE".to_owned(),
    };
    let both_build_declarations = vec![
        PathBuf::from("cargo-target/release/executor-bin"),
        PathBuf::from("cargo-target/debug/runner-bin"),
    ];
    let runner_receipt = execute_node(
        &options,
        &build_runner,
        "HEAVY",
        vec![
            digest(&serde_json::to_value(&executor_receipt).expect("executor receipt value"))
                .expect("executor receipt digest"),
        ],
        &both_build_declarations,
        &source,
        &attempt_identity,
    )
    .expect("real workspace package/bin Cargo build");

    for (node, receipt, expected_export) in [
        (
            &build_executor,
            &executor_receipt,
            "cargo-target/release/executor-bin",
        ),
        (
            &build_runner,
            &runner_receipt,
            "cargo-target/debug/runner-bin",
        ),
    ] {
        let cache = receipt
            .build_cache_manifest
            .as_ref()
            .expect("build receipt authenticates isolated cache");
        assert_eq!(
            cache.root,
            build_cache_root(&options, node).display().to_string()
        );
        assert!(
            cache.entries.len() > 1,
            "Cargo intermediates must remain authenticated in the isolated cache"
        );
        verify_historical_manifest(&build_cache_root(&options, node), cache)
            .expect("build-cache manifest independently verifies");
        assert!(attempt_root.join(expected_export).is_file());
        assert!(
            receipt
                .output_manifest
                .entries
                .iter()
                .all(|entry| !entry.path.starts_with("build-cache/"))
        );
    }
    assert_ne!(
        executor_receipt
            .build_cache_manifest
            .as_ref()
            .expect("executor cache")
            .root,
        runner_receipt
            .build_cache_manifest
            .as_ref()
            .expect("runner cache")
            .root
    );

    let executor_consumer = cargo_consumer_node(
        3,
        "consume_executor",
        "${CARGO_TARGET_DIR}/release/executor-bin",
        "tools/executor/src/main.rs",
        "objects/executor-consumed.txt",
        &fixture.root.path,
    );
    let runner_consumer = cargo_consumer_node(
        4,
        "consume_runner",
        "${CARGO_TARGET_DIR}/debug/runner-bin",
        "runner/src/main.rs",
        "objects/runner-consumed.txt",
        &fixture.root.path,
    );
    let mut consumer_declarations = both_build_declarations;
    consumer_declarations.push(PathBuf::from("objects/executor-consumed.txt"));
    execute_node(
        &options,
        &executor_consumer,
        "HEAVY",
        vec![],
        &consumer_declarations,
        &source,
        &attempt_identity,
    )
    .expect("execute staged release consumer");
    consumer_declarations.push(PathBuf::from("objects/runner-consumed.txt"));
    execute_node(
        &options,
        &runner_consumer,
        "HEAVY",
        vec![],
        &consumer_declarations,
        &source,
        &attempt_identity,
    )
    .expect("execute staged debug consumer");
    assert_eq!(
        fs::read_to_string(attempt_root.join("objects/executor-consumed.txt"))
            .expect("executor consumer output"),
        "executor-consumed\n"
    );
    assert_eq!(
        fs::read_to_string(attempt_root.join("objects/runner-consumed.txt"))
            .expect("runner consumer output"),
        "runner-consumed\n"
    );
}

fn cargo_consumer_node(
    order: u64,
    command_id: &str,
    executable: &str,
    source_path: &str,
    output: &str,
    repo: &Path,
) -> ExternalNode {
    ExternalNode {
        order,
        command_id: command_id.to_owned(),
        argv: vec![executable.to_owned(), format!("${{ATTEMPT_ROOT}}/{output}")],
        env: BTreeMap::new(),
        cwd: repo.display().to_string(),
        source_working_directory: repo.display().to_string(),
        source_inputs: vec![],
        prerequisites: vec![],
        cost_class: "HEAVY".to_owned(),
        source_path: source_path.to_owned(),
        declared_outputs: vec![output.to_owned()],
        timeout_seconds: 10,
        max_attempts: 1,
        handoff: "consumer output".to_owned(),
        harvard_access: "NONE".to_owned(),
    }
}

#[test]
fn cal_plan_is_exactly_reconstructed_from_both_csv_authorities() {
    let repo = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../..");
    let plan_path = repo.join(
        "docs/work-packages/20260727-canopy-cal-04b-calibration-readiness-and-ensemble-execution-001/artifacts/external-dag-transaction-plan.json",
    );
    let plan = load_plan(&plan_path).expect("schema-valid CAL external plan");
    reconstruct_source_inventory(&repo, &plan)
        .expect("every transformed argv, environment, prerequisite, and output must reconstruct");
}

#[test]
fn audit_reconstructs_external_inventory_once() {
    let repo = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../..");
    let plan_path = repo.join(
        "docs/work-packages/20260727-canopy-cal-04b-calibration-readiness-and-ensemble-execution-001/artifacts/external-dag-transaction-plan.json",
    );
    let plan = load_plan(&plan_path).expect("schema-valid CAL external plan");
    reset_inventory_reconstruction_count();
    let proof = reconstruct_source_inventory(&repo, &plan)
        .map(|()| json!({"plan_id": plan.plan_id, "node_count": plan.transactions.iter().map(|transaction| transaction.light.len() + transaction.heavy.len()).sum::<usize>()}))
        .expect("one independent inventory reconstruction");

    // RED at the retained clean anchor: construct_audit_report performed this
    // same authority reconstruction twice. The single proof below is the
    // object consumed by the downstream audit check.
    assert_eq!(inventory_reconstruction_count(), 1);
    assert_eq!(proof["plan_id"], plan.plan_id);
    assert!(proof["node_count"].as_u64().is_some_and(|count| count > 0));
}

#[test]
fn audit_admits_ledger_once_and_verifies_without_readmission() {
    let fixture = TempDirectory::new("openwepp-ledger-single-admission");
    let ledger = fixture.path.join("ledger.jsonl");
    fs::write(&ledger, []).expect("empty durable ledger");
    crate::pre_heavy::reset_ledger_admission_count();

    let proof = admit_attempt_ledger_with_proof(&ledger).expect("one canonical admission");
    assert_eq!(crate::pre_heavy::ledger_admission_count(), 1);
    let admitted_bytes = fs::read(&ledger).expect("admitted ledger bytes");
    verify_attempt_ledger_admission_proof(&ledger, &proof).expect("first proof verification");
    verify_attempt_ledger_admission_proof(&ledger, &proof).expect("independent proof verification");

    // RED at the retained clean anchor: the verifier called admission again,
    // producing an observed admission counter of two.
    assert_eq!(crate::pre_heavy::ledger_admission_count(), 1);
    assert_eq!(
        fs::read(&ledger).expect("verified ledger bytes"),
        admitted_bytes
    );
}

#[test]
fn external_csv_rejects_header_drift_and_unknown_columns() {
    let source_repo = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../..");
    let plan_path = source_repo.join(
        "docs/work-packages/20260727-canopy-cal-04b-calibration-readiness-and-ensemble-execution-001/artifacts/external-dag-transaction-plan.json",
    );
    let mut plan = load_plan(&plan_path).expect("schema-valid CAL external plan");
    let fixture = TempDirectory::new("openwepp-external-csv-headers");
    let source_path = source_repo.join(&plan.source_plan.path);
    let contract_path = source_repo.join(&plan.source_contract.path);
    fs::copy(&source_path, fixture.path.join("source.csv")).expect("copy source authority");
    fs::copy(&contract_path, fixture.path.join("contract.csv")).expect("copy contract authority");
    plan.source_plan.path = "source.csv".to_owned();
    plan.source_contract.path = "contract.csv".to_owned();

    let original = fs::read_to_string(fixture.path.join("source.csv")).expect("source CSV");
    let (header, rows) = original.split_once('\n').expect("source CSV header");
    let mut reordered = header.split(',').collect::<Vec<_>>();
    reordered.swap(0, 1);
    fs::write(
        fixture.path.join("source.csv"),
        format!("{}\n{rows}", reordered.join(",")),
    )
    .expect("write reordered header");
    let error = reconstruct_source_inventory(&fixture.path, &plan)
        .expect_err("reordered authority columns must fail before row projection");
    assert_eq!(error.code, "GATE-EXTERNAL-SOURCE-CSV-HEADER");
    assert_eq!(error.class, ErrorClass::Schema);

    fs::write(
        fixture.path.join("source.csv"),
        format!("{header},unadmitted_column\n{rows}"),
    )
    .expect("write unknown header");
    let error = reconstruct_source_inventory(&fixture.path, &plan)
        .expect_err("unknown authority columns must fail before row projection");
    assert_eq!(error.code, "GATE-EXTERNAL-SOURCE-CSV-HEADER");
    assert_eq!(error.class, ErrorClass::Schema);

    fs::write(fixture.path.join("source.csv"), &original).expect("restore source authority");
    let contract = fs::read_to_string(fixture.path.join("contract.csv")).expect("contract CSV");
    let (contract_header, contract_rows) = contract.split_once('\n').expect("contract CSV header");
    fs::write(
        fixture.path.join("contract.csv"),
        format!("{contract_header},unadmitted_column\n{contract_rows}"),
    )
    .expect("write unknown contract header");
    let error = verify_contract_inventory(&fixture.path, &plan, &[])
        .expect_err("unknown contract columns must fail before contract projection");
    assert_eq!(error.code, "GATE-EXTERNAL-CONTRACT-CSV-HEADER");
    assert_eq!(error.class, ErrorClass::Schema);
}

#[test]
fn external_errors_preserve_receipt_custody_ledger_and_identity_classes() {
    // RED at the retained clean anchor: substring-based classification and
    // policy_error collapsed these four independently actionable boundaries.
    let cases = [
        (
            receipt_error("GATE-EXTERNAL-RECEIPT-INVALID", "receipt"),
            ErrorClass::Receipt,
        ),
        (
            custody_error("GATE-EXTERNAL-CUSTODY-INVALID", "custody"),
            ErrorClass::Trust,
        ),
        (
            ledger_error("GATE-EXTERNAL-LEDGER-INVALID", "ledger"),
            ErrorClass::Ledger,
        ),
        (
            identity_error("GATE-EXTERNAL-IDENTITY-INVALID", "identity"),
            ErrorClass::Identity,
        ),
    ];
    for (error, expected) in cases {
        assert_eq!(error.class, expected, "typed boundary for {}", error.code);
    }
}

#[test]
fn rejects_heavy_node_in_light_inventory() {
    let node = ExternalNode {
        order: 1,
        command_id: "population".to_owned(),
        argv: vec!["false".to_owned()],
        env: BTreeMap::new(),
        cwd: "work".to_owned(),
        source_working_directory: "work".to_owned(),
        source_inputs: vec![],
        prerequisites: Vec::new(),
        cost_class: "HEAVY".to_owned(),
        source_path: "tool.py".to_owned(),
        declared_outputs: vec!["objects/result".to_owned()],
        timeout_seconds: 1,
        max_attempts: 1,
        handoff: "none".to_owned(),
        harvard_access: "NONE".to_owned(),
    };
    let error = validate_node(&node, "LIGHT").expect_err("must reject");
    assert_eq!(error.code, "GATE-EXTERNAL-NODE-INVALID");
}

#[test]
fn rejects_parent_path() {
    let error = confined_relative("../escape").expect_err("must reject");
    assert_eq!(error.code, "GATE-EXTERNAL-PATH");
}

#[test]
fn source_inventory_requires_exact_path_working_directory_inputs_and_environment() {
    let repo = Path::new("/home/workdir/openWEPP");
    let mut node = ExternalNode {
        order: 1,
        command_id: "prepare".to_owned(),
        argv: vec!["tools/prepare.py".to_owned()],
        env: BTreeMap::new(),
        cwd: "${REPO}".to_owned(),
        source_working_directory: repo.display().to_string(),
        source_inputs: vec!["first input".to_owned(), "second input".to_owned()],
        prerequisites: vec![],
        cost_class: "QUICK".to_owned(),
        source_path: "tools/prepare.py".to_owned(),
        declared_outputs: vec![],
        timeout_seconds: 1,
        max_attempts: 1,
        handoff: "none".to_owned(),
        harvard_access: "NONE".to_owned(),
    };
    assert_eq!(
        canonical_source_path(repo, "/home/workdir/openWEPP/tools/prepare.py").as_deref(),
        Some("tools/prepare.py")
    );
    assert_eq!(
        canonical_source_path(repo, "/other/tools/prepare.py"),
        None,
        "a matching suffix outside the repository is not the authenticated source"
    );
    assert_eq!(
        canonical_source_working_directory(repo, "/home/workdir/openWEPP"),
        canonical_plan_working_directory(repo, &node.cwd)
    );
    assert_eq!(
        split_source_inventory(" first input ; second input "),
        node.source_inputs
    );
    assert!(
        environment_projection_matches("tools/prepare.py", "default", &node)
            .expect("empty default environment")
    );
    assert!(
        !environment_projection_matches("tools/prepare.py", "UNBOUND=1", &node)
            .expect("parse source environment"),
        "an empty node environment must not vacuously accept arbitrary source environment"
    );
    node.env.insert("BOUND".to_owned(), "1".to_owned());
    assert!(
        environment_projection_matches("tools/prepare.py", "BOUND=1", &node)
            .expect("exact source environment")
    );
}

#[test]
fn source_order_labels_are_bound_to_the_exact_node_order() {
    let rows = vec![
        csv::StringRecord::from(vec!["8", "first"]),
        csv::StringRecord::from(vec!["8a", "second"]),
        csv::StringRecord::from(vec!["9", "third"]),
    ];
    let projected = project_source_orders(&rows).expect("valid canonical source labels");
    assert_eq!(projected["8"], 1);
    assert_eq!(projected["8a"], 2);
    assert_eq!(projected["9"], 3);
    assert_ne!(
        projected["8a"], 8,
        "a suffix label is sequence authority, not a lossy numeric parse"
    );
}

#[test]
fn placeholder_expansion_is_exact_and_unknowns_fail() {
    let options = ExternalTransitionOptions {
        repo: PathBuf::from("/repo"),
        plan_path: PathBuf::from("/plan"),
        transaction_id: "calibration-v1".to_owned(),
        attempt_root: PathBuf::from("/attempt"),
        ledger: PathBuf::from("/ledger"),
        receipt_path: PathBuf::from("/receipt"),
        custody_root: Some(PathBuf::from("/custody")),
        opening_token: None,
        claims: ExecutionClaims::default(),
    };
    assert_eq!(
        expand_operand(&options, "${OBJECTS_ROOT}/x").expect("known placeholder"),
        "/attempt/objects/x"
    );
    let error = expand_operand(&options, "${CALLER_VALUE}").expect_err("must reject");
    assert_eq!(error.code, "GATE-EXTERNAL-PLACEHOLDER-UNKNOWN");
}

#[test]
fn verifier_labels_cannot_fake_independence() {
    let first = attestation("task-a", "alice", "job-a", "cap-a");
    let mut second = attestation("task-b", "bob", "job-b", "cap-b");
    assert!(verify_independent_attestations(&[first.clone(), second.clone()]).is_ok());
    second.principal.clone_from(&first.principal);
    let error = verify_independent_attestations(&[first, second]).expect_err("must reject reuse");
    assert_eq!(error.code, "GATE-EXTERNAL-CUSTODY-INDEPENDENCE");
}

#[test]
fn generation_b_consumes_each_capability_once() {
    let fixture = CustodyFixture::new("holdout-v1");
    let before = fixture.capability_tree();

    let first_import =
        verify_custody_files(&fixture.options, &fixture.transaction, false).expect("pre-LIGHT");
    let second_import = verify_custody_files(&fixture.options, &fixture.transaction, false)
        .expect("repeat preflight");
    assert_eq!(first_import, second_import);
    assert_eq!(
        fixture.capability_tree(),
        before,
        "pre-LIGHT verification must be mutation-free"
    );

    let proof = consume_custody_capabilities(&fixture.options, &fixture.transaction)
        .expect("READY owns the sole capability transition");
    let after_consume = fixture.capability_tree();
    assert_eq!(after_consume.active, Vec::<String>::new());
    assert_eq!(after_consume.consumed.len(), 2);
    assert_eq!(proof["entries"].as_array().map(Vec::len), Some(2));
    assert_eq!(
        verify_consumed_custody_proof(&fixture.options, &fixture.transaction, &proof)
            .expect("HEAVY verifies immutable proof"),
        first_import
    );
    assert_eq!(
        verify_consumed_custody_proof(&fixture.options, &fixture.transaction, &proof)
            .expect("independent verifier verifies without consumption"),
        first_import
    );
    assert_eq!(fixture.capability_tree(), after_consume);

    // RED at the retained clean anchor: the same two capabilities were
    // presented to two mutating calls. GREEN has exactly two filesystem
    // mutations (one rename per capability) and no later mutation.
    assert_eq!(before.active.len() - after_consume.active.len(), 2);
    assert_eq!(after_consume.consumed.len() - before.consumed.len(), 2);
}

#[test]
fn light_failure_preserves_capabilities_and_post_consumption_restart_requires_new_dispatch() {
    let fixture = CustodyFixture::new("holdout-v1");
    let before_light = fixture.capability_tree();
    verify_custody_files(&fixture.options, &fixture.transaction, false)
        .expect("pre-LIGHT custody verification");

    // A LIGHT failure occurs before READY consumption. Its retained terminal
    // snapshot leaves the dispatch reusable because the capability tree did
    // not change.
    fs::write(&fixture.options.ledger, []).expect("empty attempt ledger");
    append_attempt_record(
        &fixture.options.ledger,
        json!({
            "record_type": "EXTERNAL_TRANSACTION",
            "status": "FAILED",
            "stage": "LIGHT",
            "transaction_id": fixture.transaction.transaction_id,
            "cause_key": "GATE-EXTERNAL-SUBPROCESS",
        }),
    )
    .expect("retain failed LIGHT terminal");
    let terminal_snapshot =
        fs::read(&fixture.options.ledger).expect("retained terminal-attempt snapshot");
    let terminal_attempts = terminal_snapshot
        .split(|byte| *byte == b'\n')
        .filter(|line| !line.is_empty())
        .map(|line| parse_strict(line).expect("terminal record"))
        .collect::<Vec<_>>();
    assert_eq!(fixture.capability_tree(), before_light);
    assert_eq!(terminal_attempts.len(), 1);
    assert_eq!(terminal_attempts[0]["status"], "FAILED");

    let proof = consume_custody_capabilities(&fixture.options, &fixture.transaction)
        .expect("later successful LIGHT reaches READY consumption");
    let consumed = fixture.capability_tree();
    verify_consumed_custody_proof(&fixture.options, &fixture.transaction, &proof)
        .expect("consumed attempt remains independently verifiable");
    let error = consume_custody_capabilities(&fixture.options, &fixture.transaction)
        .expect_err("post-consumption restart requires a newly dispatched capability set");
    assert!(
        matches!(
            error.code,
            "GATE-EXTERNAL-CAPABILITY-MISSING" | "GATE-EXTERNAL-CAPABILITY-ALREADY-CONSUMED"
        ),
        "unexpected restart error: {error:?}"
    );
    assert_eq!(error.class, ErrorClass::Trust);
    assert_eq!(fixture.capability_tree(), consumed);
    assert_eq!(
        fs::read(&fixture.options.ledger).expect("unchanged terminal snapshot"),
        terminal_snapshot
    );
}

#[test]
fn generation_b_rejects_stale_verifier_attestation() {
    let fixture = CustodyFixture::new("holdout-v1");
    let stale_path = fixture.custody.path.join("freeze_verify_a.json");
    let mut stale: Value =
        parse_strict(&fs::read(&stale_path).expect("attestation")).expect("attestation JSON");
    stale["transaction_id"] = json!("prior-transaction");
    stale["attestation_id"] =
        json!(derived_id(&stale, "attestation_id").expect("stale attestation identity"));
    fs::write(
        &stale_path,
        canonical_bytes(&stale).expect("canonical stale attestation"),
    )
    .expect("write stale attestation");
    let error = verify_custody_files(&fixture.options, &fixture.transaction, false)
        .expect_err("a prior transaction attestation must fail closed");
    assert_eq!(error.code, "GATE-EXTERNAL-ATTESTATION-FRESHNESS");
    assert_eq!(error.class, ErrorClass::Trust);

    let fresh = CustodyFixture::new("holdout-v1");
    let mut proof = consume_custody_capabilities(&fresh.options, &fresh.transaction)
        .expect("fresh dispatch consumption");
    proof["parent_dispatch_id"] = json!("prior-dispatch");
    proof["proof_id"] = json!(derived_id(&proof, "proof_id").expect("stale proof identity"));
    let error = verify_consumed_custody_proof(&fresh.options, &fresh.transaction, &proof)
        .expect_err("a proof from another dispatch must fail closed");
    assert_eq!(error.code, "GATE-EXTERNAL-ATTESTATION-FRESHNESS");
    assert_eq!(error.class, ErrorClass::Trust);
}

#[test]
fn python_verifier_custody_names_are_exact() {
    assert_eq!(
        verifier_id_from_attestation_path("freeze_verify_a.json").expect("verifier a"),
        "verifier_a"
    );
    assert_eq!(
        verifier_id_from_attestation_path("nested/freeze_verify_b.json").expect("verifier b"),
        "verifier_b"
    );
    let error = verifier_id_from_attestation_path("verifier_a.json")
        .expect_err("noncanonical name must fail");
    assert_eq!(error.code, "GATE-EXTERNAL-CUSTODY-PATH");
}

#[test]
fn verifier_receipt_authenticates_freeze_script_and_command() {
    let options = custody_options();
    let script = options.repo.join(
            "docs/work-packages/20260727-canopy-cal-04b-calibration-readiness-and-ensemble-execution-001/tools/freeze-verify.py",
        );
    let mut attestation = attestation("task-a", "alice", "job-a", &"4".repeat(64));
    attestation.script_sha256 = "1".repeat(64);
    let command = format!(
        "PYTHONDONTWRITEBYTECODE=1 .venv/bin/python {} --execution-root {} --verifier-id verifier_a",
        script
            .strip_prefix(&options.repo)
            .expect("relative")
            .display(),
        options.attempt_root.join("objects").display()
    );
    let row = format!(
        "verifier_id,freeze_digest,verifier_script_sha256,command,command_sha256,timestamp,state\nverifier_a,{},{},{},{},2026-07-27T00:00:00+00:00,PASS\n",
        attestation.freeze_digest,
        attestation.script_sha256,
        command,
        sha256_bytes(command.as_bytes())
    );
    authenticate_receipt_row(
        &options,
        "verifier_a",
        &script,
        &options.attempt_root.join("objects"),
        &attestation,
        row.as_bytes(),
    )
    .expect("Python receipt must authenticate");
    let forged = row.replace(&attestation.freeze_digest, &"9".repeat(64));
    let error = authenticate_receipt_row(
        &options,
        "verifier_a",
        &script,
        &options.attempt_root.join("objects"),
        &attestation,
        forged.as_bytes(),
    )
    .expect_err("foreign freeze must fail");
    assert_eq!(error.code, "GATE-EXTERNAL-CUSTODY-RECEIPT");
}

#[test]
fn holdout_execution_receipt_requires_one_complete_custody_row() {
    let valid = b"state,freeze_digest,token_sha256\nPASS_SCORED_NO_REFIT,freeze,token\n";
    let fields = holdout_execution_fields(valid).expect("one complete receipt row");
    assert_eq!(
        fields.get("freeze_digest").map(String::as_str),
        Some("freeze")
    );
    assert_eq!(
        fields.get("token_sha256").map(String::as_str),
        Some("token")
    );

    let duplicate =
        b"state,freeze_digest,token_sha256\nPASS_SCORED_NO_REFIT,freeze,token\nPASS_SCORED_NO_REFIT,freeze,token\n";
    let error = holdout_execution_fields(duplicate).expect_err("duplicate rows must fail");
    assert_eq!(error.code, "GATE-EXTERNAL-HOLDOUT-RECEIPT");
}

#[test]
fn attestation_argv_must_bind_python_verifier_context() {
    let options = custody_options();
    let custody = Path::new("/custody");
    let script = options.repo.join(
            "docs/work-packages/20260727-canopy-cal-04b-calibration-readiness-and-ensemble-execution-001/tools/freeze-verify.py",
        );
    let mut value = attestation("task-a", "alice", "job-a", &"4".repeat(64));
    value.argv = vec![
        script.display().to_string(),
        "--execution-root".to_owned(),
        options.attempt_root.join("objects").display().to_string(),
        "--custody-root".to_owned(),
        custody.display().to_string(),
        "--verifier-id".to_owned(),
        "verifier_a".to_owned(),
        "--capability".to_owned(),
        custody
            .join("capabilities")
            .join(format!("{}.cap", value.capability_hash))
            .display()
            .to_string(),
        "--attestation-out".to_owned(),
        custody.join("freeze_verify_a.json").display().to_string(),
        "--parent-dispatch-id".to_owned(),
        value.parent_dispatch_id.clone(),
        "--transaction-id".to_owned(),
        value.transaction_id.clone(),
        "--agent-task-id".to_owned(),
        value.agent_task_id.clone(),
        "--principal".to_owned(),
        value.principal.clone(),
        "--workflow".to_owned(),
        value.workflow.clone(),
        "--job".to_owned(),
        value.job.clone(),
        "--runner".to_owned(),
        value.runner.clone(),
        "--attempt".to_owned(),
        value.attempt.to_string(),
    ];
    authenticate_attestation_argv(custody, "verifier_a", &script, &value)
        .expect("exact Python invocation must authenticate");
    value.argv[2] = "relative/objects".to_owned();
    let error = authenticate_attestation_argv(custody, "verifier_a", &script, &value)
        .expect_err("relative execution root must fail");
    assert_eq!(error.code, "GATE-EXTERNAL-CUSTODY-ARGV");
}

fn integration_node(repo: &Path, command_id: &str, order: u64, cost_class: &str) -> Value {
    let (argv, source_inputs, prerequisites, output) = if command_id == "light" {
        (
            vec![
                "${REPO}/tools/light.sh".to_owned(),
                "${OBJECTS_ROOT}/light.txt".to_owned(),
            ],
            vec!["seed"],
            Vec::<&str>::new(),
            "objects/light.txt",
        )
    } else {
        (
            vec![
                "${REPO}/tools/heavy.sh".to_owned(),
                "${OBJECTS_ROOT}/light.txt".to_owned(),
                "${OBJECTS_ROOT}/heavy.txt".to_owned(),
            ],
            vec!["light output"],
            vec!["light"],
            "objects/heavy.txt",
        )
    };
    json!({
        "order": order,
        "command_id": command_id,
        "argv": argv,
        "env": {},
        "cwd": repo.display().to_string(),
        "source_working_directory": repo.display().to_string(),
        "source_inputs": source_inputs,
        "prerequisites": prerequisites,
        "cost_class": cost_class,
        "source_path": format!("tools/{command_id}.sh"),
        "declared_outputs": [output],
        "timeout_seconds": 10,
        "max_attempts": 1,
        "handoff": if command_id == "light" { "READY audit" } else { "terminal receipt" },
        "harvard_access": "NONE",
    })
}

fn integration_options(
    repo: &Path,
    plan_path: &Path,
    attempt_root: &Path,
    ledger: &Path,
    receipt_path: &Path,
    custody_root: &Path,
    attempt: u64,
) -> ExternalTransitionOptions {
    ExternalTransitionOptions {
        repo: repo.to_owned(),
        plan_path: plan_path.to_owned(),
        transaction_id: "generation-b-transition".to_owned(),
        attempt_root: attempt_root.to_owned(),
        ledger: ledger.to_owned(),
        receipt_path: receipt_path.to_owned(),
        custody_root: Some(custody_root.to_owned()),
        opening_token: None,
        claims: ExecutionClaims {
            principal: "integration-test".to_owned(),
            repository: "local/generation-b".to_owned(),
            source_event: "test".to_owned(),
            source_ref: "refs/heads/main".to_owned(),
            workflow: "generation-b-integration".to_owned(),
            job: format!("dispatch-{attempt}"),
            runner: "local-test".to_owned(),
            attempt,
        },
    }
}

fn install_custody_dispatch(
    options: &ExternalTransitionOptions,
    dispatch_id: &str,
    capability_prefix: &[u8],
) {
    let custody = options.custody_root.as_ref().expect("custody root");
    fs::create_dir_all(custody.join("capabilities")).expect("active capability root");
    fs::create_dir_all(custody.join("freeze-receipts")).expect("receipt root");
    let script = options.repo.join(
        "docs/work-packages/20260727-canopy-cal-04b-calibration-readiness-and-ensemble-execution-001/tools/freeze-verify.py",
    );
    for (suffix, task, principal, job) in [
        ("a", "task-a", "alice", "job-a"),
        ("b", "task-b", "bob", "job-b"),
    ] {
        let verifier_id = format!("verifier_{suffix}");
        let mut capability_bytes = capability_prefix.to_vec();
        capability_bytes.extend_from_slice(suffix.as_bytes());
        let capability_hash = sha256_bytes(&capability_bytes);
        fs::write(
            custody
                .join("capabilities")
                .join(format!("{capability_hash}.cap")),
            capability_bytes,
        )
        .expect("capability");
        let mut value = attestation(task, principal, job, &capability_hash);
        value.transaction_id.clone_from(&options.transaction_id);
        value.parent_dispatch_id = dispatch_id.to_owned();
        value.attempt = options.claims.attempt;
        value.script_sha256 = file_sha256(&script).expect("script digest");
        value.argv = verifier_argv(options, &script, &value, &verifier_id);
        let command = format!(
            "PYTHONDONTWRITEBYTECODE=1 .venv/bin/python {} --execution-root {} --verifier-id {verifier_id}",
            script
                .strip_prefix(&options.repo)
                .expect("relative verifier script")
                .display(),
            options.attempt_root.join("objects").display()
        );
        let receipt = format!(
            "verifier_id,freeze_digest,verifier_script_sha256,command,command_sha256,timestamp,state\n{verifier_id},{},{},{},{},2026-07-27T00:00:00+00:00,PASS\n",
            value.freeze_digest,
            value.script_sha256,
            command,
            sha256_bytes(command.as_bytes())
        );
        value.receipt_sha256 = sha256_bytes(receipt.as_bytes());
        let mut attestation_value =
            serde_json::to_value(&value).expect("serialize integration attestation");
        attestation_value["attestation_id"] = json!(
            derived_id(&attestation_value, "attestation_id")
                .expect("integration attestation identity")
        );
        fs::write(
            custody
                .join("freeze-receipts")
                .join(format!("{verifier_id}.csv")),
            receipt,
        )
        .expect("verifier receipt");
        fs::write(
            custody.join(format!("freeze_verify_{suffix}.json")),
            canonical_bytes(&attestation_value).expect("canonical integration attestation"),
        )
        .expect("attestation");
    }
}

fn capability_tree_at(custody_root: &Path) -> CapabilityTree {
    CapabilityTree {
        active: directory_entries(&custody_root.join("capabilities")),
        consumed: directory_entries(&custody_root.join("consumed-capabilities")),
    }
}

fn custody_options() -> ExternalTransitionOptions {
    ExternalTransitionOptions {
        repo: PathBuf::from("/repo"),
        plan_path: PathBuf::from("/plan"),
        transaction_id: "holdout-v1".to_owned(),
        attempt_root: PathBuf::from("/attempt"),
        ledger: PathBuf::from("/ledger"),
        receipt_path: PathBuf::from("/receipt"),
        custody_root: Some(PathBuf::from("/custody")),
        opening_token: None,
        claims: ExecutionClaims::default(),
    }
}

fn attestation(
    task: &str,
    principal: &str,
    job: &str,
    capability: &str,
) -> ExternalVerifierAttestation {
    ExternalVerifierAttestation {
        schema: "openwepp-external-verifier-attestation-v1".to_owned(),
        attestation_id: "0".repeat(64),
        capability_hash: capability.to_owned(),
        parent_dispatch_id: "dispatch".to_owned(),
        transaction_id: "holdout-v1".to_owned(),
        agent_task_id: task.to_owned(),
        principal: principal.to_owned(),
        workflow: "workflow".to_owned(),
        job: job.to_owned(),
        runner: job.to_owned(),
        attempt: 1,
        script_sha256: "1".repeat(64),
        argv: vec!["verify".to_owned()],
        receipt_sha256: "2".repeat(64),
        freeze_digest: "3".repeat(64),
        created_at: time::OffsetDateTime::now_utc()
            .format(&time::format_description::well_known::Rfc3339)
            .expect("format current attestation time"),
    }
}

#[derive(Debug, PartialEq, Eq)]
struct CapabilityTree {
    active: Vec<String>,
    consumed: Vec<String>,
}

struct CustodyFixture {
    _repo: TempDirectory,
    custody: TempDirectory,
    options: ExternalTransitionOptions,
    transaction: ExternalTransaction,
}

impl CustodyFixture {
    fn new(transaction_id: &str) -> Self {
        let repo = TempDirectory::new("openwepp-custody-repository");
        let custody = TempDirectory::new("openwepp-custody-root");
        let script = repo.path.join(
            "docs/work-packages/20260727-canopy-cal-04b-calibration-readiness-and-ensemble-execution-001/tools/freeze-verify.py",
        );
        fs::create_dir_all(script.parent().expect("script parent")).expect("script parent");
        fs::write(&script, b"#!/usr/bin/env python3\n").expect("fixture verifier");
        fs::create_dir_all(custody.path.join("capabilities")).expect("capability root");
        fs::create_dir_all(custody.path.join("freeze-receipts")).expect("receipt root");
        let attempt_root = custody.path.join("attempt");
        fs::create_dir_all(attempt_root.join("objects")).expect("attempt objects");
        let options = ExternalTransitionOptions {
            repo: repo.path.clone(),
            plan_path: repo.path.join("plan.json"),
            transaction_id: transaction_id.to_owned(),
            attempt_root,
            ledger: custody.path.join("ledger.jsonl"),
            receipt_path: custody.path.join("transaction.json"),
            custody_root: Some(custody.path.clone()),
            opening_token: None,
            claims: ExecutionClaims::default(),
        };

        let mut prerequisites = Vec::new();
        for (suffix, task, principal, job, capability_bytes) in [
            ("a", "task-a", "alice", "job-a", b"capability-a".as_slice()),
            ("b", "task-b", "bob", "job-b", b"capability-b".as_slice()),
        ] {
            let verifier_id = format!("verifier_{suffix}");
            let capability_hash = sha256_bytes(capability_bytes);
            fs::write(
                custody
                    .path
                    .join("capabilities")
                    .join(format!("{capability_hash}.cap")),
                capability_bytes,
            )
            .expect("capability");
            let mut value = attestation(task, principal, job, &capability_hash);
            value.transaction_id = transaction_id.to_owned();
            value.script_sha256 = file_sha256(&script).expect("script digest");
            value.argv = verifier_argv(&options, &script, &value, &verifier_id);
            let command = format!(
                "PYTHONDONTWRITEBYTECODE=1 .venv/bin/python {} --execution-root {} --verifier-id {verifier_id}",
                script
                    .strip_prefix(&options.repo)
                    .expect("relative script")
                    .display(),
                options.attempt_root.join("objects").display()
            );
            let receipt = format!(
                "verifier_id,freeze_digest,verifier_script_sha256,command,command_sha256,timestamp,state\n{verifier_id},{},{},{},{},2026-07-27T00:00:00+00:00,PASS\n",
                value.freeze_digest,
                value.script_sha256,
                command,
                sha256_bytes(command.as_bytes())
            );
            value.receipt_sha256 = sha256_bytes(receipt.as_bytes());
            let mut attestation_value =
                serde_json::to_value(&value).expect("serialize attestation fixture");
            attestation_value["attestation_id"] = json!(
                derived_id(&attestation_value, "attestation_id").expect("attestation identity")
            );
            fs::write(
                custody
                    .path
                    .join("freeze-receipts")
                    .join(format!("{verifier_id}.csv")),
                receipt,
            )
            .expect("verifier receipt");
            let relative = format!("freeze_verify_{suffix}.json");
            fs::write(
                custody.path.join(&relative),
                canonical_bytes(&attestation_value).expect("canonical attestation"),
            )
            .expect("attestation");
            prerequisites.push(relative);
        }
        let transaction = ExternalTransaction {
            transaction_id: transaction_id.to_owned(),
            light: vec![],
            heavy: vec![],
            custody_prerequisites: prerequisites,
            custody_receipts: vec![],
        };
        Self {
            _repo: repo,
            custody,
            options,
            transaction,
        }
    }

    fn capability_tree(&self) -> CapabilityTree {
        CapabilityTree {
            active: directory_entries(&self.custody.path.join("capabilities")),
            consumed: directory_entries(&self.custody.path.join("consumed-capabilities")),
        }
    }
}

fn directory_entries(path: &Path) -> Vec<String> {
    if !path.exists() {
        return vec![];
    }
    let mut entries = Vec::new();
    collect_directory_entries(path, path, &mut entries);
    entries.sort();
    entries
}

fn collect_directory_entries(root: &Path, path: &Path, entries: &mut Vec<String>) {
    for entry in fs::read_dir(path).expect("read capability directory") {
        let entry = entry.expect("capability entry");
        let entry_path = entry.path();
        if entry.file_type().expect("capability file type").is_dir() {
            collect_directory_entries(root, &entry_path, entries);
        } else {
            let bytes = fs::read(&entry_path).expect("capability bytes");
            entries.push(format!(
                "{}:{}",
                entry_path
                    .strip_prefix(root)
                    .expect("relative capability path")
                    .display(),
                sha256_bytes(&bytes)
            ));
        }
    }
}

fn verifier_argv(
    options: &ExternalTransitionOptions,
    script: &Path,
    value: &ExternalVerifierAttestation,
    verifier_id: &str,
) -> Vec<String> {
    let custody = options.custody_root.as_ref().expect("custody root");
    vec![
        script.display().to_string(),
        "--execution-root".to_owned(),
        options.attempt_root.join("objects").display().to_string(),
        "--custody-root".to_owned(),
        custody.display().to_string(),
        "--verifier-id".to_owned(),
        verifier_id.to_owned(),
        "--capability".to_owned(),
        custody
            .join("capabilities")
            .join(format!("{}.cap", value.capability_hash))
            .display()
            .to_string(),
        "--attestation-out".to_owned(),
        custody
            .join(format!(
                "freeze_verify_{}.json",
                &verifier_id["verifier_".len()..]
            ))
            .display()
            .to_string(),
        "--parent-dispatch-id".to_owned(),
        value.parent_dispatch_id.clone(),
        "--transaction-id".to_owned(),
        value.transaction_id.clone(),
        "--agent-task-id".to_owned(),
        value.agent_task_id.clone(),
        "--principal".to_owned(),
        value.principal.clone(),
        "--workflow".to_owned(),
        value.workflow.clone(),
        "--job".to_owned(),
        value.job.clone(),
        "--runner".to_owned(),
        value.runner.clone(),
        "--attempt".to_owned(),
        value.attempt.to_string(),
    ]
}

struct TransitionFixture {
    root: TempDirectory,
}

impl TransitionFixture {
    fn new() -> Self {
        let fixture = Self {
            root: TempDirectory::new("openwepp-external-repository"),
        };
        fixture.git(&["init", "-b", "main"]);
        fixture.git(&[
            "config",
            "user.email",
            "external-transition@example.invalid",
        ]);
        fixture.git(&["config", "user.name", "External Transition Test"]);
        fixture
    }

    fn write(&self, relative: &str, contents: &str) {
        let path = self.root.path.join(relative);
        fs::create_dir_all(path.parent().expect("fixture file parent"))
            .expect("create fixture parent");
        fs::write(path, contents).expect("write fixture file");
    }

    fn write_json(&self, relative: &str, value: &Value) {
        let mut bytes = canonical_bytes(value).expect("canonical fixture JSON");
        bytes.push(b'\n');
        let path = self.root.path.join(relative);
        fs::create_dir_all(path.parent().expect("fixture JSON parent"))
            .expect("create fixture JSON parent");
        fs::write(path, bytes).expect("write fixture JSON");
    }

    #[cfg(unix)]
    fn write_executable(&self, relative: &str, contents: &str) {
        use std::os::unix::fs::PermissionsExt;

        self.write(relative, contents);
        let path = self.root.path.join(relative);
        let mut permissions = fs::metadata(&path).expect("script metadata").permissions();
        permissions.set_mode(0o755);
        fs::set_permissions(path, permissions).expect("make fixture script executable");
    }

    #[cfg(not(unix))]
    fn write_executable(&self, relative: &str, contents: &str) {
        self.write(relative, contents);
    }

    fn copy_policy_schema(&self, name: &str) {
        let source = Path::new(env!("CARGO_MANIFEST_DIR"))
            .join("../../gate-policy/v1/schemas")
            .join(name);
        let destination = self.root.path.join("gate-policy/v1/schemas").join(name);
        fs::create_dir_all(destination.parent().expect("schema parent"))
            .expect("create schema parent");
        fs::copy(source, destination).expect("copy policy schema");
    }

    fn binding(&self, relative: &str) -> Value {
        json!({
            "path": relative,
            "sha256": sha256_bytes(
                &fs::read(self.root.path.join(relative)).expect("binding source")
            ),
        })
    }

    fn commit(&self, message: &str) -> String {
        self.git(&["add", "--all"]);
        self.git(&["commit", "-m", message]);
        self.git(&["rev-parse", "HEAD"])
    }

    fn git(&self, arguments: &[&str]) -> String {
        let output = Command::new("git")
            .args(arguments)
            .current_dir(&self.root.path)
            .output()
            .expect("run fixture git");
        assert!(
            output.status.success(),
            "git {arguments:?} failed: {}",
            String::from_utf8_lossy(&output.stderr)
        );
        String::from_utf8(output.stdout)
            .expect("git UTF-8 output")
            .trim()
            .to_owned()
    }
}

struct TempDirectory {
    path: PathBuf,
}

impl TempDirectory {
    fn new(label: &str) -> Self {
        let sequence = TEMP_SEQUENCE.fetch_add(1, Ordering::Relaxed);
        let workspace = Path::new(env!("CARGO_MANIFEST_DIR"))
            .join("../..")
            .canonicalize()
            .expect("workspace root");
        let durable_parent = workspace.parent().expect("durable workspace parent");
        let path = durable_parent.join(format!(".{label}-{}-{sequence}", std::process::id()));
        if path.exists() {
            fs::remove_dir_all(&path).expect("remove stale test directory");
        }
        fs::create_dir(&path).expect("create test directory");
        Self { path }
    }
}

impl Drop for TempDirectory {
    fn drop(&mut self) {
        let _ = fs::remove_dir_all(&self.path);
    }
}
