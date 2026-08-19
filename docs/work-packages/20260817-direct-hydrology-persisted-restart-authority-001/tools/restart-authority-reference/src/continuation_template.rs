use crate::{HexF64, Sha256Hex};
use openwepp_hillslope_orchestrator::DirectOfeWb14Parameters;
use openwepp_hillslope_orchestrator::v9_real_consumer_shadow::{
    DirectV9ShadowIntervalInput, DirectV10ShadowDayInput,
};
use openwepp_kernel_contract::{ResourceOwnerId, SoilLayerId, TileId, TransactionId};
use openwepp_land_surface_energy::{
    LandSurfaceForcing, LiquidParcel, LiquidParcelKind, LiquidTemperatureProvider, OfeId, ParcelId,
    Sha256Digest,
};
use openwepp_vegetation::transaction::{SnowFreeForcing, SoilLayerForcing};
use serde::{Deserialize, Serialize};
use thiserror::Error;

#[derive(Debug, Error)]
pub enum ContinuationTemplateError {
    #[error("identity: {0}")]
    Identity(&'static str),
    #[error("domain: {0}")]
    Domain(&'static str),
}
fn f(n: &'static str, v: &HexF64) -> Result<f64, ContinuationTemplateError> {
    let x = v.to_f64();
    x.is_finite()
        .then_some(x)
        .ok_or(ContinuationTemplateError::Domain(n))
}
fn id<T, E>(v: Result<T, E>, n: &'static str) -> Result<T, ContinuationTemplateError> {
    v.map_err(|_| ContinuationTemplateError::Identity(n))
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct LiquidParcelTemplateRestartV1 {
    pub parcel_kind: LiquidParcelKind,
    pub parcel_id: String,
    pub source_owner_id: String,
    pub source_ofe_id: String,
    pub source_tile_id: String,
    pub destination_ofe_id: String,
    pub destination_tile_id: String,
    pub start_s: HexF64,
    pub end_s: HexF64,
    pub amount_kg_m2_destination_tile_ground: HexF64,
    pub temperature_provider: LiquidTemperatureProvider,
    pub temperature_k: Option<HexF64>,
    pub specific_liquid_enthalpy_j_kg: Option<HexF64>,
    pub source_state_sha256: Option<Sha256Hex>,
}
impl LiquidParcelTemplateRestartV1 {
    fn project(v: &LiquidParcel) -> Self {
        Self {
            parcel_kind: v.parcel_kind,
            parcel_id: v.parcel_id.as_str().into(),
            source_owner_id: v.source_owner_id.as_str().into(),
            source_ofe_id: v.source_ofe_id.as_str().into(),
            source_tile_id: v.source_tile_id.as_str().into(),
            destination_ofe_id: v.destination_ofe_id.as_str().into(),
            destination_tile_id: v.destination_tile_id.as_str().into(),
            start_s: HexF64::from_f64(v.start_s),
            end_s: HexF64::from_f64(v.end_s),
            amount_kg_m2_destination_tile_ground: HexF64::from_f64(
                v.amount_kg_m2_destination_tile_ground,
            ),
            temperature_provider: v.temperature_provider,
            temperature_k: v.temperature_k.map(HexF64::from_f64),
            specific_liquid_enthalpy_j_kg: v.specific_liquid_enthalpy_j_kg.map(HexF64::from_f64),
            source_state_sha256: v
                .source_state_sha256
                .as_ref()
                .map(|x| Sha256Hex::try_new(x.as_str().to_owned()).unwrap()),
        }
    }
    fn restore(&self) -> Result<LiquidParcel, ContinuationTemplateError> {
        Ok(LiquidParcel {
            parcel_kind: self.parcel_kind,
            parcel_id: id(ParcelId::try_new(self.parcel_id.clone()), "parcel")?,
            source_owner_id: id(
                ResourceOwnerId::try_new(self.source_owner_id.clone()),
                "parcel owner",
            )?,
            source_ofe_id: id(OfeId::try_new(self.source_ofe_id.clone()), "source OFE")?,
            source_tile_id: id(TileId::try_new(self.source_tile_id.clone()), "source tile")?,
            destination_ofe_id: id(
                OfeId::try_new(self.destination_ofe_id.clone()),
                "destination OFE",
            )?,
            destination_tile_id: id(
                TileId::try_new(self.destination_tile_id.clone()),
                "destination tile",
            )?,
            start_s: f("parcel start", &self.start_s)?,
            end_s: f("parcel end", &self.end_s)?,
            amount_kg_m2_destination_tile_ground: f(
                "parcel amount",
                &self.amount_kg_m2_destination_tile_ground,
            )?,
            temperature_provider: self.temperature_provider,
            temperature_k: self
                .temperature_k
                .as_ref()
                .map(|v| f("parcel temperature", v))
                .transpose()?,
            specific_liquid_enthalpy_j_kg: self
                .specific_liquid_enthalpy_j_kg
                .as_ref()
                .map(|v| f("parcel enthalpy", v))
                .transpose()?,
            source_state_sha256: self
                .source_state_sha256
                .as_ref()
                .map(|v| id(Sha256Digest::try_new(v.as_str()), "parcel digest"))
                .transpose()?,
        })
    }
}
#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct SoilLayerTemplateRestartV1 {
    pub layer_id: String,
    pub matric_potential_mm: HexF64,
    pub hydraulic_conductivity_mm_s: HexF64,
    pub root_path_length_mm: HexF64,
    pub gravity_root_mm: HexF64,
    pub accessible: bool,
    pub frozen: bool,
}
#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct Wb14TemplateRestartV1 {
    pub ofe_id: String,
    pub effective_conductivity_m_s: HexF64,
    pub matric_potential_m: HexF64,
    pub infiltration_storage_capacity_m: HexF64,
}
#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct ContinuationIntervalTemplateRestartV1 {
    pub neutral_stability: bool,
    pub co2_pa: HexF64,
    pub reference_height_m: HexF64,
    pub runon_parcels: Vec<LiquidParcelTemplateRestartV1>,
    pub ground_albedo_vis: HexF64,
    pub ground_albedo_nir: HexF64,
    pub longwave_up_w_m2: HexF64,
    pub soil_layers: Vec<SoilLayerTemplateRestartV1>,
    pub wb14_parameters: Vec<Wb14TemplateRestartV1>,
}
#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct DirectV10ContinuationTemplateRestartV1 {
    pub day_index: u64,
    pub intervals: Vec<ContinuationIntervalTemplateRestartV1>,
}
impl DirectV10ContinuationTemplateRestartV1 {
    pub fn project(v: &DirectV10ShadowDayInput) -> Self {
        Self {
            day_index: v.day_index as u64,
            intervals: v
                .intervals
                .iter()
                .map(|i| ContinuationIntervalTemplateRestartV1 {
                    neutral_stability: i.lse_forcing.neutral_stability,
                    co2_pa: HexF64::from_f64(i.vegetation_forcing.co2_pa),
                    reference_height_m: HexF64::from_f64(i.vegetation_forcing.reference_height_m),
                    runon_parcels: i
                        .lse_forcing
                        .runon_parcels
                        .iter()
                        .map(LiquidParcelTemplateRestartV1::project)
                        .collect(),
                    ground_albedo_vis: HexF64::from_f64(i.vegetation_forcing.ground_albedo_vis),
                    ground_albedo_nir: HexF64::from_f64(i.vegetation_forcing.ground_albedo_nir),
                    longwave_up_w_m2: HexF64::from_f64(i.vegetation_forcing.longwave_up_w_m2),
                    soil_layers: i
                        .vegetation_forcing
                        .soil_layers
                        .iter()
                        .map(|l| SoilLayerTemplateRestartV1 {
                            layer_id: l.layer_id.as_str().into(),
                            matric_potential_mm: HexF64::from_f64(l.matric_potential_mm),
                            hydraulic_conductivity_mm_s: HexF64::from_f64(
                                l.hydraulic_conductivity_mm_s,
                            ),
                            root_path_length_mm: HexF64::from_f64(l.root_path_length_mm),
                            gravity_root_mm: HexF64::from_f64(l.gravity_root_mm),
                            accessible: l.accessible,
                            frozen: l.frozen,
                        })
                        .collect(),
                    wb14_parameters: i
                        .wb14_parameters
                        .iter()
                        .map(|w| Wb14TemplateRestartV1 {
                            ofe_id: w.ofe_id.as_str().into(),
                            effective_conductivity_m_s: HexF64::from_f64(
                                w.effective_conductivity_m_s,
                            ),
                            matric_potential_m: HexF64::from_f64(w.matric_potential_m),
                            infiltration_storage_capacity_m: HexF64::from_f64(
                                w.infiltration_storage_capacity_m,
                            ),
                        })
                        .collect(),
                })
                .collect(),
        }
    }
    pub fn restore(
        &self,
        first_transaction: u128,
    ) -> Result<DirectV10ShadowDayInput, ContinuationTemplateError> {
        if self.intervals.len() != 48 {
            return Err(ContinuationTemplateError::Identity("template cardinality"));
        }
        let day = usize::try_from(self.day_index)
            .map_err(|_| ContinuationTemplateError::Identity("template day"))?;
        let intervals = self
            .intervals
            .iter()
            .enumerate()
            .map(|(n, i)| {
                let tx = first_transaction
                    .checked_add(n as u128)
                    .ok_or(ContinuationTemplateError::Identity("transaction overflow"))?;
                let soil = i
                    .soil_layers
                    .iter()
                    .map(|l| {
                        Ok(SoilLayerForcing {
                            layer_id: id(SoilLayerId::try_new(l.layer_id.clone()), "soil layer")?,
                            water_beginning_kg_m2: 0.0,
                            matric_potential_mm: f("matric", &l.matric_potential_mm)?,
                            hydraulic_conductivity_mm_s: f(
                                "conductivity",
                                &l.hydraulic_conductivity_mm_s,
                            )?,
                            root_path_length_mm: f("root path", &l.root_path_length_mm)?,
                            gravity_root_mm: f("gravity", &l.gravity_root_mm)?,
                            temperature_k: 273.15,
                            accessible: l.accessible,
                            frozen: l.frozen,
                        })
                    })
                    .collect::<Result<Vec<_>, ContinuationTemplateError>>()?;
                let vegetation_forcing = SnowFreeForcing {
                    air_temperature_k: 273.15,
                    pressure_pa: 101325.0,
                    co2_pa: f("co2", &i.co2_pa)?,
                    vapor_pressure_deficit_kpa: 0.0,
                    wind_m_s: 0.0,
                    rain_kg_m2: 0.0,
                    direct_par_w_m2: 0.0,
                    diffuse_par_w_m2: 0.0,
                    direct_nir_w_m2: 0.0,
                    diffuse_nir_w_m2: 0.0,
                    solar_zenith_cosine: 0.0,
                    ground_albedo_vis: f("albedo", &i.ground_albedo_vis)?,
                    ground_albedo_nir: f("albedo", &i.ground_albedo_nir)?,
                    longwave_down_w_m2: 0.0,
                    longwave_up_w_m2: f("longwave", &i.longwave_up_w_m2)?,
                    specific_humidity: 0.0,
                    reference_height_m: f("reference height", &i.reference_height_m)?,
                    soil_layers: soil,
                    gsi: 0.0,
                };
                let wb = i
                    .wb14_parameters
                    .iter()
                    .map(|w| {
                        Ok(DirectOfeWb14Parameters {
                            ofe_id: id(OfeId::try_new(w.ofe_id.clone()), "WB14 OFE")?,
                            effective_conductivity_m_s: f("WB14", &w.effective_conductivity_m_s)?,
                            matric_potential_m: f("WB14", &w.matric_potential_m)?,
                            infiltration_storage_capacity_m: f(
                                "WB14",
                                &w.infiltration_storage_capacity_m,
                            )?,
                        })
                    })
                    .collect::<Result<Vec<_>, ContinuationTemplateError>>()?;
                let mut lse = LandSurfaceForcing {
                    forcing_sha256: id(Sha256Digest::try_new("0".repeat(64)), "digest")?,
                    transaction_id: TransactionId(tx),
                    interval_s: 1800.0,
                    air_temperature_k: 273.15,
                    air_specific_humidity_kg_kg: 0.0,
                    air_pressure_pa: 101325.0,
                    reference_wind_m_s: 0.0,
                    neutral_stability: i.neutral_stability,
                    snow_present_at_beginning: false,
                    snow_present_at_end: false,
                    snow_terminal_payload_present: false,
                    direct_vis_w_m2: 0.0,
                    diffuse_vis_w_m2: 0.0,
                    direct_nir_w_m2: 0.0,
                    diffuse_nir_w_m2: 0.0,
                    atmospheric_downward_longwave_w_m2: 0.0,
                    precipitation_parcels: Vec::new(),
                    runon_parcels: i
                        .runon_parcels
                        .iter()
                        .map(LiquidParcelTemplateRestartV1::restore)
                        .collect::<Result<Vec<_>, _>>()?,
                };
                lse.forcing_sha256 = id(lse.canonical_sha256(), "forcing digest")?;
                Ok(DirectV9ShadowIntervalInput {
                    lse_forcing: lse,
                    vegetation_forcing,
                    wb14_parameters: wb,
                })
            })
            .collect::<Result<Vec<_>, ContinuationTemplateError>>()?;
        id(DirectV10ShadowDayInput::try_new(day, intervals), "template")
    }
}
