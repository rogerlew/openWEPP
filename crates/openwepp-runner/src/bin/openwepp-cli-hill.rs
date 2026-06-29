use std::path::PathBuf;

use openwepp_runner::{
    HillslopeDefaultRuntimeActivation, HillslopeRunRequest, HillslopeRuntimeSelection,
    HillslopeRuntimeSelectionPolicy, SidecarPolicy, execute_hillslope_run_with_runtime_policy,
};

fn main() {
    if let Err(error) = run() {
        eprintln!("{error}");
        std::process::exit(1);
    }
}

#[allow(clippy::too_many_lines)]
fn run() -> Result<(), String> {
    let mut run_dir: Option<PathBuf> = None;
    let mut run_file: Option<PathBuf> = None;
    let mut output_dir: Option<PathBuf> = None;
    let mut policy = SidecarPolicy::Compat;
    let mut legacy_sidecar_discovery = false;
    let mut manifest_path: Option<PathBuf> = None;
    let mut runtime_selection = HillslopeRuntimeSelection::DefaultCandidate;
    let mut runtime_selection_flag: Option<&'static str> = None;
    let mut default_activation = HillslopeDefaultRuntimeActivation::default();
    let mut default_activation_flag: Option<&'static str> = None;

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
            "--legacy-sidecar-discovery" => {
                legacy_sidecar_discovery = true;
            }
            "--compatibility-runtime" => {
                set_runtime_selection(
                    &mut runtime_selection,
                    &mut runtime_selection_flag,
                    HillslopeRuntimeSelection::Compatibility,
                    "--compatibility-runtime",
                )?;
            }
            "--direct-runtime-skeleton" => {
                set_runtime_selection(
                    &mut runtime_selection,
                    &mut runtime_selection_flag,
                    HillslopeRuntimeSelection::DirectSkeletonNoop,
                    "--direct-runtime-skeleton",
                )?;
            }
            "--direct-publication-frame-shadow" => {
                set_runtime_selection(
                    &mut runtime_selection,
                    &mut runtime_selection_flag,
                    HillslopeRuntimeSelection::DirectPublicationFrameShadow,
                    "--direct-publication-frame-shadow",
                )?;
            }
            "--direct-publication-frame-cutover" => {
                set_runtime_selection(
                    &mut runtime_selection,
                    &mut runtime_selection_flag,
                    HillslopeRuntimeSelection::DirectPublicationFrameCutover,
                    "--direct-publication-frame-cutover",
                )?;
            }
            "--direct-production-executor" => {
                set_runtime_selection(
                    &mut runtime_selection,
                    &mut runtime_selection_flag,
                    HillslopeRuntimeSelection::DirectProductionExecutor,
                    "--direct-production-executor",
                )?;
            }
            "--direct-default-candidate" => {
                if default_activation_flag.is_some() {
                    return Err(
                        "CLIHILL-E-001 duplicate default activation flag --direct-default-candidate"
                            .to_string(),
                    );
                }
                default_activation_flag = Some("--direct-default-candidate");
                default_activation = HillslopeDefaultRuntimeActivation::DirectProductionCandidate;
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
    if default_activation_flag.is_some() && runtime_selection_flag.is_some() {
        let Some(default_activation_flag) = default_activation_flag else {
            return Err("CLIHILL-E-001 internal default activation flag state error".to_string());
        };
        return Err(format!(
            "CLIHILL-E-001 {default_activation_flag} cannot be combined with an explicit runtime flag"
        ));
    }

    let report = execute_hillslope_run_with_runtime_policy(
        &HillslopeRunRequest {
            run_dir,
            run_file,
            output_dir,
            sidecar_policy: policy,
            legacy_sidecar_discovery,
            manifest_path,
        },
        &args,
        HillslopeRuntimeSelectionPolicy::new(runtime_selection, default_activation),
    )
    .map_err(|error| error.to_string())?;

    for warning in report.sidecar_warnings {
        eprintln!("sidecar-warning: {warning}");
    }

    Ok(())
}

fn set_runtime_selection(
    runtime_selection: &mut HillslopeRuntimeSelection,
    runtime_selection_flag: &mut Option<&'static str>,
    selection: HillslopeRuntimeSelection,
    flag: &'static str,
) -> Result<(), String> {
    if let Some(previous_flag) = *runtime_selection_flag {
        return Err(format!(
            "CLIHILL-E-001 runtime flag {flag} conflicts with {previous_flag}"
        ));
    }
    *runtime_selection = selection;
    *runtime_selection_flag = Some(flag);
    Ok(())
}

fn print_help() {
    println!(
        "openwepp-cli-hill --run-dir <path> --run-file <path> --output-dir <path> [--policy compat] [--legacy-sidecar-discovery] [--manifest-path <path>] [--compatibility-runtime] [--direct-default-candidate] [--direct-runtime-skeleton] [--direct-publication-frame-shadow] [--direct-publication-frame-cutover] [--direct-production-executor]"
    );
}
