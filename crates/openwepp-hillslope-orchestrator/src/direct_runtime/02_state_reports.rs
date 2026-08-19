#[derive(Debug, Clone, PartialEq)]
pub struct DirectPhasePlan {
    phases: [DirectPhaseKind; DIRECT_PHASE_COUNT],
}

impl DirectPhasePlan {
    #[must_use]
    pub const fn phases(&self) -> &[DirectPhaseKind; DIRECT_PHASE_COUNT] {
        &self.phases
    }

    #[must_use]
    pub const fn len(&self) -> usize {
        DIRECT_PHASE_COUNT
    }

    #[must_use]
    pub const fn is_empty(&self) -> bool {
        false
    }
}

impl Default for DirectPhasePlan {
    fn default() -> Self {
        Self {
            phases: DirectPhaseKind::ORDERED,
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct DirectWaterState {
    pub soil_water_m: f64,
    pub infiltration_m: f64,
    pub runoff_m: f64,
    pub evapotranspiration_m: f64,
    pub drainage_m: f64,
    pub lateral_flow_m: f64,
}

impl DirectWaterState {
    #[must_use]
    pub const fn zero() -> Self {
        Self {
            soil_water_m: 0.0,
            infiltration_m: 0.0,
            runoff_m: 0.0,
            evapotranspiration_m: 0.0,
            drainage_m: 0.0,
            lateral_flow_m: 0.0,
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "restart-authority-evidence", derive(serde::Serialize))]
pub struct DirectDayForcing {
    pub precipitation_m: f64,
    pub effective_temperature_c: f64,
}

impl DirectDayForcing {
    #[must_use]
    pub const fn zero() -> Self {
        Self {
            precipitation_m: 0.0,
            effective_temperature_c: 0.0,
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct DirectTransferBuffers {
    pub surface_carry_m: [f64; DIRECT_TRANSFER_HOUR_COUNT],
    /// DC01: unit-normalized hourly distribution of `surface_carry_m` totals
    /// (INV-RUNOFFPART-031); consumed by WB14 supply admission.
    pub surface_hourly_weights: [f64; DIRECT_TRANSFER_HOUR_COUNT],
    pub lateral_carry_m: [f64; DIRECT_TRANSFER_HOUR_COUNT],
    pub upstream_flow_m: f64,
    pub subsurface_input_m: f64,
}

impl DirectTransferBuffers {
    #[must_use]
    pub const fn zero() -> Self {
        Self {
            surface_carry_m: [0.0; DIRECT_TRANSFER_HOUR_COUNT],
            surface_hourly_weights: [0.0; DIRECT_TRANSFER_HOUR_COUNT],
            lateral_carry_m: [0.0; DIRECT_TRANSFER_HOUR_COUNT],
            upstream_flow_m: 0.0,
            subsurface_input_m: 0.0,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct DirectInputAccountingState {
    pub precipitation_m: f64,
    pub surface_transfer_m: f64,
    pub lateral_transfer_m: f64,
    pub upstream_flow_m: f64,
    pub subsurface_input_m: f64,
    pub transfer_input_m: f64,
    pub total_accounted_input_m: f64,
}

impl DirectInputAccountingState {
    #[must_use]
    pub const fn zero() -> Self {
        Self {
            precipitation_m: 0.0,
            surface_transfer_m: 0.0,
            lateral_transfer_m: 0.0,
            upstream_flow_m: 0.0,
            subsurface_input_m: 0.0,
            transfer_input_m: 0.0,
            total_accounted_input_m: 0.0,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct DirectDownstreamOperands {
    pub precipitation_m: f64,
    pub surface_transfer_m: f64,
    pub lateral_transfer_m: f64,
    pub upstream_flow_m: f64,
    pub subsurface_input_m: f64,
    pub transfer_input_m: f64,
    pub total_accounted_input_m: f64,
}

impl DirectDownstreamOperands {
    #[must_use]
    pub const fn zero() -> Self {
        Self {
            precipitation_m: 0.0,
            surface_transfer_m: 0.0,
            lateral_transfer_m: 0.0,
            upstream_flow_m: 0.0,
            subsurface_input_m: 0.0,
            transfer_input_m: 0.0,
            total_accounted_input_m: 0.0,
        }
    }
}

impl From<DirectInputAccountingState> for DirectDownstreamOperands {
    fn from(state: DirectInputAccountingState) -> Self {
        Self {
            precipitation_m: state.precipitation_m,
            surface_transfer_m: state.surface_transfer_m,
            lateral_transfer_m: state.lateral_transfer_m,
            upstream_flow_m: state.upstream_flow_m,
            subsurface_input_m: state.subsurface_input_m,
            transfer_input_m: state.transfer_input_m,
            total_accounted_input_m: state.total_accounted_input_m,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct DirectShadowProjection {
    pub lane_index: usize,
    pub day_index: usize,
    pub precipitation_m: f64,
    pub transfer_input_m: f64,
    pub total_accounted_input_m: f64,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct DirectWaterLedgerState {
    pub total_accounted_input_m: f64,
    pub soil_water_m: f64,
    pub available_water_m: f64,
    pub direct_flux_m: f64,
    pub publication_flux_m: f64,
    pub direct_publication_delta_m: f64,
    pub diagnostic_residual_m: f64,
}

impl DirectWaterLedgerState {
    #[must_use]
    pub const fn zero() -> Self {
        Self {
            total_accounted_input_m: 0.0,
            soil_water_m: 0.0,
            available_water_m: 0.0,
            direct_flux_m: 0.0,
            publication_flux_m: 0.0,
            direct_publication_delta_m: 0.0,
            diagnostic_residual_m: 0.0,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct DirectLedgerDownstreamOperands {
    pub total_accounted_input_m: f64,
    pub soil_water_m: f64,
    pub available_water_m: f64,
    pub direct_flux_m: f64,
    pub publication_flux_m: f64,
    pub direct_publication_delta_m: f64,
    pub diagnostic_residual_m: f64,
}

impl DirectLedgerDownstreamOperands {
    #[must_use]
    pub const fn zero() -> Self {
        Self {
            total_accounted_input_m: 0.0,
            soil_water_m: 0.0,
            available_water_m: 0.0,
            direct_flux_m: 0.0,
            publication_flux_m: 0.0,
            direct_publication_delta_m: 0.0,
            diagnostic_residual_m: 0.0,
        }
    }
}

impl From<DirectWaterLedgerState> for DirectLedgerDownstreamOperands {
    fn from(state: DirectWaterLedgerState) -> Self {
        Self {
            total_accounted_input_m: state.total_accounted_input_m,
            soil_water_m: state.soil_water_m,
            available_water_m: state.available_water_m,
            direct_flux_m: state.direct_flux_m,
            publication_flux_m: state.publication_flux_m,
            direct_publication_delta_m: state.direct_publication_delta_m,
            diagnostic_residual_m: state.diagnostic_residual_m,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct DirectLedgerShadowProjection {
    pub lane_index: usize,
    pub day_index: usize,
    pub available_water_m: f64,
    pub direct_flux_m: f64,
    pub publication_flux_m: f64,
    pub direct_publication_delta_m: f64,
    pub diagnostic_residual_m: f64,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct DirectLaneTransferLedger {
    pub lane_id: u32,
    pub upstream_lane_id: u32,
    pub downstream_lane_id: u32,
    pub upstream_area_ratio: f64,
    pub area_m2: f64,
    pub outgoing_surface_m: f64,
    pub outgoing_lateral_m: f64,
    pub received_surface_m: f64,
    pub received_lateral_m: f64,
    pub net_transfer_m: f64,
}

impl DirectLaneTransferLedger {
    #[must_use]
    pub const fn zero() -> Self {
        Self {
            lane_id: 0,
            upstream_lane_id: 0,
            downstream_lane_id: 0,
            upstream_area_ratio: 0.0,
            area_m2: 0.0,
            outgoing_surface_m: 0.0,
            outgoing_lateral_m: 0.0,
            received_surface_m: 0.0,
            received_lateral_m: 0.0,
            net_transfer_m: 0.0,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct DirectRunTransferDownstreamOperands {
    pub lane_count: usize,
    pub outlet_lane_id: u32,
    pub total_outgoing_surface_m: f64,
    pub total_outgoing_lateral_m: f64,
    pub total_received_surface_m: f64,
    pub total_received_lateral_m: f64,
    pub total_net_transfer_m: f64,
}

impl DirectRunTransferDownstreamOperands {
    #[must_use]
    pub const fn zero() -> Self {
        Self {
            lane_count: 0,
            outlet_lane_id: 0,
            total_outgoing_surface_m: 0.0,
            total_outgoing_lateral_m: 0.0,
            total_received_surface_m: 0.0,
            total_received_lateral_m: 0.0,
            total_net_transfer_m: 0.0,
        }
    }
}

impl From<DirectRunTransferShadowProjection> for DirectRunTransferDownstreamOperands {
    fn from(projection: DirectRunTransferShadowProjection) -> Self {
        Self {
            lane_count: projection.lane_count,
            outlet_lane_id: projection.outlet_lane_id,
            total_outgoing_surface_m: projection.total_outgoing_surface_m,
            total_outgoing_lateral_m: projection.total_outgoing_lateral_m,
            total_received_surface_m: projection.total_received_surface_m,
            total_received_lateral_m: projection.total_received_lateral_m,
            total_net_transfer_m: projection.total_net_transfer_m,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct DirectRunTransferShadowProjection {
    pub lane_count: usize,
    pub outlet_lane_id: u32,
    pub total_outgoing_surface_m: f64,
    pub total_outgoing_lateral_m: f64,
    pub total_received_surface_m: f64,
    pub total_received_lateral_m: f64,
    pub total_net_transfer_m: f64,
}

impl DirectRunTransferShadowProjection {
    fn from_ledger(
        ledger: &[DirectLaneTransferLedger],
        outlet_lane_id: u32,
    ) -> Result<Self, DirectRuntimeError> {
        let mut total_outgoing_surface_m = 0.0;
        let mut total_outgoing_lateral_m = 0.0;
        let mut total_received_surface_m = 0.0;
        let mut total_received_lateral_m = 0.0;
        let mut total_net_transfer_m = 0.0;

        for lane in ledger {
            total_outgoing_surface_m += lane.outgoing_surface_m;
            validate_finite(
                "lane_transfer.total_outgoing_surface_m",
                total_outgoing_surface_m,
            )?;
            total_outgoing_lateral_m += lane.outgoing_lateral_m;
            validate_finite(
                "lane_transfer.total_outgoing_lateral_m",
                total_outgoing_lateral_m,
            )?;
            total_received_surface_m += lane.received_surface_m;
            validate_finite(
                "lane_transfer.total_received_surface_m",
                total_received_surface_m,
            )?;
            total_received_lateral_m += lane.received_lateral_m;
            validate_finite(
                "lane_transfer.total_received_lateral_m",
                total_received_lateral_m,
            )?;
            total_net_transfer_m += lane.net_transfer_m;
            validate_finite("lane_transfer.total_net_transfer_m", total_net_transfer_m)?;
        }

        Ok(Self {
            lane_count: ledger.len(),
            outlet_lane_id,
            total_outgoing_surface_m,
            total_outgoing_lateral_m,
            total_received_surface_m,
            total_received_lateral_m,
            total_net_transfer_m,
        })
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct DirectPhaseSpanReport {
    pub phase_count: usize,
    pub phase_entry_count: u64,
    pub direct_compute_count: u64,
    pub state_mutation_count: u64,
    pub downstream_operand_count: u64,
    pub shadow_projection_count: u64,
    pub compatibility_edge_invocation_count: u64,
    pub shadow_projection: DirectShadowProjection,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct DirectLedgerSpanReport {
    pub phase_count: usize,
    pub phase_entry_count: u64,
    pub direct_compute_count: u64,
    pub state_mutation_count: u64,
    pub downstream_operand_count: u64,
    pub shadow_projection_count: u64,
    pub compatibility_edge_invocation_count: u64,
    pub ledger_shadow_projection: DirectLedgerShadowProjection,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct DirectRunTransferSpanReport {
    pub phase_count: usize,
    pub phase_entry_count: u64,
    pub direct_compute_count: u64,
    pub state_mutation_count: u64,
    pub downstream_operand_count: u64,
    pub shadow_projection_count: u64,
    pub compatibility_edge_invocation_count: u64,
    pub transfer_shadow_projection: DirectRunTransferShadowProjection,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DirectExecutionReport {
    pub mode: DirectExecutorMode,
    pub lane_count: usize,
    pub day_count: usize,
    pub planned_phase_count: usize,
    pub canonical_phase_entry_count: u64,
    pub phase_view_count: u64,
    pub phase_status_counts: Vec<DirectPhaseStatusCount>,
    pub phase_span_run_count: u64,
    pub direct_phase_entry_count: u64,
    pub direct_compute_count: u64,
    pub state_mutation_count: u64,
    pub downstream_operand_count: u64,
    pub shadow_projection_count: u64,
    pub compatibility_edge_invocation_count: u64,
    pub day_frame_commit_count: u64,
}

#[derive(Debug, Clone, PartialEq)]
pub struct DirectPublicationExecution {
    pub report: DirectExecutionReport,
    pub publication_frame: DirectRunPublicationFrame,
}

#[derive(Debug, Clone, PartialEq)]
pub struct DirectStreamingPublicationExecution {
    pub report: DirectExecutionReport,
    pub identity: DirectRunIdentity,
    pub metadata: DirectPublicationRunMetadata,
    pub row_count: usize,
}
