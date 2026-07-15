use std::env;
use std::ffi::OsString;
use std::fmt::Write as _;
use std::path::PathBuf;

use crate::engine::{Assurance, BuildOptions, Selection};
use crate::error::{AssuranceError, Result};

const USAGE: &str = "Usage:\n  openwepp-assurance validate (--dossier <stable-id> | --all)\n  openwepp-assurance plan (--dossier <stable-id> | --all)\n  openwepp-assurance build (--dossier <stable-id> | --all) [--output-root <path>] [--snapshot <id> --snapshot-root <path>]\n  openwepp-assurance check (--dossier <stable-id> | --all)\n";

/// Parses process arguments and executes the selected assurance operation.
///
/// # Errors
///
/// Returns typed usage, validation, drift, review, and filesystem errors.
pub fn run_from_env() -> Result<String> {
    run(env::args_os())
}

/// Executes the CLI using a supplied argument iterator.
///
/// # Errors
///
/// Returns typed usage, validation, drift, review, and filesystem errors.
pub fn run<I, S>(args: I) -> Result<String>
where
    I: IntoIterator<Item = S>,
    S: Into<OsString>,
{
    let mut args = args.into_iter().map(Into::into);
    let _program = args.next();
    let command = args
        .next()
        .ok_or_else(|| AssuranceError::Usage(USAGE.to_owned()))?
        .into_string()
        .map_err(|_| AssuranceError::Usage("command must be UTF-8".to_owned()))?;
    let parsed = parse_options(args)?;
    let root = env::current_dir().map_err(|error| AssuranceError::io(".", error))?;
    let assurance = Assurance::open(root)?;
    execute(&assurance, &command, &parsed)
}

fn execute(assurance: &Assurance, command: &str, parsed: &ParsedOptions) -> Result<String> {
    match command {
        "validate" => execute_validate(assurance, parsed),
        "plan" => execute_plan(assurance, parsed),
        _ => execute_build_or_remaining(assurance, command, parsed),
    }
}

fn execute_validate(assurance: &Assurance, parsed: &ParsedOptions) -> Result<String> {
    reject_build_options(parsed)?;
    assurance.validate(&parsed.selection)?;
    Ok("validation: PASS\n".to_owned())
}

fn execute_plan(assurance: &Assurance, parsed: &ParsedOptions) -> Result<String> {
    reject_build_options(parsed)?;
    assurance.plan(&parsed.selection).map(|plan| plan.render())
}

fn execute_build(assurance: &Assurance, parsed: &ParsedOptions) -> Result<String> {
    let result = assurance.build(&parsed.selection, &parsed.build)?;
    Ok(render_build_result("build", &result))
}

fn execute_build_or_remaining(
    assurance: &Assurance,
    command: &str,
    parsed: &ParsedOptions,
) -> Result<String> {
    match command {
        "build" => execute_build(assurance, parsed),
        _ => execute_check_help_or_unknown(assurance, command, parsed),
    }
}

fn execute_check_help_or_unknown(
    assurance: &Assurance,
    command: &str,
    parsed: &ParsedOptions,
) -> Result<String> {
    match command {
        "check" => execute_check(assurance, parsed),
        "-h" | "--help" | "help" => Ok(USAGE.to_owned()),
        _ => Err(AssuranceError::Usage(format!(
            "unknown command '{command}'\n{USAGE}"
        ))),
    }
}

fn execute_check(assurance: &Assurance, parsed: &ParsedOptions) -> Result<String> {
    reject_build_options(parsed)?;
    let result = assurance.check(&parsed.selection)?;
    Ok(render_build_result("check", &result))
}

fn parse_options<I>(mut args: I) -> Result<ParsedOptions>
where
    I: Iterator<Item = OsString>,
{
    let mut dossier = None;
    let mut all = false;
    let mut build = BuildOptions::default();
    while let Some(argument) = args.next() {
        let argument = argument
            .into_string()
            .map_err(|_| AssuranceError::Usage("arguments must be UTF-8".to_owned()))?;
        match argument.as_str() {
            "--all" => all = true,
            "--dossier" => dossier = Some(next_string(&mut args, "--dossier")?),
            "--output-root" => {
                build.output_root = Some(PathBuf::from(next_string(&mut args, "--output-root")?));
            }
            "--snapshot" => build.snapshot = Some(next_string(&mut args, "--snapshot")?),
            "--snapshot-root" => {
                build.snapshot_root =
                    Some(PathBuf::from(next_string(&mut args, "--snapshot-root")?));
            }
            _ => {
                return Err(AssuranceError::Usage(format!(
                    "unknown argument '{argument}'\n{USAGE}"
                )));
            }
        }
    }
    let selection = match (all, dossier) {
        (true, None) => Selection::All,
        (false, Some(id)) => Selection::Dossier(id),
        _ => {
            return Err(AssuranceError::Usage(format!(
                "exactly one of --all or --dossier is required\n{USAGE}"
            )));
        }
    };
    Ok(ParsedOptions { selection, build })
}

fn next_string<I>(args: &mut I, option: &str) -> Result<String>
where
    I: Iterator<Item = OsString>,
{
    args.next()
        .ok_or_else(|| AssuranceError::Usage(format!("{option} requires a value")))?
        .into_string()
        .map_err(|_| AssuranceError::Usage(format!("{option} value must be UTF-8")))
}

fn reject_build_options(parsed: &ParsedOptions) -> Result<()> {
    if parsed.build.output_root.is_none()
        && parsed.build.snapshot.is_none()
        && parsed.build.snapshot_root.is_none()
    {
        Ok(())
    } else {
        Err(AssuranceError::Usage(
            "--output-root, --snapshot, and --snapshot-root are build-only".to_owned(),
        ))
    }
}

fn render_build_result(label: &str, result: &crate::engine::BuildResult) -> String {
    let mut output = format!("{label}: PASS\noutputs:\n");
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

struct ParsedOptions {
    selection: Selection,
    build: BuildOptions,
}

#[cfg(test)]
mod tests {
    use super::parse_options;
    use crate::engine::Selection;

    #[test]
    fn selector_is_exactly_one() {
        assert!(parse_options(Vec::<std::ffi::OsString>::new().into_iter()).is_err());
        assert!(parse_options(["--all", "--dossier", "x"].map(Into::into).into_iter()).is_err());
        let parsed = parse_options(["--dossier", "x"].map(Into::into).into_iter())
            .expect("valid dossier selector");
        assert!(matches!(parsed.selection, Selection::Dossier(ref id) if id == "x"));
    }
}
