use std::collections::{BTreeMap, BTreeSet};
use std::fs;
use std::path::{Path, PathBuf};
use std::time::Instant;

use openwepp_input_contract::parsers::chaninp::{ChaninpParseOptions, parse_chaninp_from_path};
use openwepp_input_contract::parsers::slope::{SlopeParserOptions, parse_slope_file};
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
use openwepp_legacy_bridge::sidecar::{
    SidecarAdapterRequest, SidecarBinding, SidecarContract, SidecarDiscovery, SidecarId,
    SidecarRequirement, adapt_sidecar_bindings,
};
use openwepp_runner::{
    HillslopeJob, HillslopeWorkerPoolReport, PassInventoryExpectation, SidecarPolicy,
    WatershedRunPlan,
};
use openwepp_topology::{
    ContributorTriplet, TopologyContributors, TopologyGraph, TopologyNode, TopologyNodeKey,
    TopologyNodeKind, validate_pre_execution_topology,
};
use openwepp_watershed_orchestrator::{
    HillslopeContribution, WatershedNetworkFrame, WatershedPublicationFrame,
    execute_watershed_dispatch_with_frame,
};
use openwepp_watershed_output::contracts::{WatershedOutputConfig, validate_output_contract};
use openwepp_watershed_output::writers::{
    WatershedInterchangeRowSeed, write_interchange_parquet_outputs,
    write_interchange_parquet_outputs_from_rows,
};
use serde::Deserialize;
use serde_json::{Value, json};

const WATERSHED_RUNFILE_SCHEMA_ID: &str = "openwepp-watershed-runfile-v1";
const HILLSLOPE_RUN_MANIFEST_SCHEMA_ID: &str = "openwepp-hillslope-run-manifest-v1";
const DEFAULT_DTCHR_SECONDS: f64 = 3_600.0;
const DEFAULT_NTCHR: f64 = 24.0;
const MOFE04_PUBLICATION_OFE_POLICY: &str = "single-row-canonicalized-hillslope-aggregate";
const MF_PUBLICATION_OFE_POLICY: &str = "per-ofe-dynamic-water-balance-state";
const MOFE04_PUBLICATION_AREA_POLICY: &str = "sum-ofe-geometry-area";
const MF_STORAGE_LINEAGE_POLICY: &str = "per-ofe-dynamic-wb-state";
const MF_PER_OFE_STATE_POLICY: &str = "published-per-ofe-wb13-records";
const MF_IDENTITY_STATUS: &str = "pass-published-per-ofe-wb13-records";
const MI_HILLSLOPE_TOTAL_IDENTITY_TOLERANCE_MM: f64 = 1.0e-9;
const MOFE_HOURLY_CARRY_POLICY: &str = "baseline-wathour-24-slot-copy-forward";
const MOFE_HOURLY_CARRY_ARRAY_COUNT: u64 = 24;
const MOFE_HOURLY_REQUIRED_ARRAYS: [&str; 4] = ["ui_SUrunf", "ui_SCrunf", "ui_LfUrf", "ui_LfCrf"];

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
    let mut sidecar_policy = SidecarPolicy::Compat;
    let mut legacy_sidecar_discovery = false;
    let mut jobs = 1usize;
    let mut hillslope_binary: Option<PathBuf> = None;

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
                sidecar_policy = value.parse().map_err(|detail: String| {
                    format!("CLIWAT-E-001 invalid --policy value: {detail}")
                })?;
            }
            "--legacy-sidecar-discovery" => {
                legacy_sidecar_discovery = true;
            }
            "--jobs" => {
                cursor += 1;
                let Some(value) = args.get(cursor) else {
                    return Err("CLIWAT-E-001 missing value for --jobs".to_string());
                };
                jobs = parse_jobs_arg(value)?;
            }
            "--hillslope-binary" => {
                cursor += 1;
                let Some(value) = args.get(cursor) else {
                    return Err("CLIWAT-E-001 missing value for --hillslope-binary".to_string());
                };
                hillslope_binary = Some(PathBuf::from(value));
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
    let output_dir = resolve_cli_output_dir(&output_dir)?;

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
    let supervisor_started = Instant::now();

    let run_file_path = resolve_run_file(&run_dir, &run_file);
    if !run_file_path.is_file() {
        return Err(format!(
            "CLIWAT-E-004 run file does not exist: {}",
            run_file_path.display()
        ));
    }

    let runfile = parse_watershed_runfile(
        &run_file_path,
        sidecar_policy,
        legacy_sidecar_discovery,
        &run_dir,
        &output_dir,
    )?;
    let hillslope_binary = match hillslope_binary {
        Some(path) => path,
        None => default_hillslope_binary()?,
    };
    let run_plan =
        build_watershed_run_plan(runfile.run_name.as_str(), jobs, hillslope_binary, &runfile)?;

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
        mode: WatershedStructureParseMode::Compatibility,
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
        mode: WatershedChannelParseMode::Compatibility,
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
    let slope = parse_slope_file(&runfile.slope_path, SlopeParserOptions::compatibility())
        .map_err(|error| {
            format!(
                "CLIWAT-E-038 failed parsing watershed slope {}: {error}",
                runfile.slope_path.display()
            )
        })?;

    let impoundment_options = WatershedImpoundmentParseOptions {
        mode: WatershedImpoundmentParseMode::Compatibility,
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

    let chaninp = if let Some(chaninp_path) = runfile.chaninp_path.as_ref() {
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

        let chaninp_options =
            ChaninpParseOptions::compatibility(watershed_channel.ipeak, watershed_channel.nchan);

        let chaninp =
            parse_chaninp_from_path(chaninp_path, chaninp_options, &valid_channel_element_ids)
                .map_err(|error| {
                    format!(
                        "CLIWAT-E-012 failed parsing chan.inp {}: {error}",
                        chaninp_path.display()
                    )
                })?;
        Some(chaninp)
    } else {
        sidecar_warnings.push(
            "chan.inp sidecar not provided; applying deterministic fallback channel globals (dtchr=3600, ntchr=24, nchnum=0, cbase=0).".to_string(),
        );
        None
    };

    let mut network_frame = WatershedNetworkFrame::from_parsed_inputs(
        topology.clone(),
        chaninp,
        watershed_channel,
        slope,
        watershed_impoundment,
        DEFAULT_DTCHR_SECONDS,
        DEFAULT_NTCHR,
    )
    .map_err(|error| {
        format!("CLIWAT-E-013 failed building typed watershed network frame: {error}")
    })?;

    let contributor_hillslopes = contributor_hillslope_ids(&topology);
    for hillslope_id in &contributor_hillslopes {
        if !runfile.hillslope_blocks_by_id.contains_key(hillslope_id) {
            return Err(format!(
                "CLIWAT-E-016 missing hillslopes_block entry for contributor hillslope id {hillslope_id}"
            ));
        }
    }

    let worker_report =
        run_plan.execute_hillslope_jobs(sidecar_policy, legacy_sidecar_discovery)?;
    let pass_inventory_started = Instant::now();
    let pass_inventory = run_plan.validate_pass_inventory()?;
    let pass_inventory_elapsed_ms = pass_inventory_started.elapsed().as_millis();

    let routing_input_started = Instant::now();
    for entry in pass_inventory.entries() {
        let hillslope_id = entry.hillslope_id;
        let class_count = usize::from(entry.npart);
        validate_contributor_mofe_metadata(
            hillslope_id,
            entry.nofe,
            entry.manifest_file_path.as_deref(),
        )?;

        let payload = &entry.latest_event_payload;
        let peak = payload.peak_runoff_m3_s;
        let duration = payload.duration_seconds;
        let total_detachment = payload.total_detachment_kg;
        let total_deposition = payload.total_deposition_kg;
        let sediment_concentrations = &payload.sediment_concentration_kg_m3;
        let particle_diameters = &payload.particle_diameter_m;
        let particle_flow_fractions = &payload.particle_flow_fraction;
        let mut typed_sediment_concentrations = Vec::with_capacity(class_count);
        let mut typed_particle_diameters = Vec::with_capacity(class_count);
        let mut typed_particle_flow_fractions = Vec::with_capacity(class_count);

        for class_index in 1..=class_count {
            let concentration = sediment_concentrations
                .get(class_index - 1)
                .copied()
                .ok_or_else(|| {
                    format!(
                        "CLIWAT-E-018 missing sediment_concentration_kg_m3 class={class_index} for hillslope {hillslope_id}"
                    )
                })?;
            let particle_diameter = particle_diameters
                .get(class_index - 1)
                .copied()
                .ok_or_else(|| {
                    format!(
                        "CLIWAT-E-018 missing particle_diameter_m class={class_index} for hillslope {hillslope_id}"
                    )
                })?;
            let fraction = particle_flow_fractions
                .get(class_index - 1)
                .copied()
                .ok_or_else(|| {
                    format!(
                        "CLIWAT-E-018 missing particle_flow_fraction class={class_index} for hillslope {hillslope_id}"
                    )
                })?;
            typed_sediment_concentrations.push(concentration);
            typed_particle_diameters.push(particle_diameter);
            typed_particle_flow_fractions.push(fraction);
        }

        network_frame.add_hillslope_contribution(HillslopeContribution {
            hillslope_id,
            peak_runoff_m3_s: peak,
            duration_seconds: duration,
            total_detachment_kg: total_detachment,
            total_deposition_kg: total_deposition,
            sediment_concentration_kg_m3: typed_sediment_concentrations,
            particle_diameter_m: typed_particle_diameters,
            particle_flow_fraction: typed_particle_flow_fractions,
        });
    }
    let routing_input_elapsed_ms = routing_input_started.elapsed().as_millis();

    let watershed_dispatch_started = Instant::now();
    let report = execute_watershed_dispatch_with_frame(&mut network_frame, &topology_validation)
        .map_err(|error| format!("CLIWAT-E-019 watershed execution failed: {error}"))?;
    let watershed_dispatch_elapsed_ms = watershed_dispatch_started.elapsed().as_millis();

    if !report.dispatch_report.is_success() {
        return Err(format!(
            "CLIWAT-E-020 watershed dispatch reported failure (message_id={})",
            report.dispatch_report.dispatch_status.message_id()
        ));
    }

    for warning in &sidecar_warnings {
        eprintln!("sidecar-warning: {warning}");
    }

    let publication_frame = network_frame
        .publish_typed_routing_report(&report)
        .map_err(|error| format!("CLIWAT-E-019 watershed publication failed: {error}"))?;
    let row_seed = publication_frame_to_row_seed(&publication_frame);
    let output_publication_started = Instant::now();
    write_watershed_interchange_outputs(&runfile.outputs, &[row_seed])?;
    let output_publication_elapsed_ms = output_publication_started.elapsed().as_millis();
    write_watershed_supervisor_timing(
        &output_dir,
        runfile.run_name.as_str(),
        sidecar_policy,
        legacy_sidecar_discovery,
        &worker_report,
        WatershedSupervisorTiming {
            pass_inventory: pass_inventory_elapsed_ms,
            routing_input: routing_input_elapsed_ms,
            watershed_dispatch: watershed_dispatch_elapsed_ms,
            output_publication: output_publication_elapsed_ms,
            total: supervisor_started.elapsed().as_millis(),
        },
    )?;

    Ok(())
}

#[derive(Debug, Clone, Copy)]
struct WatershedSupervisorTiming {
    pass_inventory: u128,
    routing_input: u128,
    watershed_dispatch: u128,
    output_publication: u128,
    total: u128,
}

fn write_watershed_supervisor_timing(
    output_dir: &Path,
    run_name: &str,
    sidecar_policy: SidecarPolicy,
    legacy_sidecar_discovery: bool,
    worker_report: &HillslopeWorkerPoolReport,
    timing: WatershedSupervisorTiming,
) -> Result<(), String> {
    let path = output_dir.join("watershed-supervisor.timing.json");
    let payload = json!({
        "schema": "openwepp-watershed-supervisor-timing-v1",
        "run_name": run_name,
        "sidecar_policy": format!("{sidecar_policy:?}"),
        "legacy_sidecar_discovery": legacy_sidecar_discovery,
        "worker_pool": {
            "requested_jobs": worker_report.requested_jobs,
            "worker_count": worker_report.worker_count,
            "launched_jobs": worker_report.launched_jobs,
            "completed_jobs": worker_report.completed_jobs,
            "skipped_jobs": worker_report.skipped_jobs,
            "elapsed_ms": worker_report.elapsed_ms,
        },
        "pass_inventory_elapsed_ms": timing.pass_inventory,
        "routing_input_elapsed_ms": timing.routing_input,
        "watershed_dispatch_elapsed_ms": timing.watershed_dispatch,
        "output_publication_elapsed_ms": timing.output_publication,
        "total_elapsed_ms": timing.total,
    });
    let text = serde_json::to_string_pretty(&payload).map_err(|error| {
        format!("CLIWAT-E-034 failed serializing watershed supervisor timing: {error}")
    })?;
    fs::write(&path, text).map_err(|error| {
        format!(
            "CLIWAT-E-034 failed writing watershed supervisor timing {}: {error}",
            path.display()
        )
    })
}

fn resolve_run_file(run_dir: &Path, run_file: &Path) -> PathBuf {
    if run_file.is_absolute() {
        run_file.to_path_buf()
    } else {
        run_dir.join(run_file)
    }
}

fn resolve_cli_output_dir(output_dir: &Path) -> Result<PathBuf, String> {
    if output_dir.is_absolute() {
        Ok(output_dir.to_path_buf())
    } else {
        let current_dir = std::env::current_dir()
            .map_err(|error| format!("CLIWAT-E-003 failed resolving current directory: {error}"))?;
        Ok(current_dir.join(output_dir))
    }
}

fn parse_jobs_arg(value: &str) -> Result<usize, String> {
    let jobs = value
        .parse::<usize>()
        .map_err(|_| format!("CLIWAT-E-041 invalid --jobs value '{value}'"))?;
    if jobs == 0 {
        return Err("CLIWAT-E-041 --jobs must be greater than zero".to_string());
    }
    Ok(jobs)
}

fn default_hillslope_binary() -> Result<PathBuf, String> {
    let current_exe = std::env::current_exe()
        .map_err(|error| format!("CLIWAT-E-042 failed resolving current executable: {error}"))?;
    let binary_name = if cfg!(windows) {
        "openwepp-cli-hill.exe"
    } else {
        "openwepp-cli-hill"
    };
    let Some(parent) = current_exe.parent() else {
        return Err(format!(
            "CLIWAT-E-042 current executable has no parent directory: {}",
            current_exe.display()
        ));
    };
    Ok(parent.join(binary_name))
}

fn generated_hillslope_output_root(output_dir: &Path, hillslope_id: u32) -> PathBuf {
    output_dir
        .join("hillslope-jobs")
        .join(format!("H{hillslope_id}"))
}

fn build_watershed_run_plan(
    run_id: &str,
    jobs: usize,
    hillslope_binary: PathBuf,
    runfile: &WatershedRunfileResolved,
) -> Result<WatershedRunPlan, String> {
    let mut hillslope_jobs = Vec::new();
    let mut expected_passes = Vec::new();
    for (hillslope_id, block) in &runfile.hillslope_blocks_by_id {
        if !block.use_existing_pass_file {
            let Some(source_run_file_path) = block.run_file_path.as_ref() else {
                return Err(format!(
                    "CLIWAT-E-042 generated hillslope block {hillslope_id} is missing run_file"
                ));
            };
            let output_root = block
                .pass_file_path
                .parent()
                .map_or_else(|| PathBuf::from("."), Path::to_path_buf);
            let Some(expected_manifest_file) = block.manifest_file_path.clone() else {
                return Err(format!(
                    "CLIWAT-E-042 generated hillslope block {hillslope_id} is missing manifest path"
                ));
            };
            hillslope_jobs.push(HillslopeJob {
                hillslope_id: *hillslope_id,
                source_run_file_path: source_run_file_path.clone(),
                generated_run_file_path: output_root.join(format!("H{hillslope_id}.run.toml")),
                output_root: output_root.clone(),
                expected_pass_file: block.pass_file_path.clone(),
                expected_manifest_file,
                stdout_log_path: output_root.join(format!("H{hillslope_id}.stdout.log")),
                stderr_log_path: output_root.join(format!("H{hillslope_id}.stderr.log")),
                timing_path: output_root.join(format!("H{hillslope_id}.timing.json")),
                freshness_marker_path: output_root.join(format!("H{hillslope_id}.freshness")),
            });
        }
        let generated_output_root = generated_hillslope_output_root_for_pass(&block.pass_file_path);
        expected_passes.push(PassInventoryExpectation {
            hillslope_id: *hillslope_id,
            pass_file_path: block.pass_file_path.clone(),
            manifest_file_path: block.manifest_file_path.clone(),
            produced_by_job: !block.use_existing_pass_file,
            freshness_marker_path: (!block.use_existing_pass_file)
                .then(|| generated_output_root.join(format!("H{hillslope_id}.freshness"))),
            timing_path: (!block.use_existing_pass_file)
                .then(|| generated_output_root.join(format!("H{hillslope_id}.timing.json"))),
        });
    }

    WatershedRunPlan::new(
        run_id.to_string(),
        jobs,
        hillslope_binary,
        hillslope_jobs,
        expected_passes,
    )
}

fn generated_hillslope_output_root_for_pass(pass_file_path: &Path) -> PathBuf {
    pass_file_path
        .parent()
        .map_or_else(|| PathBuf::from("."), Path::to_path_buf)
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
    #[serde(default)]
    applicability: WatershedRunfileApplicability,
    chaninp: Option<String>,
    tcr: Option<String>,
}

#[derive(Debug, Deserialize, Default)]
struct WatershedRunfileApplicability {
    #[serde(default)]
    chapter13_small_watershed_intent: Option<bool>,
    #[serde(default)]
    allow_partial_area_response: Option<bool>,
    #[serde(default)]
    allow_headcutting: Option<bool>,
    #[serde(default)]
    allow_bank_sloughing: Option<bool>,
    #[serde(default)]
    allow_perennial_streams: Option<bool>,
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
    #[serde(default)]
    pass_file: Option<String>,
    #[serde(default)]
    manifest_file: Option<String>,
    #[serde(default)]
    run_file: Option<String>,
    #[serde(default)]
    unit_system: Option<String>,
    #[serde(default)]
    use_existing_pass_file: Option<bool>,
}

#[derive(Debug)]
struct WatershedHillslopeBlockResolved {
    pass_file_path: PathBuf,
    manifest_file_path: Option<PathBuf>,
    run_file_path: Option<PathBuf>,
    use_existing_pass_file: bool,
}

type WatershedOutputsResolved = WatershedOutputConfig;

#[derive(Debug)]
struct WatershedRunfileResolved {
    run_name: String,
    watershed_structure_path: PathBuf,
    watershed_channel_path: PathBuf,
    watershed_impoundment_path: PathBuf,
    slope_path: PathBuf,
    chaninp_path: Option<PathBuf>,
    tcr_overlay_present: bool,
    hillslope_blocks_by_id: BTreeMap<u32, WatershedHillslopeBlockResolved>,
    runfile_warnings: Vec<String>,
    outputs: WatershedOutputsResolved,
}

#[allow(clippy::too_many_lines)]
fn parse_watershed_runfile(
    run_file_path: &Path,
    sidecar_policy: SidecarPolicy,
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
    validate_watershed_runfile_applicability(&runfile.inputs.applicability)?;

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

        let Some(use_existing_pass_file) = block.use_existing_pass_file else {
            return Err(format!(
                "CLIWAT-E-026 hillslopes_block[{id}] use_existing_pass_file must be explicit",
                id = block.hillslope_id
            ));
        };
        let hillslope_run_file_path = resolve_optional_runfile_path(
            run_file_path,
            block.run_file.as_deref(),
            "inputs.hillslopes_block[].run_file",
        )?;

        let (pass_file_path, manifest_file_path) = if use_existing_pass_file {
            if hillslope_run_file_path.is_some() {
                return Err(format!(
                    "CLIWAT-E-026 hillslopes_block[{id}] cannot combine run_file with use_existing_pass_file=true",
                    id = block.hillslope_id
                ));
            }
            let Some(pass_file) = block.pass_file.as_deref() else {
                return Err(format!(
                    "CLIWAT-E-026 hillslopes_block[{id}] pass_file is required when use_existing_pass_file=true",
                    id = block.hillslope_id
                ));
            };
            let pass_file_path = resolve_required_runfile_path(
                run_file_path,
                pass_file,
                "inputs.hillslopes_block[].pass_file",
            )?;
            if !pass_file_path.is_file() {
                return Err(format!(
                    "CLIWAT-E-027 hillslopes_block[{id}] pass file '{}' is not a readable file",
                    pass_file_path.display(),
                    id = block.hillslope_id
                ));
            }
            let manifest_file_path = resolve_optional_runfile_path(
                run_file_path,
                block.manifest_file.as_deref(),
                "inputs.hillslopes_block[].manifest_file",
            )?;
            if let Some(path) = manifest_file_path.as_ref()
                && !path.is_file()
            {
                return Err(format!(
                    "CLIWAT-E-036 hillslopes_block[{id}] manifest file '{}' is not a readable file",
                    path.display(),
                    id = block.hillslope_id
                ));
            }
            (pass_file_path, manifest_file_path)
        } else {
            let Some(hillslope_run_file_path) = hillslope_run_file_path.as_ref() else {
                return Err(format!(
                    "CLIWAT-E-026 hillslopes_block[{id}] run_file is required when use_existing_pass_file=false",
                    id = block.hillslope_id
                ));
            };
            if !hillslope_run_file_path.is_file() {
                return Err(format!(
                    "CLIWAT-E-026 hillslopes_block[{id}] run_file '{}' is not a readable file",
                    hillslope_run_file_path.display(),
                    id = block.hillslope_id
                ));
            }
            if block.pass_file.is_some() || block.manifest_file.is_some() {
                return Err(format!(
                    "CLIWAT-E-026 hillslopes_block[{id}] generated hillslope mode lets the supervisor own pass_file and manifest_file paths",
                    id = block.hillslope_id
                ));
            }
            let output_root = generated_hillslope_output_root(output_dir, block.hillslope_id);
            (
                output_root.join(format!("H{}.hbp", block.hillslope_id)),
                Some(output_root.join(format!("H{}.manifest.json", block.hillslope_id))),
            )
        };

        if hillslope_blocks_by_id
            .insert(
                block.hillslope_id,
                WatershedHillslopeBlockResolved {
                    pass_file_path,
                    manifest_file_path,
                    run_file_path: hillslope_run_file_path,
                    use_existing_pass_file,
                },
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
    let (chaninp_path, tcr_overlay_present) = if legacy_sidecar_discovery {
        if runfile.inputs.chaninp.is_some() {
            runfile_warnings.push(
                "legacy-sidecar-discovery is active; ignoring configured inputs.chaninp and probing run_dir/chan.inp".to_string(),
            );
        }
        if runfile.inputs.tcr.is_some() {
            runfile_warnings.push(
                "legacy-sidecar-discovery is active; ignoring configured inputs.tcr and probing run_dir/tcr.txt".to_string(),
            );
        }

        let mut excluded_files = vec![
            file_name_string(run_file_path),
            file_name_string(&watershed_structure_path),
            file_name_string(&watershed_channel_path),
            file_name_string(&watershed_impoundment_path),
            file_name_string(&management_path),
            file_name_string(&slope_path),
            file_name_string(&climate_path),
            file_name_string(&soil_path),
        ];
        excluded_files.extend(
            hillslope_blocks_by_id
                .values()
                .map(|block| file_name_string(&block.pass_file_path)),
        );
        excluded_files.extend(
            hillslope_blocks_by_id
                .values()
                .filter_map(|block| block.manifest_file_path.as_ref())
                .map(|path| file_name_string(path)),
        );
        excluded_files.extend(watershed_output_file_names(&runfile.outputs));

        let discovered_sidecars = discover_sidecars(run_dir, &excluded_files)?;
        let sidecar_contracts = watershed_sidecar_contracts(true)?;
        let sidecar_response = adapt_sidecar_bindings(&SidecarAdapterRequest {
            policy: sidecar_policy.as_legacy_bridge_policy(),
            contracts: sidecar_contracts,
            discovered: discovered_sidecars,
        })
        .map_err(|error| format!("CLIWAT-E-035 sidecar adaptation failed: {error}"))?;

        runfile_warnings.extend(
            sidecar_response
                .warnings
                .iter()
                .map(|warning| format!("{} {}", warning.code.message_id(), warning.detail)),
        );

        let chaninp_path = optional_sidecar_binding_path(&sidecar_response.bindings, "chaninp")
            .unwrap_or_else(|| run_dir.join("chan.inp"));
        let tcr_path = optional_sidecar_binding_path(&sidecar_response.bindings, "tcr")
            .unwrap_or_else(|| run_dir.join("tcr.txt"));

        let chaninp_path = if chaninp_path.is_file() {
            Some(chaninp_path)
        } else {
            None
        };
        (chaninp_path, tcr_path.is_file())
    } else {
        let configured_chaninp = resolve_optional_runfile_path(
            run_file_path,
            runfile.inputs.chaninp.as_deref(),
            "inputs.chaninp",
        )?;
        if let Some(path) = configured_chaninp.as_ref()
            && !path.is_file()
        {
            return Err(format!(
                "CLIWAT-E-029 configured inputs.chaninp path '{}' is not a readable file",
                path.display()
            ));
        }

        let configured_tcr = resolve_optional_runfile_path(
            run_file_path,
            runfile.inputs.tcr.as_deref(),
            "inputs.tcr",
        )?;
        if let Some(path) = configured_tcr.as_ref()
            && !path.is_file()
        {
            return Err(format!(
                "CLIWAT-E-029 configured inputs.tcr path '{}' is not a readable file",
                path.display()
            ));
        }

        (configured_chaninp, configured_tcr.is_some())
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
        run_name: runfile.run_name,
        watershed_structure_path,
        watershed_channel_path,
        watershed_impoundment_path,
        slope_path,
        chaninp_path,
        tcr_overlay_present,
        hillslope_blocks_by_id,
        runfile_warnings,
        outputs,
    })
}

fn validate_watershed_runfile_applicability(
    applicability: &WatershedRunfileApplicability,
) -> Result<(), String> {
    let Some(chapter13_small_watershed_intent) = applicability.chapter13_small_watershed_intent
    else {
        return Err(
            "CLIWAT-E-040 missing required inputs.applicability.chapter13_small_watershed_intent"
                .to_string(),
        );
    };
    if !chapter13_small_watershed_intent {
        return Err("CLIWAT-E-040 inputs.applicability.chapter13_small_watershed_intent must be true to enforce Chapter-13 small-watershed intent".to_string());
    }

    let Some(allow_partial_area_response) = applicability.allow_partial_area_response else {
        return Err(
            "CLIWAT-E-040 missing required inputs.applicability.allow_partial_area_response"
                .to_string(),
        );
    };
    if allow_partial_area_response {
        return Err("CLIWAT-E-040 inputs.applicability.allow_partial_area_response must be false (Chapter-13 excludes partial-area response)".to_string());
    }

    let Some(allow_headcutting) = applicability.allow_headcutting else {
        return Err(
            "CLIWAT-E-040 missing required inputs.applicability.allow_headcutting".to_string(),
        );
    };
    if allow_headcutting {
        return Err("CLIWAT-E-040 inputs.applicability.allow_headcutting must be false (Chapter-13 excludes headcutting)".to_string());
    }

    let Some(allow_bank_sloughing) = applicability.allow_bank_sloughing else {
        return Err(
            "CLIWAT-E-040 missing required inputs.applicability.allow_bank_sloughing".to_string(),
        );
    };
    if allow_bank_sloughing {
        return Err("CLIWAT-E-040 inputs.applicability.allow_bank_sloughing must be false (Chapter-13 excludes bank sloughing)".to_string());
    }

    let Some(allow_perennial_streams) = applicability.allow_perennial_streams else {
        return Err(
            "CLIWAT-E-040 missing required inputs.applicability.allow_perennial_streams"
                .to_string(),
        );
    };
    if allow_perennial_streams {
        return Err("CLIWAT-E-040 inputs.applicability.allow_perennial_streams must be false (Chapter-13 excludes perennial streams)".to_string());
    }

    Ok(())
}

fn logical_watershed_structure_line_count(path: &Path) -> Result<usize, std::io::Error> {
    let content = fs::read_to_string(path)?;
    Ok(content
        .lines()
        .map(str::trim)
        .filter(|line| !line.is_empty() && !line.starts_with('#'))
        .count())
}

fn file_name_string(path: &Path) -> String {
    path.file_name()
        .and_then(|value| value.to_str())
        .unwrap_or("")
        .to_string()
}

fn path_has_extension_case_insensitive(path: &Path, extension: &str) -> bool {
    path.extension()
        .and_then(|value| value.to_str())
        .is_some_and(|value| value.eq_ignore_ascii_case(extension))
}

fn discover_sidecars(
    run_dir: &Path,
    excluded_file_names: &[String],
) -> Result<Vec<SidecarDiscovery>, String> {
    let mut discoveries = Vec::new();
    let entries = fs::read_dir(run_dir)
        .map_err(|error| format!("CLIWAT-E-035 failed reading run directory sidecars: {error}"))?;

    for entry_result in entries {
        let entry = entry_result.map_err(|error| {
            format!("CLIWAT-E-035 failed scanning run directory sidecars: {error}")
        })?;
        let path = entry.path();
        if !path.is_file() {
            continue;
        }

        let file_name = file_name_string(&path);
        if excluded_file_names
            .iter()
            .any(|excluded| excluded == &file_name)
        {
            continue;
        }
        if path_has_extension_case_insensitive(&path, "hbp") {
            continue;
        }

        discoveries.push(SidecarDiscovery::new(file_name, path));
    }

    discoveries.sort_by(|left, right| left.file_name.cmp(&right.file_name));
    Ok(discoveries)
}

fn watershed_sidecar_contracts(
    legacy_optional_core_sidecars: bool,
) -> Result<Vec<SidecarContract>, String> {
    let core_sidecars = [
        ("frost", "frost.txt"),
        ("snow", "snow.txt"),
        ("wepp_ui", "wepp_ui.txt"),
        ("pmetpara", "pmetpara.txt"),
    ];

    let optional = [
        ("irrigation_depletion", "irrigation_depletion.txt"),
        ("irrigation_fixeddate", "irrigation_fixeddate.ifd"),
        ("gwcoeff", "gwcoeff.txt"),
        ("phosphorus", "phosphorus.txt"),
        ("tc", "tc.txt"),
        ("tcr", "tcr.txt"),
        ("lcwb", "lcwb.txt"),
        ("chaninp", "chan.inp"),
    ];

    let mut contracts = Vec::new();
    for (id, file_name) in core_sidecars {
        let requirement = if legacy_optional_core_sidecars {
            SidecarRequirement::Optional
        } else {
            SidecarRequirement::Required
        };
        contracts.push(build_sidecar_contract(id, file_name, requirement)?);
    }
    for (id, file_name) in optional {
        contracts.push(build_sidecar_contract(
            id,
            file_name,
            SidecarRequirement::Optional,
        )?);
    }

    Ok(contracts)
}

fn build_sidecar_contract(
    id: &'static str,
    file_name: &'static str,
    requirement: SidecarRequirement,
) -> Result<SidecarContract, String> {
    let sidecar_id =
        SidecarId::new(id).map_err(|error| format!("CLIWAT-E-035 invalid sidecar id: {error}"))?;
    Ok(SidecarContract::new(
        sidecar_id,
        file_name,
        Vec::new(),
        requirement,
    ))
}

fn optional_sidecar_binding_path(
    bindings: &[SidecarBinding],
    sidecar_id: &'static str,
) -> Option<PathBuf> {
    bindings
        .iter()
        .find(|binding| binding.sidecar_id.as_str() == sidecar_id)
        .map(|binding| binding.resolved_path.clone())
}

fn watershed_output_file_names(outputs: &WatershedRunfileOutputs) -> Vec<String> {
    [
        outputs.ebe_pw0.as_str(),
        outputs.chan_out.as_str(),
        outputs.chanwb.as_str(),
        outputs.chnwb.as_str(),
        outputs.soil_pw0.as_str(),
        outputs.totalwatsed3.as_str(),
        outputs.loss_hill.as_str(),
        outputs.loss_chn.as_str(),
        outputs.loss_out.as_str(),
        outputs.loss_class_data.as_str(),
        outputs.loss_all_years_hill.as_str(),
        outputs.loss_all_years_chn.as_str(),
        outputs.loss_all_years_out.as_str(),
        outputs.loss_all_years_class_data.as_str(),
    ]
    .iter()
    .map(|path| file_name_string(Path::new(path.trim())))
    .filter(|name| !name.is_empty())
    .collect()
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

fn validate_contributor_mofe_metadata(
    hillslope_id: u32,
    contributor_nofe: u16,
    manifest_file_path: Option<&Path>,
) -> Result<(), String> {
    if contributor_nofe > 1 {
        let Some(path) = manifest_file_path else {
            return Err(format!(
                "CLIWAT-E-036 hillslope {hillslope_id} requires inputs.hillslopes_block[].manifest_file when pass nofe={contributor_nofe} > 1"
            ));
        };
        validate_manifest_publication_metadata(hillslope_id, contributor_nofe, path)?;
    } else if let Some(path) = manifest_file_path {
        validate_manifest_publication_metadata(hillslope_id, contributor_nofe, path)?;
    }

    Ok(())
}

#[allow(clippy::too_many_lines)]
fn validate_manifest_publication_metadata(
    hillslope_id: u32,
    contributor_nofe: u16,
    manifest_file_path: &Path,
) -> Result<(), String> {
    let manifest_text = fs::read_to_string(manifest_file_path).map_err(|error| {
        format!(
            "CLIWAT-E-036 failed reading hillslope {hillslope_id} manifest_file '{}': {error}",
            manifest_file_path.display()
        )
    })?;
    let manifest: Value = serde_json::from_str(&manifest_text).map_err(|error| {
        format!(
            "CLIWAT-E-037 invalid JSON in hillslope {hillslope_id} manifest_file '{}': {error}",
            manifest_file_path.display()
        )
    })?;

    let schema = manifest
        .pointer("/schema")
        .and_then(Value::as_str)
        .ok_or_else(|| {
            format!(
                "CLIWAT-E-037 hillslope {hillslope_id} manifest_file '{}' missing string /schema",
                manifest_file_path.display()
            )
        })?;
    if schema != HILLSLOPE_RUN_MANIFEST_SCHEMA_ID {
        return Err(format!(
            "CLIWAT-E-037 hillslope {hillslope_id} manifest_file '{}' has unsupported schema '{}' (expected '{}')",
            manifest_file_path.display(),
            schema,
            HILLSLOPE_RUN_MANIFEST_SCHEMA_ID
        ));
    }

    let publication_policy = manifest
        .pointer("/wb13_publication/publication_ofe_policy")
        .and_then(Value::as_str)
        .ok_or_else(|| {
            format!(
                "CLIWAT-E-037 hillslope {hillslope_id} manifest_file '{}' missing string /wb13_publication/publication_ofe_policy",
                manifest_file_path.display()
            )
        })?;
    if !matches!(
        publication_policy,
        MOFE04_PUBLICATION_OFE_POLICY | MF_PUBLICATION_OFE_POLICY
    ) {
        return Err(format!(
            "CLIWAT-E-037 hillslope {hillslope_id} manifest_file '{}' has unsupported publication_ofe_policy '{}' (expected '{}' or '{}')",
            manifest_file_path.display(),
            publication_policy,
            MOFE04_PUBLICATION_OFE_POLICY,
            MF_PUBLICATION_OFE_POLICY
        ));
    }

    let contributor_ofe_count =
        manifest
            .pointer("/wb13_publication/contributor_ofe_count")
            .and_then(Value::as_u64)
            .ok_or_else(|| {
                format!(
                    "CLIWAT-E-037 hillslope {hillslope_id} manifest_file '{}' missing integer /wb13_publication/contributor_ofe_count",
                    manifest_file_path.display()
                )
            })?;
    if contributor_ofe_count == 0 {
        return Err(format!(
            "CLIWAT-E-037 hillslope {hillslope_id} manifest_file '{}' has contributor_ofe_count=0",
            manifest_file_path.display()
        ));
    }
    if contributor_ofe_count != u64::from(contributor_nofe) {
        return Err(format!(
            "CLIWAT-E-037 hillslope {hillslope_id} contributor_ofe_count mismatch: manifest={contributor_ofe_count} vs pass_nofe={contributor_nofe}"
        ));
    }

    let area_policy = manifest
        .pointer("/wb13_publication/area_policy")
        .and_then(Value::as_str)
        .ok_or_else(|| {
            format!(
                "CLIWAT-E-037 hillslope {hillslope_id} manifest_file '{}' missing string /wb13_publication/area_policy",
                manifest_file_path.display()
            )
        })?;
    if area_policy != MOFE04_PUBLICATION_AREA_POLICY {
        return Err(format!(
            "CLIWAT-E-037 hillslope {hillslope_id} manifest_file '{}' has unsupported area_policy '{}' (expected '{}')",
            manifest_file_path.display(),
            area_policy,
            MOFE04_PUBLICATION_AREA_POLICY
        ));
    }

    let publication_area_m2 = manifest
        .pointer("/wb13_publication/publication_area_m2")
        .and_then(Value::as_f64)
        .ok_or_else(|| {
            format!(
                "CLIWAT-E-037 hillslope {hillslope_id} manifest_file '{}' missing numeric /wb13_publication/publication_area_m2",
                manifest_file_path.display()
            )
        })?;
    if !publication_area_m2.is_finite() || publication_area_m2 <= 0.0 {
        return Err(format!(
            "CLIWAT-E-037 hillslope {hillslope_id} manifest_file '{}' invalid publication_area_m2 {} (must be finite and > 0)",
            manifest_file_path.display(),
            publication_area_m2
        ));
    }

    if publication_policy == MF_PUBLICATION_OFE_POLICY {
        validate_manifest_per_ofe_wb13_publication_metadata(
            hillslope_id,
            contributor_ofe_count,
            manifest_file_path,
            &manifest,
        )?;
    }

    validate_manifest_mofe_hourly_carry_metadata(
        hillslope_id,
        contributor_nofe,
        manifest_file_path,
        &manifest,
    )?;

    Ok(())
}

fn validate_manifest_per_ofe_wb13_publication_metadata(
    hillslope_id: u32,
    contributor_ofe_count: u64,
    manifest_file_path: &Path,
    manifest: &Value,
) -> Result<(), String> {
    validate_manifest_per_ofe_wb13_publication_policies(
        hillslope_id,
        manifest_file_path,
        manifest,
    )?;
    validate_manifest_per_ofe_wb13_publication_counts(
        hillslope_id,
        contributor_ofe_count,
        manifest_file_path,
        manifest,
    )?;
    validate_manifest_per_ofe_wb13_publication_keys(
        hillslope_id,
        contributor_ofe_count,
        manifest_file_path,
        manifest,
    )
}

fn validate_manifest_per_ofe_wb13_publication_policies(
    hillslope_id: u32,
    manifest_file_path: &Path,
    manifest: &Value,
) -> Result<(), String> {
    let storage_lineage = manifest
        .pointer("/wb13_publication/storage_lineage_policy")
        .and_then(Value::as_str)
        .ok_or_else(|| {
            format!(
                "CLIWAT-E-037 hillslope {hillslope_id} manifest_file '{}' missing string /wb13_publication/storage_lineage_policy",
                manifest_file_path.display()
            )
        })?;
    if storage_lineage != MF_STORAGE_LINEAGE_POLICY {
        return Err(format!(
            "CLIWAT-E-037 hillslope {hillslope_id} manifest_file '{}' has unsupported storage_lineage_policy '{}' (expected '{}')",
            manifest_file_path.display(),
            storage_lineage,
            MF_STORAGE_LINEAGE_POLICY
        ));
    }

    let state_policy = manifest
        .pointer("/wb13_publication/per_ofe_state_policy")
        .and_then(Value::as_str)
        .ok_or_else(|| {
            format!(
                "CLIWAT-E-037 hillslope {hillslope_id} manifest_file '{}' missing string /wb13_publication/per_ofe_state_policy",
                manifest_file_path.display()
            )
        })?;
    if state_policy != MF_PER_OFE_STATE_POLICY {
        return Err(format!(
            "CLIWAT-E-037 hillslope {hillslope_id} manifest_file '{}' has unsupported per_ofe_state_policy '{}' (expected '{}')",
            manifest_file_path.display(),
            state_policy,
            MF_PER_OFE_STATE_POLICY
        ));
    }

    for pointer in [
        "/wb13_publication/transfer_identity_status",
        "/wb13_publication/per_element_identity_status",
        "/wb13_publication/aggregate_identity_status",
    ] {
        let status = manifest.pointer(pointer).and_then(Value::as_str).ok_or_else(|| {
            format!(
                "CLIWAT-E-037 hillslope {hillslope_id} manifest_file '{}' missing string {pointer}",
                manifest_file_path.display()
            )
        })?;
        if status != MF_IDENTITY_STATUS {
            return Err(format!(
                "CLIWAT-E-037 hillslope {hillslope_id} manifest_file '{}' has unsupported {pointer} '{}' (expected '{}')",
                manifest_file_path.display(),
                status,
                MF_IDENTITY_STATUS
            ));
        }
    }

    let hillslope_total_residual = manifest
        .pointer("/wb13_publication/hillslope_total_identity_max_abs_mm")
        .and_then(Value::as_f64)
        .ok_or_else(|| {
            format!(
                "CLIWAT-E-037 hillslope {hillslope_id} manifest_file '{}' missing numeric /wb13_publication/hillslope_total_identity_max_abs_mm",
                manifest_file_path.display()
            )
        })?;
    if !hillslope_total_residual.is_finite()
        || hillslope_total_residual > MI_HILLSLOPE_TOTAL_IDENTITY_TOLERANCE_MM
    {
        return Err(format!(
            "CLIWAT-E-037 hillslope {hillslope_id} manifest_file '{}' has hillslope_total_identity_max_abs_mm={hillslope_total_residual} above tolerance {MI_HILLSLOPE_TOTAL_IDENTITY_TOLERANCE_MM}",
            manifest_file_path.display()
        ));
    }

    Ok(())
}

fn validate_manifest_per_ofe_wb13_publication_counts(
    hillslope_id: u32,
    contributor_ofe_count: u64,
    manifest_file_path: &Path,
    manifest: &Value,
) -> Result<(), String> {
    let row_count = manifest
        .pointer("/wb13_publication/row_count")
        .and_then(Value::as_u64)
        .ok_or_else(|| {
            format!(
                "CLIWAT-E-037 hillslope {hillslope_id} manifest_file '{}' missing integer /wb13_publication/row_count",
                manifest_file_path.display()
            )
        })?;
    let per_ofe_record_count = manifest
        .pointer("/wb13_publication/per_ofe_record_count")
        .and_then(Value::as_u64)
        .ok_or_else(|| {
            format!(
                "CLIWAT-E-037 hillslope {hillslope_id} manifest_file '{}' missing integer /wb13_publication/per_ofe_record_count",
                manifest_file_path.display()
            )
        })?;
    let expected_record_count = manifest
        .pointer("/wb13_publication/per_ofe_expected_record_count")
        .and_then(Value::as_u64)
        .ok_or_else(|| {
            format!(
                "CLIWAT-E-037 hillslope {hillslope_id} manifest_file '{}' missing integer /wb13_publication/per_ofe_expected_record_count",
                manifest_file_path.display()
            )
        })?;
    let day_count = manifest
        .pointer("/wb13_publication/per_ofe_internal_day_count")
        .and_then(Value::as_u64)
        .ok_or_else(|| {
            format!(
                "CLIWAT-E-037 hillslope {hillslope_id} manifest_file '{}' missing integer /wb13_publication/per_ofe_internal_day_count",
                manifest_file_path.display()
            )
        })?;

    if row_count == 0 || per_ofe_record_count == 0 {
        return Err(format!(
            "CLIWAT-E-037 hillslope {hillslope_id} manifest_file '{}' has empty per-OFE WB13 publication row counts",
            manifest_file_path.display()
        ));
    }
    if row_count != per_ofe_record_count || row_count != expected_record_count {
        return Err(format!(
            "CLIWAT-E-037 hillslope {hillslope_id} manifest_file '{}' per-OFE WB13 row counts disagree: row_count={row_count}, per_ofe_record_count={per_ofe_record_count}, expected_record_count={expected_record_count}",
            manifest_file_path.display()
        ));
    }
    let expected_from_days = day_count
        .checked_mul(contributor_ofe_count)
        .ok_or_else(|| {
            format!(
                "CLIWAT-E-037 hillslope {hillslope_id} manifest_file '{}' per-OFE WB13 expected row count overflowed",
                manifest_file_path.display()
            )
        })?;
    if row_count != expected_from_days {
        return Err(format!(
            "CLIWAT-E-037 hillslope {hillslope_id} manifest_file '{}' per-OFE WB13 row_count={row_count} does not equal day_count={day_count} * contributor_ofe_count={contributor_ofe_count}",
            manifest_file_path.display()
        ));
    }

    Ok(())
}

fn validate_manifest_per_ofe_wb13_publication_keys(
    hillslope_id: u32,
    contributor_ofe_count: u64,
    manifest_file_path: &Path,
    manifest: &Value,
) -> Result<(), String> {
    let monotonic = manifest
        .pointer("/wb13_publication/sim_day_index_monotonic")
        .and_then(Value::as_bool)
        .ok_or_else(|| {
            format!(
                "CLIWAT-E-037 hillslope {hillslope_id} manifest_file '{}' missing bool /wb13_publication/sim_day_index_monotonic",
                manifest_file_path.display()
            )
        })?;
    if !monotonic {
        return Err(format!(
            "CLIWAT-E-037 hillslope {hillslope_id} manifest_file '{}' marks per-OFE WB13 row keys non-monotonic",
            manifest_file_path.display()
        ));
    }

    let first_ofe = manifest
        .pointer("/wb13_publication/first_row_key/ofe")
        .and_then(Value::as_u64)
        .ok_or_else(|| {
            format!(
                "CLIWAT-E-037 hillslope {hillslope_id} manifest_file '{}' missing integer /wb13_publication/first_row_key/ofe",
                manifest_file_path.display()
            )
        })?;
    let last_ofe = manifest
        .pointer("/wb13_publication/last_row_key/ofe")
        .and_then(Value::as_u64)
        .ok_or_else(|| {
            format!(
                "CLIWAT-E-037 hillslope {hillslope_id} manifest_file '{}' missing integer /wb13_publication/last_row_key/ofe",
                manifest_file_path.display()
            )
        })?;
    if first_ofe != 1 || last_ofe != contributor_ofe_count {
        return Err(format!(
            "CLIWAT-E-037 hillslope {hillslope_id} manifest_file '{}' per-OFE WB13 first/last OFE keys are {first_ofe}/{last_ofe}; expected 1/{contributor_ofe_count}",
            manifest_file_path.display()
        ));
    }

    Ok(())
}

fn validate_manifest_mofe_hourly_carry_metadata(
    hillslope_id: u32,
    contributor_nofe: u16,
    manifest_file_path: &Path,
    manifest: &Value,
) -> Result<(), String> {
    let Some(carry_metadata) = manifest.pointer("/mofe_hourly_carry") else {
        if contributor_nofe > 1 {
            return Err(format!(
                "CLIWAT-E-037 hillslope {hillslope_id} manifest_file '{}' missing /mofe_hourly_carry for multi-OFE contributor",
                manifest_file_path.display()
            ));
        }
        return Ok(());
    };
    if !carry_metadata.is_object() {
        return Err(format!(
            "CLIWAT-E-037 hillslope {hillslope_id} manifest_file '{}' has non-object /mofe_hourly_carry",
            manifest_file_path.display()
        ));
    }

    let policy = carry_metadata
        .pointer("/policy")
        .and_then(Value::as_str)
        .ok_or_else(|| {
            format!(
                "CLIWAT-E-037 hillslope {hillslope_id} manifest_file '{}' missing string /mofe_hourly_carry/policy",
                manifest_file_path.display()
            )
        })?;
    if policy != MOFE_HOURLY_CARRY_POLICY {
        return Err(format!(
            "CLIWAT-E-037 hillslope {hillslope_id} manifest_file '{}' has unsupported mofe_hourly_carry policy '{}' (expected '{}')",
            manifest_file_path.display(),
            policy,
            MOFE_HOURLY_CARRY_POLICY
        ));
    }

    let active = carry_metadata
        .pointer("/active")
        .and_then(Value::as_bool)
        .ok_or_else(|| {
            format!(
                "CLIWAT-E-037 hillslope {hillslope_id} manifest_file '{}' missing bool /mofe_hourly_carry/active",
                manifest_file_path.display()
            )
        })?;
    if contributor_nofe > 1 && !active {
        return Err(format!(
            "CLIWAT-E-037 hillslope {hillslope_id} manifest_file '{}' has inactive mofe_hourly_carry for multi-OFE contributor",
            manifest_file_path.display()
        ));
    }

    let substep_count = carry_metadata
        .pointer("/substep_count")
        .and_then(Value::as_u64)
        .ok_or_else(|| {
            format!(
                "CLIWAT-E-037 hillslope {hillslope_id} manifest_file '{}' missing integer /mofe_hourly_carry/substep_count",
                manifest_file_path.display()
            )
        })?;
    if contributor_nofe == 1 && !active && substep_count == 0 {
        validate_manifest_mofe_hourly_carry_inactive_single_ofe(
            hillslope_id,
            manifest_file_path,
            carry_metadata,
        )?;
        validate_manifest_mofe_hourly_carry_totals(
            hillslope_id,
            manifest_file_path,
            carry_metadata,
        )?;
        return Ok(());
    }
    if substep_count != MOFE_HOURLY_CARRY_ARRAY_COUNT {
        return Err(format!(
            "CLIWAT-E-037 hillslope {hillslope_id} manifest_file '{}' has mofe_hourly_carry substep_count={} (expected {})",
            manifest_file_path.display(),
            substep_count,
            MOFE_HOURLY_CARRY_ARRAY_COUNT
        ));
    }

    validate_manifest_mofe_hourly_carry_required_arrays(
        hillslope_id,
        manifest_file_path,
        carry_metadata,
    )?;
    validate_manifest_mofe_hourly_carry_totals(hillslope_id, manifest_file_path, carry_metadata)?;

    Ok(())
}

fn validate_manifest_mofe_hourly_carry_inactive_single_ofe(
    hillslope_id: u32,
    manifest_file_path: &Path,
    carry_metadata: &Value,
) -> Result<(), String> {
    let required_arrays = carry_metadata
        .pointer("/required_arrays")
        .and_then(Value::as_array)
        .ok_or_else(|| {
            format!(
                "CLIWAT-E-037 hillslope {hillslope_id} manifest_file '{}' missing array /mofe_hourly_carry/required_arrays",
                manifest_file_path.display()
            )
        })?;
    if !required_arrays.is_empty() {
        return Err(format!(
            "CLIWAT-E-037 hillslope {hillslope_id} manifest_file '{}' inactive single-OFE mofe_hourly_carry must have empty required_arrays",
            manifest_file_path.display()
        ));
    }
    Ok(())
}

fn validate_manifest_mofe_hourly_carry_required_arrays(
    hillslope_id: u32,
    manifest_file_path: &Path,
    carry_metadata: &Value,
) -> Result<(), String> {
    let required_arrays = carry_metadata
        .pointer("/required_arrays")
        .and_then(Value::as_array)
        .ok_or_else(|| {
            format!(
                "CLIWAT-E-037 hillslope {hillslope_id} manifest_file '{}' missing array /mofe_hourly_carry/required_arrays",
                manifest_file_path.display()
            )
        })?;
    let observed_arrays = required_arrays
        .iter()
        .map(|value| {
            value.as_str().ok_or_else(|| {
                format!(
                    "CLIWAT-E-037 hillslope {hillslope_id} manifest_file '{}' has non-string /mofe_hourly_carry/required_arrays entry",
                    manifest_file_path.display()
                )
            })
        })
        .collect::<Result<BTreeSet<_>, _>>()?;
    for required_array in MOFE_HOURLY_REQUIRED_ARRAYS {
        if !observed_arrays.contains(required_array) {
            return Err(format!(
                "CLIWAT-E-037 hillslope {hillslope_id} manifest_file '{}' missing mofe_hourly_carry required array '{}'",
                manifest_file_path.display(),
                required_array
            ));
        }
    }
    Ok(())
}

fn validate_manifest_mofe_hourly_carry_totals(
    hillslope_id: u32,
    manifest_file_path: &Path,
    carry_metadata: &Value,
) -> Result<(), String> {
    for pointer in ["/upstream_carry_total_m", "/current_carry_total_m"] {
        let value = carry_metadata
            .pointer(pointer)
            .and_then(Value::as_f64)
            .ok_or_else(|| {
                format!(
                    "CLIWAT-E-037 hillslope {hillslope_id} manifest_file '{}' missing numeric /mofe_hourly_carry{pointer}",
                    manifest_file_path.display()
                )
            })?;
        if !value.is_finite() || value < 0.0 {
            return Err(format!(
                "CLIWAT-E-037 hillslope {hillslope_id} manifest_file '{}' invalid /mofe_hourly_carry{pointer} {} (must be finite and >= 0)",
                manifest_file_path.display(),
                value
            ));
        }
    }
    Ok(())
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

fn write_watershed_interchange_outputs(
    outputs: &WatershedOutputsResolved,
    row_seeds: &[WatershedInterchangeRowSeed],
) -> Result<(), String> {
    if row_seeds.len() == 1 {
        write_interchange_parquet_outputs(outputs, row_seeds[0])
    } else {
        write_interchange_parquet_outputs_from_rows(outputs, row_seeds)
    }
    .map_err(|error| format!("CLIWAT-E-034 watershed output writer failure: {error}"))
}

fn publication_frame_to_row_seed(
    publication_frame: &WatershedPublicationFrame,
) -> WatershedInterchangeRowSeed {
    WatershedInterchangeRowSeed {
        year: publication_frame.year,
        simulation_year: publication_frame.simulation_year,
        sim_day_index: publication_frame.sim_day_index,
        julian: publication_frame.julian,
        month: publication_frame.month,
        day_of_month: publication_frame.day_of_month,
        water_year: publication_frame.water_year,
        element_id: publication_frame.element_id,
        channel_id: publication_frame.channel_id,
        runoff_volume_m3: publication_frame.runoff_volume_m3,
        peak_discharge_m3_s: publication_frame.peak_discharge_m3_s,
        sediment_yield_kg: publication_frame.sediment_yield_kg,
        soluble_pollutant_kg: publication_frame.soluble_pollutant_kg,
        particulate_pollutant_kg: publication_frame.particulate_pollutant_kg,
        channel_outflow_m3: publication_frame.channel_outflow_m3,
        channel_storage_m3: publication_frame.channel_storage_m3,
        channel_baseflow_m3: publication_frame.channel_baseflow_m3,
        channel_loss_m3: publication_frame.channel_loss_m3,
        area_m2: publication_frame.area_m2,
        precipitation_mm: publication_frame.precipitation_mm,
        rain_melt_mm: publication_frame.rain_melt_mm,
        runoff_mm: publication_frame.runoff_mm,
        deep_percolation_mm: publication_frame.deep_percolation_mm,
        lateral_flow_mm: publication_frame.lateral_flow_mm,
        qofe_mm: publication_frame.qofe_mm,
        transpiration_mm: publication_frame.transpiration_mm,
        evaporation_soil_mm: publication_frame.evaporation_soil_mm,
        evaporation_residue_mm: publication_frame.evaporation_residue_mm,
        upstream_q_mm: publication_frame.upstream_q_mm,
        subsurface_runon_mm: publication_frame.subsurface_runon_mm,
        total_soil_water_mm: publication_frame.total_soil_water_mm,
        soil_water_total_mm: publication_frame.soil_water_total_mm,
        profile_depth_mm: publication_frame.profile_depth_mm,
        profile_porosity_cap_mm: publication_frame.profile_porosity_cap_mm,
        profile_fc_store_mm: publication_frame.profile_fc_store_mm,
        profile_wp_store_mm: publication_frame.profile_wp_store_mm,
        interception_mm: publication_frame.interception_mm,
        interception_storage_mm: publication_frame.interception_storage_mm,
        frozen_water_mm: publication_frame.frozen_water_mm,
        snow_water_mm: publication_frame.snow_water_mm,
        tile_mm: publication_frame.tile_mm,
        irrigation_mm: publication_frame.irrigation_mm,
        baseflow_mm: publication_frame.baseflow_mm,
        ..WatershedInterchangeRowSeed::default()
    }
}

fn print_help() {
    println!(
        "openwepp-cli-watershed --run-dir <path> --run-file <path> --output-dir <path> [--policy compat] [--legacy-sidecar-discovery] [--jobs N] [--hillslope-binary <path>]"
    );
}
