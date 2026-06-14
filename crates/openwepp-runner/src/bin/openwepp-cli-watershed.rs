use std::collections::{BTreeMap, BTreeSet};
use std::fs;
use std::path::{Path, PathBuf};

use openwepp_input_contract::parsers::chaninp::{ChaninpParseOptions, parse_chaninp_from_path};
use openwepp_input_contract::parsers::hbp::{
    HbpParseOptions, parse_hbp_from_path_with_latest_event_payload,
};
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
use openwepp_kernel_contract::{
    BoundarySymbol, BoundaryValue, WatershedChannelFluxField, WatershedChannelStateField,
    WatershedImpoundmentFluxField, WatershedProductionFluxSymbol, WatershedProductionStateSymbol,
};
use openwepp_legacy_bridge::sidecar::{
    SidecarAdapterRequest, SidecarBinding, SidecarContract, SidecarDiscovery, SidecarId,
    SidecarRequirement, adapt_sidecar_bindings,
};
use openwepp_runner::SidecarPolicy;
use openwepp_topology::{
    ContributorTriplet, TopologyContributors, TopologyGraph, TopologyNode, TopologyNodeKey,
    TopologyNodeKind, validate_pre_execution_topology,
};
use openwepp_watershed_orchestrator::runtime_inputs::{
    build_watershed_runtime_surface_from_chaninp,
    seed_watershed_runtime_surface_from_slope_channel_profile,
    seed_watershed_runtime_surface_from_watershed_channel,
    seed_watershed_runtime_surface_from_watershed_impoundment,
};
use openwepp_watershed_orchestrator::{
    WatershedKernelExecutionReport, WatershedWritebackSurface, Ws10ChannelImpoundmentKernel,
    execute_watershed_dispatch_with_kernel,
};
use openwepp_watershed_output::contracts::{WatershedOutputConfig, validate_output_contract};
use openwepp_watershed_output::writers::{
    WatershedInterchangeRowSeed, write_interchange_parquet_outputs,
    write_interchange_parquet_outputs_from_rows,
};
use serde::Deserialize;
use serde_json::Value;

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
        sidecar_policy,
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
    seed_watershed_runtime_surface_from_slope_channel_profile(
        &mut runtime_surface,
        &watershed_channel,
        &slope,
    )
    .map_err(|error| {
        format!("CLIWAT-E-039 failed seeding watershed channel segment runtime surface: {error}")
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

    for (hillslope_id, block) in &runfile.hillslope_blocks_by_id {
        let hbp_options = HbpParseOptions {
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
        validate_contributor_mofe_metadata(
            *hillslope_id,
            hbp.nofe,
            block.manifest_file_path.as_deref(),
        )?;

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
        let particle_diameters = latest_event_payload.as_ref().map_or_else(
            || hbp.particle_diameter_m.clone(),
            |payload| payload.particle_diameter_m.clone(),
        );
        let particle_flow_fractions = latest_event_payload.as_ref().map_or_else(
            || vec![0.0; class_count],
            |payload| payload.particle_flow_fraction.clone(),
        );
        if particle_diameters.len() != class_count {
            return Err(format!(
                "CLIWAT-E-018 particle_diameter_m length {} does not match npart={} for hillslope {}",
                particle_diameters.len(),
                class_count,
                hillslope_id
            ));
        }

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
                    WatershedProductionStateSymbol::HillslopeContributorParticleDiameterMeters {
                        hillslope_id: *hillslope_id,
                        class_index,
                    },
                ),
                BoundaryValue::scalar(particle_diameter),
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

    for warning in &sidecar_warnings {
        eprintln!("sidecar-warning: {warning}");
    }

    let row_seed = build_watershed_output_row_seed(&report);
    write_watershed_interchange_outputs(&runfile.outputs, &[row_seed])?;

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
    pass_file: String,
    #[serde(default)]
    manifest_file: Option<String>,
    #[serde(default)]
    unit_system: Option<String>,
    #[serde(default)]
    use_existing_pass_file: Option<bool>,
}

#[derive(Debug)]
struct WatershedHillslopeBlockResolved {
    pass_file_path: PathBuf,
    manifest_file_path: Option<PathBuf>,
}

type WatershedOutputsResolved = WatershedOutputConfig;

#[derive(Debug)]
struct WatershedRunfileResolved {
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

        if hillslope_blocks_by_id
            .insert(
                block.hillslope_id,
                WatershedHillslopeBlockResolved {
                    pass_file_path,
                    manifest_file_path,
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

#[allow(clippy::too_many_lines)]
fn build_watershed_output_row_seed(
    report: &WatershedKernelExecutionReport,
) -> WatershedInterchangeRowSeed {
    let mut channel_ids = BTreeSet::new();
    let mut impoundment_ids = BTreeSet::new();
    let mut contributor_hillslopes = BTreeSet::new();

    for step in &report.dispatch_report.steps {
        match step.node.kind {
            TopologyNodeKind::Channel => {
                channel_ids.insert(step.node.id);
            }
            TopologyNodeKind::Impoundment => {
                impoundment_ids.insert(step.node.id);
            }
            TopologyNodeKind::Hillslope => {}
        }
        contributor_hillslopes.extend(step.contributor_hillslopes.iter().copied());
    }

    let runoff_volume_m3 = channel_ids
        .iter()
        .copied()
        .map(|node_id| {
            boundary_scalar(
                &report.writeback_surface.flux_surface,
                WatershedProductionFluxSymbol::ChannelNode {
                    node_id,
                    field: WatershedChannelFluxField::Roff,
                },
            )
        })
        .sum::<f64>();
    let channel_outflow_m3 = impoundment_ids
        .iter()
        .copied()
        .map(|node_id| {
            boundary_scalar(
                &report.writeback_surface.flux_surface,
                WatershedProductionFluxSymbol::ImpoundmentNode {
                    node_id,
                    field: WatershedImpoundmentFluxField::OutflowVolume,
                },
            )
        })
        .sum::<f64>();
    let peak_discharge_m3_s = channel_ids.iter().copied().next().map_or(0.0, |node_id| {
        boundary_scalar(
            &report.writeback_surface.state_surface,
            WatershedProductionStateSymbol::ChannelNode {
                node_id,
                field: WatershedChannelStateField::Qpo,
            },
        )
    });
    let sediment_yield_kg = channel_ids
        .iter()
        .copied()
        .map(|node_id| {
            report
                .writeback_surface
                .state_surface
                .get(&BoundarySymbol::from(format!(
                    "ws10_channel_{node_id}_qsed"
                )))
                .map_or(0.0, |value| value.as_f64())
        })
        .sum::<f64>();
    let soluble_pollutant_kg = 0.0;
    let particulate_pollutant_kg = contributor_hillslopes
        .iter()
        .copied()
        .map(|hillslope_id| {
            boundary_scalar(
                &report.writeback_surface.state_surface,
                WatershedProductionStateSymbol::HillslopeContributorTotalDetachmentKg {
                    hillslope_id,
                },
            )
        })
        .sum::<f64>();

    WatershedInterchangeRowSeed {
        year: 1,
        simulation_year: 1,
        sim_day_index: i32::try_from(report.dispatch_report.steps.len().max(1)).unwrap_or(i32::MAX),
        julian: 1,
        month: 1,
        day_of_month: 1,
        water_year: 1,
        element_id: i32::try_from(channel_ids.iter().copied().next().unwrap_or(1))
            .unwrap_or(i32::MAX),
        channel_id: i32::try_from(channel_ids.iter().copied().next().unwrap_or(1))
            .unwrap_or(i32::MAX),
        runoff_volume_m3,
        peak_discharge_m3_s,
        sediment_yield_kg,
        soluble_pollutant_kg,
        particulate_pollutant_kg,
        channel_outflow_m3,
        channel_storage_m3: 0.0,
        channel_baseflow_m3: boundary_scalar(
            &report.writeback_surface.flux_surface,
            WatershedProductionFluxSymbol::Cbase,
        ),
        channel_loss_m3: 0.0,
        area_m2: 1.0,
        precipitation_mm: 0.0,
        rain_melt_mm: 0.0,
        runoff_mm: runoff_volume_m3 * 1_000.0,
        deep_percolation_mm: 0.0,
        lateral_flow_mm: 0.0,
        qofe_mm: 0.0,
        transpiration_mm: 0.0,
        evaporation_soil_mm: 0.0,
        evaporation_residue_mm: 0.0,
        upstream_q_mm: 0.0,
        subsurface_runon_mm: 0.0,
        total_soil_water_mm: 0.0,
        soil_water_total_mm: 0.0,
        profile_depth_mm: 0.0,
        profile_porosity_cap_mm: 0.0,
        profile_fc_store_mm: 0.0,
        profile_wp_store_mm: 0.0,
        interception_mm: 0.0,
        interception_storage_mm: 0.0,
        frozen_water_mm: 0.0,
        snow_water_mm: 0.0,
        tile_mm: 0.0,
        irrigation_mm: 0.0,
        baseflow_mm: 0.0,
        ..WatershedInterchangeRowSeed::default()
    }
}

fn boundary_scalar<T>(surface: &BTreeMap<BoundarySymbol, BoundaryValue>, symbol: T) -> f64
where
    T: Into<BoundarySymbol>,
{
    surface
        .get(&symbol.into())
        .map_or(0.0, |value| value.as_f64())
}

fn print_help() {
    println!(
        "openwepp-cli-watershed --run-dir <path> --run-file <path> --output-dir <path> [--policy compat] [--legacy-sidecar-discovery]"
    );
}
