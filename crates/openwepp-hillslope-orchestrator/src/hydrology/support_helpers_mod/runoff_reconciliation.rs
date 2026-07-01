#[allow(clippy::wildcard_imports)]
use super::super::*;
use openwepp_meteorology::surface_energy::{
    conductive_heat_flux, net_shortwave_radiation, surface_energy_balance,
    EnergyFluxWattsPerSquareMeter, PositiveLengthMeters, RadiativeFluxWattsPerSquareMeter,
    SurfaceEnergyBalanceTerms, ThermalConductivityWattsPerMeterKelvin,
};
use openwepp_unit_boundary::{FractionUnitInterval, TemperatureCelsius};

const STAGE3_RHO_WATER_KG_M3: f64 = 1_000.0;
const STAGE3_LATENT_HEAT_FUSION_J_KG: f64 = 333_550.0;
const STAGE3_SPECIFIC_HEAT_ICE_J_KG_K: f64 = 2_100.0;
const STAGE3_DEFAULT_SNOW_ALBEDO: f64 = 0.82;
const STAGE3_SECONDS_PER_HOUR: f64 = 3_600.0;
const STAGE3_LIQUID_CLOSURE_TOLERANCE_M: f64 = 1.0e-9;
const STAGE3_ENERGY_CLOSURE_TOLERANCE_J_M2: f64 = 1.0e-6;
const STAGE3_MIN_LAYER_TEMPERATURE_C: f64 = -273.15;
const STAGE3_BULK_EQUIVALENT_LAYER_CLOSURE_TOLERANCE_M: f64 = 1.0e-9;
const STAGE3_BULK_EQUIVALENT_MAX_LAYERS: usize = 16;

#[derive(Clone, Copy)]
struct Stage3AggregateState {
    swe_after_m: f64,
    depth_after_m: f64,
    density_after_kg_m3: f64,
    settle_day_count_after: f64,
}

fn inactive_direct_winter_frost_partition() -> DirectWinterFrostPartitionOutcome {
    DirectWinterFrostPartitionOutcome {
        active_frost_coupling: false,
        dthaw_after_m: 0.0,
        nft_after: 0.0,
        infcap_frz_m_s: 0.0,
        soil_water_after_frwatc_m: None,
        frwatc_soil_water_before_m: 0.0,
        frwatc_soil_water_after_m: 0.0,
        frwatc_frozen_water_before_m: 0.0,
        frwatc_frozen_water_after_m: 0.0,
        frwatc_freeze_debit_m: 0.0,
        frwatc_thaw_credit_m: 0.0,
        frwatc_net_liquid_delta_m: 0.0,
        frozen_water_after_m: 0.0,
        frost_depth_after_m: 0.0,
        thdp_after_m: 0.0,
        tfrdp_after_m: 0.0,
        tthawd_after_m: 0.0,
        fgthwd_flag_after: 0.0,
        total_fine_layer_count: 0.0,
        conductivity_tilled_w_m_k: 0.0,
        conductivity_untilled_w_m_k: 0.0,
        conductivity_residue_w_m_k: 0.0,
        shadow_total_water_before_m: 0.0,
        shadow_total_water_after_m: 0.0,
        shadow_wb_delta_m: 0.0,
        shadow_frwatc_residual_m: 0.0,
        watpdg_m: 0.0,
        watbtm_m: 0.0,
        layer_projection: Vec::new(),
        layer_shadow_projection: Vec::new(),
        fine_layer_projection: Vec::new(),
    }
}

fn active_direct_winter_frost_partition(
    frost_coupling: &FrostCouplingOutcome,
) -> DirectWinterFrostPartitionOutcome {
    DirectWinterFrostPartitionOutcome {
        active_frost_coupling: true,
        dthaw_after_m: frost_coupling.dthaw,
        nft_after: frost_coupling.nft,
        infcap_frz_m_s: frost_coupling.infcap_frz,
        soil_water_after_frwatc_m: frost_coupling.soil_water_after_frwatc,
        frwatc_soil_water_before_m: frost_coupling.frwatc_soil_water_before,
        frwatc_soil_water_after_m: frost_coupling.frwatc_soil_water_after,
        frwatc_frozen_water_before_m: frost_coupling.frwatc_frozen_water_before,
        frwatc_frozen_water_after_m: frost_coupling.frwatc_frozen_water_after,
        frwatc_freeze_debit_m: frost_coupling.frwatc_freeze_debit,
        frwatc_thaw_credit_m: frost_coupling.frwatc_thaw_credit,
        frwatc_net_liquid_delta_m: frost_coupling.frwatc_net_liquid_delta,
        frozen_water_after_m: frost_coupling.frwatc_frozen_water_after,
        frost_depth_after_m: frost_coupling.frdp_m,
        thdp_after_m: frost_coupling.thdp_m,
        tfrdp_after_m: frost_coupling.tfrdp_m,
        tthawd_after_m: frost_coupling.tthawd_m,
        fgthwd_flag_after: frost_coupling.fgthwd_flag,
        total_fine_layer_count: frost_coupling.total_fine_layer_count,
        conductivity_tilled_w_m_k: frost_coupling.conductivity_tilled_w_m_k,
        conductivity_untilled_w_m_k: frost_coupling.conductivity_untilled_w_m_k,
        conductivity_residue_w_m_k: frost_coupling.conductivity_residue_w_m_k,
        shadow_total_water_before_m: frost_coupling.shadow_total_water_before_m,
        shadow_total_water_after_m: frost_coupling.shadow_total_water_after_m,
        shadow_wb_delta_m: frost_coupling.shadow_wb_delta_m,
        shadow_frwatc_residual_m: frost_coupling.shadow_frwatc_residual_m,
        watpdg_m: frost_coupling.watpdg_m,
        watbtm_m: frost_coupling.watbtm_m,
        layer_projection: frost_coupling
            .layer_topology_state
            .iter()
            .map(|layer| DirectFrostLayerProjection {
                layer_index: layer.layer_index,
                theta_after_m: layer.theta_after_m,
                frozen_depth_m: layer.frozen_depth_m,
                frozen_water_m: layer.frzw_m,
            })
            .collect(),
        layer_shadow_projection: frost_coupling
            .shadow_layer_state
            .iter()
            .map(|layer| DirectFrostLayerShadowProjection {
                layer_index: layer.layer_index,
                st_m: layer.st_m,
                soil_water_m: layer.soil_water_m,
                frozen_depth_m: layer.frozen_depth_m,
                frozen_water_m: layer.frzw_m,
                soilf_m: layer.soilf_m,
                yst_m: layer.yst_m,
                nwfrzz_m: layer.nwfrzz_m,
            })
            .collect(),
        fine_layer_projection: frost_coupling
            .fine_layer_state
            .iter()
            .map(|fine| DirectFrostFineLayerProjection {
                layer_index: fine.layer_index,
                fine_index: fine.fine_index,
                fgfrst: fine.fgfrst,
                slfsd_m: fine.slfsd_m,
                slsic_m: fine.slsic_m,
                slsw_theta: fine.slsw_theta,
                sltime_s: fine.sltime_s,
            })
            .collect(),
    }
}

impl Wb11HydrologyKernel {
    pub(crate) fn resolve_snow_partition_terms(
        phase_class: HillslopeKernelPhaseClass,
        hyetograph_rainfall: f64,
        snow_coupling: &SnowCouplingOutcome,
    ) -> Result<(f64, f64), Wb11HydrologyKernelGuardError> {
        let runoff_snow_term = snow_coupling.signed_s
            + snow_coupling.accumulation
            + snow_coupling.rain_retained
            + snow_coupling.rain_released;
        Self::require_dynamic_state_range(
            phase_class,
            BoundarySymbol::from("snow.routed_melt_m"),
            runoff_snow_term,
            Some(0.0),
            None,
        )?;
        let runoff_snow_term = Self::normalize_non_negative_within_tolerance(runoff_snow_term);
        let hyetograph_liquid_input_raw = hyetograph_rainfall
            - snow_coupling.accumulation
            - snow_coupling.rain_retained
            - snow_coupling.rain_released;
        Self::require_state_range(
            phase_class,
            WB12_SYMBOL_RAINFALL_INPUT,
            hyetograph_liquid_input_raw,
            Some(0.0),
            None,
        )?;
        let hyetograph_liquid_input =
            Self::normalize_non_negative_within_tolerance(hyetograph_liquid_input_raw);
        Self::require_dynamic_state_range(
            phase_class,
            BoundarySymbol::from("snow.post_winter_rain_m"),
            hyetograph_liquid_input,
            Some(0.0),
            None,
        )?;

        Ok((runoff_snow_term, hyetograph_liquid_input))
    }

    pub fn compute_direct_winter_frost_partition(
        inputs: &DirectActiveFrostPartitionInputs,
    ) -> Result<DirectWinterFrostPartitionOutcome, Wb11HydrologyKernelGuardError> {
        let phase_class = HillslopeKernelPhaseClass::HydrologyRunoffReconciliation;
        Self::require_state_range(
            phase_class,
            WB14_SYMBOL_SOIL_CONDUCTIVITY,
            inputs.soil_conductivity_m_s,
            Some(0.0),
            None,
        )?;
        if !inputs.controls.wint_red_enabled {
            return Ok(inactive_direct_winter_frost_partition());
        }
        let frost_coupling = Self::compute_active_frost_coupling_from_typed(phase_class, inputs)?;
        Ok(active_direct_winter_frost_partition(&frost_coupling))
    }

    pub fn compute_direct_snow_liquid_partition_from_typed(
        inputs: &DirectActiveSnowPartitionInputs,
    ) -> Result<DirectSnowLiquidPartition, Wb11HydrologyKernelGuardError> {
        let phase_class = HillslopeKernelPhaseClass::HydrologyRunoffReconciliation;
        Self::require_direct_typed_snow_value(
            phase_class,
            BoundarySymbol::from(WB12_SYMBOL_RAINFALL_INPUT),
            inputs.hyetograph_rainfall_m,
            Some(0.0),
            None,
        )?;
        let active_snow_coupling =
            if inputs.hyetograph_rainfall_m <= WB11_ZERO_THRESHOLD
                && inputs.runtime_swe_m <= WB11_ZERO_THRESHOLD
            {
                false
            } else {
                inputs.runtime_swe_m > WB11_ZERO_THRESHOLD
                    || f64::midpoint(inputs.tmax_c, inputs.tmin_c) < 0.0
            };
        let snow_coupling = if active_snow_coupling {
            Self::compute_active_snow_coupling_from_typed(phase_class, inputs)?
        } else {
            Self::inactive_snow_coupling_from_typed(phase_class, inputs)?
        };
        let (routed_melt_m, post_winter_rain_m) =
            Self::resolve_snow_partition_terms(phase_class, inputs.hyetograph_rainfall_m, &snow_coupling)?;
        let density_outcome = Self::resolve_typed_snow_density_outcome(
            phase_class,
            inputs,
            &snow_coupling,
            routed_melt_m,
        )?;
        let mut snow_layers_after = density_outcome.layers_after;
        let stage3_diagnostics = Self::resolve_stage3_liquid_routing(
            phase_class,
            inputs,
            routed_melt_m,
            Stage3AggregateState {
                swe_after_m: density_outcome.runtime_swe_after_m,
                depth_after_m: density_outcome.runtime_depth_after_m,
                density_after_kg_m3: density_outcome.runtime_density_after_kg_m3,
                settle_day_count_after: snow_coupling.runtime_settle_day_count,
            },
            &mut snow_layers_after,
        )?;

        Ok(DirectSnowLiquidPartition {
            active_snow_coupling,
            snow_density_model: inputs.snow_density_model,
            snow_coupling_signed_s_m: snow_coupling.signed_s,
            raw_melt_m: snow_coupling.raw_melt,
            redistributed_melt_m: snow_coupling.redistributed_melt,
            routed_melt_m,
            snowpack_swe_loss_m: snow_coupling.snowpack_state_loss,
            accumulation_m: snow_coupling.accumulation,
            rain_retained_m: snow_coupling.rain_retained,
            rain_released_m: snow_coupling.rain_released,
            liquid_holding_capacity_after_m: snow_coupling.liquid_holding_capacity,
            liquid_water_retained_after_m: snow_coupling.liquid_water_retained,
            liquid_water_released_m: snow_coupling.liquid_water_released,
            sublimation_m: snow_coupling.sublimation,
            post_winter_rain_m,
            runtime_swe_after_m: density_outcome.runtime_swe_after_m,
            runtime_depth_after_m: density_outcome.runtime_depth_after_m,
            runtime_density_after_kg_m3: density_outcome.runtime_density_after_kg_m3,
            runtime_settle_day_count_after: snow_coupling.runtime_settle_day_count,
            coe_boundary_depth_after_m: density_outcome.coe_boundary_depth_after_m,
            coe_boundary_density_after_kg_m3: density_outcome.coe_boundary_density_after_kg_m3,
            coe_boundary_settle_day_count_after: snow_coupling.runtime_settle_day_count,
            density_swe_identity_residual_m: density_outcome.max_abs_swe_identity_residual_m,
            density_unbounded_swe_residual_m: density_outcome.max_abs_unbounded_swe_residual_m,
            snow_albedo_state_after: snow_coupling.snow_albedo_state_after,
            snow_layers_after,
            stage3_diagnostics,
        })
    }

    fn inactive_snow_coupling_from_typed(
        phase_class: HillslopeKernelPhaseClass,
        inputs: &DirectActiveSnowPartitionInputs,
    ) -> Result<SnowCouplingOutcome, Wb11HydrologyKernelGuardError> {
        Self::require_direct_typed_snow_value(
            phase_class,
            BoundarySymbol::from(WB14_SYMBOL_SNOW_RUNTIME_SWE),
            inputs.runtime_swe_m,
            Some(0.0),
            None,
        )?;
        Self::require_direct_typed_snow_value(
            phase_class,
            BoundarySymbol::from(SNOW_RUNTIME_DEPTH_M_SYMBOL),
            inputs.runtime_depth_m,
            Some(0.0),
            None,
        )?;
        Self::require_direct_typed_snow_value(
            phase_class,
            BoundarySymbol::from(SNOW_RUNTIME_DENSITY_KG_M3_SYMBOL),
            inputs.runtime_density_kg_m3,
            Some(0.0),
            Some(SIMIMPL29_SNOW_DENSITY_CAP_KG_M3),
        )?;
        Self::require_direct_typed_snow_value(
            phase_class,
            BoundarySymbol::from(SNOW_RUNTIME_SETTLE_DAY_COUNT_SYMBOL),
            inputs.runtime_settle_day_count,
            Some(0.0),
            None,
        )?;
        Ok(SnowCouplingOutcome {
            signed_s: 0.0,
            accumulation: 0.0,
            rain_retained: 0.0,
            rain_released: 0.0,
            liquid_holding_capacity: 0.0,
            liquid_water_retained: inputs.liquid_water_retained_m,
            liquid_water_released: 0.0,
            sublimation: 0.0,
            raw_melt: 0.0,
            redistributed_melt: 0.0,
            snowpack_state_loss: 0.0,
            runtime_swe: inputs.runtime_swe_m,
            runtime_depth_m: inputs.runtime_depth_m,
            runtime_density_kg_m3: inputs.runtime_density_kg_m3,
            runtime_settle_day_count: inputs.runtime_settle_day_count,
            snow_albedo_state_after: inputs.snow_albedo_state,
        })
    }

    fn resolve_stage3_liquid_routing(
        phase_class: HillslopeKernelPhaseClass,
        inputs: &DirectActiveSnowPartitionInputs,
        incoming_liquid_m: f64,
        aggregate: Stage3AggregateState,
        layers: &mut Vec<DirectSnowLayerState>,
    ) -> Result<DirectSnowStage3Diagnostics, Wb11HydrologyKernelGuardError> {
        if !Self::stage3_liquid_routing_enabled(phase_class, inputs, incoming_liquid_m)? {
            return Ok(DirectSnowStage3Diagnostics::disabled());
        }
        Self::prepare_stage3_layer_stack(phase_class, inputs, aggregate, layers)?;
        if layers.is_empty() {
            let meltwater_temperature_c = if incoming_liquid_m > WB11_ZERO_THRESHOLD {
                Some(Self::stage3_temperature(phase_class, 0.0)?)
            } else {
                None
            };
            return Ok(DirectSnowStage3Diagnostics {
                enabled: true,
                meltwater_temperature_c,
                incoming_liquid_m,
                routed_liquid_m: incoming_liquid_m,
                ..DirectSnowStage3Diagnostics::disabled()
            });
        }

        let mut cold_content_by_layer = Vec::with_capacity(layers.len());
        let mut cold_content_before_j_m2 = 0.0;
        for layer in layers.iter() {
            Self::validate_stage3_layer(phase_class, layer)?;
            let cold_content = Self::stage3_layer_cold_content_j_m2(layer);
            cold_content_by_layer.push(cold_content);
            cold_content_before_j_m2 += cold_content;
        }

        let hourly_surface_energy_potential_j_m2 =
            Self::stage3_hourly_surface_energy_potentials_j_m2(phase_class, inputs)?;
        let mut surface_energy_j_m2 = 0.0;
        let mut conduction_energy_j_m2 = 0.0;
        for hourly_surface_energy_j_m2 in hourly_surface_energy_potential_j_m2 {
            surface_energy_j_m2 += Self::apply_stage3_surface_energy(
                hourly_surface_energy_j_m2,
                &mut cold_content_by_layer,
            );
            conduction_energy_j_m2 += Self::apply_stage3_interlayer_conduction(
                phase_class,
                layers,
                &mut cold_content_by_layer,
            )?;
        }
        let (routed_liquid_m, retained_delta_m, refrozen_liquid_m) =
            Self::route_stage3_liquid_through_layers(
                incoming_liquid_m,
                layers,
                &mut cold_content_by_layer,
            );

        let cold_content_after_j_m2 = cold_content_by_layer.iter().sum::<f64>();
        let latent_refreeze_energy_j_m2 =
            refrozen_liquid_m * STAGE3_LATENT_HEAT_FUSION_J_KG * STAGE3_RHO_WATER_KG_M3;
        let liquid_closure_residual_m =
            incoming_liquid_m - routed_liquid_m - retained_delta_m - refrozen_liquid_m;
        let energy_closure_residual_j_m2 =
            surface_energy_j_m2 + conduction_energy_j_m2 + latent_refreeze_energy_j_m2
                - (cold_content_before_j_m2 - cold_content_after_j_m2);

        Self::require_direct_typed_snow_value(
            phase_class,
            BoundarySymbol::from("snow.stage3_liquid_closure_residual_m"),
            liquid_closure_residual_m.abs(),
            None,
            Some(STAGE3_LIQUID_CLOSURE_TOLERANCE_M),
        )?;
        Self::require_direct_typed_snow_value(
            phase_class,
            BoundarySymbol::from("snow.stage3_energy_residual_j_m2"),
            energy_closure_residual_j_m2.abs(),
            None,
            Some(STAGE3_ENERGY_CLOSURE_TOLERANCE_J_M2),
        )?;

        let meltwater_temperature_c = if routed_liquid_m > WB11_ZERO_THRESHOLD {
            Some(Self::stage3_temperature(phase_class, 0.0)?)
        } else {
            None
        };

        Ok(DirectSnowStage3Diagnostics {
            enabled: true,
            meltwater_temperature_c,
            incoming_liquid_m,
            routed_liquid_m,
            retained_liquid_m: retained_delta_m,
            refrozen_liquid_m,
            liquid_closure_residual_m,
            cold_content_before_j_m2,
            cold_content_after_j_m2,
            surface_energy_j_m2,
            conduction_energy_j_m2,
            latent_refreeze_energy_j_m2,
            energy_closure_residual_j_m2,
        })
    }

    fn stage3_liquid_routing_enabled(
        phase_class: HillslopeKernelPhaseClass,
        inputs: &DirectActiveSnowPartitionInputs,
        incoming_liquid_m: f64,
    ) -> Result<bool, Wb11HydrologyKernelGuardError> {
        if inputs.stage3_liquid_routing_model == SnowStage3LiquidRoutingModel::Disabled {
            return Ok(false);
        }
        if inputs.stage3_liquid_routing_model
            != SnowStage3LiquidRoutingModel::LayeredThermalLiquidV1
        {
            return Err(Self::stage3_domain_error(
                phase_class,
                "snow.stage3_liquid_routing_model",
                1.0,
                Some(0.0),
                Some(0.0),
            ));
        }
        if !matches!(
            inputs.snow_density_model,
            SnowDensityModel::PhysicsBulkDensityCompactionV1
                | SnowDensityModel::PhysicsBulkMultilayerDensityV1
        ) {
            return Err(Self::stage3_domain_error(
                phase_class,
                "snow.stage3_requires_bulk_or_multilayer_density_model",
                1.0,
                Some(0.0),
                Some(0.0),
            ));
        }
        Self::require_direct_typed_snow_value(
            phase_class,
            BoundarySymbol::from("snow.stage3_incoming_liquid_m"),
            incoming_liquid_m,
            Some(0.0),
            None,
        )?;
        Ok(true)
    }

    fn prepare_stage3_layer_stack(
        phase_class: HillslopeKernelPhaseClass,
        inputs: &DirectActiveSnowPartitionInputs,
        aggregate: Stage3AggregateState,
        layers: &mut Vec<DirectSnowLayerState>,
    ) -> Result<(), Wb11HydrologyKernelGuardError> {
        if inputs.snow_density_model == SnowDensityModel::PhysicsBulkMultilayerDensityV1 {
            if aggregate.swe_after_m > WB11_ZERO_THRESHOLD && layers.is_empty() {
                return Err(Self::stage3_domain_error(
                    phase_class,
                    "snow.stage3_missing_layers_with_snow",
                    aggregate.swe_after_m,
                    None,
                    Some(0.0),
                ));
            }
            return Ok(());
        }

        if aggregate.swe_after_m <= WB11_ZERO_THRESHOLD {
            layers.clear();
            return Ok(());
        }
        Self::require_direct_typed_snow_value(
            phase_class,
            BoundarySymbol::from("snow.stage3_bulk_equivalent_runtime_depth_m"),
            aggregate.depth_after_m,
            Some(WB11_ZERO_THRESHOLD),
            None,
        )?;
        Self::require_direct_typed_snow_value(
            phase_class,
            BoundarySymbol::from("snow.stage3_bulk_equivalent_density_kg_m3"),
            aggregate.density_after_kg_m3,
            Some(WB11_ZERO_THRESHOLD),
            Some(SIMIMPL29_SNOW_DENSITY_CAP_KG_M3),
        )?;

        if layers.is_empty() {
            layers.extend(inputs.snow_layers.iter().copied());
        }
        Self::adjust_stage3_layer_swe_to_target(
            layers,
            aggregate.swe_after_m,
            aggregate.depth_after_m,
            aggregate.density_after_kg_m3,
            aggregate.settle_day_count_after,
        );
        Self::merge_stage3_bulk_equivalent_bottom_layers(layers);
        Self::apply_stage3_bulk_equivalent_density(phase_class, layers, aggregate)
    }

    fn adjust_stage3_layer_swe_to_target(
        layers: &mut Vec<DirectSnowLayerState>,
        target_swe_m: f64,
        target_depth_m: f64,
        target_density_kg_m3: f64,
        settle_day_count: f64,
    ) {
        layers.retain(|layer| layer.mass_swe_m > WB11_ZERO_THRESHOLD);
        let mut current_swe_m = layers.iter().map(|layer| layer.mass_swe_m).sum::<f64>();
        if current_swe_m <= WB11_ZERO_THRESHOLD {
            layers.push(DirectSnowLayerState::new(
                target_swe_m,
                target_depth_m,
                target_density_kg_m3,
                settle_day_count,
            ));
            return;
        }

        if current_swe_m > target_swe_m + STAGE3_BULK_EQUIVALENT_LAYER_CLOSURE_TOLERANCE_M {
            let mut remaining_removal_m = current_swe_m - target_swe_m;
            while remaining_removal_m > STAGE3_BULK_EQUIVALENT_LAYER_CLOSURE_TOLERANCE_M
                && !layers.is_empty()
            {
                if layers[0].mass_swe_m <= remaining_removal_m
                    + STAGE3_BULK_EQUIVALENT_LAYER_CLOSURE_TOLERANCE_M
                {
                    remaining_removal_m -= layers[0].mass_swe_m;
                    layers.remove(0);
                } else {
                    let original_mass_m = layers[0].mass_swe_m;
                    let retained_fraction =
                        ((original_mass_m - remaining_removal_m) / original_mass_m).max(0.0);
                    layers[0].mass_swe_m -= remaining_removal_m;
                    layers[0].liquid_water_m *= retained_fraction;
                    layers[0].cold_content_j_m2 *= retained_fraction;
                    layers[0].refrozen_liquid_m *= retained_fraction;
                    remaining_removal_m = 0.0;
                }
            }
        } else if target_swe_m
            > current_swe_m + STAGE3_BULK_EQUIVALENT_LAYER_CLOSURE_TOLERANCE_M
        {
            let added_swe_m = target_swe_m - current_swe_m;
            layers.insert(
                0,
                DirectSnowLayerState::new(
                    added_swe_m,
                    added_swe_m * STAGE3_RHO_WATER_KG_M3 / target_density_kg_m3,
                    target_density_kg_m3,
                    settle_day_count,
                ),
            );
        }

        current_swe_m = layers.iter().map(|layer| layer.mass_swe_m).sum::<f64>();
        if let Some(surface) = layers.first_mut() {
            surface.mass_swe_m += target_swe_m - current_swe_m;
        }
    }

    fn merge_stage3_bulk_equivalent_bottom_layers(layers: &mut Vec<DirectSnowLayerState>) {
        while layers.len() > STAGE3_BULK_EQUIVALENT_MAX_LAYERS {
            let Some(bottom) = layers.pop() else {
                break;
            };
            let Some(previous_bottom) = layers.last_mut() else {
                layers.push(bottom);
                break;
            };
            let combined_mass_m = previous_bottom.mass_swe_m + bottom.mass_swe_m;
            if combined_mass_m > WB11_ZERO_THRESHOLD {
                previous_bottom.settle_day_count = (previous_bottom.settle_day_count
                    * previous_bottom.mass_swe_m
                    + bottom.settle_day_count * bottom.mass_swe_m)
                    / combined_mass_m;
            }
            previous_bottom.mass_swe_m = combined_mass_m;
            previous_bottom.liquid_water_m += bottom.liquid_water_m;
            previous_bottom.cold_content_j_m2 += bottom.cold_content_j_m2;
            previous_bottom.refrozen_liquid_m += bottom.refrozen_liquid_m;
        }
    }

    fn apply_stage3_bulk_equivalent_density(
        phase_class: HillslopeKernelPhaseClass,
        layers: &mut [DirectSnowLayerState],
        aggregate: Stage3AggregateState,
    ) -> Result<(), Wb11HydrologyKernelGuardError> {
        if layers.is_empty() && aggregate.swe_after_m > WB11_ZERO_THRESHOLD {
            return Err(Self::stage3_domain_error(
                phase_class,
                "snow.stage3_missing_layers_with_snow",
                aggregate.swe_after_m,
                None,
                Some(0.0),
            ));
        }
        for layer in layers.iter_mut() {
            if layer.settle_day_count <= WB11_ZERO_THRESHOLD {
                layer.settle_day_count = aggregate.settle_day_count_after;
            }
            layer.density_kg_m3 = aggregate.density_after_kg_m3;
            layer.thickness_m = layer.mass_swe_m * STAGE3_RHO_WATER_KG_M3
                / aggregate.density_after_kg_m3;
            layer.cold_content_j_m2 = Self::stage3_layer_cold_content_j_m2(layer);
            layer.temperature_c = Self::stage3_temperature_from_cold_content(layer);
            layer.refrozen_liquid_m = layer.refrozen_liquid_m.max(0.0);
            layer.liquid_water_m = layer.liquid_water_m.max(0.0);
        }

        let layer_swe_sum_m = layers.iter().map(|layer| layer.mass_swe_m).sum::<f64>();
        let layer_depth_sum_m = layers.iter().map(|layer| layer.thickness_m).sum::<f64>();
        Self::require_direct_typed_snow_value(
            phase_class,
            BoundarySymbol::from("snow.stage3_bulk_equivalent_layer_swe_residual_m"),
            (layer_swe_sum_m - aggregate.swe_after_m).abs(),
            None,
            Some(STAGE3_BULK_EQUIVALENT_LAYER_CLOSURE_TOLERANCE_M),
        )?;
        Self::require_direct_typed_snow_value(
            phase_class,
            BoundarySymbol::from("snow.stage3_bulk_equivalent_layer_depth_residual_m"),
            (layer_depth_sum_m - aggregate.depth_after_m).abs(),
            None,
            Some(STAGE3_BULK_EQUIVALENT_LAYER_CLOSURE_TOLERANCE_M),
        )
    }

    fn route_stage3_liquid_through_layers(
        incoming_liquid_m: f64,
        layers: &mut [DirectSnowLayerState],
        cold_content_by_layer: &mut [f64],
    ) -> (f64, f64, f64) {
        let mut liquid_to_route_m = incoming_liquid_m;
        let mut retained_delta_m = 0.0;
        let mut refrozen_liquid_m = 0.0;
        for (layer, cold_content) in layers.iter_mut().zip(cold_content_by_layer.iter_mut()) {
            let refreeze_capacity_m =
                (*cold_content / (STAGE3_LATENT_HEAT_FUSION_J_KG * STAGE3_RHO_WATER_KG_M3))
                    .max(0.0);
            let refrozen_here_m = liquid_to_route_m.min(refreeze_capacity_m);
            liquid_to_route_m -= refrozen_here_m;
            *cold_content -=
                refrozen_here_m * STAGE3_LATENT_HEAT_FUSION_J_KG * STAGE3_RHO_WATER_KG_M3;
            refrozen_liquid_m += refrozen_here_m;

            let capacity_m =
                Self::stage3_layer_liquid_holding_capacity_m(layer.thickness_m, layer.density_kg_m3);
            let available_capacity_m = (capacity_m - layer.liquid_water_m).max(0.0);
            let retained_here_m = liquid_to_route_m.min(available_capacity_m);
            liquid_to_route_m -= retained_here_m;
            retained_delta_m += retained_here_m;

            layer.liquid_water_m += retained_here_m;
            layer.refrozen_liquid_m = refrozen_here_m;
            layer.cold_content_j_m2 = (*cold_content).max(0.0);
            layer.temperature_c = Self::stage3_temperature_from_cold_content(layer);
        }
        (liquid_to_route_m.max(0.0), retained_delta_m, refrozen_liquid_m)
    }

    fn validate_stage3_layer(
        phase_class: HillslopeKernelPhaseClass,
        layer: &DirectSnowLayerState,
    ) -> Result<(), Wb11HydrologyKernelGuardError> {
        Self::require_direct_typed_snow_value(
            phase_class,
            BoundarySymbol::from("snow.stage3_layer_mass_swe_m"),
            layer.mass_swe_m,
            Some(0.0),
            None,
        )?;
        Self::require_direct_typed_snow_value(
            phase_class,
            BoundarySymbol::from("snow.stage3_layer_thickness_m"),
            layer.thickness_m,
            Some(0.0),
            None,
        )?;
        Self::require_direct_typed_snow_value(
            phase_class,
            BoundarySymbol::from("snow.stage3_layer_density_kg_m3"),
            layer.density_kg_m3,
            Some(0.0),
            Some(SIMIMPL29_SNOW_DENSITY_CAP_KG_M3),
        )?;
        Self::require_direct_typed_snow_value(
            phase_class,
            BoundarySymbol::from("snow.stage3_layer_temperature_c"),
            layer.temperature_c,
            None,
            Some(0.0),
        )?;
        Self::require_direct_typed_snow_value(
            phase_class,
            BoundarySymbol::from("snow.stage3_layer_liquid_water_m"),
            layer.liquid_water_m,
            Some(0.0),
            None,
        )?;
        Self::require_direct_typed_snow_value(
            phase_class,
            BoundarySymbol::from("snow.stage3_layer_cold_content_j_m2"),
            layer.cold_content_j_m2,
            Some(0.0),
            None,
        )?;
        Self::require_direct_typed_snow_value(
            phase_class,
            BoundarySymbol::from("snow.stage3_layer_refrozen_liquid_m"),
            layer.refrozen_liquid_m,
            Some(0.0),
            None,
        )
    }

    fn stage3_layer_cold_content_j_m2(layer: &DirectSnowLayerState) -> f64 {
        let cold_content = if layer.cold_content_j_m2 > WB11_ZERO_THRESHOLD {
            layer.cold_content_j_m2
        } else if layer.temperature_c >= 0.0 || layer.mass_swe_m <= WB11_ZERO_THRESHOLD {
            0.0
        } else {
            layer.mass_swe_m
                * STAGE3_RHO_WATER_KG_M3
                * STAGE3_SPECIFIC_HEAT_ICE_J_KG_K
                * (-layer.temperature_c)
        };
        cold_content
            .max(0.0)
            .min(Self::stage3_max_cold_content_j_m2(layer.mass_swe_m))
    }

    fn stage3_max_cold_content_j_m2(mass_swe_m: f64) -> f64 {
        if mass_swe_m <= WB11_ZERO_THRESHOLD {
            return 0.0;
        }
        mass_swe_m
            * STAGE3_RHO_WATER_KG_M3
            * STAGE3_SPECIFIC_HEAT_ICE_J_KG_K
            * (-STAGE3_MIN_LAYER_TEMPERATURE_C)
    }

    fn stage3_hourly_surface_energy_potentials_j_m2(
        phase_class: HillslopeKernelPhaseClass,
        inputs: &DirectActiveSnowPartitionInputs,
    ) -> Result<Vec<f64>, Wb11HydrologyKernelGuardError> {
        let albedo_value = inputs
            .snow_albedo_state
            .map_or(STAGE3_DEFAULT_SNOW_ALBEDO, |state| state.albedo);
        let albedo = FractionUnitInterval::try_new(albedo_value).map_err(|_| {
            Self::stage3_domain_error(
                phase_class,
                "snow.stage3_surface_albedo",
                albedo_value,
                Some(0.0),
                Some(1.0),
            )
        })?;
        let zero_flux = EnergyFluxWattsPerSquareMeter::try_new(0.0).map_err(|_| {
            Self::stage3_domain_error(
                phase_class,
                "snow.stage3_zero_energy_flux",
                0.0,
                None,
                None,
            )
        })?;
        let mut hourly_energy_j_m2 = Vec::with_capacity(inputs.hourly.len());
        for hourly in &inputs.hourly {
            Self::require_direct_typed_snow_value(
                phase_class,
                BoundarySymbol::from("snow.stage3_hourly_radiation_mj_m2"),
                hourly.radiation_mj_m2,
                Some(0.0),
                None,
            )?;
            let incoming_w_m2 =
                hourly.radiation_mj_m2 * 1_000_000.0 / STAGE3_SECONDS_PER_HOUR;
            let shortwave = net_shortwave_radiation(
                RadiativeFluxWattsPerSquareMeter::try_new(incoming_w_m2).map_err(|_| {
                    Self::stage3_domain_error(
                        phase_class,
                        "snow.stage3_hourly_shortwave_w_m2",
                        incoming_w_m2,
                        Some(0.0),
                        None,
                    )
                })?,
                albedo,
            )
            .map_err(|_| {
                Self::stage3_domain_error(
                    phase_class,
                    "snow.stage3_net_shortwave_w_m2",
                    incoming_w_m2,
                    None,
                    None,
                )
            })?;
            let balance = surface_energy_balance(SurfaceEnergyBalanceTerms {
                net_radiation: shortwave,
                sensible_heat: zero_flux,
                latent_heat: zero_flux,
                conduction: zero_flux,
                advected_heat: zero_flux,
            })
            .map_err(|_| {
                Self::stage3_domain_error(
                    phase_class,
                    "snow.stage3_surface_energy_balance_w_m2",
                    shortwave.as_watts_per_square_meter(),
                    None,
                    None,
                )
            })?;
            hourly_energy_j_m2
                .push(balance.as_watts_per_square_meter() * STAGE3_SECONDS_PER_HOUR);
        }
        Ok(hourly_energy_j_m2)
    }

    fn apply_stage3_interlayer_conduction(
        phase_class: HillslopeKernelPhaseClass,
        layers: &[DirectSnowLayerState],
        cold_content_by_layer: &mut [f64],
    ) -> Result<f64, Wb11HydrologyKernelGuardError> {
        for upper_index in 0..layers.len().saturating_sub(1) {
            let lower_index = upper_index + 1;
            let upper = layers[upper_index];
            let lower = layers[lower_index];
            let upper_temperature_c = Self::stage3_temperature_from_cold_content_values(
                upper.mass_swe_m,
                cold_content_by_layer[upper_index],
            );
            let lower_temperature_c = Self::stage3_temperature_from_cold_content_values(
                lower.mass_swe_m,
                cold_content_by_layer[lower_index],
            );
            let flux = conductive_heat_flux(
                ThermalConductivityWattsPerMeterKelvin::try_new(
                    Self::stage3_snow_conductivity_w_m_k(upper.density_kg_m3),
                )
                .map_err(|_| {
                    Self::stage3_domain_error(
                        phase_class,
                        "snow.stage3_upper_conductivity_w_m_k",
                        upper.density_kg_m3,
                        Some(0.0),
                        None,
                    )
                })?,
                ThermalConductivityWattsPerMeterKelvin::try_new(
                    Self::stage3_snow_conductivity_w_m_k(lower.density_kg_m3),
                )
                .map_err(|_| {
                    Self::stage3_domain_error(
                        phase_class,
                        "snow.stage3_lower_conductivity_w_m_k",
                        lower.density_kg_m3,
                        Some(0.0),
                        None,
                    )
                })?,
                Self::stage3_temperature(phase_class, upper_temperature_c)?,
                Self::stage3_temperature(phase_class, lower_temperature_c)?,
                PositiveLengthMeters::try_new(upper.thickness_m.max(WB11_ZERO_THRESHOLD))
                    .map_err(|_| {
                        Self::stage3_domain_error(
                            phase_class,
                            "snow.stage3_upper_thickness_m",
                            upper.thickness_m,
                            Some(WB11_ZERO_THRESHOLD),
                            None,
                        )
                    })?,
                PositiveLengthMeters::try_new(lower.thickness_m.max(WB11_ZERO_THRESHOLD))
                    .map_err(|_| {
                        Self::stage3_domain_error(
                            phase_class,
                            "snow.stage3_lower_thickness_m",
                            lower.thickness_m,
                            Some(WB11_ZERO_THRESHOLD),
                            None,
                        )
                    })?,
            )
            .map_err(|_| {
                Self::stage3_domain_error(
                    phase_class,
                    "snow.stage3_interlayer_conduction_w_m2",
                    upper_temperature_c - lower_temperature_c,
                    None,
                    None,
                )
            })?;
            let requested_transfer_j_m2 = flux.as_watts_per_square_meter() * STAGE3_SECONDS_PER_HOUR;
            if requested_transfer_j_m2 > 0.0 {
                let transfer_j_m2 =
                    requested_transfer_j_m2.min(cold_content_by_layer[upper_index]);
                cold_content_by_layer[upper_index] -= transfer_j_m2;
                cold_content_by_layer[lower_index] += transfer_j_m2;
            } else if requested_transfer_j_m2 < 0.0 {
                let transfer_j_m2 =
                    (-requested_transfer_j_m2).min(cold_content_by_layer[lower_index]);
                cold_content_by_layer[upper_index] += transfer_j_m2;
                cold_content_by_layer[lower_index] -= transfer_j_m2;
            }
        }
        Ok(0.0)
    }

    fn apply_stage3_surface_energy(energy_j_m2: f64, cold_content_by_layer: &mut [f64]) -> f64 {
        let Some(surface_cold_content) = cold_content_by_layer.first_mut() else {
            return 0.0;
        };
        if energy_j_m2 >= 0.0 {
            let used_j_m2 = energy_j_m2.min(*surface_cold_content);
            *surface_cold_content -= used_j_m2;
            used_j_m2
        } else {
            *surface_cold_content += -energy_j_m2;
            energy_j_m2
        }
    }

    fn stage3_layer_liquid_holding_capacity_m(
        snow_depth_m: f64,
        snow_density_kg_m3: f64,
    ) -> f64 {
        if snow_depth_m <= WB11_ZERO_THRESHOLD
            || snow_density_kg_m3 <= WB11_ZERO_THRESHOLD
            || snow_density_kg_m3 >= SIMIMPL29_RHO_ICE_KG_M3
        {
            return 0.0;
        }
        let pore_fraction = 1.0 - (snow_density_kg_m3 / SIMIMPL29_RHO_ICE_KG_M3);
        (SIMIMPL29_LIQUID_HOLDING_CAPACITY_VOLUME_FRACTION * pore_fraction * snow_depth_m)
            .max(0.0)
    }

    fn stage3_snow_conductivity_w_m_k(snow_density_kg_m3: f64) -> f64 {
        let rho_g_cm3 = snow_density_kg_m3 / 1_000.0;
        if rho_g_cm3 < 0.156 {
            0.023 + 0.234 * rho_g_cm3
        } else {
            0.138 - 1.01 * rho_g_cm3 + 3.233 * rho_g_cm3 * rho_g_cm3
        }
    }

    fn stage3_temperature_from_cold_content(layer: &DirectSnowLayerState) -> f64 {
        Self::stage3_temperature_from_cold_content_values(layer.mass_swe_m, layer.cold_content_j_m2)
    }

    fn stage3_temperature_from_cold_content_values(mass_swe_m: f64, cold_content_j_m2: f64) -> f64 {
        if cold_content_j_m2 <= WB11_ZERO_THRESHOLD || mass_swe_m <= WB11_ZERO_THRESHOLD {
            0.0
        } else {
            let capped_cold_content =
                cold_content_j_m2.min(Self::stage3_max_cold_content_j_m2(mass_swe_m));
            -capped_cold_content
                / (mass_swe_m * STAGE3_RHO_WATER_KG_M3 * STAGE3_SPECIFIC_HEAT_ICE_J_KG_K)
        }
    }

    fn stage3_temperature(
        phase_class: HillslopeKernelPhaseClass,
        value_c: f64,
    ) -> Result<TemperatureCelsius, Wb11HydrologyKernelGuardError> {
        TemperatureCelsius::try_new(value_c).map_err(|_| {
            Wb11HydrologyKernelGuardError::NonFiniteStateSymbol {
                phase_class,
                symbol: BoundarySymbol::from("snow.stage3_temperature_c"),
                value: value_c,
            }
        })
    }

    fn stage3_domain_error(
        phase_class: HillslopeKernelPhaseClass,
        symbol: &'static str,
        value: f64,
        minimum: Option<f64>,
        maximum: Option<f64>,
    ) -> Wb11HydrologyKernelGuardError {
        if !value.is_finite() {
            return Wb11HydrologyKernelGuardError::NonFiniteStateSymbol {
                phase_class,
                symbol: BoundarySymbol::from(symbol),
                value,
            };
        }
        Wb11HydrologyKernelGuardError::StateSymbolOutOfRange {
            phase_class,
            symbol: BoundarySymbol::from(symbol),
            value,
            minimum,
            maximum,
        }
    }

    fn resolve_typed_snow_density_outcome(
        phase_class: HillslopeKernelPhaseClass,
        inputs: &DirectActiveSnowPartitionInputs,
        snow_coupling: &SnowCouplingOutcome,
        routed_melt_m: f64,
    ) -> Result<SnowDensityRuntimeOutcome, Wb11HydrologyKernelGuardError> {
        let mean_air_temperature_c = inputs
            .hourly
            .iter()
            .map(|hour| hour.air_temperature_c)
            .sum::<f64>()
            / 24.0;
        update_snow_density_runtime_state(&SnowDensityRuntimeInputs {
            model: inputs.snow_density_model,
            prior_swe_m: inputs.runtime_swe_m,
            prior_depth_m: inputs.runtime_depth_m,
            prior_density_kg_m3: inputs.runtime_density_kg_m3,
            prior_settle_day_count: inputs.runtime_settle_day_count,
            prior_layers: inputs.snow_layers.clone(),
            boundary_swe_after_m: snow_coupling.runtime_swe,
            boundary_depth_after_m: snow_coupling.runtime_depth_m,
            boundary_density_after_kg_m3: snow_coupling.runtime_density_kg_m3,
            snow_input_m: snow_coupling.accumulation,
            liquid_for_compaction_m: snow_coupling.snowpack_state_loss + routed_melt_m,
            mean_air_temperature_c,
            runtime_density_cap_kg_m3: SIMIMPL29_SNOW_DENSITY_CAP_KG_M3,
            sturm_climate_class: inputs.sturm_climate_class,
            sturm_day_of_year: inputs.sturm_day_of_year,
        })
        .map_err(|error| Self::snow_density_guard_error(phase_class, &error))
    }

    fn snow_density_guard_error(
        phase_class: HillslopeKernelPhaseClass,
        error: &SnowDensityError,
    ) -> Wb11HydrologyKernelGuardError {
        match error {
            SnowDensityError::NonFiniteInput { symbol, value } => {
                Wb11HydrologyKernelGuardError::NonFiniteStateSymbol {
                    phase_class,
                    symbol: BoundarySymbol::from(*symbol),
                    value: *value,
                }
            }
            SnowDensityError::OutOfRangeInput {
                symbol,
                value,
                minimum,
                maximum,
            } => Wb11HydrologyKernelGuardError::StateSymbolOutOfRange {
                phase_class,
                symbol: BoundarySymbol::from(*symbol),
                value: *value,
                minimum: *minimum,
                maximum: *maximum,
            },
            SnowDensityError::MissingClimateClassAssignment { .. } => {
                Wb11HydrologyKernelGuardError::MissingRequiredStateSymbol {
                    phase_class,
                    symbol: BoundarySymbol::from("snow_climate_class"),
                }
            }
            SnowDensityError::MissingSturmDayOfYear { .. } => {
                Wb11HydrologyKernelGuardError::MissingRequiredStateSymbol {
                    phase_class,
                    symbol: BoundarySymbol::from("sturm2010_density_day_of_year"),
                }
            }
            SnowDensityError::MissingClimateClassDensityParameters { .. } => {
                Wb11HydrologyKernelGuardError::MissingRequiredStateSymbol {
                    phase_class,
                    symbol: BoundarySymbol::from("sturm2010_density_parameters"),
                }
            }
            SnowDensityError::LayerAggregateMismatch {
                symbol,
                value,
                expected,
            } => Wb11HydrologyKernelGuardError::StateSymbolOutOfRange {
                phase_class,
                symbol: BoundarySymbol::from(*symbol),
                value: *value,
                minimum: Some(*expected),
                maximum: Some(*expected),
            },
        }
    }

    pub(crate) fn require_direct_typed_snow_value(
        phase_class: HillslopeKernelPhaseClass,
        symbol: BoundarySymbol,
        value: f64,
        minimum: Option<f64>,
        maximum: Option<f64>,
    ) -> Result<(), Wb11HydrologyKernelGuardError> {
        if !value.is_finite() {
            return Err(Wb11HydrologyKernelGuardError::NonFiniteStateSymbol {
                phase_class,
                symbol,
                value,
            });
        }
        Self::require_dynamic_state_range(phase_class, symbol, value, minimum, maximum)
    }

    pub(crate) fn redistribute_daily_signed_snowmelt(
        hourly_state: &mut [SnowHourlyState],
    ) -> SnowMeltRedistributionOutcome {
        // SNOWSCI-S1: runtime snow storage is single-sourced from the depth/density
        // store, so routed snowpack melt must match the positive water-equivalent
        // loss already applied to that store. Negative raw melt remains available
        // through `melt_raw_m` diagnostics, but it cannot create a second SWE debit.
        let positive_melt_total_m = hourly_state
            .iter()
            .map(|hourly| hourly.melt_m.max(0.0))
            .sum::<f64>();

        if positive_melt_total_m <= WB11_ZERO_THRESHOLD {
            for hourly in hourly_state {
                hourly.melt_m = hourly.melt_m.max(0.0);
            }
            return SnowMeltRedistributionOutcome {
                routed_melt_total_m: positive_melt_total_m,
                snowpack_state_loss_m: positive_melt_total_m,
            };
        }

        for hourly in hourly_state {
            hourly.melt_m = hourly.melt_m.max(0.0);
        }
        SnowMeltRedistributionOutcome {
            routed_melt_total_m: positive_melt_total_m,
            snowpack_state_loss_m: positive_melt_total_m,
        }
    }

    pub(crate) fn normalize_non_negative_within_tolerance(value: f64) -> f64 {
        if (-WB11_ZERO_THRESHOLD..0.0).contains(&value) {
            return 0.0;
        }
        value
    }
}

#[cfg(test)]
mod cqr_row5_tests {
    use super::*;

    #[test]
    fn snow_density_guard_error_maps_all_error_variants() {
        let phase_class = HillslopeKernelPhaseClass::HydrologyRunoffReconciliation;
        let cases = [
            SnowDensityError::NonFiniteInput {
                symbol: "row5.nonfinite",
                value: f64::NAN,
            },
            SnowDensityError::OutOfRangeInput {
                symbol: "row5.range",
                value: -1.0,
                minimum: Some(0.0),
                maximum: Some(1.0),
            },
            SnowDensityError::MissingClimateClassAssignment {
                model: "sturm2010",
            },
            SnowDensityError::MissingSturmDayOfYear {
                model: "sturm2010",
            },
            SnowDensityError::MissingClimateClassDensityParameters { class: "alpine" },
            SnowDensityError::LayerAggregateMismatch {
                symbol: "row5.layers",
                value: 0.4,
                expected: 0.5,
            },
        ];

        let mapped = cases
            .iter()
            .map(|error| Wb11HydrologyKernel::snow_density_guard_error(phase_class, error))
            .collect::<Vec<_>>();

        assert!(matches!(
            mapped[0],
            Wb11HydrologyKernelGuardError::NonFiniteStateSymbol { .. }
        ));
        assert!(matches!(
            mapped[1],
            Wb11HydrologyKernelGuardError::StateSymbolOutOfRange { .. }
        ));
        assert!(matches!(
            mapped[2],
            Wb11HydrologyKernelGuardError::MissingRequiredStateSymbol { .. }
        ));
        assert!(mapped[2].to_string().contains("snow_climate_class"));
        assert!(mapped[3]
            .to_string()
            .contains("sturm2010_density_day_of_year"));
        assert!(mapped[4]
            .to_string()
            .contains("sturm2010_density_parameters"));
        assert!(mapped[5].to_string().contains("row5.layers=0.4 outside"));
    }
}
