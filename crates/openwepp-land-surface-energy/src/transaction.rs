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
    CoveredColumnCandidate, CoveredColumnInputs, CoveredColumnSolveOutcome, CoveredWaterCaps,
    DRY_AIR_GAS_CONSTANT_J_KG_K, DiagnosticFailureKind, GroundHeatJoinOperands, GroundWaterKey,
    LandSurfaceEnergyError, LandSurfaceEnergyState, LatentJoinOperands, MODEL_DEFINITION_SHA256,
    MODEL_VERSION, NormalizedResidual, NumericalDiagnostics, NumericalFailure,
    NumericalFailureCode, NumericalFailureKind, OfeId, OpenSurfaceProblem, OpenSurfaceSolveOutcome,
    OwnerEnvelopeIdentity, OwnerKind, OwnerRollbackHash, RequestingComponent, ResidualUnit,
    Sha256Digest, SoilThermalSnapshot, SolveIdentity, SolvePass, SourceId, SourceWaterCap,
    StandGroundWaterAmountBasis, StepNorms, SurfaceClass, SurfaceClassKind, SurfaceEnergyOperands,
    SurfaceId, TileState, WaterAmount, WaterAuthorization, WaterProtocol, WaterSourceType,
    canonical_digest, liquid_enthalpy_j_kg, solve_covered_column, solve_open_surface,
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
    root_identities: BTreeMap<(String, String), RootRuntimeIdentity>,
}

fn root_key(tile: &RuntimeTileIdentity, root: &RootRuntimeIdentity) -> GroundWaterKey {
    GroundWaterKey {
        transaction_id: tile.transaction_id,
        requesting_owner_id: tile.lse_owner_id.clone(),
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
    for root in roots {
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
    Ok(CoveredPotentialPhase {
        identity,
        beginning: beginning.clone(),
        accepted,
        request_batch,
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
    identity.validate()?;
    if &identity.lse_configuration_sha256 != expected_configuration_sha256 {
        return Err(LandSurfaceEnergyError::OwnerEnvelope(
            "five-owner LSE configuration mismatch",
        ));
    }
    Ok(())
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
}
