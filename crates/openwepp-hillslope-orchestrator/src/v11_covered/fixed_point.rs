include!("phase_consistent_coupled_solve.rs");

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
        && left
            .ofes
            .iter()
            .zip(&right.ofes)
            .all(|(left_ofe, right_ofe)| {
                left_ofe.ofe_id == right_ofe.ofe_id
                    && left_ofe.ordered_layers.len() == right_ofe.ordered_layers.len()
                    && left_ofe
                        .ordered_layers
                        .iter()
                        .zip(&right_ofe.ordered_layers)
                        .all(|(left_layer, right_layer)| {
                            left_layer.layer_id == right_layer.layer_id
                                && close_with_policy(
                                    left_layer.temperature_k,
                                    right_layer.temperature_k,
                                    COVERED_FIXED_POINT_POLICY.temperature_abs_k,
                                    COVERED_FIXED_POINT_POLICY.temperature_rel,
                                )
                        })
            })
}

fn covered_fixed_point_soil_candidates_equal(
    left: &DirectSoilThermalCandidate,
    right: &DirectSoilThermalCandidate,
) -> bool {
    match (left, right) {
        (DirectSoilThermalCandidate::V1(left), DirectSoilThermalCandidate::V1(right)) => {
            covered_fixed_point_soil_states_equal(left, right)
        }
        (DirectSoilThermalCandidate::V2(left), DirectSoilThermalCandidate::V2(right)) => {
            let left = left.ending_state();
            let right = right.ending_state();
            left.owner_id == right.owner_id
                && left.configuration_sha256 == right.configuration_sha256
                && left.ofes.len() == right.ofes.len()
                && left
                    .ofes
                    .iter()
                    .zip(&right.ofes)
                    .all(|(left_ofe, right_ofe)| {
                        left_ofe.ofe_id == right_ofe.ofe_id
                            && left_ofe.ordered_layers.len() == right_ofe.ordered_layers.len()
                            && left_ofe
                                .ordered_layers
                                .iter()
                                .zip(&right_ofe.ordered_layers)
                                .all(|(left_layer, right_layer)| {
                                    covered_fixed_point_v2_soil_layers_equal(
                                        left_layer,
                                        right_layer,
                                    )
                                })
                    })
        }
        _ => false,
    }
}

fn covered_fixed_point_v2_soil_layers_equal(
    left: &openwepp_land_surface_energy::SoilThermalLayerStateV2,
    right: &openwepp_land_surface_energy::SoilThermalLayerStateV2,
) -> bool {
    left.layer_id == right.layer_id
        && matches!(
            openwepp_land_surface_energy::exact_reconstructed_enthalpy_within_abs_tolerance(
                left.enthalpy_hi_j_m2_ofe_ground,
                &left.enthalpy_carry,
                right.enthalpy_hi_j_m2_ofe_ground,
                &right.enthalpy_carry,
                COVERED_FIXED_POINT_POLICY.energy_abs_j_m2,
            ),
            Ok(true)
        )
        && close_with_policy(
            left.temperature_k,
            right.temperature_k,
            COVERED_FIXED_POINT_POLICY.temperature_abs_k,
            0.0,
        )
}

#[cfg(test)]
mod v2_exact_enthalpy_convergence_tests {
    use super::*;
    use openwepp_kernel_contract::SoilLayerId;

    fn layer(
        temperature_k: f64,
        enthalpy_high_j_m2: f64,
        enthalpy_carry: openwepp_land_surface_energy::ExactDyadicEnthalpy,
    ) -> openwepp_land_surface_energy::SoilThermalLayerStateV2 {
        openwepp_land_surface_energy::SoilThermalLayerStateV2 {
            layer_id: SoilLayerId::try_new("soil-1").expect("layer identity"),
            temperature_k,
            enthalpy_hi_j_m2_ofe_ground: enthalpy_high_j_m2,
            enthalpy_carry,
            last_accepted_transaction_id: None,
        }
    }

    #[test]
    fn v2_convergence_uses_exact_total_energy_and_absolute_temperature_bounds() {
        let left = layer(
            273.15,
            1.0e16,
            openwepp_land_surface_energy::ExactDyadicEnthalpy::zero(),
        );
        let mut right = layer(
            273.15 + 5.0e-9,
            1.0e16,
            openwepp_land_surface_energy::ExactDyadicEnthalpy::from_f64(1.0e-6)
                .expect("exact tolerance carry"),
        );
        assert!(covered_fixed_point_v2_soil_layers_equal(&left, &right));

        right.enthalpy_carry = openwepp_land_surface_energy::ExactDyadicEnthalpy::from_f64(
            f64::from_bits(1.0e-6_f64.to_bits() + 1),
        )
        .expect("next binary64 above tolerance");
        assert!(!covered_fixed_point_v2_soil_layers_equal(&left, &right));

        right.enthalpy_carry = openwepp_land_surface_energy::ExactDyadicEnthalpy::zero();
        right.temperature_k = 273.15 + 2.0e-8;
        assert!(!covered_fixed_point_v2_soil_layers_equal(&left, &right));

        right.temperature_k = left.temperature_k;
        right.layer_id = SoilLayerId::try_new("soil-2").expect("different layer identity");
        assert!(!covered_fixed_point_v2_soil_layers_equal(&left, &right));
    }
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
            && close_flux(lhs.shared_heat_residual_w_m2, rhs.shared_heat_residual_w_m2)
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
                    || (left.cold_content_j_m2 == 0.0) != (right.cold_content_j_m2 == 0.0)
                    || (left.refrozen_liquid_m == 0.0) != (right.refrozen_liquid_m == 0.0)
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
                midpoint.cold_content_j_m2 = blend(left.cold_content_j_m2, right.cold_content_j_m2);
                midpoint.refrozen_liquid_m = blend(left.refrozen_liquid_m, right.refrozen_liquid_m);
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
            midpoint.fingerprint =
                Wb11HydrologyKernel::stage3_persistent_state_fingerprint(&midpoint);
            Wb11HydrologyKernel::validate_stage3_persistent_state(&midpoint).ok()?;
            Wb11HydrologyKernel::validate_stage3_persistent_cumulative_closure(&midpoint).ok()?;
            if posture(&midpoint) != posture(left)
                || midpoint
                    .layers
                    .iter()
                    .zip(&left.layers)
                    .any(|(midpoint, left)| {
                        crate::hydrology::snow_density_layer_has_resolved_mass(midpoint.mass_swe_m)
                            != crate::hydrology::snow_density_layer_has_resolved_mass(
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

#[derive(Clone, Debug, PartialEq)]
struct CoveredExactFloorTerminalPhaseSupportImageV1 {
    parent_start_ns: u128,
    parent_end_ns: u128,
    support_start_ns: u128,
    support_end_ns: u128,
    actual_vapor_kg_m2: f64,
    deposition_kg_m2: f64,
    sublimation_kg_m2: f64,
    snowfall_kg_m2: f64,
    external_liquid_kg_m2: f64,
    complete_energy_j_m2: f64,
    cold_content_export_j_m2: f64,
    ordered_energy_components_j_m2: [f64; 7],
    source_receipt_fingerprints: [u64; 6],
}

const COVERED_VAPOR_ACTIVE_SET_MIN_SUPPORT_NS: u128 =
    crate::snow_stage3_v11_attachment::STAGE3_V11_ADAPTIVE_MINIMUM_SUPPORT_NS;

#[derive(Clone, Debug, PartialEq)]
struct CoveredVaporActiveSetInterfaceV1 {
    support_image: CoveredExactFloorTerminalPhaseSupportImageV1,
    raw_authentic_support_image: CoveredExactFloorTerminalPhaseSupportImageV1,
    alpha_v: f64,
    publication_eligible: bool,
}

#[derive(Clone, Debug, PartialEq)]
struct CoveredVaporActiveSetIterateV1 {
    iterate: BTreeMap<u32, DirectSnowStage3PersistentState>,
    support_images: BTreeMap<u32, CoveredExactFloorTerminalPhaseSupportImageV1>,
    raw_authentic_candidate: BTreeMap<u32, DirectSnowStage3PersistentState>,
    transition: CoveredVaporActiveSetTransitionV1,
    publication_eligible: bool,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum CoveredVaporActiveSetTransitionV1 {
    Interface,
    BranchEntry,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum CoveredExactFloorTerminalPhaseErrorV1 {
    KeyStructure,
    IdentityStructure,
    LayerStructure,
    StateCursorStructure,
    InitialStructure,
    DensityStructure,
    SettlingStructure,
    DetachedStructure,
    BeginningStructure,
    SupportDomain,
    SupportIdentity,
    SnowfallIdentity,
    SourceIdentity,
    VaporDisposition,
    VaporMixedDisposition,
    VaporRoot,
    VaporLatent,
    Operand,
    Projection,
    Closure,
    EndpointClosure,
    EndpointCoordinateClosure,
    ProjectedClosure,
}

fn covered_vapor_active_set_error_detail_v1(
    error: CoveredExactFloorTerminalPhaseErrorV1,
) -> &'static str {
    match error {
        CoveredExactFloorTerminalPhaseErrorV1::KeyStructure => {
            "covered vapor active-set projection: key structure"
        }
        CoveredExactFloorTerminalPhaseErrorV1::IdentityStructure => {
            "covered vapor active-set projection: identity structure"
        }
        CoveredExactFloorTerminalPhaseErrorV1::LayerStructure => {
            "covered vapor active-set projection: layer structure"
        }
        CoveredExactFloorTerminalPhaseErrorV1::StateCursorStructure => {
            "covered vapor active-set projection: state cursor structure"
        }
        CoveredExactFloorTerminalPhaseErrorV1::InitialStructure => {
            "covered vapor active-set projection: initial structure"
        }
        CoveredExactFloorTerminalPhaseErrorV1::DensityStructure => {
            "covered vapor active-set projection: density structure"
        }
        CoveredExactFloorTerminalPhaseErrorV1::SettlingStructure => {
            "covered vapor active-set projection: settling structure"
        }
        CoveredExactFloorTerminalPhaseErrorV1::DetachedStructure => {
            "covered vapor active-set projection: detached structure"
        }
        CoveredExactFloorTerminalPhaseErrorV1::BeginningStructure => {
            "covered vapor active-set projection: beginning structure"
        }
        CoveredExactFloorTerminalPhaseErrorV1::SupportDomain => {
            "covered vapor active-set projection: support domain"
        }
        CoveredExactFloorTerminalPhaseErrorV1::SupportIdentity => {
            "covered vapor active-set projection: support identity"
        }
        CoveredExactFloorTerminalPhaseErrorV1::SnowfallIdentity => {
            "covered vapor active-set projection: snowfall identity"
        }
        CoveredExactFloorTerminalPhaseErrorV1::SourceIdentity => {
            "covered vapor active-set projection: source identity"
        }
        CoveredExactFloorTerminalPhaseErrorV1::VaporDisposition => {
            "covered vapor active-set projection: vapor disposition"
        }
        CoveredExactFloorTerminalPhaseErrorV1::VaporMixedDisposition => {
            "covered vapor active-set projection: mixed vapor disposition"
        }
        CoveredExactFloorTerminalPhaseErrorV1::VaporRoot => {
            "covered vapor active-set projection: vapor root"
        }
        CoveredExactFloorTerminalPhaseErrorV1::VaporLatent => {
            "covered vapor active-set projection: linked latent heat"
        }
        CoveredExactFloorTerminalPhaseErrorV1::Operand => {
            "covered vapor active-set projection: operand"
        }
        CoveredExactFloorTerminalPhaseErrorV1::Projection => {
            "covered vapor active-set projection: domain"
        }
        CoveredExactFloorTerminalPhaseErrorV1::Closure => {
            "covered vapor active-set projection: component closure"
        }
        CoveredExactFloorTerminalPhaseErrorV1::EndpointClosure => {
            "covered vapor active-set projection: endpoint closure"
        }
        CoveredExactFloorTerminalPhaseErrorV1::EndpointCoordinateClosure => {
            "covered vapor active-set projection: endpoint coordinates"
        }
        CoveredExactFloorTerminalPhaseErrorV1::ProjectedClosure => {
            "covered vapor active-set projection: projected closure"
        }
    }
}

impl CoveredExactFloorTerminalPhaseSupportImageV1 {
    fn validate(&self) -> Result<(), CoveredExactFloorTerminalPhaseErrorV1> {
        let finite = [
            self.actual_vapor_kg_m2,
            self.deposition_kg_m2,
            self.sublimation_kg_m2,
            self.snowfall_kg_m2,
            self.external_liquid_kg_m2,
            self.complete_energy_j_m2,
            self.cold_content_export_j_m2,
        ]
        .into_iter()
        .chain(self.ordered_energy_components_j_m2)
        .all(f64::is_finite);
        if !finite
            || self.support_start_ns >= self.support_end_ns
            || self.parent_start_ns > self.support_start_ns
            || self.support_end_ns > self.parent_end_ns
            || self.support_end_ns - self.support_start_ns < COVERED_VAPOR_ACTIVE_SET_MIN_SUPPORT_NS
            || self.deposition_kg_m2 < 0.0
            || self.sublimation_kg_m2 < 0.0
            || self.snowfall_kg_m2 < 0.0
            || self.external_liquid_kg_m2 < 0.0
            || self.cold_content_export_j_m2 < 0.0
        {
            return Err(CoveredExactFloorTerminalPhaseErrorV1::SupportDomain);
        }
        let vapor_residual =
            self.actual_vapor_kg_m2 - (self.deposition_kg_m2 - self.sublimation_kg_m2);
        let vapor_scale =
            self.actual_vapor_kg_m2.abs() + self.deposition_kg_m2 + self.sublimation_kg_m2;
        if vapor_residual.abs() > 1.0e-12_f64.max(1.0e-12 * vapor_scale) {
            return Err(CoveredExactFloorTerminalPhaseErrorV1::Operand);
        }
        let component_sum = self.ordered_energy_components_j_m2.iter().sum::<f64>();
        let component_scale = self.complete_energy_j_m2.abs()
            + self
                .ordered_energy_components_j_m2
                .iter()
                .map(|value| value.abs())
                .sum::<f64>();
        if (self.complete_energy_j_m2 - component_sum).abs()
            > 1.0e-6_f64.max(1.0e-12 * component_scale)
        {
            return Err(CoveredExactFloorTerminalPhaseErrorV1::Closure);
        }
        Ok(())
    }

    fn is_positive_zero(value: f64) -> bool {
        value.to_bits() == 0.0_f64.to_bits()
    }

    fn pure_vapor_side(&self) -> Option<std::cmp::Ordering> {
        if self.actual_vapor_kg_m2 > 0.0
            && self.deposition_kg_m2.to_bits() == self.actual_vapor_kg_m2.to_bits()
            && Self::is_positive_zero(self.sublimation_kg_m2)
        {
            Some(std::cmp::Ordering::Greater)
        } else if self.actual_vapor_kg_m2 < 0.0
            && Self::is_positive_zero(self.deposition_kg_m2)
            && self.sublimation_kg_m2.to_bits() == (-self.actual_vapor_kg_m2).to_bits()
        {
            Some(std::cmp::Ordering::Less)
        } else {
            None
        }
    }

    fn is_vapor_interface(&self) -> bool {
        Self::is_positive_zero(self.actual_vapor_kg_m2)
            && Self::is_positive_zero(self.deposition_kg_m2)
            && Self::is_positive_zero(self.sublimation_kg_m2)
            && Self::is_positive_zero(self.ordered_energy_components_j_m2[3])
    }

    fn validate_active_set_identity(
        &self,
        authentic: &Self,
    ) -> Result<(), CoveredExactFloorTerminalPhaseErrorV1> {
        self.validate()?;
        authentic.validate()?;
        if self.parent_start_ns != authentic.parent_start_ns
            || self.parent_end_ns != authentic.parent_end_ns
            || self.support_start_ns != authentic.support_start_ns
            || self.support_end_ns != authentic.support_end_ns
        {
            return Err(CoveredExactFloorTerminalPhaseErrorV1::SupportIdentity);
        }
        if self.snowfall_kg_m2.to_bits() != authentic.snowfall_kg_m2.to_bits() {
            return Err(CoveredExactFloorTerminalPhaseErrorV1::SnowfallIdentity);
        }
        if self.source_receipt_fingerprints != authentic.source_receipt_fingerprints {
            return Err(CoveredExactFloorTerminalPhaseErrorV1::SourceIdentity);
        }
        Ok(())
    }
}

fn covered_vapor_active_set_interface_v1(
    current: &CoveredExactFloorTerminalPhaseSupportImageV1,
    authentic: &CoveredExactFloorTerminalPhaseSupportImageV1,
) -> Result<CoveredVaporActiveSetInterfaceV1, CoveredExactFloorTerminalPhaseErrorV1> {
    current.validate_active_set_identity(authentic)?;
    let current_side = current
        .pure_vapor_side()
        .ok_or(CoveredExactFloorTerminalPhaseErrorV1::VaporMixedDisposition)?;
    let authentic_side = authentic
        .pure_vapor_side()
        .ok_or(CoveredExactFloorTerminalPhaseErrorV1::VaporMixedDisposition)?;
    if current_side == authentic_side {
        return Err(CoveredExactFloorTerminalPhaseErrorV1::VaporDisposition);
    }
    let denominator = authentic.actual_vapor_kg_m2 - current.actual_vapor_kg_m2;
    let alpha_v = -current.actual_vapor_kg_m2 / denominator;
    if !denominator.is_finite() || !alpha_v.is_finite() || !(0.0..1.0).contains(&alpha_v) {
        return Err(CoveredExactFloorTerminalPhaseErrorV1::VaporRoot);
    }
    let interpolate = |left: f64, right: f64| left + alpha_v * (right - left);
    let mut ordered_energy_components_j_m2 = std::array::from_fn(|index| {
        interpolate(
            current.ordered_energy_components_j_m2[index],
            authentic.ordered_energy_components_j_m2[index],
        )
    });
    ordered_energy_components_j_m2[3] = 0.0;
    let support_image = CoveredExactFloorTerminalPhaseSupportImageV1 {
        parent_start_ns: current.parent_start_ns,
        parent_end_ns: current.parent_end_ns,
        support_start_ns: current.support_start_ns,
        support_end_ns: current.support_end_ns,
        actual_vapor_kg_m2: 0.0,
        deposition_kg_m2: 0.0,
        sublimation_kg_m2: 0.0,
        snowfall_kg_m2: current.snowfall_kg_m2,
        external_liquid_kg_m2: interpolate(
            current.external_liquid_kg_m2,
            authentic.external_liquid_kg_m2,
        ),
        complete_energy_j_m2: ordered_energy_components_j_m2.iter().sum(),
        cold_content_export_j_m2: interpolate(
            current.cold_content_export_j_m2,
            authentic.cold_content_export_j_m2,
        ),
        ordered_energy_components_j_m2,
        source_receipt_fingerprints: current.source_receipt_fingerprints,
    };
    support_image.validate()?;
    Ok(CoveredVaporActiveSetInterfaceV1 {
        support_image,
        raw_authentic_support_image: authentic.clone(),
        alpha_v,
        publication_eligible: false,
    })
}

fn covered_vapor_active_set_branch_entry_v1(
    interface: &CoveredExactFloorTerminalPhaseSupportImageV1,
    authentic: &CoveredExactFloorTerminalPhaseSupportImageV1,
) -> Result<CoveredVaporActiveSetInterfaceV1, CoveredExactFloorTerminalPhaseErrorV1> {
    interface.validate_active_set_identity(authentic)?;
    if !interface.is_vapor_interface() {
        return Err(CoveredExactFloorTerminalPhaseErrorV1::VaporRoot);
    }
    authentic
        .pure_vapor_side()
        .ok_or(CoveredExactFloorTerminalPhaseErrorV1::VaporMixedDisposition)?;
    let specific_latent_heat =
        authentic.ordered_energy_components_j_m2[3] / authentic.actual_vapor_kg_m2;
    if !specific_latent_heat.is_finite() || specific_latent_heat <= 0.0 {
        return Err(CoveredExactFloorTerminalPhaseErrorV1::VaporLatent);
    }
    let duration_ns = interface.support_end_ns - interface.support_start_ns;
    if duration_ns < COVERED_VAPOR_ACTIVE_SET_MIN_SUPPORT_NS {
        return Err(CoveredExactFloorTerminalPhaseErrorV1::SupportDomain);
    }
    let candidate_weight = ((2 * COVERED_VAPOR_ACTIVE_SET_MIN_SUPPORT_NS) as f64
        / duration_ns as f64)
        .clamp(0.25, 0.5);
    let interpolate = |left: f64, right: f64| left + candidate_weight * (right - left);
    let actual_vapor_kg_m2 = candidate_weight * authentic.actual_vapor_kg_m2;
    let deposition_kg_m2 = actual_vapor_kg_m2.max(0.0);
    let sublimation_kg_m2 = (-actual_vapor_kg_m2).max(0.0);
    let mut ordered_energy_components_j_m2 = std::array::from_fn(|index| {
        interpolate(
            interface.ordered_energy_components_j_m2[index],
            authentic.ordered_energy_components_j_m2[index],
        )
    });
    ordered_energy_components_j_m2[3] = actual_vapor_kg_m2 * specific_latent_heat;
    let support_image = CoveredExactFloorTerminalPhaseSupportImageV1 {
        parent_start_ns: interface.parent_start_ns,
        parent_end_ns: interface.parent_end_ns,
        support_start_ns: interface.support_start_ns,
        support_end_ns: interface.support_end_ns,
        actual_vapor_kg_m2,
        deposition_kg_m2,
        sublimation_kg_m2,
        snowfall_kg_m2: interface.snowfall_kg_m2,
        external_liquid_kg_m2: interpolate(
            interface.external_liquid_kg_m2,
            authentic.external_liquid_kg_m2,
        ),
        complete_energy_j_m2: ordered_energy_components_j_m2.iter().sum(),
        cold_content_export_j_m2: interpolate(
            interface.cold_content_export_j_m2,
            authentic.cold_content_export_j_m2,
        ),
        ordered_energy_components_j_m2,
        source_receipt_fingerprints: interface.source_receipt_fingerprints,
    };
    support_image.validate()?;
    Ok(CoveredVaporActiveSetInterfaceV1 {
        support_image,
        raw_authentic_support_image: authentic.clone(),
        alpha_v: candidate_weight,
        publication_eligible: false,
    })
}

fn covered_exact_floor_terminal_phase_project_v1(
    beginning: &DirectSnowStage3PersistentState,
    support: &CoveredExactFloorTerminalPhaseSupportImageV1,
    layer_template: crate::DirectSnowLayerState,
    next_interval_index: u64,
) -> Result<DirectSnowStage3PersistentState, CoveredExactFloorTerminalPhaseErrorV1> {
    support.validate()?;
    if beginning.layers.len() > 1
        || beginning.detached_retained_liquid_kg_m2 != 0.0
        || !layer_template.density_kg_m3.is_finite()
        || layer_template.density_kg_m3 <= 0.0
    {
        return Err(CoveredExactFloorTerminalPhaseErrorV1::BeginningStructure);
    }
    Wb11HydrologyKernel::validate_stage3_persistent_cumulative_closure(beginning)
        .map_err(|_| CoveredExactFloorTerminalPhaseErrorV1::Closure)?;
    let beginning_ice_kg_m2 = beginning
        .layers
        .iter()
        .map(|layer| layer.mass_swe_m * 1_000.0)
        .sum::<f64>();
    let beginning_liquid_kg_m2 = beginning
        .layers
        .iter()
        .map(|layer| layer.liquid_water_m * 1_000.0)
        .sum::<f64>();
    let beginning_cold_j_m2 = beginning
        .layers
        .iter()
        .map(|layer| layer.cold_content_j_m2)
        .sum::<f64>();
    if support.sublimation_kg_m2
        > beginning_ice_kg_m2 + support.snowfall_kg_m2 + support.deposition_kg_m2
    {
        return Err(CoveredExactFloorTerminalPhaseErrorV1::Operand);
    }
    let water_kg_m2 = beginning_ice_kg_m2
        + beginning_liquid_kg_m2
        + support.snowfall_kg_m2
        + support.deposition_kg_m2
        - support.sublimation_kg_m2
        + support.external_liquid_kg_m2;
    let enthalpy_j_m2 = -beginning_cold_j_m2
        + crate::hydrology::STAGE3_LATENT_HEAT_FUSION_J_KG
            * (beginning_liquid_kg_m2 + support.external_liquid_kg_m2)
        + support.complete_energy_j_m2
        + support.cold_content_export_j_m2;
    if !water_kg_m2.is_finite() || water_kg_m2 < 0.0 || !enthalpy_j_m2.is_finite() {
        return Err(CoveredExactFloorTerminalPhaseErrorV1::Projection);
    }
    let fusion = crate::hydrology::STAGE3_LATENT_HEAT_FUSION_J_KG;
    let fusion_capacity = fusion * water_kg_m2;
    let (ice_kg_m2, liquid_kg_m2, cold_j_m2, unallocated_j_m2) = if enthalpy_j_m2 < 0.0 {
        (water_kg_m2, 0.0, -enthalpy_j_m2, 0.0)
    } else if enthalpy_j_m2 < fusion_capacity {
        let liquid = enthalpy_j_m2 / fusion;
        (water_kg_m2 - liquid, liquid, 0.0, 0.0)
    } else {
        (
            0.0,
            water_kg_m2,
            0.0,
            (enthalpy_j_m2 - fusion_capacity).max(0.0),
        )
    };
    let liquid_pre_kg_m2 = beginning_liquid_kg_m2 + support.external_liquid_kg_m2;
    let melt_kg_m2 = (liquid_kg_m2 - liquid_pre_kg_m2).max(0.0);
    let refrozen_kg_m2 = (liquid_pre_kg_m2 - liquid_kg_m2).max(0.0);
    let mut projected = beginning.clone();
    projected.next_interval_index = next_interval_index;
    projected.layers = vec![layer_template];
    let layer = &mut projected.layers[0];
    layer.mass_swe_m = ice_kg_m2 / 1_000.0;
    layer.liquid_water_m = liquid_kg_m2 / 1_000.0;
    layer.cold_content_j_m2 = cold_j_m2;
    layer.refrozen_liquid_m = refrozen_kg_m2 / 1_000.0;
    layer.thickness_m = layer.mass_swe_m * 1_000.0 / layer.density_kg_m3;
    layer.temperature_c = Wb11HydrologyKernel::stage3_temperature_from_cold_content_values(
        layer.mass_swe_m,
        layer.cold_content_j_m2,
    );
    projected.cumulative_snowfall_kg_m2 += support.snowfall_kg_m2;
    projected.cumulative_external_liquid_kg_m2 += support.external_liquid_kg_m2;
    projected.cumulative_deposition_kg_m2 += support.deposition_kg_m2;
    projected.cumulative_sublimation_kg_m2 += support.sublimation_kg_m2;
    projected.cumulative_melt_kg_m2 += melt_kg_m2;
    projected.cumulative_complete_energy_j_m2 += support.complete_energy_j_m2;
    projected.cumulative_cold_energy_change_j_m2 += beginning_cold_j_m2
        - cold_j_m2
        - fusion * refrozen_kg_m2
        - support.cold_content_export_j_m2;
    projected.cumulative_terminal_unallocated_energy_j_m2 += unallocated_j_m2;
    projected.fingerprint = Wb11HydrologyKernel::stage3_persistent_state_fingerprint(&projected);
    Wb11HydrologyKernel::validate_stage3_persistent_cumulative_closure(&projected)
        .map_err(|_| CoveredExactFloorTerminalPhaseErrorV1::ProjectedClosure)?;
    Ok(projected)
}

#[derive(Clone, Debug, PartialEq)]
#[allow(dead_code)]
struct CoveredExactFloorTerminalPhaseIterateV1 {
    iterate: DirectSnowStage3PersistentState,
    support_image: CoveredExactFloorTerminalPhaseSupportImageV1,
    raw_authentic_support_image: CoveredExactFloorTerminalPhaseSupportImageV1,
    publication_eligible: bool,
}

/// Retained version-31 exact-floor midpoint oracle.
///
/// Version 33 superseded this synthetic image as a production-control route,
/// but the exact canonical `W/H` reconstruction remains authoritative refusal
/// evidence. The result is always private and no controller calls this helper.
#[allow(dead_code)]
fn covered_exact_floor_terminal_phase_iterate_v1(
    beginning: &DirectSnowStage3PersistentState,
    current: &CoveredExactFloorTerminalPhaseSupportImageV1,
    authentic: &CoveredExactFloorTerminalPhaseSupportImageV1,
    layer_template: crate::DirectSnowLayerState,
    next_interval_index: u64,
) -> Result<CoveredExactFloorTerminalPhaseIterateV1, CoveredExactFloorTerminalPhaseErrorV1> {
    current.validate()?;
    authentic.validate()?;
    let same_support = current.parent_start_ns == authentic.parent_start_ns
        && current.parent_end_ns == authentic.parent_end_ns
        && current.support_start_ns == authentic.support_start_ns
        && current.support_end_ns == authentic.support_end_ns
        && current.support_end_ns - current.support_start_ns
            == COVERED_VAPOR_ACTIVE_SET_MIN_SUPPORT_NS;
    if !same_support {
        return Err(CoveredExactFloorTerminalPhaseErrorV1::SupportIdentity);
    }
    if current.snowfall_kg_m2.to_bits() != authentic.snowfall_kg_m2.to_bits() {
        return Err(CoveredExactFloorTerminalPhaseErrorV1::SnowfallIdentity);
    }
    if current.source_receipt_fingerprints != authentic.source_receipt_fingerprints {
        return Err(CoveredExactFloorTerminalPhaseErrorV1::SourceIdentity);
    }
    if !matches!(
        (current.pure_vapor_side(), authentic.pure_vapor_side()),
        (Some(left), Some(right)) if left == right
    ) {
        return Err(CoveredExactFloorTerminalPhaseErrorV1::VaporDisposition);
    }
    let midpoint = |left: f64, right: f64| 0.5 * left + 0.5 * right;
    let ordered_energy_components_j_m2 = std::array::from_fn(|index| {
        midpoint(
            current.ordered_energy_components_j_m2[index],
            authentic.ordered_energy_components_j_m2[index],
        )
    });
    let support_image = CoveredExactFloorTerminalPhaseSupportImageV1 {
        parent_start_ns: current.parent_start_ns,
        parent_end_ns: current.parent_end_ns,
        support_start_ns: current.support_start_ns,
        support_end_ns: current.support_end_ns,
        actual_vapor_kg_m2: midpoint(current.actual_vapor_kg_m2, authentic.actual_vapor_kg_m2),
        deposition_kg_m2: midpoint(current.deposition_kg_m2, authentic.deposition_kg_m2),
        sublimation_kg_m2: midpoint(current.sublimation_kg_m2, authentic.sublimation_kg_m2),
        snowfall_kg_m2: current.snowfall_kg_m2,
        external_liquid_kg_m2: midpoint(
            current.external_liquid_kg_m2,
            authentic.external_liquid_kg_m2,
        ),
        complete_energy_j_m2: ordered_energy_components_j_m2.iter().sum(),
        cold_content_export_j_m2: midpoint(
            current.cold_content_export_j_m2,
            authentic.cold_content_export_j_m2,
        ),
        ordered_energy_components_j_m2,
        source_receipt_fingerprints: current.source_receipt_fingerprints,
    };
    support_image.validate()?;
    let coordinates = |support: &CoveredExactFloorTerminalPhaseSupportImageV1| {
        let beginning_ice = beginning
            .layers
            .iter()
            .map(|layer| layer.mass_swe_m * 1_000.0)
            .sum::<f64>();
        let beginning_liquid = beginning
            .layers
            .iter()
            .map(|layer| layer.liquid_water_m * 1_000.0)
            .sum::<f64>();
        let beginning_cold = beginning
            .layers
            .iter()
            .map(|layer| layer.cold_content_j_m2)
            .sum::<f64>();
        (
            beginning_ice + beginning_liquid + support.snowfall_kg_m2 + support.deposition_kg_m2
                - support.sublimation_kg_m2
                + support.external_liquid_kg_m2,
            -beginning_cold
                + crate::hydrology::STAGE3_LATENT_HEAT_FUSION_J_KG
                    * (beginning_liquid + support.external_liquid_kg_m2)
                + support.complete_energy_j_m2
                + support.cold_content_export_j_m2,
        )
    };
    let current_coordinates = coordinates(current);
    let authentic_coordinates = coordinates(authentic);
    let midpoint_coordinates = coordinates(&support_image);
    if midpoint_coordinates.0.to_bits()
        != midpoint(current_coordinates.0, authentic_coordinates.0).to_bits()
        || midpoint_coordinates.1.to_bits()
            != midpoint(current_coordinates.1, authentic_coordinates.1).to_bits()
    {
        return Err(CoveredExactFloorTerminalPhaseErrorV1::EndpointCoordinateClosure);
    }
    let iterate = covered_exact_floor_terminal_phase_project_v1(
        beginning,
        &support_image,
        layer_template,
        next_interval_index,
    )?;
    Ok(CoveredExactFloorTerminalPhaseIterateV1 {
        iterate,
        support_image,
        raw_authentic_support_image: authentic.clone(),
        publication_eligible: false,
    })
}

fn covered_vapor_active_set_transition_v1(
    current_support: &BTreeMap<u32, CoveredExactFloorTerminalPhaseSupportImageV1>,
    authentic_support: &BTreeMap<u32, CoveredExactFloorTerminalPhaseSupportImageV1>,
) -> Option<CoveredVaporActiveSetTransitionV1> {
    if current_support.is_empty()
        || current_support.keys().collect::<BTreeSet<_>>()
            != authentic_support.keys().collect::<BTreeSet<_>>()
    {
        return None;
    }
    let opposite = current_support.iter().all(|(lane_id, current)| {
        let authentic = &authentic_support[lane_id];
        matches!(
            (current.pure_vapor_side(), authentic.pure_vapor_side()),
            (
                Some(std::cmp::Ordering::Greater),
                Some(std::cmp::Ordering::Less)
            ) | (
                Some(std::cmp::Ordering::Less),
                Some(std::cmp::Ordering::Greater)
            )
        )
    });
    if opposite {
        return Some(CoveredVaporActiveSetTransitionV1::Interface);
    }
    current_support
        .iter()
        .all(|(lane_id, current)| {
            current.is_vapor_interface() && authentic_support[lane_id].pure_vapor_side().is_some()
        })
        .then_some(CoveredVaporActiveSetTransitionV1::BranchEntry)
}

fn covered_vapor_active_set_endpoint_coordinates_close_v1(
    beginning: &DirectSnowStage3PersistentState,
    support: &CoveredExactFloorTerminalPhaseSupportImageV1,
    state: &DirectSnowStage3PersistentState,
) -> Result<bool, CoveredExactFloorTerminalPhaseErrorV1> {
    if state.layers.len() != 1 {
        return Err(CoveredExactFloorTerminalPhaseErrorV1::LayerStructure);
    }
    let expected = phase_consistent_support_coordinates_v1(beginning, support)
        .map_err(|_| CoveredExactFloorTerminalPhaseErrorV1::EndpointCoordinateClosure)?;
    let liquid = state.layers[0].liquid_water_m * 1_000.0;
    let actual = (
        state.layers[0].mass_swe_m * 1_000.0 + liquid,
        crate::hydrology::STAGE3_LATENT_HEAT_FUSION_J_KG * liquid
            - state.layers[0].cold_content_j_m2
            + state.cumulative_terminal_unallocated_energy_j_m2
            - beginning.cumulative_terminal_unallocated_energy_j_m2,
    );
    Ok((expected.0 - actual.0).abs() <= 1.0e-9 && (expected.1 - actual.1).abs() <= 1.0e-6)
}

fn covered_vapor_active_set_iterate_v1(
    current: &BTreeMap<u32, DirectSnowStage3PersistentState>,
    authentic_candidate: &BTreeMap<u32, DirectSnowStage3PersistentState>,
    immutable_beginning: &BTreeMap<u32, DirectSnowStage3PersistentState>,
    current_support: &BTreeMap<u32, CoveredExactFloorTerminalPhaseSupportImageV1>,
    authentic_support: &BTreeMap<u32, CoveredExactFloorTerminalPhaseSupportImageV1>,
    transition: CoveredVaporActiveSetTransitionV1,
) -> Result<CoveredVaporActiveSetIterateV1, CoveredExactFloorTerminalPhaseErrorV1> {
    let keys = current.keys().collect::<BTreeSet<_>>();
    if keys != authentic_candidate.keys().collect::<BTreeSet<_>>()
        || keys != immutable_beginning.keys().collect::<BTreeSet<_>>()
        || keys != current_support.keys().collect::<BTreeSet<_>>()
        || keys != authentic_support.keys().collect::<BTreeSet<_>>()
    {
        return Err(CoveredExactFloorTerminalPhaseErrorV1::KeyStructure);
    }
    let mut iterate = BTreeMap::new();
    let mut support_images = BTreeMap::new();
    for (lane_id, current_state) in current {
        let authentic_state = &authentic_candidate[lane_id];
        let beginning_state = &immutable_beginning[lane_id];
        if current_state.schema_version != authentic_state.schema_version
            || current_state.schema_version != beginning_state.schema_version
            || current_state.terminal_event_model != authentic_state.terminal_event_model
            || current_state.terminal_event_model != beginning_state.terminal_event_model
            || current_state.lane_id != authentic_state.lane_id
            || current_state.lane_id != beginning_state.lane_id
        {
            return Err(CoveredExactFloorTerminalPhaseErrorV1::IdentityStructure);
        }
        if current_state.layers.len() != 1
            || authentic_state.layers.len() != 1
            || beginning_state.layers.len() > 1
        {
            return Err(CoveredExactFloorTerminalPhaseErrorV1::LayerStructure);
        }
        if current_state.next_interval_index != authentic_state.next_interval_index {
            return Err(CoveredExactFloorTerminalPhaseErrorV1::StateCursorStructure);
        }
        if current_state.initial_ice_kg_m2.to_bits() != authentic_state.initial_ice_kg_m2.to_bits()
            || current_state.initial_ice_kg_m2.to_bits()
                != beginning_state.initial_ice_kg_m2.to_bits()
            || current_state.initial_retained_liquid_kg_m2.to_bits()
                != authentic_state.initial_retained_liquid_kg_m2.to_bits()
            || current_state.initial_retained_liquid_kg_m2.to_bits()
                != beginning_state.initial_retained_liquid_kg_m2.to_bits()
        {
            return Err(CoveredExactFloorTerminalPhaseErrorV1::InitialStructure);
        }
        if current_state.layers[0].density_kg_m3.to_bits()
            != authentic_state.layers[0].density_kg_m3.to_bits()
        {
            return Err(CoveredExactFloorTerminalPhaseErrorV1::DensityStructure);
        }
        if current_state.layers[0].settle_day_count.to_bits()
            != authentic_state.layers[0].settle_day_count.to_bits()
        {
            return Err(CoveredExactFloorTerminalPhaseErrorV1::SettlingStructure);
        }
        if current_state.detached_retained_liquid_kg_m2.to_bits()
            != authentic_state.detached_retained_liquid_kg_m2.to_bits()
        {
            return Err(CoveredExactFloorTerminalPhaseErrorV1::DetachedStructure);
        }
        Wb11HydrologyKernel::validate_stage3_persistent_cumulative_closure(current_state)
            .map_err(|_| CoveredExactFloorTerminalPhaseErrorV1::EndpointClosure)?;
        Wb11HydrologyKernel::validate_stage3_persistent_cumulative_closure(authentic_state)
            .map_err(|_| CoveredExactFloorTerminalPhaseErrorV1::EndpointClosure)?;
        let current_image = &current_support[lane_id];
        let authentic_image = &authentic_support[lane_id];
        let active_set = match transition {
            CoveredVaporActiveSetTransitionV1::Interface => {
                covered_vapor_active_set_interface_v1(current_image, authentic_image)?
            }
            CoveredVaporActiveSetTransitionV1::BranchEntry => {
                covered_vapor_active_set_branch_entry_v1(current_image, authentic_image)?
            }
        };
        if active_set.publication_eligible
            || active_set.raw_authentic_support_image != *authentic_image
        {
            return Err(CoveredExactFloorTerminalPhaseErrorV1::SourceIdentity);
        }
        for (image, state) in [
            (current_image, current_state),
            (authentic_image, authentic_state),
        ] {
            if !covered_vapor_active_set_endpoint_coordinates_close_v1(
                beginning_state,
                image,
                state,
            )? {
                return Err(CoveredExactFloorTerminalPhaseErrorV1::EndpointCoordinateClosure);
            }
        }
        let projected = covered_exact_floor_terminal_phase_project_v1(
            beginning_state,
            &active_set.support_image,
            authentic_state.layers[0],
            authentic_state.next_interval_index,
        )?;
        iterate.insert(*lane_id, projected);
        support_images.insert(*lane_id, active_set.support_image);
    }
    Ok(CoveredVaporActiveSetIterateV1 {
        iterate,
        support_images,
        raw_authentic_candidate: authentic_candidate.clone(),
        transition,
        publication_eligible: false,
    })
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
    let minimum = crate::snow_stage3_v11_attachment::STAGE3_V11_ADAPTIVE_MINIMUM_SUPPORT_NS;
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
    covered_fixed_point_relaxation_weight_v1(support_duration_ns, exact_floor_period_two_detected)
        .and_then(|weight| {
            covered_fixed_point_stage3_underrelaxed_iterate_v1(current, authentic_candidate, weight)
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
    midpoint.state_sha256 =
        super::digest_soil_state(&midpoint.owner_id, transaction_id, &midpoint.ofes).ok()?;
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

fn covered_fixed_point_soil_candidate_underrelaxed_iterate_v1(
    left: &DirectSoilThermalCandidate,
    right: &DirectSoilThermalCandidate,
    candidate_weight: f64,
) -> Option<DirectSoilThermalCandidate> {
    match (left, right) {
        (DirectSoilThermalCandidate::V1(left), DirectSoilThermalCandidate::V1(right)) => {
            covered_fixed_point_soil_underrelaxed_iterate_v1(left, right, candidate_weight)
                .and_then(|candidate| DirectSoilThermalCandidate::from_v1(candidate).ok())
        }
        // Exact carry is never interpolated or projected. A V2 authentic trial
        // remains the sole unpublished next iterate.
        (DirectSoilThermalCandidate::V2(_), DirectSoilThermalCandidate::V2(_)) => None,
        _ => None,
    }
}

fn covered_stage3_lane_state_first_difference_v1(
    lane_id: u32,
    lhs: &DirectSnowStage3PersistentState,
    rhs: &DirectSnowStage3PersistentState,
) -> Option<(u32, &'static str, u64, u64, u64, u64)> {
    let fingerprints = (lhs.fingerprint, rhs.fingerprint);
    let structural = [
        (
            "schema_version",
            lhs.schema_version as u64,
            rhs.schema_version as u64,
        ),
        ("lane_id", lhs.lane_id as u64, rhs.lane_id as u64),
        (
            "next_interval_index",
            lhs.next_interval_index,
            rhs.next_interval_index,
        ),
        (
            "layer_count",
            lhs.layers.len() as u64,
            rhs.layers.len() as u64,
        ),
    ];
    if let Some((field, left_bits, right_bits)) = structural
        .into_iter()
        .find(|(_, left_bits, right_bits)| left_bits != right_bits)
    {
        return Some((
            lane_id,
            field,
            left_bits,
            right_bits,
            fingerprints.0,
            fingerprints.1,
        ));
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
    for (left_layer, right_layer) in lhs.layers.iter().zip(&rhs.layers) {
        for (field, left, right, close) in [
            (
                "layer.mass_swe_m",
                left_layer.mass_swe_m,
                right_layer.mass_swe_m,
                close_depth(left_layer.mass_swe_m, right_layer.mass_swe_m),
            ),
            (
                "layer.thickness_m",
                left_layer.thickness_m,
                right_layer.thickness_m,
                close_depth(left_layer.thickness_m, right_layer.thickness_m),
            ),
            (
                "layer.density_kg_m3",
                left_layer.density_kg_m3,
                right_layer.density_kg_m3,
                left_layer.density_kg_m3.to_bits() == right_layer.density_kg_m3.to_bits(),
            ),
            (
                "layer.settle_day_count",
                left_layer.settle_day_count,
                right_layer.settle_day_count,
                left_layer.settle_day_count.to_bits() == right_layer.settle_day_count.to_bits(),
            ),
            (
                "layer.temperature_c",
                left_layer.temperature_c,
                right_layer.temperature_c,
                close_temperature(left_layer.temperature_c, right_layer.temperature_c),
            ),
            (
                "layer.liquid_water_m",
                left_layer.liquid_water_m,
                right_layer.liquid_water_m,
                close_depth(left_layer.liquid_water_m, right_layer.liquid_water_m),
            ),
            (
                "layer.cold_content_j_m2",
                left_layer.cold_content_j_m2,
                right_layer.cold_content_j_m2,
                close_energy(left_layer.cold_content_j_m2, right_layer.cold_content_j_m2),
            ),
            (
                "layer.refrozen_liquid_m",
                left_layer.refrozen_liquid_m,
                right_layer.refrozen_liquid_m,
                close_depth(left_layer.refrozen_liquid_m, right_layer.refrozen_liquid_m),
            ),
        ] {
            if !close {
                return Some((
                    lane_id,
                    field,
                    left.to_bits(),
                    right.to_bits(),
                    fingerprints.0,
                    fingerprints.1,
                ));
            }
        }
    }
    for (field, left, right, close) in [
        (
            "detached_retained_liquid_kg_m2",
            lhs.detached_retained_liquid_kg_m2,
            rhs.detached_retained_liquid_kg_m2,
            close_mass(
                lhs.detached_retained_liquid_kg_m2,
                rhs.detached_retained_liquid_kg_m2,
            ),
        ),
        (
            "cumulative_snowfall_kg_m2",
            lhs.cumulative_snowfall_kg_m2,
            rhs.cumulative_snowfall_kg_m2,
            close_mass(lhs.cumulative_snowfall_kg_m2, rhs.cumulative_snowfall_kg_m2),
        ),
        (
            "cumulative_external_liquid_kg_m2",
            lhs.cumulative_external_liquid_kg_m2,
            rhs.cumulative_external_liquid_kg_m2,
            close_mass(
                lhs.cumulative_external_liquid_kg_m2,
                rhs.cumulative_external_liquid_kg_m2,
            ),
        ),
        (
            "cumulative_deposition_kg_m2",
            lhs.cumulative_deposition_kg_m2,
            rhs.cumulative_deposition_kg_m2,
            close_mass(
                lhs.cumulative_deposition_kg_m2,
                rhs.cumulative_deposition_kg_m2,
            ),
        ),
        (
            "cumulative_sublimation_kg_m2",
            lhs.cumulative_sublimation_kg_m2,
            rhs.cumulative_sublimation_kg_m2,
            close_mass(
                lhs.cumulative_sublimation_kg_m2,
                rhs.cumulative_sublimation_kg_m2,
            ),
        ),
        (
            "cumulative_melt_kg_m2",
            lhs.cumulative_melt_kg_m2,
            rhs.cumulative_melt_kg_m2,
            close_mass(lhs.cumulative_melt_kg_m2, rhs.cumulative_melt_kg_m2),
        ),
        (
            "cumulative_unresolved_liquid_kg_m2",
            lhs.cumulative_unresolved_liquid_kg_m2,
            rhs.cumulative_unresolved_liquid_kg_m2,
            close_mass(
                lhs.cumulative_unresolved_liquid_kg_m2,
                rhs.cumulative_unresolved_liquid_kg_m2,
            ),
        ),
        (
            "initial_ice_kg_m2",
            lhs.initial_ice_kg_m2,
            rhs.initial_ice_kg_m2,
            lhs.initial_ice_kg_m2.to_bits() == rhs.initial_ice_kg_m2.to_bits(),
        ),
        (
            "initial_retained_liquid_kg_m2",
            lhs.initial_retained_liquid_kg_m2,
            rhs.initial_retained_liquid_kg_m2,
            lhs.initial_retained_liquid_kg_m2.to_bits()
                == rhs.initial_retained_liquid_kg_m2.to_bits(),
        ),
        (
            "cumulative_complete_energy_j_m2",
            lhs.cumulative_complete_energy_j_m2,
            rhs.cumulative_complete_energy_j_m2,
            close_energy(
                lhs.cumulative_complete_energy_j_m2,
                rhs.cumulative_complete_energy_j_m2,
            ),
        ),
        (
            "cumulative_cold_energy_change_j_m2",
            lhs.cumulative_cold_energy_change_j_m2,
            rhs.cumulative_cold_energy_change_j_m2,
            close_energy(
                lhs.cumulative_cold_energy_change_j_m2,
                rhs.cumulative_cold_energy_change_j_m2,
            ),
        ),
        (
            "cumulative_terminal_unallocated_energy_j_m2",
            lhs.cumulative_terminal_unallocated_energy_j_m2,
            rhs.cumulative_terminal_unallocated_energy_j_m2,
            close_energy(
                lhs.cumulative_terminal_unallocated_energy_j_m2,
                rhs.cumulative_terminal_unallocated_energy_j_m2,
            ),
        ),
    ] {
        if !close {
            return Some((
                lane_id,
                field,
                left.to_bits(),
                right.to_bits(),
                fingerprints.0,
                fingerprints.1,
            ));
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
