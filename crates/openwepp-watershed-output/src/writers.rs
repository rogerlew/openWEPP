use std::collections::HashMap;
use std::fmt;
use std::fs::{self, File};
use std::io;
use std::path::Path;
use std::path::PathBuf;
use std::sync::Arc;

use arrow_array::{
    ArrayRef, Float64Array, Int8Array, Int16Array, Int32Array, RecordBatch, StringArray,
};
use arrow_schema::{DataType, Field, Schema};
use openwepp_sim_contract::units::{OutputUnitRegistryError, validate_output_schema_unit};
use parquet::arrow::ArrowWriter;
use parquet::basic::Compression;
use parquet::file::properties::WriterProperties;

use crate::contracts::WatershedOutputConfig;

const INTERCHANGE_VERSION_MAJOR: u32 = 1;
const INTERCHANGE_VERSION_MINOR: u32 = 2;

#[derive(Debug)]
pub enum WatershedWriterError {
    Io {
        code: &'static str,
        path: PathBuf,
        source: io::Error,
    },
    Parquet {
        code: &'static str,
        path: PathBuf,
        detail: String,
    },
    UnitMetadata {
        detail: String,
    },
    UnsupportedFieldType {
        field_name: String,
        data_type: String,
    },
}

impl WatershedWriterError {
    #[must_use]
    pub const fn code(&self) -> &'static str {
        match self {
            Self::Io { code, .. } | Self::Parquet { code, .. } => code,
            Self::UnitMetadata { .. } => "OWOUT-UNIT-E-001",
            Self::UnsupportedFieldType { .. } => "OWSOUT-E-006",
        }
    }
}

impl fmt::Display for WatershedWriterError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Io { code, path, source } => {
                write!(formatter, "{code} io error at {}: {source}", path.display())
            }
            Self::Parquet { code, path, detail } => {
                write!(
                    formatter,
                    "{code} parquet error at {}: {detail}",
                    path.display()
                )
            }
            Self::UnitMetadata { detail } => {
                write!(
                    formatter,
                    "{} output unit metadata error: {detail}",
                    self.code()
                )
            }
            Self::UnsupportedFieldType {
                field_name,
                data_type,
            } => write!(
                formatter,
                "{} unsupported watershed output field type {data_type} for {field_name}",
                self.code()
            ),
        }
    }
}

impl std::error::Error for WatershedWriterError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::Io { source, .. } => Some(source),
            Self::Parquet { .. }
            | Self::UnitMetadata { .. }
            | Self::UnsupportedFieldType { .. } => None,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct WatershedInterchangeRowSeed {
    pub year: i16,
    pub simulation_year: i16,
    pub sim_day_index: i32,
    pub julian: i16,
    pub month: i8,
    pub day_of_month: i8,
    pub water_year: i16,
    pub element_id: i32,
    pub channel_id: i32,
    pub runoff_volume_m3: f64,
    pub peak_discharge_m3_s: f64,
    pub sediment_yield_kg: f64,
    pub soluble_pollutant_kg: f64,
    pub particulate_pollutant_kg: f64,
    pub channel_outflow_m3: f64,
    pub channel_storage_m3: f64,
    pub channel_baseflow_m3: f64,
    pub channel_loss_m3: f64,
}

impl Default for WatershedInterchangeRowSeed {
    fn default() -> Self {
        Self {
            year: 1,
            simulation_year: 1,
            sim_day_index: 1,
            julian: 1,
            month: 1,
            day_of_month: 1,
            water_year: 1,
            element_id: 1,
            channel_id: 1,
            runoff_volume_m3: 0.0,
            peak_discharge_m3_s: 0.0,
            sediment_yield_kg: 0.0,
            soluble_pollutant_kg: 0.0,
            particulate_pollutant_kg: 0.0,
            channel_outflow_m3: 0.0,
            channel_storage_m3: 0.0,
            channel_baseflow_m3: 0.0,
            channel_loss_m3: 0.0,
        }
    }
}

pub fn write_interchange_parquet_outputs(
    outputs: &WatershedOutputConfig,
    row_seed: WatershedInterchangeRowSeed,
) -> Result<(), WatershedWriterError> {
    write_single_output(&outputs.ebe_pw0, watershed_ebe_schema()?, row_seed)?;
    write_single_output(&outputs.chan_out, watershed_chan_peak_schema()?, row_seed)?;
    write_single_output(&outputs.chanwb, watershed_chanwb_schema()?, row_seed)?;
    write_single_output(&outputs.chnwb, watershed_chnwb_schema()?, row_seed)?;
    write_single_output(&outputs.soil_pw0, watershed_soil_schema()?, row_seed)?;
    write_single_output(
        &outputs.totalwatsed3,
        watershed_totalwatsed3_schema()?,
        row_seed,
    )?;
    write_single_output(
        &outputs.loss_hill,
        watershed_loss_average_hill_schema()?,
        row_seed,
    )?;
    write_single_output(
        &outputs.loss_chn,
        watershed_loss_average_chn_schema()?,
        row_seed,
    )?;
    write_single_output(
        &outputs.loss_out,
        watershed_loss_average_out_schema()?,
        row_seed,
    )?;
    write_single_output(
        &outputs.loss_class_data,
        watershed_loss_average_class_schema()?,
        row_seed,
    )?;
    write_single_output(
        &outputs.loss_all_years_hill,
        watershed_loss_all_years_hill_schema()?,
        row_seed,
    )?;
    write_single_output(
        &outputs.loss_all_years_chn,
        watershed_loss_all_years_chn_schema()?,
        row_seed,
    )?;
    write_single_output(
        &outputs.loss_all_years_out,
        watershed_loss_all_years_out_schema()?,
        row_seed,
    )?;
    write_single_output(
        &outputs.loss_all_years_class_data,
        watershed_loss_all_years_class_schema()?,
        row_seed,
    )?;
    Ok(())
}

pub fn watershed_interchange_schemas() -> Result<Vec<(&'static str, Schema)>, WatershedWriterError>
{
    Ok(vec![
        ("watershed_ebe", watershed_ebe_schema()?),
        ("watershed_chan_peak", watershed_chan_peak_schema()?),
        ("watershed_chanwb", watershed_chanwb_schema()?),
        ("watershed_chnwb", watershed_chnwb_schema()?),
        ("watershed_soil", watershed_soil_schema()?),
        ("watershed_totalwatsed3", watershed_totalwatsed3_schema()?),
        (
            "watershed_loss_average_hill",
            watershed_loss_average_hill_schema()?,
        ),
        (
            "watershed_loss_average_chn",
            watershed_loss_average_chn_schema()?,
        ),
        (
            "watershed_loss_average_out",
            watershed_loss_average_out_schema()?,
        ),
        (
            "watershed_loss_average_class",
            watershed_loss_average_class_schema()?,
        ),
        (
            "watershed_loss_all_years_hill",
            watershed_loss_all_years_hill_schema()?,
        ),
        (
            "watershed_loss_all_years_chn",
            watershed_loss_all_years_chn_schema()?,
        ),
        (
            "watershed_loss_all_years_out",
            watershed_loss_all_years_out_schema()?,
        ),
        (
            "watershed_loss_all_years_class",
            watershed_loss_all_years_class_schema()?,
        ),
    ])
}

fn field_with_meta(
    name: &str,
    data_type: DataType,
    units: Option<&str>,
    description: Option<&str>,
) -> Field {
    let mut metadata = HashMap::new();
    if let Some(units) = units {
        metadata.insert("units".to_string(), units.to_string());
    }
    if let Some(description) = description {
        metadata.insert("description".to_string(), description.to_string());
    }
    if metadata.is_empty() {
        Field::new(name, data_type, true)
    } else {
        Field::new(name, data_type, true).with_metadata(metadata)
    }
}

fn field(name: &str, data_type: DataType) -> Field {
    field_with_meta(name, data_type, None, None)
}

fn dynamic_unit_value_field() -> Field {
    let mut metadata = HashMap::new();
    metadata.insert("unit_source".to_string(), "units".to_string());
    metadata.insert(
        "description".to_string(),
        "Dynamic numeric value; physical unit is stored in the sibling units column".to_string(),
    );
    Field::new("value", DataType::Float64, true).with_metadata(metadata)
}

fn schema_with_interchange_version(
    schema_id: &'static str,
    schema: &Schema,
) -> Result<Schema, WatershedWriterError> {
    let schema = align_output_schema_units(schema_id, schema)?;
    let mut metadata = schema.metadata().clone();
    metadata.insert(
        "dataset_version".to_string(),
        format!("{INTERCHANGE_VERSION_MAJOR}.{INTERCHANGE_VERSION_MINOR}"),
    );
    metadata.insert(
        "dataset_version_major".to_string(),
        INTERCHANGE_VERSION_MAJOR.to_string(),
    );
    metadata.insert(
        "dataset_version_minor".to_string(),
        INTERCHANGE_VERSION_MINOR.to_string(),
    );
    metadata.insert(
        "schema_version".to_string(),
        INTERCHANGE_VERSION_MAJOR.to_string(),
    );
    Ok(schema.with_metadata(metadata))
}

fn output_registry_error(error: &OutputUnitRegistryError) -> WatershedWriterError {
    WatershedWriterError::UnitMetadata {
        detail: error.to_string(),
    }
}

fn align_output_schema_units(
    schema_id: &'static str,
    schema: &Schema,
) -> Result<Schema, WatershedWriterError> {
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

fn watershed_ebe_schema() -> Result<Schema, WatershedWriterError> {
    schema_with_interchange_version(
        "watershed_ebe",
        &Schema::new(vec![
            field_with_meta("year", DataType::Int16, None, Some("Calendar year")),
            field_with_meta(
                "sim_day_index",
                DataType::Int32,
                None,
                Some("1-indexed simulation day"),
            ),
            field_with_meta(
                "simulation_year",
                DataType::Int16,
                None,
                Some("WEPP simulation year reported in output"),
            ),
            field_with_meta("month", DataType::Int8, None, Some("Calendar month")),
            field_with_meta(
                "day_of_month",
                DataType::Int8,
                None,
                Some("Calendar day of month"),
            ),
            field_with_meta(
                "julian",
                DataType::Int16,
                None,
                Some("Julian day from WEPP output"),
            ),
            field_with_meta(
                "water_year",
                DataType::Int16,
                None,
                Some("Water year derived from year/julian"),
            ),
            field_with_meta(
                "precip",
                DataType::Float64,
                Some("mm"),
                Some("Watershed precipitation depth for the event"),
            ),
            field_with_meta(
                "runoff_volume",
                DataType::Float64,
                Some("m^3"),
                Some("Watershed runoff volume for the event"),
            ),
            field_with_meta(
                "peak_runoff",
                DataType::Float64,
                Some("m^3/s"),
                Some("Peak watershed discharge"),
            ),
            field_with_meta(
                "sediment_yield",
                DataType::Float64,
                Some("kg"),
                Some("Sediment yield at the watershed outlet"),
            ),
            field_with_meta(
                "soluble_pollutant",
                DataType::Float64,
                Some("kg"),
                Some("Soluble pollutant mass delivered at watershed outlet"),
            ),
            field_with_meta(
                "particulate_pollutant",
                DataType::Float64,
                Some("kg"),
                Some("Particulate pollutant mass delivered at watershed outlet"),
            ),
            field_with_meta(
                "total_pollutant",
                DataType::Float64,
                Some("kg"),
                Some("Total pollutant mass delivered (soluble + particulate)"),
            ),
            field_with_meta(
                "element_id",
                DataType::Int32,
                None,
                Some("Channel element identifier (Elmt_ID)"),
            ),
        ]),
    )
}

fn watershed_chan_peak_schema() -> Result<Schema, WatershedWriterError> {
    schema_with_interchange_version(
        "watershed_chan_peak",
        &Schema::new(vec![
            field_with_meta("year", DataType::Int16, None, Some("Calendar year")),
            field_with_meta(
                "simulation_year",
                DataType::Int16,
                None,
                Some("Simulation year from chan.out"),
            ),
            field_with_meta(
                "julian",
                DataType::Int16,
                None,
                Some("Julian day reported by WEPP"),
            ),
            field_with_meta(
                "month",
                DataType::Int8,
                None,
                Some("Calendar month derived from Julian day"),
            ),
            field_with_meta(
                "day_of_month",
                DataType::Int8,
                None,
                Some("Calendar day-of-month derived from Julian day"),
            ),
            field_with_meta(
                "water_year",
                DataType::Int16,
                None,
                Some("Water year computed from Julian day"),
            ),
            field_with_meta(
                "Elmt_ID",
                DataType::Int32,
                None,
                Some("Channel element identifier"),
            ),
            field_with_meta(
                "Chan_ID",
                DataType::Int32,
                None,
                Some("Channel ID reported by WEPP"),
            ),
            field_with_meta(
                "Time (s)",
                DataType::Float64,
                Some("s"),
                Some("Time to peak discharge"),
            ),
            field_with_meta(
                "Peak_Discharge (m^3/s)",
                DataType::Float64,
                Some("m^3/s"),
                Some("Peak discharge within the reporting interval"),
            ),
        ]),
    )
}

fn watershed_chanwb_schema() -> Result<Schema, WatershedWriterError> {
    schema_with_interchange_version(
        "watershed_chanwb",
        &Schema::new(vec![
            field_with_meta("year", DataType::Int16, None, Some("Calendar year")),
            field_with_meta(
                "simulation_year",
                DataType::Int16,
                None,
                Some("Simulation year from chanwb.out"),
            ),
            field_with_meta(
                "julian",
                DataType::Int16,
                None,
                Some("Julian day reported by WEPP"),
            ),
            field_with_meta(
                "month",
                DataType::Int8,
                None,
                Some("Calendar month derived from Julian day"),
            ),
            field_with_meta(
                "day_of_month",
                DataType::Int8,
                None,
                Some("Calendar day-of-month derived from Julian day"),
            ),
            field_with_meta(
                "water_year",
                DataType::Int16,
                None,
                Some("Water year computed from Julian day"),
            ),
            field_with_meta(
                "Elmt_ID",
                DataType::Int32,
                None,
                Some("Channel element identifier"),
            ),
            field_with_meta(
                "Chan_ID",
                DataType::Int32,
                None,
                Some("Channel ID reported by WEPP"),
            ),
            field_with_meta(
                "Inflow (m^3)",
                DataType::Float64,
                Some("m^3"),
                Some("Total inflow above channel outlet, includes baseflow, all sources"),
            ),
            field_with_meta(
                "Outflow (m^3)",
                DataType::Float64,
                Some("m^3"),
                Some("Water flow out of channel outlet"),
            ),
            field_with_meta(
                "Storage (m^3)",
                DataType::Float64,
                Some("m^3"),
                Some("Water surface storage at the end of the day"),
            ),
            field_with_meta(
                "Baseflow (m^3)",
                DataType::Float64,
                Some("m^3"),
                Some("Portion of inflow from baseflow"),
            ),
            field_with_meta(
                "Loss (m^3)",
                DataType::Float64,
                Some("m^3"),
                Some("Transmission loss in channel, infiltration"),
            ),
            field_with_meta(
                "Balance (m^3)",
                DataType::Float64,
                Some("m^3"),
                Some("Water balance error at end of day (inflow - outflow - loss - Δstorage)"),
            ),
        ]),
    )
}

#[allow(clippy::too_many_lines)]
fn watershed_chnwb_schema() -> Result<Schema, WatershedWriterError> {
    schema_with_interchange_version(
        "watershed_chnwb",
        &Schema::new(vec![
            field_with_meta(
                "wepp_id",
                DataType::Int32,
                None,
                Some("Channel (OFE) identifier"),
            ),
            field_with_meta("julian", DataType::Int16, None, Some("Julian day")),
            field_with_meta("year", DataType::Int16, None, Some("Calendar year")),
            field_with_meta(
                "simulation_year",
                DataType::Int16,
                None,
                Some("Simulation year value from input file"),
            ),
            field_with_meta("month", DataType::Int8, None, Some("Calendar month")),
            field_with_meta(
                "day_of_month",
                DataType::Int8,
                None,
                Some("Calendar day of month"),
            ),
            field_with_meta(
                "water_year",
                DataType::Int16,
                None,
                Some("Computed water year"),
            ),
            field_with_meta("OFE", DataType::Int16, None, Some("Channel OFE index")),
            field_with_meta("J", DataType::Int16, None, Some("Julian day as reported")),
            field_with_meta(
                "Y",
                DataType::Int16,
                None,
                Some("Simulation year as reported"),
            ),
            field_with_meta(
                "P (mm)",
                DataType::Float64,
                Some("mm"),
                Some("precipitation"),
            ),
            field_with_meta(
                "RM (mm)",
                DataType::Float64,
                Some("mm"),
                Some("rainfall + irrigation + snowmelt"),
            ),
            field_with_meta(
                "Q (mm)",
                DataType::Float64,
                Some("mm"),
                Some("daily runoff over effective length"),
            ),
            field_with_meta(
                "Ep (mm)",
                DataType::Float64,
                Some("mm"),
                Some("plant transpiration"),
            ),
            field_with_meta(
                "Es (mm)",
                DataType::Float64,
                Some("mm"),
                Some("soil evaporation"),
            ),
            field_with_meta(
                "Er (mm)",
                DataType::Float64,
                Some("mm"),
                Some("residue evaporation"),
            ),
            field_with_meta(
                "Dp (mm)",
                DataType::Float64,
                Some("mm"),
                Some("deep percolation"),
            ),
            field_with_meta(
                "UpStrmQ (mm)",
                DataType::Float64,
                Some("mm"),
                Some("Runon added to OFE"),
            ),
            field_with_meta(
                "SubRIn (mm)",
                DataType::Float64,
                Some("mm"),
                Some("Subsurface runon added to OFE"),
            ),
            field_with_meta(
                "latqcc (mm)",
                DataType::Float64,
                Some("mm"),
                Some("lateral subsurface flow"),
            ),
            field_with_meta(
                "Total Soil Water (mm)",
                DataType::Float64,
                Some("mm"),
                Some("Unfrozen water in soil profile"),
            ),
            field_with_meta(
                "frozwt (mm)",
                DataType::Float64,
                Some("mm"),
                Some("Frozen water in soil profile"),
            ),
            field_with_meta(
                "Snow Water (mm)",
                DataType::Float64,
                Some("mm"),
                Some("Water in surface snow"),
            ),
            field_with_meta(
                "QOFE (mm)",
                DataType::Float64,
                Some("mm"),
                Some("Daily runoff scaled to single OFE"),
            ),
            field_with_meta(
                "Tile (mm)",
                DataType::Float64,
                Some("mm"),
                Some("Tile drainage"),
            ),
            field_with_meta(
                "Irr (mm)",
                DataType::Float64,
                Some("mm"),
                Some("Irrigation"),
            ),
            field_with_meta(
                "Surf (mm)",
                DataType::Float64,
                Some("mm"),
                Some("Surface storage"),
            ),
            field_with_meta(
                "Base (mm)",
                DataType::Float64,
                Some("mm"),
                Some("Portion of runon from external baseflow"),
            ),
            field_with_meta(
                "Area (m^2)",
                DataType::Float64,
                Some("m^2"),
                Some("Area that depths apply over"),
            ),
        ]),
    )
}

fn watershed_soil_schema() -> Result<Schema, WatershedWriterError> {
    schema_with_interchange_version(
        "watershed_soil",
        &Schema::new(vec![
            field("wepp_id", DataType::Int32),
            field("ofe_id", DataType::Int16),
            field("year", DataType::Int16),
            field("day", DataType::Int16),
            field("julian", DataType::Int16),
            field("month", DataType::Int8),
            field("day_of_month", DataType::Int8),
            field("water_year", DataType::Int16),
            field("OFE", DataType::Int16),
            field_with_meta("Poros", DataType::Float64, Some("%"), Some("Soil porosity")),
            field_with_meta(
                "Keff",
                DataType::Float64,
                Some("mm/hr"),
                Some("Effective hydraulic conductivity"),
            ),
            field_with_meta(
                "Suct",
                DataType::Float64,
                Some("mm"),
                Some("Suction across wetting front"),
            ),
            field_with_meta(
                "FC",
                DataType::Float64,
                Some("mm/mm"),
                Some("Field capacity"),
            ),
            field_with_meta(
                "WP",
                DataType::Float64,
                Some("mm/mm"),
                Some("Wilting point"),
            ),
            field_with_meta(
                "Rough",
                DataType::Float64,
                Some("mm"),
                Some("Surface roughness"),
            ),
            field_with_meta(
                "Ki",
                DataType::Float64,
                Some("adjsmt"),
                Some("Interrill erodibility adjustment factor"),
            ),
            field_with_meta(
                "Kr",
                DataType::Float64,
                Some("adjsmt"),
                Some("Rill erodibility adjustment factor"),
            ),
            field_with_meta(
                "Tauc",
                DataType::Float64,
                Some("adjsmt"),
                Some("Critical shear stress adjustment factor"),
            ),
            field_with_meta(
                "Saturation",
                DataType::Float64,
                Some("frac"),
                Some("Saturation as fraction"),
            ),
            field_with_meta(
                "TSW",
                DataType::Float64,
                Some("mm"),
                Some("Total soil water"),
            ),
            field_with_meta(
                "TSMF",
                DataType::Float64,
                Some("frac"),
                Some("True soil moisture fraction (full profile)"),
            ),
        ]),
    )
}

#[allow(clippy::too_many_lines)]
fn watershed_totalwatsed3_schema() -> Result<Schema, WatershedWriterError> {
    schema_with_interchange_version(
        "watershed_totalwatsed3",
        &Schema::new(vec![
            field("year", DataType::Int16),
            field("sim_day_index", DataType::Int32),
            field("julian", DataType::Int16),
            field("month", DataType::Int8),
            field("day_of_month", DataType::Int8),
            field("water_year", DataType::Int16),
            field_with_meta(
                "runvol",
                DataType::Float64,
                Some("m^3"),
                Some("Runoff volume"),
            ),
            field_with_meta(
                "sbrunv",
                DataType::Float64,
                Some("m^3"),
                Some("Subsurface runoff volume"),
            ),
            field_with_meta(
                "tdet",
                DataType::Float64,
                Some("kg"),
                Some("Total detachment"),
            ),
            field_with_meta(
                "tdep",
                DataType::Float64,
                Some("kg"),
                Some("Total deposition"),
            ),
            field_with_meta(
                "seddep_1",
                DataType::Float64,
                Some("kg"),
                Some("Sediment Class 1 deposition"),
            ),
            field_with_meta(
                "seddep_2",
                DataType::Float64,
                Some("kg"),
                Some("Sediment Class 2 deposition"),
            ),
            field_with_meta(
                "seddep_3",
                DataType::Float64,
                Some("kg"),
                Some("Sediment Class 3 deposition"),
            ),
            field_with_meta(
                "seddep_4",
                DataType::Float64,
                Some("kg"),
                Some("Sediment Class 4 deposition"),
            ),
            field_with_meta(
                "seddep_5",
                DataType::Float64,
                Some("kg"),
                Some("Sediment Class 5 deposition"),
            ),
            field_with_meta(
                "sed_del",
                DataType::Float64,
                Some("kg"),
                Some("Total sediment delivery (sum of class masses)"),
            ),
            field_with_meta(
                "sed_vol_conc",
                DataType::Float64,
                Some("m^3/m^3"),
                Some(
                    "Total volumetric sediment concentration (solids volume divided by runoff volume)",
                ),
            ),
            field_with_meta(
                "Area",
                DataType::Float64,
                Some("m^2"),
                Some("Area that depths apply over"),
            ),
            field_with_meta(
                "P",
                DataType::Float64,
                Some("m^3"),
                Some("Precipitation volume"),
            ),
            field_with_meta(
                "RM",
                DataType::Float64,
                Some("m^3"),
                Some("Rainfall+Irrigation+Snowmelt volume"),
            ),
            field_with_meta(
                "Q",
                DataType::Float64,
                Some("m^3"),
                Some("Daily runoff over effective length volume"),
            ),
            field_with_meta(
                "Dp",
                DataType::Float64,
                Some("m^3"),
                Some("Deep percolation volume"),
            ),
            field_with_meta(
                "latqcc",
                DataType::Float64,
                Some("m^3"),
                Some("Lateral subsurface flow volume"),
            ),
            field_with_meta(
                "QOFE",
                DataType::Float64,
                Some("m^3"),
                Some("Daily runoff scaled to single OFE volume"),
            ),
            field_with_meta(
                "Ep",
                DataType::Float64,
                Some("m^3"),
                Some("Plant transpiration volume"),
            ),
            field_with_meta(
                "Es",
                DataType::Float64,
                Some("m^3"),
                Some("Soil evaporation volume"),
            ),
            field_with_meta(
                "Er",
                DataType::Float64,
                Some("m^3"),
                Some("Residue evaporation volume"),
            ),
            field_with_meta(
                "UpStrmQ",
                DataType::Float64,
                Some("mm"),
                Some("Runon added to OFE depth"),
            ),
            field_with_meta(
                "SubRIn",
                DataType::Float64,
                Some("mm"),
                Some("Subsurface runon added to OFE depth"),
            ),
            field_with_meta(
                "Total-Soil Water",
                DataType::Float64,
                Some("mm"),
                Some("Unfrozen water in soil profile depth"),
            ),
            field_with_meta(
                "SoilWaterTotal",
                DataType::Float64,
                Some("mm"),
                Some("Area-weighted hydout-equivalent aggregate soil water depth"),
            ),
            field_with_meta(
                "ProfileDepth",
                DataType::Float64,
                Some("mm"),
                Some("Area-weighted full soil profile depth (solthk(nsl))"),
            ),
            field_with_meta(
                "ProfilePorosityCap",
                DataType::Float64,
                Some("mm"),
                Some("Area-weighted full-profile porosity storage capacity (sum(por * dg))"),
            ),
            field_with_meta(
                "ProfileFCStore",
                DataType::Float64,
                Some("mm"),
                Some("Area-weighted full-profile field-capacity storage (sum(thetfc * dg))"),
            ),
            field_with_meta(
                "ProfileWPStore",
                DataType::Float64,
                Some("mm"),
                Some("Area-weighted full-profile wilting-point storage (sum(thetdr * dg))"),
            ),
            field_with_meta(
                "InterceptionStorage",
                DataType::Float64,
                Some("mm"),
                Some(
                    "Area-weighted plant/residue interception carryover storage depth (pintlv + resint)",
                ),
            ),
            field_with_meta(
                "TSMF",
                DataType::Float64,
                Some("frac"),
                Some("Area-weighted true soil moisture fraction (full profile)"),
            ),
            field_with_meta(
                "frozwt",
                DataType::Float64,
                Some("mm"),
                Some("Frozen water in soil profile depth"),
            ),
            field_with_meta(
                "Snow-Water",
                DataType::Float64,
                Some("mm"),
                Some("Water in surface snow depth"),
            ),
            field_with_meta(
                "QRain",
                DataType::Float64,
                Some("mm"),
                Some("Area-weighted rain-generated runoff depth from element partitioning"),
            ),
            field_with_meta(
                "QSnow",
                DataType::Float64,
                Some("mm"),
                Some("Area-weighted snow-generated runoff depth from element partitioning"),
            ),
            field_with_meta(
                "Tile",
                DataType::Float64,
                Some("mm"),
                Some("Tile drainage depth"),
            ),
            field_with_meta(
                "Irr",
                DataType::Float64,
                Some("mm"),
                Some("Irrigation depth"),
            ),
            field_with_meta(
                "Precipitation",
                DataType::Float64,
                Some("mm"),
                Some("Precipitation depth"),
            ),
            field_with_meta(
                "Rain+Melt",
                DataType::Float64,
                Some("mm"),
                Some("Rainfall+Irrigation+Snowmelt depth"),
            ),
            field_with_meta(
                "Percolation",
                DataType::Float64,
                Some("mm"),
                Some("Deep percolation depth"),
            ),
            field_with_meta(
                "Lateral Flow",
                DataType::Float64,
                Some("mm"),
                Some("Lateral subsurface flow depth"),
            ),
            field_with_meta(
                "Runoff",
                DataType::Float64,
                Some("mm"),
                Some("Daily runoff depth from PASS runoff volume"),
            ),
            field_with_meta(
                "Transpiration",
                DataType::Float64,
                Some("mm"),
                Some("Plant transpiration depth"),
            ),
            field_with_meta(
                "Evaporation",
                DataType::Float64,
                Some("mm"),
                Some("Soil + residue evaporation depth"),
            ),
            field_with_meta(
                "ET",
                DataType::Float64,
                Some("mm"),
                Some("Total evapotranspiration depth"),
            ),
            field_with_meta(
                "Baseflow",
                DataType::Float64,
                Some("mm"),
                Some("Baseflow depth"),
            ),
            field_with_meta(
                "Aquifer losses",
                DataType::Float64,
                Some("mm"),
                Some("Aquifer losses depth"),
            ),
            field_with_meta(
                "Reservoir Volume",
                DataType::Float64,
                Some("mm"),
                Some("Groundwater storage depth"),
            ),
            field_with_meta(
                "Streamflow",
                DataType::Float64,
                Some("mm"),
                Some("Streamflow depth"),
            ),
            field_with_meta(
                "wind_transport",
                DataType::Float64,
                Some("tonne"),
                Some("Ash transported by wind (total mass)"),
            ),
            field_with_meta(
                "wind_transport_per_ha",
                DataType::Float64,
                Some("tonne/ha"),
                Some("Ash transported by wind per unit area"),
            ),
            field_with_meta(
                "wind_transport_black",
                DataType::Float64,
                Some("tonne"),
                Some("Black ash transported by wind (total mass)"),
            ),
            field_with_meta(
                "wind_transport_black_per_ha",
                DataType::Float64,
                Some("tonne/ha"),
                Some("Black ash transported by wind per unit area over black ash hillslopes"),
            ),
            field_with_meta(
                "wind_transport_white",
                DataType::Float64,
                Some("tonne"),
                Some("White ash transported by wind (total mass)"),
            ),
            field_with_meta(
                "wind_transport_white_per_ha",
                DataType::Float64,
                Some("tonne/ha"),
                Some("White ash transported by wind per unit area over white ash hillslopes"),
            ),
            field_with_meta(
                "water_transport",
                DataType::Float64,
                Some("tonne"),
                Some("Ash transported by water (total mass)"),
            ),
            field_with_meta(
                "water_transport_per_ha",
                DataType::Float64,
                Some("tonne/ha"),
                Some("Ash transported by water per unit area"),
            ),
            field_with_meta(
                "water_transport_black",
                DataType::Float64,
                Some("tonne"),
                Some("Black ash transported by water (total mass)"),
            ),
            field_with_meta(
                "water_transport_black_per_ha",
                DataType::Float64,
                Some("tonne/ha"),
                Some("Black ash transported by water per unit area over black ash hillslopes"),
            ),
            field_with_meta(
                "water_transport_white",
                DataType::Float64,
                Some("tonne"),
                Some("White ash transported by water (total mass)"),
            ),
            field_with_meta(
                "water_transport_white_per_ha",
                DataType::Float64,
                Some("tonne/ha"),
                Some("White ash transported by water per unit area over white ash hillslopes"),
            ),
            field_with_meta(
                "ash_transport",
                DataType::Float64,
                Some("tonne"),
                Some("Total ash transported (wind + water)"),
            ),
            field_with_meta(
                "ash_transport_per_ha",
                DataType::Float64,
                Some("tonne/ha"),
                Some("Total ash transported per unit area"),
            ),
            field_with_meta(
                "ash_transport_black",
                DataType::Float64,
                Some("tonne"),
                Some("Black ash transported by wind + water (total mass)"),
            ),
            field_with_meta(
                "ash_transport_black_per_ha",
                DataType::Float64,
                Some("tonne/ha"),
                Some("Black ash transported per unit area over black ash hillslopes"),
            ),
            field_with_meta(
                "ash_transport_white",
                DataType::Float64,
                Some("tonne"),
                Some("White ash transported by wind + water (total mass)"),
            ),
            field_with_meta(
                "ash_transport_white_per_ha",
                DataType::Float64,
                Some("tonne/ha"),
                Some("White ash transported per unit area over white ash hillslopes"),
            ),
            field_with_meta(
                "transportable_ash",
                DataType::Float64,
                Some("tonne"),
                Some("Ash mass still available for transport"),
            ),
            field_with_meta(
                "transportable_ash_per_ha",
                DataType::Float64,
                Some("tonne/ha"),
                Some("Ash mass still available for transport per unit area"),
            ),
            field_with_meta(
                "ash_vol_conc",
                DataType::Float64,
                Some("m^3/m^3"),
                Some("Ash volumetric concentration (solids volume divided by runoff volume)"),
            ),
            field_with_meta(
                "sed+ash_vol_conc",
                DataType::Float64,
                Some("m^3/m^3"),
                Some(
                    "Sediment + ash volumetric concentration (total solids volume divided by runoff volume)",
                ),
            ),
            field_with_meta(
                "ash_black_pct_by_vol",
                DataType::Float64,
                Some("percent"),
                Some(
                    "Fraction of ash solids volume that is black ash (percent of total ash volume)",
                ),
            ),
        ]),
    )
}

fn watershed_loss_all_years_hill_schema() -> Result<Schema, WatershedWriterError> {
    schema_with_interchange_version(
        "watershed_loss_all_years_hill",
        &Schema::new(vec![
            field("year", DataType::Int16),
            field("Type", DataType::Utf8),
            field("wepp_id", DataType::Int32),
            field_with_meta("Runoff Volume", DataType::Float64, Some("m^3"), None),
            field_with_meta("Subrunoff Volume", DataType::Float64, Some("m^3"), None),
            field_with_meta("Baseflow Volume", DataType::Float64, Some("m^3"), None),
            field_with_meta("Soil Loss", DataType::Float64, Some("kg"), None),
            field_with_meta("Sediment Deposition", DataType::Float64, Some("kg"), None),
            field_with_meta("Sediment Yield", DataType::Float64, Some("kg"), None),
            field_with_meta(
                "Solub. React. Pollutant",
                DataType::Float64,
                Some("kg"),
                None,
            ),
            field_with_meta("Particulate Pollutant", DataType::Float64, Some("kg"), None),
            field_with_meta("Total Pollutant", DataType::Float64, Some("kg"), None),
        ])
        .with_metadata(loss_table_metadata("loss_pw0.all_years.hill")),
    )
}

fn watershed_loss_average_hill_schema() -> Result<Schema, WatershedWriterError> {
    schema_with_interchange_version(
        "watershed_loss_average_hill",
        &Schema::new(vec![
            field("Type", DataType::Utf8),
            field("wepp_id", DataType::Int32),
            field_with_meta("Runoff Volume", DataType::Float64, Some("m^3"), None),
            field_with_meta("Subrunoff Volume", DataType::Float64, Some("m^3"), None),
            field_with_meta("Baseflow Volume", DataType::Float64, Some("m^3"), None),
            field_with_meta("Soil Loss", DataType::Float64, Some("kg"), None),
            field_with_meta("Sediment Deposition", DataType::Float64, Some("kg"), None),
            field_with_meta("Sediment Yield", DataType::Float64, Some("kg"), None),
            field_with_meta("Hillslope Area", DataType::Float64, Some("ha"), None),
            field_with_meta(
                "Solub. React. Pollutant",
                DataType::Float64,
                Some("kg"),
                None,
            ),
            field_with_meta("Particulate Pollutant", DataType::Float64, Some("kg"), None),
            field_with_meta("Total Pollutant", DataType::Float64, Some("kg"), None),
        ])
        .with_metadata(loss_table_metadata("loss_pw0.hill")),
    )
}

fn watershed_loss_all_years_chn_schema() -> Result<Schema, WatershedWriterError> {
    schema_with_interchange_version(
        "watershed_loss_all_years_chn",
        &Schema::new(vec![
            field("year", DataType::Int16),
            field("Type", DataType::Utf8),
            field("chn_enum", DataType::Int32),
            field_with_meta("Discharge Volume", DataType::Float64, Some("m^3"), None),
            field_with_meta("Sediment Yield", DataType::Float64, Some("tonne"), None),
            field_with_meta("Soil Loss", DataType::Float64, Some("kg"), None),
            field_with_meta("Upland Charge", DataType::Float64, Some("m^3"), None),
            field_with_meta(
                "Subsuface Flow Volume",
                DataType::Float64,
                Some("m^3"),
                None,
            ),
            field_with_meta(
                "Solub. React. Pollutant",
                DataType::Float64,
                Some("kg"),
                None,
            ),
            field_with_meta("Particulate Pollutant", DataType::Float64, Some("kg"), None),
            field_with_meta("Total Pollutant", DataType::Float64, Some("kg"), None),
            field("wepp_id", DataType::Int32),
        ])
        .with_metadata(loss_table_metadata("loss_pw0.all_years.chn")),
    )
}

fn watershed_loss_average_chn_schema() -> Result<Schema, WatershedWriterError> {
    schema_with_interchange_version(
        "watershed_loss_average_chn",
        &Schema::new(vec![
            field("Type", DataType::Utf8),
            field("chn_enum", DataType::Int32),
            field_with_meta("Discharge Volume", DataType::Float64, Some("m^3"), None),
            field_with_meta("Sediment Yield", DataType::Float64, Some("tonne"), None),
            field_with_meta("Soil Loss", DataType::Float64, Some("kg"), None),
            field_with_meta("Upland Charge", DataType::Float64, Some("m^3"), None),
            field_with_meta(
                "Subsuface Flow Volume",
                DataType::Float64,
                Some("m^3"),
                None,
            ),
            field_with_meta("Contributing Area", DataType::Float64, Some("ha"), None),
            field_with_meta(
                "Solub. React. Pollutant",
                DataType::Float64,
                Some("kg"),
                None,
            ),
            field_with_meta("Particulate Pollutant", DataType::Float64, Some("kg"), None),
            field_with_meta("Total Pollutant", DataType::Float64, Some("kg"), None),
            field("wepp_id", DataType::Int32),
        ])
        .with_metadata(loss_table_metadata("loss_pw0.chn")),
    )
}

fn watershed_loss_all_years_out_schema() -> Result<Schema, WatershedWriterError> {
    schema_with_interchange_version(
        "watershed_loss_all_years_out",
        &Schema::new(vec![
            field("year", DataType::Int16),
            field("key", DataType::Utf8),
            dynamic_unit_value_field(),
            field_with_meta(
                "units",
                DataType::Utf8,
                None,
                Some("Row-level physical unit for value"),
            ),
        ])
        .with_metadata(loss_table_metadata("loss_pw0.all_years.out")),
    )
}

fn watershed_loss_average_out_schema() -> Result<Schema, WatershedWriterError> {
    schema_with_interchange_version(
        "watershed_loss_average_out",
        &Schema::new(vec![
            field("key", DataType::Utf8),
            dynamic_unit_value_field(),
            field_with_meta(
                "units",
                DataType::Utf8,
                None,
                Some("Row-level physical unit for value"),
            ),
        ])
        .with_metadata(loss_table_metadata("loss_pw0.out")),
    )
}

fn watershed_loss_all_years_class_schema() -> Result<Schema, WatershedWriterError> {
    schema_with_interchange_version(
        "watershed_loss_all_years_class",
        &Schema::new(vec![
            field("year", DataType::Int16),
            field("Class", DataType::Int8),
            field_with_meta("Diameter", DataType::Float64, Some("mm"), None),
            field("Specific Gravity", DataType::Float64),
            field_with_meta("Pct Sand", DataType::Float64, Some("%"), None),
            field_with_meta("Pct Silt", DataType::Float64, Some("%"), None),
            field_with_meta("Pct Clay", DataType::Float64, Some("%"), None),
            field_with_meta("Pct OM", DataType::Float64, Some("%"), None),
            field_with_meta(
                "Fraction In Flow Exiting",
                DataType::Float64,
                Some("dimensionless"),
                None,
            ),
        ])
        .with_metadata(loss_table_metadata("loss_pw0.all_years.class_data")),
    )
}

fn watershed_loss_average_class_schema() -> Result<Schema, WatershedWriterError> {
    schema_with_interchange_version(
        "watershed_loss_average_class",
        &Schema::new(vec![
            field("Class", DataType::Int8),
            field_with_meta("Diameter", DataType::Float64, Some("mm"), None),
            field("Specific Gravity", DataType::Float64),
            field_with_meta("Pct Sand", DataType::Float64, Some("%"), None),
            field_with_meta("Pct Silt", DataType::Float64, Some("%"), None),
            field_with_meta("Pct Clay", DataType::Float64, Some("%"), None),
            field_with_meta("Pct OM", DataType::Float64, Some("%"), None),
            field_with_meta(
                "Fraction In Flow Exiting",
                DataType::Float64,
                Some("dimensionless"),
                None,
            ),
        ])
        .with_metadata(loss_table_metadata("loss_pw0.class_data")),
    )
}

fn loss_table_metadata(table: &str) -> HashMap<String, String> {
    let mut metadata = HashMap::new();
    metadata.insert("schema_version".to_string(), "1".to_string());
    metadata.insert("table".to_string(), table.to_string());
    metadata
}

fn write_single_output(
    path: &Path,
    schema: Schema,
    row_seed: WatershedInterchangeRowSeed,
) -> Result<(), WatershedWriterError> {
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent).map_err(|source| WatershedWriterError::Io {
            code: "OWSOUT-E-003",
            path: parent.to_path_buf(),
            source,
        })?;
    }

    let batch = build_single_row_batch(&schema, row_seed)?;
    let file = File::create(path).map_err(|source| WatershedWriterError::Io {
        code: "OWSOUT-E-003",
        path: path.to_path_buf(),
        source,
    })?;

    let writer_properties = WriterProperties::builder()
        .set_compression(Compression::SNAPPY)
        .build();

    let mut writer = ArrowWriter::try_new(file, Arc::new(schema), Some(writer_properties))
        .map_err(|error| WatershedWriterError::Parquet {
            code: "OWSOUT-E-005",
            path: path.to_path_buf(),
            detail: format!("failed initializing parquet writer: {error}"),
        })?;
    writer
        .write(&batch)
        .map_err(|error| WatershedWriterError::Parquet {
            code: "OWSOUT-E-005",
            path: path.to_path_buf(),
            detail: format!("failed writing parquet batch: {error}"),
        })?;
    writer
        .close()
        .map_err(|error| WatershedWriterError::Parquet {
            code: "OWSOUT-E-005",
            path: path.to_path_buf(),
            detail: format!("failed finalizing parquet output: {error}"),
        })?;
    Ok(())
}

fn build_single_row_batch(
    schema: &Schema,
    row_seed: WatershedInterchangeRowSeed,
) -> Result<RecordBatch, WatershedWriterError> {
    let mut columns: Vec<ArrayRef> = Vec::with_capacity(schema.fields().len());

    for field in schema.fields() {
        let column: ArrayRef = match field.data_type() {
            DataType::Int8 => Arc::new(Int8Array::from(vec![int8_value(field.name(), row_seed)])),
            DataType::Int16 => {
                Arc::new(Int16Array::from(vec![int16_value(field.name(), row_seed)]))
            }
            DataType::Int32 => {
                Arc::new(Int32Array::from(vec![int32_value(field.name(), row_seed)]))
            }
            DataType::Float64 => Arc::new(Float64Array::from(vec![float64_value(
                field.name(),
                row_seed,
            )])),
            DataType::Utf8 => Arc::new(StringArray::from(vec![utf8_value(field.name())])),
            unsupported => {
                return Err(WatershedWriterError::UnsupportedFieldType {
                    field_name: field.name().clone(),
                    data_type: format!("{unsupported:?}"),
                });
            }
        };
        columns.push(column);
    }

    RecordBatch::try_new(Arc::new(schema.clone()), columns).map_err(|error| {
        WatershedWriterError::Parquet {
            code: "OWSOUT-E-005",
            path: PathBuf::from("<in-memory watershed schema>"),
            detail: format!("failed building watershed output record batch: {error}"),
        }
    })
}

fn int8_value(field_name: &str, row_seed: WatershedInterchangeRowSeed) -> i8 {
    match field_name {
        "month" => row_seed.month,
        "day_of_month" => row_seed.day_of_month,
        "Class" => 1,
        _ => 0,
    }
}

fn int16_value(field_name: &str, row_seed: WatershedInterchangeRowSeed) -> i16 {
    match field_name {
        "year" => row_seed.year,
        "simulation_year" | "Y" => row_seed.simulation_year,
        "day" | "julian" | "J" => row_seed.julian,
        "water_year" => row_seed.water_year,
        "ofe_id" | "OFE" => 1,
        _ => 0,
    }
}

fn int32_value(field_name: &str, row_seed: WatershedInterchangeRowSeed) -> i32 {
    match field_name {
        "sim_day_index" => row_seed.sim_day_index,
        "element_id" | "Elmt_ID" | "wepp_id" => row_seed.element_id,
        "Chan_ID" | "chn_enum" => row_seed.channel_id,
        _ => 0,
    }
}

fn float64_value(field_name: &str, row_seed: WatershedInterchangeRowSeed) -> f64 {
    let total_pollutant = row_seed.soluble_pollutant_kg + row_seed.particulate_pollutant_kg;
    let sediment_yield_tonnes = row_seed.sediment_yield_kg / 1_000.0;

    match field_name {
        "runoff_volume" | "runvol" | "Runoff Volume" | "Discharge Volume" => {
            row_seed.runoff_volume_m3
        }
        "peak_runoff" | "Peak_Discharge (m^3/s)" => row_seed.peak_discharge_m3_s,
        "sediment_yield" | "sed_del" => row_seed.sediment_yield_kg,
        "Sediment Yield" => sediment_yield_tonnes,
        "soluble_pollutant" | "Solub. React. Pollutant" => row_seed.soluble_pollutant_kg,
        "particulate_pollutant" | "Particulate Pollutant" => row_seed.particulate_pollutant_kg,
        "total_pollutant" | "Total Pollutant" => total_pollutant,
        "Inflow (m^3)" | "value" => row_seed.runoff_volume_m3,
        "Outflow (m^3)" => row_seed.channel_outflow_m3,
        "Storage (m^3)" => row_seed.channel_storage_m3,
        "Baseflow (m^3)" => row_seed.channel_baseflow_m3,
        "Loss (m^3)" => row_seed.channel_loss_m3,
        "Balance (m^3)" => {
            row_seed.runoff_volume_m3 - row_seed.channel_outflow_m3 - row_seed.channel_loss_m3
        }
        "Specific Gravity" => 2.65,
        "Fraction In Flow Exiting"
        | "Area"
        | "Area (m^2)"
        | "Hillslope Area"
        | "Contributing Area" => 1.0,
        "Diameter" => 0.25,
        _ => 0.0,
    }
}

fn utf8_value(field_name: &str) -> &'static str {
    match field_name {
        "Type" => "watershed",
        "key" => "runoff_volume",
        "units" => "m^3",
        _ => "",
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use parquet::arrow::arrow_reader::ParquetRecordBatchReaderBuilder;
    use std::fs::{self, File};
    use std::path::{Path, PathBuf};
    use std::time::{SystemTime, UNIX_EPOCH};

    fn sample_config(base: &Path) -> WatershedOutputConfig {
        WatershedOutputConfig {
            ebe_pw0: base.join("ebe_pw0.parquet"),
            chan_out: base.join("chan.out.parquet"),
            chanwb: base.join("chanwb.parquet"),
            chnwb: base.join("chnwb.parquet"),
            soil_pw0: base.join("soil_pw0.parquet"),
            totalwatsed3: base.join("totalwatsed3.parquet"),
            loss_hill: base.join("loss_pw0.hill.parquet"),
            loss_chn: base.join("loss_pw0.chn.parquet"),
            loss_out: base.join("loss_pw0.out.parquet"),
            loss_class_data: base.join("loss_pw0.class_data.parquet"),
            loss_all_years_hill: base.join("loss_pw0.all_years.hill.parquet"),
            loss_all_years_chn: base.join("loss_pw0.all_years.chn.parquet"),
            loss_all_years_out: base.join("loss_pw0.all_years.out.parquet"),
            loss_all_years_class_data: base.join("loss_pw0.all_years.class_data.parquet"),
        }
    }

    fn required_paths(config: &WatershedOutputConfig) -> [PathBuf; 14] {
        [
            config.ebe_pw0.clone(),
            config.chan_out.clone(),
            config.chanwb.clone(),
            config.chnwb.clone(),
            config.soil_pw0.clone(),
            config.totalwatsed3.clone(),
            config.loss_hill.clone(),
            config.loss_chn.clone(),
            config.loss_out.clone(),
            config.loss_class_data.clone(),
            config.loss_all_years_hill.clone(),
            config.loss_all_years_chn.clone(),
            config.loss_all_years_out.clone(),
            config.loss_all_years_class_data.clone(),
        ]
    }

    #[test]
    fn writer_emits_all_required_watershed_parquet_outputs() {
        let timestamp = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("clock should be monotonic")
            .as_nanos();
        let base = std::env::temp_dir().join(format!("openwepp_watershed_output_{timestamp}"));
        let config = sample_config(&base);

        write_interchange_parquet_outputs(&config, WatershedInterchangeRowSeed::default())
            .expect("writer should emit watershed parquet outputs");
        for output in required_paths(&config) {
            assert!(output.exists(), "expected output file {}", output.display());
            let file = File::open(&output).expect("emitted output should be readable");
            let builder = ParquetRecordBatchReaderBuilder::try_new(file)
                .expect("emitted output should include readable parquet footer");

            assert!(
                builder.schema().metadata().contains_key("dataset_version"),
                "missing dataset_version metadata for {}",
                output.display()
            );

            let reader = builder
                .build()
                .expect("parquet reader should build for emitted output");
            let row_count: usize = reader
                .map(|batch| batch.expect("record batch should decode").num_rows())
                .sum();
            assert!(
                row_count > 0,
                "emitted output should contain at least one row: {}",
                output.display()
            );
        }

        if base.exists() {
            fs::remove_dir_all(base).expect("temp directory cleanup should succeed");
        }
    }
}
