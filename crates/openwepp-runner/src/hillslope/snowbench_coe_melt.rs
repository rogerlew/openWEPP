use std::collections::BTreeMap;
use std::fs;
use std::io::{self, Write};
use std::path::{Path, PathBuf};

use openwepp_hillslope_orchestrator::{
    DirectActiveSnowPartitionInputs, DirectSnowHourlyForcing, SnowAlbedoModel, SnowAlbedoState,
    SnowMeltModel, Wb11HydrologyKernel,
};
use openwepp_input_contract::parsers::snow::{
    ParseMode as SnowParseMode, SnowParseOptions, parse_snow_file,
};
use serde::Serialize;

use super::snowbench::{
    PYSNOBAL_FORCING_COLUMNS, SnowbenchError, SnowbenchExportRequest, export_pysnobal_inputs,
};

const CONTRACT: &str = "SC-SNOWFREEZE-001 INV-SNOWFREEZE-050 INV-SNOWFREEZE-052 INV-SNOWFREEZE-055";
const RADIATION_BRIDGE_NET_SHORTWAVE_FACTOR: f64 = 0.80;
const STEFAN_BOLTZMANN_W_M2_K4: f64 = 5.670_374_419e-8;
const DEFAULT_CANOPY_COVER_FRACTION: f64 = 0.0;
const DEFAULT_UNDERLYING_SURFACE_ALBEDO: f64 = 0.2;

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
    pub final_swe_m: f64,
    pub final_depth_m: f64,
    pub final_density_kg_m3: f64,
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
    snow_water_m: f64,
    snow_depth_m: f64,
    snow_density_kg_m3: f64,
    raw_melt_m: f64,
    redistributed_melt_m: f64,
    routed_melt_m: f64,
    snowpack_swe_loss_m: f64,
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
    positive_snow_hours: usize,
    diagnostic_initial_albedo_seed_count: usize,
}

impl CoeMeltModel {
    #[must_use]
    pub const fn name(self) -> &'static str {
        match self {
            Self::LegacyCoe => "legacy_coe",
            Self::CoeShortwaveAlbedoV1 => "coe_shortwave_albedo_v1",
        }
    }

    pub fn parse(value: &str) -> Result<Self, SnowbenchError> {
        match value {
            "legacy_coe" => Ok(Self::LegacyCoe),
            "coe_shortwave_albedo_v1" => Ok(Self::CoeShortwaveAlbedoV1),
            _ => Err(SnowbenchError::InvalidInput {
                detail: format!(
                    "unknown CoE melt model '{value}', expected legacy_coe or coe_shortwave_albedo_v1"
                ),
            }),
        }
    }

    const fn snow_melt_model(self) -> SnowMeltModel {
        match self {
            Self::LegacyCoe => SnowMeltModel::LegacyCoe,
            Self::CoeShortwaveAlbedoV1 => SnowMeltModel::CoeShortwaveAlbedoV1,
        }
    }

    const fn snow_albedo_model(self) -> Option<SnowAlbedoModel> {
        match self {
            Self::LegacyCoe => None,
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
    let days = group_daily_forcing(hourly)?;
    let simulation = simulate_coe_melt(&days, request.model, snow.rst, snow.newsnw, snow.ssd)?;
    write_coe_melt_csv(
        &output_dir.join("coe_melt_snow.csv"),
        &simulation.daily_rows,
    )?;
    let report = CoeMeltReport {
        schema: "snowdensity05e-coe-melt-snowbench-v1",
        model_id: request.model.name(),
        contract: CONTRACT,
        runtime_coupling: "diagnostic snowbench replay of typed CoE melt path; no production activation",
        no_site_constants: true,
        output_dir: output_dir.display().to_string(),
        forcing_bridge_dir: export_report.output_dir,
        day_count: simulation.daily_rows.len(),
        hourly_row_count: export_report.hourly_row_count,
        positive_snow_hours: simulation.positive_snow_hours,
        constants: CoeMeltConstants {
            canopy_cover_fraction: DEFAULT_CANOPY_COVER_FRACTION,
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
) -> Result<Vec<CoeMeltDayForcing>, SnowbenchError> {
    let mut by_date: BTreeMap<String, Vec<CoeMeltHourlyForcing>> = BTreeMap::new();
    for row in hourly_rows {
        by_date.entry(row.date.clone()).or_default().push(row);
    }
    let mut days = Vec::with_capacity(by_date.len());
    for (date, rows) in by_date {
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
            hourly[row.hour_index] = DirectSnowHourlyForcing {
                rain_m: row.rain_m,
                snowfall_m: row.snowfall_depth_m,
                radiation_mj_m2: row.radiation_mj_m2,
                air_temperature_c: row.air_temperature_c,
                cloud_fraction: row.cloud_fraction,
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
    let mut snow_albedo_state: Option<SnowAlbedoState> = None;
    let mut ledger = CoeMeltLedger::default();
    let mut daily_rows = Vec::with_capacity(days.len());
    for day in days {
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
            tmax_c: day.tmax_c,
            tmin_c: day.tmin_c,
            canopy_cover_fraction: DEFAULT_CANOPY_COVER_FRACTION,
            wind_m_s: day.wind_m_s,
            dewpoint_c: day.dewpoint_c,
            snow_melt_model: model.snow_melt_model(),
            snow_albedo_model: model.snow_albedo_model(),
            snow_albedo_state,
            underlying_surface_albedo: DEFAULT_UNDERLYING_SURFACE_ALBEDO,
            hourly: day.hourly,
        };
        let partition = Wb11HydrologyKernel::compute_direct_snow_liquid_partition_from_typed(
            inputs,
        )
        .map_err(|error| SnowbenchError::InvalidForcing {
            detail: format!(
                "CoE melt diagnostic replay failed for {} {}: {error}",
                model.name(),
                day.date
            ),
        })?;
        ledger.total_snow_input_m += day.snow_input_m;
        ledger.total_rain_input_m += day.rain_m;
        ledger.total_raw_melt_m += partition.raw_melt_m;
        ledger.total_redistributed_melt_m += partition.redistributed_melt_m;
        ledger.total_routed_melt_m += partition.routed_melt_m;
        ledger.total_swe_loss_m += partition.snowpack_swe_loss_m;
        ledger.positive_snow_hours += day
            .hourly
            .iter()
            .filter(|hour| hour.snowfall_m > 0.0)
            .count();
        runtime_swe_m = partition.runtime_swe_after_m;
        runtime_depth_m = partition.runtime_depth_after_m;
        runtime_density_kg_m3 = partition.runtime_density_after_kg_m3;
        runtime_settle_day_count = partition.runtime_settle_day_count_after;
        snow_albedo_state = partition.snow_albedo_state_after;
        daily_rows.push(CoeMeltDailyRow {
            date: day.date.clone(),
            snow_water_m: runtime_swe_m,
            snow_depth_m: runtime_depth_m,
            snow_density_kg_m3: runtime_density_kg_m3,
            raw_melt_m: partition.raw_melt_m,
            redistributed_melt_m: partition.redistributed_melt_m,
            routed_melt_m: partition.routed_melt_m,
            snowpack_swe_loss_m: partition.snowpack_swe_loss_m,
            snow_albedo: snow_albedo_state.map(|state| state.albedo),
            source: model.name(),
        });
    }
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
            final_swe_m: runtime_swe_m,
            final_depth_m: runtime_depth_m,
            final_density_kg_m3: runtime_density_kg_m3,
            final_settle_day_count: runtime_settle_day_count,
            final_snow_albedo: snow_albedo_state.map(|state| state.albedo),
            diagnostic_initial_albedo_seed_count: ledger.diagnostic_initial_albedo_seed_count,
        },
    })
}

fn write_coe_melt_csv(path: &Path, rows: &[CoeMeltDailyRow]) -> Result<(), SnowbenchError> {
    let mut file = fs::File::create(path).map_err(|source| snowbench_io(path, source))?;
    writeln!(
        file,
        "date,snow_water_m,snow_depth_m,snow_density_kg_m3,raw_melt_m,redistributed_melt_m,routed_melt_m,snowpack_swe_loss_m,snow_albedo,source"
    )
    .map_err(|source| snowbench_io(path, source))?;
    for row in rows {
        let albedo = row
            .snow_albedo
            .map(|value| format!("{value:.12}"))
            .unwrap_or_default();
        writeln!(
            file,
            "{},{:.12},{:.12},{:.12},{:.12},{:.12},{:.12},{:.12},{},{}",
            row.date,
            row.snow_water_m,
            row.snow_depth_m,
            row.snow_density_kg_m3,
            row.raw_melt_m,
            row.redistributed_melt_m,
            row.routed_melt_m,
            row.snowpack_swe_loss_m,
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
