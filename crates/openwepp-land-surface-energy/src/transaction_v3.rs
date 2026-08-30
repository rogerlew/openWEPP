//! Atomic candidate construction for the snow-free forest-litter LSE V3 seam.

#![allow(clippy::missing_errors_doc)]

use serde::Serialize;
use std::time::Duration;

use crate::{
    AcceptedLitterPhaseCandidate, BeginningLitterPhaseState, FinalizedLitterVapor,
    LandSurfaceEnergyError, LitterPhaseConfiguration, LitterPhaseReceipt,
    LitterPhaseReceiptIdentity, LitterVaporEnvironment, OfeId, Sha256Digest,
    V3_MODEL_DEFINITION_SHA256, V3_MODEL_VERSION, V3_PHASE_RECEIPT_VERSION,
    apply_bounded_litter_phase, canonical_digest, evaluate_raw_litter_vapor, finalize_litter_vapor,
    install_finalized_vapor, publish_phase_free_litter_vapor, reconstruct_litter_phase_closure,
    validate_beginning_litter_state, validate_litter_phase_configuration,
};
use openwepp_kernel_contract::{ResourceOwnerId, TileId, TransactionId};

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
}

#[derive(Serialize)]
struct ReceiptDigestView<'a> {
    identity: &'a LitterPhaseReceiptIdentity,
    configuration: LitterPhaseConfiguration,
    beginning: BeginningLitterPhaseState,
    vapor: crate::LitterVaporReceipt,
    post_vapor: crate::PostVaporLitterState,
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
