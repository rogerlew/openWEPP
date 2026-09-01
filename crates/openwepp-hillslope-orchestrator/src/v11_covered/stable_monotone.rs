fn stable_monotone_append_bytes_v1(target: &mut Vec<u8>, bytes: &[u8]) {
    target.extend_from_slice(&(bytes.len() as u64).to_le_bytes());
    target.extend_from_slice(bytes);
}

fn covered_snow_soil_receipt_sets_exact_v1(
    left: &BTreeMap<u32, SnowSoilHeatReceiptV1>,
    right: &BTreeMap<u32, SnowSoilHeatReceiptV1>,
) -> bool {
    if left.keys().ne(right.keys()) {
        return false;
    }
    left.iter().all(|(lane_id, left)| {
        let Some(right) = right.get(lane_id) else {
            return false;
        };
        if crate::snow_stage3_v11_attachment::validate_snow_soil_heat_receipt(left).is_err()
            || crate::snow_stage3_v11_attachment::validate_snow_soil_heat_receipt(right).is_err()
        {
            return false;
        }
        left.schema_version == right.schema_version
            && left.model_identity_sha256 == right.model_identity_sha256
            && left.support == right.support
            && left.support_duration_ns == right.support_duration_ns
            && left.lane_id == right.lane_id
            && left.ofe_id == right.ofe_id
            && left.ofe_ground_basis == right.ofe_ground_basis
            && left.topology_identity_sha256 == right.topology_identity_sha256
            && left.configuration_identity_sha256 == right.configuration_identity_sha256
            && left.beginning_snow_owner_identity_sha256
                == right.beginning_snow_owner_identity_sha256
            && left.beginning_soil_owner_identity_sha256
                == right.beginning_soil_owner_identity_sha256
            && left.bottom_snow_layer_id == right.bottom_snow_layer_id
            && left.first_soil_layer_id == right.first_soil_layer_id
            && left.bottom_snow_half_thickness_m.to_bits()
                == right.bottom_snow_half_thickness_m.to_bits()
            && left.bottom_snow_conductivity_w_m_k.to_bits()
                == right.bottom_snow_conductivity_w_m_k.to_bits()
            && left.top_soil_half_thickness_m.to_bits() == right.top_soil_half_thickness_m.to_bits()
            && left.top_soil_conductivity_w_m_k.to_bits()
                == right.top_soil_conductivity_w_m_k.to_bits()
            && left.beginning_bottom_snow_temperature_k.to_bits()
                == right.beginning_bottom_snow_temperature_k.to_bits()
            && left.beginning_top_soil_temperature_k.to_bits()
                == right.beginning_top_soil_temperature_k.to_bits()
            && left.ending_bottom_snow_temperature_k.to_bits()
                == right.ending_bottom_snow_temperature_k.to_bits()
            && left.ending_top_soil_temperature_k.to_bits()
                == right.ending_top_soil_temperature_k.to_bits()
            && left.beginning_heat_flux_w_m2_ofe_ground.to_bits()
                == right.beginning_heat_flux_w_m2_ofe_ground.to_bits()
            && left.ending_heat_flux_w_m2_ofe_ground.to_bits()
                == right.ending_heat_flux_w_m2_ofe_ground.to_bits()
            && left.accepted_heat_flux_w_m2_ofe_ground.to_bits()
                == right.accepted_heat_flux_w_m2_ofe_ground.to_bits()
            && left.accepted_heat_j_m2_ofe_ground.to_bits()
                == right.accepted_heat_j_m2_ofe_ground.to_bits()
            && left.snow_candidate_heat_j_m2_ofe_ground.to_bits()
                == right.snow_candidate_heat_j_m2_ofe_ground.to_bits()
            && left.soil_candidate_heat_j_m2_ofe_ground.to_bits()
                == right.soil_candidate_heat_j_m2_ofe_ground.to_bits()
            && left.snow_candidate_ending_identity_sha256
                == right.snow_candidate_ending_identity_sha256
            && left.soil_candidate_ending_identity_sha256
                == right.soil_candidate_ending_identity_sha256
            && left.receipt_sha256 == right.receipt_sha256
    })
}

fn stable_monotone_stage_coordinates_v1(
    state: &DirectSnowStage3PersistentState,
) -> Result<(f64, f64, f64), DirectV11RealConsumerError> {
    Wb11HydrologyKernel::validate_stage3_persistent_state(state).map_err(|_| {
        DirectV11RealConsumerError::AdaptiveRefinement(
            "stable-monotone authentic snow owner validation",
        )
    })?;
    if state.layers.len() != 1
        || state.detached_retained_liquid_kg_m2.to_bits() != 0.0_f64.to_bits()
    {
        return Err(DirectV11RealConsumerError::AdaptiveRefinement(
            "stable-monotone terminal one-volume snow structure",
        ));
    }
    let layer = &state.layers[0];
    let water = 1_000.0 * (layer.mass_swe_m + layer.liquid_water_m);
    let enthalpy =
        crate::hydrology::STAGE3_LATENT_HEAT_FUSION_J_KG * 1_000.0 * layer.liquid_water_m
            - layer.cold_content_j_m2;
    if !water.is_finite()
        || !enthalpy.is_finite()
        || water <= 0.0
        || !layer.density_kg_m3.is_finite()
        || layer.density_kg_m3 <= 0.0
        || !layer.thickness_m.is_finite()
        || layer.thickness_m <= 0.0
        || layer.thickness_m.to_bits()
            != (1_000.0 * layer.mass_swe_m / layer.density_kg_m3).to_bits()
    {
        return Err(DirectV11RealConsumerError::AdaptiveRefinement(
            "stable-monotone finite snow coordinates",
        ));
    }
    Ok((water, enthalpy, layer.density_kg_m3))
}

fn stable_monotone_receipt_static_join_v1(
    receipt: &SnowSoilHeatReceiptV1,
) -> Result<Vec<u8>, DirectV11RealConsumerError> {
    crate::snow_stage3_v11_attachment::validate_snow_soil_heat_receipt(receipt)
        .map_err(|error| DirectV11RealConsumerError::from_stage3_physical_custody(&error))?;
    let mut bytes = Vec::new();
    bytes.extend_from_slice(b"OPENWEPP_COVERED_STABLE_MONOTONE_STATIC_RECEIPT_JOIN_V1\0");
    bytes.extend_from_slice(&receipt.schema_version.to_le_bytes());
    bytes.extend_from_slice(receipt.model_identity_sha256.as_bytes());
    bytes.extend_from_slice(&receipt.support.start_ns().get().to_le_bytes());
    bytes.extend_from_slice(&receipt.support.end_ns().get().to_le_bytes());
    bytes.extend_from_slice(&receipt.support_duration_ns.to_le_bytes());
    bytes.extend_from_slice(&receipt.lane_id.to_le_bytes());
    stable_monotone_append_bytes_v1(&mut bytes, receipt.ofe_id.as_str().as_bytes());
    bytes.push(u8::from(receipt.ofe_ground_basis));
    bytes.extend_from_slice(receipt.topology_identity_sha256.as_bytes());
    bytes.extend_from_slice(receipt.configuration_identity_sha256.as_bytes());
    bytes.extend_from_slice(receipt.beginning_snow_owner_identity_sha256.as_bytes());
    bytes.extend_from_slice(receipt.beginning_soil_owner_identity_sha256.as_bytes());
    bytes.extend_from_slice(&receipt.bottom_snow_layer_id.to_le_bytes());
    stable_monotone_append_bytes_v1(&mut bytes, receipt.first_soil_layer_id.as_str().as_bytes());
    Ok(bytes)
}

fn covered_terminal_density_constitutive_branch_v1(
    lane_id: u32,
    beginning: &DirectSnowStage3PersistentState,
    inputs: &crate::hydrology::DirectActiveSnowPartitionInputs,
    settle_day_count: f64,
) -> Result<Vec<u8>, DirectV11RealConsumerError> {
    if beginning.lane_id != lane_id
        || beginning.layers.len() > 1
        || !settle_day_count.is_finite()
        || settle_day_count < 0.0
    {
        return Err(DirectV11RealConsumerError::AdaptiveRefinement(
            "stable-monotone density constitutive branch authority",
        ));
    }
    let mut branch = Vec::new();
    branch.extend_from_slice(b"OPENWEPP_COVERED_TERMINAL_DENSITY_CONSTITUTIVE_BRANCH_V1\0");
    branch.extend_from_slice(&lane_id.to_le_bytes());
    stable_monotone_append_bytes_v1(&mut branch, inputs.snow_density_model.id().as_bytes());
    stable_monotone_append_bytes_v1(
        &mut branch,
        inputs.stage3_liquid_routing_model.id().as_bytes(),
    );
    branch.push(beginning.layers.len() as u8);
    branch.extend_from_slice(&settle_day_count.to_bits().to_le_bytes());
    match inputs.sturm_climate_class {
        Some(class) => {
            branch.push(1);
            stable_monotone_append_bytes_v1(&mut branch, class.id().as_bytes());
        }
        None => branch.push(0),
    }
    match inputs.sturm_day_of_year {
        Some(day) => {
            branch.push(1);
            branch.extend_from_slice(&day.to_bits().to_le_bytes());
        }
        None => branch.push(0),
    }
    Ok(branch)
}

fn stable_monotone_v2_carry_coordinates_v1(
    candidate: &DirectSoilThermalCandidate,
) -> Result<(Vec<u8>, Vec<(u64, i8, String, i32)>, Vec<f64>), DirectV11RealConsumerError> {
    let DirectSoilThermalCandidate::V2(candidate) = candidate else {
        return Err(DirectV11RealConsumerError::AdaptiveRefinement(
            "stable-monotone exact V2 soil carry authority",
        ));
    };
    let state = candidate.ending_state();
    state.validate().map_err(|_| {
        DirectV11RealConsumerError::AdaptiveRefinement(
            "stable-monotone exact V2 soil owner validation",
        )
    })?;
    let mut authority = Vec::new();
    authority.extend_from_slice(b"OPENWEPP_COVERED_STABLE_MONOTONE_V2_CARRY_AUTHORITY_V1\0");
    stable_monotone_append_bytes_v1(&mut authority, state.owner_id.as_str().as_bytes());
    stable_monotone_append_bytes_v1(
        &mut authority,
        state.configuration_sha256.as_str().as_bytes(),
    );
    stable_monotone_append_bytes_v1(
        &mut authority,
        openwepp_land_surface_energy::SOIL_THERMAL_OWNER_V2_SCHEMA_SHA256.as_bytes(),
    );
    stable_monotone_append_bytes_v1(
        &mut authority,
        openwepp_land_surface_energy::EXACT_DYADIC_ENTHALPY_V1_DEFINITION_SHA256.as_bytes(),
    );
    authority.extend_from_slice(b"J_M-2_OFE_GROUND\0ORDERED_OFE_LAYER\0");
    let mut wire_coordinates = Vec::new();
    let mut rounded_coordinates = Vec::new();
    for ofe in &state.ofes {
        stable_monotone_append_bytes_v1(&mut authority, ofe.ofe_id.as_str().as_bytes());
        authority.extend_from_slice(&(ofe.ordered_layers.len() as u64).to_le_bytes());
        for (layer_index, layer) in ofe.ordered_layers.iter().enumerate() {
            stable_monotone_append_bytes_v1(&mut authority, layer.layer_id.as_str().as_bytes());
            layer.enthalpy_carry.validate().map_err(|_| {
                DirectV11RealConsumerError::AdaptiveRefinement(
                    "stable-monotone normalized V2 carry wire",
                )
            })?;
            let exact = openwepp_land_surface_energy::ExactDyadicEnthalpy::exact_sum_binary64(
                layer.enthalpy_hi_j_m2_ofe_ground,
                &layer.enthalpy_carry,
                &[],
            )
            .map_err(|_| {
                DirectV11RealConsumerError::AdaptiveRefinement(
                    "stable-monotone exact high-plus-carry reconstruction",
                )
            })?;
            let rounded = exact.round_to_f64().map_err(|_| {
                DirectV11RealConsumerError::AdaptiveRefinement(
                    "stable-monotone finite high-plus-carry coordinate",
                )
            })?;
            if !rounded.is_finite() {
                return Err(DirectV11RealConsumerError::AdaptiveRefinement(
                    "stable-monotone finite high-plus-carry coordinate",
                ));
            }
            wire_coordinates.push((
                layer.enthalpy_hi_j_m2_ofe_ground.to_bits(),
                layer.enthalpy_carry.sign,
                layer.enthalpy_carry.coefficient_hex.clone(),
                layer.enthalpy_carry.exponent2,
            ));
            if layer_index == 0 {
                rounded_coordinates.push(rounded);
            }
        }
    }
    Ok((authority, wire_coordinates, rounded_coordinates))
}

#[allow(clippy::too_many_arguments)]
fn covered_stable_monotone_raw_authentic_map_v1(
    support: openwepp_coupled_time::TimeSupport,
    beginning_stage3: &BTreeMap<u32, DirectSnowStage3PersistentState>,
    iteration_stage3: &BTreeMap<u32, DirectSnowStage3PersistentState>,
    stage3_candidate: &BTreeMap<u32, DirectSnowStage3PersistentState>,
    support_images: &BTreeMap<u32, CoveredExactFloorTerminalPhaseSupportImageV1>,
    receipts: &BTreeMap<u32, SnowSoilHeatReceiptV1>,
    beginning_soil: DirectSoilThermalReadView<'_>,
    iteration_soil: &DirectSoilThermalCandidate,
    soil_candidate: &DirectSoilThermalCandidate,
    lse_configuration: &openwepp_land_surface_energy::LandSurfaceEnergyConfiguration,
    stage3_inputs_by_lane: &BTreeMap<u32, crate::hydrology::DirectActiveSnowPartitionInputs>,
    physical_evaluation_ordinal: usize,
) -> Result<CoveredStableMonotoneRawAuthenticMapV1, DirectV11RealConsumerError> {
    if support.duration_ns()
        < crate::snow_stage3_v11_attachment::STAGE3_V11_ADAPTIVE_MINIMUM_SUPPORT_NS
        || beginning_stage3.keys().ne(iteration_stage3.keys())
        || beginning_stage3.keys().ne(stage3_candidate.keys())
        || beginning_stage3.keys().ne(support_images.keys())
        || beginning_stage3.keys().ne(receipts.keys())
        || beginning_stage3.keys().ne(stage3_inputs_by_lane.keys())
    {
        return Err(DirectV11RealConsumerError::AdaptiveRefinement(
            "stable-monotone support or lane structure",
        ));
    }

    let mut coordinates = Vec::new();
    let mut authentic_seed_coordinates = Vec::new();
    let mut beginning_water = Vec::new();
    let mut beginning_enthalpy = Vec::new();
    let mut delta_water = Vec::new();
    let mut complete_energy = Vec::new();
    let mut physical_ice = Vec::new();
    let mut physical_density = Vec::new();
    let mut physical_thickness = Vec::new();
    let mut exact_density_settling_branch_satisfied = Vec::new();
    let mut phase_branch = Vec::new();
    let mut density_model_branch = Vec::new();
    let mut source_event_topology_custody = Vec::new();
    source_event_topology_custody
        .extend_from_slice(b"OPENWEPP_COVERED_STABLE_MONOTONE_SUPPORT_JOIN_V1\0");
    for (lane_id, beginning) in beginning_stage3 {
        let current = &iteration_stage3[lane_id];
        let candidate = &stage3_candidate[lane_id];
        let image = &support_images[lane_id];
        image.validate().map_err(|_| {
            DirectV11RealConsumerError::AdaptiveRefinement(
                "stable-monotone authentic support image",
            )
        })?;
        if image.support_start_ns != support.start_ns().get()
            || image.support_end_ns != support.end_ns().get()
            || beginning.layers.len() > 1
            || current.layers.len() != 1
            || candidate.layers.len() != 1
        {
            return Err(DirectV11RealConsumerError::AdaptiveRefinement(
                "stable-monotone event-free terminal one-volume support",
            ));
        }
        let current_coordinates = stable_monotone_stage_coordinates_v1(current)?;
        let candidate_coordinates = stable_monotone_stage_coordinates_v1(candidate)?;
        coordinates.extend([
            current_coordinates.0,
            current_coordinates.1,
            current_coordinates.2,
        ]);
        authentic_seed_coordinates.extend([
            candidate_coordinates.0,
            candidate_coordinates.1,
            candidate_coordinates.2,
        ]);
        let beginning_ice = beginning
            .layers
            .iter()
            .map(|layer| 1_000.0 * layer.mass_swe_m)
            .sum::<f64>();
        let beginning_liquid = beginning
            .layers
            .iter()
            .map(|layer| 1_000.0 * layer.liquid_water_m)
            .sum::<f64>();
        let beginning_cold = beginning
            .layers
            .iter()
            .map(|layer| layer.cold_content_j_m2)
            .sum::<f64>();
        beginning_water.push(beginning_ice + beginning_liquid);
        beginning_enthalpy.push(
            crate::hydrology::STAGE3_LATENT_HEAT_FUSION_J_KG * beginning_liquid - beginning_cold,
        );
        delta_water.push(
            image.snowfall_kg_m2 + image.external_liquid_kg_m2 + image.deposition_kg_m2
                - image.sublimation_kg_m2,
        );
        complete_energy.push(image.complete_energy_j_m2);
        physical_ice.push(candidate.layers[0].mass_swe_m * 1_000.0);
        physical_density.push(candidate_coordinates.2);
        physical_thickness.push(candidate.layers[0].thickness_m);
        let current_layer = &current.layers[0];
        let candidate_layer = &candidate.layers[0];
        let density_branch_satisfied =
            current_layer.settle_day_count.to_bits() == candidate_layer.settle_day_count.to_bits();
        exact_density_settling_branch_satisfied.push(density_branch_satisfied);
        if !density_branch_satisfied {
            return Err(DirectV11RealConsumerError::AdaptiveRefinement(
                "stable-monotone density-model branch or settling authority",
            ));
        }
        phase_branch.push(if current_coordinates.1 <= 0.0 {
            0
        } else if current_coordinates.1
            < crate::hydrology::STAGE3_LATENT_HEAT_FUSION_J_KG * current_coordinates.0
        {
            1
        } else {
            2
        });
        phase_branch.extend_from_slice(&current_layer.settle_day_count.to_bits().to_le_bytes());
        density_model_branch.extend_from_slice(&covered_terminal_density_constitutive_branch_v1(
            *lane_id,
            beginning,
            &stage3_inputs_by_lane[lane_id],
            current_layer.settle_day_count,
        )?);
        source_event_topology_custody.extend_from_slice(&lane_id.to_le_bytes());
        for value in [
            image.parent_start_ns,
            image.parent_end_ns,
            image.support_start_ns,
            image.support_end_ns,
        ] {
            source_event_topology_custody.extend_from_slice(&value.to_le_bytes());
        }
        for fingerprint in image.source_receipt_fingerprints {
            source_event_topology_custody.extend_from_slice(&fingerprint.to_le_bytes());
        }
    }

    let mut static_receipt_joins = Vec::new();
    let mut physical_receipt_digests = Vec::new();
    for receipt in receipts.values() {
        static_receipt_joins.push(stable_monotone_receipt_static_join_v1(receipt)?);
        physical_receipt_digests.push(receipt.receipt_sha256);
    }
    let (carry_authority, evolving_carry_coordinate_bits, iteration_soil_enthalpy) =
        stable_monotone_v2_carry_coordinates_v1(iteration_soil)?;
    let (candidate_carry_authority, _, candidate_soil_enthalpy) =
        stable_monotone_v2_carry_coordinates_v1(soil_candidate)?;
    if carry_authority != candidate_carry_authority {
        return Err(DirectV11RealConsumerError::AdaptiveRefinement(
            "stable-monotone V2 carry authority or representation change",
        ));
    }
    let beginning_ofes = beginning_soil.ordered_ofes();
    let iteration_ofes = iteration_soil.read_view().ordered_ofes();
    let candidate_ofes = soil_candidate.read_view().ordered_ofes();
    if beginning_ofes.len() != iteration_ofes.len()
        || beginning_ofes.len() != candidate_ofes.len()
        || iteration_soil_enthalpy.len() != beginning_ofes.len()
        || candidate_soil_enthalpy.len() != beginning_ofes.len()
    {
        return Err(DirectV11RealConsumerError::AdaptiveRefinement(
            "stable-monotone ordered soil coordinate structure",
        ));
    }
    let mut beginning_soil_enthalpy = Vec::new();
    let mut physical_soil_delta = Vec::new();
    let mut owner_soil_temperature = Vec::new();
    for (index, beginning_ofe) in beginning_ofes.iter().enumerate() {
        let beginning_top = beginning_ofe.ordered_layers().into_iter().next().ok_or(
            DirectV11RealConsumerError::AdaptiveRefinement(
                "stable-monotone beginning soil coordinate",
            ),
        )?;
        let iteration_top = iteration_ofes[index]
            .ordered_layers()
            .into_iter()
            .next()
            .ok_or(DirectV11RealConsumerError::AdaptiveRefinement(
                "stable-monotone iteration soil coordinate",
            ))?;
        let configured_top = lse_configuration
            .ofes
            .iter()
            .find(|ofe| &ofe.ofe_id == beginning_ofe.ofe_id())
            .and_then(|ofe| ofe.soil_interface_layers.first())
            .ok_or(DirectV11RealConsumerError::AdaptiveRefinement(
                "stable-monotone configured soil coordinate",
            ))?;
        let beginning_energy = covered_soil_layer_enthalpy_coordinate_v1(beginning_top)?;
        coordinates.extend([
            iteration_soil_enthalpy[index],
            iteration_top.temperature_k(),
        ]);
        let candidate_top = candidate_ofes[index]
            .ordered_layers()
            .into_iter()
            .next()
            .ok_or(DirectV11RealConsumerError::AdaptiveRefinement(
                "stable-monotone candidate soil coordinate",
            ))?;
        authentic_seed_coordinates.extend([
            candidate_soil_enthalpy[index],
            candidate_top.temperature_k(),
        ]);
        beginning_soil_enthalpy.push(beginning_energy);
        physical_soil_delta.push(candidate_soil_enthalpy[index] - beginning_energy);
        owner_soil_temperature.push(
            beginning_top.temperature_k()
                + (iteration_soil_enthalpy[index] - beginning_energy)
                    / configured_top.areal_heat_capacity_j_m2_k,
        );
    }
    let mut tolerances = Vec::new();
    for _ in beginning_stage3 {
        let lane = tolerances.len() / 3;
        let phase = phase_consistent_canonical_phase_projection_v1(
            coordinates[3 * lane],
            coordinates[3 * lane + 1],
            coordinates[3 * lane + 2],
        )
        .map_err(|_| {
            DirectV11RealConsumerError::AdaptiveRefinement(
                "stable-monotone canonical density geometry",
            )
        })?;
        let geometry = CoveredTerminalDensityGeometryCoordinateV1::from_canonical_phase(&phase)
            .map_err(|_| {
                DirectV11RealConsumerError::AdaptiveRefinement(
                    "stable-monotone canonical density geometry",
                )
            })?;
        tolerances.extend([
            COVERED_FIXED_POINT_POLICY.mass_abs_kg_m2,
            COVERED_FIXED_POINT_POLICY.energy_abs_j_m2,
            geometry.density_absolute_tolerance_kg_m3(),
        ]);
    }
    for _ in &beginning_ofes {
        tolerances.extend([
            COVERED_FIXED_POINT_POLICY.energy_abs_j_m2,
            COVERED_FIXED_POINT_POLICY.state_temperature_abs_k,
        ]);
    }
    let residual =
        covered_phase_consistent_residual_assemble_v1(CoveredPhaseConsistentResidualInputsV1 {
            coordinates,
            beginning_snow_water_kg_m2: beginning_water,
            beginning_snow_enthalpy_j_m2: beginning_enthalpy,
            physical_delta_water_kg_m2: delta_water,
            physical_complete_energy_j_m2: complete_energy,
            physical_ice_kg_m2: physical_ice,
            physical_density_kg_m3: physical_density,
            physical_thickness_m: physical_thickness,
            exact_density_settling_branch_satisfied,
            beginning_soil_enthalpy_j_m2: beginning_soil_enthalpy,
            physical_soil_delta_energy_j_m2: physical_soil_delta,
            owner_soil_temperature_k: owner_soil_temperature,
            absolute_tolerances: tolerances,
            algebraic_side_constraints_satisfied: true,
        })
        .map_err(|_| {
            DirectV11RealConsumerError::AdaptiveRefinement(
                "stable-monotone physical residual reconstruction",
            )
        })?;
    Ok(CoveredStableMonotoneRawAuthenticMapV1 {
        static_joins: CoveredStableMonotoneStaticJoinsV1 {
            support_start_ns: support.start_ns().get(),
            support_end_ns: support.end_ns().get(),
            source_event_topology_custody,
            static_receipt_joins,
            phase_branch,
            density_model_branch,
            carry_authority_and_representation: carry_authority,
        },
        physical_receipt_digests,
        evolving_carry_coordinate_bits,
        residual,
        authentic_seed_coordinates,
        physical_evaluation_ordinal,
        event_free_terminal_one_volume: true,
        exact_carry_reconstruction_satisfied: true,
        active_set_transition: false,
        finalization_restart: false,
        publication_eligible: false,
    })
}
