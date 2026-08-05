
use crate::winter_column::DirectSnowLayerState;
use crate::runtime_inputs::SnowPhasePartitionModel;

#[derive(Debug, Clone, Copy, Default, PartialEq)]
pub struct DirectSnowMeltHourDiagnostics {
    pub coe_melt_amelt_m: f64,
    pub coe_melt_bmelt_m: f64,
    pub coe_melt_cmelt_m: f64,
    pub coe_melt_dmelt_m: f64,
    pub coe_melt_uncapped_m: f64,
    pub coe_melt_cap_adjustment_m: f64,
    pub coe_melt_applied_m: f64,
}

#[derive(Debug, Clone, PartialEq)]
pub struct DirectSnowAccumulationMeltDiagnostics {
    pub wind_m_s: f64,
    pub dewpoint_c: f64,
    pub canopy_cover_fraction: f64,
    pub hourly_active_precipitation_m: [f64; 24],
    pub hourly_rain_m: [f64; 24],
    pub hourly_snowfall_depth_m: [f64; 24],
    pub hourly_snowfall_swe_m: [f64; 24],
    pub hourly_air_temperature_c: [f64; 24],
    pub hourly_radiation_mj_m2: [f64; 24],
    pub hourly_cloud_fraction: [f64; 24],
    pub hourly_rain_fraction: [f64; 24],
    pub hourly_snow_fraction: [f64; 24],
    pub hourly_phase_model: [SnowPhasePartitionModel; 24],
    pub hourly_hydrometeor_temperature_c: [Option<f64>; 24],
    pub hourly_melt: [DirectSnowMeltHourDiagnostics; 24],
    pub hourly_routed_melt_m: [f64; 24],
    pub hourly_liquid_holding_capacity_m: [f64; 24],
    pub hourly_liquid_water_retained_before_m: [f64; 24],
    pub hourly_liquid_water_retained_after_m: [f64; 24],
    pub hourly_liquid_water_released_m: [f64; 24],
    pub hourly_rain_released_m: [f64; 24],
    pub hourly_sublimation_m: [f64; 24],
    pub hourly_pack_depth_before_m: [f64; 24],
    pub hourly_pack_depth_after_m: [f64; 24],
    pub hourly_pack_density_before_kg_m3: [f64; 24],
    pub hourly_pack_density_after_kg_m3: [f64; 24],
    pub modeled_wind_redistribution_m: [f64; 24],
}

impl Default for DirectSnowAccumulationMeltDiagnostics {
    fn default() -> Self {
        Self {
            wind_m_s: 0.0,
            dewpoint_c: 0.0,
            canopy_cover_fraction: 0.0,
            hourly_active_precipitation_m: [0.0; 24],
            hourly_rain_m: [0.0; 24],
            hourly_snowfall_depth_m: [0.0; 24],
            hourly_snowfall_swe_m: [0.0; 24],
            hourly_air_temperature_c: [0.0; 24],
            hourly_radiation_mj_m2: [0.0; 24],
            hourly_cloud_fraction: [0.0; 24],
            hourly_rain_fraction: [0.0; 24],
            hourly_snow_fraction: [0.0; 24],
            hourly_phase_model: [SnowPhasePartitionModel::LegacyRst; 24],
            hourly_hydrometeor_temperature_c: [None; 24],
            hourly_melt: [DirectSnowMeltHourDiagnostics::default(); 24],
            hourly_routed_melt_m: [0.0; 24],
            hourly_liquid_holding_capacity_m: [0.0; 24],
            hourly_liquid_water_retained_before_m: [0.0; 24],
            hourly_liquid_water_retained_after_m: [0.0; 24],
            hourly_liquid_water_released_m: [0.0; 24],
            hourly_rain_released_m: [0.0; 24],
            hourly_sublimation_m: [0.0; 24],
            hourly_pack_depth_before_m: [0.0; 24],
            hourly_pack_depth_after_m: [0.0; 24],
            hourly_pack_density_before_kg_m3: [0.0; 24],
            hourly_pack_density_after_kg_m3: [0.0; 24],
            modeled_wind_redistribution_m: [0.0; 24],
        }
    }
}

/// WB11 hydrology production kernel for ET/perc/lateral/drain lanes.
#[derive(Debug, Clone, Default)]
pub struct Wb11HydrologyKernel;

#[derive(Debug, Clone, Copy)]
#[allow(clippy::struct_field_names)]
pub(crate) struct SnowHourlyState {
    rain_released_m: f64,
    liquid_holding_capacity_m: f64,
    liquid_water_retained_before_m: f64,
    liquid_water_retained_after_m: f64,
    liquid_water_released_m: f64,
    sublimation_m: f64,
    melt_raw_m: f64,
    melt_m: f64,
    melt_diagnostics: Option<DirectSnowMeltHourDiagnostics>,
    pack_depth_before_m: f64,
    pack_depth_after_m: f64,
    pack_density_before_kg_m3: f64,
    pack_density_after_kg_m3: f64,
}

#[derive(Debug, Clone, Copy)]
pub(crate) struct SnowMeltComputation {
    wmelt_m: f64,
    diagnostics: Option<DirectSnowMeltHourDiagnostics>,
}

#[cfg(test)]
mod tests {
    use super::*;

    fn snowsci_stage1_hourly_state(melt_m: f64) -> SnowHourlyState {
        SnowHourlyState {
            rain_released_m: 0.0,
            liquid_holding_capacity_m: 0.0,
            liquid_water_retained_before_m: 0.0,
            liquid_water_retained_after_m: 0.0,
            liquid_water_released_m: 0.0,
            sublimation_m: 0.0,
            melt_raw_m: melt_m,
            melt_m,
            melt_diagnostics: Some(DirectSnowMeltHourDiagnostics {
                coe_melt_applied_m: melt_m,
                ..DirectSnowMeltHourDiagnostics::default()
            }),
            pack_depth_before_m: 0.0,
            pack_depth_after_m: 0.0,
            pack_density_before_kg_m3: 0.0,
            pack_density_after_kg_m3: 0.0,
        }
    }

    #[test]
    fn snowsci_stage1_mixed_signed_melt_routes_authoritative_pack_loss() {
        let positive_pack_loss_m = 0.007_376_104_224;
        let negative_raw_melt_m = -0.006_171_157_610;
        let mut hourly_state = [
            snowsci_stage1_hourly_state(positive_pack_loss_m),
            snowsci_stage1_hourly_state(negative_raw_melt_m),
        ];

        let redistribution =
            Wb11HydrologyKernel::redistribute_daily_signed_snowmelt(&mut hourly_state);
        let routed_hourly_sum_m = hourly_state
            .iter()
            .map(|hourly| hourly.melt_m)
            .sum::<f64>();

        assert!(
            (redistribution.routed_melt_total_m - positive_pack_loss_m).abs() <= 1.0e-12
        );
        assert!(
            (redistribution.snowpack_state_loss_m - positive_pack_loss_m).abs() <= 1.0e-12
        );
        assert!((routed_hourly_sum_m - positive_pack_loss_m).abs() <= 1.0e-12);
        assert!(hourly_state.iter().all(|hourly| hourly.melt_m >= 0.0));
    }

}

#[derive(Debug, Clone)]
pub(crate) struct SnowCouplingOutcome {
    signed_s: f64,
    accumulation: f64,
    rain_retained: f64,
    rain_released: f64,
    liquid_holding_capacity: f64,
    liquid_water_retained: f64,
    liquid_water_released: f64,
    sublimation: f64,
    raw_melt: f64,
    redistributed_melt: f64,
    wet_compaction_liquid_input_m: f64,
    hourly_routed_melt: [f64; 24],
    verbose_diagnostics: Option<Box<SnowCouplingVerboseDiagnostics>>,
    snowpack_state_loss: f64,
    runtime_swe: f64,
    runtime_depth_m: f64,
    runtime_density_kg_m3: f64,
    runtime_settle_day_count: f64,
    snow_albedo_state_after: Option<SnowAlbedoState>,
}

#[derive(Debug, Clone)]
pub(crate) struct SnowCouplingVerboseDiagnostics {
    hourly_melt: [DirectSnowMeltHourDiagnostics; 24],
    hourly_trace: SnowHourlyTrace,
}

#[derive(Debug, Clone, Copy, Default)]
pub(crate) struct SnowHourlyTrace {
    liquid_holding_capacity: [f64; 24],
    liquid_water_retained_before: [f64; 24],
    liquid_water_retained_after: [f64; 24],
    liquid_water_released: [f64; 24],
    rain_released: [f64; 24],
    sublimation: [f64; 24],
    pack_depth_before: [f64; 24],
    pack_depth_after: [f64; 24],
    pack_density_before: [f64; 24],
    pack_density_after: [f64; 24],
}

#[derive(Debug, Clone, PartialEq)]
pub struct DirectSnowLiquidPartition {
    pub active_snow_coupling: bool,
    pub snow_density_model: SnowDensityModel,
    pub snow_coupling_signed_s_m: f64,
    pub mass_transition_ledgers: DirectSnowMassTransitionLedgers,
    pub hourly_routed_melt_m: [f64; 24],
    pub accumulation_m: f64,
    pub rain_retained_m: f64,
    pub liquid_holding_capacity_after_m: f64,
    pub liquid_water_retained_after_m: f64,
    pub liquid_water_released_m: f64,
    pub sublimation_m: f64,
    pub post_winter_rain_m: f64,
    pub runtime_swe_after_m: f64,
    pub runtime_depth_after_m: f64,
    pub runtime_density_after_kg_m3: f64,
    pub runtime_settle_day_count_after: f64,
    pub coe_boundary_depth_after_m: f64,
    pub coe_boundary_density_after_kg_m3: f64,
    pub coe_boundary_settle_day_count_after: f64,
    pub density_swe_identity_residual_m: f64,
    pub density_unbounded_swe_residual_m: f64,
    pub density_process_diagnostics: SnowDensityProcessDiagnostics,
    pub verbose_diagnostics: Option<Box<DirectSnowVerboseDiagnostics>>,
    pub snow_albedo_state_after: Option<SnowAlbedoState>,
    pub snow_layers_after: Vec<DirectSnowLayerState>,
}

impl DirectSnowLiquidPartition {
    #[must_use]
    pub const fn solid_to_liquid_ledger(&self) -> DirectSnowSolidToLiquidLedger {
        self.mass_transition_ledgers.solid_to_liquid()
    }

    #[must_use]
    pub const fn liquid_disposition_ledger(&self) -> DirectSnowLiquidDispositionLedger {
        self.mass_transition_ledgers.liquid_disposition()
    }

    #[must_use]
    pub const fn stage3_outcome(&self) -> DirectSnowStage3Outcome {
        self.mass_transition_ledgers.stage3_outcome()
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SnowStage3LiquidRoutingModel {
    Disabled,
    LayeredThermalLiquidV1,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum SnowSurfaceLongwaveModel {
    #[default]
    Disabled,
    DilleyUnsworthSubcanopyV1,
}

impl SnowSurfaceLongwaveModel {
    #[must_use]
    pub const fn id(self) -> &'static str {
        match self {
            Self::Disabled => "disabled",
            Self::DilleyUnsworthSubcanopyV1 => "dilley_unsworth_subcanopy_v1",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum SnowSurfaceSublimationModel {
    #[default]
    Disabled,
    NeutralBulkStage3V1,
}

impl SnowSurfaceSublimationModel {
    #[must_use]
    pub const fn id(self) -> &'static str {
        match self {
            Self::Disabled => "disabled",
            Self::NeutralBulkStage3V1 => "neutral_bulk_stage3_v1",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct DirectSnowTurbulentGeometry {
    pub air_temperature_height_m: f64,
    pub vapor_pressure_height_m: f64,
    pub wind_speed_height_m: f64,
    pub aerodynamic_roughness_length_m: f64,
}

impl DirectSnowTurbulentGeometry {
    /// Contract-bound CLIGEN virtual instruments above the modeled snow surface.
    pub const CLIGEN_V1: Self = Self {
        air_temperature_height_m: 5.0,
        vapor_pressure_height_m: 5.0,
        wind_speed_height_m: 5.0,
        aerodynamic_roughness_length_m: 0.005,
    };
}

impl Default for DirectSnowTurbulentGeometry {
    fn default() -> Self {
        Self::CLIGEN_V1
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct DirectSnowSurfaceEnergyOptions {
    pub longwave_model: SnowSurfaceLongwaveModel,
    pub sublimation_model: SnowSurfaceSublimationModel,
    pub daily_solar_radiation_mj_m2: f64,
    pub daily_extraterrestrial_radiation_mj_m2: f64,
    pub daylight: bool,
    pub atmospheric_pressure_pa: f64,
    pub turbulent_geometry: DirectSnowTurbulentGeometry,
}

impl Default for DirectSnowSurfaceEnergyOptions {
    fn default() -> Self {
        Self {
            longwave_model: SnowSurfaceLongwaveModel::Disabled,
            sublimation_model: SnowSurfaceSublimationModel::Disabled,
            daily_solar_radiation_mj_m2: 0.0,
            daily_extraterrestrial_radiation_mj_m2: 0.0,
            daylight: false,
            atmospheric_pressure_pa: 101_324.6,
            turbulent_geometry: DirectSnowTurbulentGeometry::CLIGEN_V1,
        }
    }
}

impl SnowStage3LiquidRoutingModel {
    #[must_use]
    pub const fn id(self) -> &'static str {
        match self {
            Self::Disabled => "disabled",
            Self::LayeredThermalLiquidV1 => "layered_thermal_liquid_v1",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct DirectSnowSurfaceEnergyHourDiagnostics {
    pub surface_temperature_c: f64,
    pub canopy_temperature_equals_air: bool,
    pub atmospheric_longwave_w_m2: f64,
    pub canopy_longwave_w_m2: f64,
    pub sky_view_fraction: f64,
    pub subcanopy_longwave_w_m2: f64,
    pub outgoing_longwave_w_m2: f64,
    pub net_longwave_w_m2: f64,
    pub net_shortwave_w_m2: f64,
    pub vapor_mass_exchange_kg_m2: f64,
    pub latent_heat_j_kg: f64,
    pub latent_flux_w_m2: f64,
    pub potential_surface_energy_j_m2: f64,
    pub applied_surface_energy_j_m2: f64,
    pub unused_positive_energy_j_m2: f64,
    pub active_layer_mass_kg_m2: f64,
    pub active_layer_depth_m: f64,
    pub active_layer_temperature_c: f64,
    pub active_layer_cold_content_j_m2: f64,
    pub active_layer_effective_conductivity_w_m_k: f64,
    pub active_layer_thermal_resistance_m2_k_w: f64,
    pub lower_layer_present_fraction: f64,
    pub lower_layer_mass_kg_m2: f64,
    pub lower_layer_depth_m: f64,
    pub lower_layer_temperature_c: f64,
    pub lower_layer_cold_content_j_m2: f64,
    pub lower_layer_effective_conductivity_w_m_k: f64,
    pub lower_layer_thermal_resistance_m2_k_w: f64,
    pub atmospheric_pressure_pa: f64,
    pub active_lower_conduction_w_m2: f64,
    pub requested_active_lower_conduction_w_m2: f64,
    pub rejected_active_lower_conduction_w_m2: f64,
    pub peak_substep_applied_g0_w_m2: f64,
    pub peak_substep_requested_g0_w_m2: f64,
    pub peak_substep_rejected_g0_w_m2: f64,
    pub peak_substep_pressure_pa: f64,
    pub peak_substep_active_temperature_c: f64,
    pub peak_substep_lower_temperature_c: f64,
    pub peak_substep_active_depth_m: f64,
    pub peak_substep_lower_depth_m: f64,
    pub peak_substep_active_conductivity_w_m_k: f64,
    pub peak_substep_lower_conductivity_w_m_k: f64,
    pub peak_substep_active_resistance_m2_k_w: f64,
    pub peak_substep_lower_resistance_m2_k_w: f64,
    pub substep_count: u16,
    pub minimum_substep_seconds: f64,
    pub maximum_active_energy_closure_residual_j_m2: f64,
    pub maximum_lower_energy_closure_residual_j_m2: f64,
    pub maximum_conduction_cancellation_residual_j_m2: f64,
}

impl DirectSnowSurfaceEnergyHourDiagnostics {
    #[must_use]
    pub const fn zero() -> Self {
        Self {
            surface_temperature_c: 0.0,
            canopy_temperature_equals_air: true,
            atmospheric_longwave_w_m2: 0.0,
            canopy_longwave_w_m2: 0.0,
            sky_view_fraction: 0.0,
            subcanopy_longwave_w_m2: 0.0,
            outgoing_longwave_w_m2: 0.0,
            net_longwave_w_m2: 0.0,
            net_shortwave_w_m2: 0.0,
            vapor_mass_exchange_kg_m2: 0.0,
            latent_heat_j_kg: 0.0,
            latent_flux_w_m2: 0.0,
            potential_surface_energy_j_m2: 0.0,
            applied_surface_energy_j_m2: 0.0,
            unused_positive_energy_j_m2: 0.0,
            active_layer_mass_kg_m2: 0.0,
            active_layer_depth_m: 0.0,
            active_layer_temperature_c: 0.0,
            active_layer_cold_content_j_m2: 0.0,
            active_layer_effective_conductivity_w_m_k: 0.0,
            active_layer_thermal_resistance_m2_k_w: 0.0,
            lower_layer_present_fraction: 0.0,
            lower_layer_mass_kg_m2: 0.0,
            lower_layer_depth_m: 0.0,
            lower_layer_temperature_c: 0.0,
            lower_layer_cold_content_j_m2: 0.0,
            lower_layer_effective_conductivity_w_m_k: 0.0,
            lower_layer_thermal_resistance_m2_k_w: 0.0,
            atmospheric_pressure_pa: 0.0,
            active_lower_conduction_w_m2: 0.0,
            requested_active_lower_conduction_w_m2: 0.0,
            rejected_active_lower_conduction_w_m2: 0.0,
            peak_substep_applied_g0_w_m2: 0.0,
            peak_substep_requested_g0_w_m2: 0.0,
            peak_substep_rejected_g0_w_m2: 0.0,
            peak_substep_pressure_pa: 0.0,
            peak_substep_active_temperature_c: 0.0,
            peak_substep_lower_temperature_c: 0.0,
            peak_substep_active_depth_m: 0.0,
            peak_substep_lower_depth_m: 0.0,
            peak_substep_active_conductivity_w_m_k: 0.0,
            peak_substep_lower_conductivity_w_m_k: 0.0,
            peak_substep_active_resistance_m2_k_w: 0.0,
            peak_substep_lower_resistance_m2_k_w: 0.0,
            substep_count: 0,
            minimum_substep_seconds: 0.0,
            maximum_active_energy_closure_residual_j_m2: 0.0,
            maximum_lower_energy_closure_residual_j_m2: 0.0,
            maximum_conduction_cancellation_residual_j_m2: 0.0,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct DirectSnowStage3Diagnostics {
    pub cold_content_before_j_m2: f64,
    pub cold_content_after_j_m2: f64,
    pub surface_energy_j_m2: f64,
    pub conduction_energy_j_m2: f64,
    pub latent_refreeze_energy_j_m2: f64,
    pub energy_closure_residual_j_m2: f64,
    pub shortwave_energy_j_m2: f64,
    pub longwave_energy_j_m2: f64,
    pub latent_energy_j_m2: f64,
    pub vapor_mass_exchange_kg_m2: f64,
    pub latent_mass_energy_j_m2: f64,
    pub cold_content_export_j_m2: f64,
    pub mass_latent_identity_residual_j_m2: f64,
    pub unused_positive_energy_j_m2: f64,
    pub thermal_domain_suspended_seconds: f64,
    pub minimum_unresolved_thermal_mass_kg_m2: f64,
    pub lower_thermal_volume_collapsed_seconds: f64,
    pub minimum_collapsed_lower_mass_kg_m2: f64,
    pub hourly_surface_energy: [DirectSnowSurfaceEnergyHourDiagnostics; 24],
}

impl DirectSnowStage3Diagnostics {
    #[must_use]
    pub const fn disabled() -> Self {
        Self {
            cold_content_before_j_m2: 0.0,
            cold_content_after_j_m2: 0.0,
            surface_energy_j_m2: 0.0,
            conduction_energy_j_m2: 0.0,
            latent_refreeze_energy_j_m2: 0.0,
            energy_closure_residual_j_m2: 0.0,
            shortwave_energy_j_m2: 0.0,
            longwave_energy_j_m2: 0.0,
            latent_energy_j_m2: 0.0,
            vapor_mass_exchange_kg_m2: 0.0,
            latent_mass_energy_j_m2: 0.0,
            cold_content_export_j_m2: 0.0,
            mass_latent_identity_residual_j_m2: 0.0,
            unused_positive_energy_j_m2: 0.0,
            thermal_domain_suspended_seconds: 0.0,
            minimum_unresolved_thermal_mass_kg_m2: 0.0,
            lower_thermal_volume_collapsed_seconds: 0.0,
            minimum_collapsed_lower_mass_kg_m2: 0.0,
            hourly_surface_energy: [DirectSnowSurfaceEnergyHourDiagnostics::zero(); 24],
        }
    }

}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct DirectSnowHourlyForcing {
    pub active_precipitation_m: f64,
    pub rain_m: f64,
    pub snowfall_m: f64,
    pub radiation_mj_m2: f64,
    pub air_temperature_c: f64,
    pub cloud_fraction: f64,
    pub phase_model: SnowPhasePartitionModel,
    pub rain_fraction: f64,
    pub snow_fraction: f64,
    pub hydrometeor_temperature_c: Option<f64>,
}

impl DirectSnowHourlyForcing {
    #[must_use]
    pub const fn zero() -> Self {
        Self {
            active_precipitation_m: 0.0,
            rain_m: 0.0,
            snowfall_m: 0.0,
            radiation_mj_m2: 0.0,
            air_temperature_c: 0.0,
            cloud_fraction: 0.0,
            phase_model: SnowPhasePartitionModel::LegacyRst,
            rain_fraction: 0.0,
            snow_fraction: 0.0,
            hydrometeor_temperature_c: None,
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct DirectActiveSnowPartitionInputs {
    pub hyetograph_rainfall_m: f64,
    pub rst_c: f64,
    pub newsnw_kg_m3: f64,
    pub ssd_kg_m3: f64,
    pub runtime_swe_m: f64,
    pub runtime_depth_m: f64,
    pub runtime_density_kg_m3: f64,
    pub runtime_settle_day_count: f64,
    pub liquid_water_retained_m: f64,
    pub tmax_c: f64,
    pub tmin_c: f64,
    pub canopy_cover_fraction: f64,
    pub wind_m_s: f64,
    pub dewpoint_c: f64,
    pub snow_melt_model: SnowMeltModel,
    pub snow_density_model: SnowDensityModel,
    pub stage3_liquid_routing_model: SnowStage3LiquidRoutingModel,
    pub surface_energy_options: DirectSnowSurfaceEnergyOptions,
    pub sturm_climate_class: Option<SnowClimateClass>,
    pub sturm_day_of_year: Option<f64>,
    pub coe_boundary_depth_m: f64,
    pub coe_boundary_density_kg_m3: f64,
    pub coe_boundary_settle_day_count: f64,
    pub snow_albedo_model: Option<SnowAlbedoModel>,
    pub snow_albedo_state: Option<SnowAlbedoState>,
    pub snow_layers: Vec<DirectSnowLayerState>,
    pub underlying_surface_albedo: f64,
    pub hourly: [DirectSnowHourlyForcing; SIMIMPL29_HOURS_PER_DAY],
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct DirectFrostHourlyForcing {
    pub radiation_mj_m2: f64,
    pub air_temperature_c: f64,
    pub cloud_fraction: f64,
}

impl DirectFrostHourlyForcing {
    #[must_use]
    pub const fn zero() -> Self {
        Self {
            radiation_mj_m2: 0.0,
            air_temperature_c: 0.0,
            cloud_fraction: 0.0,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct DirectFrostControlInputs {
    pub frost_file_present: bool,
    pub wint_red_enabled: bool,
    pub fine_top_count: usize,
    pub fine_bot_count: usize,
    pub ksnowf: f64,
    pub kresf: f64,
    pub ksoilf: f64,
    pub kfactor1: f64,
    pub kfactor2: f64,
    pub kfactor3: f64,
    pub landuse_class_proxy: Option<f64>,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct DirectFrostThermalInputs {
    pub snow_depth_m: f64,
    pub snow_density_kg_m3: f64,
    pub residue_depth_m: f64,
    pub wind_m_s: f64,
    pub albedo: f64,
    pub canopy_height_m: f64,
    pub random_roughness_m: f64,
    pub day_of_year: f64,
    pub seasonal_temperature_curve: FrostSeasonalTemperatureCurve,
}

// The fitted legacy tmpcft seasonal air-temperature wave. Derived once per
// lane from the twelve monthly max/min normals (which are static for a run)
// via `Wb11HydrologyKernel::fit_seasonal_temperature_curve`, and carried as
// the single authority so the kernel never re-fits per solve.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct FrostSeasonalTemperatureCurve {
    pub annual_mean_c: f64,
    pub amplitude_c: f64,
    pub phase_shift_days: f64,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct DirectFrostLayerInput {
    pub layer_index: usize,
    pub theta_m: f64,
    pub upper_limit_m: f64,
    pub depth_m: f64,
    pub residual_theta: f64,
    pub bulk_density_kg_m3: f64,
    pub frozen_depth_m: f64,
    pub frozen_water_m: f64,
}

#[derive(Debug, Clone, PartialEq)]
pub struct DirectFrostPriorStateInput {
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
    pub layer_shadows: Vec<DirectFrostLayerShadowProjection>,
    pub fine_layers: Vec<DirectFrostFineLayerProjection>,
}

impl DirectFrostPriorStateInput {
    #[must_use]
    pub fn zero() -> Self {
        Self {
            active_frost_coupling: false,
            dfrost_m: 0.0,
            dthaw_m: 0.0,
            nft: 0.0,
            ws_frz_m: 0.0,
            infcap_frz_m_s: 0.0,
            frwatc_soil_water_before_m: 0.0,
            frwatc_soil_water_after_m: 0.0,
            frwatc_frozen_water_before_m: 0.0,
            frwatc_frozen_water_after_m: 0.0,
            frwatc_freeze_debit_m: 0.0,
            frwatc_thaw_credit_m: 0.0,
            frwatc_net_liquid_delta_m: 0.0,
            frdp_m: 0.0,
            thdp_m: 0.0,
            tfrdp_m: 0.0,
            tthawd_m: 0.0,
            fgthwd_flag: 0.0,
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
            layer_shadows: Vec::new(),
            fine_layers: Vec::new(),
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct DirectActiveFrostPartitionInputs {
    pub controls: DirectFrostControlInputs,
    pub thermal: DirectFrostThermalInputs,
    pub profile_depth_m: f64,
    pub soil_water_m: f64,
    pub theta_residual: f64,
    pub theta_field_capacity: f64,
    pub soil_conductivity_m_s: f64,
    pub prior_state: DirectFrostPriorStateInput,
    pub layers: Vec<DirectFrostLayerInput>,
    pub hourly: [DirectFrostHourlyForcing; SIMIMPL29_HOURS_PER_DAY],
}

#[derive(Debug, Clone, PartialEq)]
pub struct DirectWinterFrostComputeInputs {
    pub controls: DirectFrostControlInputs,
    pub thermal: DirectFrostThermalInputs,
    pub theta_residual: f64,
    pub theta_field_capacity: f64,
    pub soil_conductivity_m_s: Option<f64>,
    pub layer_bulk_density_kg_m3: Vec<f64>,
    pub hourly: [DirectFrostHourlyForcing; SIMIMPL29_HOURS_PER_DAY],
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct DirectFrostLayerProjection {
    pub layer_index: usize,
    pub theta_after_m: f64,
    pub frozen_depth_m: f64,
    pub frozen_water_m: f64,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct DirectFrostLayerShadowProjection {
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
pub struct DirectFrostFineLayerProjection {
    pub layer_index: usize,
    pub fine_index: usize,
    pub fgfrst: f64,
    pub slfsd_m: f64,
    pub slsic_m: f64,
    pub slsw_theta: f64,
    pub sltime_s: f64,
}

#[derive(Debug, Clone, PartialEq)]
pub struct DirectWinterFrostPartitionOutcome {
    pub active_frost_coupling: bool,
    pub dthaw_after_m: f64,
    pub nft_after: f64,
    pub infcap_frz_m_s: f64,
    pub soil_water_after_frwatc_m: Option<f64>,
    pub frwatc_soil_water_before_m: f64,
    pub frwatc_soil_water_after_m: f64,
    pub frwatc_frozen_water_before_m: f64,
    pub frwatc_frozen_water_after_m: f64,
    pub frwatc_freeze_debit_m: f64,
    pub frwatc_thaw_credit_m: f64,
    pub frwatc_net_liquid_delta_m: f64,
    pub frozen_water_after_m: f64,
    pub frost_depth_after_m: f64,
    pub thdp_after_m: f64,
    pub tfrdp_after_m: f64,
    pub tthawd_after_m: f64,
    pub fgthwd_flag_after: f64,
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
    pub layer_projection: Vec<DirectFrostLayerProjection>,
    pub layer_shadow_projection: Vec<DirectFrostLayerShadowProjection>,
    pub fine_layer_projection: Vec<DirectFrostFineLayerProjection>,
}

#[derive(Debug, Clone, Copy)]
pub(crate) struct SnowMeltRedistributionOutcome {
    routed_melt_total_m: f64,
    snowpack_state_loss_m: f64,
}

#[derive(Debug, Clone)]
pub(crate) struct FrostCouplingOutcome {
    dthaw: f64,
    nft: f64,
    ws_frz: f64,
    infcap_frz: f64,
    soil_water_after_frwatc: Option<f64>,
    frwatc_soil_water_before: f64,
    frwatc_soil_water_after: f64,
    frwatc_frozen_water_before: f64,
    frwatc_frozen_water_after: f64,
    frwatc_freeze_debit: f64,
    frwatc_thaw_credit: f64,
    frwatc_net_liquid_delta: f64,
    frdp_m: f64,
    thdp_m: f64,
    tfrdp_m: f64,
    tthawd_m: f64,
    fgthwd_flag: f64,
    total_fine_layer_count: f64,
    conductivity_tilled_w_m_k: f64,
    conductivity_untilled_w_m_k: f64,
    conductivity_residue_w_m_k: f64,
    shadow_total_water_before_m: f64,
    shadow_total_water_after_m: f64,
    shadow_wb_delta_m: f64,
    shadow_frwatc_residual_m: f64,
    watpdg_m: f64,
    watbtm_m: f64,
    hourly_state: [FrostHourlyState; SIMIMPL29_HOURS_PER_DAY],
    layer_topology_state: Vec<FrostLayerTopologyState>,
    shadow_layer_state: Vec<FrostLayerShadowState>,
    fine_layer_state: Vec<FrostFineLayerDiagnosticState>,
}

#[derive(Debug, Clone, Copy)]
pub(crate) struct FrostHourlyState {
    hour: usize,
    frzflg: f64,
    surface_temp_c: f64,
    qsrf_w_m2: f64,
    quf_w_m2: f64,
    ksrf_w_m_k: f64,
    tilled_frozen_depth_m: f64,
    untilled_frozen_depth_m: f64,
}

#[derive(Debug, Clone, Copy)]
pub(crate) struct FrostLayerTopologyState {
    layer_index: usize,
    theta_after_m: f64,
    frozen_depth_m: f64,
    frzw_m: f64,
}

#[derive(Debug, Clone, Copy)]
pub(crate) struct FrostLayerShadowState {
    layer_index: usize,
    st_m: f64,
    soil_water_m: f64,
    frozen_depth_m: f64,
    frzw_m: f64,
    soilf_m: f64,
    yst_m: f64,
    nwfrzz_m: f64,
}

#[derive(Debug, Clone, Copy)]
pub(crate) struct FrostFineLayerDiagnosticState {
    layer_index: usize,
    fine_index: usize,
    fgfrst: f64,
    slfsd_m: f64,
    slsic_m: f64,
    slsw_theta: f64,
    sltime_s: f64,
}

const SNOW_RUNTIME_DEPTH_M_SYMBOL: &str = "snow.runtime_depth_m";
const SNOW_RUNTIME_DENSITY_KG_M3_SYMBOL: &str = "snow.runtime_density_kg_m3";
const SNOW_RUNTIME_SETTLE_DAY_COUNT_SYMBOL: &str = "snow.runtime_settle_day_count";const SNOW_HOURLY_MELT_ROOT: &str = "snow.hourly.melt_m";const SNOW_HOURLY_RAIN_ROOT: &str = "snow.hourly.rain_m";const SNOW_HOURLY_SNOWFALL_ROOT: &str = "snow.hourly.snowfall_m";
const SNOW_HOURLY_SUBLIMATION_ROOT: &str = "snow.hourly.sublimation_m";

const WINTER_HOURLY_RAD_ROOT: &str = "winter.hourly.rad_mj_m2";
const WINTER_HOURLY_AIR_TEMP_ROOT: &str = "winter.hourly.air_temp_c";
const WINTER_HOURLY_CLOUD_ROOT: &str = "winter.hourly.cloud_fraction";const FROST_RUNTIME_FRDP_M_SYMBOL: &str = "frost.runtime_frdp_m";
const FROST_RUNTIME_THDP_M_SYMBOL: &str = "frost.runtime_thdp_m";
const FROST_RUNTIME_TFRDP_M_SYMBOL: &str = "frost.runtime_tfrdp_m";
const FROST_RUNTIME_TTHAWD_M_SYMBOL: &str = "frost.runtime_tthawd_m";
const FROST_RUNTIME_FGTHWD_FLAG_SYMBOL: &str = "frost.runtime_fgthwd_flag";const FROST_RUNTIME_FINE_FGFRST_ROOT: &str = "frost.runtime_fgfrst";
const FROST_RUNTIME_FINE_SLFSD_M_ROOT: &str = "frost.runtime_slfsd_m";
const FROST_RUNTIME_FINE_SLSIC_M_ROOT: &str = "frost.runtime_slsic_m";
const FROST_RUNTIME_FINE_SLSW_THETA_ROOT: &str = "frost.runtime_slsw_theta";
const FROST_RUNTIME_FINE_SLTIME_S_ROOT: &str = "frost.runtime_sltime_s";
const FROST_RUNTIME_LAYER_YST_M_ROOT: &str = "frost.runtime_yst_m";
const FROST_RUNTIME_LAYER_NWFRZZ_M_ROOT: &str = "frost.runtime_nwfrzz_m";const FROST_RUNTIME_SNOW_DEPTH_SYMBOL: &str = "snow.runtime_depth_m";
const FROST_RUNTIME_RESIDUE_DEPTH_SYMBOL: &str = "frost.runtime_residue_depth_m";
const FROST_LANDUSE_CLASS_PROXY_SYMBOL: &str = "landuse.class_proxy";
const FROST_RUNTIME_TILLAGE_DEPTH_M: f64 = 0.20;
const FROST_RUNTIME_KFTILL_W_M_K: f64 = 1.75;
const FROST_RUNTIME_KFUTIL_W_M_K: f64 = 2.1;
const FROST_RUNTIME_KRES_BASE_W_M_K: f64 = 0.05;
const FROST_RUNTIME_LATENT_HEAT_WATER_J_M3: f64 = 3.35e8;
const FROST_RUNTIME_SECONDS_PER_HOUR: f64 = 3_600.0;
const FROST_RUNTIME_UNFROZEN_LOWER_HEAT_PATH_M: f64 = 1.0;
const FROST_RUNTIME_UNFROZEN_CONDUCTIVITY_FALLBACK_W_M_K: f64 = 0.2;
const FROST_RUNTIME_SOIL_DAMPING_DEPTH_M: f64 = 2.0;
const FROST_RUNTIME_FINE_THETA_BOUND_TOLERANCE: f64 = 1.0e-10;
const FROST_RUNTIME_SHALLOW_FRONT_MIN_CONDUCTION_PATH_M: f64 = 0.005;

const SIMIMPL29_HOURS_PER_DAY: usize = 24;
const SIMIMPL29_SNOW_DENSITY_CAP_KG_M3: f64 = 522.0;
const SIMIMPL29_DENSITY_MELT_GATE_KG_M3: f64 = 350.0;
const SIMIMPL29_SNOWPACK_SETTLE_BASE: f64 = 0.041_666_7;
const SIMIMPL29_LIQUID_HOLDING_CAPACITY_VOLUME_FRACTION: f64 = 0.01;
const SIMIMPL29_RHO_ICE_KG_M3: f64 = 917.0;
const SIMIMPL29_CANOPY_FACTOR: f64 = 1.0;
const SIMIMPL29_WIND_MEASUREMENT_HEIGHT_M: f64 = 10.0;
const SIMIMPL29_SNOWPACK_STATE_LOSS_OVERDRAW_TOLERANCE_M: f64 = 0.005;
const SNOW_SUBLIMATION_ROUGHNESS_LENGTH_M: f64 = 0.005;
const SNOW_SUBLIMATION_VON_KARMAN: f64 = 0.4;
const SNOW_SUBLIMATION_WATER_MOLECULAR_WEIGHT_KG_MOL: f64 = 0.018_015_28;
const SNOW_SUBLIMATION_UNIVERSAL_GAS_CONSTANT_J_MOL_K: f64 = 8.314_462_618;
const SNOW_SUBLIMATION_SURFACE_TEMP_K: f64 = 273.15;
const SNOW_SUBLIMATION_MIN_AIR_TEMP_K: f64 = 173.15;
const SNOW_SUBLIMATION_KPA_TO_PA: f64 = 1_000.0;
const SNOW_SUBLIMATION_RHO_WATER_KG_M3: f64 = 1_000.0;
const SNOW_SUBLIMATION_STAGE_B_ACTIVE_LAYER_DEPTH_M: f64 = 0.25;
const SNOW_SUBLIMATION_STAGE_B_ICE_HEAT_CAPACITY_J_KG_K: f64 = 2_100.0;// UNIT-CONVERSION-ALLOW: mm_m_scale legacy minimum snow-depth threshold in meters, not conversion.
const SIMIMPL29_MIN_CONDUCTIVE_SNOW_DEPTH_M: f64 = 0.001;


mod support_helpers_mod;
pub use support_helpers_mod::{
    DirectKsatadjEffectiveConductivityInputs, DirectKsatadjEffectiveConductivityOutcome,
    DirectKsatadjLayerInputs, DirectSnowDiagnosticCapture,
    DirectSnowLiquidDispositionLedger, DirectSnowMassTransitionLedgerError,
    DirectSnowMassTransitionLedgers, DirectSnowSolidToLiquidLedger,
    DirectSnowStage3Outcome, DirectSnowVerboseDiagnostics,
};
pub(crate) use support_helpers_mod::DirectSnowStage3Resolution;
