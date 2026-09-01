//! V16 exact per-tile LSE surface-enthalpy owner.
//!
//! Frozen LSE V3 and surface-owner V2 fields remain binary64 high mirrors.
//! This companion alone owns `U = exact(U_hi) + R_U`.

#![allow(clippy::missing_errors_doc)]

use std::{collections::BTreeMap, fmt};

use openwepp_kernel_contract::{ResourceOwnerId, TransactionId};
use openwepp_land_surface_energy::{
    ExactDyadicEnthalpy, ExactDyadicEnthalpyError, LandSurfaceEnergyConfiguration,
    LandSurfaceEnergyV3State, Sha256Digest,
};
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};

use crate::{
    DirectSurfaceLiquidStoreKey, SurfaceLiquidConfigurationV2, SurfaceLiquidOwnerEnvelopeV2,
};

pub const LSE_SURFACE_ENTHALPY_OWNER_V1_TAG: &str = "OPENWEPP_LSE_SURFACE_ENTHALPY_OWNER_V1";
pub const LSE_SURFACE_ENTHALPY_OWNER_V1_SCHEMA_SHA256: &str =
    "cfc5118bad0c0aa940fba68b0b0a218c3a13ad09dec676be162126219a1d88e5";
pub const LSE_SURFACE_ENTHALPY_EXACT_CARRY_V1_DEFINITION_SHA256: &str =
    "add7641bb5e7e60cd4b15243f95d5eef03b45446fe898975bc89c55164b085de";
pub const LSE_SURFACE_ENTHALPY_ENERGY_CREDIT_RECEIPT_V1_TAG: &str =
    "OPENWEPP_LSE_SURFACE_ENTHALPY_ENERGY_CREDIT_RECEIPT_V1";
const ZERO_SHA256: &str = "0000000000000000000000000000000000000000000000000000000000000000";

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum LseSurfaceEnthalpyErrorV1 {
    Exact(ExactDyadicEnthalpyError),
    Identity(&'static str),
    Domain(&'static str),
    Cardinality(&'static str),
    Reconstruction,
    Serialization(String),
    DowngradeProhibited,
}

impl fmt::Display for LseSurfaceEnthalpyErrorV1 {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str("LSEB-E-050 / SURFACELIQUID-E-012: ")?;
        match self {
            Self::Exact(error) => write!(formatter, "{error}"),
            Self::Identity(detail) => write!(formatter, "identity refusal: {detail}"),
            Self::Domain(detail) => write!(formatter, "domain refusal: {detail}"),
            Self::Cardinality(detail) => write!(formatter, "cardinality refusal: {detail}"),
            Self::Reconstruction => formatter.write_str("exact reconstruction mismatch"),
            Self::Serialization(detail) => write!(formatter, "serialization refusal: {detail}"),
            Self::DowngradeProhibited => formatter.write_str("production exact-owner downgrade"),
        }
    }
}

impl std::error::Error for LseSurfaceEnthalpyErrorV1 {}

impl From<ExactDyadicEnthalpyError> for LseSurfaceEnthalpyErrorV1 {
    fn from(error: ExactDyadicEnthalpyError) -> Self {
        Self::Exact(error)
    }
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, Ord, PartialEq, PartialOrd, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum LseSurfaceEnthalpyEnergyOperandKindV1 {
    PhaseFreeSurfaceEnergy,
    LitterFusionEnergy,
    RetainedIngressTileCredit,
}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct LseSurfaceEnthalpyAcceptedEnergyOperandV1 {
    pub surface_key: DirectSurfaceLiquidStoreKey,
    pub kind: LseSurfaceEnthalpyEnergyOperandKindV1,
    pub ordinal: u32,
    pub source_owner_id: ResourceOwnerId,
    pub source_receipt_sha256: Sha256Digest,
    pub transaction_id: TransactionId,
    pub predecessor_transaction_id: Option<TransactionId>,
    pub support_start_ns: u128,
    pub support_end_ns: u128,
    pub units: String,
    pub basis: String,
    pub energy_j_m2_tile_ground: f64,
}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct LseSurfaceEnthalpyStateRecordV1 {
    pub surface_key: DirectSurfaceLiquidStoreKey,
    pub enthalpy_hi_j_m2_tile: f64,
    pub enthalpy_carry: ExactDyadicEnthalpy,
    pub last_accepted_transaction_id: Option<TransactionId>,
}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct LseSurfaceEnthalpyOwnerEnvelopeV1 {
    pub owner_tag: String,
    pub schema_sha256: Sha256Digest,
    pub exact_carry_definition_sha256: Sha256Digest,
    pub owner_id: ResourceOwnerId,
    pub run_id: String,
    pub configuration_sha256: Sha256Digest,
    pub frozen_lse_v3_state_sha256: Sha256Digest,
    pub frozen_surface_owner_v2_sha256: Sha256Digest,
    pub state_sha256: Sha256Digest,
    pub receipt_chain_sha256: Sha256Digest,
    pub records: Vec<LseSurfaceEnthalpyStateRecordV1>,
}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct LseSurfaceEnthalpyEnergyCreditReceiptV1 {
    pub receipt_tag: String,
    pub schema_sha256: Sha256Digest,
    pub exact_carry_definition_sha256: Sha256Digest,
    pub transaction_id: TransactionId,
    pub predecessor_transaction_id: Option<TransactionId>,
    pub support_start_ns: u128,
    pub support_end_ns: u128,
    pub beginning_owner_state_sha256: Sha256Digest,
    pub ending_owner_state_sha256: Sha256Digest,
    pub predecessor_receipt_chain_sha256: Sha256Digest,
    pub accepted_operands: Vec<LseSurfaceEnthalpyAcceptedEnergyOperandV1>,
    pub receipt_sha256: Sha256Digest,
}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct LseSurfaceEnthalpyOwnerRestartV1 {
    pub owner: LseSurfaceEnthalpyOwnerEnvelopeV1,
    pub restart_sha256: Sha256Digest,
}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct LseSurfaceEnthalpyOwnerCheckpointV1 {
    pub owner: LseSurfaceEnthalpyOwnerEnvelopeV1,
    pub receipt: Option<LseSurfaceEnthalpyEnergyCreditReceiptV1>,
    pub checkpoint_sha256: Sha256Digest,
}

#[derive(Clone, Debug, PartialEq)]
pub struct LseSurfaceEnthalpyAcceptedCandidateV1 {
    pub ending_owner: LseSurfaceEnthalpyOwnerEnvelopeV1,
    pub receipt: LseSurfaceEnthalpyEnergyCreditReceiptV1,
}

impl LseSurfaceEnthalpyEnergyCreditReceiptV1 {
    fn recomputed_sha256(&self) -> Result<Sha256Digest, LseSurfaceEnthalpyErrorV1> {
        let mut value = self.clone();
        value.receipt_sha256 = wire_digest(ZERO_SHA256)?;
        digest(&value)
    }

    pub fn validate(
        &self,
        beginning: &LseSurfaceEnthalpyOwnerEnvelopeV1,
        ending: &LseSurfaceEnthalpyOwnerEnvelopeV1,
    ) -> Result<(), LseSurfaceEnthalpyErrorV1> {
        beginning.validate()?;
        ending.validate()?;
        if self.receipt_tag != LSE_SURFACE_ENTHALPY_ENERGY_CREDIT_RECEIPT_V1_TAG
            || self.schema_sha256.as_str() != LSE_SURFACE_ENTHALPY_OWNER_V1_SCHEMA_SHA256
            || self.exact_carry_definition_sha256.as_str()
                != LSE_SURFACE_ENTHALPY_EXACT_CARRY_V1_DEFINITION_SHA256
            || self.transaction_id.0 == 0
            || self.support_start_ns >= self.support_end_ns
            || self.beginning_owner_state_sha256 != beginning.state_sha256
            || self.ending_owner_state_sha256 != ending.state_sha256
            || self.predecessor_receipt_chain_sha256 != beginning.receipt_chain_sha256
            || ending.receipt_chain_sha256 != self.receipt_sha256
            || self.receipt_sha256 != self.recomputed_sha256()?
        {
            return Err(LseSurfaceEnthalpyErrorV1::Identity(
                "exact-surface credit receipt seal",
            ));
        }
        validate_owner_lineage(beginning, ending, self)?;
        validate_operand_structure(
            &self.accepted_operands,
            beginning.records(),
            self.transaction_id,
            self.predecessor_transaction_id,
            self.support_start_ns,
            self.support_end_ns,
            None,
        )?;
        validate_exact_reconstruction(beginning, ending, &self.accepted_operands)?;
        Ok(())
    }

    /// Replay the accepted physical operand set without trusting the receipt's
    /// own copy, then reconstruct every ending exact total.
    pub fn validate_independent(
        &self,
        beginning: &LseSurfaceEnthalpyOwnerEnvelopeV1,
        ending: &LseSurfaceEnthalpyOwnerEnvelopeV1,
        expected_operands: &[LseSurfaceEnthalpyAcceptedEnergyOperandV1],
    ) -> Result<(), LseSurfaceEnthalpyErrorV1> {
        self.validate(beginning, ending)?;
        if !operands_bit_identical(&self.accepted_operands, expected_operands) {
            return Err(LseSurfaceEnthalpyErrorV1::Identity(
                "independent accepted operand replay",
            ));
        }
        validate_exact_reconstruction(beginning, ending, expected_operands)
    }
}

fn operands_bit_identical(
    left: &[LseSurfaceEnthalpyAcceptedEnergyOperandV1],
    right: &[LseSurfaceEnthalpyAcceptedEnergyOperandV1],
) -> bool {
    left.len() == right.len()
        && left.iter().zip(right).all(|(left, right)| {
            left.surface_key == right.surface_key
                && left.kind == right.kind
                && left.ordinal == right.ordinal
                && left.source_owner_id == right.source_owner_id
                && left.source_receipt_sha256 == right.source_receipt_sha256
                && left.transaction_id == right.transaction_id
                && left.predecessor_transaction_id == right.predecessor_transaction_id
                && left.support_start_ns == right.support_start_ns
                && left.support_end_ns == right.support_end_ns
                && left.units == right.units
                && left.basis == right.basis
                && left.energy_j_m2_tile_ground.to_bits() == right.energy_j_m2_tile_ground.to_bits()
        })
}

fn validate_owner_lineage(
    beginning: &LseSurfaceEnthalpyOwnerEnvelopeV1,
    ending: &LseSurfaceEnthalpyOwnerEnvelopeV1,
    receipt: &LseSurfaceEnthalpyEnergyCreditReceiptV1,
) -> Result<(), LseSurfaceEnthalpyErrorV1> {
    if beginning.owner_tag != ending.owner_tag
        || beginning.schema_sha256 != ending.schema_sha256
        || beginning.exact_carry_definition_sha256 != ending.exact_carry_definition_sha256
        || beginning.owner_id != ending.owner_id
        || beginning.run_id != ending.run_id
        || beginning.configuration_sha256 != ending.configuration_sha256
        || beginning.records.len() != ending.records.len()
        || beginning
            .records
            .iter()
            .zip(&ending.records)
            .any(|(left, right)| {
                left.surface_key != right.surface_key
                    || left.last_accepted_transaction_id != receipt.predecessor_transaction_id
                    || right.last_accepted_transaction_id != Some(receipt.transaction_id)
            })
    {
        return Err(LseSurfaceEnthalpyErrorV1::Identity(
            "beginning/ending exact-owner lineage",
        ));
    }
    Ok(())
}

#[allow(clippy::too_many_arguments)]
#[allow(clippy::too_many_lines)]
fn validate_operand_structure(
    operands: &[LseSurfaceEnthalpyAcceptedEnergyOperandV1],
    records: &[LseSurfaceEnthalpyStateRecordV1],
    transaction_id: TransactionId,
    predecessor_transaction_id: Option<TransactionId>,
    support_start_ns: u128,
    support_end_ns: u128,
    expected_source_owner_id: Option<&ResourceOwnerId>,
) -> Result<(), LseSurfaceEnthalpyErrorV1> {
    if records.is_empty() || support_start_ns >= support_end_ns {
        return Err(LseSurfaceEnthalpyErrorV1::Identity(
            "operand support or owner cardinality",
        ));
    }
    let mut previous = None;
    let mut common_source_owner = None;
    let mut retained_ordinals = Vec::new();
    for operand in operands {
        let identity = (&operand.surface_key, operand.kind, operand.ordinal);
        if previous.as_ref().is_some_and(|prior| prior >= &identity)
            || operand.transaction_id != transaction_id
            || operand.predecessor_transaction_id != predecessor_transaction_id
            || operand.support_start_ns != support_start_ns
            || operand.support_end_ns != support_end_ns
            || operand.units != "J m^-2 tile-ground"
            || operand.basis != "tile_ground"
            || !operand.energy_j_m2_tile_ground.is_finite()
            || operand.source_receipt_sha256.as_str() == ZERO_SHA256
            || !records
                .iter()
                .any(|record| record.surface_key == operand.surface_key)
            || expected_source_owner_id.is_some_and(|expected| expected != &operand.source_owner_id)
        {
            return Err(LseSurfaceEnthalpyErrorV1::Identity(
                "exact-surface credit operand seal",
            ));
        }
        if common_source_owner
            .as_ref()
            .is_some_and(|owner| owner != &operand.source_owner_id)
        {
            return Err(LseSurfaceEnthalpyErrorV1::Identity(
                "mixed exact-surface operand source owner",
            ));
        }
        common_source_owner.get_or_insert_with(|| operand.source_owner_id.clone());
        if operand.kind == LseSurfaceEnthalpyEnergyOperandKindV1::RetainedIngressTileCredit {
            retained_ordinals.push(operand.ordinal);
        }
        previous = Some(identity);
    }
    if retained_ordinals
        .iter()
        .copied()
        .enumerate()
        .any(|(expected, observed)| u32::try_from(expected) != Ok(observed))
    {
        return Err(LseSurfaceEnthalpyErrorV1::Cardinality(
            "retained-ingress operand ordinals",
        ));
    }
    for record in records {
        let per_key = operands
            .iter()
            .filter(|operand| operand.surface_key == record.surface_key)
            .collect::<Vec<_>>();
        let phase = per_key
            .iter()
            .copied()
            .filter(|operand| {
                operand.kind == LseSurfaceEnthalpyEnergyOperandKindV1::PhaseFreeSurfaceEnergy
            })
            .collect::<Vec<_>>();
        let fusion = per_key
            .iter()
            .copied()
            .filter(|operand| {
                operand.kind == LseSurfaceEnthalpyEnergyOperandKindV1::LitterFusionEnergy
            })
            .collect::<Vec<_>>();
        let retained_count = per_key
            .iter()
            .filter(|operand| {
                operand.kind == LseSurfaceEnthalpyEnergyOperandKindV1::RetainedIngressTileCredit
            })
            .count();
        if record.surface_key.surface_class
            == openwepp_land_surface_energy::SurfaceClass::ForestLitter
        {
            if phase.len() != 6
                || phase
                    .iter()
                    .enumerate()
                    .any(|(ordinal, operand)| u32::try_from(ordinal) != Ok(operand.ordinal))
            {
                return Err(LseSurfaceEnthalpyErrorV1::Cardinality(
                    "exhaustive phase-free operand ordinals",
                ));
            }
            if fusion.len() != 1 || fusion[0].ordinal != 0 {
                return Err(LseSurfaceEnthalpyErrorV1::Cardinality(
                    "exactly one fusion operand",
                ));
            }
            if phase
                .iter()
                .chain(fusion.iter())
                .any(|operand| operand.source_receipt_sha256 != phase[0].source_receipt_sha256)
            {
                return Err(LseSurfaceEnthalpyErrorV1::Identity(
                    "phase/fusion source receipt join",
                ));
            }
        } else if !phase.is_empty() || !fusion.is_empty() {
            return Err(LseSurfaceEnthalpyErrorV1::Identity(
                "litter-only phase/fusion operands",
            ));
        }
        if retained_count > 1 {
            return Err(LseSurfaceEnthalpyErrorV1::Cardinality(
                "retained-ingress operand cardinality per surface",
            ));
        }
    }
    Ok(())
}

fn validate_exact_reconstruction(
    beginning: &LseSurfaceEnthalpyOwnerEnvelopeV1,
    ending: &LseSurfaceEnthalpyOwnerEnvelopeV1,
    operands: &[LseSurfaceEnthalpyAcceptedEnergyOperandV1],
) -> Result<(), LseSurfaceEnthalpyErrorV1> {
    for (beginning_record, ending_record) in beginning.records.iter().zip(&ending.records) {
        let values = operands
            .iter()
            .filter(|operand| operand.surface_key == beginning_record.surface_key)
            .map(|operand| operand.energy_j_m2_tile_ground)
            .collect::<Vec<_>>();
        let (expected_high, expected_carry) = if values.iter().all(|value| *value == 0.0) {
            (
                beginning_record.enthalpy_hi_j_m2_tile,
                beginning_record.enthalpy_carry.clone(),
            )
        } else {
            ExactDyadicEnthalpy::exact_sum_binary64(
                beginning_record.enthalpy_hi_j_m2_tile,
                &beginning_record.enthalpy_carry,
                &values,
            )?
            .rounded_high_and_remainder()?
        };
        if ending_record.enthalpy_hi_j_m2_tile.to_bits() != expected_high.to_bits()
            || ending_record.enthalpy_carry != expected_carry
        {
            return Err(LseSurfaceEnthalpyErrorV1::Reconstruction);
        }
    }
    Ok(())
}

impl LseSurfaceEnthalpyOwnerRestartV1 {
    pub fn validate(&self) -> Result<(), LseSurfaceEnthalpyErrorV1> {
        self.owner.validate()?;
        let mut value = self.clone();
        value.restart_sha256 = wire_digest(ZERO_SHA256)?;
        if self.restart_sha256 != digest(&value)? {
            return Err(LseSurfaceEnthalpyErrorV1::Identity("restart digest"));
        }
        Ok(())
    }

    pub fn canonical_bytes(&self) -> Result<Vec<u8>, LseSurfaceEnthalpyErrorV1> {
        self.validate()?;
        serde_json::to_vec(self)
            .map_err(|error| LseSurfaceEnthalpyErrorV1::Serialization(error.to_string()))
    }

    pub fn from_canonical_bytes(bytes: &[u8]) -> Result<Self, LseSurfaceEnthalpyErrorV1> {
        let value: Self = serde_json::from_slice(bytes)
            .map_err(|error| LseSurfaceEnthalpyErrorV1::Serialization(error.to_string()))?;
        if value.canonical_bytes()? != bytes {
            return Err(LseSurfaceEnthalpyErrorV1::Serialization(
                "noncanonical restart bytes".to_owned(),
            ));
        }
        Ok(value)
    }
}

impl LseSurfaceEnthalpyOwnerCheckpointV1 {
    pub fn validate(&self) -> Result<(), LseSurfaceEnthalpyErrorV1> {
        self.owner.validate()?;
        if let Some(receipt) = &self.receipt {
            if receipt.ending_owner_state_sha256 != self.owner.state_sha256
                || receipt.receipt_sha256 != self.owner.receipt_chain_sha256
            {
                return Err(LseSurfaceEnthalpyErrorV1::Identity(
                    "checkpoint owner/receipt join",
                ));
            }
            if receipt.receipt_sha256 != receipt.recomputed_sha256()? {
                return Err(LseSurfaceEnthalpyErrorV1::Identity(
                    "checkpoint receipt digest",
                ));
            }
        }
        let mut value = self.clone();
        value.checkpoint_sha256 = wire_digest(ZERO_SHA256)?;
        if self.checkpoint_sha256 != digest(&value)? {
            return Err(LseSurfaceEnthalpyErrorV1::Identity("checkpoint digest"));
        }
        Ok(())
    }

    pub fn canonical_bytes(&self) -> Result<Vec<u8>, LseSurfaceEnthalpyErrorV1> {
        self.validate()?;
        serde_json::to_vec(self)
            .map_err(|error| LseSurfaceEnthalpyErrorV1::Serialization(error.to_string()))
    }

    pub fn from_canonical_bytes(bytes: &[u8]) -> Result<Self, LseSurfaceEnthalpyErrorV1> {
        let value: Self = serde_json::from_slice(bytes)
            .map_err(|error| LseSurfaceEnthalpyErrorV1::Serialization(error.to_string()))?;
        if value.canonical_bytes()? != bytes {
            return Err(LseSurfaceEnthalpyErrorV1::Serialization(
                "noncanonical checkpoint bytes".to_owned(),
            ));
        }
        Ok(value)
    }
}

fn digest<T: Serialize>(value: &T) -> Result<Sha256Digest, LseSurfaceEnthalpyErrorV1> {
    let bytes = serde_json::to_vec(value)
        .map_err(|error| LseSurfaceEnthalpyErrorV1::Serialization(error.to_string()))?;
    Sha256Digest::try_new(format!("{:x}", Sha256::digest(bytes)))
        .map_err(|error| LseSurfaceEnthalpyErrorV1::Serialization(error.to_string()))
}

fn wire_digest(value: &str) -> Result<Sha256Digest, LseSurfaceEnthalpyErrorV1> {
    Sha256Digest::try_new(value)
        .map_err(|error| LseSurfaceEnthalpyErrorV1::Serialization(error.to_string()))
}

impl LseSurfaceEnthalpyOwnerEnvelopeV1 {
    pub fn validate_frozen_parent_join(
        &self,
        lse_configuration: &LandSurfaceEnergyConfiguration,
        lse_v3: &LandSurfaceEnergyV3State,
        surface_configuration: &SurfaceLiquidConfigurationV2,
        surface_v2: &SurfaceLiquidOwnerEnvelopeV2,
    ) -> Result<(), LseSurfaceEnthalpyErrorV1> {
        self.validate()?;
        lse_v3
            .validate(lse_configuration)
            .map_err(|_| LseSurfaceEnthalpyErrorV1::Identity("frozen LSE V3 state"))?;
        surface_v2
            .canonical_bytes(surface_configuration.parent(), Some(surface_configuration))
            .map_err(|_| LseSurfaceEnthalpyErrorV1::Identity("frozen surface-owner V2"))?;
        let surface_state = surface_v2
            .v2_state()
            .ok_or(LseSurfaceEnthalpyErrorV1::Identity(
                "surface owner is not V2",
            ))?;
        if self.run_id != surface_configuration.parent().run_id.to_string()
            || self.configuration_sha256.as_str() != surface_configuration.configuration_sha256()
            || self.frozen_lse_v3_state_sha256 != lse_v3.0.state_sha256
            || self.frozen_surface_owner_v2_sha256.as_str() != surface_v2.envelope_sha256()
            || self.records.len() != lse_v3.0.tiles.len()
            || self.records.len() != surface_state.records().len()
        {
            return Err(LseSurfaceEnthalpyErrorV1::Identity(
                "exact owner frozen-parent identity",
            ));
        }
        for exact in &self.records {
            let surface = surface_state
                .records()
                .iter()
                .find(|record| record.key == exact.surface_key)
                .ok_or(LseSurfaceEnthalpyErrorV1::Cardinality("surface mirror key"))?;
            let lse = lse_v3
                .0
                .tiles
                .iter()
                .find(|tile| {
                    tile.ofe_id == exact.surface_key.ofe_id
                        && tile.tile_id == exact.surface_key.tile_id
                })
                .ok_or(LseSurfaceEnthalpyErrorV1::Cardinality("LSE mirror key"))?;
            if exact.enthalpy_hi_j_m2_tile.to_bits() != surface.surface_enthalpy_j_m2_tile.to_bits()
                || exact.enthalpy_hi_j_m2_tile.to_bits()
                    != lse.surface_enthalpy_j_m2_tile_ground.to_bits()
                || exact.last_accepted_transaction_id != surface.last_accepted_transaction_id
                || exact.last_accepted_transaction_id != lse_v3.0.last_accepted_transaction_id
            {
                return Err(LseSurfaceEnthalpyErrorV1::Identity(
                    "exact owner frozen high mirrors or predecessor",
                ));
            }
        }
        Ok(())
    }

    pub fn adopt_from_frozen_v2_v3(
        owner_id: ResourceOwnerId,
        lse_configuration: &LandSurfaceEnergyConfiguration,
        lse_v3: &LandSurfaceEnergyV3State,
        surface_configuration: &SurfaceLiquidConfigurationV2,
        surface_v2: &SurfaceLiquidOwnerEnvelopeV2,
    ) -> Result<Self, LseSurfaceEnthalpyErrorV1> {
        lse_v3
            .validate(lse_configuration)
            .map_err(|_| LseSurfaceEnthalpyErrorV1::Identity("frozen LSE V3 state"))?;
        surface_v2
            .canonical_bytes(surface_configuration.parent(), Some(surface_configuration))
            .map_err(|_| LseSurfaceEnthalpyErrorV1::Identity("frozen surface-owner V2"))?;
        let surface_state = surface_v2
            .v2_state()
            .ok_or(LseSurfaceEnthalpyErrorV1::Identity(
                "surface owner is not V2",
            ))?;
        if owner_id == lse_v3.0.owner_id || owner_id == surface_configuration.parent().owner_id {
            return Err(LseSurfaceEnthalpyErrorV1::Identity(
                "exact owner aliases a parent",
            ));
        }
        let mut records = Vec::with_capacity(surface_state.records().len());
        for surface in surface_state.records() {
            let lse = lse_v3
                .0
                .tiles
                .iter()
                .find(|tile| {
                    tile.ofe_id == surface.key.ofe_id && tile.tile_id == surface.key.tile_id
                })
                .ok_or(LseSurfaceEnthalpyErrorV1::Cardinality(
                    "LSE/surface key set",
                ))?;
            if lse.surface_enthalpy_j_m2_tile_ground.to_bits()
                != surface.surface_enthalpy_j_m2_tile.to_bits()
                || !surface.surface_enthalpy_j_m2_tile.is_finite()
            {
                return Err(LseSurfaceEnthalpyErrorV1::Identity("frozen high mirrors"));
            }
            records.push(LseSurfaceEnthalpyStateRecordV1 {
                surface_key: surface.key.clone(),
                enthalpy_hi_j_m2_tile: surface.surface_enthalpy_j_m2_tile,
                enthalpy_carry: ExactDyadicEnthalpy::zero(),
                last_accepted_transaction_id: surface.last_accepted_transaction_id,
            });
        }
        if records.len() != lse_v3.0.tiles.len() {
            return Err(LseSurfaceEnthalpyErrorV1::Cardinality(
                "complete LSE key set",
            ));
        }
        let zero = wire_digest(ZERO_SHA256)?;
        let mut owner = Self {
            owner_tag: LSE_SURFACE_ENTHALPY_OWNER_V1_TAG.to_owned(),
            schema_sha256: wire_digest(LSE_SURFACE_ENTHALPY_OWNER_V1_SCHEMA_SHA256)?,
            exact_carry_definition_sha256: wire_digest(
                LSE_SURFACE_ENTHALPY_EXACT_CARRY_V1_DEFINITION_SHA256,
            )?,
            owner_id,
            run_id: surface_configuration.parent().run_id.to_string(),
            configuration_sha256: wire_digest(surface_configuration.configuration_sha256())?,
            frozen_lse_v3_state_sha256: lse_v3.0.state_sha256.clone(),
            frozen_surface_owner_v2_sha256: wire_digest(surface_v2.envelope_sha256())?,
            state_sha256: zero.clone(),
            receipt_chain_sha256: zero,
            records,
        };
        owner.state_sha256 = owner.recomputed_state_sha256()?;
        owner.validate_frozen_parent_join(
            lse_configuration,
            lse_v3,
            surface_configuration,
            surface_v2,
        )?;
        Ok(owner)
    }

    fn recomputed_state_sha256(&self) -> Result<Sha256Digest, LseSurfaceEnthalpyErrorV1> {
        let mut value = self.clone();
        value.state_sha256 = wire_digest(ZERO_SHA256)?;
        // The scientific state digest and the receipt-chain digest are
        // adjacent owner identities. Keeping the chain outside this preimage
        // avoids an impossible cryptographic fixed point while the complete
        // canonical envelope still binds both.
        value.receipt_chain_sha256 = wire_digest(ZERO_SHA256)?;
        digest(&value)
    }

    pub fn validate(&self) -> Result<(), LseSurfaceEnthalpyErrorV1> {
        if self.owner_tag != LSE_SURFACE_ENTHALPY_OWNER_V1_TAG
            || self.schema_sha256.as_str() != LSE_SURFACE_ENTHALPY_OWNER_V1_SCHEMA_SHA256
            || self.exact_carry_definition_sha256.as_str()
                != LSE_SURFACE_ENTHALPY_EXACT_CARRY_V1_DEFINITION_SHA256
            || self.records.is_empty()
        {
            return Err(LseSurfaceEnthalpyErrorV1::Identity(
                "owner schema or cardinality",
            ));
        }
        let mut previous = None;
        for record in &self.records {
            if !record.enthalpy_hi_j_m2_tile.is_finite() {
                return Err(LseSurfaceEnthalpyErrorV1::Domain("nonfinite high mirror"));
            }
            record.enthalpy_carry.validate()?;
            if previous
                .as_ref()
                .is_some_and(|key| key >= &record.surface_key)
            {
                return Err(LseSurfaceEnthalpyErrorV1::Cardinality(
                    "unordered surface keys",
                ));
            }
            previous = Some(record.surface_key.clone());
        }
        if self.state_sha256 != self.recomputed_state_sha256()? {
            return Err(LseSurfaceEnthalpyErrorV1::Identity("owner state digest"));
        }
        Ok(())
    }

    pub fn canonical_bytes(&self) -> Result<Vec<u8>, LseSurfaceEnthalpyErrorV1> {
        self.validate()?;
        serde_json::to_vec(self)
            .map_err(|error| LseSurfaceEnthalpyErrorV1::Serialization(error.to_string()))
    }

    pub fn from_canonical_bytes(bytes: &[u8]) -> Result<Self, LseSurfaceEnthalpyErrorV1> {
        let value: Self = serde_json::from_slice(bytes)
            .map_err(|error| LseSurfaceEnthalpyErrorV1::Serialization(error.to_string()))?;
        if value.canonical_bytes()? != bytes {
            return Err(LseSurfaceEnthalpyErrorV1::Serialization(
                "noncanonical owner bytes".to_owned(),
            ));
        }
        Ok(value)
    }

    #[must_use]
    pub fn records(&self) -> &[LseSurfaceEnthalpyStateRecordV1] {
        &self.records
    }

    #[allow(clippy::too_many_arguments, clippy::too_many_lines)]
    pub fn advance_exact(
        &self,
        lse_v3_candidate: &LandSurfaceEnergyV3State,
        surface_configuration: &SurfaceLiquidConfigurationV2,
        surface_v2_candidate: &SurfaceLiquidOwnerEnvelopeV2,
        transaction_id: TransactionId,
        predecessor_transaction_id: Option<TransactionId>,
        support_start_ns: u128,
        support_end_ns: u128,
        expected_operands: &[LseSurfaceEnthalpyAcceptedEnergyOperandV1],
        accepted_operands: Vec<LseSurfaceEnthalpyAcceptedEnergyOperandV1>,
    ) -> Result<LseSurfaceEnthalpyAcceptedCandidateV1, LseSurfaceEnthalpyErrorV1> {
        self.validate()?;
        if transaction_id.0 == 0 || support_start_ns >= support_end_ns {
            return Err(LseSurfaceEnthalpyErrorV1::Identity(
                "transaction or support",
            ));
        }
        let current_predecessor = self
            .records
            .first()
            .and_then(|record| record.last_accepted_transaction_id);
        if current_predecessor != predecessor_transaction_id
            || self
                .records
                .iter()
                .any(|record| record.last_accepted_transaction_id != predecessor_transaction_id)
        {
            return Err(LseSurfaceEnthalpyErrorV1::Identity(
                "predecessor transaction",
            ));
        }
        surface_v2_candidate
            .canonical_bytes(surface_configuration.parent(), Some(surface_configuration))
            .map_err(|_| LseSurfaceEnthalpyErrorV1::Identity("candidate surface V2"))?;
        let surface_state =
            surface_v2_candidate
                .v2_state()
                .ok_or(LseSurfaceEnthalpyErrorV1::Identity(
                    "candidate surface owner is not V2",
                ))?;
        validate_operand_structure(
            expected_operands,
            &self.records,
            transaction_id,
            predecessor_transaction_id,
            support_start_ns,
            support_end_ns,
            Some(&surface_configuration.parent().owner_id),
        )?;
        if !operands_bit_identical(&accepted_operands, expected_operands) {
            return Err(LseSurfaceEnthalpyErrorV1::Identity(
                "accepted operands do not match independently reconstructed physical receipts",
            ));
        }
        validate_operand_structure(
            &accepted_operands,
            &self.records,
            transaction_id,
            predecessor_transaction_id,
            support_start_ns,
            support_end_ns,
            Some(&surface_configuration.parent().owner_id),
        )?;
        let mut grouped = BTreeMap::<DirectSurfaceLiquidStoreKey, Vec<f64>>::new();
        for operand in &accepted_operands {
            grouped
                .entry(operand.surface_key.clone())
                .or_default()
                .push(operand.energy_j_m2_tile_ground);
        }
        let mut records = Vec::with_capacity(self.records.len());
        for beginning in &self.records {
            let values = grouped.remove(&beginning.surface_key).unwrap_or_default();
            let (high, carry) = if values.iter().all(|value| *value == 0.0) {
                (
                    beginning.enthalpy_hi_j_m2_tile,
                    beginning.enthalpy_carry.clone(),
                )
            } else {
                ExactDyadicEnthalpy::exact_sum_binary64(
                    beginning.enthalpy_hi_j_m2_tile,
                    &beginning.enthalpy_carry,
                    &values,
                )?
                .rounded_high_and_remainder()?
            };
            let surface = surface_state
                .records()
                .iter()
                .find(|record| record.key == beginning.surface_key)
                .ok_or(LseSurfaceEnthalpyErrorV1::Cardinality("surface mirror key"))?;
            let lse = lse_v3_candidate
                .0
                .tiles
                .iter()
                .find(|tile| {
                    tile.ofe_id == beginning.surface_key.ofe_id
                        && tile.tile_id == beginning.surface_key.tile_id
                })
                .ok_or(LseSurfaceEnthalpyErrorV1::Cardinality("LSE mirror key"))?;
            if surface.surface_enthalpy_j_m2_tile.to_bits() != high.to_bits()
                || lse.surface_enthalpy_j_m2_tile_ground.to_bits() != high.to_bits()
            {
                return Err(LseSurfaceEnthalpyErrorV1::Identity(
                    "candidate high mirrors",
                ));
            }
            records.push(LseSurfaceEnthalpyStateRecordV1 {
                surface_key: beginning.surface_key.clone(),
                enthalpy_hi_j_m2_tile: high,
                enthalpy_carry: carry,
                last_accepted_transaction_id: Some(transaction_id),
            });
        }
        if !grouped.is_empty() {
            return Err(LseSurfaceEnthalpyErrorV1::Cardinality(
                "foreign operand key",
            ));
        }
        let beginning_state_sha256 = self.state_sha256.clone();
        let predecessor_receipt_chain_sha256 = self.receipt_chain_sha256.clone();
        let mut ending = Self {
            owner_tag: self.owner_tag.clone(),
            schema_sha256: self.schema_sha256.clone(),
            exact_carry_definition_sha256: self.exact_carry_definition_sha256.clone(),
            owner_id: self.owner_id.clone(),
            run_id: self.run_id.clone(),
            configuration_sha256: self.configuration_sha256.clone(),
            frozen_lse_v3_state_sha256: lse_v3_candidate.0.state_sha256.clone(),
            frozen_surface_owner_v2_sha256: wire_digest(surface_v2_candidate.envelope_sha256())?,
            state_sha256: wire_digest(ZERO_SHA256)?,
            receipt_chain_sha256: predecessor_receipt_chain_sha256.clone(),
            records,
        };
        ending.state_sha256 = ending.recomputed_state_sha256()?;
        let mut receipt = LseSurfaceEnthalpyEnergyCreditReceiptV1 {
            receipt_tag: LSE_SURFACE_ENTHALPY_ENERGY_CREDIT_RECEIPT_V1_TAG.to_owned(),
            schema_sha256: self.schema_sha256.clone(),
            exact_carry_definition_sha256: self.exact_carry_definition_sha256.clone(),
            transaction_id,
            predecessor_transaction_id,
            support_start_ns,
            support_end_ns,
            beginning_owner_state_sha256: beginning_state_sha256,
            ending_owner_state_sha256: ending.state_sha256.clone(),
            predecessor_receipt_chain_sha256,
            accepted_operands,
            receipt_sha256: wire_digest(ZERO_SHA256)?,
        };
        receipt.receipt_sha256 = digest(&receipt)?;
        ending.receipt_chain_sha256 = receipt.receipt_sha256.clone();
        ending.validate()?;
        receipt.validate(self, &ending)?;
        receipt.validate_independent(self, &ending, expected_operands)?;
        Ok(LseSurfaceEnthalpyAcceptedCandidateV1 {
            ending_owner: ending,
            receipt,
        })
    }

    pub fn restart(&self) -> Result<LseSurfaceEnthalpyOwnerRestartV1, LseSurfaceEnthalpyErrorV1> {
        self.validate()?;
        let mut restart = LseSurfaceEnthalpyOwnerRestartV1 {
            owner: self.clone(),
            restart_sha256: wire_digest(ZERO_SHA256)?,
        };
        restart.restart_sha256 = digest(&restart)?;
        restart.validate()?;
        Ok(restart)
    }

    pub fn checkpoint(
        &self,
        receipt: Option<LseSurfaceEnthalpyEnergyCreditReceiptV1>,
    ) -> Result<LseSurfaceEnthalpyOwnerCheckpointV1, LseSurfaceEnthalpyErrorV1> {
        self.validate()?;
        let mut checkpoint = LseSurfaceEnthalpyOwnerCheckpointV1 {
            owner: self.clone(),
            receipt,
            checkpoint_sha256: wire_digest(ZERO_SHA256)?,
        };
        checkpoint.checkpoint_sha256 = digest(&checkpoint)?;
        checkpoint.validate()?;
        Ok(checkpoint)
    }
}

pub fn refuse_lse_surface_enthalpy_v1_downgrade() -> Result<(), LseSurfaceEnthalpyErrorV1> {
    Err(LseSurfaceEnthalpyErrorV1::DowngradeProhibited)
}
