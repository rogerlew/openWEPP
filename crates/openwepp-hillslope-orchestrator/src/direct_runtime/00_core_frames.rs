#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub enum DirectExecutorMode {
    #[default]
    Noop,
    ShadowOnly,
    ProductionDirect,
}

impl DirectExecutorMode {
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Noop => "noop",
            Self::ShadowOnly => "shadow-only",
            Self::ProductionDirect => "production-direct",
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

    #[must_use]
    pub const fn rank(self) -> usize {
        match self {
            Self::Normalization => 0,
            Self::StorageBounds => 1,
            Self::DecompositionTransition => 2,
            Self::ResiduePartitionTransition => 3,
            Self::AnnualGrowthTransition => 4,
            Self::PerennialGrowthTransition => 5,
            Self::PercolationDeepSeepage => 6,
            Self::Evapotranspiration => 7,
            Self::Drainage => 8,
            Self::LateralTransfer => 9,
            Self::PlantRootUptake => 10,
            Self::RunoffReconciliation => 11,
            Self::StorageReconciliation => 12,
            Self::ClosureDiagnostics => 13,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DirectPhaseLifecycleStatus {
    Executed,
    Hold,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct DirectPhaseStatusCount {
    pub phase: DirectPhaseKind,
    pub status: DirectPhaseLifecycleStatus,
    pub count: u64,
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
pub struct DirectRunConstructorInputs {
    pub identity: DirectRunIdentity,
    pub lanes: Vec<DirectLaneConstructorInputs>,
    pub phase_plan: DirectPhasePlan,
}

impl DirectRunConstructorInputs {
    #[must_use]
    pub fn new(identity: DirectRunIdentity, lanes: Vec<DirectLaneConstructorInputs>) -> Self {
        Self {
            identity,
            lanes,
            phase_plan: DirectPhasePlan::default(),
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct DirectLaneConstructorInputs {
    pub lane_id: u32,
    pub upstream_lane_id: u32,
    pub downstream_lane_id: u32,
    pub upstream_area_ratio: f64,
    pub area_m2: f64,
    pub runoff_publication_q_scale: f64,
    pub runoff_publication_qofe_scale: f64,
    pub runoff_publication_efflen_m: f64,
    pub runoff_publication_cumulative_length_m: f64,
    pub runoff_publication_ofe_length_m: f64,
    pub water: DirectWaterState,
    pub transfer: DirectTransferBuffers,
    pub publication: DirectPublicationFrame,
    pub subsurface_layers: Vec<DirectSubsurfaceLayerState>,
    pub evapotranspiration_stage_state: Option<Box<DirectEvapotranspirationStageState>>,
    pub plant_growth_state: Box<DirectGrowthStateSurface>,
    pub plant_water_stress: f64,
    pub winter_column: Box<DirectWinterColumnState>,
    pub snow_runtime_carry: Option<DirectSnowRuntimeCarry>,
    pub frost_runtime_carry: Option<DirectFrostRuntimeCarry>,
    pub day_inputs: Vec<DirectDayConstructorInputs>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct DirectSnowRuntimeCarry {
    pub runtime_swe_m: f64,
    pub runtime_depth_m: f64,
    pub runtime_density_kg_m3: f64,
    pub runtime_settle_day_count: f64,
    pub coe_boundary_depth_m: f64,
    pub coe_boundary_density_kg_m3: f64,
    pub coe_boundary_settle_day_count: f64,
    pub liquid_water_retained_m: f64,
    pub snow_albedo_state: Option<SnowAlbedoState>,
    pub layers: Vec<DirectSnowLayerState>,
}

impl From<DirectSnowRuntimeCarry> for DirectSnowLaneState {
    fn from(carry: DirectSnowRuntimeCarry) -> Self {
        Self::from_runtime_values_boundary_liquid_albedo_and_layers(
            carry.runtime_swe_m,
            carry.runtime_depth_m,
            carry.runtime_density_kg_m3,
            carry.runtime_settle_day_count,
            carry.coe_boundary_depth_m,
            carry.coe_boundary_density_kg_m3,
            carry.coe_boundary_settle_day_count,
            carry.liquid_water_retained_m,
            carry.snow_albedo_state,
            carry.layers,
        )
    }
}

impl From<&DirectSnowLaneState> for DirectSnowRuntimeCarry {
    fn from(state: &DirectSnowLaneState) -> Self {
        Self {
            runtime_swe_m: state.runtime_swe_m,
            runtime_depth_m: state.runtime_depth_m,
            runtime_density_kg_m3: state.runtime_density_kg_m3,
            runtime_settle_day_count: state.runtime_settle_day_count,
            coe_boundary_depth_m: state.coe_boundary_depth_m,
            coe_boundary_density_kg_m3: state.coe_boundary_density_kg_m3,
            coe_boundary_settle_day_count: state.coe_boundary_settle_day_count,
            liquid_water_retained_m: state.liquid_water_retained_m,
            snow_albedo_state: state.snow_albedo_state,
            layers: state.layers.clone(),
        }
    }
}

impl From<DirectSnowLaneState> for DirectSnowRuntimeCarry {
    fn from(state: DirectSnowLaneState) -> Self {
        Self {
            runtime_swe_m: state.runtime_swe_m,
            runtime_depth_m: state.runtime_depth_m,
            runtime_density_kg_m3: state.runtime_density_kg_m3,
            runtime_settle_day_count: state.runtime_settle_day_count,
            coe_boundary_depth_m: state.coe_boundary_depth_m,
            coe_boundary_density_kg_m3: state.coe_boundary_density_kg_m3,
            coe_boundary_settle_day_count: state.coe_boundary_settle_day_count,
            liquid_water_retained_m: state.liquid_water_retained_m,
            snow_albedo_state: state.snow_albedo_state,
            layers: state.layers,
        }
    }
}

fn direct_snow_runtime_carry_from_winter_state(
    state: &DirectSnowLaneState,
) -> Option<DirectSnowRuntimeCarry> {
    state
        .has_runtime_state()
        .then(|| DirectSnowRuntimeCarry::from(state))
}

#[derive(Debug, Clone, PartialEq)]
pub struct DirectFrostRuntimeCarry {
    pub active_frost_coupling: bool,
    pub dfrost_m: f64,
    pub dthaw_m: f64,
    pub nft: f64,
    pub ws_frz_m: f64,
    pub infcap_frz_m_s: f64,
    pub frwatc_soil_water_before_m: f64,
    pub frwatc_soil_water_after_m: f64,
    pub frwatc_frozen_water_before_m: f64,
    pub frwatc_frozen_water_after_m: f64,
    pub frwatc_freeze_debit_m: f64,
    pub frwatc_thaw_credit_m: f64,
    pub frwatc_net_liquid_delta_m: f64,
    pub frdp_m: f64,
    pub thdp_m: f64,
    pub tfrdp_m: f64,
    pub tthawd_m: f64,
    pub fgthwd_flag: f64,
    pub total_fine_layer_count: f64,
    pub conductivity_tilled_w_m_k: f64,
    pub conductivity_untilled_w_m_k: f64,
    pub conductivity_residue_w_m_k: f64,
    pub shadow_total_water_before_m: f64,
    pub shadow_total_water_after_m: f64,
    pub shadow_wb_delta_m: f64,
    pub shadow_frwatc_residual_m: f64,
    pub watpdg_m: f64,
    pub watbtm_m: f64,
    pub layer_shadows: Vec<DirectFrostLayerShadowCarry>,
    pub fine_layers: Vec<DirectFrostFineLayerCarry>,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct DirectFrostLayerShadowCarry {
    pub layer_index: usize,
    pub st_m: f64,
    pub soil_water_m: f64,
    pub frozen_depth_m: f64,
    pub frozen_water_m: f64,
    pub soilf_m: f64,
    pub yst_m: f64,
    pub nwfrzz_m: f64,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct DirectFrostFineLayerCarry {
    pub layer_index: usize,
    pub fine_index: usize,
    pub fgfrst: f64,
    pub slfsd_m: f64,
    pub slsic_m: f64,
    pub slsw_theta: f64,
    pub sltime_s: f64,
}

impl From<DirectFrostLayerShadowCarry> for DirectFrostLayerShadowState {
    fn from(carry: DirectFrostLayerShadowCarry) -> Self {
        Self {
            layer_index: carry.layer_index,
            st_m: carry.st_m,
            soil_water_m: carry.soil_water_m,
            frozen_depth_m: carry.frozen_depth_m,
            frozen_water_m: carry.frozen_water_m,
            soilf_m: carry.soilf_m,
            yst_m: carry.yst_m,
            nwfrzz_m: carry.nwfrzz_m,
        }
    }
}

impl From<DirectFrostLayerShadowState> for DirectFrostLayerShadowCarry {
    fn from(state: DirectFrostLayerShadowState) -> Self {
        Self {
            layer_index: state.layer_index,
            st_m: state.st_m,
            soil_water_m: state.soil_water_m,
            frozen_depth_m: state.frozen_depth_m,
            frozen_water_m: state.frozen_water_m,
            soilf_m: state.soilf_m,
            yst_m: state.yst_m,
            nwfrzz_m: state.nwfrzz_m,
        }
    }
}

impl From<DirectFrostFineLayerCarry> for DirectFrostFineLayerState {
    fn from(carry: DirectFrostFineLayerCarry) -> Self {
        Self {
            layer_index: carry.layer_index,
            fine_index: carry.fine_index,
            fgfrst: carry.fgfrst,
            slfsd_m: carry.slfsd_m,
            slsic_m: carry.slsic_m,
            slsw_theta: carry.slsw_theta,
            sltime_s: carry.sltime_s,
        }
    }
}

impl From<DirectFrostFineLayerState> for DirectFrostFineLayerCarry {
    fn from(state: DirectFrostFineLayerState) -> Self {
        Self {
            layer_index: state.layer_index,
            fine_index: state.fine_index,
            fgfrst: state.fgfrst,
            slfsd_m: state.slfsd_m,
            slsic_m: state.slsic_m,
            slsw_theta: state.slsw_theta,
            sltime_s: state.sltime_s,
        }
    }
}

impl From<DirectFrostRuntimeCarry> for DirectFrostLaneState {
    fn from(carry: DirectFrostRuntimeCarry) -> Self {
        Self {
            active_frost_coupling: carry.active_frost_coupling,
            dfrost_m: carry.dfrost_m,
            dthaw_m: carry.dthaw_m,
            nft: carry.nft,
            ws_frz_m: carry.ws_frz_m,
            infcap_frz_m_s: carry.infcap_frz_m_s,
            frwatc_soil_water_before_m: carry.frwatc_soil_water_before_m,
            frwatc_soil_water_after_m: carry.frwatc_soil_water_after_m,
            frwatc_frozen_water_before_m: carry.frwatc_frozen_water_before_m,
            frwatc_frozen_water_after_m: carry.frwatc_frozen_water_after_m,
            frwatc_freeze_debit_m: carry.frwatc_freeze_debit_m,
            frwatc_thaw_credit_m: carry.frwatc_thaw_credit_m,
            frwatc_net_liquid_delta_m: carry.frwatc_net_liquid_delta_m,
            frdp_m: carry.frdp_m,
            thdp_m: carry.thdp_m,
            tfrdp_m: carry.tfrdp_m,
            tthawd_m: carry.tthawd_m,
            fgthwd_flag: carry.fgthwd_flag,
            total_fine_layer_count: carry.total_fine_layer_count,
            conductivity_tilled_w_m_k: carry.conductivity_tilled_w_m_k,
            conductivity_untilled_w_m_k: carry.conductivity_untilled_w_m_k,
            conductivity_residue_w_m_k: carry.conductivity_residue_w_m_k,
            shadow_total_water_before_m: carry.shadow_total_water_before_m,
            shadow_total_water_after_m: carry.shadow_total_water_after_m,
            shadow_wb_delta_m: carry.shadow_wb_delta_m,
            shadow_frwatc_residual_m: carry.shadow_frwatc_residual_m,
            watpdg_m: carry.watpdg_m,
            watbtm_m: carry.watbtm_m,
            layer_shadows: carry.layer_shadows.into_iter().map(Into::into).collect(),
            fine_layers: carry.fine_layers.into_iter().map(Into::into).collect(),
        }
    }
}

impl From<DirectFrostLaneState> for DirectFrostRuntimeCarry {
    fn from(state: DirectFrostLaneState) -> Self {
        Self {
            active_frost_coupling: state.active_frost_coupling,
            dfrost_m: state.dfrost_m,
            dthaw_m: state.dthaw_m,
            nft: state.nft,
            ws_frz_m: state.ws_frz_m,
            infcap_frz_m_s: state.infcap_frz_m_s,
            frwatc_soil_water_before_m: state.frwatc_soil_water_before_m,
            frwatc_soil_water_after_m: state.frwatc_soil_water_after_m,
            frwatc_frozen_water_before_m: state.frwatc_frozen_water_before_m,
            frwatc_frozen_water_after_m: state.frwatc_frozen_water_after_m,
            frwatc_freeze_debit_m: state.frwatc_freeze_debit_m,
            frwatc_thaw_credit_m: state.frwatc_thaw_credit_m,
            frwatc_net_liquid_delta_m: state.frwatc_net_liquid_delta_m,
            frdp_m: state.frdp_m,
            thdp_m: state.thdp_m,
            tfrdp_m: state.tfrdp_m,
            tthawd_m: state.tthawd_m,
            fgthwd_flag: state.fgthwd_flag,
            total_fine_layer_count: state.total_fine_layer_count,
            conductivity_tilled_w_m_k: state.conductivity_tilled_w_m_k,
            conductivity_untilled_w_m_k: state.conductivity_untilled_w_m_k,
            conductivity_residue_w_m_k: state.conductivity_residue_w_m_k,
            shadow_total_water_before_m: state.shadow_total_water_before_m,
            shadow_total_water_after_m: state.shadow_total_water_after_m,
            shadow_wb_delta_m: state.shadow_wb_delta_m,
            shadow_frwatc_residual_m: state.shadow_frwatc_residual_m,
            watpdg_m: state.watpdg_m,
            watbtm_m: state.watbtm_m,
            layer_shadows: state.layer_shadows.into_iter().map(Into::into).collect(),
            fine_layers: state.fine_layers.into_iter().map(Into::into).collect(),
        }
    }
}

fn direct_frost_runtime_carry_from_winter_state(
    state: &DirectFrostLaneState,
) -> Option<DirectFrostRuntimeCarry> {
    state.has_runtime_state().then(|| state.clone().into())
}

impl DirectLaneConstructorInputs {
    pub fn from_topology_with_dynamic_day_inputs(
        lane_index: usize,
        lane_count: usize,
    ) -> Result<Self, DirectRuntimeError> {
        Self::from_topology(lane_index, lane_count, 0)
    }

    pub fn from_topology(
        lane_index: usize,
        lane_count: usize,
        day_count: usize,
    ) -> Result<Self, DirectRuntimeError> {
        let skeleton = DirectLaneFrame::skeleton(lane_index, lane_count)?;
        Ok(Self {
            lane_id: skeleton.lane_id,
            upstream_lane_id: skeleton.upstream_lane_id,
            downstream_lane_id: skeleton.downstream_lane_id,
            upstream_area_ratio: 1.0,
            area_m2: 1.0,
            runoff_publication_q_scale: 1.0,
            runoff_publication_qofe_scale: 1.0,
            runoff_publication_efflen_m: 1.0,
            runoff_publication_cumulative_length_m: 1.0,
            runoff_publication_ofe_length_m: 1.0,
            water: DirectWaterState::zero(),
            transfer: DirectTransferBuffers::zero(),
            publication: DirectPublicationFrame::empty(),
            subsurface_layers: Vec::new(),
            evapotranspiration_stage_state: None,
            plant_growth_state: Box::new(DirectGrowthStateSurface::zero()),
            plant_water_stress: 1.0,
            winter_column: Box::new(DirectWinterColumnState::zero()),
            snow_runtime_carry: None,
            frost_runtime_carry: None,
            day_inputs: vec![DirectDayConstructorInputs::zero(); day_count],
        })
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct DirectDayConstructorInputs {
    pub forcing: DirectDayForcing,
    pub normalization_inputs: DirectNormalizationInputs,
    pub interception_m: f64,
    pub storage_bounds_inputs: DirectStorageBoundsInputs,
    pub decomposition_inputs: DirectDecompositionInputs,
    pub residue_partition_inputs: DirectResiduePartitionInputs,
    pub annual_growth_inputs: DirectGrowthInputs,
    pub perennial_growth_inputs: DirectGrowthInputs,
    pub storage_input_inputs: DirectStorageInputInputs,
    pub liquid_input_inputs: DirectLiquidInputInputs,
    pub runon_carry_inputs: DirectRunonCarryInputs,
    pub infiltration_depression_inputs: DirectInfiltrationDepressionInputs,
    pub saturation_addback_inputs: DirectSaturationAddbackInputs,
    pub runoff_partition_inputs: DirectRunoffPartitionInputs,
    pub percolation_inputs: DirectPercolationInputs,
    pub subsurface_compute_inputs: DirectSubsurfaceComputeInputs,
    pub deep_seepage_inputs: DirectDeepSeepageInputs,
    pub subsurface_loss_inputs: DirectSubsurfaceLossInputs,
    pub evapotranspiration_compute_inputs: DirectEvapotranspirationComputeInputs,
    pub evapotranspiration_inputs: DirectEvapotranspirationInputs,
    pub snow_coupling_inputs: DirectSnowCouplingInputs,
    pub storage_reconciliation_inputs: DirectStorageReconciliationInputs,
    pub hydrology_projection_inputs: DirectHydrologyProjectionInputs,
    pub erosion_inputs: DirectErosionInputs,
    pub frost_storage_liquid_delta_m: Option<f64>,
    pub winter_frost_compute_inputs: Option<crate::hydrology::DirectWinterFrostComputeInputs>,
    pub winter_frost_outcome: Option<Box<crate::hydrology::DirectWinterFrostPartitionOutcome>>,
    pub frost_layer_carry_projection: Option<Vec<DirectFrostLayerCarryProjection>>,
    pub snow_runtime_carry: Option<Box<DirectSnowRuntimeCarry>>,
    pub frost_runtime_carry: Option<DirectFrostRuntimeCarry>,
}

impl DirectDayConstructorInputs {
    #[must_use]
    pub fn zero() -> Self {
        Self {
            forcing: DirectDayForcing::zero(),
            normalization_inputs: DirectNormalizationInputs::zero(),
            interception_m: 0.0,
            storage_bounds_inputs: DirectStorageBoundsInputs::zero(),
            decomposition_inputs: DirectDecompositionInputs::zero(),
            residue_partition_inputs: DirectResiduePartitionInputs::zero(),
            annual_growth_inputs: DirectGrowthInputs::zero(),
            perennial_growth_inputs: DirectGrowthInputs::zero(),
            storage_input_inputs: DirectStorageInputInputs::zero(),
            liquid_input_inputs: DirectLiquidInputInputs::zero(),
            runon_carry_inputs: DirectRunonCarryInputs::zero(),
            infiltration_depression_inputs: DirectInfiltrationDepressionInputs::zero(),
            saturation_addback_inputs: DirectSaturationAddbackInputs::zero(),
            runoff_partition_inputs: DirectRunoffPartitionInputs::zero(),
            percolation_inputs: DirectPercolationInputs::neutral(),
            subsurface_compute_inputs: DirectSubsurfaceComputeInputs::neutral(),
            deep_seepage_inputs: DirectDeepSeepageInputs::zero(),
            subsurface_loss_inputs: DirectSubsurfaceLossInputs::zero(),
            evapotranspiration_compute_inputs: DirectEvapotranspirationComputeInputs::zero(),
            evapotranspiration_inputs: DirectEvapotranspirationInputs::zero(),
            snow_coupling_inputs: DirectSnowCouplingInputs::zero(),
            storage_reconciliation_inputs: DirectStorageReconciliationInputs::zero(),
            hydrology_projection_inputs: DirectHydrologyProjectionInputs::zero(),
            erosion_inputs: DirectErosionInputs::zero(),
            frost_storage_liquid_delta_m: None,
            winter_frost_compute_inputs: None,
            winter_frost_outcome: None,
            frost_layer_carry_projection: None,
            snow_runtime_carry: None,
            frost_runtime_carry: None,
        }
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
    pub groundwater: DirectGroundwaterRunState,
    /// D15A (rev 27): the opt-in ACTIVE routing configuration. `Some` IS the
    /// activation selector inside the orchestrator (the runner sets it from
    /// `OPENWEPP_LANED_ACTIVE=1` after its fail-closed preflight); `None`
    /// keeps the default execution path untouched (`INV-OFEROUTE-010`).
    pub laned_active: Option<Box<laned_active::DirectLanedActiveConfig>>,
    /// D15A (rev 27): run-level active evidence accumulated by the executor;
    /// the runner surfaces it as the manifest `laned_active` block.
    pub laned_active_summary: Option<Box<laned_active::DirectLanedActiveRunSummary>>,
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
            groundwater: DirectGroundwaterRunState::disabled(),
            laned_active: None,
            laned_active_summary: None,
        })
    }

    pub fn from_constructor_inputs(
        inputs: DirectRunConstructorInputs,
    ) -> Result<Self, DirectRuntimeError> {
        validate_direct_run_constructor_inputs(&inputs)?;
        DIRECT_AUDIT.record_run_frame_construction();
        let lanes = inputs
            .lanes
            .into_iter()
            .map(DirectLaneFrame::from_constructor_inputs)
            .collect::<Vec<_>>();

        Ok(Self {
            identity: inputs.identity,
            lanes,
            phase_plan: inputs.phase_plan,
            publication: DirectPublicationFrame::empty(),
            lane_transfer_ledger: vec![
                DirectLaneTransferLedger::zero();
                inputs.identity.lane_count
            ],
            lane_transfer_downstream_operands: DirectRunTransferDownstreamOperands::zero(),
            lane_transfer_shadow_projection: None,
            groundwater: DirectGroundwaterRunState::disabled(),
            laned_active: None,
            laned_active_summary: None,
        })
    }

    pub fn configure_groundwater(
        &mut self,
        authority: DirectGroundwaterAuthority,
    ) -> Result<(), DirectRuntimeError> {
        let total_area_m2 = self.total_area_m2()?;
        self.groundwater = DirectGroundwaterRunState::from_authority(authority, total_area_m2)?;
        Ok(())
    }

    pub fn run_groundwater_day_from_lane_frames(
        &mut self,
        day_index: usize,
        day_frames: &mut [DirectDayFrame],
    ) -> Result<DirectGroundwaterDayOutput, DirectRuntimeError> {
        if day_frames.len() != self.lanes.len() {
            return Err(DirectRuntimeError::FrameLaneCountMismatch {
                identity_lane_count: self.lanes.len(),
                actual_lane_count: day_frames.len(),
            });
        }
        let mut recharge_m3 = 0.0;
        let mut total_area_m2 = 0.0;
        for (lane_index, (lane, day_frame)) in self.lanes.iter().zip(day_frames.iter()).enumerate()
        {
            if day_frame.day_index != day_index {
                return Err(DirectRuntimeError::DayIndexOutOfRange {
                    day_index: day_frame.day_index,
                    day_count: self.identity.day_count,
                });
            }
            if day_frame.lane_index != lane_index {
                return Err(DirectRuntimeError::LaneIndexOutOfRange {
                    lane_index: day_frame.lane_index,
                    lane_count: self.lanes.len(),
                });
            }
            validate_nonnegative_direct_m(
                "groundwater.deep_percolation_m",
                day_frame.hydrology_projection.deep_percolation_m,
            )?;
            validate_finite("groundwater.lane_area_m2", lane.area_m2)?;
            if lane.area_m2 <= 0.0 {
                return Err(DirectRuntimeError::InvalidPublicationArea {
                    lane_id: lane.lane_id,
                    area_m2: lane.area_m2,
                });
            }
            recharge_m3 += day_frame.hydrology_projection.deep_percolation_m * lane.area_m2;
            total_area_m2 += lane.area_m2;
        }
        validate_nonnegative_direct_m("groundwater.total_recharge_m3", recharge_m3)?;
        let output = self.groundwater.run_day(recharge_m3, total_area_m2)?;
        for day_frame in day_frames {
            day_frame.groundwater_output = output;
        }
        Ok(output)
    }

    fn total_area_m2(&self) -> Result<f64, DirectRuntimeError> {
        let mut total_area_m2 = 0.0;
        for lane in &self.lanes {
            validate_finite("groundwater.lane_area_m2", lane.area_m2)?;
            if lane.area_m2 <= 0.0 {
                return Err(DirectRuntimeError::InvalidPublicationArea {
                    lane_id: lane.lane_id,
                    area_m2: lane.area_m2,
                });
            }
            total_area_m2 += lane.area_m2;
        }
        validate_finite("groundwater.total_area_m2", total_area_m2)?;
        if total_area_m2 <= 0.0 {
            return Err(DirectRuntimeError::DirectDomainViolation {
                field: "groundwater.total_area_m2",
            });
        }
        Ok(total_area_m2)
    }

    fn seed_day_frame(
        &self,
        lane_index: usize,
        day_index: usize,
    ) -> Result<DirectDayFrame, DirectRuntimeError> {
        if self.lanes.len() != self.identity.lane_count {
            return Err(DirectRuntimeError::FrameLaneCountMismatch {
                identity_lane_count: self.identity.lane_count,
                actual_lane_count: self.lanes.len(),
            });
        }
        let lane = self
            .lanes
            .get(lane_index)
            .ok_or(DirectRuntimeError::LaneIndexOutOfRange {
                lane_index,
                lane_count: self.lanes.len(),
            })?;
        let mut day_frame = DirectDayFrame::seed(self.identity, lane_index, day_index)?;
        day_frame.upstream_area_ratio = lane.upstream_area_ratio;
        day_frame.water = lane.water.clone();
        day_frame.transfer = lane.transfer.clone();
        day_frame.publication = lane.publication.clone();
        day_frame.prior_erosion_downstream_operands = lane.erosion_downstream_operands.clone();
        if lane.upstream_lane_id != 0 {
            let upstream_index = (lane.upstream_lane_id - 1) as usize;
            let upstream_lane =
                self.lanes
                    .get(upstream_index)
                    .ok_or(DirectRuntimeError::InvalidLaneTopology {
                        lane_index,
                        lane_id: lane.lane_id,
                        upstream_lane_id: lane.upstream_lane_id,
                        downstream_lane_id: lane.downstream_lane_id,
                    })?;
            day_frame.upstream_erosion_downstream_operands =
                upstream_lane.erosion_downstream_operands.clone();
        }
        if let Some(day_inputs) = lane.day_inputs.get(day_index) {
            day_frame.apply_constructor_inputs(day_inputs.clone())?;
        }
        if !lane.subsurface_layers.is_empty() {
            day_frame.percolation_inputs.soil_water_initial_m = lane.water.soil_water_m;
            day_frame
                .percolation_inputs
                .layers
                .clone_from(&lane.subsurface_layers);
            day_frame.subsurface_compute_inputs.layers = lane
                .subsurface_layers
                .iter()
                .cloned()
                .map(Into::into)
                .collect();
        }
        day_frame.evapotranspiration_compute_inputs.stage_state =
            lane.evapotranspiration_stage_state.as_deref().copied();
        day_frame.winter_column.clone_from(&lane.winter_column);
        day_frame.snow_runtime_carry =
            direct_snow_runtime_carry_from_winter_state(&lane.winter_column.snow)
                .or_else(|| lane.snow_runtime_carry.as_deref().cloned());
        day_frame.frost_runtime_carry =
            direct_frost_runtime_carry_from_winter_state(&lane.winter_column.frost)
                .or_else(|| lane.frost_runtime_carry.clone());
        // SC-SED-001 1b-C: carry the persistent erosion state into the day
        // (advanced in the erosion span, committed back at day end).
        day_frame.erosion_runtime_carry = lane.erosion_runtime_carry;
        // E.3: the inter-OFE inflow intake is PER-DAY — cloned here and
        // CLEARED at commit so a stale intake can never leak into a later
        // day (the upstream publisher re-populates on days it routes).
        day_frame
            .erosion_inflow_intake
            .clone_from(&lane.erosion_inflow_intake);
        Ok(day_frame)
    }

    fn commit_day_frame(&mut self, day_frame: &DirectDayFrame) -> Result<(), DirectRuntimeError> {
        if day_frame.identity != self.identity {
            return Err(DirectRuntimeError::DirectDomainViolation {
                field: "day_frame.identity",
            });
        }
        if day_frame.day_index >= self.identity.day_count {
            return Err(DirectRuntimeError::DayIndexOutOfRange {
                day_index: day_frame.day_index,
                day_count: self.identity.day_count,
            });
        }
        let lane = self.lanes.get_mut(day_frame.lane_index).ok_or(
            DirectRuntimeError::LaneIndexOutOfRange {
                lane_index: day_frame.lane_index,
                lane_count: self.identity.lane_count,
            },
        )?;
        lane.commit_day(day_frame)?;
        DIRECT_AUDIT.record_day_frame_commit();
        Ok(())
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
    pub runoff_publication_q_scale: f64,
    pub runoff_publication_qofe_scale: f64,
    pub runoff_publication_efflen_m: f64,
    pub runoff_publication_cumulative_length_m: f64,
    pub runoff_publication_ofe_length_m: f64,
    pub water: DirectWaterState,
    pub transfer: DirectTransferBuffers,
    pub publication: DirectPublicationFrame,
    pub erosion_downstream_operands: DirectErosionDownstreamOperands,
    /// E.3: the inter-OFE erosion inflow published by the UPSTREAM lane's
    /// erosion span for the current day (copied into this lane's day frame
    /// at seeding; absent on OFE-1 / single-OFE lanes).
    pub erosion_inflow_intake: Option<Box<DirectErosionInflowIntake>>,
    pub subsurface_layers: Vec<DirectSubsurfaceLayerState>,
    pub evapotranspiration_stage_state: Option<Box<DirectEvapotranspirationStageState>>,
    pub plant_growth_state: Box<DirectGrowthStateSurface>,
    pub plant_water_stress: f64,
    pub winter_column: Box<DirectWinterColumnState>,
    pub snow_runtime_carry: Option<Box<DirectSnowRuntimeCarry>>,
    pub frost_runtime_carry: Option<DirectFrostRuntimeCarry>,
    /// SC-SED-001 1b-C persistent erosion carry (`rfcum`/`daydis`/`ifrost`/
    /// rill width), threaded day→day like the snow/frost carries.
    pub erosion_runtime_carry: DirectErosionRuntimeCarry,
    pub day_inputs: Vec<DirectDayConstructorInputs>,
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
            runoff_publication_q_scale: 1.0,
            runoff_publication_qofe_scale: 1.0,
            runoff_publication_efflen_m: 1.0,
            runoff_publication_cumulative_length_m: 1.0,
            runoff_publication_ofe_length_m: 1.0,
            water: DirectWaterState::zero(),
            transfer: DirectTransferBuffers::zero(),
            publication: DirectPublicationFrame::empty(),
            erosion_downstream_operands: DirectErosionDownstreamOperands::zero(),
            erosion_inflow_intake: None,
            subsurface_layers: Vec::new(),
            evapotranspiration_stage_state: None,
            plant_growth_state: Box::new(DirectGrowthStateSurface::zero()),
            plant_water_stress: 1.0,
            winter_column: Box::new(DirectWinterColumnState::zero()),
            snow_runtime_carry: None,
            frost_runtime_carry: None,
            erosion_runtime_carry: DirectErosionRuntimeCarry::inert(),
            day_inputs: Vec::new(),
        })
    }

    fn from_constructor_inputs(inputs: DirectLaneConstructorInputs) -> Self {
        let mut winter_column = *inputs.winter_column;
        if !winter_column.snow.has_runtime_state() {
            if let Some(carry) = inputs.snow_runtime_carry.clone() {
                winter_column.snow = carry.into();
            }
        }
        if !winter_column.frost.has_runtime_state() {
            if let Some(carry) = inputs.frost_runtime_carry.clone() {
                winter_column.frost = carry.into();
            }
        }
        let snow_runtime_carry = direct_snow_runtime_carry_from_winter_state(&winter_column.snow)
            .or(inputs.snow_runtime_carry)
            .map(Box::new);
        let frost_runtime_carry =
            direct_frost_runtime_carry_from_winter_state(&winter_column.frost)
                .or(inputs.frost_runtime_carry);
        Self {
            lane_id: inputs.lane_id,
            upstream_lane_id: inputs.upstream_lane_id,
            downstream_lane_id: inputs.downstream_lane_id,
            upstream_area_ratio: inputs.upstream_area_ratio,
            area_m2: inputs.area_m2,
            runoff_publication_q_scale: inputs.runoff_publication_q_scale,
            runoff_publication_qofe_scale: inputs.runoff_publication_qofe_scale,
            runoff_publication_efflen_m: inputs.runoff_publication_efflen_m,
            runoff_publication_cumulative_length_m: inputs.runoff_publication_cumulative_length_m,
            runoff_publication_ofe_length_m: inputs.runoff_publication_ofe_length_m,
            water: inputs.water,
            transfer: inputs.transfer,
            publication: inputs.publication,
            erosion_downstream_operands: DirectErosionDownstreamOperands::zero(),
            subsurface_layers: inputs.subsurface_layers,
            evapotranspiration_stage_state: inputs.evapotranspiration_stage_state,
            plant_growth_state: inputs.plant_growth_state,
            plant_water_stress: inputs.plant_water_stress,
            winter_column: Box::new(winter_column),
            snow_runtime_carry,
            frost_runtime_carry,
            // Consolidation age seeds inert (daydis = 0). Seeding from the
            // management `daydi1` is an enable-time adjudication item (it
            // ages the erodibility-adjustment factors); inert is faithful
            // for a freshly-disturbed start and inert behind the disabled
            // seed regardless.
            erosion_runtime_carry: DirectErosionRuntimeCarry::inert(),
            erosion_inflow_intake: None,
            day_inputs: inputs.day_inputs,
        }
    }

    fn commit_day(&mut self, day_frame: &DirectDayFrame) -> Result<(), DirectRuntimeError> {
        let expected_lane_index =
            usize::try_from(self.lane_id.saturating_sub(1)).map_err(|_| {
                DirectRuntimeError::LaneIdOverflow {
                    lane_index: day_frame.lane_index,
                }
            })?;
        if day_frame.lane_index != expected_lane_index {
            return Err(DirectRuntimeError::InvalidLaneTopology {
                lane_index: day_frame.lane_index,
                lane_id: self.lane_id,
                upstream_lane_id: self.upstream_lane_id,
                downstream_lane_id: self.downstream_lane_id,
            });
        }
        self.water = day_frame.water.clone();
        validate_nonnegative_direct_m(
            "lane_commit.storage_reconciled_m",
            day_frame.storage_reconciliation.storage_reconciled_m,
        )?;
        self.water.soil_water_m = day_frame.storage_reconciliation.storage_reconciled_m;
        self.transfer = day_frame.transfer.clone();
        self.publication = day_frame.publication.clone();
        self.erosion_downstream_operands = day_frame.erosion_downstream_operands.clone();
        let apply_coarse_frost_projection = !day_frame.winter_column.frost.has_runtime_state()
            && (day_frame.hydrology_projection.frost_depth_m > WB11_ZERO_THRESHOLD
                || day_frame.hydrology_projection.frozen_soil_water_m > WB11_ZERO_THRESHOLD);
        if !day_frame
            .evapotranspiration_compute
            .layer_state_after_root_uptake
            .is_empty()
        {
            self.subsurface_layers.clone_from(
                &day_frame
                    .evapotranspiration_compute
                    .layer_state_after_root_uptake,
            );
            if apply_coarse_frost_projection {
                apply_direct_frost_carry_projection(
                    &mut self.subsurface_layers,
                    day_frame.frost_layer_carry_projection.as_deref(),
                )?;
            }
        } else if !day_frame.subsurface_compute.layer_state_after.is_empty() {
            self.subsurface_layers
                .clone_from(&day_frame.subsurface_compute.layer_state_after);
            if apply_coarse_frost_projection {
                apply_direct_frost_carry_projection(
                    &mut self.subsurface_layers,
                    day_frame.frost_layer_carry_projection.as_deref(),
                )?;
            }
        } else if !day_frame.percolation.layer_state_after.is_empty() {
            self.subsurface_layers
                .clone_from(&day_frame.percolation.layer_state_after);
            if apply_coarse_frost_projection {
                apply_direct_frost_carry_projection(
                    &mut self.subsurface_layers,
                    day_frame.frost_layer_carry_projection.as_deref(),
                )?;
            }
        }
        self.evapotranspiration_stage_state = day_frame
            .evapotranspiration_surface
            .stage_state_after
            .map(Box::new);
        if day_frame.perennial_growth_inputs.active_context.is_active() {
            *self.plant_growth_state = day_frame.perennial_growth.state_after;
        } else if day_frame.annual_growth_inputs.active_context.is_active() {
            *self.plant_growth_state = day_frame.annual_growth.state_after;
        }
        self.plant_water_stress = day_frame.evapotranspiration_compute.water_stress;
        self.winter_column.clone_from(&day_frame.winter_column);
        self.snow_runtime_carry =
            direct_snow_runtime_carry_from_winter_state(&self.winter_column.snow)
                .or_else(|| day_frame.snow_runtime_carry.clone())
                .map(Box::new);
        self.frost_runtime_carry =
            direct_frost_runtime_carry_from_winter_state(&self.winter_column.frost)
                .or_else(|| day_frame.frost_runtime_carry.clone());
        // SC-SED-001 1b-C: persist the erosion carry advanced in the day's
        // erosion span (`rfcum`/`daydis`/`ifrost`/rill width) to the lane.
        self.erosion_runtime_carry = day_frame.erosion_runtime_carry;
        // E.3: the day consumed (or dropped) this lane's inflow intake.
        self.erosion_inflow_intake = None;
        Ok(())
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct DirectErosionDailyConsumers {
    pub canopy_cover_fraction: f64,
    pub canopy_height_m: f64,
    pub interrill_cover_fraction: f64,
    pub rill_cover_fraction: f64,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct DirectFrostDailyConsumers {
    pub residue_depth_m: f64,
    pub canopy_height_m: f64,
}

#[derive(Debug, Clone, PartialEq)]
pub struct DirectDayFrame {
    pub identity: DirectRunIdentity,
    pub lane_index: usize,
    pub day_index: usize,
    pub upstream_area_ratio: f64,
    pub forcing: DirectDayForcing,
    pub interception_m: f64,
    pub water: DirectWaterState,
    pub transfer: DirectTransferBuffers,
    /// DC01: WB14 per-hour infiltration-excess profile (INV-RUNOFFPART-031),
    /// set at R4K, consumed by the downstream surface-transfer publication.
    pub wb14_hourly_excess_m: [f64; 24],
    /// SC-SED-001 1b-C: WB14 per-hour RAINFALL depth (m), the parallel of
    /// `wb14_hourly_excess_m`, set at R4K. Feeds the erosion `effint`
    /// (mean rainfall intensity over excess periods); unread until the
    /// Wave-1 seed is enabled.
    pub wb14_hourly_rainfall_m: [f64; 24],
    /// SC-SED-001 1b-C: persistent erosion carry threaded from the lane,
    /// advanced in the erosion span, committed back at day end.
    pub erosion_runtime_carry: DirectErosionRuntimeCarry,
    /// E.3: the inter-OFE erosion inflow for THIS day (copied from the
    /// lane intake at day seeding; absent on OFE-1 / single-OFE lanes).
    pub erosion_inflow_intake: Option<Box<DirectErosionInflowIntake>>,
    /// ADR-0036 / INV-SED-013: the day's unit-normalized hourly runoff
    /// weights, set in the erosion span from the selected shape authority
    /// (default/off DC01 source weights, or D13 routed-hydrograph weights
    /// when explicitly selected). All-zero on no-runoff days.
    pub wave1_hourly_weights: [f64; 24],
    /// ADR-0036 / INV-SED-013: the per-hydraulically-active-hour Wave-1
    /// solve plan — `(hour, assembled continuity inputs)`. Empty on
    /// non-routed days; built in R7D8, consumed by the R7D6 solve.
    pub wave1_hourly_plan: Vec<(usize, DirectWave1ContinuityInputs)>,
    pub publication: DirectPublicationFrame,
    pub normalization_inputs: DirectNormalizationInputs,
    pub normalization: DirectNormalizationState,
    pub normalization_downstream_operands: DirectNormalizationDownstreamOperands,
    pub normalization_shadow_projection: Option<DirectNormalizationShadowProjection>,
    pub storage_bounds_inputs: DirectStorageBoundsInputs,
    pub storage_bounds: DirectStorageBoundsState,
    pub storage_bounds_downstream_operands: DirectStorageBoundsDownstreamOperands,
    pub storage_bounds_shadow_projection: Option<DirectStorageBoundsShadowProjection>,
    pub decomposition_inputs: DirectDecompositionInputs,
    pub decomposition: DirectDecompositionState,
    pub decomposition_downstream_operands: DirectDecompositionDownstreamOperands,
    pub decomposition_shadow_projection: Option<DirectDecompositionShadowProjection>,
    pub residue_partition_inputs: DirectResiduePartitionInputs,
    pub residue_partition: DirectResiduePartitionState,
    pub residue_partition_downstream_operands: DirectResiduePartitionDownstreamOperands,
    pub residue_partition_shadow_projection: Option<DirectResiduePartitionShadowProjection>,
    pub annual_growth_inputs: DirectGrowthInputs,
    pub annual_growth: DirectGrowthState,
    pub annual_growth_downstream_operands: DirectGrowthDownstreamOperands,
    pub annual_growth_shadow_projection: Option<DirectGrowthShadowProjection>,
    pub perennial_growth_inputs: DirectGrowthInputs,
    pub perennial_growth: DirectGrowthState,
    pub perennial_growth_downstream_operands: DirectGrowthDownstreamOperands,
    pub perennial_growth_shadow_projection: Option<DirectGrowthShadowProjection>,
    pub input_accounting: DirectInputAccountingState,
    pub downstream_operands: DirectDownstreamOperands,
    pub shadow_projection: Option<DirectShadowProjection>,
    pub storage_input_inputs: DirectStorageInputInputs,
    pub liquid_input_inputs: DirectLiquidInputInputs,
    pub liquid_input: DirectLiquidInputState,
    pub liquid_input_downstream_operands: DirectLiquidInputDownstreamOperands,
    pub liquid_input_shadow_projection: Option<DirectLiquidInputShadowProjection>,
    pub runon_carry_inputs: DirectRunonCarryInputs,
    pub runon_carry: DirectRunonCarryState,
    pub runon_carry_downstream_operands: DirectRunonCarryDownstreamOperands,
    pub runon_carry_shadow_projection: Option<DirectRunonCarryShadowProjection>,
    pub infiltration_depression_inputs: DirectInfiltrationDepressionInputs,
    pub infiltration_depression: DirectInfiltrationDepressionState,
    pub infiltration_depression_downstream_operands: DirectInfiltrationDepressionDownstreamOperands,
    pub infiltration_depression_shadow_projection:
        Option<DirectInfiltrationDepressionShadowProjection>,
    pub saturation_addback_inputs: DirectSaturationAddbackInputs,
    pub saturation_addback: DirectSaturationAddbackState,
    pub saturation_addback_downstream_operands: DirectSaturationAddbackDownstreamOperands,
    pub saturation_addback_shadow_projection: Option<DirectSaturationAddbackShadowProjection>,
    pub runoff_partition_inputs: DirectRunoffPartitionInputs,
    pub runoff_partition: DirectRunoffPartitionState,
    pub runoff_downstream_operands: DirectRunoffDownstreamOperands,
    pub runoff_shadow_projection: Option<DirectRunoffShadowProjection>,
    pub peak_runoff: DirectPeakRunoffState,
    pub peak_runoff_downstream_operands: DirectPeakRunoffDownstreamOperands,
    pub peak_runoff_shadow_projection: Option<DirectPeakRunoffShadowProjection>,
    pub percolation_inputs: DirectPercolationInputs,
    pub percolation: DirectPercolationState,
    pub percolation_downstream_operands: DirectPercolationDownstreamOperands,
    pub percolation_shadow_projection: Option<DirectPercolationShadowProjection>,
    pub subsurface_compute_inputs: DirectSubsurfaceComputeInputs,
    pub subsurface_compute: DirectSubsurfaceComputeState,
    pub subsurface_compute_downstream_operands: DirectSubsurfaceComputeDownstreamOperands,
    pub subsurface_compute_shadow_projection: Option<DirectSubsurfaceComputeShadowProjection>,
    pub storage_input: DirectStorageInputState,
    pub storage_input_downstream_operands: DirectStorageInputDownstreamOperands,
    pub storage_input_shadow_projection: Option<DirectStorageInputShadowProjection>,
    pub deep_seepage_inputs: DirectDeepSeepageInputs,
    pub deep_seepage: DirectDeepSeepageState,
    pub deep_seepage_downstream_operands: DirectDeepSeepageDownstreamOperands,
    pub deep_seepage_shadow_projection: Option<DirectDeepSeepageShadowProjection>,
    pub subsurface_loss_inputs: DirectSubsurfaceLossInputs,
    pub subsurface_loss: DirectSubsurfaceLossState,
    pub subsurface_loss_downstream_operands: DirectSubsurfaceLossDownstreamOperands,
    pub subsurface_loss_shadow_projection: Option<DirectSubsurfaceLossShadowProjection>,
    pub evapotranspiration_compute_inputs: DirectEvapotranspirationComputeInputs,
    pub evapotranspiration_surface: DirectEvapotranspirationSurfaceState,
    pub evapotranspiration_surface_downstream_operands:
        DirectEvapotranspirationSurfaceDownstreamOperands,
    pub evapotranspiration_surface_shadow_projection:
        Option<DirectEvapotranspirationSurfaceShadowProjection>,
    pub evapotranspiration_compute: DirectEvapotranspirationComputeState,
    pub evapotranspiration_compute_downstream_operands:
        DirectEvapotranspirationComputeDownstreamOperands,
    pub evapotranspiration_compute_shadow_projection:
        Option<DirectEvapotranspirationComputeShadowProjection>,
    pub evapotranspiration_inputs: DirectEvapotranspirationInputs,
    pub evapotranspiration: DirectEvapotranspirationState,
    pub evapotranspiration_downstream_operands: DirectEvapotranspirationDownstreamOperands,
    pub evapotranspiration_shadow_projection: Option<DirectEvapotranspirationShadowProjection>,
    pub snow_coupling_inputs: DirectSnowCouplingInputs,
    pub snow_coupling: DirectSnowCouplingState,
    pub snow_coupling_downstream_operands: DirectSnowCouplingDownstreamOperands,
    pub snow_coupling_shadow_projection: Option<Box<DirectSnowCouplingShadowProjection>>,
    pub storage_reconciliation_inputs: DirectStorageReconciliationInputs,
    pub storage_reconciliation: DirectStorageReconciliationState,
    pub storage_downstream_operands: DirectStorageDownstreamOperands,
    pub storage_shadow_projection: Option<DirectStorageShadowProjection>,
    pub frost_storage_liquid_delta_m: Option<f64>,
    pub hydrology_projection_inputs: DirectHydrologyProjectionInputs,
    pub hydrology_projection: DirectHydrologyProjectionState,
    pub hydrology_projection_downstream_operands: DirectHydrologyProjectionDownstreamOperands,
    pub hydrology_projection_shadow_projection: Option<DirectHydrologyProjectionShadowProjection>,
    pub groundwater_output: DirectGroundwaterDayOutput,
    pub prior_erosion_downstream_operands: DirectErosionDownstreamOperands,
    pub upstream_erosion_downstream_operands: DirectErosionDownstreamOperands,
    pub erosion_inputs: DirectErosionInputs,
    pub erosion: DirectErosionState,
    pub erosion_downstream_operands: DirectErosionDownstreamOperands,
    pub erosion_shadow_projection: Option<DirectErosionShadowProjection>,
    /// Exact values read by the active erosion daily-state assembly.
    pub erosion_daily_consumers: Option<DirectErosionDailyConsumers>,
    /// Exact values read by the active frost thermal compute.
    pub frost_daily_consumers: Option<DirectFrostDailyConsumers>,
    pub frost_layer_carry_projection: Option<Vec<DirectFrostLayerCarryProjection>>,
    pub winter_column: Box<DirectWinterColumnState>,
    pub snow_runtime_carry: Option<DirectSnowRuntimeCarry>,
    pub frost_runtime_carry: Option<DirectFrostRuntimeCarry>,
    // Single-solve authority (WP-2): carried frost outcome consumed by
    // the R4W ingress step; None on non-winter days.
    pub winter_frost_outcome: Option<Box<crate::hydrology::DirectWinterFrostPartitionOutcome>>,
    pub water_ledger: DirectWaterLedgerState,
    pub ledger_downstream_operands: DirectLedgerDownstreamOperands,
    pub ledger_shadow_projection: Option<DirectLedgerShadowProjection>,
    /// D15A (rev 27): per-lane-day routed evidence when the opt-in active
    /// owner routes this lane-day; `None` on the default path and on
    /// zero-source active days.
    pub laned_active_routing: Option<Box<laned_active::DirectLanedActiveDayRouting>>,
}

impl DirectDayFrame {
    #[allow(clippy::too_many_lines)]
    pub fn seed(
        identity: DirectRunIdentity,
        lane_index: usize,
        day_index: usize,
    ) -> Result<Self, DirectRuntimeError> {
        Self::validate_seed_indices(identity, lane_index, day_index)?;
        DIRECT_AUDIT.record_day_frame_construction();

        Ok(Self {
            identity,
            lane_index,
            day_index,
            upstream_area_ratio: 1.0,
            forcing: DirectDayForcing::zero(),
            interception_m: 0.0,
            water: DirectWaterState::zero(),
            transfer: DirectTransferBuffers::zero(),
            wb14_hourly_excess_m: [0.0; 24],
            wb14_hourly_rainfall_m: [0.0; 24],
            erosion_runtime_carry: DirectErosionRuntimeCarry::inert(),
            erosion_inflow_intake: None,
            wave1_hourly_weights: [0.0; 24],
            wave1_hourly_plan: Vec::new(),
            publication: DirectPublicationFrame::empty(),
            normalization_inputs: DirectNormalizationInputs::zero(),
            normalization: DirectNormalizationState::zero(),
            normalization_downstream_operands: DirectNormalizationDownstreamOperands::zero(),
            normalization_shadow_projection: None,
            storage_bounds_inputs: DirectStorageBoundsInputs::zero(),
            storage_bounds: DirectStorageBoundsState::zero(),
            storage_bounds_downstream_operands: DirectStorageBoundsDownstreamOperands::zero(),
            storage_bounds_shadow_projection: None,
            decomposition_inputs: DirectDecompositionInputs::zero(),
            decomposition: DirectDecompositionState::zero(),
            decomposition_downstream_operands: DirectDecompositionDownstreamOperands::zero(),
            decomposition_shadow_projection: None,
            residue_partition_inputs: DirectResiduePartitionInputs::zero(),
            residue_partition: DirectResiduePartitionState::zero(),
            residue_partition_downstream_operands: DirectResiduePartitionDownstreamOperands::zero(),
            residue_partition_shadow_projection: None,
            annual_growth_inputs: DirectGrowthInputs::zero(),
            annual_growth: DirectGrowthState::zero(),
            annual_growth_downstream_operands: DirectGrowthDownstreamOperands::zero(),
            annual_growth_shadow_projection: None,
            perennial_growth_inputs: DirectGrowthInputs::zero(),
            perennial_growth: DirectGrowthState::zero(),
            perennial_growth_downstream_operands: DirectGrowthDownstreamOperands::zero(),
            perennial_growth_shadow_projection: None,
            input_accounting: DirectInputAccountingState::zero(),
            downstream_operands: DirectDownstreamOperands::zero(),
            shadow_projection: None,
            storage_input_inputs: DirectStorageInputInputs::zero(),
            liquid_input_inputs: DirectLiquidInputInputs::zero(),
            liquid_input: DirectLiquidInputState::zero(),
            liquid_input_downstream_operands: DirectLiquidInputDownstreamOperands::zero(),
            liquid_input_shadow_projection: None,
            runon_carry_inputs: DirectRunonCarryInputs::zero(),
            runon_carry: DirectRunonCarryState::zero(),
            runon_carry_downstream_operands: DirectRunonCarryDownstreamOperands::zero(),
            runon_carry_shadow_projection: None,
            infiltration_depression_inputs: DirectInfiltrationDepressionInputs::zero(),
            infiltration_depression: DirectInfiltrationDepressionState::zero(),
            infiltration_depression_downstream_operands:
                DirectInfiltrationDepressionDownstreamOperands::zero(),
            infiltration_depression_shadow_projection: None,
            saturation_addback_inputs: DirectSaturationAddbackInputs::zero(),
            saturation_addback: DirectSaturationAddbackState::zero(),
            saturation_addback_downstream_operands: DirectSaturationAddbackDownstreamOperands::zero(
            ),
            saturation_addback_shadow_projection: None,
            runoff_partition_inputs: DirectRunoffPartitionInputs::zero(),
            runoff_partition: DirectRunoffPartitionState::zero(),
            runoff_downstream_operands: DirectRunoffDownstreamOperands::zero(),
            runoff_shadow_projection: None,
            peak_runoff: DirectPeakRunoffState::zero(),
            peak_runoff_downstream_operands: DirectPeakRunoffDownstreamOperands::zero(),
            peak_runoff_shadow_projection: None,
            percolation_inputs: DirectPercolationInputs::neutral(),
            percolation: DirectPercolationState::zero(),
            percolation_downstream_operands: DirectPercolationDownstreamOperands::zero(),
            percolation_shadow_projection: None,
            subsurface_compute_inputs: DirectSubsurfaceComputeInputs::neutral(),
            subsurface_compute: DirectSubsurfaceComputeState::zero(),
            subsurface_compute_downstream_operands: DirectSubsurfaceComputeDownstreamOperands::zero(
            ),
            subsurface_compute_shadow_projection: None,
            storage_input: DirectStorageInputState::zero(),
            storage_input_downstream_operands: DirectStorageInputDownstreamOperands::zero(),
            storage_input_shadow_projection: None,
            deep_seepage_inputs: DirectDeepSeepageInputs::zero(),
            deep_seepage: DirectDeepSeepageState::zero(),
            deep_seepage_downstream_operands: DirectDeepSeepageDownstreamOperands::zero(),
            deep_seepage_shadow_projection: None,
            subsurface_loss_inputs: DirectSubsurfaceLossInputs::zero(),
            subsurface_loss: DirectSubsurfaceLossState::zero(),
            subsurface_loss_downstream_operands: DirectSubsurfaceLossDownstreamOperands::zero(),
            subsurface_loss_shadow_projection: None,
            evapotranspiration_compute_inputs: DirectEvapotranspirationComputeInputs::zero(),
            evapotranspiration_surface: DirectEvapotranspirationSurfaceState::zero(),
            evapotranspiration_surface_downstream_operands:
                DirectEvapotranspirationSurfaceDownstreamOperands::zero(),
            evapotranspiration_surface_shadow_projection: None,
            evapotranspiration_compute: DirectEvapotranspirationComputeState::zero(),
            evapotranspiration_compute_downstream_operands:
                DirectEvapotranspirationComputeDownstreamOperands::zero(),
            evapotranspiration_compute_shadow_projection: None,
            evapotranspiration_inputs: DirectEvapotranspirationInputs::zero(),
            evapotranspiration: DirectEvapotranspirationState::zero(),
            evapotranspiration_downstream_operands:
                DirectEvapotranspirationDownstreamOperands::zero(),
            evapotranspiration_shadow_projection: None,
            snow_coupling_inputs: DirectSnowCouplingInputs::zero(),
            snow_coupling: DirectSnowCouplingState::zero(),
            snow_coupling_downstream_operands: DirectSnowCouplingDownstreamOperands::zero(),
            snow_coupling_shadow_projection: None,
            storage_reconciliation_inputs: DirectStorageReconciliationInputs::zero(),
            storage_reconciliation: DirectStorageReconciliationState::zero(),
            storage_downstream_operands: DirectStorageDownstreamOperands::zero(),
            storage_shadow_projection: None,
            frost_storage_liquid_delta_m: None,
            hydrology_projection_inputs: DirectHydrologyProjectionInputs::zero(),
            hydrology_projection: DirectHydrologyProjectionState::zero(),
            hydrology_projection_downstream_operands:
                DirectHydrologyProjectionDownstreamOperands::zero(),
            hydrology_projection_shadow_projection: None,
            groundwater_output: DirectGroundwaterDayOutput::zero(),
            prior_erosion_downstream_operands: DirectErosionDownstreamOperands::zero(),
            upstream_erosion_downstream_operands: DirectErosionDownstreamOperands::zero(),
            erosion_inputs: DirectErosionInputs::zero(),
            erosion: DirectErosionState::inactive(),
            erosion_downstream_operands: DirectErosionDownstreamOperands::zero(),
            erosion_shadow_projection: None,
            erosion_daily_consumers: None,
            frost_daily_consumers: None,
            frost_layer_carry_projection: None,
            winter_column: Box::new(DirectWinterColumnState::zero()),
            snow_runtime_carry: None,
            frost_runtime_carry: None,
            winter_frost_outcome: None,
            water_ledger: DirectWaterLedgerState::zero(),
            ledger_downstream_operands: DirectLedgerDownstreamOperands::zero(),
            ledger_shadow_projection: None,
            laned_active_routing: None,
        })
    }

    pub fn from_constructor_inputs(
        identity: DirectRunIdentity,
        lane_index: usize,
        day_index: usize,
        inputs: DirectDayConstructorInputs,
    ) -> Result<Self, DirectRuntimeError> {
        let mut frame = Self::seed(identity, lane_index, day_index)?;
        frame.apply_constructor_inputs(inputs)?;
        Ok(frame)
    }

    fn apply_constructor_inputs(
        &mut self,
        inputs: DirectDayConstructorInputs,
    ) -> Result<(), DirectRuntimeError> {
        validate_direct_day_constructor_inputs(&inputs)?;
        self.forcing = inputs.forcing;
        self.normalization_inputs = inputs.normalization_inputs;
        self.interception_m = inputs.interception_m;
        self.storage_bounds_inputs = inputs.storage_bounds_inputs;
        self.decomposition_inputs = inputs.decomposition_inputs;
        self.residue_partition_inputs = inputs.residue_partition_inputs;
        self.annual_growth_inputs = inputs.annual_growth_inputs;
        self.perennial_growth_inputs = inputs.perennial_growth_inputs;
        self.storage_input_inputs = inputs.storage_input_inputs;
        self.liquid_input_inputs = inputs.liquid_input_inputs;
        self.runon_carry_inputs = inputs.runon_carry_inputs;
        self.infiltration_depression_inputs = inputs.infiltration_depression_inputs;
        self.saturation_addback_inputs = inputs.saturation_addback_inputs;
        self.runoff_partition_inputs = inputs.runoff_partition_inputs;
        self.percolation_inputs = inputs.percolation_inputs;
        self.subsurface_compute_inputs = inputs.subsurface_compute_inputs;
        self.deep_seepage_inputs = inputs.deep_seepage_inputs;
        self.subsurface_loss_inputs = inputs.subsurface_loss_inputs;
        self.evapotranspiration_compute_inputs = inputs.evapotranspiration_compute_inputs;
        self.evapotranspiration_inputs = inputs.evapotranspiration_inputs;
        self.snow_coupling_inputs = inputs.snow_coupling_inputs;
        self.storage_reconciliation_inputs = inputs.storage_reconciliation_inputs;
        self.storage_reconciliation_inputs.interception_m = inputs.interception_m;
        self.frost_storage_liquid_delta_m = inputs.frost_storage_liquid_delta_m;
        self.winter_frost_outcome = inputs.winter_frost_outcome;
        self.hydrology_projection_inputs = inputs.hydrology_projection_inputs;
        self.erosion_inputs = inputs.erosion_inputs;
        self.frost_layer_carry_projection = inputs.frost_layer_carry_projection;
        if let Some(carry) = inputs.snow_runtime_carry.as_deref() {
            self.winter_column.snow = carry.clone().into();
        }
        self.snow_runtime_carry =
            direct_snow_runtime_carry_from_winter_state(&self.winter_column.snow)
                .or_else(|| inputs.snow_runtime_carry.map(|carry| *carry));
        if let Some(carry) = inputs.frost_runtime_carry.clone() {
            self.winter_column.frost = carry.into();
        }
        self.frost_runtime_carry =
            direct_frost_runtime_carry_from_winter_state(&self.winter_column.frost)
                .or(inputs.frost_runtime_carry);
        Ok(())
    }

    fn validate_seed_indices(
        identity: DirectRunIdentity,
        lane_index: usize,
        day_index: usize,
    ) -> Result<(), DirectRuntimeError> {
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
        Ok(())
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

        let raw_surface_transfer_m =
            sum_nonnegative_direct_m("transfer.surface_carry_m", &self.transfer.surface_carry_m)?;
        let raw_lateral_transfer_m =
            sum_nonnegative_direct_m("transfer.lateral_carry_m", &self.transfer.lateral_carry_m)?;
        let surface_transfer_m = scaled_direct_transfer_total_m(
            "input_accounting.surface_transfer_m",
            raw_surface_transfer_m,
            self.upstream_area_ratio,
        )?;
        let lateral_transfer_m = scaled_direct_transfer_total_m(
            "input_accounting.lateral_transfer_m",
            raw_lateral_transfer_m,
            self.upstream_area_ratio,
        )?;
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
}

fn validate_direct_run_constructor_inputs(
    inputs: &DirectRunConstructorInputs,
) -> Result<(), DirectRuntimeError> {
    if inputs.identity.lane_count == 0 {
        return Err(DirectRuntimeError::InvalidLaneCount {
            lane_count: inputs.identity.lane_count,
        });
    }
    if inputs.identity.day_count == 0 {
        return Err(DirectRuntimeError::InvalidDayCount {
            day_count: inputs.identity.day_count,
        });
    }
    if inputs.lanes.len() != inputs.identity.lane_count {
        return Err(DirectRuntimeError::FrameLaneCountMismatch {
            identity_lane_count: inputs.identity.lane_count,
            actual_lane_count: inputs.lanes.len(),
        });
    }

    for (lane_index, lane) in inputs.lanes.iter().enumerate() {
        validate_direct_lane_constructor_inputs(inputs.identity, lane_index, lane)?;
    }
    validate_direct_constructor_topology(&inputs.lanes)
}

fn validate_direct_lane_constructor_inputs(
    identity: DirectRunIdentity,
    lane_index: usize,
    inputs: &DirectLaneConstructorInputs,
) -> Result<(), DirectRuntimeError> {
    let expected_lane_id = u32::try_from(lane_index + 1)
        .map_err(|_| DirectRuntimeError::LaneIdOverflow { lane_index })?;
    if inputs.lane_id != expected_lane_id {
        return Err(DirectRuntimeError::InvalidLaneTopology {
            lane_index,
            lane_id: inputs.lane_id,
            upstream_lane_id: inputs.upstream_lane_id,
            downstream_lane_id: inputs.downstream_lane_id,
        });
    }
    validate_positive_direct("constructor.area_m2", inputs.area_m2)?;
    validate_nonnegative_direct_m(
        "constructor.upstream_area_ratio",
        inputs.upstream_area_ratio,
    )?;
    validate_positive_direct(
        "constructor.runoff_publication_q_scale",
        inputs.runoff_publication_q_scale,
    )?;
    validate_positive_direct(
        "constructor.runoff_publication_qofe_scale",
        inputs.runoff_publication_qofe_scale,
    )?;
    validate_positive_direct(
        "constructor.runoff_publication_efflen_m",
        inputs.runoff_publication_efflen_m,
    )?;
    validate_positive_direct(
        "constructor.runoff_publication_cumulative_length_m",
        inputs.runoff_publication_cumulative_length_m,
    )?;
    validate_positive_direct(
        "constructor.runoff_publication_ofe_length_m",
        inputs.runoff_publication_ofe_length_m,
    )?;
    validate_direct_water_state(&inputs.water)?;
    validate_direct_transfer_buffers(&inputs.transfer)?;
    validate_direct_publication_frame(&inputs.publication)?;
    for layer in &inputs.subsurface_layers {
        validate_direct_subsurface_layer(layer)?;
    }
    if let Some(stage) = inputs.evapotranspiration_stage_state.as_deref().copied() {
        validate_direct_evapotranspiration_stage(stage)?;
    }
    validate_direct_growth_state_surface(
        "constructor.plant_growth_state",
        *inputs.plant_growth_state,
    )?;
    validate_unit_interval("constructor.plant_water_stress", inputs.plant_water_stress)?;
    validate_direct_snow_lane_state("constructor.winter_column.snow", &inputs.winter_column.snow)?;
    if let Some(carry) = &inputs.snow_runtime_carry {
        validate_direct_snow_runtime_carry(carry)?;
    }
    if !(inputs.day_inputs.is_empty() || inputs.day_inputs.len() == identity.day_count) {
        return Err(DirectRuntimeError::DirectDomainViolation {
            field: "constructor.day_inputs",
        });
    }
    for day_inputs in &inputs.day_inputs {
        validate_direct_day_constructor_inputs(day_inputs)?;
    }
    Ok(())
}

fn validate_direct_constructor_topology(
    lanes: &[DirectLaneConstructorInputs],
) -> Result<(), DirectRuntimeError> {
    let lane_count_u32 =
        u32::try_from(lanes.len()).map_err(|_| DirectRuntimeError::LaneIdOverflow {
            lane_index: lanes.len(),
        })?;
    let mut outlet_count = 0_usize;

    for (lane_index, lane) in lanes.iter().enumerate() {
        if lane.upstream_lane_id > lane_count_u32 || lane.downstream_lane_id > lane_count_u32 {
            return Err(DirectRuntimeError::InvalidLaneTopology {
                lane_index,
                lane_id: lane.lane_id,
                upstream_lane_id: lane.upstream_lane_id,
                downstream_lane_id: lane.downstream_lane_id,
            });
        }
        if lane.downstream_lane_id == 0 {
            outlet_count += 1;
        }
        if lane.upstream_lane_id != 0 {
            let upstream_index = (lane.upstream_lane_id - 1) as usize;
            if lanes[upstream_index].downstream_lane_id != lane.lane_id {
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
            if lanes[downstream_index].upstream_lane_id != lane.lane_id {
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
        Ok(())
    } else {
        Err(DirectRuntimeError::InvalidLaneOutletCount { outlet_count })
    }
}

fn validate_direct_day_constructor_inputs(
    inputs: &DirectDayConstructorInputs,
) -> Result<(), DirectRuntimeError> {
    validate_direct_day_forcing_constructor_inputs(inputs)?;
    validate_direct_day_runoff_constructor_inputs(inputs)?;
    validate_direct_day_storage_constructor_inputs(inputs)?;
    validate_direct_day_projection_constructor_inputs(inputs)?;
    validate_direct_evapotranspiration_compute_inputs(&inputs.evapotranspiration_compute_inputs)?;
    validate_direct_day_layer_constructor_inputs(inputs)?;
    validate_direct_frost_constructor_inputs(inputs)?;
    Ok(())
}

fn validate_direct_day_forcing_constructor_inputs(
    inputs: &DirectDayConstructorInputs,
) -> Result<(), DirectRuntimeError> {
    validate_nonnegative_direct_m(
        "constructor.forcing.precipitation_m",
        inputs.forcing.precipitation_m,
    )?;
    validate_finite(
        "constructor.forcing.effective_temperature_c",
        inputs.forcing.effective_temperature_c,
    )?;
    validate_nonnegative_direct_m("constructor.interception_m", inputs.interception_m)?;
    validate_nonnegative_direct_m(
        "constructor.normalization.precipitation_m",
        inputs.normalization_inputs.precipitation_m,
    )?;
    validate_finite(
        "constructor.normalization.effective_temperature_c",
        inputs.normalization_inputs.effective_temperature_c,
    )?;
    validate_nonnegative_direct_m(
        "constructor.storage_bounds.closure_tolerance_m",
        inputs.storage_bounds_inputs.closure_tolerance_m,
    )?;
    Ok(())
}

fn validate_direct_day_runoff_constructor_inputs(
    inputs: &DirectDayConstructorInputs,
) -> Result<(), DirectRuntimeError> {
    if let Some(precip_input_handoff_m) = inputs.storage_input_inputs.precip_input_handoff_m {
        validate_nonnegative_direct_m(
            "constructor.storage_input.precip_input_handoff_m",
            precip_input_handoff_m,
        )?;
    }
    validate_nonnegative_direct_m(
        "constructor.liquid_input_handoff_m",
        inputs.liquid_input_inputs.liquid_input_handoff_m,
    )?;
    validate_nonnegative_direct_m(
        "constructor.runon.surface_runon_handoff_m",
        inputs.runon_carry_inputs.surface_runon_handoff_m,
    )?;
    validate_nonnegative_direct_m(
        "constructor.runon.subsurface_carry_handoff_m",
        inputs.runon_carry_inputs.subsurface_carry_handoff_m,
    )?;
    validate_nonnegative_direct_m(
        "constructor.runoff.liquid_input_m",
        inputs.runoff_partition_inputs.liquid_input_m,
    )?;
    validate_nonnegative_direct_m(
        "constructor.percolation.soil_water_initial_m",
        inputs.percolation_inputs.soil_water_initial_m,
    )?;
    Ok(())
}

fn validate_direct_day_storage_constructor_inputs(
    inputs: &DirectDayConstructorInputs,
) -> Result<(), DirectRuntimeError> {
    validate_nonnegative_direct_m(
        "constructor.deep_seepage_handoff_m",
        inputs.deep_seepage_inputs.deep_seepage_handoff_m,
    )?;
    validate_nonnegative_direct_m(
        "constructor.subsurface_loss_handoff_m",
        inputs.subsurface_loss_inputs.subsurface_loss_handoff_m,
    )?;
    validate_nonnegative_direct_m(
        "constructor.evapotranspiration_handoff_m",
        inputs
            .evapotranspiration_inputs
            .evapotranspiration_handoff_m,
    )?;
    validate_nonnegative_direct_m(
        "constructor.snow_coupling_handoff_m",
        inputs.snow_coupling_inputs.snow_coupling_handoff_m,
    )?;
    validate_nonnegative_direct_m(
        "constructor.storage_reconciliation.closure_tolerance_m",
        inputs.storage_reconciliation_inputs.closure_tolerance_m,
    )?;
    validate_nonnegative_direct_m(
        "constructor.storage_reconciliation.interception_m",
        inputs.storage_reconciliation_inputs.interception_m,
    )?;
    validate_finite(
        "constructor.storage_reconciliation.frost_liquid_delta_m",
        inputs.storage_reconciliation_inputs.frost_liquid_delta_m,
    )?;
    validate_optional_finite_direct_m(
        "constructor.frost_storage_liquid_delta_m",
        inputs.frost_storage_liquid_delta_m,
    )?;
    Ok(())
}

fn validate_direct_day_projection_constructor_inputs(
    inputs: &DirectDayConstructorInputs,
) -> Result<(), DirectRuntimeError> {
    validate_nonnegative_direct_m(
        "constructor.hydrology_projection.aggregate_storage_tolerance_m",
        inputs
            .hydrology_projection_inputs
            .aggregate_storage_tolerance_m,
    )?;
    validate_optional_nonnegative_direct_m(
        "constructor.hydrology_projection.profile_depth_m",
        inputs.hydrology_projection_inputs.profile_depth_m,
    )?;
    Ok(())
}

fn validate_direct_day_layer_constructor_inputs(
    inputs: &DirectDayConstructorInputs,
) -> Result<(), DirectRuntimeError> {
    for layer in &inputs.percolation_inputs.layers {
        validate_direct_subsurface_layer(layer)?;
    }
    for layer in &inputs.subsurface_compute_inputs.layers {
        validate_direct_subsurface_layer_inputs(layer)?;
    }
    Ok(())
}

fn validate_direct_frost_constructor_inputs(
    inputs: &DirectDayConstructorInputs,
) -> Result<(), DirectRuntimeError> {
    if let Some(projection) = &inputs.frost_layer_carry_projection {
        for projection in projection {
            validate_positive_direct(
                "constructor.frost_layer_carry_projection.fine_layer_thickness_m",
                projection.fine_layer_thickness_m,
            )?;
            if projection.fine_layer_count == 0 {
                return Err(DirectRuntimeError::DirectDomainViolation {
                    field: "constructor.frost_layer_carry_projection.fine_layer_count",
                });
            }
        }
    }
    if let Some(carry) = &inputs.frost_runtime_carry {
        validate_direct_frost_runtime_carry(carry)?;
    }
    if let Some(carry) = &inputs.snow_runtime_carry {
        validate_direct_snow_runtime_carry(carry)?;
    }
    Ok(())
}

fn validate_direct_snow_runtime_carry(
    carry: &DirectSnowRuntimeCarry,
) -> Result<(), DirectRuntimeError> {
    validate_direct_snow_lane_state("constructor.snow_runtime_carry", &carry.clone().into())
}

fn validate_direct_snow_lane_state(
    prefix: &'static str,
    state: &DirectSnowLaneState,
) -> Result<(), DirectRuntimeError> {
    for (field, value) in [
        ("runtime_swe_m", state.runtime_swe_m),
        ("runtime_depth_m", state.runtime_depth_m),
        ("runtime_density_kg_m3", state.runtime_density_kg_m3),
        ("runtime_settle_day_count", state.runtime_settle_day_count),
        ("coe_boundary_depth_m", state.coe_boundary_depth_m),
        (
            "coe_boundary_density_kg_m3",
            state.coe_boundary_density_kg_m3,
        ),
        (
            "coe_boundary_settle_day_count",
            state.coe_boundary_settle_day_count,
        ),
        ("liquid_water_retained_m", state.liquid_water_retained_m),
    ] {
        validate_nonnegative_direct_m(direct_snow_lane_validation_field(prefix, field), value)?;
    }
    if state.runtime_density_kg_m3 > 522.0 {
        return Err(DirectRuntimeError::DirectDomainViolation {
            field: direct_snow_lane_validation_field(prefix, "runtime_density_kg_m3"),
        });
    }
    if state.coe_boundary_density_kg_m3 > 522.0 {
        return Err(DirectRuntimeError::DirectDomainViolation {
            field: direct_snow_lane_validation_field(prefix, "coe_boundary_density_kg_m3"),
        });
    }
    if let Some(snow_albedo_state) = state.snow_albedo_state {
        snow_albedo_state
            .validate()
            .map_err(|_| DirectRuntimeError::DirectDomainViolation {
                field: direct_snow_lane_validation_field(prefix, "snow_albedo_state"),
            })?;
    }
    validate_direct_snow_layers(prefix, state)?;
    Ok(())
}

fn validate_direct_snow_layers(
    prefix: &'static str,
    state: &DirectSnowLaneState,
) -> Result<(), DirectRuntimeError> {
    const LAYER_CLOSURE_TOLERANCE_M: f64 = 1.0e-9;

    if state.layers.is_empty() {
        return Ok(());
    }

    let mut layer_swe_sum_m = 0.0;
    let mut layer_depth_sum_m = 0.0;
    for layer in &state.layers {
        for (field, value) in [
            ("layers.mass_swe_m", layer.mass_swe_m),
            ("layers.thickness_m", layer.thickness_m),
            ("layers.density_kg_m3", layer.density_kg_m3),
            ("layers.settle_day_count", layer.settle_day_count),
        ] {
            validate_nonnegative_direct_m(direct_snow_lane_validation_field(prefix, field), value)?;
        }
        if layer.density_kg_m3 > 522.0 {
            return Err(DirectRuntimeError::DirectDomainViolation {
                field: direct_snow_lane_validation_field(prefix, "layers.density_kg_m3"),
            });
        }
        layer_swe_sum_m += layer.mass_swe_m;
        layer_depth_sum_m += layer.thickness_m;
    }

    if (layer_swe_sum_m - state.runtime_swe_m).abs() > LAYER_CLOSURE_TOLERANCE_M {
        return Err(DirectRuntimeError::DirectDomainViolation {
            field: direct_snow_lane_validation_field(prefix, "layers.mass_swe_m"),
        });
    }
    if (layer_depth_sum_m - state.runtime_depth_m).abs() > LAYER_CLOSURE_TOLERANCE_M {
        return Err(DirectRuntimeError::DirectDomainViolation {
            field: direct_snow_lane_validation_field(prefix, "layers.thickness_m"),
        });
    }
    if state.runtime_swe_m <= LAYER_CLOSURE_TOLERANCE_M
        || state.runtime_depth_m <= LAYER_CLOSURE_TOLERANCE_M
    {
        return Err(DirectRuntimeError::DirectDomainViolation {
            field: direct_snow_lane_validation_field(prefix, "layers"),
        });
    }
    Ok(())
}

fn direct_snow_lane_validation_field(prefix: &'static str, field: &'static str) -> &'static str {
    match (prefix, field) {
        ("constructor.winter_column.snow", "runtime_swe_m") => {
            "constructor.winter_column.snow.runtime_swe_m"
        }
        ("constructor.winter_column.snow", "runtime_depth_m") => {
            "constructor.winter_column.snow.runtime_depth_m"
        }
        ("constructor.winter_column.snow", "runtime_density_kg_m3") => {
            "constructor.winter_column.snow.runtime_density_kg_m3"
        }
        ("constructor.winter_column.snow", "runtime_settle_day_count") => {
            "constructor.winter_column.snow.runtime_settle_day_count"
        }
        ("constructor.winter_column.snow", "coe_boundary_depth_m") => {
            "constructor.winter_column.snow.coe_boundary_depth_m"
        }
        ("constructor.winter_column.snow", "coe_boundary_density_kg_m3") => {
            "constructor.winter_column.snow.coe_boundary_density_kg_m3"
        }
        ("constructor.winter_column.snow", "coe_boundary_settle_day_count") => {
            "constructor.winter_column.snow.coe_boundary_settle_day_count"
        }
        ("constructor.winter_column.snow", "snow_albedo_state") => {
            "constructor.winter_column.snow.snow_albedo_state"
        }
        ("constructor.snow_runtime_carry", "runtime_swe_m") => {
            "constructor.snow_runtime_carry.runtime_swe_m"
        }
        ("constructor.snow_runtime_carry", "runtime_depth_m") => {
            "constructor.snow_runtime_carry.runtime_depth_m"
        }
        ("constructor.snow_runtime_carry", "runtime_density_kg_m3") => {
            "constructor.snow_runtime_carry.runtime_density_kg_m3"
        }
        ("constructor.snow_runtime_carry", "runtime_settle_day_count") => {
            "constructor.snow_runtime_carry.runtime_settle_day_count"
        }
        ("constructor.snow_runtime_carry", "coe_boundary_depth_m") => {
            "constructor.snow_runtime_carry.coe_boundary_depth_m"
        }
        ("constructor.snow_runtime_carry", "coe_boundary_density_kg_m3") => {
            "constructor.snow_runtime_carry.coe_boundary_density_kg_m3"
        }
        ("constructor.snow_runtime_carry", "coe_boundary_settle_day_count") => {
            "constructor.snow_runtime_carry.coe_boundary_settle_day_count"
        }
        ("constructor.snow_runtime_carry", "snow_albedo_state") => {
            "constructor.snow_runtime_carry.snow_albedo_state"
        }
        _ => "constructor.snow_lane_state",
    }
}

fn validate_direct_frost_runtime_carry(
    carry: &DirectFrostRuntimeCarry,
) -> Result<(), DirectRuntimeError> {
    validate_direct_frost_runtime_scalar_carry(carry)?;
    validate_direct_frost_runtime_layer_shadows(carry)?;
    validate_direct_frost_runtime_fine_layers(carry)
}

fn validate_direct_frost_runtime_scalar_carry(
    carry: &DirectFrostRuntimeCarry,
) -> Result<(), DirectRuntimeError> {
    for (field, value) in [
        ("constructor.frost_runtime_carry.dfrost_m", carry.dfrost_m),
        ("constructor.frost_runtime_carry.dthaw_m", carry.dthaw_m),
        ("constructor.frost_runtime_carry.nft", carry.nft),
        ("constructor.frost_runtime_carry.ws_frz_m", carry.ws_frz_m),
        (
            "constructor.frost_runtime_carry.infcap_frz_m_s",
            carry.infcap_frz_m_s,
        ),
        (
            "constructor.frost_runtime_carry.frwatc_soil_water_before_m",
            carry.frwatc_soil_water_before_m,
        ),
        (
            "constructor.frost_runtime_carry.frwatc_soil_water_after_m",
            carry.frwatc_soil_water_after_m,
        ),
        (
            "constructor.frost_runtime_carry.frwatc_frozen_water_before_m",
            carry.frwatc_frozen_water_before_m,
        ),
        (
            "constructor.frost_runtime_carry.frwatc_frozen_water_after_m",
            carry.frwatc_frozen_water_after_m,
        ),
        (
            "constructor.frost_runtime_carry.frwatc_freeze_debit_m",
            carry.frwatc_freeze_debit_m,
        ),
        (
            "constructor.frost_runtime_carry.frwatc_thaw_credit_m",
            carry.frwatc_thaw_credit_m,
        ),
        ("constructor.frost_runtime_carry.frdp_m", carry.frdp_m),
        ("constructor.frost_runtime_carry.thdp_m", carry.thdp_m),
        ("constructor.frost_runtime_carry.tfrdp_m", carry.tfrdp_m),
        ("constructor.frost_runtime_carry.tthawd_m", carry.tthawd_m),
        (
            "constructor.frost_runtime_carry.fgthwd_flag",
            carry.fgthwd_flag,
        ),
        (
            "constructor.frost_runtime_carry.total_fine_layer_count",
            carry.total_fine_layer_count,
        ),
        (
            "constructor.frost_runtime_carry.conductivity_tilled_w_m_k",
            carry.conductivity_tilled_w_m_k,
        ),
        (
            "constructor.frost_runtime_carry.conductivity_untilled_w_m_k",
            carry.conductivity_untilled_w_m_k,
        ),
        (
            "constructor.frost_runtime_carry.conductivity_residue_w_m_k",
            carry.conductivity_residue_w_m_k,
        ),
        (
            "constructor.frost_runtime_carry.shadow_total_water_before_m",
            carry.shadow_total_water_before_m,
        ),
        (
            "constructor.frost_runtime_carry.shadow_total_water_after_m",
            carry.shadow_total_water_after_m,
        ),
        ("constructor.frost_runtime_carry.watpdg_m", carry.watpdg_m),
        ("constructor.frost_runtime_carry.watbtm_m", carry.watbtm_m),
    ] {
        validate_nonnegative_direct_m(field, value)?;
    }
    for (field, value) in [
        (
            "constructor.frost_runtime_carry.frwatc_net_liquid_delta_m",
            carry.frwatc_net_liquid_delta_m,
        ),
        (
            "constructor.frost_runtime_carry.shadow_wb_delta_m",
            carry.shadow_wb_delta_m,
        ),
        (
            "constructor.frost_runtime_carry.shadow_frwatc_residual_m",
            carry.shadow_frwatc_residual_m,
        ),
    ] {
        validate_finite(field, value)?;
    }
    Ok(())
}

fn validate_direct_frost_runtime_layer_shadows(
    carry: &DirectFrostRuntimeCarry,
) -> Result<(), DirectRuntimeError> {
    for layer in &carry.layer_shadows {
        if layer.layer_index == 0 {
            return Err(DirectRuntimeError::DirectDomainViolation {
                field: "constructor.frost_runtime_carry.layer_shadow.layer_index",
            });
        }
        for (field, value) in [
            (
                "constructor.frost_runtime_carry.layer_shadow.st_m",
                layer.st_m,
            ),
            (
                "constructor.frost_runtime_carry.layer_shadow.soil_water_m",
                layer.soil_water_m,
            ),
            (
                "constructor.frost_runtime_carry.layer_shadow.frozen_depth_m",
                layer.frozen_depth_m,
            ),
            (
                "constructor.frost_runtime_carry.layer_shadow.frozen_water_m",
                layer.frozen_water_m,
            ),
            (
                "constructor.frost_runtime_carry.layer_shadow.soilf_m",
                layer.soilf_m,
            ),
            (
                "constructor.frost_runtime_carry.layer_shadow.yst_m",
                layer.yst_m,
            ),
            (
                "constructor.frost_runtime_carry.layer_shadow.nwfrzz_m",
                layer.nwfrzz_m,
            ),
        ] {
            validate_nonnegative_direct_m(field, value)?;
        }
    }
    Ok(())
}

fn validate_direct_frost_runtime_fine_layers(
    carry: &DirectFrostRuntimeCarry,
) -> Result<(), DirectRuntimeError> {
    for fine in &carry.fine_layers {
        if fine.layer_index == 0 {
            return Err(DirectRuntimeError::DirectDomainViolation {
                field: "constructor.frost_runtime_carry.fine_layer.layer_index",
            });
        }
        if fine.fine_index == 0 {
            return Err(DirectRuntimeError::DirectDomainViolation {
                field: "constructor.frost_runtime_carry.fine_layer.fine_index",
            });
        }
        for (field, value) in [
            (
                "constructor.frost_runtime_carry.fine_layer.fgfrst",
                fine.fgfrst,
            ),
            (
                "constructor.frost_runtime_carry.fine_layer.slfsd_m",
                fine.slfsd_m,
            ),
            (
                "constructor.frost_runtime_carry.fine_layer.slsic_m",
                fine.slsic_m,
            ),
            (
                "constructor.frost_runtime_carry.fine_layer.slsw_theta",
                fine.slsw_theta,
            ),
            (
                "constructor.frost_runtime_carry.fine_layer.sltime_s",
                fine.sltime_s,
            ),
        ] {
            validate_nonnegative_direct_m(field, value)?;
        }
    }
    Ok(())
}

fn validate_direct_evapotranspiration_compute_inputs(
    inputs: &DirectEvapotranspirationComputeInputs,
) -> Result<(), DirectRuntimeError> {
    validate_nonnegative_direct_m("constructor.et.et_demand_m", inputs.et_demand_m)?;
    validate_nonnegative_direct_m("constructor.et.leaf_area_index", inputs.leaf_area_index)?;
    validate_nonnegative_direct_m("constructor.et.canopy_height_m", inputs.canopy_height_m)?;
    validate_unit_interval(
        "constructor.et.canopy_cover_fraction",
        inputs.canopy_cover_fraction,
    )?;
    validate_nonnegative_direct_m(
        "constructor.et.residue_interception_m",
        inputs.residue_interception_m,
    )?;
    validate_nonnegative_direct_m(
        "constructor.et.same_pass_infiltration_m",
        inputs.same_pass_infiltration_m,
    )?;
    validate_nonnegative_direct_m(
        "constructor.et.outside_water_depth_m",
        inputs.outside_water_depth_m,
    )?;
    validate_nonnegative_direct_m("constructor.et.root_depth_m", inputs.root_depth_m)?;
    validate_finite("constructor.et.plant_tolerance", inputs.plant_tolerance)?;
    if let Some(stage) = inputs.stage_state {
        validate_direct_evapotranspiration_stage(stage)?;
    }
    if let Some(pmet) = inputs.pmet {
        validate_nonnegative_direct_m(
            "constructor.pmet.soil_evaporation_m",
            pmet.soil_evaporation_m,
        )?;
        validate_nonnegative_direct_m(
            "constructor.pmet.plant_transpiration_m",
            pmet.plant_transpiration_m,
        )?;
        validate_nonnegative_direct_m(
            "constructor.pmet.soil_evaporation_storage_return_m",
            pmet.soil_evaporation_storage_return_m,
        )?;
    }
    Ok(())
}

fn validate_direct_growth_state_surface(
    field_root: &'static str,
    inputs: DirectGrowthStateSurface,
) -> Result<(), DirectRuntimeError> {
    validate_nonnegative_direct_m(field_root, inputs.sumgdd)?;
    validate_nonnegative_direct_m(field_root, inputs.live_biomass_kg_m2)?;
    validate_nonnegative_direct_m(field_root, inputs.interception_live_biomass_kg_m2)?;
    validate_unit_interval(field_root, inputs.canopy_cover_fraction)?;
    validate_nonnegative_direct_m(field_root, inputs.leaf_area_index)?;
    validate_nonnegative_direct_m(field_root, inputs.root_mass_kg_m2)?;
    validate_nonnegative_direct_m(field_root, inputs.root_depth_m)?;
    validate_unit_interval(field_root, inputs.harvest_index)
}

fn validate_direct_water_state(inputs: &DirectWaterState) -> Result<(), DirectRuntimeError> {
    validate_nonnegative_direct_m("constructor.water.soil_water_m", inputs.soil_water_m)?;
    validate_nonnegative_direct_m("constructor.water.infiltration_m", inputs.infiltration_m)?;
    validate_nonnegative_direct_m("constructor.water.runoff_m", inputs.runoff_m)?;
    validate_nonnegative_direct_m(
        "constructor.water.evapotranspiration_m",
        inputs.evapotranspiration_m,
    )?;
    validate_nonnegative_direct_m("constructor.water.drainage_m", inputs.drainage_m)?;
    validate_nonnegative_direct_m("constructor.water.lateral_flow_m", inputs.lateral_flow_m)
}

fn validate_direct_transfer_buffers(
    inputs: &DirectTransferBuffers,
) -> Result<(), DirectRuntimeError> {
    let _ = sum_nonnegative_direct_m(
        "constructor.transfer.surface_carry_m",
        &inputs.surface_carry_m,
    )?;
    let _ = sum_nonnegative_direct_m(
        "constructor.transfer.lateral_carry_m",
        &inputs.lateral_carry_m,
    )?;
    validate_nonnegative_direct_m(
        "constructor.transfer.upstream_flow_m",
        inputs.upstream_flow_m,
    )?;
    validate_nonnegative_direct_m(
        "constructor.transfer.subsurface_input_m",
        inputs.subsurface_input_m,
    )
}

fn validate_direct_publication_frame(
    inputs: &DirectPublicationFrame,
) -> Result<(), DirectRuntimeError> {
    validate_nonnegative_direct_m("constructor.publication.runoff_m", inputs.runoff_m)?;
    validate_nonnegative_direct_m(
        "constructor.publication.infiltration_m",
        inputs.infiltration_m,
    )?;
    validate_nonnegative_direct_m(
        "constructor.publication.evapotranspiration_m",
        inputs.evapotranspiration_m,
    )?;
    validate_nonnegative_direct_m("constructor.publication.drainage_m", inputs.drainage_m)?;
    validate_nonnegative_direct_m(
        "constructor.publication.lateral_flow_m",
        inputs.lateral_flow_m,
    )
}

fn validate_direct_subsurface_layer(
    layer: &DirectSubsurfaceLayerState,
) -> Result<(), DirectRuntimeError> {
    validate_direct_subsurface_layer_inputs(&DirectSubsurfaceLayerInputs::from(layer.clone()))
}

fn validate_direct_subsurface_layer_inputs(
    layer: &DirectSubsurfaceLayerInputs,
) -> Result<(), DirectRuntimeError> {
    validate_nonnegative_direct_m("constructor.layer.theta_m", layer.theta_m)?;
    validate_nonnegative_direct_m("constructor.layer.field_capacity_m", layer.field_capacity_m)?;
    validate_nonnegative_direct_m("constructor.layer.upper_limit_m", layer.upper_limit_m)?;
    validate_nonnegative_direct_m("constructor.layer.conductivity_m_s", layer.conductivity_m_s)?;
    validate_positive_direct("constructor.layer.depth_m", layer.depth_m)?;
    validate_finite("constructor.layer.residual_theta", layer.residual_theta)?;
    validate_nonnegative_direct_m("constructor.layer.frozen_depth_m", layer.frozen_depth_m)?;
    validate_nonnegative_direct_m("constructor.layer.frozen_water_m", layer.frozen_water_m)?;
    validate_unit_interval("constructor.layer.porosity", layer.porosity)?;
    validate_unit_interval(
        "constructor.layer.field_capacity_theta",
        layer.field_capacity_theta,
    )?;
    validate_finite("constructor.layer.coca", layer.coca)?;
    validate_nonnegative_direct_m(
        "constructor.layer.lateral_conductivity_m_s",
        layer.lateral_conductivity_m_s,
    )?;
    if layer.field_capacity_m > layer.upper_limit_m || layer.frozen_depth_m > layer.depth_m {
        return Err(DirectRuntimeError::DirectDomainViolation {
            field: "constructor.layer",
        });
    }
    Ok(())
}

fn validate_direct_evapotranspiration_stage(
    stage: DirectEvapotranspirationStageState,
) -> Result<(), DirectRuntimeError> {
    validate_nonnegative_direct_m("constructor.et_stage.s1_m", stage.s1_m)?;
    validate_nonnegative_direct_m("constructor.et_stage.s2_m", stage.s2_m)?;
    validate_nonnegative_direct_m("constructor.et_stage.threshold_m", stage.threshold_m)?;
    validate_nonnegative_direct_m("constructor.et_stage.counter", stage.counter)
}

fn validate_optional_nonnegative_direct_m(
    field: &'static str,
    value: Option<f64>,
) -> Result<(), DirectRuntimeError> {
    if let Some(value) = value {
        validate_nonnegative_direct_m(field, value)?;
    }
    Ok(())
}

fn validate_optional_finite_direct_m(
    field: &'static str,
    value: Option<f64>,
) -> Result<(), DirectRuntimeError> {
    if let Some(value) = value {
        validate_finite(field, value)?;
    }
    Ok(())
}

fn validate_positive_direct(field: &'static str, value: f64) -> Result<(), DirectRuntimeError> {
    validate_finite(field, value)?;
    if value > 0.0 {
        Ok(())
    } else {
        Err(DirectRuntimeError::DirectDomainViolation { field })
    }
}

fn validate_unit_interval(field: &'static str, value: f64) -> Result<(), DirectRuntimeError> {
    validate_finite(field, value)?;
    if (0.0..=1.0).contains(&value) {
        Ok(())
    } else if value < 0.0 {
        Err(DirectRuntimeError::NegativeDirectValue { field })
    } else {
        Err(DirectRuntimeError::DirectDomainViolation { field })
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
