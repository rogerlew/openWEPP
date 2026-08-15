//! Independent receiver-lineage sealing for standalone LSE finalization.

use super::{
    BTreeSet, OwnerKind, OwnerRollbackHash, ReceiverEnvelopeViolation, RequestingComponent,
    ResourceOwnerId, Sha256Digest, SoilThermalTileCandidate, TileState,
    UnifiedReceiverExpectations, WaterProtocol, missing_rollback_detail, rollback_violation,
};

#[allow(clippy::too_many_lines)]
pub(super) fn first_sealed_finalization_violation(
    expectations: &UnifiedReceiverExpectations,
    protocol: &WaterProtocol,
    lse_tiles: &[TileState],
    thermal_tiles: &[SoilThermalTileCandidate],
    rollback_hashes: &[OwnerRollbackHash],
) -> Option<ReceiverEnvelopeViolation> {
    if protocol.hydrology_owner_id != expectations.hydrology_owner_id
        || protocol.beginning_snapshot_sha256 != expectations.beginning_hydrology_snapshot_sha256
    {
        return Some(ReceiverEnvelopeViolation::for_owner(
            OwnerKind::Hydrology,
            Some(protocol.hydrology_owner_id.clone()),
            "hydrology protocol differs from independent expectations",
        ));
    }
    if protocol
        .requests
        .iter()
        .filter(|request| request.key.requesting_component == RequestingComponent::GroundSurface)
        .any(|request| request.key.requesting_owner_id != expectations.lse_owner_id)
    {
        return Some(ReceiverEnvelopeViolation::for_owner(
            OwnerKind::LandSurfaceEnergy,
            Some(expectations.lse_owner_id.clone()),
            "LSE request owner differs from independent expectations",
        ));
    }
    let lse_owner = Some(expectations.lse_owner_id.clone());
    if lse_tiles.is_empty() {
        return Some(ReceiverEnvelopeViolation::for_owner(
            OwnerKind::LandSurfaceEnergy,
            lse_owner,
            "missing sealed LSE tile receiver",
        ));
    }
    if let Some(tile) = first_duplicate_lse_tile(lse_tiles) {
        return Some(ReceiverEnvelopeViolation::for_tile(
            OwnerKind::LandSurfaceEnergy,
            lse_owner,
            tile.ofe_id.clone(),
            tile.tile_id.clone(),
            "duplicate sealed LSE tile receiver",
        ));
    }
    if let Some(tile) = first_duplicate_thermal_tile(thermal_tiles) {
        return Some(ReceiverEnvelopeViolation::for_tile(
            OwnerKind::SoilThermal,
            Some(tile.owner_id.clone()),
            tile.ofe_id.clone(),
            tile.tile_id.clone(),
            "duplicate sealed soil-thermal tile receiver",
        ));
    }
    for index in 0..lse_tiles.len().max(thermal_tiles.len()) {
        let lse = lse_tiles.get(index);
        let thermal = thermal_tiles.get(index);
        if lse.map(|tile| (&tile.ofe_id, &tile.tile_id))
            != thermal.map(|tile| (&tile.ofe_id, &tile.tile_id))
        {
            return Some(match thermal {
                Some(tile) => ReceiverEnvelopeViolation::for_tile(
                    OwnerKind::SoilThermal,
                    Some(tile.owner_id.clone()),
                    tile.ofe_id.clone(),
                    tile.tile_id.clone(),
                    "sealed LSE/soil-thermal tile receiver mismatch",
                ),
                None => ReceiverEnvelopeViolation::for_tile(
                    OwnerKind::SoilThermal,
                    unique_sealed_thermal_owner(thermal_tiles, rollback_hashes),
                    lse?.ofe_id.clone(),
                    lse?.tile_id.clone(),
                    "missing sealed soil-thermal tile receiver",
                ),
            });
        }
    }
    if let Some(candidate) = thermal_tiles.iter().find(|candidate| {
        candidate.layers.is_empty()
            || candidate
                .layers
                .iter()
                .map(|layer| &layer.layer_id)
                .collect::<BTreeSet<_>>()
                .len()
                != candidate.layers.len()
    }) {
        return Some(ReceiverEnvelopeViolation::for_tile(
            OwnerKind::SoilThermal,
            Some(candidate.owner_id.clone()),
            candidate.ofe_id.clone(),
            candidate.tile_id.clone(),
            "invalid sealed soil-thermal layer receiver set",
        ));
    }
    let expected_tiles = expectations
        .ordered_thermal_layers
        .iter()
        .map(|(identity, _)| identity)
        .collect::<Vec<_>>();
    let ground_requests = protocol
        .requests
        .iter()
        .filter(|request| request.key.requesting_component == RequestingComponent::GroundSurface)
        .collect::<Vec<_>>();
    for expected in &expected_tiles {
        if !ground_requests.iter().any(|request| {
            request.key.ofe_id == expected.0 && request.key.requesting_tile_id == expected.1
        }) {
            return Some(ReceiverEnvelopeViolation::cardinality_for_tile(
                OwnerKind::LandSurfaceEnergy,
                Some(expectations.lse_owner_id.clone()),
                expected.0.clone(),
                expected.1.clone(),
                "missing independently expected ground D/A/F identity",
            ));
        }
    }
    if let Some(request) = ground_requests.iter().find(|request| {
        !expected_tiles.iter().any(|expected| {
            request.key.ofe_id == expected.0 && request.key.requesting_tile_id == expected.1
        })
    }) {
        return Some(ReceiverEnvelopeViolation::cardinality_for_tile(
            OwnerKind::LandSurfaceEnergy,
            Some(expectations.lse_owner_id.clone()),
            request.key.ofe_id.clone(),
            request.key.requesting_tile_id.clone(),
            "unexpected ground D/A/F identity outside independent expectations",
        ));
    }
    for (index, expected) in expected_tiles.iter().enumerate() {
        let lse = lse_tiles.get(index);
        let thermal = thermal_tiles.get(index);
        if lse.map(|tile| (&tile.ofe_id, &tile.tile_id)) != Some((&expected.0, &expected.1)) {
            return Some(lse.map_or_else(
                || {
                    ReceiverEnvelopeViolation::for_tile(
                        OwnerKind::LandSurfaceEnergy,
                        Some(expectations.lse_owner_id.clone()),
                        expected.0.clone(),
                        expected.1.clone(),
                        "missing independently expected LSE tile receiver",
                    )
                },
                |tile| {
                    ReceiverEnvelopeViolation::for_tile(
                        OwnerKind::LandSurfaceEnergy,
                        Some(expectations.lse_owner_id.clone()),
                        tile.ofe_id.clone(),
                        tile.tile_id.clone(),
                        "unexpected LSE tile receiver identity",
                    )
                },
            ));
        }
        let Some(thermal) = thermal else {
            return Some(ReceiverEnvelopeViolation::for_tile(
                OwnerKind::SoilThermal,
                Some(expectations.soil_thermal_owner_id.clone()),
                expected.0.clone(),
                expected.1.clone(),
                "missing independently expected soil-thermal tile receiver",
            ));
        };
        if thermal.owner_id != expectations.soil_thermal_owner_id
            || thermal.beginning_state_sha256 != expectations.beginning_soil_thermal_state_sha256
            || thermal
                .layers
                .iter()
                .map(|layer| &layer.layer_id)
                .collect::<Vec<_>>()
                != expectations.ordered_thermal_layers[index]
                    .1
                    .iter()
                    .collect::<Vec<_>>()
        {
            return Some(ReceiverEnvelopeViolation::for_tile(
                OwnerKind::SoilThermal,
                Some(thermal.owner_id.clone()),
                thermal.ofe_id.clone(),
                thermal.tile_id.clone(),
                "soil-thermal receiver differs from independent expectations",
            ));
        }
    }
    if lse_tiles.len() != expected_tiles.len() || thermal_tiles.len() != expected_tiles.len() {
        let (kind, owner, ofe, tile) = lse_tiles
            .get(expected_tiles.len())
            .map(|row| {
                (
                    OwnerKind::LandSurfaceEnergy,
                    expectations.lse_owner_id.clone(),
                    row.ofe_id.clone(),
                    row.tile_id.clone(),
                )
            })
            .or_else(|| {
                thermal_tiles.get(expected_tiles.len()).map(|row| {
                    (
                        OwnerKind::SoilThermal,
                        row.owner_id.clone(),
                        row.ofe_id.clone(),
                        row.tile_id.clone(),
                    )
                })
            })?;
        return Some(ReceiverEnvelopeViolation::for_tile(
            kind,
            Some(owner),
            ofe,
            tile,
            "unexpected receiver outside independent expectations",
        ));
    }
    first_sealed_rollback_violation(expectations, rollback_hashes)
}

fn first_duplicate_lse_tile(tiles: &[TileState]) -> Option<&TileState> {
    let mut seen = BTreeSet::new();
    tiles
        .iter()
        .find(|tile| !seen.insert((tile.ofe_id.clone(), tile.tile_id.clone())))
}

fn first_duplicate_thermal_tile(
    tiles: &[SoilThermalTileCandidate],
) -> Option<&SoilThermalTileCandidate> {
    let mut seen = BTreeSet::new();
    tiles
        .iter()
        .find(|tile| !seen.insert((tile.ofe_id.clone(), tile.tile_id.clone())))
}

fn first_sealed_rollback_violation(
    expectations: &UnifiedReceiverExpectations,
    rows: &[OwnerRollbackHash],
) -> Option<ReceiverEnvelopeViolation> {
    let expected = [
        (
            OwnerKind::LandSurfaceEnergy,
            &expectations.lse_owner_id,
            &expectations.beginning_lse_state_sha256,
        ),
        (
            OwnerKind::Hydrology,
            &expectations.hydrology_owner_id,
            &expectations.beginning_hydrology_snapshot_sha256,
        ),
        (
            OwnerKind::SoilThermal,
            &expectations.soil_thermal_owner_id,
            &expectations.beginning_soil_thermal_state_sha256,
        ),
    ];
    if rows.len() < expected.len() {
        for (kind, owner, _) in &expected {
            if !rows
                .iter()
                .any(|row| row.owner_kind == *kind && row.owner_id == owner.as_str())
            {
                if let Some(substituted) = rows.iter().find(|row| row.owner_kind == *kind) {
                    return Some(rollback_violation(
                        substituted,
                        "substituted sealed rollback owner row",
                    ));
                }
                return Some(ReceiverEnvelopeViolation::for_owner(
                    *kind,
                    Some((*owner).clone()),
                    missing_rollback_detail(*kind),
                ));
            }
        }
    }
    for (index, (kind, owner, beginning)) in expected.iter().enumerate() {
        let Some(actual) = rows.get(index) else {
            return Some(ReceiverEnvelopeViolation::for_owner(
                *kind,
                Some((*owner).clone()),
                missing_rollback_detail(*kind),
            ));
        };
        if actual.owner_kind != *kind
            || actual.owner_id != owner.as_str()
            || &actual.before_sha256 != *beginning
            || actual.before_sha256 != actual.after_sha256
        {
            return Some(rollback_violation(
                actual,
                "unexpected sealed rollback owner row",
            ));
        }
    }
    rows.get(expected.len())
        .map(|unexpected| rollback_violation(unexpected, "unexpected sealed rollback owner row"))
}

fn unique_sealed_thermal_lineage(
    thermal_tiles: &[SoilThermalTileCandidate],
) -> Option<(ResourceOwnerId, Sha256Digest)> {
    let mut lineages = thermal_tiles
        .iter()
        .map(|candidate| (&candidate.owner_id, &candidate.beginning_state_sha256));
    let first = lineages.next()?;
    lineages
        .all(|lineage| lineage == first)
        .then(|| (first.0.clone(), first.1.clone()))
}

fn unique_sealed_thermal_owner(
    thermal_tiles: &[SoilThermalTileCandidate],
    rollback_hashes: &[OwnerRollbackHash],
) -> Option<ResourceOwnerId> {
    unique_sealed_thermal_lineage(thermal_tiles)
        .map(|(owner, _)| owner)
        .or_else(|| {
            let mut rows = rollback_hashes
                .iter()
                .filter(|row| row.owner_kind == OwnerKind::SoilThermal);
            let first = rows.next()?;
            rows.next()
                .is_none()
                .then(|| ResourceOwnerId::try_new(first.owner_id.clone()).ok())?
        })
}
