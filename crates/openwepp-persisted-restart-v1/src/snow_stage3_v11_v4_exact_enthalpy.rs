//! Additive V16 exact-enthalpy supplement for the unchanged Stage-3/V3 wire.

use base64::{Engine as _, engine::general_purpose::STANDARD};
use openwepp_hillslope_orchestrator::{
    SurfaceLiquidCompleteOwnerProjectionV4,
    snow_stage3_v11_attachment::{
        DirectSnowStage3V11ShadowAttachment,
        restart_authority_encode_publication_rotation_state_v3,
        restart_authority_encode_support_liquid_custody_state_v3,
        restart_authority_restore_publication_rotation_state_v3,
        restart_authority_restore_support_liquid_custody_state_v3,
    },
    v9_real_consumer_shadow::{
        DirectV10RealConsumerShadow, FrozenLitterV3Resident, FrozenLitterV4Resident,
    },
};
use serde::{Deserialize, Serialize};
use sha2::{Digest as _, Sha256};

use crate::snow_stage3_v11::{
    SnowStage3V11ExactResidentPositionV4, SnowStage3V11ExactResidentRestorerV4,
};
use crate::{
    DirectFrozenLitterExactEnthalpyCheckpointV4, DirectSnowStage3V11AttachmentRestartV2,
    DirectSnowStage3V11AttachmentRestartV3, ExpectedFrozenLitterExactEnthalpyContextV4,
    ExpectedSnowStage3V11RestartContext, ExpectedStage3CommittedDayArchiveV3, Sha256Hex,
    SnowStage3V11RestartError, admit_frozen_litter_exact_enthalpy_checkpoint_v4,
    from_canonical_bytes, to_canonical_bytes,
};

pub const DIRECT_SNOW_STAGE3_V11_EXACT_ENTHALPY_RESTART_V4_SCHEMA: &str =
    "OPENWEPP_SNOW_STAGE3_V11_EXACT_ENTHALPY_RESTART_V4";

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct SnowStage3V11ExactResidentSupplementV4 {
    pub checkpoint: DirectFrozenLitterExactEnthalpyCheckpointV4,
    pub physical_v3_publication_bytes: Vec<Vec<u8>>,
    pub exact_v4_publication_bytes: Vec<Vec<u8>>,
}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct SnowStage3V11ExactResidentSetV4 {
    pub committed: SnowStage3V11ExactResidentSupplementV4,
    pub pending_candidate: Option<SnowStage3V11ExactResidentSupplementV4>,
    pub in_progress_day_candidate: Option<SnowStage3V11ExactResidentSupplementV4>,
    pub in_progress_support_current: Option<SnowStage3V11ExactResidentSupplementV4>,
}

pub struct ExpectedSnowStage3V11ExactResidentContextsV4<'a> {
    pub committed: &'a ExpectedFrozenLitterExactEnthalpyContextV4<'a>,
    pub pending_candidate: Option<&'a ExpectedFrozenLitterExactEnthalpyContextV4<'a>>,
    pub in_progress_day_candidate: Option<&'a ExpectedFrozenLitterExactEnthalpyContextV4<'a>>,
    pub in_progress_support_current: Option<&'a ExpectedFrozenLitterExactEnthalpyContextV4<'a>>,
}

pub struct ExpectedSnowStage3V11ExactEnthalpyRestartContextV4<'a> {
    pub stage3: &'a ExpectedSnowStage3V11RestartContext<'a>,
    pub archive: &'a ExpectedStage3CommittedDayArchiveV3<'a>,
    pub exact: ExpectedSnowStage3V11ExactResidentContextsV4<'a>,
}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct DirectSnowStage3V11ExactEnthalpyRestartV4 {
    pub schema: String,
    pub version: u16,
    pub nested_stage3_v3_bytes: Vec<u8>,
    pub nested_stage3_v3_sha256: Sha256Hex,
    pub exact_residents: SnowStage3V11ExactResidentSetV4,
    pub payload_sha256: Sha256Hex,
}

#[derive(Serialize)]
struct DigestBody<'a> {
    schema: &'a str,
    version: u16,
    nested_stage3_v3_bytes: &'a [u8],
    nested_stage3_v3_sha256: &'a Sha256Hex,
    exact_residents: &'a SnowStage3V11ExactResidentSetV4,
}

#[derive(Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
struct Stage3V3WireProbe {
    schema: String,
    version: u16,
    static_context_sha256: Sha256Hex,
    archive_record_count: u64,
    archive_content_root_sha256: openwepp_coupled_time::Digest32,
    archived_receipt_prefix:
        openwepp_hillslope_orchestrator::snow_stage3_v11_attachment::Stage3ArchivedReceiptPrefixV1,
    active_attachment_v2_canonical_base64: String,
    active_attachment_v2_sha256: Sha256Hex,
    support_liquid_custody_canonical_base64: String,
    support_liquid_custody_sha256: Sha256Hex,
    publication_rotation_canonical_base64: String,
    publication_rotation_sha256: Sha256Hex,
    payload_sha256: Sha256Hex,
}

impl DirectSnowStage3V11ExactEnthalpyRestartV4 {
    #[allow(clippy::too_many_arguments)]
    pub fn project(
        value: &DirectSnowStage3V11ShadowAttachment,
        phase_plan_sha256: &Sha256Hex,
        day_input_digests: &[Sha256Hex],
        archive: &ExpectedStage3CommittedDayArchiveV3<'_>,
        exact_residents: SnowStage3V11ExactResidentSetV4,
        exact_contexts: &ExpectedSnowStage3V11ExactResidentContextsV4<'_>,
    ) -> Result<Self, SnowStage3V11RestartError> {
        validate_posture(value, &exact_residents, exact_contexts)?;
        validate_live_residents(value, &exact_residents, exact_contexts)?;
        let nested = DirectSnowStage3V11AttachmentRestartV3::project(
            value,
            phase_plan_sha256,
            day_input_digests,
            archive,
        )?
        .to_canonical_bytes()?;
        let mut projected = Self {
            schema: DIRECT_SNOW_STAGE3_V11_EXACT_ENTHALPY_RESTART_V4_SCHEMA.to_owned(),
            version: 4,
            nested_stage3_v3_sha256: sha(&nested)?,
            nested_stage3_v3_bytes: nested,
            exact_residents,
            payload_sha256: zero_sha()?,
        };
        projected.payload_sha256 = projected.compute_digest()?;
        Ok(projected)
    }

    pub fn restore(
        &self,
        context: &ExpectedSnowStage3V11ExactEnthalpyRestartContextV4<'_>,
    ) -> Result<DirectSnowStage3V11ShadowAttachment, SnowStage3V11RestartError> {
        self.validate_envelope()?;
        validate_supplement_context_posture(&self.exact_residents, &context.exact)?;
        let probe: Stage3V3WireProbe =
            from_canonical_bytes(&self.nested_stage3_v3_bytes).map_err(nested)?;
        validate_probe_shape(&probe)?;
        let active = decode_blob(
            &probe.active_attachment_v2_canonical_base64,
            &probe.active_attachment_v2_sha256,
        )?;
        let active_v2: DirectSnowStage3V11AttachmentRestartV2 =
            from_canonical_bytes(&active).map_err(nested)?;
        if active_v2.to_canonical_bytes()? != active
            || active_v2.static_context_sha256 != probe.static_context_sha256
        {
            return Err(SnowStage3V11RestartError::Identity(
                "V4 nested V3/active-V2 canonical join",
            ));
        }
        let exact_installer = ExactResidentRestorer {
            values: &self.exact_residents,
            contexts: &context.exact,
        };
        let mut restored = active_v2.restore_active_base_v4(context.stage3, &exact_installer)?;
        restored
            .restart_authority_install_archived_receipt_prefix_v3(
                probe.archived_receipt_prefix.clone(),
            )
            .map_err(nested)?;
        let custody = decode_blob(
            &probe.support_liquid_custody_canonical_base64,
            &probe.support_liquid_custody_sha256,
        )?;
        restart_authority_restore_support_liquid_custody_state_v3(&mut restored, &custody)
            .map_err(nested)?;
        let rotation = decode_blob(
            &probe.publication_rotation_canonical_base64,
            &probe.publication_rotation_sha256,
        )?;
        restart_authority_restore_publication_rotation_state_v3(&mut restored, &rotation)
            .map_err(nested)?;

        // Reprojection proves the complete V3 archive/custody/rotation state
        // and every unchanged nested byte after the exact residents exist.
        let replay = DirectSnowStage3V11AttachmentRestartV3::project(
            &restored,
            context.stage3.real_consumer_context.phase_plan_sha256,
            context.stage3.real_consumer_context.day_input_digests,
            context.archive,
        )?
        .to_canonical_bytes()?;
        if replay != self.nested_stage3_v3_bytes
            || restart_authority_encode_support_liquid_custody_state_v3(&restored)
                .map_err(nested)?
                != custody
            || restart_authority_encode_publication_rotation_state_v3(&restored).map_err(nested)?
                != rotation
        {
            return Err(SnowStage3V11RestartError::Identity(
                "V4 exact resident changed nested V3 bytes",
            ));
        }
        validate_live_residents(&restored, &self.exact_residents, &context.exact)?;
        Ok(restored)
    }

    pub fn to_canonical_bytes(&self) -> Result<Vec<u8>, SnowStage3V11RestartError> {
        to_canonical_bytes(self).map_err(nested)
    }

    pub fn from_canonical_bytes(
        bytes: &[u8],
        context: &ExpectedSnowStage3V11ExactEnthalpyRestartContextV4<'_>,
    ) -> Result<Self, SnowStage3V11RestartError> {
        let value: Self = from_canonical_bytes(bytes).map_err(nested)?;
        if value.to_canonical_bytes()? != bytes {
            return Err(SnowStage3V11RestartError::Identity("V4 canonical bytes"));
        }
        value.restore(context)?;
        Ok(value)
    }

    fn validate_envelope(&self) -> Result<(), SnowStage3V11RestartError> {
        if self.schema != DIRECT_SNOW_STAGE3_V11_EXACT_ENTHALPY_RESTART_V4_SCHEMA
            || self.version != 4
            || self.nested_stage3_v3_sha256 != sha(&self.nested_stage3_v3_bytes)?
            || self.payload_sha256 != self.compute_digest()?
        {
            return Err(SnowStage3V11RestartError::Identity(
                "V4 schema, nested V3, or payload digest",
            ));
        }
        Ok(())
    }

    fn compute_digest(&self) -> Result<Sha256Hex, SnowStage3V11RestartError> {
        let bytes = to_canonical_bytes(&DigestBody {
            schema: &self.schema,
            version: self.version,
            nested_stage3_v3_bytes: &self.nested_stage3_v3_bytes,
            nested_stage3_v3_sha256: &self.nested_stage3_v3_sha256,
            exact_residents: &self.exact_residents,
        })
        .map_err(nested)?;
        sha(&bytes)
    }

    pub(crate) fn nested_active_v2_bytes(&self) -> Result<Vec<u8>, SnowStage3V11RestartError> {
        self.validate_envelope()?;
        let probe: Stage3V3WireProbe =
            from_canonical_bytes(&self.nested_stage3_v3_bytes).map_err(nested)?;
        validate_probe_shape(&probe)?;
        decode_blob(
            &probe.active_attachment_v2_canonical_base64,
            &probe.active_attachment_v2_sha256,
        )
    }
}

struct ExactResidentRestorer<'a> {
    values: &'a SnowStage3V11ExactResidentSetV4,
    contexts: &'a ExpectedSnowStage3V11ExactResidentContextsV4<'a>,
}

impl SnowStage3V11ExactResidentRestorerV4 for ExactResidentRestorer<'_> {
    fn restore_exact_resident(
        &self,
        position: SnowStage3V11ExactResidentPositionV4,
        consumer: &mut DirectV10RealConsumerShadow,
    ) -> Result<(), SnowStage3V11RestartError> {
        let (supplement, context) = select(self.values, self.contexts, position)?;
        let checkpoint_bytes = to_canonical_bytes(&supplement.checkpoint).map_err(nested)?;
        let admitted = admit_frozen_litter_exact_enthalpy_checkpoint_v4(&checkpoint_bytes, context)
            .map_err(nested)?;
        let physical_context = &context.parent_v3.scientific;
        let mut physical = FrozenLitterV3Resident::try_new(
            physical_context.lse_configuration.clone(),
            admitted.parent_v3.scientific.lse_v3.clone(),
            physical_context.surface_liquid_configuration.clone(),
            admitted.parent_v3.scientific.surface_liquid_v2.clone(),
        )
        .map_err(nested)?;
        let wb14_parent_bytes = &admitted
            .parent_v3
            .scientific
            .wb14_v2_parent_working_state_bytes;
        physical
            .restore_restart_authority(
                &supplement.physical_v3_publication_bytes,
                (!wb14_parent_bytes.is_empty()).then_some(wb14_parent_bytes.as_slice()),
                admitted
                    .parent_v3
                    .persisted
                    .scientific
                    .publication_authority
                    .receipt_chain_sha256
                    .as_str(),
            )
            .map_err(nested)?;
        let exact = FrozenLitterV4Resident::try_restore(
            &physical,
            admitted.exact_surface_owner,
            &supplement.exact_v4_publication_bytes,
            context.publication_history_beginning_lse_v3_state_sha256,
        )
        .map_err(nested)?;
        consumer
            .install_restored_frozen_litter_v4_residents(physical, exact)
            .map_err(nested)
    }
}

fn select<'a>(
    values: &'a SnowStage3V11ExactResidentSetV4,
    contexts: &'a ExpectedSnowStage3V11ExactResidentContextsV4<'a>,
    position: SnowStage3V11ExactResidentPositionV4,
) -> Result<
    (
        &'a SnowStage3V11ExactResidentSupplementV4,
        &'a ExpectedFrozenLitterExactEnthalpyContextV4<'a>,
    ),
    SnowStage3V11RestartError,
> {
    let pair = match position {
        SnowStage3V11ExactResidentPositionV4::Committed => {
            Some((&values.committed, contexts.committed))
        }
        SnowStage3V11ExactResidentPositionV4::PendingCandidate => values
            .pending_candidate
            .as_ref()
            .zip(contexts.pending_candidate),
        SnowStage3V11ExactResidentPositionV4::InProgressDayCandidate => values
            .in_progress_day_candidate
            .as_ref()
            .zip(contexts.in_progress_day_candidate),
        SnowStage3V11ExactResidentPositionV4::InProgressSupportCurrent => values
            .in_progress_support_current
            .as_ref()
            .zip(contexts.in_progress_support_current),
    };
    pair.ok_or(SnowStage3V11RestartError::Identity(
        "missing V4 exact-resident supplement/context",
    ))
}

fn validate_posture(
    value: &DirectSnowStage3V11ShadowAttachment,
    supplements: &SnowStage3V11ExactResidentSetV4,
    contexts: &ExpectedSnowStage3V11ExactResidentContextsV4<'_>,
) -> Result<(), SnowStage3V11RestartError> {
    let pending = value.restart_authority_pending_candidate().is_some();
    let in_progress = value.restart_authority_in_progress_execution_v2();
    if supplements.pending_candidate.is_some() != pending
        || contexts.pending_candidate.is_some() != pending
        || supplements.in_progress_day_candidate.is_some() != in_progress.is_some()
        || contexts.in_progress_day_candidate.is_some() != in_progress.is_some()
        || supplements.in_progress_support_current.is_some() != in_progress.is_some()
        || contexts.in_progress_support_current.is_some() != in_progress.is_some()
        || in_progress.is_some_and(|execution| execution.support_current().is_none())
    {
        return Err(SnowStage3V11RestartError::Identity(
            "V4 exact-resident supplement posture",
        ));
    }
    Ok(())
}

fn validate_supplement_context_posture(
    supplements: &SnowStage3V11ExactResidentSetV4,
    contexts: &ExpectedSnowStage3V11ExactResidentContextsV4<'_>,
) -> Result<(), SnowStage3V11RestartError> {
    if supplements.pending_candidate.is_some() != contexts.pending_candidate.is_some()
        || supplements.in_progress_day_candidate.is_some()
            != contexts.in_progress_day_candidate.is_some()
        || supplements.in_progress_support_current.is_some()
            != contexts.in_progress_support_current.is_some()
    {
        return Err(SnowStage3V11RestartError::Identity(
            "V4 supplement/context posture",
        ));
    }
    Ok(())
}

fn validate_live_residents(
    value: &DirectSnowStage3V11ShadowAttachment,
    supplements: &SnowStage3V11ExactResidentSetV4,
    contexts: &ExpectedSnowStage3V11ExactResidentContextsV4<'_>,
) -> Result<(), SnowStage3V11RestartError> {
    validate_live_consumer(
        &value.committed.real_consumer,
        &supplements.committed,
        contexts.committed,
    )?;
    if let Some(candidate) = value.restart_authority_pending_candidate() {
        let (supplement, context) = supplements
            .pending_candidate
            .as_ref()
            .zip(contexts.pending_candidate)
            .ok_or(SnowStage3V11RestartError::Identity(
                "pending V4 exact resident",
            ))?;
        validate_live_consumer(&candidate.ending_state.real_consumer, supplement, context)?;
    }
    if let Some(execution) = value.restart_authority_in_progress_execution_v2() {
        let (day_supplement, day_context) = supplements
            .in_progress_day_candidate
            .as_ref()
            .zip(contexts.in_progress_day_candidate)
            .ok_or(SnowStage3V11RestartError::Identity(
                "in-progress day V4 exact resident",
            ))?;
        validate_live_consumer(
            &execution.day_candidate().real_consumer,
            day_supplement,
            day_context,
        )?;
        let (support_supplement, support_context) = supplements
            .in_progress_support_current
            .as_ref()
            .zip(contexts.in_progress_support_current)
            .ok_or(SnowStage3V11RestartError::Identity(
                "in-progress support V4 exact resident",
            ))?;
        validate_live_consumer(
            &execution
                .support_current()
                .ok_or(SnowStage3V11RestartError::Identity(
                    "in-progress support owner",
                ))?
                .real_consumer,
            support_supplement,
            support_context,
        )?;
    }
    Ok(())
}

fn validate_live_consumer(
    consumer: &DirectV10RealConsumerShadow,
    supplement: &SnowStage3V11ExactResidentSupplementV4,
    context: &ExpectedFrozenLitterExactEnthalpyContextV4<'_>,
) -> Result<(), SnowStage3V11RestartError> {
    let bytes = to_canonical_bytes(&supplement.checkpoint).map_err(nested)?;
    let admitted =
        admit_frozen_litter_exact_enthalpy_checkpoint_v4(&bytes, context).map_err(nested)?;
    let physical =
        consumer
            .frozen_litter_v3_resident()
            .ok_or(SnowStage3V11RestartError::Identity(
                "missing production V3 resident",
            ))?;
    let exact = consumer
        .frozen_litter_v4_resident()
        .ok_or(SnowStage3V11RestartError::Identity(
            "missing production V4 exact resident",
        ))?;
    let expected_wb14 = (!admitted
        .parent_v3
        .scientific
        .wb14_v2_parent_working_state_bytes
        .is_empty())
    .then(|| {
        admitted
            .parent_v3
            .scientific
            .wb14_v2_parent_working_state_bytes
            .clone()
    });
    if physical.lse_state() != &admitted.parent_v3.scientific.lse_v3
        || physical.surface_owner() != &admitted.parent_v3.scientific.surface_liquid_v2
        || physical
            .accepted_publication_supports_canonical_bytes()
            .map_err(nested)?
            != supplement.physical_v3_publication_bytes
        || physical
            .restart_wb14_parent_working_state_bytes()
            .map_err(nested)?
            != expected_wb14
        || exact.exact_surface_owner() != &admitted.exact_surface_owner
        || exact.accepted_publication_supports_canonical_bytes()
            != supplement.exact_v4_publication_bytes
    {
        return Err(SnowStage3V11RestartError::Identity(
            "V4 supplemental/live resident join",
        ));
    }
    if let Some(projection) = admitted.complete_owner_projection_v4 {
        let last = supplement.exact_v4_publication_bytes.last().ok_or(
            SnowStage3V11RestartError::Identity("accepted V4 checkpoint publication history"),
        )?;
        let replay = SurfaceLiquidCompleteOwnerProjectionV4::from_canonical_bytes(
            physical.surface_configuration(),
            last,
            context
                .accepted_support_beginning_lse_v3_state_sha256
                .as_str(),
        )
        .map_err(nested)?;
        if replay != projection {
            return Err(SnowStage3V11RestartError::Identity(
                "V4 checkpoint/latest publication join",
            ));
        }
    } else if !supplement.exact_v4_publication_bytes.is_empty() {
        return Err(SnowStage3V11RestartError::Identity(
            "receipt-free V4 checkpoint publication history",
        ));
    }
    Ok(())
}

fn validate_probe_shape(probe: &Stage3V3WireProbe) -> Result<(), SnowStage3V11RestartError> {
    if probe.schema != "OPENWEPP_SNOW_STAGE3_V11_ATTACHMENT_RESTART_V3"
        || probe.version != 3
        || usize::try_from(probe.archive_record_count).ok()
            != Some(probe.archived_receipt_prefix.archived_day_count)
        || probe.archive_content_root_sha256
            != probe.archived_receipt_prefix.archive_content_root_sha256
        || probe
            .payload_sha256
            .as_str()
            .chars()
            .all(|byte| byte == '0')
    {
        return Err(SnowStage3V11RestartError::Identity(
            "V4 nested V3 envelope shape",
        ));
    }
    Ok(())
}

fn decode_blob(encoded: &str, digest: &Sha256Hex) -> Result<Vec<u8>, SnowStage3V11RestartError> {
    let bytes = STANDARD
        .decode(encoded)
        .map_err(|_| SnowStage3V11RestartError::Identity("V4 nested V3 base64"))?;
    if STANDARD.encode(&bytes) != encoded || &sha(&bytes)? != digest {
        return Err(SnowStage3V11RestartError::Identity(
            "V4 nested V3 blob digest",
        ));
    }
    Ok(bytes)
}

fn sha(bytes: &[u8]) -> Result<Sha256Hex, SnowStage3V11RestartError> {
    Sha256Hex::try_new(format!("{:x}", Sha256::digest(bytes)))
        .map_err(|_| SnowStage3V11RestartError::Projection("V4 digest"))
}

fn zero_sha() -> Result<Sha256Hex, SnowStage3V11RestartError> {
    Sha256Hex::try_new("0".repeat(64))
        .map_err(|_| SnowStage3V11RestartError::Projection("V4 digest seed"))
}

fn nested(error: impl std::fmt::Display) -> SnowStage3V11RestartError {
    SnowStage3V11RestartError::Nested(error.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn stage3_v4_is_additive_and_does_not_alias_v3() {
        assert_eq!(
            DIRECT_SNOW_STAGE3_V11_EXACT_ENTHALPY_RESTART_V4_SCHEMA,
            "OPENWEPP_SNOW_STAGE3_V11_EXACT_ENTHALPY_RESTART_V4"
        );
        assert_ne!(
            DIRECT_SNOW_STAGE3_V11_EXACT_ENTHALPY_RESTART_V4_SCHEMA,
            "OPENWEPP_SNOW_STAGE3_V11_ATTACHMENT_RESTART_V3"
        );
    }

    #[test]
    fn stage3_v4_wire_rejects_partial_or_unknown_supplements() {
        let partial = br#"{"schema":"OPENWEPP_SNOW_STAGE3_V11_EXACT_ENTHALPY_RESTART_V4","version":4,"nested_stage3_v3_bytes":[],"unknown":true}"#;
        assert!(
            from_canonical_bytes::<DirectSnowStage3V11ExactEnthalpyRestartV4>(partial).is_err()
        );
    }

    #[test]
    fn nested_blob_digest_and_canonical_base64_are_both_mandatory() {
        let bytes = b"unchanged-stage3-v3";
        let digest = sha(bytes).expect("digest");
        let encoded = STANDARD.encode(bytes);
        assert_eq!(decode_blob(&encoded, &digest).expect("decode"), bytes);
        assert!(decode_blob(&format!("{encoded}="), &digest).is_err());
        assert!(decode_blob(&encoded, &sha(b"substituted").expect("poison digest")).is_err());
    }
}
