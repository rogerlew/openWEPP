use super::{
    DIRECT_AUDIT, DIRECT_R4PQZ_PHASE_SPAN_COUNT, DirectDayFrame, DirectRuntimeError,
    DirectSubsurfaceLayerState, validate_finite, validate_nonnegative_direct_m,
};

impl DirectDayFrame {
    pub fn run_r4pqz_hydrology_projection_span(
        &mut self,
    ) -> Result<DirectHydrologyProjectionSpanReport, DirectRuntimeError> {
        DIRECT_AUDIT.record_phase_span_run();
        let phase_count = DIRECT_R4PQZ_PHASE_SPAN_COUNT;
        let mut phase_entry_count = 0_u64;

        DIRECT_AUDIT.record_direct_phase_entry();
        phase_entry_count += 1;
        let projection = self.compute_r4pqz_hydrology_projection()?;
        DIRECT_AUDIT.record_direct_compute_operation();

        DIRECT_AUDIT.record_direct_phase_entry();
        phase_entry_count += 1;
        self.hydrology_projection = projection.clone();
        DIRECT_AUDIT.record_direct_state_mutation();
        self.hydrology_projection_downstream_operands =
            DirectHydrologyProjectionDownstreamOperands::from(projection.clone());
        DIRECT_AUDIT.record_downstream_operand_production();

        let hydrology_projection_shadow_projection = DirectHydrologyProjectionShadowProjection {
            lane_index: self.lane_index,
            day_index: self.day_index,
            aggregate_storage_from_layers_m: projection.aggregate_storage_from_layers_m,
            storage_reconciled_m: projection.storage_reconciled_m,
            aggregate_storage_delta_m: projection.aggregate_storage_delta_m,
            q_runoff_m: projection.q_runoff_m,
            q_ofe_m: projection.q_ofe_m,
            deep_percolation_m: projection.deep_percolation_m,
            lateral_flow_m: projection.lateral_flow_m,
            tile_drainage_m: projection.tile_drainage_m,
            subsurface_loss_m: projection.subsurface_loss_m,
            evapotranspiration_m: projection.evapotranspiration_m,
            soil_evaporation_m: projection.soil_evaporation_m,
            residue_evaporation_m: projection.residue_evaporation_m,
            plant_transpiration_m: projection.plant_transpiration_m,
            water_stress: projection.water_stress,
            snow_frost_coupling_m: projection.snow_frost_coupling_m,
            snow_water_m: projection.snow_water_m,
            frozen_soil_water_m: projection.frozen_soil_water_m,
            total_soil_m: projection.total_soil_m,
            soil_water_total_m: projection.soil_water_total_m,
            runon_input_m: projection.runon_input_m,
            subsurface_carry_m: projection.subsurface_carry_m,
            profile_depth_m: projection.profile_depth_m,
            profile_porosity_cap_m: projection.profile_porosity_cap_m,
            profile_field_capacity_m: projection.profile_field_capacity_m,
            profile_wilting_point_m: projection.profile_wilting_point_m,
            publication_runoff_m: projection.publication_runoff_m,
            publication_evapotranspiration_m: projection.publication_evapotranspiration_m,
            publication_deep_percolation_m: projection.publication_deep_percolation_m,
            publication_lateral_flow_m: projection.publication_lateral_flow_m,
            public_output_cutover: projection.public_output_cutover,
        };
        self.hydrology_projection_shadow_projection =
            Some(hydrology_projection_shadow_projection.clone());
        DIRECT_AUDIT.record_shadow_projection();

        Ok(DirectHydrologyProjectionSpanReport {
            phase_count,
            phase_entry_count,
            direct_compute_count: 1,
            state_mutation_count: 1,
            downstream_operand_count: 1,
            shadow_projection_count: 1,
            compatibility_edge_invocation_count: 0,
            hydrology_projection_shadow_projection,
        })
    }

    fn compute_r4pqz_hydrology_projection(
        &self,
    ) -> Result<DirectHydrologyProjectionState, DirectRuntimeError> {
        self.validate_r4pqz_hydrology_projection_domain()?;

        let runoff = self.runoff_shadow_projection.as_ref().ok_or(
            DirectRuntimeError::MissingDirectUpstream {
                upstream: "R4A runoff partition",
            },
        )?;
        let percolation = self.percolation_shadow_projection.as_ref().ok_or(
            DirectRuntimeError::MissingDirectUpstream {
                upstream: "R4M percolation producer",
            },
        )?;
        let subsurface = self.subsurface_compute_shadow_projection.as_ref().ok_or(
            DirectRuntimeError::MissingDirectUpstream {
                upstream: "R4O subsurface compute producer",
            },
        )?;
        let evapotranspiration = self
            .evapotranspiration_compute_shadow_projection
            .as_ref()
            .ok_or(DirectRuntimeError::MissingDirectUpstream {
                upstream: "R4N evapotranspiration/root-uptake producer",
            })?;
        let snow_coupling = self.snow_coupling_shadow_projection.as_ref().ok_or(
            DirectRuntimeError::MissingDirectUpstream {
                upstream: "R4G snow-coupling producer",
            },
        )?;
        let storage = self.storage_shadow_projection.as_ref().ok_or(
            DirectRuntimeError::MissingDirectUpstream {
                upstream: "R4B storage reconciliation",
            },
        )?;
        let runon = self.runon_carry_shadow_projection.as_ref().ok_or(
            DirectRuntimeError::MissingDirectUpstream {
                upstream: "R4J runon/carry producer",
            },
        )?;

        let (aggregate_storage_from_layers_m, frozen_layer_storage_m) =
            aggregate_storage_from_layers(&evapotranspiration.layer_state_after_root_uptake)?;
        let frozen_soil_water_m =
            frozen_layer_storage_m + self.hydrology_projection_inputs.frozen_soil_water_m;
        validate_nonnegative_direct_m(
            "hydrology_projection.frozen_soil_water_m",
            frozen_soil_water_m,
        )?;
        let snow_water_m = self.hydrology_projection_inputs.snow_water_m;
        let total_soil_m = aggregate_storage_from_layers_m + frozen_soil_water_m;
        validate_finite("hydrology_projection.total_soil_m", total_soil_m)?;
        let soil_water_total_m = total_soil_m;
        let aggregate_storage_delta_m =
            aggregate_storage_from_layers_m - storage.storage_reconciled_m;
        validate_finite(
            "hydrology_projection.aggregate_storage_delta_m",
            aggregate_storage_delta_m,
        )?;
        if aggregate_storage_delta_m.abs()
            > self
                .hydrology_projection_inputs
                .aggregate_storage_tolerance_m
        {
            return Err(DirectRuntimeError::DirectClosureToleranceExceeded {
                field: "hydrology_projection.aggregate_storage_delta_m",
            });
        }

        Ok(DirectHydrologyProjectionState {
            aggregate_storage_from_layers_m,
            storage_reconciled_m: storage.storage_reconciled_m,
            aggregate_storage_delta_m,
            q_runoff_m: runoff.q_runoff_m,
            q_ofe_m: runoff.q_runoff_m,
            deep_percolation_m: percolation.deep_seepage_m,
            lateral_flow_m: subsurface.lateral_flow_m,
            tile_drainage_m: subsurface.tile_drainage_m,
            subsurface_loss_m: subsurface.subsurface_loss_m,
            evapotranspiration_m: evapotranspiration.evapotranspiration_m,
            soil_evaporation_m: evapotranspiration.soil_evaporation_m,
            residue_evaporation_m: evapotranspiration.residue_evaporation_m,
            plant_transpiration_m: evapotranspiration.plant_transpiration_m,
            water_stress: evapotranspiration.water_stress,
            snow_frost_coupling_m: snow_coupling.snow_coupling_m,
            snow_water_m,
            frozen_soil_water_m,
            total_soil_m,
            soil_water_total_m,
            runon_input_m: runon.runon_input_m,
            subsurface_carry_m: runon.subsurface_carry_m,
            profile_depth_m: self.hydrology_projection_inputs.profile_depth_m,
            profile_porosity_cap_m: self.hydrology_projection_inputs.profile_porosity_cap_m,
            profile_field_capacity_m: self.hydrology_projection_inputs.profile_field_capacity_m,
            profile_wilting_point_m: self.hydrology_projection_inputs.profile_wilting_point_m,
            publication_runoff_m: self.publication.runoff_m,
            publication_evapotranspiration_m: self.publication.evapotranspiration_m,
            publication_deep_percolation_m: self.publication.drainage_m,
            publication_lateral_flow_m: self.publication.lateral_flow_m,
            public_output_cutover: false,
        })
    }

    fn validate_r4pqz_hydrology_projection_domain(&self) -> Result<(), DirectRuntimeError> {
        if self.storage_shadow_projection.is_none() {
            return Err(DirectRuntimeError::MissingDirectUpstream {
                upstream: "R4B storage reconciliation",
            });
        }
        if self.evapotranspiration_compute_shadow_projection.is_none() {
            return Err(DirectRuntimeError::MissingDirectUpstream {
                upstream: "R4N evapotranspiration/root-uptake producer",
            });
        }
        if self.runoff_shadow_projection.is_none() {
            return Err(DirectRuntimeError::MissingDirectUpstream {
                upstream: "R4A runoff partition",
            });
        }
        if self.percolation_shadow_projection.is_none() {
            return Err(DirectRuntimeError::MissingDirectUpstream {
                upstream: "R4M percolation producer",
            });
        }
        if self.subsurface_compute_shadow_projection.is_none() {
            return Err(DirectRuntimeError::MissingDirectUpstream {
                upstream: "R4O subsurface compute producer",
            });
        }
        if self.snow_coupling_shadow_projection.is_none() {
            return Err(DirectRuntimeError::MissingDirectUpstream {
                upstream: "R4G snow-coupling producer",
            });
        }
        if self.runon_carry_shadow_projection.is_none() {
            return Err(DirectRuntimeError::MissingDirectUpstream {
                upstream: "R4J runon/carry producer",
            });
        }

        validate_nonnegative_direct_m(
            "hydrology_projection.aggregate_storage_tolerance_m",
            self.hydrology_projection_inputs
                .aggregate_storage_tolerance_m,
        )?;
        validate_nonnegative_direct_m(
            "hydrology_projection.snow_water_m",
            self.hydrology_projection_inputs.snow_water_m,
        )?;
        validate_nonnegative_direct_m(
            "hydrology_projection.frozen_soil_water_m",
            self.hydrology_projection_inputs.frozen_soil_water_m,
        )?;
        validate_optional_nonnegative_m(
            "hydrology_projection.profile_depth_m",
            self.hydrology_projection_inputs.profile_depth_m,
        )?;
        validate_optional_nonnegative_m(
            "hydrology_projection.profile_porosity_cap_m",
            self.hydrology_projection_inputs.profile_porosity_cap_m,
        )?;
        validate_optional_nonnegative_m(
            "hydrology_projection.profile_field_capacity_m",
            self.hydrology_projection_inputs.profile_field_capacity_m,
        )?;
        validate_optional_nonnegative_m(
            "hydrology_projection.profile_wilting_point_m",
            self.hydrology_projection_inputs.profile_wilting_point_m,
        )?;
        validate_nonnegative_direct_m(
            "hydrology_projection.publication_runoff_m",
            self.publication.runoff_m,
        )?;
        validate_nonnegative_direct_m(
            "hydrology_projection.publication_evapotranspiration_m",
            self.publication.evapotranspiration_m,
        )?;
        validate_nonnegative_direct_m(
            "hydrology_projection.publication_deep_percolation_m",
            self.publication.drainage_m,
        )?;
        validate_nonnegative_direct_m(
            "hydrology_projection.publication_lateral_flow_m",
            self.publication.lateral_flow_m,
        )?;
        Ok(())
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct DirectHydrologyProjectionInputs {
    pub aggregate_storage_tolerance_m: f64,
    pub snow_water_m: f64,
    pub frozen_soil_water_m: f64,
    pub profile_depth_m: Option<f64>,
    pub profile_porosity_cap_m: Option<f64>,
    pub profile_field_capacity_m: Option<f64>,
    pub profile_wilting_point_m: Option<f64>,
}

impl DirectHydrologyProjectionInputs {
    #[must_use]
    pub const fn zero() -> Self {
        Self {
            aggregate_storage_tolerance_m: 0.0,
            snow_water_m: 0.0,
            frozen_soil_water_m: 0.0,
            profile_depth_m: None,
            profile_porosity_cap_m: None,
            profile_field_capacity_m: None,
            profile_wilting_point_m: None,
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct DirectHydrologyProjectionState {
    pub aggregate_storage_from_layers_m: f64,
    pub storage_reconciled_m: f64,
    pub aggregate_storage_delta_m: f64,
    pub q_runoff_m: f64,
    pub q_ofe_m: f64,
    pub deep_percolation_m: f64,
    pub lateral_flow_m: f64,
    pub tile_drainage_m: f64,
    pub subsurface_loss_m: f64,
    pub evapotranspiration_m: f64,
    pub soil_evaporation_m: f64,
    pub residue_evaporation_m: f64,
    pub plant_transpiration_m: f64,
    pub water_stress: f64,
    pub snow_frost_coupling_m: f64,
    pub snow_water_m: f64,
    pub frozen_soil_water_m: f64,
    pub total_soil_m: f64,
    pub soil_water_total_m: f64,
    pub runon_input_m: f64,
    pub subsurface_carry_m: f64,
    pub profile_depth_m: Option<f64>,
    pub profile_porosity_cap_m: Option<f64>,
    pub profile_field_capacity_m: Option<f64>,
    pub profile_wilting_point_m: Option<f64>,
    pub publication_runoff_m: f64,
    pub publication_evapotranspiration_m: f64,
    pub publication_deep_percolation_m: f64,
    pub publication_lateral_flow_m: f64,
    pub public_output_cutover: bool,
}

impl DirectHydrologyProjectionState {
    #[must_use]
    pub const fn zero() -> Self {
        Self {
            aggregate_storage_from_layers_m: 0.0,
            storage_reconciled_m: 0.0,
            aggregate_storage_delta_m: 0.0,
            q_runoff_m: 0.0,
            q_ofe_m: 0.0,
            deep_percolation_m: 0.0,
            lateral_flow_m: 0.0,
            tile_drainage_m: 0.0,
            subsurface_loss_m: 0.0,
            evapotranspiration_m: 0.0,
            soil_evaporation_m: 0.0,
            residue_evaporation_m: 0.0,
            plant_transpiration_m: 0.0,
            water_stress: 0.0,
            snow_frost_coupling_m: 0.0,
            snow_water_m: 0.0,
            frozen_soil_water_m: 0.0,
            total_soil_m: 0.0,
            soil_water_total_m: 0.0,
            runon_input_m: 0.0,
            subsurface_carry_m: 0.0,
            profile_depth_m: None,
            profile_porosity_cap_m: None,
            profile_field_capacity_m: None,
            profile_wilting_point_m: None,
            publication_runoff_m: 0.0,
            publication_evapotranspiration_m: 0.0,
            publication_deep_percolation_m: 0.0,
            publication_lateral_flow_m: 0.0,
            public_output_cutover: false,
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct DirectHydrologyProjectionDownstreamOperands {
    pub aggregate_storage_from_layers_m: f64,
    pub storage_reconciled_m: f64,
    pub aggregate_storage_delta_m: f64,
    pub q_runoff_m: f64,
    pub q_ofe_m: f64,
    pub deep_percolation_m: f64,
    pub lateral_flow_m: f64,
    pub tile_drainage_m: f64,
    pub subsurface_loss_m: f64,
    pub evapotranspiration_m: f64,
    pub soil_evaporation_m: f64,
    pub residue_evaporation_m: f64,
    pub plant_transpiration_m: f64,
    pub water_stress: f64,
    pub snow_frost_coupling_m: f64,
    pub snow_water_m: f64,
    pub frozen_soil_water_m: f64,
    pub total_soil_m: f64,
    pub soil_water_total_m: f64,
    pub runon_input_m: f64,
    pub subsurface_carry_m: f64,
    pub profile_depth_m: Option<f64>,
    pub profile_porosity_cap_m: Option<f64>,
    pub profile_field_capacity_m: Option<f64>,
    pub profile_wilting_point_m: Option<f64>,
    pub public_output_cutover: bool,
}

impl DirectHydrologyProjectionDownstreamOperands {
    #[must_use]
    pub const fn zero() -> Self {
        Self {
            aggregate_storage_from_layers_m: 0.0,
            storage_reconciled_m: 0.0,
            aggregate_storage_delta_m: 0.0,
            q_runoff_m: 0.0,
            q_ofe_m: 0.0,
            deep_percolation_m: 0.0,
            lateral_flow_m: 0.0,
            tile_drainage_m: 0.0,
            subsurface_loss_m: 0.0,
            evapotranspiration_m: 0.0,
            soil_evaporation_m: 0.0,
            residue_evaporation_m: 0.0,
            plant_transpiration_m: 0.0,
            water_stress: 0.0,
            snow_frost_coupling_m: 0.0,
            snow_water_m: 0.0,
            frozen_soil_water_m: 0.0,
            total_soil_m: 0.0,
            soil_water_total_m: 0.0,
            runon_input_m: 0.0,
            subsurface_carry_m: 0.0,
            profile_depth_m: None,
            profile_porosity_cap_m: None,
            profile_field_capacity_m: None,
            profile_wilting_point_m: None,
            public_output_cutover: false,
        }
    }
}

impl From<DirectHydrologyProjectionState> for DirectHydrologyProjectionDownstreamOperands {
    fn from(state: DirectHydrologyProjectionState) -> Self {
        Self {
            aggregate_storage_from_layers_m: state.aggregate_storage_from_layers_m,
            storage_reconciled_m: state.storage_reconciled_m,
            aggregate_storage_delta_m: state.aggregate_storage_delta_m,
            q_runoff_m: state.q_runoff_m,
            q_ofe_m: state.q_ofe_m,
            deep_percolation_m: state.deep_percolation_m,
            lateral_flow_m: state.lateral_flow_m,
            tile_drainage_m: state.tile_drainage_m,
            subsurface_loss_m: state.subsurface_loss_m,
            evapotranspiration_m: state.evapotranspiration_m,
            soil_evaporation_m: state.soil_evaporation_m,
            residue_evaporation_m: state.residue_evaporation_m,
            plant_transpiration_m: state.plant_transpiration_m,
            water_stress: state.water_stress,
            snow_frost_coupling_m: state.snow_frost_coupling_m,
            snow_water_m: state.snow_water_m,
            frozen_soil_water_m: state.frozen_soil_water_m,
            total_soil_m: state.total_soil_m,
            soil_water_total_m: state.soil_water_total_m,
            runon_input_m: state.runon_input_m,
            subsurface_carry_m: state.subsurface_carry_m,
            profile_depth_m: state.profile_depth_m,
            profile_porosity_cap_m: state.profile_porosity_cap_m,
            profile_field_capacity_m: state.profile_field_capacity_m,
            profile_wilting_point_m: state.profile_wilting_point_m,
            public_output_cutover: state.public_output_cutover,
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct DirectHydrologyProjectionShadowProjection {
    pub lane_index: usize,
    pub day_index: usize,
    pub aggregate_storage_from_layers_m: f64,
    pub storage_reconciled_m: f64,
    pub aggregate_storage_delta_m: f64,
    pub q_runoff_m: f64,
    pub q_ofe_m: f64,
    pub deep_percolation_m: f64,
    pub lateral_flow_m: f64,
    pub tile_drainage_m: f64,
    pub subsurface_loss_m: f64,
    pub evapotranspiration_m: f64,
    pub soil_evaporation_m: f64,
    pub residue_evaporation_m: f64,
    pub plant_transpiration_m: f64,
    pub water_stress: f64,
    pub snow_frost_coupling_m: f64,
    pub snow_water_m: f64,
    pub frozen_soil_water_m: f64,
    pub total_soil_m: f64,
    pub soil_water_total_m: f64,
    pub runon_input_m: f64,
    pub subsurface_carry_m: f64,
    pub profile_depth_m: Option<f64>,
    pub profile_porosity_cap_m: Option<f64>,
    pub profile_field_capacity_m: Option<f64>,
    pub profile_wilting_point_m: Option<f64>,
    pub publication_runoff_m: f64,
    pub publication_evapotranspiration_m: f64,
    pub publication_deep_percolation_m: f64,
    pub publication_lateral_flow_m: f64,
    pub public_output_cutover: bool,
}

#[derive(Debug, Clone, PartialEq)]
pub struct DirectHydrologyProjectionSpanReport {
    pub phase_count: usize,
    pub phase_entry_count: u64,
    pub direct_compute_count: u64,
    pub state_mutation_count: u64,
    pub downstream_operand_count: u64,
    pub shadow_projection_count: u64,
    pub compatibility_edge_invocation_count: u64,
    pub hydrology_projection_shadow_projection: DirectHydrologyProjectionShadowProjection,
}

fn aggregate_storage_from_layers(
    layers: &[DirectSubsurfaceLayerState],
) -> Result<(f64, f64), DirectRuntimeError> {
    let mut liquid_storage_m = 0.0;
    let mut frozen_storage_m = 0.0;
    for layer in layers {
        validate_nonnegative_direct_m("hydrology_projection.layer.theta_m", layer.theta_m)?;
        validate_nonnegative_direct_m(
            "hydrology_projection.layer.frozen_water_m",
            layer.frozen_water_m,
        )?;
        validate_nonnegative_direct_m("hydrology_projection.layer.depth_m", layer.depth_m)?;
        validate_nonnegative_direct_m(
            "hydrology_projection.layer.upper_limit_m",
            layer.upper_limit_m,
        )?;
        liquid_storage_m += layer.theta_m;
        validate_finite(
            "hydrology_projection.aggregate_storage_from_layers_m",
            liquid_storage_m,
        )?;
        frozen_storage_m += layer.frozen_water_m;
        validate_finite(
            "hydrology_projection.frozen_layer_storage_m",
            frozen_storage_m,
        )?;
    }
    Ok((liquid_storage_m, frozen_storage_m))
}

fn validate_optional_nonnegative_m(
    field: &'static str,
    value: Option<f64>,
) -> Result<(), DirectRuntimeError> {
    if let Some(value) = value {
        validate_nonnegative_direct_m(field, value)?;
    }
    Ok(())
}
