//! Sealed, uncommitted V8 vegetation-owner candidate construction.
//!
//! The dependency-neutral receipt types in this module are populated only
//! after the LSE owner has validated its sealed fixed-authorization payload.
//! Component identity is joined to vegetation occupancy identity through an
//! explicit typed bijection; no composite string is parsed here.

use std::collections::{BTreeMap, BTreeSet};

use openwepp_kernel_contract::{OccupancyId, SoilLayerId, TileId, TransactionId};
use serde::{Deserialize, Serialize};

use crate::carbon_nitrogen::MaterialTransfer;
use crate::diagnostics::CoupledSolvePass;
use crate::v8_persistent::{
    UncommittedV8PersistentPhase, V8OccupancyCarbonOperands, V8OccupancyCarbonReceipt,
    ValidatedV8CarbonPass,
};
use crate::v8_state::{
    V8_MODEL_SHA256, V8CoupledOwnedState, V8OccupancyState, V8TileCanopyAirState,
};
use crate::vegetation_candidate::{
    VegetationLedgerStateView, bind_material_proposals_from_strata,
    construct_ending_strata_from_preallocations, construct_ledgers_from_preallocations,
};
use crate::vegetation_ledger::{
    VegetationCarbonLedger, VegetationDryMaterialLedger, VegetationNitrogenLedger,
    validate_vegetation_ledgers,
};
use crate::{VegetationConfiguration, VegetationError};

#[derive(Clone, Debug, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
#[serde(transparent)]
pub struct V8LseComponentId(String);

impl V8LseComponentId {
    pub fn try_new(value: impl Into<String>) -> Result<Self, VegetationError> {
        let value = value.into();
        if value.is_empty() || value.trim() != value {
            return Err(VegetationError::Receipt(
                "empty or noncanonical LSE component identity".into(),
            ));
        }
        Ok(Self(value))
    }

    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct V8ComponentOccupancyBinding {
    pub component_id: V8LseComponentId,
    pub occupancy_id: OccupancyId,
}

#[derive(Clone, Debug, PartialEq)]
pub struct V8FinalRootWaterReceipt {
    pub layer_id: SoilLayerId,
    pub request_kg_m2_stand_ground: f64,
    pub authorization_kg_m2_stand_ground: f64,
    pub finalized_use_kg_m2_stand_ground: f64,
}

#[derive(Clone, Debug, PartialEq)]
pub struct V8FinalOccupancyReceipt {
    pub component_id: V8LseComponentId,
    pub beginning_canopy_liquid_kg_m2_tile_ground: f64,
    pub ending_canopy_liquid_kg_m2_tile_ground: f64,
    pub dry_stem_temperature_k: f64,
    pub root_node_potential_mm: f64,
    pub shade_ci_pa: f64,
    pub shade_leaf_potential_mm: f64,
    pub shade_leaf_temperature_k: f64,
    pub stem_potential_mm: f64,
    pub sun_ci_pa: f64,
    pub sun_leaf_potential_mm: f64,
    pub sun_leaf_temperature_k: f64,
    pub wet_surface_temperature_k: f64,
    pub beta_hyd: f64,
    pub carbon: V8OccupancyCarbonOperands,
    pub root_water: Vec<V8FinalRootWaterReceipt>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct V8FinalTileReceipt {
    pub pass: V8PhysicalReceiptPass,
    pub transaction_id: TransactionId,
    pub vegetation_model_definition_sha256: String,
    pub vegetation_configuration_sha256: String,
    pub vegetation_beginning_state_sha256: String,
    pub lse_configuration_sha256: String,
    pub lse_beginning_state_sha256: String,
    pub tile_id: TileId,
    pub tile_fraction: f64,
    pub interval_s: f64,
    pub canopy_air_temperature_k: f64,
    pub canopy_air_specific_humidity_kg_kg: f64,
    pub occupancies: Vec<V8FinalOccupancyReceipt>,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum V8PhysicalReceiptPass {
    FixedAuthorizationFinal,
}

#[derive(Clone, Debug, PartialEq)]
pub struct ValidatedV8FinalStatePass {
    transaction_id: TransactionId,
    vegetation_configuration_sha256: String,
    vegetation_beginning_state_sha256: String,
    bindings: BTreeMap<V8LseComponentId, OccupancyId>,
    tiles: Vec<V8FinalTileReceipt>,
}

impl ValidatedV8FinalStatePass {
    pub fn try_new(
        bindings: Vec<V8ComponentOccupancyBinding>,
        tiles: Vec<V8FinalTileReceipt>,
        configuration: &VegetationConfiguration,
        beginning: &V8CoupledOwnedState,
    ) -> Result<Self, VegetationError> {
        configuration.validate_v8()?;
        beginning.validate(configuration).map_err(|error| {
            VegetationError::Receipt(format!("invalid V8 beginning state: {error}"))
        })?;
        let expected_transaction = TransactionId(
            beginning
                .last_transaction_id
                .checked_add(1)
                .ok_or_else(|| VegetationError::Receipt("V8 transaction overflow".into()))?,
        );
        let expected_occupancies = configuration.expected_occupancies();
        let mut binding_map = BTreeMap::new();
        let mut bound_occupancies = BTreeSet::new();
        for binding in bindings {
            if binding_map
                .insert(binding.component_id, binding.occupancy_id.clone())
                .is_some()
                || !bound_occupancies.insert(binding.occupancy_id)
            {
                return Err(VegetationError::Receipt(
                    "V8 component/occupancy mapping is not bijective".into(),
                ));
            }
        }
        if bound_occupancies != expected_occupancies {
            return Err(VegetationError::Receipt(
                "V8 component/occupancy mapping is incomplete".into(),
            ));
        }
        if tiles
            .windows(2)
            .any(|pair| pair[0].tile_id >= pair[1].tile_id)
        {
            return Err(VegetationError::Receipt(
                "V8 final tile receipts are not canonically ordered".into(),
            ));
        }
        let expected_tiles = expected_occupancies
            .iter()
            .map(|occupancy| occupancy.tile_id.clone())
            .collect::<BTreeSet<_>>();
        let fractions = configuration
            .topology_tiles
            .iter()
            .map(|tile| (&tile.tile_id, tile.fraction))
            .collect::<BTreeMap<_, _>>();
        let mut actual_tiles = BTreeSet::new();
        let mut actual_occupancies = BTreeSet::new();
        let mut common_lse_lineage: Option<(&str, &str)> = None;
        for tile in &tiles {
            require_sha256(&tile.lse_configuration_sha256)?;
            require_sha256(&tile.lse_beginning_state_sha256)?;
            if tile.pass != V8PhysicalReceiptPass::FixedAuthorizationFinal
                || tile.transaction_id != expected_transaction
                || tile.vegetation_model_definition_sha256 != V8_MODEL_SHA256
                || tile.vegetation_configuration_sha256 != configuration.configuration_sha256
                || tile.vegetation_beginning_state_sha256 != beginning.state_sha256
                || tile.interval_s.to_bits() != configuration.dt_s.to_bits()
                || fractions.get(&tile.tile_id).map(|value| value.to_bits())
                    != Some(tile.tile_fraction.to_bits())
                || !actual_tiles.insert(tile.tile_id.clone())
            {
                return Err(VegetationError::Receipt(
                    "V8 final tile receipt lineage".into(),
                ));
            }
            match common_lse_lineage {
                None => {
                    common_lse_lineage = Some((
                        &tile.lse_configuration_sha256,
                        &tile.lse_beginning_state_sha256,
                    ));
                }
                Some((configuration_sha256, state_sha256))
                    if configuration_sha256 == tile.lse_configuration_sha256
                        && state_sha256 == tile.lse_beginning_state_sha256 => {}
                Some(_) => {
                    return Err(VegetationError::Receipt("mixed LSE receipt lineage".into()));
                }
            }
            validate_tile_receipt(
                tile,
                &binding_map,
                configuration,
                beginning,
                &mut actual_occupancies,
            )?;
        }
        if actual_tiles != expected_tiles || actual_occupancies != expected_occupancies {
            return Err(VegetationError::Receipt(
                "incomplete V8 final state receipt set".into(),
            ));
        }
        Ok(Self {
            transaction_id: expected_transaction,
            vegetation_configuration_sha256: configuration.configuration_sha256.clone(),
            vegetation_beginning_state_sha256: beginning.state_sha256.clone(),
            bindings: binding_map,
            tiles,
        })
    }

    fn carbon_receipts(&self) -> Vec<V8OccupancyCarbonReceipt> {
        let mut values = self
            .tiles
            .iter()
            .flat_map(|tile| {
                tile.occupancies
                    .iter()
                    .map(|receipt| V8OccupancyCarbonReceipt {
                        occupancy_id: self.bindings[&receipt.component_id].clone(),
                        tile_fraction: tile.tile_fraction,
                        operands: receipt.carbon,
                    })
            })
            .collect::<Vec<_>>();
        values.sort_by(|left, right| left.occupancy_id.cmp(&right.occupancy_id));
        values
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct UncommittedV8VegetationCandidate {
    transaction_id: TransactionId,
    beginning_state_sha256: String,
    ending_state: V8CoupledOwnedState,
    final_state_receipts: ValidatedV8FinalStatePass,
    persistent_phase: UncommittedV8PersistentPhase,
    material_proposals: Vec<MaterialTransfer>,
    carbon_ledgers: Vec<VegetationCarbonLedger>,
    nitrogen_ledgers: Vec<VegetationNitrogenLedger>,
    dry_material_ledgers: Vec<VegetationDryMaterialLedger>,
}

impl UncommittedV8VegetationCandidate {
    #[must_use]
    pub fn transaction_id(&self) -> TransactionId {
        self.transaction_id
    }

    #[must_use]
    pub fn beginning_state_sha256(&self) -> &str {
        &self.beginning_state_sha256
    }

    #[must_use]
    pub fn ending_state(&self) -> &V8CoupledOwnedState {
        &self.ending_state
    }

    #[must_use]
    pub fn material_proposals(&self) -> &[MaterialTransfer] {
        &self.material_proposals
    }

    /// Exact mineral-nitrogen request, maximum-authorization, and finalized-use
    /// protocol retained by the sealed persistent phase.
    #[must_use]
    pub fn nitrogen_protocol(
        &self,
    ) -> (
        &[crate::transaction::NitrogenRequest],
        &[crate::transaction::NitrogenAuthorization],
        &[crate::transaction::NitrogenUse],
    ) {
        (
            self.persistent_phase.requests(),
            self.persistent_phase.authorizations(),
            self.persistent_phase.finalized_uses(),
        )
    }

    pub fn validate_sealed(&self) -> Result<(), VegetationError> {
        if self.transaction_id != self.persistent_phase.transaction_id()
            || self.transaction_id != self.final_state_receipts.transaction_id
            || self.beginning_state_sha256
                != self.final_state_receipts.vegetation_beginning_state_sha256
            || self.ending_state.last_transaction_id != self.transaction_id.0
        {
            return Err(VegetationError::Receipt(
                "V8 vegetation candidate lineage".into(),
            ));
        }
        validate_candidate_ledgers(self)
    }
}

pub fn construct_uncommitted_v8_vegetation_candidate(
    configuration: &VegetationConfiguration,
    beginning: &V8CoupledOwnedState,
    potential: &ValidatedV8CarbonPass,
    capped: &ValidatedV8CarbonPass,
    final_state: &ValidatedV8FinalStatePass,
    persistent: &UncommittedV8PersistentPhase,
) -> Result<UncommittedV8VegetationCandidate, VegetationError> {
    configuration.validate_v8()?;
    beginning.validate(configuration).map_err(|error| {
        VegetationError::Receipt(format!("invalid V8 beginning state: {error}"))
    })?;
    if potential.pass() != CoupledSolvePass::Potential
        || capped.pass() != CoupledSolvePass::Capped
        || final_state.transaction_id != persistent.transaction_id()
        || final_state.vegetation_configuration_sha256 != configuration.configuration_sha256
        || final_state.vegetation_beginning_state_sha256 != beginning.state_sha256
        || !persistent.matches_sources(
            potential,
            capped,
            &configuration.configuration_sha256,
            &beginning.state_sha256,
        )
        || final_state.carbon_receipts() != capped.occupancies()
    {
        return Err(VegetationError::Receipt(
            "V8 candidate source receipt mismatch".into(),
        ));
    }
    let transaction_id = persistent.transaction_id();
    let ending_strata = construct_ending_strata_from_preallocations(
        configuration,
        transaction_id,
        persistent.strata(),
    )?;
    let ending_state = construct_v8_ending_state(
        configuration,
        beginning,
        transaction_id,
        final_state,
        ending_strata,
    )?;
    let material_proposals =
        bind_material_proposals_from_strata(transaction_id, persistent.strata())?;
    let (carbon_ledgers, nitrogen_ledgers, dry_material_ledgers) =
        construct_ledgers_from_preallocations(
            configuration,
            VegetationLedgerStateView {
                strata: &beginning.strata,
                state_sha256: &beginning.state_sha256,
            },
            VegetationLedgerStateView {
                strata: &ending_state.strata,
                state_sha256: &ending_state.state_sha256,
            },
            transaction_id,
            persistent.strata(),
            &material_proposals,
        )?;
    let candidate = UncommittedV8VegetationCandidate {
        transaction_id,
        beginning_state_sha256: beginning.state_sha256.clone(),
        ending_state,
        final_state_receipts: final_state.clone(),
        persistent_phase: persistent.clone(),
        material_proposals,
        carbon_ledgers,
        nitrogen_ledgers,
        dry_material_ledgers,
    };
    candidate.validate_sealed()?;
    Ok(candidate)
}

fn construct_v8_ending_state(
    configuration: &VegetationConfiguration,
    beginning: &V8CoupledOwnedState,
    transaction_id: TransactionId,
    final_state: &ValidatedV8FinalStatePass,
    ending_strata: BTreeMap<openwepp_kernel_contract::StratumId, crate::StratumSharedState>,
) -> Result<V8CoupledOwnedState, VegetationError> {
    let mut ending_occupancies = BTreeMap::new();
    let mut ending_tile_air = BTreeMap::new();
    for tile in &final_state.tiles {
        if ending_tile_air
            .insert(
                tile.tile_id.clone(),
                V8TileCanopyAirState {
                    canopy_air_specific_humidity_kg_kg: tile.canopy_air_specific_humidity_kg_kg,
                    canopy_air_temperature_k: tile.canopy_air_temperature_k,
                },
            )
            .is_some()
        {
            return Err(VegetationError::Receipt("duplicate V8 tile air".into()));
        }
        for receipt in &tile.occupancies {
            let occupancy_id = final_state.bindings[&receipt.component_id].clone();
            let lane = V8OccupancyState {
                beta_hyd: receipt.beta_hyd,
                canopy_liquid_kg_h2o_m2_tile_ground: receipt.ending_canopy_liquid_kg_m2_tile_ground,
                dry_stem_temperature_k: receipt.dry_stem_temperature_k,
                last_accepted_transaction_id: Some(transaction_id.0),
                root_node_potential_mm: receipt.root_node_potential_mm,
                shade_ci_pa: receipt.shade_ci_pa,
                shade_leaf_potential_mm: receipt.shade_leaf_potential_mm,
                shade_leaf_temperature_k: receipt.shade_leaf_temperature_k,
                stem_potential_mm: receipt.stem_potential_mm,
                sun_ci_pa: receipt.sun_ci_pa,
                sun_leaf_potential_mm: receipt.sun_leaf_potential_mm,
                sun_leaf_temperature_k: receipt.sun_leaf_temperature_k,
                wet_surface_temperature_k: receipt.wet_surface_temperature_k,
            };
            if ending_occupancies.insert(occupancy_id, lane).is_some() {
                return Err(VegetationError::Receipt(
                    "duplicate V8 ending occupancy".into(),
                ));
            }
        }
    }
    let mut ending_state = V8CoupledOwnedState {
        configuration_sha256: beginning.configuration_sha256.clone(),
        last_transaction_id: transaction_id.0,
        model_definition_sha256: beginning.model_definition_sha256.clone(),
        occupancies: ending_occupancies,
        state_sha256: String::new(),
        strata: ending_strata,
        tile_canopy_air: ending_tile_air,
    };
    ending_state.state_sha256 = ending_state.canonical_sha256();
    ending_state
        .validate(configuration)
        .map_err(|error| VegetationError::Receipt(format!("invalid V8 ending state: {error}")))?;
    Ok(ending_state)
}

fn validate_tile_receipt(
    tile: &V8FinalTileReceipt,
    bindings: &BTreeMap<V8LseComponentId, OccupancyId>,
    configuration: &VegetationConfiguration,
    beginning: &V8CoupledOwnedState,
    actual: &mut BTreeSet<OccupancyId>,
) -> Result<(), VegetationError> {
    if !tile.canopy_air_temperature_k.is_finite()
        || tile.canopy_air_temperature_k <= 0.0
        || !tile.canopy_air_specific_humidity_kg_kg.is_finite()
        || tile.canopy_air_specific_humidity_kg_kg < 0.0
        || tile
            .occupancies
            .windows(2)
            .any(|pair| pair[0].component_id >= pair[1].component_id)
    {
        return Err(VegetationError::Receipt(
            "invalid V8 final tile state operands".into(),
        ));
    }
    for receipt in &tile.occupancies {
        let occupancy_id = bindings
            .get(&receipt.component_id)
            .ok_or_else(|| VegetationError::Receipt("unbound V8 LSE component identity".into()))?;
        if occupancy_id.tile_id != tile.tile_id || !actual.insert(occupancy_id.clone()) {
            return Err(VegetationError::Receipt(
                "V8 final occupancy topology".into(),
            ));
        }
        let beginning_lane = &beginning.occupancies[occupancy_id];
        if receipt.beginning_canopy_liquid_kg_m2_tile_ground.to_bits()
            != beginning_lane.canopy_liquid_kg_h2o_m2_tile_ground.to_bits()
        {
            return Err(VegetationError::Receipt(
                "V8 E04 beginning-store lineage".into(),
            ));
        }
        let stratum = configuration
            .strata
            .iter()
            .find(|row| row.stratum_id == occupancy_id.stratum_id)
            .ok_or_else(|| VegetationError::Receipt("V8 receipt stratum identity".into()))?;
        let expected_layers = stratum
            .root_layers
            .iter()
            .map(|root| root.layer_id.clone())
            .collect::<BTreeSet<_>>();
        let actual_layers = receipt
            .root_water
            .iter()
            .map(|root| root.layer_id.clone())
            .collect::<BTreeSet<_>>();
        if receipt.root_water.len() != expected_layers.len() || actual_layers != expected_layers {
            return Err(VegetationError::Receipt(
                "V8 final root-water layer set".into(),
            ));
        }
        for root in &receipt.root_water {
            if [
                root.request_kg_m2_stand_ground,
                root.authorization_kg_m2_stand_ground,
                root.finalized_use_kg_m2_stand_ground,
            ]
            .iter()
            .any(|value| !value.is_finite())
                || root.finalized_use_kg_m2_stand_ground < 0.0
                || root.finalized_use_kg_m2_stand_ground > root.authorization_kg_m2_stand_ground
                || root.authorization_kg_m2_stand_ground > root.request_kg_m2_stand_ground
            {
                return Err(VegetationError::Receipt("V8 final root-water D/A/F".into()));
            }
        }
        // Full state validation occurs after every final lane and shared tile
        // lane have been assembled.
    }
    Ok(())
}

fn require_sha256(value: &str) -> Result<(), VegetationError> {
    if value.len() == 64
        && value
            .bytes()
            .all(|byte| byte.is_ascii_digit() || (b'a'..=b'f').contains(&byte))
    {
        Ok(())
    } else {
        Err(VegetationError::Receipt(
            "invalid LSE receipt digest".into(),
        ))
    }
}

fn validate_candidate_ledgers(
    candidate: &UncommittedV8VegetationCandidate,
) -> Result<(), VegetationError> {
    let expected = candidate
        .ending_state
        .strata
        .keys()
        .cloned()
        .collect::<BTreeSet<_>>();
    validate_vegetation_ledgers(
        &expected,
        candidate.transaction_id,
        &candidate.beginning_state_sha256,
        &candidate.ending_state.state_sha256,
        &candidate.carbon_ledgers,
        &candidate.nitrogen_ledgers,
        &candidate.dry_material_ledgers,
    )
}

#[cfg(test)]
mod tests {
    use std::cell::Cell;

    use openwepp_kernel_contract::MineralNitrogenKey;

    use super::*;
    use crate::transaction::{NitrogenArbiter, NitrogenAuthorization, NitrogenRequest};
    use crate::v8_persistent::{
        V8PersistentForcingReceipt, execute_uncommitted_v8_persistent_phase,
    };

    struct FullNitrogen {
        calls: Cell<u32>,
    }

    impl NitrogenArbiter for FullNitrogen {
        fn beginning_amount(&self, _: &MineralNitrogenKey) -> Result<f64, VegetationError> {
            Ok(1.0)
        }

        fn authorize(
            &self,
            requests: &[NitrogenRequest],
        ) -> Result<Vec<NitrogenAuthorization>, VegetationError> {
            self.calls.set(self.calls.get() + 1);
            Ok(requests
                .iter()
                .map(|request| NitrogenAuthorization {
                    transaction_id: request.transaction_id,
                    owner_id: request.owner_id.clone(),
                    key: request.key.clone(),
                    amount: request.amount,
                    basis: request.basis,
                })
                .collect())
        }
    }

    fn carbon(gross: f64) -> V8OccupancyCarbonOperands {
        V8OccupancyCarbonOperands {
            sun_leaf_area_m2_m2_tile_ground: 1.0,
            shade_leaf_area_m2_m2_tile_ground: 1.0,
            sun_gross_assimilation_umol_co2_m2_leaf_s: gross,
            shade_gross_assimilation_umol_co2_m2_leaf_s: gross / 2.0,
            sun_dark_respiration_umol_co2_m2_leaf_s: 0.1,
            shade_dark_respiration_umol_co2_m2_leaf_s: 0.1,
        }
    }

    fn carbon_pass(
        configuration: &VegetationConfiguration,
        beginning: &V8CoupledOwnedState,
        pass: CoupledSolvePass,
        gross: f64,
    ) -> ValidatedV8CarbonPass {
        let fractions = configuration
            .topology_tiles
            .iter()
            .map(|tile| (&tile.tile_id, tile.fraction))
            .collect::<BTreeMap<_, _>>();
        let receipts = configuration
            .expected_occupancies()
            .into_iter()
            .map(|occupancy_id| V8OccupancyCarbonReceipt {
                tile_fraction: fractions[&occupancy_id.tile_id],
                occupancy_id,
                operands: carbon(gross),
            })
            .collect();
        ValidatedV8CarbonPass::try_new(
            V8_MODEL_SHA256.into(),
            configuration.configuration_sha256.clone(),
            TransactionId(beginning.last_transaction_id + 1),
            beginning.state_sha256.clone(),
            pass,
            configuration.dt_s,
            receipts,
            configuration,
            beginning,
        )
        .expect("carbon pass")
    }

    #[allow(clippy::too_many_lines)]
    fn setup() -> (
        VegetationConfiguration,
        V8CoupledOwnedState,
        ValidatedV8CarbonPass,
        ValidatedV8CarbonPass,
        UncommittedV8PersistentPhase,
        Vec<V8ComponentOccupancyBinding>,
        Vec<V8FinalTileReceipt>,
    ) {
        let (configuration, beginning) = crate::v8_state::v8_test_fixture();
        let potential = carbon_pass(
            &configuration,
            &beginning,
            CoupledSolvePass::Potential,
            12.0,
        );
        let capped = carbon_pass(&configuration, &beginning, CoupledSolvePass::Capped, 9.0);
        let forcing = V8PersistentForcingReceipt {
            model_definition_sha256: V8_MODEL_SHA256.into(),
            configuration_sha256: configuration.configuration_sha256.clone(),
            transaction_id: TransactionId(beginning.last_transaction_id + 1),
            vegetation_beginning_state_sha256: beginning.state_sha256.clone(),
            air_temperature_k: 295.0,
            gsi: 1.0,
            soil_temperature_k_by_layer: configuration
                .strata
                .iter()
                .flat_map(|stratum| &stratum.root_layers)
                .map(|root| (root.layer_id.clone(), 293.0))
                .collect(),
        };
        let nitrogen = FullNitrogen {
            calls: Cell::new(0),
        };
        let persistent = execute_uncommitted_v8_persistent_phase(
            &configuration,
            &beginning,
            &potential,
            &capped,
            &forcing,
            &nitrogen,
        )
        .expect("persistent phase");
        assert_eq!(nitrogen.calls.get(), 1);

        let mut bindings = Vec::new();
        let mut by_tile = BTreeMap::<TileId, Vec<V8FinalOccupancyReceipt>>::new();
        for (index, occupancy_id) in configuration.expected_occupancies().into_iter().enumerate() {
            let component_id =
                V8LseComponentId::try_new(format!("component-{index}")).expect("component");
            bindings.push(V8ComponentOccupancyBinding {
                component_id: component_id.clone(),
                occupancy_id: occupancy_id.clone(),
            });
            let lane = &beginning.occupancies[&occupancy_id];
            let stratum = configuration
                .strata
                .iter()
                .find(|row| row.stratum_id == occupancy_id.stratum_id)
                .expect("stratum");
            by_tile
                .entry(occupancy_id.tile_id)
                .or_default()
                .push(V8FinalOccupancyReceipt {
                    component_id,
                    beginning_canopy_liquid_kg_m2_tile_ground: lane
                        .canopy_liquid_kg_h2o_m2_tile_ground,
                    ending_canopy_liquid_kg_m2_tile_ground: lane
                        .canopy_liquid_kg_h2o_m2_tile_ground,
                    dry_stem_temperature_k: lane.dry_stem_temperature_k,
                    root_node_potential_mm: lane.root_node_potential_mm,
                    shade_ci_pa: lane.shade_ci_pa,
                    shade_leaf_potential_mm: lane.shade_leaf_potential_mm,
                    shade_leaf_temperature_k: lane.shade_leaf_temperature_k,
                    stem_potential_mm: lane.stem_potential_mm,
                    sun_ci_pa: lane.sun_ci_pa,
                    sun_leaf_potential_mm: lane.sun_leaf_potential_mm,
                    sun_leaf_temperature_k: lane.sun_leaf_temperature_k + 0.5,
                    wet_surface_temperature_k: lane.wet_surface_temperature_k,
                    beta_hyd: lane.beta_hyd,
                    carbon: carbon(9.0),
                    root_water: stratum
                        .root_layers
                        .iter()
                        .map(|root| V8FinalRootWaterReceipt {
                            layer_id: root.layer_id.clone(),
                            request_kg_m2_stand_ground: 0.0,
                            authorization_kg_m2_stand_ground: 0.0,
                            finalized_use_kg_m2_stand_ground: 0.0,
                        })
                        .collect(),
                });
        }
        bindings.sort();
        let fractions = configuration
            .topology_tiles
            .iter()
            .map(|tile| (&tile.tile_id, tile.fraction))
            .collect::<BTreeMap<_, _>>();
        let mut tiles = by_tile
            .into_iter()
            .map(|(tile_id, mut occupancies)| {
                occupancies.sort_by(|left, right| left.component_id.cmp(&right.component_id));
                let air = &beginning.tile_canopy_air[&tile_id];
                V8FinalTileReceipt {
                    pass: V8PhysicalReceiptPass::FixedAuthorizationFinal,
                    transaction_id: TransactionId(beginning.last_transaction_id + 1),
                    vegetation_model_definition_sha256: V8_MODEL_SHA256.into(),
                    vegetation_configuration_sha256: configuration.configuration_sha256.clone(),
                    vegetation_beginning_state_sha256: beginning.state_sha256.clone(),
                    lse_configuration_sha256: "a".repeat(64),
                    lse_beginning_state_sha256: "b".repeat(64),
                    tile_fraction: fractions[&tile_id],
                    tile_id,
                    interval_s: configuration.dt_s,
                    canopy_air_temperature_k: air.canopy_air_temperature_k + 0.25,
                    canopy_air_specific_humidity_kg_kg: air.canopy_air_specific_humidity_kg_kg,
                    occupancies,
                }
            })
            .collect::<Vec<_>>();
        tiles.sort_by(|left, right| left.tile_id.cmp(&right.tile_id));
        (
            configuration,
            beginning,
            potential,
            capped,
            persistent,
            bindings,
            tiles,
        )
    }

    #[test]
    fn fixed_final_receipts_construct_one_uncommitted_v8_owner_candidate() {
        let (configuration, beginning, potential, capped, persistent, bindings, tiles) = setup();
        let before = serde_json::to_vec(&beginning).expect("beginning bytes");
        let receipts =
            ValidatedV8FinalStatePass::try_new(bindings, tiles, &configuration, &beginning)
                .expect("final receipts");
        let candidate = construct_uncommitted_v8_vegetation_candidate(
            &configuration,
            &beginning,
            &potential,
            &capped,
            &receipts,
            &persistent,
        )
        .expect("candidate");
        candidate.validate_sealed().expect("sealed candidate");
        assert_eq!(candidate.transaction_id(), TransactionId(1));
        assert_ne!(
            candidate.ending_state().state_sha256,
            beginning.state_sha256
        );
        assert_eq!(
            candidate
                .ending_state()
                .occupancies
                .values()
                .next()
                .expect("lane")
                .sun_leaf_temperature_k
                .to_bits(),
            (beginning
                .occupancies
                .values()
                .next()
                .expect("beginning lane")
                .sun_leaf_temperature_k
                + 0.5)
                .to_bits()
        );
        assert_eq!(serde_json::to_vec(&beginning).expect("after bytes"), before);
    }

    #[test]
    fn component_mapping_must_be_an_exact_bijection() {
        let (configuration, beginning, _, _, _, mut bindings, tiles) = setup();
        bindings.push(bindings[0].clone());
        assert!(matches!(
            ValidatedV8FinalStatePass::try_new(bindings, tiles, &configuration, &beginning),
            Err(VegetationError::Receipt(_))
        ));
    }

    #[test]
    fn mismatched_capped_payload_and_invalid_final_receipt_roll_back() {
        let (configuration, beginning, potential, capped, persistent, bindings, mut tiles) =
            setup();
        let before = serde_json::to_vec(&beginning).expect("beginning bytes");
        tiles[0].occupancies[0]
            .carbon
            .sun_gross_assimilation_umol_co2_m2_leaf_s += 1.0;
        let receipts =
            ValidatedV8FinalStatePass::try_new(bindings, tiles, &configuration, &beginning)
                .expect("structurally valid final receipt");
        assert!(matches!(
            construct_uncommitted_v8_vegetation_candidate(
                &configuration,
                &beginning,
                &potential,
                &capped,
                &receipts,
                &persistent,
            ),
            Err(VegetationError::Receipt(_))
        ));
        assert_eq!(serde_json::to_vec(&beginning).expect("after bytes"), before);
    }
}
