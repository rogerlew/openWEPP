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

fn covered_fixed_point_soil_states_equal(
    left: &SoilThermalSnapshot,
    right: &SoilThermalSnapshot,
) -> bool {
    left.ofes.len() == right.ofes.len()
        && left.ofes.iter().zip(&right.ofes).all(|(left_ofe, right_ofe)| {
            left_ofe.ofe_id == right_ofe.ofe_id
                && left_ofe.ordered_layers.len() == right_ofe.ordered_layers.len()
                && left_ofe.ordered_layers.iter().zip(&right_ofe.ordered_layers).all(
                    |(left_layer, right_layer)| {
                        left_layer.layer_id == right_layer.layer_id
                            && close_with_policy(
                                left_layer.temperature_k,
                                right_layer.temperature_k,
                                COVERED_FIXED_POINT_POLICY.temperature_abs_k,
                                COVERED_FIXED_POINT_POLICY.temperature_rel,
                            )
                    },
                )
        })
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
            && close_flux(
                lhs.shared_heat_residual_w_m2,
                rhs.shared_heat_residual_w_m2,
            )
            && close_flux(
                lhs.shared_heat_tolerance_w_m2,
                rhs.shared_heat_tolerance_w_m2,
            )
            && close_vapor(
                lhs.shared_vapor_residual_kg_m2_s,
                rhs.shared_vapor_residual_kg_m2_s,
            )
            && close_vapor(
                lhs.shared_vapor_tolerance_kg_m2_s,
                rhs.shared_vapor_tolerance_kg_m2_s,
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

fn covered_fixed_point_stage3_underrelaxed_iterate_v1(
    left: &BTreeMap<u32, DirectSnowStage3PersistentState>,
    right: &BTreeMap<u32, DirectSnowStage3PersistentState>,
    candidate_weight: f64,
) -> Option<BTreeMap<u32, DirectSnowStage3PersistentState>> {
    if !candidate_weight.is_finite() || !(0.0..=1.0).contains(&candidate_weight) {
        return None;
    }
    let blend = |left: f64, right: f64| left + candidate_weight * (right - left);
    if left.keys().collect::<BTreeSet<_>>() != right.keys().collect::<BTreeSet<_>>() {
        return None;
    }
    left.iter()
        .map(|(lane_id, left)| {
            let right = right.get(lane_id)?;
            let posture = |state: &DirectSnowStage3PersistentState| {
                (
                    crate::hydrology::stage3_is_resolved_thermal_domain(state),
                    crate::hydrology::stage3_is_terminal_event_domain(state),
                    crate::hydrology::stage3_has_represented_ice(state),
                )
            };
            if left.schema_version != right.schema_version
                || left.terminal_event_model != right.terminal_event_model
                || left.lane_id != right.lane_id
                || left.next_interval_index != right.next_interval_index
                || left.layers.len() != right.layers.len()
                || left.initial_ice_kg_m2.to_bits() != right.initial_ice_kg_m2.to_bits()
                || left.initial_retained_liquid_kg_m2.to_bits()
                    != right.initial_retained_liquid_kg_m2.to_bits()
                || left.cumulative_snowfall_kg_m2.to_bits()
                    != right.cumulative_snowfall_kg_m2.to_bits()
                || left.cumulative_external_liquid_kg_m2.to_bits()
                    != right.cumulative_external_liquid_kg_m2.to_bits()
                || (left.detached_retained_liquid_kg_m2 == 0.0)
                    != (right.detached_retained_liquid_kg_m2 == 0.0)
                || posture(left) != posture(right)
            {
                return None;
            }
            let mut midpoint = right.clone();
            for ((midpoint, left), right) in midpoint
                .layers
                .iter_mut()
                .zip(&left.layers)
                .zip(&right.layers)
            {
                if left.settle_day_count.to_bits() != right.settle_day_count.to_bits()
                    || (left.mass_swe_m == 0.0) != (right.mass_swe_m == 0.0)
                    || (left.liquid_water_m == 0.0) != (right.liquid_water_m == 0.0)
                    || (left.cold_content_j_m2 == 0.0)
                        != (right.cold_content_j_m2 == 0.0)
                    || (left.refrozen_liquid_m == 0.0)
                        != (right.refrozen_liquid_m == 0.0)
                    || crate::hydrology::snow_density_layer_has_resolved_mass(left.mass_swe_m)
                        != crate::hydrology::snow_density_layer_has_resolved_mass(right.mass_swe_m)
                {
                    return None;
                }
                // Density remains an exact, unblended structural value from
                // the authentic candidate. A one-ULP candidate change must
                // still fail the convergence predicate above, but it cannot
                // disable damping of the continuous mass/energy iterate and
                // thereby perpetuate the change.
                midpoint.mass_swe_m = blend(left.mass_swe_m, right.mass_swe_m);
                midpoint.liquid_water_m = blend(left.liquid_water_m, right.liquid_water_m);
                midpoint.cold_content_j_m2 =
                    blend(left.cold_content_j_m2, right.cold_content_j_m2);
                midpoint.refrozen_liquid_m =
                    blend(left.refrozen_liquid_m, right.refrozen_liquid_m);
                midpoint.thickness_m = midpoint.mass_swe_m * 1_000.0 / midpoint.density_kg_m3;
                midpoint.temperature_c =
                    Wb11HydrologyKernel::stage3_temperature_from_cold_content_values(
                        midpoint.mass_swe_m,
                        midpoint.cold_content_j_m2,
                    );
            }
            midpoint.detached_retained_liquid_kg_m2 = blend(
                left.detached_retained_liquid_kg_m2,
                right.detached_retained_liquid_kg_m2,
            );
            midpoint.cumulative_deposition_kg_m2 = blend(
                left.cumulative_deposition_kg_m2,
                right.cumulative_deposition_kg_m2,
            );
            midpoint.cumulative_sublimation_kg_m2 = blend(
                left.cumulative_sublimation_kg_m2,
                right.cumulative_sublimation_kg_m2,
            );
            midpoint.cumulative_melt_kg_m2 =
                blend(left.cumulative_melt_kg_m2, right.cumulative_melt_kg_m2);
            midpoint.cumulative_unresolved_liquid_kg_m2 = blend(
                left.cumulative_unresolved_liquid_kg_m2,
                right.cumulative_unresolved_liquid_kg_m2,
            );
            midpoint.cumulative_complete_energy_j_m2 = blend(
                left.cumulative_complete_energy_j_m2,
                right.cumulative_complete_energy_j_m2,
            );
            midpoint.cumulative_cold_energy_change_j_m2 = blend(
                left.cumulative_cold_energy_change_j_m2,
                right.cumulative_cold_energy_change_j_m2,
            );
            midpoint.cumulative_terminal_unallocated_energy_j_m2 = blend(
                left.cumulative_terminal_unallocated_energy_j_m2,
                right.cumulative_terminal_unallocated_energy_j_m2,
            );
            midpoint.fingerprint = Wb11HydrologyKernel::stage3_persistent_state_fingerprint(&midpoint);
            Wb11HydrologyKernel::validate_stage3_persistent_state(&midpoint).ok()?;
            Wb11HydrologyKernel::validate_stage3_persistent_cumulative_closure(&midpoint).ok()?;
            if posture(&midpoint) != posture(left)
                || midpoint
                    .layers
                    .iter()
                    .zip(&left.layers)
                    .any(|(midpoint, left)| {
                        crate::hydrology::snow_density_layer_has_resolved_mass(
                            midpoint.mass_swe_m,
                        ) != crate::hydrology::snow_density_layer_has_resolved_mass(
                            left.mass_swe_m,
                        )
                    })
            {
                return None;
            }
            Some((*lane_id, midpoint))
        })
        .collect()
}

fn covered_fixed_point_exact_floor_period_two_detected_v1(
    previous_previous: Option<&BTreeMap<u32, DirectSnowStage3PersistentState>>,
    previous: Option<&BTreeMap<u32, DirectSnowStage3PersistentState>>,
    candidate: &BTreeMap<u32, DirectSnowStage3PersistentState>,
) -> bool {
    let (Some(previous_previous), Some(previous)) = (previous_previous, previous) else {
        return false;
    };
    !covered_fixed_point_stage3_states_equal(previous, candidate)
        && covered_fixed_point_stage3_states_equal(previous_previous, candidate)
}

fn covered_fixed_point_relaxation_weight_v1(
    support_duration_ns: u128,
    exact_floor_period_two_detected: bool,
) -> Option<f64> {
    let minimum =
        crate::snow_stage3_v11_attachment::STAGE3_V11_ADAPTIVE_MINIMUM_SUPPORT_NS;
    if support_duration_ns < minimum
        || (support_duration_ns == minimum && !exact_floor_period_two_detected)
    {
        return None;
    }
    let support_scaled = (2 * minimum) as f64 / support_duration_ns as f64;
    Some(support_scaled.clamp(0.25, 0.5))
}

#[derive(Default)]
struct CoveredFinalizationStabilizationV1 {
    pending: bool,
}

impl CoveredFinalizationStabilizationV1 {
    fn observe_restart(&mut self, relaxed_restart_applied: bool) {
        self.pending = relaxed_restart_applied;
    }

    fn picard_accepts_convergence(
        &mut self,
        coupled_map_converged: bool,
        relaxation_enabled: bool,
    ) -> bool {
        if coupled_map_converged && self.pending && relaxation_enabled {
            self.pending = false;
            return false;
        }
        coupled_map_converged
    }
}

fn covered_fixed_point_finalization_stage3_iterate_v1(
    current: &BTreeMap<u32, DirectSnowStage3PersistentState>,
    authentic_candidate: &BTreeMap<u32, DirectSnowStage3PersistentState>,
    support_duration_ns: u128,
    exact_floor_period_two_detected: bool,
) -> BTreeMap<u32, DirectSnowStage3PersistentState> {
    covered_fixed_point_relaxation_weight_v1(
        support_duration_ns,
        exact_floor_period_two_detected,
    )
    .and_then(|weight| {
        covered_fixed_point_stage3_underrelaxed_iterate_v1(
            current,
            authentic_candidate,
            weight,
        )
    })
    .unwrap_or_else(|| authentic_candidate.clone())
}

fn covered_fixed_point_soil_underrelaxed_iterate_v1(
    left: &SoilThermalSnapshot,
    right: &SoilThermalSnapshot,
    candidate_weight: f64,
) -> Option<SoilThermalSnapshot> {
    if !candidate_weight.is_finite() || !(0.0..=1.0).contains(&candidate_weight) {
        return None;
    }
    let blend = |left: f64, right: f64| left + candidate_weight * (right - left);
    if left.owner_id != right.owner_id
        || left.configuration_sha256 != right.configuration_sha256
        || left.last_accepted_transaction_id != right.last_accepted_transaction_id
        || left.ofes.len() != right.ofes.len()
    {
        return None;
    }
    let mut midpoint = right.clone();
    for ((midpoint_ofe, left_ofe), right_ofe) in
        midpoint.ofes.iter_mut().zip(&left.ofes).zip(&right.ofes)
    {
        if left_ofe.ofe_id != right_ofe.ofe_id
            || left_ofe.ordered_layers.len() != right_ofe.ordered_layers.len()
        {
            return None;
        }
        for ((midpoint_layer, left_layer), right_layer) in midpoint_ofe
            .ordered_layers
            .iter_mut()
            .zip(&left_ofe.ordered_layers)
            .zip(&right_ofe.ordered_layers)
        {
            if left_layer.layer_id != right_layer.layer_id {
                return None;
            }
            midpoint_layer.temperature_k =
                blend(left_layer.temperature_k, right_layer.temperature_k);
            midpoint_layer.enthalpy_j_m2_ofe_ground = blend(
                left_layer.enthalpy_j_m2_ofe_ground,
                right_layer.enthalpy_j_m2_ofe_ground,
            );
        }
    }
    let transaction_id = midpoint.last_accepted_transaction_id?;
    midpoint.state_sha256 = super::digest_soil_state(
        &midpoint.owner_id,
        transaction_id,
        &midpoint.ofes,
    )
    .ok()?;
    midpoint.snapshot_sha256 = super::digest_soil_snapshot(
        &midpoint.owner_id,
        &midpoint.configuration_sha256,
        &midpoint.state_sha256,
        transaction_id,
        &midpoint.ofes,
    )
    .ok()?;
    midpoint.validate().ok()?;
    Some(midpoint)
}

fn covered_stage3_lane_state_first_difference_v1(
    lane_id: u32,
    lhs: &DirectSnowStage3PersistentState,
    rhs: &DirectSnowStage3PersistentState,
) -> Option<(u32, &'static str, u64, u64, u64, u64)> {
    let fingerprints = (lhs.fingerprint, rhs.fingerprint);
    let structural = [
        ("schema_version", lhs.schema_version as u64, rhs.schema_version as u64),
        ("lane_id", lhs.lane_id as u64, rhs.lane_id as u64),
        (
            "next_interval_index",
            lhs.next_interval_index,
            rhs.next_interval_index,
        ),
        ("layer_count", lhs.layers.len() as u64, rhs.layers.len() as u64),
    ];
    if let Some((field, left_bits, right_bits)) = structural
        .into_iter()
        .find(|(_, left_bits, right_bits)| left_bits != right_bits)
    {
        return Some((lane_id, field, left_bits, right_bits, fingerprints.0, fingerprints.1));
    }
    if lhs.terminal_event_model != rhs.terminal_event_model {
        return Some((
            lane_id,
            "terminal_event_model",
            digest_bytes(&serde_json::to_vec(&lhs.terminal_event_model).ok()?).as_bytes()[0] as u64,
            digest_bytes(&serde_json::to_vec(&rhs.terminal_event_model).ok()?).as_bytes()[0] as u64,
            fingerprints.0,
            fingerprints.1,
        ));
    }
    let close_depth = |left, right| {
        close_with_policy(left, right, COVERED_FIXED_POINT_POLICY.depth_abs_m, 0.0)
    };
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
    for (left_layer, right_layer) in lhs.layers.iter().zip(&rhs.layers) {
        for (field, left, right, close) in [
            ("layer.mass_swe_m", left_layer.mass_swe_m, right_layer.mass_swe_m, close_depth(left_layer.mass_swe_m, right_layer.mass_swe_m)),
            ("layer.thickness_m", left_layer.thickness_m, right_layer.thickness_m, close_depth(left_layer.thickness_m, right_layer.thickness_m)),
            ("layer.density_kg_m3", left_layer.density_kg_m3, right_layer.density_kg_m3, left_layer.density_kg_m3.to_bits() == right_layer.density_kg_m3.to_bits()),
            ("layer.settle_day_count", left_layer.settle_day_count, right_layer.settle_day_count, left_layer.settle_day_count.to_bits() == right_layer.settle_day_count.to_bits()),
            ("layer.temperature_c", left_layer.temperature_c, right_layer.temperature_c, close_temperature(left_layer.temperature_c, right_layer.temperature_c)),
            ("layer.liquid_water_m", left_layer.liquid_water_m, right_layer.liquid_water_m, close_depth(left_layer.liquid_water_m, right_layer.liquid_water_m)),
            ("layer.cold_content_j_m2", left_layer.cold_content_j_m2, right_layer.cold_content_j_m2, close_energy(left_layer.cold_content_j_m2, right_layer.cold_content_j_m2)),
            ("layer.refrozen_liquid_m", left_layer.refrozen_liquid_m, right_layer.refrozen_liquid_m, close_depth(left_layer.refrozen_liquid_m, right_layer.refrozen_liquid_m)),
        ] {
            if !close {
                return Some((lane_id, field, left.to_bits(), right.to_bits(), fingerprints.0, fingerprints.1));
            }
        }
    }
    for (field, left, right, close) in [
        ("detached_retained_liquid_kg_m2", lhs.detached_retained_liquid_kg_m2, rhs.detached_retained_liquid_kg_m2, close_mass(lhs.detached_retained_liquid_kg_m2, rhs.detached_retained_liquid_kg_m2)),
        ("cumulative_snowfall_kg_m2", lhs.cumulative_snowfall_kg_m2, rhs.cumulative_snowfall_kg_m2, close_mass(lhs.cumulative_snowfall_kg_m2, rhs.cumulative_snowfall_kg_m2)),
        ("cumulative_external_liquid_kg_m2", lhs.cumulative_external_liquid_kg_m2, rhs.cumulative_external_liquid_kg_m2, close_mass(lhs.cumulative_external_liquid_kg_m2, rhs.cumulative_external_liquid_kg_m2)),
        ("cumulative_deposition_kg_m2", lhs.cumulative_deposition_kg_m2, rhs.cumulative_deposition_kg_m2, close_mass(lhs.cumulative_deposition_kg_m2, rhs.cumulative_deposition_kg_m2)),
        ("cumulative_sublimation_kg_m2", lhs.cumulative_sublimation_kg_m2, rhs.cumulative_sublimation_kg_m2, close_mass(lhs.cumulative_sublimation_kg_m2, rhs.cumulative_sublimation_kg_m2)),
        ("cumulative_melt_kg_m2", lhs.cumulative_melt_kg_m2, rhs.cumulative_melt_kg_m2, close_mass(lhs.cumulative_melt_kg_m2, rhs.cumulative_melt_kg_m2)),
        ("cumulative_unresolved_liquid_kg_m2", lhs.cumulative_unresolved_liquid_kg_m2, rhs.cumulative_unresolved_liquid_kg_m2, close_mass(lhs.cumulative_unresolved_liquid_kg_m2, rhs.cumulative_unresolved_liquid_kg_m2)),
        ("initial_ice_kg_m2", lhs.initial_ice_kg_m2, rhs.initial_ice_kg_m2, lhs.initial_ice_kg_m2.to_bits() == rhs.initial_ice_kg_m2.to_bits()),
        ("initial_retained_liquid_kg_m2", lhs.initial_retained_liquid_kg_m2, rhs.initial_retained_liquid_kg_m2, lhs.initial_retained_liquid_kg_m2.to_bits() == rhs.initial_retained_liquid_kg_m2.to_bits()),
        ("cumulative_complete_energy_j_m2", lhs.cumulative_complete_energy_j_m2, rhs.cumulative_complete_energy_j_m2, close_energy(lhs.cumulative_complete_energy_j_m2, rhs.cumulative_complete_energy_j_m2)),
        ("cumulative_cold_energy_change_j_m2", lhs.cumulative_cold_energy_change_j_m2, rhs.cumulative_cold_energy_change_j_m2, close_energy(lhs.cumulative_cold_energy_change_j_m2, rhs.cumulative_cold_energy_change_j_m2)),
        ("cumulative_terminal_unallocated_energy_j_m2", lhs.cumulative_terminal_unallocated_energy_j_m2, rhs.cumulative_terminal_unallocated_energy_j_m2, close_energy(lhs.cumulative_terminal_unallocated_energy_j_m2, rhs.cumulative_terminal_unallocated_energy_j_m2)),
    ] {
        if !close {
            return Some((lane_id, field, left.to_bits(), right.to_bits(), fingerprints.0, fingerprints.1));
        }
    }
    (lhs.fingerprint != rhs.fingerprint).then_some((
        lane_id,
        "fingerprint",
        lhs.fingerprint,
        rhs.fingerprint,
        fingerprints.0,
        fingerprints.1,
    ))
}

fn covered_stage3_state_first_difference_v1(
    left: &BTreeMap<u32, DirectSnowStage3PersistentState>,
    right: &BTreeMap<u32, DirectSnowStage3PersistentState>,
) -> Option<(u32, &'static str, u64, u64, u64, u64)> {
    if left.keys().collect::<BTreeSet<_>>() != right.keys().collect::<BTreeSet<_>>() {
        return left
            .keys()
            .chain(right.keys())
            .copied()
            .find(|lane| !left.contains_key(lane) || !right.contains_key(lane))
            .map(|lane| (lane, "lane_set", 0, 0, 0, 0));
    }
    left.iter().find_map(|(lane_id, lhs)| {
        covered_stage3_lane_state_first_difference_v1(*lane_id, lhs, &right[lane_id])
    })
}
