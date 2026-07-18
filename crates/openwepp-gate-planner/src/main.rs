use std::collections::BTreeMap;
use std::fs;
use std::io::Write;
use std::path::{Path, PathBuf};

use openwepp_gate_planner::canonical::{canonical_bytes, parse_strict};
use openwepp_gate_planner::error::{ErrorClass, GatePolicyError, Result};
use openwepp_gate_planner::ledger::{verify_assurance_impact, verify_campaign_ledger};
use openwepp_gate_planner::planner::reconcile_intent_terminal;
use openwepp_gate_planner::planner::{NextestInventory, PlanRequest, Planner, PlanningStage};
use openwepp_gate_planner::repository::{ObservedSource, observe_committed, observe_dirty};
use openwepp_gate_planner::verifier::{DirectoryArtifacts, verify_receipt};
use serde_json::{Value, json};

type CommandHandler = fn(&Path, &BTreeMap<String, String>) -> Result<Value>;

const COMMANDS: [(&str, CommandHandler); 5] = [
    ("plan", plan_command),
    ("verify-receipt", receipt_command),
    ("verify-ledger", ledger_command),
    ("verify-assurance", assurance_command),
    ("reconcile", reconcile_command),
];

fn main() {
    match run() {
        Ok(value) => match serde_json::to_string(&value) {
            Ok(text) => println!("{text}"),
            Err(error) => {
                eprintln!("GATE-CLI-SERIALIZE: {error}");
                std::process::exit(2);
            }
        },
        Err(error) => {
            eprintln!("{error}");
            std::process::exit(2);
        }
    }
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
            "output",
            "predecessor",
            "authorized-paths",
        ],
        "reconcile" => &["repo", "intent", "terminal"],
        "verify-receipt" => &["repo", "plan", "receipt", "artifact-root"],
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
    use rustix::fs::{AtFlags, Mode, OFlags, ResolveFlags, openat, openat2, renameat, unlinkat};
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
    for nonce in 0_u8..16 {
        let temporary = format!(
            ".{}.tmp-{}-{nonce}",
            output
                .file_name()
                .and_then(|name| name.to_str())
                .ok_or_else(|| {
                    GatePolicyError::new(
                        ErrorClass::Cli,
                        "GATE-CLI-OUTPUT",
                        "output path needs a UTF-8 file name",
                    )
                })?,
            std::process::id()
        );
        let temporary_fd = match openat(
            &parent_fd,
            &temporary,
            OFlags::WRONLY | OFlags::CREATE | OFlags::EXCL | OFlags::NOFOLLOW | OFlags::CLOEXEC,
            Mode::RUSR | Mode::WUSR,
        ) {
            Ok(file) => file,
            Err(error) if error == rustix::io::Errno::EXIST => {
                continue;
            }
            Err(error) => {
                return Err(GatePolicyError::new(
                    ErrorClass::Io,
                    "GATE-CLI-WRITE",
                    error.to_string(),
                ));
            }
        };
        let mut file = fs::File::from(temporary_fd);
        let written = file.write_all(bytes).and_then(|()| file.sync_all());
        drop(file);
        if let Err(error) = written {
            let _cleanup = unlinkat(&parent_fd, &temporary, AtFlags::empty());
            return Err(GatePolicyError::new(
                ErrorClass::Io,
                "GATE-CLI-WRITE",
                error.to_string(),
            ));
        }
        if let Err(error) = renameat(&parent_fd, &temporary, &parent_fd, output_name) {
            let _cleanup = unlinkat(&parent_fd, &temporary, AtFlags::empty());
            return Err(GatePolicyError::new(
                ErrorClass::Io,
                "GATE-CLI-RENAME",
                error.to_string(),
            ));
        }
        fs::File::from(parent_fd).sync_all().map_err(|error| {
            GatePolicyError::new(ErrorClass::Io, "GATE-CLI-DIR-SYNC", error.to_string())
        })?;
        return Ok(());
    }
    Err(GatePolicyError::new(
        ErrorClass::Io,
        "GATE-CLI-WRITE",
        "could not reserve a unique temporary output",
    ))
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
        "usage: openwepp-gate-plan <plan|reconcile|verify-receipt|verify-ledger|verify-assurance> --key value ...",
    )
}

#[cfg(test)]
mod tests {
    use super::{parse_options, run_arguments};

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
}
