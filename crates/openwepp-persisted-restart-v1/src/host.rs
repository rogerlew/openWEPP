use openwepp_hillslope_orchestrator::{
    runtime_inputs::{
        DirectGsiDailyReceiptV1, PreparedSnowFreeGsiDayV1, SnowFreeHalfHourDayReceipt,
        restart_authority_prepare_from_restored_receipts, restart_authority_restore_gsi_state,
    },
    v9_real_consumer_shadow::{
        DirectV10RealConsumerError, DirectV10RealConsumerShadow, DirectV10ShadowDayInput,
    },
    vegetation_real_hydrology_shadow::{RealHydrologyLaneLayerMap, RealHydrologyOfeLaneId},
};
use thiserror::Error;

use crate::{
    ExpectedRestartStaticContext, IsolatedRestoredCheckpointV1, RestartAdmissionFailureV1,
    RestoredCompleteCommittedOwnerStateV1, RestoredScientificOwnerStateSetV1, admit_checkpoint_v1,
};

/// Default-off host for one isolated restored V10 real-consumer owner set.
///
/// Construction is fallible, installation is one assignment, and this type
/// has no selector, production-frame mutation, or publication API.
pub enum DirectV10RestartHost {
    #[non_exhaustive]
    BetweenDays {
        shadow: DirectV10RealConsumerShadow,
        accepted_interval_count: u64,
    },
    #[non_exhaustive]
    InProgressDay {
        shadow: DirectV10RealConsumerShadow,
        day_index: u64,
        next_interval_index: u8,
        accepted_interval_count: u64,
        committed_day_beginning: Box<crate::CompleteCommittedOwnerStateV1>,
        accepted_gsi_daily_receipt: DirectGsiDailyReceiptV1,
        validated_forcing_day_receipts: Vec<SnowFreeHalfHourDayReceipt>,
        continuation_template: DirectV10ShadowDayInput,
        prepared: PreparedSnowFreeGsiDayV1,
    },
}

#[derive(Debug, Error)]
pub enum RestartInstallError {
    #[error(transparent)]
    Admission(#[from] RestartAdmissionFailureV1),
    #[error(transparent)]
    Consumer(#[from] DirectV10RealConsumerError),
    #[error("owner_validation: {0}")]
    OwnerValidation(&'static str),
}

/// Admit and construct every owner in isolation, then replace the target once.
pub fn admit_and_install_checkpoint_v1(
    target: &mut DirectV10RestartHost,
    bytes: &[u8],
    context: &ExpectedRestartStaticContext<'_>,
) -> Result<(), RestartInstallError> {
    let restored = admit_checkpoint_v1(bytes, context)?;
    let candidate = DirectV10RestartHost::from_isolated(restored, context)?;
    install_restored_checkpoint(target, candidate);
    Ok(())
}

/// Non-fallible atomic replacement after admission and isolated construction.
pub fn install_restored_checkpoint(
    target: &mut DirectV10RestartHost,
    restored: DirectV10RestartHost,
) {
    *target = restored;
}

impl DirectV10RestartHost {
    pub fn from_isolated(
        restored: IsolatedRestoredCheckpointV1,
        context: &ExpectedRestartStaticContext<'_>,
    ) -> Result<Self, RestartInstallError> {
        match restored {
            IsolatedRestoredCheckpointV1::BetweenDays {
                next_day_index,
                accepted_interval_count,
                committed,
            } => {
                let mut shadow = build_shadow(committed, next_day_index, context)?;
                if accepted_interval_count != 0 {
                    shadow.restart_authority_install_scheduler_position(accepted_interval_count)?;
                }
                Ok(Self::BetweenDays {
                    shadow,
                    accepted_interval_count,
                })
            }
            IsolatedRestoredCheckpointV1::InProgressDay {
                day_index,
                next_interval_index,
                accepted_interval_count,
                committed_day_beginning,
                committed_day_beginning_wire,
                staged_scientific,
                staged_gsi_ending_state,
                accepted_gsi_daily_receipt,
                validated_forcing_day_receipts,
                ending_provider_cursor,
                continuation_template,
            } => {
                let RestoredCompleteCommittedOwnerStateV1 {
                    gsi_state,
                    provider_cursor,
                    scientific: _,
                } = committed_day_beginning;
                let beginning_cursor = provider_cursor.clone();
                let native_ending_gsi =
                    restart_authority_restore_gsi_state(&staged_gsi_ending_state)
                        .map_err(DirectV10RealConsumerError::from)?;
                let prepared = restart_authority_prepare_from_restored_receipts(
                    accepted_gsi_daily_receipt.clone(),
                    native_ending_gsi.clone(),
                    validated_forcing_day_receipts.clone(),
                    beginning_cursor,
                    ending_provider_cursor.clone(),
                    context.forcing_static_configuration,
                )
                .map_err(DirectV10RealConsumerError::from)?;
                let committed = RestoredCompleteCommittedOwnerStateV1 {
                    gsi_state,
                    provider_cursor,
                    scientific: staged_scientific,
                };
                let mut shadow = build_shadow(committed, day_index, context)?;
                shadow.restart_authority_install_scheduler_position(accepted_interval_count)?;
                shadow.restart_authority_install_staged_daily_owners(
                    native_ending_gsi,
                    ending_provider_cursor,
                    usize::try_from(day_index.checked_add(1).ok_or(
                        RestartInstallError::OwnerValidation("ending provider day overflow"),
                    )?)
                    .map_err(|_| {
                        RestartInstallError::OwnerValidation("ending provider day width")
                    })?,
                )?;
                Ok(Self::InProgressDay {
                    shadow,
                    day_index,
                    next_interval_index,
                    accepted_interval_count,
                    committed_day_beginning: Box::new(committed_day_beginning_wire),
                    accepted_gsi_daily_receipt,
                    validated_forcing_day_receipts,
                    continuation_template,
                    prepared,
                })
            }
        }
    }

    #[must_use]
    pub const fn shadow(&self) -> &DirectV10RealConsumerShadow {
        match self {
            Self::BetweenDays { shadow, .. } | Self::InProgressDay { shadow, .. } => shadow,
        }
    }

    /// Advance an admitted in-progress transaction over the existing physical path.
    pub fn advance_to(&mut self, end_interval_exclusive: u8) -> Result<(), RestartInstallError> {
        let Self::InProgressDay {
            shadow,
            day_index,
            next_interval_index,
            accepted_interval_count,
            continuation_template,
            prepared,
            ..
        } = self
        else {
            return Err(RestartInstallError::OwnerValidation(
                "advance requires in-progress checkpoint",
            ));
        };
        let start = usize::from(*next_interval_index);
        let end = usize::from(end_interval_exclusive);
        shadow.restart_authority_advance_staged_intervals(
            prepared,
            continuation_template.clone(),
            start,
            end,
        )?;
        *accepted_interval_count = shadow.restart_authority_accepted_interval_count();
        *next_interval_index = end_interval_exclusive;
        if u64::try_from(start / 48).unwrap_or(0) > *day_index {
            return Err(RestartInstallError::OwnerValidation("scheduler day drift"));
        }
        Ok(())
    }

    /// Abort an in-progress transaction to the exact admitted day-beginning bytes.
    #[must_use]
    pub fn abort_to_day_beginning(&self) -> Option<&crate::CompleteCommittedOwnerStateV1> {
        match self {
            Self::InProgressDay {
                committed_day_beginning,
                ..
            } => Some(committed_day_beginning),
            Self::BetweenDays { .. } => None,
        }
    }

    /// Finish a fully accepted day with one host replacement.
    pub fn finish(self) -> Result<Self, RestartInstallError> {
        match self {
            Self::InProgressDay {
                shadow,
                accepted_interval_count,
                next_interval_index: 48,
                ..
            } => Ok(Self::BetweenDays {
                shadow,
                accepted_interval_count,
            }),
            Self::InProgressDay { .. } => Err(RestartInstallError::OwnerValidation(
                "finish requires 48 accepted intervals",
            )),
            Self::BetweenDays { .. } => Err(RestartInstallError::OwnerValidation(
                "finish requires in-progress checkpoint",
            )),
        }
    }
}

fn build_shadow(
    committed: RestoredCompleteCommittedOwnerStateV1,
    next_day_index: u64,
    context: &ExpectedRestartStaticContext<'_>,
) -> Result<DirectV10RealConsumerShadow, RestartInstallError> {
    let RestoredCompleteCommittedOwnerStateV1 {
        gsi_state,
        provider_cursor,
        scientific,
    } = committed;
    let RestoredScientificOwnerStateSetV1 {
        vegetation_v10,
        lse_v2,
        direct_hydrology,
        soil_thermal,
        biogeochemistry,
    } = scientific;
    if direct_hydrology.lanes.len() != soil_thermal.ofes.len() {
        return Err(RestartInstallError::OwnerValidation(
            "hydrology/soil OFE cardinality",
        ));
    }
    let layer_maps = direct_hydrology
        .lanes
        .iter()
        .zip(&soil_thermal.ofes)
        .enumerate()
        .map(|(lane_index, (lane, ofe))| RealHydrologyLaneLayerMap {
            ofe_lane: RealHydrologyOfeLaneId {
                lane_index,
                lane_id: lane.lane_id,
            },
            layer_ids: ofe
                .ordered_layers
                .iter()
                .map(|layer| layer.layer_id.clone())
                .collect(),
        })
        .collect();
    Ok(DirectV10RealConsumerShadow::try_new(
        context.vegetation_configuration.clone(),
        vegetation_v10,
        context.vegetation_owner_id.clone(),
        context.lse_configuration.clone(),
        lse_v2,
        context.surface_liquid_configuration.clone(),
        layer_maps,
        soil_thermal,
        biogeochemistry,
        direct_hydrology,
        usize::try_from(next_day_index)
            .map_err(|_| RestartInstallError::OwnerValidation("next day width"))?,
        context.gsi_configuration.clone(),
        restart_authority_restore_gsi_state(&gsi_state)
            .map_err(DirectV10RealConsumerError::from)?,
        context.forcing_static_configuration.clone(),
        provider_cursor,
        context.root_zone_hydraulic_configuration.clone(),
    )?)
}

#[cfg(all(test, feature = "fixtures"))]
mod tests {
    use openwepp_hillslope_orchestrator::v9_real_consumer_shadow::{
        DirectRootZoneHydraulicConfiguration, DirectRootZoneLayerConfiguration,
        DirectRootZoneStratumGeometry,
    };

    use crate::{
        ExpectedRestartStaticContext, Sha256Hex, admit_and_install_checkpoint_v1,
        project_complete_owner_state_v1, restart_authority_identities,
        restart_authority_in_progress_checkpoint_fixture, to_canonical_bytes,
    };

    use super::DirectV10RestartHost;

    #[test]
    fn install_is_atomic_and_targets_the_actual_consumer_host() {
        let (fixture, checkpoint, run, topology) =
            restart_authority_in_progress_checkpoint_fixture(24);
        let context = ExpectedRestartStaticContext {
            run_identity_sha256: &run,
            topology_sha256: &topology,
            vegetation_configuration: fixture
                .owners
                .runtime
                .shadow
                .restart_authority_vegetation_configuration(),
            vegetation_owner_id: fixture
                .owners
                .runtime
                .shadow
                .restart_authority_vegetation_owner_id(),
            soil_thermal_owner_id: &fixture
                .owners
                .runtime
                .shadow
                .restart_authority_soil_thermal()
                .owner_id,
            soil_thermal_configuration_sha256: &fixture
                .owners
                .runtime
                .shadow
                .restart_authority_soil_thermal()
                .configuration_sha256,
            lse_configuration: fixture
                .owners
                .runtime
                .shadow
                .restart_authority_lse_configuration(),
            surface_liquid_configuration: fixture
                .owners
                .runtime
                .shadow
                .restart_authority_surface_configuration(),
            gsi_configuration: fixture.owners.runtime.shadow.gsi_owner_configuration(),
            forcing_static_configuration: fixture
                .owners
                .runtime
                .shadow
                .provider_static_configuration(),
            root_zone_hydraulic_configuration: fixture
                .owners
                .runtime
                .shadow
                .root_zone_hydraulic_configuration(),
            phase_plan: &fixture
                .owners
                .runtime
                .shadow
                .restart_authority_hydrology_frame()
                .phase_plan,
            phase_plan_sha256: &fixture.owners.phase_plan_sha256,
            day_inputs: &fixture.owners.day_inputs,
            day_input_digests: &fixture.owners.day_input_digests,
        };
        let mut target = DirectV10RestartHost::BetweenDays {
            shadow: fixture.owners.runtime.shadow.clone(),
            accepted_interval_count: 0,
        };
        let before = to_canonical_bytes(
            &project_complete_owner_state_v1(
                target.shadow(),
                &fixture.owners.phase_plan_sha256,
                &fixture.owners.day_input_digests,
                0,
            )
            .unwrap(),
        )
        .unwrap();

        let mut poisoned = checkpoint.clone();
        poisoned.payload_sha256 = Sha256Hex::try_new("f".repeat(64)).unwrap();
        assert!(
            admit_and_install_checkpoint_v1(
                &mut target,
                &to_canonical_bytes(&poisoned).unwrap(),
                &context,
            )
            .is_err()
        );
        let after_failure = to_canonical_bytes(
            &project_complete_owner_state_v1(
                target.shadow(),
                &fixture.owners.phase_plan_sha256,
                &fixture.owners.day_input_digests,
                0,
            )
            .unwrap(),
        )
        .unwrap();
        assert_eq!(after_failure, before);

        let original_root = context.root_zone_hydraulic_configuration;
        let changed_layers = original_root
            .ordered_layers()
            .iter()
            .enumerate()
            .map(|(index, layer)| {
                let (lane_index, lane_id, layer_id, psi_sat, b) = layer.restart_identity_fields();
                DirectRootZoneLayerConfiguration::try_new(
                    lane_index,
                    lane_id,
                    layer_id.clone(),
                    psi_sat,
                    if index == 0 { b + 1.0 } else { b },
                )
                .unwrap()
            })
            .collect();
        let changed_strata = original_root
            .ordered_strata()
            .iter()
            .map(|stratum| {
                let (stratum_id, path_m) = stratum.restart_identity_fields();
                DirectRootZoneStratumGeometry::try_new(stratum_id.clone(), path_m).unwrap()
            })
            .collect();
        let changed_root =
            DirectRootZoneHydraulicConfiguration::try_new(changed_layers, changed_strata).unwrap();
        let changed_context = ExpectedRestartStaticContext {
            root_zone_hydraulic_configuration: &changed_root,
            ..context
        };
        assert!(matches!(
            admit_and_install_checkpoint_v1(
                &mut target,
                &to_canonical_bytes(&checkpoint).unwrap(),
                &changed_context,
            )
            .unwrap_err(),
            crate::RestartInstallError::Admission(
                crate::RestartAdmissionFailureV1::TopologyIdentity
            )
        ));
        let after_root_identity_failure = to_canonical_bytes(
            &project_complete_owner_state_v1(
                target.shadow(),
                &fixture.owners.phase_plan_sha256,
                &fixture.owners.day_input_digests,
                0,
            )
            .unwrap(),
        )
        .unwrap();
        assert_eq!(after_root_identity_failure, before);

        admit_and_install_checkpoint_v1(
            &mut target,
            &to_canonical_bytes(&checkpoint).unwrap(),
            &context,
        )
        .unwrap();
        assert!(matches!(
            target,
            DirectV10RestartHost::InProgressDay {
                next_interval_index: 24,
                accepted_interval_count: 24,
                ..
            }
        ));
        assert_eq!(
            to_canonical_bytes(target.abort_to_day_beginning().unwrap()).unwrap(),
            to_canonical_bytes(&fixture.owners.committed).unwrap()
        );
        target.advance_to(48).unwrap();
        let finished = target.finish().unwrap();
        assert!(matches!(
            finished,
            DirectV10RestartHost::BetweenDays {
                accepted_interval_count: 48,
                ..
            }
        ));
        let (expected_run, expected_topology) = restart_authority_identities(
            &fixture.owners.committed,
            fixture
                .owners
                .runtime
                .shadow
                .root_zone_hydraulic_configuration(),
        );
        assert_eq!(run, expected_run);
        assert_eq!(topology, expected_topology);
    }
}
