use openwepp_hillslope_orchestrator::{
    runtime_inputs::{
        HillslopeClimateRuntimeRequest, PreparedSnowFreeGsiDayV1,
        restart_authority_project_gsi_state,
    },
    v9_real_consumer_shadow::{
        DirectV10RealConsumerError, DirectV10RealConsumerShadow, DirectV10ShadowDayInput,
    },
};
use thiserror::Error;

use crate::{
    AcceptedIntervalCount, CompleteCommittedOwnerStateV1, DirectGsiDailyReceiptRestartV1,
    DirectGsiOwnerStateRestartV1, DirectV10CheckpointPhaseV1,
    DirectV10ContinuationTemplateRestartV1, DirectV10RealConsumerCheckpointV1,
    DirectV10RestartHost, ExpectedRestartStaticContext, InProgressIntervalIndex,
    RestartAdmissionFailureV1, Sha256Hex, SnowFreeHalfHourDayReceiptRestartV1,
    SnowFreeHalfHourProviderCursorRestartV1, WireDayIndex, admit_checkpoint_v1,
    checkpoint_identities_v1, project_complete_owner_state_v1, project_scientific_owner_state_v1,
    to_canonical_bytes,
};

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
        let committed = project_complete_owner_state_v1(shadow, &phase_plan, &day_inputs, day)
            .map_err(RestartTransactionError::Projection)?;
        let (run, topology) =
            checkpoint_identities_v1(&committed).map_err(RestartTransactionError::Projection)?;
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
                staged_scientific: project_scientific_owner_state_v1(
                    &self.staged,
                    &self.phase_plan,
                    &self.day_inputs,
                )
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
                committed: project_complete_owner_state_v1(
                    &self.staged,
                    &self.phase_plan,
                    &self.day_inputs,
                    usize::try_from(self.day + 1)
                        .map_err(|_| RestartTransactionError::Phase("day width"))?,
                )
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

#[cfg(all(test, feature = "fixtures"))]
mod tests {
    use crate::{
        ExpectedRestartStaticContext, restart_authority_prepared_day_fixture, to_canonical_bytes,
    };

    use super::DirectV10PreparedDayTransactionV1;

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
        let soil_thermal_owner_id = source.restart_authority_soil_thermal().owner_id.clone();
        let soil_thermal_configuration_sha256 = source
            .restart_authority_soil_thermal()
            .configuration_sha256
            .clone();
        let lse_configuration = source.restart_authority_lse_configuration().clone();
        let surface_liquid_configuration = source.restart_authority_surface_configuration().clone();
        let gsi_configuration = source.gsi_owner_configuration().clone();
        let forcing_static_configuration = source.provider_static_configuration().clone();
        let phase_plan = source
            .restart_authority_hydrology_frame()
            .phase_plan
            .clone();
        let (run, topology) = crate::checkpoint_identities_v1(&committed).unwrap();
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
}
