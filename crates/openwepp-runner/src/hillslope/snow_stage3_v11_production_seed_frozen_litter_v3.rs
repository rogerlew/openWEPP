//! Checked bootstrap of the additive native frozen-litter V3 resident.

use std::collections::BTreeMap;

use openwepp_hillslope_orchestrator::v9_real_consumer_shadow::{
    DirectV10RealConsumerError, FrozenLitterV3Resident,
};
use openwepp_hillslope_orchestrator::{
    DirectSurfaceLiquidConfiguration, DirectSurfaceLiquidError, DirectSurfaceLiquidOwnedState,
    DirectSurfaceLiquidStoreKey, SurfaceLiquidConfigurationV2, SurfaceLiquidOwnerEnvelopeV2,
    SurfaceLiquidOwnerModelDefinitionV2, migrate_v1_to_v2,
};
use openwepp_land_surface_energy::{
    LandSurfaceEnergyConfiguration, LandSurfaceEnergyV2State, SurfaceConfiguration,
    migrate_v2_configuration_to_v3, migrate_v2_state_to_v3,
};

// These are the immutable contract bytes bound by the canonical LSE V3 model
// definition. Later narrative amendments do not silently re-identify a model.
const LSE_V3_CONTRACT_SHA256: &str =
    "857b49f06fdb675cd91fe2776727388aea72d19fdb999e2e4cd6e248f0e836d1";
const SURFACE_LIQUID_V2_CONTRACT_SHA256: &str =
    "bbb165f03c2f3588b32d4e97b41757612a73ef2641c4b4c8ae4d07f4a66df7e8";

pub(super) fn bootstrap_frozen_litter_v3_resident(
    lse_v2_configuration: &LandSurfaceEnergyConfiguration,
    lse_v2_state: &LandSurfaceEnergyV2State,
    surface_v1_configuration: &DirectSurfaceLiquidConfiguration,
    surface_v1_state: &DirectSurfaceLiquidOwnedState,
) -> Result<FrozenLitterV3Resident, DirectV10RealConsumerError> {
    let lse_v3_configuration = migrate_v2_configuration_to_v3(lse_v2_configuration)?;
    let lse_v3_state =
        migrate_v2_state_to_v3(lse_v2_configuration, lse_v2_state, &lse_v3_configuration)?;

    let model = SurfaceLiquidOwnerModelDefinitionV2::new(
        SURFACE_LIQUID_V2_CONTRACT_SHA256,
        LSE_V3_CONTRACT_SHA256,
        lse_v2_configuration
            .hydrology_configuration
            .model_definition_sha256
            .as_str(),
    )?;
    let litter_depths =
        litter_depths_by_surface_key(lse_v2_configuration, surface_v1_configuration)?;
    let surface_v2_configuration =
        SurfaceLiquidConfigurationV2::new(surface_v1_configuration.clone(), model, &litter_depths)?;
    let enthalpy = surface_enthalpy_by_key(&lse_v3_state, surface_v1_configuration)?;
    let surface_v2_state =
        migrate_v1_to_v2(&surface_v2_configuration, surface_v1_state, &enthalpy)?;
    let surface_v2_owner =
        SurfaceLiquidOwnerEnvelopeV2::wrap_v2(&surface_v2_configuration, surface_v2_state)?;

    FrozenLitterV3Resident::try_new(
        lse_v3_configuration,
        lse_v3_state,
        surface_v2_configuration,
        surface_v2_owner,
    )
}

fn litter_depths_by_surface_key(
    lse_configuration: &LandSurfaceEnergyConfiguration,
    surface_configuration: &DirectSurfaceLiquidConfiguration,
) -> Result<BTreeMap<DirectSurfaceLiquidStoreKey, f64>, DirectSurfaceLiquidError> {
    surface_configuration
        .records
        .iter()
        .filter(|record| {
            record.key.surface_class == openwepp_land_surface_energy::SurfaceClass::ForestLitter
        })
        .map(|record| {
            let configured = configured_surface(lse_configuration, &record.key)?;
            let SurfaceConfiguration::ForestLitter { thickness_m, .. } = configured else {
                return Err(DirectSurfaceLiquidError::Identity(
                    "frozen-litter V3 seed surface-class/configuration join",
                ));
            };
            Ok((record.key.clone(), *thickness_m))
        })
        .collect()
}

fn surface_enthalpy_by_key(
    lse_state: &openwepp_land_surface_energy::LandSurfaceEnergyV3State,
    surface_configuration: &DirectSurfaceLiquidConfiguration,
) -> Result<BTreeMap<DirectSurfaceLiquidStoreKey, f64>, DirectSurfaceLiquidError> {
    surface_configuration
        .records
        .iter()
        .map(|record| {
            let tile = lse_state
                .0
                .tiles
                .iter()
                .find(|tile| tile.ofe_id == record.key.ofe_id && tile.tile_id == record.key.tile_id)
                .ok_or(DirectSurfaceLiquidError::Identity(
                    "frozen-litter V3 seed LSE/surface state topology",
                ))?;
            Ok((record.key.clone(), tile.surface_enthalpy_j_m2_tile_ground))
        })
        .collect()
}

fn configured_surface<'a>(
    configuration: &'a LandSurfaceEnergyConfiguration,
    key: &DirectSurfaceLiquidStoreKey,
) -> Result<&'a SurfaceConfiguration, DirectSurfaceLiquidError> {
    configuration
        .ofes
        .iter()
        .find(|ofe| ofe.ofe_id == key.ofe_id)
        .and_then(|ofe| ofe.tiles.iter().find(|tile| tile.tile_id == key.tile_id))
        .map(|tile| &tile.surface)
        .ok_or(DirectSurfaceLiquidError::Identity(
            "frozen-litter V3 seed LSE/surface configuration topology",
        ))
}
