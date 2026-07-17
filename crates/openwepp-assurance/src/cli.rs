use std::env;
use std::ffi::OsString;
use std::fmt::Write as _;
use std::path::PathBuf;

use crate::{
    Assurance, AssuranceError, BuildOptions, Result, V2AmendMode, V2AssemblyResult,
    V2NormalizationMode, V2PublicationOptions, V2PublicationResult, V2RecoveryAction,
    V2ReleaseIdentity, V2ReleaseVerification, V2Repository, amend_attribution_at_generation,
    amend_lifecycle_at_generation, amend_normalize, amend_normalize_at_generation,
    amend_principal_at_generation, amend_role_at_generation, inspect_report, rebind_implementation,
    recover_amendment, verify_generation, verify_v2_release_snapshot,
};

const USAGE: &str = "Usage:\n  openwepp-assurance validate (--all | --report <id>)\n  openwepp-assurance inspect --report <id> [--format human|json]\n  openwepp-assurance amend attribution --principal <id> [--display-name <name>] [--affiliation <text>]... [--if-generation <id>] (--check | --apply)\n  openwepp-assurance amend principal --request <yaml> [--if-generation <id>] (--check | --apply)\n  openwepp-assurance amend role --report <id> --request <yaml> [--if-generation <id>] (--check | --apply)\n  openwepp-assurance amend normalize --report <id> --language en-US [--if-generation <id>] (--check | --apply)\n  openwepp-assurance amend recover (--inspect | --finish-cleanup | --restore-old)\n  openwepp-assurance amend rebind-implementation --all (--check | --apply)\n  openwepp-assurance lifecycle --report <id> --request <yaml> [--if-generation <id>] (--check | --apply)\n  openwepp-assurance verify-generation --base-ref <commit>\n  openwepp-assurance normalize --report <id> --language en-US (--check | --apply)\n  openwepp-assurance plan (--all | --report <id>) [--format human|json]\n  openwepp-assurance build (--all | --report <id>) --staging-root <path>\n  openwepp-assurance check (--all | --report <id>) --staging-root <path>\n  openwepp-assurance publish (--all | --report <id>) --staging-root <path> --usersum-root <path> --publication-snapshot-root <path> --release-commit <sha> --release-configuration <id>\n  openwepp-assurance publish-test-fixture (--all | --report <id>) --staging-root <path> --usersum-root <path> --publication-snapshot-root <path> --release-commit <sha> --release-configuration <id>\n  openwepp-assurance verify-release --all --snapshot-dir <path> --receipt <path> --release-commit <sha> --release-configuration <id>\n";

/// Parses process arguments and executes one assurance operation.
///
/// # Errors
///
/// Returns typed usage, validation, drift, snapshot, and filesystem errors.
pub fn run_from_env() -> Result<String> {
    run(env::args_os())
}

/// Executes the CLI using a supplied argument iterator.
///
/// # Errors
///
/// Returns typed usage, validation, drift, snapshot, and filesystem errors.
pub fn run<I, S>(args: I) -> Result<String>
where
    I: IntoIterator<Item = S>,
    S: Into<OsString>,
{
    let mut args = args.into_iter().map(Into::into);
    let _program = args.next();
    let command = utf8(args.next(), "command")?;
    if matches!(command.as_str(), "-h" | "--help" | "help") {
        ensure_no_arguments(args)?;
        return Ok(USAGE.to_owned());
    }
    if command == "amend" {
        let root = env::current_dir().map_err(|error| AssuranceError::io(".", error))?;
        return execute_amend(&root, args);
    }
    if command == "inspect" {
        let root = env::current_dir().map_err(|error| AssuranceError::io(".", error))?;
        return execute_inspect(&root, args);
    }
    if command == "lifecycle" {
        let root = env::current_dir().map_err(|error| AssuranceError::io(".", error))?;
        return execute_lifecycle(&root, args);
    }
    if command == "verify-generation" {
        let root = env::current_dir().map_err(|error| AssuranceError::io(".", error))?;
        return execute_verify_generation(&root, args);
    }
    let options = parse_options(args)?;
    let root = env::current_dir().map_err(|error| AssuranceError::io(".", error))?;
    let assurance = Assurance::open(&root)?;
    execute(&root, &assurance, &command, &options)
}

fn execute_amend<I>(root: &std::path::Path, mut args: I) -> Result<String>
where
    I: Iterator<Item = OsString>,
{
    let operation = utf8(args.next(), "amend operation")?;
    match operation.as_str() {
        "attribution" => execute_amend_attribution(root, args),
        "principal" => execute_amend_principal(root, args),
        "role" => execute_amend_role(root, args),
        "normalize" => execute_amend_normalize(root, args),
        "recover" => execute_amend_recover(root, args),
        "rebind-implementation" => execute_rebind_implementation(root, args),
        _ => Err(AssuranceError::Usage(format!(
            "unknown amend operation '{operation}'\n{USAGE}"
        ))),
    }
}

fn execute_rebind_implementation<I>(root: &std::path::Path, args: I) -> Result<String>
where
    I: Iterator<Item = OsString>,
{
    let mut all = false;
    let mut mode = None;
    for argument in args {
        match utf8(Some(argument), "rebind-implementation argument")?.as_str() {
            "--all" if !all => all = true,
            "--check" if mode.is_none() => mode = Some(V2AmendMode::Check),
            "--apply" if mode.is_none() => mode = Some(V2AmendMode::Apply),
            value => {
                return Err(AssuranceError::Usage(format!(
                    "unknown rebind-implementation argument '{value}'\n{USAGE}"
                )));
            }
        }
    }
    if !all {
        return Err(AssuranceError::Usage(
            "rebind-implementation requires --all".to_owned(),
        ));
    }
    rebind_implementation(root, *required_option(mode.as_ref(), "--check or --apply")?)?
        .render_json()
}

fn execute_amend_recover<I>(root: &std::path::Path, args: I) -> Result<String>
where
    I: Iterator<Item = OsString>,
{
    let mut action = None;
    for argument in args {
        let argument = argument
            .into_string()
            .map_err(|_| AssuranceError::Usage("recovery argument must be UTF-8".to_owned()))?;
        let selected = match argument.as_str() {
            "--inspect" => V2RecoveryAction::Inspect,
            "--finish-cleanup" => V2RecoveryAction::FinishCleanup,
            "--restore-old" => V2RecoveryAction::RestoreOld,
            _ => {
                return Err(AssuranceError::Usage(format!(
                    "unknown recovery argument '{argument}'\n{USAGE}"
                )));
            }
        };
        if action.replace(selected).is_some() {
            return Err(AssuranceError::Usage(
                "amend recover accepts exactly one action".to_owned(),
            ));
        }
    }
    recover_amendment(
        root,
        action
            .ok_or_else(|| AssuranceError::Usage("amend recover requires one action".to_owned()))?,
    )
}

fn execute_verify_generation<I>(root: &std::path::Path, mut args: I) -> Result<String>
where
    I: Iterator<Item = OsString>,
{
    let first = utf8(args.next(), "verify-generation argument")?;
    if first != "--base-ref" {
        return Err(AssuranceError::Usage(
            "verify-generation requires --base-ref <commit>".to_owned(),
        ));
    }
    let base_ref = next_string(&mut args, "--base-ref")?;
    ensure_no_arguments(args)?;
    verify_generation(root, &base_ref)
}

fn execute_amend_attribution<I>(root: &std::path::Path, mut args: I) -> Result<String>
where
    I: Iterator<Item = OsString>,
{
    let mut principal = None;
    let mut display_name = None;
    let mut affiliations = Vec::new();
    let mut if_generation = None;
    let mut mode = None;
    while let Some(argument) = args.next() {
        let argument = argument
            .into_string()
            .map_err(|_| AssuranceError::Usage("amend argument must be UTF-8".to_owned()))?;
        match argument.as_str() {
            "--principal" if principal.is_none() => {
                principal = Some(next_string(&mut args, "--principal")?);
            }
            "--display-name" if display_name.is_none() => {
                display_name = Some(next_string(&mut args, "--display-name")?);
            }
            "--affiliation" => affiliations.push(next_string(&mut args, "--affiliation")?),
            "--if-generation" if if_generation.is_none() => {
                if_generation = Some(next_string(&mut args, "--if-generation")?);
            }
            "--check" if mode.is_none() => mode = Some(V2AmendMode::Check),
            "--apply" if mode.is_none() => mode = Some(V2AmendMode::Apply),
            _ => {
                return Err(AssuranceError::Usage(format!(
                    "unknown or duplicate attribution argument '{argument}'\n{USAGE}"
                )));
            }
        }
    }
    let affiliations = (!affiliations.is_empty()).then_some(affiliations);
    amend_attribution_at_generation(
        root,
        required_option(principal.as_ref(), "--principal")?,
        display_name.as_deref(),
        affiliations,
        *required_option(mode.as_ref(), "--check or --apply")?,
        if_generation.as_deref(),
    )?
    .render_json()
}

fn execute_amend_principal<I>(root: &std::path::Path, mut args: I) -> Result<String>
where
    I: Iterator<Item = OsString>,
{
    let mut request = None;
    let mut if_generation = None;
    let mut mode = None;
    while let Some(argument) = args.next() {
        let argument = argument
            .into_string()
            .map_err(|_| AssuranceError::Usage("amend argument must be UTF-8".to_owned()))?;
        match argument.as_str() {
            "--request" if request.is_none() => {
                request = Some(PathBuf::from(next_string(&mut args, "--request")?));
            }
            "--if-generation" if if_generation.is_none() => {
                if_generation = Some(next_string(&mut args, "--if-generation")?);
            }
            "--check" if mode.is_none() => mode = Some(V2AmendMode::Check),
            "--apply" if mode.is_none() => mode = Some(V2AmendMode::Apply),
            _ => {
                return Err(AssuranceError::Usage(format!(
                    "unknown or duplicate principal argument '{argument}'\n{USAGE}"
                )));
            }
        }
    }
    let request = required_option(request.as_ref(), "--request")?;
    let bytes = std::fs::read(request).map_err(|error| AssuranceError::io(request, error))?;
    amend_principal_at_generation(
        root,
        &bytes,
        *required_option(mode.as_ref(), "--check or --apply")?,
        if_generation.as_deref(),
    )?
    .render_json()
}

fn execute_amend_role<I>(root: &std::path::Path, mut args: I) -> Result<String>
where
    I: Iterator<Item = OsString>,
{
    let mut report = None;
    let mut request = None;
    let mut if_generation = None;
    let mut mode = None;
    while let Some(argument) = args.next() {
        let argument = argument
            .into_string()
            .map_err(|_| AssuranceError::Usage("amend argument must be UTF-8".to_owned()))?;
        match argument.as_str() {
            "--report" if report.is_none() => report = Some(next_string(&mut args, "--report")?),
            "--request" if request.is_none() => {
                request = Some(PathBuf::from(next_string(&mut args, "--request")?));
            }
            "--if-generation" if if_generation.is_none() => {
                if_generation = Some(next_string(&mut args, "--if-generation")?);
            }
            "--check" if mode.is_none() => mode = Some(V2AmendMode::Check),
            "--apply" if mode.is_none() => mode = Some(V2AmendMode::Apply),
            _ => {
                return Err(AssuranceError::Usage(format!(
                    "unknown or duplicate role argument '{argument}'\n{USAGE}"
                )));
            }
        }
    }
    let request = required_option(request.as_ref(), "--request")?;
    let bytes = std::fs::read(request).map_err(|error| AssuranceError::io(request, error))?;
    amend_role_at_generation(
        root,
        required_option(report.as_ref(), "--report")?,
        &bytes,
        *required_option(mode.as_ref(), "--check or --apply")?,
        if_generation.as_deref(),
    )?
    .render_json()
}

fn execute_amend_normalize<I>(root: &std::path::Path, mut args: I) -> Result<String>
where
    I: Iterator<Item = OsString>,
{
    let mut report = None;
    let mut language = None;
    let mut if_generation = None;
    let mut mode = None;
    while let Some(argument) = args.next() {
        let argument = argument
            .into_string()
            .map_err(|_| AssuranceError::Usage("amend argument must be UTF-8".to_owned()))?;
        match argument.as_str() {
            "--report" if report.is_none() => {
                report = Some(next_string(&mut args, "--report")?);
            }
            "--language" if language.is_none() => {
                language = Some(next_string(&mut args, "--language")?);
            }
            "--if-generation" if if_generation.is_none() => {
                if_generation = Some(next_string(&mut args, "--if-generation")?);
            }
            "--check" if mode.is_none() => mode = Some(V2AmendMode::Check),
            "--apply" if mode.is_none() => mode = Some(V2AmendMode::Apply),
            _ => {
                return Err(AssuranceError::Usage(format!(
                    "unknown or duplicate normalize argument '{argument}'\n{USAGE}"
                )));
            }
        }
    }
    amend_normalize_at_generation(
        root,
        required_option(report.as_ref(), "--report")?,
        required_option(language.as_ref(), "--language")?,
        *required_option(mode.as_ref(), "--check or --apply")?,
        if_generation.as_deref(),
    )?
    .render_json()
}

fn execute_inspect<I>(root: &std::path::Path, mut args: I) -> Result<String>
where
    I: Iterator<Item = OsString>,
{
    let mut report = None;
    let mut format = OutputFormat::Human;
    while let Some(argument) = args.next() {
        let argument = argument
            .into_string()
            .map_err(|_| AssuranceError::Usage("inspect argument must be UTF-8".to_owned()))?;
        match argument.as_str() {
            "--report" if report.is_none() => report = Some(next_string(&mut args, "--report")?),
            "--format" => {
                let value = next_string(&mut args, "--format")?;
                format = match value.as_str() {
                    "human" => OutputFormat::Human,
                    "json" => OutputFormat::Json,
                    _ => {
                        return Err(AssuranceError::Usage(
                            "--format must be human or json".to_owned(),
                        ));
                    }
                };
            }
            _ => {
                return Err(AssuranceError::Usage(format!(
                    "unknown or duplicate inspect argument '{argument}'\n{USAGE}"
                )));
            }
        }
    }
    let inspection = inspect_report(root, required_option(report.as_ref(), "--report")?)?;
    match format {
        OutputFormat::Human => Ok(inspection.render_human()),
        OutputFormat::Json => inspection.render_json(),
    }
}

fn execute_lifecycle<I>(root: &std::path::Path, mut args: I) -> Result<String>
where
    I: Iterator<Item = OsString>,
{
    let mut report = None;
    let mut request = None;
    let mut if_generation = None;
    let mut mode = None;
    while let Some(argument) = args.next() {
        let argument = argument
            .into_string()
            .map_err(|_| AssuranceError::Usage("lifecycle argument must be UTF-8".to_owned()))?;
        match argument.as_str() {
            "--report" if report.is_none() => {
                report = Some(next_string(&mut args, "--report")?);
            }
            "--request" if request.is_none() => {
                request = Some(PathBuf::from(next_string(&mut args, "--request")?));
            }
            "--if-generation" if if_generation.is_none() => {
                if_generation = Some(next_string(&mut args, "--if-generation")?);
            }
            "--check" if mode.is_none() => mode = Some(V2AmendMode::Check),
            "--apply" if mode.is_none() => mode = Some(V2AmendMode::Apply),
            _ => {
                return Err(AssuranceError::Usage(format!(
                    "unknown or duplicate lifecycle argument '{argument}'\n{USAGE}"
                )));
            }
        }
    }
    let request = required_option(request.as_ref(), "--request")?;
    let bytes = std::fs::read(request).map_err(|error| AssuranceError::io(request, error))?;
    amend_lifecycle_at_generation(
        root,
        required_option(report.as_ref(), "--report")?,
        &bytes,
        *required_option(mode.as_ref(), "--check or --apply")?,
        if_generation.as_deref(),
    )?
    .render_json()
}

fn utf8(value: Option<OsString>, name: &str) -> Result<String> {
    value
        .ok_or_else(|| AssuranceError::Usage(USAGE.to_owned()))?
        .into_string()
        .map_err(|_| AssuranceError::Usage(format!("{name} must be UTF-8")))
}

fn ensure_no_arguments<I>(mut args: I) -> Result<()>
where
    I: Iterator<Item = OsString>,
{
    if args.next().is_some() {
        return Err(AssuranceError::Usage(format!(
            "help accepts no arguments\n{USAGE}"
        )));
    }
    Ok(())
}

fn execute(
    root: &std::path::Path,
    assurance: &Assurance,
    command: &str,
    options: &Options,
) -> Result<String> {
    match command {
        "validate" => execute_validate(root, assurance, options),
        "normalize" => execute_normalize(root, options),
        "plan" => execute_plan(root, assurance, options),
        "build" => execute_build(root, assurance, options),
        "check" => execute_check(root, assurance, options),
        "publish" => execute_publish(root, options, false),
        "publish-test-fixture" => execute_publish(root, options, true),
        "verify-release" => execute_verify_release(options),
        _ => Err(AssuranceError::Usage(format!(
            "unknown command '{command}'\n{USAGE}"
        ))),
    }
}

fn execute_validate(
    root: &std::path::Path,
    assurance: &Assurance,
    options: &Options,
) -> Result<String> {
    reject_normalization_options(options)?;
    reject_publication_options(options)?;
    reject_build_options(options)?;
    reject_staging_root(options)?;
    reject_plan_format(options)?;
    assurance.validate()?;
    let repository = V2Repository::open(root)?;
    match &options.selection {
        Selection::All => repository.validate_all().map(|summary| summary.render()),
        Selection::Report(report_id) => repository
            .validate_report(report_id)
            .map(|summary| summary.render()),
    }
}

fn execute_normalize(root: &std::path::Path, options: &Options) -> Result<String> {
    reject_publication_options(options)?;
    reject_build_options(options)?;
    reject_staging_root(options)?;
    reject_plan_format(options)?;
    let report_id = match &options.selection {
        Selection::Report(report_id) => report_id,
        Selection::All => {
            return Err(AssuranceError::Usage(
                "normalize requires --report; all-report mutation is intentionally unsupported"
                    .to_owned(),
            ));
        }
    };
    let language = required_option(options.language.as_ref(), "--language")?;
    let mode = required_option(options.normalization_mode.as_ref(), "--check or --apply")?;
    let amend_mode = match mode {
        V2NormalizationMode::Check => V2AmendMode::Check,
        V2NormalizationMode::Apply => V2AmendMode::Apply,
    };
    amend_normalize(root, report_id, language, amend_mode)?.render_json()
}

fn execute_plan(
    root: &std::path::Path,
    assurance: &Assurance,
    options: &Options,
) -> Result<String> {
    reject_normalization_options(options)?;
    reject_publication_options(options)?;
    reject_build_options(options)?;
    reject_staging_root(options)?;
    let public_plan = assurance.plan()?;
    let repository = V2Repository::open(root)?;
    let plan = match &options.selection {
        Selection::All => repository.plan_all()?,
        Selection::Report(report_id) => repository.plan_report(report_id)?,
    };
    if plan.publication_state != public_plan.publication_state {
        return Err(AssuranceError::Invalid(
            "v2 planner publication boundary disagrees with zero-report source".to_owned(),
        ));
    }
    match options.format.unwrap_or(OutputFormat::Human) {
        OutputFormat::Human => Ok(plan.render()),
        OutputFormat::Json => plan.render_json(),
    }
}

fn execute_build(
    root: &std::path::Path,
    assurance: &Assurance,
    options: &Options,
) -> Result<String> {
    reject_normalization_options(options)?;
    reject_publication_options(options)?;
    reject_plan_format(options)?;
    if let Some(staging_root) = &options.staging_root {
        reject_public_build_options(options)?;
        let repository = V2Repository::open(root)?;
        let result = match &options.selection {
            Selection::All => repository.build_all(staging_root)?,
            Selection::Report(report_id) => repository.build_report(report_id, staging_root)?,
        };
        return Ok(render_assembly_result("build", &result));
    }
    reject_report_selection(options, "report-specific assembly requires --staging-root")?;
    assurance
        .build(&options.build)
        .map(|result| render_result("build", &result))
}

fn execute_check(
    root: &std::path::Path,
    assurance: &Assurance,
    options: &Options,
) -> Result<String> {
    reject_normalization_options(options)?;
    reject_publication_options(options)?;
    reject_plan_format(options)?;
    if let Some(staging_root) = &options.staging_root {
        reject_public_build_options(options)?;
        let repository = V2Repository::open(root)?;
        let result = match &options.selection {
            Selection::All => repository.check_all(staging_root)?,
            Selection::Report(report_id) => repository.check_report(report_id, staging_root)?,
        };
        return Ok(render_assembly_result("check", &result));
    }
    reject_report_selection(
        options,
        "report-specific assembly checks require --staging-root",
    )?;
    reject_build_options(options)?;
    assurance
        .check()
        .map(|result| render_result("check", &result))
}

fn execute_publish(root: &std::path::Path, options: &Options, test_only: bool) -> Result<String> {
    reject_normalization_options(options)?;
    reject_plan_format(options)?;
    reject_public_build_options(options)?;
    let publication = publication_options(options)?;
    let repository = V2Repository::open(root)?;
    publish_selected(&repository, &options.selection, &publication, test_only)
        .map(|result| render_publication_result(&result))
}

fn publication_options(options: &Options) -> Result<V2PublicationOptions> {
    let staging_root = required_option(options.staging_root.as_ref(), "--staging-root")?;
    let usersum_root = required_option(options.usersum_root.as_ref(), "--usersum-root")?;
    let snapshot_root = required_option(
        options.publication_snapshot_root.as_ref(),
        "--publication-snapshot-root",
    )?;
    let release = release_identity(options)?;
    reject_verify_inputs(options)?;
    Ok(V2PublicationOptions::new(
        staging_root.clone(),
        usersum_root.clone(),
        snapshot_root.clone(),
        release,
    ))
}

fn publish_selected(
    repository: &V2Repository,
    selection: &Selection,
    publication: &V2PublicationOptions,
    test_only: bool,
) -> Result<V2PublicationResult> {
    match (selection, test_only) {
        (Selection::All, false) => repository.publish_all(publication),
        (Selection::Report(report), false) => repository.publish_report(report, publication),
        (Selection::All, true) => repository.publish_all_test_fixtures(publication),
        (Selection::Report(report), true) => {
            repository.publish_test_fixture_report(report, publication)
        }
    }
}

fn execute_verify_release(options: &Options) -> Result<String> {
    reject_normalization_options(options)?;
    reject_plan_format(options)?;
    reject_build_options(options)?;
    reject_staging_root(options)?;
    if options.usersum_root.is_some() || options.publication_snapshot_root.is_some() {
        return Err(AssuranceError::Usage(
            "--usersum-root and --publication-snapshot-root are publish-only".to_owned(),
        ));
    }
    let snapshot = required_option(options.snapshot_dir.as_ref(), "--snapshot-dir")?;
    let receipt = required_option(options.receipt.as_ref(), "--receipt")?;
    let release = release_identity(options)?;
    verify_v2_release_snapshot(snapshot, receipt, &release)
        .map(|result| render_release_verification(&result))
}

fn parse_options<I>(mut args: I) -> Result<Options>
where
    I: Iterator<Item = OsString>,
{
    let mut options = OptionAccumulator::default();
    while let Some(argument) = args.next() {
        let argument = argument
            .into_string()
            .map_err(|_| AssuranceError::Usage("arguments must be UTF-8".to_owned()))?;
        if parse_selection_argument(&argument, &mut args, &mut options.selection)? {
            continue;
        }
        if parse_build_argument(&argument, &mut args, &mut options)? {
            continue;
        }
        if parse_publication_argument(&argument, &mut args, &mut options)? {
            continue;
        }
        if parse_verification_argument(&argument, &mut args, &mut options)? {
            continue;
        }
        if parse_normalization_argument(&argument, &mut args, &mut options)? {
            continue;
        }
        parse_remaining_argument(&argument, &mut args, &mut options)?;
    }
    options.finish()
}

fn parse_normalization_argument<I>(
    argument: &str,
    args: &mut I,
    options: &mut OptionAccumulator,
) -> Result<bool>
where
    I: Iterator<Item = OsString>,
{
    match argument {
        "--language" if options.language.is_none() => {
            options.language = Some(next_string(args, "--language")?);
        }
        "--check" if options.normalization_mode.is_none() => {
            options.normalization_mode = Some(V2NormalizationMode::Check);
        }
        "--apply" if options.normalization_mode.is_none() => {
            options.normalization_mode = Some(V2NormalizationMode::Apply);
        }
        "--language" | "--check" | "--apply" => {
            return Err(AssuranceError::Usage(format!(
                "unknown or duplicate argument '{argument}'\n{USAGE}"
            )));
        }
        _ => return Ok(false),
    }
    Ok(true)
}

fn require_selection(selection: Option<Selection>) -> Result<Selection> {
    selection.ok_or_else(|| {
        AssuranceError::Usage(format!(
            "exactly one of --all or --report is required\n{USAGE}"
        ))
    })
}

fn parse_build_argument<I>(
    argument: &str,
    args: &mut I,
    options: &mut OptionAccumulator,
) -> Result<bool>
where
    I: Iterator<Item = OsString>,
{
    match argument {
        "--output-root" => {
            options.build.output_root = Some(next_path(args, "--output-root")?);
        }
        "--snapshot" => options.build.snapshot = Some(next_string(args, "--snapshot")?),
        "--snapshot-root" => {
            options.build.snapshot_root = Some(next_path(args, "--snapshot-root")?);
        }
        _ => return Ok(false),
    }
    Ok(true)
}

fn parse_publication_argument<I>(
    argument: &str,
    args: &mut I,
    options: &mut OptionAccumulator,
) -> Result<bool>
where
    I: Iterator<Item = OsString>,
{
    let target = match argument {
        "--staging-root" => &mut options.staging_root,
        "--usersum-root" => &mut options.usersum_root,
        "--publication-snapshot-root" => &mut options.publication_snapshot_root,
        _ => return parse_release_argument(argument, args, options),
    };
    if target.is_some() {
        return Ok(false);
    }
    *target = Some(next_path(args, argument)?);
    Ok(true)
}

fn parse_release_argument<I>(
    argument: &str,
    args: &mut I,
    options: &mut OptionAccumulator,
) -> Result<bool>
where
    I: Iterator<Item = OsString>,
{
    let target = match argument {
        "--release-commit" => &mut options.release_commit,
        "--release-configuration" => &mut options.release_configuration,
        _ => return Ok(false),
    };
    if target.is_some() {
        return Ok(false);
    }
    *target = Some(next_string(args, argument)?);
    Ok(true)
}

fn parse_verification_argument<I>(
    argument: &str,
    args: &mut I,
    options: &mut OptionAccumulator,
) -> Result<bool>
where
    I: Iterator<Item = OsString>,
{
    let target = match argument {
        "--snapshot-dir" => &mut options.snapshot_dir,
        "--receipt" => &mut options.receipt,
        _ => return Ok(false),
    };
    if target.is_some() {
        return Ok(false);
    }
    *target = Some(next_path(args, argument)?);
    Ok(true)
}

fn parse_remaining_argument<I>(
    argument: &str,
    args: &mut I,
    options: &mut OptionAccumulator,
) -> Result<()>
where
    I: Iterator<Item = OsString>,
{
    match argument {
        "--format" if options.format.is_none() => {
            options.format = Some(parse_format(&next_string(args, "--format")?)?);
            Ok(())
        }
        "--dossier" => Err(AssuranceError::Usage(
            "v1 dossier selection is retired; only --all with zero reports is allowed".to_owned(),
        )),
        _ => Err(AssuranceError::Usage(format!(
            "unknown or duplicate argument '{argument}'\n{USAGE}"
        ))),
    }
}

fn parse_selection_argument<I>(
    argument: &str,
    args: &mut I,
    selection: &mut Option<Selection>,
) -> Result<bool>
where
    I: Iterator<Item = OsString>,
{
    match (argument, selection.is_none()) {
        ("--all", true) => *selection = Some(Selection::All),
        ("--report", true) => {
            *selection = Some(Selection::Report(next_string(args, "--report")?));
        }
        ("--all" | "--report", false) => {
            return Err(AssuranceError::Usage(format!(
                "unknown or duplicate argument '{argument}'\n{USAGE}"
            )));
        }
        _ => return Ok(false),
    }
    Ok(true)
}

fn parse_format(value: &str) -> Result<OutputFormat> {
    match value {
        "human" => Ok(OutputFormat::Human),
        "json" => Ok(OutputFormat::Json),
        _ => Err(AssuranceError::Usage(
            "--format must be 'human' or 'json'".to_owned(),
        )),
    }
}

fn next_string<I>(args: &mut I, option: &str) -> Result<String>
where
    I: Iterator<Item = OsString>,
{
    utf8(args.next(), option)
}

fn next_path<I>(args: &mut I, option: &str) -> Result<PathBuf>
where
    I: Iterator<Item = OsString>,
{
    next_string(args, option).map(PathBuf::from)
}

fn reject_build_options(options: &Options) -> Result<()> {
    if options.build == BuildOptions::default() {
        return Ok(());
    }
    Err(AssuranceError::Usage(
        "--output-root, --snapshot, and --snapshot-root are build-only".to_owned(),
    ))
}

fn reject_public_build_options(options: &Options) -> Result<()> {
    if options.build == BuildOptions::default() {
        return Ok(());
    }
    Err(AssuranceError::Usage(
        "--staging-root cannot be combined with --output-root, --snapshot, or --snapshot-root"
            .to_owned(),
    ))
}

fn reject_staging_root(options: &Options) -> Result<()> {
    if options.staging_root.is_none() {
        return Ok(());
    }
    Err(AssuranceError::Usage(
        "--staging-root is build/check-only".to_owned(),
    ))
}

fn reject_plan_format(options: &Options) -> Result<()> {
    if options.format.is_none() {
        return Ok(());
    }
    Err(AssuranceError::Usage("--format is plan-only".to_owned()))
}

fn reject_report_selection(options: &Options, owner: &str) -> Result<()> {
    if matches!(options.selection, Selection::All) {
        return Ok(());
    }
    Err(AssuranceError::Usage(format!(
        "{owner}; this command does not support --report"
    )))
}

fn reject_publication_options(options: &Options) -> Result<()> {
    if options.usersum_root.is_none()
        && options.publication_snapshot_root.is_none()
        && options.release_commit.is_none()
        && options.release_configuration.is_none()
        && options.snapshot_dir.is_none()
        && options.receipt.is_none()
    {
        Ok(())
    } else {
        Err(AssuranceError::Usage(
            "publication and release-verification options are command-specific".to_owned(),
        ))
    }
}

fn reject_verify_inputs(options: &Options) -> Result<()> {
    if options.snapshot_dir.is_none() && options.receipt.is_none() {
        Ok(())
    } else {
        Err(AssuranceError::Usage(
            "--snapshot-dir and --receipt are verify-release-only".to_owned(),
        ))
    }
}

fn reject_normalization_options(options: &Options) -> Result<()> {
    if options.language.is_none() && options.normalization_mode.is_none() {
        Ok(())
    } else {
        Err(AssuranceError::Usage(
            "--language, --check, and --apply are normalize-only".to_owned(),
        ))
    }
}

fn required_option<'a, T>(value: Option<&'a T>, name: &str) -> Result<&'a T> {
    value.ok_or_else(|| AssuranceError::Usage(format!("{name} is required\n{USAGE}")))
}

fn release_identity(options: &Options) -> Result<V2ReleaseIdentity> {
    V2ReleaseIdentity::new(
        required_option(options.release_commit.as_ref(), "--release-commit")?.clone(),
        required_option(
            options.release_configuration.as_ref(),
            "--release-configuration",
        )?
        .clone(),
    )
}

fn render_result(label: &str, result: &crate::BuildResult) -> String {
    let mut output = format!("{label}: PASS\nreports: 0\noutputs:\n");
    for (path, digest) in &result.outputs {
        let _ = writeln!(output, "  - {} sha256={digest}", path.display());
    }
    if let (Some(path), Some(digest)) =
        (&result.snapshot_manifest, &result.snapshot_manifest_sha256)
    {
        let state = if result.snapshot_confirmed_existing {
            "confirmed"
        } else {
            "created"
        };
        let _ = writeln!(
            output,
            "snapshot: {state} {} sha256={digest}",
            path.display()
        );
    }
    output
}

fn render_assembly_result(label: &str, result: &V2AssemblyResult) -> String {
    let mut output = format!(
        "{label}: PASS\nreports: {}\noutputs:\n",
        result.reports.len()
    );
    for (path, digest) in &result.outputs {
        let _ = writeln!(output, "  - {} sha256={digest}", path.display());
    }
    output
}

fn render_publication_result(result: &V2PublicationResult) -> String {
    format!(
        "publication: PASS\nreports: {}\nsnapshot_id: {}\nsnapshot_path: {}\nreceipt_id: {}\nreceipt_path: {}\npublic_tree_sha256: {}\n",
        result.report_ids.len(),
        result.snapshot_id,
        result.snapshot_path.display(),
        result.receipt_id,
        result.receipt_path.display(),
        result.public_tree_sha256,
    )
}

fn render_release_verification(result: &V2ReleaseVerification) -> String {
    format!(
        "release verification: PASS\nreports: {}\nsnapshot_id: {}\nreceipt_id: {}\npublic_tree_sha256: {}\n",
        result.report_ids.len(),
        result.snapshot_id,
        result.receipt_id,
        result.public_tree_sha256,
    )
}

struct Options {
    selection: Selection,
    build: BuildOptions,
    format: Option<OutputFormat>,
    staging_root: Option<PathBuf>,
    usersum_root: Option<PathBuf>,
    publication_snapshot_root: Option<PathBuf>,
    release_commit: Option<String>,
    release_configuration: Option<String>,
    snapshot_dir: Option<PathBuf>,
    receipt: Option<PathBuf>,
    language: Option<String>,
    normalization_mode: Option<V2NormalizationMode>,
}

#[derive(Default)]
struct OptionAccumulator {
    selection: Option<Selection>,
    build: BuildOptions,
    format: Option<OutputFormat>,
    staging_root: Option<PathBuf>,
    usersum_root: Option<PathBuf>,
    publication_snapshot_root: Option<PathBuf>,
    release_commit: Option<String>,
    release_configuration: Option<String>,
    snapshot_dir: Option<PathBuf>,
    receipt: Option<PathBuf>,
    language: Option<String>,
    normalization_mode: Option<V2NormalizationMode>,
}

impl OptionAccumulator {
    fn finish(self) -> Result<Options> {
        Ok(Options {
            selection: require_selection(self.selection)?,
            build: self.build,
            format: self.format,
            staging_root: self.staging_root,
            usersum_root: self.usersum_root,
            publication_snapshot_root: self.publication_snapshot_root,
            release_commit: self.release_commit,
            release_configuration: self.release_configuration,
            snapshot_dir: self.snapshot_dir,
            receipt: self.receipt,
            language: self.language,
            normalization_mode: self.normalization_mode,
        })
    }
}

#[derive(Clone, Copy, PartialEq, Eq)]
enum OutputFormat {
    Human,
    Json,
}

enum Selection {
    All,
    Report(String),
}

#[cfg(test)]
mod tests {
    use std::ffi::OsString;
    use std::fs;
    use std::path::{Path, PathBuf};
    use std::sync::atomic::{AtomicU64, Ordering};

    use super::{
        V2NormalizationMode, execute_amend, execute_inspect, execute_lifecycle, execute_normalize,
        parse_options, publication_options,
    };

    fn repository_root() -> PathBuf {
        Path::new(env!("CARGO_MANIFEST_DIR")).join("../..")
    }

    fn amend(root: &Path, arguments: &[&str]) -> crate::Result<String> {
        execute_amend(root, arguments.iter().map(OsString::from))
    }

    fn request_file(label: &str, contents: &str) -> PathBuf {
        static SERIAL: AtomicU64 = AtomicU64::new(0);
        let serial = SERIAL.fetch_add(1, Ordering::Relaxed);
        let path = std::env::temp_dir().join(format!(
            "openwepp-assurance-cli-{label}-{}-{serial}.yaml",
            std::process::id()
        ));
        fs::write(&path, contents).expect("write CLI request fixture");
        path
    }

    #[test]
    fn exactly_one_selection_is_admitted() {
        assert!(parse_options(Vec::<std::ffi::OsString>::new().into_iter()).is_err());
        assert!(parse_options(["--dossier", "x"].map(Into::into).into_iter()).is_err());
        assert!(parse_options(["--all"].map(Into::into).into_iter()).is_ok());
        assert!(parse_options(["--report", "x"].map(Into::into).into_iter()).is_ok());
        assert!(parse_options(["--all", "--all"].map(Into::into).into_iter()).is_err());
        assert!(parse_options(["--all", "--report", "x"].map(Into::into).into_iter()).is_err());
    }

    #[test]
    fn publication_options_bind_all_explicit_cli_inputs() {
        let options = parse_options(
            [
                "--report",
                "report-id",
                "--staging-root",
                "/tmp/staging",
                "--usersum-root",
                "/tmp/usersum",
                "--publication-snapshot-root",
                "/tmp/snapshots",
                "--release-commit",
                "ec396c458a5015c504011a75814ff13e274544a1",
                "--release-configuration",
                "openwepp-release-default-v1",
            ]
            .map(Into::into)
            .into_iter(),
        )
        .expect("parse complete publication options");

        let publication = publication_options(&options).expect("bind publication options");
        assert_eq!(
            publication.release().commit(),
            "ec396c458a5015c504011a75814ff13e274544a1"
        );
        assert_eq!(
            publication.release().configuration(),
            "openwepp-release-default-v1"
        );
    }

    #[test]
    fn normalization_requires_one_explicit_mode() {
        let check = parse_options(
            ["--report", "report-id", "--language", "en-US", "--check"]
                .map(Into::into)
                .into_iter(),
        )
        .expect("parse normalization check");
        assert_eq!(check.normalization_mode, Some(V2NormalizationMode::Check));
        assert!(
            parse_options(
                [
                    "--report",
                    "report-id",
                    "--language",
                    "en-US",
                    "--check",
                    "--apply",
                ]
                .map(Into::into)
                .into_iter(),
            )
            .is_err()
        );

        let missing_mode = parse_options(
            ["--report", "report-id", "--language", "en-US"]
                .map(Into::into)
                .into_iter(),
        )
        .expect("parse incomplete normalization for execution rejection");
        assert!(execute_normalize(std::path::Path::new("/not-opened"), &missing_mode).is_err());

        let all = parse_options(
            ["--all", "--language", "en-US", "--check"]
                .map(Into::into)
                .into_iter(),
        )
        .expect("parse all-report selection for execution rejection");
        assert!(execute_normalize(std::path::Path::new("/not-opened"), &all).is_err());

        let foreign = parse_options(
            [
                "--report",
                "report-id",
                "--language",
                "en-US",
                "--check",
                "--staging-root",
                "/tmp/stage",
            ]
            .map(Into::into)
            .into_iter(),
        )
        .expect("parse foreign option for execution rejection");
        assert!(execute_normalize(std::path::Path::new("/not-opened"), &foreign).is_err());
    }

    #[test]
    fn amendment_recovery_command_reaches_typed_backend() {
        let root = repository_root();
        assert!(
            amend(&root, &["rebind-implementation", "--all", "--check"])
                .expect("calculate implementation rebind")
                .contains("\"changed\": false")
        );
        assert!(
            amend(&root, &["recover", "--inspect"])
                .unwrap()
                .contains("recovery")
        );
        assert!(amend(&root, &["unknown-operation"]).is_err());
    }

    #[test]
    fn amendment_data_commands_parse_complete_requests_before_backend_entry() {
        let missing = Path::new("/not-opened");
        assert!(
            amend(
                missing,
                &[
                    "attribution",
                    "--principal",
                    "roger-lew",
                    "--display-name",
                    "Roger Lew",
                    "--affiliation",
                    "University of Idaho",
                    "--if-generation",
                    "generation",
                    "--check",
                ],
            )
            .is_err()
        );
        let principal = request_file("principal", "schema_version: 1\n");
        let role = request_file("role", "schema_version: 1\n");
        assert!(
            amend(
                missing,
                &[
                    "principal",
                    "--request",
                    principal.to_str().unwrap(),
                    "--if-generation",
                    "generation",
                    "--apply",
                ],
            )
            .is_err()
        );
        assert!(
            amend(
                missing,
                &[
                    "role",
                    "--report",
                    "report",
                    "--request",
                    role.to_str().unwrap(),
                    "--if-generation",
                    "generation",
                    "--check",
                ],
            )
            .is_err()
        );
        assert!(
            amend(
                missing,
                &[
                    "normalize",
                    "--report",
                    "report",
                    "--language",
                    "en-US",
                    "--if-generation",
                    "generation",
                    "--check",
                ],
            )
            .is_err()
        );
        fs::remove_file(principal).unwrap();
        fs::remove_file(role).unwrap();
    }

    #[test]
    fn inspect_and_lifecycle_commands_cover_human_json_and_request_paths() {
        let root = repository_root();
        let report = "snow-and-frozen-soil-process-evaluation";
        assert!(
            execute_inspect(
                &root,
                ["--report", report, "--format", "human"]
                    .map(OsString::from)
                    .into_iter(),
            )
            .unwrap()
            .contains(report)
        );
        assert!(
            execute_inspect(
                &root,
                ["--report", report, "--format", "json"]
                    .map(OsString::from)
                    .into_iter(),
            )
            .unwrap()
            .contains(report)
        );
        assert!(
            execute_inspect(
                &root,
                ["--report", report, "--format", "xml"]
                    .map(OsString::from)
                    .into_iter(),
            )
            .is_err()
        );
        let request = request_file("lifecycle", "schema_version: 1\n");
        assert!(
            execute_lifecycle(
                Path::new("/not-opened"),
                [
                    "--report",
                    report,
                    "--request",
                    request.to_str().unwrap(),
                    "--if-generation",
                    "generation",
                    "--check",
                ]
                .map(OsString::from)
                .into_iter(),
            )
            .is_err()
        );
        fs::remove_file(request).unwrap();
    }
}
