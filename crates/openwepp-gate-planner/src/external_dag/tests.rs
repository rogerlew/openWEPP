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
        "order,command_id,source_path,command,environment,reserved_a,reserved_b,outputs,harvard_access,cost_class\n\
             1,light,tools/light.sh,tools/light.sh ${OBJECTS_ROOT}/light.txt,default,-,-,light.txt,FORBIDDEN,QUICK\n\
             2,heavy,tools/heavy.sh,tools/heavy.sh ${OBJECTS_ROOT}/light.txt ${OBJECTS_ROOT}/heavy.txt,default,-,-,heavy.txt,FORBIDDEN,HEAVY\n",
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
        let path = std::env::temp_dir().join(format!("{label}-{}-{sequence}", std::process::id()));
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
