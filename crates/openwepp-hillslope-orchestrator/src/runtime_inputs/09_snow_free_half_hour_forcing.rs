use openwepp_meteorology::snow_free_forcing::{
    SnowFreeAtmosphericError, atmospheric_longwave_dilley_unsworth, celsius_to_kelvin,
    fao56_station_pressure_kpa, liquid_specific_enthalpy_j_kg, weiss_norman_partition,
};
use openwepp_plant_phenology::{
    GsiDailyForcing, GsiDailyIndicators, GsiDailyResult, GsiDate, GsiError, GsiParameters,
    GsiState,
};
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use thiserror::Error;

const SNOW_FREE_INTERVAL_S: f64 = 1_800.0;
const SNOW_FREE_INTERVAL_COUNT: usize = 48;
const SNOW_FREE_PROVIDER_SHA256: &str =
    "4658de9f7590897633ffbfe0facedd52b5c9b9754f7d829f25869ef2c592f153";

#[derive(Clone, Debug, PartialEq)]
pub struct SnowFreeHalfHourDestination {
    pub ofe_id: String,
    pub tile_id: String,
    pub wb14_configuration_sha256: String,
}

#[derive(Clone, Debug, PartialEq)]
pub struct SnowFreeHalfHourProviderConfiguration {
    pub run_id: String,
    pub co2_pa: f64,
    pub reference_height_m: f64,
    pub gsi: f64,
    pub gsi_receipt_sha256: String,
    pub destinations: Vec<SnowFreeHalfHourDestination>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct SnowFreeHalfHourStaticConfiguration {
    pub run_id: String,
    pub co2_pa: f64,
    pub reference_height_m: f64,
    pub gsi_owner_configuration_sha256: String,
    pub destinations: Vec<SnowFreeHalfHourDestination>,
}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct DirectGsiStateV1 {
    pub history: Vec<f64>,
    pub last_year: Option<i32>,
    pub last_ordinal_day: Option<u16>,
    pub state_sha256: String,
}

#[derive(Clone, Copy, Debug, Deserialize, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct DirectGsiParametersV1 {
    pub minimum_temperature_inactive_c: f64,
    pub minimum_temperature_unconstrained_c: f64,
    pub vapor_pressure_deficit_unconstrained_pa: f64,
    pub vapor_pressure_deficit_inactive_pa: f64,
    pub photoperiod_inactive_hours: f64,
    pub photoperiod_unconstrained_hours: f64,
}

#[derive(Clone, Copy, Debug, Deserialize, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct DirectGsiForcingV1 {
    pub minimum_temperature_c: f64,
    pub vapor_pressure_deficit_pa: f64,
    pub latitude_degrees: f64,
    pub year: i32,
    pub ordinal_day: u16,
}

#[derive(Clone, Copy, Debug, Deserialize, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct DirectGsiResultV1 {
    pub minimum_temperature_indicator: f64,
    pub vapor_pressure_deficit_indicator: f64,
    pub photoperiod_indicator: f64,
    pub instantaneous_gsi: f64,
    pub photoperiod_hours: f64,
    pub growing_season_index: f64,
    pub sample_count: usize,
}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct DirectGsiDailyReceiptV1 {
    pub schema_version: String,
    pub beginning_state: DirectGsiStateV1,
    pub ending_state: DirectGsiStateV1,
    pub parameters: DirectGsiParametersV1,
    pub forcing: DirectGsiForcingV1,
    pub result: DirectGsiResultV1,
    pub configuration_sha256: String,
    pub forcing_sha256: String,
    pub result_sha256: String,
    pub receipt_sha256: String,
}

impl DirectGsiDailyReceiptV1 {
    pub fn configuration_sha256(
        parameters: GsiParameters,
    ) -> Result<String, SnowFreeHalfHourForcingError> {
        canonical_sha256(&direct_gsi_parameters(parameters))
    }

    pub fn prepare(
        beginning: &GsiState,
        parameters: GsiParameters,
        forcing: GsiDailyForcing,
    ) -> Result<(Self, GsiState), SnowFreeHalfHourForcingError> {
        let mut ending = beginning.clone();
        let result = ending.advance(parameters, forcing)?;
        let beginning_state = direct_gsi_state(beginning)?;
        let ending_state = direct_gsi_state(&ending)?;
        let parameters = direct_gsi_parameters(parameters);
        let forcing = direct_gsi_forcing(forcing);
        let result = direct_gsi_result(result);
        let mut receipt = Self {
            schema_version: "DIRECT_GSI_DAILY_RECEIPT_V1".into(),
            beginning_state,
            ending_state,
            configuration_sha256: canonical_sha256(&parameters)?,
            forcing_sha256: canonical_sha256(&forcing)?,
            result_sha256: canonical_sha256(&result)?,
            parameters,
            forcing,
            result,
            receipt_sha256: String::new(),
        };
        receipt.receipt_sha256 = canonical_sha256(&receipt)?;
        receipt.validate()?;
        Ok((receipt, ending))
    }

    pub fn validate(&self) -> Result<(), SnowFreeHalfHourForcingError> {
        if self.schema_version != "DIRECT_GSI_DAILY_RECEIPT_V1"
            || self.configuration_sha256 != canonical_sha256(&self.parameters)?
            || self.forcing_sha256 != canonical_sha256(&self.forcing)?
            || self.result_sha256 != canonical_sha256(&self.result)?
        {
            return Err(SnowFreeHalfHourForcingError::Identity("daily GSI receipt"));
        }
        let beginning = restore_direct_gsi_state(&self.beginning_state)?;
        let mut ending = beginning;
        let result = direct_gsi_result(
            ending.advance(gsi_parameters(self.parameters), gsi_forcing(self.forcing))?,
        );
        if direct_gsi_state(&ending)? != self.ending_state
            || result != self.result
            || self.receipt_sha256 != canonical_sha256_without_receipt(self)?
        {
            return Err(SnowFreeHalfHourForcingError::Identity("daily GSI closure"));
        }
        Ok(())
    }
}

fn direct_gsi_state(state: &GsiState) -> Result<DirectGsiStateV1, SnowFreeHalfHourForcingError> {
    let mut value = DirectGsiStateV1 {
        history: state.history(),
        last_year: state.last_date().map(|date| date.year),
        last_ordinal_day: state.last_date().map(|date| date.ordinal_day),
        state_sha256: String::new(),
    };
    value.state_sha256 = canonical_sha256(&value)?;
    Ok(value)
}

fn restore_direct_gsi_state(
    value: &DirectGsiStateV1,
) -> Result<GsiState, SnowFreeHalfHourForcingError> {
    let mut canonical = value.clone();
    canonical.state_sha256.clear();
    if value.state_sha256 != canonical_sha256(&canonical)?
        || value.last_year.is_some() != value.last_ordinal_day.is_some()
    {
        return Err(SnowFreeHalfHourForcingError::Identity("GSI state digest"));
    }
    let last_date = value
        .last_year
        .zip(value.last_ordinal_day)
        .map(|(year, ordinal_day)| GsiDate { year, ordinal_day });
    Ok(GsiState::try_from_history(&value.history, last_date)?)
}

const fn direct_gsi_parameters(value: GsiParameters) -> DirectGsiParametersV1 {
    DirectGsiParametersV1 {
        minimum_temperature_inactive_c: value.minimum_temperature_inactive_c,
        minimum_temperature_unconstrained_c: value.minimum_temperature_unconstrained_c,
        vapor_pressure_deficit_unconstrained_pa: value.vapor_pressure_deficit_unconstrained_pa,
        vapor_pressure_deficit_inactive_pa: value.vapor_pressure_deficit_inactive_pa,
        photoperiod_inactive_hours: value.photoperiod_inactive_hours,
        photoperiod_unconstrained_hours: value.photoperiod_unconstrained_hours,
    }
}

const fn gsi_parameters(value: DirectGsiParametersV1) -> GsiParameters {
    GsiParameters {
        minimum_temperature_inactive_c: value.minimum_temperature_inactive_c,
        minimum_temperature_unconstrained_c: value.minimum_temperature_unconstrained_c,
        vapor_pressure_deficit_unconstrained_pa: value.vapor_pressure_deficit_unconstrained_pa,
        vapor_pressure_deficit_inactive_pa: value.vapor_pressure_deficit_inactive_pa,
        photoperiod_inactive_hours: value.photoperiod_inactive_hours,
        photoperiod_unconstrained_hours: value.photoperiod_unconstrained_hours,
    }
}

const fn direct_gsi_forcing(value: GsiDailyForcing) -> DirectGsiForcingV1 {
    DirectGsiForcingV1 {
        minimum_temperature_c: value.minimum_temperature_c,
        vapor_pressure_deficit_pa: value.vapor_pressure_deficit_pa,
        latitude_degrees: value.latitude_degrees,
        year: value.date.year,
        ordinal_day: value.date.ordinal_day,
    }
}

const fn gsi_forcing(value: DirectGsiForcingV1) -> GsiDailyForcing {
    GsiDailyForcing {
        minimum_temperature_c: value.minimum_temperature_c,
        vapor_pressure_deficit_pa: value.vapor_pressure_deficit_pa,
        latitude_degrees: value.latitude_degrees,
        date: GsiDate {
            year: value.year,
            ordinal_day: value.ordinal_day,
        },
    }
}

const fn direct_gsi_result(value: GsiDailyResult) -> DirectGsiResultV1 {
    let GsiDailyIndicators {
        minimum_temperature,
        vapor_pressure_deficit,
        photoperiod,
        instantaneous_gsi,
        photoperiod_hours,
    } = value.indicators;
    DirectGsiResultV1 {
        minimum_temperature_indicator: minimum_temperature,
        vapor_pressure_deficit_indicator: vapor_pressure_deficit,
        photoperiod_indicator: photoperiod,
        instantaneous_gsi,
        photoperiod_hours,
        growing_season_index: value.growing_season_index,
        sample_count: value.sample_count,
    }
}

fn canonical_sha256_without_receipt(
    value: &DirectGsiDailyReceiptV1,
) -> Result<String, SnowFreeHalfHourForcingError> {
    let mut canonical = value.clone();
    canonical.receipt_sha256.clear();
    canonical_sha256(&canonical)
}

/// Sequential provider cursor retaining exact next-day precipitation parcels.
#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct SnowFreeHalfHourProviderCursor {
    next_day_index: usize,
    configuration_sha256: Option<String>,
    pending_carry: Vec<SnowFreePrecipitationParcelReceipt>,
}

#[derive(Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
struct SnowFreeHalfHourProviderCursorSnapshot {
    next_day_index: usize,
    configuration_sha256: Option<String>,
    pending_carry: Vec<SnowFreePrecipitationParcelReceipt>,
}

impl SnowFreeHalfHourProviderCursor {
    /// Serialize the complete provider cursor for a persisted restart owner.
    pub fn to_json_bytes(&self) -> Result<Vec<u8>, SnowFreeHalfHourForcingError> {
        serde_json::to_vec(self)
            .map_err(|error| SnowFreeHalfHourForcingError::Serialization(error.to_string()))
    }

    /// Restore and validate a cursor against the expected static provider
    /// configuration and scheduler day.
    pub fn restore_json(
        bytes: &[u8],
        configuration: &SnowFreeHalfHourProviderConfiguration,
        expected_next_day_index: usize,
    ) -> Result<Self, SnowFreeHalfHourForcingError> {
        validate_snow_free_configuration(configuration)?;
        let snapshot: SnowFreeHalfHourProviderCursorSnapshot = serde_json::from_slice(bytes)
            .map_err(|error| SnowFreeHalfHourForcingError::Serialization(error.to_string()))?;
        let value = Self {
            next_day_index: snapshot.next_day_index,
            configuration_sha256: snapshot.configuration_sha256,
            pending_carry: snapshot.pending_carry,
        };
        let expected_configuration_sha256 = snow_free_configuration_sha256(configuration);
        if value.next_day_index != expected_next_day_index
            || value.configuration_sha256.as_deref()
                != Some(expected_configuration_sha256.as_str())
        {
            return Err(SnowFreeHalfHourForcingError::Identity(
                "restored provider cursor",
            ));
        }
        let destinations = configuration
            .destinations
            .iter()
            .map(|destination| (&destination.ofe_id, &destination.tile_id))
            .collect::<std::collections::BTreeSet<_>>();
        let mut parcel_ids = std::collections::BTreeSet::new();
        for parcel in &value.pending_carry {
            validate_precipitation_parcel(parcel)?;
            if !parcel_ids.insert((
                &parcel.destination_ofe_id,
                &parcel.destination_tile_id,
                &parcel.parcel_id,
            ))
                || !destinations.contains(&(
                    &parcel.destination_ofe_id,
                    &parcel.destination_tile_id,
                ))
                || parcel.start_s < 0.0
                || parcel.end_s > 86_400.0
            {
                return Err(SnowFreeHalfHourForcingError::Identity(
                    "restored provider carry",
                ));
            }
        }
        Ok(value)
    }
}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct SnowFreePrecipitationParcelReceipt {
    pub parcel_id: String,
    pub source_owner_id: String,
    pub destination_ofe_id: String,
    pub destination_tile_id: String,
    pub start_s: f64,
    pub end_s: f64,
    pub mass_kg_m2: f64,
    pub temperature_k: f64,
    pub enthalpy_j_m2: f64,
}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct SnowFreeHalfHourIntervalReceipt {
    pub provider_definition_sha256: String,
    pub source_climate_sha256: String,
    pub run_id: String,
    pub day_index: usize,
    pub ofe_id: String,
    pub tile_id: String,
    pub interval_index: usize,
    pub transaction_id: String,
    pub start_s: usize,
    pub end_s: usize,
    pub parent_hour_index: usize,
    pub air_temperature_c: f64,
    pub dew_point_c: f64,
    pub wind_m_s: f64,
    pub pressure_kpa: f64,
    pub actual_vapor_pressure_kpa: f64,
    pub specific_humidity_kg_kg: f64,
    pub vpd_kpa: f64,
    pub cloud_fraction: f64,
    pub solar_zenith_cosine: f64,
    pub global_horizontal_shortwave_w_m2: f64,
    pub direct_visible_w_m2: f64,
    pub diffuse_visible_w_m2: f64,
    pub direct_nir_w_m2: f64,
    pub diffuse_nir_w_m2: f64,
    pub downward_longwave_w_m2: f64,
    pub co2_pa: f64,
    pub reference_height_m: f64,
    pub gsi: f64,
    pub gsi_receipt_sha256: String,
    pub wb14_configuration_sha256: String,
    pub precipitation_parcels: Vec<SnowFreePrecipitationParcelReceipt>,
    pub interval_receipt_sha256: String,
}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct SnowFreeHalfHourDayReceipt {
    pub provider_version: String,
    pub provider_definition_sha256: String,
    pub source_climate_sha256: String,
    pub run_id: String,
    pub day_index: usize,
    pub daily_horizontal_energy_mj_m2: f64,
    pub intervals: Vec<SnowFreeHalfHourIntervalReceipt>,
    pub next_day_precipitation_carry: Vec<SnowFreePrecipitationParcelReceipt>,
    pub receipt_sha256: String,
}

/// Opaque proof that receipts were constructed and validated by the
/// repository provider. Closure-eligible consumers accept this wrapper, not
/// caller-constructed receipt DTOs.
#[derive(Clone, Debug, Serialize)]
pub struct ValidatedSnowFreeHalfHourForcingReceipts {
    receipts: Vec<SnowFreeHalfHourDayReceipt>,
    beginning_cursor: SnowFreeHalfHourProviderCursor,
    ending_cursor: SnowFreeHalfHourProviderCursor,
}

impl ValidatedSnowFreeHalfHourForcingReceipts {
    #[must_use]
    pub fn receipts(&self) -> &[SnowFreeHalfHourDayReceipt] {
        &self.receipts
    }

    /// Commit the prepared cursor transition only after every downstream
    /// owner has accepted the projected day.
    pub fn commit_cursor(
        self,
        cursor: &mut SnowFreeHalfHourProviderCursor,
    ) -> Result<(), SnowFreeHalfHourForcingError> {
        if cursor != &self.beginning_cursor {
            return Err(SnowFreeHalfHourForcingError::Identity(
                "provider cursor commit beginning",
            ));
        }
        *cursor = self.ending_cursor;
        Ok(())
    }
}

impl std::ops::Deref for ValidatedSnowFreeHalfHourForcingReceipts {
    type Target = [SnowFreeHalfHourDayReceipt];

    fn deref(&self) -> &Self::Target {
        &self.receipts
    }
}

impl SnowFreeHalfHourDayReceipt {
    pub fn validate(&self) -> Result<(), SnowFreeHalfHourForcingError> {
        if self.provider_version != "OPENWEPP_SNOW_FREE_HALF_HOUR_FORCING_V1"
            || self.provider_definition_sha256 != SNOW_FREE_PROVIDER_SHA256
            || !is_sha256(&self.source_climate_sha256)
            || self.intervals.len() != SNOW_FREE_INTERVAL_COUNT
        {
            return Err(SnowFreeHalfHourForcingError::Identity("day receipt"));
        }
        let expected_ofe = &self.intervals[0].ofe_id;
        let expected_tile = &self.intervals[0].tile_id;
        let mut parcel_ids = std::collections::BTreeSet::new();
        for (expected, interval) in self.intervals.iter().enumerate() {
            if interval.provider_definition_sha256 != self.provider_definition_sha256
                || interval.source_climate_sha256 != self.source_climate_sha256
                || interval.run_id != self.run_id
                || interval.day_index != self.day_index
                || interval.interval_index != expected
                || interval.start_s != expected * 1_800
                || interval.end_s != (expected + 1) * 1_800
                || interval.parent_hour_index != expected / 2
                || interval.ofe_id != *expected_ofe
                || interval.tile_id != *expected_tile
                || interval.transaction_id
                    != format!("{}:{}:{expected}", self.run_id, self.day_index)
                || interval.interval_receipt_sha256 != canonical_sha256(interval)?
            {
                return Err(SnowFreeHalfHourForcingError::Identity(
                    "interval receipt",
                ));
            }
            validate_interval_physics(interval)?;
            let interval_start_s = f64::from(
                u32::try_from(interval.start_s)
                    .map_err(|_| SnowFreeHalfHourForcingError::Identity("interval support"))?,
            );
            let interval_end_s = f64::from(
                u32::try_from(interval.end_s)
                    .map_err(|_| SnowFreeHalfHourForcingError::Identity("interval support"))?,
            );
            for parcel in &interval.precipitation_parcels {
                validate_precipitation_parcel(parcel)?;
                if !parcel_ids.insert(&parcel.parcel_id)
                    || parcel.destination_ofe_id != interval.ofe_id
                    || parcel.destination_tile_id != interval.tile_id
                    || parcel.start_s < interval_start_s
                    || parcel.end_s > interval_end_s
                {
                    return Err(SnowFreeHalfHourForcingError::Identity(
                        "interval parcel binding",
                    ));
                }
            }
        }
        for parcel in &self.next_day_precipitation_carry {
            validate_precipitation_parcel(parcel)?;
            if !parcel_ids.insert(&parcel.parcel_id)
                || parcel.source_owner_id != self.source_climate_sha256
                || parcel.destination_ofe_id != *expected_ofe
                || parcel.destination_tile_id != *expected_tile
                || parcel.end_s > 86_400.0
            {
                return Err(SnowFreeHalfHourForcingError::Identity(
                    "carry parcel binding",
                ));
            }
        }
        for pair in self.intervals.chunks_exact(2) {
            validate_parent_hold(&pair[0], &pair[1])?;
        }
        let reconstructed_energy = self
            .intervals
            .chunks_exact(2)
            .map(|pair| pair[0].global_horizontal_shortwave_w_m2 * 3_600.0 / 1_000_000.0)
            .sum::<f64>();
        if (reconstructed_energy - self.daily_horizontal_energy_mj_m2).abs()
            > 1.0e-12 * self.daily_horizontal_energy_mj_m2.abs().max(1.0)
        {
            return Err(SnowFreeHalfHourForcingError::Closure(
                "daily horizontal radiation",
            ));
        }
        if self.receipt_sha256 != canonical_sha256(self)? {
            return Err(SnowFreeHalfHourForcingError::Identity(
                "day receipt digest",
            ));
        }
        Ok(())
    }
}

fn validate_parent_hold(
    first: &SnowFreeHalfHourIntervalReceipt,
    second: &SnowFreeHalfHourIntervalReceipt,
) -> Result<(), SnowFreeHalfHourForcingError> {
    let first_values = [
        first.air_temperature_c,
        first.dew_point_c,
        first.wind_m_s,
        first.pressure_kpa,
        first.actual_vapor_pressure_kpa,
        first.specific_humidity_kg_kg,
        first.vpd_kpa,
        first.cloud_fraction,
        first.solar_zenith_cosine,
        first.global_horizontal_shortwave_w_m2,
        first.direct_visible_w_m2,
        first.diffuse_visible_w_m2,
        first.direct_nir_w_m2,
        first.diffuse_nir_w_m2,
        first.downward_longwave_w_m2,
        first.co2_pa,
        first.reference_height_m,
        first.gsi,
    ];
    let second_values = [
        second.air_temperature_c,
        second.dew_point_c,
        second.wind_m_s,
        second.pressure_kpa,
        second.actual_vapor_pressure_kpa,
        second.specific_humidity_kg_kg,
        second.vpd_kpa,
        second.cloud_fraction,
        second.solar_zenith_cosine,
        second.global_horizontal_shortwave_w_m2,
        second.direct_visible_w_m2,
        second.diffuse_visible_w_m2,
        second.direct_nir_w_m2,
        second.diffuse_nir_w_m2,
        second.downward_longwave_w_m2,
        second.co2_pa,
        second.reference_height_m,
        second.gsi,
    ];
    if first_values
        .iter()
        .zip(second_values)
        .any(|(left, right)| left.to_bits() != right.to_bits())
        || first.gsi_receipt_sha256 != second.gsi_receipt_sha256
        || first.wb14_configuration_sha256 != second.wb14_configuration_sha256
    {
        return Err(SnowFreeHalfHourForcingError::Identity(
            "parent zero-order hold",
        ));
    }
    Ok(())
}

fn validate_interval_physics(
    interval: &SnowFreeHalfHourIntervalReceipt,
) -> Result<(), SnowFreeHalfHourForcingError> {
    let operands = [
        interval.air_temperature_c,
        interval.dew_point_c,
        interval.wind_m_s,
        interval.pressure_kpa,
        interval.actual_vapor_pressure_kpa,
        interval.specific_humidity_kg_kg,
        interval.vpd_kpa,
        interval.cloud_fraction,
        interval.solar_zenith_cosine,
        interval.global_horizontal_shortwave_w_m2,
        interval.direct_visible_w_m2,
        interval.diffuse_visible_w_m2,
        interval.direct_nir_w_m2,
        interval.diffuse_nir_w_m2,
        interval.downward_longwave_w_m2,
        interval.co2_pa,
        interval.reference_height_m,
        interval.gsi,
    ];
    let reconstructed_shortwave = interval.direct_visible_w_m2
        + interval.diffuse_visible_w_m2
        + interval.direct_nir_w_m2
        + interval.diffuse_nir_w_m2;
    if operands.iter().any(|value| !value.is_finite())
        || interval.wind_m_s <= 0.0
        || interval.pressure_kpa <= interval.actual_vapor_pressure_kpa
        || interval.specific_humidity_kg_kg < 0.0
        || !(0.0..=1.0).contains(&interval.cloud_fraction)
        || interval.solar_zenith_cosine < 0.0
        || interval.global_horizontal_shortwave_w_m2 < 0.0
        || interval.direct_visible_w_m2 < 0.0
        || interval.diffuse_visible_w_m2 < 0.0
        || interval.direct_nir_w_m2 < 0.0
        || interval.diffuse_nir_w_m2 < 0.0
        || interval.downward_longwave_w_m2 < 0.0
        || interval.co2_pa <= 0.0
        || interval.reference_height_m <= 0.0
        || !(0.0..=1.0).contains(&interval.gsi)
        || !is_sha256(&interval.gsi_receipt_sha256)
        || !is_sha256(&interval.wb14_configuration_sha256)
        || (reconstructed_shortwave - interval.global_horizontal_shortwave_w_m2).abs()
            > 1.0e-12
                * interval
                    .global_horizontal_shortwave_w_m2
                    .abs()
                    .max(1.0)
    {
        return Err(SnowFreeHalfHourForcingError::Closure(
            "interval physical operands",
        ));
    }
    Ok(())
}

#[derive(Clone, Debug, Error, PartialEq)]
pub enum SnowFreeHalfHourForcingError {
    #[error(transparent)]
    Climate(#[from] ClimateRuntimeInputError),
    #[error(transparent)]
    Atmospheric(#[from] SnowFreeAtmosphericError),
    #[error(transparent)]
    Gsi(#[from] GsiError),
    #[error("forcing provider identity failure: {0}")]
    Identity(&'static str),
    #[error("forcing provider unsupported domain: {0}")]
    Unsupported(&'static str),
    #[error("forcing provider closure failure: {0}")]
    Closure(&'static str),
    #[error("forcing provider serialization failure: {0}")]
    Serialization(String),
}

#[derive(Clone, Copy, Debug)]
struct SnowFreeHourlyParent {
    air_temperature_c: f64,
    dew_point_c: f64,
    wind_m_s: f64,
    pressure_kpa: f64,
    actual_vapor_pressure_kpa: f64,
    specific_humidity_kg_kg: f64,
    vpd_kpa: f64,
    cloud_fraction: f64,
    solar_zenith_cosine: f64,
    global_horizontal_shortwave_w_m2: f64,
    direct_visible_w_m2: f64,
    diffuse_visible_w_m2: f64,
    direct_nir_w_m2: f64,
    diffuse_nir_w_m2: f64,
    downward_longwave_w_m2: f64,
    hydrometeor_temperature_c: f64,
    snow_fraction: f64,
}

#[derive(Clone, Copy)]
struct HourlyParentContext {
    day_of_year: i32,
    tmax_c: f64,
    tmin_c: f64,
    dew_point_c: f64,
    wind_m_s: f64,
    pressure_kpa: f64,
    radmj: f64,
    daily_horizontal_potential: f64,
    geometry: Simimpl28AspectGeometry,
    sunmap: Simimpl28SunmapResult,
}

type PrecipitationSupport = (f64, f64, f64);
type ChildMassesAndCarry = ([f64; 48], Vec<PrecipitationSupport>);

impl HillslopeClimateRuntimeRequest {
    /// Closure-eligible projection joining static provider configuration to an
    /// accepted CP-GSI01 daily receipt. Daily GSI is never part of cursor
    /// identity and cannot be supplied independently of its receipt.
    pub fn snow_free_half_hour_forcing_receipts_with_gsi(
        &self,
        day_index: usize,
        configuration: &SnowFreeHalfHourStaticConfiguration,
        gsi_receipt: &DirectGsiDailyReceiptV1,
        cursor: &SnowFreeHalfHourProviderCursor,
    ) -> Result<ValidatedSnowFreeHalfHourForcingReceipts, SnowFreeHalfHourForcingError> {
        gsi_receipt.validate()?;
        if configuration.gsi_owner_configuration_sha256 != gsi_receipt.configuration_sha256 {
            return Err(SnowFreeHalfHourForcingError::Identity(
                "provider GSI owner configuration",
            ));
        }
        let daily = SnowFreeHalfHourProviderConfiguration {
            run_id: configuration.run_id.clone(),
            co2_pa: configuration.co2_pa,
            reference_height_m: configuration.reference_height_m,
            gsi: gsi_receipt.result.growing_season_index,
            gsi_receipt_sha256: gsi_receipt.receipt_sha256.clone(),
            destinations: configuration.destinations.clone(),
        };
        self.snow_free_half_hour_forcing_receipts_impl(
            day_index,
            &daily,
            snow_free_static_configuration_sha256(configuration),
            cursor,
        )
    }

    /// Project actual repository climate inputs into 48 digest-bound receipts
    /// for every configured destination. This API remains explicit/default-off.
    pub fn snow_free_half_hour_forcing_receipts(
        &self,
        day_index: usize,
        configuration: &SnowFreeHalfHourProviderConfiguration,
        cursor: &SnowFreeHalfHourProviderCursor,
    ) -> Result<ValidatedSnowFreeHalfHourForcingReceipts, SnowFreeHalfHourForcingError> {
        self.snow_free_half_hour_forcing_receipts_impl(
            day_index,
            configuration,
            snow_free_configuration_sha256(configuration),
            cursor,
        )
    }

    fn snow_free_half_hour_forcing_receipts_impl(
        &self,
        day_index: usize,
        configuration: &SnowFreeHalfHourProviderConfiguration,
        configuration_sha256: String,
        cursor: &SnowFreeHalfHourProviderCursor,
    ) -> Result<ValidatedSnowFreeHalfHourForcingReceipts, SnowFreeHalfHourForcingError> {
        if day_index != cursor.next_day_index {
            return Err(SnowFreeHalfHourForcingError::Identity(
                "provider cursor day",
            ));
        }
        validate_snow_free_configuration(configuration)?;
        if cursor
            .configuration_sha256
            .as_ref()
            .is_some_and(|value| value != &configuration_sha256)
        {
            return Err(SnowFreeHalfHourForcingError::Identity(
                "provider cursor configuration",
            ));
        }
        let forcing = select_day_forcing(&self.shared, day_index)?;
        let source_climate_sha256 = source_climate_sha256(self, day_index, forcing);
        let parents = build_snow_free_hourly_parents(forcing, &self.metadata)?;
        let (child_masses, carry_supports) = precipitation_child_masses(forcing, &parents)?;
        let daily_energy = parents
            .iter()
            .map(|parent| parent.global_horizontal_shortwave_w_m2 * 3_600.0 / 1_000_000.0)
            .sum::<f64>();
        let mut receipts = Vec::with_capacity(configuration.destinations.len());
        for destination in &configuration.destinations {
            let receipt = build_destination_receipt(
                day_index,
                configuration,
                destination,
                &source_climate_sha256,
                &parents,
                &child_masses,
                &carry_supports,
                daily_energy,
            )?;
            receipt.validate()?;
            receipts.push(receipt);
        }
        apply_pending_carry(&mut receipts, &cursor.pending_carry)?;
        let pending_carry = receipts
            .iter()
            .flat_map(|receipt| receipt.next_day_precipitation_carry.iter().cloned())
            .collect();
        let mut ending_cursor = cursor.clone();
        ending_cursor.pending_carry = pending_carry;
        ending_cursor.configuration_sha256 = Some(configuration_sha256);
        ending_cursor.next_day_index += 1;
        Ok(ValidatedSnowFreeHalfHourForcingReceipts {
            receipts,
            beginning_cursor: cursor.clone(),
            ending_cursor,
        })
    }
}

fn snow_free_configuration_sha256(value: &SnowFreeHalfHourProviderConfiguration) -> String {
    let mut digest = Sha256::new();
    update_string_digest(&mut digest, &value.run_id);
    digest.update(value.reference_height_m.to_bits().to_le_bytes());
    digest.update((value.destinations.len() as u64).to_le_bytes());
    for destination in &value.destinations {
        update_string_digest(&mut digest, &destination.ofe_id);
        update_string_digest(&mut digest, &destination.tile_id);
        update_string_digest(&mut digest, &destination.wb14_configuration_sha256);
    }
    format!("{:x}", digest.finalize())
}

fn snow_free_static_configuration_sha256(value: &SnowFreeHalfHourStaticConfiguration) -> String {
    let mut digest = Sha256::new();
    update_string_digest(&mut digest, &value.run_id);
    digest.update(value.co2_pa.to_bits().to_le_bytes());
    digest.update(value.reference_height_m.to_bits().to_le_bytes());
    update_string_digest(&mut digest, &value.gsi_owner_configuration_sha256);
    digest.update((value.destinations.len() as u64).to_le_bytes());
    for destination in &value.destinations {
        update_string_digest(&mut digest, &destination.ofe_id);
        update_string_digest(&mut digest, &destination.tile_id);
        update_string_digest(&mut digest, &destination.wb14_configuration_sha256);
    }
    format!("{:x}", digest.finalize())
}

fn apply_pending_carry(
    receipts: &mut [SnowFreeHalfHourDayReceipt],
    pending: &[SnowFreePrecipitationParcelReceipt],
) -> Result<(), SnowFreeHalfHourForcingError> {
    for parcel in pending {
        let receipt = receipts
            .iter_mut()
            .find(|receipt| {
                receipt.intervals[0].ofe_id == parcel.destination_ofe_id
                    && receipt.intervals[0].tile_id == parcel.destination_tile_id
            })
            .ok_or(SnowFreeHalfHourForcingError::Identity(
                "carry destination",
            ))?;
        let mut interval_index = None;
        for (index, interval) in receipt.intervals.iter().enumerate() {
            let start = f64::from(u32::try_from(interval.start_s).map_err(|_| {
                SnowFreeHalfHourForcingError::Identity("interval support")
            })?);
            let end = f64::from(u32::try_from(interval.end_s).map_err(|_| {
                SnowFreeHalfHourForcingError::Identity("interval support")
            })?);
            if parcel.start_s >= start && parcel.end_s <= end {
                interval_index = Some(index);
                break;
            }
        }
        let interval = receipt.intervals.get_mut(interval_index.ok_or(
            SnowFreeHalfHourForcingError::Identity("carry support"),
        )?)
        .ok_or(SnowFreeHalfHourForcingError::Identity("carry support"))?;
        interval.precipitation_parcels.push(parcel.clone());
        interval.interval_receipt_sha256 = canonical_sha256(interval)?;
    }
    for receipt in receipts {
        receipt.receipt_sha256 = canonical_sha256(receipt)?;
        receipt.validate()?;
    }
    Ok(())
}

fn validate_snow_free_configuration(
    value: &SnowFreeHalfHourProviderConfiguration,
) -> Result<(), SnowFreeHalfHourForcingError> {
    if value.run_id.is_empty()
        || !is_sha256(&value.gsi_receipt_sha256)
        || value.destinations.is_empty()
        || !value.co2_pa.is_finite()
        || value.co2_pa <= 0.0
        || !value.reference_height_m.is_finite()
        || value.reference_height_m <= 0.0
        || !value.gsi.is_finite()
        || !(0.0..=1.0).contains(&value.gsi)
    {
        return Err(SnowFreeHalfHourForcingError::Identity(
            "provider configuration",
        ));
    }
    let mut identities = std::collections::BTreeSet::new();
    for destination in &value.destinations {
        if destination.ofe_id.is_empty()
            || destination.tile_id.is_empty()
            || !is_sha256(&destination.wb14_configuration_sha256)
            || !identities.insert((&destination.ofe_id, &destination.tile_id))
        {
            return Err(SnowFreeHalfHourForcingError::Identity(
                "destination configuration",
            ));
        }
    }
    Ok(())
}

fn is_sha256(value: &str) -> bool {
    value.len() == 64
        && value
            .bytes()
            .all(|byte| byte.is_ascii_digit() || (b'a'..=b'f').contains(&byte))
}

fn source_climate_sha256(
    request: &HillslopeClimateRuntimeRequest,
    day_index: usize,
    forcing: &HillslopeClimateDailyForcing,
) -> String {
    let mut digest = Sha256::new();
    update_string_digest(&mut digest, &request.shared.station_id);
    digest.update(request.shared.datver.to_bits().to_le_bytes());
    digest.update(request.metadata.deglat.to_bits().to_le_bytes());
    digest.update(request.metadata.elev.to_bits().to_le_bytes());
    digest.update((day_index as u64).to_le_bytes());
    let (day, month, year, precipitation, tmax, tmin, radiation, wind, dew, storm_start, times, intensities) =
        match forcing {
            HillslopeClimateDailyForcing::NoBreakpoint(value) => (
                value.day, value.mon, value.year, value.prcp, value.tmax, value.tmin, value.rad,
                value.vwind, value.tdpt, None, &value.timem, &value.intsty,
            ),
            HillslopeClimateDailyForcing::Breakpoint(value) => (
                value.day, value.mon, value.year, value.prcp, value.tmax, value.tmin, value.rad,
                value.vwind, value.tdpt, Some(value.stmstr), &value.timem, &value.intsty,
            ),
        };
    digest.update(day.to_le_bytes());
    digest.update(month.to_le_bytes());
    digest.update(year.to_le_bytes());
    for value in [precipitation, tmax, tmin, radiation, wind, dew] {
        digest.update(value.to_bits().to_le_bytes());
    }
    digest.update(storm_start.unwrap_or(-1.0).to_bits().to_le_bytes());
    digest.update((times.len() as u64).to_le_bytes());
    for value in times {
        digest.update(value.to_bits().to_le_bytes());
    }
    digest.update((intensities.len() as u64).to_le_bytes());
    for value in intensities {
        digest.update(value.to_bits().to_le_bytes());
    }
    match forcing {
        HillslopeClimateDailyForcing::NoBreakpoint(value) => {
            digest.update(b"no-breakpoint");
            for operand in [
                value.stmdur,
                value.timep,
                value.ip,
                value.avrint,
                value.mxint,
                value.wind,
            ] {
                digest.update(operand.to_bits().to_le_bytes());
            }
            digest.update((value.ninten as u64).to_le_bytes());
        }
        HillslopeClimateDailyForcing::Breakpoint(value) => {
            digest.update(b"breakpoint");
            for operand in [value.stmdur, value.mxint, value.wind] {
                digest.update(operand.to_bits().to_le_bytes());
            }
            digest.update((value.nbrkpt as u64).to_le_bytes());
        }
    }
    format!("{:x}", digest.finalize())
}

fn update_string_digest(digest: &mut Sha256, value: &str) {
    digest.update((value.len() as u64).to_le_bytes());
    digest.update(value.as_bytes());
}

fn build_snow_free_hourly_parents(
    forcing: &HillslopeClimateDailyForcing,
    metadata: &ClimateMetadata,
) -> Result<[SnowFreeHourlyParent; 24], SnowFreeHalfHourForcingError> {
    let (day, month, year, tmax, tmin, radly, dew_point_c, wind_m_s) = match forcing {
        HillslopeClimateDailyForcing::NoBreakpoint(value) => (
            value.day, value.mon, value.year, value.tmax, value.tmin, value.rad, value.tdpt,
            value.vwind,
        ),
        HillslopeClimateDailyForcing::Breakpoint(value) => (
            value.day, value.mon, value.year, value.tmax, value.tmin, value.rad, value.tdpt,
            value.vwind,
        ),
    };
    if wind_m_s <= 0.0 {
        return Err(SnowFreeHalfHourForcingError::Unsupported(
            "nonpositive wind",
        ));
    }
    let day_of_year = simimpl28_day_of_year(day, month, year)?;
    let geometry = simimpl28_aspect_geometry(metadata.deglat, 0.0, 0.0)?;
    let radmj = simimpl28_langleys_to_mj_m2("rad", radly)?;
    let sunmap = simimpl28_sunmap(radly, day_of_year, geometry)?;
    let mut hourly_potential = [0.0; 24];
    for hour in 1..=24 {
        hourly_potential[hour - 1] =
            simimpl28_radcur(day_of_year, hour, geometry.radlat, sunmap.dsunmp)?;
    }
    let daily_horizontal_potential = hourly_potential.iter().sum::<f64>();
    let pressure_kpa = fao56_station_pressure_kpa(metadata.elev)?;
    let mut parents = [SnowFreeHourlyParent {
        air_temperature_c: 0.0,
        dew_point_c: 0.0,
        wind_m_s: 0.0,
        pressure_kpa: 0.0,
        actual_vapor_pressure_kpa: 0.0,
        specific_humidity_kg_kg: 0.0,
        vpd_kpa: 0.0,
        cloud_fraction: 0.0,
        solar_zenith_cosine: 0.0,
        global_horizontal_shortwave_w_m2: 0.0,
        direct_visible_w_m2: 0.0,
        diffuse_visible_w_m2: 0.0,
        direct_nir_w_m2: 0.0,
        diffuse_nir_w_m2: 0.0,
        downward_longwave_w_m2: 0.0,
        hydrometeor_temperature_c: 0.0,
        snow_fraction: 0.0,
    }; 24];
    let context = HourlyParentContext {
        day_of_year,
        tmax_c: tmax,
        tmin_c: tmin,
        dew_point_c,
        wind_m_s,
        pressure_kpa,
        radmj,
        daily_horizontal_potential,
        geometry,
        sunmap,
    };
    for hour in 1..=24 {
        parents[hour - 1] = build_hourly_parent(hour, hourly_potential[hour - 1], context)?;
    }
    Ok(parents)
}

fn build_hourly_parent(
    hour: usize,
    hourly_potential: f64,
    context: HourlyParentContext,
) -> Result<SnowFreeHourlyParent, SnowFreeHalfHourForcingError> {
    let horizontal_energy = if context.daily_horizontal_potential == 0.0 {
        0.0
    } else {
        context.radmj * hourly_potential / context.daily_horizontal_potential
    };
    let air_c = if context.tmax_c - context.tmin_c <= 1.0 {
        f64::midpoint(context.tmax_c, context.tmin_c)
    } else {
        simimpl28_hrtmp(hour, context.sunmap.halfdy, context.tmax_c, context.tmin_c)
    };
    let air = TemperatureCelsius::try_new(air_c)
        .map_err(|_| SnowFreeHalfHourForcingError::Unsupported("air temperature"))?;
    let dew = TemperatureCelsius::try_new(context.dew_point_c)
        .map_err(|_| SnowFreeHalfHourForcingError::Unsupported("dew-point temperature"))?;
    let actual_vapor_pressure_kpa = saturation_vapor_pressure_water_kpa(dew)
        .map_err(|_| SnowFreeHalfHourForcingError::Unsupported("actual vapor pressure"))?
        .as_kilopascals();
    if context.pressure_kpa <= actual_vapor_pressure_kpa {
        return Err(SnowFreeHalfHourForcingError::Unsupported(
            "pressure not above vapor pressure",
        ));
    }
    let specific_humidity_kg_kg = 0.622 * actual_vapor_pressure_kpa
        / (context.pressure_kpa - 0.378 * actual_vapor_pressure_kpa);
    let saturation = saturation_vapor_pressure_water_kpa(air)
        .map_err(|_| SnowFreeHalfHourForcingError::Unsupported("air saturation"))?
        .as_kilopascals();
    let vpd_kpa = saturation - actual_vapor_pressure_kpa;
    let phase_relative_humidity = FractionUnitInterval::try_new(
        (actual_vapor_pressure_kpa / saturation).min(1.0),
    )
    .map_err(|_| SnowFreeHalfHourForcingError::Unsupported("phase relative humidity"))?;
    let phase = harder_pomeroy_phase_from_relative_humidity(
        air,
        phase_relative_humidity,
        PhaseTimescale::Hourly,
    )
    .map_err(|_| SnowFreeHalfHourForcingError::Unsupported("hydrometeor phase"))?;
    let global_w_m2 = horizontal_energy * 1_000_000.0 / 3_600.0;
    let mean_mu = simimpl28_parent_mean_mu(
        context.day_of_year,
        hour,
        context.geometry.radlat,
        context.sunmap.dsunmp,
    )?;
    let shortwave = weiss_norman_partition(global_w_m2, mean_mu, context.pressure_kpa)?;
    Ok(SnowFreeHourlyParent {
        air_temperature_c: air_c,
        dew_point_c: context.dew_point_c,
        wind_m_s: context.wind_m_s,
        pressure_kpa: context.pressure_kpa,
        actual_vapor_pressure_kpa,
        specific_humidity_kg_kg,
        vpd_kpa,
        cloud_fraction: context.sunmap.cloud_fraction,
        solar_zenith_cosine: mean_mu,
        global_horizontal_shortwave_w_m2: global_w_m2,
        direct_visible_w_m2: shortwave.direct_visible_w_m2,
        diffuse_visible_w_m2: shortwave.diffuse_visible_w_m2,
        direct_nir_w_m2: shortwave.direct_nir_w_m2,
        diffuse_nir_w_m2: shortwave.diffuse_nir_w_m2,
        downward_longwave_w_m2: atmospheric_longwave_dilley_unsworth(
            air_c + 273.15,
            actual_vapor_pressure_kpa,
            context.sunmap.cloud_fraction,
        )?,
        hydrometeor_temperature_c: phase.hydrometeor_temperature.temperature.as_celsius(),
        snow_fraction: phase.fractions.snow_fraction.as_fraction(),
    })
}

fn precipitation_child_masses(
    forcing: &HillslopeClimateDailyForcing,
    parents: &[SnowFreeHourlyParent; 24],
) -> Result<ChildMassesAndCarry, SnowFreeHalfHourForcingError> {
    let mut masses = [0.0; 48];
    let mut carry = Vec::new();
    match forcing {
        HillslopeClimateDailyForcing::Breakpoint(value) => {
            let offset = value.stmstr * 3_600.0;
            for segment in 0..value.timem.len().saturating_sub(1) {
                let start = offset + value.timem[segment];
                let end = offset + value.timem[segment + 1];
                let intensity = value.intsty[segment];
                for (index, mass) in masses.iter_mut().enumerate() {
                    let index_u32 = u32::try_from(index).map_err(|_| {
                        SnowFreeHalfHourForcingError::Identity("child interval index")
                    })?;
                    let child_start = f64::from(index_u32) * SNOW_FREE_INTERVAL_S;
                    let child_end = child_start + SNOW_FREE_INTERVAL_S;
                    let overlap = (child_end.min(end) - child_start.max(start)).max(0.0);
                    *mass += 1_000.0 * overlap * intensity;
                }
                if end > 86_400.0 {
                    let translated_start = (start - 86_400.0).max(0.0);
                    let translated_end = end - 86_400.0;
                    for child_index in 0_u32..48 {
                        let child_start = f64::from(child_index) * SNOW_FREE_INTERVAL_S;
                        let child_end = child_start + SNOW_FREE_INTERVAL_S;
                        let overlap_start = child_start.max(translated_start);
                        let overlap_end = child_end.min(translated_end);
                        if overlap_end > overlap_start && intensity > 0.0 {
                            carry.push((overlap_start, overlap_end, intensity));
                        }
                    }
                }
            }
            let admitted_mass = masses.iter().sum::<f64>()
                + carry
                    .iter()
                    .map(|(start, end, intensity)| 1_000.0 * (end - start) * intensity)
                    .sum::<f64>();
            if (admitted_mass - value.prcp * 1_000.0).abs()
                > 1.0e-12 * (value.prcp * 1_000.0).abs().max(1.0)
            {
                return Err(SnowFreeHalfHourForcingError::Closure(
                    "breakpoint daily precipitation",
                ));
            }
        }
        HillslopeClimateDailyForcing::NoBreakpoint(value) => {
            let day_of_year = simimpl28_day_of_year(value.day, value.mon, value.year)?;
            let start_hour = simimpl28_winter_random_start_hour(day_of_year);
            for hour in 1_u32..=24 {
                let partition = simimpl28_stmtim_hourly_partition_with_model(
                    value.prcp,
                    value.stmdur,
                    f64::from(hour),
                    start_hour,
                    0.0,
                    parents[usize::try_from(hour - 1).map_err(|_| {
                        SnowFreeHalfHourForcingError::Identity("parent hour index")
                    })?]
                    .air_temperature_c,
                    value.tdpt,
                    SnowPhasePartitionModel::HarderPomeroyHourly,
                )?;
                let hour_index = usize::try_from(hour - 1).map_err(|_| {
                    SnowFreeHalfHourForcingError::Identity("parent hour index")
                })?;
                masses[2 * hour_index] = 500.0 * partition.hrrain_m;
                masses[2 * hour_index + 1] = 500.0 * partition.hrrain_m;
                if partition.hrsnow_m > 0.0 {
                    return Err(SnowFreeHalfHourForcingError::Unsupported(
                        "snow or mixed precipitation",
                    ));
                }
            }
            if (masses.iter().sum::<f64>() - value.prcp * 1_000.0).abs()
                > 1.0e-12 * (value.prcp * 1_000.0).abs().max(1.0)
            {
                return Err(SnowFreeHalfHourForcingError::Closure(
                    "parent-hour daily precipitation",
                ));
            }
        }
    }
    for (index, mass) in masses.iter().enumerate() {
        if *mass > 0.0 && parents[index / 2].snow_fraction > 0.0 {
            return Err(SnowFreeHalfHourForcingError::Unsupported(
                "snow or mixed precipitation",
            ));
        }
    }
    if !carry.is_empty() && parents[23].snow_fraction > 0.0 {
        return Err(SnowFreeHalfHourForcingError::Unsupported(
            "snow or mixed precipitation carry",
        ));
    }
    Ok((masses, carry))
}

#[allow(clippy::too_many_arguments)]
fn build_destination_receipt(
    day_index: usize,
    configuration: &SnowFreeHalfHourProviderConfiguration,
    destination: &SnowFreeHalfHourDestination,
    source_climate_sha256: &str,
    parents: &[SnowFreeHourlyParent; 24],
    child_masses: &[f64; 48],
    carry_supports: &[(f64, f64, f64)],
    daily_energy: f64,
) -> Result<SnowFreeHalfHourDayReceipt, SnowFreeHalfHourForcingError> {
    let mut intervals = Vec::with_capacity(SNOW_FREE_INTERVAL_COUNT);
    for interval_index in 0..SNOW_FREE_INTERVAL_COUNT {
        let parent = parents[interval_index / 2];
        let interval_u32 = u32::try_from(interval_index)
            .map_err(|_| SnowFreeHalfHourForcingError::Identity("child interval index"))?;
        let interval_start_s = f64::from(interval_u32) * SNOW_FREE_INTERVAL_S;
        let interval_end_s = interval_start_s + SNOW_FREE_INTERVAL_S;
        let precipitation_parcels = if child_masses[interval_index] == 0.0 {
            Vec::new()
        } else {
            vec![precipitation_parcel(
                day_index,
                interval_index,
                destination,
                source_climate_sha256,
                interval_start_s,
                interval_end_s,
                child_masses[interval_index],
                parent.hydrometeor_temperature_c,
            )]
        };
        let mut interval = SnowFreeHalfHourIntervalReceipt {
            provider_definition_sha256: SNOW_FREE_PROVIDER_SHA256.to_string(),
            source_climate_sha256: source_climate_sha256.to_string(),
            run_id: configuration.run_id.clone(),
            day_index,
            ofe_id: destination.ofe_id.clone(),
            tile_id: destination.tile_id.clone(),
            interval_index,
            transaction_id: format!("{}:{day_index}:{interval_index}", configuration.run_id),
            start_s: interval_index * 1_800,
            end_s: (interval_index + 1) * 1_800,
            parent_hour_index: interval_index / 2,
            air_temperature_c: parent.air_temperature_c,
            dew_point_c: parent.dew_point_c,
            wind_m_s: parent.wind_m_s,
            pressure_kpa: parent.pressure_kpa,
            actual_vapor_pressure_kpa: parent.actual_vapor_pressure_kpa,
            specific_humidity_kg_kg: parent.specific_humidity_kg_kg,
            vpd_kpa: parent.vpd_kpa,
            cloud_fraction: parent.cloud_fraction,
            solar_zenith_cosine: parent.solar_zenith_cosine,
            global_horizontal_shortwave_w_m2: parent.global_horizontal_shortwave_w_m2,
            direct_visible_w_m2: parent.direct_visible_w_m2,
            diffuse_visible_w_m2: parent.diffuse_visible_w_m2,
            direct_nir_w_m2: parent.direct_nir_w_m2,
            diffuse_nir_w_m2: parent.diffuse_nir_w_m2,
            downward_longwave_w_m2: parent.downward_longwave_w_m2,
            co2_pa: configuration.co2_pa,
            reference_height_m: configuration.reference_height_m,
            gsi: configuration.gsi,
            gsi_receipt_sha256: configuration.gsi_receipt_sha256.clone(),
            wb14_configuration_sha256: destination.wb14_configuration_sha256.clone(),
            precipitation_parcels,
            interval_receipt_sha256: String::new(),
        };
        interval.interval_receipt_sha256 = canonical_sha256(&interval)?;
        intervals.push(interval);
    }
    let mut next_day_precipitation_carry = Vec::with_capacity(carry_supports.len());
    for (index, (start, end, intensity)) in carry_supports.iter().copied().enumerate() {
        let parent = parents[23];
        next_day_precipitation_carry.push(precipitation_parcel(
            day_index,
            48 + index,
            destination,
            source_climate_sha256,
            start,
            end,
            1_000.0 * (end - start) * intensity,
            parent.hydrometeor_temperature_c,
        ));
    }
    let mut receipt = SnowFreeHalfHourDayReceipt {
        provider_version: "OPENWEPP_SNOW_FREE_HALF_HOUR_FORCING_V1".to_string(),
        provider_definition_sha256: SNOW_FREE_PROVIDER_SHA256.to_string(),
        source_climate_sha256: source_climate_sha256.to_string(),
        run_id: configuration.run_id.clone(),
        day_index,
        daily_horizontal_energy_mj_m2: daily_energy,
        intervals,
        next_day_precipitation_carry,
        receipt_sha256: String::new(),
    };
    receipt.receipt_sha256 = canonical_sha256(&receipt)?;
    Ok(receipt)
}

#[allow(clippy::too_many_arguments)]
fn precipitation_parcel(
    day_index: usize,
    parcel_index: usize,
    destination: &SnowFreeHalfHourDestination,
    source_climate_sha256: &str,
    start_s: f64,
    end_s: f64,
    mass_kg_m2: f64,
    hydrometeor_temperature_c: f64,
) -> SnowFreePrecipitationParcelReceipt {
    let temperature_k = celsius_to_kelvin(hydrometeor_temperature_c);
    SnowFreePrecipitationParcelReceipt {
        parcel_id: format!("climate-rain:{day_index}:{parcel_index}"),
        source_owner_id: source_climate_sha256.to_string(),
        destination_ofe_id: destination.ofe_id.clone(),
        destination_tile_id: destination.tile_id.clone(),
        start_s,
        end_s,
        mass_kg_m2,
        temperature_k,
        enthalpy_j_m2: mass_kg_m2 * liquid_specific_enthalpy_j_kg(temperature_k),
    }
}

fn validate_precipitation_parcel(
    parcel: &SnowFreePrecipitationParcelReceipt,
) -> Result<(), SnowFreeHalfHourForcingError> {
    let expected = parcel.mass_kg_m2 * liquid_specific_enthalpy_j_kg(parcel.temperature_k);
    if parcel.parcel_id.is_empty()
        || parcel.source_owner_id.is_empty()
        || parcel.destination_ofe_id.is_empty()
        || parcel.destination_tile_id.is_empty()
        || !parcel.start_s.is_finite()
        || !parcel.end_s.is_finite()
        || parcel.start_s < 0.0
        || parcel.end_s <= parcel.start_s
        || !parcel.mass_kg_m2.is_finite()
        || parcel.mass_kg_m2 < 0.0
        || !parcel.temperature_k.is_finite()
        || parcel.temperature_k <= 0.0
        || parcel.enthalpy_j_m2.to_bits() != expected.to_bits()
    {
        return Err(SnowFreeHalfHourForcingError::Identity(
            "precipitation parcel",
        ));
    }
    Ok(())
}

fn canonical_sha256<T: Serialize>(value: &T) -> Result<String, SnowFreeHalfHourForcingError> {
    let mut json = serde_json::to_value(value)
        .map_err(|error| SnowFreeHalfHourForcingError::Serialization(error.to_string()))?;
    if let Some(object) = json.as_object_mut() {
        object.remove("interval_receipt_sha256");
        object.remove("receipt_sha256");
    }
    let mut bytes = serde_json::to_vec(&json)
        .map_err(|error| SnowFreeHalfHourForcingError::Serialization(error.to_string()))?;
    bytes.push(b'\n');
    Ok(format!("{:x}", Sha256::digest(bytes)))
}
