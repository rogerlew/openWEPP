//! Dependency-neutral potential/final transaction and owner-candidate surfaces.
//!
//! Hydrology supplies typed maximum authorizations and post-solve ingress
//! partitions. This module never calls a hydrology implementation and never
//! mutates an owner. It reconstructs the fixed-cap solve from a retained clone
//! of the immutable beginning problem and returns sealed candidate operands.

#![allow(clippy::missing_errors_doc)]

use std::collections::{BTreeMap, BTreeSet};

use openwepp_kernel_contract::{ResourceOwnerId, SoilLayerId, TileId, TransactionId};
use serde::{Deserialize, Serialize};

use crate::{
    AIR_HEAT_CAPACITY_J_KG_K, AcceptedOpenSurface, ComponentId, CondensationCredit,
    CoveredColumnCandidate, CoveredColumnInputs, CoveredColumnSolveOutcome,
    CoveredOccupancyLiquidLedger, CoveredWaterCaps, DRY_AIR_GAS_CONSTANT_J_KG_K,
    DiagnosticFailureKind, GroundHeatJoinOperands, GroundWaterKey, LandSurfaceEnergyError,
    LandSurfaceEnergyState, LatentJoinOperands, MODEL_DEFINITION_SHA256, MODEL_VERSION,
    NormalizedResidual, NumericalDiagnostics, NumericalFailure, NumericalFailureCode,
    NumericalFailureKind, OfeId, OpenSurfaceProblem, OpenSurfaceSolveOutcome,
    OwnerEnvelopeIdentity, OwnerKind, OwnerRollbackHash, RequestingComponent, ResidualUnit,
    Sha256Digest, SoilThermalSnapshot, SolveIdentity, SolvePass, SourceId, SourceWaterCap,
    StandGroundWaterAmountBasis, StepNorms, SurfaceClass, SurfaceClassKind, SurfaceEnergyOperands,
    SurfaceId, TileState, VEGETATION_MODEL_DEFINITION_SHA256, VEGETATION_MODEL_VERSION,
    WaterAmount, WaterAuthorization, WaterProtocol, WaterSourceType, canonical_digest,
    liquid_enthalpy_j_kg, solve_covered_column, solve_open_surface,
    under_canopy_neutral_resistance, validate_ground_heat_join, validate_latent_join,
    validate_surface_energy,
};

#[derive(Clone, Debug, PartialEq)]
pub struct RuntimeTileIdentity {
    pub transaction_id: TransactionId,
    pub lse_owner_id: ResourceOwnerId,
    pub hydrology_owner_id: ResourceOwnerId,
    pub soil_thermal_owner_id: ResourceOwnerId,
    pub configuration_sha256: Sha256Digest,
    pub beginning_lse_state_sha256: Sha256Digest,
    pub beginning_hydrology_snapshot_sha256: Sha256Digest,
    pub beginning_soil_thermal_state_sha256: Sha256Digest,
    pub ofe_id: OfeId,
    pub tile_id: TileId,
    pub surface_id: SurfaceId,
    pub surface_class: SurfaceClass,
    pub ground_source_type: WaterSourceType,
    pub ground_source_id: SourceId,
    pub ground_source_tile_id: Option<TileId>,
    pub ground_soil_layer_id: Option<SoilLayerId>,
    pub tile_fraction: f64,
    pub interval_s: f64,
}

impl RuntimeTileIdentity {
    fn validate(&self) -> Result<(), LandSurfaceEnergyError> {
        if self.transaction_id.0 == 0 {
            return Err(LandSurfaceEnergyError::StateLineage(
                "zero runtime transaction",
            ));
        }
        if !self.tile_fraction.is_finite()
            || self.tile_fraction <= 0.0
            || self.tile_fraction > 1.0
            || !self.interval_s.is_finite()
            || self.interval_s <= 0.0
        {
            return Err(LandSurfaceEnergyError::ConstitutiveDomain(
                "runtime tile fraction or interval",
            ));
        }
        match self.ground_source_type {
            WaterSourceType::SoilLayerLiquid => {
                if self.ground_source_tile_id.is_some() || self.ground_soil_layer_id.is_none() {
                    return Err(LandSurfaceEnergyError::water_identity(
                        "ground soil source identity",
                    ));
                }
            }
            WaterSourceType::SurfaceLiquid | WaterSourceType::LitterLiquid => {
                if self.ground_source_tile_id.as_ref() != Some(&self.tile_id)
                    || self.ground_soil_layer_id.is_some()
                {
                    return Err(LandSurfaceEnergyError::water_identity(
                        "ground tile source identity",
                    ));
                }
            }
        }
        Ok(())
    }

    fn ground_key(&self) -> GroundWaterKey {
        GroundWaterKey {
            transaction_id: self.transaction_id,
            requesting_owner_id: self.lse_owner_id.clone(),
            requesting_component: RequestingComponent::GroundSurface,
            ofe_id: self.ofe_id.clone(),
            requesting_tile_id: self.tile_id.clone(),
            occupancy_id: None,
            surface_id: Some(self.surface_id.clone()),
            surface_class: Some(self.surface_class),
            source_type: self.ground_source_type,
            source_id: self.ground_source_id.clone(),
            source_tile_id: self.ground_source_tile_id.clone(),
            soil_layer_id: self.ground_soil_layer_id.clone(),
            amount_basis: StandGroundWaterAmountBasis::KgH2oM2StandGroundInterval,
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct RootRuntimeIdentity {
    pub solver_occupancy_id: String,
    /// Vegetation owner that issued this root withdrawal. This is deliberately
    /// distinct from the LSE owner carried by [`RuntimeTileIdentity`].
    pub requesting_owner_id: ResourceOwnerId,
    pub occupancy_id: ComponentId,
    pub layer_id: SoilLayerId,
    pub source_id: SourceId,
}

#[derive(Clone, Debug, PartialEq, Serialize)]
struct PotentialSignature<'a> {
    transaction_id: TransactionId,
    beginning_lse_state_sha256: &'a Sha256Digest,
    requests: &'a [WaterAmount],
}

#[derive(Clone, Debug, PartialEq)]
pub struct PotentialWaterRequestBatch {
    pub transaction_id: TransactionId,
    pub beginning_lse_state_sha256: Sha256Digest,
    pub requests: Vec<WaterAmount>,
    pub potential_signature_sha256: Sha256Digest,
}

impl PotentialWaterRequestBatch {
    pub fn try_new(
        transaction_id: TransactionId,
        beginning_lse_state_sha256: Sha256Digest,
        requests: Vec<WaterAmount>,
    ) -> Result<Self, LandSurfaceEnergyError> {
        let potential_signature_sha256 = canonical_digest(&PotentialSignature {
            transaction_id,
            beginning_lse_state_sha256: &beginning_lse_state_sha256,
            requests: &requests,
        })?;
        let batch = Self {
            transaction_id,
            beginning_lse_state_sha256,
            requests,
            potential_signature_sha256,
        };
        batch.validate()?;
        Ok(batch)
    }

    pub fn validate(&self) -> Result<(), LandSurfaceEnergyError> {
        if self.transaction_id.0 == 0 {
            return Err(LandSurfaceEnergyError::water_identity(
                "empty or zero-transaction potential request batch",
            ));
        }
        if self.requests.is_empty() {
            return Err(LandSurfaceEnergyError::water_cardinality(
                "empty or zero-transaction potential request batch",
            ));
        }
        let mut keys = BTreeSet::new();
        for request in &self.requests {
            request.key.validate(self.transaction_id)?;
            if !request.amount_kg_m2_stand_ground.is_finite() {
                return Err(LandSurfaceEnergyError::NonFinite(
                    "invalid or duplicate potential request",
                ));
            }
            if request.amount_kg_m2_stand_ground < 0.0 {
                return Err(LandSurfaceEnergyError::water_bound(
                    "invalid or duplicate potential request",
                ));
            }
            if !keys.insert(request.key.clone()) {
                return Err(LandSurfaceEnergyError::water_cardinality(
                    "invalid or duplicate potential request",
                ));
            }
        }
        let computed = canonical_digest(&PotentialSignature {
            transaction_id: self.transaction_id,
            beginning_lse_state_sha256: &self.beginning_lse_state_sha256,
            requests: &self.requests,
        })?;
        if computed != self.potential_signature_sha256 {
            return Err(LandSurfaceEnergyError::Identity {
                field: "potential request batch digest",
                expected: computed.to_string(),
                found: self.potential_signature_sha256.to_string(),
            });
        }
        Ok(())
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct OpenPotentialPhase {
    pub identity: RuntimeTileIdentity,
    beginning: OpenSurfaceProblem,
    pub accepted: AcceptedOpenSurface,
    pub request_batch: PotentialWaterRequestBatch,
}

pub fn solve_open_potential_phase(
    identity: RuntimeTileIdentity,
    beginning: &OpenSurfaceProblem,
    initial_trial: Option<Vec<f64>>,
) -> Result<OpenPotentialPhase, LandSurfaceEnergyError> {
    identity.validate()?;
    if identity.tile_fraction.to_bits() != beginning.tile_fraction.to_bits()
        || identity.interval_s.to_bits() != beginning.interval_s.to_bits()
        || identity.surface_class
            != match beginning.class {
                SurfaceClassKind::BareMineralSoil => SurfaceClass::BareMineralSoil,
                SurfaceClassKind::ForestLitter => SurfaceClass::ForestLitter,
            }
    {
        return Err(LandSurfaceEnergyError::Identity {
            field: "runtime problem identity",
            expected: "matching tile fraction, interval, and surface class".into(),
            found: "mismatch".into(),
        });
    }
    let accepted = match solve_open_surface(beginning, None, initial_trial)? {
        OpenSurfaceSolveOutcome::Accepted(value) => value,
        OpenSurfaceSolveOutcome::Rejected(_) => {
            return Err(LandSurfaceEnergyError::NumericalAcceptedResidual);
        }
    };
    let requests = vec![WaterAmount {
        key: identity.ground_key(),
        amount_kg_m2_stand_ground: accepted.evaluation.water.request_kg_m2_stand_ground,
    }];
    let signature = canonical_digest(&PotentialSignature {
        transaction_id: identity.transaction_id,
        beginning_lse_state_sha256: &identity.beginning_lse_state_sha256,
        requests: &requests,
    })?;
    let request_batch = PotentialWaterRequestBatch {
        transaction_id: identity.transaction_id,
        beginning_lse_state_sha256: identity.beginning_lse_state_sha256.clone(),
        requests,
        potential_signature_sha256: signature,
    };
    request_batch.validate()?;
    Ok(OpenPotentialPhase {
        identity,
        beginning: beginning.clone(),
        accepted,
        request_batch,
    })
}

#[derive(Clone, Debug, PartialEq)]
pub struct CoveredPotentialPhase {
    pub identity: RuntimeTileIdentity,
    beginning: CoveredColumnInputs,
    pub accepted: Box<CoveredColumnCandidate>,
    pub request_batch: PotentialWaterRequestBatch,
    pub potential_vegetation_operands: PotentialCoveredVegetationOperands,
    root_identities: BTreeMap<(String, String), RootRuntimeIdentity>,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize)]
pub enum CoveredVegetationOperandPass {
    Potential,
    FixedAuthorizationFinal,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum SealedCoveredVegetationOperands {
    Potential,
    FixedAuthorizationFinal,
}

#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct PotentialCoveredOccupancyCarbonOperands {
    pub occupancy_id: ComponentId,
    pub sun_leaf_area_m2_m2_tile_ground: f64,
    pub shade_leaf_area_m2_m2_tile_ground: f64,
    pub sun_gross_assimilation_umol_co2_m2_leaf_s: f64,
    pub shade_gross_assimilation_umol_co2_m2_leaf_s: f64,
    pub sun_net_assimilation_umol_co2_m2_leaf_s: f64,
    pub shade_net_assimilation_umol_co2_m2_leaf_s: f64,
    pub sun_dark_respiration_umol_co2_m2_leaf_s: f64,
    pub shade_dark_respiration_umol_co2_m2_leaf_s: f64,
    pub liquid: CoveredOccupancyLiquidLedger,
}

#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct PotentialCoveredVegetationOperands {
    pub pass: CoveredVegetationOperandPass,
    pub transaction_id: TransactionId,
    pub vegetation_model_version: &'static str,
    pub vegetation_model_definition_sha256: &'static str,
    pub lse_configuration_sha256: Sha256Digest,
    pub beginning_lse_state_sha256: Sha256Digest,
    pub vegetation_owner_id: ResourceOwnerId,
    pub ofe_id: OfeId,
    pub tile_id: TileId,
    pub tile_fraction: f64,
    pub interval_s: f64,
    pub canopy_air_temperature_k: f64,
    pub canopy_air_specific_humidity_kg_kg: f64,
    pub top_rain_kg_m2_tile_ground: f64,
    pub ground_canopy_release_kg_m2_tile_ground: f64,
    pub ground_stemflow_kg_m2_tile_ground: f64,
    pub occupancies: Vec<PotentialCoveredOccupancyCarbonOperands>,
    #[serde(skip)]
    payload_sha256: Sha256Digest,
    #[serde(skip)]
    seal: SealedCoveredVegetationOperands,
}

impl PotentialCoveredVegetationOperands {
    pub fn validate(&self) -> Result<(), LandSurfaceEnergyError> {
        if self.pass != CoveredVegetationOperandPass::Potential
            || self.seal != SealedCoveredVegetationOperands::Potential
            || self.transaction_id.0 == 0
            || self.vegetation_model_version != VEGETATION_MODEL_VERSION
            || self.vegetation_model_definition_sha256 != VEGETATION_MODEL_DEFINITION_SHA256
        {
            return Err(LandSurfaceEnergyError::Identity {
                field: "potential vegetation operand identity",
                expected: "sealed OPENWEPP_C3_WOODY_V8 potential pass".into(),
                found: "mismatch".into(),
            });
        }
        if canonical_digest(self)? != self.payload_sha256 {
            return Err(LandSurfaceEnergyError::Identity {
                field: "potential vegetation operand digest",
                expected: self.payload_sha256.to_string(),
                found: canonical_digest(self)?.to_string(),
            });
        }
        if !self.tile_fraction.is_finite()
            || self.tile_fraction <= 0.0
            || self.tile_fraction > 1.0
            || !self.interval_s.is_finite()
            || self.interval_s <= 0.0
            || !(200.0..=350.0).contains(&self.canopy_air_temperature_k)
            || !(0.0..=0.1).contains(&self.canopy_air_specific_humidity_kg_kg)
            || !self.top_rain_kg_m2_tile_ground.is_finite()
            || self.top_rain_kg_m2_tile_ground < 0.0
            || !self.ground_canopy_release_kg_m2_tile_ground.is_finite()
            || self.ground_canopy_release_kg_m2_tile_ground < 0.0
            || !self.ground_stemflow_kg_m2_tile_ground.is_finite()
            || self.ground_stemflow_kg_m2_tile_ground < 0.0
            || self.occupancies.is_empty()
        {
            return Err(LandSurfaceEnergyError::ConstitutiveDomain(
                "potential vegetation shared operand",
            ));
        }
        let mut identities = BTreeSet::new();
        let mut expected_incident = self.top_rain_kg_m2_tile_ground;
        let mut stemflow = 0.0;
        for occupancy in &self.occupancies {
            if !identities.insert(occupancy.occupancy_id.clone()) {
                return Err(LandSurfaceEnergyError::topology_cardinality(
                    "duplicate potential vegetation occupancy",
                ));
            }
            let values = [
                occupancy.sun_leaf_area_m2_m2_tile_ground,
                occupancy.shade_leaf_area_m2_m2_tile_ground,
                occupancy.sun_gross_assimilation_umol_co2_m2_leaf_s,
                occupancy.shade_gross_assimilation_umol_co2_m2_leaf_s,
                occupancy.sun_net_assimilation_umol_co2_m2_leaf_s,
                occupancy.shade_net_assimilation_umol_co2_m2_leaf_s,
                occupancy.sun_dark_respiration_umol_co2_m2_leaf_s,
                occupancy.shade_dark_respiration_umol_co2_m2_leaf_s,
            ];
            if values.iter().any(|value| !value.is_finite()) {
                return Err(LandSurfaceEnergyError::NonFinite(
                    "potential vegetation carbon operand",
                ));
            }
            occupancy.liquid.validate()?;
            if occupancy.liquid.pass != crate::CoveredLiquidPass::Potential {
                return Err(LandSurfaceEnergyError::StateLineage(
                    "potential vegetation liquid pass",
                ));
            }
            if occupancy.liquid.incident_rain_kg_m2_tile.to_bits() != expected_incident.to_bits() {
                return Err(LandSurfaceEnergyError::water_closure(
                    "potential vegetation liquid routing",
                ));
            }
            expected_incident = occupancy.liquid.throughfall_kg_m2_tile
                + occupancy.liquid.initial_drainage_kg_m2_tile
                + occupancy.liquid.second_drainage_kg_m2_tile;
            stemflow += occupancy.liquid.stemflow_kg_m2_tile;
            if values[0] < 0.0
                || values[1] < 0.0
                || values[2] < 0.0
                || values[3] < 0.0
                || values[6] < 0.0
                || values[7] < 0.0
                || values[4].to_bits() != (values[2] - values[6]).to_bits()
                || values[5].to_bits() != (values[3] - values[7]).to_bits()
            {
                return Err(LandSurfaceEnergyError::ConstitutiveDomain(
                    "potential vegetation carbon operand",
                ));
            }
        }
        if expected_incident.to_bits() != self.ground_canopy_release_kg_m2_tile_ground.to_bits()
            || stemflow.to_bits() != self.ground_stemflow_kg_m2_tile_ground.to_bits()
        {
            return Err(LandSurfaceEnergyError::water_closure(
                "potential vegetation ground liquid routing",
            ));
        }
        Ok(())
    }
}

fn root_key(tile: &RuntimeTileIdentity, root: &RootRuntimeIdentity) -> GroundWaterKey {
    GroundWaterKey {
        transaction_id: tile.transaction_id,
        requesting_owner_id: root.requesting_owner_id.clone(),
        requesting_component: RequestingComponent::VegetationRoot,
        ofe_id: tile.ofe_id.clone(),
        requesting_tile_id: tile.tile_id.clone(),
        occupancy_id: Some(root.occupancy_id.clone()),
        surface_id: None,
        surface_class: None,
        source_type: WaterSourceType::SoilLayerLiquid,
        source_id: root.source_id.clone(),
        source_tile_id: None,
        soil_layer_id: Some(root.layer_id.clone()),
        amount_basis: StandGroundWaterAmountBasis::KgH2oM2StandGroundInterval,
    }
}

#[allow(clippy::too_many_lines)]
pub fn solve_covered_potential_phase(
    identity: RuntimeTileIdentity,
    beginning: &CoveredColumnInputs,
    roots: Vec<RootRuntimeIdentity>,
    initial_trial: Vec<f64>,
) -> Result<CoveredPotentialPhase, LandSurfaceEnergyError> {
    identity.validate()?;
    if identity.tile_fraction.to_bits() != beginning.tile_fraction.to_bits()
        || identity.interval_s.to_bits() != beginning.interval_s.to_bits()
    {
        return Err(LandSurfaceEnergyError::Identity {
            field: "covered runtime problem identity",
            expected: "matching tile fraction and interval".into(),
            found: "mismatch".into(),
        });
    }
    let accepted = match solve_covered_column(beginning, None, initial_trial)? {
        CoveredColumnSolveOutcome::Accepted(value) => value,
        CoveredColumnSolveOutcome::Rejected(_) => {
            return Err(LandSurfaceEnergyError::NumericalAcceptedResidual);
        }
    };
    let mut identities = BTreeMap::new();
    let mut vegetation_owner = None;
    for root in roots {
        if root.requesting_owner_id == identity.lse_owner_id {
            return Err(LandSurfaceEnergyError::water_identity(
                "vegetation root owner aliases land-surface-energy owner",
            ));
        }
        if vegetation_owner
            .as_ref()
            .is_some_and(|owner| owner != &root.requesting_owner_id)
        {
            return Err(LandSurfaceEnergyError::water_identity(
                "mixed vegetation root owners",
            ));
        }
        vegetation_owner = Some(root.requesting_owner_id.clone());
        let key = (
            root.solver_occupancy_id.clone(),
            root.layer_id.as_str().to_owned(),
        );
        if root.source_id.as_str() != root.layer_id.as_str()
            || identities.insert(key, root).is_some()
        {
            return Err(LandSurfaceEnergyError::water_cardinality(
                "duplicate or aliased root source identity",
            ));
        }
    }
    let expected: BTreeSet<_> = accepted
        .root_water
        .iter()
        .map(|row| (row.occupancy_id.clone(), row.layer_id.clone()))
        .collect();
    if expected != identities.keys().cloned().collect() {
        return Err(LandSurfaceEnergyError::water_cardinality(
            "root identity set mismatch",
        ));
    }
    let mut requests = Vec::with_capacity(accepted.root_water.len() + 1);
    for row in &accepted.root_water {
        let key = identities
            .get(&(row.occupancy_id.clone(), row.layer_id.clone()))
            .ok_or(LandSurfaceEnergyError::water_cardinality(
                "missing root identity",
            ))?;
        requests.push(WaterAmount {
            key: root_key(&identity, key),
            amount_kg_m2_stand_ground: row.request_kg_m2_stand_ground,
        });
    }
    requests.push(WaterAmount {
        key: identity.ground_key(),
        amount_kg_m2_stand_ground: accepted.ground_water.request_kg_m2_stand_ground,
    });
    let signature = canonical_digest(&PotentialSignature {
        transaction_id: identity.transaction_id,
        beginning_lse_state_sha256: &identity.beginning_lse_state_sha256,
        requests: &requests,
    })?;
    let request_batch = PotentialWaterRequestBatch {
        transaction_id: identity.transaction_id,
        beginning_lse_state_sha256: identity.beginning_lse_state_sha256.clone(),
        requests,
        potential_signature_sha256: signature,
    };
    request_batch.validate()?;
    let vegetation_owner_id = identities
        .values()
        .next()
        .ok_or(LandSurfaceEnergyError::water_cardinality(
            "missing potential vegetation owner",
        ))?
        .requesting_owner_id
        .clone();
    let mut carbon_operands = Vec::with_capacity(beginning.occupancies.len());
    for (input, evaluation) in beginning
        .occupancies
        .iter()
        .zip(&accepted.evaluation.occupancies)
    {
        let runtime = identities
            .iter()
            .find_map(|((solver_occupancy, _), runtime)| {
                (solver_occupancy == &input.occupancy_id).then_some(runtime)
            })
            .ok_or(LandSurfaceEnergyError::water_identity(
                "missing potential vegetation occupancy identity",
            ))?;
        carbon_operands.push(PotentialCoveredOccupancyCarbonOperands {
            occupancy_id: runtime.occupancy_id.clone(),
            sun_leaf_area_m2_m2_tile_ground: input.sun.leaf_area_m2_m2_tile,
            shade_leaf_area_m2_m2_tile_ground: input.shade.leaf_area_m2_m2_tile,
            sun_gross_assimilation_umol_co2_m2_leaf_s: evaluation
                .gross_assimilation_umol_co2_m2_leaf_s[0],
            shade_gross_assimilation_umol_co2_m2_leaf_s: evaluation
                .gross_assimilation_umol_co2_m2_leaf_s[1],
            sun_net_assimilation_umol_co2_m2_leaf_s: evaluation.net_assimilation_umol_co2_m2_leaf_s
                [0],
            shade_net_assimilation_umol_co2_m2_leaf_s: evaluation
                .net_assimilation_umol_co2_m2_leaf_s[1],
            sun_dark_respiration_umol_co2_m2_leaf_s: evaluation.dark_respiration_umol_co2_m2_leaf_s
                [0],
            shade_dark_respiration_umol_co2_m2_leaf_s: evaluation
                .dark_respiration_umol_co2_m2_leaf_s[1],
            liquid: evaluation.liquid,
        });
    }
    let mut potential_vegetation_operands = PotentialCoveredVegetationOperands {
        pass: CoveredVegetationOperandPass::Potential,
        transaction_id: identity.transaction_id,
        vegetation_model_version: VEGETATION_MODEL_VERSION,
        vegetation_model_definition_sha256: VEGETATION_MODEL_DEFINITION_SHA256,
        lse_configuration_sha256: identity.configuration_sha256.clone(),
        beginning_lse_state_sha256: identity.beginning_lse_state_sha256.clone(),
        vegetation_owner_id,
        ofe_id: identity.ofe_id.clone(),
        tile_id: identity.tile_id.clone(),
        tile_fraction: identity.tile_fraction,
        interval_s: identity.interval_s,
        canopy_air_temperature_k: accepted.evaluation.canopy_air_temperature_k,
        canopy_air_specific_humidity_kg_kg: accepted.evaluation.canopy_air_specific_humidity_kg_kg,
        top_rain_kg_m2_tile_ground: beginning.top_rain_kg_m2_tile,
        ground_canopy_release_kg_m2_tile_ground: accepted
            .evaluation
            .ground_canopy_release_kg_m2_tile,
        ground_stemflow_kg_m2_tile_ground: accepted.evaluation.ground_stemflow_kg_m2_tile,
        occupancies: carbon_operands,
        payload_sha256: identity.beginning_lse_state_sha256.clone(),
        seal: SealedCoveredVegetationOperands::Potential,
    };
    potential_vegetation_operands.payload_sha256 =
        canonical_digest(&potential_vegetation_operands)?;
    potential_vegetation_operands.validate()?;
    Ok(CoveredPotentialPhase {
        identity,
        beginning: beginning.clone(),
        accepted,
        request_batch,
        potential_vegetation_operands,
        root_identities: identities,
    })
}

#[derive(Clone, Debug, PartialEq)]
pub struct TileEnergyOperandSet {
    pub surface: SurfaceEnergyOperands,
    pub latent: LatentJoinOperands,
    pub ground_heat: Vec<GroundHeatJoinOperands>,
}

impl TileEnergyOperandSet {
    pub fn validate(&self) -> Result<(), LandSurfaceEnergyError> {
        validate_surface_energy(self.surface)?;
        validate_latent_join(self.latent)?;
        if self.ground_heat.is_empty() {
            return Err(LandSurfaceEnergyError::GroundHeatJoin(
                "empty ground heat receipt set",
            ));
        }
        for join in &self.ground_heat {
            validate_ground_heat_join(*join)?;
        }
        Ok(())
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct SoilThermalLayerCandidate {
    pub layer_id: SoilLayerId,
    pub beginning_enthalpy_j_m2_ofe_ground: f64,
    pub ground_heat_credit_j_m2_ofe_ground: f64,
    pub infiltration_enthalpy_credit_j_m2_ofe_ground: f64,
    pub ending_enthalpy_j_m2_ofe_ground: f64,
    pub ending_temperature_k: f64,
}

#[derive(Clone, Debug, PartialEq)]
pub struct SoilThermalTileCandidate {
    pub owner_id: ResourceOwnerId,
    pub beginning_state_sha256: Sha256Digest,
    pub ofe_id: OfeId,
    pub tile_id: TileId,
    pub layers: Vec<SoilThermalLayerCandidate>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct FinalTileCandidate<C> {
    pub transaction_id: TransactionId,
    pub identity: RuntimeTileIdentity,
    pub final_solver_candidate: C,
    pub water_protocol: WaterProtocol,
    pub ending_tile_state_pre_ingress: TileState,
    pub soil_thermal: SoilThermalTileCandidate,
    pub energy_operands: TileEnergyOperandSet,
    pub diagnostics: NumericalDiagnostics,
    pub rollback_hashes: Vec<OwnerRollbackHash>,
}

fn exact_authorization_map(
    batch: &PotentialWaterRequestBatch,
    authorizations: Vec<WaterAuthorization>,
) -> Result<BTreeMap<GroundWaterKey, WaterAuthorization>, LandSurfaceEnergyError> {
    let requests: BTreeMap<_, _> = batch
        .requests
        .iter()
        .map(|row| (row.key.clone(), row.amount_kg_m2_stand_ground))
        .collect();
    let mut result = BTreeMap::new();
    for authorization in authorizations {
        authorization.key.validate(batch.transaction_id)?;
        let request =
            requests
                .get(&authorization.key)
                .ok_or(LandSurfaceEnergyError::water_cardinality(
                    "authorization without potential request",
                ))?;
        if !authorization.amount_kg_m2_stand_ground.is_finite() {
            return Err(LandSurfaceEnergyError::NonFinite(
                "invalid or duplicate fixed authorization",
            ));
        }
        if authorization.amount_kg_m2_stand_ground < 0.0
            || authorization.amount_kg_m2_stand_ground > *request
        {
            return Err(LandSurfaceEnergyError::water_bound(
                "invalid or duplicate fixed authorization",
            ));
        }
        if result
            .insert(authorization.key.clone(), authorization)
            .is_some()
        {
            return Err(LandSurfaceEnergyError::water_cardinality(
                "invalid or duplicate fixed authorization",
            ));
        }
    }
    if result.len() != requests.len() {
        return Err(LandSurfaceEnergyError::water_cardinality(
            "incomplete fixed authorization set",
        ));
    }
    Ok(result)
}

fn rollback_hashes(identity: &RuntimeTileIdentity) -> Vec<OwnerRollbackHash> {
    [
        (
            OwnerKind::LandSurfaceEnergy,
            identity.lse_owner_id.as_str(),
            &identity.beginning_lse_state_sha256,
        ),
        (
            OwnerKind::Hydrology,
            identity.hydrology_owner_id.as_str(),
            &identity.beginning_hydrology_snapshot_sha256,
        ),
        (
            OwnerKind::SoilThermal,
            identity.soil_thermal_owner_id.as_str(),
            &identity.beginning_soil_thermal_state_sha256,
        ),
        (
            OwnerKind::Vegetation,
            "vegetation",
            &identity.beginning_lse_state_sha256,
        ),
        (
            OwnerKind::Biogeochemistry,
            "biogeochemistry",
            &identity.beginning_lse_state_sha256,
        ),
    ]
    .into_iter()
    .map(|(kind, owner, digest)| OwnerRollbackHash {
        owner_kind: kind,
        owner_id: owner.to_owned(),
        before_sha256: digest.clone(),
        after_sha256: digest.clone(),
    })
    .collect()
}

fn residual_diagnostics(
    raw: &[f64],
    tolerances: &[f64],
    normalized: &[f64],
) -> Vec<NormalizedResidual> {
    raw.iter()
        .zip(tolerances)
        .zip(normalized)
        .enumerate()
        .map(
            |(index, ((raw, tolerance), normalized))| NormalizedResidual {
                identity: format!("ordered_residual_{index}"),
                raw: *raw,
                scale: raw.abs().max(1.0),
                tolerance: *tolerance,
                normalized: *normalized,
                unit: ResidualUnit::Dimensionless,
            },
        )
        .collect()
}

fn accepted_diagnostics(
    identity: &RuntimeTileIdentity,
    solve: SolveIdentity,
    iterations: u32,
    backtracking_count: u32,
    residuals: (&[f64], &[f64], &[f64]),
    step_norms: StepNorms,
    active_caps: Vec<GroundWaterKey>,
) -> Result<NumericalDiagnostics, LandSurfaceEnergyError> {
    let diagnostics = NumericalDiagnostics {
        model_version: MODEL_VERSION.into(),
        canonical_contract: "SC-LANDSURFACEENERGY-001@3".into(),
        model_definition_sha256: Sha256Digest::try_new(MODEL_DEFINITION_SHA256)?,
        configuration_sha256: identity.configuration_sha256.clone(),
        beginning_state_sha256: identity.beginning_lse_state_sha256.clone(),
        transaction_id: identity.transaction_id,
        ofe_id: identity.ofe_id.clone(),
        tile_id: identity.tile_id.clone(),
        occupancy_id: None,
        pass: SolvePass::FinalFixedCap,
        solve,
        accepted: true,
        failure_code: None,
        failure_kind: None,
        iterations,
        backtracking_count,
        ordered_residuals: residual_diagnostics(residuals.0, residuals.1, residuals.2),
        step_norms,
        active_bounds: Vec::new(),
        active_water_caps: active_caps,
        bracket: None,
        pivot_magnitude: None,
        matrix_infinity_norm: None,
        owner_rollback_hashes: rollback_hashes(identity),
    };
    diagnostics.validate()?;
    Ok(diagnostics)
}

pub fn rejected_numerical_diagnostics(
    identity: &RuntimeTileIdentity,
    solve: SolveIdentity,
    failure: &NumericalFailure,
) -> Result<NumericalDiagnostics, LandSurfaceEnergyError> {
    let failure_kind = match failure.kind {
        NumericalFailureKind::SingularPivot => DiagnosticFailureKind::SingularPivot,
        NumericalFailureKind::BacktrackingLimit => DiagnosticFailureKind::BacktrackingLimit,
        NumericalFailureKind::IterationLimit => DiagnosticFailureKind::IterationLimit,
    };
    let normalized: Vec<_> = failure
        .normalized_residuals
        .iter()
        .enumerate()
        .map(|(index, value)| NormalizedResidual {
            identity: format!("ordered_residual_{index}"),
            raw: *value,
            scale: 1.0,
            tolerance: 1.0,
            normalized: *value,
            unit: ResidualUnit::Dimensionless,
        })
        .collect();
    let diagnostics = NumericalDiagnostics {
        model_version: MODEL_VERSION.into(),
        canonical_contract: "SC-LANDSURFACEENERGY-001@3".into(),
        model_definition_sha256: Sha256Digest::try_new(MODEL_DEFINITION_SHA256)?,
        configuration_sha256: identity.configuration_sha256.clone(),
        beginning_state_sha256: identity.beginning_lse_state_sha256.clone(),
        transaction_id: identity.transaction_id,
        ofe_id: identity.ofe_id.clone(),
        tile_id: identity.tile_id.clone(),
        occupancy_id: None,
        pass: SolvePass::FinalFixedCap,
        solve,
        accepted: false,
        failure_code: Some(NumericalFailureCode::LsebE034),
        failure_kind: Some(failure_kind),
        iterations: failure.iterations,
        backtracking_count: failure.backtracking_count,
        ordered_residuals: normalized,
        step_norms: StepNorms {
            temperature_k: failure.step_norm,
            humidity_kg_kg: None,
            ci_pa: None,
            hydraulic_mm: None,
            beta: None,
        },
        active_bounds: Vec::new(),
        active_water_caps: Vec::new(),
        bracket: None,
        pivot_magnitude: failure.pivot_magnitude,
        matrix_infinity_norm: failure.matrix_norm,
        owner_rollback_hashes: rollback_hashes(identity),
    };
    diagnostics.validate()?;
    Ok(diagnostics)
}

pub fn validate_five_owner_envelope(
    identity: &OwnerEnvelopeIdentity,
    expected_configuration_sha256: &Sha256Digest,
) -> Result<(), LandSurfaceEnergyError> {
    identity
        .validate_identity_stage_with_expected_configuration(Some(expected_configuration_sha256))?;
    identity.validate_after_identity_stage()
}

fn build_energy_and_soil(
    identity: &RuntimeTileIdentity,
    _problem: &OpenSurfaceProblem,
    final_value: &AcceptedOpenSurface,
    soil: &SoilThermalSnapshot,
) -> Result<(TileEnergyOperandSet, SoilThermalTileCandidate), LandSurfaceEnergyError> {
    soil.validate()?;
    if soil.owner_id != identity.soil_thermal_owner_id
        || soil.state_sha256 != identity.beginning_soil_thermal_state_sha256
    {
        return Err(LandSurfaceEnergyError::OwnerEnvelope(
            "soil thermal beginning identity mismatch",
        ));
    }
    let beginning_ofe = soil
        .ofes
        .iter()
        .find(|row| row.ofe_id == identity.ofe_id)
        .ok_or(LandSurfaceEnergyError::OwnerEnvelope(
            "missing soil thermal OFE",
        ))?;
    if beginning_ofe.ordered_layers.len() != final_value.evaluation.soil_thermal.len() {
        return Err(LandSurfaceEnergyError::OwnerEnvelope(
            "soil thermal layer cardinality mismatch",
        ));
    }
    let mut joins = Vec::new();
    let mut layers = Vec::new();
    for ((beginning, residual), ending_temperature) in beginning_ofe
        .ordered_layers
        .iter()
        .zip(&final_value.evaluation.soil_thermal)
        .zip(&final_value.evaluation.soil_temperature_k)
    {
        if beginning.layer_id.as_str() != residual.layer_id {
            return Err(LandSurfaceEnergyError::OwnerEnvelope(
                "soil thermal layer order mismatch",
            ));
        }
        let credit = residual.incoming_cn_w_m2 * identity.tile_fraction * identity.interval_s;
        if layers.is_empty() {
            joins.push(GroundHeatJoinOperands {
                surface_debit_j_m2: final_value.evaluation.ground_heat_cn_w_m2_tile[0]
                    * identity.tile_fraction
                    * identity.interval_s,
                soil_credit_j_m2: credit,
            });
        }
        let storage = residual.storage_w_m2 * identity.tile_fraction * identity.interval_s;
        layers.push(SoilThermalLayerCandidate {
            layer_id: beginning.layer_id.clone(),
            beginning_enthalpy_j_m2_ofe_ground: beginning.enthalpy_j_m2_ofe_ground,
            ground_heat_credit_j_m2_ofe_ground: credit,
            infiltration_enthalpy_credit_j_m2_ofe_ground: 0.0,
            ending_enthalpy_j_m2_ofe_ground: beginning.enthalpy_j_m2_ofe_ground + storage,
            ending_temperature_k: *ending_temperature,
        });
    }
    let evaluation = &final_value.evaluation;
    let signed_amount = evaluation.water.final_kg_m2_tile_s * identity.interval_s;
    let energy = TileEnergyOperandSet {
        surface: SurfaceEnergyOperands {
            absorbed_shortwave_w_m2: evaluation.shortwave_absorbed_w_m2_tile.total(),
            net_longwave_w_m2: evaluation.longwave_net_w_m2_tile,
            sensible_w_m2: evaluation.sensible_w_m2_tile,
            signed_vapor_kg_m2_s: evaluation.water.final_kg_m2_tile_s,
            surface_temperature_k: evaluation.surface_temperature_k,
            ground_heat_w_m2: evaluation.ground_heat_cn_w_m2_tile[0],
            storage_w_m2: evaluation.surface_storage_w_m2_tile,
        },
        latent: LatentJoinOperands {
            signed_vapor_kg_m2_s: evaluation.water.final_kg_m2_tile_s,
            interval_s: identity.interval_s,
            surface_temperature_k: evaluation.surface_temperature_k,
            signed_water_amount_kg_m2: signed_amount,
            vapor_energy_j_m2: evaluation.vapor_energy_w_m2_tile * identity.interval_s,
        },
        ground_heat: joins,
    };
    energy.validate()?;
    Ok((
        energy,
        SoilThermalTileCandidate {
            owner_id: soil.owner_id.clone(),
            beginning_state_sha256: soil.state_sha256.clone(),
            ofe_id: identity.ofe_id.clone(),
            tile_id: identity.tile_id.clone(),
            layers,
        },
    ))
}

pub fn finalize_open_phase(
    phase: &OpenPotentialPhase,
    expected_beginning_lse_state_sha256: &Sha256Digest,
    authorization: &WaterAuthorization,
    final_initial_trial: Option<Vec<f64>>,
    soil: &SoilThermalSnapshot,
) -> Result<FinalTileCandidate<AcceptedOpenSurface>, LandSurfaceEnergyError> {
    if expected_beginning_lse_state_sha256 != &phase.identity.beginning_lse_state_sha256 {
        return Err(LandSurfaceEnergyError::StateLineage(
            "stale potential beginning state",
        ));
    }
    let mut authorizations =
        exact_authorization_map(&phase.request_batch, vec![authorization.clone()])?;
    let fixed = authorizations
        .remove(&phase.request_batch.requests[0].key)
        .ok_or(LandSurfaceEnergyError::water_cardinality(
            "missing exact ground authorization",
        ))?;
    let cap_rate = fixed.amount_kg_m2_stand_ground
        / (phase.identity.tile_fraction * phase.identity.interval_s);
    // The retained immutable `beginning` is the only solver input here.
    let final_value =
        match solve_open_surface(&phase.beginning, Some(cap_rate), final_initial_trial)? {
            OpenSurfaceSolveOutcome::Accepted(value) => value,
            OpenSurfaceSolveOutcome::Rejected(_) => {
                return Err(LandSurfaceEnergyError::NumericalAcceptedResidual);
            }
        };
    let finalized = WaterAmount {
        key: phase.request_batch.requests[0].key.clone(),
        amount_kg_m2_stand_ground: if final_value.evaluation.water.branch
            == crate::WaterBranch::AuthorizationActiveOrTie
        {
            fixed.amount_kg_m2_stand_ground
        } else {
            final_value
                .evaluation
                .water
                .finalized_use_kg_m2_stand_ground
        },
    };
    let condensation_credits = condensation_credits(
        &phase.identity,
        final_value
            .evaluation
            .water
            .condensation_credit_kg_m2_stand_ground,
        final_value.evaluation.surface_temperature_k,
    )?;
    let protocol = WaterProtocol {
        transaction_id: phase.identity.transaction_id,
        hydrology_owner_id: phase.identity.hydrology_owner_id.clone(),
        beginning_snapshot_sha256: phase.identity.beginning_hydrology_snapshot_sha256.clone(),
        requests: phase.request_batch.requests.clone(),
        authorizations: vec![fixed],
        finalized_uses: vec![finalized],
        condensation_credits,
    };
    protocol.validate()?;
    let (energy_operands, soil_thermal) =
        build_energy_and_soil(&phase.identity, &phase.beginning, &final_value, soil)?;
    let active_caps =
        if final_value.evaluation.water.branch == crate::WaterBranch::AuthorizationActiveOrTie {
            vec![phase.request_batch.requests[0].key.clone()]
        } else {
            Vec::new()
        };
    let diagnostics = accepted_diagnostics(
        &phase.identity,
        SolveIdentity::SurfaceEnergy,
        final_value.iterations,
        final_value.backtracking_count,
        (
            &final_value.evaluation.raw_residuals,
            &final_value.evaluation.tolerances,
            &final_value.evaluation.normalized_residuals,
        ),
        StepNorms {
            temperature_k: Some(final_value.step_norm),
            humidity_kg_kg: None,
            ci_pa: None,
            hydraulic_mm: None,
            beta: None,
        },
        active_caps,
    )?;
    Ok(FinalTileCandidate {
        transaction_id: phase.identity.transaction_id,
        identity: phase.identity.clone(),
        ending_tile_state_pre_ingress: TileState {
            ofe_id: phase.identity.ofe_id.clone(),
            tile_id: phase.identity.tile_id.clone(),
            surface_enthalpy_j_m2_tile_ground: final_value.candidate.surface_enthalpy_j_m2_tile,
            surface_temperature_warm_start_k: final_value
                .candidate
                .surface_temperature_warm_start_k,
        },
        final_solver_candidate: final_value,
        water_protocol: protocol,
        soil_thermal,
        energy_operands,
        diagnostics,
        rollback_hashes: rollback_hashes(&phase.identity),
    })
}

#[derive(Clone, Debug, PartialEq)]
pub struct FinalCoveredTileCandidate {
    pub transaction_id: TransactionId,
    pub identity: RuntimeTileIdentity,
    pub final_solver_candidate: Box<CoveredColumnCandidate>,
    pub water_protocol: WaterProtocol,
    pub ending_tile_state_pre_ingress: TileState,
    pub soil_thermal: SoilThermalTileCandidate,
    pub energy_operands: TileEnergyOperandSet,
    pub diagnostics: NumericalDiagnostics,
    pub rollback_hashes: Vec<OwnerRollbackHash>,
    /// Dependency-neutral, sealed V8 vegetation operands projected only from
    /// the accepted fixed-cap solve. The LSE crate does not construct or
    /// mutate a vegetation owner candidate.
    pub vegetation_operands: AcceptedCoveredVegetationOperands,
}

/// One exact vegetation-root D/A/F row retained from the shared water
/// protocol. Amounts use the key's stand-ground interval basis.
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct AcceptedRootWaterOperands {
    pub key: GroundWaterKey,
    pub request_kg_m2_stand_ground: f64,
    pub authorization_kg_m2_stand_ground: f64,
    pub finalized_use_kg_m2_stand_ground: f64,
}

/// Accepted V8 state and carbon operands for one exact occupancy.
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct AcceptedCoveredOccupancyOperands {
    pub occupancy_id: ComponentId,
    pub sun_leaf_area_m2_m2_tile_ground: f64,
    pub shade_leaf_area_m2_m2_tile_ground: f64,
    pub sun_leaf_potential_mm: f64,
    pub shade_leaf_potential_mm: f64,
    pub stem_potential_mm: f64,
    pub root_node_potential_mm: f64,
    pub beta_sun: f64,
    pub beta_shade: f64,
    pub sun_emax_kg_m2_tile_s: f64,
    pub shade_emax_kg_m2_tile_s: f64,
    pub beta_hyd: f64,
    pub sun_leaf_temperature_k: f64,
    pub shade_leaf_temperature_k: f64,
    pub wet_surface_temperature_k: f64,
    pub dry_stem_temperature_k: f64,
    pub sun_ci_pa: f64,
    pub shade_ci_pa: f64,
    pub sun_gross_assimilation_umol_co2_m2_leaf_s: f64,
    pub shade_gross_assimilation_umol_co2_m2_leaf_s: f64,
    pub sun_net_assimilation_umol_co2_m2_leaf_s: f64,
    pub shade_net_assimilation_umol_co2_m2_leaf_s: f64,
    pub sun_dark_respiration_umol_co2_m2_leaf_s: f64,
    pub shade_dark_respiration_umol_co2_m2_leaf_s: f64,
    /// Signed accepted wet-surface phase change: positive evaporation,
    /// negative condensation, on tile-ground interval basis.
    pub signed_wet_phase_change_kg_m2_tile_ground: f64,
    pub wet_phase_branch: crate::WaterBranch,
    pub liquid: CoveredOccupancyLiquidLedger,
    pub root_water: Vec<AcceptedRootWaterOperands>,
}

/// Complete accepted vegetation-facing payload for one covered tile.
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct AcceptedCoveredVegetationOperands {
    pub pass: CoveredVegetationOperandPass,
    pub transaction_id: TransactionId,
    pub vegetation_model_version: &'static str,
    pub vegetation_model_definition_sha256: &'static str,
    pub lse_configuration_sha256: Sha256Digest,
    pub beginning_lse_state_sha256: Sha256Digest,
    pub vegetation_owner_id: ResourceOwnerId,
    pub ofe_id: OfeId,
    pub tile_id: TileId,
    pub tile_fraction: f64,
    pub interval_s: f64,
    pub canopy_air_temperature_k: f64,
    pub canopy_air_specific_humidity_kg_kg: f64,
    pub top_rain_kg_m2_tile_ground: f64,
    pub ground_canopy_release_kg_m2_tile_ground: f64,
    pub ground_stemflow_kg_m2_tile_ground: f64,
    pub occupancies: Vec<AcceptedCoveredOccupancyOperands>,
    #[serde(skip)]
    payload_sha256: Sha256Digest,
    #[serde(skip)]
    seal: SealedCoveredVegetationOperands,
}

impl AcceptedCoveredVegetationOperands {
    #[allow(clippy::too_many_lines)]
    pub fn validate(&self) -> Result<(), LandSurfaceEnergyError> {
        if self.pass != CoveredVegetationOperandPass::FixedAuthorizationFinal
            || self.seal != SealedCoveredVegetationOperands::FixedAuthorizationFinal
            || self.transaction_id.0 == 0
        {
            return Err(LandSurfaceEnergyError::StateLineage(
                "invalid accepted vegetation final pass",
            ));
        }
        if canonical_digest(self)? != self.payload_sha256 {
            return Err(LandSurfaceEnergyError::Identity {
                field: "accepted vegetation operand digest",
                expected: self.payload_sha256.to_string(),
                found: canonical_digest(self)?.to_string(),
            });
        }
        if self.vegetation_model_version != VEGETATION_MODEL_VERSION
            || self.vegetation_model_definition_sha256 != VEGETATION_MODEL_DEFINITION_SHA256
        {
            return Err(LandSurfaceEnergyError::Identity {
                field: "accepted vegetation model",
                expected: format!(
                    "{VEGETATION_MODEL_VERSION}/{VEGETATION_MODEL_DEFINITION_SHA256}"
                ),
                found: format!(
                    "{}/{}",
                    self.vegetation_model_version, self.vegetation_model_definition_sha256
                ),
            });
        }
        let shared = [
            self.tile_fraction,
            self.interval_s,
            self.canopy_air_temperature_k,
            self.canopy_air_specific_humidity_kg_kg,
            self.top_rain_kg_m2_tile_ground,
            self.ground_canopy_release_kg_m2_tile_ground,
            self.ground_stemflow_kg_m2_tile_ground,
        ];
        if shared.iter().any(|value| !value.is_finite()) {
            return Err(LandSurfaceEnergyError::NonFinite(
                "accepted vegetation shared operand",
            ));
        }
        if !(0.0..=1.0).contains(&self.tile_fraction)
            || self.tile_fraction == 0.0
            || self.interval_s <= 0.0
            || !(200.0..=350.0).contains(&self.canopy_air_temperature_k)
            || !(0.0..=0.1).contains(&self.canopy_air_specific_humidity_kg_kg)
            || self.top_rain_kg_m2_tile_ground < 0.0
            || self.ground_canopy_release_kg_m2_tile_ground < 0.0
            || self.ground_stemflow_kg_m2_tile_ground < 0.0
            || self.occupancies.is_empty()
        {
            return Err(LandSurfaceEnergyError::ConstitutiveDomain(
                "accepted vegetation shared operand",
            ));
        }
        let mut occupancy_ids = BTreeSet::new();
        let mut expected_incident = self.top_rain_kg_m2_tile_ground;
        let mut stemflow = 0.0;
        for occupancy in &self.occupancies {
            if !occupancy_ids.insert(occupancy.occupancy_id.clone()) {
                return Err(LandSurfaceEnergyError::topology_cardinality(
                    "duplicate accepted vegetation occupancy",
                ));
            }
            let finite = [
                occupancy.sun_leaf_area_m2_m2_tile_ground,
                occupancy.shade_leaf_area_m2_m2_tile_ground,
                occupancy.sun_leaf_potential_mm,
                occupancy.shade_leaf_potential_mm,
                occupancy.stem_potential_mm,
                occupancy.root_node_potential_mm,
                occupancy.beta_sun,
                occupancy.beta_shade,
                occupancy.sun_emax_kg_m2_tile_s,
                occupancy.shade_emax_kg_m2_tile_s,
                occupancy.beta_hyd,
                occupancy.sun_leaf_temperature_k,
                occupancy.shade_leaf_temperature_k,
                occupancy.wet_surface_temperature_k,
                occupancy.dry_stem_temperature_k,
                occupancy.sun_ci_pa,
                occupancy.shade_ci_pa,
                occupancy.sun_gross_assimilation_umol_co2_m2_leaf_s,
                occupancy.shade_gross_assimilation_umol_co2_m2_leaf_s,
                occupancy.sun_net_assimilation_umol_co2_m2_leaf_s,
                occupancy.shade_net_assimilation_umol_co2_m2_leaf_s,
                occupancy.sun_dark_respiration_umol_co2_m2_leaf_s,
                occupancy.shade_dark_respiration_umol_co2_m2_leaf_s,
                occupancy.signed_wet_phase_change_kg_m2_tile_ground,
            ];
            if finite.iter().any(|value| !value.is_finite()) {
                return Err(LandSurfaceEnergyError::NonFinite(
                    "accepted vegetation occupancy operand",
                ));
            }
            occupancy.liquid.validate()?;
            if occupancy.liquid.pass != crate::CoveredLiquidPass::FixedAuthorizationFinal {
                return Err(LandSurfaceEnergyError::StateLineage(
                    "accepted vegetation liquid pass",
                ));
            }
            if occupancy.liquid.incident_rain_kg_m2_tile.to_bits() != expected_incident.to_bits() {
                return Err(LandSurfaceEnergyError::water_closure(
                    "accepted vegetation liquid routing",
                ));
            }
            expected_incident = occupancy.liquid.throughfall_kg_m2_tile
                + occupancy.liquid.initial_drainage_kg_m2_tile
                + occupancy.liquid.second_drainage_kg_m2_tile;
            stemflow += occupancy.liquid.stemflow_kg_m2_tile;
            if occupancy.sun_leaf_area_m2_m2_tile_ground < 0.0
                || occupancy.shade_leaf_area_m2_m2_tile_ground < 0.0
                || !(0.0..=1.0).contains(&occupancy.beta_sun)
                || !(0.0..=1.0).contains(&occupancy.beta_shade)
                || occupancy.sun_emax_kg_m2_tile_s < 0.0
                || occupancy.shade_emax_kg_m2_tile_s < 0.0
                || !(0.0..=1.0).contains(&occupancy.beta_hyd)
                || [
                    occupancy.sun_leaf_temperature_k,
                    occupancy.shade_leaf_temperature_k,
                    occupancy.wet_surface_temperature_k,
                    occupancy.dry_stem_temperature_k,
                ]
                .iter()
                .any(|value| !(200.0..=350.0).contains(value))
                || occupancy.sun_ci_pa <= 0.0
                || occupancy.shade_ci_pa <= 0.0
                || occupancy.sun_gross_assimilation_umol_co2_m2_leaf_s < 0.0
                || occupancy.shade_gross_assimilation_umol_co2_m2_leaf_s < 0.0
                || occupancy.sun_dark_respiration_umol_co2_m2_leaf_s < 0.0
                || occupancy.shade_dark_respiration_umol_co2_m2_leaf_s < 0.0
                || occupancy.root_water.is_empty()
                || match occupancy.wet_phase_branch {
                    crate::WaterBranch::Condensation => {
                        occupancy.signed_wet_phase_change_kg_m2_tile_ground >= 0.0
                    }
                    crate::WaterBranch::ConstitutiveLaw
                    | crate::WaterBranch::AuthorizationActiveOrTie => {
                        occupancy.signed_wet_phase_change_kg_m2_tile_ground < 0.0
                    }
                }
                || occupancy
                    .signed_wet_phase_change_kg_m2_tile_ground
                    .to_bits()
                    != (occupancy.liquid.evaporation_kg_m2_tile
                        - occupancy.liquid.condensation_kg_m2_tile)
                        .to_bits()
            {
                return Err(LandSurfaceEnergyError::ConstitutiveDomain(
                    "accepted vegetation occupancy operand",
                ));
            }
            let maximum = occupancy.sun_emax_kg_m2_tile_s + occupancy.shade_emax_kg_m2_tile_s;
            let expected_beta = if maximum == 0.0 {
                1.0
            } else {
                (occupancy.sun_emax_kg_m2_tile_s * occupancy.beta_sun
                    + occupancy.shade_emax_kg_m2_tile_s * occupancy.beta_shade)
                    / maximum
            };
            if occupancy.beta_hyd.to_bits() != expected_beta.to_bits()
                || occupancy.sun_net_assimilation_umol_co2_m2_leaf_s.to_bits()
                    != (occupancy.sun_gross_assimilation_umol_co2_m2_leaf_s
                        - occupancy.sun_dark_respiration_umol_co2_m2_leaf_s)
                        .to_bits()
                || occupancy
                    .shade_net_assimilation_umol_co2_m2_leaf_s
                    .to_bits()
                    != (occupancy.shade_gross_assimilation_umol_co2_m2_leaf_s
                        - occupancy.shade_dark_respiration_umol_co2_m2_leaf_s)
                        .to_bits()
            {
                return Err(LandSurfaceEnergyError::OwnerEnvelope(
                    "accepted vegetation derived operand mismatch",
                ));
            }
            let mut root_keys = BTreeSet::new();
            for root in &occupancy.root_water {
                root.key.validate(self.transaction_id)?;
                if root.key.requesting_component != RequestingComponent::VegetationRoot
                    || root.key.requesting_owner_id != self.vegetation_owner_id
                    || root.key.occupancy_id.as_ref() != Some(&occupancy.occupancy_id)
                    || !root_keys.insert(root.key.clone())
                {
                    return Err(LandSurfaceEnergyError::water_identity(
                        "accepted vegetation root identity",
                    ));
                }
                if [
                    root.request_kg_m2_stand_ground,
                    root.authorization_kg_m2_stand_ground,
                    root.finalized_use_kg_m2_stand_ground,
                ]
                .iter()
                .any(|value| !value.is_finite())
                {
                    return Err(LandSurfaceEnergyError::NonFinite(
                        "accepted vegetation root amount",
                    ));
                }
                if root.finalized_use_kg_m2_stand_ground < 0.0
                    || root.finalized_use_kg_m2_stand_ground > root.authorization_kg_m2_stand_ground
                    || root.authorization_kg_m2_stand_ground > root.request_kg_m2_stand_ground
                {
                    return Err(LandSurfaceEnergyError::water_bound(
                        "accepted vegetation root D/A/F",
                    ));
                }
            }
        }
        if expected_incident.to_bits() != self.ground_canopy_release_kg_m2_tile_ground.to_bits()
            || stemflow.to_bits() != self.ground_stemflow_kg_m2_tile_ground.to_bits()
        {
            return Err(LandSurfaceEnergyError::water_closure(
                "accepted vegetation ground liquid routing",
            ));
        }
        Ok(())
    }
}

fn build_covered_soil_candidate(
    phase: &CoveredPotentialPhase,
    final_value: &CoveredColumnCandidate,
    soil: &SoilThermalSnapshot,
) -> Result<SoilThermalTileCandidate, LandSurfaceEnergyError> {
    soil.validate()?;
    let identity = &phase.identity;
    if soil.owner_id != identity.soil_thermal_owner_id
        || soil.state_sha256 != identity.beginning_soil_thermal_state_sha256
    {
        return Err(LandSurfaceEnergyError::OwnerEnvelope(
            "covered soil thermal beginning identity mismatch",
        ));
    }
    let beginning_ofe = soil
        .ofes
        .iter()
        .find(|row| row.ofe_id == identity.ofe_id)
        .ok_or(LandSurfaceEnergyError::OwnerEnvelope(
            "missing covered soil thermal OFE",
        ))?;
    let evaluation = &final_value.evaluation;
    if beginning_ofe.ordered_layers.len() != evaluation.soil_temperature_k.len()
        || phase.beginning.ground.soil_nodes.len() != evaluation.soil_temperature_k.len()
    {
        return Err(LandSurfaceEnergyError::OwnerEnvelope(
            "covered soil thermal layer cardinality mismatch",
        ));
    }
    let mut layers = Vec::with_capacity(evaluation.soil_temperature_k.len());
    for (((beginning, node), ending_temperature), incoming) in beginning_ofe
        .ordered_layers
        .iter()
        .zip(&phase.beginning.ground.soil_nodes)
        .zip(&evaluation.soil_temperature_k)
        .zip(&evaluation.ground_heat_cn_w_m2_tile)
    {
        if beginning.layer_id.as_str() != node.layer_id {
            return Err(LandSurfaceEnergyError::OwnerEnvelope(
                "covered soil layer order mismatch",
            ));
        }
        let storage = node.heat_capacity_j_m2_k
            * (*ending_temperature - node.beginning_temperature_k)
            * identity.tile_fraction;
        layers.push(SoilThermalLayerCandidate {
            layer_id: beginning.layer_id.clone(),
            beginning_enthalpy_j_m2_ofe_ground: beginning.enthalpy_j_m2_ofe_ground,
            ground_heat_credit_j_m2_ofe_ground: incoming
                * identity.tile_fraction
                * identity.interval_s,
            infiltration_enthalpy_credit_j_m2_ofe_ground: 0.0,
            ending_enthalpy_j_m2_ofe_ground: beginning.enthalpy_j_m2_ofe_ground + storage,
            ending_temperature_k: *ending_temperature,
        });
    }
    Ok(SoilThermalTileCandidate {
        owner_id: soil.owner_id.clone(),
        beginning_state_sha256: soil.state_sha256.clone(),
        ofe_id: identity.ofe_id.clone(),
        tile_id: identity.tile_id.clone(),
        layers,
    })
}

fn build_covered_energy_operands(
    phase: &CoveredPotentialPhase,
    final_value: &CoveredColumnCandidate,
) -> Result<TileEnergyOperandSet, LandSurfaceEnergyError> {
    let identity = &phase.identity;
    let evaluation = &final_value.evaluation;
    let resistance = under_canopy_neutral_resistance(
        phase.beginning.under_canopy_geometry,
        phase.beginning.reference_wind_m_s,
    )?;
    let rho = phase.beginning.pressure_pa
        / (DRY_AIR_GAS_CONSTANT_J_KG_K * evaluation.canopy_air_temperature_k);
    let ground_sensible = rho
        * AIR_HEAT_CAPACITY_J_KG_K
        * (evaluation.ground_temperature_k - evaluation.canopy_air_temperature_k)
        / resistance.resistance_s_m;
    let signed_amount = evaluation.ground_water.final_kg_m2_tile_s * identity.interval_s;
    let surface = SurfaceEnergyOperands {
        absorbed_shortwave_w_m2: crate::partition_ground_shortwave(
            phase.beginning.ground.terminal_shortwave_w_m2_tile,
            phase.beginning.ground.surface_vis_albedo,
            phase.beginning.ground.surface_nir_albedo,
        )?
        .absorbed
        .total(),
        net_longwave_w_m2: evaluation.whole_column_longwave.ground_net_w_m2,
        sensible_w_m2: ground_sensible,
        signed_vapor_kg_m2_s: evaluation.ground_water.final_kg_m2_tile_s,
        surface_temperature_k: evaluation.ground_temperature_k,
        ground_heat_w_m2: evaluation.ground_heat_cn_w_m2_tile[0],
        storage_w_m2: evaluation.ground_storage_w_m2_tile,
    };
    let ground_heat_amount =
        evaluation.ground_heat_cn_w_m2_tile[0] * identity.tile_fraction * identity.interval_s;
    let operands = TileEnergyOperandSet {
        surface,
        latent: LatentJoinOperands {
            signed_vapor_kg_m2_s: evaluation.ground_water.final_kg_m2_tile_s,
            interval_s: identity.interval_s,
            surface_temperature_k: evaluation.ground_temperature_k,
            signed_water_amount_kg_m2: signed_amount,
            vapor_energy_j_m2: crate::vapor_export_w_m2(
                evaluation.ground_water.final_kg_m2_tile_s,
                evaluation.ground_temperature_k,
            )? * identity.interval_s,
        },
        ground_heat: vec![GroundHeatJoinOperands {
            surface_debit_j_m2: ground_heat_amount,
            soil_credit_j_m2: ground_heat_amount,
        }],
    };
    operands.validate()?;
    Ok(operands)
}

fn build_covered_energy_and_soil(
    phase: &CoveredPotentialPhase,
    final_value: &CoveredColumnCandidate,
    soil: &SoilThermalSnapshot,
) -> Result<(TileEnergyOperandSet, SoilThermalTileCandidate), LandSurfaceEnergyError> {
    Ok((
        build_covered_energy_operands(phase, final_value)?,
        build_covered_soil_candidate(phase, final_value, soil)?,
    ))
}

fn covered_caps_from_authorizations(
    phase: &CoveredPotentialPhase,
    exact: &BTreeMap<GroundWaterKey, WaterAuthorization>,
) -> Result<CoveredWaterCaps, LandSurfaceEnergyError> {
    let denominator = phase.identity.tile_fraction * phase.identity.interval_s;
    let mut root = BTreeMap::new();
    for ((solver_occupancy, solver_layer), runtime) in &phase.root_identities {
        let key = root_key(&phase.identity, runtime);
        let request = phase
            .request_batch
            .requests
            .iter()
            .find(|row| row.key == key)
            .ok_or(LandSurfaceEnergyError::water_cardinality(
                "missing covered potential root request",
            ))?;
        let authorization = exact
            .get(&key)
            .ok_or(LandSurfaceEnergyError::water_cardinality(
                "missing covered root authorization",
            ))?;
        root.insert(
            (solver_occupancy.clone(), solver_layer.clone()),
            SourceWaterCap {
                request_rate_kg_m2_tile_s: request.amount_kg_m2_stand_ground / denominator,
                authorization_rate_kg_m2_tile_s: authorization.amount_kg_m2_stand_ground
                    / denominator,
            },
        );
    }
    let ground_key = phase.identity.ground_key();
    let request = phase
        .request_batch
        .requests
        .iter()
        .find(|row| row.key == ground_key)
        .ok_or(LandSurfaceEnergyError::water_cardinality(
            "missing covered potential ground request",
        ))?;
    let authorization = exact
        .get(&ground_key)
        .ok_or(LandSurfaceEnergyError::water_cardinality(
            "missing covered ground authorization",
        ))?;
    Ok(CoveredWaterCaps {
        root,
        ground: SourceWaterCap {
            request_rate_kg_m2_tile_s: request.amount_kg_m2_stand_ground / denominator,
            authorization_rate_kg_m2_tile_s: authorization.amount_kg_m2_stand_ground / denominator,
        },
    })
}

fn covered_water_protocol(
    phase: &CoveredPotentialPhase,
    final_value: &CoveredColumnCandidate,
    exact: BTreeMap<GroundWaterKey, WaterAuthorization>,
) -> Result<WaterProtocol, LandSurfaceEnergyError> {
    let mut finalized = Vec::with_capacity(final_value.root_water.len() + 1);
    for row in &final_value.root_water {
        let runtime = phase
            .root_identities
            .get(&(row.occupancy_id.clone(), row.layer_id.clone()))
            .ok_or(LandSurfaceEnergyError::water_identity(
                "final covered root identity mismatch",
            ))?;
        let key = root_key(&phase.identity, runtime);
        let amount = if row.branch == crate::WaterBranch::AuthorizationActiveOrTie {
            exact
                .get(&key)
                .ok_or(LandSurfaceEnergyError::water_cardinality(
                    "missing exact active root authorization",
                ))?
                .amount_kg_m2_stand_ground
        } else {
            row.finalized_use_kg_m2_stand_ground
        };
        finalized.push(WaterAmount {
            key,
            amount_kg_m2_stand_ground: amount,
        });
    }
    let ground_key = phase.identity.ground_key();
    let ground_amount =
        if final_value.ground_water.branch == crate::WaterBranch::AuthorizationActiveOrTie {
            exact
                .get(&ground_key)
                .ok_or(LandSurfaceEnergyError::water_cardinality(
                    "missing exact active ground authorization",
                ))?
                .amount_kg_m2_stand_ground
        } else {
            final_value.ground_water.finalized_use_kg_m2_stand_ground
        };
    finalized.push(WaterAmount {
        key: ground_key,
        amount_kg_m2_stand_ground: ground_amount,
    });
    let condensation_credits = condensation_credits(
        &phase.identity,
        final_value
            .ground_water
            .condensation_credit_kg_m2_stand_ground,
        final_value.evaluation.ground_temperature_k,
    )?;
    let protocol = WaterProtocol {
        transaction_id: phase.identity.transaction_id,
        hydrology_owner_id: phase.identity.hydrology_owner_id.clone(),
        beginning_snapshot_sha256: phase.identity.beginning_hydrology_snapshot_sha256.clone(),
        requests: phase.request_batch.requests.clone(),
        authorizations: exact.into_values().collect(),
        finalized_uses: finalized,
        condensation_credits,
    };
    protocol.validate()?;
    Ok(protocol)
}

#[allow(clippy::too_many_lines)]
fn accepted_covered_vegetation_operands(
    phase: &CoveredPotentialPhase,
    final_value: &CoveredColumnCandidate,
    protocol: &WaterProtocol,
) -> Result<AcceptedCoveredVegetationOperands, LandSurfaceEnergyError> {
    if final_value.evaluation.occupancies.len() != phase.beginning.occupancies.len()
        || final_value.solution.len() < 10 * phase.beginning.occupancies.len() + 2
    {
        return Err(LandSurfaceEnergyError::OwnerEnvelope(
            "accepted vegetation final occupancy shape",
        ));
    }
    let requests: BTreeMap<_, _> = protocol
        .requests
        .iter()
        .map(|row| (row.key.clone(), row.amount_kg_m2_stand_ground))
        .collect();
    let authorizations: BTreeMap<_, _> = protocol
        .authorizations
        .iter()
        .map(|row| (row.key.clone(), row.amount_kg_m2_stand_ground))
        .collect();
    let finalized: BTreeMap<_, _> = protocol
        .finalized_uses
        .iter()
        .map(|row| (row.key.clone(), row.amount_kg_m2_stand_ground))
        .collect();
    let mut occupancies = Vec::with_capacity(phase.beginning.occupancies.len());
    for (index, (input, evaluation)) in phase
        .beginning
        .occupancies
        .iter()
        .zip(&final_value.evaluation.occupancies)
        .enumerate()
    {
        let block = &final_value.solution[index * 10..(index + 1) * 10];
        let runtime_identity = phase
            .root_identities
            .iter()
            .find_map(|((solver_occupancy, _), runtime)| {
                (solver_occupancy == &input.occupancy_id).then_some(runtime)
            })
            .ok_or(LandSurfaceEnergyError::water_identity(
                "missing accepted vegetation occupancy identity",
            ))?;
        let mut root_water = Vec::with_capacity(evaluation.source_water.len());
        for source in &evaluation.source_water {
            let runtime = phase
                .root_identities
                .get(&(source.occupancy_id.clone(), source.layer_id.clone()))
                .ok_or(LandSurfaceEnergyError::water_identity(
                    "missing accepted vegetation root identity",
                ))?;
            if runtime.occupancy_id != runtime_identity.occupancy_id {
                return Err(LandSurfaceEnergyError::water_identity(
                    "mixed accepted vegetation occupancy identity",
                ));
            }
            let key = root_key(&phase.identity, runtime);
            root_water.push(AcceptedRootWaterOperands {
                request_kg_m2_stand_ground: *requests.get(&key).ok_or(
                    LandSurfaceEnergyError::water_cardinality(
                        "missing accepted vegetation root request",
                    ),
                )?,
                authorization_kg_m2_stand_ground: *authorizations.get(&key).ok_or(
                    LandSurfaceEnergyError::water_cardinality(
                        "missing accepted vegetation root authorization",
                    ),
                )?,
                finalized_use_kg_m2_stand_ground: *finalized.get(&key).ok_or(
                    LandSurfaceEnergyError::water_cardinality(
                        "missing accepted vegetation root final use",
                    ),
                )?,
                key,
            });
        }
        let maximum = input.emax_sun_kg_m2_s + input.emax_shade_kg_m2_s;
        let beta_hyd = if maximum == 0.0 {
            1.0
        } else {
            (input.emax_sun_kg_m2_s * block[4] + input.emax_shade_kg_m2_s * block[5]) / maximum
        };
        occupancies.push(AcceptedCoveredOccupancyOperands {
            occupancy_id: runtime_identity.occupancy_id.clone(),
            sun_leaf_area_m2_m2_tile_ground: input.sun.leaf_area_m2_m2_tile,
            shade_leaf_area_m2_m2_tile_ground: input.shade.leaf_area_m2_m2_tile,
            sun_leaf_potential_mm: block[0],
            shade_leaf_potential_mm: block[1],
            stem_potential_mm: block[2],
            root_node_potential_mm: block[3],
            beta_sun: block[4],
            beta_shade: block[5],
            sun_emax_kg_m2_tile_s: input.emax_sun_kg_m2_s,
            shade_emax_kg_m2_tile_s: input.emax_shade_kg_m2_s,
            beta_hyd,
            sun_leaf_temperature_k: evaluation.component_temperatures_k[0],
            shade_leaf_temperature_k: evaluation.component_temperatures_k[1],
            wet_surface_temperature_k: evaluation.component_temperatures_k[2],
            dry_stem_temperature_k: evaluation.component_temperatures_k[3],
            sun_ci_pa: evaluation.ci_pa[0],
            shade_ci_pa: evaluation.ci_pa[1],
            sun_gross_assimilation_umol_co2_m2_leaf_s: evaluation
                .gross_assimilation_umol_co2_m2_leaf_s[0],
            shade_gross_assimilation_umol_co2_m2_leaf_s: evaluation
                .gross_assimilation_umol_co2_m2_leaf_s[1],
            sun_net_assimilation_umol_co2_m2_leaf_s: evaluation.net_assimilation_umol_co2_m2_leaf_s
                [0],
            shade_net_assimilation_umol_co2_m2_leaf_s: evaluation
                .net_assimilation_umol_co2_m2_leaf_s[1],
            sun_dark_respiration_umol_co2_m2_leaf_s: evaluation.dark_respiration_umol_co2_m2_leaf_s
                [0],
            shade_dark_respiration_umol_co2_m2_leaf_s: evaluation
                .dark_respiration_umol_co2_m2_leaf_s[1],
            signed_wet_phase_change_kg_m2_tile_ground: evaluation.wet_vapor_kg_m2_s
                * phase.identity.interval_s,
            wet_phase_branch: evaluation.wet_branch,
            liquid: evaluation.liquid,
            root_water,
        });
    }
    let vegetation_owner_id = phase
        .root_identities
        .values()
        .next()
        .ok_or(LandSurfaceEnergyError::water_cardinality(
            "missing accepted vegetation owner",
        ))?
        .requesting_owner_id
        .clone();
    let mut operands = AcceptedCoveredVegetationOperands {
        pass: CoveredVegetationOperandPass::FixedAuthorizationFinal,
        transaction_id: phase.identity.transaction_id,
        vegetation_model_version: VEGETATION_MODEL_VERSION,
        vegetation_model_definition_sha256: VEGETATION_MODEL_DEFINITION_SHA256,
        lse_configuration_sha256: phase.identity.configuration_sha256.clone(),
        beginning_lse_state_sha256: phase.identity.beginning_lse_state_sha256.clone(),
        vegetation_owner_id,
        ofe_id: phase.identity.ofe_id.clone(),
        tile_id: phase.identity.tile_id.clone(),
        tile_fraction: phase.identity.tile_fraction,
        interval_s: phase.identity.interval_s,
        canopy_air_temperature_k: final_value.evaluation.canopy_air_temperature_k,
        canopy_air_specific_humidity_kg_kg: final_value
            .evaluation
            .canopy_air_specific_humidity_kg_kg,
        top_rain_kg_m2_tile_ground: phase.beginning.top_rain_kg_m2_tile,
        ground_canopy_release_kg_m2_tile_ground: final_value
            .evaluation
            .ground_canopy_release_kg_m2_tile,
        ground_stemflow_kg_m2_tile_ground: final_value.evaluation.ground_stemflow_kg_m2_tile,
        occupancies,
        payload_sha256: phase.identity.beginning_lse_state_sha256.clone(),
        seal: SealedCoveredVegetationOperands::FixedAuthorizationFinal,
    };
    operands.payload_sha256 = canonical_digest(&operands)?;
    operands.validate()?;
    Ok(operands)
}

fn condensation_credits(
    identity: &RuntimeTileIdentity,
    amount_kg_m2_stand_ground: f64,
    temperature_k: f64,
) -> Result<Vec<CondensationCredit>, LandSurfaceEnergyError> {
    if !amount_kg_m2_stand_ground.is_finite() {
        return Err(LandSurfaceEnergyError::NonFinite(
            "invalid condensation amount",
        ));
    }
    if amount_kg_m2_stand_ground < 0.0 {
        return Err(LandSurfaceEnergyError::water_bound(
            "invalid condensation amount",
        ));
    }
    if amount_kg_m2_stand_ground == 0.0 {
        return Ok(Vec::new());
    }
    if !matches!(
        identity.ground_source_type,
        WaterSourceType::SurfaceLiquid | WaterSourceType::LitterLiquid
    ) {
        return Err(LandSurfaceEnergyError::water_identity(
            "condensation requires tile surface-liquid source",
        ));
    }
    Ok(vec![CondensationCredit {
        transaction_id: identity.transaction_id,
        hydrology_owner_id: identity.hydrology_owner_id.clone(),
        ofe_id: identity.ofe_id.clone(),
        tile_id: identity.tile_id.clone(),
        surface_id: identity.surface_id.clone(),
        amount_kg_m2_stand_ground,
        amount_basis: StandGroundWaterAmountBasis::KgH2oM2StandGroundInterval,
        temperature_k,
        specific_liquid_enthalpy_j_kg: liquid_enthalpy_j_kg(temperature_k),
    }])
}

fn active_cap_keys(protocol: &WaterProtocol) -> Vec<GroundWaterKey> {
    let authorizations: BTreeMap<_, _> = protocol
        .authorizations
        .iter()
        .map(|row| (row.key.clone(), row.amount_kg_m2_stand_ground))
        .collect();
    protocol
        .finalized_uses
        .iter()
        .filter(|used| {
            authorizations
                .get(&used.key)
                .is_some_and(|amount| used.amount_kg_m2_stand_ground.to_bits() == amount.to_bits())
        })
        .map(|used| used.key.clone())
        .collect()
}

pub fn finalize_covered_phase(
    phase: &CoveredPotentialPhase,
    expected_beginning_lse_state_sha256: &Sha256Digest,
    authorizations: Vec<WaterAuthorization>,
    final_initial_trial: Vec<f64>,
    soil: &SoilThermalSnapshot,
) -> Result<FinalCoveredTileCandidate, LandSurfaceEnergyError> {
    if expected_beginning_lse_state_sha256 != &phase.identity.beginning_lse_state_sha256 {
        return Err(LandSurfaceEnergyError::StateLineage(
            "stale covered potential beginning state",
        ));
    }
    let exact = exact_authorization_map(&phase.request_batch, authorizations)?;
    let caps = covered_caps_from_authorizations(phase, &exact)?;
    // Rebuild from `phase.beginning`; no potential solution enters this call.
    let final_value =
        match solve_covered_column(&phase.beginning, Some(&caps), final_initial_trial)? {
            CoveredColumnSolveOutcome::Accepted(value) => value,
            CoveredColumnSolveOutcome::Rejected(_) => {
                return Err(LandSurfaceEnergyError::NumericalAcceptedResidual);
            }
        };
    let protocol = covered_water_protocol(phase, &final_value, exact)?;
    let vegetation_operands = accepted_covered_vegetation_operands(phase, &final_value, &protocol)?;
    let (energy_operands, soil_thermal) = build_covered_energy_and_soil(phase, &final_value, soil)?;
    let active_caps = active_cap_keys(&protocol);
    let diagnostics = accepted_diagnostics(
        &phase.identity,
        SolveIdentity::JointCanopyGround,
        final_value.iterations,
        final_value.backtracking_count,
        (
            &final_value.evaluation.raw_residuals,
            &final_value.evaluation.tolerances,
            &final_value.evaluation.normalized_residuals,
        ),
        StepNorms {
            temperature_k: Some(final_value.step_norms.temperature_k),
            humidity_kg_kg: Some(final_value.step_norms.humidity_kg_kg),
            ci_pa: Some(final_value.step_norms.ci_pa),
            hydraulic_mm: Some(final_value.step_norms.hydraulic_mm),
            beta: Some(final_value.step_norms.beta),
        },
        active_caps,
    )?;
    Ok(FinalCoveredTileCandidate {
        transaction_id: phase.identity.transaction_id,
        identity: phase.identity.clone(),
        ending_tile_state_pre_ingress: TileState {
            ofe_id: phase.identity.ofe_id.clone(),
            tile_id: phase.identity.tile_id.clone(),
            surface_enthalpy_j_m2_tile_ground: final_value.surface_enthalpy_j_m2_tile,
            surface_temperature_warm_start_k: final_value.evaluation.ground_temperature_k,
        },
        final_solver_candidate: final_value,
        water_protocol: protocol,
        soil_thermal,
        energy_operands,
        diagnostics,
        rollback_hashes: rollback_hashes(&phase.identity),
        vegetation_operands,
    })
}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum PostIngressDisposition {
    RetainedSurface,
    Infiltration,
    RoutedRunoff,
    OutletRunoff,
}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct AdvectedLiquidRecord {
    pub parcel_id: String,
    pub disposition: PostIngressDisposition,
    pub mass_kg_m2_tile_ground: f64,
    pub temperature_k: Option<f64>,
    pub enthalpy_j_m2_tile_ground: f64,
}

impl AdvectedLiquidRecord {
    pub fn validate(&self) -> Result<(), LandSurfaceEnergyError> {
        crate::validate_liquid_parcel(&crate::LiquidParcelOperands {
            parcel_id: self.parcel_id.clone(),
            mass_kg_m2: self.mass_kg_m2_tile_ground,
            temperature_k: self.temperature_k,
            enthalpy_j_m2: self.enthalpy_j_m2_tile_ground,
        })?;
        Ok(())
    }
}

pub fn apply_post_ingress(
    candidate: &mut FinalTileCandidate<AcceptedOpenSurface>,
    records: &[AdvectedLiquidRecord],
) -> Result<(), LandSurfaceEnergyError> {
    let mut identities = BTreeSet::new();
    let mut retained = 0.0;
    let mut infiltration = 0.0;
    for record in records {
        record.validate()?;
        if !identities.insert(record.parcel_id.as_str()) {
            return Err(LandSurfaceEnergyError::OwnerEnvelope(
                "duplicate advected parcel disposition",
            ));
        }
        match record.disposition {
            PostIngressDisposition::RetainedSurface => {
                retained += record.enthalpy_j_m2_tile_ground;
            }
            PostIngressDisposition::Infiltration => {
                infiltration += record.enthalpy_j_m2_tile_ground;
            }
            PostIngressDisposition::RoutedRunoff | PostIngressDisposition::OutletRunoff => {}
        }
    }
    candidate
        .ending_tile_state_pre_ingress
        .surface_enthalpy_j_m2_tile_ground += retained;
    let first =
        candidate
            .soil_thermal
            .layers
            .first_mut()
            .ok_or(LandSurfaceEnergyError::OwnerEnvelope(
                "missing first soil thermal layer",
            ))?;
    first.infiltration_enthalpy_credit_j_m2_ofe_ground +=
        infiltration * candidate.identity.tile_fraction;
    first.ending_enthalpy_j_m2_ofe_ground += infiltration * candidate.identity.tile_fraction;
    Ok(())
}

pub fn build_lse_ending_state(
    beginning: &LandSurfaceEnergyState,
    transaction_id: TransactionId,
    tiles: Vec<TileState>,
) -> Result<LandSurfaceEnergyState, LandSurfaceEnergyError> {
    beginning.validate_transaction_lineage(transaction_id)?;
    let expected: BTreeSet<_> = beginning
        .tiles
        .iter()
        .map(|tile| (tile.ofe_id.clone(), tile.tile_id.clone()))
        .collect();
    let actual: BTreeSet<_> = tiles
        .iter()
        .map(|tile| (tile.ofe_id.clone(), tile.tile_id.clone()))
        .collect();
    if expected != actual || tiles.len() != actual.len() {
        return Err(LandSurfaceEnergyError::topology_cardinality(
            "ending LSE tile identity set",
        ));
    }
    let mut ending = LandSurfaceEnergyState {
        model_definition_sha256: beginning.model_definition_sha256.clone(),
        configuration_sha256: beginning.configuration_sha256.clone(),
        state_sha256: Sha256Digest::try_new("0".repeat(64))?,
        owner_id: beginning.owner_id.clone(),
        last_accepted_transaction_id: Some(transaction_id),
        tiles,
    };
    ending.state_sha256 = ending.canonical_sha256()?;
    ending.validate_schema()?;
    Ok(ending)
}

pub fn specific_liquid_enthalpy(temperature_k: f64) -> Result<f64, LandSurfaceEnergyError> {
    Ok(liquid_enthalpy_j_kg(temperature_k))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        BandDirectionalFluxes, BareSoilParameters, MODEL_DEFINITION_SHA256, OpenNeutralGeometry,
        SoilThermalLayerSnapshot, SoilThermalNodeOperands, SoilThermalOfeSnapshot,
        SurfaceStorageBranch,
    };
    use serde_json::Value;

    fn digest(byte: char) -> Sha256Digest {
        Sha256Digest::try_new(byte.to_string().repeat(64)).expect("test digest")
    }

    fn owner(value: &str) -> ResourceOwnerId {
        ResourceOwnerId::try_new(value).expect("test owner")
    }

    fn layer(value: &str) -> SoilLayerId {
        SoilLayerId::try_new(value).expect("test layer")
    }

    fn problem() -> OpenSurfaceProblem {
        OpenSurfaceProblem {
            interval_s: 1_800.0,
            tile_fraction: 1.0,
            class: SurfaceClassKind::BareMineralSoil,
            storage_branch: SurfaceStorageBranch::FiniteCapacity,
            terminal_shortwave_w_m2_tile: BandDirectionalFluxes {
                direct_vis: 91.0,
                diffuse_vis: 31.0,
                direct_nir: 117.0,
                diffuse_nir: 39.0,
            },
            surface_vis_albedo: 0.18,
            surface_nir_albedo: 0.31,
            surface_emissivity: 1.0,
            surface_depth_m: 0.02,
            surface_conductivity_w_m_k: 0.75,
            surface_dry_heat_capacity_j_m2_k: 42_000.0,
            litter_capacity_kg_m2_tile: None,
            open_geometry: OpenNeutralGeometry {
                reference_height_m: 20.0,
                roughness_momentum_m: 0.12,
                roughness_heat_m: 0.015,
                roughness_vapor_m: 0.010,
            },
            air_temperature_k: 294.0,
            air_specific_humidity_kg_kg: 0.0095,
            air_pressure_pa: 93_000.0,
            reference_wind_m_s: 2.4,
            atmospheric_downward_longwave_w_m2: 335.0,
            surface_liquid_kg_m2_tile: 0.0,
            surface_enthalpy_j_m2_tile: 42_000.0 * (295.0 - crate::REFERENCE_TEMPERATURE_K),
            surface_temperature_warm_start_k: 295.0,
            bare_soil: Some(BareSoilParameters {
                top_layer_liquid_kg_m2: 26.0,
                top_layer_ice_kg_m2: 0.0,
                porosity: 0.46,
                saturated_matric_potential_mm: -120.0,
                clapp_hornberger_b: 4.05,
                theta_initial: 0.22,
            }),
            soil_nodes: (0..4)
                .map(|index| SoilThermalNodeOperands {
                    layer_id: format!("thermal-{}", index + 1),
                    depth_m: 0.08 + 0.05 * f64::from(index),
                    conductivity_w_m_k: 1.1 + 0.12 * f64::from(index),
                    heat_capacity_j_m2_k: 120_000.0 + 35_000.0 * f64::from(index),
                    beginning_temperature_k: 291.5 - 1.1 * f64::from(index),
                })
                .collect(),
        }
    }

    fn identity() -> RuntimeTileIdentity {
        RuntimeTileIdentity {
            transaction_id: TransactionId(41),
            lse_owner_id: owner("lse"),
            hydrology_owner_id: owner("hydrology"),
            soil_thermal_owner_id: owner("soil-thermal"),
            configuration_sha256: digest('a'),
            beginning_lse_state_sha256: digest('b'),
            beginning_hydrology_snapshot_sha256: digest('c'),
            beginning_soil_thermal_state_sha256: digest('d'),
            ofe_id: OfeId::try_new("ofe-1").expect("ofe"),
            tile_id: TileId::try_new("tile-open").expect("tile"),
            surface_id: SurfaceId::try_new("surface-open").expect("surface"),
            surface_class: SurfaceClass::BareMineralSoil,
            ground_source_type: WaterSourceType::SoilLayerLiquid,
            ground_source_id: SourceId::try_new("soil-layer-1").expect("source"),
            ground_source_tile_id: None,
            ground_soil_layer_id: Some(layer("soil-layer-1")),
            tile_fraction: 1.0,
            interval_s: 1_800.0,
        }
    }

    fn soil_snapshot() -> SoilThermalSnapshot {
        SoilThermalSnapshot {
            owner_id: owner("soil-thermal"),
            configuration_sha256: digest('e'),
            state_sha256: digest('d'),
            snapshot_sha256: digest('f'),
            last_accepted_transaction_id: Some(TransactionId(40)),
            ofes: vec![SoilThermalOfeSnapshot {
                ofe_id: OfeId::try_new("ofe-1").expect("ofe"),
                ordered_layers: (0..4)
                    .map(|index| SoilThermalLayerSnapshot {
                        layer_id: layer(&format!("thermal-{}", index + 1)),
                        temperature_k: 291.5 - 1.1 * f64::from(index),
                        enthalpy_j_m2_ofe_ground: 1.0e6 * f64::from(index + 1),
                    })
                    .collect(),
            }],
        }
    }

    fn vectors() -> Value {
        serde_json::from_str(include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/../../docs/work-packages/20260814-snow-free-land-surface-energy-authority-001/artifacts/openwepp_snow_free_lse_v1_vectors.json"
        )))
        .expect("authority vectors")
    }

    #[test]
    fn open_transaction_binds_frozen_vector_and_rebuilds_from_beginning() {
        let beginning = problem();
        let phase = solve_open_potential_phase(identity(), &beginning, None).expect("potential");
        assert_eq!(beginning, problem());
        let request_before = phase.request_batch.requests.clone();
        let authorization = WaterAuthorization {
            key: request_before[0].key.clone(),
            amount_kg_m2_stand_ground: 0.000_053_040_160_893_323_02 * 1_800.0,
            reason: crate::WaterAuthorizationReason::ProportionalSupply,
        };
        let candidate =
            finalize_open_phase(&phase, &digest('b'), &authorization, None, &soil_snapshot())
                .expect("final");
        assert_eq!(candidate.water_protocol.requests, request_before);
        assert_eq!(
            candidate.water_protocol.finalized_uses[0]
                .amount_kg_m2_stand_ground
                .to_bits(),
            authorization.amount_kg_m2_stand_ground.to_bits()
        );
        assert_eq!(candidate.rollback_hashes.len(), 5);
        assert!(
            candidate
                .rollback_hashes
                .iter()
                .all(|row| row.before_sha256 == row.after_sha256)
        );
        let expected = &vectors()["exact_model_reductions"]["open_bare_soil_four_layer"]["fixed_cap_rebuilt_from_beginning"]
            ["solution"];
        for (actual, frozen) in candidate
            .final_solver_candidate
            .solution
            .iter()
            .zip(expected.as_array().expect("solution array"))
        {
            assert!((actual - frozen.as_f64().expect("number")).abs() < 2.0e-10);
        }
        candidate
            .energy_operands
            .validate()
            .expect("independent operands");
    }

    #[test]
    fn stale_potential_and_producer_residual_evasion_fail_closed() {
        let phase = solve_open_potential_phase(identity(), &problem(), None).expect("potential");
        let mut altered_batch = phase.request_batch.clone();
        altered_batch.requests[0].amount_kg_m2_stand_ground = f64::from_bits(
            altered_batch.requests[0]
                .amount_kg_m2_stand_ground
                .to_bits()
                + 1,
        );
        assert!(matches!(
            altered_batch.validate(),
            Err(LandSurfaceEnergyError::Identity {
                field: "potential request batch digest",
                ..
            })
        ));
        let authorization = WaterAuthorization {
            key: phase.request_batch.requests[0].key.clone(),
            amount_kg_m2_stand_ground: phase.request_batch.requests[0].amount_kg_m2_stand_ground,
            reason: crate::WaterAuthorizationReason::FullSupply,
        };
        assert!(matches!(
            finalize_open_phase(&phase, &digest('9'), &authorization, None, &soil_snapshot()),
            Err(LandSurfaceEnergyError::StateLineage(_))
        ));
        let mut candidate =
            finalize_open_phase(&phase, &digest('b'), &authorization, None, &soil_snapshot())
                .expect("final");
        candidate.energy_operands.surface.storage_w_m2 += 1.0;
        assert!(matches!(
            candidate.energy_operands.validate(),
            Err(LandSurfaceEnergyError::ControlVolumeClosure(_))
        ));
    }

    #[test]
    fn post_ingress_carries_exact_enthalpy_to_surface_and_soil() {
        let phase = solve_open_potential_phase(identity(), &problem(), None).expect("potential");
        let authorization = WaterAuthorization {
            key: phase.request_batch.requests[0].key.clone(),
            amount_kg_m2_stand_ground: phase.request_batch.requests[0].amount_kg_m2_stand_ground,
            reason: crate::WaterAuthorizationReason::FullSupply,
        };
        let mut candidate =
            finalize_open_phase(&phase, &digest('b'), &authorization, None, &soil_snapshot())
                .expect("final");
        let surface_before = candidate
            .ending_tile_state_pre_ingress
            .surface_enthalpy_j_m2_tile_ground;
        let soil_before = candidate.soil_thermal.layers[0].ending_enthalpy_j_m2_ofe_ground;
        let records = [
            AdvectedLiquidRecord {
                parcel_id: "rain-retained".into(),
                disposition: PostIngressDisposition::RetainedSurface,
                mass_kg_m2_tile_ground: 0.2,
                temperature_k: Some(285.0),
                enthalpy_j_m2_tile_ground: 0.2 * liquid_enthalpy_j_kg(285.0),
            },
            AdvectedLiquidRecord {
                parcel_id: "rain-infiltration".into(),
                disposition: PostIngressDisposition::Infiltration,
                mass_kg_m2_tile_ground: 0.3,
                temperature_k: Some(286.0),
                enthalpy_j_m2_tile_ground: 0.3 * liquid_enthalpy_j_kg(286.0),
            },
        ];
        apply_post_ingress(&mut candidate, &records).expect("post ingress");
        assert_eq!(
            candidate
                .ending_tile_state_pre_ingress
                .surface_enthalpy_j_m2_tile_ground
                .to_bits(),
            (surface_before + records[0].enthalpy_j_m2_tile_ground).to_bits()
        );
        assert_eq!(
            candidate.soil_thermal.layers[0]
                .ending_enthalpy_j_m2_ofe_ground
                .to_bits(),
            (soil_before + records[1].enthalpy_j_m2_tile_ground).to_bits()
        );
    }

    #[test]
    fn ending_state_digest_binds_transaction_and_every_tile() {
        let beginning = LandSurfaceEnergyState {
            model_definition_sha256: Sha256Digest::try_new(MODEL_DEFINITION_SHA256)
                .expect("model digest"),
            configuration_sha256: digest('a'),
            state_sha256: digest('b'),
            owner_id: owner("lse"),
            last_accepted_transaction_id: Some(TransactionId(40)),
            tiles: vec![TileState {
                ofe_id: OfeId::try_new("ofe-1").expect("ofe"),
                tile_id: TileId::try_new("tile-open").expect("tile"),
                surface_enthalpy_j_m2_tile_ground: 1.0,
                surface_temperature_warm_start_k: 290.0,
            }],
        };
        let ending = build_lse_ending_state(
            &beginning,
            TransactionId(41),
            vec![TileState {
                ofe_id: OfeId::try_new("ofe-1").expect("ofe"),
                tile_id: TileId::try_new("tile-open").expect("tile"),
                surface_enthalpy_j_m2_tile_ground: 2.0,
                surface_temperature_warm_start_k: 291.0,
            }],
        )
        .expect("ending state");
        assert_eq!(ending.last_accepted_transaction_id, Some(TransactionId(41)));
        assert_ne!(ending.state_sha256, beginning.state_sha256);
    }

    #[test]
    fn rejected_diagnostics_carry_complete_exact_rollback_owner_set() {
        let failure = NumericalFailure {
            kind: NumericalFailureKind::BacktrackingLimit,
            iterations: 7,
            normalized_residuals: vec![2.0, -3.0],
            backtracking_count: 20,
            step_norm: Some(1.0e-5),
            pivot_magnitude: Some(2.0e-9),
            matrix_norm: Some(4.0),
        };
        let diagnostics =
            rejected_numerical_diagnostics(&identity(), SolveIdentity::SurfaceEnergy, &failure)
                .expect("typed failure diagnostics");
        assert!(!diagnostics.accepted);
        assert_eq!(
            diagnostics.failure_kind,
            Some(DiagnosticFailureKind::BacktrackingLimit)
        );
        assert_eq!(diagnostics.owner_rollback_hashes.len(), 5);
        assert!(
            diagnostics
                .owner_rollback_hashes
                .iter()
                .all(|row| row.before_sha256 == row.after_sha256)
        );
    }

    #[test]
    fn condensation_receipt_binds_exact_surface_identity_temperature_and_enthalpy() {
        let mut identity = identity();
        identity.ground_source_type = WaterSourceType::SurfaceLiquid;
        identity.ground_source_tile_id = Some(identity.tile_id.clone());
        identity.ground_soil_layer_id = None;
        let credits = condensation_credits(&identity, 0.0125, 281.0).expect("credit");
        assert_eq!(credits.len(), 1);
        let credit = &credits[0];
        assert_eq!(credit.transaction_id, identity.transaction_id);
        assert_eq!(credit.tile_id, identity.tile_id);
        assert_eq!(credit.surface_id, identity.surface_id);
        assert!((credit.amount_kg_m2_stand_ground - 0.0125).abs() < f64::EPSILON);
        assert_eq!(
            credit.specific_liquid_enthalpy_j_kg.to_bits(),
            liquid_enthalpy_j_kg(281.0).to_bits()
        );
    }

    fn accepted_vegetation_fixture() -> AcceptedCoveredVegetationOperands {
        let tile = identity();
        let occupancy_id = ComponentId::try_new("stratum-a@tile-open").expect("occupancy");
        let runtime = RootRuntimeIdentity {
            solver_occupancy_id: "canopy-rank-0".into(),
            requesting_owner_id: owner("vegetation-v8"),
            occupancy_id: occupancy_id.clone(),
            layer_id: layer("soil-layer-1"),
            source_id: SourceId::try_new("soil-layer-1").expect("source"),
        };
        let mut result = AcceptedCoveredVegetationOperands {
            pass: CoveredVegetationOperandPass::FixedAuthorizationFinal,
            transaction_id: tile.transaction_id,
            vegetation_model_version: VEGETATION_MODEL_VERSION,
            vegetation_model_definition_sha256: VEGETATION_MODEL_DEFINITION_SHA256,
            lse_configuration_sha256: tile.configuration_sha256.clone(),
            beginning_lse_state_sha256: tile.beginning_lse_state_sha256.clone(),
            vegetation_owner_id: runtime.requesting_owner_id.clone(),
            ofe_id: tile.ofe_id.clone(),
            tile_id: tile.tile_id.clone(),
            tile_fraction: tile.tile_fraction,
            interval_s: tile.interval_s,
            canopy_air_temperature_k: 295.0,
            canopy_air_specific_humidity_kg_kg: 0.01,
            top_rain_kg_m2_tile_ground: 0.0,
            ground_canopy_release_kg_m2_tile_ground: 0.0,
            ground_stemflow_kg_m2_tile_ground: 0.0,
            occupancies: vec![AcceptedCoveredOccupancyOperands {
                occupancy_id,
                sun_leaf_area_m2_m2_tile_ground: 1.2,
                shade_leaf_area_m2_m2_tile_ground: 0.8,
                sun_leaf_potential_mm: -5_900.0,
                shade_leaf_potential_mm: -5_500.0,
                stem_potential_mm: -4_300.0,
                root_node_potential_mm: -2_850.0,
                beta_sun: 0.5,
                beta_shade: 0.25,
                sun_emax_kg_m2_tile_s: 3.0,
                shade_emax_kg_m2_tile_s: 1.0,
                beta_hyd: 0.4375,
                sun_leaf_temperature_k: 296.0,
                shade_leaf_temperature_k: 295.5,
                wet_surface_temperature_k: 295.2,
                dry_stem_temperature_k: 294.8,
                sun_ci_pa: 28.0,
                shade_ci_pa: 30.0,
                sun_gross_assimilation_umol_co2_m2_leaf_s: 12.0,
                shade_gross_assimilation_umol_co2_m2_leaf_s: 6.0,
                sun_net_assimilation_umol_co2_m2_leaf_s: 11.0,
                shade_net_assimilation_umol_co2_m2_leaf_s: 5.5,
                sun_dark_respiration_umol_co2_m2_leaf_s: 1.0,
                shade_dark_respiration_umol_co2_m2_leaf_s: 0.5,
                signed_wet_phase_change_kg_m2_tile_ground: 0.01,
                wet_phase_branch: crate::WaterBranch::ConstitutiveLaw,
                liquid: CoveredOccupancyLiquidLedger {
                    pass: crate::CoveredLiquidPass::FixedAuthorizationFinal,
                    beginning_store_kg_m2_tile: 0.02,
                    incident_rain_kg_m2_tile: 0.0,
                    ending_store_kg_m2_tile: 0.01,
                    evaporation_kg_m2_tile: 0.01,
                    condensation_kg_m2_tile: 0.0,
                    throughfall_kg_m2_tile: 0.0,
                    stemflow_kg_m2_tile: 0.0,
                    initial_drainage_kg_m2_tile: 0.0,
                    second_drainage_kg_m2_tile: 0.0,
                    wet_fraction: 0.5,
                    wet_surface_temperature_k: 295.2,
                    wet_surface_specific_enthalpy_j_kg: crate::WATER_HEAT_CAPACITY_J_KG_K
                        * (295.2 - crate::REFERENCE_TEMPERATURE_K),
                },
                root_water: vec![AcceptedRootWaterOperands {
                    key: root_key(&tile, &runtime),
                    request_kg_m2_stand_ground: 0.3,
                    authorization_kg_m2_stand_ground: 0.2,
                    finalized_use_kg_m2_stand_ground: 0.15,
                }],
            }],
            payload_sha256: tile.beginning_lse_state_sha256.clone(),
            seal: SealedCoveredVegetationOperands::FixedAuthorizationFinal,
        };
        result.payload_sha256 = canonical_digest(&result).expect("accepted payload digest");
        result
    }

    #[test]
    fn accepted_v8_payload_validates_identity_state_carbon_and_root_daf() {
        let accepted = accepted_vegetation_fixture();
        accepted.validate().expect("accepted V8 operands");
        assert_eq!(
            accepted.occupancies[0]
                .sun_net_assimilation_umol_co2_m2_leaf_s
                .to_bits(),
            (accepted.occupancies[0].sun_gross_assimilation_umol_co2_m2_leaf_s
                - accepted.occupancies[0].sun_dark_respiration_umol_co2_m2_leaf_s)
                .to_bits()
        );
        assert_eq!(
            accepted.occupancies[0].root_water[0]
                .key
                .requesting_owner_id,
            accepted.vegetation_owner_id
        );
    }

    #[test]
    fn accepted_v8_payload_rejects_pass_derived_and_identity_poisons() {
        let mut wrong_pass = accepted_vegetation_fixture();
        wrong_pass.pass = CoveredVegetationOperandPass::Potential;
        assert!(matches!(
            wrong_pass.validate(),
            Err(LandSurfaceEnergyError::StateLineage(_))
        ));

        let mut wrong_carbon = accepted_vegetation_fixture();
        wrong_carbon.occupancies[0].sun_net_assimilation_umol_co2_m2_leaf_s = 12.0;
        assert!(matches!(
            wrong_carbon.validate(),
            Err(LandSurfaceEnergyError::Identity {
                field: "accepted vegetation operand digest",
                ..
            })
        ));

        let mut wrong_owner = accepted_vegetation_fixture();
        wrong_owner.occupancies[0].root_water[0]
            .key
            .requesting_owner_id = owner("other-vegetation");
        assert!(matches!(
            wrong_owner.validate(),
            Err(LandSurfaceEnergyError::Identity {
                field: "accepted vegetation operand digest",
                ..
            })
        ));

        let mut wrong_daf = accepted_vegetation_fixture();
        wrong_daf.occupancies[0].root_water[0].finalized_use_kg_m2_stand_ground = 0.21;
        assert!(matches!(
            wrong_daf.validate(),
            Err(LandSurfaceEnergyError::Identity {
                field: "accepted vegetation operand digest",
                ..
            })
        ));

        let mut stale_potential_liquid = accepted_vegetation_fixture();
        stale_potential_liquid.occupancies[0]
            .liquid
            .ending_store_kg_m2_tile = 0.02;
        stale_potential_liquid.occupancies[0]
            .liquid
            .evaporation_kg_m2_tile = 0.0;
        stale_potential_liquid.occupancies[0].signed_wet_phase_change_kg_m2_tile_ground = 0.0;
        assert!(matches!(
            stale_potential_liquid.validate(),
            Err(LandSurfaceEnergyError::Identity {
                field: "accepted vegetation operand digest",
                ..
            })
        ));

        // Module-local adversarial tests can recompute the payload seal. The
        // independent final-pass and D/A/F validators must still reject
        // producer-consistent but noncanonical payloads.
        let mut sealed_stale_potential_liquid = accepted_vegetation_fixture();
        sealed_stale_potential_liquid.occupancies[0].liquid.pass =
            crate::CoveredLiquidPass::Potential;
        sealed_stale_potential_liquid.payload_sha256 =
            canonical_digest(&sealed_stale_potential_liquid)
                .expect("reseal stale-potential poison");
        assert!(matches!(
            sealed_stale_potential_liquid.validate(),
            Err(LandSurfaceEnergyError::StateLineage(
                "accepted vegetation liquid pass"
            ))
        ));

        let mut sealed_finalized_above_authorization = accepted_vegetation_fixture();
        sealed_finalized_above_authorization.occupancies[0].root_water[0]
            .finalized_use_kg_m2_stand_ground = 0.21;
        sealed_finalized_above_authorization.payload_sha256 =
            canonical_digest(&sealed_finalized_above_authorization).expect("reseal D/A/F poison");
        assert!(matches!(
            sealed_finalized_above_authorization.validate(),
            Err(LandSurfaceEnergyError::WaterIdentityOrBound {
                class: crate::WaterErrorClass::Bound,
                ..
            })
        ));
    }
}
