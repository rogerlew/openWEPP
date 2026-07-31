#[allow(clippy::wildcard_imports)]
use super::super::*;
use openwepp_meteorology::surface_energy::{
    conductive_heat_flux, latent_heat_flux_from_mass_flux,
    latent_heat_for_surface_temperature, net_shortwave_radiation,
    saturation_vapor_pressure_snobal_pa, snow_effective_thermal_conductivity_snobal,
    snow_longwave_dilley_unsworth, surface_energy_balance, EnergyFluxWattsPerSquareMeter,
    MassFluxKilogramsPerSquareMeterSecond, PositiveLengthMeters, PressurePascals,
    RadiativeFluxWattsPerSquareMeter, SnowLongwaveInputs, SurfaceEnergyBalanceTerms,
    ThermalConductivityWattsPerMeterKelvin,
};
use openwepp_unit_boundary::{FractionUnitInterval, TemperatureCelsius};

const STAGE3_RHO_WATER_KG_M3: f64 = 1_000.0;
const STAGE3_LATENT_HEAT_FUSION_J_KG: f64 = 333_550.0;
const STAGE3_SPECIFIC_HEAT_ICE_J_KG_K: f64 = 2_100.0;
const STAGE3_DEFAULT_SNOW_ALBEDO: f64 = 0.82;
const STAGE3_SECONDS_PER_HOUR: f64 = 3_600.0;
const STAGE3_ACTIVE_LAYER_MAX_DEPTH_M: f64 = 0.25;
const STAGE3_NORMAL_TIMESTEP_MASS_KG_M2: f64 = 60.0;
const STAGE3_MEDIUM_TIMESTEP_MASS_KG_M2: f64 = 10.0;
const STAGE3_MEDIUM_TIMESTEP_SECONDS: f64 = 900.0;
const STAGE3_SMALL_TIMESTEP_SECONDS: f64 = 60.0;
const STAGE3_LIQUID_CLOSURE_TOLERANCE_M: f64 = 1.0e-9;
const STAGE3_ENERGY_CLOSURE_TOLERANCE_J_M2: f64 = 1.0e-6;
const STAGE3_BULK_EQUIVALENT_LAYER_CLOSURE_TOLERANCE_M: f64 = 1.0e-9;
const STAGE3_BULK_EQUIVALENT_MAX_LAYERS: usize = 16;

#[derive(Clone, Copy)]
struct Stage3AggregateState {
    swe_after_m: f64,
    depth_after_m: f64,
    density_after_kg_m3: f64,
    settle_day_count_after: f64,
}

#[derive(Clone, Copy)]
struct Stage3HourlySurfaceEnergy {
    total_j_m2: f64,
    longwave_j_m2: f64,
    latent_j_m2: f64,
    sublimation_m: f64,
    mass_latent_identity_residual_j_m2: f64,
    diagnostics: DirectSnowSurfaceEnergyHourDiagnostics,
}

#[derive(Clone, Copy)]
struct Stage3ThermalControlVolume {
    mass_swe_m: f64,
    depth_m: f64,
    density_kg_m3: f64,
    cold_content_j_m2: f64,
    conductivity_w_m_k: f64,
}

#[derive(Clone, Copy)]
struct Stage3ConductionExchange {
    requested_active_energy: f64,
    flux: f64,
    active_energy: f64,
    lower_energy: f64,
    rejected_active_energy: f64,
}

impl Stage3ConductionExchange {
    const ZERO: Self = Self {
        requested_active_energy: 0.0,
        flux: 0.0,
        active_energy: 0.0,
        lower_energy: 0.0,
        rejected_active_energy: 0.0,
    };
}

#[derive(Clone, Copy)]
struct Stage3SubstepDiagnostics {
    surface: DirectSnowSurfaceEnergyHourDiagnostics,
    duration_seconds: f64,
    applied_j_m2: f64,
    unused_j_m2: f64,
    active: Stage3ThermalControlVolume,
    lower: Option<Stage3ThermalControlVolume>,
    conduction: Stage3ConductionExchange,
    active_energy_closure_residual_j_m2: f64,
    lower_energy_closure_residual_j_m2: f64,
    atmospheric_pressure_pa: f64,
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
        Self::require_dynamic_state_range_with(
            phase_class,
            || BoundarySymbol::from("snow.routed_melt_m"),
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
        Self::require_dynamic_state_range_with(
            phase_class,
            || BoundarySymbol::from("snow.post_winter_rain_m"),
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

    // This public conservation boundary keeps the snow-coupling, density,
    // Stage-3 energy, and aggregate-state handoffs visible in one sequence.
    #[allow(clippy::too_many_lines)]
    pub fn compute_direct_snow_liquid_partition_from_typed(
        inputs: &DirectActiveSnowPartitionInputs,
    ) -> Result<DirectSnowLiquidPartition, Wb11HydrologyKernelGuardError> {
        let phase_class = HillslopeKernelPhaseClass::HydrologyRunoffReconciliation;
        Self::require_direct_typed_snow_value_with(
            phase_class,
            || BoundarySymbol::from(WB12_SYMBOL_RAINFALL_INPUT),
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
        snow_layers_after.retain(|layer| layer.mass_swe_m > WB11_ZERO_THRESHOLD);
        let runtime_swe_after_m =
            (density_outcome.runtime_swe_after_m - stage3_diagnostics.sublimation_m).max(0.0);
        let runtime_depth_after_m = if stage3_diagnostics.enabled {
            snow_layers_after
                .iter()
                .map(|layer| layer.thickness_m)
                .sum::<f64>()
        } else {
            density_outcome.runtime_depth_after_m
        };
        let runtime_density_after_kg_m3 = if runtime_swe_after_m <= WB11_ZERO_THRESHOLD {
            0.0
        } else if stage3_diagnostics.sublimation_m > 0.0 {
            (runtime_swe_after_m * STAGE3_RHO_WATER_KG_M3 / runtime_depth_after_m)
                .min(SIMIMPL29_SNOW_DENSITY_CAP_KG_M3)
        } else {
            density_outcome.runtime_density_after_kg_m3
        };
        let (coe_boundary_depth_after_m, coe_boundary_density_after_kg_m3) =
            if stage3_diagnostics.sublimation_m > 0.0 {
                (runtime_depth_after_m, runtime_density_after_kg_m3)
            } else {
                (
                    density_outcome.coe_boundary_depth_after_m,
                    density_outcome.coe_boundary_density_after_kg_m3,
                )
            };

        Ok(DirectSnowLiquidPartition {
            active_snow_coupling,
            snow_density_model: inputs.snow_density_model,
            snow_coupling_signed_s_m: snow_coupling.signed_s,
            raw_melt_m: snow_coupling.raw_melt,
            redistributed_melt_m: snow_coupling.redistributed_melt,
            routed_melt_m,
            hourly_routed_melt_m: snow_coupling.hourly_routed_melt,
            snowpack_swe_loss_m: snow_coupling.snowpack_state_loss,
            accumulation_m: snow_coupling.accumulation,
            rain_retained_m: snow_coupling.rain_retained,
            rain_released_m: snow_coupling.rain_released,
            liquid_holding_capacity_after_m: snow_coupling.liquid_holding_capacity,
            liquid_water_retained_after_m: snow_coupling.liquid_water_retained,
            liquid_water_released_m: snow_coupling.liquid_water_released,
            sublimation_m: snow_coupling.sublimation + stage3_diagnostics.sublimation_m,
            post_winter_rain_m,
            runtime_swe_after_m,
            runtime_depth_after_m,
            runtime_density_after_kg_m3,
            runtime_settle_day_count_after: snow_coupling.runtime_settle_day_count,
            coe_boundary_depth_after_m,
            coe_boundary_density_after_kg_m3,
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
        Self::require_direct_typed_snow_value_with(
            phase_class,
            || BoundarySymbol::from(WB14_SYMBOL_SNOW_RUNTIME_SWE),
            inputs.runtime_swe_m,
            Some(0.0),
            None,
        )?;
        Self::require_direct_typed_snow_value_with(
            phase_class,
            || BoundarySymbol::from(SNOW_RUNTIME_DEPTH_M_SYMBOL),
            inputs.runtime_depth_m,
            Some(0.0),
            None,
        )?;
        Self::require_direct_typed_snow_value_with(
            phase_class,
            || BoundarySymbol::from(SNOW_RUNTIME_DENSITY_KG_M3_SYMBOL),
            inputs.runtime_density_kg_m3,
            Some(0.0),
            Some(SIMIMPL29_SNOW_DENSITY_CAP_KG_M3),
        )?;
        Self::require_direct_typed_snow_value_with(
            phase_class,
            || BoundarySymbol::from(SNOW_RUNTIME_SETTLE_DAY_COUNT_SYMBOL),
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
            hourly_routed_melt: [0.0; 24],
            snowpack_state_loss: 0.0,
            runtime_swe: inputs.runtime_swe_m,
            runtime_depth_m: inputs.runtime_depth_m,
            runtime_density_kg_m3: inputs.runtime_density_kg_m3,
            runtime_settle_day_count: inputs.runtime_settle_day_count,
            snow_albedo_state_after: inputs.snow_albedo_state,
        })
    }

    // The orchestration is intentionally linear so every conservation operand
    // remains visible in the same closure boundary.
    #[allow(clippy::too_many_lines)]
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
        let mut active_layer_count: usize;

        let mut surface_energy_j_m2 = 0.0;
        let mut conduction_energy_j_m2 = 0.0;
        let mut longwave_energy_j_m2 = 0.0;
        let mut latent_energy_j_m2 = 0.0;
        let mut sublimation_m = 0.0;
        let mut cold_content_export_j_m2 = 0.0;
        let mut mass_latent_identity_residual_j_m2 = 0.0;
        let mut unused_positive_energy_j_m2 = 0.0;
        let mut hourly_surface_energy = [DirectSnowSurfaceEnergyHourDiagnostics::zero(); 24];
        for (hour_index, hourly) in inputs.hourly.iter().enumerate() {
            if layers.is_empty() {
                break;
            }
            let mut elapsed_seconds = 0.0;
            let mut hour_diagnostics = DirectSnowSurfaceEnergyHourDiagnostics::zero();
            let mut hour_latent_energy_j_m2 = 0.0;
            while elapsed_seconds < STAGE3_SECONDS_PER_HOUR && !layers.is_empty() {
                active_layer_count =
                    Self::align_stage3_active_layer_boundary(layers, &mut cold_content_by_layer);
                Self::normalize_stage3_control_volume_temperature(
                    &mut layers[..active_layer_count],
                    &mut cold_content_by_layer[..active_layer_count],
                );
                Self::normalize_stage3_control_volume_temperature(
                    &mut layers[active_layer_count..],
                    &mut cold_content_by_layer[active_layer_count..],
                );
                active_layer_count = Self::coalesce_stage3_thermal_fragments(
                    layers,
                    &mut cold_content_by_layer,
                    active_layer_count,
                );
                let requested_substep_seconds =
                    Self::stage3_substep_seconds(layers, active_layer_count);
                let substep_seconds =
                    requested_substep_seconds.min(STAGE3_SECONDS_PER_HOUR - elapsed_seconds);
                let active_state = Self::stage3_control_volume_state(
                    phase_class,
                    &layers[..active_layer_count],
                    &cold_content_by_layer[..active_layer_count],
                    inputs.surface_energy_options.atmospheric_pressure_pa,
                )?;
                let lower_state = if active_layer_count < layers.len() {
                    Some(Self::stage3_control_volume_state(
                        phase_class,
                        &layers[active_layer_count..],
                        &cold_content_by_layer[active_layer_count..],
                        inputs.surface_energy_options.atmospheric_pressure_pa,
                    )?)
                } else {
                    None
                };
                let surface_temperature_c =
                    Self::stage3_temperature_from_cold_content_values(
                        active_state.mass_swe_m,
                        active_state.cold_content_j_m2,
                    );
                let carrier = Self::stage3_hourly_surface_energy(
                    phase_class,
                    inputs,
                    *hourly,
                    surface_temperature_c,
                    active_state.depth_m,
                    active_state.density_kg_m3,
                    substep_seconds,
                )?;
                longwave_energy_j_m2 += carrier.longwave_j_m2;
                latent_energy_j_m2 += carrier.latent_j_m2;
                hour_latent_energy_j_m2 += carrier.latent_j_m2;
                mass_latent_identity_residual_j_m2 +=
                    carrier.mass_latent_identity_residual_j_m2;
                let conduction = Self::apply_stage3_active_lower_conduction(
                    phase_class,
                    layers,
                    &mut cold_content_by_layer,
                    active_layer_count,
                    substep_seconds,
                    inputs.surface_energy_options.atmospheric_pressure_pa,
                )?;
                conduction_energy_j_m2 +=
                    conduction.active_energy + conduction.lower_energy;
                let applied = Self::apply_stage3_control_volume_energy(
                    carrier.total_j_m2,
                    layers,
                    &mut cold_content_by_layer,
                    0,
                    active_layer_count,
                );
                let active_cold_content_after_fluxes_j_m2 =
                    cold_content_by_layer[..active_layer_count]
                        .iter()
                        .sum::<f64>();
                let lower_cold_content_after_fluxes_j_m2 =
                    cold_content_by_layer[active_layer_count..]
                        .iter()
                        .sum::<f64>();
                let active_energy_closure_residual_j_m2 = applied
                    + conduction.active_energy
                    - (active_state.cold_content_j_m2
                        - active_cold_content_after_fluxes_j_m2);
                let lower_energy_closure_residual_j_m2 = conduction.lower_energy
                    - (lower_state.map_or(0.0, |state| state.cold_content_j_m2)
                        - lower_cold_content_after_fluxes_j_m2);
                Self::require_direct_typed_snow_value_with(
                    phase_class,
                    || BoundarySymbol::from("snow.stage3_active_energy_residual_j_m2"),
                    active_energy_closure_residual_j_m2.abs(),
                    None,
                    Some(STAGE3_ENERGY_CLOSURE_TOLERANCE_J_M2),
                )?;
                Self::require_direct_typed_snow_value_with(
                    phase_class,
                    || BoundarySymbol::from("snow.stage3_lower_energy_residual_j_m2"),
                    lower_energy_closure_residual_j_m2.abs(),
                    None,
                    Some(STAGE3_ENERGY_CLOSURE_TOLERANCE_J_M2),
                )?;
                surface_energy_j_m2 += applied;
                let unused = (carrier.total_j_m2 - applied).max(0.0);
                unused_positive_energy_j_m2 += unused;
                Self::accumulate_stage3_hour_diagnostics(
                    &mut hour_diagnostics,
                    &Stage3SubstepDiagnostics {
                        surface: carrier.diagnostics,
                        duration_seconds: substep_seconds,
                        applied_j_m2: applied,
                        unused_j_m2: unused,
                        active: active_state,
                        lower: lower_state,
                        conduction,
                        active_energy_closure_residual_j_m2,
                        lower_energy_closure_residual_j_m2,
                        atmospheric_pressure_pa: inputs
                            .surface_energy_options
                            .atmospheric_pressure_pa,
                    },
                );
                if carrier.sublimation_m > 0.0 {
                    let (removed_m, exported_j_m2, _) =
                        Self::remove_stage3_active_sublimation(
                            carrier.sublimation_m,
                            layers,
                            &mut cold_content_by_layer,
                            &mut active_layer_count,
                        );
                    cold_content_export_j_m2 += exported_j_m2;
                    sublimation_m += removed_m;
                }
                elapsed_seconds += substep_seconds;
            }
            Self::finish_stage3_hour_diagnostics(
                &mut hour_diagnostics,
                hour_latent_energy_j_m2,
            );
            hourly_surface_energy[hour_index] = hour_diagnostics;
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
            surface_energy_j_m2
                + conduction_energy_j_m2
                + latent_refreeze_energy_j_m2
                + cold_content_export_j_m2
                - (cold_content_before_j_m2 - cold_content_after_j_m2);

        Self::require_direct_typed_snow_value_with(
            phase_class,
            || BoundarySymbol::from("snow.stage3_liquid_closure_residual_m"),
            liquid_closure_residual_m.abs(),
            None,
            Some(STAGE3_LIQUID_CLOSURE_TOLERANCE_M),
        )?;
        Self::require_direct_typed_snow_value_with(
            phase_class,
            || BoundarySymbol::from("snow.stage3_energy_residual_j_m2"),
            energy_closure_residual_j_m2.abs(),
            None,
            Some(STAGE3_ENERGY_CLOSURE_TOLERANCE_J_M2),
        )?;
        Self::require_direct_typed_snow_value_with(
            phase_class,
            || BoundarySymbol::from("snow.mass_latent_identity_residual_j_m2"),
            mass_latent_identity_residual_j_m2.abs(),
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
            longwave_energy_j_m2,
            latent_energy_j_m2,
            sublimation_m,
            cold_content_export_j_m2,
            mass_latent_identity_residual_j_m2,
            unused_positive_energy_j_m2,
            hourly_surface_energy,
        })
    }

    fn stage3_liquid_routing_enabled(
        phase_class: HillslopeKernelPhaseClass,
        inputs: &DirectActiveSnowPartitionInputs,
        incoming_liquid_m: f64,
    ) -> Result<bool, Wb11HydrologyKernelGuardError> {
        let surface_energy_enabled = inputs.surface_energy_options.longwave_model
            != SnowSurfaceLongwaveModel::Disabled
            || inputs.surface_energy_options.sublimation_model
                != SnowSurfaceSublimationModel::Disabled;
        if inputs.stage3_liquid_routing_model == SnowStage3LiquidRoutingModel::Disabled {
            if surface_energy_enabled {
                return Err(Self::stage3_domain_error(
                    phase_class,
                    "snow.surface_energy_requires_stage3_provider",
                    1.0,
                    Some(0.0),
                    Some(0.0),
                ));
            }
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
        Self::require_direct_typed_snow_value_with(
            phase_class,
            || BoundarySymbol::from("snow.stage3_incoming_liquid_m"),
            incoming_liquid_m,
            Some(0.0),
            None,
        )?;
        if inputs.surface_energy_options.sublimation_model
            != SnowSurfaceSublimationModel::Disabled
            && matches!(
                inputs.snow_melt_model,
                SnowMeltModel::CoeOpenSublimationStageAV1
                    | SnowMeltModel::CoeOpenSublimationStageBV1
            )
        {
            return Err(Self::stage3_domain_error(
                phase_class,
                "snow.incompatible_sublimation_selectors",
                1.0,
                Some(0.0),
                Some(0.0),
            ));
        }
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
        Self::require_direct_typed_snow_value_with(
            phase_class,
            || BoundarySymbol::from("snow.stage3_bulk_equivalent_runtime_depth_m"),
            aggregate.depth_after_m,
            Some(WB11_ZERO_THRESHOLD),
            None,
        )?;
        Self::require_direct_typed_snow_value_with(
            phase_class,
            || BoundarySymbol::from("snow.stage3_bulk_equivalent_density_kg_m3"),
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
        Self::require_direct_typed_snow_value_with(
            phase_class,
            || BoundarySymbol::from("snow.stage3_bulk_equivalent_layer_swe_residual_m"),
            (layer_swe_sum_m - aggregate.swe_after_m).abs(),
            None,
            Some(STAGE3_BULK_EQUIVALENT_LAYER_CLOSURE_TOLERANCE_M),
        )?;
        Self::require_direct_typed_snow_value_with(
            phase_class,
            || BoundarySymbol::from("snow.stage3_bulk_equivalent_layer_depth_residual_m"),
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
        Self::require_direct_typed_snow_value_with(
            phase_class,
            || BoundarySymbol::from("snow.stage3_layer_mass_swe_m"),
            layer.mass_swe_m,
            Some(0.0),
            None,
        )?;
        Self::require_direct_typed_snow_value_with(
            phase_class,
            || BoundarySymbol::from("snow.stage3_layer_thickness_m"),
            layer.thickness_m,
            Some(0.0),
            None,
        )?;
        Self::require_direct_typed_snow_value_with(
            phase_class,
            || BoundarySymbol::from("snow.stage3_layer_density_kg_m3"),
            layer.density_kg_m3,
            Some(0.0),
            Some(SIMIMPL29_SNOW_DENSITY_CAP_KG_M3),
        )?;
        Self::require_direct_typed_snow_value_with(
            phase_class,
            || BoundarySymbol::from("snow.stage3_layer_temperature_c"),
            layer.temperature_c,
            None,
            Some(0.0),
        )?;
        Self::require_direct_typed_snow_value_with(
            phase_class,
            || BoundarySymbol::from("snow.stage3_layer_liquid_water_m"),
            layer.liquid_water_m,
            Some(0.0),
            None,
        )?;
        Self::require_direct_typed_snow_value_with(
            phase_class,
            || BoundarySymbol::from("snow.stage3_layer_cold_content_j_m2"),
            layer.cold_content_j_m2,
            Some(0.0),
            None,
        )?;
        Self::require_direct_typed_snow_value_with(
            phase_class,
            || BoundarySymbol::from("snow.stage3_layer_refrozen_liquid_m"),
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
        cold_content.max(0.0)
    }

    // This contract adapter keeps the independently reconstructable longwave,
    // vapor-mass, latent-energy, and total-energy operands adjacent.
    #[allow(clippy::too_many_lines)]
    fn stage3_hourly_surface_energy(
        phase_class: HillslopeKernelPhaseClass,
        inputs: &DirectActiveSnowPartitionInputs,
        hourly: DirectSnowHourlyForcing,
        surface_temperature_c: f64,
        snow_depth_m: f64,
        snow_density_kg_m3: f64,
        duration_seconds: f64,
    ) -> Result<Stage3HourlySurfaceEnergy, Wb11HydrologyKernelGuardError> {
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
        Self::require_direct_typed_snow_value_with(
            phase_class,
            || BoundarySymbol::from("snow.stage3_hourly_radiation_mj_m2"),
            hourly.radiation_mj_m2,
            Some(0.0),
            None,
        )?;
        // UNIT-CONVERSION-ALLOW: contract-bound MJ m^-2 hourly energy to W m^-2.
        let incoming_w_m2 = hourly.radiation_mj_m2 * 1_000_000.0 / STAGE3_SECONDS_PER_HOUR;
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
        let mut longwave_w_m2 = 0.0;
        let mut diagnostics = DirectSnowSurfaceEnergyHourDiagnostics {
            surface_temperature_c,
            net_shortwave_w_m2: shortwave.as_watts_per_square_meter(),
            ..DirectSnowSurfaceEnergyHourDiagnostics::zero()
        };
        if inputs.surface_energy_options.longwave_model
            == SnowSurfaceLongwaveModel::DilleyUnsworthSubcanopyV1
        {
            let air_temperature = Self::stage3_temperature(phase_class, hourly.air_temperature_c)?;
            let surface_temperature =
                Self::stage3_temperature(phase_class, surface_temperature_c)?;
            let actual_vapor_pressure =
                saturation_vapor_pressure_snobal_pa(Self::stage3_temperature(
                    phase_class,
                    inputs.dewpoint_c,
                )?)
                .map_err(|_| {
                    Self::stage3_domain_error(
                        phase_class,
                        "snow.actual_vapor_pressure",
                        inputs.dewpoint_c,
                        None,
                        None,
                    )
                })?;
            let fluxes = snow_longwave_dilley_unsworth(SnowLongwaveInputs {
                air_temperature,
                surface_temperature,
                actual_vapor_pressure,
                daily_solar_radiation_mj_m2: inputs
                    .surface_energy_options
                    .daily_solar_radiation_mj_m2,
                daily_extraterrestrial_radiation_mj_m2: inputs
                    .surface_energy_options
                    .daily_extraterrestrial_radiation_mj_m2,
                daylight: inputs.surface_energy_options.daylight,
                canopy_cover: FractionUnitInterval::try_new(inputs.canopy_cover_fraction)
                    .map_err(|_| {
                        Self::stage3_domain_error(
                            phase_class,
                            "snow.canopy_cover_fraction",
                            inputs.canopy_cover_fraction,
                            Some(0.0),
                            Some(1.0),
                        )
                    })?,
            })
            .map_err(|error| match error {
                openwepp_meteorology::MeteorologyError::CloudForcingUnavailable => {
                    Wb11HydrologyKernelGuardError::MissingRequiredStateSymbol {
                        phase_class,
                        symbol: BoundarySymbol::from("snow.cloud_forcing_unavailable"),
                    }
                }
                openwepp_meteorology::MeteorologyError::OutOfAuthority { value, .. } => {
                    Self::stage3_domain_error(
                        phase_class,
                        "snow.longwave_out_of_authority",
                        value,
                        Some(0.0),
                        Some(1.0),
                    )
                }
                _ => Self::stage3_domain_error(
                    phase_class,
                    "snow.longwave_forcing",
                    inputs
                        .surface_energy_options
                        .daily_extraterrestrial_radiation_mj_m2,
                    Some(1.0e-9),
                    None,
                ),
            })?;
            longwave_w_m2 = fluxes.net_longwave.as_watts_per_square_meter();
            diagnostics.atmospheric_longwave_w_m2 =
                fluxes.atmospheric_longwave.as_watts_per_square_meter();
            diagnostics.canopy_longwave_w_m2 =
                fluxes.canopy_longwave.as_watts_per_square_meter();
            diagnostics.sky_view_fraction = fluxes.sky_view_fraction.as_fraction();
            diagnostics.subcanopy_longwave_w_m2 =
                fluxes.subcanopy_longwave.as_watts_per_square_meter();
            diagnostics.outgoing_longwave_w_m2 =
                fluxes.outgoing_longwave.as_watts_per_square_meter();
            diagnostics.net_longwave_w_m2 = longwave_w_m2;
        }
        let mut sublimation_m = 0.0;
        let mut latent_w_m2 = 0.0;
        let mut latent_heat_j_kg = 0.0;
        if inputs.surface_energy_options.sublimation_model
            == SnowSurfaceSublimationModel::NeutralBulkStage3V1
        {
            sublimation_m = Self::coe_open_sublimation_hour_m(
                phase_class,
                inputs.canopy_cover_fraction,
                inputs.wind_m_s,
                hourly.air_temperature_c,
                inputs.dewpoint_c,
                snow_depth_m,
                surface_temperature_c,
                true,
            )?
            * (duration_seconds / STAGE3_SECONDS_PER_HOUR);
            sublimation_m = sublimation_m
            .min(snow_depth_m * snow_density_kg_m3 / STAGE3_RHO_WATER_KG_M3);
            let mass_flux = MassFluxKilogramsPerSquareMeterSecond::try_new(
                -sublimation_m * STAGE3_RHO_WATER_KG_M3 / duration_seconds,
            )
            .map_err(|_| {
                Self::stage3_domain_error(
                    phase_class,
                    "snow.sublimation_mass_flux",
                    sublimation_m,
                    None,
                    None,
                )
            })?;
            let latent_heat = latent_heat_for_surface_temperature(Self::stage3_temperature(
                phase_class,
                surface_temperature_c,
            )?)
            .map_err(|_| {
                Self::stage3_domain_error(
                    phase_class,
                    "snow.latent_heat",
                    surface_temperature_c,
                    None,
                    None,
                )
            })?;
            latent_heat_j_kg = latent_heat.as_joules_per_kilogram();
            latent_w_m2 = latent_heat_flux_from_mass_flux(mass_flux, latent_heat)
                .map_err(|_| {
                    Self::stage3_domain_error(
                        phase_class,
                        "snow.latent_heat_flux",
                        sublimation_m,
                        None,
                        None,
                    )
                })?
                .as_watts_per_square_meter();
            diagnostics.vapor_mass_exchange_kg_m2 =
                -sublimation_m * STAGE3_RHO_WATER_KG_M3;
            diagnostics.latent_heat_j_kg = latent_heat_j_kg;
            diagnostics.latent_flux_w_m2 = latent_w_m2;
        }
        let zero = EnergyFluxWattsPerSquareMeter::try_new(0.0).map_err(|_| {
            Self::stage3_domain_error(phase_class, "snow.stage3_zero_flux", 0.0, None, None)
        })?;
        let balance = surface_energy_balance(SurfaceEnergyBalanceTerms {
            net_radiation: EnergyFluxWattsPerSquareMeter::try_new(
                shortwave.as_watts_per_square_meter() + longwave_w_m2,
            )
            .map_err(|_| {
                Self::stage3_domain_error(
                    phase_class,
                    "snow.stage3_net_radiation",
                    longwave_w_m2,
                    None,
                    None,
                )
            })?,
            sensible_heat: zero,
            latent_heat: EnergyFluxWattsPerSquareMeter::try_new(latent_w_m2).map_err(|_| {
                Self::stage3_domain_error(
                    phase_class,
                    "snow.stage3_latent_flux",
                    latent_w_m2,
                    None,
                    None,
                )
            })?,
            conduction: zero,
            advected_heat: zero,
        })
        .map_err(|_| {
            Self::stage3_domain_error(
                phase_class,
                "snow.stage3_surface_energy_balance",
                longwave_w_m2 + latent_w_m2,
                None,
                None,
            )
        })?;
        Ok(Stage3HourlySurfaceEnergy {
            total_j_m2: balance.as_watts_per_square_meter() * duration_seconds,
            longwave_j_m2: longwave_w_m2 * duration_seconds,
            latent_j_m2: latent_w_m2 * duration_seconds,
            sublimation_m,
            mass_latent_identity_residual_j_m2: latent_w_m2 * duration_seconds
                - diagnostics.vapor_mass_exchange_kg_m2 * latent_heat_j_kg,
            diagnostics: DirectSnowSurfaceEnergyHourDiagnostics {
                potential_surface_energy_j_m2: balance.as_watts_per_square_meter()
                    * duration_seconds,
                ..diagnostics
            },
        })
    }

    fn align_stage3_active_layer_boundary(
        layers: &mut Vec<DirectSnowLayerState>,
        cold_content_by_layer: &mut Vec<f64>,
    ) -> usize {
        let total_depth_m = layers.iter().map(|layer| layer.thickness_m).sum::<f64>();
        let target_depth_m = total_depth_m.min(STAGE3_ACTIVE_LAYER_MAX_DEPTH_M);
        let mut depth_above_m = 0.0;
        for index in 0..layers.len() {
            let layer = layers[index];
            let depth_below_m = depth_above_m + layer.thickness_m;
            if depth_below_m + WB11_ZERO_THRESHOLD < target_depth_m {
                depth_above_m = depth_below_m;
                continue;
            }
            let active_depth_in_layer_m = target_depth_m - depth_above_m;
            if active_depth_in_layer_m > WB11_ZERO_THRESHOLD
                && active_depth_in_layer_m
                    < layer.thickness_m - WB11_ZERO_THRESHOLD
            {
                let active_fraction = active_depth_in_layer_m / layer.thickness_m;
                let lower_fraction = 1.0 - active_fraction;
                let original_cold_content = cold_content_by_layer[index];
                let mut active_part = layer;
                active_part.mass_swe_m *= active_fraction;
                active_part.thickness_m = active_depth_in_layer_m;
                active_part.liquid_water_m *= active_fraction;
                active_part.cold_content_j_m2 = original_cold_content * active_fraction;
                active_part.refrozen_liquid_m *= active_fraction;
                let mut lower_part = layer;
                lower_part.mass_swe_m *= lower_fraction;
                lower_part.thickness_m -= active_depth_in_layer_m;
                lower_part.liquid_water_m *= lower_fraction;
                lower_part.cold_content_j_m2 = original_cold_content * lower_fraction;
                lower_part.refrozen_liquid_m *= lower_fraction;
                layers[index] = active_part;
                layers.insert(index + 1, lower_part);
                cold_content_by_layer[index] = active_part.cold_content_j_m2;
                cold_content_by_layer.insert(index + 1, lower_part.cold_content_j_m2);
            }
            return index + 1;
        }
        layers.len()
    }

    fn normalize_stage3_control_volume_temperature(
        layers: &mut [DirectSnowLayerState],
        cold_content_by_layer: &mut [f64],
    ) {
        if layers.is_empty() {
            return;
        }
        let total_mass_swe_m = layers.iter().map(|layer| layer.mass_swe_m).sum::<f64>();
        let total_cold_content_j_m2 = cold_content_by_layer.iter().sum::<f64>();
        let temperature_c = Self::stage3_temperature_from_cold_content_values(
            total_mass_swe_m,
            total_cold_content_j_m2,
        );
        for (layer, cold_content) in layers.iter_mut().zip(cold_content_by_layer.iter_mut()) {
            *cold_content =
                total_cold_content_j_m2 * layer.mass_swe_m / total_mass_swe_m;
            layer.cold_content_j_m2 = *cold_content;
            layer.temperature_c = temperature_c;
        }
    }

    fn coalesce_stage3_thermal_fragments(
        layers: &mut Vec<DirectSnowLayerState>,
        cold_content_by_layer: &mut Vec<f64>,
        mut active_layer_count: usize,
    ) -> usize {
        let mut index = 0;
        while index + 1 < layers.len() {
            if index + 1 == active_layer_count
                || (layers[index].density_kg_m3 - layers[index + 1].density_kg_m3).abs()
                    > WB11_ZERO_THRESHOLD
                || (layers[index].settle_day_count - layers[index + 1].settle_day_count).abs()
                    > WB11_ZERO_THRESHOLD
                || (layers[index].temperature_c - layers[index + 1].temperature_c).abs()
                    > WB11_ZERO_THRESHOLD
            {
                index += 1;
                continue;
            }
            let upper = layers[index];
            let lower = layers[index + 1];
            let merged = Self::merge_stage3_thermal_fragments(
                upper,
                lower,
                cold_content_by_layer[index] + cold_content_by_layer[index + 1],
            );
            layers[index] = merged;
            layers.remove(index + 1);
            cold_content_by_layer[index] = merged.cold_content_j_m2;
            cold_content_by_layer.remove(index + 1);
            if index + 1 < active_layer_count {
                active_layer_count -= 1;
            }
        }
        active_layer_count
    }

    fn merge_stage3_thermal_fragments(
        upper: DirectSnowLayerState,
        lower: DirectSnowLayerState,
        cold_content_j_m2: f64,
    ) -> DirectSnowLayerState {
        let mass_swe_m = upper.mass_swe_m + lower.mass_swe_m;
        let thickness_m = upper.thickness_m + lower.thickness_m;
        let mut projected = upper;
        projected.mass_swe_m = mass_swe_m;
        projected.thickness_m = thickness_m;
        projected.density_kg_m3 = mass_swe_m * STAGE3_RHO_WATER_KG_M3 / thickness_m;
        projected.settle_day_count = if mass_swe_m > WB11_ZERO_THRESHOLD {
            (upper.settle_day_count * upper.mass_swe_m
                + lower.settle_day_count * lower.mass_swe_m)
                / mass_swe_m
        } else {
            0.0
        };
        projected.liquid_water_m = upper.liquid_water_m + lower.liquid_water_m;
        projected.cold_content_j_m2 = cold_content_j_m2;
        projected.temperature_c =
            Self::stage3_temperature_from_cold_content_values(mass_swe_m, cold_content_j_m2);
        projected.refrozen_liquid_m = upper.refrozen_liquid_m + lower.refrozen_liquid_m;
        projected
    }

    fn stage3_control_volume_state(
        phase_class: HillslopeKernelPhaseClass,
        layers: &[DirectSnowLayerState],
        cold_content_by_layer: &[f64],
        atmospheric_pressure_pa: f64,
    ) -> Result<Stage3ThermalControlVolume, Wb11HydrologyKernelGuardError> {
        let mass_swe_m = layers.iter().map(|layer| layer.mass_swe_m).sum::<f64>();
        let depth_m = layers.iter().map(|layer| layer.thickness_m).sum::<f64>();
        let cold_content_j_m2 = cold_content_by_layer.iter().sum::<f64>();
        let temperature_c =
            Self::stage3_temperature_from_cold_content_values(mass_swe_m, cold_content_j_m2);
        let temperature = Self::stage3_temperature(phase_class, temperature_c)?;
        let pressure = PressurePascals::try_new(atmospheric_pressure_pa).map_err(|_| {
            Self::stage3_domain_error(
                phase_class,
                "snow.stage3_atmospheric_pressure_pa",
                atmospheric_pressure_pa,
                Some(0.0),
                None,
            )
        })?;
        let mut resistance_m2_k_w = 0.0;
        for layer in layers {
            let conductivity = snow_effective_thermal_conductivity_snobal(
                layer.density_kg_m3,
                temperature,
                pressure,
            )
            .map_err(|_| {
                Self::stage3_domain_error(
                    phase_class,
                    "snow.stage3_effective_snow_conductivity_w_m_k",
                    layer.density_kg_m3,
                    Some(0.0),
                    None,
                )
            })?;
            resistance_m2_k_w +=
                layer.thickness_m / conductivity.as_watts_per_meter_kelvin();
        }
        Ok(Stage3ThermalControlVolume {
            mass_swe_m,
            depth_m,
            density_kg_m3: mass_swe_m * STAGE3_RHO_WATER_KG_M3 / depth_m,
            cold_content_j_m2,
            conductivity_w_m_k: depth_m / resistance_m2_k_w,
        })
    }

    fn stage3_substep_seconds(
        layers: &[DirectSnowLayerState],
        active_layer_count: usize,
    ) -> f64 {
        let active_mass_kg_m2 = layers[..active_layer_count]
            .iter()
            .map(|layer| layer.mass_swe_m * STAGE3_RHO_WATER_KG_M3)
            .sum::<f64>();
        let lower_mass_kg_m2 = layers[active_layer_count..]
            .iter()
            .map(|layer| layer.mass_swe_m * STAGE3_RHO_WATER_KG_M3)
            .sum::<f64>();
        let minimum_mass_kg_m2 = if lower_mass_kg_m2 > WB11_ZERO_THRESHOLD {
            active_mass_kg_m2.min(lower_mass_kg_m2)
        } else {
            active_mass_kg_m2
        };
        if minimum_mass_kg_m2 >= STAGE3_NORMAL_TIMESTEP_MASS_KG_M2 {
            STAGE3_SECONDS_PER_HOUR
        } else if minimum_mass_kg_m2 >= STAGE3_MEDIUM_TIMESTEP_MASS_KG_M2 {
            STAGE3_MEDIUM_TIMESTEP_SECONDS
        } else {
            STAGE3_SMALL_TIMESTEP_SECONDS
        }
    }

    fn apply_stage3_control_volume_energy(
        energy_j_m2: f64,
        layers: &[DirectSnowLayerState],
        cold_content_by_layer: &mut [f64],
        start: usize,
        end: usize,
    ) -> f64 {
        if start >= end {
            return 0.0;
        }
        let cold_content_j_m2 = cold_content_by_layer[start..end].iter().sum::<f64>();
        if energy_j_m2 >= 0.0 {
            let used_j_m2 = energy_j_m2.min(cold_content_j_m2);
            if cold_content_j_m2 > WB11_ZERO_THRESHOLD {
                let retained_fraction = 1.0 - used_j_m2 / cold_content_j_m2;
                for cold_content in &mut cold_content_by_layer[start..end] {
                    *cold_content *= retained_fraction;
                }
            }
            used_j_m2
        } else {
            let total_mass_swe_m = layers[start..end]
                .iter()
                .map(|layer| layer.mass_swe_m)
                .sum::<f64>();
            for (layer, cold_content) in layers[start..end]
                .iter()
                .zip(&mut cold_content_by_layer[start..end])
            {
                *cold_content +=
                    -energy_j_m2 * layer.mass_swe_m / total_mass_swe_m;
            }
            energy_j_m2
        }
    }

    fn apply_stage3_active_lower_conduction(
        phase_class: HillslopeKernelPhaseClass,
        layers: &[DirectSnowLayerState],
        cold_content_by_layer: &mut [f64],
        active_layer_count: usize,
        duration_seconds: f64,
        atmospheric_pressure_pa: f64,
    ) -> Result<Stage3ConductionExchange, Wb11HydrologyKernelGuardError> {
        if active_layer_count >= layers.len() {
            return Ok(Stage3ConductionExchange::ZERO);
        }
        let active = Self::stage3_control_volume_state(
            phase_class,
            &layers[..active_layer_count],
            &cold_content_by_layer[..active_layer_count],
            atmospheric_pressure_pa,
        )?;
        let lower = Self::stage3_control_volume_state(
            phase_class,
            &layers[active_layer_count..],
            &cold_content_by_layer[active_layer_count..],
            atmospheric_pressure_pa,
        )?;
        let requested_transfer_j_m2 =
            Self::stage3_active_lower_conduction_energy(
                phase_class,
                active,
                lower,
                duration_seconds,
            )?;
        let exchange = if requested_transfer_j_m2 > 0.0 {
            let transfer_j_m2 = requested_transfer_j_m2.min(
                cold_content_by_layer[..active_layer_count]
                    .iter()
                    .sum::<f64>(),
            );
            Self::apply_stage3_control_volume_energy(
                transfer_j_m2,
                layers,
                cold_content_by_layer,
                0,
                active_layer_count,
            );
            Self::apply_stage3_control_volume_energy(
                -transfer_j_m2,
                layers,
                cold_content_by_layer,
                active_layer_count,
                layers.len(),
            );
            Stage3ConductionExchange {
                requested_active_energy: requested_transfer_j_m2,
                flux: transfer_j_m2 / duration_seconds,
                active_energy: transfer_j_m2,
                lower_energy: -transfer_j_m2,
                rejected_active_energy: requested_transfer_j_m2 - transfer_j_m2,
            }
        } else if requested_transfer_j_m2 < 0.0 {
            let transfer_j_m2 = (-requested_transfer_j_m2).min(
                cold_content_by_layer[active_layer_count..]
                    .iter()
                    .sum::<f64>(),
            );
            Self::apply_stage3_control_volume_energy(
                -transfer_j_m2,
                layers,
                cold_content_by_layer,
                0,
                active_layer_count,
            );
            Self::apply_stage3_control_volume_energy(
                transfer_j_m2,
                layers,
                cold_content_by_layer,
                active_layer_count,
                layers.len(),
            );
            Stage3ConductionExchange {
                requested_active_energy: requested_transfer_j_m2,
                flux: -transfer_j_m2 / duration_seconds,
                active_energy: -transfer_j_m2,
                lower_energy: transfer_j_m2,
                rejected_active_energy: requested_transfer_j_m2 + transfer_j_m2,
            }
        } else {
            Stage3ConductionExchange::ZERO
        };
        Ok(exchange)
    }

    fn stage3_active_lower_conduction_energy(
        phase_class: HillslopeKernelPhaseClass,
        active: Stage3ThermalControlVolume,
        lower: Stage3ThermalControlVolume,
        duration_seconds: f64,
    ) -> Result<f64, Wb11HydrologyKernelGuardError> {
        let active_temperature_c = Self::stage3_temperature_from_cold_content_values(
            active.mass_swe_m,
            active.cold_content_j_m2,
        );
        let lower_temperature_c = Self::stage3_temperature_from_cold_content_values(
            lower.mass_swe_m,
            lower.cold_content_j_m2,
        );
        let flux = conductive_heat_flux(
            ThermalConductivityWattsPerMeterKelvin::try_new(active.conductivity_w_m_k)
                .map_err(|_| {
                    Self::stage3_domain_error(
                        phase_class,
                        "snow.stage3_active_conductivity_w_m_k",
                        active.conductivity_w_m_k,
                        Some(0.0),
                        None,
                    )
                })?,
            ThermalConductivityWattsPerMeterKelvin::try_new(lower.conductivity_w_m_k)
                .map_err(|_| {
                    Self::stage3_domain_error(
                        phase_class,
                        "snow.stage3_lower_conductivity_w_m_k",
                        lower.conductivity_w_m_k,
                        Some(0.0),
                        None,
                    )
                })?,
            Self::stage3_temperature(phase_class, active_temperature_c)?,
            Self::stage3_temperature(phase_class, lower_temperature_c)?,
            PositiveLengthMeters::try_new(active.depth_m).map_err(|_| {
                Self::stage3_domain_error(
                    phase_class,
                    "snow.stage3_active_depth_m",
                    active.depth_m,
                    Some(0.0),
                    None,
                )
            })?,
            PositiveLengthMeters::try_new(lower.depth_m).map_err(|_| {
                Self::stage3_domain_error(
                    phase_class,
                    "snow.stage3_lower_depth_m",
                    lower.depth_m,
                    Some(0.0),
                    None,
                )
            })?,
        )
        .map_err(|_| {
            Self::stage3_domain_error(
                phase_class,
                "snow.stage3_active_lower_conduction_w_m2",
                active_temperature_c - lower_temperature_c,
                None,
                None,
            )
        })?;
        Ok(flux.as_watts_per_square_meter() * duration_seconds)
    }

    fn remove_stage3_active_sublimation(
        requested_m: f64,
        layers: &mut Vec<DirectSnowLayerState>,
        cold_content_by_layer: &mut Vec<f64>,
        active_layer_count: &mut usize,
    ) -> (f64, f64, usize) {
        let mut remaining_m = requested_m.min(
            layers[..*active_layer_count]
                .iter()
                .map(|layer| layer.mass_swe_m)
                .sum::<f64>(),
        );
        let mut removed_m = 0.0;
        let mut exported_j_m2 = 0.0;
        let mut removed_layer_count = 0;
        while remaining_m > WB11_ZERO_THRESHOLD && *active_layer_count > 0 {
            let layer_mass_m = layers[0].mass_swe_m;
            let removal_m = remaining_m.min(layer_mass_m);
            let fraction_removed = removal_m / layer_mass_m;
            let exported = cold_content_by_layer[0] * fraction_removed;
            cold_content_by_layer[0] -= exported;
            exported_j_m2 += exported;
            removed_m += removal_m;
            remaining_m -= removal_m;
            layers[0].mass_swe_m -= removal_m;
            layers[0].liquid_water_m *= 1.0 - fraction_removed;
            layers[0].refrozen_liquid_m *= 1.0 - fraction_removed;
            layers[0].thickness_m =
                layers[0].mass_swe_m * STAGE3_RHO_WATER_KG_M3 / layers[0].density_kg_m3;
            if layers[0].mass_swe_m <= WB11_ZERO_THRESHOLD {
                layers.remove(0);
                cold_content_by_layer.remove(0);
                *active_layer_count -= 1;
                removed_layer_count += 1;
            }
        }
        (removed_m, exported_j_m2, removed_layer_count)
    }

    fn accumulate_stage3_hour_diagnostics(
        hourly: &mut DirectSnowSurfaceEnergyHourDiagnostics,
        substep: &Stage3SubstepDiagnostics,
    ) {
        let Stage3SubstepDiagnostics {
            surface,
            duration_seconds,
            applied_j_m2,
            unused_j_m2,
            active,
            lower,
            conduction,
            active_energy_closure_residual_j_m2,
            lower_energy_closure_residual_j_m2,
            atmospheric_pressure_pa,
        } = *substep;
        let weight = duration_seconds / STAGE3_SECONDS_PER_HOUR;
        hourly.surface_temperature_c += surface.surface_temperature_c * weight;
        hourly.canopy_temperature_equals_air &= surface.canopy_temperature_equals_air;
        hourly.atmospheric_longwave_w_m2 += surface.atmospheric_longwave_w_m2 * weight;
        hourly.canopy_longwave_w_m2 += surface.canopy_longwave_w_m2 * weight;
        hourly.sky_view_fraction += surface.sky_view_fraction * weight;
        hourly.subcanopy_longwave_w_m2 += surface.subcanopy_longwave_w_m2 * weight;
        hourly.outgoing_longwave_w_m2 += surface.outgoing_longwave_w_m2 * weight;
        hourly.net_longwave_w_m2 += surface.net_longwave_w_m2 * weight;
        hourly.net_shortwave_w_m2 += surface.net_shortwave_w_m2 * weight;
        hourly.vapor_mass_exchange_kg_m2 += surface.vapor_mass_exchange_kg_m2;
        hourly.potential_surface_energy_j_m2 += surface.potential_surface_energy_j_m2;
        hourly.applied_surface_energy_j_m2 += applied_j_m2;
        hourly.unused_positive_energy_j_m2 += unused_j_m2;
        hourly.active_layer_mass_kg_m2 +=
            active.mass_swe_m * STAGE3_RHO_WATER_KG_M3 * weight;
        hourly.active_layer_depth_m += active.depth_m * weight;
        hourly.active_layer_temperature_c +=
            Self::stage3_temperature_from_cold_content_values(
                active.mass_swe_m,
                active.cold_content_j_m2,
            ) * weight;
        hourly.active_layer_cold_content_j_m2 += active.cold_content_j_m2 * weight;
        hourly.active_layer_effective_conductivity_w_m_k +=
            active.conductivity_w_m_k * weight;
        hourly.active_layer_thermal_resistance_m2_k_w +=
            active.depth_m / active.conductivity_w_m_k * weight;
        hourly.atmospheric_pressure_pa += atmospheric_pressure_pa * weight;
        if let Some(lower) = lower {
            hourly.lower_layer_present_fraction += weight;
            hourly.lower_layer_mass_kg_m2 +=
                lower.mass_swe_m * STAGE3_RHO_WATER_KG_M3 * weight;
            hourly.lower_layer_depth_m += lower.depth_m * weight;
            hourly.lower_layer_temperature_c +=
                Self::stage3_temperature_from_cold_content_values(
                    lower.mass_swe_m,
                    lower.cold_content_j_m2,
                ) * weight;
            hourly.lower_layer_cold_content_j_m2 += lower.cold_content_j_m2 * weight;
            hourly.lower_layer_effective_conductivity_w_m_k +=
                lower.conductivity_w_m_k * weight;
            hourly.lower_layer_thermal_resistance_m2_k_w +=
                lower.depth_m / lower.conductivity_w_m_k * weight;
            Self::accumulate_stage3_conduction_diagnostics(
                hourly,
                lower,
                substep,
                weight,
            );
        }
        hourly.active_lower_conduction_w_m2 += conduction.flux * weight;
        hourly.substep_count += 1;
        if hourly.minimum_substep_seconds == 0.0 {
            hourly.minimum_substep_seconds = duration_seconds;
        } else {
            hourly.minimum_substep_seconds =
                hourly.minimum_substep_seconds.min(duration_seconds);
        }
        hourly.maximum_active_energy_closure_residual_j_m2 = hourly
            .maximum_active_energy_closure_residual_j_m2
            .max(active_energy_closure_residual_j_m2.abs());
        hourly.maximum_lower_energy_closure_residual_j_m2 = hourly
            .maximum_lower_energy_closure_residual_j_m2
            .max(lower_energy_closure_residual_j_m2.abs());
        hourly.maximum_conduction_cancellation_residual_j_m2 = hourly
            .maximum_conduction_cancellation_residual_j_m2
            .max((conduction.active_energy + conduction.lower_energy).abs());
    }

    fn accumulate_stage3_conduction_diagnostics(
        hourly: &mut DirectSnowSurfaceEnergyHourDiagnostics,
        lower: Stage3ThermalControlVolume,
        substep: &Stage3SubstepDiagnostics,
        weight: f64,
    ) {
        let active = substep.active;
        let conduction = substep.conduction;
        let requested_conduction_w_m2 =
            conduction.requested_active_energy / substep.duration_seconds;
        let rejected_conduction_w_m2 =
            conduction.rejected_active_energy / substep.duration_seconds;
        hourly.requested_active_lower_conduction_w_m2 +=
            requested_conduction_w_m2 * weight;
        hourly.rejected_active_lower_conduction_w_m2 +=
            rejected_conduction_w_m2 * weight;
        if requested_conduction_w_m2.abs()
            <= hourly.peak_substep_requested_g0_w_m2.abs()
        {
            return;
        }
        hourly.peak_substep_applied_g0_w_m2 = conduction.flux;
        hourly.peak_substep_requested_g0_w_m2 =
            requested_conduction_w_m2;
        hourly.peak_substep_rejected_g0_w_m2 =
            rejected_conduction_w_m2;
        hourly.peak_substep_pressure_pa = substep.atmospheric_pressure_pa;
        hourly.peak_substep_active_temperature_c =
            Self::stage3_temperature_from_cold_content_values(
                active.mass_swe_m,
                active.cold_content_j_m2,
            );
        hourly.peak_substep_lower_temperature_c =
            Self::stage3_temperature_from_cold_content_values(
                lower.mass_swe_m,
                lower.cold_content_j_m2,
            );
        hourly.peak_substep_active_depth_m = active.depth_m;
        hourly.peak_substep_lower_depth_m = lower.depth_m;
        hourly.peak_substep_active_conductivity_w_m_k =
            active.conductivity_w_m_k;
        hourly.peak_substep_lower_conductivity_w_m_k =
            lower.conductivity_w_m_k;
        hourly.peak_substep_active_resistance_m2_k_w =
            active.depth_m / active.conductivity_w_m_k;
        hourly.peak_substep_lower_resistance_m2_k_w =
            lower.depth_m / lower.conductivity_w_m_k;
    }

    fn finish_stage3_hour_diagnostics(
        hourly: &mut DirectSnowSurfaceEnergyHourDiagnostics,
        latent_energy_j_m2: f64,
    ) {
        hourly.latent_flux_w_m2 = latent_energy_j_m2 / STAGE3_SECONDS_PER_HOUR;
        if hourly.vapor_mass_exchange_kg_m2.abs() > WB11_ZERO_THRESHOLD {
            hourly.latent_heat_j_kg =
                latent_energy_j_m2 / hourly.vapor_mass_exchange_kg_m2;
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

    fn stage3_temperature_from_cold_content(layer: &DirectSnowLayerState) -> f64 {
        Self::stage3_temperature_from_cold_content_values(layer.mass_swe_m, layer.cold_content_j_m2)
    }

    fn stage3_temperature_from_cold_content_values(mass_swe_m: f64, cold_content_j_m2: f64) -> f64 {
        if cold_content_j_m2 <= WB11_ZERO_THRESHOLD || mass_swe_m <= WB11_ZERO_THRESHOLD {
            0.0
        } else {
            -cold_content_j_m2
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

    // Finiteness + range guard with the symbol name built only on failure;
    // see require_state_range_with.
    pub(crate) fn require_direct_typed_snow_value_with(
        phase_class: HillslopeKernelPhaseClass,
        symbol: impl Fn() -> BoundarySymbol,
        value: f64,
        minimum: Option<f64>,
        maximum: Option<f64>,
    ) -> Result<(), Wb11HydrologyKernelGuardError> {
        if !value.is_finite() {
            return Err(Wb11HydrologyKernelGuardError::NonFiniteStateSymbol {
                phase_class,
                symbol: symbol(),
                value,
            });
        }
        Self::require_state_range_with(phase_class, symbol, value, minimum, maximum)
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
