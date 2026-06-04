use std::env;
use std::error::Error;
use std::fmt;
use std::fs;
use std::path::{Path, PathBuf};
use std::process::ExitCode;

use openwepp_hillslope_orchestrator::HillslopePhaseGraph;
use openwepp_hillslope_orchestrator::schedule_export::{
    ScheduleExportError, canonical_hillslope_schedule_export, diff_schedule_json,
    render_schedule_diff, validate_hillslope_schedule_graph,
};

#[derive(Debug)]
enum CliError {
    Args(String),
    Export(ScheduleExportError),
    Io(std::io::Error),
}

impl fmt::Display for CliError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Args(message) => write!(f, "{message}"),
            Self::Export(source) => write!(f, "{source}"),
            Self::Io(source) => write!(f, "I/O failed: {source}"),
        }
    }
}

impl Error for CliError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            Self::Args(_) => None,
            Self::Export(source) => Some(source),
            Self::Io(source) => Some(source),
        }
    }
}

impl From<ScheduleExportError> for CliError {
    fn from(value: ScheduleExportError) -> Self {
        Self::Export(value)
    }
}

impl From<std::io::Error> for CliError {
    fn from(value: std::io::Error) -> Self {
        Self::Io(value)
    }
}

fn main() -> ExitCode {
    match run() {
        Ok(()) => ExitCode::SUCCESS,
        Err(error) => {
            eprintln!("{error}");
            ExitCode::FAILURE
        }
    }
}

fn run() -> Result<(), CliError> {
    let args: Vec<String> = env::args().skip(1).collect();
    let Some(command) = args.first().map(String::as_str) else {
        return Err(CliError::Args(usage()));
    };

    match command {
        "export" => export_command(&args[1..]),
        "generate" => generate_command(&args[1..]),
        "validate" => validate_command(&args[1..]),
        "topological-order" => topological_order_command(&args[1..]),
        "diff" => diff_command(&args[1..]),
        "-h" | "--help" | "help" => {
            print!("{}", usage());
            Ok(())
        }
        _ => Err(CliError::Args(usage())),
    }
}

fn export_command(args: &[String]) -> Result<(), CliError> {
    let mut format = "json";
    let mut output_path: Option<PathBuf> = None;
    let mut index = 0;
    while index < args.len() {
        match args[index].as_str() {
            "--format" => {
                let Some(value) = args.get(index + 1) else {
                    return Err(CliError::Args("--format requires a value".to_owned()));
                };
                format = value;
                index += 2;
            }
            "--output" => {
                let Some(value) = args.get(index + 1) else {
                    return Err(CliError::Args("--output requires a path".to_owned()));
                };
                output_path = Some(PathBuf::from(value));
                index += 2;
            }
            _ => return Err(CliError::Args(usage())),
        }
    }

    let export = canonical_hillslope_schedule_export()?;
    let rendered = match format {
        "json" => export.render_json(),
        "mermaid" | "mmd" => export.render_mermaid(),
        "dot" => export.render_dot(),
        _ => {
            let mut message = "unsupported format: ".to_owned();
            message.push_str(format);
            return Err(CliError::Args(message));
        }
    };

    write_output(&rendered, output_path.as_deref())
}

fn generate_command(args: &[String]) -> Result<(), CliError> {
    let mut output_dir = PathBuf::from("docs/architecture/generated");
    let mut index = 0;
    while index < args.len() {
        match args[index].as_str() {
            "--output-dir" => {
                let Some(value) = args.get(index + 1) else {
                    return Err(CliError::Args("--output-dir requires a path".to_owned()));
                };
                output_dir = PathBuf::from(value);
                index += 2;
            }
            _ => return Err(CliError::Args(usage())),
        }
    }

    fs::create_dir_all(&output_dir)?;
    let export = canonical_hillslope_schedule_export()?;
    fs::write(
        output_dir.join("hillslope-phase-schedule.json"),
        export.render_json(),
    )?;
    fs::write(
        output_dir.join("hillslope-phase-schedule.mmd"),
        export.render_mermaid(),
    )?;
    fs::write(
        output_dir.join("hillslope-phase-schedule.dot"),
        export.render_dot(),
    )?;
    Ok(())
}

fn validate_command(args: &[String]) -> Result<(), CliError> {
    if !args.is_empty() {
        return Err(CliError::Args(usage()));
    }

    let graph = HillslopePhaseGraph::canonical();
    let report = validate_hillslope_schedule_graph(&graph);
    if report.is_valid() {
        println!("schedule validation: ok");
        return Ok(());
    }

    for diagnostic in report.diagnostics {
        println!("schedule validation: {}", diagnostic.message());
    }
    Err(CliError::Args("schedule validation failed".to_owned()))
}

fn topological_order_command(args: &[String]) -> Result<(), CliError> {
    if !args.is_empty() {
        return Err(CliError::Args(usage()));
    }

    let export = canonical_hillslope_schedule_export()?;
    for phase in export.topological_order {
        println!("{}", phase.as_str());
    }
    Ok(())
}

fn diff_command(args: &[String]) -> Result<(), CliError> {
    if args.len() != 2 {
        return Err(CliError::Args(usage()));
    }

    let base = fs::read_to_string(&args[0])?;
    let head = fs::read_to_string(&args[1])?;
    let diff = diff_schedule_json(&base, &head)?;
    print!("{}", render_schedule_diff(&diff));
    Ok(())
}

fn write_output(contents: &str, output_path: Option<&Path>) -> Result<(), CliError> {
    if let Some(path) = output_path {
        fs::write(path, contents)?;
    } else {
        print!("{contents}");
    }
    Ok(())
}

fn usage() -> String {
    [
        "Usage:",
        "  openwepp_hillslope_schedule_export export --format json|mermaid|dot [--output PATH]",
        "  openwepp_hillslope_schedule_export generate [--output-dir DIR]",
        "  openwepp_hillslope_schedule_export validate",
        "  openwepp_hillslope_schedule_export topological-order",
        "  openwepp_hillslope_schedule_export diff BASE_JSON HEAD_JSON",
        "",
    ]
    .join("\n")
}
