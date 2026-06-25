use std::path::PathBuf;

use openwepp_runner::{SnowbenchExportRequest, export_pysnobal_inputs};

fn main() {
    if let Err(error) = run() {
        eprintln!("{error}");
        std::process::exit(1);
    }
}

fn run() -> Result<(), String> {
    let mut args = std::env::args().skip(1);
    let Some(command) = args.next() else {
        print_help();
        return Err("SNOWBENCH-E-CLI missing command".to_string());
    };
    if command == "--help" || command == "-h" {
        print_help();
        return Ok(());
    }
    if command != "export-pysnobal" {
        return Err(format!("SNOWBENCH-E-CLI unrecognized command {command}"));
    }

    let mut run_dir: Option<PathBuf> = None;
    let mut run_file: Option<PathBuf> = None;
    let mut output_dir: Option<PathBuf> = None;
    while let Some(flag) = args.next() {
        match flag.as_str() {
            "--run-dir" => {
                run_dir = Some(next_path(&mut args, "--run-dir")?);
            }
            "--run-file" => {
                run_file = Some(next_path(&mut args, "--run-file")?);
            }
            "--output-dir" => {
                output_dir = Some(next_path(&mut args, "--output-dir")?);
            }
            "--help" | "-h" => {
                print_help();
                return Ok(());
            }
            _ => return Err(format!("SNOWBENCH-E-CLI unrecognized argument {flag}")),
        }
    }

    let run_dir = run_dir.ok_or_else(|| "SNOWBENCH-E-CLI missing --run-dir".to_string())?;
    let output_dir =
        output_dir.ok_or_else(|| "SNOWBENCH-E-CLI missing --output-dir".to_string())?;

    let report = export_pysnobal_inputs(&SnowbenchExportRequest {
        run_dir,
        run_file,
        output_dir,
        include_openwepp_snow_projection: true,
    })
    .map_err(|error| error.to_string())?;
    println!(
        "exported {} hourly rows across {} lane(s) to {}",
        report.hourly_row_count, report.lane_count, report.output_dir
    );
    Ok(())
}

fn next_path(
    args: &mut impl Iterator<Item = String>,
    flag: &'static str,
) -> Result<PathBuf, String> {
    args.next()
        .map(PathBuf::from)
        .ok_or_else(|| format!("SNOWBENCH-E-CLI missing value for {flag}"))
}

fn print_help() {
    println!(
        "openwepp-snowbench export-pysnobal --run-dir <path> [--run-file <path>] --output-dir <path>"
    );
}
