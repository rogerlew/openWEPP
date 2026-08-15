//! Public unified-entry precedence and identity-envelope validation.

use std::collections::BTreeMap;

use crate::direct_runtime::{
    preflight_surface_liquid_ingress_input_identities, surface_liquid_raw_snapshot_attempt_sha256,
};

use super::{
    DirectSurfaceLiquidConfiguration, DirectSurfaceLiquidError, DirectSurfaceLiquidErrorCode,
    DirectSurfaceLiquidErrorContext, DirectSurfaceLiquidIngressInput, DirectSurfaceLiquidPhase,
    DirectSurfaceLiquidRollbackHashes, FramedSha256, GroundWaterKey,
    LandSurfaceEnergyRealHydrologyAdapter, LandSurfaceEnergyShadowError, MixedRealHydrologyRequest,
    PotentialWaterRequestBatch, RealHydrologySourceKey, Sha256Digest, UnifiedReceiverExpectations,
    WaterAmount, compose_unified_beginning_hydrology_snapshot_sha256, partition_requests,
    preflight_request_bounds, preflight_request_cardinality, preflight_request_domains,
    preflight_request_identities, protocol_error_code_and_detail, receiver_expectations_sha256,
    request_failure, shadow_error_code, snapshot_failure,
    unified_beginning_hydrology_snapshot_sha256, validate_native_shadow_exact_one_custody,
    validate_native_shadow_supported_domain, validate_receiver_expectations,
    validate_surface_production_binding, water_request_batch_sha256,
};

pub(super) struct UnifiedEntryPreflight {
    pub(super) actual_snapshot: Sha256Digest,
    pub(super) attempted_sha256: String,
    pub(super) soil_requests: Vec<MixedRealHydrologyRequest>,
    pub(super) surface_requests: Vec<WaterAmount>,
}

pub(super) fn validate_unified_entry(
    soil_adapter: &LandSurfaceEnergyRealHydrologyAdapter<'_>,
    configuration: &DirectSurfaceLiquidConfiguration,
    receiver_expectations: &UnifiedReceiverExpectations,
    request_batch: &PotentialWaterRequestBatch,
    soil_sources: &BTreeMap<GroundWaterKey, RealHydrologySourceKey>,
    ingress: &DirectSurfaceLiquidIngressInput,
    expected_snapshot: &Sha256Digest,
) -> Result<UnifiedEntryPreflight, LandSurfaceEnergyShadowError> {
    let (actual_snapshot, attempted_sha256, soil_requests, surface_requests) =
        preflight_unified_entry_identity_envelope(
            soil_adapter,
            configuration,
            receiver_expectations,
            request_batch,
            soil_sources,
            ingress,
            expected_snapshot,
        )?;
    unified_beginning_hydrology_snapshot_sha256(soil_adapter, configuration)
        .map_err(|error| complete_unified_failure(error, &actual_snapshot, &attempted_sha256))?;
    if !ingress.interval_s.is_finite() {
        return Err(unified_entry_failure(
            DirectSurfaceLiquidErrorCode::E003,
            soil_adapter,
            configuration,
            receiver_expectations,
            request_batch,
            soil_sources,
            ingress,
            &actual_snapshot,
            expected_snapshot,
            "nonfinite ingress interval",
        )
        .into());
    }
    preflight_request_domains(request_batch, &actual_snapshot)
        .map_err(|error| complete_unified_failure(error, &actual_snapshot, &attempted_sha256))?;
    validate_native_shadow_supported_domain(
        soil_adapter.owner,
        configuration,
        &actual_snapshot,
        &attempted_sha256,
    )?;
    preflight_request_cardinality(request_batch, &actual_snapshot)
        .map_err(|error| complete_unified_failure(error, &actual_snapshot, &attempted_sha256))?;
    preflight_request_bounds(request_batch, &actual_snapshot)
        .map_err(|error| complete_unified_failure(error, &actual_snapshot, &attempted_sha256))?;
    if let Err(error) = request_batch.validate() {
        let (code, detail) = protocol_error_code_and_detail(&error);
        return Err(complete_unified_failure(
            request_failure(code, request_batch, &actual_snapshot, None, detail),
            &actual_snapshot,
            &attempted_sha256,
        ));
    }
    validate_native_shadow_exact_one_custody(
        soil_adapter.owner,
        configuration,
        &actual_snapshot,
        &attempted_sha256,
    )?;
    if ingress.day_index != soil_adapter.owner.day_index()
        || ingress.interval_s.to_bits() != soil_adapter.owner.interval_s().to_bits()
    {
        return Err(unified_entry_failure(
            DirectSurfaceLiquidErrorCode::E008,
            soil_adapter,
            configuration,
            receiver_expectations,
            request_batch,
            soil_sources,
            ingress,
            &actual_snapshot,
            expected_snapshot,
            "unified ingress cadence or continuation",
        )
        .into());
    }
    validate_receiver_expectations(
        soil_adapter.owner,
        configuration,
        receiver_expectations,
        request_batch,
        &actual_snapshot,
    )
    .map_err(|error| complete_unified_failure(error, &actual_snapshot, &attempted_sha256))?;
    Ok(UnifiedEntryPreflight {
        actual_snapshot,
        attempted_sha256,
        soil_requests,
        surface_requests,
    })
}

#[allow(clippy::too_many_lines)]
fn preflight_unified_entry_identity_envelope(
    soil_adapter: &LandSurfaceEnergyRealHydrologyAdapter<'_>,
    configuration: &DirectSurfaceLiquidConfiguration,
    receiver_expectations: &UnifiedReceiverExpectations,
    request_batch: &PotentialWaterRequestBatch,
    soil_sources: &BTreeMap<GroundWaterKey, RealHydrologySourceKey>,
    ingress: &DirectSurfaceLiquidIngressInput,
    expected_snapshot: &Sha256Digest,
) -> Result<
    (
        Sha256Digest,
        String,
        Vec<MixedRealHydrologyRequest>,
        Vec<WaterAmount>,
    ),
    LandSurfaceEnergyShadowError,
> {
    let provisional_unified_attempt = unified_entry_attempt_sha256(
        soil_adapter,
        request_batch,
        soil_sources,
        ingress,
        receiver_expectations,
        expected_snapshot,
        expected_snapshot,
    );
    configuration
        .preflight_schema_and_identities()
        .map_err(|error| {
            join_raw_and_unified_attempt(
                snapshot_failure(
                    error.code(),
                    soil_adapter.owner,
                    configuration,
                    "invalid surface-liquid configuration identity envelope",
                ),
                &provisional_unified_attempt,
            )
        })?;
    let surface_state = soil_adapter
        .owner
        .beginning_frame()
        .surface_liquid_shadow
        .as_deref()
        .ok_or_else(|| {
            join_raw_and_unified_attempt(
                snapshot_failure(
                    DirectSurfaceLiquidErrorCode::E002,
                    soil_adapter.owner,
                    configuration,
                    "missing beginning surface-liquid owner",
                ),
                &provisional_unified_attempt,
            )
        })?;
    surface_state
        .preflight_schema_and_identities(configuration)
        .map_err(|error| {
            join_raw_and_unified_attempt(
                snapshot_failure(
                    error.code(),
                    soil_adapter.owner,
                    configuration,
                    "invalid beginning surface-liquid identity envelope",
                ),
                &provisional_unified_attempt,
            )
        })?;
    validate_surface_production_binding(soil_adapter.owner, configuration)
        .map_err(|error| join_raw_and_unified_attempt(error, &provisional_unified_attempt))?;
    if &configuration.owner_id != soil_adapter.owner.hydrology_owner_id() {
        return Err(join_raw_and_unified_attempt(
            snapshot_failure(
                DirectSurfaceLiquidErrorCode::E002,
                soil_adapter.owner,
                configuration,
                "mixed unified hydrology owner",
            ),
            &provisional_unified_attempt,
        ));
    }
    let actual_snapshot = compose_unified_beginning_hydrology_snapshot_sha256(
        soil_adapter,
        configuration,
        surface_state,
    )?;
    let unified_attempt = unified_entry_attempt_sha256(
        soil_adapter,
        request_batch,
        soil_sources,
        ingress,
        receiver_expectations,
        &actual_snapshot,
        expected_snapshot,
    );
    let raw_attempt = surface_liquid_raw_snapshot_attempt_sha256(
        soil_adapter.owner.snapshot_bytes(),
        configuration,
        Some(surface_state),
    );
    let attempted_sha256 = raw_and_unified_attempt_sha256(&raw_attempt, &unified_attempt);
    preflight_request_identities(request_batch, &actual_snapshot)
        .map_err(|error| complete_unified_failure(error, &actual_snapshot, &attempted_sha256))?;
    if let Err(error) = preflight_surface_liquid_ingress_input_identities(configuration, ingress) {
        return Err(contextualize_ingress_identity_failure(
            &error,
            soil_adapter,
            configuration,
            receiver_expectations,
            request_batch,
            soil_sources,
            ingress,
            &actual_snapshot,
            expected_snapshot,
        )
        .into());
    }
    let (soil_requests, surface_requests) =
        partition_requests(request_batch, soil_sources, configuration, &actual_snapshot).map_err(
            |error| complete_unified_failure(error, &actual_snapshot, &attempted_sha256),
        )?;
    if request_batch.transaction_id.0 == 0
        || request_batch.transaction_id != soil_adapter.owner.transaction_id()
        || ingress.transaction_id != request_batch.transaction_id
        || &actual_snapshot != expected_snapshot
    {
        return Err(unified_entry_failure(
            DirectSurfaceLiquidErrorCode::E002,
            soil_adapter,
            configuration,
            receiver_expectations,
            request_batch,
            soil_sources,
            ingress,
            &actual_snapshot,
            expected_snapshot,
            "unified transaction or beginning snapshot identity",
        )
        .into());
    }
    configuration.validate().map_err(|error| {
        join_raw_and_unified_attempt(
            snapshot_failure(
                error.code(),
                soil_adapter.owner,
                configuration,
                "invalid surface-liquid configuration",
            ),
            &attempted_sha256,
        )
    })?;
    surface_state.validate(configuration).map_err(|error| {
        join_raw_and_unified_attempt(
            snapshot_failure(
                error.code(),
                soil_adapter.owner,
                configuration,
                "invalid beginning surface-liquid owner",
            ),
            &attempted_sha256,
        )
    })?;
    Ok((
        actual_snapshot,
        attempted_sha256,
        soil_requests,
        surface_requests,
    ))
}

fn join_raw_and_unified_attempt(
    error: LandSurfaceEnergyShadowError,
    unified_attempt: &str,
) -> LandSurfaceEnergyShadowError {
    let LandSurfaceEnergyShadowError::SurfaceLiquid(surface_error) = error else {
        return error;
    };
    let Some(failure) = surface_error.failure() else {
        return LandSurfaceEnergyShadowError::SurfaceLiquid(surface_error);
    };
    let attempted_owner_sha256 = failure
        .rollback
        .attempted_owner_sha256
        .as_deref()
        .map_or_else(
            || unified_attempt.to_owned(),
            |raw_attempt| raw_and_unified_attempt_sha256(raw_attempt, unified_attempt),
        );
    DirectSurfaceLiquidError::canonical_failure(
        failure.code,
        failure.phase,
        failure.context.clone(),
        DirectSurfaceLiquidRollbackHashes {
            beginning_owner_sha256: failure.rollback.beginning_owner_sha256.clone(),
            attempted_owner_sha256: Some(attempted_owner_sha256),
        },
        failure.detail.clone(),
    )
    .into()
}

fn raw_and_unified_attempt_sha256(raw_attempt: &str, unified_attempt: &str) -> String {
    let mut joined = FramedSha256::new("openwepp-unified-entry-raw-attempt-join-v1");
    joined.string("raw_attempt", raw_attempt);
    joined.string("unified_attempt", unified_attempt);
    joined.finish()
}

pub(super) fn complete_unified_failure(
    error: LandSurfaceEnergyShadowError,
    actual_snapshot: &Sha256Digest,
    attempted_sha256: &str,
) -> LandSurfaceEnergyShadowError {
    let LandSurfaceEnergyShadowError::SurfaceLiquid(error) = error else {
        return error;
    };
    let code = error.code();
    let (phase, context, detail) = error.failure().map_or_else(
        || {
            (
                DirectSurfaceLiquidPhase::Authorization,
                DirectSurfaceLiquidErrorContext::default(),
                error.to_string(),
            )
        },
        |failure| {
            (
                failure.phase,
                failure.context.clone(),
                failure.detail.clone(),
            )
        },
    );
    DirectSurfaceLiquidError::canonical_failure(
        code,
        phase,
        context,
        DirectSurfaceLiquidRollbackHashes {
            beginning_owner_sha256: Some(actual_snapshot.to_string()),
            attempted_owner_sha256: Some(attempted_sha256.to_owned()),
        },
        detail,
    )
    .into()
}

pub(super) fn canonicalize_callback_failure(
    error: &LandSurfaceEnergyShadowError,
    transaction_id: super::TransactionId,
    actual_snapshot: &Sha256Digest,
    attempted_sha256: &str,
) -> LandSurfaceEnergyShadowError {
    let code = shadow_error_code(error);
    let (mut context, detail) = match error {
        LandSurfaceEnergyShadowError::SurfaceLiquid(error) => error.failure().map_or_else(
            || {
                (
                    DirectSurfaceLiquidErrorContext::default(),
                    error.to_string(),
                )
            },
            |failure| (failure.context.clone(), failure.detail.clone()),
        ),
        _ => (
            DirectSurfaceLiquidErrorContext::default(),
            error.to_string(),
        ),
    };
    context.transaction_id = Some(transaction_id);
    DirectSurfaceLiquidError::canonical_failure(
        code,
        DirectSurfaceLiquidPhase::ResourceCandidate,
        context,
        DirectSurfaceLiquidRollbackHashes {
            beginning_owner_sha256: Some(actual_snapshot.to_string()),
            attempted_owner_sha256: Some(attempted_sha256.to_owned()),
        },
        detail,
    )
    .into()
}

#[allow(clippy::too_many_arguments)]
fn contextualize_ingress_identity_failure(
    error: &DirectSurfaceLiquidError,
    soil_adapter: &LandSurfaceEnergyRealHydrologyAdapter<'_>,
    configuration: &DirectSurfaceLiquidConfiguration,
    receiver_expectations: &UnifiedReceiverExpectations,
    request_batch: &PotentialWaterRequestBatch,
    soil_sources: &BTreeMap<GroundWaterKey, RealHydrologySourceKey>,
    ingress: &DirectSurfaceLiquidIngressInput,
    actual_snapshot: &Sha256Digest,
    expected_snapshot: &Sha256Digest,
) -> DirectSurfaceLiquidError {
    let code = error.code();
    let (mut context, detail) = error.failure().map_or_else(
        || {
            (
                DirectSurfaceLiquidErrorContext::default(),
                error.to_string(),
            )
        },
        |failure| (failure.context.clone(), failure.detail.clone()),
    );
    context
        .transaction_id
        .get_or_insert(request_batch.transaction_id);
    context
        .owner_id
        .get_or_insert_with(|| configuration.owner_id.clone());
    DirectSurfaceLiquidError::canonical_failure(
        code,
        DirectSurfaceLiquidPhase::Authorization,
        context,
        DirectSurfaceLiquidRollbackHashes {
            beginning_owner_sha256: Some(actual_snapshot.to_string()),
            attempted_owner_sha256: Some(unified_entry_attempt_sha256(
                soil_adapter,
                request_batch,
                soil_sources,
                ingress,
                receiver_expectations,
                actual_snapshot,
                expected_snapshot,
            )),
        },
        detail,
    )
}

#[allow(clippy::too_many_arguments)]
fn unified_entry_failure(
    code: DirectSurfaceLiquidErrorCode,
    soil_adapter: &LandSurfaceEnergyRealHydrologyAdapter<'_>,
    configuration: &DirectSurfaceLiquidConfiguration,
    receiver_expectations: &UnifiedReceiverExpectations,
    request_batch: &PotentialWaterRequestBatch,
    soil_sources: &BTreeMap<GroundWaterKey, RealHydrologySourceKey>,
    ingress: &DirectSurfaceLiquidIngressInput,
    actual_snapshot: &Sha256Digest,
    expected_snapshot: &Sha256Digest,
    detail: &'static str,
) -> DirectSurfaceLiquidError {
    let unified_attempt = unified_entry_attempt_sha256(
        soil_adapter,
        request_batch,
        soil_sources,
        ingress,
        receiver_expectations,
        actual_snapshot,
        expected_snapshot,
    );
    let raw_attempt = surface_liquid_raw_snapshot_attempt_sha256(
        soil_adapter.owner.snapshot_bytes(),
        configuration,
        soil_adapter
            .owner
            .beginning_frame()
            .surface_liquid_shadow
            .as_deref(),
    );
    DirectSurfaceLiquidError::canonical_failure(
        code,
        DirectSurfaceLiquidPhase::Authorization,
        DirectSurfaceLiquidErrorContext {
            transaction_id: Some(request_batch.transaction_id),
            owner_id: Some(configuration.owner_id.clone()),
            ..DirectSurfaceLiquidErrorContext::default()
        },
        DirectSurfaceLiquidRollbackHashes {
            beginning_owner_sha256: Some(actual_snapshot.to_string()),
            attempted_owner_sha256: Some(raw_and_unified_attempt_sha256(
                &raw_attempt,
                &unified_attempt,
            )),
        },
        detail,
    )
}

fn unified_entry_attempt_sha256(
    soil_adapter: &LandSurfaceEnergyRealHydrologyAdapter<'_>,
    request_batch: &PotentialWaterRequestBatch,
    soil_sources: &BTreeMap<GroundWaterKey, RealHydrologySourceKey>,
    ingress: &DirectSurfaceLiquidIngressInput,
    receiver_expectations: &UnifiedReceiverExpectations,
    actual_snapshot: &Sha256Digest,
    expected_snapshot: &Sha256Digest,
) -> String {
    let mut attempted = FramedSha256::new("openwepp-unified-entry-v3");
    attempted.u128("request_transaction", request_batch.transaction_id.0);
    attempted.u128("owner_transaction", soil_adapter.owner.transaction_id().0);
    attempted.u128("ingress_transaction", ingress.transaction_id.0);
    attempted.u64("ingress_day", ingress.day_index as u64);
    attempted.u64("ingress_interval_index", u64::from(ingress.interval_index));
    attempted.f64("ingress_interval", ingress.interval_s);
    attempted.string(
        "water_request_batch",
        &water_request_batch_sha256(request_batch),
    );
    attempted.count("tile_ingress_count", ingress.tile_ingress.len());
    for tile in &ingress.tile_ingress {
        frame_tile_ingress(&mut attempted, tile);
    }
    attempted.count("wb14_count", ingress.wb14_parameters.len());
    for parameter in &ingress.wb14_parameters {
        attempted.string("wb14_ofe", parameter.ofe_id.as_str());
        attempted.f64("wb14_conductivity", parameter.effective_conductivity_m_s);
        attempted.f64("wb14_matric_potential", parameter.matric_potential_m);
        attempted.f64(
            "wb14_storage_capacity",
            parameter.infiltration_storage_capacity_m,
        );
    }
    attempted.count("soil_source_count", soil_sources.len());
    for (key, source) in soil_sources {
        frame_ground_water_key(&mut attempted, key);
        attempted.u64("soil_source_lane_index", source.ofe_lane.lane_index as u64);
        attempted.u64("soil_source_lane_id", u64::from(source.ofe_lane.lane_id));
        attempted.string("soil_source_layer", source.layer_id.as_str());
    }
    attempted.string(
        "expected_lse_owner",
        receiver_expectations.lse_owner_id.as_str(),
    );
    attempted.string(
        "expected_lse_beginning",
        receiver_expectations.beginning_lse_state_sha256.as_str(),
    );
    attempted.string(
        "expected_hydrology_owner",
        receiver_expectations.hydrology_owner_id.as_str(),
    );
    attempted.string(
        "expected_hydrology_beginning",
        receiver_expectations
            .beginning_hydrology_snapshot_sha256
            .as_str(),
    );
    attempted.string(
        "expected_thermal_owner",
        receiver_expectations.soil_thermal_owner_id.as_str(),
    );
    attempted.string(
        "expected_thermal_beginning",
        receiver_expectations
            .beginning_soil_thermal_state_sha256
            .as_str(),
    );
    attempted.count(
        "expected_thermal_tile_count",
        receiver_expectations.ordered_thermal_layers.len(),
    );
    for ((ofe_id, tile_id), layers) in &receiver_expectations.ordered_thermal_layers {
        attempted.string("expected_thermal_ofe", ofe_id.as_str());
        attempted.string("expected_thermal_tile", tile_id.as_str());
        attempted.count("expected_thermal_layer_count", layers.len());
        for layer_id in layers {
            attempted.string("expected_thermal_layer", layer_id.as_str());
        }
    }
    attempted.string(
        "receiver_expectations",
        &receiver_expectations_sha256(receiver_expectations),
    );
    attempted.string("actual_snapshot", actual_snapshot.as_str());
    attempted.string("expected_snapshot", expected_snapshot.as_str());
    attempted.finish()
}

fn frame_tile_ingress(out: &mut FramedSha256, tile: &crate::DirectTileGroundIngress) {
    match tile {
        crate::DirectTileGroundIngress::OpenRawPrecipitation {
            ofe_id,
            tile_id,
            surface_id,
            raw_precipitation,
        } => {
            out.string("tile_ingress_mode", "open_raw_precipitation");
            frame_ingress_identity(out, ofe_id, tile_id, surface_id);
            frame_ingress_amount(out, "raw_precipitation", raw_precipitation);
        }
        crate::DirectTileGroundIngress::CoveredCanopyRelease {
            ofe_id,
            tile_id,
            surface_id,
            release,
        } => {
            out.string("tile_ingress_mode", "covered_canopy_release");
            frame_ingress_identity(out, ofe_id, tile_id, surface_id);
            frame_ingress_amount(out, "throughfall", &release.throughfall);
            frame_ingress_amount(out, "initial_drainage", &release.initial_drainage);
            frame_ingress_amount(out, "second_drainage", &release.second_drainage);
            frame_ingress_amount(out, "stemflow", &release.stemflow);
        }
    }
}

fn frame_ingress_identity(
    out: &mut FramedSha256,
    ofe_id: &super::OfeId,
    tile_id: &super::TileId,
    surface_id: &super::SurfaceId,
) {
    out.string("tile_ingress_ofe", ofe_id.as_str());
    out.string("tile_ingress_tile", tile_id.as_str());
    out.string("tile_ingress_surface", surface_id.as_str());
}

fn frame_ingress_amount(
    out: &mut FramedSha256,
    component: &'static str,
    amount: &crate::DirectIngressAmount,
) {
    out.string("ingress_component", component);
    out.f64("ingress_mass", amount.mass_kg_m2_tile_ground);
    out.f64("ingress_temperature", amount.temperature_k);
    out.f64(
        "ingress_specific_enthalpy",
        amount.specific_liquid_enthalpy_j_kg,
    );
    out.f64("ingress_start", amount.start_s);
    out.f64("ingress_end", amount.end_s);
}

fn frame_ground_water_key(out: &mut FramedSha256, key: &GroundWaterKey) {
    out.u128("soil_map_transaction", key.transaction_id.0);
    out.string("soil_map_owner", key.requesting_owner_id.as_str());
    out.string(
        "soil_map_component",
        &format!("{:?}", key.requesting_component),
    );
    out.string("soil_map_ofe", key.ofe_id.as_str());
    out.string("soil_map_requesting_tile", key.requesting_tile_id.as_str());
    out.string(
        "soil_map_occupancy",
        key.occupancy_id
            .as_ref()
            .map_or("", super::ComponentId::as_str),
    );
    out.string(
        "soil_map_surface",
        key.surface_id.as_ref().map_or("", super::SurfaceId::as_str),
    );
    out.string(
        "soil_map_surface_class",
        &format!("{:?}", key.surface_class),
    );
    out.string("soil_map_source_type", &format!("{:?}", key.source_type));
    out.string("soil_map_source", key.source_id.as_str());
    out.string(
        "soil_map_source_tile",
        key.source_tile_id
            .as_ref()
            .map_or("", super::TileId::as_str),
    );
    out.string(
        "soil_map_layer",
        key.soil_layer_id
            .as_ref()
            .map_or("", super::SoilLayerId::as_str),
    );
    out.string("soil_map_basis", &format!("{:?}", key.amount_basis));
}
