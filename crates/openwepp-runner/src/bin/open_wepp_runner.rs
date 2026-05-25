use std::path::PathBuf;

use openwepp_runner::{
    RunnerError, RunnerLaunchRequest, SidecarPolicy, launch_hillslope, lint_release_directory,
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
            argument: "release <lint>".to_string(),
        });
    };

    match subcommand {
        "lint" => {
            let mut release_dir: Option<PathBuf> = None;
            let mut cursor = 1usize;
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

            lint_release_directory(&release_dir)
                .map_err(|source| RunnerError::ReleaseLint { source })?;
            Ok(())
        }
        _ => Err(RunnerError::MissingArgument {
            argument: "release lint".to_string(),
        }),
    }
}

fn print_help() {
    println!(
        "open_wepp_runner run-hillslope --hillslope-binary <path> --run-dir <path> --run-file <path> --output-dir <path> [--policy compat] [--legacy-sidecar-discovery] [--manifest-path <path>]"
    );
    println!("open_wepp_runner release lint --release-dir <path>");
}
