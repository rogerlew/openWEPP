//! Canonical persisted framing for the authoritative exact-carry soil owner.

use base64::{Engine as _, engine::general_purpose::STANDARD as BASE64};
use openwepp_hillslope_orchestrator::v9_real_consumer_shadow::{
    SoilThermalAcceptedCandidateV2, SoilThermalExpectedAcceptedOperandSetV2,
    SoilThermalOrchestratorSealsV2, validate_soil_thermal_orchestrator_seals_v2,
};
use openwepp_kernel_contract::ResourceOwnerId;
use openwepp_land_surface_energy::{
    LandSurfaceEnergyConfiguration, Sha256Digest, SoilThermalAcceptedEnergyOperandV2,
    SoilThermalEnergyCreditReceiptV2, SoilThermalOwnerCheckpointV2, SoilThermalOwnerEnvelopeV2,
    SoilThermalOwnerRestartV2, SoilThermalTemperatureProjectionV2,
};
use serde::{Deserialize, Serialize, de::DeserializeOwned};
use thiserror::Error;

use crate::{
    Sha256Hex, SoilThermalStateRestartV1, canonical_sha256, from_canonical_bytes,
    to_canonical_bytes,
};

pub const SOIL_THERMAL_RESTART_V2_SCHEMA: &str = "OPENWEPP_SOIL_THERMAL_RESTART_V2";

/// A canonical JSON object retained as an explicit type-tagged frame.
///
/// The frame prevents serde implementation details in this crate from becoming
/// an alternate native soil-owner schema. Admission decodes the native type,
/// validates it, and requires its byte-for-byte canonical re-encoding.
#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct CanonicalNativeFrameV2 {
    pub type_tag: String,
    pub canonical_json_base64: String,
    pub canonical_sha256: Sha256Hex,
}

impl CanonicalNativeFrameV2 {
    pub fn encode<T: Serialize>(
        type_tag: &'static str,
        value: &T,
    ) -> Result<Self, SoilThermalRestartV2Error> {
        let bytes = to_canonical_bytes(value).map_err(|_| SoilThermalRestartV2Error::Canonical)?;
        Ok(Self {
            type_tag: type_tag.to_owned(),
            canonical_json_base64: BASE64.encode(&bytes),
            canonical_sha256: sha(&bytes)?,
        })
    }

    pub fn decode<T>(&self, type_tag: &'static str) -> Result<T, SoilThermalRestartV2Error>
    where
        T: DeserializeOwned + Serialize,
    {
        if self.type_tag != type_tag {
            return Err(SoilThermalRestartV2Error::NativeType);
        }
        let bytes = BASE64
            .decode(&self.canonical_json_base64)
            .map_err(|_| SoilThermalRestartV2Error::Canonical)?;
        if sha(&bytes)? != self.canonical_sha256 {
            return Err(SoilThermalRestartV2Error::NativeDigest);
        }
        let value =
            from_canonical_bytes::<T>(&bytes).map_err(|_| SoilThermalRestartV2Error::Canonical)?;
        if to_canonical_bytes(&value).map_err(|_| SoilThermalRestartV2Error::Canonical)? != bytes {
            return Err(SoilThermalRestartV2Error::Canonical);
        }
        Ok(value)
    }
}

fn sha(bytes: &[u8]) -> Result<Sha256Hex, SoilThermalRestartV2Error> {
    use sha2::{Digest, Sha256};
    Sha256Hex::try_new(format!("{:x}", Sha256::digest(bytes)))
        .map_err(|_| SoilThermalRestartV2Error::NativeDigest)
}

/// Native authority required until LSE publishes seal constructors/validators.
///
/// Implementations must validate the complete native seal, including its final
/// digest. Persisted restart deliberately does not infer or duplicate that hash
/// formula.
pub trait SoilThermalNativeSealAuthorityV2 {
    fn validate_restart_seal(
        &self,
        envelope: &SoilThermalOwnerEnvelopeV2,
        seal: &SoilThermalOwnerRestartV2,
    ) -> Result<(), &'static str>;

    fn validate_checkpoint_seal(
        &self,
        envelope: &SoilThermalOwnerEnvelopeV2,
        seal: &SoilThermalOwnerCheckpointV2,
    ) -> Result<(), &'static str>;
}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct SoilThermalOwnerStateRestartV2 {
    pub schema: String,
    pub version: u16,
    pub parent_v1: SoilThermalStateRestartV1,
    pub parent_v1_restart_payload_sha256: Sha256Hex,
    pub owner_envelope: CanonicalNativeFrameV2,
    pub restart_seal: CanonicalNativeFrameV2,
    pub checkpoint_seal: CanonicalNativeFrameV2,
    pub credit_beginning_owner_envelope: Option<CanonicalNativeFrameV2>,
    pub latest_credit_receipt: Option<CanonicalNativeFrameV2>,
    pub expected_accepted_operands: Vec<SoilThermalAcceptedEnergyOperandV2>,
    pub expected_accepted_operands_sha256: Sha256Hex,
    pub expected_temperature_projections: Vec<SoilThermalTemperatureProjectionV2>,
    pub expected_temperature_projections_sha256: Sha256Hex,
    pub native_expected_source_set: Option<CanonicalNativeFrameV2>,
    pub native_orchestrator_seals: Option<CanonicalNativeFrameV2>,
    pub restart_payload_sha256: Sha256Hex,
}

#[derive(Serialize)]
struct SoilThermalRestartDigestBody<'a> {
    schema: &'a str,
    version: u16,
    parent_v1: &'a SoilThermalStateRestartV1,
    parent_v1_restart_payload_sha256: &'a Sha256Hex,
    owner_envelope: &'a CanonicalNativeFrameV2,
    restart_seal: &'a CanonicalNativeFrameV2,
    checkpoint_seal: &'a CanonicalNativeFrameV2,
    credit_beginning_owner_envelope: &'a Option<CanonicalNativeFrameV2>,
    latest_credit_receipt: &'a Option<CanonicalNativeFrameV2>,
    expected_accepted_operands: &'a [SoilThermalAcceptedEnergyOperandV2],
    expected_accepted_operands_sha256: &'a Sha256Hex,
    expected_temperature_projections: &'a [SoilThermalTemperatureProjectionV2],
    expected_temperature_projections_sha256: &'a Sha256Hex,
    native_expected_source_set: &'a Option<CanonicalNativeFrameV2>,
    native_orchestrator_seals: &'a Option<CanonicalNativeFrameV2>,
}

#[derive(Debug, Error, Clone, Copy, Eq, PartialEq)]
pub enum SoilThermalRestartV2Error {
    #[error("schema")]
    Schema,
    #[error("unsupported_version")]
    UnsupportedVersion,
    #[error("canonical_native_frame")]
    Canonical,
    #[error("native_type")]
    NativeType,
    #[error("native_digest")]
    NativeDigest,
    #[error("native_validation")]
    NativeValidation,
    #[error("native_seal_authority")]
    NativeSealAuthority,
    #[error("owner_identity")]
    OwnerIdentity,
    #[error("configuration_identity")]
    ConfigurationIdentity,
    #[error("parent_v1_identity")]
    ParentV1Identity,
    #[error("owner_restart_checkpoint_join")]
    SealJoin,
    #[error("receipt_chain")]
    ReceiptChain,
    #[error("accepted_operands")]
    AcceptedOperands,
    #[error("temperature_projections")]
    TemperatureProjections,
    #[error("payload_digest")]
    PayloadDigest,
    #[error("v2_to_v1_downgrade_prohibited")]
    DowngradeProhibited,
}

/// Complete native bundle supplied by the LSE seal authority.
pub struct SoilThermalNativeBundleV2 {
    pub owner_envelope: SoilThermalOwnerEnvelopeV2,
    pub restart_seal: SoilThermalOwnerRestartV2,
    pub checkpoint_seal: SoilThermalOwnerCheckpointV2,
    pub credit_beginning_owner_envelope: Option<SoilThermalOwnerEnvelopeV2>,
    pub latest_credit_receipt: Option<SoilThermalEnergyCreditReceiptV2>,
    pub expected_accepted_operands: Vec<SoilThermalAcceptedEnergyOperandV2>,
    pub expected_temperature_projections: Vec<SoilThermalTemperatureProjectionV2>,
    pub native_expected_source_set: Option<SoilThermalExpectedAcceptedOperandSetV2>,
    pub native_orchestrator_seals: Option<SoilThermalOrchestratorSealsV2>,
}

impl SoilThermalOwnerStateRestartV2 {
    pub fn from_accepted_candidate(
        parent_v1: SoilThermalStateRestartV1,
        beginning: SoilThermalOwnerEnvelopeV2,
        candidate: SoilThermalAcceptedCandidateV2,
        seals: SoilThermalOrchestratorSealsV2,
        configuration: &LandSurfaceEnergyConfiguration,
    ) -> Result<Self, SoilThermalRestartV2Error> {
        candidate
            .expected_sources
            .validate(&beginning, configuration)
            .map_err(|_| SoilThermalRestartV2Error::AcceptedOperands)?;
        validate_soil_thermal_orchestrator_seals_v2(&beginning, &candidate, &seals)
            .map_err(|_| SoilThermalRestartV2Error::NativeSealAuthority)?;
        let authority = AcceptedCandidateSealAuthority {
            beginning: beginning.clone(),
            candidate: candidate.clone(),
            seals: seals.clone(),
        };
        let value = Self::from_native(
            parent_v1,
            SoilThermalNativeBundleV2 {
                owner_envelope: candidate.ending_owner.clone(),
                restart_seal: seals.restart.clone(),
                checkpoint_seal: seals.checkpoint.clone(),
                credit_beginning_owner_envelope: Some(beginning),
                latest_credit_receipt: Some(candidate.credit_receipt.clone()),
                expected_accepted_operands: candidate.expected_sources.accepted_operands().to_vec(),
                expected_temperature_projections: candidate
                    .expected_sources
                    .temperature_projections()
                    .to_vec(),
                native_expected_source_set: Some(candidate.expected_sources),
                native_orchestrator_seals: Some(seals),
            },
            &configuration.soil_thermal_configuration.owner_id,
            &configuration
                .soil_thermal_configuration
                .configuration_sha256,
            &authority,
        )?;
        value.validate_with_configuration(
            &configuration.soil_thermal_configuration.owner_id,
            configuration,
            &authority,
        )?;
        Ok(value)
    }

    pub fn from_native(
        parent_v1: SoilThermalStateRestartV1,
        native: SoilThermalNativeBundleV2,
        expected_owner_id: &ResourceOwnerId,
        expected_configuration_sha256: &Sha256Digest,
        seal_authority: &dyn SoilThermalNativeSealAuthorityV2,
    ) -> Result<Self, SoilThermalRestartV2Error> {
        validate_native_bundle(
            &parent_v1,
            &native,
            expected_owner_id,
            expected_configuration_sha256,
            seal_authority,
        )?;
        let expected_accepted_operands_sha256 = digest(&native.expected_accepted_operands)?;
        let expected_temperature_projections_sha256 =
            digest(&native.expected_temperature_projections)?;
        let mut value = Self {
            schema: SOIL_THERMAL_RESTART_V2_SCHEMA.to_owned(),
            version: 2,
            parent_v1_restart_payload_sha256: parent_v1.restart_payload_sha256.clone(),
            parent_v1,
            owner_envelope: CanonicalNativeFrameV2::encode(
                "SoilThermalOwnerEnvelopeV2",
                &native.owner_envelope,
            )?,
            restart_seal: CanonicalNativeFrameV2::encode(
                "SoilThermalOwnerRestartV2",
                &native.restart_seal,
            )?,
            checkpoint_seal: CanonicalNativeFrameV2::encode(
                "SoilThermalOwnerCheckpointV2",
                &native.checkpoint_seal,
            )?,
            credit_beginning_owner_envelope: native
                .credit_beginning_owner_envelope
                .as_ref()
                .map(|envelope| {
                    CanonicalNativeFrameV2::encode("SoilThermalOwnerEnvelopeV2", envelope)
                })
                .transpose()?,
            latest_credit_receipt: native
                .latest_credit_receipt
                .as_ref()
                .map(|receipt| {
                    CanonicalNativeFrameV2::encode("SoilThermalEnergyCreditReceiptV2", receipt)
                })
                .transpose()?,
            expected_accepted_operands: native.expected_accepted_operands,
            expected_accepted_operands_sha256,
            expected_temperature_projections: native.expected_temperature_projections,
            expected_temperature_projections_sha256,
            native_expected_source_set: native
                .native_expected_source_set
                .as_ref()
                .map(|set| {
                    CanonicalNativeFrameV2::encode("SoilThermalExpectedAcceptedOperandSetV2", set)
                })
                .transpose()?,
            native_orchestrator_seals: native
                .native_orchestrator_seals
                .as_ref()
                .map(|seals| {
                    CanonicalNativeFrameV2::encode("SoilThermalOrchestratorSealsV2", seals)
                })
                .transpose()?,
            restart_payload_sha256: zero_sha()?,
        };
        value.reseal()?;
        Ok(value)
    }

    pub fn compute_restart_payload_sha256(&self) -> Result<Sha256Hex, SoilThermalRestartV2Error> {
        digest(&SoilThermalRestartDigestBody {
            schema: &self.schema,
            version: self.version,
            parent_v1: &self.parent_v1,
            parent_v1_restart_payload_sha256: &self.parent_v1_restart_payload_sha256,
            owner_envelope: &self.owner_envelope,
            restart_seal: &self.restart_seal,
            checkpoint_seal: &self.checkpoint_seal,
            credit_beginning_owner_envelope: &self.credit_beginning_owner_envelope,
            latest_credit_receipt: &self.latest_credit_receipt,
            expected_accepted_operands: &self.expected_accepted_operands,
            expected_accepted_operands_sha256: &self.expected_accepted_operands_sha256,
            expected_temperature_projections: &self.expected_temperature_projections,
            expected_temperature_projections_sha256: &self.expected_temperature_projections_sha256,
            native_expected_source_set: &self.native_expected_source_set,
            native_orchestrator_seals: &self.native_orchestrator_seals,
        })
    }

    pub fn reseal(&mut self) -> Result<(), SoilThermalRestartV2Error> {
        self.restart_payload_sha256 = self.compute_restart_payload_sha256()?;
        Ok(())
    }

    pub fn validate(
        &self,
        expected_owner_id: &ResourceOwnerId,
        expected_configuration_sha256: &Sha256Digest,
        seal_authority: &dyn SoilThermalNativeSealAuthorityV2,
    ) -> Result<SoilThermalOwnerEnvelopeV2, SoilThermalRestartV2Error> {
        if self.schema != SOIL_THERMAL_RESTART_V2_SCHEMA {
            return Err(SoilThermalRestartV2Error::Schema);
        }
        if self.version != 2 {
            return Err(SoilThermalRestartV2Error::UnsupportedVersion);
        }
        if self.parent_v1.restart_payload_sha256 != self.parent_v1_restart_payload_sha256 {
            return Err(SoilThermalRestartV2Error::ParentV1Identity);
        }
        if digest(&self.expected_accepted_operands)? != self.expected_accepted_operands_sha256 {
            return Err(SoilThermalRestartV2Error::AcceptedOperands);
        }
        if digest(&self.expected_temperature_projections)?
            != self.expected_temperature_projections_sha256
        {
            return Err(SoilThermalRestartV2Error::TemperatureProjections);
        }
        if self.compute_restart_payload_sha256()? != self.restart_payload_sha256 {
            return Err(SoilThermalRestartV2Error::PayloadDigest);
        }
        let native = self.decode_native()?;
        validate_native_bundle(
            &self.parent_v1,
            &native,
            expected_owner_id,
            expected_configuration_sha256,
            seal_authority,
        )?;
        Ok(native.owner_envelope)
    }

    pub fn decode_native(&self) -> Result<SoilThermalNativeBundleV2, SoilThermalRestartV2Error> {
        Ok(SoilThermalNativeBundleV2 {
            owner_envelope: self.owner_envelope.decode("SoilThermalOwnerEnvelopeV2")?,
            restart_seal: self.restart_seal.decode("SoilThermalOwnerRestartV2")?,
            checkpoint_seal: self
                .checkpoint_seal
                .decode("SoilThermalOwnerCheckpointV2")?,
            credit_beginning_owner_envelope: self
                .credit_beginning_owner_envelope
                .as_ref()
                .map(|frame| frame.decode("SoilThermalOwnerEnvelopeV2"))
                .transpose()?,
            latest_credit_receipt: self
                .latest_credit_receipt
                .as_ref()
                .map(|frame| frame.decode("SoilThermalEnergyCreditReceiptV2"))
                .transpose()?,
            expected_accepted_operands: self.expected_accepted_operands.clone(),
            expected_temperature_projections: self.expected_temperature_projections.clone(),
            native_expected_source_set: self
                .native_expected_source_set
                .as_ref()
                .map(|frame| frame.decode("SoilThermalExpectedAcceptedOperandSetV2"))
                .transpose()?,
            native_orchestrator_seals: self
                .native_orchestrator_seals
                .as_ref()
                .map(|frame| frame.decode("SoilThermalOrchestratorSealsV2"))
                .transpose()?,
        })
    }

    pub fn validate_with_configuration(
        &self,
        expected_owner_id: &ResourceOwnerId,
        configuration: &LandSurfaceEnergyConfiguration,
        seal_authority: &dyn SoilThermalNativeSealAuthorityV2,
    ) -> Result<SoilThermalOwnerEnvelopeV2, SoilThermalRestartV2Error> {
        let ending = self.validate(
            expected_owner_id,
            &configuration
                .soil_thermal_configuration
                .configuration_sha256,
            seal_authority,
        )?;
        let native = self.decode_native()?;
        match (
            native.credit_beginning_owner_envelope,
            native.latest_credit_receipt,
            native.native_expected_source_set,
            native.native_orchestrator_seals,
        ) {
            (Some(beginning), Some(receipt), Some(expected_sources), Some(seals)) => {
                expected_sources
                    .validate(&beginning, configuration)
                    .map_err(|_| SoilThermalRestartV2Error::AcceptedOperands)?;
                if expected_sources.accepted_operands() != self.expected_accepted_operands
                    || expected_sources.temperature_projections()
                        != self.expected_temperature_projections
                {
                    return Err(SoilThermalRestartV2Error::AcceptedOperands);
                }
                let candidate = SoilThermalAcceptedCandidateV2 {
                    ending_owner: ending.clone(),
                    credit_receipt: receipt,
                    expected_sources,
                };
                validate_soil_thermal_orchestrator_seals_v2(&beginning, &candidate, &seals)
                    .map_err(|_| SoilThermalRestartV2Error::NativeSealAuthority)?;
            }
            (None, None, None, None) => {}
            _ => return Err(SoilThermalRestartV2Error::SealJoin),
        }
        Ok(ending)
    }
}

struct AcceptedCandidateSealAuthority {
    beginning: SoilThermalOwnerEnvelopeV2,
    candidate: SoilThermalAcceptedCandidateV2,
    seals: SoilThermalOrchestratorSealsV2,
}

impl SoilThermalNativeSealAuthorityV2 for AcceptedCandidateSealAuthority {
    fn validate_restart_seal(
        &self,
        envelope: &SoilThermalOwnerEnvelopeV2,
        seal: &SoilThermalOwnerRestartV2,
    ) -> Result<(), &'static str> {
        if envelope != &self.candidate.ending_owner || seal != &self.seals.restart {
            return Err("persisted restart/native restart seal mismatch");
        }
        validate_soil_thermal_orchestrator_seals_v2(&self.beginning, &self.candidate, &self.seals)
            .map_err(|_| "native restart seal")
    }

    fn validate_checkpoint_seal(
        &self,
        envelope: &SoilThermalOwnerEnvelopeV2,
        seal: &SoilThermalOwnerCheckpointV2,
    ) -> Result<(), &'static str> {
        if envelope != &self.candidate.ending_owner || seal != &self.seals.checkpoint {
            return Err("persisted restart/native checkpoint seal mismatch");
        }
        validate_soil_thermal_orchestrator_seals_v2(&self.beginning, &self.candidate, &self.seals)
            .map_err(|_| "native checkpoint seal")
    }
}

fn validate_native_bundle(
    parent_v1: &SoilThermalStateRestartV1,
    native: &SoilThermalNativeBundleV2,
    expected_owner_id: &ResourceOwnerId,
    expected_configuration_sha256: &Sha256Digest,
    seal_authority: &dyn SoilThermalNativeSealAuthorityV2,
) -> Result<(), SoilThermalRestartV2Error> {
    native
        .owner_envelope
        .validate()
        .map_err(|_| SoilThermalRestartV2Error::NativeValidation)?;
    if native.owner_envelope.state.owner_id != *expected_owner_id {
        return Err(SoilThermalRestartV2Error::OwnerIdentity);
    }
    if native.owner_envelope.state.configuration_sha256 != *expected_configuration_sha256 {
        return Err(SoilThermalRestartV2Error::ConfigurationIdentity);
    }
    let mut resealed_parent = parent_v1.clone();
    resealed_parent
        .seal_restart_payload()
        .map_err(|_| SoilThermalRestartV2Error::ParentV1Identity)?;
    if native.owner_envelope.parent_v1_state_sha256.as_str() != parent_v1.state_sha256.as_str()
        || resealed_parent.restart_payload_sha256 != parent_v1.restart_payload_sha256
    {
        return Err(SoilThermalRestartV2Error::ParentV1Identity);
    }
    validate_seal_joins(
        &native.owner_envelope,
        &native.restart_seal,
        &native.checkpoint_seal,
    )?;
    match (
        &native.credit_beginning_owner_envelope,
        &native.latest_credit_receipt,
        &native.native_expected_source_set,
        &native.native_orchestrator_seals,
    ) {
        (Some(beginning), Some(receipt), Some(expected_sources), Some(seals)) => {
            let candidate = SoilThermalAcceptedCandidateV2 {
                ending_owner: native.owner_envelope.clone(),
                credit_receipt: receipt.clone(),
                expected_sources: expected_sources.clone(),
            };
            if seals.restart != native.restart_seal || seals.checkpoint != native.checkpoint_seal {
                return Err(SoilThermalRestartV2Error::SealJoin);
            }
            validate_soil_thermal_orchestrator_seals_v2(beginning, &candidate, seals)
                .map_err(|_| SoilThermalRestartV2Error::NativeSealAuthority)?;
        }
        (None, None, None, None) => {
            seal_authority
                .validate_restart_seal(&native.owner_envelope, &native.restart_seal)
                .map_err(|_| SoilThermalRestartV2Error::NativeSealAuthority)?;
            seal_authority
                .validate_checkpoint_seal(&native.owner_envelope, &native.checkpoint_seal)
                .map_err(|_| SoilThermalRestartV2Error::NativeSealAuthority)?;
        }
        _ => return Err(SoilThermalRestartV2Error::SealJoin),
    }
    match (
        &native.credit_beginning_owner_envelope,
        &native.latest_credit_receipt,
    ) {
        (Some(beginning), Some(receipt)) => receipt
            .validate_independent(
                beginning,
                &native.owner_envelope,
                &native.expected_accepted_operands,
                &native.expected_temperature_projections,
            )
            .map_err(|_| SoilThermalRestartV2Error::ReceiptChain)?,
        (None, None)
            if native.expected_accepted_operands.is_empty()
                && native.expected_temperature_projections.is_empty() => {}
        _ => return Err(SoilThermalRestartV2Error::ReceiptChain),
    }
    Ok(())
}

fn validate_seal_joins(
    envelope: &SoilThermalOwnerEnvelopeV2,
    restart: &SoilThermalOwnerRestartV2,
    checkpoint: &SoilThermalOwnerCheckpointV2,
) -> Result<(), SoilThermalRestartV2Error> {
    let common = restart.owner_tag == envelope.owner_tag
        && restart.schema_sha256 == envelope.schema_sha256
        && restart.exact_carry_definition_sha256 == envelope.exact_carry_definition_sha256
        && restart.parent_v1_state_sha256 == envelope.parent_v1_state_sha256
        && restart.owner_state_sha256 == envelope.state.state_sha256
        && restart.last_accepted_transaction_id == envelope.state.last_accepted_transaction_id
        && restart.receipt_chain_sha256 == envelope.receipt_chain_sha256
        && checkpoint.owner_tag == envelope.owner_tag
        && checkpoint.schema_sha256 == envelope.schema_sha256
        && checkpoint.exact_carry_definition_sha256 == envelope.exact_carry_definition_sha256
        && checkpoint.parent_v1_state_sha256 == envelope.parent_v1_state_sha256
        && checkpoint.owner_state_sha256 == envelope.state.state_sha256
        && checkpoint.last_accepted_transaction_id == envelope.state.last_accepted_transaction_id
        && checkpoint.receipt_chain_sha256 == envelope.receipt_chain_sha256;
    common
        .then_some(())
        .ok_or(SoilThermalRestartV2Error::SealJoin)
}

fn digest<T: Serialize>(value: &T) -> Result<Sha256Hex, SoilThermalRestartV2Error> {
    Sha256Hex::try_new(
        canonical_sha256(value).map_err(|_| SoilThermalRestartV2Error::PayloadDigest)?,
    )
    .map_err(|_| SoilThermalRestartV2Error::PayloadDigest)
}

fn zero_sha() -> Result<Sha256Hex, SoilThermalRestartV2Error> {
    Sha256Hex::try_new("0".repeat(64)).map_err(|_| SoilThermalRestartV2Error::PayloadDigest)
}

/// V2 is authoritative once admitted; no zero-carry exception permits downgrade.
pub fn refuse_soil_thermal_restart_v2_to_v1(
    _value: &SoilThermalOwnerStateRestartV2,
) -> Result<SoilThermalStateRestartV1, SoilThermalRestartV2Error> {
    Err(SoilThermalRestartV2Error::DowngradeProhibited)
}
