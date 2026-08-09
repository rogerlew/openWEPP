use std::collections::HashMap;
use std::fmt;
use std::fs::File;
use std::io;
use std::path::{Path, PathBuf};
use std::sync::Arc;

use arrow_array::{ArrayRef, Float64Array, Int8Array, Int16Array, Int32Array, RecordBatch};
use arrow_schema::{DataType, Field, Schema};
use openwepp_sim_contract::units::{OutputUnitRegistryError, validate_output_schema_unit};
use parquet::arrow::{ArrowWriter, arrow_writer::ArrowWriterOptions};
use parquet::basic::Compression;
use parquet::file::properties::WriterProperties;

use crate::hillslope_wat::{InterchangeVersion, stable_arrow_schema_file_metadata};

#[derive(Debug, Clone, PartialEq)]
pub struct HillslopePassRow {
    pub wepp_id: i32,
    pub year: i16,
    pub sim_day_index: i32,
    pub julian: i16,
    pub month: i8,
    pub day_of_month: i8,
    pub water_year: i16,
    pub runvol_m3: f64,
    pub sbrunv_m3: f64,
    pub peakro_m3_s: f64,
    pub total_detachment_kg: f64,
    pub total_deposition_kg: f64,
    pub sediment_concentration_kg_m3: [f64; 5],
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct WriteSummary {
    pub rows_written: usize,
}

#[derive(Debug)]
pub enum HillslopePassParquetError {
    Io { path: PathBuf, source: io::Error },
    Parquet { detail: String },
    UnitMetadata { detail: String },
}

impl HillslopePassParquetError {
    #[must_use]
    pub const fn code(&self) -> &'static str {
        match self {
            Self::Io { .. } => "OHOUT-PASS-E-001",
            Self::Parquet { .. } => "OHOUT-PASS-E-002",
            Self::UnitMetadata { .. } => "OHOUT-PASS-E-003",
        }
    }

    fn parquet(detail: impl Into<String>) -> Self {
        Self::Parquet {
            detail: detail.into(),
        }
    }
}

impl fmt::Display for HillslopePassParquetError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Io { path, source } => {
                write!(
                    f,
                    "{} io error at {}: {source}",
                    self.code(),
                    path.display()
                )
            }
            Self::Parquet { detail } => write!(f, "{} parquet error: {detail}", self.code()),
            Self::UnitMetadata { detail } => {
                write!(f, "{} unit metadata error: {detail}", self.code())
            }
        }
    }
}

impl std::error::Error for HillslopePassParquetError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::Io { source, .. } => Some(source),
            Self::Parquet { .. } | Self::UnitMetadata { .. } => None,
        }
    }
}

#[allow(clippy::too_many_lines)]
pub fn hillslope_pass_schema(
    version: InterchangeVersion,
) -> Result<Schema, HillslopePassParquetError> {
    let mut metadata = HashMap::new();
    metadata.insert("dataset_version".to_string(), version.dataset_version());
    metadata.insert(
        "dataset_version_major".to_string(),
        version.major.to_string(),
    );
    metadata.insert(
        "dataset_version_minor".to_string(),
        version.minor.to_string(),
    );
    metadata.insert("schema_version".to_string(), version.major.to_string());
    metadata.insert(
        "producer".to_string(),
        "openwepp-runoff-delivery".to_string(),
    );

    let schema = Schema::new_with_metadata(
        vec![
            Field::new("wepp_id", DataType::Int32, false),
            Field::new("year", DataType::Int16, false),
            field_with_meta(
                "sim_day_index",
                DataType::Int32,
                false,
                None,
                Some("1-indexed simulation day"),
            ),
            Field::new("julian", DataType::Int16, false),
            Field::new("month", DataType::Int8, false),
            Field::new("day_of_month", DataType::Int8, false),
            Field::new("water_year", DataType::Int16, false),
            field_with_meta(
                "runvol",
                DataType::Float64,
                false,
                Some("m^3"),
                Some("Outlet routed surface runoff delivery volume"),
            ),
            field_with_meta(
                "sbrunv",
                DataType::Float64,
                false,
                Some("m^3"),
                Some("Outlet lateral/subsurface delivery volume"),
            ),
            field_with_meta(
                "peakro",
                DataType::Float64,
                false,
                Some("m^3/s"),
                Some("Maximum hourly mean runoff flow"),
            ),
            field_with_meta(
                "tdet",
                DataType::Float64,
                false,
                Some("kg"),
                Some("Total detachment diagnostic"),
            ),
            field_with_meta(
                "tdep",
                DataType::Float64,
                false,
                Some("kg"),
                Some("Total deposition diagnostic"),
            ),
            field_with_meta(
                "sedcon_1",
                DataType::Float64,
                false,
                Some("kg/m^3"),
                Some("Sediment concentration class 1"),
            ),
            field_with_meta(
                "sedcon_2",
                DataType::Float64,
                false,
                Some("kg/m^3"),
                Some("Sediment concentration class 2"),
            ),
            field_with_meta(
                "sedcon_3",
                DataType::Float64,
                false,
                Some("kg/m^3"),
                Some("Sediment concentration class 3"),
            ),
            field_with_meta(
                "sedcon_4",
                DataType::Float64,
                false,
                Some("kg/m^3"),
                Some("Sediment concentration class 4"),
            ),
            field_with_meta(
                "sedcon_5",
                DataType::Float64,
                false,
                Some("kg/m^3"),
                Some("Sediment concentration class 5"),
            ),
        ],
        metadata,
    );
    align_output_schema_units("hillslope_pass", &schema)
}

pub fn write_hillslope_pass_parquet(
    path: &Path,
    rows: &[HillslopePassRow],
    version: InterchangeVersion,
) -> Result<WriteSummary, HillslopePassParquetError> {
    let mut writer = HillslopePassParquetRowGroupWriter::create(path, version)?;
    writer.write_rows(rows)?;
    writer.close()
}

pub struct HillslopePassParquetRowGroupWriter {
    writer: ArrowWriter<File>,
    schema: Arc<Schema>,
    rows_written: usize,
}

impl HillslopePassParquetRowGroupWriter {
    pub fn create(
        path: &Path,
        version: InterchangeVersion,
    ) -> Result<Self, HillslopePassParquetError> {
        let schema = Arc::new(hillslope_pass_schema(version)?);
        let file = File::create(path).map_err(|source| HillslopePassParquetError::Io {
            path: path.to_path_buf(),
            source,
        })?;
        let writer_properties = WriterProperties::builder()
            .set_compression(Compression::SNAPPY)
            .set_key_value_metadata(Some(
                stable_arrow_schema_file_metadata(&schema)
                    .map_err(|error| HillslopePassParquetError::parquet(error.to_string()))?,
            ))
            .build();
        let writer_options = ArrowWriterOptions::new()
            .with_properties(writer_properties)
            .with_skip_arrow_metadata(true);
        let writer = ArrowWriter::try_new_with_options(file, Arc::clone(&schema), writer_options)
            .map_err(|error| HillslopePassParquetError::parquet(error.to_string()))?;
        Ok(Self {
            writer,
            schema,
            rows_written: 0,
        })
    }

    pub fn write_rows(
        &mut self,
        rows: &[HillslopePassRow],
    ) -> Result<(), HillslopePassParquetError> {
        if rows.is_empty() {
            return Ok(());
        }
        let batch = hillslope_pass_rows_to_batch(self.schema.as_ref(), rows)?;
        self.writer
            .write(&batch)
            .map_err(|error| HillslopePassParquetError::parquet(error.to_string()))?;
        self.rows_written = self.rows_written.checked_add(rows.len()).ok_or_else(|| {
            HillslopePassParquetError::parquet(
                "hillslope PASS row count overflow while writing parquet",
            )
        })?;
        Ok(())
    }

    pub fn close(self) -> Result<WriteSummary, HillslopePassParquetError> {
        self.writer
            .close()
            .map_err(|error| HillslopePassParquetError::parquet(error.to_string()))?;
        Ok(WriteSummary {
            rows_written: self.rows_written,
        })
    }
}

fn output_registry_error(error: &OutputUnitRegistryError) -> HillslopePassParquetError {
    HillslopePassParquetError::UnitMetadata {
        detail: error.to_string(),
    }
}

fn align_output_schema_units(
    schema_id: &'static str,
    schema: &Schema,
) -> Result<Schema, HillslopePassParquetError> {
    let mut fields = Vec::with_capacity(schema.fields().len());

    for field_ref in schema.fields() {
        let field = field_ref.as_ref();
        let Some(local_unit) = field.metadata().get("units") else {
            fields.push(field.clone());
            continue;
        };
        let registry_unit = validate_output_schema_unit(schema_id, field.name(), local_unit)
            .map_err(|error| output_registry_error(&error))?;
        let mut metadata = field.metadata().clone();
        metadata.insert("units".to_string(), registry_unit.to_string());
        fields.push(field.clone().with_metadata(metadata));
    }

    Ok(Schema::new_with_metadata(fields, schema.metadata().clone()))
}

fn field_with_meta(
    name: &str,
    data_type: DataType,
    nullable: bool,
    units: Option<&str>,
    description: Option<&str>,
) -> Field {
    let mut field = Field::new(name, data_type, nullable);

    let mut metadata = HashMap::new();
    if let Some(units) = units {
        metadata.insert("units".to_string(), units.to_string());
    }
    if let Some(description) = description {
        metadata.insert("description".to_string(), description.to_string());
    }

    if !metadata.is_empty() {
        field = field.with_metadata(metadata);
    }

    field
}

fn hillslope_pass_rows_to_batch(
    schema: &Schema,
    rows: &[HillslopePassRow],
) -> Result<RecordBatch, HillslopePassParquetError> {
    let mut wepp_id = Vec::with_capacity(rows.len());
    let mut year = Vec::with_capacity(rows.len());
    let mut sim_day_index = Vec::with_capacity(rows.len());
    let mut julian = Vec::with_capacity(rows.len());
    let mut month = Vec::with_capacity(rows.len());
    let mut day_of_month = Vec::with_capacity(rows.len());
    let mut water_year = Vec::with_capacity(rows.len());
    let mut runvol = Vec::with_capacity(rows.len());
    let mut sbrunv = Vec::with_capacity(rows.len());
    let mut peakro = Vec::with_capacity(rows.len());
    let mut total_detachment = Vec::with_capacity(rows.len());
    let mut total_deposition = Vec::with_capacity(rows.len());
    let mut sedcon_1 = Vec::with_capacity(rows.len());
    let mut sedcon_2 = Vec::with_capacity(rows.len());
    let mut sedcon_3 = Vec::with_capacity(rows.len());
    let mut sedcon_4 = Vec::with_capacity(rows.len());
    let mut sedcon_5 = Vec::with_capacity(rows.len());

    for row in rows {
        wepp_id.push(row.wepp_id);
        year.push(row.year);
        sim_day_index.push(row.sim_day_index);
        julian.push(row.julian);
        month.push(row.month);
        day_of_month.push(row.day_of_month);
        water_year.push(row.water_year);
        runvol.push(row.runvol_m3);
        sbrunv.push(row.sbrunv_m3);
        peakro.push(row.peakro_m3_s);
        total_detachment.push(row.total_detachment_kg);
        total_deposition.push(row.total_deposition_kg);
        sedcon_1.push(row.sediment_concentration_kg_m3[0]);
        sedcon_2.push(row.sediment_concentration_kg_m3[1]);
        sedcon_3.push(row.sediment_concentration_kg_m3[2]);
        sedcon_4.push(row.sediment_concentration_kg_m3[3]);
        sedcon_5.push(row.sediment_concentration_kg_m3[4]);
    }

    let columns: Vec<ArrayRef> = vec![
        Arc::new(Int32Array::from(wepp_id)),
        Arc::new(Int16Array::from(year)),
        Arc::new(Int32Array::from(sim_day_index)),
        Arc::new(Int16Array::from(julian)),
        Arc::new(Int8Array::from(month)),
        Arc::new(Int8Array::from(day_of_month)),
        Arc::new(Int16Array::from(water_year)),
        Arc::new(Float64Array::from(runvol)),
        Arc::new(Float64Array::from(sbrunv)),
        Arc::new(Float64Array::from(peakro)),
        Arc::new(Float64Array::from(total_detachment)),
        Arc::new(Float64Array::from(total_deposition)),
        Arc::new(Float64Array::from(sedcon_1)),
        Arc::new(Float64Array::from(sedcon_2)),
        Arc::new(Float64Array::from(sedcon_3)),
        Arc::new(Float64Array::from(sedcon_4)),
        Arc::new(Float64Array::from(sedcon_5)),
    ];

    RecordBatch::try_new(Arc::new(schema.clone()), columns)
        .map_err(|error| HillslopePassParquetError::parquet(error.to_string()))
}

#[cfg(test)]
mod tests {
    use std::fs;

    use parquet::arrow::arrow_reader::ParquetRecordBatchReaderBuilder;

    use super::*;

    fn sample_row() -> HillslopePassRow {
        HillslopePassRow {
            wepp_id: 5,
            year: 2024,
            sim_day_index: 17,
            julian: 42,
            month: 2,
            day_of_month: 11,
            water_year: 2024,
            runvol_m3: 12.5,
            sbrunv_m3: 0.75,
            peakro_m3_s: 0.125,
            total_detachment_kg: 1.25,
            total_deposition_kg: 0.25,
            sediment_concentration_kg_m3: [0.1, 0.2, 0.3, 0.4, 0.5],
        }
    }

    #[test]
    fn writer_emits_valid_parquet_file_with_schema_metadata() {
        let timestamp = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .expect("unix epoch should be before now")
            .as_nanos();
        let output_path = std::env::temp_dir().join(format!(
            "openwepp_hillslope_pass_writer_{timestamp}.parquet"
        ));

        let summary = write_hillslope_pass_parquet(
            &output_path,
            &[sample_row()],
            InterchangeVersion::default(),
        )
        .expect("writer should emit parquet");
        assert_eq!(summary.rows_written, 1);

        let file = File::open(&output_path).expect("parquet file should be readable");
        let builder = ParquetRecordBatchReaderBuilder::try_new(file)
            .expect("parquet should have readable footer/schema");
        let schema = builder.schema();

        assert!(schema.metadata().contains_key("dataset_version"));
        let runvol_field = schema
            .fields()
            .iter()
            .find(|field| field.name() == "runvol")
            .expect("runvol field should exist in parquet schema");
        assert_eq!(
            runvol_field.metadata().get("units").map(String::as_str),
            Some("m^3")
        );

        let _ = fs::remove_file(output_path);
    }

    #[test]
    fn writer_emits_byte_stable_parquet_for_identical_rows() {
        let timestamp = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .expect("unix epoch should be before now")
            .as_nanos();
        let output_path_a = std::env::temp_dir().join(format!(
            "openwepp_hillslope_pass_writer_stable_a_{timestamp}.parquet"
        ));
        let output_path_b = std::env::temp_dir().join(format!(
            "openwepp_hillslope_pass_writer_stable_b_{timestamp}.parquet"
        ));

        write_hillslope_pass_parquet(
            &output_path_a,
            &[sample_row()],
            InterchangeVersion::default(),
        )
        .expect("first writer should emit parquet");
        write_hillslope_pass_parquet(
            &output_path_b,
            &[sample_row()],
            InterchangeVersion::default(),
        )
        .expect("second writer should emit parquet");

        let first = fs::read(&output_path_a).expect("first parquet should be readable");
        let second = fs::read(&output_path_b).expect("second parquet should be readable");
        assert_eq!(first, second);

        let _ = fs::remove_file(output_path_a);
        let _ = fs::remove_file(output_path_b);
    }
}
