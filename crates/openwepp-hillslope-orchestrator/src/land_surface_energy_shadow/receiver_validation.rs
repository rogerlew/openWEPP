use super::receiver_failure::{canonical_receiver_failure, receiver_operand_failure};
use super::{
    CondensationCredit, Digest, DirectSurfaceLiquidClosureUnit, DirectSurfaceLiquidConfiguration,
    DirectSurfaceLiquidError, DirectSurfaceLiquidErrorCode, DirectSurfaceLiquidErrorContext,
    DirectSurfaceLiquidPhase, DirectSurfaceLiquidRollbackHashes, GroundWaterKey,
    LandSurfaceEnergyError, LandSurfaceEnergyErrorClass, LandSurfaceEnergyShadowError, OfeId,
    OwnerKind, OwnerRollbackHash, PotentialWaterRequestBatch, ProductionSoilLayerReceiverOperands,
    ProductionSoilReceiverOperands, RealReceiverClosureOperands, RequestingComponent,
    ResourceOwnerId, Sha256, Sha256Digest, SoilLayerId, SoilThermalTileCandidate, SourceId,
    SurfaceId, TileId, TileState, UnifiedReceiverExpectations, WaterProtocol, WaterProtocolRow,
    WaterProtocolViolation, checked_surface_liquid_add, checked_surface_liquid_close,
    checked_surface_liquid_div, checked_surface_liquid_mul, checked_surface_liquid_sub,
    checked_surface_liquid_sum,
};
use crate::DirectSurfaceLiquidConfigurationRecord;
use crate::direct_runtime::{
    SurfaceLiquidFrameDomainViolation, SurfaceLiquidFrameIdentityMismatch,
    first_invalid_surface_liquid_production_lane, first_surface_liquid_frame_identity_mismatch,
    surface_liquid_raw_snapshot_attempt_sha256, surface_liquid_raw_snapshot_sha256,
};
use crate::vegetation_real_hydrology_shadow::RealHydrologyShadowAdapter;

pub(super) fn lse_error_code(error: &LandSurfaceEnergyError) -> DirectSurfaceLiquidErrorCode {
    match error.class() {
        LandSurfaceEnergyErrorClass::Malformed => DirectSurfaceLiquidErrorCode::E001,
        LandSurfaceEnergyErrorClass::Identity => DirectSurfaceLiquidErrorCode::E002,
        LandSurfaceEnergyErrorClass::Domain => DirectSurfaceLiquidErrorCode::E003,
        LandSurfaceEnergyErrorClass::Unsupported => DirectSurfaceLiquidErrorCode::E004,
        LandSurfaceEnergyErrorClass::Cardinality => DirectSurfaceLiquidErrorCode::E005,
        LandSurfaceEnergyErrorClass::Bound => DirectSurfaceLiquidErrorCode::E006,
        LandSurfaceEnergyErrorClass::Closure => DirectSurfaceLiquidErrorCode::E010,
        LandSurfaceEnergyErrorClass::OwnerEnvelope => DirectSurfaceLiquidErrorCode::E011,
    }
}

pub(super) fn shadow_error_code(
    error: &LandSurfaceEnergyShadowError,
) -> DirectSurfaceLiquidErrorCode {
    match error {
        LandSurfaceEnergyShadowError::Identity(_) => DirectSurfaceLiquidErrorCode::E002,
        LandSurfaceEnergyShadowError::Operand(_) => DirectSurfaceLiquidErrorCode::E003,
        LandSurfaceEnergyShadowError::Bound(_) => DirectSurfaceLiquidErrorCode::E006,
        LandSurfaceEnergyShadowError::UnsupportedCustody(_) => DirectSurfaceLiquidErrorCode::E004,
        LandSurfaceEnergyShadowError::LandSurface(error) => lse_error_code(error),
        LandSurfaceEnergyShadowError::SurfaceLiquid(error) => error.code(),
    }
}

pub(super) fn validate_surface_production_binding(
    owner: &RealHydrologyShadowAdapter,
    configuration: &DirectSurfaceLiquidConfiguration,
) -> Result<(), LandSurfaceEnergyShadowError> {
    let frame = owner.beginning_frame();
    let mismatch = first_surface_liquid_frame_identity_mismatch(
        frame.identity.run_id,
        &frame.lanes,
        configuration,
        owner.layer_maps().len(),
        |topology_index, binding, lane| {
            let mapping = &owner.layer_maps()[topology_index];
            binding.production_lane_index == mapping.ofe_lane.lane_index
                && binding.production_lane_id == mapping.ofe_lane.lane_id
                && binding.production_lane_id == lane.lane_id
                && binding.ordered_soil_layer_ids == mapping.layer_ids
        },
    );
    let Some(mismatch) = mismatch else {
        return Ok(());
    };
    let (ofe_id, record, detail) = match mismatch {
        SurfaceLiquidFrameIdentityMismatch::Run
        | SurfaceLiquidFrameIdentityMismatch::LaneCardinality => {
            let mismatch_rank = configuration
                .ofe_bindings
                .len()
                .min(owner.layer_maps().len())
                .min(frame.lanes.len());
            (
                configuration
                    .ofe_bindings
                    .get(mismatch_rank)
                    .map(|binding| &binding.ofe_id),
                None,
                "surface production run or lane count",
            )
        }
        SurfaceLiquidFrameIdentityMismatch::Lane { topology_index }
        | SurfaceLiquidFrameIdentityMismatch::LayerCardinality { topology_index } => (
            configuration
                .ofe_bindings
                .get(topology_index)
                .map(|binding| &binding.ofe_id),
            None,
            "surface production OFE/lane/area/layer binding",
        ),
        SurfaceLiquidFrameIdentityMismatch::Area { record_index, .. } => {
            let record = configuration.records.get(record_index);
            (
                record.map(|record| &record.key.ofe_id),
                record,
                "surface production OFE/lane/area/layer binding",
            )
        }
    };
    Err(snapshot_failure_at(
        DirectSurfaceLiquidErrorCode::E002,
        owner,
        configuration,
        ofe_id,
        record,
        detail,
    ))
}

pub(super) fn validate_surface_production_lane_domains(
    owner: &RealHydrologyShadowAdapter,
    configuration: &DirectSurfaceLiquidConfiguration,
) -> Result<(), LandSurfaceEnergyShadowError> {
    let frame = owner.beginning_frame();
    let Some(SurfaceLiquidFrameDomainViolation::ProductionLaneAreaNotFinitePositive {
        topology_index,
    }) = first_invalid_surface_liquid_production_lane(&frame.lanes)
    else {
        return Ok(());
    };
    Err(snapshot_failure_at(
        DirectSurfaceLiquidErrorCode::E003,
        owner,
        configuration,
        configuration
            .ofe_bindings
            .get(topology_index)
            .map(|binding| &binding.ofe_id),
        None,
        "surface production lane area is not finite positive",
    ))
}

pub(super) fn snapshot_failure(
    code: DirectSurfaceLiquidErrorCode,
    owner: &RealHydrologyShadowAdapter,
    configuration: &DirectSurfaceLiquidConfiguration,
    detail: &'static str,
) -> LandSurfaceEnergyShadowError {
    snapshot_failure_at(code, owner, configuration, None, None, detail)
}

fn snapshot_failure_at(
    code: DirectSurfaceLiquidErrorCode,
    owner: &RealHydrologyShadowAdapter,
    configuration: &DirectSurfaceLiquidConfiguration,
    ofe_id: Option<&OfeId>,
    record: Option<&DirectSurfaceLiquidConfigurationRecord>,
    detail: &'static str,
) -> LandSurfaceEnergyShadowError {
    let state = owner.beginning_frame().surface_liquid_shadow.as_deref();
    let beginning = surface_liquid_raw_snapshot_sha256(owner.snapshot_bytes(), state);
    let attempted =
        surface_liquid_raw_snapshot_attempt_sha256(owner.snapshot_bytes(), configuration, state);
    DirectSurfaceLiquidError::canonical_failure(
        code,
        DirectSurfaceLiquidPhase::Restart,
        DirectSurfaceLiquidErrorContext {
            transaction_id: Some(owner.transaction_id()),
            owner_id: Some(configuration.owner_id.clone()),
            ofe_id: record
                .map(|row| row.key.ofe_id.clone())
                .or_else(|| ofe_id.cloned()),
            tile_id: record.map(|row| row.key.tile_id.clone()),
            surface_id: record.map(|row| row.key.surface_id.clone()),
            source_id: record.map(|row| row.key.source_id.clone()),
            parcel_id: None,
        },
        DirectSurfaceLiquidRollbackHashes {
            beginning_owner_sha256: Some(beginning),
            attempted_owner_sha256: Some(attempted),
        },
        detail,
    )
    .into()
}

pub(super) fn preflight_request_domains(
    batch: &PotentialWaterRequestBatch,
    beginning_sha256: &Sha256Digest,
) -> Result<(), LandSurfaceEnergyShadowError> {
    if let Some(request) = batch
        .requests
        .iter()
        .find(|request| !request.amount_kg_m2_stand_ground.is_finite())
    {
        return Err(request_failure(
            DirectSurfaceLiquidErrorCode::E003,
            batch,
            beginning_sha256,
            Some(&request.key),
            "nonfinite potential water request",
        ));
    }
    Ok(())
}

pub(super) fn preflight_request_bounds(
    batch: &PotentialWaterRequestBatch,
    beginning_sha256: &Sha256Digest,
) -> Result<(), LandSurfaceEnergyShadowError> {
    if let Some(request) = batch
        .requests
        .iter()
        .find(|request| request.amount_kg_m2_stand_ground < 0.0)
    {
        return Err(request_failure(
            DirectSurfaceLiquidErrorCode::E006,
            batch,
            beginning_sha256,
            Some(&request.key),
            "negative potential water request",
        ));
    }
    Ok(())
}

pub(super) fn request_failure(
    code: DirectSurfaceLiquidErrorCode,
    batch: &PotentialWaterRequestBatch,
    beginning_sha256: &Sha256Digest,
    key: Option<&GroundWaterKey>,
    detail: impl Into<String>,
) -> LandSurfaceEnergyShadowError {
    DirectSurfaceLiquidError::canonical_failure(
        code,
        DirectSurfaceLiquidPhase::Authorization,
        DirectSurfaceLiquidErrorContext {
            transaction_id: Some(key.map_or(batch.transaction_id, |key| key.transaction_id)),
            owner_id: key.map(|key| key.requesting_owner_id.clone()),
            ofe_id: key.map(|key| key.ofe_id.clone()),
            tile_id: key.map(|key| key.requesting_tile_id.clone()),
            surface_id: key.and_then(|key| key.surface_id.clone()),
            source_id: key.map(|key| key.source_id.clone()),
            parcel_id: None,
        },
        DirectSurfaceLiquidRollbackHashes {
            beginning_owner_sha256: Some(beginning_sha256.to_string()),
            attempted_owner_sha256: Some(water_request_batch_sha256(batch)),
        },
        detail,
    )
    .into()
}

pub(super) fn canonicalize_unified_error(
    error: LandSurfaceEnergyShadowError,
    batch: &PotentialWaterRequestBatch,
    beginning_sha256: &Sha256Digest,
) -> LandSurfaceEnergyShadowError {
    let (code, detail) = match error {
        LandSurfaceEnergyShadowError::SurfaceLiquid(error) => {
            if let Some(failure) = error.failure() {
                return DirectSurfaceLiquidError::canonical_failure(
                    failure.code,
                    failure.phase,
                    failure.context.clone(),
                    DirectSurfaceLiquidRollbackHashes {
                        beginning_owner_sha256: failure
                            .rollback
                            .beginning_owner_sha256
                            .clone()
                            .or_else(|| Some(beginning_sha256.to_string())),
                        attempted_owner_sha256: failure
                            .rollback
                            .attempted_owner_sha256
                            .clone()
                            .or_else(|| Some(water_request_batch_sha256(batch))),
                    },
                    failure.detail.clone(),
                )
                .into();
            }
            return request_failure(
                error.code(),
                batch,
                beginning_sha256,
                None,
                error.to_string(),
            );
        }
        error => (shadow_error_code(&error), error.to_string()),
    };
    request_failure(code, batch, beginning_sha256, None, detail)
}

pub(super) fn preflight_protocol_domains(
    protocol: &WaterProtocol,
    beginning_sha256: &Sha256Digest,
    attempted_sha256: &str,
) -> Result<(), LandSurfaceEnergyShadowError> {
    protocol.validate_domain_stage().map_err(|violation| {
        contextualize_protocol_violation(protocol, beginning_sha256, attempted_sha256, &violation)
    })
}

pub(super) fn preflight_protocol_bounds(
    protocol: &WaterProtocol,
    beginning_sha256: &Sha256Digest,
    attempted_sha256: &str,
) -> Result<(), LandSurfaceEnergyShadowError> {
    protocol.validate_bound_stage().map_err(|violation| {
        contextualize_protocol_violation(protocol, beginning_sha256, attempted_sha256, &violation)
    })
}

pub(super) fn preflight_request_identities(
    batch: &PotentialWaterRequestBatch,
    beginning_sha256: &Sha256Digest,
) -> Result<(), LandSurfaceEnergyShadowError> {
    for request in &batch.requests {
        if let Err(error) = request.key.validate(batch.transaction_id) {
            let (code, detail) = protocol_error_code_and_detail(&error);
            return Err(request_failure(
                code,
                batch,
                beginning_sha256,
                Some(&request.key),
                detail,
            ));
        }
    }
    Ok(())
}

pub(super) fn preflight_request_cardinality(
    batch: &PotentialWaterRequestBatch,
    beginning_sha256: &Sha256Digest,
) -> Result<(), LandSurfaceEnergyShadowError> {
    if batch.requests.is_empty() {
        return Err(request_failure(
            DirectSurfaceLiquidErrorCode::E005,
            batch,
            beginning_sha256,
            None,
            "empty potential request cardinality",
        ));
    }
    let mut keys = std::collections::BTreeSet::new();
    for request in &batch.requests {
        if !keys.insert(request.key.clone()) {
            return Err(request_failure(
                DirectSurfaceLiquidErrorCode::E005,
                batch,
                beginning_sha256,
                Some(&request.key),
                "duplicate potential water request",
            ));
        }
    }
    Ok(())
}

pub(super) fn preflight_protocol_identities(
    protocol: &WaterProtocol,
    beginning_sha256: &Sha256Digest,
    attempted_sha256: &str,
) -> Result<(), LandSurfaceEnergyShadowError> {
    protocol.validate_identity_stage().map_err(|violation| {
        contextualize_protocol_violation(protocol, beginning_sha256, attempted_sha256, &violation)
    })
}

pub(super) fn preflight_protocol_cardinality(
    protocol: &WaterProtocol,
    beginning_sha256: &Sha256Digest,
    attempted_sha256: &str,
) -> Result<(), LandSurfaceEnergyShadowError> {
    protocol.validate_cardinality_stage().map_err(|violation| {
        contextualize_protocol_violation(protocol, beginning_sha256, attempted_sha256, &violation)
    })
}

pub(super) fn protocol_error_code_and_detail(
    error: &LandSurfaceEnergyError,
) -> (DirectSurfaceLiquidErrorCode, String) {
    (lse_error_code(error), error.to_string())
}

fn contextualize_protocol_violation(
    protocol: &WaterProtocol,
    beginning_sha256: &Sha256Digest,
    attempted_sha256: &str,
    violation: &WaterProtocolViolation,
) -> LandSurfaceEnergyShadowError {
    let (code, detail) = protocol_error_code_and_detail(&violation.error);
    let key = match violation.row {
        WaterProtocolRow::Request(index) => protocol.requests.get(index).map(|row| &row.key),
        WaterProtocolRow::Authorization(index) => {
            protocol.authorizations.get(index).map(|row| &row.key)
        }
        WaterProtocolRow::FinalizedUse(index) => {
            protocol.finalized_uses.get(index).map(|row| &row.key)
        }
        WaterProtocolRow::Protocol | WaterProtocolRow::CondensationCredit(_) => None,
    };
    if let Some(key) = key {
        return protocol_failure_for_key(
            code,
            protocol,
            beginning_sha256,
            attempted_sha256,
            key,
            detail,
        );
    }
    if let WaterProtocolRow::CondensationCredit(index) = violation.row
        && let Some(credit) = protocol.condensation_credits.get(index)
    {
        return protocol_failure_for_condensation(
            code,
            beginning_sha256,
            attempted_sha256,
            credit,
            detail,
        );
    }
    protocol_failure(code, protocol, beginning_sha256, attempted_sha256, detail)
}

pub(super) fn protocol_failure(
    code: DirectSurfaceLiquidErrorCode,
    protocol: &WaterProtocol,
    beginning_sha256: &Sha256Digest,
    attempted_sha256: &str,
    detail: impl Into<String>,
) -> LandSurfaceEnergyShadowError {
    let key = protocol
        .requests
        .first()
        .map(|row| &row.key)
        .or_else(|| protocol.authorizations.first().map(|row| &row.key))
        .or_else(|| protocol.finalized_uses.first().map(|row| &row.key));
    protocol_failure_with_context(
        code,
        protocol,
        beginning_sha256,
        attempted_sha256,
        key.map(|key| key.ofe_id.clone()),
        key.map(|key| key.requesting_tile_id.clone()),
        key.and_then(|key| key.surface_id.clone()),
        key.map(|key| key.source_id.clone()),
        detail,
    )
}

pub(super) fn canonicalize_finalized_error(
    error: LandSurfaceEnergyShadowError,
    protocol: &WaterProtocol,
) -> LandSurfaceEnergyShadowError {
    let (code, detail) = match error {
        LandSurfaceEnergyShadowError::SurfaceLiquid(error) => {
            return LandSurfaceEnergyShadowError::SurfaceLiquid(error);
        }
        error => (shadow_error_code(&error), error.to_string()),
    };
    protocol_failure(
        code,
        protocol,
        &protocol.beginning_snapshot_sha256,
        &water_protocol_sha256(protocol),
        detail,
    )
}

fn protocol_failure_for_key(
    code: DirectSurfaceLiquidErrorCode,
    protocol: &WaterProtocol,
    beginning_sha256: &Sha256Digest,
    attempted_sha256: &str,
    key: &GroundWaterKey,
    detail: impl Into<String>,
) -> LandSurfaceEnergyShadowError {
    DirectSurfaceLiquidError::canonical_failure(
        code,
        DirectSurfaceLiquidPhase::ResourceCandidate,
        DirectSurfaceLiquidErrorContext {
            transaction_id: Some(key.transaction_id),
            owner_id: Some(protocol.hydrology_owner_id.clone()),
            ofe_id: Some(key.ofe_id.clone()),
            tile_id: Some(key.requesting_tile_id.clone()),
            surface_id: key.surface_id.clone(),
            source_id: Some(key.source_id.clone()),
            parcel_id: None,
        },
        DirectSurfaceLiquidRollbackHashes {
            beginning_owner_sha256: Some(beginning_sha256.to_string()),
            attempted_owner_sha256: Some(attempted_sha256.to_owned()),
        },
        detail,
    )
    .into()
}

fn protocol_failure_for_condensation(
    code: DirectSurfaceLiquidErrorCode,
    beginning_sha256: &Sha256Digest,
    attempted_sha256: &str,
    credit: &CondensationCredit,
    detail: impl Into<String>,
) -> LandSurfaceEnergyShadowError {
    DirectSurfaceLiquidError::canonical_failure(
        code,
        DirectSurfaceLiquidPhase::ResourceCandidate,
        DirectSurfaceLiquidErrorContext {
            transaction_id: Some(credit.transaction_id),
            owner_id: Some(credit.hydrology_owner_id.clone()),
            ofe_id: Some(credit.ofe_id.clone()),
            tile_id: Some(credit.tile_id.clone()),
            surface_id: Some(credit.surface_id.clone()),
            source_id: None,
            parcel_id: None,
        },
        DirectSurfaceLiquidRollbackHashes {
            beginning_owner_sha256: Some(beginning_sha256.to_string()),
            attempted_owner_sha256: Some(attempted_sha256.to_owned()),
        },
        detail,
    )
    .into()
}

#[allow(clippy::too_many_arguments)]
fn protocol_failure_with_context(
    code: DirectSurfaceLiquidErrorCode,
    protocol: &WaterProtocol,
    beginning_sha256: &Sha256Digest,
    attempted_sha256: &str,
    ofe_id: Option<OfeId>,
    tile_id: Option<TileId>,
    surface_id: Option<SurfaceId>,
    source_id: Option<SourceId>,
    detail: impl Into<String>,
) -> LandSurfaceEnergyShadowError {
    DirectSurfaceLiquidError::canonical_failure(
        code,
        DirectSurfaceLiquidPhase::ResourceCandidate,
        DirectSurfaceLiquidErrorContext {
            transaction_id: Some(protocol.transaction_id),
            owner_id: Some(protocol.hydrology_owner_id.clone()),
            ofe_id,
            tile_id,
            surface_id,
            source_id,
            parcel_id: None,
        },
        DirectSurfaceLiquidRollbackHashes {
            beginning_owner_sha256: Some(beginning_sha256.to_string()),
            attempted_owner_sha256: Some(attempted_sha256.to_owned()),
        },
        detail,
    )
    .into()
}

pub(super) struct FramedSha256(Sha256);

impl FramedSha256 {
    pub(super) fn new(domain: &'static str) -> Self {
        let mut framed = Self(Sha256::new());
        framed.bytes("domain", domain.as_bytes());
        framed
    }

    pub(super) fn bytes(&mut self, tag: &'static str, value: &[u8]) {
        self.0.update((tag.len() as u64).to_be_bytes());
        self.0.update(tag.as_bytes());
        self.0.update((value.len() as u64).to_be_bytes());
        self.0.update(value);
    }

    pub(super) fn string(&mut self, tag: &'static str, value: &str) {
        self.bytes(tag, value.as_bytes());
    }

    pub(super) fn count(&mut self, tag: &'static str, value: usize) {
        self.bytes(tag, &(value as u64).to_be_bytes());
    }

    pub(super) fn u64(&mut self, tag: &'static str, value: u64) {
        self.bytes(tag, &value.to_be_bytes());
    }

    pub(super) fn u128(&mut self, tag: &'static str, value: u128) {
        self.bytes(tag, &value.to_be_bytes());
    }

    pub(super) fn f64(&mut self, tag: &'static str, value: f64) {
        self.u64(tag, value.to_bits());
    }

    pub(super) fn finish(self) -> String {
        format!("{:x}", self.0.finalize())
    }
}

pub(super) fn receiver_expectation_fields_sha256(
    lse_owner_id: &ResourceOwnerId,
    beginning_lse: &super::Sha256Digest,
    hydrology_owner_id: &ResourceOwnerId,
    beginning_hydrology: &super::Sha256Digest,
    thermal_owner_id: &ResourceOwnerId,
    beginning_thermal: &super::Sha256Digest,
    rows: &[(OfeId, TileId, Vec<SoilLayerId>)],
) -> String {
    let mut out = FramedSha256::new("openwepp-unified-receiver-expectations-v3");
    out.string("lse_owner", lse_owner_id.as_str());
    out.string("lse_beginning", beginning_lse.as_str());
    out.string("hydrology_owner", hydrology_owner_id.as_str());
    out.string("hydrology_beginning", beginning_hydrology.as_str());
    out.string("thermal_owner", thermal_owner_id.as_str());
    out.string("thermal_beginning", beginning_thermal.as_str());
    out.count("tile_count", rows.len());
    for (ofe, tile, layers) in rows {
        out.string("ofe", ofe.as_str());
        out.string("tile", tile.as_str());
        out.count("layer_count", layers.len());
        for layer in layers {
            out.string("layer", layer.as_str());
        }
    }
    out.finish()
}

pub(super) fn receiver_expectations_sha256(value: &UnifiedReceiverExpectations) -> String {
    let rows = value
        .ordered_thermal_layers
        .iter()
        .map(|((ofe, tile), layers)| (ofe.clone(), tile.clone(), layers.clone()))
        .collect::<Vec<_>>();
    receiver_expectation_fields_sha256(
        &value.lse_owner_id,
        &value.beginning_lse_state_sha256,
        &value.hydrology_owner_id,
        &value.beginning_hydrology_snapshot_sha256,
        &value.soil_thermal_owner_id,
        &value.beginning_soil_thermal_state_sha256,
        &rows,
    )
}

pub(super) fn finalization_receiver_sets_sha256(
    lse_tiles: &[TileState],
    thermal_tiles: &[SoilThermalTileCandidate],
    rollback: &[OwnerRollbackHash],
) -> String {
    let mut out = FramedSha256::new("openwepp-sealed-lse-finalization-receivers-v2");
    out.count("lse_tile_count", lse_tiles.len());
    for tile in lse_tiles {
        out.string("lse_ofe", tile.ofe_id.as_str());
        out.string("lse_tile", tile.tile_id.as_str());
        out.f64("surface_enthalpy", tile.surface_enthalpy_j_m2_tile_ground);
        out.f64("warm_start", tile.surface_temperature_warm_start_k);
    }
    out.count("thermal_tile_count", thermal_tiles.len());
    for tile in thermal_tiles {
        out.string("thermal_owner", tile.owner_id.as_str());
        out.string("thermal_beginning", tile.beginning_state_sha256.as_str());
        out.string("thermal_ofe", tile.ofe_id.as_str());
        out.string("thermal_tile", tile.tile_id.as_str());
        out.count("thermal_layer_count", tile.layers.len());
        for layer in &tile.layers {
            out.string("thermal_layer", layer.layer_id.as_str());
            out.f64(
                "thermal_beginning_enthalpy",
                layer.beginning_enthalpy_j_m2_ofe_ground,
            );
            out.f64(
                "thermal_ground_heat_credit",
                layer.ground_heat_credit_j_m2_ofe_ground,
            );
            out.f64(
                "thermal_infiltration_enthalpy_credit",
                layer.infiltration_enthalpy_credit_j_m2_ofe_ground,
            );
            out.f64(
                "thermal_ending_enthalpy",
                layer.ending_enthalpy_j_m2_ofe_ground,
            );
            out.f64("thermal_ending_temperature", layer.ending_temperature_k);
        }
    }
    out.count("rollback_count", rollback.len());
    for row in rollback {
        out.u64("rollback_kind", row.owner_kind as u64);
        out.string("rollback_owner", &row.owner_id);
        out.string("rollback_before", row.before_sha256.as_str());
        out.string("rollback_after", row.after_sha256.as_str());
    }
    out.finish()
}

pub(super) fn water_protocol_sha256(protocol: &WaterProtocol) -> String {
    let mut out = FramedSha256::new("openwepp-water-protocol-attempt-v1");
    out.u128("transaction", protocol.transaction_id.0);
    out.string("owner", protocol.hydrology_owner_id.as_str());
    out.string("beginning", protocol.beginning_snapshot_sha256.as_str());
    out.count("request_count", protocol.requests.len());
    for row in &protocol.requests {
        frame_water_key(&mut out, &row.key);
        out.f64("request_amount", row.amount_kg_m2_stand_ground);
    }
    out.count("authorization_count", protocol.authorizations.len());
    for row in &protocol.authorizations {
        frame_water_key(&mut out, &row.key);
        out.f64("authorization_amount", row.amount_kg_m2_stand_ground);
        out.string("authorization_reason", &format!("{:?}", row.reason));
    }
    out.count("use_count", protocol.finalized_uses.len());
    for row in &protocol.finalized_uses {
        frame_water_key(&mut out, &row.key);
        out.f64("use_amount", row.amount_kg_m2_stand_ground);
    }
    out.count("credit_count", protocol.condensation_credits.len());
    for row in &protocol.condensation_credits {
        out.u128("credit_transaction", row.transaction_id.0);
        out.string("credit_owner", row.hydrology_owner_id.as_str());
        out.string("credit_ofe", row.ofe_id.as_str());
        out.string("credit_tile", row.tile_id.as_str());
        out.string("credit_surface", row.surface_id.as_str());
        out.f64("credit_amount", row.amount_kg_m2_stand_ground);
        out.string("credit_basis", &format!("{:?}", row.amount_basis));
        out.f64("credit_temperature", row.temperature_k);
        out.f64("credit_enthalpy", row.specific_liquid_enthalpy_j_kg);
    }
    out.finish()
}

pub(super) fn water_request_batch_sha256(batch: &PotentialWaterRequestBatch) -> String {
    let mut out = FramedSha256::new("openwepp-water-request-batch-attempt-v1");
    out.u128("transaction", batch.transaction_id.0);
    out.string("beginning_lse", batch.beginning_lse_state_sha256.as_str());
    out.string(
        "potential_signature",
        batch.potential_signature_sha256.as_str(),
    );
    out.count("request_count", batch.requests.len());
    for row in &batch.requests {
        frame_water_key(&mut out, &row.key);
        out.f64("request_amount", row.amount_kg_m2_stand_ground);
    }
    out.finish()
}

fn frame_water_key(out: &mut FramedSha256, key: &GroundWaterKey) {
    out.u128("key_transaction", key.transaction_id.0);
    out.string("key_owner", key.requesting_owner_id.as_str());
    out.string("key_component", &format!("{:?}", key.requesting_component));
    out.string("key_ofe", key.ofe_id.as_str());
    out.string("key_requesting_tile", key.requesting_tile_id.as_str());
    frame_optional(
        out,
        "key_occupancy",
        key.occupancy_id.as_ref().map(super::ComponentId::as_str),
    );
    frame_optional(
        out,
        "key_surface",
        key.surface_id.as_ref().map(super::SurfaceId::as_str),
    );
    out.string("key_surface_class", &format!("{:?}", key.surface_class));
    out.string("key_source_type", &format!("{:?}", key.source_type));
    out.string("key_source", key.source_id.as_str());
    frame_optional(
        out,
        "key_source_tile",
        key.source_tile_id.as_ref().map(TileId::as_str),
    );
    frame_optional(
        out,
        "key_soil_layer",
        key.soil_layer_id.as_ref().map(SoilLayerId::as_str),
    );
    out.string("key_basis", &format!("{:?}", key.amount_basis));
}

fn frame_optional(out: &mut FramedSha256, tag: &'static str, value: Option<&str>) {
    match value {
        Some(value) => {
            out.u64("optional_presence", 1);
            out.string(tag, value);
        }
        None => out.u64("optional_presence", 0),
    }
}

/// Independently reconstruct all real receiver ending equations from frozen operands.
pub fn validate_real_receiver_closure(
    operands: &RealReceiverClosureOperands,
) -> Result<(), DirectSurfaceLiquidError> {
    validate_receiver_envelope(operands)?;
    validate_production_soil_receiver_closure(operands)?;
    for thermal in &operands.soil_thermal {
        let expected_credit = checked_surface_liquid_add(
            thermal.beginning_infiltration_credit_j_m2_ofe_ground,
            thermal.infiltration_enthalpy_j_m2_ofe_ground,
        );
        let expected_ending = checked_surface_liquid_add(
            thermal.beginning_enthalpy_j_m2_ofe_ground,
            thermal.infiltration_enthalpy_j_m2_ofe_ground,
        );
        let (expected_credit, expected_ending) =
            expected_credit.zip(expected_ending).ok_or_else(|| {
                receiver_arithmetic_failure(
                    operands,
                    OwnerKind::SoilThermal,
                    &operands.soil_thermal_owner_id,
                    Some(&thermal.ofe_id),
                    Some(&thermal.tile_id),
                    "soil-thermal infiltration enthalpy arithmetic",
                )
            })?;
        require_receiver_close(
            operands,
            OwnerKind::SoilThermal,
            &operands.soil_thermal_owner_id,
            thermal.ending_infiltration_credit_j_m2_ofe_ground,
            expected_credit,
            DirectSurfaceLiquidClosureUnit::EnthalpyJM2,
            Some(&thermal.ofe_id),
            Some(&thermal.tile_id),
            "soil-thermal infiltration-credit ending equation",
        )?;
        require_receiver_close(
            operands,
            OwnerKind::SoilThermal,
            &operands.soil_thermal_owner_id,
            thermal.ending_enthalpy_j_m2_ofe_ground,
            expected_ending,
            DirectSurfaceLiquidClosureUnit::EnthalpyJM2,
            Some(&thermal.ofe_id),
            Some(&thermal.tile_id),
            "soil-thermal infiltration enthalpy ending equation",
        )?;
    }
    for tile in &operands.lse_tiles {
        if !tile.tile_fraction.is_finite() || tile.tile_fraction <= 0.0 {
            return Err(receiver_atomic_failure(
                operands,
                OwnerKind::LandSurfaceEnergy,
                &operands.lse_owner_id,
                Some(&tile.ofe_id),
                Some(&tile.tile_id),
                "LSE retained tile fraction",
            ));
        }
        let expected =
            checked_surface_liquid_div(tile.retained_enthalpy_j_m2_ofe_ground, tile.tile_fraction)
                .and_then(|retained| {
                    checked_surface_liquid_add(tile.beginning_enthalpy_j_m2_tile_ground, retained)
                })
                .ok_or_else(|| {
                    receiver_arithmetic_failure(
                        operands,
                        OwnerKind::LandSurfaceEnergy,
                        &operands.lse_owner_id,
                        Some(&tile.ofe_id),
                        Some(&tile.tile_id),
                        "LSE retained enthalpy arithmetic",
                    )
                })?;
        require_receiver_close(
            operands,
            OwnerKind::LandSurfaceEnergy,
            &operands.lse_owner_id,
            tile.ending_enthalpy_j_m2_tile_ground,
            expected,
            DirectSurfaceLiquidClosureUnit::EnthalpyJM2,
            Some(&tile.ofe_id),
            Some(&tile.tile_id),
            "LSE retained enthalpy ending equation",
        )?;
    }
    Ok(())
}

#[allow(clippy::too_many_lines)]
fn validate_production_soil_receiver_closure(
    operands: &RealReceiverClosureOperands,
) -> Result<(), DirectSurfaceLiquidError> {
    for lane in &operands.production_soil {
        if lane.ordered_layers.is_empty()
            || !lane.infiltration_m.is_finite()
            || lane.infiltration_m < 0.0
            || !lane.tillage_depth_m.is_finite()
        {
            return Err(receiver_atomic_failure(
                operands,
                OwnerKind::Hydrology,
                &operands.hydrology_owner_id,
                Some(&lane.ofe_id),
                None,
                "production soil receiver operand domain",
            ));
        }
        let expected = independently_reconstruct_infiltration(lane).ok_or_else(|| {
            receiver_atomic_failure(
                operands,
                OwnerKind::Hydrology,
                &operands.hydrology_owner_id,
                Some(&lane.ofe_id),
                None,
                "production soil receiver reconstruction domain",
            )
        })?;
        for (layer, expected_ending) in lane.ordered_layers.iter().zip(expected) {
            require_receiver_close(
                operands,
                OwnerKind::Hydrology,
                &operands.hydrology_owner_id,
                layer.ending_liquid_m,
                expected_ending,
                DirectSurfaceLiquidClosureUnit::MassM,
                Some(&lane.ofe_id),
                None,
                "ordered production soil-layer infiltration equation",
            )?;
        }
        let beginning_terms = lane
            .ordered_layers
            .iter()
            .map(|layer| checked_receiver_layer_total(layer.beginning_liquid_m, layer))
            .collect::<Option<Vec<_>>>();
        let ending_terms = lane
            .ordered_layers
            .iter()
            .map(|layer| checked_receiver_layer_total(layer.ending_liquid_m, layer))
            .collect::<Option<Vec<_>>>();
        let beginning_sum = beginning_terms
            .and_then(checked_surface_liquid_sum)
            .ok_or_else(|| {
                receiver_arithmetic_failure(
                    operands,
                    OwnerKind::Hydrology,
                    &operands.hydrology_owner_id,
                    Some(&lane.ofe_id),
                    None,
                    "beginning aggregate soil-water arithmetic",
                )
            })?;
        let ending_sum = ending_terms
            .and_then(checked_surface_liquid_sum)
            .ok_or_else(|| {
                receiver_arithmetic_failure(
                    operands,
                    OwnerKind::Hydrology,
                    &operands.hydrology_owner_id,
                    Some(&lane.ofe_id),
                    None,
                    "ending aggregate soil-water arithmetic",
                )
            })?;
        let expected_aggregate_ending =
            checked_surface_liquid_add(lane.beginning_aggregate_soil_water_m, lane.infiltration_m)
                .ok_or_else(|| {
                    receiver_arithmetic_failure(
                        operands,
                        OwnerKind::Hydrology,
                        &operands.hydrology_owner_id,
                        Some(&lane.ofe_id),
                        None,
                        "aggregate soil-water ending arithmetic",
                    )
                })?;
        require_receiver_close(
            operands,
            OwnerKind::Hydrology,
            &operands.hydrology_owner_id,
            lane.beginning_aggregate_soil_water_m,
            beginning_sum,
            DirectSurfaceLiquidClosureUnit::MassM,
            Some(&lane.ofe_id),
            None,
            "beginning aggregate production soil-water equation",
        )?;
        require_receiver_close(
            operands,
            OwnerKind::Hydrology,
            &operands.hydrology_owner_id,
            lane.ending_aggregate_soil_water_m,
            ending_sum,
            DirectSurfaceLiquidClosureUnit::MassM,
            Some(&lane.ofe_id),
            None,
            "ending aggregate production soil-water equation",
        )?;
        require_receiver_close(
            operands,
            OwnerKind::Hydrology,
            &operands.hydrology_owner_id,
            lane.ending_aggregate_soil_water_m,
            expected_aggregate_ending,
            DirectSurfaceLiquidClosureUnit::MassM,
            Some(&lane.ofe_id),
            None,
            "aggregate production soil-water ending equation",
        )?;
    }
    Ok(())
}

fn checked_receiver_layer_total(
    liquid_m: f64,
    layer: &ProductionSoilLayerReceiverOperands,
) -> Option<f64> {
    let unfrozen_depth = checked_surface_liquid_sub(layer.layer_depth_m, layer.frozen_depth_m)?;
    let residual = checked_surface_liquid_mul(layer.residual_theta, unfrozen_depth.max(0.0))?;
    checked_surface_liquid_add(liquid_m, residual)
}

fn independently_reconstruct_infiltration(
    lane: &ProductionSoilReceiverOperands,
) -> Option<Vec<f64>> {
    let first_depth = lane.ordered_layers.first()?.layer_depth_m;
    let resolved_tillage_depth_m = if lane.tillage_depth_m > 1.0e-12 {
        lane.tillage_depth_m
    } else {
        first_depth
    };
    if !resolved_tillage_depth_m.is_finite() || resolved_tillage_depth_m <= 0.0 {
        return None;
    }
    let mut remaining = lane.infiltration_m;
    let mut cumulative_depth_m = 0.0;
    let mut expected = lane
        .ordered_layers
        .iter()
        .map(|layer| layer.beginning_liquid_m)
        .collect::<Vec<_>>();
    for (layer, ending) in lane.ordered_layers.iter().zip(&mut expected) {
        if remaining <= 0.0 {
            break;
        }
        if !layer.layer_depth_m.is_finite()
            || layer.layer_depth_m <= 0.0
            || !layer.residual_theta.is_finite()
            || layer.residual_theta < 0.0
            || !layer.frozen_depth_m.is_finite()
            || layer.frozen_depth_m < 0.0
        {
            return None;
        }
        cumulative_depth_m = checked_surface_liquid_add(cumulative_depth_m, layer.layer_depth_m)?;
        let addition = if cumulative_depth_m < resolved_tillage_depth_m - 1.0e-12 {
            checked_surface_liquid_mul(remaining, layer.layer_depth_m)
                .and_then(|value| checked_surface_liquid_div(value, resolved_tillage_depth_m))?
        } else {
            remaining
        };
        *ending = checked_surface_liquid_add(*ending, addition.max(0.0))?;
        remaining = checked_surface_liquid_sub(remaining, addition)?;
    }
    if remaining > 0.0 {
        let last = expected.last_mut()?;
        *last = checked_surface_liquid_add(*last, remaining)?;
    }
    Some(expected)
}

#[allow(clippy::too_many_arguments)]
pub(super) fn require_receiver_close(
    operands: &RealReceiverClosureOperands,
    owner_kind: OwnerKind,
    owner_id: &ResourceOwnerId,
    actual: f64,
    expected: f64,
    unit: DirectSurfaceLiquidClosureUnit,
    ofe_id: Option<&OfeId>,
    tile_id: Option<&TileId>,
    detail: &'static str,
) -> Result<(), DirectSurfaceLiquidError> {
    match checked_surface_liquid_close(actual, expected, unit) {
        Some(true) => Ok(()),
        Some(false) => Err(receiver_equation_failure(
            operands, owner_kind, owner_id, ofe_id, tile_id, detail,
        )),
        None => Err(receiver_arithmetic_failure(
            operands, owner_kind, owner_id, ofe_id, tile_id, detail,
        )),
    }
}

fn receiver_equation_failure(
    operands: &RealReceiverClosureOperands,
    owner_kind: OwnerKind,
    owner_id: &ResourceOwnerId,
    ofe_id: Option<&OfeId>,
    tile_id: Option<&TileId>,
    detail: &'static str,
) -> DirectSurfaceLiquidError {
    receiver_operand_failure(
        operands,
        DirectSurfaceLiquidErrorCode::E010,
        DirectSurfaceLiquidPhase::IndependentClosure,
        owner_kind,
        owner_id,
        ofe_id,
        tile_id,
        detail,
    )
}

pub(super) fn receiver_atomic_failure(
    operands: &RealReceiverClosureOperands,
    owner_kind: OwnerKind,
    owner_id: &ResourceOwnerId,
    ofe_id: Option<&OfeId>,
    tile_id: Option<&TileId>,
    detail: &'static str,
) -> DirectSurfaceLiquidError {
    receiver_operand_failure(
        operands,
        DirectSurfaceLiquidErrorCode::E011,
        DirectSurfaceLiquidPhase::AtomicEnvelope,
        owner_kind,
        owner_id,
        ofe_id,
        tile_id,
        detail,
    )
}

fn receiver_arithmetic_failure(
    operands: &RealReceiverClosureOperands,
    owner_kind: OwnerKind,
    owner_id: &ResourceOwnerId,
    ofe_id: Option<&OfeId>,
    tile_id: Option<&TileId>,
    detail: &'static str,
) -> DirectSurfaceLiquidError {
    receiver_operand_failure(
        operands,
        DirectSurfaceLiquidErrorCode::E003,
        DirectSurfaceLiquidPhase::IndependentClosure,
        owner_kind,
        owner_id,
        ofe_id,
        tile_id,
        detail,
    )
}

type ProductionIdentity = (OfeId, usize, u32, Vec<SoilLayerId>);
type ThermalIdentity = (OfeId, TileId, SoilLayerId);
type TileIdentity = (OfeId, TileId);

#[allow(clippy::type_complexity)]
pub(super) fn expected_receiver_identities(
    configuration: &DirectSurfaceLiquidConfiguration,
) -> (
    Vec<ProductionIdentity>,
    Vec<ThermalIdentity>,
    Vec<TileIdentity>,
) {
    let production = configuration
        .ofe_bindings
        .iter()
        .map(|binding| {
            (
                binding.ofe_id.clone(),
                binding.production_lane_index,
                binding.production_lane_id,
                binding.ordered_soil_layer_ids.clone(),
            )
        })
        .collect();
    let thermal = configuration
        .records
        .iter()
        .filter_map(|record| {
            configuration
                .ofe_bindings
                .iter()
                .find(|binding| binding.ofe_id == record.key.ofe_id)
                .map(|binding| {
                    (
                        record.key.ofe_id.clone(),
                        record.key.tile_id.clone(),
                        binding.infiltration_soil_thermal_layer_id.clone(),
                    )
                })
        })
        .collect();
    let lse = configuration
        .records
        .iter()
        .map(|record| (record.key.ofe_id.clone(), record.key.tile_id.clone()))
        .collect();
    (production, thermal, lse)
}

fn validate_configured_receiver_context(
    operands: &RealReceiverClosureOperands,
) -> Result<(), DirectSurfaceLiquidError> {
    let configured_tiles = operands
        .configured_surface_context
        .iter()
        .map(|(ofe, tile, _, _)| (ofe.clone(), tile.clone()))
        .collect::<Vec<_>>();
    if configured_tiles != operands.expected_lse_tiles {
        let index = first_mismatch(&operands.expected_lse_tiles, &configured_tiles);
        let identity = operands
            .expected_lse_tiles
            .get(index)
            .or_else(|| configured_tiles.get(index));
        return Err(receiver_envelope_failure(
            operands,
            OwnerKind::Hydrology,
            &operands.hydrology_owner_id,
            identity.map(|row| &row.0),
            identity.map(|row| &row.1),
            "configured surface receiver context mismatch",
        ));
    }
    Ok(())
}

fn actual_production_identities(operands: &RealReceiverClosureOperands) -> Vec<ProductionIdentity> {
    operands
        .production_soil
        .iter()
        .map(|lane| {
            (
                lane.ofe_id.clone(),
                lane.production_lane_index,
                lane.production_lane_id,
                lane.ordered_layers
                    .iter()
                    .map(|layer| layer.layer_id.clone())
                    .collect(),
            )
        })
        .collect()
}

pub(super) fn validate_receiver_envelope(
    operands: &RealReceiverClosureOperands,
) -> Result<(), DirectSurfaceLiquidError> {
    validate_numeric_domains(operands)?;
    validate_frozen_rollback_provenance(operands)?;
    validate_configured_receiver_context(operands)?;
    let production = actual_production_identities(operands);
    if production != operands.expected_production_soil {
        let index = first_mismatch(&operands.expected_production_soil, &production);
        let ofe = production
            .get(index)
            .or_else(|| operands.expected_production_soil.get(index))
            .map(|identity| &identity.0);
        if production.len() != operands.expected_production_soil.len() {
            return Err(receiver_envelope_failure(
                operands,
                OwnerKind::Hydrology,
                &operands.hydrology_owner_id,
                ofe,
                None,
                "production soil receiver cardinality mismatch",
            ));
        }
        return Err(join_failure(
            operands,
            OwnerKind::Hydrology,
            &operands.hydrology_owner_id,
            ofe,
            None,
            "production soil receiver topology mismatch",
        ));
    }
    let thermal = operands
        .soil_thermal
        .iter()
        .map(|row| {
            (
                row.ofe_id.clone(),
                row.tile_id.clone(),
                row.layer_id.clone(),
            )
        })
        .collect::<Vec<_>>();
    if thermal != operands.expected_soil_thermal {
        let index = first_mismatch(&operands.expected_soil_thermal, &thermal);
        let identity = thermal
            .get(index)
            .or_else(|| operands.expected_soil_thermal.get(index));
        if thermal.len() != operands.expected_soil_thermal.len() {
            return Err(receiver_envelope_failure(
                operands,
                OwnerKind::SoilThermal,
                &operands.soil_thermal_owner_id,
                identity.map(|row| &row.0),
                identity.map(|row| &row.1),
                "soil-thermal receiver cardinality mismatch",
            ));
        }
        return Err(join_failure(
            operands,
            OwnerKind::SoilThermal,
            &operands.soil_thermal_owner_id,
            identity.map(|row| &row.0),
            identity.map(|row| &row.1),
            "soil-thermal receiver topology mismatch",
        ));
    }
    let lse = operands
        .lse_tiles
        .iter()
        .map(|row| (row.ofe_id.clone(), row.tile_id.clone()))
        .collect::<Vec<_>>();
    if lse != operands.expected_lse_tiles {
        let index = first_mismatch(&operands.expected_lse_tiles, &lse);
        let identity = lse
            .get(index)
            .or_else(|| operands.expected_lse_tiles.get(index));
        if lse.len() != operands.expected_lse_tiles.len() {
            return Err(receiver_envelope_failure(
                operands,
                OwnerKind::LandSurfaceEnergy,
                &operands.lse_owner_id,
                identity.map(|row| &row.0),
                identity.map(|row| &row.1),
                "LSE tile receiver cardinality mismatch",
            ));
        }
        return Err(join_failure(
            operands,
            OwnerKind::LandSurfaceEnergy,
            &operands.lse_owner_id,
            identity.map(|row| &row.0),
            identity.map(|row| &row.1),
            "LSE tile receiver topology mismatch",
        ));
    }
    Ok(())
}

fn validate_frozen_rollback_provenance(
    operands: &RealReceiverClosureOperands,
) -> Result<(), DirectSurfaceLiquidError> {
    let expected = [
        (
            OwnerKind::LandSurfaceEnergy,
            &operands.lse_owner_id,
            &operands.beginning_lse_state_sha256,
        ),
        (
            OwnerKind::Hydrology,
            &operands.hydrology_owner_id,
            &operands.beginning_hydrology_snapshot_sha256,
        ),
        (
            OwnerKind::SoilThermal,
            &operands.soil_thermal_owner_id,
            &operands.beginning_soil_thermal_state_sha256,
        ),
    ];
    for (owner_kind, owner_id, _) in expected {
        let count = operands
            .rollback_hashes
            .iter()
            .filter(|row| row.owner_kind == owner_kind && row.owner_id == owner_id.as_str())
            .count();
        if count != 1 {
            return Err(receiver_operand_failure(
                operands,
                DirectSurfaceLiquidErrorCode::E011,
                DirectSurfaceLiquidPhase::AtomicEnvelope,
                owner_kind,
                owner_id,
                None,
                None,
                if count == 0 {
                    "missing frozen receiver rollback row"
                } else {
                    "duplicate frozen receiver rollback row"
                },
            ));
        }
    }
    for (index, (owner_kind, owner_id, beginning)) in expected.into_iter().enumerate() {
        let Some(actual) = operands.rollback_hashes.get(index) else {
            return Err(receiver_operand_failure(
                operands,
                DirectSurfaceLiquidErrorCode::E011,
                DirectSurfaceLiquidPhase::AtomicEnvelope,
                owner_kind,
                owner_id,
                None,
                None,
                "missing ordered frozen receiver rollback row",
            ));
        };
        if actual.owner_kind != owner_kind
            || actual.owner_id != owner_id.as_str()
            || &actual.before_sha256 != beginning
            || &actual.after_sha256 != beginning
        {
            return Err(receiver_operand_failure(
                operands,
                DirectSurfaceLiquidErrorCode::E011,
                DirectSurfaceLiquidPhase::AtomicEnvelope,
                owner_kind,
                owner_id,
                None,
                None,
                "frozen receiver rollback row mismatch",
            ));
        }
    }
    if operands.rollback_hashes.len() != expected.len() {
        let actual = &operands.rollback_hashes[expected.len()];
        let owner_id = ResourceOwnerId::try_new(actual.owner_id.clone()).ok();
        return Err(canonical_receiver_failure(
            DirectSurfaceLiquidErrorCode::E011,
            DirectSurfaceLiquidPhase::AtomicEnvelope,
            operands.transaction_id,
            Some(actual.owner_kind),
            owner_id.as_ref(),
            None,
            None,
            None,
            None,
            None,
            &operands.rollback_hashes,
            &receiver_operands_sha256(operands),
            "unexpected frozen receiver rollback row",
        ));
    }
    Ok(())
}

pub(super) fn preflight_finalization_receiver_numerics(
    transaction_id: super::TransactionId,
    configuration: &DirectSurfaceLiquidConfiguration,
    expectations: &UnifiedReceiverExpectations,
    lse_tiles: &[TileState],
    thermal_tiles: &[SoilThermalTileCandidate],
    rollback_hashes: &[OwnerRollbackHash],
    attempted_sha256: &str,
) -> Result<(), super::LandSurfaceEnergyShadowError> {
    if let Some(tile) = lse_tiles.iter().find(|tile| {
        !tile.surface_enthalpy_j_m2_tile_ground.is_finite()
            || !tile.surface_temperature_warm_start_k.is_finite()
    }) {
        return Err(finalization_numeric_failure(
            transaction_id,
            configuration,
            &expectations.lse_owner_id,
            OwnerKind::LandSurfaceEnergy,
            &tile.ofe_id,
            &tile.tile_id,
            rollback_hashes,
            attempted_sha256,
            "nonfinite LSE tile receiver",
        ));
    }
    if let Some(tile) = thermal_tiles.iter().find(|tile| {
        tile.layers.iter().any(|layer| {
            !layer.beginning_enthalpy_j_m2_ofe_ground.is_finite()
                || !layer.ground_heat_credit_j_m2_ofe_ground.is_finite()
                || !layer
                    .infiltration_enthalpy_credit_j_m2_ofe_ground
                    .is_finite()
                || !layer.ending_enthalpy_j_m2_ofe_ground.is_finite()
                || !layer.ending_temperature_k.is_finite()
        })
    }) {
        return Err(finalization_numeric_failure(
            transaction_id,
            configuration,
            &tile.owner_id,
            OwnerKind::SoilThermal,
            &tile.ofe_id,
            &tile.tile_id,
            rollback_hashes,
            attempted_sha256,
            "nonfinite soil-thermal tile receiver",
        ));
    }
    Ok(())
}

pub(super) fn preflight_sealed_finalization_numerics(
    protocol: &WaterProtocol,
    lse_tiles: &[TileState],
    thermal_tiles: &[SoilThermalTileCandidate],
    rollback_hashes: &[OwnerRollbackHash],
    attempted_sha256: &str,
) -> Result<(), super::LandSurfaceEnergyShadowError> {
    let lse_failure = lse_tiles.iter().find(|tile| {
        !tile.surface_enthalpy_j_m2_tile_ground.is_finite()
            || !tile.surface_temperature_warm_start_k.is_finite()
    });
    let thermal_failure = thermal_tiles.iter().find(|tile| {
        tile.layers.iter().any(|layer| {
            !layer.beginning_enthalpy_j_m2_ofe_ground.is_finite()
                || !layer.ground_heat_credit_j_m2_ofe_ground.is_finite()
                || !layer
                    .infiltration_enthalpy_credit_j_m2_ofe_ground
                    .is_finite()
                || !layer.ending_enthalpy_j_m2_ofe_ground.is_finite()
                || !layer.ending_temperature_k.is_finite()
        })
    });
    let (context, owner_kind, detail) = if let Some(tile) = lse_failure {
        let ground_request = protocol
            .requests
            .iter()
            .find(|request| {
                request.key.requesting_component == RequestingComponent::GroundSurface
                    && request.key.ofe_id == tile.ofe_id
                    && request.key.requesting_tile_id == tile.tile_id
            })
            .map(|request| &request.key);
        let lse_owner_id = ground_request
            .or_else(|| {
                protocol
                    .requests
                    .iter()
                    .find(|request| {
                        request.key.requesting_component == RequestingComponent::GroundSurface
                    })
                    .map(|request| &request.key)
            })
            .map(|key| key.requesting_owner_id.clone());
        (
            DirectSurfaceLiquidErrorContext {
                transaction_id: Some(protocol.transaction_id),
                owner_id: lse_owner_id,
                ofe_id: Some(tile.ofe_id.clone()),
                tile_id: Some(tile.tile_id.clone()),
                surface_id: ground_request.and_then(|key| key.surface_id.clone()),
                source_id: ground_request.map(|key| key.source_id.clone()),
                parcel_id: None,
            },
            OwnerKind::LandSurfaceEnergy,
            "nonfinite sealed LSE tile receiver",
        )
    } else if let Some(tile) = thermal_failure {
        (
            DirectSurfaceLiquidErrorContext {
                transaction_id: Some(protocol.transaction_id),
                owner_id: Some(tile.owner_id.clone()),
                ofe_id: Some(tile.ofe_id.clone()),
                tile_id: Some(tile.tile_id.clone()),
                ..DirectSurfaceLiquidErrorContext::default()
            },
            OwnerKind::SoilThermal,
            "nonfinite sealed soil-thermal tile receiver",
        )
    } else {
        return Ok(());
    };
    Err(canonical_receiver_failure(
        DirectSurfaceLiquidErrorCode::E003,
        DirectSurfaceLiquidPhase::IndependentClosure,
        protocol.transaction_id,
        Some(owner_kind),
        context.owner_id.as_ref(),
        context.ofe_id.as_ref(),
        context.tile_id.as_ref(),
        context.surface_id,
        context.source_id,
        context.parcel_id,
        rollback_hashes,
        attempted_sha256,
        detail,
    )
    .into())
}

#[allow(clippy::too_many_arguments)]
fn finalization_numeric_failure(
    transaction_id: super::TransactionId,
    configuration: &DirectSurfaceLiquidConfiguration,
    owner_id: &ResourceOwnerId,
    owner_kind: OwnerKind,
    ofe_id: &OfeId,
    tile_id: &TileId,
    rollback_hashes: &[OwnerRollbackHash],
    attempted_sha256: &str,
    detail: &'static str,
) -> super::LandSurfaceEnergyShadowError {
    let record = configuration
        .records
        .iter()
        .find(|row| &row.key.ofe_id == ofe_id && &row.key.tile_id == tile_id);
    canonical_receiver_failure(
        DirectSurfaceLiquidErrorCode::E003,
        DirectSurfaceLiquidPhase::IndependentClosure,
        transaction_id,
        Some(owner_kind),
        Some(owner_id),
        Some(ofe_id),
        Some(tile_id),
        record.map(|row| row.key.surface_id.clone()),
        record.map(|row| row.key.source_id.clone()),
        None,
        rollback_hashes,
        attempted_sha256,
        detail,
    )
    .into()
}

#[allow(clippy::too_many_lines)] // Explicit complete owner-domain scan preserves E003 precedence.
fn validate_numeric_domains(
    operands: &RealReceiverClosureOperands,
) -> Result<(), DirectSurfaceLiquidError> {
    for lane in &operands.production_soil {
        let scalars = [
            lane.tillage_depth_m,
            lane.infiltration_m,
            lane.beginning_aggregate_soil_water_m,
            lane.ending_aggregate_soil_water_m,
        ];
        let invalid_layer = lane.ordered_layers.iter().any(|layer| {
            ![
                layer.beginning_liquid_m,
                layer.ending_liquid_m,
                layer.layer_depth_m,
                layer.residual_theta,
                layer.frozen_depth_m,
            ]
            .iter()
            .all(|value| value.is_finite())
                || layer.beginning_liquid_m < 0.0
                || layer.ending_liquid_m < 0.0
                || layer.layer_depth_m <= 0.0
                || layer.residual_theta < 0.0
                || layer.frozen_depth_m < 0.0
                || layer.frozen_depth_m > layer.layer_depth_m
        });
        if !scalars.iter().all(|value| value.is_finite())
            || lane.tillage_depth_m < 0.0
            || lane.infiltration_m < 0.0
            || lane.beginning_aggregate_soil_water_m < 0.0
            || lane.ending_aggregate_soil_water_m < 0.0
            || invalid_layer
            || !production_arithmetic_is_defined(lane)
        {
            return Err(receiver_arithmetic_failure(
                operands,
                OwnerKind::Hydrology,
                &operands.hydrology_owner_id,
                Some(&lane.ofe_id),
                None,
                "production soil receiver operand domain",
            ));
        }
    }
    for row in &operands.soil_thermal {
        if ![
            row.beginning_infiltration_credit_j_m2_ofe_ground,
            row.ending_infiltration_credit_j_m2_ofe_ground,
            row.beginning_enthalpy_j_m2_ofe_ground,
            row.infiltration_enthalpy_j_m2_ofe_ground,
            row.ending_enthalpy_j_m2_ofe_ground,
        ]
        .iter()
        .all(|value| value.is_finite())
            || checked_surface_liquid_add(
                row.beginning_infiltration_credit_j_m2_ofe_ground,
                row.infiltration_enthalpy_j_m2_ofe_ground,
            )
            .is_none()
            || checked_surface_liquid_add(
                row.beginning_enthalpy_j_m2_ofe_ground,
                row.infiltration_enthalpy_j_m2_ofe_ground,
            )
            .is_none()
        {
            return Err(receiver_arithmetic_failure(
                operands,
                OwnerKind::SoilThermal,
                &operands.soil_thermal_owner_id,
                Some(&row.ofe_id),
                Some(&row.tile_id),
                "soil-thermal receiver operand domain",
            ));
        }
    }
    for row in &operands.lse_tiles {
        if !row.tile_fraction.is_finite()
            || row.tile_fraction <= 0.0
            || row.tile_fraction > 1.0
            || ![
                row.beginning_enthalpy_j_m2_tile_ground,
                row.retained_enthalpy_j_m2_ofe_ground,
                row.ending_enthalpy_j_m2_tile_ground,
            ]
            .iter()
            .all(|value| value.is_finite())
            || checked_surface_liquid_div(row.retained_enthalpy_j_m2_ofe_ground, row.tile_fraction)
                .and_then(|retained| {
                    checked_surface_liquid_add(row.beginning_enthalpy_j_m2_tile_ground, retained)
                })
                .is_none()
        {
            return Err(receiver_arithmetic_failure(
                operands,
                OwnerKind::LandSurfaceEnergy,
                &operands.lse_owner_id,
                Some(&row.ofe_id),
                Some(&row.tile_id),
                "LSE tile receiver operand domain",
            ));
        }
    }
    Ok(())
}

fn production_arithmetic_is_defined(lane: &ProductionSoilReceiverOperands) -> bool {
    let totals = |ending| {
        lane.ordered_layers
            .iter()
            .map(|layer| {
                checked_receiver_layer_total(
                    if ending {
                        layer.ending_liquid_m
                    } else {
                        layer.beginning_liquid_m
                    },
                    layer,
                )
            })
            .collect::<Option<Vec<_>>>()
            .and_then(checked_surface_liquid_sum)
    };
    independently_reconstruct_infiltration(lane).is_some()
        && totals(false).is_some()
        && totals(true).is_some()
        && checked_surface_liquid_add(lane.beginning_aggregate_soil_water_m, lane.infiltration_m)
            .is_some()
}

fn first_mismatch<T: PartialEq>(expected: &[T], actual: &[T]) -> usize {
    (0..expected.len().max(actual.len()))
        .find(|&index| expected.get(index) != actual.get(index))
        .unwrap_or(0)
}

fn join_failure(
    operands: &RealReceiverClosureOperands,
    owner_kind: OwnerKind,
    owner_id: &ResourceOwnerId,
    ofe_id: Option<&OfeId>,
    tile_id: Option<&TileId>,
    detail: &'static str,
) -> DirectSurfaceLiquidError {
    receiver_operand_failure(
        operands,
        DirectSurfaceLiquidErrorCode::E010,
        DirectSurfaceLiquidPhase::IndependentClosure,
        owner_kind,
        owner_id,
        ofe_id,
        tile_id,
        detail,
    )
}

fn receiver_envelope_failure(
    operands: &RealReceiverClosureOperands,
    owner_kind: OwnerKind,
    owner_id: &ResourceOwnerId,
    ofe_id: Option<&OfeId>,
    tile_id: Option<&TileId>,
    detail: &'static str,
) -> DirectSurfaceLiquidError {
    receiver_operand_failure(
        operands,
        DirectSurfaceLiquidErrorCode::E011,
        DirectSurfaceLiquidPhase::AtomicEnvelope,
        owner_kind,
        owner_id,
        ofe_id,
        tile_id,
        detail,
    )
}
#[allow(clippy::too_many_lines)] // Complete explicit framing is the canonical anti-aliasing surface.
pub(super) fn receiver_operands_sha256(operands: &RealReceiverClosureOperands) -> String {
    let mut out = FramedSha256::new("openwepp-real-receiver-closure-operands-v5");
    out.u128("transaction", operands.transaction_id.0);
    out.string("hydrology_owner", operands.hydrology_owner_id.as_str());
    out.string("lse_owner", operands.lse_owner_id.as_str());
    out.string("thermal_owner", operands.soil_thermal_owner_id.as_str());
    out.string(
        "beginning_hydrology_snapshot",
        operands.beginning_hydrology_snapshot_sha256.as_str(),
    );
    out.string(
        "beginning_lse_state",
        operands.beginning_lse_state_sha256.as_str(),
    );
    out.string(
        "beginning_soil_thermal_state",
        operands.beginning_soil_thermal_state_sha256.as_str(),
    );
    frame_receiver_rollback_hashes(&mut out, &operands.rollback_hashes);
    out.count(
        "expected_production_count",
        operands.expected_production_soil.len(),
    );
    for (ofe, index, lane_id, layers) in &operands.expected_production_soil {
        out.string("expected_production_ofe", ofe.as_str());
        out.u64("expected_production_index", *index as u64);
        out.u64("expected_production_lane", u64::from(*lane_id));
        out.count("expected_production_layer_count", layers.len());
        for layer in layers {
            out.string("expected_production_layer", layer.as_str());
        }
    }
    out.count(
        "expected_thermal_count",
        operands.expected_soil_thermal.len(),
    );
    for (ofe, tile, layer) in &operands.expected_soil_thermal {
        out.string("expected_thermal_ofe", ofe.as_str());
        out.string("expected_thermal_tile", tile.as_str());
        out.string("expected_thermal_layer", layer.as_str());
    }
    out.count("expected_lse_count", operands.expected_lse_tiles.len());
    for (ofe, tile) in &operands.expected_lse_tiles {
        out.string("expected_lse_ofe", ofe.as_str());
        out.string("expected_lse_tile", tile.as_str());
    }
    frame_configured_receiver_context(&mut out, operands);
    out.count("production_count", operands.production_soil.len());
    for lane in &operands.production_soil {
        out.string("production_ofe", lane.ofe_id.as_str());
        out.u64("production_index", lane.production_lane_index as u64);
        out.u64("production_lane", u64::from(lane.production_lane_id));
        out.f64("tillage_depth", lane.tillage_depth_m);
        out.f64("infiltration", lane.infiltration_m);
        out.f64(
            "beginning_soil_water",
            lane.beginning_aggregate_soil_water_m,
        );
        out.f64("ending_soil_water", lane.ending_aggregate_soil_water_m);
        out.count("production_layer_count", lane.ordered_layers.len());
        for layer in &lane.ordered_layers {
            out.string("production_layer", layer.layer_id.as_str());
            out.f64("beginning_liquid", layer.beginning_liquid_m);
            out.f64("ending_liquid", layer.ending_liquid_m);
            out.f64("layer_depth", layer.layer_depth_m);
            out.f64("residual_theta", layer.residual_theta);
            out.f64("frozen_depth", layer.frozen_depth_m);
        }
    }
    out.count("thermal_count", operands.soil_thermal.len());
    for thermal in &operands.soil_thermal {
        out.string("thermal_ofe", thermal.ofe_id.as_str());
        out.string("thermal_tile", thermal.tile_id.as_str());
        out.string("thermal_layer", thermal.layer_id.as_str());
        out.f64(
            "beginning_infiltration_credit",
            thermal.beginning_infiltration_credit_j_m2_ofe_ground,
        );
        out.f64(
            "ending_infiltration_credit",
            thermal.ending_infiltration_credit_j_m2_ofe_ground,
        );
        out.f64(
            "beginning_enthalpy",
            thermal.beginning_enthalpy_j_m2_ofe_ground,
        );
        out.f64(
            "infiltration_enthalpy",
            thermal.infiltration_enthalpy_j_m2_ofe_ground,
        );
        out.f64("ending_enthalpy", thermal.ending_enthalpy_j_m2_ofe_ground);
    }
    out.count("lse_count", operands.lse_tiles.len());
    for tile in &operands.lse_tiles {
        out.string("lse_ofe", tile.ofe_id.as_str());
        out.string("lse_tile", tile.tile_id.as_str());
        out.f64("tile_fraction", tile.tile_fraction);
        out.f64(
            "beginning_surface_enthalpy",
            tile.beginning_enthalpy_j_m2_tile_ground,
        );
        out.f64("retained_enthalpy", tile.retained_enthalpy_j_m2_ofe_ground);
        out.f64(
            "ending_surface_enthalpy",
            tile.ending_enthalpy_j_m2_tile_ground,
        );
    }
    out.finish()
}

fn frame_receiver_rollback_hashes(out: &mut FramedSha256, rows: &[OwnerRollbackHash]) {
    out.count("rollback_count", rows.len());
    for row in rows {
        out.string("rollback_owner_kind", owner_kind_tag(row.owner_kind));
        out.string("rollback_owner", &row.owner_id);
        out.string("rollback_before", row.before_sha256.as_str());
        out.string("rollback_after", row.after_sha256.as_str());
    }
}

const fn owner_kind_tag(owner_kind: OwnerKind) -> &'static str {
    match owner_kind {
        OwnerKind::LandSurfaceEnergy => "land-surface-energy",
        OwnerKind::Hydrology => "hydrology",
        OwnerKind::SoilThermal => "soil-thermal",
        OwnerKind::Vegetation => "vegetation",
        OwnerKind::Biogeochemistry => "biogeochemistry",
        OwnerKind::Envelope => "envelope",
    }
}

fn frame_configured_receiver_context(
    out: &mut FramedSha256,
    operands: &RealReceiverClosureOperands,
) {
    out.count(
        "configured_surface_context_count",
        operands.configured_surface_context.len(),
    );
    for (ofe, tile, surface, source) in &operands.configured_surface_context {
        out.string("configured_surface_context_ofe", ofe.as_str());
        out.string("configured_surface_context_tile", tile.as_str());
        out.string("configured_surface_context_surface", surface.as_str());
        out.string("configured_surface_context_source", source.as_str());
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use openwepp_kernel_contract::TransactionId;
    use openwepp_land_surface_energy::SoilThermalLayerCandidate;

    fn digest(character: char) -> Sha256Digest {
        Sha256Digest::try_new(character.to_string().repeat(64)).expect("digest")
    }

    #[test]
    #[allow(clippy::too_many_lines)]
    fn post_ingress_numeric_preflight_uses_implicated_owner_rollback_hash_or_absence() {
        let ofe_id = OfeId::try_new("ofe-owner-hash").expect("OFE");
        let tile_id = TileId::try_new("tile-owner-hash").expect("tile");
        let lse_owner = ResourceOwnerId::try_new("lse-owner-hash").expect("LSE owner");
        let thermal_owner = ResourceOwnerId::try_new("thermal-owner-hash").expect("thermal owner");
        let hydrology_owner =
            ResourceOwnerId::try_new("hydrology-owner-hash").expect("hydrology owner");
        let configuration = DirectSurfaceLiquidConfiguration {
            owner_id: ResourceOwnerId::try_new("surface-owner-hash").expect("surface owner"),
            run_id: 1,
            configuration_sha256: String::new(),
            ofe_topology: Vec::new(),
            ofe_bindings: Vec::new(),
            records: Vec::new(),
        };
        let expectations = UnifiedReceiverExpectations::try_new(
            lse_owner.clone(),
            digest('2'),
            hydrology_owner.clone(),
            digest('3'),
            thermal_owner.clone(),
            digest('4'),
            vec![(
                ofe_id.clone(),
                tile_id.clone(),
                vec![SoilLayerId::try_new("layer-owner-hash").expect("layer")],
            )],
        )
        .expect("receiver expectations");
        let lse_tiles = vec![TileState {
            ofe_id: ofe_id.clone(),
            tile_id: tile_id.clone(),
            surface_enthalpy_j_m2_tile_ground: 10.0,
            surface_temperature_warm_start_k: 290.0,
        }];
        let thermal_tiles = vec![SoilThermalTileCandidate {
            owner_id: thermal_owner.clone(),
            beginning_state_sha256: digest('4'),
            ofe_id,
            tile_id,
            layers: vec![SoilThermalLayerCandidate {
                layer_id: SoilLayerId::try_new("layer-owner-hash").expect("layer"),
                beginning_enthalpy_j_m2_ofe_ground: 1.0,
                ground_heat_credit_j_m2_ofe_ground: 0.0,
                infiltration_enthalpy_credit_j_m2_ofe_ground: 0.0,
                ending_enthalpy_j_m2_ofe_ground: 1.0,
                ending_temperature_k: 290.0,
            }],
        }];
        let rollback = [
            (OwnerKind::LandSurfaceEnergy, &lse_owner, '2'),
            (OwnerKind::Hydrology, &hydrology_owner, '3'),
            (OwnerKind::SoilThermal, &thermal_owner, '4'),
        ]
        .into_iter()
        .map(|(owner_kind, owner_id, character)| OwnerRollbackHash {
            owner_kind,
            owner_id: owner_id.as_str().to_owned(),
            before_sha256: digest(character),
            after_sha256: digest(character),
        })
        .collect::<Vec<_>>();

        let mut invalid_lse = lse_tiles.clone();
        invalid_lse[0].surface_enthalpy_j_m2_tile_ground = f64::NAN;
        let lse_attempt = super::super::finalization_receiver_sets_sha256(
            &invalid_lse,
            &thermal_tiles,
            &rollback,
        );
        assert_numeric_owner_hash(
            preflight_finalization_receiver_numerics(
                TransactionId(41),
                &configuration,
                &expectations,
                &invalid_lse,
                &thermal_tiles,
                &rollback,
                &lse_attempt,
            ),
            "lse-owner-hash",
            Some(digest('2').as_str()),
            &lse_attempt,
        );

        let mut invalid_thermal = thermal_tiles.clone();
        invalid_thermal[0].layers[0].ending_temperature_k = f64::NAN;
        let thermal_attempt = super::super::finalization_receiver_sets_sha256(
            &lse_tiles,
            &invalid_thermal,
            &rollback,
        );
        assert_numeric_owner_hash(
            preflight_finalization_receiver_numerics(
                TransactionId(41),
                &configuration,
                &expectations,
                &lse_tiles,
                &invalid_thermal,
                &rollback,
                &thermal_attempt,
            ),
            "thermal-owner-hash",
            Some(digest('4').as_str()),
            &thermal_attempt,
        );

        let mut duplicate_lse = rollback.clone();
        duplicate_lse.push(rollback[0].clone());
        let duplicate_attempt = super::super::finalization_receiver_sets_sha256(
            &invalid_lse,
            &thermal_tiles,
            &duplicate_lse,
        );
        assert_numeric_owner_hash(
            preflight_finalization_receiver_numerics(
                TransactionId(41),
                &configuration,
                &expectations,
                &invalid_lse,
                &thermal_tiles,
                &duplicate_lse,
                &duplicate_attempt,
            ),
            "lse-owner-hash",
            None,
            &duplicate_attempt,
        );
    }

    fn assert_numeric_owner_hash(
        result: Result<(), LandSurfaceEnergyShadowError>,
        owner_id: &str,
        beginning: Option<&str>,
        attempted: &str,
    ) {
        let LandSurfaceEnergyShadowError::SurfaceLiquid(error) =
            result.expect_err("numeric receiver poison")
        else {
            panic!("numeric receiver poison must remain canonical");
        };
        let failure = error.failure().expect("canonical numeric failure");
        assert_eq!(failure.code, DirectSurfaceLiquidErrorCode::E003);
        assert_eq!(
            failure
                .context
                .owner_id
                .as_ref()
                .map(ResourceOwnerId::as_str),
            Some(owner_id)
        );
        assert_eq!(
            failure.rollback.beginning_owner_sha256.as_deref(),
            beginning
        );
        assert_eq!(
            failure.rollback.attempted_owner_sha256.as_deref(),
            Some(attempted)
        );
    }

    #[allow(clippy::too_many_lines)]
    fn fixture() -> RealReceiverClosureOperands {
        let hydrology_owner_id =
            ResourceOwnerId::try_new("production-hydrology").expect("hydrology owner");
        let lse_owner_id = ResourceOwnerId::try_new("land-surface-energy-v1").expect("LSE owner");
        let soil_thermal_owner_id =
            ResourceOwnerId::try_new("soil-thermal").expect("thermal owner");
        let mut production_soil = Vec::new();
        let mut soil_thermal = Vec::new();
        let mut lse_tiles = Vec::new();
        for index in 1..=2 {
            let ofe_id = OfeId::try_new(format!("ofe-{index}")).expect("OFE");
            let tile_id = TileId::try_new(format!("tile-{index}")).expect("tile");
            let layer_id = SoilLayerId::try_new(format!("layer-{index}")).expect("layer");
            production_soil.push(ProductionSoilReceiverOperands {
                ofe_id: ofe_id.clone(),
                production_lane_index: index - 1,
                production_lane_id: u32::try_from(index).expect("bounded lane ID"),
                tillage_depth_m: 0.1,
                infiltration_m: 0.0,
                beginning_aggregate_soil_water_m: 0.01,
                ending_aggregate_soil_water_m: 0.01,
                ordered_layers: vec![ProductionSoilLayerReceiverOperands {
                    layer_id: layer_id.clone(),
                    beginning_liquid_m: 0.01,
                    ending_liquid_m: 0.01,
                    layer_depth_m: 0.1,
                    residual_theta: 0.0,
                    frozen_depth_m: 0.0,
                }],
            });
            soil_thermal.push(super::super::SoilThermalReceiverOperands {
                ofe_id: ofe_id.clone(),
                tile_id: tile_id.clone(),
                layer_id: layer_id.clone(),
                beginning_infiltration_credit_j_m2_ofe_ground: 0.0,
                ending_infiltration_credit_j_m2_ofe_ground: 0.0,
                beginning_enthalpy_j_m2_ofe_ground: 10.0,
                infiltration_enthalpy_j_m2_ofe_ground: 0.0,
                ending_enthalpy_j_m2_ofe_ground: 10.0,
            });
            lse_tiles.push(super::super::LseTileReceiverOperands {
                ofe_id,
                tile_id,
                tile_fraction: 0.5,
                beginning_enthalpy_j_m2_tile_ground: 10.0,
                retained_enthalpy_j_m2_ofe_ground: 0.0,
                ending_enthalpy_j_m2_tile_ground: 10.0,
            });
        }
        let expected_production_soil = production_soil
            .iter()
            .map(|row| {
                (
                    row.ofe_id.clone(),
                    row.production_lane_index,
                    row.production_lane_id,
                    row.ordered_layers
                        .iter()
                        .map(|layer| layer.layer_id.clone())
                        .collect(),
                )
            })
            .collect();
        let expected_soil_thermal = soil_thermal
            .iter()
            .map(|row| {
                (
                    row.ofe_id.clone(),
                    row.tile_id.clone(),
                    row.layer_id.clone(),
                )
            })
            .collect();
        let expected_lse_tiles = lse_tiles
            .iter()
            .map(|row| (row.ofe_id.clone(), row.tile_id.clone()))
            .collect();
        let configured_surface_context = lse_tiles
            .iter()
            .enumerate()
            .map(|(index, row)| {
                (
                    row.ofe_id.clone(),
                    row.tile_id.clone(),
                    SurfaceId::try_new(format!("surface-{}", index + 1)).expect("surface"),
                    SourceId::try_new(format!("source-{}", index + 1)).expect("source"),
                )
            })
            .collect();
        RealReceiverClosureOperands {
            transaction_id: TransactionId(41),
            rollback_hashes: vec![
                OwnerRollbackHash {
                    owner_kind: OwnerKind::LandSurfaceEnergy,
                    owner_id: lse_owner_id.as_str().to_owned(),
                    before_sha256: digest('b'),
                    after_sha256: digest('b'),
                },
                OwnerRollbackHash {
                    owner_kind: OwnerKind::Hydrology,
                    owner_id: hydrology_owner_id.as_str().to_owned(),
                    before_sha256: digest('a'),
                    after_sha256: digest('a'),
                },
                OwnerRollbackHash {
                    owner_kind: OwnerKind::SoilThermal,
                    owner_id: soil_thermal_owner_id.as_str().to_owned(),
                    before_sha256: digest('c'),
                    after_sha256: digest('c'),
                },
            ],
            beginning_lse_state_sha256: digest('b'),
            beginning_soil_thermal_state_sha256: digest('c'),
            hydrology_owner_id,
            lse_owner_id,
            soil_thermal_owner_id,
            beginning_hydrology_snapshot_sha256: Sha256Digest::try_new("a".repeat(64))
                .expect("digest"),
            production_soil,
            soil_thermal,
            lse_tiles,
            expected_production_soil,
            expected_soil_thermal,
            expected_lse_tiles,
            configured_surface_context,
        }
    }

    fn assert_later_e010(operands: &RealReceiverClosureOperands, owner: &str, tile: Option<&str>) {
        let error = validate_real_receiver_closure(operands).expect_err("later mismatch");
        let failure = error.failure().expect("canonical failure");
        assert_eq!(failure.code, DirectSurfaceLiquidErrorCode::E010);
        assert_eq!(failure.phase, DirectSurfaceLiquidPhase::IndependentClosure);
        assert_eq!(
            failure
                .context
                .owner_id
                .as_ref()
                .map(ResourceOwnerId::as_str),
            Some(owner)
        );
        assert_eq!(
            failure.context.ofe_id.as_ref().map(OfeId::as_str),
            Some("ofe-2")
        );
        assert_eq!(failure.context.tile_id.as_ref().map(TileId::as_str), tile);
        assert_eq!(
            failure.context.surface_id.as_ref().map(SurfaceId::as_str),
            Some("surface-2")
        );
        assert_eq!(
            failure.context.source_id.as_ref().map(SourceId::as_str),
            Some("source-2")
        );
        assert!(failure.context.parcel_id.is_none());
    }

    #[test]
    fn later_receiver_equation_mismatches_are_e010_and_cardinality_is_e011() {
        let mut production = fixture();
        production.production_soil[1].ordered_layers[0].ending_liquid_m += 1.0e-3;
        production.production_soil[1].ending_aggregate_soil_water_m += 1.0e-3;
        assert_later_e010(&production, "production-hydrology", None);

        let mut thermal = fixture();
        thermal.soil_thermal[1].ending_enthalpy_j_m2_ofe_ground += 1.0;
        assert_later_e010(&thermal, "soil-thermal", Some("tile-2"));

        let mut lse = fixture();
        lse.lse_tiles[1].ending_enthalpy_j_m2_tile_ground += 1.0;
        assert_later_e010(&lse, "land-surface-energy-v1", Some("tile-2"));

        let mut incomplete = fixture();
        incomplete.soil_thermal.pop();
        let error = validate_real_receiver_closure(&incomplete).expect_err("missing owner row");
        let failure = error.failure().expect("canonical envelope failure");
        assert_eq!(failure.code, DirectSurfaceLiquidErrorCode::E011);
        assert_eq!(failure.phase, DirectSurfaceLiquidPhase::AtomicEnvelope);
        assert_eq!(
            failure.context.ofe_id.as_ref().map(OfeId::as_str),
            Some("ofe-2")
        );
        assert_eq!(
            failure.context.tile_id.as_ref().map(TileId::as_str),
            Some("tile-2")
        );
        assert_eq!(
            failure.context.surface_id.as_ref().map(SurfaceId::as_str),
            Some("surface-2")
        );
        assert_eq!(
            failure.context.source_id.as_ref().map(SourceId::as_str),
            Some("source-2")
        );
    }

    #[test]
    fn receiver_close_e010_and_e003_carry_configured_surface_context() {
        let operands = fixture();
        let ofe = &operands.soil_thermal[1].ofe_id;
        let tile = &operands.soil_thermal[1].tile_id;
        for (actual, expected, code) in [
            (11.0, 10.0, DirectSurfaceLiquidErrorCode::E010),
            (f64::NAN, 10.0, DirectSurfaceLiquidErrorCode::E003),
        ] {
            let error = require_receiver_close(
                &operands,
                OwnerKind::SoilThermal,
                &operands.soil_thermal_owner_id,
                actual,
                expected,
                DirectSurfaceLiquidClosureUnit::EnthalpyJM2,
                Some(ofe),
                Some(tile),
                "receiver-close context poison",
            )
            .expect_err("poison must fail");
            let failure = error.failure().expect("canonical failure");
            assert_eq!(failure.code, code);
            assert_eq!(
                failure.context.surface_id.as_ref().map(SurfaceId::as_str),
                Some("surface-2")
            );
            assert_eq!(
                failure.context.source_id.as_ref().map(SourceId::as_str),
                Some("source-2")
            );
            assert!(failure.context.parcel_id.is_none());
        }
    }

    #[test]
    fn topology_join_and_context_digest_bind_configured_ids() {
        let baseline = fixture();
        let baseline_sha256 = receiver_operands_sha256(&baseline);

        let mut topology = baseline.clone();
        topology.soil_thermal.swap(0, 1);
        let error = validate_real_receiver_closure(&topology).expect_err("topology poison");
        let failure = error.failure().expect("canonical join failure");
        assert_eq!(failure.code, DirectSurfaceLiquidErrorCode::E010);
        assert_eq!(
            failure.context.surface_id.as_ref().map(SurfaceId::as_str),
            Some("surface-2")
        );
        assert_eq!(
            failure.context.source_id.as_ref().map(SourceId::as_str),
            Some("source-2")
        );

        let mut identity_poison = baseline;
        identity_poison.configured_surface_context[1].3 =
            SourceId::try_new("source-2-poison").expect("source poison");
        assert_ne!(
            receiver_operands_sha256(&identity_poison),
            baseline_sha256,
            "configured receiver context must be digest-bound"
        );
    }
}
