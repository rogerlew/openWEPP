//! Atomic frozen-litter V3 runtime coordination.

use openwepp_kernel_contract::TransactionId;
use openwepp_land_surface_energy::{
    LandSurfaceEnergyConfiguration, LandSurfaceEnergyV3State, LitterPhaseReceipt,
    LitterPhaseTransactionIdentity, LitterPhaseTransactionInput, Sha256Digest,
    SoilThermalOwnerEnvelopeV2, SoilThermalOwnerRestartV2, V3TilePhaseUpdate,
    build_v3_ending_state, execute_litter_phase_v3,
};

use crate::direct_runtime::{
    DirectSurfaceLiquidIngressCandidateV2, DirectSurfaceLiquidIngressInput,
    DirectWb14CoupledChildBindingV1, DirectWb14ParentWorkingStateV2,
    SurfaceLiquidCompleteOwnerProjectionIdentityV3, SurfaceLiquidCompleteOwnerProjectionV3,
    SurfaceLiquidConfigurationV2, SurfaceLiquidOwnerEnvelopeV2,
    execute_surface_liquid_ingress_v2_with_parent_state_and_coupled_binding,
    prepare_surface_liquid_resource_candidate_v2,
};

use super::v3_input_projection::{
    FrozenLitterV3PhaseFreeInput, FrozenLitterV3RuntimeError, checked_support_seconds,
    project_frozen_litter_v3_phase,
};
use super::v3_rollback::FrozenLitterV3RollbackSnapshot;

pub(crate) struct FrozenLitterV3RuntimeInput<'a> {
    pub transaction_id: TransactionId,
    pub predecessor_transaction_id: Option<TransactionId>,
    pub parent_support_start_ns: u128,
    pub parent_support_end_ns: u128,
    pub support_start_ns: u128,
    pub support_end_ns: u128,
    pub predecessor_receipt_chain_sha256: String,
    pub surface_configuration: &'a SurfaceLiquidConfigurationV2,
    pub beginning_surface_owner: &'a SurfaceLiquidOwnerEnvelopeV2,
    pub lse_configuration: &'a LandSurfaceEnergyConfiguration,
    pub beginning_lse_state: &'a LandSurfaceEnergyV3State,
    pub phase_inputs: &'a [FrozenLitterV3PhaseFreeInput],
    pub current_ingress: &'a DirectSurfaceLiquidIngressInput,
    pub wb14_parent: Option<&'a DirectWb14ParentWorkingStateV2>,
    pub coupled_binding: DirectWb14CoupledChildBindingV1,
    pub soil_thermal_owner: &'a SoilThermalOwnerEnvelopeV2,
    pub soil_thermal_restart: &'a SoilThermalOwnerRestartV2,
}

pub(crate) struct AcceptedFrozenLitterV3RuntimeCandidate {
    pub phase_adjusted_surface_owner: SurfaceLiquidOwnerEnvelopeV2,
    pub ending_surface_owner: SurfaceLiquidOwnerEnvelopeV2,
    pub ending_lse_state: LandSurfaceEnergyV3State,
    pub litter_phase_receipts: Vec<LitterPhaseReceipt>,
    pub ingress: DirectSurfaceLiquidIngressCandidateV2,
    pub complete_owner_projection: SurfaceLiquidCompleteOwnerProjectionV3,
    pub rollback: FrozenLitterV3RollbackSnapshot,
}

fn typed_digest(value: &str) -> Result<Sha256Digest, FrozenLitterV3RuntimeError> {
    Sha256Digest::try_new(value).map_err(FrozenLitterV3RuntimeError::LandSurfaceEnergy)
}

fn validate_runtime_identity(
    input: &FrozenLitterV3RuntimeInput<'_>,
) -> Result<(), FrozenLitterV3RuntimeError> {
    let interval_s = checked_support_seconds(input.support_start_ns, input.support_end_ns)?;
    if input.transaction_id.0 == 0
        || input.parent_support_start_ns > input.support_start_ns
        || input.support_end_ns > input.parent_support_end_ns
        || input.parent_support_start_ns >= input.parent_support_end_ns
        || input.current_ingress.transaction_id != input.transaction_id
        || input.current_ingress.interval_s.to_bits() != interval_s.to_bits()
        || input.coupled_binding.parent_support_start_ns != input.parent_support_start_ns
        || input.coupled_binding.parent_support_end_ns != input.parent_support_end_ns
        || input.coupled_binding.child_support_start_ns != input.support_start_ns
        || input.coupled_binding.child_support_end_ns != input.support_end_ns
        || f64::from_bits(input.coupled_binding.proposed_upper_bound_s_bits) < interval_s
    {
        return Err(FrozenLitterV3RuntimeError::Chronology(
            "runtime, ingress, and coupled-child support join",
        ));
    }
    input
        .beginning_lse_state
        .validate(input.lse_configuration)?;
    if input.beginning_lse_state.0.last_accepted_transaction_id != input.predecessor_transaction_id
        || input
            .beginning_lse_state
            .0
            .last_accepted_transaction_id
            .is_some_and(|prior| prior.0 >= input.transaction_id.0)
    {
        return Err(FrozenLitterV3RuntimeError::Identity(
            "LSE predecessor transaction",
        ));
    }
    input.soil_thermal_owner.validate()?;
    if input.soil_thermal_owner.run_id != input.surface_configuration.parent().run_id.to_string()
        || input.soil_thermal_owner.transaction_id != input.transaction_id
        || input.soil_thermal_owner.expected_predecessor_transaction_id
            != input.predecessor_transaction_id
        || input.soil_thermal_owner.support_start_ns != input.support_start_ns
        || input.soil_thermal_owner.support_end_ns != input.support_end_ns
        || input.soil_thermal_restart.owner_tag != input.soil_thermal_owner.owner_tag
        || input.soil_thermal_restart.schema_sha256 != input.soil_thermal_owner.schema_sha256
        || input.soil_thermal_restart.exact_carry_definition_sha256
            != input.soil_thermal_owner.exact_carry_definition_sha256
        || input.soil_thermal_restart.parent_v1_state_sha256
            != input.soil_thermal_owner.parent_v1_state_sha256
        || input.soil_thermal_restart.owner_state_sha256
            != input.soil_thermal_owner.state.state_sha256
        || input.soil_thermal_restart.receipt_chain_sha256
            != input.soil_thermal_owner.receipt_chain_sha256
    {
        return Err(FrozenLitterV3RuntimeError::Identity(
            "soil V2 owner/restart runtime join",
        ));
    }
    Ok(())
}

/// Execute one accepted positive frozen-litter child. Every input is immutable;
/// the returned candidates are publishable only after the final projection
/// replay succeeds.
#[allow(clippy::too_many_lines)]
pub(crate) fn execute_frozen_litter_v3(
    input: &FrozenLitterV3RuntimeInput<'_>,
) -> Result<AcceptedFrozenLitterV3RuntimeCandidate, FrozenLitterV3RuntimeError> {
    validate_runtime_identity(input)?;
    let rollback = FrozenLitterV3RollbackSnapshot::capture(
        input.surface_configuration,
        input.beginning_surface_owner,
        input.beginning_lse_state,
        input.soil_thermal_owner,
        input.soil_thermal_restart,
        input.wb14_parent,
    )?;
    let projected = project_frozen_litter_v3_phase(
        input.surface_configuration,
        input.beginning_surface_owner,
        input.lse_configuration,
        input.beginning_lse_state,
        input.transaction_id,
        input.support_start_ns,
        input.support_end_ns,
        input.phase_inputs,
    )?;
    let beginning_digest = typed_digest(input.beginning_surface_owner.envelope_sha256())?;
    let phase_digest = typed_digest(projected.phase_adjusted_owner.envelope_sha256())?;
    let mut receipts = Vec::with_capacity(input.phase_inputs.len());
    let mut updates = Vec::with_capacity(input.phase_inputs.len());
    for (phase_input, preview) in input.phase_inputs.iter().zip(&projected.endings) {
        let candidate = execute_litter_phase_v3(&LitterPhaseTransactionInput {
            identity: LitterPhaseTransactionIdentity {
                lse_configuration_sha256: input.lse_configuration.configuration_sha256.clone(),
                transaction_id: input.transaction_id,
                ofe_id: phase_input.ofe_id.clone(),
                tile_id: phase_input.tile_id.clone(),
                surface_owner_id: input.surface_configuration.parent().owner_id.clone(),
                beginning_surface_owner_sha256: beginning_digest.clone(),
                candidate_surface_owner_sha256: phase_digest.clone(),
                support_start_ns: input.support_start_ns,
                support_end_ns: input.support_end_ns,
            },
            configuration: phase_input.configuration,
            beginning: phase_input.beginning,
            vapor_environment: phase_input.accepted_vapor().raw.environment,
            finalized_vapor: phase_input.accepted_vapor().finalized,
            phase_free_surface_energy: phase_input.accepted_surface_energy(),
        })?;
        if candidate.ending != *preview
            || candidate.receipt.vapor != phase_input.accepted_vapor()
            || candidate.receipt.post_vapor != phase_input.accepted_post_vapor()
        {
            return Err(FrozenLitterV3RuntimeError::Closure(
                "phase preseal and accepted fixed-final evaluation differ",
            ));
        }
        updates.push(V3TilePhaseUpdate {
            ofe_id: phase_input.ofe_id.clone(),
            tile_id: phase_input.tile_id.clone(),
            ending_sensible_energy_j_m2_tile: candidate.ending.sensible_energy_j_m2_tile,
            ending_temperature_k: candidate.ending.temperature_k,
        });
        receipts.push(candidate.receipt);
    }
    let ending_lse_state = build_v3_ending_state(
        input.beginning_lse_state,
        input.lse_configuration,
        input.transaction_id,
        &updates,
    )?;
    let resource = prepare_surface_liquid_resource_candidate_v2(
        input.surface_configuration,
        input.beginning_surface_owner,
        &projected.phase_adjusted_owner,
        input.transaction_id,
        &projected.closure,
    )?;
    let ingress = execute_surface_liquid_ingress_v2_with_parent_state_and_coupled_binding(
        input.surface_configuration,
        &resource,
        input.current_ingress,
        input.wb14_parent,
        false,
        Some(input.coupled_binding),
    )?;
    let wb14_parent =
        ingress
            .parent_working_state()
            .ok_or(FrozenLitterV3RuntimeError::Chronology(
                "accepted child omitted open WB14 parent",
            ))?;
    let wb14_bytes = wb14_parent.restart_bytes(input.surface_configuration)?;
    let projection = SurfaceLiquidCompleteOwnerProjectionV3::new(
        input.surface_configuration,
        SurfaceLiquidCompleteOwnerProjectionIdentityV3 {
            run_id: input.surface_configuration.parent().run_id,
            transaction_id: input.transaction_id,
            predecessor_transaction_id: input.predecessor_transaction_id,
            parent_support_start_ns: input.parent_support_start_ns,
            parent_support_end_ns: input.parent_support_end_ns,
            support_start_ns: input.support_start_ns,
            support_end_ns: input.support_end_ns,
            beginning_surface_owner_sha256: input.beginning_surface_owner.envelope_sha256().into(),
            phase_adjusted_surface_owner_sha256: projected
                .phase_adjusted_owner
                .envelope_sha256()
                .into(),
            predecessor_receipt_chain_sha256: input.predecessor_receipt_chain_sha256.clone(),
            receipt_chain_sha256: "0".repeat(64),
        },
        ingress.ending_owner(),
        &wb14_bytes,
        &receipts,
        ingress.inner().receipts(),
        input.soil_thermal_owner,
        input.soil_thermal_restart,
    )?;
    let projection_bytes = projection.canonical_bytes(input.surface_configuration)?;
    let replay = SurfaceLiquidCompleteOwnerProjectionV3::from_canonical_bytes(
        input.surface_configuration,
        &projection_bytes,
    )?;
    if replay != projection {
        return Err(FrozenLitterV3RuntimeError::Closure(
            "complete-owner projection V3 replay",
        ));
    }
    Ok(AcceptedFrozenLitterV3RuntimeCandidate {
        phase_adjusted_surface_owner: projected.phase_adjusted_owner,
        ending_surface_owner: ingress.ending_owner().clone(),
        ending_lse_state,
        litter_phase_receipts: receipts,
        ingress,
        complete_owner_projection: projection,
        rollback,
    })
}
