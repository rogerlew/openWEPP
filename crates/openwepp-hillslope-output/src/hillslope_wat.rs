use std::collections::HashMap;
use std::fmt;
use std::fs::File;
use std::io;
use std::path::{Path, PathBuf};
use std::sync::Arc;

use arrow_array::{ArrayRef, Float64Array, Int8Array, Int16Array, Int32Array, RecordBatch};
use arrow_schema::{DataType, Field, Schema};
use openwepp_sim_contract::units::{OutputUnitRegistryError, validate_output_schema_unit};
use parquet::arrow::ArrowWriter;
use parquet::basic::Compression;
use parquet::file::properties::WriterProperties;

/// Default interchange dataset version aligned to WEPPpy/WEPPpyo3.
pub const DEFAULT_DATASET_VERSION_MAJOR: u32 = 1;
pub const DEFAULT_DATASET_VERSION_MINOR: u32 = 2;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct InterchangeVersion {
    pub major: u32,
    pub minor: u32,
}

impl InterchangeVersion {
    #[must_use]
    pub const fn new(major: u32, minor: u32) -> Self {
        Self { major, minor }
    }

    #[must_use]
    pub fn dataset_version(self) -> String {
        format!("{}.{}", self.major, self.minor)
    }
}

impl Default for InterchangeVersion {
    fn default() -> Self {
        Self::new(DEFAULT_DATASET_VERSION_MAJOR, DEFAULT_DATASET_VERSION_MINOR)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct HillslopeWatRow {
    pub wepp_id: i32,
    pub ofe_id: i16,
    pub year: i16,
    pub sim_day_index: i32,
    pub julian: i16,
    pub month: i8,
    pub day_of_month: i8,
    pub water_year: i16,
    pub ofe: i16,
    pub p: f64,
    pub rm: f64,
    pub q: f64,
    pub ep: f64,
    pub es: f64,
    pub er: f64,
    pub dp: f64,
    pub up_strm_q: f64,
    pub sub_r_in: f64,
    pub latqcc: f64,
    pub total_soil_water: f64,
    pub frozwt: f64,
    pub snow_water: f64,
    pub qofe: f64,
    pub tile: f64,
    pub irr: f64,
    pub area: f64,
    pub soil_water_total: Option<f64>,
    pub profile_depth: Option<f64>,
    pub profile_porosity_cap: Option<f64>,
    pub profile_fc_store: Option<f64>,
    pub profile_wp_store: Option<f64>,
    pub interception_storage: Option<f64>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct WriteSummary {
    pub rows_written: usize,
}

#[derive(Debug)]
pub enum HillslopeWatParquetError {
    Io { path: PathBuf, source: io::Error },
    Parquet { detail: String },
    UnitMetadata { detail: String },
}

impl HillslopeWatParquetError {
    #[must_use]
    pub const fn code(&self) -> &'static str {
        match self {
            Self::Io { .. } => "OHOUT-WAT-E-001",
            Self::Parquet { .. } => "OHOUT-WAT-E-002",
            Self::UnitMetadata { .. } => "OHOUT-WAT-E-003",
        }
    }

    fn parquet(detail: impl Into<String>) -> Self {
        Self::Parquet {
            detail: detail.into(),
        }
    }
}

impl fmt::Display for HillslopeWatParquetError {
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

impl std::error::Error for HillslopeWatParquetError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::Io { source, .. } => Some(source),
            Self::Parquet { .. } | Self::UnitMetadata { .. } => None,
        }
    }
}

#[allow(clippy::too_many_lines)]
pub fn hillslope_wat_schema(
    version: InterchangeVersion,
) -> Result<Schema, HillslopeWatParquetError> {
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

    let schema = Schema::new_with_metadata(
        vec![
            Field::new("wepp_id", DataType::Int32, false),
            Field::new("ofe_id", DataType::Int16, false),
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
            Field::new("OFE", DataType::Int16, false),
            field_with_meta(
                "P",
                DataType::Float64,
                false,
                Some("mm"),
                Some("Precipitation"),
            ),
            field_with_meta(
                "RM",
                DataType::Float64,
                false,
                Some("mm"),
                Some("Rainfall+Irrigation+Snowmelt"),
            ),
            field_with_meta(
                "Q",
                DataType::Float64,
                false,
                Some("mm"),
                Some("Daily runoff over eff length"),
            ),
            field_with_meta(
                "Ep",
                DataType::Float64,
                false,
                Some("mm"),
                Some("Plant transpiration"),
            ),
            field_with_meta(
                "Es",
                DataType::Float64,
                false,
                Some("mm"),
                Some("Soil evaporation"),
            ),
            field_with_meta(
                "Er",
                DataType::Float64,
                false,
                Some("mm"),
                Some("Residue evaporation"),
            ),
            field_with_meta(
                "Dp",
                DataType::Float64,
                false,
                Some("mm"),
                Some("Deep percolation"),
            ),
            field_with_meta(
                "UpStrmQ",
                DataType::Float64,
                false,
                Some("mm"),
                Some("Runon added to OFE"),
            ),
            field_with_meta(
                "SubRIn",
                DataType::Float64,
                false,
                Some("mm"),
                Some("Subsurface runon added to OFE"),
            ),
            field_with_meta(
                "latqcc",
                DataType::Float64,
                false,
                Some("mm"),
                Some("Lateral subsurface flow"),
            ),
            field_with_meta(
                "Total-Soil",
                DataType::Float64,
                false,
                Some("mm"),
                Some("Unfrozen water in soil profile"),
            ),
            field_with_meta(
                "frozwt",
                DataType::Float64,
                false,
                Some("mm"),
                Some("Frozen water in soil profile"),
            ),
            field_with_meta(
                "Snow-Water",
                DataType::Float64,
                false,
                Some("mm"),
                Some("Water in surface snow"),
            ),
            field_with_meta(
                "QOFE",
                DataType::Float64,
                false,
                Some("mm"),
                Some("Daily runoff scaled to single OFE"),
            ),
            field_with_meta(
                "Tile",
                DataType::Float64,
                false,
                Some("mm"),
                Some("Tile drainage"),
            ),
            field_with_meta(
                "Irr",
                DataType::Float64,
                false,
                Some("mm"),
                Some("Irrigation"),
            ),
            field_with_meta(
                "Area",
                DataType::Float64,
                false,
                Some("m^2"),
                Some("Area that depths apply over"),
            ),
            field_with_meta(
                "SoilWaterTotal",
                DataType::Float64,
                true,
                Some("mm"),
                Some(
                    "Full-profile soil water depth (watcon + frozwt), optional producer-authoritative term",
                ),
            ),
            field_with_meta(
                "ProfileDepth",
                DataType::Float64,
                true,
                Some("mm"),
                Some("Full soil profile depth (solthk(nsl)), optional producer-authoritative term"),
            ),
            field_with_meta(
                "ProfilePorosityCap",
                DataType::Float64,
                true,
                Some("mm"),
                Some(
                    "Full-profile porosity storage capacity (sum(por * dg)), optional producer-authoritative term",
                ),
            ),
            field_with_meta(
                "ProfileFCStore",
                DataType::Float64,
                true,
                Some("mm"),
                Some(
                    "Full-profile field-capacity storage (sum(thetfc * dg)), optional producer-authoritative term",
                ),
            ),
            field_with_meta(
                "ProfileWPStore",
                DataType::Float64,
                true,
                Some("mm"),
                Some(
                    "Full-profile wilting-point storage (sum(thetdr * dg)), optional producer-authoritative term",
                ),
            ),
            field_with_meta(
                "InterceptionStorage",
                DataType::Float64,
                true,
                Some("mm"),
                Some(
                    "Plant/residue interception carryover storage (pintlv + resint), optional producer-authoritative term",
                ),
            ),
        ],
        metadata,
    );
    align_output_schema_units("hillslope_wat", &schema)
}

pub fn write_hillslope_wat_parquet(
    path: &Path,
    rows: &[HillslopeWatRow],
    version: InterchangeVersion,
) -> Result<WriteSummary, HillslopeWatParquetError> {
    let schema = hillslope_wat_schema(version)?;
    let batch = hillslope_wat_rows_to_batch(&schema, rows)?;

    let file = File::create(path).map_err(|source| HillslopeWatParquetError::Io {
        path: path.to_path_buf(),
        source,
    })?;

    let writer_properties = WriterProperties::builder()
        .set_compression(Compression::SNAPPY)
        .build();

    let mut writer = ArrowWriter::try_new(file, Arc::new(schema), Some(writer_properties))
        .map_err(|error| HillslopeWatParquetError::parquet(error.to_string()))?;
    writer
        .write(&batch)
        .map_err(|error| HillslopeWatParquetError::parquet(error.to_string()))?;
    writer
        .close()
        .map_err(|error| HillslopeWatParquetError::parquet(error.to_string()))?;

    Ok(WriteSummary {
        rows_written: rows.len(),
    })
}

fn output_registry_error(error: &OutputUnitRegistryError) -> HillslopeWatParquetError {
    HillslopeWatParquetError::UnitMetadata {
        detail: error.to_string(),
    }
}

fn align_output_schema_units(
    schema_id: &'static str,
    schema: &Schema,
) -> Result<Schema, HillslopeWatParquetError> {
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

#[allow(clippy::too_many_lines)]
fn hillslope_wat_rows_to_batch(
    schema: &Schema,
    rows: &[HillslopeWatRow],
) -> Result<RecordBatch, HillslopeWatParquetError> {
    let mut wepp_id = Vec::with_capacity(rows.len());
    let mut ofe_id = Vec::with_capacity(rows.len());
    let mut year = Vec::with_capacity(rows.len());
    let mut sim_day_index = Vec::with_capacity(rows.len());
    let mut julian = Vec::with_capacity(rows.len());
    let mut month = Vec::with_capacity(rows.len());
    let mut day_of_month = Vec::with_capacity(rows.len());
    let mut water_year = Vec::with_capacity(rows.len());
    let mut ofe = Vec::with_capacity(rows.len());
    let mut p = Vec::with_capacity(rows.len());
    let mut rm = Vec::with_capacity(rows.len());
    let mut q = Vec::with_capacity(rows.len());
    let mut ep = Vec::with_capacity(rows.len());
    let mut es = Vec::with_capacity(rows.len());
    let mut er = Vec::with_capacity(rows.len());
    let mut dp = Vec::with_capacity(rows.len());
    let mut up_strm_q = Vec::with_capacity(rows.len());
    let mut sub_r_in = Vec::with_capacity(rows.len());
    let mut latqcc = Vec::with_capacity(rows.len());
    let mut total_soil_water = Vec::with_capacity(rows.len());
    let mut frozwt = Vec::with_capacity(rows.len());
    let mut snow_water = Vec::with_capacity(rows.len());
    let mut qofe = Vec::with_capacity(rows.len());
    let mut tile = Vec::with_capacity(rows.len());
    let mut irr = Vec::with_capacity(rows.len());
    let mut area = Vec::with_capacity(rows.len());
    let mut soil_water_total = Vec::with_capacity(rows.len());
    let mut profile_depth = Vec::with_capacity(rows.len());
    let mut profile_porosity_cap = Vec::with_capacity(rows.len());
    let mut profile_fc_store = Vec::with_capacity(rows.len());
    let mut profile_wp_store = Vec::with_capacity(rows.len());
    let mut interception_storage = Vec::with_capacity(rows.len());

    for row in rows {
        wepp_id.push(row.wepp_id);
        ofe_id.push(row.ofe_id);
        year.push(row.year);
        sim_day_index.push(row.sim_day_index);
        julian.push(row.julian);
        month.push(row.month);
        day_of_month.push(row.day_of_month);
        water_year.push(row.water_year);
        ofe.push(row.ofe);
        p.push(row.p);
        rm.push(row.rm);
        q.push(row.q);
        ep.push(row.ep);
        es.push(row.es);
        er.push(row.er);
        dp.push(row.dp);
        up_strm_q.push(row.up_strm_q);
        sub_r_in.push(row.sub_r_in);
        latqcc.push(row.latqcc);
        total_soil_water.push(row.total_soil_water);
        frozwt.push(row.frozwt);
        snow_water.push(row.snow_water);
        qofe.push(row.qofe);
        tile.push(row.tile);
        irr.push(row.irr);
        area.push(row.area);
        soil_water_total.push(row.soil_water_total);
        profile_depth.push(row.profile_depth);
        profile_porosity_cap.push(row.profile_porosity_cap);
        profile_fc_store.push(row.profile_fc_store);
        profile_wp_store.push(row.profile_wp_store);
        interception_storage.push(row.interception_storage);
    }

    let columns: Vec<ArrayRef> = vec![
        Arc::new(Int32Array::from(wepp_id)),
        Arc::new(Int16Array::from(ofe_id)),
        Arc::new(Int16Array::from(year)),
        Arc::new(Int32Array::from(sim_day_index)),
        Arc::new(Int16Array::from(julian)),
        Arc::new(Int8Array::from(month)),
        Arc::new(Int8Array::from(day_of_month)),
        Arc::new(Int16Array::from(water_year)),
        Arc::new(Int16Array::from(ofe)),
        Arc::new(Float64Array::from(p)),
        Arc::new(Float64Array::from(rm)),
        Arc::new(Float64Array::from(q)),
        Arc::new(Float64Array::from(ep)),
        Arc::new(Float64Array::from(es)),
        Arc::new(Float64Array::from(er)),
        Arc::new(Float64Array::from(dp)),
        Arc::new(Float64Array::from(up_strm_q)),
        Arc::new(Float64Array::from(sub_r_in)),
        Arc::new(Float64Array::from(latqcc)),
        Arc::new(Float64Array::from(total_soil_water)),
        Arc::new(Float64Array::from(frozwt)),
        Arc::new(Float64Array::from(snow_water)),
        Arc::new(Float64Array::from(qofe)),
        Arc::new(Float64Array::from(tile)),
        Arc::new(Float64Array::from(irr)),
        Arc::new(Float64Array::from(area)),
        Arc::new(Float64Array::from(soil_water_total)),
        Arc::new(Float64Array::from(profile_depth)),
        Arc::new(Float64Array::from(profile_porosity_cap)),
        Arc::new(Float64Array::from(profile_fc_store)),
        Arc::new(Float64Array::from(profile_wp_store)),
        Arc::new(Float64Array::from(interception_storage)),
    ];

    RecordBatch::try_new(Arc::new(schema.clone()), columns)
        .map_err(|error| HillslopeWatParquetError::parquet(error.to_string()))
}

#[cfg(test)]
mod tests {
    use std::fs;

    use parquet::arrow::arrow_reader::ParquetRecordBatchReaderBuilder;

    use super::*;

    fn sample_row() -> HillslopeWatRow {
        HillslopeWatRow {
            wepp_id: 1,
            ofe_id: 1,
            year: 1987,
            sim_day_index: 1,
            julian: 1,
            month: 1,
            day_of_month: 1,
            water_year: 1987,
            ofe: 1,
            p: 10.0,
            rm: 0.0,
            q: 0.0,
            ep: 1.0,
            es: 1.0,
            er: 0.1,
            dp: 0.1,
            up_strm_q: 0.0,
            sub_r_in: 0.0,
            latqcc: 0.0,
            total_soil_water: 100.0,
            frozwt: 0.0,
            snow_water: 0.0,
            qofe: 0.0,
            tile: 0.0,
            irr: 0.0,
            area: 1.0,
            soil_water_total: Some(100.0),
            profile_depth: Some(1_200.0),
            profile_porosity_cap: Some(400.0),
            profile_fc_store: Some(300.0),
            profile_wp_store: Some(150.0),
            interception_storage: None,
        }
    }

    #[test]
    fn schema_includes_required_dataset_metadata_keys() {
        let schema = hillslope_wat_schema(InterchangeVersion::default())
            .expect("hillslope WAT schema should construct");
        let metadata = schema.metadata();

        for key in [
            "dataset_version",
            "dataset_version_major",
            "dataset_version_minor",
            "schema_version",
        ] {
            assert!(metadata.contains_key(key), "missing key: {key}");
        }
    }

    #[test]
    fn schema_includes_units_and_description_field_metadata() {
        let schema = hillslope_wat_schema(InterchangeVersion::default())
            .expect("hillslope WAT schema should construct");
        let p_field = schema
            .fields
            .iter()
            .find(|field| field.name() == "P")
            .expect("P field should exist");
        let p_meta = p_field.metadata();

        assert_eq!(p_meta.get("units").map(String::as_str), Some("mm"));
        assert_eq!(
            p_meta.get("description").map(String::as_str),
            Some("Precipitation")
        );

        let interception_field = schema
            .fields
            .iter()
            .find(|field| field.name() == "InterceptionStorage")
            .expect("InterceptionStorage field should exist");
        let interception_meta = interception_field.metadata();
        assert_eq!(
            interception_meta.get("units").map(String::as_str),
            Some("mm")
        );
        assert!(
            interception_meta
                .get("description")
                .is_some_and(|value| value.contains("optional producer-authoritative term")),
            "InterceptionStorage description should include optional producer-authoritative note"
        );
    }

    #[test]
    fn writer_emits_valid_parquet_file_with_schema_metadata() {
        let timestamp = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .expect("unix epoch should be before now")
            .as_nanos();
        let output_path =
            std::env::temp_dir().join(format!("openwepp_hillslope_wat_writer_{timestamp}.parquet"));

        let summary = write_hillslope_wat_parquet(
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
        assert!(
            schema
                .fields()
                .iter()
                .any(|field| field.name() == "InterceptionStorage")
        );

        let _ = fs::remove_file(output_path);
    }
}
