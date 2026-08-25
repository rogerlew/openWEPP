#[allow(clippy::wildcard_imports)]
use super::*;

impl Wb11HydrologyKernel {
    pub(super) fn route_stage3_liquid_through_layers(
        incoming_liquid_m: f64,
        layers: &mut [DirectSnowLayerState],
        cold_content_by_layer: &mut [f64],
        reconstruct_temperature: bool,
    ) -> (f64, f64, f64) {
        let mut liquid_to_route_m = incoming_liquid_m;
        let mut retained_delta_m = 0.0;
        let mut refrozen_liquid_m = 0.0;
        for (layer, cold_content) in layers.iter_mut().zip(cold_content_by_layer.iter_mut()) {
            let refreeze_capacity_m = (*cold_content
                / (STAGE3_LATENT_HEAT_FUSION_J_KG * STAGE3_RHO_WATER_KG_M3))
                .max(0.0);
            let refrozen_here_m = liquid_to_route_m.min(refreeze_capacity_m);
            liquid_to_route_m -= refrozen_here_m;
            *cold_content -=
                refrozen_here_m * STAGE3_LATENT_HEAT_FUSION_J_KG * STAGE3_RHO_WATER_KG_M3;
            refrozen_liquid_m += refrozen_here_m;

            let capacity_m = Self::stage3_layer_liquid_holding_capacity_m(
                layer.thickness_m,
                layer.density_kg_m3,
            );
            let available_capacity_m = (capacity_m - layer.liquid_water_m).max(0.0);
            let retained_here_m = liquid_to_route_m.min(available_capacity_m);
            liquid_to_route_m -= retained_here_m;
            retained_delta_m += retained_here_m;

            layer.liquid_water_m += retained_here_m;
            layer.refrozen_liquid_m += refrozen_here_m;
            layer.cold_content_j_m2 = (*cold_content).max(0.0);
            if reconstruct_temperature {
                layer.temperature_c = Self::stage3_temperature_from_cold_content(layer);
            }
        }
        (
            liquid_to_route_m.max(0.0),
            retained_delta_m,
            refrozen_liquid_m,
        )
    }

    pub(super) fn validate_stage3_layer(
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
            || BoundarySymbol::from("snow.stage3_layer_settle_day_count"),
            layer.settle_day_count,
            Some(0.0),
            None,
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

    pub(super) fn stage3_total_ice_mass_swe_m(layers: &[DirectSnowLayerState]) -> f64 {
        layers.iter().map(|layer| layer.mass_swe_m).sum()
    }

    pub(super) fn stage3_control_volume_masses_swe_m(
        layers: &[DirectSnowLayerState],
        active_layer_count: usize,
    ) -> (f64, f64) {
        let active_mass_swe_m = Self::stage3_total_ice_mass_swe_m(&layers[..active_layer_count]);
        let lower_mass_swe_m = if active_layer_count < layers.len() {
            Self::stage3_total_ice_mass_swe_m(&layers[active_layer_count..])
        } else {
            0.0
        };
        (active_mass_swe_m, lower_mass_swe_m)
    }

    pub(crate) fn stage3_lower_volume_is_subresolution_swe_m(lower_mass_swe_m: f64) -> bool {
        lower_mass_swe_m > 0.0 && lower_mass_swe_m < STAGE3_MINIMUM_RESOLVED_THERMAL_MASS_SWE_M
    }

    pub(super) fn stage3_layer_cold_content_j_m2(layer: &DirectSnowLayerState) -> f64 {
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
}
