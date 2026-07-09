use std::path::{Path, PathBuf};

use crate::{
    ClassMap, LanduseMigrationError, MigrationAuthority, MigrationRequest, MigrationTarget,
    ReportFormat, format_arg_spec, format_migration_report, format_validation_report,
    load_args_file, load_class_map, migrate_path, required_args_for_path, validate_path,
};

pub fn run_from_env() -> Result<String, LanduseMigrationError> {
    let args: Vec<String> = std::env::args().skip(1).collect();
    run_cli_args(&args)
}

#[allow(clippy::too_many_lines)]
pub fn run_cli_args(args: &[String]) -> Result<String, LanduseMigrationError> {
    if args.is_empty() {
        return Ok(help_text());
    }

    let mut input: Option<PathBuf> = None;
    let mut args_for_target: Option<MigrationTarget> = None;
    let mut validate = false;
    let mut target: Option<MigrationTarget> = None;
    let mut output: Option<PathBuf> = None;
    let mut disturbed_class: Option<String> = None;
    let mut class_map_file: Option<PathBuf> = None;
    let mut args_file: Option<PathBuf> = None;
    let mut dry_run = false;
    let mut report: Option<PathBuf> = None;
    let mut report_format = ReportFormat::Text;
    let mut stdout_format = ReportFormat::Text;

    let mut cursor = 0usize;
    while cursor < args.len() {
        match args[cursor].as_str() {
            "--help" | "-h" => return Ok(help_text()),
            "--args-for-migration-to" => {
                cursor += 1;
                args_for_target = Some(parse_target(value_at(
                    args,
                    cursor,
                    "--args-for-migration-to",
                )?)?);
            }
            "--validate" => {
                validate = true;
            }
            "--to" => {
                cursor += 1;
                target = Some(parse_target(value_at(args, cursor, "--to")?)?);
            }
            "--output" => {
                cursor += 1;
                output = Some(PathBuf::from(value_at(args, cursor, "--output")?));
            }
            "--disturbed-class" => {
                cursor += 1;
                disturbed_class = Some(value_at(args, cursor, "--disturbed-class")?.to_string());
            }
            "--disturbed-class-map" => {
                cursor += 1;
                class_map_file = Some(PathBuf::from(value_at(
                    args,
                    cursor,
                    "--disturbed-class-map",
                )?));
            }
            "--args-file" => {
                cursor += 1;
                args_file = Some(PathBuf::from(value_at(args, cursor, "--args-file")?));
            }
            "--dry-run" => {
                dry_run = true;
            }
            "--report" => {
                cursor += 1;
                report = Some(PathBuf::from(value_at(args, cursor, "--report")?));
            }
            "--report-format" => {
                cursor += 1;
                report_format = value_at(args, cursor, "--report-format")?.parse()?;
            }
            "--format" => {
                cursor += 1;
                stdout_format = value_at(args, cursor, "--format")?.parse()?;
            }
            flag if flag.starts_with('-') => {
                return Err(LanduseMigrationError::InvalidCommand {
                    detail: format!("unrecognized argument {flag}"),
                });
            }
            path => {
                if input.is_some() {
                    return Err(LanduseMigrationError::InvalidCommand {
                        detail: format!("multiple input paths supplied; unexpected {path}"),
                    });
                }
                input = Some(PathBuf::from(path));
            }
        }
        cursor += 1;
    }

    let input = input.ok_or_else(|| LanduseMigrationError::InvalidCommand {
        detail: "missing input path".to_string(),
    })?;

    if let Some(discovery_target) = args_for_target {
        if validate || target.is_some() || output.is_some() {
            return Err(LanduseMigrationError::InvalidCommand {
                detail:
                    "--args-for-migration-to cannot be combined with --validate, --to, or --output"
                        .to_string(),
            });
        }
        let spec = required_args_for_path(input, discovery_target)?;
        return format_arg_spec(&spec, stdout_format);
    }

    let args_file_values = if let Some(path) = &args_file {
        Some(load_args_file(path)?)
    } else {
        None
    };
    if target.is_none() {
        target = args_file_values.as_ref().and_then(|values| values.target);
    }

    let mut authority = MigrationAuthority {
        disturbed_class,
        disturbed_class_map: ClassMap::default(),
    };
    if let Some(args_file_values) = args_file_values {
        authority.merge_from_args_file(args_file_values)?;
    }
    if let Some(path) = class_map_file.as_deref() {
        authority
            .disturbed_class_map
            .merge_checked(load_class_map(path)?)?;
    }

    if validate {
        let target = validate_target_for_input(&input, target)?;
        let request = MigrationRequest {
            input,
            target,
            output: None,
            authority,
            dry_run: true,
            report,
            report_format,
        };
        let validation = validate_path(&request)?;
        if let Some(report_path) = &request.report {
            let report_text = format_validation_report(&validation, request.report_format)?;
            std::fs::write(report_path, report_text).map_err(|source| {
                LanduseMigrationError::Io {
                    action: "write",
                    path: report_path.clone(),
                    source,
                }
            })?;
        }
        return format_validation_report(&validation, stdout_format);
    }

    let target = target.ok_or_else(|| LanduseMigrationError::InvalidCommand {
        detail: "missing --to <target>; use --validate for validation-only mode".to_string(),
    })?;
    let request = MigrationRequest {
        input,
        target,
        output,
        authority,
        dry_run,
        report,
        report_format,
    };
    let output = migrate_path(&request)?;
    if request.dry_run {
        return format_migration_report(&output.report, stdout_format);
    }
    let Some(output_path) = output.output_path.as_ref() else {
        return Err(LanduseMigrationError::InvalidCommand {
            detail: "migration completed without an output path".to_string(),
        });
    };
    Ok(format!("wrote {}\n", output_path.display()))
}

fn validate_target_for_input(
    input: &Path,
    target: Option<MigrationTarget>,
) -> Result<MigrationTarget, LanduseMigrationError> {
    if let Some(target) = target {
        return Ok(target);
    }
    if openwepp_management_schema::consumer_accepts_management_yaml_extension(input) {
        return Ok(MigrationTarget::Latest);
    }
    Err(LanduseMigrationError::InvalidCommand {
        detail: "flat-source --validate requires --to <target>".to_string(),
    })
}

fn parse_target(value: &str) -> Result<MigrationTarget, LanduseMigrationError> {
    value.parse()
}

fn value_at<'a>(
    args: &'a [String],
    cursor: usize,
    flag: &str,
) -> Result<&'a str, LanduseMigrationError> {
    args.get(cursor)
        .map(String::as_str)
        .ok_or_else(|| LanduseMigrationError::InvalidCommand {
            detail: format!("missing value for {flag}"),
        })
}

#[must_use]
pub fn help_text() -> String {
    [
        "openwepp-landuse-migrate <input> --args-for-migration-to <ow-lanuse-1|latest> [--format text|json|toml]",
        "openwepp-landuse-migrate <input> --validate [--to <ow-lanuse-1|latest>] [--disturbed-class <class>] [--disturbed-class-map <path>] [--args-file <path>] [--format text|json|toml]",
        "openwepp-landuse-migrate <input> --to <ow-lanuse-1|latest> [--output <output.yaml>] [--disturbed-class <class>] [--disturbed-class-map <path>] [--args-file <path>] [--dry-run] [--report <path>] [--report-format text|json|toml]",
        "",
    ]
    .join("\n")
}
