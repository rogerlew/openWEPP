//! Complete public composition of the covered V8 uncommitted transaction.

use std::collections::{BTreeMap, BTreeSet};

use openwepp_biogeochemistry::BiogeochemistryState;
use openwepp_kernel_contract::{OccupancyId, TileId};
use openwepp_vegetation::{
    NitrogenArbiter, V8_MODEL_SHA256, V8ComponentOccupancyBinding, V8CoupledOwnedState,
    V8LseComponentId, V8PersistentForcingReceipt, VegetationConfiguration, VegetationError,
};
use thiserror::Error;

use super::{
    CoveredColumnInputs, DirectSurfaceLiquidConfiguration, DirectSurfaceLiquidIngressInput,
    GroundWaterKey, LandSurfaceEnergyRealHydrologyAdapter, LandSurfaceEnergyShadowError,
    RealHydrologySourceKey, RootRuntimeIdentity, RuntimeTileIdentity, SoilThermalSnapshot,
    SoilThermalTileCandidate, TileState, UnifiedReceiverExpectations, WaterAmount,
    construct_covered_v8_owner_envelope, execute_covered_forest_shadow,
};
use super::{CoveredV8OwnerEnvelopeError, UncommittedCoveredV8OwnerEnvelope};

#[derive(Clone, Debug, Error, PartialEq)]
pub enum CoveredV8TransactionError {
    #[error("V8 beginning-trial identity failure: {0}")]
    Identity(&'static str),
    #[error(transparent)]
    Physical(#[from] LandSurfaceEnergyShadowError),
    #[error(transparent)]
    Vegetation(#[from] VegetationError),
    #[error(transparent)]
    Envelope(#[from] CoveredV8OwnerEnvelopeError),
}

/// Construct the exact joint-solver initial vector from persistent beginning
/// state. The caller cannot supply or override a numerical trial.
pub fn construct_v8_beginning_trial(
    column: &CoveredColumnInputs,
    tile_id: &TileId,
    ofe_id: &super::OfeId,
    vegetation_beginning: &V8CoupledOwnedState,
    bindings: &[V8ComponentOccupancyBinding],
    soil_thermal: &SoilThermalSnapshot,
) -> Result<Vec<f64>, CoveredV8TransactionError> {
    let mut binding_map = BTreeMap::<V8LseComponentId, OccupancyId>::new();
    let mut bound_occupancies = BTreeSet::new();
    for binding in bindings {
        if binding_map
            .insert(binding.component_id.clone(), binding.occupancy_id.clone())
            .is_some()
            || !bound_occupancies.insert(binding.occupancy_id.clone())
        {
            return Err(CoveredV8TransactionError::Identity(
                "component/occupancy binding is not bijective",
            ));
        }
    }
    let mut used = BTreeSet::new();
    let mut trial =
        Vec::with_capacity(10 * column.occupancies.len() + 3 + column.ground.soil_nodes.len());
    for occupancy in &column.occupancies {
        let component = V8LseComponentId::try_new(&occupancy.occupancy_id)?;
        let occupancy_id =
            binding_map
                .get(&component)
                .ok_or(CoveredV8TransactionError::Identity(
                    "unbound ordered covered component",
                ))?;
        let lane = vegetation_beginning.occupancies.get(occupancy_id).ok_or(
            CoveredV8TransactionError::Identity("missing V8 beginning occupancy lane"),
        )?;
        if occupancy_id.tile_id != *tile_id || !used.insert(occupancy_id.clone()) {
            return Err(CoveredV8TransactionError::Identity(
                "covered occupancy beginning-state identity",
            ));
        }
        trial.extend_from_slice(&occupancy_trial_block(
            occupancy.beginning_canopy_liquid_kg_m2_tile,
            lane,
        )?);
    }
    if used != bound_occupancies {
        return Err(CoveredV8TransactionError::Identity(
            "unused component/occupancy binding",
        ));
    }
    let canopy_air = vegetation_beginning.tile_canopy_air.get(tile_id).ok_or(
        CoveredV8TransactionError::Identity("missing shared tile canopy-air lane"),
    )?;
    trial.push(canopy_air.canopy_air_temperature_k);
    trial.push(canopy_air.canopy_air_specific_humidity_kg_kg);
    trial.push(column.ground.surface_temperature_warm_start_k);
    let thermal_ofe = soil_thermal
        .ofes
        .iter()
        .find(|value| &value.ofe_id == ofe_id)
        .ok_or(CoveredV8TransactionError::Identity(
            "missing soil-thermal OFE warm starts",
        ))?;
    if thermal_ofe.ordered_layers.len() != column.ground.soil_nodes.len() {
        return Err(CoveredV8TransactionError::Identity(
            "soil-thermal warm-start cardinality",
        ));
    }
    for (node, thermal) in column
        .ground
        .soil_nodes
        .iter()
        .zip(&thermal_ofe.ordered_layers)
    {
        if node.layer_id != thermal.layer_id.as_str()
            || node.beginning_temperature_k.to_bits() != thermal.temperature_k.to_bits()
        {
            return Err(CoveredV8TransactionError::Identity(
                "soil-thermal warm-start identity",
            ));
        }
        trial.push(thermal.temperature_k);
    }
    Ok(trial)
}

fn occupancy_trial_block(
    lse_beginning_canopy_liquid: f64,
    lane: &openwepp_vegetation::V8OccupancyState,
) -> Result<[f64; 10], CoveredV8TransactionError> {
    if lse_beginning_canopy_liquid.to_bits() != lane.canopy_liquid_kg_h2o_m2_tile_ground.to_bits() {
        return Err(CoveredV8TransactionError::Identity(
            "covered occupancy beginning liquid",
        ));
    }
    Ok([
        lane.sun_leaf_potential_mm,
        lane.shade_leaf_potential_mm,
        lane.stem_potential_mm,
        lane.root_node_potential_mm,
        lane.beta_hyd,
        lane.beta_hyd,
        lane.sun_leaf_temperature_k,
        lane.shade_leaf_temperature_k,
        lane.wet_surface_temperature_k,
        lane.dry_stem_temperature_k,
    ])
}

/// Execute physical potential/final passes from one beginning-derived trial,
/// then compose V8 persistence, BGC, and the heterogeneous owner envelope.
#[allow(clippy::too_many_arguments)]
pub fn execute_covered_v8_transaction(
    soil_adapter: &LandSurfaceEnergyRealHydrologyAdapter<'_>,
    surface_configuration: &DirectSurfaceLiquidConfiguration,
    receiver_expectations: &UnifiedReceiverExpectations,
    identity: RuntimeTileIdentity,
    beginning: &CoveredColumnInputs,
    roots: Vec<RootRuntimeIdentity>,
    soil_sources: &BTreeMap<GroundWaterKey, RealHydrologySourceKey>,
    ingress: &DirectSurfaceLiquidIngressInput,
    soil_thermal: &SoilThermalSnapshot,
    companion_potential_requests: &[WaterAmount],
    companion_finalized_uses: &[WaterAmount],
    companion_ending_lse_tiles: &[TileState],
    companion_soil_thermal_candidates: &[SoilThermalTileCandidate],
    vegetation_configuration: &VegetationConfiguration,
    vegetation_beginning: &V8CoupledOwnedState,
    bindings: Vec<V8ComponentOccupancyBinding>,
    persistent_forcing: &V8PersistentForcingReceipt,
    nitrogen: &dyn NitrogenArbiter,
    biogeochemistry_beginning: &BiogeochemistryState,
) -> Result<UncommittedCoveredV8OwnerEnvelope, CoveredV8TransactionError> {
    vegetation_configuration.validate_v8()?;
    vegetation_beginning
        .validate(vegetation_configuration)
        .map_err(|_| CoveredV8TransactionError::Identity("invalid V8 vegetation beginning"))?;
    let expected_transaction = vegetation_beginning
        .last_transaction_id
        .checked_add(1)
        .ok_or(CoveredV8TransactionError::Identity(
            "V8 transaction overflow",
        ))?;
    let configured_layers = vegetation_configuration
        .strata
        .iter()
        .flat_map(|stratum| &stratum.root_layers)
        .map(|root| root.layer_id.clone())
        .collect::<BTreeSet<_>>();
    if identity.transaction_id.0 != expected_transaction
        || persistent_forcing.model_definition_sha256 != V8_MODEL_SHA256
        || persistent_forcing.configuration_sha256 != vegetation_configuration.configuration_sha256
        || persistent_forcing.transaction_id != identity.transaction_id
        || persistent_forcing.vegetation_beginning_state_sha256 != vegetation_beginning.state_sha256
        || !persistent_forcing.air_temperature_k.is_finite()
        || persistent_forcing.air_temperature_k <= 0.0
        || !persistent_forcing.gsi.is_finite()
        || persistent_forcing
            .soil_temperature_k_by_layer
            .keys()
            .cloned()
            .collect::<BTreeSet<_>>()
            != configured_layers
        || persistent_forcing
            .soil_temperature_k_by_layer
            .values()
            .any(|value| !value.is_finite() || *value <= 0.0)
    {
        return Err(CoveredV8TransactionError::Identity(
            "persistent forcing preflight",
        ));
    }
    let trial = construct_v8_beginning_trial(
        beginning,
        &identity.tile_id,
        &identity.ofe_id,
        vegetation_beginning,
        &bindings,
        soil_thermal,
    )?;
    let physical = execute_covered_forest_shadow(
        soil_adapter,
        surface_configuration,
        receiver_expectations,
        identity,
        beginning,
        roots,
        soil_sources,
        ingress,
        trial.clone(),
        trial,
        soil_thermal,
        companion_potential_requests,
        companion_finalized_uses,
        companion_ending_lse_tiles,
        companion_soil_thermal_candidates,
    )?;
    Ok(construct_covered_v8_owner_envelope(
        physical,
        bindings,
        vegetation_configuration,
        vegetation_beginning,
        persistent_forcing,
        nitrogen,
        biogeochemistry_beginning,
    )?)
}

#[cfg(test)]
mod tests {
    use openwepp_vegetation::V8OccupancyState;

    use super::*;

    fn lane() -> V8OccupancyState {
        V8OccupancyState {
            beta_hyd: 0.73,
            canopy_liquid_kg_h2o_m2_tile_ground: 0.17,
            dry_stem_temperature_k: 294.8,
            last_accepted_transaction_id: Some(4),
            root_node_potential_mm: -5_100.0,
            shade_ci_pa: 27.1,
            shade_leaf_potential_mm: -7_100.0,
            shade_leaf_temperature_k: 295.0,
            stem_potential_mm: -6_600.0,
            sun_ci_pa: 25.9,
            sun_leaf_potential_mm: -7_400.0,
            sun_leaf_temperature_k: 296.2,
            wet_surface_temperature_k: 294.9,
        }
    }

    #[test]
    fn occupancy_trial_uses_exact_v8_lane_order_and_shared_beta_start() {
        let lane = lane();
        let actual = occupancy_trial_block(0.17, &lane).expect("exact beginning liquid");
        let expected: [f64; 10] = [
            -7_400.0, -7_100.0, -6_600.0, -5_100.0, 0.73, 0.73, 296.2, 295.0, 294.9, 294.8,
        ];
        assert!(
            actual
                .iter()
                .zip(expected)
                .all(|(left, right)| left.to_bits() == right.to_bits())
        );
    }

    #[test]
    fn lse_canopy_liquid_cannot_replace_v8_beginning_store() {
        assert_eq!(
            occupancy_trial_block(f64::from_bits(0.17_f64.to_bits() + 1), &lane()),
            Err(CoveredV8TransactionError::Identity(
                "covered occupancy beginning liquid"
            ))
        );
    }
}
