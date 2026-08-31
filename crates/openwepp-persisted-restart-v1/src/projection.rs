use openwepp_hillslope_orchestrator::{
    runtime_inputs::restart_authority_project_gsi_state,
    v9_real_consumer_shadow::DirectV10RealConsumerShadow,
};

use crate::{
    BiogeochemistryStateRestartV1, CompleteCommittedOwnerStateV1,
    DirectGsiOwnerConfigurationRestartV1, DirectGsiOwnerStateRestartV1, DirectHydrologyRestartV1,
    DirectSurfaceLiquidConfigurationRestartV1, LseV2StateRestartV1, ScientificOwnerStateSetV1,
    Sha256Hex, SnowFreeHalfHourProviderCursorRestartV1,
    SnowFreeHalfHourStaticConfigurationRestartV1, SoilThermalStateRestartV1,
    VegetationV10StateRestartV1,
};

/// Project every scientific owner from the actual default-off consumer.
pub fn project_scientific_owner_state_v1(
    shadow: &DirectV10RealConsumerShadow,
    phase_plan_sha256: &Sha256Hex,
    day_input_digests: &[Sha256Hex],
) -> Result<ScientificOwnerStateSetV1, &'static str> {
    Ok(ScientificOwnerStateSetV1 {
        vegetation_v10: VegetationV10StateRestartV1::project(
            shadow.vegetation_state(),
            shadow.restart_authority_vegetation_configuration(),
            shadow.restart_authority_vegetation_owner_id(),
        )
        .map_err(|_| "vegetation projection")?,
        lse_v2: LseV2StateRestartV1::project(
            shadow.lse_state(),
            shadow.restart_authority_lse_configuration(),
        )
        .map_err(|_| "LSE projection")?,
        direct_hydrology: DirectHydrologyRestartV1::project(
            shadow.restart_authority_hydrology_frame(),
            phase_plan_sha256.clone(),
            day_input_digests,
        )
        .map_err(|_| "hydrology projection")?,
        soil_thermal: SoilThermalStateRestartV1::project(
            shadow
                .restart_authority_soil_thermal()
                .map_err(|_| "V1 soil resident projection")?,
        )
        .map_err(|_| "soil projection")?,
        biogeochemistry: BiogeochemistryStateRestartV1::project(
            shadow.restart_authority_biogeochemistry(),
        )
        .map_err(|_| "biogeochemistry projection")?,
    })
}

/// Project the complete between-days owner envelope from the actual consumer.
pub fn project_complete_owner_state_v1(
    shadow: &DirectV10RealConsumerShadow,
    phase_plan_sha256: &Sha256Hex,
    day_input_digests: &[Sha256Hex],
    expected_next_day_index: usize,
) -> Result<CompleteCommittedOwnerStateV1, &'static str> {
    let native_gsi_state = restart_authority_project_gsi_state(shadow.gsi_state())
        .map_err(|_| "GSI native projection")?;
    Ok(CompleteCommittedOwnerStateV1 {
        gsi_configuration: DirectGsiOwnerConfigurationRestartV1::project(
            shadow.gsi_owner_configuration(),
        )
        .map_err(|_| "GSI configuration projection")?,
        gsi_state: DirectGsiOwnerStateRestartV1::project(&native_gsi_state)
            .map_err(|_| "GSI state projection")?,
        static_forcing_configuration: SnowFreeHalfHourStaticConfigurationRestartV1::project(
            shadow.provider_static_configuration(),
        )
        .map_err(|_| "forcing configuration projection")?,
        provider_cursor: SnowFreeHalfHourProviderCursorRestartV1::project(
            shadow.provider_cursor(),
            shadow.provider_static_configuration(),
            expected_next_day_index,
        )
        .map_err(|_| "provider cursor projection")?,
        surface_liquid_configuration: DirectSurfaceLiquidConfigurationRestartV1::project(
            shadow.restart_authority_surface_configuration(),
        )
        .map_err(|_| "surface configuration projection")?,
        scientific: project_scientific_owner_state_v1(
            shadow,
            phase_plan_sha256,
            day_input_digests,
        )?,
    })
}

/// Derive the released run and topology identities from a complete owner set.
pub fn checkpoint_identities_v1(
    committed: &CompleteCommittedOwnerStateV1,
    root_zone_hydraulic_configuration: &openwepp_hillslope_orchestrator::v9_real_consumer_shadow::DirectRootZoneHydraulicConfiguration,
) -> Result<(Sha256Hex, Sha256Hex), &'static str> {
    let hydrology = &committed.scientific.direct_hydrology;
    let run = Sha256Hex::try_new(
        crate::canonical_sha256(&(
            hydrology.run_id,
            hydrology.hillslope_id,
            hydrology.lane_count,
            hydrology.day_count,
        ))
        .map_err(|_| "run identity projection")?,
    )
    .map_err(|_| "run identity projection")?;
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
        "soil_thermal_layer_maps": committed.scientific.soil_thermal.ofes.iter().map(|ofe| (
            &ofe.ofe_id,
            ofe.ordered_layers.iter().map(|layer| &layer.layer_id).collect::<Vec<_>>(),
        )).collect::<Vec<_>>(),
        "root_zone_hydraulic_configuration_sha256": root_zone_hydraulic_configuration
            .restart_identity_sha256()
            .map_err(|_| "root-zone configuration identity projection")?,
    });
    let topology = Sha256Hex::try_new(
        crate::canonical_sha256(&topology).map_err(|_| "topology identity projection")?,
    )
    .map_err(|_| "topology identity projection")?;
    Ok((run, topology))
}
