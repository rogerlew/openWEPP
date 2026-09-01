//! Additive Stage-3/V11 V5 restart for authenticated snow enthalpy carry.
//!
//! The complete V4 restart is nested byte-for-byte. This companion owns only
//! the authenticated snow material owner and its ordered carry-receipt
//! chronology at each live restart posture. Admission is isolated: no live
//! attachment is returned until the V4 state, every native frame, the receipt
//! chronology, and every base-material join have been revalidated.

use std::collections::BTreeMap;

use openwepp_hillslope_orchestrator::{
    AuthenticatedCoveredSnowMaterialOwnerV1, CoveredSnowEnthalpyCarryReceiptV1,
    DirectSnowStage3PersistentState,
    snow_stage3_v11_attachment::{
        DirectSnowStage3V11ShadowAttachment, SnowStage3V11SnowEnthalpyMaterialResidentSetV1,
        SnowStage3V11SnowEnthalpyMaterialResidentV1,
    },
};
use serde::{Deserialize, Serialize};
use sha2::{Digest as _, Sha256};
use thiserror::Error;

use crate::{
    DirectSnowStage3V11ExactEnthalpyRestartV4, ExpectedSnowStage3V11ExactEnthalpyRestartContextV4,
    Sha256Hex, SnowStage3V11RestartError, from_canonical_bytes, to_canonical_bytes,
};

pub const DIRECT_SNOW_STAGE3_V11_SNOW_ENTHALPY_RESTART_V5_SCHEMA: &str =
    "OPENWEPP_SNOW_STAGE3_V11_SNOW_ENTHALPY_RESTART_V5";

const OWNER_TYPE_TAG: &str = "AuthenticatedCoveredSnowMaterialOwnerV1";
const RECEIPT_TYPE_TAG: &str = "CoveredSnowEnthalpyCarryReceiptV1";

#[derive(Clone, Debug, Error, Eq, PartialEq)]
pub enum SnowStage3V11SnowEnthalpyRestartErrorV5 {
    #[error("schema")]
    Schema,
    #[error("unsupported_version")]
    UnsupportedVersion,
    #[error("noncanonical_bytes")]
    Canonical,
    #[error("payload_digest")]
    PayloadDigest,
    #[error("nested_v4: {0}")]
    NestedV4(String),
    #[error("native_frame")]
    NativeFrame,
    #[error("posture")]
    Posture,
    #[error("compound_owner")]
    CompoundOwner,
    #[error("receipt_chronology")]
    ReceiptChronology,
    #[error("base_material_join")]
    BaseMaterialJoin,
    #[error("nonzero_carry_blocks_downgrade")]
    DowngradeProhibited,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct NativeSnowEnthalpyFrameV5 {
    pub type_tag: String,
    pub canonical_json: Vec<u8>,
    pub canonical_sha256: Sha256Hex,
}

impl NativeSnowEnthalpyFrameV5 {
    fn encode<T: Serialize>(
        type_tag: &'static str,
        value: &T,
    ) -> Result<Self, SnowStage3V11SnowEnthalpyRestartErrorV5> {
        let canonical_json = to_canonical_bytes(value)
            .map_err(|_| SnowStage3V11SnowEnthalpyRestartErrorV5::Canonical)?;
        Ok(Self {
            type_tag: type_tag.to_owned(),
            canonical_sha256: sha(&canonical_json)?,
            canonical_json,
        })
    }

    fn decode<T>(
        &self,
        type_tag: &'static str,
    ) -> Result<T, SnowStage3V11SnowEnthalpyRestartErrorV5>
    where
        T: for<'de> Deserialize<'de> + Serialize,
    {
        if self.type_tag != type_tag || self.canonical_sha256 != sha(&self.canonical_json)? {
            return Err(SnowStage3V11SnowEnthalpyRestartErrorV5::NativeFrame);
        }
        from_canonical_bytes(&self.canonical_json)
            .map_err(|_| SnowStage3V11SnowEnthalpyRestartErrorV5::NativeFrame)
    }
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct SnowStage3V11SnowEnthalpyResidentV5 {
    pub compound_owner: NativeSnowEnthalpyFrameV5,
    pub accepted_compound_owner_chronology: Vec<NativeSnowEnthalpyFrameV5>,
    pub accepted_receipt_chronology: Vec<NativeSnowEnthalpyFrameV5>,
}

impl SnowStage3V11SnowEnthalpyResidentV5 {
    pub fn from_native(
        compound_owner: &AuthenticatedCoveredSnowMaterialOwnerV1,
        accepted_compound_owner_chronology: &[AuthenticatedCoveredSnowMaterialOwnerV1],
        accepted_receipt_chronology: &[CoveredSnowEnthalpyCarryReceiptV1],
    ) -> Result<Self, SnowStage3V11SnowEnthalpyRestartErrorV5> {
        validate_native(
            compound_owner,
            accepted_compound_owner_chronology,
            accepted_receipt_chronology,
            None,
        )?;
        Ok(Self {
            compound_owner: NativeSnowEnthalpyFrameV5::encode(OWNER_TYPE_TAG, compound_owner)?,
            accepted_compound_owner_chronology: accepted_compound_owner_chronology
                .iter()
                .map(|owner| NativeSnowEnthalpyFrameV5::encode(OWNER_TYPE_TAG, owner))
                .collect::<Result<_, _>>()?,
            accepted_receipt_chronology: accepted_receipt_chronology
                .iter()
                .map(|receipt| NativeSnowEnthalpyFrameV5::encode(RECEIPT_TYPE_TAG, receipt))
                .collect::<Result<_, _>>()?,
        })
    }

    fn decode(
        &self,
        expected_base: Option<&BTreeMap<u32, DirectSnowStage3PersistentState>>,
    ) -> Result<RestoredSnowStage3V11SnowEnthalpyResidentV5, SnowStage3V11SnowEnthalpyRestartErrorV5>
    {
        let compound_owner = self.compound_owner.decode(OWNER_TYPE_TAG)?;
        let accepted_compound_owner_chronology = self
            .accepted_compound_owner_chronology
            .iter()
            .map(|frame| frame.decode(OWNER_TYPE_TAG))
            .collect::<Result<Vec<_>, _>>()?;
        let accepted_receipt_chronology = self
            .accepted_receipt_chronology
            .iter()
            .map(|frame| frame.decode(RECEIPT_TYPE_TAG))
            .collect::<Result<Vec<_>, _>>()?;
        validate_native(
            &compound_owner,
            &accepted_compound_owner_chronology,
            &accepted_receipt_chronology,
            expected_base,
        )?;
        Ok(RestoredSnowStage3V11SnowEnthalpyResidentV5 {
            compound_owner,
            accepted_compound_owner_chronology,
            accepted_receipt_chronology,
        })
    }
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct SnowStage3V11SnowEnthalpyResidentSetV5 {
    pub committed: SnowStage3V11SnowEnthalpyResidentV5,
    pub pending_candidate: Option<SnowStage3V11SnowEnthalpyResidentV5>,
    pub in_progress_day_candidate: Option<SnowStage3V11SnowEnthalpyResidentV5>,
    pub in_progress_support_current: Option<SnowStage3V11SnowEnthalpyResidentV5>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct RestoredSnowStage3V11SnowEnthalpyResidentV5 {
    pub compound_owner: AuthenticatedCoveredSnowMaterialOwnerV1,
    pub accepted_compound_owner_chronology: Vec<AuthenticatedCoveredSnowMaterialOwnerV1>,
    pub accepted_receipt_chronology: Vec<CoveredSnowEnthalpyCarryReceiptV1>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct RestoredSnowStage3V11SnowEnthalpyResidentSetV5 {
    pub committed: RestoredSnowStage3V11SnowEnthalpyResidentV5,
    pub pending_candidate: Option<RestoredSnowStage3V11SnowEnthalpyResidentV5>,
    pub in_progress_day_candidate: Option<RestoredSnowStage3V11SnowEnthalpyResidentV5>,
    pub in_progress_support_current: Option<RestoredSnowStage3V11SnowEnthalpyResidentV5>,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct DirectSnowStage3V11SnowEnthalpyRestartV5 {
    pub schema: String,
    pub version: u16,
    pub nested_stage3_v4_bytes: Vec<u8>,
    pub nested_stage3_v4_sha256: Sha256Hex,
    pub snow_enthalpy_residents: SnowStage3V11SnowEnthalpyResidentSetV5,
    pub payload_sha256: Sha256Hex,
}

#[derive(Serialize)]
struct DigestBodyV5<'a> {
    schema: &'a str,
    version: u16,
    nested_stage3_v4_bytes: &'a [u8],
    nested_stage3_v4_sha256: &'a Sha256Hex,
    snow_enthalpy_residents: &'a SnowStage3V11SnowEnthalpyResidentSetV5,
}

pub struct ExpectedSnowStage3V11SnowEnthalpyRestartContextV5<'a> {
    pub stage3_v4: &'a ExpectedSnowStage3V11ExactEnthalpyRestartContextV4<'a>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct IsolatedRestoredSnowStage3V11SnowEnthalpyRestartV5 {
    pub persisted: DirectSnowStage3V11SnowEnthalpyRestartV5,
    pub stage3: DirectSnowStage3V11ShadowAttachment,
    pub snow_enthalpy_residents: RestoredSnowStage3V11SnowEnthalpyResidentSetV5,
}

impl DirectSnowStage3V11SnowEnthalpyRestartV5 {
    pub fn project(
        value: &DirectSnowStage3V11ShadowAttachment,
        nested_stage3_v4: &DirectSnowStage3V11ExactEnthalpyRestartV4,
    ) -> Result<Self, SnowStage3V11SnowEnthalpyRestartErrorV5> {
        let snow_enthalpy_residents = project_resident_set(value)?;
        Self::project_with_residents(value, nested_stage3_v4, snow_enthalpy_residents)
    }

    fn project_with_residents(
        value: &DirectSnowStage3V11ShadowAttachment,
        nested_stage3_v4: &DirectSnowStage3V11ExactEnthalpyRestartV4,
        snow_enthalpy_residents: SnowStage3V11SnowEnthalpyResidentSetV5,
    ) -> Result<Self, SnowStage3V11SnowEnthalpyRestartErrorV5> {
        let nested_stage3_v4_bytes = nested_stage3_v4.to_canonical_bytes().map_err(nested_v4)?;
        decode_resident_set(&snow_enthalpy_residents, value)?;
        let mut projected = Self {
            schema: DIRECT_SNOW_STAGE3_V11_SNOW_ENTHALPY_RESTART_V5_SCHEMA.to_owned(),
            version: 5,
            nested_stage3_v4_sha256: sha(&nested_stage3_v4_bytes)?,
            nested_stage3_v4_bytes,
            snow_enthalpy_residents,
            payload_sha256: zero_sha()?,
        };
        projected.payload_sha256 = projected.compute_digest()?;
        Ok(projected)
    }

    /// V4 migration requires caller-supplied authenticated owners because a
    /// restart layer may not invent transaction, support, topology, or receipt
    /// authority. This constructor proves every supplied carry is canonical
    /// exact zero before creating V5 bytes.
    pub fn migrate_v4_with_zero_carry(
        value: &DirectSnowStage3V11ShadowAttachment,
        nested_stage3_v4: &DirectSnowStage3V11ExactEnthalpyRestartV4,
        zero_carry_residents: SnowStage3V11SnowEnthalpyResidentSetV5,
    ) -> Result<Self, SnowStage3V11SnowEnthalpyRestartErrorV5> {
        let projected =
            Self::project_with_residents(value, nested_stage3_v4, zero_carry_residents)?;
        projected.require_all_zero_carry()?;
        Ok(projected)
    }

    pub fn restore(
        &self,
        context: &ExpectedSnowStage3V11SnowEnthalpyRestartContextV5<'_>,
    ) -> Result<
        IsolatedRestoredSnowStage3V11SnowEnthalpyRestartV5,
        SnowStage3V11SnowEnthalpyRestartErrorV5,
    > {
        self.validate_envelope()?;
        let nested_stage3_v4 = DirectSnowStage3V11ExactEnthalpyRestartV4::from_canonical_bytes(
            &self.nested_stage3_v4_bytes,
            context.stage3_v4,
        )
        .map_err(nested_v4)?;
        let isolated_residents = decode_isolated_resident_set(&self.snow_enthalpy_residents)?;
        let mut stage3 = nested_stage3_v4
            .restore(context.stage3_v4)
            .map_err(nested_v4)?;
        stage3
            .restart_authority_install_snow_enthalpy_material_residents_v1(native_resident_set(
                &isolated_residents,
            ))
            .map_err(|_| SnowStage3V11SnowEnthalpyRestartErrorV5::BaseMaterialJoin)?;
        let snow_enthalpy_residents = decode_resident_set(&self.snow_enthalpy_residents, &stage3)?;
        if snow_enthalpy_residents != isolated_residents {
            return Err(SnowStage3V11SnowEnthalpyRestartErrorV5::BaseMaterialJoin);
        }
        Ok(IsolatedRestoredSnowStage3V11SnowEnthalpyRestartV5 {
            persisted: self.clone(),
            stage3,
            snow_enthalpy_residents,
        })
    }

    pub fn to_canonical_bytes(&self) -> Result<Vec<u8>, SnowStage3V11SnowEnthalpyRestartErrorV5> {
        to_canonical_bytes(self).map_err(|_| SnowStage3V11SnowEnthalpyRestartErrorV5::Canonical)
    }

    pub fn from_canonical_bytes(
        bytes: &[u8],
        context: &ExpectedSnowStage3V11SnowEnthalpyRestartContextV5<'_>,
    ) -> Result<Self, SnowStage3V11SnowEnthalpyRestartErrorV5> {
        let value: Self = from_canonical_bytes(bytes)
            .map_err(|_| SnowStage3V11SnowEnthalpyRestartErrorV5::Canonical)?;
        if value.to_canonical_bytes()? != bytes {
            return Err(SnowStage3V11SnowEnthalpyRestartErrorV5::Canonical);
        }
        value.restore(context)?;
        Ok(value)
    }

    /// Return the unchanged V4 bytes only when every current and historical
    /// snow carry is exact zero. Refusal is read-only and therefore preserves
    /// the V5 checkpoint byte-for-byte.
    pub fn downgrade_to_v4_bytes(
        &self,
    ) -> Result<Vec<u8>, SnowStage3V11SnowEnthalpyRestartErrorV5> {
        self.validate_envelope()?;
        self.require_all_zero_carry()?;
        Ok(self.nested_stage3_v4_bytes.clone())
    }

    fn validate_envelope(&self) -> Result<(), SnowStage3V11SnowEnthalpyRestartErrorV5> {
        if self.schema != DIRECT_SNOW_STAGE3_V11_SNOW_ENTHALPY_RESTART_V5_SCHEMA {
            return Err(SnowStage3V11SnowEnthalpyRestartErrorV5::Schema);
        }
        if self.version != 5 {
            return Err(SnowStage3V11SnowEnthalpyRestartErrorV5::UnsupportedVersion);
        }
        if self.nested_stage3_v4_sha256 != sha(&self.nested_stage3_v4_bytes)?
            || self.payload_sha256 != self.compute_digest()?
        {
            return Err(SnowStage3V11SnowEnthalpyRestartErrorV5::PayloadDigest);
        }
        Ok(())
    }

    fn compute_digest(&self) -> Result<Sha256Hex, SnowStage3V11SnowEnthalpyRestartErrorV5> {
        let bytes = to_canonical_bytes(&DigestBodyV5 {
            schema: &self.schema,
            version: self.version,
            nested_stage3_v4_bytes: &self.nested_stage3_v4_bytes,
            nested_stage3_v4_sha256: &self.nested_stage3_v4_sha256,
            snow_enthalpy_residents: &self.snow_enthalpy_residents,
        })
        .map_err(|_| SnowStage3V11SnowEnthalpyRestartErrorV5::Canonical)?;
        sha(&bytes)
    }

    fn require_all_zero_carry(&self) -> Result<(), SnowStage3V11SnowEnthalpyRestartErrorV5> {
        for resident in resident_values(&self.snow_enthalpy_residents) {
            let restored = resident.decode(None)?;
            restored
                .compound_owner
                .refuse_nonzero_carry_downgrade()
                .map_err(|_| SnowStage3V11SnowEnthalpyRestartErrorV5::DowngradeProhibited)?;
            if restored
                .accepted_compound_owner_chronology
                .iter()
                .any(|owner| owner.refuse_nonzero_carry_downgrade().is_err())
            {
                return Err(SnowStage3V11SnowEnthalpyRestartErrorV5::DowngradeProhibited);
            }
            if restored.accepted_receipt_chronology.iter().any(|receipt| {
                receipt
                    .beginning_carries()
                    .iter()
                    .chain(receipt.ending_carries())
                    .any(|state| state.enthalpy_carry().sign != 0)
            }) {
                return Err(SnowStage3V11SnowEnthalpyRestartErrorV5::DowngradeProhibited);
            }
        }
        Ok(())
    }
}

fn decode_resident_set(
    values: &SnowStage3V11SnowEnthalpyResidentSetV5,
    stage3: &DirectSnowStage3V11ShadowAttachment,
) -> Result<RestoredSnowStage3V11SnowEnthalpyResidentSetV5, SnowStage3V11SnowEnthalpyRestartErrorV5>
{
    let pending = stage3.restart_authority_pending_candidate();
    let in_progress = stage3.restart_authority_in_progress_execution_v2();
    if values.pending_candidate.is_some() != pending.is_some()
        || values.in_progress_day_candidate.is_some() != in_progress.is_some()
        || values.in_progress_support_current.is_some() != in_progress.is_some()
        || in_progress.is_some_and(|execution| execution.support_current().is_none())
    {
        return Err(SnowStage3V11SnowEnthalpyRestartErrorV5::Posture);
    }
    let committed = values
        .committed
        .decode(Some(&stage3.committed.stage3_by_lane))?;
    let pending_candidate = values
        .pending_candidate
        .as_ref()
        .zip(pending)
        .map(|(resident, candidate)| resident.decode(Some(&candidate.ending_state.stage3_by_lane)))
        .transpose()?;
    let in_progress_day_candidate = values
        .in_progress_day_candidate
        .as_ref()
        .zip(in_progress)
        .map(|(resident, execution)| {
            resident.decode(Some(&execution.day_candidate().stage3_by_lane))
        })
        .transpose()?;
    let in_progress_support_current = values
        .in_progress_support_current
        .as_ref()
        .zip(in_progress.and_then(|execution| execution.support_current()))
        .map(|(resident, current)| resident.decode(Some(&current.stage3_by_lane)))
        .transpose()?;
    Ok(RestoredSnowStage3V11SnowEnthalpyResidentSetV5 {
        committed,
        pending_candidate,
        in_progress_day_candidate,
        in_progress_support_current,
    })
}

fn project_resident_set(
    stage3: &DirectSnowStage3V11ShadowAttachment,
) -> Result<SnowStage3V11SnowEnthalpyResidentSetV5, SnowStage3V11SnowEnthalpyRestartErrorV5> {
    let values = stage3
        .restart_authority_snow_enthalpy_material_residents_v1()
        .map_err(|_| SnowStage3V11SnowEnthalpyRestartErrorV5::CompoundOwner)?;
    Ok(SnowStage3V11SnowEnthalpyResidentSetV5 {
        committed: project_native_resident(&values.committed)?,
        pending_candidate: values
            .pending_candidate
            .as_ref()
            .map(project_native_resident)
            .transpose()?,
        in_progress_day_candidate: values
            .in_progress_day_candidate
            .as_ref()
            .map(project_native_resident)
            .transpose()?,
        in_progress_support_current: values
            .in_progress_support_current
            .as_ref()
            .map(project_native_resident)
            .transpose()?,
    })
}

fn project_native_resident(
    value: &SnowStage3V11SnowEnthalpyMaterialResidentV1,
) -> Result<SnowStage3V11SnowEnthalpyResidentV5, SnowStage3V11SnowEnthalpyRestartErrorV5> {
    let owner = value
        .current_owner
        .as_ref()
        .ok_or(SnowStage3V11SnowEnthalpyRestartErrorV5::CompoundOwner)?;
    let owners = &value.accepted_owner_chronology;
    let receipts = owners
        .iter()
        .map(|owner| owner.receipt().clone())
        .collect::<Vec<_>>();
    SnowStage3V11SnowEnthalpyResidentV5::from_native(owner, owners, &receipts)
}

fn decode_isolated_resident_set(
    values: &SnowStage3V11SnowEnthalpyResidentSetV5,
) -> Result<RestoredSnowStage3V11SnowEnthalpyResidentSetV5, SnowStage3V11SnowEnthalpyRestartErrorV5>
{
    Ok(RestoredSnowStage3V11SnowEnthalpyResidentSetV5 {
        committed: values.committed.decode(None)?,
        pending_candidate: values
            .pending_candidate
            .as_ref()
            .map(|value| value.decode(None))
            .transpose()?,
        in_progress_day_candidate: values
            .in_progress_day_candidate
            .as_ref()
            .map(|value| value.decode(None))
            .transpose()?,
        in_progress_support_current: values
            .in_progress_support_current
            .as_ref()
            .map(|value| value.decode(None))
            .transpose()?,
    })
}

fn native_resident_set(
    values: &RestoredSnowStage3V11SnowEnthalpyResidentSetV5,
) -> SnowStage3V11SnowEnthalpyMaterialResidentSetV1 {
    SnowStage3V11SnowEnthalpyMaterialResidentSetV1 {
        committed: native_resident(&values.committed),
        pending_candidate: values.pending_candidate.as_ref().map(native_resident),
        in_progress_day_candidate: values
            .in_progress_day_candidate
            .as_ref()
            .map(native_resident),
        in_progress_support_current: values
            .in_progress_support_current
            .as_ref()
            .map(native_resident),
    }
}

fn native_resident(
    value: &RestoredSnowStage3V11SnowEnthalpyResidentV5,
) -> SnowStage3V11SnowEnthalpyMaterialResidentV1 {
    SnowStage3V11SnowEnthalpyMaterialResidentV1 {
        current_owner: Some(value.compound_owner.clone()),
        accepted_owner_chronology: value.accepted_compound_owner_chronology.clone(),
    }
}

fn validate_native(
    owner: &AuthenticatedCoveredSnowMaterialOwnerV1,
    owners: &[AuthenticatedCoveredSnowMaterialOwnerV1],
    receipts: &[CoveredSnowEnthalpyCarryReceiptV1],
    expected_base: Option<&BTreeMap<u32, DirectSnowStage3PersistentState>>,
) -> Result<(), SnowStage3V11SnowEnthalpyRestartErrorV5> {
    owner
        .validate()
        .map_err(|_| SnowStage3V11SnowEnthalpyRestartErrorV5::CompoundOwner)?;
    if owners.is_empty()
        || receipts.is_empty()
        || owners.len() != receipts.len()
        || owners.iter().any(|owner| owner.validate().is_err())
        || receipts.iter().any(|receipt| receipt.validate().is_err())
        || owner
            != owners
                .last()
                .ok_or(SnowStage3V11SnowEnthalpyRestartErrorV5::ReceiptChronology)?
        || owners
            .iter()
            .zip(receipts)
            .any(|(owner, receipt)| owner.receipt() != receipt)
    {
        return Err(SnowStage3V11SnowEnthalpyRestartErrorV5::ReceiptChronology);
    }
    for index in 1..receipts.len() {
        if receipts[index]
            .validate_successor_of(&owners[index - 1])
            .is_err()
        {
            return Err(SnowStage3V11SnowEnthalpyRestartErrorV5::ReceiptChronology);
        }
    }
    if let Some(expected_base) = expected_base {
        if owner.base_material_owner() != expected_base {
            return Err(SnowStage3V11SnowEnthalpyRestartErrorV5::BaseMaterialJoin);
        }
    }
    Ok(())
}

fn resident_values(
    values: &SnowStage3V11SnowEnthalpyResidentSetV5,
) -> impl Iterator<Item = &SnowStage3V11SnowEnthalpyResidentV5> {
    std::iter::once(&values.committed)
        .chain(values.pending_candidate.iter())
        .chain(values.in_progress_day_candidate.iter())
        .chain(values.in_progress_support_current.iter())
}

fn sha(bytes: &[u8]) -> Result<Sha256Hex, SnowStage3V11SnowEnthalpyRestartErrorV5> {
    Sha256Hex::try_new(format!("{:x}", Sha256::digest(bytes)))
        .map_err(|_| SnowStage3V11SnowEnthalpyRestartErrorV5::PayloadDigest)
}

fn zero_sha() -> Result<Sha256Hex, SnowStage3V11SnowEnthalpyRestartErrorV5> {
    Sha256Hex::try_new("0".repeat(64))
        .map_err(|_| SnowStage3V11SnowEnthalpyRestartErrorV5::PayloadDigest)
}

fn nested_v4(error: SnowStage3V11RestartError) -> SnowStage3V11SnowEnthalpyRestartErrorV5 {
    SnowStage3V11SnowEnthalpyRestartErrorV5::NestedV4(error.to_string())
}

#[cfg(test)]
mod tests {
    use openwepp_coupled_time::{Digest32, ModelTimeNs, ParentTransactionId, TimeSupport};
    use openwepp_hillslope_orchestrator::{
        CoveredSnowEnthalpyCarryReceiptInputsV1, CoveredSnowEnthalpyCarryStateV1,
        CoveredSnowEnthalpyEnergyOperandKindV1, CoveredSnowEnthalpyEnergyOperandV1,
        DirectSnowLayerState, covered_snow_base_material_owner_sha256,
        covered_snow_material_candidate_sha256,
    };
    use openwepp_land_surface_energy::ExactDyadicEnthalpy;

    use super::*;

    fn digest(value: u8) -> Digest32 {
        Digest32::from_bytes([value; 32])
    }

    fn base_state(enthalpy_hi: f64) -> BTreeMap<u32, DirectSnowStage3PersistentState> {
        let cold_content = -enthalpy_hi;
        BTreeMap::from([(
            1,
            DirectSnowStage3PersistentState {
                schema_version: 2,
                terminal_event_model: None,
                fingerprint: 2,
                lane_id: 1,
                next_interval_index: 2,
                layers: vec![
                    DirectSnowLayerState::new(0.04, 0.4, 100.0, 1.0)
                        .with_stage3_thermal_liquid_state(-10.0, 0.0, cold_content, 0.0),
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
                cumulative_complete_energy_j_m2: cold_content,
                cumulative_cold_energy_change_j_m2: cold_content,
                cumulative_terminal_unallocated_energy_j_m2: 0.0,
            },
        )])
    }

    fn carry(cold: f64, nonzero: bool) -> CoveredSnowEnthalpyCarryStateV1 {
        let remainder = if nonzero {
            ExactDyadicEnthalpy::try_new(1, "1", -80).expect("sub-ULP carry")
        } else {
            ExactDyadicEnthalpy::zero()
        };
        CoveredSnowEnthalpyCarryStateV1::new(1, 0, cold, remainder, 263.15).expect("frozen carry")
    }

    fn receipt(
        base: &BTreeMap<u32, DirectSnowStage3PersistentState>,
        beginning: Vec<CoveredSnowEnthalpyCarryStateV1>,
        ending: Vec<CoveredSnowEnthalpyCarryStateV1>,
        start_ns: u128,
        predecessor_receipt: Digest32,
        beginning_compound_owner: Digest32,
        transaction_seed: u8,
    ) -> CoveredSnowEnthalpyCarryReceiptV1 {
        let candidate =
            covered_snow_material_candidate_sha256(base, &ending).expect("candidate digest");
        CoveredSnowEnthalpyCarryReceiptV1::seal(CoveredSnowEnthalpyCarryReceiptInputsV1 {
            support: TimeSupport::new(ModelTimeNs::new(start_ns), ModelTimeNs::new(start_ns + 60))
                .expect("support"),
            transaction_id: ParentTransactionId::from_digest(digest(transaction_seed)),
            predecessor_transaction_id: Some(ParentTransactionId::from_digest(digest(
                transaction_seed.saturating_sub(1).max(1),
            ))),
            beginning_carries: beginning,
            ending_carries: ending,
            ordered_energy_operands: vec![
                CoveredSnowEnthalpyEnergyOperandV1::new(
                    0,
                    CoveredSnowEnthalpyEnergyOperandKindV1::SnowSoilCrankNicolson,
                    0.0,
                )
                .expect("operand"),
            ],
            base_material_owner_sha256: covered_snow_base_material_owner_sha256(base)
                .expect("base digest"),
            beginning_compound_owner_sha256: beginning_compound_owner,
            predecessor_receipt_chain_sha256: predecessor_receipt,
            branch_identity_sha256: digest(92),
            topology_identity_sha256: digest(93),
            configuration_identity_sha256: digest(94),
            custody_identity_sha256: digest(95),
            candidate_sha256: candidate,
        })
        .expect("receipt")
    }

    fn owner_and_chronology(
        nonzero: bool,
    ) -> (
        AuthenticatedCoveredSnowMaterialOwnerV1,
        Vec<AuthenticatedCoveredSnowMaterialOwnerV1>,
        Vec<CoveredSnowEnthalpyCarryReceiptV1>,
    ) {
        let base = base_state(-8_000.0);
        let zero = vec![carry(-8_000.0, false)];
        let first = receipt(
            &base,
            zero.clone(),
            zero.clone(),
            60,
            digest(90),
            digest(91),
            11,
        );
        let first_owner = AuthenticatedCoveredSnowMaterialOwnerV1::seal(
            base.clone(),
            zero.clone(),
            first.clone(),
        )
        .expect("first owner");
        let ending = vec![carry(-8_000.0, nonzero)];
        let second = receipt(
            &base,
            zero,
            ending.clone(),
            120,
            first.receipt_sha256(),
            first_owner.compound_owner_sha256(),
            12,
        );
        let owner = AuthenticatedCoveredSnowMaterialOwnerV1::seal(base, ending, second.clone())
            .expect("owner");
        (owner.clone(), vec![first_owner, owner], vec![first, second])
    }

    fn resident(nonzero: bool) -> SnowStage3V11SnowEnthalpyResidentV5 {
        let (owner, owners, receipts) = owner_and_chronology(nonzero);
        SnowStage3V11SnowEnthalpyResidentV5::from_native(&owner, &owners, &receipts)
            .expect("resident")
    }

    fn checkpoint(
        resident: SnowStage3V11SnowEnthalpyResidentV5,
    ) -> DirectSnowStage3V11SnowEnthalpyRestartV5 {
        let nested = b"canonical-v4-placeholder".to_vec();
        let mut value = DirectSnowStage3V11SnowEnthalpyRestartV5 {
            schema: DIRECT_SNOW_STAGE3_V11_SNOW_ENTHALPY_RESTART_V5_SCHEMA.to_owned(),
            version: 5,
            nested_stage3_v4_sha256: sha(&nested).expect("nested digest"),
            nested_stage3_v4_bytes: nested,
            snow_enthalpy_residents: SnowStage3V11SnowEnthalpyResidentSetV5 {
                committed: resident,
                pending_candidate: None,
                in_progress_day_candidate: None,
                in_progress_support_current: None,
            },
            payload_sha256: zero_sha().expect("seed"),
        };
        value.payload_sha256 = value.compute_digest().expect("payload");
        value
    }

    #[test]
    fn v5_schema_is_additive_and_distinct_from_v4() {
        assert_eq!(
            DIRECT_SNOW_STAGE3_V11_SNOW_ENTHALPY_RESTART_V5_SCHEMA,
            "OPENWEPP_SNOW_STAGE3_V11_SNOW_ENTHALPY_RESTART_V5"
        );
        assert_ne!(
            DIRECT_SNOW_STAGE3_V11_SNOW_ENTHALPY_RESTART_V5_SCHEMA,
            crate::DIRECT_SNOW_STAGE3_V11_EXACT_ENTHALPY_RESTART_V4_SCHEMA
        );
    }

    #[test]
    fn v5_wire_refuses_partial_unknown_or_misordered_members() {
        let partial = br#"{"schema":"OPENWEPP_SNOW_STAGE3_V11_SNOW_ENTHALPY_RESTART_V5","version":5,"unknown":true}"#;
        assert!(from_canonical_bytes::<DirectSnowStage3V11SnowEnthalpyRestartV5>(partial).is_err());
        let misordered = br#"{"version":5,"schema":"OPENWEPP_SNOW_STAGE3_V11_SNOW_ENTHALPY_RESTART_V5","nested_stage3_v4_bytes":[],"nested_stage3_v4_sha256":"0000000000000000000000000000000000000000000000000000000000000000","snow_enthalpy_residents":{},"payload_sha256":"0000000000000000000000000000000000000000000000000000000000000000"}"#;
        assert!(
            from_canonical_bytes::<DirectSnowStage3V11SnowEnthalpyRestartV5>(misordered).is_err()
        );
    }

    #[test]
    fn native_frame_requires_type_digest_and_exact_canonical_bytes() {
        #[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
        #[serde(deny_unknown_fields)]
        struct Probe {
            lane_id: u32,
        }
        let frame =
            NativeSnowEnthalpyFrameV5::encode("Probe", &Probe { lane_id: 7 }).expect("frame");
        assert_eq!(frame.decode::<Probe>("Probe").expect("decode").lane_id, 7);
        assert_eq!(
            frame.decode::<Probe>("Foreign"),
            Err(SnowStage3V11SnowEnthalpyRestartErrorV5::NativeFrame)
        );
        let mut corrupt = frame;
        corrupt.canonical_json.push(b' ');
        corrupt.canonical_sha256 = sha(&corrupt.canonical_json).expect("reseal poison");
        assert_eq!(
            corrupt.decode::<Probe>("Probe"),
            Err(SnowStage3V11SnowEnthalpyRestartErrorV5::NativeFrame)
        );
    }

    #[test]
    fn receipt_chronology_round_trips_and_refuses_stale_order_or_cross_support() {
        let (owner, owners, chronology) = owner_and_chronology(false);
        let resident =
            SnowStage3V11SnowEnthalpyResidentV5::from_native(&owner, &owners, &chronology)
                .expect("chronology");
        let replay = resident
            .decode(Some(owner.base_material_owner()))
            .expect("exact restored joins");
        assert_eq!(replay.compound_owner, owner);
        assert_eq!(replay.accepted_compound_owner_chronology, owners);
        assert_eq!(replay.accepted_receipt_chronology, chronology);

        let mut stale = chronology.clone();
        stale[1] = receipt(
            owner.base_material_owner(),
            stale[0].ending_carries().to_vec(),
            stale[1].ending_carries().to_vec(),
            120,
            digest(77),
            owners[0].compound_owner_sha256(),
            12,
        );
        assert_eq!(
            validate_native(&owner, &owners, &stale, None),
            Err(SnowStage3V11SnowEnthalpyRestartErrorV5::ReceiptChronology)
        );

        let reversed = vec![chronology[1].clone(), chronology[0].clone()];
        let reversed_owners = vec![owners[1].clone(), owners[0].clone()];
        assert_eq!(
            validate_native(&owner, &reversed_owners, &reversed, None),
            Err(SnowStage3V11SnowEnthalpyRestartErrorV5::ReceiptChronology)
        );

        let foreign = base_state(-8_001.0);
        assert_eq!(
            validate_native(&owner, &owners, &chronology, Some(&foreign)),
            Err(SnowStage3V11SnowEnthalpyRestartErrorV5::BaseMaterialJoin)
        );
    }

    #[test]
    fn zero_carry_downgrade_and_split_wire_equivalence_are_exact() {
        let checkpoint = checkpoint(resident(false));
        let before = checkpoint.to_canonical_bytes().expect("V5 bytes");
        assert_eq!(
            checkpoint.downgrade_to_v4_bytes().expect("zero downgrade"),
            b"canonical-v4-placeholder"
        );
        let replay: DirectSnowStage3V11SnowEnthalpyRestartV5 =
            from_canonical_bytes(&before).expect("split-run wire replay");
        assert_eq!(replay, checkpoint);
        assert_eq!(replay.to_canonical_bytes().expect("replayed bytes"), before);
    }

    #[test]
    fn nonzero_current_or_historical_carry_blocks_downgrade_without_mutation() {
        let checkpoint = checkpoint(resident(true));
        let before = checkpoint.to_canonical_bytes().expect("before refusal");
        assert_eq!(
            checkpoint.downgrade_to_v4_bytes(),
            Err(SnowStage3V11SnowEnthalpyRestartErrorV5::DowngradeProhibited)
        );
        assert_eq!(
            checkpoint.to_canonical_bytes().expect("after refusal"),
            before,
            "fail-closed downgrade must not mutate V5 custody"
        );
    }

    #[test]
    fn every_live_posture_restores_from_the_persisted_compound_chronology() {
        let (owner, owners, receipts) = owner_and_chronology(false);
        let restored = RestoredSnowStage3V11SnowEnthalpyResidentV5 {
            compound_owner: owner.clone(),
            accepted_compound_owner_chronology: owners.clone(),
            accepted_receipt_chronology: receipts,
        };
        let values = RestoredSnowStage3V11SnowEnthalpyResidentSetV5 {
            committed: restored.clone(),
            pending_candidate: Some(restored.clone()),
            in_progress_day_candidate: Some(restored.clone()),
            in_progress_support_current: Some(restored),
        };
        let native = native_resident_set(&values);
        for resident in [
            Some(&native.committed),
            native.pending_candidate.as_ref(),
            native.in_progress_day_candidate.as_ref(),
            native.in_progress_support_current.as_ref(),
        ] {
            let resident = resident.expect("complete live posture");
            assert_eq!(resident.current_owner.as_ref(), Some(&owner));
            assert_eq!(resident.accepted_owner_chronology, owners);
        }
    }

    #[test]
    fn persisted_v5_contains_no_microstepping_diagnostics() {
        let bytes = checkpoint(resident(false))
            .to_canonical_bytes()
            .expect("V5 bytes");
        let wire = String::from_utf8(bytes).expect("JSON wire");
        for forbidden in [
            "microstep",
            "diagnostic",
            "iteration_count",
            "residual",
            "evaluation_budget",
            "rejection_reason",
        ] {
            assert!(
                !wire.contains(forbidden),
                "production V5 persisted forbidden diagnostic field {forbidden}"
            );
        }
    }
}
