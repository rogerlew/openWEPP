#![allow(clippy::missing_errors_doc)]

use std::collections::BTreeSet;

use openwepp_kernel_contract::{ResourceOwnerId, SoilLayerId, TileId, TransactionId};
use serde::{Deserialize, Serialize};

use crate::{
    ComponentId, ExactDyadicEnthalpy, LandSurfaceEnergyError, MINIMUM_SUPPORT_NS, OfeId,
    SOIL_THERMAL_ENERGY_CREDIT_RECEIPT_V2_TAG, Sha256Digest, SoilThermalAcceptedEnergyOperandV2,
    SoilThermalEnergyCreditReceiptV2, SoilThermalExactCarryError, SoilThermalLayerEnergyCreditV2,
    SoilThermalTemperatureProjectionV2, SourceId, WaterProtocol, WaterSourceType, canonical_digest,
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

impl SoilThermalSnapshotV2 {
    pub fn validate(&self) -> Result<(), SoilThermalExactCarryError> {
        if self.owner_tag != SOIL_THERMAL_OWNER_V2_TAG
            || self.schema_sha256.as_str() != SOIL_THERMAL_OWNER_V2_SCHEMA_SHA256
            || self.exact_carry_definition_sha256.as_str()
                != EXACT_DYADIC_ENTHALPY_V1_DEFINITION_SHA256
        {
            return Err(SoilThermalExactCarryError::Identity(
                "V2 snapshot tag, schema, or exact-carry definition",
            ));
        }
        self.state.validate()?;
        let expected =
            canonical_digest(&self.state).map_err(|error| exact_carry_serialization(&error))?;
        if self.snapshot_sha256 != expected {
            return Err(SoilThermalExactCarryError::Identity("V2 snapshot digest"));
        }
        Ok(())
    }
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

/// Native receipt-free seals for one prepared beginning owner.
///
/// These seals are valid only while the current support has no accepted
/// energy receipt: the state lineage equals the expected predecessor and is
/// not the support transaction itself.
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct SoilThermalReceiptFreeOwnerSealsV2 {
    pub restart: SoilThermalOwnerRestartV2,
    pub checkpoint: SoilThermalOwnerCheckpointV2,
    pub receipt_free_seal_sha256: Sha256Digest,
}

/// Borrowed physical read surface. It cannot outlive or replace the native
/// owner envelope and therefore cannot become a second authoritative owner.
#[derive(Clone, Copy)]
pub struct SoilThermalPhysicalReadViewV2<'a> {
    owner: &'a SoilThermalOwnerEnvelopeV2,
}

impl<'a> SoilThermalPhysicalReadViewV2<'a> {
    #[must_use]
    pub const fn owner(&self) -> &'a SoilThermalOwnerEnvelopeV2 {
        self.owner
    }

    #[must_use]
    pub fn layer(
        &self,
        ofe_id: &OfeId,
        layer_id: &SoilLayerId,
    ) -> Option<&'a SoilThermalLayerStateV2> {
        self.owner.state.layer(ofe_id, layer_id)
    }

    pub fn exact_layer_enthalpy(
        &self,
        ofe_id: &OfeId,
        layer_id: &SoilLayerId,
    ) -> Result<ExactDyadicEnthalpy, SoilThermalExactCarryError> {
        let layer = self
            .layer(ofe_id, layer_id)
            .ok_or(SoilThermalExactCarryError::Identity(
                "physical read layer identity",
            ))?;
        Ok(ExactDyadicEnthalpy::exact_sum([
            &ExactDyadicEnthalpy::from_f64(layer.enthalpy_hi_j_m2_ofe_ground)?,
            &layer.enthalpy_carry,
        ])?)
    }
}

/// Complete immutable beginning identity retained by a provisional
/// soil-thermal candidate.
#[derive(Clone, Debug, PartialEq)]
#[allow(
    clippy::large_enum_variant,
    reason = "the provisional candidate must retain the complete authenticated V2 identity inline"
)]
pub enum SoilThermalCandidateBeginningIdentity {
    V1 {
        configuration_sha256: Sha256Digest,
        last_accepted_transaction_id: Option<TransactionId>,
    },
    V2 {
        owner_tag: String,
        schema_sha256: Sha256Digest,
        exact_carry_definition_sha256: Sha256Digest,
        parent_v1_state_sha256: Sha256Digest,
        contract_version: u32,
        model_version: String,
        model_definition_sha256: Sha256Digest,
        run_id: String,
        configuration_sha256: Sha256Digest,
        transaction_id: TransactionId,
        expected_predecessor_transaction_id: Option<TransactionId>,
        support_start_ns: u128,
        support_end_ns: u128,
        receipt_chain_sha256: Sha256Digest,
    },
}

/// Typed read-only beginning accepted by LSE finalization. V2 remains a
/// borrowed view of its authenticated owner rather than a projected V1 cache.
#[derive(Clone, Copy)]
pub enum SoilThermalFinalizationBeginning<'a> {
    V1(&'a SoilThermalSnapshot),
    V2(SoilThermalPhysicalReadViewV2<'a>),
    V2Unpublished(&'a SoilThermalUnpublishedPhysicalBeginningV2),
}

/// Authenticated physical beginning for a child evaluated from an unpublished
/// predecessor trial. This is not an owner envelope and cannot be sealed,
/// accepted, restarted, or published as one.
#[derive(Clone, Debug, PartialEq)]
pub struct SoilThermalUnpublishedPhysicalBeginningV2 {
    authority: PreparedSoilThermalSupportV2,
    predecessor_trial: SoilThermalTrialStateV2,
    transaction_id: TransactionId,
    support_start_ns: u128,
    support_end_ns: u128,
}

impl SoilThermalUnpublishedPhysicalBeginningV2 {
    pub fn try_new(
        authority: &PreparedSoilThermalSupportV2,
        predecessor_trial: &SoilThermalTrialStateV2,
        transaction_id: TransactionId,
        support_start_ns: u128,
        support_end_ns: u128,
    ) -> Result<Self, SoilThermalExactCarryError> {
        let beginning = Self {
            authority: authority.clone(),
            predecessor_trial: predecessor_trial.clone(),
            transaction_id,
            support_start_ns,
            support_end_ns,
        };
        beginning.validate()?;
        Ok(beginning)
    }

    fn validate(&self) -> Result<(), SoilThermalExactCarryError> {
        let owner = self.authority.beginning_owner();
        owner.validate()?;
        self.predecessor_trial.validate_seal()?;
        let ending = self.predecessor_trial.ending_state();
        if self.transaction_id != owner.transaction_id
            || self.support_start_ns != self.predecessor_trial.support_end_ns()
            || self.support_end_ns != owner.support_end_ns
            || owner.support_start_ns > self.predecessor_trial.support_start_ns()
            || self.support_start_ns >= self.support_end_ns
            || self.support_end_ns - self.support_start_ns < MINIMUM_SUPPORT_NS
            || ending.owner_id != owner.state.owner_id
            || ending.configuration_sha256 != owner.state.configuration_sha256
            || ending.ofes.len() != owner.state.ofes.len()
            || ending.last_accepted_transaction_id != Some(self.predecessor_trial.transaction_id())
        {
            return Err(SoilThermalExactCarryError::Identity(
                "unpublished physical beginning identity or support",
            ));
        }
        for (ending_ofe, authority_ofe) in ending.ofes.iter().zip(&owner.state.ofes) {
            if ending_ofe.ofe_id != authority_ofe.ofe_id
                || ending_ofe.ordered_layers.len() != authority_ofe.ordered_layers.len()
                || ending_ofe
                    .ordered_layers
                    .iter()
                    .zip(&authority_ofe.ordered_layers)
                    .any(|(ending_layer, authority_layer)| {
                        ending_layer.layer_id != authority_layer.layer_id
                    })
            {
                return Err(SoilThermalExactCarryError::Identity(
                    "unpublished physical beginning topology",
                ));
            }
        }
        Ok(())
    }

    #[must_use]
    pub const fn authority(&self) -> &PreparedSoilThermalSupportV2 {
        &self.authority
    }

    #[must_use]
    pub const fn predecessor_trial(&self) -> &SoilThermalTrialStateV2 {
        &self.predecessor_trial
    }

    #[must_use]
    pub const fn transaction_id(&self) -> TransactionId {
        self.transaction_id
    }

    #[must_use]
    pub const fn support_start_ns(&self) -> u128 {
        self.support_start_ns
    }

    #[must_use]
    pub const fn support_end_ns(&self) -> u128 {
        self.support_end_ns
    }
}

pub(crate) struct SoilThermalLayerRead<'a> {
    pub(crate) layer_id: &'a SoilLayerId,
    pub(crate) temperature_k: f64,
    pub(crate) enthalpy_hi_j_m2_ofe_ground: f64,
    pub(crate) enthalpy_carry: Option<&'a ExactDyadicEnthalpy>,
}

pub(crate) enum SoilThermalOfeRead<'a> {
    V1(&'a SoilThermalOfeSnapshot),
    V2(&'a SoilThermalOfeStateV2),
}

impl<'a> SoilThermalFinalizationBeginning<'a> {
    pub(crate) fn validate(self) -> Result<(), LandSurfaceEnergyError> {
        match self {
            Self::V1(snapshot) => snapshot.validate(),
            Self::V2(view) => {
                let owner = view.owner();
                owner
                    .validate()
                    .map_err(|_| LandSurfaceEnergyError::OwnerEnvelope("invalid V2 beginning"))?;
                if owner.state.last_accepted_transaction_id
                    != owner.expected_predecessor_transaction_id
                    || owner.state.last_accepted_transaction_id == Some(owner.transaction_id)
                {
                    return Err(LandSurfaceEnergyError::OwnerEnvelope(
                        "invalid V2 beginning lineage",
                    ));
                }
                Ok(())
            }
            Self::V2Unpublished(beginning) => beginning.validate().map_err(|_| {
                LandSurfaceEnergyError::OwnerEnvelope("invalid unpublished V2 physical beginning")
            }),
        }
    }

    pub(crate) fn ofe(self, ofe_id: &OfeId) -> Option<SoilThermalOfeRead<'a>> {
        match self {
            Self::V1(snapshot) => snapshot
                .ofes
                .iter()
                .find(|row| &row.ofe_id == ofe_id)
                .map(SoilThermalOfeRead::V1),
            Self::V2(view) => view
                .owner()
                .state
                .ofes
                .iter()
                .find(|row| &row.ofe_id == ofe_id)
                .map(SoilThermalOfeRead::V2),
            Self::V2Unpublished(beginning) => beginning
                .predecessor_trial()
                .ending_state()
                .ofes
                .iter()
                .find(|row| &row.ofe_id == ofe_id)
                .map(SoilThermalOfeRead::V2),
        }
    }

    #[must_use]
    pub fn owner_id(self) -> &'a ResourceOwnerId {
        match self {
            Self::V1(snapshot) => &snapshot.owner_id,
            Self::V2(view) => &view.owner().state.owner_id,
            Self::V2Unpublished(beginning) => {
                &beginning.predecessor_trial().ending_state().owner_id
            }
        }
    }

    #[must_use]
    pub fn state_sha256(self) -> &'a Sha256Digest {
        match self {
            Self::V1(snapshot) => &snapshot.state_sha256,
            Self::V2(view) => &view.owner().state.state_sha256,
            Self::V2Unpublished(beginning) => {
                &beginning.predecessor_trial().ending_state().state_sha256
            }
        }
    }

    #[must_use]
    pub const fn is_v2(self) -> bool {
        matches!(self, Self::V2(_) | Self::V2Unpublished(_))
    }

    #[must_use]
    pub fn candidate_identity(self) -> SoilThermalCandidateBeginningIdentity {
        match self {
            Self::V1(snapshot) => SoilThermalCandidateBeginningIdentity::V1 {
                configuration_sha256: snapshot.configuration_sha256.clone(),
                last_accepted_transaction_id: snapshot.last_accepted_transaction_id,
            },
            Self::V2(view) => {
                let owner = view.owner();
                SoilThermalCandidateBeginningIdentity::V2 {
                    owner_tag: owner.owner_tag.clone(),
                    schema_sha256: owner.schema_sha256.clone(),
                    exact_carry_definition_sha256: owner.exact_carry_definition_sha256.clone(),
                    parent_v1_state_sha256: owner.parent_v1_state_sha256.clone(),
                    contract_version: owner.contract_version,
                    model_version: owner.model_version.clone(),
                    model_definition_sha256: owner.model_definition_sha256.clone(),
                    run_id: owner.run_id.clone(),
                    configuration_sha256: owner.state.configuration_sha256.clone(),
                    transaction_id: owner.transaction_id,
                    expected_predecessor_transaction_id: owner.expected_predecessor_transaction_id,
                    support_start_ns: owner.support_start_ns,
                    support_end_ns: owner.support_end_ns,
                    receipt_chain_sha256: owner.receipt_chain_sha256.clone(),
                }
            }
            Self::V2Unpublished(beginning) => {
                let owner = beginning.authority().beginning_owner();
                SoilThermalCandidateBeginningIdentity::V2 {
                    owner_tag: owner.owner_tag.clone(),
                    schema_sha256: owner.schema_sha256.clone(),
                    exact_carry_definition_sha256: owner.exact_carry_definition_sha256.clone(),
                    parent_v1_state_sha256: owner.parent_v1_state_sha256.clone(),
                    contract_version: owner.contract_version,
                    model_version: owner.model_version.clone(),
                    model_definition_sha256: owner.model_definition_sha256.clone(),
                    run_id: owner.run_id.clone(),
                    configuration_sha256: owner.state.configuration_sha256.clone(),
                    transaction_id: beginning.transaction_id(),
                    expected_predecessor_transaction_id: owner.expected_predecessor_transaction_id,
                    support_start_ns: beginning.support_start_ns(),
                    support_end_ns: beginning.support_end_ns(),
                    receipt_chain_sha256: owner.receipt_chain_sha256.clone(),
                }
            }
        }
    }
}

impl<'a> SoilThermalOfeRead<'a> {
    pub(crate) fn len(&self) -> usize {
        match self {
            Self::V1(ofe) => ofe.ordered_layers.len(),
            Self::V2(ofe) => ofe.ordered_layers.len(),
        }
    }

    pub(crate) fn layer(&self, index: usize) -> Option<SoilThermalLayerRead<'a>> {
        match self {
            Self::V1(ofe) => ofe
                .ordered_layers
                .get(index)
                .map(|layer| SoilThermalLayerRead {
                    layer_id: &layer.layer_id,
                    temperature_k: layer.temperature_k,
                    enthalpy_hi_j_m2_ofe_ground: layer.enthalpy_j_m2_ofe_ground,
                    enthalpy_carry: None,
                }),
            Self::V2(ofe) => ofe
                .ordered_layers
                .get(index)
                .map(|layer| SoilThermalLayerRead {
                    layer_id: &layer.layer_id,
                    temperature_k: layer.temperature_k,
                    enthalpy_hi_j_m2_ofe_ground: layer.enthalpy_hi_j_m2_ofe_ground,
                    enthalpy_carry: Some(&layer.enthalpy_carry),
                }),
        }
    }
}

/// Prepared native beginning owner for one half-open physical support.
#[derive(Clone, Debug, PartialEq)]
pub struct PreparedSoilThermalSupportV2 {
    beginning_owner: SoilThermalOwnerEnvelopeV2,
}

impl PreparedSoilThermalSupportV2 {
    #[must_use]
    pub const fn beginning_owner(&self) -> &SoilThermalOwnerEnvelopeV2 {
        &self.beginning_owner
    }

    #[must_use]
    pub const fn physical_read_view(&self) -> SoilThermalPhysicalReadViewV2<'_> {
        SoilThermalPhysicalReadViewV2 {
            owner: &self.beginning_owner,
        }
    }
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
            || self.support_end_ns - self.support_start_ns < MINIMUM_SUPPORT_NS
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

pub fn prepare_soil_thermal_support_v2(
    accepted_owner: &SoilThermalOwnerEnvelopeV2,
    transaction_id: TransactionId,
    support_start_ns: u128,
    support_end_ns: u128,
) -> Result<PreparedSoilThermalSupportV2, SoilThermalExactCarryError> {
    accepted_owner.validate()?;
    if transaction_id.0 == 0
        || Some(transaction_id) == accepted_owner.state.last_accepted_transaction_id
        || support_start_ns >= support_end_ns
        || support_end_ns - support_start_ns < MINIMUM_SUPPORT_NS
    {
        return Err(SoilThermalExactCarryError::Identity(
            "prepared support transaction or bounds",
        ));
    }
    let mut beginning_owner = accepted_owner.clone();
    beginning_owner.transaction_id = transaction_id;
    beginning_owner.expected_predecessor_transaction_id =
        accepted_owner.state.last_accepted_transaction_id;
    beginning_owner.support_start_ns = support_start_ns;
    beginning_owner.support_end_ns = support_end_ns;
    beginning_owner.validate()?;
    Ok(PreparedSoilThermalSupportV2 { beginning_owner })
}

fn soil_thermal_restart_v2_sha256(
    restart: &SoilThermalOwnerRestartV2,
) -> Result<Sha256Digest, SoilThermalExactCarryError> {
    canonical_digest(&(
        "OPENWEPP_SOIL_THERMAL_OWNER_RESTART_V2",
        &restart.owner_tag,
        &restart.schema_sha256,
        &restart.exact_carry_definition_sha256,
        &restart.parent_v1_state_sha256,
        &restart.owner_state_sha256,
        restart.last_accepted_transaction_id,
        &restart.receipt_chain_sha256,
    ))
    .map_err(|error| exact_carry_serialization(&error))
}

fn soil_thermal_checkpoint_v2_sha256(
    checkpoint: &SoilThermalOwnerCheckpointV2,
) -> Result<Sha256Digest, SoilThermalExactCarryError> {
    canonical_digest(&(
        "OPENWEPP_SOIL_THERMAL_OWNER_CHECKPOINT_V2",
        &checkpoint.owner_tag,
        &checkpoint.schema_sha256,
        &checkpoint.exact_carry_definition_sha256,
        &checkpoint.parent_v1_state_sha256,
        &checkpoint.owner_state_sha256,
        checkpoint.last_accepted_transaction_id,
        &checkpoint.receipt_chain_sha256,
    ))
    .map_err(|error| exact_carry_serialization(&error))
}

pub fn seal_soil_thermal_receipt_free_owner_v2(
    prepared: &PreparedSoilThermalSupportV2,
) -> Result<SoilThermalReceiptFreeOwnerSealsV2, SoilThermalExactCarryError> {
    let owner = prepared.beginning_owner();
    owner.validate()?;
    if owner.state.last_accepted_transaction_id != owner.expected_predecessor_transaction_id
        || owner.state.last_accepted_transaction_id == Some(owner.transaction_id)
    {
        return Err(SoilThermalExactCarryError::Identity(
            "receipt-free owner has current accepted transaction",
        ));
    }
    let zero =
        Sha256Digest::try_new("0".repeat(64)).map_err(|error| exact_carry_serialization(&error))?;
    let mut restart = SoilThermalOwnerRestartV2 {
        owner_tag: owner.owner_tag.clone(),
        schema_sha256: owner.schema_sha256.clone(),
        exact_carry_definition_sha256: owner.exact_carry_definition_sha256.clone(),
        parent_v1_state_sha256: owner.parent_v1_state_sha256.clone(),
        owner_state_sha256: owner.state.state_sha256.clone(),
        last_accepted_transaction_id: owner.state.last_accepted_transaction_id,
        receipt_chain_sha256: owner.receipt_chain_sha256.clone(),
        restart_sha256: zero.clone(),
    };
    restart.restart_sha256 = soil_thermal_restart_v2_sha256(&restart)?;
    let mut checkpoint = SoilThermalOwnerCheckpointV2 {
        owner_tag: owner.owner_tag.clone(),
        schema_sha256: owner.schema_sha256.clone(),
        exact_carry_definition_sha256: owner.exact_carry_definition_sha256.clone(),
        parent_v1_state_sha256: owner.parent_v1_state_sha256.clone(),
        owner_state_sha256: owner.state.state_sha256.clone(),
        last_accepted_transaction_id: owner.state.last_accepted_transaction_id,
        receipt_chain_sha256: owner.receipt_chain_sha256.clone(),
        checkpoint_sha256: zero,
    };
    checkpoint.checkpoint_sha256 = soil_thermal_checkpoint_v2_sha256(&checkpoint)?;
    let receipt_free_seal_sha256 = canonical_digest(&(
        "OPENWEPP_SOIL_THERMAL_RECEIPT_FREE_OWNER_SEALS_V2",
        &restart,
        &checkpoint,
        owner.transaction_id,
        owner.support_start_ns,
        owner.support_end_ns,
    ))
    .map_err(|error| exact_carry_serialization(&error))?;
    let seals = SoilThermalReceiptFreeOwnerSealsV2 {
        restart,
        checkpoint,
        receipt_free_seal_sha256,
    };
    validate_soil_thermal_receipt_free_owner_v2(prepared, &seals)?;
    Ok(seals)
}

pub fn validate_soil_thermal_receipt_free_owner_v2(
    prepared: &PreparedSoilThermalSupportV2,
    seals: &SoilThermalReceiptFreeOwnerSealsV2,
) -> Result<(), SoilThermalExactCarryError> {
    let owner = prepared.beginning_owner();
    owner.validate()?;
    let common = owner.state.last_accepted_transaction_id
        == owner.expected_predecessor_transaction_id
        && owner.state.last_accepted_transaction_id != Some(owner.transaction_id)
        && seals.restart.owner_tag == owner.owner_tag
        && seals.restart.schema_sha256 == owner.schema_sha256
        && seals.restart.exact_carry_definition_sha256 == owner.exact_carry_definition_sha256
        && seals.restart.parent_v1_state_sha256 == owner.parent_v1_state_sha256
        && seals.restart.owner_state_sha256 == owner.state.state_sha256
        && seals.restart.last_accepted_transaction_id == owner.state.last_accepted_transaction_id
        && seals.restart.receipt_chain_sha256 == owner.receipt_chain_sha256
        && seals.restart.restart_sha256 == soil_thermal_restart_v2_sha256(&seals.restart)?
        && seals.checkpoint.owner_tag == owner.owner_tag
        && seals.checkpoint.schema_sha256 == owner.schema_sha256
        && seals.checkpoint.exact_carry_definition_sha256 == owner.exact_carry_definition_sha256
        && seals.checkpoint.parent_v1_state_sha256 == owner.parent_v1_state_sha256
        && seals.checkpoint.owner_state_sha256 == owner.state.state_sha256
        && seals.checkpoint.last_accepted_transaction_id
            == owner.state.last_accepted_transaction_id
        && seals.checkpoint.receipt_chain_sha256 == owner.receipt_chain_sha256
        && seals.checkpoint.checkpoint_sha256
            == soil_thermal_checkpoint_v2_sha256(&seals.checkpoint)?
        && seals.receipt_free_seal_sha256
            == canonical_digest(&(
                "OPENWEPP_SOIL_THERMAL_RECEIPT_FREE_OWNER_SEALS_V2",
                &seals.restart,
                &seals.checkpoint,
                owner.transaction_id,
                owner.support_start_ns,
                owner.support_end_ns,
            ))
            .map_err(|error| exact_carry_serialization(&error))?;
    common
        .then_some(())
        .ok_or(SoilThermalExactCarryError::Identity(
            "receipt-free owner seal join",
        ))
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

/// Clone-only candidate: a refusal never mutates the supplied beginning owner.
#[derive(Clone, Debug, PartialEq)]
pub struct SoilThermalExactCarryCandidateV2 {
    pub ending_owner: SoilThermalOwnerEnvelopeV2,
    pub credit_receipt: SoilThermalEnergyCreditReceiptV2,
}

/// Unpublished exact receiver trial. It contains no accepted receipt and has
/// no installation API; only final receipt sealing can publish its ending.
#[derive(Clone, Debug, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
enum SoilThermalUnpublishedPredecessorCustodyV2 {
    AcceptedReceiptChain(Sha256Digest),
    UnpublishedTrial(Sha256Digest),
    NumericalCoordinateProjection {
        authority_sha256: Sha256Digest,
        accepted_receipt_chain_sha256: Sha256Digest,
        coordinate_set_sha256: Sha256Digest,
    },
}

/// One ordered private numerical coordinate for a native V2 soil layer.
///
/// The proposed enthalpy is the binary64 total `E`, not the stored high term.
/// Projection therefore stores this value as the high term with canonical
/// exact-zero carry; copying the beginning carry would count it twice.
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct SoilThermalUnpublishedCoordinateV2 {
    pub ofe_id: OfeId,
    pub layer_id: SoilLayerId,
    pub proposed_total_enthalpy_j_m2_ofe_ground: f64,
    pub proposed_temperature_k: f64,
}

/// Resealed private numerical image with no accepted-owner or receipt API.
#[derive(Clone, Debug, PartialEq)]
pub struct SoilThermalUnpublishedCoordinateProjectionV2 {
    trial: SoilThermalTrialStateV2,
}

impl SoilThermalUnpublishedCoordinateProjectionV2 {
    #[must_use]
    pub const fn trial(&self) -> &SoilThermalTrialStateV2 {
        &self.trial
    }

    #[must_use]
    pub fn into_trial(self) -> SoilThermalTrialStateV2 {
        self.trial
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct SoilThermalTrialStateV2 {
    transaction_id: TransactionId,
    predecessor_transaction_id: Option<TransactionId>,
    support_start_ns: u128,
    support_end_ns: u128,
    beginning_state_sha256: Sha256Digest,
    predecessor_custody: SoilThermalUnpublishedPredecessorCustodyV2,
    ending_state: SoilThermalOwnedStateV2,
    layer_credits: Vec<SoilThermalLayerEnergyCreditV2>,
    unpublished_trial_sha256: Sha256Digest,
}

impl SoilThermalTrialStateV2 {
    #[must_use]
    pub const fn transaction_id(&self) -> TransactionId {
        self.transaction_id
    }

    #[must_use]
    pub const fn predecessor_transaction_id(&self) -> Option<TransactionId> {
        self.predecessor_transaction_id
    }

    #[must_use]
    pub const fn support_start_ns(&self) -> u128 {
        self.support_start_ns
    }

    #[must_use]
    pub const fn support_end_ns(&self) -> u128 {
        self.support_end_ns
    }

    #[must_use]
    pub const fn beginning_state_sha256(&self) -> &Sha256Digest {
        &self.beginning_state_sha256
    }

    #[must_use]
    pub fn accepted_predecessor_receipt_chain_sha256(&self) -> Option<&Sha256Digest> {
        match &self.predecessor_custody {
            SoilThermalUnpublishedPredecessorCustodyV2::AcceptedReceiptChain(digest) => {
                Some(digest)
            }
            SoilThermalUnpublishedPredecessorCustodyV2::UnpublishedTrial(_) => None,
            SoilThermalUnpublishedPredecessorCustodyV2::NumericalCoordinateProjection {
                accepted_receipt_chain_sha256,
                ..
            } => Some(accepted_receipt_chain_sha256),
        }
    }

    #[must_use]
    pub fn unpublished_predecessor_trial_sha256(&self) -> Option<&Sha256Digest> {
        match &self.predecessor_custody {
            SoilThermalUnpublishedPredecessorCustodyV2::UnpublishedTrial(digest) => Some(digest),
            SoilThermalUnpublishedPredecessorCustodyV2::AcceptedReceiptChain(_)
            | SoilThermalUnpublishedPredecessorCustodyV2::NumericalCoordinateProjection {
                ..
            } => None,
        }
    }

    #[must_use]
    pub fn numerical_coordinate_set_sha256(&self) -> Option<&Sha256Digest> {
        match &self.predecessor_custody {
            SoilThermalUnpublishedPredecessorCustodyV2::NumericalCoordinateProjection {
                coordinate_set_sha256,
                ..
            } => Some(coordinate_set_sha256),
            SoilThermalUnpublishedPredecessorCustodyV2::AcceptedReceiptChain(_)
            | SoilThermalUnpublishedPredecessorCustodyV2::UnpublishedTrial(_) => None,
        }
    }

    #[must_use]
    pub fn numerical_coordinate_authority_sha256(&self) -> Option<&Sha256Digest> {
        match &self.predecessor_custody {
            SoilThermalUnpublishedPredecessorCustodyV2::NumericalCoordinateProjection {
                authority_sha256,
                ..
            } => Some(authority_sha256),
            SoilThermalUnpublishedPredecessorCustodyV2::AcceptedReceiptChain(_)
            | SoilThermalUnpublishedPredecessorCustodyV2::UnpublishedTrial(_) => None,
        }
    }

    #[must_use]
    pub const fn ending_state(&self) -> &SoilThermalOwnedStateV2 {
        &self.ending_state
    }

    #[must_use]
    pub fn layer_credits(&self) -> &[SoilThermalLayerEnergyCreditV2] {
        &self.layer_credits
    }

    #[must_use]
    pub const fn unpublished_trial_sha256(&self) -> &Sha256Digest {
        &self.unpublished_trial_sha256
    }

    fn reseal(&mut self) -> Result<(), SoilThermalExactCarryError> {
        self.unpublished_trial_sha256 = canonical_digest(&(
            "OPENWEPP_SOIL_THERMAL_UNPUBLISHED_TRIAL_V2",
            self.transaction_id,
            self.predecessor_transaction_id,
            self.support_start_ns,
            self.support_end_ns,
            &self.beginning_state_sha256,
            &self.predecessor_custody,
            &self.ending_state,
            &self.layer_credits,
        ))
        .map_err(|error| exact_carry_serialization(&error))?;
        Ok(())
    }

    fn validate_seal(&self) -> Result<(), SoilThermalExactCarryError> {
        let mut rebuilt = self.clone();
        rebuilt.reseal()?;
        if rebuilt.unpublished_trial_sha256 != self.unpublished_trial_sha256 {
            return Err(SoilThermalExactCarryError::Identity(
                "unpublished trial seal",
            ));
        }
        self.ending_state.validate()?;
        Ok(())
    }
}

fn unique_temperature_projection<'a>(
    projections: &'a [SoilThermalTemperatureProjectionV2],
    ofe_id: &OfeId,
    layer_id: &SoilLayerId,
) -> Result<&'a SoilThermalTemperatureProjectionV2, SoilThermalExactCarryError> {
    let mut matches = projections
        .iter()
        .filter(|row| &row.ofe_id == ofe_id && &row.layer_id == layer_id);
    let projection = matches
        .next()
        .ok_or(SoilThermalExactCarryError::Cardinality(
            "missing layer temperature projection",
        ))?;
    if matches.next().is_some() {
        return Err(SoilThermalExactCarryError::Cardinality(
            "duplicate layer temperature projection",
        ));
    }
    Ok(projection)
}

fn validate_exact_carry_predecessor(
    beginning: &SoilThermalOwnerEnvelopeV2,
) -> Result<(), SoilThermalExactCarryError> {
    beginning.validate()?;
    if beginning.state.last_accepted_transaction_id != beginning.expected_predecessor_transaction_id
        || beginning.expected_predecessor_transaction_id == Some(beginning.transaction_id)
    {
        return Err(SoilThermalExactCarryError::Identity(
            "stale or replayed V2 predecessor",
        ));
    }
    Ok(())
}

pub fn apply_soil_thermal_energy_credit_v2(
    beginning: &SoilThermalOwnerEnvelopeV2,
    accepted_operands: &[SoilThermalAcceptedEnergyOperandV2],
    temperature_projections: &[SoilThermalTemperatureProjectionV2],
) -> Result<SoilThermalExactCarryCandidateV2, SoilThermalExactCarryError> {
    validate_exact_carry_predecessor(beginning)?;
    let trial = advance_soil_thermal_trial_from_beginning_v2(
        beginning,
        accepted_operands,
        temperature_projections,
    )?;
    let mut ending = beginning.clone();
    ending.state = trial.ending_state;

    let zero_digest = Sha256Digest::try_new("0".repeat(64))
        .map_err(|error| SoilThermalExactCarryError::Serialization(error.to_string()))?;
    let mut receipt = SoilThermalEnergyCreditReceiptV2 {
        receipt_tag: SOIL_THERMAL_ENERGY_CREDIT_RECEIPT_V2_TAG.to_owned(),
        schema_sha256: Sha256Digest::try_new(SOIL_THERMAL_OWNER_V2_SCHEMA_SHA256)
            .map_err(|error| SoilThermalExactCarryError::Serialization(error.to_string()))?,
        exact_carry_definition_sha256: Sha256Digest::try_new(
            EXACT_DYADIC_ENTHALPY_V1_DEFINITION_SHA256,
        )
        .map_err(|error| SoilThermalExactCarryError::Serialization(error.to_string()))?,
        contract_version: 15,
        model_version: beginning.model_version.clone(),
        model_definition_sha256: beginning.model_definition_sha256.clone(),
        configuration_sha256: beginning.state.configuration_sha256.clone(),
        run_id: beginning.run_id.clone(),
        soil_thermal_owner_id: beginning.state.owner_id.clone(),
        transaction_id: beginning.transaction_id,
        predecessor_transaction_id: beginning.expected_predecessor_transaction_id,
        support_start_ns: beginning.support_start_ns,
        support_end_ns: beginning.support_end_ns,
        beginning_owner_state_sha256: beginning.state.state_sha256.clone(),
        ending_owner_state_sha256: ending.state.state_sha256.clone(),
        predecessor_receipt_chain_sha256: beginning.receipt_chain_sha256.clone(),
        layer_credits: trial.layer_credits,
        receipt_sha256: zero_digest,
    };
    receipt.reseal()?;
    ending.receipt_chain_sha256 = receipt.receipt_sha256.clone();
    receipt.validate_independent(
        beginning,
        &ending,
        accepted_operands,
        temperature_projections,
    )?;
    Ok(SoilThermalExactCarryCandidateV2 {
        ending_owner: ending,
        credit_receipt: receipt,
    })
}

pub fn advance_soil_thermal_trial_v2(
    prepared: &PreparedSoilThermalSupportV2,
    physical_operands: &[SoilThermalAcceptedEnergyOperandV2],
    temperature_projections: &[SoilThermalTemperatureProjectionV2],
) -> Result<SoilThermalTrialStateV2, SoilThermalExactCarryError> {
    advance_soil_thermal_trial_from_beginning_v2(
        prepared.beginning_owner(),
        physical_operands,
        temperature_projections,
    )
}

/// Recompute an unpublished composed trial from its original authenticated
/// beginning while binding the trial to the latest contiguous child support.
///
/// The returned state remains receipt-free and cannot be installed. The child
/// support is metadata for continuation validation only; all energy operands
/// are accumulated and applied once to the original beginning.
pub fn advance_soil_thermal_composed_trial_v2(
    original_prepared: &PreparedSoilThermalSupportV2,
    child_support_start_ns: u128,
    child_support_end_ns: u128,
    accumulated_operands: &[SoilThermalAcceptedEnergyOperandV2],
    temperature_projections: &[SoilThermalTemperatureProjectionV2],
) -> Result<SoilThermalTrialStateV2, SoilThermalExactCarryError> {
    let original = original_prepared.beginning_owner();
    if child_support_start_ns < original.support_start_ns
        || child_support_end_ns > original.support_end_ns
        || child_support_start_ns >= child_support_end_ns
        || child_support_end_ns - child_support_start_ns < MINIMUM_SUPPORT_NS
    {
        return Err(SoilThermalExactCarryError::Identity(
            "composed trial child support",
        ));
    }
    let mut trial = advance_soil_thermal_trial_from_beginning_v2(
        original,
        accumulated_operands,
        temperature_projections,
    )?;
    trial.support_start_ns = child_support_start_ns;
    trial.support_end_ns = child_support_end_ns;
    trial.reseal()?;
    Ok(trial)
}

/// Advance one private unpublished child from the retained trial ending.
///
/// The predecessor is another unpublished trial seal, never an accepted
/// receipt chain. Consequently this result cannot satisfy an owner-envelope
/// acceptance or installation API, while its physical ending and layer
/// credits are the exact sequential child result.
pub fn advance_soil_thermal_sequential_unpublished_trial_v2(
    retained: &SoilThermalTrialStateV2,
    child_support_start_ns: u128,
    child_support_end_ns: u128,
    child_operands: &[SoilThermalAcceptedEnergyOperandV2],
    temperature_projections: &[SoilThermalTemperatureProjectionV2],
) -> Result<SoilThermalTrialStateV2, SoilThermalExactCarryError> {
    retained.validate_seal()?;
    if retained.numerical_coordinate_set_sha256().is_some()
        || retained.support_end_ns != child_support_start_ns
        || child_support_start_ns >= child_support_end_ns
        || child_support_end_ns - child_support_start_ns < MINIMUM_SUPPORT_NS
    {
        return Err(SoilThermalExactCarryError::Identity(
            "sequential unpublished child support",
        ));
    }
    let transaction_id = retained
        .transaction_id
        .0
        .checked_add(1)
        .map(TransactionId)
        .ok_or(SoilThermalExactCarryError::Identity(
            "sequential unpublished child transaction overflow",
        ))?;
    advance_soil_thermal_trial_from_state_v2(
        transaction_id,
        Some(retained.transaction_id),
        child_support_start_ns,
        child_support_end_ns,
        retained.ending_state.clone(),
        SoilThermalUnpublishedPredecessorCustodyV2::UnpublishedTrial(
            retained.unpublished_trial_sha256.clone(),
        ),
        child_operands,
        temperature_projections,
    )
}

/// Validate the private unpublished-trial seal and ending state without
/// promoting the trial into an accepted owner or receipt.
pub fn validate_soil_thermal_unpublished_trial_v2(
    trial: &SoilThermalTrialStateV2,
) -> Result<(), SoilThermalExactCarryError> {
    trial.validate_seal()
}

fn soil_thermal_numerical_coordinate_digests_v2(
    beginning: &SoilThermalOwnerEnvelopeV2,
    scope: SoilThermalNumericalCoordinateScopeV2,
    coordinates: &[SoilThermalUnpublishedCoordinateV2],
) -> Result<(Sha256Digest, Sha256Digest), SoilThermalExactCarryError> {
    let authority = canonical_digest(&(
        "OPENWEPP_SOIL_THERMAL_NUMERICAL_COORDINATE_AUTHORITY_V2",
        beginning,
    ))
    .map_err(|error| exact_carry_serialization(&error))?;
    let coordinates = canonical_digest(&(
        "OPENWEPP_SOIL_THERMAL_NUMERICAL_COORDINATE_SET_V2",
        &authority,
        scope,
        coordinates,
    ))
    .map_err(|error| exact_carry_serialization(&error))?;
    Ok((authority, coordinates))
}

#[derive(Clone, Copy, Serialize)]
enum SoilThermalNumericalCoordinateScopeV2 {
    AllLayers,
    TopLayerPerOfe,
}

/// Project one ordered numerical `E/T` image from an immutable authenticated
/// V2 beginning without executing physics or creating publication authority.
///
/// Each proposed `E` is the complete binary64 total. It is independently
/// reconstructed as `exact(E) + 0`, stored as the high term with canonical
/// exact-zero carry, and resealed into a private trial. The dedicated custody
/// tag is rejected by sequential and accepted-owner construction paths.
pub fn project_soil_thermal_unpublished_coordinates_v2(
    prepared: &PreparedSoilThermalSupportV2,
    coordinates: &[SoilThermalUnpublishedCoordinateV2],
) -> Result<SoilThermalUnpublishedCoordinateProjectionV2, SoilThermalExactCarryError> {
    project_soil_thermal_unpublished_coordinates_with_scope_v2(
        prepared,
        coordinates,
        SoilThermalNumericalCoordinateScopeV2::AllLayers,
    )
}

/// Project exactly one ordered top-layer `E/T` coordinate per OFE while
/// retaining every lower layer bit-for-bit from the authenticated beginning.
pub fn project_soil_thermal_unpublished_top_layer_coordinates_v2(
    prepared: &PreparedSoilThermalSupportV2,
    coordinates: &[SoilThermalUnpublishedCoordinateV2],
) -> Result<SoilThermalUnpublishedCoordinateProjectionV2, SoilThermalExactCarryError> {
    project_soil_thermal_unpublished_coordinates_with_scope_v2(
        prepared,
        coordinates,
        SoilThermalNumericalCoordinateScopeV2::TopLayerPerOfe,
    )
}

fn project_soil_thermal_unpublished_coordinates_with_scope_v2(
    prepared: &PreparedSoilThermalSupportV2,
    coordinates: &[SoilThermalUnpublishedCoordinateV2],
    scope: SoilThermalNumericalCoordinateScopeV2,
) -> Result<SoilThermalUnpublishedCoordinateProjectionV2, SoilThermalExactCarryError> {
    let beginning = prepared.beginning_owner();
    validate_exact_carry_predecessor(beginning)?;
    let layer_count = match scope {
        SoilThermalNumericalCoordinateScopeV2::AllLayers => beginning
            .state
            .ofes
            .iter()
            .map(|ofe| ofe.ordered_layers.len())
            .sum::<usize>(),
        SoilThermalNumericalCoordinateScopeV2::TopLayerPerOfe => beginning.state.ofes.len(),
    };
    if coordinates.len() != layer_count {
        return Err(SoilThermalExactCarryError::Cardinality(
            "numerical coordinate layer count",
        ));
    }
    let (authority_sha256, coordinate_set_sha256) =
        soil_thermal_numerical_coordinate_digests_v2(beginning, scope, coordinates)?;
    let mut ending = beginning.state.clone();
    let mut coordinate_iter = coordinates.iter();
    for ofe in &beginning.state.ofes {
        for (layer_index, layer) in ofe.ordered_layers.iter().enumerate() {
            if matches!(scope, SoilThermalNumericalCoordinateScopeV2::TopLayerPerOfe)
                && layer_index != 0
            {
                continue;
            }
            let coordinate =
                coordinate_iter
                    .next()
                    .ok_or(SoilThermalExactCarryError::Cardinality(
                        "missing numerical coordinate",
                    ))?;
            if coordinate.ofe_id != ofe.ofe_id || coordinate.layer_id != layer.layer_id {
                return Err(SoilThermalExactCarryError::Identity(
                    "numerical coordinate layer order",
                ));
            }
            if !coordinate
                .proposed_total_enthalpy_j_m2_ofe_ground
                .is_finite()
                || (coordinate.proposed_total_enthalpy_j_m2_ofe_ground == 0.0
                    && coordinate.proposed_total_enthalpy_j_m2_ofe_ground.to_bits()
                        != 0.0_f64.to_bits())
                || !coordinate.proposed_temperature_k.is_finite()
                || !(200.0..=350.0).contains(&coordinate.proposed_temperature_k)
            {
                return Err(SoilThermalExactCarryError::Domain(
                    "numerical coordinate value",
                ));
            }
            let high =
                ExactDyadicEnthalpy::from_f64(coordinate.proposed_total_enthalpy_j_m2_ofe_ground)?;
            let carry = ExactDyadicEnthalpy::zero();
            let total = ExactDyadicEnthalpy::exact_sum([&high, &carry])?;
            let (reconstructed_high, reconstructed_carry) = total.rounded_high_and_remainder()?;
            if reconstructed_high.to_bits()
                != coordinate.proposed_total_enthalpy_j_m2_ofe_ground.to_bits()
                || reconstructed_carry != carry
            {
                return Err(SoilThermalExactCarryError::Reconstruction);
            }
            let ending_layer = ending.layer_mut(&ofe.ofe_id, &layer.layer_id).ok_or(
                SoilThermalExactCarryError::Identity("numerical coordinate ending layer"),
            )?;
            ending_layer.enthalpy_hi_j_m2_ofe_ground = reconstructed_high;
            ending_layer.enthalpy_carry = reconstructed_carry;
            ending_layer.temperature_k = coordinate.proposed_temperature_k;
            if matches!(scope, SoilThermalNumericalCoordinateScopeV2::AllLayers) {
                ending_layer.last_accepted_transaction_id = Some(beginning.transaction_id);
            }
        }
    }
    if coordinate_iter.next().is_some() {
        return Err(SoilThermalExactCarryError::Cardinality(
            "extra numerical coordinate",
        ));
    }
    if matches!(scope, SoilThermalNumericalCoordinateScopeV2::AllLayers) {
        ending.last_accepted_transaction_id = Some(beginning.transaction_id);
    }
    ending.reseal()?;
    ending.validate()?;
    let zero =
        Sha256Digest::try_new("0".repeat(64)).map_err(|error| exact_carry_serialization(&error))?;
    let mut trial = SoilThermalTrialStateV2 {
        transaction_id: beginning.transaction_id,
        predecessor_transaction_id: beginning.expected_predecessor_transaction_id,
        support_start_ns: beginning.support_start_ns,
        support_end_ns: beginning.support_end_ns,
        beginning_state_sha256: beginning.state.state_sha256.clone(),
        predecessor_custody:
            SoilThermalUnpublishedPredecessorCustodyV2::NumericalCoordinateProjection {
                authority_sha256,
                accepted_receipt_chain_sha256: beginning.receipt_chain_sha256.clone(),
                coordinate_set_sha256,
            },
        ending_state: ending,
        layer_credits: Vec::new(),
        unpublished_trial_sha256: zero,
    };
    trial.reseal()?;
    trial.validate_seal()?;
    Ok(SoilThermalUnpublishedCoordinateProjectionV2 { trial })
}

/// Seal one outer accepted owner from an authenticated sequential unpublished
/// ending and the complete canonical operand accumulation.
///
/// The selected ending supplies exact constitutive temperature/high/carry.
/// Independent accepted-credit reconstruction from the outer beginning and
/// every accumulated operand must reproduce those physical fields exactly.
/// No intermediate child receipt or accepted owner is created.
pub fn compose_soil_thermal_accepted_from_unpublished_v2(
    original_prepared: &PreparedSoilThermalSupportV2,
    selected_trial: &SoilThermalTrialStateV2,
    accumulated_operands: &[SoilThermalAcceptedEnergyOperandV2],
    layer_credit_chain: &[Vec<SoilThermalLayerEnergyCreditV2>],
) -> Result<SoilThermalExactCarryCandidateV2, SoilThermalExactCarryError> {
    let beginning = original_prepared.beginning_owner();
    validate_exact_carry_predecessor(beginning)?;
    selected_trial.validate_seal()?;
    if selected_trial
        .unpublished_predecessor_trial_sha256()
        .is_none()
        || selected_trial
            .accepted_predecessor_receipt_chain_sha256()
            .is_some()
        || selected_trial.support_start_ns < beginning.support_start_ns
        || selected_trial.support_end_ns != beginning.support_end_ns
    {
        return Err(SoilThermalExactCarryError::Identity(
            "selected unpublished outer support or custody",
        ));
    }
    if layer_credit_chain.len() < 2
        || layer_credit_chain.last().map(Vec::as_slice) != Some(selected_trial.layer_credits())
    {
        return Err(SoilThermalExactCarryError::Identity(
            "selected unpublished layer-credit chain",
        ));
    }
    let mut ending = beginning.clone();
    ending.state = selected_trial.ending_state().clone();
    ending.state.last_accepted_transaction_id = Some(beginning.transaction_id);
    for ofe in &mut ending.state.ofes {
        for layer in &mut ofe.ordered_layers {
            layer.last_accepted_transaction_id = Some(beginning.transaction_id);
        }
    }
    ending.state.reseal()?;
    let zero_digest = Sha256Digest::try_new("0".repeat(64))
        .map_err(|error| SoilThermalExactCarryError::Serialization(error.to_string()))?;
    let mut receipt = SoilThermalEnergyCreditReceiptV2 {
        receipt_tag: SOIL_THERMAL_ENERGY_CREDIT_RECEIPT_V2_TAG.to_owned(),
        schema_sha256: Sha256Digest::try_new(SOIL_THERMAL_OWNER_V2_SCHEMA_SHA256)
            .map_err(|error| SoilThermalExactCarryError::Serialization(error.to_string()))?,
        exact_carry_definition_sha256: Sha256Digest::try_new(
            EXACT_DYADIC_ENTHALPY_V1_DEFINITION_SHA256,
        )
        .map_err(|error| SoilThermalExactCarryError::Serialization(error.to_string()))?,
        contract_version: 15,
        model_version: beginning.model_version.clone(),
        model_definition_sha256: beginning.model_definition_sha256.clone(),
        configuration_sha256: beginning.state.configuration_sha256.clone(),
        run_id: beginning.run_id.clone(),
        soil_thermal_owner_id: beginning.state.owner_id.clone(),
        transaction_id: beginning.transaction_id,
        predecessor_transaction_id: beginning.expected_predecessor_transaction_id,
        support_start_ns: beginning.support_start_ns,
        support_end_ns: beginning.support_end_ns,
        beginning_owner_state_sha256: beginning.state.state_sha256.clone(),
        ending_owner_state_sha256: ending.state.state_sha256.clone(),
        predecessor_receipt_chain_sha256: beginning.receipt_chain_sha256.clone(),
        layer_credits: layer_credit_chain.iter().flatten().cloned().collect(),
        receipt_sha256: zero_digest,
    };
    receipt.reseal()?;
    ending.receipt_chain_sha256 = receipt.receipt_sha256.clone();
    receipt.validate_independent(beginning, &ending, accumulated_operands, &[])?;
    Ok(SoilThermalExactCarryCandidateV2 {
        ending_owner: ending,
        credit_receipt: receipt,
    })
}

fn advance_soil_thermal_trial_from_beginning_v2(
    beginning: &SoilThermalOwnerEnvelopeV2,
    accepted_operands: &[SoilThermalAcceptedEnergyOperandV2],
    temperature_projections: &[SoilThermalTemperatureProjectionV2],
) -> Result<SoilThermalTrialStateV2, SoilThermalExactCarryError> {
    validate_exact_carry_predecessor(beginning)?;

    advance_soil_thermal_trial_from_state_v2(
        beginning.transaction_id,
        beginning.expected_predecessor_transaction_id,
        beginning.support_start_ns,
        beginning.support_end_ns,
        beginning.state.clone(),
        SoilThermalUnpublishedPredecessorCustodyV2::AcceptedReceiptChain(
            beginning.receipt_chain_sha256.clone(),
        ),
        accepted_operands,
        temperature_projections,
    )
}

#[allow(clippy::too_many_arguments)]
fn advance_soil_thermal_trial_from_state_v2(
    transaction_id: TransactionId,
    predecessor_transaction_id: Option<TransactionId>,
    support_start_ns: u128,
    support_end_ns: u128,
    beginning: SoilThermalOwnedStateV2,
    predecessor_custody: SoilThermalUnpublishedPredecessorCustodyV2,
    accepted_operands: &[SoilThermalAcceptedEnergyOperandV2],
    temperature_projections: &[SoilThermalTemperatureProjectionV2],
) -> Result<SoilThermalTrialStateV2, SoilThermalExactCarryError> {
    beginning.validate()?;
    if transaction_id.0 == 0
        || support_start_ns >= support_end_ns
        || support_end_ns - support_start_ns < MINIMUM_SUPPORT_NS
    {
        return Err(SoilThermalExactCarryError::Identity(
            "unpublished trial transaction or support",
        ));
    }
    let mut ending = beginning.clone();
    let mut layer_credits = Vec::new();
    for ofe in &beginning.ofes {
        for layer in &ofe.ordered_layers {
            let projection = unique_temperature_projection(
                temperature_projections,
                &ofe.ofe_id,
                &layer.layer_id,
            )?;
            let layer_operands: Vec<_> = accepted_operands
                .iter()
                .filter(|operand| {
                    operand.ofe_id == ofe.ofe_id && operand.layer_id == layer.layer_id
                })
                .cloned()
                .collect();
            let values: Vec<_> = layer_operands
                .iter()
                .map(|operand| operand.energy_j_m2_ofe_ground)
                .collect();
            let exact_total = ExactDyadicEnthalpy::exact_sum_binary64(
                layer.enthalpy_hi_j_m2_ofe_ground,
                &layer.enthalpy_carry,
                &values,
            )?;
            let (high, carry) = if values.is_empty() {
                (
                    layer.enthalpy_hi_j_m2_ofe_ground,
                    layer.enthalpy_carry.clone(),
                )
            } else {
                exact_total.rounded_high_and_remainder()?
            };
            let ending_layer = ending.layer_mut(&ofe.ofe_id, &layer.layer_id).ok_or(
                SoilThermalExactCarryError::Identity("candidate layer identity"),
            )?;
            ending_layer.enthalpy_hi_j_m2_ofe_ground = high;
            ending_layer.enthalpy_carry = carry.clone();
            ending_layer.temperature_k = projection.ending_temperature_k;
            ending_layer.last_accepted_transaction_id = Some(transaction_id);
            layer_credits.push(SoilThermalLayerEnergyCreditV2 {
                ofe_id: ofe.ofe_id.clone(),
                layer_id: layer.layer_id.clone(),
                beginning_enthalpy_hi_j_m2_ofe_ground: layer.enthalpy_hi_j_m2_ofe_ground,
                beginning_enthalpy_carry: layer.enthalpy_carry.clone(),
                beginning_temperature_k: layer.temperature_k,
                ending_enthalpy_hi_j_m2_ofe_ground: high,
                ending_enthalpy_carry: carry,
                ending_temperature_k: projection.ending_temperature_k,
                heat_capacity_j_m2_k: projection.heat_capacity_j_m2_k,
                accepted_operands: layer_operands,
            });
        }
    }
    ending.last_accepted_transaction_id = Some(transaction_id);
    ending.reseal()?;
    let zero =
        Sha256Digest::try_new("0".repeat(64)).map_err(|error| exact_carry_serialization(&error))?;
    let mut trial = SoilThermalTrialStateV2 {
        transaction_id,
        predecessor_transaction_id,
        support_start_ns,
        support_end_ns,
        beginning_state_sha256: beginning.state_sha256,
        predecessor_custody,
        ending_state: ending,
        layer_credits,
        unpublished_trial_sha256: zero,
    };
    trial.reseal()?;
    Ok(trial)
}
