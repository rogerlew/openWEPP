use super::*;

pub(super) fn finalization_expectations(
    protocol: &WaterProtocol,
    thermal: &[SoilThermalTileCandidate],
) -> UnifiedReceiverExpectations {
    let thermal_owner = thermal.first().map_or_else(
        || ResourceOwnerId::try_new("soil-thermal").expect("thermal owner"),
        |tile| tile.owner_id.clone(),
    );
    let thermal_beginning = thermal
        .first()
        .map_or_else(|| digest('4'), |tile| tile.beginning_state_sha256.clone());
    UnifiedReceiverExpectations::try_new(
        ResourceOwnerId::try_new("land-surface-energy-v1").expect("LSE owner"),
        digest('2'),
        protocol.hydrology_owner_id.clone(),
        protocol.beginning_snapshot_sha256.clone(),
        thermal_owner,
        thermal_beginning,
        thermal
            .iter()
            .map(|tile| {
                (
                    tile.ofe_id.clone(),
                    tile.tile_id.clone(),
                    tile.layers
                        .iter()
                        .map(|layer| layer.layer_id.clone())
                        .collect(),
                )
            })
            .collect(),
    )
    .expect("finalization expectations")
}
