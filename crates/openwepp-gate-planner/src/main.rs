use std::collections::BTreeMap;
use std::fs;
use std::io::Write;
use std::path::{Path, PathBuf};
use std::time::Instant;

use openwepp_gate_planner::canonical::{canonical_bytes, parse_strict};
use openwepp_gate_planner::error::{ErrorClass, GatePolicyError, Result};
use openwepp_gate_planner::executor::{ExecutionClaims, execute_plan, execute_plan_stage};
use openwepp_gate_planner::ledger::{verify_assurance_impact, verify_campaign_ledger};
use openwepp_gate_planner::package_validation::{validate_package, validate_package_chain};
use openwepp_gate_planner::planner::reconcile_intent_terminal;
use openwepp_gate_planner::planner::{NextestInventory, PlanRequest, Planner, PlanningStage};
use openwepp_gate_planner::pre_heavy::{
    ConstructedAudit, admit_attempt_ledger, append_attempt_record, build_audit,
    build_failure_audit, close_tooling_defect, construct_audit, reconcile_orphaned_attempts,
    record_heavy_failure, validate_resume_ledger,
};
use openwepp_gate_planner::repository::{ObservedSource, observe_committed, observe_dirty};
use openwepp_gate_planner::resume::{ResumeCandidate, load_candidate_after_ready_audit};
use openwepp_gate_planner::verifier::{
    DirectoryArtifacts, verify_receipt, verify_receipt_after_ready_audit, verify_receipt_envelope,
};
use serde_json::{Value, json};

type CommandHandler = fn(&Path, &BTreeMap<String, String>) -> Result<Value>;
type CommandDefinition = (&'static str, CommandHandler, &'static [&'static str]);

const RECEIPT_OPTIONS: &[&str] = &["repo", "plan", "receipt", "artifact-root"];
const COMMANDS: [CommandDefinition; 12] = [
    (
        "plan",
        plan_command,
        &[
            "repo",
            "stage",
            "base",
            "head",
            "boundary",
            "campaign",
            "combined-proof-id",
            "output",
            "predecessor",
            "authorized-paths",
            "package-authority-chain",
        ],
    ),
    (
        "run",
        run_command,
        &[
            "repo",
            "plan",
            "artifact-root",
            "output",
            "principal",
            "repository",
            "source-event",
            "source-ref",
            "workflow",
            "job",
            "runner",
            "attempt",
            "stage",
            "audit",
            "resume",
            "light-output",
            "audit-output",
        ],
    ),
    ("verify-receipt", receipt_command, RECEIPT_OPTIONS),
    (
        "verify-receipt-envelope",
        receipt_envelope_command,
        RECEIPT_OPTIONS,
    ),
    (
        "verify-ledger",
        ledger_command,
        &["repo", "ledger", "predecessor"],
    ),
    ("verify-assurance", assurance_command, &["repo", "record"]),
    (
        "reconcile",
        reconcile_command,
        &["repo", "intent", "terminal"],
    ),
    (
        "pre-heavy-audit",
        pre_heavy_audit_command,
        &[
            "repo",
            "plan",
            "light-receipts",
            "artifact-root",
            "ledger",
            "output",
        ],
    ),
    (
        "validate-package",
        validate_package_command,
        &["repo", "base", "package", "output"],
    ),
    (
        "validate-package-chain",
        validate_package_chain_command,
        &["repo", "base", "head", "package", "output"],
    ),
    (
        "reconcile-attempts",
        reconcile_attempts_command,
        &["repo", "ledger"],
    ),
    (
        "close-tooling-defect",
        close_tooling_defect_command,
        &[
            "repo",
            "ledger",
            "defect-id",
            "correction-commit",
            "closure-evidence",
            "invalidated-recovery-root",
        ],
    ),
];

fn main() {
    let status = emit(run());
    if status != 0 {
        std::process::exit(status);
    }
}

fn emit(result: Result<Value>) -> i32 {
    match result {
        Ok(value) => emit_value(&value),
        Err(error) => {
            eprintln!("{error}");
            2
        }
    }
}

fn emit_value(value: &Value) -> i32 {
    let Ok(text) = serde_json::to_string(value) else {
        eprintln!("GATE-CLI-SERIALIZE: result is not serializable");
        return 2;
    };
    println!("{text}");
    i32::from(matches!(
        value["result"].as_str(),
        Some("FAIL" | "BLOCKED" | "INVALID")
    ))
}

fn run() -> Result<Value> {
    run_arguments(std::env::args().skip(1))
}

fn run_arguments(arguments: impl IntoIterator<Item = String>) -> Result<Value> {
    let mut arguments = arguments.into_iter();
    let command = arguments.next().ok_or_else(usage_error)?;
    let remaining = arguments.collect::<Vec<_>>();
    let options = parse_options(&remaining)?;
    let repo = PathBuf::from(options.get("repo").map_or(".", String::as_str));
    dispatch(&command, &repo, &options)
}

fn dispatch(command: &str, repo: &Path, options: &BTreeMap<String, String>) -> Result<Value> {
    let (handler, allowed) = COMMANDS
        .iter()
        .find(|(name, _, _)| *name == command)
        .map(|(_, handler, allowed)| (*handler, *allowed))
        .ok_or_else(usage_error)?;
    reject_unknown_options(options, allowed)?;
    handler(repo, options)
}

fn reject_unknown_options(options: &BTreeMap<String, String>, allowed: &[&str]) -> Result<()> {
    if options.keys().all(|key| allowed.contains(&key.as_str())) {
        Ok(())
    } else {
        Err(usage_error())
    }
}

fn run_command(repo: &Path, options: &BTreeMap<String, String>) -> Result<Value> {
    let (plan, artifact_root, claims) = execution_inputs(options)?;
    if let Some(stage) = options.get("stage") {
        return staged_run_command(repo, options, &plan, &artifact_root, &claims, stage);
    }
    let (receipt, verdict) = verified_execution(repo, &plan, &artifact_root, &claims)?;
    let output = persist_plan(repo, options, &receipt)?;
    Ok(json!({
        "result": verdict["result"],
        "receipt_id": verdict["receipt_id"],
        "trust_class": verdict["trust_class"],
        "output": output
    }))
}

fn staged_run_command(
    repo: &Path,
    options: &BTreeMap<String, String>,
    plan: &Value,
    artifact_root: &Path,
    claims: &ExecutionClaims,
    stage: &str,
) -> Result<Value> {
    if stage == "transition" {
        return trusted_transition_command(repo, options, plan, artifact_root, claims);
    }
    if stage == "light" {
        let receipt = execute_plan_stage(repo, plan, artifact_root, claims, "LIGHT", None, None)?;
        let output = persist_plan(repo, options, &receipt)?;
        return Ok(json!({
            "result": receipt["result"],
            "receipt_id": receipt["stage_receipt_id"],
            "output": output,
            "stage": "LIGHT"
        }));
    }
    if stage != "heavy" {
        return Err(usage_error());
    }
    Err(GatePolicyError::new(
        ErrorClass::Trust,
        "GATE-EXEC-AUDIT-UNAUTHENTICATED",
        "standalone HEAVY cannot authenticate a self-hashed READY audit; use the in-process transition",
    ))
}

fn trusted_transition_command(
    repo: &Path,
    options: &BTreeMap<String, String>,
    plan: &Value,
    artifact_root: &Path,
    claims: &ExecutionClaims,
) -> Result<Value> {
    let ledger = prepare_transition(repo, options)?;
    let light = execute_light_transition(repo, options, plan, artifact_root, claims, &ledger)?;
    let audit = construct_transition_audit(repo, options, plan, &light, artifact_root, &ledger)?;
    finish_transition(repo, options, plan, artifact_root, claims, &ledger, &audit)
}

fn prepare_transition(repo: &Path, options: &BTreeMap<String, String>) -> Result<PathBuf> {
    validate_transition_outputs(repo, options)?;
    let ledger = PathBuf::from(required(options, "resume")?);
    admit_attempt_ledger(&ledger)?;
    reconcile_orphaned_attempts(&ledger)?;
    Ok(ledger)
}

fn execute_light_transition(
    repo: &Path,
    options: &BTreeMap<String, String>,
    plan: &Value,
    artifact_root: &Path,
    claims: &ExecutionClaims,
    ledger: &Path,
) -> Result<Value> {
    let light_started = Instant::now();
    let light = execute_plan_stage(repo, plan, artifact_root, claims, "LIGHT", None, None)?;
    persist_named(repo, options, "light-output", &light)?;
    append_attempt_record(
        ledger,
        json!({
            "record_type": "STAGE_ATTEMPT",
            "status": "CLOSED",
            "stage": "LIGHT",
            "plan_id": plan["plan_id"],
            "receipt_id": light["stage_receipt_id"],
            "result": light["result"],
            "artifact_root": artifact_root.display().to_string(),
            "wall_time_ms": elapsed_millis(&light_started),
        }),
    )?;
    Ok(light)
}

fn construct_transition_audit(
    repo: &Path,
    options: &BTreeMap<String, String>,
    plan: &Value,
    light: &Value,
    artifact_root: &Path,
    ledger: &Path,
) -> Result<ConstructedAudit> {
    let audit = construct_audit(repo, plan, light, artifact_root, ledger)?;
    persist_named(repo, options, "audit-output", audit.as_value())?;
    Ok(audit)
}

fn finish_transition(
    repo: &Path,
    options: &BTreeMap<String, String>,
    plan: &Value,
    artifact_root: &Path,
    claims: &ExecutionClaims,
    ledger: &Path,
    audit: &ConstructedAudit,
) -> Result<Value> {
    if audit.as_value()["status"] != "READY" {
        return Ok(json!({
            "result": audit.as_value()["status"],
            "audit_id": audit.as_value()["audit_id"],
            "reason_codes": audit.as_value()["reason_codes"],
            "stage": "AUDIT"
        }));
    }
    trusted_heavy_run(repo, options, plan, artifact_root, claims, ledger, audit)
}

#[cfg(target_os = "linux")]
fn validate_transition_outputs(repo: &Path, options: &BTreeMap<String, String>) -> Result<()> {
    use std::collections::BTreeSet;
    use std::os::fd::AsRawFd;

    let mut identities = BTreeSet::new();
    for name in ["resume", "plan", "light-output", "audit-output", "output"] {
        let path = PathBuf::from(required(options, name)?);
        if matches!(name, "resume" | "plan") {
            let metadata = fs::symlink_metadata(&path).map_err(|error| {
                GatePolicyError::new(ErrorClass::Io, "GATE-CLI-INPUT", error.to_string())
            })?;
            if metadata.file_type().is_symlink() || !metadata.is_file() {
                return Err(GatePolicyError::new(
                    ErrorClass::Cli,
                    "GATE-CLI-INPUT-SYMLINK",
                    format!("{name} must be an existing regular file, not a symlink"),
                ));
            }
        }
        let (parent, filename) = confined_output_parent(repo, &path)?;
        let stable_parent = fs::canonicalize(format!("/proc/self/fd/{}", parent.as_raw_fd()))
            .map_err(|error| {
                GatePolicyError::new(ErrorClass::Io, "GATE-CLI-OUTPUT-DIR", error.to_string())
            })?;
        if !identities.insert(stable_parent.join(filename)) {
            return Err(GatePolicyError::new(
                ErrorClass::Cli,
                "GATE-CLI-OUTPUT-COLLISION",
                "transition inputs and outputs must resolve to distinct paths",
            ));
        }
    }
    Ok(())
}

#[cfg(not(target_os = "linux"))]
fn validate_transition_outputs(_repo: &Path, _options: &BTreeMap<String, String>) -> Result<()> {
    Err(GatePolicyError::new(
        ErrorClass::Io,
        "GATE-CLI-OUTPUT-UNSUPPORTED",
        "descriptor-confined output is unavailable on this platform",
    ))
}

fn trusted_heavy_run(
    repo: &Path,
    options: &BTreeMap<String, String>,
    plan: &Value,
    artifact_root: &Path,
    claims: &ExecutionClaims,
    ledger: &Path,
    audit: &ConstructedAudit,
) -> Result<Value> {
    let inputs = HeavyAttemptInputs {
        repo,
        options,
        plan,
        artifact_root,
        claims,
        ledger,
        audit,
    };
    let context = begin_heavy_attempt(&inputs)?;
    let outcome = execute_heavy_attempt(&inputs, &context.started_entry_sha256);
    finish_heavy_attempt(outcome, &inputs, &context)
}

struct HeavyAttemptInputs<'a> {
    repo: &'a Path,
    options: &'a BTreeMap<String, String>,
    plan: &'a Value,
    artifact_root: &'a Path,
    claims: &'a ExecutionClaims,
    ledger: &'a Path,
    audit: &'a ConstructedAudit,
}

struct HeavyAttemptContext {
    started: Instant,
    recovery_root: Value,
    submitted_audit_id: Value,
    started_entry_sha256: String,
}

fn begin_heavy_attempt(inputs: &HeavyAttemptInputs<'_>) -> Result<HeavyAttemptContext> {
    let started = Instant::now();
    let recovery_root = std::env::var_os("OPENWEPP_GATE_CHECKPOINT_MIRROR_ROOT")
        .map(PathBuf::from)
        .map_or(Value::Null, |path| json!(path.display().to_string()));
    let submitted_audit_id = inputs.audit.as_value()["audit_id"].clone();
    let started_entry_sha256 = append_attempt_record(
        inputs.ledger,
        json!({
            "record_type": "STAGE_ATTEMPT",
            "status": "STARTED",
            "stage": "HEAVY",
            "plan_id": inputs.plan["plan_id"],
            "audit_id": submitted_audit_id,
            "phase": "ADMISSION",
            "artifact_root": inputs.artifact_root.display().to_string(),
            "recovery_root": recovery_root,
            "workflow": inputs.claims.workflow,
            "job": inputs.claims.job,
            "runner": inputs.claims.runner,
            "attempt": inputs.claims.attempt,
        }),
    )?;
    Ok(HeavyAttemptContext {
        started,
        recovery_root,
        submitted_audit_id,
        started_entry_sha256,
    })
}

fn execute_heavy_attempt(
    inputs: &HeavyAttemptInputs<'_>,
    started_entry_sha256: &str,
) -> Result<Value> {
    let resume_candidate = admit_heavy_resume(
        inputs.repo,
        inputs.plan,
        inputs.artifact_root,
        inputs.claims,
        inputs.ledger,
        inputs.audit,
        started_entry_sha256,
    )?;
    execute_and_verify_heavy(
        inputs.repo,
        inputs.options,
        inputs.plan,
        inputs.artifact_root,
        inputs.claims,
        inputs.audit,
        resume_candidate.as_ref(),
    )
}

fn admit_heavy_resume(
    repo: &Path,
    plan: &Value,
    artifact_root: &Path,
    claims: &ExecutionClaims,
    ledger: &Path,
    audit: &ConstructedAudit,
    started_entry_sha256: &str,
) -> Result<Option<ResumeCandidate>> {
    validate_resume_ledger(
        repo,
        plan,
        audit.as_value(),
        artifact_root,
        ledger,
        started_entry_sha256,
        claims,
    )?;
    load_candidate_after_ready_audit(repo, plan, ledger, claims, audit, started_entry_sha256)
}

fn execute_and_verify_heavy(
    repo: &Path,
    options: &BTreeMap<String, String>,
    plan: &Value,
    artifact_root: &Path,
    claims: &ExecutionClaims,
    audit: &ConstructedAudit,
    resume_candidate: Option<&ResumeCandidate>,
) -> Result<Value> {
    let receipt = execute_plan_stage(
        repo,
        plan,
        artifact_root,
        claims,
        "HEAVY",
        Some(audit),
        resume_candidate,
    )?;
    verify_receipt_after_ready_audit(
        repo,
        plan,
        &receipt,
        &DirectoryArtifacts::new(artifact_root.to_owned()),
    )?;
    let output = persist_plan(repo, options, &receipt)?;
    Ok(json!({
        "result": receipt["result"],
        "receipt_id": receipt["receipt_id"],
        "output": output,
        "stage": "HEAVY"
    }))
}

fn finish_heavy_attempt(
    outcome: Result<Value>,
    inputs: &HeavyAttemptInputs<'_>,
    context: &HeavyAttemptContext,
) -> Result<Value> {
    match outcome {
        Ok(value) => close_heavy_attempt(value, inputs, context),
        Err(error) => fail_heavy_attempt(error, inputs, context),
    }
}

fn close_heavy_attempt(
    value: Value,
    inputs: &HeavyAttemptInputs<'_>,
    context: &HeavyAttemptContext,
) -> Result<Value> {
    append_attempt_record(
        inputs.ledger,
        json!({
            "record_type": "STAGE_ATTEMPT",
            "status": "CLOSED",
            "stage": "HEAVY",
            "plan_id": inputs.plan["plan_id"],
            "audit_id": context.submitted_audit_id,
            "artifact_root": inputs.artifact_root.display().to_string(),
            "recovery_root": context.recovery_root,
            "workflow": inputs.claims.workflow,
            "job": inputs.claims.job,
            "runner": inputs.claims.runner,
            "attempt": inputs.claims.attempt,
            "receipt_id": value["receipt_id"],
            "result": value["result"],
            "wall_time_ms": elapsed_millis(&context.started),
            "started_entry_sha256": context.started_entry_sha256,
        }),
    )?;
    Ok(value)
}

fn fail_heavy_attempt(
    error: GatePolicyError,
    inputs: &HeavyAttemptInputs<'_>,
    context: &HeavyAttemptContext,
) -> Result<Value> {
    let cause_key = error.code;
    record_heavy_failure(
        inputs.ledger,
        json!({
            "record_type": "STAGE_ATTEMPT",
            "status": "FAILED",
            "stage": "HEAVY",
            "plan_id": inputs.plan["plan_id"],
            "audit_id": context.submitted_audit_id,
            "artifact_root": inputs.artifact_root.display().to_string(),
            "recovery_root": context.recovery_root,
            "workflow": inputs.claims.workflow,
            "job": inputs.claims.job,
            "runner": inputs.claims.runner,
            "attempt": inputs.claims.attempt,
            "result": null,
            "error_code": error.code,
            "error_message": error.message,
            "cause_key": cause_key,
            "failure_class": if cause_key.contains("SPAWN") || cause_key.contains("TIMEOUT") || cause_key.contains("RUNNER") {"INFRASTRUCTURE"} else {"TOOLING"},
            "wall_time_ms": elapsed_millis(&context.started),
            "started_entry_sha256": context.started_entry_sha256,
        }),
        cause_key,
    )?;
    Err(error)
}

#[allow(clippy::cast_possible_truncation)]
fn elapsed_millis(started: &Instant) -> u64 {
    // A live process cannot span the roughly 584 million years needed to exceed u64 milliseconds.
    started.elapsed().as_millis() as u64
}

fn reconcile_attempts_command(_repo: &Path, options: &BTreeMap<String, String>) -> Result<Value> {
    let reconciled = reconcile_orphaned_attempts(Path::new(required(options, "ledger")?))?;
    Ok(json!({"result": "PASS", "reconciled_attempts": reconciled}))
}

fn close_tooling_defect_command(_repo: &Path, options: &BTreeMap<String, String>) -> Result<Value> {
    let ledger = Path::new(required(options, "ledger")?);
    let defect_id = required(options, "defect-id")?;
    let entry_sha256 = close_tooling_defect(
        ledger,
        defect_id,
        required(options, "correction-commit")?,
        required(options, "closure-evidence")?,
        options
            .get("invalidated-recovery-root")
            .map(PathBuf::from)
            .as_deref(),
    )?;
    Ok(json!({
        "result": "CLOSED",
        "defect_id": defect_id,
        "entry_sha256": entry_sha256,
    }))
}

fn pre_heavy_audit_command(repo: &Path, options: &BTreeMap<String, String>) -> Result<Value> {
    let inputs = pre_heavy_audit_inputs(options)?;
    let ledger_preparation = prepare_audit_ledger(&inputs.ledger);
    let light_result = read_json(Path::new(required(options, "light-receipts")?));
    let audit = select_pre_heavy_audit(repo, &inputs, light_result, ledger_preparation)?;
    let output = persist_plan(repo, options, &audit)?;
    Ok(json!({
        "result": audit["status"],
        "audit_id": audit["audit_id"],
        "reason_codes": audit["reason_codes"],
        "output": output
    }))
}

struct PreHeavyAuditInputs {
    plan: Value,
    artifact_root: PathBuf,
    ledger: PathBuf,
}

fn pre_heavy_audit_inputs(options: &BTreeMap<String, String>) -> Result<PreHeavyAuditInputs> {
    Ok(PreHeavyAuditInputs {
        plan: read_json(Path::new(required(options, "plan")?))?,
        artifact_root: PathBuf::from(required(options, "artifact-root")?),
        ledger: PathBuf::from(required(options, "ledger")?),
    })
}

fn prepare_audit_ledger(ledger: &Path) -> Result<()> {
    admit_attempt_ledger(ledger).and_then(|()| reconcile_orphaned_attempts(ledger).map(|_| ()))
}

fn select_pre_heavy_audit(
    repo: &Path,
    inputs: &PreHeavyAuditInputs,
    light_result: Result<Value>,
    ledger_preparation: Result<()>,
) -> Result<Value> {
    match light_result {
        Ok(light) => audit_for_readable_light(repo, inputs, &light, ledger_preparation),
        Err(failure) => audit_for_invalid_light(repo, inputs, &failure),
    }
}

fn audit_for_readable_light(
    repo: &Path,
    inputs: &PreHeavyAuditInputs,
    light: &Value,
    ledger_preparation: Result<()>,
) -> Result<Value> {
    match ledger_preparation {
        Ok(()) => build_or_failure_audit(repo, inputs, light),
        Err(failure) => build_failure_audit(
            repo,
            &inputs.plan,
            light,
            &inputs.artifact_root,
            &inputs.ledger,
            &failure,
        ),
    }
}

fn build_or_failure_audit(
    repo: &Path,
    inputs: &PreHeavyAuditInputs,
    light: &Value,
) -> Result<Value> {
    match build_audit(
        repo,
        &inputs.plan,
        light,
        &inputs.artifact_root,
        &inputs.ledger,
    ) {
        Ok(audit) => Ok(audit),
        Err(failure) => build_failure_audit(
            repo,
            &inputs.plan,
            light,
            &inputs.artifact_root,
            &inputs.ledger,
            &failure,
        ),
    }
}

fn audit_for_invalid_light(
    repo: &Path,
    inputs: &PreHeavyAuditInputs,
    failure: &GatePolicyError,
) -> Result<Value> {
    let represented = GatePolicyError::new(
        failure.class,
        "GATE-AUDIT-LIGHT-INPUT-INVALID",
        format!("{}: {}", failure.code, failure.message),
    );
    build_failure_audit(
        repo,
        &inputs.plan,
        &json!({}),
        &inputs.artifact_root,
        &inputs.ledger,
        &represented,
    )
}

fn validate_package_command(repo: &Path, options: &BTreeMap<String, String>) -> Result<Value> {
    let base = required(options, "base")?;
    let package = PathBuf::from(required(options, "package")?);
    let audit = validate_package(repo, base, &package)?;
    let output = persist_plan(repo, options, &audit)?;
    Ok(json!({
        "result": audit["status"],
        "package_audit_id": audit["package_audit_id"],
        "reason_codes": audit["reason_codes"],
        "output": output
    }))
}

fn validate_package_chain_command(
    repo: &Path,
    options: &BTreeMap<String, String>,
) -> Result<Value> {
    let (base, head, package) = package_chain_command_inputs(options)?;
    let chain = validate_package_chain(repo, base, Some(head), &package)?;
    let output = persist_plan(repo, options, &chain)?;
    Ok(json!({
        "result": chain["status"],
        "package_authority_chain_id": chain["package_authority_chain_id"],
        "reason_codes": chain["reason_codes"],
        "output": output
    }))
}

fn package_chain_command_inputs(
    options: &BTreeMap<String, String>,
) -> Result<(&str, &str, PathBuf)> {
    let base = required(options, "base")?;
    let head = required(options, "head")?;
    let package = PathBuf::from(required(options, "package")?);
    Ok((base, head, package))
}

fn execution_inputs(
    options: &BTreeMap<String, String>,
) -> Result<(Value, PathBuf, ExecutionClaims)> {
    let plan = read_json(Path::new(required(options, "plan")?))?;
    let artifact_root = PathBuf::from(required(options, "artifact-root")?);
    let claims = execution_claims(options)?;
    Ok((plan, artifact_root, claims))
}

fn verified_execution(
    repo: &Path,
    plan: &Value,
    artifact_root: &Path,
    claims: &ExecutionClaims,
) -> Result<(Value, Value)> {
    let receipt = execute_plan(repo, plan, artifact_root, claims)?;
    let artifacts = DirectoryArtifacts::new(artifact_root.to_owned());
    let verdict = verify_receipt(repo, plan, &receipt, &artifacts)?;
    let summary = json!({
        "result": verdict.result(),
        "receipt_id": verdict.receipt_id(),
        "trust_class": verdict.trust_class()
    });
    Ok((receipt, summary))
}

fn execution_claims(options: &BTreeMap<String, String>) -> Result<ExecutionClaims> {
    let defaults = ExecutionClaims::default();
    let attempt = options
        .get("attempt")
        .map(|value| value.parse::<u64>())
        .transpose()
        .map_err(|_| usage_error())?
        .unwrap_or(defaults.attempt);
    if attempt == 0 {
        return Err(usage_error());
    }
    Ok(ExecutionClaims {
        principal: option_or(options, "principal", defaults.principal),
        repository: option_or(options, "repository", defaults.repository),
        source_event: option_or(options, "source-event", defaults.source_event),
        source_ref: option_or(options, "source-ref", defaults.source_ref),
        workflow: option_or(options, "workflow", defaults.workflow),
        job: option_or(options, "job", defaults.job),
        runner: option_or(options, "runner", defaults.runner),
        attempt,
    })
}

fn option_or(options: &BTreeMap<String, String>, key: &str, default: String) -> String {
    options.get(key).cloned().unwrap_or(default)
}

fn reconcile_command(repo: &Path, options: &BTreeMap<String, String>) -> Result<Value> {
    let (intent, terminal) = reconciliation_inputs(options)?;
    let reconciliation = reconcile_intent_terminal(repo, &intent, &terminal)?;
    Ok(json!({
        "result": "PASS",
        "intent_plan_id": intent["plan_id"],
        "terminal_plan_id": terminal["plan_id"],
        "added_paths": reconciliation.added_paths,
        "removed_paths": reconciliation.removed_paths,
        "risk_escalated": reconciliation.risk_escalated
    }))
}

fn reconciliation_inputs(options: &BTreeMap<String, String>) -> Result<(Value, Value)> {
    let intent_path = required(options, "intent")?;
    let terminal_path = required(options, "terminal")?;
    Ok((
        read_json(Path::new(intent_path))?,
        read_json(Path::new(terminal_path))?,
    ))
}

fn plan_command(repo: &Path, options: &BTreeMap<String, String>) -> Result<Value> {
    let request = plan_request(repo, options)?;
    let plan = Planner::new(NextestInventory).build(repo, &request)?;
    let output = persist_plan(repo, options, &plan)?;
    Ok(json!({"result": "PASS", "plan_id": plan["plan_id"], "output": output}))
}

fn plan_request(repo: &Path, options: &BTreeMap<String, String>) -> Result<PlanRequest> {
    let (authorized_paths, source, package_authority) = planning_context(repo, options)?;
    let stage = planning_stage(options)?;
    let (package_authority_chain_id, intent_package_path) =
        package_authority_fields(&package_authority)?;
    Ok(PlanRequest {
        stage,
        predecessor_intent_plan_id: options.get("predecessor").cloned(),
        boundary: boundary(options),
        campaign_id: options.get("campaign").cloned(),
        combined_quality_proof_id: options.get("combined-proof-id").cloned(),
        authorized_paths,
        package_authority_chain_id,
        intent_package_path,
        source,
    })
}

fn planning_context(
    repo: &Path,
    options: &BTreeMap<String, String>,
) -> Result<(Vec<String>, ObservedSource, Value)> {
    let authorized_paths = authorized_paths(options)?;
    let source = planning_source(repo, options)?;
    let package_authority = package_authority(repo, options, &authorized_paths, &source)?;
    Ok((authorized_paths, source, package_authority))
}

fn package_authority_fields(authority: &Value) -> Result<(String, String)> {
    let package_authority_chain_id = authority["package_authority_chain_id"]
        .as_str()
        .ok_or_else(usage_error)?
        .to_owned();
    let intent_package_path = authority["intent_package_path"]
        .as_str()
        .ok_or_else(usage_error)?
        .to_owned();
    Ok((package_authority_chain_id, intent_package_path))
}

fn package_authority(
    repo: &Path,
    options: &BTreeMap<String, String>,
    authorized: &[String],
    source: &ObservedSource,
) -> Result<Value> {
    let value = read_package_authority(options)?;
    let reconstructed = reconstruct_package_authority(repo, &value, source)?;
    require_exact_package_authority(&value, &reconstructed, authorized)?;
    Ok(value)
}

fn read_package_authority(options: &BTreeMap<String, String>) -> Result<Value> {
    read_json(Path::new(required(options, "package-authority-chain")?))
}

fn reconstruct_package_authority(
    repo: &Path,
    value: &Value,
    source: &ObservedSource,
) -> Result<Value> {
    let intent_package = value["intent_package_path"]
        .as_str()
        .ok_or_else(usage_error)?;
    let head = source.head_commit.as_deref().ok_or_else(|| {
        GatePolicyError::new(
            ErrorClass::Identity,
            "GATE-PLAN-PACKAGE-AUTHORITY",
            "package authority requires a committed head",
        )
    })?;
    validate_package_chain(
        repo,
        &source.base_commit,
        Some(head),
        Path::new(intent_package),
    )
}

fn require_exact_package_authority(
    value: &Value,
    reconstructed: &Value,
    authorized: &[String],
) -> Result<()> {
    let paths = value["changed_paths"]
        .as_array()
        .ok_or_else(usage_error)?
        .iter()
        .map(|path| path.as_str().map(str::to_owned).ok_or_else(usage_error))
        .collect::<Result<Vec<_>>>()?;
    if value["status"] != "READY" || paths != authorized || value != reconstructed {
        return Err(GatePolicyError::new(
            ErrorClass::Identity,
            "GATE-PLAN-PACKAGE-AUTHORITY",
            "package authority chain does not bind authorized paths",
        ));
    }
    Ok(())
}

fn authorized_paths(options: &BTreeMap<String, String>) -> Result<Vec<String>> {
    let value = read_json(Path::new(required(options, "authorized-paths")?))?;
    value
        .as_array()
        .ok_or_else(usage_error)?
        .iter()
        .map(|path| path.as_str().map(str::to_owned).ok_or_else(usage_error))
        .collect()
}

fn planning_stage(options: &BTreeMap<String, String>) -> Result<PlanningStage> {
    let stage = match required(options, "stage")? {
        "intent" => PlanningStage::Intent,
        "terminal" => PlanningStage::Terminal,
        _ => return Err(usage_error()),
    };
    Ok(stage)
}

fn planning_source(repo: &Path, options: &BTreeMap<String, String>) -> Result<ObservedSource> {
    let base = required(options, "base")?;
    let source = if let Some(head) = options.get("head") {
        observe_committed(repo, base, head)?
    } else {
        observe_dirty(repo, base)?
    };
    Ok(source)
}

fn boundary(options: &BTreeMap<String, String>) -> String {
    options
        .get("boundary")
        .cloned()
        .unwrap_or_else(|| "INCREMENT".to_owned())
}

fn persist_plan(repo: &Path, options: &BTreeMap<String, String>, plan: &Value) -> Result<PathBuf> {
    persist_named(repo, options, "output", plan)
}

fn persist_named(
    repo: &Path,
    options: &BTreeMap<String, String>,
    name: &str,
    value: &Value,
) -> Result<PathBuf> {
    let output = PathBuf::from(required(options, name)?);
    write_plan_confined(repo, &output, &canonical_bytes(value)?)?;
    Ok(output)
}

#[cfg(target_os = "linux")]
fn write_plan_confined(repo: &Path, output: &Path, bytes: &[u8]) -> Result<()> {
    let (parent_fd, output_name) = confined_output_parent(repo, output)?;
    let (temporary_fd, temporary_name) = reserve_temporary_output(&parent_fd, output)?;
    persist_reserved_output(
        parent_fd,
        &output_name,
        temporary_fd,
        &temporary_name,
        bytes,
    )
}

#[cfg(target_os = "linux")]
fn confined_output_parent(
    repo: &Path,
    output: &Path,
) -> Result<(std::os::fd::OwnedFd, std::ffi::OsString)> {
    use rustix::fs::{Mode, OFlags, ResolveFlags, openat2};
    use std::os::fd::AsRawFd;

    let parent = output
        .parent()
        .filter(|parent| !parent.as_os_str().is_empty())
        .unwrap_or_else(|| Path::new("."));
    let parent_fd = openat2(
        rustix::fs::CWD,
        parent,
        OFlags::RDONLY | OFlags::DIRECTORY | OFlags::CLOEXEC,
        Mode::empty(),
        ResolveFlags::NO_MAGICLINKS | ResolveFlags::NO_SYMLINKS,
    )
    .map_err(|error| {
        GatePolicyError::new(ErrorClass::Io, "GATE-CLI-OUTPUT-DIR", error.to_string())
    })?;
    let stable_parent = fs::canonicalize(format!("/proc/self/fd/{}", parent_fd.as_raw_fd()))
        .map_err(|error| {
            GatePolicyError::new(ErrorClass::Io, "GATE-CLI-OUTPUT-DIR", error.to_string())
        })?;
    let repository = fs::canonicalize(repo).map_err(|error| {
        GatePolicyError::new(ErrorClass::Io, "GATE-CLI-REPO", error.to_string())
    })?;
    if stable_parent.starts_with(repository) {
        return Err(GatePolicyError::new(
            ErrorClass::Cli,
            "GATE-CLI-OUTPUT-IN-REPOSITORY",
            "plan output must be outside the observed repository",
        ));
    }
    let output_name = output.file_name().ok_or_else(|| {
        GatePolicyError::new(
            ErrorClass::Cli,
            "GATE-CLI-OUTPUT",
            "missing output file name",
        )
    })?;
    Ok((parent_fd, output_name.to_owned()))
}

#[cfg(target_os = "linux")]
fn reserve_temporary_output(
    parent_fd: &std::os::fd::OwnedFd,
    output: &Path,
) -> Result<(std::os::fd::OwnedFd, String)> {
    use rustix::fs::{Mode, OFlags, openat};

    let output_name = output
        .file_name()
        .and_then(|name| name.to_str())
        .ok_or_else(|| {
            GatePolicyError::new(
                ErrorClass::Cli,
                "GATE-CLI-OUTPUT",
                "output path needs a UTF-8 file name",
            )
        })?;
    for nonce in 0_u8..16 {
        let temporary = format!(".{output_name}.tmp-{}-{nonce}", std::process::id());
        match openat(
            parent_fd,
            &temporary,
            OFlags::WRONLY | OFlags::CREATE | OFlags::EXCL | OFlags::NOFOLLOW | OFlags::CLOEXEC,
            Mode::RUSR | Mode::WUSR,
        ) {
            Ok(file) => return Ok((file, temporary)),
            Err(error) if error == rustix::io::Errno::EXIST => {}
            Err(error) => {
                return Err(GatePolicyError::new(
                    ErrorClass::Io,
                    "GATE-CLI-WRITE",
                    error.to_string(),
                ));
            }
        }
    }
    Err(GatePolicyError::new(
        ErrorClass::Io,
        "GATE-CLI-WRITE",
        "could not reserve a unique temporary output",
    ))
}

#[cfg(target_os = "linux")]
fn persist_reserved_output(
    parent_fd: std::os::fd::OwnedFd,
    output_name: &std::ffi::OsStr,
    temporary_fd: std::os::fd::OwnedFd,
    temporary_name: &str,
    bytes: &[u8],
) -> Result<()> {
    use rustix::fs::{AtFlags, renameat, unlinkat};

    let mut file = fs::File::from(temporary_fd);
    let written = file.write_all(bytes).and_then(|()| file.sync_all());
    drop(file);
    if let Err(error) = written {
        let _cleanup = unlinkat(&parent_fd, temporary_name, AtFlags::empty());
        return Err(GatePolicyError::new(
            ErrorClass::Io,
            "GATE-CLI-WRITE",
            error.to_string(),
        ));
    }
    if let Err(error) = renameat(&parent_fd, temporary_name, &parent_fd, output_name) {
        let _cleanup = unlinkat(&parent_fd, temporary_name, AtFlags::empty());
        return Err(GatePolicyError::new(
            ErrorClass::Io,
            "GATE-CLI-RENAME",
            error.to_string(),
        ));
    }
    fs::File::from(parent_fd).sync_all().map_err(|error| {
        GatePolicyError::new(ErrorClass::Io, "GATE-CLI-DIR-SYNC", error.to_string())
    })
}

#[cfg(not(target_os = "linux"))]
fn write_plan_confined(_repo: &Path, _output: &Path, _bytes: &[u8]) -> Result<()> {
    Err(GatePolicyError::new(
        ErrorClass::Io,
        "GATE-CLI-OUTPUT-UNSUPPORTED",
        "descriptor-confined output is unavailable on this platform",
    ))
}

fn receipt_command(repo: &Path, options: &BTreeMap<String, String>) -> Result<Value> {
    let (plan, receipt) = receipt_inputs(options)?;
    let artifacts = receipt_artifacts(options)?;
    let verdict = verify_receipt(repo, &plan, &receipt, &artifacts)?;
    Ok(json!({
        "result": verdict.result(),
        "receipt_id": verdict.receipt_id(),
        "trust_class": verdict.trust_class(),
        "claimed_trust_class": verdict.claimed_trust_class()
    }))
}

fn receipt_envelope_command(repo: &Path, options: &BTreeMap<String, String>) -> Result<Value> {
    let (plan, receipt) = receipt_inputs(options)?;
    let artifacts = receipt_artifacts(options)?;
    let verdict = verify_receipt_envelope(repo, &plan, &receipt, &artifacts)?;
    Ok(json!({
        "result": verdict.result(),
        "receipt_id": verdict.receipt_id(),
        "trust_class": verdict.trust_class(),
        "claimed_trust_class": verdict.claimed_trust_class()
    }))
}

fn receipt_inputs(options: &BTreeMap<String, String>) -> Result<(Value, Value)> {
    let plan_path = required(options, "plan")?;
    let receipt_path = required(options, "receipt")?;
    Ok((
        read_json(Path::new(plan_path))?,
        read_json(Path::new(receipt_path))?,
    ))
}

fn receipt_artifacts(options: &BTreeMap<String, String>) -> Result<DirectoryArtifacts> {
    required(options, "artifact-root")
        .map(PathBuf::from)
        .map(DirectoryArtifacts::new)
}

fn ledger_command(repo: &Path, options: &BTreeMap<String, String>) -> Result<Value> {
    let ledger = read_json(Path::new(required(options, "ledger")?))?;
    let predecessor = options
        .get("predecessor")
        .map(|path| read_json(Path::new(path)))
        .transpose()?;
    verify_campaign_ledger(repo, &ledger, predecessor.as_ref())?;
    Ok(json!({"result": "PASS", "ledger_id": ledger["ledger_id"]}))
}

fn assurance_command(repo: &Path, options: &BTreeMap<String, String>) -> Result<Value> {
    let record = read_json(Path::new(required(options, "record")?))?;
    verify_assurance_impact(repo, &record)?;
    Ok(json!({"result": "PASS", "record_id": record["record_id"]}))
}

fn parse_options(arguments: &[String]) -> Result<BTreeMap<String, String>> {
    let pairs = arguments.chunks_exact(2);
    if !pairs.remainder().is_empty() {
        return Err(usage_error());
    }
    let mut options = BTreeMap::new();
    for pair in pairs {
        insert_option(&mut options, &pair[0], &pair[1])?;
    }
    Ok(options)
}

fn insert_option(
    options: &mut BTreeMap<String, String>,
    argument: &str,
    value: &str,
) -> Result<()> {
    let key = argument.strip_prefix("--").ok_or_else(usage_error)?;
    if value.starts_with("--") {
        return Err(usage_error());
    }
    reject_duplicate(options.insert(key.to_owned(), value.to_owned()))
}

fn reject_duplicate(previous: Option<String>) -> Result<()> {
    previous.map_or(Ok(()), |_| Err(usage_error()))
}

fn required<'a>(options: &'a BTreeMap<String, String>, key: &str) -> Result<&'a str> {
    options.get(key).map(String::as_str).ok_or_else(usage_error)
}

fn read_json(path: &Path) -> Result<Value> {
    let bytes = fs::read(path).map_err(|error| {
        GatePolicyError::new(
            ErrorClass::Io,
            "GATE-CLI-READ",
            format!("{}: {error}", path.display()),
        )
    })?;
    parse_strict(&bytes)
}

fn usage_error() -> GatePolicyError {
    GatePolicyError::new(
        ErrorClass::Cli,
        "GATE-CLI-USAGE",
        "usage: openwepp-gate-plan <plan|validate-package-chain|run|reconcile|reconcile-attempts|close-tooling-defect|verify-receipt|verify-receipt-envelope|verify-ledger|verify-assurance> --key value ...",
    )
}

#[cfg(test)]
mod tests {
    use std::collections::BTreeMap;
    use std::fs;
    use std::path::{Path, PathBuf};
    use std::process::Command;
    use std::sync::atomic::{AtomicU64, Ordering};

    use super::{
        close_tooling_defect_command, package_authority, parse_options, plan_request,
        require_exact_package_authority, run_arguments, staged_run_command,
        validate_package_chain_command, validate_transition_outputs, write_plan_confined,
    };
    use openwepp_gate_planner::executor::ExecutionClaims;
    use openwepp_gate_planner::pre_heavy::append_attempt_record;
    use openwepp_gate_planner::repository::ObservedSource;
    use serde_json::json;

    fn arguments(values: &[&str]) -> Vec<String> {
        values.iter().map(|value| (*value).to_owned()).collect()
    }

    #[test]
    fn options_require_complete_unique_key_value_pairs() {
        let parsed =
            parse_options(&arguments(&["--repo", ".", "--base", "HEAD"])).expect("valid options");
        assert_eq!(parsed.get("base").map(String::as_str), Some("HEAD"));

        for invalid in [
            arguments(&["repo", "."]),
            arguments(&["--repo"]),
            arguments(&["--repo", "--base"]),
            arguments(&["--repo", ".", "--repo", "elsewhere"]),
        ] {
            let error = parse_options(&invalid).expect_err("invalid options must fail");
            assert_eq!(error.code, "GATE-CLI-USAGE");
        }
    }

    #[test]
    fn tooling_defect_closure_cli_persists_the_canonical_result() {
        let root = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join(format!(
            "../../target/openwepp-gate-closure-cli-{}",
            std::process::id()
        ));
        let history = root.join("history");
        fs::create_dir_all(history.join("recovery")).expect("history");
        let ledger = history.join("attempts.jsonl");
        fs::write(&ledger, "").expect("ledger");
        append_attempt_record(
            &ledger,
            json!({
                "record_type": "TOOLING_DEFECT",
                "defect_id": "AUTO-cli",
                "status": "OPEN",
            }),
        )
        .expect("open defect");
        let result = close_tooling_defect_command(
            &root,
            &BTreeMap::from([
                ("ledger".to_owned(), ledger.display().to_string()),
                ("defect-id".to_owned(), "AUTO-cli".to_owned()),
                ("correction-commit".to_owned(), "c".repeat(40)),
                (
                    "closure-evidence".to_owned(),
                    "dual review passed".to_owned(),
                ),
                (
                    "invalidated-recovery-root".to_owned(),
                    history.join("recovery/failed").display().to_string(),
                ),
            ]),
        )
        .expect("closure command");
        assert_eq!(result["result"], "CLOSED");
        assert_eq!(result["defect_id"], "AUTO-cli");
        fs::remove_dir_all(root).expect("remove scratch");
    }

    #[test]
    fn planning_rejects_forged_package_authority_artifacts() {
        let authorized = vec!["src/lib.rs".to_owned()];
        let live = json!({
            "status": "READY",
            "changed_paths": authorized,
            "package_authority_chain_id": "a".repeat(64)
        });
        require_exact_package_authority(&live, &live, &authorized)
            .expect("exact reconstructed authority");
        let mut forged = live.clone();
        forged["package_authority_chain_id"] = json!("b".repeat(64));
        let error = require_exact_package_authority(&forged, &live, &authorized)
            .expect_err("forged authority");
        assert_eq!(error.code, "GATE-PLAN-PACKAGE-AUTHORITY");
    }

    #[test]
    fn package_chain_command_and_plan_request_preserve_authority_identity() {
        let fixture = PlanningFixture::new();
        fixture.write_package("child", "- `docs/work-packages/child/**`\n- `src/input.rs`");
        let base = fixture.commit("child authority");
        fixture.write_source("src/input.rs", "pub fn input() {}\n");
        let head = fixture.commit("authorized source");
        let output = fixture.external("package-authority-chain.json");
        let command_options = BTreeMap::from([
            ("base".to_owned(), base.clone()),
            ("head".to_owned(), head.clone()),
            (
                "package".to_owned(),
                "docs/work-packages/child/package.md".to_owned(),
            ),
            ("output".to_owned(), output.display().to_string()),
        ]);

        let summary = validate_package_chain_command(&fixture.root, &command_options)
            .expect("package chain command");
        assert_eq!(summary["result"], "READY");
        let authority = openwepp_gate_planner::canonical::parse_strict(
            &fs::read(&output).expect("persisted package authority"),
        )
        .expect("strict package authority JSON");
        assert_eq!(
            summary["package_authority_chain_id"],
            authority["package_authority_chain_id"]
        );
        assert_eq!(summary["output"], output.display().to_string());

        let authorized = fixture.external("authorized-paths.json");
        fs::write(
            &authorized,
            serde_json::to_vec(&authority["changed_paths"]).expect("authorized path bytes"),
        )
        .expect("authorized paths");
        let plan_options = BTreeMap::from([
            ("stage".to_owned(), "intent".to_owned()),
            ("base".to_owned(), base.clone()),
            ("head".to_owned(), head.clone()),
            (
                "authorized-paths".to_owned(),
                authorized.display().to_string(),
            ),
            (
                "package-authority-chain".to_owned(),
                output.display().to_string(),
            ),
        ]);
        let request = plan_request(&fixture.root, &plan_options).expect("bound plan request");
        assert_eq!(request.source.base_commit, base);
        assert_eq!(request.source.head_commit.as_deref(), Some(head.as_str()));
        assert_eq!(
            request.package_authority_chain_id,
            authority["package_authority_chain_id"]
                .as_str()
                .expect("authority identity")
        );
        assert_eq!(
            request.intent_package_path,
            "docs/work-packages/child/package.md"
        );
        assert_eq!(json!(request.authorized_paths), authority["changed_paths"]);
    }

    #[test]
    fn package_authority_requires_a_committed_head_before_reconstruction() {
        let fixture = PlanningFixture::new();
        let authority = fixture.external("authority.json");
        fs::write(
            &authority,
            r#"{"intent_package_path":"docs/work-packages/child/package.md"}"#,
        )
        .expect("authority fixture");
        let options = BTreeMap::from([(
            "package-authority-chain".to_owned(),
            authority.display().to_string(),
        )]);
        let source = ObservedSource {
            base_commit: "a".repeat(40),
            head_commit: None,
            dirty_tree_digest: Some("c".repeat(64)),
            index_digest: Some("d".repeat(64)),
            worktree_digest: Some("e".repeat(64)),
            untracked_digest: Some("f".repeat(64)),
            changes: Vec::new(),
        };

        let error = package_authority(&fixture.root, &options, &[], &source)
            .expect_err("dirty source cannot bind package authority");
        assert_eq!(error.code, "GATE-PLAN-PACKAGE-AUTHORITY");
        assert_eq!(error.message, "package authority requires a committed head");
    }

    #[test]
    fn standalone_heavy_rejects_an_unauthenticated_ready_document() {
        let repo = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../..");
        let root = repo.join("target").join(format!(
            "testgate-heavy-lifecycle-{}-{}",
            std::process::id(),
            std::thread::current().name().unwrap_or("test")
        ));
        fs::create_dir_all(&root).expect("test root");
        let ledger = root.join("attempts.jsonl");
        let audit = root.join("audit.json");
        fs::write(&ledger, "").expect("ledger");
        fs::write(&audit, r#"{"status":"READY"}"#).expect("forged audit");
        let options = BTreeMap::from([
            ("resume".to_owned(), ledger.display().to_string()),
            ("audit".to_owned(), audit.display().to_string()),
            (
                "output".to_owned(),
                root.join("receipt.json").display().to_string(),
            ),
        ]);
        let error = staged_run_command(
            &repo,
            &options,
            &json!({"plan_id": "1".repeat(64)}),
            &root,
            &ExecutionClaims::default(),
            "heavy",
        )
        .expect_err("standalone HEAVY must fail");
        assert_eq!(error.code, "GATE-EXEC-AUDIT-UNAUTHENTICATED");
        let records = fs::read_to_string(&ledger).expect("ledger records");
        assert!(
            records.is_empty(),
            "rejected transport is not an admitted attempt"
        );
        fs::remove_dir_all(root).expect("remove test root");
    }

    #[test]
    fn command_dispatch_fails_closed_for_unknown_or_incomplete_requests() {
        let cases = [
            arguments(&[]),
            arguments(&["unknown"]),
            arguments(&["plan", "--stage", "unsupported"]),
            arguments(&["reconcile"]),
            arguments(&["verify-receipt"]),
            arguments(&["verify-ledger"]),
            arguments(&["verify-assurance"]),
        ];
        for case in cases {
            let error = run_arguments(case).expect_err("incomplete command must fail");
            assert_eq!(error.code, "GATE-CLI-USAGE");
        }
    }

    #[cfg(target_os = "linux")]
    #[test]
    fn transition_rejects_missing_or_colliding_outputs_before_execution() {
        let repo = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../..");
        let empty = BTreeMap::new();
        let error = validate_transition_outputs(&repo, &empty)
            .expect_err("missing transition outputs must fail");
        assert_eq!(error.code, "GATE-CLI-USAGE");

        let scratch = std::env::temp_dir().join(format!(
            "testgate-transition-preflight-{}",
            std::process::id()
        ));
        fs::create_dir_all(&scratch).expect("transition preflight scratch");
        let history = scratch.join("history");
        fs::create_dir(&history).expect("transition history");
        let same = history.join("attempts.jsonl").display().to_string();
        fs::write(&same, "").expect("transition ledger");
        let plan = scratch.join("plan.json");
        fs::write(&plan, "{}").expect("transition plan");
        let alias = history
            .join("..")
            .join("history")
            .join("attempts.jsonl")
            .display()
            .to_string();
        let options = BTreeMap::from([
            ("resume".to_owned(), same.clone()),
            ("plan".to_owned(), plan.display().to_string()),
            ("light-output".to_owned(), alias),
            (
                "audit-output".to_owned(),
                scratch.join("audit.json").display().to_string(),
            ),
            (
                "output".to_owned(),
                scratch.join("receipt.json").display().to_string(),
            ),
        ]);
        let error = validate_transition_outputs(&repo, &options)
            .expect_err("colliding transition outputs must fail");
        assert_eq!(error.code, "GATE-CLI-OUTPUT-COLLISION");

        let target = scratch.join("symlink-target.json");
        fs::write(&target, "ledger").expect("symlink target");
        let link = scratch.join("ledger-link.jsonl");
        std::os::unix::fs::symlink(&target, &link).expect("ledger symlink");
        let symlink_options = BTreeMap::from([
            ("resume".to_owned(), link.display().to_string()),
            ("plan".to_owned(), plan.display().to_string()),
            (
                "light-output".to_owned(),
                scratch.join("light.json").display().to_string(),
            ),
            (
                "audit-output".to_owned(),
                scratch.join("audit.json").display().to_string(),
            ),
            ("output".to_owned(), target.display().to_string()),
        ]);
        let error = validate_transition_outputs(&repo, &symlink_options)
            .expect_err("final-component ledger symlink must fail");
        assert_eq!(error.code, "GATE-CLI-INPUT-SYMLINK");
        fs::remove_dir_all(scratch).expect("remove transition preflight scratch");
    }

    #[cfg(target_os = "linux")]
    #[test]
    fn confined_plan_output_is_atomic_and_outside_the_repository() {
        let repo = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("../..");
        let scratch = std::env::temp_dir().join(format!(
            "openwepp-gate-plan-confined-{}-{}",
            std::process::id(),
            std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .expect("system clock is after the Unix epoch")
                .as_nanos()
        ));
        std::fs::create_dir(&scratch).expect("create test scratch directory");
        let output = scratch.join("gate-plan.json");

        write_plan_confined(&repo, &output, b"first").expect("write outside repository");
        assert_eq!(std::fs::read(&output).expect("read output"), b"first");
        write_plan_confined(&repo, &output, b"replacement").expect("replace atomically");
        assert_eq!(
            std::fs::read(&output).expect("read replacement"),
            b"replacement"
        );

        let in_repository = repo.join("target/confined-output-must-not-exist.json");
        let error = write_plan_confined(&repo, &in_repository, b"forbidden")
            .expect_err("repository-confined output must fail closed");
        assert_eq!(error.code, "GATE-CLI-OUTPUT-IN-REPOSITORY");
        std::fs::remove_dir_all(&scratch).expect("remove test scratch directory");
    }

    static PLANNING_SEQUENCE: AtomicU64 = AtomicU64::new(0);

    struct PlanningFixture {
        root: PathBuf,
        external_root: PathBuf,
    }

    impl PlanningFixture {
        fn new() -> Self {
            let sequence = PLANNING_SEQUENCE.fetch_add(1, Ordering::Relaxed);
            let root = std::env::temp_dir().join(format!(
                "openwepp-main-planning-{}-{sequence}",
                std::process::id()
            ));
            let external_root = std::env::temp_dir().join(format!(
                "openwepp-main-planning-output-{}-{sequence}",
                std::process::id()
            ));
            fs::create_dir_all(root.join("gate-policy/v1/schemas")).expect("schema directory");
            fs::create_dir(&external_root).expect("external output directory");
            fs::write(
                root.join("gate-policy/v1/schemas/package-authority-chain.schema.json"),
                "{\"type\":\"object\"}\n",
            )
            .expect("permissive fixture schema");
            Self::git(&root, &["init", "-q"]);
            Self::git(&root, &["config", "user.email", "test@example.invalid"]);
            Self::git(&root, &["config", "user.name", "Test"]);
            fs::write(root.join("README.md"), "# Fixture\n").expect("fixture root");
            Self {
                root,
                external_root,
            }
        }

        fn write_package(&self, name: &str, write_set: &str) {
            let directory = self.root.join(format!("docs/work-packages/{name}"));
            fs::create_dir_all(directory.join("prompts/active")).expect("active prompt directory");
            fs::write(
                directory.join("package.md"),
                format!("# {name}\n\nStatus: `ACTIVE`\n\n## Intended Write Set\n\n{write_set}\n"),
            )
            .expect("package text");
            fs::write(directory.join("prompts/active/kickoff.md"), "# Kickoff\n")
                .expect("active prompt");
        }

        fn write_source(&self, path: &str, text: &str) {
            let path = self.root.join(path);
            fs::create_dir_all(path.parent().expect("source parent")).expect("source directory");
            fs::write(path, text).expect("source text");
        }

        fn external(&self, name: &str) -> PathBuf {
            self.external_root.join(name)
        }

        fn commit(&self, message: &str) -> String {
            Self::git(&self.root, &["add", "."]);
            Self::git(&self.root, &["commit", "-qm", message]);
            String::from_utf8(Self::git_output(&self.root, &["rev-parse", "HEAD"]))
                .expect("UTF-8 commit")
                .trim()
                .to_owned()
        }

        fn git(repo: &Path, args: &[&str]) {
            let _output = Self::git_output(repo, args);
        }

        fn git_output(repo: &Path, args: &[&str]) -> Vec<u8> {
            let output = Command::new("git")
                .args(args)
                .current_dir(repo)
                .output()
                .expect("git command");
            assert!(
                output.status.success(),
                "git {:?}: {}",
                args,
                String::from_utf8_lossy(&output.stderr)
            );
            output.stdout
        }
    }

    impl Drop for PlanningFixture {
        fn drop(&mut self) {
            fs::remove_dir_all(&self.root).expect("remove planning fixture");
            fs::remove_dir_all(&self.external_root).expect("remove planning fixture outputs");
        }
    }
}
