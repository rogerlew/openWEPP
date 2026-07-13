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

pub fn run_cli_args(args: &[String]) -> Result<String, LanduseMigrationError> {
    if args.is_empty() {
        return Ok(help_text());
    }

    let Some(mut options) = parse_options(args)? else {
        return Ok(help_text());
    };
    let input = options
        .input
        .take()
        .ok_or_else(|| LanduseMigrationError::InvalidCommand {
            detail: "missing input path".to_string(),
        })?;

    if let Some(output) = run_discovery(&input, &options)? {
        return Ok(output);
    }

    let authority = parse_authority(&mut options)?;
    if options.validate {
        run_validation(input, &options, authority)
    } else {
        run_migration(input, &options, authority)
    }
}

#[derive(Default)]
struct ParsedCliArgs {
    input: Option<PathBuf>,
    args_for_target: Option<MigrationTarget>,
    validate: bool,
    target: Option<MigrationTarget>,
    output: Option<PathBuf>,
    disturbed_class: Option<String>,
    class_map_file: Option<PathBuf>,
    args_file: Option<PathBuf>,
    dry_run: bool,
    report: Option<PathBuf>,
    report_format: ReportFormat,
    stdout_format: ReportFormat,
}

fn parse_options(args: &[String]) -> Result<Option<ParsedCliArgs>, LanduseMigrationError> {
    let mut options = ParsedCliArgs::default();
    let mut cursor = 0usize;
    while cursor < args.len() {
        match args[cursor].as_str() {
            "--help" | "-h" => return Ok(None),
            "--args-for-migration-to" => {
                cursor += 1;
                options.args_for_target = Some(parse_target(value_at(
                    args,
                    cursor,
                    "--args-for-migration-to",
                )?)?);
            }
            "--validate" => options.validate = true,
            "--to" => {
                cursor += 1;
                options.target = Some(parse_target(value_at(args, cursor, "--to")?)?);
            }
            "--output" => {
                cursor += 1;
                options.output = Some(PathBuf::from(value_at(args, cursor, "--output")?));
            }
            "--disturbed-class" => {
                cursor += 1;
                options.disturbed_class =
                    Some(value_at(args, cursor, "--disturbed-class")?.to_string());
            }
            "--disturbed-class-map" => {
                cursor += 1;
                options.class_map_file = Some(PathBuf::from(value_at(
                    args,
                    cursor,
                    "--disturbed-class-map",
                )?));
            }
            "--args-file" => {
                cursor += 1;
                options.args_file = Some(PathBuf::from(value_at(args, cursor, "--args-file")?));
            }
            "--dry-run" => options.dry_run = true,
            "--report" => {
                cursor += 1;
                options.report = Some(PathBuf::from(value_at(args, cursor, "--report")?));
            }
            "--report-format" => {
                cursor += 1;
                options.report_format = value_at(args, cursor, "--report-format")?.parse()?;
            }
            "--format" => {
                cursor += 1;
                options.stdout_format = value_at(args, cursor, "--format")?.parse()?;
            }
            flag if flag.starts_with('-') => {
                return Err(LanduseMigrationError::InvalidCommand {
                    detail: format!("unrecognized argument {flag}"),
                });
            }
            path => {
                if options.input.is_some() {
                    return Err(LanduseMigrationError::InvalidCommand {
                        detail: format!("multiple input paths supplied; unexpected {path}"),
                    });
                }
                options.input = Some(PathBuf::from(path));
            }
        }
        cursor += 1;
    }
    Ok(Some(options))
}

fn run_discovery(
    input: &Path,
    options: &ParsedCliArgs,
) -> Result<Option<String>, LanduseMigrationError> {
    let Some(discovery_target) = options.args_for_target else {
        return Ok(None);
    };
    if options.validate || options.target.is_some() || options.output.is_some() {
        return Err(LanduseMigrationError::InvalidCommand {
            detail: "--args-for-migration-to cannot be combined with --validate, --to, or --output"
                .to_string(),
        });
    }
    let spec = required_args_for_path(input, discovery_target)?;
    format_arg_spec(&spec, options.stdout_format).map(Some)
}

fn parse_authority(
    options: &mut ParsedCliArgs,
) -> Result<MigrationAuthority, LanduseMigrationError> {
    let args_file_values = if let Some(path) = &options.args_file {
        Some(load_args_file(path)?)
    } else {
        None
    };
    if options.target.is_none() {
        options.target = args_file_values.as_ref().and_then(|values| values.target);
    }

    let mut authority = MigrationAuthority {
        disturbed_class: options.disturbed_class.clone(),
        disturbed_class_map: ClassMap::default(),
    };
    if let Some(args_file_values) = args_file_values {
        authority.merge_from_args_file(args_file_values)?;
    }
    if let Some(path) = options.class_map_file.as_deref() {
        authority
            .disturbed_class_map
            .merge_checked(load_class_map(path)?)?;
    }
    Ok(authority)
}

fn run_validation(
    input: PathBuf,
    options: &ParsedCliArgs,
    authority: MigrationAuthority,
) -> Result<String, LanduseMigrationError> {
    let target = validate_target_for_input(&input, options.target)?;
    let request = MigrationRequest {
        input,
        target,
        output: None,
        authority,
        dry_run: true,
        report: options.report.clone(),
        report_format: options.report_format,
    };
    let validation = validate_path(&request)?;
    if let Some(report_path) = &request.report {
        let report_text = format_validation_report(&validation, request.report_format)?;
        std::fs::write(report_path, report_text).map_err(|source| LanduseMigrationError::Io {
            action: "write",
            path: report_path.clone(),
            source,
        })?;
    }
    format_validation_report(&validation, options.stdout_format)
}

fn run_migration(
    input: PathBuf,
    options: &ParsedCliArgs,
    authority: MigrationAuthority,
) -> Result<String, LanduseMigrationError> {
    let target = options
        .target
        .ok_or_else(|| LanduseMigrationError::InvalidCommand {
            detail: "missing --to <target>; use --validate for validation-only mode".to_string(),
        })?;
    let request = MigrationRequest {
        input,
        target,
        output: options.output.clone(),
        authority,
        dry_run: options.dry_run,
        report: options.report.clone(),
        report_format: options.report_format,
    };
    let output = migrate_path(&request)?;
    if request.dry_run {
        return format_migration_report(&output.report, options.stdout_format);
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

#[cfg(test)]
mod m13_tests {
    use std::fs;
    use std::time::{SystemTime, UNIX_EPOCH};

    use super::*;

    fn args_file() -> PathBuf {
        let nanos = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("system time must be after epoch")
            .as_nanos();
        let path = std::env::temp_dir().join(format!(
            "openwepp-landuse-migrate-m13-target-{}-{nanos}.json",
            std::process::id()
        ));
        fs::write(
            &path,
            r#"{"target":"latest","disturbed_class":"agriculture crops"}"#,
        )
        .expect("M-13 args file should write");
        path
    }

    #[test]
    fn m13_explicit_target_precedes_args_file_and_absent_target_inherits() {
        let path = args_file();
        let explicit = vec![
            "input.man".to_string(),
            "--to".to_string(),
            "ow-lanuse-1".to_string(),
            "--args-file".to_string(),
            path.display().to_string(),
        ];
        let mut explicit = parse_options(&explicit)
            .expect("explicit options should parse")
            .expect("explicit options are not help");
        let explicit_authority =
            parse_authority(&mut explicit).expect("explicit authority should parse");
        assert_eq!(explicit.target, Some(MigrationTarget::OwLanuse1));
        assert_eq!(
            explicit_authority.disturbed_class.as_deref(),
            Some("agriculture crops"),
        );

        let inherited = vec![
            "input.man".to_string(),
            "--args-file".to_string(),
            path.display().to_string(),
        ];
        let mut inherited = parse_options(&inherited)
            .expect("inherited options should parse")
            .expect("inherited options are not help");
        parse_authority(&mut inherited).expect("inherited authority should parse");
        assert_eq!(inherited.target, Some(MigrationTarget::Latest));
    }
}
