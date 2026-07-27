use super::*;
use std::sync::atomic::{AtomicU64, Ordering};

static TEMP_SEQUENCE: AtomicU64 = AtomicU64::new(0);

#[test]
fn real_subprocess_transition_persists_light_ready_heavy_and_verifies() {
    let fixture = TransitionFixture::new();
    let package = "docs/work-packages/integration/package.md";
    fixture.write(
        package,
        "# Integration\n\nStatus: `ACTIVE`\n\n## Intended Write Set\n\n- `evidence/**`\n- `plan/**`\n- `tools/**`\n",
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
            "order,command_id,source_path,command,environment,working_directory,inputs,outputs,harvard_access,cost_class\n\
             1,light,tools/light.sh,tools/light.sh ${{OBJECTS_ROOT}}/light.txt,default,{},seed,light.txt,FORBIDDEN,QUICK\n\
             2,heavy,tools/heavy.sh,tools/heavy.sh ${{OBJECTS_ROOT}}/light.txt ${{OBJECTS_ROOT}}/heavy.txt,default,{},light output,heavy.txt,FORBIDDEN,HEAVY\n",
            fixture.root.path.display(),
            fixture.root.path.display(),
        ),
    );
    fixture.write(
        "plan/contract.csv",
        concat!(
            "command_id,prerequisites,outputs\n",
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
        created_at: "2026-07-27T00:00:00Z".to_owned(),
    }
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
