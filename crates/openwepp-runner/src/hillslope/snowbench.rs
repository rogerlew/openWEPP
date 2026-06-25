use std::collections::BTreeMap;
use std::error::Error;
use std::fmt;
use std::fs;
use std::io::{self, Write};
use std::path::{Path, PathBuf};

use openwepp_hillslope_orchestrator::runtime_inputs::{
    DirectWinterHourlyContext, DirectWinterHourlyForcing, build_hillslope_climate_runtime_request,
};
use openwepp_input_contract::parsers::climate::ClimateDailyRecord;
use openwepp_kernel_contract::{BoundarySymbol, BoundaryValue};
use serde::Serialize;

use crate::api::HillslopeRunRequest;
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

#[derive(Debug, Clone)]
pub struct SnowbenchExportRequest {
    pub run_dir: PathBuf,
    pub run_file: Option<PathBuf>,
    pub output_dir: PathBuf,
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
    pub lane_ids: Vec<&'static str>,
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
            | Self::InvalidForcing { .. } => None,
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
    reason: &'static str,
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
    let static_parts =
        super::build_static_runtime_surface_parts(&hillslope_request, &inputs, &mut sidecars)?;
    let climate_request =
        build_hillslope_climate_runtime_request(&inputs.climate).map_err(|error| {
            SnowbenchError::ClimateRuntime {
                detail: error.to_string(),
            }
        })?;
    let context = winter_context_from_surface(&static_parts.runtime_surface.state_surface)?;
    let daily_forcing =
        build_daily_forcing(&inputs.climate.daily_records, &climate_request, context)?;
    let snow_density = require_state_scalar(
        &static_parts.snow_surface.state_surface,
        "snow.options.newsnw",
    )?;

    write_openwepp_snow_placeholder(&output_dir)?;

    let mut report_total_precip = 0.0;
    let mut report_total_snow = 0.0;
    for lane in GROUND_TEMP_LANES {
        let rows = lane_rows(&daily_forcing, lane, snow_density)?;
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
            &request.run_dir,
            &generated_runfile,
            lane,
            &rows,
            daily_forcing.len(),
        );
        write_json(&lane_dir.join("audit.json"), &audit)?;
        write_audit_markdown(&lane_dir.join("audit.md"), &audit)?;
        report_total_precip = audit.total_precip_mass_mm;
        report_total_snow = audit.total_snow_precip_mass_mm;
    }

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

fn write_openwepp_snow_placeholder(output_dir: &Path) -> Result<(), SnowbenchError> {
    write_text(
        &output_dir.join("openwepp_snow.csv"),
        "date,Snow-Water_mm,Snow-Depth_mm,source\n",
    )?;
    let availability = OpenweppSnowAvailability {
        schema: "snowfrost-fidelity-g0-openwepp-snow-availability-v1",
        status: "NOT_EXPORTED_BY_G0",
        reason: "G0 reads WEPP inputs and exports PySnobal forcing. It does not run openWEPP; harness comparisons use openWEPP rows only when a future exporter supplies this CSV.",
    };
    write_json(
        &output_dir.join("openwepp_snow_availability.json"),
        &availability,
    )
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
    use super::{CalendarDate, calendar_day_number, validate_next_daily_date};

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
}
