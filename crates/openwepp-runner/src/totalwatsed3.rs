#![allow(
    clippy::module_name_repetitions,
    clippy::similar_names,
    clippy::struct_field_names,
    clippy::too_many_arguments,
    clippy::too_many_lines,
    clippy::type_complexity
)]

use std::collections::BTreeMap;
use std::fmt;
use std::fs::File;
use std::path::{Path, PathBuf};

use arrow_array::{Array, Float64Array, Int8Array, Int16Array, Int32Array, RecordBatch};
use openwepp_watershed_output::writers::{
    WatershedInterchangeRowSeed, WatershedWriterError, write_totalwatsed3_parquet,
};
use parquet::arrow::arrow_reader::ParquetRecordBatchReaderBuilder;

const SEDIMENT_DENSITY_KG_M3: [f64; 5] = [2_600.0, 2_650.0, 1_800.0, 1_600.0, 2_650.0];

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Totalwatsed3Config {
    pub pass_path: PathBuf,
    pub wat_path: PathBuf,
    pub soil_path: Option<PathBuf>,
    pub element_path: Option<PathBuf>,
    pub output_path: PathBuf,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Totalwatsed3WriteSummary {
    pub output_path: PathBuf,
    pub rows_written: usize,
}

#[derive(Debug)]
pub enum Totalwatsed3Error {
    MissingInput {
        role: &'static str,
        path: PathBuf,
    },
    Open {
        path: PathBuf,
        detail: String,
    },
    Read {
        path: PathBuf,
        detail: String,
    },
    MissingColumn {
        path: PathBuf,
        column: String,
    },
    UnsupportedColumnType {
        path: PathBuf,
        column: String,
    },
    NullValue {
        path: PathBuf,
        column: String,
        row_index: usize,
    },
    InvalidValue {
        path: PathBuf,
        column: String,
        row_index: usize,
        value: f64,
    },
    EmptyWatInput {
        path: PathBuf,
    },
    Write {
        path: PathBuf,
        detail: String,
    },
}

impl Totalwatsed3Error {
    #[must_use]
    pub const fn code(&self) -> &'static str {
        match self {
            Self::MissingInput { .. } => "TW3-E-001",
            Self::Open { .. } => "TW3-E-002",
            Self::Read { .. } => "TW3-E-003",
            Self::MissingColumn { .. } => "TW3-E-004",
            Self::UnsupportedColumnType { .. } => "TW3-E-005",
            Self::NullValue { .. } => "TW3-E-006",
            Self::InvalidValue { .. } => "TW3-E-007",
            Self::EmptyWatInput { .. } => "TW3-E-008",
            Self::Write { .. } => "TW3-E-009",
        }
    }
}

impl fmt::Display for Totalwatsed3Error {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::MissingInput { role, path } => write!(
                formatter,
                "{} missing required {role} input {}",
                self.code(),
                path.display()
            ),
            Self::Open { path, detail } => write!(
                formatter,
                "{} failed opening parquet {}: {detail}",
                self.code(),
                path.display()
            ),
            Self::Read { path, detail } => write!(
                formatter,
                "{} failed reading parquet {}: {detail}",
                self.code(),
                path.display()
            ),
            Self::MissingColumn { path, column } => write!(
                formatter,
                "{} parquet {} is missing required column {column}",
                self.code(),
                path.display()
            ),
            Self::UnsupportedColumnType { path, column } => write!(
                formatter,
                "{} parquet {} has unsupported type for column {column}",
                self.code(),
                path.display()
            ),
            Self::NullValue {
                path,
                column,
                row_index,
            } => write!(
                formatter,
                "{} parquet {} has null value in column {column} at row {row_index}",
                self.code(),
                path.display()
            ),
            Self::InvalidValue {
                path,
                column,
                row_index,
                value,
            } => write!(
                formatter,
                "{} parquet {} has invalid value {value} in column {column} at row {row_index}",
                self.code(),
                path.display()
            ),
            Self::EmptyWatInput { path } => write!(
                formatter,
                "{} WAT parquet {} has no rows to aggregate",
                self.code(),
                path.display()
            ),
            Self::Write { path, detail } => write!(
                formatter,
                "{} failed writing totalwatsed3 parquet {}: {detail}",
                self.code(),
                path.display()
            ),
        }
    }
}

impl std::error::Error for Totalwatsed3Error {}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
struct DayKey {
    year: i16,
    julian: i16,
    sim_day_index: i32,
    month: i8,
    day_of_month: i8,
    water_year: i16,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
struct DateOfeKey {
    wepp_id: i32,
    ofe_id: i16,
    year: i16,
    julian: i16,
    month: i8,
    day_of_month: i8,
    water_year: i16,
}

#[derive(Debug, Clone, Copy)]
struct AreaLookupEntry {
    day_key: DayKey,
    area_m2: f64,
}

#[derive(Debug, Clone, Copy, Default)]
struct PassAccumulator {
    runvol_m3: f64,
    sbrunv_m3: f64,
    tdet_kg: f64,
    tdep_kg: f64,
    sediment_class_deposition_kg: [f64; 5],
}

impl PassAccumulator {
    fn add(&mut self, values: PassValues) {
        self.runvol_m3 += values.runvol_m3;
        self.sbrunv_m3 += values.sbrunv_m3;
        self.tdet_kg += values.tdet_kg;
        self.tdep_kg += values.tdep_kg;
        for (target, contribution) in self
            .sediment_class_deposition_kg
            .iter_mut()
            .zip(values.sediment_class_deposition_kg)
        {
            *target += contribution;
        }
    }

    fn sediment_delivery_kg(self) -> f64 {
        self.sediment_class_deposition_kg.iter().sum()
    }

    fn sediment_volume_concentration(self) -> f64 {
        if self.runvol_m3 <= 0.0 {
            return 0.0;
        }
        let solids_volume_m3: f64 = self
            .sediment_class_deposition_kg
            .iter()
            .zip(SEDIMENT_DENSITY_KG_M3)
            .map(|(mass_kg, density_kg_m3)| mass_kg / density_kg_m3)
            .sum();
        solids_volume_m3 / self.runvol_m3
    }
}

#[derive(Debug, Clone, Copy)]
struct PassValues {
    runvol_m3: f64,
    sbrunv_m3: f64,
    tdet_kg: f64,
    tdep_kg: f64,
    sediment_class_deposition_kg: [f64; 5],
}

#[derive(Debug, Clone, Copy, Default)]
struct DailyWatAccumulator {
    area_m2: f64,
    precipitation_mm_m2: f64,
    rain_melt_mm_m2: f64,
    q_diagnostic_mm_m2: f64,
    deep_percolation_mm_m2: f64,
    lateral_flow_mm_m2: f64,
    qofe_mm_m2: f64,
    transpiration_mm_m2: f64,
    evaporation_soil_mm_m2: f64,
    evaporation_residue_mm_m2: f64,
    upstream_q_mm_m2: f64,
    subsurface_runon_mm_m2: f64,
    total_soil_water_mm_m2: f64,
    soil_water_total_mm_m2: f64,
    profile_depth_mm_m2: f64,
    profile_porosity_cap_mm_m2: f64,
    profile_fc_store_mm_m2: f64,
    profile_wp_store_mm_m2: f64,
    interception_mm_m2: f64,
    interception_storage_mm_m2: f64,
    frozen_water_mm_m2: f64,
    snow_water_mm_m2: f64,
    tile_mm_m2: f64,
    irrigation_mm_m2: f64,
}

impl DailyWatAccumulator {
    fn add_weighted(&mut self, area_m2: f64, values: WatValues, include_lateral_flow: bool) {
        self.area_m2 += area_m2;
        self.precipitation_mm_m2 += values.precipitation_mm * area_m2;
        self.rain_melt_mm_m2 += values.rain_melt_mm * area_m2;
        self.q_diagnostic_mm_m2 += values.q_diagnostic_mm * area_m2;
        self.deep_percolation_mm_m2 += values.deep_percolation_mm * area_m2;
        if include_lateral_flow {
            self.lateral_flow_mm_m2 += values.lateral_flow_mm * area_m2;
        }
        self.qofe_mm_m2 += values.qofe_mm * area_m2;
        self.transpiration_mm_m2 += values.transpiration_mm * area_m2;
        self.evaporation_soil_mm_m2 += values.evaporation_soil_mm * area_m2;
        self.evaporation_residue_mm_m2 += values.evaporation_residue_mm * area_m2;
        self.upstream_q_mm_m2 += values.upstream_q_mm * area_m2;
        self.subsurface_runon_mm_m2 += values.subsurface_runon_mm * area_m2;
        self.total_soil_water_mm_m2 += values.total_soil_water_mm * area_m2;
        self.soil_water_total_mm_m2 += values.soil_water_total_mm * area_m2;
        self.profile_depth_mm_m2 += values.profile_depth_mm * area_m2;
        self.profile_porosity_cap_mm_m2 += values.profile_porosity_cap_mm * area_m2;
        self.profile_fc_store_mm_m2 += values.profile_fc_store_mm * area_m2;
        self.profile_wp_store_mm_m2 += values.profile_wp_store_mm * area_m2;
        self.interception_mm_m2 += values.interception_mm * area_m2;
        self.interception_storage_mm_m2 += values.interception_storage_mm * area_m2;
        self.frozen_water_mm_m2 += values.frozen_water_mm * area_m2;
        self.snow_water_mm_m2 += values.snow_water_mm * area_m2;
        self.tile_mm_m2 += values.tile_mm * area_m2;
        self.irrigation_mm_m2 += values.irrigation_mm * area_m2;
    }
}

#[derive(Debug, Clone, Copy)]
struct WatValues {
    precipitation_mm: f64,
    rain_melt_mm: f64,
    q_diagnostic_mm: f64,
    deep_percolation_mm: f64,
    lateral_flow_mm: f64,
    qofe_mm: f64,
    transpiration_mm: f64,
    evaporation_soil_mm: f64,
    evaporation_residue_mm: f64,
    upstream_q_mm: f64,
    subsurface_runon_mm: f64,
    total_soil_water_mm: f64,
    soil_water_total_mm: f64,
    profile_depth_mm: f64,
    profile_porosity_cap_mm: f64,
    profile_fc_store_mm: f64,
    profile_wp_store_mm: f64,
    interception_mm: f64,
    interception_storage_mm: f64,
    frozen_water_mm: f64,
    snow_water_mm: f64,
    tile_mm: f64,
    irrigation_mm: f64,
}

#[derive(Debug, Clone, Copy)]
struct WatInputRow {
    key: DayKey,
    wepp_id: i32,
    ofe_id: Option<i16>,
    area_m2: f64,
    values: WatValues,
}

#[derive(Debug, Clone, Copy, Default)]
struct WeightedOptionalAccumulator {
    weighted_value_m2: f64,
    area_m2: f64,
}

impl WeightedOptionalAccumulator {
    fn add(&mut self, value: f64, area_m2: f64) {
        self.weighted_value_m2 += value * area_m2;
        self.area_m2 += area_m2;
    }

    fn depth_or_zero(self) -> f64 {
        if self.area_m2 > 0.0 {
            self.weighted_value_m2 / self.area_m2
        } else {
            0.0
        }
    }
}

#[derive(Debug, Clone, Default)]
struct OptionalAggregates {
    tsmf_by_day: BTreeMap<DayKey, WeightedOptionalAccumulator>,
    qrain_by_day: BTreeMap<DayKey, WeightedOptionalAccumulator>,
    qsnow_by_day: BTreeMap<DayKey, WeightedOptionalAccumulator>,
}

pub fn write_totalwatsed3(
    config: &Totalwatsed3Config,
) -> Result<Totalwatsed3WriteSummary, Totalwatsed3Error> {
    require_file("PASS", &config.pass_path)?;
    require_file("WAT", &config.wat_path)?;
    if let Some(path) = &config.soil_path {
        require_file("soil", path)?;
    }
    if let Some(path) = &config.element_path {
        require_file("element", path)?;
    }

    let pass_by_day = read_pass_daily(&config.pass_path)?;
    let (wat_by_day, area_lookup) = read_wat_daily(&config.wat_path)?;
    if wat_by_day.is_empty() {
        return Err(Totalwatsed3Error::EmptyWatInput {
            path: config.wat_path.clone(),
        });
    }

    let mut optional_aggregates = OptionalAggregates::default();
    if let Some(path) = &config.soil_path {
        read_soil_tsmf(path, &area_lookup, &mut optional_aggregates)?;
    }
    if let Some(path) = &config.element_path {
        read_element_partitions(path, &area_lookup, &mut optional_aggregates)?;
    }

    let rows = build_rows(&wat_by_day, &pass_by_day, &optional_aggregates)?;
    write_totalwatsed3_parquet(&config.output_path, &rows)
        .map_err(|error| writer_error(&config.output_path, &error))?;

    Ok(Totalwatsed3WriteSummary {
        output_path: config.output_path.clone(),
        rows_written: rows.len(),
    })
}

fn require_file(role: &'static str, path: &Path) -> Result<(), Totalwatsed3Error> {
    if path.is_file() {
        Ok(())
    } else {
        Err(Totalwatsed3Error::MissingInput {
            role,
            path: path.to_path_buf(),
        })
    }
}

fn writer_error(path: &Path, error: &WatershedWriterError) -> Totalwatsed3Error {
    Totalwatsed3Error::Write {
        path: path.to_path_buf(),
        detail: error.to_string(),
    }
}

fn build_rows(
    wat_by_day: &BTreeMap<DayKey, DailyWatAccumulator>,
    pass_by_day: &BTreeMap<DayKey, PassAccumulator>,
    optional_aggregates: &OptionalAggregates,
) -> Result<Vec<WatershedInterchangeRowSeed>, Totalwatsed3Error> {
    let mut rows = Vec::with_capacity(wat_by_day.len());
    for (key, wat) in wat_by_day {
        if !wat.area_m2.is_finite() || wat.area_m2 <= 0.0 {
            return Err(Totalwatsed3Error::InvalidValue {
                path: PathBuf::from("<aggregated WAT>"),
                column: "Area".to_string(),
                row_index: usize::try_from(key.sim_day_index).unwrap_or(usize::MAX),
                value: wat.area_m2,
            });
        }
        let pass = pass_by_day.get(key).copied().unwrap_or_default();
        let area = wat.area_m2;
        let sediment_delivery_kg = pass.sediment_delivery_kg();
        rows.push(WatershedInterchangeRowSeed {
            year: key.year,
            simulation_year: key.year,
            sim_day_index: key.sim_day_index,
            julian: key.julian,
            month: key.month,
            day_of_month: key.day_of_month,
            water_year: key.water_year,
            runoff_volume_m3: pass.runvol_m3,
            subsurface_runoff_volume_m3: pass.sbrunv_m3,
            total_detachment_kg: pass.tdet_kg,
            total_deposition_kg: pass.tdep_kg,
            sediment_class_deposition_kg: pass.sediment_class_deposition_kg,
            sediment_yield_kg: sediment_delivery_kg,
            sediment_volume_concentration_m3_m3: pass.sediment_volume_concentration(),
            channel_outflow_m3: pass.runvol_m3,
            area_m2: area,
            precipitation_mm: wat.precipitation_mm_m2 / area,
            rain_melt_mm: wat.rain_melt_mm_m2 / area,
            runoff_mm: pass.runvol_m3 / area * 1_000.0,
            q_diagnostic_mm: Some(wat.q_diagnostic_mm_m2 / area),
            deep_percolation_mm: wat.deep_percolation_mm_m2 / area,
            lateral_flow_mm: wat.lateral_flow_mm_m2 / area,
            qofe_mm: wat.qofe_mm_m2 / area,
            transpiration_mm: wat.transpiration_mm_m2 / area,
            evaporation_soil_mm: wat.evaporation_soil_mm_m2 / area,
            evaporation_residue_mm: wat.evaporation_residue_mm_m2 / area,
            upstream_q_mm: wat.upstream_q_mm_m2 / area,
            subsurface_runon_mm: wat.subsurface_runon_mm_m2 / area,
            total_soil_water_mm: wat.total_soil_water_mm_m2 / area,
            soil_water_total_mm: wat.soil_water_total_mm_m2 / area,
            profile_depth_mm: wat.profile_depth_mm_m2 / area,
            profile_porosity_cap_mm: wat.profile_porosity_cap_mm_m2 / area,
            profile_fc_store_mm: wat.profile_fc_store_mm_m2 / area,
            profile_wp_store_mm: wat.profile_wp_store_mm_m2 / area,
            interception_mm: wat.interception_mm_m2 / area,
            interception_storage_mm: wat.interception_storage_mm_m2 / area,
            frozen_water_mm: wat.frozen_water_mm_m2 / area,
            snow_water_mm: wat.snow_water_mm_m2 / area,
            tile_mm: wat.tile_mm_m2 / area,
            irrigation_mm: wat.irrigation_mm_m2 / area,
            tsmf_fraction: optional_aggregates
                .tsmf_by_day
                .get(key)
                .copied()
                .unwrap_or_default()
                .depth_or_zero(),
            qrain_mm: optional_aggregates
                .qrain_by_day
                .get(key)
                .copied()
                .unwrap_or_default()
                .depth_or_zero(),
            qsnow_mm: optional_aggregates
                .qsnow_by_day
                .get(key)
                .copied()
                .unwrap_or_default()
                .depth_or_zero(),
            ..WatershedInterchangeRowSeed::default()
        });
    }
    Ok(rows)
}

fn read_pass_daily(path: &Path) -> Result<BTreeMap<DayKey, PassAccumulator>, Totalwatsed3Error> {
    let mut daily = BTreeMap::<DayKey, PassAccumulator>::new();
    for_batch(path, |batch, row_offset| {
        let wepp_ids = int32_column(path, batch, "wepp_id")?;
        let years = int16_column(path, batch, "year")?;
        let sim_day_indexes = int32_column_any(path, batch, &["sim_day_index", "day"])?;
        let julians = int16_column(path, batch, "julian")?;
        let months = int8_column(path, batch, "month")?;
        let day_of_months = int8_column(path, batch, "day_of_month")?;
        let water_years = int16_column(path, batch, "water_year")?;
        let runvol = f64_column(path, batch, "runvol")?;
        let sbrunv = f64_column(path, batch, "sbrunv")?;
        let tdet = f64_column(path, batch, "tdet")?;
        let tdep = f64_column(path, batch, "tdep")?;
        let sedcon = [
            f64_column(path, batch, "sedcon_1")?,
            f64_column(path, batch, "sedcon_2")?,
            f64_column(path, batch, "sedcon_3")?,
            f64_column(path, batch, "sedcon_4")?,
            f64_column(path, batch, "sedcon_5")?,
        ];

        for row in 0..batch.num_rows() {
            let row_index = row_offset + row;
            let _wepp_id = int32_value(path, "wepp_id", wepp_ids, row, row_index)?;
            let key = day_key_from_columns(
                path,
                years,
                sim_day_indexes,
                julians,
                months,
                day_of_months,
                water_years,
                row,
                row_index,
            )?;
            let runvol_m3 = nonnegative_f64_value(path, "runvol", runvol, row, row_index)?;
            let mut sediment_class_deposition_kg = [0.0_f64; 5];
            for (index, column) in sedcon.iter().enumerate() {
                let concentration = nonnegative_f64_value(path, "sedcon", column, row, row_index)?;
                sediment_class_deposition_kg[index] = concentration * runvol_m3;
            }
            daily.entry(key).or_default().add(PassValues {
                runvol_m3,
                sbrunv_m3: nonnegative_f64_value(path, "sbrunv", sbrunv, row, row_index)?,
                tdet_kg: f64_value(path, "tdet", tdet, row, row_index)?,
                tdep_kg: f64_value(path, "tdep", tdep, row, row_index)?,
                sediment_class_deposition_kg,
            });
        }
        Ok(())
    })?;
    Ok(daily)
}

fn read_wat_daily(
    path: &Path,
) -> Result<
    (
        BTreeMap<DayKey, DailyWatAccumulator>,
        BTreeMap<DateOfeKey, AreaLookupEntry>,
    ),
    Totalwatsed3Error,
> {
    let mut rows = Vec::new();
    let mut area_lookup = BTreeMap::<DateOfeKey, AreaLookupEntry>::new();
    for_batch(path, |batch, row_offset| {
        read_wat_batch(path, batch, row_offset, &mut rows, &mut area_lookup)
    })?;

    let mut outlet_ofe_by_day = BTreeMap::<(DayKey, i32), i16>::new();
    for row in &rows {
        if let Some(ofe_id) = row.ofe_id {
            outlet_ofe_by_day
                .entry((row.key, row.wepp_id))
                .and_modify(|current| *current = (*current).max(ofe_id))
                .or_insert(ofe_id);
        }
    }

    let mut daily = BTreeMap::<DayKey, DailyWatAccumulator>::new();
    for row in rows {
        let include_lateral_flow = row.ofe_id.is_none_or(|ofe_id| {
            outlet_ofe_by_day
                .get(&(row.key, row.wepp_id))
                .is_none_or(|outlet_ofe_id| ofe_id == *outlet_ofe_id)
        });
        daily.entry(row.key).or_default().add_weighted(
            row.area_m2,
            row.values,
            include_lateral_flow,
        );
    }

    Ok((daily, area_lookup))
}

fn read_wat_batch(
    path: &Path,
    batch: &RecordBatch,
    row_offset: usize,
    rows: &mut Vec<WatInputRow>,
    area_lookup: &mut BTreeMap<DateOfeKey, AreaLookupEntry>,
) -> Result<(), Totalwatsed3Error> {
    let wepp_ids = int32_column(path, batch, "wepp_id")?;
    let ofe_ids = optional_int16_column_any(path, batch, &["ofe_id", "OFE"])?;
    let years = int16_column(path, batch, "year")?;
    let sim_day_indexes = int32_column_any(path, batch, &["sim_day_index", "day"])?;
    let julians = int16_column(path, batch, "julian")?;
    let months = int8_column(path, batch, "month")?;
    let day_of_months = int8_column(path, batch, "day_of_month")?;
    let water_years = int16_column(path, batch, "water_year")?;
    let areas = f64_column(path, batch, "Area")?;
    let precipitation = f64_column(path, batch, "P")?;
    let rain_melt = f64_column(path, batch, "RM")?;
    let q_diagnostic = f64_column(path, batch, "Q")?;
    let deep_percolation = f64_column(path, batch, "Dp")?;
    let lateral_flow = f64_column(path, batch, "latqcc")?;
    let qofe = f64_column(path, batch, "QOFE")?;
    let transpiration = f64_column(path, batch, "Ep")?;
    let evaporation_soil = f64_column(path, batch, "Es")?;
    let evaporation_residue = f64_column(path, batch, "Er")?;
    let upstream_q = f64_column(path, batch, "UpStrmQ")?;
    let subsurface_runon = f64_column(path, batch, "SubRIn")?;
    let total_soil_water = f64_column_any(path, batch, &["Total-Soil Water", "Total-Soil"])?;
    let soil_water_total = optional_f64_column(path, batch, "SoilWaterTotal")?;
    let profile_depth = optional_f64_column(path, batch, "ProfileDepth")?;
    let profile_porosity_cap = optional_f64_column(path, batch, "ProfilePorosityCap")?;
    let profile_fc_store = optional_f64_column(path, batch, "ProfileFCStore")?;
    let profile_wp_store = optional_f64_column(path, batch, "ProfileWPStore")?;
    let interception = optional_f64_column(path, batch, "Interception")?;
    let interception_storage = optional_f64_column(path, batch, "InterceptionStorage")?;
    let frozen_water = f64_column(path, batch, "frozwt")?;
    let snow_water = f64_column(path, batch, "Snow-Water")?;
    let tile = optional_f64_column(path, batch, "Tile")?;
    let irrigation = optional_f64_column(path, batch, "Irr")?;

    for row in 0..batch.num_rows() {
        let row_index = row_offset + row;
        let wepp_id = int32_value(path, "wepp_id", wepp_ids, row, row_index)?;
        let ofe_id = optional_int16_value(path, "ofe_id|OFE", ofe_ids, row, row_index)?;
        let area_m2 = nonnegative_f64_value(path, "Area", areas, row, row_index)?;
        if area_m2 <= 0.0 {
            return Err(Totalwatsed3Error::InvalidValue {
                path: path.to_path_buf(),
                column: "Area".to_string(),
                row_index,
                value: area_m2,
            });
        }
        let key = day_key_from_columns(
            path,
            years,
            sim_day_indexes,
            julians,
            months,
            day_of_months,
            water_years,
            row,
            row_index,
        )?;
        if let Some(ofe_id) = ofe_id {
            area_lookup.insert(
                DateOfeKey {
                    wepp_id,
                    ofe_id,
                    year: key.year,
                    julian: key.julian,
                    month: key.month,
                    day_of_month: key.day_of_month,
                    water_year: key.water_year,
                },
                AreaLookupEntry {
                    day_key: key,
                    area_m2,
                },
            );
        }
        let total_soil_water_mm =
            f64_value(path, "Total-Soil Water", total_soil_water, row, row_index)?;
        rows.push(WatInputRow {
            key,
            wepp_id,
            ofe_id,
            area_m2,
            values: WatValues {
                precipitation_mm: f64_value(path, "P", precipitation, row, row_index)?,
                rain_melt_mm: f64_value(path, "RM", rain_melt, row, row_index)?,
                q_diagnostic_mm: f64_value(path, "Q", q_diagnostic, row, row_index)?,
                deep_percolation_mm: f64_value(path, "Dp", deep_percolation, row, row_index)?,
                lateral_flow_mm: f64_value(path, "latqcc", lateral_flow, row, row_index)?,
                qofe_mm: f64_value(path, "QOFE", qofe, row, row_index)?,
                transpiration_mm: f64_value(path, "Ep", transpiration, row, row_index)?,
                evaporation_soil_mm: f64_value(path, "Es", evaporation_soil, row, row_index)?,
                evaporation_residue_mm: f64_value(path, "Er", evaporation_residue, row, row_index)?,
                upstream_q_mm: f64_value(path, "UpStrmQ", upstream_q, row, row_index)?,
                subsurface_runon_mm: f64_value(path, "SubRIn", subsurface_runon, row, row_index)?,
                total_soil_water_mm,
                soil_water_total_mm: optional_f64_value(
                    path,
                    "SoilWaterTotal",
                    soil_water_total,
                    row,
                    row_index,
                )?
                .unwrap_or(total_soil_water_mm),
                profile_depth_mm: optional_f64_value(
                    path,
                    "ProfileDepth",
                    profile_depth,
                    row,
                    row_index,
                )?
                .unwrap_or(0.0),
                profile_porosity_cap_mm: optional_f64_value(
                    path,
                    "ProfilePorosityCap",
                    profile_porosity_cap,
                    row,
                    row_index,
                )?
                .unwrap_or(0.0),
                profile_fc_store_mm: optional_f64_value(
                    path,
                    "ProfileFCStore",
                    profile_fc_store,
                    row,
                    row_index,
                )?
                .unwrap_or(0.0),
                profile_wp_store_mm: optional_f64_value(
                    path,
                    "ProfileWPStore",
                    profile_wp_store,
                    row,
                    row_index,
                )?
                .unwrap_or(0.0),
                interception_mm: optional_f64_value(
                    path,
                    "Interception",
                    interception,
                    row,
                    row_index,
                )?
                .unwrap_or(0.0),
                interception_storage_mm: optional_f64_value(
                    path,
                    "InterceptionStorage",
                    interception_storage,
                    row,
                    row_index,
                )?
                .unwrap_or(0.0),
                frozen_water_mm: f64_value(path, "frozwt", frozen_water, row, row_index)?,
                snow_water_mm: f64_value(path, "Snow-Water", snow_water, row, row_index)?,
                tile_mm: optional_f64_value(path, "Tile", tile, row, row_index)?.unwrap_or(0.0),
                irrigation_mm: optional_f64_value(path, "Irr", irrigation, row, row_index)?
                    .unwrap_or(0.0),
            },
        });
    }
    Ok(())
}

fn read_soil_tsmf(
    path: &Path,
    area_lookup: &BTreeMap<DateOfeKey, AreaLookupEntry>,
    optional_aggregates: &mut OptionalAggregates,
) -> Result<(), Totalwatsed3Error> {
    for_batch(path, |batch, row_offset| {
        let wepp_ids = int32_column(path, batch, "wepp_id")?;
        let ofe_ids = int16_column_any(path, batch, &["ofe_id", "OFE"])?;
        let years = int16_column(path, batch, "year")?;
        let julians = int16_column(path, batch, "julian")?;
        let months = int8_column(path, batch, "month")?;
        let day_of_months = int8_column(path, batch, "day_of_month")?;
        let water_years = int16_column(path, batch, "water_year")?;
        let tsmf = f64_column(path, batch, "TSMF")?;
        for row in 0..batch.num_rows() {
            let row_index = row_offset + row;
            let date_ofe_key = date_ofe_key_from_columns(
                path,
                wepp_ids,
                ofe_ids,
                years,
                julians,
                months,
                day_of_months,
                water_years,
                row,
                row_index,
            )?;
            if let Some(area_entry) = area_lookup.get(&date_ofe_key) {
                optional_aggregates
                    .tsmf_by_day
                    .entry(area_entry.day_key)
                    .or_default()
                    .add(
                        f64_value(path, "TSMF", tsmf, row, row_index)?,
                        area_entry.area_m2,
                    );
            }
        }
        Ok(())
    })
}

fn read_element_partitions(
    path: &Path,
    area_lookup: &BTreeMap<DateOfeKey, AreaLookupEntry>,
    optional_aggregates: &mut OptionalAggregates,
) -> Result<(), Totalwatsed3Error> {
    for_batch(path, |batch, row_offset| {
        let wepp_ids = int32_column(path, batch, "wepp_id")?;
        let ofe_ids = int16_column_any(path, batch, &["ofe_id", "OFE"])?;
        let years = int16_column(path, batch, "year")?;
        let julians = int16_column(path, batch, "julian")?;
        let months = int8_column(path, batch, "month")?;
        let day_of_months = int8_column(path, batch, "day_of_month")?;
        let water_years = int16_column(path, batch, "water_year")?;
        let qrain = optional_f64_column(path, batch, "QRain")?;
        let qsnow = optional_f64_column(path, batch, "QSnow")?;
        for row in 0..batch.num_rows() {
            let row_index = row_offset + row;
            let date_ofe_key = date_ofe_key_from_columns(
                path,
                wepp_ids,
                ofe_ids,
                years,
                julians,
                months,
                day_of_months,
                water_years,
                row,
                row_index,
            )?;
            if let Some(area_entry) = area_lookup.get(&date_ofe_key) {
                if let Some(value) = optional_f64_value(path, "QRain", qrain, row, row_index)? {
                    optional_aggregates
                        .qrain_by_day
                        .entry(area_entry.day_key)
                        .or_default()
                        .add(value, area_entry.area_m2);
                }
                if let Some(value) = optional_f64_value(path, "QSnow", qsnow, row, row_index)? {
                    optional_aggregates
                        .qsnow_by_day
                        .entry(area_entry.day_key)
                        .or_default()
                        .add(value, area_entry.area_m2);
                }
            }
        }
        Ok(())
    })
}

fn for_batch<F>(path: &Path, mut apply: F) -> Result<(), Totalwatsed3Error>
where
    F: FnMut(&RecordBatch, usize) -> Result<(), Totalwatsed3Error>,
{
    let file = File::open(path).map_err(|error| Totalwatsed3Error::Open {
        path: path.to_path_buf(),
        detail: error.to_string(),
    })?;
    let builder = ParquetRecordBatchReaderBuilder::try_new(file).map_err(|error| {
        Totalwatsed3Error::Read {
            path: path.to_path_buf(),
            detail: error.to_string(),
        }
    })?;
    let reader = builder.build().map_err(|error| Totalwatsed3Error::Read {
        path: path.to_path_buf(),
        detail: error.to_string(),
    })?;
    let mut row_offset = 0_usize;
    for batch_result in reader {
        let batch = batch_result.map_err(|error| Totalwatsed3Error::Read {
            path: path.to_path_buf(),
            detail: error.to_string(),
        })?;
        apply(&batch, row_offset)?;
        row_offset += batch.num_rows();
    }
    Ok(())
}

fn day_key_from_columns(
    path: &Path,
    years: &Int16Array,
    sim_day_indexes: &Int32Array,
    julians: &Int16Array,
    months: &Int8Array,
    day_of_months: &Int8Array,
    water_years: &Int16Array,
    row: usize,
    row_index: usize,
) -> Result<DayKey, Totalwatsed3Error> {
    Ok(DayKey {
        year: int16_value(path, "year", years, row, row_index)?,
        julian: int16_value(path, "julian", julians, row, row_index)?,
        sim_day_index: int32_value(path, "sim_day_index", sim_day_indexes, row, row_index)?,
        month: int8_value(path, "month", months, row, row_index)?,
        day_of_month: int8_value(path, "day_of_month", day_of_months, row, row_index)?,
        water_year: int16_value(path, "water_year", water_years, row, row_index)?,
    })
}

fn date_ofe_key_from_columns(
    path: &Path,
    wepp_ids: &Int32Array,
    ofe_ids: &Int16Array,
    years: &Int16Array,
    julians: &Int16Array,
    months: &Int8Array,
    day_of_months: &Int8Array,
    water_years: &Int16Array,
    row: usize,
    row_index: usize,
) -> Result<DateOfeKey, Totalwatsed3Error> {
    Ok(DateOfeKey {
        wepp_id: int32_value(path, "wepp_id", wepp_ids, row, row_index)?,
        ofe_id: int16_value(path, "ofe_id|OFE", ofe_ids, row, row_index)?,
        year: int16_value(path, "year", years, row, row_index)?,
        julian: int16_value(path, "julian", julians, row, row_index)?,
        month: int8_value(path, "month", months, row, row_index)?,
        day_of_month: int8_value(path, "day_of_month", day_of_months, row, row_index)?,
        water_year: int16_value(path, "water_year", water_years, row, row_index)?,
    })
}

fn int8_column<'a>(
    path: &Path,
    batch: &'a RecordBatch,
    name: &str,
) -> Result<&'a Int8Array, Totalwatsed3Error> {
    column(path, batch, name)
}

fn int16_column<'a>(
    path: &Path,
    batch: &'a RecordBatch,
    name: &str,
) -> Result<&'a Int16Array, Totalwatsed3Error> {
    column(path, batch, name)
}

fn int16_column_any<'a>(
    path: &Path,
    batch: &'a RecordBatch,
    names: &[&str],
) -> Result<&'a Int16Array, Totalwatsed3Error> {
    for name in names {
        match optional_int16_column(path, batch, name) {
            Ok(Some(column)) => return Ok(column),
            Ok(None) => {}
            Err(error) => return Err(error),
        }
    }
    Err(Totalwatsed3Error::MissingColumn {
        path: path.to_path_buf(),
        column: names.join("|"),
    })
}

fn int32_column<'a>(
    path: &Path,
    batch: &'a RecordBatch,
    name: &str,
) -> Result<&'a Int32Array, Totalwatsed3Error> {
    column(path, batch, name)
}

fn int32_column_any<'a>(
    path: &Path,
    batch: &'a RecordBatch,
    names: &[&str],
) -> Result<&'a Int32Array, Totalwatsed3Error> {
    for name in names {
        match optional_int32_column(path, batch, name) {
            Ok(Some(column)) => return Ok(column),
            Ok(None) => {}
            Err(error) => return Err(error),
        }
    }
    Err(Totalwatsed3Error::MissingColumn {
        path: path.to_path_buf(),
        column: names.join("|"),
    })
}

fn f64_column<'a>(
    path: &Path,
    batch: &'a RecordBatch,
    name: &str,
) -> Result<&'a Float64Array, Totalwatsed3Error> {
    column(path, batch, name)
}

fn f64_column_any<'a>(
    path: &Path,
    batch: &'a RecordBatch,
    names: &[&str],
) -> Result<&'a Float64Array, Totalwatsed3Error> {
    for name in names {
        match optional_f64_column(path, batch, name) {
            Ok(Some(column)) => return Ok(column),
            Ok(None) => {}
            Err(error) => return Err(error),
        }
    }
    Err(Totalwatsed3Error::MissingColumn {
        path: path.to_path_buf(),
        column: names.join("|"),
    })
}

fn optional_int16_column_any<'a>(
    path: &Path,
    batch: &'a RecordBatch,
    names: &[&str],
) -> Result<Option<&'a Int16Array>, Totalwatsed3Error> {
    for name in names {
        match optional_int16_column(path, batch, name) {
            Ok(Some(column)) => return Ok(Some(column)),
            Ok(None) => {}
            Err(error) => return Err(error),
        }
    }
    Ok(None)
}

fn optional_int16_column<'a>(
    path: &Path,
    batch: &'a RecordBatch,
    name: &str,
) -> Result<Option<&'a Int16Array>, Totalwatsed3Error> {
    let schema = batch.schema();
    let Ok(index) = schema.index_of(name) else {
        return Ok(None);
    };
    batch
        .column(index)
        .as_any()
        .downcast_ref::<Int16Array>()
        .map(Some)
        .ok_or_else(|| Totalwatsed3Error::UnsupportedColumnType {
            path: path.to_path_buf(),
            column: name.to_string(),
        })
}

fn optional_int32_column<'a>(
    path: &Path,
    batch: &'a RecordBatch,
    name: &str,
) -> Result<Option<&'a Int32Array>, Totalwatsed3Error> {
    let schema = batch.schema();
    let Ok(index) = schema.index_of(name) else {
        return Ok(None);
    };
    batch
        .column(index)
        .as_any()
        .downcast_ref::<Int32Array>()
        .map(Some)
        .ok_or_else(|| Totalwatsed3Error::UnsupportedColumnType {
            path: path.to_path_buf(),
            column: name.to_string(),
        })
}

fn optional_f64_column<'a>(
    path: &Path,
    batch: &'a RecordBatch,
    name: &str,
) -> Result<Option<&'a Float64Array>, Totalwatsed3Error> {
    let schema = batch.schema();
    let Ok(index) = schema.index_of(name) else {
        return Ok(None);
    };
    batch
        .column(index)
        .as_any()
        .downcast_ref::<Float64Array>()
        .map(Some)
        .ok_or_else(|| Totalwatsed3Error::UnsupportedColumnType {
            path: path.to_path_buf(),
            column: name.to_string(),
        })
}

fn column<'a, T: 'static>(
    path: &Path,
    batch: &'a RecordBatch,
    name: &str,
) -> Result<&'a T, Totalwatsed3Error> {
    let schema = batch.schema();
    let index = schema
        .index_of(name)
        .map_err(|_| Totalwatsed3Error::MissingColumn {
            path: path.to_path_buf(),
            column: name.to_string(),
        })?;
    batch
        .column(index)
        .as_any()
        .downcast_ref::<T>()
        .ok_or_else(|| Totalwatsed3Error::UnsupportedColumnType {
            path: path.to_path_buf(),
            column: name.to_string(),
        })
}

fn int8_value(
    path: &Path,
    column_name: &str,
    array: &Int8Array,
    row: usize,
    row_index: usize,
) -> Result<i8, Totalwatsed3Error> {
    if array.is_null(row) {
        return Err(null_value(path, column_name, row_index));
    }
    Ok(array.value(row))
}

fn int16_value(
    path: &Path,
    column_name: &str,
    array: &Int16Array,
    row: usize,
    row_index: usize,
) -> Result<i16, Totalwatsed3Error> {
    if array.is_null(row) {
        return Err(null_value(path, column_name, row_index));
    }
    Ok(array.value(row))
}

fn int32_value(
    path: &Path,
    column_name: &str,
    array: &Int32Array,
    row: usize,
    row_index: usize,
) -> Result<i32, Totalwatsed3Error> {
    if array.is_null(row) {
        return Err(null_value(path, column_name, row_index));
    }
    Ok(array.value(row))
}

fn optional_int16_value(
    path: &Path,
    column_name: &str,
    array: Option<&Int16Array>,
    row: usize,
    row_index: usize,
) -> Result<Option<i16>, Totalwatsed3Error> {
    array.map_or(Ok(None), |array| {
        int16_value(path, column_name, array, row, row_index).map(Some)
    })
}

fn f64_value(
    path: &Path,
    column_name: &str,
    array: &Float64Array,
    row: usize,
    row_index: usize,
) -> Result<f64, Totalwatsed3Error> {
    if array.is_null(row) {
        return Err(null_value(path, column_name, row_index));
    }
    let value = array.value(row);
    if !value.is_finite() {
        return Err(Totalwatsed3Error::InvalidValue {
            path: path.to_path_buf(),
            column: column_name.to_string(),
            row_index,
            value,
        });
    }
    Ok(value)
}

fn nonnegative_f64_value(
    path: &Path,
    column_name: &str,
    array: &Float64Array,
    row: usize,
    row_index: usize,
) -> Result<f64, Totalwatsed3Error> {
    let value = f64_value(path, column_name, array, row, row_index)?;
    if value < 0.0 {
        return Err(Totalwatsed3Error::InvalidValue {
            path: path.to_path_buf(),
            column: column_name.to_string(),
            row_index,
            value,
        });
    }
    Ok(value)
}

fn optional_f64_value(
    path: &Path,
    column_name: &str,
    array: Option<&Float64Array>,
    row: usize,
    row_index: usize,
) -> Result<Option<f64>, Totalwatsed3Error> {
    let Some(array) = array else {
        return Ok(None);
    };
    if array.is_null(row) {
        if array.null_count() == array.len() {
            return Ok(None);
        }
        return Err(null_value(path, column_name, row_index));
    }
    f64_value(path, column_name, array, row, row_index).map(Some)
}

fn null_value(path: &Path, column_name: &str, row_index: usize) -> Totalwatsed3Error {
    Totalwatsed3Error::NullValue {
        path: path.to_path_buf(),
        column: column_name.to_string(),
        row_index,
    }
}
