//! Exact signed-dyadic energy arithmetic for soil-thermal V2 custody.
//!
//! The wire value is `sign * coefficient * 2^exponent2`. Arithmetic is
//! dependency-free and integer-exact; conversion back to binary64 performs one
//! round-to-nearest, ties-to-even operation.

#![allow(clippy::missing_errors_doc)]

use core::{cmp::Ordering, fmt};

use openwepp_kernel_contract::{ResourceOwnerId, SoilLayerId, TransactionId};
use serde::{Deserialize, Deserializer, Serialize};

use crate::{
    EXACT_DYADIC_ENTHALPY_V1_DEFINITION_SHA256, OfeId, SOIL_THERMAL_OWNER_V2_SCHEMA_SHA256,
    Sha256Digest, SoilThermalOwnerEnvelopeV2, canonical_digest,
};

const MAX_WIRE_HEX_DIGITS: usize = 1_048_576;
const MAX_WIRE_EXPONENT_MAGNITUDE: i32 = 16_777_216;

/// `LSEB-E-049` exact-carry schema/domain/overflow refusal.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum ExactDyadicEnthalpyError {
    NonFiniteBinary64,
    NonCanonicalWire(&'static str),
    CoefficientResourceLimit,
    ExponentOutOfRange,
    Binary64Overflow,
}

impl fmt::Display for ExactDyadicEnthalpyError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str("LSEB-E-049: ")?;
        match self {
            Self::NonFiniteBinary64 => formatter.write_str("nonfinite binary64 exact operand"),
            Self::NonCanonicalWire(detail) => {
                write!(formatter, "noncanonical exact-dyadic wire: {detail}")
            }
            Self::CoefficientResourceLimit => {
                formatter.write_str("exact-dyadic coefficient resource limit")
            }
            Self::ExponentOutOfRange => formatter.write_str("exact-dyadic exponent out of range"),
            Self::Binary64Overflow => formatter.write_str("exact total rounds outside binary64"),
        }
    }
}

impl std::error::Error for ExactDyadicEnthalpyError {}

/// Typed V2 owner/receipt refusal family required by `INV-LANDSURFACEENERGY-150`.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum SoilThermalExactCarryError {
    Exact(ExactDyadicEnthalpyError),
    Identity(&'static str),
    Domain(&'static str),
    Cardinality(&'static str),
    Receipt(&'static str),
    Reconstruction,
    DowngradeProhibited,
    Serialization(String),
}

impl fmt::Display for SoilThermalExactCarryError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str("LSEB-E-049: ")?;
        match self {
            Self::Exact(error) => write!(formatter, "{error}"),
            Self::Identity(detail) => write!(formatter, "identity mismatch: {detail}"),
            Self::Domain(detail) => write!(formatter, "domain refusal: {detail}"),
            Self::Cardinality(detail) => write!(formatter, "cardinality refusal: {detail}"),
            Self::Receipt(detail) => write!(formatter, "receipt refusal: {detail}"),
            Self::Reconstruction => formatter.write_str("exact reconstruction mismatch"),
            Self::DowngradeProhibited => formatter.write_str("production V2-to-V1 downgrade"),
            Self::Serialization(detail) => write!(formatter, "serialization refusal: {detail}"),
        }
    }
}

impl std::error::Error for SoilThermalExactCarryError {}

impl From<ExactDyadicEnthalpyError> for SoilThermalExactCarryError {
    fn from(error: ExactDyadicEnthalpyError) -> Self {
        Self::Exact(error)
    }
}

/// Canonical normalized signed-dyadic energy wire form.
#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct ExactDyadicEnthalpy {
    pub sign: i8,
    pub coefficient_hex: String,
    pub exponent2: i32,
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct ExactDyadicWire {
    sign: i8,
    coefficient_hex: String,
    exponent2: i32,
}

impl<'de> Deserialize<'de> for ExactDyadicEnthalpy {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let wire = ExactDyadicWire::deserialize(deserializer)?;
        Self::try_new(wire.sign, wire.coefficient_hex, wire.exponent2)
            .map_err(serde::de::Error::custom)
    }
}

impl ExactDyadicEnthalpy {
    #[must_use]
    pub fn zero() -> Self {
        Self {
            sign: 0,
            coefficient_hex: "0".to_owned(),
            exponent2: 0,
        }
    }

    pub fn try_new(
        sign: i8,
        coefficient_hex: impl Into<String>,
        exponent2: i32,
    ) -> Result<Self, ExactDyadicEnthalpyError> {
        let value = Self {
            sign,
            coefficient_hex: coefficient_hex.into(),
            exponent2,
        };
        value.validate()?;
        Ok(value)
    }

    pub fn validate(&self) -> Result<(), ExactDyadicEnthalpyError> {
        if self.coefficient_hex.len() > MAX_WIRE_HEX_DIGITS {
            return Err(ExactDyadicEnthalpyError::CoefficientResourceLimit);
        }
        if self.exponent2.unsigned_abs() > MAX_WIRE_EXPONENT_MAGNITUDE as u32 {
            return Err(ExactDyadicEnthalpyError::ExponentOutOfRange);
        }
        if self.sign == 0 {
            return if self.coefficient_hex == "0" && self.exponent2 == 0 {
                Ok(())
            } else {
                Err(ExactDyadicEnthalpyError::NonCanonicalWire(
                    "zero is exactly (0,\"0\",0)",
                ))
            };
        }
        if !matches!(self.sign, -1 | 1) {
            return Err(ExactDyadicEnthalpyError::NonCanonicalWire(
                "nonzero sign must be -1 or 1",
            ));
        }
        let bytes = self.coefficient_hex.as_bytes();
        if bytes.is_empty() || bytes[0] == b'0' {
            return Err(ExactDyadicEnthalpyError::NonCanonicalWire(
                "nonzero coefficient is positive without a leading zero",
            ));
        }
        if !bytes
            .iter()
            .all(|byte| byte.is_ascii_digit() || (b'a'..=b'f').contains(byte))
        {
            return Err(ExactDyadicEnthalpyError::NonCanonicalWire(
                "coefficient must be lowercase hexadecimal",
            ));
        }
        let last = *bytes
            .last()
            .ok_or(ExactDyadicEnthalpyError::NonCanonicalWire(
                "empty coefficient",
            ))?;
        if !matches!(last, b'1' | b'3' | b'5' | b'7' | b'9' | b'b' | b'd' | b'f') {
            return Err(ExactDyadicEnthalpyError::NonCanonicalWire(
                "nonzero coefficient must be odd",
            ));
        }
        Ok(())
    }

    pub fn from_f64(value: f64) -> Result<Self, ExactDyadicEnthalpyError> {
        Dyadic::from_f64(value).map(Dyadic::into_wire)
    }

    pub fn exact_sum<'a>(
        values: impl IntoIterator<Item = &'a Self>,
    ) -> Result<Self, ExactDyadicEnthalpyError> {
        let mut total = Dyadic::zero();
        for value in values {
            total = total.add(&Dyadic::from_wire(value)?);
        }
        Ok(total.into_wire())
    }

    pub fn exact_sum_binary64(
        beginning_high: f64,
        beginning_carry: &Self,
        operands: &[f64],
    ) -> Result<Self, ExactDyadicEnthalpyError> {
        let mut total = Dyadic::from_f64(beginning_high)?.add(&Dyadic::from_wire(beginning_carry)?);
        for operand in operands {
            total = total.add(&Dyadic::from_f64(*operand)?);
        }
        Ok(total.into_wire())
    }

    pub fn round_to_f64(&self) -> Result<f64, ExactDyadicEnthalpyError> {
        Dyadic::from_wire(self)?.round_to_f64()
    }

    pub fn rounded_high_and_remainder(&self) -> Result<(f64, Self), ExactDyadicEnthalpyError> {
        let total = Dyadic::from_wire(self)?;
        let high = total.round_to_f64()?;
        let remainder = total.add(&Dyadic::from_f64(-high)?).into_wire();
        Ok((high, remainder))
    }
}

/// Correctly rounded projection of the unchanged layer equation
/// `T_end = T_begin + (E_end - E_begin) / C`.
pub fn project_soil_temperature_k(
    beginning_temperature_k: f64,
    heat_capacity_j_m2_k: f64,
    beginning_enthalpy_hi_j_m2_ofe_ground: f64,
    beginning_enthalpy_carry: &ExactDyadicEnthalpy,
    ending_enthalpy_hi_j_m2_ofe_ground: f64,
    ending_enthalpy_carry: &ExactDyadicEnthalpy,
) -> Result<f64, SoilThermalExactCarryError> {
    if !beginning_temperature_k.is_finite()
        || !(200.0..=350.0).contains(&beginning_temperature_k)
        || !heat_capacity_j_m2_k.is_finite()
        || heat_capacity_j_m2_k <= 0.0
    {
        return Err(SoilThermalExactCarryError::Domain(
            "soil temperature projection input",
        ));
    }
    let beginning_energy = Dyadic::from_f64(beginning_enthalpy_hi_j_m2_ofe_ground)?
        .add(&Dyadic::from_wire(beginning_enthalpy_carry)?);
    let ending_energy = Dyadic::from_f64(ending_enthalpy_hi_j_m2_ofe_ground)?
        .add(&Dyadic::from_wire(ending_enthalpy_carry)?);
    let energy_delta = ending_energy.add(&beginning_energy.negated());
    let capacity = Dyadic::from_f64(heat_capacity_j_m2_k)?;
    let numerator = Dyadic::from_f64(beginning_temperature_k)?
        .multiply(&capacity)
        .add(&energy_delta);
    let projected = round_dyadic_ratio_to_f64(&numerator, &capacity)?;
    if !(200.0..=350.0).contains(&projected) {
        return Err(SoilThermalExactCarryError::Domain(
            "projected soil temperature bounds",
        ));
    }
    Ok(projected)
}

pub const SOIL_THERMAL_ENERGY_CREDIT_RECEIPT_V2_TAG: &str =
    "OPENWEPP_SOIL_THERMAL_ENERGY_CREDIT_RECEIPT_V2";

#[derive(Clone, Copy, Debug, Deserialize, Eq, Ord, PartialEq, PartialOrd, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum SoilThermalEnergyOperandKindV2 {
    SoilInternal,
    TopBoundary,
    Infiltration,
}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct SoilThermalAcceptedEnergyOperandV2 {
    pub ofe_id: OfeId,
    pub layer_id: SoilLayerId,
    pub source_kind: SoilThermalEnergyOperandKindV2,
    pub source_owner_id: ResourceOwnerId,
    pub debit_credit_identity_sha256: Sha256Digest,
    pub ordinal: u32,
    pub units: String,
    pub basis: String,
    pub energy_j_m2_ofe_ground: f64,
}

/// Solver-authoritative temperature projection bound to unchanged heat capacity.
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct SoilThermalTemperatureProjectionV2 {
    pub ofe_id: OfeId,
    pub layer_id: SoilLayerId,
    pub heat_capacity_j_m2_k: f64,
    pub ending_temperature_k: f64,
}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct SoilThermalLayerEnergyCreditV2 {
    pub ofe_id: OfeId,
    pub layer_id: SoilLayerId,
    pub beginning_enthalpy_hi_j_m2_ofe_ground: f64,
    pub beginning_enthalpy_carry: ExactDyadicEnthalpy,
    pub beginning_temperature_k: f64,
    pub ending_enthalpy_hi_j_m2_ofe_ground: f64,
    pub ending_enthalpy_carry: ExactDyadicEnthalpy,
    pub ending_temperature_k: f64,
    pub heat_capacity_j_m2_k: f64,
    pub accepted_operands: Vec<SoilThermalAcceptedEnergyOperandV2>,
}

/// Sealed accepted V2 credit receipt with complete beginning/ending and Q custody.
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct SoilThermalEnergyCreditReceiptV2 {
    pub receipt_tag: String,
    pub schema_sha256: Sha256Digest,
    pub exact_carry_definition_sha256: Sha256Digest,
    pub contract_version: u32,
    pub model_version: String,
    pub model_definition_sha256: Sha256Digest,
    pub configuration_sha256: Sha256Digest,
    pub run_id: String,
    pub soil_thermal_owner_id: ResourceOwnerId,
    pub transaction_id: TransactionId,
    pub predecessor_transaction_id: Option<TransactionId>,
    pub support_start_ns: u128,
    pub support_end_ns: u128,
    pub beginning_owner_state_sha256: Sha256Digest,
    pub ending_owner_state_sha256: Sha256Digest,
    pub predecessor_receipt_chain_sha256: Sha256Digest,
    pub layer_credits: Vec<SoilThermalLayerEnergyCreditV2>,
    pub receipt_sha256: Sha256Digest,
}

#[derive(Serialize)]
struct SoilThermalReceiptDigestBody<'a> {
    receipt_tag: &'a str,
    schema_sha256: &'a Sha256Digest,
    exact_carry_definition_sha256: &'a Sha256Digest,
    contract_version: u32,
    model_version: &'a str,
    model_definition_sha256: &'a Sha256Digest,
    configuration_sha256: &'a Sha256Digest,
    run_id: &'a str,
    soil_thermal_owner_id: &'a ResourceOwnerId,
    transaction_id: TransactionId,
    predecessor_transaction_id: Option<TransactionId>,
    support_start_ns: u128,
    support_end_ns: u128,
    beginning_owner_state_sha256: &'a Sha256Digest,
    ending_owner_state_sha256: &'a Sha256Digest,
    predecessor_receipt_chain_sha256: &'a Sha256Digest,
    layer_credits: &'a [SoilThermalLayerEnergyCreditV2],
}

impl SoilThermalEnergyCreditReceiptV2 {
    pub fn canonical_sha256(&self) -> Result<Sha256Digest, SoilThermalExactCarryError> {
        canonical_digest(&SoilThermalReceiptDigestBody {
            receipt_tag: &self.receipt_tag,
            schema_sha256: &self.schema_sha256,
            exact_carry_definition_sha256: &self.exact_carry_definition_sha256,
            contract_version: self.contract_version,
            model_version: &self.model_version,
            model_definition_sha256: &self.model_definition_sha256,
            configuration_sha256: &self.configuration_sha256,
            run_id: &self.run_id,
            soil_thermal_owner_id: &self.soil_thermal_owner_id,
            transaction_id: self.transaction_id,
            predecessor_transaction_id: self.predecessor_transaction_id,
            support_start_ns: self.support_start_ns,
            support_end_ns: self.support_end_ns,
            beginning_owner_state_sha256: &self.beginning_owner_state_sha256,
            ending_owner_state_sha256: &self.ending_owner_state_sha256,
            predecessor_receipt_chain_sha256: &self.predecessor_receipt_chain_sha256,
            layer_credits: &self.layer_credits,
        })
        .map_err(|error| SoilThermalExactCarryError::Serialization(error.to_string()))
    }

    pub fn reseal(&mut self) -> Result<(), SoilThermalExactCarryError> {
        self.receipt_sha256 = self.canonical_sha256()?;
        Ok(())
    }

    #[allow(clippy::too_many_lines)]
    pub fn validate_independent(
        &self,
        beginning: &SoilThermalOwnerEnvelopeV2,
        ending: &SoilThermalOwnerEnvelopeV2,
        expected_operands: &[SoilThermalAcceptedEnergyOperandV2],
        expected_temperature_projections: &[SoilThermalTemperatureProjectionV2],
    ) -> Result<(), SoilThermalExactCarryError> {
        beginning.validate()?;
        ending.validate()?;
        if self.receipt_tag != SOIL_THERMAL_ENERGY_CREDIT_RECEIPT_V2_TAG
            || self.schema_sha256.as_str() != SOIL_THERMAL_OWNER_V2_SCHEMA_SHA256
            || self.exact_carry_definition_sha256.as_str()
                != EXACT_DYADIC_ENTHALPY_V1_DEFINITION_SHA256
            || self.contract_version != 15
            || self.model_version != beginning.model_version
            || self.model_definition_sha256 != beginning.model_definition_sha256
            || self.configuration_sha256 != beginning.state.configuration_sha256
            || self.run_id != beginning.run_id
            || self.soil_thermal_owner_id != beginning.state.owner_id
            || self.transaction_id != beginning.transaction_id
            || self.predecessor_transaction_id != beginning.expected_predecessor_transaction_id
            || self.support_start_ns != beginning.support_start_ns
            || self.support_end_ns != beginning.support_end_ns
            || self.beginning_owner_state_sha256 != beginning.state.state_sha256
            || self.ending_owner_state_sha256 != ending.state.state_sha256
            || self.predecessor_receipt_chain_sha256 != beginning.receipt_chain_sha256
            || ending.receipt_chain_sha256 != self.receipt_sha256
            || beginning.parent_v1_state_sha256 != ending.parent_v1_state_sha256
            || beginning.state.owner_id != ending.state.owner_id
            || beginning.state.configuration_sha256 != ending.state.configuration_sha256
            || beginning.owner_tag != ending.owner_tag
            || beginning.schema_sha256 != ending.schema_sha256
            || beginning.exact_carry_definition_sha256 != ending.exact_carry_definition_sha256
            || beginning.contract_version != ending.contract_version
            || beginning.model_version != ending.model_version
            || beginning.model_definition_sha256 != ending.model_definition_sha256
            || beginning.run_id != ending.run_id
            || beginning.transaction_id != ending.transaction_id
            || beginning.expected_predecessor_transaction_id
                != ending.expected_predecessor_transaction_id
            || beginning.support_start_ns != ending.support_start_ns
            || beginning.support_end_ns != ending.support_end_ns
            || self.canonical_sha256()? != self.receipt_sha256
        {
            return Err(SoilThermalExactCarryError::Identity(
                "soil-thermal energy receipt envelope join",
            ));
        }

        let flattened: Vec<_> = self
            .layer_credits
            .iter()
            .flat_map(|credit| credit.accepted_operands.iter().cloned())
            .collect();
        if flattened != expected_operands {
            return Err(SoilThermalExactCarryError::Receipt(
                "accepted operand omission, duplication, reorder, or substitution",
            ));
        }
        let bound_projections: Vec<_> = self
            .layer_credits
            .iter()
            .map(|credit| SoilThermalTemperatureProjectionV2 {
                ofe_id: credit.ofe_id.clone(),
                layer_id: credit.layer_id.clone(),
                heat_capacity_j_m2_k: credit.heat_capacity_j_m2_k,
                ending_temperature_k: credit.ending_temperature_k,
            })
            .collect();
        if bound_projections != expected_temperature_projections {
            return Err(SoilThermalExactCarryError::Receipt(
                "temperature projection omission, reorder, or substitution",
            ));
        }
        let mut all_debit_identities = std::collections::BTreeSet::new();
        if flattened.iter().any(|operand| {
            !all_debit_identities.insert(operand.debit_credit_identity_sha256.clone())
        }) {
            return Err(SoilThermalExactCarryError::Receipt(
                "duplicate debit/credit identity across layers",
            ));
        }
        let expected_layer_count: usize = beginning
            .state
            .ofes
            .iter()
            .map(|ofe| ofe.ordered_layers.len())
            .sum();
        if self.layer_credits.len() != expected_layer_count {
            return Err(SoilThermalExactCarryError::Cardinality(
                "incomplete or duplicate layer credit",
            ));
        }

        let mut credits = self.layer_credits.iter();
        if beginning.state.ofes.len() != ending.state.ofes.len() {
            return Err(SoilThermalExactCarryError::Cardinality(
                "ending OFE partition",
            ));
        }
        for (ofe, ending_ofe) in beginning.state.ofes.iter().zip(&ending.state.ofes) {
            if ofe.ofe_id != ending_ofe.ofe_id
                || ofe.ordered_layers.len() != ending_ofe.ordered_layers.len()
            {
                return Err(SoilThermalExactCarryError::Identity(
                    "ending ordered OFE/layer partition",
                ));
            }
            for (layer, ordered_ending_layer) in
                ofe.ordered_layers.iter().zip(&ending_ofe.ordered_layers)
            {
                let credit = credits
                    .next()
                    .ok_or(SoilThermalExactCarryError::Cardinality(
                        "missing layer credit",
                    ))?;
                let ending_layer = ending
                    .state
                    .layer(&ofe.ofe_id, &layer.layer_id)
                    .ok_or(SoilThermalExactCarryError::Identity("ending layer"))?;
                if ordered_ending_layer.layer_id != layer.layer_id
                    || ordered_ending_layer.last_accepted_transaction_id
                        != Some(beginning.transaction_id)
                    || credit.ofe_id != ofe.ofe_id
                    || credit.layer_id != layer.layer_id
                    || credit.beginning_enthalpy_hi_j_m2_ofe_ground.to_bits()
                        != layer.enthalpy_hi_j_m2_ofe_ground.to_bits()
                    || credit.beginning_enthalpy_carry != layer.enthalpy_carry
                    || credit.beginning_temperature_k.to_bits() != layer.temperature_k.to_bits()
                    || credit.ending_enthalpy_hi_j_m2_ofe_ground.to_bits()
                        != ending_layer.enthalpy_hi_j_m2_ofe_ground.to_bits()
                    || credit.ending_enthalpy_carry != ending_layer.enthalpy_carry
                    || credit.ending_temperature_k.to_bits() != ending_layer.temperature_k.to_bits()
                    || !credit.heat_capacity_j_m2_k.is_finite()
                    || credit.heat_capacity_j_m2_k <= 0.0
                {
                    return Err(SoilThermalExactCarryError::Identity(
                        "layer beginning/ending credit binding",
                    ));
                }
                validate_operand_order(credit)?;
                let values: Vec<_> = credit
                    .accepted_operands
                    .iter()
                    .map(|operand| operand.energy_j_m2_ofe_ground)
                    .collect();
                let total = ExactDyadicEnthalpy::exact_sum_binary64(
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
                    total.rounded_high_and_remainder()?
                };
                if high.to_bits() != ending_layer.enthalpy_hi_j_m2_ofe_ground.to_bits()
                    || carry != ending_layer.enthalpy_carry
                {
                    return Err(SoilThermalExactCarryError::Reconstruction);
                }
                let reconstructed = ExactDyadicEnthalpy::exact_sum([
                    &ExactDyadicEnthalpy::from_f64(high)?,
                    &carry,
                ])?;
                if reconstructed != total {
                    return Err(SoilThermalExactCarryError::Reconstruction);
                }
                let projected_temperature = project_soil_temperature_k(
                    layer.temperature_k,
                    credit.heat_capacity_j_m2_k,
                    layer.enthalpy_hi_j_m2_ofe_ground,
                    &layer.enthalpy_carry,
                    ending_layer.enthalpy_hi_j_m2_ofe_ground,
                    &ending_layer.enthalpy_carry,
                )?;
                if projected_temperature.to_bits() != ending_layer.temperature_k.to_bits() {
                    return Err(SoilThermalExactCarryError::Reconstruction);
                }
            }
        }
        Ok(())
    }
}

fn validate_operand_order(
    credit: &SoilThermalLayerEnergyCreditV2,
) -> Result<(), SoilThermalExactCarryError> {
    let mut previous = None;
    let mut debit_identities = std::collections::BTreeSet::new();
    for operand in &credit.accepted_operands {
        if operand.ofe_id != credit.ofe_id
            || operand.layer_id != credit.layer_id
            || operand.units != "J m^-2 OFE-ground"
            || operand.basis != "ofe_ground"
            || !operand.energy_j_m2_ofe_ground.is_finite()
            || !debit_identities.insert(operand.debit_credit_identity_sha256.clone())
        {
            return Err(SoilThermalExactCarryError::Receipt(
                "operand identity, unit, basis, finite domain, or uniqueness",
            ));
        }
        let key = (operand.source_kind, operand.ordinal);
        if previous.is_some_and(|prior| prior >= key) {
            return Err(SoilThermalExactCarryError::Receipt(
                "noncanonical operand kind/ordinal order",
            ));
        }
        previous = Some(key);
    }
    Ok(())
}

#[derive(Clone, Debug, Eq, PartialEq)]
struct BigUint {
    limbs: Vec<u64>,
}

impl BigUint {
    fn zero() -> Self {
        Self { limbs: Vec::new() }
    }

    fn from_u64(value: u64) -> Self {
        if value == 0 {
            Self::zero()
        } else {
            Self { limbs: vec![value] }
        }
    }

    fn from_hex(value: &str) -> Result<Self, ExactDyadicEnthalpyError> {
        if value.len() > MAX_WIRE_HEX_DIGITS {
            return Err(ExactDyadicEnthalpyError::CoefficientResourceLimit);
        }
        let mut result = Self::zero();
        for chunk_end in (1..=value.len()).rev().step_by(16) {
            let chunk_start = chunk_end.saturating_sub(16);
            let chunk = &value[chunk_start..chunk_end];
            let limb = u64::from_str_radix(chunk, 16).map_err(|_| {
                ExactDyadicEnthalpyError::NonCanonicalWire("invalid hexadecimal coefficient")
            })?;
            result.limbs.push(limb);
            if chunk_start == 0 {
                break;
            }
        }
        result.normalize();
        Ok(result)
    }

    fn to_hex(&self) -> String {
        let Some(last) = self.limbs.last() else {
            return "0".to_owned();
        };
        let mut result = format!("{last:x}");
        for limb in self.limbs[..self.limbs.len() - 1].iter().rev() {
            use core::fmt::Write;
            let _ = write!(result, "{limb:016x}");
        }
        result
    }

    fn normalize(&mut self) {
        while self.limbs.last() == Some(&0) {
            self.limbs.pop();
        }
    }

    fn is_zero(&self) -> bool {
        self.limbs.is_empty()
    }

    fn is_odd(&self) -> bool {
        self.limbs.first().is_some_and(|limb| limb & 1 == 1)
    }

    fn bit_len(&self) -> usize {
        self.limbs.last().map_or(0, |last| {
            (self.limbs.len() - 1) * 64 + (64 - last.leading_zeros() as usize)
        })
    }

    fn bit(&self, index: usize) -> bool {
        self.limbs
            .get(index / 64)
            .is_some_and(|limb| limb & (1_u64 << (index % 64)) != 0)
    }

    fn any_below(&self, bit_count: usize) -> bool {
        if bit_count == 0 {
            return false;
        }
        let complete_limbs = bit_count / 64;
        if self
            .limbs
            .iter()
            .take(complete_limbs)
            .any(|limb| *limb != 0)
        {
            return true;
        }
        let remaining = bit_count % 64;
        remaining != 0
            && self
                .limbs
                .get(complete_limbs)
                .is_some_and(|limb| limb & ((1_u64 << remaining).wrapping_sub(1)) != 0)
    }

    fn trailing_zeros(&self) -> usize {
        let zero_limbs = self.limbs.iter().take_while(|limb| **limb == 0).count();
        self.limbs
            .get(zero_limbs)
            .map_or(0, |limb| zero_limbs * 64 + limb.trailing_zeros() as usize)
    }

    fn cmp_magnitude(&self, other: &Self) -> Ordering {
        self.limbs
            .len()
            .cmp(&other.limbs.len())
            .then_with(|| self.limbs.iter().rev().cmp(other.limbs.iter().rev()))
    }

    fn add(&self, other: &Self) -> Self {
        let mut result = Vec::with_capacity(self.limbs.len().max(other.limbs.len()) + 1);
        let mut carry = 0_u128;
        for index in 0..self.limbs.len().max(other.limbs.len()) {
            let sum = u128::from(*self.limbs.get(index).unwrap_or(&0))
                + u128::from(*other.limbs.get(index).unwrap_or(&0))
                + carry;
            let bytes = sum.to_le_bytes();
            result.push(u64::from_le_bytes([
                bytes[0], bytes[1], bytes[2], bytes[3], bytes[4], bytes[5], bytes[6], bytes[7],
            ]));
            carry = sum >> 64;
        }
        if carry != 0 {
            result.push(u64::from(carry != 0));
        }
        Self { limbs: result }
    }

    fn multiply(&self, other: &Self) -> Self {
        if self.is_zero() || other.is_zero() {
            return Self::zero();
        }
        let mut result = vec![0_u64; self.limbs.len() + other.limbs.len()];
        for (left_index, left) in self.limbs.iter().enumerate() {
            let mut carry = 0_u128;
            for (right_index, right) in other.limbs.iter().enumerate() {
                let index = left_index + right_index;
                let product =
                    u128::from(*left) * u128::from(*right) + u128::from(result[index]) + carry;
                let bytes = product.to_le_bytes();
                result[index] = u64::from_le_bytes([
                    bytes[0], bytes[1], bytes[2], bytes[3], bytes[4], bytes[5], bytes[6], bytes[7],
                ]);
                carry = product >> 64;
            }
            let mut index = left_index + other.limbs.len();
            while carry != 0 {
                let sum = u128::from(result[index]) + carry;
                let bytes = sum.to_le_bytes();
                result[index] = u64::from_le_bytes([
                    bytes[0], bytes[1], bytes[2], bytes[3], bytes[4], bytes[5], bytes[6], bytes[7],
                ]);
                carry = sum >> 64;
                index += 1;
                if index == result.len() && carry != 0 {
                    result.push(0);
                }
            }
        }
        let mut result = Self { limbs: result };
        result.normalize();
        result
    }

    fn sub(&self, other: &Self) -> Self {
        debug_assert!(self.cmp_magnitude(other) != Ordering::Less);
        let mut result = Vec::with_capacity(self.limbs.len());
        let mut borrow = false;
        for index in 0..self.limbs.len() {
            let lhs = self.limbs[index];
            let rhs = *other.limbs.get(index).unwrap_or(&0);
            let (partial, borrow_rhs) = lhs.overflowing_sub(rhs);
            let (value, borrow_carry) = partial.overflowing_sub(u64::from(borrow));
            result.push(value);
            borrow = borrow_rhs || borrow_carry;
        }
        debug_assert!(!borrow);
        let mut result = Self { limbs: result };
        result.normalize();
        result
    }

    fn shl(&self, shift: usize) -> Self {
        if self.is_zero() {
            return Self::zero();
        }
        let limb_shift = shift / 64;
        let bit_shift = shift % 64;
        let mut result = vec![0; limb_shift];
        let mut carry = 0_u64;
        for limb in &self.limbs {
            result.push((*limb << bit_shift) | carry);
            carry = if bit_shift == 0 {
                0
            } else {
                *limb >> (64 - bit_shift)
            };
        }
        if carry != 0 {
            result.push(carry);
        }
        Self { limbs: result }
    }

    fn shr(&self, shift: usize) -> Self {
        let limb_shift = shift / 64;
        if limb_shift >= self.limbs.len() {
            return Self::zero();
        }
        let bit_shift = shift % 64;
        let mut result = Vec::with_capacity(self.limbs.len() - limb_shift);
        let mut carry = 0_u64;
        for limb in self.limbs[limb_shift..].iter().rev() {
            let value = if bit_shift == 0 {
                *limb
            } else {
                (*limb >> bit_shift) | carry
            };
            result.push(value);
            carry = if bit_shift == 0 {
                0
            } else {
                *limb << (64 - bit_shift)
            };
        }
        result.reverse();
        let mut result = Self { limbs: result };
        result.normalize();
        result
    }

    fn add_one(&mut self) {
        for limb in &mut self.limbs {
            let (value, carry) = limb.overflowing_add(1);
            *limb = value;
            if !carry {
                return;
            }
        }
        self.limbs.push(1);
    }

    fn set_bit(&mut self, index: usize) {
        let limb_index = index / 64;
        self.limbs.resize(limb_index + 1, 0);
        self.limbs[limb_index] |= 1_u64 << (index % 64);
    }

    fn div_rem(&self, divisor: &Self) -> (Self, Self) {
        debug_assert!(!divisor.is_zero());
        if self.cmp_magnitude(divisor) == Ordering::Less {
            return (Self::zero(), self.clone());
        }
        let shift = self.bit_len() - divisor.bit_len();
        let mut shifted_divisor = divisor.shl(shift);
        let mut remainder = self.clone();
        let mut quotient = Self::zero();
        for bit in (0..=shift).rev() {
            if remainder.cmp_magnitude(&shifted_divisor) != Ordering::Less {
                remainder = remainder.sub(&shifted_divisor);
                quotient.set_bit(bit);
            }
            shifted_divisor = shifted_divisor.shr(1);
        }
        (quotient, remainder)
    }

    fn to_u64(&self) -> Option<u64> {
        match self.limbs.as_slice() {
            [] => Some(0),
            [value] => Some(*value),
            _ => None,
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
struct Dyadic {
    sign: i8,
    coefficient: BigUint,
    exponent2: i32,
}

impl Dyadic {
    fn zero() -> Self {
        Self {
            sign: 0,
            coefficient: BigUint::zero(),
            exponent2: 0,
        }
    }

    fn normalized(sign: i8, mut coefficient: BigUint, exponent2: i32) -> Self {
        if coefficient.is_zero() {
            return Self::zero();
        }
        let trailing = coefficient.trailing_zeros();
        coefficient = coefficient.shr(trailing);
        let trailing_exponent = i32::try_from(trailing).unwrap_or(i32::MAX);
        Self {
            sign,
            coefficient,
            exponent2: exponent2.saturating_add(trailing_exponent),
        }
    }

    fn from_wire(value: &ExactDyadicEnthalpy) -> Result<Self, ExactDyadicEnthalpyError> {
        value.validate()?;
        if value.sign == 0 {
            return Ok(Self::zero());
        }
        Ok(Self {
            sign: value.sign,
            coefficient: BigUint::from_hex(&value.coefficient_hex)?,
            exponent2: value.exponent2,
        })
    }

    fn into_wire(self) -> ExactDyadicEnthalpy {
        if self.sign == 0 {
            ExactDyadicEnthalpy::zero()
        } else {
            ExactDyadicEnthalpy {
                sign: self.sign,
                coefficient_hex: self.coefficient.to_hex(),
                exponent2: self.exponent2,
            }
        }
    }

    fn from_f64(value: f64) -> Result<Self, ExactDyadicEnthalpyError> {
        if !value.is_finite() {
            return Err(ExactDyadicEnthalpyError::NonFiniteBinary64);
        }
        let bits = value.to_bits();
        let sign = if bits >> 63 == 0 { 1 } else { -1 };
        let exponent = ((bits >> 52) & 0x7ff) as i32;
        let fraction = bits & ((1_u64 << 52) - 1);
        if exponent == 0 && fraction == 0 {
            return Ok(Self::zero());
        }
        let (coefficient, exponent2) = if exponent == 0 {
            (fraction, -1074)
        } else {
            ((1_u64 << 52) | fraction, exponent - 1023 - 52)
        };
        Ok(Self::normalized(
            sign,
            BigUint::from_u64(coefficient),
            exponent2,
        ))
    }

    fn add(&self, other: &Self) -> Self {
        if self.sign == 0 {
            return other.clone();
        }
        if other.sign == 0 {
            return self.clone();
        }
        let exponent2 = self.exponent2.min(other.exponent2);
        let lhs = self
            .coefficient
            .shl((self.exponent2 - exponent2).unsigned_abs() as usize);
        let rhs = other
            .coefficient
            .shl((other.exponent2 - exponent2).unsigned_abs() as usize);
        if self.sign == other.sign {
            return Self::normalized(self.sign, lhs.add(&rhs), exponent2);
        }
        match lhs.cmp_magnitude(&rhs) {
            Ordering::Equal => Self::zero(),
            Ordering::Greater => Self::normalized(self.sign, lhs.sub(&rhs), exponent2),
            Ordering::Less => Self::normalized(other.sign, rhs.sub(&lhs), exponent2),
        }
    }

    fn negated(&self) -> Self {
        let mut result = self.clone();
        result.sign = -result.sign;
        result
    }

    fn multiply(&self, other: &Self) -> Self {
        if self.sign == 0 || other.sign == 0 {
            return Self::zero();
        }
        Self::normalized(
            self.sign * other.sign,
            self.coefficient.multiply(&other.coefficient),
            self.exponent2.saturating_add(other.exponent2),
        )
    }

    fn rounded_integer_at(&self, unit_exponent: i32) -> BigUint {
        if self.exponent2 >= unit_exponent {
            return self
                .coefficient
                .shl((self.exponent2 - unit_exponent).unsigned_abs() as usize);
        }
        let shift = (unit_exponent - self.exponent2).unsigned_abs() as usize;
        let mut quotient = self.coefficient.shr(shift);
        let halfway = shift != 0 && self.coefficient.bit(shift - 1);
        let above_halfway = halfway && self.coefficient.any_below(shift - 1);
        if above_halfway || halfway && quotient.is_odd() {
            quotient.add_one();
        }
        quotient
    }

    fn round_to_f64(&self) -> Result<f64, ExactDyadicEnthalpyError> {
        if self.sign == 0 {
            return Ok(0.0);
        }
        let top_exponent = i64::from(self.exponent2)
            + i64::try_from(self.coefficient.bit_len() - 1)
                .map_err(|_| ExactDyadicEnthalpyError::ExponentOutOfRange)?;
        if top_exponent > 1023 {
            return Err(ExactDyadicEnthalpyError::Binary64Overflow);
        }
        let sign_bit = if self.sign < 0 { 1_u64 << 63 } else { 0 };
        if top_exponent < -1022 {
            let rounded = self.rounded_integer_at(-1074);
            let significand = rounded
                .to_u64()
                .ok_or(ExactDyadicEnthalpyError::Binary64Overflow)?;
            if significand > 1_u64 << 52 {
                return Err(ExactDyadicEnthalpyError::Binary64Overflow);
            }
            if significand == 1_u64 << 52 {
                return Ok(f64::from_bits(sign_bit | (1_u64 << 52)));
            }
            return Ok(f64::from_bits(sign_bit | significand));
        }

        let mut binary_exponent = i32::try_from(top_exponent)
            .map_err(|_| ExactDyadicEnthalpyError::ExponentOutOfRange)?;
        let mut significand = self
            .rounded_integer_at(binary_exponent - 52)
            .to_u64()
            .ok_or(ExactDyadicEnthalpyError::Binary64Overflow)?;
        if significand == 1_u64 << 53 {
            significand = 1_u64 << 52;
            binary_exponent += 1;
        }
        if binary_exponent > 1023 {
            return Err(ExactDyadicEnthalpyError::Binary64Overflow);
        }
        let exponent_bits = u64::try_from(binary_exponent + 1023)
            .map_err(|_| ExactDyadicEnthalpyError::ExponentOutOfRange)?;
        let fraction = significand - (1_u64 << 52);
        Ok(f64::from_bits(sign_bit | (exponent_bits << 52) | fraction))
    }
}

fn compare_ratio_to_power_of_two(
    numerator: &BigUint,
    denominator: &BigUint,
    ratio_exponent2: i32,
    power_exponent2: i64,
) -> Ordering {
    let shift = i64::from(ratio_exponent2) - power_exponent2;
    if shift >= 0 {
        numerator
            .shl(usize::try_from(shift).unwrap_or(usize::MAX))
            .cmp_magnitude(denominator)
    } else {
        numerator.cmp_magnitude(&denominator.shl(usize::try_from(-shift).unwrap_or(usize::MAX)))
    }
}

fn round_dyadic_ratio_to_f64(
    numerator: &Dyadic,
    denominator: &Dyadic,
) -> Result<f64, ExactDyadicEnthalpyError> {
    if denominator.sign <= 0 {
        return Err(ExactDyadicEnthalpyError::NonCanonicalWire(
            "ratio denominator must be positive",
        ));
    }
    if numerator.sign == 0 {
        return Ok(0.0);
    }
    let ratio_exponent2 = numerator.exponent2.saturating_sub(denominator.exponent2);
    let tentative_exponent = i64::try_from(numerator.coefficient.bit_len())
        .map_err(|_| ExactDyadicEnthalpyError::ExponentOutOfRange)?
        - i64::try_from(denominator.coefficient.bit_len())
            .map_err(|_| ExactDyadicEnthalpyError::ExponentOutOfRange)?
        + i64::from(ratio_exponent2);
    let top_exponent = if compare_ratio_to_power_of_two(
        &numerator.coefficient,
        &denominator.coefficient,
        ratio_exponent2,
        tentative_exponent,
    ) == Ordering::Less
    {
        tentative_exponent - 1
    } else {
        tentative_exponent
    };
    if top_exponent > 1023 {
        return Err(ExactDyadicEnthalpyError::Binary64Overflow);
    }
    let unit_exponent = if top_exponent < -1022 {
        -1074_i64
    } else {
        top_exponent - 52
    };
    let scale_shift = i64::from(ratio_exponent2) - unit_exponent;
    let (dividend, divisor) = if scale_shift >= 0 {
        (
            numerator
                .coefficient
                .shl(usize::try_from(scale_shift).unwrap_or(usize::MAX)),
            denominator.coefficient.clone(),
        )
    } else {
        (
            numerator.coefficient.clone(),
            denominator
                .coefficient
                .shl(usize::try_from(-scale_shift).unwrap_or(usize::MAX)),
        )
    };
    let (mut quotient, remainder) = dividend.div_rem(&divisor);
    let doubled_remainder = remainder.shl(1);
    let remainder_order = doubled_remainder.cmp_magnitude(&divisor);
    if remainder_order == Ordering::Greater
        || remainder_order == Ordering::Equal && quotient.is_odd()
    {
        quotient.add_one();
    }
    let mut significand = quotient
        .to_u64()
        .ok_or(ExactDyadicEnthalpyError::Binary64Overflow)?;
    let sign_bit = if numerator.sign < 0 { 1_u64 << 63 } else { 0 };
    if top_exponent < -1022 {
        if significand > 1_u64 << 52 {
            return Err(ExactDyadicEnthalpyError::Binary64Overflow);
        }
        return Ok(f64::from_bits(sign_bit | significand));
    }
    let mut binary_exponent =
        i32::try_from(top_exponent).map_err(|_| ExactDyadicEnthalpyError::ExponentOutOfRange)?;
    if significand == 1_u64 << 53 {
        significand = 1_u64 << 52;
        binary_exponent += 1;
    }
    if binary_exponent > 1023 {
        return Err(ExactDyadicEnthalpyError::Binary64Overflow);
    }
    let exponent_bits = u64::try_from(binary_exponent + 1023)
        .map_err(|_| ExactDyadicEnthalpyError::ExponentOutOfRange)?;
    Ok(f64::from_bits(
        sign_bit | (exponent_bits << 52) | (significand - (1_u64 << 52)),
    ))
}

#[cfg(test)]
#[allow(
    clippy::excessive_precision,
    clippy::float_cmp,
    clippy::unreadable_literal
)]
mod tests {
    use super::*;
    use crate::{
        SOIL_THERMAL_OWNER_V2_TAG, SoilThermalLayerSnapshot, SoilThermalOfeSnapshot,
        SoilThermalSnapshot, SoilThermalV2MigrationIdentity, apply_soil_thermal_energy_credit_v2,
        migrate_soil_thermal_v1_to_v2, refuse_soil_thermal_v2_to_v1_downgrade,
    };

    fn add(values: &[f64]) -> (f64, ExactDyadicEnthalpy) {
        let total =
            ExactDyadicEnthalpy::exact_sum_binary64(0.0, &ExactDyadicEnthalpy::zero(), values)
                .expect("finite exact sum");
        total
            .rounded_high_and_remainder()
            .expect("finite rounded split")
    }

    #[test]
    fn canonical_wat5_credit_retains_unchanged_high_and_nonzero_carry() {
        let beginning = -34315.42154113602_f64;
        let credit = -8.0670339832330148e-19_f64;
        let total = ExactDyadicEnthalpy::exact_sum_binary64(
            beginning,
            &ExactDyadicEnthalpy::zero(),
            &[credit],
        )
        .expect("WAT5 exact sum");
        let (high, carry) = total
            .rounded_high_and_remainder()
            .expect("WAT5 exact split");
        assert_eq!(high.to_bits(), beginning.to_bits());
        assert_eq!(
            carry,
            ExactDyadicEnthalpy::try_new(-1, "1dc319224e55f", -109).expect("canonical WAT5 carry")
        );
        assert_eq!(
            ExactDyadicEnthalpy::exact_sum([
                &ExactDyadicEnthalpy::from_f64(high).expect("high dyadic"),
                &carry,
            ])
            .expect("reconstruction"),
            total
        );
    }

    #[test]
    fn nearest_even_halfway_and_adjacent_crossing_vectors() {
        let one = ExactDyadicEnthalpy::from_f64(1.0).expect("one");
        let half_ulp = ExactDyadicEnthalpy::try_new(1, "1", -53).expect("half ulp");
        let even_tie = ExactDyadicEnthalpy::exact_sum([&one, &half_ulp]).expect("even tie");
        assert_eq!(even_tie.round_to_f64().expect("round"), 1.0);

        let odd_low =
            ExactDyadicEnthalpy::from_f64(f64::from_bits(1.0_f64.to_bits() + 1)).expect("odd low");
        let odd_tie = ExactDyadicEnthalpy::exact_sum([&odd_low, &half_ulp]).expect("odd tie");
        assert_eq!(
            odd_tie.round_to_f64().expect("round"),
            f64::from_bits(1.0_f64.to_bits() + 2)
        );

        let above_half = ExactDyadicEnthalpy::try_new(1, "3", -54).expect("above half");
        let crossing = ExactDyadicEnthalpy::exact_sum([&one, &above_half]).expect("crossing");
        assert_eq!(
            crossing.round_to_f64().expect("round"),
            f64::from_bits(1.0_f64.to_bits() + 1)
        );

        let one_third_projection = project_soil_temperature_k(
            273.15,
            3.0,
            0.0,
            &ExactDyadicEnthalpy::zero(),
            1.0,
            &ExactDyadicEnthalpy::zero(),
        )
        .expect("exact rational projection");
        assert_eq!(one_third_projection.to_bits(), 0x4071_17bb_bbbb_bbbb);
    }

    #[test]
    fn signed_cancellation_subnormal_boundary_and_order_are_exact() {
        let minimum = f64::from_bits(1);
        let values = [1.0, -1.0, minimum, minimum, -minimum, 0.25, -0.25];
        let permutations = [
            values.to_vec(),
            values.iter().copied().rev().collect(),
            vec![minimum, 1.0, 0.25, -minimum, -0.25, -1.0, minimum],
        ];
        let totals: Vec<_> = permutations
            .iter()
            .map(|values| {
                ExactDyadicEnthalpy::exact_sum_binary64(0.0, &ExactDyadicEnthalpy::zero(), values)
                    .expect("permutation sum")
            })
            .collect();
        assert!(totals.windows(2).all(|pair| pair[0] == pair[1]));
        assert_eq!(totals[0].round_to_f64().expect("subnormal"), minimum);
        assert_eq!(add(&[minimum, -minimum]).1, ExactDyadicEnthalpy::zero());

        let largest_subnormal = f64::from_bits((1_u64 << 52) - 1);
        let (high, carry) = add(&[largest_subnormal, minimum]);
        assert_eq!(high.to_bits(), 1_u64 << 52);
        assert_eq!(carry, ExactDyadicEnthalpy::zero());
    }

    #[test]
    fn both_signs_and_overflow_refuse_without_clamping() {
        for value in [f64::from_bits(1), -f64::from_bits(1), 7.5, -7.5] {
            let exact = ExactDyadicEnthalpy::from_f64(value).expect("finite decode");
            assert_eq!(
                exact.round_to_f64().expect("roundtrip").to_bits(),
                value.to_bits()
            );
        }
        let largest = ExactDyadicEnthalpy::from_f64(f64::MAX).expect("largest finite");
        let quarter_ulp = ExactDyadicEnthalpy::try_new(1, "1", 969).expect("quarter max ulp");
        let still_largest =
            ExactDyadicEnthalpy::exact_sum([&largest, &quarter_ulp]).expect("boundary total");
        assert_eq!(
            still_largest.round_to_f64().expect("largest boundary"),
            f64::MAX
        );
        let half_ulp = ExactDyadicEnthalpy::try_new(1, "1", 970).expect("half max ulp");
        let overflow = ExactDyadicEnthalpy::exact_sum([&largest, &half_ulp]).expect("exact total");
        assert_eq!(
            overflow.round_to_f64(),
            Err(ExactDyadicEnthalpyError::Binary64Overflow)
        );
        assert_eq!(
            ExactDyadicEnthalpy::from_f64(f64::INFINITY),
            Err(ExactDyadicEnthalpyError::NonFiniteBinary64)
        );

        let half_minimum = ExactDyadicEnthalpy::try_new(1, "1", -1075).expect("half minimum");
        assert_eq!(
            half_minimum
                .round_to_f64()
                .expect("positive zero")
                .to_bits(),
            0
        );
        let negative_half_minimum =
            ExactDyadicEnthalpy::try_new(-1, "1", -1075).expect("negative half minimum");
        assert_eq!(
            negative_half_minimum
                .round_to_f64()
                .expect("negative zero")
                .to_bits(),
            1_u64 << 63
        );
    }

    #[test]
    fn wire_schema_rejects_all_equivalent_noncanonical_forms() {
        for json in [
            r#"{"sign":0,"coefficient_hex":"0","exponent2":1}"#,
            r#"{"sign":1,"coefficient_hex":"0","exponent2":0}"#,
            r#"{"sign":1,"coefficient_hex":"01","exponent2":0}"#,
            r#"{"sign":1,"coefficient_hex":"2","exponent2":0}"#,
            r#"{"sign":1,"coefficient_hex":"A","exponent2":0}"#,
            r#"{"sign":-2,"coefficient_hex":"1","exponent2":0}"#,
            r#"{"sign":-1,"coefficient_hex":"-1","exponent2":0}"#,
        ] {
            assert!(
                serde_json::from_str::<ExactDyadicEnthalpy>(json).is_err(),
                "{json}"
            );
        }
        let zero: ExactDyadicEnthalpy =
            serde_json::from_str(r#"{"sign":0,"coefficient_hex":"0","exponent2":0}"#)
                .expect("unique zero");
        assert_eq!(zero, ExactDyadicEnthalpy::zero());
    }

    fn digest(character: char) -> Sha256Digest {
        Sha256Digest::try_new(character.to_string().repeat(64)).expect("digest")
    }

    fn v1_snapshot() -> SoilThermalSnapshot {
        SoilThermalSnapshot {
            owner_id: ResourceOwnerId::try_new("soil-owner").expect("owner"),
            configuration_sha256: digest('a'),
            state_sha256: digest('b'),
            snapshot_sha256: digest('c'),
            last_accepted_transaction_id: Some(TransactionId(40)),
            ofes: vec![SoilThermalOfeSnapshot {
                ofe_id: OfeId::try_new("ofe-1").expect("OFE"),
                ordered_layers: vec![SoilThermalLayerSnapshot {
                    layer_id: SoilLayerId::try_new("layer-1").expect("layer"),
                    temperature_k: f64::from_bits(273.15_f64.to_bits()),
                    enthalpy_j_m2_ofe_ground: -34315.42154113602,
                }],
            }],
        }
    }

    fn migrated() -> SoilThermalOwnerEnvelopeV2 {
        migrate_soil_thermal_v1_to_v2(
            &v1_snapshot(),
            SoilThermalV2MigrationIdentity {
                model_version: "soil-thermal-model-v1".to_owned(),
                model_definition_sha256: digest('d'),
                run_id: "run-1".to_owned(),
                transaction_id: TransactionId(41),
                support_start_ns: 0,
                support_end_ns: 60_000_000_000,
                receipt_chain_sha256: digest('e'),
            },
        )
        .expect("V1-to-V2 migration")
    }

    fn operand(
        kind: SoilThermalEnergyOperandKindV2,
        ordinal: u32,
        value: f64,
        identity: char,
    ) -> SoilThermalAcceptedEnergyOperandV2 {
        SoilThermalAcceptedEnergyOperandV2 {
            ofe_id: OfeId::try_new("ofe-1").expect("OFE"),
            layer_id: SoilLayerId::try_new("layer-1").expect("layer"),
            source_kind: kind,
            source_owner_id: ResourceOwnerId::try_new("source-owner").expect("source owner"),
            debit_credit_identity_sha256: digest(identity),
            ordinal,
            units: "J m^-2 OFE-ground".to_owned(),
            basis: "ofe_ground".to_owned(),
            energy_j_m2_ofe_ground: value,
        }
    }

    fn temperature_projection(
        heat_capacity_j_m2_k: f64,
        ending_temperature_k: f64,
    ) -> SoilThermalTemperatureProjectionV2 {
        SoilThermalTemperatureProjectionV2 {
            ofe_id: OfeId::try_new("ofe-1").expect("OFE"),
            layer_id: SoilLayerId::try_new("layer-1").expect("layer"),
            heat_capacity_j_m2_k,
            ending_temperature_k,
        }
    }

    #[test]
    fn v1_bytes_are_frozen_and_migration_is_zero_carry_with_no_downgrade() {
        let v1 = v1_snapshot();
        let bytes_before = serde_json::to_vec(&v1).expect("V1 bytes");
        let envelope = migrate_soil_thermal_v1_to_v2(
            &v1,
            SoilThermalV2MigrationIdentity {
                model_version: "soil-thermal-model-v1".to_owned(),
                model_definition_sha256: digest('d'),
                run_id: "run-1".to_owned(),
                transaction_id: TransactionId(41),
                support_start_ns: 0,
                support_end_ns: 60_000_000_000,
                receipt_chain_sha256: digest('e'),
            },
        )
        .expect("migration");
        assert_eq!(serde_json::to_vec(&v1).expect("V1 bytes"), bytes_before);
        let migrated = envelope
            .state
            .layer(&v1.ofes[0].ofe_id, &v1.ofes[0].ordered_layers[0].layer_id)
            .expect("migrated layer");
        assert_eq!(
            migrated.temperature_k.to_bits(),
            v1.ofes[0].ordered_layers[0].temperature_k.to_bits()
        );
        assert_eq!(
            migrated.enthalpy_hi_j_m2_ofe_ground.to_bits(),
            v1.ofes[0].ordered_layers[0]
                .enthalpy_j_m2_ofe_ground
                .to_bits()
        );
        assert_eq!(migrated.enthalpy_carry, ExactDyadicEnthalpy::zero());
        assert_eq!(envelope.owner_tag, SOIL_THERMAL_OWNER_V2_TAG);
        assert_eq!(envelope.parent_v1_state_sha256, v1.state_sha256);
        let roundtrip: SoilThermalOwnerEnvelopeV2 =
            serde_json::from_slice(&serde_json::to_vec(&envelope).expect("V2 bytes"))
                .expect("V2 roundtrip");
        assert_eq!(roundtrip, envelope);
        assert_eq!(
            refuse_soil_thermal_v2_to_v1_downgrade(&envelope.state),
            Err(SoilThermalExactCarryError::DowngradeProhibited)
        );
    }

    #[test]
    fn typed_credit_binds_wat5_beginning_ending_and_exact_q() {
        let beginning = migrated();
        let wat5 = operand(
            SoilThermalEnergyOperandKindV2::Infiltration,
            0,
            -8.0670339832330148e-19,
            'f',
        );
        let projection = temperature_projection(2_000.0, 273.15);
        let candidate = apply_soil_thermal_energy_credit_v2(
            &beginning,
            std::slice::from_ref(&wat5),
            std::slice::from_ref(&projection),
        )
        .expect("WAT5 candidate");
        candidate
            .credit_receipt
            .validate_independent(&beginning, &candidate.ending_owner, &[wat5], &[projection])
            .expect("independent receipt reconstruction");
        let layer = &candidate.ending_owner.state.ofes[0].ordered_layers[0];
        assert_eq!(
            layer.enthalpy_hi_j_m2_ofe_ground.to_bits(),
            (-34315.42154113602_f64).to_bits()
        );
        assert_eq!(
            layer.enthalpy_carry,
            ExactDyadicEnthalpy::try_new(-1, "1dc319224e55f", -109).expect("WAT5 carry")
        );
        assert_eq!(layer.temperature_k.to_bits(), 273.15_f64.to_bits());
        assert_eq!(
            candidate.credit_receipt.predecessor_transaction_id,
            Some(TransactionId(40))
        );
        assert_eq!(candidate.credit_receipt.transaction_id, TransactionId(41));
    }

    #[test]
    fn normal_energy_delta_requires_authoritative_temperature_change() {
        let beginning = migrated();
        let energy = operand(SoilThermalEnergyOperandKindV2::TopBoundary, 0, 1_000.0, '9');
        let projection = temperature_projection(2_000.0, 273.65);
        let candidate = apply_soil_thermal_energy_credit_v2(
            &beginning,
            std::slice::from_ref(&energy),
            std::slice::from_ref(&projection),
        )
        .expect("normal delta projection");
        assert_eq!(
            candidate.ending_owner.state.ofes[0].ordered_layers[0]
                .temperature_k
                .to_bits(),
            273.65_f64.to_bits()
        );

        let stale = temperature_projection(2_000.0, 273.15);
        let before = serde_json::to_vec(&beginning).expect("beginning bytes");
        assert!(
            apply_soil_thermal_energy_credit_v2(
                &beginning,
                std::slice::from_ref(&energy),
                &[stale],
            )
            .is_err()
        );
        assert_eq!(serde_json::to_vec(&beginning).expect("rollback"), before);
    }

    #[test]
    fn no_op_candidate_preserves_negative_zero_high_bits() {
        let mut v1 = v1_snapshot();
        v1.ofes[0].ordered_layers[0].enthalpy_j_m2_ofe_ground = -0.0;
        let beginning = migrate_soil_thermal_v1_to_v2(
            &v1,
            SoilThermalV2MigrationIdentity {
                model_version: "soil-thermal-model-v1".to_owned(),
                model_definition_sha256: digest('d'),
                run_id: "run-1".to_owned(),
                transaction_id: TransactionId(41),
                support_start_ns: 0,
                support_end_ns: 60_000_000_000,
                receipt_chain_sha256: digest('e'),
            },
        )
        .expect("migration");
        let projection = temperature_projection(2_000.0, 273.15);
        let candidate =
            apply_soil_thermal_energy_credit_v2(&beginning, &[], std::slice::from_ref(&projection))
                .expect("no-op credit");
        assert_eq!(
            candidate.ending_owner.state.ofes[0].ordered_layers[0]
                .enthalpy_hi_j_m2_ofe_ground
                .to_bits(),
            (-0.0_f64).to_bits()
        );
        candidate
            .credit_receipt
            .validate_independent(&beginning, &candidate.ending_owner, &[], &[projection])
            .expect("no-op independent validation");
    }

    #[test]
    #[allow(clippy::too_many_lines)]
    fn receipt_omission_duplication_reorder_and_identity_substitution_refuse() {
        let beginning = migrated();
        let operands = vec![
            operand(SoilThermalEnergyOperandKindV2::SoilInternal, 0, 0.25, '1'),
            operand(SoilThermalEnergyOperandKindV2::TopBoundary, 0, -0.125, '2'),
            operand(
                SoilThermalEnergyOperandKindV2::Infiltration,
                0,
                -8.0670339832330148e-19,
                '3',
            ),
        ];
        let expected_temperature = 273.15 + 0.125 / 2_000.0;
        let projection = temperature_projection(2_000.0, expected_temperature);
        let candidate = apply_soil_thermal_energy_credit_v2(
            &beginning,
            &operands,
            std::slice::from_ref(&projection),
        )
        .expect("canonical receipt");
        let beginning_bytes = serde_json::to_vec(&beginning).expect("beginning bytes");

        let mut expected_poisons = Vec::new();
        expected_poisons.push(operands[..2].to_vec());
        let mut duplicate = operands.clone();
        duplicate.push(operands[2].clone());
        expected_poisons.push(duplicate);
        let mut reordered = operands.clone();
        reordered.swap(0, 1);
        expected_poisons.push(reordered);
        for poison in expected_poisons {
            assert!(
                candidate
                    .credit_receipt
                    .validate_independent(
                        &beginning,
                        &candidate.ending_owner,
                        &poison,
                        std::slice::from_ref(&projection),
                    )
                    .is_err()
            );
        }

        let mut receipt_poisons = Vec::new();
        let mut wrong_support = candidate.credit_receipt.clone();
        wrong_support.support_end_ns += 1;
        wrong_support.reseal().expect("reseal support poison");
        receipt_poisons.push(wrong_support);
        let mut wrong_predecessor = candidate.credit_receipt.clone();
        wrong_predecessor.predecessor_transaction_id = Some(TransactionId(39));
        wrong_predecessor
            .reseal()
            .expect("reseal predecessor poison");
        receipt_poisons.push(wrong_predecessor);
        let mut wrong_layer = candidate.credit_receipt.clone();
        wrong_layer.layer_credits[0].layer_id = SoilLayerId::try_new("wrong").expect("layer");
        wrong_layer.reseal().expect("reseal layer poison");
        receipt_poisons.push(wrong_layer);
        let mut wrong_beginning = candidate.credit_receipt.clone();
        wrong_beginning.layer_credits[0].beginning_enthalpy_hi_j_m2_ofe_ground = 1.0;
        wrong_beginning.reseal().expect("reseal beginning poison");
        receipt_poisons.push(wrong_beginning);
        let mut wrong_q = candidate.credit_receipt.clone();
        wrong_q.layer_credits[0].accepted_operands[0].energy_j_m2_ofe_ground = 0.5;
        wrong_q.reseal().expect("reseal Q poison");
        receipt_poisons.push(wrong_q);
        let mut wrong_carry = candidate.credit_receipt.clone();
        wrong_carry.layer_credits[0].ending_enthalpy_carry = ExactDyadicEnthalpy::zero();
        wrong_carry.reseal().expect("reseal carry poison");
        receipt_poisons.push(wrong_carry);
        let mut wrong_temperature = candidate.credit_receipt.clone();
        wrong_temperature.layer_credits[0].ending_temperature_k = 273.15;
        wrong_temperature
            .reseal()
            .expect("reseal temperature poison");
        receipt_poisons.push(wrong_temperature);
        let mut wrong_capacity = candidate.credit_receipt.clone();
        wrong_capacity.layer_credits[0].heat_capacity_j_m2_k = 1_000.0;
        wrong_capacity.reseal().expect("reseal capacity poison");
        receipt_poisons.push(wrong_capacity);
        for poison in receipt_poisons {
            assert!(
                poison
                    .validate_independent(
                        &beginning,
                        &candidate.ending_owner,
                        &operands,
                        std::slice::from_ref(&projection),
                    )
                    .is_err()
            );
            assert_eq!(
                serde_json::to_vec(&beginning).expect("rollback bytes"),
                beginning_bytes
            );
        }

        let mut nonfinite = operands;
        nonfinite[2].energy_j_m2_ofe_ground = f64::NAN;
        assert!(
            apply_soil_thermal_energy_credit_v2(
                &beginning,
                &nonfinite,
                std::slice::from_ref(&projection),
            )
            .is_err()
        );
        assert!(apply_soil_thermal_energy_credit_v2(&beginning, &[], &[]).is_err());
        assert_eq!(
            serde_json::to_vec(&beginning).expect("rollback bytes"),
            beginning_bytes
        );
    }
}
