//! Persistent, default-off Stage-3/V11 shadow attachment.
//!
//! The attachment is deliberately owned by `DirectRunFrame`.  It is advanced
//! by the ordinary scheduler after the live day spans have produced their
//! operands, and its candidate is committed only after the day frame has
//! crossed the scheduler commit boundary.  Callers provide sealed forcing and
//! static owner configuration; they do not construct a carrier, event,
//! ledger, owner set, or owner executor.

use std::collections::BTreeMap;

use openwepp_coupled_time::{ModelTimeNs, digest_bytes};
use openwepp_kernel_contract::{ResourceOwnerId, TransactionId};
use openwepp_land_surface_energy::{ParcelId, Sha256Digest};
use serde::{Deserialize, Serialize};

use super::{
    DirectDayFrame, DirectIngressAmount, DirectOfeWb14Parameters, DirectOpenLiquidIngressParcel,
    DirectPublicationDayInput, DirectRunFrame, DirectRuntimeError,
    DirectSurfaceLiquidConfiguration, DirectSurfaceLiquidIngressInput,
    DirectSurfaceLiquidOwnedState, DirectSurfaceLiquidParcelKind, DirectTileGroundIngress,
    apply_surface_liquid_resource_phase, authorize_surface_liquid_withdrawals,
    execute_surface_liquid_ingress,
};
use crate::snow_stage3_terminal_handoff::{
    CanopyLongwaveComponent, CarrierSurface, CompleteOwnerSet, ParticipantSupportReceipt,
    SealedExposureReceipt, SegmentPhase, SharedCarrierInput, SnowCarrierLedgerInput,
    SnowStage3HandoffError, SnowStage3HandoffRuntime, SnowStage3OwnerExecutionReceipt,
    SnowStage3TerminalHandoffRequest, TerminalEventInput, TerminalStateRates,
};

const SIGMA_W_M2_K4: f64 = 5.670_374_419e-8;
const TERMINAL_TEMPERATURE_K: f64 = 273.15;
const TERMINAL_ENTHALPY_J_KG: f64 = 0.0;
const NS_PER_SECOND: f64 = 1.0e9;
const WATER_DENSITY_KG_M3: f64 = 1_000.0;

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct DirectSnowStage3StagedSurfaceReceipt {
    pub receipt_id: String,
    pub temperature_k: f64,
    pub specific_humidity: f64,
    pub heat_transfer_m_s: f64,
    pub vapor_transfer_m_s: f64,
}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct DirectSnowStage3SealedForcing {
    pub exposure: SealedExposureReceipt,
    pub air_temperature_k: f64,
    pub air_specific_humidity: f64,
    pub atmospheric_longwave_w_m2: f64,
    pub canopy: DirectSnowStage3StagedSurfaceReceipt,
    pub snow: DirectSnowStage3StagedSurfaceReceipt,
    pub canopy_longwave_components: Vec<CanopyLongwaveComponent>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct DirectSnowStage3ShadowConfiguration {
    pub enabled: bool,
    pub event_lane_index: usize,
    pub event_day_index: usize,
    pub parent_duration_ns: u128,
    pub event_elapsed_ns: u128,
    pub minimum_support_ns: u128,
    pub sealed_forcing: DirectSnowStage3SealedForcing,
    pub surface_liquid_configuration: DirectSurfaceLiquidConfiguration,
    pub wb14_parameters: Vec<DirectOfeWb14Parameters>,
}

/// Candidate additive restart payload for a versioned Stage-3 successor.
///
/// The frozen `DirectHydrologyRestartV1` wire is snow-free and deliberately
/// does not carry this payload. Projection to that wire rejects a configured
/// attachment instead of silently discarding its state.
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct DirectSnowStage3ShadowRestartV1 {
    pub enabled: bool,
    pub event_lane_index: usize,
    pub event_day_index: usize,
    pub parent_duration_ns: u128,
    pub event_elapsed_ns: u128,
    pub minimum_support_ns: u128,
    pub sealed_forcing: DirectSnowStage3SealedForcing,
    pub wb14_parameters: Vec<DirectOfeWb14Parameters>,
    pub runtime: SnowStage3HandoffRuntime,
    #[serde(default)]
    pub terminal_consumed: bool,
}

#[derive(Clone, Debug, PartialEq)]
pub struct DirectSnowStage3ShadowAttachment {
    configuration: DirectSnowStage3ShadowConfiguration,
    runtime: SnowStage3HandoffRuntime,
    surface_liquid_configuration: DirectSurfaceLiquidConfiguration,
    pending_surface_liquid: Option<DirectSurfaceLiquidOwnedState>,
    pending_event_day: Option<usize>,
    terminal_consumed: bool,
}

impl DirectSnowStage3ShadowAttachment {
    pub(crate) fn from_frame(
        frame: &DirectRunFrame,
        configuration: DirectSnowStage3ShadowConfiguration,
    ) -> Result<Self, DirectRuntimeError> {
        validate_configuration(&configuration)?;
        let surface_state = frame.surface_liquid_shadow.as_deref().ok_or(
            DirectRuntimeError::DirectKernelGuardFailure {
                phase: "snow_stage3_shadow.configuration",
                detail: "Stage-3 attachment requires the persistent surface-liquid owner".into(),
            },
        )?;
        let beginning_owners = owner_set_from_live_frame(
            frame,
            surface_state,
            &configuration.surface_liquid_configuration,
            configuration.event_lane_index,
        )?;
        let runtime = SnowStage3HandoffRuntime::new(ModelTimeNs::new(0), beginning_owners)
            .map_err(handoff_runtime_error("snow_stage3_shadow.configuration"))?;
        Ok(Self {
            surface_liquid_configuration: configuration.surface_liquid_configuration.clone(),
            configuration,
            runtime,
            pending_surface_liquid: None,
            pending_event_day: None,
            terminal_consumed: false,
        })
    }

    #[must_use]
    pub fn enabled(&self) -> bool {
        self.configuration.enabled
    }

    #[must_use]
    pub fn runtime(&self) -> &SnowStage3HandoffRuntime {
        &self.runtime
    }

    pub fn restart_v1(&self) -> Result<DirectSnowStage3ShadowRestartV1, DirectRuntimeError> {
        if self.pending_event_day.is_some() || self.pending_surface_liquid.is_some() {
            return Err(DirectRuntimeError::DirectKernelGuardFailure {
                phase: "snow_stage3_shadow.restart",
                detail: "cannot persist an uncommitted Stage-3 owner candidate".into(),
            });
        }
        self.runtime
            .validate_restored()
            .map_err(handoff_runtime_error("snow_stage3_shadow.restart"))?;
        Ok(DirectSnowStage3ShadowRestartV1 {
            enabled: self.configuration.enabled,
            event_lane_index: self.configuration.event_lane_index,
            event_day_index: self.configuration.event_day_index,
            parent_duration_ns: self.configuration.parent_duration_ns,
            event_elapsed_ns: self.configuration.event_elapsed_ns,
            minimum_support_ns: self.configuration.minimum_support_ns,
            sealed_forcing: self.configuration.sealed_forcing.clone(),
            wb14_parameters: self.configuration.wb14_parameters.clone(),
            runtime: self.runtime.clone(),
            terminal_consumed: self.terminal_consumed,
        })
    }

    pub fn from_restart(
        frame: &DirectRunFrame,
        restart: DirectSnowStage3ShadowRestartV1,
        surface_liquid_configuration: DirectSurfaceLiquidConfiguration,
    ) -> Result<Self, DirectRuntimeError> {
        if frame.surface_liquid_shadow.is_none() {
            return Err(DirectRuntimeError::DirectKernelGuardFailure {
                phase: "snow_stage3_shadow.restart",
                detail: "restored Stage-3 attachment has no restored surface-liquid owner".into(),
            });
        }
        restart
            .runtime
            .validate_restored()
            .map_err(handoff_runtime_error("snow_stage3_shadow.restart"))?;
        let runtime = restart.runtime;
        let terminal_consumed = restart.terminal_consumed || !runtime.receipt_history().is_empty();
        let configuration = DirectSnowStage3ShadowConfiguration {
            enabled: restart.enabled,
            event_lane_index: restart.event_lane_index,
            event_day_index: restart.event_day_index,
            parent_duration_ns: restart.parent_duration_ns,
            event_elapsed_ns: restart.event_elapsed_ns,
            minimum_support_ns: restart.minimum_support_ns,
            sealed_forcing: restart.sealed_forcing,
            surface_liquid_configuration: surface_liquid_configuration.clone(),
            wb14_parameters: restart.wb14_parameters,
        };
        validate_configuration(&configuration)?;
        Ok(Self {
            configuration,
            runtime,
            surface_liquid_configuration,
            pending_surface_liquid: None,
            pending_event_day: None,
            terminal_consumed,
        })
    }

    /// Stage the event after all ordinary hydrology/LSE day spans have run.
    /// The returned candidate is held inside the attachment until the frame
    /// itself commits.  A caller cannot supply any of the handoff DTOs here.
    pub(crate) fn stage_after_live_day(
        &mut self,
        frame: &DirectRunFrame,
        day_input: &DirectPublicationDayInput,
        day_frame: &DirectDayFrame,
    ) -> Result<(), DirectRuntimeError> {
        if !self.configuration.enabled
            || day_frame.lane_index != self.configuration.event_lane_index
            || day_frame.day_index != self.configuration.event_day_index
        {
            return Ok(());
        }
        if self.terminal_consumed {
            return Ok(());
        }
        if self.pending_event_day.is_some() {
            return Err(DirectRuntimeError::DirectKernelGuardFailure {
                phase: "snow_stage3_shadow.stage",
                detail: "Stage-3 attachment already has an uncommitted event".into(),
            });
        }
        let beginning_surface = frame.surface_liquid_shadow.as_deref().ok_or(
            DirectRuntimeError::DirectKernelGuardFailure {
                phase: "snow_stage3_shadow.stage",
                detail: "live surface-liquid owner is missing at terminal event".into(),
            },
        )?;
        let carrier = self.derive_shared_carrier(day_frame)?;
        let event = self.derive_terminal_event(day_frame)?;
        let terminal_liquid = terminal_liquid_from_live_snow(day_frame)?;
        let continuation_ns = self
            .configuration
            .parent_duration_ns
            .checked_sub(self.configuration.event_elapsed_ns)
            .ok_or(DirectRuntimeError::DirectKernelGuardFailure {
                phase: "snow_stage3_shadow.stage",
                detail: "terminal event elapsed time exceeds its parent support".into(),
            })?;
        let ending_surface = if continuation_ns == 0 {
            beginning_surface.clone()
        } else {
            self.advance_remaining_surface_support(
                beginning_surface,
                day_input,
                day_frame,
                terminal_liquid,
                continuation_ns,
            )?
        };
        let beginning_owners = owner_set_from_live_frame(
            frame,
            beginning_surface,
            &self.surface_liquid_configuration,
            day_frame.lane_index,
        )?;
        if beginning_owners != *self.runtime.committed_owners() {
            return Err(DirectRuntimeError::DirectKernelGuardFailure {
                phase: "snow_stage3_shadow.stage",
                detail: "live owner set does not match the committed Stage-3 owner set".into(),
            });
        }
        let ending_owners = owner_set_after_sequential_consumers(
            day_frame,
            &ending_surface,
            &self.surface_liquid_configuration,
            event.event_ordinal,
        )?;
        let owner_execution = SnowStage3OwnerExecutionReceipt::from_owner_set(
            "ordinary-scheduler-stage3-v11-lse-hydrology",
            ending_owners.clone(),
        )
        .map_err(handoff_runtime_error("snow_stage3_shadow.owner_candidate"))?;
        let request = SnowStage3TerminalHandoffRequest {
            carrier,
            event,
            beginning_owners,
            ending_owners,
            owner_execution,
            retained_liquid_kg_m2: snow_depth_to_mass(
                day_frame.snow_coupling.liquid_water_retained_after_m,
                "snow.liquid_water_retained_after_m",
            )?,
            snow_support_rain_kg_m2: snow_depth_to_mass(
                day_frame.snow_coupling.post_winter_rain_m,
                "snow.post_winter_rain_m",
            )?,
            terminal_melt_kg_m2: snow_depth_to_mass(
                day_frame.snow_coupling.liquid_water_released_m,
                "snow.liquid_water_released_m",
            )?,
            terminal_refreeze_kg_m2: 0.0,
            continuation: crate::snow_stage3_terminal_handoff::SnowFreeContinuationInput {
                duration_ns: ModelTimeNs::new(continuation_ns),
                terminal_liquid_kg_m2: terminal_liquid,
                post_event_contains_snow_operands: false,
            },
        };
        self.runtime
            .stage(request)
            .map_err(handoff_runtime_error("snow_stage3_shadow.stage"))?;
        self.pending_surface_liquid = Some(ending_surface);
        self.pending_event_day = Some(day_frame.day_index);
        Ok(())
    }

    pub(crate) fn commit_after_live_day(
        &mut self,
        frame: &mut DirectRunFrame,
    ) -> Result<(), DirectRuntimeError> {
        let Some(day_index) = self.pending_event_day else {
            return Ok(());
        };
        if day_index != self.configuration.event_day_index {
            return Err(DirectRuntimeError::DirectKernelGuardFailure {
                phase: "snow_stage3_shadow.commit",
                detail: "Stage-3 commit day does not match the staged event".into(),
            });
        }
        let state = self
            .pending_surface_liquid
            .as_ref()
            .ok_or(DirectRuntimeError::DirectKernelGuardFailure {
                phase: "snow_stage3_shadow.commit",
                detail: "surface-liquid candidate missing at Stage-3 commit".into(),
            })?
            .clone();
        let mut runtime_candidate = self.runtime.clone();
        runtime_candidate
            .commit_pending()
            .map_err(handoff_runtime_error("snow_stage3_shadow.commit"))?;
        frame.surface_liquid_shadow = Some(Box::new(state));
        self.runtime = runtime_candidate;
        self.pending_surface_liquid = None;
        self.pending_event_day = None;
        self.terminal_consumed = true;
        Ok(())
    }

    fn derive_shared_carrier(
        &self,
        day_frame: &DirectDayFrame,
    ) -> Result<SharedCarrierInput, DirectRuntimeError> {
        let forcing = &self.configuration.sealed_forcing;
        let duration_s = self.configuration.parent_duration_ns as f64 / NS_PER_SECOND;
        let canopy_cover = day_frame
            .evapotranspiration_compute_inputs
            .canopy_cover_fraction;
        let snow_ice = snow_depth_to_mass(
            day_frame.snow_coupling.runtime_swe_after_m,
            "snow.runtime_swe_after_m",
        )?;
        let liquid = snow_depth_to_mass(
            day_frame.snow_coupling.liquid_water_retained_after_m,
            "snow.liquid_water_retained_after_m",
        )?;
        let canopy_longwave = forcing
            .canopy_longwave_components
            .iter()
            .map(|component| {
                component.emissive_area_weight * SIGMA_W_M2_K4 * component.temperature_k.powi(4)
            })
            .sum::<f64>();
        let sky_view = (1.0 - canopy_cover).powf(1.6);
        let snow_emission = SIGMA_W_M2_K4 * forcing.snow.temperature_k.powi(4);
        let snow_canopy_exchange = (1.0 - sky_view) * (snow_emission - canopy_longwave);
        let exchange = -snow_canopy_exchange * duration_s;
        let support = ParticipantSupportReceipt {
            participant_id: "stage3-snow".into(),
            support_receipt_id: "ordinary-scheduler-stage3-support".into(),
            minimum_support_ns: ModelTimeNs::new(self.configuration.minimum_support_ns),
        };
        Ok(SharedCarrierInput {
            phase: SegmentPhase::SnowCovered,
            rho_air_kg_m3: 1.225,
            cp_air_j_kg_k: 1_004.0,
            reference: CarrierSurface {
                temperature_k: forcing.air_temperature_k,
                specific_humidity: forcing.air_specific_humidity,
                heat_conductance_m_s: forcing.exposure.wind_m_s.max(1.0e-9),
                vapor_conductance_m_s: forcing.exposure.wind_m_s.max(1.0e-9),
            },
            canopy: CarrierSurface {
                temperature_k: forcing.canopy.temperature_k,
                specific_humidity: forcing.canopy.specific_humidity,
                heat_conductance_m_s: forcing.canopy.heat_transfer_m_s,
                vapor_conductance_m_s: forcing.canopy.vapor_transfer_m_s,
            },
            snow: CarrierSurface {
                temperature_k: forcing.snow.temperature_k,
                specific_humidity: forcing.snow.specific_humidity,
                heat_conductance_m_s: forcing.snow.heat_transfer_m_s,
                vapor_conductance_m_s: forcing.snow.vapor_transfer_m_s,
            },
            canopy_longwave_components: forcing.canopy_longwave_components.clone(),
            exposure: forcing.exposure.clone(),
            active_participants: vec!["stage3-snow".into()],
            support_receipts: vec![support],
            atmospheric_longwave_w_m2: forcing.atmospheric_longwave_w_m2,
            effective_canopy_cover: canopy_cover,
            canopy_intercepted_snow: false,
            ledger: SnowCarrierLedgerInput {
                duration_s,
                snow_ice_start_kg_m2: snow_ice,
                solid_precipitation_kg_m2: 0.0,
                melt_kg_m2: 0.0,
                sublimation_kg_m2: 0.0,
                deposition_kg_m2: 0.0,
                liquid_start_kg_m2: liquid,
                rain_kg_m2: 0.0,
                refreeze_kg_m2: 0.0,
                liquid_runoff_kg_m2: 0.0,
                energy_start_j_m2: 0.0,
                external_energy_j_m2: 0.0,
                canopy_energy_j_m2: 0.0,
                snow_energy_j_m2: 0.0,
                energy_end_j_m2: 0.0,
                canopy_snow_longwave_exchange_j_m2: -exchange,
                snow_canopy_longwave_exchange_j_m2: exchange,
            },
        })
    }

    fn derive_terminal_event(
        &self,
        day_frame: &DirectDayFrame,
    ) -> Result<TerminalEventInput, DirectRuntimeError> {
        let start = self.runtime.accepted_cursor_ns();
        let end_ns = start
            .get()
            .checked_add(self.configuration.parent_duration_ns)
            .ok_or(DirectRuntimeError::DirectKernelGuardFailure {
                phase: "snow_stage3_shadow.event",
                detail: "terminal parent tick overflow".into(),
            })?;
        let event_ns = start
            .get()
            .checked_add(self.configuration.event_elapsed_ns)
            .ok_or(DirectRuntimeError::DirectKernelGuardFailure {
                phase: "snow_stage3_shadow.event",
                detail: "terminal event tick overflow".into(),
            })?;
        let support = ParticipantSupportReceipt {
            participant_id: "stage3-snow".into(),
            support_receipt_id: "ordinary-scheduler-stage3-support".into(),
            minimum_support_ns: ModelTimeNs::new(self.configuration.minimum_support_ns),
        };
        let snow = snow_depth_to_mass(day_frame.snow_coupling.runtime_swe_after_m, "snow.swe")?;
        let liquid = terminal_liquid_from_live_snow(day_frame)?;
        Ok(TerminalEventInput {
            parent_identity: format!(
                "run:{}:lane:{}",
                day_frame.identity.run_id, day_frame.lane_index
            ),
            segment_identity: format!(
                "stage3:{}:{}",
                day_frame.day_index,
                self.runtime.accepted_event_ordinal()
            ),
            event_ordinal: self.runtime.accepted_event_ordinal().checked_add(1).ok_or(
                DirectRuntimeError::DirectKernelGuardFailure {
                    phase: "snow_stage3_shadow.event",
                    detail: "terminal event ordinal overflow".into(),
                },
            )?,
            parent_start_tick: start,
            parent_end_tick: ModelTimeNs::new(end_ns),
            proposed_event_tick: ModelTimeNs::new(event_ns),
            candidate_ticks: vec![ModelTimeNs::new(event_ns)],
            pre_active_participants: vec![support.clone()],
            post_active_participants: vec![support],
            event_time_tolerance_ns: ModelTimeNs::new(0),
            snow_mass_tolerance_kg_m2: 0.0,
            liquid_mass_tolerance_kg_m2: 0.0,
            energy_tolerance_j_m2: 0.0,
            terminal_state: TerminalStateRates {
                snow_start_kg_m2: snow,
                snow_rate_kg_m2_s: 0.0,
                snow_target_kg_m2: snow,
                liquid_start_kg_m2: liquid,
                liquid_rate_kg_m2_s: 0.0,
                liquid_target_kg_m2: liquid,
                energy_start_j_m2: 0.0,
                energy_rate_j_m2_s: 0.0,
                energy_target_j_m2: 0.0,
            },
        })
    }

    fn advance_remaining_surface_support(
        &self,
        beginning: &DirectSurfaceLiquidOwnedState,
        _day_input: &DirectPublicationDayInput,
        day_frame: &DirectDayFrame,
        terminal_liquid: f64,
        continuation_ns: u128,
    ) -> Result<DirectSurfaceLiquidOwnedState, DirectRuntimeError> {
        let duration_s = continuation_ns as f64 / NS_PER_SECOND;
        if !duration_s.is_finite() || duration_s <= 0.0 || duration_s > 1_800.0 {
            return Err(DirectRuntimeError::DirectKernelGuardFailure {
                phase: "snow_stage3_shadow.surface_liquid",
                detail: "remaining surface-liquid support is outside the tagged half-open bin"
                    .into(),
            });
        }
        if self.surface_liquid_configuration.records.len() != 1 {
            return Err(DirectRuntimeError::DirectKernelGuardFailure {
                phase: "snow_stage3_shadow.surface_liquid",
                detail: "terminal shadow currently requires one exact receiver store".into(),
            });
        }
        let record = &self.surface_liquid_configuration.records[0];
        let continuation = beginning
            .continuations
            .iter()
            .find(|row| row.ofe_id == record.key.ofe_id)
            .ok_or(DirectRuntimeError::DirectKernelGuardFailure {
                phase: "snow_stage3_shadow.surface_liquid",
                detail: "terminal receiver has no WB14 continuation".into(),
            })?;
        let transaction_id = TransactionId(u128::from(
            self.runtime.accepted_event_ordinal().checked_add(1).ok_or(
                DirectRuntimeError::DirectKernelGuardFailure {
                    phase: "snow_stage3_shadow.surface_liquid",
                    detail: "surface-liquid transaction overflow".into(),
                },
            )?,
        ));
        let source_digest =
            Sha256Digest::try_new(beginning.state_sha256.clone()).map_err(|_| {
                DirectRuntimeError::DirectKernelGuardFailure {
                    phase: "snow_stage3_shadow.surface_liquid",
                    detail: "surface-liquid owner digest is not a sealed SHA-256 identity".into(),
                }
            })?;
        let mass_tile = terminal_liquid / record.tile_fraction;
        let amount = DirectIngressAmount {
            mass_kg_m2_tile_ground: mass_tile,
            temperature_k: TERMINAL_TEMPERATURE_K,
            specific_liquid_enthalpy_j_kg: TERMINAL_ENTHALPY_J_KG,
            start_s: 0.0,
            end_s: duration_s,
        };
        let receiver = DirectOpenLiquidIngressParcel {
            kind: DirectSurfaceLiquidParcelKind::TerminalReceiver,
            parcel_id: ParcelId::try_new(format!(
                "stage3-terminal:{}:{}",
                day_frame.identity.run_id, day_frame.day_index
            ))
            .map_err(|_| DirectRuntimeError::DirectKernelGuardFailure {
                phase: "snow_stage3_shadow.surface_liquid",
                detail: "terminal parcel identity is invalid".into(),
            })?,
            source_owner_id: ResourceOwnerId::try_new("stage3-snow-terminal").map_err(|_| {
                DirectRuntimeError::DirectKernelGuardFailure {
                    phase: "snow_stage3_shadow.surface_liquid",
                    detail: "terminal parcel source owner identity is invalid".into(),
                }
            })?,
            source_ofe_id: record.key.ofe_id.clone(),
            source_tile_id: record.key.tile_id.clone(),
            destination_ofe_id: record.key.ofe_id.clone(),
            destination_tile_id: record.key.tile_id.clone(),
            accepted_source_state_sha256: source_digest,
            amount,
        };
        let ingress = DirectTileGroundIngress::OpenLiquidParcels {
            ofe_id: record.key.ofe_id.clone(),
            tile_id: record.key.tile_id.clone(),
            surface_id: record.key.surface_id.clone(),
            parcels: vec![receiver],
        };
        let input = DirectSurfaceLiquidIngressInput {
            transaction_id,
            day_index: day_frame.day_index,
            interval_index: continuation.next_interval_index,
            interval_s: duration_s,
            tile_ingress: vec![ingress],
            wb14_parameters: self.configuration.wb14_parameters.clone(),
        };
        let arbitration = authorize_surface_liquid_withdrawals(
            &self.surface_liquid_configuration,
            beginning,
            transaction_id,
            beginning
                .records
                .iter()
                .find_map(|record| record.last_accepted_transaction_id),
            &[],
        )
        .map_err(surface_liquid_runtime_error("authorize"))?;
        let resource = apply_surface_liquid_resource_phase(
            &self.surface_liquid_configuration,
            &arbitration,
            &[],
            &[],
        )
        .map_err(surface_liquid_runtime_error("resource"))?;
        let candidate =
            execute_surface_liquid_ingress(&self.surface_liquid_configuration, &resource, &input)
                .map_err(surface_liquid_runtime_error("wb14"))?;
        Ok(candidate.ending_state().clone())
    }
}

impl DirectRunFrame {
    pub fn configure_snow_stage3_shadow(
        &mut self,
        configuration: DirectSnowStage3ShadowConfiguration,
    ) -> Result<(), DirectRuntimeError> {
        if !configuration.enabled {
            self.snow_stage3_shadow = None;
            return Ok(());
        }
        let attachment = DirectSnowStage3ShadowAttachment::from_frame(self, configuration)?;
        self.snow_stage3_shadow = Some(Box::new(attachment));
        Ok(())
    }

    pub fn snow_stage3_shadow_restart(
        &self,
    ) -> Result<Option<DirectSnowStage3ShadowRestartV1>, DirectRuntimeError> {
        self.snow_stage3_shadow
            .as_deref()
            .map(DirectSnowStage3ShadowAttachment::restart_v1)
            .transpose()
    }

    pub fn restore_snow_stage3_shadow(
        &mut self,
        restart: DirectSnowStage3ShadowRestartV1,
        surface_liquid_configuration: DirectSurfaceLiquidConfiguration,
    ) -> Result<(), DirectRuntimeError> {
        let attachment = DirectSnowStage3ShadowAttachment::from_restart(
            self,
            restart,
            surface_liquid_configuration,
        )?;
        self.snow_stage3_shadow = Some(Box::new(attachment));
        Ok(())
    }
}

fn validate_configuration(
    configuration: &DirectSnowStage3ShadowConfiguration,
) -> Result<(), DirectRuntimeError> {
    let valid_surface_receipts = [
        &configuration.sealed_forcing.canopy,
        &configuration.sealed_forcing.snow,
    ]
    .into_iter()
    .all(|receipt| {
        !receipt.receipt_id.is_empty()
            && receipt.temperature_k.is_finite()
            && receipt.temperature_k > 0.0
            && receipt.specific_humidity.is_finite()
            && (0.0..=1.0).contains(&receipt.specific_humidity)
            && receipt.heat_transfer_m_s.is_finite()
            && receipt.heat_transfer_m_s > 0.0
            && receipt.vapor_transfer_m_s.is_finite()
            && receipt.vapor_transfer_m_s > 0.0
    });
    if configuration.parent_duration_ns == 0
        || configuration.event_elapsed_ns == 0
        || configuration.event_elapsed_ns >= configuration.parent_duration_ns
        || configuration.minimum_support_ns == 0
        || configuration.minimum_support_ns > configuration.event_elapsed_ns
        || configuration
            .surface_liquid_configuration
            .records
            .is_empty()
        || configuration.wb14_parameters.is_empty()
        || !valid_surface_receipts
    {
        return Err(DirectRuntimeError::DirectKernelGuardFailure {
            phase: "snow_stage3_shadow.configuration",
            detail: "invalid terminal parent, event, support, or owner configuration".into(),
        });
    }
    Ok(())
}

fn owner_set_from_live_frame(
    frame: &DirectRunFrame,
    surface: &DirectSurfaceLiquidOwnedState,
    configuration: &DirectSurfaceLiquidConfiguration,
    lane_index: usize,
) -> Result<CompleteOwnerSet, DirectRuntimeError> {
    let surface_bytes = surface.canonical_bytes(configuration).map_err(|_| {
        DirectRuntimeError::DirectKernelGuardFailure {
            phase: "snow_stage3_shadow.owner_binding",
            detail: "surface-liquid owner cannot produce canonical bytes".into(),
        }
    })?;
    let lane = frame
        .lanes
        .get(lane_index)
        .ok_or(DirectRuntimeError::DirectKernelGuardFailure {
            phase: "snow_stage3_shadow.owner_binding",
            detail: "live frame has no lane owner".into(),
        })?;
    let mut owners = BTreeMap::new();
    owners.insert("vegetation".into(), live_payload("vegetation", lane));
    owners.insert("snow".into(), live_payload("snow", lane));
    owners.insert(
        "land_surface_energy".into(),
        live_payload("land_surface_energy", lane),
    );
    owners.insert("surface_liquid".into(), surface_bytes);
    owners.insert("hydrology".into(), live_payload("hydrology", lane));
    owners.insert("bgc".into(), live_payload("bgc", lane));
    owners.insert("soil_thermal".into(), live_payload("soil_thermal", lane));
    CompleteOwnerSet::new(owners).map_err(handoff_runtime_error("snow_stage3_shadow.owner_binding"))
}

fn owner_set_after_sequential_consumers(
    day_frame: &DirectDayFrame,
    surface: &DirectSurfaceLiquidOwnedState,
    configuration: &DirectSurfaceLiquidConfiguration,
    event_ordinal: u64,
) -> Result<CompleteOwnerSet, DirectRuntimeError> {
    let surface_bytes = surface.canonical_bytes(configuration).map_err(|_| {
        DirectRuntimeError::DirectKernelGuardFailure {
            phase: "snow_stage3_shadow.owner_candidate",
            detail: "surface-liquid candidate cannot produce canonical bytes".into(),
        }
    })?;
    let mut owners = BTreeMap::new();
    let mut predecessor = digest_bytes(b"ordinary-scheduler-stage3-owner-sequence");
    for (name, payload) in [
        ("vegetation", format!("{day_frame:?}")),
        ("snow", format!("{:?}", day_frame.snow_coupling)),
        (
            "land_surface_energy",
            format!("{:?}", day_frame.evapotranspiration),
        ),
        (
            "surface_liquid",
            String::from_utf8_lossy(&surface_bytes).into_owned(),
        ),
        ("hydrology", format!("{:?}", day_frame.hydrology_projection)),
        ("bgc", format!("{:?}", day_frame.input_accounting)),
        ("soil_thermal", format!("{:?}", day_frame.winter_column)),
    ] {
        let mut bytes = Vec::new();
        bytes.extend_from_slice(name.as_bytes());
        bytes.extend_from_slice(&event_ordinal.to_le_bytes());
        bytes.extend_from_slice(predecessor.as_bytes());
        bytes.extend_from_slice(payload.as_bytes());
        predecessor = digest_bytes(&bytes);
        owners.insert(name.into(), bytes);
    }
    CompleteOwnerSet::new(owners)
        .map_err(handoff_runtime_error("snow_stage3_shadow.owner_candidate"))
}

fn live_payload<T: std::fmt::Debug>(owner: &str, value: &T) -> Vec<u8> {
    format!("{owner}:{value:?}").into_bytes()
}

fn terminal_liquid_from_live_snow(day_frame: &DirectDayFrame) -> Result<f64, DirectRuntimeError> {
    let retained = snow_depth_to_mass(
        day_frame.snow_coupling.liquid_water_retained_after_m,
        "snow.liquid_water_retained_after_m",
    )?;
    let rain = snow_depth_to_mass(
        day_frame.snow_coupling.post_winter_rain_m,
        "snow.post_winter_rain_m",
    )?;
    let melt = snow_depth_to_mass(
        day_frame.snow_coupling.liquid_water_released_m,
        "snow.liquid_water_released_m",
    )?;
    let terminal = retained + rain + melt;
    terminal
        .is_finite()
        .then_some(terminal)
        .ok_or(DirectRuntimeError::DirectKernelGuardFailure {
            phase: "snow_stage3_shadow.terminal_liquid",
            detail: "terminal liquid parcel arithmetic is nonfinite".into(),
        })
}

fn snow_depth_to_mass(value: f64, field: &'static str) -> Result<f64, DirectRuntimeError> {
    let depth = nonnegative_live(value, field)?;
    let mass = depth * WATER_DENSITY_KG_M3;
    mass.is_finite()
        .then_some(mass)
        .ok_or(DirectRuntimeError::DirectKernelGuardFailure {
            phase: "snow_stage3_shadow.live_owner",
            detail: format!("{field} mass conversion is nonfinite"),
        })
}

fn nonnegative_live(value: f64, field: &'static str) -> Result<f64, DirectRuntimeError> {
    if !value.is_finite() || value < 0.0 {
        return Err(DirectRuntimeError::DirectKernelGuardFailure {
            phase: "snow_stage3_shadow.live_owner",
            detail: format!("{field} is not a finite nonnegative live owner operand"),
        });
    }
    Ok(value)
}

fn handoff_runtime_error(
    phase: &'static str,
) -> impl Fn(SnowStage3HandoffError) -> DirectRuntimeError {
    move |error| DirectRuntimeError::DirectKernelGuardFailure {
        phase,
        detail: error.to_string(),
    }
}

fn surface_liquid_runtime_error(
    phase: &'static str,
) -> impl Fn(super::DirectSurfaceLiquidError) -> DirectRuntimeError {
    move |error| DirectRuntimeError::DirectKernelGuardFailure {
        phase: "snow_stage3_shadow.surface_liquid",
        detail: format!("{phase}: {error}"),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        DirectExecutorMode, DirectFrameExecutor, DirectGroundIngressMode,
        DirectLaneConstructorInputs, DirectPublicationCalendarDay, DirectPublicationRunMetadata,
        DirectRunConstructorInputs, DirectRunIdentity, DirectSurfaceLiquidConfigurationRecord,
        DirectSurfaceLiquidOfeBinding, DirectSurfaceLiquidStoreKey,
    };
    use openwepp_kernel_contract::{ResourceOwnerId, SoilLayerId, TileId};
    use openwepp_land_surface_energy::{OfeId, SourceId, SurfaceClass, SurfaceId, WaterSourceType};

    fn surface_configuration() -> DirectSurfaceLiquidConfiguration {
        let ofe_id = OfeId::try_new("only").expect("ofe");
        let tile_id = TileId::try_new("tile").expect("tile");
        let binding = DirectSurfaceLiquidOfeBinding {
            ofe_id: ofe_id.clone(),
            production_lane_index: 0,
            production_lane_id: 1,
            ordered_soil_layer_ids: vec![SoilLayerId::try_new("soil-top").expect("layer")],
            infiltration_soil_thermal_layer_id: SoilLayerId::try_new("soil-top").expect("layer"),
        };
        let record = DirectSurfaceLiquidConfigurationRecord {
            key: DirectSurfaceLiquidStoreKey {
                run_id: 91,
                ofe_id: ofe_id.clone(),
                tile_id: tile_id.clone(),
                surface_id: SurfaceId::try_new("surface").expect("surface"),
                surface_class: SurfaceClass::BareMineralSoil,
                source_type: WaterSourceType::SurfaceLiquid,
                source_id: SourceId::try_new("source").expect("source"),
            },
            tile_fraction: 1.0,
            capacity_kg_m2_tile: 1.0,
            ofe_area_m2: 100.0,
            ground_ingress_mode: DirectGroundIngressMode::OpenRawPrecipitation,
            runon_destination_ofe_id: None,
            runon_destination_tile_id: None,
        };
        DirectSurfaceLiquidConfiguration::new(
            ResourceOwnerId::try_new("surface-water").expect("owner"),
            91,
            vec![ofe_id],
            vec![binding],
            vec![record],
        )
        .expect("surface configuration")
    }

    fn surface_state(
        configuration: &DirectSurfaceLiquidConfiguration,
    ) -> DirectSurfaceLiquidOwnedState {
        let liquid = configuration
            .records
            .iter()
            .map(|record| (record.key.clone(), 0.0))
            .collect();
        DirectSurfaceLiquidOwnedState::new_initial(configuration, &liquid, 0)
            .expect("surface state")
    }

    fn attachment_configuration(
        configuration: DirectSurfaceLiquidConfiguration,
    ) -> DirectSnowStage3ShadowConfiguration {
        DirectSnowStage3ShadowConfiguration {
            enabled: true,
            event_lane_index: 0,
            event_day_index: 0,
            parent_duration_ns: 1_800_000_000,
            event_elapsed_ns: 1_200_000_000,
            minimum_support_ns: 600_000_000,
            sealed_forcing: DirectSnowStage3SealedForcing {
                exposure: SealedExposureReceipt {
                    receipt_id: "exposure-1".into(),
                    provider: "sealed-stage3-exposure".into(),
                    provider_digest: "digest-1".into(),
                    source: "sealed-exposure-v1".into(),
                    wind_m_s: 2.0,
                    transfer_height_m: 5.0,
                    roughness_m: 0.005,
                },
                air_temperature_k: 273.15,
                air_specific_humidity: 0.001,
                atmospheric_longwave_w_m2: 300.0,
                canopy: DirectSnowStage3StagedSurfaceReceipt {
                    receipt_id: "canopy-receipt-1".into(),
                    temperature_k: 273.15,
                    specific_humidity: 0.001,
                    heat_transfer_m_s: 0.01,
                    vapor_transfer_m_s: 0.01,
                },
                snow: DirectSnowStage3StagedSurfaceReceipt {
                    receipt_id: "snow-receipt-1".into(),
                    temperature_k: 273.15,
                    specific_humidity: 0.001,
                    heat_transfer_m_s: 0.01,
                    vapor_transfer_m_s: 0.01,
                },
                canopy_longwave_components: vec![
                    CanopyLongwaveComponent {
                        temperature_k: 273.15,
                        emissive_area_weight: 0.5,
                    },
                    CanopyLongwaveComponent {
                        temperature_k: 273.15,
                        emissive_area_weight: 0.5,
                    },
                ],
            },
            surface_liquid_configuration: configuration,
            wb14_parameters: vec![DirectOfeWb14Parameters {
                ofe_id: OfeId::try_new("only").expect("ofe"),
                effective_conductivity_m_s: 1.0e-6,
                matric_potential_m: 0.1,
                infiltration_storage_capacity_m: 1.0,
            }],
        }
    }

    #[test]
    fn ordinary_attachment_stages_terminal_receiver_and_restart_round_trips() {
        let configuration = surface_configuration();
        let identity = DirectRunIdentity::new(91, 1, 1, 1).expect("identity");
        let lane = DirectLaneConstructorInputs::from_topology(0, 1, 1).expect("lane");
        let mut frame = DirectRunFrame::from_constructor_inputs(DirectRunConstructorInputs::new(
            identity,
            vec![lane],
        ))
        .expect("frame");
        frame.surface_liquid_shadow = Some(Box::new(surface_state(&configuration)));
        let shadow_configuration = attachment_configuration(configuration.clone());
        frame
            .configure_snow_stage3_shadow(shadow_configuration)
            .expect("attachment");
        let before_event_restart = frame
            .snow_stage3_shadow_restart()
            .expect("before-event restart projection")
            .expect("before-event attachment");
        let mut resumed_before_event = frame.clone();
        resumed_before_event
            .restore_snow_stage3_shadow(before_event_restart, configuration.clone())
            .expect("before-event restore");
        let batch = DirectFrameExecutor::new(DirectExecutorMode::ProductionDirect)
            .run_publication_batch_with_interleaved_day_inputs_and_day_frames(
                &mut frame,
                DirectPublicationRunMetadata {
                    run_name: "stage3-shadow-test".into(),
                    runtime_selection: "default-off-stage3-shadow".into(),
                    output_policy: "test-only".into(),
                },
                |_frame, _day_index, _lane_index| {
                    Ok(DirectPublicationDayInput::calendar_only(
                        DirectPublicationCalendarDay {
                            year: 2026,
                            julian_day: 233,
                            month: 8,
                            day_of_month: 21,
                            water_year: 2026,
                        },
                    ))
                },
            )
            .expect("ordinary scheduler batch");
        let resumed_batch = DirectFrameExecutor::new(DirectExecutorMode::ProductionDirect)
            .run_publication_batch_with_interleaved_day_inputs_and_day_frames(
                &mut resumed_before_event,
                DirectPublicationRunMetadata {
                    run_name: "stage3-shadow-before-event-restart".into(),
                    runtime_selection: "default-off-stage3-shadow".into(),
                    output_policy: "test-only".into(),
                },
                |_frame, _day_index, _lane_index| {
                    Ok(DirectPublicationDayInput::calendar_only(
                        DirectPublicationCalendarDay {
                            year: 2026,
                            julian_day: 233,
                            month: 8,
                            day_of_month: 21,
                            water_year: 2026,
                        },
                    ))
                },
            )
            .expect("ordinary scheduler after before-event restart");
        assert_eq!(batch.rows().len(), 1);
        assert_eq!(resumed_batch.rows().len(), 1);
        let attachment = frame.snow_stage3_shadow.as_deref().expect("attachment");
        let resumed_attachment = resumed_before_event
            .snow_stage3_shadow
            .as_deref()
            .expect("resumed attachment");
        assert_eq!(attachment.runtime().receipt_chain().len(), 1);
        assert_eq!(
            attachment.runtime().accepted_cursor_ns().get(),
            1_800_000_000
        );
        let receipt = attachment
            .runtime()
            .receipt_history()
            .first()
            .expect("terminal receipt");
        assert_eq!(receipt.accepted_event_tick.get(), 1_200_000_000);
        assert_eq!(receipt.continuation_duration_ns.get(), 600_000_000);
        assert_eq!(resumed_attachment.runtime(), attachment.runtime());
        assert_eq!(
            resumed_before_event.surface_liquid_shadow,
            frame.surface_liquid_shadow
        );
        let restart = attachment.restart_v1().expect("restart");
        let restored =
            DirectSnowStage3ShadowAttachment::from_restart(&frame, restart, configuration)
                .expect("restored attachment");
        assert_eq!(restored.runtime(), attachment.runtime());

        let surface_after_event = frame.surface_liquid_shadow.clone();
        let mut replay = frame.clone();
        let replay_batch = DirectFrameExecutor::new(DirectExecutorMode::ProductionDirect)
            .run_publication_batch_with_interleaved_day_inputs_and_day_frames(
                &mut replay,
                DirectPublicationRunMetadata {
                    run_name: "stage3-shadow-replay".into(),
                    runtime_selection: "default-off-stage3-shadow".into(),
                    output_policy: "test-only".into(),
                },
                |_frame, _day_index, _lane_index| {
                    Ok(DirectPublicationDayInput::calendar_only(
                        DirectPublicationCalendarDay {
                            year: 2026,
                            julian_day: 233,
                            month: 8,
                            day_of_month: 21,
                            water_year: 2026,
                        },
                    ))
                },
            )
            .expect("replay must be an ordinary no-op for the terminal attachment");
        assert_eq!(replay_batch.rows().len(), 1);
        assert_eq!(
            replay
                .snow_stage3_shadow
                .as_deref()
                .expect("replay attachment")
                .runtime()
                .receipt_chain()
                .len(),
            1
        );
        assert_eq!(replay.surface_liquid_shadow, surface_after_event);
    }

    #[test]
    fn publication_batch_discards_late_failure_and_owner_candidates() {
        let configuration = surface_configuration();
        let identity = DirectRunIdentity::new(91, 1, 1, 2).expect("identity");
        let lane = DirectLaneConstructorInputs::from_topology(0, 1, 2).expect("lane");
        let mut frame = DirectRunFrame::from_constructor_inputs(DirectRunConstructorInputs::new(
            identity,
            vec![lane],
        ))
        .expect("frame");
        frame.surface_liquid_shadow = Some(Box::new(surface_state(&configuration)));
        frame
            .configure_snow_stage3_shadow(attachment_configuration(configuration))
            .expect("attachment");
        let before = frame.clone();
        let error = DirectFrameExecutor::new(DirectExecutorMode::ProductionDirect)
            .run_publication_batch_with_interleaved_day_inputs_and_day_frames(
                &mut frame,
                DirectPublicationRunMetadata {
                    run_name: "stage3-shadow-late-failure".into(),
                    runtime_selection: "default-off-stage3-shadow".into(),
                    output_policy: "test-only".into(),
                },
                |_frame, day_index, _lane_index| {
                    if day_index == 1 {
                        return Err(DirectRuntimeError::DirectDomainViolation {
                            field: "test.late-publication-failure",
                        });
                    }
                    Ok(DirectPublicationDayInput::calendar_only(
                        DirectPublicationCalendarDay {
                            year: 2026,
                            julian_day: 233,
                            month: 8,
                            day_of_month: 21,
                            water_year: 2026,
                        },
                    ))
                },
            )
            .expect_err("late publication failure");
        assert!(matches!(
            error,
            DirectRuntimeError::DirectDomainViolation {
                field: "test.late-publication-failure"
            }
        ));
        assert_eq!(frame, before);
    }

    #[test]
    fn disabled_attachment_is_absent_from_the_default_frame() {
        let identity = DirectRunIdentity::new(91, 1, 1, 1).expect("identity");
        let lane = DirectLaneConstructorInputs::from_topology(0, 1, 1).expect("lane");
        let mut frame = DirectRunFrame::from_constructor_inputs(DirectRunConstructorInputs::new(
            identity,
            vec![lane],
        ))
        .expect("frame");
        let mut configuration = attachment_configuration(surface_configuration());
        configuration.enabled = false;
        frame
            .configure_snow_stage3_shadow(configuration)
            .expect("default-off configuration");
        assert!(frame.snow_stage3_shadow.is_none());
    }
}
