//! Checked V1-to-V2 projection boundaries.

use openwepp_kernel_contract::ResourceOwnerId;
use openwepp_land_surface_energy::{
    PreparedSoilThermalSupportV2, Sha256Digest, SoilThermalOwnerCheckpointV2,
    SoilThermalOwnerEnvelopeV2, SoilThermalOwnerRestartV2, SoilThermalReceiptFreeOwnerSealsV2,
    SoilThermalV2MigrationIdentity, migrate_soil_thermal_v1_to_v2, prepare_soil_thermal_support_v2,
    seal_soil_thermal_receipt_free_owner_v2, validate_soil_thermal_receipt_free_owner_v2,
};
use thiserror::Error;

use crate::{
    CompleteCommittedOwnerStateV1, CompleteCommittedOwnerStateV2, ScientificOwnerStateSetV1,
    ScientificOwnerStateSetV2, Sha256Hex, SoilThermalNativeBundleV2,
    SoilThermalNativeSealAuthorityV2, SoilThermalOwnerStateRestartV2, SoilThermalRestartV2Error,
    SoilThermalStateRestartV1, canonical_sha256,
};

#[derive(Debug, Error)]
pub enum RestartProjectionV2Error {
    #[error("v1_soil_owner")]
    V1SoilOwner,
    #[error("native_migration")]
    NativeMigration,
    #[error(transparent)]
    Restart(#[from] SoilThermalRestartV2Error),
}

struct ReceiptFreeNativeSealAuthorityV2<'a> {
    prepared: &'a PreparedSoilThermalSupportV2,
    seals: &'a SoilThermalReceiptFreeOwnerSealsV2,
}

impl SoilThermalNativeSealAuthorityV2 for ReceiptFreeNativeSealAuthorityV2<'_> {
    fn validate_restart_seal(
        &self,
        envelope: &SoilThermalOwnerEnvelopeV2,
        seal: &SoilThermalOwnerRestartV2,
    ) -> Result<(), &'static str> {
        if envelope != self.prepared.beginning_owner() || seal != &self.seals.restart {
            return Err("receipt-free restart join");
        }
        validate_soil_thermal_receipt_free_owner_v2(self.prepared, self.seals)
            .map_err(|_| "receipt-free restart seal")
    }

    fn validate_checkpoint_seal(
        &self,
        envelope: &SoilThermalOwnerEnvelopeV2,
        seal: &SoilThermalOwnerCheckpointV2,
    ) -> Result<(), &'static str> {
        if envelope != self.prepared.beginning_owner() || seal != &self.seals.checkpoint {
            return Err("receipt-free checkpoint join");
        }
        validate_soil_thermal_receipt_free_owner_v2(self.prepared, self.seals)
            .map_err(|_| "receipt-free checkpoint seal")
    }
}

pub fn project_receipt_free_soil_thermal_owner_state_v2(
    parent_v1: SoilThermalStateRestartV1,
    prepared: &PreparedSoilThermalSupportV2,
    seals: &SoilThermalReceiptFreeOwnerSealsV2,
) -> Result<SoilThermalOwnerStateRestartV2, RestartProjectionV2Error> {
    validate_soil_thermal_receipt_free_owner_v2(prepared, seals)
        .map_err(|_| RestartProjectionV2Error::NativeMigration)?;
    let owner = prepared.beginning_owner();
    let authority = ReceiptFreeNativeSealAuthorityV2 { prepared, seals };
    SoilThermalOwnerStateRestartV2::from_native(
        parent_v1,
        SoilThermalNativeBundleV2 {
            owner_envelope: owner.clone(),
            restart_seal: seals.restart.clone(),
            checkpoint_seal: seals.checkpoint.clone(),
            credit_beginning_owner_envelope: None,
            latest_credit_receipt: None,
            expected_accepted_operands: Vec::new(),
            expected_temperature_projections: Vec::new(),
            native_expected_source_set: None,
            native_orchestrator_seals: None,
        },
        &owner.state.owner_id,
        &owner.state.configuration_sha256,
        &authority,
    )
    .map_err(Into::into)
}

pub fn bootstrap_soil_thermal_restart_v1_to_v2(
    parent: SoilThermalStateRestartV1,
    expected_owner_id: &ResourceOwnerId,
    expected_configuration_sha256: &Sha256Digest,
    identity: SoilThermalV2MigrationIdentity,
) -> Result<SoilThermalOwnerStateRestartV2, RestartProjectionV2Error> {
    let snapshot = parent
        .restore(expected_owner_id, expected_configuration_sha256)
        .map_err(|_| RestartProjectionV2Error::V1SoilOwner)?;
    let envelope = migrate_soil_thermal_v1_to_v2(&snapshot, identity)
        .map_err(|_| RestartProjectionV2Error::NativeMigration)?;
    let prepared = prepare_soil_thermal_support_v2(
        &envelope,
        envelope.transaction_id,
        envelope.support_start_ns,
        envelope.support_end_ns,
    )
    .map_err(|_| RestartProjectionV2Error::NativeMigration)?;
    let seals = seal_soil_thermal_receipt_free_owner_v2(&prepared)
        .map_err(|_| RestartProjectionV2Error::NativeMigration)?;
    project_receipt_free_soil_thermal_owner_state_v2(parent, &prepared, &seals)
}

/// Checked one-way bootstrap of a complete committed owner set. The V1 soil
/// payload is retained only as the immutable parent bound by the V2 owner;
/// runtime custody is the canonical native zero-carry V2 envelope.
pub fn bootstrap_complete_owner_state_v1_to_v2(
    parent: CompleteCommittedOwnerStateV1,
    expected_owner_id: &ResourceOwnerId,
    expected_configuration_sha256: &Sha256Digest,
    identity: SoilThermalV2MigrationIdentity,
) -> Result<CompleteCommittedOwnerStateV2, RestartProjectionV2Error> {
    let soil_thermal_v2 = bootstrap_soil_thermal_restart_v1_to_v2(
        parent.scientific.soil_thermal.clone(),
        expected_owner_id,
        expected_configuration_sha256,
        identity,
    )?;
    Ok(substitute_complete_soil_owner_v2(parent, soil_thermal_v2))
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
