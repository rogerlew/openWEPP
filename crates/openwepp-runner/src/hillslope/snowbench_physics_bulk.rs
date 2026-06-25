use std::error::Error;
use std::fmt;
use std::fs;
use std::io::{self, Write};
use std::path::{Path, PathBuf};

use serde::Serialize;

use super::snowbench::{
    PYSNOBAL_FORCING_COLUMNS, SnowbenchError, SnowbenchExportRequest, export_pysnobal_inputs,
};

const DEFAULT_MODEL_ID: &str = "physics_bulk_candidate_v1";
const CONTRACT: &str = "SC-SNOWFREEZE-001 INV-SNOWFREEZE-051 OBL-SNOWFREEZE-P-026 ADR-0027";
const RHO_WATER_KG_M3: f64 = 1_000.0;
const RHO_ICE_KG_M3: f64 = 917.0;
const LATENT_HEAT_FUSION_J_KG: f64 = 333_500.0;
const SPECIFIC_HEAT_ICE_J_KG_K: f64 = 2_100.0;
const SPECIFIC_HEAT_WATER_J_KG_K: f64 = 4_186.0;
const ZERO_MASS_KG_M2: f64 = 1.0e-9;

#[derive(Debug, Clone)]
pub struct PhysicsBulkRequest {
    pub run_dir: PathBuf,
    pub run_file: Option<PathBuf>,
    pub output_dir: PathBuf,
    pub variant: PhysicsBulkVariant,
}

#[derive(Debug, Clone, Serialize)]
pub struct PhysicsBulkReport {
    pub schema: &'static str,
    pub model_id: &'static str,
    pub variant: &'static str,
    pub contract: &'static str,
    pub output_dir: String,
    pub forcing_bridge_dir: String,
    pub day_count: usize,
    pub hourly_row_count: usize,
    pub positive_snow_hours: usize,
    pub no_site_constants: bool,
    pub runtime_coupling: &'static str,
    pub constants: PhysicsBulkConstants,
    pub summary: PhysicsBulkSummary,
}

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Serialize)]
pub enum PhysicsBulkVariant {
    #[default]
    CandidateV1,
    SlowMeltV1,
    DenseSlowMeltV1,
    ColdDenseSlowMeltV1,
}

#[derive(Debug, Clone, Copy, Serialize)]
pub struct PhysicsBulkConstants {
    pub new_snow_density_min_kg_m3: f64,
    pub new_snow_density_max_kg_m3: f64,
    pub new_snow_density_base_kg_m3: f64,
    pub new_snow_density_temperature_threshold_c: f64,
    pub new_snow_density_temperature_coefficient: f64,
    pub dry_compaction_max_density_kg_m3: f64,
    pub dry_compaction_swe_max_kg_m2: f64,
    pub wet_compaction_max_density_kg_m3: f64,
    pub wet_compaction_half_saturation_ratio: f64,
    pub max_liquid_water_volume_fraction: f64,
    pub positive_degree_melt_kg_m2_per_c_hour: f64,
    pub solar_melt_efficiency: f64,
    pub subfreezing_cold_content_relaxation_per_hour: f64,
    pub dry_compaction_multiplier: f64,
    pub wet_compaction_multiplier: f64,
}

#[derive(Debug, Clone, Serialize)]
pub struct PhysicsBulkSummary {
    pub total_snow_input_kg_m2: f64,
    pub total_rain_on_snow_input_kg_m2: f64,
    pub total_liquid_release_kg_m2: f64,
    pub final_swe_kg_m2: f64,
    pub final_depth_m: f64,
    pub final_density_kg_m3: f64,
    pub final_liquid_water_kg_m2: f64,
    pub final_cold_content_j_m2: f64,
    pub max_abs_mass_balance_residual_kg_m2: f64,
    pub max_abs_cold_content_residual_j_m2: f64,
    pub max_density_kg_m3: f64,
    pub min_nonzero_density_kg_m3: Option<f64>,
}

#[derive(Debug)]
pub enum PhysicsBulkError {
    Csv { path: PathBuf, detail: String },
    Io { path: PathBuf, source: io::Error },
    Invalid { detail: String },
}

impl fmt::Display for PhysicsBulkError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Csv { path, detail } => {
                write!(f, "SNOWBENCH-PHYSBULK-E-001 {}: {detail}", path.display())
            }
            Self::Io { path, source } => {
                write!(
                    f,
                    "SNOWBENCH-PHYSBULK-E-002 io error at {}: {source}",
                    path.display()
                )
            }
            Self::Invalid { detail } => write!(f, "SNOWBENCH-PHYSBULK-E-003 {detail}"),
        }
    }
}

impl Error for PhysicsBulkError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            Self::Io { source, .. } => Some(source),
            Self::Csv { .. } | Self::Invalid { .. } => None,
        }
    }
}

impl From<PhysicsBulkError> for SnowbenchError {
    fn from(source: PhysicsBulkError) -> Self {
        SnowbenchError::InvalidForcing {
            detail: source.to_string(),
        }
    }
}

#[derive(Debug, Clone)]
struct PhysicsBulkForcingRow {
    date: String,
    net_solar_w_m2: f64,
    temp_air_c: f64,
    wind_speed_m_s: f64,
    precip_mass_kg_m2: f64,
    snow_precip_fraction: f64,
}

#[derive(Debug, Clone, Copy, Default)]
struct PhysicsBulkState {
    ice_mass_kg_m2: f64,
    liquid_water_kg_m2: f64,
    density_kg_m3: f64,
    cold_content_j_m2: f64,
    snow_cover_age_h: f64,
}

#[derive(Debug, Clone, Serialize)]
struct PhysicsBulkDailyRow {
    date: String,
    snow_water_m: f64,
    snow_depth_m: f64,
    snow_density_kg_m3: f64,
    liquid_water_m: f64,
    cold_content_j_m2: f64,
    snow_bulk_temperature_c: f64,
    released_liquid_m: f64,
    source: &'static str,
}

#[derive(Debug, Clone)]
struct PhysicsBulkSimulation {
    daily_rows: Vec<PhysicsBulkDailyRow>,
    summary: PhysicsBulkSummary,
}

#[derive(Debug, Clone, Copy, Default)]
struct SimulationLedger {
    total_snow_input_kg_m2: f64,
    total_rain_on_snow_input_kg_m2: f64,
    total_liquid_release_kg_m2: f64,
    max_abs_mass_balance_residual_kg_m2: f64,
    max_abs_cold_content_residual_j_m2: f64,
    max_density_kg_m3: f64,
    min_nonzero_density_kg_m3: Option<f64>,
}

#[must_use]
pub const fn physics_bulk_constants() -> PhysicsBulkConstants {
    physics_bulk_constants_for_variant(PhysicsBulkVariant::CandidateV1)
}

#[must_use]
pub const fn physics_bulk_constants_for_variant(
    variant: PhysicsBulkVariant,
) -> PhysicsBulkConstants {
    match variant {
        PhysicsBulkVariant::CandidateV1 => PhysicsBulkConstants {
            new_snow_density_min_kg_m3: 50.0,
            new_snow_density_max_kg_m3: 200.0,
            new_snow_density_base_kg_m3: 50.0,
            new_snow_density_temperature_threshold_c: -15.0,
            new_snow_density_temperature_coefficient: 1.7,
            dry_compaction_max_density_kg_m3: 550.0,
            dry_compaction_swe_max_kg_m2: 2_000.0,
            wet_compaction_max_density_kg_m3: 550.0,
            wet_compaction_half_saturation_ratio: 0.4,
            max_liquid_water_volume_fraction: 0.01,
            positive_degree_melt_kg_m2_per_c_hour: 0.18,
            solar_melt_efficiency: 0.02,
            subfreezing_cold_content_relaxation_per_hour: 0.015,
            dry_compaction_multiplier: 1.0,
            wet_compaction_multiplier: 1.0,
        },
        PhysicsBulkVariant::SlowMeltV1 => PhysicsBulkConstants {
            positive_degree_melt_kg_m2_per_c_hour: 0.05,
            solar_melt_efficiency: 0.005,
            ..physics_bulk_constants_for_variant(PhysicsBulkVariant::CandidateV1)
        },
        PhysicsBulkVariant::DenseSlowMeltV1 => PhysicsBulkConstants {
            new_snow_density_min_kg_m3: 75.0,
            new_snow_density_max_kg_m3: 250.0,
            new_snow_density_base_kg_m3: 75.0,
            positive_degree_melt_kg_m2_per_c_hour: 0.05,
            solar_melt_efficiency: 0.005,
            dry_compaction_multiplier: 4.0,
            wet_compaction_multiplier: 2.0,
            ..physics_bulk_constants_for_variant(PhysicsBulkVariant::CandidateV1)
        },
        PhysicsBulkVariant::ColdDenseSlowMeltV1 => PhysicsBulkConstants {
            new_snow_density_min_kg_m3: 75.0,
            new_snow_density_max_kg_m3: 250.0,
            new_snow_density_base_kg_m3: 75.0,
            positive_degree_melt_kg_m2_per_c_hour: 0.03,
            solar_melt_efficiency: 0.002,
            subfreezing_cold_content_relaxation_per_hour: 0.03,
            dry_compaction_multiplier: 4.0,
            wet_compaction_multiplier: 2.0,
            ..physics_bulk_constants_for_variant(PhysicsBulkVariant::CandidateV1)
        },
    }
}

impl PhysicsBulkVariant {
    #[must_use]
    pub const fn name(self) -> &'static str {
        match self {
            Self::CandidateV1 => "candidate_v1",
            Self::SlowMeltV1 => "slow_melt_v1",
            Self::DenseSlowMeltV1 => "dense_slow_melt_v1",
            Self::ColdDenseSlowMeltV1 => "cold_dense_slow_melt_v1",
        }
    }

    #[must_use]
    pub const fn model_id(self) -> &'static str {
        match self {
            Self::CandidateV1 => DEFAULT_MODEL_ID,
            Self::SlowMeltV1 => "physics_bulk_slow_melt_v1",
            Self::DenseSlowMeltV1 => "physics_bulk_dense_slow_melt_v1",
            Self::ColdDenseSlowMeltV1 => "physics_bulk_cold_dense_slow_melt_v1",
        }
    }

    pub fn parse(value: &str) -> Result<Self, PhysicsBulkError> {
        match value {
            "candidate_v1" => Ok(Self::CandidateV1),
            "slow_melt_v1" => Ok(Self::SlowMeltV1),
            "dense_slow_melt_v1" => Ok(Self::DenseSlowMeltV1),
            "cold_dense_slow_melt_v1" => Ok(Self::ColdDenseSlowMeltV1),
            _ => Err(PhysicsBulkError::Invalid {
                detail: format!(
                    "unknown physics_bulk variant '{value}', expected one of {}",
                    Self::names().join(", ")
                ),
            }),
        }
    }

    #[must_use]
    pub fn all() -> &'static [Self] {
        &[
            Self::CandidateV1,
            Self::SlowMeltV1,
            Self::DenseSlowMeltV1,
            Self::ColdDenseSlowMeltV1,
        ]
    }

    #[must_use]
    pub fn names() -> Vec<&'static str> {
        Self::all().iter().map(|variant| variant.name()).collect()
    }
}

pub fn run_physics_bulk_snowbench(
    request: &PhysicsBulkRequest,
) -> Result<PhysicsBulkReport, SnowbenchError> {
    let output_dir = absolute_path(&request.output_dir)?;
    fs::create_dir_all(&output_dir).map_err(|source| snowbench_io(&output_dir, source))?;
    let forcing_bridge_dir = output_dir.join("forcing_bridge");
    let export_report = export_pysnobal_inputs(&SnowbenchExportRequest {
        run_dir: request.run_dir.clone(),
        run_file: request.run_file.clone(),
        output_dir: forcing_bridge_dir.clone(),
        include_openwepp_snow_projection: false,
    })?;
    let forcing_csv = forcing_bridge_dir.join("tg_0p0c_zg0p10m/forcing.csv");
    let forcing = read_physics_bulk_forcing(&forcing_csv)?;
    let constants = physics_bulk_constants_for_variant(request.variant);
    let simulation = simulate_physics_bulk(&forcing, constants, request.variant.model_id())?;
    write_physics_bulk_csv(
        &output_dir.join("physics_bulk_snow.csv"),
        &simulation.daily_rows,
    )?;
    let report = PhysicsBulkReport {
        schema: "snowdensity03-physics-bulk-snowbench-v1",
        model_id: request.variant.model_id(),
        variant: request.variant.name(),
        contract: CONTRACT,
        output_dir: output_dir.display().to_string(),
        forcing_bridge_dir: forcing_bridge_dir.display().to_string(),
        day_count: simulation.daily_rows.len(),
        hourly_row_count: forcing.len(),
        positive_snow_hours: forcing
            .iter()
            .filter(|row| row.precip_mass_kg_m2 * row.snow_precip_fraction > ZERO_MASS_KG_M2)
            .count(),
        no_site_constants: true,
        runtime_coupling: "none; offline snowbench candidate only",
        constants,
        summary: simulation.summary,
    };
    write_json(&output_dir.join("physics_bulk_summary.json"), &report)?;
    write_markdown(
        &output_dir.join("physics_bulk_summary.md"),
        &report,
        &export_report.output_dir,
    )?;
    Ok(report)
}

fn absolute_path(path: &Path) -> Result<PathBuf, SnowbenchError> {
    if path.is_absolute() {
        return Ok(path.to_path_buf());
    }
    let cwd = std::env::current_dir().map_err(|source| snowbench_io(".", source))?;
    Ok(cwd.join(path))
}

fn read_physics_bulk_forcing(path: &Path) -> Result<Vec<PhysicsBulkForcingRow>, PhysicsBulkError> {
    let text = fs::read_to_string(path).map_err(|source| PhysicsBulkError::Io {
        path: path.to_path_buf(),
        source,
    })?;
    let mut lines = text.lines();
    let header = lines.next().ok_or_else(|| PhysicsBulkError::Csv {
        path: path.to_path_buf(),
        detail: "missing header".to_string(),
    })?;
    let expected_header = format!("Datetime,{}", PYSNOBAL_FORCING_COLUMNS.join(","));
    if header != expected_header {
        return Err(PhysicsBulkError::Csv {
            path: path.to_path_buf(),
            detail: format!("unexpected header '{header}', expected '{expected_header}'"),
        });
    }
    let mut rows = Vec::new();
    for (line_number, line) in lines.enumerate() {
        if line.trim().is_empty() {
            continue;
        }
        rows.push(parse_forcing_line(path, line_number + 2, line)?);
    }
    if rows.is_empty() {
        return Err(PhysicsBulkError::Csv {
            path: path.to_path_buf(),
            detail: "forcing contains no rows".to_string(),
        });
    }
    Ok(rows)
}

fn parse_forcing_line(
    path: &Path,
    line_number: usize,
    line: &str,
) -> Result<PhysicsBulkForcingRow, PhysicsBulkError> {
    let columns = line.split(',').collect::<Vec<_>>();
    if columns.len() != PYSNOBAL_FORCING_COLUMNS.len() + 1 {
        return Err(PhysicsBulkError::Csv {
            path: path.to_path_buf(),
            detail: format!(
                "line {line_number} has {} columns, expected {}",
                columns.len(),
                PYSNOBAL_FORCING_COLUMNS.len() + 1
            ),
        });
    }
    let timestamp = columns[0].to_string();
    let date = timestamp
        .get(0..10)
        .ok_or_else(|| PhysicsBulkError::Csv {
            path: path.to_path_buf(),
            detail: format!("line {line_number} timestamp '{timestamp}' is too short"),
        })?
        .to_string();
    let row = PhysicsBulkForcingRow {
        date,
        net_solar_w_m2: parse_column(path, line_number, "net_solar_Wm-2", columns[1])?,
        temp_air_c: parse_column(path, line_number, "temp_air_degC", columns[3])?,
        wind_speed_m_s: parse_column(path, line_number, "wind_speed_ms-1", columns[6])?,
        precip_mass_kg_m2: parse_column(path, line_number, "precip_mass_mm", columns[7])?,
        snow_precip_fraction: parse_column(path, line_number, "snow_precip_fraction", columns[9])?,
    };
    validate_forcing_row(path, line_number, &row)?;
    Ok(row)
}

fn parse_column(
    path: &Path,
    line_number: usize,
    field: &'static str,
    raw: &str,
) -> Result<f64, PhysicsBulkError> {
    let value = raw.parse::<f64>().map_err(|source| PhysicsBulkError::Csv {
        path: path.to_path_buf(),
        detail: format!("line {line_number} field {field} is not numeric '{raw}': {source}"),
    })?;
    if !value.is_finite() {
        return Err(PhysicsBulkError::Csv {
            path: path.to_path_buf(),
            detail: format!("line {line_number} field {field} is non-finite: {value}"),
        });
    }
    Ok(value)
}

fn validate_forcing_row(
    path: &Path,
    line_number: usize,
    row: &PhysicsBulkForcingRow,
) -> Result<(), PhysicsBulkError> {
    if row.net_solar_w_m2 < 0.0 || row.wind_speed_m_s < 0.0 || row.precip_mass_kg_m2 < 0.0 {
        return Err(PhysicsBulkError::Csv {
            path: path.to_path_buf(),
            detail: format!("line {line_number} has negative radiation, wind, or precipitation"),
        });
    }
    if !(0.0..=1.0).contains(&row.snow_precip_fraction) {
        return Err(PhysicsBulkError::Csv {
            path: path.to_path_buf(),
            detail: format!(
                "line {line_number} snow_precip_fraction must be in [0,1], observed {}",
                row.snow_precip_fraction
            ),
        });
    }
    Ok(())
}

fn simulate_physics_bulk(
    forcing: &[PhysicsBulkForcingRow],
    constants: PhysicsBulkConstants,
    model_id: &'static str,
) -> Result<PhysicsBulkSimulation, PhysicsBulkError> {
    let mut state = PhysicsBulkState::default();
    let mut ledger = SimulationLedger::default();
    let mut daily_rows = Vec::new();
    let mut current_date =
        forcing
            .first()
            .map(|row| row.date.clone())
            .ok_or_else(|| PhysicsBulkError::Invalid {
                detail: "physics_bulk forcing is empty".to_string(),
            })?;
    let mut daily_release = 0.0;
    for row in forcing {
        if row.date != current_date {
            daily_rows.push(daily_row(&current_date, &state, daily_release, model_id));
            current_date.clone_from(&row.date);
            daily_release = 0.0;
        }
        daily_release += step_physics_bulk(row, &mut state, &mut ledger, constants)?;
        update_density_extrema(&state, &mut ledger);
    }
    daily_rows.push(daily_row(&current_date, &state, daily_release, model_id));
    let summary = PhysicsBulkSummary {
        total_snow_input_kg_m2: ledger.total_snow_input_kg_m2,
        total_rain_on_snow_input_kg_m2: ledger.total_rain_on_snow_input_kg_m2,
        total_liquid_release_kg_m2: ledger.total_liquid_release_kg_m2,
        final_swe_kg_m2: state.total_mass_kg_m2(),
        final_depth_m: state.depth_m(),
        final_density_kg_m3: state.observed_density_kg_m3(),
        final_liquid_water_kg_m2: state.liquid_water_kg_m2,
        final_cold_content_j_m2: state.cold_content_j_m2,
        max_abs_mass_balance_residual_kg_m2: ledger.max_abs_mass_balance_residual_kg_m2,
        max_abs_cold_content_residual_j_m2: ledger.max_abs_cold_content_residual_j_m2,
        max_density_kg_m3: ledger.max_density_kg_m3,
        min_nonzero_density_kg_m3: ledger.min_nonzero_density_kg_m3,
    };
    Ok(PhysicsBulkSimulation {
        daily_rows,
        summary,
    })
}

fn step_physics_bulk(
    row: &PhysicsBulkForcingRow,
    state: &mut PhysicsBulkState,
    ledger: &mut SimulationLedger,
    constants: PhysicsBulkConstants,
) -> Result<f64, PhysicsBulkError> {
    let initial_total_mass = state.total_mass_kg_m2();
    let initial_cold = state.cold_content_j_m2;
    let snow_input = row.precip_mass_kg_m2 * row.snow_precip_fraction;
    let rain_input = row.precip_mass_kg_m2 - snow_input;
    let mut rain_on_snow = 0.0;
    let mut cold_added = 0.0;
    if snow_input > ZERO_MASS_KG_M2 {
        add_fresh_snow(row, state, snow_input, constants)?;
        cold_added += snow_input * SPECIFIC_HEAT_ICE_J_KG_K * (-row.temp_air_c.min(0.0));
        ledger.total_snow_input_kg_m2 += snow_input;
    }
    if state.has_snow() {
        rain_on_snow = rain_input;
        state.liquid_water_kg_m2 += rain_on_snow;
        ledger.total_rain_on_snow_input_kg_m2 += rain_on_snow;
    }

    cold_added += ambient_cold_content_added(row, state, constants);
    state.cold_content_j_m2 += cold_added;
    let warm_energy = warm_energy_available_j_m2(row, rain_on_snow, constants);
    let warm_to_cold = warm_energy.min(state.cold_content_j_m2);
    state.cold_content_j_m2 -= warm_to_cold;
    let remaining_warm_energy = warm_energy - warm_to_cold;
    let melt = (remaining_warm_energy / LATENT_HEAT_FUSION_J_KG).min(state.ice_mass_kg_m2);
    if melt > 0.0 {
        state.ice_mass_kg_m2 -= melt;
        state.liquid_water_kg_m2 += melt;
    }

    let refreeze = refreeze_liquid_water(state);
    let cold_used_for_refreeze = refreeze * LATENT_HEAT_FUSION_J_KG;
    if melt + rain_on_snow > ZERO_MASS_KG_M2 {
        apply_wet_compaction(state, melt + rain_on_snow, constants);
    }
    if state.has_snow() {
        apply_time_compaction(state, constants);
    }
    let release = release_excess_liquid(state, constants);
    ledger.total_liquid_release_kg_m2 += release;
    maybe_reset_empty_snowpack(state);

    let final_total_mass = state.total_mass_kg_m2();
    let mass_residual =
        snow_input + rain_on_snow - release - (final_total_mass - initial_total_mass);
    ledger.max_abs_mass_balance_residual_kg_m2 = ledger
        .max_abs_mass_balance_residual_kg_m2
        .max(mass_residual.abs());
    let cold_residual = state.cold_content_j_m2
        - (initial_cold + cold_added - warm_to_cold - cold_used_for_refreeze);
    ledger.max_abs_cold_content_residual_j_m2 = ledger
        .max_abs_cold_content_residual_j_m2
        .max(cold_residual.abs());
    state.snow_cover_age_h = if state.has_snow() {
        state.snow_cover_age_h + 1.0
    } else {
        0.0
    };
    Ok(release)
}

fn add_fresh_snow(
    row: &PhysicsBulkForcingRow,
    state: &mut PhysicsBulkState,
    snow_input_kg_m2: f64,
    constants: PhysicsBulkConstants,
) -> Result<(), PhysicsBulkError> {
    let fresh_density = fresh_snow_density_kg_m3(row.temp_air_c, constants)?;
    let old_depth = state.depth_m();
    state.ice_mass_kg_m2 += snow_input_kg_m2;
    let new_depth = old_depth + snow_input_kg_m2 / fresh_density;
    state.density_kg_m3 = if new_depth > 0.0 {
        state.total_mass_kg_m2() / new_depth
    } else {
        0.0
    };
    Ok(())
}

fn ambient_cold_content_added(
    row: &PhysicsBulkForcingRow,
    state: &PhysicsBulkState,
    constants: PhysicsBulkConstants,
) -> f64 {
    if !state.has_snow() || row.temp_air_c >= 0.0 {
        return 0.0;
    }
    state.ice_mass_kg_m2
        * SPECIFIC_HEAT_ICE_J_KG_K
        * (-row.temp_air_c)
        * constants.subfreezing_cold_content_relaxation_per_hour
}

fn warm_energy_available_j_m2(
    row: &PhysicsBulkForcingRow,
    rain_on_snow_kg_m2: f64,
    constants: PhysicsBulkConstants,
) -> f64 {
    let degree_day_energy = row.temp_air_c.max(0.0)
        * constants.positive_degree_melt_kg_m2_per_c_hour
        * LATENT_HEAT_FUSION_J_KG;
    let rain_heat = rain_on_snow_kg_m2 * SPECIFIC_HEAT_WATER_J_KG_K * row.temp_air_c.max(0.0);
    let solar_energy = if row.temp_air_c > -1.0 {
        row.net_solar_w_m2 * 3_600.0 * constants.solar_melt_efficiency
    } else {
        0.0
    };
    degree_day_energy + rain_heat + solar_energy
}

fn refreeze_liquid_water(state: &mut PhysicsBulkState) -> f64 {
    if state.liquid_water_kg_m2 <= ZERO_MASS_KG_M2 || state.cold_content_j_m2 <= 0.0 {
        return 0.0;
    }
    let refreeze = state
        .liquid_water_kg_m2
        .min(state.cold_content_j_m2 / LATENT_HEAT_FUSION_J_KG);
    state.liquid_water_kg_m2 -= refreeze;
    state.ice_mass_kg_m2 += refreeze;
    state.cold_content_j_m2 -= refreeze * LATENT_HEAT_FUSION_J_KG;
    refreeze
}

fn apply_time_compaction(state: &mut PhysicsBulkState, constants: PhysicsBulkConstants) {
    let density = state.observed_density_kg_m3();
    if density <= 0.0 || density >= constants.dry_compaction_max_density_kg_m3 {
        return;
    }
    let swe = state.total_mass_kg_m2();
    let snow_temp_c = state.bulk_temperature_c();
    let rate = if swe >= constants.dry_compaction_swe_max_kg_m2 {
        1.0
    } else {
        23.5 * (std::f64::consts::PI * swe / constants.dry_compaction_swe_max_kg_m2).cos() + 24.5
    };
    let c11 = if density < 100.0 {
        1.0
    } else {
        (-0.046 * (density - 100.0)).exp()
    };
    let freeze_minus_snow_temp = -snow_temp_c;
    let destructive_metamorphism = 0.01 * c11 * (-0.04 * freeze_minus_snow_temp).exp() / rate;
    let overburden_compaction = 0.026
        * (-0.08 * freeze_minus_snow_temp).exp()
        * swe
        * (-21.0 * (density / RHO_WATER_KG_M3)).exp()
        / rate;
    state.density_kg_m3 = (density
        + constants.dry_compaction_multiplier
            * (destructive_metamorphism + overburden_compaction)
            * density)
        .min(constants.dry_compaction_max_density_kg_m3);
}

fn apply_wet_compaction(
    state: &mut PhysicsBulkState,
    liquid_added_kg_m2: f64,
    constants: PhysicsBulkConstants,
) {
    let density = state.observed_density_kg_m3();
    if density <= 0.0 || density >= constants.wet_compaction_max_density_kg_m3 {
        return;
    }
    let total_mass = state.total_mass_kg_m2();
    if total_mass <= ZERO_MASS_KG_M2 {
        return;
    }
    let h2o_added_ratio = liquid_added_kg_m2 / total_mass;
    if h2o_added_ratio <= 1.0e-6 {
        return;
    }
    let density_delta = constants.wet_compaction_multiplier
        * (constants.wet_compaction_max_density_kg_m3 - density)
        / (1.0 + constants.wet_compaction_half_saturation_ratio / h2o_added_ratio);
    state.density_kg_m3 = (density + density_delta).min(constants.wet_compaction_max_density_kg_m3);
}

fn release_excess_liquid(state: &mut PhysicsBulkState, constants: PhysicsBulkConstants) -> f64 {
    if !state.has_snow() {
        let release = state.liquid_water_kg_m2;
        state.liquid_water_kg_m2 = 0.0;
        return release;
    }
    let capacity = liquid_water_capacity_kg_m2(state, constants);
    if state.liquid_water_kg_m2 <= capacity {
        return 0.0;
    }
    let release = state.liquid_water_kg_m2 - capacity;
    state.liquid_water_kg_m2 = capacity;
    release
}

fn liquid_water_capacity_kg_m2(state: &PhysicsBulkState, constants: PhysicsBulkConstants) -> f64 {
    let density = state.observed_density_kg_m3();
    if density <= 0.0 || density >= RHO_ICE_KG_M3 {
        return 0.0;
    }
    let capacity_factor =
        constants.max_liquid_water_volume_fraction * RHO_WATER_KG_M3 * (RHO_ICE_KG_M3 - density)
            / (RHO_ICE_KG_M3 * density);
    if capacity_factor >= 1.0 {
        return state.liquid_water_kg_m2;
    }
    capacity_factor * state.ice_mass_kg_m2 / (1.0 - capacity_factor)
}

fn maybe_reset_empty_snowpack(state: &mut PhysicsBulkState) {
    if state.ice_mass_kg_m2 > ZERO_MASS_KG_M2 || state.liquid_water_kg_m2 > ZERO_MASS_KG_M2 {
        if state.density_kg_m3 <= 0.0 {
            state.density_kg_m3 = physics_bulk_constants().new_snow_density_min_kg_m3;
        }
        return;
    }
    *state = PhysicsBulkState::default();
}

fn daily_row(
    date: &str,
    state: &PhysicsBulkState,
    daily_release_kg_m2: f64,
    model_id: &'static str,
) -> PhysicsBulkDailyRow {
    PhysicsBulkDailyRow {
        date: date.to_string(),
        snow_water_m: state.total_mass_kg_m2() / RHO_WATER_KG_M3,
        snow_depth_m: state.depth_m(),
        snow_density_kg_m3: state.observed_density_kg_m3(),
        liquid_water_m: state.liquid_water_kg_m2 / RHO_WATER_KG_M3,
        cold_content_j_m2: state.cold_content_j_m2,
        snow_bulk_temperature_c: state.bulk_temperature_c(),
        released_liquid_m: daily_release_kg_m2 / RHO_WATER_KG_M3,
        source: model_id,
    }
}

fn update_density_extrema(state: &PhysicsBulkState, ledger: &mut SimulationLedger) {
    let density = state.observed_density_kg_m3();
    ledger.max_density_kg_m3 = ledger.max_density_kg_m3.max(density);
    if density > 0.0 {
        ledger.min_nonzero_density_kg_m3 = Some(
            ledger
                .min_nonzero_density_kg_m3
                .map_or(density, |current| current.min(density)),
        );
    }
}

pub fn fresh_snow_density_kg_m3(
    temp_air_c: f64,
    constants: PhysicsBulkConstants,
) -> Result<f64, PhysicsBulkError> {
    if !temp_air_c.is_finite() {
        return Err(PhysicsBulkError::Invalid {
            detail: format!("fresh-snow temperature is non-finite: {temp_air_c}"),
        });
    }
    let density = if temp_air_c <= constants.new_snow_density_temperature_threshold_c {
        constants.new_snow_density_base_kg_m3
    } else {
        constants.new_snow_density_base_kg_m3
            + constants.new_snow_density_temperature_coefficient
                * (temp_air_c - constants.new_snow_density_temperature_threshold_c).powf(1.5)
    };
    Ok(density.clamp(
        constants.new_snow_density_min_kg_m3,
        constants.new_snow_density_max_kg_m3,
    ))
}

impl PhysicsBulkState {
    fn has_snow(&self) -> bool {
        self.total_mass_kg_m2() > ZERO_MASS_KG_M2
    }

    fn total_mass_kg_m2(&self) -> f64 {
        self.ice_mass_kg_m2 + self.liquid_water_kg_m2
    }

    fn depth_m(&self) -> f64 {
        let total = self.total_mass_kg_m2();
        if total <= ZERO_MASS_KG_M2 || self.density_kg_m3 <= 0.0 {
            0.0
        } else {
            total / self.density_kg_m3
        }
    }

    fn observed_density_kg_m3(&self) -> f64 {
        let depth = self.depth_m();
        if depth <= 0.0 {
            0.0
        } else {
            self.total_mass_kg_m2() / depth
        }
    }

    fn bulk_temperature_c(&self) -> f64 {
        if self.ice_mass_kg_m2 <= ZERO_MASS_KG_M2 {
            0.0
        } else {
            (-self.cold_content_j_m2 / (self.ice_mass_kg_m2 * SPECIFIC_HEAT_ICE_J_KG_K))
                .clamp(-30.0, 0.0)
        }
    }
}

fn write_physics_bulk_csv(path: &Path, rows: &[PhysicsBulkDailyRow]) -> Result<(), SnowbenchError> {
    let mut file = fs::File::create(path).map_err(|source| snowbench_io(path, source))?;
    writeln!(
        file,
        "date,snow_water_m,snow_depth_m,snow_density_kg_m3,liquid_water_m,cold_content_j_m2,snow_bulk_temperature_c,released_liquid_m,source"
    )
    .map_err(|source| snowbench_io(path, source))?;
    for row in rows {
        writeln!(
            file,
            "{},{:.12},{:.12},{:.12},{:.12},{:.12},{:.12},{:.12},{}",
            row.date,
            row.snow_water_m,
            row.snow_depth_m,
            row.snow_density_kg_m3,
            row.liquid_water_m,
            row.cold_content_j_m2,
            row.snow_bulk_temperature_c,
            row.released_liquid_m,
            row.source
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
    write_text(path, &(text + "\n"))
}

fn write_markdown(
    path: &Path,
    report: &PhysicsBulkReport,
    forcing_dir: &str,
) -> Result<(), SnowbenchError> {
    let text = format!(
        "# Physics-Bulk Snowbench Summary\n\n\
         - Schema: `{}`\n\
         - Model: `{}`\n\
         - Variant: `{}`\n\
         - Contract: `{}`\n\
         - Runtime coupling: `{}`\n\
         - No site constants: `{}`\n\
         - Days: `{}`\n\
         - Hourly rows: `{}`\n\
         - Positive snow hours: `{}`\n\
         - Total snow input: `{:.6}` kg m^-2\n\
         - Total rain-on-snow input: `{:.6}` kg m^-2\n\
         - Total liquid release: `{:.6}` kg m^-2\n\
         - Final SWE: `{:.6}` kg m^-2\n\
         - Final depth: `{:.6}` m\n\
         - Final density: `{:.6}` kg m^-3\n\
         - Max mass residual: `{:.12}` kg m^-2\n\
         - Max cold-content residual: `{:.12}` J m^-2\n\
         - Forcing bridge: `{}`\n",
        report.schema,
        report.model_id,
        report.variant,
        report.contract,
        report.runtime_coupling,
        report.no_site_constants,
        report.day_count,
        report.hourly_row_count,
        report.positive_snow_hours,
        report.summary.total_snow_input_kg_m2,
        report.summary.total_rain_on_snow_input_kg_m2,
        report.summary.total_liquid_release_kg_m2,
        report.summary.final_swe_kg_m2,
        report.summary.final_depth_m,
        report.summary.final_density_kg_m3,
        report.summary.max_abs_mass_balance_residual_kg_m2,
        report.summary.max_abs_cold_content_residual_j_m2,
        forcing_dir,
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
    use super::{
        PhysicsBulkConstants, PhysicsBulkForcingRow, PhysicsBulkState, PhysicsBulkVariant,
        SimulationLedger, fresh_snow_density_kg_m3, liquid_water_capacity_kg_m2,
        physics_bulk_constants, physics_bulk_constants_for_variant, release_excess_liquid,
        simulate_physics_bulk, step_physics_bulk,
    };

    #[test]
    fn fresh_snow_density_is_bounded_and_warmer_snow_is_denser() {
        let constants = physics_bulk_constants();
        let cold = fresh_snow_density_kg_m3(-20.0, constants).expect("finite");
        let mid = fresh_snow_density_kg_m3(-8.0, constants).expect("finite");
        let warm = fresh_snow_density_kg_m3(0.0, constants).expect("finite");

        assert!((cold - constants.new_snow_density_min_kg_m3).abs() < 1.0e-12);
        assert!(mid > cold);
        assert!(warm > mid);
        assert!(warm <= constants.new_snow_density_max_kg_m3);
    }

    #[test]
    fn named_variants_are_global_and_distinct_from_candidate_v1() {
        let candidate = physics_bulk_constants_for_variant(PhysicsBulkVariant::CandidateV1);
        let slow = physics_bulk_constants_for_variant(PhysicsBulkVariant::SlowMeltV1);
        let dense = physics_bulk_constants_for_variant(PhysicsBulkVariant::DenseSlowMeltV1);

        assert_eq!(
            PhysicsBulkVariant::parse("candidate_v1").expect("variant"),
            PhysicsBulkVariant::CandidateV1
        );
        assert!(PhysicsBulkVariant::parse("site_specific").is_err());
        assert!(
            slow.positive_degree_melt_kg_m2_per_c_hour
                < candidate.positive_degree_melt_kg_m2_per_c_hour
        );
        assert!(dense.new_snow_density_base_kg_m3 > candidate.new_snow_density_base_kg_m3);
        assert!(dense.dry_compaction_multiplier > candidate.dry_compaction_multiplier);
    }

    #[test]
    fn dry_compaction_increases_density_without_mass_drift() {
        let constants = physics_bulk_constants();
        let forcing = sample_row("2000-01-01 00:00:00", -5.0, 0.0, 0.0, 1.0);
        let mut state = PhysicsBulkState {
            ice_mass_kg_m2: 250.0,
            liquid_water_kg_m2: 0.0,
            density_kg_m3: 120.0,
            cold_content_j_m2: 250.0 * 2_100.0 * 5.0,
            snow_cover_age_h: 0.0,
        };
        let before_mass = state.total_mass_kg_m2();
        let before_density = state.observed_density_kg_m3();
        let mut ledger = SimulationLedger::default();

        step_physics_bulk(&forcing, &mut state, &mut ledger, constants).expect("step should run");

        assert!((state.total_mass_kg_m2() - before_mass).abs() < 1.0e-9);
        assert!(state.observed_density_kg_m3() > before_density);
        assert!(ledger.max_abs_mass_balance_residual_kg_m2 < 1.0e-9);
    }

    #[test]
    fn wet_compaction_releases_excess_liquid_and_preserves_mass_balance() {
        let constants = physics_bulk_constants();
        let forcing = sample_row("2000-01-01 00:00:00", 2.0, 0.0, 40.0, 0.0);
        let mut state = PhysicsBulkState {
            ice_mass_kg_m2: 200.0,
            liquid_water_kg_m2: 0.0,
            density_kg_m3: 180.0,
            cold_content_j_m2: 0.0,
            snow_cover_age_h: 0.0,
        };
        let mut ledger = SimulationLedger::default();

        let release =
            step_physics_bulk(&forcing, &mut state, &mut ledger, constants).expect("rain step");

        assert!(release > 0.0);
        assert!(
            state.liquid_water_kg_m2 <= liquid_water_capacity_kg_m2(&state, constants) + 1.0e-9
        );
        assert!(state.observed_density_kg_m3() > 180.0);
        assert!(ledger.max_abs_mass_balance_residual_kg_m2 < 1.0e-9);
    }

    #[test]
    fn subfreezing_pack_refreezes_retained_liquid_using_cold_content() {
        let constants = physics_bulk_constants();
        let mut state = PhysicsBulkState {
            ice_mass_kg_m2: 100.0,
            liquid_water_kg_m2: 2.0,
            density_kg_m3: 150.0,
            cold_content_j_m2: 2.0 * 333_500.0,
            snow_cover_age_h: 0.0,
        };
        let before_liquid = state.liquid_water_kg_m2;
        let before_ice = state.ice_mass_kg_m2;
        let mut ledger = SimulationLedger::default();
        let forcing = sample_row("2000-01-01 00:00:00", -4.0, 0.0, 0.0, 0.0);

        step_physics_bulk(&forcing, &mut state, &mut ledger, constants).expect("cold step");

        assert!(state.liquid_water_kg_m2 < before_liquid);
        assert!(state.ice_mass_kg_m2 > before_ice);
        assert!(ledger.max_abs_cold_content_residual_j_m2 < 1.0e-6);
    }

    #[test]
    fn sample_sequence_closes_mass_and_emits_daily_rows() {
        let constants = physics_bulk_constants();
        let rows = vec![
            sample_row("2000-01-01 00:00:00", -6.0, 0.0, 10.0, 1.0),
            sample_row("2000-01-01 01:00:00", -3.0, 0.0, 0.0, 0.0),
            sample_row("2000-01-02 00:00:00", 3.0, 120.0, 5.0, 0.0),
        ];

        let simulation =
            simulate_physics_bulk(&rows, constants, "test_variant").expect("simulation should run");

        assert_eq!(simulation.daily_rows.len(), 2);
        assert_eq!(simulation.daily_rows[0].source, "test_variant");
        assert!(simulation.summary.total_snow_input_kg_m2 > 0.0);
        assert!(simulation.summary.total_liquid_release_kg_m2 > 0.0);
        assert!(simulation.summary.max_abs_mass_balance_residual_kg_m2 < 1.0e-9);
        assert!(simulation.summary.max_density_kg_m3 <= constants.wet_compaction_max_density_kg_m3);
    }

    #[test]
    fn excess_liquid_release_ignores_bare_ground_rain() {
        let constants = PhysicsBulkConstants {
            max_liquid_water_volume_fraction: 0.01,
            ..physics_bulk_constants()
        };
        let mut state = PhysicsBulkState {
            liquid_water_kg_m2: 3.0,
            ..PhysicsBulkState::default()
        };

        let release = release_excess_liquid(&mut state, constants);

        assert!((release - 3.0).abs() < 1.0e-12);
        assert!(state.liquid_water_kg_m2.abs() < 1.0e-12);
    }

    fn sample_row(
        timestamp: &'static str,
        temp_air_c: f64,
        net_solar_w_m2: f64,
        precip_mass_kg_m2: f64,
        snow_precip_fraction: f64,
    ) -> PhysicsBulkForcingRow {
        PhysicsBulkForcingRow {
            date: timestamp[0..10].to_string(),
            net_solar_w_m2,
            temp_air_c,
            wind_speed_m_s: 1.0,
            precip_mass_kg_m2,
            snow_precip_fraction,
        }
    }
}
