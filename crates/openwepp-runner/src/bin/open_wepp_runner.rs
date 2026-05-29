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

#[allow(clippy::too_many_lines)]
fn run_hillslope_command(args: &[String]) -> Result<(), RunnerError> {
    let mut hillslope_binary: Option<PathBuf> = None;
    let mut run_dir: Option<PathBuf> = None;
    let mut run_file: Option<PathBuf> = None;
    let mut output_dir: Option<PathBuf> = None;
    let mut sidecar_policy = SidecarPolicy::Compat;
    let mut legacy_sidecar_discovery = false;
    let mut manifest_path: Option<PathBuf> = None;

    let mut cursor = 0usize;
    while cursor < args.len() {
        match args[cursor].as_str() {
            "--hillslope-binary" => {
                cursor += 1;
                let Some(value) = args.get(cursor) else {
                    return Err(RunnerError::MissingArgument {
                        argument: "--hillslope-binary <path>".to_string(),
                    });
                };
                hillslope_binary = Some(PathBuf::from(value));
            }
            "--run-dir" => {
                cursor += 1;
                let Some(value) = args.get(cursor) else {
                    return Err(RunnerError::MissingArgument {
                        argument: "--run-dir <path>".to_string(),
                    });
                };
                run_dir = Some(PathBuf::from(value));
            }
            "--run-file" => {
                cursor += 1;
                let Some(value) = args.get(cursor) else {
                    return Err(RunnerError::MissingArgument {
                        argument: "--run-file <path>".to_string(),
                    });
                };
                run_file = Some(PathBuf::from(value));
            }
            "--output-dir" => {
                cursor += 1;
                let Some(value) = args.get(cursor) else {
                    return Err(RunnerError::MissingArgument {
                        argument: "--output-dir <path>".to_string(),
                    });
                };
                output_dir = Some(PathBuf::from(value));
            }
            "--policy" => {
                cursor += 1;
                let Some(value) = args.get(cursor) else {
                    return Err(RunnerError::MissingArgument {
                        argument: "--policy <compat>".to_string(),
                    });
                };
                sidecar_policy = value.parse().map_err(|_| RunnerError::MissingArgument {
                    argument: "--policy <compat>".to_string(),
                })?;
            }
            "--manifest-path" => {
                cursor += 1;
                let Some(value) = args.get(cursor) else {
                    return Err(RunnerError::MissingArgument {
                        argument: "--manifest-path <path>".to_string(),
                    });
                };
                manifest_path = Some(PathBuf::from(value));
            }
            "--legacy-sidecar-discovery" => {
                legacy_sidecar_discovery = true;
            }
            flag => {
                return Err(RunnerError::MissingArgument {
                    argument: format!("unrecognized argument {flag}"),
                });
            }
        }

        cursor += 1;
    }

    let Some(hillslope_binary) = hillslope_binary else {
        return Err(RunnerError::MissingArgument {
            argument: "--hillslope-binary".to_string(),
        });
    };
    let Some(run_dir) = run_dir else {
        return Err(RunnerError::MissingArgument {
            argument: "--run-dir".to_string(),
        });
    };
    let Some(run_file) = run_file else {
        return Err(RunnerError::MissingArgument {
            argument: "--run-file".to_string(),
        });
    };
    let Some(output_dir) = output_dir else {
        return Err(RunnerError::MissingArgument {
            argument: "--output-dir".to_string(),
        });
    };

    launch_hillslope(&RunnerLaunchRequest {
        hillslope_binary,
        run_dir,
        run_file,
        output_dir,
        sidecar_policy,
        legacy_sidecar_discovery,
        manifest_path,
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
}
