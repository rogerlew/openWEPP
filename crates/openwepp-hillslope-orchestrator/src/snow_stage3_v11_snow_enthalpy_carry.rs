//! Authenticated compound material ownership for the V56 frozen-snow carry.
//!
//! This module owns identity and custody only. It deliberately does not run
//! Stage 3, derive heat exchange, select a solver candidate, or publish state.

use std::collections::{BTreeMap, BTreeSet};

use openwepp_coupled_time::{Digest32, ParentTransactionId, TimeSupport, digest_bytes};
use openwepp_land_surface_energy::ExactDyadicEnthalpy;
use serde::{Deserialize, Serialize};
use thiserror::Error;

use crate::hydrology::DirectSnowStage3PersistentState;

const OWNER_SCHEMA_VERSION: u16 = 1;
const RECEIPT_SCHEMA_VERSION: u16 = 1;
const DEFINITION_DOMAIN: &[u8] = b"OPENWEPP_COVERED_SNOW_ENTHALPY_HIGH_PLUS_CARRY_V1";
const OWNER_DOMAIN: &str = "OPENWEPP_AUTHENTICATED_COVERED_SNOW_MATERIAL_OWNER_V1";
const BASE_OWNER_DOMAIN: &str = "OPENWEPP_STAGE3_CANONICAL_SNOW_OWNER_V1";

#[derive(Clone, Debug, Error, Eq, PartialEq)]
pub enum CoveredSnowEnthalpyCarryErrorV1 {
    #[error("invalid V56 snow-carry identity: {0}")]
    Identity(&'static str),
    #[error("invalid V56 snow-carry state: {0}")]
    State(&'static str),
    #[error("invalid V56 snow-carry custody: {0}")]
    Custody(&'static str),
    #[error("noncanonical V56 snow-carry bytes: {0}")]
    CanonicalBytes(&'static str),
    #[error("V56 snow-carry downgrade refused: {0}")]
    Downgrade(&'static str),
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct CoveredSnowEnthalpyCarryStateV1 {
    lane_id: u32,
    layer_order: u32,
    enthalpy_hi_j_m2_ofe_ground_bits: u64,
    enthalpy_carry: ExactDyadicEnthalpy,
    temperature_k_bits: u64,
}

impl CoveredSnowEnthalpyCarryStateV1 {
    pub fn new(
        lane_id: u32,
        layer_order: u32,
        enthalpy_hi_j_m2_ofe_ground: f64,
        enthalpy_carry: ExactDyadicEnthalpy,
        temperature_k: f64,
    ) -> Result<Self, CoveredSnowEnthalpyCarryErrorV1> {
        let value = Self {
            lane_id,
            layer_order,
            enthalpy_hi_j_m2_ofe_ground_bits: enthalpy_hi_j_m2_ofe_ground.to_bits(),
            enthalpy_carry,
            temperature_k_bits: temperature_k.to_bits(),
        };
        value.validate()?;
        Ok(value)
    }

    pub fn zero_carry(
        lane_id: u32,
        layer_order: u32,
        enthalpy_hi_j_m2_ofe_ground: f64,
        temperature_k: f64,
    ) -> Result<Self, CoveredSnowEnthalpyCarryErrorV1> {
        Self::new(
            lane_id,
            layer_order,
            enthalpy_hi_j_m2_ofe_ground,
            ExactDyadicEnthalpy::zero(),
            temperature_k,
        )
    }

    pub fn validate(&self) -> Result<(), CoveredSnowEnthalpyCarryErrorV1> {
        let high = self.enthalpy_hi_j_m2_ofe_ground();
        let temperature = self.temperature_k();
        if !high.is_finite() || !temperature.is_finite() || !(0.0..273.15).contains(&temperature) {
            return Err(CoveredSnowEnthalpyCarryErrorV1::State(
                "finite frozen high/temperature",
            ));
        }
        self.enthalpy_carry.validate().map_err(|_| {
            CoveredSnowEnthalpyCarryErrorV1::State("canonical exact-dyadic remainder")
        })?;
        let total = ExactDyadicEnthalpy::exact_sum([
            &ExactDyadicEnthalpy::from_f64(high)
                .map_err(|_| CoveredSnowEnthalpyCarryErrorV1::State("finite enthalpy high"))?,
            &self.enthalpy_carry,
        ])
        .map_err(|_| CoveredSnowEnthalpyCarryErrorV1::State("exact high-plus-carry sum"))?;
        if total
            .round_to_f64()
            .map_err(|_| CoveredSnowEnthalpyCarryErrorV1::State("round-nearest-even high"))?
            .to_bits()
            != high.to_bits()
        {
            return Err(CoveredSnowEnthalpyCarryErrorV1::State(
                "high is not the round-nearest-even image of high-plus-carry",
            ));
        }
        Ok(())
    }

    #[must_use]
    pub const fn lane_id(&self) -> u32 {
        self.lane_id
    }

    #[must_use]
    pub const fn layer_order(&self) -> u32 {
        self.layer_order
    }

    #[must_use]
    pub fn enthalpy_hi_j_m2_ofe_ground(&self) -> f64 {
        f64::from_bits(self.enthalpy_hi_j_m2_ofe_ground_bits)
    }

    #[must_use]
    pub const fn enthalpy_carry(&self) -> &ExactDyadicEnthalpy {
        &self.enthalpy_carry
    }

    #[must_use]
    pub fn temperature_k(&self) -> f64 {
        f64::from_bits(self.temperature_k_bits)
    }

    #[must_use]
    pub fn has_zero_carry(&self) -> bool {
        self.enthalpy_carry == ExactDyadicEnthalpy::zero()
    }
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum CoveredSnowEnthalpyEnergyOperandKindV1 {
    ExternalSurface,
    InternalConduction,
    SnowSoilCrankNicolson,
    LatentMassTransfer,
    MeltRefreeze,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct CoveredSnowEnthalpyEnergyOperandV1 {
    ordinal: u32,
    kind: CoveredSnowEnthalpyEnergyOperandKindV1,
    energy_j_m2_ofe_ground_bits: u64,
}

impl CoveredSnowEnthalpyEnergyOperandV1 {
    pub fn new(
        ordinal: u32,
        kind: CoveredSnowEnthalpyEnergyOperandKindV1,
        energy_j_m2_ofe_ground: f64,
    ) -> Result<Self, CoveredSnowEnthalpyCarryErrorV1> {
        if !energy_j_m2_ofe_ground.is_finite() {
            return Err(CoveredSnowEnthalpyCarryErrorV1::State(
                "finite ordered energy operand",
            ));
        }
        Ok(Self {
            ordinal,
            kind,
            energy_j_m2_ofe_ground_bits: energy_j_m2_ofe_ground.to_bits(),
        })
    }

    #[must_use]
    pub const fn ordinal(&self) -> u32 {
        self.ordinal
    }

    #[must_use]
    pub const fn kind(&self) -> CoveredSnowEnthalpyEnergyOperandKindV1 {
        self.kind
    }

    #[must_use]
    pub fn energy_j_m2_ofe_ground(&self) -> f64 {
        f64::from_bits(self.energy_j_m2_ofe_ground_bits)
    }
}

#[derive(Clone, Debug)]
pub struct CoveredSnowEnthalpyCarryReceiptInputsV1 {
    pub support: TimeSupport,
    pub transaction_id: ParentTransactionId,
    pub predecessor_transaction_id: Option<ParentTransactionId>,
    pub beginning_carries: Vec<CoveredSnowEnthalpyCarryStateV1>,
    pub ending_carries: Vec<CoveredSnowEnthalpyCarryStateV1>,
    pub ordered_energy_operands: Vec<CoveredSnowEnthalpyEnergyOperandV1>,
    pub base_material_owner_sha256: Digest32,
    pub beginning_compound_owner_sha256: Digest32,
    pub predecessor_receipt_chain_sha256: Digest32,
    pub branch_identity_sha256: Digest32,
    pub topology_identity_sha256: Digest32,
    pub configuration_identity_sha256: Digest32,
    pub custody_identity_sha256: Digest32,
    pub candidate_sha256: Digest32,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct CoveredSnowEnthalpyCarryReceiptV1 {
    schema_version: u16,
    definition_sha256: Digest32,
    support: TimeSupport,
    transaction_id: ParentTransactionId,
    predecessor_transaction_id: Option<ParentTransactionId>,
    beginning_carries: Vec<CoveredSnowEnthalpyCarryStateV1>,
    ending_carries: Vec<CoveredSnowEnthalpyCarryStateV1>,
    ordered_energy_operands: Vec<CoveredSnowEnthalpyEnergyOperandV1>,
    base_material_owner_sha256: Digest32,
    beginning_compound_owner_sha256: Digest32,
    predecessor_receipt_chain_sha256: Digest32,
    branch_identity_sha256: Digest32,
    topology_identity_sha256: Digest32,
    configuration_identity_sha256: Digest32,
    custody_identity_sha256: Digest32,
    candidate_sha256: Digest32,
    receipt_sha256: Digest32,
}

impl CoveredSnowEnthalpyCarryReceiptV1 {
    pub fn seal(
        inputs: CoveredSnowEnthalpyCarryReceiptInputsV1,
    ) -> Result<Self, CoveredSnowEnthalpyCarryErrorV1> {
        let mut value = Self {
            schema_version: RECEIPT_SCHEMA_VERSION,
            definition_sha256: covered_snow_enthalpy_carry_definition_sha256(),
            support: inputs.support,
            transaction_id: inputs.transaction_id,
            predecessor_transaction_id: inputs.predecessor_transaction_id,
            beginning_carries: inputs.beginning_carries,
            ending_carries: inputs.ending_carries,
            ordered_energy_operands: inputs.ordered_energy_operands,
            base_material_owner_sha256: inputs.base_material_owner_sha256,
            beginning_compound_owner_sha256: inputs.beginning_compound_owner_sha256,
            predecessor_receipt_chain_sha256: inputs.predecessor_receipt_chain_sha256,
            branch_identity_sha256: inputs.branch_identity_sha256,
            topology_identity_sha256: inputs.topology_identity_sha256,
            configuration_identity_sha256: inputs.configuration_identity_sha256,
            custody_identity_sha256: inputs.custody_identity_sha256,
            candidate_sha256: inputs.candidate_sha256,
            receipt_sha256: Digest32::zero(),
        };
        value.validate_unsealed()?;
        value.receipt_sha256 = digest_bytes(&value.seal_preimage()?);
        value.validate()?;
        Ok(value)
    }

    pub fn validate(&self) -> Result<(), CoveredSnowEnthalpyCarryErrorV1> {
        self.validate_unsealed()?;
        if self.receipt_sha256 == Digest32::zero()
            || self.receipt_sha256 != digest_bytes(&self.seal_preimage()?)
        {
            return Err(CoveredSnowEnthalpyCarryErrorV1::Identity(
                "carry receipt seal",
            ));
        }
        Ok(())
    }

    fn validate_unsealed(&self) -> Result<(), CoveredSnowEnthalpyCarryErrorV1> {
        if self.schema_version != RECEIPT_SCHEMA_VERSION
            || self.definition_sha256 != covered_snow_enthalpy_carry_definition_sha256()
            || self.transaction_id.digest() == Digest32::zero()
            || self
                .predecessor_transaction_id
                .is_some_and(|value| value.digest() == Digest32::zero())
            || self.base_material_owner_sha256 == Digest32::zero()
            || self.beginning_compound_owner_sha256 == Digest32::zero()
            || self.branch_identity_sha256 == Digest32::zero()
            || self.topology_identity_sha256 == Digest32::zero()
            || self.configuration_identity_sha256 == Digest32::zero()
            || self.custody_identity_sha256 == Digest32::zero()
            || self.candidate_sha256 == Digest32::zero()
        {
            return Err(CoveredSnowEnthalpyCarryErrorV1::Identity(
                "schema, definition, transaction, owner, branch, topology, configuration, custody, or candidate",
            ));
        }
        let has_predecessor_transaction = self.predecessor_transaction_id.is_some();
        let has_predecessor_receipt = self.predecessor_receipt_chain_sha256 != Digest32::zero();
        if has_predecessor_transaction != has_predecessor_receipt {
            return Err(CoveredSnowEnthalpyCarryErrorV1::Custody(
                "transaction and receipt predecessor presence",
            ));
        }
        validate_carry_order(&self.beginning_carries)?;
        validate_carry_order(&self.ending_carries)?;
        if self.beginning_carries.len() != self.ending_carries.len()
            || self
                .beginning_carries
                .iter()
                .zip(&self.ending_carries)
                .any(|(beginning, ending)| {
                    (beginning.lane_id(), beginning.layer_order())
                        != (ending.lane_id(), ending.layer_order())
                })
        {
            return Err(CoveredSnowEnthalpyCarryErrorV1::Custody(
                "beginning/ending lane and layer order",
            ));
        }
        if self.ordered_energy_operands.is_empty()
            || self
                .ordered_energy_operands
                .iter()
                .enumerate()
                .any(|(index, operand)| {
                    usize::try_from(operand.ordinal()).ok() != Some(index)
                        || !operand.energy_j_m2_ofe_ground().is_finite()
                })
        {
            return Err(CoveredSnowEnthalpyCarryErrorV1::Custody(
                "complete canonical energy operand order",
            ));
        }
        Ok(())
    }

    fn seal_preimage(&self) -> Result<Vec<u8>, CoveredSnowEnthalpyCarryErrorV1> {
        let mut value = self.clone();
        value.receipt_sha256 = Digest32::zero();
        serde_json::to_vec(&value).map_err(|_| {
            CoveredSnowEnthalpyCarryErrorV1::CanonicalBytes("carry receipt seal preimage")
        })
    }

    pub fn canonical_bytes(&self) -> Result<Vec<u8>, CoveredSnowEnthalpyCarryErrorV1> {
        self.validate()?;
        serde_json::to_vec(self)
            .map_err(|_| CoveredSnowEnthalpyCarryErrorV1::CanonicalBytes("carry receipt bytes"))
    }

    pub fn from_canonical_bytes(bytes: &[u8]) -> Result<Self, CoveredSnowEnthalpyCarryErrorV1> {
        let value: Self = serde_json::from_slice(bytes)
            .map_err(|_| CoveredSnowEnthalpyCarryErrorV1::CanonicalBytes("carry receipt decode"))?;
        value.validate()?;
        if value.canonical_bytes()?.as_slice() != bytes {
            return Err(CoveredSnowEnthalpyCarryErrorV1::CanonicalBytes(
                "carry receipt re-encoding",
            ));
        }
        Ok(value)
    }

    pub fn validate_successor_of(
        &self,
        predecessor: &AuthenticatedCoveredSnowMaterialOwnerV1,
    ) -> Result<(), CoveredSnowEnthalpyCarryErrorV1> {
        self.validate()?;
        predecessor.validate()?;
        if self.predecessor_transaction_id != Some(predecessor.receipt.transaction_id())
            || self.predecessor_receipt_chain_sha256 != predecessor.receipt.receipt_sha256()
            || self.beginning_compound_owner_sha256 != predecessor.compound_owner_sha256()
            || self.beginning_carries != predecessor.carries
            || self.support.start_ns() != predecessor.receipt.support().end_ns()
            || self.branch_identity_sha256 != predecessor.receipt.branch_identity_sha256()
            || self.topology_identity_sha256 != predecessor.receipt.topology_identity_sha256()
            || self.configuration_identity_sha256
                != predecessor.receipt.configuration_identity_sha256()
            || self.custody_identity_sha256 != predecessor.receipt.custody_identity_sha256()
        {
            return Err(CoveredSnowEnthalpyCarryErrorV1::Custody(
                "stale or discontinuous compound-owner predecessor",
            ));
        }
        Ok(())
    }

    #[must_use]
    pub const fn support(&self) -> TimeSupport {
        self.support
    }

    #[must_use]
    pub const fn support_start_ns(&self) -> u128 {
        self.support.start_ns().get()
    }

    #[must_use]
    pub const fn support_end_ns(&self) -> u128 {
        self.support.end_ns().get()
    }

    #[must_use]
    pub const fn transaction_id(&self) -> ParentTransactionId {
        self.transaction_id
    }

    #[must_use]
    pub const fn predecessor_transaction_id(&self) -> Option<ParentTransactionId> {
        self.predecessor_transaction_id
    }

    #[must_use]
    pub fn beginning_carries(&self) -> &[CoveredSnowEnthalpyCarryStateV1] {
        &self.beginning_carries
    }

    #[must_use]
    pub fn ending_carries(&self) -> &[CoveredSnowEnthalpyCarryStateV1] {
        &self.ending_carries
    }

    #[must_use]
    pub fn ordered_energy_operands(&self) -> &[CoveredSnowEnthalpyEnergyOperandV1] {
        &self.ordered_energy_operands
    }

    #[must_use]
    pub const fn base_material_owner_sha256(&self) -> Digest32 {
        self.base_material_owner_sha256
    }

    #[must_use]
    pub const fn beginning_compound_owner_sha256(&self) -> Digest32 {
        self.beginning_compound_owner_sha256
    }

    #[must_use]
    pub const fn predecessor_receipt_chain_sha256(&self) -> Digest32 {
        self.predecessor_receipt_chain_sha256
    }

    #[must_use]
    pub const fn candidate_sha256(&self) -> Digest32 {
        self.candidate_sha256
    }

    #[must_use]
    pub const fn definition_sha256(&self) -> Digest32 {
        self.definition_sha256
    }

    #[must_use]
    pub const fn branch_identity_sha256(&self) -> Digest32 {
        self.branch_identity_sha256
    }

    #[must_use]
    pub const fn topology_identity_sha256(&self) -> Digest32 {
        self.topology_identity_sha256
    }

    #[must_use]
    pub const fn configuration_identity_sha256(&self) -> Digest32 {
        self.configuration_identity_sha256
    }

    #[must_use]
    pub const fn custody_identity_sha256(&self) -> Digest32 {
        self.custody_identity_sha256
    }

    #[must_use]
    pub const fn receipt_sha256(&self) -> Digest32 {
        self.receipt_sha256
    }
}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct AuthenticatedCoveredSnowMaterialOwnerV1 {
    schema_version: u16,
    definition_sha256: Digest32,
    base_material_owner: BTreeMap<u32, DirectSnowStage3PersistentState>,
    carries: Vec<CoveredSnowEnthalpyCarryStateV1>,
    receipt: CoveredSnowEnthalpyCarryReceiptV1,
    base_material_owner_sha256: Digest32,
    candidate_sha256: Digest32,
    compound_owner_sha256: Digest32,
}

impl AuthenticatedCoveredSnowMaterialOwnerV1 {
    pub fn seal(
        base_material_owner: BTreeMap<u32, DirectSnowStage3PersistentState>,
        carries: Vec<CoveredSnowEnthalpyCarryStateV1>,
        receipt: CoveredSnowEnthalpyCarryReceiptV1,
    ) -> Result<Self, CoveredSnowEnthalpyCarryErrorV1> {
        let base_material_owner_sha256 =
            covered_snow_base_material_owner_sha256(&base_material_owner)?;
        let candidate_sha256 =
            covered_snow_material_candidate_sha256(&base_material_owner, &carries)?;
        let mut value = Self {
            schema_version: OWNER_SCHEMA_VERSION,
            definition_sha256: covered_snow_enthalpy_carry_definition_sha256(),
            base_material_owner,
            carries,
            receipt,
            base_material_owner_sha256,
            candidate_sha256,
            compound_owner_sha256: Digest32::zero(),
        };
        value.validate_unsealed()?;
        value.compound_owner_sha256 = digest_bytes(&value.seal_preimage()?);
        value.validate()?;
        Ok(value)
    }

    /// V4-to-V5 migration is deliberately limited to canonical exact-zero carry.
    pub fn migrate_zero_carry(
        base_material_owner: BTreeMap<u32, DirectSnowStage3PersistentState>,
        receipt: CoveredSnowEnthalpyCarryReceiptV1,
    ) -> Result<Self, CoveredSnowEnthalpyCarryErrorV1> {
        if receipt
            .ending_carries()
            .iter()
            .any(|state| !state.has_zero_carry())
        {
            return Err(CoveredSnowEnthalpyCarryErrorV1::Custody(
                "migration may introduce only canonical exact-zero carry",
            ));
        }
        Self::seal(
            base_material_owner,
            receipt.ending_carries().to_vec(),
            receipt,
        )
    }

    pub fn validate(&self) -> Result<(), CoveredSnowEnthalpyCarryErrorV1> {
        self.validate_unsealed()?;
        if self.compound_owner_sha256 == Digest32::zero()
            || self.compound_owner_sha256 != digest_bytes(&self.seal_preimage()?)
        {
            return Err(CoveredSnowEnthalpyCarryErrorV1::Identity(
                "compound owner seal",
            ));
        }
        Ok(())
    }

    fn validate_unsealed(&self) -> Result<(), CoveredSnowEnthalpyCarryErrorV1> {
        if self.schema_version != OWNER_SCHEMA_VERSION
            || self.definition_sha256 != covered_snow_enthalpy_carry_definition_sha256()
        {
            return Err(CoveredSnowEnthalpyCarryErrorV1::Identity(
                "owner schema or definition",
            ));
        }
        self.receipt.validate()?;
        validate_carry_order(&self.carries)?;
        validate_base_material_owner(&self.base_material_owner, &self.carries)?;
        let base_digest = covered_snow_base_material_owner_sha256(&self.base_material_owner)?;
        let candidate =
            covered_snow_material_candidate_sha256(&self.base_material_owner, &self.carries)?;
        if self.base_material_owner_sha256 != base_digest
            || self.receipt.base_material_owner_sha256() != base_digest
            || self.candidate_sha256 != candidate
            || self.receipt.candidate_sha256() != candidate
            || self.receipt.ending_carries() != self.carries
        {
            return Err(CoveredSnowEnthalpyCarryErrorV1::Custody(
                "base owner, candidate, or ending carry join",
            ));
        }
        Ok(())
    }

    fn seal_preimage(&self) -> Result<Vec<u8>, CoveredSnowEnthalpyCarryErrorV1> {
        let mut value = self.clone();
        value.compound_owner_sha256 = Digest32::zero();
        serde_json::to_vec(&value).map_err(|_| {
            CoveredSnowEnthalpyCarryErrorV1::CanonicalBytes("compound owner seal preimage")
        })
    }

    pub fn canonical_bytes(&self) -> Result<Vec<u8>, CoveredSnowEnthalpyCarryErrorV1> {
        self.validate()?;
        serde_json::to_vec(self)
            .map_err(|_| CoveredSnowEnthalpyCarryErrorV1::CanonicalBytes("compound owner bytes"))
    }

    pub fn from_canonical_bytes(bytes: &[u8]) -> Result<Self, CoveredSnowEnthalpyCarryErrorV1> {
        let value: Self = serde_json::from_slice(bytes).map_err(|_| {
            CoveredSnowEnthalpyCarryErrorV1::CanonicalBytes("compound owner decode")
        })?;
        value.validate()?;
        if value.canonical_bytes()?.as_slice() != bytes {
            return Err(CoveredSnowEnthalpyCarryErrorV1::CanonicalBytes(
                "compound owner re-encoding",
            ));
        }
        Ok(value)
    }

    pub fn refuse_nonzero_carry_downgrade(
        &self,
    ) -> Result<&BTreeMap<u32, DirectSnowStage3PersistentState>, CoveredSnowEnthalpyCarryErrorV1>
    {
        self.validate()?;
        if self.carries.iter().any(|state| !state.has_zero_carry()) {
            return Err(CoveredSnowEnthalpyCarryErrorV1::Downgrade(
                "nonzero carry blocks downgrade",
            ));
        }
        Ok(&self.base_material_owner)
    }

    #[must_use]
    pub const fn base_material_owner(&self) -> &BTreeMap<u32, DirectSnowStage3PersistentState> {
        &self.base_material_owner
    }

    #[must_use]
    pub fn carries(&self) -> &[CoveredSnowEnthalpyCarryStateV1] {
        &self.carries
    }

    #[must_use]
    pub const fn receipt(&self) -> &CoveredSnowEnthalpyCarryReceiptV1 {
        &self.receipt
    }

    #[must_use]
    pub const fn base_material_owner_sha256(&self) -> Digest32 {
        self.base_material_owner_sha256
    }

    #[must_use]
    pub const fn candidate_sha256(&self) -> Digest32 {
        self.candidate_sha256
    }

    #[must_use]
    pub const fn compound_owner_sha256(&self) -> Digest32 {
        self.compound_owner_sha256
    }

    pub fn whole_compound_eq(&self, other: &Self) -> Result<bool, CoveredSnowEnthalpyCarryErrorV1> {
        Ok(self.canonical_bytes()? == other.canonical_bytes()?)
    }
}

#[must_use]
pub fn covered_snow_enthalpy_carry_definition_sha256() -> Digest32 {
    digest_bytes(DEFINITION_DOMAIN)
}

pub fn covered_snow_base_material_owner_sha256(
    owner: &BTreeMap<u32, DirectSnowStage3PersistentState>,
) -> Result<Digest32, CoveredSnowEnthalpyCarryErrorV1> {
    if owner.is_empty() {
        return Err(CoveredSnowEnthalpyCarryErrorV1::State(
            "nonempty base material owner",
        ));
    }
    #[derive(Serialize)]
    struct CanonicalSnowOwner<'a> {
        schema: &'static str,
        lanes: Vec<(&'a u32, &'a DirectSnowStage3PersistentState)>,
    }
    let bytes = serde_json::to_vec(&CanonicalSnowOwner {
        schema: BASE_OWNER_DOMAIN,
        lanes: owner.iter().collect(),
    })
    .map_err(|_| CoveredSnowEnthalpyCarryErrorV1::CanonicalBytes("base material owner"))?;
    Ok(digest_bytes(&bytes))
}

pub fn covered_snow_material_candidate_sha256(
    owner: &BTreeMap<u32, DirectSnowStage3PersistentState>,
    carries: &[CoveredSnowEnthalpyCarryStateV1],
) -> Result<Digest32, CoveredSnowEnthalpyCarryErrorV1> {
    validate_carry_order(carries)?;
    validate_base_material_owner(owner, carries)?;
    #[derive(Serialize)]
    struct Candidate<'a> {
        domain: &'static str,
        base_material_owner_sha256: Digest32,
        carries: &'a [CoveredSnowEnthalpyCarryStateV1],
    }
    let bytes = serde_json::to_vec(&Candidate {
        domain: OWNER_DOMAIN,
        base_material_owner_sha256: covered_snow_base_material_owner_sha256(owner)?,
        carries,
    })
    .map_err(|_| CoveredSnowEnthalpyCarryErrorV1::CanonicalBytes("candidate identity"))?;
    Ok(digest_bytes(&bytes))
}

fn validate_carry_order(
    carries: &[CoveredSnowEnthalpyCarryStateV1],
) -> Result<(), CoveredSnowEnthalpyCarryErrorV1> {
    if carries.is_empty() {
        return Err(CoveredSnowEnthalpyCarryErrorV1::State(
            "nonempty carry state",
        ));
    }
    let mut seen = BTreeSet::new();
    let mut previous = None;
    for carry in carries {
        carry.validate()?;
        let key = (carry.lane_id(), carry.layer_order());
        if !seen.insert(key) || previous.is_some_and(|value| value >= key) {
            return Err(CoveredSnowEnthalpyCarryErrorV1::Custody(
                "strict lane/layer order without aliases",
            ));
        }
        previous = Some(key);
    }
    Ok(())
}

fn validate_base_material_owner(
    owner: &BTreeMap<u32, DirectSnowStage3PersistentState>,
    carries: &[CoveredSnowEnthalpyCarryStateV1],
) -> Result<(), CoveredSnowEnthalpyCarryErrorV1> {
    if owner.len() != carries.len() {
        return Err(CoveredSnowEnthalpyCarryErrorV1::Custody(
            "one ordered carry per base-owner lane",
        ));
    }
    for (carry, (lane_id, state)) in carries.iter().zip(owner) {
        if carry.layer_order() != 0
            || carry.lane_id() != *lane_id
            || state.lane_id != *lane_id
            || state.schema_version == 0
            || state.layers.len() != 1
        {
            return Err(CoveredSnowEnthalpyCarryErrorV1::Custody(
                "terminal-one-volume lane identity/order",
            ));
        }
        let layer = &state.layers[0];
        let numerics = [
            layer.mass_swe_m,
            layer.thickness_m,
            layer.density_kg_m3,
            layer.settle_day_count,
            layer.temperature_c,
            layer.liquid_water_m,
            layer.cold_content_j_m2,
            layer.refrozen_liquid_m,
            state.detached_retained_liquid_kg_m2,
            state.cumulative_terminal_unallocated_energy_j_m2,
        ];
        if numerics.iter().any(|value| !value.is_finite())
            || layer.mass_swe_m <= 0.0
            || layer.thickness_m <= 0.0
            || layer.density_kg_m3 <= 0.0
            || layer.temperature_c >= 0.0
            || layer.liquid_water_m.to_bits() != 0.0_f64.to_bits()
            || layer.refrozen_liquid_m.to_bits() != 0.0_f64.to_bits()
            || state.detached_retained_liquid_kg_m2.to_bits() != 0.0_f64.to_bits()
            || state.cumulative_terminal_unallocated_energy_j_m2.to_bits() != 0.0_f64.to_bits()
            || carry.enthalpy_hi_j_m2_ofe_ground().to_bits() != (-layer.cold_content_j_m2).to_bits()
            || carry.temperature_k().to_bits() != (layer.temperature_c + 273.15).to_bits()
        {
            return Err(CoveredSnowEnthalpyCarryErrorV1::State(
                "strictly frozen terminal-one-volume material/carry join",
            ));
        }
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::winter_column::DirectSnowLayerState;
    use openwepp_coupled_time::ModelTimeNs;

    fn digest(value: u8) -> Digest32 {
        Digest32::from_bytes([value; 32])
    }

    fn state(lane_id: u32, cold: f64, temperature_c: f64) -> DirectSnowStage3PersistentState {
        DirectSnowStage3PersistentState {
            schema_version: 2,
            terminal_event_model: None,
            fingerprint: u64::from(lane_id) + 1,
            lane_id,
            next_interval_index: 2,
            layers: vec![
                DirectSnowLayerState::new(0.04, 0.4, 100.0, 1.0).with_stage3_thermal_liquid_state(
                    temperature_c,
                    0.0,
                    cold,
                    0.0,
                ),
            ],
            detached_retained_liquid_kg_m2: 0.0,
            initial_ice_kg_m2: 40.0,
            initial_retained_liquid_kg_m2: 0.0,
            cumulative_snowfall_kg_m2: 0.0,
            cumulative_external_liquid_kg_m2: 0.0,
            cumulative_deposition_kg_m2: 0.0,
            cumulative_sublimation_kg_m2: 0.0,
            cumulative_melt_kg_m2: 0.0,
            cumulative_unresolved_liquid_kg_m2: 0.0,
            cumulative_complete_energy_j_m2: cold,
            cumulative_cold_energy_change_j_m2: cold,
            cumulative_terminal_unallocated_energy_j_m2: 0.0,
        }
    }

    fn carry(lane_id: u32, cold: f64, temperature_c: f64) -> CoveredSnowEnthalpyCarryStateV1 {
        CoveredSnowEnthalpyCarryStateV1::zero_carry(lane_id, 0, cold, temperature_c + 273.15)
            .expect("zero carry")
    }

    fn fixture() -> AuthenticatedCoveredSnowMaterialOwnerV1 {
        let base = BTreeMap::from([(1, state(1, 8000.0, -10.0))]);
        let carries = vec![carry(1, -8000.0, -10.0)];
        let candidate =
            covered_snow_material_candidate_sha256(&base, &carries).expect("candidate digest");
        let receipt =
            CoveredSnowEnthalpyCarryReceiptV1::seal(CoveredSnowEnthalpyCarryReceiptInputsV1 {
                support: TimeSupport::new(ModelTimeNs::new(60), ModelTimeNs::new(120))
                    .expect("support"),
                transaction_id: ParentTransactionId::from_digest(digest(1)),
                predecessor_transaction_id: Some(ParentTransactionId::from_digest(digest(2))),
                beginning_carries: carries.clone(),
                ending_carries: carries.clone(),
                ordered_energy_operands: vec![
                    CoveredSnowEnthalpyEnergyOperandV1::new(
                        0,
                        CoveredSnowEnthalpyEnergyOperandKindV1::SnowSoilCrankNicolson,
                        0.0,
                    )
                    .expect("operand"),
                ],
                base_material_owner_sha256: covered_snow_base_material_owner_sha256(&base)
                    .expect("base digest"),
                beginning_compound_owner_sha256: digest(3),
                predecessor_receipt_chain_sha256: digest(4),
                branch_identity_sha256: digest(5),
                topology_identity_sha256: digest(6),
                configuration_identity_sha256: digest(7),
                custody_identity_sha256: digest(8),
                candidate_sha256: candidate,
            })
            .expect("receipt");
        AuthenticatedCoveredSnowMaterialOwnerV1::seal(base, carries, receipt)
            .expect("compound owner")
    }

    #[test]
    fn compound_owner_is_canonical_and_whole_equal() {
        let owner = fixture();
        owner.validate().expect("valid owner");
        let bytes = owner.canonical_bytes().expect("canonical bytes");
        let replay = AuthenticatedCoveredSnowMaterialOwnerV1::from_canonical_bytes(&bytes)
            .expect("canonical replay");
        assert!(owner.whole_compound_eq(&replay).expect("whole equality"));
        assert_eq!(
            owner.compound_owner_sha256(),
            replay.compound_owner_sha256()
        );
    }

    #[test]
    fn aliases_order_stale_candidate_and_carry_poison_fail_closed() {
        let base = BTreeMap::from([(1, state(1, 8000.0, -10.0)), (2, state(2, 9000.0, -11.0))]);
        let duplicate = vec![carry(1, -8000.0, -10.0), carry(1, -8000.0, -10.0)];
        assert!(covered_snow_material_candidate_sha256(&base, &duplicate).is_err());

        let reversed = vec![carry(2, -9000.0, -11.0), carry(1, -8000.0, -10.0)];
        assert!(covered_snow_material_candidate_sha256(&base, &reversed).is_err());

        let mut owner = fixture();
        owner.candidate_sha256 = digest(91);
        assert!(owner.validate().is_err());

        let nonzero = ExactDyadicEnthalpy::try_new(1, "1", -80).expect("nonzero carry");
        let mut owner = fixture();
        owner.carries[0].enthalpy_carry = nonzero;
        assert!(owner.validate().is_err());
    }

    #[test]
    fn zero_migration_admits_and_nonzero_downgrade_refuses() {
        let owner = fixture();
        owner
            .refuse_nonzero_carry_downgrade()
            .expect("zero-carry downgrade");
        let migrated = AuthenticatedCoveredSnowMaterialOwnerV1::migrate_zero_carry(
            owner.base_material_owner.clone(),
            owner.receipt.clone(),
        )
        .expect("zero-carry migration");
        assert!(
            migrated
                .whole_compound_eq(&owner)
                .expect("migration equality")
        );

        let nonzero = ExactDyadicEnthalpy::try_new(1, "1", -80).expect("nonzero carry");
        let high = -8000.0;
        let exact = ExactDyadicEnthalpy::exact_sum([
            &ExactDyadicEnthalpy::from_f64(high).expect("high"),
            &nonzero,
        ])
        .expect("exact total");
        assert_eq!(
            exact.round_to_f64().expect("rounded high").to_bits(),
            high.to_bits()
        );
        let mut nonzero_owner = fixture();
        nonzero_owner.carries[0].enthalpy_carry = nonzero.clone();
        nonzero_owner.receipt.ending_carries[0].enthalpy_carry = nonzero;
        nonzero_owner.candidate_sha256 = covered_snow_material_candidate_sha256(
            &nonzero_owner.base_material_owner,
            &nonzero_owner.carries,
        )
        .expect("nonzero candidate");
        nonzero_owner.receipt.candidate_sha256 = nonzero_owner.candidate_sha256;
        nonzero_owner.receipt.receipt_sha256 = digest_bytes(
            &nonzero_owner
                .receipt
                .seal_preimage()
                .expect("receipt preimage"),
        );
        nonzero_owner.compound_owner_sha256 =
            digest_bytes(&nonzero_owner.seal_preimage().expect("owner preimage"));
        nonzero_owner.validate().expect("valid nonzero owner");
        assert!(matches!(
            nonzero_owner.refuse_nonzero_carry_downgrade(),
            Err(CoveredSnowEnthalpyCarryErrorV1::Downgrade(
                "nonzero carry blocks downgrade"
            ))
        ));
    }

    #[test]
    fn receipt_substitution_and_noncanonical_bytes_refuse() {
        let mut owner = fixture();
        owner.receipt.predecessor_receipt_chain_sha256 = digest(42);
        assert!(owner.validate().is_err());

        let owner = fixture();
        let mut bytes = owner.canonical_bytes().expect("canonical bytes");
        bytes.push(b' ');
        assert!(AuthenticatedCoveredSnowMaterialOwnerV1::from_canonical_bytes(&bytes).is_err());
    }
}
