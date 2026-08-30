//! Versioned complete-owner DTOs with the authoritative soil-thermal V2 owner.

use openwepp_kernel_contract::ResourceOwnerId;
use openwepp_land_surface_energy::{LandSurfaceEnergyConfiguration, SoilThermalOwnerEnvelopeV2};
use serde::{Deserialize, Serialize};

use crate::{
    BiogeochemistryStateRestartV1, DirectGsiOwnerConfigurationRestartV1,
    DirectGsiOwnerStateRestartV1, DirectHydrologyRestartV1,
    DirectSurfaceLiquidConfigurationRestartV1, LseV2StateRestartV1,
    SnowFreeHalfHourProviderCursorRestartV1, SnowFreeHalfHourStaticConfigurationRestartV1,
    SoilThermalNativeSealAuthorityV2, SoilThermalOwnerStateRestartV2, SoilThermalRestartV2Error,
    VegetationV10StateRestartV1,
};

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct ScientificOwnerStateSetV2 {
    pub vegetation_v10: VegetationV10StateRestartV1,
    pub lse_v2: LseV2StateRestartV1,
    pub direct_hydrology: DirectHydrologyRestartV1,
    pub soil_thermal_v2: SoilThermalOwnerStateRestartV2,
    pub biogeochemistry: BiogeochemistryStateRestartV1,
}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct CompleteCommittedOwnerStateV2 {
    pub gsi_configuration: DirectGsiOwnerConfigurationRestartV1,
    pub gsi_state: DirectGsiOwnerStateRestartV1,
    pub static_forcing_configuration: SnowFreeHalfHourStaticConfigurationRestartV1,
    pub provider_cursor: SnowFreeHalfHourProviderCursorRestartV1,
    pub surface_liquid_configuration: DirectSurfaceLiquidConfigurationRestartV1,
    pub scientific: ScientificOwnerStateSetV2,
}

impl ScientificOwnerStateSetV2 {
    pub fn validate_soil_owner(
        &self,
        expected_owner_id: &ResourceOwnerId,
        configuration: &LandSurfaceEnergyConfiguration,
        authority: &dyn SoilThermalNativeSealAuthorityV2,
    ) -> Result<SoilThermalOwnerEnvelopeV2, SoilThermalRestartV2Error> {
        let envelope = self.soil_thermal_v2.validate_with_configuration(
            expected_owner_id,
            configuration,
            authority,
        )?;
        let lse_ofes = self
            .lse_v2
            .tiles
            .iter()
            .map(|tile| tile.ofe_id.as_str())
            .collect::<Vec<_>>();
        let soil_ofes = envelope
            .state
            .ofes
            .iter()
            .map(|ofe| ofe.ofe_id.as_str())
            .collect::<Vec<_>>();
        let unique_lse_ofes = lse_ofes
            .iter()
            .copied()
            .collect::<std::collections::BTreeSet<_>>();
        if unique_lse_ofes.into_iter().collect::<Vec<_>>() != soil_ofes {
            return Err(SoilThermalRestartV2Error::OwnerIdentity);
        }
        Ok(envelope)
    }
}
