use std::collections::BTreeMap;
use std::error::Error;
use std::fmt;
use std::fs::{self, File};
use std::io::{self, Write};
use std::path::{Path, PathBuf};

use arrow_array::{Array, Float64Array, Int8Array, Int16Array, Int32Array, RecordBatch};
use openwepp_hillslope_orchestrator::runtime_inputs::{
    DirectWinterHourlyContext, DirectWinterHourlyForcing, HillslopeClimateRuntimeRequest,
    build_hillslope_climate_runtime_request,
};
use openwepp_hillslope_orchestrator::{
    DirectExecutorMode, DirectFrameExecutor, DirectPublicationRunMetadata, DirectRuntimeError,
};
use openwepp_input_contract::parsers::climate::ClimateDailyRecord;
use openwepp_kernel_contract::{BoundarySymbol, BoundaryValue};
use parquet::arrow::arrow_reader::ParquetRecordBatchReaderBuilder;
use serde::Serialize;

use crate::api::{HillslopeRunRequest, HillslopeRuntimeSelection};
use crate::hillslope::intake_lane_setup::saturation_vapor_pressure_kpa;
use crate::{HillslopeCliError, SidecarPolicy};

pub const PYSNOBAL_FORCING_COLUMNS: [&str; 10] = [
    "net_solar_Wm-2",
    "downwelling_thermal_Wm-2",
    "temp_air_degC",
    "temp_ground_degC",
    "vapor_pressure_Pa",
    "wind_speed_ms-1",
    "precip_mass_mm",
    "precip_temp_degC",
    "snow_precip_fraction",
    "snow_precip_density_kgm-3",
];

const GROUND_TEMP_LANES: [GroundTempLane; 3] = [
    GroundTempLane {
        id: "tg_neg2p5c_zg0p10m",
        temp_ground_c: -2.5,
        soil_temp_depth_m: 0.10,
    },
    GroundTempLane {
        id: "tg_neg0p5c_zg0p10m",
        temp_ground_c: -0.5,
        soil_temp_depth_m: 0.10,
    },
    GroundTempLane {
        id: "tg_0p0c_zg0p10m",
        temp_ground_c: 0.0,
        soil_temp_depth_m: 0.10,
    },
];
const NET_SHORTWAVE_FACTOR: f64 = 0.80;
const DEFAULT_AIR_TEMP_HEIGHT_M: f64 = 2.0;
const DEFAULT_WIND_SPEED_HEIGHT_M: f64 = 2.0;
const DEFAULT_ROUGHNESS_LENGTH_M: f64 = 0.005;
const STEFAN_BOLTZMANN_W_M2_K4: f64 = 5.670_374_419e-8;
const CANOPY_SERIES_SOURCE: &str =
    "direct_production_day_input.growth_state_for_publication.cancov";
const CANOPY_SERIES_FILENAME: &str = "canopy_series.csv";

#[derive(Debug, Clone)]
pub struct SnowbenchExportRequest {
    pub run_dir: PathBuf,
    pub run_file: Option<PathBuf>,
    pub output_dir: PathBuf,
    pub include_openwepp_snow_projection: bool,
}

#[derive(Debug, Clone, Serialize)]
pub struct SnowbenchExportReport {
    pub schema: &'static str,
    pub run_dir: String,
    pub generated_runfile: String,
    pub output_dir: String,
    pub day_count: usize,
    pub hourly_row_count: usize,
    pub lane_count: usize,
    pub total_precip_mass_mm: f64,
    pub total_snow_precip_mass_mm: f64,
    pub primary_canopy_cover_fraction: f64,
    pub canopy_source: &'static str,
    pub canopy_series_path: String,
    pub canopy_series_summary: SnowbenchCanopySeriesSummary,
    pub lane_ids: Vec<&'static str>,
}

#[derive(Debug, Clone, Copy, Serialize)]
pub struct SnowbenchCanopySeriesSummary {
    pub day_count: usize,
    pub min: f64,
    pub max: f64,
    pub mean: f64,
    pub first: f64,
    pub last: f64,
    pub dynamic: bool,
}

#[derive(Debug)]
pub enum SnowbenchError {
    Io {
        path: PathBuf,
        source: io::Error,
    },
    Json {
        path: PathBuf,
        source: serde_json::Error,
    },
    Runner {
        source: HillslopeCliError,
    },
    ClimateRuntime {
        detail: String,
    },
    InvalidInput {
        detail: String,
    },
    InvalidForcing {
        detail: String,
    },
    OpenweppSnow {
        detail: String,
    },
}

impl SnowbenchError {
    fn io(path: impl Into<PathBuf>, source: io::Error) -> Self {
        Self::Io {
            path: path.into(),
            source,
        }
    }
}

impl fmt::Display for SnowbenchError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Io { path, source } => {
                write!(
                    f,
                    "SNOWBENCH-E-001 io error at {}: {source}",
                    path.display()
                )
            }
            Self::Json { path, source } => write!(
                f,
                "SNOWBENCH-E-002 failed to write JSON at {}: {source}",
                path.display()
            ),
            Self::Runner { source } => write!(f, "SNOWBENCH-E-003 runner input error: {source}"),
            Self::ClimateRuntime { detail } => {
                write!(f, "SNOWBENCH-E-004 climate runtime forcing error: {detail}")
            }
            Self::InvalidInput { detail } => write!(f, "SNOWBENCH-E-005 invalid input: {detail}"),
            Self::InvalidForcing { detail } => {
                write!(f, "SNOWBENCH-E-006 invalid forcing: {detail}")
            }
            Self::OpenweppSnow { detail } => {
                write!(
                    f,
                    "SNOWBENCH-E-007 openWEPP snow diagnostic error: {detail}"
                )
            }
        }
    }
}

impl Error for SnowbenchError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            Self::Io { source, .. } => Some(source),
            Self::Json { source, .. } => Some(source),
            Self::Runner { source } => Some(source),
            Self::ClimateRuntime { .. }
            | Self::InvalidInput { .. }
            | Self::InvalidForcing { .. }
            | Self::OpenweppSnow { .. } => None,
        }
    }
}

impl From<HillslopeCliError> for SnowbenchError {
    fn from(source: HillslopeCliError) -> Self {
        Self::Runner { source }
    }
}

#[derive(Debug, Clone, Copy, Serialize)]
struct GroundTempLane {
    id: &'static str,
    temp_ground_c: f64,
    soil_temp_depth_m: f64,
}

#[derive(Debug, Clone)]
struct DailyForcingExport {
    date: CalendarDate,
    wind_speed_m_s: f64,
    dew_point_c: f64,
    hourly: [DirectWinterHourlyForcing; 24],
}

#[derive(Debug, Clone, Copy)]
struct CanopySeriesDay {
    date: CalendarDate,
    canopy_cover_fraction: f64,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct CalendarDate {
    year: i32,
    month: i32,
    day: i32,
}

#[derive(Debug, Clone, Copy)]
struct ExportRow {
    date: CalendarDate,
    hour_index: usize,
    net_solar_w_m2: f64,
    downwelling_thermal_w_m2: f64,
    temp_air_c: f64,
    temp_ground_c: f64,
    vapor_pressure_pa: f64,
    wind_speed_m_s: f64,
    precip_mass_mm: f64,
    snow_precip_mass_mm: f64,
    precip_temp_c: f64,
    snow_precip_fraction: f64,
    snow_precip_density_kg_m3: f64,
}

#[derive(Debug, Serialize)]
#[allow(clippy::struct_excessive_bools)]
struct AuditDocument {
    schema: &'static str,
    run_dir: String,
    runfile: String,
    lane: GroundTempLane,
    rows: usize,
    day_count: usize,
    total_precip_mass_mm: f64,
    total_snow_precip_mass_mm: f64,
    positive_snow_precip_rows: usize,
    nonfinite_rows_rejected: bool,
    negative_precip_rejected: bool,
    uniform_hourly_timestamps: bool,
    snowfall_depth_to_mass_conversion: &'static str,
    wat_snow_water_is_not_depth: bool,
    frost_surface_temp_is_not_ground_temp: bool,
    daily_radiation_not_exported_as_hourly_wm2: bool,
}

#[derive(Debug, Serialize)]
struct LineageDocument {
    schema: &'static str,
    lane: GroundTempLane,
    fields: BTreeMap<&'static str, LineageField>,
}

#[derive(Debug, Serialize)]
struct LineageField {
    units: &'static str,
    source_class: &'static str,
    source: &'static str,
    conversion: &'static str,
    rejected_aliases: Vec<&'static str>,
}

#[derive(Debug, Serialize)]
struct OpenweppSnowAvailability {
    schema: &'static str,
    status: &'static str,
    reason: String,
    source_wat_parquet: Option<String>,
    row_count: usize,
}

#[derive(Debug)]
struct OpenweppSnowRow {
    year: i16,
    month: i8,
    day_of_month: i8,
    sim_day_index: i32,
    snow_water_mm: f64,
    snow_depth_mm: Option<f64>,
}

pub fn export_pysnobal_inputs(
    request: &SnowbenchExportRequest,
) -> Result<SnowbenchExportReport, SnowbenchError> {
    let run_dir = make_absolute(&request.run_dir)?;
    let output_dir = make_absolute(&request.output_dir)?;
    fs::create_dir_all(&output_dir).map_err(|source| SnowbenchError::io(&output_dir, source))?;

    let legacy_run_file = request.run_file.as_ref().map_or_else(
        || discover_single_legacy_run_file(&run_dir),
        |path| Ok(resolve_path(&run_dir, path)),
    )?;
    let run_stem = legacy_run_file
        .file_stem()
        .and_then(|value| value.to_str())
        .ok_or_else(|| SnowbenchError::InvalidInput {
            detail: format!("run file '{}' has no UTF-8 stem", legacy_run_file.display()),
        })?;
    let generated_runfile = output_dir.join("openwepp_snowbench.run.toml");
    write_generated_runfile(&generated_runfile, &run_dir, run_stem, &output_dir)?;

    let hillslope_request = HillslopeRunRequest {
        run_dir: run_dir.clone(),
        run_file: generated_runfile.clone(),
        output_dir: output_dir.join("openwepp_outputs"),
        sidecar_policy: SidecarPolicy::Compat,
        legacy_sidecar_discovery: true,
        manifest_path: None,
    };
    let inputs = super::load_hillslope_run_inputs(&hillslope_request)?;
    let targets = super::resolve_hillslope_output_targets(&inputs.runfile)?;
    let mut sidecars = super::resolve_hillslope_sidecars(&hillslope_request, &inputs, &targets)?;
    let static_setup =
        super::build_static_hillslope_runtime_setup(&hillslope_request, &inputs, &mut sidecars)?;
    let runtime_surface = &static_setup.execution_state.runtime_surface;
    let primary_canopy_cover_fraction =
        require_state_scalar(&runtime_surface.state_surface, "cancov")?;
    if !(0.0..=1.0).contains(&primary_canopy_cover_fraction) {
        return Err(SnowbenchError::InvalidInput {
            detail: format!(
                "runtime canopy cover fraction must be in [0,1], observed {primary_canopy_cover_fraction}"
            ),
        });
    }
    let climate_request =
        build_hillslope_climate_runtime_request(&inputs.climate).map_err(|error| {
            SnowbenchError::ClimateRuntime {
                detail: error.to_string(),
            }
        })?;
    let context = winter_context_from_surface(&runtime_surface.state_surface)?;
    let daily_forcing =
        build_daily_forcing(&inputs.climate.daily_records, &climate_request, context)?;
    let canopy_series = build_direct_runtime_canopy_series(
        targets.output_hillslope_id,
        &static_setup.execution_state,
        &climate_request,
    )?;
    validate_canopy_series_alignment(&daily_forcing, &canopy_series)?;
    let canopy_series_summary = summarize_canopy_series(&canopy_series)?;
    let canopy_series_path = output_dir.join(CANOPY_SERIES_FILENAME);
    write_canopy_series_csv(&canopy_series_path, &canopy_series)?;
    let snow_density = require_state_scalar(&runtime_surface.state_surface, "snow.options.newsnw")?;

    if request.include_openwepp_snow_projection {
        write_openwepp_snow_projection(&output_dir, &hillslope_request, &daily_forcing)?;
    } else {
        write_openwepp_snow_placeholder(&output_dir)?;
    }

    let (report_total_precip, report_total_snow) = write_pysnobal_lane_exports(
        &output_dir,
        &request.run_dir,
        &generated_runfile,
        &daily_forcing,
        &climate_request,
        snow_density,
    )?;

    let report = SnowbenchExportReport {
        schema: "snowfrost-fidelity-g0-pysnobal-export-v1",
        run_dir: run_dir.display().to_string(),
        generated_runfile: generated_runfile.display().to_string(),
        output_dir: output_dir.display().to_string(),
        day_count: daily_forcing.len(),
        hourly_row_count: daily_forcing.len() * 24,
        lane_count: GROUND_TEMP_LANES.len(),
        total_precip_mass_mm: report_total_precip,
        total_snow_precip_mass_mm: report_total_snow,
        primary_canopy_cover_fraction,
        canopy_source: CANOPY_SERIES_SOURCE,
        canopy_series_path: canopy_series_path.display().to_string(),
        canopy_series_summary,
        lane_ids: GROUND_TEMP_LANES.iter().map(|lane| lane.id).collect(),
    };
    write_json(&output_dir.join("export_summary.json"), &report)?;
    Ok(report)
}

fn make_absolute(path: &Path) -> Result<PathBuf, SnowbenchError> {
    if path.is_absolute() {
        return Ok(path.to_path_buf());
    }
    let cwd = std::env::current_dir().map_err(|source| SnowbenchError::io(".", source))?;
    Ok(cwd.join(path))
}

fn write_pysnobal_lane_exports(
    output_dir: &Path,
    request_run_dir: &Path,
    generated_runfile: &Path,
    daily_forcing: &[DailyForcingExport],
    climate_request: &HillslopeClimateRuntimeRequest,
    snow_density: f64,
) -> Result<(f64, f64), SnowbenchError> {
    let mut report_total_precip = 0.0;
    let mut report_total_snow = 0.0;
    for lane in GROUND_TEMP_LANES {
        let rows = lane_rows(daily_forcing, lane, snow_density)?;
        let lane_dir = output_dir.join(lane.id);
        fs::create_dir_all(&lane_dir).map_err(|source| SnowbenchError::io(&lane_dir, source))?;
        write_forcing_csv(&lane_dir.join("forcing.csv"), &rows)?;
        write_config_yaml(
            &lane_dir.join("config.yaml"),
            &lane_dir.join("forcing.csv"),
            &lane_dir.join("pysnobal_output.csv"),
            lane,
            climate_request.direct_elevation_m(),
        )?;
        write_lineage_json(&lane_dir.join("lineage.json"), lane)?;
        let audit = audit_document(
            request_run_dir,
            generated_runfile,
            lane,
            &rows,
            daily_forcing.len(),
        );
        write_json(&lane_dir.join("audit.json"), &audit)?;
        write_audit_markdown(&lane_dir.join("audit.md"), &audit)?;
        report_total_precip = audit.total_precip_mass_mm;
        report_total_snow = audit.total_snow_precip_mass_mm;
    }
    Ok((report_total_precip, report_total_snow))
}

fn discover_single_legacy_run_file(run_dir: &Path) -> Result<PathBuf, SnowbenchError> {
    let entries = fs::read_dir(run_dir).map_err(|source| SnowbenchError::io(run_dir, source))?;
    let mut candidates = Vec::new();
    for entry in entries {
        let entry = entry.map_err(|source| SnowbenchError::io(run_dir, source))?;
        let path = entry.path();
        if path.extension().and_then(|value| value.to_str()) == Some("run") && path.is_file() {
            candidates.push(path);
        }
    }
    candidates.sort();
    match candidates.as_slice() {
        [path] => Ok(path.clone()),
        [] => Err(SnowbenchError::InvalidInput {
            detail: format!("no .run file found under '{}'", run_dir.display()),
        }),
        _ => Err(SnowbenchError::InvalidInput {
            detail: format!(
                "multiple .run files found under '{}'; pass --run-file",
                run_dir.display()
            ),
        }),
    }
}

fn resolve_path(base: &Path, path: &Path) -> PathBuf {
    if path.is_absolute() {
        path.to_path_buf()
    } else {
        base.join(path)
    }
}

fn write_generated_runfile(
    path: &Path,
    fixture_dir: &Path,
    run_stem: &str,
    output_dir: &Path,
) -> Result<(), SnowbenchError> {
    let payload = format!(
        "schema = \"openwepp-hillslope-runfile-v1\"\n\
         run_name = \"snowfrost-fidelity-g0-pysnobal\"\n\
         unit_system = \"metric\"\n\n\
         [inputs]\n\
         soil = \"{}\"\n\
         management = \"{}\"\n\
         slope = \"{}\"\n\
         climate = \"{}\"\n\
         wepp_ui = false\n\n\
         [outputs]\n\
         pass = \"{}\"\n\
         loss = \"{}\"\n\
         wat = \"{}\"\n",
        toml_path(&fixture_dir.join(format!("{run_stem}.sol"))),
        toml_path(&fixture_dir.join(format!("{run_stem}.man"))),
        toml_path(&fixture_dir.join(format!("{run_stem}.slp"))),
        toml_path(&fixture_dir.join(format!("{run_stem}.cli"))),
        toml_path(&output_dir.join("openwepp_outputs/snowbench.hbp")),
        toml_path(&output_dir.join("openwepp_outputs/snowbench.loss.json")),
        toml_path(&output_dir.join("openwepp_outputs/snowbench.wat.parquet")),
    );
    write_text(path, &payload)
}

fn toml_path(path: &Path) -> String {
    path.display()
        .to_string()
        .replace('\\', "\\\\")
        .replace('"', "\\\"")
}

fn winter_context_from_surface(
    surface: &BTreeMap<BoundarySymbol, BoundaryValue>,
) -> Result<DirectWinterHourlyContext, SnowbenchError> {
    Ok(DirectWinterHourlyContext {
        snow_runtime_swe_m: 0.0,
        frost_runtime_depth_m: 0.0,
        frost_runtime_frozen_water_m: 0.0,
        frost_file_present: require_state_scalar(surface, "frost.options.frost_file_present")?
            > 0.5,
        frost_wint_red_enabled: require_state_scalar(surface, "frost.options.wintRed")? > 0.5,
        avg_slope: require_state_scalar(surface, "avgslp")?,
        azimuth: require_state_scalar(surface, "azm")?,
        snow_rst_c: require_state_scalar(surface, "snow.options.rst")?,
    })
}

fn require_state_scalar(
    surface: &BTreeMap<BoundarySymbol, BoundaryValue>,
    symbol: &'static str,
) -> Result<f64, SnowbenchError> {
    let value = surface
        .get(&BoundarySymbol::from(symbol))
        .ok_or_else(|| SnowbenchError::InvalidInput {
            detail: format!("missing required runtime symbol {symbol}"),
        })?
        .as_f64();
    require_finite(symbol, value)?;
    Ok(value)
}

fn build_direct_runtime_canopy_series(
    output_hillslope_id: u32,
    state: &super::HillslopeClimateExecutionState,
    climate_request: &HillslopeClimateRuntimeRequest,
) -> Result<Vec<CanopySeriesDay>, SnowbenchError> {
    let lane_seed_surfaces = super::direct_production_lane_seed_surfaces(
        &state.runtime_surface,
        state.persistent_lane_state.as_ref(),
        state.per_ofe_lane_areas_m2.len(),
    )?;
    let mut frame =
        super::build_direct_production_run_frame(&super::DirectProductionRunFrameBuildInputs {
            output_hillslope_id,
            lane_areas_m2: &state.per_ofe_lane_areas_m2,
            runoff_publication_geometries: &state.per_ofe_runoff_publication_geometries,
            day_count: state.climate_span.days.len(),
            climate_request,
            climate_span: &state.climate_span,
            climate_context_surface: &state.runtime_surface,
            lane_seed_surfaces: &lane_seed_surfaces,
            execution_lane: state.lane_context.lane,
        })?;
    let day_input_builder = super::DirectProductionDayInputBuilder::new(
        climate_request,
        &state.climate_span,
        &lane_seed_surfaces,
        &state.runtime_surface,
        state.lane_context.lane,
    )?;
    let metadata = DirectPublicationRunMetadata {
        run_name: "snowbench-per-day-cancov".to_string(),
        runtime_selection: HillslopeRuntimeSelection::DirectProductionExecutor
            .as_str()
            .to_string(),
        output_policy: super::direct_publication_output_policy(
            HillslopeRuntimeSelection::DirectProductionExecutor,
        )
        .to_string(),
    };
    let mut canopy_by_day = vec![None; state.climate_span.days.len()];
    DirectFrameExecutor::new(DirectExecutorMode::ProductionDirect)
        .run_publication_capture_with_interleaved_day_inputs(
            &mut frame,
            metadata,
            |frame, day_index, lane_index| {
                let day_input = day_input_builder
                    .build(frame, day_index, lane_index)
                    .map_err(|error| super::direct_publication_day_input_build_error(&error))?;
                if lane_index == 0 {
                    let canopy_cover_fraction = day_input.canopy_cover_fraction.ok_or(
                        DirectRuntimeError::DirectDomainViolation {
                            field: "publication_input.canopy_cover_fraction",
                        },
                    )?;
                    let day = state.climate_span.days.get(day_index).ok_or(
                        DirectRuntimeError::DayIndexOutOfRange {
                            day_index,
                            day_count: state.climate_span.days.len(),
                        },
                    )?;
                    canopy_by_day[day_index] = Some(CanopySeriesDay {
                        date: CalendarDate {
                            year: day.year,
                            month: day.month,
                            day: day.day_of_month,
                        },
                        canopy_cover_fraction,
                    });
                }
                Ok(day_input)
            },
        )
        .map_err(|source| SnowbenchError::Runner {
            source: super::direct_production_runtime_error(&source),
        })?;
    canopy_by_day
        .into_iter()
        .enumerate()
        .map(|(day_index, value)| {
            value.ok_or_else(|| SnowbenchError::OpenweppSnow {
                detail: format!(
                    "direct production canopy series missing day {}",
                    day_index + 1
                ),
            })
        })
        .collect()
}

fn validate_canopy_series_alignment(
    daily_forcing: &[DailyForcingExport],
    canopy_series: &[CanopySeriesDay],
) -> Result<(), SnowbenchError> {
    if daily_forcing.len() != canopy_series.len() {
        return Err(SnowbenchError::InvalidForcing {
            detail: format!(
                "daily canopy series length {} does not match forcing day count {}",
                canopy_series.len(),
                daily_forcing.len()
            ),
        });
    }
    for (index, (forcing, canopy)) in daily_forcing.iter().zip(canopy_series).enumerate() {
        if forcing.date != canopy.date {
            return Err(SnowbenchError::InvalidForcing {
                detail: format!(
                    "daily canopy date mismatch at day {}: forcing {} vs canopy {}",
                    index + 1,
                    forcing.date,
                    canopy.date
                ),
            });
        }
        require_unit_interval("cancov_daily_series", canopy.canopy_cover_fraction)?;
    }
    Ok(())
}

fn summarize_canopy_series(
    canopy_series: &[CanopySeriesDay],
) -> Result<SnowbenchCanopySeriesSummary, SnowbenchError> {
    let first = canopy_series
        .first()
        .ok_or_else(|| SnowbenchError::InvalidForcing {
            detail: "daily canopy series is empty".to_string(),
        })?
        .canopy_cover_fraction;
    let last = canopy_series
        .last()
        .ok_or_else(|| SnowbenchError::InvalidForcing {
            detail: "daily canopy series is empty".to_string(),
        })?
        .canopy_cover_fraction;
    let mut min = f64::INFINITY;
    let mut max = f64::NEG_INFINITY;
    let mut sum = 0.0;
    for day in canopy_series {
        require_unit_interval("cancov_daily_series", day.canopy_cover_fraction)?;
        min = min.min(day.canopy_cover_fraction);
        max = max.max(day.canopy_cover_fraction);
        sum += day.canopy_cover_fraction;
    }
    let day_count =
        u32::try_from(canopy_series.len()).map_err(|_| SnowbenchError::InvalidForcing {
            detail: format!(
                "daily canopy series length {} exceeds supported summary range",
                canopy_series.len()
            ),
        })?;
    Ok(SnowbenchCanopySeriesSummary {
        day_count: canopy_series.len(),
        min,
        max,
        mean: sum / f64::from(day_count),
        first,
        last,
        dynamic: (max - min).abs() > 1.0e-12,
    })
}

fn write_canopy_series_csv(
    path: &Path,
    canopy_series: &[CanopySeriesDay],
) -> Result<(), SnowbenchError> {
    let mut file = fs::File::create(path).map_err(|source| SnowbenchError::io(path, source))?;
    writeln!(file, "date,day_index,canopy_cover_fraction,source")
        .map_err(|source| SnowbenchError::io(path, source))?;
    for (day_index, day) in canopy_series.iter().enumerate() {
        writeln!(
            file,
            "{},{},{:.12},{}",
            day.date,
            day_index + 1,
            day.canopy_cover_fraction,
            CANOPY_SERIES_SOURCE
        )
        .map_err(|source| SnowbenchError::io(path, source))?;
    }
    Ok(())
}

fn build_daily_forcing(
    records: &[ClimateDailyRecord],
    climate: &openwepp_hillslope_orchestrator::runtime_inputs::HillslopeClimateRuntimeRequest,
    context: DirectWinterHourlyContext,
) -> Result<Vec<DailyForcingExport>, SnowbenchError> {
    let mut rows = Vec::with_capacity(records.len());
    let mut previous_serial = None;
    for (day_index, record) in records.iter().enumerate() {
        let date = record_date(record);
        validate_next_daily_date(previous_serial, date)?;
        previous_serial = Some(calendar_day_number(date)?);
        let day_forcing = climate.direct_day_forcing(day_index).map_err(|error| {
            SnowbenchError::ClimateRuntime {
                detail: error.to_string(),
            }
        })?;
        let hourly = climate
            .diagnostic_winter_hourly_forcing(day_index, context)
            .map_err(|error| SnowbenchError::ClimateRuntime {
                detail: error.to_string(),
            })?;
        rows.push(DailyForcingExport {
            date,
            wind_speed_m_s: day_forcing.vwind_m_s,
            dew_point_c: day_forcing.tdpt_c,
            hourly,
        });
    }
    Ok(rows)
}

fn record_date(record: &ClimateDailyRecord) -> CalendarDate {
    match record {
        ClimateDailyRecord::NoBreakpoint(day) => CalendarDate {
            year: day.year,
            month: day.mon,
            day: day.day,
        },
        ClimateDailyRecord::Breakpoint(day) => CalendarDate {
            year: day.year,
            month: day.mon,
            day: day.day,
        },
    }
}

fn validate_next_daily_date(
    previous_serial: Option<i64>,
    date: CalendarDate,
) -> Result<(), SnowbenchError> {
    let serial = calendar_day_number(date)?;
    if previous_serial.is_some_and(|previous| serial != previous + 1) {
        return Err(SnowbenchError::InvalidForcing {
            detail: format!(
                "climate forcing dates must be contiguous daily records; observed non-uniform date step before {date}"
            ),
        });
    }
    Ok(())
}

fn calendar_day_number(date: CalendarDate) -> Result<i64, SnowbenchError> {
    if !(1..=12).contains(&date.month) {
        return Err(SnowbenchError::InvalidForcing {
            detail: format!("calendar month out of range in {date}"),
        });
    }
    let max_day = days_in_month(date.year, date.month);
    if date.day < 1 || date.day > max_day {
        return Err(SnowbenchError::InvalidForcing {
            detail: format!("calendar day out of range in {date}"),
        });
    }
    let month_adjusted_year = i64::from(date.year) - i64::from(date.month <= 2);
    let era = if month_adjusted_year >= 0 {
        month_adjusted_year
    } else {
        month_adjusted_year - 399
    } / 400;
    let year_of_era = month_adjusted_year - era * 400;
    let month = i64::from(date.month);
    let day_of_year =
        (153 * (month + if month > 2 { -3 } else { 9 }) + 2) / 5 + i64::from(date.day) - 1;
    Ok(era * 146_097 + year_of_era * 365 + year_of_era / 4 - year_of_era / 100 + day_of_year)
}

fn days_in_month(year: i32, month: i32) -> i32 {
    match month {
        1 | 3 | 5 | 7 | 8 | 10 | 12 => 31,
        4 | 6 | 9 | 11 => 30,
        2 if is_leap_year(year) => 29,
        2 => 28,
        _ => 0,
    }
}

fn is_leap_year(year: i32) -> bool {
    (year % 4 == 0 && year % 100 != 0) || year % 400 == 0
}

impl fmt::Display for CalendarDate {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:04}-{:02}-{:02}", self.year, self.month, self.day)
    }
}

fn lane_rows(
    daily: &[DailyForcingExport],
    lane: GroundTempLane,
    snow_density_kg_m3: f64,
) -> Result<Vec<ExportRow>, SnowbenchError> {
    require_finite("snow.options.newsnw", snow_density_kg_m3)?;
    if snow_density_kg_m3 <= 0.0 {
        return Err(SnowbenchError::InvalidForcing {
            detail: format!("snow density must be positive, observed {snow_density_kg_m3}"),
        });
    }
    let mut rows = Vec::with_capacity(daily.len() * 24);
    for day in daily {
        let vapor_pressure_pa = saturation_vapor_pressure_kpa(day.dew_point_c) * 1_000.0;
        for (hour_index, forcing) in day.hourly.iter().enumerate() {
            let rain_mass_mm = forcing.rain_m * 1_000.0;
            let snow_mass_mm = forcing.snowfall_m * snow_density_kg_m3;
            let precip_mass_mm = rain_mass_mm + snow_mass_mm;
            let snow_precip_fraction = if precip_mass_mm > 0.0 {
                snow_mass_mm / precip_mass_mm
            } else {
                0.0
            };
            let row = ExportRow {
                date: day.date,
                hour_index,
                net_solar_w_m2: forcing.radiation_mj_m2 * 1_000_000.0 / 3_600.0
                    * NET_SHORTWAVE_FACTOR,
                downwelling_thermal_w_m2: diagnostic_longwave_w_m2(
                    forcing.air_temperature_c,
                    forcing.cloud_fraction,
                )?,
                temp_air_c: forcing.air_temperature_c,
                temp_ground_c: lane.temp_ground_c,
                vapor_pressure_pa,
                wind_speed_m_s: day.wind_speed_m_s,
                precip_mass_mm,
                snow_precip_mass_mm: snow_mass_mm,
                precip_temp_c: forcing.air_temperature_c,
                snow_precip_fraction,
                snow_precip_density_kg_m3: snow_density_kg_m3,
            };
            validate_row(&row)?;
            rows.push(row);
        }
    }
    Ok(rows)
}

fn diagnostic_longwave_w_m2(air_temp_c: f64, cloud_fraction: f64) -> Result<f64, SnowbenchError> {
    require_finite("winter.hourly.air_temp_c", air_temp_c)?;
    require_unit_interval("winter.hourly.cloud_fraction", cloud_fraction)?;
    let temp_k = air_temp_c + 273.15;
    let clear_sky_emissivity = 0.72;
    let cloud_adjusted_emissivity = (clear_sky_emissivity + 0.28 * cloud_fraction).clamp(0.0, 1.0);
    let value = cloud_adjusted_emissivity * STEFAN_BOLTZMANN_W_M2_K4 * temp_k.powi(4);
    require_finite("downwelling_thermal_Wm-2", value)?;
    Ok(value)
}

fn validate_row(row: &ExportRow) -> Result<(), SnowbenchError> {
    for (field, value) in [
        ("net_solar_Wm-2", row.net_solar_w_m2),
        ("downwelling_thermal_Wm-2", row.downwelling_thermal_w_m2),
        ("temp_air_degC", row.temp_air_c),
        ("temp_ground_degC", row.temp_ground_c),
        ("vapor_pressure_Pa", row.vapor_pressure_pa),
        ("wind_speed_ms-1", row.wind_speed_m_s),
        ("precip_mass_mm", row.precip_mass_mm),
        ("precip_temp_degC", row.precip_temp_c),
        ("snow_precip_fraction", row.snow_precip_fraction),
        ("snow_precip_density_kgm-3", row.snow_precip_density_kg_m3),
    ] {
        require_finite(field, value)?;
    }
    if row.net_solar_w_m2 < 0.0 || row.downwelling_thermal_w_m2 < 0.0 {
        return Err(SnowbenchError::InvalidForcing {
            detail: "radiation forcing must be non-negative".to_string(),
        });
    }
    if row.vapor_pressure_pa < 0.0 || row.wind_speed_m_s < 0.0 || row.precip_mass_mm < 0.0 {
        return Err(SnowbenchError::InvalidForcing {
            detail: "vapor pressure, wind speed, and precipitation must be non-negative"
                .to_string(),
        });
    }
    require_unit_interval("snow_precip_fraction", row.snow_precip_fraction)
}

fn require_finite(field: &'static str, value: f64) -> Result<(), SnowbenchError> {
    if value.is_finite() {
        Ok(())
    } else {
        Err(SnowbenchError::InvalidForcing {
            detail: format!("{field} is non-finite: {value}"),
        })
    }
}

fn require_unit_interval(field: &'static str, value: f64) -> Result<(), SnowbenchError> {
    require_finite(field, value)?;
    if (0.0..=1.0).contains(&value) {
        Ok(())
    } else {
        Err(SnowbenchError::InvalidForcing {
            detail: format!("{field} must be in [0,1], observed {value}"),
        })
    }
}

fn write_forcing_csv(path: &Path, rows: &[ExportRow]) -> Result<(), SnowbenchError> {
    let mut file = fs::File::create(path).map_err(|source| SnowbenchError::io(path, source))?;
    writeln!(file, "Datetime,{}", PYSNOBAL_FORCING_COLUMNS.join(","))
        .map_err(|source| SnowbenchError::io(path, source))?;
    for row in rows {
        writeln!(
            file,
            "{},{:.12},{:.12},{:.12},{:.12},{:.12},{:.12},{:.12},{:.12},{:.12},{:.12}",
            timestamp(row.date, row.hour_index),
            row.net_solar_w_m2,
            row.downwelling_thermal_w_m2,
            row.temp_air_c,
            row.temp_ground_c,
            row.vapor_pressure_pa,
            row.wind_speed_m_s,
            row.precip_mass_mm,
            row.precip_temp_c,
            row.snow_precip_fraction,
            row.snow_precip_density_kg_m3,
        )
        .map_err(|source| SnowbenchError::io(path, source))?;
    }
    Ok(())
}

fn timestamp(date: CalendarDate, hour_index: usize) -> String {
    format!(
        "{:04}-{:02}-{:02} {:02}:00:00",
        date.year, date.month, date.day, hour_index
    )
}

fn write_config_yaml(
    path: &Path,
    forcing_path: &Path,
    output_path: &Path,
    lane: GroundTempLane,
    elevation_m: f64,
) -> Result<(), SnowbenchError> {
    let payload = format!(
        "io:\n\
         \x20\x20forcing_path: \"{}\"\n\
         \x20\x20output_path: \"{}\"\n\
         z:\n\
         \x20\x20air_temp_m: {:.2}\n\
         \x20\x20soil_temp_m: {:.2}\n\
         \x20\x20wind_speed_m: {:.2}\n\
         params:\n\
         \x20\x20elevation_m: {:.3}\n\
         \x20\x20roughness_length_m: {:.4}\n\
         init:\n\
         \x20\x20snow_depth_m: null\n\
         \x20\x20bulk_snow_density_kgm-3: null\n\
         \x20\x20active_layer_temp_degC: null\n\
         \x20\x20avg_snow_temp_degC: null\n\
         \x20\x20h2o_sat_%: null\n\
         defaults:\n\
         \x20\x20relative_heights: null\n\
         \x20\x20max_h2o_vol_frac: null\n\
         \x20\x20max_active_layer_thickness_m: null\n\
         \x20\x20normal_tstep_mass_thresh_kgm-2: null\n\
         \x20\x20medium_tstep_mass_thresh_kgm-2: null\n\
         \x20\x20small_tstep_mass_thresh_kgm-2: null\n\
         \x20\x20normal_tstep_min: null\n\
         \x20\x20medium_tstep_min: null\n\
         \x20\x20small_tstep_min: null\n",
        yaml_path(forcing_path),
        yaml_path(output_path),
        DEFAULT_AIR_TEMP_HEIGHT_M,
        lane.soil_temp_depth_m,
        DEFAULT_WIND_SPEED_HEIGHT_M,
        elevation_m,
        DEFAULT_ROUGHNESS_LENGTH_M,
    );
    write_text(path, &payload)
}

fn yaml_path(path: &Path) -> String {
    path.display()
        .to_string()
        .replace('\\', "\\\\")
        .replace('"', "\\\"")
}

fn write_lineage_json(path: &Path, lane: GroundTempLane) -> Result<(), SnowbenchError> {
    let document = LineageDocument {
        schema: "snowfrost-fidelity-g0-pysnobal-lineage-v1",
        lane,
        fields: lineage_fields(),
    };
    write_json(path, &document)
}

#[allow(clippy::too_many_lines)]
fn lineage_fields() -> BTreeMap<&'static str, LineageField> {
    let mut fields = BTreeMap::new();
    fields.insert(
        "net_solar_Wm-2",
        LineageField {
            units: "W m^-2",
            source_class: "diagnostic-proxy",
            source: "SIMIMPL28 winter.hourly.rad_mj_m2 converted from hourly MJ m^-2 h^-1",
            conversion: "MJ m^-2 h^-1 * 1e6 / 3600 * fixed diagnostic net-shortwave factor 0.80",
            rejected_aliases: vec!["raw daily climate rad in langleys/day"],
        },
    );
    fields.insert(
        "downwelling_thermal_Wm-2",
        LineageField {
            units: "W m^-2",
            source_class: "diagnostic-proxy",
            source: "hourly air temperature plus SIMIMPL28 cloud fraction",
            conversion: "diagnostic Stefan-Boltzmann estimate with cloud-adjusted emissivity",
            rejected_aliases: vec!["net radiation", "shortwave radiation"],
        },
    );
    fields.insert(
        "temp_air_degC",
        LineageField {
            units: "degC",
            source_class: "mechanical",
            source: "SIMIMPL28 winter.hourly.air_temp_c",
            conversion: "none",
            rejected_aliases: vec!["daily tmax", "daily tmin"],
        },
    );
    fields.insert(
        "temp_ground_degC",
        LineageField {
            units: "degC",
            source_class: "diagnostic-proxy",
            source: "constant G0 sensitivity lane at z.soil_temp_m",
            conversion: "none",
            rejected_aliases: vec![
                "frost.hourly.surface_temp_c_####",
                "surtmp(hour)",
                "snow-surface temperature",
            ],
        },
    );
    fields.insert(
        "vapor_pressure_Pa",
        LineageField {
            units: "Pa",
            source_class: "deterministic-derived",
            source: "climate tdpt through openWEPP saturation_vapor_pressure_kpa",
            conversion: "kPa * 1000",
            rejected_aliases: vec![
                "relative humidity",
                "air-temperature saturation vapor pressure",
            ],
        },
    );
    fields.insert(
        "wind_speed_ms-1",
        LineageField {
            units: "m s^-1",
            source_class: "mechanical",
            source: "climate vwind repeated hourly",
            conversion: "none",
            rejected_aliases: vec!["wind direction degrees"],
        },
    );
    fields.insert(
        "precip_mass_mm",
        LineageField {
            units: "mm water equivalent",
            source_class: "deterministic-derived",
            source: "SIMIMPL28 hourly rain depth plus snowfall depth converted with snow.options.newsnw",
            conversion: "rain_m * 1000 + snowfall_depth_m * snow_density_kg_m3",
            rejected_aliases: vec!["snow.hourly.snowfall_m_#### as millimeters water equivalent"],
        },
    );
    fields.insert(
        "precip_temp_degC",
        LineageField {
            units: "degC",
            source_class: "diagnostic-proxy",
            source: "hourly air temperature",
            conversion: "none",
            rejected_aliases: vec!["ground temperature", "snow-surface temperature"],
        },
    );
    fields.insert(
        "snow_precip_fraction",
        LineageField {
            units: "unit interval",
            source_class: "deterministic-derived",
            source: "snow precipitation mass divided by total precipitation mass",
            conversion: "snow_mass_mm / precip_mass_mm when precip_mass_mm > 0 else 0",
            rejected_aliases: vec!["rain/snow branch flag"],
        },
    );
    fields.insert(
        "snow_precip_density_kgm-3",
        LineageField {
            units: "kg m^-3",
            source_class: "mechanical",
            source: "snow.options.newsnw from snow.txt or parser default",
            conversion: "none",
            rejected_aliases: vec!["WAT Snow-Water", "WAT Snow-Depth"],
        },
    );
    fields
}

fn audit_document(
    run_dir: &Path,
    runfile: &Path,
    lane: GroundTempLane,
    rows: &[ExportRow],
    day_count: usize,
) -> AuditDocument {
    AuditDocument {
        schema: "snowfrost-fidelity-g0-pysnobal-audit-v1",
        run_dir: run_dir.display().to_string(),
        runfile: runfile.display().to_string(),
        lane,
        rows: rows.len(),
        day_count,
        total_precip_mass_mm: rows.iter().map(|row| row.precip_mass_mm).sum(),
        total_snow_precip_mass_mm: rows.iter().map(|row| row.snow_precip_mass_mm).sum(),
        positive_snow_precip_rows: rows
            .iter()
            .filter(|row| row.snow_precip_mass_mm > 0.0)
            .count(),
        nonfinite_rows_rejected: true,
        negative_precip_rejected: true,
        uniform_hourly_timestamps: true,
        snowfall_depth_to_mass_conversion: "snow.hourly.snowfall_m depth * snow.options.newsnw kg/m3 = kg/m2 = mm water equivalent",
        wat_snow_water_is_not_depth: true,
        frost_surface_temp_is_not_ground_temp: true,
        daily_radiation_not_exported_as_hourly_wm2: true,
    }
}

fn write_audit_markdown(path: &Path, audit: &AuditDocument) -> Result<(), SnowbenchError> {
    let text = format!(
        "# PySnobal Export Audit\n\n\
         - Schema: `{}`\n\
         - Lane: `{}`\n\
         - Rows: `{}`\n\
         - Days: `{}`\n\
         - Total precipitation mass: `{:.6}` mm\n\
         - Total snow precipitation mass: `{:.6}` mm\n\
         - Positive snow precipitation rows: `{}`\n\
         - Snowfall conversion: `{}`\n\
         - WAT Snow-Water rejected as depth: `{}`\n\
         - Frost surface temperature rejected as ground temperature: `{}`\n",
        audit.schema,
        audit.lane.id,
        audit.rows,
        audit.day_count,
        audit.total_precip_mass_mm,
        audit.total_snow_precip_mass_mm,
        audit.positive_snow_precip_rows,
        audit.snowfall_depth_to_mass_conversion,
        audit.wat_snow_water_is_not_depth,
        audit.frost_surface_temp_is_not_ground_temp,
    );
    write_text(path, &text)
}

fn write_openwepp_snow_projection(
    output_dir: &Path,
    request: &HillslopeRunRequest,
    daily_forcing: &[DailyForcingExport],
) -> Result<(), SnowbenchError> {
    let argv = vec![
        "openwepp-snowbench".to_string(),
        "export-pysnobal".to_string(),
        "--diagnostic-openwepp-snow".to_string(),
    ];
    let report = super::execute_hillslope_run_with_runtime_selection(
        request,
        &argv,
        HillslopeRuntimeSelection::Compatibility,
    )?;
    let wat_path = report
        .optional_outputs
        .iter()
        .find(|path| {
            path.file_name().and_then(|value| value.to_str()) == Some("snowbench.wat.parquet")
        })
        .cloned()
        .unwrap_or_else(|| request.output_dir.join("snowbench.wat.parquet"));
    export_openwepp_snow_csv_from_wat_with_dates(&wat_path, output_dir, Some(daily_forcing))?;
    Ok(())
}

pub fn export_openwepp_snow_csv_from_wat(
    wat_path: &Path,
    output_dir: &Path,
) -> Result<usize, SnowbenchError> {
    export_openwepp_snow_csv_from_wat_with_dates(wat_path, output_dir, None)
}

fn export_openwepp_snow_csv_from_wat_with_dates(
    wat_path: &Path,
    output_dir: &Path,
    daily_forcing: Option<&[DailyForcingExport]>,
) -> Result<usize, SnowbenchError> {
    let rows = read_openwepp_snow_rows(wat_path)?;
    write_openwepp_snow_csv(&output_dir.join("openwepp_snow.csv"), &rows, daily_forcing)?;
    let availability = OpenweppSnowAvailability {
        schema: "snowfrost-fidelity-g1-openwepp-snow-availability-v1",
        status: "EXPORTED_FROM_COMPATIBILITY_WAT",
        reason: "G1 executes the generated diagnostic run through the existing compatibility publication path and extracts WAT Snow-Water/Snow-Depth rows for PySnobal comparison; this is an output-surface projection, not a new snow calculation.".to_string(),
        source_wat_parquet: Some(wat_path.display().to_string()),
        row_count: rows.len(),
    };
    write_json(
        &output_dir.join("openwepp_snow_availability.json"),
        &availability,
    )?;
    Ok(rows.len())
}

fn write_openwepp_snow_placeholder(output_dir: &Path) -> Result<(), SnowbenchError> {
    write_text(
        &output_dir.join("openwepp_snow.csv"),
        "date,sim_day_index,Snow-Water_mm,Snow-Depth_mm,source\n",
    )?;
    let availability = OpenweppSnowAvailability {
        schema: "snowfrost-fidelity-g1-openwepp-snow-availability-v1",
        status: "NOT_REQUESTED",
        reason: "This export disabled compatibility WAT snow projection for a focused schema test. The openwepp-snowbench CLI enables it for diagnostic comparator runs.".to_string(),
        source_wat_parquet: None,
        row_count: 0,
    };
    write_json(
        &output_dir.join("openwepp_snow_availability.json"),
        &availability,
    )
}

fn read_openwepp_snow_rows(path: &Path) -> Result<Vec<OpenweppSnowRow>, SnowbenchError> {
    let file = File::open(path).map_err(|source| SnowbenchError::OpenweppSnow {
        detail: format!("failed opening WAT parquet {}: {source}", path.display()),
    })?;
    let builder = ParquetRecordBatchReaderBuilder::try_new(file).map_err(|error| {
        SnowbenchError::OpenweppSnow {
            detail: format!(
                "failed reading WAT parquet metadata {}: {error}",
                path.display()
            ),
        }
    })?;
    let reader = builder
        .build()
        .map_err(|error| SnowbenchError::OpenweppSnow {
            detail: format!(
                "failed building WAT parquet reader {}: {error}",
                path.display()
            ),
        })?;

    let mut rows = Vec::new();
    for batch_result in reader {
        let batch = batch_result.map_err(|error| SnowbenchError::OpenweppSnow {
            detail: format!(
                "failed reading WAT parquet batch {}: {error}",
                path.display()
            ),
        })?;
        append_openwepp_snow_batch(path, &batch, &mut rows)?;
    }
    if rows.is_empty() {
        return Err(SnowbenchError::OpenweppSnow {
            detail: format!("WAT parquet {} contained no snow rows", path.display()),
        });
    }
    Ok(rows)
}

fn append_openwepp_snow_batch(
    path: &Path,
    batch: &RecordBatch,
    rows: &mut Vec<OpenweppSnowRow>,
) -> Result<(), SnowbenchError> {
    let years = int16_column(path, batch, "year")?;
    let months = int8_column(path, batch, "month")?;
    let days = int8_column(path, batch, "day_of_month")?;
    let sim_days = int32_column(path, batch, "sim_day_index")?;
    let snow_water = f64_column(path, batch, "Snow-Water")?;
    let snow_depth = optional_f64_column(path, batch, "Snow-Depth")?;
    for row_index in 0..batch.num_rows() {
        rows.push(OpenweppSnowRow {
            year: int16_value(path, "year", years, row_index)?,
            month: int8_value(path, "month", months, row_index)?,
            day_of_month: int8_value(path, "day_of_month", days, row_index)?,
            sim_day_index: int32_value(path, "sim_day_index", sim_days, row_index)?,
            snow_water_mm: f64_value(path, "Snow-Water", snow_water, row_index)?,
            snow_depth_mm: optional_f64_value(path, "Snow-Depth", snow_depth, row_index)?,
        });
    }
    Ok(())
}

fn write_openwepp_snow_csv(
    path: &Path,
    rows: &[OpenweppSnowRow],
    daily_forcing: Option<&[DailyForcingExport]>,
) -> Result<(), SnowbenchError> {
    let mut file = fs::File::create(path).map_err(|source| SnowbenchError::io(path, source))?;
    writeln!(
        file,
        "date,sim_day_index,Snow-Water_mm,Snow-Depth_mm,source"
    )
    .map_err(|source| SnowbenchError::io(path, source))?;
    for row in rows {
        let date = openwepp_snow_row_date(row, daily_forcing)?;
        let snow_depth = row
            .snow_depth_mm
            .map(|value| format!("{value:.12}"))
            .unwrap_or_default();
        writeln!(
            file,
            "{},{},{:.12},{},openwepp_compatibility_wat",
            date, row.sim_day_index, row.snow_water_mm, snow_depth,
        )
        .map_err(|source| SnowbenchError::io(path, source))?;
    }
    Ok(())
}

fn openwepp_snow_row_date(
    row: &OpenweppSnowRow,
    daily_forcing: Option<&[DailyForcingExport]>,
) -> Result<String, SnowbenchError> {
    if let Some(daily_forcing) = daily_forcing {
        let index =
            usize::try_from(row.sim_day_index - 1).map_err(|_| SnowbenchError::OpenweppSnow {
                detail: format!("sim_day_index must be >= 1, observed {}", row.sim_day_index),
            })?;
        let Some(day) = daily_forcing.get(index) else {
            return Err(SnowbenchError::OpenweppSnow {
                detail: format!(
                    "sim_day_index {} exceeds climate date count {}",
                    row.sim_day_index,
                    daily_forcing.len()
                ),
            });
        };
        return Ok(day.date.to_string());
    }
    Ok(format!(
        "{:04}-{:02}-{:02}",
        row.year, row.month, row.day_of_month
    ))
}

fn int8_column<'a>(
    path: &Path,
    batch: &'a RecordBatch,
    name: &str,
) -> Result<&'a Int8Array, SnowbenchError> {
    column(path, batch, name)
}

fn int16_column<'a>(
    path: &Path,
    batch: &'a RecordBatch,
    name: &str,
) -> Result<&'a Int16Array, SnowbenchError> {
    column(path, batch, name)
}

fn int32_column<'a>(
    path: &Path,
    batch: &'a RecordBatch,
    name: &str,
) -> Result<&'a Int32Array, SnowbenchError> {
    column(path, batch, name)
}

fn f64_column<'a>(
    path: &Path,
    batch: &'a RecordBatch,
    name: &str,
) -> Result<&'a Float64Array, SnowbenchError> {
    column(path, batch, name)
}

fn optional_f64_column<'a>(
    path: &Path,
    batch: &'a RecordBatch,
    name: &str,
) -> Result<Option<&'a Float64Array>, SnowbenchError> {
    let schema = batch.schema();
    let Ok(index) = schema.index_of(name) else {
        return Ok(None);
    };
    batch
        .column(index)
        .as_any()
        .downcast_ref::<Float64Array>()
        .map(Some)
        .ok_or_else(|| SnowbenchError::OpenweppSnow {
            detail: format!(
                "WAT parquet {} column {name} has unsupported type",
                path.display()
            ),
        })
}

fn column<'a, T: 'static>(
    path: &Path,
    batch: &'a RecordBatch,
    name: &str,
) -> Result<&'a T, SnowbenchError> {
    let schema = batch.schema();
    let index = schema
        .index_of(name)
        .map_err(|_| SnowbenchError::OpenweppSnow {
            detail: format!(
                "WAT parquet {} is missing required column {name}",
                path.display()
            ),
        })?;
    batch
        .column(index)
        .as_any()
        .downcast_ref::<T>()
        .ok_or_else(|| SnowbenchError::OpenweppSnow {
            detail: format!(
                "WAT parquet {} column {name} has unsupported type",
                path.display()
            ),
        })
}

fn int8_value(
    path: &Path,
    column_name: &str,
    column: &Int8Array,
    row_index: usize,
) -> Result<i8, SnowbenchError> {
    if column.is_null(row_index) {
        return Err(null_openwepp_snow_value(path, column_name, row_index));
    }
    Ok(column.value(row_index))
}

fn int16_value(
    path: &Path,
    column_name: &str,
    column: &Int16Array,
    row_index: usize,
) -> Result<i16, SnowbenchError> {
    if column.is_null(row_index) {
        return Err(null_openwepp_snow_value(path, column_name, row_index));
    }
    Ok(column.value(row_index))
}

fn int32_value(
    path: &Path,
    column_name: &str,
    column: &Int32Array,
    row_index: usize,
) -> Result<i32, SnowbenchError> {
    if column.is_null(row_index) {
        return Err(null_openwepp_snow_value(path, column_name, row_index));
    }
    Ok(column.value(row_index))
}

fn f64_value(
    path: &Path,
    column_name: &str,
    column: &Float64Array,
    row_index: usize,
) -> Result<f64, SnowbenchError> {
    if column.is_null(row_index) {
        return Err(null_openwepp_snow_value(path, column_name, row_index));
    }
    let value = column.value(row_index);
    if value.is_finite() {
        Ok(value)
    } else {
        Err(SnowbenchError::OpenweppSnow {
            detail: format!(
                "WAT parquet {} column {column_name} row {row_index} is non-finite: {value}",
                path.display()
            ),
        })
    }
}

fn optional_f64_value(
    path: &Path,
    column_name: &str,
    column: Option<&Float64Array>,
    row_index: usize,
) -> Result<Option<f64>, SnowbenchError> {
    let Some(column) = column else {
        return Ok(None);
    };
    if column.is_null(row_index) {
        return Ok(None);
    }
    f64_value(path, column_name, column, row_index).map(Some)
}

fn null_openwepp_snow_value(path: &Path, column_name: &str, row_index: usize) -> SnowbenchError {
    SnowbenchError::OpenweppSnow {
        detail: format!(
            "WAT parquet {} column {column_name} row {row_index} is null",
            path.display()
        ),
    }
}

fn write_json<T: Serialize>(path: &Path, value: &T) -> Result<(), SnowbenchError> {
    let text = serde_json::to_string_pretty(value).map_err(|source| SnowbenchError::Json {
        path: path.to_path_buf(),
        source,
    })?;
    write_text(path, &(text + "\n"))
}

fn write_text(path: &Path, text: &str) -> Result<(), SnowbenchError> {
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent).map_err(|source| SnowbenchError::io(parent, source))?;
    }
    fs::write(path, text).map_err(|source| SnowbenchError::io(path, source))
}

#[cfg(test)]
mod tests {
    use openwepp_hillslope_orchestrator::runtime_inputs::DirectWinterHourlyForcing;

    use super::{
        CalendarDate, DailyForcingExport, OpenweppSnowRow, calendar_day_number,
        openwepp_snow_row_date, validate_next_daily_date,
    };

    #[test]
    fn date_continuity_accepts_leap_day_sequence() {
        let first = CalendarDate {
            year: 2020,
            month: 2,
            day: 28,
        };
        let leap = CalendarDate {
            year: 2020,
            month: 2,
            day: 29,
        };
        let march = CalendarDate {
            year: 2020,
            month: 3,
            day: 1,
        };
        validate_next_daily_date(None, first).expect("first date should be valid");
        validate_next_daily_date(Some(calendar_day_number(first).unwrap()), leap)
            .expect("leap day should follow Feb 28 in a leap year");
        validate_next_daily_date(Some(calendar_day_number(leap).unwrap()), march)
            .expect("Mar 1 should follow leap day");
    }

    #[test]
    fn date_continuity_rejects_non_uniform_daily_step() {
        let first = CalendarDate {
            year: 2024,
            month: 1,
            day: 1,
        };
        let skipped = CalendarDate {
            year: 2024,
            month: 1,
            day: 3,
        };
        let error = validate_next_daily_date(Some(calendar_day_number(first).unwrap()), skipped)
            .expect_err("skipped daily date must be rejected");
        assert!(error.to_string().contains("contiguous daily records"));
    }

    #[test]
    fn date_continuity_rejects_invalid_calendar_day() {
        let invalid = CalendarDate {
            year: 2023,
            month: 2,
            day: 29,
        };
        let error = validate_next_daily_date(None, invalid)
            .expect_err("invalid non-leap Feb 29 must be rejected");
        assert!(error.to_string().contains("calendar day out of range"));
    }

    #[test]
    fn openwepp_snow_projection_uses_climate_date_for_sim_day_index() {
        let daily = [
            sample_daily_forcing(CalendarDate {
                year: 1980,
                month: 2,
                day: 28,
            }),
            sample_daily_forcing(CalendarDate {
                year: 1980,
                month: 2,
                day: 29,
            }),
        ];
        let row = OpenweppSnowRow {
            year: 1,
            month: 2,
            day_of_month: 29,
            sim_day_index: 2,
            snow_water_mm: 42.0,
            snow_depth_mm: Some(210.0),
        };

        let date = openwepp_snow_row_date(&row, Some(&daily))
            .expect("sim day should map to external climate date");

        assert_eq!(date, "1980-02-29");
    }

    fn sample_daily_forcing(date: CalendarDate) -> DailyForcingExport {
        DailyForcingExport {
            date,
            wind_speed_m_s: 0.0,
            dew_point_c: 0.0,
            hourly: [DirectWinterHourlyForcing::zero(); 24],
        }
    }
}
