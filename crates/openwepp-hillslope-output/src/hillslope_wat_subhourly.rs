use std::collections::HashMap;
use std::fmt;
use std::fs::{File, OpenOptions};
use std::io;
use std::path::{Path, PathBuf};
use std::sync::Arc;

use arrow_array::{ArrayRef, Float64Array, Int32Array, RecordBatch, StringArray};
use arrow_schema::{DataType, Field, Schema};
use openwepp_sim_contract::units::validate_output_schema_unit;
use parquet::arrow::arrow_reader::ParquetRecordBatchReaderBuilder;
use parquet::arrow::{ArrowWriter, arrow_writer::ArrowWriterOptions};
use parquet::basic::Compression;
use parquet::file::properties::WriterProperties;

use crate::hillslope_pass::WriteSummary;

pub const HILLSLOPE_WAT_SUBHOURLY_SCHEMA_ID: &str = "openwepp-hillslope-wat-subhourly-v2.0";
const OUTPUT_REGISTRY_SCHEMA_ID: &str = "hillslope_wat_subhourly";
const WAT5_INTERVAL_SECONDS: f64 = 300.0;
const WAT5_INTERVALS_PER_HOUR: i32 = 12;
const WAT5_INTERVALS_PER_DAY: i32 = 288;
const WAT5_PUBLICATION_TOLERANCE_MM: f64 = 1.0e-9;

#[derive(Debug, Clone, PartialEq)]
pub struct HillslopeWatSubhourlyRow {
    pub wepp_id: i32,
    pub ofe_id: i32,
    pub year: i32,
    pub sim_day_index: i32,
    pub julian: i32,
    pub event_ordinal: i32,
    pub hour_index: i32,
    pub subinterval_index: i32,
    pub interval_start_s: f64,
    pub interval_duration_s: f64,
    pub rainfall_depth_mm: f64,
    pub additional_supply_depth_mm: f64,
    pub raw_green_ampt_infiltration_depth_mm: f64,
    pub depression_storage_retention_depth_mm: f64,
    pub raw_wb14_post_depression_generation_depth_mm: f64,
    pub closed_wb14_generation_depth_mm: f64,
    pub saturation_return_depth_mm: f64,
    pub closing_surface_generation_depth_mm: f64,
    pub closing_surface_generation_intensity_mm_h: f64,
    pub hourly_authoritative_runoff_depth_mm: f64,
    pub hourly_mean_generation_intensity_mm_h: f64,
    pub hourly_power_equivalent_generation_intensity_mm_h: Option<f64>,
    pub hourly_power_equivalent_duration_s: Option<f64>,
    pub power_exponent: Option<f64>,
    pub method_code: String,
    pub source_completeness_code: String,
    pub hourly_closure_residual_mm: f64,
}

#[derive(Debug)]
pub enum HillslopeWatSubhourlyError {
    Io { path: PathBuf, source: io::Error },
    Parquet { detail: String },
    UnitMetadata { detail: String },
}

impl HillslopeWatSubhourlyError {
    #[must_use]
    pub const fn code(&self) -> &'static str {
        match self {
            Self::Io { .. } => "OHOUT-WAT5-E-001",
            Self::Parquet { .. } => "OHOUT-WAT5-E-002",
            Self::UnitMetadata { .. } => "OHOUT-WAT5-E-003",
        }
    }
}

impl fmt::Display for HillslopeWatSubhourlyError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Io { path, source } => {
                write!(
                    formatter,
                    "{} WAT5-E-005 output I/O at {}: {source}",
                    self.code(),
                    path.display()
                )
            }
            Self::Parquet { detail } => write!(
                formatter,
                "{} WAT5-E-005 parquet error: {detail}",
                self.code()
            ),
            Self::UnitMetadata { detail } => {
                write!(
                    formatter,
                    "{} WAT5-E-005 unit metadata error: {detail}",
                    self.code()
                )
            }
        }
    }
}

impl std::error::Error for HillslopeWatSubhourlyError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::Io { source, .. } => Some(source),
            Self::Parquet { .. } | Self::UnitMetadata { .. } => None,
        }
    }
}

fn field(name: &str, data_type: DataType, nullable: bool, units: Option<&str>) -> Field {
    let field = Field::new(name, data_type, nullable);
    units.map_or(field.clone(), |unit| {
        field.with_metadata(HashMap::from([("units".to_string(), unit.to_string())]))
    })
}

#[allow(clippy::too_many_lines)]
pub fn hillslope_wat_subhourly_schema() -> Result<Schema, HillslopeWatSubhourlyError> {
    let fields = vec![
        field("wepp_id", DataType::Int32, false, None),
        field("ofe_id", DataType::Int32, false, None),
        field("year", DataType::Int32, false, None),
        field("sim_day_index", DataType::Int32, false, None),
        field("julian", DataType::Int32, false, None),
        field("event_ordinal", DataType::Int32, false, None),
        field("hour_index", DataType::Int32, false, None),
        field("subinterval_index", DataType::Int32, false, None),
        field("interval_start_s", DataType::Float64, false, Some("s")),
        field("interval_duration_s", DataType::Float64, false, Some("s")),
        field("rainfall_depth_mm", DataType::Float64, false, Some("mm")),
        field(
            "additional_supply_depth_mm",
            DataType::Float64,
            false,
            Some("mm"),
        ),
        field(
            "raw_green_ampt_infiltration_depth_mm",
            DataType::Float64,
            false,
            Some("mm"),
        ),
        field(
            "depression_storage_retention_depth_mm",
            DataType::Float64,
            false,
            Some("mm"),
        ),
        field(
            "raw_wb14_post_depression_generation_depth_mm",
            DataType::Float64,
            false,
            Some("mm"),
        ),
        field(
            "closed_wb14_generation_depth_mm",
            DataType::Float64,
            false,
            Some("mm"),
        ),
        field(
            "saturation_return_depth_mm",
            DataType::Float64,
            false,
            Some("mm"),
        ),
        field(
            "closing_surface_generation_depth_mm",
            DataType::Float64,
            false,
            Some("mm"),
        ),
        field(
            "closing_surface_generation_intensity_mm_h",
            DataType::Float64,
            false,
            Some("mm/h"),
        ),
        field(
            "hourly_authoritative_runoff_depth_mm",
            DataType::Float64,
            false,
            Some("mm"),
        ),
        field(
            "hourly_mean_generation_intensity_mm_h",
            DataType::Float64,
            false,
            Some("mm/h"),
        ),
        field(
            "hourly_power_equivalent_generation_intensity_mm_h",
            DataType::Float64,
            true,
            Some("mm/h"),
        ),
        field(
            "hourly_power_equivalent_duration_s",
            DataType::Float64,
            true,
            Some("s"),
        ),
        field("power_exponent", DataType::Float64, true, Some("1")),
        field("method_code", DataType::Utf8, false, None),
        field("source_completeness_code", DataType::Utf8, false, None),
        field(
            "hourly_closure_residual_mm",
            DataType::Float64,
            false,
            Some("mm"),
        ),
    ];
    for item in &fields {
        if let Some(unit) = item.metadata().get("units") {
            validate_output_schema_unit(OUTPUT_REGISTRY_SCHEMA_ID, item.name(), unit).map_err(
                |error| HillslopeWatSubhourlyError::UnitMetadata {
                    detail: error.to_string(),
                },
            )?;
        }
    }
    Ok(Schema::new_with_metadata(
        fields,
        HashMap::from([
            (
                "dataset_id".to_string(),
                HILLSLOPE_WAT_SUBHOURLY_SCHEMA_ID.to_string(),
            ),
            ("producer".to_string(), "openwepp".to_string()),
            (
                "sparse_support".to_string(),
                "first_through_last_active_bin".to_string(),
            ),
            ("omitted_bins".to_string(), "exact_zero".to_string()),
            (
                "raw_wb14_semantics".to_string(),
                "isolated_green_ampt_with_explicit_depression_storage".to_string(),
            ),
            (
                "closed_wb14_semantics".to_string(),
                "hourly_mass_authority_normalized".to_string(),
            ),
            (
                "closing_surface_semantics".to_string(),
                "wb14_plus_saturation_return".to_string(),
            ),
            (
                "saturation_timing".to_string(),
                "hourly_zero_order_hold".to_string(),
            ),
            (
                "power_equivalent_semantics".to_string(),
                "null_no_erosion_adoption".to_string(),
            ),
        ]),
    ))
}

pub struct HillslopeWatSubhourlyParquetRowGroupWriter {
    writer: Option<ArrowWriter<File>>,
    schema: Arc<Schema>,
    target_path: PathBuf,
    temporary_path: PathBuf,
    rows_written: usize,
    pending_hour: Option<Wat5HourValidation>,
}

#[derive(Clone, Copy)]
struct Wat5HourValidation {
    identity: (i32, i32, i32, i32),
    last_subinterval_index: i32,
    observed_closing_depth_mm: f64,
    authoritative_depth_mm: f64,
    reported_residual_mm: f64,
}

impl HillslopeWatSubhourlyParquetRowGroupWriter {
    pub fn create(path: &Path) -> Result<Self, HillslopeWatSubhourlyError> {
        let schema = Arc::new(hillslope_wat_subhourly_schema()?);
        if path.exists() {
            return Err(HillslopeWatSubhourlyError::Io {
                path: path.to_path_buf(),
                source: io::Error::new(
                    io::ErrorKind::AlreadyExists,
                    "WAT5 output target already exists",
                ),
            });
        }
        let temporary_path = path.with_extension(format!("parquet.tmp.{}", std::process::id()));
        let file = OpenOptions::new()
            .write(true)
            .create_new(true)
            .open(&temporary_path)
            .map_err(|source| HillslopeWatSubhourlyError::Io {
                path: temporary_path.clone(),
                source,
            })?;
        let properties = WriterProperties::builder()
            .set_compression(Compression::SNAPPY)
            .build();
        let writer = ArrowWriter::try_new_with_options(
            file,
            Arc::clone(&schema),
            ArrowWriterOptions::new().with_properties(properties),
        );
        let writer = match writer {
            Ok(writer) => writer,
            Err(error) => {
                let _ = std::fs::remove_file(&temporary_path);
                return Err(HillslopeWatSubhourlyError::Parquet {
                    detail: error.to_string(),
                });
            }
        };
        Ok(Self {
            writer: Some(writer),
            schema,
            target_path: path.to_path_buf(),
            temporary_path,
            rows_written: 0,
            pending_hour: None,
        })
    }

    pub fn write_rows(
        &mut self,
        rows: &[HillslopeWatSubhourlyRow],
    ) -> Result<(), HillslopeWatSubhourlyError> {
        if rows.is_empty() {
            return Ok(());
        }
        self.validate_rows(rows)?;
        let batch = rows_to_batch(self.schema.as_ref(), rows)?;
        self.writer
            .as_mut()
            .ok_or_else(|| HillslopeWatSubhourlyError::Parquet {
                detail: "WAT5 writer is already closed".to_string(),
            })?
            .write(&batch)
            .map_err(|error| HillslopeWatSubhourlyError::Parquet {
                detail: error.to_string(),
            })?;
        self.rows_written = self.rows_written.checked_add(rows.len()).ok_or_else(|| {
            HillslopeWatSubhourlyError::Parquet {
                detail: "WAT5 row count overflow".to_string(),
            }
        })?;
        Ok(())
    }

    pub fn close(mut self) -> Result<WriteSummary, HillslopeWatSubhourlyError> {
        self.validate_pending_hour()?;
        let writer = self
            .writer
            .take()
            .ok_or_else(|| HillslopeWatSubhourlyError::Parquet {
                detail: "WAT5 writer is already closed".to_string(),
            })?;
        let _closed_file_metadata =
            writer
                .close()
                .map_err(|error| HillslopeWatSubhourlyError::Parquet {
                    detail: error.to_string(),
                })?;
        let completed_file =
            File::open(&self.temporary_path).map_err(|source| HillslopeWatSubhourlyError::Io {
                path: self.temporary_path.clone(),
                source,
            })?;
        let completed =
            ParquetRecordBatchReaderBuilder::try_new(completed_file).map_err(|error| {
                HillslopeWatSubhourlyError::Parquet {
                    detail: format!("completed WAT5 metadata validation failed: {error}"),
                }
            })?;
        let observed_rows = usize::try_from(completed.metadata().file_metadata().num_rows())
            .map_err(|_| HillslopeWatSubhourlyError::Parquet {
                detail: "completed WAT5 row count exceeds usize".to_string(),
            })?;
        if observed_rows != self.rows_written {
            return Err(HillslopeWatSubhourlyError::Parquet {
                detail: format!(
                    "completed WAT5 row count mismatch: expected {}, observed {observed_rows}",
                    self.rows_written
                ),
            });
        }
        if completed.schema().as_ref() != self.schema.as_ref() {
            return Err(HillslopeWatSubhourlyError::Parquet {
                detail: "completed WAT5 schema or required metadata mismatch".to_string(),
            });
        }
        // A same-directory hard link publishes the completed inode atomically
        // and, unlike `rename` on Unix, cannot replace a target that appeared
        // after `create`. Removing the temporary name leaves the target as the
        // sole link to the completed file.
        std::fs::hard_link(&self.temporary_path, &self.target_path).map_err(|source| {
            HillslopeWatSubhourlyError::Io {
                path: self.target_path.clone(),
                source,
            }
        })?;
        std::fs::remove_file(&self.temporary_path).map_err(|source| {
            HillslopeWatSubhourlyError::Io {
                path: self.temporary_path.clone(),
                source,
            }
        })?;
        Ok(WriteSummary {
            rows_written: self.rows_written,
        })
    }
}

impl HillslopeWatSubhourlyParquetRowGroupWriter {
    fn validate_rows(
        &mut self,
        rows: &[HillslopeWatSubhourlyRow],
    ) -> Result<(), HillslopeWatSubhourlyError> {
        for row in rows {
            validate_row(row)?;
            let identity = (row.wepp_id, row.ofe_id, row.sim_day_index, row.hour_index);
            if self
                .pending_hour
                .is_some_and(|pending| pending.identity != identity)
            {
                self.validate_pending_hour()?;
            }
            let pending = self.pending_hour.get_or_insert(Wat5HourValidation {
                identity,
                last_subinterval_index: row.subinterval_index - 1,
                observed_closing_depth_mm: 0.0,
                authoritative_depth_mm: row.hourly_authoritative_runoff_depth_mm,
                reported_residual_mm: row.hourly_closure_residual_mm,
            });
            if row.subinterval_index <= pending.last_subinterval_index {
                return Err(wat5_validation_error(
                    "rows are not strictly ordered within an OFE/day/hour",
                ));
            }
            if !approximately_equal(
                row.hourly_authoritative_runoff_depth_mm,
                pending.authoritative_depth_mm,
            ) || !approximately_equal(
                row.hourly_closure_residual_mm,
                pending.reported_residual_mm,
            ) {
                return Err(wat5_validation_error(
                    "hourly authority or residual changes within an hour",
                ));
            }
            pending.last_subinterval_index = row.subinterval_index;
            pending.observed_closing_depth_mm += row.closing_surface_generation_depth_mm;
        }
        Ok(())
    }

    fn validate_pending_hour(&mut self) -> Result<(), HillslopeWatSubhourlyError> {
        let Some(pending) = self.pending_hour.take() else {
            return Ok(());
        };
        let reconstructed_residual_mm =
            pending.observed_closing_depth_mm - pending.authoritative_depth_mm;
        if !approximately_equal(reconstructed_residual_mm, pending.reported_residual_mm)
            || reconstructed_residual_mm.abs()
                > WAT5_PUBLICATION_TOLERANCE_MM * pending.authoritative_depth_mm.abs().max(1.0)
        {
            return Err(wat5_validation_error(
                "emitted rows do not close to their hourly authority",
            ));
        }
        Ok(())
    }
}

fn wat5_validation_error(detail: impl Into<String>) -> HillslopeWatSubhourlyError {
    HillslopeWatSubhourlyError::Parquet {
        detail: format!("WAT5-E-003 invalid public row: {}", detail.into()),
    }
}

fn approximately_equal(left: f64, right: f64) -> bool {
    (left - right).abs() <= WAT5_PUBLICATION_TOLERANCE_MM * left.abs().max(right.abs()).max(1.0)
}

fn require_finite_nonnegative(
    value: f64,
    field: &'static str,
) -> Result<(), HillslopeWatSubhourlyError> {
    if !value.is_finite() || value < 0.0 {
        return Err(wat5_validation_error(format!(
            "{field} must be finite and nonnegative"
        )));
    }
    Ok(())
}

#[allow(clippy::too_many_lines)]
fn validate_row(row: &HillslopeWatSubhourlyRow) -> Result<(), HillslopeWatSubhourlyError> {
    if row.wepp_id <= 0
        || row.ofe_id <= 0
        || row.sim_day_index < 0
        || !(1..=366).contains(&row.julian)
        || row.event_ordinal != 0
        || !(0..24).contains(&row.hour_index)
        || !(0..WAT5_INTERVALS_PER_DAY).contains(&row.subinterval_index)
        || row.hour_index != row.subinterval_index / WAT5_INTERVALS_PER_HOUR
    {
        return Err(wat5_validation_error("invalid identity or clock key"));
    }
    let expected_start_s = f64::from(row.subinterval_index) * WAT5_INTERVAL_SECONDS;
    if !approximately_equal(row.interval_start_s, expected_start_s)
        || !approximately_equal(row.interval_duration_s, WAT5_INTERVAL_SECONDS)
    {
        return Err(wat5_validation_error("invalid interval start or duration"));
    }
    for (value, field) in [
        (row.rainfall_depth_mm, "rainfall_depth_mm"),
        (row.additional_supply_depth_mm, "additional_supply_depth_mm"),
        (
            row.raw_green_ampt_infiltration_depth_mm,
            "raw_green_ampt_infiltration_depth_mm",
        ),
        (
            row.depression_storage_retention_depth_mm,
            "depression_storage_retention_depth_mm",
        ),
        (
            row.raw_wb14_post_depression_generation_depth_mm,
            "raw_wb14_post_depression_generation_depth_mm",
        ),
        (
            row.closed_wb14_generation_depth_mm,
            "closed_wb14_generation_depth_mm",
        ),
        (row.saturation_return_depth_mm, "saturation_return_depth_mm"),
        (
            row.closing_surface_generation_depth_mm,
            "closing_surface_generation_depth_mm",
        ),
        (
            row.closing_surface_generation_intensity_mm_h,
            "closing_surface_generation_intensity_mm_h",
        ),
        (
            row.hourly_authoritative_runoff_depth_mm,
            "hourly_authoritative_runoff_depth_mm",
        ),
        (
            row.hourly_mean_generation_intensity_mm_h,
            "hourly_mean_generation_intensity_mm_h",
        ),
    ] {
        require_finite_nonnegative(value, field)?;
    }
    if row.additional_supply_depth_mm != 0.0
        || !row.hourly_closure_residual_mm.is_finite()
        || row.method_code.is_empty()
        || row.source_completeness_code.is_empty()
    {
        return Err(wat5_validation_error(
            "unsupported supply, non-finite residual, or empty method/source code",
        ));
    }
    for (value, field) in [
        (
            row.hourly_power_equivalent_generation_intensity_mm_h,
            "hourly_power_equivalent_generation_intensity_mm_h",
        ),
        (
            row.hourly_power_equivalent_duration_s,
            "hourly_power_equivalent_duration_s",
        ),
        (row.power_exponent, "power_exponent"),
    ] {
        if let Some(value) = value {
            require_finite_nonnegative(value, field)?;
        }
    }
    let raw_accounted_mm = row.raw_green_ampt_infiltration_depth_mm
        + row.depression_storage_retention_depth_mm
        + row.raw_wb14_post_depression_generation_depth_mm;
    if !approximately_equal(row.rainfall_depth_mm, raw_accounted_mm)
        || !approximately_equal(
            row.closing_surface_generation_depth_mm,
            row.closed_wb14_generation_depth_mm + row.saturation_return_depth_mm,
        )
        || !approximately_equal(
            row.closing_surface_generation_intensity_mm_h,
            row.closing_surface_generation_depth_mm * 12.0,
        )
        || !approximately_equal(
            row.hourly_mean_generation_intensity_mm_h,
            row.hourly_authoritative_runoff_depth_mm,
        )
    {
        return Err(wat5_validation_error("row closure or rate identity failed"));
    }
    Ok(())
}

impl Drop for HillslopeWatSubhourlyParquetRowGroupWriter {
    fn drop(&mut self) {
        if self.temporary_path.exists() {
            let _ = std::fs::remove_file(&self.temporary_path);
        }
    }
}

fn rows_to_batch(
    schema: &Schema,
    rows: &[HillslopeWatSubhourlyRow],
) -> Result<RecordBatch, HillslopeWatSubhourlyError> {
    macro_rules! ints {
        ($field:ident) => {
            Arc::new(Int32Array::from(
                rows.iter().map(|row| row.$field).collect::<Vec<_>>(),
            )) as ArrayRef
        };
    }
    macro_rules! floats {
        ($field:ident) => {
            Arc::new(Float64Array::from(
                rows.iter().map(|row| row.$field).collect::<Vec<_>>(),
            )) as ArrayRef
        };
    }
    macro_rules! optional_floats {
        ($field:ident) => {
            Arc::new(Float64Array::from(
                rows.iter().map(|row| row.$field).collect::<Vec<_>>(),
            )) as ArrayRef
        };
    }
    let columns: Vec<ArrayRef> = vec![
        ints!(wepp_id),
        ints!(ofe_id),
        ints!(year),
        ints!(sim_day_index),
        ints!(julian),
        ints!(event_ordinal),
        ints!(hour_index),
        ints!(subinterval_index),
        floats!(interval_start_s),
        floats!(interval_duration_s),
        floats!(rainfall_depth_mm),
        floats!(additional_supply_depth_mm),
        floats!(raw_green_ampt_infiltration_depth_mm),
        floats!(depression_storage_retention_depth_mm),
        floats!(raw_wb14_post_depression_generation_depth_mm),
        floats!(closed_wb14_generation_depth_mm),
        floats!(saturation_return_depth_mm),
        floats!(closing_surface_generation_depth_mm),
        floats!(closing_surface_generation_intensity_mm_h),
        floats!(hourly_authoritative_runoff_depth_mm),
        floats!(hourly_mean_generation_intensity_mm_h),
        optional_floats!(hourly_power_equivalent_generation_intensity_mm_h),
        optional_floats!(hourly_power_equivalent_duration_s),
        optional_floats!(power_exponent),
        Arc::new(StringArray::from(
            rows.iter()
                .map(|row| row.method_code.as_str())
                .collect::<Vec<_>>(),
        )),
        Arc::new(StringArray::from(
            rows.iter()
                .map(|row| row.source_completeness_code.as_str())
                .collect::<Vec<_>>(),
        )),
        floats!(hourly_closure_residual_mm),
    ];
    RecordBatch::try_new(Arc::new(schema.clone()), columns).map_err(|error| {
        HillslopeWatSubhourlyError::Parquet {
            detail: error.to_string(),
        }
    })
}

#[cfg(test)]
mod tests {
    use arrow_array::Array;
    use parquet::arrow::arrow_reader::ParquetRecordBatchReaderBuilder;

    use super::*;

    #[test]
    fn writer_atomically_emits_schema_rows_and_null_candidate_columns() {
        let nonce = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .expect("test clock")
            .as_nanos();
        let path = std::env::temp_dir().join(format!("openwepp-wat5-{nonce}.parquet"));
        let temporary_path = path.with_extension(format!("parquet.tmp.{}", std::process::id()));
        let row = HillslopeWatSubhourlyRow {
            wepp_id: 1,
            ofe_id: 1,
            year: 2026,
            sim_day_index: 1,
            julian: 1,
            event_ordinal: 0,
            hour_index: 0,
            subinterval_index: 0,
            interval_start_s: 0.0,
            interval_duration_s: 300.0,
            rainfall_depth_mm: 2.0,
            additional_supply_depth_mm: 0.0,
            raw_green_ampt_infiltration_depth_mm: 0.5,
            depression_storage_retention_depth_mm: 0.0,
            raw_wb14_post_depression_generation_depth_mm: 1.5,
            closed_wb14_generation_depth_mm: 1.5,
            saturation_return_depth_mm: 0.0,
            closing_surface_generation_depth_mm: 1.5,
            closing_surface_generation_intensity_mm_h: 18.0,
            hourly_authoritative_runoff_depth_mm: 1.5,
            hourly_mean_generation_intensity_mm_h: 1.5,
            hourly_power_equivalent_generation_intensity_mm_h: None,
            hourly_power_equivalent_duration_s: None,
            power_exponent: None,
            method_code: "water_only_no_erosion_adoption".to_string(),
            source_completeness_code: "rainfall_complete".to_string(),
            hourly_closure_residual_mm: 0.0,
        };
        let mut writer =
            HillslopeWatSubhourlyParquetRowGroupWriter::create(&path).expect("create WAT5 writer");
        writer.write_rows(&[row]).expect("write WAT5 row");
        let summary = writer.close().expect("close WAT5 writer");
        assert_eq!(summary.rows_written, 1);
        assert!(path.is_file());
        assert!(!temporary_path.exists());

        let file = File::open(&path).expect("open WAT5 parquet");
        let builder = ParquetRecordBatchReaderBuilder::try_new(file).expect("read WAT5 metadata");
        assert_eq!(
            builder
                .schema()
                .metadata()
                .get("dataset_id")
                .map(String::as_str),
            Some(HILLSLOPE_WAT_SUBHOURLY_SCHEMA_ID)
        );
        let mut reader = builder.build().expect("build WAT5 reader");
        let batch = reader.next().expect("one WAT5 batch").expect("valid batch");
        assert_eq!(batch.num_rows(), 1);
        for column in [21, 22, 23] {
            assert_eq!(batch.column(column).null_count(), 1);
        }
        std::fs::remove_file(&path).expect("remove WAT5 test output");
    }

    #[test]
    fn writer_preserves_an_existing_target() {
        let nonce = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .expect("test clock")
            .as_nanos();
        let path = std::env::temp_dir().join(format!("openwepp-wat5-existing-{nonce}.parquet"));
        std::fs::write(&path, b"existing").expect("seed existing target");
        let error = HillslopeWatSubhourlyParquetRowGroupWriter::create(&path)
            .err()
            .expect("existing target must fail");
        assert_eq!(error.code(), "OHOUT-WAT5-E-001");
        assert!(error.to_string().contains("WAT5-E-005"));
        assert!(error.to_string().contains("already exists"));
        assert_eq!(
            std::fs::read(&path).expect("read preserved target"),
            b"existing"
        );
        std::fs::remove_file(&path).expect("remove existing-target fixture");
    }

    #[test]
    fn wat5_publication_errors_have_stable_codes_and_contract_binding() {
        let errors = [
            HillslopeWatSubhourlyError::Parquet {
                detail: "probe".to_string(),
            },
            HillslopeWatSubhourlyError::UnitMetadata {
                detail: "probe".to_string(),
            },
        ];
        assert_eq!(errors[0].code(), "OHOUT-WAT5-E-002");
        assert_eq!(errors[1].code(), "OHOUT-WAT5-E-003");
        assert!(
            errors
                .iter()
                .all(|error| error.to_string().contains("WAT5-E-005"))
        );
    }

    #[test]
    fn writer_validates_completed_row_count_before_publication() {
        let nonce = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .expect("test clock")
            .as_nanos();
        let path = std::env::temp_dir().join(format!("openwepp-wat5-count-{nonce}.parquet"));
        let temporary_path = path.with_extension(format!("parquet.tmp.{}", std::process::id()));
        let mut writer =
            HillslopeWatSubhourlyParquetRowGroupWriter::create(&path).expect("create WAT5 writer");
        writer.rows_written = 1;
        let error = writer
            .close()
            .expect_err("mismatched completed count must fail");
        assert_eq!(error.code(), "OHOUT-WAT5-E-002");
        assert!(error.to_string().contains("row count mismatch"));
        assert!(!path.exists());
        assert!(!temporary_path.exists());
    }

    #[test]
    fn writer_rejects_invalid_public_rows_before_publication() {
        let nonce = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .expect("test clock")
            .as_nanos();
        let path = std::env::temp_dir().join(format!("openwepp-wat5-invalid-{nonce}.parquet"));
        let mut row = HillslopeWatSubhourlyRow {
            wepp_id: 1,
            ofe_id: 1,
            year: 2026,
            sim_day_index: 1,
            julian: 1,
            event_ordinal: 0,
            hour_index: 0,
            subinterval_index: 0,
            interval_start_s: 0.0,
            interval_duration_s: 300.0,
            rainfall_depth_mm: 2.0,
            additional_supply_depth_mm: 0.0,
            raw_green_ampt_infiltration_depth_mm: 0.5,
            depression_storage_retention_depth_mm: 0.0,
            raw_wb14_post_depression_generation_depth_mm: 1.5,
            closed_wb14_generation_depth_mm: 1.5,
            saturation_return_depth_mm: 0.0,
            closing_surface_generation_depth_mm: 1.5,
            closing_surface_generation_intensity_mm_h: 18.0,
            hourly_authoritative_runoff_depth_mm: 1.5,
            hourly_mean_generation_intensity_mm_h: 1.5,
            hourly_power_equivalent_generation_intensity_mm_h: None,
            hourly_power_equivalent_duration_s: None,
            power_exponent: None,
            method_code: "water_only_no_erosion_adoption".to_string(),
            source_completeness_code: "rainfall_complete".to_string(),
            hourly_closure_residual_mm: 0.0,
        };
        row.depression_storage_retention_depth_mm = f64::NAN;
        let mut writer =
            HillslopeWatSubhourlyParquetRowGroupWriter::create(&path).expect("create WAT5 writer");
        let error = writer
            .write_rows(&[row])
            .expect_err("invalid row must fail at writer boundary");
        assert!(error.to_string().contains("WAT5-E-003"));
        drop(writer);
        assert!(!path.exists());
    }
}
