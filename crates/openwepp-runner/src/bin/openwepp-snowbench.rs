use std::path::PathBuf;

use openwepp_runner::{
    CoeBoundDensityRequest, CoeMeltModel, CoeMeltRequest, JenningsPhaseValidationRequest,
    PhysicsBulkRequest, PhysicsBulkVariant, SnowbenchExportRequest, export_pysnobal_inputs,
    run_coe_bound_density_snowbench, run_coe_melt_snowbench, run_jennings_phase_validation,
    run_physics_bulk_snowbench,
};

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
    if command == "jennings-phase" {
        return run_jennings_phase_args(args);
    }
    let mut run_dir: Option<PathBuf> = None;
    let mut run_file: Option<PathBuf> = None;
    let mut output_dir: Option<PathBuf> = None;
    let mut variant = PhysicsBulkVariant::default();
    let mut coe_model = CoeMeltModel::default();
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
            "--variant" => {
                let value = args
                    .next()
                    .ok_or_else(|| "SNOWBENCH-E-CLI missing value for --variant".to_string())?;
                variant = PhysicsBulkVariant::parse(&value).map_err(|error| error.to_string())?;
            }
            "--model" => {
                let value = args
                    .next()
                    .ok_or_else(|| "SNOWBENCH-E-CLI missing value for --model".to_string())?;
                coe_model = CoeMeltModel::parse(&value).map_err(|error| error.to_string())?;
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

    match command.as_str() {
        "export-pysnobal" => {
            run_export_pysnobal(run_dir, run_file, output_dir, variant, coe_model)?;
        }
        "physics-bulk" => {
            run_physics_bulk(run_dir, run_file, output_dir, variant, coe_model)?;
        }
        "coe-melt" => {
            run_coe_melt(run_dir, run_file, output_dir, variant, coe_model)?;
        }
        "coe-bound-density" => {
            run_coe_bound_density(run_dir, run_file, output_dir, variant, coe_model)?;
        }
        _ => return Err(format!("SNOWBENCH-E-CLI unrecognized command {command}")),
    }
    Ok(())
}

fn run_export_pysnobal(
    run_dir: PathBuf,
    run_file: Option<PathBuf>,
    output_dir: PathBuf,
    variant: PhysicsBulkVariant,
    coe_model: CoeMeltModel,
) -> Result<(), String> {
    if variant != PhysicsBulkVariant::CandidateV1 {
        return Err("SNOWBENCH-E-CLI --variant is only valid for physics-bulk".to_string());
    }
    if coe_model != CoeMeltModel::LegacyCoe {
        return Err("SNOWBENCH-E-CLI --model is only valid for coe-melt".to_string());
    }
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

fn run_physics_bulk(
    run_dir: PathBuf,
    run_file: Option<PathBuf>,
    output_dir: PathBuf,
    variant: PhysicsBulkVariant,
    coe_model: CoeMeltModel,
) -> Result<(), String> {
    if coe_model != CoeMeltModel::LegacyCoe {
        return Err("SNOWBENCH-E-CLI --model is only valid for coe-melt".to_string());
    }
    let report = run_physics_bulk_snowbench(&PhysicsBulkRequest {
        run_dir,
        run_file,
        output_dir,
        variant,
    })
    .map_err(|error| error.to_string())?;
    println!(
        "ran {} ({}) for {} hourly rows across {} day(s) to {}",
        report.model_id,
        report.variant,
        report.hourly_row_count,
        report.day_count,
        report.output_dir
    );
    Ok(())
}

fn run_coe_melt(
    run_dir: PathBuf,
    run_file: Option<PathBuf>,
    output_dir: PathBuf,
    variant: PhysicsBulkVariant,
    coe_model: CoeMeltModel,
) -> Result<(), String> {
    if variant != PhysicsBulkVariant::CandidateV1 {
        return Err("SNOWBENCH-E-CLI --variant is only valid for physics-bulk".to_string());
    }
    let report = run_coe_melt_snowbench(&CoeMeltRequest {
        run_dir,
        run_file,
        output_dir,
        model: coe_model,
    })
    .map_err(|error| error.to_string())?;
    println!(
        "ran diagnostic {} CoE melt replay for {} hourly rows across {} day(s) to {}",
        report.model_id, report.hourly_row_count, report.day_count, report.output_dir
    );
    Ok(())
}

fn run_coe_bound_density(
    run_dir: PathBuf,
    run_file: Option<PathBuf>,
    output_dir: PathBuf,
    variant: PhysicsBulkVariant,
    coe_model: CoeMeltModel,
) -> Result<(), String> {
    let report = run_coe_bound_density_snowbench(&CoeBoundDensityRequest {
        run_dir,
        run_file,
        output_dir,
        coe_model,
        density_variant: variant,
    })
    .map_err(|error| error.to_string())?;
    println!(
        "ran {} for {} hourly rows across {} day(s) to {}",
        report.model_id, report.hourly_row_count, report.day_count, report.output_dir
    );
    Ok(())
}

fn run_jennings_phase_args(mut args: impl Iterator<Item = String>) -> Result<(), String> {
    let mut observations_path: Option<PathBuf> = None;
    let mut thresholds_path: Option<PathBuf> = None;
    let mut output_dir: Option<PathBuf> = None;
    let mut max_rows: Option<usize> = None;
    while let Some(flag) = args.next() {
        match flag.as_str() {
            "--observations" => {
                observations_path = Some(next_path(&mut args, "--observations")?);
            }
            "--thresholds" => {
                thresholds_path = Some(next_path(&mut args, "--thresholds")?);
            }
            "--output-dir" => {
                output_dir = Some(next_path(&mut args, "--output-dir")?);
            }
            "--max-rows" => {
                let value = args
                    .next()
                    .ok_or_else(|| "SNOWBENCH-E-CLI missing value for --max-rows".to_string())?;
                max_rows = Some(value.parse::<usize>().map_err(|_| {
                    format!(
                        "SNOWBENCH-E-CLI --max-rows must be a positive integer, observed {value}"
                    )
                })?);
            }
            "--help" | "-h" => {
                print_help();
                return Ok(());
            }
            _ => return Err(format!("SNOWBENCH-E-CLI unrecognized argument {flag}")),
        }
    }
    let observations_path =
        observations_path.ok_or_else(|| "SNOWBENCH-E-CLI missing --observations".to_string())?;
    let thresholds_path =
        thresholds_path.ok_or_else(|| "SNOWBENCH-E-CLI missing --thresholds".to_string())?;
    let output_dir =
        output_dir.ok_or_else(|| "SNOWBENCH-E-CLI missing --output-dir".to_string())?;
    let report = run_jennings_phase_validation(&JenningsPhaseValidationRequest {
        observations_path,
        thresholds_path,
        output_dir,
        max_rows,
    })
    .map_err(|error| error.to_string())?;
    println!(
        "scored {} Jennings rows across {} station(s); Harder-Pomeroy accuracy {:.6}; legacy RST accuracy {:.6}; report {}",
        report.rows_scored,
        report.stations_scored,
        report.harder_pomeroy_hourly.accuracy,
        report.legacy_rst_0c.accuracy,
        report.report_json_path
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
        "openwepp-snowbench <export-pysnobal|physics-bulk|coe-melt|coe-bound-density> --run-dir <path> [--run-file <path>] --output-dir <path> [--variant <candidate_v1|slow_melt_v1|dense_slow_melt_v1|cold_dense_slow_melt_v1|density_compaction_v1|spring_densification_v1>] [--model <legacy_coe|coe_shortwave_albedo_v1|coe_winter_thaw_state_loss_v1>]\nopenwepp-snowbench jennings-phase --observations <file2.csv> --thresholds <file3.csv> --output-dir <path> [--max-rows <n>]"
    );
}
