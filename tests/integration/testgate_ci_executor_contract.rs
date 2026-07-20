use std::fs;
use std::path::{Path, PathBuf};
use std::process::{Command, Output};
use std::sync::atomic::{AtomicU64, Ordering};

use serde_json::Value;

fn root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
}

fn text(path: &str) -> String {
    fs::read_to_string(root().join(path)).expect("contract source must be readable")
}

static SCRATCH_COUNTER: AtomicU64 = AtomicU64::new(0);

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
    ] {
        assert!(
            executor.contains(required),
            "missing executor guard: {required}"
        );
    }
    assert!(!executor.contains("sh -c"));
    let cli = text("crates/openwepp-gate-planner/src/main.rs");
    assert!(cli.contains("verify_receipt(repo, &plan, &receipt, &artifacts)"));
    assert!(cli.contains("Some(\"FAIL\" | \"BLOCKED\" | \"INVALID\")"));

    let verifier = text("crates/openwepp-gate-planner/src/verifier.rs");
    for required in [
        "GATE-RECEIPT-EXECUTED-INVENTORY",
        "GATE-RECEIPT-UNAVAILABLE",
        "GATE-RECEIPT-PREREQUISITE",
        "verifier_accepts_truthful_fail_and_blocked_receipts",
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
    assert!(workflow.contains("persist-credentials: false"));
    assert!(workflow.contains("git merge-base --is-ancestor"));
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

#[test]
fn blocking_executor_and_affected_quality_preserve_manual_rollback() {
    assert_receipt_runtime_guards();
    let definitions: Value = serde_json::from_str(&text("gate-policy/v1/gate-definitions.json"))
        .expect("gate definitions JSON");
    assert_eq!(definitions["enforcement_status"], "BLOCKING");
    let entries = definitions["definitions"].as_array().expect("definitions");
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

    let plan_schema = text("gate-policy/v1/schemas/gate-plan.schema.json");
    assert!(plan_schema.contains("quality_scope"));
    assert!(plan_schema.contains("covering_inventory_ids"));
    assert!(plan_schema.contains("ESCALATED_GLOBAL"));

    let affected_driver = text("tools/release/run_adjudicated_crap_gate.sh");
    assert!(affected_driver.contains("SCOPE=\"global\""));
    assert!(affected_driver.contains("affected scope requires fresh acquisition"));
    assert!(affected_driver.contains("--expected-package"));
    assert!(affected_driver.contains("cargo llvm-cov show-env --sh"));
    assert!(affected_driver.contains("cargo nextest run"));
    assert!(affected_driver.contains("--config-file \"${NEXTEST_CONFIG}\""));
    assert!(affected_driver.contains("COVERAGE_PROFILE=\"${NEXTEST_PROFILE:-full}\""));
    assert!(affected_driver.contains("OUTPUT_DIR=\"target/adjudicated-crap\""));
    assert!(affected_driver.contains("OUTPUT_DIR_OVERRIDDEN=0"));
    assert!(affected_driver.contains("OUTPUT_DIR_OVERRIDDEN=1"));
    assert!(affected_driver.contains("elif [[ \"${OUTPUT_DIR_OVERRIDDEN}\" -eq 0 ]]"));
    assert!(affected_driver.contains("OUTPUT_DIR=\"${ROOT_DIR}/${OUTPUT_DIR}\""));
    assert!(affected_driver.contains("COVERAGE_TMP=\"${OPENWEPP_GATE_ARTIFACT_ROOT}/tmp\""));
    assert!(affected_driver.contains("COVERAGE_TMP=\"${OUTPUT_DIR}/tmp\""));
    assert!(affected_driver.contains("TMPDIR=\"${COVERAGE_TMP}\""));
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
fn runner_container_has_no_host_or_privileged_mounts() {
    let manager = text("tools/ci/omarchy-runner/manage.sh");
    let image = text("tools/ci/omarchy-runner/Dockerfile");
    let workflow = text(".github/workflows/testgate-shadow.yml");
    let host_receipt = text(
        "docs/work-packages/20260718-testgate-accelerated-cutover-001/artifacts/host-capacity-security.md",
    );
    let image_id = "sha256:034ce655da139123cd775317d590d04dec6377788e4d124dc0e674f8d021e7e8";
    assert_eq!(manager.matches(image_id).count(), 1);
    assert_eq!(workflow.matches(image_id).count(), 2);
    assert_eq!(host_receipt.matches(image_id).count(), 1);
    assert!(manager.contains("--security-opt no-new-privileges=true"));
    assert!(manager.contains("--cap-drop ALL"));
    assert!(manager.contains("--read-only"));
    assert!(manager.contains("dst=/runner-state,readonly"));
    assert!(manager.contains("--tmpfs"));
    assert!(manager.contains("/t:rw,exec,nosuid,nodev"));
    assert!(manager.contains("/t:rw,exec,nosuid,nodev,size=40g"));
    assert!(manager.contains("--cpus 32 --cpuset-cpus 0-31 --memory 48g --memory-swap 48g"));
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
