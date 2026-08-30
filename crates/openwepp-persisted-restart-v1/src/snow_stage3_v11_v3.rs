//! Additive bounded-history restart for the Stage-3/V11 attachment.
//!
//! V3 preserves V2 as the active-tail owner projection, but admits it only
//! beside an exact archived-prefix anchor, independently supplied archive
//! evidence, and the publication-rotation checkpoint. V1/V2 bytes and their
//! admission behavior are unchanged.

use std::collections::BTreeSet;

use base64::{Engine as _, engine::general_purpose::STANDARD};
use openwepp_coupled_time::{Digest32, digest_bytes};
use openwepp_hillslope_orchestrator::snow_stage3_v11_attachment::{
    DirectSnowStage3V11ShadowAttachment, Stage3ArchivedReceiptPrefixV1,
    Stage3CommittedDayArchiveEntryV1, Stage3CommittedDayArchiveManifestV1,
    restart_authority_encode_publication_rotation_state_v3,
    restart_authority_encode_support_liquid_custody_state_v3,
    restart_authority_restore_publication_rotation_state_v3,
    restart_authority_restore_support_liquid_custody_state_v3,
};
#[cfg(all(test, feature = "fixtures"))]
use openwepp_hillslope_orchestrator::snow_stage3_v11_attachment::{
    RestartAuthoritySupportLiquidCustodyPoisonV3, RestartAuthorityTerminalLiquidCustodyPoisonV3,
    restart_authority_poison_support_liquid_custody_state_v3,
    restart_authority_poison_terminal_liquid_custody_state_v3,
};
use serde::{Deserialize, Serialize};
use sha2::Digest as _;

use crate::{
    DirectSnowStage3V11AttachmentRestartV2, ExpectedSnowStage3V11RestartContext, Sha256Hex,
    SnowStage3V11RestartError, canonical_sha256, from_canonical_bytes, to_canonical_bytes,
};

const SCHEMA: &str = "OPENWEPP_SNOW_STAGE3_V11_ATTACHMENT_RESTART_V3";
const VERSION: u16 = 3;

/// Content-addressed archive access. Storage locators and paths are excluded
/// from restart identity; only the exact bytes returned for a requested digest
/// participate in admission.
pub trait Stage3CommittedDayArchiveReaderV3 {
    fn read_canonical_uncompressed(&self, content_sha256: Digest32) -> Option<Vec<u8>>;
}

/// External archive evidence required for V3 admission. Entries are bounded
/// manifest metadata; detailed day payloads remain outside the restart wire.
pub struct ExpectedStage3CommittedDayArchiveV3<'a> {
    pub manifest: &'a Stage3CommittedDayArchiveManifestV1,
    pub reader: &'a dyn Stage3CommittedDayArchiveReaderV3,
}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct DirectSnowStage3V11AttachmentRestartV3 {
    pub schema: String,
    pub version: u16,
    pub static_context_sha256: Sha256Hex,
    pub archive_record_count: u64,
    pub archive_content_root_sha256: Digest32,
    pub archived_receipt_prefix: Stage3ArchivedReceiptPrefixV1,
    active_attachment_v2_canonical_base64: String,
    active_attachment_v2_sha256: Sha256Hex,
    support_liquid_custody_canonical_base64: String,
    support_liquid_custody_sha256: Sha256Hex,
    publication_rotation_canonical_base64: String,
    publication_rotation_sha256: Sha256Hex,
    pub payload_sha256: Sha256Hex,
}

#[derive(Serialize)]
struct DigestInput<'a> {
    schema: &'a str,
    version: u16,
    static_context_sha256: &'a Sha256Hex,
    archive_record_count: u64,
    archive_content_root_sha256: Digest32,
    archived_receipt_prefix: &'a Stage3ArchivedReceiptPrefixV1,
    active_attachment_v2_canonical_base64: &'a str,
    active_attachment_v2_sha256: &'a Sha256Hex,
    support_liquid_custody_canonical_base64: &'a str,
    support_liquid_custody_sha256: &'a Sha256Hex,
    publication_rotation_canonical_base64: &'a str,
    publication_rotation_sha256: &'a Sha256Hex,
}

impl DirectSnowStage3V11AttachmentRestartV3 {
    pub fn project(
        value: &DirectSnowStage3V11ShadowAttachment,
        phase_plan_sha256: &Sha256Hex,
        day_input_digests: &[Sha256Hex],
        archive: &ExpectedStage3CommittedDayArchiveV3<'_>,
    ) -> Result<Self, SnowStage3V11RestartError> {
        let archived_receipt_prefix = value
            .restart_authority_archived_receipt_prefix_v3()
            .map_err(nested)?;
        validate_archive_evidence(&archived_receipt_prefix, archive)?;
        validate_resident_tail_bound(value, &archived_receipt_prefix)?;
        let active = DirectSnowStage3V11AttachmentRestartV2::project_active_base_v3(
            value,
            phase_plan_sha256,
            day_input_digests,
        )?
        .to_canonical_bytes()?;
        let support_liquid_custody =
            restart_authority_encode_support_liquid_custody_state_v3(value).map_err(nested)?;
        let publication_rotation =
            restart_authority_encode_publication_rotation_state_v3(value).map_err(nested)?;
        let static_context_sha256 = static_context_sha256_from_active_v2(&active)?;
        let archive_record_count = u64::try_from(archived_receipt_prefix.archived_day_count)
            .map_err(|_| SnowStage3V11RestartError::Projection("V3 archive count width"))?;
        let mut projected = Self {
            schema: SCHEMA.into(),
            version: VERSION,
            static_context_sha256,
            archive_record_count,
            archive_content_root_sha256: archived_receipt_prefix.archive_content_root_sha256,
            archived_receipt_prefix,
            active_attachment_v2_sha256: sha256_hex(&active)?,
            active_attachment_v2_canonical_base64: STANDARD.encode(active),
            support_liquid_custody_sha256: sha256_hex(&support_liquid_custody)?,
            support_liquid_custody_canonical_base64: STANDARD.encode(support_liquid_custody),
            publication_rotation_sha256: sha256_hex(&publication_rotation)?,
            publication_rotation_canonical_base64: STANDARD.encode(publication_rotation),
            payload_sha256: zero_sha256()?,
        };
        projected.seal()?;
        Ok(projected)
    }

    pub fn restore(
        &self,
        context: &ExpectedSnowStage3V11RestartContext<'_>,
        archive: &ExpectedStage3CommittedDayArchiveV3<'_>,
    ) -> Result<DirectSnowStage3V11ShadowAttachment, SnowStage3V11RestartError> {
        self.validate_envelope(archive)?;
        let active = decode_blob(
            &self.active_attachment_v2_canonical_base64,
            &self.active_attachment_v2_sha256,
        )?;
        let active_v2 =
            DirectSnowStage3V11AttachmentRestartV2::from_canonical_bytes_active_base_v3(
                &active, context,
            )?;
        if active_v2.static_context_sha256 != self.static_context_sha256 {
            return Err(SnowStage3V11RestartError::Identity(
                "V3/V2 static context join",
            ));
        }
        let mut restored = active_v2.restore_active_base_v3(context)?;
        restored
            .restart_authority_install_archived_receipt_prefix_v3(
                self.archived_receipt_prefix.clone(),
            )
            .map_err(nested)?;
        let support_liquid_custody = decode_blob(
            &self.support_liquid_custody_canonical_base64,
            &self.support_liquid_custody_sha256,
        )?;
        restart_authority_restore_support_liquid_custody_state_v3(
            &mut restored,
            &support_liquid_custody,
        )
        .map_err(nested)?;
        let rotation = decode_blob(
            &self.publication_rotation_canonical_base64,
            &self.publication_rotation_sha256,
        )?;
        restart_authority_restore_publication_rotation_state_v3(&mut restored, &rotation)
            .map_err(nested)?;
        validate_publication_rotation_prefix(&restored, &self.archived_receipt_prefix)?;
        validate_resident_tail_bound(&restored, &self.archived_receipt_prefix)?;
        if restored
            .restart_authority_archived_receipt_prefix_v3()
            .map_err(nested)?
            != self.archived_receipt_prefix
            || restart_authority_encode_publication_rotation_state_v3(&restored).map_err(nested)?
                != rotation
            || restart_authority_encode_support_liquid_custody_state_v3(&restored)
                .map_err(nested)?
                != support_liquid_custody
        {
            return Err(SnowStage3V11RestartError::Identity(
                "V3 restored prefix or publication rotation join",
            ));
        }
        Ok(restored)
    }

    pub fn to_canonical_bytes(&self) -> Result<Vec<u8>, SnowStage3V11RestartError> {
        to_canonical_bytes(self).map_err(nested)
    }

    pub fn from_canonical_bytes(
        bytes: &[u8],
        context: &ExpectedSnowStage3V11RestartContext<'_>,
        archive: &ExpectedStage3CommittedDayArchiveV3<'_>,
    ) -> Result<Self, SnowStage3V11RestartError> {
        let value: Self = from_canonical_bytes(bytes).map_err(nested)?;
        if value.to_canonical_bytes()? != bytes {
            return Err(SnowStage3V11RestartError::Identity("V3 canonical bytes"));
        }
        value.restore(context, archive)?;
        Ok(value)
    }

    fn validate_envelope(
        &self,
        archive: &ExpectedStage3CommittedDayArchiveV3<'_>,
    ) -> Result<(), SnowStage3V11RestartError> {
        self.archived_receipt_prefix.validate().map_err(nested)?;
        let exact_count = usize::try_from(self.archive_record_count)
            .map_err(|_| SnowStage3V11RestartError::Identity("V3 archive count width"))?;
        if self.schema != SCHEMA
            || self.version != VERSION
            || exact_count != self.archived_receipt_prefix.archived_day_count
            || self.archive_content_root_sha256
                != self.archived_receipt_prefix.archive_content_root_sha256
            || self.payload_sha256 != self.compute_digest()?
        {
            return Err(SnowStage3V11RestartError::Identity(
                "V3 schema, version, archive prefix, or payload digest",
            ));
        }
        validate_archive_evidence(&self.archived_receipt_prefix, archive)
    }

    fn compute_digest(&self) -> Result<Sha256Hex, SnowStage3V11RestartError> {
        Sha256Hex::try_new(
            canonical_sha256(&DigestInput {
                schema: &self.schema,
                version: self.version,
                static_context_sha256: &self.static_context_sha256,
                archive_record_count: self.archive_record_count,
                archive_content_root_sha256: self.archive_content_root_sha256,
                archived_receipt_prefix: &self.archived_receipt_prefix,
                active_attachment_v2_canonical_base64: &self.active_attachment_v2_canonical_base64,
                active_attachment_v2_sha256: &self.active_attachment_v2_sha256,
                support_liquid_custody_canonical_base64: &self
                    .support_liquid_custody_canonical_base64,
                support_liquid_custody_sha256: &self.support_liquid_custody_sha256,
                publication_rotation_canonical_base64: &self.publication_rotation_canonical_base64,
                publication_rotation_sha256: &self.publication_rotation_sha256,
            })
            .map_err(nested)?,
        )
        .map_err(|_| SnowStage3V11RestartError::Projection("V3 payload digest"))
    }

    fn seal(&mut self) -> Result<(), SnowStage3V11RestartError> {
        self.payload_sha256 = self.compute_digest()?;
        Ok(())
    }

    #[cfg(all(test, feature = "fixtures"))]
    pub(crate) fn restart_authority_with_support_liquid_custody_poison_v3(
        &self,
        poison: RestartAuthoritySupportLiquidCustodyPoisonV3,
    ) -> Result<Self, SnowStage3V11RestartError> {
        let canonical = decode_blob(
            &self.support_liquid_custody_canonical_base64,
            &self.support_liquid_custody_sha256,
        )?;
        let poisoned = restart_authority_poison_support_liquid_custody_state_v3(&canonical, poison)
            .map_err(nested)?;
        let mut value = self.clone();
        value.support_liquid_custody_sha256 = sha256_hex(&poisoned)?;
        value.support_liquid_custody_canonical_base64 = STANDARD.encode(poisoned);
        value.seal()?;
        Ok(value)
    }

    #[cfg(all(test, feature = "fixtures"))]
    pub(crate) fn restart_authority_with_terminal_liquid_custody_poison_v3(
        &self,
        poison: RestartAuthorityTerminalLiquidCustodyPoisonV3,
    ) -> Result<Self, SnowStage3V11RestartError> {
        let canonical = decode_blob(
            &self.support_liquid_custody_canonical_base64,
            &self.support_liquid_custody_sha256,
        )?;
        let poisoned =
            restart_authority_poison_terminal_liquid_custody_state_v3(&canonical, poison)
                .map_err(nested)?;
        let mut value = self.clone();
        value.support_liquid_custody_sha256 = sha256_hex(&poisoned)?;
        value.support_liquid_custody_canonical_base64 = STANDARD.encode(poisoned);
        value.seal()?;
        Ok(value)
    }

    #[cfg(all(test, feature = "fixtures"))]
    pub(crate) fn restart_authority_with_publication_rotation_substitution_v3(
        &self,
        substituted: &[u8],
    ) -> Result<Self, SnowStage3V11RestartError> {
        let mut value = self.clone();
        value.publication_rotation_sha256 = sha256_hex(substituted)?;
        value.publication_rotation_canonical_base64 = STANDARD.encode(substituted);
        value.seal()?;
        Ok(value)
    }
}

fn validate_publication_rotation_prefix(
    value: &DirectSnowStage3V11ShadowAttachment,
    prefix: &Stage3ArchivedReceiptPrefixV1,
) -> Result<(), SnowStage3V11RestartError> {
    let expected_supports = usize::try_from(
        prefix
            .qualification_accumulator
            .accepted_publication_support_count,
    )
    .map_err(|_| SnowStage3V11RestartError::Identity("V3 publication prefix support width"))?;
    let expected_events = usize::try_from(prefix.qualification_accumulator.publication_event_count)
        .map_err(|_| SnowStage3V11RestartError::Identity("V3 publication prefix event width"))?;
    let validate = |state: &openwepp_hillslope_orchestrator::snow_stage3_v11_attachment::DirectSnowStage3V11CommittedState| {
        let retained = state
            .real_consumer
            .accepted_publication_retention_state_v1();
        if retained.sealed_support_count() != expected_supports
            || retained.sealed_event_count() != expected_events
            || (prefix.archived_day_count > 0
                && retained.sealed_prefix_authority_sha256() == Digest32::zero())
        {
            return Err(SnowStage3V11RestartError::Identity(
                "V3 archive/publication sealed-prefix count or root join",
            ));
        }
        Ok(())
    };
    validate(&value.committed)?;
    if let Some(pending) = value.restart_authority_pending_candidate() {
        validate(&pending.ending_state)?;
    }
    if let Some(execution) = value.restart_authority_in_progress_execution_v2() {
        validate(execution.day_candidate())?;
        validate(
            execution
                .support_current()
                .ok_or(SnowStage3V11RestartError::Identity(
                    "V3 in-progress publication prefix owner",
                ))?,
        )?;
    }
    Ok(())
}

fn validate_resident_tail_bound(
    value: &DirectSnowStage3V11ShadowAttachment,
    prefix: &Stage3ArchivedReceiptPrefixV1,
) -> Result<(), SnowStage3V11RestartError> {
    let validate = |receipts: &[openwepp_hillslope_orchestrator::snow_stage3_v11_attachment::DirectSnowStage3V11ParentReceipt]| {
        if receipts.len() > 1
            || receipts
                .first()
                .is_some_and(|receipt| receipt.day_index < prefix.archived_day_count)
        {
            return Err(SnowStage3V11RestartError::Identity(
                "V3 resident receipt tail bound or archive overlap",
            ));
        }
        Ok(())
    };
    validate(&value.committed.receipt_chain)?;
    if let Some(pending) = value.restart_authority_pending_candidate() {
        validate(&pending.ending_state.receipt_chain)?;
    }
    if let Some(execution) = value.restart_authority_in_progress_execution_v2() {
        validate(&execution.day_candidate().receipt_chain)?;
        validate(
            &execution
                .support_current()
                .ok_or(SnowStage3V11RestartError::Identity(
                    "V3 in-progress support owner",
                ))?
                .receipt_chain,
        )?;
    }
    Ok(())
}

fn validate_archive_evidence(
    prefix: &Stage3ArchivedReceiptPrefixV1,
    archive: &ExpectedStage3CommittedDayArchiveV3<'_>,
) -> Result<(), SnowStage3V11RestartError> {
    prefix.validate().map_err(nested)?;
    archive.manifest.validate().map_err(nested)?;
    if archive.manifest.run_identity != prefix.run_identity
        || archive.manifest.topology_identity != prefix.topology_identity
        || archive.manifest.committed_day_count != prefix.archived_day_count
        || archive.manifest.entries.len() != prefix.archived_day_count
        || archive.manifest.ordered_day_chain_sha256 != prefix.ordered_day_chain_sha256
        || archive.manifest.archive_content_root_sha256 != prefix.archive_content_root_sha256
    {
        return Err(SnowStage3V11RestartError::Identity(
            "V3 missing, truncated, or root-mismatched archive manifest",
        ));
    }
    let mut prior_entry: Option<&Stage3CommittedDayArchiveEntryV1> = None;
    let mut record_ids = BTreeSet::new();
    let mut content_ids = BTreeSet::new();
    for (ordinal, entry) in archive.manifest.entries.iter().enumerate() {
        entry.validate().map_err(nested)?;
        if entry.day_index != ordinal
            || !record_ids.insert(entry.record_sha256)
            || !content_ids.insert(entry.content_sha256)
            || prior_entry.is_some_and(|prior| {
                entry.previous_ordered_day_chain_sha256 != prior.resulting_ordered_day_chain_sha256
                    || entry.previous_archive_content_root_sha256
                        != prior.resulting_archive_content_root_sha256
                    || entry.beginning_owner_set_sha256 != prior.ending_owner_set_sha256
                    || entry.ending_next_parent_sequence <= prior.ending_next_parent_sequence
            })
        {
            return Err(SnowStage3V11RestartError::Identity(
                "V3 reordered, duplicated, or discontinuous archive manifest",
            ));
        }
        let bytes = archive
            .reader
            .read_canonical_uncompressed(entry.content_sha256)
            .ok_or(SnowStage3V11RestartError::Identity(
                "V3 missing archive content",
            ))?;
        if u64::try_from(bytes.len()).ok() != Some(entry.canonical_uncompressed_len)
            || digest_bytes(&bytes) != entry.content_sha256
        {
            return Err(SnowStage3V11RestartError::Identity(
                "V3 truncated or substituted archive content",
            ));
        }
        prior_entry = Some(entry);
    }
    match prior_entry {
        None => {
            if prefix.archived_day_count != 0
                || prefix.last_day_index.is_some()
                || prefix.ending_owner_set_sha256.is_some()
            {
                return Err(SnowStage3V11RestartError::Identity(
                    "V3 empty archive prefix join",
                ));
            }
        }
        Some(last) => {
            if prefix.last_day_index != Some(last.day_index)
                || prefix.ordered_day_chain_sha256 != last.resulting_ordered_day_chain_sha256
                || prefix.archive_content_root_sha256 != last.resulting_archive_content_root_sha256
                || prefix.last_day_record_sha256 != last.record_sha256
                || prefix.last_parent_receipt_sha256 != last.parent_receipt_sha256
                || prefix.ending_owner_set_sha256 != Some(last.ending_owner_set_sha256)
                || prefix.accepted_until_ns != last.ending_accepted_until_ns
                || prefix.next_parent_sequence != last.ending_next_parent_sequence
            {
                return Err(SnowStage3V11RestartError::Identity(
                    "V3 archive root, count, or endpoint mismatch",
                ));
            }
        }
    }
    Ok(())
}

fn static_context_sha256_from_active_v2(
    bytes: &[u8],
) -> Result<Sha256Hex, SnowStage3V11RestartError> {
    let active: DirectSnowStage3V11AttachmentRestartV2 =
        from_canonical_bytes(bytes).map_err(nested)?;
    Ok(active.static_context_sha256)
}

fn sha256_hex(bytes: &[u8]) -> Result<Sha256Hex, SnowStage3V11RestartError> {
    Sha256Hex::try_new(format!("{:x}", sha2::Sha256::digest(bytes)))
        .map_err(|_| SnowStage3V11RestartError::Projection("V3 blob digest"))
}

fn decode_blob(encoded: &str, expected: &Sha256Hex) -> Result<Vec<u8>, SnowStage3V11RestartError> {
    let bytes = STANDARD
        .decode(encoded)
        .map_err(|_| SnowStage3V11RestartError::Identity("V3 base64"))?;
    if STANDARD.encode(&bytes) != encoded || &sha256_hex(&bytes)? != expected {
        return Err(SnowStage3V11RestartError::Identity("V3 blob digest"));
    }
    Ok(bytes)
}

fn zero_sha256() -> Result<Sha256Hex, SnowStage3V11RestartError> {
    Sha256Hex::try_new("0".repeat(64))
        .map_err(|_| SnowStage3V11RestartError::Projection("V3 digest seed"))
}

fn nested(error: impl std::fmt::Display) -> SnowStage3V11RestartError {
    SnowStage3V11RestartError::Nested(error.to_string())
}
