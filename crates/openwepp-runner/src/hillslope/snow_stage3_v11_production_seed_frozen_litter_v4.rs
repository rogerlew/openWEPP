//! Checked V16 bootstrap over the immutable V3 physical resident.

use openwepp_hillslope_orchestrator::v9_real_consumer_shadow::{
    DirectV10RealConsumerError, FrozenLitterV3Resident,
};
use openwepp_hillslope_orchestrator::{
    DirectSurfaceLiquidConfiguration, DirectSurfaceLiquidOwnedState,
    LseSurfaceEnthalpyOwnerEnvelopeV1,
};
use openwepp_kernel_contract::ResourceOwnerId;
use openwepp_land_surface_energy::{LandSurfaceEnergyConfiguration, LandSurfaceEnergyV2State};

use super::snow_stage3_v11_production_seed_frozen_litter_v3::bootstrap_frozen_litter_v3_resident;

pub(super) fn bootstrap_frozen_litter_v4_resident(
    lse_v2_configuration: &LandSurfaceEnergyConfiguration,
    lse_v2_state: &LandSurfaceEnergyV2State,
    surface_v1_configuration: &DirectSurfaceLiquidConfiguration,
    surface_v1_state: &DirectSurfaceLiquidOwnedState,
) -> Result<(FrozenLitterV3Resident, LseSurfaceEnthalpyOwnerEnvelopeV1), DirectV10RealConsumerError>
{
    let physical = bootstrap_frozen_litter_v3_resident(
        lse_v2_configuration,
        lse_v2_state,
        surface_v1_configuration,
        surface_v1_state,
    )?;
    let owner_id = ResourceOwnerId::try_new(format!(
        "{}::lse-surface-enthalpy-v1",
        physical.lse_configuration().owner_id.as_str()
    ))
    .map_err(|_| {
        openwepp_hillslope_orchestrator::v9_real_consumer_shadow::DirectV9RealConsumerError::Identity(
            "frozen-litter V4 exact owner identity",
        )
    })?;
    let exact = LseSurfaceEnthalpyOwnerEnvelopeV1::adopt_from_frozen_v2_v3(
        owner_id,
        physical.lse_configuration(),
        physical.lse_state(),
        physical.surface_configuration(),
        physical.surface_owner(),
    )
    .map_err(|error| {
        openwepp_hillslope_orchestrator::v9_real_consumer_shadow::DirectV9RealConsumerError::Serialization(
            error.to_string(),
        )
    })?;
    Ok((physical, exact))
}
