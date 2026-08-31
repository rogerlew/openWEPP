//! Authenticated frozen-litter V3 phase projection before owner sealing.

use std::collections::BTreeSet;
use std::time::Duration;

use openwepp_kernel_contract::{TileId, TransactionId};
use openwepp_land_surface_energy::{
    BeginningLitterPhaseState, EndingLitterPhaseState, LandSurfaceEnergyConfiguration,
    LandSurfaceEnergyError, LandSurfaceEnergyV3State, LitterPhaseConfiguration, LitterVaporReceipt,
    OfeId, PostVaporLitterState, SurfaceConfiguration, V3PhaseFreeCoveredEvaluation,
    V3PhaseFreeSurfaceEnergyLedger, apply_bounded_litter_phase,
};
use thiserror::Error;

use crate::direct_runtime::{
    DirectSurfaceLiquidError, SurfaceLiquidConfigurationV2, SurfaceLiquidOwnerClosureRecordV2,
    SurfaceLiquidOwnerEnvelopeV2, SurfaceLiquidStateRecordV2,
};

pub(crate) const FROZEN_LITTER_V3_SUPPORT_FLOOR_NS: u128 = 60_000_000_000;

#[derive(Debug, Error, PartialEq)]
pub(crate) enum FrozenLitterV3RuntimeError {
    #[error("frozen-litter V3 identity failure: {0}")]
    Identity(&'static str),
    #[error("frozen-litter V3 chronology failure: {0}")]
    Chronology(&'static str),
    #[error("frozen-litter V3 closure failure: {0}")]
    Closure(&'static str),
    #[error(transparent)]
    LandSurfaceEnergy(#[from] LandSurfaceEnergyError),
    #[error(transparent)]
    LseState(#[from] openwepp_land_surface_energy::LseV3StateError),
    #[error(transparent)]
    SurfaceLiquid(#[from] DirectSurfaceLiquidError),
    #[error(transparent)]
    SoilThermal(#[from] openwepp_land_surface_energy::SoilThermalExactCarryError),
    #[error("frozen-litter V3 canonical serialization failure: {0}")]
    Serialization(&'static str),
}

/// Accepted fixed-authorization phase-free evaluation for one configured
/// forest-litter row. It carries no phase transfer and no current ingress.
#[derive(Clone, Debug, PartialEq)]
pub(crate) struct FrozenLitterV3PhaseFreeInput {
    pub ofe_id: OfeId,
    pub tile_id: TileId,
    pub configuration: LitterPhaseConfiguration,
    pub beginning: BeginningLitterPhaseState,
    accepted_vapor: LitterVaporReceipt,
    accepted_post_vapor: PostVaporLitterState,
    accepted_surface_energy: V3PhaseFreeSurfaceEnergyLedger,
}

impl FrozenLitterV3PhaseFreeInput {
    #[allow(dead_code)]
    pub(crate) fn from_accepted_fixed_final(
        ofe_id: OfeId,
        tile_id: TileId,
        configuration: LitterPhaseConfiguration,
        beginning: BeginningLitterPhaseState,
        accepted_final: &V3PhaseFreeCoveredEvaluation,
    ) -> Self {
        Self {
            ofe_id,
            tile_id,
            configuration,
            beginning,
            accepted_vapor: accepted_final.vapor,
            accepted_post_vapor: accepted_final.post_vapor,
            accepted_surface_energy: accepted_final.surface_energy,
        }
    }

    #[cfg(test)]
    pub(crate) fn from_authority_operands_for_test(
        ofe_id: OfeId,
        tile_id: TileId,
        configuration: LitterPhaseConfiguration,
        beginning: BeginningLitterPhaseState,
        accepted_vapor: LitterVaporReceipt,
        accepted_post_vapor: PostVaporLitterState,
        accepted_surface_energy: V3PhaseFreeSurfaceEnergyLedger,
    ) -> Self {
        Self {
            ofe_id,
            tile_id,
            configuration,
            beginning,
            accepted_vapor,
            accepted_post_vapor,
            accepted_surface_energy,
        }
    }

    pub(crate) const fn accepted_vapor(&self) -> LitterVaporReceipt {
        self.accepted_vapor
    }

    pub(crate) const fn accepted_post_vapor(&self) -> PostVaporLitterState {
        self.accepted_post_vapor
    }

    pub(crate) const fn accepted_surface_energy(&self) -> V3PhaseFreeSurfaceEnergyLedger {
        self.accepted_surface_energy
    }
}

#[derive(Clone, Debug, PartialEq)]
pub(crate) struct ProjectedFrozenLitterV3Phase {
    pub phase_adjusted_owner: SurfaceLiquidOwnerEnvelopeV2,
    pub closure: Vec<SurfaceLiquidOwnerClosureRecordV2>,
    pub endings: Vec<EndingLitterPhaseState>,
}

pub(crate) fn checked_support_seconds(
    support_start_ns: u128,
    support_end_ns: u128,
) -> Result<f64, FrozenLitterV3RuntimeError> {
    let duration_ns = support_end_ns.checked_sub(support_start_ns).ok_or(
        FrozenLitterV3RuntimeError::Chronology("nonpositive half-open support"),
    )?;
    if duration_ns < FROZEN_LITTER_V3_SUPPORT_FLOOR_NS
        || duration_ns % FROZEN_LITTER_V3_SUPPORT_FLOOR_NS != 0
    {
        return Err(FrozenLitterV3RuntimeError::Chronology(
            "support is below or off the exact 60-second grid",
        ));
    }
    let duration_ns = u64::try_from(duration_ns)
        .map_err(|_| FrozenLitterV3RuntimeError::Chronology("support exceeds u64 nanoseconds"))?;
    Ok(Duration::from_nanos(duration_ns).as_secs_f64())
}

#[allow(clippy::too_many_arguments, clippy::too_many_lines)]
pub(crate) fn project_frozen_litter_v3_phase(
    surface_configuration: &SurfaceLiquidConfigurationV2,
    beginning_surface_owner: &SurfaceLiquidOwnerEnvelopeV2,
    lse_configuration: &LandSurfaceEnergyConfiguration,
    beginning_lse_state: &LandSurfaceEnergyV3State,
    transaction_id: TransactionId,
    support_start_ns: u128,
    support_end_ns: u128,
    phase_inputs: &[FrozenLitterV3PhaseFreeInput],
) -> Result<ProjectedFrozenLitterV3Phase, FrozenLitterV3RuntimeError> {
    let interval_s = checked_support_seconds(support_start_ns, support_end_ns)?;
    if transaction_id.0 == 0 {
        return Err(FrozenLitterV3RuntimeError::Identity(
            "zero transaction identity",
        ));
    }
    beginning_lse_state.validate(lse_configuration)?;
    beginning_surface_owner
        .canonical_bytes(surface_configuration.parent(), Some(surface_configuration))?;
    let beginning =
        beginning_surface_owner
            .v2_state()
            .ok_or(FrozenLitterV3RuntimeError::Identity(
                "surface beginning owner is not V2",
            ))?;
    let configured_litter = surface_configuration
        .records()
        .iter()
        .filter(|record| record.litter_depth_m.is_some())
        .collect::<Vec<_>>();
    if configured_litter.is_empty() || configured_litter.len() != phase_inputs.len() {
        return Err(FrozenLitterV3RuntimeError::Identity(
            "forest-litter phase input cardinality",
        ));
    }
    let mut seen = BTreeSet::new();
    let mut transfers = Vec::with_capacity(phase_inputs.len());
    let mut endings = Vec::with_capacity(phase_inputs.len());
    for ((configured, input), owner_record) in
        configured_litter
            .iter()
            .zip(phase_inputs)
            .zip(beginning.records().iter().filter(|record| {
                surface_configuration
                    .records()
                    .iter()
                    .find(|configured| configured.key == record.key)
                    .is_some_and(|configured| configured.litter_depth_m.is_some())
            }))
    {
        if configured.key != owner_record.key
            || configured.key.ofe_id != input.ofe_id
            || configured.key.tile_id != input.tile_id
            || !seen.insert((input.ofe_id.clone(), input.tile_id.clone()))
            || input.configuration.litter_depth_m.to_bits()
                != configured
                    .litter_depth_m
                    .ok_or(FrozenLitterV3RuntimeError::Identity(
                        "missing configured litter depth",
                    ))?
                    .to_bits()
            || input.configuration.ice_capacity_kg_m2_tile.to_bits()
                != configured
                    .litter_ice_capacity_kg_m2_tile
                    .ok_or(FrozenLitterV3RuntimeError::Identity(
                        "missing configured litter ice capacity",
                    ))?
                    .to_bits()
            || input.beginning.liquid_kg_m2_tile.to_bits()
                != owner_record.liquid_kg_m2_tile.to_bits()
            || input.beginning.ice_kg_m2_tile.to_bits()
                != owner_record.litter_ice_kg_m2_tile.to_bits()
            || input.beginning.sensible_energy_j_m2_tile.to_bits()
                != owner_record.surface_enthalpy_j_m2_tile.to_bits()
        {
            return Err(FrozenLitterV3RuntimeError::Identity(
                "phase-free input and beginning owner join",
            ));
        }
        let lse_tile = beginning_lse_state
            .0
            .tiles
            .iter()
            .find(|tile| tile.ofe_id == input.ofe_id && tile.tile_id == input.tile_id)
            .ok_or(FrozenLitterV3RuntimeError::Identity(
                "phase-free input LSE tile",
            ))?;
        let lse_configured = lse_configuration
            .ofes
            .iter()
            .find(|ofe| ofe.ofe_id == input.ofe_id)
            .and_then(|ofe| ofe.tiles.iter().find(|tile| tile.tile_id == input.tile_id))
            .ok_or(FrozenLitterV3RuntimeError::Identity(
                "phase-free input LSE configuration tile",
            ))?;
        let SurfaceConfiguration::ForestLitter {
            liquid_capacity_kg_m2_tile_ground,
            thickness_m,
            dry_density_kg_m3,
            dry_specific_heat_j_kg_k,
        } = lse_configured.surface
        else {
            return Err(FrozenLitterV3RuntimeError::Identity(
                "phase input is not an LSE forest-litter tile",
            ));
        };
        let dry_heat_capacity = thickness_m * dry_density_kg_m3 * dry_specific_heat_j_kg_k;
        if lse_tile.surface_enthalpy_j_m2_tile_ground.to_bits()
            != input.beginning.sensible_energy_j_m2_tile.to_bits()
            || lse_tile.surface_temperature_warm_start_k.to_bits()
                != input.beginning.temperature_k.to_bits()
            || input.configuration.liquid_capacity_kg_m2_tile.to_bits()
                != liquid_capacity_kg_m2_tile_ground.to_bits()
            || input.configuration.litter_depth_m.to_bits() != thickness_m.to_bits()
            || input.configuration.dry_heat_capacity_j_m2_k.to_bits() != dry_heat_capacity.to_bits()
            || input.accepted_post_vapor.liquid_kg_m2_tile < 0.0
            || input.accepted_post_vapor.ice_kg_m2_tile < 0.0
        {
            return Err(FrozenLitterV3RuntimeError::Identity(
                "phase-free input and LSE state join",
            ));
        }
        let (transfer, ending) =
            apply_bounded_litter_phase(input.configuration, input.accepted_post_vapor, interval_s)?;
        transfers.push(transfer);
        endings.push(ending);
    }

    let mut phase_index = 0_usize;
    let mut records = Vec::with_capacity(beginning.records().len());
    let mut closure = Vec::with_capacity(beginning.records().len());
    for record in beginning.records() {
        let is_litter = surface_configuration
            .records()
            .iter()
            .find(|configured| configured.key == record.key)
            .is_some_and(|configured| configured.litter_depth_m.is_some());
        if !is_litter {
            records.push(record.clone());
            closure.push(SurfaceLiquidOwnerClosureRecordV2 {
                key: record.key.clone(),
                liquid_debit_kg_m2_tile: 0.0,
                liquid_credit_kg_m2_tile: 0.0,
                ice_debit_kg_m2_tile: 0.0,
                ice_credit_kg_m2_tile: 0.0,
            });
            continue;
        }
        let input = phase_inputs
            .get(phase_index)
            .ok_or(FrozenLitterV3RuntimeError::Identity(
                "missing ordered litter phase input",
            ))?;
        let ending = endings
            .get(phase_index)
            .ok_or(FrozenLitterV3RuntimeError::Identity(
                "missing ordered litter phase ending",
            ))?;
        let vapor = input.accepted_vapor;
        let transfer = transfers
            .get(phase_index)
            .ok_or(FrozenLitterV3RuntimeError::Identity(
                "missing ordered litter phase transfer",
            ))?;
        let liquid_vapor_debit = vapor.liquid_signed_mass_kg_m2.max(0.0);
        let liquid_vapor_credit = (-vapor.liquid_signed_mass_kg_m2).max(0.0);
        let ice_vapor_debit = vapor.ice_signed_mass_kg_m2.max(0.0);
        let ice_vapor_credit = (-vapor.ice_signed_mass_kg_m2).max(0.0);
        records.push(SurfaceLiquidStateRecordV2 {
            key: record.key.clone(),
            liquid_kg_m2_tile: ending.liquid_kg_m2_tile,
            litter_ice_kg_m2_tile: ending.ice_kg_m2_tile,
            surface_enthalpy_j_m2_tile: ending.sensible_energy_j_m2_tile,
            last_accepted_transaction_id: record.last_accepted_transaction_id,
        });
        closure.push(SurfaceLiquidOwnerClosureRecordV2 {
            key: record.key.clone(),
            liquid_debit_kg_m2_tile: liquid_vapor_debit + transfer.freeze_kg_m2,
            liquid_credit_kg_m2_tile: liquid_vapor_credit + transfer.melt_kg_m2,
            ice_debit_kg_m2_tile: ice_vapor_debit + transfer.melt_kg_m2,
            ice_credit_kg_m2_tile: ice_vapor_credit + transfer.freeze_kg_m2,
        });
        phase_index += 1;
    }
    if phase_index != phase_inputs.len() {
        return Err(FrozenLitterV3RuntimeError::Identity(
            "unused litter phase input",
        ));
    }
    let phase_adjusted_owner = beginning_surface_owner.try_replace_v2_state(
        surface_configuration,
        records,
        beginning.continuations().to_vec(),
    )?;
    crate::validate_surface_liquid_owner_mass_closure_v2(
        surface_configuration,
        beginning,
        phase_adjusted_owner
            .v2_state()
            .ok_or(FrozenLitterV3RuntimeError::Identity(
                "phase-adjusted owner is not V2",
            ))?,
        &closure,
    )?;
    Ok(ProjectedFrozenLitterV3Phase {
        phase_adjusted_owner,
        closure,
        endings,
    })
}
