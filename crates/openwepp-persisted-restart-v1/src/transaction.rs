use openwepp_hillslope_orchestrator::{
    runtime_inputs::{
        HillslopeClimateRuntimeRequest, PreparedSnowFreeGsiDayV1,
        restart_authority_project_gsi_state,
    },
    v9_real_consumer_shadow::{
        DirectV10RealConsumerError, DirectV10RealConsumerShadow, DirectV10ShadowDayInput,
    },
};
use serde::{Deserialize, Serialize};
use thiserror::Error;

use crate::{
    AcceptedIntervalCount, CompleteCommittedOwnerStateV1, DirectGsiDailyReceiptRestartV1,
    DirectGsiOwnerStateRestartV1, DirectHydrologyExactEnthalpyProjectionInputsV2,
    DirectHydrologyExactEnthalpyRestartV2, DirectV10CheckpointPhaseV1,
    DirectV10ContinuationTemplateRestartV1, DirectV10RealConsumerCheckpointV1,
    DirectV10RestartHost, ExpectedDirectHydrologyExactEnthalpyRestartContextV2,
    ExpectedRestartStaticContext, InProgressIntervalIndex, RestartAdmissionFailureV1, Sha256Hex,
    SnowFreeHalfHourDayReceiptRestartV1, SnowFreeHalfHourProviderCursorRestartV1, WireDayIndex,
    admit_checkpoint_v1, canonical_sha256, checkpoint_identities_v1, from_canonical_bytes,
    project_complete_owner_state_v1, project_exact_hydrology_state_v2,
    project_scientific_owner_state_v1, to_canonical_bytes,
};

const DIRECT_V10_EXACT_ENTHALPY_CHECKPOINT_V2_SCHEMA: &str =
    "OPENWEPP_DIRECT_V10_EXACT_ENTHALPY_CHECKPOINT_V2";

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(tag = "kind", rename_all = "snake_case", deny_unknown_fields)]
pub enum DirectV10ExactEnthalpyCheckpointPhaseV2 {
    BetweenDays {
        committed_hydrology: DirectHydrologyExactEnthalpyRestartV2,
    },
    InProgressDay {
        committed_day_beginning_hydrology: DirectHydrologyExactEnthalpyRestartV2,
        staged_hydrology: DirectHydrologyExactEnthalpyRestartV2,
    },
}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct DirectV10ExactEnthalpyCheckpointV2 {
    pub schema: String,
    pub version: u16,
    pub parent_v1_checkpoint_bytes: Vec<u8>,
    pub parent_v1_checkpoint_sha256: Sha256Hex,
    pub exact_phase: DirectV10ExactEnthalpyCheckpointPhaseV2,
    pub payload_sha256: Sha256Hex,
}

#[derive(Serialize)]
struct DirectV10ExactEnthalpyCheckpointDigestBodyV2<'a> {
    schema: &'a str,
    version: u16,
    parent_v1_checkpoint_bytes: &'a [u8],
    parent_v1_checkpoint_sha256: &'a Sha256Hex,
    exact_phase: &'a DirectV10ExactEnthalpyCheckpointPhaseV2,
}

#[derive(Debug, Error)]
pub enum RestartTransactionError {
    #[error(transparent)]
    Consumer(#[from] DirectV10RealConsumerError),
    #[error(transparent)]
    Admission(#[from] RestartAdmissionFailureV1),
    #[error("projection: {0}")]
    Projection(&'static str),
    #[error("invalid transaction phase: {0}")]
    Phase(&'static str),
}

/// Isolated prepared-day transaction over the actual default-off V10 consumer.
pub struct DirectV10PreparedDayTransactionV1 {
    committed: CompleteCommittedOwnerStateV1,
    staged: DirectV10RealConsumerShadow,
    prepared: PreparedSnowFreeGsiDayV1,
    template: DirectV10ShadowDayInput,
    run: Sha256Hex,
    topology: Sha256Hex,
    phase_plan: Sha256Hex,
    day_inputs: Vec<Sha256Hex>,
    day: u64,
    next: u8,
    accepted_at_beginning: u64,
}

impl DirectV10PreparedDayTransactionV1 {
    pub fn prepare(
        shadow: &DirectV10RealConsumerShadow,
        request: &HillslopeClimateRuntimeRequest,
        template: DirectV10ShadowDayInput,
        phase_plan: Sha256Hex,
        day_inputs: Vec<Sha256Hex>,
    ) -> Result<Self, RestartTransactionError> {
        Self::prepare_inner(shadow, request, template, phase_plan, day_inputs, false)
    }

    fn prepare_inner(
        shadow: &DirectV10RealConsumerShadow,
        request: &HillslopeClimateRuntimeRequest,
        template: DirectV10ShadowDayInput,
        phase_plan: Sha256Hex,
        day_inputs: Vec<Sha256Hex>,
        allow_nested_v4_parent: bool,
    ) -> Result<Self, RestartTransactionError> {
        let day = shadow.restart_authority_next_day_index();
        if template.day_index != day {
            return Err(RestartTransactionError::Phase("template day"));
        }
        let prepared = request
            .prepare_snow_free_gsi_day_from_repository(
                day,
                shadow.provider_static_configuration(),
                shadow.gsi_owner_configuration(),
                shadow.gsi_state(),
                shadow.provider_cursor(),
            )
            .map_err(DirectV10RealConsumerError::from)?;
        let committed = if allow_nested_v4_parent {
            crate::projection::project_complete_owner_state_v1_for_exact_parent(
                shadow,
                &phase_plan,
                &day_inputs,
                day,
            )
        } else {
            project_complete_owner_state_v1(shadow, &phase_plan, &day_inputs, day)
        }
        .map_err(RestartTransactionError::Projection)?;
        let (run, topology) =
            checkpoint_identities_v1(&committed, shadow.root_zone_hydraulic_configuration())
                .map_err(RestartTransactionError::Projection)?;
        let mut ending_gsi = shadow.gsi_state().clone();
        let mut ending_cursor = shadow.provider_cursor().clone();
        prepared
            .clone()
            .commit(&mut ending_gsi, &mut ending_cursor)
            .map_err(DirectV10RealConsumerError::from)?;
        let mut staged = shadow.clone();
        staged.restart_authority_install_staged_daily_owners(
            ending_gsi,
            ending_cursor,
            day.checked_add(1)
                .ok_or(RestartTransactionError::Phase("day overflow"))?,
        )?;
        Ok(Self {
            committed,
            staged,
            prepared,
            template,
            run,
            topology,
            phase_plan,
            day_inputs,
            day: u64::try_from(day).map_err(|_| RestartTransactionError::Phase("day width"))?,
            next: 0,
            accepted_at_beginning: shadow.restart_authority_accepted_interval_count(),
        })
    }

    pub fn restore(
        bytes: &[u8],
        context: &ExpectedRestartStaticContext<'_>,
    ) -> Result<Self, RestartTransactionError> {
        let restored = admit_checkpoint_v1(bytes, context)?;
        let host = DirectV10RestartHost::from_isolated(restored, context)
            .map_err(|_| RestartTransactionError::Phase("host restoration"))?;
        let DirectV10RestartHost::InProgressDay {
            shadow,
            day_index,
            next_interval_index,
            accepted_interval_count,
            committed_day_beginning,
            continuation_template,
            prepared,
            ..
        } = host
        else {
            return Err(RestartTransactionError::Phase(
                "restore requires in-progress bytes",
            ));
        };
        Ok(Self {
            committed: *committed_day_beginning,
            staged: shadow,
            prepared,
            template: continuation_template,
            run: context.run_identity_sha256.clone(),
            topology: context.topology_sha256.clone(),
            phase_plan: context.phase_plan_sha256.clone(),
            day_inputs: context.day_input_digests.to_vec(),
            day: day_index,
            next: next_interval_index,
            accepted_at_beginning: accepted_interval_count
                .checked_sub(u64::from(next_interval_index))
                .ok_or(RestartTransactionError::Phase("accepted count"))?,
        })
    }

    fn restore_exact_parent(
        bytes: &[u8],
        context: &ExpectedRestartStaticContext<'_>,
    ) -> Result<Self, RestartTransactionError> {
        let original: DirectV10RealConsumerCheckpointV1 = from_canonical_bytes(bytes)
            .map_err(|_| RestartTransactionError::Phase("nested parent V1 checkpoint"))?;
        if original.compute_digest()? != original.payload_sha256 {
            return Err(RestartAdmissionFailureV1::PayloadDigest.into());
        }
        let mut admission = original.clone();
        let DirectV10CheckpointPhaseV1::InProgressDay {
            committed_day_beginning,
            staged_scientific,
            ..
        } = &mut admission.phase
        else {
            return Err(RestartTransactionError::Phase(
                "restore requires in-progress exact parent bytes",
            ));
        };
        committed_day_beginning
            .scientific
            .direct_hydrology
            .snow_stage3_v11_attachment = None;
        staged_scientific
            .direct_hydrology
            .snow_stage3_v11_attachment = None;
        admission.seal()?;
        let admission_bytes = to_canonical_bytes(&admission)
            .map_err(|_| RestartTransactionError::Projection("exact parent admission bytes"))?;
        let mut restored = Self::restore(&admission_bytes, context)?;
        let DirectV10CheckpointPhaseV1::InProgressDay {
            committed_day_beginning,
            ..
        } = original.phase
        else {
            return Err(RestartTransactionError::Phase(
                "restore requires in-progress exact parent bytes",
            ));
        };
        restored.committed = committed_day_beginning;
        Ok(restored)
    }

    pub fn advance_one_interval(&mut self) -> Result<(), RestartTransactionError> {
        if self.next >= 48 {
            return Err(RestartTransactionError::Phase("day complete"));
        }
        let start = usize::from(self.next);
        self.staged.restart_authority_advance_staged_intervals(
            &self.prepared,
            self.template.clone(),
            start,
            start + 1,
        )?;
        self.next += 1;
        Ok(())
    }

    pub fn checkpoint(&self) -> Result<Vec<u8>, RestartTransactionError> {
        self.checkpoint_inner(false)
    }

    fn checkpoint_inner(
        &self,
        allow_nested_v4_parent: bool,
    ) -> Result<Vec<u8>, RestartTransactionError> {
        let accepted = self
            .accepted_at_beginning
            .checked_add(u64::from(self.next))
            .ok_or(RestartTransactionError::Phase("accepted count overflow"))?;
        let phase = if self.next == 0 {
            DirectV10CheckpointPhaseV1::BetweenDays {
                next_day_index: WireDayIndex(self.day),
                accepted_interval_count: AcceptedIntervalCount::try_new(accepted)
                    .map_err(|_| RestartTransactionError::Phase("accepted count"))?,
                committed: self.committed.clone(),
            }
        } else if self.next < 48 {
            let gsi = restart_authority_project_gsi_state(self.staged.gsi_state())
                .map_err(DirectV10RealConsumerError::from)?;
            DirectV10CheckpointPhaseV1::InProgressDay {
                day_index: WireDayIndex(self.day),
                next_interval_index: InProgressIntervalIndex::try_new(self.next)
                    .map_err(|_| RestartTransactionError::Phase("interval"))?,
                accepted_interval_count: AcceptedIntervalCount::try_new(accepted)
                    .map_err(|_| RestartTransactionError::Phase("accepted count"))?,
                committed_day_beginning: self.committed.clone(),
                staged_scientific: if allow_nested_v4_parent {
                    crate::projection::project_scientific_owner_state_v1_for_exact_parent(
                        &self.staged,
                        &self.phase_plan,
                        &self.day_inputs,
                    )
                } else {
                    project_scientific_owner_state_v1(
                        &self.staged,
                        &self.phase_plan,
                        &self.day_inputs,
                    )
                }
                .map_err(RestartTransactionError::Projection)?,
                accepted_gsi_daily_receipt: DirectGsiDailyReceiptRestartV1::project(
                    self.prepared.gsi_receipt(),
                )
                .map_err(|_| RestartTransactionError::Projection("GSI receipt"))?,
                staged_gsi_ending_state: DirectGsiOwnerStateRestartV1::project(&gsi)
                    .map_err(|_| RestartTransactionError::Projection("GSI ending state"))?,
                ending_provider_cursor: SnowFreeHalfHourProviderCursorRestartV1::project(
                    self.staged.provider_cursor(),
                    self.staged.provider_static_configuration(),
                    usize::try_from(self.day + 1)
                        .map_err(|_| RestartTransactionError::Phase("day width"))?,
                )
                .map_err(|_| RestartTransactionError::Projection("provider cursor"))?,
                validated_forcing_day_receipts: self
                    .prepared
                    .forcing_receipts()
                    .receipts()
                    .iter()
                    .map(SnowFreeHalfHourDayReceiptRestartV1::project)
                    .collect::<Result<_, _>>()
                    .map_err(|_| RestartTransactionError::Projection("forcing receipts"))?,
                continuation_template: DirectV10ContinuationTemplateRestartV1::project(
                    &self.template,
                ),
            }
        } else {
            DirectV10CheckpointPhaseV1::BetweenDays {
                next_day_index: WireDayIndex(self.day + 1),
                accepted_interval_count: AcceptedIntervalCount::try_new(accepted)
                    .map_err(|_| RestartTransactionError::Phase("accepted count"))?,
                committed: if allow_nested_v4_parent {
                    crate::projection::project_complete_owner_state_v1_for_exact_parent(
                        &self.staged,
                        &self.phase_plan,
                        &self.day_inputs,
                        usize::try_from(self.day + 1)
                            .map_err(|_| RestartTransactionError::Phase("day width"))?,
                    )
                } else {
                    project_complete_owner_state_v1(
                        &self.staged,
                        &self.phase_plan,
                        &self.day_inputs,
                        usize::try_from(self.day + 1)
                            .map_err(|_| RestartTransactionError::Phase("day width"))?,
                    )
                }
                .map_err(RestartTransactionError::Projection)?,
            }
        };
        let mut checkpoint = DirectV10RealConsumerCheckpointV1 {
            schema: "OPENWEPP_DIRECT_V10_REAL_CONSUMER_CHECKPOINT_V1".into(),
            version: 1,
            run_identity_sha256: self.run.clone(),
            topology_sha256: self.topology.clone(),
            phase,
            payload_sha256: Sha256Hex::try_new("0".repeat(64))
                .map_err(|_| RestartTransactionError::Projection("digest"))?,
        };
        checkpoint.seal()?;
        to_canonical_bytes(&checkpoint)
            .map_err(|_| RestartTransactionError::Projection("canonical checkpoint"))
    }

    #[must_use]
    pub fn abort(self) -> CompleteCommittedOwnerStateV1 {
        self.committed
    }

    pub fn finish(self) -> Result<DirectV10RestartHost, RestartTransactionError> {
        if self.next != 48 {
            return Err(RestartTransactionError::Phase("finish before 48"));
        }
        Ok(DirectV10RestartHost::BetweenDays {
            shadow: self.staged,
            accepted_interval_count: self
                .accepted_at_beginning
                .checked_add(48)
                .ok_or(RestartTransactionError::Phase("accepted count overflow"))?,
        })
    }
}

impl DirectV10ExactEnthalpyCheckpointV2 {
    fn compute_digest(&self) -> Result<Sha256Hex, RestartTransactionError> {
        Sha256Hex::try_new(
            canonical_sha256(&DirectV10ExactEnthalpyCheckpointDigestBodyV2 {
                schema: &self.schema,
                version: self.version,
                parent_v1_checkpoint_bytes: &self.parent_v1_checkpoint_bytes,
                parent_v1_checkpoint_sha256: &self.parent_v1_checkpoint_sha256,
                exact_phase: &self.exact_phase,
            })
            .map_err(|_| RestartTransactionError::Projection("exact checkpoint digest"))?,
        )
        .map_err(|_| RestartTransactionError::Projection("exact checkpoint digest"))
    }

    fn validate(&self) -> Result<(), RestartTransactionError> {
        if self.schema != DIRECT_V10_EXACT_ENTHALPY_CHECKPOINT_V2_SCHEMA
            || self.version != 2
            || self.parent_v1_checkpoint_sha256
                != Sha256Hex::try_new(canonical_sha256(&self.parent_v1_checkpoint_bytes).map_err(
                    |_| RestartTransactionError::Projection("parent V1 checkpoint digest"),
                )?)
                .map_err(|_| RestartTransactionError::Projection("parent V1 checkpoint digest"))?
            || self.payload_sha256 != self.compute_digest()?
        {
            return Err(RestartTransactionError::Phase(
                "exact checkpoint schema or digest",
            ));
        }
        let parent: DirectV10RealConsumerCheckpointV1 =
            from_canonical_bytes(&self.parent_v1_checkpoint_bytes)
                .map_err(|_| RestartTransactionError::Phase("nested parent V1 checkpoint"))?;
        if to_canonical_bytes(&parent)
            .map_err(|_| RestartTransactionError::Phase("nested parent V1 checkpoint"))?
            != self.parent_v1_checkpoint_bytes
        {
            return Err(RestartTransactionError::Phase(
                "noncanonical nested parent V1 checkpoint",
            ));
        }
        match (&parent.phase, &self.exact_phase) {
            (
                DirectV10CheckpointPhaseV1::BetweenDays { committed, .. },
                DirectV10ExactEnthalpyCheckpointPhaseV2::BetweenDays {
                    committed_hydrology,
                },
            ) => validate_exact_parent_hydrology(
                &committed.scientific.direct_hydrology,
                committed_hydrology,
            )?,
            (
                DirectV10CheckpointPhaseV1::InProgressDay {
                    committed_day_beginning,
                    staged_scientific,
                    ..
                },
                DirectV10ExactEnthalpyCheckpointPhaseV2::InProgressDay {
                    committed_day_beginning_hydrology,
                    staged_hydrology,
                },
            ) => {
                validate_exact_parent_hydrology(
                    &committed_day_beginning.scientific.direct_hydrology,
                    committed_day_beginning_hydrology,
                )?;
                validate_exact_parent_hydrology(
                    &staged_scientific.direct_hydrology,
                    staged_hydrology,
                )?;
            }
            _ => {
                return Err(RestartTransactionError::Phase(
                    "exact checkpoint phase join",
                ));
            }
        }
        Ok(())
    }

    fn seal(&mut self) -> Result<(), RestartTransactionError> {
        self.payload_sha256 = self.compute_digest()?;
        self.validate()
    }

    pub fn to_canonical_bytes(&self) -> Result<Vec<u8>, RestartTransactionError> {
        self.validate()?;
        to_canonical_bytes(self)
            .map_err(|_| RestartTransactionError::Projection("exact checkpoint bytes"))
    }

    pub fn from_canonical_bytes(bytes: &[u8]) -> Result<Self, RestartTransactionError> {
        let value: Self = from_canonical_bytes(bytes)
            .map_err(|_| RestartTransactionError::Phase("exact checkpoint bytes"))?;
        value.validate()?;
        if value.to_canonical_bytes()? != bytes {
            return Err(RestartTransactionError::Phase(
                "noncanonical exact checkpoint bytes",
            ));
        }
        Ok(value)
    }
}

fn validate_exact_parent_hydrology(
    parent: &crate::DirectHydrologyRestartV1,
    exact: &DirectHydrologyExactEnthalpyRestartV2,
) -> Result<(), RestartTransactionError> {
    exact
        .to_canonical_bytes()
        .map_err(|_| RestartTransactionError::Phase("exact hydrology envelope"))?;
    if to_canonical_bytes(parent)
        .map_err(|_| RestartTransactionError::Phase("parent hydrology bytes"))?
        != exact.parent_v1_bytes
    {
        return Err(RestartTransactionError::Phase(
            "exact hydrology nested V1 join",
        ));
    }
    Ok(())
}

/// Additive prepared-day transaction used whenever the retained Stage-3 graph
/// owns V4 exact enthalpy. Its parent V1 bytes remain byte-for-byte unchanged;
/// every checkpoint carries and reloads the authoritative hydrology V2
/// supplement before the staged shadow can be used.
pub struct DirectV10PreparedDayExactEnthalpyTransactionV2 {
    parent: DirectV10PreparedDayTransactionV1,
    committed_exact_hydrology: DirectHydrologyExactEnthalpyRestartV2,
}

impl DirectV10PreparedDayExactEnthalpyTransactionV2 {
    #[allow(clippy::too_many_arguments)]
    pub fn prepare(
        shadow: &DirectV10RealConsumerShadow,
        request: &HillslopeClimateRuntimeRequest,
        template: DirectV10ShadowDayInput,
        phase_plan: Sha256Hex,
        day_inputs: Vec<Sha256Hex>,
        exact_inputs: DirectHydrologyExactEnthalpyProjectionInputsV2<'_>,
    ) -> Result<Self, RestartTransactionError> {
        let committed_exact_hydrology =
            project_exact_hydrology_state_v2(shadow, &phase_plan, &day_inputs, exact_inputs)
                .map_err(RestartTransactionError::Projection)?;
        let parent = DirectV10PreparedDayTransactionV1::prepare_inner(
            shadow, request, template, phase_plan, day_inputs, true,
        )?;
        validate_exact_parent_hydrology(
            &parent.committed.scientific.direct_hydrology,
            &committed_exact_hydrology,
        )?;
        Ok(Self {
            parent,
            committed_exact_hydrology,
        })
    }

    pub fn advance_one_interval(&mut self) -> Result<(), RestartTransactionError> {
        self.parent.advance_one_interval()
    }

    pub fn checkpoint(
        &self,
        staged_exact_inputs: DirectHydrologyExactEnthalpyProjectionInputsV2<'_>,
    ) -> Result<Vec<u8>, RestartTransactionError> {
        let parent_v1_checkpoint_bytes = self.parent.checkpoint_inner(true)?;
        let staged = (self.parent.next != 0)
            .then(|| {
                project_exact_hydrology_state_v2(
                    &self.parent.staged,
                    &self.parent.phase_plan,
                    &self.parent.day_inputs,
                    staged_exact_inputs,
                )
                .map_err(RestartTransactionError::Projection)
            })
            .transpose()?;
        let exact_phase = match self.parent.next {
            0 => DirectV10ExactEnthalpyCheckpointPhaseV2::BetweenDays {
                committed_hydrology: self.committed_exact_hydrology.clone(),
            },
            1..=47 => DirectV10ExactEnthalpyCheckpointPhaseV2::InProgressDay {
                committed_day_beginning_hydrology: self.committed_exact_hydrology.clone(),
                staged_hydrology: staged.ok_or(RestartTransactionError::Phase(
                    "missing staged exact hydrology",
                ))?,
            },
            48 => DirectV10ExactEnthalpyCheckpointPhaseV2::BetweenDays {
                committed_hydrology: staged.ok_or(RestartTransactionError::Phase(
                    "missing ending exact hydrology",
                ))?,
            },
            _ => return Err(RestartTransactionError::Phase("interval posture")),
        };
        let parent_v1_checkpoint_sha256 = Sha256Hex::try_new(
            canonical_sha256(&parent_v1_checkpoint_bytes)
                .map_err(|_| RestartTransactionError::Projection("parent checkpoint digest"))?,
        )
        .map_err(|_| RestartTransactionError::Projection("parent checkpoint digest"))?;
        let mut value = DirectV10ExactEnthalpyCheckpointV2 {
            schema: DIRECT_V10_EXACT_ENTHALPY_CHECKPOINT_V2_SCHEMA.to_owned(),
            version: 2,
            parent_v1_checkpoint_bytes,
            parent_v1_checkpoint_sha256,
            exact_phase,
            payload_sha256: Sha256Hex::try_new("0".repeat(64))
                .map_err(|_| RestartTransactionError::Projection("exact checkpoint digest"))?,
        };
        value.seal()?;
        value.to_canonical_bytes()
    }

    pub fn restore(
        bytes: &[u8],
        parent_context: &ExpectedRestartStaticContext<'_>,
        committed_exact_context: &ExpectedDirectHydrologyExactEnthalpyRestartContextV2<'_>,
        staged_exact_context: &ExpectedDirectHydrologyExactEnthalpyRestartContextV2<'_>,
    ) -> Result<Self, RestartTransactionError> {
        let checkpoint = DirectV10ExactEnthalpyCheckpointV2::from_canonical_bytes(bytes)?;
        let DirectV10ExactEnthalpyCheckpointPhaseV2::InProgressDay {
            committed_day_beginning_hydrology,
            staged_hydrology,
        } = checkpoint.exact_phase
        else {
            return Err(RestartTransactionError::Phase(
                "restore requires in-progress exact bytes",
            ));
        };
        committed_day_beginning_hydrology
            .restore(committed_exact_context)
            .map_err(|_| RestartTransactionError::Phase("committed exact hydrology restore"))?;
        let staged_frame = staged_hydrology
            .restore(staged_exact_context)
            .map_err(|_| RestartTransactionError::Phase("staged exact hydrology restore"))?;
        let mut parent = DirectV10PreparedDayTransactionV1::restore_exact_parent(
            &checkpoint.parent_v1_checkpoint_bytes,
            parent_context,
        )?;
        parent
            .staged
            .restart_authority_install_exact_hydrology_frame_v2(staged_frame)?;
        Ok(Self {
            parent,
            committed_exact_hydrology: committed_day_beginning_hydrology,
        })
    }

    #[must_use]
    pub fn abort(
        self,
    ) -> (
        CompleteCommittedOwnerStateV1,
        DirectHydrologyExactEnthalpyRestartV2,
    ) {
        (self.parent.abort(), self.committed_exact_hydrology)
    }

    pub fn finish(self) -> Result<DirectV10RestartHost, RestartTransactionError> {
        self.parent.finish()
    }
}

#[cfg(all(test, feature = "fixtures"))]
mod tests {
    use std::collections::BTreeMap;

    use openwepp_coupled_time::{Digest32, digest_bytes};
    use openwepp_hillslope_orchestrator::{
        DirectOfeWb14Parameters, Wb11HydrologyKernel,
        land_surface_energy_shadow::{accepted_negative_zero_v4_evidence_v1, endpoint_fixture},
        snow_stage3_v11_attachment::{
            DirectSnowStage3V11ProductionConfigurationV1, DirectSnowStage3V11ShadowAttachment,
            Stage3CommittedDayArchiveManifestV1,
        },
        v9_real_consumer_shadow::{FrozenLitterV3Resident, FrozenLitterV4Resident},
    };
    use openwepp_land_surface_energy::{
        SoilThermalOwnerCheckpointV2, SoilThermalOwnerEnvelopeV2, SoilThermalOwnerRestartV2,
    };

    use crate::{
        DirectFrozenLitterCheckpointV3, DirectFrozenLitterExactEnthalpyCheckpointV4,
        DirectHydrologyExactEnthalpyProjectionInputsV2,
        ExpectedDirectHydrologyExactEnthalpyRestartContextV2,
        ExpectedDirectHydrologyRestartContext, ExpectedFrozenLitterCheckpointContextV3,
        ExpectedFrozenLitterExactEnthalpyContextV4, ExpectedRestartStaticContext,
        ExpectedSnowStage3V11ExactEnthalpyRestartContextV4,
        ExpectedSnowStage3V11ExactResidentContextsV4, ExpectedSnowStage3V11RestartContext,
        ExpectedStage3CommittedDayArchiveV3, FrozenLitterExpectedScientificContextV3,
        FrozenLitterProjectionSealAuthorityV3, FrozenLitterPublicationAuthorityV3,
        NativeFrozenLitterProjectionAuthorityV3, Sha256Hex, SnowStage3V11ExactResidentSetV4,
        SnowStage3V11ExactResidentSupplementV4, SoilThermalNativeBundleV2,
        SoilThermalNativeSealAuthorityV2, SoilThermalOwnerStateRestartV2,
        SoilThermalStateRestartV1, Stage3CommittedDayArchiveReaderV3, checkpoint_identities_v1,
        project_frozen_litter_scientific_owner_v3, restart_authority_prepared_day_fixture,
        to_canonical_bytes,
    };

    use super::{
        DirectV10ExactEnthalpyCheckpointV2, DirectV10PreparedDayExactEnthalpyTransactionV2,
        DirectV10PreparedDayTransactionV1,
    };

    struct EmptyArchiveReader;

    impl Stage3CommittedDayArchiveReaderV3 for EmptyArchiveReader {
        fn read_canonical_uncompressed(&self, _content_sha256: Digest32) -> Option<Vec<u8>> {
            None
        }
    }

    struct CarriedSoilAuthority<'a> {
        owner: &'a SoilThermalOwnerEnvelopeV2,
        restart: &'a SoilThermalOwnerRestartV2,
        checkpoint: &'a SoilThermalOwnerCheckpointV2,
    }

    impl SoilThermalNativeSealAuthorityV2 for CarriedSoilAuthority<'_> {
        fn validate_restart_seal(
            &self,
            envelope: &SoilThermalOwnerEnvelopeV2,
            seal: &SoilThermalOwnerRestartV2,
        ) -> Result<(), &'static str> {
            (envelope == self.owner && seal == self.restart)
                .then_some(())
                .ok_or("carried exact restart join")
        }

        fn validate_checkpoint_seal(
            &self,
            envelope: &SoilThermalOwnerEnvelopeV2,
            seal: &SoilThermalOwnerCheckpointV2,
        ) -> Result<(), &'static str> {
            (envelope == self.owner && seal == self.checkpoint)
                .then_some(())
                .ok_or("carried exact checkpoint join")
        }
    }

    fn wire_digest(fill: char) -> Sha256Hex {
        Sha256Hex::try_new(fill.to_string().repeat(64)).expect("wire digest")
    }

    #[test]
    fn production_transaction_originates_restores_finishes_and_aborts() {
        let mut fixture = restart_authority_prepared_day_fixture();
        for inputs in &mut fixture.owners.day_inputs {
            if let Some(first) = inputs.first().cloned() {
                inputs.push(first);
            }
        }
        fixture.owners.day_input_digests = fixture
            .owners
            .day_inputs
            .iter()
            .map(|inputs| {
                crate::hydrology_restart::canonical_operand_sha256(
                    "DirectDayConstructorInputsV1",
                    inputs,
                )
                .unwrap()
            })
            .collect();
        fixture
            .owners
            .committed
            .scientific
            .direct_hydrology
            .day_count = 2;
        for (lane, digest) in fixture
            .owners
            .committed
            .scientific
            .direct_hydrology
            .lanes
            .iter_mut()
            .zip(&fixture.owners.day_input_digests)
        {
            lane.day_inputs_sha256 = digest.clone();
        }
        let request = fixture.request.clone();
        let template = fixture.template.clone();
        let committed = fixture.owners.committed.clone();
        let phase_plan_sha256 = fixture.owners.phase_plan_sha256.clone();
        let day_input_digests = fixture.owners.day_input_digests.clone();
        let day_inputs = fixture.owners.day_inputs.clone();
        let source = &fixture.owners.runtime.shadow;
        let vegetation_configuration = source.restart_authority_vegetation_configuration().clone();
        let vegetation_owner_id = source.restart_authority_vegetation_owner_id().clone();
        let soil_thermal_owner_id = source
            .restart_authority_soil_thermal()
            .expect("V1 transaction fixture soil resident")
            .owner_id
            .clone();
        let soil_thermal_configuration_sha256 = source
            .restart_authority_soil_thermal()
            .expect("V1 transaction fixture soil resident")
            .configuration_sha256
            .clone();
        let lse_configuration = source.restart_authority_lse_configuration().clone();
        let surface_liquid_configuration = source.restart_authority_surface_configuration().clone();
        let gsi_configuration = source.gsi_owner_configuration().clone();
        let forcing_static_configuration = source.provider_static_configuration().clone();
        let root_zone_hydraulic_configuration = source.root_zone_hydraulic_configuration().clone();
        let phase_plan = source
            .restart_authority_hydrology_frame()
            .phase_plan
            .clone();
        let (run, topology) =
            crate::checkpoint_identities_v1(&committed, &root_zone_hydraulic_configuration)
                .unwrap();
        drop(fixture);
        let context = ExpectedRestartStaticContext {
            run_identity_sha256: &run,
            topology_sha256: &topology,
            vegetation_configuration: &vegetation_configuration,
            vegetation_owner_id: &vegetation_owner_id,
            soil_thermal_owner_id: &soil_thermal_owner_id,
            soil_thermal_configuration_sha256: &soil_thermal_configuration_sha256,
            lse_configuration: &lse_configuration,
            surface_liquid_configuration: &surface_liquid_configuration,
            gsi_configuration: &gsi_configuration,
            forcing_static_configuration: &forcing_static_configuration,
            root_zone_hydraulic_configuration: &root_zone_hydraulic_configuration,
            phase_plan: &phase_plan,
            phase_plan_sha256: &phase_plan_sha256,
            day_inputs: &day_inputs,
            day_input_digests: &day_input_digests,
        };
        let mut seed = crate::DirectV10RealConsumerCheckpointV1 {
            schema: "OPENWEPP_DIRECT_V10_REAL_CONSUMER_CHECKPOINT_V1".into(),
            version: 1,
            run_identity_sha256: run.clone(),
            topology_sha256: topology.clone(),
            phase: crate::DirectV10CheckpointPhaseV1::BetweenDays {
                next_day_index: crate::WireDayIndex(0),
                accepted_interval_count: crate::AcceptedIntervalCount::try_new(0).unwrap(),
                committed: committed.clone(),
            },
            payload_sha256: crate::Sha256Hex::try_new("0".repeat(64)).unwrap(),
        };
        seed.seal().unwrap();
        let restored =
            crate::admit_checkpoint_v1(&to_canonical_bytes(&seed).unwrap(), &context).unwrap();
        let valid_host = crate::DirectV10RestartHost::from_isolated(restored, &context).unwrap();
        for stop in [0, 1, 15, 24, 47, 48] {
            let mut candidate = DirectV10PreparedDayTransactionV1::prepare(
                valid_host.shadow(),
                &request,
                template.clone(),
                phase_plan_sha256.clone(),
                day_input_digests.clone(),
            )
            .unwrap();
            for _ in 0..stop {
                candidate.advance_one_interval().unwrap();
            }
            if stop == 48 {
                assert!(candidate.advance_one_interval().is_err());
            }
            assert_eq!(
                to_canonical_bytes(&candidate.abort()).unwrap(),
                to_canonical_bytes(&committed).unwrap()
            );
        }
        let mut continuous = DirectV10PreparedDayTransactionV1::prepare(
            valid_host.shadow(),
            &request,
            template.clone(),
            phase_plan_sha256.clone(),
            day_input_digests.clone(),
        )
        .unwrap();
        for _ in 0..48 {
            continuous.advance_one_interval().unwrap();
        }
        let continuous_ending = continuous.checkpoint().unwrap();
        let mut transaction = DirectV10PreparedDayTransactionV1::prepare(
            valid_host.shadow(),
            &request,
            template.clone(),
            phase_plan_sha256.clone(),
            day_input_digests.clone(),
        )
        .unwrap();
        let beginning = transaction.checkpoint().unwrap();
        for _ in 0..24 {
            transaction.advance_one_interval().unwrap();
        }
        let interval_24 = transaction.checkpoint().unwrap();
        let abort = transaction.abort();
        assert_eq!(
            to_canonical_bytes(&abort).unwrap(),
            to_canonical_bytes(&committed).unwrap()
        );
        assert!(matches!(
            crate::from_canonical_bytes::<crate::DirectV10RealConsumerCheckpointV1>(&beginning)
                .unwrap()
                .phase,
            crate::DirectV10CheckpointPhaseV1::BetweenDays { .. }
        ));
        drop(continuous);
        drop(valid_host);
        let mut resumed =
            DirectV10PreparedDayTransactionV1::restore(&interval_24, &context).unwrap();
        let restored_abort = DirectV10PreparedDayTransactionV1::restore(&interval_24, &context)
            .unwrap()
            .abort();
        assert_eq!(
            to_canonical_bytes(&restored_abort).unwrap(),
            to_canonical_bytes(&committed).unwrap()
        );
        for _ in 24..48 {
            resumed.advance_one_interval().unwrap();
        }
        let ending = resumed.checkpoint().unwrap();
        assert_eq!(ending, continuous_ending);
        assert!(matches!(
            crate::from_canonical_bytes::<crate::DirectV10RealConsumerCheckpointV1>(&ending)
                .unwrap()
                .phase,
            crate::DirectV10CheckpointPhaseV1::BetweenDays {
                next_day_index: crate::WireDayIndex(1),
                ..
            }
        ));
        let host = resumed.finish().unwrap();
        assert_eq!(host.shadow().restart_authority_next_day_index(), 1);
        let mut day_one_template = template.clone();
        day_one_template.day_index = 1;
        for (index, interval) in day_one_template.intervals.iter_mut().enumerate() {
            interval.lse_forcing.transaction_id =
                openwepp_kernel_contract::TransactionId(89 + index as u128);
            interval.lse_forcing.forcing_sha256 = interval.lse_forcing.canonical_sha256().unwrap();
        }
        let mut day_one = DirectV10PreparedDayTransactionV1::prepare(
            host.shadow(),
            &request,
            day_one_template,
            phase_plan_sha256.clone(),
            day_input_digests.clone(),
        )
        .unwrap();
        day_one.advance_one_interval().unwrap();
        let day_one_checkpoint = crate::from_canonical_bytes::<
            crate::DirectV10RealConsumerCheckpointV1,
        >(&day_one.checkpoint().unwrap())
        .unwrap();
        assert!(matches!(
            day_one_checkpoint.phase,
            crate::DirectV10CheckpointPhaseV1::InProgressDay {
                day_index: crate::WireDayIndex(1),
                accepted_interval_count,
                ..
            } if accepted_interval_count.get() == 49
        ));
    }

    #[test]
    fn production_exact_enthalpy_transaction_checkpoints_and_reloads_signed_zero() {
        let evidence = accepted_negative_zero_v4_evidence_v1();
        assert!(
            evidence
                .beginning_exact_surface_owner
                .records
                .iter()
                .any(|record| record.enthalpy_hi_j_m2_tile.to_bits() == (-0.0_f64).to_bits())
        );

        let parent_soil = SoilThermalStateRestartV1::project(&endpoint_fixture().thermal)
            .expect("exact fixture parent soil");
        let soil_checkpoint = SoilThermalOwnerCheckpointV2 {
            owner_tag: evidence.soil_thermal_owner.owner_tag.clone(),
            schema_sha256: evidence.soil_thermal_owner.schema_sha256.clone(),
            exact_carry_definition_sha256: evidence
                .soil_thermal_owner
                .exact_carry_definition_sha256
                .clone(),
            parent_v1_state_sha256: evidence.soil_thermal_owner.parent_v1_state_sha256.clone(),
            owner_state_sha256: evidence.soil_thermal_owner.state.state_sha256.clone(),
            last_accepted_transaction_id: evidence
                .soil_thermal_owner
                .state
                .last_accepted_transaction_id,
            receipt_chain_sha256: evidence.soil_thermal_owner.receipt_chain_sha256.clone(),
            checkpoint_sha256: evidence.soil_thermal_restart.restart_sha256.clone(),
        };
        let soil_authority = CarriedSoilAuthority {
            owner: &evidence.soil_thermal_owner,
            restart: &evidence.soil_thermal_restart,
            checkpoint: &soil_checkpoint,
        };
        let persisted_soil = SoilThermalOwnerStateRestartV2::from_native(
            parent_soil,
            SoilThermalNativeBundleV2 {
                owner_envelope: evidence.soil_thermal_owner.clone(),
                restart_seal: evidence.soil_thermal_restart.clone(),
                checkpoint_seal: soil_checkpoint.clone(),
                credit_beginning_owner_envelope: None,
                latest_credit_receipt: None,
                expected_accepted_operands: Vec::new(),
                expected_temperature_projections: Vec::new(),
                native_expected_source_set: None,
                native_orchestrator_seals: None,
            },
            &evidence.soil_thermal_owner.state.owner_id,
            &evidence
                .lse_configuration
                .soil_thermal_configuration
                .configuration_sha256,
            &soil_authority,
        )
        .expect("persisted exact fixture soil");
        let projection_v3_bytes = evidence
            .projection_v3
            .canonical_bytes(&evidence.surface_configuration)
            .expect("physical projection bytes");
        let publication =
            FrozenLitterPublicationAuthorityV3::from_projection(&evidence.projection_v3)
                .expect("physical publication authority");
        let projection_authority = NativeFrozenLitterProjectionAuthorityV3;
        let validated_projection = projection_authority
            .validate_projection(
                &evidence.surface_configuration,
                &projection_v3_bytes,
                &publication,
            )
            .expect("physical projection authority");
        let scientific_context = FrozenLitterExpectedScientificContextV3 {
            lse_configuration: &evidence.lse_configuration,
            surface_liquid_configuration: &evidence.surface_configuration,
            soil_thermal_owner_id: &evidence.soil_thermal_owner.state.owner_id,
            soil_thermal_seal_authority: &soil_authority,
            projection_seal_authority: &projection_authority,
        };
        let scientific = project_frozen_litter_scientific_owner_v3(
            &evidence.lse_configuration,
            &evidence.ending_lse_state,
            &evidence.surface_configuration,
            &evidence.ending_surface_owner,
            validated_projection.wb14_parent_working_state_bytes.clone(),
            persisted_soil,
            projection_v3_bytes.clone(),
            publication,
            &scientific_context,
        )
        .expect("physical scientific owner");
        let parent_v2 = wire_digest('7');
        let frozen_run = wire_digest('8');
        let frozen_topology = wire_digest('9');
        let physical_checkpoint = DirectFrozenLitterCheckpointV3::new(
            parent_v2.clone(),
            frozen_run.clone(),
            frozen_topology.clone(),
            scientific,
        )
        .expect("physical checkpoint");
        let physical_context = ExpectedFrozenLitterCheckpointContextV3 {
            parent_v2_checkpoint_sha256: &parent_v2,
            run_identity_sha256: &frozen_run,
            topology_sha256: &frozen_topology,
            scientific: scientific_context,
        };
        let exact_context = ExpectedFrozenLitterExactEnthalpyContextV4 {
            parent_v3: physical_context,
            exact_surface_owner_id: &evidence.ending_exact_surface_owner.owner_id,
            accepted_support_beginning_lse_v3_state_sha256: &evidence
                .beginning_lse_state
                .0
                .state_sha256,
            publication_history_beginning_lse_v3_state_sha256: &evidence
                .beginning_lse_state
                .0
                .state_sha256,
        };
        let exact_checkpoint = DirectFrozenLitterExactEnthalpyCheckpointV4::accepted_credit(
            physical_checkpoint,
            evidence.beginning_exact_surface_owner.clone(),
            evidence.ending_exact_surface_restart.clone(),
            evidence.ending_exact_surface_checkpoint.clone(),
            evidence.projection_v4.clone(),
            &exact_context,
        )
        .expect("accepted exact checkpoint");
        let projection_v4_bytes = evidence
            .projection_v4
            .canonical_bytes(&evidence.surface_configuration)
            .expect("exact projection bytes");
        let supplement = SnowStage3V11ExactResidentSupplementV4 {
            checkpoint: exact_checkpoint,
            physical_v3_publication_bytes: vec![evidence.physical_v3_publication_bytes.clone()],
            exact_v4_publication_bytes: vec![projection_v4_bytes.clone()],
        };
        let resident_set = SnowStage3V11ExactResidentSetV4 {
            committed: supplement.clone(),
            pending_candidate: None,
            in_progress_day_candidate: None,
            in_progress_support_current: None,
        };
        let resident_contexts = ExpectedSnowStage3V11ExactResidentContextsV4 {
            committed: &exact_context,
            pending_candidate: None,
            in_progress_day_candidate: None,
            in_progress_support_current: None,
        };

        let mut physical = FrozenLitterV3Resident::try_new(
            evidence.lse_configuration.clone(),
            evidence.ending_lse_state.clone(),
            evidence.surface_configuration.clone(),
            evidence.ending_surface_owner.clone(),
        )
        .expect("live physical resident");
        physical
            .restore_restart_authority(
                std::slice::from_ref(&evidence.physical_v3_publication_bytes),
                (!validated_projection
                    .wb14_parent_working_state_bytes
                    .is_empty())
                .then_some(
                    validated_projection
                        .wb14_parent_working_state_bytes
                        .as_slice(),
                ),
                &evidence.projection_v3.identity().receipt_chain_sha256,
            )
            .expect("live physical publication history");
        let exact = FrozenLitterV4Resident::try_restore(
            &physical,
            evidence.ending_exact_surface_owner.clone(),
            std::slice::from_ref(&projection_v4_bytes),
            &evidence.beginning_lse_state.0.state_sha256,
        )
        .expect("live exact publication history");

        let mut fixture = restart_authority_prepared_day_fixture();
        let source = &fixture.owners.runtime.shadow;
        let (baseline_run, baseline_topology) = checkpoint_identities_v1(
            &fixture.owners.committed,
            source.root_zone_hydraulic_configuration(),
        )
        .expect("baseline restart identities");
        let baseline_context = ExpectedRestartStaticContext {
            run_identity_sha256: &baseline_run,
            topology_sha256: &baseline_topology,
            vegetation_configuration: source.restart_authority_vegetation_configuration(),
            vegetation_owner_id: source.restart_authority_vegetation_owner_id(),
            soil_thermal_owner_id: &source
                .restart_authority_soil_thermal()
                .expect("baseline soil resident")
                .owner_id,
            soil_thermal_configuration_sha256: &source
                .restart_authority_soil_thermal()
                .expect("baseline soil resident")
                .configuration_sha256,
            lse_configuration: source.restart_authority_lse_configuration(),
            surface_liquid_configuration: source.restart_authority_surface_configuration(),
            gsi_configuration: source.gsi_owner_configuration(),
            forcing_static_configuration: source.provider_static_configuration(),
            root_zone_hydraulic_configuration: source.root_zone_hydraulic_configuration(),
            phase_plan: &source.restart_authority_hydrology_frame().phase_plan,
            phase_plan_sha256: &fixture.owners.phase_plan_sha256,
            day_inputs: &fixture.owners.day_inputs,
            day_input_digests: &fixture.owners.day_input_digests,
        };
        let mut baseline_checkpoint = crate::DirectV10RealConsumerCheckpointV1 {
            schema: "OPENWEPP_DIRECT_V10_REAL_CONSUMER_CHECKPOINT_V1".into(),
            version: 1,
            run_identity_sha256: baseline_run.clone(),
            topology_sha256: baseline_topology.clone(),
            phase: crate::DirectV10CheckpointPhaseV1::BetweenDays {
                next_day_index: crate::WireDayIndex(0),
                accepted_interval_count: crate::AcceptedIntervalCount::try_new(0)
                    .expect("baseline interval count"),
                committed: fixture.owners.committed.clone(),
            },
            payload_sha256: wire_digest('0'),
        };
        baseline_checkpoint
            .seal()
            .expect("baseline checkpoint seal");
        let baseline = crate::admit_checkpoint_v1(
            &to_canonical_bytes(&baseline_checkpoint).expect("baseline checkpoint bytes"),
            &baseline_context,
        )
        .expect("baseline checkpoint admission");
        let baseline_host = crate::DirectV10RestartHost::from_isolated(baseline, &baseline_context)
            .expect("baseline checkpoint host");
        fixture.owners.runtime.shadow = baseline_host.shadow().clone();
        let mut inner = fixture.owners.runtime.shadow.clone();
        inner
            .install_restored_frozen_litter_v4_residents(physical, exact)
            .expect("live exact resident installation");
        let lane_id = inner
            .restart_authority_hydrology_frame()
            .lanes
            .first()
            .expect("exact fixture lane")
            .lane_id;
        let snow = Wb11HydrologyKernel::initialize_stage3_persistent_state(lane_id, Vec::new())
            .expect("empty exact fixture snow state");
        let attachment = DirectSnowStage3V11ShadowAttachment::new_production(
            DirectSnowStage3V11ProductionConfigurationV1 {
                run_identity: digest_bytes(b"transaction-exact-stage3-run"),
                topology_identity: digest_bytes(b"transaction-exact-stage3-topology"),
                calendar_receipt: digest_bytes(b"transaction-exact-stage3-calendar"),
                controller_policy: digest_bytes(b"transaction-exact-stage3-controller"),
                surface_liquid_configuration: inner
                    .restart_authority_surface_configuration()
                    .clone(),
                wb14_parameters: vec![DirectOfeWb14Parameters {
                    ofe_id: evidence.surface_configuration.parent().ofe_topology[0].clone(),
                    effective_conductivity_m_s: 1.0e-6,
                    matric_potential_m: 0.1,
                    infiltration_storage_capacity_m: 0.04,
                }],
            },
            BTreeMap::from([(lane_id, snow)]),
            inner,
        )
        .expect("exact production Stage-3 attachment");
        let static_context = attachment.static_context.clone();
        let mut exact_frame = fixture
            .owners
            .runtime
            .shadow
            .restart_authority_hydrology_frame()
            .clone();
        exact_frame.snow_stage3_v11_attachment = Some(Box::new(attachment));
        fixture
            .owners
            .runtime
            .shadow
            .restart_authority_install_exact_hydrology_frame_v2(exact_frame)
            .expect("outer exact hydrology installation");

        let shadow = &fixture.owners.runtime.shadow;
        assert!(
            DirectV10PreparedDayTransactionV1::prepare(
                shadow,
                &fixture.request,
                fixture.template.clone(),
                fixture.owners.phase_plan_sha256.clone(),
                fixture.owners.day_input_digests.clone(),
            )
            .is_err()
        );
        let (run, topology) = checkpoint_identities_v1(
            &fixture.owners.committed,
            shadow.root_zone_hydraulic_configuration(),
        )
        .expect("outer restart identities");
        let parent_context = ExpectedRestartStaticContext {
            run_identity_sha256: &run,
            topology_sha256: &topology,
            vegetation_configuration: shadow.restart_authority_vegetation_configuration(),
            vegetation_owner_id: shadow.restart_authority_vegetation_owner_id(),
            soil_thermal_owner_id: &shadow
                .restart_authority_soil_thermal()
                .expect("outer soil resident")
                .owner_id,
            soil_thermal_configuration_sha256: &shadow
                .restart_authority_soil_thermal()
                .expect("outer soil resident")
                .configuration_sha256,
            lse_configuration: shadow.restart_authority_lse_configuration(),
            surface_liquid_configuration: shadow.restart_authority_surface_configuration(),
            gsi_configuration: shadow.gsi_owner_configuration(),
            forcing_static_configuration: shadow.provider_static_configuration(),
            root_zone_hydraulic_configuration: shadow.root_zone_hydraulic_configuration(),
            phase_plan: &shadow.restart_authority_hydrology_frame().phase_plan,
            phase_plan_sha256: &fixture.owners.phase_plan_sha256,
            day_inputs: &fixture.owners.day_inputs,
            day_input_digests: &fixture.owners.day_input_digests,
        };
        let hydrology_context = ExpectedDirectHydrologyRestartContext {
            phase_plan: &shadow.restart_authority_hydrology_frame().phase_plan,
            phase_plan_sha256: &fixture.owners.phase_plan_sha256,
            day_inputs: &fixture.owners.day_inputs,
            day_input_digests: &fixture.owners.day_input_digests,
            surface_liquid_configuration: shadow.restart_authority_surface_configuration(),
        };
        let stage3_context = ExpectedSnowStage3V11RestartContext {
            static_context: &static_context,
            real_consumer_context: &parent_context,
        };
        let manifest = Stage3CommittedDayArchiveManifestV1::empty(
            static_context.run_identity,
            static_context.topology_identity,
        )
        .expect("empty exact archive manifest");
        let archive_reader = EmptyArchiveReader;
        let archive = ExpectedStage3CommittedDayArchiveV3 {
            manifest: &manifest,
            reader: &archive_reader,
        };
        let exact_stage3_context = ExpectedSnowStage3V11ExactEnthalpyRestartContextV4 {
            stage3: &stage3_context,
            archive: &archive,
            exact: ExpectedSnowStage3V11ExactResidentContextsV4 {
                committed: &exact_context,
                pending_candidate: None,
                in_progress_day_candidate: None,
                in_progress_support_current: None,
            },
        };
        let exact_hydrology_context = ExpectedDirectHydrologyExactEnthalpyRestartContextV2 {
            hydrology: &hydrology_context,
            stage3_v4: &exact_stage3_context,
        };
        let exact_inputs = || DirectHydrologyExactEnthalpyProjectionInputsV2 {
            archive: &archive,
            exact_residents: resident_set.clone(),
            exact_contexts: &resident_contexts,
        };
        let mut transaction = DirectV10PreparedDayExactEnthalpyTransactionV2::prepare(
            shadow,
            &fixture.request,
            fixture.template.clone(),
            fixture.owners.phase_plan_sha256.clone(),
            fixture.owners.day_input_digests.clone(),
            exact_inputs(),
        )
        .expect("exact production transaction prepare");
        transaction
            .advance_one_interval()
            .expect("exact production interval");
        let checkpoint = transaction
            .checkpoint(exact_inputs())
            .expect("exact production checkpoint");
        let decoded = DirectV10ExactEnthalpyCheckpointV2::from_canonical_bytes(&checkpoint)
            .expect("exact production checkpoint admission");
        let super::DirectV10ExactEnthalpyCheckpointPhaseV2::InProgressDay {
            staged_hydrology, ..
        } = &decoded.exact_phase
        else {
            panic!("one interval must checkpoint in progress");
        };
        assert_eq!(
            staged_hydrology
                .stage3_v4
                .exact_residents
                .committed
                .checkpoint,
            supplement.checkpoint
        );
        let mut poisoned = checkpoint.clone();
        *poisoned.last_mut().expect("checkpoint byte") ^= 1;
        assert!(
            DirectV10PreparedDayExactEnthalpyTransactionV2::restore(
                &poisoned,
                &parent_context,
                &exact_hydrology_context,
                &exact_hydrology_context,
            )
            .is_err()
        );
        let mut parent_digest_poison = decoded.clone();
        let mut nested_parent: crate::DirectV10RealConsumerCheckpointV1 =
            crate::from_canonical_bytes(&parent_digest_poison.parent_v1_checkpoint_bytes)
                .expect("nested parent digest poison fixture");
        nested_parent.payload_sha256 = wire_digest('f');
        parent_digest_poison.parent_v1_checkpoint_bytes =
            to_canonical_bytes(&nested_parent).expect("nested parent digest poison bytes");
        parent_digest_poison.parent_v1_checkpoint_sha256 = Sha256Hex::try_new(
            crate::canonical_sha256(&parent_digest_poison.parent_v1_checkpoint_bytes)
                .expect("nested parent digest poison outer digest"),
        )
        .expect("nested parent digest poison outer digest wire");
        parent_digest_poison
            .seal()
            .expect("nested parent digest poison outer seal");
        assert!(matches!(
            DirectV10PreparedDayExactEnthalpyTransactionV2::restore(
                &parent_digest_poison
                    .to_canonical_bytes()
                    .expect("nested parent digest poison checkpoint"),
                &parent_context,
                &exact_hydrology_context,
                &exact_hydrology_context,
            ),
            Err(super::RestartTransactionError::Admission(
                crate::RestartAdmissionFailureV1::PayloadDigest
            ))
        ));
        let restored = DirectV10PreparedDayExactEnthalpyTransactionV2::restore(
            &checkpoint,
            &parent_context,
            &exact_hydrology_context,
            &exact_hydrology_context,
        )
        .expect("exact production checkpoint restore after poison refusal");
        assert_eq!(
            restored
                .checkpoint(exact_inputs())
                .expect("restored exact checkpoint"),
            checkpoint
        );
        let (aborted_parent, aborted_exact) = restored.abort();
        assert_eq!(
            aborted_exact.stage3_v4.exact_residents.committed.checkpoint,
            supplement.checkpoint
        );
        assert_eq!(
            aborted_parent.scientific.direct_hydrology,
            decoded_parent(&decoded)
        );
    }

    fn decoded_parent(
        checkpoint: &DirectV10ExactEnthalpyCheckpointV2,
    ) -> crate::DirectHydrologyRestartV1 {
        let parent: crate::DirectV10RealConsumerCheckpointV1 =
            crate::from_canonical_bytes(&checkpoint.parent_v1_checkpoint_bytes)
                .expect("nested parent checkpoint");
        match parent.phase {
            crate::DirectV10CheckpointPhaseV1::InProgressDay {
                committed_day_beginning,
                ..
            } => committed_day_beginning.scientific.direct_hydrology,
            crate::DirectV10CheckpointPhaseV1::BetweenDays { committed, .. } => {
                committed.scientific.direct_hydrology
            }
        }
    }
}
