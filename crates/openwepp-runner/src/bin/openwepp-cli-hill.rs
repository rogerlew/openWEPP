use std::path::PathBuf;

use openwepp_runner::{
    HillslopeDefaultRuntimeActivation, HillslopeRunRequest, HillslopeRuntimeSelection,
    HillslopeRuntimeSelectionPolicy, SidecarPolicy, execute_hillslope_run_with_runtime_policy,
};

fn main() {
    std::process::exit(run_process(std::env::args()));
}

fn run_process(args: impl IntoIterator<Item = String>) -> i32 {
    let args = args.into_iter().collect::<Vec<_>>();
    match run_with_args(&args) {
        Ok(()) => 0,
        Err(error) => {
            eprintln!("{error}");
            1
        }
    }
}

fn run_with_args(args: &[String]) -> Result<(), String> {
    let action = parse_cli_args(args)?;
    let CliAction::Execute(options) = action else {
        print_help();
        return Ok(());
    };
    execute_cli(options, args)
}

#[derive(Debug)]
struct CliOptions {
    run_dir: PathBuf,
    run_file: PathBuf,
    output_dir: PathBuf,
    policy: SidecarPolicy,
    legacy_sidecar_discovery: bool,
    manifest_path: Option<PathBuf>,
    runtime_selection: HillslopeRuntimeSelection,
    default_activation: HillslopeDefaultRuntimeActivation,
}

#[derive(Debug)]
enum CliAction {
    Help,
    Execute(CliOptions),
}

struct CliParseState {
    run_dir: Option<PathBuf>,
    run_file: Option<PathBuf>,
    output_dir: Option<PathBuf>,
    policy: SidecarPolicy,
    legacy_sidecar_discovery: bool,
    manifest_path: Option<PathBuf>,
    runtime_selection: HillslopeRuntimeSelection,
    runtime_selection_flag: Option<&'static str>,
    default_activation: HillslopeDefaultRuntimeActivation,
    default_activation_flag: Option<&'static str>,
}

impl Default for CliParseState {
    fn default() -> Self {
        Self {
            run_dir: None,
            run_file: None,
            output_dir: None,
            policy: SidecarPolicy::Compat,
            legacy_sidecar_discovery: false,
            manifest_path: None,
            runtime_selection: HillslopeRuntimeSelection::DefaultCandidate,
            runtime_selection_flag: None,
            default_activation: HillslopeDefaultRuntimeActivation::default(),
            default_activation_flag: None,
        }
    }
}

fn parse_cli_args(args: &[String]) -> Result<CliAction, String> {
    let mut state = CliParseState::default();
    let mut cursor = 1usize;
    while cursor < args.len() {
        match args[cursor].as_str() {
            "--run-dir" => {
                state.run_dir = Some(PathBuf::from(next_cli_value(
                    args,
                    &mut cursor,
                    "--run-dir",
                )?));
            }
            "--run-file" => {
                state.run_file = Some(PathBuf::from(next_cli_value(
                    args,
                    &mut cursor,
                    "--run-file",
                )?));
            }
            "--output-dir" => {
                state.output_dir = Some(PathBuf::from(next_cli_value(
                    args,
                    &mut cursor,
                    "--output-dir",
                )?));
            }
            "--policy" => {
                let value = next_cli_value(args, &mut cursor, "--policy")?;
                state.policy = value.parse().map_err(|detail: String| {
                    format!("CLIHILL-E-001 invalid --policy value: {detail}")
                })?;
            }
            "--manifest-path" => {
                state.manifest_path = Some(PathBuf::from(next_cli_value(
                    args,
                    &mut cursor,
                    "--manifest-path",
                )?));
            }
            "--legacy-sidecar-discovery" => state.legacy_sidecar_discovery = true,
            "--direct-production-executor" => {
                set_runtime_selection(
                    &mut state.runtime_selection,
                    &mut state.runtime_selection_flag,
                    HillslopeRuntimeSelection::DirectProductionExecutor,
                    "--direct-production-executor",
                )?;
            }
            "--direct-default-candidate" => {
                set_default_activation(&mut state)?;
            }
            "--help" | "-h" => return Ok(CliAction::Help),
            flag => {
                return Err(format!("CLIHILL-E-001 unrecognized argument {flag}"));
            }
        }

        cursor += 1;
    }

    finalize_cli_options(state).map(CliAction::Execute)
}

fn next_cli_value<'a>(
    args: &'a [String],
    cursor: &mut usize,
    flag: &str,
) -> Result<&'a str, String> {
    *cursor += 1;
    args.get(*cursor)
        .map(String::as_str)
        .ok_or_else(|| format!("CLIHILL-E-001 missing value for {flag}"))
}

fn set_default_activation(state: &mut CliParseState) -> Result<(), String> {
    if state.default_activation_flag.is_some() {
        return Err(
            "CLIHILL-E-001 duplicate default activation flag --direct-default-candidate"
                .to_string(),
        );
    }
    state.default_activation_flag = Some("--direct-default-candidate");
    state.default_activation = HillslopeDefaultRuntimeActivation::DirectProductionCandidate;
    Ok(())
}

fn finalize_cli_options(state: CliParseState) -> Result<CliOptions, String> {
    let Some(run_dir) = state.run_dir else {
        return Err("CLIHILL-E-001 missing --run-dir".to_string());
    };
    let Some(run_file) = state.run_file else {
        return Err("CLIHILL-E-001 missing --run-file".to_string());
    };
    let Some(output_dir) = state.output_dir else {
        return Err("CLIHILL-E-001 missing --output-dir".to_string());
    };
    if let (Some(default_flag), Some(_)) =
        (state.default_activation_flag, state.runtime_selection_flag)
    {
        return Err(format!(
            "CLIHILL-E-001 {default_flag} cannot be combined with an explicit runtime flag"
        ));
    }
    Ok(CliOptions {
        run_dir,
        run_file,
        output_dir,
        policy: state.policy,
        legacy_sidecar_discovery: state.legacy_sidecar_discovery,
        manifest_path: state.manifest_path,
        runtime_selection: state.runtime_selection,
        default_activation: state.default_activation,
    })
}

fn execute_cli(options: CliOptions, args: &[String]) -> Result<(), String> {
    let report = execute_hillslope_run_with_runtime_policy(
        &HillslopeRunRequest {
            run_dir: options.run_dir,
            run_file: options.run_file,
            output_dir: options.output_dir,
            sidecar_policy: options.policy,
            legacy_sidecar_discovery: options.legacy_sidecar_discovery,
            manifest_path: options.manifest_path,
        },
        args,
        HillslopeRuntimeSelectionPolicy::new(options.runtime_selection, options.default_activation),
    )
    .map_err(|error| error.to_string())?;

    for warning in report.sidecar_warnings {
        eprintln!("sidecar-warning: {warning}");
    }

    Ok(())
}

fn set_runtime_selection(
    runtime_selection: &mut HillslopeRuntimeSelection,
    runtime_selection_flag: &mut Option<&'static str>,
    selection: HillslopeRuntimeSelection,
    flag: &'static str,
) -> Result<(), String> {
    if let Some(previous_flag) = *runtime_selection_flag {
        return Err(format!(
            "CLIHILL-E-001 runtime flag {flag} conflicts with {previous_flag}"
        ));
    }
    *runtime_selection = selection;
    *runtime_selection_flag = Some(flag);
    Ok(())
}

fn print_help() {
    println!(
        "openwepp-cli-hill --run-dir <path> --run-file <path> --output-dir <path> [--policy compat] [--legacy-sidecar-discovery] [--manifest-path <path>] [--direct-default-candidate] [--direct-production-executor]"
    );
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::Path;

    fn args(values: &[&str]) -> Vec<String> {
        values.iter().map(|value| (*value).to_string()).collect()
    }

    fn copy_fixture(source: &Path, destination: &Path) {
        std::fs::create_dir_all(destination).expect("fixture destination");
        for entry in std::fs::read_dir(source).expect("fixture source") {
            let entry = entry.expect("fixture entry");
            let target = destination.join(entry.file_name());
            if entry.path().is_dir() {
                copy_fixture(&entry.path(), &target);
            } else {
                std::fs::copy(entry.path(), target).expect("fixture file copy");
            }
        }
    }

    #[test]
    fn parse_complete_request_preserves_paths_policy_and_runtime_selection() {
        let parsed = parse_cli_args(&args(&[
            "openwepp-cli-hill",
            "--run-dir",
            "run",
            "--run-file",
            "case.run",
            "--output-dir",
            "out",
            "--policy",
            "compat",
            "--legacy-sidecar-discovery",
            "--manifest-path",
            "manifest.json",
            "--direct-production-executor",
        ]))
        .expect("valid CLI");
        let CliAction::Execute(options) = parsed else {
            panic!("complete request must execute");
        };
        assert_eq!(options.run_dir, PathBuf::from("run"));
        assert_eq!(options.run_file, PathBuf::from("case.run"));
        assert_eq!(options.output_dir, PathBuf::from("out"));
        assert_eq!(options.policy, SidecarPolicy::Compat);
        assert!(options.legacy_sidecar_discovery);
        assert_eq!(options.manifest_path, Some(PathBuf::from("manifest.json")));
        assert_eq!(
            options.runtime_selection,
            HillslopeRuntimeSelection::DirectProductionExecutor
        );
    }

    #[test]
    fn parse_help_short_circuits_and_value_flags_keep_exact_errors() {
        assert!(matches!(
            parse_cli_args(&args(&["openwepp-cli-hill", "--help", "--unknown"])),
            Ok(CliAction::Help)
        ));
        for flag in [
            "--run-dir",
            "--run-file",
            "--output-dir",
            "--policy",
            "--manifest-path",
        ] {
            assert_eq!(
                parse_cli_args(&args(&["openwepp-cli-hill", flag])).expect_err("missing value"),
                format!("CLIHILL-E-001 missing value for {flag}")
            );
        }
        assert!(
            parse_cli_args(&args(&["openwepp-cli-hill", "--policy", "invalid"]))
                .expect_err("invalid policy")
                .starts_with("CLIHILL-E-001 invalid --policy value:")
        );
    }

    #[test]
    fn parse_required_and_selector_guards_preserve_priority_and_messages() {
        for (argv, expected) in [
            (vec!["openwepp-cli-hill"], "CLIHILL-E-001 missing --run-dir"),
            (
                vec!["openwepp-cli-hill", "--run-dir", "run"],
                "CLIHILL-E-001 missing --run-file",
            ),
            (
                vec![
                    "openwepp-cli-hill",
                    "--run-dir",
                    "run",
                    "--run-file",
                    "case.run",
                ],
                "CLIHILL-E-001 missing --output-dir",
            ),
        ] {
            assert_eq!(
                parse_cli_args(&args(&argv)).expect_err("required guard"),
                expected
            );
        }
        let base = [
            "openwepp-cli-hill",
            "--run-dir",
            "run",
            "--run-file",
            "case.run",
            "--output-dir",
            "out",
        ];
        let mut duplicate_default = base.to_vec();
        duplicate_default.extend(["--direct-default-candidate", "--direct-default-candidate"]);
        assert_eq!(
            parse_cli_args(&args(&duplicate_default)).expect_err("duplicate default"),
            "CLIHILL-E-001 duplicate default activation flag --direct-default-candidate"
        );
        let mut duplicate_runtime = base.to_vec();
        duplicate_runtime.extend([
            "--direct-production-executor",
            "--direct-production-executor",
        ]);
        assert_eq!(
            parse_cli_args(&args(&duplicate_runtime)).expect_err("runtime conflict"),
            "CLIHILL-E-001 runtime flag --direct-production-executor conflicts with --direct-production-executor"
        );
        let mut mixed = base.to_vec();
        mixed.extend(["--direct-default-candidate", "--direct-production-executor"]);
        assert_eq!(
            parse_cli_args(&args(&mixed)).expect_err("mixed selector classes"),
            "CLIHILL-E-001 --direct-default-candidate cannot be combined with an explicit runtime flag"
        );
        assert_eq!(
            parse_cli_args(&args(&["openwepp-cli-hill", "--unknown"]))
                .expect_err("unknown argument"),
            "CLIHILL-E-001 unrecognized argument --unknown"
        );
    }

    #[test]
    fn run_with_args_preserves_help_short_circuit_and_exact_error() {
        run_with_args(&args(&["openwepp-cli-hill", "--help", "--unknown"]))
            .expect("help short-circuits later arguments");
        assert_eq!(
            run_with_args(&args(&["openwepp-cli-hill", "--unknown"]))
                .expect_err("unknown argument"),
            "CLIHILL-E-001 unrecognized argument --unknown"
        );
        assert_eq!(
            run_process(args(&["openwepp-cli-hill", "--help", "--unknown"])),
            0
        );
        assert_eq!(run_process(args(&["openwepp-cli-hill", "--unknown"])), 1);
    }

    #[test]
    fn execute_cli_runs_real_minimal_fixture_and_publishes_outputs() {
        let source = Path::new(env!("CARGO_MANIFEST_DIR"))
            .join("../../tests/fixtures/cli01/hillslope_run_dir");
        let run_dir =
            std::env::temp_dir().join(format!("openwepp-ha09-execute-cli-{}", std::process::id()));
        let _ = std::fs::remove_dir_all(&run_dir);
        copy_fixture(&source, &run_dir);
        let output_dir = run_dir.join("output");
        let manifest_path = output_dir.join("manifest.json");
        let argv = args(&[
            "openwepp-cli-hill",
            "--run-dir",
            run_dir.to_str().expect("UTF-8 run dir"),
            "--run-file",
            "case.run",
            "--output-dir",
            output_dir.to_str().expect("UTF-8 output dir"),
            "--manifest-path",
            manifest_path.to_str().expect("UTF-8 manifest path"),
            "--direct-production-executor",
        ]);
        assert_eq!(run_process(argv), 0, "real CLI execution");
        assert!(run_dir.join("output/H5.hbp").is_file());
        assert!(run_dir.join("output/H5.loss.json").is_file());
        assert!(manifest_path.is_file());
        let manifest = std::fs::read_to_string(manifest_path).expect("manifest text");
        assert!(manifest.contains("R7C-DIRECT-PRODUCTION-EXECUTOR"));
        let _ = std::fs::remove_dir_all(run_dir);
    }
}
