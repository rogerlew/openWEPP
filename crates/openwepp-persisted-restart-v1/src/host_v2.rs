//! Atomic isolated host for admitted V2 checkpoints.

use crate::{
    CompleteCommittedOwnerStateV2, ExpectedRestartStaticContextV2, IsolatedRestoredCheckpointV2,
    RestartAdmissionFailureV2, SoilThermalNativeSealAuthorityV2, admit_checkpoint_v2,
};
use openwepp_land_surface_energy::{
    LandSurfaceEnergyConfiguration, PreparedSoilThermalSupportV2, SoilThermalOwnerCheckpointV2,
    SoilThermalOwnerEnvelopeV2, SoilThermalOwnerRestartV2, SoilThermalReceiptFreeOwnerSealsV2,
    validate_soil_thermal_receipt_free_owner_v2,
};
use thiserror::Error;

/// Orchestrator-independent atomic host. Runtime installation is intentionally
/// a later integration step because the production shadow does not yet expose
/// a V2 soil-owner installation API.
#[derive(Clone, Debug, PartialEq)]
pub struct DirectV10RestartHostV2 {
    admitted: IsolatedRestoredCheckpointV2,
}

impl DirectV10RestartHostV2 {
    #[must_use]
    pub const fn from_isolated(admitted: IsolatedRestoredCheckpointV2) -> Self {
        Self { admitted }
    }

    #[must_use]
    pub const fn admitted(&self) -> &IsolatedRestoredCheckpointV2 {
        &self.admitted
    }
}

pub fn admit_and_install_checkpoint_v2(
    target: &mut DirectV10RestartHostV2,
    bytes: &[u8],
    context: &ExpectedRestartStaticContextV2<'_>,
) -> Result<(), RestartAdmissionFailureV2> {
    let admitted = admit_checkpoint_v2(bytes, context)?;
    *target = DirectV10RestartHostV2::from_isolated(admitted);
    Ok(())
}

#[derive(Debug, Error)]
pub enum NativeOwnerHostV2Error {
    #[error("native_receipt_free_seal")]
    NativeSeal,
    #[error("persisted_owner_join")]
    PersistedOwnerJoin,
    #[error("configuration_identity")]
    ConfigurationIdentity,
}

struct ReceiptFreeHostAuthority<'a> {
    prepared: &'a PreparedSoilThermalSupportV2,
    seals: &'a SoilThermalReceiptFreeOwnerSealsV2,
}

impl SoilThermalNativeSealAuthorityV2 for ReceiptFreeHostAuthority<'_> {
    fn validate_restart_seal(
        &self,
        envelope: &SoilThermalOwnerEnvelopeV2,
        seal: &SoilThermalOwnerRestartV2,
    ) -> Result<(), &'static str> {
        if envelope != self.prepared.beginning_owner() || seal != &self.seals.restart {
            return Err("native restart join");
        }
        validate_soil_thermal_receipt_free_owner_v2(self.prepared, self.seals)
            .map_err(|_| "native restart validation")
    }

    fn validate_checkpoint_seal(
        &self,
        envelope: &SoilThermalOwnerEnvelopeV2,
        seal: &SoilThermalOwnerCheckpointV2,
    ) -> Result<(), &'static str> {
        if envelope != self.prepared.beginning_owner() || seal != &self.seals.checkpoint {
            return Err("native checkpoint join");
        }
        validate_soil_thermal_receipt_free_owner_v2(self.prepared, self.seals)
            .map_err(|_| "native checkpoint validation")
    }
}

/// Atomic complete-owner host retaining the native exact-carry soil owner as
/// the sole runtime custody surface.
#[derive(Clone, Debug, PartialEq)]
pub struct DirectV10NativeOwnerHostV2 {
    committed: CompleteCommittedOwnerStateV2,
    soil_thermal: SoilThermalOwnerEnvelopeV2,
}

impl DirectV10NativeOwnerHostV2 {
    pub fn from_receipt_free_native(
        committed: CompleteCommittedOwnerStateV2,
        prepared: &PreparedSoilThermalSupportV2,
        seals: &SoilThermalReceiptFreeOwnerSealsV2,
        configuration: &LandSurfaceEnergyConfiguration,
    ) -> Result<Self, NativeOwnerHostV2Error> {
        validate_soil_thermal_receipt_free_owner_v2(prepared, seals)
            .map_err(|_| NativeOwnerHostV2Error::NativeSeal)?;
        let owner = prepared.beginning_owner();
        if owner.state.owner_id != configuration.soil_thermal_configuration.owner_id
            || owner.state.configuration_sha256
                != configuration
                    .soil_thermal_configuration
                    .configuration_sha256
        {
            return Err(NativeOwnerHostV2Error::ConfigurationIdentity);
        }
        let authority = ReceiptFreeHostAuthority { prepared, seals };
        let restored = committed
            .scientific
            .validate_soil_owner(&owner.state.owner_id, configuration, &authority)
            .map_err(|_| NativeOwnerHostV2Error::PersistedOwnerJoin)?;
        if restored != *owner {
            return Err(NativeOwnerHostV2Error::PersistedOwnerJoin);
        }
        Ok(Self {
            committed,
            soil_thermal: owner.clone(),
        })
    }

    #[must_use]
    pub const fn committed(&self) -> &CompleteCommittedOwnerStateV2 {
        &self.committed
    }

    #[must_use]
    pub const fn soil_thermal(&self) -> &SoilThermalOwnerEnvelopeV2 {
        &self.soil_thermal
    }
}

pub fn install_native_owner_host_v2(
    target: &mut DirectV10NativeOwnerHostV2,
    candidate: DirectV10NativeOwnerHostV2,
) {
    *target = candidate;
}
