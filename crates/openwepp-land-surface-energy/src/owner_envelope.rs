#![allow(clippy::missing_errors_doc)]

use std::collections::BTreeSet;

use openwepp_kernel_contract::{ResourceOwnerId, SoilLayerId, TileId, TransactionId};
use serde::{Deserialize, Serialize};

use crate::{
    ComponentId, ExactDyadicEnthalpy, LandSurfaceEnergyError, OfeId, Sha256Digest,
    SoilThermalExactCarryError, SourceId, WaterProtocol, WaterSourceType, canonical_digest,
    require_finite, require_finite_nonnegative,
};

pub const SOIL_THERMAL_OWNER_V2_TAG: &str = "OPENWEPP_SOIL_THERMAL_OWNER_V2";
pub const SOIL_THERMAL_OWNER_V2_SCHEMA_CANONICAL: &str = "OPENWEPP_SOIL_THERMAL_OWNER_V2|contract_version=15|owner_tag|schema_sha256|exact_carry_definition_sha256|parent_v1_state_sha256|model_version|model_definition_sha256|run_id|transaction_id|expected_predecessor_transaction_id|support_start_ns|support_end_ns|receipt_chain_sha256|state(owner_id,configuration_sha256,state_sha256,last_accepted_transaction_id,ordered_ofes(ofe_id,ordered_layers(layer_id,temperature_k_bits,enthalpy_hi_bits,enthalpy_carry,last_accepted_transaction_id)))|credit_layer(beginning_temperature_k_bits,ending_temperature_k_bits,heat_capacity_j_m2_k_bits,beginning_HR,ending_HR,ordered_Q)|temperature_projection=round_nearest_even(T_begin+(E_end-E_begin)/C)";
pub const SOIL_THERMAL_OWNER_V2_SCHEMA_SHA256: &str =
    "7877f2a227b0fa98c0c92ae2fb7397744857555fc2f2f77d91a6de327ca88be4";
pub const EXACT_DYADIC_ENTHALPY_V1_DEFINITION_CANONICAL: &str = "OPENWEPP_EXACT_DYADIC_ENTHALPY_V1|value=sign*coefficient*2^exponent2|zero=(0,0,0)|nonzero_sign=-1_or_1|coefficient=lowercase_positive_odd_hex_no_leading_zero|exponent2=i32|binary64=round_nearest_ties_even|overflow=refuse";
pub const EXACT_DYADIC_ENTHALPY_V1_DEFINITION_SHA256: &str =
    "7ceb6e80567a05625b0ac7c33fc8c48ac9a776bab8f9863e02e5a87696714014";

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

/// V2 soil-layer state. The exact owner value is `exact(H_hi) + R`.
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct SoilThermalLayerStateV2 {
    pub layer_id: SoilLayerId,
    pub temperature_k: f64,
    pub enthalpy_hi_j_m2_ofe_ground: f64,
    pub enthalpy_carry: ExactDyadicEnthalpy,
    pub last_accepted_transaction_id: Option<TransactionId>,
}

/// Ordered OFE partition of a V2 soil-thermal owner.
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct SoilThermalOfeStateV2 {
    pub ofe_id: OfeId,
    pub ordered_layers: Vec<SoilThermalLayerStateV2>,
}

/// Persistable V2 owner state, independently sealed from its envelope.
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct SoilThermalOwnedStateV2 {
    pub owner_id: ResourceOwnerId,
    pub configuration_sha256: Sha256Digest,
    pub state_sha256: Sha256Digest,
    pub last_accepted_transaction_id: Option<TransactionId>,
    pub ofes: Vec<SoilThermalOfeStateV2>,
}

/// Immutable V2 beginning snapshot supplied to LSE.
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct SoilThermalSnapshotV2 {
    pub owner_tag: String,
    pub schema_sha256: Sha256Digest,
    pub exact_carry_definition_sha256: Sha256Digest,
    pub parent_v1_state_sha256: Sha256Digest,
    pub snapshot_sha256: Sha256Digest,
    pub state: SoilThermalOwnedStateV2,
}

/// Complete tagged V2 candidate identity and owner state.
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct SoilThermalOwnerEnvelopeV2 {
    pub owner_tag: String,
    pub schema_sha256: Sha256Digest,
    pub exact_carry_definition_sha256: Sha256Digest,
    pub parent_v1_state_sha256: Sha256Digest,
    pub contract_version: u32,
    pub model_version: String,
    pub model_definition_sha256: Sha256Digest,
    pub run_id: String,
    pub transaction_id: TransactionId,
    pub expected_predecessor_transaction_id: Option<TransactionId>,
    pub support_start_ns: u128,
    pub support_end_ns: u128,
    pub receipt_chain_sha256: Sha256Digest,
    pub state: SoilThermalOwnedStateV2,
}

/// Restart seal for the complete V2 owner and receipt-chain identity.
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct SoilThermalOwnerRestartV2 {
    pub owner_tag: String,
    pub schema_sha256: Sha256Digest,
    pub exact_carry_definition_sha256: Sha256Digest,
    pub parent_v1_state_sha256: Sha256Digest,
    pub owner_state_sha256: Sha256Digest,
    pub last_accepted_transaction_id: Option<TransactionId>,
    pub receipt_chain_sha256: Sha256Digest,
    pub restart_sha256: Sha256Digest,
}

/// Checkpoint seal for the complete V2 owner and receipt-chain identity.
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct SoilThermalOwnerCheckpointV2 {
    pub owner_tag: String,
    pub schema_sha256: Sha256Digest,
    pub exact_carry_definition_sha256: Sha256Digest,
    pub parent_v1_state_sha256: Sha256Digest,
    pub owner_state_sha256: Sha256Digest,
    pub last_accepted_transaction_id: Option<TransactionId>,
    pub receipt_chain_sha256: Sha256Digest,
    pub checkpoint_sha256: Sha256Digest,
}

#[derive(Serialize)]
struct SoilThermalStateDigestBody<'a> {
    owner_tag: &'static str,
    schema_sha256: &'static str,
    exact_carry_definition_sha256: &'static str,
    owner_id: &'a ResourceOwnerId,
    configuration_sha256: &'a Sha256Digest,
    last_accepted_transaction_id: Option<TransactionId>,
    ofes: &'a [SoilThermalOfeStateV2],
}

fn exact_carry_serialization(error: &LandSurfaceEnergyError) -> SoilThermalExactCarryError {
    SoilThermalExactCarryError::Serialization(error.to_string())
}

impl SoilThermalOwnedStateV2 {
    pub fn canonical_state_sha256(&self) -> Result<Sha256Digest, SoilThermalExactCarryError> {
        canonical_digest(&SoilThermalStateDigestBody {
            owner_tag: SOIL_THERMAL_OWNER_V2_TAG,
            schema_sha256: SOIL_THERMAL_OWNER_V2_SCHEMA_SHA256,
            exact_carry_definition_sha256: EXACT_DYADIC_ENTHALPY_V1_DEFINITION_SHA256,
            owner_id: &self.owner_id,
            configuration_sha256: &self.configuration_sha256,
            last_accepted_transaction_id: self.last_accepted_transaction_id,
            ofes: &self.ofes,
        })
        .map_err(|error| exact_carry_serialization(&error))
    }

    pub fn reseal(&mut self) -> Result<(), SoilThermalExactCarryError> {
        self.state_sha256 = self.canonical_state_sha256()?;
        Ok(())
    }

    pub fn validate(&self) -> Result<(), SoilThermalExactCarryError> {
        if self.ofes.is_empty() {
            return Err(SoilThermalExactCarryError::Cardinality("empty V2 OFE set"));
        }
        let mut ofes = BTreeSet::new();
        for ofe in &self.ofes {
            if !ofes.insert(ofe.ofe_id.clone()) || ofe.ordered_layers.is_empty() {
                return Err(SoilThermalExactCarryError::Cardinality(
                    "duplicate OFE or empty V2 layer set",
                ));
            }
            let mut layers = BTreeSet::new();
            for layer in &ofe.ordered_layers {
                if !layers.insert(layer.layer_id.clone()) {
                    return Err(SoilThermalExactCarryError::Cardinality(
                        "duplicate V2 soil layer",
                    ));
                }
                if !layer.temperature_k.is_finite()
                    || !(200.0..=350.0).contains(&layer.temperature_k)
                    || !layer.enthalpy_hi_j_m2_ofe_ground.is_finite()
                {
                    return Err(SoilThermalExactCarryError::Domain(
                        "nonfinite or out-of-domain V2 layer state",
                    ));
                }
                layer.enthalpy_carry.validate()?;
                let high_exact = ExactDyadicEnthalpy::from_f64(layer.enthalpy_hi_j_m2_ofe_ground)?;
                let total = ExactDyadicEnthalpy::exact_sum([&high_exact, &layer.enthalpy_carry])?;
                let signed_zero_exception = layer.enthalpy_hi_j_m2_ofe_ground == 0.0
                    && layer.enthalpy_carry == ExactDyadicEnthalpy::zero();
                if !signed_zero_exception {
                    let (canonical_high, canonical_carry) = total.rounded_high_and_remainder()?;
                    if canonical_high.to_bits() != layer.enthalpy_hi_j_m2_ofe_ground.to_bits()
                        || canonical_carry != layer.enthalpy_carry
                    {
                        return Err(SoilThermalExactCarryError::Reconstruction);
                    }
                }
                if layer.last_accepted_transaction_id != self.last_accepted_transaction_id {
                    return Err(SoilThermalExactCarryError::Identity(
                        "mixed V2 layer transaction lineage",
                    ));
                }
            }
        }
        if self.canonical_state_sha256()? != self.state_sha256 {
            return Err(SoilThermalExactCarryError::Identity("V2 state digest"));
        }
        Ok(())
    }

    #[must_use]
    pub fn layer(
        &self,
        ofe_id: &OfeId,
        layer_id: &SoilLayerId,
    ) -> Option<&SoilThermalLayerStateV2> {
        self.ofes
            .iter()
            .find(|ofe| &ofe.ofe_id == ofe_id)
            .and_then(|ofe| {
                ofe.ordered_layers
                    .iter()
                    .find(|layer| &layer.layer_id == layer_id)
            })
    }

    pub(crate) fn layer_mut(
        &mut self,
        ofe_id: &OfeId,
        layer_id: &SoilLayerId,
    ) -> Option<&mut SoilThermalLayerStateV2> {
        self.ofes
            .iter_mut()
            .find(|ofe| &ofe.ofe_id == ofe_id)
            .and_then(|ofe| {
                ofe.ordered_layers
                    .iter_mut()
                    .find(|layer| &layer.layer_id == layer_id)
            })
    }
}

impl SoilThermalOwnerEnvelopeV2 {
    pub fn validate(&self) -> Result<(), SoilThermalExactCarryError> {
        if self.owner_tag != SOIL_THERMAL_OWNER_V2_TAG
            || self.schema_sha256.as_str() != SOIL_THERMAL_OWNER_V2_SCHEMA_SHA256
            || self.exact_carry_definition_sha256.as_str()
                != EXACT_DYADIC_ENTHALPY_V1_DEFINITION_SHA256
            || self.contract_version != 15
            || self.run_id.trim().is_empty()
            || self.transaction_id.0 == 0
            || self.support_start_ns >= self.support_end_ns
            || !matches!(
                self.state.last_accepted_transaction_id,
                value if value == self.expected_predecessor_transaction_id
                    || value == Some(self.transaction_id)
            )
        {
            return Err(SoilThermalExactCarryError::Identity(
                "V2 envelope tag, support, version, or predecessor",
            ));
        }
        self.state.validate()
    }

    pub fn snapshot(&self) -> Result<SoilThermalSnapshotV2, SoilThermalExactCarryError> {
        self.validate()?;
        let snapshot_sha256 =
            canonical_digest(&self.state).map_err(|error| exact_carry_serialization(&error))?;
        Ok(SoilThermalSnapshotV2 {
            owner_tag: self.owner_tag.clone(),
            schema_sha256: self.schema_sha256.clone(),
            exact_carry_definition_sha256: self.exact_carry_definition_sha256.clone(),
            parent_v1_state_sha256: self.parent_v1_state_sha256.clone(),
            snapshot_sha256,
            state: self.state.clone(),
        })
    }
}

/// Identity material required for checked V1-to-V2 migration.
#[derive(Clone, Debug, PartialEq)]
pub struct SoilThermalV2MigrationIdentity {
    pub model_version: String,
    pub model_definition_sha256: Sha256Digest,
    pub run_id: String,
    pub transaction_id: TransactionId,
    pub support_start_ns: u128,
    pub support_end_ns: u128,
    pub receipt_chain_sha256: Sha256Digest,
}

pub fn migrate_soil_thermal_v1_to_v2(
    beginning: &SoilThermalSnapshot,
    identity: SoilThermalV2MigrationIdentity,
) -> Result<SoilThermalOwnerEnvelopeV2, SoilThermalExactCarryError> {
    beginning
        .validate()
        .map_err(|error| exact_carry_serialization(&error))?;
    let mut state = SoilThermalOwnedStateV2 {
        owner_id: beginning.owner_id.clone(),
        configuration_sha256: beginning.configuration_sha256.clone(),
        state_sha256: beginning.state_sha256.clone(),
        last_accepted_transaction_id: beginning.last_accepted_transaction_id,
        ofes: beginning
            .ofes
            .iter()
            .map(|ofe| SoilThermalOfeStateV2 {
                ofe_id: ofe.ofe_id.clone(),
                ordered_layers: ofe
                    .ordered_layers
                    .iter()
                    .map(|layer| SoilThermalLayerStateV2 {
                        layer_id: layer.layer_id.clone(),
                        temperature_k: layer.temperature_k,
                        enthalpy_hi_j_m2_ofe_ground: layer.enthalpy_j_m2_ofe_ground,
                        enthalpy_carry: ExactDyadicEnthalpy::zero(),
                        last_accepted_transaction_id: beginning.last_accepted_transaction_id,
                    })
                    .collect(),
            })
            .collect(),
    };
    state.reseal()?;
    let envelope = SoilThermalOwnerEnvelopeV2 {
        owner_tag: SOIL_THERMAL_OWNER_V2_TAG.to_owned(),
        schema_sha256: Sha256Digest::try_new(SOIL_THERMAL_OWNER_V2_SCHEMA_SHA256)
            .map_err(|error| exact_carry_serialization(&error))?,
        exact_carry_definition_sha256: Sha256Digest::try_new(
            EXACT_DYADIC_ENTHALPY_V1_DEFINITION_SHA256,
        )
        .map_err(|error| exact_carry_serialization(&error))?,
        parent_v1_state_sha256: beginning.state_sha256.clone(),
        contract_version: 15,
        model_version: identity.model_version,
        model_definition_sha256: identity.model_definition_sha256,
        run_id: identity.run_id,
        transaction_id: identity.transaction_id,
        expected_predecessor_transaction_id: beginning.last_accepted_transaction_id,
        support_start_ns: identity.support_start_ns,
        support_end_ns: identity.support_end_ns,
        receipt_chain_sha256: identity.receipt_chain_sha256,
        state,
    };
    envelope.validate()?;
    Ok(envelope)
}

pub fn refuse_soil_thermal_v2_to_v1_downgrade(
    _state: &SoilThermalOwnedStateV2,
) -> Result<SoilThermalSnapshot, SoilThermalExactCarryError> {
    Err(SoilThermalExactCarryError::DowngradeProhibited)
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
