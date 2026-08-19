//! Explicit, layout-independent projections of every persisted scientific owner.

use crate::{HexF64, HexU128, Sha256Hex, canonical_sha256};
use openwepp_biogeochemistry::{
    BiogeochemistryState, MaterialPool, MineralLayer, available_by_key,
};
use openwepp_kernel_contract::{
    MaterialDonorClass, MaterialReceiverClass, OccupancyId, ResourceOwnerId, SoilLayerId,
    StratumId, TileId, TransactionId,
};
use openwepp_land_surface_energy::{
    LandSurfaceEnergyConfiguration, LandSurfaceEnergyState, LandSurfaceEnergyV2State, OfeId,
    Sha256Digest, SoilThermalLayerSnapshot, SoilThermalOfeSnapshot, SoilThermalSnapshot, TileState,
    project_v2_runtime_to_v1,
};
use openwepp_vegetation::carbon_nitrogen::{ElementPool, MaterialTransfer, Tissue, TissuePool};
use openwepp_vegetation::{
    PhenologyPhase, StratumSharedState, V8CoupledOwnedState, V8OccupancyState,
    V8TileCanopyAirState, V10CoupledOwnedState, VegetationConfiguration, project_v10_runtime_to_v9,
};
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;
use thiserror::Error;

#[derive(Debug, Error, PartialEq, Eq)]
pub enum ScientificOwnerRestartError {
    #[error("invalid owner identity: {0}")]
    Identity(&'static str),
    #[error("invalid owner numeric domain: {0}")]
    Domain(&'static str),
    #[error("invalid owner ordering: {0}")]
    Ordering(&'static str),
}

fn finite(field: &'static str, value: &HexF64) -> Result<f64, ScientificOwnerRestartError> {
    let value = value.to_f64();
    value
        .is_finite()
        .then_some(value)
        .ok_or(ScientificOwnerRestartError::Domain(field))
}
fn sha(value: &str) -> Result<Sha256Hex, ScientificOwnerRestartError> {
    Sha256Hex::try_new(value.to_owned())
        .map_err(|_| ScientificOwnerRestartError::Identity("sha256"))
}
fn digest(value: &Sha256Hex) -> Result<Sha256Digest, ScientificOwnerRestartError> {
    Sha256Digest::try_new(value.as_str())
        .map_err(|_| ScientificOwnerRestartError::Identity("sha256"))
}
fn owner(value: &str) -> Result<ResourceOwnerId, ScientificOwnerRestartError> {
    ResourceOwnerId::try_new(value.to_owned())
        .map_err(|_| ScientificOwnerRestartError::Identity("owner"))
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, Ord, PartialEq, PartialOrd, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum TissueRestartV1 {
    Leaf,
    FineRoot,
    LiveStem,
    DeadStem,
    LiveCoarseRoot,
    DeadCoarseRoot,
}
impl From<Tissue> for TissueRestartV1 {
    fn from(v: Tissue) -> Self {
        match v {
            Tissue::Leaf => Self::Leaf,
            Tissue::FineRoot => Self::FineRoot,
            Tissue::LiveStem => Self::LiveStem,
            Tissue::DeadStem => Self::DeadStem,
            Tissue::LiveCoarseRoot => Self::LiveCoarseRoot,
            Tissue::DeadCoarseRoot => Self::DeadCoarseRoot,
        }
    }
}
impl From<TissueRestartV1> for Tissue {
    fn from(v: TissueRestartV1) -> Self {
        match v {
            TissueRestartV1::Leaf => Self::Leaf,
            TissueRestartV1::FineRoot => Self::FineRoot,
            TissueRestartV1::LiveStem => Self::LiveStem,
            TissueRestartV1::DeadStem => Self::DeadStem,
            TissueRestartV1::LiveCoarseRoot => Self::LiveCoarseRoot,
            TissueRestartV1::DeadCoarseRoot => Self::DeadCoarseRoot,
        }
    }
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum PhenologyPhaseRestartV1 {
    Dormant,
    Onset,
    Active,
    Offset,
}
impl From<PhenologyPhase> for PhenologyPhaseRestartV1 {
    fn from(v: PhenologyPhase) -> Self {
        match v {
            PhenologyPhase::Dormant => Self::Dormant,
            PhenologyPhase::Onset => Self::Onset,
            PhenologyPhase::Active => Self::Active,
            PhenologyPhase::Offset => Self::Offset,
        }
    }
}
impl From<PhenologyPhaseRestartV1> for PhenologyPhase {
    fn from(v: PhenologyPhaseRestartV1) -> Self {
        match v {
            PhenologyPhaseRestartV1::Dormant => Self::Dormant,
            PhenologyPhaseRestartV1::Onset => Self::Onset,
            PhenologyPhaseRestartV1::Active => Self::Active,
            PhenologyPhaseRestartV1::Offset => Self::Offset,
        }
    }
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum MaterialDonorRestartV1 {
    Leaf,
    FineRoot,
    LiveStem,
    DeadStem,
    LiveCoarseRoot,
    DeadCoarseRoot,
}
impl From<MaterialDonorClass> for MaterialDonorRestartV1 {
    fn from(v: MaterialDonorClass) -> Self {
        match v {
            MaterialDonorClass::Leaf => Self::Leaf,
            MaterialDonorClass::FineRoot => Self::FineRoot,
            MaterialDonorClass::LiveStem => Self::LiveStem,
            MaterialDonorClass::DeadStem => Self::DeadStem,
            MaterialDonorClass::LiveCoarseRoot => Self::LiveCoarseRoot,
            MaterialDonorClass::DeadCoarseRoot => Self::DeadCoarseRoot,
        }
    }
}
impl From<MaterialDonorRestartV1> for MaterialDonorClass {
    fn from(v: MaterialDonorRestartV1) -> Self {
        match v {
            MaterialDonorRestartV1::Leaf => Self::Leaf,
            MaterialDonorRestartV1::FineRoot => Self::FineRoot,
            MaterialDonorRestartV1::LiveStem => Self::LiveStem,
            MaterialDonorRestartV1::DeadStem => Self::DeadStem,
            MaterialDonorRestartV1::LiveCoarseRoot => Self::LiveCoarseRoot,
            MaterialDonorRestartV1::DeadCoarseRoot => Self::DeadCoarseRoot,
        }
    }
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, Ord, PartialEq, PartialOrd, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum MaterialReceiverRestartV1 {
    Metabolic,
    Cellulose,
    Lignin,
    CoarseWoodyDebris,
}
impl From<MaterialReceiverClass> for MaterialReceiverRestartV1 {
    fn from(v: MaterialReceiverClass) -> Self {
        match v {
            MaterialReceiverClass::Metabolic => Self::Metabolic,
            MaterialReceiverClass::Cellulose => Self::Cellulose,
            MaterialReceiverClass::Lignin => Self::Lignin,
            MaterialReceiverClass::CoarseWoodyDebris => Self::CoarseWoodyDebris,
        }
    }
}
impl From<MaterialReceiverRestartV1> for MaterialReceiverClass {
    fn from(v: MaterialReceiverRestartV1) -> Self {
        match v {
            MaterialReceiverRestartV1::Metabolic => Self::Metabolic,
            MaterialReceiverRestartV1::Cellulose => Self::Cellulose,
            MaterialReceiverRestartV1::Lignin => Self::Lignin,
            MaterialReceiverRestartV1::CoarseWoodyDebris => Self::CoarseWoodyDebris,
        }
    }
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct ElementPoolRestartV1 {
    pub carbon: HexF64,
    pub nitrogen: HexF64,
}
impl ElementPoolRestartV1 {
    fn project(v: ElementPool) -> Self {
        Self {
            carbon: HexF64::from_f64(v.carbon),
            nitrogen: HexF64::from_f64(v.nitrogen),
        }
    }
    fn restore(&self) -> Result<ElementPool, ScientificOwnerRestartError> {
        Ok(ElementPool {
            carbon: finite("element.carbon", &self.carbon)?,
            nitrogen: finite("element.nitrogen", &self.nitrogen)?,
        })
    }
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct TissuePoolRestartV1 {
    pub display: ElementPoolRestartV1,
    pub storage: ElementPoolRestartV1,
    pub transfer: ElementPoolRestartV1,
}
impl TissuePoolRestartV1 {
    fn project(v: TissuePool) -> Self {
        Self {
            display: ElementPoolRestartV1::project(v.display),
            storage: ElementPoolRestartV1::project(v.storage),
            transfer: ElementPoolRestartV1::project(v.transfer),
        }
    }
    fn restore(&self) -> Result<TissuePool, ScientificOwnerRestartError> {
        Ok(TissuePool {
            display: self.display.restore()?,
            storage: self.storage.restore()?,
            transfer: self.transfer.restore()?,
        })
    }
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct TissueEntryRestartV1 {
    pub tissue: TissueRestartV1,
    pub pool: TissuePoolRestartV1,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct MaterialTransferRestartV1 {
    pub transaction_id: HexU128,
    pub owner_id: String,
    pub proposal_id: u64,
    pub donor: MaterialDonorRestartV1,
    pub receiver: MaterialReceiverRestartV1,
    pub carbon: HexF64,
    pub nitrogen: HexF64,
    pub dry_matter: HexF64,
}
impl MaterialTransferRestartV1 {
    fn project(v: &MaterialTransfer) -> Self {
        Self {
            transaction_id: HexU128::from_u128(v.transaction_id),
            owner_id: v.owner_id.as_str().into(),
            proposal_id: v.proposal_id,
            donor: v.donor.into(),
            receiver: v.receiver.into(),
            carbon: HexF64::from_f64(v.carbon),
            nitrogen: HexF64::from_f64(v.nitrogen),
            dry_matter: HexF64::from_f64(v.dry_matter),
        }
    }
    fn restore(&self) -> Result<MaterialTransfer, ScientificOwnerRestartError> {
        Ok(MaterialTransfer {
            transaction_id: self.transaction_id.to_u128(),
            owner_id: owner(&self.owner_id)?,
            proposal_id: self.proposal_id,
            donor: self.donor.into(),
            receiver: self.receiver.into(),
            carbon: finite("transfer.carbon", &self.carbon)?,
            nitrogen: finite("transfer.nitrogen", &self.nitrogen)?,
            dry_matter: finite("transfer.dry_matter", &self.dry_matter)?,
        })
    }
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct StratumStateRestartV1 {
    pub stratum_id: String,
    pub tissues: Vec<TissueEntryRestartV1>,
    pub retranslocation_n: HexF64,
    pub nsc_c: HexF64,
    pub xs_c: HexF64,
    pub standing_dead: ElementPoolRestartV1,
    pub standing_dead_dm: HexF64,
    pub phase: PhenologyPhaseRestartV1,
    pub onset_remaining_s: HexF64,
    pub offset_remaining_s: HexF64,
    pub previous_gsi: HexF64,
    pub pending_transfers: Vec<MaterialTransferRestartV1>,
    pub t10_k: HexF64,
    pub leaf_area: HexF64,
    pub root_area: HexF64,
    pub stem_area: HexF64,
    pub last_transaction_id: HexU128,
}
impl StratumStateRestartV1 {
    fn project(id: &StratumId, v: &StratumSharedState) -> Self {
        Self {
            stratum_id: id.as_str().into(),
            tissues: v
                .tissues
                .iter()
                .map(|(t, p)| TissueEntryRestartV1 {
                    tissue: (*t).into(),
                    pool: TissuePoolRestartV1::project(*p),
                })
                .collect(),
            retranslocation_n: HexF64::from_f64(v.retranslocation_n),
            nsc_c: HexF64::from_f64(v.nsc_c),
            xs_c: HexF64::from_f64(v.xs_c),
            standing_dead: ElementPoolRestartV1::project(v.standing_dead),
            standing_dead_dm: HexF64::from_f64(v.standing_dead_dm),
            phase: v.phase.into(),
            onset_remaining_s: HexF64::from_f64(v.onset_remaining_s),
            offset_remaining_s: HexF64::from_f64(v.offset_remaining_s),
            previous_gsi: HexF64::from_f64(v.previous_gsi),
            pending_transfers: v
                .pending_transfers
                .iter()
                .map(MaterialTransferRestartV1::project)
                .collect(),
            t10_k: HexF64::from_f64(v.t10_k),
            leaf_area: HexF64::from_f64(v.leaf_area),
            root_area: HexF64::from_f64(v.root_area),
            stem_area: HexF64::from_f64(v.stem_area),
            last_transaction_id: HexU128::from_u128(v.last_transaction_id),
        }
    }
    fn restore(&self) -> Result<(StratumId, StratumSharedState), ScientificOwnerRestartError> {
        let id = StratumId::try_new(self.stratum_id.clone())
            .map_err(|_| ScientificOwnerRestartError::Identity("stratum"))?;
        let tissues = self
            .tissues
            .iter()
            .map(|e| Ok((e.tissue.into(), e.pool.restore()?)))
            .collect::<Result<BTreeMap<_, _>, ScientificOwnerRestartError>>()?;
        if tissues.len() != self.tissues.len() {
            return Err(ScientificOwnerRestartError::Ordering("duplicate tissue"));
        }
        Ok((
            id,
            StratumSharedState {
                tissues,
                retranslocation_n: finite("stratum.retranslocation_n", &self.retranslocation_n)?,
                nsc_c: finite("stratum.nsc_c", &self.nsc_c)?,
                xs_c: finite("stratum.xs_c", &self.xs_c)?,
                standing_dead: self.standing_dead.restore()?,
                standing_dead_dm: finite("stratum.standing_dead_dm", &self.standing_dead_dm)?,
                phase: self.phase.into(),
                onset_remaining_s: finite("stratum.onset_remaining_s", &self.onset_remaining_s)?,
                offset_remaining_s: finite("stratum.offset_remaining_s", &self.offset_remaining_s)?,
                previous_gsi: finite("stratum.previous_gsi", &self.previous_gsi)?,
                pending_transfers: self
                    .pending_transfers
                    .iter()
                    .map(MaterialTransferRestartV1::restore)
                    .collect::<Result<_, _>>()?,
                t10_k: finite("stratum.t10_k", &self.t10_k)?,
                leaf_area: finite("stratum.leaf_area", &self.leaf_area)?,
                root_area: finite("stratum.root_area", &self.root_area)?,
                stem_area: finite("stratum.stem_area", &self.stem_area)?,
                last_transaction_id: self.last_transaction_id.to_u128(),
            },
        ))
    }
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct OccupancyStateRestartV1 {
    pub stratum_id: String,
    pub tile_id: String,
    pub beta_hyd: HexF64,
    pub canopy_liquid_kg_h2o_m2_tile_ground: HexF64,
    pub dry_stem_temperature_k: HexF64,
    pub last_accepted_transaction_id: Option<HexU128>,
    pub root_node_potential_mm: HexF64,
    pub shade_ci_pa: HexF64,
    pub shade_leaf_potential_mm: HexF64,
    pub shade_leaf_temperature_k: HexF64,
    pub stem_potential_mm: HexF64,
    pub sun_ci_pa: HexF64,
    pub sun_leaf_potential_mm: HexF64,
    pub sun_leaf_temperature_k: HexF64,
    pub wet_surface_temperature_k: HexF64,
}
impl OccupancyStateRestartV1 {
    fn project(id: &OccupancyId, v: &V8OccupancyState) -> Self {
        Self {
            stratum_id: id.stratum_id.as_str().into(),
            tile_id: id.tile_id.as_str().into(),
            beta_hyd: HexF64::from_f64(v.beta_hyd),
            canopy_liquid_kg_h2o_m2_tile_ground: HexF64::from_f64(
                v.canopy_liquid_kg_h2o_m2_tile_ground,
            ),
            dry_stem_temperature_k: HexF64::from_f64(v.dry_stem_temperature_k),
            last_accepted_transaction_id: v.last_accepted_transaction_id.map(HexU128::from_u128),
            root_node_potential_mm: HexF64::from_f64(v.root_node_potential_mm),
            shade_ci_pa: HexF64::from_f64(v.shade_ci_pa),
            shade_leaf_potential_mm: HexF64::from_f64(v.shade_leaf_potential_mm),
            shade_leaf_temperature_k: HexF64::from_f64(v.shade_leaf_temperature_k),
            stem_potential_mm: HexF64::from_f64(v.stem_potential_mm),
            sun_ci_pa: HexF64::from_f64(v.sun_ci_pa),
            sun_leaf_potential_mm: HexF64::from_f64(v.sun_leaf_potential_mm),
            sun_leaf_temperature_k: HexF64::from_f64(v.sun_leaf_temperature_k),
            wet_surface_temperature_k: HexF64::from_f64(v.wet_surface_temperature_k),
        }
    }
    fn restore(&self) -> Result<(OccupancyId, V8OccupancyState), ScientificOwnerRestartError> {
        Ok((
            OccupancyId {
                stratum_id: StratumId::try_new(self.stratum_id.clone())
                    .map_err(|_| ScientificOwnerRestartError::Identity("occupancy stratum"))?,
                tile_id: TileId::try_new(self.tile_id.clone())
                    .map_err(|_| ScientificOwnerRestartError::Identity("occupancy tile"))?,
            },
            V8OccupancyState {
                beta_hyd: finite("occupancy.beta_hyd", &self.beta_hyd)?,
                canopy_liquid_kg_h2o_m2_tile_ground: finite(
                    "occupancy.canopy_liquid",
                    &self.canopy_liquid_kg_h2o_m2_tile_ground,
                )?,
                dry_stem_temperature_k: finite(
                    "occupancy.dry_stem_temperature",
                    &self.dry_stem_temperature_k,
                )?,
                last_accepted_transaction_id: self
                    .last_accepted_transaction_id
                    .as_ref()
                    .map(HexU128::to_u128),
                root_node_potential_mm: finite(
                    "occupancy.root_node_potential",
                    &self.root_node_potential_mm,
                )?,
                shade_ci_pa: finite("occupancy.shade_ci", &self.shade_ci_pa)?,
                shade_leaf_potential_mm: finite(
                    "occupancy.shade_leaf_potential",
                    &self.shade_leaf_potential_mm,
                )?,
                shade_leaf_temperature_k: finite(
                    "occupancy.shade_leaf_temperature",
                    &self.shade_leaf_temperature_k,
                )?,
                stem_potential_mm: finite("occupancy.stem_potential", &self.stem_potential_mm)?,
                sun_ci_pa: finite("occupancy.sun_ci", &self.sun_ci_pa)?,
                sun_leaf_potential_mm: finite(
                    "occupancy.sun_leaf_potential",
                    &self.sun_leaf_potential_mm,
                )?,
                sun_leaf_temperature_k: finite(
                    "occupancy.sun_leaf_temperature",
                    &self.sun_leaf_temperature_k,
                )?,
                wet_surface_temperature_k: finite(
                    "occupancy.wet_surface_temperature",
                    &self.wet_surface_temperature_k,
                )?,
            },
        ))
    }
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct TileCanopyAirRestartV1 {
    pub tile_id: String,
    pub canopy_air_specific_humidity_kg_kg: HexF64,
    pub canopy_air_temperature_k: HexF64,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct VegetationV10StateRestartV1 {
    pub model_definition_sha256: Sha256Hex,
    pub configuration_sha256: Sha256Hex,
    pub state_sha256: Sha256Hex,
    pub last_transaction_id: HexU128,
    pub strata: Vec<StratumStateRestartV1>,
    pub occupancies: Vec<OccupancyStateRestartV1>,
    pub tile_canopy_air: Vec<TileCanopyAirRestartV1>,
}
impl VegetationV10StateRestartV1 {
    pub fn project(
        value: &V10CoupledOwnedState,
        configuration: &VegetationConfiguration,
    ) -> Result<Self, ScientificOwnerRestartError> {
        value
            .validate(configuration)
            .map_err(|_| ScientificOwnerRestartError::Identity("vegetation V10"))?;
        Ok(Self {
            model_definition_sha256: sha(&value.0.model_definition_sha256)?,
            configuration_sha256: sha(&value.0.configuration_sha256)?,
            state_sha256: sha(&value.0.state_sha256)?,
            last_transaction_id: HexU128::from_u128(value.0.last_transaction_id),
            strata: value
                .0
                .strata
                .iter()
                .map(|(id, state)| StratumStateRestartV1::project(id, state))
                .collect(),
            occupancies: value
                .0
                .occupancies
                .iter()
                .map(|(id, state)| OccupancyStateRestartV1::project(id, state))
                .collect(),
            tile_canopy_air: value
                .0
                .tile_canopy_air
                .iter()
                .map(|(id, state)| TileCanopyAirRestartV1 {
                    tile_id: id.as_str().into(),
                    canopy_air_specific_humidity_kg_kg: HexF64::from_f64(
                        state.canopy_air_specific_humidity_kg_kg,
                    ),
                    canopy_air_temperature_k: HexF64::from_f64(state.canopy_air_temperature_k),
                })
                .collect(),
        })
    }
    pub fn restore(
        &self,
        configuration: &VegetationConfiguration,
    ) -> Result<V10CoupledOwnedState, ScientificOwnerRestartError> {
        if self.strata.windows(2).any(|pair| pair[0].stratum_id >= pair[1].stratum_id)
            || self
                .occupancies
                .windows(2)
                .any(|pair| {
                    (&pair[0].stratum_id, &pair[0].tile_id)
                        >= (&pair[1].stratum_id, &pair[1].tile_id)
                })
            || self
                .tile_canopy_air
                .windows(2)
                .any(|pair| pair[0].tile_id >= pair[1].tile_id)
        {
            return Err(ScientificOwnerRestartError::Ordering(
                "vegetation canonical order",
            ));
        }
        let strata = self
            .strata
            .iter()
            .map(StratumStateRestartV1::restore)
            .collect::<Result<BTreeMap<_, _>, _>>()?;
        let occupancies = self
            .occupancies
            .iter()
            .map(OccupancyStateRestartV1::restore)
            .collect::<Result<BTreeMap<_, _>, _>>()?;
        let tile_canopy_air = self
            .tile_canopy_air
            .iter()
            .map(|entry| {
                Ok((
                    TileId::try_new(entry.tile_id.clone())
                        .map_err(|_| ScientificOwnerRestartError::Identity("canopy tile"))?,
                    V8TileCanopyAirState {
                        canopy_air_specific_humidity_kg_kg: finite(
                            "canopy_air.humidity",
                            &entry.canopy_air_specific_humidity_kg_kg,
                        )?,
                        canopy_air_temperature_k: finite(
                            "canopy_air.temperature",
                            &entry.canopy_air_temperature_k,
                        )?,
                    },
                ))
            })
            .collect::<Result<BTreeMap<_, _>, ScientificOwnerRestartError>>()?;
        if strata.len() != self.strata.len()
            || occupancies.len() != self.occupancies.len()
            || tile_canopy_air.len() != self.tile_canopy_air.len()
        {
            return Err(ScientificOwnerRestartError::Ordering(
                "duplicate vegetation identity",
            ));
        }
        let value = V10CoupledOwnedState(V8CoupledOwnedState {
            configuration_sha256: self.configuration_sha256.as_str().into(),
            last_transaction_id: self.last_transaction_id.to_u128(),
            model_definition_sha256: self.model_definition_sha256.as_str().into(),
            occupancies,
            state_sha256: self.state_sha256.as_str().into(),
            strata,
            tile_canopy_air,
        });
        value
            .validate(configuration)
            .map_err(|_| ScientificOwnerRestartError::Identity("vegetation V10"))?;
        project_v10_runtime_to_v9(configuration, &value).map_err(|_| {
            ScientificOwnerRestartError::Identity("vegetation V10 to V9 projection")
        })?;
        Ok(value)
    }
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct LseV2TileStateRestartV1 {
    pub ofe_id: String,
    pub tile_id: String,
    pub surface_enthalpy_j_m2_tile_ground: HexF64,
    pub surface_temperature_warm_start_k: HexF64,
}
#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct LseV2StateRestartV1 {
    pub model_definition_sha256: Sha256Hex,
    pub configuration_sha256: Sha256Hex,
    pub state_sha256: Sha256Hex,
    pub owner_id: String,
    pub last_accepted_transaction_id: Option<HexU128>,
    pub tiles: Vec<LseV2TileStateRestartV1>,
}
impl LseV2StateRestartV1 {
    pub fn project(
        value: &LandSurfaceEnergyV2State,
        configuration: &LandSurfaceEnergyConfiguration,
    ) -> Result<Self, ScientificOwnerRestartError> {
        value
            .validate(configuration)
            .map_err(|_| ScientificOwnerRestartError::Identity("LSE V2"))?;
        Ok(Self {
            model_definition_sha256: sha(value.0.model_definition_sha256.as_str())?,
            configuration_sha256: sha(value.0.configuration_sha256.as_str())?,
            state_sha256: sha(value.0.state_sha256.as_str())?,
            owner_id: value.0.owner_id.as_str().into(),
            last_accepted_transaction_id: value
                .0
                .last_accepted_transaction_id
                .map(|v| HexU128::from_u128(v.0)),
            tiles: value
                .0
                .tiles
                .iter()
                .map(|v| LseV2TileStateRestartV1 {
                    ofe_id: v.ofe_id.as_str().into(),
                    tile_id: v.tile_id.as_str().into(),
                    surface_enthalpy_j_m2_tile_ground: HexF64::from_f64(
                        v.surface_enthalpy_j_m2_tile_ground,
                    ),
                    surface_temperature_warm_start_k: HexF64::from_f64(
                        v.surface_temperature_warm_start_k,
                    ),
                })
                .collect(),
        })
    }
    pub fn restore(
        &self,
        configuration: &LandSurfaceEnergyConfiguration,
    ) -> Result<LandSurfaceEnergyV2State, ScientificOwnerRestartError> {
        let value = LandSurfaceEnergyV2State(LandSurfaceEnergyState {
            model_definition_sha256: digest(&self.model_definition_sha256)?,
            configuration_sha256: digest(&self.configuration_sha256)?,
            state_sha256: digest(&self.state_sha256)?,
            owner_id: owner(&self.owner_id)?,
            last_accepted_transaction_id: self
                .last_accepted_transaction_id
                .as_ref()
                .map(|v| TransactionId(v.to_u128())),
            tiles: self
                .tiles
                .iter()
                .map(|v| {
                    Ok(TileState {
                        ofe_id: OfeId::try_new(v.ofe_id.clone())
                            .map_err(|_| ScientificOwnerRestartError::Identity("LSE OFE"))?,
                        tile_id: TileId::try_new(v.tile_id.clone())
                            .map_err(|_| ScientificOwnerRestartError::Identity("LSE tile"))?,
                        surface_enthalpy_j_m2_tile_ground: finite(
                            "LSE enthalpy",
                            &v.surface_enthalpy_j_m2_tile_ground,
                        )?,
                        surface_temperature_warm_start_k: finite(
                            "LSE temperature",
                            &v.surface_temperature_warm_start_k,
                        )?,
                    })
                })
                .collect::<Result<_, ScientificOwnerRestartError>>()?,
        });
        value
            .validate(configuration)
            .map_err(|_| ScientificOwnerRestartError::Identity("LSE V2"))?;
        project_v2_runtime_to_v1(
            configuration,
            &value,
            &configuration.vegetation_configuration.configuration_sha256,
        )
        .map_err(|_| ScientificOwnerRestartError::Identity("LSE V2 to V1 projection"))?;
        Ok(value)
    }
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct SoilThermalLayerRestartV1 {
    pub layer_id: String,
    pub temperature_k: HexF64,
    pub enthalpy_j_m2_ofe_ground: HexF64,
}
#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct SoilThermalOfeRestartV1 {
    pub ofe_id: String,
    pub ordered_layers: Vec<SoilThermalLayerRestartV1>,
}
#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct SoilThermalStateRestartV1 {
    pub owner_id: String,
    pub configuration_sha256: Sha256Hex,
    pub state_sha256: Sha256Hex,
    pub snapshot_sha256: Sha256Hex,
    pub last_accepted_transaction_id: Option<HexU128>,
    pub ofes: Vec<SoilThermalOfeRestartV1>,
    pub restart_payload_sha256: Sha256Hex,
}
impl SoilThermalStateRestartV1 {
    pub fn project(v: &SoilThermalSnapshot) -> Result<Self, ScientificOwnerRestartError> {
        v.validate()
            .map_err(|_| ScientificOwnerRestartError::Domain("soil thermal"))?;
        let mut projected = Self {
            owner_id: v.owner_id.as_str().into(),
            configuration_sha256: sha(v.configuration_sha256.as_str())?,
            state_sha256: sha(v.state_sha256.as_str())?,
            snapshot_sha256: sha(v.snapshot_sha256.as_str())?,
            last_accepted_transaction_id: v
                .last_accepted_transaction_id
                .map(|x| HexU128::from_u128(x.0)),
            ofes: v
                .ofes
                .iter()
                .map(|o| SoilThermalOfeRestartV1 {
                    ofe_id: o.ofe_id.as_str().into(),
                    ordered_layers: o
                        .ordered_layers
                        .iter()
                        .map(|l| SoilThermalLayerRestartV1 {
                            layer_id: l.layer_id.as_str().into(),
                            temperature_k: HexF64::from_f64(l.temperature_k),
                            enthalpy_j_m2_ofe_ground: HexF64::from_f64(l.enthalpy_j_m2_ofe_ground),
                        })
                        .collect(),
                })
                .collect(),
            restart_payload_sha256: Sha256Hex::try_new("0".repeat(64)).unwrap(),
        };
        projected.restart_payload_sha256 =
            Sha256Hex::try_new(projected.compute_restart_sha256()?).unwrap();
        Ok(projected)
    }
    pub fn restore(&self) -> Result<SoilThermalSnapshot, ScientificOwnerRestartError> {
        if self.restart_payload_sha256.as_str() != self.compute_restart_sha256()? {
            return Err(ScientificOwnerRestartError::Identity(
                "soil thermal restart digest",
            ));
        }
        let v = SoilThermalSnapshot {
            owner_id: owner(&self.owner_id)?,
            configuration_sha256: digest(&self.configuration_sha256)?,
            state_sha256: digest(&self.state_sha256)?,
            snapshot_sha256: digest(&self.snapshot_sha256)?,
            last_accepted_transaction_id: self
                .last_accepted_transaction_id
                .as_ref()
                .map(|x| TransactionId(x.to_u128())),
            ofes: self
                .ofes
                .iter()
                .map(|o| {
                    Ok(SoilThermalOfeSnapshot {
                        ofe_id: OfeId::try_new(o.ofe_id.clone())
                            .map_err(|_| ScientificOwnerRestartError::Identity("soil OFE"))?,
                        ordered_layers: o
                            .ordered_layers
                            .iter()
                            .map(|l| {
                                Ok(SoilThermalLayerSnapshot {
                                    layer_id: SoilLayerId::try_new(l.layer_id.clone()).map_err(
                                        |_| ScientificOwnerRestartError::Identity("soil layer"),
                                    )?,
                                    temperature_k: finite("soil temperature", &l.temperature_k)?,
                                    enthalpy_j_m2_ofe_ground: finite(
                                        "soil enthalpy",
                                        &l.enthalpy_j_m2_ofe_ground,
                                    )?,
                                })
                            })
                            .collect::<Result<_, ScientificOwnerRestartError>>()?,
                    })
                })
                .collect::<Result<_, ScientificOwnerRestartError>>()?,
        };
        v.validate()
            .map_err(|_| ScientificOwnerRestartError::Domain("soil thermal"))?;
        Ok(v)
    }

    fn compute_restart_sha256(&self) -> Result<String, ScientificOwnerRestartError> {
        canonical_sha256(&(
            &self.owner_id,
            &self.configuration_sha256,
            &self.state_sha256,
            &self.snapshot_sha256,
            &self.last_accepted_transaction_id,
            &self.ofes,
        ))
        .map_err(|_| ScientificOwnerRestartError::Identity("soil thermal restart digest"))
    }
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct MineralLayerRestartV1 {
    pub layer_id: String,
    pub ammonium_n: HexF64,
    pub nitrate_n: HexF64,
}
#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct MaterialPoolRestartV1 {
    pub receiver: MaterialReceiverRestartV1,
    pub carbon: HexF64,
    pub nitrogen: HexF64,
    pub dry_matter: HexF64,
}
#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct BiogeochemistryStateRestartV1 {
    pub owner_id: String,
    pub configuration_sha256: Sha256Hex,
    pub layers: Vec<MineralLayerRestartV1>,
    pub receivers: Vec<MaterialPoolRestartV1>,
    pub last_transaction_id: HexU128,
    pub state_sha256: Sha256Hex,
}
impl BiogeochemistryStateRestartV1 {
    pub fn seal(&mut self) -> Result<(), ScientificOwnerRestartError> {
        self.state_sha256 = Sha256Hex::try_new(self.compute_state_sha256()?)
            .map_err(|_| ScientificOwnerRestartError::Identity("BGC state digest"))?;
        Ok(())
    }
    pub fn project(v: &BiogeochemistryState) -> Result<Self, ScientificOwnerRestartError> {
        validate_bgc(v)?;
        let configuration_sha256 = Sha256Hex::try_new(
            canonical_sha256(&("biogeochemistry", "OPENWEPP_BGC_OWNER_V1")).unwrap(),
        )
        .unwrap();
        let mut projected = Self {
            owner_id: "biogeochemistry".into(),
            configuration_sha256,
            layers: v
                .layers
                .iter()
                .map(|(id, l)| MineralLayerRestartV1 {
                    layer_id: id.clone(),
                    ammonium_n: HexF64::from_f64(l.ammonium_n),
                    nitrate_n: HexF64::from_f64(l.nitrate_n),
                })
                .collect(),
            receivers: v
                .receivers
                .iter()
                .map(|(r, p)| MaterialPoolRestartV1 {
                    receiver: (*r).into(),
                    carbon: HexF64::from_f64(p.carbon),
                    nitrogen: HexF64::from_f64(p.nitrogen),
                    dry_matter: HexF64::from_f64(p.dry_matter),
                })
                .collect(),
            last_transaction_id: HexU128::from_u128(v.last_transaction_id),
            state_sha256: Sha256Hex::try_new("0".repeat(64)).unwrap(),
        };
        projected.seal()?;
        Ok(projected)
    }
    pub fn restore(&self) -> Result<BiogeochemistryState, ScientificOwnerRestartError> {
        if self.owner_id != "biogeochemistry"
            || self.configuration_sha256.as_str()
                != canonical_sha256(&("biogeochemistry", "OPENWEPP_BGC_OWNER_V1")).unwrap()
            || self.state_sha256.as_str() != self.compute_state_sha256()?
        {
            return Err(ScientificOwnerRestartError::Identity("BGC owner/digest"));
        }
        if self
            .layers
            .windows(2)
            .any(|pair| pair[0].layer_id >= pair[1].layer_id)
            || self
                .receivers
                .windows(2)
                .any(|pair| pair[0].receiver >= pair[1].receiver)
        {
            return Err(ScientificOwnerRestartError::Ordering("BGC canonical order"));
        }
        let layers = self
            .layers
            .iter()
            .map(|l| {
                Ok((
                    l.layer_id.clone(),
                    MineralLayer {
                        ammonium_n: finite("BGC ammonium", &l.ammonium_n)?,
                        nitrate_n: finite("BGC nitrate", &l.nitrate_n)?,
                    },
                ))
            })
            .collect::<Result<BTreeMap<_, _>, ScientificOwnerRestartError>>()?;
        let receivers = self
            .receivers
            .iter()
            .map(|p| {
                Ok((
                    p.receiver.into(),
                    MaterialPool {
                        carbon: finite("BGC carbon", &p.carbon)?,
                        nitrogen: finite("BGC nitrogen", &p.nitrogen)?,
                        dry_matter: finite("BGC dry matter", &p.dry_matter)?,
                    },
                ))
            })
            .collect::<Result<BTreeMap<_, _>, ScientificOwnerRestartError>>()?;
        if layers.len() != self.layers.len() || receivers.len() != self.receivers.len() {
            return Err(ScientificOwnerRestartError::Ordering(
                "duplicate BGC identity",
            ));
        }
        let v = BiogeochemistryState {
            layers,
            receivers,
            last_transaction_id: self.last_transaction_id.to_u128(),
        };
        validate_bgc(&v)?;
        Ok(v)
    }

    fn compute_state_sha256(&self) -> Result<String, ScientificOwnerRestartError> {
        canonical_sha256(&(
            &self.owner_id,
            &self.configuration_sha256,
            &self.layers,
            &self.receivers,
            &self.last_transaction_id,
        ))
        .map_err(|_| ScientificOwnerRestartError::Identity("BGC state digest"))
    }
}
fn validate_bgc(v: &BiogeochemistryState) -> Result<(), ScientificOwnerRestartError> {
    available_by_key(v).map_err(|_| ScientificOwnerRestartError::Domain("BGC mineral layer"))?;
    for pool in v.receivers.values() {
        if [pool.carbon, pool.nitrogen, pool.dry_matter]
            .iter()
            .any(|x| !x.is_finite() || *x < 0.0)
        {
            return Err(ScientificOwnerRestartError::Domain("BGC material pool"));
        }
    }
    Ok(())
}
