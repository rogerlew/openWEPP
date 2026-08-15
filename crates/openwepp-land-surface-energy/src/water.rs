#![allow(clippy::missing_errors_doc)]

use std::collections::{BTreeMap, BTreeSet};

use openwepp_kernel_contract::{ResourceOwnerId, SoilLayerId, TileId, TransactionId};
use serde::{Deserialize, Serialize};

use crate::{
    ComponentId, LandSurfaceEnergyError, OfeId, Sha256Digest, SourceId, SurfaceId, require_finite,
    require_finite_water_nonnegative, require_finite_water_positive,
};

#[derive(Clone, Copy, Debug, Deserialize, Eq, Ord, PartialEq, PartialOrd, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum RequestingComponent {
    VegetationRoot,
    GroundSurface,
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, Ord, PartialEq, PartialOrd, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum SurfaceClass {
    BareMineralSoil,
    ForestLitter,
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, Ord, PartialEq, PartialOrd, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum WaterSourceType {
    SurfaceLiquid,
    LitterLiquid,
    SoilLayerLiquid,
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, Ord, PartialEq, PartialOrd, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum StandGroundWaterAmountBasis {
    #[serde(rename = "kg_h2o_m-2_stand_ground_interval")]
    KgH2oM2StandGroundInterval,
}

#[derive(Clone, Debug, Deserialize, Eq, Ord, PartialEq, PartialOrd, Serialize)]
#[serde(deny_unknown_fields)]
pub struct GroundWaterKey {
    pub transaction_id: TransactionId,
    pub requesting_owner_id: ResourceOwnerId,
    pub requesting_component: RequestingComponent,
    pub ofe_id: OfeId,
    pub requesting_tile_id: TileId,
    pub occupancy_id: Option<ComponentId>,
    pub surface_id: Option<SurfaceId>,
    pub surface_class: Option<SurfaceClass>,
    pub source_type: WaterSourceType,
    pub source_id: SourceId,
    pub source_tile_id: Option<TileId>,
    pub soil_layer_id: Option<SoilLayerId>,
    pub amount_basis: StandGroundWaterAmountBasis,
}

impl GroundWaterKey {
    pub fn validate(&self, transaction_id: TransactionId) -> Result<(), LandSurfaceEnergyError> {
        if self.transaction_id.0 == 0 || self.transaction_id != transaction_id {
            return Err(LandSurfaceEnergyError::water_identity(
                "water key transaction mismatch",
            ));
        }
        match self.requesting_component {
            RequestingComponent::VegetationRoot => {
                if self.occupancy_id.is_none()
                    || self.surface_id.is_some()
                    || self.surface_class.is_some()
                    || self.source_type != WaterSourceType::SoilLayerLiquid
                    || self.source_tile_id.is_some()
                    || self.soil_layer_id.is_none()
                {
                    return Err(LandSurfaceEnergyError::water_identity(
                        "invalid vegetation-root water identity",
                    ));
                }
            }
            RequestingComponent::GroundSurface => {
                if self.occupancy_id.is_some()
                    || self.surface_id.is_none()
                    || self.surface_class.is_none()
                {
                    return Err(LandSurfaceEnergyError::water_identity(
                        "invalid ground-surface water identity",
                    ));
                }
            }
        }
        match self.source_type {
            WaterSourceType::SoilLayerLiquid => {
                if self.source_tile_id.is_some() || self.soil_layer_id.is_none() {
                    return Err(LandSurfaceEnergyError::water_identity(
                        "invalid soil-layer source identity",
                    ));
                }
            }
            WaterSourceType::SurfaceLiquid | WaterSourceType::LitterLiquid => {
                if self.source_tile_id.is_none() || self.soil_layer_id.is_some() {
                    return Err(LandSurfaceEnergyError::water_identity(
                        "invalid tile-water source identity",
                    ));
                }
            }
        }
        Ok(())
    }
}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct WaterAmount {
    pub key: GroundWaterKey,
    pub amount_kg_m2_stand_ground: f64,
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum WaterAuthorizationReason {
    FullSupply,
    ProportionalSupply,
    ZeroSupply,
    DrySource,
    FrozenSource,
    InaccessibleSource,
}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct WaterAuthorization {
    pub key: GroundWaterKey,
    pub amount_kg_m2_stand_ground: f64,
    pub reason: WaterAuthorizationReason,
}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct CondensationCredit {
    pub transaction_id: TransactionId,
    pub hydrology_owner_id: ResourceOwnerId,
    pub ofe_id: OfeId,
    pub tile_id: TileId,
    pub surface_id: SurfaceId,
    pub amount_kg_m2_stand_ground: f64,
    pub amount_basis: StandGroundWaterAmountBasis,
    pub temperature_k: f64,
    pub specific_liquid_enthalpy_j_kg: f64,
}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct WaterProtocol {
    pub transaction_id: TransactionId,
    pub hydrology_owner_id: ResourceOwnerId,
    pub beginning_snapshot_sha256: Sha256Digest,
    pub requests: Vec<WaterAmount>,
    pub authorizations: Vec<WaterAuthorization>,
    pub finalized_uses: Vec<WaterAmount>,
    pub condensation_credits: Vec<CondensationCredit>,
}

impl WaterProtocol {
    pub fn validate(&self) -> Result<(), LandSurfaceEnergyError> {
        if self.transaction_id.0 == 0 {
            return Err(LandSurfaceEnergyError::water_identity(
                "zero water transaction",
            ));
        }
        let mut requests = BTreeMap::new();
        for row in &self.requests {
            row.key.validate(self.transaction_id)?;
            require_finite_water_nonnegative(row.amount_kg_m2_stand_ground, "water request")?;
            if requests
                .insert(row.key.clone(), row.amount_kg_m2_stand_ground)
                .is_some()
            {
                return Err(LandSurfaceEnergyError::water_cardinality(
                    "duplicate water request",
                ));
            }
        }
        let mut authorizations = BTreeMap::new();
        for row in &self.authorizations {
            row.key.validate(self.transaction_id)?;
            require_finite_water_nonnegative(row.amount_kg_m2_stand_ground, "water authorization")?;
            let request =
                requests
                    .get(&row.key)
                    .ok_or(LandSurfaceEnergyError::water_cardinality(
                        "authorization without exact request",
                    ))?;
            if row.amount_kg_m2_stand_ground > *request {
                return Err(LandSurfaceEnergyError::water_bound(
                    "authorization exceeds request",
                ));
            }
            if authorizations
                .insert(row.key.clone(), row.amount_kg_m2_stand_ground)
                .is_some()
            {
                return Err(LandSurfaceEnergyError::water_cardinality(
                    "duplicate water authorization",
                ));
            }
        }
        let mut uses = BTreeSet::new();
        for row in &self.finalized_uses {
            row.key.validate(self.transaction_id)?;
            require_finite_water_nonnegative(row.amount_kg_m2_stand_ground, "finalized water use")?;
            let authorization =
                authorizations
                    .get(&row.key)
                    .ok_or(LandSurfaceEnergyError::water_cardinality(
                        "finalized use without exact authorization",
                    ))?;
            if row.amount_kg_m2_stand_ground > *authorization {
                return Err(LandSurfaceEnergyError::water_bound(
                    "finalized use exceeds authorization",
                ));
            }
            if !uses.insert(row.key.clone()) {
                return Err(LandSurfaceEnergyError::water_cardinality(
                    "duplicate finalized water use",
                ));
            }
        }
        if requests.len() != authorizations.len() || requests.len() != uses.len() {
            return Err(LandSurfaceEnergyError::water_cardinality(
                "incomplete request-authorization-use identity set",
            ));
        }
        let mut credits = BTreeSet::new();
        for credit in &self.condensation_credits {
            if credit.transaction_id != self.transaction_id
                || credit.hydrology_owner_id != self.hydrology_owner_id
            {
                return Err(LandSurfaceEnergyError::water_identity(
                    "condensation identity mismatch",
                ));
            }
            require_finite_water_positive(credit.amount_kg_m2_stand_ground, "condensation amount")?;
            require_finite(credit.temperature_k, "condensation temperature")?;
            if !(200.0..=350.0).contains(&credit.temperature_k) {
                return Err(LandSurfaceEnergyError::ConstitutiveDomain(
                    "condensation temperature",
                ));
            }
            require_finite(
                credit.specific_liquid_enthalpy_j_kg,
                "condensation enthalpy",
            )?;
            let key = (
                credit.ofe_id.clone(),
                credit.tile_id.clone(),
                credit.surface_id.clone(),
            );
            if !credits.insert(key) {
                return Err(LandSurfaceEnergyError::water_cardinality(
                    "duplicate condensation credit",
                ));
            }
        }
        Ok(())
    }
}
