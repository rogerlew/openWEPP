use std::fs;
use std::path::{Path, PathBuf};

use openwepp_runner::{Totalwatsed3Config, write_totalwatsed3};

fn main() {
    if let Err(error) = run() {
        eprintln!("{error}");
        std::process::exit(1);
    }
}

fn run() -> Result<(), String> {
    let args: Vec<String> = std::env::args().collect();
    let invocation = parse_invocation(&args)?;
    let Invocation::Run(options) = invocation else {
        print_help();
        return Ok(());
    };

    let options = validate_required_options(options)?;
    let config = collect_resolved_inputs(options)?;
    execute_totalwatsed3(&config)
}

#[derive(Default)]
struct ParsedOptions {
    input_dir: Option<PathBuf>,
    output_path: Option<PathBuf>,
    pass_path: Option<PathBuf>,
    wat_path: Option<PathBuf>,
    soil_path: Option<PathBuf>,
    element_path: Option<PathBuf>,
}

struct ValidatedOptions {
    input_dir: PathBuf,
    output_path: PathBuf,
    pass_path: Option<PathBuf>,
    wat_path: Option<PathBuf>,
    soil_path: Option<PathBuf>,
    element_path: Option<PathBuf>,
}

enum Invocation {
    Help,
    Run(ParsedOptions),
}

fn parse_invocation(args: &[String]) -> Result<Invocation, String> {
    let mut options = ParsedOptions::default();

    let mut cursor = 1_usize;
    while cursor < args.len() {
        match args[cursor].as_str() {
            "--input-dir" => {
                options.input_dir = Some(parse_path_option(args, &mut cursor, "--input-dir")?);
            }
            "--output" => {
                options.output_path = Some(parse_path_option(args, &mut cursor, "--output")?);
            }
            "--pass" => {
                options.pass_path = Some(parse_path_option(args, &mut cursor, "--pass")?);
            }
            "--wat" => {
                options.wat_path = Some(parse_path_option(args, &mut cursor, "--wat")?);
            }
            "--soil" => {
                options.soil_path = Some(parse_path_option(args, &mut cursor, "--soil")?);
            }
            "--element" => {
                options.element_path = Some(parse_path_option(args, &mut cursor, "--element")?);
            }
            "--help" | "-h" => return Ok(Invocation::Help),
            flag => return Err(format!("CLITW3-E-001 unrecognized argument {flag}")),
        }
        cursor += 1;
    }

    Ok(Invocation::Run(options))
}

fn parse_path_option(args: &[String], cursor: &mut usize, flag: &str) -> Result<PathBuf, String> {
    *cursor += 1;
    let Some(value) = args.get(*cursor) else {
        return Err(format!("CLITW3-E-001 missing value for {flag}"));
    };
    Ok(PathBuf::from(value))
}

fn validate_required_options(options: ParsedOptions) -> Result<ValidatedOptions, String> {
    let ParsedOptions {
        input_dir,
        output_path,
        pass_path,
        wat_path,
        soil_path,
        element_path,
    } = options;

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

    Ok(ValidatedOptions {
        input_dir,
        output_path,
        pass_path,
        wat_path,
        soil_path,
        element_path,
    })
}

fn collect_resolved_inputs(options: ValidatedOptions) -> Result<Totalwatsed3Config, String> {
    let ValidatedOptions {
        input_dir,
        output_path,
        pass_path,
        wat_path,
        soil_path,
        element_path,
    } = options;

    let pass_paths =
        resolve_required_input_paths(&input_dir, pass_path, "H.pass.parquet", ".pass.parquet");
    if pass_paths.is_empty() {
        return Err(format!(
            "CLITW3-E-004 required PASS parquet does not exist: {}",
            input_dir.join("H.pass.parquet").display()
        ));
    }
    let wat_paths =
        resolve_required_input_paths(&input_dir, wat_path, "H.wat.parquet", ".wat.parquet");
    if wat_paths.is_empty() {
        return Err(format!(
            "CLITW3-E-005 required WAT parquet does not exist: {}",
            input_dir.join("H.wat.parquet").display()
        ));
    }
    let soil_paths =
        resolve_optional_input_paths(&input_dir, soil_path, "H.soil.parquet", ".soil.parquet")?;
    let element_paths = resolve_optional_input_paths(
        &input_dir,
        element_path,
        "H.element.parquet",
        ".element.parquet",
    )?;

    Ok(Totalwatsed3Config {
        pass_paths,
        wat_paths,
        soil_paths,
        element_paths,
        output_path,
    })
}

fn execute_totalwatsed3(config: &Totalwatsed3Config) -> Result<(), String> {
    let summary = write_totalwatsed3(config)
        .map_err(|error| format!("CLITW3-E-010 totalwatsed3 production failed: {error}"))?;

    eprintln!(
        "CLITW3-I-001 wrote {} rows to {}",
        summary.rows_written,
        summary.output_path.display()
    );
    Ok(())
}

fn resolve_required_input_paths(
    input_dir: &Path,
    candidate: Option<PathBuf>,
    default_name: &str,
    per_hillslope_suffix: &str,
) -> Vec<PathBuf> {
    if let Some(candidate) = candidate {
        let path = resolve_input_path(input_dir, Some(candidate), default_name);
        return path.is_file().then_some(path).into_iter().collect();
    }

    let combined = input_dir.join(default_name);
    if combined.is_file() {
        return vec![combined];
    }
    collect_per_hillslope_inputs(input_dir, per_hillslope_suffix)
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

fn resolve_optional_input_paths(
    input_dir: &Path,
    candidate: Option<PathBuf>,
    default_name: &str,
    per_hillslope_suffix: &str,
) -> Result<Vec<PathBuf>, String> {
    let explicitly_configured = candidate.is_some();
    let path = resolve_input_path(input_dir, candidate, default_name);
    if path.is_file() {
        return Ok(vec![path]);
    }
    if explicitly_configured {
        return Err(format!(
            "CLITW3-E-006 optional input was explicitly configured but does not exist: {}",
            path.display()
        ));
    }
    Ok(collect_per_hillslope_inputs(
        input_dir,
        per_hillslope_suffix,
    ))
}

fn collect_per_hillslope_inputs(input_dir: &Path, suffix: &str) -> Vec<PathBuf> {
    let mut paths = fs::read_dir(input_dir)
        .ok()
        .into_iter()
        .flat_map(|entries| entries.filter_map(Result::ok))
        .map(|entry| entry.path())
        .filter(|path| {
            path.file_name()
                .and_then(|value| value.to_str())
                .is_some_and(|name| {
                    let Some(id_text) = name
                        .strip_suffix(suffix)
                        .and_then(|stem| stem.strip_prefix('H').or_else(|| stem.strip_prefix('h')))
                    else {
                        return false;
                    };
                    !id_text.is_empty() && id_text.bytes().all(|byte| byte.is_ascii_digit())
                })
        })
        .collect::<Vec<_>>();
    paths.sort_by_key(|path| file_name_sort_key(path));
    paths
}

fn file_name_sort_key(path: &Path) -> String {
    path.file_name()
        .and_then(|value| value.to_str())
        .unwrap_or_default()
        .to_string()
}

fn print_help() {
    println!(
        "openwepp-cli-totalwatsed3 --input-dir <interchange-dir> --output <totalwatsed3.parquet> [--pass H.pass.parquet] [--wat H.wat.parquet] [--soil H.soil.parquet] [--element H.element.parquet]"
    );
}
