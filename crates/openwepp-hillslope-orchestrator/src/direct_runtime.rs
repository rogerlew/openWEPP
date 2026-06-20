use std::error::Error;
use std::fmt;
use std::sync::atomic::{AtomicU64, Ordering};

pub const DIRECT_TRANSFER_HOUR_COUNT: usize = 24;
pub const DIRECT_PHASE_COUNT: usize = 14;
pub const DIRECT_R3A_PHASE_SPAN_COUNT: usize = 2;
pub const DIRECT_R3A_INPUT_ACCOUNTING_SPAN: [DirectPhaseKind; DIRECT_R3A_PHASE_SPAN_COUNT] = [
    DirectPhaseKind::Normalization,
    DirectPhaseKind::LateralTransfer,
];
pub const DIRECT_R3B_PHASE_SPAN_COUNT: usize = 3;
pub const DIRECT_R3B_WATER_LEDGER_SPAN: [DirectPhaseKind; DIRECT_R3B_PHASE_SPAN_COUNT] = [
    DirectPhaseKind::RunoffReconciliation,
    DirectPhaseKind::StorageReconciliation,
    DirectPhaseKind::ClosureDiagnostics,
];
pub const DIRECT_R3C_PHASE_SPAN_COUNT: usize = 3;
pub const DIRECT_R3C_LANE_TRANSFER_SPAN: [DirectPhaseKind; DIRECT_R3C_PHASE_SPAN_COUNT] = [
    DirectPhaseKind::LateralTransfer,
    DirectPhaseKind::RunoffReconciliation,
    DirectPhaseKind::ClosureDiagnostics,
];
pub const DIRECT_R4A_PHASE_SPAN_COUNT: usize = 3;
pub const DIRECT_R4A_RUNOFF_PARTITION_SPAN: [DirectPhaseKind; DIRECT_R4A_PHASE_SPAN_COUNT] = [
    DirectPhaseKind::RunoffReconciliation,
    DirectPhaseKind::StorageReconciliation,
    DirectPhaseKind::ClosureDiagnostics,
];
pub const DIRECT_R4B_PHASE_SPAN_COUNT: usize = 2;
pub const DIRECT_R4B_STORAGE_RECONCILIATION_SPAN: [DirectPhaseKind; DIRECT_R4B_PHASE_SPAN_COUNT] = [
    DirectPhaseKind::StorageReconciliation,
    DirectPhaseKind::ClosureDiagnostics,
];

static DIRECT_AUDIT: DirectRuntimeAuditCounters = DirectRuntimeAuditCounters::new();

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub enum DirectExecutorMode {
    #[default]
    Noop,
    ShadowOnly,
}

impl DirectExecutorMode {
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Noop => "noop",
            Self::ShadowOnly => "shadow-only",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DirectPhaseKind {
    Normalization,
    StorageBounds,
    DecompositionTransition,
    ResiduePartitionTransition,
    AnnualGrowthTransition,
    PerennialGrowthTransition,
    PercolationDeepSeepage,
    Evapotranspiration,
    Drainage,
    LateralTransfer,
    PlantRootUptake,
    RunoffReconciliation,
    StorageReconciliation,
    ClosureDiagnostics,
}

impl DirectPhaseKind {
    pub const ORDERED: [Self; DIRECT_PHASE_COUNT] = [
        Self::Normalization,
        Self::StorageBounds,
        Self::DecompositionTransition,
        Self::ResiduePartitionTransition,
        Self::AnnualGrowthTransition,
        Self::PerennialGrowthTransition,
        Self::PercolationDeepSeepage,
        Self::Evapotranspiration,
        Self::Drainage,
        Self::LateralTransfer,
        Self::PlantRootUptake,
        Self::RunoffReconciliation,
        Self::StorageReconciliation,
        Self::ClosureDiagnostics,
    ];
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct DirectRunIdentity {
    pub run_id: u64,
    pub hillslope_id: u32,
    pub lane_count: usize,
    pub day_count: usize,
}

impl DirectRunIdentity {
    pub fn new(
        run_id: u64,
        hillslope_id: u32,
        lane_count: usize,
        day_count: usize,
    ) -> Result<Self, DirectRuntimeError> {
        if lane_count == 0 {
            return Err(DirectRuntimeError::InvalidLaneCount { lane_count });
        }
        if day_count == 0 {
            return Err(DirectRuntimeError::InvalidDayCount { day_count });
        }

        Ok(Self {
            run_id,
            hillslope_id,
            lane_count,
            day_count,
        })
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct DirectRunFrame {
    pub identity: DirectRunIdentity,
    pub lanes: Vec<DirectLaneFrame>,
    pub phase_plan: DirectPhasePlan,
    pub publication: DirectPublicationFrame,
    pub lane_transfer_ledger: Vec<DirectLaneTransferLedger>,
    pub lane_transfer_downstream_operands: DirectRunTransferDownstreamOperands,
    pub lane_transfer_shadow_projection: Option<DirectRunTransferShadowProjection>,
}

impl DirectRunFrame {
    pub fn skeleton(identity: DirectRunIdentity) -> Result<Self, DirectRuntimeError> {
        DIRECT_AUDIT.record_run_frame_construction();
        let lanes = (0..identity.lane_count)
            .map(|lane_index| DirectLaneFrame::skeleton(lane_index, identity.lane_count))
            .collect::<Result<Vec<_>, _>>()?;

        Ok(Self {
            identity,
            lanes,
            phase_plan: DirectPhasePlan::default(),
            publication: DirectPublicationFrame::empty(),
            lane_transfer_ledger: vec![DirectLaneTransferLedger::zero(); identity.lane_count],
            lane_transfer_downstream_operands: DirectRunTransferDownstreamOperands::zero(),
            lane_transfer_shadow_projection: None,
        })
    }

    pub fn run_r3c_lane_transfer_span(
        &mut self,
    ) -> Result<DirectRunTransferSpanReport, DirectRuntimeError> {
        DIRECT_AUDIT.record_phase_span_run();
        let phase_count = DIRECT_R3C_PHASE_SPAN_COUNT;
        let mut phase_entry_count = 0_u64;
        let mut direct_compute_count = 0_u64;
        let mut state_mutation_count = 0_u64;
        let mut downstream_operand_count = 0_u64;
        let mut shadow_projection_count = 0_u64;

        DIRECT_AUDIT.record_direct_phase_entry();
        phase_entry_count += 1;
        let (ledger, transfer_shadow_projection) = self.compute_r3c_lane_transfer_ledger()?;
        DIRECT_AUDIT.record_direct_compute_operation();
        direct_compute_count += 1;

        DIRECT_AUDIT.record_direct_phase_entry();
        phase_entry_count += 1;
        self.lane_transfer_ledger = ledger;
        DIRECT_AUDIT.record_direct_state_mutation();
        state_mutation_count += 1;

        DIRECT_AUDIT.record_direct_phase_entry();
        phase_entry_count += 1;
        self.lane_transfer_downstream_operands =
            DirectRunTransferDownstreamOperands::from(transfer_shadow_projection);
        DIRECT_AUDIT.record_downstream_operand_production();
        downstream_operand_count += 1;

        self.lane_transfer_shadow_projection = Some(transfer_shadow_projection);
        DIRECT_AUDIT.record_shadow_projection();
        shadow_projection_count += 1;

        Ok(DirectRunTransferSpanReport {
            phase_count,
            phase_entry_count,
            direct_compute_count,
            state_mutation_count,
            downstream_operand_count,
            shadow_projection_count,
            compatibility_edge_invocation_count: 0,
            transfer_shadow_projection,
        })
    }

    fn compute_r3c_lane_transfer_ledger(
        &self,
    ) -> Result<
        (
            Vec<DirectLaneTransferLedger>,
            DirectRunTransferShadowProjection,
        ),
        DirectRuntimeError,
    > {
        let outlet_lane_id = self.validate_r3c_lane_transfer_domain()?;
        let outgoing = self
            .lanes
            .iter()
            .map(|lane| {
                Ok((
                    sum_nonnegative_direct_m(
                        "transfer.surface_carry_m",
                        &lane.transfer.surface_carry_m,
                    )?,
                    sum_nonnegative_direct_m(
                        "transfer.lateral_carry_m",
                        &lane.transfer.lateral_carry_m,
                    )?,
                ))
            })
            .collect::<Result<Vec<_>, DirectRuntimeError>>()?;

        let mut ledger = Vec::with_capacity(self.lanes.len());
        for (lane_index, lane) in self.lanes.iter().enumerate() {
            let (outgoing_surface_m, outgoing_lateral_m) = outgoing[lane_index];
            let (received_surface_m, received_lateral_m) = if lane.upstream_lane_id == 0 {
                (0.0, 0.0)
            } else {
                let upstream_index = (lane.upstream_lane_id - 1) as usize;
                let received_surface_m = outgoing[upstream_index].0 * lane.upstream_area_ratio;
                validate_finite("lane_transfer.received_surface_m", received_surface_m)?;
                let received_lateral_m = outgoing[upstream_index].1 * lane.upstream_area_ratio;
                validate_finite("lane_transfer.received_lateral_m", received_lateral_m)?;
                (received_surface_m, received_lateral_m)
            };
            let net_transfer_m =
                received_surface_m + received_lateral_m - outgoing_surface_m - outgoing_lateral_m;
            validate_finite("lane_transfer.net_transfer_m", net_transfer_m)?;

            ledger.push(DirectLaneTransferLedger {
                lane_id: lane.lane_id,
                upstream_lane_id: lane.upstream_lane_id,
                downstream_lane_id: lane.downstream_lane_id,
                upstream_area_ratio: lane.upstream_area_ratio,
                area_m2: lane.area_m2,
                outgoing_surface_m,
                outgoing_lateral_m,
                received_surface_m,
                received_lateral_m,
                net_transfer_m,
            });
        }

        let transfer_shadow_projection =
            DirectRunTransferShadowProjection::from_ledger(&ledger, outlet_lane_id)?;
        Ok((ledger, transfer_shadow_projection))
    }

    fn validate_r3c_lane_transfer_domain(&self) -> Result<u32, DirectRuntimeError> {
        if self.lanes.len() != self.identity.lane_count {
            return Err(DirectRuntimeError::FrameLaneCountMismatch {
                identity_lane_count: self.identity.lane_count,
                actual_lane_count: self.lanes.len(),
            });
        }
        let lane_count_u32 =
            u32::try_from(self.lanes.len()).map_err(|_| DirectRuntimeError::LaneIdOverflow {
                lane_index: self.lanes.len(),
            })?;
        let mut outlet_lane_id = 0_u32;
        let mut outlet_count = 0_usize;

        for (lane_index, lane) in self.lanes.iter().enumerate() {
            let expected_lane_id = u32::try_from(lane_index + 1)
                .map_err(|_| DirectRuntimeError::LaneIdOverflow { lane_index })?;
            if lane.lane_id != expected_lane_id
                || lane.upstream_lane_id > lane_count_u32
                || lane.downstream_lane_id > lane_count_u32
            {
                return Err(DirectRuntimeError::InvalidLaneTopology {
                    lane_index,
                    lane_id: lane.lane_id,
                    upstream_lane_id: lane.upstream_lane_id,
                    downstream_lane_id: lane.downstream_lane_id,
                });
            }
            validate_nonnegative_direct_m("lane.upstream_area_ratio", lane.upstream_area_ratio)?;
            validate_nonnegative_direct_m("lane.area_m2", lane.area_m2)?;
            if lane.downstream_lane_id == 0 {
                outlet_count += 1;
                outlet_lane_id = lane.lane_id;
            }
            if lane.upstream_lane_id != 0 {
                let upstream_index = (lane.upstream_lane_id - 1) as usize;
                if self.lanes[upstream_index].downstream_lane_id != lane.lane_id {
                    return Err(DirectRuntimeError::InvalidLaneTopology {
                        lane_index,
                        lane_id: lane.lane_id,
                        upstream_lane_id: lane.upstream_lane_id,
                        downstream_lane_id: lane.downstream_lane_id,
                    });
                }
            }
            if lane.downstream_lane_id != 0 {
                let downstream_index = (lane.downstream_lane_id - 1) as usize;
                if self.lanes[downstream_index].upstream_lane_id != lane.lane_id {
                    return Err(DirectRuntimeError::InvalidLaneTopology {
                        lane_index,
                        lane_id: lane.lane_id,
                        upstream_lane_id: lane.upstream_lane_id,
                        downstream_lane_id: lane.downstream_lane_id,
                    });
                }
            }
        }

        if outlet_count == 1 {
            Ok(outlet_lane_id)
        } else {
            Err(DirectRuntimeError::InvalidLaneOutletCount { outlet_count })
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct DirectLaneFrame {
    pub lane_id: u32,
    pub upstream_lane_id: u32,
    pub downstream_lane_id: u32,
    pub upstream_area_ratio: f64,
    pub area_m2: f64,
    pub water: DirectWaterState,
    pub transfer: DirectTransferBuffers,
}

impl DirectLaneFrame {
    fn skeleton(lane_index: usize, lane_count: usize) -> Result<Self, DirectRuntimeError> {
        let lane_id = u32::try_from(lane_index + 1)
            .map_err(|_| DirectRuntimeError::LaneIdOverflow { lane_index })?;
        let upstream_lane_id = lane_id.saturating_sub(1);
        let downstream_lane_id = if lane_index + 1 == lane_count {
            0
        } else {
            lane_id + 1
        };

        Ok(Self {
            lane_id,
            upstream_lane_id,
            downstream_lane_id,
            upstream_area_ratio: 1.0,
            area_m2: 0.0,
            water: DirectWaterState::zero(),
            transfer: DirectTransferBuffers::zero(),
        })
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct DirectDayFrame {
    pub identity: DirectRunIdentity,
    pub lane_index: usize,
    pub day_index: usize,
    pub forcing: DirectDayForcing,
    pub water: DirectWaterState,
    pub transfer: DirectTransferBuffers,
    pub publication: DirectPublicationFrame,
    pub input_accounting: DirectInputAccountingState,
    pub downstream_operands: DirectDownstreamOperands,
    pub shadow_projection: Option<DirectShadowProjection>,
    pub runoff_partition_inputs: DirectRunoffPartitionInputs,
    pub runoff_partition: DirectRunoffPartitionState,
    pub runoff_downstream_operands: DirectRunoffDownstreamOperands,
    pub runoff_shadow_projection: Option<DirectRunoffShadowProjection>,
    pub storage_reconciliation_inputs: DirectStorageReconciliationInputs,
    pub storage_reconciliation: DirectStorageReconciliationState,
    pub storage_downstream_operands: DirectStorageDownstreamOperands,
    pub storage_shadow_projection: Option<DirectStorageShadowProjection>,
    pub water_ledger: DirectWaterLedgerState,
    pub ledger_downstream_operands: DirectLedgerDownstreamOperands,
    pub ledger_shadow_projection: Option<DirectLedgerShadowProjection>,
}

impl DirectDayFrame {
    pub fn seed(
        identity: DirectRunIdentity,
        lane_index: usize,
        day_index: usize,
    ) -> Result<Self, DirectRuntimeError> {
        if lane_index >= identity.lane_count {
            return Err(DirectRuntimeError::LaneIndexOutOfRange {
                lane_index,
                lane_count: identity.lane_count,
            });
        }
        if day_index >= identity.day_count {
            return Err(DirectRuntimeError::DayIndexOutOfRange {
                day_index,
                day_count: identity.day_count,
            });
        }

        DIRECT_AUDIT.record_day_frame_construction();

        Ok(Self {
            identity,
            lane_index,
            day_index,
            forcing: DirectDayForcing::zero(),
            water: DirectWaterState::zero(),
            transfer: DirectTransferBuffers::zero(),
            publication: DirectPublicationFrame::empty(),
            input_accounting: DirectInputAccountingState::zero(),
            downstream_operands: DirectDownstreamOperands::zero(),
            shadow_projection: None,
            runoff_partition_inputs: DirectRunoffPartitionInputs::zero(),
            runoff_partition: DirectRunoffPartitionState::zero(),
            runoff_downstream_operands: DirectRunoffDownstreamOperands::zero(),
            runoff_shadow_projection: None,
            storage_reconciliation_inputs: DirectStorageReconciliationInputs::zero(),
            storage_reconciliation: DirectStorageReconciliationState::zero(),
            storage_downstream_operands: DirectStorageDownstreamOperands::zero(),
            storage_shadow_projection: None,
            water_ledger: DirectWaterLedgerState::zero(),
            ledger_downstream_operands: DirectLedgerDownstreamOperands::zero(),
            ledger_shadow_projection: None,
        })
    }

    pub fn phase_view(&mut self, phase: DirectPhaseKind) -> DirectPhaseView<'_> {
        DIRECT_AUDIT.record_phase_view_construction();
        DirectPhaseView {
            phase,
            water: &mut self.water,
            transfer: &mut self.transfer,
            publication: &mut self.publication,
        }
    }

    pub fn run_r3a_input_accounting_span(
        &mut self,
    ) -> Result<DirectPhaseSpanReport, DirectRuntimeError> {
        DIRECT_AUDIT.record_phase_span_run();
        let phase_count = DIRECT_R3A_PHASE_SPAN_COUNT;
        let mut phase_entry_count = 0_u64;
        let mut direct_compute_count = 0_u64;
        let mut state_mutation_count = 0_u64;
        let mut downstream_operand_count = 0_u64;
        let mut shadow_projection_count = 0_u64;

        DIRECT_AUDIT.record_direct_phase_entry();
        phase_entry_count += 1;
        self.validate_r3a_input_accounting_domain()?;

        let surface_transfer_m =
            sum_nonnegative_direct_m("transfer.surface_carry_m", &self.transfer.surface_carry_m)?;
        let lateral_transfer_m =
            sum_nonnegative_direct_m("transfer.lateral_carry_m", &self.transfer.lateral_carry_m)?;
        let transfer_input_m = surface_transfer_m
            + lateral_transfer_m
            + self.transfer.upstream_flow_m
            + self.transfer.subsurface_input_m;
        validate_finite("input_accounting.transfer_input_m", transfer_input_m)?;
        let total_accounted_input_m = self.forcing.precipitation_m + transfer_input_m;
        validate_finite(
            "input_accounting.total_accounted_input_m",
            total_accounted_input_m,
        )?;
        DIRECT_AUDIT.record_direct_compute_operation();
        direct_compute_count += 1;

        self.input_accounting = DirectInputAccountingState {
            precipitation_m: self.forcing.precipitation_m,
            surface_transfer_m,
            lateral_transfer_m,
            upstream_flow_m: self.transfer.upstream_flow_m,
            subsurface_input_m: self.transfer.subsurface_input_m,
            transfer_input_m,
            total_accounted_input_m,
        };
        DIRECT_AUDIT.record_direct_state_mutation();
        state_mutation_count += 1;

        DIRECT_AUDIT.record_direct_phase_entry();
        phase_entry_count += 1;
        self.downstream_operands = DirectDownstreamOperands::from(self.input_accounting);
        DIRECT_AUDIT.record_downstream_operand_production();
        downstream_operand_count += 1;

        let shadow_projection = DirectShadowProjection {
            lane_index: self.lane_index,
            day_index: self.day_index,
            precipitation_m: self.downstream_operands.precipitation_m,
            transfer_input_m: self.downstream_operands.transfer_input_m,
            total_accounted_input_m: self.downstream_operands.total_accounted_input_m,
        };
        self.shadow_projection = Some(shadow_projection);
        DIRECT_AUDIT.record_shadow_projection();
        shadow_projection_count += 1;

        Ok(DirectPhaseSpanReport {
            phase_count,
            phase_entry_count,
            direct_compute_count,
            state_mutation_count,
            downstream_operand_count,
            shadow_projection_count,
            compatibility_edge_invocation_count: 0,
            shadow_projection,
        })
    }

    pub fn run_r4a_runoff_partition_span(
        &mut self,
    ) -> Result<DirectRunoffPartitionSpanReport, DirectRuntimeError> {
        DIRECT_AUDIT.record_phase_span_run();
        let phase_count = DIRECT_R4A_PHASE_SPAN_COUNT;
        let mut phase_entry_count = 0_u64;
        let mut direct_compute_count = 0_u64;
        let mut state_mutation_count = 0_u64;
        let mut downstream_operand_count = 0_u64;
        let mut shadow_projection_count = 0_u64;

        DIRECT_AUDIT.record_direct_phase_entry();
        phase_entry_count += 1;
        let runoff_partition = self.compute_r4a_runoff_partition()?;
        DIRECT_AUDIT.record_direct_compute_operation();
        direct_compute_count += 1;

        DIRECT_AUDIT.record_direct_phase_entry();
        phase_entry_count += 1;
        self.runoff_partition = runoff_partition;
        self.water.infiltration_m = runoff_partition.cumulative_infiltration_m;
        self.water.runoff_m = runoff_partition.q_runoff_m;
        DIRECT_AUDIT.record_direct_state_mutation();
        state_mutation_count += 1;

        DIRECT_AUDIT.record_direct_phase_entry();
        phase_entry_count += 1;
        self.runoff_downstream_operands = DirectRunoffDownstreamOperands::from(runoff_partition);
        DIRECT_AUDIT.record_downstream_operand_production();
        downstream_operand_count += 1;

        let runoff_shadow_projection = DirectRunoffShadowProjection {
            lane_index: self.lane_index,
            day_index: self.day_index,
            liquid_input_m: self.runoff_downstream_operands.liquid_input_m,
            runon_input_m: self.runoff_downstream_operands.runon_input_m,
            cumulative_infiltration_m: self.runoff_downstream_operands.cumulative_infiltration_m,
            depression_storage_delta_m: self.runoff_downstream_operands.depression_storage_delta_m,
            surface_saturation_runoff_m: self
                .runoff_downstream_operands
                .surface_saturation_runoff_m,
            partition_runoff_m: self.runoff_downstream_operands.partition_runoff_m,
            q_runoff_m: self.runoff_downstream_operands.q_runoff_m,
            closure_residual_m: self.runoff_downstream_operands.closure_residual_m,
        };
        self.runoff_shadow_projection = Some(runoff_shadow_projection);
        DIRECT_AUDIT.record_shadow_projection();
        shadow_projection_count += 1;

        Ok(DirectRunoffPartitionSpanReport {
            phase_count,
            phase_entry_count,
            direct_compute_count,
            state_mutation_count,
            downstream_operand_count,
            shadow_projection_count,
            compatibility_edge_invocation_count: 0,
            runoff_shadow_projection,
        })
    }

    pub fn run_r3b_water_ledger_span(
        &mut self,
    ) -> Result<DirectLedgerSpanReport, DirectRuntimeError> {
        DIRECT_AUDIT.record_phase_span_run();
        let phase_count = DIRECT_R3B_PHASE_SPAN_COUNT;
        let mut phase_entry_count = 0_u64;
        let mut direct_compute_count = 0_u64;
        let mut state_mutation_count = 0_u64;
        let mut downstream_operand_count = 0_u64;
        let mut shadow_projection_count = 0_u64;

        DIRECT_AUDIT.record_direct_phase_entry();
        phase_entry_count += 1;
        self.validate_r3b_water_ledger_domain()?;

        let direct_flux_m = sum_finite_direct_m(
            "water_ledger.direct_flux_m",
            &[
                self.water.infiltration_m,
                self.water.runoff_m,
                self.water.evapotranspiration_m,
                self.water.drainage_m,
                self.water.lateral_flow_m,
            ],
        )?;
        let publication_flux_m = sum_finite_direct_m(
            "water_ledger.publication_flux_m",
            &[
                self.publication.infiltration_m,
                self.publication.runoff_m,
                self.publication.evapotranspiration_m,
                self.publication.drainage_m,
                self.publication.lateral_flow_m,
            ],
        )?;
        let available_water_m =
            self.input_accounting.total_accounted_input_m + self.water.soil_water_m;
        validate_finite("water_ledger.available_water_m", available_water_m)?;
        let direct_publication_delta_m = direct_flux_m - publication_flux_m;
        validate_finite(
            "water_ledger.direct_publication_delta_m",
            direct_publication_delta_m,
        )?;
        let diagnostic_residual_m = available_water_m - direct_flux_m;
        validate_finite("water_ledger.diagnostic_residual_m", diagnostic_residual_m)?;
        DIRECT_AUDIT.record_direct_compute_operation();
        direct_compute_count += 1;

        DIRECT_AUDIT.record_direct_phase_entry();
        phase_entry_count += 1;
        self.water_ledger = DirectWaterLedgerState {
            total_accounted_input_m: self.input_accounting.total_accounted_input_m,
            soil_water_m: self.water.soil_water_m,
            available_water_m,
            direct_flux_m,
            publication_flux_m,
            direct_publication_delta_m,
            diagnostic_residual_m,
        };
        DIRECT_AUDIT.record_direct_state_mutation();
        state_mutation_count += 1;

        DIRECT_AUDIT.record_direct_phase_entry();
        phase_entry_count += 1;
        self.ledger_downstream_operands = DirectLedgerDownstreamOperands::from(self.water_ledger);
        DIRECT_AUDIT.record_downstream_operand_production();
        downstream_operand_count += 1;

        let ledger_shadow_projection = DirectLedgerShadowProjection {
            lane_index: self.lane_index,
            day_index: self.day_index,
            available_water_m: self.ledger_downstream_operands.available_water_m,
            direct_flux_m: self.ledger_downstream_operands.direct_flux_m,
            publication_flux_m: self.ledger_downstream_operands.publication_flux_m,
            direct_publication_delta_m: self.ledger_downstream_operands.direct_publication_delta_m,
            diagnostic_residual_m: self.ledger_downstream_operands.diagnostic_residual_m,
        };
        self.ledger_shadow_projection = Some(ledger_shadow_projection);
        DIRECT_AUDIT.record_shadow_projection();
        shadow_projection_count += 1;

        Ok(DirectLedgerSpanReport {
            phase_count,
            phase_entry_count,
            direct_compute_count,
            state_mutation_count,
            downstream_operand_count,
            shadow_projection_count,
            compatibility_edge_invocation_count: 0,
            ledger_shadow_projection,
        })
    }

    pub fn run_r4b_storage_reconciliation_span(
        &mut self,
    ) -> Result<DirectStorageReconciliationSpanReport, DirectRuntimeError> {
        DIRECT_AUDIT.record_phase_span_run();
        let phase_count = DIRECT_R4B_PHASE_SPAN_COUNT;
        let mut phase_entry_count = 0_u64;

        DIRECT_AUDIT.record_direct_phase_entry();
        phase_entry_count += 1;
        let storage_reconciliation = self.compute_r4b_storage_reconciliation()?;
        DIRECT_AUDIT.record_direct_compute_operation();

        DIRECT_AUDIT.record_direct_phase_entry();
        phase_entry_count += 1;
        self.storage_reconciliation = storage_reconciliation;
        self.water.soil_water_m = storage_reconciliation.storage_reconciled_m;
        DIRECT_AUDIT.record_direct_state_mutation();
        self.storage_downstream_operands =
            DirectStorageDownstreamOperands::from(storage_reconciliation);
        DIRECT_AUDIT.record_downstream_operand_production();

        let storage_shadow_projection = DirectStorageShadowProjection {
            lane_index: self.lane_index,
            day_index: self.day_index,
            storage_initial_m: self.storage_downstream_operands.storage_initial_m,
            precip_input_m: self.storage_downstream_operands.precip_input_m,
            snow_coupling_m: self.storage_downstream_operands.snow_coupling_m,
            q_runoff_m: self.storage_downstream_operands.q_runoff_m,
            evapotranspiration_m: self.storage_downstream_operands.evapotranspiration_m,
            deep_seepage_m: self.storage_downstream_operands.deep_seepage_m,
            subsurface_loss_m: self.storage_downstream_operands.subsurface_loss_m,
            storage_reconciled_m: self.storage_downstream_operands.storage_reconciled_m,
            closure_residual_m: self.storage_downstream_operands.closure_residual_m,
        };
        self.storage_shadow_projection = Some(storage_shadow_projection);
        DIRECT_AUDIT.record_shadow_projection();

        Ok(DirectStorageReconciliationSpanReport {
            phase_count,
            phase_entry_count,
            direct_compute_count: 1,
            state_mutation_count: 1,
            downstream_operand_count: 1,
            shadow_projection_count: 1,
            compatibility_edge_invocation_count: 0,
            storage_shadow_projection,
        })
    }

    fn compute_r4a_runoff_partition(
        &self,
    ) -> Result<DirectRunoffPartitionState, DirectRuntimeError> {
        self.validate_r4a_runoff_partition_domain()?;
        let inputs = self.runoff_partition_inputs;
        let liquid_and_runon_m = inputs.liquid_input_m + inputs.runon_input_m;
        validate_finite("runoff_partition.liquid_and_runon_m", liquid_and_runon_m)?;
        let retained_m = inputs.cumulative_infiltration_m + inputs.depression_storage_delta_m;
        validate_finite("runoff_partition.retained_m", retained_m)?;
        let partition_runoff_m = liquid_and_runon_m - retained_m;
        validate_finite("runoff_partition.partition_runoff_m", partition_runoff_m)?;
        validate_nonnegative_direct_m("runoff_partition.partition_runoff_m", partition_runoff_m)?;
        let q_runoff_m = partition_runoff_m + inputs.surface_saturation_runoff_m;
        validate_finite("runoff_partition.q_runoff_m", q_runoff_m)?;
        validate_nonnegative_direct_m("runoff_partition.q_runoff_m", q_runoff_m)?;
        let closure_residual_m =
            inputs.liquid_input_m + inputs.runon_input_m + inputs.surface_saturation_runoff_m
                - inputs.cumulative_infiltration_m
                - inputs.depression_storage_delta_m
                - q_runoff_m;
        validate_finite("runoff_partition.closure_residual_m", closure_residual_m)?;

        Ok(DirectRunoffPartitionState {
            liquid_input_m: inputs.liquid_input_m,
            runon_input_m: inputs.runon_input_m,
            cumulative_infiltration_m: inputs.cumulative_infiltration_m,
            depression_storage_delta_m: inputs.depression_storage_delta_m,
            surface_saturation_runoff_m: inputs.surface_saturation_runoff_m,
            partition_runoff_m,
            q_runoff_m,
            closure_residual_m,
        })
    }

    fn compute_r4b_storage_reconciliation(
        &self,
    ) -> Result<DirectStorageReconciliationState, DirectRuntimeError> {
        self.validate_r4b_storage_reconciliation_domain()?;
        let inputs = self.storage_reconciliation_inputs;
        let q_runoff_m = self.runoff_downstream_operands.q_runoff_m;
        let storage_reconciled_m =
            inputs.storage_initial_m + inputs.precip_input_m + inputs.snow_coupling_m
                - q_runoff_m
                - inputs.evapotranspiration_m
                - inputs.deep_seepage_m
                - inputs.subsurface_loss_m;
        validate_finite(
            "storage_reconciliation.storage_reconciled_m",
            storage_reconciled_m,
        )?;
        validate_nonnegative_direct_m(
            "storage_reconciliation.storage_reconciled_m",
            storage_reconciled_m,
        )?;
        let closure_residual_m =
            inputs.storage_initial_m + inputs.precip_input_m + inputs.snow_coupling_m
                - q_runoff_m
                - inputs.evapotranspiration_m
                - inputs.deep_seepage_m
                - inputs.subsurface_loss_m
                - storage_reconciled_m;
        validate_finite(
            "storage_reconciliation.closure_residual_m",
            closure_residual_m,
        )?;
        if closure_residual_m.abs() > inputs.closure_tolerance_m {
            return Err(DirectRuntimeError::DirectClosureToleranceExceeded {
                field: "storage_reconciliation.closure_residual_m",
            });
        }

        Ok(DirectStorageReconciliationState {
            storage_initial_m: inputs.storage_initial_m,
            precip_input_m: inputs.precip_input_m,
            snow_coupling_m: inputs.snow_coupling_m,
            q_runoff_m,
            evapotranspiration_m: inputs.evapotranspiration_m,
            deep_seepage_m: inputs.deep_seepage_m,
            subsurface_loss_m: inputs.subsurface_loss_m,
            closure_tolerance_m: inputs.closure_tolerance_m,
            storage_reconciled_m,
            closure_residual_m,
        })
    }

    fn validate_r3a_input_accounting_domain(&self) -> Result<(), DirectRuntimeError> {
        validate_nonnegative_direct_m("forcing.precipitation_m", self.forcing.precipitation_m)?;
        validate_finite(
            "forcing.effective_temperature_c",
            self.forcing.effective_temperature_c,
        )?;
        validate_nonnegative_direct_m("water.soil_water_m", self.water.soil_water_m)?;
        validate_nonnegative_direct_m("water.infiltration_m", self.water.infiltration_m)?;
        validate_nonnegative_direct_m("water.runoff_m", self.water.runoff_m)?;
        validate_nonnegative_direct_m(
            "water.evapotranspiration_m",
            self.water.evapotranspiration_m,
        )?;
        validate_nonnegative_direct_m("water.drainage_m", self.water.drainage_m)?;
        validate_nonnegative_direct_m("water.lateral_flow_m", self.water.lateral_flow_m)?;
        validate_nonnegative_direct_m("transfer.upstream_flow_m", self.transfer.upstream_flow_m)?;
        validate_nonnegative_direct_m(
            "transfer.subsurface_input_m",
            self.transfer.subsurface_input_m,
        )?;
        validate_nonnegative_direct_m("publication.runoff_m", self.publication.runoff_m)?;
        validate_nonnegative_direct_m(
            "publication.infiltration_m",
            self.publication.infiltration_m,
        )?;
        validate_nonnegative_direct_m(
            "publication.evapotranspiration_m",
            self.publication.evapotranspiration_m,
        )?;
        validate_nonnegative_direct_m("publication.drainage_m", self.publication.drainage_m)?;
        validate_nonnegative_direct_m(
            "publication.lateral_flow_m",
            self.publication.lateral_flow_m,
        )?;
        Ok(())
    }

    fn validate_r3b_water_ledger_domain(&self) -> Result<(), DirectRuntimeError> {
        validate_nonnegative_direct_m(
            "input_accounting.total_accounted_input_m",
            self.input_accounting.total_accounted_input_m,
        )?;
        validate_nonnegative_direct_m("water.soil_water_m", self.water.soil_water_m)?;
        validate_nonnegative_direct_m("water.infiltration_m", self.water.infiltration_m)?;
        validate_nonnegative_direct_m("water.runoff_m", self.water.runoff_m)?;
        validate_nonnegative_direct_m(
            "water.evapotranspiration_m",
            self.water.evapotranspiration_m,
        )?;
        validate_nonnegative_direct_m("water.drainage_m", self.water.drainage_m)?;
        validate_nonnegative_direct_m("water.lateral_flow_m", self.water.lateral_flow_m)?;
        validate_nonnegative_direct_m("publication.runoff_m", self.publication.runoff_m)?;
        validate_nonnegative_direct_m(
            "publication.infiltration_m",
            self.publication.infiltration_m,
        )?;
        validate_nonnegative_direct_m(
            "publication.evapotranspiration_m",
            self.publication.evapotranspiration_m,
        )?;
        validate_nonnegative_direct_m("publication.drainage_m", self.publication.drainage_m)?;
        validate_nonnegative_direct_m(
            "publication.lateral_flow_m",
            self.publication.lateral_flow_m,
        )?;
        Ok(())
    }

    fn validate_r4a_runoff_partition_domain(&self) -> Result<(), DirectRuntimeError> {
        validate_nonnegative_direct_m(
            "runoff_partition.liquid_input_m",
            self.runoff_partition_inputs.liquid_input_m,
        )?;
        validate_nonnegative_direct_m(
            "runoff_partition.runon_input_m",
            self.runoff_partition_inputs.runon_input_m,
        )?;
        validate_nonnegative_direct_m(
            "runoff_partition.cumulative_infiltration_m",
            self.runoff_partition_inputs.cumulative_infiltration_m,
        )?;
        validate_nonnegative_direct_m(
            "runoff_partition.depression_storage_delta_m",
            self.runoff_partition_inputs.depression_storage_delta_m,
        )?;
        validate_nonnegative_direct_m(
            "runoff_partition.surface_saturation_runoff_m",
            self.runoff_partition_inputs.surface_saturation_runoff_m,
        )?;
        Ok(())
    }

    fn validate_r4b_storage_reconciliation_domain(&self) -> Result<(), DirectRuntimeError> {
        if self.runoff_shadow_projection.is_none() {
            return Err(DirectRuntimeError::MissingDirectUpstream {
                upstream: "R4A runoff partition",
            });
        }
        validate_nonnegative_direct_m(
            "storage_reconciliation.storage_initial_m",
            self.storage_reconciliation_inputs.storage_initial_m,
        )?;
        validate_nonnegative_direct_m(
            "storage_reconciliation.precip_input_m",
            self.storage_reconciliation_inputs.precip_input_m,
        )?;
        validate_finite(
            "storage_reconciliation.snow_coupling_m",
            self.storage_reconciliation_inputs.snow_coupling_m,
        )?;
        validate_nonnegative_direct_m(
            "storage_reconciliation.q_runoff_m",
            self.runoff_downstream_operands.q_runoff_m,
        )?;
        validate_nonnegative_direct_m(
            "storage_reconciliation.evapotranspiration_m",
            self.storage_reconciliation_inputs.evapotranspiration_m,
        )?;
        validate_nonnegative_direct_m(
            "storage_reconciliation.deep_seepage_m",
            self.storage_reconciliation_inputs.deep_seepage_m,
        )?;
        validate_nonnegative_direct_m(
            "storage_reconciliation.subsurface_loss_m",
            self.storage_reconciliation_inputs.subsurface_loss_m,
        )?;
        validate_nonnegative_direct_m(
            "storage_reconciliation.closure_tolerance_m",
            self.storage_reconciliation_inputs.closure_tolerance_m,
        )?;
        Ok(())
    }
}

#[derive(Debug, PartialEq)]
pub struct DirectPhaseView<'day> {
    phase: DirectPhaseKind,
    water: &'day mut DirectWaterState,
    transfer: &'day mut DirectTransferBuffers,
    publication: &'day mut DirectPublicationFrame,
}

impl DirectPhaseView<'_> {
    #[must_use]
    pub const fn phase(&self) -> DirectPhaseKind {
        self.phase
    }

    #[must_use]
    pub fn water_state(&self) -> &DirectWaterState {
        self.water
    }

    #[must_use]
    pub fn transfer_buffers(&self) -> &DirectTransferBuffers {
        self.transfer
    }

    #[must_use]
    pub fn publication_frame(&self) -> &DirectPublicationFrame {
        self.publication
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct DirectPublicationFrame {
    pub runoff_m: f64,
    pub infiltration_m: f64,
    pub evapotranspiration_m: f64,
    pub drainage_m: f64,
    pub lateral_flow_m: f64,
}

impl DirectPublicationFrame {
    #[must_use]
    pub const fn empty() -> Self {
        Self {
            runoff_m: 0.0,
            infiltration_m: 0.0,
            evapotranspiration_m: 0.0,
            drainage_m: 0.0,
            lateral_flow_m: 0.0,
        }
    }
}

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
    pub lateral_carry_m: [f64; DIRECT_TRANSFER_HOUR_COUNT],
    pub upstream_flow_m: f64,
    pub subsurface_input_m: f64,
}

impl DirectTransferBuffers {
    #[must_use]
    pub const fn zero() -> Self {
        Self {
            surface_carry_m: [0.0; DIRECT_TRANSFER_HOUR_COUNT],
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
pub struct DirectRunoffPartitionInputs {
    pub liquid_input_m: f64,
    pub runon_input_m: f64,
    pub cumulative_infiltration_m: f64,
    pub depression_storage_delta_m: f64,
    pub surface_saturation_runoff_m: f64,
}

impl DirectRunoffPartitionInputs {
    #[must_use]
    pub const fn zero() -> Self {
        Self {
            liquid_input_m: 0.0,
            runon_input_m: 0.0,
            cumulative_infiltration_m: 0.0,
            depression_storage_delta_m: 0.0,
            surface_saturation_runoff_m: 0.0,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct DirectRunoffPartitionState {
    pub liquid_input_m: f64,
    pub runon_input_m: f64,
    pub cumulative_infiltration_m: f64,
    pub depression_storage_delta_m: f64,
    pub surface_saturation_runoff_m: f64,
    pub partition_runoff_m: f64,
    pub q_runoff_m: f64,
    pub closure_residual_m: f64,
}

impl DirectRunoffPartitionState {
    #[must_use]
    pub const fn zero() -> Self {
        Self {
            liquid_input_m: 0.0,
            runon_input_m: 0.0,
            cumulative_infiltration_m: 0.0,
            depression_storage_delta_m: 0.0,
            surface_saturation_runoff_m: 0.0,
            partition_runoff_m: 0.0,
            q_runoff_m: 0.0,
            closure_residual_m: 0.0,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct DirectRunoffDownstreamOperands {
    pub liquid_input_m: f64,
    pub runon_input_m: f64,
    pub cumulative_infiltration_m: f64,
    pub depression_storage_delta_m: f64,
    pub surface_saturation_runoff_m: f64,
    pub partition_runoff_m: f64,
    pub q_runoff_m: f64,
    pub closure_residual_m: f64,
}

impl DirectRunoffDownstreamOperands {
    #[must_use]
    pub const fn zero() -> Self {
        Self {
            liquid_input_m: 0.0,
            runon_input_m: 0.0,
            cumulative_infiltration_m: 0.0,
            depression_storage_delta_m: 0.0,
            surface_saturation_runoff_m: 0.0,
            partition_runoff_m: 0.0,
            q_runoff_m: 0.0,
            closure_residual_m: 0.0,
        }
    }
}

impl From<DirectRunoffPartitionState> for DirectRunoffDownstreamOperands {
    fn from(state: DirectRunoffPartitionState) -> Self {
        Self {
            liquid_input_m: state.liquid_input_m,
            runon_input_m: state.runon_input_m,
            cumulative_infiltration_m: state.cumulative_infiltration_m,
            depression_storage_delta_m: state.depression_storage_delta_m,
            surface_saturation_runoff_m: state.surface_saturation_runoff_m,
            partition_runoff_m: state.partition_runoff_m,
            q_runoff_m: state.q_runoff_m,
            closure_residual_m: state.closure_residual_m,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct DirectRunoffShadowProjection {
    pub lane_index: usize,
    pub day_index: usize,
    pub liquid_input_m: f64,
    pub runon_input_m: f64,
    pub cumulative_infiltration_m: f64,
    pub depression_storage_delta_m: f64,
    pub surface_saturation_runoff_m: f64,
    pub partition_runoff_m: f64,
    pub q_runoff_m: f64,
    pub closure_residual_m: f64,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct DirectStorageReconciliationInputs {
    pub storage_initial_m: f64,
    pub precip_input_m: f64,
    pub snow_coupling_m: f64,
    pub evapotranspiration_m: f64,
    pub deep_seepage_m: f64,
    pub subsurface_loss_m: f64,
    pub closure_tolerance_m: f64,
}

impl DirectStorageReconciliationInputs {
    #[must_use]
    pub const fn zero() -> Self {
        Self {
            storage_initial_m: 0.0,
            precip_input_m: 0.0,
            snow_coupling_m: 0.0,
            evapotranspiration_m: 0.0,
            deep_seepage_m: 0.0,
            subsurface_loss_m: 0.0,
            closure_tolerance_m: 0.0,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct DirectStorageReconciliationState {
    pub storage_initial_m: f64,
    pub precip_input_m: f64,
    pub snow_coupling_m: f64,
    pub q_runoff_m: f64,
    pub evapotranspiration_m: f64,
    pub deep_seepage_m: f64,
    pub subsurface_loss_m: f64,
    pub closure_tolerance_m: f64,
    pub storage_reconciled_m: f64,
    pub closure_residual_m: f64,
}

impl DirectStorageReconciliationState {
    #[must_use]
    pub const fn zero() -> Self {
        Self {
            storage_initial_m: 0.0,
            precip_input_m: 0.0,
            snow_coupling_m: 0.0,
            q_runoff_m: 0.0,
            evapotranspiration_m: 0.0,
            deep_seepage_m: 0.0,
            subsurface_loss_m: 0.0,
            closure_tolerance_m: 0.0,
            storage_reconciled_m: 0.0,
            closure_residual_m: 0.0,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct DirectStorageDownstreamOperands {
    pub storage_initial_m: f64,
    pub precip_input_m: f64,
    pub snow_coupling_m: f64,
    pub q_runoff_m: f64,
    pub evapotranspiration_m: f64,
    pub deep_seepage_m: f64,
    pub subsurface_loss_m: f64,
    pub storage_reconciled_m: f64,
    pub closure_residual_m: f64,
}

impl DirectStorageDownstreamOperands {
    #[must_use]
    pub const fn zero() -> Self {
        Self {
            storage_initial_m: 0.0,
            precip_input_m: 0.0,
            snow_coupling_m: 0.0,
            q_runoff_m: 0.0,
            evapotranspiration_m: 0.0,
            deep_seepage_m: 0.0,
            subsurface_loss_m: 0.0,
            storage_reconciled_m: 0.0,
            closure_residual_m: 0.0,
        }
    }
}

impl From<DirectStorageReconciliationState> for DirectStorageDownstreamOperands {
    fn from(state: DirectStorageReconciliationState) -> Self {
        Self {
            storage_initial_m: state.storage_initial_m,
            precip_input_m: state.precip_input_m,
            snow_coupling_m: state.snow_coupling_m,
            q_runoff_m: state.q_runoff_m,
            evapotranspiration_m: state.evapotranspiration_m,
            deep_seepage_m: state.deep_seepage_m,
            subsurface_loss_m: state.subsurface_loss_m,
            storage_reconciled_m: state.storage_reconciled_m,
            closure_residual_m: state.closure_residual_m,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct DirectStorageShadowProjection {
    pub lane_index: usize,
    pub day_index: usize,
    pub storage_initial_m: f64,
    pub precip_input_m: f64,
    pub snow_coupling_m: f64,
    pub q_runoff_m: f64,
    pub evapotranspiration_m: f64,
    pub deep_seepage_m: f64,
    pub subsurface_loss_m: f64,
    pub storage_reconciled_m: f64,
    pub closure_residual_m: f64,
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
pub struct DirectRunoffPartitionSpanReport {
    pub phase_count: usize,
    pub phase_entry_count: u64,
    pub direct_compute_count: u64,
    pub state_mutation_count: u64,
    pub downstream_operand_count: u64,
    pub shadow_projection_count: u64,
    pub compatibility_edge_invocation_count: u64,
    pub runoff_shadow_projection: DirectRunoffShadowProjection,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct DirectStorageReconciliationSpanReport {
    pub phase_count: usize,
    pub phase_entry_count: u64,
    pub direct_compute_count: u64,
    pub state_mutation_count: u64,
    pub downstream_operand_count: u64,
    pub shadow_projection_count: u64,
    pub compatibility_edge_invocation_count: u64,
    pub storage_shadow_projection: DirectStorageShadowProjection,
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
    pub phase_view_count: u64,
    pub phase_span_run_count: u64,
    pub direct_phase_entry_count: u64,
    pub direct_compute_count: u64,
    pub state_mutation_count: u64,
    pub downstream_operand_count: u64,
    pub shadow_projection_count: u64,
    pub compatibility_edge_invocation_count: u64,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DirectFrameExecutor {
    mode: DirectExecutorMode,
}

impl DirectFrameExecutor {
    #[must_use]
    pub fn new(mode: DirectExecutorMode) -> Self {
        DIRECT_AUDIT.record_executor_construction();
        Self { mode }
    }

    #[must_use]
    pub const fn mode(&self) -> DirectExecutorMode {
        self.mode
    }

    pub fn run_skeleton(
        &self,
        frame: &mut DirectRunFrame,
    ) -> Result<DirectExecutionReport, DirectRuntimeError> {
        DIRECT_AUDIT.record_skeleton_run();
        let mut phase_view_count = 0_u64;
        let mut phase_span_run_count = 0_u64;
        let mut direct_phase_entry_count = 0_u64;
        let mut direct_compute_count = 0_u64;
        let mut state_mutation_count = 0_u64;
        let mut downstream_operand_count = 0_u64;
        let mut shadow_projection_count = 0_u64;
        let mut compatibility_edge_invocation_count = 0_u64;

        let transfer_span_report = frame.run_r3c_lane_transfer_span()?;
        phase_span_run_count += 1;
        direct_phase_entry_count += transfer_span_report.phase_entry_count;
        direct_compute_count += transfer_span_report.direct_compute_count;
        state_mutation_count += transfer_span_report.state_mutation_count;
        downstream_operand_count += transfer_span_report.downstream_operand_count;
        shadow_projection_count += transfer_span_report.shadow_projection_count;
        compatibility_edge_invocation_count +=
            transfer_span_report.compatibility_edge_invocation_count;

        for lane_index in 0..frame.lanes.len() {
            let mut day_frame = DirectDayFrame::seed(frame.identity, lane_index, 0)?;
            let input_span_report = day_frame.run_r3a_input_accounting_span()?;
            phase_span_run_count += 1;
            direct_phase_entry_count += input_span_report.phase_entry_count;
            direct_compute_count += input_span_report.direct_compute_count;
            state_mutation_count += input_span_report.state_mutation_count;
            downstream_operand_count += input_span_report.downstream_operand_count;
            shadow_projection_count += input_span_report.shadow_projection_count;
            compatibility_edge_invocation_count +=
                input_span_report.compatibility_edge_invocation_count;

            let runoff_span_report = day_frame.run_r4a_runoff_partition_span()?;
            phase_span_run_count += 1;
            direct_phase_entry_count += runoff_span_report.phase_entry_count;
            direct_compute_count += runoff_span_report.direct_compute_count;
            state_mutation_count += runoff_span_report.state_mutation_count;
            downstream_operand_count += runoff_span_report.downstream_operand_count;
            shadow_projection_count += runoff_span_report.shadow_projection_count;
            compatibility_edge_invocation_count +=
                runoff_span_report.compatibility_edge_invocation_count;

            let storage_span_report = day_frame.run_r4b_storage_reconciliation_span()?;
            phase_span_run_count += 1;
            direct_phase_entry_count += storage_span_report.phase_entry_count;
            direct_compute_count += storage_span_report.direct_compute_count;
            state_mutation_count += storage_span_report.state_mutation_count;
            downstream_operand_count += storage_span_report.downstream_operand_count;
            shadow_projection_count += storage_span_report.shadow_projection_count;
            compatibility_edge_invocation_count +=
                storage_span_report.compatibility_edge_invocation_count;

            let ledger_span_report = day_frame.run_r3b_water_ledger_span()?;
            phase_span_run_count += 1;
            direct_phase_entry_count += ledger_span_report.phase_entry_count;
            direct_compute_count += ledger_span_report.direct_compute_count;
            state_mutation_count += ledger_span_report.state_mutation_count;
            downstream_operand_count += ledger_span_report.downstream_operand_count;
            shadow_projection_count += ledger_span_report.shadow_projection_count;
            compatibility_edge_invocation_count +=
                ledger_span_report.compatibility_edge_invocation_count;
            for phase in frame.phase_plan.phases() {
                let view = day_frame.phase_view(*phase);
                let _phase = view.phase();
                phase_view_count += 1;
            }
        }

        Ok(DirectExecutionReport {
            mode: self.mode,
            lane_count: frame.lanes.len(),
            day_count: frame.identity.day_count,
            planned_phase_count: frame.phase_plan.len(),
            phase_view_count,
            phase_span_run_count,
            direct_phase_entry_count,
            direct_compute_count,
            state_mutation_count,
            downstream_operand_count,
            shadow_projection_count,
            compatibility_edge_invocation_count,
        })
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct DirectRuntimeAuditSnapshot {
    pub run_frame_constructions: u64,
    pub day_frame_constructions: u64,
    pub executor_constructions: u64,
    pub skeleton_runs: u64,
    pub phase_view_constructions: u64,
    pub phase_span_runs: u64,
    pub direct_phase_entries: u64,
    pub direct_compute_operations: u64,
    pub direct_state_mutations: u64,
    pub downstream_operand_productions: u64,
    pub shadow_projections: u64,
    pub compatibility_edge_invocations: u64,
}

#[must_use]
pub fn direct_runtime_audit_snapshot() -> DirectRuntimeAuditSnapshot {
    DIRECT_AUDIT.snapshot()
}

pub fn reset_direct_runtime_audit_counters() {
    DIRECT_AUDIT.reset();
}

pub fn record_direct_runtime_compatibility_edge_invocation() {
    DIRECT_AUDIT.record_compatibility_edge_invocation();
}

struct DirectRuntimeAuditCounters {
    run_frame_constructions: AtomicU64,
    day_frame_constructions: AtomicU64,
    executor_constructions: AtomicU64,
    skeleton_runs: AtomicU64,
    phase_view_constructions: AtomicU64,
    phase_span_runs: AtomicU64,
    direct_phase_entries: AtomicU64,
    direct_compute_operations: AtomicU64,
    direct_state_mutations: AtomicU64,
    downstream_operand_productions: AtomicU64,
    shadow_projections: AtomicU64,
    compatibility_edge_invocations: AtomicU64,
}

impl DirectRuntimeAuditCounters {
    const fn new() -> Self {
        Self {
            run_frame_constructions: AtomicU64::new(0),
            day_frame_constructions: AtomicU64::new(0),
            executor_constructions: AtomicU64::new(0),
            skeleton_runs: AtomicU64::new(0),
            phase_view_constructions: AtomicU64::new(0),
            phase_span_runs: AtomicU64::new(0),
            direct_phase_entries: AtomicU64::new(0),
            direct_compute_operations: AtomicU64::new(0),
            direct_state_mutations: AtomicU64::new(0),
            downstream_operand_productions: AtomicU64::new(0),
            shadow_projections: AtomicU64::new(0),
            compatibility_edge_invocations: AtomicU64::new(0),
        }
    }

    fn record_run_frame_construction(&self) {
        self.run_frame_constructions.fetch_add(1, Ordering::Relaxed);
    }

    fn record_day_frame_construction(&self) {
        self.day_frame_constructions.fetch_add(1, Ordering::Relaxed);
    }

    fn record_executor_construction(&self) {
        self.executor_constructions.fetch_add(1, Ordering::Relaxed);
    }

    fn record_skeleton_run(&self) {
        self.skeleton_runs.fetch_add(1, Ordering::Relaxed);
    }

    fn record_phase_view_construction(&self) {
        self.phase_view_constructions
            .fetch_add(1, Ordering::Relaxed);
    }

    fn record_phase_span_run(&self) {
        self.phase_span_runs.fetch_add(1, Ordering::Relaxed);
    }

    fn record_direct_phase_entry(&self) {
        self.direct_phase_entries.fetch_add(1, Ordering::Relaxed);
    }

    fn record_direct_compute_operation(&self) {
        self.direct_compute_operations
            .fetch_add(1, Ordering::Relaxed);
    }

    fn record_direct_state_mutation(&self) {
        self.direct_state_mutations.fetch_add(1, Ordering::Relaxed);
    }

    fn record_downstream_operand_production(&self) {
        self.downstream_operand_productions
            .fetch_add(1, Ordering::Relaxed);
    }

    fn record_shadow_projection(&self) {
        self.shadow_projections.fetch_add(1, Ordering::Relaxed);
    }

    fn record_compatibility_edge_invocation(&self) {
        self.compatibility_edge_invocations
            .fetch_add(1, Ordering::Relaxed);
    }

    fn snapshot(&self) -> DirectRuntimeAuditSnapshot {
        DirectRuntimeAuditSnapshot {
            run_frame_constructions: self.run_frame_constructions.load(Ordering::Relaxed),
            day_frame_constructions: self.day_frame_constructions.load(Ordering::Relaxed),
            executor_constructions: self.executor_constructions.load(Ordering::Relaxed),
            skeleton_runs: self.skeleton_runs.load(Ordering::Relaxed),
            phase_view_constructions: self.phase_view_constructions.load(Ordering::Relaxed),
            phase_span_runs: self.phase_span_runs.load(Ordering::Relaxed),
            direct_phase_entries: self.direct_phase_entries.load(Ordering::Relaxed),
            direct_compute_operations: self.direct_compute_operations.load(Ordering::Relaxed),
            direct_state_mutations: self.direct_state_mutations.load(Ordering::Relaxed),
            downstream_operand_productions: self
                .downstream_operand_productions
                .load(Ordering::Relaxed),
            shadow_projections: self.shadow_projections.load(Ordering::Relaxed),
            compatibility_edge_invocations: self
                .compatibility_edge_invocations
                .load(Ordering::Relaxed),
        }
    }

    fn reset(&self) {
        self.run_frame_constructions.store(0, Ordering::Relaxed);
        self.day_frame_constructions.store(0, Ordering::Relaxed);
        self.executor_constructions.store(0, Ordering::Relaxed);
        self.skeleton_runs.store(0, Ordering::Relaxed);
        self.phase_view_constructions.store(0, Ordering::Relaxed);
        self.phase_span_runs.store(0, Ordering::Relaxed);
        self.direct_phase_entries.store(0, Ordering::Relaxed);
        self.direct_compute_operations.store(0, Ordering::Relaxed);
        self.direct_state_mutations.store(0, Ordering::Relaxed);
        self.downstream_operand_productions
            .store(0, Ordering::Relaxed);
        self.shadow_projections.store(0, Ordering::Relaxed);
        self.compatibility_edge_invocations
            .store(0, Ordering::Relaxed);
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum DirectRuntimeError {
    InvalidLaneCount {
        lane_count: usize,
    },
    InvalidDayCount {
        day_count: usize,
    },
    LaneIdOverflow {
        lane_index: usize,
    },
    FrameLaneCountMismatch {
        identity_lane_count: usize,
        actual_lane_count: usize,
    },
    InvalidLaneTopology {
        lane_index: usize,
        lane_id: u32,
        upstream_lane_id: u32,
        downstream_lane_id: u32,
    },
    InvalidLaneOutletCount {
        outlet_count: usize,
    },
    LaneIndexOutOfRange {
        lane_index: usize,
        lane_count: usize,
    },
    DayIndexOutOfRange {
        day_index: usize,
        day_count: usize,
    },
    MissingDirectUpstream {
        upstream: &'static str,
    },
    NonFiniteDirectValue {
        field: &'static str,
    },
    NegativeDirectValue {
        field: &'static str,
    },
    DirectClosureToleranceExceeded {
        field: &'static str,
    },
}

impl fmt::Display for DirectRuntimeError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InvalidLaneCount { lane_count } => {
                write!(
                    formatter,
                    "direct runtime requires at least one lane, observed {lane_count}"
                )
            }
            Self::InvalidDayCount { day_count } => {
                write!(
                    formatter,
                    "direct runtime requires at least one day, observed {day_count}"
                )
            }
            Self::LaneIdOverflow { lane_index } => {
                write!(
                    formatter,
                    "direct runtime lane index {lane_index} cannot be represented as a u32 lane id"
                )
            }
            Self::FrameLaneCountMismatch {
                identity_lane_count,
                actual_lane_count,
            } => {
                write!(
                    formatter,
                    "direct runtime frame lane count {actual_lane_count} does not match identity lane count {identity_lane_count}"
                )
            }
            Self::InvalidLaneTopology {
                lane_index,
                lane_id,
                upstream_lane_id,
                downstream_lane_id,
            } => {
                write!(
                    formatter,
                    "direct runtime lane topology is invalid at index {lane_index}: lane {lane_id}, upstream {upstream_lane_id}, downstream {downstream_lane_id}"
                )
            }
            Self::InvalidLaneOutletCount { outlet_count } => {
                write!(
                    formatter,
                    "direct runtime requires exactly one lane outlet, observed {outlet_count}"
                )
            }
            Self::LaneIndexOutOfRange {
                lane_index,
                lane_count,
            } => {
                write!(
                    formatter,
                    "direct runtime lane index {lane_index} is outside lane count {lane_count}"
                )
            }
            Self::DayIndexOutOfRange {
                day_index,
                day_count,
            } => {
                write!(
                    formatter,
                    "direct runtime day index {day_index} is outside day count {day_count}"
                )
            }
            Self::MissingDirectUpstream { upstream } => {
                write!(
                    formatter,
                    "direct runtime upstream span {upstream} must execute before this span"
                )
            }
            Self::NonFiniteDirectValue { field } => {
                write!(formatter, "direct runtime field {field} must be finite")
            }
            Self::NegativeDirectValue { field } => {
                write!(
                    formatter,
                    "direct runtime field {field} must be nonnegative"
                )
            }
            Self::DirectClosureToleranceExceeded { field } => {
                write!(
                    formatter,
                    "direct runtime field {field} exceeds declared closure tolerance"
                )
            }
        }
    }
}

impl Error for DirectRuntimeError {}

fn validate_finite(field: &'static str, value: f64) -> Result<(), DirectRuntimeError> {
    if value.is_finite() {
        Ok(())
    } else {
        Err(DirectRuntimeError::NonFiniteDirectValue { field })
    }
}

fn validate_nonnegative_direct_m(
    field: &'static str,
    value: f64,
) -> Result<(), DirectRuntimeError> {
    validate_finite(field, value)?;
    if value >= 0.0 {
        Ok(())
    } else {
        Err(DirectRuntimeError::NegativeDirectValue { field })
    }
}

fn sum_nonnegative_direct_m(
    field: &'static str,
    values: &[f64; DIRECT_TRANSFER_HOUR_COUNT],
) -> Result<f64, DirectRuntimeError> {
    let mut total = 0.0;
    for value in values {
        validate_nonnegative_direct_m(field, *value)?;
        total += value;
        validate_finite(field, total)?;
    }
    Ok(total)
}

fn sum_finite_direct_m(field: &'static str, values: &[f64]) -> Result<f64, DirectRuntimeError> {
    let mut total = 0.0;
    for value in values {
        total += value;
        validate_finite(field, total)?;
    }
    Ok(total)
}
