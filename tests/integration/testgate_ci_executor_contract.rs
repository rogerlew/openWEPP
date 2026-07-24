use std::fs;
use std::path::{Path, PathBuf};
use std::process::{Command, Output};
use std::sync::atomic::{AtomicU64, Ordering};

use openwepp_gate_planner::Result;
use openwepp_gate_planner::planner::{InventoryProvider, PlanRequest, Planner, PlanningStage};
use openwepp_gate_planner::policy::GateDefinition;
use openwepp_gate_planner::repository::{ObservedChange, ObservedSource};
use serde_json::Value;

#[derive(Clone, Copy)]
struct FixedInventory;

impl InventoryProvider for FixedInventory {
    fn inventory(
        &self,
        _repo: &Path,
        definition: &GateDefinition,
        target: &str,
    ) -> Result<Vec<String>> {
        Ok(vec![format!("{}:{target}", definition.gate_definition_id)])
    }
}

fn root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
}

fn text(path: &str) -> String {
    fs::read_to_string(root().join(path)).expect("contract source must be readable")
}

static SCRATCH_COUNTER: AtomicU64 = AtomicU64::new(0);

#[test]
fn multi_package_inventory_follows_expanded_node_packages() {
    let repo = root();
    let head = Command::new("git")
        .args(["rev-parse", "HEAD"])
        .current_dir(&repo)
        .output()
        .expect("git rev-parse");
    let head = String::from_utf8(head.stdout)
        .expect("UTF-8 head")
        .trim()
        .to_owned();
    let path = "crates/openwepp-management-schema/src/lib.rs";
    let plan = Planner::new(FixedInventory)
        .build(
            &repo,
            &PlanRequest {
                stage: PlanningStage::Intent,
                predecessor_intent_plan_id: None,
                boundary: "INCREMENT".to_owned(),
                campaign_id: Some("CANOPY-PHENOLOGY-02".to_owned()),
                combined_quality_proof_id: None,
                authorized_paths: vec![path.to_owned()],
                package_authority_chain_id: "aa".repeat(32),
                intent_package_path: "docs/work-packages/fixture/package.md".to_owned(),
                source: ObservedSource {
                    base_commit: head,
                    head_commit: None,
                    dirty_tree_digest: Some("11".repeat(32)),
                    index_digest: Some("22".repeat(32)),
                    worktree_digest: Some("33".repeat(32)),
                    untracked_digest: Some("44".repeat(32)),
                    changes: vec![ObservedChange {
                        path: path.to_owned(),
                        change_kind: "MODIFY".to_owned(),
                        object_kind: "REGULAR".to_owned(),
                        old_mode: Some("100644".to_owned()),
                        new_mode: Some("100644".to_owned()),
                    }],
                },
            },
        )
        .expect("native canopy management plan");
    let node = plan["nodes"]
        .as_array()
        .expect("nodes")
        .iter()
        .find(|node| node["gate_definition_id"] == "hard-invariant-native-canopy-management-v1")
        .expect("management A1 node");
    let arguments = node["arguments"]
        .as_array()
        .expect("arguments")
        .iter()
        .map(|argument| argument.as_str().expect("string argument"))
        .collect::<Vec<_>>();
    let packages = arguments
        .windows(2)
        .filter_map(|window| (window[0] == "--package").then_some(window[1]))
        .collect::<Vec<_>>();
    assert_eq!(
        packages,
        [
            "openwepp-management-schema",
            "openwepp-input-contract",
            "openwepp-landuse-migrate"
        ]
    );
    let expected = node["expected_inventory"]["ids"]
        .as_array()
        .expect("inventory IDs");
    assert_eq!(expected.len(), packages.len());
    for package in packages {
        assert!(
            expected.iter().any(|id| {
                id == &format!("hard-invariant-native-canopy-management-v1:{package}")
            })
        );
    }
}

struct Scratch {
    path: PathBuf,
}

impl Scratch {
    fn new() -> Self {
        let count = SCRATCH_COUNTER.fetch_add(1, Ordering::Relaxed);
        let path = std::env::temp_dir().join(format!(
            "openwepp-testgate-output-{}-{count}",
            std::process::id()
        ));
        fs::create_dir(&path).expect("create unique TESTGATE output scratch");
        Self { path }
    }
}

impl Drop for Scratch {
    fn drop(&mut self) {
        fs::remove_dir_all(&self.path).expect("remove TESTGATE output scratch");
    }
}

fn run_crap_path_probe(
    script: &Path,
    working_directory: &Path,
    output_dir: Option<&Path>,
    artifact_root: Option<&Path>,
) -> Output {
    let mut command = Command::new("bash");
    command
        .arg(script)
        .current_dir(working_directory)
        .env_remove("OPENWEPP_GATE_ARTIFACT_ROOT");
    if let Some(output_dir) = output_dir {
        command.arg("--output-dir").arg(output_dir);
    }
    if let Some(artifact_root) = artifact_root {
        command.env("OPENWEPP_GATE_ARTIFACT_ROOT", artifact_root);
    }
    command.output().expect("run CRAP output-resolution probe")
}

fn write_executable(path: &Path, contents: &str) {
    use std::os::unix::fs::PermissionsExt;

    fs::write(path, contents).expect("write executable probe");
    fs::set_permissions(path, fs::Permissions::from_mode(0o755)).expect("make probe executable");
}

fn run_executor_crap_probe(
    artifact_root: &Path,
    nextest_config: Option<&Path>,
    temporary: Option<&Path>,
) -> Output {
    let mut command = Command::new("bash");
    command
        .arg(root().join("tools/release/run_adjudicated_crap_gate.sh"))
        .arg("--output-dir")
        .arg("nested-crap")
        .current_dir(root())
        .env("OPENWEPP_GATE_ARTIFACT_ROOT", artifact_root)
        .env_remove("OPENWEPP_GATE_NEXTEST_CONFIG")
        .env_remove("TMPDIR");
    if let Some(nextest_config) = nextest_config {
        command.env("OPENWEPP_GATE_NEXTEST_CONFIG", nextest_config);
    }
    if let Some(temporary) = temporary {
        command.env("TMPDIR", temporary);
    }
    command.output().expect("run executor CRAP contract probe")
}

struct ShortProcessTemp {
    path: PathBuf,
}

impl ShortProcessTemp {
    fn directory(mode: u32) -> Self {
        use std::os::unix::fs::PermissionsExt;

        let count = SCRATCH_COUNTER.fetch_add(1, Ordering::Relaxed);
        let path = PathBuf::from(format!("/tmp/owg-{}-{count}", std::process::id()));
        fs::create_dir(&path).expect("create short executor temporary directory");
        fs::set_permissions(&path, fs::Permissions::from_mode(mode))
            .expect("set executor temporary directory mode");
        Self { path }
    }

    fn long_directory() -> Self {
        let count = SCRATCH_COUNTER.fetch_add(1, Ordering::Relaxed);
        let path = PathBuf::from(format!(
            "/tmp/owg-{}{:020}-{:020}",
            std::process::id(),
            count,
            count
        ));
        fs::create_dir(&path).expect("create overlong executor temporary directory");
        Self { path }
    }

    fn hexadecimal_directory() -> Self {
        use std::os::unix::fs::PermissionsExt;

        let count = SCRATCH_COUNTER.fetch_add(1, Ordering::Relaxed);
        let path = PathBuf::from(format!("/tmp/owg-{}-{:x}", std::process::id(), count + 10));
        fs::create_dir(&path).expect("create hexadecimal executor temporary directory");
        fs::set_permissions(&path, fs::Permissions::from_mode(0o700))
            .expect("set hexadecimal executor temporary directory mode");
        Self { path }
    }

    fn symlink(target: &Path) -> Self {
        let count = SCRATCH_COUNTER.fetch_add(1, Ordering::Relaxed);
        let path = PathBuf::from(format!("/tmp/owg-{}-{count}", std::process::id()));
        std::os::unix::fs::symlink(target, &path).expect("create executor temporary symlink");
        Self { path }
    }
}

impl Drop for ShortProcessTemp {
    fn drop(&mut self) {
        match fs::symlink_metadata(&self.path) {
            Ok(metadata) if metadata.file_type().is_symlink() => {
                fs::remove_file(&self.path).expect("remove executor temporary symlink");
            }
            Ok(_) => {
                fs::remove_dir_all(&self.path).expect("remove executor temporary directory");
            }
            Err(error) if error.kind() == std::io::ErrorKind::NotFound => {}
            Err(error) => panic!("inspect executor temporary path: {error}"),
        }
    }
}

fn assert_failed_after_output_resolution(output: &Output, resolved_output: &Path) {
    assert!(!output.status.success());
    assert!(
        resolved_output.join("run-status.json").is_file(),
        "resolved output was not initialized: {}\nstderr: {}",
        resolved_output.display(),
        String::from_utf8_lossy(&output.stderr)
    );
}

fn assert_receipt_runtime_guards() {
    let executor = text("crates/openwepp-gate-planner/src/executor.rs");
    for required in [
        "reconstruct_plan_in(",
        ".env_clear()",
        "Command::new(program)",
        "GATE-EXEC-INVENTORY-DRIFT",
        "GATE-EXEC-SOURCE-MUTATION",
        "GATE-EXEC-OUTPUT-COLLISION",
        "GATE-EXEC-UNSUPPORTED",
        "observed_source_snapshot",
        "OPENWEPP_GATE_ARTIFACT_ROOT",
        "CARGO_TARGET_DIR",
        "PREREQUISITE_NONPASS",
        "PROCESS_SPAWN_FAILED",
        "TIMEOUT",
        "SOURCE_MUTATION_DETECTED",
        "TEST_NOT_EXECUTED",
        "GATE-EXEC-SHELL-STRING",
        "GATE-EXEC-HEAVY-REQUIRES-AUDIT",
        "write_node_checkpoint",
    ] {
        assert!(
            executor.contains(required),
            "missing executor guard: {required}"
        );
    }
    assert!(!executor.contains("sh -c"));
    let cli = text("crates/openwepp-gate-planner/src/main.rs");
    assert!(cli.contains("verify_receipt(repo, &plan, &receipt, &artifacts)"));
    assert!(cli.contains("pre-heavy-audit"));
    assert!(cli.contains("validate-package-chain"));
    assert!(cli.contains("package_authority_chain_id"));
    assert!(cli.contains("Some(\"FAIL\" | \"BLOCKED\" | \"INVALID\")"));
    assert!(cli.contains("reconcile_orphaned_attempts(ledger).map(|_| ())"));
    assert!(cli.contains("&context.started_entry_sha256"));
    assert!(cli.contains("verify_receipt_after_ready_audit("));
    assert!(cli.contains("trusted_transition_command("));
    assert!(cli.contains("GATE-EXEC-AUDIT-UNAUTHENTICATED"));
    assert!(cli.contains(
        "load_candidate_after_ready_audit(repo, plan, ledger, claims, audit, started_entry_sha256)"
    ));
    let final_context_check = executor
        .rfind("validate_current_execution_context(&repository, plan)?")
        .expect("final HEAVY context check");
    let execution_spawn = executor
        .find("let execution = execute_nodes_for(")
        .expect("execution boundary");
    assert!(
        final_context_check < execution_spawn,
        "current context must be rechecked at the final HEAVY execution boundary"
    );

    let pre_heavy = text("crates/openwepp-gate-planner/src/pre_heavy.rs");
    for required in [
        "documentation_scope_is_exact(plan)",
        "reconstruct_plan_in(",
        "ledger_head_sha256",
        "GATE-AUDIT-LEDGER-SUCCESSOR",
        "no_open_tooling_defect_at_head",
        "current_execution_context(repo)",
        "reconstructed_plan_is_exact",
        "pub struct ConstructedAudit(Value)",
    ] {
        assert!(
            pre_heavy.contains(required),
            "missing pre-heavy closure guard: {required}"
        );
    }
    assert!(!pre_heavy.contains("Command::new(\"markdown-doc\")"));

    let verifier = text("crates/openwepp-gate-planner/src/verifier.rs");
    for required in [
        "GATE-RECEIPT-EXECUTED-INVENTORY",
        "GATE-RECEIPT-UNAVAILABLE",
        "GATE-RECEIPT-PREREQUISITE",
        "verifier_accepts_truthful_fail_and_blocked_receipts",
        "verify_receipt_after_ready_audit",
    ] {
        assert!(
            verifier.contains(required),
            "missing verifier proof: {required}"
        );
    }
}

fn assert_testgate_workflow_surface() {
    let workflow = text(".github/workflows/testgate-shadow.yml");
    for context in [
        "increment-gates:",
        "verify-increment:",
        "name: openwepp/verify-increment",
        "name: openwepp/increment-gates",
        "name: openwepp/execute-increment",
        "runs-on: [self-hosted, Linux, X64, openwepp, forest1, trusted]",
        "runs-on: ubuntu-24.04",
        "bootstrap_dependencies.sh",
        "tools/local_ci/testgate.py",
        "--boundary INCREMENT",
        "actions/attest@f7c74d28b9d84cb8768d0b8ca14a4bac6ef463e6",
        "attestation-predicate.json",
        "github-attestation.jsonl",
        "gh attestation verify",
        "verify-receipt-envelope",
        "Independently admit comparison base",
        "_intent_authorization",
        "--job openwepp/execute-increment",
        "--signer-workflow",
        "--source-digest",
        "--deny-self-hosted-runners",
        "actions/download-artifact@d3f86a106a0bac45b974a628896c90dbdf5c8093",
        "id-token: write",
        "attestations: write",
        "cargo-nextest@0.9.138",
        "ripgrep 14.1.0",
        "--artifact-root \"${EVIDENCE_DIR}/execution\"",
        "CARGO_TARGET_DIR: /t",
        "TESTGATE_EXECUTION_ROOT: /t",
        "evidence_dir=\"${TESTGATE_EXECUTION_ROOT}/e\"",
        "planner_tmp=\"${TESTGATE_EXECUTION_ROOT}/p\"",
        "TMPDIR=\"${planner_tmp}\"",
        "orchestration-error.log",
        "if: ${{ always() }}",
        "group: openwepp-forest1-testgate",
        "queue: single",
        "cancel-in-progress: false",
        "Reject superseded head before gate execution",
        "Reject superseded head before authority",
        "Reject superseded head after authority verification",
        "ERROR: superseded TESTGATE head",
        "ERROR: refusing to verify superseded head",
        "ERROR: refusing authority for superseded head",
        "ERROR: refusing aggregate success for superseded head",
        "Re-ingest exact durable attempt archive",
        "TESTGATE_HISTORY_ROOT: /testgate-history",
        "OPENWEPP_GATE_CHECKPOINT_MIRROR_ROOT:",
        "Finalize pre-receipt recovery evidence",
        "Restore and verify newest durable attempt history",
        "--history-ledger \"${TESTGATE_HISTORY_ROOT}/attempts.jsonl\"",
        "_verify_attempt_archive",
        "_restore_attempt_archive",
        "testgate-recovery-verified-",
        "testgate-recovery/v1",
        "recovery-attestation.jsonl",
        "reconcile-attempts",
        "combined_quality_proof_id",
        "--combined-proof-id",
    ] {
        assert!(
            workflow.contains(context),
            "missing blocking workflow contract {context}"
        );
    }
    for forbidden in [
        "pull_request:",
        "pull_request_target:",
        "schedule:",
        "ubuntu-latest",
        "conservative-rollback:",
        "inputs.boundary",
        "inputs.mode",
        "cargo install",
        "${RUNNER_TEMP}/testgate-",
        "continue-on-error: true",
        "openwepp-forest1-testgate-v",
    ] {
        assert!(
            !workflow.contains(forbidden),
            "forbidden workflow contract {forbidden}"
        );
    }
    assert_testgate_workflow_admission(&workflow);
}

fn assert_testgate_workflow_admission(workflow: &str) {
    assert!(workflow.contains("persist-credentials: false"));
    assert_eq!(
        workflow
            .matches("resolve_testgate_comparison_base.py")
            .count(),
        2
    );
    assert!(workflow.contains(
        "concurrency:\n  group: openwepp-forest1-testgate\n  queue: single\n  cancel-in-progress: false"
    ));
    assert_eq!(
        workflow
            .matches("git rev-parse --verify refs/remotes/origin/main")
            .count(),
        3,
        "execution admission, pre-gate execution, and independent verification must require current main"
    );
    assert!(
        !workflow
            .contains("git merge-base --is-ancestor \"${GITHUB_SHA}\" refs/remotes/origin/main")
    );
    assert!(workflow.contains("permissions:\n  contents: read"));
    assert_eq!(
        workflow.matches("include-hidden-files: true").count(),
        4,
        "every TESTGATE evidence publication must preserve indexed hidden files"
    );
    assert!(workflow.contains("execute-increment:\n    name: openwepp/execute-increment"));
    assert!(workflow.contains(
        "increment-gates:\n    name: openwepp/increment-gates\n    needs: [execute-increment, verify-increment]\n    if: ${{ always() }}"
    ));
}

fn assert_testgate_job_order() {
    let workflow = text(".github/workflows/testgate-shadow.yml");
    let execution_job = workflow
        .split_once("  execute-increment:")
        .expect("execution job")
        .1
        .split_once("  verify-increment:")
        .expect("verification job boundary")
        .0;
    let admission = execution_job
        .find("Admit trusted main comparison")
        .expect("execution admission");
    let bootstrap = execution_job
        .find("Bootstrap locked base and head dependencies")
        .expect("dependency bootstrap");
    let pre_gate = execution_job
        .find("Reject superseded head before gate execution")
        .expect("pre-gate supersession guard");
    let execute = execution_job
        .find("Execute content-verifiable increment gates")
        .expect("gate execution");
    assert!(admission < bootstrap);
    assert!(bootstrap < pre_gate);
    assert!(pre_gate < execute);

    let verifier_job = workflow
        .split_once("  verify-increment:")
        .expect("verification job")
        .1
        .split_once("  increment-gates:")
        .expect("aggregate job boundary")
        .0;
    let verifier_guard = verifier_job
        .find("ERROR: refusing to verify superseded head")
        .expect("verification current-main guard");
    let verifier_build = verifier_job
        .find("Build immutable-envelope verifier")
        .expect("verifier build");
    let verifier_upload = verifier_job
        .find("Upload independently verified evidence")
        .expect("verified evidence upload");
    assert!(verifier_guard < verifier_build);
    assert!(verifier_build < verifier_upload);

    let signer = workflow
        .split_once("  increment-gates:")
        .expect("signer job")
        .1;
    assert!(!signer.contains("actions/checkout"));
    assert!(!signer.contains("cargo build"));
    assert!(!signer.contains("python3"));
    let before_authority = signer
        .find("Reject superseded head before authority")
        .expect("pre-authority current-main guard");
    let attest = signer
        .find("Authenticate receipt to repository and workflow identity")
        .expect("attestation step");
    let verify_attestation = signer
        .find("Verify native attestation before authority succeeds")
        .expect("native verification step");
    let after_authority = signer
        .find("Reject superseded head after authority verification")
        .expect("post-authority current-main guard");
    let upload = signer
        .find("Upload authenticated gate evidence")
        .expect("authenticated evidence upload");
    assert!(before_authority < attest);
    assert!(attest < verify_attestation);
    assert!(verify_attestation < upload);
    assert!(upload < after_authority);
    let authenticated_upload = &signer[upload..after_authority];
    for required in [
        "path: |",
        "testgate-evidence/receipt.json",
        "testgate-evidence/terminal-plan.json",
        "testgate-evidence/attestation-predicate.json",
        "testgate-evidence/envelope-verification.json",
        "testgate-evidence/github-attestation.jsonl",
        "testgate-evidence/attestation-verification.json",
        "testgate-evidence/execution/target/gate-plan/adjudicated-crap.json",
    ] {
        assert!(
            authenticated_upload.contains(required),
            "authenticated publication missing bounded evidence: {required}"
        );
    }
    assert!(
        !authenticated_upload.contains("path: ${{ runner.temp }}/testgate-evidence\n"),
        "authenticated publication must not re-upload the full verified archive"
    );
    assert!(!signer[after_authority + 1..].contains("      - name:"));
}

fn assert_conservative_rollback_contract() {
    let conservative = text(".github/workflows/testgate-conservative.yml");
    assert!(conservative.contains("conservative-rollback:"));
    assert!(conservative.contains("name: openwepp/conservative-rollback"));
    assert!(conservative.contains("runs-on: ubuntu-24.04"));
    assert!(conservative.contains("run_release_candidate_gates.sh"));
    assert!(conservative.contains("--mode validate"));
    assert!(!conservative.contains("--skip-authority-required"));
    assert!(conservative.contains("--skip-stability"));
    assert!(conservative.contains("testgate_run_id:"));
    assert!(conservative.contains("gh attestation verify"));
    assert!(conservative.contains("GLOBAL_WORKSPACE"));
    assert!(conservative.contains("Run conservative comparison with reused global CRAP"));
    assert!(conservative.contains("--authority-only"));
    assert!(!conservative.contains("runs-on: [self-hosted"));
    assert!(conservative.contains("--deny-self-hosted-runners"));
    assert!(conservative.contains("smoke_only:"));
    assert!(conservative.contains("Prove hosted rollback smoke"));
    assert!(conservative.contains("RUNNER_ENVIRONMENT: ${{ runner.environment }}"));
    assert!(conservative.contains("qualification_claim:false"));
    assert!(conservative.contains("conservative-smoke-${{ github.run_id }}"));
    assert_eq!(
        conservative.matches("if: ${{ !inputs.smoke_only").count(),
        6,
        "all six broad or reuse steps must reject smoke mode"
    );
    assert!(conservative.contains(".raw_over_threshold_count == .adjudicated_count"));
    assert!(!conservative.contains(".raw_over_threshold_count == 0"));
    assert!(!conservative.contains(".adjudicated_count == 0"));

    let release = text(".github/workflows/release-gates.yml");
    assert!(!release.contains("runs-on: self-hosted"));
    assert!(release.contains("runs-on: [self-hosted, Linux, X64, openwepp, release, trusted]"));

    let rollback =
        text("docs/work-packages/20260718-testgate-ci-shadow-executor-001/artifacts/rollback.md");
    assert!(rollback.contains("entire nonrequired shadow workflow"));
    assert!(rollback.contains("required only after provider-side cutover"));
}

fn assert_workflow_and_rollback_contract() {
    assert_testgate_workflow_surface();
    assert_testgate_job_order();
    assert_conservative_rollback_contract();
}

fn assert_gate_definition_contract(entries: &[Value]) {
    let affected = entries
        .iter()
        .find(|entry| entry["gate_definition_id"] == "affected-adjudicated-crap-v1")
        .expect("affected CRAP definition");
    assert_eq!(
        affected["risk_classes"],
        serde_json::json!(["BOUNDED_COMPONENT", "INTEGRATED_DOMAIN"])
    );
    assert!(
        affected["arguments_template"]
            .as_array()
            .expect("affected arguments")
            .iter()
            .any(|argument| argument == "affected")
    );
    let global = entries
        .iter()
        .find(|entry| entry["gate_definition_id"] == "adjudicated-crap-v1")
        .expect("global CRAP definition");
    assert_eq!(global["risk_classes"], serde_json::json!(["CRITICAL"]));
    let combined = entries
        .iter()
        .find(|entry| entry["gate_definition_id"] == "combined-workspace-quality-v1")
        .expect("combined quality definition");
    assert_eq!(combined["inventory_source"], "NEXTEST_WORKSPACE");
    assert_eq!(combined["trust_requirement"], "PROTECTED_CI");
    assert_eq!(combined["output_paths"].as_array().map(Vec::len), Some(3));
    assert!(
        !combined["arguments_template"]
            .as_array()
            .expect("combined arguments")
            .iter()
            .any(|argument| argument == "--nextest-profile")
    );
    for (id, authority_class) in [
        ("authority-admission-v1", "A0"),
        ("required-authority-v1", "A3"),
    ] {
        let authority = entries
            .iter()
            .find(|entry| entry["gate_definition_id"] == id)
            .unwrap_or_else(|| panic!("missing authority definition {id}"));
        assert_eq!(authority["authority_class"], authority_class);
        assert_eq!(authority["risk_classes"], serde_json::json!(["CRITICAL"]));
    }
    let full = entries
        .iter()
        .find(|entry| entry["gate_definition_id"] == "workspace-full-nextest-v1")
        .expect("full workspace definition");
    assert_eq!(full["authority_class"], "NONE");
}

fn assert_crap_driver_contract() {
    let plan_schema = text("gate-policy/v1/schemas/gate-plan.schema.json");
    assert!(plan_schema.contains("quality_scope"));
    assert!(plan_schema.contains("covering_inventory_ids"));
    assert!(plan_schema.contains("ESCALATED_GLOBAL"));

    let affected_driver = text("tools/release/run_adjudicated_crap_gate.sh");
    assert!(affected_driver.contains("SCOPE=\"global\""));
    assert!(affected_driver.contains("affected scope requires fresh acquisition"));
    assert!(affected_driver.contains("--expected-package"));
    assert!(affected_driver.contains("--validate-expected-packages"));
    assert!(affected_driver.contains("affected-package-scope.json"));
    assert!(affected_driver.contains("--expected-package-scope"));
    assert!(affected_driver.contains("affected package scope changed before report publication"));
    assert!(affected_driver.contains("cargo llvm-cov show-env --sh"));
    assert!(affected_driver.contains("cargo nextest run"));
    assert_eq!(affected_driver.matches("cargo nextest run").count(), 1);
    assert!(affected_driver.contains("--config-file \"${NEXTEST_CONFIG}\""));
    assert!(affected_driver.contains("COVERAGE_PROFILE=\"${NEXTEST_PROFILE:-full}\""));
    assert!(affected_driver.contains("OUTPUT_DIR=\"target/adjudicated-crap\""));
    assert!(affected_driver.contains("OUTPUT_DIR_OVERRIDDEN=0"));
    assert!(affected_driver.contains("OUTPUT_DIR_OVERRIDDEN=1"));
    assert!(affected_driver.contains("elif [[ \"${OUTPUT_DIR_OVERRIDDEN}\" -eq 0 ]]"));
    assert!(affected_driver.contains("OUTPUT_DIR=\"${ROOT_DIR}/${OUTPUT_DIR}\""));
    assert!(affected_driver.contains("OPENWEPP_GATE_NEXTEST_CONFIG"));
    assert!(affected_driver.contains("COVERAGE_TMP=\"${TMPDIR}\""));
    assert!(affected_driver.contains("^/tmp/owg-[0-9]+-[0-9a-f]+$"));
    assert!(affected_driver.contains("mktemp -d /tmp/owg-crap-XXXXXX"));
    assert!(affected_driver.contains("TMPDIR=\"${COVERAGE_TMP}\""));
    assert!(affected_driver.contains("trap 'terminate 143' TERM"));
    assert!(affected_driver.contains("CARGO_BUILD_JOBS=4"));
    assert!(affected_driver.contains("CARGO_PROFILE_TEST_DEBUG=0"));
    assert!(affected_driver.contains("workspace-metadata.json"));
    let generated_files = affected_driver
        .split_once("GENERATED_FILES=(")
        .expect("generated artifact inventory")
        .1
        .split_once("\n)")
        .expect("generated artifact inventory terminator")
        .0;
    assert!(generated_files.contains("workspace-metadata.json"));
    assert!(affected_driver.contains("REPORT_SCOPE_ARGS+=(--package \"${package}\")"));
    assert!(affected_driver.contains("global LCOV report requires explicit workspace packages"));
    assert!(affected_driver.contains("OPENWEPP_GATE_ARTIFACT_ROOT"));
    assert!(affected_driver.contains("for package in \"${PACKAGES[@]}\""));
}

#[test]
fn blocking_executor_and_affected_quality_preserve_manual_rollback() {
    assert_receipt_runtime_guards();
    let definitions: Value = serde_json::from_str(&text("gate-policy/v1/gate-definitions.json"))
        .expect("gate definitions JSON");
    assert_eq!(definitions["enforcement_status"], "BLOCKING");
    let entries = definitions["definitions"].as_array().expect("definitions");
    assert_gate_definition_contract(entries);
    assert_crap_driver_contract();

    let profiles = text(".config/nextest.toml");
    for profile in ["affected", "checkpoint", "campaign", "release", "full"] {
        assert!(
            profiles.contains(&format!("[profile.{profile}]")),
            "missing lifecycle profile {profile}"
        );
    }

    assert_workflow_and_rollback_contract();
}

#[test]
fn coverage_scheduling_bounds_the_complete_assurance_publication_binary() {
    let profiles = text(".config/nextest.toml");
    assert!(profiles.contains("[test-groups.assurance-publication]\nmax-threads = 4"));
    let publication_override = profiles
        .split_once("filter = 'binary(assurance_v2_publication_contract)'")
        .expect("assurance publication override")
        .1
        .split_once("[[profile.default.overrides]]")
        .expect("next override must terminate assurance publication override")
        .0;
    assert!(publication_override.contains("test-group = \"assurance-publication\""));
    assert!(publication_override.contains("threads-required = 2"));
    assert!(!publication_override.contains("slow-timeout"));
    assert!(profiles.contains(
        "[profile.full]\ninherits = \"default\"\ndefault-filter = \"all()\"\n\
         fail-fast = false\nslow-timeout = { period = \"90s\", terminate-after = 8 }"
    ));
    assert!(!profiles.contains("all() - binary(assurance_v2_publication_contract)"));
}

#[test]
fn crap_runner_resolves_executor_and_standalone_output_branches() {
    let scratch = Scratch::new();
    let scratch_repo = scratch.path.join("repo");
    let script_directory = scratch_repo.join("tools/release");
    let working_directory = scratch.path.join("working");
    fs::create_dir_all(&script_directory).expect("create scratch script directory");
    fs::create_dir(&working_directory).expect("create scratch working directory");
    let script = script_directory.join("run_adjudicated_crap_gate.sh");
    fs::copy(
        root().join("tools/release/run_adjudicated_crap_gate.sh"),
        &script,
    )
    .expect("copy CRAP runner into isolated scratch repository");

    let standalone_default = run_crap_path_probe(&script, &working_directory, None, None);
    assert_failed_after_output_resolution(
        &standalone_default,
        &scratch_repo.join("target/adjudicated-crap"),
    );

    let standalone_relative = run_crap_path_probe(
        &script,
        &working_directory,
        Some(Path::new("explicit-relative")),
        None,
    );
    assert_failed_after_output_resolution(
        &standalone_relative,
        &working_directory.join("explicit-relative"),
    );

    let standalone_absolute_path = scratch.path.join("explicit-absolute");
    let standalone_absolute = run_crap_path_probe(
        &script,
        &working_directory,
        Some(&standalone_absolute_path),
        None,
    );
    assert_failed_after_output_resolution(&standalone_absolute, &standalone_absolute_path);

    let artifact_root = scratch.path.join("artifacts");
    fs::create_dir(&artifact_root).expect("create executor artifact root");
    let executor_default =
        run_crap_path_probe(&script, &working_directory, None, Some(&artifact_root));
    assert_failed_after_output_resolution(
        &executor_default,
        &artifact_root.join("target/adjudicated-crap"),
    );

    let executor_relative = run_crap_path_probe(
        &script,
        &working_directory,
        Some(Path::new("executor-relative")),
        Some(&artifact_root),
    );
    assert_failed_after_output_resolution(
        &executor_relative,
        &artifact_root.join("executor-relative"),
    );

    let rejected_absolute = run_crap_path_probe(
        &script,
        &working_directory,
        Some(&scratch.path.join("executor-absolute")),
        Some(&artifact_root),
    );
    assert!(!rejected_absolute.status.success());
    assert!(String::from_utf8_lossy(&rejected_absolute.stderr).contains(
        "executor artifact relocation requires an absolute root and safe relative output path"
    ));

    let rejected_traversal = run_crap_path_probe(
        &script,
        &working_directory,
        Some(Path::new("../executor-escape")),
        Some(&artifact_root),
    );
    assert!(!rejected_traversal.status.success());
    assert!(!scratch.path.join("executor-escape").exists());
}

#[test]
#[cfg(unix)]
fn crap_runner_rejects_unsafe_executor_nested_contracts() {
    use std::os::unix::fs::PermissionsExt;

    let scratch = Scratch::new();
    let artifact_root = scratch.path.join("artifacts");
    let outside_config = scratch.path.join("outside-nextest.toml");
    fs::create_dir(&artifact_root).expect("create executor artifact root");
    fs::write(&outside_config, "[store]\n").expect("write outside Nextest config");

    let missing_config = run_executor_crap_probe(&artifact_root, None, None);
    assert_eq!(missing_config.status.code(), Some(2));

    let outside = run_executor_crap_probe(&artifact_root, Some(&outside_config), None);
    assert_eq!(outside.status.code(), Some(2));

    let config = artifact_root.join("qualified-nextest.toml");
    fs::write(&config, "[store]\n").expect("write qualified Nextest config");
    let config_symlink = artifact_root.join("qualified-nextest-link.toml");
    std::os::unix::fs::symlink(&config, &config_symlink).expect("create Nextest config symlink");
    let symlinked = run_executor_crap_probe(&artifact_root, Some(&config_symlink), None);
    assert_eq!(symlinked.status.code(), Some(2));

    let hexadecimal = ShortProcessTemp::hexadecimal_directory();
    let hexadecimal_result = run_executor_crap_probe(
        &artifact_root,
        Some(&config_symlink),
        Some(&hexadecimal.path),
    );
    assert_eq!(hexadecimal_result.status.code(), Some(2));
    assert!(
        String::from_utf8_lossy(&hexadecimal_result.stderr)
            .contains("nested Nextest configuration is missing or unsafe"),
        "hexadecimal executor TMPDIR must reach config validation"
    );

    let missing_tmp = run_executor_crap_probe(&artifact_root, Some(&config), None);
    assert_eq!(missing_tmp.status.code(), Some(2));

    let wrong_mode = ShortProcessTemp::directory(0o755);
    let wrong_mode_result =
        run_executor_crap_probe(&artifact_root, Some(&config), Some(&wrong_mode.path));
    assert_eq!(wrong_mode_result.status.code(), Some(2));

    let long = ShortProcessTemp::long_directory();
    let long_result = run_executor_crap_probe(&artifact_root, Some(&config), Some(&long.path));
    assert_eq!(long_result.status.code(), Some(2));

    let symlink_target = scratch.path.join("tmp-target");
    fs::create_dir(&symlink_target).expect("create temporary symlink target");
    fs::set_permissions(&symlink_target, fs::Permissions::from_mode(0o700))
        .expect("set temporary symlink target mode");
    let symlink_tmp = ShortProcessTemp::symlink(&symlink_target);
    let symlink_result =
        run_executor_crap_probe(&artifact_root, Some(&config), Some(&symlink_tmp.path));
    assert_eq!(symlink_result.status.code(), Some(2));
}

#[test]
#[cfg(unix)]
fn crap_runner_removes_standalone_temporary_root_after_failure() {
    let scratch = Scratch::new();
    let fake_bin = scratch.path.join("fake-bin");
    let output = scratch.path.join("failure-output");
    let observed_tmp = scratch.path.join("observed-tmp");
    fs::create_dir(&fake_bin).expect("create fake binary directory");
    write_executable(
        &fake_bin.join("cargo"),
        "#!/bin/sh\nprintf '%s\\n' \"${TMPDIR}\" > \"${FAKE_CARGO_TMP}\"\nexit 9\n",
    );
    let inherited_path = std::env::var("PATH").expect("PATH");
    let result = Command::new("bash")
        .arg(root().join("tools/release/run_adjudicated_crap_gate.sh"))
        .arg("--output-dir")
        .arg(&output)
        .env("OPENWEPP_GATE_ARTIFACT_ROOT", "/tmp/outer-gate-artifact")
        .env(
            "OPENWEPP_GATE_NEXTEST_CONFIG",
            "/tmp/outer-gate-config.toml",
        )
        .env_remove("OPENWEPP_GATE_ARTIFACT_ROOT")
        .env_remove("OPENWEPP_GATE_NEXTEST_CONFIG")
        .env("PATH", format!("{}:{inherited_path}", fake_bin.display()))
        .env("FAKE_CARGO_TMP", &observed_tmp)
        .output()
        .expect("run standalone cleanup failure probe");
    assert_eq!(result.status.code(), Some(9));
    let temporary = PathBuf::from(
        fs::read_to_string(observed_tmp)
            .expect("observed TMPDIR")
            .trim(),
    );
    assert!(!temporary.exists());
    let run_status: serde_json::Value =
        serde_json::from_slice(&fs::read(output.join("run-status.json")).expect("run status"))
            .expect("parse run status");
    assert_eq!(run_status["result"], "FAIL");
    assert_eq!(run_status["exit_status"], 9);
}

#[test]
#[cfg(unix)]
fn crap_runner_records_signal_termination_as_failure() {
    use std::os::unix::process::CommandExt;
    use std::thread;
    use std::time::{Duration, Instant};

    let scratch = Scratch::new();
    let fake_bin = scratch.path.join("fake-bin");
    let output = scratch.path.join("signal-output");
    let started = scratch.path.join("cargo-started");
    fs::create_dir(&fake_bin).expect("create fake binary directory");
    let fake_cargo = fake_bin.join("cargo");
    write_executable(
        &fake_cargo,
        "#!/bin/sh\nprintf '%s\\n' \"${TMPDIR}\" > \"${FAKE_CARGO_TMP}\"\n: > \"${FAKE_CARGO_STARTED}\"\nsleep 60\n",
    );
    let observed_tmp = scratch.path.join("observed-tmp");
    let inherited_path = std::env::var("PATH").expect("PATH");
    let mut child = Command::new("bash");
    child
        .arg(root().join("tools/release/run_adjudicated_crap_gate.sh"))
        .arg("--output-dir")
        .arg(&output)
        .env("OPENWEPP_GATE_ARTIFACT_ROOT", "/tmp/outer-gate-artifact")
        .env(
            "OPENWEPP_GATE_NEXTEST_CONFIG",
            "/tmp/outer-gate-config.toml",
        )
        .env_remove("OPENWEPP_GATE_ARTIFACT_ROOT")
        .env_remove("OPENWEPP_GATE_NEXTEST_CONFIG")
        .env("PATH", format!("{}:{inherited_path}", fake_bin.display()))
        .env("FAKE_CARGO_STARTED", &started)
        .env("FAKE_CARGO_TMP", &observed_tmp)
        .process_group(0);
    let mut child = child.spawn().expect("spawn signal probe");
    let deadline = Instant::now() + Duration::from_secs(10);
    while !started.is_file() && Instant::now() < deadline {
        thread::sleep(Duration::from_millis(20));
    }
    if !started.is_file() {
        let _ = Command::new("kill")
            .args(["-KILL", "--"])
            .arg(format!("-{}", child.id()))
            .status();
        let _ = child.wait();
        panic!("fake cargo did not start");
    }
    let terminated = Command::new("kill")
        .arg("-TERM")
        .arg("--")
        .arg(format!("-{}", child.id()))
        .status()
        .expect("terminate CRAP process group");
    assert!(terminated.success());
    let status = child.wait().expect("wait for signal probe");
    assert_eq!(status.code(), Some(143));
    let run_status: serde_json::Value =
        serde_json::from_slice(&fs::read(output.join("run-status.json")).expect("read run status"))
            .expect("parse run status");
    assert_eq!(run_status["result"], "FAIL");
    assert_eq!(run_status["exit_status"], 143);
    let temporary = PathBuf::from(
        fs::read_to_string(observed_tmp)
            .expect("observed TMPDIR")
            .trim(),
    );
    assert!(!temporary.exists());
    let group_deadline = Instant::now() + Duration::from_secs(2);
    loop {
        let group_gone = Command::new("kill")
            .args(["-0", "--"])
            .arg(format!("-{}", child.id()))
            .status()
            .expect("probe terminated process group");
        if !group_gone.success() {
            break;
        }
        assert!(
            Instant::now() < group_deadline,
            "fake cargo descendant survived SIGTERM"
        );
        thread::sleep(Duration::from_millis(20));
    }
}

#[test]
fn runner_container_has_no_host_or_privileged_mounts() {
    let manager = text("tools/ci/omarchy-runner/manage.sh");
    let image = text("tools/ci/omarchy-runner/Dockerfile");
    let workflow = text(".github/workflows/testgate-shadow.yml");
    let host_receipt = text(
        "docs/work-packages/20260718-testgate-accelerated-cutover-001/artifacts/host-capacity-security.md",
    );
    let runner_recovery_evidence = text(
        "docs/work-packages/20260723-testgate-runner-gh-cli-recovery-001/artifacts/gate-evidence.md",
    );
    let image_id = "sha256:8a551a87d0784a74be1a76452beb1e4e6726cc36135722020e20a042e04bae84";
    let historical_image_id =
        "sha256:034ce655da139123cd775317d590d04dec6377788e4d124dc0e674f8d021e7e8";
    assert_eq!(manager.matches(image_id).count(), 1);
    assert_eq!(workflow.matches(image_id).count(), 2);
    assert_eq!(runner_recovery_evidence.matches(image_id).count(), 2);
    assert_eq!(host_receipt.matches(historical_image_id).count(), 1);
    assert!(manager.contains("--security-opt no-new-privileges=true"));
    assert!(manager.contains("--cap-drop ALL"));
    assert!(manager.contains("--read-only"));
    assert!(manager.contains("dst=/runner-state,readonly"));
    assert!(manager.contains("dst=/testgate-history"));
    assert!(!manager.contains("volume rm \"${HISTORY_VOLUME}\""));
    assert!(manager.contains("--tmpfs"));
    assert!(manager.contains("/t:rw,exec,nosuid,nodev"));
    assert!(manager.contains("/t:rw,exec,nosuid,nodev,size=56g"));
    assert!(manager.contains("/tmp:rw,exec,nosuid,nodev,size=24g"));
    assert!(manager.contains("--cpus 32 --cpuset-cpus 0-31 --memory 64g --memory-swap 64g"));
    assert!(manager.contains("forest1-openwepp-01"));
    assert!(manager.contains("omarchy-openwepp-01"));
    assert!(manager.contains("build-image"));
    assert!(manager.contains("install-image"));
    assert!(manager.contains("--resource cpuset-cpus=0-7"));
    assert!(manager.contains("--resource memory=24g"));
    assert!(manager.contains("--resource memory-swap=24g"));
    assert!(manager.contains("docker save --output"));
    assert!(manager.contains("transferred runner image archive digest mismatch"));
    assert!(manager.contains("provider_contract_matches"));
    assert!(manager.contains("runner must be uniquely online, idle, and exactly labeled"));
    assert!(manager.contains("/cache/cargo:rw,nosuid,nodev"));
    assert!(manager.contains("job-completed-hook.sh"));
    assert!(!manager.contains("/var/run/docker.sock"));
    assert!(!manager.contains("--privileged"));
    assert!(!manager.contains("--network host"));
    assert!(manager.contains("registration_token"));
    assert!(manager.contains("printf '%s\\n' \"${registration_token}\""));
    assert!(image.contains("RUSTUP_TOOLCHAIN=1.92.0-x86_64-unknown-linux-gnu"));
    assert!(image.contains("pyarrow==22.0.0"));
    assert!(image.contains("pandas==3.0.3"));
    assert!(image.contains("rustup component add llvm-tools-preview"));
    assert!(image.contains("UK2US_COMMIT=6ce03a96a9466bed029fb0287786cd903f1876d6"));
    assert!(image.contains("GH_VERSION=2.96.0"));
    assert!(image.contains(
        "GH_LINUX_AMD64_SHA256=83d5c2ccad5498f58bf6368acb1ab32588cf43ab3a4b1c301bf36328b1c8bd60"
    ));
    assert!(image.contains("gh_${GH_VERSION}_linux_amd64.tar.gz"));
    assert!(image.contains("sha256sum --check --strict"));
    assert!(
        image.contains("test \"$(gh --version | awk 'NR == 1 { print $3 }')\" = \"${GH_VERSION}\"")
    );
    assert!(image.contains("chown root:root /usr/local/bin/gh"));
    assert!(workflow.contains("test \"$(gh --version | awk 'NR == 1 { print $3 }')\" = '2.96.0'"));
    assert!(image.contains("python-is-python3 php-cli"));
    assert!(manager.contains("uk2us_rules.json"));
    let bootstrap = text("tools/ci/omarchy-runner/bootstrap_dependencies.sh");
    assert!(bootstrap.contains("python3 -m venv --system-site-packages .venv"));
    assert!(bootstrap.contains("pyarrow.__version__ == \"22.0.0\""));
    assert!(bootstrap.contains("pandas.__version__ == \"3.0.3\""));
    assert!(workflow.contains("rustup component list --installed"));
    assert!(image.contains(
        "ACTIONS_RUNNER_HOOK_JOB_COMPLETED=/usr/local/bin/openwepp-job-completed-hook.sh"
    ));
    let hook = text("tools/ci/omarchy-runner/job-completed-hook.sh");
    assert!(hook.contains("/runner-work /cache/cargo /t /home/runner /tmp"));
    assert!(hook.contains("/runner-state/_diag"));
    assert!(hook.contains("for round in {1..10}"));
}

#[test]
fn runner_github_cli_preflight_rejects_version_suffix_drift() {
    let probe = r#"test "$(printf '%s\n' "$1" | awk 'NR == 1 { print $3 }')" = '2.96.0'"#;
    let exact = Command::new("bash")
        .args(["-c", probe, "_", "gh version 2.96.0 (2026-07-02)"])
        .status()
        .expect("run exact GitHub CLI version probe");
    assert!(exact.success());

    let suffix_drift = Command::new("bash")
        .args(["-c", probe, "_", "gh version 2.96.0-malicious (2026-07-02)"])
        .status()
        .expect("run suffix-drift GitHub CLI version probe");
    assert!(!suffix_drift.success());
}

#[test]
fn trusted_workflow_binds_one_explicit_intent_package() {
    let workflow = text(".github/workflows/testgate-shadow.yml");
    let resolver = text("tools/local_ci/resolve_testgate_intent_package.py");
    let base_resolver = text("tools/local_ci/resolve_testgate_comparison_base.py");

    let document: serde_yaml::Value =
        serde_yaml::from_str(&workflow).expect("workflow YAML must parse");
    let events = document
        .get("on")
        .and_then(serde_yaml::Value::as_mapping)
        .expect("workflow event map");
    assert_eq!(events.len(), 1);
    let dispatch = events
        .get("workflow_dispatch")
        .and_then(serde_yaml::Value::as_mapping)
        .expect("workflow_dispatch must be the sole event");
    let inputs = dispatch
        .get("inputs")
        .and_then(serde_yaml::Value::as_mapping)
        .expect("workflow_dispatch inputs");
    for input_name in ["base_ref", "intent_package"] {
        let input = inputs
            .get(input_name)
            .and_then(serde_yaml::Value::as_mapping)
            .expect("required dispatch input");
        assert_eq!(
            input.get("required"),
            Some(&serde_yaml::Value::Bool(true)),
            "{input_name} must remain required"
        );
        assert!(
            !input.contains_key("default"),
            "{input_name} must not have a default"
        );
    }
    let jobs = document
        .get("jobs")
        .and_then(serde_yaml::Value::as_mapping)
        .expect("workflow jobs");
    for (job_name, step_name) in [
        ("execute-increment", "Admit trusted main comparison"),
        ("verify-increment", "Independently admit comparison base"),
    ] {
        let steps = jobs
            .get(job_name)
            .and_then(serde_yaml::Value::as_mapping)
            .and_then(|job| job.get("steps"))
            .and_then(serde_yaml::Value::as_sequence)
            .expect("admission job steps");
        let script = steps
            .iter()
            .filter_map(serde_yaml::Value::as_mapping)
            .find(|step| step.get("name").and_then(serde_yaml::Value::as_str) == Some(step_name))
            .and_then(|step| step.get("run"))
            .and_then(serde_yaml::Value::as_str)
            .expect("admission step script");
        assert_eq!(
            script
                .matches(r#"[[ ! "${INPUT_BASE}" =~ ^[0-9a-f]{40}$ ]]"#)
                .count(),
            1,
            "{job_name} must independently reject a non-exact base_ref"
        );
    }
    assert!(workflow.contains("resolve_testgate_intent_package.py"));
    assert_eq!(
        workflow
            .matches("resolve_testgate_comparison_base.py")
            .count(),
        2
    );
    assert!(workflow.contains("echo \"intent_package=${intent_package}\""));
    assert!(
        workflow.contains("--intent-package \"${{ steps.comparison.outputs.intent_package }}\"")
    );
    assert!(resolver.contains("TESTGATE-Intent-Package"));
    assert!(resolver.contains("push head must contain exactly one"));
    assert!(resolver.contains("workflow_dispatch requires an explicit"));
    assert!(resolver.contains("parts[2] not in {\"\", \".\", \"..\"}"));
    assert!(resolver.contains("\"\\r\" not in parts[2]"));
    assert!(resolver.contains("\"\\n\" not in parts[2]"));
    assert!(base_resolver.contains("TESTGATE-Comparison-Base"));
    assert!(base_resolver.contains("may only expand to an ancestor"));
    assert!(workflow.contains("predicate[\"intent_authorization\"][\"intent_package_path\"]"));
    assert!(!workflow.contains("predicate[\"intent_authorization\"][\"package_path\"]"));
    assert!(workflow.contains("Path(\"target/debug/openwepp-gate-plan\")"));
    assert!(workflow.contains("evidence / \"reconstructed-intent-authorization.json\""));
}
