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
    frozen_water_mm_m2: f64,
    snow_water_mm_m2: f64,
    tile_mm_m2: f64,
    irrigation_mm_m2: f64,
    baseflow_mm_m2: f64,
}

impl DailyWatAccumulator {
    fn add_weighted(&mut self, area_m2: f64, values: WatRowValues) {
        self.area_m2 += area_m2;
        self.precipitation_mm_m2 += values.precipitation_mm * area_m2;
        self.rain_melt_mm_m2 += values.rain_melt_mm * area_m2;
        self.runoff_mm_m2 += values.runoff_mm * area_m2;
        self.deep_percolation_mm_m2 += values.deep_percolation_mm * area_m2;
        self.lateral_flow_mm_m2 += values.lateral_flow_mm * area_m2;
        self.qofe_mm_m2 += values.qofe_mm * area_m2;
        self.transpiration_mm_m2 += values.transpiration_mm * area_m2;
        self.evaporation_soil_mm_m2 += values.evaporation_soil_mm * area_m2;
        self.evaporation_residue_mm_m2 += values.evaporation_residue_mm * area_m2;
        self.upstream_q_mm_m2 += values.upstream_q_mm * area_m2;
        self.subsurface_runon_mm_m2 += values.subsurface_runon_mm * area_m2;
        self.total_soil_water_mm_m2 += values.total_soil_water_mm * area_m2;
        self.soil_water_total_mm_m2 += values.soil_water_total_mm * area_m2;
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
    frozen_water_mm: f64,
    snow_water_mm: f64,
    tile_mm: f64,
    irrigation_mm: f64,
    baseflow_mm: f64,
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
    for batch_result in reader {
        let batch = batch_result.map_err(|error| WatershedWatPublicationError::Read {
            path: path.to_path_buf(),
            detail: error.to_string(),
        })?;
        read_batch_into(path, &batch, row_offset, daily)?;
        row_offset += batch.num_rows();
    }
    Ok(())
}

#[allow(clippy::too_many_lines)]
fn read_batch_into(
    path: &Path,
    batch: &RecordBatch,
    row_offset: usize,
    daily: &mut BTreeMap<DayKey, DailyWatAccumulator>,
) -> Result<(), WatershedWatPublicationError> {
    let years = int16_column(path, batch, "year")?;
    let sim_day_indexes = int32_column(path, batch, "sim_day_index")?;
    let julians = int16_column(path, batch, "julian")?;
    let months = int8_column(path, batch, "month")?;
    let day_of_months = int8_column(path, batch, "day_of_month")?;
    let water_years = int16_column(path, batch, "water_year")?;
    let areas = f64_column(path, batch, "Area")?;
    let precipitation = f64_column(path, batch, "P")?;
    let rain_melt = f64_column(path, batch, "RM")?;
    let runoff = f64_column(path, batch, "Q")?;
    let deep_percolation = f64_column(path, batch, "Dp")?;
    let lateral_flow = f64_column(path, batch, "latqcc")?;
    let qofe = f64_column(path, batch, "QOFE")?;
    let transpiration = f64_column(path, batch, "Ep")?;
    let evaporation_soil = f64_column(path, batch, "Es")?;
    let evaporation_residue = f64_column(path, batch, "Er")?;
    let upstream_q = f64_column(path, batch, "UpStrmQ")?;
    let subsurface_runon = f64_column(path, batch, "SubRIn")?;
    let total_soil_water = f64_column_any(path, batch, &["Total-Soil Water", "Total-Soil"])?;
    let soil_water_total = f64_column(path, batch, "SoilWaterTotal")?;
    let frozen_water = f64_column(path, batch, "frozwt")?;
    let snow_water = f64_column(path, batch, "Snow-Water")?;
    let tile = optional_f64_column(path, batch, "Tile")?;
    let irrigation = optional_f64_column(path, batch, "Irr")?;
    let baseflow = optional_f64_column(path, batch, "Base")?;

    for row in 0..batch.num_rows() {
        let row_index = row_offset + row;
        let area_m2 = f64_value(path, "Area", areas, row, row_index)?;
        if area_m2 <= 0.0 {
            return Err(WatershedWatPublicationError::InvalidValue {
                path: path.to_path_buf(),
                column: "Area".to_string(),
                row_index,
                value: area_m2,
            });
        }

        let key = DayKey {
            year: int16_value(path, "year", years, row, row_index)?,
            sim_day_index: int32_value(path, "sim_day_index", sim_day_indexes, row, row_index)?,
            julian: int16_value(path, "julian", julians, row, row_index)?,
            month: int8_value(path, "month", months, row, row_index)?,
            day_of_month: int8_value(path, "day_of_month", day_of_months, row, row_index)?,
            water_year: int16_value(path, "water_year", water_years, row, row_index)?,
        };
        let values = WatRowValues {
            precipitation_mm: f64_value(path, "P", precipitation, row, row_index)?,
            rain_melt_mm: f64_value(path, "RM", rain_melt, row, row_index)?,
            runoff_mm: f64_value(path, "Q", runoff, row, row_index)?,
            deep_percolation_mm: f64_value(path, "Dp", deep_percolation, row, row_index)?,
            lateral_flow_mm: f64_value(path, "latqcc", lateral_flow, row, row_index)?,
            qofe_mm: f64_value(path, "QOFE", qofe, row, row_index)?,
            transpiration_mm: f64_value(path, "Ep", transpiration, row, row_index)?,
            evaporation_soil_mm: f64_value(path, "Es", evaporation_soil, row, row_index)?,
            evaporation_residue_mm: f64_value(path, "Er", evaporation_residue, row, row_index)?,
            upstream_q_mm: f64_value(path, "UpStrmQ", upstream_q, row, row_index)?,
            subsurface_runon_mm: f64_value(path, "SubRIn", subsurface_runon, row, row_index)?,
            total_soil_water_mm: f64_value(
                path,
                "Total-Soil Water",
                total_soil_water,
                row,
                row_index,
            )?,
            soil_water_total_mm: f64_value(
                path,
                "SoilWaterTotal",
                soil_water_total,
                row,
                row_index,
            )?,
            frozen_water_mm: f64_value(path, "frozwt", frozen_water, row, row_index)?,
            snow_water_mm: f64_value(path, "Snow-Water", snow_water, row, row_index)?,
            tile_mm: optional_f64_value(path, "Tile", tile, row, row_index)?,
            irrigation_mm: optional_f64_value(path, "Irr", irrigation, row, row_index)?,
            baseflow_mm: optional_f64_value(path, "Base", baseflow, row, row_index)?,
        };
        daily.entry(key).or_default().add_weighted(area_m2, values);
    }

    Ok(())
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
    array.map_or(Ok(0.0), |array| {
        f64_value(path, column_name, array, row, row_index)
    })
}

fn null_value(path: &Path, column_name: &str, row_index: usize) -> WatershedWatPublicationError {
    WatershedWatPublicationError::NullValue {
        path: path.to_path_buf(),
        column: column_name.to_string(),
        row_index,
    }
}
