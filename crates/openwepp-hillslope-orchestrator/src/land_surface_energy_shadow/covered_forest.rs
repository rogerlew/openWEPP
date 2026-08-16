//! Covered forest/litter potential/final orchestration over the real owners.

use std::collections::{BTreeMap, BTreeSet};

use super::{
    CoveredColumnInputs, CoveredPotentialPhase, DirectSurfaceLiquidConfiguration,
    DirectSurfaceLiquidIngressInput, FinalCoveredTileCandidate, GroundWaterKey,
    LandSurfaceEnergyRealHydrologyAdapter, LandSurfaceEnergyShadowError, OwnerKind,
    PotentialWaterRequestBatch, RealHydrologySourceKey, RootRuntimeIdentity, RuntimeTileIdentity,
    SoilThermalSnapshot, SoilThermalTileCandidate, TileState, UnifiedLseFinalization,
    UnifiedRealHydrologyCandidate, UnifiedReceiverExpectations, WaterAmount,
    execute_unified_real_hydrology_shadow, finalize_covered_phase, solve_covered_potential_phase,
};

/// Result of one covered forest/litter potential/final transaction against the
/// persistent production hydrology owners.
#[derive(Clone, Debug, PartialEq)]
pub struct CoveredForestShadowResult {
    potential: CoveredPotentialPhase,
    submitted_request_batch: PotentialWaterRequestBatch,
    final_tile: FinalCoveredTileCandidate,
    hydrology_candidate: UnifiedRealHydrologyCandidate,
}

impl CoveredForestShadowResult {
    #[must_use]
    pub const fn potential(&self) -> &CoveredPotentialPhase {
        &self.potential
    }

    #[must_use]
    pub const fn submitted_request_batch(&self) -> &PotentialWaterRequestBatch {
        &self.submitted_request_batch
    }

    #[must_use]
    pub const fn final_tile(&self) -> &FinalCoveredTileCandidate {
        &self.final_tile
    }

    #[must_use]
    pub const fn hydrology_candidate(&self) -> &UnifiedRealHydrologyCandidate {
        &self.hydrology_candidate
    }
}

/// Execute the admitted covered forest/litter water transaction against one
/// immutable production hydrology snapshot.
///
/// The covered solve emits root plus litter requests. Companion requests and
/// receivers are caller-produced results for the other configured tiles; this
/// function never synthesizes their physics. All requests enter one unified
/// authorization, then the covered solve is rebuilt from its immutable
/// beginning state under its fixed subset of those authorizations. The unified
/// owner applies signed condensation credit and timed ingress only after the
/// complete final protocol and receiver topology are sealed.
#[allow(clippy::too_many_arguments)]
pub fn execute_covered_forest_shadow(
    soil_adapter: &LandSurfaceEnergyRealHydrologyAdapter<'_>,
    surface_configuration: &DirectSurfaceLiquidConfiguration,
    receiver_expectations: &UnifiedReceiverExpectations,
    identity: RuntimeTileIdentity,
    beginning: &CoveredColumnInputs,
    roots: Vec<RootRuntimeIdentity>,
    soil_sources: &BTreeMap<GroundWaterKey, RealHydrologySourceKey>,
    ingress: &DirectSurfaceLiquidIngressInput,
    potential_initial_trial: Vec<f64>,
    final_initial_trial: Vec<f64>,
    soil_thermal: &SoilThermalSnapshot,
    companion_potential_requests: &[WaterAmount],
    companion_finalized_uses: &[WaterAmount],
    companion_ending_lse_tiles: &[TileState],
    companion_soil_thermal_candidates: &[SoilThermalTileCandidate],
) -> Result<CoveredForestShadowResult, LandSurfaceEnergyShadowError> {
    let potential =
        solve_covered_potential_phase(identity, beginning, roots, potential_initial_trial)?;
    let mut submitted_requests = potential.request_batch.requests.clone();
    submitted_requests.extend_from_slice(companion_potential_requests);
    let submitted_request_batch = PotentialWaterRequestBatch::try_new(
        potential.request_batch.transaction_id,
        potential.request_batch.beginning_lse_state_sha256.clone(),
        submitted_requests,
    )?;
    let mut retained_final = None;
    let hydrology_candidate = execute_unified_real_hydrology_shadow(
        soil_adapter,
        surface_configuration,
        receiver_expectations,
        &submitted_request_batch,
        soil_sources,
        ingress,
        |authorizations| {
            let covered_keys: BTreeSet<_> = potential
                .request_batch
                .requests
                .iter()
                .map(|row| row.key.clone())
                .collect();
            let covered_authorizations = authorizations
                .iter()
                .filter(|row| covered_keys.contains(&row.key))
                .cloned()
                .collect();
            let mut final_tile = finalize_covered_phase(
                &potential,
                &potential.identity.beginning_lse_state_sha256,
                covered_authorizations,
                final_initial_trial,
                soil_thermal,
            )?;
            final_tile
                .water_protocol
                .requests
                .clone_from(&submitted_request_batch.requests);
            final_tile.water_protocol.authorizations = authorizations.to_vec();
            final_tile
                .water_protocol
                .finalized_uses
                .extend_from_slice(companion_finalized_uses);
            final_tile.water_protocol.validate()?;

            let mut ending_lse_tiles = vec![final_tile.ending_tile_state_pre_ingress.clone()];
            ending_lse_tiles.extend_from_slice(companion_ending_lse_tiles);
            let mut soil_thermal_candidates = vec![final_tile.soil_thermal.clone()];
            soil_thermal_candidates.extend_from_slice(companion_soil_thermal_candidates);
            let sealed = UnifiedLseFinalization::try_new(
                receiver_expectations,
                final_tile.water_protocol.clone(),
                ending_lse_tiles,
                soil_thermal_candidates,
                final_tile
                    .rollback_hashes
                    .iter()
                    .filter(|row| {
                        matches!(
                            row.owner_kind,
                            OwnerKind::LandSurfaceEnergy
                                | OwnerKind::Hydrology
                                | OwnerKind::SoilThermal
                        )
                    })
                    .cloned()
                    .collect(),
            )?;
            retained_final = Some(final_tile);
            Ok(sealed)
        },
    )?;
    let final_tile = retained_final.ok_or(LandSurfaceEnergyShadowError::Identity(
        "covered fixed-cap finalizer did not return a candidate",
    ))?;
    Ok(CoveredForestShadowResult {
        potential,
        submitted_request_batch,
        final_tile,
        hydrology_candidate,
    })
}
