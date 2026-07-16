use std::env;
use std::ffi::OsString;
use std::fmt::Write as _;
use std::path::PathBuf;

use crate::{
    Assurance, AssuranceError, BuildOptions, Result, V2AssemblyResult, V2PublicationOptions,
    V2PublicationResult, V2ReleaseIdentity, V2ReleaseVerification, V2Repository,
    verify_v2_release_snapshot,
};

const USAGE: &str = "Usage:\n  openwepp-assurance validate (--all | --report <id>)\n  openwepp-assurance plan (--all | --report <id>) [--format human|json]\n  openwepp-assurance build --all [--output-root <path>] [--snapshot <id> --snapshot-root <path>]\n  openwepp-assurance check --all\n  openwepp-assurance build (--all | --report <id>) --staging-root <path>\n  openwepp-assurance check (--all | --report <id>) --staging-root <path>\n  openwepp-assurance publish (--all | --report <id>) --staging-root <path> --usersum-root <path> --publication-snapshot-root <path> --release-commit <sha> --release-configuration <id>\n  openwepp-assurance publish-test-fixture (--all | --report <id>) --staging-root <path> --usersum-root <path> --publication-snapshot-root <path> --release-commit <sha> --release-configuration <id>\n  openwepp-assurance verify-release --all --snapshot-dir <path> --receipt <path> --release-commit <sha> --release-configuration <id>\n";

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
    let options = parse_options(args)?;
    let root = env::current_dir().map_err(|error| AssuranceError::io(".", error))?;
    let assurance = Assurance::open(&root)?;
    execute(&root, &assurance, &command, &options)
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

fn execute_plan(
    root: &std::path::Path,
    assurance: &Assurance,
    options: &Options,
) -> Result<String> {
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
        parse_remaining_argument(&argument, &mut args, &mut options)?;
    }
    options.finish()
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
    use super::{parse_options, publication_options};

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
}
