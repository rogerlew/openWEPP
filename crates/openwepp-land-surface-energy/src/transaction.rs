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
    CoveredCanopyAirEnergyOperands, CoveredColumnCandidate, CoveredColumnEnergyOperands,
    CoveredColumnInputs, CoveredColumnLongwaveOperands, CoveredColumnShortwaveOperands,
    CoveredColumnSolveOutcome, CoveredOccupancyEnergyOperands, CoveredOccupancyLiquidLedger,
    CoveredSurfaceEnergyOperands, CoveredWaterCaps, DRY_AIR_GAS_CONSTANT_J_KG_K,
    DiagnosticFailureKind, GroundHeatJoinOperands, GroundWaterKey, LandSurfaceEnergyError,
    LandSurfaceEnergyState, LatentJoinOperands, MODEL_DEFINITION_SHA256, MODEL_VERSION,
    NormalizedResidual, NumericalDiagnostics, NumericalFailure, NumericalFailureCode,
    NumericalFailureKind, OfeId, OpenSurfaceProblem, OpenSurfaceSolveOutcome,
    OwnerEnvelopeIdentity, OwnerKind, OwnerRollbackHash, RequestingComponent, Sha256Digest,
    SoilThermalSnapshot, SolveIdentity, SolvePass, SourceId, SourceWaterCap,
    StandGroundWaterAmountBasis, StepNorms, SurfaceClass, SurfaceClassKind, SurfaceEnergyOperands,
    SurfaceId, TileState, VEGETATION_MODEL_DEFINITION_SHA256, VEGETATION_MODEL_VERSION,
    WaterAmount, WaterAuthorization, WaterProtocol, WaterSourceType, canonical_digest,
    evaluate_covered_column, evaluate_open_surface, liquid_enthalpy_j_kg, solve_covered_column,
    solve_open_surface,
    solver::{covered_failure_residuals, open_failure_residuals},
    under_canopy_neutral_resistance, validate_ground_heat_join, validate_latent_join,
    validate_surface_energy,
};

#[derive(Clone, Debug, PartialEq)]
pub struct RuntimeTileIdentity {
    pub transaction_id: TransactionId,
    pub lse_owner_id: ResourceOwnerId,
    pub hydrology_owner_id: ResourceOwnerId,
    pub soil_thermal_owner_id: ResourceOwnerId,
    pub vegetation_owner_id: ResourceOwnerId,
    pub biogeochemistry_owner_id: ResourceOwnerId,
    pub configuration_sha256: Sha256Digest,
    pub beginning_lse_state_sha256: Sha256Digest,
    pub beginning_hydrology_snapshot_sha256: Sha256Digest,
    pub beginning_soil_thermal_state_sha256: Sha256Digest,
    pub beginning_vegetation_state_sha256: Sha256Digest,
    pub beginning_biogeochemistry_state_sha256: Sha256Digest,
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
        OpenSurfaceSolveOutcome::Rejected(failure) => {
            return Err(numerical_failure_error(
                &identity,
                SolvePass::Potential,
                SolveIdentity::SurfaceEnergy,
                &failure,
                Vec::new(),
            )?);
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
    identity: RuntimeTileIdentity,
    beginning: CoveredColumnInputs,
    accepted: Box<CoveredColumnCandidate>,
    request_batch: PotentialWaterRequestBatch,
    pub potential_vegetation_operands: PotentialCoveredVegetationOperands,
    root_identities: BTreeMap<(String, String), RootRuntimeIdentity>,
    gas_branches: Vec<[crate::V10LeafGasBranch; 2]>,
}

impl CoveredPotentialPhase {
    #[must_use]
    pub const fn identity(&self) -> &RuntimeTileIdentity {
        &self.identity
    }

    #[must_use]
    pub const fn request_batch(&self) -> &PotentialWaterRequestBatch {
        &self.request_batch
    }
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
        CoveredColumnSolveOutcome::Rejected(failure) => {
            return Err(numerical_failure_error(
                &identity,
                SolvePass::Potential,
                SolveIdentity::JointCanopyGround,
                &failure,
                Vec::new(),
            )?);
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
    let gas_branches = accepted
        .evaluation
        .occupancies
        .iter()
        .map(|occupancy| occupancy.gas_branches)
        .collect();
    Ok(CoveredPotentialPhase {
        identity,
        beginning: beginning.clone(),
        accepted,
        request_batch,
        potential_vegetation_operands,
        root_identities: identities,
        gas_branches,
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

/// Covered-column energy receipt. The ground control volume remains the same
/// independently reconstructed type used by the open path, while `column`
/// makes every canopy surface and the shared canopy-air node mandatory.
#[derive(Clone, Debug, PartialEq)]
pub struct CoveredTileEnergyOperandSet {
    pub ground: TileEnergyOperandSet,
    pub column: CoveredColumnEnergyOperands,
}

impl CoveredTileEnergyOperandSet {
    pub fn validate(&self) -> Result<(), LandSurfaceEnergyError> {
        self.ground.validate()?;
        self.column.validate()?;
        if self.ground.surface.absorbed_shortwave_w_m2.to_bits()
            != self
                .column
                .shortwave
                .ground_absorbed_w_m2_tile
                .total()
                .to_bits()
            || self.ground.surface.sensible_w_m2.to_bits()
                != self
                    .column
                    .canopy_air
                    .ground_sensible_to_canopy_air_w_m2_tile
                    .to_bits()
            || self.ground.surface.signed_vapor_kg_m2_s.to_bits()
                != self
                    .column
                    .canopy_air
                    .ground_vapor_to_canopy_air_kg_m2_tile_s
                    .to_bits()
            || self.ground.surface.net_longwave_w_m2.to_bits()
                != self.column.longwave.ground_net_w_m2_tile.to_bits()
        {
            return Err(LandSurfaceEnergyError::ComponentClosure(
                "covered ground/column energy join",
            ));
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
            identity.vegetation_owner_id.as_str(),
            &identity.beginning_vegetation_state_sha256,
        ),
        (
            OwnerKind::Biogeochemistry,
            identity.biogeochemistry_owner_id.as_str(),
            &identity.beginning_biogeochemistry_state_sha256,
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

fn accepted_diagnostics(
    identity: &RuntimeTileIdentity,
    solve: SolveIdentity,
    iterations: u32,
    backtracking_count: u32,
    ordered_residuals: Vec<NormalizedResidual>,
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
        ordered_residuals,
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
    pass: SolvePass,
    solve: SolveIdentity,
    failure: &NumericalFailure,
    active_water_caps: Vec<GroundWaterKey>,
) -> Result<NumericalDiagnostics, LandSurfaceEnergyError> {
    let failure_kind = match failure.kind {
        NumericalFailureKind::SingularPivot => DiagnosticFailureKind::SingularPivot,
        NumericalFailureKind::BacktrackingLimit => DiagnosticFailureKind::BacktrackingLimit,
        NumericalFailureKind::IterationLimit => DiagnosticFailureKind::IterationLimit,
    };
    if failure.ordered_residuals.is_empty() {
        return Err(LandSurfaceEnergyError::OwnerEnvelope(
            "numerical failure missing typed residual diagnostics",
        ));
    }
    let diagnostics = NumericalDiagnostics {
        model_version: MODEL_VERSION.into(),
        canonical_contract: "SC-LANDSURFACEENERGY-001@3".into(),
        model_definition_sha256: Sha256Digest::try_new(MODEL_DEFINITION_SHA256)?,
        configuration_sha256: identity.configuration_sha256.clone(),
        beginning_state_sha256: identity.beginning_lse_state_sha256.clone(),
        transaction_id: identity.transaction_id,
        ofe_id: identity.ofe_id.clone(),
        tile_id: identity.tile_id.clone(),
        occupancy_id: failure
            .occupancy_id
            .as_deref()
            .map(ComponentId::try_new)
            .transpose()?,
        pass,
        solve,
        accepted: false,
        failure_code: Some(NumericalFailureCode::LsebE034),
        failure_kind: Some(failure_kind),
        iterations: failure.iterations,
        backtracking_count: failure.backtracking_count,
        ordered_residuals: failure.ordered_residuals.clone(),
        step_norms: failure.step_norms.clone(),
        active_bounds: failure.active_bounds.clone(),
        active_water_caps,
        bracket: None,
        pivot_magnitude: failure.pivot_magnitude,
        matrix_infinity_norm: failure.matrix_norm,
        owner_rollback_hashes: rollback_hashes(identity),
    };
    diagnostics.validate()?;
    Ok(diagnostics)
}

fn numerical_failure_error(
    identity: &RuntimeTileIdentity,
    pass: SolvePass,
    solve: SolveIdentity,
    failure: &NumericalFailure,
    active_water_caps: Vec<GroundWaterKey>,
) -> Result<LandSurfaceEnergyError, LandSurfaceEnergyError> {
    let diagnostics = Box::new(rejected_numerical_diagnostics(
        identity,
        pass,
        solve,
        failure,
        active_water_caps,
    )?);
    Ok(match failure.kind {
        NumericalFailureKind::SingularPivot => {
            let pivot = failure
                .pivot_magnitude
                .ok_or(LandSurfaceEnergyError::OwnerEnvelope(
                    "singular failure missing pivot evidence",
                ))?;
            let matrix_norm = failure
                .matrix_norm
                .ok_or(LandSurfaceEnergyError::OwnerEnvelope(
                    "singular failure missing matrix evidence",
                ))?;
            LandSurfaceEnergyError::NumericalSingular {
                pivot,
                matrix_norm,
                diagnostics,
            }
        }
        NumericalFailureKind::BacktrackingLimit => {
            LandSurfaceEnergyError::NumericalBacktrackingLimit { diagnostics }
        }
        NumericalFailureKind::IterationLimit => {
            LandSurfaceEnergyError::NumericalIterationLimit { diagnostics }
        }
    })
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

fn exact_open_cap_rate(
    phase: &OpenPotentialPhase,
    authorization: &WaterAuthorization,
) -> Result<(WaterAuthorization, f64), LandSurfaceEnergyError> {
    let mut authorizations =
        exact_authorization_map(&phase.request_batch, vec![authorization.clone()])?;
    let fixed = authorizations
        .remove(&phase.request_batch.requests[0].key)
        .ok_or(LandSurfaceEnergyError::water_cardinality(
            "missing exact ground authorization",
        ))?;
    let cap_rate = fixed.amount_kg_m2_stand_ground
        / (phase.identity.tile_fraction * phase.identity.interval_s);
    Ok((fixed, cap_rate))
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
    let (fixed, cap_rate) = exact_open_cap_rate(phase, authorization)?;
    // The retained immutable `beginning` is the only solver input here.
    let final_value =
        match solve_open_surface(&phase.beginning, Some(cap_rate), final_initial_trial)? {
            OpenSurfaceSolveOutcome::Accepted(value) => value,
            OpenSurfaceSolveOutcome::Rejected(failure) => {
                let active_caps = rejected_open_active_caps(phase, &failure, cap_rate)?;
                return Err(numerical_failure_error(
                    &phase.identity,
                    SolvePass::FinalFixedCap,
                    SolveIdentity::SurfaceEnergy,
                    &failure,
                    active_caps,
                )?);
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
        open_failure_residuals(&phase.beginning, &final_value.evaluation),
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
    pub energy_operands: CoveredTileEnergyOperandSet,
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

fn covered_occupancy_energy_operands(
    phase: &CoveredPotentialPhase,
    evaluation: &crate::covered_output::CoveredColumnEvaluation,
) -> Result<Vec<CoveredOccupancyEnergyOperands>, LandSurfaceEnergyError> {
    if evaluation.occupancies.len() != phase.beginning.occupancies.len() {
        return Err(LandSurfaceEnergyError::ComponentClosure(
            "covered energy occupancy cardinality",
        ));
    }
    Ok(phase
        .beginning
        .occupancies
        .iter()
        .zip(&evaluation.occupancies)
        .map(|(input, accepted)| {
            let surface = |index| CoveredSurfaceEnergyOperands {
                absorbed_shortwave_w_m2_tile: accepted.absorbed_shortwave_w_m2[index],
                net_longwave_w_m2_tile: accepted.net_longwave_w_m2[index],
                sensible_to_canopy_air_w_m2_tile: accepted.sensible_to_canopy_air_w_m2[index],
                signed_vapor_to_canopy_air_kg_m2_tile_s: accepted
                    .signed_vapor_to_canopy_air_kg_m2_s[index],
                surface_temperature_k: accepted.component_temperatures_k[index],
                latent_heat_j_kg: phase.beginning.latent_heat_j_kg,
            };
            CoveredOccupancyEnergyOperands {
                occupancy_id: input.occupancy_id.clone(),
                sun_leaf: surface(0),
                shade_leaf: surface(1),
                wet_surface: surface(2),
                dry_stem: surface(3),
            }
        })
        .collect())
}

fn build_covered_energy_operands(
    phase: &CoveredPotentialPhase,
    final_value: &CoveredColumnCandidate,
) -> Result<CoveredTileEnergyOperandSet, LandSurfaceEnergyError> {
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
    let occupancies = covered_occupancy_energy_operands(phase, evaluation)?;
    let ground_shortwave = crate::partition_ground_shortwave(
        phase.beginning.ground.terminal_shortwave_w_m2_tile,
        phase.beginning.ground.surface_vis_albedo,
        phase.beginning.ground.surface_nir_albedo,
    )?;
    let longwave = &evaluation.whole_column_longwave;
    let ground = TileEnergyOperandSet {
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
    let operands = CoveredTileEnergyOperandSet {
        ground,
        column: CoveredColumnEnergyOperands {
            occupancies,
            canopy_air: CoveredCanopyAirEnergyOperands {
                canopy_air_temperature_k: evaluation.canopy_air_temperature_k,
                canopy_air_specific_humidity_kg_kg: evaluation.canopy_air_specific_humidity_kg_kg,
                ground_sensible_to_canopy_air_w_m2_tile: evaluation
                    .ground_sensible_to_canopy_air_w_m2,
                ground_vapor_to_canopy_air_kg_m2_tile_s: evaluation.ground_water.final_kg_m2_tile_s,
                sensible_to_reference_air_w_m2_tile: evaluation.sensible_to_reference_air_w_m2,
                vapor_to_reference_air_kg_m2_tile_s: evaluation.vapor_to_reference_air_kg_m2_s,
            },
            shortwave: CoveredColumnShortwaveOperands {
                incident_w_m2_tile: phase.beginning.shortwave.incident_w_m2_tile,
                top_reflected_w_m2_tile: phase.beginning.shortwave.top_reflected_w_m2_tile,
                ground_absorbed_by_incident_w_m2_tile: phase
                    .beginning
                    .shortwave
                    .ground_absorbed_by_incident_w_m2_tile,
                ground_terminal_w_m2_tile: phase.beginning.ground.terminal_shortwave_w_m2_tile,
                ground_absorbed_w_m2_tile: ground_shortwave.absorbed,
                ground_reflected_w_m2_tile: ground_shortwave.reflected,
                occupancies: phase.beginning.shortwave.occupancies.clone(),
            },
            longwave: CoveredColumnLongwaveOperands {
                atmospheric_downward_w_m2_tile: phase.beginning.atmospheric_downward_longwave_w_m2,
                transmissivities: longwave.transmissivities.clone(),
                downward_boundaries_w_m2_tile: longwave.downward_boundaries_w_m2.clone(),
                upward_boundaries_w_m2_tile: longwave.upward_boundaries_w_m2.clone(),
                top_upward_w_m2_tile: longwave.top_upward_w_m2,
                ground_net_w_m2_tile: longwave.ground_net_w_m2,
                occupancy_component_net_w_m2_tile: phase
                    .beginning
                    .occupancies
                    .iter()
                    .zip(&longwave.component_net_w_m2)
                    .map(|(input, values)| (input.occupancy_id.clone(), *values))
                    .collect(),
            },
        },
    };
    operands.validate()?;
    Ok(operands)
}

fn build_covered_energy_and_soil(
    phase: &CoveredPotentialPhase,
    final_value: &CoveredColumnCandidate,
    soil: &SoilThermalSnapshot,
) -> Result<(CoveredTileEnergyOperandSet, SoilThermalTileCandidate), LandSurfaceEnergyError> {
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
        let maximum = evaluation.emax_kg_m2_s[0] + evaluation.emax_kg_m2_s[1];
        let beta_hyd = if maximum == 0.0 {
            1.0
        } else {
            (evaluation.emax_kg_m2_s[0] * block[4] + evaluation.emax_kg_m2_s[1] * block[5])
                / maximum
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
            sun_emax_kg_m2_tile_s: evaluation.emax_kg_m2_s[0],
            shade_emax_kg_m2_tile_s: evaluation.emax_kg_m2_s[1],
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

fn rejected_open_active_caps(
    phase: &OpenPotentialPhase,
    failure: &NumericalFailure,
    cap_rate: f64,
) -> Result<Vec<GroundWaterKey>, LandSurfaceEnergyError> {
    let detail = evaluate_open_surface(
        &phase.beginning,
        &failure.failed_solution,
        Some(cap_rate),
        None,
    )?;
    Ok(
        if detail.water.branch == crate::WaterBranch::AuthorizationActiveOrTie {
            vec![phase.request_batch.requests[0].key.clone()]
        } else {
            Vec::new()
        },
    )
}

fn rejected_covered_active_caps(
    phase: &CoveredPotentialPhase,
    failure: &NumericalFailure,
    caps: &CoveredWaterCaps,
) -> Result<Vec<GroundWaterKey>, LandSurfaceEnergyError> {
    let detail =
        evaluate_covered_column(&phase.beginning, &failure.failed_solution, Some(caps), None)?;
    let mut active = BTreeSet::new();
    if cap_is_active_or_tie(
        detail.ground_water.law_kg_m2_tile_s,
        caps.ground.authorization_rate_kg_m2_tile_s,
    ) {
        active.insert(phase.identity.ground_key());
    }
    for occupancy in &detail.occupancies {
        for source in &occupancy.source_water {
            let cap = caps
                .root
                .get(&(source.occupancy_id.clone(), source.layer_id.clone()))
                .ok_or(LandSurfaceEnergyError::water_cardinality(
                    "missing failed covered root authorization cap",
                ))?;
            if cap_is_active_or_tie(source.law_kg_m2_tile_s, cap.authorization_rate_kg_m2_tile_s) {
                let runtime = phase
                    .root_identities
                    .get(&(source.occupancy_id.clone(), source.layer_id.clone()))
                    .ok_or(LandSurfaceEnergyError::water_identity(
                        "missing failed covered root runtime identity",
                    ))?;
                active.insert(root_key(&phase.identity, runtime));
            }
        }
    }
    Ok(ordered_active_cap_keys(
        &phase.request_batch.requests,
        &active,
    ))
}

fn cap_is_active_or_tie(law_rate: f64, cap_rate: f64) -> bool {
    cap_rate <= law_rate
}

fn ordered_active_cap_keys(
    requests: &[WaterAmount],
    active: &BTreeSet<GroundWaterKey>,
) -> Vec<GroundWaterKey> {
    requests
        .iter()
        .filter(|request| active.contains(&request.key))
        .map(|request| request.key.clone())
        .collect()
}

fn v10_exact_full_supply(
    request_batch: &PotentialWaterRequestBatch,
    exact: &BTreeMap<GroundWaterKey, WaterAuthorization>,
) -> bool {
    request_batch
        .requests
        .iter()
        .all(|request| v10_request_is_exact_full_supply(request, exact))
}

fn v10_request_is_exact_full_supply(
    request: &WaterAmount,
    exact: &BTreeMap<GroundWaterKey, WaterAuthorization>,
) -> bool {
    exact.get(&request.key).is_some_and(|authorization| {
        let amount_equal = authorization.amount_kg_m2_stand_ground.to_bits()
            == request.amount_kg_m2_stand_ground.to_bits();
        let reason_matches = if request.amount_kg_m2_stand_ground == 0.0 {
            matches!(
                authorization.reason,
                crate::WaterAuthorizationReason::ZeroSupply
                    | crate::WaterAuthorizationReason::DrySource
                    | crate::WaterAuthorizationReason::FrozenSource
                    | crate::WaterAuthorizationReason::InaccessibleSource
            )
        } else {
            authorization.reason == crate::WaterAuthorizationReason::FullSupply
        };
        amount_equal && reason_matches
    })
}

fn v10_exact_root_full_supply(
    request_batch: &PotentialWaterRequestBatch,
    exact: &BTreeMap<GroundWaterKey, WaterAuthorization>,
) -> bool {
    request_batch
        .requests
        .iter()
        .filter(|request| {
            request.key.requesting_component == crate::RequestingComponent::VegetationRoot
        })
        .all(|request| v10_request_is_exact_full_supply(request, exact))
}

fn require_v10_exact_full_supply(
    v10_nonpositive_assimilation: bool,
    request_batch: &PotentialWaterRequestBatch,
    exact: &BTreeMap<GroundWaterKey, WaterAuthorization>,
) -> Result<(), LandSurfaceEnergyError> {
    if v10_nonpositive_assimilation && !v10_exact_root_full_supply(request_batch, exact) {
        return Err(LandSurfaceEnergyError::UnsupportedDomain(
            "V10 nonpositive-assimilation partial root authorization",
        ));
    }
    Ok(())
}

fn sealed_v10_nonpositive_assimilation(phase: &CoveredPotentialPhase) -> bool {
    phase.gas_branches.iter().flatten().any(|branch| {
        matches!(
            branch,
            crate::V10LeafGasBranch::ExactZeroPar | crate::V10LeafGasBranch::RespirationDominated
        )
    })
}

fn validate_sealed_gas_branches(
    potential: &[[crate::V10LeafGasBranch; 2]],
    final_value: &[[crate::V10LeafGasBranch; 2]],
) -> Result<(), LandSurfaceEnergyError> {
    if final_value != potential {
        return Err(LandSurfaceEnergyError::water_identity(
            "covered potential/final gas branch receipt",
        ));
    }
    Ok(())
}

fn validate_covered_phase_lineage(
    identity: &RuntimeTileIdentity,
    request_batch: &PotentialWaterRequestBatch,
) -> Result<(), LandSurfaceEnergyError> {
    request_batch.validate()?;
    if request_batch.transaction_id != identity.transaction_id
        || request_batch.beginning_lse_state_sha256 != identity.beginning_lse_state_sha256
    {
        return Err(LandSurfaceEnergyError::water_identity(
            "covered potential phase/request lineage",
        ));
    }
    Ok(())
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
    validate_covered_phase_lineage(&phase.identity, &phase.request_batch)?;
    let exact = exact_authorization_map(&phase.request_batch, authorizations)?;
    let v10_nonpositive_assimilation = sealed_v10_nonpositive_assimilation(phase);
    require_v10_exact_full_supply(v10_nonpositive_assimilation, &phase.request_batch, &exact)?;
    let caps = covered_caps_from_authorizations(phase, &exact)?;
    // Rebuild from `phase.beginning`; no potential solution enters this call.
    let v10_all_sources_full_supply =
        v10_nonpositive_assimilation && v10_exact_full_supply(&phase.request_batch, &exact);
    let selected_initial_trial = if v10_all_sources_full_supply {
        phase.accepted.solution.clone()
    } else {
        final_initial_trial
    };
    let final_value = match if v10_all_sources_full_supply {
        crate::solver::solve_v10_full_supply_final(&phase.beginning, &caps, selected_initial_trial)
    } else {
        solve_covered_column(&phase.beginning, Some(&caps), selected_initial_trial)
    }? {
        CoveredColumnSolveOutcome::Accepted(value) => value,
        CoveredColumnSolveOutcome::Rejected(failure) => {
            let active_caps = rejected_covered_active_caps(phase, &failure, &caps)?;
            return Err(numerical_failure_error(
                &phase.identity,
                SolvePass::FinalFixedCap,
                SolveIdentity::JointCanopyGround,
                &failure,
                active_caps,
            )?);
        }
    };
    let final_gas_branches: Vec<_> = final_value
        .evaluation
        .occupancies
        .iter()
        .map(|occupancy| occupancy.gas_branches)
        .collect();
    validate_sealed_gas_branches(&phase.gas_branches, &final_gas_branches)?;
    let protocol = covered_water_protocol(phase, &final_value, exact)?;
    let vegetation_operands = accepted_covered_vegetation_operands(phase, &final_value, &protocol)?;
    let (energy_operands, soil_thermal) = build_covered_energy_and_soil(phase, &final_value, soil)?;
    let active_caps = active_cap_keys(&protocol);
    let diagnostics = accepted_diagnostics(
        &phase.identity,
        SolveIdentity::JointCanopyGround,
        final_value.iterations,
        final_value.backtracking_count,
        covered_failure_residuals(&phase.beginning, &final_value.evaluation),
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
include!("transaction_tests.rs");
