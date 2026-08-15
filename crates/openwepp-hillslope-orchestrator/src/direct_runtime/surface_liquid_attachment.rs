//! Canonical error completion for attaching the optional surface-liquid owner.

use openwepp_land_surface_energy::OfeId;

use super::{
    DirectSurfaceLiquidConfiguration, DirectSurfaceLiquidError, DirectSurfaceLiquidErrorCode,
    DirectSurfaceLiquidErrorContext, DirectSurfaceLiquidOwnedState, DirectSurfaceLiquidPhase,
    DirectSurfaceLiquidRollbackHashes,
};

pub(crate) fn surface_liquid_configuration_context(
    configuration: &DirectSurfaceLiquidConfiguration,
    ofe_id: Option<&OfeId>,
) -> DirectSurfaceLiquidErrorContext {
    let record = ofe_id.map_or_else(
        || configuration.records.first(),
        |ofe_id| {
            configuration
                .records
                .iter()
                .find(|record| &record.key.ofe_id == ofe_id)
        },
    );
    record.map_or_else(
        || DirectSurfaceLiquidErrorContext {
            owner_id: Some(configuration.owner_id.clone()),
            ofe_id: ofe_id.cloned(),
            ..DirectSurfaceLiquidErrorContext::default()
        },
        |record| DirectSurfaceLiquidErrorContext {
            owner_id: Some(configuration.owner_id.clone()),
            ofe_id: Some(record.key.ofe_id.clone()),
            tile_id: Some(record.key.tile_id.clone()),
            surface_id: Some(record.key.surface_id.clone()),
            source_id: Some(record.key.source_id.clone()),
            ..DirectSurfaceLiquidErrorContext::default()
        },
    )
}

pub(crate) fn surface_liquid_state_context(
    state: &DirectSurfaceLiquidOwnedState,
) -> DirectSurfaceLiquidErrorContext {
    state.records.first().map_or_else(
        || DirectSurfaceLiquidErrorContext {
            owner_id: Some(state.owner_id.clone()),
            ..DirectSurfaceLiquidErrorContext::default()
        },
        |record| DirectSurfaceLiquidErrorContext {
            transaction_id: record.last_accepted_transaction_id,
            owner_id: Some(state.owner_id.clone()),
            ofe_id: Some(record.key.ofe_id.clone()),
            tile_id: Some(record.key.tile_id.clone()),
            surface_id: Some(record.key.surface_id.clone()),
            source_id: Some(record.key.source_id.clone()),
            parcel_id: None,
        },
    )
}

pub(crate) fn surface_liquid_attachment_error(
    error: DirectSurfaceLiquidError,
    phase: DirectSurfaceLiquidPhase,
    fallback_context: DirectSurfaceLiquidErrorContext,
    beginning_owner_sha256: Option<String>,
    attempted_owner_sha256: Option<String>,
) -> DirectSurfaceLiquidError {
    let code = error.code();
    let mut completed = error.complete_context(
        code,
        phase,
        fallback_context,
        beginning_owner_sha256.clone(),
        attempted_owner_sha256.clone(),
    );
    if let DirectSurfaceLiquidError::Failure(failure) = &mut completed {
        failure.rollback.beginning_owner_sha256 = beginning_owner_sha256;
        failure.rollback.attempted_owner_sha256 = attempted_owner_sha256;
    }
    completed
}

pub(crate) fn surface_liquid_frame_identity_error(
    configuration: &DirectSurfaceLiquidConfiguration,
    ofe_id: Option<&OfeId>,
    beginning_owner_sha256: Option<String>,
    attempted_owner_sha256: Option<String>,
    detail: &'static str,
) -> DirectSurfaceLiquidError {
    DirectSurfaceLiquidError::canonical_failure(
        DirectSurfaceLiquidErrorCode::E002,
        DirectSurfaceLiquidPhase::Configuration,
        surface_liquid_configuration_context(configuration, ofe_id),
        DirectSurfaceLiquidRollbackHashes {
            beginning_owner_sha256,
            attempted_owner_sha256,
        },
        detail,
    )
}
