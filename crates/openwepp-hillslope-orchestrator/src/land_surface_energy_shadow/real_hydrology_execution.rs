type SoilThermalEnergyOperandV2 = openwepp_land_surface_energy::SoilThermalAcceptedEnergyOperandV2;

/// Join one immutable LSE request batch to both actual water owners.
#[allow(clippy::too_many_lines)]
pub(crate) fn execute_unified_real_hydrology_shadow<F>(
    soil_adapter: &LandSurfaceEnergyRealHydrologyAdapter<'_>,
    surface_configuration: &DirectSurfaceLiquidConfiguration,
    receiver_expectations: &UnifiedReceiverExpectations,
    request_batch: &PotentialWaterRequestBatch,
    soil_sources: &BTreeMap<GroundWaterKey, RealHydrologySourceKey>,
    ingress: &DirectSurfaceLiquidIngressInput,
    finalize_fixed_caps: F,
) -> Result<UnifiedRealHydrologyCandidate, LandSurfaceEnergyShadowError>
where
    F: FnOnce(
        &[WaterAuthorization],
    ) -> Result<UnifiedLseFinalization, LandSurfaceEnergyShadowError>,
{
    let expected_beginning_hydrology_snapshot_sha256 =
        &receiver_expectations.beginning_hydrology_snapshot_sha256;
    let unified_entry_preflight::UnifiedEntryPreflight {
        actual_snapshot,
        attempted_sha256,
        soil_requests,
        surface_requests,
    } = unified_entry_preflight::validate_unified_entry(
        soil_adapter,
        surface_configuration,
        receiver_expectations,
        request_batch,
        soil_sources,
        ingress,
        expected_beginning_hydrology_snapshot_sha256,
    )?;
    let beginning_surface = soil_adapter
        .owner
        .beginning_frame()
        .surface_liquid_shadow
        .as_deref()
        .ok_or(LandSurfaceEnergyShadowError::Identity(
            "missing beginning surface-liquid owner",
        ))?;
    let soil = soil_adapter.authorize(&soil_requests).map_err(|error| {
        unified_entry_preflight::complete_unified_failure(
            canonicalize_unified_error(
                error,
                request_batch,
                expected_beginning_hydrology_snapshot_sha256,
            ),
            &actual_snapshot,
            &attempted_sha256,
        )
    })?;
    let surface = authorize_surface_liquid_withdrawals(
        surface_configuration,
        beginning_surface,
        request_batch.transaction_id,
        beginning_surface
            .records
            .first()
            .and_then(|record| record.last_accepted_transaction_id),
        &surface_requests,
    )
    .map_err(|error| {
        unified_entry_preflight::complete_unified_failure(
            canonicalize_unified_error(
                error.into(),
                request_batch,
                expected_beginning_hydrology_snapshot_sha256,
            ),
            &actual_snapshot,
            &attempted_sha256,
        )
    })?;
    let authorizations = restore_authorization_order(
        request_batch,
        &soil,
        &surface,
        expected_beginning_hydrology_snapshot_sha256,
    )
    .map_err(|error| {
        unified_entry_preflight::complete_unified_failure(
            error,
            &actual_snapshot,
            &attempted_sha256,
        )
    })?;
    let arbitration = UnifiedRealHydrologyArbitration {
        transaction_id: request_batch.transaction_id,
        requests: request_batch.requests.clone(),
        authorizations,
        soil,
        surface: Some(surface),
    };
    let finalized = finalize_fixed_caps(&arbitration.authorizations).map_err(|error| {
        unified_entry_preflight::canonicalize_callback_failure(
            &error,
            request_batch.transaction_id,
            &actual_snapshot,
            &attempted_sha256,
        )
    })?;
    validate_final_protocol(
        &finalized.water_protocol,
        &arbitration,
        expected_beginning_hydrology_snapshot_sha256,
        &surface_configuration.owner_id,
    )?;
    let finalized_protocol = finalized.water_protocol().clone();
    construct_unified_candidate(
        soil_adapter,
        surface_configuration,
        receiver_expectations,
        request_batch,
        arbitration,
        finalized,
        ingress,
        true,
        None,
        None,
    )
    .map_err(|error| canonicalize_finalized_error(error, &finalized_protocol))
}

fn validate_native_shadow_supported_domain(
    owner: &RealHydrologyShadowAdapter,
    configuration: &DirectSurfaceLiquidConfiguration,
    beginning_hydrology_snapshot_sha256: &Sha256Digest,
    attempted_sha256: &str,
) -> Result<(), LandSurfaceEnergyShadowError> {
    if let Some(lane_index) = owner.beginning_frame().lanes.iter().position(|lane| {
        crate::direct_runtime::validate_direct_production_winter_lane_domain(lane).is_err()
            || !frost_indices_fit_production_layers(lane)
    }) {
        return Err(DirectSurfaceLiquidError::canonical_failure(
            DirectSurfaceLiquidErrorCode::E003,
            DirectSurfaceLiquidPhase::AtomicEnvelope,
            first_lane_error_context(owner, configuration, lane_index),
            DirectSurfaceLiquidRollbackHashes {
                beginning_owner_sha256: Some(beginning_hydrology_snapshot_sha256.to_string()),
                attempted_owner_sha256: Some(attempted_sha256.to_owned()),
            },
            "nonfinite or negative production snow lane scalar",
        )
        .into());
    }
    if let Some(lane_index) = owner
        .beginning_frame()
        .lanes
        .iter()
        .position(lane_has_unsupported_frozen_or_snow_state)
    {
        return Err(DirectSurfaceLiquidError::canonical_failure(
            DirectSurfaceLiquidErrorCode::E004,
            DirectSurfaceLiquidPhase::AtomicEnvelope,
            first_lane_error_context(owner, configuration, lane_index),
            DirectSurfaceLiquidRollbackHashes {
                beginning_owner_sha256: Some(beginning_hydrology_snapshot_sha256.to_string()),
                attempted_owner_sha256: Some(attempted_sha256.to_owned()),
            },
            "snow, terminal snow, frozen, or thawing production frame",
        )
        .into());
    }
    Ok(())
}

fn frost_indices_fit_production_layers(lane: &crate::direct_runtime::DirectLaneFrame) -> bool {
    let layer_count = lane.subsurface_layers.len();
    let winter = &lane.winter_column.frost;
    let winter_fits = winter
        .layer_shadows
        .iter()
        .all(|layer| (1..=layer_count).contains(&layer.layer_index))
        && winter
            .fine_layers
            .iter()
            .all(|layer| (1..=layer_count).contains(&layer.layer_index));
    let carry_fits = lane.frost_runtime_carry.as_ref().is_none_or(|carry| {
        carry
            .layer_shadows
            .iter()
            .all(|layer| (1..=layer_count).contains(&layer.layer_index))
            && carry
                .fine_layers
                .iter()
                .all(|layer| (1..=layer_count).contains(&layer.layer_index))
    });
    winter_fits && carry_fits
}

fn validate_native_shadow_exact_one_custody(
    owner: &RealHydrologyShadowAdapter,
    configuration: &DirectSurfaceLiquidConfiguration,
    beginning_hydrology_snapshot_sha256: &Sha256Digest,
    attempted_sha256: &str,
) -> Result<(), LandSurfaceEnergyShadowError> {
    let legacy_custody = owner.beginning_day_frames().iter().position(|day| {
        day.infiltration_depression_inputs
            .depression_storage_delta_handoff_m
            .to_bits()
            != 0.0_f64.to_bits()
            || day
                .infiltration_depression_inputs
                .producer_inputs
                .as_ref()
                .is_some_and(|inputs| {
                    inputs.depression_storage_capacity_m.to_bits() != 0.0_f64.to_bits()
                })
            || day
                .infiltration_depression
                .depression_storage_delta_m
                .to_bits()
                != 0.0_f64.to_bits()
    });
    if let Some(lane_index) = legacy_custody {
        return Err(DirectSurfaceLiquidError::exact_one_owner_failure(
            DirectSurfaceLiquidPhase::AtomicEnvelope,
            first_lane_error_context(owner, configuration, lane_index),
            Some(beginning_hydrology_snapshot_sha256.to_string()),
            Some(attempted_sha256.to_owned()),
            "legacy infiltration/depression liquid custody is nonzero",
        )
        .into());
    }
    Ok(())
}

fn lane_has_unsupported_frozen_or_snow_state(
    lane: &crate::direct_runtime::DirectLaneFrame,
) -> bool {
    lane.winter_column.snow.has_runtime_state()
        || lane.winter_column.snow.liquid_water_retained_m > 0.0
        || lane.snow_runtime_carry.is_some()
        || lane.winter_column.frost.has_runtime_state()
        || lane.frost_runtime_carry.is_some()
        || lane
            .subsurface_layers
            .iter()
            .any(|layer| layer.frozen_depth_m > 0.0 || layer.frozen_water_m > 0.0)
}

fn first_lane_error_context(
    owner: &RealHydrologyShadowAdapter,
    configuration: &DirectSurfaceLiquidConfiguration,
    lane_index: usize,
) -> DirectSurfaceLiquidErrorContext {
    let binding = configuration
        .ofe_bindings
        .iter()
        .find(|binding| binding.production_lane_index == lane_index);
    let record = binding.and_then(|binding| {
        configuration
            .records
            .iter()
            .find(|record| record.key.ofe_id == binding.ofe_id)
    });
    DirectSurfaceLiquidErrorContext {
        transaction_id: Some(owner.transaction_id()),
        owner_id: Some(owner.hydrology_owner_id().clone()),
        ofe_id: binding.map(|binding| binding.ofe_id.clone()),
        tile_id: record.map(|record| record.key.tile_id.clone()),
        surface_id: record.map(|record| record.key.surface_id.clone()),
        source_id: record.map(|record| record.key.source_id.clone()),
        ..DirectSurfaceLiquidErrorContext::default()
    }
}

#[allow(clippy::too_many_lines)]
pub(super) fn validate_receiver_expectations(
    owner: &RealHydrologyShadowAdapter,
    configuration: &DirectSurfaceLiquidConfiguration,
    expectations: &UnifiedReceiverExpectations,
    request_batch: &PotentialWaterRequestBatch,
    beginning_hydrology_snapshot_sha256: &Sha256Digest,
) -> Result<(), LandSurfaceEnergyShadowError> {
    let lse_owners = request_batch
        .requests
        .iter()
        .filter(|request| request.key.requesting_component == RequestingComponent::GroundSurface)
        .map(|request| request.key.requesting_owner_id.clone())
        .collect::<BTreeSet<_>>();
    let expected_tiles = configuration
        .records
        .iter()
        .map(|record| (record.key.ofe_id.clone(), record.key.tile_id.clone()))
        .collect::<Vec<_>>();
    let thermal_tiles = expectations
        .ordered_thermal_layers
        .iter()
        .map(|(identity, _)| identity.clone())
        .collect::<Vec<_>>();
    if request_batch.beginning_lse_state_sha256 != expectations.beginning_lse_state_sha256
        || lse_owners.len() != 1
        || !lse_owners.contains(&expectations.lse_owner_id)
    {
        return Err(DirectSurfaceLiquidError::atomic_envelope_failure(
            DirectSurfaceLiquidErrorContext {
                transaction_id: Some(owner.transaction_id()),
                owner_id: Some(expectations.lse_owner_id.clone()),
                ..DirectSurfaceLiquidErrorContext::default()
            },
            Some(beginning_hydrology_snapshot_sha256.to_string()),
            Some(receiver_expectations_sha256(expectations)),
            "independent LSE receiver expectations",
        )
        .into());
    }
    if owner.hydrology_owner_id() != &expectations.hydrology_owner_id
        || beginning_hydrology_snapshot_sha256 != &expectations.beginning_hydrology_snapshot_sha256
    {
        return Err(DirectSurfaceLiquidError::atomic_envelope_failure(
            DirectSurfaceLiquidErrorContext {
                transaction_id: Some(owner.transaction_id()),
                owner_id: Some(expectations.hydrology_owner_id.clone()),
                ..DirectSurfaceLiquidErrorContext::default()
            },
            Some(beginning_hydrology_snapshot_sha256.to_string()),
            Some(receiver_expectations_sha256(expectations)),
            "independent hydrology receiver expectations",
        )
        .into());
    }
    if expectations.soil_thermal_owner_id == expectations.lse_owner_id
        || expectations.soil_thermal_owner_id == expectations.hydrology_owner_id
        || expectations.beginning_soil_thermal_state_sha256
            == expectations.beginning_lse_state_sha256
        || expectations.beginning_soil_thermal_state_sha256
            == expectations.beginning_hydrology_snapshot_sha256
    {
        return Err(DirectSurfaceLiquidError::atomic_envelope_failure(
            DirectSurfaceLiquidErrorContext {
                transaction_id: Some(owner.transaction_id()),
                owner_id: Some(expectations.soil_thermal_owner_id.clone()),
                ..DirectSurfaceLiquidErrorContext::default()
            },
            Some(beginning_hydrology_snapshot_sha256.to_string()),
            Some(receiver_expectations_sha256(expectations)),
            "independent soil-thermal receiver expectation lineage",
        )
        .into());
    }
    if let Some(violation) = first_expected_identity_violation(
        &expected_tiles,
        &thermal_tiles,
        OwnerKind::SoilThermal,
        &expectations.soil_thermal_owner_id,
        "independent soil-thermal receiver expectation topology",
    ) {
        return Err(DirectSurfaceLiquidError::atomic_envelope_failure(
            DirectSurfaceLiquidErrorContext {
                transaction_id: Some(owner.transaction_id()),
                owner_id: violation.owner_id,
                ofe_id: violation.ofe_id,
                tile_id: violation.tile_id,
                ..DirectSurfaceLiquidErrorContext::default()
            },
            Some(beginning_hydrology_snapshot_sha256.to_string()),
            Some(receiver_expectations_sha256(expectations)),
            violation.detail,
        )
        .into());
    }
    for ((ofe_id, tile_id), layers) in &expectations.ordered_thermal_layers {
        let configured_infiltration_layer = configuration
            .ofe_bindings
            .iter()
            .find(|binding| &binding.ofe_id == ofe_id)
            .map(|binding| &binding.infiltration_soil_thermal_layer_id);
        if configured_infiltration_layer != layers.first() {
            return Err(DirectSurfaceLiquidError::atomic_envelope_failure(
                DirectSurfaceLiquidErrorContext {
                    transaction_id: Some(owner.transaction_id()),
                    owner_id: Some(expectations.soil_thermal_owner_id.clone()),
                    ofe_id: Some(ofe_id.clone()),
                    tile_id: Some(tile_id.clone()),
                    ..DirectSurfaceLiquidErrorContext::default()
                },
                Some(beginning_hydrology_snapshot_sha256.to_string()),
                Some(receiver_expectations_sha256(expectations)),
                "soil-thermal infiltration layer is not the configured first layer",
            )
            .into());
        }
    }
    Ok(())
}

fn validate_final_protocol(
    protocol: &WaterProtocol,
    arbitration: &UnifiedRealHydrologyArbitration,
    expected_snapshot: &Sha256Digest,
    expected_owner: &openwepp_kernel_contract::ResourceOwnerId,
) -> Result<(), LandSurfaceEnergyShadowError> {
    let attempted_sha256 = water_protocol_sha256(protocol);
    if protocol.transaction_id != arbitration.transaction_id
        || &protocol.hydrology_owner_id != expected_owner
        || &protocol.beginning_snapshot_sha256 != expected_snapshot
        || protocol.requests != arbitration.requests
        || protocol.authorizations != arbitration.authorizations
    {
        return Err(protocol_failure(
            DirectSurfaceLiquidErrorCode::E002,
            protocol,
            expected_snapshot,
            &attempted_sha256,
            "final water protocol lineage or D/A identity",
        ));
    }
    preflight_protocol_identities(protocol, expected_snapshot, &attempted_sha256)?;
    preflight_protocol_domains(protocol, expected_snapshot, &attempted_sha256)?;
    preflight_protocol_cardinality(protocol, expected_snapshot, &attempted_sha256)?;
    preflight_protocol_bounds(protocol, expected_snapshot, &attempted_sha256)?;
    Ok(())
}

pub(super) fn partition_requests(
    batch: &PotentialWaterRequestBatch,
    soil_sources: &BTreeMap<GroundWaterKey, RealHydrologySourceKey>,
    configuration: &DirectSurfaceLiquidConfiguration,
    beginning_sha256: &Sha256Digest,
) -> Result<(Vec<MixedRealHydrologyRequest>, Vec<WaterAmount>), LandSurfaceEnergyShadowError> {
    let mut soil = Vec::new();
    let mut surface = Vec::new();
    let mut consumed_soil_keys = BTreeSet::new();
    for request in &batch.requests {
        match request.key.source_type {
            WaterSourceType::SoilLayerLiquid => {
                let source = soil_sources.get(&request.key).ok_or_else(|| {
                    request_failure(
                        DirectSurfaceLiquidErrorCode::E002,
                        batch,
                        beginning_sha256,
                        Some(&request.key),
                        "missing soil source mapping",
                    )
                })?;
                if request.key.soil_layer_id.as_ref() != Some(&source.layer_id) {
                    return Err(request_failure(
                        DirectSurfaceLiquidErrorCode::E002,
                        batch,
                        beginning_sha256,
                        Some(&request.key),
                        "mixed source identity",
                    ));
                }
                let binding = configuration
                    .ofe_bindings
                    .iter()
                    .find(|binding| binding.ofe_id == request.key.ofe_id);
                if !binding.is_some_and(|binding| {
                    source.ofe_lane.lane_index == binding.production_lane_index
                        && source.ofe_lane.lane_id == binding.production_lane_id
                        && binding.ordered_soil_layer_ids.contains(&source.layer_id)
                }) {
                    return Err(request_failure(
                        DirectSurfaceLiquidErrorCode::E002,
                        batch,
                        beginning_sha256,
                        Some(&request.key),
                        "soil source is not an exact configured OFE/lane/layer member",
                    ));
                }
                consumed_soil_keys.insert(request.key.clone());
                soil.push(MixedRealHydrologyRequest {
                    request: request.clone(),
                    source: source.clone(),
                });
            }
            WaterSourceType::SurfaceLiquid | WaterSourceType::LitterLiquid => {
                if soil_sources.contains_key(&request.key) {
                    return Err(request_failure(
                        DirectSurfaceLiquidErrorCode::E002,
                        batch,
                        beginning_sha256,
                        Some(&request.key),
                        "surface request has soil mapping",
                    ));
                }
                let exact_configured_store = configuration.records.iter().any(|record| {
                    request.key.requesting_tile_id == record.key.tile_id
                        && request.key.ofe_id == record.key.ofe_id
                        && request.key.source_tile_id.as_ref() == Some(&record.key.tile_id)
                        && request.key.surface_id.as_ref() == Some(&record.key.surface_id)
                        && request.key.surface_class == Some(record.key.surface_class)
                        && request.key.source_type == record.key.source_type
                        && request.key.source_id == record.key.source_id
                });
                if !exact_configured_store {
                    return Err(request_failure(
                        DirectSurfaceLiquidErrorCode::E002,
                        batch,
                        beginning_sha256,
                        Some(&request.key),
                        "surface request has no exact configured store",
                    ));
                }
                surface.push(request.clone());
            }
        }
    }
    if consumed_soil_keys.len() != soil_sources.len() {
        return Err(request_failure(
            DirectSurfaceLiquidErrorCode::E002,
            batch,
            beginning_sha256,
            None,
            "unused soil source mapping",
        ));
    }
    Ok((soil, surface))
}

fn restore_authorization_order(
    batch: &PotentialWaterRequestBatch,
    soil: &MixedRealHydrologyArbitration,
    surface: &DirectSurfaceLiquidArbitration,
    beginning_sha256: &Sha256Digest,
) -> Result<Vec<WaterAuthorization>, LandSurfaceEnergyShadowError> {
    let by_key = soil
        .authorizations
        .iter()
        .map(|row| (row.authorization.key.clone(), row.authorization.clone()))
        .chain(
            surface
                .authorizations()
                .iter()
                .map(|row| (row.key.clone(), row.clone())),
        )
        .collect::<BTreeMap<_, _>>();
    if by_key.len() != batch.requests.len() {
        return Err(request_failure(
            DirectSurfaceLiquidErrorCode::E005,
            batch,
            beginning_sha256,
            None,
            "incomplete unified authorization",
        ));
    }
    batch
        .requests
        .iter()
        .map(|request| {
            by_key.get(&request.key).cloned().ok_or_else(|| {
                request_failure(
                    DirectSurfaceLiquidErrorCode::E002,
                    batch,
                    beginning_sha256,
                    Some(&request.key),
                    "authorization order identity",
                )
            })
        })
        .collect()
}

fn construct_unified_candidate(
    soil_adapter: &LandSurfaceEnergyRealHydrologyAdapter<'_>,
    surface_configuration: &DirectSurfaceLiquidConfiguration,
    receiver_expectations: &UnifiedReceiverExpectations,
    request_batch: &PotentialWaterRequestBatch,
    arbitration: UnifiedRealHydrologyArbitration,
    finalized: UnifiedLseFinalization,
    ingress: &DirectSurfaceLiquidIngressInput,
    finalize_wb14_parent_interval: bool,
    wb14_parent_working_state: Option<&DirectWb14ParentWorkingState>,
    wb14_coupled_child_binding: Option<crate::direct_runtime::DirectWb14CoupledChildBindingV1>,
) -> Result<UnifiedRealHydrologyCandidate, LandSurfaceEnergyShadowError> {
    use crate::snow_stage3_v11_attachment::{
        begin_adaptive_parent_fixed_point_phase_v1 as profile_start,
        record_adaptive_parent_profile_detail_v1 as profile_record,
    };

    let UnifiedLseFinalization {
        water_protocol,
        mut ending_tile_states_pre_ingress,
        mut soil_thermal_candidates,
        rollback_hashes,
    } = finalized;
    let soil_started = profile_start();
    let (soil_uses, surface_uses) =
        partition_finalized_uses(&arbitration, &water_protocol.finalized_uses)?;
    let soil_candidate =
        soil_adapter.candidate_from_finalized_uses(&arbitration.soil, &soil_uses)?;
    profile_record("candidate soil", soil_started);
    let surface_resource_started = profile_start();
    let surface_arbitration =
        arbitration
            .surface
            .as_ref()
            .ok_or(LandSurfaceEnergyShadowError::Identity(
                "missing legacy surface arbitration",
            ))?;
    let surface_resource = apply_surface_liquid_resource_phase(
        surface_configuration,
        surface_arbitration,
        &surface_uses,
        &water_protocol.condensation_credits,
    )?;
    profile_record("candidate surface resource", surface_resource_started);
    let surface_ingress_started = profile_start();
    let surface_ingress = execute_surface_liquid_ingress_with_parent_state_and_coupled_binding(
        surface_configuration,
        &surface_resource,
        ingress,
        wb14_parent_working_state,
        finalize_wb14_parent_interval,
        wb14_coupled_child_binding,
    )?;
    profile_record("candidate surface ingress", surface_ingress_started);
    let receivers_started = profile_start();
    let mut ending_frame = soil_candidate.ending_frame().clone();
    let pre_ingress_soil_thermal_candidates = soil_thermal_candidates.clone();
    let pre_ingress_soil_thermal_sha256 =
        finalization_receiver_sets_sha256(&[], &pre_ingress_soil_thermal_candidates, &[]);
    let receiver_closure_operands = apply_ingress_to_real_receivers(
        soil_adapter.owner,
        surface_configuration,
        receiver_expectations,
        request_batch,
        &surface_ingress,
        &mut ending_frame,
        &mut ending_tile_states_pre_ingress,
        &mut soil_thermal_candidates,
        &rollback_hashes,
        &water_protocol.beginning_snapshot_sha256,
        false,
    )?;
    profile_record("candidate receivers", receivers_started);
    let validation_started = profile_start();
    ending_frame.surface_liquid_shadow = Some(Box::new(surface_ingress.ending_state().clone()));
    let candidate = UnifiedRealHydrologyCandidate {
        transaction_id: arbitration.transaction_id,
        beginning_frame: soil_candidate.beginning_frame().clone(),
        ending_frame,
        arbitration,
        finalized_uses: water_protocol.finalized_uses,
        condensation_credits: water_protocol.condensation_credits,
        surface_resource,
        surface_ingress,
        ending_lse_tile_states: ending_tile_states_pre_ingress,
        pre_ingress_soil_thermal_candidates,
        pre_ingress_soil_thermal_sha256,
        soil_thermal_candidates,
        receiver_closure_operands,
        rollback_hashes,
    };
    candidate.validate(surface_configuration)?;
    profile_record("candidate validation", validation_started);
    Ok(candidate)
}

/// Join an already accepted native frozen-litter V3 transaction to the
/// production soil and receiver owners without re-running either surface
/// authorization, the covered solver, phase physics, or WB14. The V1-shaped
/// surface values retained here are arithmetic carriers from the accepted V2
/// owner transaction; they are never published as an owner downgrade.
#[allow(clippy::too_many_lines)]
pub(crate) fn construct_v3_unified_hydrology_candidate(
    soil_adapter: &LandSurfaceEnergyRealHydrologyAdapter<'_>,
    surface_configuration: &DirectSurfaceLiquidConfiguration,
    fixed: &v3_multitile_adoption::V3MultiTileAcceptedFixedFinalCandidate,
    accepted: &v3_execution::AcceptedFrozenLitterV3RuntimeCandidate,
    exact_surface_custody: bool,
) -> Result<UnifiedRealHydrologyCandidate, LandSurfaceEnergyShadowError> {
    let request_batch = PotentialWaterRequestBatch::try_new(
        fixed.water_protocol.transaction_id,
        fixed
            .receiver_expectations
            .beginning_lse_state_sha256
            .clone(),
        fixed.water_protocol.requests.clone(),
    )?;
    let arbitration = UnifiedRealHydrologyArbitration {
        transaction_id: fixed.water_protocol.transaction_id,
        requests: fixed.water_protocol.requests.clone(),
        authorizations: fixed.water_protocol.authorizations.clone(),
        soil: fixed.soil_arbitration.clone(),
        surface: None,
    };
    validate_final_protocol(
        &fixed.water_protocol,
        &arbitration,
        &fixed
            .receiver_expectations
            .beginning_hydrology_snapshot_sha256,
        &surface_configuration.owner_id,
    )?;
    let finalized = fixed.unified_finalization(&accepted.ending_lse_state)?;
    let UnifiedLseFinalization {
        water_protocol,
        mut ending_tile_states_pre_ingress,
        mut soil_thermal_candidates,
        rollback_hashes,
    } = finalized;
    let (soil_uses, surface_uses) =
        partition_finalized_uses(&arbitration, &water_protocol.finalized_uses)?;

    // Every surface row on this seam must be the exact aggregate view of one
    // named frozen-litter phase receipt. Ordinary surface-liquid rows require
    // their own V2 application and therefore fail closed here.
    let mut matched_phase_keys = BTreeSet::new();
    for receipt in &accepted.litter_phase_receipts {
        let fixed_tile = fixed
            .frozen_litter_tiles
            .iter()
            .find(|tile| {
                tile.fixed_final.identity.ofe_id == receipt.identity.ofe_id
                    && tile.fixed_final.identity.tile_id == receipt.identity.tile_id
            })
            .ok_or(LandSurfaceEnergyShadowError::Identity(
                "V3 phase receipt/fixed-final tile",
            ))?;
        let row = surface_uses
            .iter()
            .find(|row| {
                row.key.ofe_id == receipt.identity.ofe_id
                    && row.key.requesting_tile_id == receipt.identity.tile_id
                    && row.key.source_type == WaterSourceType::LitterLiquid
            })
            .ok_or(LandSurfaceEnergyShadowError::UnsupportedCustody(
                "V3 phase receipt has no exact litter finalized use",
            ))?;
        let expected = openwepp_land_surface_energy::V3PhaseSpecificVaporAuthorization {
            liquid_outbound_rate_kg_m2_s: receipt
                .vapor
                .finalized
                .liquid_signed_rate_kg_m2_s
                .max(0.0),
            ice_outbound_rate_kg_m2_s: receipt.vapor.finalized.ice_signed_rate_kg_m2_s.max(0.0),
        }
        .aggregate_outbound_kg_m2_stand_ground(
            fixed_tile.fixed_final.identity.tile_fraction,
            f64::from_bits(receipt.identity.support_duration_seconds_bits),
        )?;
        if row.amount_kg_m2_stand_ground.to_bits() != expected.to_bits() {
            return Err(LandSurfaceEnergyShadowError::Identity(
                "V3 phase receipt/finalized use amount",
            ));
        }
        matched_phase_keys.insert(row.key.clone());
    }
    if surface_uses.iter().any(|row| {
        !matched_phase_keys.contains(&row.key)
            && row.amount_kg_m2_stand_ground.to_bits() != 0.0_f64.to_bits()
    }) {
        return Err(LandSurfaceEnergyShadowError::UnsupportedCustody(
            "V3 finalized uses include a nonzero ordinary surface row",
        ));
    }

    let soil_candidate =
        soil_adapter.candidate_from_finalized_uses(&arbitration.soil, &soil_uses)?;
    let surface_resource = accepted.surface_resource.liquid_arithmetic().clone();
    let surface_ingress = accepted.ingress.inner().clone();
    let mut ending_frame = soil_candidate.ending_frame().clone();
    let pre_ingress_soil_thermal_candidates = soil_thermal_candidates.clone();
    let pre_ingress_soil_thermal_sha256 =
        finalization_receiver_sets_sha256(&[], &pre_ingress_soil_thermal_candidates, &[]);
    let receiver_closure_operands = apply_ingress_to_real_receivers(
        soil_adapter.owner,
        surface_configuration,
        &fixed.receiver_expectations,
        &request_batch,
        &surface_ingress,
        &mut ending_frame,
        &mut ending_tile_states_pre_ingress,
        &mut soil_thermal_candidates,
        &rollback_hashes,
        &water_protocol.beginning_snapshot_sha256,
        exact_surface_custody,
    )?;
    ending_frame.surface_liquid_shadow = Some(Box::new(surface_ingress.ending_state().clone()));
    let candidate = UnifiedRealHydrologyCandidate {
        transaction_id: arbitration.transaction_id,
        beginning_frame: soil_candidate.beginning_frame().clone(),
        ending_frame,
        arbitration,
        finalized_uses: water_protocol.finalized_uses,
        condensation_credits: water_protocol.condensation_credits,
        surface_resource,
        surface_ingress,
        ending_lse_tile_states: ending_tile_states_pre_ingress,
        pre_ingress_soil_thermal_candidates,
        pre_ingress_soil_thermal_sha256,
        soil_thermal_candidates,
        receiver_closure_operands,
        rollback_hashes,
    };
    candidate.validate(surface_configuration)?;
    Ok(candidate)
}

#[allow(clippy::too_many_arguments, clippy::too_many_lines)]
fn apply_ingress_to_real_receivers(
    owner: &RealHydrologyShadowAdapter,
    configuration: &DirectSurfaceLiquidConfiguration,
    receiver_expectations: &UnifiedReceiverExpectations,
    request_batch: &PotentialWaterRequestBatch,
    ingress: &DirectSurfaceLiquidIngressCandidate,
    ending_frame: &mut DirectRunFrame,
    lse_tiles: &mut [TileState],
    soil_thermal: &mut [SoilThermalTileCandidate],
    rollback_hashes: &[OwnerRollbackHash],
    beginning_hydrology_snapshot_sha256: &Sha256Digest,
    exact_surface_custody: bool,
) -> Result<RealReceiverClosureOperands, LandSurfaceEnergyShadowError> {
    validate_surface_production_binding(owner, configuration)?;
    let receiver_attempt_sha256 =
        finalization_receiver_sets_sha256(lse_tiles, soil_thermal, rollback_hashes);
    receiver_validation::preflight_finalization_receiver_numerics(
        ingress.transaction_id(),
        configuration,
        receiver_expectations,
        lse_tiles,
        soil_thermal,
        rollback_hashes,
        &receiver_attempt_sha256,
    )?;
    receiver_preflight::preflight_receiver_derived_arithmetic(
        owner,
        configuration,
        receiver_expectations,
        ingress,
        ending_frame,
        lse_tiles,
        soil_thermal,
        rollback_hashes,
        &receiver_attempt_sha256,
    )?;
    validate_receiver_expectations(
        owner,
        configuration,
        receiver_expectations,
        request_batch,
        beginning_hydrology_snapshot_sha256,
    )?;
    validate_receiver_sets(
        configuration,
        receiver_expectations,
        lse_tiles,
        soil_thermal,
    )
    .map_err(|violation| {
        receiver_envelope_failure(
            ingress.transaction_id(),
            configuration,
            &violation,
            rollback_hashes,
            &receiver_attempt_sha256,
        )
    })?;
    validate_rollback_joins(
        owner,
        receiver_expectations,
        soil_thermal,
        rollback_hashes,
        beginning_hydrology_snapshot_sha256,
    )
    .map_err(|violation| {
        receiver_envelope_failure(
            ingress.transaction_id(),
            configuration,
            &violation,
            rollback_hashes,
            &receiver_attempt_sha256,
        )
    })?;
    let beginning_frame = ending_frame.clone();
    let beginning_lse_tiles = lse_tiles.to_vec();
    let beginning_soil_thermal = soil_thermal.to_vec();
    let failure_scope = ReceiverFailureScope {
        transaction_id: ingress.transaction_id(),
        configuration,
        expectations: receiver_expectations,
        hydrology_owner_id: owner.hydrology_owner_id(),
        rollback_hashes,
        attempted_sha256: &receiver_attempt_sha256,
    };
    let mut infiltration_m_by_lane =
        BTreeMap::<usize, (f64, &DirectSurfaceLiquidParcelReceipt)>::new();
    let mut retained_by_store =
        BTreeMap::<DirectSurfaceLiquidStoreKey, Vec<&DirectSurfaceLiquidParcelReceipt>>::new();
    for receipt in ingress.receipts() {
        if let (
            DirectSurfaceLiquidReceiptDisposition::RetainedSurface,
            DirectSurfaceLiquidReceiptRecipient::SurfaceStore { store_key },
        ) = (&receipt.disposition, &receipt.recipient)
        {
            if store_key != &receipt.recipient_store_key {
                return Err(failure_scope.failure(
                    DirectSurfaceLiquidErrorCode::E009,
                    &failure_scope.configuration.owner_id,
                    receipt,
                    "retained receipt typed receiver",
                ));
            }
            retained_by_store
                .entry(store_key.clone())
                .or_default()
                .push(receipt);
            continue;
        }
        if let Some((lane_index, infiltration_m)) =
            apply_receiver_receipt(&failure_scope, receipt, lse_tiles, soil_thermal)?
        {
            let accumulated = infiltration_m_by_lane
                .entry(lane_index)
                .or_insert((0.0, receipt));
            accumulated.0 =
                checked_surface_liquid_add(accumulated.0, infiltration_m).ok_or_else(|| {
                    receiver_phase_arithmetic_failure(
                        ingress.transaction_id(),
                        Some(OwnerKind::Hydrology),
                        owner.hydrology_owner_id(),
                        receipt,
                        rollback_hashes,
                        &receiver_attempt_sha256,
                        "infiltration lane accumulation is nonfinite or underflowed",
                    )
                })?;
            accumulated.1 = receipt;
        }
    }
    for (store_key, receipts) in retained_by_store {
        credit_retained_receipt_group(
            &failure_scope,
            &store_key,
            &receipts,
            lse_tiles,
            exact_surface_custody,
        )?;
    }
    apply_production_infiltration(owner, &failure_scope, ending_frame, infiltration_m_by_lane)?;
    let operands = freeze_real_receiver_closure_operands(
        owner,
        configuration,
        receiver_expectations,
        ingress,
        &beginning_frame,
        ending_frame,
        &beginning_lse_tiles,
        lse_tiles,
        &beginning_soil_thermal,
        soil_thermal,
        rollback_hashes,
        beginning_hydrology_snapshot_sha256,
    )?;
    validate_real_receiver_closure(&operands)?;
    Ok(operands)
}

fn validate_receiver_sets(
    configuration: &DirectSurfaceLiquidConfiguration,
    expectations: &UnifiedReceiverExpectations,
    lse_tiles: &[TileState],
    soil_thermal: &[SoilThermalTileCandidate],
) -> Result<(), ReceiverEnvelopeViolation> {
    let expected_tiles = configuration
        .records
        .iter()
        .map(|record| (record.key.ofe_id.clone(), record.key.tile_id.clone()))
        .collect::<Vec<_>>();
    validate_receiver_topologies(&expected_tiles, expectations, lse_tiles, soil_thermal)?;
    for candidate in soil_thermal {
        validate_thermal_receiver(configuration, expectations, candidate)?;
    }
    Ok(())
}

fn validate_receiver_topologies(
    expected_tiles: &[(OfeId, TileId)],
    expectations: &UnifiedReceiverExpectations,
    lse_tiles: &[TileState],
    soil_thermal: &[SoilThermalTileCandidate],
) -> Result<(), ReceiverEnvelopeViolation> {
    let expected_expectations = expectations
        .ordered_thermal_layers
        .iter()
        .map(|(identity, _)| identity.clone())
        .collect::<Vec<_>>();
    if let Some(violation) = first_expected_identity_violation(
        expected_tiles,
        &expected_expectations,
        OwnerKind::SoilThermal,
        &expectations.soil_thermal_owner_id,
        "independent soil-thermal expectation topology mismatch",
    ) {
        return Err(violation);
    }
    let actual_lse_tiles = lse_tiles
        .iter()
        .map(|tile| (tile.ofe_id.clone(), tile.tile_id.clone()))
        .collect::<Vec<_>>();
    if let Some(violation) = first_expected_identity_violation(
        expected_tiles,
        &actual_lse_tiles,
        OwnerKind::LandSurfaceEnergy,
        &expectations.lse_owner_id,
        "LSE tile receiver topology mismatch",
    ) {
        return Err(violation);
    }
    let actual_thermal_tiles = soil_thermal
        .iter()
        .map(|tile| (tile.ofe_id.clone(), tile.tile_id.clone()))
        .collect::<Vec<_>>();
    if let Some(index) = first_identity_mismatch(expected_tiles, &actual_thermal_tiles) {
        let missing_expected = if actual_thermal_tiles.len() < expected_tiles.len() {
            let actual_membership = actual_thermal_tiles.iter().collect::<BTreeSet<_>>();
            expected_tiles
                .iter()
                .enumerate()
                .find(|(_, identity)| !actual_membership.contains(identity))
                .map(|(index, _)| index)
        } else {
            None
        };
        let violation = if let Some(missing_index) = missing_expected {
            let (ofe_id, tile_id) = &expected_tiles[missing_index];
            ReceiverEnvelopeViolation::for_tile(
                OwnerKind::SoilThermal,
                Some(expectations.soil_thermal_owner_id.clone()),
                ofe_id.clone(),
                tile_id.clone(),
                "missing soil-thermal tile receiver",
            )
        } else if let Some(candidate) = soil_thermal.get(index) {
            ReceiverEnvelopeViolation::for_tile(
                OwnerKind::SoilThermal,
                Some(candidate.owner_id.clone()),
                candidate.ofe_id.clone(),
                candidate.tile_id.clone(),
                "soil-thermal tile receiver topology mismatch",
            )
        } else {
            let (ofe_id, tile_id) = &expected_tiles[index];
            ReceiverEnvelopeViolation::for_tile(
                OwnerKind::SoilThermal,
                Some(expectations.soil_thermal_owner_id.clone()),
                ofe_id.clone(),
                tile_id.clone(),
                "missing soil-thermal tile receiver",
            )
        };
        return Err(violation);
    }
    Ok(())
}

fn validate_thermal_receiver(
    configuration: &DirectSurfaceLiquidConfiguration,
    expectations: &UnifiedReceiverExpectations,
    candidate: &SoilThermalTileCandidate,
) -> Result<(), ReceiverEnvelopeViolation> {
    let violation = ReceiverEnvelopeViolation::for_tile(
        OwnerKind::SoilThermal,
        Some(candidate.owner_id.clone()),
        candidate.ofe_id.clone(),
        candidate.tile_id.clone(),
        "invalid soil-thermal receiver row",
    );
    let Some(binding) = configuration
        .ofe_bindings
        .iter()
        .find(|binding| binding.ofe_id == candidate.ofe_id)
    else {
        return Err(violation);
    };
    let Some(expected_layers) = expectations
        .ordered_thermal_layers
        .iter()
        .find(|(identity, _)| identity == &(candidate.ofe_id.clone(), candidate.tile_id.clone()))
        .map(|(_, layers)| layers)
    else {
        return Err(violation);
    };
    let layers = candidate
        .layers
        .iter()
        .map(|layer| layer.layer_id.clone())
        .collect::<Vec<_>>();
    let has_nonfinite = candidate.layers.iter().any(|layer| {
        !layer.beginning_enthalpy_j_m2_ofe_ground.is_finite()
            || !layer.ground_heat_credit_j_m2_ofe_ground.is_finite()
            || !layer
                .infiltration_enthalpy_credit_j_m2_ofe_ground
                .is_finite()
            || !layer.ending_enthalpy_j_m2_ofe_ground.is_finite()
            || !layer.ending_temperature_k.is_finite()
    });
    let has_duplicate_layer = layers.iter().collect::<BTreeSet<_>>().len() != layers.len();
    if candidate.owner_id != expectations.soil_thermal_owner_id
        || candidate.beginning_state_sha256 != expectations.beginning_soil_thermal_state_sha256
        || layers != *expected_layers
        || has_nonfinite
        || candidate.layers.is_empty()
        || candidate.layers[0].layer_id != binding.infiltration_soil_thermal_layer_id
        || has_duplicate_layer
    {
        return Err(violation);
    }
    Ok(())
}

fn first_identity_mismatch(
    expected: &[(OfeId, TileId)],
    actual: &[(OfeId, TileId)],
) -> Option<usize> {
    (0..expected.len().max(actual.len())).find(|&index| expected.get(index) != actual.get(index))
}

fn first_expected_identity_violation(
    expected: &[(OfeId, TileId)],
    actual: &[(OfeId, TileId)],
    owner_kind: OwnerKind,
    owner_id: &ResourceOwnerId,
    detail: &'static str,
) -> Option<ReceiverEnvelopeViolation> {
    if actual.len() < expected.len() {
        let actual_membership = actual.iter().collect::<BTreeSet<_>>();
        if let Some((ofe_id, tile_id)) = expected
            .iter()
            .find(|identity| !actual_membership.contains(identity))
        {
            return Some(ReceiverEnvelopeViolation::for_tile(
                owner_kind,
                Some(owner_id.clone()),
                ofe_id.clone(),
                tile_id.clone(),
                detail,
            ));
        }
    }
    let index = first_identity_mismatch(expected, actual)?;
    let (ofe_id, tile_id) = actual.get(index).or_else(|| expected.get(index))?;
    Some(ReceiverEnvelopeViolation::for_tile(
        owner_kind,
        Some(owner_id.clone()),
        ofe_id.clone(),
        tile_id.clone(),
        detail,
    ))
}

fn validate_rollback_joins(
    owner: &RealHydrologyShadowAdapter,
    expectations: &UnifiedReceiverExpectations,
    soil_thermal: &[SoilThermalTileCandidate],
    rollback_hashes: &[OwnerRollbackHash],
    beginning_hydrology_snapshot_sha256: &Sha256Digest,
) -> Result<(), ReceiverEnvelopeViolation> {
    let expected = [
        (
            OwnerKind::LandSurfaceEnergy,
            expectations.lse_owner_id.as_str(),
            &expectations.beginning_lse_state_sha256,
        ),
        (
            OwnerKind::Hydrology,
            owner.hydrology_owner_id().as_str(),
            beginning_hydrology_snapshot_sha256,
        ),
        (
            OwnerKind::SoilThermal,
            expectations.soil_thermal_owner_id.as_str(),
            &expectations.beginning_soil_thermal_state_sha256,
        ),
    ];
    if rollback_hashes.len() < expected.len() {
        for (kind, owner_id, _) in expected {
            let expected_present = rollback_hashes
                .iter()
                .any(|actual| actual.owner_kind == kind && actual.owner_id == owner_id);
            if !expected_present {
                return Err(ReceiverEnvelopeViolation::for_owner(
                    kind,
                    ResourceOwnerId::try_new(owner_id.to_owned()).ok(),
                    missing_rollback_detail(kind),
                ));
            }
        }
    }
    for index in 0..rollback_hashes.len().max(expected.len()) {
        let actual = rollback_hashes.get(index);
        let expected_row = expected.get(index);
        match (actual, expected_row) {
            (Some(actual), Some((kind, owner_id, beginning)))
                if actual.owner_kind != *kind
                    || actual.owner_id != *owner_id
                    || &actual.before_sha256 != *beginning
                    || &actual.after_sha256 != *beginning =>
            {
                return Err(rollback_violation(actual, "rollback owner row mismatch"));
            }
            (Some(actual), None) => {
                return Err(rollback_violation(actual, "unexpected rollback owner row"));
            }
            (None, Some((kind, owner_id, _))) => {
                return Err(ReceiverEnvelopeViolation::for_owner(
                    *kind,
                    ResourceOwnerId::try_new((*owner_id).to_owned()).ok(),
                    missing_rollback_detail(*kind),
                ));
            }
            _ => {}
        }
    }
    if let Some(candidate) = soil_thermal.iter().find(|candidate| {
        candidate.owner_id != expectations.soil_thermal_owner_id
            || candidate.beginning_state_sha256 != expectations.beginning_soil_thermal_state_sha256
    }) {
        return Err(ReceiverEnvelopeViolation::for_tile(
            OwnerKind::SoilThermal,
            Some(candidate.owner_id.clone()),
            candidate.ofe_id.clone(),
            candidate.tile_id.clone(),
            "soil-thermal rollback lineage mismatch",
        ));
    }
    Ok(())
}

fn rollback_violation(
    row: &OwnerRollbackHash,
    fallback: &'static str,
) -> ReceiverEnvelopeViolation {
    let detail = match row.owner_kind {
        OwnerKind::LandSurfaceEnergy => "LSE rollback owner row mismatch",
        OwnerKind::SoilThermal => "soil-thermal rollback owner row mismatch",
        OwnerKind::Hydrology => "hydrology rollback owner row mismatch",
        OwnerKind::Vegetation => "vegetation rollback owner row mismatch",
        OwnerKind::Biogeochemistry => "biogeochemistry rollback owner row mismatch",
        OwnerKind::Envelope => fallback,
    };
    ReceiverEnvelopeViolation::for_owner(
        row.owner_kind,
        ResourceOwnerId::try_new(row.owner_id.clone()).ok(),
        detail,
    )
}

const fn missing_rollback_detail(kind: OwnerKind) -> &'static str {
    match kind {
        OwnerKind::LandSurfaceEnergy => "missing LSE rollback owner row",
        OwnerKind::Hydrology => "missing hydrology rollback owner row",
        OwnerKind::SoilThermal => "missing soil-thermal rollback owner row",
        OwnerKind::Vegetation => "missing vegetation rollback owner row",
        OwnerKind::Biogeochemistry => "missing biogeochemistry rollback owner row",
        OwnerKind::Envelope => "missing envelope rollback owner row",
    }
}

fn receiver_envelope_failure(
    transaction_id: TransactionId,
    configuration: &DirectSurfaceLiquidConfiguration,
    violation: &ReceiverEnvelopeViolation,
    rollback_hashes: &[OwnerRollbackHash],
    attempted_sha256: &str,
) -> LandSurfaceEnergyShadowError {
    let record = violation.ofe_id.as_ref().and_then(|ofe_id| {
        configuration.records.iter().find(|record| {
            &record.key.ofe_id == ofe_id
                && violation
                    .tile_id
                    .as_ref()
                    .is_some_and(|tile_id| &record.key.tile_id == tile_id)
        })
    });
    canonical_receiver_failure(
        DirectSurfaceLiquidErrorCode::E011,
        DirectSurfaceLiquidPhase::AtomicEnvelope,
        transaction_id,
        violation.owner_kind,
        violation.owner_id.as_ref(),
        violation.ofe_id.as_ref(),
        violation.tile_id.as_ref(),
        record.map(|record| record.key.surface_id.clone()),
        record.map(|record| record.key.source_id.clone()),
        None,
        rollback_hashes,
        attempted_sha256,
        violation.detail,
    )
    .into()
}

fn receiver_phase_arithmetic_failure(
    transaction_id: TransactionId,
    owner_kind: Option<OwnerKind>,
    owner_id: &ResourceOwnerId,
    receipt: &DirectSurfaceLiquidParcelReceipt,
    rollback_hashes: &[OwnerRollbackHash],
    attempted_sha256: &str,
    detail: &'static str,
) -> LandSurfaceEnergyShadowError {
    canonical_receiver_failure(
        DirectSurfaceLiquidErrorCode::E003,
        DirectSurfaceLiquidPhase::IndependentClosure,
        transaction_id,
        owner_kind,
        Some(owner_id),
        Some(&receipt.recipient_store_key.ofe_id),
        Some(&receipt.recipient_store_key.tile_id),
        Some(receipt.recipient_store_key.surface_id.clone()),
        Some(receipt.recipient_store_key.source_id.clone()),
        Some(receipt.parcel_id.clone()),
        rollback_hashes,
        attempted_sha256,
        detail,
    )
    .into()
}

struct ReceiverFailureScope<'a> {
    transaction_id: TransactionId,
    configuration: &'a DirectSurfaceLiquidConfiguration,
    expectations: &'a UnifiedReceiverExpectations,
    hydrology_owner_id: &'a ResourceOwnerId,
    rollback_hashes: &'a [OwnerRollbackHash],
    attempted_sha256: &'a str,
}

impl ReceiverFailureScope<'_> {
    fn owner_kind(&self, owner_id: &ResourceOwnerId) -> Option<OwnerKind> {
        if owner_id == &self.expectations.lse_owner_id {
            return Some(OwnerKind::LandSurfaceEnergy);
        }
        if owner_id == &self.expectations.soil_thermal_owner_id {
            return Some(OwnerKind::SoilThermal);
        }
        (owner_id == self.hydrology_owner_id).then_some(OwnerKind::Hydrology)
    }

    fn failure(
        &self,
        code: DirectSurfaceLiquidErrorCode,
        owner_id: &ResourceOwnerId,
        receipt: &DirectSurfaceLiquidParcelReceipt,
        detail: &'static str,
    ) -> LandSurfaceEnergyShadowError {
        canonical_receiver_failure(
            code,
            DirectSurfaceLiquidPhase::IndependentClosure,
            self.transaction_id,
            self.owner_kind(owner_id),
            Some(owner_id),
            Some(&receipt.recipient_store_key.ofe_id),
            Some(&receipt.recipient_store_key.tile_id),
            Some(receipt.recipient_store_key.surface_id.clone()),
            Some(receipt.recipient_store_key.source_id.clone()),
            Some(receipt.parcel_id.clone()),
            self.rollback_hashes,
            self.attempted_sha256,
            detail,
        )
        .into()
    }
}

fn apply_receiver_receipt(
    scope: &ReceiverFailureScope<'_>,
    receipt: &DirectSurfaceLiquidParcelReceipt,
    _lse_tiles: &mut [TileState],
    soil_thermal: &mut [SoilThermalTileCandidate],
) -> Result<Option<(usize, f64)>, LandSurfaceEnergyShadowError> {
    let binding = scope
        .configuration
        .ofe_bindings
        .iter()
        .find(|binding| binding.ofe_id == receipt.recipient_store_key.ofe_id)
        .ok_or_else(|| {
            scope.failure(
                DirectSurfaceLiquidErrorCode::E009,
                &scope.configuration.owner_id,
                receipt,
                "ingress receipt OFE binding",
            )
        })?;
    match (&receipt.disposition, &receipt.recipient) {
        (
            DirectSurfaceLiquidReceiptDisposition::Infiltration,
            DirectSurfaceLiquidReceiptRecipient::SoilInfiltration {
                ofe_id,
                production_lane_index,
                production_lane_id,
                ordered_soil_layer_ids,
                soil_thermal_layer_id,
            },
        ) => credit_infiltration_receipt(
            binding,
            receipt,
            soil_thermal,
            ofe_id,
            *production_lane_index,
            *production_lane_id,
            ordered_soil_layer_ids,
            soil_thermal_layer_id,
            scope,
        )
        .map(Some),
        (
            DirectSurfaceLiquidReceiptDisposition::RetainedSurface,
            DirectSurfaceLiquidReceiptRecipient::SurfaceStore { .. },
        ) => Err(scope.failure(
            DirectSurfaceLiquidErrorCode::E009,
            &scope.configuration.owner_id,
            receipt,
            "retained receipt escaped grouped receiver",
        )),
        (
            DirectSurfaceLiquidReceiptDisposition::RoutedRunoff,
            DirectSurfaceLiquidReceiptRecipient::RoutedOfe { .. },
        )
        | (
            DirectSurfaceLiquidReceiptDisposition::OutletRunoff,
            DirectSurfaceLiquidReceiptRecipient::Outlet { .. },
        ) => Ok(None),
        _ => Err(scope.failure(
            DirectSurfaceLiquidErrorCode::E009,
            &scope.configuration.owner_id,
            receipt,
            "receipt disposition/recipient join",
        )),
    }
}

#[allow(clippy::too_many_arguments)]
fn credit_infiltration_receipt(
    binding: &DirectSurfaceLiquidOfeBinding,
    receipt: &DirectSurfaceLiquidParcelReceipt,
    soil_thermal: &mut [SoilThermalTileCandidate],
    ofe_id: &OfeId,
    production_lane_index: usize,
    production_lane_id: u32,
    ordered_soil_layer_ids: &[openwepp_kernel_contract::SoilLayerId],
    soil_thermal_layer_id: &openwepp_kernel_contract::SoilLayerId,
    scope: &ReceiverFailureScope<'_>,
) -> Result<(usize, f64), LandSurfaceEnergyShadowError> {
    if ofe_id != &binding.ofe_id
        || production_lane_index != binding.production_lane_index
        || production_lane_id != binding.production_lane_id
        || ordered_soil_layer_ids != binding.ordered_soil_layer_ids
        || soil_thermal_layer_id != &binding.infiltration_soil_thermal_layer_id
    {
        return Err(scope.failure(
            DirectSurfaceLiquidErrorCode::E009,
            &scope.expectations.soil_thermal_owner_id,
            receipt,
            "infiltration receipt production receiver binding",
        ));
    }
    let thermal = soil_thermal
        .iter_mut()
        .find(|candidate| {
            candidate.ofe_id == receipt.recipient_store_key.ofe_id
                && candidate.tile_id == receipt.recipient_store_key.tile_id
        })
        .ok_or_else(|| {
            scope.failure(
                DirectSurfaceLiquidErrorCode::E010,
                &scope.expectations.soil_thermal_owner_id,
                receipt,
                "missing infiltration soil-thermal tile receiver",
            )
        })?;
    let layer = thermal
        .layers
        .iter_mut()
        .find(|layer| layer.layer_id == binding.infiltration_soil_thermal_layer_id)
        .ok_or_else(|| {
            scope.failure(
                DirectSurfaceLiquidErrorCode::E010,
                &scope.expectations.soil_thermal_owner_id,
                receipt,
                "missing infiltration soil-thermal layer receiver",
            )
        })?;
    layer.infiltration_enthalpy_credit_j_m2_ofe_ground = checked_receiver_credit_add(
        layer.infiltration_enthalpy_credit_j_m2_ofe_ground,
        receipt.enthalpy_j_m2_basis_ofe_ground,
    )
    .ok_or_else(|| {
        scope.failure(
            DirectSurfaceLiquidErrorCode::E003,
            &scope.expectations.soil_thermal_owner_id,
            receipt,
            "soil-thermal infiltration enthalpy arithmetic",
        )
    })?;
    layer.ending_enthalpy_j_m2_ofe_ground = checked_receiver_credit_add(
        layer.ending_enthalpy_j_m2_ofe_ground,
        receipt.enthalpy_j_m2_basis_ofe_ground,
    )
    .ok_or_else(|| {
        scope.failure(
            DirectSurfaceLiquidErrorCode::E003,
            &scope.expectations.soil_thermal_owner_id,
            receipt,
            "soil-thermal ending enthalpy arithmetic",
        )
    })?;
    let infiltration_m =
        checked_surface_liquid_div(receipt.mass_kg_m2_basis_ofe_ground, WATER_DENSITY_KG_M3)
            .ok_or_else(|| {
                scope.failure(
                    DirectSurfaceLiquidErrorCode::E003,
                    &scope.expectations.soil_thermal_owner_id,
                    receipt,
                    "infiltration mass-to-depth arithmetic",
                )
            })?;
    Ok((binding.production_lane_index, infiltration_m))
}

fn credit_retained_receipt_group(
    scope: &ReceiverFailureScope<'_>,
    store_key: &DirectSurfaceLiquidStoreKey,
    receipts: &[&DirectSurfaceLiquidParcelReceipt],
    lse_tiles: &mut [TileState],
    exact_surface_custody: bool,
) -> Result<(), LandSurfaceEnergyShadowError> {
    let representative = receipts.first().copied().ok_or({
        LandSurfaceEnergyShadowError::Identity("empty retained surface receipt group")
    })?;
    if receipts.iter().any(|receipt| {
        receipt.recipient_store_key != *store_key
            || receipt.disposition != DirectSurfaceLiquidReceiptDisposition::RetainedSurface
            || !matches!(
                &receipt.recipient,
                DirectSurfaceLiquidReceiptRecipient::SurfaceStore { store_key: recipient }
                    if recipient == store_key
            )
    }) {
        return Err(scope.failure(
            DirectSurfaceLiquidErrorCode::E009,
            &scope.configuration.owner_id,
            representative,
            "retained receipt group identity",
        ));
    }
    let tile = lse_tiles
        .iter_mut()
        .find(|tile| tile.ofe_id == store_key.ofe_id && tile.tile_id == store_key.tile_id)
        .ok_or_else(|| {
            scope.failure(
                DirectSurfaceLiquidErrorCode::E010,
                &scope.expectations.lse_owner_id,
                representative,
                "missing retained LSE tile receiver",
            )
        })?;
    let record = scope
        .configuration
        .records
        .iter()
        .find(|record| record.key == *store_key)
        .ok_or_else(|| {
            scope.failure(
                DirectSurfaceLiquidErrorCode::E009,
                &scope.configuration.owner_id,
                representative,
                "retained receipt store receiver",
            )
        })?;
    let retained_ofe = checked_surface_liquid_sum(
        receipts
            .iter()
            .map(|receipt| receipt.enthalpy_j_m2_basis_ofe_ground),
    )
    .ok_or_else(|| {
        scope.failure(
            DirectSurfaceLiquidErrorCode::E003,
            &scope.expectations.lse_owner_id,
            representative,
            "retained enthalpy group accumulation",
        )
    })?;
    let retained_tile =
        checked_surface_liquid_div(retained_ofe, record.tile_fraction).ok_or_else(|| {
            scope.failure(
                DirectSurfaceLiquidErrorCode::E003,
                &scope.expectations.lse_owner_id,
                representative,
                "retained enthalpy OFE-to-tile arithmetic",
            )
        })?;
    // This is a frozen binary64 high mirror only when the caller is the V16
    // path that already carries and has validated the authoritative exact
    // owner. Legacy and V3-only candidates retain their historical fail-closed
    // refusal when a nonzero credit would disappear below high-term spacing.
    let beginning = tile.surface_enthalpy_j_m2_tile_ground;
    tile.surface_enthalpy_j_m2_tile_ground = checked_retained_surface_high_add(
        beginning,
        retained_tile,
        exact_surface_custody,
    )
    .ok_or_else(|| {
        scope.failure(
            DirectSurfaceLiquidErrorCode::E003,
            &scope.expectations.lse_owner_id,
            representative,
            "retained surface enthalpy arithmetic",
        )
    })?;
    Ok(())
}

fn checked_retained_surface_high_add(
    beginning: f64,
    credit: f64,
    exact_surface_custody: bool,
) -> Option<f64> {
    if exact_surface_custody {
        checked_surface_liquid_add(beginning, credit)
    } else {
        checked_receiver_credit_add(beginning, credit)
    }
}

fn checked_receiver_credit_add(beginning: f64, credit: f64) -> Option<f64> {
    let ending = checked_surface_liquid_add(beginning, credit)?;
    (credit == 0.0 || ending.to_bits() != beginning.to_bits()).then_some(ending)
}

#[cfg(test)]
mod v16_retained_surface_high_custody_tests {
    use super::checked_retained_surface_high_add;

    #[test]
    fn sub_ulp_retained_credit_requires_exact_v4_custody() {
        let beginning = 2.0_f64.powi(100);
        for credit in [1.0, -1.0] {
            assert!(
                checked_retained_surface_high_add(beginning, credit, false).is_none(),
                "legacy and V3-only callers must refuse a lost retained credit",
            );
            let ending = checked_retained_surface_high_add(beginning, credit, true)
                .expect("V4 exact custody admits an unchanged high mirror");
            assert_eq!(ending.to_bits(), beginning.to_bits());
        }
    }
}

/// Exact V16 custody operand produced after the existing accepted binary64
/// OFE-to-tile retained-ingress conversion.  The physical parcel partition and
/// basis conversion remain owned by the surface-liquid transaction; the
/// exact-surface owner decodes only `energy_j_m2_tile_ground`.
#[derive(Clone, Debug, PartialEq)]
pub(crate) struct RetainedSurfaceTileCreditV1 {
    pub store_key: DirectSurfaceLiquidStoreKey,
    pub source_receipt_sha256: Sha256Digest,
    pub ordinal: u32,
    pub energy_j_m2_ofe_ground: f64,
    pub tile_fraction: f64,
    pub energy_j_m2_tile_ground: f64,
}

/// Reconstruct the named accepted retained-ingress tile credits without
/// reading the mutated high mirror or a producer residual.
/// Replay the retained tile-credit groups directly from the canonical nested
/// ingress receipts. V4 projection/restart validation uses this path so source
/// identities are checked independently of the exact receipt's operand copy.
pub(crate) fn retained_surface_tile_credits_from_receipts_v1(
    configuration: &crate::direct_runtime::SurfaceLiquidConfigurationV2,
    transaction_id: TransactionId,
    receipts: &[DirectSurfaceLiquidParcelReceipt],
) -> Result<Vec<RetainedSurfaceTileCreditV1>, LandSurfaceEnergyShadowError> {
    let mut grouped =
        BTreeMap::<DirectSurfaceLiquidStoreKey, Vec<&DirectSurfaceLiquidParcelReceipt>>::new();
    for receipt in receipts {
        if receipt.disposition != DirectSurfaceLiquidReceiptDisposition::RetainedSurface {
            continue;
        }
        let DirectSurfaceLiquidReceiptRecipient::SurfaceStore { store_key } = &receipt.recipient
        else {
            return Err(LandSurfaceEnergyShadowError::Identity(
                "V16 retained receipt typed recipient",
            ));
        };
        if receipt.transaction_id != transaction_id
            || receipt.recipient_store_key != *store_key
            || receipt.basis_ofe_id != store_key.ofe_id
            || !receipt.enthalpy_j_m2_basis_ofe_ground.is_finite()
        {
            return Err(LandSurfaceEnergyShadowError::Identity(
                "V16 retained receipt identity or domain",
            ));
        }
        grouped.entry(store_key.clone()).or_default().push(receipt);
    }

    let mut credits = Vec::with_capacity(grouped.len());
    for (ordinal, (store_key, mut receipts)) in grouped.into_iter().enumerate() {
        receipts.sort_unstable_by(|left, right| {
            (&left.parcel_id, &left.source_parcel_id)
                .cmp(&(&right.parcel_id, &right.source_parcel_id))
        });
        if receipts
            .windows(2)
            .any(|rows| rows[0].parcel_id == rows[1].parcel_id)
        {
            return Err(LandSurfaceEnergyShadowError::Identity(
                "V16 duplicate retained receipt identity",
            ));
        }
        let record = configuration
            .parent()
            .records
            .iter()
            .find(|record| record.key == store_key)
            .ok_or(LandSurfaceEnergyShadowError::Identity(
                "V16 retained receipt configured surface",
            ))?;
        let energy_j_m2_ofe_ground = checked_surface_liquid_sum(
            receipts
                .iter()
                .map(|receipt| receipt.enthalpy_j_m2_basis_ofe_ground),
        )
        .ok_or(LandSurfaceEnergyShadowError::Bound(
            "V16 retained receipt OFE-ground sum",
        ))?;
        let energy_j_m2_tile_ground =
            checked_surface_liquid_div(energy_j_m2_ofe_ground, record.tile_fraction).ok_or(
                LandSurfaceEnergyShadowError::Bound(
                    "V16 retained receipt accepted OFE-to-tile conversion",
                ),
            )?;
        let source_receipt_sha256 = v2_physical_operand_digest(&receipts)?;
        credits.push(RetainedSurfaceTileCreditV1 {
            store_key,
            source_receipt_sha256,
            ordinal: u32::try_from(ordinal).map_err(|_| {
                LandSurfaceEnergyShadowError::Bound("V16 retained receipt ordinal overflow")
            })?,
            energy_j_m2_ofe_ground,
            tile_fraction: record.tile_fraction,
            energy_j_m2_tile_ground,
        });
    }
    Ok(credits)
}

/// Reconstruct canonical V2 soil-energy operands from accepted LSE storage
/// deltas and typed infiltration receipts before a soil credit exists.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct PhysicalSoilEnergyTransactionAuthorityV2 {
    pub(crate) source_transaction_id: TransactionId,
    pub(crate) soil_thermal_transaction_id: TransactionId,
    beginning_posture: PhysicalSoilEnergyBeginningPostureV2,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum PhysicalSoilEnergyBeginningPostureV2 {
    LegacyV1,
    NativeV2,
}

impl PhysicalSoilEnergyTransactionAuthorityV2 {
    pub fn try_new(
        source_transaction_id: TransactionId,
        soil_thermal_transaction_id: TransactionId,
    ) -> Result<Self, LandSurfaceEnergyShadowError> {
        if source_transaction_id.0 == 0 || soil_thermal_transaction_id.0 == 0 {
            return Err(LandSurfaceEnergyShadowError::Identity(
                "V2 soil transaction authority",
            ));
        }
        Ok(Self {
            source_transaction_id,
            soil_thermal_transaction_id,
            beginning_posture: PhysicalSoilEnergyBeginningPostureV2::NativeV2,
        })
    }

    pub fn try_from_pre_ingress_candidates(
        source_transaction_id: TransactionId,
        support_start_ns: u128,
        support_end_ns: u128,
        candidates: &[SoilThermalTileCandidate],
    ) -> Result<Self, LandSurfaceEnergyShadowError> {
        let first = candidates
            .first()
            .ok_or(LandSurfaceEnergyShadowError::Identity(
                "soil target transaction authority",
            ))?;
        let authority = match &first.beginning_identity {
                openwepp_land_surface_energy::SoilThermalCandidateBeginningIdentity::V2 {
                    transaction_id,
                    ..
                } => Self::try_new(source_transaction_id, *transaction_id)?,
                openwepp_land_surface_energy::SoilThermalCandidateBeginningIdentity::V1 {
                    ..
                } => Self {
                    source_transaction_id,
                    soil_thermal_transaction_id: source_transaction_id,
                    beginning_posture: PhysicalSoilEnergyBeginningPostureV2::LegacyV1,
                },
            };
        if source_transaction_id.0 == 0 {
            return Err(LandSurfaceEnergyShadowError::Identity(
                "soil transaction authority",
            ));
        }
        authority.validate_pre_ingress_candidates(
            support_start_ns,
            support_end_ns,
            candidates,
        )?;
        Ok(authority)
    }

    fn validate_pre_ingress_candidates(
        self,
        support_start_ns: u128,
        support_end_ns: u128,
        candidates: &[SoilThermalTileCandidate],
    ) -> Result<(), LandSurfaceEnergyShadowError> {
        let first = candidates
            .first()
            .ok_or(LandSurfaceEnergyShadowError::Identity(
                "V2 soil target transaction or support",
            ))?;
        if candidates.iter().any(|candidate| {
                candidate.owner_id != first.owner_id
                    || candidate.beginning_state_sha256 != first.beginning_state_sha256
                    || candidate.beginning_identity != first.beginning_identity
                    || match self.beginning_posture {
                        PhysicalSoilEnergyBeginningPostureV2::LegacyV1 => {
                            self.source_transaction_id != self.soil_thermal_transaction_id
                                || !matches!(
                                    &candidate.beginning_identity,
                                    openwepp_land_surface_energy::SoilThermalCandidateBeginningIdentity::V1 { .. }
                                )
                        }
                        PhysicalSoilEnergyBeginningPostureV2::NativeV2 => !matches!(
                            &candidate.beginning_identity,
                            openwepp_land_surface_energy::SoilThermalCandidateBeginningIdentity::V2 {
                                transaction_id,
                                support_start_ns: candidate_start_ns,
                                support_end_ns: candidate_end_ns,
                                ..
                            } if *transaction_id == self.soil_thermal_transaction_id
                                && *candidate_start_ns == support_start_ns
                                && *candidate_end_ns == support_end_ns
                        ),
                    }
            })
        {
            return Err(LandSurfaceEnergyShadowError::Identity(
                "V2 soil target transaction or support",
            ));
        }
        Ok(())
    }
}

pub fn physical_soil_energy_operands_v2(
    authority: PhysicalSoilEnergyTransactionAuthorityV2,
    support_start_ns: u128,
    support_end_ns: u128,
    lse_owner_id: &ResourceOwnerId,
    surface_owner_id: &ResourceOwnerId,
    pre_ingress_candidates: &[SoilThermalTileCandidate],
    ingress: &DirectSurfaceLiquidIngressCandidate,
) -> Result<Vec<SoilThermalEnergyOperandV2>, LandSurfaceEnergyShadowError> {
    if authority.source_transaction_id.0 == 0
        || authority.soil_thermal_transaction_id.0 == 0
        || ingress.transaction_id() != authority.source_transaction_id
        || ingress
            .receipts()
            .iter()
            .any(|receipt| receipt.transaction_id != authority.source_transaction_id)
        || support_start_ns >= support_end_ns
    {
        return Err(LandSurfaceEnergyShadowError::Identity(
            "V2 soil support identity",
        ));
    }
    authority.validate_pre_ingress_candidates(
        support_start_ns,
        support_end_ns,
        pre_ingress_candidates,
    )?;
    let mut operands = Vec::new();
    append_v2_soil_internal_operands(
        authority,
        support_start_ns,
        support_end_ns,
        lse_owner_id,
        pre_ingress_candidates,
        &mut operands,
    )?;
    append_v2_infiltration_operands(
        authority,
        support_start_ns,
        support_end_ns,
        surface_owner_id,
        ingress,
        &mut operands,
    )?;
    operands.sort_unstable_by(|left, right| {
        (&left.ofe_id, &left.layer_id, left.source_kind, left.ordinal).cmp(&(
            &right.ofe_id,
            &right.layer_id,
            right.source_kind,
            right.ordinal,
        ))
    });
    Ok(operands)
}

fn append_v2_soil_internal_operands(
    authority: PhysicalSoilEnergyTransactionAuthorityV2,
    support_start_ns: u128,
    support_end_ns: u128,
    lse_owner_id: &ResourceOwnerId,
    pre_ingress_candidates: &[SoilThermalTileCandidate],
    operands: &mut Vec<SoilThermalEnergyOperandV2>,
) -> Result<(), LandSurfaceEnergyShadowError> {
    let mut candidates = pre_ingress_candidates.iter().collect::<Vec<_>>();
    candidates.sort_unstable_by(|left, right| {
        (&left.ofe_id, &left.tile_id).cmp(&(&right.ofe_id, &right.tile_id))
    });
    let mut internal_ordinals = BTreeMap::<(OfeId, SoilLayerId), u32>::new();
    for candidate in candidates {
        for layer in &candidate.layers {
            if layer.infiltration_enthalpy_credit_j_m2_ofe_ground.to_bits() != 0.0_f64.to_bits()
                || !layer.ending_enthalpy_j_m2_ofe_ground.is_finite()
                || !layer.beginning_enthalpy_j_m2_ofe_ground.is_finite()
            {
                return Err(LandSurfaceEnergyShadowError::Identity(
                    "V2 pre-ingress soil energy",
                ));
            }
            let energy =
                layer.ending_enthalpy_j_m2_ofe_ground - layer.beginning_enthalpy_j_m2_ofe_ground;
            if !energy.is_finite() {
                return Err(LandSurfaceEnergyShadowError::Bound(
                    "V2 soil-internal energy delta",
                ));
            }
            let ordinal = next_v2_operand_ordinal(
                &mut internal_ordinals,
                &candidate.ofe_id,
                &layer.layer_id,
            )?;
            let digest = match authority.beginning_posture {
                PhysicalSoilEnergyBeginningPostureV2::LegacyV1 => {
                    v2_physical_operand_digest(&(
                        "OPENWEPP_ACCEPTED_SOIL_INTERNAL_ENERGY_V2",
                        authority.source_transaction_id,
                        support_start_ns,
                        support_end_ns,
                        lse_owner_id,
                        &candidate.owner_id,
                        &candidate.beginning_state_sha256,
                        &candidate.ofe_id,
                        &candidate.tile_id,
                        &layer.layer_id,
                        ordinal,
                        layer.beginning_enthalpy_j_m2_ofe_ground,
                        layer.ending_enthalpy_j_m2_ofe_ground,
                        energy,
                    ))?
                }
                PhysicalSoilEnergyBeginningPostureV2::NativeV2 => {
                    v2_physical_operand_digest(&(
                        "OPENWEPP_ACCEPTED_SOIL_INTERNAL_ENERGY_V2_TX_SPLIT_V1",
                        ("source_transaction_id", authority.source_transaction_id),
                        (
                            "soil_thermal_transaction_id",
                            authority.soil_thermal_transaction_id,
                        ),
                        support_start_ns,
                        support_end_ns,
                        lse_owner_id,
                        &candidate.owner_id,
                        &candidate.beginning_state_sha256,
                        &candidate.ofe_id,
                        &candidate.tile_id,
                        &layer.layer_id,
                        ordinal,
                        layer.beginning_enthalpy_j_m2_ofe_ground,
                        layer.ending_enthalpy_j_m2_ofe_ground,
                        energy,
                    ))?
                }
            };
            operands.push(SoilThermalEnergyOperandV2 {
                ofe_id: candidate.ofe_id.clone(),
                layer_id: layer.layer_id.clone(),
                source_kind:
                    openwepp_land_surface_energy::SoilThermalEnergyOperandKindV2::SoilInternal,
                source_owner_id: lse_owner_id.clone(),
                debit_credit_identity_sha256: digest,
                ordinal,
                units: "J m^-2 OFE-ground".to_owned(),
                basis: "ofe_ground".to_owned(),
                energy_j_m2_ofe_ground: energy,
            });
        }
    }
    Ok(())
}

fn append_v2_infiltration_operands(
    authority: PhysicalSoilEnergyTransactionAuthorityV2,
    support_start_ns: u128,
    support_end_ns: u128,
    surface_owner_id: &ResourceOwnerId,
    ingress: &DirectSurfaceLiquidIngressCandidate,
    operands: &mut Vec<SoilThermalEnergyOperandV2>,
) -> Result<(), LandSurfaceEnergyShadowError> {
    let mut infiltration_ordinals = BTreeMap::<(OfeId, SoilLayerId), u32>::new();
    for receipt in ingress.receipts() {
        let (
            DirectSurfaceLiquidReceiptDisposition::Infiltration,
            DirectSurfaceLiquidReceiptRecipient::SoilInfiltration {
                ofe_id,
                soil_thermal_layer_id,
                ..
            },
        ) = (&receipt.disposition, &receipt.recipient)
        else {
            continue;
        };
        if ofe_id != &receipt.recipient_store_key.ofe_id
            || !receipt.enthalpy_j_m2_basis_ofe_ground.is_finite()
        {
            return Err(LandSurfaceEnergyShadowError::Identity(
                "V2 infiltration energy receipt identity",
            ));
        }
        let ordinal =
            next_v2_operand_ordinal(&mut infiltration_ordinals, ofe_id, soil_thermal_layer_id)?;
        let digest = match authority.beginning_posture {
            PhysicalSoilEnergyBeginningPostureV2::LegacyV1 => v2_physical_operand_digest(&(
                "OPENWEPP_ACCEPTED_SOIL_INFILTRATION_ENERGY_V2",
                authority.source_transaction_id,
                support_start_ns,
                support_end_ns,
                surface_owner_id,
                receipt,
                soil_thermal_layer_id,
                ordinal,
            ))?,
            PhysicalSoilEnergyBeginningPostureV2::NativeV2 => v2_physical_operand_digest(&(
                "OPENWEPP_ACCEPTED_SOIL_INFILTRATION_ENERGY_V2_TX_SPLIT_V1",
                ("source_transaction_id", authority.source_transaction_id),
                (
                    "soil_thermal_transaction_id",
                    authority.soil_thermal_transaction_id,
                ),
                support_start_ns,
                support_end_ns,
                surface_owner_id,
                receipt,
                soil_thermal_layer_id,
                ordinal,
            ))?,
        };
        operands.push(SoilThermalEnergyOperandV2 {
            ofe_id: ofe_id.clone(),
            layer_id: soil_thermal_layer_id.clone(),
            source_kind: openwepp_land_surface_energy::SoilThermalEnergyOperandKindV2::Infiltration,
            source_owner_id: surface_owner_id.clone(),
            debit_credit_identity_sha256: digest,
            ordinal,
            units: "J m^-2 OFE-ground".to_owned(),
            basis: "ofe_ground".to_owned(),
            energy_j_m2_ofe_ground: receipt.enthalpy_j_m2_basis_ofe_ground,
        });
    }
    Ok(())
}

fn next_v2_operand_ordinal(
    ordinals: &mut BTreeMap<(OfeId, SoilLayerId), u32>,
    ofe_id: &OfeId,
    layer_id: &SoilLayerId,
) -> Result<u32, LandSurfaceEnergyShadowError> {
    let next = ordinals
        .entry((ofe_id.clone(), layer_id.clone()))
        .or_insert(0);
    let ordinal = *next;
    *next = next
        .checked_add(1)
        .ok_or(LandSurfaceEnergyShadowError::Bound(
            "V2 soil-energy operand ordinal overflow",
        ))?;
    Ok(ordinal)
}

pub(super) fn v2_physical_operand_digest<T: serde::Serialize>(
    value: &T,
) -> Result<Sha256Digest, LandSurfaceEnergyShadowError> {
    let bytes = serde_json::to_vec(value).map_err(|_| {
        LandSurfaceEnergyShadowError::Identity("V2 physical energy receipt serialization")
    })?;
    Sha256Digest::try_new(format!("{:x}", Sha256::digest(bytes)))
        .map_err(LandSurfaceEnergyShadowError::from)
}

fn apply_production_infiltration(
    owner: &RealHydrologyShadowAdapter,
    scope: &ReceiverFailureScope<'_>,
    ending_frame: &mut DirectRunFrame,
    infiltration_m_by_lane: BTreeMap<usize, (f64, &DirectSurfaceLiquidParcelReceipt)>,
) -> Result<(), LandSurfaceEnergyShadowError> {
    for (lane_index, (infiltration_m, receipt)) in infiltration_m_by_lane {
        let failure =
            |code, detail| scope.failure(code, owner.hydrology_owner_id(), receipt, detail);
        let lane = ending_frame.lanes.get_mut(lane_index).ok_or_else(|| {
            failure(
                DirectSurfaceLiquidErrorCode::E010,
                "infiltration production lane receiver",
            )
        })?;
        let day = owner
            .beginning_day_frames()
            .get(lane_index)
            .ok_or_else(|| {
                failure(
                    DirectSurfaceLiquidErrorCode::E010,
                    "infiltration production day receiver",
                )
            })?;
        apply_direct_same_pass_infiltration(
            &mut lane.subsurface_layers,
            infiltration_m,
            day.percolation_inputs.tillage_depth_m,
        )
        .map_err(|_| {
            failure(
                DirectSurfaceLiquidErrorCode::E003,
                "production infiltration receiver",
            )
        })?;
        lane.water.soil_water_m = aggregate_direct_soil_water(
            &lane.subsurface_layers,
            "land_surface_energy_shadow.ingress_soil_water",
        )
        .map_err(|_| {
            failure(
                DirectSurfaceLiquidErrorCode::E003,
                "ingress soil reconstruction",
            )
        })?;
    }
    Ok(())
}

#[allow(clippy::too_many_arguments)]
fn freeze_real_receiver_closure_operands(
    owner: &RealHydrologyShadowAdapter,
    configuration: &DirectSurfaceLiquidConfiguration,
    receiver_expectations: &UnifiedReceiverExpectations,
    ingress: &DirectSurfaceLiquidIngressCandidate,
    beginning_frame: &DirectRunFrame,
    ending_frame: &DirectRunFrame,
    beginning_lse_tiles: &[TileState],
    ending_lse_tiles: &[TileState],
    beginning_soil_thermal: &[SoilThermalTileCandidate],
    ending_soil_thermal: &[SoilThermalTileCandidate],
    rollback_hashes: &[OwnerRollbackHash],
    beginning_hydrology_snapshot_sha256: &Sha256Digest,
) -> Result<RealReceiverClosureOperands, LandSurfaceEnergyShadowError> {
    let amounts = receiver_preflight::aggregate_receiver_receipts(
        ingress.receipts(),
        |owner, receipt, detail| {
            let owner_id = match owner {
                receiver_preflight::ReceiptAggregationOwner::SurfaceLiquid => {
                    &configuration.owner_id
                }
                receiver_preflight::ReceiptAggregationOwner::LandSurfaceEnergy => {
                    &receiver_expectations.lse_owner_id
                }
                receiver_preflight::ReceiptAggregationOwner::SoilThermal => {
                    &receiver_expectations.soil_thermal_owner_id
                }
            };
            let owner_kind = match owner {
                receiver_preflight::ReceiptAggregationOwner::SurfaceLiquid => None,
                receiver_preflight::ReceiptAggregationOwner::LandSurfaceEnergy => {
                    Some(OwnerKind::LandSurfaceEnergy)
                }
                receiver_preflight::ReceiptAggregationOwner::SoilThermal => {
                    Some(OwnerKind::SoilThermal)
                }
            };
            receiver_phase_arithmetic_failure(
                ingress.transaction_id(),
                owner_kind,
                owner_id,
                receipt,
                rollback_hashes,
                &ingress.ending_state().state_sha256,
                detail,
            )
        },
    )?;
    let production_soil = freeze_production_soil_receivers(
        owner,
        configuration,
        beginning_frame,
        ending_frame,
        &amounts.infiltration_m_by_ofe,
    )?;
    let (soil_thermal, lse_tiles) = freeze_energy_receivers(
        configuration,
        beginning_lse_tiles,
        ending_lse_tiles,
        beginning_soil_thermal,
        ending_soil_thermal,
        &amounts.infiltration_enthalpy_by_tile,
        &amounts.retained_enthalpy_by_tile,
    )?;
    let (expected_production_soil, expected_soil_thermal, expected_lse_tiles) =
        receiver_validation::expected_receiver_identities(configuration);
    let configured_surface_context = configuration
        .records
        .iter()
        .map(|record| {
            (
                record.key.ofe_id.clone(),
                record.key.tile_id.clone(),
                record.key.surface_id.clone(),
                record.key.source_id.clone(),
            )
        })
        .collect();
    Ok(RealReceiverClosureOperands {
        transaction_id: ingress.transaction_id(),
        hydrology_owner_id: owner.hydrology_owner_id().clone(),
        lse_owner_id: receiver_expectations.lse_owner_id.clone(),
        soil_thermal_owner_id: receiver_expectations.soil_thermal_owner_id.clone(),
        beginning_hydrology_snapshot_sha256: beginning_hydrology_snapshot_sha256.clone(),
        beginning_lse_state_sha256: receiver_expectations.beginning_lse_state_sha256.clone(),
        beginning_soil_thermal_state_sha256: receiver_expectations
            .beginning_soil_thermal_state_sha256
            .clone(),
        rollback_hashes: rollback_hashes.to_vec(),
        production_soil,
        soil_thermal,
        lse_tiles,
        expected_production_soil,
        expected_soil_thermal,
        expected_lse_tiles,
        configured_surface_context,
    })
}

fn freeze_production_soil_receivers(
    owner: &RealHydrologyShadowAdapter,
    configuration: &DirectSurfaceLiquidConfiguration,
    beginning_frame: &DirectRunFrame,
    ending_frame: &DirectRunFrame,
    infiltration_m_by_ofe: &receiver_preflight::OfeAmountMap,
) -> Result<Vec<ProductionSoilReceiverOperands>, LandSurfaceEnergyShadowError> {
    let mut production_soil = Vec::with_capacity(configuration.ofe_bindings.len());
    for binding in &configuration.ofe_bindings {
        let beginning_lane = beginning_frame
            .lanes
            .get(binding.production_lane_index)
            .ok_or(LandSurfaceEnergyShadowError::Identity(
                "receiver closure beginning lane",
            ))?;
        let ending_lane = ending_frame
            .lanes
            .get(binding.production_lane_index)
            .ok_or(LandSurfaceEnergyShadowError::Identity(
                "receiver closure ending lane",
            ))?;
        let day = owner
            .beginning_day_frames()
            .get(binding.production_lane_index)
            .ok_or(LandSurfaceEnergyShadowError::Identity(
                "receiver closure production day",
            ))?;
        if beginning_lane.lane_id != binding.production_lane_id
            || ending_lane.lane_id != binding.production_lane_id
            || beginning_lane.subsurface_layers.len() != binding.ordered_soil_layer_ids.len()
            || ending_lane.subsurface_layers.len() != binding.ordered_soil_layer_ids.len()
        {
            return Err(LandSurfaceEnergyShadowError::Identity(
                "receiver closure lane/layer identity",
            ));
        }
        let ordered_layers = binding
            .ordered_soil_layer_ids
            .iter()
            .zip(&beginning_lane.subsurface_layers)
            .zip(&ending_lane.subsurface_layers)
            .map(
                |((layer_id, beginning), ending)| ProductionSoilLayerReceiverOperands {
                    layer_id: layer_id.clone(),
                    beginning_liquid_m: beginning.theta_m,
                    ending_liquid_m: ending.theta_m,
                    layer_depth_m: beginning.depth_m,
                    residual_theta: beginning.residual_theta,
                    frozen_depth_m: beginning.frozen_depth_m,
                },
            )
            .collect();
        production_soil.push(ProductionSoilReceiverOperands {
            ofe_id: binding.ofe_id.clone(),
            production_lane_index: binding.production_lane_index,
            production_lane_id: binding.production_lane_id,
            tillage_depth_m: day.percolation_inputs.tillage_depth_m,
            infiltration_m: infiltration_m_by_ofe
                .get(&binding.ofe_id)
                .copied()
                .unwrap_or(0.0),
            beginning_aggregate_soil_water_m: beginning_lane.water.soil_water_m,
            ending_aggregate_soil_water_m: ending_lane.water.soil_water_m,
            ordered_layers,
        });
    }
    Ok(production_soil)
}

#[allow(clippy::too_many_arguments)]
fn freeze_energy_receivers(
    configuration: &DirectSurfaceLiquidConfiguration,
    beginning_lse_tiles: &[TileState],
    ending_lse_tiles: &[TileState],
    beginning_soil_thermal: &[SoilThermalTileCandidate],
    ending_soil_thermal: &[SoilThermalTileCandidate],
    infiltration_enthalpy_by_tile: &receiver_preflight::TileAmountMap,
    retained_enthalpy_by_tile: &receiver_preflight::TileAmountMap,
) -> Result<
    (
        Vec<SoilThermalReceiverOperands>,
        Vec<LseTileReceiverOperands>,
    ),
    LandSurfaceEnergyShadowError,
> {
    let mut soil_thermal = Vec::new();
    let mut lse_tiles = Vec::new();
    for record in &configuration.records {
        let tile_key = (record.key.ofe_id.clone(), record.key.tile_id.clone());
        let beginning_thermal = beginning_soil_thermal
            .iter()
            .find(|candidate| {
                candidate.ofe_id == record.key.ofe_id && candidate.tile_id == record.key.tile_id
            })
            .ok_or(LandSurfaceEnergyShadowError::Identity(
                "receiver closure beginning soil thermal",
            ))?;
        let ending_thermal = ending_soil_thermal
            .iter()
            .find(|candidate| {
                candidate.ofe_id == record.key.ofe_id && candidate.tile_id == record.key.tile_id
            })
            .ok_or(LandSurfaceEnergyShadowError::Identity(
                "receiver closure ending soil thermal",
            ))?;
        let binding = configuration
            .ofe_bindings
            .iter()
            .find(|binding| binding.ofe_id == record.key.ofe_id)
            .ok_or(LandSurfaceEnergyShadowError::Identity(
                "receiver closure OFE binding",
            ))?;
        let beginning_layer = beginning_thermal
            .layers
            .iter()
            .find(|layer| layer.layer_id == binding.infiltration_soil_thermal_layer_id)
            .ok_or(LandSurfaceEnergyShadowError::Identity(
                "receiver closure beginning thermal layer",
            ))?;
        let ending_layer = ending_thermal
            .layers
            .iter()
            .find(|layer| layer.layer_id == binding.infiltration_soil_thermal_layer_id)
            .ok_or(LandSurfaceEnergyShadowError::Identity(
                "receiver closure ending thermal layer",
            ))?;
        soil_thermal.push(SoilThermalReceiverOperands {
            ofe_id: record.key.ofe_id.clone(),
            tile_id: record.key.tile_id.clone(),
            layer_id: binding.infiltration_soil_thermal_layer_id.clone(),
            beginning_infiltration_credit_j_m2_ofe_ground: beginning_layer
                .infiltration_enthalpy_credit_j_m2_ofe_ground,
            ending_infiltration_credit_j_m2_ofe_ground: ending_layer
                .infiltration_enthalpy_credit_j_m2_ofe_ground,
            beginning_enthalpy_j_m2_ofe_ground: beginning_layer.ending_enthalpy_j_m2_ofe_ground,
            infiltration_enthalpy_j_m2_ofe_ground: infiltration_enthalpy_by_tile
                .get(&tile_key)
                .copied()
                .unwrap_or(0.0),
            ending_enthalpy_j_m2_ofe_ground: ending_layer.ending_enthalpy_j_m2_ofe_ground,
        });

        let beginning_lse = beginning_lse_tiles
            .iter()
            .find(|tile| tile.ofe_id == record.key.ofe_id && tile.tile_id == record.key.tile_id)
            .ok_or(LandSurfaceEnergyShadowError::Identity(
                "receiver closure beginning LSE tile",
            ))?;
        let ending_lse = ending_lse_tiles
            .iter()
            .find(|tile| tile.ofe_id == record.key.ofe_id && tile.tile_id == record.key.tile_id)
            .ok_or(LandSurfaceEnergyShadowError::Identity(
                "receiver closure ending LSE tile",
            ))?;
        lse_tiles.push(LseTileReceiverOperands {
            ofe_id: record.key.ofe_id.clone(),
            tile_id: record.key.tile_id.clone(),
            tile_fraction: record.tile_fraction,
            beginning_enthalpy_j_m2_tile_ground: beginning_lse.surface_enthalpy_j_m2_tile_ground,
            retained_enthalpy_j_m2_ofe_ground: retained_enthalpy_by_tile
                .get(&tile_key)
                .copied()
                .unwrap_or(0.0),
            ending_enthalpy_j_m2_tile_ground: ending_lse.surface_enthalpy_j_m2_tile_ground,
        });
    }
    Ok((soil_thermal, lse_tiles))
}

pub use receiver_validation::validate_real_receiver_closure;
fn partition_finalized_uses(
    arbitration: &UnifiedRealHydrologyArbitration,
    finalized_uses: &[WaterAmount],
) -> Result<(Vec<MixedRealHydrologyUse>, Vec<WaterAmount>), LandSurfaceEnergyShadowError> {
    let soil_sources = arbitration
        .soil
        .requests
        .iter()
        .map(|row| (row.request.key.clone(), row.source.clone()))
        .collect::<BTreeMap<_, _>>();
    let surface_keys = arbitration
        .requests
        .iter()
        .filter(|row| !soil_sources.contains_key(&row.key))
        .map(|row| row.key.clone())
        .collect::<BTreeSet<_>>();
    let mut seen = BTreeSet::new();
    let mut soil = Vec::new();
    let mut surface = Vec::new();
    for row in finalized_uses {
        if !seen.insert(row.key.clone()) {
            return Err(LandSurfaceEnergyShadowError::Identity(
                "duplicate unified finalized use",
            ));
        }
        if let Some(source) = soil_sources.get(&row.key) {
            soil.push(MixedRealHydrologyUse {
                finalized_use: row.clone(),
                source: source.clone(),
            });
        } else if surface_keys.contains(&row.key) {
            surface.push(row.clone());
        } else {
            return Err(LandSurfaceEnergyShadowError::Identity(
                "unknown unified finalized use",
            ));
        }
    }
    if seen.len() != arbitration.requests.len() {
        return Err(LandSurfaceEnergyShadowError::Identity(
            "incomplete unified finalized use",
        ));
    }
    Ok((soil, surface))
}

#[cfg(test)]
mod v39_soil_energy_transaction_authority_tests {
    use super::*;
    use openwepp_land_surface_energy::{
        ExactDyadicEnthalpy, SoilThermalCandidateBeginningIdentity,
        SoilThermalLayerCandidate,
    };

    fn digest(character: char) -> Sha256Digest {
        Sha256Digest::try_new(character.to_string().repeat(64)).expect("digest")
    }

    fn v2_candidate(
        transaction_id: TransactionId,
        support_start_ns: u128,
        support_end_ns: u128,
        suffix: &str,
    ) -> SoilThermalTileCandidate {
        SoilThermalTileCandidate {
            owner_id: ResourceOwnerId::try_new(format!("soil-owner-{suffix}"))
                .expect("soil owner"),
            beginning_state_sha256: digest('1'),
            beginning_identity: SoilThermalCandidateBeginningIdentity::V2 {
                owner_tag: format!("soil-owner-{suffix}"),
                schema_sha256: digest('2'),
                exact_carry_definition_sha256: digest('3'),
                parent_v1_state_sha256: digest('4'),
                contract_version: 2,
                model_version: "v39-test".to_owned(),
                model_definition_sha256: digest('5'),
                run_id: "v39-test".to_owned(),
                configuration_sha256: digest('6'),
                transaction_id,
                expected_predecessor_transaction_id: Some(TransactionId(
                    transaction_id.0.saturating_sub(1),
                )),
                support_start_ns,
                support_end_ns,
                receipt_chain_sha256: digest('7'),
            },
            ofe_id: OfeId::try_new(format!("ofe-{suffix}")).expect("OFE"),
            tile_id: TileId::try_new(format!("tile-{suffix}")).expect("tile"),
            layers: vec![SoilThermalLayerCandidate {
                layer_id: SoilLayerId::try_new(format!("layer-{suffix}")).expect("layer"),
                beginning_enthalpy_j_m2_ofe_ground: 10.0,
                beginning_enthalpy_carry: ExactDyadicEnthalpy::zero(),
                ground_heat_credit_j_m2_ofe_ground: 0.0,
                infiltration_enthalpy_credit_j_m2_ofe_ground: 0.0,
                ending_enthalpy_j_m2_ofe_ground: 11.0,
                ending_temperature_k: 273.15,
            }],
        }
    }

    fn v1_candidate(suffix: &str) -> SoilThermalTileCandidate {
        SoilThermalTileCandidate {
            owner_id: ResourceOwnerId::try_new(format!("soil-owner-{suffix}"))
                .expect("soil owner"),
            beginning_state_sha256: digest('8'),
            beginning_identity: SoilThermalCandidateBeginningIdentity::V1 {
                configuration_sha256: digest('9'),
                last_accepted_transaction_id: Some(TransactionId(41)),
            },
            ofe_id: OfeId::try_new(format!("ofe-{suffix}")).expect("OFE"),
            tile_id: TileId::try_new(format!("tile-{suffix}")).expect("tile"),
            layers: vec![SoilThermalLayerCandidate {
                layer_id: SoilLayerId::try_new(format!("layer-{suffix}")).expect("layer"),
                beginning_enthalpy_j_m2_ofe_ground: 10.0,
                beginning_enthalpy_carry: ExactDyadicEnthalpy::zero(),
                ground_heat_credit_j_m2_ofe_ground: 0.0,
                infiltration_enthalpy_credit_j_m2_ofe_ground: 0.0,
                ending_enthalpy_j_m2_ofe_ground: 11.0,
                ending_temperature_k: 273.15,
            }],
        }
    }

    #[test]
    fn v39_legacy_v1_candidates_retain_exact_single_transaction_operand_identity() {
        let candidate = v1_candidate("legacy");
        let authority = PhysicalSoilEnergyTransactionAuthorityV2::
            try_from_pre_ingress_candidates(
                TransactionId(42),
                1_800,
                1_860,
                std::slice::from_ref(&candidate),
            )
            .expect("legacy V1 authority");
        assert_eq!(authority.source_transaction_id, TransactionId(42));
        assert_eq!(authority.soil_thermal_transaction_id, TransactionId(42));
        assert_eq!(
            authority.beginning_posture,
            PhysicalSoilEnergyBeginningPostureV2::LegacyV1
        );
        let lse_owner = ResourceOwnerId::try_new("legacy-lse-owner").expect("LSE owner");
        let mut operands = Vec::new();
        append_v2_soil_internal_operands(
            authority,
            1_800,
            1_860,
            &lse_owner,
            std::slice::from_ref(&candidate),
            &mut operands,
        )
        .expect("legacy operand");
        let layer = &candidate.layers[0];
        let expected = v2_physical_operand_digest(&(
            "OPENWEPP_ACCEPTED_SOIL_INTERNAL_ENERGY_V2",
            TransactionId(42),
            1_800_u128,
            1_860_u128,
            &lse_owner,
            &candidate.owner_id,
            &candidate.beginning_state_sha256,
            &candidate.ofe_id,
            &candidate.tile_id,
            &layer.layer_id,
            0_u32,
            layer.beginning_enthalpy_j_m2_ofe_ground,
            layer.ending_enthalpy_j_m2_ofe_ground,
            1.0_f64,
        ))
        .expect("legacy digest oracle");
        assert_eq!(operands[0].debit_credit_identity_sha256, expected);
    }

    #[test]
    fn v39_legacy_v1_refuses_split_target_mixed_posture_or_identity_substitution() {
        let candidate = v1_candidate("legacy-poison");
        let split = PhysicalSoilEnergyTransactionAuthorityV2::try_new(
            TransactionId(42),
            TransactionId(43),
        )
        .expect("native split authority");
        assert!(split
            .validate_pre_ingress_candidates(
                1_800,
                1_860,
                std::slice::from_ref(&candidate),
            )
            .is_err());

        let mut mixed = v2_candidate(TransactionId(42), 1_800, 1_860, "mixed");
        mixed.owner_id = candidate.owner_id.clone();
        mixed.beginning_state_sha256 = candidate.beginning_state_sha256.clone();
        assert!(PhysicalSoilEnergyTransactionAuthorityV2::
            try_from_pre_ingress_candidates(
                TransactionId(42),
                1_800,
                1_860,
                &[candidate.clone(), mixed],
            )
            .is_err());

        let mut changed_identity = candidate.clone();
        changed_identity.beginning_identity = SoilThermalCandidateBeginningIdentity::V1 {
            configuration_sha256: digest('0'),
            last_accepted_transaction_id: Some(TransactionId(41)),
        };
        assert!(PhysicalSoilEnergyTransactionAuthorityV2::
            try_from_pre_ingress_candidates(
                TransactionId(42),
                1_800,
                1_860,
                &[candidate, changed_identity],
            )
            .is_err());
    }

    #[test]
    fn v39_physical_soil_energy_operands_bind_outer_source_and_soil_target_transactions() {
        let candidate = v2_candidate(TransactionId(43), 1_860, 1_980, "bind");
        let authority = PhysicalSoilEnergyTransactionAuthorityV2::
            try_from_pre_ingress_candidates(
                TransactionId(42),
                1_860,
                1_980,
                std::slice::from_ref(&candidate),
            )
            .expect("split authority");
        assert_eq!(authority.source_transaction_id, TransactionId(42));
        assert_eq!(authority.soil_thermal_transaction_id, TransactionId(43));

        let lse_owner = ResourceOwnerId::try_new("lse-owner").expect("LSE owner");
        let mut exact = Vec::new();
        append_v2_soil_internal_operands(
            authority,
            1_860,
            1_980,
            &lse_owner,
            std::slice::from_ref(&candidate),
            &mut exact,
        )
        .expect("exact operand");
        let mut changed_source = Vec::new();
        append_v2_soil_internal_operands(
            PhysicalSoilEnergyTransactionAuthorityV2::try_new(
                TransactionId(44),
                TransactionId(43),
            )
            .expect("changed source authority"),
            1_860,
            1_980,
            &lse_owner,
            std::slice::from_ref(&candidate),
            &mut changed_source,
        )
        .expect("changed source operand");
        let mut changed_target = Vec::new();
        append_v2_soil_internal_operands(
            PhysicalSoilEnergyTransactionAuthorityV2::try_new(
                TransactionId(42),
                TransactionId(44),
            )
            .expect("changed target authority"),
            1_860,
            1_980,
            &lse_owner,
            std::slice::from_ref(&candidate),
            &mut changed_target,
        )
        .expect("changed target operand");
        assert_ne!(
            exact[0].debit_credit_identity_sha256,
            changed_source[0].debit_credit_identity_sha256
        );
        assert_ne!(
            exact[0].debit_credit_identity_sha256,
            changed_target[0].debit_credit_identity_sha256
        );
    }

    #[test]
    fn v39_second_child_soil_operands_keep_outer_ingress_transaction() {
        let first = v2_candidate(TransactionId(42), 1_800, 1_860, "first");
        let second = v2_candidate(TransactionId(43), 1_860, 1_980, "second");
        let first_authority = PhysicalSoilEnergyTransactionAuthorityV2::
            try_from_pre_ingress_candidates(
                TransactionId(42),
                1_800,
                1_860,
                std::slice::from_ref(&first),
            )
            .expect("first child authority");
        let second_authority = PhysicalSoilEnergyTransactionAuthorityV2::
            try_from_pre_ingress_candidates(
                TransactionId(42),
                1_860,
                1_980,
                std::slice::from_ref(&second),
            )
            .expect("second child authority");
        assert_eq!(first_authority.source_transaction_id, TransactionId(42));
        assert_eq!(first_authority.soil_thermal_transaction_id, TransactionId(42));
        assert_eq!(second_authority.source_transaction_id, TransactionId(42));
        assert_eq!(second_authority.soil_thermal_transaction_id, TransactionId(43));
    }

    #[test]
    fn v39_soil_operand_transaction_substitution_refuses_without_publication() {
        let candidate = v2_candidate(TransactionId(43), 1_860, 1_980, "poison");
        let substituted = PhysicalSoilEnergyTransactionAuthorityV2::try_new(
            TransactionId(42),
            TransactionId(44),
        )
        .expect("substituted authority");
        assert!(substituted
            .validate_pre_ingress_candidates(
                1_860,
                1_980,
                std::slice::from_ref(&candidate),
            )
            .is_err());
        assert!(PhysicalSoilEnergyTransactionAuthorityV2::
            try_from_pre_ingress_candidates(
                TransactionId(42),
                1_800,
                1_980,
                std::slice::from_ref(&candidate),
            )
            .is_err());
        let foreign = v2_candidate(TransactionId(44), 1_860, 1_980, "foreign");
        assert!(PhysicalSoilEnergyTransactionAuthorityV2::
            try_from_pre_ingress_candidates(
                TransactionId(42),
                1_860,
                1_980,
                &[candidate.clone(), foreign],
            )
            .is_err());
        let mut foreign_owner = candidate.clone();
        foreign_owner.owner_id =
            ResourceOwnerId::try_new("foreign-owner").expect("foreign owner");
        assert!(PhysicalSoilEnergyTransactionAuthorityV2::
            try_from_pre_ingress_candidates(
                TransactionId(42),
                1_860,
                1_980,
                &[candidate, foreign_owner],
            )
            .is_err());
        assert!(PhysicalSoilEnergyTransactionAuthorityV2::try_new(
            TransactionId(0),
            TransactionId(43),
        )
        .is_err());
        assert!(PhysicalSoilEnergyTransactionAuthorityV2::try_new(
            TransactionId(42),
            TransactionId(0),
        )
        .is_err());
    }
}
