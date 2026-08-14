#![allow(clippy::missing_errors_doc, clippy::struct_excessive_bools)]
// The domain flags are distinct required fields in the frozen forcing schema.

use std::collections::BTreeSet;

use openwepp_kernel_contract::{ResourceOwnerId, TileId, TransactionId};
use serde::{Deserialize, Serialize};

use crate::{
    LandSurfaceEnergyError, OfeId, ParcelId, Sha256Digest, canonical_digest, require_finite,
    require_finite_nonnegative, require_finite_positive,
};

#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum LiquidParcelKind {
    Precipitation,
    RoutedRunon,
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum LiquidTemperatureProvider {
    HarderPomeroyHourly,
    AcceptedUpstreamOutletParcel,
}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct LiquidParcel {
    pub parcel_kind: LiquidParcelKind,
    pub parcel_id: ParcelId,
    pub source_owner_id: ResourceOwnerId,
    pub source_ofe_id: OfeId,
    pub source_tile_id: TileId,
    pub destination_ofe_id: OfeId,
    pub destination_tile_id: TileId,
    pub amount_kg_m2_destination_tile_ground: f64,
    pub temperature_provider: LiquidTemperatureProvider,
    pub temperature_k: Option<f64>,
    pub specific_liquid_enthalpy_j_kg: Option<f64>,
    pub source_state_sha256: Option<Sha256Digest>,
}

impl LiquidParcel {
    fn validate(&self) -> Result<(), LandSurfaceEnergyError> {
        require_finite_nonnegative(
            self.amount_kg_m2_destination_tile_ground,
            "parcel.amount_kg_m2_destination_tile_ground",
        )?;
        let expected_provider = match self.parcel_kind {
            LiquidParcelKind::Precipitation => LiquidTemperatureProvider::HarderPomeroyHourly,
            LiquidParcelKind::RoutedRunon => {
                LiquidTemperatureProvider::AcceptedUpstreamOutletParcel
            }
        };
        if self.temperature_provider != expected_provider {
            return Err(LandSurfaceEnergyError::Identity {
                field: "parcel.temperature_provider",
                expected: format!("{expected_provider:?}"),
                found: format!("{:?}", self.temperature_provider),
            });
        }
        if self.amount_kg_m2_destination_tile_ground == 0.0 {
            if self.temperature_k.is_some() || self.specific_liquid_enthalpy_j_kg.is_some() {
                return Err(LandSurfaceEnergyError::ConstitutiveDomain(
                    "zero-mass parcel carries thermal operands",
                ));
            }
            return Ok(());
        }
        let temperature = self
            .temperature_k
            .ok_or(LandSurfaceEnergyError::UnsupportedDomain(
                "positive parcel missing temperature",
            ))?;
        let specific_enthalpy =
            self.specific_liquid_enthalpy_j_kg
                .ok_or(LandSurfaceEnergyError::UnsupportedDomain(
                    "positive parcel missing enthalpy",
                ))?;
        if self.source_state_sha256.is_none() {
            return Err(LandSurfaceEnergyError::StateLineage(
                "positive parcel missing source state digest",
            ));
        }
        require_finite_positive(temperature, "parcel.temperature_k")?;
        require_finite(specific_enthalpy, "parcel.specific_liquid_enthalpy_j_kg")?;
        let expected_enthalpy = 4218.0 * (temperature - 273.15);
        if specific_enthalpy.to_bits() != expected_enthalpy.to_bits() {
            return Err(LandSurfaceEnergyError::ConstitutiveDomain(
                "parcel liquid enthalpy identity",
            ));
        }
        Ok(())
    }
}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct LandSurfaceForcing {
    pub forcing_sha256: Sha256Digest,
    pub transaction_id: TransactionId,
    pub interval_s: f64,
    pub air_temperature_k: f64,
    pub air_specific_humidity_kg_kg: f64,
    pub air_pressure_pa: f64,
    pub reference_wind_m_s: f64,
    pub neutral_stability: bool,
    pub snow_present_at_beginning: bool,
    pub snow_present_at_end: bool,
    pub snow_terminal_payload_present: bool,
    pub direct_vis_w_m2: f64,
    pub diffuse_vis_w_m2: f64,
    pub direct_nir_w_m2: f64,
    pub diffuse_nir_w_m2: f64,
    pub atmospheric_downward_longwave_w_m2: f64,
    pub precipitation_parcels: Vec<LiquidParcel>,
    pub runon_parcels: Vec<LiquidParcel>,
}

impl LandSurfaceForcing {
    pub fn validate(
        &self,
        expected_transaction: TransactionId,
    ) -> Result<(), LandSurfaceEnergyError> {
        if self.transaction_id.0 == 0 || self.transaction_id != expected_transaction {
            return Err(LandSurfaceEnergyError::Identity {
                field: "forcing.transaction_id",
                expected: expected_transaction.to_string(),
                found: self.transaction_id.to_string(),
            });
        }
        require_finite_positive(self.interval_s, "forcing.interval_s")?;
        require_finite_positive(self.air_temperature_k, "forcing.air_temperature_k")?;
        require_finite_nonnegative(
            self.air_specific_humidity_kg_kg,
            "forcing.air_specific_humidity_kg_kg",
        )?;
        require_finite_positive(self.air_pressure_pa, "forcing.air_pressure_pa")?;
        require_finite_positive(self.reference_wind_m_s, "forcing.reference_wind_m_s")?;
        for (value, field) in [
            (self.direct_vis_w_m2, "forcing.direct_vis_w_m2"),
            (self.diffuse_vis_w_m2, "forcing.diffuse_vis_w_m2"),
            (self.direct_nir_w_m2, "forcing.direct_nir_w_m2"),
            (self.diffuse_nir_w_m2, "forcing.diffuse_nir_w_m2"),
            (
                self.atmospheric_downward_longwave_w_m2,
                "forcing.atmospheric_downward_longwave_w_m2",
            ),
        ] {
            require_finite_nonnegative(value, field)?;
        }
        if !self.neutral_stability {
            return Err(LandSurfaceEnergyError::UnsupportedDomain(
                "nonneutral stability",
            ));
        }
        if self.snow_present_at_beginning || self.snow_present_at_end {
            return Err(LandSurfaceEnergyError::UnsupportedDomain("snow present"));
        }
        if self.snow_terminal_payload_present {
            return Err(LandSurfaceEnergyError::UnsupportedDomain(
                "snow terminal payload",
            ));
        }
        let mut identities = BTreeSet::new();
        for parcel in self
            .precipitation_parcels
            .iter()
            .chain(self.runon_parcels.iter())
        {
            if !identities.insert(parcel.parcel_id.clone()) {
                return Err(LandSurfaceEnergyError::Topology("duplicate liquid parcel"));
            }
            parcel.validate()?;
        }
        if self
            .precipitation_parcels
            .iter()
            .any(|parcel| parcel.parcel_kind != LiquidParcelKind::Precipitation)
            || self
                .runon_parcels
                .iter()
                .any(|parcel| parcel.parcel_kind != LiquidParcelKind::RoutedRunon)
        {
            return Err(LandSurfaceEnergyError::Topology(
                "parcel in wrong forcing collection",
            ));
        }
        let computed = self.canonical_sha256()?;
        if computed != self.forcing_sha256 {
            return Err(LandSurfaceEnergyError::Identity {
                field: "forcing_sha256",
                expected: computed.to_string(),
                found: self.forcing_sha256.to_string(),
            });
        }
        Ok(())
    }

    pub fn canonical_sha256(&self) -> Result<Sha256Digest, LandSurfaceEnergyError> {
        let mut value = serde_json::to_value(self)
            .map_err(|error| LandSurfaceEnergyError::MalformedSerialization(error.to_string()))?;
        let digest = value.get_mut("forcing_sha256").ok_or(
            LandSurfaceEnergyError::MalformedSerialization(
                "forcing_sha256 absent from serialized forcing".into(),
            ),
        )?;
        *digest = serde_json::Value::String(String::new());
        canonical_digest(&value)
    }
}
