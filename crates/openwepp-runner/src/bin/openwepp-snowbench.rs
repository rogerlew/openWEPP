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
    run_with_args(std::env::args().skip(1))
}

fn run_with_args(mut args: impl Iterator<Item = String>) -> Result<(), String> {
    let Some(command) = args.next() else {
        print_help();
        return Err("SNOWBENCH-E-CLI missing command".to_string());
    };
    match classify_top_level_command(command) {
        TopLevelCommand::Help => {
            print_help();
            Ok(())
        }
        TopLevelCommand::JenningsPhase => run_jennings_phase_args(args),
        TopLevelCommand::Snowbench(command) => run_snowbench_command(&command, args),
    }
}

fn classify_top_level_command(command: String) -> TopLevelCommand {
    match command.as_str() {
        "--help" | "-h" => TopLevelCommand::Help,
        "jennings-phase" => TopLevelCommand::JenningsPhase,
        _ => TopLevelCommand::Snowbench(command),
    }
}

enum TopLevelCommand {
    Help,
    JenningsPhase,
    Snowbench(String),
}

fn run_snowbench_command(command: &str, args: impl Iterator<Item = String>) -> Result<(), String> {
    let ParsedCliArgs::Run(parsed) = parse_common_snowbench_args(args)? else {
        return Ok(());
    };
    let run_dir = parsed
        .run_dir
        .ok_or_else(|| "SNOWBENCH-E-CLI missing --run-dir".to_string())?;
    let output_dir = parsed
        .output_dir
        .ok_or_else(|| "SNOWBENCH-E-CLI missing --output-dir".to_string())?;

    match command {
        "export-pysnobal" => {
            run_export_pysnobal(
                run_dir,
                parsed.run_file,
                output_dir,
                parsed.variant,
                parsed.coe_model,
            )?;
        }
        "physics-bulk" => {
            run_physics_bulk(
                run_dir,
                parsed.run_file,
                output_dir,
                parsed.variant,
                parsed.coe_model,
            )?;
        }
        "coe-melt" => {
            run_coe_melt(
                run_dir,
                parsed.run_file,
                output_dir,
                parsed.variant,
                parsed.coe_model,
            )?;
        }
        "coe-bound-density" => {
            run_coe_bound_density(
                run_dir,
                parsed.run_file,
                output_dir,
                parsed.variant,
                parsed.coe_model,
            )?;
        }
        _ => return Err(format!("SNOWBENCH-E-CLI unrecognized command {command}")),
    }
    Ok(())
}

#[derive(Debug, Clone, Default, PartialEq, Eq)]
struct CommonSnowbenchArgs {
    run_dir: Option<PathBuf>,
    run_file: Option<PathBuf>,
    output_dir: Option<PathBuf>,
    variant: PhysicsBulkVariant,
    coe_model: CoeMeltModel,
}

fn parse_common_snowbench_args(
    mut args: impl Iterator<Item = String>,
) -> Result<ParsedCliArgs<CommonSnowbenchArgs>, String> {
    let mut parsed = CommonSnowbenchArgs::default();
    while let Some(flag) = args.next() {
        if apply_common_snowbench_flag(&mut parsed, &flag, &mut args)? == CliParseAction::Help {
            print_help();
            return Ok(ParsedCliArgs::Help);
        }
    }
    Ok(ParsedCliArgs::Run(parsed))
}

fn apply_common_snowbench_flag(
    parsed: &mut CommonSnowbenchArgs,
    flag: &str,
    args: &mut impl Iterator<Item = String>,
) -> Result<CliParseAction, String> {
    match flag {
        "--run-dir" => parsed.run_dir = Some(next_path(args, "--run-dir")?),
        "--run-file" => parsed.run_file = Some(next_path(args, "--run-file")?),
        "--output-dir" => parsed.output_dir = Some(next_path(args, "--output-dir")?),
        "--variant" => {
            parsed.variant = parse_physics_bulk_variant(args)?;
        }
        "--model" => {
            parsed.coe_model = parse_coe_model(args)?;
        }
        "--help" | "-h" => return Ok(CliParseAction::Help),
        _ => return Err(format!("SNOWBENCH-E-CLI unrecognized argument {flag}")),
    }
    Ok(CliParseAction::Continue)
}

fn parse_physics_bulk_variant(
    args: &mut impl Iterator<Item = String>,
) -> Result<PhysicsBulkVariant, String> {
    let value = args
        .next()
        .ok_or_else(|| "SNOWBENCH-E-CLI missing value for --variant".to_string())?;
    PhysicsBulkVariant::parse(&value).map_err(|error| error.to_string())
}

fn parse_coe_model(args: &mut impl Iterator<Item = String>) -> Result<CoeMeltModel, String> {
    let value = args
        .next()
        .ok_or_else(|| "SNOWBENCH-E-CLI missing value for --model".to_string())?;
    CoeMeltModel::parse(&value).map_err(|error| error.to_string())
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum CliParseAction {
    Continue,
    Help,
}

#[derive(Debug, Clone, PartialEq, Eq)]
enum ParsedCliArgs<T> {
    Help,
    Run(T),
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

fn run_jennings_phase_args(args: impl Iterator<Item = String>) -> Result<(), String> {
    let ParsedCliArgs::Run(parsed) = parse_jennings_phase_args(args)? else {
        return Ok(());
    };
    run_jennings_phase_validation_args(parsed)
}

#[derive(Debug, Clone, Default, PartialEq, Eq)]
struct JenningsPhaseArgs {
    observations_path: Option<PathBuf>,
    thresholds_path: Option<PathBuf>,
    output_dir: Option<PathBuf>,
    max_rows: Option<usize>,
}

fn parse_jennings_phase_args(
    mut args: impl Iterator<Item = String>,
) -> Result<ParsedCliArgs<JenningsPhaseArgs>, String> {
    let mut parsed = JenningsPhaseArgs::default();
    while let Some(flag) = args.next() {
        if apply_jennings_phase_flag(&mut parsed, &flag, &mut args)? == CliParseAction::Help {
            print_help();
            return Ok(ParsedCliArgs::Help);
        }
    }
    Ok(ParsedCliArgs::Run(parsed))
}

fn apply_jennings_phase_flag(
    parsed: &mut JenningsPhaseArgs,
    flag: &str,
    args: &mut impl Iterator<Item = String>,
) -> Result<CliParseAction, String> {
    match flag {
        "--observations" => parsed.observations_path = Some(next_path(args, "--observations")?),
        "--thresholds" => parsed.thresholds_path = Some(next_path(args, "--thresholds")?),
        "--output-dir" => parsed.output_dir = Some(next_path(args, "--output-dir")?),
        "--max-rows" => parsed.max_rows = Some(parse_max_rows(args)?),
        "--help" | "-h" => return Ok(CliParseAction::Help),
        _ => return Err(format!("SNOWBENCH-E-CLI unrecognized argument {flag}")),
    }
    Ok(CliParseAction::Continue)
}

fn parse_max_rows(args: &mut impl Iterator<Item = String>) -> Result<usize, String> {
    let value = args
        .next()
        .ok_or_else(|| "SNOWBENCH-E-CLI missing value for --max-rows".to_string())?;
    value.parse::<usize>().map_err(|_| {
        format!("SNOWBENCH-E-CLI --max-rows must be a positive integer, observed {value}")
    })
}

fn run_jennings_phase_validation_args(parsed: JenningsPhaseArgs) -> Result<(), String> {
    let observations_path = parsed
        .observations_path
        .ok_or_else(|| "SNOWBENCH-E-CLI missing --observations".to_string())?;
    let thresholds_path = parsed
        .thresholds_path
        .ok_or_else(|| "SNOWBENCH-E-CLI missing --thresholds".to_string())?;
    let output_dir = parsed
        .output_dir
        .ok_or_else(|| "SNOWBENCH-E-CLI missing --output-dir".to_string())?;
    let report = run_jennings_phase_validation(&JenningsPhaseValidationRequest {
        observations_path,
        thresholds_path,
        output_dir,
        max_rows: parsed.max_rows,
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

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    #[test]
    fn classifies_top_level_help_and_jennings_commands() {
        assert!(matches!(
            classify_top_level_command("--help".to_string()),
            TopLevelCommand::Help
        ));
        assert!(matches!(
            classify_top_level_command("-h".to_string()),
            TopLevelCommand::Help
        ));
        assert!(matches!(
            classify_top_level_command("jennings-phase".to_string()),
            TopLevelCommand::JenningsPhase
        ));
        match classify_top_level_command("physics-bulk".to_string()) {
            TopLevelCommand::Snowbench(command) => assert_eq!(command, "physics-bulk"),
            _ => panic!("expected snowbench command"),
        }
    }

    #[test]
    fn common_snowbench_args_parse_all_options() {
        let parsed = parse_common_snowbench_args(args(&[
            "--run-dir",
            "run",
            "--run-file",
            "custom.run",
            "--output-dir",
            "out",
            "--variant",
            "density_compaction_v1",
            "--model",
            "coe_shortwave_albedo_v1",
        ]))
        .expect("common args should parse");

        assert_eq!(
            parsed,
            ParsedCliArgs::Run(CommonSnowbenchArgs {
                run_dir: Some(PathBuf::from("run")),
                run_file: Some(PathBuf::from("custom.run")),
                output_dir: Some(PathBuf::from("out")),
                variant: PhysicsBulkVariant::DensityCompactionV1,
                coe_model: CoeMeltModel::CoeShortwaveAlbedoV1,
            })
        );
    }

    #[test]
    fn common_snowbench_args_preserve_error_and_help_precedence() {
        assert_eq!(
            parse_common_snowbench_args(args(&["--variant"]))
                .expect_err("missing variant value should fail"),
            "SNOWBENCH-E-CLI missing value for --variant"
        );
        assert_eq!(
            parse_common_snowbench_args(args(&["--model"]))
                .expect_err("missing model value should fail"),
            "SNOWBENCH-E-CLI missing value for --model"
        );
        assert_eq!(
            parse_common_snowbench_args(args(&["--bad"])).expect_err("unknown flag should fail"),
            "SNOWBENCH-E-CLI unrecognized argument --bad"
        );
        assert_eq!(
            parse_common_snowbench_args(args(&["--run-dir", "run", "--help", "--bad"]))
                .expect("help should short-circuit"),
            ParsedCliArgs::Help
        );
    }

    #[test]
    fn run_with_args_preserves_command_error_precedence() {
        assert_eq!(
            run_with_args(args(&[])).expect_err("missing command should fail"),
            "SNOWBENCH-E-CLI missing command"
        );
        assert_eq!(
            run_with_args(args(&["unknown"])).expect_err("missing run dir wins first"),
            "SNOWBENCH-E-CLI missing --run-dir"
        );
        assert_eq!(
            run_with_args(args(&["unknown", "--bad"]))
                .expect_err("unknown argument wins before command dispatch"),
            "SNOWBENCH-E-CLI unrecognized argument --bad"
        );
        assert_eq!(
            run_with_args(args(&[
                "unknown",
                "--run-dir",
                "run",
                "--output-dir",
                "out"
            ]))
            .expect_err("unknown command should fail after common args parse"),
            "SNOWBENCH-E-CLI unrecognized command unknown"
        );
        assert!(run_with_args(args(&["unknown", "--help"])).is_ok());
    }

    #[test]
    fn command_specific_guards_return_before_heavy_work() {
        assert_eq!(
            run_with_args(args(&[
                "export-pysnobal",
                "--run-dir",
                "run",
                "--output-dir",
                "out",
                "--variant",
                "slow_melt_v1",
            ]))
            .expect_err("export-pysnobal rejects variant before running"),
            "SNOWBENCH-E-CLI --variant is only valid for physics-bulk"
        );
        assert_eq!(
            run_with_args(args(&[
                "physics-bulk",
                "--run-dir",
                "run",
                "--output-dir",
                "out",
                "--model",
                "coe_shortwave_albedo_v1",
            ]))
            .expect_err("physics-bulk rejects model before running"),
            "SNOWBENCH-E-CLI --model is only valid for coe-melt"
        );
        assert_eq!(
            run_with_args(args(&[
                "coe-melt",
                "--run-dir",
                "run",
                "--output-dir",
                "out",
                "--variant",
                "slow_melt_v1",
            ]))
            .expect_err("coe-melt rejects variant before running"),
            "SNOWBENCH-E-CLI --variant is only valid for physics-bulk"
        );
        assert!(
            run_with_args(args(&[
                "coe-bound-density",
                "--run-dir",
                "run",
                "--output-dir",
                "out",
                "--variant",
                "candidate_v1",
            ]))
            .expect_err("coe-bound-density rejects unsupported density variant")
            .contains(
                "coe-bound-density currently accepts only density_compaction_v1 or spring_densification_v1"
            )
        );
    }

    #[test]
    fn jennings_phase_args_parse_all_options() {
        let parsed = parse_jennings_phase_args(args(&[
            "--observations",
            "observations.csv",
            "--thresholds",
            "thresholds.csv",
            "--output-dir",
            "out",
            "--max-rows",
            "42",
        ]))
        .expect("jennings args should parse");

        assert_eq!(
            parsed,
            ParsedCliArgs::Run(JenningsPhaseArgs {
                observations_path: Some(PathBuf::from("observations.csv")),
                thresholds_path: Some(PathBuf::from("thresholds.csv")),
                output_dir: Some(PathBuf::from("out")),
                max_rows: Some(42),
            })
        );
    }

    #[test]
    fn jennings_phase_args_preserve_error_and_help_precedence() {
        assert_eq!(
            parse_jennings_phase_args(args(&["--max-rows"]))
                .expect_err("missing max rows value should fail"),
            "SNOWBENCH-E-CLI missing value for --max-rows"
        );
        assert_eq!(
            parse_jennings_phase_args(args(&["--max-rows", "abc"]))
                .expect_err("invalid max rows should fail"),
            "SNOWBENCH-E-CLI --max-rows must be a positive integer, observed abc"
        );
        assert_eq!(
            parse_jennings_phase_args(args(&["--bad"])).expect_err("unknown flag should fail"),
            "SNOWBENCH-E-CLI unrecognized argument --bad"
        );
        assert_eq!(
            parse_jennings_phase_args(args(&["--observations", "obs.csv", "--help", "--bad"]))
                .expect("help should short-circuit"),
            ParsedCliArgs::Help
        );
    }

    #[test]
    fn jennings_phase_run_reports_missing_required_inputs_in_order() {
        assert_eq!(
            run_jennings_phase_args(args(&[])).expect_err("observations is required first"),
            "SNOWBENCH-E-CLI missing --observations"
        );
        assert_eq!(
            run_jennings_phase_args(args(&["--observations", "obs.csv"]))
                .expect_err("thresholds is required second"),
            "SNOWBENCH-E-CLI missing --thresholds"
        );
        assert_eq!(
            run_jennings_phase_args(args(&[
                "--observations",
                "obs.csv",
                "--thresholds",
                "thresholds.csv",
            ]))
            .expect_err("output dir is required third"),
            "SNOWBENCH-E-CLI missing --output-dir"
        );
        assert!(run_jennings_phase_args(args(&["--help"])).is_ok());
    }

    #[test]
    fn jennings_phase_run_accepts_minimal_valid_fixture() {
        let case_dir = PathBuf::from("target/openwepp_snowbench_cli_tests/jennings_minimal");
        if case_dir.exists() {
            fs::remove_dir_all(&case_dir).expect("cleanup should remove stale Jennings fixture");
        }
        fs::create_dir_all(&case_dir).expect("fixture directory should be created");
        let observations = case_dir.join("file2.csv");
        let thresholds = case_dir.join("file3.csv");
        let output_dir = case_dir.join("out");
        fs::write(
            &observations,
            "Station_ID,Date,Hour,Air_Temp,Dewpoint,RH,gridded_data_pres,Prec_Type,Snow_Phase,Rain_Phase\nSTATION,2020-01-01,1,1.0,0.0,95.0,0.01,rain,0,1\nSTATION,2020-01-01,2,-1.0,-2.0,80.0,0.01,snow,1,0\n",
        )
        .expect("observations fixture should be written");
        fs::write(&thresholds, "Station_ID,temp50\nSTATION,0.0\n")
            .expect("threshold fixture should be written");

        run_jennings_phase_args(args(&[
            "--observations",
            observations
                .to_str()
                .expect("observations path should be utf-8"),
            "--thresholds",
            thresholds
                .to_str()
                .expect("thresholds path should be utf-8"),
            "--output-dir",
            output_dir.to_str().expect("output path should be utf-8"),
        ]))
        .expect("minimal Jennings fixture should validate");

        assert!(output_dir.join("jennings-validation-report.json").is_file());
        assert!(output_dir.join("jennings-validation-report.md").is_file());
    }

    fn args(values: &[&str]) -> std::vec::IntoIter<String> {
        values
            .iter()
            .map(|value| (*value).to_string())
            .collect::<Vec<_>>()
            .into_iter()
    }
}
