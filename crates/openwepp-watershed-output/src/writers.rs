use std::collections::HashMap;

use arrow_schema::{DataType, Field, Schema};

use crate::contracts::WatershedOutputConfig;

const INTERCHANGE_VERSION_MAJOR: u32 = 1;
const INTERCHANGE_VERSION_MINOR: u32 = 2;

pub type WatershedWriterError = String;

pub fn write_interchange_parquet_outputs(
    _outputs: &WatershedOutputConfig,
) -> Result<(), WatershedWriterError> {
    let _ = [
        watershed_ebe_schema(),
        watershed_chan_peak_schema(),
        watershed_chanwb_schema(),
        watershed_chnwb_schema(),
        watershed_soil_schema(),
        watershed_totalwatsed3_schema(),
        watershed_loss_average_hill_schema(),
        watershed_loss_average_chn_schema(),
        watershed_loss_average_out_schema(),
        watershed_loss_average_class_schema(),
        watershed_loss_all_years_hill_schema(),
        watershed_loss_all_years_chn_schema(),
        watershed_loss_all_years_out_schema(),
        watershed_loss_all_years_class_schema(),
    ];
    Err(
        "OWSOUT-E-004 watershed interchange emission is not implemented; refusing empty placeholder parquet outputs"
            .to_string(),
    )
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

fn schema_with_interchange_version(schema: Schema) -> Schema {
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
    schema.with_metadata(metadata)
}

fn watershed_ebe_schema() -> Schema {
    schema_with_interchange_version(Schema::new(vec![
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
    ]))
}

fn watershed_chan_peak_schema() -> Schema {
    schema_with_interchange_version(Schema::new(vec![
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
    ]))
}

fn watershed_chanwb_schema() -> Schema {
    schema_with_interchange_version(Schema::new(vec![
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
    ]))
}

#[allow(clippy::too_many_lines)]
fn watershed_chnwb_schema() -> Schema {
    schema_with_interchange_version(Schema::new(vec![
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
    ]))
}

fn watershed_soil_schema() -> Schema {
    schema_with_interchange_version(Schema::new(vec![
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
    ]))
}

#[allow(clippy::too_many_lines)]
fn watershed_totalwatsed3_schema() -> Schema {
    schema_with_interchange_version(Schema::new(vec![
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
            Some("Area-weighted full-profile soil water depth (watcon + frozwt)"),
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
            Some("Fraction of ash solids volume that is black ash (percent of total ash volume)"),
        ),
    ]))
}

fn watershed_loss_all_years_hill_schema() -> Schema {
    schema_with_interchange_version(
        Schema::new(vec![
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

fn watershed_loss_average_hill_schema() -> Schema {
    schema_with_interchange_version(
        Schema::new(vec![
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

fn watershed_loss_all_years_chn_schema() -> Schema {
    schema_with_interchange_version(
        Schema::new(vec![
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

fn watershed_loss_average_chn_schema() -> Schema {
    schema_with_interchange_version(
        Schema::new(vec![
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

fn watershed_loss_all_years_out_schema() -> Schema {
    schema_with_interchange_version(
        Schema::new(vec![
            field("year", DataType::Int16),
            field("key", DataType::Utf8),
            field("value", DataType::Float64),
            field("units", DataType::Utf8),
        ])
        .with_metadata(loss_table_metadata("loss_pw0.all_years.out")),
    )
}

fn watershed_loss_average_out_schema() -> Schema {
    schema_with_interchange_version(
        Schema::new(vec![
            field("key", DataType::Utf8),
            field("value", DataType::Float64),
            field("units", DataType::Utf8),
        ])
        .with_metadata(loss_table_metadata("loss_pw0.out")),
    )
}

fn watershed_loss_all_years_class_schema() -> Schema {
    schema_with_interchange_version(
        Schema::new(vec![
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
                Some(""),
                None,
            ),
        ])
        .with_metadata(loss_table_metadata("loss_pw0.all_years.class_data")),
    )
}

fn watershed_loss_average_class_schema() -> Schema {
    schema_with_interchange_version(
        Schema::new(vec![
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
                Some(""),
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

#[cfg(test)]
mod tests {
    use super::*;
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
    fn writer_rejects_placeholder_emission_with_typed_guard() {
        let timestamp = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("clock should be monotonic")
            .as_nanos();
        let base = std::env::temp_dir().join(format!("openwepp_watershed_output_{timestamp}"));
        let config = sample_config(&base);

        let error = write_interchange_parquet_outputs(&config)
            .expect_err("writer must refuse placeholder empty parquet emission");
        assert!(
            error.contains("OWSOUT-E-004"),
            "typed guard code missing from error: {error}"
        );
        for output in required_paths(&config) {
            assert!(
                !output.exists(),
                "no output file should be created while writer is gated"
            );
        }

        if base.exists() {
            std::fs::remove_dir_all(base).expect("temp directory cleanup should succeed");
        }
    }
}
