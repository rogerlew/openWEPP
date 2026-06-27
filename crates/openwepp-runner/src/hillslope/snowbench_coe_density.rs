use std::collections::BTreeMap;
use std::fs;
use std::io::{self, Write};
use std::path::{Path, PathBuf};

use serde::Serialize;

use super::snowbench::{PYSNOBAL_FORCING_COLUMNS, SnowbenchError};
use super::snowbench_coe_melt::{CoeMeltModel, CoeMeltRequest, run_coe_melt_snowbench};
use super::snowbench_physics_bulk::{
    PhysicsBulkConstants, PhysicsBulkVariant, fresh_snow_density_kg_m3,
    physics_bulk_constants_for_variant,
};

const CONTRACT: &str = "SC-SNOWFREEZE-001 INV-SNOWFREEZE-050 INV-SNOWFREEZE-058 INV-SNOWFREEZE-059 OBL-SNOWFREEZE-P-034";
const RHO_WATER_KG_M3: f64 = 1_000.0;
const ZERO_MASS_KG_M2: f64 = 1.0e-9;
const DAILY_COMPACTION_STEPS: u8 = 24;

#[derive(Debug, Clone)]
pub struct CoeBoundDensityRequest {
    pub run_dir: PathBuf,
    pub run_file: Option<PathBuf>,
    pub output_dir: PathBuf,
    pub coe_model: CoeMeltModel,
    pub density_variant: PhysicsBulkVariant,
}

#[derive(Debug, Clone, Serialize)]
pub struct CoeBoundDensityReport {
    pub schema: &'static str,
    pub model_id: String,
    pub coe_boundary_model: &'static str,
    pub density_variant: &'static str,
    pub contract: &'static str,
    pub runtime_coupling: &'static str,
    pub no_site_constants: bool,
    pub output_dir: String,
    pub coe_boundary_dir: String,
    pub day_count: usize,
    pub hourly_row_count: usize,
    pub constants: PhysicsBulkConstants,
    pub summary: CoeBoundDensitySummary,
}

#[derive(Debug, Clone, Serialize)]
pub struct CoeBoundDensitySummary {
    pub total_snow_input_kg_m2: f64,
    pub total_rain_input_kg_m2: f64,
    pub total_boundary_swe_loss_m: f64,
    pub total_boundary_routed_melt_m: f64,
    pub final_swe_m: f64,
    pub final_depth_m: f64,
    pub final_density_kg_m3: f64,
    pub max_abs_coe_swe_identity_residual_m: f64,
    pub max_abs_unbounded_swe_residual_m: f64,
    pub max_density_kg_m3: f64,
    pub min_nonzero_density_kg_m3: Option<f64>,
}

#[derive(Debug, Clone)]
struct CoeBoundaryRow {
    date: String,
    snow_water_m: f64,
    snow_depth_m: f64,
    snow_density_kg_m3: f64,
    routed_melt_m: f64,
    snowpack_swe_loss_m: f64,
}

#[derive(Debug, Clone)]
struct DailyForcing {
    date: String,
    snow_input_kg_m2: f64,
    rain_input_kg_m2: f64,
    mean_air_temperature_c: f64,
    hourly_row_count: usize,
}

#[derive(Debug, Clone, Copy, Default)]
struct CoeBoundDensityState {
    mass_kg_m2: f64,
    density_kg_m3: f64,
}

#[derive(Debug, Clone, Serialize)]
struct CoeBoundDensityDailyRow {
    date: String,
    snow_water_m: f64,
    snow_depth_m: f64,
    snow_density_kg_m3: f64,
    coe_snow_water_m: f64,
    coe_snow_depth_m: f64,
    coe_snow_density_kg_m3: f64,
    coe_snowpack_swe_loss_m: f64,
    coe_routed_melt_m: f64,
    source: String,
}

#[derive(Debug, Default)]
struct CoeBoundDensityLedger {
    total_snow_input_kg_m2: f64,
    total_rain_input_kg_m2: f64,
    total_boundary_swe_loss_m: f64,
    total_boundary_routed_melt_m: f64,
    max_abs_coe_swe_identity_residual_m: f64,
    max_abs_unbounded_swe_residual_m: f64,
    max_density_kg_m3: f64,
    min_nonzero_density_kg_m3: Option<f64>,
    hourly_row_count: usize,
}

pub fn run_coe_bound_density_snowbench(
    request: &CoeBoundDensityRequest,
) -> Result<CoeBoundDensityReport, SnowbenchError> {
    if !matches!(
        request.density_variant,
        PhysicsBulkVariant::DensityCompactionV1 | PhysicsBulkVariant::SpringDensificationV1
    ) {
        return Err(SnowbenchError::InvalidInput {
            detail: "coe-bound-density currently accepts only density_compaction_v1 or spring_densification_v1".to_string(),
        });
    }
    let output_dir = absolute_path(&request.output_dir)?;
    fs::create_dir_all(&output_dir).map_err(|source| snowbench_io(&output_dir, source))?;
    let coe_boundary_dir = output_dir.join("coe_boundary");
    let coe_report = run_coe_melt_snowbench(&CoeMeltRequest {
        run_dir: request.run_dir.clone(),
        run_file: request.run_file.clone(),
        output_dir: coe_boundary_dir.clone(),
        model: request.coe_model,
    })?;
    let forcing_path = PathBuf::from(&coe_report.forcing_bridge_dir)
        .join("tg_0p0c_zg0p10m")
        .join("forcing.csv");
    let forcing = read_daily_forcing(&forcing_path)?;
    let boundary = read_coe_boundary(&coe_boundary_dir.join("coe_melt_snow.csv"))?;
    let constants = physics_bulk_constants_for_variant(request.density_variant);
    let model_id = format!(
        "coe_bound_{}_{}",
        request.density_variant.name(),
        request.coe_model.name()
    );
    let (daily_rows, summary) =
        simulate_coe_bound_density(&forcing, &boundary, constants, &model_id)?;
    write_coe_bound_density_csv(&output_dir.join("coe_bound_density_snow.csv"), &daily_rows)?;
    let report = CoeBoundDensityReport {
        schema: "snowdensity06b-coe-bound-density-snowbench-v1",
        model_id,
        coe_boundary_model: request.coe_model.name(),
        density_variant: request.density_variant.name(),
        contract: CONTRACT,
        runtime_coupling: "none; offline CoE-bound density replay only",
        no_site_constants: true,
        output_dir: output_dir.display().to_string(),
        coe_boundary_dir: coe_boundary_dir.display().to_string(),
        day_count: daily_rows.len(),
        hourly_row_count: summary_hourly_count(&forcing),
        constants,
        summary,
    };
    write_json(&output_dir.join("coe_bound_density_summary.json"), &report)?;
    write_markdown(&output_dir.join("coe_bound_density_summary.md"), &report)?;
    Ok(report)
}

fn absolute_path(path: &Path) -> Result<PathBuf, SnowbenchError> {
    if path.is_absolute() {
        return Ok(path.to_path_buf());
    }
    let cwd = std::env::current_dir().map_err(|source| snowbench_io(".", source))?;
    Ok(cwd.join(path))
}

fn read_daily_forcing(path: &Path) -> Result<Vec<DailyForcing>, SnowbenchError> {
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
    let mut by_date: BTreeMap<String, DailyForcingAccumulator> = BTreeMap::new();
    for (line_index, line) in lines.enumerate() {
        if line.trim().is_empty() {
            continue;
        }
        let row = parse_forcing_line(path, line_index + 2, line)?;
        by_date.entry(row.date.clone()).or_default().accumulate(row);
    }
    if by_date.is_empty() {
        return Err(SnowbenchError::InvalidForcing {
            detail: format!("{} contained no forcing rows", path.display()),
        });
    }
    by_date
        .into_values()
        .map(DailyForcingAccumulator::finish)
        .collect()
}

#[derive(Debug, Default)]
struct DailyForcingAccumulator {
    date: String,
    snow_input_kg_m2: f64,
    rain_input_kg_m2: f64,
    air_temperature_sum_c: f64,
    hourly_row_count: usize,
}

#[derive(Debug)]
struct HourlyForcing {
    date: String,
    snow_input_kg_m2: f64,
    rain_input_kg_m2: f64,
    air_temperature_c: f64,
}

impl DailyForcingAccumulator {
    fn accumulate(&mut self, row: HourlyForcing) {
        if self.date.is_empty() {
            self.date = row.date;
        }
        self.snow_input_kg_m2 += row.snow_input_kg_m2;
        self.rain_input_kg_m2 += row.rain_input_kg_m2;
        self.air_temperature_sum_c += row.air_temperature_c;
        self.hourly_row_count += 1;
    }

    fn finish(self) -> Result<DailyForcing, SnowbenchError> {
        if self.hourly_row_count != 24 {
            return Err(SnowbenchError::InvalidForcing {
                detail: format!(
                    "date {} has {} CoE-bound density hourly rows, expected 24",
                    self.date, self.hourly_row_count
                ),
            });
        }
        Ok(DailyForcing {
            date: self.date,
            snow_input_kg_m2: self.snow_input_kg_m2,
            rain_input_kg_m2: self.rain_input_kg_m2,
            mean_air_temperature_c: self.air_temperature_sum_c / 24.0,
            hourly_row_count: self.hourly_row_count,
        })
    }
}

fn parse_forcing_line(
    path: &Path,
    line_number: usize,
    line: &str,
) -> Result<HourlyForcing, SnowbenchError> {
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
        })?
        .to_string();
    let temp_air_c = parse_column(path, line_number, "temp_air_degC", columns[3])?;
    let precip_mass_kg_m2 = parse_column(path, line_number, "precip_mass_mm", columns[7])?;
    let snow_precip_fraction = parse_column(path, line_number, "snow_precip_fraction", columns[9])?;
    if precip_mass_kg_m2 < 0.0 || !(0.0..=1.0).contains(&snow_precip_fraction) {
        return Err(SnowbenchError::InvalidForcing {
            detail: format!(
                "{} line {line_number} has invalid precipitation or snow fraction",
                path.display()
            ),
        });
    }
    let snow_input_kg_m2 = precip_mass_kg_m2 * snow_precip_fraction;
    Ok(HourlyForcing {
        date,
        snow_input_kg_m2,
        rain_input_kg_m2: precip_mass_kg_m2 - snow_input_kg_m2,
        air_temperature_c: temp_air_c,
    })
}

fn read_coe_boundary(path: &Path) -> Result<Vec<CoeBoundaryRow>, SnowbenchError> {
    let text = fs::read_to_string(path).map_err(|source| snowbench_io(path, source))?;
    let mut lines = text.lines();
    let header = lines.next().ok_or_else(|| SnowbenchError::InvalidForcing {
        detail: format!("{} missing header", path.display()),
    })?;
    let header_columns = header.split(',').collect::<Vec<_>>();
    let date_index = coe_boundary_required_column(path, &header_columns, "date")?;
    let snow_water_index = coe_boundary_required_column(path, &header_columns, "snow_water_m")?;
    let snow_depth_index = coe_boundary_required_column(path, &header_columns, "snow_depth_m")?;
    let snow_density_index =
        coe_boundary_required_column(path, &header_columns, "snow_density_kg_m3")?;
    let routed_melt_index = coe_boundary_required_column(path, &header_columns, "routed_melt_m")?;
    let snowpack_swe_loss_index =
        coe_boundary_required_column(path, &header_columns, "snowpack_swe_loss_m")?;
    let mut rows = Vec::new();
    for (line_index, line) in lines.enumerate() {
        if line.trim().is_empty() {
            continue;
        }
        let columns = line.split(',').collect::<Vec<_>>();
        if columns.len() != header_columns.len() {
            return Err(SnowbenchError::InvalidForcing {
                detail: format!(
                    "{} line {} has {} columns, expected {}",
                    path.display(),
                    line_index + 2,
                    columns.len(),
                    header_columns.len()
                ),
            });
        }
        rows.push(CoeBoundaryRow {
            date: columns[date_index].to_string(),
            snow_water_m: parse_column(
                path,
                line_index + 2,
                "snow_water_m",
                columns[snow_water_index],
            )?,
            snow_depth_m: parse_column(
                path,
                line_index + 2,
                "snow_depth_m",
                columns[snow_depth_index],
            )?,
            snow_density_kg_m3: parse_column(
                path,
                line_index + 2,
                "snow_density_kg_m3",
                columns[snow_density_index],
            )?,
            routed_melt_m: parse_column(
                path,
                line_index + 2,
                "routed_melt_m",
                columns[routed_melt_index],
            )?,
            snowpack_swe_loss_m: parse_column(
                path,
                line_index + 2,
                "snowpack_swe_loss_m",
                columns[snowpack_swe_loss_index],
            )?,
        });
    }
    if rows.is_empty() {
        return Err(SnowbenchError::InvalidForcing {
            detail: format!("{} contained no CoE boundary rows", path.display()),
        });
    }
    Ok(rows)
}

fn coe_boundary_required_column(
    path: &Path,
    header_columns: &[&str],
    field: &'static str,
) -> Result<usize, SnowbenchError> {
    header_columns
        .iter()
        .position(|column| *column == field)
        .ok_or_else(|| SnowbenchError::InvalidForcing {
            detail: format!(
                "{} CoE boundary header missing required field {field}",
                path.display()
            ),
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
        .map_err(|source| SnowbenchError::InvalidForcing {
            detail: format!(
                "{} line {line_number} field {field} is not numeric '{raw}': {source}",
                path.display()
            ),
        })?;
    if value.is_finite() {
        Ok(value)
    } else {
        Err(SnowbenchError::InvalidForcing {
            detail: format!(
                "{} line {line_number} field {field} is non-finite: {value}",
                path.display()
            ),
        })
    }
}

fn simulate_coe_bound_density(
    forcing: &[DailyForcing],
    boundary: &[CoeBoundaryRow],
    constants: PhysicsBulkConstants,
    model_id: &str,
) -> Result<(Vec<CoeBoundDensityDailyRow>, CoeBoundDensitySummary), SnowbenchError> {
    if forcing.len() != boundary.len() {
        return Err(SnowbenchError::InvalidForcing {
            detail: format!(
                "CoE-bound density replay has {} forcing days and {} boundary days",
                forcing.len(),
                boundary.len()
            ),
        });
    }
    let mut state = CoeBoundDensityState::default();
    let mut ledger = CoeBoundDensityLedger::default();
    let mut rows = Vec::with_capacity(boundary.len());
    for (forcing_day, boundary_day) in forcing.iter().zip(boundary) {
        if forcing_day.date != boundary_day.date {
            return Err(SnowbenchError::InvalidForcing {
                detail: format!(
                    "CoE-bound density date mismatch: forcing {} vs boundary {}",
                    forcing_day.date, boundary_day.date
                ),
            });
        }
        ledger.total_snow_input_kg_m2 += forcing_day.snow_input_kg_m2;
        ledger.total_rain_input_kg_m2 += forcing_day.rain_input_kg_m2;
        ledger.total_boundary_swe_loss_m += boundary_day.snowpack_swe_loss_m;
        ledger.total_boundary_routed_melt_m += boundary_day.routed_melt_m;
        ledger.hourly_row_count += forcing_day.hourly_row_count;

        if forcing_day.snow_input_kg_m2 > ZERO_MASS_KG_M2 {
            add_fresh_snow(
                &mut state,
                forcing_day.snow_input_kg_m2,
                forcing_day.mean_air_temperature_c,
                constants,
            )?;
        }
        apply_daily_compaction(
            &mut state,
            (boundary_day.snowpack_swe_loss_m + boundary_day.routed_melt_m).max(0.0)
                * RHO_WATER_KG_M3,
            forcing_day.mean_air_temperature_c.clamp(-30.0, 0.0),
            constants,
        );
        let unbounded_swe_m = state.mass_kg_m2 / RHO_WATER_KG_M3;
        let coe_swe_m = boundary_day.snow_water_m.max(0.0);
        ledger.max_abs_unbounded_swe_residual_m = ledger
            .max_abs_unbounded_swe_residual_m
            .max((unbounded_swe_m - coe_swe_m).abs());
        state.mass_kg_m2 = coe_swe_m * RHO_WATER_KG_M3;
        if state.mass_kg_m2 <= ZERO_MASS_KG_M2 {
            state = CoeBoundDensityState::default();
        } else if state.density_kg_m3 <= 0.0 {
            state.density_kg_m3 = constants.new_snow_density_min_kg_m3;
        }
        let identity_residual = state.mass_kg_m2 / RHO_WATER_KG_M3 - boundary_day.snow_water_m;
        ledger.max_abs_coe_swe_identity_residual_m = ledger
            .max_abs_coe_swe_identity_residual_m
            .max(identity_residual.abs());
        update_density_extrema(&state, &mut ledger);
        rows.push(CoeBoundDensityDailyRow {
            date: boundary_day.date.clone(),
            snow_water_m: state.mass_kg_m2 / RHO_WATER_KG_M3,
            snow_depth_m: state.depth_m(),
            snow_density_kg_m3: state.observed_density_kg_m3(),
            coe_snow_water_m: boundary_day.snow_water_m,
            coe_snow_depth_m: boundary_day.snow_depth_m,
            coe_snow_density_kg_m3: boundary_day.snow_density_kg_m3,
            coe_snowpack_swe_loss_m: boundary_day.snowpack_swe_loss_m,
            coe_routed_melt_m: boundary_day.routed_melt_m,
            source: model_id.to_string(),
        });
    }
    let summary = CoeBoundDensitySummary {
        total_snow_input_kg_m2: ledger.total_snow_input_kg_m2,
        total_rain_input_kg_m2: ledger.total_rain_input_kg_m2,
        total_boundary_swe_loss_m: ledger.total_boundary_swe_loss_m,
        total_boundary_routed_melt_m: ledger.total_boundary_routed_melt_m,
        final_swe_m: state.mass_kg_m2 / RHO_WATER_KG_M3,
        final_depth_m: state.depth_m(),
        final_density_kg_m3: state.observed_density_kg_m3(),
        max_abs_coe_swe_identity_residual_m: ledger.max_abs_coe_swe_identity_residual_m,
        max_abs_unbounded_swe_residual_m: ledger.max_abs_unbounded_swe_residual_m,
        max_density_kg_m3: ledger.max_density_kg_m3,
        min_nonzero_density_kg_m3: ledger.min_nonzero_density_kg_m3,
    };
    Ok((rows, summary))
}

fn add_fresh_snow(
    state: &mut CoeBoundDensityState,
    snow_input_kg_m2: f64,
    air_temperature_c: f64,
    constants: PhysicsBulkConstants,
) -> Result<(), SnowbenchError> {
    let fresh_density =
        fresh_snow_density_kg_m3(air_temperature_c, constants).map_err(SnowbenchError::from)?;
    let new_depth_m = state.depth_m() + snow_input_kg_m2 / fresh_density;
    state.mass_kg_m2 += snow_input_kg_m2;
    state.density_kg_m3 = if new_depth_m > 0.0 {
        state.mass_kg_m2 / new_depth_m
    } else {
        0.0
    };
    Ok(())
}

fn apply_time_compaction(
    state: &mut CoeBoundDensityState,
    snow_temperature_c: f64,
    constants: PhysicsBulkConstants,
) {
    let density = state.observed_density_kg_m3();
    if density <= 0.0 || density >= constants.dry_compaction_max_density_kg_m3 {
        return;
    }
    let swe = state.mass_kg_m2;
    let rate = if swe >= constants.dry_compaction_swe_max_kg_m2 {
        1.0
    } else {
        constants.compaction_rate_cos_amplitude
            * (std::f64::consts::PI * swe / constants.dry_compaction_swe_max_kg_m2).cos()
            + constants.compaction_rate_offset
    };
    let c11 = if density < constants.ptm_density_threshold_kg_m3 {
        1.0
    } else {
        (-constants.ptm_density_decay_m3_per_kg * (density - constants.ptm_density_threshold_kg_m3))
            .exp()
    };
    let freeze_minus_snow_temp = -snow_temperature_c.min(0.0);
    let destructive_metamorphism = constants.ptm_rate_per_hour
        * c11
        * (-constants.ptm_temperature_decay_per_c * freeze_minus_snow_temp).exp()
        / rate;
    let overburden_compaction = constants.poc_rate_per_hour
        * (-constants.poc_temperature_decay_per_c * freeze_minus_snow_temp).exp()
        * swe
        * (-constants.poc_density_decay * (density / RHO_WATER_KG_M3)).exp()
        / rate;
    state.density_kg_m3 = (density
        + constants.dry_compaction_multiplier
            * (destructive_metamorphism + overburden_compaction)
            * density)
        .min(constants.dry_compaction_max_density_kg_m3);
}

fn apply_daily_compaction(
    state: &mut CoeBoundDensityState,
    liquid_for_compaction_kg_m2: f64,
    snow_temperature_c: f64,
    constants: PhysicsBulkConstants,
) {
    let wet_substeps = constants.wet_compaction_substeps_per_day.max(1);
    if wet_substeps == 1 {
        if liquid_for_compaction_kg_m2 > ZERO_MASS_KG_M2 {
            apply_wet_compaction(state, liquid_for_compaction_kg_m2, constants);
        }
        for _ in 0..DAILY_COMPACTION_STEPS {
            apply_time_compaction(state, snow_temperature_c, constants);
        }
        return;
    }

    let liquid_per_step = liquid_for_compaction_kg_m2 / f64::from(wet_substeps);
    if liquid_for_compaction_kg_m2 > ZERO_MASS_KG_M2 {
        apply_wet_compaction(state, liquid_for_compaction_kg_m2, constants);
    }
    for step in 0..DAILY_COMPACTION_STEPS {
        let wet_step = step < wet_substeps && liquid_per_step > ZERO_MASS_KG_M2;
        apply_time_compaction_scaled(
            state,
            snow_temperature_c,
            constants,
            if wet_step {
                constants.wet_compaction_multiplier
            } else {
                1.0
            },
        );
    }
}

fn apply_time_compaction_scaled(
    state: &mut CoeBoundDensityState,
    snow_temperature_c: f64,
    constants: PhysicsBulkConstants,
    multiplier_scale: f64,
) {
    let density = state.observed_density_kg_m3();
    if density <= 0.0 || density >= constants.dry_compaction_max_density_kg_m3 {
        return;
    }
    let swe = state.mass_kg_m2;
    let rate = if swe >= constants.dry_compaction_swe_max_kg_m2 {
        1.0
    } else {
        constants.compaction_rate_cos_amplitude
            * (std::f64::consts::PI * swe / constants.dry_compaction_swe_max_kg_m2).cos()
            + constants.compaction_rate_offset
    };
    let c11 = if density < constants.ptm_density_threshold_kg_m3 {
        1.0
    } else {
        (-constants.ptm_density_decay_m3_per_kg * (density - constants.ptm_density_threshold_kg_m3))
            .exp()
    };
    let freeze_minus_snow_temp = -snow_temperature_c.min(0.0);
    let destructive_metamorphism = constants.ptm_rate_per_hour
        * c11
        * (-constants.ptm_temperature_decay_per_c * freeze_minus_snow_temp).exp()
        / rate;
    let overburden_compaction = constants.poc_rate_per_hour
        * (-constants.poc_temperature_decay_per_c * freeze_minus_snow_temp).exp()
        * swe
        * (-constants.poc_density_decay * (density / RHO_WATER_KG_M3)).exp()
        / rate;
    state.density_kg_m3 = (density
        + constants.dry_compaction_multiplier
            * multiplier_scale
            * (destructive_metamorphism + overburden_compaction)
            * density)
        .min(constants.dry_compaction_max_density_kg_m3);
}

fn apply_wet_compaction(
    state: &mut CoeBoundDensityState,
    liquid_added_kg_m2: f64,
    constants: PhysicsBulkConstants,
) {
    let density = state.observed_density_kg_m3();
    if density <= 0.0 || density >= constants.wet_compaction_max_density_kg_m3 {
        return;
    }
    if state.mass_kg_m2 <= ZERO_MASS_KG_M2 {
        return;
    }
    let h2o_added_ratio = liquid_added_kg_m2 / state.mass_kg_m2;
    if h2o_added_ratio <= 1.0e-6 {
        return;
    }
    let density_delta = constants.wet_compaction_multiplier
        * (constants.wet_compaction_max_density_kg_m3 - density)
        / (1.0 + constants.wet_compaction_half_saturation_ratio / h2o_added_ratio);
    state.density_kg_m3 = (density + density_delta).min(constants.wet_compaction_max_density_kg_m3);
}

fn update_density_extrema(state: &CoeBoundDensityState, ledger: &mut CoeBoundDensityLedger) {
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

impl CoeBoundDensityState {
    fn depth_m(self) -> f64 {
        if self.mass_kg_m2 <= ZERO_MASS_KG_M2 || self.density_kg_m3 <= 0.0 {
            0.0
        } else {
            self.mass_kg_m2 / self.density_kg_m3
        }
    }

    fn observed_density_kg_m3(self) -> f64 {
        let depth = self.depth_m();
        if depth <= 0.0 {
            0.0
        } else {
            self.mass_kg_m2 / depth
        }
    }
}

fn summary_hourly_count(forcing: &[DailyForcing]) -> usize {
    forcing.iter().map(|day| day.hourly_row_count).sum()
}

fn write_coe_bound_density_csv(
    path: &Path,
    rows: &[CoeBoundDensityDailyRow],
) -> Result<(), SnowbenchError> {
    let mut file = fs::File::create(path).map_err(|source| snowbench_io(path, source))?;
    writeln!(
        file,
        "date,snow_water_m,snow_depth_m,snow_density_kg_m3,coe_snow_water_m,coe_snow_depth_m,coe_snow_density_kg_m3,coe_snowpack_swe_loss_m,coe_routed_melt_m,source"
    )
    .map_err(|source| snowbench_io(path, source))?;
    for row in rows {
        writeln!(
            file,
            "{},{:.12},{:.12},{:.12},{:.12},{:.12},{:.12},{:.12},{:.12},{}",
            row.date,
            row.snow_water_m,
            row.snow_depth_m,
            row.snow_density_kg_m3,
            row.coe_snow_water_m,
            row.coe_snow_depth_m,
            row.coe_snow_density_kg_m3,
            row.coe_snowpack_swe_loss_m,
            row.coe_routed_melt_m,
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
    write_text(path, &(text + "\n"))
}

fn write_markdown(path: &Path, report: &CoeBoundDensityReport) -> Result<(), SnowbenchError> {
    let text = format!(
        "# CoE-Bound Density Snowbench Summary\n\n\
         - Schema: `{}`\n\
         - Model: `{}`\n\
         - CoE boundary model: `{}`\n\
         - Density variant: `{}`\n\
         - Contract: `{}`\n\
         - Runtime coupling: `{}`\n\
         - No site constants: `{}`\n\
         - Days: `{}`\n\
         - Hourly rows: `{}`\n\
         - Total snow input: `{:.6}` kg m^-2\n\
         - Total boundary SWE loss: `{:.6}` m\n\
         - Total boundary routed melt: `{:.6}` m\n\
         - Final SWE: `{:.6}` m\n\
         - Final depth: `{:.6}` m\n\
         - Final density: `{:.6}` kg m^-3\n\
         - Max CoE SWE identity residual: `{:.12}` m\n\
         - Max unbounded SWE residual: `{:.12}` m\n",
        report.schema,
        report.model_id,
        report.coe_boundary_model,
        report.density_variant,
        report.contract,
        report.runtime_coupling,
        report.no_site_constants,
        report.day_count,
        report.hourly_row_count,
        report.summary.total_snow_input_kg_m2,
        report.summary.total_boundary_swe_loss_m,
        report.summary.total_boundary_routed_melt_m,
        report.summary.final_swe_m,
        report.summary.final_depth_m,
        report.summary.final_density_kg_m3,
        report.summary.max_abs_coe_swe_identity_residual_m,
        report.summary.max_abs_unbounded_swe_residual_m,
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
