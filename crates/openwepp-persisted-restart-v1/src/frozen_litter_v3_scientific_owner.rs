//! Immutable persisted scientific-owner set for frozen-litter V3.

use openwepp_hillslope_orchestrator::{SurfaceLiquidConfigurationV2, SurfaceLiquidOwnerEnvelopeV2};
use openwepp_kernel_contract::ResourceOwnerId;
use openwepp_land_surface_energy::{
    LandSurfaceEnergyConfiguration, LandSurfaceEnergyV3State, SoilThermalOwnerEnvelopeV2,
    SoilThermalOwnerRestartV2,
};
use serde::{Deserialize, Serialize};
use thiserror::Error;

use crate::{
    FrozenLitterProjectionRestartError, FrozenLitterProjectionSealAuthorityV3,
    FrozenLitterPublicationAuthorityV3, Sha256Hex, SoilThermalNativeBundleV2,
    SoilThermalNativeSealAuthorityV2, SoilThermalOwnerStateRestartV2, canonical_sha256,
};

pub const FROZEN_LITTER_SCIENTIFIC_OWNER_V3_SCHEMA: &str =
    "OPENWEPP_FROZEN_LITTER_SCIENTIFIC_OWNER_V3";

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct FrozenLitterScientificOwnerStateV3 {
    pub schema: String,
    pub version: u16,
    pub lse_configuration_json: Vec<u8>,
    pub lse_v3_state_json: Vec<u8>,
    pub surface_liquid_v2_configuration_bytes: Vec<u8>,
    pub surface_liquid_v2_envelope_bytes: Vec<u8>,
    pub wb14_v2_parent_working_state_bytes: Vec<u8>,
    pub soil_thermal_v2: SoilThermalOwnerStateRestartV2,
    pub complete_owner_projection_v3_bytes: Vec<u8>,
    pub publication_authority: FrozenLitterPublicationAuthorityV3,
    pub scientific_owner_sha256: Sha256Hex,
}

#[derive(Serialize)]
struct ScientificOwnerDigestBody<'a> {
    schema: &'a str,
    version: u16,
    lse_configuration_json: &'a [u8],
    lse_v3_state_json: &'a [u8],
    surface_liquid_v2_configuration_bytes: &'a [u8],
    surface_liquid_v2_envelope_bytes: &'a [u8],
    wb14_v2_parent_working_state_bytes: &'a [u8],
    soil_thermal_v2: &'a SoilThermalOwnerStateRestartV2,
    complete_owner_projection_v3_bytes: &'a [u8],
    publication_authority: &'a FrozenLitterPublicationAuthorityV3,
}

pub struct FrozenLitterExpectedScientificContextV3<'a> {
    pub lse_configuration: &'a LandSurfaceEnergyConfiguration,
    pub surface_liquid_configuration: &'a SurfaceLiquidConfigurationV2,
    pub soil_thermal_owner_id: &'a ResourceOwnerId,
    pub soil_thermal_seal_authority: &'a dyn SoilThermalNativeSealAuthorityV2,
    pub projection_seal_authority: &'a dyn FrozenLitterProjectionSealAuthorityV3,
}

#[derive(Clone, Debug, PartialEq)]
pub struct RestoredFrozenLitterScientificOwnerV3 {
    pub persisted: FrozenLitterScientificOwnerStateV3,
    pub lse_v3: LandSurfaceEnergyV3State,
    pub surface_liquid_v2: SurfaceLiquidOwnerEnvelopeV2,
    pub wb14_v2_parent_working_state_bytes: Vec<u8>,
    pub soil_thermal_v2: SoilThermalOwnerEnvelopeV2,
    pub soil_thermal_restart_v2: SoilThermalOwnerRestartV2,
}

#[derive(Debug, Error, Clone, Copy, Eq, PartialEq)]
pub enum FrozenLitterScientificOwnerRestartError {
    #[error("schema")]
    Schema,
    #[error("unsupported_version")]
    UnsupportedVersion,
    #[error("canonical_or_digest")]
    Canonical,
    #[error("lse_v3")]
    Lse,
    #[error("surface_liquid_v2")]
    SurfaceLiquid,
    #[error("soil_thermal_v2")]
    SoilThermal,
    #[error("complete_projection_v3")]
    Projection,
    #[error("cross_owner_identity")]
    Identity,
    #[error("receipt_chain")]
    ReceiptChain,
}

impl From<FrozenLitterProjectionRestartError> for FrozenLitterScientificOwnerRestartError {
    fn from(_: FrozenLitterProjectionRestartError) -> Self {
        Self::Projection
    }
}

impl FrozenLitterScientificOwnerStateV3 {
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        lse_configuration: &LandSurfaceEnergyConfiguration,
        lse_v3: &LandSurfaceEnergyV3State,
        surface_configuration: &SurfaceLiquidConfigurationV2,
        surface_owner: &SurfaceLiquidOwnerEnvelopeV2,
        wb14_v2_parent_working_state_bytes: Vec<u8>,
        soil_thermal_v2: SoilThermalOwnerStateRestartV2,
        complete_owner_projection_v3_bytes: Vec<u8>,
        publication_authority: FrozenLitterPublicationAuthorityV3,
        context: &FrozenLitterExpectedScientificContextV3<'_>,
    ) -> Result<Self, FrozenLitterScientificOwnerRestartError> {
        let mut value = Self {
            schema: FROZEN_LITTER_SCIENTIFIC_OWNER_V3_SCHEMA.to_owned(),
            version: 3,
            lse_configuration_json: serde_json::to_vec(lse_configuration)
                .map_err(|_| FrozenLitterScientificOwnerRestartError::Canonical)?,
            lse_v3_state_json: lse_v3
                .to_json()
                .map_err(|_| FrozenLitterScientificOwnerRestartError::Lse)?,
            surface_liquid_v2_configuration_bytes: surface_configuration
                .canonical_bytes()
                .map_err(|_| FrozenLitterScientificOwnerRestartError::SurfaceLiquid)?,
            surface_liquid_v2_envelope_bytes: surface_owner
                .canonical_bytes(surface_configuration.parent(), Some(surface_configuration))
                .map_err(|_| FrozenLitterScientificOwnerRestartError::SurfaceLiquid)?,
            wb14_v2_parent_working_state_bytes,
            soil_thermal_v2,
            complete_owner_projection_v3_bytes,
            publication_authority,
            scientific_owner_sha256: zero_digest()?,
        };
        value.scientific_owner_sha256 = value.compute_digest()?;
        value.validate(context)?;
        Ok(value)
    }

    pub fn compute_digest(&self) -> Result<Sha256Hex, FrozenLitterScientificOwnerRestartError> {
        wire_digest(
            &canonical_sha256(&ScientificOwnerDigestBody {
                schema: &self.schema,
                version: self.version,
                lse_configuration_json: &self.lse_configuration_json,
                lse_v3_state_json: &self.lse_v3_state_json,
                surface_liquid_v2_configuration_bytes: &self.surface_liquid_v2_configuration_bytes,
                surface_liquid_v2_envelope_bytes: &self.surface_liquid_v2_envelope_bytes,
                wb14_v2_parent_working_state_bytes: &self.wb14_v2_parent_working_state_bytes,
                soil_thermal_v2: &self.soil_thermal_v2,
                complete_owner_projection_v3_bytes: &self.complete_owner_projection_v3_bytes,
                publication_authority: &self.publication_authority,
            })
            .map_err(|_| FrozenLitterScientificOwnerRestartError::Canonical)?,
        )
    }

    pub fn validate(
        &self,
        context: &FrozenLitterExpectedScientificContextV3<'_>,
    ) -> Result<RestoredFrozenLitterScientificOwnerV3, FrozenLitterScientificOwnerRestartError>
    {
        if self.schema != FROZEN_LITTER_SCIENTIFIC_OWNER_V3_SCHEMA {
            return Err(FrozenLitterScientificOwnerRestartError::Schema);
        }
        if self.version != 3 {
            return Err(FrozenLitterScientificOwnerRestartError::UnsupportedVersion);
        }
        if self.scientific_owner_sha256 != self.compute_digest()? {
            return Err(FrozenLitterScientificOwnerRestartError::Canonical);
        }
        self.publication_authority.validate()?;
        context
            .lse_configuration
            .validate_v3()
            .map_err(|_| FrozenLitterScientificOwnerRestartError::Lse)?;
        let expected_lse_configuration = serde_json::to_vec(context.lse_configuration)
            .map_err(|_| FrozenLitterScientificOwnerRestartError::Canonical)?;
        let replay_lse_configuration: LandSurfaceEnergyConfiguration =
            serde_json::from_slice(&self.lse_configuration_json)
                .map_err(|_| FrozenLitterScientificOwnerRestartError::Lse)?;
        replay_lse_configuration
            .validate_v3()
            .map_err(|_| FrozenLitterScientificOwnerRestartError::Lse)?;
        if self.lse_configuration_json != expected_lse_configuration
            || replay_lse_configuration != *context.lse_configuration
        {
            return Err(FrozenLitterScientificOwnerRestartError::Identity);
        }
        let lse_v3 =
            LandSurfaceEnergyV3State::from_json(&self.lse_v3_state_json, context.lse_configuration)
                .map_err(|_| FrozenLitterScientificOwnerRestartError::Lse)?;
        if lse_v3
            .to_json()
            .map_err(|_| FrozenLitterScientificOwnerRestartError::Lse)?
            != self.lse_v3_state_json
        {
            return Err(FrozenLitterScientificOwnerRestartError::Canonical);
        }
        let expected_surface_configuration = context
            .surface_liquid_configuration
            .canonical_bytes()
            .map_err(|_| FrozenLitterScientificOwnerRestartError::SurfaceLiquid)?;
        if self.surface_liquid_v2_configuration_bytes != expected_surface_configuration {
            return Err(FrozenLitterScientificOwnerRestartError::Identity);
        }
        let surface_liquid_v2 = SurfaceLiquidOwnerEnvelopeV2::from_canonical_bytes(
            context.surface_liquid_configuration.parent(),
            Some(context.surface_liquid_configuration),
            &self.surface_liquid_v2_envelope_bytes,
        )
        .map_err(|_| FrozenLitterScientificOwnerRestartError::SurfaceLiquid)?;
        if surface_liquid_v2.v2_state().is_none() {
            return Err(FrozenLitterScientificOwnerRestartError::SurfaceLiquid);
        }
        let lse_keys = context
            .lse_configuration
            .ofes
            .iter()
            .flat_map(|ofe| {
                ofe.tiles
                    .iter()
                    .map(move |tile| (ofe.ofe_id.clone(), tile.tile_id.clone()))
            })
            .collect::<Vec<_>>();
        let surface_keys = context
            .surface_liquid_configuration
            .records()
            .iter()
            .map(|record| (record.key.ofe_id.clone(), record.key.tile_id.clone()))
            .collect::<Vec<_>>();
        if lse_keys != surface_keys
            || lse_keys.len() != context.surface_liquid_configuration.records().len()
        {
            return Err(FrozenLitterScientificOwnerRestartError::Identity);
        }
        let surface_state = surface_liquid_v2
            .v2_state()
            .ok_or(FrozenLitterScientificOwnerRestartError::SurfaceLiquid)?;
        if lse_v3.0.tiles.len() != surface_state.records().len()
            || lse_v3
                .0
                .tiles
                .iter()
                .zip(surface_state.records())
                .any(|(lse, surface)| {
                    lse.ofe_id != surface.key.ofe_id
                        || lse.tile_id != surface.key.tile_id
                        || lse.surface_enthalpy_j_m2_tile_ground.to_bits()
                            != surface.surface_enthalpy_j_m2_tile.to_bits()
                        || surface.last_accepted_transaction_id
                            != Some(self.publication_authority.transaction_id)
                })
        {
            return Err(FrozenLitterScientificOwnerRestartError::Identity);
        }
        let native = self
            .soil_thermal_v2
            .validate_with_configuration(
                context.soil_thermal_owner_id,
                context.lse_configuration,
                context.soil_thermal_seal_authority,
            )
            .map_err(|_| FrozenLitterScientificOwnerRestartError::SoilThermal)?;
        let SoilThermalNativeBundleV2 {
            owner_envelope,
            restart_seal,
            ..
        } = self
            .soil_thermal_v2
            .decode_native()
            .map_err(|_| FrozenLitterScientificOwnerRestartError::SoilThermal)?;
        if owner_envelope != native {
            return Err(FrozenLitterScientificOwnerRestartError::SoilThermal);
        }
        let lse_ofes = context
            .lse_configuration
            .ofes
            .iter()
            .map(|ofe| &ofe.ofe_id)
            .collect::<Vec<_>>();
        let soil_ofes = owner_envelope
            .state
            .ofes
            .iter()
            .map(|ofe| &ofe.ofe_id)
            .collect::<Vec<_>>();
        if lse_ofes != soil_ofes {
            return Err(FrozenLitterScientificOwnerRestartError::Identity);
        }
        let projection = context.projection_seal_authority.validate_projection(
            context.surface_liquid_configuration,
            &self.complete_owner_projection_v3_bytes,
            &self.publication_authority,
        )?;
        let owner_bytes = serde_json::to_vec(&owner_envelope)
            .map_err(|_| FrozenLitterScientificOwnerRestartError::Canonical)?;
        let restart_bytes = serde_json::to_vec(&restart_seal)
            .map_err(|_| FrozenLitterScientificOwnerRestartError::Canonical)?;
        if projection.ending_surface_owner_bytes != self.surface_liquid_v2_envelope_bytes
            || projection.soil_thermal_owner_envelope_bytes != owner_bytes
            || projection.soil_thermal_restart_identity_bytes != restart_bytes
            || self.wb14_v2_parent_working_state_bytes != projection.wb14_parent_working_state_bytes
            || self.publication_authority.run_id
                != context.surface_liquid_configuration.parent().run_id
            || lse_v3.0.last_accepted_transaction_id
                != Some(self.publication_authority.transaction_id)
            || owner_envelope.transaction_id != self.publication_authority.transaction_id
            || owner_envelope.support_start_ns != self.publication_authority.support_start_ns
            || owner_envelope.support_end_ns != self.publication_authority.support_end_ns
        {
            return Err(FrozenLitterScientificOwnerRestartError::Identity);
        }
        Ok(RestoredFrozenLitterScientificOwnerV3 {
            persisted: self.clone(),
            lse_v3,
            surface_liquid_v2,
            wb14_v2_parent_working_state_bytes: self.wb14_v2_parent_working_state_bytes.clone(),
            soil_thermal_v2: owner_envelope,
            soil_thermal_restart_v2: restart_seal,
        })
    }
}

#[allow(clippy::too_many_arguments)]
pub fn project_frozen_litter_scientific_owner_v3(
    lse_configuration: &LandSurfaceEnergyConfiguration,
    lse_v3: &LandSurfaceEnergyV3State,
    surface_configuration: &SurfaceLiquidConfigurationV2,
    surface_owner: &SurfaceLiquidOwnerEnvelopeV2,
    wb14_v2_parent_working_state_bytes: Vec<u8>,
    soil_thermal_v2: SoilThermalOwnerStateRestartV2,
    complete_owner_projection_v3_bytes: Vec<u8>,
    publication_authority: FrozenLitterPublicationAuthorityV3,
    context: &FrozenLitterExpectedScientificContextV3<'_>,
) -> Result<FrozenLitterScientificOwnerStateV3, FrozenLitterScientificOwnerRestartError> {
    FrozenLitterScientificOwnerStateV3::new(
        lse_configuration,
        lse_v3,
        surface_configuration,
        surface_owner,
        wb14_v2_parent_working_state_bytes,
        soil_thermal_v2,
        complete_owner_projection_v3_bytes,
        publication_authority,
        context,
    )
}

fn wire_digest(value: &str) -> Result<Sha256Hex, FrozenLitterScientificOwnerRestartError> {
    Sha256Hex::try_new(value.to_owned())
        .map_err(|_| FrozenLitterScientificOwnerRestartError::Canonical)
}

fn zero_digest() -> Result<Sha256Hex, FrozenLitterScientificOwnerRestartError> {
    wire_digest(&"0".repeat(64))
}
