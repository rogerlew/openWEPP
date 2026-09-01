fn covered_normalized_delta_v1(left: f64, right: f64, absolute: f64, relative: f64) -> f64 {
    if !left.is_finite() || !right.is_finite() {
        return f64::INFINITY;
    }
    let delta = (left - right).abs();
    if delta == 0.0 {
        return 0.0;
    }
    let scale = absolute + relative * left.abs().max(right.abs());
    if scale > 0.0 && scale.is_finite() {
        delta / scale
    } else {
        f64::INFINITY
    }
}

fn covered_boundary_max_normalized_delta_v1(
    left: Option<&BTreeMap<(OfeId, TileId), Stage3SnowCoveredLowerBoundary>>,
    right: &BTreeMap<(OfeId, TileId), Stage3SnowCoveredLowerBoundary>,
) -> f64 {
    let Some(left) = left else { return f64::NAN };
    if left.keys().ne(right.keys()) {
        return f64::INFINITY;
    }
    let mut maximum = 0.0_f64;
    for (destination, lhs) in left {
        let Some(rhs) = right.get(destination) else {
            return f64::INFINITY;
        };
        for delta in [
            covered_normalized_delta_v1(
                lhs.snow_temperature_k,
                rhs.snow_temperature_k,
                COVERED_FIXED_POINT_POLICY.temperature_abs_k,
                COVERED_FIXED_POINT_POLICY.temperature_rel,
            ),
            covered_normalized_delta_v1(
                lhs.sensible_to_canopy_air_w_m2,
                rhs.sensible_to_canopy_air_w_m2,
                COVERED_FIXED_POINT_POLICY.flux_abs_w_m2,
                COVERED_FIXED_POINT_POLICY.flux_rel,
            ),
            covered_normalized_delta_v1(
                lhs.vapor_to_canopy_air_kg_m2_s,
                rhs.vapor_to_canopy_air_kg_m2_s,
                COVERED_FIXED_POINT_POLICY.vapor_abs_kg_m2_s,
                COVERED_FIXED_POINT_POLICY.vapor_rel,
            ),
            covered_normalized_delta_v1(
                lhs.net_longwave_w_m2,
                rhs.net_longwave_w_m2,
                COVERED_FIXED_POINT_POLICY.flux_abs_w_m2,
                COVERED_FIXED_POINT_POLICY.flux_rel,
            ),
            covered_normalized_delta_v1(
                lhs.shortwave_absorbed_w_m2,
                rhs.shortwave_absorbed_w_m2,
                COVERED_FIXED_POINT_POLICY.flux_abs_w_m2,
                COVERED_FIXED_POINT_POLICY.flux_rel,
            ),
            covered_normalized_delta_v1(
                lhs.precipitation_advection_w_m2,
                rhs.precipitation_advection_w_m2,
                COVERED_FIXED_POINT_POLICY.flux_abs_w_m2,
                COVERED_FIXED_POINT_POLICY.flux_rel,
            ),
        ] {
            maximum = maximum.max(delta);
        }
    }
    maximum
}

fn covered_lse_max_normalized_delta_v1(
    left: Option<&BTreeMap<(OfeId, TileId), CoveredLseIterationState>>,
    right: &BTreeMap<(OfeId, TileId), CoveredLseIterationState>,
) -> f64 {
    let Some(left) = left else { return f64::NAN };
    if left.keys().ne(right.keys()) {
        return f64::INFINITY;
    }
    let mut maximum = 0.0_f64;
    let mut observe = |left, right, absolute, relative| {
        maximum = maximum.max(covered_normalized_delta_v1(left, right, absolute, relative));
    };
    for (destination, lhs) in left {
        let Some(rhs) = right.get(destination) else {
            return f64::INFINITY;
        };
        for (left, right) in [
            (lhs.canopy_air_temperature_k, rhs.canopy_air_temperature_k),
            (lhs.snow_temperature_k, rhs.snow_temperature_k),
        ] {
            observe(
                left,
                right,
                COVERED_FIXED_POINT_POLICY.temperature_abs_k,
                COVERED_FIXED_POINT_POLICY.temperature_rel,
            );
        }
        observe(
            lhs.canopy_air_specific_humidity_kg_kg,
            rhs.canopy_air_specific_humidity_kg_kg,
            COVERED_FIXED_POINT_POLICY.humidity_abs_kg_kg,
            COVERED_FIXED_POINT_POLICY.humidity_rel,
        );
        for (left, right) in [
            (lhs.snow_sensible_w_m2, rhs.snow_sensible_w_m2),
            (lhs.snow_latent_w_m2, rhs.snow_latent_w_m2),
            (lhs.snow_net_longwave_w_m2, rhs.snow_net_longwave_w_m2),
            (lhs.canopy_sensible_w_m2, rhs.canopy_sensible_w_m2),
            (
                lhs.sensible_to_reference_air_w_m2,
                rhs.sensible_to_reference_air_w_m2,
            ),
            (lhs.shared_heat_residual_w_m2, rhs.shared_heat_residual_w_m2),
            (
                lhs.shared_heat_tolerance_w_m2,
                rhs.shared_heat_tolerance_w_m2,
            ),
        ] {
            observe(
                left,
                right,
                COVERED_FIXED_POINT_POLICY.flux_abs_w_m2,
                COVERED_FIXED_POINT_POLICY.flux_rel,
            );
        }
        for (left, right) in [
            (lhs.snow_vapor_kg_m2_s, rhs.snow_vapor_kg_m2_s),
            (lhs.canopy_vapor_kg_m2_s, rhs.canopy_vapor_kg_m2_s),
            (
                lhs.vapor_to_reference_air_kg_m2_s,
                rhs.vapor_to_reference_air_kg_m2_s,
            ),
            (
                lhs.shared_vapor_residual_kg_m2_s,
                rhs.shared_vapor_residual_kg_m2_s,
            ),
            (
                lhs.shared_vapor_tolerance_kg_m2_s,
                rhs.shared_vapor_tolerance_kg_m2_s,
            ),
        ] {
            observe(
                left,
                right,
                COVERED_FIXED_POINT_POLICY.vapor_abs_kg_m2_s,
                COVERED_FIXED_POINT_POLICY.vapor_rel,
            );
        }
        if lhs.component_temperatures_k.len() != rhs.component_temperatures_k.len()
            || lhs.component_carrier_surfaces.len() != rhs.component_carrier_surfaces.len()
        {
            return f64::INFINITY;
        }
        for ((left_id, left_values), (right_id, right_values)) in lhs
            .component_temperatures_k
            .iter()
            .zip(&rhs.component_temperatures_k)
        {
            if left_id != right_id {
                return f64::INFINITY;
            }
            for (left, right) in left_values.iter().zip(right_values) {
                observe(
                    *left,
                    *right,
                    COVERED_FIXED_POINT_POLICY.temperature_abs_k,
                    COVERED_FIXED_POINT_POLICY.temperature_rel,
                );
            }
        }
        for (left, right) in lhs
            .component_carrier_surfaces
            .iter()
            .zip(&rhs.component_carrier_surfaces)
        {
            if left.occupancy_id != right.occupancy_id
                || left.component_ordinal != right.component_ordinal
            {
                return f64::INFINITY;
            }
            observe(
                left.surface_area_m2_m2_tile,
                right.surface_area_m2_m2_tile,
                0.0,
                0.0,
            );
            observe(
                left.emissive_area_m2_m2_tile,
                right.emissive_area_m2_m2_tile,
                0.0,
                0.0,
            );
            observe(
                left.heat_conductance_m_s_tile,
                right.heat_conductance_m_s_tile,
                0.0,
                COVERED_FIXED_POINT_POLICY.flux_rel,
            );
            observe(
                left.vapor_conductance_m_s_tile,
                right.vapor_conductance_m_s_tile,
                0.0,
                COVERED_FIXED_POINT_POLICY.vapor_rel,
            );
            match (
                left.vapor_authorization_kg_m2_tile_s,
                right.vapor_authorization_kg_m2_tile_s,
            ) {
                (Some(left), Some(right)) => observe(
                    left,
                    right,
                    COVERED_FIXED_POINT_POLICY.vapor_abs_kg_m2_s,
                    COVERED_FIXED_POINT_POLICY.vapor_rel,
                ),
                (None, None) => {}
                _ => return f64::INFINITY,
            }
            observe(
                left.temperature_k,
                right.temperature_k,
                COVERED_FIXED_POINT_POLICY.temperature_abs_k,
                COVERED_FIXED_POINT_POLICY.temperature_rel,
            );
            observe(
                left.specific_humidity_kg_kg,
                right.specific_humidity_kg_kg,
                COVERED_FIXED_POINT_POLICY.humidity_abs_kg_kg,
                COVERED_FIXED_POINT_POLICY.humidity_rel,
            );
            observe(
                left.sensible_to_canopy_air_w_m2,
                right.sensible_to_canopy_air_w_m2,
                COVERED_FIXED_POINT_POLICY.flux_abs_w_m2,
                COVERED_FIXED_POINT_POLICY.flux_rel,
            );
            observe(
                left.vapor_to_canopy_air_kg_m2_s,
                right.vapor_to_canopy_air_kg_m2_s,
                COVERED_FIXED_POINT_POLICY.vapor_abs_kg_m2_s,
                COVERED_FIXED_POINT_POLICY.vapor_rel,
            );
        }
    }
    maximum
}

fn covered_stage3_max_normalized_delta_v1(
    left: Option<&BTreeMap<u32, DirectSnowStage3PersistentState>>,
    right: &BTreeMap<u32, DirectSnowStage3PersistentState>,
) -> f64 {
    let Some(left) = left else { return f64::NAN };
    if left.keys().ne(right.keys()) {
        return f64::INFINITY;
    }
    let mut maximum = 0.0_f64;
    let mut observe = |left, right, absolute| {
        maximum = maximum.max(covered_normalized_delta_v1(left, right, absolute, 0.0));
    };
    for (lane_id, lhs) in left {
        let Some(rhs) = right.get(lane_id) else {
            return f64::INFINITY;
        };
        if lhs.schema_version != rhs.schema_version
            || lhs.terminal_event_model != rhs.terminal_event_model
            || lhs.lane_id != rhs.lane_id
            || lhs.next_interval_index != rhs.next_interval_index
            || lhs.layers.len() != rhs.layers.len()
        {
            return f64::INFINITY;
        }
        for (left, right) in lhs.layers.iter().zip(&rhs.layers) {
            observe(
                left.mass_swe_m,
                right.mass_swe_m,
                COVERED_FIXED_POINT_POLICY.depth_abs_m,
            );
            observe(
                left.thickness_m,
                right.thickness_m,
                COVERED_FIXED_POINT_POLICY.depth_abs_m,
            );
            observe(left.density_kg_m3, right.density_kg_m3, 0.0);
            observe(left.settle_day_count, right.settle_day_count, 0.0);
            observe(
                left.temperature_c,
                right.temperature_c,
                COVERED_FIXED_POINT_POLICY.state_temperature_abs_k,
            );
            observe(
                left.liquid_water_m,
                right.liquid_water_m,
                COVERED_FIXED_POINT_POLICY.depth_abs_m,
            );
            observe(
                left.cold_content_j_m2,
                right.cold_content_j_m2,
                COVERED_FIXED_POINT_POLICY.energy_abs_j_m2,
            );
            observe(
                left.refrozen_liquid_m,
                right.refrozen_liquid_m,
                COVERED_FIXED_POINT_POLICY.depth_abs_m,
            );
        }
        for (left, right) in [
            (
                lhs.detached_retained_liquid_kg_m2,
                rhs.detached_retained_liquid_kg_m2,
            ),
            (lhs.cumulative_snowfall_kg_m2, rhs.cumulative_snowfall_kg_m2),
            (
                lhs.cumulative_external_liquid_kg_m2,
                rhs.cumulative_external_liquid_kg_m2,
            ),
            (
                lhs.cumulative_deposition_kg_m2,
                rhs.cumulative_deposition_kg_m2,
            ),
            (
                lhs.cumulative_sublimation_kg_m2,
                rhs.cumulative_sublimation_kg_m2,
            ),
            (lhs.cumulative_melt_kg_m2, rhs.cumulative_melt_kg_m2),
            (
                lhs.cumulative_unresolved_liquid_kg_m2,
                rhs.cumulative_unresolved_liquid_kg_m2,
            ),
        ] {
            observe(left, right, COVERED_FIXED_POINT_POLICY.mass_abs_kg_m2);
        }
        observe(lhs.initial_ice_kg_m2, rhs.initial_ice_kg_m2, 0.0);
        observe(
            lhs.initial_retained_liquid_kg_m2,
            rhs.initial_retained_liquid_kg_m2,
            0.0,
        );
        for (left, right) in [
            (
                lhs.cumulative_complete_energy_j_m2,
                rhs.cumulative_complete_energy_j_m2,
            ),
            (
                lhs.cumulative_cold_energy_change_j_m2,
                rhs.cumulative_cold_energy_change_j_m2,
            ),
            (
                lhs.cumulative_terminal_unallocated_energy_j_m2,
                rhs.cumulative_terminal_unallocated_energy_j_m2,
            ),
        ] {
            observe(left, right, COVERED_FIXED_POINT_POLICY.energy_abs_j_m2);
        }
    }
    maximum
}

fn covered_soil_max_normalized_deltas_v1(
    left: Option<&DirectSoilThermalCandidate>,
    right: &DirectSoilThermalCandidate,
) -> (f64, f64) {
    let Some(left) = left else {
        return (f64::NAN, f64::NAN);
    };
    if std::mem::discriminant(left) != std::mem::discriminant(right) {
        return (f64::INFINITY, f64::INFINITY);
    }
    let left_ofes = left.read_view().ordered_ofes();
    let right_ofes = right.read_view().ordered_ofes();
    if left_ofes.len() != right_ofes.len() {
        return (f64::INFINITY, f64::INFINITY);
    }
    let mut enthalpy = 0.0_f64;
    let mut temperature = 0.0_f64;
    for (left_ofe, right_ofe) in left_ofes.into_iter().zip(right_ofes) {
        let left_layers = left_ofe.ordered_layers();
        let right_layers = right_ofe.ordered_layers();
        if left_ofe.ofe_id() != right_ofe.ofe_id() || left_layers.len() != right_layers.len() {
            return (f64::INFINITY, f64::INFINITY);
        }
        for (left, right) in left_layers.into_iter().zip(right_layers) {
            if left.layer_id() != right.layer_id() {
                return (f64::INFINITY, f64::INFINITY);
            }
            enthalpy = enthalpy.max(covered_normalized_delta_v1(
                left.enthalpy_high_j_m2_ofe_ground(),
                right.enthalpy_high_j_m2_ofe_ground(),
                COVERED_FIXED_POINT_POLICY.energy_abs_j_m2,
                0.0,
            ));
            // Exact carry remains part of the authoritative convergence
            // boolean. The high-component magnitude is still retained here
            // so a slowly contracting exact-carry join is distinguishable
            // from a topology failure.
            let _exact_carry_converged = left.exact_carry() == right.exact_carry();
            temperature = temperature.max(covered_normalized_delta_v1(
                left.temperature_k(),
                right.temperature_k(),
                COVERED_FIXED_POINT_POLICY.temperature_abs_k,
                COVERED_FIXED_POINT_POLICY.temperature_rel,
            ));
        }
    }
    (enthalpy, temperature)
}
