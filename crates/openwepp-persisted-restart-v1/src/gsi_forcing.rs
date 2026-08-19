use crate::{HexF64, Sha256Hex, WireDayIndex, canonical_sha256};
use openwepp_hillslope_orchestrator::runtime_inputs::{
    DirectGsiDailyReceiptV1, DirectGsiDateV1, DirectGsiForcingV1, DirectGsiOwnerConfigurationV1,
    DirectGsiOwnerStateV1, DirectGsiParametersV1, DirectGsiResultV1, SnowFreeHalfHourDayReceipt,
    SnowFreeHalfHourDestination, SnowFreeHalfHourIntervalReceipt, SnowFreeHalfHourProviderCursor,
    SnowFreeHalfHourStaticConfiguration, SnowFreePrecipitationParcelReceipt,
};
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use thiserror::Error;

#[derive(Debug, Error, PartialEq, Eq)]
pub enum GsiForcingRestartError {
    #[error("invalid GSI/forcing identity or digest: {0}")]
    Identity(&'static str),
    #[error("invalid GSI/forcing numeric domain: {0}")]
    Domain(&'static str),
    #[error("invalid GSI/forcing cardinality or ordering: {0}")]
    Ordering(&'static str),
    #[error("platform-width value is not representable")]
    Width,
}
fn f(field: &'static str, value: &HexF64) -> Result<f64, GsiForcingRestartError> {
    let value = value.to_f64();
    value
        .is_finite()
        .then_some(value)
        .ok_or(GsiForcingRestartError::Domain(field))
}
fn sha(value: String) -> Result<Sha256Hex, GsiForcingRestartError> {
    Sha256Hex::try_new(value).map_err(|_| GsiForcingRestartError::Identity("sha256"))
}
fn native_sha(value: &Sha256Hex) -> String {
    value.as_str().to_owned()
}

#[derive(Clone, Debug, Deserialize, PartialEq, Eq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct SnowFreeHalfHourDestinationRestartV1 {
    pub ofe_id: String,
    pub tile_id: String,
    pub wb14_configuration_sha256: Sha256Hex,
}

#[derive(Clone, Debug, Deserialize, PartialEq, Eq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct SnowFreeHalfHourStaticConfigurationRestartV1 {
    pub run_id: String,
    pub co2_pa: HexF64,
    pub reference_height_m: HexF64,
    pub gsi_owner_configuration_sha256: Sha256Hex,
    pub destinations: Vec<SnowFreeHalfHourDestinationRestartV1>,
    pub configuration_sha256: Sha256Hex,
}

impl SnowFreeHalfHourStaticConfigurationRestartV1 {
    pub fn project(
        value: &SnowFreeHalfHourStaticConfiguration,
    ) -> Result<Self, GsiForcingRestartError> {
        value
            .validate()
            .map_err(|_| GsiForcingRestartError::Identity("forcing configuration"))?;
        Ok(Self {
            run_id: value.run_id.clone(),
            co2_pa: HexF64::from_f64(value.co2_pa),
            reference_height_m: HexF64::from_f64(value.reference_height_m),
            gsi_owner_configuration_sha256: sha(value.gsi_owner_configuration_sha256.clone())?,
            destinations: value
                .destinations
                .iter()
                .map(|destination| {
                    Ok(SnowFreeHalfHourDestinationRestartV1 {
                        ofe_id: destination.ofe_id.clone(),
                        tile_id: destination.tile_id.clone(),
                        wb14_configuration_sha256: sha(destination
                            .wb14_configuration_sha256
                            .clone())?,
                    })
                })
                .collect::<Result<_, GsiForcingRestartError>>()?,
            configuration_sha256: sha(value.configuration_sha256())?,
        })
    }

    pub fn restore(&self) -> Result<SnowFreeHalfHourStaticConfiguration, GsiForcingRestartError> {
        if self
            .destinations
            .windows(2)
            .any(|pair| (&pair[0].ofe_id, &pair[0].tile_id) >= (&pair[1].ofe_id, &pair[1].tile_id))
        {
            return Err(GsiForcingRestartError::Ordering(
                "forcing destination canonical order",
            ));
        }
        let value = SnowFreeHalfHourStaticConfiguration {
            run_id: self.run_id.clone(),
            co2_pa: f("forcing.co2_pa", &self.co2_pa)?,
            reference_height_m: f("forcing.reference_height_m", &self.reference_height_m)?,
            gsi_owner_configuration_sha256: native_sha(&self.gsi_owner_configuration_sha256),
            destinations: self
                .destinations
                .iter()
                .map(|destination| SnowFreeHalfHourDestination {
                    ofe_id: destination.ofe_id.clone(),
                    tile_id: destination.tile_id.clone(),
                    wb14_configuration_sha256: native_sha(&destination.wb14_configuration_sha256),
                })
                .collect(),
        };
        value
            .validate()
            .map_err(|_| GsiForcingRestartError::Identity("forcing configuration"))?;
        if value.configuration_sha256() != self.configuration_sha256.as_str() {
            return Err(GsiForcingRestartError::Identity(
                "forcing configuration digest",
            ));
        }
        Ok(value)
    }
}

#[derive(Clone, Debug, Deserialize, PartialEq, Eq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct DirectGsiDateRestartV1 {
    pub year: i32,
    pub ordinal_day: u16,
}
impl DirectGsiDateRestartV1 {
    fn project(v: &DirectGsiDateV1) -> Self {
        Self {
            year: v.year,
            ordinal_day: v.ordinal_day,
        }
    }
    fn restore(&self) -> Result<DirectGsiDateV1, GsiForcingRestartError> {
        if self.ordinal_day == 0 || self.ordinal_day > 366 {
            return Err(GsiForcingRestartError::Domain("gsi.ordinal_day"));
        }
        Ok(DirectGsiDateV1 {
            year: self.year,
            ordinal_day: self.ordinal_day,
        })
    }
}

#[derive(Clone, Debug, Deserialize, PartialEq, Eq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct DirectGsiParametersRestartV1 {
    pub minimum_temperature_inactive_c: HexF64,
    pub minimum_temperature_unconstrained_c: HexF64,
    pub vapor_pressure_deficit_unconstrained_pa: HexF64,
    pub vapor_pressure_deficit_inactive_pa: HexF64,
    pub photoperiod_inactive_hours: HexF64,
    pub photoperiod_unconstrained_hours: HexF64,
}
impl DirectGsiParametersRestartV1 {
    fn project(v: DirectGsiParametersV1) -> Self {
        Self {
            minimum_temperature_inactive_c: HexF64::from_f64(v.minimum_temperature_inactive_c),
            minimum_temperature_unconstrained_c: HexF64::from_f64(
                v.minimum_temperature_unconstrained_c,
            ),
            vapor_pressure_deficit_unconstrained_pa: HexF64::from_f64(
                v.vapor_pressure_deficit_unconstrained_pa,
            ),
            vapor_pressure_deficit_inactive_pa: HexF64::from_f64(
                v.vapor_pressure_deficit_inactive_pa,
            ),
            photoperiod_inactive_hours: HexF64::from_f64(v.photoperiod_inactive_hours),
            photoperiod_unconstrained_hours: HexF64::from_f64(v.photoperiod_unconstrained_hours),
        }
    }
    fn restore(&self) -> Result<DirectGsiParametersV1, GsiForcingRestartError> {
        Ok(DirectGsiParametersV1 {
            minimum_temperature_inactive_c: f(
                "gsi.minimum_temperature_inactive_c",
                &self.minimum_temperature_inactive_c,
            )?,
            minimum_temperature_unconstrained_c: f(
                "gsi.minimum_temperature_unconstrained_c",
                &self.minimum_temperature_unconstrained_c,
            )?,
            vapor_pressure_deficit_unconstrained_pa: f(
                "gsi.vpd_unconstrained_pa",
                &self.vapor_pressure_deficit_unconstrained_pa,
            )?,
            vapor_pressure_deficit_inactive_pa: f(
                "gsi.vpd_inactive_pa",
                &self.vapor_pressure_deficit_inactive_pa,
            )?,
            photoperiod_inactive_hours: f(
                "gsi.photoperiod_inactive_hours",
                &self.photoperiod_inactive_hours,
            )?,
            photoperiod_unconstrained_hours: f(
                "gsi.photoperiod_unconstrained_hours",
                &self.photoperiod_unconstrained_hours,
            )?,
        })
    }
}

#[derive(Clone, Debug, Deserialize, PartialEq, Eq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct DirectGsiOwnerConfigurationRestartV1 {
    pub schema_version: String,
    pub owner_id: String,
    pub parameters: DirectGsiParametersRestartV1,
    pub latitude_degrees: HexF64,
    pub configuration_sha256: Sha256Hex,
}
impl DirectGsiOwnerConfigurationRestartV1 {
    pub fn project(v: &DirectGsiOwnerConfigurationV1) -> Result<Self, GsiForcingRestartError> {
        v.validate()
            .map_err(|_| GsiForcingRestartError::Identity("gsi configuration"))?;
        Ok(Self {
            schema_version: v.schema_version.clone(),
            owner_id: v.owner_id.clone(),
            parameters: DirectGsiParametersRestartV1::project(v.parameters),
            latitude_degrees: HexF64::from_f64(v.latitude_degrees),
            configuration_sha256: sha(v.configuration_sha256.clone())?,
        })
    }
    pub fn restore(&self) -> Result<DirectGsiOwnerConfigurationV1, GsiForcingRestartError> {
        let v = DirectGsiOwnerConfigurationV1 {
            schema_version: self.schema_version.clone(),
            owner_id: self.owner_id.clone(),
            parameters: self.parameters.restore()?,
            latitude_degrees: f("gsi.latitude_degrees", &self.latitude_degrees)?,
            configuration_sha256: native_sha(&self.configuration_sha256),
        };
        v.validate()
            .map_err(|_| GsiForcingRestartError::Identity("gsi configuration"))?;
        Ok(v)
    }
}

#[derive(Clone, Debug, Deserialize, PartialEq, Eq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct DirectGsiOwnerStateRestartV1 {
    pub history_oldest_first: Vec<HexF64>,
    pub last_date: Option<DirectGsiDateRestartV1>,
    pub state_sha256: Sha256Hex,
}
impl DirectGsiOwnerStateRestartV1 {
    pub fn project(v: &DirectGsiOwnerStateV1) -> Result<Self, GsiForcingRestartError> {
        Ok(Self {
            history_oldest_first: v
                .history_oldest_first
                .iter()
                .copied()
                .map(HexF64::from_f64)
                .collect(),
            last_date: v.last_date.as_ref().map(DirectGsiDateRestartV1::project),
            state_sha256: sha(v.state_sha256.clone())?,
        })
    }
    pub fn restore(&self) -> Result<DirectGsiOwnerStateV1, GsiForcingRestartError> {
        if self.history_oldest_first.len() > 21 {
            return Err(GsiForcingRestartError::Ordering("gsi history cardinality"));
        }
        if self.history_oldest_first.is_empty() != self.last_date.is_none() {
            return Err(GsiForcingRestartError::Ordering(
                "gsi history/date equivalence",
            ));
        }
        let v = DirectGsiOwnerStateV1 {
            history_oldest_first: self
                .history_oldest_first
                .iter()
                .map(|x| {
                    let value = f("gsi.history", x)?;
                    if !(0.0..=1.0).contains(&value) {
                        return Err(GsiForcingRestartError::Domain("gsi.history"));
                    }
                    Ok(value)
                })
                .collect::<Result<_, _>>()?,
            last_date: self
                .last_date
                .as_ref()
                .map(DirectGsiDateRestartV1::restore)
                .transpose()?,
            state_sha256: native_sha(&self.state_sha256),
        };
        let mut digest_input = v.clone();
        digest_input.state_sha256.clear();
        let json = serde_json::to_value(&digest_input)
            .map_err(|_| GsiForcingRestartError::Identity("gsi state digest"))?;
        let mut bytes = serde_json::to_vec(&json)
            .map_err(|_| GsiForcingRestartError::Identity("gsi state digest"))?;
        bytes.push(b'\n');
        let computed = format!("{:x}", Sha256::digest(bytes));
        if computed != v.state_sha256 {
            return Err(GsiForcingRestartError::Identity("gsi state digest"));
        }
        openwepp_hillslope_orchestrator::runtime_inputs::restart_authority_restore_gsi_state(&v)
            .map_err(|_| GsiForcingRestartError::Domain("native GSI state"))?;
        Ok(v)
    }

    pub fn seal_wire_digest(&mut self) -> Result<(), GsiForcingRestartError> {
        let value = DirectGsiOwnerStateV1 {
            history_oldest_first: self
                .history_oldest_first
                .iter()
                .map(|value| f("gsi.history", value))
                .collect::<Result<_, _>>()?,
            last_date: self
                .last_date
                .as_ref()
                .map(DirectGsiDateRestartV1::restore)
                .transpose()?,
            state_sha256: String::new(),
        };
        let json = serde_json::to_value(&value)
            .map_err(|_| GsiForcingRestartError::Identity("gsi state digest"))?;
        let mut bytes = serde_json::to_vec(&json)
            .map_err(|_| GsiForcingRestartError::Identity("gsi state digest"))?;
        bytes.push(b'\n');
        self.state_sha256 = sha(format!("{:x}", Sha256::digest(bytes)))?;
        Ok(())
    }
}

#[derive(Clone, Debug, Deserialize, PartialEq, Eq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct DirectGsiForcingRestartV1 {
    pub minimum_temperature_c: HexF64,
    pub vapor_pressure_deficit_pa: HexF64,
    pub latitude_degrees: HexF64,
    pub year: i32,
    pub ordinal_day: u16,
}
impl DirectGsiForcingRestartV1 {
    fn project(v: DirectGsiForcingV1) -> Self {
        Self {
            minimum_temperature_c: HexF64::from_f64(v.minimum_temperature_c),
            vapor_pressure_deficit_pa: HexF64::from_f64(v.vapor_pressure_deficit_pa),
            latitude_degrees: HexF64::from_f64(v.latitude_degrees),
            year: v.year,
            ordinal_day: v.ordinal_day,
        }
    }
    fn restore(&self) -> Result<DirectGsiForcingV1, GsiForcingRestartError> {
        if self.ordinal_day == 0 || self.ordinal_day > 366 {
            return Err(GsiForcingRestartError::Domain("gsi forcing date"));
        }
        Ok(DirectGsiForcingV1 {
            minimum_temperature_c: f("gsi.minimum_temperature_c", &self.minimum_temperature_c)?,
            vapor_pressure_deficit_pa: f("gsi.vpd_pa", &self.vapor_pressure_deficit_pa)?,
            latitude_degrees: f("gsi.latitude", &self.latitude_degrees)?,
            year: self.year,
            ordinal_day: self.ordinal_day,
        })
    }
}

#[derive(Clone, Debug, Deserialize, PartialEq, Eq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct DirectGsiResultRestartV1 {
    pub minimum_temperature_indicator: HexF64,
    pub vapor_pressure_deficit_indicator: HexF64,
    pub photoperiod_indicator: HexF64,
    pub instantaneous_gsi: HexF64,
    pub photoperiod_hours: HexF64,
    pub growing_season_index: HexF64,
    pub sample_count: u32,
}
impl DirectGsiResultRestartV1 {
    fn project(v: DirectGsiResultV1) -> Self {
        Self {
            minimum_temperature_indicator: HexF64::from_f64(v.minimum_temperature_indicator),
            vapor_pressure_deficit_indicator: HexF64::from_f64(v.vapor_pressure_deficit_indicator),
            photoperiod_indicator: HexF64::from_f64(v.photoperiod_indicator),
            instantaneous_gsi: HexF64::from_f64(v.instantaneous_gsi),
            photoperiod_hours: HexF64::from_f64(v.photoperiod_hours),
            growing_season_index: HexF64::from_f64(v.growing_season_index),
            sample_count: v.sample_count,
        }
    }
    fn restore(&self) -> Result<DirectGsiResultV1, GsiForcingRestartError> {
        Ok(DirectGsiResultV1 {
            minimum_temperature_indicator: f(
                "gsi.minimum_temperature_indicator",
                &self.minimum_temperature_indicator,
            )?,
            vapor_pressure_deficit_indicator: f(
                "gsi.vpd_indicator",
                &self.vapor_pressure_deficit_indicator,
            )?,
            photoperiod_indicator: f("gsi.photoperiod_indicator", &self.photoperiod_indicator)?,
            instantaneous_gsi: f("gsi.instantaneous_gsi", &self.instantaneous_gsi)?,
            photoperiod_hours: f("gsi.photoperiod_hours", &self.photoperiod_hours)?,
            growing_season_index: f("gsi.growing_season_index", &self.growing_season_index)?,
            sample_count: self.sample_count,
        })
    }
}

#[derive(Clone, Debug, Deserialize, PartialEq, Eq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct DirectGsiDailyReceiptRestartV1 {
    pub schema_version: String,
    pub owner_id: String,
    pub run_id: String,
    pub day_index: WireDayIndex,
    pub source_climate_sha256: Sha256Hex,
    pub beginning_state: DirectGsiOwnerStateRestartV1,
    pub ending_state: DirectGsiOwnerStateRestartV1,
    pub parameters: DirectGsiParametersRestartV1,
    pub forcing: DirectGsiForcingRestartV1,
    pub result: DirectGsiResultRestartV1,
    pub configuration_sha256: Sha256Hex,
    pub beginning_state_sha256: Sha256Hex,
    pub ending_state_sha256: Sha256Hex,
    pub forcing_sha256: Sha256Hex,
    pub result_sha256: Sha256Hex,
    pub receipt_sha256: Sha256Hex,
}
impl DirectGsiDailyReceiptRestartV1 {
    pub fn project(v: &DirectGsiDailyReceiptV1) -> Result<Self, GsiForcingRestartError> {
        v.validate()
            .map_err(|_| GsiForcingRestartError::Identity("gsi receipt"))?;
        Ok(Self {
            schema_version: v.schema_version.clone(),
            owner_id: v.owner_id.clone(),
            run_id: v.run_id.clone(),
            day_index: WireDayIndex(v.day_index),
            source_climate_sha256: sha(v.source_climate_sha256.clone())?,
            beginning_state: DirectGsiOwnerStateRestartV1::project(&v.beginning_state)?,
            ending_state: DirectGsiOwnerStateRestartV1::project(&v.ending_state)?,
            parameters: DirectGsiParametersRestartV1::project(v.parameters),
            forcing: DirectGsiForcingRestartV1::project(v.forcing),
            result: DirectGsiResultRestartV1::project(v.result),
            configuration_sha256: sha(v.configuration_sha256.clone())?,
            beginning_state_sha256: sha(v.beginning_state_sha256.clone())?,
            ending_state_sha256: sha(v.ending_state_sha256.clone())?,
            forcing_sha256: sha(v.forcing_sha256.clone())?,
            result_sha256: sha(v.result_sha256.clone())?,
            receipt_sha256: sha(v.receipt_sha256.clone())?,
        })
    }
    pub fn restore(&self) -> Result<DirectGsiDailyReceiptV1, GsiForcingRestartError> {
        let v = DirectGsiDailyReceiptV1 {
            schema_version: self.schema_version.clone(),
            owner_id: self.owner_id.clone(),
            run_id: self.run_id.clone(),
            day_index: self.day_index.0,
            source_climate_sha256: native_sha(&self.source_climate_sha256),
            beginning_state: self.beginning_state.restore()?,
            ending_state: self.ending_state.restore()?,
            parameters: self.parameters.restore()?,
            forcing: self.forcing.restore()?,
            result: self.result.restore()?,
            configuration_sha256: native_sha(&self.configuration_sha256),
            beginning_state_sha256: native_sha(&self.beginning_state_sha256),
            ending_state_sha256: native_sha(&self.ending_state_sha256),
            forcing_sha256: native_sha(&self.forcing_sha256),
            result_sha256: native_sha(&self.result_sha256),
            receipt_sha256: native_sha(&self.receipt_sha256),
        };
        v.validate()
            .map_err(|_| GsiForcingRestartError::Identity("gsi receipt"))?;
        Ok(v)
    }
}

#[derive(Clone, Debug, Deserialize, PartialEq, Eq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct SnowFreePrecipitationParcelRestartV1 {
    pub parcel_id: String,
    pub source_owner_id: String,
    pub destination_ofe_id: String,
    pub destination_tile_id: String,
    pub start_s: HexF64,
    pub end_s: HexF64,
    pub mass_kg_m2: HexF64,
    pub temperature_k: HexF64,
    pub enthalpy_j_m2: HexF64,
}
impl SnowFreePrecipitationParcelRestartV1 {
    fn project(v: &SnowFreePrecipitationParcelReceipt) -> Self {
        Self {
            parcel_id: v.parcel_id.clone(),
            source_owner_id: v.source_owner_id.clone(),
            destination_ofe_id: v.destination_ofe_id.clone(),
            destination_tile_id: v.destination_tile_id.clone(),
            start_s: HexF64::from_f64(v.start_s),
            end_s: HexF64::from_f64(v.end_s),
            mass_kg_m2: HexF64::from_f64(v.mass_kg_m2),
            temperature_k: HexF64::from_f64(v.temperature_k),
            enthalpy_j_m2: HexF64::from_f64(v.enthalpy_j_m2),
        }
    }
    pub fn restore(&self) -> Result<SnowFreePrecipitationParcelReceipt, GsiForcingRestartError> {
        Ok(SnowFreePrecipitationParcelReceipt {
            parcel_id: self.parcel_id.clone(),
            source_owner_id: self.source_owner_id.clone(),
            destination_ofe_id: self.destination_ofe_id.clone(),
            destination_tile_id: self.destination_tile_id.clone(),
            start_s: f("parcel.start_s", &self.start_s)?,
            end_s: f("parcel.end_s", &self.end_s)?,
            mass_kg_m2: f("parcel.mass", &self.mass_kg_m2)?,
            temperature_k: f("parcel.temperature", &self.temperature_k)?,
            enthalpy_j_m2: f("parcel.enthalpy", &self.enthalpy_j_m2)?,
        })
    }
}

// The interval and day DTOs deliberately name every runtime field. Their
// restoration is admitted only through the runtime day-receipt validator.
#[derive(Clone, Debug, Deserialize, PartialEq, Eq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct SnowFreeHalfHourIntervalReceiptRestartV1 {
    pub provider_definition_sha256: Sha256Hex,
    pub source_climate_sha256: Sha256Hex,
    pub run_id: String,
    pub day_index: WireDayIndex,
    pub ofe_id: String,
    pub tile_id: String,
    pub interval_index: u8,
    pub transaction_id: String,
    pub start_s: u32,
    pub end_s: u32,
    pub parent_hour_index: u8,
    pub air_temperature_c: HexF64,
    pub dew_point_c: HexF64,
    pub wind_m_s: HexF64,
    pub pressure_kpa: HexF64,
    pub actual_vapor_pressure_kpa: HexF64,
    pub specific_humidity_kg_kg: HexF64,
    pub vpd_kpa: HexF64,
    pub cloud_fraction: HexF64,
    pub solar_zenith_cosine: HexF64,
    pub global_horizontal_shortwave_w_m2: HexF64,
    pub direct_visible_w_m2: HexF64,
    pub diffuse_visible_w_m2: HexF64,
    pub direct_nir_w_m2: HexF64,
    pub diffuse_nir_w_m2: HexF64,
    pub downward_longwave_w_m2: HexF64,
    pub co2_pa: HexF64,
    pub reference_height_m: HexF64,
    pub gsi: HexF64,
    pub gsi_receipt_sha256: Sha256Hex,
    pub wb14_configuration_sha256: Sha256Hex,
    pub precipitation_parcels: Vec<SnowFreePrecipitationParcelRestartV1>,
    pub interval_receipt_sha256: Sha256Hex,
}
macro_rules! pf {
    ($v:ident,$n:ident) => {
        HexF64::from_f64($v.$n)
    };
}
impl SnowFreeHalfHourIntervalReceiptRestartV1 {
    fn project(v: &SnowFreeHalfHourIntervalReceipt) -> Result<Self, GsiForcingRestartError> {
        Ok(Self {
            provider_definition_sha256: sha(v.provider_definition_sha256.clone())?,
            source_climate_sha256: sha(v.source_climate_sha256.clone())?,
            run_id: v.run_id.clone(),
            day_index: WireDayIndex(
                u64::try_from(v.day_index).map_err(|_| GsiForcingRestartError::Width)?,
            ),
            ofe_id: v.ofe_id.clone(),
            tile_id: v.tile_id.clone(),
            interval_index: u8::try_from(v.interval_index)
                .map_err(|_| GsiForcingRestartError::Width)?,
            transaction_id: v.transaction_id.clone(),
            start_s: u32::try_from(v.start_s).map_err(|_| GsiForcingRestartError::Width)?,
            end_s: u32::try_from(v.end_s).map_err(|_| GsiForcingRestartError::Width)?,
            parent_hour_index: u8::try_from(v.parent_hour_index)
                .map_err(|_| GsiForcingRestartError::Width)?,
            air_temperature_c: pf!(v, air_temperature_c),
            dew_point_c: pf!(v, dew_point_c),
            wind_m_s: pf!(v, wind_m_s),
            pressure_kpa: pf!(v, pressure_kpa),
            actual_vapor_pressure_kpa: pf!(v, actual_vapor_pressure_kpa),
            specific_humidity_kg_kg: pf!(v, specific_humidity_kg_kg),
            vpd_kpa: pf!(v, vpd_kpa),
            cloud_fraction: pf!(v, cloud_fraction),
            solar_zenith_cosine: pf!(v, solar_zenith_cosine),
            global_horizontal_shortwave_w_m2: pf!(v, global_horizontal_shortwave_w_m2),
            direct_visible_w_m2: pf!(v, direct_visible_w_m2),
            diffuse_visible_w_m2: pf!(v, diffuse_visible_w_m2),
            direct_nir_w_m2: pf!(v, direct_nir_w_m2),
            diffuse_nir_w_m2: pf!(v, diffuse_nir_w_m2),
            downward_longwave_w_m2: pf!(v, downward_longwave_w_m2),
            co2_pa: pf!(v, co2_pa),
            reference_height_m: pf!(v, reference_height_m),
            gsi: pf!(v, gsi),
            gsi_receipt_sha256: sha(v.gsi_receipt_sha256.clone())?,
            wb14_configuration_sha256: sha(v.wb14_configuration_sha256.clone())?,
            precipitation_parcels: v
                .precipitation_parcels
                .iter()
                .map(SnowFreePrecipitationParcelRestartV1::project)
                .collect(),
            interval_receipt_sha256: sha(v.interval_receipt_sha256.clone())?,
        })
    }
    fn restore(&self) -> Result<SnowFreeHalfHourIntervalReceipt, GsiForcingRestartError> {
        Ok(SnowFreeHalfHourIntervalReceipt {
            provider_definition_sha256: native_sha(&self.provider_definition_sha256),
            source_climate_sha256: native_sha(&self.source_climate_sha256),
            run_id: self.run_id.clone(),
            day_index: usize::try_from(self.day_index.0)
                .map_err(|_| GsiForcingRestartError::Width)?,
            ofe_id: self.ofe_id.clone(),
            tile_id: self.tile_id.clone(),
            interval_index: usize::from(self.interval_index),
            transaction_id: self.transaction_id.clone(),
            start_s: self.start_s as usize,
            end_s: self.end_s as usize,
            parent_hour_index: usize::from(self.parent_hour_index),
            air_temperature_c: f("interval.air_temperature_c", &self.air_temperature_c)?,
            dew_point_c: f("interval.dew_point_c", &self.dew_point_c)?,
            wind_m_s: f("interval.wind_m_s", &self.wind_m_s)?,
            pressure_kpa: f("interval.pressure_kpa", &self.pressure_kpa)?,
            actual_vapor_pressure_kpa: f(
                "interval.actual_vapor_pressure_kpa",
                &self.actual_vapor_pressure_kpa,
            )?,
            specific_humidity_kg_kg: f(
                "interval.specific_humidity",
                &self.specific_humidity_kg_kg,
            )?,
            vpd_kpa: f("interval.vpd_kpa", &self.vpd_kpa)?,
            cloud_fraction: f("interval.cloud_fraction", &self.cloud_fraction)?,
            solar_zenith_cosine: f("interval.solar_zenith", &self.solar_zenith_cosine)?,
            global_horizontal_shortwave_w_m2: f(
                "interval.shortwave",
                &self.global_horizontal_shortwave_w_m2,
            )?,
            direct_visible_w_m2: f("interval.direct_visible", &self.direct_visible_w_m2)?,
            diffuse_visible_w_m2: f("interval.diffuse_visible", &self.diffuse_visible_w_m2)?,
            direct_nir_w_m2: f("interval.direct_nir", &self.direct_nir_w_m2)?,
            diffuse_nir_w_m2: f("interval.diffuse_nir", &self.diffuse_nir_w_m2)?,
            downward_longwave_w_m2: f("interval.longwave", &self.downward_longwave_w_m2)?,
            co2_pa: f("interval.co2", &self.co2_pa)?,
            reference_height_m: f("interval.reference_height", &self.reference_height_m)?,
            gsi: f("interval.gsi", &self.gsi)?,
            gsi_receipt_sha256: native_sha(&self.gsi_receipt_sha256),
            wb14_configuration_sha256: native_sha(&self.wb14_configuration_sha256),
            precipitation_parcels: self
                .precipitation_parcels
                .iter()
                .map(SnowFreePrecipitationParcelRestartV1::restore)
                .collect::<Result<_, _>>()?,
            interval_receipt_sha256: native_sha(&self.interval_receipt_sha256),
        })
    }
}

#[derive(Clone, Debug, Deserialize, PartialEq, Eq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct SnowFreeHalfHourDayReceiptRestartV1 {
    pub provider_version: String,
    pub provider_definition_sha256: Sha256Hex,
    pub source_climate_sha256: Sha256Hex,
    pub run_id: String,
    pub day_index: WireDayIndex,
    pub daily_horizontal_energy_mj_m2: HexF64,
    pub intervals: Vec<SnowFreeHalfHourIntervalReceiptRestartV1>,
    pub next_day_precipitation_carry: Vec<SnowFreePrecipitationParcelRestartV1>,
    pub receipt_sha256: Sha256Hex,
}
impl SnowFreeHalfHourDayReceiptRestartV1 {
    pub fn project(v: &SnowFreeHalfHourDayReceipt) -> Result<Self, GsiForcingRestartError> {
        v.validate()
            .map_err(|_| GsiForcingRestartError::Identity("forcing day receipt"))?;
        Ok(Self {
            provider_version: v.provider_version.clone(),
            provider_definition_sha256: sha(v.provider_definition_sha256.clone())?,
            source_climate_sha256: sha(v.source_climate_sha256.clone())?,
            run_id: v.run_id.clone(),
            day_index: WireDayIndex(
                u64::try_from(v.day_index).map_err(|_| GsiForcingRestartError::Width)?,
            ),
            daily_horizontal_energy_mj_m2: HexF64::from_f64(v.daily_horizontal_energy_mj_m2),
            intervals: v
                .intervals
                .iter()
                .map(SnowFreeHalfHourIntervalReceiptRestartV1::project)
                .collect::<Result<_, _>>()?,
            next_day_precipitation_carry: v
                .next_day_precipitation_carry
                .iter()
                .map(SnowFreePrecipitationParcelRestartV1::project)
                .collect(),
            receipt_sha256: sha(v.receipt_sha256.clone())?,
        })
    }
    pub fn restore(&self) -> Result<SnowFreeHalfHourDayReceipt, GsiForcingRestartError> {
        if self.intervals.len() != 48 {
            return Err(GsiForcingRestartError::Ordering(
                "forcing interval cardinality",
            ));
        }
        let v = SnowFreeHalfHourDayReceipt {
            provider_version: self.provider_version.clone(),
            provider_definition_sha256: native_sha(&self.provider_definition_sha256),
            source_climate_sha256: native_sha(&self.source_climate_sha256),
            run_id: self.run_id.clone(),
            day_index: usize::try_from(self.day_index.0)
                .map_err(|_| GsiForcingRestartError::Width)?,
            daily_horizontal_energy_mj_m2: f("day.energy", &self.daily_horizontal_energy_mj_m2)?,
            intervals: self
                .intervals
                .iter()
                .map(SnowFreeHalfHourIntervalReceiptRestartV1::restore)
                .collect::<Result<_, _>>()?,
            next_day_precipitation_carry: self
                .next_day_precipitation_carry
                .iter()
                .map(SnowFreePrecipitationParcelRestartV1::restore)
                .collect::<Result<_, _>>()?,
            receipt_sha256: native_sha(&self.receipt_sha256),
        };
        v.validate()
            .map_err(|_| GsiForcingRestartError::Identity("forcing day receipt"))?;
        Ok(v)
    }
}

#[derive(Deserialize, Serialize)]
struct NativeCursorSnapshot {
    next_day_index: usize,
    configuration_sha256: Option<String>,
    pending_carry: Vec<SnowFreePrecipitationParcelReceipt>,
}
#[derive(Clone, Debug, Deserialize, PartialEq, Eq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct SnowFreeHalfHourProviderCursorRestartV1 {
    pub next_day_index: WireDayIndex,
    pub static_configuration_sha256: Sha256Hex,
    pub pending_carry: Vec<SnowFreePrecipitationParcelRestartV1>,
    pub cursor_sha256: Sha256Hex,
}
#[derive(Serialize)]
struct CursorDigestInput<'a> {
    next_day_index: &'a WireDayIndex,
    static_configuration_sha256: &'a Sha256Hex,
    pending_carry: &'a [SnowFreePrecipitationParcelRestartV1],
}
impl SnowFreeHalfHourProviderCursorRestartV1 {
    pub fn seal(&mut self) -> Result<(), GsiForcingRestartError> {
        self.cursor_sha256 = sha(canonical_sha256(&CursorDigestInput {
            next_day_index: &self.next_day_index,
            static_configuration_sha256: &self.static_configuration_sha256,
            pending_carry: &self.pending_carry,
        })
        .map_err(|_| GsiForcingRestartError::Identity("cursor digest"))?)?;
        Ok(())
    }
    pub fn project(
        v: &SnowFreeHalfHourProviderCursor,
        configuration: &SnowFreeHalfHourStaticConfiguration,
        expected_next_day_index: usize,
    ) -> Result<Self, GsiForcingRestartError> {
        v.validate_for_configuration(configuration, expected_next_day_index)
            .map_err(|_| GsiForcingRestartError::Identity("provider cursor"))?;
        let snapshot: NativeCursorSnapshot = serde_json::from_slice(
            &v.to_json_bytes()
                .map_err(|_| GsiForcingRestartError::Identity("provider cursor"))?,
        )
        .map_err(|_| GsiForcingRestartError::Identity("provider cursor"))?;
        let configuration_sha256 = configuration.configuration_sha256();
        if snapshot
            .configuration_sha256
            .as_deref()
            .is_some_and(|digest| digest != configuration_sha256)
        {
            return Err(GsiForcingRestartError::Identity("cursor configuration"));
        }
        let mut dto = Self {
            next_day_index: WireDayIndex(
                u64::try_from(snapshot.next_day_index)
                    .map_err(|_| GsiForcingRestartError::Width)?,
            ),
            static_configuration_sha256: sha(configuration_sha256.to_owned())?,
            pending_carry: snapshot
                .pending_carry
                .iter()
                .map(SnowFreePrecipitationParcelRestartV1::project)
                .collect(),
            cursor_sha256: sha("0".repeat(64))?,
        };
        dto.seal()?;
        Ok(dto)
    }
    pub fn restore(
        &self,
        configuration: &SnowFreeHalfHourStaticConfiguration,
        expected_next_day_index: usize,
    ) -> Result<SnowFreeHalfHourProviderCursor, GsiForcingRestartError> {
        let digest = canonical_sha256(&CursorDigestInput {
            next_day_index: &self.next_day_index,
            static_configuration_sha256: &self.static_configuration_sha256,
            pending_carry: &self.pending_carry,
        })
        .map_err(|_| GsiForcingRestartError::Identity("cursor digest"))?;
        if digest != self.cursor_sha256.as_str()
            || configuration.configuration_sha256() != self.static_configuration_sha256.as_str()
        {
            return Err(GsiForcingRestartError::Identity(
                "cursor digest/configuration",
            ));
        }
        let snapshot = NativeCursorSnapshot {
            next_day_index: usize::try_from(self.next_day_index.0)
                .map_err(|_| GsiForcingRestartError::Width)?,
            configuration_sha256: Some(native_sha(&self.static_configuration_sha256)),
            pending_carry: self
                .pending_carry
                .iter()
                .map(SnowFreePrecipitationParcelRestartV1::restore)
                .collect::<Result<_, _>>()?,
        };
        let bytes = serde_json::to_vec(&snapshot)
            .map_err(|_| GsiForcingRestartError::Identity("cursor serialization"))?;
        SnowFreeHalfHourProviderCursor::restore_json(&bytes, configuration, expected_next_day_index)
            .map_err(|_| GsiForcingRestartError::Identity("provider cursor"))
    }
}
