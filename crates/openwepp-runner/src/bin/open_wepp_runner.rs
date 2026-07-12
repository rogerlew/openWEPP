use std::path::PathBuf;

use openwepp_runner::{
    BinaryRole, RunnerError, RunnerLaunchRequest, SidecarPolicy, launch_hillslope,
    lint_release_directory, write_release_sidecar_for_binary,
};

fn main() {
    if let Err(error) = run() {
        eprintln!("{error}");
        std::process::exit(1);
    }
}

fn run() -> Result<(), RunnerError> {
    let args: Vec<String> = std::env::args().collect();
    let Some(command) = args.get(1).map(String::as_str) else {
        print_help();
        return Err(RunnerError::MissingArgument {
            argument: "<command>".to_string(),
        });
    };

    match command {
        "run-hillslope" => run_hillslope_command(&args[2..]),
        "release" => run_release_command(&args[2..]),
        "--help" | "-h" => {
            print_help();
            Ok(())
        }
        _ => {
            print_help();
            Err(RunnerError::MissingArgument {
                argument: "valid command".to_string(),
            })
        }
    }
}

fn run_hillslope_command(args: &[String]) -> Result<(), RunnerError> {
    launch_hillslope_options(parse_hillslope_args(args)?)
}

#[derive(Debug)]
struct HillslopeCommandOptions {
    hillslope_binary: PathBuf,
    run_dir: PathBuf,
    run_file: PathBuf,
    output_dir: PathBuf,
    sidecar_policy: SidecarPolicy,
    legacy_sidecar_discovery: bool,
    manifest_path: Option<PathBuf>,
}

struct HillslopeParseState {
    hillslope_binary: Option<PathBuf>,
    run_dir: Option<PathBuf>,
    run_file: Option<PathBuf>,
    output_dir: Option<PathBuf>,
    sidecar_policy: SidecarPolicy,
    legacy_sidecar_discovery: bool,
    manifest_path: Option<PathBuf>,
}

impl Default for HillslopeParseState {
    fn default() -> Self {
        Self {
            hillslope_binary: None,
            run_dir: None,
            run_file: None,
            output_dir: None,
            sidecar_policy: SidecarPolicy::Compat,
            legacy_sidecar_discovery: false,
            manifest_path: None,
        }
    }
}

fn parse_hillslope_args(args: &[String]) -> Result<HillslopeCommandOptions, RunnerError> {
    let mut state = HillslopeParseState::default();
    let mut cursor = 0usize;
    while cursor < args.len() {
        match args[cursor].as_str() {
            "--hillslope-binary" => {
                state.hillslope_binary = Some(PathBuf::from(next_hillslope_value(
                    args,
                    &mut cursor,
                    "--hillslope-binary <path>",
                )?));
            }
            "--run-dir" => {
                state.run_dir = Some(PathBuf::from(next_hillslope_value(
                    args,
                    &mut cursor,
                    "--run-dir <path>",
                )?));
            }
            "--run-file" => {
                state.run_file = Some(PathBuf::from(next_hillslope_value(
                    args,
                    &mut cursor,
                    "--run-file <path>",
                )?));
            }
            "--output-dir" => {
                state.output_dir = Some(PathBuf::from(next_hillslope_value(
                    args,
                    &mut cursor,
                    "--output-dir <path>",
                )?));
            }
            "--policy" => {
                state.sidecar_policy =
                    next_hillslope_value(args, &mut cursor, "--policy <compat>")?
                        .parse()
                        .map_err(|_| missing_hillslope_argument("--policy <compat>"))?;
            }
            "--manifest-path" => {
                state.manifest_path = Some(PathBuf::from(next_hillslope_value(
                    args,
                    &mut cursor,
                    "--manifest-path <path>",
                )?));
            }
            "--legacy-sidecar-discovery" => state.legacy_sidecar_discovery = true,
            flag => {
                return Err(missing_hillslope_argument(&format!(
                    "unrecognized argument {flag}"
                )));
            }
        }

        cursor += 1;
    }

    finalize_hillslope_options(state)
}

fn missing_hillslope_argument(argument: &str) -> RunnerError {
    RunnerError::MissingArgument {
        argument: argument.to_string(),
    }
}

fn next_hillslope_value<'a>(
    args: &'a [String],
    cursor: &mut usize,
    argument: &str,
) -> Result<&'a str, RunnerError> {
    *cursor += 1;
    args.get(*cursor)
        .map(String::as_str)
        .ok_or_else(|| missing_hillslope_argument(argument))
}

fn finalize_hillslope_options(
    state: HillslopeParseState,
) -> Result<HillslopeCommandOptions, RunnerError> {
    Ok(HillslopeCommandOptions {
        hillslope_binary: state
            .hillslope_binary
            .ok_or_else(|| missing_hillslope_argument("--hillslope-binary"))?,
        run_dir: state
            .run_dir
            .ok_or_else(|| missing_hillslope_argument("--run-dir"))?,
        run_file: state
            .run_file
            .ok_or_else(|| missing_hillslope_argument("--run-file"))?,
        output_dir: state
            .output_dir
            .ok_or_else(|| missing_hillslope_argument("--output-dir"))?,
        sidecar_policy: state.sidecar_policy,
        legacy_sidecar_discovery: state.legacy_sidecar_discovery,
        manifest_path: state.manifest_path,
    })
}

fn launch_hillslope_options(options: HillslopeCommandOptions) -> Result<(), RunnerError> {
    launch_hillslope(&RunnerLaunchRequest {
        hillslope_binary: options.hillslope_binary,
        run_dir: options.run_dir,
        run_file: options.run_file,
        output_dir: options.output_dir,
        sidecar_policy: options.sidecar_policy,
        legacy_sidecar_discovery: options.legacy_sidecar_discovery,
        manifest_path: options.manifest_path,
    })
}

fn run_release_command(args: &[String]) -> Result<(), RunnerError> {
    let Some(subcommand) = args.first().map(String::as_str) else {
        return Err(RunnerError::MissingArgument {
            argument: "release <lint|sidecar>".to_string(),
        });
    };

    match subcommand {
        "lint" => run_release_lint_command(&args[1..]),
        "sidecar" => run_release_sidecar_command(&args[1..]),
        _ => Err(RunnerError::MissingArgument {
            argument: "release <lint|sidecar>".to_string(),
        }),
    }
}

fn run_release_lint_command(args: &[String]) -> Result<(), RunnerError> {
    let mut release_dir: Option<PathBuf> = None;
    let mut cursor = 0usize;
    while cursor < args.len() {
        match args[cursor].as_str() {
            "--release-dir" => {
                cursor += 1;
                let Some(value) = args.get(cursor) else {
                    return Err(RunnerError::MissingArgument {
                        argument: "--release-dir <path>".to_string(),
                    });
                };
                release_dir = Some(PathBuf::from(value));
            }
            flag => {
                return Err(RunnerError::MissingArgument {
                    argument: format!("unrecognized release lint argument {flag}"),
                });
            }
        }
        cursor += 1;
    }

    let Some(release_dir) = release_dir else {
        return Err(RunnerError::MissingArgument {
            argument: "--release-dir".to_string(),
        });
    };

    lint_release_directory(&release_dir).map_err(|source| RunnerError::ReleaseLint { source })?;
    Ok(())
}

fn run_release_sidecar_command(args: &[String]) -> Result<(), RunnerError> {
    let mut binary_path: Option<PathBuf> = None;
    let mut binary_role: Option<BinaryRole> = None;
    let mut cursor = 0usize;

    while cursor < args.len() {
        match args[cursor].as_str() {
            "--binary" => {
                cursor += 1;
                let Some(value) = args.get(cursor) else {
                    return Err(RunnerError::MissingArgument {
                        argument: "--binary <path>".to_string(),
                    });
                };
                binary_path = Some(PathBuf::from(value));
            }
            "--role" => {
                cursor += 1;
                let Some(value) = args.get(cursor) else {
                    return Err(RunnerError::MissingArgument {
                        argument: "--role <watershed|hillslope|replay>".to_string(),
                    });
                };
                binary_role = parse_binary_role(value);
                if binary_role.is_none() {
                    return Err(RunnerError::MissingArgument {
                        argument: "--role <watershed|hillslope|replay>".to_string(),
                    });
                }
            }
            flag => {
                return Err(RunnerError::MissingArgument {
                    argument: format!("unrecognized release sidecar argument {flag}"),
                });
            }
        }

        cursor += 1;
    }

    let Some(binary_path) = binary_path else {
        return Err(RunnerError::MissingArgument {
            argument: "--binary".to_string(),
        });
    };
    let Some(binary_role) = binary_role else {
        return Err(RunnerError::MissingArgument {
            argument: "--role".to_string(),
        });
    };

    write_release_sidecar_for_binary(&binary_path, binary_role)
        .map_err(|source| RunnerError::ReleaseMetadata { source })?;
    Ok(())
}

fn parse_binary_role(value: &str) -> Option<BinaryRole> {
    match value {
        "watershed" => Some(BinaryRole::Watershed),
        "hillslope" => Some(BinaryRole::Hillslope),
        "replay" => Some(BinaryRole::Replay),
        _ => None,
    }
}

fn print_help() {
    println!(
        "open_wepp_runner run-hillslope --hillslope-binary <path> --run-dir <path> --run-file <path> --output-dir <path> [--policy compat] [--legacy-sidecar-discovery] [--manifest-path <path>]"
    );
    println!("open_wepp_runner release lint --release-dir <path>");
    println!(
        "open_wepp_runner release sidecar --binary <path> --role <watershed|hillslope|replay>"
    );
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::Value;
    use std::fs;
    use std::time::{SystemTime, UNIX_EPOCH};

    fn args(values: &[&str]) -> Vec<String> {
        values.iter().map(|value| (*value).to_string()).collect()
    }

    fn unique_temp_dir(prefix: &str) -> PathBuf {
        let token = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("system time should be after unix epoch")
            .as_nanos();
        let dir = std::env::temp_dir().join(format!("{prefix}_{token}"));
        fs::create_dir_all(&dir).expect("temp directory should be creatable");
        dir
    }

    #[test]
    fn release_sidecar_command_writes_sidecar_for_requested_role() {
        let dir = unique_temp_dir("runner_release_sidecar_command");
        let binary_path = dir.join("openwepp_260529_hill");
        fs::write(&binary_path, b"fixture-binary").expect("binary fixture write should succeed");

        let args = vec![
            "sidecar".to_string(),
            "--binary".to_string(),
            binary_path.display().to_string(),
            "--role".to_string(),
            "hillslope".to_string(),
        ];
        run_release_command(&args).expect("release sidecar command should succeed");

        let sidecar_path = PathBuf::from(format!("{}.json", binary_path.display()));
        assert!(sidecar_path.is_file(), "expected sidecar to be emitted");

        let payload = fs::read_to_string(&sidecar_path).expect("sidecar should be readable");
        let json: Value = serde_json::from_str(&payload).expect("sidecar should be valid json");
        assert_eq!(
            json.get("binary_role").and_then(Value::as_str),
            Some("hillslope")
        );

        fs::remove_dir_all(dir).expect("temp directory cleanup should succeed");
    }

    #[test]
    fn release_sidecar_command_rejects_unsupported_role_value() {
        let dir = unique_temp_dir("runner_release_sidecar_role_reject");
        let binary_path = dir.join("openwepp_260529_hill");
        fs::write(&binary_path, b"fixture-binary").expect("binary fixture write should succeed");

        let args = vec![
            "sidecar".to_string(),
            "--binary".to_string(),
            binary_path.display().to_string(),
            "--role".to_string(),
            "invalid-role".to_string(),
        ];
        let error = run_release_command(&args).expect_err("unsupported role should fail");
        assert_eq!(error.code(), "RUNNER-E-001");
        assert_eq!(
            error.to_string(),
            "RUNNER-E-001 missing required argument --role <watershed|hillslope|replay>"
        );

        fs::remove_dir_all(dir).expect("temp directory cleanup should succeed");
    }

    #[test]
    fn hillslope_parse_preserves_complete_request() {
        let options = parse_hillslope_args(&args(&[
            "--hillslope-binary",
            "/tmp/hill",
            "--run-dir",
            "/tmp/run",
            "--run-file",
            "case.run",
            "--output-dir",
            "/tmp/out",
            "--policy",
            "compat",
            "--legacy-sidecar-discovery",
            "--manifest-path",
            "/tmp/out/manifest.json",
        ]))
        .expect("complete hillslope request");
        assert_eq!(options.hillslope_binary, PathBuf::from("/tmp/hill"));
        assert_eq!(options.run_dir, PathBuf::from("/tmp/run"));
        assert_eq!(options.run_file, PathBuf::from("case.run"));
        assert_eq!(options.output_dir, PathBuf::from("/tmp/out"));
        assert_eq!(options.sidecar_policy, SidecarPolicy::Compat);
        assert!(options.legacy_sidecar_discovery);
        assert_eq!(
            options.manifest_path,
            Some(PathBuf::from("/tmp/out/manifest.json"))
        );
    }

    #[test]
    fn hillslope_parse_preserves_value_and_required_guard_priority() {
        for (flag, descriptor) in [
            ("--hillslope-binary", "--hillslope-binary <path>"),
            ("--run-dir", "--run-dir <path>"),
            ("--run-file", "--run-file <path>"),
            ("--output-dir", "--output-dir <path>"),
            ("--policy", "--policy <compat>"),
            ("--manifest-path", "--manifest-path <path>"),
        ] {
            let error = parse_hillslope_args(&args(&[flag])).expect_err("missing flag value");
            assert_eq!(
                error.to_string(),
                format!("RUNNER-E-001 missing required argument {descriptor}")
            );
        }
        assert_eq!(
            parse_hillslope_args(&args(&["--policy", "invalid"]))
                .expect_err("invalid policy")
                .to_string(),
            "RUNNER-E-001 missing required argument --policy <compat>"
        );
        assert_eq!(
            parse_hillslope_args(&args(&["--unknown"]))
                .expect_err("unknown flag")
                .to_string(),
            "RUNNER-E-001 missing required argument unrecognized argument --unknown"
        );
        for (argv, missing) in [
            (Vec::new(), "--hillslope-binary"),
            (args(&["--hillslope-binary", "/tmp/hill"]), "--run-dir"),
            (
                args(&["--hillslope-binary", "/tmp/hill", "--run-dir", "/tmp/run"]),
                "--run-file",
            ),
            (
                args(&[
                    "--hillslope-binary",
                    "/tmp/hill",
                    "--run-dir",
                    "/tmp/run",
                    "--run-file",
                    "case.run",
                ]),
                "--output-dir",
            ),
        ] {
            assert_eq!(
                parse_hillslope_args(&argv)
                    .expect_err("required option")
                    .to_string(),
                format!("RUNNER-E-001 missing required argument {missing}")
            );
        }
    }

    #[test]
    fn hillslope_command_propagates_parse_failure() {
        assert_eq!(
            run_hillslope_command(&[])
                .expect_err("command boundary must propagate parse failure")
                .to_string(),
            "RUNNER-E-001 missing required argument --hillslope-binary"
        );
    }

    #[cfg(unix)]
    #[test]
    fn hillslope_command_launches_real_consumer_with_exact_argv() {
        use std::os::unix::fs::PermissionsExt;

        let dir = unique_temp_dir("runner_hillslope_launch_consumer");
        let capture = dir.join("argv.txt");
        let binary = dir.join("capture-hill.sh");
        fs::write(
            &binary,
            format!("#!/bin/sh\nprintf '%s\\n' \"$@\" > {}\n", capture.display()),
        )
        .expect("capture binary");
        let mut permissions = fs::metadata(&binary)
            .expect("capture metadata")
            .permissions();
        permissions.set_mode(0o755);
        fs::set_permissions(&binary, permissions).expect("capture executable permissions");

        run_hillslope_command(&args(&[
            "--hillslope-binary",
            binary.to_str().expect("UTF-8 binary"),
            "--run-dir",
            "/tmp/run",
            "--run-file",
            "case.run",
            "--output-dir",
            "/tmp/out",
            "--policy",
            "compat",
            "--legacy-sidecar-discovery",
            "--manifest-path",
            "/tmp/out/manifest.json",
        ]))
        .expect("launch consumer");
        let captured = fs::read_to_string(capture).expect("captured argv");
        assert_eq!(
            captured.lines().collect::<Vec<_>>(),
            vec![
                "--run-dir",
                "/tmp/run",
                "--run-file",
                "case.run",
                "--output-dir",
                "/tmp/out",
                "--policy",
                "compat",
                "--legacy-sidecar-discovery",
                "--manifest-path",
                "/tmp/out/manifest.json",
            ]
        );
        fs::remove_dir_all(dir).expect("temp directory cleanup");
    }
}
