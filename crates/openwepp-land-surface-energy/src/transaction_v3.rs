//! Atomic candidate construction for the snow-free forest-litter LSE V3 seam.

#![allow(clippy::missing_errors_doc)]

use serde::Serialize;
use std::collections::{BTreeMap, BTreeSet};
use std::time::Duration;

use crate::{
    AcceptedLitterPhaseCandidate, BeginningLitterPhaseState, CoveredColumnInputs, CoveredWaterCaps,
    FinalizedLitterVapor, GroundWaterKey, LandSurfaceEnergyError, LitterPhaseConfiguration,
    LitterPhaseReceipt, LitterPhaseReceiptIdentity, LitterVaporEnvironment, OfeId,
    PotentialCoveredVegetationOperands, Sha256Digest, SoilThermalFinalizationBeginning,
    SourceWaterCap, StandGroundWaterAmountBasis, SurfaceClass, SurfaceClassKind,
    V3_MODEL_DEFINITION_SHA256, V3_MODEL_VERSION, V3_PHASE_RECEIPT_VERSION,
    V3LitterResidualContext, V3PhaseFreeCoveredEvaluation, V3PhaseFreeSurfaceEnergyLedger,
    WaterAmount, WaterAuthorization, WaterProtocol, WaterSourceType, apply_bounded_litter_phase,
    canonical_digest, evaluate_raw_litter_vapor, finalize_litter_vapor, install_finalized_vapor,
    publish_phase_free_litter_vapor, reconstruct_litter_phase_closure,
    solve_v3_phase_free_covered_column, validate_beginning_litter_state,
    validate_litter_phase_configuration,
};
use openwepp_kernel_contract::{ResourceOwnerId, TileId, TransactionId};

use crate::{
    NormalizedSolveOutcome, PotentialWaterRequestBatch, RequestingComponent, RootRuntimeIdentity,
    RuntimeTileIdentity,
};

pub const EXACT_SUPPORT_FLOOR_NS: u128 = 60_000_000_000;

fn support_seconds(duration_ns: u128) -> Result<f64, LandSurfaceEnergyError> {
    let nanoseconds = u64::try_from(duration_ns).map_err(|_| {
        LandSurfaceEnergyError::FrozenLitterTransaction("support duration exceeds u64 nanoseconds")
    })?;
    Ok(Duration::from_nanos(nanoseconds).as_secs_f64())
}

#[derive(Clone, Debug, PartialEq)]
pub struct LitterPhaseTransactionIdentity {
    pub lse_configuration_sha256: Sha256Digest,
    pub transaction_id: TransactionId,
    pub ofe_id: OfeId,
    pub tile_id: TileId,
    pub surface_owner_id: ResourceOwnerId,
    pub beginning_surface_owner_sha256: Sha256Digest,
    pub candidate_surface_owner_sha256: Sha256Digest,
    pub support_start_ns: u128,
    pub support_end_ns: u128,
}

#[derive(Clone, Debug, PartialEq)]
pub struct LitterPhaseTransactionInput {
    pub identity: LitterPhaseTransactionIdentity,
    pub configuration: LitterPhaseConfiguration,
    pub beginning: BeginningLitterPhaseState,
    pub vapor_environment: LitterVaporEnvironment,
    pub finalized_vapor: FinalizedLitterVapor,
    pub phase_free_surface_energy: V3PhaseFreeSurfaceEnergyLedger,
}

#[derive(Serialize)]
struct ReceiptDigestView<'a> {
    identity: &'a LitterPhaseReceiptIdentity,
    configuration: LitterPhaseConfiguration,
    beginning: BeginningLitterPhaseState,
    vapor: crate::LitterVaporReceipt,
    post_vapor: crate::PostVaporLitterState,
    phase_free_surface_energy: V3PhaseFreeSurfaceEnergyLedger,
    transfer: crate::LitterPhaseTransfer,
    ending: crate::EndingLitterPhaseState,
    closure: crate::LitterPhaseClosure,
    same_support_resolve_count: u8,
}

pub fn canonical_litter_phase_receipt_sha256(
    receipt: &LitterPhaseReceipt,
) -> Result<Sha256Digest, LandSurfaceEnergyError> {
    canonical_digest(&ReceiptDigestView {
        identity: &receipt.identity,
        configuration: receipt.configuration,
        beginning: receipt.beginning,
        vapor: receipt.vapor,
        post_vapor: receipt.post_vapor,
        phase_free_surface_energy: receipt.phase_free_surface_energy,
        transfer: receipt.transfer,
        ending: receipt.ending,
        closure: receipt.closure,
        same_support_resolve_count: receipt.same_support_resolve_count,
    })
}

/// Build a complete V3 vapor/phase candidate. No owner is mutated; callers
/// install the candidate and receipt only after their enclosing joins pass.
pub fn execute_litter_phase_v3(
    input: &LitterPhaseTransactionInput,
) -> Result<AcceptedLitterPhaseCandidate, LandSurfaceEnergyError> {
    if input.identity.transaction_id.0 == 0
        || input.identity.support_end_ns <= input.identity.support_start_ns
    {
        return Err(LandSurfaceEnergyError::FrozenLitterTransaction(
            "invalid transaction or support identity",
        ));
    }
    let duration_ns = input.identity.support_end_ns - input.identity.support_start_ns;
    if duration_ns < EXACT_SUPPORT_FLOOR_NS {
        return Err(LandSurfaceEnergyError::SupportBelowMinimum {
            requested_ns: duration_ns,
            minimum_ns: EXACT_SUPPORT_FLOOR_NS,
        });
    }
    let interval_s = support_seconds(duration_ns)?;
    if !interval_s.is_finite() || interval_s <= 0.0 {
        return Err(LandSurfaceEnergyError::FrozenLitterTransaction(
            "support duration conversion",
        ));
    }
    let phase_free = publish_phase_free_litter_vapor(
        input.configuration,
        input.beginning,
        input.vapor_environment,
    )?;
    if phase_free.nonlinear_phase_evaluation_count != 0 {
        return Err(LandSurfaceEnergyError::FrozenLitterTransaction(
            "phase entered nonlinear solve",
        ));
    }
    let vapor = finalize_litter_vapor(
        phase_free.raw_vapor,
        input.finalized_vapor,
        input.beginning,
        phase_free.accepted_surface_temperature_k,
        interval_s,
    )?;
    let post_vapor = install_finalized_vapor(
        input.configuration,
        input.beginning,
        phase_free.accepted_surface_temperature_k,
        vapor,
    )?;
    let (transfer, ending) =
        apply_bounded_litter_phase(input.configuration, post_vapor, interval_s)?;
    let closure = reconstruct_litter_phase_closure(
        input.configuration,
        input.beginning,
        vapor,
        post_vapor,
        input.phase_free_surface_energy,
        interval_s,
        transfer,
        ending,
    )?;
    let identity = LitterPhaseReceiptIdentity {
        receipt_version: V3_PHASE_RECEIPT_VERSION.into(),
        model_version: V3_MODEL_VERSION.into(),
        model_definition_sha256: Sha256Digest::try_new(V3_MODEL_DEFINITION_SHA256)?,
        lse_configuration_sha256: input.identity.lse_configuration_sha256.clone(),
        transaction_id: input.identity.transaction_id,
        ofe_id: input.identity.ofe_id.clone(),
        tile_id: input.identity.tile_id.clone(),
        surface_owner_id: input.identity.surface_owner_id.clone(),
        beginning_surface_owner_sha256: input.identity.beginning_surface_owner_sha256.clone(),
        candidate_surface_owner_sha256: input.identity.candidate_surface_owner_sha256.clone(),
        support_start_ns: input.identity.support_start_ns,
        support_end_ns: input.identity.support_end_ns,
        support_duration_seconds_bits: interval_s.to_bits(),
    };
    let placeholder = Sha256Digest::try_new("0".repeat(64))?;
    let mut receipt = LitterPhaseReceipt {
        identity,
        receipt_sha256: placeholder,
        configuration: input.configuration,
        beginning: input.beginning,
        vapor,
        post_vapor,
        phase_free_surface_energy: input.phase_free_surface_energy,
        transfer,
        ending,
        closure,
        same_support_resolve_count: 0,
    };
    receipt.receipt_sha256 = canonical_litter_phase_receipt_sha256(&receipt)?;
    validate_litter_phase_receipt(&receipt)?;
    Ok(AcceptedLitterPhaseCandidate { ending, receipt })
}

pub fn validate_litter_phase_receipt(
    receipt: &LitterPhaseReceipt,
) -> Result<(), LandSurfaceEnergyError> {
    if receipt.identity.receipt_version != V3_PHASE_RECEIPT_VERSION
        || receipt.identity.model_version != V3_MODEL_VERSION
        || receipt.identity.model_definition_sha256.as_str() != V3_MODEL_DEFINITION_SHA256
        || receipt.identity.transaction_id.0 == 0
        || receipt.identity.support_end_ns <= receipt.identity.support_start_ns
        || receipt.same_support_resolve_count != 0
    {
        return Err(LandSurfaceEnergyError::FrozenLitterTransaction(
            "phase receipt identity or chronology",
        ));
    }
    let duration_ns = receipt.identity.support_end_ns - receipt.identity.support_start_ns;
    if duration_ns < EXACT_SUPPORT_FLOOR_NS
        || receipt.identity.support_duration_seconds_bits != support_seconds(duration_ns)?.to_bits()
    {
        return Err(LandSurfaceEnergyError::FrozenLitterTransaction(
            "phase receipt support",
        ));
    }
    validate_litter_phase_configuration(receipt.configuration)?;
    validate_beginning_litter_state(receipt.configuration, receipt.beginning)?;
    let expected_raw = evaluate_raw_litter_vapor(
        receipt.configuration,
        receipt.beginning,
        receipt.vapor.raw.environment,
    )?;
    if expected_raw != receipt.vapor.raw {
        return Err(LandSurfaceEnergyError::FrozenLitterVapor(
            "raw phase-specific vapor reconstruction",
        ));
    }
    let interval_s = support_seconds(duration_ns)?;
    let expected_vapor = finalize_litter_vapor(
        expected_raw,
        receipt.vapor.finalized,
        receipt.beginning,
        expected_raw.environment.accepted_phase_free_temperature_k,
        interval_s,
    )?;
    if expected_vapor != receipt.vapor {
        return Err(LandSurfaceEnergyError::FrozenLitterVapor(
            "finalized vapor mass-energy reconstruction",
        ));
    }
    let expected_post_vapor = install_finalized_vapor(
        receipt.configuration,
        receipt.beginning,
        expected_raw.environment.accepted_phase_free_temperature_k,
        expected_vapor,
    )?;
    if expected_post_vapor != receipt.post_vapor {
        return Err(LandSurfaceEnergyError::FrozenLitterVapor(
            "post-vapor state reconstruction",
        ));
    }
    let (expected_transfer, expected_ending) =
        apply_bounded_litter_phase(receipt.configuration, expected_post_vapor, interval_s)?;
    if expected_transfer != receipt.transfer || expected_ending != receipt.ending {
        return Err(LandSurfaceEnergyError::FrozenLitterPhaseClosure(
            "bounded transfer or ending-state reconstruction",
        ));
    }
    let expected_closure = reconstruct_litter_phase_closure(
        receipt.configuration,
        receipt.beginning,
        expected_vapor,
        expected_post_vapor,
        receipt.phase_free_surface_energy,
        interval_s,
        expected_transfer,
        expected_ending,
    )?;
    if expected_closure != receipt.closure {
        return Err(LandSurfaceEnergyError::FrozenLitterPhaseClosure(
            "closure operand substitution",
        ));
    }
    let computed = canonical_litter_phase_receipt_sha256(receipt)?;
    if receipt.receipt_sha256 != computed {
        return Err(LandSurfaceEnergyError::FrozenLitterTransaction(
            "phase receipt digest mismatch",
        ));
    }
    Ok(())
}

/// Canonical serialized receipt bytes for restart/checkpoint envelopes.
pub fn litter_phase_receipt_json(
    receipt: &LitterPhaseReceipt,
) -> Result<Vec<u8>, LandSurfaceEnergyError> {
    validate_litter_phase_receipt(receipt)?;
    serde_json::to_vec(receipt)
        .map_err(|error| LandSurfaceEnergyError::MalformedSerialization(error.to_string()))
}

/// Fail closed on absent, stale, mixed or malformed restart receipt bytes.
pub fn litter_phase_receipt_from_json(
    bytes: &[u8],
) -> Result<LitterPhaseReceipt, LandSurfaceEnergyError> {
    let receipt: LitterPhaseReceipt = serde_json::from_slice(bytes)
        .map_err(|error| LandSurfaceEnergyError::MalformedSerialization(error.to_string()))?;
    validate_litter_phase_receipt(&receipt)?;
    Ok(receipt)
}

/// One accepted V3 covered-column solve. The complete V3 evaluation is kept
/// beside the physical solution so callers never need to reconstruct the
/// accepted fixed-final vapor/energy operands with another solve.
#[derive(Clone, Debug, PartialEq)]
pub struct AcceptedV3CoveredSolve {
    pub solution: Vec<f64>,
    pub evaluation: V3PhaseFreeCoveredEvaluation,
    pub iterations: u32,
    pub backtracking_count: u32,
    pub step_norm: f64,
    pub pivot_magnitude: Option<f64>,
    pub matrix_norm: Option<f64>,
}

/// Separately authorized maximum outbound rate for each litter phase.
/// Negative constitutive vapor is not capped: the fixed-final V3 evaluator
/// credits its named phase exactly.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct V3PhaseSpecificVaporAuthorization {
    pub liquid_outbound_rate_kg_m2_s: f64,
    pub ice_outbound_rate_kg_m2_s: f64,
}

impl V3PhaseSpecificVaporAuthorization {
    fn as_solver_authorization(self) -> FinalizedLitterVapor {
        FinalizedLitterVapor {
            liquid_signed_rate_kg_m2_s: self.liquid_outbound_rate_kg_m2_s,
            ice_signed_rate_kg_m2_s: self.ice_outbound_rate_kg_m2_s,
        }
    }

    /// Aggregate only for the existing stand-ground hydrology protocol. The
    /// named rates remain authoritative for phase custody.
    pub fn aggregate_outbound_kg_m2_stand_ground(
        self,
        tile_fraction: f64,
        interval_s: f64,
    ) -> Result<f64, LandSurfaceEnergyError> {
        if !tile_fraction.is_finite()
            || tile_fraction <= 0.0
            || tile_fraction > 1.0
            || !interval_s.is_finite()
            || interval_s <= 0.0
            || !self.liquid_outbound_rate_kg_m2_s.is_finite()
            || self.liquid_outbound_rate_kg_m2_s < 0.0
            || !self.ice_outbound_rate_kg_m2_s.is_finite()
            || self.ice_outbound_rate_kg_m2_s < 0.0
        {
            return Err(LandSurfaceEnergyError::FrozenLitterVapor(
                "phase-specific authorization aggregation domain",
            ));
        }
        let amount = (self.liquid_outbound_rate_kg_m2_s + self.ice_outbound_rate_kg_m2_s)
            * (tile_fraction * interval_s);
        if !amount.is_finite() {
            return Err(LandSurfaceEnergyError::FrozenLitterVapor(
                "phase-specific authorization aggregation overflow",
            ));
        }
        Ok(amount)
    }
}

/// Immutable potential/request result for the V3 covered forest-litter path.
#[derive(Clone, Debug, PartialEq)]
pub struct V3CoveredPotentialPhase {
    identity: RuntimeTileIdentity,
    beginning: CoveredColumnInputs,
    accepted: Box<AcceptedV3CoveredSolve>,
    request_batch: PotentialWaterRequestBatch,
    root_identities: BTreeMap<(String, String), RootRuntimeIdentity>,
    configuration: LitterPhaseConfiguration,
    litter_beginning: BeginningLitterPhaseState,
    pub potential_vegetation_operands: PotentialCoveredVegetationOperands,
}

impl V3CoveredPotentialPhase {
    #[must_use]
    pub const fn identity(&self) -> &RuntimeTileIdentity {
        &self.identity
    }

    #[must_use]
    pub const fn accepted(&self) -> &AcceptedV3CoveredSolve {
        &self.accepted
    }

    #[must_use]
    pub const fn request_batch(&self) -> &PotentialWaterRequestBatch {
        &self.request_batch
    }

    pub(crate) const fn beginning(&self) -> &CoveredColumnInputs {
        &self.beginning
    }

    pub(crate) const fn root_identities(&self) -> &BTreeMap<(String, String), RootRuntimeIdentity> {
        &self.root_identities
    }
}

/// Complete V3 physical envelope corresponding to the legacy covered final
/// candidate, without the V1-only persisted diagnostics schema.
#[derive(Clone, Debug, PartialEq)]
pub struct V3CoveredTileEnergyOperandSet {
    pub authority: crate::CoveredColumnAuthority,
    pub occupancies: Vec<crate::CoveredOccupancyEnergyOperands>,
    pub canopy_air: crate::CoveredCanopyAirEnergyOperands,
    pub shortwave: crate::CoveredColumnShortwaveOperands,
    pub longwave: crate::CoveredColumnLongwaveOperands,
    pub litter_surface: V3PhaseFreeSurfaceEnergyLedger,
}

/// Complete V3 physical envelope corresponding to the legacy covered final
/// candidate, without the V1-only persisted diagnostics schema or its
/// liquid-only lower-boundary energy representation.
#[derive(Clone, Debug, PartialEq)]
pub struct V3CompleteCoveredTileCandidate {
    pub transaction_id: TransactionId,
    pub identity: RuntimeTileIdentity,
    pub final_solver_candidate: Box<crate::CoveredColumnCandidate>,
    pub water_protocol: WaterProtocol,
    pub ending_tile_state_pre_ingress: crate::TileState,
    pub soil_thermal: crate::SoilThermalTileCandidate,
    pub energy_operands: V3CoveredTileEnergyOperandSet,
    pub rollback_hashes: Vec<crate::OwnerRollbackHash>,
    pub vegetation_operands: crate::AcceptedCoveredVegetationOperands,
}

/// Accepted V3 fixed-authorization candidate. `accepted_fixed_final` is the
/// exact object consumed by the post-solve phase coordinator; it is never
/// produced by a predecessor V1/V2 transaction or a same-support re-solve.
#[derive(Clone, Debug, PartialEq)]
pub struct V3FixedFinalCoveredCandidate {
    pub transaction_id: TransactionId,
    pub identity: RuntimeTileIdentity,
    pub accepted_fixed_final: Box<AcceptedV3CoveredSolve>,
    pub potential_request_batch: PotentialWaterRequestBatch,
    pub phase_specific_authorization: V3PhaseSpecificVaporAuthorization,
    pub water_protocol: WaterProtocol,
    pub complete_physical_candidate: V3CompleteCoveredTileCandidate,
}

fn validate_v3_covered_runtime_identity(
    identity: &RuntimeTileIdentity,
    beginning: &CoveredColumnInputs,
) -> Result<(), LandSurfaceEnergyError> {
    let grid_units = identity.interval_s / 60.0;
    if identity.transaction_id.0 == 0
        || !identity.tile_fraction.is_finite()
        || identity.tile_fraction <= 0.0
        || identity.tile_fraction > 1.0
        || !identity.interval_s.is_finite()
        || identity.interval_s < 60.0
        || grid_units.fract() != 0.0
    {
        return Err(LandSurfaceEnergyError::FrozenLitterTransaction(
            "V3 covered runtime identity or 60-second support grid",
        ));
    }
    if identity.tile_fraction.to_bits() != beginning.tile_fraction.to_bits()
        || identity.interval_s.to_bits() != beginning.interval_s.to_bits()
        || identity.surface_class != SurfaceClass::ForestLitter
        || beginning.ground.class != SurfaceClassKind::ForestLitter
        || identity.ground_source_type != WaterSourceType::LitterLiquid
        || identity.ground_source_tile_id.as_ref() != Some(&identity.tile_id)
        || identity.ground_soil_layer_id.is_some()
    {
        return Err(LandSurfaceEnergyError::FrozenLitterV3Identity(
            "V3 covered runtime/beginning/source identity",
        ));
    }
    Ok(())
}

fn v3_ground_key(identity: &RuntimeTileIdentity) -> GroundWaterKey {
    GroundWaterKey {
        transaction_id: identity.transaction_id,
        requesting_owner_id: identity.lse_owner_id.clone(),
        requesting_component: RequestingComponent::GroundSurface,
        ofe_id: identity.ofe_id.clone(),
        requesting_tile_id: identity.tile_id.clone(),
        occupancy_id: None,
        surface_id: Some(identity.surface_id.clone()),
        surface_class: Some(identity.surface_class),
        source_type: identity.ground_source_type,
        source_id: identity.ground_source_id.clone(),
        source_tile_id: identity.ground_source_tile_id.clone(),
        soil_layer_id: identity.ground_soil_layer_id.clone(),
        amount_basis: StandGroundWaterAmountBasis::KgH2oM2StandGroundInterval,
    }
}

fn v3_root_key(identity: &RuntimeTileIdentity, root: &RootRuntimeIdentity) -> GroundWaterKey {
    GroundWaterKey {
        transaction_id: identity.transaction_id,
        requesting_owner_id: root.requesting_owner_id.clone(),
        requesting_component: RequestingComponent::VegetationRoot,
        ofe_id: identity.ofe_id.clone(),
        requesting_tile_id: identity.tile_id.clone(),
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

fn index_v3_root_identities(
    identity: &RuntimeTileIdentity,
    roots: Vec<RootRuntimeIdentity>,
    accepted: &V3PhaseFreeCoveredEvaluation,
) -> Result<BTreeMap<(String, String), RootRuntimeIdentity>, LandSurfaceEnergyError> {
    let mut indexed = BTreeMap::new();
    let mut vegetation_owner = None;
    for root in roots {
        if root.requesting_owner_id == identity.lse_owner_id
            || vegetation_owner
                .as_ref()
                .is_some_and(|owner| owner != &root.requesting_owner_id)
            || root.source_id.as_str() != root.layer_id.as_str()
        {
            return Err(LandSurfaceEnergyError::water_identity(
                "V3 covered root owner/source identity",
            ));
        }
        vegetation_owner = Some(root.requesting_owner_id.clone());
        let key = (
            root.solver_occupancy_id.clone(),
            root.layer_id.as_str().to_owned(),
        );
        if indexed.insert(key, root).is_some() {
            return Err(LandSurfaceEnergyError::water_cardinality(
                "duplicate V3 covered root identity",
            ));
        }
    }
    let expected: BTreeSet<_> = accepted
        .predecessor
        .occupancies
        .iter()
        .flat_map(|occupancy| &occupancy.source_water)
        .map(|source| (source.occupancy_id.clone(), source.layer_id.clone()))
        .collect();
    if expected != indexed.keys().cloned().collect() {
        return Err(LandSurfaceEnergyError::water_cardinality(
            "V3 covered root identity set mismatch",
        ));
    }
    Ok(indexed)
}

fn accepted_v3_solve(
    outcome: NormalizedSolveOutcome<V3PhaseFreeCoveredEvaluation>,
    pass: &'static str,
) -> Result<AcceptedV3CoveredSolve, LandSurfaceEnergyError> {
    match outcome {
        NormalizedSolveOutcome::Accepted {
            solution,
            detail,
            iterations,
            residual_norm_history: _,
            backtracking_count,
            step_norm,
            pivot_magnitude,
            matrix_norm,
        } => Ok(AcceptedV3CoveredSolve {
            solution,
            evaluation: detail,
            iterations,
            backtracking_count,
            step_norm,
            pivot_magnitude,
            matrix_norm,
        }),
        NormalizedSolveOutcome::Rejected(_) => {
            Err(LandSurfaceEnergyError::FrozenLitterTransaction(pass))
        }
    }
}

/// Execute the potential/request V3 solve. Every residual, finite-difference
/// probe, and backtracking evaluation is routed through the V3 phase-free
/// covered evaluator.
pub fn solve_v3_covered_potential_phase(
    identity: RuntimeTileIdentity,
    beginning: &CoveredColumnInputs,
    roots: Vec<RootRuntimeIdentity>,
    initial_trial: &[f64],
    configuration: LitterPhaseConfiguration,
    litter_beginning: BeginningLitterPhaseState,
) -> Result<V3CoveredPotentialPhase, LandSurfaceEnergyError> {
    validate_v3_covered_runtime_identity(&identity, beginning)?;
    validate_litter_phase_configuration(configuration)?;
    validate_beginning_litter_state(configuration, litter_beginning)?;
    let accepted = accepted_v3_solve(
        solve_v3_phase_free_covered_column(
            beginning,
            None,
            initial_trial,
            V3LitterResidualContext {
                configuration,
                beginning: litter_beginning,
                finalized_vapor: None,
            },
        )?,
        "V3 covered potential solve rejected",
    )?;
    let root_identities = index_v3_root_identities(&identity, roots, &accepted.evaluation)?;
    let mut requests = Vec::new();
    for source in accepted
        .evaluation
        .predecessor
        .occupancies
        .iter()
        .flat_map(|occupancy| &occupancy.source_water)
    {
        let runtime = root_identities
            .get(&(source.occupancy_id.clone(), source.layer_id.clone()))
            .ok_or(LandSurfaceEnergyError::water_identity(
                "missing V3 covered root request identity",
            ))?;
        requests.push(WaterAmount {
            key: v3_root_key(&identity, runtime),
            amount_kg_m2_stand_ground: source.request_kg_m2_stand_ground,
        });
    }
    let potential_vapor = accepted.evaluation.vapor.finalized;
    let named_potential_amount = V3PhaseSpecificVaporAuthorization {
        liquid_outbound_rate_kg_m2_s: potential_vapor.liquid_signed_rate_kg_m2_s.max(0.0),
        ice_outbound_rate_kg_m2_s: potential_vapor.ice_signed_rate_kg_m2_s.max(0.0),
    }
    .aggregate_outbound_kg_m2_stand_ground(identity.tile_fraction, identity.interval_s)?;
    requests.push(WaterAmount {
        key: v3_ground_key(&identity),
        amount_kg_m2_stand_ground: accepted
            .evaluation
            .predecessor
            .ground_water
            .request_kg_m2_stand_ground
            .max(named_potential_amount),
    });
    let request_batch = PotentialWaterRequestBatch::try_new(
        identity.transaction_id,
        identity.beginning_lse_state_sha256.clone(),
        requests,
    )?;
    let potential_vegetation_operands =
        crate::transaction::transaction_v3_bridge::build_v3_potential_vegetation_operands(
            &identity,
            beginning,
            &accepted,
            &root_identities,
        )?;
    Ok(V3CoveredPotentialPhase {
        identity,
        beginning: beginning.clone(),
        accepted: Box::new(accepted),
        request_batch,
        root_identities,
        configuration,
        litter_beginning,
        potential_vegetation_operands,
    })
}

fn exact_v3_authorization_map(
    batch: &PotentialWaterRequestBatch,
    authorizations: Vec<WaterAuthorization>,
) -> Result<BTreeMap<GroundWaterKey, WaterAuthorization>, LandSurfaceEnergyError> {
    batch.validate()?;
    let requests: BTreeMap<_, _> = batch
        .requests
        .iter()
        .map(|request| (request.key.clone(), request.amount_kg_m2_stand_ground))
        .collect();
    let mut exact = BTreeMap::new();
    for authorization in authorizations {
        authorization.key.validate(batch.transaction_id)?;
        let request =
            requests
                .get(&authorization.key)
                .ok_or(LandSurfaceEnergyError::water_cardinality(
                    "V3 authorization without potential request",
                ))?;
        if !authorization.amount_kg_m2_stand_ground.is_finite()
            || authorization.amount_kg_m2_stand_ground < 0.0
            || authorization.amount_kg_m2_stand_ground > *request
        {
            return Err(LandSurfaceEnergyError::water_bound(
                "invalid V3 fixed authorization",
            ));
        }
        if exact
            .insert(authorization.key.clone(), authorization)
            .is_some()
        {
            return Err(LandSurfaceEnergyError::water_cardinality(
                "duplicate V3 fixed authorization",
            ));
        }
    }
    if exact.len() != requests.len() {
        return Err(LandSurfaceEnergyError::water_cardinality(
            "incomplete V3 fixed authorization set",
        ));
    }
    Ok(exact)
}

fn v3_caps_from_authorizations(
    phase: &V3CoveredPotentialPhase,
    exact: &BTreeMap<GroundWaterKey, WaterAuthorization>,
) -> Result<CoveredWaterCaps, LandSurfaceEnergyError> {
    let denominator = phase.identity.tile_fraction * phase.identity.interval_s;
    let mut root = BTreeMap::new();
    for ((solver_occupancy, solver_layer), runtime) in &phase.root_identities {
        let key = v3_root_key(&phase.identity, runtime);
        let request = phase
            .request_batch
            .requests
            .iter()
            .find(|row| row.key == key)
            .ok_or(LandSurfaceEnergyError::water_cardinality(
                "missing V3 covered root request",
            ))?;
        let authorization = exact
            .get(&key)
            .ok_or(LandSurfaceEnergyError::water_cardinality(
                "missing V3 covered root authorization",
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
    let ground_key = v3_ground_key(&phase.identity);
    let request = phase
        .request_batch
        .requests
        .iter()
        .find(|row| row.key == ground_key)
        .ok_or(LandSurfaceEnergyError::water_cardinality(
            "missing V3 covered ground request",
        ))?;
    let authorization = exact
        .get(&ground_key)
        .ok_or(LandSurfaceEnergyError::water_cardinality(
            "missing V3 covered ground authorization",
        ))?;
    Ok(CoveredWaterCaps {
        root,
        ground: SourceWaterCap {
            request_rate_kg_m2_tile_s: request.amount_kg_m2_stand_ground / denominator,
            authorization_rate_kg_m2_tile_s: authorization.amount_kg_m2_stand_ground / denominator,
        },
    })
}

fn validate_phase_specific_authorization(
    phase: &V3CoveredPotentialPhase,
    exact: &BTreeMap<GroundWaterKey, WaterAuthorization>,
    authorization: V3PhaseSpecificVaporAuthorization,
) -> Result<(), LandSurfaceEnergyError> {
    let values = [
        authorization.liquid_outbound_rate_kg_m2_s,
        authorization.ice_outbound_rate_kg_m2_s,
    ];
    let potential = phase.accepted.evaluation.vapor.finalized;
    let maxima = [
        potential.liquid_signed_rate_kg_m2_s.max(0.0),
        potential.ice_signed_rate_kg_m2_s.max(0.0),
    ];
    if values
        .iter()
        .zip(maxima)
        .any(|(value, maximum)| !value.is_finite() || *value < 0.0 || *value > maximum)
    {
        return Err(LandSurfaceEnergyError::FrozenLitterVapor(
            "phase-specific authorization exceeds potential named phase",
        ));
    }
    let aggregate = authorization.aggregate_outbound_kg_m2_stand_ground(
        phase.identity.tile_fraction,
        phase.identity.interval_s,
    )?;
    let ground = exact.get(&v3_ground_key(&phase.identity)).ok_or(
        LandSurfaceEnergyError::water_cardinality("missing aggregate V3 ground authorization"),
    )?;
    if aggregate.to_bits() != ground.amount_kg_m2_stand_ground.to_bits() {
        return Err(LandSurfaceEnergyError::FrozenLitterVapor(
            "phase-specific/aggregate authorization join",
        ));
    }
    Ok(())
}

fn validate_accepted_fixed_vapor(
    accepted: &V3PhaseFreeCoveredEvaluation,
    authorization: V3PhaseSpecificVaporAuthorization,
) -> Result<(), LandSurfaceEnergyError> {
    let raw = accepted.vapor.raw;
    let finalized = accepted.vapor.finalized;
    for (raw_rate, final_rate, authorized_rate) in [
        (
            raw.raw_liquid_signed_rate_kg_m2_s,
            finalized.liquid_signed_rate_kg_m2_s,
            authorization.liquid_outbound_rate_kg_m2_s,
        ),
        (
            raw.raw_ice_signed_rate_kg_m2_s,
            finalized.ice_signed_rate_kg_m2_s,
            authorization.ice_outbound_rate_kg_m2_s,
        ),
    ] {
        if (raw_rate < 0.0 && final_rate.to_bits() != raw_rate.to_bits())
            || (raw_rate >= 0.0 && (final_rate < 0.0 || final_rate > authorized_rate))
        {
            return Err(LandSurfaceEnergyError::FrozenLitterVapor(
                "accepted fixed-final phase authorization",
            ));
        }
    }
    Ok(())
}

fn v3_water_protocol(
    phase: &V3CoveredPotentialPhase,
    accepted: &V3PhaseFreeCoveredEvaluation,
    exact: BTreeMap<GroundWaterKey, WaterAuthorization>,
) -> Result<WaterProtocol, LandSurfaceEnergyError> {
    let mut finalized_uses = Vec::new();
    for source in accepted
        .predecessor
        .occupancies
        .iter()
        .flat_map(|occupancy| &occupancy.source_water)
    {
        let runtime = phase
            .root_identities
            .get(&(source.occupancy_id.clone(), source.layer_id.clone()))
            .ok_or(LandSurfaceEnergyError::water_identity(
                "missing accepted V3 root identity",
            ))?;
        finalized_uses.push(WaterAmount {
            key: v3_root_key(&phase.identity, runtime),
            amount_kg_m2_stand_ground: source.finalized_use_kg_m2_stand_ground,
        });
    }
    let accepted_phase_debit = V3PhaseSpecificVaporAuthorization {
        liquid_outbound_rate_kg_m2_s: accepted.vapor.finalized.liquid_signed_rate_kg_m2_s.max(0.0),
        ice_outbound_rate_kg_m2_s: accepted.vapor.finalized.ice_signed_rate_kg_m2_s.max(0.0),
    }
    .aggregate_outbound_kg_m2_stand_ground(
        phase.identity.tile_fraction,
        phase.identity.interval_s,
    )?;
    finalized_uses.push(WaterAmount {
        key: v3_ground_key(&phase.identity),
        amount_kg_m2_stand_ground: accepted_phase_debit,
    });
    let protocol = WaterProtocol {
        transaction_id: phase.identity.transaction_id,
        hydrology_owner_id: phase.identity.hydrology_owner_id.clone(),
        beginning_snapshot_sha256: phase.identity.beginning_hydrology_snapshot_sha256.clone(),
        requests: phase.request_batch.requests.clone(),
        authorizations: exact.into_values().collect(),
        finalized_uses,
        // Named liquid condensation and ice deposition remain in the sealed
        // V3 evaluation. Aggregating them into the V1 liquid-only credit type
        // would erase phase custody.
        condensation_credits: Vec::new(),
    };
    protocol.validate()?;
    Ok(protocol)
}

/// Execute exactly one fixed-authorization V3 solve from the immutable
/// beginning retained by `phase`. The accepted V3 evaluation is returned
/// intact for the post-solve phase/owner coordinator.
pub fn finalize_v3_covered_phase(
    phase: &V3CoveredPotentialPhase,
    expected_beginning_lse_state_sha256: &Sha256Digest,
    authorizations: Vec<WaterAuthorization>,
    phase_specific_authorization: V3PhaseSpecificVaporAuthorization,
    final_initial_trial: &[f64],
    soil: SoilThermalFinalizationBeginning<'_>,
) -> Result<V3FixedFinalCoveredCandidate, LandSurfaceEnergyError> {
    if expected_beginning_lse_state_sha256 != &phase.identity.beginning_lse_state_sha256 {
        return Err(LandSurfaceEnergyError::StateLineage(
            "stale V3 covered potential beginning state",
        ));
    }
    phase.request_batch.validate()?;
    let exact = exact_v3_authorization_map(&phase.request_batch, authorizations)?;
    validate_phase_specific_authorization(phase, &exact, phase_specific_authorization)?;
    let caps = v3_caps_from_authorizations(phase, &exact)?;
    let accepted = accepted_v3_solve(
        solve_v3_phase_free_covered_column(
            &phase.beginning,
            Some(&caps),
            final_initial_trial,
            V3LitterResidualContext {
                configuration: phase.configuration,
                beginning: phase.litter_beginning,
                finalized_vapor: Some(phase_specific_authorization.as_solver_authorization()),
            },
        )?,
        "V3 covered fixed-final solve rejected",
    )?;
    validate_accepted_fixed_vapor(&accepted.evaluation, phase_specific_authorization)?;
    let water_protocol = v3_water_protocol(phase, &accepted.evaluation, exact)?;
    let complete_physical_candidate =
        crate::transaction::transaction_v3_bridge::build_complete_v3_final_candidate(
            phase,
            &accepted,
            &water_protocol,
            soil,
        )?;
    Ok(V3FixedFinalCoveredCandidate {
        transaction_id: phase.identity.transaction_id,
        identity: phase.identity.clone(),
        accepted_fixed_final: Box::new(accepted),
        potential_request_batch: phase.request_batch.clone(),
        phase_specific_authorization,
        water_protocol,
        complete_physical_candidate,
    })
}

#[cfg(test)]
#[path = "transaction_v3_tests.rs"]
mod covered_transaction_tests;
