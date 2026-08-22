#[allow(clippy::wildcard_imports)]
use super::super::*;
use openwepp_meteorology::surface_energy::{
    conductive_heat_flux, latent_heat_flux_from_mass_flux,
    latent_heat_for_surface_temperature, net_shortwave_radiation,
    precipitation_advected_heat_flux, saturation_vapor_pressure_snobal_pa,
    snow_effective_thermal_conductivity_snobal, snow_longwave_dilley_unsworth, specific_heat_ice,
    specific_heat_water, surface_energy_balance,
    turbulent_fluxes_monin_obukhov_with_diagnostics, EnergyFluxWattsPerSquareMeter,
    MassFluxKilogramsPerSquareMeterSecond, PositiveLengthMeters,
    PrecipitationAdvectedHeatInputs, PrecipitationMassFluxKilogramsPerSquareMeterSecond,
    PressurePascals, RadiativeFluxWattsPerSquareMeter, SnowLongwaveInputs,
    SurfaceEnergyBalanceTerms, ThermalConductivityWattsPerMeterKelvin, TurbulentFluxDiagnostics,
    TurbulentFluxInputs, TurbulentTransferOptions,
};
use openwepp_unit_boundary::{
    FractionUnitInterval, LinearRateMetersPerSecond, TemperatureCelsius,
};
use crate::snow_stage3_terminal_handoff::Stage3SnowSurfaceBoundaryReceiptV1;
use super::snow_mass_transition::{
    SNOW_SOLID_TO_LIQUID_CLOSURE_TOLERANCE_M, SNOW_STAGE3_LIQUID_CLOSURE_TOLERANCE_M,
};

mod stage3_solver;

const STAGE3_RHO_WATER_KG_M3: f64 = 1_000.0;
const STAGE3_LATENT_HEAT_FUSION_J_KG: f64 = 333_600.0;
const STAGE3_SPECIFIC_HEAT_ICE_J_KG_K: f64 = 2_100.0;
pub(crate) const STAGE3_DEFAULT_SNOW_ALBEDO: f64 = 0.82;
const STAGE3_SECONDS_PER_HOUR: f64 = 3_600.0;
const STAGE3_ACTIVE_LAYER_MAX_DEPTH_M: f64 = 0.25;
const STAGE3_NORMAL_TIMESTEP_MASS_KG_M2: f64 = 60.0;
const STAGE3_MEDIUM_TIMESTEP_MASS_KG_M2: f64 = 10.0;
const STAGE3_MINIMUM_RESOLVED_THERMAL_MASS_SWE_M: f64 = 0.001;
const STAGE3_MEDIUM_TIMESTEP_SECONDS: f64 = 900.0;
const STAGE3_SMALL_TIMESTEP_SECONDS: f64 = 60.0;
const STAGE3_ENERGY_CLOSURE_TOLERANCE_J_M2: f64 = 1.0e-6;
const STAGE3_BULK_EQUIVALENT_LAYER_CLOSURE_TOLERANCE_M: f64 = 1.0e-9;
const STAGE3_BULK_EQUIVALENT_MAX_LAYERS: usize = 16;

impl SnowStage3ConductivityError {
    /// Replay the exact rejected SNOBAL conductivity primitive.
    ///
    /// # Errors
    ///
    /// Returns the same typed meteorology error when the captured inputs remain
    /// outside the primitive's domain.
    pub fn replay(
        &self,
    ) -> Result<
        ThermalConductivityWattsPerMeterKelvin,
        openwepp_meteorology::MeteorologyError,
    > {
        let pressure = PressurePascals::try_new(self.atmospheric_pressure_pa)?;
        snow_effective_thermal_conductivity_snobal(
            self.layer.density_kg_m3,
            self.control_volume_temperature,
            pressure,
        )
    }
}

#[derive(Clone, Copy)]
struct Stage3AggregateState {
    swe_after_m: f64,
    depth_after_m: f64,
    density_after_kg_m3: f64,
    settle_day_count_after: f64,
}

#[derive(Clone)]
struct Stage3HourlySurfaceEnergy {
    total_j_m2: f64,
    shortwave_j_m2: f64,
    longwave_j_m2: f64,
    latent_j_m2: f64,
    vapor_mass_exchange_kg_m2: f64,
    latent_mass_energy_j_m2: f64,
    sublimation_m: f64,
    mass_latent_identity_residual_j_m2: f64,
    diagnostics: Option<DirectSnowSurfaceEnergyHourDiagnostics>,
    reconciliation: Option<Stage3CarrierReconciliation>,
}

#[derive(Clone, Copy)]
struct Stage3CarrierReconciliation {
    air_temperature_c: f64,
    dewpoint_c: f64,
    wind_speed_m_s: f64,
    air_pressure_pa: f64,
    hourly_radiation_mj_m2: f64,
    daily_solar_radiation_mj_m2: f64,
    daily_extraterrestrial_radiation_mj_m2: f64,
    daylight: bool,
    canopy_cover_fraction: f64,
    rain_m: f64,
    snowfall_geometric_m: f64,
    rain_mass_flux_kg_m2_s: f64,
    snow_mass_flux_kg_m2_s: f64,
    rain_temperature_c: f64,
    snow_temperature_c: f64,
    rain_specific_heat_j_kg_k: f64,
    snow_specific_heat_j_kg_k: f64,
    incoming_shortwave_w_m2: f64,
    snow_albedo_fraction: f64,
    snow_albedo_source_id: &'static str,
    snow_albedo_model_id: Option<&'static str>,
    snow_albedo_accumulated_positive_temperature_c_day: Option<f64>,
    net_shortwave_w_m2: f64,
    actual_vapor_pressure_pa: f64,
    longwave_cloud_fraction: f64,
    sky_view_fraction: f64,
    atmospheric_longwave_w_m2: f64,
    canopy_longwave_w_m2: f64,
    subcanopy_longwave_w_m2: f64,
    outgoing_longwave_w_m2: f64,
    net_longwave_w_m2: f64,
    longwave_model_id: &'static str,
    sublimation_model_id: &'static str,
    air_temperature_height_m: f64,
    vapor_pressure_height_m: f64,
    wind_speed_height_m: f64,
    aerodynamic_roughness_length_m: f64,
    turbulent_options: TurbulentTransferOptions,
    surface_vapor_pressure_pa: f64,
    surface_latent_heat_j_kg: Option<f64>,
    turbulent: Option<TurbulentFluxDiagnostics>,
    vapor_mass_flux_kg_m2_s: f64,
    sensible_flux_w_m2: f64,
    latent_flux_w_m2: f64,
    precipitation_advected_flux_w_m2: f64,
    complete_external_flux_w_m2: f64,
}

#[derive(Clone, Copy)]
struct Stage3ReconciliationState {
    active_layer_count: usize,
    total_layer_count: usize,
    active_fingerprint: u64,
    total_fingerprint: u64,
    effective_input_fingerprint: u64,
    active_ice_mass_kg_m2: f64,
    total_ice_mass_kg_m2: f64,
    total_retained_liquid_kg_m2: f64,
    active_depth_m: f64,
    active_density_kg_m3: f64,
    active_cold_j_m2: f64,
    total_cold_j_m2: f64,
    surface_temperature_c: f64,
}

#[allow(clippy::struct_field_names)]
#[derive(Clone, Copy)]
struct Stage3ReconciliationTransfer {
    active_cold_energy_change_j_m2: Option<f64>,
    lower_cold_energy_change_j_m2: Option<f64>,
    cold_content_export_j_m2: Option<f64>,
    internal_active_lower_conduction_j_m2: Option<f64>,
    melt_kg_m2: Option<f64>,
    sublimation_kg_m2: Option<f64>,
    deposition_kg_m2: Option<f64>,
    legacy_sequential_complete_j_m2: Option<f64>,
    energy_closure_residual_j_m2: Option<f64>,
}

impl Stage3ReconciliationTransfer {
    const SAME_STATE: Self = Self {
        active_cold_energy_change_j_m2: None,
        lower_cold_energy_change_j_m2: None,
        cold_content_export_j_m2: None,
        internal_active_lower_conduction_j_m2: None,
        melt_kg_m2: None,
        sublimation_kg_m2: None,
        deposition_kg_m2: None,
        legacy_sequential_complete_j_m2: None,
        energy_closure_residual_j_m2: None,
    };
}

#[derive(Clone, Copy)]
struct Stage3SurfaceInterval {
    surface_temperature_c: f64,
    snow_depth_m: f64,
    snow_density_kg_m3: f64,
    duration_seconds: f64,
    forcing_duration_seconds: f64,
    boundary: Option<Stage3SnowSurfaceBoundaryReceiptV1>,
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

#[derive(Clone, Copy, PartialEq, Eq)]
struct Stage3EvaluationTag {
    operator: SnowStage3EvaluationOperator,
    source_snapshot_id: &'static str,
    support_id: &'static str,
    cadence_id: &'static str,
    carrier_id: &'static str,
    coverage_id: &'static str,
    claim_class: &'static str,
    unresolved_boundaries_id: &'static str,
    pairing_id: Option<&'static str>,
    arm_ids: [&'static str; 2],
    arm_count: u8,
}

impl Stage3EvaluationTag {
    const fn new(operator: SnowStage3EvaluationOperator) -> Self {
        let paired = matches!(
            operator,
            SnowStage3EvaluationOperator::SameStatePairedCarrierV1
        );
        let persistent = matches!(
            operator,
            SnowStage3EvaluationOperator::PersistentAccumulationShadowV1
        );
        Self {
            operator,
            source_snapshot_id: if persistent {
                "pre_interval_authoritative_initial_snapshot_v1"
            } else {
                "post_coe_daily_initial_snapshot_v1"
            },
            support_id: if persistent {
                "stage3_persistent_daily_24_hour_support_v1"
            } else {
                "stage3_daily_24_hour_support_v1"
            },
            cadence_id: if paired {
                "stage3_fixed_hourly_immutable_snapshot_v1"
            } else {
                "stage3_dynamic_substep_with_hourly_forcing_v1"
            },
            carrier_id: if paired {
                "stage3_carrier_pair_v1"
            } else {
                "stage3_complete_carrier_v1"
            },
            coverage_id: "evaluated_seconds_over_requested_seconds_v1",
            claim_class: operator.claim_class(),
            unresolved_boundaries_id:
                "snow_ground_cross_day_terminal_recipient_unresolved_v1",
            pairing_id: if paired {
                Some("stage3_carrier_pair_v1")
            } else {
                None
            },
            arm_ids: if paired {
                ["stage3_surface_energy_v1", "stage3_complete_carrier_v1"]
            } else {
                ["stage3_complete_carrier_v1", "not_applicable"]
            },
            arm_count: if paired { 2 } else { 1 },
        }
    }
}

#[derive(Clone)]
struct Stage3ShadowSummary {
    tag: Stage3EvaluationTag,
    source_fingerprint: u64,
    forcing_fingerprint: u64,
    geometry_fingerprint: u64,
    non_formulation_fingerprint: u64,
    surface_arm_non_formulation_fingerprint: u64,
    complete_arm_non_formulation_fingerprint: u64,
    requested_seconds: f64,
    evaluated_seconds: f64,
    surface_arm_shortwave_j_m2: f64,
    surface_arm_longwave_j_m2: f64,
    surface_arm_latent_j_m2: f64,
    surface_arm_total_j_m2: f64,
    complete_shortwave_j_m2: f64,
    complete_longwave_j_m2: f64,
    complete_sensible_j_m2: f64,
    complete_latent_j_m2: f64,
    complete_advected_j_m2: f64,
    internal_active_lower_conduction_j_m2: f64,
    complete_vapor_mass_exchange_kg_m2: f64,
    cold_content_export_j_m2: f64,
    available_ice_kg_m2: f64,
    complete_energy_j_m2: f64,
    cold_energy_change_j_m2: f64,
    excess_energy_j_m2: f64,
    sublimation_kg_m2: f64,
    melt_kg_m2: f64,
    unallocated_after_exhaustion_j_m2: f64,
    maximum_energy_closure_residual_j_m2: f64,
    hourly: [DirectSnowStage3EvaluationHourDiagnostics; 24],
    reconciliation: DirectSnowStage3OperatorReconciliation,
    final_layers: Vec<DirectSnowLayerState>,
    terminal_event: Option<DirectSnowTerminalEventResult>,
    terminal_intervals: Vec<DirectSnowTerminalEventResult>,
    terminal_refrozen_kg_m2: f64,
    terminal_deposition_kg_m2: f64,
}

impl Stage3ShadowSummary {
    const fn new(tag: Stage3EvaluationTag) -> Self {
        Self {
        tag,
        source_fingerprint: 0,
        forcing_fingerprint: 0,
        geometry_fingerprint: 0,
        non_formulation_fingerprint: 0,
        surface_arm_non_formulation_fingerprint: 0,
        complete_arm_non_formulation_fingerprint: 0,
        requested_seconds: 24.0 * STAGE3_SECONDS_PER_HOUR,
        evaluated_seconds: 0.0,
        surface_arm_shortwave_j_m2: 0.0,
        surface_arm_longwave_j_m2: 0.0,
        surface_arm_latent_j_m2: 0.0,
        surface_arm_total_j_m2: 0.0,
        complete_shortwave_j_m2: 0.0,
        complete_longwave_j_m2: 0.0,
        complete_sensible_j_m2: 0.0,
        complete_latent_j_m2: 0.0,
        complete_advected_j_m2: 0.0,
        internal_active_lower_conduction_j_m2: 0.0,
        complete_vapor_mass_exchange_kg_m2: 0.0,
        cold_content_export_j_m2: 0.0,
        available_ice_kg_m2: 0.0,
        complete_energy_j_m2: 0.0,
        cold_energy_change_j_m2: 0.0,
        excess_energy_j_m2: 0.0,
        sublimation_kg_m2: 0.0,
        melt_kg_m2: 0.0,
        unallocated_after_exhaustion_j_m2: 0.0,
        maximum_energy_closure_residual_j_m2: 0.0,
        hourly: [DirectSnowStage3EvaluationHourDiagnostics::zero(); 24],
        reconciliation: DirectSnowStage3OperatorReconciliation {
            schema_version: 6,
            hourly_status: [DirectSnowStage3ReconciliationHourStatus::not_selected(); 24],
            tuples: Vec::new(),
        },
        final_layers: Vec::new(),
        terminal_event: None,
        terminal_intervals: Vec::new(),
        terminal_refrozen_kg_m2: 0.0,
        terminal_deposition_kg_m2: 0.0,
        }
    }
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
    /// Attaches an evaluation-only Stage 3 record to an authoritative inactive
    /// snow partition without requesting forcing or advancing snow state.
    ///
    /// The returned schema-v6 record declares the full daily support as
    /// requested but unevaluated. Its empty tuple inventory and
    /// `operator_not_selected` hourly statuses make the inactive lifecycle
    /// explicit while preserving the authoritative partition byte-for-byte.
    #[must_use]
    pub fn attach_inactive_stage3_evaluation(
        authoritative: DirectSnowLiquidPartition,
        operator: SnowStage3EvaluationOperator,
    ) -> DirectSnowStage3EvaluationWithReconciliationResult {
        let mut summary = Stage3ShadowSummary::new(Stage3EvaluationTag::new(operator));
        for hourly in &mut summary.hourly {
            hourly.requested_seconds = STAGE3_SECONDS_PER_HOUR;
        }
        let evaluation = Self::stage3_evaluation_diagnostics(&summary);
        DirectSnowStage3EvaluationWithReconciliationResult {
            result: DirectSnowStage3EvaluationResult {
                authoritative,
                evaluation: Some(evaluation),
            },
            reconciliation: Some(Box::new(summary.reconciliation)),
        }
    }

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
    pub fn compute_direct_snow_liquid_partition_from_typed(
        inputs: &DirectActiveSnowPartitionInputs,
    ) -> Result<DirectSnowLiquidPartition, Wb11HydrologyKernelGuardError> {
        Self::compute_direct_snow_liquid_partition_with_capture(
            inputs,
            DirectSnowDiagnosticCapture::Verbose,
        )
    }

    /// Computes the authoritative snow mass transition and optionally retains
    /// the allocation-heavy diagnostics needed by the selected trace consumer.
    #[allow(clippy::too_many_lines)]
    pub fn compute_direct_snow_liquid_partition_with_capture(
        inputs: &DirectActiveSnowPartitionInputs,
        capture: DirectSnowDiagnosticCapture,
    ) -> Result<DirectSnowLiquidPartition, Wb11HydrologyKernelGuardError> {
        match Self::compute_direct_snow_liquid_partition_with_capture_and_evaluation(
            inputs, capture, None,
        ) {
            Ok(result) => Ok(result.authoritative),
            Err(DirectSnowStage3EvaluationError::Kernel(source)) => Err(*source),
            Err(DirectSnowStage3EvaluationError::TurbulentTransfer(snapshot)) => Err(
                Wb11HydrologyKernelGuardError::StateSymbolOutOfRange {
                    phase_class: snapshot.phase_class,
                    symbol: BoundarySymbol::from("snow.stage3_shadow_turbulent_flux"),
                    value: snapshot.wind_speed_m_s,
                    minimum: Some(0.0),
                    maximum: None,
                },
            ),
            Err(DirectSnowStage3EvaluationError::TerminalNumerics(_)) => Err(
                Wb11HydrologyKernelGuardError::MissingRequiredStateSymbol {
                    phase_class: HillslopeKernelPhaseClass::HydrologyRunoffReconciliation,
                    symbol: BoundarySymbol::from("snow.unreachable_terminal_numerics"),
                },
            ),
        }
    }

    /// Computes a verbose authoritative result plus one bounded evaluation
    /// operator without changing the legacy options record or default entry point.
    pub fn compute_direct_snow_liquid_partition_with_evaluation(
        inputs: &DirectActiveSnowPartitionInputs,
        operator: SnowStage3EvaluationOperator,
    ) -> Result<DirectSnowStage3EvaluationResult, DirectSnowStage3EvaluationError> {
        Self::compute_direct_snow_liquid_partition_with_capture_and_evaluation(
            inputs,
            DirectSnowDiagnosticCapture::Verbose,
            Some(operator),
        )
    }

    /// Computes the authoritative result with an additive, evaluator-only
    /// request used by the selected internal trace consumer.
    #[allow(clippy::too_many_lines)]
    pub fn compute_direct_snow_liquid_partition_with_capture_and_evaluation(
        inputs: &DirectActiveSnowPartitionInputs,
        capture: DirectSnowDiagnosticCapture,
        evaluation_operator: Option<SnowStage3EvaluationOperator>,
    ) -> Result<DirectSnowStage3EvaluationResult, DirectSnowStage3EvaluationError> {
        Self::compute_direct_snow_liquid_partition_with_capture_and_reconciliation(
            inputs,
            capture,
            evaluation_operator,
        )
        .map(|result| result.result)
    }

    /// Computes the protected evaluation result plus its enabled-only schema-v6
    /// reconciliation companion for the internal trace consumer.
    #[allow(clippy::too_many_lines)]
    pub fn compute_direct_snow_liquid_partition_with_capture_and_reconciliation(
        inputs: &DirectActiveSnowPartitionInputs,
        capture: DirectSnowDiagnosticCapture,
        evaluation_operator: Option<SnowStage3EvaluationOperator>,
    ) -> Result<DirectSnowStage3EvaluationWithReconciliationResult, DirectSnowStage3EvaluationError>
    {
        let phase_class = HillslopeKernelPhaseClass::HydrologyRunoffReconciliation;
        Self::require_direct_typed_snow_value_with(
            phase_class,
            || BoundarySymbol::from(WB12_SYMBOL_RAINFALL_INPUT),
            inputs.hyetograph_rainfall_m,
            Some(0.0),
            None,
        )?;
        let typed_hourly_snowfall_present = inputs
            .hourly
            .iter()
            .any(|hour| hour.snowfall_m > WB11_ZERO_THRESHOLD);
        let active_snow_coupling = inputs.runtime_swe_m > WB11_ZERO_THRESHOLD
            || typed_hourly_snowfall_present
            || (inputs.hyetograph_rainfall_m > WB11_ZERO_THRESHOLD
                && f64::midpoint(inputs.tmax_c, inputs.tmin_c) < 0.0);
        let snow_coupling = if active_snow_coupling {
            Self::compute_active_snow_coupling_from_typed(phase_class, inputs, capture)?
        } else {
            Self::inactive_snow_coupling_from_typed(phase_class, inputs, capture)?
        };
        let (routed_melt_m, post_winter_rain_m) =
            Self::resolve_snow_partition_terms(phase_class, inputs.hyetograph_rainfall_m, &snow_coupling)?;
        let density_outcome = Self::resolve_typed_snow_density_outcome(
            phase_class,
            inputs,
            &snow_coupling,
        )?;
        let mut density_process_diagnostics = density_outcome.density_process_diagnostics;
        let mut snow_layers_after = density_outcome.layers_after;
        let stage3_resolution = Self::resolve_stage3_liquid_routing(
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
            capture,
            evaluation_operator,
        )?;
        snow_layers_after
            .retain(|layer| snow_density_layer_has_resolved_mass(layer.mass_swe_m));
        let runtime_swe_after_m =
            (density_outcome.runtime_swe_after_m - stage3_resolution.outcome.sublimation_m).max(0.0);
        let runtime_depth_after_m = if stage3_resolution.outcome.enabled {
            snow_layers_after
                .iter()
                .map(|layer| layer.thickness_m)
                .sum::<f64>()
        } else {
            density_outcome.runtime_depth_after_m
        };
        let runtime_density_after_kg_m3 = if runtime_swe_after_m <= WB11_ZERO_THRESHOLD {
            0.0
        } else if stage3_resolution.outcome.sublimation_m > 0.0 {
            (runtime_swe_after_m * STAGE3_RHO_WATER_KG_M3 / runtime_depth_after_m)
                .min(SIMIMPL29_SNOW_DENSITY_CAP_KG_M3)
        } else {
            density_outcome.runtime_density_after_kg_m3
        };
        let (coe_boundary_depth_after_m, coe_boundary_density_after_kg_m3) =
            if stage3_resolution.outcome.sublimation_m > 0.0 {
                (runtime_depth_after_m, runtime_density_after_kg_m3)
            } else {
                (
                    density_outcome.coe_boundary_depth_after_m,
                    density_outcome.coe_boundary_density_after_kg_m3,
                )
            };
        density_process_diagnostics
            .apply_downstream_stage3_density(runtime_density_after_kg_m3)
            .map_err(|error| {
                Self::snow_density_guard_error(
                    phase_class,
                    &error,
                    inputs.runtime_swe_m,
                    inputs.runtime_depth_m,
                    &inputs.snow_layers,
                )
            })?;
        let accumulation_melt_diagnostics = snow_coupling.verbose_diagnostics.as_deref().map(|verbose| DirectSnowAccumulationMeltDiagnostics {
            wind_m_s: inputs.wind_m_s,
            dewpoint_c: inputs.dewpoint_c,
            canopy_cover_fraction: inputs.canopy_cover_fraction,
            hourly_active_precipitation_m: std::array::from_fn(|index| {
                inputs.hourly[index].active_precipitation_m
            }),
            hourly_rain_m: std::array::from_fn(|index| inputs.hourly[index].rain_m),
            hourly_snowfall_depth_m: std::array::from_fn(|index| {
                inputs.hourly[index].snowfall_m
            }),
            hourly_snowfall_swe_m: std::array::from_fn(|index| {
                inputs.hourly[index].snowfall_m * 0.1
            }),
            hourly_air_temperature_c: std::array::from_fn(|index| {
                inputs.hourly[index].air_temperature_c
            }),
            hourly_radiation_mj_m2: std::array::from_fn(|index| {
                inputs.hourly[index].radiation_mj_m2
            }),
            hourly_cloud_fraction: std::array::from_fn(|index| {
                inputs.hourly[index].cloud_fraction
            }),
            hourly_rain_fraction: std::array::from_fn(|index| {
                inputs.hourly[index].rain_fraction
            }),
            hourly_snow_fraction: std::array::from_fn(|index| {
                inputs.hourly[index].snow_fraction
            }),
            hourly_phase_model: std::array::from_fn(|index| {
                inputs.hourly[index].phase_model
            }),
            hourly_hydrometeor_temperature_c: std::array::from_fn(|index| {
                inputs.hourly[index].hydrometeor_temperature_c
            }),
            hourly_melt: verbose.hourly_melt,
            hourly_routed_melt_m: snow_coupling.hourly_routed_melt,
            hourly_liquid_holding_capacity_m: verbose.hourly_trace.liquid_holding_capacity,
            hourly_liquid_water_retained_before_m: verbose.hourly_trace.liquid_water_retained_before,
            hourly_liquid_water_retained_after_m: verbose.hourly_trace.liquid_water_retained_after,
            hourly_liquid_water_released_m: verbose.hourly_trace.liquid_water_released,
            hourly_rain_released_m: verbose.hourly_trace.rain_released,
            hourly_sublimation_m: verbose.hourly_trace.sublimation,
            hourly_pack_depth_before_m: verbose.hourly_trace.pack_depth_before,
            hourly_pack_depth_after_m: verbose.hourly_trace.pack_depth_after,
            hourly_pack_density_before_kg_m3: verbose.hourly_trace.pack_density_before,
            hourly_pack_density_after_kg_m3: verbose.hourly_trace.pack_density_after,
            modeled_wind_redistribution_m: [0.0; 24],
        });
        let verbose_diagnostics = match (
            accumulation_melt_diagnostics,
            stage3_resolution.diagnostics,
        ) {
            (Some(accumulation_melt), Some(stage3)) => Some(Box::new(
                DirectSnowVerboseDiagnostics { accumulation_melt, stage3 },
            )),
            (None, None) => None,
            _ => {
                return Err(Wb11HydrologyKernelGuardError::StateSymbolOutOfRange {
                    phase_class,
                    symbol: BoundarySymbol::from("snow.verbose_diagnostic_capture_mismatch"),
                    value: 1.0,
                    minimum: Some(0.0),
                    maximum: Some(0.0),
                }
                .into());
            }
        };

        let solid_to_liquid_ledger = DirectSnowSolidToLiquidLedger {
            raw_signed_melt_m: snow_coupling.raw_melt,
            redistributed_positive_melt_m: snow_coupling.redistributed_melt,
            snowpack_swe_loss_m: snow_coupling.snowpack_state_loss,
            rain_released_m: snow_coupling.rain_released,
            liquid_handoff_m: routed_melt_m,
        };

        let mass_transition_ledgers = DirectSnowMassTransitionLedgers::from_authoritative_parts(
            solid_to_liquid_ledger,
            stage3_resolution.liquid_disposition_ledger,
            stage3_resolution.outcome,
        );
        mass_transition_ledgers.validate().map_err(|source| {
            Wb11HydrologyKernelGuardError::SnowMassTransitionLedger {
                phase_class,
                source,
            }
        })?;
        let partition = DirectSnowLiquidPartition {
            active_snow_coupling,
            snow_density_model: inputs.snow_density_model,
            snow_coupling_signed_s_m: snow_coupling.signed_s,
            mass_transition_ledgers,
            hourly_routed_melt_m: snow_coupling.hourly_routed_melt,
            accumulation_m: snow_coupling.accumulation,
            rain_retained_m: snow_coupling.rain_retained,
            liquid_holding_capacity_after_m: snow_coupling.liquid_holding_capacity,
            liquid_water_retained_after_m: snow_coupling.liquid_water_retained,
            liquid_water_released_m: snow_coupling.liquid_water_released,
            sublimation_m: snow_coupling.sublimation + stage3_resolution.outcome.sublimation_m,
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
            density_process_diagnostics,
            verbose_diagnostics,
            snow_albedo_state_after: snow_coupling.snow_albedo_state_after,
            snow_layers_after,
        };
        Self::validate_direct_snow_storage_closure(phase_class, inputs, &partition)?;
        Ok(DirectSnowStage3EvaluationWithReconciliationResult {
            result: DirectSnowStage3EvaluationResult {
                authoritative: partition,
                evaluation: stage3_resolution.evaluation,
            },
            reconciliation: stage3_resolution.reconciliation,
        })
    }

    fn validate_direct_snow_storage_closure(
        phase_class: HillslopeKernelPhaseClass,
        inputs: &DirectActiveSnowPartitionInputs,
        partition: &DirectSnowLiquidPartition,
    ) -> Result<(), Wb11HydrologyKernelGuardError> {
        let typed_snowfall_swe_m = inputs
            .hourly
            .iter()
            .map(|hour| hour.snowfall_m * 0.1)
            .sum::<f64>();
        let residual_m = inputs.runtime_swe_m
            + typed_snowfall_swe_m
            + partition.rain_retained_m
            - partition
                .mass_transition_ledgers
                .solid_to_liquid()
                .snowpack_swe_loss_m
            - partition.sublimation_m
            - partition.runtime_swe_after_m;
        Self::validate_direct_snow_storage_residual(phase_class, residual_m)
    }

    fn validate_direct_snow_storage_residual(
        phase_class: HillslopeKernelPhaseClass,
        residual_m: f64,
    ) -> Result<(), Wb11HydrologyKernelGuardError> {
        let symbol = || BoundarySymbol::from("snow.daily_storage_closure_residual_m");
        if !residual_m.is_finite() {
            return Err(Wb11HydrologyKernelGuardError::NonFiniteStateSymbol {
                phase_class,
                symbol: symbol(),
                value: residual_m,
            });
        }
        if residual_m.abs() > SNOW_SOLID_TO_LIQUID_CLOSURE_TOLERANCE_M {
            return Err(Wb11HydrologyKernelGuardError::StateSymbolOutOfRange {
                phase_class,
                symbol: symbol(),
                value: residual_m,
                minimum: Some(-SNOW_SOLID_TO_LIQUID_CLOSURE_TOLERANCE_M),
                maximum: Some(SNOW_SOLID_TO_LIQUID_CLOSURE_TOLERANCE_M),
            });
        }
        Ok(())
    }

    fn inactive_snow_coupling_from_typed(
        phase_class: HillslopeKernelPhaseClass,
        inputs: &DirectActiveSnowPartitionInputs,
        capture: DirectSnowDiagnosticCapture,
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
            wet_compaction_liquid_input_m: 0.0,
            hourly_routed_melt: [0.0; 24],
            verbose_diagnostics: capture.is_verbose().then(|| Box::new(
                SnowCouplingVerboseDiagnostics {
                    hourly_melt: [DirectSnowMeltHourDiagnostics::default(); 24],
                    hourly_trace: SnowHourlyTrace::default(),
                },
            )),
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
    fn resolve_typed_snow_density_outcome(
        phase_class: HillslopeKernelPhaseClass,
        inputs: &DirectActiveSnowPartitionInputs,
        snow_coupling: &SnowCouplingOutcome,
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
            liquid_for_compaction_m: snow_coupling.wet_compaction_liquid_input_m,
            mean_air_temperature_c,
            runtime_density_cap_kg_m3: SIMIMPL29_SNOW_DENSITY_CAP_KG_M3,
            sturm_climate_class: inputs.sturm_climate_class,
            sturm_day_of_year: inputs.sturm_day_of_year,
        })
        .map_err(|error| {
            Self::snow_density_guard_error(
                phase_class,
                &error,
                inputs.runtime_swe_m,
                inputs.runtime_depth_m,
                &inputs.snow_layers,
            )
        })
    }

    fn snow_density_guard_error(
        phase_class: HillslopeKernelPhaseClass,
        error: &SnowDensityError,
        prior_swe_m: f64,
        prior_depth_m: f64,
        prior_layers: &[DirectSnowLayerState],
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
            } => Wb11HydrologyKernelGuardError::SnowLayerAggregateMismatch(Box::new(
                SnowLayerAggregateMismatchError {
                    phase_class,
                    symbol,
                    value: *value,
                    expected: *expected,
                    prior_swe_m,
                    prior_depth_m,
                    prior_layers: prior_layers.to_vec(),
                },
            )),
            SnowDensityError::DiagnosticClosureViolation {
                residual_kg_m3,
                tolerance_kg_m3,
            } => Wb11HydrologyKernelGuardError::StateSymbolOutOfRange {
                phase_class,
                symbol: BoundarySymbol::from(
                    "snow_density_process_closure_residual_kg_m3",
                ),
                value: *residual_kg_m3,
                minimum: Some(-*tolerance_kg_m3),
                maximum: Some(*tolerance_kg_m3),
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
    fn eb04w2b_storage_guard_enforces_exact_tolerance_and_nonfinite_rejection() {
        let phase_class = HillslopeKernelPhaseClass::HydrologyRunoffReconciliation;
        for residual_m in [
            -SNOW_SOLID_TO_LIQUID_CLOSURE_TOLERANCE_M,
            0.0,
            SNOW_SOLID_TO_LIQUID_CLOSURE_TOLERANCE_M,
        ] {
            Wb11HydrologyKernel::validate_direct_snow_storage_residual(
                phase_class,
                residual_m,
            )
            .expect("exact-tolerance daily snow closure residual must be accepted");
        }

        for residual_m in [
            f64::from_bits(SNOW_SOLID_TO_LIQUID_CLOSURE_TOLERANCE_M.to_bits() + 1),
            -f64::from_bits(SNOW_SOLID_TO_LIQUID_CLOSURE_TOLERANCE_M.to_bits() + 1),
        ] {
            let error = Wb11HydrologyKernel::validate_direct_snow_storage_residual(
                phase_class,
                residual_m,
            )
            .expect_err("over-tolerance daily snow closure residual must fail closed");
            assert!(matches!(
                error,
                Wb11HydrologyKernelGuardError::StateSymbolOutOfRange { .. }
            ));
            assert_eq!(error.code(), "HKERNEL-WB14-RUNOFF-E-003");
            assert!(error
                .to_string()
                .contains("snow.daily_storage_closure_residual_m"));
        }

        let error = Wb11HydrologyKernel::validate_direct_snow_storage_residual(
            phase_class,
            f64::NAN,
        )
        .expect_err("non-finite daily snow closure residual must fail closed");
        assert!(matches!(
            error,
            Wb11HydrologyKernelGuardError::NonFiniteStateSymbol { .. }
        ));
        assert_eq!(error.code(), "HKERNEL-WB14-RUNOFF-E-002");
        assert!(error
            .to_string()
            .contains("snow.daily_storage_closure_residual_m"));
    }

    #[test]
    fn eb04c_lower_volume_threshold_is_strict_on_native_swe() {
        let threshold = STAGE3_MINIMUM_RESOLVED_THERMAL_MASS_SWE_M;
        let just_below = f64::from_bits(threshold.to_bits() - 1);
        let just_above = f64::from_bits(threshold.to_bits() + 1);

        assert!(Wb11HydrologyKernel::stage3_lower_volume_is_subresolution_swe_m(
            just_below
        ));
        assert!(!Wb11HydrologyKernel::stage3_lower_volume_is_subresolution_swe_m(
            threshold
        ));
        assert!(!Wb11HydrologyKernel::stage3_lower_volume_is_subresolution_swe_m(
            just_above
        ));
    }

    #[test]
    fn partial_sublimation_retains_mass_resolved_subnanometer_swe_remainder() {
        let original_mass_swe_m = 1.0e-6;
        let represented_remainder_swe_m = 5.0e-10;
        let requested_m = original_mass_swe_m - represented_remainder_swe_m;
        let mut layer = DirectSnowLayerState::new(original_mass_swe_m, 2.0e-6, 500.0, 8.0);
        layer.liquid_water_m = 2.0e-7;
        layer.refrozen_liquid_m = 1.0e-7;
        let mut layers = vec![layer];
        let original_cold_content_j_m2 = 2.1;
        let mut cold_content_by_layer = vec![original_cold_content_j_m2];
        let mut active_layer_count = 1;

        let (removed_m, exported_j_m2, removed_layer_count) =
            Wb11HydrologyKernel::remove_stage3_active_sublimation(
                requested_m,
                &mut layers,
                &mut cold_content_by_layer,
                &mut active_layer_count,
            );

        assert_eq!(layers.len(), 1);
        assert_eq!(active_layer_count, 1);
        assert_eq!(removed_layer_count, 0);
        assert!(snow_density_layer_has_resolved_mass(layers[0].mass_swe_m));
        assert!((layers[0].mass_swe_m - represented_remainder_swe_m).abs() <= 1.0e-18);
        assert!((removed_m + layers[0].mass_swe_m - original_mass_swe_m).abs() <= 1.0e-18);
        assert!(
            (exported_j_m2 + cold_content_by_layer[0] - original_cold_content_j_m2).abs()
                <= 1.0e-12
        );
        assert!((layers[0].liquid_water_m - 1.0e-10).abs() <= 1.0e-18);
        assert!((layers[0].refrozen_liquid_m - 5.0e-11).abs() <= 1.0e-18);
    }

    #[test]
    fn stage3_target_trim_preserves_coupled_mass_resolved_remainder() {
        let original_mass_swe_m = 2.0e-6;
        let represented_remainder_swe_m = 5.0e-10;
        let removal_m = original_mass_swe_m - represented_remainder_swe_m;
        let mut surface = DirectSnowLayerState::new(original_mass_swe_m, 4.0e-6, 500.0, 9.0);
        surface.temperature_c = -4.0;
        surface.liquid_water_m = 4.0e-7;
        surface.cold_content_j_m2 = 16.8;
        surface.refrozen_liquid_m = 2.0e-7;
        let lower = DirectSnowLayerState::new(0.1, 0.2, 500.0, 20.0);
        let target_swe_m = surface.mass_swe_m + lower.mass_swe_m - removal_m;
        let mut layers = vec![surface, lower];

        Wb11HydrologyKernel::adjust_stage3_layer_swe_to_target(
            &mut layers,
            target_swe_m,
            0.2,
            500.0,
            20.0,
        );

        assert_eq!(layers.len(), 2);
        let retained = layers[0];
        let retained_fraction = retained.mass_swe_m / original_mass_swe_m;
        assert!((retained.mass_swe_m - represented_remainder_swe_m).abs() <= 1.0e-15);
        assert!(snow_density_layer_has_resolved_mass(retained.mass_swe_m));
        assert!((retained.liquid_water_m - surface.liquid_water_m * retained_fraction).abs() <= 1.0e-18);
        assert!((retained.refrozen_liquid_m - surface.refrozen_liquid_m * retained_fraction).abs() <= 1.0e-18);
        assert!((retained.cold_content_j_m2 - surface.cold_content_j_m2 * retained_fraction).abs() <= 1.0e-15);
        assert_eq!(retained.density_kg_m3.to_bits(), surface.density_kg_m3.to_bits());
        assert_eq!(retained.temperature_c.to_bits(), surface.temperature_c.to_bits());
        assert_eq!(retained.settle_day_count.to_bits(), surface.settle_day_count.to_bits());
        let reconstructed_swe_m = layers.iter().map(|layer| layer.mass_swe_m).sum::<f64>();
        assert!((reconstructed_swe_m - target_swe_m).abs() <= 1.0e-15);
    }

    #[test]
    fn stage3_target_trim_continues_below_residual_tolerance_across_layers() {
        let mut removed = DirectSnowLayerState::new(2.0e-6, 4.0e-6, 500.0, 9.0);
        removed.liquid_water_m = 4.0e-7;
        removed.cold_content_j_m2 = 16.8;
        removed.refrozen_liquid_m = 2.0e-7;
        let mut retained = DirectSnowLayerState::new(2.0e-9, 4.0e-9, 500.0, 12.0);
        retained.temperature_c = -3.0;
        retained.liquid_water_m = 8.0e-10;
        retained.cold_content_j_m2 = 4.2e-3;
        retained.refrozen_liquid_m = 4.0e-10;
        let target_swe_m = 1.5e-9;
        let mut layers = vec![removed, retained];

        Wb11HydrologyKernel::adjust_stage3_layer_swe_to_target(
            &mut layers,
            target_swe_m,
            3.0e-9,
            500.0,
            12.0,
        );

        assert_eq!(layers.len(), 1);
        let result = layers[0];
        let retained_fraction = 0.75;
        assert!((result.mass_swe_m - target_swe_m).abs() <= 1.0e-18);
        assert!(snow_density_layer_has_resolved_mass(result.mass_swe_m));
        assert!((result.liquid_water_m - retained.liquid_water_m * retained_fraction).abs() <= 1.0e-18);
        assert!((result.refrozen_liquid_m - retained.refrozen_liquid_m * retained_fraction).abs() <= 1.0e-18);
        assert!((result.cold_content_j_m2 - retained.cold_content_j_m2 * retained_fraction).abs() <= 1.0e-15);
        assert_eq!(result.density_kg_m3.to_bits(), retained.density_kg_m3.to_bits());
        assert_eq!(result.temperature_c.to_bits(), retained.temperature_c.to_bits());
        assert_eq!(result.settle_day_count.to_bits(), retained.settle_day_count.to_bits());
    }

    #[test]
    fn snow_density_guard_error_maps_all_error_variants() {
        let phase_class = HillslopeKernelPhaseClass::HydrologyRunoffReconciliation;
        let replay_layers = [DirectSnowLayerState::new(0.2, 0.4, 500.0, 2.0)];
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
                symbol: "prior_layers.thickness_m",
                value: 0.4,
                expected: 0.5,
            },
            SnowDensityError::DiagnosticClosureViolation {
                residual_kg_m3: 2.0e-9,
                tolerance_kg_m3: 1.0e-9,
            },
        ];

        let mapped = cases
            .iter()
            .map(|error| {
                Wb11HydrologyKernel::snow_density_guard_error(
                    phase_class,
                    error,
                    0.2,
                    0.5,
                    &replay_layers,
                )
            })
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
        assert!(matches!(
            mapped[5],
            Wb11HydrologyKernelGuardError::SnowLayerAggregateMismatch(_)
        ));
        assert!(matches!(
            mapped[6],
            Wb11HydrologyKernelGuardError::StateSymbolOutOfRange { .. }
        ));
        if let Wb11HydrologyKernelGuardError::SnowLayerAggregateMismatch(snapshot) = &mapped[5] {
            assert!((snapshot.replay_value() - snapshot.value).abs() <= f64::EPSILON);
            assert!((snapshot.replay_value() - snapshot.expected).abs() > f64::EPSILON);
            assert!((snapshot.expected - snapshot.prior_depth_m).abs() <= f64::EPSILON);
            let replay_swe_m = snapshot
                .prior_layers
                .iter()
                .map(|layer| layer.mass_swe_m)
                .sum::<f64>();
            assert!((replay_swe_m - snapshot.prior_swe_m).abs() <= f64::EPSILON);
        }
        assert!(mapped[5]
            .to_string()
            .contains("prior_layers.thickness_m=0.4 does not match expected 0.5"));
    }
}
