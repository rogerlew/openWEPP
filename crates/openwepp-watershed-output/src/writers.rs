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
use openwepp_watershed_orchestrator::WatershedPublicationFrame;
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
    pub area_m2: f64,
    pub subsurface_runoff_volume_m3: f64,
    pub total_detachment_kg: f64,
    pub total_deposition_kg: f64,
    pub sediment_class_deposition_kg: [f64; 5],
    pub sediment_volume_concentration_m3_m3: f64,
    pub precipitation_mm: f64,
    pub rain_melt_mm: f64,
    pub runoff_mm: f64,
    pub q_diagnostic_mm: Option<f64>,
    pub deep_percolation_mm: f64,
    pub lateral_flow_mm: f64,
    pub qofe_mm: f64,
    pub transpiration_mm: f64,
    pub evaporation_soil_mm: f64,
    pub evaporation_residue_mm: f64,
    pub upstream_q_mm: f64,
    pub subsurface_runon_mm: f64,
    pub total_soil_water_mm: f64,
    pub soil_water_total_mm: f64,
    pub profile_depth_mm: f64,
    pub profile_porosity_cap_mm: f64,
    pub profile_fc_store_mm: f64,
    pub profile_wp_store_mm: f64,
    pub interception_mm: f64,
    pub interception_storage_mm: f64,
    pub frozen_water_mm: f64,
    pub snow_water_mm: f64,
    pub tile_mm: f64,
    pub irrigation_mm: f64,
    pub baseflow_mm: f64,
    pub tsmf_fraction: f64,
    pub qrain_mm: f64,
    pub qsnow_mm: f64,
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
            area_m2: 0.0,
            subsurface_runoff_volume_m3: 0.0,
            total_detachment_kg: 0.0,
            total_deposition_kg: 0.0,
            sediment_class_deposition_kg: [0.0; 5],
            sediment_volume_concentration_m3_m3: 0.0,
            precipitation_mm: 0.0,
            rain_melt_mm: 0.0,
            runoff_mm: 0.0,
            q_diagnostic_mm: None,
            deep_percolation_mm: 0.0,
            lateral_flow_mm: 0.0,
            qofe_mm: 0.0,
            transpiration_mm: 0.0,
            evaporation_soil_mm: 0.0,
            evaporation_residue_mm: 0.0,
            upstream_q_mm: 0.0,
            subsurface_runon_mm: 0.0,
            total_soil_water_mm: 0.0,
            soil_water_total_mm: 0.0,
            profile_depth_mm: 0.0,
            profile_porosity_cap_mm: 0.0,
            profile_fc_store_mm: 0.0,
            profile_wp_store_mm: 0.0,
            interception_mm: 0.0,
            interception_storage_mm: 0.0,
            frozen_water_mm: 0.0,
            snow_water_mm: 0.0,
            tile_mm: 0.0,
            irrigation_mm: 0.0,
            baseflow_mm: 0.0,
            tsmf_fraction: 0.0,
            qrain_mm: 0.0,
            qsnow_mm: 0.0,
        }
    }
}

trait WatershedOutputRecord {
    fn year(&self) -> i16;
    fn simulation_year(&self) -> i16;
    fn sim_day_index(&self) -> i32;
    fn julian(&self) -> i16;
    fn month(&self) -> i8;
    fn day_of_month(&self) -> i8;
    fn water_year(&self) -> i16;
    fn element_id(&self) -> i32;
    fn channel_id(&self) -> i32;
    fn runoff_volume_m3(&self) -> Option<f64>;
    fn peak_discharge_m3_s(&self) -> Option<f64>;
    fn sediment_yield_kg(&self) -> Option<f64>;
    fn soluble_pollutant_kg(&self) -> Option<f64>;
    fn particulate_pollutant_kg(&self) -> Option<f64>;
    fn channel_inflow_m3(&self) -> Option<f64>;
    fn channel_outflow_m3(&self) -> Option<f64>;
    fn channel_storage_m3(&self) -> Option<f64>;
    fn channel_baseflow_m3(&self) -> Option<f64>;
    fn channel_loss_m3(&self) -> Option<f64>;
    fn area_m2(&self) -> Option<f64>;
    fn subsurface_runoff_volume_m3(&self) -> Option<f64>;
    fn total_detachment_kg(&self) -> Option<f64>;
    fn total_deposition_kg(&self) -> Option<f64>;
    fn sediment_class_deposition_kg(&self, class_index: usize) -> Option<f64>;
    fn sediment_volume_concentration_m3_m3(&self) -> Option<f64>;
    fn precipitation_mm(&self) -> Option<f64>;
    fn rain_melt_mm(&self) -> Option<f64>;
    fn runoff_mm(&self) -> Option<f64>;
    fn q_diagnostic_mm(&self) -> Option<f64>;
    fn deep_percolation_mm(&self) -> Option<f64>;
    fn lateral_flow_mm(&self) -> Option<f64>;
    fn qofe_mm(&self) -> Option<f64>;
    fn transpiration_mm(&self) -> Option<f64>;
    fn evaporation_soil_mm(&self) -> Option<f64>;
    fn evaporation_residue_mm(&self) -> Option<f64>;
    fn upstream_q_mm(&self) -> Option<f64>;
    fn subsurface_runon_mm(&self) -> Option<f64>;
    fn total_soil_water_mm(&self) -> Option<f64>;
    fn soil_water_total_mm(&self) -> Option<f64>;
    fn profile_depth_mm(&self) -> Option<f64>;
    fn profile_porosity_cap_mm(&self) -> Option<f64>;
    fn profile_fc_store_mm(&self) -> Option<f64>;
    fn profile_wp_store_mm(&self) -> Option<f64>;
    fn interception_mm(&self) -> Option<f64>;
    fn interception_storage_mm(&self) -> Option<f64>;
    fn frozen_water_mm(&self) -> Option<f64>;
    fn snow_water_mm(&self) -> Option<f64>;
    fn tile_mm(&self) -> Option<f64>;
    fn irrigation_mm(&self) -> Option<f64>;
    fn baseflow_mm(&self) -> Option<f64>;
    fn tsmf_fraction(&self) -> Option<f64>;
    fn qrain_mm(&self) -> Option<f64>;
    fn qsnow_mm(&self) -> Option<f64>;
}

impl WatershedOutputRecord for WatershedInterchangeRowSeed {
    fn year(&self) -> i16 {
        self.year
    }

    fn simulation_year(&self) -> i16 {
        self.simulation_year
    }

    fn sim_day_index(&self) -> i32 {
        self.sim_day_index
    }

    fn julian(&self) -> i16 {
        self.julian
    }

    fn month(&self) -> i8 {
        self.month
    }

    fn day_of_month(&self) -> i8 {
        self.day_of_month
    }

    fn water_year(&self) -> i16 {
        self.water_year
    }

    fn element_id(&self) -> i32 {
        self.element_id
    }

    fn channel_id(&self) -> i32 {
        self.channel_id
    }

    fn runoff_volume_m3(&self) -> Option<f64> {
        Some(self.runoff_volume_m3)
    }

    fn peak_discharge_m3_s(&self) -> Option<f64> {
        Some(self.peak_discharge_m3_s)
    }

    fn sediment_yield_kg(&self) -> Option<f64> {
        Some(self.sediment_yield_kg)
    }

    fn soluble_pollutant_kg(&self) -> Option<f64> {
        Some(self.soluble_pollutant_kg)
    }

    fn particulate_pollutant_kg(&self) -> Option<f64> {
        Some(self.particulate_pollutant_kg)
    }

    fn channel_inflow_m3(&self) -> Option<f64> {
        Some(self.runoff_volume_m3)
    }

    fn channel_outflow_m3(&self) -> Option<f64> {
        Some(self.channel_outflow_m3)
    }

    fn channel_storage_m3(&self) -> Option<f64> {
        Some(self.channel_storage_m3)
    }

    fn channel_baseflow_m3(&self) -> Option<f64> {
        Some(self.channel_baseflow_m3)
    }

    fn channel_loss_m3(&self) -> Option<f64> {
        Some(self.channel_loss_m3)
    }

    fn area_m2(&self) -> Option<f64> {
        Some(self.area_m2)
    }

    fn precipitation_mm(&self) -> Option<f64> {
        Some(self.precipitation_mm)
    }

    fn rain_melt_mm(&self) -> Option<f64> {
        Some(self.rain_melt_mm)
    }

    fn runoff_mm(&self) -> Option<f64> {
        Some(self.runoff_mm)
    }

    fn deep_percolation_mm(&self) -> Option<f64> {
        Some(self.deep_percolation_mm)
    }

    fn lateral_flow_mm(&self) -> Option<f64> {
        Some(self.lateral_flow_mm)
    }

    fn qofe_mm(&self) -> Option<f64> {
        Some(self.qofe_mm)
    }

    fn transpiration_mm(&self) -> Option<f64> {
        Some(self.transpiration_mm)
    }

    fn evaporation_soil_mm(&self) -> Option<f64> {
        Some(self.evaporation_soil_mm)
    }

    fn evaporation_residue_mm(&self) -> Option<f64> {
        Some(self.evaporation_residue_mm)
    }

    fn upstream_q_mm(&self) -> Option<f64> {
        Some(self.upstream_q_mm)
    }

    fn subsurface_runon_mm(&self) -> Option<f64> {
        Some(self.subsurface_runon_mm)
    }

    fn total_soil_water_mm(&self) -> Option<f64> {
        Some(self.total_soil_water_mm)
    }

    fn soil_water_total_mm(&self) -> Option<f64> {
        Some(self.soil_water_total_mm)
    }

    fn profile_depth_mm(&self) -> Option<f64> {
        Some(self.profile_depth_mm)
    }

    fn profile_porosity_cap_mm(&self) -> Option<f64> {
        Some(self.profile_porosity_cap_mm)
    }

    fn profile_fc_store_mm(&self) -> Option<f64> {
        Some(self.profile_fc_store_mm)
    }

    fn profile_wp_store_mm(&self) -> Option<f64> {
        Some(self.profile_wp_store_mm)
    }

    fn interception_mm(&self) -> Option<f64> {
        Some(self.interception_mm)
    }

    fn interception_storage_mm(&self) -> Option<f64> {
        Some(self.interception_storage_mm)
    }

    fn frozen_water_mm(&self) -> Option<f64> {
        Some(self.frozen_water_mm)
    }

    fn snow_water_mm(&self) -> Option<f64> {
        Some(self.snow_water_mm)
    }

    fn tile_mm(&self) -> Option<f64> {
        Some(self.tile_mm)
    }

    fn irrigation_mm(&self) -> Option<f64> {
        Some(self.irrigation_mm)
    }

    fn baseflow_mm(&self) -> Option<f64> {
        Some(self.baseflow_mm)
    }

    fn q_diagnostic_mm(&self) -> Option<f64> {
        self.q_diagnostic_mm
    }

    fn subsurface_runoff_volume_m3(&self) -> Option<f64> {
        Some(self.subsurface_runoff_volume_m3)
    }

    fn total_detachment_kg(&self) -> Option<f64> {
        Some(self.total_detachment_kg)
    }

    fn total_deposition_kg(&self) -> Option<f64> {
        Some(self.total_deposition_kg)
    }

    fn sediment_class_deposition_kg(&self, class_index: usize) -> Option<f64> {
        Some(
            self.sediment_class_deposition_kg
                .get(class_index)
                .copied()
                .unwrap_or(0.0),
        )
    }

    fn sediment_volume_concentration_m3_m3(&self) -> Option<f64> {
        Some(self.sediment_volume_concentration_m3_m3)
    }

    fn tsmf_fraction(&self) -> Option<f64> {
        Some(self.tsmf_fraction)
    }

    fn qrain_mm(&self) -> Option<f64> {
        Some(self.qrain_mm)
    }

    fn qsnow_mm(&self) -> Option<f64> {
        Some(self.qsnow_mm)
    }
}

impl WatershedOutputRecord for WatershedPublicationFrame {
    fn year(&self) -> i16 {
        self.year
    }

    fn simulation_year(&self) -> i16 {
        self.simulation_year
    }

    fn sim_day_index(&self) -> i32 {
        self.sim_day_index
    }

    fn julian(&self) -> i16 {
        self.julian
    }

    fn month(&self) -> i8 {
        self.month
    }

    fn day_of_month(&self) -> i8 {
        self.day_of_month
    }

    fn water_year(&self) -> i16 {
        self.water_year
    }

    fn element_id(&self) -> i32 {
        self.element_id
    }

    fn channel_id(&self) -> i32 {
        self.channel_id
    }

    fn runoff_volume_m3(&self) -> Option<f64> {
        Some(self.runoff_volume_m3)
    }

    fn peak_discharge_m3_s(&self) -> Option<f64> {
        Some(self.peak_discharge_m3_s)
    }

    fn sediment_yield_kg(&self) -> Option<f64> {
        Some(self.sediment_yield_kg)
    }

    fn soluble_pollutant_kg(&self) -> Option<f64> {
        self.soluble_pollutant_kg
    }

    fn particulate_pollutant_kg(&self) -> Option<f64> {
        self.particulate_pollutant_kg
    }

    fn channel_inflow_m3(&self) -> Option<f64> {
        self.channel_inflow_m3
    }

    fn channel_outflow_m3(&self) -> Option<f64> {
        self.channel_outflow_m3
    }

    fn channel_storage_m3(&self) -> Option<f64> {
        self.channel_storage_m3
    }

    fn channel_baseflow_m3(&self) -> Option<f64> {
        self.channel_baseflow_m3
    }

    fn channel_loss_m3(&self) -> Option<f64> {
        self.channel_loss_m3
    }

    fn area_m2(&self) -> Option<f64> {
        self.area_m2
    }

    fn precipitation_mm(&self) -> Option<f64> {
        self.precipitation_mm
    }

    fn rain_melt_mm(&self) -> Option<f64> {
        self.rain_melt_mm
    }

    fn runoff_mm(&self) -> Option<f64> {
        self.runoff_mm
    }

    fn deep_percolation_mm(&self) -> Option<f64> {
        self.deep_percolation_mm
    }

    fn lateral_flow_mm(&self) -> Option<f64> {
        self.lateral_flow_mm
    }

    fn qofe_mm(&self) -> Option<f64> {
        self.qofe_mm
    }

    fn transpiration_mm(&self) -> Option<f64> {
        self.transpiration_mm
    }

    fn evaporation_soil_mm(&self) -> Option<f64> {
        self.evaporation_soil_mm
    }

    fn evaporation_residue_mm(&self) -> Option<f64> {
        self.evaporation_residue_mm
    }

    fn upstream_q_mm(&self) -> Option<f64> {
        self.upstream_q_mm
    }

    fn subsurface_runon_mm(&self) -> Option<f64> {
        self.subsurface_runon_mm
    }

    fn total_soil_water_mm(&self) -> Option<f64> {
        self.total_soil_water_mm
    }

    fn soil_water_total_mm(&self) -> Option<f64> {
        self.soil_water_total_mm
    }

    fn profile_depth_mm(&self) -> Option<f64> {
        self.profile_depth_mm
    }

    fn profile_porosity_cap_mm(&self) -> Option<f64> {
        self.profile_porosity_cap_mm
    }

    fn profile_fc_store_mm(&self) -> Option<f64> {
        self.profile_fc_store_mm
    }

    fn profile_wp_store_mm(&self) -> Option<f64> {
        self.profile_wp_store_mm
    }

    fn interception_mm(&self) -> Option<f64> {
        self.interception_mm
    }

    fn interception_storage_mm(&self) -> Option<f64> {
        self.interception_storage_mm
    }

    fn frozen_water_mm(&self) -> Option<f64> {
        self.frozen_water_mm
    }

    fn snow_water_mm(&self) -> Option<f64> {
        self.snow_water_mm
    }

    fn tile_mm(&self) -> Option<f64> {
        self.tile_mm
    }

    fn irrigation_mm(&self) -> Option<f64> {
        self.irrigation_mm
    }

    fn baseflow_mm(&self) -> Option<f64> {
        self.baseflow_mm
    }

    fn q_diagnostic_mm(&self) -> Option<f64> {
        self.q_diagnostic_mm
    }

    fn subsurface_runoff_volume_m3(&self) -> Option<f64> {
        self.subsurface_runoff_volume_m3
    }

    fn total_detachment_kg(&self) -> Option<f64> {
        Some(self.total_detachment_kg)
    }

    fn total_deposition_kg(&self) -> Option<f64> {
        Some(self.total_deposition_kg)
    }

    fn sediment_class_deposition_kg(&self, class_index: usize) -> Option<f64> {
        self.sediment_class_deposition_kg
            .and_then(|values| values.get(class_index).copied())
    }

    fn sediment_volume_concentration_m3_m3(&self) -> Option<f64> {
        self.sediment_volume_concentration_m3_m3
    }

    fn tsmf_fraction(&self) -> Option<f64> {
        self.tsmf_fraction
    }

    fn qrain_mm(&self) -> Option<f64> {
        self.qrain_mm
    }

    fn qsnow_mm(&self) -> Option<f64> {
        self.qsnow_mm
    }
}

pub fn write_interchange_parquet_outputs(
    outputs: &WatershedOutputConfig,
    row_seed: WatershedInterchangeRowSeed,
) -> Result<(), WatershedWriterError> {
    write_interchange_parquet_outputs_from_rows(outputs, &[row_seed])
}

pub fn write_interchange_parquet_outputs_from_rows(
    outputs: &WatershedOutputConfig,
    row_seeds: &[WatershedInterchangeRowSeed],
) -> Result<(), WatershedWriterError> {
    write_output_record_parquet_outputs(outputs, row_seeds)
}

pub fn write_typed_publication_parquet_outputs(
    outputs: &WatershedOutputConfig,
    publication_frames: &[WatershedPublicationFrame],
) -> Result<(), WatershedWriterError> {
    write_output_record_parquet_outputs(outputs, publication_frames)
}

fn write_output_record_parquet_outputs<T>(
    outputs: &WatershedOutputConfig,
    records: &[T],
) -> Result<(), WatershedWriterError>
where
    T: WatershedOutputRecord,
{
    write_schema_output(&outputs.ebe_pw0, watershed_ebe_schema(), records)?;
    write_schema_output(&outputs.chan_out, watershed_chan_peak_schema(), records)?;
    write_schema_output(&outputs.chanwb, watershed_chanwb_schema(), records)?;
    write_schema_output(&outputs.chnwb, watershed_chnwb_schema(), records)?;
    write_schema_output(&outputs.soil_pw0, watershed_soil_schema(), records)?;
    write_schema_output(
        &outputs.totalwatsed3,
        watershed_totalwatsed3_schema(),
        records,
    )?;
    write_schema_output(
        &outputs.loss_hill,
        watershed_loss_average_hill_schema(),
        records,
    )?;
    write_schema_output(
        &outputs.loss_chn,
        watershed_loss_average_chn_schema(),
        records,
    )?;
    write_schema_output(
        &outputs.loss_out,
        watershed_loss_average_out_schema(),
        records,
    )?;
    write_schema_output(
        &outputs.loss_class_data,
        watershed_loss_average_class_schema(),
        records,
    )?;
    write_schema_output(
        &outputs.loss_all_years_hill,
        watershed_loss_all_years_hill_schema(),
        records,
    )?;
    write_schema_output(
        &outputs.loss_all_years_chn,
        watershed_loss_all_years_chn_schema(),
        records,
    )?;
    write_schema_output(
        &outputs.loss_all_years_out,
        watershed_loss_all_years_out_schema(),
        records,
    )?;
    write_schema_output(
        &outputs.loss_all_years_class_data,
        watershed_loss_all_years_class_schema(),
        records,
    )?;
    Ok(())
}

fn write_schema_output(
    path: &Path,
    schema: Result<Schema, WatershedWriterError>,
    records: &[impl WatershedOutputRecord],
) -> Result<(), WatershedWriterError> {
    write_single_output(path, schema?, records)
}

pub fn write_totalwatsed3_parquet(
    output: &Path,
    row_seeds: &[WatershedInterchangeRowSeed],
) -> Result<(), WatershedWriterError> {
    write_single_output(output, watershed_totalwatsed3_schema()?, row_seeds)
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
                "Interception",
                DataType::Float64,
                Some("mm"),
                Some("Daily canopy/residue interception flux depth"),
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
    records: &[impl WatershedOutputRecord],
) -> Result<(), WatershedWriterError> {
    if records.is_empty() {
        return Err(WatershedWriterError::Parquet {
            code: "OWSOUT-E-004",
            path: path.to_path_buf(),
            detail: "watershed output requires at least one routed row".to_string(),
        });
    }

    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent).map_err(|source| WatershedWriterError::Io {
            code: "OWSOUT-E-003",
            path: parent.to_path_buf(),
            source,
        })?;
    }

    let batch = build_row_batch(&schema, records)?;
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

fn build_row_batch(
    schema: &Schema,
    records: &[impl WatershedOutputRecord],
) -> Result<RecordBatch, WatershedWriterError> {
    let mut columns: Vec<ArrayRef> = Vec::with_capacity(schema.fields().len());

    for field in schema.fields() {
        let column: ArrayRef = match field.data_type() {
            DataType::Int8 => Arc::new(Int8Array::from(
                records
                    .iter()
                    .map(|record| int8_value(field.name(), record))
                    .collect::<Vec<_>>(),
            )),
            DataType::Int16 => Arc::new(Int16Array::from(
                records
                    .iter()
                    .map(|record| int16_value(field.name(), record))
                    .collect::<Vec<_>>(),
            )),
            DataType::Int32 => Arc::new(Int32Array::from(
                records
                    .iter()
                    .map(|record| int32_value(field.name(), record))
                    .collect::<Vec<_>>(),
            )),
            DataType::Float64 => Arc::new(Float64Array::from(
                records
                    .iter()
                    .map(|record| float64_value(field.name(), record))
                    .collect::<Vec<Option<f64>>>(),
            )),
            DataType::Utf8 => Arc::new(StringArray::from(
                records
                    .iter()
                    .map(|_| utf8_value(field.name()))
                    .collect::<Vec<_>>(),
            )),
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

fn int8_value(field_name: &str, record: &impl WatershedOutputRecord) -> i8 {
    match field_name {
        "month" => record.month(),
        "day_of_month" => record.day_of_month(),
        "Class" => 1,
        _ => 0,
    }
}

fn int16_value(field_name: &str, record: &impl WatershedOutputRecord) -> i16 {
    match field_name {
        "year" => record.year(),
        "simulation_year" | "Y" => record.simulation_year(),
        "day" | "julian" | "J" => record.julian(),
        "water_year" => record.water_year(),
        "ofe_id" | "OFE" => 1,
        _ => 0,
    }
}

fn int32_value(field_name: &str, record: &impl WatershedOutputRecord) -> i32 {
    match field_name {
        "sim_day_index" => record.sim_day_index(),
        "element_id" | "Elmt_ID" | "wepp_id" => record.element_id(),
        "Chan_ID" | "chn_enum" => record.channel_id(),
        _ => 0,
    }
}

enum Float64FieldMatch {
    Unmatched,
    Matched(Option<f64>),
}

fn float64_value(field_name: &str, record: &impl WatershedOutputRecord) -> Option<f64> {
    if let Float64FieldMatch::Matched(value) = float64_runoff_sediment_value(field_name, record) {
        return value;
    }
    if let Float64FieldMatch::Matched(value) = float64_hydrology_volume_value(field_name, record) {
        return value;
    }
    if let Float64FieldMatch::Matched(value) = float64_hydrology_flux_value(field_name, record) {
        return value;
    }
    if let Float64FieldMatch::Matched(value) = float64_hydrology_storage_value(field_name, record) {
        return value;
    }
    match float64_storage_channel_value(field_name, record) {
        Float64FieldMatch::Matched(value) => value,
        Float64FieldMatch::Unmatched => None,
    }
}

fn float64_runoff_sediment_value(
    field_name: &str,
    record: &impl WatershedOutputRecord,
) -> Float64FieldMatch {
    let total_pollutant = option_sum2(
        record.soluble_pollutant_kg(),
        record.particulate_pollutant_kg(),
    );
    let sediment_yield_tonnes = record.sediment_yield_kg().map(|value| value / 1_000.0);
    let value = match field_name {
        "runoff_volume" | "runvol" | "Runoff Volume" | "Discharge Volume" => {
            record.runoff_volume_m3()
        }
        "sbrunv" => record.subsurface_runoff_volume_m3(),
        "tdet" => record.total_detachment_kg(),
        "tdep" => record.total_deposition_kg(),
        "seddep_1" => record.sediment_class_deposition_kg(0),
        "seddep_2" => record.sediment_class_deposition_kg(1),
        "seddep_3" => record.sediment_class_deposition_kg(2),
        "seddep_4" => record.sediment_class_deposition_kg(3),
        "seddep_5" => record.sediment_class_deposition_kg(4),
        "sed_vol_conc" => record.sediment_volume_concentration_m3_m3(),
        "peak_runoff" | "Peak_Discharge (m^3/s)" => record.peak_discharge_m3_s(),
        "sediment_yield" | "sed_del" => record.sediment_yield_kg(),
        "Sediment Yield" => sediment_yield_tonnes,
        "soluble_pollutant" | "Solub. React. Pollutant" => record.soluble_pollutant_kg(),
        "particulate_pollutant" | "Particulate Pollutant" => record.particulate_pollutant_kg(),
        "total_pollutant" | "Total Pollutant" => total_pollutant,
        _ => return Float64FieldMatch::Unmatched,
    };
    Float64FieldMatch::Matched(value)
}

fn float64_hydrology_volume_value(
    field_name: &str,
    record: &impl WatershedOutputRecord,
) -> Float64FieldMatch {
    let volume_from_depth = |depth_mm: Option<f64>| {
        option_product2(depth_mm, record.area_m2()).map(|depth_area| depth_area / 1_000.0)
    };
    let q_diagnostic_mm = record.q_diagnostic_mm().or_else(|| record.runoff_mm());
    let value = match field_name {
        "P" => volume_from_depth(record.precipitation_mm()),
        "RM" => volume_from_depth(record.rain_melt_mm()),
        "Q" => volume_from_depth(q_diagnostic_mm),
        "Dp" => volume_from_depth(record.deep_percolation_mm()),
        "latqcc" => volume_from_depth(record.lateral_flow_mm()),
        "QOFE" => volume_from_depth(record.qofe_mm()),
        "Ep" => volume_from_depth(record.transpiration_mm()),
        "Es" => volume_from_depth(record.evaporation_soil_mm()),
        "Er" => volume_from_depth(record.evaporation_residue_mm()),
        _ => return Float64FieldMatch::Unmatched,
    };
    Float64FieldMatch::Matched(value)
}

fn float64_hydrology_flux_value(
    field_name: &str,
    record: &impl WatershedOutputRecord,
) -> Float64FieldMatch {
    let q_diagnostic_mm = record.q_diagnostic_mm().or_else(|| record.runoff_mm());
    let value = match field_name {
        "P (mm)" | "Precipitation" | "precip" => record.precipitation_mm(),
        "RM (mm)" | "Rain+Melt" => record.rain_melt_mm(),
        "Q (mm)" => q_diagnostic_mm,
        "Runoff" => record.runoff_mm(),
        "Dp (mm)" | "Percolation" => record.deep_percolation_mm(),
        "latqcc (mm)" | "Lateral Flow" => record.lateral_flow_mm(),
        "QOFE (mm)" => record.qofe_mm(),
        "Ep (mm)" | "Transpiration" => record.transpiration_mm(),
        "Es (mm)" => record.evaporation_soil_mm(),
        "Er (mm)" => record.evaporation_residue_mm(),
        "Evaporation" => option_sum2(
            record.evaporation_soil_mm(),
            record.evaporation_residue_mm(),
        ),
        "ET" => option_sum3(
            record.transpiration_mm(),
            record.evaporation_soil_mm(),
            record.evaporation_residue_mm(),
        ),
        _ => return Float64FieldMatch::Unmatched,
    };
    Float64FieldMatch::Matched(value)
}

fn float64_hydrology_storage_value(
    field_name: &str,
    record: &impl WatershedOutputRecord,
) -> Float64FieldMatch {
    let value = match field_name {
        "UpStrmQ" | "UpStrmQ (mm)" => record.upstream_q_mm(),
        "SubRIn" | "SubRIn (mm)" => record.subsurface_runon_mm(),
        "Total-Soil Water" | "Total Soil Water (mm)" | "TSW" => record.total_soil_water_mm(),
        "SoilWaterTotal" => record.soil_water_total_mm(),
        "ProfileDepth" => record.profile_depth_mm(),
        "ProfilePorosityCap" => record.profile_porosity_cap_mm(),
        "ProfileFCStore" => record.profile_fc_store_mm(),
        "ProfileWPStore" => record.profile_wp_store_mm(),
        "Interception" => record.interception_mm(),
        "InterceptionStorage" => record.interception_storage_mm(),
        "frozwt" | "frozwt (mm)" => record.frozen_water_mm(),
        "Snow-Water" | "Snow Water (mm)" => record.snow_water_mm(),
        "TSMF" => record.tsmf_fraction(),
        "QRain" => record.qrain_mm(),
        "QSnow" => record.qsnow_mm(),
        "Tile" | "Tile (mm)" => record.tile_mm(),
        "Irr" | "Irr (mm)" => record.irrigation_mm(),
        "Baseflow" | "Base (mm)" => record.baseflow_mm(),
        _ => return Float64FieldMatch::Unmatched,
    };
    Float64FieldMatch::Matched(value)
}

fn float64_storage_channel_value(
    field_name: &str,
    record: &impl WatershedOutputRecord,
) -> Float64FieldMatch {
    let value = match field_name {
        "Inflow (m^3)" => record.channel_inflow_m3(),
        "value" => record.runoff_volume_m3(),
        "Outflow (m^3)" => record.channel_outflow_m3(),
        "Storage (m^3)" => record.channel_storage_m3(),
        "Baseflow (m^3)" => record.channel_baseflow_m3(),
        "Loss (m^3)" => record.channel_loss_m3(),
        "Balance (m^3)" => option_balance(
            record.channel_inflow_m3(),
            record.channel_outflow_m3(),
            record.channel_loss_m3(),
            record.channel_storage_m3(),
        ),
        "Specific Gravity" => Some(2.65),
        "Fraction In Flow Exiting" => Some(1.0),
        "Area" | "Area (m^2)" => record.area_m2(),
        "Hillslope Area" | "Contributing Area" => record.area_m2().map(|area| area / 10_000.0),
        "Diameter" => Some(0.25),
        _ => return Float64FieldMatch::Unmatched,
    };
    Float64FieldMatch::Matched(value)
}

fn option_sum2(left: Option<f64>, right: Option<f64>) -> Option<f64> {
    Some(left? + right?)
}

fn option_sum3(first: Option<f64>, second: Option<f64>, third: Option<f64>) -> Option<f64> {
    Some(first? + second? + third?)
}

fn option_product2(left: Option<f64>, right: Option<f64>) -> Option<f64> {
    Some(left? * right?)
}

fn option_balance(
    inflow_m3: Option<f64>,
    outflow_m3: Option<f64>,
    loss_m3: Option<f64>,
    storage_m3: Option<f64>,
) -> Option<f64> {
    Some(inflow_m3? - outflow_m3? - loss_m3? - storage_m3?)
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
    use arrow_array::Array;
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

    fn int32_column<'a>(batch: &'a RecordBatch, name: &str) -> &'a Int32Array {
        let schema = batch.schema();
        batch
            .column(schema.index_of(name).expect("int32 column should exist"))
            .as_any()
            .downcast_ref::<Int32Array>()
            .expect("column should be int32")
    }

    fn float64_column<'a>(batch: &'a RecordBatch, name: &str) -> &'a Float64Array {
        let schema = batch.schema();
        batch
            .column(schema.index_of(name).expect("float64 column should exist"))
            .as_any()
            .downcast_ref::<Float64Array>()
            .expect("column should be float64")
    }

    fn assert_int32_values(batch: &RecordBatch, name: &str, expected: &[i32]) {
        let column = int32_column(batch, name);
        for (row, expected_value) in expected.iter().copied().enumerate() {
            assert_eq!(column.value(row), expected_value, "{name}[{row}]");
        }
    }

    fn assert_float64_values(batch: &RecordBatch, name: &str, expected: &[f64]) {
        let column = float64_column(batch, name);
        for (row, expected_value) in expected.iter().copied().enumerate() {
            assert!(
                (column.value(row) - expected_value).abs() <= 1.0e-12,
                "{name}[{row}] expected {expected_value}, observed {}",
                column.value(row)
            );
        }
    }

    fn assert_float64_nulls(batch: &RecordBatch, names: &[&str]) {
        for name in names {
            let column = float64_column(batch, name);
            assert_eq!(column.len(), 1, "{name} should have one test row");
            assert!(column.is_null(0), "{name}[0] should be null");
        }
    }

    fn temp_config(prefix: &str) -> (PathBuf, WatershedOutputConfig) {
        let timestamp = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("clock should be monotonic")
            .as_nanos();
        let base = std::env::temp_dir().join(format!("{prefix}_{timestamp}"));
        let config = sample_config(&base);
        (base, config)
    }

    fn first_batch(path: &Path, label: &str) -> RecordBatch {
        let file = File::open(path).unwrap_or_else(|_| panic!("{label} should be readable"));
        let builder = ParquetRecordBatchReaderBuilder::try_new(file)
            .unwrap_or_else(|_| panic!("{label} should include readable parquet footer"));
        let mut reader = builder
            .build()
            .unwrap_or_else(|_| panic!("{label} parquet reader should build"));
        reader
            .next()
            .unwrap_or_else(|| panic!("{label} should include a batch"))
            .unwrap_or_else(|_| panic!("{label} batch should decode"))
    }

    fn sample_typed_publication_frame() -> WatershedPublicationFrame {
        WatershedPublicationFrame {
            year: 2026,
            simulation_year: 3,
            sim_day_index: 77,
            julian: 121,
            month: 4,
            day_of_month: 30,
            water_year: 2026,
            element_id: 44,
            channel_id: 12,
            runoff_volume_m3: 25.0,
            peak_discharge_m3_s: 1.5,
            sediment_yield_kg: 6.0,
            soluble_pollutant_kg: Some(0.25),
            particulate_pollutant_kg: Some(0.75),
            channel_inflow_m3: Some(31.0),
            channel_outflow_m3: Some(20.0),
            channel_storage_m3: Some(2.0),
            channel_baseflow_m3: Some(1.0),
            channel_loss_m3: Some(3.0),
            area_m2: Some(5_000.0),
            subsurface_runoff_volume_m3: Some(0.75),
            total_detachment_kg: 9.0,
            total_deposition_kg: 3.0,
            sediment_class_deposition_kg: Some([0.1, 0.2, 0.3, 0.4, 0.5]),
            sediment_volume_concentration_m3_m3: Some(0.015),
            precipitation_mm: Some(10.0),
            rain_melt_mm: Some(8.0),
            runoff_mm: Some(5.0),
            q_diagnostic_mm: Some(5.0),
            deep_percolation_mm: Some(2.0),
            lateral_flow_mm: Some(1.0),
            qofe_mm: Some(4.0),
            transpiration_mm: Some(3.0),
            evaporation_soil_mm: Some(0.5),
            evaporation_residue_mm: Some(0.25),
            upstream_q_mm: Some(0.75),
            subsurface_runon_mm: Some(0.5),
            total_soil_water_mm: Some(180.0),
            soil_water_total_mm: Some(175.0),
            profile_depth_mm: Some(900.0),
            profile_porosity_cap_mm: Some(410.0),
            profile_fc_store_mm: Some(250.0),
            profile_wp_store_mm: Some(125.0),
            interception_mm: Some(0.4),
            interception_storage_mm: Some(0.2),
            frozen_water_mm: Some(12.0),
            snow_water_mm: Some(7.5),
            tile_mm: Some(0.1),
            irrigation_mm: Some(0.0),
            baseflow_mm: Some(0.2),
            tsmf_fraction: Some(0.6),
            qrain_mm: Some(2.5),
            qsnow_mm: Some(1.5),
        }
    }

    #[test]
    fn typed_publication_writer_reads_publication_frame_directly() {
        let (base, config) = temp_config("openwepp_typed_publication");
        let frame = sample_typed_publication_frame();

        write_typed_publication_parquet_outputs(&config, &[frame])
            .expect("typed publication writer should emit watershed parquet outputs");

        for output in required_paths(&config) {
            assert!(output.exists(), "expected output file {}", output.display());
        }

        let batch = first_batch(&config.totalwatsed3, "totalwatsed3");
        assert_int32_values(&batch, "sim_day_index", &[77]);
        assert_float64_values(&batch, "runvol", &[25.0]);
        assert_float64_values(&batch, "Runoff", &[5.0]);
        assert_float64_values(&batch, "P", &[50.0]);
        assert_float64_values(&batch, "Q", &[25.0]);
        assert_float64_values(&batch, "Dp", &[10.0]);
        assert_float64_values(&batch, "latqcc", &[5.0]);
        assert_float64_values(&batch, "tdet", &[9.0]);
        assert_float64_values(&batch, "tdep", &[3.0]);
        assert_float64_values(&batch, "sed_del", &[6.0]);

        let batch = first_batch(&config.ebe_pw0, "ebe_pw0");
        assert_float64_values(&batch, "precip", &[10.0]);

        let batch = first_batch(&config.chan_out, "chan.out");
        assert_int32_values(&batch, "Elmt_ID", &[44]);
        assert_int32_values(&batch, "Chan_ID", &[12]);
        assert_float64_values(&batch, "Peak_Discharge (m^3/s)", &[1.5]);

        let batch = first_batch(&config.chanwb, "chanwb");
        assert_float64_values(&batch, "Inflow (m^3)", &[31.0]);
        assert_float64_values(&batch, "Outflow (m^3)", &[20.0]);
        assert_float64_values(&batch, "Storage (m^3)", &[2.0]);
        assert_float64_values(&batch, "Baseflow (m^3)", &[1.0]);
        assert_float64_values(&batch, "Loss (m^3)", &[3.0]);
        assert_float64_values(&batch, "Balance (m^3)", &[6.0]);

        if base.exists() {
            fs::remove_dir_all(base).expect("temp directory cleanup should succeed");
        }
    }

    #[test]
    fn typed_publication_writer_keeps_unavailable_operands_null() {
        let (base, config) = temp_config("openwepp_typed_publication_nulls");

        write_typed_publication_parquet_outputs(&config, &[WatershedPublicationFrame::default()])
            .expect("typed publication writer should emit watershed parquet outputs");

        let batch = first_batch(&config.totalwatsed3, "totalwatsed3");
        assert_float64_nulls(&batch, &["sbrunv", "seddep_1", "sed_vol_conc", "Q", "Area"]);

        let batch = first_batch(&config.chanwb, "chanwb");
        assert_float64_nulls(
            &batch,
            &[
                "Inflow (m^3)",
                "Outflow (m^3)",
                "Storage (m^3)",
                "Baseflow (m^3)",
                "Loss (m^3)",
                "Balance (m^3)",
            ],
        );

        if base.exists() {
            fs::remove_dir_all(base).expect("temp directory cleanup should succeed");
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

    #[test]
    fn writer_preserves_multiple_watershed_daily_rows_and_wat_fields() {
        let timestamp = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("clock should be monotonic")
            .as_nanos();
        let base =
            std::env::temp_dir().join(format!("openwepp_watershed_output_multi_{timestamp}"));
        let config = sample_config(&base);
        let mut row_1 = WatershedInterchangeRowSeed {
            sim_day_index: 1,
            julian: 1,
            day_of_month: 1,
            area_m2: 2_000.0,
            precipitation_mm: 10.0,
            rain_melt_mm: 8.0,
            runoff_mm: 2.0,
            deep_percolation_mm: 0.5,
            lateral_flow_mm: 0.25,
            qofe_mm: 1.75,
            transpiration_mm: 1.5,
            evaporation_soil_mm: 0.4,
            evaporation_residue_mm: 0.1,
            runoff_volume_m3: 4.0,
            soil_water_total_mm: 120.0,
            profile_porosity_cap_mm: 240.0,
            interception_mm: 1.5,
            ..WatershedInterchangeRowSeed::default()
        };
        row_1.channel_outflow_m3 = row_1.runoff_volume_m3;
        let mut row_2 = WatershedInterchangeRowSeed {
            sim_day_index: 2,
            julian: 2,
            day_of_month: 2,
            area_m2: 2_000.0,
            precipitation_mm: 12.0,
            rain_melt_mm: 0.0,
            runoff_mm: 3.0,
            deep_percolation_mm: 0.75,
            lateral_flow_mm: 0.5,
            qofe_mm: 2.25,
            transpiration_mm: 1.0,
            evaporation_soil_mm: 0.3,
            evaporation_residue_mm: 0.2,
            runoff_volume_m3: 6.0,
            soil_water_total_mm: 118.0,
            profile_porosity_cap_mm: 238.0,
            interception_mm: 1.25,
            ..WatershedInterchangeRowSeed::default()
        };
        row_2.channel_outflow_m3 = row_2.runoff_volume_m3;

        write_interchange_parquet_outputs_from_rows(&config, &[row_1, row_2])
            .expect("multi-row writer should emit watershed parquet outputs");

        let file = File::open(&config.totalwatsed3).expect("totalwatsed3 should be readable");
        let builder = ParquetRecordBatchReaderBuilder::try_new(file)
            .expect("totalwatsed3 should include readable parquet footer");
        let mut reader = builder
            .build()
            .expect("totalwatsed3 parquet reader should build");
        let batch = reader
            .next()
            .expect("totalwatsed3 should include a batch")
            .expect("totalwatsed3 batch should decode");
        assert_eq!(batch.num_rows(), 2);

        assert_int32_values(&batch, "sim_day_index", &[1, 2]);
        assert_float64_values(&batch, "P", &[20.0, 24.0]);
        assert_float64_values(&batch, "Precipitation", &[10.0, 12.0]);
        assert_float64_values(&batch, "runvol", &[4.0, 6.0]);
        assert_float64_values(&batch, "Runoff", &[2.0, 3.0]);
        assert_float64_values(&batch, "RM", &[16.0, 0.0]);
        assert_float64_values(&batch, "Dp", &[1.0, 1.5]);
        assert_float64_values(&batch, "latqcc", &[0.5, 1.0]);
        assert_float64_values(&batch, "QOFE", &[3.5, 4.5]);
        assert_float64_values(&batch, "Ep", &[3.0, 2.0]);
        assert_float64_values(&batch, "Es", &[0.8, 0.6]);
        assert_float64_values(&batch, "Er", &[0.2, 0.4]);
        assert_float64_values(&batch, "SoilWaterTotal", &[120.0, 118.0]);
        assert_float64_values(&batch, "ProfilePorosityCap", &[240.0, 238.0]);
        assert_float64_values(&batch, "Interception", &[1.5, 1.25]);

        if base.exists() {
            fs::remove_dir_all(base).expect("temp directory cleanup should succeed");
        }
    }

    #[test]
    fn hb09_writer_public_helpers_and_error_taxonomy_are_characterized() {
        let errors = [
            WatershedWriterError::Io {
                code: "OWSOUT-E-001",
                path: PathBuf::from("missing/output.parquet"),
                source: io::Error::new(io::ErrorKind::NotFound, "missing"),
            },
            WatershedWriterError::Parquet {
                code: "OWSOUT-E-005",
                path: PathBuf::from("bad.parquet"),
                detail: "bad footer".to_string(),
            },
            WatershedWriterError::UnitMetadata {
                detail: "missing unit".to_string(),
            },
            WatershedWriterError::UnsupportedFieldType {
                field_name: "unsupported".to_string(),
                data_type: "Binary".to_string(),
            },
        ];
        for error in &errors {
            assert!(!error.to_string().is_empty());
            assert!(!error.code().is_empty());
        }
        assert!(std::error::Error::source(&errors[0]).is_some());
        assert!(std::error::Error::source(&errors[1]).is_none());
        assert!(std::error::Error::source(&errors[2]).is_none());
        assert!(std::error::Error::source(&errors[3]).is_none());

        let (base, _) = temp_config("hb09_public_helpers");
        fs::create_dir_all(&base).expect("temp directory");
        let total = base.join("totalwatsed3.parquet");
        write_totalwatsed3_parquet(&total, &[WatershedInterchangeRowSeed::default()])
            .expect("public totalwatsed3 helper writes");
        assert!(total.exists());
        let schemas = watershed_interchange_schemas().expect("schema inventory");
        assert_eq!(schemas.len(), 14);

        let blocked_parent = base.join("blocked");
        File::create(&blocked_parent).expect("blocking parent file");
        let missing_parent = blocked_parent.join("ebe.parquet");
        let error = write_single_output(
            &missing_parent,
            watershed_ebe_schema().expect("EBE schema"),
            &[WatershedInterchangeRowSeed::default()],
        )
        .expect_err("missing parent must preserve IO failure");
        assert_eq!(error.code(), "OWSOUT-E-003");
        assert!(std::error::Error::source(&error).is_some());

        let unsupported = write_single_output(
            &base.join("unsupported.parquet"),
            Schema::new(vec![field("unsupported", DataType::Binary)]),
            &[WatershedInterchangeRowSeed::default()],
        )
        .expect_err("unsupported Arrow types must fail before publication");
        assert_eq!(unsupported.code(), "OWSOUT-E-006");

        let output_directory = base.join("output-directory");
        fs::create_dir(&output_directory).expect("output directory");
        let create_error = write_single_output(
            &output_directory,
            watershed_ebe_schema().expect("EBE schema"),
            &[WatershedInterchangeRowSeed::default()],
        )
        .expect_err("a directory cannot be replaced by a parquet file");
        assert_eq!(create_error.code(), "OWSOUT-E-003");

        let registry_error = output_registry_error(&OutputUnitRegistryError::RegistryEmpty);
        assert_eq!(registry_error.code(), "OWOUT-UNIT-E-001");
        assert!(registry_error.to_string().contains("must contain entries"));

        let empty_error = write_single_output(
            &base.join("empty.parquet"),
            watershed_ebe_schema().expect("EBE schema"),
            &[] as &[WatershedInterchangeRowSeed],
        )
        .expect_err("empty watershed publication must be rejected");
        assert_eq!(empty_error.code(), "OWSOUT-E-004");
        fs::remove_dir_all(base).expect("temp cleanup");
    }
}
