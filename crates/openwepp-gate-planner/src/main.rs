use std::collections::BTreeMap;
use std::fs;
use std::io::Write;
use std::path::{Path, PathBuf};
use std::time::Instant;

use openwepp_gate_planner::canonical::{canonical_bytes, parse_strict};
use openwepp_gate_planner::error::{ErrorClass, GatePolicyError, Result};
use openwepp_gate_planner::executor::{ExecutionClaims, execute_plan, execute_plan_stage};
use openwepp_gate_planner::ledger::{verify_assurance_impact, verify_campaign_ledger};
use openwepp_gate_planner::package_validation::validate_package;
use openwepp_gate_planner::planner::reconcile_intent_terminal;
use openwepp_gate_planner::planner::{NextestInventory, PlanRequest, Planner, PlanningStage};
use openwepp_gate_planner::pre_heavy::{
    admit_attempt_ledger, append_attempt_record, build_audit, build_failure_audit,
    reconcile_orphaned_attempts, record_heavy_failure, validate_resume_ledger,
};
use openwepp_gate_planner::repository::{ObservedSource, observe_committed, observe_dirty};
use openwepp_gate_planner::resume::load_candidate;
use openwepp_gate_planner::verifier::{
    DirectoryArtifacts, verify_receipt, verify_receipt_envelope,
};
use serde_json::{Value, json};

type CommandHandler = fn(&Path, &BTreeMap<String, String>) -> Result<Value>;

const COMMANDS: [(&str, CommandHandler); 10] = [
    ("plan", plan_command),
    ("run", run_command),
    ("verify-receipt", receipt_command),
    ("verify-receipt-envelope", receipt_envelope_command),
    ("verify-ledger", ledger_command),
    ("verify-assurance", assurance_command),
    ("reconcile", reconcile_command),
    ("pre-heavy-audit", pre_heavy_audit_command),
    ("validate-package", validate_package_command),
    ("reconcile-attempts", reconcile_attempts_command),
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
    let handler = COMMANDS
        .iter()
        .find(|(name, _)| *name == command)
        .map(|(_, handler)| *handler)
        .ok_or_else(usage_error)?;
    reject_unknown_options(command, options)?;
    handler(repo, options)
}

fn reject_unknown_options(command: &str, options: &BTreeMap<String, String>) -> Result<()> {
    let allowed: &[&str] = match command {
        "plan" => &[
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
        ],
        "run" => &[
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
        ],
        "pre-heavy-audit" => &[
            "repo",
            "plan",
            "light-receipts",
            "artifact-root",
            "ledger",
            "output",
        ],
        "validate-package" => &["repo", "base", "package", "output"],
        "reconcile-attempts" => &["repo", "ledger"],
        "reconcile" => &["repo", "intent", "terminal"],
        "verify-receipt" | "verify-receipt-envelope" => {
            &["repo", "plan", "receipt", "artifact-root"]
        }
        "verify-ledger" => &["repo", "ledger", "predecessor"],
        "verify-assurance" => &["repo", "record"],
        _ => return Err(usage_error()),
    };
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
    let ledger = PathBuf::from(required(options, "resume")?);
    admit_attempt_ledger(&ledger)?;
    reconcile_orphaned_attempts(&ledger)?;
    let started = Instant::now();
    let recovery_root = std::env::var_os("OPENWEPP_GATE_CHECKPOINT_MIRROR_ROOT")
        .map(PathBuf::from)
        .map_or(Value::Null, |path| json!(path.display().to_string()));
    let audit_path = options.get("audit").map(PathBuf::from);
    let submitted_audit_id = audit_path
        .as_ref()
        .and_then(|path| fs::read(path).ok())
        .and_then(|bytes| parse_strict(&bytes).ok())
        .map_or(Value::Null, |value| value["audit_id"].clone());
    let started_entry_sha256 = append_attempt_record(
        &ledger,
        json!({
            "record_type": "STAGE_ATTEMPT",
            "status": "STARTED",
            "stage": "HEAVY",
            "plan_id": plan["plan_id"],
            "audit_id": submitted_audit_id,
            "phase": "ADMISSION",
            "artifact_root": artifact_root.display().to_string(),
            "recovery_root": recovery_root,
            "workflow": claims.workflow,
            "job": claims.job,
            "runner": claims.runner,
            "attempt": claims.attempt,
        }),
    )?;
    let execute = || -> Result<Value> {
        let audit_path = audit_path.as_ref().ok_or_else(usage_error)?;
        let audit = read_json(&audit_path)?;
        validate_resume_ledger(repo, plan, &audit, artifact_root, &ledger)?;
        let resume_candidate = load_candidate(repo, plan, &ledger, claims)?;
        let receipt = execute_plan_stage(
            repo,
            plan,
            artifact_root,
            claims,
            "HEAVY",
            Some(&audit),
            resume_candidate.as_ref(),
        )?;
        verify_receipt(
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
    };
    match execute() {
        Ok(value) => {
            append_attempt_record(
                &ledger,
                json!({
                    "record_type": "STAGE_ATTEMPT",
                    "status": "CLOSED",
                    "stage": "HEAVY",
                    "plan_id": plan["plan_id"],
                    "audit_id": submitted_audit_id,
                    "artifact_root": artifact_root.display().to_string(),
                    "recovery_root": recovery_root,
                    "workflow": claims.workflow,
                    "job": claims.job,
                    "runner": claims.runner,
                    "attempt": claims.attempt,
                    "receipt_id": value["receipt_id"],
                    "result": value["result"],
                    "wall_time_ms": started.elapsed().as_millis() as u64,
                    "started_entry_sha256": started_entry_sha256,
                }),
            )?;
            Ok(value)
        }
        Err(error) => {
            let cause_key = error.code;
            record_heavy_failure(
                &ledger,
                json!({
                    "record_type": "STAGE_ATTEMPT",
                    "status": "FAILED",
                    "stage": "HEAVY",
                    "plan_id": plan["plan_id"],
                    "audit_id": submitted_audit_id,
                    "artifact_root": artifact_root.display().to_string(),
                    "recovery_root": recovery_root,
                    "workflow": claims.workflow,
                    "job": claims.job,
                    "runner": claims.runner,
                    "attempt": claims.attempt,
                    "result": null,
                    "error_code": error.code,
                    "error_message": error.message,
                    "cause_key": cause_key,
                    "failure_class": if cause_key.contains("SPAWN") || cause_key.contains("TIMEOUT") || cause_key.contains("RUNNER") {"INFRASTRUCTURE"} else {"TOOLING"},
                    "wall_time_ms": started.elapsed().as_millis() as u64,
                    "started_entry_sha256": started_entry_sha256,
                }),
                cause_key,
            )?;
            Err(error)
        }
    }
}

fn reconcile_attempts_command(_repo: &Path, options: &BTreeMap<String, String>) -> Result<Value> {
    let reconciled = reconcile_orphaned_attempts(Path::new(required(options, "ledger")?))?;
    Ok(json!({"result": "PASS", "reconciled_attempts": reconciled}))
}

fn pre_heavy_audit_command(repo: &Path, options: &BTreeMap<String, String>) -> Result<Value> {
    let plan = read_json(Path::new(required(options, "plan")?))?;
    let artifact_root = PathBuf::from(required(options, "artifact-root")?);
    let ledger = PathBuf::from(required(options, "ledger")?);
    let light_result = read_json(Path::new(required(options, "light-receipts")?));
    let audit = match light_result {
        Ok(light) => match build_audit(repo, &plan, &light, &artifact_root, &ledger) {
            Ok(audit) => audit,
            Err(failure) => {
                build_failure_audit(repo, &plan, &light, &artifact_root, &ledger, &failure)?
            }
        },
        Err(failure) => {
            let represented = GatePolicyError::new(
                failure.class,
                "GATE-AUDIT-LIGHT-INPUT-INVALID",
                format!("{}: {}", failure.code, failure.message),
            );
            build_failure_audit(
                repo,
                &plan,
                &json!({}),
                &artifact_root,
                &ledger,
                &represented,
            )?
        }
    };
    let output = persist_plan(repo, options, &audit)?;
    Ok(json!({
        "result": audit["status"],
        "audit_id": audit["audit_id"],
        "reason_codes": audit["reason_codes"],
        "output": output
    }))
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
    Ok(PlanRequest {
        stage: planning_stage(options)?,
        predecessor_intent_plan_id: options.get("predecessor").cloned(),
        boundary: boundary(options),
        campaign_id: options.get("campaign").cloned(),
        combined_quality_proof_id: options.get("combined-proof-id").cloned(),
        authorized_paths: authorized_paths(options)?,
        source: planning_source(repo, options)?,
    })
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
    let output = PathBuf::from(required(options, "output")?);
    write_plan_confined(repo, &output, &canonical_bytes(plan)?)?;
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
        "usage: openwepp-gate-plan <plan|run|reconcile|reconcile-attempts|verify-receipt|verify-receipt-envelope|verify-ledger|verify-assurance> --key value ...",
    )
}

#[cfg(test)]
mod tests {
    use std::collections::BTreeMap;
    use std::fs;
    use std::path::PathBuf;

    use super::{parse_options, run_arguments, staged_run_command, write_plan_confined};
    use openwepp_gate_planner::executor::ExecutionClaims;
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
    fn heavy_audit_parse_failure_has_balanced_lifecycle_records() {
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
        fs::write(&audit, "{").expect("invalid audit");
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
        .expect_err("invalid audit must fail");
        assert_eq!(error.code, "GATE-JSON-INVALID");
        let records = fs::read_to_string(&ledger).expect("ledger records");
        assert!(records.contains("\"status\":\"STARTED\""));
        assert!(records.contains("\"status\":\"FAILED\""));
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
}
