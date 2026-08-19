use openwepp_hillslope_orchestrator::{
    runtime_inputs::{
        DirectGsiDailyReceiptV1, SnowFreeHalfHourDayReceipt, restart_authority_restore_gsi_state,
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
    BetweenDays {
        shadow: DirectV10RealConsumerShadow,
        accepted_interval_count: u64,
    },
    InProgressDay {
        shadow: DirectV10RealConsumerShadow,
        day_index: u64,
        next_interval_index: u8,
        accepted_interval_count: u64,
        accepted_gsi_daily_receipt: DirectGsiDailyReceiptV1,
        validated_forcing_day_receipts: Vec<SnowFreeHalfHourDayReceipt>,
        continuation_template: DirectV10ShadowDayInput,
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
            } => Ok(Self::BetweenDays {
                shadow: build_shadow(committed, next_day_index, context)?,
                accepted_interval_count,
            }),
            IsolatedRestoredCheckpointV1::InProgressDay {
                day_index,
                next_interval_index,
                accepted_interval_count,
                committed_day_beginning,
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
                let committed = RestoredCompleteCommittedOwnerStateV1 {
                    gsi_state,
                    provider_cursor,
                    scientific: staged_scientific,
                };
                let mut shadow = build_shadow(committed, day_index, context)?;
                shadow.restart_authority_install_scheduler_position(accepted_interval_count)?;
                shadow.restart_authority_install_staged_daily_owners(
                    restart_authority_restore_gsi_state(&staged_gsi_ending_state)
                        .map_err(DirectV10RealConsumerError::from)?,
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
                    accepted_gsi_daily_receipt,
                    validated_forcing_day_receipts,
                    continuation_template,
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
    )?)
}
