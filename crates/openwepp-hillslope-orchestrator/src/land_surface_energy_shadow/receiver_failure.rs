//! Canonical receiver-failure provenance and contextual construction.

use super::{
    DirectSurfaceLiquidError, DirectSurfaceLiquidErrorCode, DirectSurfaceLiquidErrorContext,
    DirectSurfaceLiquidPhase, DirectSurfaceLiquidRollbackHashes, OfeId, OwnerKind,
    OwnerRollbackHash, RealReceiverClosureOperands, ResourceOwnerId, SourceId, SurfaceId, TileId,
};

#[allow(clippy::too_many_arguments)]
pub(super) fn canonical_receiver_failure(
    code: DirectSurfaceLiquidErrorCode,
    phase: DirectSurfaceLiquidPhase,
    transaction_id: super::TransactionId,
    owner_kind: Option<OwnerKind>,
    owner_id: Option<&ResourceOwnerId>,
    ofe_id: Option<&OfeId>,
    tile_id: Option<&TileId>,
    surface_id: Option<SurfaceId>,
    source_id: Option<SourceId>,
    parcel_id: Option<String>,
    rollback_hashes: &[OwnerRollbackHash],
    attempted_sha256: &str,
    detail: &'static str,
) -> DirectSurfaceLiquidError {
    let beginning_owner_sha256 = owner_kind
        .zip(owner_id)
        .and_then(|(kind, owner)| unique_owner_beginning_rollback(rollback_hashes, kind, owner));
    DirectSurfaceLiquidError::canonical_failure(
        code,
        phase,
        DirectSurfaceLiquidErrorContext {
            transaction_id: Some(transaction_id),
            owner_id: owner_id.cloned(),
            ofe_id: ofe_id.cloned(),
            tile_id: tile_id.cloned(),
            surface_id,
            source_id,
            parcel_id,
        },
        DirectSurfaceLiquidRollbackHashes {
            beginning_owner_sha256,
            attempted_owner_sha256: Some(attempted_sha256.to_owned()),
        },
        detail,
    )
}

#[allow(clippy::too_many_arguments)]
pub(super) fn receiver_operand_failure(
    operands: &RealReceiverClosureOperands,
    code: DirectSurfaceLiquidErrorCode,
    phase: DirectSurfaceLiquidPhase,
    owner_kind: OwnerKind,
    owner_id: &ResourceOwnerId,
    ofe_id: Option<&OfeId>,
    tile_id: Option<&TileId>,
    detail: &'static str,
) -> DirectSurfaceLiquidError {
    let (surface_id, source_id) = configured_receiver_context(operands, ofe_id, tile_id);
    canonical_receiver_failure(
        code,
        phase,
        operands.transaction_id,
        Some(owner_kind),
        Some(owner_id),
        ofe_id,
        tile_id,
        surface_id,
        source_id,
        None,
        &operands.rollback_hashes,
        &super::receiver_validation::receiver_operands_sha256(operands),
        detail,
    )
}

pub(super) fn unique_owner_beginning_rollback(
    rows: &[OwnerRollbackHash],
    owner_kind: OwnerKind,
    owner_id: &ResourceOwnerId,
) -> Option<String> {
    let mut matching = rows
        .iter()
        .filter(|row| row.owner_kind == owner_kind && row.owner_id.as_str() == owner_id.as_str());
    let beginning = matching.next()?.before_sha256.to_string();
    matching.next().is_none().then_some(beginning)
}

fn configured_receiver_context(
    operands: &RealReceiverClosureOperands,
    ofe_id: Option<&OfeId>,
    tile_id: Option<&TileId>,
) -> (Option<SurfaceId>, Option<SourceId>) {
    let Some(ofe_id) = ofe_id else {
        return (None, None);
    };
    let exact = tile_id.and_then(|tile_id| {
        operands
            .configured_surface_context
            .iter()
            .find(|(ofe, tile, _, _)| ofe == ofe_id && tile == tile_id)
    });
    let context = exact.or_else(|| {
        let mut matches = operands
            .configured_surface_context
            .iter()
            .filter(|(ofe, _, _, _)| ofe == ofe_id);
        let first = matches.next()?;
        matches.next().is_none().then_some(first)
    });
    context.map_or((None, None), |(_, _, surface, source)| {
        (Some(surface.clone()), Some(source.clone()))
    })
}
