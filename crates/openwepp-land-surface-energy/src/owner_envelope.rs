#![allow(clippy::missing_errors_doc)]

use std::collections::BTreeSet;

use openwepp_kernel_contract::{ResourceOwnerId, SoilLayerId, TileId, TransactionId};
use serde::{Deserialize, Serialize};

use crate::{
    ComponentId, LandSurfaceEnergyError, OfeId, Sha256Digest, SourceId, WaterProtocol,
    WaterSourceType, require_finite, require_finite_nonnegative,
};

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct OwnerStateRef {
    pub owner_id: ResourceOwnerId,
    pub model_version: String,
    pub model_definition_sha256: Sha256Digest,
    pub configuration_sha256: Sha256Digest,
    pub state_sha256: Sha256Digest,
    pub last_accepted_transaction_id: Option<TransactionId>,
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum SnapshotAvailability {
    ImmutableBeginningBeforeCurrentIntervalIngress,
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum HydrologyStoreAmountBasis {
    #[serde(rename = "kg_h2o_m-2_tile_ground")]
    KgH2oM2TileGround,
    #[serde(rename = "kg_h2o_m-2_stand_ground")]
    KgH2oM2StandGround,
}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct HydrologyStoreSnapshot {
    pub ofe_id: OfeId,
    pub tile_id: Option<TileId>,
    pub source_type: WaterSourceType,
    pub source_id: SourceId,
    pub soil_layer_id: Option<SoilLayerId>,
    pub amount_basis: HydrologyStoreAmountBasis,
    pub liquid_amount_kg_m2: f64,
    pub ice_amount_kg_m2: f64,
    pub frozen: bool,
    pub thawing: bool,
}

impl HydrologyStoreSnapshot {
    fn validate(&self) -> Result<(), LandSurfaceEnergyError> {
        require_finite_nonnegative(self.liquid_amount_kg_m2, "hydrology liquid amount")?;
        require_finite_nonnegative(self.ice_amount_kg_m2, "hydrology ice amount")?;
        if self.frozen || self.thawing {
            return Err(LandSurfaceEnergyError::UnsupportedDomain(
                "frozen or thawing hydrology store",
            ));
        }
        match self.source_type {
            WaterSourceType::SoilLayerLiquid => {
                if self.tile_id.is_some()
                    || self.soil_layer_id.is_none()
                    || self.amount_basis != HydrologyStoreAmountBasis::KgH2oM2StandGround
                {
                    return Err(LandSurfaceEnergyError::OwnerEnvelope(
                        "invalid soil hydrology store identity",
                    ));
                }
            }
            WaterSourceType::SurfaceLiquid | WaterSourceType::LitterLiquid => {
                if self.tile_id.is_none()
                    || self.soil_layer_id.is_some()
                    || self.amount_basis != HydrologyStoreAmountBasis::KgH2oM2TileGround
                    || self.ice_amount_kg_m2 != 0.0
                {
                    return Err(LandSurfaceEnergyError::OwnerEnvelope(
                        "invalid surface hydrology store identity",
                    ));
                }
            }
        }
        Ok(())
    }
}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct HydrologySnapshot {
    pub owner_id: ResourceOwnerId,
    pub configuration_sha256: Sha256Digest,
    pub state_sha256: Sha256Digest,
    pub snapshot_sha256: Sha256Digest,
    pub last_accepted_transaction_id: Option<TransactionId>,
    pub availability_time: SnapshotAvailability,
    pub stores: Vec<HydrologyStoreSnapshot>,
}

impl HydrologySnapshot {
    pub fn validate(&self) -> Result<(), LandSurfaceEnergyError> {
        if self.stores.is_empty() {
            return Err(LandSurfaceEnergyError::OwnerEnvelope(
                "empty hydrology snapshot",
            ));
        }
        let mut identities = BTreeSet::new();
        for store in &self.stores {
            store.validate()?;
            let identity = (
                store.ofe_id.clone(),
                store.tile_id.clone(),
                store.source_type,
                store.source_id.clone(),
                store.soil_layer_id.clone(),
            );
            if !identities.insert(identity) {
                return Err(LandSurfaceEnergyError::OwnerEnvelope(
                    "duplicate hydrology store identity",
                ));
            }
        }
        Ok(())
    }
}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct SoilThermalLayerSnapshot {
    pub layer_id: SoilLayerId,
    pub temperature_k: f64,
    pub enthalpy_j_m2_ofe_ground: f64,
}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct SoilThermalOfeSnapshot {
    pub ofe_id: OfeId,
    pub ordered_layers: Vec<SoilThermalLayerSnapshot>,
}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct SoilThermalSnapshot {
    pub owner_id: ResourceOwnerId,
    pub configuration_sha256: Sha256Digest,
    pub state_sha256: Sha256Digest,
    pub snapshot_sha256: Sha256Digest,
    pub last_accepted_transaction_id: Option<TransactionId>,
    pub ofes: Vec<SoilThermalOfeSnapshot>,
}

impl SoilThermalSnapshot {
    pub fn validate(&self) -> Result<(), LandSurfaceEnergyError> {
        if self.ofes.is_empty() {
            return Err(LandSurfaceEnergyError::OwnerEnvelope(
                "empty soil thermal snapshot",
            ));
        }
        let mut ofes = BTreeSet::new();
        for ofe in &self.ofes {
            if !ofes.insert(ofe.ofe_id.clone()) || ofe.ordered_layers.is_empty() {
                return Err(LandSurfaceEnergyError::OwnerEnvelope(
                    "duplicate OFE or empty soil thermal layers",
                ));
            }
            let mut layers = BTreeSet::new();
            for layer in &ofe.ordered_layers {
                if !layers.insert(layer.layer_id.clone()) {
                    return Err(LandSurfaceEnergyError::OwnerEnvelope(
                        "duplicate soil thermal layer",
                    ));
                }
                require_finite(layer.temperature_k, "soil thermal temperature")?;
                require_finite(layer.enthalpy_j_m2_ofe_ground, "soil thermal enthalpy")?;
                if !(200.0..=350.0).contains(&layer.temperature_k) {
                    return Err(LandSurfaceEnergyError::UnsupportedDomain(
                        "soil thermal temperature outside admitted bounds",
                    ));
                }
            }
        }
        Ok(())
    }
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, Ord, PartialEq, PartialOrd, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum CanopyLiquidReleaseKind {
    Throughfall,
    FirstDrainage,
    SecondDrainage,
    Stemflow,
}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct CanopyLiquidRelease {
    pub transaction_id: TransactionId,
    pub vegetation_owner_id: ResourceOwnerId,
    pub ofe_id: OfeId,
    pub tile_id: TileId,
    pub occupancy_id: ComponentId,
    pub release_kind: CanopyLiquidReleaseKind,
    pub amount_kg_m2_tile_ground: f64,
    pub wet_surface_temperature_k: f64,
    pub specific_liquid_enthalpy_j_kg: f64,
}

impl CanopyLiquidRelease {
    pub fn validate(&self, transaction_id: TransactionId) -> Result<(), LandSurfaceEnergyError> {
        if self.transaction_id != transaction_id || transaction_id.0 == 0 {
            return Err(LandSurfaceEnergyError::OwnerEnvelope(
                "canopy liquid release transaction mismatch",
            ));
        }
        if !self.amount_kg_m2_tile_ground.is_finite() || self.amount_kg_m2_tile_ground <= 0.0 {
            return Err(LandSurfaceEnergyError::ConstitutiveDomain(
                "canopy liquid release amount",
            ));
        }
        require_finite(self.wet_surface_temperature_k, "release temperature")?;
        require_finite(
            self.specific_liquid_enthalpy_j_kg,
            "release specific enthalpy",
        )?;
        if !(200.0..=350.0).contains(&self.wet_surface_temperature_k) {
            return Err(LandSurfaceEnergyError::ConstitutiveDomain(
                "release temperature bounds",
            ));
        }
        Ok(())
    }
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, Ord, PartialEq, PartialOrd, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum CandidateOwnerKind {
    Vegetation,
    Hydrology,
    LandSurfaceEnergy,
    Biogeochemistry,
    SoilThermal,
}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct CandidateOwnerReceipt {
    pub transaction_id: TransactionId,
    pub owner_kind: CandidateOwnerKind,
    pub owner_id: ResourceOwnerId,
    pub beginning_state_sha256: Sha256Digest,
    pub candidate_state_sha256: Sha256Digest,
}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct CandidateOwnerBody<S> {
    pub transaction_id: TransactionId,
    pub owner_id: ResourceOwnerId,
    pub model_version: String,
    pub model_definition_sha256: Sha256Digest,
    pub configuration_sha256: Sha256Digest,
    pub beginning_state_sha256: Sha256Digest,
    pub ending_state: S,
    pub water_protocol_sha256: Option<Sha256Digest>,
}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct CandidateOwnerSet<S> {
    pub vegetation: CandidateOwnerBody<S>,
    pub hydrology: CandidateOwnerBody<S>,
    pub land_surface_energy: CandidateOwnerBody<S>,
    pub soil_thermal: CandidateOwnerBody<S>,
    pub biogeochemistry: CandidateOwnerBody<S>,
}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct CandidateReceiptSet {
    pub vegetation: CandidateOwnerReceipt,
    pub hydrology: CandidateOwnerReceipt,
    pub land_surface_energy: CandidateOwnerReceipt,
    pub soil_thermal: CandidateOwnerReceipt,
    pub biogeochemistry: CandidateOwnerReceipt,
}

impl CandidateReceiptSet {
    pub fn validate(&self, transaction_id: TransactionId) -> Result<(), LandSurfaceEnergyError> {
        self.validate_transaction_identities(transaction_id)?;
        self.validate_owner_set()
    }

    fn validate_transaction_identities(
        &self,
        transaction_id: TransactionId,
    ) -> Result<(), LandSurfaceEnergyError> {
        for receipt in self.rows().map(|(receipt, _)| receipt) {
            if receipt.transaction_id != transaction_id {
                return Err(LandSurfaceEnergyError::Identity {
                    field: "candidate_receipt.transaction_id",
                    expected: transaction_id.0.to_string(),
                    found: receipt.transaction_id.0.to_string(),
                });
            }
        }
        Ok(())
    }

    fn validate_owner_set(&self) -> Result<(), LandSurfaceEnergyError> {
        let rows = self.rows();
        let mut owners = BTreeSet::new();
        for (receipt, expected_kind) in rows {
            if receipt.owner_kind != expected_kind || !owners.insert(receipt.owner_id.clone()) {
                return Err(LandSurfaceEnergyError::OwnerEnvelope(
                    "candidate receipt owner-set mismatch",
                ));
            }
        }
        Ok(())
    }

    fn rows(&self) -> impl Iterator<Item = (&CandidateOwnerReceipt, CandidateOwnerKind)> {
        let rows = [
            (&self.vegetation, CandidateOwnerKind::Vegetation),
            (&self.hydrology, CandidateOwnerKind::Hydrology),
            (
                &self.land_surface_energy,
                CandidateOwnerKind::LandSurfaceEnergy,
            ),
            (&self.soil_thermal, CandidateOwnerKind::SoilThermal),
            (&self.biogeochemistry, CandidateOwnerKind::Biogeochemistry),
        ];
        rows.into_iter()
    }
}

/// Identity-only joins that every complete candidate envelope must carry.
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct OwnerEnvelopeIdentity {
    pub transaction_id: TransactionId,
    pub lse_configuration_sha256: Sha256Digest,
    pub water_protocol: WaterProtocol,
    pub candidate_owner_receipts: CandidateReceiptSet,
}

impl OwnerEnvelopeIdentity {
    pub fn validate(&self) -> Result<(), LandSurfaceEnergyError> {
        self.validate_identity_stage()?;
        self.validate_after_identity_stage()
    }

    pub fn validate_identity_stage(&self) -> Result<(), LandSurfaceEnergyError> {
        self.validate_identity_stage_with_expected_configuration(None)
    }

    pub(crate) fn validate_identity_stage_with_expected_configuration(
        &self,
        expected_configuration_sha256: Option<&Sha256Digest>,
    ) -> Result<(), LandSurfaceEnergyError> {
        if self.transaction_id.0 == 0 {
            return Err(LandSurfaceEnergyError::water_identity(
                "zero owner envelope transaction",
            ));
        }
        if self.water_protocol.transaction_id != self.transaction_id {
            return Err(LandSurfaceEnergyError::Identity {
                field: "owner_envelope.transaction_id",
                expected: self.transaction_id.0.to_string(),
                found: self.water_protocol.transaction_id.0.to_string(),
            });
        }
        if let Some(expected) = expected_configuration_sha256
            && &self.lse_configuration_sha256 != expected
        {
            return Err(LandSurfaceEnergyError::Identity {
                field: "owner_envelope.lse_configuration_sha256",
                expected: expected.to_string(),
                found: self.lse_configuration_sha256.to_string(),
            });
        }
        self.water_protocol
            .validate_identity_stage()
            .map_err(|violation| violation.error)?;
        self.candidate_owner_receipts
            .validate_transaction_identities(self.transaction_id)?;
        if self.candidate_owner_receipts.hydrology.owner_id
            != self.water_protocol.hydrology_owner_id
        {
            return Err(LandSurfaceEnergyError::Identity {
                field: "candidate_receipt.hydrology.owner_id",
                expected: self.water_protocol.hydrology_owner_id.as_str().to_owned(),
                found: self
                    .candidate_owner_receipts
                    .hydrology
                    .owner_id
                    .as_str()
                    .to_owned(),
            });
        }
        Ok(())
    }

    pub(crate) fn validate_after_identity_stage(&self) -> Result<(), LandSurfaceEnergyError> {
        self.water_protocol
            .validate_domain_stage()
            .and_then(|()| self.water_protocol.validate_cardinality_stage())
            .and_then(|()| self.water_protocol.validate_bound_stage())
            .map_err(|violation| violation.error)?;
        self.candidate_owner_receipts.validate_owner_set()
    }
}
