use std::path::{Path, PathBuf};

use openwepp_runner::{Totalwatsed3Config, write_totalwatsed3};

fn main() {
    if let Err(error) = run() {
        eprintln!("{error}");
        std::process::exit(1);
    }
}

#[allow(clippy::too_many_lines)]
fn run() -> Result<(), String> {
    let args: Vec<String> = std::env::args().collect();
    let mut input_dir: Option<PathBuf> = None;
    let mut output_path: Option<PathBuf> = None;
    let mut pass_path: Option<PathBuf> = None;
    let mut wat_path: Option<PathBuf> = None;
    let mut soil_path: Option<PathBuf> = None;
    let mut element_path: Option<PathBuf> = None;

    let mut cursor = 1_usize;
    while cursor < args.len() {
        match args[cursor].as_str() {
            "--input-dir" => {
                cursor += 1;
                let Some(value) = args.get(cursor) else {
                    return Err("CLITW3-E-001 missing value for --input-dir".to_string());
                };
                input_dir = Some(PathBuf::from(value));
            }
            "--output" => {
                cursor += 1;
                let Some(value) = args.get(cursor) else {
                    return Err("CLITW3-E-001 missing value for --output".to_string());
                };
                output_path = Some(PathBuf::from(value));
            }
            "--pass" => {
                cursor += 1;
                let Some(value) = args.get(cursor) else {
                    return Err("CLITW3-E-001 missing value for --pass".to_string());
                };
                pass_path = Some(PathBuf::from(value));
            }
            "--wat" => {
                cursor += 1;
                let Some(value) = args.get(cursor) else {
                    return Err("CLITW3-E-001 missing value for --wat".to_string());
                };
                wat_path = Some(PathBuf::from(value));
            }
            "--soil" => {
                cursor += 1;
                let Some(value) = args.get(cursor) else {
                    return Err("CLITW3-E-001 missing value for --soil".to_string());
                };
                soil_path = Some(PathBuf::from(value));
            }
            "--element" => {
                cursor += 1;
                let Some(value) = args.get(cursor) else {
                    return Err("CLITW3-E-001 missing value for --element".to_string());
                };
                element_path = Some(PathBuf::from(value));
            }
            "--help" | "-h" => {
                print_help();
                return Ok(());
            }
            flag => return Err(format!("CLITW3-E-001 unrecognized argument {flag}")),
        }
        cursor += 1;
    }

    let Some(input_dir) = input_dir else {
        return Err("CLITW3-E-001 missing --input-dir".to_string());
    };
    let Some(output_path) = output_path else {
        return Err("CLITW3-E-001 missing --output".to_string());
    };
    if !input_dir.is_dir() {
        return Err(format!(
            "CLITW3-E-002 input directory does not exist: {}",
            input_dir.display()
        ));
    }

    let pass_path = resolve_input_path(&input_dir, pass_path, "H.pass.parquet");
    if !pass_path.is_file() {
        return Err(format!(
            "CLITW3-E-004 required PASS parquet does not exist: {}",
            pass_path.display()
        ));
    }
    let wat_path = resolve_input_path(&input_dir, wat_path, "H.wat.parquet");
    if !wat_path.is_file() {
        return Err(format!(
            "CLITW3-E-005 required WAT parquet does not exist: {}",
            wat_path.display()
        ));
    }
    let soil_path = resolve_optional_input_path(&input_dir, soil_path, "H.soil.parquet")?;
    let element_path = resolve_optional_input_path(&input_dir, element_path, "H.element.parquet")?;

    let summary = write_totalwatsed3(&Totalwatsed3Config {
        pass_path,
        wat_path,
        soil_path,
        element_path,
        output_path,
    })
    .map_err(|error| format!("CLITW3-E-010 totalwatsed3 production failed: {error}"))?;

    eprintln!(
        "CLITW3-I-001 wrote {} rows to {}",
        summary.rows_written,
        summary.output_path.display()
    );
    Ok(())
}

fn resolve_input_path(input_dir: &Path, candidate: Option<PathBuf>, default_name: &str) -> PathBuf {
    candidate.map_or_else(
        || input_dir.join(default_name),
        |path| {
            if path.is_absolute() {
                path
            } else {
                input_dir.join(path)
            }
        },
    )
}

fn resolve_optional_input_path(
    input_dir: &Path,
    candidate: Option<PathBuf>,
    default_name: &str,
) -> Result<Option<PathBuf>, String> {
    let explicitly_configured = candidate.is_some();
    let path = resolve_input_path(input_dir, candidate, default_name);
    if path.is_file() {
        return Ok(Some(path));
    }
    if explicitly_configured {
        return Err(format!(
            "CLITW3-E-006 optional input was explicitly configured but does not exist: {}",
            path.display()
        ));
    }
    Ok(None)
}

fn print_help() {
    println!(
        "openwepp-cli-totalwatsed3 --input-dir <interchange-dir> --output <totalwatsed3.parquet> [--pass H.pass.parquet] [--wat H.wat.parquet] [--soil H.soil.parquet] [--element H.element.parquet]"
    );
}
