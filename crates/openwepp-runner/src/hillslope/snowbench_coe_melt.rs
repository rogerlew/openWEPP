use std::collections::BTreeMap;
use std::fs;
use std::io::{self, Write};
use std::path::{Path, PathBuf};

use openwepp_hillslope_orchestrator::{
    DirectActiveSnowPartitionInputs, DirectSnowHourlyForcing, SnowAlbedoModel, SnowAlbedoState,
    SnowDensityModel, SnowMeltModel, Wb11HydrologyKernel,
};
use openwepp_input_contract::parsers::snow::{
    ParseMode as SnowParseMode, SnowParseOptions, parse_snow_file,
};
use serde::Serialize;

use super::snowbench::{
    PYSNOBAL_FORCING_COLUMNS, SnowbenchCanopySeriesSummary, SnowbenchError, SnowbenchExportRequest,
    export_pysnobal_inputs,
};

const CONTRACT: &str = "SC-SNOWFREEZE-001 INV-SNOWFREEZE-050 INV-SNOWFREEZE-052 INV-SNOWFREEZE-055 INV-SNOWFREEZE-057 INV-SNOWFREEZE-066 INV-SNOWFREEZE-073";
const RADIATION_BRIDGE_NET_SHORTWAVE_FACTOR: f64 = 0.80;
const STEFAN_BOLTZMANN_W_M2_K4: f64 = 5.670_374_419e-8;
const DEFAULT_UNDERLYING_SURFACE_ALBEDO: f64 = 0.2;
const DAILY_SWE_CLOSURE_TOLERANCE_M: f64 = 1.0e-9;

#[derive(Debug, Clone)]
pub struct CoeMeltRequest {
    pub run_dir: PathBuf,
    pub run_file: Option<PathBuf>,
    pub output_dir: PathBuf,
    pub model: CoeMeltModel,
}

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Serialize)]
pub enum CoeMeltModel {
    #[default]
    LegacyCoe,
    CoeShortwaveAlbedoV1,
    CoeWinterThawStateLossV1,
    CoeLiquidHoldingCapacityV1,
    CoeOpenSublimationStageAV1,
    CoeOpenSublimationStageBV1,
}

#[derive(Debug, Clone, Serialize)]
pub struct CoeMeltReport {
    pub schema: &'static str,
    pub model_id: &'static str,
    pub contract: &'static str,
    pub runtime_coupling: &'static str,
    pub no_site_constants: bool,
    pub output_dir: String,
    pub forcing_bridge_dir: String,
    pub canopy_source: &'static str,
    pub canopy_series_path: String,
    pub canopy_series_summary: SnowbenchCanopySeriesSummary,
    pub shortwave_source: &'static str,
    pub shortwave_bridge_identity: &'static str,
    pub shortwave_bridge_like_for_like: bool,
    pub day_count: usize,
    pub hourly_row_count: usize,
    pub positive_snow_hours: usize,
    pub constants: CoeMeltConstants,
    pub summary: CoeMeltSummary,
}

#[derive(Debug, Clone, Copy, Serialize)]
pub struct CoeMeltConstants {
    pub canopy_cover_fraction: f64,
    pub underlying_surface_albedo: f64,
    pub radiation_bridge_net_shortwave_factor: f64,
}

#[derive(Debug, Clone, Serialize)]
pub struct CoeMeltSummary {
    pub total_snow_input_m: f64,
    pub total_rain_input_m: f64,
    pub total_raw_melt_m: f64,
    pub total_redistributed_melt_m: f64,
    pub total_routed_melt_m: f64,
    pub total_swe_loss_m: f64,
    pub total_sublimation_m: f64,
    pub total_liquid_water_released_m: f64,
    pub final_swe_m: f64,
    pub final_depth_m: f64,
    pub final_density_kg_m3: f64,
    pub final_liquid_holding_capacity_m: f64,
    pub final_liquid_water_retained_m: f64,
    pub final_settle_day_count: f64,
    pub final_snow_albedo: Option<f64>,
    pub diagnostic_initial_albedo_seed_count: usize,
}

#[derive(Debug, Clone)]
struct CoeMeltHourlyForcing {
    date: String,
    hour_index: usize,
    rain_m: f64,
    snow_water_m: f64,
    snowfall_depth_m: f64,
    radiation_mj_m2: f64,
    air_temperature_c: f64,
    cloud_fraction: f64,
    dewpoint_c: f64,
    wind_m_s: f64,
}

#[derive(Debug, Clone)]
struct CoeMeltDayForcing {
    date: String,
    hourly: [DirectSnowHourlyForcing; 24],
    canopy_cover_fraction: f64,
    dewpoint_c: f64,
    wind_m_s: f64,
    tmax_c: f64,
    tmin_c: f64,
    rain_m: f64,
    hyetograph_precip_m: f64,
    snow_input_m: f64,
}

#[derive(Debug, Clone, Serialize)]
struct CoeMeltDailyRow {
    date: String,
    snow_water_before_m: f64,
    snow_input_m: f64,
    rain_input_m: f64,
    rain_retained_m: f64,
    rain_released_m: f64,
    liquid_holding_capacity_m: f64,
    liquid_water_retained_m: f64,
    liquid_water_released_m: f64,
    snow_water_m: f64,
    snow_depth_m: f64,
    snow_density_kg_m3: f64,
    raw_melt_m: f64,
    redistributed_melt_m: f64,
    routed_melt_m: f64,
    snowpack_swe_loss_m: f64,
    sublimation_m: f64,
    snowpack_swe_balance_residual_m: f64,
    routed_state_loss_residual_m: f64,
    state_loss_available_storage_margin_m: f64,
    snow_albedo: Option<f64>,
    source: &'static str,
}

#[derive(Debug, Clone)]
struct CoeMeltSimulation {
    daily_rows: Vec<CoeMeltDailyRow>,
    summary: CoeMeltSummary,
    positive_snow_hours: usize,
}

#[derive(Debug, Default)]
struct CoeMeltLedger {
    total_snow_input_m: f64,
    total_rain_input_m: f64,
    total_raw_melt_m: f64,
    total_redistributed_melt_m: f64,
    total_routed_melt_m: f64,
    total_swe_loss_m: f64,
    total_sublimation_m: f64,
    total_liquid_water_released_m: f64,
    positive_snow_hours: usize,
    diagnostic_initial_albedo_seed_count: usize,
}

impl CoeMeltModel {
    #[must_use]
    pub const fn name(self) -> &'static str {
        match self {
            Self::LegacyCoe => "legacy_coe",
            Self::CoeShortwaveAlbedoV1 => "coe_shortwave_albedo_v1",
            Self::CoeWinterThawStateLossV1 => "coe_winter_thaw_state_loss_v1",
            Self::CoeLiquidHoldingCapacityV1 => "coe_liquid_holding_capacity_v1",
            Self::CoeOpenSublimationStageAV1 => "coe_open_sublimation_stage_a_v1",
            Self::CoeOpenSublimationStageBV1 => "coe_open_sublimation_stage_b_v1",
        }
    }

    pub fn parse(value: &str) -> Result<Self, SnowbenchError> {
        match value {
            "legacy_coe" => Ok(Self::LegacyCoe),
            "coe_shortwave_albedo_v1" => Ok(Self::CoeShortwaveAlbedoV1),
            "coe_winter_thaw_state_loss_v1" => Ok(Self::CoeWinterThawStateLossV1),
            "coe_liquid_holding_capacity_v1" => Ok(Self::CoeLiquidHoldingCapacityV1),
            "coe_open_sublimation_stage_a_v1" => Ok(Self::CoeOpenSublimationStageAV1),
            "coe_open_sublimation_stage_b_v1" => Ok(Self::CoeOpenSublimationStageBV1),
            _ => Err(SnowbenchError::InvalidInput {
                detail: format!(
                    "unknown CoE melt model '{value}', expected legacy_coe, coe_shortwave_albedo_v1, coe_winter_thaw_state_loss_v1, coe_liquid_holding_capacity_v1, coe_open_sublimation_stage_a_v1, or coe_open_sublimation_stage_b_v1"
                ),
            }),
        }
    }

    const fn snow_melt_model(self) -> SnowMeltModel {
        match self {
            Self::LegacyCoe => SnowMeltModel::LegacyCoe,
            Self::CoeShortwaveAlbedoV1 => SnowMeltModel::CoeShortwaveAlbedoV1,
            Self::CoeWinterThawStateLossV1 => SnowMeltModel::CoeWinterThawStateLossV1,
            Self::CoeLiquidHoldingCapacityV1 => SnowMeltModel::CoeLiquidHoldingCapacityV1,
            Self::CoeOpenSublimationStageAV1 => SnowMeltModel::CoeOpenSublimationStageAV1,
            Self::CoeOpenSublimationStageBV1 => SnowMeltModel::CoeOpenSublimationStageBV1,
        }
    }

    const fn snow_albedo_model(self) -> Option<SnowAlbedoModel> {
        match self {
            Self::LegacyCoe
            | Self::CoeWinterThawStateLossV1
            | Self::CoeLiquidHoldingCapacityV1
            | Self::CoeOpenSublimationStageAV1
            | Self::CoeOpenSublimationStageBV1 => None,
            Self::CoeShortwaveAlbedoV1 => Some(SnowAlbedoModel::Brock2000TemperatureAgeV1),
        }
    }
}

pub fn run_coe_melt_snowbench(request: &CoeMeltRequest) -> Result<CoeMeltReport, SnowbenchError> {
    let output_dir = absolute_path(&request.output_dir)?;
    fs::create_dir_all(&output_dir).map_err(|source| snowbench_io(&output_dir, source))?;
    let forcing_bridge_dir = output_dir.join("forcing_bridge");
    let export_report = export_pysnobal_inputs(&SnowbenchExportRequest {
        run_dir: request.run_dir.clone(),
        run_file: request.run_file.clone(),
        output_dir: forcing_bridge_dir.clone(),
        include_openwepp_snow_projection: false,
    })?;
    let snow = parse_snow_file(
        absolute_path(&request.run_dir)?.join("snow.txt"),
        SnowParseOptions {
            mode: SnowParseMode::Compatibility,
        },
    )
    .map_err(|error| SnowbenchError::InvalidInput {
        detail: format!("failed parsing diagnostic snow.txt for CoE melt replay: {error}"),
    })?;
    let forcing_csv = forcing_bridge_dir.join("tg_0p0c_zg0p10m/forcing.csv");
    let hourly = read_coe_melt_forcing(&forcing_csv)?;
    let canopy_by_date = read_canopy_series(&PathBuf::from(&export_report.canopy_series_path))?;
    let days = group_daily_forcing(hourly, &canopy_by_date)?;
    let simulation = simulate_coe_melt(&days, request.model, snow.rst, snow.newsnw, snow.ssd)?;
    write_coe_melt_csv(
        &output_dir.join("coe_melt_snow.csv"),
        &simulation.daily_rows,
    )?;
    let report = CoeMeltReport {
        schema: "snowdensity05g-coe-melt-snowbench-v1",
        model_id: request.model.name(),
        contract: CONTRACT,
        runtime_coupling: "diagnostic snowbench replay of typed CoE melt path; no production activation",
        no_site_constants: true,
        output_dir: output_dir.display().to_string(),
        forcing_bridge_dir: export_report.output_dir,
        canopy_source: export_report.canopy_source,
        canopy_series_path: export_report.canopy_series_path,
        canopy_series_summary: export_report.canopy_series_summary,
        shortwave_source: "pysnobal_bridge_inversion_of_openwepp_winter_hourly_rad_mj_m2",
        shortwave_bridge_identity: "net_solar_Wm-2 = hrrad_MJ_m-2_h-1 * 1000000 / 3600 * 0.8; replay hrrad_MJ_m-2_h-1 = net_solar_Wm-2 * 3600 / 1000000 / 0.8",
        shortwave_bridge_like_for_like: true,
        day_count: simulation.daily_rows.len(),
        hourly_row_count: export_report.hourly_row_count,
        positive_snow_hours: simulation.positive_snow_hours,
        constants: CoeMeltConstants {
            canopy_cover_fraction: export_report.canopy_series_summary.mean,
            underlying_surface_albedo: DEFAULT_UNDERLYING_SURFACE_ALBEDO,
            radiation_bridge_net_shortwave_factor: RADIATION_BRIDGE_NET_SHORTWAVE_FACTOR,
        },
        summary: simulation.summary,
    };
    write_json(&output_dir.join("coe_melt_summary.json"), &report)?;
    write_markdown(&output_dir.join("coe_melt_summary.md"), &report)?;
    Ok(report)
}

fn absolute_path(path: &Path) -> Result<PathBuf, SnowbenchError> {
    if path.is_absolute() {
        return Ok(path.to_path_buf());
    }
    let cwd = std::env::current_dir().map_err(|source| snowbench_io(".", source))?;
    Ok(cwd.join(path))
}

fn read_coe_melt_forcing(path: &Path) -> Result<Vec<CoeMeltHourlyForcing>, SnowbenchError> {
    let text = fs::read_to_string(path).map_err(|source| snowbench_io(path, source))?;
    let mut lines = text.lines();
    let header = lines.next().ok_or_else(|| SnowbenchError::InvalidForcing {
        detail: format!("{} missing header", path.display()),
    })?;
    let expected_header = format!("Datetime,{}", PYSNOBAL_FORCING_COLUMNS.join(","));
    if header != expected_header {
        return Err(SnowbenchError::InvalidForcing {
            detail: format!(
                "{} has unexpected header '{header}', expected '{expected_header}'",
                path.display()
            ),
        });
    }
    let mut rows = Vec::new();
    for (line_index, line) in lines.enumerate() {
        if line.trim().is_empty() {
            continue;
        }
        rows.push(parse_forcing_line(path, line_index + 2, line)?);
    }
    if rows.is_empty() {
        return Err(SnowbenchError::InvalidForcing {
            detail: format!("{} contained no forcing rows", path.display()),
        });
    }
    Ok(rows)
}

fn read_canopy_series(path: &Path) -> Result<BTreeMap<String, f64>, SnowbenchError> {
    let text = fs::read_to_string(path).map_err(|source| snowbench_io(path, source))?;
    let mut lines = text.lines();
    let header = lines.next().ok_or_else(|| SnowbenchError::InvalidForcing {
        detail: format!("{} missing header", path.display()),
    })?;
    let expected_header = "date,day_index,canopy_cover_fraction,source";
    if header != expected_header {
        return Err(SnowbenchError::InvalidForcing {
            detail: format!(
                "{} has unexpected canopy header '{header}', expected '{expected_header}'",
                path.display()
            ),
        });
    }
    let mut rows = BTreeMap::new();
    let mut expected_day_index = 1_usize;
    for (line_index, line) in lines.enumerate() {
        if line.trim().is_empty() {
            continue;
        }
        let columns = line.split(',').collect::<Vec<_>>();
        if columns.len() != 4 {
            return Err(SnowbenchError::InvalidForcing {
                detail: format!(
                    "{} line {} has {} canopy columns, expected 4",
                    path.display(),
                    line_index + 2,
                    columns.len()
                ),
            });
        }
        let day_index =
            columns[1]
                .parse::<usize>()
                .map_err(|error| SnowbenchError::InvalidForcing {
                    detail: format!(
                        "{} line {} canopy day_index is not numeric '{}': {error}",
                        path.display(),
                        line_index + 2,
                        columns[1]
                    ),
                })?;
        if day_index != expected_day_index {
            return Err(SnowbenchError::InvalidForcing {
                detail: format!(
                    "{} line {} canopy day_index {}, expected {}",
                    path.display(),
                    line_index + 2,
                    day_index,
                    expected_day_index
                ),
            });
        }
        expected_day_index += 1;
        let canopy_cover_fraction =
            parse_column(path, line_index + 2, "canopy_cover_fraction", columns[2])?;
        require_unit_interval(
            path,
            line_index + 2,
            "canopy_cover_fraction",
            canopy_cover_fraction,
        )?;
        if rows
            .insert(columns[0].to_string(), canopy_cover_fraction)
            .is_some()
        {
            return Err(SnowbenchError::InvalidForcing {
                detail: format!(
                    "{} line {} duplicates canopy date {}",
                    path.display(),
                    line_index + 2,
                    columns[0]
                ),
            });
        }
    }
    if rows.is_empty() {
        return Err(SnowbenchError::InvalidForcing {
            detail: format!("{} contained no canopy rows", path.display()),
        });
    }
    Ok(rows)
}

fn parse_forcing_line(
    path: &Path,
    line_number: usize,
    line: &str,
) -> Result<CoeMeltHourlyForcing, SnowbenchError> {
    let columns = line.split(',').collect::<Vec<_>>();
    if columns.len() != PYSNOBAL_FORCING_COLUMNS.len() + 1 {
        return Err(SnowbenchError::InvalidForcing {
            detail: format!(
                "{} line {line_number} has {} columns, expected {}",
                path.display(),
                columns.len(),
                PYSNOBAL_FORCING_COLUMNS.len() + 1
            ),
        });
    }
    let timestamp = columns[0];
    let date = timestamp
        .get(0..10)
        .ok_or_else(|| SnowbenchError::InvalidForcing {
            detail: format!(
                "{} line {line_number} timestamp is too short",
                path.display()
            ),
        })?;
    let hour_index = timestamp
        .get(11..13)
        .ok_or_else(|| SnowbenchError::InvalidForcing {
            detail: format!("{} line {line_number} timestamp lacks hour", path.display()),
        })?
        .parse::<usize>()
        .map_err(|error| SnowbenchError::InvalidForcing {
            detail: format!(
                "{} line {line_number} timestamp hour is not numeric: {error}",
                path.display()
            ),
        })?;
    if hour_index >= 24 {
        return Err(SnowbenchError::InvalidForcing {
            detail: format!(
                "{} line {line_number} timestamp hour out of range: {hour_index}",
                path.display()
            ),
        });
    }
    let net_solar_w_m2 = parse_column(path, line_number, "net_solar_Wm-2", columns[1])?;
    let downwelling_thermal_w_m2 =
        parse_column(path, line_number, "downwelling_thermal_Wm-2", columns[2])?;
    let temp_air_c = parse_column(path, line_number, "temp_air_degC", columns[3])?;
    let vapor_pressure_pa = parse_column(path, line_number, "vapor_pressure_Pa", columns[5])?;
    let wind_m_s = parse_column(path, line_number, "wind_speed_ms-1", columns[6])?;
    let precip_mass_mm = parse_column(path, line_number, "precip_mass_mm", columns[7])?;
    let snow_precip_fraction = parse_column(path, line_number, "snow_precip_fraction", columns[9])?;
    let snow_density_kg_m3 =
        parse_column(path, line_number, "snow_precip_density_kgm-3", columns[10])?;
    require_unit_interval(
        path,
        line_number,
        "snow_precip_fraction",
        snow_precip_fraction,
    )?;
    if net_solar_w_m2 < 0.0 || wind_m_s < 0.0 || precip_mass_mm < 0.0 {
        return Err(SnowbenchError::InvalidForcing {
            detail: format!(
                "{} line {line_number} has negative radiation, wind, or precipitation",
                path.display()
            ),
        });
    }
    if snow_density_kg_m3 <= 0.0 {
        return Err(SnowbenchError::InvalidForcing {
            detail: format!(
                "{} line {line_number} snow density must be positive, observed {snow_density_kg_m3}",
                path.display()
            ),
        });
    }
    let snow_mass_mm = precip_mass_mm * snow_precip_fraction;
    let rain_m = (precip_mass_mm - snow_mass_mm) / 1_000.0;
    let snowfall_depth_m = snow_mass_mm / snow_density_kg_m3;
    let snow_water_m = snow_mass_mm / 1_000.0;
    Ok(CoeMeltHourlyForcing {
        date: date.to_string(),
        hour_index,
        rain_m,
        snow_water_m,
        snowfall_depth_m,
        radiation_mj_m2: net_solar_w_m2 * 3_600.0
            / 1_000_000.0
            / RADIATION_BRIDGE_NET_SHORTWAVE_FACTOR,
        air_temperature_c: temp_air_c,
        cloud_fraction: cloud_fraction_from_longwave(temp_air_c, downwelling_thermal_w_m2)?,
        dewpoint_c: dewpoint_from_vapor_pressure(vapor_pressure_pa)?,
        wind_m_s,
    })
}

fn parse_column(
    path: &Path,
    line_number: usize,
    field: &'static str,
    raw: &str,
) -> Result<f64, SnowbenchError> {
    let value = raw
        .parse::<f64>()
        .map_err(|error| SnowbenchError::InvalidForcing {
            detail: format!(
                "{} line {line_number} field {field} is not numeric '{raw}': {error}",
                path.display()
            ),
        })?;
    require_finite(path, line_number, field, value)?;
    Ok(value)
}

fn require_finite(
    path: &Path,
    line_number: usize,
    field: &'static str,
    value: f64,
) -> Result<(), SnowbenchError> {
    if value.is_finite() {
        Ok(())
    } else {
        Err(SnowbenchError::InvalidForcing {
            detail: format!(
                "{} line {line_number} field {field} is non-finite: {value}",
                path.display()
            ),
        })
    }
}

fn require_unit_interval(
    path: &Path,
    line_number: usize,
    field: &'static str,
    value: f64,
) -> Result<(), SnowbenchError> {
    require_finite(path, line_number, field, value)?;
    if (0.0..=1.0).contains(&value) {
        Ok(())
    } else {
        Err(SnowbenchError::InvalidForcing {
            detail: format!(
                "{} line {line_number} field {field} must be in [0,1], observed {value}",
                path.display()
            ),
        })
    }
}

fn cloud_fraction_from_longwave(
    temp_air_c: f64,
    downwelling_thermal_w_m2: f64,
) -> Result<f64, SnowbenchError> {
    let temp_k = temp_air_c + 273.15;
    if temp_k <= 0.0 {
        return Err(SnowbenchError::InvalidForcing {
            detail: format!(
                "temperature is physically invalid for longwave inversion: {temp_air_c}"
            ),
        });
    }
    let denom = STEFAN_BOLTZMANN_W_M2_K4 * temp_k.powi(4);
    if denom <= 0.0 || !denom.is_finite() {
        return Err(SnowbenchError::InvalidForcing {
            detail: format!("longwave inversion denominator is invalid: {denom}"),
        });
    }
    let emissivity = downwelling_thermal_w_m2 / denom;
    Ok(((emissivity - 0.72) / 0.28).clamp(0.0, 1.0))
}

fn dewpoint_from_vapor_pressure(vapor_pressure_pa: f64) -> Result<f64, SnowbenchError> {
    let pressure_kpa = vapor_pressure_pa / 1_000.0;
    if pressure_kpa <= 0.0 || !pressure_kpa.is_finite() {
        return Err(SnowbenchError::InvalidForcing {
            detail: format!("vapor pressure must be positive, observed {vapor_pressure_pa} Pa"),
        });
    }
    let ratio_ln = (pressure_kpa / 0.6108).ln();
    Ok(237.3 * ratio_ln / (17.27 - ratio_ln))
}

fn group_daily_forcing(
    hourly_rows: Vec<CoeMeltHourlyForcing>,
    canopy_by_date: &BTreeMap<String, f64>,
) -> Result<Vec<CoeMeltDayForcing>, SnowbenchError> {
    let mut by_date: BTreeMap<String, Vec<CoeMeltHourlyForcing>> = BTreeMap::new();
    for row in hourly_rows {
        by_date.entry(row.date.clone()).or_default().push(row);
    }
    if canopy_by_date.len() != by_date.len() {
        return Err(SnowbenchError::InvalidForcing {
            detail: format!(
                "CoE melt diagnostic canopy day count {} does not match forcing day count {}",
                canopy_by_date.len(),
                by_date.len()
            ),
        });
    }
    let mut days = Vec::with_capacity(by_date.len());
    for (date, rows) in by_date {
        let canopy_cover_fraction =
            *canopy_by_date
                .get(&date)
                .ok_or_else(|| SnowbenchError::InvalidForcing {
                    detail: format!("CoE melt diagnostic missing canopy row for {date}"),
                })?;
        if !(0.0..=1.0).contains(&canopy_cover_fraction) {
            return Err(SnowbenchError::InvalidForcing {
                detail: format!(
                    "CoE melt diagnostic canopy cover fraction must be in [0,1], observed {canopy_cover_fraction} for {date}"
                ),
            });
        }
        if rows.len() != 24 {
            return Err(SnowbenchError::InvalidForcing {
                detail: format!("date {date} has {} hourly rows, expected 24", rows.len()),
            });
        }
        let mut hourly = [DirectSnowHourlyForcing::zero(); 24];
        let mut occupied = [false; 24];
        let mut tmax_c = f64::NEG_INFINITY;
        let mut tmin_c = f64::INFINITY;
        let mut rain_m = 0.0;
        let mut snow_input_m = 0.0;
        let mut dewpoint_sum = 0.0;
        let mut wind_sum = 0.0;
        for row in rows {
            if occupied[row.hour_index] {
                return Err(SnowbenchError::InvalidForcing {
                    detail: format!("date {date} has duplicate hour {}", row.hour_index),
                });
            }
            occupied[row.hour_index] = true;
            let runtime_snowfall_swe_m = row.snowfall_depth_m * 0.1;
            let active_precipitation_m = row.rain_m + runtime_snowfall_swe_m;
            let (rain_fraction, snow_fraction) = if active_precipitation_m > 0.0 {
                (
                    row.rain_m / active_precipitation_m,
                    runtime_snowfall_swe_m / active_precipitation_m,
                )
            } else {
                (0.0, 0.0)
            };
            hourly[row.hour_index] = DirectSnowHourlyForcing {
                active_precipitation_m,
                rain_m: row.rain_m,
                snowfall_m: row.snowfall_depth_m,
                radiation_mj_m2: row.radiation_mj_m2,
                air_temperature_c: row.air_temperature_c,
                cloud_fraction: row.cloud_fraction,
                rain_fraction,
                snow_fraction,
                ..DirectSnowHourlyForcing::zero()
            };
            tmax_c = tmax_c.max(row.air_temperature_c);
            tmin_c = tmin_c.min(row.air_temperature_c);
            rain_m += row.rain_m;
            snow_input_m += row.snow_water_m;
            dewpoint_sum += row.dewpoint_c;
            wind_sum += row.wind_m_s;
        }
        days.push(CoeMeltDayForcing {
            date,
            hourly,
            canopy_cover_fraction,
            dewpoint_c: dewpoint_sum / 24.0,
            wind_m_s: wind_sum / 24.0,
            tmax_c,
            tmin_c,
            rain_m,
            hyetograph_precip_m: rain_m + snow_input_m,
            snow_input_m,
        });
    }
    Ok(days)
}

#[allow(clippy::too_many_lines)]
fn simulate_coe_melt(
    days: &[CoeMeltDayForcing],
    model: CoeMeltModel,
    rst_c: f64,
    newsnw_kg_m3: f64,
    ssd_kg_m3: f64,
) -> Result<CoeMeltSimulation, SnowbenchError> {
    let mut runtime_swe_m = 0.0;
    let mut runtime_depth_m = 0.0;
    let mut runtime_density_kg_m3 = 0.0;
    let mut runtime_settle_day_count = 0.0;
    let mut liquid_water_retained_m = 0.0;
    let mut snow_albedo_state: Option<SnowAlbedoState> = None;
    let mut ledger = CoeMeltLedger::default();
    let mut daily_rows = Vec::with_capacity(days.len());
    for day in days {
        let snow_water_before_m = runtime_swe_m;
        if model == CoeMeltModel::CoeShortwaveAlbedoV1
            && snow_albedo_state.is_none()
            && (runtime_swe_m > 0.0 || runtime_depth_m > 0.0 || day.snow_input_m > 0.0)
        {
            snow_albedo_state = Some(SnowAlbedoState {
                model: SnowAlbedoModel::Brock2000TemperatureAgeV1,
                albedo: 0.85,
                accumulated_positive_temperature_c_day: 0.0,
            });
            ledger.diagnostic_initial_albedo_seed_count += 1;
        }
        let inputs = DirectActiveSnowPartitionInputs {
            hyetograph_rainfall_m: day.hyetograph_precip_m,
            rst_c,
            newsnw_kg_m3,
            ssd_kg_m3,
            runtime_swe_m,
            runtime_depth_m,
            runtime_density_kg_m3,
            runtime_settle_day_count,
            liquid_water_retained_m,
            tmax_c: day.tmax_c,
            tmin_c: day.tmin_c,
            canopy_cover_fraction: day.canopy_cover_fraction,
            wind_m_s: day.wind_m_s,
            dewpoint_c: day.dewpoint_c,
            snow_melt_model: model.snow_melt_model(),
            snow_density_model: SnowDensityModel::LegacyWepp,
            stage3_liquid_routing_model:
                openwepp_hillslope_orchestrator::SnowStage3LiquidRoutingModel::Disabled,
            surface_energy_options:
                openwepp_hillslope_orchestrator::DirectSnowSurfaceEnergyOptions::default(),
            sturm_climate_class: None,
            sturm_day_of_year: None,
            coe_boundary_depth_m: runtime_depth_m,
            coe_boundary_density_kg_m3: runtime_density_kg_m3,
            coe_boundary_settle_day_count: runtime_settle_day_count,
            snow_albedo_model: model.snow_albedo_model(),
            snow_albedo_state,
            snow_layers: Vec::new(),
            underlying_surface_albedo: DEFAULT_UNDERLYING_SURFACE_ALBEDO,
            hourly: day.hourly,
        };
        let partition = Wb11HydrologyKernel::compute_direct_snow_liquid_partition_with_capture(
            &inputs,
            openwepp_hillslope_orchestrator::DirectSnowDiagnosticCapture::Disabled,
        )
        .map_err(|source| SnowbenchError::SnowKernel { source })?;
        let solid_to_liquid = partition.solid_to_liquid_ledger();
        let snowpack_swe_balance_residual_m =
            snow_water_before_m + day.snow_input_m + partition.rain_retained_m
                - solid_to_liquid.snowpack_swe_loss_m
                - partition.sublimation_m
                - partition.runtime_swe_after_m;
        require_coe_melt_swe_closure(&day.date, snowpack_swe_balance_residual_m)?;
        let routed_state_loss_residual_m = solid_to_liquid.liquid_handoff_m
            - solid_to_liquid.rain_released_m
            - solid_to_liquid.snowpack_swe_loss_m;
        let state_loss_available_storage_margin_m =
            snow_water_before_m + day.snow_input_m + day.rain_m
                - solid_to_liquid.snowpack_swe_loss_m
                - partition.sublimation_m;
        ledger.total_snow_input_m += day.snow_input_m;
        ledger.total_rain_input_m += day.rain_m;
        ledger.total_raw_melt_m += solid_to_liquid.raw_signed_melt_m;
        ledger.total_redistributed_melt_m += solid_to_liquid.redistributed_positive_melt_m;
        ledger.total_routed_melt_m += solid_to_liquid.liquid_handoff_m;
        ledger.total_swe_loss_m += solid_to_liquid.snowpack_swe_loss_m;
        ledger.total_sublimation_m += partition.sublimation_m;
        ledger.total_liquid_water_released_m += partition.liquid_water_released_m;
        ledger.positive_snow_hours += day
            .hourly
            .iter()
            .filter(|hour| hour.snowfall_m > 0.0)
            .count();
        runtime_swe_m = partition.runtime_swe_after_m;
        runtime_depth_m = partition.runtime_depth_after_m;
        runtime_density_kg_m3 = partition.runtime_density_after_kg_m3;
        runtime_settle_day_count = partition.runtime_settle_day_count_after;
        liquid_water_retained_m = partition.liquid_water_retained_after_m;
        snow_albedo_state = partition.snow_albedo_state_after;
        daily_rows.push(CoeMeltDailyRow {
            date: day.date.clone(),
            snow_water_before_m,
            snow_input_m: day.snow_input_m,
            rain_input_m: day.rain_m,
            rain_retained_m: partition.rain_retained_m,
            rain_released_m: solid_to_liquid.rain_released_m,
            liquid_holding_capacity_m: partition.liquid_holding_capacity_after_m,
            liquid_water_retained_m: partition.liquid_water_retained_after_m,
            liquid_water_released_m: partition.liquid_water_released_m,
            snow_water_m: runtime_swe_m,
            snow_depth_m: runtime_depth_m,
            snow_density_kg_m3: runtime_density_kg_m3,
            raw_melt_m: solid_to_liquid.raw_signed_melt_m,
            redistributed_melt_m: solid_to_liquid.redistributed_positive_melt_m,
            routed_melt_m: solid_to_liquid.liquid_handoff_m,
            snowpack_swe_loss_m: solid_to_liquid.snowpack_swe_loss_m,
            sublimation_m: partition.sublimation_m,
            snowpack_swe_balance_residual_m,
            routed_state_loss_residual_m,
            state_loss_available_storage_margin_m,
            snow_albedo: snow_albedo_state.map(|state| state.albedo),
            source: model.name(),
        });
    }
    let final_liquid_holding_capacity_m = daily_rows
        .last()
        .map_or(0.0, |row| row.liquid_holding_capacity_m);
    Ok(CoeMeltSimulation {
        daily_rows,
        positive_snow_hours: ledger.positive_snow_hours,
        summary: CoeMeltSummary {
            total_snow_input_m: ledger.total_snow_input_m,
            total_rain_input_m: ledger.total_rain_input_m,
            total_raw_melt_m: ledger.total_raw_melt_m,
            total_redistributed_melt_m: ledger.total_redistributed_melt_m,
            total_routed_melt_m: ledger.total_routed_melt_m,
            total_swe_loss_m: ledger.total_swe_loss_m,
            total_sublimation_m: ledger.total_sublimation_m,
            total_liquid_water_released_m: ledger.total_liquid_water_released_m,
            final_swe_m: runtime_swe_m,
            final_depth_m: runtime_depth_m,
            final_density_kg_m3: runtime_density_kg_m3,
            final_liquid_holding_capacity_m,
            final_liquid_water_retained_m: liquid_water_retained_m,
            final_settle_day_count: runtime_settle_day_count,
            final_snow_albedo: snow_albedo_state.map(|state| state.albedo),
            diagnostic_initial_albedo_seed_count: ledger.diagnostic_initial_albedo_seed_count,
        },
    })
}

fn require_coe_melt_swe_closure(date: &str, residual_m: f64) -> Result<(), SnowbenchError> {
    if !residual_m.is_finite() || residual_m.abs() > DAILY_SWE_CLOSURE_TOLERANCE_M {
        return Err(SnowbenchError::SnowStorageClosure {
            date: date.to_string(),
            residual_m,
            tolerance_m: DAILY_SWE_CLOSURE_TOLERANCE_M,
        });
    }
    Ok(())
}

fn write_coe_melt_csv(path: &Path, rows: &[CoeMeltDailyRow]) -> Result<(), SnowbenchError> {
    let mut file = fs::File::create(path).map_err(|source| snowbench_io(path, source))?;
    writeln!(
        file,
        "date,snow_water_before_m,snow_input_m,rain_input_m,rain_retained_m,rain_released_m,liquid_holding_capacity_m,liquid_water_retained_m,liquid_water_released_m,snow_water_m,snow_depth_m,snow_density_kg_m3,raw_melt_m,redistributed_melt_m,routed_melt_m,snowpack_swe_loss_m,sublimation_m,snowpack_swe_balance_residual_m,routed_state_loss_residual_m,state_loss_available_storage_margin_m,snow_albedo,source"
    )
    .map_err(|source| snowbench_io(path, source))?;
    for row in rows {
        let albedo = row
            .snow_albedo
            .map(|value| format!("{value:.12}"))
            .unwrap_or_default();
        writeln!(
            file,
            "{},{:.12},{:.12},{:.12},{:.12},{:.12},{:.12},{:.12},{:.12},{:.12},{:.12},{:.12},{:.12},{:.12},{:.12},{:.12},{:.12},{:.12},{:.12},{:.12},{},{}",
            row.date,
            row.snow_water_before_m,
            row.snow_input_m,
            row.rain_input_m,
            row.rain_retained_m,
            row.rain_released_m,
            row.liquid_holding_capacity_m,
            row.liquid_water_retained_m,
            row.liquid_water_released_m,
            row.snow_water_m,
            row.snow_depth_m,
            row.snow_density_kg_m3,
            row.raw_melt_m,
            row.redistributed_melt_m,
            row.routed_melt_m,
            row.snowpack_swe_loss_m,
            row.sublimation_m,
            row.snowpack_swe_balance_residual_m,
            row.routed_state_loss_residual_m,
            row.state_loss_available_storage_margin_m,
            albedo,
            row.source,
        )
        .map_err(|source| snowbench_io(path, source))?;
    }
    Ok(())
}

fn write_json<T: Serialize>(path: &Path, value: &T) -> Result<(), SnowbenchError> {
    let text = serde_json::to_string_pretty(value).map_err(|source| SnowbenchError::Json {
        path: path.to_path_buf(),
        source,
    })?;
    write_text(path, &text)
}

fn write_markdown(path: &Path, report: &CoeMeltReport) -> Result<(), SnowbenchError> {
    let text = format!(
        "# CoE Melt Snowbench Summary\n\n\
         - Schema: `{}`\n\
         - Model: `{}`\n\
         - Contract: `{}`\n\
         - Runtime coupling: `{}`\n\
         - No site constants: `{}`\n\
         - Canopy source: `{}`\n\
         - Canopy series: `{}`\n\
         - Canopy mean/min/max: `{:.6}` / `{:.6}` / `{:.6}`\n\
         - Shortwave source: `{}`\n\
         - Shortwave bridge like-for-like: `{}`\n\
         - Days: `{}`\n\
         - Hourly rows: `{}`\n\
         - Total snow input: `{:.6}` m water equivalent\n\
         - Total raw melt: `{:.6}` m\n\
         - Total routed melt: `{:.6}` m\n\
         - Final SWE: `{:.6}` m\n\
         - Final depth: `{:.6}` m\n\
         - Diagnostic albedo cold-start seeds: `{}`\n",
        report.schema,
        report.model_id,
        report.contract,
        report.runtime_coupling,
        report.no_site_constants,
        report.canopy_source,
        report.canopy_series_path,
        report.canopy_series_summary.mean,
        report.canopy_series_summary.min,
        report.canopy_series_summary.max,
        report.shortwave_source,
        report.shortwave_bridge_like_for_like,
        report.day_count,
        report.hourly_row_count,
        report.summary.total_snow_input_m,
        report.summary.total_raw_melt_m,
        report.summary.total_routed_melt_m,
        report.summary.final_swe_m,
        report.summary.final_depth_m,
        report.summary.diagnostic_initial_albedo_seed_count,
    );
    write_text(path, &text)
}

fn write_text(path: &Path, text: &str) -> Result<(), SnowbenchError> {
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent).map_err(|source| snowbench_io(parent, source))?;
    }
    fs::write(path, text).map_err(|source| snowbench_io(path, source))
}

fn snowbench_io(path: impl Into<PathBuf>, source: io::Error) -> SnowbenchError {
    SnowbenchError::Io {
        path: path.into(),
        source,
    }
}

#[cfg(test)]
mod tests {
    use std::fmt::Debug;
    use std::sync::atomic::{AtomicUsize, Ordering};

    use super::*;

    static TEMP_SEQUENCE: AtomicUsize = AtomicUsize::new(0);

    fn temp_path(label: &str) -> PathBuf {
        let sequence = TEMP_SEQUENCE.fetch_add(1, Ordering::Relaxed);
        std::env::temp_dir().join(format!(
            "openwepp-coe-melt-{label}-{}-{sequence}.csv",
            std::process::id()
        ))
    }

    fn invalid_forcing_detail<T: Debug>(result: Result<T, SnowbenchError>) -> String {
        match result {
            Err(SnowbenchError::InvalidForcing { detail }) => detail,
            other => panic!("expected invalid-forcing error, observed {other:?}"),
        }
    }

    fn canopy_result(contents: &str) -> Result<BTreeMap<String, f64>, SnowbenchError> {
        let path = temp_path("canopy");
        fs::write(&path, contents).expect("temporary canopy fixture must be writable");
        let result = read_canopy_series(&path);
        fs::remove_file(path).expect("temporary canopy fixture must be removable");
        result
    }

    fn forcing_line(timestamp: &str, replacements: &[(usize, &str)]) -> String {
        let mut columns = vec![
            timestamp, "100", "250", "-5", "-2", "500", "2", "1", "0", "0.5", "100",
        ];
        for (index, value) in replacements {
            columns[*index] = value;
        }
        columns.join(",")
    }

    #[test]
    fn canopy_series_rejects_io_header_shape_sequence_and_domain_failures() {
        let missing = temp_path("missing");
        assert!(matches!(
            read_canopy_series(&missing),
            Err(SnowbenchError::Io { .. })
        ));

        let cases = [
            ("", "missing header"),
            ("wrong,header\n", "unexpected canopy header"),
            (
                "date,day_index,canopy_cover_fraction,source\n2020-01-01,1,0.5\n",
                "3 canopy columns, expected 4",
            ),
            (
                "date,day_index,canopy_cover_fraction,source\n2020-01-01,bad,0.5,growth\n",
                "day_index is not numeric",
            ),
            (
                "date,day_index,canopy_cover_fraction,source\n2020-01-01,2,0.5,growth\n",
                "day_index 2, expected 1",
            ),
            (
                "date,day_index,canopy_cover_fraction,source\n2020-01-01,1,bad,growth\n",
                "canopy_cover_fraction is not numeric",
            ),
            (
                "date,day_index,canopy_cover_fraction,source\n2020-01-01,1,NaN,growth\n",
                "canopy_cover_fraction is non-finite",
            ),
            (
                "date,day_index,canopy_cover_fraction,source\n2020-01-01,1,1.1,growth\n",
                "must be in [0,1]",
            ),
            (
                "date,day_index,canopy_cover_fraction,source\n2020-01-01,1,0.4,growth\n2020-01-01,2,0.5,growth\n",
                "duplicates canopy date",
            ),
            (
                "date,day_index,canopy_cover_fraction,source\n",
                "contained no canopy rows",
            ),
        ];

        for (contents, expected_detail) in cases {
            let detail = invalid_forcing_detail(canopy_result(contents));
            assert!(
                detail.contains(expected_detail),
                "detail '{detail}' must contain '{expected_detail}'"
            );
        }
    }

    #[test]
    fn canopy_series_accepts_ordered_finite_unit_interval_rows() {
        let rows = canopy_result(
            "date,day_index,canopy_cover_fraction,source\n\
             2020-01-01,1,0,growth\n\
             \n\
             2020-01-02,2,1,growth\n",
        )
        .expect("valid canopy series must parse");

        assert_eq!(rows.len(), 2);
        assert!(rows["2020-01-01"].abs() <= f64::EPSILON);
        assert!((rows["2020-01-02"] - 1.0).abs() <= f64::EPSILON);
    }

    #[test]
    fn forcing_line_rejects_column_and_timestamp_failures() {
        let path = Path::new("forcing.csv");
        let cases = [
            (
                "2020-01-01T00:00:00,1".to_string(),
                "has 2 columns, expected 11",
            ),
            (forcing_line("short", &[]), "timestamp is too short"),
            (forcing_line("2020-01-01", &[]), "timestamp lacks hour"),
            (
                forcing_line("2020-01-01Txx:00:00", &[]),
                "timestamp hour is not numeric",
            ),
            (
                forcing_line("2020-01-01T24:00:00", &[]),
                "timestamp hour out of range: 24",
            ),
        ];

        for (line, expected_detail) in cases {
            let detail = invalid_forcing_detail(parse_forcing_line(path, 2, &line));
            assert!(detail.contains(expected_detail));
        }
    }

    #[test]
    fn forcing_line_rejects_numeric_and_physical_domain_failures() {
        let path = Path::new("forcing.csv");
        let cases = [
            (vec![(1, "bad")], "net_solar_Wm-2 is not numeric"),
            (vec![(1, "NaN")], "net_solar_Wm-2 is non-finite"),
            (vec![(9, "1.1")], "snow_precip_fraction must be in [0,1]"),
            (
                vec![(1, "-1")],
                "has negative radiation, wind, or precipitation",
            ),
            (
                vec![(6, "-1")],
                "has negative radiation, wind, or precipitation",
            ),
            (
                vec![(7, "-1")],
                "has negative radiation, wind, or precipitation",
            ),
            (vec![(10, "0")], "snow density must be positive"),
            (
                vec![(3, "-273.15")],
                "temperature is physically invalid for longwave inversion",
            ),
            (
                vec![(3, "1e100")],
                "longwave inversion denominator is invalid",
            ),
            (vec![(5, "0")], "vapor pressure must be positive"),
        ];

        for (replacements, expected_detail) in cases {
            let line = forcing_line("2020-01-01T00:00:00", &replacements);
            let detail = invalid_forcing_detail(parse_forcing_line(path, 2, &line));
            assert!(
                detail.contains(expected_detail),
                "detail '{detail}' must contain '{expected_detail}'"
            );
        }
    }

    #[test]
    fn forcing_line_accepts_and_converts_complete_physical_row() {
        let line = forcing_line("2020-01-01T00:00:00", &[]);
        let row = parse_forcing_line(Path::new("forcing.csv"), 2, &line)
            .expect("valid forcing row must parse");

        assert_eq!(row.date, "2020-01-01");
        assert_eq!(row.hour_index, 0);
        assert!((row.rain_m - 0.000_5).abs() < 1e-12);
        assert!((row.snow_water_m - 0.000_5).abs() < 1e-12);
        assert!((row.snowfall_depth_m - 0.005).abs() < 1e-12);
        assert!((row.radiation_mj_m2 - 0.45).abs() < 1e-12);
        assert!((row.air_temperature_c + 5.0).abs() <= f64::EPSILON);
        assert!((0.0..=1.0).contains(&row.cloud_fraction));
        assert!(row.dewpoint_c.is_finite());
        assert!((row.wind_m_s - 2.0).abs() <= f64::EPSILON);
    }

    #[test]
    fn noncanonical_new_snow_density_fails_closed_at_consumer_storage_boundary() {
        let rows = (0..24)
            .map(|hour| {
                let timestamp = format!("2020-01-01T{hour:02}:00:00");
                parse_forcing_line(
                    Path::new("forcing.csv"),
                    hour + 2,
                    &forcing_line(&timestamp, &[(10, "200")]),
                )
            })
            .collect::<Result<Vec<_>, _>>()
            .expect("noncanonical snow density forcing must parse");
        let canopy = BTreeMap::from([("2020-01-01".to_string(), 0.5)]);
        let days = group_daily_forcing(rows, &canopy)
            .expect("runtime phase operands must close for noncanonical density");
        let hour = days[0].hourly[0];
        let runtime_snowfall_swe_m = hour.snowfall_m * 0.1;

        assert!((hour.active_precipitation_m - hour.rain_m - runtime_snowfall_swe_m).abs() < 1e-12);
        assert!((hour.rain_m - hour.active_precipitation_m * hour.rain_fraction).abs() < 1e-12);
        assert!(
            (runtime_snowfall_swe_m - hour.active_precipitation_m * hour.snow_fraction).abs()
                < 1e-12
        );
        let error = simulate_coe_melt(&days, CoeMeltModel::LegacyCoe, 0.0, 200.0, 350.0)
            .expect_err("source-mass versus fixed-density runtime mismatch must fail closed");
        assert!(matches!(error, SnowbenchError::SnowStorageClosure { .. }));
    }

    #[test]
    fn coe_melt_consumer_fails_closed_on_material_daily_swe_residual() {
        let error = require_coe_melt_swe_closure("2020-01-01", 1.1e-9)
            .expect_err("material daily SWE residual must fail closed");
        assert!(matches!(error, SnowbenchError::SnowStorageClosure { .. }));
        require_coe_melt_swe_closure("2020-01-01", -1.0e-9)
            .expect("residual at the canonical tolerance is accepted");
        assert!(matches!(
            require_coe_melt_swe_closure("2020-01-01", f64::NAN),
            Err(SnowbenchError::SnowStorageClosure { .. })
        ));
    }
}
