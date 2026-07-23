use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;
use std::sync::atomic::{AtomicU64, Ordering};

use serde_json::{Value, json};

use super::*;
use crate::planner::{
    NextestInventory, PlanRequest, Planner, PlanningStage, current_execution_context,
    derive_execution_key, derive_plan_id,
};
use crate::repository::observe_committed;

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
        fs::create_dir_all(&root).expect("fixture root");
        for directory in ["gate-policy", "assurance", "tools"] {
            assert!(
                Command::new("cp")
                    .arg("-R")
                    .arg(repository.join(directory))
                    .arg(&root)
                    .status()
                    .expect("copy policy authority")
                    .success()
            );
        }
        fs::create_dir_all(root.join("docs")).expect("docs directory");
        assert!(
            Command::new("cp")
                .arg("-R")
                .arg(repository.join("docs/standards"))
                .arg(root.join("docs"))
                .status()
                .expect("copy standards authority")
                .success()
        );
        fs::copy(repository.join("Cargo.lock"), root.join("Cargo.lock")).expect("copy Cargo.lock");
        fs::create_dir_all(root.join("docs/work-packages/owner")).expect("package directory");
        fs::create_dir_all(root.join("docs/work-packages/owner/prompts/active"))
            .expect("active prompt directory");
        fs::write(
            root.join("docs/work-packages/owner/prompts/active/kickoff.md"),
            "# Kickoff\n",
        )
        .expect("active prompt");
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
        Self::git(&root, &["add", "."]);
        Self::git(&root, &["commit", "-qm", "change"]);
        let head = String::from_utf8(Self::git_output(&root, &["rev-parse", "HEAD"]))
            .expect("UTF-8 head")
            .trim()
            .to_owned();
        let authority = crate::package_validation::validate_package_chain(
            &root,
            &base,
            Some(&head),
            Path::new("docs/work-packages/owner/package.md"),
        )
        .expect("fixture package authority");
        let artifacts = root.join("target/artifacts");
        fs::create_dir_all(&artifacts).expect("artifact root");
        let ledger = repository.join("target").join(format!(
            "pre-heavy-coverage-ledger-{}-{sequence}.jsonl",
            std::process::id()
        ));
        fs::write(&ledger, "").expect("durable ledger");
        let mut plan = json!({
            "source": {"base_commit": base, "head_commit": head},
            "authorized_paths": ["docs/work-packages/owner/package.md", "src/lib.rs"],
            "package_authority": {
                "chain_id": authority["package_authority_chain_id"],
                "intent_package_path": "docs/work-packages/owner/package.md"
            },
            "changed_objects": [], "nodes": [],
            "plan_id": "0".repeat(64), "execution_key": "0".repeat(64),
            "execution_context": current_execution_context(&root).expect("execution context"),
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
        plan["plan_id"] = json!(derive_plan_id(&plan).expect("plan ID"));
        plan["execution_key"] = json!(derive_execution_key(&plan).expect("execution key"));
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
            format!("# Owner\n\nStatus: ACTIVE\n\n{note}\n\n## Declared Write Set\n\n- `docs/work-packages/**`\n- `src/**`\n"),
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

fn characterize_constructed_audit_resume(
    fixture: &AuditFixture,
    audit: &Value,
    claims: &ExecutionClaims,
) {
    let constructed = ConstructedAudit(audit.clone());
    let candidate = crate::resume::load_candidate_after_ready_audit(
        &fixture.root,
        &fixture.plan,
        &fixture.ledger,
        claims,
        &constructed,
        "",
    )
    .expect("constructed READY audit admits resume discovery");
    assert!(
        candidate.is_none(),
        "empty ledger has no recovery candidate"
    );

    let mut blocked = audit.clone();
    blocked["status"] = json!("BLOCKED");
    let error = crate::resume::load_candidate_after_ready_audit(
        &fixture.root,
        &fixture.plan,
        &fixture.ledger,
        claims,
        &ConstructedAudit(blocked),
        "",
    )
    .err()
    .expect("non-READY constructed audit must fail");
    assert_eq!(error.code, "GATE-RESUME-AUDIT-BINDING");

    let mut wrong_plan_id = fixture.plan.clone();
    wrong_plan_id["plan_id"] = json!("f".repeat(64));
    let error = crate::resume::load_candidate_after_ready_audit(
        &fixture.root,
        &wrong_plan_id,
        &fixture.ledger,
        claims,
        &constructed,
        "",
    )
    .err()
    .expect("plan substitution must fail");
    assert_eq!(error.code, "GATE-RESUME-AUDIT-BINDING");

    let mut wrong_plan_digest = fixture.plan.clone();
    wrong_plan_digest["resume_probe"] = json!(true);
    let error = crate::resume::load_candidate_after_ready_audit(
        &fixture.root,
        &wrong_plan_digest,
        &fixture.ledger,
        claims,
        &constructed,
        "",
    )
    .err()
    .expect("plan digest substitution must fail");
    assert_eq!(error.code, "GATE-RESUME-AUDIT-BINDING");
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

    build_audit(
        &fixture.root,
        &fixture.plan,
        &receipt,
        &fixture.artifacts,
        &fixture.ledger,
    )
    .expect("public audit construction wrapper");
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
    characterize_constructed_audit_resume(&fixture, &audit, &claims);

    validate_execution_claim_binding(&audit, &claims).expect("claim binding");
    validate_audit_for_execution(
        &fixture.root,
        &fixture.plan,
        &audit,
        &fixture.artifacts,
        &claims,
    )
    .expect("execution validation wrapper");

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

#[test]
fn low_coverage_binding_helpers_exercise_their_reject_arms() {
    let fixture = AuditFixture::new();
    let mut audit = json!({"audit_id": "0".repeat(64)});
    audit["audit_id"] = json!(derived_id(&audit, "audit_id").expect("audit ID"));
    validate_sealed_audit_identity(&audit).expect("sealed identity");
    audit["audit_id"] = json!("f".repeat(64));
    assert!(validate_sealed_audit_identity(&audit).is_err());

    let plan = json!({
        "nodes": [{"node_id": "n", "execution_cost_class": "LIGHT"}]
    });
    let manifest = node_manifest(&plan).expect("node manifest");
    assert_eq!(manifest.as_array().map(Vec::len), Some(1));
    assert!(node_manifest(&json!({"nodes": [{"node_id": 1}]})).is_err());
    assert!(validate_current_audit_inventory(&plan, &json!({"node_manifest": manifest})).is_ok());
    assert!(validate_current_audit_inventory(&plan, &json!({"node_manifest": []})).is_err());

    let binary = current_executable_sha256().expect("binary digest");
    validate_executor_binary_binding(&json!({"executor_binary_sha256": binary}))
        .expect("binary binding");
    assert!(
        validate_executor_binary_binding(&json!({"executor_binary_sha256": "0".repeat(64)}))
            .is_err()
    );
    let claims = ExecutionClaims {
        workflow: "w".to_owned(),
        job: "j".to_owned(),
        runner: "r".to_owned(),
        attempt: 1,
        ..ExecutionClaims::default()
    };
    assert!(
        validate_execution_claim_binding(
            &json!({"light_receipt": {"claims": {"workflow": "other"}}}),
            &claims
        )
        .is_err()
    );
    assert!(require_executor_binary_digest(&json!({"executor_binary_sha256": "short"})).is_err());
    assert!(
        artifact_identity(
            &json!({"artifact_root_sha256": "0".repeat(64)}),
            &fixture.artifacts
        )
        .is_err()
    );

    fs::write(fixture.root.join("src/lib.rs"), "pub fn changed() { } \n")
        .expect("whitespace defect");
    assert!(require_clean_diff(&fixture.root, &fixture.plan).is_err());
    fs::write(
        &fixture.ledger,
        "{\"previous_entry_sha256\":\"wrong\",\"entry_sha256\":\"wrong\"}\n",
    )
    .expect("malformed chain");
    assert!(verify_ledger_chain(&fixture.ledger).is_err());

    for (decision, definitions, expected) in [
        (
            json!({"decision": "COMBINED", "accepted_proof_id": "p", "proof_sha256": "s", "baseline_count": 3}),
            ["combined-workspace-quality-v1"].into_iter().collect(),
            true,
        ),
        (json!({"decision": "UNKNOWN"}), BTreeSet::new(), false),
    ] {
        assert_eq!(
            combined_decision_is_valid(&decision, &definitions),
            expected
        );
    }
}

#[test]
fn exact_planner_output_reconstructs_through_the_public_audit_path() {
    let repo = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../..");
    let source = observe_committed(&repo, "HEAD^", "HEAD").expect("committed source");
    let authorized_paths = source
        .changes
        .iter()
        .map(|change| change.path.clone())
        .collect();
    let request = PlanRequest {
        stage: PlanningStage::Intent,
        predecessor_intent_plan_id: None,
        boundary: "INCREMENT".to_owned(),
        campaign_id: Some("CQR-PRE-HEAVY-COVERAGE".to_owned()),
        combined_quality_proof_id: None,
        authorized_paths,
        package_authority_chain_id: "aa".repeat(32),
        intent_package_path: "docs/work-packages/fixture/package.md".to_owned(),
        source,
    };
    let plan = Planner::new(NextestInventory)
        .build(&repo, &request)
        .expect("canonical planner output");
    let artifacts = repo.join("target/pre-heavy-exact-reconstruction");
    fs::create_dir_all(artifacts.join(".work")).expect("artifact work root");
    reconstruct_exact_plan(&repo, &plan, &artifacts).expect("exact audit reconstruction");
    fs::remove_dir_all(artifacts).expect("remove artifact root");
}
