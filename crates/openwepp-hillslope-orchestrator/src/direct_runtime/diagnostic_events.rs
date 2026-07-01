use super::{
    DirectDayFrame, DirectPercolationInputs, DirectPercolationState, DirectSubsurfaceComputeInputs,
    DirectSubsurfaceLayerState,
};

#[derive(Debug, Clone, PartialEq)]
pub struct DirectRunoffRebalanceTraceEvent {
    pub day_index: usize,
    pub lane_index: usize,
    pub target_m: f64,
    pub aggregate_m: f64,
    pub delta_m: f64,
    pub tolerance_m: f64,
    pub accepted: bool,
    pub storage_initial_m: f64,
    pub precip_input_m: f64,
    pub snow_coupling_m: f64,
    pub runon_input_m: f64,
    pub frost_liquid_delta_m: f64,
    pub interception_m: f64,
    pub q_runoff_m: f64,
    pub evapotranspiration_m: f64,
    pub evapotranspiration_storage_return_m: f64,
    pub deep_seepage_m: f64,
    pub subsurface_loss_m: f64,
    pub liquid_input_m: f64,
    pub cumulative_infiltration_m: f64,
    pub depression_storage_delta_m: f64,
    pub frost_retained_local_liquid_m: f64,
}

impl DirectRunoffRebalanceTraceEvent {
    #[must_use]
    pub fn from_day_frame(
        day_frame: &DirectDayFrame,
        target_m: f64,
        aggregate_m: f64,
        delta_m: f64,
        tolerance_m: f64,
        accepted: bool,
    ) -> Self {
        let inputs = day_frame.storage_reconciliation_inputs;
        let runoff_inputs = day_frame.runoff_partition_inputs;
        Self {
            day_index: day_frame.day_index,
            lane_index: day_frame.lane_index,
            target_m,
            aggregate_m,
            delta_m,
            tolerance_m,
            accepted,
            storage_initial_m: inputs.storage_initial_m,
            precip_input_m: inputs.precip_input_m,
            snow_coupling_m: inputs.snow_coupling_m,
            runon_input_m: inputs.runon_input_m,
            frost_liquid_delta_m: inputs.frost_liquid_delta_m,
            interception_m: inputs.interception_m,
            q_runoff_m: day_frame.runoff_partition.q_runoff_m,
            evapotranspiration_m: inputs.evapotranspiration_m,
            evapotranspiration_storage_return_m: inputs.evapotranspiration_storage_return_m,
            deep_seepage_m: inputs.deep_seepage_m,
            subsurface_loss_m: inputs.subsurface_loss_m,
            liquid_input_m: runoff_inputs.liquid_input_m,
            cumulative_infiltration_m: runoff_inputs.cumulative_infiltration_m,
            depression_storage_delta_m: runoff_inputs.depression_storage_delta_m,
            frost_retained_local_liquid_m: runoff_inputs.frost_retained_local_liquid_m,
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct DirectEvapotranspirationTraceEvent {
    pub day_index: usize,
    pub lane_index: usize,
    pub et_demand_m: f64,
    pub root_depth_m: f64,
    pub leaf_area_index: f64,
    pub canopy_cover_fraction: f64,
    pub residue_interception_m: f64,
    pub plant_tolerance: f64,
    pub growth_input_vdmt_before_kg_m2: f64,
    pub growth_input_tlive_before_kg_m2: f64,
    pub growth_input_hia_before: f64,
    pub growth_output_vdmt_after_kg_m2: f64,
    pub growth_output_tlive_after_kg_m2: f64,
    pub growth_output_hia_after: f64,
    pub pmet_soil_evaporation_m: Option<f64>,
    pub pmet_plant_transpiration_m: Option<f64>,
    pub pmet_soil_evaporation_storage_return_m: Option<f64>,
    pub surface_soil_water_before_m: f64,
    pub surface_soil_water_after_m: f64,
    pub root_soil_water_before_m: f64,
    pub root_soil_water_after_m: f64,
    pub soil_evaporation_m: f64,
    pub residue_evaporation_m: f64,
    pub plant_transpiration_m: f64,
    pub water_stress: f64,
    pub uptake_potential_m: f64,
    pub uptake_actual_m: f64,
    pub surface_layer_theta_m: Vec<f64>,
    pub root_layer_theta_m: Vec<f64>,
    pub root_layer_upper_limit_m: Vec<f64>,
    pub root_layer_depth_m: Vec<f64>,
    pub root_layer_uptake_potential_m: Vec<f64>,
    pub root_layer_uptake_actual_m: Vec<f64>,
}

impl DirectEvapotranspirationTraceEvent {
    #[must_use]
    pub fn from_day_frame(day_frame: &DirectDayFrame) -> Self {
        let inputs = day_frame.evapotranspiration_compute_inputs.clone();
        let surface = &day_frame.evapotranspiration_surface;
        let root = &day_frame.evapotranspiration_compute;
        let pmet = inputs.pmet;
        Self {
            day_index: day_frame.day_index,
            lane_index: day_frame.lane_index,
            et_demand_m: inputs.et_demand_m,
            root_depth_m: inputs.root_depth_m,
            leaf_area_index: inputs.leaf_area_index,
            canopy_cover_fraction: inputs.canopy_cover_fraction,
            residue_interception_m: inputs.residue_interception_m,
            plant_tolerance: inputs.plant_tolerance,
            growth_input_vdmt_before_kg_m2: day_frame
                .annual_growth_inputs
                .state_before
                .live_biomass_kg_m2,
            growth_input_tlive_before_kg_m2: day_frame
                .annual_growth_inputs
                .state_before
                .interception_live_biomass_kg_m2,
            growth_input_hia_before: day_frame.annual_growth_inputs.state_before.harvest_index,
            growth_output_vdmt_after_kg_m2: day_frame.annual_growth.state_after.live_biomass_kg_m2,
            growth_output_tlive_after_kg_m2: day_frame
                .annual_growth
                .state_after
                .interception_live_biomass_kg_m2,
            growth_output_hia_after: day_frame.annual_growth.state_after.harvest_index,
            pmet_soil_evaporation_m: pmet.map(|pmet| pmet.soil_evaporation_m),
            pmet_plant_transpiration_m: pmet.map(|pmet| pmet.plant_transpiration_m),
            pmet_soil_evaporation_storage_return_m: pmet
                .map(|pmet| pmet.soil_evaporation_storage_return_m),
            surface_soil_water_before_m: surface.soil_water_before_m,
            surface_soil_water_after_m: surface.soil_water_after_soil_evap_m,
            root_soil_water_before_m: root.soil_water_before_root_uptake_m,
            root_soil_water_after_m: root.soil_water_after_m,
            soil_evaporation_m: root.soil_evaporation_m,
            residue_evaporation_m: root.residue_evaporation_m,
            plant_transpiration_m: root.plant_transpiration_m,
            water_stress: root.water_stress,
            uptake_potential_m: root.uptake_potential_m,
            uptake_actual_m: root.uptake_actual_m,
            surface_layer_theta_m: surface
                .layer_state_after_soil_evap
                .iter()
                .map(|layer| layer.theta_m)
                .collect(),
            root_layer_theta_m: root
                .layer_state_after_root_uptake
                .iter()
                .map(|layer| layer.theta_m)
                .collect(),
            root_layer_upper_limit_m: root
                .layer_state_after_root_uptake
                .iter()
                .map(|layer| layer.upper_limit_m)
                .collect(),
            root_layer_depth_m: root
                .layer_state_after_root_uptake
                .iter()
                .map(|layer| layer.depth_m)
                .collect(),
            root_layer_uptake_potential_m: root.layer_uptake_potential_m.clone(),
            root_layer_uptake_actual_m: root.layer_uptake_actual_m.clone(),
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct DirectPercolationTraceEvent {
    pub day_index: usize,
    pub lane_index: usize,
    pub lane_substeps: usize,
    pub same_pass_infiltration_m: f64,
    pub same_pass_infiltration_lineage: bool,
    pub tillage_depth_m: f64,
    pub soil_water_before_m: f64,
    pub computed_soil_water_before_m: f64,
    pub soil_water_after_m: f64,
    pub deep_seepage_m: f64,
    pub per_layer_flux_m: Vec<f64>,
    pub layer_state_after: Vec<DirectSubsurfaceLayerState>,
}

impl DirectPercolationTraceEvent {
    #[must_use]
    pub fn from_state(
        day_index: usize,
        lane_index: usize,
        state: &DirectPercolationState,
        inputs: &DirectPercolationInputs,
    ) -> Self {
        Self {
            day_index,
            lane_index,
            lane_substeps: inputs.lane_substeps,
            same_pass_infiltration_m: inputs.same_pass_infiltration_m,
            same_pass_infiltration_lineage: inputs.same_pass_infiltration_lineage,
            tillage_depth_m: inputs.tillage_depth_m,
            soil_water_before_m: state.soil_water_before_m,
            computed_soil_water_before_m: state.computed_soil_water_before_m,
            soil_water_after_m: state.soil_water_after_m,
            deep_seepage_m: state.deep_seepage_m,
            per_layer_flux_m: state.per_layer_flux_m.clone(),
            layer_state_after: state.layer_state_after.clone(),
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct DirectSubsurfaceSaturationTraceEvent {
    pub day_index: usize,
    pub lane_index: usize,
    pub substep_index: usize,
    pub lane_substeps: usize,
    pub mofe_hourly_carry_arrays_enabled: bool,
    pub solwpv_mode: i32,
    pub theta_before_m: f64,
    pub upper_limit_m: f64,
    pub frozen_water_m: f64,
    pub effective_upper_limit_m: f64,
    pub saturation_excess_m: f64,
    pub current_saturation_runoff_m: f64,
    pub theta_after_m: f64,
}

impl DirectSubsurfaceSaturationTraceEvent {
    #[must_use]
    #[allow(clippy::too_many_arguments)]
    pub fn from_substep(
        day_index: usize,
        lane_index: usize,
        substep_index: usize,
        inputs: &DirectSubsurfaceComputeInputs,
        theta_before_m: f64,
        upper_limit_m: f64,
        frozen_water_m: f64,
        effective_upper_limit_m: f64,
        saturation_excess_m: f64,
        current_saturation_runoff_m: f64,
        theta_after_m: f64,
    ) -> Self {
        Self {
            day_index,
            lane_index,
            substep_index,
            lane_substeps: inputs.lane_substeps,
            mofe_hourly_carry_arrays_enabled: inputs.mofe_hourly_carry_arrays_enabled,
            solwpv_mode: inputs.solwpv_mode,
            theta_before_m,
            upper_limit_m,
            frozen_water_m,
            effective_upper_limit_m,
            saturation_excess_m,
            current_saturation_runoff_m,
            theta_after_m,
        }
    }
}
