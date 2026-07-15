use std::env;
use std::ffi::OsString;
use std::fmt::Write as _;
use std::path::PathBuf;

use crate::{Assurance, AssuranceError, BuildOptions, Result};

const USAGE: &str = "Usage:\n  openwepp-assurance validate --all\n  openwepp-assurance plan --all\n  openwepp-assurance build --all [--output-root <path>] [--snapshot <id> --snapshot-root <path>]\n  openwepp-assurance check --all\n";

/// Parses process arguments and executes one zero-report operation.
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
    execute(&Assurance::open(root)?, &command, &options)
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

fn execute(assurance: &Assurance, command: &str, options: &Options) -> Result<String> {
    match command {
        "validate" => {
            reject_build_options(options)?;
            assurance.validate()?;
            Ok("validation: PASS\nreports: 0\n".to_owned())
        }
        "plan" => {
            reject_build_options(options)?;
            assurance.plan().map(|plan| plan.render())
        }
        "build" => assurance
            .build(&options.build)
            .map(|result| render_result("build", &result)),
        "check" => {
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
    let mut all = false;
    let mut build = BuildOptions::default();
    while let Some(argument) = args.next() {
        let argument = argument
            .into_string()
            .map_err(|_| AssuranceError::Usage("arguments must be UTF-8".to_owned()))?;
        match argument.as_str() {
            "--all" if !all => all = true,
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
    if !all {
        return Err(AssuranceError::Usage(format!(
            "--all is required; v1 dossier selection is retired\n{USAGE}"
        )));
    }
    Ok(Options { build })
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
    build: BuildOptions,
}

#[cfg(test)]
mod tests {
    use super::parse_options;

    #[test]
    fn only_all_selection_is_admitted() {
        assert!(parse_options(Vec::<std::ffi::OsString>::new().into_iter()).is_err());
        assert!(parse_options(["--dossier", "x"].map(Into::into).into_iter()).is_err());
        assert!(parse_options(["--all"].map(Into::into).into_iter()).is_ok());
        assert!(parse_options(["--all", "--all"].map(Into::into).into_iter()).is_err());
    }
}
