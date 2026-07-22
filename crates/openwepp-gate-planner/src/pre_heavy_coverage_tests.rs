use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;
use std::sync::atomic::{AtomicU64, Ordering};

use serde_json::{Value, json};

use super::*;

static SEQUENCE: AtomicU64 = AtomicU64::new(0);

struct AuditFixture {
    root: PathBuf,
    plan: Value,
    artifacts: PathBuf,
    ledger: PathBuf,
}

impl AuditFixture {
    fn new() -> Self {
        let sequence = SEQUENCE.fetch_add(1, Ordering::Relaxed);
        let root = std::env::temp_dir().join(format!(
            "openwepp-pre-heavy-coverage-{}-{sequence}",
            std::process::id()
        ));
        let repository = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../..");
        fs::create_dir_all(root.join("gate-policy/v1/schemas")).expect("schema directory");
        for schema in [
            "package-audit.schema.json",
            "pre-heavy-audit.schema.json",
            "stage-receipt.schema.json",
        ] {
            fs::copy(
                repository.join("gate-policy/v1/schemas").join(schema),
                root.join("gate-policy/v1/schemas").join(schema),
            )
            .expect("copy schema");
        }
        fs::create_dir_all(root.join("docs/work-packages/owner")).expect("package directory");
        fs::create_dir_all(root.join("src")).expect("source directory");
        Self::write_package(&root, "base");
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
        Self::write_package(&root, "changed");
        fs::write(root.join("src/lib.rs"), "pub fn changed() {}\n").expect("changed source");
        let artifacts = root.join("target/artifacts");
        fs::create_dir_all(&artifacts).expect("artifact root");
        let ledger = repository.join("target").join(format!(
            "pre-heavy-coverage-ledger-{}-{sequence}.jsonl",
            std::process::id()
        ));
        fs::write(&ledger, "").expect("durable ledger");
        let plan = json!({
            "source": {"base_commit": base},
            "authorized_paths": ["docs/work-packages/owner/package.md", "src/lib.rs"],
            "changed_objects": [], "nodes": [],
            "plan_id": "2".repeat(64), "execution_key": "4".repeat(64),
            "execution_context": {
                "configuration_sha256": "a".repeat(64),
                "environment_manifest_sha256": "b".repeat(64),
                "fixture_manifest_sha256": "c".repeat(64),
                "tool_manifest_sha256": "d".repeat(64)
            },
            "environment_roots": {
                "execution_root": "/execution", "authority_root": "/authority",
                "documentation_root": "/documentation"
            },
            "combined_quality": {
                "decision": "NOT_APPLICABLE", "reason_code": "NO_GLOBAL_QUALITY",
                "requested_proof_id": null, "accepted_proof_id": null,
                "proof_sha256": null, "baseline_count": 0
            }
        });
        Self {
            root,
            plan,
            artifacts,
            ledger,
        }
    }

    fn write_package(root: &Path, note: &str) {
        fs::write(
            root.join("docs/work-packages/owner/package.md"),
            format!("# Owner\n\n{note}\n\n## Declared Write Set\n\n- `docs/work-packages/**`\n- `src/**`\n"),
        )
        .expect("package");
    }

    fn git(root: &Path, arguments: &[&str]) {
        assert!(
            Command::new("git")
                .args(arguments)
                .current_dir(root)
                .status()
                .expect("git")
                .success()
        );
    }

    fn git_output(root: &Path, arguments: &[&str]) -> Vec<u8> {
        Command::new("git")
            .args(arguments)
            .current_dir(root)
            .output()
            .expect("git")
            .stdout
    }
}

impl Drop for AuditFixture {
    fn drop(&mut self) {
        fs::remove_dir_all(&self.root).expect("remove fixture");
        fs::remove_file(&self.ledger).expect("remove ledger");
    }
}

#[test]
fn ready_audit_validation_execution_and_resume_chains_are_directly_bound() {
    let fixture = AuditFixture::new();
    let repository = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../..");
    let mut receipt =
        read_json(&repository.join("gate-policy/v1/fixtures/valid/stage-receipt.json"))
            .expect("receipt fixture");
    receipt["plan_id"] = fixture.plan["plan_id"].clone();
    receipt["plan_sha256"] = json!(digest(&fixture.plan).expect("plan digest"));
    receipt["execution_key"] = fixture.plan["execution_key"].clone();
    receipt["artifact_root_sha256"] = json!(path_digest(&fixture.artifacts));
    receipt["executor_binary_sha256"] = json!(current_executable_sha256().expect("binary digest"));
    receipt["claims"] = json!({"workflow": "w", "job": "j", "runner": "r", "attempt": 1});
    receipt["stage_receipt_id"] = json!("0".repeat(64));
    receipt["stage_receipt_id"] =
        json!(derived_id(&receipt, "stage_receipt_id").expect("receipt ID"));
    let admission = package_admission(&fixture.root, &fixture.plan).expect("package admission");
    let checks = CHECK_IDS
        .iter()
        .map(|id| check(id, Ok(()), json!({"id": id})).expect("check"))
        .collect();
    let audit = audit_document(AuditDocumentInputs {
        plan: &fixture.plan,
        light_receipt: &receipt,
        artifact_root: &fixture.artifacts,
        ledger: &fixture.ledger,
        ledger_head_sha256: None,
        package_admission: admission,
        checks,
        combined_execution: fixture.plan["combined_quality"].clone(),
        status: "READY",
        reason_codes: Vec::new(),
    })
    .and_then(|audit| seal_audit(&fixture.root, audit))
    .expect("sealed audit");

    build_unsealed_audit(
        &fixture.root,
        &fixture.plan,
        &receipt,
        &fixture.artifacts,
        &fixture.ledger,
    )
    .expect("unsealed audit chain");
    validate_audit(&fixture.root, &fixture.plan, &audit, &fixture.artifacts)
        .expect("READY audit validation chain");
    let claims = ExecutionClaims {
        workflow: "w".to_owned(),
        job: "j".to_owned(),
        runner: "r".to_owned(),
        attempt: 1,
        ..ExecutionClaims::default()
    };
    validate_execution_claim_binding(&audit, &claims).expect("claim binding");
    assert!(
        validate_audit_for_execution(
            &fixture.root,
            &fixture.plan,
            &audit,
            &fixture.artifacts,
            &claims
        )
        .is_err()
    );

    let started = append_attempt_record(
        &fixture.ledger,
        json!({
            "record_type": "STAGE_ATTEMPT", "status": "STARTED", "stage": "HEAVY",
            "phase": "ADMISSION", "plan_id": fixture.plan["plan_id"], "audit_id": audit["audit_id"],
            "artifact_root": fixture.artifacts.display().to_string(), "workflow": "w", "job": "j",
            "runner": "r", "attempt": 1
        }),
    )
    .expect("STARTED record");
    validate_resume_ledger(
        &fixture.root,
        &fixture.plan,
        &audit,
        &fixture.artifacts,
        &fixture.ledger,
        &started,
        &claims,
    )
    .expect("resume chain");

    let mut invalid = receipt;
    invalid["stage_receipt_id"] = json!("f".repeat(64));
    assert_eq!(
        validate_stage_receipt_plan_binding(&fixture.plan, &invalid)
            .expect_err("invalid receipt identity")
            .code,
        "GATE-AUDIT-STAGE-RECEIPT-IDENTITY"
    );
}
