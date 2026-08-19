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
        soil_thermal: SoilThermalStateRestartV1::project(shadow.restart_authority_soil_thermal())
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
