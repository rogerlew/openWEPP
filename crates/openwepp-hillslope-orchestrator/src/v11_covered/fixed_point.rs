fn reciprocal_longwave_receipt_digest(
    destination: &(OfeId, TileId),
    support: openwepp_coupled_time::TimeSupport,
    net_longwave_w_m2: f64,
) -> Digest32 {
    let mut bytes = Vec::with_capacity(192);
    bytes.extend_from_slice(b"OPENWEPP_RECIPROCAL_LONGWAVE_RECEIPT_V1\0");
    bytes.extend_from_slice(&support.start_ns().get().to_le_bytes());
    bytes.extend_from_slice(&support.end_ns().get().to_le_bytes());
    bytes.extend_from_slice(destination.0.as_str().as_bytes());
    bytes.push(0);
    bytes.extend_from_slice(destination.1.as_str().as_bytes());
    bytes.extend_from_slice(&net_longwave_w_m2.to_bits().to_le_bytes());
    digest_bytes(&bytes)
}
fn covered_fixed_point_boundaries_equal(
    left: &BTreeMap<(OfeId, TileId), Stage3SnowCoveredLowerBoundary>,
    right: &BTreeMap<(OfeId, TileId), Stage3SnowCoveredLowerBoundary>,
) -> bool {
    if left.keys().collect::<BTreeSet<_>>() != right.keys().collect::<BTreeSet<_>>() {
        return false;
    }
    left.iter().all(|(destination, lhs)| {
        let Some(rhs) = right.get(destination) else {
            return false;
        };
        close_with_policy(
            lhs.snow_temperature_k,
            rhs.snow_temperature_k,
            COVERED_FIXED_POINT_POLICY.temperature_abs_k,
            COVERED_FIXED_POINT_POLICY.temperature_rel,
        ) && close_with_policy(
            lhs.sensible_to_canopy_air_w_m2,
            rhs.sensible_to_canopy_air_w_m2,
            COVERED_FIXED_POINT_POLICY.flux_abs_w_m2,
            COVERED_FIXED_POINT_POLICY.flux_rel,
        ) && close_with_policy(
            lhs.vapor_to_canopy_air_kg_m2_s,
            rhs.vapor_to_canopy_air_kg_m2_s,
            COVERED_FIXED_POINT_POLICY.vapor_abs_kg_m2_s,
            COVERED_FIXED_POINT_POLICY.vapor_rel,
        ) && close_with_policy(
            lhs.net_longwave_w_m2,
            rhs.net_longwave_w_m2,
            COVERED_FIXED_POINT_POLICY.flux_abs_w_m2,
            COVERED_FIXED_POINT_POLICY.flux_rel,
        ) && close_with_policy(
            lhs.shortwave_absorbed_w_m2,
            rhs.shortwave_absorbed_w_m2,
            COVERED_FIXED_POINT_POLICY.flux_abs_w_m2,
            COVERED_FIXED_POINT_POLICY.flux_rel,
        ) && close_with_policy(
            lhs.precipitation_advection_w_m2,
            rhs.precipitation_advection_w_m2,
            COVERED_FIXED_POINT_POLICY.flux_abs_w_m2,
            COVERED_FIXED_POINT_POLICY.flux_rel,
        )
    })
}

fn close_with_policy(left: f64, right: f64, absolute: f64, relative: f64) -> bool {
    left.is_finite()
        && right.is_finite()
        && (left - right).abs() <= absolute + relative * left.abs().max(right.abs())
}

fn covered_fixed_point_lse_states_equal(
    left: &BTreeMap<(OfeId, TileId), CoveredLseIterationState>,
    right: &BTreeMap<(OfeId, TileId), CoveredLseIterationState>,
) -> bool {
    if left.keys().collect::<BTreeSet<_>>() != right.keys().collect::<BTreeSet<_>>() {
        return false;
    }
    let close_temperature = |a: f64, b: f64| {
        close_with_policy(
            a,
            b,
            COVERED_FIXED_POINT_POLICY.temperature_abs_k,
            COVERED_FIXED_POINT_POLICY.temperature_rel,
        )
    };
    let close_humidity = |a: f64, b: f64| {
        close_with_policy(
            a,
            b,
            COVERED_FIXED_POINT_POLICY.humidity_abs_kg_kg,
            COVERED_FIXED_POINT_POLICY.humidity_rel,
        )
    };
    let close_flux = |a: f64, b: f64| {
        close_with_policy(
            a,
            b,
            COVERED_FIXED_POINT_POLICY.flux_abs_w_m2,
            COVERED_FIXED_POINT_POLICY.flux_rel,
        )
    };
    let close_vapor = |a: f64, b: f64| {
        close_with_policy(
            a,
            b,
            COVERED_FIXED_POINT_POLICY.vapor_abs_kg_m2_s,
            COVERED_FIXED_POINT_POLICY.vapor_rel,
        )
    };
    left.iter().all(|(destination, lhs)| {
        let Some(rhs) = right.get(destination) else {
            return false;
        };
        close_temperature(lhs.canopy_air_temperature_k, rhs.canopy_air_temperature_k)
            && close_humidity(
                lhs.canopy_air_specific_humidity_kg_kg,
                rhs.canopy_air_specific_humidity_kg_kg,
            )
            && close_temperature(lhs.snow_temperature_k, rhs.snow_temperature_k)
            && close_flux(lhs.snow_sensible_w_m2, rhs.snow_sensible_w_m2)
            && close_vapor(lhs.snow_vapor_kg_m2_s, rhs.snow_vapor_kg_m2_s)
            && close_flux(lhs.snow_latent_w_m2, rhs.snow_latent_w_m2)
            && close_flux(lhs.snow_net_longwave_w_m2, rhs.snow_net_longwave_w_m2)
            && close_flux(lhs.canopy_sensible_w_m2, rhs.canopy_sensible_w_m2)
            && close_vapor(lhs.canopy_vapor_kg_m2_s, rhs.canopy_vapor_kg_m2_s)
            && close_flux(
                lhs.sensible_to_reference_air_w_m2,
                rhs.sensible_to_reference_air_w_m2,
            )
            && close_vapor(
                lhs.vapor_to_reference_air_kg_m2_s,
                rhs.vapor_to_reference_air_kg_m2_s,
            )
            && lhs.component_temperatures_k.len() == rhs.component_temperatures_k.len()
            && lhs
                .component_temperatures_k
                .iter()
                .zip(&rhs.component_temperatures_k)
                .all(|((left_id, left_values), (right_id, right_values))| {
                    left_id == right_id
                        && left_values
                            .iter()
                            .zip(right_values)
                            .all(|(left, right)| close_temperature(*left, *right))
                })
            && lhs.component_carrier_surfaces.len() == rhs.component_carrier_surfaces.len()
            && lhs
                .component_carrier_surfaces
                .iter()
                .zip(&rhs.component_carrier_surfaces)
                .all(|(left, right)| {
                    left.occupancy_id == right.occupancy_id
                        && left.component_ordinal == right.component_ordinal
                        && left.surface_area_m2_m2_tile.to_bits()
                            == right.surface_area_m2_m2_tile.to_bits()
                        && left.emissive_area_m2_m2_tile.to_bits()
                            == right.emissive_area_m2_m2_tile.to_bits()
                        && close_with_policy(
                            left.heat_conductance_m_s_tile,
                            right.heat_conductance_m_s_tile,
                            0.0,
                            COVERED_FIXED_POINT_POLICY.flux_rel,
                        )
                        && close_with_policy(
                            left.vapor_conductance_m_s_tile,
                            right.vapor_conductance_m_s_tile,
                            0.0,
                            COVERED_FIXED_POINT_POLICY.vapor_rel,
                        )
                        && match (
                            left.vapor_authorization_kg_m2_tile_s,
                            right.vapor_authorization_kg_m2_tile_s,
                        ) {
                            (Some(left), Some(right)) => close_vapor(left, right),
                            (None, None) => true,
                            _ => false,
                        }
                        && close_temperature(left.temperature_k, right.temperature_k)
                        && close_humidity(
                            left.specific_humidity_kg_kg,
                            right.specific_humidity_kg_kg,
                        )
                        && close_flux(
                            left.sensible_to_canopy_air_w_m2,
                            right.sensible_to_canopy_air_w_m2,
                        )
                        && close_vapor(
                            left.vapor_to_canopy_air_kg_m2_s,
                            right.vapor_to_canopy_air_kg_m2_s,
                        )
                })
    })
}

fn covered_fixed_point_stage3_states_equal(
    left: &BTreeMap<u32, DirectSnowStage3PersistentState>,
    right: &BTreeMap<u32, DirectSnowStage3PersistentState>,
) -> bool {
    let close_depth =
        |left, right| close_with_policy(left, right, COVERED_FIXED_POINT_POLICY.depth_abs_m, 0.0);
    let close_temperature = |left, right| {
        close_with_policy(
            left,
            right,
            COVERED_FIXED_POINT_POLICY.state_temperature_abs_k,
            0.0,
        )
    };
    let close_mass = |left, right| {
        close_with_policy(left, right, COVERED_FIXED_POINT_POLICY.mass_abs_kg_m2, 0.0)
    };
    let close_energy = |left, right| {
        close_with_policy(left, right, COVERED_FIXED_POINT_POLICY.energy_abs_j_m2, 0.0)
    };
    left.keys().collect::<BTreeSet<_>>() == right.keys().collect::<BTreeSet<_>>()
        && left.iter().all(|(lane_id, lhs)| {
            let Some(rhs) = right.get(lane_id) else {
                return false;
            };
            if lhs.fingerprint != Wb11HydrologyKernel::stage3_persistent_state_fingerprint(lhs)
                || rhs.fingerprint != Wb11HydrologyKernel::stage3_persistent_state_fingerprint(rhs)
            {
                return false;
            }
            lhs.schema_version == rhs.schema_version
                && lhs.terminal_event_model == rhs.terminal_event_model
                && lhs.lane_id == rhs.lane_id
                && lhs.next_interval_index == rhs.next_interval_index
                && lhs.layers.len() == rhs.layers.len()
                && lhs.layers.iter().zip(&rhs.layers).all(|(left, right)| {
                    close_depth(left.mass_swe_m, right.mass_swe_m)
                        && close_depth(left.thickness_m, right.thickness_m)
                        && left.density_kg_m3.to_bits() == right.density_kg_m3.to_bits()
                        && left.settle_day_count.to_bits() == right.settle_day_count.to_bits()
                        && close_temperature(left.temperature_c, right.temperature_c)
                        && close_depth(left.liquid_water_m, right.liquid_water_m)
                        && close_energy(left.cold_content_j_m2, right.cold_content_j_m2)
                        && close_depth(left.refrozen_liquid_m, right.refrozen_liquid_m)
                })
                && [
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
                ]
                .into_iter()
                .all(|(left, right)| close_mass(left, right))
                && lhs.initial_ice_kg_m2.to_bits() == rhs.initial_ice_kg_m2.to_bits()
                && lhs.initial_retained_liquid_kg_m2.to_bits()
                    == rhs.initial_retained_liquid_kg_m2.to_bits()
                && [
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
                ]
                .into_iter()
                .all(|(left, right)| close_energy(left, right))
        })
}
