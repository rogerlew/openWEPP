#![allow(clippy::missing_errors_doc)]

use std::collections::BTreeSet;

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

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum WaterProtocolRow {
    Protocol,
    Request(usize),
    Authorization(usize),
    FinalizedUse(usize),
    CondensationCredit(usize),
}

#[derive(Clone, Debug, PartialEq)]
pub struct WaterProtocolViolation {
    pub error: LandSurfaceEnergyError,
    pub row: WaterProtocolRow,
}

fn protocol_violation(
    row: WaterProtocolRow,
    error: LandSurfaceEnergyError,
) -> WaterProtocolViolation {
    WaterProtocolViolation { error, row }
}

impl WaterProtocol {
    pub fn validate(&self) -> Result<(), LandSurfaceEnergyError> {
        self.validate_identity_stage()
            .and_then(|()| self.validate_domain_stage())
            .and_then(|()| self.validate_cardinality_stage())
            .and_then(|()| self.validate_bound_stage())
            .map_err(|violation| violation.error)
    }

    pub fn validate_identity_stage(&self) -> Result<(), WaterProtocolViolation> {
        if self.transaction_id.0 == 0 {
            return Err(protocol_violation(
                WaterProtocolRow::Protocol,
                LandSurfaceEnergyError::water_identity("zero water transaction"),
            ));
        }
        for (index, row) in self.requests.iter().enumerate() {
            row.key
                .validate(self.transaction_id)
                .map_err(|error| protocol_violation(WaterProtocolRow::Request(index), error))?;
        }
        for (index, row) in self.authorizations.iter().enumerate() {
            row.key.validate(self.transaction_id).map_err(|error| {
                protocol_violation(WaterProtocolRow::Authorization(index), error)
            })?;
        }
        for (index, row) in self.finalized_uses.iter().enumerate() {
            row.key.validate(self.transaction_id).map_err(|error| {
                protocol_violation(WaterProtocolRow::FinalizedUse(index), error)
            })?;
        }
        for (index, credit) in self.condensation_credits.iter().enumerate() {
            if credit.transaction_id != self.transaction_id
                || credit.hydrology_owner_id != self.hydrology_owner_id
            {
                return Err(protocol_violation(
                    WaterProtocolRow::CondensationCredit(index),
                    LandSurfaceEnergyError::water_identity("condensation identity mismatch"),
                ));
            }
        }
        Ok(())
    }

    pub fn validate_domain_stage(&self) -> Result<(), WaterProtocolViolation> {
        for (index, row) in self.requests.iter().enumerate() {
            require_finite(row.amount_kg_m2_stand_ground, "water request")
                .map_err(|error| protocol_violation(WaterProtocolRow::Request(index), error))?;
        }
        for (index, row) in self.finalized_uses.iter().enumerate() {
            require_finite(row.amount_kg_m2_stand_ground, "finalized water use").map_err(
                |error| protocol_violation(WaterProtocolRow::FinalizedUse(index), error),
            )?;
        }
        for (index, row) in self.authorizations.iter().enumerate() {
            require_finite(row.amount_kg_m2_stand_ground, "water authorization").map_err(
                |error| protocol_violation(WaterProtocolRow::Authorization(index), error),
            )?;
        }
        for (index, credit) in self.condensation_credits.iter().enumerate() {
            let row = WaterProtocolRow::CondensationCredit(index);
            require_finite(credit.amount_kg_m2_stand_ground, "condensation amount")
                .map_err(|error| protocol_violation(row, error))?;
            require_finite(credit.temperature_k, "condensation temperature")
                .map_err(|error| protocol_violation(row, error))?;
            if !(200.0..=350.0).contains(&credit.temperature_k) {
                return Err(protocol_violation(
                    row,
                    LandSurfaceEnergyError::ConstitutiveDomain("condensation temperature"),
                ));
            }
            require_finite(
                credit.specific_liquid_enthalpy_j_kg,
                "condensation enthalpy",
            )
            .map_err(|error| protocol_violation(row, error))?;
        }
        Ok(())
    }

    pub fn validate_cardinality_stage(&self) -> Result<(), WaterProtocolViolation> {
        let mut requests = BTreeSet::new();
        for (index, row) in self.requests.iter().enumerate() {
            if !requests.insert(row.key.clone()) {
                return Err(protocol_violation(
                    WaterProtocolRow::Request(index),
                    LandSurfaceEnergyError::water_cardinality("duplicate water request"),
                ));
            }
        }
        let mut authorizations = BTreeSet::new();
        for (index, row) in self.authorizations.iter().enumerate() {
            if !requests.contains(&row.key) {
                return Err(protocol_violation(
                    WaterProtocolRow::Authorization(index),
                    LandSurfaceEnergyError::water_cardinality(
                        "authorization without exact request",
                    ),
                ));
            }
            if !authorizations.insert(row.key.clone()) {
                return Err(protocol_violation(
                    WaterProtocolRow::Authorization(index),
                    LandSurfaceEnergyError::water_cardinality("duplicate water authorization"),
                ));
            }
        }
        let mut uses = BTreeSet::new();
        for (index, row) in self.finalized_uses.iter().enumerate() {
            if !authorizations.contains(&row.key) {
                return Err(protocol_violation(
                    WaterProtocolRow::FinalizedUse(index),
                    LandSurfaceEnergyError::water_cardinality(
                        "finalized use without exact authorization",
                    ),
                ));
            }
            if !uses.insert(row.key.clone()) {
                return Err(protocol_violation(
                    WaterProtocolRow::FinalizedUse(index),
                    LandSurfaceEnergyError::water_cardinality("duplicate finalized water use"),
                ));
            }
        }
        if let Some((index, _)) = self
            .requests
            .iter()
            .enumerate()
            .find(|(_, row)| !authorizations.contains(&row.key) || !uses.contains(&row.key))
        {
            return Err(protocol_violation(
                WaterProtocolRow::Request(index),
                LandSurfaceEnergyError::water_cardinality(
                    "incomplete request-authorization-use identity set",
                ),
            ));
        }
        if let Some((index, _)) = self
            .authorizations
            .iter()
            .enumerate()
            .find(|(_, row)| !uses.contains(&row.key))
        {
            return Err(protocol_violation(
                WaterProtocolRow::Authorization(index),
                LandSurfaceEnergyError::water_cardinality(
                    "incomplete request-authorization-use identity set",
                ),
            ));
        }
        let mut credits = BTreeSet::new();
        for (index, credit) in self.condensation_credits.iter().enumerate() {
            let key = (
                credit.ofe_id.clone(),
                credit.tile_id.clone(),
                credit.surface_id.clone(),
            );
            if !credits.insert(key) {
                return Err(protocol_violation(
                    WaterProtocolRow::CondensationCredit(index),
                    LandSurfaceEnergyError::water_cardinality("duplicate condensation credit"),
                ));
            }
        }
        Ok(())
    }

    pub fn validate_bound_stage(&self) -> Result<(), WaterProtocolViolation> {
        for (index, row) in self.requests.iter().enumerate() {
            require_finite_water_nonnegative(row.amount_kg_m2_stand_ground, "water request")
                .map_err(|error| protocol_violation(WaterProtocolRow::Request(index), error))?;
        }
        for (index, row) in self.finalized_uses.iter().enumerate() {
            require_finite_water_nonnegative(row.amount_kg_m2_stand_ground, "finalized water use")
                .map_err(|error| {
                    protocol_violation(WaterProtocolRow::FinalizedUse(index), error)
                })?;
        }
        for (index, row) in self.authorizations.iter().enumerate() {
            require_finite_water_nonnegative(row.amount_kg_m2_stand_ground, "water authorization")
                .map_err(|error| {
                    protocol_violation(WaterProtocolRow::Authorization(index), error)
                })?;
            if self.requests.iter().any(|request| {
                request.key == row.key
                    && row.amount_kg_m2_stand_ground > request.amount_kg_m2_stand_ground
            }) {
                return Err(protocol_violation(
                    WaterProtocolRow::Authorization(index),
                    LandSurfaceEnergyError::water_bound("authorization exceeds request"),
                ));
            }
        }
        for (index, row) in self.finalized_uses.iter().enumerate() {
            if self.authorizations.iter().any(|authorization| {
                authorization.key == row.key
                    && row.amount_kg_m2_stand_ground > authorization.amount_kg_m2_stand_ground
            }) {
                return Err(protocol_violation(
                    WaterProtocolRow::FinalizedUse(index),
                    LandSurfaceEnergyError::water_bound("finalized use exceeds authorization"),
                ));
            }
        }
        for (index, credit) in self.condensation_credits.iter().enumerate() {
            require_finite_water_positive(credit.amount_kg_m2_stand_ground, "condensation amount")
                .map_err(|error| {
                    protocol_violation(WaterProtocolRow::CondensationCredit(index), error)
                })?;
        }
        Ok(())
    }
}
