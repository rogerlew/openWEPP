//! Exact immutable beginning bytes for frozen-litter V3 rejection.

use openwepp_land_surface_energy::{
    LandSurfaceEnergyV3State, SoilThermalOwnerEnvelopeV2, SoilThermalOwnerRestartV2,
};

use crate::direct_runtime::{
    DirectWb14ParentWorkingStateV2, SurfaceLiquidConfigurationV2, SurfaceLiquidOwnerEnvelopeV2,
};

use super::v3_input_projection::FrozenLitterV3RuntimeError;

#[derive(Clone, Debug, PartialEq)]
pub(crate) struct FrozenLitterV3RollbackSnapshot {
    surface_owner: Vec<u8>,
    lse_owner: Vec<u8>,
    soil_owner: Vec<u8>,
    soil_restart: Vec<u8>,
    wb14_parent: Option<Vec<u8>>,
}

impl FrozenLitterV3RollbackSnapshot {
    pub(crate) fn capture(
        surface_configuration: &SurfaceLiquidConfigurationV2,
        surface_owner: &SurfaceLiquidOwnerEnvelopeV2,
        lse_owner: &LandSurfaceEnergyV3State,
        soil_owner: &SoilThermalOwnerEnvelopeV2,
        soil_restart: &SoilThermalOwnerRestartV2,
        wb14_parent: Option<&DirectWb14ParentWorkingStateV2>,
    ) -> Result<Self, FrozenLitterV3RuntimeError> {
        surface_owner
            .canonical_bytes(surface_configuration.parent(), Some(surface_configuration))?;
        soil_owner.validate()?;
        Ok(Self {
            surface_owner: surface_owner
                .canonical_bytes(surface_configuration.parent(), Some(surface_configuration))?,
            lse_owner: lse_owner.to_json()?,
            soil_owner: serde_json::to_vec(soil_owner).map_err(|_| {
                FrozenLitterV3RuntimeError::Serialization("soil owner rollback bytes")
            })?,
            soil_restart: serde_json::to_vec(soil_restart).map_err(|_| {
                FrozenLitterV3RuntimeError::Serialization("soil restart rollback bytes")
            })?,
            wb14_parent: wb14_parent
                .map(|parent| parent.restart_bytes(surface_configuration))
                .transpose()?,
        })
    }

    #[allow(dead_code)]
    pub(crate) fn require_exactly_unchanged(
        &self,
        surface_configuration: &SurfaceLiquidConfigurationV2,
        surface_owner: &SurfaceLiquidOwnerEnvelopeV2,
        lse_owner: &LandSurfaceEnergyV3State,
        soil_owner: &SoilThermalOwnerEnvelopeV2,
        soil_restart: &SoilThermalOwnerRestartV2,
        wb14_parent: Option<&DirectWb14ParentWorkingStateV2>,
    ) -> Result<(), FrozenLitterV3RuntimeError> {
        let actual = Self::capture(
            surface_configuration,
            surface_owner,
            lse_owner,
            soil_owner,
            soil_restart,
            wb14_parent,
        )?;
        if actual != *self {
            return Err(FrozenLitterV3RuntimeError::Closure(
                "rejected candidate changed beginning owner bytes",
            ));
        }
        Ok(())
    }
}
