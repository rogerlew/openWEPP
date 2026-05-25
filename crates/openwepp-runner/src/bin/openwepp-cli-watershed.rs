use std::collections::{BTreeMap, BTreeSet};
use std::fs;
use std::path::{Path, PathBuf};

use openwepp_input_contract::parsers::chaninp::{ChaninpParseOptions, parse_chaninp_from_path};
use openwepp_input_contract::parsers::hbp::{
    HbpParseMode, HbpParseOptions, parse_hbp_from_path_with_latest_event_payload,
};
use openwepp_input_contract::parsers::watershed_channel::{
    WatershedChannelParseMode, WatershedChannelParseOptions, parse_watershed_channel_from_path,
};
use openwepp_input_contract::parsers::watershed_impoundment::{
    ParseMode as WatershedImpoundmentParseMode, WatershedImpoundmentParseOptions,
    parse_watershed_impoundment_from_path,
};
use openwepp_input_contract::parsers::watershed_structure::{
    ParseMode as WatershedStructureParseMode, WatershedStructureFile,
    WatershedStructureParseOptions, parse_watershed_structure_from_path,
};
use openwepp_kernel_contract::{BoundarySymbol, BoundaryValue, WatershedProductionStateSymbol};
use openwepp_runner::SidecarPolicy;
use openwepp_topology::{
    ContributorTriplet, TopologyContributors, TopologyGraph, TopologyNode, TopologyNodeKey,
    TopologyNodeKind, validate_pre_execution_topology,
};
use openwepp_watershed_orchestrator::runtime_inputs::{
    build_watershed_runtime_surface_from_chaninp,
    seed_watershed_runtime_surface_from_watershed_channel,
    seed_watershed_runtime_surface_from_watershed_impoundment,
};
use openwepp_watershed_orchestrator::{
    WatershedWritebackSurface, Ws10ChannelImpoundmentKernel, execute_watershed_dispatch_with_kernel,
};
use openwepp_watershed_output::contracts::{WatershedOutputConfig, validate_output_contract};
use openwepp_watershed_output::writers::write_interchange_parquet_outputs;
use serde::Deserialize;

const WATERSHED_RUNFILE_SCHEMA_ID: &str = "openwepp-watershed-runfile-v1";
const DEFAULT_DTCHR_SECONDS: f64 = 3_600.0;
const DEFAULT_NTCHR: f64 = 24.0;

fn main() {
    if let Err(error) = run() {
        eprintln!("{error}");
        std::process::exit(1);
    }
}

#[allow(clippy::too_many_lines, clippy::similar_names)]
fn run() -> Result<(), String> {
    let mut run_dir: Option<PathBuf> = None;
    let mut run_file: Option<PathBuf> = None;
    let mut output_dir: Option<PathBuf> = None;
    let mut policy = SidecarPolicy::Strict;
    let mut legacy_sidecar_discovery = false;

    let args: Vec<String> = std::env::args().collect();
    let mut cursor = 1usize;
    while cursor < args.len() {
        match args[cursor].as_str() {
            "--run-dir" => {
                cursor += 1;
                let Some(value) = args.get(cursor) else {
                    return Err("CLIWAT-E-001 missing value for --run-dir".to_string());
                };
                run_dir = Some(PathBuf::from(value));
            }
            "--run-file" => {
                cursor += 1;
                let Some(value) = args.get(cursor) else {
                    return Err("CLIWAT-E-001 missing value for --run-file".to_string());
                };
                run_file = Some(PathBuf::from(value));
            }
            "--output-dir" => {
                cursor += 1;
                let Some(value) = args.get(cursor) else {
                    return Err("CLIWAT-E-001 missing value for --output-dir".to_string());
                };
                output_dir = Some(PathBuf::from(value));
            }
            "--policy" => {
                cursor += 1;
                let Some(value) = args.get(cursor) else {
                    return Err("CLIWAT-E-001 missing value for --policy".to_string());
                };
                policy = value.parse().map_err(|detail: String| {
                    format!("CLIWAT-E-001 invalid --policy value: {detail}")
                })?;
            }
            "--legacy-sidecar-discovery" => {
                legacy_sidecar_discovery = true;
            }
            "--help" | "-h" => {
                print_help();
                return Ok(());
            }
            flag => {
                return Err(format!("CLIWAT-E-001 unrecognized argument {flag}"));
            }
        }
        cursor += 1;
    }

    let Some(run_dir) = run_dir else {
        return Err("CLIWAT-E-001 missing --run-dir".to_string());
    };
    let Some(run_file) = run_file else {
        return Err("CLIWAT-E-001 missing --run-file".to_string());
    };
    let Some(output_dir) = output_dir else {
        return Err("CLIWAT-E-001 missing --output-dir".to_string());
    };

    if !run_dir.is_dir() {
        return Err(format!(
            "CLIWAT-E-002 run directory does not exist: {}",
            run_dir.display()
        ));
    }
    fs::create_dir_all(&output_dir).map_err(|error| {
        format!(
            "CLIWAT-E-003 failed creating output directory {}: {error}",
            output_dir.display()
        )
    })?;

    let run_file_path = resolve_run_file(&run_dir, &run_file);
    if !run_file_path.is_file() {
        return Err(format!(
            "CLIWAT-E-004 run file does not exist: {}",
            run_file_path.display()
        ));
    }

    let runfile = parse_watershed_runfile(
        &run_file_path,
        legacy_sidecar_discovery,
        &run_dir,
        &output_dir,
    )?;

    let structure_line_count = logical_watershed_structure_line_count(
        &runfile.watershed_structure_path,
    )
    .map_err(|error| {
        format!(
            "CLIWAT-E-005 failed reading watershed structure {}: {error}",
            runfile.watershed_structure_path.display()
        )
    })?;

    let expected_structure_rows = structure_line_count.checked_sub(1).ok_or_else(|| {
        format!(
            "CLIWAT-E-006 watershed structure {} has no row payload",
            runfile.watershed_structure_path.display()
        )
    })?;

    let structure_options = WatershedStructureParseOptions {
        mode: match policy {
            SidecarPolicy::Strict => WatershedStructureParseMode::Strict,
            SidecarPolicy::Compat => WatershedStructureParseMode::Compatibility,
        },
        nhill: runfile.hillslope_blocks_by_id.len(),
        expected_rows: Some(expected_structure_rows),
        expected_channel_count: None,
        expected_impoundment_count: None,
    };

    let structure =
        parse_watershed_structure_from_path(&runfile.watershed_structure_path, structure_options)
            .map_err(|error| {
            format!(
                "CLIWAT-E-007 failed parsing watershed structure {}: {error}",
                runfile.watershed_structure_path.display()
            )
        })?;

    let topology = build_topology_from_watershed_structure(&structure)?;
    let topology_validation = validate_pre_execution_topology(&topology)
        .map_err(|error| format!("CLIWAT-E-008 topology validation failed: {error}"))?;
    if !topology_validation.is_valid() {
        return Err(format!(
            "CLIWAT-E-008 topology precondition report is not valid (message_id={})",
            topology_validation.status.message_id()
        ));
    }

    let mut channel_options = WatershedChannelParseOptions {
        mode: match policy {
            SidecarPolicy::Strict => WatershedChannelParseMode::Strict,
            SidecarPolicy::Compat => WatershedChannelParseMode::Compatibility,
        },
        expected_channel_count: Some(structure.summary.channel_count),
        chan_inp_present: runfile.chaninp_path.is_some(),
        tcr_overlay_present: runfile.tcr_overlay_present,
        slplst_override: None,
    };
    if runfile.chaninp_path.is_none() {
        channel_options.chan_inp_present = false;
    }

    let watershed_channel =
        parse_watershed_channel_from_path(&runfile.watershed_channel_path, channel_options)
            .map_err(|error| {
                format!(
                    "CLIWAT-E-009 failed parsing watershed channel {}: {error}",
                    runfile.watershed_channel_path.display()
                )
            })?;

    let impoundment_options = WatershedImpoundmentParseOptions {
        mode: match policy {
            SidecarPolicy::Strict => WatershedImpoundmentParseMode::Strict,
            SidecarPolicy::Compat => WatershedImpoundmentParseMode::Compatibility,
        },
        expected_structural_count: Some(structure.summary.impoundment_count),
        ..WatershedImpoundmentParseOptions::default()
    };

    let watershed_impoundment = parse_watershed_impoundment_from_path(
        &runfile.watershed_impoundment_path,
        impoundment_options,
    )
    .map_err(|error| {
        format!(
            "CLIWAT-E-010 failed parsing watershed impoundment {}: {error}",
            runfile.watershed_impoundment_path.display()
        )
    })?;

    let mut sidecar_warnings = runfile.runfile_warnings;

    let mut runtime_surface = if let Some(chaninp_path) = runfile.chaninp_path.as_ref() {
        let valid_channel_element_ids = structure
            .rows
            .iter()
            .filter(|row| row.element_type_code == 2)
            .map(|row| row.element_id)
            .map(|id| {
                if id <= 0 {
                    Err(format!(
                        "CLIWAT-E-011 invalid channel element id {id} in watershed structure"
                    ))
                } else {
                    Ok(id)
                }
            })
            .collect::<Result<BTreeSet<_>, _>>()?;

        let chaninp_options = match policy {
            SidecarPolicy::Strict => {
                ChaninpParseOptions::strict(watershed_channel.ipeak, watershed_channel.nchan)
            }
            SidecarPolicy::Compat => {
                ChaninpParseOptions::compatibility(watershed_channel.ipeak, watershed_channel.nchan)
            }
        };

        let chaninp =
            parse_chaninp_from_path(chaninp_path, chaninp_options, &valid_channel_element_ids)
                .map_err(|error| {
                    format!(
                        "CLIWAT-E-012 failed parsing chan.inp {}: {error}",
                        chaninp_path.display()
                    )
                })?;

        build_watershed_runtime_surface_from_chaninp(&chaninp).map_err(|error| {
            format!("CLIWAT-E-013 failed building runtime surface from chan.inp: {error}")
        })?
    } else {
        sidecar_warnings.push(
            "chan.inp sidecar not provided; applying deterministic fallback channel globals (dtchr=3600, ntchr=24, nchnum=0, cbase=0).".to_string(),
        );
        build_default_chaninp_surface(&watershed_channel)
    };

    seed_watershed_runtime_surface_from_watershed_channel(&mut runtime_surface, &watershed_channel)
        .map_err(|error| {
            format!("CLIWAT-E-014 failed seeding watershed channel runtime surface: {error}")
        })?;

    seed_watershed_runtime_surface_from_watershed_impoundment(
        &mut runtime_surface,
        &watershed_impoundment,
    )
    .map_err(|error| {
        format!("CLIWAT-E-015 failed seeding watershed impoundment runtime surface: {error}")
    })?;

    let contributor_hillslopes = contributor_hillslope_ids(&topology);
    for hillslope_id in &contributor_hillslopes {
        if !runfile.hillslope_blocks_by_id.contains_key(hillslope_id) {
            return Err(format!(
                "CLIWAT-E-016 missing hillslopes_block entry for contributor hillslope id {hillslope_id}"
            ));
        }
    }

    let hbp_parse_mode = match policy {
        SidecarPolicy::Strict => HbpParseMode::Strict,
        SidecarPolicy::Compat => HbpParseMode::Compatibility,
    };

    for (hillslope_id, block) in &runfile.hillslope_blocks_by_id {
        let hbp_options = HbpParseOptions {
            mode: hbp_parse_mode,
            expected_hillslope_id: Some(*hillslope_id),
        };
        let (hbp, latest_event_payload) = parse_hbp_from_path_with_latest_event_payload(
            &block.pass_file_path,
            hbp_options,
        )
        .map_err(|error| {
            format!(
                "CLIWAT-E-017 failed parsing hillslope pass file {} for hillslope {}: {error}",
                block.pass_file_path.display(),
                hillslope_id
            )
        })?;

        let class_count = usize::from(hbp.npart);
        if class_count == 0 {
            return Err(format!(
                "CLIWAT-E-018 pass file {} reports npart=0 for hillslope {}",
                block.pass_file_path.display(),
                hillslope_id
            ));
        }

        let peak = latest_event_payload
            .as_ref()
            .map_or(0.0, |payload| payload.peak_runoff_m3_s);
        let duration = latest_event_payload
            .as_ref()
            .map_or(0.0, |payload| payload.duration_seconds);
        let total_detachment = latest_event_payload
            .as_ref()
            .map_or(0.0, |payload| payload.total_detachment_kg);
        let total_deposition = latest_event_payload
            .as_ref()
            .map_or(0.0, |payload| payload.total_deposition_kg);
        let sediment_concentrations = latest_event_payload.as_ref().map_or_else(
            || vec![0.0; class_count],
            |payload| payload.sediment_concentration_kg_m3.clone(),
        );
        let particle_flow_fractions = latest_event_payload.as_ref().map_or_else(
            || vec![0.0; class_count],
            |payload| payload.particle_flow_fraction.clone(),
        );

        runtime_surface.state_surface.insert(
            BoundarySymbol::from(WatershedProductionStateSymbol::HillslopeContributorPeak {
                hillslope_id: *hillslope_id,
            }),
            BoundaryValue::scalar(peak),
        );
        runtime_surface.state_surface.insert(
            BoundarySymbol::from(
                WatershedProductionStateSymbol::HillslopeContributorDuration {
                    hillslope_id: *hillslope_id,
                },
            ),
            BoundaryValue::scalar(duration),
        );
        runtime_surface.state_surface.insert(
            BoundarySymbol::from(
                WatershedProductionStateSymbol::HillslopeContributorTotalDetachmentKg {
                    hillslope_id: *hillslope_id,
                },
            ),
            BoundaryValue::scalar(total_detachment),
        );
        runtime_surface.state_surface.insert(
            BoundarySymbol::from(
                WatershedProductionStateSymbol::HillslopeContributorTotalDepositionKg {
                    hillslope_id: *hillslope_id,
                },
            ),
            BoundaryValue::scalar(total_deposition),
        );
        runtime_surface.state_surface.insert(
            BoundarySymbol::from(
                WatershedProductionStateSymbol::HillslopeContributorParticleClassCount {
                    hillslope_id: *hillslope_id,
                },
            ),
            BoundaryValue::scalar(f64::from(hbp.npart)),
        );

        for class_index in 1..=class_count {
            let concentration = sediment_concentrations
                .get(class_index - 1)
                .copied()
                .unwrap_or(0.0);
            let fraction = particle_flow_fractions
                .get(class_index - 1)
                .copied()
                .unwrap_or(0.0);
            runtime_surface.state_surface.insert(
                BoundarySymbol::from(
                    WatershedProductionStateSymbol::HillslopeContributorSedimentConcentrationKgM3 {
                        hillslope_id: *hillslope_id,
                        class_index,
                    },
                ),
                BoundaryValue::scalar(concentration),
            );
            runtime_surface.state_surface.insert(
                BoundarySymbol::from(
                    WatershedProductionStateSymbol::HillslopeContributorParticleFlowFraction {
                        hillslope_id: *hillslope_id,
                        class_index,
                    },
                ),
                BoundaryValue::scalar(fraction),
            );
        }
    }

    let mut kernel = Ws10ChannelImpoundmentKernel;
    let report = execute_watershed_dispatch_with_kernel(
        &topology,
        &topology_validation,
        &mut kernel,
        runtime_surface,
    )
    .map_err(|error| format!("CLIWAT-E-019 watershed execution failed: {error}"))?;

    if !report.dispatch_report.is_success() {
        return Err(format!(
            "CLIWAT-E-020 watershed dispatch reported failure (message_id={})",
            report.dispatch_report.dispatch_status.message_id()
        ));
    }

    write_watershed_interchange_outputs(&runfile.outputs)?;

    for warning in sidecar_warnings {
        eprintln!("sidecar-warning: {warning}");
    }

    Ok(())
}

fn resolve_run_file(run_dir: &Path, run_file: &Path) -> PathBuf {
    if run_file.is_absolute() {
        run_file.to_path_buf()
    } else {
        run_dir.join(run_file)
    }
}

fn resolve_runfile_relative_path(run_file_path: &Path, candidate: &str) -> PathBuf {
    let candidate_path = PathBuf::from(candidate);
    if candidate_path.is_absolute() {
        candidate_path
    } else {
        run_file_path
            .parent()
            .unwrap_or_else(|| Path::new("."))
            .join(candidate_path)
    }
}

fn resolve_required_runfile_path(
    run_file_path: &Path,
    candidate: &str,
    field: &'static str,
) -> Result<PathBuf, String> {
    let trimmed = candidate.trim();
    if trimmed.is_empty() {
        return Err(format!("CLIWAT-E-021 missing required non-empty {field}"));
    }
    Ok(resolve_runfile_relative_path(run_file_path, trimmed))
}

fn resolve_required_output_path(
    output_dir: &Path,
    candidate: &str,
    field: &'static str,
) -> Result<PathBuf, String> {
    let trimmed = candidate.trim();
    if trimmed.is_empty() {
        return Err(format!("CLIWAT-E-021 missing required non-empty {field}"));
    }
    let candidate_path = PathBuf::from(trimmed);
    if candidate_path.is_absolute() {
        Ok(candidate_path)
    } else {
        Ok(output_dir.join(candidate_path))
    }
}

fn resolve_optional_runfile_path(
    run_file_path: &Path,
    candidate: Option<&str>,
    field: &'static str,
) -> Result<Option<PathBuf>, String> {
    candidate.map_or(Ok(None), |value| {
        let trimmed = value.trim();
        if trimmed.is_empty() {
            Err(format!("CLIWAT-E-021 {field} cannot be an empty string"))
        } else {
            Ok(Some(resolve_runfile_relative_path(run_file_path, trimmed)))
        }
    })
}

#[derive(Debug, Deserialize, Default)]
struct WatershedRunfileDocument {
    schema: String,
    run_name: String,
    unit_system: String,
    #[serde(default)]
    inputs: WatershedRunfileInputs,
    #[serde(default)]
    outputs: WatershedRunfileOutputs,
}

#[derive(Debug, Deserialize, Default)]
struct WatershedRunfileInputs {
    pw0_str: String,
    pw0_chn: String,
    pw0_imp: String,
    pw0_man: String,
    pw0_slp: String,
    pw0_cli: String,
    pw0_sol: String,
    #[serde(default)]
    hillslopes_block: Vec<WatershedHillslopeBlock>,
    chaninp: Option<String>,
    tcr: Option<String>,
}

#[derive(Debug, Deserialize, Default)]
struct WatershedRunfileOutputs {
    ebe_pw0: String,
    chan_out: String,
    chanwb: String,
    chnwb: String,
    soil_pw0: String,
    totalwatsed3: String,
    loss_hill: String,
    loss_chn: String,
    loss_out: String,
    loss_class_data: String,
    loss_all_years_hill: String,
    loss_all_years_chn: String,
    loss_all_years_out: String,
    loss_all_years_class_data: String,
}

#[derive(Debug, Deserialize)]
struct WatershedHillslopeBlock {
    hillslope_id: u32,
    pass_file: String,
    #[serde(default)]
    unit_system: Option<String>,
    #[serde(default)]
    use_existing_pass_file: Option<bool>,
}

#[derive(Debug)]
struct WatershedHillslopeBlockResolved {
    pass_file_path: PathBuf,
}

type WatershedOutputsResolved = WatershedOutputConfig;

#[derive(Debug)]
struct WatershedRunfileResolved {
    watershed_structure_path: PathBuf,
    watershed_channel_path: PathBuf,
    watershed_impoundment_path: PathBuf,
    chaninp_path: Option<PathBuf>,
    tcr_overlay_present: bool,
    hillslope_blocks_by_id: BTreeMap<u32, WatershedHillslopeBlockResolved>,
    runfile_warnings: Vec<String>,
    outputs: WatershedOutputsResolved,
}

#[allow(clippy::too_many_lines)]
fn parse_watershed_runfile(
    run_file_path: &Path,
    legacy_sidecar_discovery: bool,
    run_dir: &Path,
    output_dir: &Path,
) -> Result<WatershedRunfileResolved, String> {
    let payload = fs::read_to_string(run_file_path).map_err(|error| {
        format!(
            "CLIWAT-E-022 failed reading run file {}: {error}",
            run_file_path.display()
        )
    })?;

    let runfile: WatershedRunfileDocument = toml::from_str(&payload).map_err(|error| {
        format!(
            "CLIWAT-E-023 invalid TOML in {}: {error}",
            run_file_path.display()
        )
    })?;

    if runfile.schema != WATERSHED_RUNFILE_SCHEMA_ID {
        return Err(format!(
            "CLIWAT-E-024 unsupported schema '{}' (expected '{}')",
            runfile.schema, WATERSHED_RUNFILE_SCHEMA_ID
        ));
    }

    if runfile.run_name.trim().is_empty() {
        return Err("CLIWAT-E-024 missing required non-empty run_name".to_string());
    }

    if runfile.unit_system.trim() != "metric" {
        return Err(format!(
            "CLIWAT-E-024 unsupported unit_system '{}' (expected 'metric')",
            runfile.unit_system
        ));
    }

    if runfile.inputs.hillslopes_block.is_empty() {
        return Err(
            "CLIWAT-E-024 inputs.hillslopes_block must contain at least one hillslope entry"
                .to_string(),
        );
    }

    let watershed_structure_path =
        resolve_required_runfile_path(run_file_path, &runfile.inputs.pw0_str, "inputs.pw0_str")?;
    let watershed_channel_path =
        resolve_required_runfile_path(run_file_path, &runfile.inputs.pw0_chn, "inputs.pw0_chn")?;
    let watershed_impoundment_path =
        resolve_required_runfile_path(run_file_path, &runfile.inputs.pw0_imp, "inputs.pw0_imp")?;
    let management_path =
        resolve_required_runfile_path(run_file_path, &runfile.inputs.pw0_man, "inputs.pw0_man")?;
    let slope_path =
        resolve_required_runfile_path(run_file_path, &runfile.inputs.pw0_slp, "inputs.pw0_slp")?;
    let climate_path =
        resolve_required_runfile_path(run_file_path, &runfile.inputs.pw0_cli, "inputs.pw0_cli")?;
    let soil_path =
        resolve_required_runfile_path(run_file_path, &runfile.inputs.pw0_sol, "inputs.pw0_sol")?;

    for (field, path) in [
        ("inputs.pw0_str", &watershed_structure_path),
        ("inputs.pw0_chn", &watershed_channel_path),
        ("inputs.pw0_imp", &watershed_impoundment_path),
        ("inputs.pw0_man", &management_path),
        ("inputs.pw0_slp", &slope_path),
        ("inputs.pw0_cli", &climate_path),
        ("inputs.pw0_sol", &soil_path),
    ] {
        if !path.is_file() {
            return Err(format!(
                "CLIWAT-E-025 required {field} path '{}' is not a readable file",
                path.display()
            ));
        }
    }

    let mut hillslope_blocks_by_id = BTreeMap::new();
    for block in &runfile.inputs.hillslopes_block {
        if let Some(unit_system) = block.unit_system.as_deref() {
            let normalized = unit_system.trim().to_ascii_lowercase();
            if normalized != "m" && normalized != "metric" {
                return Err(format!(
                    "CLIWAT-E-026 hillslopes_block[{id}] unit_system must be 'M' or 'metric', observed '{}'",
                    unit_system,
                    id = block.hillslope_id
                ));
            }
        }

        if let Some(use_existing_pass_file) = block.use_existing_pass_file
            && !use_existing_pass_file
        {
            return Err(format!(
                "CLIWAT-E-026 hillslopes_block[{id}] use_existing_pass_file=false is unsupported",
                id = block.hillslope_id
            ));
        }

        let pass_file_path = resolve_required_runfile_path(
            run_file_path,
            &block.pass_file,
            "inputs.hillslopes_block[].pass_file",
        )?;
        if !pass_file_path.is_file() {
            return Err(format!(
                "CLIWAT-E-027 hillslopes_block[{id}] pass file '{}' is not a readable file",
                pass_file_path.display(),
                id = block.hillslope_id
            ));
        }

        if hillslope_blocks_by_id
            .insert(
                block.hillslope_id,
                WatershedHillslopeBlockResolved { pass_file_path },
            )
            .is_some()
        {
            return Err(format!(
                "CLIWAT-E-028 duplicate hillslope_id {} in inputs.hillslopes_block",
                block.hillslope_id
            ));
        }
    }

    let mut runfile_warnings = Vec::new();
    let chaninp_path = if legacy_sidecar_discovery {
        let discovered = run_dir.join("chan.inp");
        if runfile.inputs.chaninp.is_some() {
            runfile_warnings.push(
                "legacy-sidecar-discovery is active; ignoring configured inputs.chaninp and probing run_dir/chan.inp".to_string(),
            );
        }
        if discovered.is_file() {
            Some(discovered)
        } else {
            None
        }
    } else {
        let configured = resolve_optional_runfile_path(
            run_file_path,
            runfile.inputs.chaninp.as_deref(),
            "inputs.chaninp",
        )?;

        if let Some(path) = configured.as_ref()
            && !path.is_file()
        {
            return Err(format!(
                "CLIWAT-E-029 configured inputs.chaninp path '{}' is not a readable file",
                path.display()
            ));
        }

        configured
    };

    let tcr_overlay_present = if legacy_sidecar_discovery {
        let discovered = run_dir.join("tcr.txt");
        if runfile.inputs.tcr.is_some() {
            runfile_warnings.push(
                "legacy-sidecar-discovery is active; ignoring configured inputs.tcr and probing run_dir/tcr.txt".to_string(),
            );
        }
        discovered.is_file()
    } else {
        let configured = resolve_optional_runfile_path(
            run_file_path,
            runfile.inputs.tcr.as_deref(),
            "inputs.tcr",
        )?;
        if let Some(path) = configured.as_ref()
            && !path.is_file()
        {
            return Err(format!(
                "CLIWAT-E-029 configured inputs.tcr path '{}' is not a readable file",
                path.display()
            ));
        }
        configured.is_some()
    };

    let outputs = WatershedOutputsResolved {
        ebe_pw0: resolve_required_output_path(
            output_dir,
            &runfile.outputs.ebe_pw0,
            "outputs.ebe_pw0",
        )?,
        chan_out: resolve_required_output_path(
            output_dir,
            &runfile.outputs.chan_out,
            "outputs.chan_out",
        )?,
        chanwb: resolve_required_output_path(
            output_dir,
            &runfile.outputs.chanwb,
            "outputs.chanwb",
        )?,
        chnwb: resolve_required_output_path(output_dir, &runfile.outputs.chnwb, "outputs.chnwb")?,
        soil_pw0: resolve_required_output_path(
            output_dir,
            &runfile.outputs.soil_pw0,
            "outputs.soil_pw0",
        )?,
        totalwatsed3: resolve_required_output_path(
            output_dir,
            &runfile.outputs.totalwatsed3,
            "outputs.totalwatsed3",
        )?,
        loss_hill: resolve_required_output_path(
            output_dir,
            &runfile.outputs.loss_hill,
            "outputs.loss_hill",
        )?,
        loss_chn: resolve_required_output_path(
            output_dir,
            &runfile.outputs.loss_chn,
            "outputs.loss_chn",
        )?,
        loss_out: resolve_required_output_path(
            output_dir,
            &runfile.outputs.loss_out,
            "outputs.loss_out",
        )?,
        loss_class_data: resolve_required_output_path(
            output_dir,
            &runfile.outputs.loss_class_data,
            "outputs.loss_class_data",
        )?,
        loss_all_years_hill: resolve_required_output_path(
            output_dir,
            &runfile.outputs.loss_all_years_hill,
            "outputs.loss_all_years_hill",
        )?,
        loss_all_years_chn: resolve_required_output_path(
            output_dir,
            &runfile.outputs.loss_all_years_chn,
            "outputs.loss_all_years_chn",
        )?,
        loss_all_years_out: resolve_required_output_path(
            output_dir,
            &runfile.outputs.loss_all_years_out,
            "outputs.loss_all_years_out",
        )?,
        loss_all_years_class_data: resolve_required_output_path(
            output_dir,
            &runfile.outputs.loss_all_years_class_data,
            "outputs.loss_all_years_class_data",
        )?,
    };
    validate_output_contract(&outputs)
        .map_err(|error| format!("CLIWAT-E-034 invalid watershed output contract: {error}"))?;

    Ok(WatershedRunfileResolved {
        watershed_structure_path,
        watershed_channel_path,
        watershed_impoundment_path,
        chaninp_path,
        tcr_overlay_present,
        hillslope_blocks_by_id,
        runfile_warnings,
        outputs,
    })
}

fn logical_watershed_structure_line_count(path: &Path) -> Result<usize, std::io::Error> {
    let content = fs::read_to_string(path)?;
    Ok(content
        .lines()
        .map(str::trim)
        .filter(|line| !line.is_empty() && !line.starts_with('#'))
        .count())
}

fn contributor_hillslope_ids(topology: &TopologyGraph) -> BTreeSet<u32> {
    let mut ids = BTreeSet::new();
    for edge in topology.edges() {
        if edge.from.kind == TopologyNodeKind::Hillslope && edge.from.id > 0 {
            ids.insert(edge.from.id);
        }
    }
    ids
}

#[allow(clippy::too_many_lines)]
fn build_topology_from_watershed_structure(
    structure: &WatershedStructureFile,
) -> Result<TopologyGraph, String> {
    let mut element_lookup: BTreeMap<i32, TopologyNodeKey> = BTreeMap::new();
    for row in &structure.rows {
        let kind = if row.element_type_code == 2 {
            TopologyNodeKind::Channel
        } else {
            TopologyNodeKind::Impoundment
        };
        element_lookup.insert(
            row.element_id,
            TopologyNodeKey::new(
                kind,
                u32::try_from(row.element_local_index).map_err(|_| {
                    format!(
                        "CLIWAT-E-030 element local index out of range at structure row {}",
                        row.record_index
                    )
                })?,
            ),
        );
    }

    let mut nodes = Vec::with_capacity(structure.rows.len());
    for row in &structure.rows {
        let node_kind = if row.element_type_code == 2 {
            TopologyNodeKind::Channel
        } else {
            TopologyNodeKind::Impoundment
        };
        let node_id = u32::try_from(row.element_local_index).map_err(|_| {
            format!(
                "CLIWAT-E-030 element local index out of range at structure row {}",
                row.record_index
            )
        })?;

        let contributors = TopologyContributors::new(
            ContributorTriplet::new(
                u32::try_from(row.nhleft.max(0)).map_err(|_| {
                    format!(
                        "CLIWAT-E-031 hillslope contributor conversion failed at row {}",
                        row.record_index
                    )
                })?,
                u32::try_from(row.nhrght.max(0)).map_err(|_| {
                    format!(
                        "CLIWAT-E-031 hillslope contributor conversion failed at row {}",
                        row.record_index
                    )
                })?,
                u32::try_from(row.nhtop.max(0)).map_err(|_| {
                    format!(
                        "CLIWAT-E-031 hillslope contributor conversion failed at row {}",
                        row.record_index
                    )
                })?,
            ),
            ContributorTriplet::new(
                resolve_structure_contributor_local_id(
                    row.ncleft,
                    TopologyNodeKind::Channel,
                    &element_lookup,
                )?,
                resolve_structure_contributor_local_id(
                    row.ncrght,
                    TopologyNodeKind::Channel,
                    &element_lookup,
                )?,
                resolve_structure_contributor_local_id(
                    row.nctop,
                    TopologyNodeKind::Channel,
                    &element_lookup,
                )?,
            ),
            ContributorTriplet::new(
                resolve_structure_contributor_local_id(
                    row.nileft,
                    TopologyNodeKind::Impoundment,
                    &element_lookup,
                )?,
                resolve_structure_contributor_local_id(
                    row.nirght,
                    TopologyNodeKind::Impoundment,
                    &element_lookup,
                )?,
                resolve_structure_contributor_local_id(
                    row.nitop,
                    TopologyNodeKind::Impoundment,
                    &element_lookup,
                )?,
            ),
        );

        nodes.push(TopologyNode::new(
            TopologyNodeKey::new(node_kind, node_id),
            contributors,
        ));
    }

    Ok(TopologyGraph::new(
        u32::try_from(structure.nhill)
            .map_err(|_| "CLIWAT-E-032 structure nhill out of range".to_string())?,
        u32::try_from(structure.summary.channel_count)
            .map_err(|_| "CLIWAT-E-032 structure channel_count out of range".to_string())?,
        u32::try_from(structure.summary.impoundment_count)
            .map_err(|_| "CLIWAT-E-032 structure impoundment_count out of range".to_string())?,
        nodes,
    ))
}

fn resolve_structure_contributor_local_id(
    contributor_element_id: i32,
    expected_kind: TopologyNodeKind,
    element_lookup: &BTreeMap<i32, TopologyNodeKey>,
) -> Result<u32, String> {
    if contributor_element_id == 0 {
        return Ok(0);
    }

    let Some(node_key) = element_lookup.get(&contributor_element_id) else {
        return Err(format!(
            "CLIWAT-E-033 unresolved contributor element id {contributor_element_id}"
        ));
    };

    if node_key.kind != expected_kind {
        return Err(format!(
            "CLIWAT-E-033 contributor element id {} maps to kind '{}' but expected '{}'",
            contributor_element_id,
            node_key.kind.as_str(),
            expected_kind.as_str()
        ));
    }

    Ok(node_key.id)
}

fn build_default_chaninp_surface(
    watershed_channel: &openwepp_input_contract::parsers::watershed_channel::WatershedChannelFile,
) -> WatershedWritebackSurface {
    let mut state_surface = BTreeMap::new();
    state_surface.insert(
        BoundarySymbol::from(WatershedProductionStateSymbol::Ipeak),
        BoundaryValue::scalar(f64::from(watershed_channel.ipeak)),
    );
    state_surface.insert(
        BoundarySymbol::from("nchan"),
        BoundaryValue::scalar(f64::from(
            u32::try_from(watershed_channel.nchan).unwrap_or(u32::MAX),
        )),
    );
    state_surface.insert(
        BoundarySymbol::from("dtchr"),
        BoundaryValue::scalar(DEFAULT_DTCHR_SECONDS),
    );
    state_surface.insert(
        BoundarySymbol::from("ntchr"),
        BoundaryValue::scalar(DEFAULT_NTCHR),
    );
    state_surface.insert(BoundarySymbol::from("nchnum"), BoundaryValue::scalar(0.0));

    let mut flux_surface = BTreeMap::new();
    flux_surface.insert(BoundarySymbol::from("cbase"), BoundaryValue::scalar(0.0));

    WatershedWritebackSurface {
        state_surface,
        flux_surface,
    }
}

fn write_watershed_interchange_outputs(outputs: &WatershedOutputsResolved) -> Result<(), String> {
    write_interchange_parquet_outputs(outputs)
        .map_err(|error| format!("CLIWAT-E-034 watershed output writer failure: {error}"))
}

fn print_help() {
    println!(
        "openwepp-cli-watershed --run-dir <path> --run-file <path> --output-dir <path> [--policy strict|compat] [--legacy-sidecar-discovery]"
    );
}
