//! Checked V1-to-V2 projection boundaries.

use openwepp_kernel_contract::ResourceOwnerId;
use openwepp_land_surface_energy::{
    Sha256Digest, SoilThermalOwnerCheckpointV2, SoilThermalOwnerEnvelopeV2,
    SoilThermalOwnerRestartV2, SoilThermalV2MigrationIdentity, migrate_soil_thermal_v1_to_v2,
};
use thiserror::Error;

use crate::{
    CompleteCommittedOwnerStateV1, CompleteCommittedOwnerStateV2, ScientificOwnerStateSetV1,
    ScientificOwnerStateSetV2, Sha256Hex, SoilThermalNativeBundleV2,
    SoilThermalNativeSealAuthorityV2, SoilThermalOwnerStateRestartV2, SoilThermalRestartV2Error,
    SoilThermalStateRestartV1, canonical_sha256,
};

/// Missing native LSE constructor boundary. The implementation must originate
/// both seals from the authoritative owner implementation, never from a digest
/// formula duplicated by persisted restart.
pub trait SoilThermalNativeSealConstructorV2: SoilThermalNativeSealAuthorityV2 {
    fn construct_seals(
        &self,
        envelope: &SoilThermalOwnerEnvelopeV2,
    ) -> Result<(SoilThermalOwnerRestartV2, SoilThermalOwnerCheckpointV2), &'static str>;
}

#[derive(Debug, Error)]
pub enum RestartProjectionV2Error {
    #[error("v1_soil_owner")]
    V1SoilOwner,
    #[error("native_migration")]
    NativeMigration,
    #[error("native_seal_constructor_unavailable")]
    NativeSealConstructorUnavailable,
    #[error(transparent)]
    Restart(#[from] SoilThermalRestartV2Error),
}

pub fn migrate_soil_thermal_restart_v1_to_v2(
    parent: SoilThermalStateRestartV1,
    expected_owner_id: &ResourceOwnerId,
    expected_configuration_sha256: &Sha256Digest,
    identity: SoilThermalV2MigrationIdentity,
    constructor: &dyn SoilThermalNativeSealConstructorV2,
) -> Result<SoilThermalOwnerStateRestartV2, RestartProjectionV2Error> {
    let snapshot = parent
        .restore(expected_owner_id, expected_configuration_sha256)
        .map_err(|_| RestartProjectionV2Error::V1SoilOwner)?;
    let envelope = migrate_soil_thermal_v1_to_v2(&snapshot, identity)
        .map_err(|_| RestartProjectionV2Error::NativeMigration)?;
    let (restart_seal, checkpoint_seal) = constructor
        .construct_seals(&envelope)
        .map_err(|_| RestartProjectionV2Error::NativeSealConstructorUnavailable)?;
    SoilThermalOwnerStateRestartV2::from_native(
        parent,
        SoilThermalNativeBundleV2 {
            owner_envelope: envelope,
            restart_seal,
            checkpoint_seal,
            credit_beginning_owner_envelope: None,
            latest_credit_receipt: None,
            expected_accepted_operands: Vec::new(),
            expected_temperature_projections: Vec::new(),
            native_expected_source_set: None,
            native_orchestrator_seals: None,
        },
        expected_owner_id,
        expected_configuration_sha256,
        constructor,
    )
    .map_err(Into::into)
}

#[must_use]
pub fn substitute_scientific_soil_owner_v2(
    parent: ScientificOwnerStateSetV1,
    soil_thermal_v2: SoilThermalOwnerStateRestartV2,
) -> ScientificOwnerStateSetV2 {
    ScientificOwnerStateSetV2 {
        vegetation_v10: parent.vegetation_v10,
        lse_v2: parent.lse_v2,
        direct_hydrology: parent.direct_hydrology,
        soil_thermal_v2,
        biogeochemistry: parent.biogeochemistry,
    }
}

/// Explicit successor projection after the authoritative V2 soil owner has
/// already been constructed and sealed.
#[must_use]
pub fn project_scientific_owner_state_v2(
    unaffected_v1: ScientificOwnerStateSetV1,
    soil_thermal_v2: SoilThermalOwnerStateRestartV2,
) -> ScientificOwnerStateSetV2 {
    substitute_scientific_soil_owner_v2(unaffected_v1, soil_thermal_v2)
}

#[must_use]
pub fn substitute_complete_soil_owner_v2(
    parent: CompleteCommittedOwnerStateV1,
    soil_thermal_v2: SoilThermalOwnerStateRestartV2,
) -> CompleteCommittedOwnerStateV2 {
    CompleteCommittedOwnerStateV2 {
        gsi_configuration: parent.gsi_configuration,
        gsi_state: parent.gsi_state,
        static_forcing_configuration: parent.static_forcing_configuration,
        provider_cursor: parent.provider_cursor,
        surface_liquid_configuration: parent.surface_liquid_configuration,
        scientific: substitute_scientific_soil_owner_v2(parent.scientific, soil_thermal_v2),
    }
}

/// Explicit successor complete-owner projection. No V1 soil owner remains in
/// the projected scientific set; it survives only as the immutable migration
/// parent embedded inside the V2 soil DTO.
#[must_use]
pub fn project_complete_owner_state_v2(
    unaffected_v1: CompleteCommittedOwnerStateV1,
    soil_thermal_v2: SoilThermalOwnerStateRestartV2,
) -> CompleteCommittedOwnerStateV2 {
    substitute_complete_soil_owner_v2(unaffected_v1, soil_thermal_v2)
}

pub fn checkpoint_identities_v2(
    committed: &CompleteCommittedOwnerStateV2,
    root_zone_hydraulic_configuration: &openwepp_hillslope_orchestrator::v9_real_consumer_shadow::DirectRootZoneHydraulicConfiguration,
) -> Result<(Sha256Hex, Sha256Hex), RestartProjectionV2Error> {
    let hydrology = &committed.scientific.direct_hydrology;
    let run = Sha256Hex::try_new(
        canonical_sha256(&(
            hydrology.run_id,
            hydrology.hillslope_id,
            hydrology.lane_count,
            hydrology.day_count,
        ))
        .map_err(|_| RestartProjectionV2Error::NativeMigration)?,
    )
    .map_err(|_| RestartProjectionV2Error::NativeMigration)?;
    let soil = committed
        .scientific
        .soil_thermal_v2
        .decode_native()
        .map_err(RestartProjectionV2Error::from)?
        .owner_envelope;
    let topology = serde_json::json!({
        "ordered_lanes": hydrology.lanes.iter().map(|lane| serde_json::json!({
            "lane_id": lane.lane_id,
            "upstream_lane_id": lane.upstream_lane_id,
            "downstream_lane_id": lane.downstream_lane_id,
            "soil_layer_count": lane.subsurface_layers.len(),
        })).collect::<Vec<_>>(),
        "ordered_ofe_tiles": committed.static_forcing_configuration.destinations.iter().map(|destination| (
            &destination.ofe_id, &destination.tile_id, &destination.wb14_configuration_sha256,
        )).collect::<Vec<_>>(),
        "lse_tiles": committed.scientific.lse_v2.tiles.iter().map(|tile| (&tile.ofe_id, &tile.tile_id)).collect::<Vec<_>>(),
        "soil_thermal_layer_maps": soil.state.ofes.iter().map(|ofe| (
            ofe.ofe_id.as_str(),
            ofe.ordered_layers.iter().map(|layer| layer.layer_id.as_str()).collect::<Vec<_>>(),
        )).collect::<Vec<_>>(),
        "root_zone_hydraulic_configuration_sha256": root_zone_hydraulic_configuration
            .restart_identity_sha256()
            .map_err(|_| RestartProjectionV2Error::NativeMigration)?,
    });
    let topology = Sha256Hex::try_new(
        canonical_sha256(&topology).map_err(|_| RestartProjectionV2Error::NativeMigration)?,
    )
    .map_err(|_| RestartProjectionV2Error::NativeMigration)?;
    Ok((run, topology))
}
