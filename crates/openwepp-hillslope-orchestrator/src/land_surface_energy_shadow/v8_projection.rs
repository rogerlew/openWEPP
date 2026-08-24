//! Typed projection from an accepted covered LSE transaction into V8 receipts.
//!
//! This module performs no constitutive calculation, resource arbitration, or
//! owner mutation. It only validates and joins the sealed LSE operands, the
//! actual unified hydrology candidate, and caller-supplied typed component to
//! occupancy bindings.

#![allow(dead_code)]

use std::collections::{BTreeMap, BTreeSet};

use openwepp_kernel_contract::{OccupancyId, TransactionId};
use openwepp_land_surface_energy::{
    AcceptedCoveredOccupancyOperands, AcceptedCoveredVegetationOperands, ComponentId,
    CoveredVegetationOperandPass, GroundWaterKey, PotentialCoveredVegetationOperands,
    RequestingComponent, WaterAmount, WaterAuthorization,
};
use openwepp_vegetation::diagnostics::CoupledSolvePass;
use openwepp_vegetation::{
    V8ComponentOccupancyBinding, V8CoupledOwnedState, V8FinalOccupancyReceipt,
    V8FinalRootWaterReceipt, V8FinalTileReceipt, V8LseComponentId, V8OccupancyCarbonOperands,
    V8OccupancyCarbonReceipt, V8PhysicalReceiptPass, ValidatedV8CarbonPass,
    ValidatedV8FinalStatePass, VegetationConfiguration, VegetationError,
    validate_v8_component_bindings,
};
use thiserror::Error;

use super::{CoveredForestShadowResult, UnifiedRealHydrologyCandidate};

/// Failure while projecting already accepted physical operands into V8.
#[derive(Clone, Debug, Error, PartialEq)]
pub enum V8ProjectionError {
    #[error("V8 covered projection identity failure: {0}")]
    Identity(&'static str),
    #[error(transparent)]
    LandSurface(#[from] openwepp_land_surface_energy::LandSurfaceEnergyError),
    #[error(transparent)]
    Vegetation(#[from] VegetationError),
}

/// Complete dependency-neutral V8 receipts projected from one accepted
/// covered-forest transaction.
#[derive(Clone, Debug, PartialEq)]
pub(crate) struct V8CoveredProjection {
    potential: ValidatedV8CarbonPass,
    capped: ValidatedV8CarbonPass,
    final_state: ValidatedV8FinalStatePass,
}

impl V8CoveredProjection {
    #[must_use]
    pub(crate) const fn potential(&self) -> &ValidatedV8CarbonPass {
        &self.potential
    }

    #[must_use]
    pub(crate) const fn capped(&self) -> &ValidatedV8CarbonPass {
        &self.capped
    }

    #[must_use]
    pub(crate) const fn final_state(&self) -> &ValidatedV8FinalStatePass {
        &self.final_state
    }
}

/// Project one already executed covered-forest shadow result into the three
/// validated receipts required by the V8 vegetation owner.
///
/// Component IDs are never parsed. The caller supplies the exact typed
/// component-to-occupancy bijection, and this function verifies it against the
/// complete potential and fixed-final payloads.
pub(crate) fn project_covered_forest_v8_passes(
    physical: &CoveredForestShadowResult,
    bindings: &[V8ComponentOccupancyBinding],
    configuration: &VegetationConfiguration,
    beginning: &V8CoupledOwnedState,
) -> Result<V8CoveredProjection, V8ProjectionError> {
    configuration.validate_v8()?;
    beginning
        .validate(configuration)
        .map_err(|_| V8ProjectionError::Identity("invalid V8 vegetation beginning state"))?;

    let potential_source = &physical.potential().potential_vegetation_operands;
    let final_source = &physical.final_tile().vegetation_operands;
    potential_source.validate()?;
    final_source.validate()?;
    physical.final_tile().water_protocol.validate()?;
    validate_shared_lineage(
        physical,
        potential_source,
        final_source,
        configuration,
        beginning,
    )?;

    let binding_map = validate_v8_component_bindings(bindings, configuration)?;
    validate_component_sets(potential_source, final_source, &binding_map, beginning)?;
    validate_unified_root_protocol(final_source, physical.hydrology_candidate())?;

    let potential =
        project_potential_carbon(potential_source, &binding_map, configuration, beginning)?;
    let capped = project_capped_carbon(final_source, &binding_map, configuration, beginning)?;
    let final_state = project_final_state(
        final_source,
        bindings,
        &binding_map,
        configuration,
        beginning,
    )?;
    Ok(V8CoveredProjection {
        potential,
        capped,
        final_state,
    })
}

/// Project the complete configured covered-tile set into one V8 receipt set.
/// Open tiles are deliberately absent: V8 owner state exists only for the
/// configured covered occupancies.
#[allow(clippy::too_many_lines)]
pub(crate) fn project_multi_tile_v8_passes(
    potentials: &[&PotentialCoveredVegetationOperands],
    finals: &[&AcceptedCoveredVegetationOperands],
    bindings: &[V8ComponentOccupancyBinding],
    hydrology: &UnifiedRealHydrologyCandidate,
    configuration: &VegetationConfiguration,
    beginning: &V8CoupledOwnedState,
) -> Result<V8CoveredProjection, V8ProjectionError> {
    project_multi_tile_v8_passes_with_duration(
        potentials,
        finals,
        bindings,
        hydrology,
        configuration,
        beginning,
        None,
    )
}

/// V11 projection using the authenticated common-slab duration bits while
/// retaining the immutable nominal V8/V10 configuration identity.
#[allow(clippy::too_many_arguments)]
pub(crate) fn project_multi_tile_v8_passes_v11(
    potentials: &[&PotentialCoveredVegetationOperands],
    finals: &[&AcceptedCoveredVegetationOperands],
    bindings: &[V8ComponentOccupancyBinding],
    hydrology: &UnifiedRealHydrologyCandidate,
    configuration: &VegetationConfiguration,
    beginning: &V8CoupledOwnedState,
    duration_s_bits: u64,
) -> Result<V8CoveredProjection, V8ProjectionError> {
    project_multi_tile_v8_passes_with_duration(
        potentials,
        finals,
        bindings,
        hydrology,
        configuration,
        beginning,
        Some(duration_s_bits),
    )
}

#[allow(clippy::too_many_arguments, clippy::too_many_lines)]
fn project_multi_tile_v8_passes_with_duration(
    potentials: &[&PotentialCoveredVegetationOperands],
    finals: &[&AcceptedCoveredVegetationOperands],
    bindings: &[V8ComponentOccupancyBinding],
    hydrology: &UnifiedRealHydrologyCandidate,
    configuration: &VegetationConfiguration,
    beginning: &V8CoupledOwnedState,
    duration_s_bits: Option<u64>,
) -> Result<V8CoveredProjection, V8ProjectionError> {
    configuration.validate_v8()?;
    beginning
        .validate(configuration)
        .map_err(|_| V8ProjectionError::Identity("invalid V8 vegetation beginning state"))?;
    if potentials.len() != finals.len() {
        return Err(V8ProjectionError::Identity(
            "incomplete covered potential/final tile set",
        ));
    }
    if potentials.is_empty() {
        if !configuration.expected_occupancies().is_empty() || !bindings.is_empty() {
            return Err(V8ProjectionError::Identity(
                "missing configured covered potential/final tile set",
            ));
        }
        return project_empty_v8_passes(configuration, beginning, duration_s_bits);
    }
    let binding_map = validate_v8_component_bindings(bindings, configuration)?;
    let mut potential_components = BTreeSet::new();
    let mut final_components = BTreeSet::new();
    let mut potential_receipts = Vec::new();
    let mut capped_receipts = Vec::new();
    let mut final_tiles = Vec::new();
    let expected_transaction = TransactionId(
        beginning
            .last_transaction_id
            .checked_add(1)
            .ok_or(V8ProjectionError::Identity("V8 transaction overflow"))?,
    );

    for (potential, final_value) in potentials.iter().zip(finals) {
        potential.validate()?;
        final_value.validate()?;
        if potential.pass != CoveredVegetationOperandPass::Potential
            || final_value.pass != CoveredVegetationOperandPass::FixedAuthorizationFinal
            || potential.transaction_id != expected_transaction
            || final_value.transaction_id != expected_transaction
            || hydrology.transaction_id() != expected_transaction
            || potential.vegetation_model_definition_sha256
                != final_value.vegetation_model_definition_sha256
            || potential.lse_configuration_sha256 != final_value.lse_configuration_sha256
            || potential.beginning_lse_state_sha256 != final_value.beginning_lse_state_sha256
            || potential.vegetation_owner_id != final_value.vegetation_owner_id
            || potential.ofe_id != final_value.ofe_id
            || potential.tile_id != final_value.tile_id
            || potential.tile_fraction.to_bits() != final_value.tile_fraction.to_bits()
            || potential.interval_s.to_bits() != final_value.interval_s.to_bits()
            || potential.interval_s.to_bits()
                != duration_s_bits.unwrap_or_else(|| configuration.dt_s.to_bits())
            || potential.top_rain_kg_m2_tile_ground.to_bits()
                != final_value.top_rain_kg_m2_tile_ground.to_bits()
        {
            return Err(V8ProjectionError::Identity(
                "multi-tile potential/final V8 lineage mismatch",
            ));
        }
        validate_unified_root_protocol(final_value, hydrology)?;

        for occupancy in &potential.occupancies {
            let component = component_id(&occupancy.occupancy_id)?;
            let occupancy_id = mapped_occupancy(&binding_map, &occupancy.occupancy_id)?;
            if occupancy.liquid.beginning_store_kg_m2_tile.to_bits()
                != beginning.occupancies[&occupancy_id]
                    .canopy_liquid_kg_h2o_m2_tile_ground
                    .to_bits()
                || !potential_components.insert(component)
            {
                return Err(V8ProjectionError::Identity(
                    "multi-tile potential component lineage",
                ));
            }
            potential_receipts.push(V8OccupancyCarbonReceipt {
                occupancy_id,
                tile_fraction: potential.tile_fraction,
                operands: V8OccupancyCarbonOperands {
                    sun_leaf_area_m2_m2_tile_ground: occupancy.sun_leaf_area_m2_m2_tile_ground,
                    shade_leaf_area_m2_m2_tile_ground: occupancy.shade_leaf_area_m2_m2_tile_ground,
                    sun_gross_assimilation_umol_co2_m2_leaf_s: occupancy
                        .sun_gross_assimilation_umol_co2_m2_leaf_s,
                    shade_gross_assimilation_umol_co2_m2_leaf_s: occupancy
                        .shade_gross_assimilation_umol_co2_m2_leaf_s,
                    sun_dark_respiration_umol_co2_m2_leaf_s: occupancy
                        .sun_dark_respiration_umol_co2_m2_leaf_s,
                    shade_dark_respiration_umol_co2_m2_leaf_s: occupancy
                        .shade_dark_respiration_umol_co2_m2_leaf_s,
                },
            });
        }

        let mut final_occupancies = Vec::new();
        let mut final_vegetation_tile_ids = BTreeSet::new();
        for occupancy in &final_value.occupancies {
            let component = component_id(&occupancy.occupancy_id)?;
            let occupancy_id = mapped_occupancy(&binding_map, &occupancy.occupancy_id)?;
            final_vegetation_tile_ids.insert(occupancy_id.tile_id.clone());
            if occupancy.liquid.beginning_store_kg_m2_tile.to_bits()
                != beginning.occupancies[&occupancy_id]
                    .canopy_liquid_kg_h2o_m2_tile_ground
                    .to_bits()
                || !final_components.insert(component.clone())
            {
                return Err(V8ProjectionError::Identity(
                    "multi-tile final component lineage",
                ));
            }
            let carbon = carbon_operands(occupancy);
            capped_receipts.push(V8OccupancyCarbonReceipt {
                occupancy_id,
                tile_fraction: final_value.tile_fraction,
                operands: carbon,
            });
            final_occupancies.push(V8FinalOccupancyReceipt {
                component_id: component,
                beginning_canopy_liquid_kg_m2_tile_ground: occupancy
                    .liquid
                    .beginning_store_kg_m2_tile,
                ending_canopy_liquid_kg_m2_tile_ground: occupancy.liquid.ending_store_kg_m2_tile,
                dry_stem_temperature_k: occupancy.dry_stem_temperature_k,
                root_node_potential_mm: occupancy.root_node_potential_mm,
                shade_ci_pa: occupancy.shade_ci_pa,
                shade_leaf_potential_mm: occupancy.shade_leaf_potential_mm,
                shade_leaf_temperature_k: occupancy.shade_leaf_temperature_k,
                stem_potential_mm: occupancy.stem_potential_mm,
                sun_ci_pa: occupancy.sun_ci_pa,
                sun_leaf_potential_mm: occupancy.sun_leaf_potential_mm,
                sun_leaf_temperature_k: occupancy.sun_leaf_temperature_k,
                wet_surface_temperature_k: occupancy.wet_surface_temperature_k,
                beta_hyd: occupancy.beta_hyd,
                carbon,
                root_water: occupancy
                    .root_water
                    .iter()
                    .map(|root| {
                        Ok(V8FinalRootWaterReceipt {
                            layer_id: root
                                .key
                                .soil_layer_id
                                .clone()
                                .ok_or(V8ProjectionError::Identity("root water layer identity"))?,
                            request_kg_m2_stand_ground: root.request_kg_m2_stand_ground,
                            authorization_kg_m2_stand_ground: root.authorization_kg_m2_stand_ground,
                            finalized_use_kg_m2_stand_ground: root.finalized_use_kg_m2_stand_ground,
                        })
                    })
                    .collect::<Result<Vec<_>, V8ProjectionError>>()?,
            });
        }
        let vegetation_tile_count = final_vegetation_tile_ids.len();
        let vegetation_tile_id = final_vegetation_tile_ids
            .into_iter()
            .next()
            .filter(|_| vegetation_tile_count == 1)
            .ok_or(V8ProjectionError::Identity(
                "multi-tile final vegetation-tile mapping",
            ))?;
        final_occupancies.sort_by(|left, right| left.component_id.cmp(&right.component_id));
        final_tiles.push(V8FinalTileReceipt {
            pass: V8PhysicalReceiptPass::FixedAuthorizationFinal,
            transaction_id: final_value.transaction_id,
            vegetation_model_definition_sha256: final_value
                .vegetation_model_definition_sha256
                .to_owned(),
            vegetation_configuration_sha256: configuration.configuration_sha256.clone(),
            vegetation_beginning_state_sha256: beginning.state_sha256.clone(),
            lse_configuration_sha256: final_value.lse_configuration_sha256.as_str().to_owned(),
            lse_beginning_state_sha256: final_value.beginning_lse_state_sha256.as_str().to_owned(),
            tile_id: vegetation_tile_id,
            tile_fraction: final_value.tile_fraction,
            interval_s: final_value.interval_s,
            canopy_air_temperature_k: final_value.canopy_air_temperature_k,
            canopy_air_specific_humidity_kg_kg: final_value.canopy_air_specific_humidity_kg_kg,
            occupancies: final_occupancies,
        });
    }
    if potential_components != final_components
        || potential_components != binding_map.keys().cloned().collect()
    {
        return Err(V8ProjectionError::Identity(
            "multi-tile potential/final component set mismatch",
        ));
    }
    potential_receipts.sort_by(|left, right| left.occupancy_id.cmp(&right.occupancy_id));
    capped_receipts.sort_by(|left, right| left.occupancy_id.cmp(&right.occupancy_id));
    final_tiles.sort_by(|left, right| left.tile_id.cmp(&right.tile_id));
    let first = potentials[0];
    let make_pass = |pass, receipts| match duration_s_bits {
        Some(bits) => ValidatedV8CarbonPass::try_new_v11(
            first.vegetation_model_definition_sha256.to_owned(),
            configuration.configuration_sha256.clone(),
            expected_transaction,
            beginning.state_sha256.clone(),
            pass,
            first.interval_s,
            receipts,
            configuration,
            beginning,
            bits,
        ),
        None => ValidatedV8CarbonPass::try_new(
            first.vegetation_model_definition_sha256.to_owned(),
            configuration.configuration_sha256.clone(),
            expected_transaction,
            beginning.state_sha256.clone(),
            pass,
            first.interval_s,
            receipts,
            configuration,
            beginning,
        ),
    };
    let potential = make_pass(CoupledSolvePass::Potential, potential_receipts)?;
    let capped = make_pass(CoupledSolvePass::Capped, capped_receipts)?;
    let final_state = match duration_s_bits {
        Some(bits) => ValidatedV8FinalStatePass::try_new_v11(
            bindings,
            final_tiles,
            configuration,
            beginning,
            bits,
        )?,
        None => {
            ValidatedV8FinalStatePass::try_new(bindings, final_tiles, configuration, beginning)?
        }
    };
    Ok(V8CoveredProjection {
        potential,
        capped,
        final_state,
    })
}

fn project_empty_v8_passes(
    configuration: &VegetationConfiguration,
    beginning: &V8CoupledOwnedState,
    duration_s_bits: Option<u64>,
) -> Result<V8CoveredProjection, V8ProjectionError> {
    let transaction_id = TransactionId(
        beginning
            .last_transaction_id
            .checked_add(1)
            .ok_or(V8ProjectionError::Identity("V8 transaction overflow"))?,
    );
    let interval_s = duration_s_bits.map_or(configuration.dt_s, f64::from_bits);
    let make_carbon = |pass| match duration_s_bits {
        Some(bits) => ValidatedV8CarbonPass::try_new_v11(
            openwepp_vegetation::V8_MODEL_SHA256.into(),
            configuration.configuration_sha256.clone(),
            transaction_id,
            beginning.state_sha256.clone(),
            pass,
            interval_s,
            Vec::new(),
            configuration,
            beginning,
            bits,
        ),
        None => ValidatedV8CarbonPass::try_new(
            openwepp_vegetation::V8_MODEL_SHA256.into(),
            configuration.configuration_sha256.clone(),
            transaction_id,
            beginning.state_sha256.clone(),
            pass,
            interval_s,
            Vec::new(),
            configuration,
            beginning,
        ),
    };
    let final_state = match duration_s_bits {
        Some(bits) => {
            ValidatedV8FinalStatePass::try_new_v11(&[], Vec::new(), configuration, beginning, bits)?
        }
        None => ValidatedV8FinalStatePass::try_new(&[], Vec::new(), configuration, beginning)?,
    };
    Ok(V8CoveredProjection {
        potential: make_carbon(CoupledSolvePass::Potential)?,
        capped: make_carbon(CoupledSolvePass::Capped)?,
        final_state,
    })
}

fn validate_shared_lineage(
    physical: &CoveredForestShadowResult,
    potential: &PotentialCoveredVegetationOperands,
    final_value: &AcceptedCoveredVegetationOperands,
    configuration: &VegetationConfiguration,
    beginning: &V8CoupledOwnedState,
) -> Result<(), V8ProjectionError> {
    let expected_transaction = TransactionId(
        beginning
            .last_transaction_id
            .checked_add(1)
            .ok_or(V8ProjectionError::Identity("V8 transaction overflow"))?,
    );
    let final_identity = &physical.final_tile().identity;
    if potential.pass != CoveredVegetationOperandPass::Potential
        || final_value.pass != CoveredVegetationOperandPass::FixedAuthorizationFinal
        || potential.transaction_id != expected_transaction
        || final_value.transaction_id != expected_transaction
        || physical.hydrology_candidate().transaction_id() != expected_transaction
        || final_identity.transaction_id != expected_transaction
        || potential.vegetation_model_definition_sha256 != beginning.model_definition_sha256
        || final_value.vegetation_model_definition_sha256 != beginning.model_definition_sha256
        || potential.lse_configuration_sha256 != final_value.lse_configuration_sha256
        || potential.beginning_lse_state_sha256 != final_value.beginning_lse_state_sha256
        || potential.lse_configuration_sha256 != final_identity.configuration_sha256
        || potential.beginning_lse_state_sha256 != final_identity.beginning_lse_state_sha256
        || potential.vegetation_owner_id != final_value.vegetation_owner_id
        || potential.ofe_id != final_value.ofe_id
        || potential.tile_id != final_value.tile_id
        || potential.tile_id != final_identity.tile_id
        || potential.tile_fraction.to_bits() != final_value.tile_fraction.to_bits()
        || potential.tile_fraction.to_bits() != final_identity.tile_fraction.to_bits()
        || potential.interval_s.to_bits() != final_value.interval_s.to_bits()
        || potential.interval_s.to_bits() != configuration.dt_s.to_bits()
        || potential.interval_s.to_bits() != final_identity.interval_s.to_bits()
        || potential.top_rain_kg_m2_tile_ground.to_bits()
            != final_value.top_rain_kg_m2_tile_ground.to_bits()
    {
        return Err(V8ProjectionError::Identity(
            "potential/final V8 physical lineage mismatch",
        ));
    }
    if physical.submitted_request_batch().transaction_id != expected_transaction
        || physical.submitted_request_batch().requests
            != physical.hydrology_candidate().arbitration().requests
        || physical.final_tile().water_protocol.requests
            != physical.hydrology_candidate().arbitration().requests
        || physical.final_tile().water_protocol.authorizations
            != physical.hydrology_candidate().arbitration().authorizations
        || physical.final_tile().water_protocol.finalized_uses
            != physical.hydrology_candidate().finalized_uses()
    {
        return Err(V8ProjectionError::Identity(
            "unified hydrology protocol lineage mismatch",
        ));
    }
    Ok(())
}

fn component_id(value: &ComponentId) -> Result<V8LseComponentId, V8ProjectionError> {
    Ok(V8LseComponentId::try_new(value.as_str())?)
}

fn validate_component_sets(
    potential: &PotentialCoveredVegetationOperands,
    final_value: &AcceptedCoveredVegetationOperands,
    bindings: &BTreeMap<V8LseComponentId, OccupancyId>,
    beginning: &V8CoupledOwnedState,
) -> Result<(), V8ProjectionError> {
    let potential_components = potential
        .occupancies
        .iter()
        .map(|value| component_id(&value.occupancy_id))
        .collect::<Result<BTreeSet<_>, _>>()?;
    let final_components = final_value
        .occupancies
        .iter()
        .map(|value| component_id(&value.occupancy_id))
        .collect::<Result<BTreeSet<_>, _>>()?;
    if potential_components != final_components
        || potential_components != bindings.keys().cloned().collect()
    {
        return Err(V8ProjectionError::Identity(
            "potential/final component set mismatch",
        ));
    }
    for occupancy in &potential.occupancies {
        let id = bindings
            .get(&component_id(&occupancy.occupancy_id)?)
            .ok_or(V8ProjectionError::Identity("unbound potential component"))?;
        if id.tile_id != potential.tile_id
            || occupancy.liquid.beginning_store_kg_m2_tile.to_bits()
                != beginning.occupancies[id]
                    .canopy_liquid_kg_h2o_m2_tile_ground
                    .to_bits()
        {
            return Err(V8ProjectionError::Identity(
                "potential E04 beginning occupancy lineage",
            ));
        }
    }
    for occupancy in &final_value.occupancies {
        let id = bindings
            .get(&component_id(&occupancy.occupancy_id)?)
            .ok_or(V8ProjectionError::Identity("unbound final component"))?;
        if id.tile_id != final_value.tile_id
            || occupancy.liquid.beginning_store_kg_m2_tile.to_bits()
                != beginning.occupancies[id]
                    .canopy_liquid_kg_h2o_m2_tile_ground
                    .to_bits()
        {
            return Err(V8ProjectionError::Identity(
                "final E04 beginning occupancy lineage",
            ));
        }
    }
    Ok(())
}

fn project_potential_carbon(
    source: &PotentialCoveredVegetationOperands,
    bindings: &BTreeMap<V8LseComponentId, OccupancyId>,
    configuration: &VegetationConfiguration,
    beginning: &V8CoupledOwnedState,
) -> Result<ValidatedV8CarbonPass, V8ProjectionError> {
    let mut occupancies = source
        .occupancies
        .iter()
        .map(|value| {
            Ok(V8OccupancyCarbonReceipt {
                occupancy_id: mapped_occupancy(bindings, &value.occupancy_id)?,
                tile_fraction: source.tile_fraction,
                operands: V8OccupancyCarbonOperands {
                    sun_leaf_area_m2_m2_tile_ground: value.sun_leaf_area_m2_m2_tile_ground,
                    shade_leaf_area_m2_m2_tile_ground: value.shade_leaf_area_m2_m2_tile_ground,
                    sun_gross_assimilation_umol_co2_m2_leaf_s: value
                        .sun_gross_assimilation_umol_co2_m2_leaf_s,
                    shade_gross_assimilation_umol_co2_m2_leaf_s: value
                        .shade_gross_assimilation_umol_co2_m2_leaf_s,
                    sun_dark_respiration_umol_co2_m2_leaf_s: value
                        .sun_dark_respiration_umol_co2_m2_leaf_s,
                    shade_dark_respiration_umol_co2_m2_leaf_s: value
                        .shade_dark_respiration_umol_co2_m2_leaf_s,
                },
            })
        })
        .collect::<Result<Vec<_>, V8ProjectionError>>()?;
    occupancies.sort_by(|left, right| left.occupancy_id.cmp(&right.occupancy_id));
    Ok(ValidatedV8CarbonPass::try_new(
        source.vegetation_model_definition_sha256.to_owned(),
        configuration.configuration_sha256.clone(),
        source.transaction_id,
        beginning.state_sha256.clone(),
        CoupledSolvePass::Potential,
        source.interval_s,
        occupancies,
        configuration,
        beginning,
    )?)
}

fn project_capped_carbon(
    source: &AcceptedCoveredVegetationOperands,
    bindings: &BTreeMap<V8LseComponentId, OccupancyId>,
    configuration: &VegetationConfiguration,
    beginning: &V8CoupledOwnedState,
) -> Result<ValidatedV8CarbonPass, V8ProjectionError> {
    let mut occupancies = source
        .occupancies
        .iter()
        .map(|value| {
            Ok(V8OccupancyCarbonReceipt {
                occupancy_id: mapped_occupancy(bindings, &value.occupancy_id)?,
                tile_fraction: source.tile_fraction,
                operands: carbon_operands(value),
            })
        })
        .collect::<Result<Vec<_>, V8ProjectionError>>()?;
    occupancies.sort_by(|left, right| left.occupancy_id.cmp(&right.occupancy_id));
    Ok(ValidatedV8CarbonPass::try_new(
        source.vegetation_model_definition_sha256.to_owned(),
        configuration.configuration_sha256.clone(),
        source.transaction_id,
        beginning.state_sha256.clone(),
        CoupledSolvePass::Capped,
        source.interval_s,
        occupancies,
        configuration,
        beginning,
    )?)
}

fn project_final_state(
    source: &AcceptedCoveredVegetationOperands,
    bindings: &[V8ComponentOccupancyBinding],
    binding_map: &BTreeMap<V8LseComponentId, OccupancyId>,
    configuration: &VegetationConfiguration,
    beginning: &V8CoupledOwnedState,
) -> Result<ValidatedV8FinalStatePass, V8ProjectionError> {
    let mut occupancies = source
        .occupancies
        .iter()
        .map(|value| {
            let component_id = component_id(&value.occupancy_id)?;
            let _ = binding_map
                .get(&component_id)
                .ok_or(V8ProjectionError::Identity("unbound final component"))?;
            Ok(V8FinalOccupancyReceipt {
                component_id,
                beginning_canopy_liquid_kg_m2_tile_ground: value.liquid.beginning_store_kg_m2_tile,
                ending_canopy_liquid_kg_m2_tile_ground: value.liquid.ending_store_kg_m2_tile,
                dry_stem_temperature_k: value.dry_stem_temperature_k,
                root_node_potential_mm: value.root_node_potential_mm,
                shade_ci_pa: value.shade_ci_pa,
                shade_leaf_potential_mm: value.shade_leaf_potential_mm,
                shade_leaf_temperature_k: value.shade_leaf_temperature_k,
                stem_potential_mm: value.stem_potential_mm,
                sun_ci_pa: value.sun_ci_pa,
                sun_leaf_potential_mm: value.sun_leaf_potential_mm,
                sun_leaf_temperature_k: value.sun_leaf_temperature_k,
                wet_surface_temperature_k: value.wet_surface_temperature_k,
                beta_hyd: value.beta_hyd,
                carbon: carbon_operands(value),
                root_water: value
                    .root_water
                    .iter()
                    .map(|root| {
                        Ok(V8FinalRootWaterReceipt {
                            layer_id: root
                                .key
                                .soil_layer_id
                                .clone()
                                .ok_or(V8ProjectionError::Identity("root water layer identity"))?,
                            request_kg_m2_stand_ground: root.request_kg_m2_stand_ground,
                            authorization_kg_m2_stand_ground: root.authorization_kg_m2_stand_ground,
                            finalized_use_kg_m2_stand_ground: root.finalized_use_kg_m2_stand_ground,
                        })
                    })
                    .collect::<Result<Vec<_>, V8ProjectionError>>()?,
            })
        })
        .collect::<Result<Vec<_>, V8ProjectionError>>()?;
    occupancies.sort_by(|left, right| left.component_id.cmp(&right.component_id));
    Ok(ValidatedV8FinalStatePass::try_new(
        bindings,
        vec![V8FinalTileReceipt {
            pass: V8PhysicalReceiptPass::FixedAuthorizationFinal,
            transaction_id: source.transaction_id,
            vegetation_model_definition_sha256: source
                .vegetation_model_definition_sha256
                .to_owned(),
            vegetation_configuration_sha256: configuration.configuration_sha256.clone(),
            vegetation_beginning_state_sha256: beginning.state_sha256.clone(),
            lse_configuration_sha256: source.lse_configuration_sha256.as_str().to_owned(),
            lse_beginning_state_sha256: source.beginning_lse_state_sha256.as_str().to_owned(),
            tile_id: source.tile_id.clone(),
            tile_fraction: source.tile_fraction,
            interval_s: source.interval_s,
            canopy_air_temperature_k: source.canopy_air_temperature_k,
            canopy_air_specific_humidity_kg_kg: source.canopy_air_specific_humidity_kg_kg,
            occupancies,
        }],
        configuration,
        beginning,
    )?)
}

fn mapped_occupancy(
    bindings: &BTreeMap<V8LseComponentId, OccupancyId>,
    component: &ComponentId,
) -> Result<OccupancyId, V8ProjectionError> {
    bindings
        .get(&component_id(component)?)
        .cloned()
        .ok_or(V8ProjectionError::Identity(
            "unbound LSE component identity",
        ))
}

fn carbon_operands(value: &AcceptedCoveredOccupancyOperands) -> V8OccupancyCarbonOperands {
    V8OccupancyCarbonOperands {
        sun_leaf_area_m2_m2_tile_ground: value.sun_leaf_area_m2_m2_tile_ground,
        shade_leaf_area_m2_m2_tile_ground: value.shade_leaf_area_m2_m2_tile_ground,
        sun_gross_assimilation_umol_co2_m2_leaf_s: value.sun_gross_assimilation_umol_co2_m2_leaf_s,
        shade_gross_assimilation_umol_co2_m2_leaf_s: value
            .shade_gross_assimilation_umol_co2_m2_leaf_s,
        sun_dark_respiration_umol_co2_m2_leaf_s: value.sun_dark_respiration_umol_co2_m2_leaf_s,
        shade_dark_respiration_umol_co2_m2_leaf_s: value.shade_dark_respiration_umol_co2_m2_leaf_s,
    }
}

fn validate_unified_root_protocol(
    source: &AcceptedCoveredVegetationOperands,
    hydrology: &UnifiedRealHydrologyCandidate,
) -> Result<(), V8ProjectionError> {
    let requests = rows_by_key(&hydrology.arbitration().requests)?;
    let authorizations = authorization_rows_by_key(&hydrology.arbitration().authorizations)?;
    let uses = rows_by_key(hydrology.finalized_uses())?;
    let expected = source
        .occupancies
        .iter()
        .flat_map(|occupancy| occupancy.root_water.iter().map(|root| root.key.clone()))
        .collect::<BTreeSet<_>>();
    let actual = requests
        .keys()
        .filter(|key| {
            key.requesting_component == RequestingComponent::VegetationRoot
                && key.requesting_owner_id == source.vegetation_owner_id
                && key.ofe_id == source.ofe_id
                && key.requesting_tile_id == source.tile_id
        })
        .cloned()
        .collect::<BTreeSet<_>>();
    if expected != actual
        || expected
            != authorizations
                .keys()
                .filter(|key| actual.contains(*key))
                .cloned()
                .collect()
        || expected
            != uses
                .keys()
                .filter(|key| actual.contains(*key))
                .cloned()
                .collect()
    {
        return Err(V8ProjectionError::Identity(
            "covered root protocol key set mismatch",
        ));
    }
    for occupancy in &source.occupancies {
        for root in &occupancy.root_water {
            if root.key.occupancy_id.as_ref() != Some(&occupancy.occupancy_id)
                || requests[&root.key].to_bits() != root.request_kg_m2_stand_ground.to_bits()
                || authorizations[&root.key].to_bits()
                    != root.authorization_kg_m2_stand_ground.to_bits()
                || uses[&root.key].to_bits() != root.finalized_use_kg_m2_stand_ground.to_bits()
            {
                return Err(V8ProjectionError::Identity("covered root D/A/F mismatch"));
            }
        }
    }
    Ok(())
}

fn rows_by_key(rows: &[WaterAmount]) -> Result<BTreeMap<GroundWaterKey, f64>, V8ProjectionError> {
    let mut values = BTreeMap::new();
    for row in rows {
        if values
            .insert(row.key.clone(), row.amount_kg_m2_stand_ground)
            .is_some()
        {
            return Err(V8ProjectionError::Identity(
                "duplicate unified water amount key",
            ));
        }
    }
    Ok(values)
}

fn authorization_rows_by_key(
    rows: &[WaterAuthorization],
) -> Result<BTreeMap<GroundWaterKey, f64>, V8ProjectionError> {
    let mut values = BTreeMap::new();
    for row in rows {
        if values
            .insert(row.key.clone(), row.amount_kg_m2_stand_ground)
            .is_some()
        {
            return Err(V8ProjectionError::Identity(
                "duplicate unified water authorization key",
            ));
        }
    }
    Ok(values)
}

#[cfg(test)]
mod tests {
    use std::collections::BTreeMap;

    use openwepp_kernel_contract::{ResourceOwnerId, SoilLayerId, TileId, TransactionId};
    use openwepp_land_surface_energy::{
        OfeId, SourceId, StandGroundWaterAmountBasis, WaterSourceType,
    };

    use super::*;
    use openwepp_vegetation::{TopologyTile, V8_MODEL_SHA256};

    fn root_key() -> GroundWaterKey {
        GroundWaterKey {
            transaction_id: TransactionId(7),
            requesting_owner_id: ResourceOwnerId::try_new("vegetation-v8").expect("owner"),
            requesting_component: RequestingComponent::VegetationRoot,
            ofe_id: OfeId::try_new("ofe-1").expect("OFE"),
            requesting_tile_id: TileId::try_new("forest").expect("tile"),
            occupancy_id: Some(ComponentId::try_new("upper").expect("component")),
            surface_id: None,
            surface_class: None,
            source_type: WaterSourceType::SoilLayerLiquid,
            source_id: SourceId::try_new("soil-1").expect("source"),
            source_tile_id: None,
            soil_layer_id: Some(SoilLayerId::try_new("soil-1").expect("layer")),
            amount_basis: StandGroundWaterAmountBasis::KgH2oM2StandGroundInterval,
        }
    }

    #[test]
    fn duplicate_unified_root_rows_are_rejected_before_projection() {
        let row = WaterAmount {
            key: root_key(),
            amount_kg_m2_stand_ground: 0.2,
        };
        assert_eq!(
            rows_by_key(&[row.clone(), row]),
            Err(V8ProjectionError::Identity(
                "duplicate unified water amount key"
            ))
        );
    }

    #[test]
    fn component_identity_is_converted_without_string_parsing_or_normalization() {
        let canonical = ComponentId::try_new("upper").expect("component");
        assert_eq!(
            component_id(&canonical).expect("V8 component").as_str(),
            canonical.as_str()
        );

        // The LSE identity type admits nonempty whitespace-surrounded bytes;
        // the stricter V8 boundary rejects rather than trims or parses them.
        let noncanonical = ComponentId::try_new(" upper ").expect("LSE component");
        assert!(matches!(
            component_id(&noncanonical),
            Err(V8ProjectionError::Vegetation(VegetationError::Receipt(_)))
        ));
    }

    #[test]
    fn all_open_topology_projects_empty_canonical_v8_passes() {
        let mut configuration = VegetationConfiguration {
            model_definition_sha256: V8_MODEL_SHA256.into(),
            configuration_sha256: String::new(),
            initial_state_sha256: "0".repeat(64),
            area_m2: 1.0,
            timestamp: "2026-08-17T00:00:00Z".into(),
            dt_s: 1_800.0,
            topology_tiles: vec![TopologyTile {
                tile_id: TileId::try_new("open").expect("tile"),
                fraction: 1.0,
            }],
            strata: Vec::new(),
        };
        configuration.configuration_sha256 = configuration
            .canonical_sha256()
            .expect("configuration digest");
        let mut beginning = V8CoupledOwnedState {
            configuration_sha256: configuration.configuration_sha256.clone(),
            last_transaction_id: 40,
            model_definition_sha256: V8_MODEL_SHA256.into(),
            occupancies: BTreeMap::new(),
            state_sha256: String::new(),
            strata: BTreeMap::new(),
            tile_canopy_air: BTreeMap::new(),
        };
        beginning.state_sha256 = beginning.canonical_sha256();
        project_empty_v8_passes(&configuration, &beginning, None).expect("empty projection");
    }
}
