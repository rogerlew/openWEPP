use std::env;
use std::ffi::OsString;
use std::fmt::Write as _;
use std::path::PathBuf;

use crate::{Assurance, AssuranceError, BuildOptions, Result, V2Repository};

const USAGE: &str = "Usage:\n  openwepp-assurance validate (--all | --report <id>)\n  openwepp-assurance plan --all\n  openwepp-assurance build --all [--output-root <path>] [--snapshot <id> --snapshot-root <path>]\n  openwepp-assurance check --all\n";

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
        "validate" => {
            reject_build_options(options)?;
            assurance.validate()?;
            let repository = V2Repository::open(root)?;
            match &options.selection {
                Selection::All => repository.validate_all().map(|summary| summary.render()),
                Selection::Report(report_id) => repository
                    .validate_report(report_id)
                    .map(|summary| summary.render()),
            }
        }
        "plan" => {
            reject_report_selection(options, "ASSURE-04B owns report-specific planning")?;
            reject_build_options(options)?;
            assurance.plan().map(|plan| plan.render())
        }
        "build" => {
            reject_report_selection(options, "ASSURE-04C owns report-specific assembly")?;
            assurance
                .build(&options.build)
                .map(|result| render_result("build", &result))
        }
        "check" => {
            reject_report_selection(options, "ASSURE-04C owns report-specific assembly checks")?;
            reject_build_options(options)?;
            assurance
                .check()
                .map(|result| render_result("check", &result))
        }
        _ => Err(AssuranceError::Usage(format!(
            "unknown command '{command}'\n{USAGE}"
        ))),
    }
}

fn parse_options<I>(mut args: I) -> Result<Options>
where
    I: Iterator<Item = OsString>,
{
    let mut selection = None;
    let mut build = BuildOptions::default();
    while let Some(argument) = args.next() {
        let argument = argument
            .into_string()
            .map_err(|_| AssuranceError::Usage("arguments must be UTF-8".to_owned()))?;
        match argument.as_str() {
            "--all" if selection.is_none() => selection = Some(Selection::All),
            "--report" if selection.is_none() => {
                selection = Some(Selection::Report(next_string(&mut args, "--report")?));
            }
            "--output-root" => build.output_root = Some(next_path(&mut args, "--output-root")?),
            "--snapshot" => build.snapshot = Some(next_string(&mut args, "--snapshot")?),
            "--snapshot-root" => {
                build.snapshot_root = Some(next_path(&mut args, "--snapshot-root")?);
            }
            "--dossier" => {
                return Err(AssuranceError::Usage(
                    "v1 dossier selection is retired; only --all with zero reports is allowed"
                        .to_owned(),
                ));
            }
            _ => {
                return Err(AssuranceError::Usage(format!(
                    "unknown or duplicate argument '{argument}'\n{USAGE}"
                )));
            }
        }
    }
    let Some(selection) = selection else {
        return Err(AssuranceError::Usage(format!(
            "exactly one of --all or --report is required\n{USAGE}"
        )));
    };
    Ok(Options { selection, build })
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

fn reject_report_selection(options: &Options, owner: &str) -> Result<()> {
    if matches!(options.selection, Selection::All) {
        return Ok(());
    }
    Err(AssuranceError::Usage(format!(
        "{owner}; ASSURE-04A supports --report only for validate"
    )))
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

struct Options {
    selection: Selection,
    build: BuildOptions,
}

enum Selection {
    All,
    Report(String),
}

#[cfg(test)]
mod tests {
    use super::parse_options;

    #[test]
    fn exactly_one_selection_is_admitted() {
        assert!(parse_options(Vec::<std::ffi::OsString>::new().into_iter()).is_err());
        assert!(parse_options(["--dossier", "x"].map(Into::into).into_iter()).is_err());
        assert!(parse_options(["--all"].map(Into::into).into_iter()).is_ok());
        assert!(parse_options(["--report", "x"].map(Into::into).into_iter()).is_ok());
        assert!(parse_options(["--all", "--all"].map(Into::into).into_iter()).is_err());
        assert!(parse_options(["--all", "--report", "x"].map(Into::into).into_iter()).is_err());
    }
}
