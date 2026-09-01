//! Canonical additive restart for the constitutive Stage-3/V11 attachment.

use base64::{Engine as _, engine::general_purpose::STANDARD};
use openwepp_coupled_time::{
    CoupledTimeRestartV2, DiagnosticReductionV1, Digest32, complete_owner_set_digest, digest_bytes,
};
use openwepp_hillslope_orchestrator::{
    DirectSnowStage3PersistentState,
    runtime_inputs::SnowFreeHalfHourProviderCursor,
    snow_stage3_v11_attachment::{
        DirectSnowStage3V11CommittedState, DirectSnowStage3V11InProgressExecutionV2,
        DirectSnowStage3V11InterruptionPostureV2, DirectSnowStage3V11ParentCandidate,
        DirectSnowStage3V11ShadowAttachment, DirectSnowStage3V11StaticContext,
        restart_authority_decode_in_progress_metadata_v2,
        restart_authority_decode_receipt_state_v2,
        restart_authority_encode_in_progress_metadata_base_v3,
        restart_authority_encode_in_progress_metadata_v2,
        restart_authority_encode_receipt_state_base_v3, restart_authority_encode_receipt_state_v2,
    },
    v9_real_consumer_shadow::DirectV10RealConsumerShadow,
};
use openwepp_vegetation::V11ParentTransactionCheckpoint;
use serde::{Deserialize, Serialize};
use sha2::{Digest as _, Sha256};
use thiserror::Error;

use crate::projection::project_complete_owner_state_v1_for_exact_parent;
use crate::{
    AcceptedIntervalCount, CompleteCommittedOwnerStateV1, DirectV10CheckpointPhaseV1,
    DirectV10RealConsumerCheckpointV1, DirectV10RestartHost, ExpectedRestartStaticContext, HexU128,
    RestartAdmissionFailureV1, Sha256Hex, VegetationV10StateRestartV1, WireDayIndex,
    admit_checkpoint_v1, checkpoint_identities_v1, project_complete_owner_state_v1,
    to_canonical_bytes,
};

const SCHEMA: &str = "OPENWEPP_SNOW_STAGE3_V11_ATTACHMENT_RESTART_V2";
const VERSION: u16 = 2;
const CLOCK_MODEL_DOMAIN: &[u8] = b"OPENWEPP_STAGE3_V11_COUPLED_CLOCK_RESTART_MODEL_V1";
const CLOCK_AUTHORITY_DOMAIN: &[u8] = b"SC-SNOWENERGY-001@22+SC-COUPLEDTIME-001";
const CLOCK_REDUCTION_ID: &str = "stage3-v11-restart-no-publication";

#[derive(Debug, Error)]
pub enum SnowStage3V11RestartError {
    #[error("Stage-3/V11 restart projection failed: {0}")]
    Projection(&'static str),
    #[error("Stage-3/V11 restart identity failed: {0}")]
    Identity(&'static str),
    #[error("Stage-3/V11 nested restart failed: {0}")]
    Nested(String),
    #[error("Stage-3/V11 nested restart failed during {phase}: {detail}")]
    NestedPhase { phase: &'static str, detail: String },
    #[error("Stage-3/V11 real-consumer checkpoint admission failed: {0}")]
    RealConsumerAdmission(#[source] RestartAdmissionFailureV1),
    #[error(
        "Stage-3/V11 parent-finalization {owner} lineage expected {expected}, found {actual:?}"
    )]
    ParentFinalizationLineage {
        owner: &'static str,
        expected: u128,
        actual: Option<u128>,
    },
    #[error(
        "Stage-3/V11 admitted scheduler next-day {next_day}, count {accepted_count}, vegetation {vegetation}, LSE {lse:?}, soil {soil:?}, BGC {biogeochemistry}"
    )]
    AdmittedSchedulerLineage {
        next_day: u64,
        accepted_count: u64,
        vegetation: u128,
        lse: Option<u128>,
        soil: Option<u128>,
        biogeochemistry: u128,
    },
    #[error(
        "Stage-3/V11 vegetation configuration expected {expected_sha256}, found {actual_sha256}"
    )]
    VegetationConfigurationMismatch {
        expected_sha256: String,
        actual_sha256: String,
    },
    #[error(
        "Stage-3/V11 parent-clock owner {owner} expected {expected:?}/tx {expected_transaction:?}, found {actual:?}/tx {actual_transaction:?}"
    )]
    V11ClockOwnerJoin {
        owner: String,
        expected: Option<Digest32>,
        actual: Option<Digest32>,
        expected_transaction: Option<u128>,
        actual_transaction: Option<u128>,
    },
    #[error(
        "Stage-3/V11 completed-parent posture has {accepted_segments} accepted segments and {zero_duration_transitions} zero-duration transitions"
    )]
    CompletedParentPosture {
        accepted_segments: usize,
        zero_duration_transitions: usize,
    },
}

include!("snow_stage3_v11_owner_join.rs");

pub struct ExpectedSnowStage3V11RestartContext<'a> {
    pub static_context: &'a DirectSnowStage3V11StaticContext,
    pub real_consumer_context: &'a ExpectedRestartStaticContext<'a>,
}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
struct DirectSnowStage3V11CommittedRestartV2 {
    stage3_by_lane: Vec<(u32, DirectSnowStage3PersistentState)>,
    real_consumer: Box<CompleteCommittedOwnerStateV1>,
    real_consumer_next_day_index: WireDayIndex,
    real_consumer_accepted_interval_count: AcceptedIntervalCount,
    real_consumer_provider_cursor_configuration_bound: bool,
    real_consumer_wb14_parent_canonical_base64: Option<String>,
    real_consumer_wb14_parent_sha256: Option<Sha256Hex>,
    accepted_publication_supports_canonical_base64: String,
    accepted_publication_supports_sha256: Sha256Hex,
    v11_parent_checkpoint: V11ParentTransactionCheckpoint,
    coupled_clock_canonical_base64: String,
    coupled_clock_sha256: Sha256Hex,
    next_parent_sequence: HexU128,
    has_last_v11_parent_candidate: bool,
    receipt_state_canonical_base64: String,
    receipt_state_sha256: Sha256Hex,
}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
struct DirectSnowStage3V11PendingRestartV2 {
    ending_state: Box<DirectSnowStage3V11CommittedRestartV2>,
}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
struct DirectSnowStage3V11InProgressRestartV2 {
    day_candidate: Box<DirectSnowStage3V11CommittedRestartV2>,
    support_current: Box<DirectSnowStage3V11CommittedRestartV2>,
    metadata_canonical_base64: String,
    metadata_sha256: Sha256Hex,
}

#[derive(Deserialize)]
struct InProgressPostureProbe {
    posture: DirectSnowStage3V11InterruptionPostureV2,
}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct DirectSnowStage3V11AttachmentRestartV2 {
    pub schema: String,
    pub version: u16,
    pub static_context_sha256: Sha256Hex,
    committed: DirectSnowStage3V11CommittedRestartV2,
    pending_candidate: Option<Box<DirectSnowStage3V11PendingRestartV2>>,
    #[serde(default)]
    in_progress_execution: Option<Box<DirectSnowStage3V11InProgressRestartV2>>,
    pub payload_sha256: Sha256Hex,
}

#[derive(Serialize)]
struct DigestInput<'a> {
    schema: &'a str,
    version: u16,
    static_context_sha256: &'a Sha256Hex,
    committed: &'a DirectSnowStage3V11CommittedRestartV2,
    pending_candidate: &'a Option<Box<DirectSnowStage3V11PendingRestartV2>>,
    in_progress_execution: &'a Option<Box<DirectSnowStage3V11InProgressRestartV2>>,
}

#[derive(Serialize)]
struct StaticContextIdentity<'a> {
    run_identity: Digest32,
    topology_identity: Digest32,
    parent_duration_ns: String,
    minimum_support_ns: String,
    calendar_receipt: Digest32,
    controller_policy: Digest32,
    parent_sequence: String,
    lane_ids: &'a [u32],
    vegetation_configuration_sha256: &'a str,
    surface_liquid_configuration_sha256: &'a str,
    wb14_parameters: &'a [openwepp_hillslope_orchestrator::DirectOfeWb14Parameters],
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) enum SnowStage3V11ExactResidentPositionV4 {
    Committed,
    PendingCandidate,
    InProgressDayCandidate,
    InProgressSupportCurrent,
}

pub(crate) trait SnowStage3V11ExactResidentRestorerV4 {
    fn restore_exact_resident(
        &self,
        position: SnowStage3V11ExactResidentPositionV4,
        consumer: &mut DirectV10RealConsumerShadow,
    ) -> Result<(), SnowStage3V11RestartError>;
}

impl DirectSnowStage3V11AttachmentRestartV2 {
    pub fn project(
        value: &DirectSnowStage3V11ShadowAttachment,
        phase_plan_sha256: &Sha256Hex,
        day_input_digests: &[Sha256Hex],
    ) -> Result<Self, SnowStage3V11RestartError> {
        Self::project_active_base(value, phase_plan_sha256, day_input_digests, false)
    }

    pub(crate) fn project_active_base_v3(
        value: &DirectSnowStage3V11ShadowAttachment,
        phase_plan_sha256: &Sha256Hex,
        day_input_digests: &[Sha256Hex],
    ) -> Result<Self, SnowStage3V11RestartError> {
        Self::project_active_base(value, phase_plan_sha256, day_input_digests, true)
    }

    fn project_active_base(
        value: &DirectSnowStage3V11ShadowAttachment,
        phase_plan_sha256: &Sha256Hex,
        day_input_digests: &[Sha256Hex],
        for_v3: bool,
    ) -> Result<Self, SnowStage3V11RestartError> {
        let static_context_sha256 = static_context_sha256(&value.static_context)?;
        let committed = project_committed_mode(
            &value.committed,
            phase_plan_sha256,
            day_input_digests,
            for_v3,
        )?;
        let pending_candidate = value
            .restart_authority_pending_candidate()
            .map(|candidate| {
                Ok(Box::new(DirectSnowStage3V11PendingRestartV2 {
                    ending_state: Box::new(project_committed_mode(
                        &candidate.ending_state,
                        phase_plan_sha256,
                        day_input_digests,
                        for_v3,
                    )?),
                }))
            })
            .transpose()?;
        let in_progress_execution =
            value
                .restart_authority_in_progress_execution_v2()
                .map(|in_progress| {
                    let support_current = in_progress.support_current().ok_or(
                        SnowStage3V11RestartError::Projection("in-progress support owner"),
                    )?;
                    if matches!(
                        in_progress.posture(),
                        openwepp_hillslope_orchestrator::snow_stage3_v11_attachment::DirectSnowStage3V11InterruptionPostureV2::AdaptiveMicrostepBoundary
                    ) && support_current
                        .real_consumer
                        .restart_authority_wb14_parent_canonical_bytes()
                        .map_err(nested)?
                        .is_none()
                    {
                        return Err(SnowStage3V11RestartError::Identity(
                            "adaptive boundary WB14 parent posture",
                        ));
                    }
                    let metadata = if for_v3 {
                        restart_authority_encode_in_progress_metadata_base_v3(in_progress)
                    } else {
                        restart_authority_encode_in_progress_metadata_v2(in_progress)
                    }
                    .map_err(nested)?;
                    Ok(Box::new(DirectSnowStage3V11InProgressRestartV2 {
                        day_candidate: Box::new(project_committed_mode(
                            in_progress.day_candidate(),
                            phase_plan_sha256,
                            day_input_digests,
                            for_v3,
                        )?),
                        support_current: Box::new(project_committed_mode(
                            support_current,
                            phase_plan_sha256,
                            day_input_digests,
                            for_v3,
                        )?),
                        metadata_sha256: sha256_hex(&metadata)?,
                        metadata_canonical_base64: STANDARD.encode(metadata),
                    }))
                })
                .transpose()?;
        if pending_candidate.is_some() && in_progress_execution.is_some() {
            return Err(SnowStage3V11RestartError::Identity(
                "pending/in-progress exclusivity",
            ));
        }
        if committed
            .real_consumer_wb14_parent_canonical_base64
            .is_some()
            || pending_candidate.as_deref().is_some_and(|pending| {
                pending
                    .ending_state
                    .real_consumer_wb14_parent_canonical_base64
                    .is_some()
            })
            || in_progress_execution.as_deref().is_some_and(|in_progress| {
                in_progress
                    .day_candidate
                    .real_consumer_wb14_parent_canonical_base64
                    .is_some()
            })
        {
            return Err(SnowStage3V11RestartError::Identity(
                "WB14 parent blob outside in-progress support",
            ));
        }
        if in_progress_execution.as_deref().is_some_and(|in_progress| {
            in_progress
                .support_current
                .real_consumer_provider_cursor_configuration_bound
                != in_progress
                    .day_candidate
                    .real_consumer_provider_cursor_configuration_bound
                || in_progress
                    .day_candidate
                    .real_consumer_provider_cursor_configuration_bound
                    != committed.real_consumer_provider_cursor_configuration_bound
        }) {
            return Err(SnowStage3V11RestartError::Identity(
                "in-progress provider cursor posture join",
            ));
        }
        let mut projected = Self {
            schema: SCHEMA.into(),
            version: VERSION,
            static_context_sha256,
            committed,
            pending_candidate,
            in_progress_execution,
            payload_sha256: Sha256Hex::try_new("0".repeat(64))
                .map_err(|_| SnowStage3V11RestartError::Projection("payload digest seed"))?,
        };
        projected.seal()?;
        Ok(projected)
    }

    pub fn restore(
        &self,
        context: &ExpectedSnowStage3V11RestartContext<'_>,
    ) -> Result<DirectSnowStage3V11ShadowAttachment, SnowStage3V11RestartError> {
        self.restore_mode(context, false)
    }

    pub(crate) fn restore_active_base_v3(
        &self,
        context: &ExpectedSnowStage3V11RestartContext<'_>,
    ) -> Result<DirectSnowStage3V11ShadowAttachment, SnowStage3V11RestartError> {
        self.restore_mode(context, true)
    }

    fn restore_mode(
        &self,
        context: &ExpectedSnowStage3V11RestartContext<'_>,
        for_v3: bool,
    ) -> Result<DirectSnowStage3V11ShadowAttachment, SnowStage3V11RestartError> {
        self.validate_restore_envelope(context)?;
        let committed = restore_committed(
            &self.committed,
            context,
            CommittedRestartPosture::BetweenDays,
            for_v3,
            SnowStage3V11ExactResidentPositionV4::Committed,
            None,
        )?;
        let pending_candidate = self
            .pending_candidate
            .as_deref()
            .map(|pending| {
                let ending_state = restore_committed(
                    &pending.ending_state,
                    context,
                    CommittedRestartPosture::BetweenDays,
                    for_v3,
                    SnowStage3V11ExactResidentPositionV4::PendingCandidate,
                    None,
                )?;
                let parent_receipt = ending_state.receipt_chain.last().cloned().ok_or(
                    SnowStage3V11RestartError::Identity("pending candidate parent receipt"),
                )?;
                Ok(DirectSnowStage3V11ParentCandidate {
                    ending_state,
                    parent_receipt,
                })
            })
            .transpose()?;
        let in_progress_execution = self.restore_in_progress(context, for_v3, None)?;
        DirectSnowStage3V11ShadowAttachment::restart_authority_restore_parts_with_in_progress_v2(
            context.static_context.clone(),
            committed,
            pending_candidate,
            in_progress_execution,
        )
        .map_err(nested)
    }

    fn validate_restore_envelope(
        &self,
        context: &ExpectedSnowStage3V11RestartContext<'_>,
    ) -> Result<(), SnowStage3V11RestartError> {
        if self.schema != SCHEMA
            || self.version != VERSION
            || self.static_context_sha256 != static_context_sha256(context.static_context)?
            || self.payload_sha256 != self.compute_digest()?
        {
            return Err(SnowStage3V11RestartError::Identity(
                "schema, version, static context, or payload digest",
            ));
        }
        if self
            .committed
            .real_consumer_wb14_parent_canonical_base64
            .is_some()
            || self.pending_candidate.as_deref().is_some_and(|pending| {
                pending
                    .ending_state
                    .real_consumer_wb14_parent_canonical_base64
                    .is_some()
            })
            || self
                .in_progress_execution
                .as_deref()
                .is_some_and(|in_progress| {
                    in_progress
                        .day_candidate
                        .real_consumer_wb14_parent_canonical_base64
                        .is_some()
                })
        {
            return Err(SnowStage3V11RestartError::Identity(
                "WB14 parent blob outside in-progress support",
            ));
        }
        if self
            .in_progress_execution
            .as_deref()
            .is_some_and(|in_progress| {
                in_progress
                    .support_current
                    .real_consumer_provider_cursor_configuration_bound
                    != in_progress
                        .day_candidate
                        .real_consumer_provider_cursor_configuration_bound
                    || in_progress
                        .day_candidate
                        .real_consumer_provider_cursor_configuration_bound
                        != self
                            .committed
                            .real_consumer_provider_cursor_configuration_bound
            })
        {
            return Err(SnowStage3V11RestartError::Identity(
                "in-progress provider cursor posture join",
            ));
        }
        Ok(())
    }

    fn restore_in_progress(
        &self,
        context: &ExpectedSnowStage3V11RestartContext<'_>,
        for_v3: bool,
        exact_restorer: Option<&dyn SnowStage3V11ExactResidentRestorerV4>,
    ) -> Result<Option<DirectSnowStage3V11InProgressExecutionV2>, SnowStage3V11RestartError> {
        let in_progress_execution = self
            .in_progress_execution
            .as_deref()
            .map(|in_progress| {
                let metadata = decode_blob(
                    &in_progress.metadata_canonical_base64,
                    &in_progress.metadata_sha256,
                )?;
                let interruption = serde_json::from_slice::<InProgressPostureProbe>(&metadata)
                    .map_err(|_| {
                        SnowStage3V11RestartError::Identity(
                            "in-progress interruption posture metadata",
                        )
                    })?
                    .posture;
                let day_candidate = restore_committed(
                    &in_progress.day_candidate,
                    context,
                    CommittedRestartPosture::InProgressDayCandidate,
                    for_v3,
                    SnowStage3V11ExactResidentPositionV4::InProgressDayCandidate,
                    exact_restorer,
                )?;
                let mut support_current = restore_committed(
                    &in_progress.support_current,
                    context,
                    CommittedRestartPosture::InProgressSupport(interruption),
                    for_v3,
                    SnowStage3V11ExactResidentPositionV4::InProgressSupportCurrent,
                    exact_restorer,
                )?;
                if in_progress.support_current.has_last_v11_parent_candidate
                    != day_candidate.last_v11_parent_candidate.is_some()
                {
                    return Err(SnowStage3V11RestartError::Identity(
                        "active-support inherited last-candidate posture",
                    ));
                }
                support_current
                    .last_v11_parent_candidate
                    .clone_from(&day_candidate.last_v11_parent_candidate);
                let restored = restart_authority_decode_in_progress_metadata_v2(
                    &metadata,
                    day_candidate,
                    Some(support_current),
                )
                .map_err(nested)?;
                if matches!(
                    restored.posture(),
                    DirectSnowStage3V11InterruptionPostureV2::AfterSnowReappearance
                ) {
                    restored
                        .restart_authority_validate_after_snow_reappearance_publication_v2()
                        .map_err(nested)?;
                }
                if matches!(
                    restored.posture(),
                    openwepp_hillslope_orchestrator::snow_stage3_v11_attachment::DirectSnowStage3V11InterruptionPostureV2::AdaptiveMicrostepBoundary
                ) && restored
                    .support_current()
                    .ok_or(SnowStage3V11RestartError::Identity(
                        "in-progress support owner",
                    ))?
                    .real_consumer
                    .restart_authority_wb14_parent_canonical_bytes()
                    .map_err(nested)?
                    .is_none()
                {
                    return Err(SnowStage3V11RestartError::Identity(
                        "adaptive boundary WB14 parent posture",
                    ));
                }
                Ok(restored)
            })
            .transpose()?;
        Ok(in_progress_execution)
    }

    pub(crate) fn restore_active_base_v4(
        &self,
        context: &ExpectedSnowStage3V11RestartContext<'_>,
        restorer: &dyn SnowStage3V11ExactResidentRestorerV4,
    ) -> Result<DirectSnowStage3V11ShadowAttachment, SnowStage3V11RestartError> {
        self.restore_mode_v4(context, restorer)
    }

    fn restore_mode_v4(
        &self,
        context: &ExpectedSnowStage3V11RestartContext<'_>,
        restorer: &dyn SnowStage3V11ExactResidentRestorerV4,
    ) -> Result<DirectSnowStage3V11ShadowAttachment, SnowStage3V11RestartError> {
        self.validate_restore_envelope(context)?;
        let committed = restore_committed(
            &self.committed,
            context,
            CommittedRestartPosture::BetweenDays,
            true,
            SnowStage3V11ExactResidentPositionV4::Committed,
            Some(restorer),
        )?;
        let pending_candidate = self
            .pending_candidate
            .as_deref()
            .map(|pending| {
                let ending_state = restore_committed(
                    &pending.ending_state,
                    context,
                    CommittedRestartPosture::BetweenDays,
                    true,
                    SnowStage3V11ExactResidentPositionV4::PendingCandidate,
                    Some(restorer),
                )?;
                let parent_receipt = ending_state.receipt_chain.last().cloned().ok_or(
                    SnowStage3V11RestartError::Identity("pending candidate parent receipt"),
                )?;
                Ok(DirectSnowStage3V11ParentCandidate {
                    ending_state,
                    parent_receipt,
                })
            })
            .transpose()?;
        let in_progress_execution = self.restore_in_progress(context, true, Some(restorer))?;
        DirectSnowStage3V11ShadowAttachment::restart_authority_restore_parts_with_in_progress_v2(
            context.static_context.clone(),
            committed,
            pending_candidate,
            in_progress_execution,
        )
        .map_err(nested)
    }

    pub fn to_canonical_bytes(&self) -> Result<Vec<u8>, SnowStage3V11RestartError> {
        to_canonical_bytes(self).map_err(nested)
    }

    pub fn from_canonical_bytes(
        bytes: &[u8],
        context: &ExpectedSnowStage3V11RestartContext<'_>,
    ) -> Result<Self, SnowStage3V11RestartError> {
        let value: Self = crate::from_canonical_bytes(bytes).map_err(nested)?;
        if value.to_canonical_bytes()? != bytes {
            return Err(SnowStage3V11RestartError::Identity("canonical bytes"));
        }
        value.restore(context)?;
        Ok(value)
    }

    pub(crate) fn from_canonical_bytes_active_base_v3(
        bytes: &[u8],
        context: &ExpectedSnowStage3V11RestartContext<'_>,
    ) -> Result<Self, SnowStage3V11RestartError> {
        let value: Self = crate::from_canonical_bytes(bytes).map_err(nested)?;
        if value.to_canonical_bytes()? != bytes {
            return Err(SnowStage3V11RestartError::Identity("canonical bytes"));
        }
        value.restore_active_base_v3(context)?;
        Ok(value)
    }

    fn compute_digest(&self) -> Result<Sha256Hex, SnowStage3V11RestartError> {
        Sha256Hex::try_new(
            crate::canonical_sha256(&DigestInput {
                schema: &self.schema,
                version: self.version,
                static_context_sha256: &self.static_context_sha256,
                committed: &self.committed,
                pending_candidate: &self.pending_candidate,
                in_progress_execution: &self.in_progress_execution,
            })
            .map_err(nested)?,
        )
        .map_err(|_| SnowStage3V11RestartError::Projection("payload digest"))
    }

    fn seal(&mut self) -> Result<(), SnowStage3V11RestartError> {
        self.payload_sha256 = self.compute_digest()?;
        Ok(())
    }
}

fn project_committed_mode(
    value: &DirectSnowStage3V11CommittedState,
    phase_plan_sha256: &Sha256Hex,
    day_input_digests: &[Sha256Hex],
    for_v3: bool,
) -> Result<DirectSnowStage3V11CommittedRestartV2, SnowStage3V11RestartError> {
    let consumer = &value.real_consumer;
    let consumer_next_day = consumer.v11_next_day_index();
    let real_consumer_provider_cursor_configuration_bound =
        provider_cursor_configuration_bound(consumer.provider_cursor())?;
    let real_consumer_wb14_parent = consumer
        .restart_authority_wb14_parent_canonical_bytes()
        .map_err(nested)?;
    let real_consumer = if for_v3 {
        project_complete_owner_state_v1_for_exact_parent(
            consumer,
            phase_plan_sha256,
            day_input_digests,
            consumer_next_day,
        )
    } else {
        project_complete_owner_state_v1(
            consumer,
            phase_plan_sha256,
            day_input_digests,
            consumer_next_day,
        )
    }
    .map_err(SnowStage3V11RestartError::Projection)?;
    let accepted_publication_supports = if for_v3 {
        consumer.restart_authority_accepted_publication_active_tail_canonical_bytes_v3()
    } else {
        consumer.restart_authority_accepted_publication_supports_canonical_bytes()
    }
    .map_err(nested)?;
    let receipt_state = if for_v3 {
        restart_authority_encode_receipt_state_base_v3(
            &value.terminal_parcels,
            &value.receipt_chain,
        )
    } else {
        restart_authority_encode_receipt_state_v2(&value.terminal_parcels, &value.receipt_chain)
    }
    .map_err(nested)?;
    let coupled_clock = CoupledTimeRestartV2::new(
        digest_bytes(CLOCK_MODEL_DOMAIN),
        digest_bytes(CLOCK_AUTHORITY_DOMAIN),
        value.coupled_clock.clone(),
        DiagnosticReductionV1::new(CLOCK_REDUCTION_ID.into(), "dimensionless".into())
            .map_err(nested)?,
        None,
        Vec::new(),
    )
    .map_err(nested)?
    .to_canonical_json()
    .map_err(nested)?;
    Ok(DirectSnowStage3V11CommittedRestartV2 {
        stage3_by_lane: value
            .stage3_by_lane
            .iter()
            .map(|(lane, state)| (*lane, state.clone()))
            .collect(),
        real_consumer: Box::new(real_consumer),
        real_consumer_next_day_index: WireDayIndex(
            u64::try_from(consumer_next_day)
                .map_err(|_| SnowStage3V11RestartError::Projection("consumer day width"))?,
        ),
        real_consumer_accepted_interval_count: AcceptedIntervalCount::try_new(
            consumer.restart_authority_accepted_interval_count(),
        )
        .map_err(nested)?,
        real_consumer_provider_cursor_configuration_bound,
        real_consumer_wb14_parent_sha256: real_consumer_wb14_parent
            .as_deref()
            .map(sha256_hex)
            .transpose()?,
        real_consumer_wb14_parent_canonical_base64: real_consumer_wb14_parent
            .as_deref()
            .map(|bytes| STANDARD.encode(bytes)),
        accepted_publication_supports_sha256: sha256_hex(&accepted_publication_supports)?,
        accepted_publication_supports_canonical_base64: STANDARD
            .encode(accepted_publication_supports),
        v11_parent_checkpoint: value.v11_parent_state.checkpoint(),
        coupled_clock_sha256: sha256_hex(&coupled_clock)?,
        coupled_clock_canonical_base64: STANDARD.encode(&coupled_clock),
        next_parent_sequence: HexU128::from_u128(value.next_parent_sequence),
        has_last_v11_parent_candidate: value.last_v11_parent_candidate.is_some(),
        receipt_state_sha256: sha256_hex(&receipt_state)?,
        receipt_state_canonical_base64: STANDARD.encode(receipt_state),
    })
}

#[derive(Clone, Copy)]
enum CommittedRestartPosture {
    BetweenDays,
    InProgressDayCandidate,
    InProgressSupport(DirectSnowStage3V11InterruptionPostureV2),
}

impl CommittedRestartPosture {
    const fn is_in_progress(self) -> bool {
        matches!(
            self,
            Self::InProgressDayCandidate | Self::InProgressSupport(_)
        )
    }

    const fn owns_serialized_last_candidate(self) -> bool {
        !matches!(self, Self::InProgressSupport(_))
    }

    const fn permits_receiver_surface_successor(self) -> bool {
        matches!(
            self,
            Self::InProgressSupport(
                DirectSnowStage3V11InterruptionPostureV2::AfterTerminalReceiver
            )
        )
    }

    const fn defers_pre_support_reappearance_publication(self) -> bool {
        matches!(
            self,
            Self::InProgressSupport(
                DirectSnowStage3V11InterruptionPostureV2::AfterSnowReappearance
            )
        )
    }
}

fn restore_committed(
    value: &DirectSnowStage3V11CommittedRestartV2,
    context: &ExpectedSnowStage3V11RestartContext<'_>,
    posture: CommittedRestartPosture,
    for_v3: bool,
    exact_position: SnowStage3V11ExactResidentPositionV4,
    exact_restorer: Option<&dyn SnowStage3V11ExactResidentRestorerV4>,
) -> Result<DirectSnowStage3V11CommittedState, SnowStage3V11RestartError> {
    if value.stage3_by_lane.is_empty()
        || value
            .stage3_by_lane
            .windows(2)
            .any(|pair| pair[0].0 >= pair[1].0)
    {
        return Err(SnowStage3V11RestartError::Identity("ordered Stage-3 lanes"));
    }
    let clock_bytes = decode_blob(
        &value.coupled_clock_canonical_base64,
        &value.coupled_clock_sha256,
    )?;
    let coupled_clock = CoupledTimeRestartV2::from_canonical_json(
        &clock_bytes,
        digest_bytes(CLOCK_MODEL_DOMAIN),
        digest_bytes(CLOCK_AUTHORITY_DOMAIN),
        context.static_context.controller_policy,
    )
    .map_err(nested)?
    .clock()
    .clone();
    let mut real_consumer = restore_real_consumer(
        value,
        context.real_consumer_context,
        posture,
        &coupled_clock,
        for_v3,
    )?;
    if let Some(restorer) = exact_restorer {
        restorer.restore_exact_resident(exact_position, &mut real_consumer)?;
    }
    // The checkpoint is the authority for the exact dynamic parent state.
    // `initial_state_sha256` is an imported receipt and is intentionally not
    // part of the canonical vegetation-configuration digest; rebuilding the
    // parent from that receipt would discard the independently sealed V11
    // checkpoint state.
    let v11_parent_state = openwepp_vegetation::V11ParentTransaction::restore(
        &context.static_context.vegetation_configuration,
        value.v11_parent_checkpoint.clone(),
    )
    .map_err(nested)?;
    if value.v11_parent_checkpoint.parent_transaction_id != coupled_clock.parent_transaction_id()
        || value.v11_parent_checkpoint.accepted_until_ns != coupled_clock.accepted_until().get()
    {
        return Err(SnowStage3V11RestartError::Identity(
            "V11 parent/coupled-clock chronology join",
        ));
    }
    if let Some(publication_ending) = real_consumer
        .restart_authority_accepted_publication_traversed_ending_owner_sha256()
        .map_err(nested)?
        && !posture.defers_pre_support_reappearance_publication()
    {
        let coupled_ending = complete_owner_set_digest(coupled_clock.owners()).map_err(nested)?;
        if publication_ending != coupled_ending {
            return Err(SnowStage3V11RestartError::Identity(
                "accepted publication event handoff/current owner join",
            ));
        }
    }
    let receipt_bytes = decode_blob(
        &value.receipt_state_canonical_base64,
        &value.receipt_state_sha256,
    )?;
    let (terminal_parcels, mut receipt_chain) = restart_authority_decode_receipt_state_v2(
        &receipt_bytes,
        &context.static_context.vegetation_configuration,
    )
    .map_err(nested)?;
    if receipt_chain.last().is_some_and(|receipt| {
        receipt.day_index.checked_add(1).is_none_or(|next_day| {
            u64::try_from(next_day).ok() != Some(value.real_consumer_next_day_index.0)
        })
    }) {
        return Err(SnowStage3V11RestartError::Identity(
            "receipt-chain/provider day chronology",
        ));
    }
    let last_v11_parent_candidate =
        if value.has_last_v11_parent_candidate && posture.owns_serialized_last_candidate() {
            if value.v11_parent_checkpoint.accepted_segments.is_empty() {
                return Err(SnowStage3V11RestartError::CompletedParentPosture {
                    accepted_segments: 0,
                    zero_duration_transitions: value
                        .v11_parent_checkpoint
                        .accepted_zero_duration_owner_transitions
                        .len(),
                });
            }
            let finalized = v11_parent_state
                .clone()
                .finalize(&context.static_context.vegetation_configuration)
                .map_err(nested)?;
            Some(reconstruct_completed_parent_candidate(
                &value.v11_parent_checkpoint,
                finalized,
                &coupled_clock,
                &real_consumer,
            )?)
        } else {
            exact_checkpoint_owner_join(&value.v11_parent_checkpoint, &coupled_clock)?;
            None
        };
    if for_v3
        && let (Some(receipt), Some(candidate)) =
            (receipt_chain.last_mut(), last_v11_parent_candidate.as_ref())
    {
        if receipt.ending_v11_parent_state.checkpoint() != value.v11_parent_checkpoint
            || receipt.ending_last_v11_parent_candidate.is_none()
        {
            return Err(SnowStage3V11RestartError::Identity(
                "V3 receipt/candidate checkpoint authority join",
            ));
        }
        // The V2 receipt wire intentionally stores the sealed ending parent
        // checkpoint plus candidate posture, not a duplicate candidate blob.
        // V3 support-liquid receiver transitions can advance exact physical
        // owners after the checkpoint's cached `finalize` projection. Reuse
        // the independently reconstructed, clock-bound candidate only after
        // the receipt checkpoint and posture match exactly.
        receipt.ending_last_v11_parent_candidate = Some(candidate.clone());
    }
    let stage3_by_lane: std::collections::BTreeMap<_, _> =
        value.stage3_by_lane.iter().cloned().collect();
    Ok(DirectSnowStage3V11CommittedState {
        stage3_by_lane,
        real_consumer,
        v11_parent_state,
        coupled_clock,
        next_parent_sequence: value.next_parent_sequence.to_u128(),
        last_v11_parent_candidate,
        terminal_parcels,
        receipt_chain,
        snow_enthalpy_material_owner: None,
        snow_enthalpy_material_owner_chronology: Vec::new(),
    })
}

fn restore_real_consumer(
    value: &DirectSnowStage3V11CommittedRestartV2,
    context: &ExpectedRestartStaticContext<'_>,
    posture: CommittedRestartPosture,
    coupled_clock: &openwepp_coupled_time::CoupledClockStateV1,
    for_v3: bool,
) -> Result<DirectV10RealConsumerShadow, SnowStage3V11RestartError> {
    let (run, topology) = checkpoint_identities_v1(
        &value.real_consumer,
        context.root_zone_hydraulic_configuration,
    )
    .map_err(SnowStage3V11RestartError::Projection)?;
    if &run != context.run_identity_sha256 || &topology != context.topology_sha256 {
        return Err(SnowStage3V11RestartError::Identity(
            "real-consumer run/topology context",
        ));
    }
    let exact_accepted_interval_count = value.real_consumer_accepted_interval_count.get();
    let expected_between_days_count = value.real_consumer_next_day_index.0.checked_mul(48).ok_or(
        SnowStage3V11RestartError::Identity("real-consumer scheduler position overflow"),
    )?;
    let mut admitted_consumer = (*value.real_consumer).clone();
    let v3_between_days_surface_liquid_bytes = if for_v3 && !posture.is_in_progress() {
        value
            .real_consumer
            .scientific
            .direct_hydrology
            .surface_liquid_owned_state
            .as_deref()
            .map(|surface| {
                surface
                    .restore_with_configuration(context.surface_liquid_configuration)
                    .map_err(nested)?
                    .canonical_bytes(context.surface_liquid_configuration)
                    .map_err(nested)
            })
            .transpose()?
    } else {
        None
    };
    let logical_finalization = normalize_parent_finalization_for_v1_admission(
        &mut admitted_consumer,
        context,
        posture,
        coupled_clock,
        for_v3,
    )?;
    let exact_surface_liquid_bytes = if posture.is_in_progress() {
        let next_interval_index = exact_accepted_interval_count
            .checked_sub(expected_between_days_count)
            .and_then(|value| u8::try_from(value).ok())
            .filter(|value| *value <= 48)
            .ok_or(SnowStage3V11RestartError::Identity(
                "in-progress real-consumer scheduler position",
            ))?;
        let surface = value
            .real_consumer
            .scientific
            .direct_hydrology
            .surface_liquid_owned_state
            .as_deref()
            .ok_or(SnowStage3V11RestartError::Identity(
                "in-progress surface-liquid owner",
            ))?;
        if surface.continuations.is_empty()
            || surface.continuations.iter().any(|continuation| {
                continuation.day_index != value.real_consumer_next_day_index.0
                    || continuation.next_interval_index != next_interval_index
            })
        {
            return Err(SnowStage3V11RestartError::Identity(
                "in-progress surface-liquid scheduler join",
            ));
        }
        let surface = surface
            .restore_with_configuration(context.surface_liquid_configuration)
            .map_err(nested)?;
        admitted_consumer
            .scientific
            .direct_hydrology
            .surface_liquid_owned_state = None;
        Some(
            surface
                .canonical_bytes(context.surface_liquid_configuration)
                .map_err(nested)?,
        )
    } else {
        v3_between_days_surface_liquid_bytes
    };
    let admitted_interval_count = if exact_surface_liquid_bytes.is_some() {
        AcceptedIntervalCount::try_new(expected_between_days_count).map_err(nested)?
    } else {
        value.real_consumer_accepted_interval_count
    };
    let mut checkpoint = DirectV10RealConsumerCheckpointV1 {
        schema: "OPENWEPP_DIRECT_V10_REAL_CONSUMER_CHECKPOINT_V1".into(),
        version: 1,
        run_identity_sha256: run,
        topology_sha256: topology,
        phase: DirectV10CheckpointPhaseV1::BetweenDays {
            next_day_index: value.real_consumer_next_day_index,
            accepted_interval_count: admitted_interval_count,
            committed: admitted_consumer,
        },
        payload_sha256: Sha256Hex::try_new("0".repeat(64))
            .map_err(|_| SnowStage3V11RestartError::Projection("consumer digest seed"))?,
    };
    checkpoint
        .seal()
        .map_err(|_| SnowStage3V11RestartError::Identity("real-consumer checkpoint seal"))?;
    let bytes = to_canonical_bytes(&checkpoint).map_err(|_| {
        SnowStage3V11RestartError::Identity("real-consumer checkpoint canonical bytes")
    })?;
    let restored = admit_checkpoint_v1(&bytes, context)
        .map_err(SnowStage3V11RestartError::RealConsumerAdmission)?;
    let host = DirectV10RestartHost::from_isolated(restored, context)
        .map_err(|error| nested_phase("real-consumer checkpoint host reconstruction", error))?;
    let mut shadow = host.shadow().clone();
    if shadow.restart_authority_accepted_interval_count() != admitted_interval_count.get() {
        return Err(SnowStage3V11RestartError::Identity(
            "real-consumer admitted scheduler position",
        ));
    }
    let admitted_vegetation = shadow.vegetation_state().0.last_transaction_id;
    let admitted_lse = shadow
        .lse_state()
        .0
        .last_accepted_transaction_id
        .map(|value| value.0);
    let admitted_soil = shadow
        .restart_authority_soil_thermal()
        .map_err(|error| nested_phase("V1 soil resident admission", error))?
        .last_accepted_transaction_id
        .map(|value| value.0);
    let admitted_biogeochemistry = shadow
        .restart_authority_biogeochemistry()
        .last_transaction_id;
    if admitted_interval_count.get() != 0
        && (admitted_lse != Some(admitted_vegetation)
            || admitted_soil != Some(admitted_vegetation)
            || admitted_biogeochemistry != admitted_vegetation)
    {
        return Err(SnowStage3V11RestartError::AdmittedSchedulerLineage {
            next_day: value.real_consumer_next_day_index.0,
            accepted_count: admitted_interval_count.get(),
            vegetation: admitted_vegetation,
            lse: admitted_lse,
            soil: admitted_soil,
            biogeochemistry: admitted_biogeochemistry,
        });
    }
    if let Some(surface_liquid_bytes) = exact_surface_liquid_bytes.as_deref() {
        shadow
            .restart_authority_restore_surface_liquid_canonical_bytes(surface_liquid_bytes)
            .map_err(|error| nested_phase("surface-liquid owner restore", error))?;
        if exact_accepted_interval_count != expected_between_days_count {
            shadow
                .restart_authority_install_scheduler_position(exact_accepted_interval_count)
                .map_err(|error| nested_phase("scheduler position restore", error))?;
        }
    }
    if !value.real_consumer_provider_cursor_configuration_bound {
        if value.real_consumer_next_day_index.0 != 0
            || !value.real_consumer.provider_cursor.pending_carry.is_empty()
            || !value
                .real_consumer
                .provider_cursor
                .pending_solid_carry
                .is_empty()
        {
            return Err(SnowStage3V11RestartError::Identity(
                "unbound provider cursor posture",
            ));
        }
        shadow
            .restart_authority_install_staged_daily_owners(
                shadow.gsi_state().clone(),
                SnowFreeHalfHourProviderCursor::default(),
                0,
            )
            .map_err(nested)?;
    }
    let wb14_parent = match (
        value.real_consumer_wb14_parent_canonical_base64.as_deref(),
        value.real_consumer_wb14_parent_sha256.as_ref(),
    ) {
        (None, None) => None,
        (Some(encoded), Some(digest)) => Some(decode_blob(encoded, digest)?),
        _ => {
            return Err(SnowStage3V11RestartError::Identity(
                "WB14 parent blob posture",
            ));
        }
    };
    shadow
        .restart_authority_restore_wb14_parent_canonical_bytes(wb14_parent.as_deref())
        .map_err(|error| nested_phase("WB14 parent restore", error))?;
    let accepted_publication_supports = decode_blob(
        &value.accepted_publication_supports_canonical_base64,
        &value.accepted_publication_supports_sha256,
    )?;
    if for_v3 {
        shadow
            .restart_authority_restore_accepted_publication_active_tail_canonical_bytes_v3(
                &accepted_publication_supports,
            )
            .map_err(|error| nested_phase("V3 active publication-tail restore", error))?;
    } else {
        shadow
            .restart_authority_restore_accepted_publication_supports_canonical_bytes(
                &accepted_publication_supports,
            )
            .map_err(|error| nested_phase("accepted publication restore", error))?;
    }
    if let Some((vegetation, biogeochemistry)) = logical_finalization {
        shadow
            .restart_authority_restore_parent_finalization_logical_owners(
                vegetation,
                biogeochemistry,
            )
            .map_err(|error| nested_phase("parent-finalization logical owners", error))?;
    }
    Ok(shadow)
}

fn normalize_parent_finalization_for_v1_admission(
    admitted: &mut CompleteCommittedOwnerStateV1,
    context: &ExpectedRestartStaticContext<'_>,
    posture: CommittedRestartPosture,
    coupled_clock: &openwepp_coupled_time::CoupledClockStateV1,
    for_v3: bool,
) -> Result<
    Option<(
        openwepp_vegetation::V10CoupledOwnedState,
        openwepp_biogeochemistry::BiogeochemistryState,
    )>,
    SnowStage3V11RestartError,
> {
    let scientific = &admitted.scientific;
    let vegetation_transaction = scientific.vegetation_v10.last_transaction_id.to_u128();
    let biogeochemistry_transaction = scientific.biogeochemistry.last_transaction_id.to_u128();
    let lse_transaction = scientific
        .lse_v2
        .last_accepted_transaction_id
        .as_ref()
        .map(HexU128::to_u128);
    let soil_transaction = scientific
        .soil_thermal
        .last_accepted_transaction_id
        .as_ref()
        .map(HexU128::to_u128);
    let surface_lineages = scientific
        .direct_hydrology
        .surface_liquid_owned_state
        .as_deref()
        .into_iter()
        .flat_map(|state| {
            state
                .records
                .iter()
                .map(|record| {
                    record
                        .last_accepted_transaction_id
                        .as_ref()
                        .map(HexU128::to_u128)
                })
                .chain(state.continuations.iter().map(|continuation| {
                    continuation
                        .last_accepted_transaction_id
                        .as_ref()
                        .map(HexU128::to_u128)
                }))
        })
        .collect::<Vec<_>>();
    if lse_transaction.is_none_or(|value| value == vegetation_transaction)
        && soil_transaction.is_none_or(|value| value == vegetation_transaction)
        && biogeochemistry_transaction == vegetation_transaction
        && surface_lineages
            .iter()
            .all(|lineage| lineage.is_none_or(|value| value == vegetation_transaction))
    {
        return Ok(None);
    }
    let predecessor =
        vegetation_transaction
            .checked_sub(1)
            .ok_or(SnowStage3V11RestartError::Identity(
                "parent-finalization predecessor underflow",
            ))?;
    let receiver_surface_successor =
        vegetation_transaction
            .checked_add(1)
            .ok_or(SnowStage3V11RestartError::Identity(
                "receiver surface successor overflow",
            ))?;
    if posture.permits_receiver_surface_successor()
        && biogeochemistry_transaction == vegetation_transaction
        && lse_transaction == Some(vegetation_transaction)
        && soil_transaction == Some(vegetation_transaction)
        && !surface_lineages.is_empty()
        && surface_lineages
            .iter()
            .all(|lineage| *lineage == Some(receiver_surface_successor))
    {
        let surface = admitted
            .scientific
            .direct_hydrology
            .surface_liquid_owned_state
            .as_deref()
            .ok_or(SnowStage3V11RestartError::Identity(
                "receiver surface admission owner",
            ))?
            .restore()
            .map_err(nested)?;
        let exact_surface_bytes = surface
            .canonical_bytes(context.surface_liquid_configuration)
            .map_err(nested)?;
        let normalized = surface
            .restart_authority_with_admission_lineage(
                context.surface_liquid_configuration,
                openwepp_kernel_contract::TransactionId(vegetation_transaction),
            )
            .map_err(nested)?;
        let surface_owner_id = "surface_liquid";
        let matching_surface_owners = coupled_clock
            .owners()
            .iter()
            .filter(|owner| owner.owner_id() == surface_owner_id)
            .collect::<Vec<_>>();
        if matching_surface_owners.len() != 1
            || matching_surface_owners[0].state_bytes() != exact_surface_bytes
        {
            return Err(SnowStage3V11RestartError::Identity(
                "receiver surface clock owner",
            ));
        }
        let ending_digest = complete_owner_set_digest(coupled_clock.owners()).map_err(nested)?;
        let receiver_event = coupled_clock.accepted_event_receipts().last().ok_or(
            SnowStage3V11RestartError::Identity("receiver accepted event"),
        )?;
        receiver_event.validate().map_err(nested)?;
        if receiver_event.tick() != coupled_clock.accepted_until()
            || receiver_event.parent_transaction_id() != coupled_clock.parent_transaction_id()
            || receiver_event.ending_owner_set_digest() != ending_digest
            || receiver_event.beginning_owner_set_digest() == ending_digest
        {
            return Err(SnowStage3V11RestartError::Identity(
                "receiver accepted event/clock chain",
            ));
        }
        admitted
            .scientific
            .direct_hydrology
            .surface_liquid_owned_state = Some(Box::new(
            crate::DirectSurfaceLiquidOwnedStateRestartV1::project(&normalized).map_err(nested)?,
        ));
        return Ok(None);
    }
    // A zero-duration terminal receiver advances the physical LSE/soil owner
    // lineage with vegetation/BGC while the exact surface-liquid continuation
    // remains on its positive-support predecessor.  V1 host admission requires
    // one lineage, so use an identity-only surface clone for admission and let
    // `restore_real_consumer` reinstall the independently sealed exact surface
    // bytes immediately afterward.
    if biogeochemistry_transaction == vegetation_transaction
        && lse_transaction == Some(vegetation_transaction)
        && soil_transaction == Some(vegetation_transaction)
        && !surface_lineages.is_empty()
        && surface_lineages
            .iter()
            .all(|lineage| *lineage == Some(predecessor))
    {
        let surface = admitted
            .scientific
            .direct_hydrology
            .surface_liquid_owned_state
            .as_deref()
            .ok_or(SnowStage3V11RestartError::Identity(
                "receiver surface admission owner",
            ))?
            .restore()
            .map_err(nested)?;
        let normalized = surface
            .restart_authority_with_admission_lineage(
                context.surface_liquid_configuration,
                openwepp_kernel_contract::TransactionId(vegetation_transaction),
            )
            .map_err(nested)?;
        admitted
            .scientific
            .direct_hydrology
            .surface_liquid_owned_state = Some(Box::new(
            crate::DirectSurfaceLiquidOwnedStateRestartV1::project(&normalized).map_err(nested)?,
        ));
        return Ok(None);
    }
    if biogeochemistry_transaction != vegetation_transaction {
        return Err(SnowStage3V11RestartError::ParentFinalizationLineage {
            owner: "biogeochemistry",
            expected: vegetation_transaction,
            actual: Some(biogeochemistry_transaction),
        });
    }
    if lse_transaction != Some(predecessor) {
        return Err(SnowStage3V11RestartError::ParentFinalizationLineage {
            owner: "land-surface-energy",
            expected: predecessor,
            actual: lse_transaction,
        });
    }
    if soil_transaction != Some(predecessor) {
        return Err(SnowStage3V11RestartError::ParentFinalizationLineage {
            owner: "soil-thermal",
            expected: predecessor,
            actual: soil_transaction,
        });
    }
    if surface_lineages
        .iter()
        .any(|lineage| *lineage != Some(predecessor))
    {
        if !for_v3 || surface_lineages.is_empty() {
            return Err(SnowStage3V11RestartError::Identity(
                "parent-finalization surface predecessor",
            ));
        }
        // V3 separately seals the support-liquid receiver custody that can
        // advance the physical surface owner more than once during a parent.
        // The V1 host still requires a single parent-finalization lineage, so
        // admit an identity-only predecessor clone only after the exact
        // receiver-advanced bytes match the coupled-clock surface owner. The
        // caller reinstalls these independently sealed exact bytes before the
        // V3 custody supplemental is installed and strictly validated.
        let surface = admitted
            .scientific
            .direct_hydrology
            .surface_liquid_owned_state
            .as_deref()
            .ok_or(SnowStage3V11RestartError::Identity(
                "parent-finalization V3 surface owner",
            ))?
            .restore_with_configuration(context.surface_liquid_configuration)
            .map_err(nested)?;
        let exact_surface_bytes = surface
            .canonical_bytes(context.surface_liquid_configuration)
            .map_err(nested)?;
        let matching_surface_owners = coupled_clock
            .owners()
            .iter()
            .filter(|owner| owner.owner_id() == "surface_liquid")
            .collect::<Vec<_>>();
        if matching_surface_owners.len() != 1
            || matching_surface_owners[0].state_bytes() != exact_surface_bytes
        {
            return Err(SnowStage3V11RestartError::Identity(
                "parent-finalization V3 surface clock owner",
            ));
        }
        let normalized = surface
            .restart_authority_with_admission_lineage(
                context.surface_liquid_configuration,
                openwepp_kernel_contract::TransactionId(predecessor),
            )
            .map_err(nested)?;
        admitted
            .scientific
            .direct_hydrology
            .surface_liquid_owned_state = Some(Box::new(
            crate::DirectSurfaceLiquidOwnedStateRestartV1::project(&normalized).map_err(nested)?,
        ));
    }
    let finalized_vegetation = admitted
        .scientific
        .vegetation_v10
        .restore(
            context.vegetation_configuration,
            context.vegetation_owner_id,
        )
        .map_err(nested)?;
    let finalized_biogeochemistry = admitted
        .scientific
        .biogeochemistry
        .restore()
        .map_err(nested)?;
    let mut normalized_vegetation = finalized_vegetation.clone();
    normalized_vegetation.0.last_transaction_id = predecessor;
    normalized_vegetation.0.state_sha256 = normalized_vegetation.0.canonical_sha256();
    admitted.scientific.vegetation_v10 = VegetationV10StateRestartV1::project(
        &normalized_vegetation,
        context.vegetation_configuration,
        context.vegetation_owner_id,
    )
    .map_err(nested)?;
    let mut normalized_biogeochemistry = finalized_biogeochemistry.clone();
    normalized_biogeochemistry.last_transaction_id = predecessor;
    admitted.scientific.biogeochemistry =
        crate::BiogeochemistryStateRestartV1::project(&normalized_biogeochemistry)
            .map_err(nested)?;
    Ok(Some((finalized_vegetation, finalized_biogeochemistry)))
}

fn provider_cursor_configuration_bound(
    cursor: &openwepp_hillslope_orchestrator::runtime_inputs::SnowFreeHalfHourProviderCursor,
) -> Result<bool, SnowStage3V11RestartError> {
    let bytes = cursor.to_json_bytes().map_err(nested)?;
    let value: serde_json::Value = serde_json::from_slice(&bytes).map_err(nested)?;
    match value.get("configuration_sha256") {
        Some(serde_json::Value::Null) => Ok(false),
        Some(serde_json::Value::String(digest)) if digest.len() == 64 => Ok(true),
        _ => Err(SnowStage3V11RestartError::Identity(
            "provider cursor binding posture",
        )),
    }
}

fn static_context_sha256(
    value: &DirectSnowStage3V11StaticContext,
) -> Result<Sha256Hex, SnowStage3V11RestartError> {
    Sha256Hex::try_new(
        crate::canonical_sha256(&StaticContextIdentity {
            run_identity: value.run_identity,
            topology_identity: value.topology_identity,
            parent_duration_ns: value.parent_duration_ns.to_string(),
            minimum_support_ns: value.minimum_support_ns.to_string(),
            calendar_receipt: value.calendar_receipt,
            controller_policy: value.controller_policy,
            parent_sequence: value.parent_sequence.to_string(),
            lane_ids: &value.lane_ids,
            vegetation_configuration_sha256: &value.vegetation_configuration.configuration_sha256,
            surface_liquid_configuration_sha256: &value
                .surface_liquid_configuration
                .configuration_sha256,
            wb14_parameters: &value.wb14_parameters,
        })
        .map_err(nested)?,
    )
    .map_err(|_| SnowStage3V11RestartError::Projection("static context digest"))
}

fn sha256_hex(bytes: &[u8]) -> Result<Sha256Hex, SnowStage3V11RestartError> {
    Sha256Hex::try_new(format!("{:x}", Sha256::digest(bytes)))
        .map_err(|_| SnowStage3V11RestartError::Projection("blob digest"))
}

fn decode_blob(encoded: &str, expected: &Sha256Hex) -> Result<Vec<u8>, SnowStage3V11RestartError> {
    let bytes = STANDARD
        .decode(encoded)
        .map_err(|_| SnowStage3V11RestartError::Identity("base64"))?;
    if STANDARD.encode(&bytes) != encoded || &sha256_hex(&bytes)? != expected {
        return Err(SnowStage3V11RestartError::Identity("blob digest"));
    }
    Ok(bytes)
}

#[cfg(all(test, feature = "fixtures"))]
#[path = "snow_stage3_v11_tests.rs"]
mod tests;
