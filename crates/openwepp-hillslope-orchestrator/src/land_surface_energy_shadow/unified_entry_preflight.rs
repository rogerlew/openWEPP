//! Public unified-entry precedence and identity-envelope validation.

use crate::direct_runtime::preflight_surface_liquid_ingress_input_identities;

use super::{
    DirectSurfaceLiquidConfiguration, DirectSurfaceLiquidError, DirectSurfaceLiquidErrorCode,
    DirectSurfaceLiquidErrorContext, DirectSurfaceLiquidIngressInput, DirectSurfaceLiquidPhase,
    DirectSurfaceLiquidRollbackHashes, FramedSha256, LandSurfaceEnergyRealHydrologyAdapter,
    LandSurfaceEnergyShadowError, PotentialWaterRequestBatch, Sha256Digest,
    compose_unified_beginning_hydrology_snapshot_sha256, preflight_request_bounds,
    preflight_request_cardinality, preflight_request_domains, preflight_request_identities,
    protocol_error_code_and_detail, request_failure, snapshot_failure,
    unified_beginning_hydrology_snapshot_sha256, validate_native_shadow_exact_one_custody,
    validate_native_shadow_supported_domain, validate_surface_production_binding,
    water_request_batch_sha256,
};

pub(super) fn validate_unified_entry(
    soil_adapter: &LandSurfaceEnergyRealHydrologyAdapter<'_>,
    configuration: &DirectSurfaceLiquidConfiguration,
    request_batch: &PotentialWaterRequestBatch,
    ingress: &DirectSurfaceLiquidIngressInput,
    expected_snapshot: &Sha256Digest,
) -> Result<(), LandSurfaceEnergyShadowError> {
    let actual_snapshot = preflight_unified_entry_identity_envelope(
        soil_adapter,
        configuration,
        request_batch,
        ingress,
        expected_snapshot,
    )?;
    unified_beginning_hydrology_snapshot_sha256(soil_adapter, configuration)?;
    if !ingress.interval_s.is_finite() {
        return Err(unified_entry_failure(
            DirectSurfaceLiquidErrorCode::E003,
            soil_adapter,
            configuration,
            request_batch,
            ingress,
            expected_snapshot,
            &actual_snapshot,
            "nonfinite ingress interval",
        )
        .into());
    }
    preflight_request_domains(request_batch, expected_snapshot)?;
    validate_native_shadow_supported_domain(
        soil_adapter.owner,
        configuration,
        expected_snapshot,
        &water_request_batch_sha256(request_batch),
    )?;
    preflight_request_cardinality(request_batch, expected_snapshot)?;
    preflight_request_bounds(request_batch, expected_snapshot)?;
    if let Err(error) = request_batch.validate() {
        let (code, detail) = protocol_error_code_and_detail(&error);
        return Err(request_failure(
            code,
            request_batch,
            expected_snapshot,
            None,
            detail,
        ));
    }
    validate_native_shadow_exact_one_custody(
        soil_adapter.owner,
        configuration,
        expected_snapshot,
        &water_request_batch_sha256(request_batch),
    )?;
    if ingress.day_index != soil_adapter.owner.day_index()
        || ingress.interval_s.to_bits() != soil_adapter.owner.interval_s().to_bits()
    {
        return Err(unified_entry_failure(
            DirectSurfaceLiquidErrorCode::E008,
            soil_adapter,
            configuration,
            request_batch,
            ingress,
            expected_snapshot,
            &actual_snapshot,
            "unified ingress cadence or continuation",
        )
        .into());
    }
    Ok(())
}

fn preflight_unified_entry_identity_envelope(
    soil_adapter: &LandSurfaceEnergyRealHydrologyAdapter<'_>,
    configuration: &DirectSurfaceLiquidConfiguration,
    request_batch: &PotentialWaterRequestBatch,
    ingress: &DirectSurfaceLiquidIngressInput,
    expected_snapshot: &Sha256Digest,
) -> Result<Sha256Digest, LandSurfaceEnergyShadowError> {
    configuration
        .preflight_schema_and_identities()
        .map_err(|error| {
            snapshot_failure(
                error.code(),
                soil_adapter.owner,
                configuration,
                "invalid surface-liquid configuration identity envelope",
            )
        })?;
    let surface_state = soil_adapter
        .owner
        .beginning_frame()
        .surface_liquid_shadow
        .as_deref()
        .ok_or_else(|| {
            snapshot_failure(
                DirectSurfaceLiquidErrorCode::E002,
                soil_adapter.owner,
                configuration,
                "missing beginning surface-liquid owner",
            )
        })?;
    surface_state
        .preflight_schema_and_identities(configuration)
        .map_err(|error| {
            snapshot_failure(
                error.code(),
                soil_adapter.owner,
                configuration,
                "invalid beginning surface-liquid identity envelope",
            )
        })?;
    preflight_request_identities(request_batch, expected_snapshot)?;
    validate_surface_production_binding(soil_adapter.owner, configuration)?;
    if &configuration.owner_id != soil_adapter.owner.hydrology_owner_id() {
        return Err(snapshot_failure(
            DirectSurfaceLiquidErrorCode::E002,
            soil_adapter.owner,
            configuration,
            "mixed unified hydrology owner",
        ));
    }
    let actual_snapshot = compose_unified_beginning_hydrology_snapshot_sha256(
        soil_adapter,
        configuration,
        surface_state,
    )?;
    if let Err(error) = preflight_surface_liquid_ingress_input_identities(configuration, ingress) {
        return Err(contextualize_ingress_identity_failure(
            &error,
            soil_adapter,
            configuration,
            request_batch,
            ingress,
            expected_snapshot,
            &actual_snapshot,
        )
        .into());
    }
    if request_batch.transaction_id.0 == 0
        || request_batch.transaction_id != soil_adapter.owner.transaction_id()
        || ingress.transaction_id != request_batch.transaction_id
        || &actual_snapshot != expected_snapshot
    {
        return Err(unified_entry_failure(
            DirectSurfaceLiquidErrorCode::E002,
            soil_adapter,
            configuration,
            request_batch,
            ingress,
            expected_snapshot,
            &actual_snapshot,
            "unified transaction or beginning snapshot identity",
        )
        .into());
    }
    Ok(actual_snapshot)
}

#[allow(clippy::too_many_arguments)]
fn contextualize_ingress_identity_failure(
    error: &DirectSurfaceLiquidError,
    soil_adapter: &LandSurfaceEnergyRealHydrologyAdapter<'_>,
    configuration: &DirectSurfaceLiquidConfiguration,
    request_batch: &PotentialWaterRequestBatch,
    ingress: &DirectSurfaceLiquidIngressInput,
    expected_snapshot: &Sha256Digest,
    actual_snapshot: &Sha256Digest,
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
            beginning_owner_sha256: Some(expected_snapshot.to_string()),
            attempted_owner_sha256: Some(unified_entry_attempt_sha256(
                soil_adapter,
                request_batch,
                ingress,
                actual_snapshot,
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
    request_batch: &PotentialWaterRequestBatch,
    ingress: &DirectSurfaceLiquidIngressInput,
    expected_snapshot: &Sha256Digest,
    actual_snapshot: &Sha256Digest,
    detail: &'static str,
) -> DirectSurfaceLiquidError {
    DirectSurfaceLiquidError::canonical_failure(
        code,
        DirectSurfaceLiquidPhase::Authorization,
        DirectSurfaceLiquidErrorContext {
            transaction_id: Some(request_batch.transaction_id),
            owner_id: Some(configuration.owner_id.clone()),
            ..DirectSurfaceLiquidErrorContext::default()
        },
        DirectSurfaceLiquidRollbackHashes {
            beginning_owner_sha256: Some(expected_snapshot.to_string()),
            attempted_owner_sha256: Some(unified_entry_attempt_sha256(
                soil_adapter,
                request_batch,
                ingress,
                actual_snapshot,
            )),
        },
        detail,
    )
}

fn unified_entry_attempt_sha256(
    soil_adapter: &LandSurfaceEnergyRealHydrologyAdapter<'_>,
    request_batch: &PotentialWaterRequestBatch,
    ingress: &DirectSurfaceLiquidIngressInput,
    actual_snapshot: &Sha256Digest,
) -> String {
    let mut attempted = FramedSha256::new("openwepp-unified-entry-v1");
    attempted.u128("request_transaction", request_batch.transaction_id.0);
    attempted.u128("owner_transaction", soil_adapter.owner.transaction_id().0);
    attempted.u128("ingress_transaction", ingress.transaction_id.0);
    attempted.u64("ingress_day", ingress.day_index as u64);
    attempted.f64("ingress_interval", ingress.interval_s);
    attempted.string("actual_snapshot", actual_snapshot.as_str());
    attempted.finish()
}
