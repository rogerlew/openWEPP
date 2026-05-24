use std::path::PathBuf;

use openwepp_runner::{HillslopeRunRequest, SidecarPolicy, execute_hillslope_run};

fn main() {
    if let Err(error) = run() {
        eprintln!("{error}");
        std::process::exit(1);
    }
}

fn run() -> Result<(), String> {
    let mut run_dir: Option<PathBuf> = None;
    let mut run_file: Option<PathBuf> = None;
    let mut output_dir: Option<PathBuf> = None;
    let mut policy = SidecarPolicy::Strict;
    let mut manifest_path: Option<PathBuf> = None;

    let args: Vec<String> = std::env::args().collect();
    let mut cursor = 1usize;
    while cursor < args.len() {
        match args[cursor].as_str() {
            "--run-dir" => {
                cursor += 1;
                let Some(value) = args.get(cursor) else {
                    return Err("CLIHILL-E-001 missing value for --run-dir".to_string());
                };
                run_dir = Some(PathBuf::from(value));
            }
            "--run-file" => {
                cursor += 1;
                let Some(value) = args.get(cursor) else {
                    return Err("CLIHILL-E-001 missing value for --run-file".to_string());
                };
                run_file = Some(PathBuf::from(value));
            }
            "--output-dir" => {
                cursor += 1;
                let Some(value) = args.get(cursor) else {
                    return Err("CLIHILL-E-001 missing value for --output-dir".to_string());
                };
                output_dir = Some(PathBuf::from(value));
            }
            "--policy" => {
                cursor += 1;
                let Some(value) = args.get(cursor) else {
                    return Err("CLIHILL-E-001 missing value for --policy".to_string());
                };
                policy = value.parse().map_err(|detail: String| {
                    format!("CLIHILL-E-001 invalid --policy value: {detail}")
                })?;
            }
            "--manifest-path" => {
                cursor += 1;
                let Some(value) = args.get(cursor) else {
                    return Err("CLIHILL-E-001 missing value for --manifest-path".to_string());
                };
                manifest_path = Some(PathBuf::from(value));
            }
            "--help" | "-h" => {
                print_help();
                return Ok(());
            }
            flag => {
                return Err(format!("CLIHILL-E-001 unrecognized argument {flag}"));
            }
        }

        cursor += 1;
    }

    let Some(run_dir) = run_dir else {
        return Err("CLIHILL-E-001 missing --run-dir".to_string());
    };
    let Some(run_file) = run_file else {
        return Err("CLIHILL-E-001 missing --run-file".to_string());
    };
    let Some(output_dir) = output_dir else {
        return Err("CLIHILL-E-001 missing --output-dir".to_string());
    };

    let report = execute_hillslope_run(
        &HillslopeRunRequest {
            run_dir,
            run_file,
            output_dir,
            sidecar_policy: policy,
            manifest_path,
        },
        &args,
    )
    .map_err(|error| error.to_string())?;

    for warning in report.sidecar_warnings {
        eprintln!("sidecar-warning: {warning}");
    }

    Ok(())
}

fn print_help() {
    println!(
        "openwepp-cli-hill --run-dir <path> --run-file <path> --output-dir <path> [--policy strict|compat] [--manifest-path <path>]"
    );
}
