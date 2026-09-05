use std::cell::RefCell;
use std::collections::BTreeMap;

use openwepp_coupled_time::{Digest32, ModelTimeNs, TimeSupport};

use crate::hydrology::DirectSnowStage3PersistentState;
use crate::runtime_inputs::{HillslopeClimateRuntimeRequest, PreparedSnowFreeGsiDayV1};
use crate::snow_stage3_v11_attachment::{
    DirectSnowStage3V11AttachmentError, DirectSnowStage3V11CommittedState,
    DirectSnowStage3V11DualRegimeSupportInputsV1, DirectSnowStage3V11PreparedDay,
    DirectSnowStage3V11PreparedSupport, DirectSnowStage3V11ProductionConfigurationV1,
    DirectSnowStage3V11ShadowAttachment, DirectSnowStage3V11StaticContext, PreparedStage3V11DayV1,
    STAGE3_V11_DAY_NS, STAGE3_V11_PARENT_SUPPORT_COUNT, STAGE3_V11_PARENT_SUPPORT_NS,
};
use crate::v9_real_consumer_shadow::DirectV10RealConsumerShadow;
use crate::v9_real_consumer_shadow::{
    DirectV9ShadowIntervalInput, DirectV11SnowCoveredSegmentInput,
};

use super::{
    DirectDayFrame, DirectPublicationDayInput, DirectRunFrame, DirectRuntimeError,
    Stage3AcceptedPublicationDayV1,
};

/// Result-blind observation of successful outer-frame adoption.  This audit
/// records only after the validated committed frame and its attachment have
/// both replaced the live runner state.
#[doc(hidden)]
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct SnowStage3V11AttachmentAdoptionAuditV1 {
    pub native_inactive_prefix_validation_count: u64,
    pub native_inactive_prefix_receipt_counts: Vec<usize>,
    pub accepted_history_append_count: u64,
    pub appended_support_sha256: Vec<Digest32>,
    pub staged_history_append_attempt_count: u64,
    pub successful_adoption_count: u64,
    pub accepted_support_sha256: Vec<Digest32>,
}

thread_local! {
    static SNOW_STAGE3_V11_ATTACHMENT_ADOPTION_AUDIT: RefCell<Option<SnowStage3V11AttachmentAdoptionAuditV1>> = const { RefCell::new(None) };
}

#[doc(hidden)]
pub fn begin_snow_stage3_v11_attachment_adoption_audit_v1() {
    SNOW_STAGE3_V11_ATTACHMENT_ADOPTION_AUDIT.with(|audit| {
        assert!(
            audit
                .replace(Some(SnowStage3V11AttachmentAdoptionAuditV1::default()))
                .is_none(),
            "nested Stage-3/V11 attachment-adoption audit"
        );
    });
}

#[doc(hidden)]
pub fn take_snow_stage3_v11_attachment_adoption_audit_v1() -> SnowStage3V11AttachmentAdoptionAuditV1
{
    SNOW_STAGE3_V11_ATTACHMENT_ADOPTION_AUDIT.with(|audit| {
        audit
            .replace(None)
            .expect("Stage-3/V11 attachment-adoption audit was not begun")
    })
}

fn record_snow_stage3_v11_attachment_adoption_v1(
    accepted_support_sha256: Digest32,
    installed_history: Vec<Digest32>,
) {
    SNOW_STAGE3_V11_ATTACHMENT_ADOPTION_AUDIT.with(|audit| {
        let mut audit = audit.borrow_mut();
        let Some(audit) = audit.as_mut() else {
            return;
        };
        audit.successful_adoption_count = audit
            .successful_adoption_count
            .checked_add(1)
            .expect("Stage-3/V11 attachment-adoption audit count overflow");
        audit.accepted_support_sha256.push(accepted_support_sha256);
        audit.accepted_history_append_count = u64::try_from(installed_history.len())
            .expect("Stage-3/V11 installed history count width");
        audit.appended_support_sha256 = installed_history;
    });
}

pub(crate) fn record_snow_stage3_v11_accepted_history_append_v1(accepted_support_sha256: Digest32) {
    SNOW_STAGE3_V11_ATTACHMENT_ADOPTION_AUDIT.with(|audit| {
        let mut audit = audit.borrow_mut();
        let Some(audit) = audit.as_mut() else {
            return;
        };
        audit.staged_history_append_attempt_count = audit
            .staged_history_append_attempt_count
            .checked_add(1)
            .expect("Stage-3/V11 staged-history append audit count overflow");
        let _ = accepted_support_sha256;
    });
}

pub(crate) fn record_snow_stage3_v11_native_inactive_prefix_validation_v1(receipt_count: usize) {
    SNOW_STAGE3_V11_ATTACHMENT_ADOPTION_AUDIT.with(|audit| {
        let mut audit = audit.borrow_mut();
        let Some(audit) = audit.as_mut() else {
            return;
        };
        audit.native_inactive_prefix_validation_count = audit
            .native_inactive_prefix_validation_count
            .checked_add(1)
            .expect("Stage-3/V11 native inactive-prefix audit count overflow");
        audit
            .native_inactive_prefix_receipt_counts
            .push(receipt_count);
    });
}

impl DirectRunFrame {
    /// Seal the just-committed complete day into bounded qualification and
    /// content-addressed archive evidence. The detailed day remains resident
    /// until the runner durably spools and acknowledges the exact record.
    pub fn stage_snow_stage3_v11_committed_day_archive(
        &mut self,
        day_index: usize,
    ) -> Result<(), DirectRuntimeError> {
        let attachment = self.snow_stage3_v11_attachment.as_deref_mut().ok_or(
            DirectRuntimeError::DirectKernelGuardFailure {
                phase: "snow_stage3_v11.archive_stage",
                detail: "constitutive attachment is not installed".into(),
            },
        )?;
        let delta = attachment
            .build_qualification_day_delta_v1(day_index)
            .map_err(attachment_runtime_error("qualification_day_delta"))?;
        attachment
            .stage_committed_day_archive_v1(delta)
            .map_err(attachment_runtime_error("archive_stage"))
    }

    pub fn snow_stage3_v11_pending_committed_day_evidence(
        &self,
    ) -> Result<
        &crate::snow_stage3_v11_attachment::Stage3PendingCommittedDayEvidenceV1,
        DirectRuntimeError,
    > {
        self.snow_stage3_v11_attachment
            .as_deref()
            .and_then(DirectSnowStage3V11ShadowAttachment::pending_committed_day_evidence_v1)
            .ok_or(DirectRuntimeError::DirectKernelGuardFailure {
                phase: "snow_stage3_v11.archive_pending",
                detail: "committed day archive evidence is not staged".into(),
            })
    }

    pub fn write_snow_stage3_v11_pending_committed_day_evidence(
        &self,
        writer: &mut (impl std::io::Write + ?Sized),
    ) -> Result<(), DirectRuntimeError> {
        let attachment = self.snow_stage3_v11_attachment.as_deref().ok_or(
            DirectRuntimeError::DirectKernelGuardFailure {
                phase: "snow_stage3_v11.archive_stream",
                detail: "constitutive attachment is not installed".into(),
            },
        )?;
        attachment
            .write_pending_committed_day_evidence_v1(writer)
            .map_err(attachment_runtime_error("archive_stream"))
    }

    pub fn acknowledge_snow_stage3_v11_committed_day_archive(
        &mut self,
        record_sha256: Digest32,
    ) -> Result<(), DirectRuntimeError> {
        let attachment = self.snow_stage3_v11_attachment.as_deref_mut().ok_or(
            DirectRuntimeError::DirectKernelGuardFailure {
                phase: "snow_stage3_v11.archive_acknowledge",
                detail: "constitutive attachment is not installed".into(),
            },
        )?;
        attachment
            .acknowledge_committed_day_archive_v1(record_sha256)
            .map_err(attachment_runtime_error("archive_acknowledge"))
    }

    pub fn snow_stage3_v11_archived_receipt_prefix(
        &self,
    ) -> Result<&crate::snow_stage3_v11_attachment::Stage3ArchivedReceiptPrefixV1, DirectRuntimeError>
    {
        self.snow_stage3_v11_attachment
            .as_deref()
            .map(DirectSnowStage3V11ShadowAttachment::archived_receipt_prefix_v1)
            .ok_or(DirectRuntimeError::DirectKernelGuardFailure {
                phase: "snow_stage3_v11.archive_prefix",
                detail: "constitutive attachment is not installed".into(),
            })
    }

    /// Return sealed evidence reconstructed exclusively from fully committed
    /// Stage-3 owners and their accepted support receipts.
    pub fn snow_stage3_v11_production_qualification_snapshot(
        &self,
    ) -> Result<
        crate::snow_stage3_v11_attachment::SnowStage3V11ProductionQualificationSnapshotV1,
        DirectRuntimeError,
    > {
        let attachment = self.snow_stage3_v11_attachment.as_deref().ok_or(
            DirectRuntimeError::DirectKernelGuardFailure {
                phase: "snow_stage3_v11.qualification",
                detail: "constitutive attachment is not installed".into(),
            },
        )?;
        attachment
            .production_qualification_snapshot()
            .map_err(attachment_runtime_error("qualification"))
    }

    /// Prepare one provider/GSI day from the live owners retained by the
    /// installed attachment. The returned capability is not yet staged; it
    /// must be joined to the 48 physical support inputs for this same day.
    pub fn prepare_snow_stage3_v11_repository_provider_day(
        &self,
        climate: &HillslopeClimateRuntimeRequest,
        day_index: usize,
    ) -> Result<PreparedSnowFreeGsiDayV1, DirectRuntimeError> {
        let attachment = self.snow_stage3_v11_attachment.as_deref().ok_or(
            DirectRuntimeError::DirectKernelGuardFailure {
                phase: "snow_stage3_v11.prepare_provider",
                detail: "constitutive attachment is not installed".into(),
            },
        )?;
        attachment
            .prepare_repository_provider_day(climate, day_index)
            .map_err(attachment_runtime_error("prepare_provider"))
    }

    /// Install the production Stage-3 owner from physical beginning state.
    /// V11 owner envelopes, the parent transaction, coupled clock,
    /// participants, lane manifest, and sequence are derived inside the
    /// attachment and cannot be assembled by the runner.
    pub fn initialize_snow_stage3_v11_production(
        &mut self,
        configuration: DirectSnowStage3V11ProductionConfigurationV1,
        stage3_by_lane: BTreeMap<u32, DirectSnowStage3PersistentState>,
        real_consumer: DirectV10RealConsumerShadow,
    ) -> Result<(), DirectRuntimeError> {
        if self.snow_stage3_v11_attachment.is_some() {
            return Err(DirectRuntimeError::DirectKernelGuardFailure {
                phase: "snow_stage3_v11.initialize",
                detail: "production attachment is already installed".into(),
            });
        }
        if real_consumer.hydrology_frame().identity != self.identity
            || stage3_by_lane.len() != self.identity.lane_count
            || !self
                .lanes
                .iter()
                .all(|lane| stage3_by_lane.contains_key(&lane.lane_id) && lane.lane_id != 0)
        {
            return Err(DirectRuntimeError::DirectKernelGuardFailure {
                phase: "snow_stage3_v11.initialize",
                detail: "production frame, real-consumer, and Stage-3 lane identity".into(),
            });
        }
        let attachment = DirectSnowStage3V11ShadowAttachment::new_production(
            configuration,
            stage3_by_lane,
            real_consumer,
        )
        .map_err(attachment_runtime_error("initialize"))?;
        self.snow_stage3_v11_attachment = Some(Box::new(attachment));
        Ok(())
    }

    pub fn configure_snow_stage3_v11_attachment(
        &mut self,
        static_context: DirectSnowStage3V11StaticContext,
        committed: DirectSnowStage3V11CommittedState,
    ) -> Result<(), DirectRuntimeError> {
        let attachment = DirectSnowStage3V11ShadowAttachment::new(static_context, committed)
            .map_err(attachment_runtime_error("configure"))?;
        self.snow_stage3_v11_attachment = Some(Box::new(attachment));
        Ok(())
    }

    pub(crate) fn prepare_snow_stage3_v11_day(
        &mut self,
        prepared: DirectSnowStage3V11PreparedDay,
    ) -> Result<(), DirectRuntimeError> {
        let attachment = self.snow_stage3_v11_attachment.as_deref_mut().ok_or(
            DirectRuntimeError::DirectKernelGuardFailure {
                phase: "snow_stage3_v11.prepare",
                detail: "constitutive attachment is not installed".into(),
            },
        )?;
        attachment
            .stage_prepared_day(&prepared)
            .map_err(attachment_runtime_error("prepare"))
    }

    /// Bind the runner's already validated GSI/provider day to the 48
    /// runner-built supports before installation. The provider day owns the
    /// atmospheric clock and cursor transition; no completed daily result can
    /// satisfy this boundary.
    pub fn prepare_snow_stage3_v11_day_from_provider(
        &mut self,
        provider: &PreparedSnowFreeGsiDayV1,
        day_index: usize,
        supports: Vec<crate::snow_stage3_v11_attachment::DirectSnowStage3V11PreparedSupport>,
    ) -> Result<(), DirectRuntimeError> {
        let prepared =
            PreparedStage3V11DayV1::bind_production_provider_day(provider, day_index, supports)
                .map_err(attachment_runtime_error("bind_provider"))?;
        self.prepare_snow_stage3_v11_day(prepared)
    }

    /// Build and stage exactly 48 run-relative half-hour supports just in
    /// time for one provider/GSI day. `build_support` receives chronology,
    /// not a predicted snow regime. Each returned support must carry the
    /// sealed dual-regime capability enforced by
    /// `bind_production_provider_day`; live sequential attachment state alone
    /// chooses snow-free, covered, meltout, or reappearance execution.
    pub fn prepare_snow_stage3_v11_production_day<F>(
        &mut self,
        provider: &PreparedSnowFreeGsiDayV1,
        day_index: usize,
        mut build_support: F,
    ) -> Result<(), DirectRuntimeError>
    where
        F: FnMut(
            usize,
            TimeSupport,
        ) -> Result<
            DirectSnowStage3V11DualRegimeSupportInputsV1,
            DirectSnowStage3V11AttachmentError,
        >,
    {
        let chronology = production_day_support_chronology(day_index)?;
        let mut supports = Vec::with_capacity(STAGE3_V11_PARENT_SUPPORT_COUNT);
        for (support_index, support) in chronology.into_iter().enumerate() {
            let inputs = build_support(support_index, support)
                .map_err(attachment_runtime_error("build_support"))?;
            let prepared = DirectSnowStage3V11PreparedSupport::from_dual_regime_production_inputs(
                support, inputs,
            )
            .map_err(attachment_runtime_error("seal_support"))?;
            supports.push(prepared);
        }
        self.prepare_snow_stage3_v11_day_from_provider(provider, day_index, supports)
    }

    /// Prepare the repository provider/GSI transition from the attachment's
    /// committed V10 owners and stage its 48 physical supports in one call.
    /// The support builder receives only scheduler chronology; it cannot
    /// substitute provider configuration, GSI state, or cursor authority.
    pub fn prepare_snow_stage3_v11_production_day_from_repository<F>(
        &mut self,
        climate: &HillslopeClimateRuntimeRequest,
        day_index: usize,
        interval_template: &DirectV9ShadowIntervalInput,
        build_support: F,
    ) -> Result<(), DirectRuntimeError>
    where
        F: FnMut(
            &PreparedSnowFreeGsiDayV1,
            &DirectV9ShadowIntervalInput,
            &DirectV11SnowCoveredSegmentInput,
            usize,
            TimeSupport,
        ) -> Result<
            DirectSnowStage3V11DualRegimeSupportInputsV1,
            DirectSnowStage3V11AttachmentError,
        >,
    {
        let provider = self.prepare_snow_stage3_v11_repository_provider_day(climate, day_index)?;
        let intervals = self
            .snow_stage3_v11_attachment
            .as_deref()
            .ok_or(DirectRuntimeError::DirectKernelGuardFailure {
                phase: "snow_stage3_v11.prepare_intervals",
                detail: "constitutive attachment is not installed".into(),
            })?
            .prepare_repository_v11_intervals(&provider, interval_template)
            .map_err(attachment_runtime_error("prepare_intervals"))?;
        if intervals.len() != STAGE3_V11_PARENT_SUPPORT_COUNT {
            return Err(DirectRuntimeError::DirectKernelGuardFailure {
                phase: "snow_stage3_v11.prepare_intervals",
                detail: "repository interval cardinality".into(),
            });
        }
        let mut build_support = build_support;
        self.prepare_snow_stage3_v11_production_day(&provider, day_index, |index, support| {
            let (snow_free, covered) =
                intervals
                    .get(index)
                    .ok_or(DirectSnowStage3V11AttachmentError::Support(
                        "repository interval cardinality",
                    ))?;
            build_support(&provider, snow_free, covered, index, support)
        })
    }

    pub(crate) fn stage_snow_stage3_shadow(
        &mut self,
        day_input: &DirectPublicationDayInput,
        day_frame: &DirectDayFrame,
    ) -> Result<(), DirectRuntimeError> {
        #[cfg(test)]
        if let Some(mut attachment) = self.snow_stage3_shadow.take().map(|value| *value) {
            let result = attachment.stage_after_live_day(self, day_input, day_frame);
            self.snow_stage3_shadow = Some(Box::new(attachment));
            return result;
        }
        #[cfg(not(test))]
        let _ = (day_input, day_frame);
        // In production the constitutive attachment is staged only from its
        // sealed 48-support capability. It never reads the completed day
        // frame as a physical beginning state.
        Ok(())
    }

    pub(crate) fn commit_snow_stage3_shadow(
        &mut self,
        publication_inputs: &[DirectPublicationDayInput],
    ) -> Result<(), DirectRuntimeError> {
        #[cfg(test)]
        if let Some(mut attachment) = self.snow_stage3_shadow.take().map(|value| *value) {
            let result = attachment.commit_after_live_day(self);
            self.snow_stage3_shadow = Some(Box::new(attachment));
            return result;
        }
        if let Some(mut attachment) = self
            .snow_stage3_v11_attachment
            .take()
            .map(|attachment| *attachment)
        {
            let day_index = attachment.committed.real_consumer.v11_next_day_index();
            let (
                mut completed_frame,
                supports,
                event_handoffs,
                terminal_event_groups,
                coupled_subslabs,
                beginning_stage3,
                ending_stage3,
                surface_configuration,
            ) = attachment
                .pending_publication_completion_inputs(day_index)
                .map_err(attachment_runtime_error("publication_inputs"))?;
            completed_frame.laned_active.clone_from(&self.laned_active);
            completed_frame
                .laned_active_summary
                .clone_from(&self.laned_active_summary);
            let publication_day = Stage3AcceptedPublicationDayV1::try_complete(
                &mut completed_frame,
                day_index,
                publication_inputs,
                &supports,
                &event_handoffs,
                &terminal_event_groups,
                &coupled_subslabs,
                &beginning_stage3,
                &ending_stage3,
                &surface_configuration,
            )?;
            let laned_active = self.laned_active.clone();
            let laned_active_summary = publication_day.laned_active_summary().cloned();
            attachment
                .complete_pending_publication_day(publication_day)
                .map_err(attachment_runtime_error("publication_complete"))?;
            attachment
                .commit_staged_day()
                .map_err(attachment_runtime_error("commit"))?;
            self.promote_snow_stage3_v11_committed_frame(attachment)?;
            self.laned_active = laned_active;
            self.laned_active_summary = laned_active_summary.map(Box::new);
        }
        Ok(())
    }

    pub(crate) fn committed_snow_stage3_publication_day(
        &self,
        day_index: usize,
    ) -> Result<&Stage3AcceptedPublicationDayV1, DirectRuntimeError> {
        self.snow_stage3_v11_attachment
            .as_deref()
            .ok_or(DirectRuntimeError::DirectKernelGuardFailure {
                phase: "snow_stage3_v11.publication",
                detail: "constitutive attachment is not installed".into(),
            })?
            .committed_publication_day(day_index)
            .map_err(attachment_runtime_error("publication_read"))
    }

    fn promote_snow_stage3_v11_committed_frame(
        &mut self,
        attachment: DirectSnowStage3V11ShadowAttachment,
    ) -> Result<(), DirectRuntimeError> {
        let committed_frame = attachment.committed.real_consumer.hydrology_frame().clone();
        if committed_frame.identity != self.identity {
            self.snow_stage3_v11_attachment = Some(Box::new(attachment));
            return Err(DirectRuntimeError::DirectKernelGuardFailure {
                phase: "snow_stage3_v11.commit",
                detail: "committed real-consumer frame identity".into(),
            });
        }
        let accepted_support_sha256 = attachment
            .committed
            .real_consumer
            .latest_accepted_publication_support_digest_v1();
        let installed_history = attachment
            .committed
            .real_consumer
            .accepted_publication_support_digests_v1();
        // The V11 real consumer, not the day-oriented compatibility frame,
        // owns every accepted hydrology transition. Promote that exact
        // committed frame before publication can observe it, while retaining
        // the constitutive attachment as the sole owner of the next
        // provider/GSI day and Stage-3 state.
        *self = committed_frame;
        self.snow_stage3_v11_attachment = Some(Box::new(attachment));
        if let Some(accepted_support_sha256) = accepted_support_sha256 {
            record_snow_stage3_v11_attachment_adoption_v1(
                accepted_support_sha256,
                installed_history,
            );
        }
        #[cfg(test)]
        if let Some(accepted_support_sha256) = accepted_support_sha256 {
            crate::v9_real_consumer_shadow::record_canonical_covered_accepted_parent_adoption_v1(
                accepted_support_sha256,
            );
        }
        Ok(())
    }
}

fn production_day_support_chronology(
    day_index: usize,
) -> Result<Vec<TimeSupport>, DirectRuntimeError> {
    let day_start = u128::try_from(day_index)
        .map_err(|_| DirectRuntimeError::DirectKernelGuardFailure {
            phase: "snow_stage3_v11.prepare",
            detail: "day index width".into(),
        })?
        .checked_mul(STAGE3_V11_DAY_NS)
        .ok_or(DirectRuntimeError::DirectKernelGuardFailure {
            phase: "snow_stage3_v11.prepare",
            detail: "day support start overflow".into(),
        })?;
    (0..STAGE3_V11_PARENT_SUPPORT_COUNT)
        .map(|support_index| {
            let offset = u128::try_from(support_index)
                .map_err(|_| DirectRuntimeError::DirectKernelGuardFailure {
                    phase: "snow_stage3_v11.prepare",
                    detail: "support index width".into(),
                })?
                .checked_mul(STAGE3_V11_PARENT_SUPPORT_NS)
                .ok_or(DirectRuntimeError::DirectKernelGuardFailure {
                    phase: "snow_stage3_v11.prepare",
                    detail: "support offset overflow".into(),
                })?;
            let start = day_start.checked_add(offset).ok_or(
                DirectRuntimeError::DirectKernelGuardFailure {
                    phase: "snow_stage3_v11.prepare",
                    detail: "support start overflow".into(),
                },
            )?;
            let end = start.checked_add(STAGE3_V11_PARENT_SUPPORT_NS).ok_or(
                DirectRuntimeError::DirectKernelGuardFailure {
                    phase: "snow_stage3_v11.prepare",
                    detail: "support end overflow".into(),
                },
            )?;
            TimeSupport::new(ModelTimeNs::new(start), ModelTimeNs::new(end)).map_err(|error| {
                DirectRuntimeError::DirectKernelGuardFailure {
                    phase: "snow_stage3_v11.prepare",
                    detail: format!("support chronology: {error}"),
                }
            })
        })
        .collect()
}

fn attachment_runtime_error(
    phase: &'static str,
) -> impl Fn(DirectSnowStage3V11AttachmentError) -> DirectRuntimeError {
    move |error| DirectRuntimeError::DirectKernelGuardFailure {
        phase: "snow_stage3_v11",
        detail: format!("{phase}: {error}"),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn production_chronology_builds_exactly_48_contiguous_supports() {
        let supports = production_day_support_chronology(0).expect("day chronology");
        assert_eq!(supports.len(), STAGE3_V11_PARENT_SUPPORT_COUNT);
        assert_eq!(supports[0].start_ns(), ModelTimeNs::new(0));
        assert_eq!(
            supports[STAGE3_V11_PARENT_SUPPORT_COUNT - 1].end_ns(),
            ModelTimeNs::new(STAGE3_V11_DAY_NS)
        );
        assert!(supports.windows(2).all(|pair| {
            pair[0].end_ns() == pair[1].start_ns()
                && pair[0].duration_ns() == STAGE3_V11_PARENT_SUPPORT_NS
        }));
    }

    #[test]
    fn production_chronology_joins_days_without_midnight_gap_or_overlap() {
        let day_zero = production_day_support_chronology(0).expect("day zero");
        let day_one = production_day_support_chronology(1).expect("day one");
        assert_eq!(
            day_zero.last().expect("last support").end_ns(),
            day_one.first().expect("first support").start_ns()
        );
        assert_eq!(
            day_one.first().expect("first support").start_ns(),
            ModelTimeNs::new(STAGE3_V11_DAY_NS)
        );
    }
}
