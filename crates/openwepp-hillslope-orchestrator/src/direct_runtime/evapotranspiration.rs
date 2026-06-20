use crate::constants::{
    WB11_ZERO_THRESHOLD, WB15_CANCOV_MAX, WB17_CANOPY_BARE_SOIL_OFFSET,
    WB17_CANOPY_EAJ_COEFFICIENT, WB17_PLTOL_MAX, WB17_PLTOL_MIN, WB17_SOIL_EVAPORATION_DEPTH_M,
    WB17_STAGE_ONE_DEFICIT_SCALE, WB17_STAGE_TWO_DEFICIT_SCALE, WB17_STAGE_TWO_DENOMINATOR,
    WB17_SWU_UB, WB17_SWU_UOB, WB17_TRANSPIRATION_LAI_FULL_COVER,
};

use super::{
    DIRECT_AUDIT, DIRECT_R4N_ROOT_PHASE_SPAN_COUNT, DIRECT_R4N_SURFACE_PHASE_SPAN_COUNT,
    DirectDayFrame, DirectRuntimeError, DirectSubsurfaceLayerState, validate_finite,
    validate_nonnegative_direct_m,
};

impl DirectDayFrame {
    pub fn run_r4n_surface_et_span(
        &mut self,
    ) -> Result<DirectEvapotranspirationSurfaceSpanReport, DirectRuntimeError> {
        DIRECT_AUDIT.record_phase_span_run();
        let phase_count = DIRECT_R4N_SURFACE_PHASE_SPAN_COUNT;

        DIRECT_AUDIT.record_direct_phase_entry();
        let surface = self.compute_r4n_surface_et()?;
        DIRECT_AUDIT.record_direct_compute_operation();

        self.water.soil_water_m = surface.soil_water_after_soil_evap_m;
        self.evapotranspiration_surface = surface.clone();
        DIRECT_AUDIT.record_direct_state_mutation();

        self.evapotranspiration_surface_downstream_operands =
            DirectEvapotranspirationSurfaceDownstreamOperands::from(surface.clone());
        DIRECT_AUDIT.record_downstream_operand_production();

        let evapotranspiration_surface_shadow_projection =
            DirectEvapotranspirationSurfaceShadowProjection {
                lane_index: self.lane_index,
                day_index: self.day_index,
                soil_water_before_m: self
                    .evapotranspiration_surface_downstream_operands
                    .soil_water_before_m,
                soil_water_after_soil_evap_m: self
                    .evapotranspiration_surface_downstream_operands
                    .soil_water_after_soil_evap_m,
                evapotranspiration_seed_m: self
                    .evapotranspiration_surface_downstream_operands
                    .evapotranspiration_seed_m,
                transpiration_demand_m: self
                    .evapotranspiration_surface_downstream_operands
                    .transpiration_demand_m,
                soil_evaporation_m: self
                    .evapotranspiration_surface_downstream_operands
                    .soil_evaporation_m,
                residue_evaporation_m: self
                    .evapotranspiration_surface_downstream_operands
                    .residue_evaporation_m,
                residue_interception_after_m: self
                    .evapotranspiration_surface_downstream_operands
                    .residue_interception_after_m,
                layer_soil_evaporation_withdrawal_m: self
                    .evapotranspiration_surface_downstream_operands
                    .layer_soil_evaporation_withdrawal_m
                    .clone(),
                layer_state_after_soil_evap: self
                    .evapotranspiration_surface_downstream_operands
                    .layer_state_after_soil_evap
                    .clone(),
            };
        self.evapotranspiration_surface_shadow_projection =
            Some(evapotranspiration_surface_shadow_projection.clone());
        DIRECT_AUDIT.record_shadow_projection();

        Ok(DirectEvapotranspirationSurfaceSpanReport {
            phase_count,
            phase_entry_count: phase_count as u64,
            direct_compute_count: 1,
            state_mutation_count: 1,
            downstream_operand_count: 1,
            shadow_projection_count: 1,
            compatibility_edge_invocation_count: 0,
            evapotranspiration_surface_shadow_projection,
        })
    }

    pub fn run_r4n_root_uptake_span(
        &mut self,
    ) -> Result<DirectEvapotranspirationComputeSpanReport, DirectRuntimeError> {
        DIRECT_AUDIT.record_phase_span_run();
        let phase_count = DIRECT_R4N_ROOT_PHASE_SPAN_COUNT;
        let mut phase_entry_count = 0_u64;

        DIRECT_AUDIT.record_direct_phase_entry();
        phase_entry_count += 1;
        let evapotranspiration_compute = self.compute_r4n_root_uptake()?;
        DIRECT_AUDIT.record_direct_compute_operation();

        DIRECT_AUDIT.record_direct_phase_entry();
        phase_entry_count += 1;
        self.water.soil_water_m = evapotranspiration_compute.soil_water_after_m;
        self.water.evapotranspiration_m = evapotranspiration_compute.evapotranspiration_m;
        self.evapotranspiration_compute = evapotranspiration_compute.clone();
        self.storage_reconciliation_inputs.evapotranspiration_m =
            evapotranspiration_compute.evapotranspiration_m;
        DIRECT_AUDIT.record_direct_state_mutation();

        self.evapotranspiration_compute_downstream_operands =
            DirectEvapotranspirationComputeDownstreamOperands::from(
                evapotranspiration_compute.clone(),
            );
        DIRECT_AUDIT.record_downstream_operand_production();

        let evapotranspiration_compute_shadow_projection =
            DirectEvapotranspirationComputeShadowProjection {
                lane_index: self.lane_index,
                day_index: self.day_index,
                soil_water_before_root_uptake_m: self
                    .evapotranspiration_compute_downstream_operands
                    .soil_water_before_root_uptake_m,
                soil_water_after_m: self
                    .evapotranspiration_compute_downstream_operands
                    .soil_water_after_m,
                evapotranspiration_m: self
                    .evapotranspiration_compute_downstream_operands
                    .evapotranspiration_m,
                soil_evaporation_m: self
                    .evapotranspiration_compute_downstream_operands
                    .soil_evaporation_m,
                residue_evaporation_m: self
                    .evapotranspiration_compute_downstream_operands
                    .residue_evaporation_m,
                plant_transpiration_m: self
                    .evapotranspiration_compute_downstream_operands
                    .plant_transpiration_m,
                transpiration_demand_m: self
                    .evapotranspiration_compute_downstream_operands
                    .transpiration_demand_m,
                water_stress: self
                    .evapotranspiration_compute_downstream_operands
                    .water_stress,
                uptake_potential_m: self
                    .evapotranspiration_compute_downstream_operands
                    .uptake_potential_m,
                uptake_actual_m: self
                    .evapotranspiration_compute_downstream_operands
                    .uptake_actual_m,
                effective_plant_tolerance: self
                    .evapotranspiration_compute_downstream_operands
                    .effective_plant_tolerance,
                layer_uptake_potential_m: self
                    .evapotranspiration_compute_downstream_operands
                    .layer_uptake_potential_m
                    .clone(),
                layer_uptake_actual_m: self
                    .evapotranspiration_compute_downstream_operands
                    .layer_uptake_actual_m
                    .clone(),
                layer_state_after_root_uptake: self
                    .evapotranspiration_compute_downstream_operands
                    .layer_state_after_root_uptake
                    .clone(),
            };
        self.evapotranspiration_compute_shadow_projection =
            Some(evapotranspiration_compute_shadow_projection.clone());
        DIRECT_AUDIT.record_shadow_projection();

        Ok(DirectEvapotranspirationComputeSpanReport {
            phase_count,
            phase_entry_count,
            direct_compute_count: 1,
            state_mutation_count: 1,
            downstream_operand_count: 1,
            shadow_projection_count: 1,
            compatibility_edge_invocation_count: 0,
            evapotranspiration_compute_shadow_projection,
        })
    }

    #[allow(clippy::too_many_lines)]
    fn compute_r4n_surface_et(
        &self,
    ) -> Result<DirectEvapotranspirationSurfaceState, DirectRuntimeError> {
        let percolation = self.percolation_shadow_projection.as_ref().ok_or(
            DirectRuntimeError::MissingDirectUpstream {
                upstream: "R4M percolation producer",
            },
        )?;
        let inputs = &self.evapotranspiration_compute_inputs;
        validate_surface_inputs(inputs)?;

        let mut layers = percolation.layer_state_after.clone();
        validate_et_layers("evapotranspiration.layers", &layers)?;
        let soil_water_before_m = percolation.soil_water_after_m;
        let mut stage_state_after = inputs.stage_state;

        let (soil_evaporation_with_residue_m, transpiration_demand_m, pmet_component_mode) =
            if let Some(pmet) = inputs.pmet {
                validate_pmet_inputs(pmet)?;
                if let Some(top_layer) = layers.first_mut() {
                    top_layer.theta_m += pmet.soil_evaporation_storage_return_m;
                    validate_finite("evapotranspiration.top_layer_storage_m", top_layer.theta_m)?;
                }
                (
                    normalize_within_zero_tolerance(pmet.soil_evaporation_m)?,
                    pmet.plant_transpiration_m,
                    true,
                )
            } else {
                let soil_evaporation_partition_potential_m = inputs.et_demand_m
                    * (-WB17_CANOPY_EAJ_COEFFICIENT
                        * (inputs.canopy_cover_fraction + WB17_CANOPY_BARE_SOIL_OFFSET))
                        .exp();
                validate_nonnegative_direct_m(
                    "evapotranspiration.soil_evaporation_partition_potential_m",
                    soil_evaporation_partition_potential_m,
                )?;
                if soil_evaporation_partition_potential_m > inputs.et_demand_m + WB11_ZERO_THRESHOLD
                {
                    return Err(DirectRuntimeError::DirectDomainViolation {
                        field: "evapotranspiration.soil_evaporation_partition_potential_m",
                    });
                }

                let transpiration_partition_potential_m = if inputs.leaf_area_index
                    > WB17_TRANSPIRATION_LAI_FULL_COVER
                {
                    inputs.et_demand_m
                } else {
                    inputs.leaf_area_index * inputs.et_demand_m / WB17_TRANSPIRATION_LAI_FULL_COVER
                };
                validate_nonnegative_direct_m(
                    "evapotranspiration.transpiration_partition_potential_m",
                    transpiration_partition_potential_m,
                )?;

                let residue_evaporation_m = inputs
                    .residue_interception_m
                    .min(soil_evaporation_partition_potential_m);
                let soil_evaporation_potential_m =
                    soil_evaporation_partition_potential_m - residue_evaporation_m;
                let soil_evaporation_demand_m = if let Some(stage_state) = inputs.stage_state {
                    let (stage_soil_evaporation_m, next_stage) = compute_stage_soil_evaporation(
                        stage_state,
                        inputs.same_pass_infiltration_m,
                        soil_evaporation_potential_m,
                    )?;
                    stage_state_after = Some(next_stage);
                    stage_soil_evaporation_m
                } else {
                    soil_evaporation_potential_m
                };

                let mut soil_evaporation_with_residue_m =
                    soil_evaporation_demand_m + inputs.residue_interception_m;
                let potential_et_before_layer_m =
                    soil_evaporation_with_residue_m + transpiration_partition_potential_m;
                if inputs.et_demand_m < potential_et_before_layer_m {
                    soil_evaporation_with_residue_m =
                        (inputs.et_demand_m - transpiration_partition_potential_m).max(0.0);
                }
                (
                    soil_evaporation_with_residue_m,
                    transpiration_partition_potential_m,
                    false,
                )
            };

        validate_nonnegative_direct_m(
            "evapotranspiration.soil_evaporation_with_residue_m",
            soil_evaporation_with_residue_m,
        )?;
        validate_nonnegative_direct_m(
            "evapotranspiration.transpiration_demand_m",
            transpiration_demand_m,
        )?;

        let mut residue_evaporation_m = inputs.residue_interception_m;
        let soil_evaporation_extraction_demand_m =
            if soil_evaporation_with_residue_m < inputs.residue_interception_m {
                residue_evaporation_m = if pmet_component_mode {
                    soil_evaporation_with_residue_m
                } else {
                    soil_evaporation_with_residue_m.max(0.0)
                };
                if let Some(top_layer) = layers.first_mut() {
                    top_layer.theta_m += inputs.residue_interception_m - residue_evaporation_m;
                    validate_finite("evapotranspiration.top_layer_storage_m", top_layer.theta_m)?;
                }
                0.0
            } else {
                soil_evaporation_with_residue_m - inputs.residue_interception_m
            };

        let (soil_evaporation_m, layer_soil_evaporation_withdrawal_m) =
            withdraw_soil_evaporation(&mut layers, soil_evaporation_extraction_demand_m)?;
        apply_post_et_upper_limit_redistribution(
            &mut layers,
            inputs.outside_water_depth_m > 1.0e-6,
        )?;
        validate_et_layers("evapotranspiration.layers_after_soil_evap", &layers)?;
        let soil_water_after_soil_evap_m = aggregate_soil_water(&layers)?;
        let evapotranspiration_seed_m = residue_evaporation_m + soil_evaporation_m;
        validate_nonnegative_direct_m(
            "evapotranspiration.evapotranspiration_seed_m",
            evapotranspiration_seed_m,
        )?;

        Ok(DirectEvapotranspirationSurfaceState {
            soil_water_before_m,
            soil_water_after_soil_evap_m,
            evapotranspiration_seed_m,
            transpiration_demand_m,
            soil_evaporation_m,
            residue_evaporation_m,
            residue_interception_after_m: 0.0,
            stage_state_after,
            layer_soil_evaporation_withdrawal_m,
            layer_state_after_soil_evap: layers,
        })
    }

    fn compute_r4n_root_uptake(
        &self,
    ) -> Result<DirectEvapotranspirationComputeState, DirectRuntimeError> {
        let surface = self
            .evapotranspiration_surface_shadow_projection
            .as_ref()
            .ok_or(DirectRuntimeError::MissingDirectUpstream {
                upstream: "R4N surface evapotranspiration producer",
            })?;
        let subsurface = self.subsurface_compute_shadow_projection.as_ref().ok_or(
            DirectRuntimeError::MissingDirectUpstream {
                upstream: "R4O subsurface compute producer",
            },
        )?;
        let inputs = &self.evapotranspiration_compute_inputs;
        validate_root_inputs(inputs)?;

        let mut layers = subsurface.layer_state_after.clone();
        validate_et_layers("root_uptake.layers", &layers)?;
        if layers.len() != surface.layer_state_after_soil_evap.len() {
            return Err(DirectRuntimeError::DirectDomainViolation {
                field: "root_uptake.layer_count",
            });
        }

        let soil_water_before_root_uptake_m = subsurface.soil_water_after_m;
        let computed_soil_water_before_m = aggregate_soil_water(&layers)?;
        let effective_plant_tolerance = effective_swu_plant_tolerance(inputs.plant_tolerance);
        validate_between(
            "root_uptake.effective_plant_tolerance",
            effective_plant_tolerance,
            WB17_PLTOL_MIN,
            WB17_PLTOL_MAX,
        )?;

        let mut layer_uptake_potential_m = vec![0.0_f64; layers.len()];
        let mut layer_uptake_actual_m = vec![0.0_f64; layers.len()];
        let mut plant_transpiration_m = 0.0;
        let profile_depth_m = layers.iter().map(|layer| layer.depth_m).sum::<f64>();
        validate_nonnegative_direct_m("root_uptake.profile_depth_m", profile_depth_m)?;
        let effective_root_depth_m = inputs.root_depth_m.min(profile_depth_m);
        if surface.transpiration_demand_m > WB11_ZERO_THRESHOLD
            && effective_root_depth_m > WB11_ZERO_THRESHOLD
        {
            run_swu_root_uptake(
                &mut layers,
                surface.transpiration_demand_m,
                effective_root_depth_m,
                effective_plant_tolerance,
                &mut layer_uptake_potential_m,
                &mut layer_uptake_actual_m,
                &mut plant_transpiration_m,
            )?;
        }

        let uptake_potential_m = layer_uptake_potential_m.iter().sum::<f64>();
        validate_nonnegative_direct_m("root_uptake.uptake_potential_m", uptake_potential_m)?;
        let uptake_actual_m = layer_uptake_actual_m.iter().sum::<f64>();
        validate_nonnegative_direct_m("root_uptake.uptake_actual_m", uptake_actual_m)?;

        let mut soil_water_after_m = soil_water_before_root_uptake_m;
        if uptake_actual_m > WB11_ZERO_THRESHOLD {
            soil_water_after_m = aggregate_soil_water(&layers)?;
            let storage_uptake_m = computed_soil_water_before_m - soil_water_after_m;
            let storage_correction_m = storage_uptake_m - uptake_actual_m;
            if storage_correction_m.abs() > f64::EPSILON
                && let Some(index) = layer_uptake_actual_m.iter().rposition(|value| *value > 0.0)
            {
                layers[index].theta_m += storage_correction_m;
                validate_nonnegative_direct_m(
                    "root_uptake.corrected_layer_theta_m",
                    layers[index].theta_m,
                )?;
                soil_water_after_m = aggregate_soil_water(&layers)?;
            }
        }
        validate_nonnegative_direct_m("root_uptake.soil_water_after_m", soil_water_after_m)?;

        let evapotranspiration_m = surface.evapotranspiration_seed_m + uptake_actual_m;
        validate_nonnegative_direct_m("root_uptake.evapotranspiration_m", evapotranspiration_m)?;
        let water_stress = if surface.transpiration_demand_m <= WB11_ZERO_THRESHOLD
            || effective_root_depth_m <= WB11_ZERO_THRESHOLD
        {
            1.0
        } else {
            (uptake_actual_m / surface.transpiration_demand_m).min(1.0)
        };
        validate_between("root_uptake.water_stress", water_stress, 0.0, 1.0)?;

        Ok(DirectEvapotranspirationComputeState {
            soil_water_before_root_uptake_m,
            soil_water_after_m,
            evapotranspiration_m,
            soil_evaporation_m: surface.soil_evaporation_m,
            residue_evaporation_m: surface.residue_evaporation_m,
            plant_transpiration_m,
            transpiration_demand_m: surface.transpiration_demand_m,
            water_stress,
            uptake_potential_m,
            uptake_actual_m,
            effective_plant_tolerance,
            layer_uptake_potential_m,
            layer_uptake_actual_m,
            layer_state_after_root_uptake: layers,
        })
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct DirectEvapotranspirationStageState {
    pub s1_m: f64,
    pub s2_m: f64,
    pub threshold_m: f64,
    pub counter: f64,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct DirectEvapotranspirationPmetInputs {
    pub soil_evaporation_m: f64,
    pub plant_transpiration_m: f64,
    pub soil_evaporation_storage_return_m: f64,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct DirectEvapotranspirationComputeInputs {
    pub et_demand_m: f64,
    pub leaf_area_index: f64,
    pub canopy_cover_fraction: f64,
    pub residue_interception_m: f64,
    pub same_pass_infiltration_m: f64,
    pub outside_water_depth_m: f64,
    pub root_depth_m: f64,
    pub plant_tolerance: f64,
    pub stage_state: Option<DirectEvapotranspirationStageState>,
    pub pmet: Option<DirectEvapotranspirationPmetInputs>,
}

impl DirectEvapotranspirationComputeInputs {
    #[must_use]
    pub const fn zero() -> Self {
        Self {
            et_demand_m: 0.0,
            leaf_area_index: 0.0,
            canopy_cover_fraction: 0.0,
            residue_interception_m: 0.0,
            same_pass_infiltration_m: 0.0,
            outside_water_depth_m: 0.0,
            root_depth_m: 0.0,
            plant_tolerance: 0.0,
            stage_state: None,
            pmet: None,
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct DirectEvapotranspirationSurfaceState {
    pub soil_water_before_m: f64,
    pub soil_water_after_soil_evap_m: f64,
    pub evapotranspiration_seed_m: f64,
    pub transpiration_demand_m: f64,
    pub soil_evaporation_m: f64,
    pub residue_evaporation_m: f64,
    pub residue_interception_after_m: f64,
    pub stage_state_after: Option<DirectEvapotranspirationStageState>,
    pub layer_soil_evaporation_withdrawal_m: Vec<f64>,
    pub layer_state_after_soil_evap: Vec<DirectSubsurfaceLayerState>,
}

impl DirectEvapotranspirationSurfaceState {
    #[must_use]
    pub fn zero() -> Self {
        Self {
            soil_water_before_m: 0.0,
            soil_water_after_soil_evap_m: 0.0,
            evapotranspiration_seed_m: 0.0,
            transpiration_demand_m: 0.0,
            soil_evaporation_m: 0.0,
            residue_evaporation_m: 0.0,
            residue_interception_after_m: 0.0,
            stage_state_after: None,
            layer_soil_evaporation_withdrawal_m: Vec::new(),
            layer_state_after_soil_evap: Vec::new(),
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct DirectEvapotranspirationSurfaceDownstreamOperands {
    pub soil_water_before_m: f64,
    pub soil_water_after_soil_evap_m: f64,
    pub evapotranspiration_seed_m: f64,
    pub transpiration_demand_m: f64,
    pub soil_evaporation_m: f64,
    pub residue_evaporation_m: f64,
    pub residue_interception_after_m: f64,
    pub layer_soil_evaporation_withdrawal_m: Vec<f64>,
    pub layer_state_after_soil_evap: Vec<DirectSubsurfaceLayerState>,
}

impl DirectEvapotranspirationSurfaceDownstreamOperands {
    #[must_use]
    pub fn zero() -> Self {
        Self {
            soil_water_before_m: 0.0,
            soil_water_after_soil_evap_m: 0.0,
            evapotranspiration_seed_m: 0.0,
            transpiration_demand_m: 0.0,
            soil_evaporation_m: 0.0,
            residue_evaporation_m: 0.0,
            residue_interception_after_m: 0.0,
            layer_soil_evaporation_withdrawal_m: Vec::new(),
            layer_state_after_soil_evap: Vec::new(),
        }
    }
}

impl From<DirectEvapotranspirationSurfaceState>
    for DirectEvapotranspirationSurfaceDownstreamOperands
{
    fn from(state: DirectEvapotranspirationSurfaceState) -> Self {
        Self {
            soil_water_before_m: state.soil_water_before_m,
            soil_water_after_soil_evap_m: state.soil_water_after_soil_evap_m,
            evapotranspiration_seed_m: state.evapotranspiration_seed_m,
            transpiration_demand_m: state.transpiration_demand_m,
            soil_evaporation_m: state.soil_evaporation_m,
            residue_evaporation_m: state.residue_evaporation_m,
            residue_interception_after_m: state.residue_interception_after_m,
            layer_soil_evaporation_withdrawal_m: state.layer_soil_evaporation_withdrawal_m,
            layer_state_after_soil_evap: state.layer_state_after_soil_evap,
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct DirectEvapotranspirationSurfaceShadowProjection {
    pub lane_index: usize,
    pub day_index: usize,
    pub soil_water_before_m: f64,
    pub soil_water_after_soil_evap_m: f64,
    pub evapotranspiration_seed_m: f64,
    pub transpiration_demand_m: f64,
    pub soil_evaporation_m: f64,
    pub residue_evaporation_m: f64,
    pub residue_interception_after_m: f64,
    pub layer_soil_evaporation_withdrawal_m: Vec<f64>,
    pub layer_state_after_soil_evap: Vec<DirectSubsurfaceLayerState>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct DirectEvapotranspirationComputeState {
    pub soil_water_before_root_uptake_m: f64,
    pub soil_water_after_m: f64,
    pub evapotranspiration_m: f64,
    pub soil_evaporation_m: f64,
    pub residue_evaporation_m: f64,
    pub plant_transpiration_m: f64,
    pub transpiration_demand_m: f64,
    pub water_stress: f64,
    pub uptake_potential_m: f64,
    pub uptake_actual_m: f64,
    pub effective_plant_tolerance: f64,
    pub layer_uptake_potential_m: Vec<f64>,
    pub layer_uptake_actual_m: Vec<f64>,
    pub layer_state_after_root_uptake: Vec<DirectSubsurfaceLayerState>,
}

impl DirectEvapotranspirationComputeState {
    #[must_use]
    pub fn zero() -> Self {
        Self {
            soil_water_before_root_uptake_m: 0.0,
            soil_water_after_m: 0.0,
            evapotranspiration_m: 0.0,
            soil_evaporation_m: 0.0,
            residue_evaporation_m: 0.0,
            plant_transpiration_m: 0.0,
            transpiration_demand_m: 0.0,
            water_stress: 1.0,
            uptake_potential_m: 0.0,
            uptake_actual_m: 0.0,
            effective_plant_tolerance: 0.25,
            layer_uptake_potential_m: Vec::new(),
            layer_uptake_actual_m: Vec::new(),
            layer_state_after_root_uptake: Vec::new(),
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct DirectEvapotranspirationComputeDownstreamOperands {
    pub soil_water_before_root_uptake_m: f64,
    pub soil_water_after_m: f64,
    pub evapotranspiration_m: f64,
    pub soil_evaporation_m: f64,
    pub residue_evaporation_m: f64,
    pub plant_transpiration_m: f64,
    pub transpiration_demand_m: f64,
    pub water_stress: f64,
    pub uptake_potential_m: f64,
    pub uptake_actual_m: f64,
    pub effective_plant_tolerance: f64,
    pub layer_uptake_potential_m: Vec<f64>,
    pub layer_uptake_actual_m: Vec<f64>,
    pub layer_state_after_root_uptake: Vec<DirectSubsurfaceLayerState>,
}

impl DirectEvapotranspirationComputeDownstreamOperands {
    #[must_use]
    pub fn zero() -> Self {
        Self {
            soil_water_before_root_uptake_m: 0.0,
            soil_water_after_m: 0.0,
            evapotranspiration_m: 0.0,
            soil_evaporation_m: 0.0,
            residue_evaporation_m: 0.0,
            plant_transpiration_m: 0.0,
            transpiration_demand_m: 0.0,
            water_stress: 1.0,
            uptake_potential_m: 0.0,
            uptake_actual_m: 0.0,
            effective_plant_tolerance: 0.25,
            layer_uptake_potential_m: Vec::new(),
            layer_uptake_actual_m: Vec::new(),
            layer_state_after_root_uptake: Vec::new(),
        }
    }
}

impl From<DirectEvapotranspirationComputeState>
    for DirectEvapotranspirationComputeDownstreamOperands
{
    fn from(state: DirectEvapotranspirationComputeState) -> Self {
        Self {
            soil_water_before_root_uptake_m: state.soil_water_before_root_uptake_m,
            soil_water_after_m: state.soil_water_after_m,
            evapotranspiration_m: state.evapotranspiration_m,
            soil_evaporation_m: state.soil_evaporation_m,
            residue_evaporation_m: state.residue_evaporation_m,
            plant_transpiration_m: state.plant_transpiration_m,
            transpiration_demand_m: state.transpiration_demand_m,
            water_stress: state.water_stress,
            uptake_potential_m: state.uptake_potential_m,
            uptake_actual_m: state.uptake_actual_m,
            effective_plant_tolerance: state.effective_plant_tolerance,
            layer_uptake_potential_m: state.layer_uptake_potential_m,
            layer_uptake_actual_m: state.layer_uptake_actual_m,
            layer_state_after_root_uptake: state.layer_state_after_root_uptake,
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct DirectEvapotranspirationComputeShadowProjection {
    pub lane_index: usize,
    pub day_index: usize,
    pub soil_water_before_root_uptake_m: f64,
    pub soil_water_after_m: f64,
    pub evapotranspiration_m: f64,
    pub soil_evaporation_m: f64,
    pub residue_evaporation_m: f64,
    pub plant_transpiration_m: f64,
    pub transpiration_demand_m: f64,
    pub water_stress: f64,
    pub uptake_potential_m: f64,
    pub uptake_actual_m: f64,
    pub effective_plant_tolerance: f64,
    pub layer_uptake_potential_m: Vec<f64>,
    pub layer_uptake_actual_m: Vec<f64>,
    pub layer_state_after_root_uptake: Vec<DirectSubsurfaceLayerState>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct DirectEvapotranspirationSurfaceSpanReport {
    pub phase_count: usize,
    pub phase_entry_count: u64,
    pub direct_compute_count: u64,
    pub state_mutation_count: u64,
    pub downstream_operand_count: u64,
    pub shadow_projection_count: u64,
    pub compatibility_edge_invocation_count: u64,
    pub evapotranspiration_surface_shadow_projection:
        DirectEvapotranspirationSurfaceShadowProjection,
}

#[derive(Debug, Clone, PartialEq)]
pub struct DirectEvapotranspirationComputeSpanReport {
    pub phase_count: usize,
    pub phase_entry_count: u64,
    pub direct_compute_count: u64,
    pub state_mutation_count: u64,
    pub downstream_operand_count: u64,
    pub shadow_projection_count: u64,
    pub compatibility_edge_invocation_count: u64,
    pub evapotranspiration_compute_shadow_projection:
        DirectEvapotranspirationComputeShadowProjection,
}

fn validate_surface_inputs(
    inputs: &DirectEvapotranspirationComputeInputs,
) -> Result<(), DirectRuntimeError> {
    validate_nonnegative_direct_m("evapotranspiration.et_demand_m", inputs.et_demand_m)?;
    validate_nonnegative_direct_m("evapotranspiration.leaf_area_index", inputs.leaf_area_index)?;
    validate_between(
        "evapotranspiration.canopy_cover_fraction",
        inputs.canopy_cover_fraction,
        0.0,
        WB15_CANCOV_MAX,
    )?;
    validate_nonnegative_direct_m(
        "evapotranspiration.residue_interception_m",
        inputs.residue_interception_m,
    )?;
    validate_nonnegative_direct_m(
        "evapotranspiration.same_pass_infiltration_m",
        inputs.same_pass_infiltration_m,
    )?;
    validate_nonnegative_direct_m(
        "evapotranspiration.outside_water_depth_m",
        inputs.outside_water_depth_m,
    )?;
    if let Some(stage) = inputs.stage_state {
        validate_stage_state(stage)?;
    }
    Ok(())
}

fn validate_root_inputs(
    inputs: &DirectEvapotranspirationComputeInputs,
) -> Result<(), DirectRuntimeError> {
    validate_nonnegative_direct_m("root_uptake.root_depth_m", inputs.root_depth_m)?;
    validate_finite("root_uptake.plant_tolerance", inputs.plant_tolerance)?;
    Ok(())
}

fn validate_stage_state(
    stage: DirectEvapotranspirationStageState,
) -> Result<(), DirectRuntimeError> {
    validate_nonnegative_direct_m("evapotranspiration.stage_s1_m", stage.s1_m)?;
    validate_nonnegative_direct_m("evapotranspiration.stage_s2_m", stage.s2_m)?;
    validate_positive("evapotranspiration.stage_threshold_m", stage.threshold_m)?;
    validate_nonnegative_direct_m("evapotranspiration.stage_counter", stage.counter)?;
    Ok(())
}

fn validate_pmet_inputs(
    pmet: DirectEvapotranspirationPmetInputs,
) -> Result<(), DirectRuntimeError> {
    validate_finite(
        "evapotranspiration.pmet_soil_evaporation_m",
        pmet.soil_evaporation_m,
    )?;
    if pmet.soil_evaporation_m < -WB11_ZERO_THRESHOLD {
        return Err(DirectRuntimeError::DirectDomainViolation {
            field: "evapotranspiration.pmet_soil_evaporation_m",
        });
    }
    validate_nonnegative_direct_m(
        "evapotranspiration.pmet_plant_transpiration_m",
        pmet.plant_transpiration_m,
    )?;
    validate_nonnegative_direct_m(
        "evapotranspiration.pmet_soil_evaporation_storage_return_m",
        pmet.soil_evaporation_storage_return_m,
    )?;
    Ok(())
}

fn validate_et_layers(
    field: &'static str,
    layers: &[DirectSubsurfaceLayerState],
) -> Result<(), DirectRuntimeError> {
    if layers.is_empty() {
        return Err(DirectRuntimeError::DirectDomainViolation { field });
    }
    for layer in layers {
        validate_nonnegative_direct_m("evapotranspiration.layer_theta_m", layer.theta_m)?;
        validate_positive(
            "evapotranspiration.layer_upper_limit_m",
            layer.upper_limit_m,
        )?;
        validate_positive("evapotranspiration.layer_depth_m", layer.depth_m)?;
        validate_nonnegative_direct_m(
            "evapotranspiration.layer_residual_theta",
            layer.residual_theta,
        )?;
        validate_nonnegative_direct_m(
            "evapotranspiration.layer_frozen_depth_m",
            layer.frozen_depth_m,
        )?;
        validate_nonnegative_direct_m(
            "evapotranspiration.layer_frozen_water_m",
            layer.frozen_water_m,
        )?;
        if layer.frozen_depth_m > layer.depth_m + WB11_ZERO_THRESHOLD {
            return Err(DirectRuntimeError::DirectDomainViolation {
                field: "evapotranspiration.layer_frozen_depth_m",
            });
        }
    }
    Ok(())
}

fn compute_stage_soil_evaporation(
    stage: DirectEvapotranspirationStageState,
    infiltration_m: f64,
    soil_evaporation_potential_m: f64,
) -> Result<(f64, DirectEvapotranspirationStageState), DirectRuntimeError> {
    let mut s1 = stage.s1_m;
    let mut s2 = stage.s2_m;
    let tu = stage.threshold_m;
    let mut tv = stage.counter;
    let mut es_stage_m = soil_evaporation_potential_m;

    if s1 < tu {
        s2 = 0.0;
        let sp = s1 - infiltration_m;
        s1 = if sp > 0.0 { sp } else { 0.0 };
        s1 += soil_evaporation_potential_m;
        let su = s1 - tu;
        if su > 0.0 {
            es_stage_m = soil_evaporation_potential_m - WB17_STAGE_ONE_DEFICIT_SCALE * su;
            s2 = WB17_STAGE_TWO_DEFICIT_SCALE * su;
            tv = (s2 / WB17_STAGE_TWO_DENOMINATOR).powi(2);
        }
    } else {
        let sb = infiltration_m - s2;
        if sb < 0.0 {
            tv += 1.0;
            es_stage_m = WB17_STAGE_TWO_DENOMINATOR * tv.sqrt() - s2;
            if infiltration_m > 0.0 {
                let mut esx_m = 0.8 * infiltration_m;
                if es_stage_m > esx_m {
                    esx_m = es_stage_m + infiltration_m;
                }
                if esx_m > soil_evaporation_potential_m {
                    esx_m = soil_evaporation_potential_m;
                }
                es_stage_m = esx_m;
            } else if es_stage_m > soil_evaporation_potential_m {
                es_stage_m = soil_evaporation_potential_m;
            }
            s2 += es_stage_m - infiltration_m;
            tv = (s2 / WB17_STAGE_TWO_DENOMINATOR).powi(2);
        } else {
            s1 = tu - sb;
            tv = 0.0;
            s2 = 0.0;
            if s1 < 0.0 {
                s1 = 0.0;
            }
            s1 += soil_evaporation_potential_m;
            let su = s1 - tu;
            if su > 0.0 {
                es_stage_m = soil_evaporation_potential_m - WB17_STAGE_ONE_DEFICIT_SCALE * su;
                s2 = WB17_STAGE_TWO_DEFICIT_SCALE * su;
                tv = (s2 / WB17_STAGE_TWO_DENOMINATOR).powi(2);
            }
        }
    }

    validate_nonnegative_direct_m("evapotranspiration.stage_es_m", es_stage_m)?;
    if es_stage_m > soil_evaporation_potential_m + WB11_ZERO_THRESHOLD {
        return Err(DirectRuntimeError::DirectDomainViolation {
            field: "evapotranspiration.stage_es_m",
        });
    }
    let next_stage = DirectEvapotranspirationStageState {
        s1_m: s1,
        s2_m: s2,
        threshold_m: tu,
        counter: tv,
    };
    validate_stage_state(next_stage)?;
    Ok((es_stage_m, next_stage))
}

fn withdraw_soil_evaporation(
    layers: &mut [DirectSubsurfaceLayerState],
    demand_m: f64,
) -> Result<(f64, Vec<f64>), DirectRuntimeError> {
    validate_nonnegative_direct_m("evapotranspiration.soil_evaporation_demand_m", demand_m)?;
    let mut layer_withdrawal_m = vec![0.0_f64; layers.len()];
    let mut remaining_m = demand_m;
    let mut cumulative_depth_m = 0.0;
    for (index, layer) in layers.iter_mut().enumerate() {
        if remaining_m <= WB11_ZERO_THRESHOLD {
            break;
        }
        let previous_depth_m = cumulative_depth_m;
        cumulative_depth_m += layer.depth_m;
        if previous_depth_m >= WB17_SOIL_EVAPORATION_DEPTH_M {
            break;
        }

        let withdrawable_m = if cumulative_depth_m > WB17_SOIL_EVAPORATION_DEPTH_M {
            let layer_interval_m = cumulative_depth_m - previous_depth_m;
            validate_positive(
                "evapotranspiration.soil_evaporation_layer_interval_m",
                layer_interval_m,
            )?;
            let evaporation_interval_m =
                (WB17_SOIL_EVAPORATION_DEPTH_M - previous_depth_m).max(0.0);
            layer.theta_m * evaporation_interval_m / layer_interval_m
        } else {
            layer.theta_m
        };
        validate_nonnegative_direct_m(
            "evapotranspiration.soil_evaporation_withdrawable_m",
            withdrawable_m,
        )?;
        if withdrawable_m > 0.0 {
            let withdrawn_m = remaining_m.min(withdrawable_m);
            layer.theta_m -= withdrawn_m;
            remaining_m -= withdrawn_m;
            layer_withdrawal_m[index] = withdrawn_m;
            if layer.theta_m < 1.0e-10 {
                layer.theta_m = 0.0;
            }
        }
        if cumulative_depth_m > WB17_SOIL_EVAPORATION_DEPTH_M {
            break;
        }
    }
    let soil_evaporation_actual_m = demand_m - remaining_m;
    validate_nonnegative_direct_m(
        "evapotranspiration.soil_evaporation_actual_m",
        soil_evaporation_actual_m,
    )?;
    Ok((soil_evaporation_actual_m, layer_withdrawal_m))
}

fn apply_post_et_upper_limit_redistribution(
    layers: &mut [DirectSubsurfaceLayerState],
    outside_water_active: bool,
) -> Result<(), DirectRuntimeError> {
    for index in (1..layers.len()).rev() {
        let active_cap_m = if outside_water_active {
            (layers[index].upper_limit_m - layers[index].frozen_water_m).max(0.0)
        } else {
            layers[index].upper_limit_m
        };
        validate_nonnegative_direct_m("evapotranspiration.active_cap_m", active_cap_m)?;
        if layers[index].theta_m > active_cap_m + WB11_ZERO_THRESHOLD {
            let excess_m = layers[index].theta_m - active_cap_m;
            layers[index].theta_m = active_cap_m;
            layers[index - 1].theta_m += excess_m;
            validate_nonnegative_direct_m(
                "evapotranspiration.redistributed_layer_theta_m",
                layers[index].theta_m,
            )?;
            validate_nonnegative_direct_m(
                "evapotranspiration.redistributed_layer_theta_m",
                layers[index - 1].theta_m,
            )?;
        }
    }
    Ok(())
}

fn aggregate_soil_water(layers: &[DirectSubsurfaceLayerState]) -> Result<f64, DirectRuntimeError> {
    let mut soil_water_m = 0.0;
    for layer in layers {
        let unfrozen_depth_m = (layer.depth_m - layer.frozen_depth_m).max(0.0);
        soil_water_m += layer.theta_m + layer.residual_theta * unfrozen_depth_m;
        validate_finite("evapotranspiration.aggregate_soil_water_m", soil_water_m)?;
    }
    Ok(soil_water_m.max(0.0))
}

fn effective_swu_plant_tolerance(raw_plant_tolerance: f64) -> f64 {
    if raw_plant_tolerance <= 0.0 {
        0.25
    } else {
        raw_plant_tolerance.clamp(WB17_PLTOL_MIN, WB17_PLTOL_MAX)
    }
}

fn run_swu_root_uptake(
    layers: &mut [DirectSubsurfaceLayerState],
    transpiration_demand_m: f64,
    effective_root_depth_m: f64,
    plant_tolerance: f64,
    layer_uptake_potential_m: &mut [f64],
    layer_uptake_actual_m: &mut [f64],
    plant_transpiration_m: &mut f64,
) -> Result<(), DirectRuntimeError> {
    let mut rooted_layer_count = layers.len();
    let mut root_cumulative_depth_m = 0.0;
    for (index, layer) in layers.iter().enumerate() {
        root_cumulative_depth_m += layer.depth_m;
        if effective_root_depth_m <= root_cumulative_depth_m + WB11_ZERO_THRESHOLD {
            rooted_layer_count = index + 1;
            break;
        }
    }

    let mut previous_cumulative_potential_m = 0.0;
    let mut layer_cumulative_depth_m = 0.0;
    for index in 0..rooted_layer_count {
        layer_cumulative_depth_m += layers[index].depth_m;
        let gx_m = if index + 1 < rooted_layer_count {
            layer_cumulative_depth_m
        } else {
            effective_root_depth_m
        };
        let cumulative_potential_m = (1.0 - (-WB17_SWU_UB * gx_m / effective_root_depth_m).exp())
            * transpiration_demand_m
            / WB17_SWU_UOB;
        let mut potential_uptake_m = cumulative_potential_m - previous_cumulative_potential_m;
        if potential_uptake_m < 0.0 && potential_uptake_m.abs() <= WB11_ZERO_THRESHOLD {
            potential_uptake_m = 0.0;
        }
        validate_nonnegative_direct_m("root_uptake.potential_uptake_m", potential_uptake_m)?;
        layer_uptake_potential_m[index] = potential_uptake_m;

        let stress_threshold_m = plant_tolerance * layers[index].upper_limit_m;
        validate_nonnegative_direct_m("root_uptake.stress_threshold_m", stress_threshold_m)?;
        let mut layer_uptake_m = potential_uptake_m;
        if stress_threshold_m > 0.0 && layers[index].theta_m < stress_threshold_m {
            layer_uptake_m *= layers[index].theta_m / stress_threshold_m;
        }
        if layers[index].theta_m < layer_uptake_m {
            layer_uptake_m = layers[index].theta_m;
        }
        let remaining_transpiration_m = (transpiration_demand_m - *plant_transpiration_m).max(0.0);
        if layer_uptake_m > remaining_transpiration_m {
            layer_uptake_m = remaining_transpiration_m;
        }
        if layer_uptake_m < 1.0e-10 {
            layer_uptake_m = 0.0;
        }
        layer_uptake_actual_m[index] = layer_uptake_m;
        layers[index].theta_m -= layer_uptake_m;
        if layers[index].theta_m < 1.0e-10 {
            layers[index].theta_m = 0.0;
        }
        *plant_transpiration_m += layer_uptake_m;
        validate_nonnegative_direct_m("root_uptake.layer_theta_after_m", layers[index].theta_m)?;
        validate_nonnegative_direct_m("root_uptake.layer_uptake_m", layer_uptake_m)?;
        previous_cumulative_potential_m = cumulative_potential_m;
    }
    Ok(())
}

fn validate_positive(field: &'static str, value: f64) -> Result<(), DirectRuntimeError> {
    validate_finite(field, value)?;
    if value > WB11_ZERO_THRESHOLD {
        Ok(())
    } else {
        Err(DirectRuntimeError::DirectDomainViolation { field })
    }
}

fn validate_between(
    field: &'static str,
    value: f64,
    minimum: f64,
    maximum: f64,
) -> Result<(), DirectRuntimeError> {
    validate_finite(field, value)?;
    if value >= minimum - WB11_ZERO_THRESHOLD && value <= maximum + WB11_ZERO_THRESHOLD {
        Ok(())
    } else {
        Err(DirectRuntimeError::DirectDomainViolation { field })
    }
}

fn normalize_within_zero_tolerance(value: f64) -> Result<f64, DirectRuntimeError> {
    validate_finite("evapotranspiration.zero_tolerant_value_m", value)?;
    if value < 0.0 && value.abs() <= WB11_ZERO_THRESHOLD {
        Ok(0.0)
    } else {
        Ok(value)
    }
}
