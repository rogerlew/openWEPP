use std::collections::BTreeMap;
use std::fmt;
use std::fs::File;
use std::path::{Path, PathBuf};

use arrow_array::{Array, Float64Array, Int8Array, Int16Array, Int32Array, RecordBatch};
use openwepp_watershed_output::writers::WatershedInterchangeRowSeed;
use parquet::arrow::arrow_reader::ParquetRecordBatchReaderBuilder;

#[derive(Debug)]
pub enum WatershedWatPublicationError {
    MissingWatSibling {
        pass_file: PathBuf,
        expected_wat_file: PathBuf,
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
}

impl fmt::Display for WatershedWatPublicationError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::MissingWatSibling {
                pass_file,
                expected_wat_file,
            } => write!(
                formatter,
                "missing sibling WAT parquet for pass file {}: expected {}",
                pass_file.display(),
                expected_wat_file.display()
            ),
            Self::Open { path, detail } => {
                write!(
                    formatter,
                    "failed opening WAT parquet {}: {detail}",
                    path.display()
                )
            }
            Self::Read { path, detail } => {
                write!(
                    formatter,
                    "failed reading WAT parquet {}: {detail}",
                    path.display()
                )
            }
            Self::MissingColumn { path, column } => write!(
                formatter,
                "WAT parquet {} is missing required column {column}",
                path.display()
            ),
            Self::UnsupportedColumnType { path, column } => write!(
                formatter,
                "WAT parquet {} has unsupported type for column {column}",
                path.display()
            ),
            Self::NullValue {
                path,
                column,
                row_index,
            } => write!(
                formatter,
                "WAT parquet {} has null value in column {column} at row {row_index}",
                path.display()
            ),
            Self::InvalidValue {
                path,
                column,
                row_index,
                value,
            } => write!(
                formatter,
                "WAT parquet {} has invalid value {value} in column {column} at row {row_index}",
                path.display()
            ),
        }
    }
}

impl std::error::Error for WatershedWatPublicationError {}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
struct DayKey {
    year: i16,
    sim_day_index: i32,
    julian: i16,
    month: i8,
    day_of_month: i8,
    water_year: i16,
}

#[derive(Debug, Clone, Copy, Default)]
#[allow(clippy::struct_field_names)]
struct DailyWatAccumulator {
    area_m2: f64,
    precipitation_mm_m2: f64,
    rain_melt_mm_m2: f64,
    runoff_mm_m2: f64,
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
    baseflow_mm_m2: f64,
}

impl DailyWatAccumulator {
    fn add_weighted(&mut self, area_m2: f64, values: WatRowValues, include_lateral_flow: bool) {
        self.area_m2 += area_m2;
        self.precipitation_mm_m2 += values.precipitation_mm * area_m2;
        self.rain_melt_mm_m2 += values.rain_melt_mm * area_m2;
        self.runoff_mm_m2 += values.runoff_mm * area_m2;
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
        self.baseflow_mm_m2 += values.baseflow_mm * area_m2;
    }
}

#[derive(Debug, Clone, Copy)]
#[allow(clippy::struct_field_names)]
struct WatRowValues {
    precipitation_mm: f64,
    rain_melt_mm: f64,
    runoff_mm: f64,
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
    baseflow_mm: f64,
}

#[derive(Debug, Clone, Copy)]
struct WatFileRow {
    key: DayKey,
    wepp_id: Option<i32>,
    ofe_id: Option<i16>,
    area_m2: f64,
    values: WatRowValues,
}

#[derive(Debug, Clone, Copy)]
struct WatBatchColumns<'a> {
    identity: WatIdentityColumns<'a>,
    values: WatValueColumns<'a>,
}

#[derive(Debug, Clone, Copy)]
struct WatIdentityColumns<'a> {
    wepp_ids: Option<&'a Int32Array>,
    ofe_ids: Option<&'a Int16Array>,
    years: &'a Int16Array,
    sim_day_indexes: &'a Int32Array,
    julians: &'a Int16Array,
    months: &'a Int8Array,
    day_of_months: &'a Int8Array,
    water_years: &'a Int16Array,
    areas: &'a Float64Array,
}

#[derive(Debug, Clone, Copy)]
struct WatValueColumns<'a> {
    precipitation: &'a Float64Array,
    rain_melt: &'a Float64Array,
    runoff: &'a Float64Array,
    deep_percolation: &'a Float64Array,
    lateral_flow: &'a Float64Array,
    qofe: &'a Float64Array,
    transpiration: &'a Float64Array,
    evaporation_soil: &'a Float64Array,
    evaporation_residue: &'a Float64Array,
    upstream_q: &'a Float64Array,
    subsurface_runon: &'a Float64Array,
    total_soil_water: &'a Float64Array,
    soil_water_total: &'a Float64Array,
    profile_depth: Option<&'a Float64Array>,
    profile_porosity_cap: Option<&'a Float64Array>,
    profile_fc_store: Option<&'a Float64Array>,
    profile_wp_store: Option<&'a Float64Array>,
    interception: Option<&'a Float64Array>,
    interception_storage: Option<&'a Float64Array>,
    frozen_water: &'a Float64Array,
    snow_water: &'a Float64Array,
    tile: Option<&'a Float64Array>,
    irrigation: Option<&'a Float64Array>,
    baseflow: Option<&'a Float64Array>,
}

impl<'a> WatBatchColumns<'a> {
    fn load(path: &Path, batch: &'a RecordBatch) -> Result<Self, WatershedWatPublicationError> {
        Ok(Self {
            identity: WatIdentityColumns::load(path, batch)?,
            values: WatValueColumns::load(path, batch)?,
        })
    }
}

impl<'a> WatIdentityColumns<'a> {
    fn load(path: &Path, batch: &'a RecordBatch) -> Result<Self, WatershedWatPublicationError> {
        Ok(Self {
            wepp_ids: optional_int32_column(path, batch, "wepp_id")?,
            ofe_ids: optional_int16_column_any(path, batch, &["ofe_id", "OFE"])?,
            years: int16_column(path, batch, "year")?,
            sim_day_indexes: int32_column(path, batch, "sim_day_index")?,
            julians: int16_column(path, batch, "julian")?,
            months: int8_column(path, batch, "month")?,
            day_of_months: int8_column(path, batch, "day_of_month")?,
            water_years: int16_column(path, batch, "water_year")?,
            areas: f64_column(path, batch, "Area")?,
        })
    }
}

impl<'a> WatValueColumns<'a> {
    fn load(path: &Path, batch: &'a RecordBatch) -> Result<Self, WatershedWatPublicationError> {
        Ok(Self {
            precipitation: f64_column(path, batch, "P")?,
            rain_melt: f64_column(path, batch, "RM")?,
            runoff: f64_column(path, batch, "Q")?,
            deep_percolation: f64_column(path, batch, "Dp")?,
            lateral_flow: f64_column(path, batch, "latqcc")?,
            qofe: f64_column(path, batch, "QOFE")?,
            transpiration: f64_column(path, batch, "Ep")?,
            evaporation_soil: f64_column(path, batch, "Es")?,
            evaporation_residue: f64_column(path, batch, "Er")?,
            upstream_q: f64_column(path, batch, "UpStrmQ")?,
            subsurface_runon: f64_column(path, batch, "SubRIn")?,
            total_soil_water: f64_column_any(path, batch, &["Total-Soil Water", "Total-Soil"])?,
            soil_water_total: f64_column(path, batch, "SoilWaterTotal")?,
            profile_depth: optional_f64_column(path, batch, "ProfileDepth")?,
            profile_porosity_cap: optional_f64_column(path, batch, "ProfilePorosityCap")?,
            profile_fc_store: optional_f64_column(path, batch, "ProfileFCStore")?,
            profile_wp_store: optional_f64_column(path, batch, "ProfileWPStore")?,
            interception: optional_f64_column(path, batch, "Interception")?,
            interception_storage: optional_f64_column(path, batch, "InterceptionStorage")?,
            frozen_water: f64_column(path, batch, "frozwt")?,
            snow_water: f64_column(path, batch, "Snow-Water")?,
            tile: optional_f64_column(path, batch, "Tile")?,
            irrigation: optional_f64_column(path, batch, "Irr")?,
            baseflow: optional_f64_column(path, batch, "Base")?,
        })
    }
}

pub fn build_watershed_daily_rows_from_wat<I, P>(
    pass_file_paths: I,
    base_seed: WatershedInterchangeRowSeed,
) -> Result<Option<Vec<WatershedInterchangeRowSeed>>, WatershedWatPublicationError>
where
    I: IntoIterator<Item = P>,
    P: AsRef<Path>,
{
    let pass_files = pass_file_paths
        .into_iter()
        .map(|path| path.as_ref().to_path_buf())
        .collect::<Vec<_>>();

    let mut wat_files = Vec::new();
    let mut missing = Vec::new();
    for pass_file in &pass_files {
        let wat_file = pass_file.with_extension("wat.parquet");
        if wat_file.is_file() {
            wat_files.push(wat_file);
        } else {
            missing.push((pass_file.clone(), wat_file));
        }
    }

    if wat_files.is_empty() {
        return Ok(None);
    }
    if let Some((pass_file, expected_wat_file)) = missing.into_iter().next() {
        return Err(WatershedWatPublicationError::MissingWatSibling {
            pass_file,
            expected_wat_file,
        });
    }

    let mut daily = BTreeMap::<DayKey, DailyWatAccumulator>::new();
    for wat_file in &wat_files {
        read_wat_file_into(wat_file, &mut daily)?;
    }

    let mut rows = Vec::with_capacity(daily.len());
    for (key, aggregate) in daily {
        if !aggregate.area_m2.is_finite() || aggregate.area_m2 <= 0.0 {
            return Err(WatershedWatPublicationError::InvalidValue {
                path: PathBuf::from("<aggregated WAT>"),
                column: "Area".to_string(),
                row_index: usize::try_from(key.sim_day_index).unwrap_or(usize::MAX),
                value: aggregate.area_m2,
            });
        }

        let area = aggregate.area_m2;
        let runoff_mm = aggregate.runoff_mm_m2 / area;
        let runoff_volume_m3 = runoff_mm * area / 1_000.0;
        let baseflow_mm = aggregate.baseflow_mm_m2 / area;

        rows.push(WatershedInterchangeRowSeed {
            year: key.year,
            simulation_year: key.year,
            sim_day_index: key.sim_day_index,
            julian: key.julian,
            month: key.month,
            day_of_month: key.day_of_month,
            water_year: key.water_year,
            runoff_volume_m3,
            channel_outflow_m3: runoff_volume_m3,
            channel_baseflow_m3: baseflow_mm * area / 1_000.0,
            area_m2: area,
            precipitation_mm: aggregate.precipitation_mm_m2 / area,
            rain_melt_mm: aggregate.rain_melt_mm_m2 / area,
            runoff_mm,
            deep_percolation_mm: aggregate.deep_percolation_mm_m2 / area,
            lateral_flow_mm: aggregate.lateral_flow_mm_m2 / area,
            qofe_mm: aggregate.qofe_mm_m2 / area,
            transpiration_mm: aggregate.transpiration_mm_m2 / area,
            evaporation_soil_mm: aggregate.evaporation_soil_mm_m2 / area,
            evaporation_residue_mm: aggregate.evaporation_residue_mm_m2 / area,
            upstream_q_mm: aggregate.upstream_q_mm_m2 / area,
            subsurface_runon_mm: aggregate.subsurface_runon_mm_m2 / area,
            total_soil_water_mm: aggregate.total_soil_water_mm_m2 / area,
            soil_water_total_mm: aggregate.soil_water_total_mm_m2 / area,
            profile_depth_mm: aggregate.profile_depth_mm_m2 / area,
            profile_porosity_cap_mm: aggregate.profile_porosity_cap_mm_m2 / area,
            profile_fc_store_mm: aggregate.profile_fc_store_mm_m2 / area,
            profile_wp_store_mm: aggregate.profile_wp_store_mm_m2 / area,
            interception_mm: aggregate.interception_mm_m2 / area,
            interception_storage_mm: aggregate.interception_storage_mm_m2 / area,
            frozen_water_mm: aggregate.frozen_water_mm_m2 / area,
            snow_water_mm: aggregate.snow_water_mm_m2 / area,
            tile_mm: aggregate.tile_mm_m2 / area,
            irrigation_mm: aggregate.irrigation_mm_m2 / area,
            baseflow_mm,
            ..base_seed
        });
    }

    Ok(Some(rows))
}

fn read_wat_file_into(
    path: &Path,
    daily: &mut BTreeMap<DayKey, DailyWatAccumulator>,
) -> Result<(), WatershedWatPublicationError> {
    let file = File::open(path).map_err(|error| WatershedWatPublicationError::Open {
        path: path.to_path_buf(),
        detail: error.to_string(),
    })?;
    let builder = ParquetRecordBatchReaderBuilder::try_new(file).map_err(|error| {
        WatershedWatPublicationError::Read {
            path: path.to_path_buf(),
            detail: error.to_string(),
        }
    })?;
    let reader = builder
        .build()
        .map_err(|error| WatershedWatPublicationError::Read {
            path: path.to_path_buf(),
            detail: error.to_string(),
        })?;

    let mut row_offset = 0_usize;
    let mut file_rows = Vec::new();
    for batch_result in reader {
        let batch = batch_result.map_err(|error| WatershedWatPublicationError::Read {
            path: path.to_path_buf(),
            detail: error.to_string(),
        })?;
        read_batch_into(path, &batch, row_offset, &mut file_rows)?;
        row_offset += batch.num_rows();
    }

    aggregate_file_rows_into(daily, file_rows);

    Ok(())
}

fn aggregate_file_rows_into(
    daily: &mut BTreeMap<DayKey, DailyWatAccumulator>,
    file_rows: Vec<WatFileRow>,
) {
    let mut outlet_ofe_by_day = BTreeMap::<(DayKey, Option<i32>), i16>::new();
    for row in &file_rows {
        if let Some(ofe_id) = row.ofe_id {
            outlet_ofe_by_day
                .entry((row.key, row.wepp_id))
                .and_modify(|current| *current = (*current).max(ofe_id))
                .or_insert(ofe_id);
        }
    }

    for row in file_rows {
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
}

fn read_batch_into(
    path: &Path,
    batch: &RecordBatch,
    row_offset: usize,
    rows: &mut Vec<WatFileRow>,
) -> Result<(), WatershedWatPublicationError> {
    let columns = WatBatchColumns::load(path, batch)?;
    for row in 0..batch.num_rows() {
        let row_index = row_offset + row;
        rows.push(read_wat_file_row(path, &columns, row, row_index)?);
    }

    Ok(())
}

fn read_wat_file_row(
    path: &Path,
    columns: &WatBatchColumns<'_>,
    row: usize,
    row_index: usize,
) -> Result<WatFileRow, WatershedWatPublicationError> {
    let identity = columns.identity;
    Ok(WatFileRow {
        key: day_key_from_columns(path, columns, row, row_index)?,
        wepp_id: optional_int32_value(path, "wepp_id", identity.wepp_ids, row, row_index)?,
        ofe_id: optional_int16_value(path, "ofe_id|OFE", identity.ofe_ids, row, row_index)?,
        area_m2: positive_area_m2(path, identity.areas, row, row_index)?,
        values: wat_values_from_columns(path, columns, row, row_index)?,
    })
}

fn positive_area_m2(
    path: &Path,
    areas: &Float64Array,
    row: usize,
    row_index: usize,
) -> Result<f64, WatershedWatPublicationError> {
    let area_m2 = f64_value(path, "Area", areas, row, row_index)?;
    if area_m2 <= 0.0 {
        return Err(WatershedWatPublicationError::InvalidValue {
            path: path.to_path_buf(),
            column: "Area".to_string(),
            row_index,
            value: area_m2,
        });
    }
    Ok(area_m2)
}

fn day_key_from_columns(
    path: &Path,
    columns: &WatBatchColumns<'_>,
    row: usize,
    row_index: usize,
) -> Result<DayKey, WatershedWatPublicationError> {
    let identity = columns.identity;
    Ok(DayKey {
        year: int16_value(path, "year", identity.years, row, row_index)?,
        sim_day_index: int32_value(
            path,
            "sim_day_index",
            identity.sim_day_indexes,
            row,
            row_index,
        )?,
        julian: int16_value(path, "julian", identity.julians, row, row_index)?,
        month: int8_value(path, "month", identity.months, row, row_index)?,
        day_of_month: int8_value(path, "day_of_month", identity.day_of_months, row, row_index)?,
        water_year: int16_value(path, "water_year", identity.water_years, row, row_index)?,
    })
}

fn wat_values_from_columns(
    path: &Path,
    columns: &WatBatchColumns<'_>,
    row: usize,
    row_index: usize,
) -> Result<WatRowValues, WatershedWatPublicationError> {
    let values = columns.values;
    Ok(WatRowValues {
        precipitation_mm: f64_value(path, "P", values.precipitation, row, row_index)?,
        rain_melt_mm: f64_value(path, "RM", values.rain_melt, row, row_index)?,
        runoff_mm: f64_value(path, "Q", values.runoff, row, row_index)?,
        deep_percolation_mm: f64_value(path, "Dp", values.deep_percolation, row, row_index)?,
        lateral_flow_mm: f64_value(path, "latqcc", values.lateral_flow, row, row_index)?,
        qofe_mm: f64_value(path, "QOFE", values.qofe, row, row_index)?,
        transpiration_mm: f64_value(path, "Ep", values.transpiration, row, row_index)?,
        evaporation_soil_mm: f64_value(path, "Es", values.evaporation_soil, row, row_index)?,
        evaporation_residue_mm: f64_value(path, "Er", values.evaporation_residue, row, row_index)?,
        upstream_q_mm: f64_value(path, "UpStrmQ", values.upstream_q, row, row_index)?,
        subsurface_runon_mm: f64_value(path, "SubRIn", values.subsurface_runon, row, row_index)?,
        total_soil_water_mm: f64_value(
            path,
            "Total-Soil Water",
            values.total_soil_water,
            row,
            row_index,
        )?,
        soil_water_total_mm: f64_value(
            path,
            "SoilWaterTotal",
            values.soil_water_total,
            row,
            row_index,
        )?,
        profile_depth_mm: optional_f64_value(
            path,
            "ProfileDepth",
            values.profile_depth,
            row,
            row_index,
        )?,
        profile_porosity_cap_mm: optional_f64_value(
            path,
            "ProfilePorosityCap",
            values.profile_porosity_cap,
            row,
            row_index,
        )?,
        profile_fc_store_mm: optional_f64_value(
            path,
            "ProfileFCStore",
            values.profile_fc_store,
            row,
            row_index,
        )?,
        profile_wp_store_mm: optional_f64_value(
            path,
            "ProfileWPStore",
            values.profile_wp_store,
            row,
            row_index,
        )?,
        interception_mm: optional_f64_value(
            path,
            "Interception",
            values.interception,
            row,
            row_index,
        )?,
        interception_storage_mm: optional_f64_value(
            path,
            "InterceptionStorage",
            values.interception_storage,
            row,
            row_index,
        )?,
        frozen_water_mm: f64_value(path, "frozwt", values.frozen_water, row, row_index)?,
        snow_water_mm: f64_value(path, "Snow-Water", values.snow_water, row, row_index)?,
        tile_mm: optional_f64_value(path, "Tile", values.tile, row, row_index)?,
        irrigation_mm: optional_f64_value(path, "Irr", values.irrigation, row, row_index)?,
        baseflow_mm: optional_f64_value(path, "Base", values.baseflow, row, row_index)?,
    })
}

fn int8_column<'a>(
    path: &Path,
    batch: &'a RecordBatch,
    name: &str,
) -> Result<&'a Int8Array, WatershedWatPublicationError> {
    column(path, batch, name)
}

fn int16_column<'a>(
    path: &Path,
    batch: &'a RecordBatch,
    name: &str,
) -> Result<&'a Int16Array, WatershedWatPublicationError> {
    column(path, batch, name)
}

fn int32_column<'a>(
    path: &Path,
    batch: &'a RecordBatch,
    name: &str,
) -> Result<&'a Int32Array, WatershedWatPublicationError> {
    column(path, batch, name)
}

fn f64_column<'a>(
    path: &Path,
    batch: &'a RecordBatch,
    name: &str,
) -> Result<&'a Float64Array, WatershedWatPublicationError> {
    column(path, batch, name)
}

fn optional_int16_column_any<'a>(
    path: &Path,
    batch: &'a RecordBatch,
    names: &[&str],
) -> Result<Option<&'a Int16Array>, WatershedWatPublicationError> {
    for name in names {
        match optional_int16_column(path, batch, name) {
            Ok(Some(column)) => return Ok(Some(column)),
            Ok(None) => {}
            Err(error) => return Err(error),
        }
    }
    Ok(None)
}

fn f64_column_any<'a>(
    path: &Path,
    batch: &'a RecordBatch,
    names: &[&str],
) -> Result<&'a Float64Array, WatershedWatPublicationError> {
    for name in names {
        match optional_f64_column(path, batch, name) {
            Ok(Some(column)) => return Ok(column),
            Ok(None) => {}
            Err(error) => return Err(error),
        }
    }
    Err(WatershedWatPublicationError::MissingColumn {
        path: path.to_path_buf(),
        column: names.join("|"),
    })
}

fn optional_int16_column<'a>(
    path: &Path,
    batch: &'a RecordBatch,
    name: &str,
) -> Result<Option<&'a Int16Array>, WatershedWatPublicationError> {
    let schema = batch.schema();
    let Ok(index) = schema.index_of(name) else {
        return Ok(None);
    };
    batch
        .column(index)
        .as_any()
        .downcast_ref::<Int16Array>()
        .map(Some)
        .ok_or_else(|| WatershedWatPublicationError::UnsupportedColumnType {
            path: path.to_path_buf(),
            column: name.to_string(),
        })
}

fn optional_int32_column<'a>(
    path: &Path,
    batch: &'a RecordBatch,
    name: &str,
) -> Result<Option<&'a Int32Array>, WatershedWatPublicationError> {
    let schema = batch.schema();
    let Ok(index) = schema.index_of(name) else {
        return Ok(None);
    };
    batch
        .column(index)
        .as_any()
        .downcast_ref::<Int32Array>()
        .map(Some)
        .ok_or_else(|| WatershedWatPublicationError::UnsupportedColumnType {
            path: path.to_path_buf(),
            column: name.to_string(),
        })
}

fn optional_f64_column<'a>(
    path: &Path,
    batch: &'a RecordBatch,
    name: &str,
) -> Result<Option<&'a Float64Array>, WatershedWatPublicationError> {
    let schema = batch.schema();
    let Ok(index) = schema.index_of(name) else {
        return Ok(None);
    };
    batch
        .column(index)
        .as_any()
        .downcast_ref::<Float64Array>()
        .map(Some)
        .ok_or_else(|| WatershedWatPublicationError::UnsupportedColumnType {
            path: path.to_path_buf(),
            column: name.to_string(),
        })
}

fn column<'a, T: 'static>(
    path: &Path,
    batch: &'a RecordBatch,
    name: &str,
) -> Result<&'a T, WatershedWatPublicationError> {
    let schema = batch.schema();
    let index = schema
        .index_of(name)
        .map_err(|_| WatershedWatPublicationError::MissingColumn {
            path: path.to_path_buf(),
            column: name.to_string(),
        })?;
    batch
        .column(index)
        .as_any()
        .downcast_ref::<T>()
        .ok_or_else(|| WatershedWatPublicationError::UnsupportedColumnType {
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
) -> Result<i8, WatershedWatPublicationError> {
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
) -> Result<i16, WatershedWatPublicationError> {
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
) -> Result<i32, WatershedWatPublicationError> {
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
) -> Result<Option<i16>, WatershedWatPublicationError> {
    array.map_or(Ok(None), |array| {
        int16_value(path, column_name, array, row, row_index).map(Some)
    })
}

fn optional_int32_value(
    path: &Path,
    column_name: &str,
    array: Option<&Int32Array>,
    row: usize,
    row_index: usize,
) -> Result<Option<i32>, WatershedWatPublicationError> {
    array.map_or(Ok(None), |array| {
        int32_value(path, column_name, array, row, row_index).map(Some)
    })
}

fn f64_value(
    path: &Path,
    column_name: &str,
    array: &Float64Array,
    row: usize,
    row_index: usize,
) -> Result<f64, WatershedWatPublicationError> {
    if array.is_null(row) {
        return Err(null_value(path, column_name, row_index));
    }
    let value = array.value(row);
    if !value.is_finite() {
        return Err(WatershedWatPublicationError::InvalidValue {
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
) -> Result<f64, WatershedWatPublicationError> {
    let Some(array) = array else {
        return Ok(0.0);
    };
    if array.is_null(row) {
        if array.null_count() == array.len() {
            return Ok(0.0);
        }
        return Err(null_value(path, column_name, row_index));
    }
    f64_value(path, column_name, array, row, row_index)
}

fn null_value(path: &Path, column_name: &str, row_index: usize) -> WatershedWatPublicationError {
    WatershedWatPublicationError::NullValue {
        path: path.to_path_buf(),
        column: column_name.to_string(),
        row_index,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::Arc;

    use arrow_array::ArrayRef;
    use arrow_schema::{DataType, Field, Schema};

    fn day_key() -> DayKey {
        DayKey {
            year: 2004,
            sim_day_index: 1,
            julian: 1,
            month: 1,
            day_of_month: 1,
            water_year: 2004,
        }
    }

    fn values(
        runoff_mm: f64,
        lateral_flow_mm: f64,
        profile_porosity_cap_mm: f64,
        interception_mm: f64,
    ) -> WatRowValues {
        WatRowValues {
            precipitation_mm: 10.0,
            rain_melt_mm: 10.0,
            runoff_mm,
            deep_percolation_mm: 0.5,
            lateral_flow_mm,
            qofe_mm: runoff_mm + 1.0,
            transpiration_mm: 2.0,
            evaporation_soil_mm: 0.25,
            evaporation_residue_mm: 0.0,
            upstream_q_mm: 0.0,
            subsurface_runon_mm: 0.0,
            total_soil_water_mm: 100.0,
            soil_water_total_mm: 100.0,
            profile_depth_mm: 1_000.0,
            profile_porosity_cap_mm,
            profile_fc_store_mm: 150.0,
            profile_wp_store_mm: 50.0,
            interception_mm,
            interception_storage_mm: 0.25,
            frozen_water_mm: 0.0,
            snow_water_mm: 0.0,
            tile_mm: 0.0,
            irrigation_mm: 0.0,
            baseflow_mm: 0.0,
        }
    }

    fn field(name: &str, data_type: DataType, nullable: bool) -> Field {
        Field::new(name, data_type, nullable)
    }

    fn i8_array(values: Vec<i8>) -> ArrayRef {
        Arc::new(Int8Array::from(values))
    }

    fn i16_array(values: Vec<i16>) -> ArrayRef {
        Arc::new(Int16Array::from(values))
    }

    fn i32_array(values: Vec<i32>) -> ArrayRef {
        Arc::new(Int32Array::from(values))
    }

    fn f64_array(values: Vec<f64>) -> ArrayRef {
        Arc::new(Float64Array::from(values))
    }

    fn nullable_f64_array(values: Vec<Option<f64>>) -> ArrayRef {
        Arc::new(Float64Array::from(values))
    }

    fn wat_batch_for_reader_tests(area: Vec<f64>) -> RecordBatch {
        let schema = Schema::new(vec![
            field("wepp_id", DataType::Int32, false),
            field("OFE", DataType::Int16, false),
            field("year", DataType::Int16, false),
            field("sim_day_index", DataType::Int32, false),
            field("julian", DataType::Int16, false),
            field("month", DataType::Int8, false),
            field("day_of_month", DataType::Int8, false),
            field("water_year", DataType::Int16, false),
            field("Area", DataType::Float64, false),
            field("P", DataType::Float64, false),
            field("RM", DataType::Float64, false),
            field("Q", DataType::Float64, false),
            field("Dp", DataType::Float64, false),
            field("latqcc", DataType::Float64, false),
            field("QOFE", DataType::Float64, false),
            field("Ep", DataType::Float64, false),
            field("Es", DataType::Float64, false),
            field("Er", DataType::Float64, false),
            field("UpStrmQ", DataType::Float64, false),
            field("SubRIn", DataType::Float64, false),
            field("Total-Soil", DataType::Float64, false),
            field("SoilWaterTotal", DataType::Float64, false),
            field("ProfileDepth", DataType::Float64, false),
            field("ProfilePorosityCap", DataType::Float64, false),
            field("ProfileFCStore", DataType::Float64, false),
            field("ProfileWPStore", DataType::Float64, false),
            field("Interception", DataType::Float64, false),
            field("InterceptionStorage", DataType::Float64, true),
            field("frozwt", DataType::Float64, false),
            field("Snow-Water", DataType::Float64, false),
            field("Tile", DataType::Float64, false),
            field("Irr", DataType::Float64, false),
            field("Base", DataType::Float64, true),
        ]);
        let columns = vec![
            i32_array(vec![7, 7]),
            i16_array(vec![1, 2]),
            i16_array(vec![2004, 2004]),
            i32_array(vec![1, 2]),
            i16_array(vec![1, 2]),
            i8_array(vec![1, 1]),
            i8_array(vec![1, 2]),
            i16_array(vec![2004, 2004]),
            f64_array(area),
            f64_array(vec![10.0, 20.0]),
            f64_array(vec![8.0, 18.0]),
            f64_array(vec![5.0, 7.0]),
            f64_array(vec![0.5, 0.7]),
            f64_array(vec![0.2, 0.4]),
            f64_array(vec![6.0, 8.0]),
            f64_array(vec![2.0, 3.0]),
            f64_array(vec![0.25, 0.5]),
            f64_array(vec![0.125, 0.25]),
            f64_array(vec![1.0, 2.0]),
            f64_array(vec![0.75, 1.5]),
            f64_array(vec![100.0, 110.0]),
            f64_array(vec![101.0, 111.0]),
            f64_array(vec![1_000.0, 1_100.0]),
            f64_array(vec![250.0, 260.0]),
            f64_array(vec![180.0, 190.0]),
            f64_array(vec![60.0, 70.0]),
            f64_array(vec![1.25, 2.5]),
            nullable_f64_array(vec![None, None]),
            f64_array(vec![0.0, 0.1]),
            f64_array(vec![3.0, 4.0]),
            f64_array(vec![0.0, 0.2]),
            f64_array(vec![0.0, 0.3]),
            nullable_f64_array(vec![None, None]),
        ];
        RecordBatch::try_new(Arc::new(schema), columns).expect("test WAT batch should be valid")
    }

    #[test]
    fn aggregate_file_rows_uses_outlet_lateral_and_preserves_optional_wat_fields() {
        let key = day_key();
        let rows = vec![
            WatFileRow {
                key,
                wepp_id: Some(7),
                ofe_id: Some(1),
                area_m2: 500.0,
                values: values(5.0, 9.0, 210.0, 1.0),
            },
            WatFileRow {
                key,
                wepp_id: Some(7),
                ofe_id: Some(2),
                area_m2: 1_000.0,
                values: values(7.0, 4.0, 230.0, 3.0),
            },
            WatFileRow {
                key,
                wepp_id: Some(8),
                ofe_id: Some(1),
                area_m2: 1_000.0,
                values: values(11.0, 12.0, 250.0, 2.0),
            },
            WatFileRow {
                key,
                wepp_id: Some(8),
                ofe_id: Some(3),
                area_m2: 500.0,
                values: values(13.0, 6.0, 270.0, 4.0),
            },
        ];
        let mut daily = BTreeMap::<DayKey, DailyWatAccumulator>::new();

        aggregate_file_rows_into(&mut daily, rows);

        let aggregate = daily.get(&key).expect("day should aggregate");
        assert!((aggregate.area_m2 - 3_000.0).abs() <= 1.0e-12);
        assert!((aggregate.runoff_mm_m2 / aggregate.area_m2 - 9.0).abs() <= 1.0e-12);
        assert!(
            (aggregate.lateral_flow_mm_m2 / aggregate.area_m2 - 2.333_333_333_333_333_5).abs()
                <= 1.0e-12
        );
        assert!(
            (aggregate.profile_porosity_cap_mm_m2 / aggregate.area_m2 - 240.0).abs() <= 1.0e-12
        );
        assert!((aggregate.interception_mm_m2 / aggregate.area_m2 - 2.5).abs() <= 1.0e-12);
    }

    #[test]
    fn optional_f64_value_treats_all_null_column_as_absent_but_rejects_mixed_nulls() {
        let path = Path::new("<optional-test>");
        let all_nulls = Float64Array::from(vec![None, None]);
        let all_null_value =
            optional_f64_value(path, "InterceptionStorage", Some(&all_nulls), 0, 7)
                .expect("all-null optional column is absent-equivalent");
        assert!(all_null_value.abs() <= 1.0e-12);

        let mixed = Float64Array::from(vec![Some(1.25), None]);
        let mixed_value = optional_f64_value(path, "Interception", Some(&mixed), 0, 8)
            .expect("non-null optional value should read");
        assert!((mixed_value - 1.25).abs() <= 1.0e-12);
        let error = optional_f64_value(path, "Interception", Some(&mixed), 1, 9)
            .expect_err("mixed-null optional value should fail closed");
        assert!(matches!(
            error,
            WatershedWatPublicationError::NullValue {
                column,
                row_index: 9,
                ..
            } if column == "Interception"
        ));
    }

    #[test]
    fn read_batch_into_reads_aliases_optional_defaults_and_row_values() {
        let path = Path::new("<wat-batch-test>");
        let batch = wat_batch_for_reader_tests(vec![100.0, 300.0]);
        let mut rows = Vec::new();

        read_batch_into(path, &batch, 5, &mut rows).expect("batch should read");

        assert_eq!(rows.len(), 2);
        let first = rows[0];
        assert_eq!(first.key.year, 2004);
        assert_eq!(first.key.sim_day_index, 1);
        assert_eq!(first.key.julian, 1);
        assert_eq!(first.key.month, 1);
        assert_eq!(first.key.day_of_month, 1);
        assert_eq!(first.key.water_year, 2004);
        assert_eq!(first.wepp_id, Some(7));
        assert_eq!(first.ofe_id, Some(1));
        assert!((first.area_m2 - 100.0).abs() <= 1.0e-12);
        assert!((first.values.precipitation_mm - 10.0).abs() <= 1.0e-12);
        assert!((first.values.total_soil_water_mm - 100.0).abs() <= 1.0e-12);
        assert!((first.values.soil_water_total_mm - 101.0).abs() <= 1.0e-12);
        assert!((first.values.interception_storage_mm).abs() <= 1.0e-12);
        assert!((first.values.baseflow_mm).abs() <= 1.0e-12);

        let second = rows[1];
        assert_eq!(second.key.sim_day_index, 2);
        assert_eq!(second.key.day_of_month, 2);
        assert_eq!(second.ofe_id, Some(2));
        assert!((second.area_m2 - 300.0).abs() <= 1.0e-12);
        assert!((second.values.rain_melt_mm - 18.0).abs() <= 1.0e-12);
        assert!((second.values.profile_porosity_cap_mm - 260.0).abs() <= 1.0e-12);
        assert!((second.values.tile_mm - 0.2).abs() <= 1.0e-12);
        assert!((second.values.irrigation_mm - 0.3).abs() <= 1.0e-12);
    }

    #[test]
    fn read_batch_into_rejects_invalid_area_with_absolute_row_index() {
        let path = Path::new("<wat-batch-test>");
        let batch = wat_batch_for_reader_tests(vec![100.0, -1.0]);
        let mut rows = Vec::new();

        let error = read_batch_into(path, &batch, 10, &mut rows)
            .expect_err("negative area should fail closed");

        assert!(matches!(
            error,
            WatershedWatPublicationError::InvalidValue {
                column,
                row_index: 11,
                value,
                ..
            } if column == "Area" && (value + 1.0).abs() <= 1.0e-12
        ));
    }
}
