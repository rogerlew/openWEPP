use openwepp_coupled_time::Digest32;
use openwepp_meteorology::snow_free_forcing::{
    SnowFreeAtmosphericError, atmospheric_longwave_dilley_unsworth, celsius_to_kelvin,
    fao56_station_pressure_kpa, liquid_specific_enthalpy_j_kg, weiss_norman_partition,
};
use openwepp_plant_phenology::{
    GsiDailyForcing, GsiDailyIndicators, GsiDailyResult, GsiDate, GsiError, GsiParameters, GsiState,
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

impl SnowFreeHalfHourStaticConfiguration {
    /// Validate static provider identity independently of any daily GSI value.
    pub fn validate(&self) -> Result<(), SnowFreeHalfHourForcingError> {
        validate_snow_free_static_configuration(self)
    }

    #[must_use]
    pub fn configuration_sha256(&self) -> String {
        snow_free_static_configuration_sha256(self)
    }
}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct DirectGsiDateV1 {
    pub year: i32,
    pub ordinal_day: u16,
}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct DirectGsiOwnerStateV1 {
    pub history_oldest_first: Vec<f64>,
    pub last_date: Option<DirectGsiDateV1>,
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

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct DirectGsiOwnerConfigurationV1 {
    pub schema_version: String,
    pub owner_id: String,
    pub parameters: DirectGsiParametersV1,
    pub latitude_degrees: f64,
    pub configuration_sha256: String,
}

impl DirectGsiOwnerConfigurationV1 {
    #[must_use]
    pub const fn parameters(&self) -> GsiParameters {
        gsi_parameters(self.parameters)
    }

    pub fn try_new(
        owner_id: String,
        parameters: GsiParameters,
        latitude_degrees: f64,
    ) -> Result<Self, SnowFreeHalfHourForcingError> {
        let mut value = Self {
            schema_version: "DIRECT_GSI_OWNER_CONFIGURATION_V1".into(),
            owner_id,
            parameters: direct_gsi_parameters(parameters),
            latitude_degrees,
            configuration_sha256: String::new(),
        };
        value.configuration_sha256 = canonical_sha256(&value)?;
        value.validate()?;
        Ok(value)
    }

    pub fn validate(&self) -> Result<(), SnowFreeHalfHourForcingError> {
        self.parameters().validate()?;
        let mut canonical = self.clone();
        canonical.configuration_sha256.clear();
        if self.schema_version != "DIRECT_GSI_OWNER_CONFIGURATION_V1"
            || self.owner_id.is_empty()
            || !self.latitude_degrees.is_finite()
            || !(-90.0..=90.0).contains(&self.latitude_degrees)
            || self.configuration_sha256 != canonical_sha256(&canonical)?
        {
            return Err(SnowFreeHalfHourForcingError::Identity(
                "GSI owner configuration",
            ));
        }
        Ok(())
    }
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
    pub sample_count: u32,
}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct DirectGsiDailyReceiptV1 {
    pub schema_version: String,
    pub owner_id: String,
    pub run_id: String,
    pub day_index: u64,
    pub source_climate_sha256: String,
    pub beginning_state: DirectGsiOwnerStateV1,
    pub ending_state: DirectGsiOwnerStateV1,
    pub parameters: DirectGsiParametersV1,
    pub forcing: DirectGsiForcingV1,
    pub result: DirectGsiResultV1,
    pub configuration_sha256: String,
    pub beginning_state_sha256: String,
    pub ending_state_sha256: String,
    pub forcing_sha256: String,
    pub result_sha256: String,
    pub receipt_sha256: String,
}

impl DirectGsiDailyReceiptV1 {
    pub fn prepare_owned(
        beginning: &GsiState,
        owner_configuration: &DirectGsiOwnerConfigurationV1,
        run_id: &str,
        day_index: u64,
        source_climate_sha256: &str,
        forcing: GsiDailyForcing,
    ) -> Result<(Self, GsiState), SnowFreeHalfHourForcingError> {
        owner_configuration.validate()?;
        if run_id.is_empty()
            || !is_sha256(source_climate_sha256)
            || forcing.latitude_degrees.to_bits() != owner_configuration.latitude_degrees.to_bits()
        {
            return Err(SnowFreeHalfHourForcingError::Identity(
                "daily GSI source join",
            ));
        }
        let parameters = gsi_parameters(owner_configuration.parameters);
        let mut ending = beginning.clone();
        let result = ending.advance(parameters, forcing)?;
        let beginning_state = direct_gsi_state(beginning)?;
        let ending_state = direct_gsi_state(&ending)?;
        let parameters = direct_gsi_parameters(parameters);
        let forcing = direct_gsi_forcing(forcing);
        let result = direct_gsi_result(result)?;
        let mut receipt = Self {
            schema_version: "DIRECT_GSI_DAILY_RECEIPT_V1".into(),
            owner_id: owner_configuration.owner_id.clone(),
            run_id: run_id.into(),
            day_index,
            source_climate_sha256: source_climate_sha256.into(),
            beginning_state_sha256: beginning_state.state_sha256.clone(),
            ending_state_sha256: ending_state.state_sha256.clone(),
            beginning_state,
            ending_state,
            configuration_sha256: owner_configuration.configuration_sha256.clone(),
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
        let owner_configuration = DirectGsiOwnerConfigurationV1 {
            schema_version: "DIRECT_GSI_OWNER_CONFIGURATION_V1".into(),
            owner_id: self.owner_id.clone(),
            parameters: self.parameters,
            latitude_degrees: self.forcing.latitude_degrees,
            configuration_sha256: self.configuration_sha256.clone(),
        };
        owner_configuration.validate()?;
        if self.schema_version != "DIRECT_GSI_DAILY_RECEIPT_V1"
            || self.owner_id.is_empty()
            || self.run_id.is_empty()
            || !is_sha256(&self.source_climate_sha256)
            || !is_sha256(&self.configuration_sha256)
            || self.beginning_state_sha256 != self.beginning_state.state_sha256
            || self.ending_state_sha256 != self.ending_state.state_sha256
            || self.forcing_sha256 != canonical_sha256(&self.forcing)?
            || self.result_sha256 != canonical_sha256(&self.result)?
        {
            return Err(SnowFreeHalfHourForcingError::Identity("daily GSI receipt"));
        }
        let beginning = restore_direct_gsi_state(&self.beginning_state)?;
        let mut ending = beginning;
        let result = direct_gsi_result(
            ending.advance(gsi_parameters(self.parameters), gsi_forcing(self.forcing))?,
        )?;
        if direct_gsi_state(&ending)? != self.ending_state
            || result != self.result
            || self.receipt_sha256 != canonical_sha256_without_receipt(self)?
        {
            return Err(SnowFreeHalfHourForcingError::Identity("daily GSI closure"));
        }
        Ok(())
    }
}

pub(crate) fn direct_gsi_state(
    state: &GsiState,
) -> Result<DirectGsiOwnerStateV1, SnowFreeHalfHourForcingError> {
    let mut value = DirectGsiOwnerStateV1 {
        history_oldest_first: state.history(),
        last_date: state.last_date().map(|date| DirectGsiDateV1 {
            year: date.year,
            ordinal_day: date.ordinal_day,
        }),
        state_sha256: String::new(),
    };
    value.state_sha256 = canonical_sha256(&value)?;
    Ok(value)
}

fn restore_direct_gsi_state(
    value: &DirectGsiOwnerStateV1,
) -> Result<GsiState, SnowFreeHalfHourForcingError> {
    let mut canonical = value.clone();
    canonical.state_sha256.clear();
    if value.state_sha256 != canonical_sha256(&canonical)? {
        return Err(SnowFreeHalfHourForcingError::Identity("GSI state digest"));
    }
    let last_date = value.last_date.as_ref().map(|date| GsiDate {
        year: date.year,
        ordinal_day: date.ordinal_day,
    });
    Ok(GsiState::try_from_history(
        &value.history_oldest_first,
        last_date,
    )?)
}

#[cfg(any(
    feature = "restart-authority-evidence",
    feature = "persisted-restart-v1"
))]
pub fn restart_authority_restore_gsi_state(
    value: &DirectGsiOwnerStateV1,
) -> Result<GsiState, SnowFreeHalfHourForcingError> {
    restore_direct_gsi_state(value)
}

/// Project the actual live GSI state for restart-authority evidence.
#[cfg(any(
    feature = "restart-authority-evidence",
    feature = "persisted-restart-v1"
))]
pub fn restart_authority_project_gsi_state(
    state: &GsiState,
) -> Result<DirectGsiOwnerStateV1, SnowFreeHalfHourForcingError> {
    direct_gsi_state(state)
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

fn direct_gsi_result(
    value: GsiDailyResult,
) -> Result<DirectGsiResultV1, SnowFreeHalfHourForcingError> {
    let GsiDailyIndicators {
        minimum_temperature,
        vapor_pressure_deficit,
        photoperiod,
        instantaneous_gsi,
        photoperiod_hours,
    } = value.indicators;
    Ok(DirectGsiResultV1 {
        minimum_temperature_indicator: minimum_temperature,
        vapor_pressure_deficit_indicator: vapor_pressure_deficit,
        photoperiod_indicator: photoperiod,
        instantaneous_gsi,
        photoperiod_hours,
        growing_season_index: value.growing_season_index,
        sample_count: u32::try_from(value.sample_count)
            .map_err(|_| SnowFreeHalfHourForcingError::Identity("GSI sample count"))?,
    })
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
    pending_solid_carry: Vec<SnowFreeSolidPrecipitationParcelReceipt>,
}

#[derive(Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
struct SnowFreeHalfHourProviderCursorSnapshot {
    next_day_index: usize,
    configuration_sha256: Option<String>,
    pending_carry: Vec<SnowFreePrecipitationParcelReceipt>,
    pending_solid_carry: Vec<SnowFreeSolidPrecipitationParcelReceipt>,
}

impl SnowFreeHalfHourProviderCursor {
    /// Validate this live cursor against its static owner and scheduler position.
    pub fn validate_for_configuration(
        &self,
        configuration: &SnowFreeHalfHourStaticConfiguration,
        expected_next_day_index: usize,
    ) -> Result<(), SnowFreeHalfHourForcingError> {
        configuration.validate()?;
        if self.next_day_index != expected_next_day_index
            || self
                .configuration_sha256
                .as_ref()
                .is_some_and(|digest| digest != &configuration.configuration_sha256())
            || (self.configuration_sha256.is_none()
                && (!self.pending_carry.is_empty() || !self.pending_solid_carry.is_empty()))
        {
            return Err(SnowFreeHalfHourForcingError::Identity(
                "provider cursor configuration",
            ));
        }
        Ok(())
    }

    /// Serialize the complete provider cursor for a persisted restart owner.
    pub fn to_json_bytes(&self) -> Result<Vec<u8>, SnowFreeHalfHourForcingError> {
        serde_json::to_vec(self)
            .map_err(|error| SnowFreeHalfHourForcingError::Serialization(error.to_string()))
    }

    /// Restore and validate a cursor against the expected static provider
    /// configuration and scheduler day.
    pub fn restore_json(
        bytes: &[u8],
        configuration: &SnowFreeHalfHourStaticConfiguration,
        expected_next_day_index: usize,
    ) -> Result<Self, SnowFreeHalfHourForcingError> {
        validate_snow_free_static_configuration(configuration)?;
        let snapshot: SnowFreeHalfHourProviderCursorSnapshot = serde_json::from_slice(bytes)
            .map_err(|error| SnowFreeHalfHourForcingError::Serialization(error.to_string()))?;
        let value = Self {
            next_day_index: snapshot.next_day_index,
            configuration_sha256: snapshot.configuration_sha256,
            pending_carry: snapshot.pending_carry,
            pending_solid_carry: snapshot.pending_solid_carry,
        };
        let expected_configuration_sha256 = snow_free_static_configuration_sha256(configuration);
        if value.next_day_index != expected_next_day_index
            || value.configuration_sha256.as_deref() != Some(expected_configuration_sha256.as_str())
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
            )) || !destinations
                .contains(&(&parcel.destination_ofe_id, &parcel.destination_tile_id))
                || parcel.start_s < 0.0
                || parcel.end_s > 86_400.0
            {
                return Err(SnowFreeHalfHourForcingError::Identity(
                    "restored provider carry",
                ));
            }
        }
        for parcel in &value.pending_solid_carry {
            validate_solid_precipitation_parcel(parcel)?;
            if !parcel_ids.insert((
                &parcel.destination_ofe_id,
                &parcel.destination_tile_id,
                &parcel.parcel_id,
            )) || !destinations
                .contains(&(&parcel.destination_ofe_id, &parcel.destination_tile_id))
                || parcel.start_s < 0.0
                || parcel.end_s > 86_400.0
            {
                return Err(SnowFreeHalfHourForcingError::Identity(
                    "restored provider solid carry",
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

/// A solid atmospheric precipitation parcel. Solid custody is distinct from
/// liquid rain custody so snow-free LSE projection cannot reinterpret snow as
/// liquid water merely because both phases share one climate source.
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct SnowFreeSolidPrecipitationParcelReceipt {
    pub parcel_id: String,
    pub source_owner_id: String,
    pub destination_ofe_id: String,
    pub destination_tile_id: String,
    pub start_s: f64,
    pub end_s: f64,
    pub mass_kg_m2: f64,
    pub temperature_k: f64,
    /// Sensible ice enthalpy relative to ice at the melting point.  Solid
    /// parcels never borrow the liquid-water enthalpy convention.
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
    pub active_precipitation_m: f64,
    pub rain_m: f64,
    pub snowfall_m: f64,
    pub rain_fraction: f64,
    pub snow_fraction: f64,
    pub hydrometeor_temperature_c: Option<f64>,
    pub precipitation_parcels: Vec<SnowFreePrecipitationParcelReceipt>,
    pub solid_precipitation_parcels: Vec<SnowFreeSolidPrecipitationParcelReceipt>,
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
    pub next_day_solid_precipitation_carry: Vec<SnowFreeSolidPrecipitationParcelReceipt>,
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

#[derive(Clone, Debug)]
pub struct PreparedSnowFreeGsiDayV1 {
    gsi_receipt: DirectGsiDailyReceiptV1,
    ending_gsi_state: GsiState,
    forcing_receipts: ValidatedSnowFreeHalfHourForcingReceipts,
}

impl PreparedSnowFreeGsiDayV1 {
    #[must_use]
    pub const fn gsi_receipt(&self) -> &DirectGsiDailyReceiptV1 {
        &self.gsi_receipt
    }

    #[must_use]
    pub const fn forcing_receipts(&self) -> &ValidatedSnowFreeHalfHourForcingReceipts {
        &self.forcing_receipts
    }

    pub fn gsi_receipt_digest(&self) -> Result<Digest32, SnowFreeHalfHourForcingError> {
        digest_from_lower_hex(&self.gsi_receipt.receipt_sha256)
    }

    /// Commit both stateful provider owners after every fallible guard. The
    /// final replacement is deliberately assignment-only and cannot fail.
    pub fn commit(
        self,
        gsi_state: &mut GsiState,
        cursor: &mut SnowFreeHalfHourProviderCursor,
    ) -> Result<(), SnowFreeHalfHourForcingError> {
        self.gsi_receipt.validate()?;
        if direct_gsi_state(gsi_state)? != self.gsi_receipt.beginning_state
            || cursor != &self.forcing_receipts.beginning_cursor
        {
            return Err(SnowFreeHalfHourForcingError::Identity(
                "GSI/provider atomic commit beginning",
            ));
        }
        let ending_cursor = self.forcing_receipts.ending_cursor;
        *gsi_state = self.ending_gsi_state;
        *cursor = ending_cursor;
        Ok(())
    }
}

/// Reconstruct a prepared day exclusively from admitted restart owners.
#[cfg(any(
    feature = "restart-authority-evidence",
    feature = "persisted-restart-v1"
))]
pub fn restart_authority_prepare_from_restored_receipts(
    gsi_receipt: DirectGsiDailyReceiptV1,
    ending_gsi_state: GsiState,
    receipts: Vec<SnowFreeHalfHourDayReceipt>,
    beginning_cursor: SnowFreeHalfHourProviderCursor,
    ending_cursor: SnowFreeHalfHourProviderCursor,
    configuration: &SnowFreeHalfHourStaticConfiguration,
) -> Result<PreparedSnowFreeGsiDayV1, SnowFreeHalfHourForcingError> {
    gsi_receipt.validate()?;
    let direct_ending = direct_gsi_state(&ending_gsi_state)?;
    let gsi_day_index = usize::try_from(gsi_receipt.day_index)
        .map_err(|_| SnowFreeHalfHourForcingError::Identity("restart GSI day index width"))?;
    if direct_ending != gsi_receipt.ending_state {
        return Err(SnowFreeHalfHourForcingError::Identity(
            "restart staged GSI ending state",
        ));
    }
    if configuration.run_id != gsi_receipt.run_id
        || configuration.gsi_owner_configuration_sha256 != gsi_receipt.configuration_sha256
    {
        return Err(SnowFreeHalfHourForcingError::Identity(
            "restart static/GSI configuration join",
        ));
    }
    beginning_cursor.validate_for_configuration(configuration, gsi_day_index)?;
    ending_cursor.validate_for_configuration(
        configuration,
        gsi_day_index
            .checked_add(1)
            .ok_or(SnowFreeHalfHourForcingError::Identity(
                "restart ending cursor day overflow",
            ))?,
    )?;
    if receipts.len() != configuration.destinations.len() {
        return Err(SnowFreeHalfHourForcingError::Identity(
            "restart forcing destination cardinality",
        ));
    }
    for (receipt, destination) in receipts.iter().zip(&configuration.destinations) {
        receipt.validate()?;
        if receipt.run_id != gsi_receipt.run_id
            || receipt.day_index != gsi_day_index
            || receipt.source_climate_sha256 != gsi_receipt.source_climate_sha256
            || receipt.intervals.len() != 48
            || receipt.intervals.iter().any(|interval| {
                interval.ofe_id != destination.ofe_id
                    || interval.tile_id != destination.tile_id
                    || interval.wb14_configuration_sha256 != destination.wb14_configuration_sha256
                    || interval.co2_pa.to_bits() != configuration.co2_pa.to_bits()
                    || interval.reference_height_m.to_bits()
                        != configuration.reference_height_m.to_bits()
            })
            || receipt
                .intervals
                .iter()
                .any(|interval| interval.gsi_receipt_sha256 != gsi_receipt.receipt_sha256)
        {
            return Err(SnowFreeHalfHourForcingError::Identity(
                "restart forcing/GSI receipt join",
            ));
        }
    }
    let expected_carry = receipts
        .iter()
        .flat_map(|receipt| receipt.next_day_precipitation_carry.iter().cloned())
        .collect::<Vec<_>>();
    if expected_carry != ending_cursor.pending_carry {
        return Err(SnowFreeHalfHourForcingError::Identity(
            "restart ending cursor carry",
        ));
    }
    let expected_solid_carry = receipts
        .iter()
        .flat_map(|receipt| receipt.next_day_solid_precipitation_carry.iter().cloned())
        .collect::<Vec<_>>();
    if expected_solid_carry != ending_cursor.pending_solid_carry {
        return Err(SnowFreeHalfHourForcingError::Identity(
            "restart ending cursor solid carry",
        ));
    }
    for pending in &beginning_cursor.pending_carry {
        let found = receipts
            .iter()
            .flat_map(|receipt| &receipt.intervals)
            .flat_map(|interval| &interval.precipitation_parcels)
            .filter(|parcel| *parcel == pending)
            .count();
        if found != 1 {
            return Err(SnowFreeHalfHourForcingError::Identity(
                "restart beginning cursor carry consumption",
            ));
        }
    }
    for pending in &beginning_cursor.pending_solid_carry {
        let found = receipts
            .iter()
            .flat_map(|receipt| &receipt.intervals)
            .flat_map(|interval| &interval.solid_precipitation_parcels)
            .filter(|parcel| *parcel == pending)
            .count();
        if found != 1 {
            return Err(SnowFreeHalfHourForcingError::Identity(
                "restart beginning cursor solid carry consumption",
            ));
        }
    }
    Ok(PreparedSnowFreeGsiDayV1 {
        gsi_receipt,
        ending_gsi_state,
        forcing_receipts: ValidatedSnowFreeHalfHourForcingReceipts {
            receipts,
            beginning_cursor,
            ending_cursor,
        },
    })
}

impl ValidatedSnowFreeHalfHourForcingReceipts {
    #[must_use]
    pub fn receipts(&self) -> &[SnowFreeHalfHourDayReceipt] {
        &self.receipts
    }

    #[must_use]
    pub const fn beginning_cursor(&self) -> &SnowFreeHalfHourProviderCursor {
        &self.beginning_cursor
    }

    #[must_use]
    pub const fn ending_cursor(&self) -> &SnowFreeHalfHourProviderCursor {
        &self.ending_cursor
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
                return Err(SnowFreeHalfHourForcingError::Identity("interval receipt"));
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
            for parcel in &interval.solid_precipitation_parcels {
                validate_solid_precipitation_parcel(parcel)?;
                if !parcel_ids.insert(&parcel.parcel_id)
                    || parcel.destination_ofe_id != interval.ofe_id
                    || parcel.destination_tile_id != interval.tile_id
                    || parcel.start_s < interval_start_s
                    || parcel.end_s > interval_end_s
                {
                    return Err(SnowFreeHalfHourForcingError::Identity(
                        "interval solid parcel binding",
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
        for parcel in &self.next_day_solid_precipitation_carry {
            validate_solid_precipitation_parcel(parcel)?;
            if !parcel_ids.insert(&parcel.parcel_id)
                || parcel.source_owner_id != self.source_climate_sha256
                || parcel.destination_ofe_id != *expected_ofe
                || parcel.destination_tile_id != *expected_tile
                || parcel.end_s > 86_400.0
            {
                return Err(SnowFreeHalfHourForcingError::Identity(
                    "solid carry parcel binding",
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
            return Err(SnowFreeHalfHourForcingError::Identity("day receipt digest"));
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
    validate_half_hour_precipitation_phase(HalfHourPrecipitationPhase {
        active_precipitation_m: interval.active_precipitation_m,
        rain_m: interval.rain_m,
        snowfall_m: interval.snowfall_m,
        rain_fraction: interval.rain_fraction,
        snow_fraction: interval.snow_fraction,
        hydrometeor_temperature_c: interval.hydrometeor_temperature_c,
    })?;
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
    let precipitation_operands = [
        interval.active_precipitation_m,
        interval.rain_m,
        interval.snowfall_m,
        interval.rain_fraction,
        interval.snow_fraction,
    ];
    let precipitation_scale = interval.active_precipitation_m.abs().max(1.0);
    if operands.iter().any(|value| !value.is_finite())
        || precipitation_operands
            .iter()
            .any(|value| !value.is_finite())
        || precipitation_operands.iter().any(|value| *value < 0.0)
        || !(0.0..=1.0).contains(&interval.rain_fraction)
        || !(0.0..=1.0).contains(&interval.snow_fraction)
        || interval
            .hydrometeor_temperature_c
            .is_some_and(|value| !value.is_finite())
        || (interval.rain_m + interval.snowfall_m / 10.0 - interval.active_precipitation_m).abs()
            > 1.0e-12 * precipitation_scale
        || (interval.active_precipitation_m > 0.0
            && ((interval.rain_fraction + interval.snow_fraction - 1.0).abs() > 1.0e-12
                || (interval.rain_m - interval.active_precipitation_m * interval.rain_fraction)
                    .abs()
                    > 1.0e-12 * precipitation_scale
                || (interval.snowfall_m / 10.0
                    - interval.active_precipitation_m * interval.snow_fraction)
                    .abs()
                    > 1.0e-12 * precipitation_scale))
        || (interval.active_precipitation_m == 0.0
            && (interval.rain_fraction != 0.0
                || interval.snow_fraction != 0.0
                || interval.hydrometeor_temperature_c.is_some()))
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
            > 1.0e-12 * interval.global_horizontal_shortwave_w_m2.abs().max(1.0)
    {
        return Err(SnowFreeHalfHourForcingError::Closure(
            "interval physical operands",
        ));
    }
    Ok(())
}

fn validate_half_hour_precipitation_phase(
    phase: HalfHourPrecipitationPhase,
) -> Result<(), SnowFreeHalfHourForcingError> {
    let operands = [
        phase.active_precipitation_m,
        phase.rain_m,
        phase.snowfall_m,
        phase.rain_fraction,
        phase.snow_fraction,
    ];
    let scale = phase.active_precipitation_m.abs().max(1.0);
    let invalid_domain = operands.iter().any(|value| !value.is_finite() || *value < 0.0)
        || !(0.0..=1.0).contains(&phase.rain_fraction)
        || !(0.0..=1.0).contains(&phase.snow_fraction)
        || phase
            .hydrometeor_temperature_c
            .is_some_and(|value| !value.is_finite());
    let invalid_dry = phase.active_precipitation_m == 0.0
        && (phase.rain_m != 0.0
            || phase.snowfall_m != 0.0
            || phase.rain_fraction != 0.0
            || phase.snow_fraction != 0.0
            || phase.hydrometeor_temperature_c.is_some());
    let invalid_wet = phase.active_precipitation_m > 0.0
        && (phase.hydrometeor_temperature_c.is_none()
            || (phase.rain_m + phase.snowfall_m / 10.0 - phase.active_precipitation_m).abs()
                > 1.0e-12 * scale
            || (phase.rain_fraction + phase.snow_fraction - 1.0).abs() > 1.0e-12
            || (phase.rain_m - phase.active_precipitation_m * phase.rain_fraction).abs()
                > 1.0e-12 * scale
            || (phase.snowfall_m / 10.0
                - phase.active_precipitation_m * phase.snow_fraction)
                .abs()
                > 1.0e-12 * scale);
    if invalid_domain || invalid_dry || invalid_wet {
        return Err(SnowFreeHalfHourForcingError::Closure(
            "interval precipitation phase",
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

#[derive(Clone, Copy, Debug, Default)]
struct HalfHourPrecipitationPhase {
    active_precipitation_m: f64,
    rain_m: f64,
    snowfall_m: f64,
    rain_fraction: f64,
    snow_fraction: f64,
    hydrometeor_temperature_c: Option<f64>,
}

#[derive(Clone, Copy, Debug)]
struct PrecipitationCarrySupport {
    start_s: f64,
    end_s: f64,
    phase: HalfHourPrecipitationPhase,
}

type ChildPhaseAndCarry = (
    [HalfHourPrecipitationPhase; 48],
    Vec<PrecipitationCarrySupport>,
);

impl HillslopeClimateRuntimeRequest {
    /// Canonical production solar-geometry authority used by the Stage-3
    /// surface-energy capability. This projects only the immutable climate
    /// date/radiation and site geometry; it does not invoke the retired
    /// diagnostic winter snow/phase forcing path.
    pub fn stage3_daily_extraterrestrial_radiation_mj_m2(
        &self,
        day_index: usize,
        avg_slope: f64,
        azimuth: f64,
    ) -> Result<f64, SnowFreeHalfHourForcingError> {
        let selected = select_day_forcing(&self.shared, day_index)?;
        let (day, month, year, radly) = match selected {
            HillslopeClimateDailyForcing::NoBreakpoint(value) => {
                (value.day, value.mon, value.year, value.rad)
            }
            HillslopeClimateDailyForcing::Breakpoint(value) => {
                (value.day, value.mon, value.year, value.rad)
            }
        };
        let day_of_year = simimpl28_day_of_year(day, month, year)?;
        let geometry = simimpl28_aspect_geometry(self.metadata.deglat, avg_slope, azimuth)?;
        Ok(simimpl28_sunmap(radly, day_of_year, geometry)?.rpoth_mj_m2)
    }

    pub fn prepare_snow_free_gsi_day_from_repository(
        &self,
        day_index: usize,
        configuration: &SnowFreeHalfHourStaticConfiguration,
        gsi_owner_configuration: &DirectGsiOwnerConfigurationV1,
        gsi_state: &GsiState,
        cursor: &SnowFreeHalfHourProviderCursor,
    ) -> Result<PreparedSnowFreeGsiDayV1, SnowFreeHalfHourForcingError> {
        gsi_owner_configuration.validate()?;
        if configuration.run_id.is_empty()
            || configuration.gsi_owner_configuration_sha256
                != gsi_owner_configuration.configuration_sha256
            || self.metadata.deglat.to_bits() != gsi_owner_configuration.latitude_degrees.to_bits()
        {
            return Err(SnowFreeHalfHourForcingError::Identity(
                "repository GSI owner join",
            ));
        }
        let selected = select_day_forcing(&self.shared, day_index)?;
        let source_climate_sha256 = source_climate_sha256(self, day_index, selected);
        let (day, month, year, tmin, tmax, dew_point) = match selected {
            HillslopeClimateDailyForcing::NoBreakpoint(value) => (
                value.day, value.mon, value.year, value.tmin, value.tmax, value.tdpt,
            ),
            HillslopeClimateDailyForcing::Breakpoint(value) => (
                value.day, value.mon, value.year, value.tmin, value.tmax, value.tdpt,
            ),
        };
        let ordinal_day = u16::try_from(simimpl28_day_of_year(day, month, year)?)
            .map_err(|_| SnowFreeHalfHourForcingError::Identity("repository GSI date"))?;
        let saturation_kpa = |temperature_c| {
            saturation_vapor_pressure_water_kpa(
                TemperatureCelsius::try_new(temperature_c)
                    .map_err(|_| SnowFreeHalfHourForcingError::Unsupported("GSI daily VPD"))?,
            )
            .map(openwepp_meteorology::psychrometrics::VaporPressureKilopascals::as_kilopascals)
            .map_err(|_| SnowFreeHalfHourForcingError::Unsupported("GSI daily VPD"))
        };
        let mean_saturation_kpa = f64::midpoint(saturation_kpa(tmax)?, saturation_kpa(tmin)?);
        let actual_vapor_pressure_kpa = saturation_kpa(dew_point)?;
        let vapor_pressure_deficit_pa = (mean_saturation_kpa - actual_vapor_pressure_kpa) * 1_000.0;
        if !vapor_pressure_deficit_pa.is_finite() || vapor_pressure_deficit_pa < 0.0 {
            return Err(SnowFreeHalfHourForcingError::Unsupported("GSI daily VPD"));
        }
        let forcing = GsiDailyForcing {
            minimum_temperature_c: tmin,
            vapor_pressure_deficit_pa,
            latitude_degrees: self.metadata.deglat,
            date: GsiDate { year, ordinal_day },
        };
        let (gsi_receipt, ending_gsi_state) = DirectGsiDailyReceiptV1::prepare_owned(
            gsi_state,
            gsi_owner_configuration,
            &configuration.run_id,
            u64::try_from(day_index)
                .map_err(|_| SnowFreeHalfHourForcingError::Identity("GSI day index"))?,
            &source_climate_sha256,
            forcing,
        )?;
        let forcing_receipts = self.snow_free_half_hour_forcing_receipts_with_gsi(
            day_index,
            configuration,
            &gsi_receipt,
            cursor,
        )?;
        Ok(PreparedSnowFreeGsiDayV1 {
            gsi_receipt,
            ending_gsi_state,
            forcing_receipts,
        })
    }

    /// Closure-eligible projection joining static provider configuration to an
    /// accepted CP-GSI01 daily receipt. Daily GSI is never part of cursor
    /// identity and cannot be supplied independently of its receipt.
    fn snow_free_half_hour_forcing_receipts_with_gsi(
        &self,
        day_index: usize,
        configuration: &SnowFreeHalfHourStaticConfiguration,
        gsi_receipt: &DirectGsiDailyReceiptV1,
        cursor: &SnowFreeHalfHourProviderCursor,
    ) -> Result<ValidatedSnowFreeHalfHourForcingReceipts, SnowFreeHalfHourForcingError> {
        gsi_receipt.validate()?;
        let selected = select_day_forcing(&self.shared, day_index)?;
        let expected_source_climate_sha256 = source_climate_sha256(self, day_index, selected);
        if configuration.gsi_owner_configuration_sha256 != gsi_receipt.configuration_sha256
            || configuration.run_id != gsi_receipt.run_id
            || gsi_receipt.day_index
                != u64::try_from(day_index)
                    .map_err(|_| SnowFreeHalfHourForcingError::Identity("GSI day index"))?
            || gsi_receipt.source_climate_sha256 != expected_source_climate_sha256
            || gsi_receipt.forcing.latitude_degrees.to_bits() != self.metadata.deglat.to_bits()
        {
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
        let (child_phases, carry_supports) = precipitation_child_phases(forcing, &parents)?;
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
                &child_phases,
                &carry_supports,
                daily_energy,
            )?;
            receipt.validate()?;
            receipts.push(receipt);
        }
        apply_pending_carry(
            &mut receipts,
            &cursor.pending_carry,
            &cursor.pending_solid_carry,
        )?;
        let pending_carry = receipts
            .iter()
            .flat_map(|receipt| receipt.next_day_precipitation_carry.iter().cloned())
            .collect();
        let pending_solid_carry = receipts
            .iter()
            .flat_map(|receipt| receipt.next_day_solid_precipitation_carry.iter().cloned())
            .collect();
        let mut ending_cursor = cursor.clone();
        ending_cursor.pending_carry = pending_carry;
        ending_cursor.pending_solid_carry = pending_solid_carry;
        ending_cursor.configuration_sha256 = Some(configuration_sha256);
        ending_cursor.next_day_index += 1;
        Ok(ValidatedSnowFreeHalfHourForcingReceipts {
            receipts,
            beginning_cursor: cursor.clone(),
            ending_cursor,
        })
    }
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
    pending_solid: &[SnowFreeSolidPrecipitationParcelReceipt],
) -> Result<(), SnowFreeHalfHourForcingError> {
    for parcel in pending {
        let receipt = receipts
            .iter_mut()
            .find(|receipt| {
                receipt.intervals[0].ofe_id == parcel.destination_ofe_id
                    && receipt.intervals[0].tile_id == parcel.destination_tile_id
            })
            .ok_or(SnowFreeHalfHourForcingError::Identity("carry destination"))?;
        let mut interval_index = None;
        for (index, interval) in receipt.intervals.iter().enumerate() {
            let start = f64::from(
                u32::try_from(interval.start_s)
                    .map_err(|_| SnowFreeHalfHourForcingError::Identity("interval support"))?,
            );
            let end = f64::from(
                u32::try_from(interval.end_s)
                    .map_err(|_| SnowFreeHalfHourForcingError::Identity("interval support"))?,
            );
            if parcel.start_s >= start && parcel.end_s <= end {
                interval_index = Some(index);
                break;
            }
        }
        let interval = receipt
            .intervals
            .get_mut(interval_index.ok_or(SnowFreeHalfHourForcingError::Identity("carry support"))?)
            .ok_or(SnowFreeHalfHourForcingError::Identity("carry support"))?;
        if interval.hydrometeor_temperature_c.is_none() {
            interval.hydrometeor_temperature_c = Some(parcel.temperature_k - 273.15);
        }
        interval.precipitation_parcels.push(parcel.clone());
        interval.rain_m += parcel.mass_kg_m2 / 1_000.0;
        interval.active_precipitation_m += parcel.mass_kg_m2 / 1_000.0;
        interval.rain_fraction = interval.rain_m / interval.active_precipitation_m;
        interval.snow_fraction = interval.snowfall_m / 10.0 / interval.active_precipitation_m;
        interval.interval_receipt_sha256 = canonical_sha256(interval)?;
    }
    for parcel in pending_solid {
        let receipt = receipts
            .iter_mut()
            .find(|receipt| {
                receipt.intervals[0].ofe_id == parcel.destination_ofe_id
                    && receipt.intervals[0].tile_id == parcel.destination_tile_id
            })
            .ok_or(SnowFreeHalfHourForcingError::Identity(
                "solid carry destination",
            ))?;
        let interval_index = receipt
            .intervals
            .iter()
            .position(|interval| {
                let start = interval.start_s as f64;
                let end = interval.end_s as f64;
                parcel.start_s >= start && parcel.end_s <= end
            })
            .ok_or(SnowFreeHalfHourForcingError::Identity(
                "solid carry support",
            ))?;
        let interval = receipt.intervals.get_mut(interval_index).ok_or(
            SnowFreeHalfHourForcingError::Identity("solid carry support"),
        )?;
        if interval.hydrometeor_temperature_c.is_none() {
            interval.hydrometeor_temperature_c = Some(parcel.temperature_k - 273.15);
        }
        interval.solid_precipitation_parcels.push(parcel.clone());
        interval.snowfall_m += parcel.mass_kg_m2 / 100.0;
        interval.active_precipitation_m += parcel.mass_kg_m2 / 1_000.0;
        interval.rain_fraction = interval.rain_m / interval.active_precipitation_m;
        interval.snow_fraction = interval.snowfall_m / 10.0 / interval.active_precipitation_m;
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

fn validate_snow_free_static_configuration(
    value: &SnowFreeHalfHourStaticConfiguration,
) -> Result<(), SnowFreeHalfHourForcingError> {
    if value.run_id.is_empty()
        || !is_sha256(&value.gsi_owner_configuration_sha256)
        || value.destinations.is_empty()
        || !value.co2_pa.is_finite()
        || value.co2_pa <= 0.0
        || !value.reference_height_m.is_finite()
        || value.reference_height_m <= 0.0
    {
        return Err(SnowFreeHalfHourForcingError::Identity(
            "static provider configuration",
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

fn digest_from_lower_hex(value: &str) -> Result<Digest32, SnowFreeHalfHourForcingError> {
    if !is_sha256(value) {
        return Err(SnowFreeHalfHourForcingError::Identity(
            "lowercase SHA-256 digest",
        ));
    }
    let mut bytes = [0_u8; 32];
    for (index, chunk) in value.as_bytes().chunks_exact(2).enumerate() {
        let text = std::str::from_utf8(chunk)
            .map_err(|_| SnowFreeHalfHourForcingError::Identity("SHA-256 digest encoding"))?;
        bytes[index] = u8::from_str_radix(text, 16)
            .map_err(|_| SnowFreeHalfHourForcingError::Identity("SHA-256 digest digits"))?;
    }
    Ok(Digest32::from_bytes(bytes))
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
    let (
        day,
        month,
        year,
        precipitation,
        tmax,
        tmin,
        radiation,
        wind,
        dew,
        storm_start,
        times,
        intensities,
    ) = match forcing {
        HillslopeClimateDailyForcing::NoBreakpoint(value) => (
            value.day,
            value.mon,
            value.year,
            value.prcp,
            value.tmax,
            value.tmin,
            value.rad,
            value.vwind,
            value.tdpt,
            None,
            &value.timem,
            &value.intsty,
        ),
        HillslopeClimateDailyForcing::Breakpoint(value) => (
            value.day,
            value.mon,
            value.year,
            value.prcp,
            value.tmax,
            value.tmin,
            value.rad,
            value.vwind,
            value.tdpt,
            Some(value.stmstr),
            &value.timem,
            &value.intsty,
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
            value.day,
            value.mon,
            value.year,
            value.tmax,
            value.tmin,
            value.rad,
            value.tdpt,
            value.vwind,
        ),
        HillslopeClimateDailyForcing::Breakpoint(value) => (
            value.day,
            value.mon,
            value.year,
            value.tmax,
            value.tmin,
            value.rad,
            value.tdpt,
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
    let phase_relative_humidity =
        FractionUnitInterval::try_new((actual_vapor_pressure_kpa / saturation).min(1.0))
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

fn precipitation_child_phases(
    forcing: &HillslopeClimateDailyForcing,
    parents: &[SnowFreeHourlyParent; 24],
) -> Result<ChildPhaseAndCarry, SnowFreeHalfHourForcingError> {
    let mut phases = [HalfHourPrecipitationPhase::default(); 48];
    let mut carry = Vec::new();
    match forcing {
        HillslopeClimateDailyForcing::Breakpoint(value) => {
            let offset = value.stmstr * 3_600.0;
            for segment in 0..value.timem.len().saturating_sub(1) {
                let start = offset + value.timem[segment];
                let end = offset + value.timem[segment + 1];
                let intensity = value.intsty[segment];
                for (index, child) in phases.iter_mut().enumerate() {
                    let index_u32 = u32::try_from(index).map_err(|_| {
                        SnowFreeHalfHourForcingError::Identity("child interval index")
                    })?;
                    let child_start = f64::from(index_u32) * SNOW_FREE_INTERVAL_S;
                    let child_end = child_start + SNOW_FREE_INTERVAL_S;
                    let overlap = (child_end.min(end) - child_start.max(start)).max(0.0);
                    let active_m = overlap * intensity;
                    if active_m > 0.0 {
                        let snow_fraction = parents[index / 2].snow_fraction;
                        child.active_precipitation_m += active_m;
                        child.rain_m += active_m * (1.0 - snow_fraction);
                        child.snowfall_m += active_m * snow_fraction * 10.0;
                        child.hydrometeor_temperature_c =
                            Some(parents[index / 2].hydrometeor_temperature_c);
                    }
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
                            let active_m = (overlap_end - overlap_start) * intensity;
                            let snow_fraction = parents[23].snow_fraction;
                            carry.push(PrecipitationCarrySupport {
                                start_s: overlap_start,
                                end_s: overlap_end,
                                phase: HalfHourPrecipitationPhase {
                                    active_precipitation_m: active_m,
                                    rain_m: active_m * (1.0 - snow_fraction),
                                    snowfall_m: active_m * snow_fraction * 10.0,
                                    rain_fraction: 1.0 - snow_fraction,
                                    snow_fraction,
                                    hydrometeor_temperature_c: Some(
                                        parents[23].hydrometeor_temperature_c,
                                    ),
                                },
                            });
                        }
                    }
                }
            }
            for child in &mut phases {
                if child.active_precipitation_m > 0.0 {
                    child.rain_fraction = child.rain_m / child.active_precipitation_m;
                    child.snow_fraction = child.snowfall_m / 10.0 / child.active_precipitation_m;
                }
            }
            let admitted_depth = phases
                .iter()
                .map(|child| child.active_precipitation_m)
                .sum::<f64>()
                + carry
                    .iter()
                    .map(|support| support.phase.active_precipitation_m)
                    .sum::<f64>();
            if (admitted_depth - value.prcp).abs() > 1.0e-12 * value.prcp.abs().max(1.0) {
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
                let hour_index = usize::try_from(hour - 1)
                    .map_err(|_| SnowFreeHalfHourForcingError::Identity("parent hour index"))?;
                let child = if partition.active_precipitation_m > 0.0 {
                    HalfHourPrecipitationPhase {
                        active_precipitation_m: 0.5 * partition.active_precipitation_m,
                        rain_m: 0.5 * partition.hrrain_m,
                        snowfall_m: 0.5 * partition.hrsnow_m,
                        rain_fraction: partition.rain_fraction,
                        snow_fraction: partition.snow_fraction,
                        hydrometeor_temperature_c: partition.hydrometeor_temperature_c,
                    }
                } else {
                    HalfHourPrecipitationPhase::default()
                };
                phases[2 * hour_index] = child;
                phases[2 * hour_index + 1] = child;
            }
            if (phases
                .iter()
                .map(|child| child.active_precipitation_m)
                .sum::<f64>()
                - value.prcp)
                .abs()
                > 1.0e-12 * value.prcp.abs().max(1.0)
            {
                return Err(SnowFreeHalfHourForcingError::Closure(
                    "parent-hour daily precipitation",
                ));
            }
        }
    }
    Ok((phases, carry))
}

#[allow(clippy::too_many_arguments)]
fn build_destination_receipt(
    day_index: usize,
    configuration: &SnowFreeHalfHourProviderConfiguration,
    destination: &SnowFreeHalfHourDestination,
    source_climate_sha256: &str,
    parents: &[SnowFreeHourlyParent; 24],
    child_phases: &[HalfHourPrecipitationPhase; 48],
    carry_supports: &[PrecipitationCarrySupport],
    daily_energy: f64,
) -> Result<SnowFreeHalfHourDayReceipt, SnowFreeHalfHourForcingError> {
    let mut intervals = Vec::with_capacity(SNOW_FREE_INTERVAL_COUNT);
    for interval_index in 0..SNOW_FREE_INTERVAL_COUNT {
        let parent = parents[interval_index / 2];
        let interval_u32 = u32::try_from(interval_index)
            .map_err(|_| SnowFreeHalfHourForcingError::Identity("child interval index"))?;
        let interval_start_s = f64::from(interval_u32) * SNOW_FREE_INTERVAL_S;
        let interval_end_s = interval_start_s + SNOW_FREE_INTERVAL_S;
        let phase = child_phases[interval_index];
        let precipitation_parcels = if phase.rain_m == 0.0 {
            Vec::new()
        } else {
            vec![precipitation_parcel(
                day_index,
                interval_index,
                destination,
                source_climate_sha256,
                interval_start_s,
                interval_end_s,
                phase.rain_m * 1_000.0,
                phase
                    .hydrometeor_temperature_c
                    .unwrap_or(parent.hydrometeor_temperature_c),
            )]
        };
        let solid_precipitation_parcels = if phase.snowfall_m == 0.0 {
            Vec::new()
        } else {
            vec![solid_precipitation_parcel(
                day_index,
                interval_index,
                destination,
                source_climate_sha256,
                interval_start_s,
                interval_end_s,
                phase.snowfall_m * 100.0,
                phase
                    .hydrometeor_temperature_c
                    .unwrap_or(parent.hydrometeor_temperature_c),
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
            active_precipitation_m: phase.active_precipitation_m,
            rain_m: phase.rain_m,
            snowfall_m: phase.snowfall_m,
            rain_fraction: phase.rain_fraction,
            snow_fraction: phase.snow_fraction,
            hydrometeor_temperature_c: phase.hydrometeor_temperature_c,
            precipitation_parcels,
            solid_precipitation_parcels,
            interval_receipt_sha256: String::new(),
        };
        interval.interval_receipt_sha256 = canonical_sha256(&interval)?;
        intervals.push(interval);
    }
    let mut next_day_precipitation_carry = Vec::with_capacity(carry_supports.len());
    let mut next_day_solid_precipitation_carry = Vec::with_capacity(carry_supports.len());
    for (index, support) in carry_supports.iter().copied().enumerate() {
        let parent = parents[23];
        let hydrometeor_temperature_c = support
            .phase
            .hydrometeor_temperature_c
            .unwrap_or(parent.hydrometeor_temperature_c);
        if support.phase.rain_m > 0.0 {
            next_day_precipitation_carry.push(precipitation_parcel(
                day_index,
                48 + index,
                destination,
                source_climate_sha256,
                support.start_s,
                support.end_s,
                support.phase.rain_m * 1_000.0,
                hydrometeor_temperature_c,
            ));
        }
        if support.phase.snowfall_m > 0.0 {
            next_day_solid_precipitation_carry.push(solid_precipitation_parcel(
                day_index,
                48 + index,
                destination,
                source_climate_sha256,
                support.start_s,
                support.end_s,
                support.phase.snowfall_m * 100.0,
                hydrometeor_temperature_c,
            ));
        }
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
        next_day_solid_precipitation_carry,
        receipt_sha256: String::new(),
    };
    receipt.receipt_sha256 = canonical_sha256(&receipt)?;
    Ok(receipt)
}

#[allow(clippy::too_many_arguments)]
fn solid_precipitation_parcel(
    day_index: usize,
    parcel_index: usize,
    destination: &SnowFreeHalfHourDestination,
    source_climate_sha256: &str,
    start_s: f64,
    end_s: f64,
    mass_kg_m2: f64,
    hydrometeor_temperature_c: f64,
) -> SnowFreeSolidPrecipitationParcelReceipt {
    const ICE_HEAT_CAPACITY_J_KG_K: f64 = 2_100.0;
    const MELTING_TEMPERATURE_K: f64 = 273.15;
    // Harder-Pomeroy supplies one bulk hydrometeor temperature for a mixed
    // phase. The solid parcel is separately bounded by the melting point;
    // liquid custody retains the unmodified bulk temperature below.
    let temperature_k = celsius_to_kelvin(hydrometeor_temperature_c.min(0.0));
    SnowFreeSolidPrecipitationParcelReceipt {
        parcel_id: format!("climate-snow:{day_index}:{parcel_index}"),
        source_owner_id: source_climate_sha256.to_string(),
        destination_ofe_id: destination.ofe_id.clone(),
        destination_tile_id: destination.tile_id.clone(),
        start_s,
        end_s,
        mass_kg_m2,
        temperature_k,
        enthalpy_j_m2: mass_kg_m2
            * ICE_HEAT_CAPACITY_J_KG_K
            * (temperature_k - MELTING_TEMPERATURE_K),
    }
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

fn validate_solid_precipitation_parcel(
    parcel: &SnowFreeSolidPrecipitationParcelReceipt,
) -> Result<(), SnowFreeHalfHourForcingError> {
    const ICE_HEAT_CAPACITY_J_KG_K: f64 = 2_100.0;
    const MELTING_TEMPERATURE_K: f64 = 273.15;
    let expected_enthalpy = parcel.mass_kg_m2
        * ICE_HEAT_CAPACITY_J_KG_K
        * (parcel.temperature_k - MELTING_TEMPERATURE_K);
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
        || parcel.temperature_k > MELTING_TEMPERATURE_K
        || !parcel.enthalpy_j_m2.is_finite()
        || parcel.enthalpy_j_m2.to_bits() != expected_enthalpy.to_bits()
    {
        return Err(SnowFreeHalfHourForcingError::Identity(
            "solid precipitation parcel",
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

#[cfg(test)]
mod snow_free_half_hour_phase_tests {
    use super::*;

    #[test]
    fn dry_phase_metadata_and_mixed_geometric_closure_poisons_fail_closed() {
        validate_half_hour_precipitation_phase(HalfHourPrecipitationPhase::default())
            .expect("canonical dry phase");

        let dry_metadata_poison = HalfHourPrecipitationPhase {
            hydrometeor_temperature_c: Some(-1.0),
            ..HalfHourPrecipitationPhase::default()
        };
        assert_eq!(
            validate_half_hour_precipitation_phase(dry_metadata_poison),
            Err(SnowFreeHalfHourForcingError::Closure(
                "interval precipitation phase"
            ))
        );

        let mixed = HalfHourPrecipitationPhase {
            active_precipitation_m: 0.01,
            rain_m: 0.006,
            snowfall_m: 0.04,
            rain_fraction: 0.6,
            snow_fraction: 0.4,
            hydrometeor_temperature_c: Some(-0.5),
        };
        validate_half_hour_precipitation_phase(mixed).expect("canonical mixed phase");

        let mut geometric_poison = mixed;
        geometric_poison.snowfall_m = 0.004;
        assert_eq!(
            validate_half_hour_precipitation_phase(geometric_poison),
            Err(SnowFreeHalfHourForcingError::Closure(
                "interval precipitation phase"
            ))
        );

        let mut temperature_provider_poison = mixed;
        temperature_provider_poison.hydrometeor_temperature_c = None;
        assert_eq!(
            validate_half_hour_precipitation_phase(temperature_provider_poison),
            Err(SnowFreeHalfHourForcingError::Closure(
                "interval precipitation phase"
            ))
        );
    }

    #[test]
    fn solid_parcel_enthalpy_poison_fails_closed() {
        let canonical = solid_precipitation_parcel(
            0,
            0,
            &SnowFreeHalfHourDestination {
                ofe_id: "ofe-1".to_owned(),
                tile_id: "tile-1".to_owned(),
                wb14_configuration_sha256: "0".repeat(64),
            },
            &"1".repeat(64),
            0.0,
            SNOW_FREE_INTERVAL_S,
            1.0,
            -1.0,
        );
        validate_solid_precipitation_parcel(&canonical).expect("canonical solid parcel");

        let mut poisoned = canonical;
        poisoned.enthalpy_j_m2 += 1.0;
        assert_eq!(
            validate_solid_precipitation_parcel(&poisoned),
            Err(SnowFreeHalfHourForcingError::Identity(
                "solid precipitation parcel"
            ))
        );
    }

    #[test]
    fn warm_mixed_phase_splits_solid_temperature_without_changing_liquid_custody() {
        const BULK_HYDROMETEOR_TEMPERATURE_C: f64 = 1.25;
        const ACTIVE_PRECIPITATION_M: f64 = 0.01;
        const RAIN_FRACTION: f64 = 0.6;
        const SNOW_FRACTION: f64 = 0.4;
        const MELTING_TEMPERATURE_K: f64 = 273.15;
        let rain_m = ACTIVE_PRECIPITATION_M * RAIN_FRACTION;
        let snowfall_m = ACTIVE_PRECIPITATION_M * SNOW_FRACTION * 10.0;
        let phase = HalfHourPrecipitationPhase {
            active_precipitation_m: ACTIVE_PRECIPITATION_M,
            rain_m,
            snowfall_m,
            rain_fraction: RAIN_FRACTION,
            snow_fraction: SNOW_FRACTION,
            hydrometeor_temperature_c: Some(BULK_HYDROMETEOR_TEMPERATURE_C),
        };
        validate_half_hour_precipitation_phase(phase).expect("warm mixed phase closure");

        let destination = SnowFreeHalfHourDestination {
            ofe_id: "ofe-warm-mixed".to_owned(),
            tile_id: "tile-warm-mixed".to_owned(),
            wb14_configuration_sha256: "0".repeat(64),
        };
        let source_climate_sha256 = "1".repeat(64);
        let liquid = precipitation_parcel(
            7,
            11,
            &destination,
            &source_climate_sha256,
            19_800.0,
            21_600.0,
            rain_m * 1_000.0,
            BULK_HYDROMETEOR_TEMPERATURE_C,
        );
        let solid = solid_precipitation_parcel(
            7,
            11,
            &destination,
            &source_climate_sha256,
            19_800.0,
            21_600.0,
            snowfall_m * 100.0,
            BULK_HYDROMETEOR_TEMPERATURE_C,
        );

        validate_precipitation_parcel(&liquid).expect("warm liquid parcel");
        validate_solid_precipitation_parcel(&solid).expect("bounded solid parcel");
        assert_eq!(solid.temperature_k.to_bits(), MELTING_TEMPERATURE_K.to_bits());
        assert_eq!(solid.enthalpy_j_m2.to_bits(), 0.0_f64.to_bits());
        assert_eq!(
            liquid.temperature_k.to_bits(),
            celsius_to_kelvin(BULK_HYDROMETEOR_TEMPERATURE_C).to_bits()
        );
        assert_eq!(
            liquid.enthalpy_j_m2.to_bits(),
            (liquid.mass_kg_m2 * liquid_specific_enthalpy_j_kg(liquid.temperature_k)).to_bits()
        );
        assert_eq!(liquid.mass_kg_m2.to_bits(), (rain_m * 1_000.0).to_bits());
        assert_eq!(solid.mass_kg_m2.to_bits(), (snowfall_m * 100.0).to_bits());
        assert_eq!(
            (liquid.mass_kg_m2 + solid.mass_kg_m2).to_bits(),
            (ACTIVE_PRECIPITATION_M * 1_000.0).to_bits()
        );
        assert_eq!(liquid.source_owner_id, source_climate_sha256);
        assert_eq!(solid.source_owner_id, source_climate_sha256);
        assert_eq!(liquid.destination_ofe_id, solid.destination_ofe_id);
        assert_eq!(liquid.destination_tile_id, solid.destination_tile_id);
        assert_eq!(liquid.start_s.to_bits(), solid.start_s.to_bits());
        assert_eq!(liquid.end_s.to_bits(), solid.end_s.to_bits());
        assert_ne!(liquid.parcel_id, solid.parcel_id);

        let mut temperature_poison = solid.clone();
        temperature_poison.temperature_k = MELTING_TEMPERATURE_K + 0.01;
        temperature_poison.enthalpy_j_m2 = temperature_poison.mass_kg_m2
            * 2_100.0
            * (temperature_poison.temperature_k - MELTING_TEMPERATURE_K);
        assert_eq!(
            validate_solid_precipitation_parcel(&temperature_poison),
            Err(SnowFreeHalfHourForcingError::Identity(
                "solid precipitation parcel"
            ))
        );

        let mut source_poison = solid;
        source_poison.source_owner_id.clear();
        assert_eq!(
            validate_solid_precipitation_parcel(&source_poison),
            Err(SnowFreeHalfHourForcingError::Identity(
                "solid precipitation parcel"
            ))
        );
    }
}
