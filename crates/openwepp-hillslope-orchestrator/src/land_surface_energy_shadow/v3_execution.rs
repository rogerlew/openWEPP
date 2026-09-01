//! Atomic frozen-litter V3 runtime coordination.

use std::collections::BTreeMap;

use openwepp_kernel_contract::TransactionId;
use openwepp_land_surface_energy::{
    ExactDyadicEnthalpy, LITTER_ICE_HEAT_CAPACITY_J_KG_K, LandSurfaceEnergyConfiguration,
    LandSurfaceEnergyV3State, LitterPhaseReceipt, LitterPhaseTransactionIdentity,
    LitterPhaseTransactionInput, REFERENCE_TEMPERATURE_K, Sha256Digest, SoilThermalOwnerEnvelopeV2,
    SoilThermalOwnerRestartV2, SurfaceConfiguration, SurfaceHeatStorageMode, V3TilePhaseUpdate,
    WATER_HEAT_CAPACITY_J_KG_K, build_v3_ending_state, execute_litter_phase_v3,
};

use crate::direct_runtime::{
    DirectSurfaceLiquidIngressCandidateV2, DirectSurfaceLiquidIngressInput,
    DirectSurfaceLiquidParcelReceipt, DirectWb14CoupledChildBindingV1,
    DirectWb14ParentWorkingStateV2, LseSurfaceEnthalpyAcceptedEnergyOperandV1,
    LseSurfaceEnthalpyEnergyCreditReceiptV1, LseSurfaceEnthalpyEnergyOperandKindV1,
    LseSurfaceEnthalpyErrorV1, LseSurfaceEnthalpyOwnerEnvelopeV1,
    SurfaceLiquidCompleteOwnerProjectionIdentityV3, SurfaceLiquidCompleteOwnerProjectionV3,
    SurfaceLiquidCompleteOwnerProjectionV4, SurfaceLiquidConfigurationV2,
    SurfaceLiquidOwnerEnvelopeV2,
    execute_surface_liquid_ingress_v2_with_parent_state_and_coupled_binding,
    prepare_surface_liquid_resource_candidate_v2,
};

use super::retained_surface_tile_credits_from_receipts_v1;
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
    pub finalize_wb14_parent_interval: bool,
    pub coupled_binding: DirectWb14CoupledChildBindingV1,
    pub soil_thermal_owner: &'a SoilThermalOwnerEnvelopeV2,
    pub soil_thermal_restart: &'a SoilThermalOwnerRestartV2,
}

pub(crate) struct AcceptedFrozenLitterV3RuntimeCandidate {
    pub phase_adjusted_surface_owner: SurfaceLiquidOwnerEnvelopeV2,
    pub ending_surface_owner: SurfaceLiquidOwnerEnvelopeV2,
    pub ending_lse_state: LandSurfaceEnergyV3State,
    pub litter_phase_receipts: Vec<LitterPhaseReceipt>,
    pub surface_resource: crate::direct_runtime::DirectSurfaceLiquidResourceCandidateV2,
    pub ingress: DirectSurfaceLiquidIngressCandidateV2,
    pub complete_owner_projection: SurfaceLiquidCompleteOwnerProjectionV3,
    pub rollback: FrozenLitterV3RollbackSnapshot,
}

/// V16 successor input. The V3 physical transaction remains the immutable
/// producer; the exact companion is the sole authoritative surface-enthalpy
/// owner on this path.
pub(crate) struct FrozenLitterV4RuntimeInput<'a> {
    pub physical: FrozenLitterV3RuntimeInput<'a>,
    pub beginning_exact_surface_owner: &'a LseSurfaceEnthalpyOwnerEnvelopeV1,
}

/// Atomic V16 candidate. Publication may occur only after projection-V4 replay
/// and exact receipt validation have both succeeded.
pub(crate) struct AcceptedFrozenLitterV4RuntimeCandidate {
    pub physical: AcceptedFrozenLitterV3RuntimeCandidate,
    pub ending_exact_surface_owner: LseSurfaceEnthalpyOwnerEnvelopeV1,
    pub exact_surface_receipt: LseSurfaceEnthalpyEnergyCreditReceiptV1,
    pub complete_owner_projection: SurfaceLiquidCompleteOwnerProjectionV4,
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
        || input.finalize_wb14_parent_interval
            != (input.support_end_ns == input.parent_support_end_ns)
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
    if input.soil_thermal_owner.run_id != input.surface_configuration.parent().run_id.to_string() {
        return Err(FrozenLitterV3RuntimeError::Identity(
            "soil V2/surface run identity",
        ));
    }
    if input.soil_thermal_owner.transaction_id != input.transaction_id {
        return Err(FrozenLitterV3RuntimeError::Identity(
            "soil V2/current transaction",
        ));
    }
    if input.soil_thermal_owner.support_start_ns != input.support_start_ns
        || input.soil_thermal_owner.support_end_ns != input.support_end_ns
    {
        return Err(FrozenLitterV3RuntimeError::Identity(
            "soil V2 runtime support",
        ));
    }
    if input.soil_thermal_restart.owner_tag != input.soil_thermal_owner.owner_tag
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
            "soil V2 owner/restart seal join",
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
        input.finalize_wb14_parent_interval,
        Some(input.coupled_binding),
    )?;
    let wb14_bytes = ingress
        .parent_working_state()
        .map(|parent| parent.restart_bytes(input.surface_configuration))
        .transpose()?;
    let projection = SurfaceLiquidCompleteOwnerProjectionV3::new(
        input.surface_configuration,
        SurfaceLiquidCompleteOwnerProjectionIdentityV3 {
            run_id: input.surface_configuration.parent().run_id,
            transaction_id: input.transaction_id,
            predecessor_transaction_id: input.predecessor_transaction_id,
            soil_thermal_predecessor_transaction_id: input
                .soil_thermal_owner
                .expected_predecessor_transaction_id,
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
        &projected.phase_adjusted_owner,
        wb14_bytes.as_deref(),
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
        surface_resource: resource,
        ingress,
        complete_owner_projection: projection,
        rollback,
    })
}

fn accepted_surface_energy_operands_v1(
    input: &FrozenLitterV3RuntimeInput<'_>,
    accepted: &AcceptedFrozenLitterV3RuntimeCandidate,
) -> Result<Vec<LseSurfaceEnthalpyAcceptedEnergyOperandV1>, FrozenLitterV3RuntimeError> {
    reconstruct_surface_energy_operands_v1(
        input.surface_configuration,
        input.transaction_id,
        input.predecessor_transaction_id,
        input.support_start_ns,
        input.support_end_ns,
        &accepted.litter_phase_receipts,
        accepted.ingress.inner().receipts(),
    )
}

#[allow(clippy::too_many_arguments, clippy::too_many_lines)]
pub(crate) fn reconstruct_surface_energy_operands_v1(
    surface_configuration: &SurfaceLiquidConfigurationV2,
    transaction_id: TransactionId,
    predecessor_transaction_id: Option<TransactionId>,
    support_start_ns: u128,
    support_end_ns: u128,
    litter_phase_receipts: &[LitterPhaseReceipt],
    current_ingress_receipts: &[DirectSurfaceLiquidParcelReceipt],
) -> Result<Vec<LseSurfaceEnthalpyAcceptedEnergyOperandV1>, FrozenLitterV3RuntimeError> {
    let mut operands = Vec::with_capacity(litter_phase_receipts.len() * 8);
    for receipt in litter_phase_receipts {
        let identity = &receipt.identity;
        let surface_key = surface_configuration
            .parent()
            .records
            .iter()
            .find(|record| {
                record.key.ofe_id == identity.ofe_id && record.key.tile_id == identity.tile_id
            })
            .map(|record| record.key.clone())
            .ok_or(FrozenLitterV3RuntimeError::Identity(
                "V16 litter receipt surface key",
            ))?;
        if identity.transaction_id != transaction_id
            || identity.support_start_ns != support_start_ns
            || identity.support_end_ns != support_end_ns
            || identity.surface_owner_id != surface_configuration.parent().owner_id
        {
            return Err(FrozenLitterV3RuntimeError::Identity(
                "V16 litter receipt transaction/support/owner",
            ));
        }
        let duration_s = f64::from_bits(identity.support_duration_seconds_bits);
        let ledger = receipt.phase_free_surface_energy;
        let primitive_fluxes = [
            ledger.absorbed_shortwave_w_m2,
            ledger.net_longwave_w_m2,
            -ledger.sensible_to_canopy_air_w_m2,
            -ledger.liquid_vapor_energy_w_m2,
            -ledger.ice_vapor_energy_w_m2,
            -ledger.ground_heat_w_m2,
        ];
        for (ordinal, flux_w_m2) in primitive_fluxes.into_iter().enumerate() {
            let energy_j_m2_tile_ground = flux_w_m2 * duration_s;
            if !energy_j_m2_tile_ground.is_finite() {
                return Err(FrozenLitterV3RuntimeError::Closure(
                    "V16 finite accepted phase-free primitive",
                ));
            }
            operands.push(LseSurfaceEnthalpyAcceptedEnergyOperandV1 {
                surface_key: surface_key.clone(),
                kind: LseSurfaceEnthalpyEnergyOperandKindV1::PhaseFreeSurfaceEnergy,
                ordinal: u32::try_from(ordinal).map_err(|_| {
                    FrozenLitterV3RuntimeError::Closure("V16 phase-free operand ordinal")
                })?,
                source_owner_id: identity.surface_owner_id.clone(),
                source_receipt_sha256: receipt.receipt_sha256.clone(),
                transaction_id,
                predecessor_transaction_id,
                support_start_ns,
                support_end_ns,
                units: "J m^-2 tile-ground".to_owned(),
                basis: "tile_ground".to_owned(),
                energy_j_m2_tile_ground,
            });
        }
        if !receipt.transfer.fusion_energy_j_m2.is_finite() {
            return Err(FrozenLitterV3RuntimeError::Closure(
                "V16 finite accepted litter fusion",
            ));
        }
        operands.push(LseSurfaceEnthalpyAcceptedEnergyOperandV1 {
            surface_key,
            kind: LseSurfaceEnthalpyEnergyOperandKindV1::LitterFusionEnergy,
            ordinal: 0,
            source_owner_id: identity.surface_owner_id.clone(),
            source_receipt_sha256: receipt.receipt_sha256.clone(),
            transaction_id,
            predecessor_transaction_id,
            support_start_ns,
            support_end_ns,
            units: "J m^-2 tile-ground".to_owned(),
            basis: "tile_ground".to_owned(),
            energy_j_m2_tile_ground: receipt.transfer.fusion_energy_j_m2,
        });
    }
    for credit in retained_surface_tile_credits_from_receipts_v1(
        surface_configuration,
        transaction_id,
        current_ingress_receipts,
    )
    .map_err(|_| FrozenLitterV3RuntimeError::Closure("V16 retained tile-credit reconstruction"))?
    {
        operands.push(LseSurfaceEnthalpyAcceptedEnergyOperandV1 {
            surface_key: credit.store_key,
            kind: LseSurfaceEnthalpyEnergyOperandKindV1::RetainedIngressTileCredit,
            ordinal: credit.ordinal,
            source_owner_id: surface_configuration.parent().owner_id.clone(),
            source_receipt_sha256: credit.source_receipt_sha256,
            transaction_id,
            predecessor_transaction_id,
            support_start_ns,
            support_end_ns,
            units: "J m^-2 tile-ground".to_owned(),
            basis: "tile_ground".to_owned(),
            energy_j_m2_tile_ground: credit.energy_j_m2_tile_ground,
        });
    }
    operands.sort_by(|left, right| {
        (&left.surface_key, &left.kind, left.ordinal).cmp(&(
            &right.surface_key,
            &right.kind,
            right.ordinal,
        ))
    });
    Ok(operands)
}

fn lse_candidate_with_surface_high_mirrors(
    input: &FrozenLitterV3RuntimeInput<'_>,
    accepted: &AcceptedFrozenLitterV3RuntimeCandidate,
) -> Result<LandSurfaceEnergyV3State, FrozenLitterV3RuntimeError> {
    let surface =
        accepted
            .ending_surface_owner
            .v2_state()
            .ok_or(FrozenLitterV3RuntimeError::Identity(
                "V16 ending surface owner V2",
            ))?;
    let mut updates = Vec::with_capacity(surface.records().len());
    for record in surface.records() {
        let configured = input
            .lse_configuration
            .ofes
            .iter()
            .find(|ofe| ofe.ofe_id == record.key.ofe_id)
            .and_then(|ofe| {
                ofe.tiles
                    .iter()
                    .find(|tile| tile.tile_id == record.key.tile_id)
            })
            .ok_or(FrozenLitterV3RuntimeError::Identity(
                "V16 LSE/surface topology join",
            ))?;
        let ending_temperature_k = if configured.surface_heat_storage_mode
            == SurfaceHeatStorageMode::EquilibriumZero
        {
            if record.surface_enthalpy_j_m2_tile.to_bits() != 0.0_f64.to_bits() {
                return Err(FrozenLitterV3RuntimeError::Closure(
                    "V16 equilibrium-zero high mirror",
                ));
            }
            accepted
                .ending_lse_state
                .0
                .tiles
                .iter()
                .find(|tile| tile.ofe_id == record.key.ofe_id && tile.tile_id == record.key.tile_id)
                .map(|tile| tile.surface_temperature_warm_start_k)
                .ok_or(FrozenLitterV3RuntimeError::Identity(
                    "V16 equilibrium-zero LSE tile",
                ))?
        } else {
            let dry_capacity = match configured.surface {
                SurfaceConfiguration::BareMineralSoil {
                    dry_areal_heat_capacity_j_m2_k,
                    ..
                } => dry_areal_heat_capacity_j_m2_k,
                SurfaceConfiguration::ForestLitter {
                    thickness_m,
                    dry_density_kg_m3,
                    dry_specific_heat_j_kg_k,
                    ..
                } => thickness_m * dry_density_kg_m3 * dry_specific_heat_j_kg_k,
            };
            let capacity = dry_capacity
                + record.liquid_kg_m2_tile * WATER_HEAT_CAPACITY_J_KG_K
                + record.litter_ice_kg_m2_tile * LITTER_ICE_HEAT_CAPACITY_J_KG_K;
            let temperature =
                REFERENCE_TEMPERATURE_K + record.surface_enthalpy_j_m2_tile / capacity;
            if !capacity.is_finite() || capacity <= 0.0 || !(200.0..=350.0).contains(&temperature) {
                return Err(FrozenLitterV3RuntimeError::Closure(
                    "V16 high-mirror temperature projection",
                ));
            }
            temperature
        };
        updates.push(V3TilePhaseUpdate {
            ofe_id: record.key.ofe_id.clone(),
            tile_id: record.key.tile_id.clone(),
            ending_sensible_energy_j_m2_tile: record.surface_enthalpy_j_m2_tile,
            ending_temperature_k,
        });
    }
    Ok(build_v3_ending_state(
        input.beginning_lse_state,
        input.lse_configuration,
        input.transaction_id,
        &updates,
    )?)
}

fn exact_rounded_surface_highs(
    beginning: &LseSurfaceEnthalpyOwnerEnvelopeV1,
    operands: &[LseSurfaceEnthalpyAcceptedEnergyOperandV1],
) -> Result<BTreeMap<crate::DirectSurfaceLiquidStoreKey, f64>, FrozenLitterV3RuntimeError> {
    beginning
        .records()
        .iter()
        .map(|record| {
            let values = operands
                .iter()
                .filter(|operand| operand.surface_key == record.surface_key)
                .map(|operand| operand.energy_j_m2_tile_ground)
                .collect::<Vec<_>>();
            if values.iter().all(|value| *value == 0.0) {
                return Ok((record.surface_key.clone(), record.enthalpy_hi_j_m2_tile));
            }
            let total = ExactDyadicEnthalpy::exact_sum_binary64(
                record.enthalpy_hi_j_m2_tile,
                &record.enthalpy_carry,
                &values,
            )
            .map_err(LseSurfaceEnthalpyErrorV1::from)?;
            let (high, _) = total
                .rounded_high_and_remainder()
                .map_err(LseSurfaceEnthalpyErrorV1::from)?;
            Ok((record.surface_key.clone(), high))
        })
        .collect()
}

fn surface_owner_with_exact_high_mirrors(
    input: &FrozenLitterV3RuntimeInput<'_>,
    accepted: &AcceptedFrozenLitterV3RuntimeCandidate,
    highs: &BTreeMap<crate::DirectSurfaceLiquidStoreKey, f64>,
) -> Result<SurfaceLiquidOwnerEnvelopeV2, FrozenLitterV3RuntimeError> {
    let ending =
        accepted
            .ending_surface_owner
            .v2_state()
            .ok_or(FrozenLitterV3RuntimeError::Identity(
                "V16 ending surface owner V2",
            ))?;
    if ending.records().len() != highs.len() {
        return Err(FrozenLitterV3RuntimeError::Identity(
            "V16 exact high-mirror cardinality",
        ));
    }
    let records = ending
        .records()
        .iter()
        .map(|record| {
            let mut record = record.clone();
            record.surface_enthalpy_j_m2_tile =
                *highs
                    .get(&record.key)
                    .ok_or(FrozenLitterV3RuntimeError::Identity(
                        "V16 exact high-mirror key",
                    ))?;
            Ok(record)
        })
        .collect::<Result<Vec<_>, FrozenLitterV3RuntimeError>>()?;
    Ok(accepted.ending_surface_owner.try_replace_v2_state(
        input.surface_configuration,
        records,
        ending.continuations().to_vec(),
    )?)
}

fn reseal_v3_projection_with_exact_high_mirror(
    input: &FrozenLitterV3RuntimeInput<'_>,
    accepted: &AcceptedFrozenLitterV3RuntimeCandidate,
) -> Result<SurfaceLiquidCompleteOwnerProjectionV3, FrozenLitterV3RuntimeError> {
    let wb14_bytes = accepted
        .ingress
        .parent_working_state()
        .map(|parent| parent.restart_bytes(input.surface_configuration))
        .transpose()?;
    Ok(SurfaceLiquidCompleteOwnerProjectionV3::new(
        input.surface_configuration,
        SurfaceLiquidCompleteOwnerProjectionIdentityV3 {
            run_id: input.surface_configuration.parent().run_id,
            transaction_id: input.transaction_id,
            predecessor_transaction_id: input.predecessor_transaction_id,
            soil_thermal_predecessor_transaction_id: input
                .soil_thermal_owner
                .expected_predecessor_transaction_id,
            parent_support_start_ns: input.parent_support_start_ns,
            parent_support_end_ns: input.parent_support_end_ns,
            support_start_ns: input.support_start_ns,
            support_end_ns: input.support_end_ns,
            beginning_surface_owner_sha256: input.beginning_surface_owner.envelope_sha256().into(),
            phase_adjusted_surface_owner_sha256: accepted
                .phase_adjusted_surface_owner
                .envelope_sha256()
                .into(),
            predecessor_receipt_chain_sha256: input.predecessor_receipt_chain_sha256.clone(),
            receipt_chain_sha256: "0".repeat(64),
        },
        &accepted.ending_surface_owner,
        &accepted.phase_adjusted_surface_owner,
        wb14_bytes.as_deref(),
        &accepted.litter_phase_receipts,
        accepted.ingress.inner().receipts(),
        input.soil_thermal_owner,
        input.soil_thermal_restart,
    )?)
}

/// Execute the V16 exact-surface successor without feeding carry into V14
/// physics. The V3 transaction runs first against immutable beginnings; only
/// its accepted named receipt operands enter exact integer aggregation.
pub(crate) fn execute_frozen_litter_v4(
    input: &FrozenLitterV4RuntimeInput<'_>,
) -> Result<AcceptedFrozenLitterV4RuntimeCandidate, FrozenLitterV3RuntimeError> {
    input
        .beginning_exact_surface_owner
        .validate_frozen_parent_join(
            input.physical.lse_configuration,
            input.physical.beginning_lse_state,
            input.physical.surface_configuration,
            input.physical.beginning_surface_owner,
        )?;
    let mut physical = execute_frozen_litter_v3(&input.physical)?;
    let operands = accepted_surface_energy_operands_v1(&input.physical, &physical)?;
    let exact_highs = exact_rounded_surface_highs(input.beginning_exact_surface_owner, &operands)?;
    let ending_surface_owner =
        surface_owner_with_exact_high_mirrors(&input.physical, &physical, &exact_highs)?;
    physical.ingress = physical.ingress.with_exact_surface_enthalpy_high_owner(
        input.physical.surface_configuration,
        ending_surface_owner.clone(),
    )?;
    physical.ending_surface_owner = ending_surface_owner;
    physical.ending_lse_state =
        lse_candidate_with_surface_high_mirrors(&input.physical, &physical)?;
    physical.complete_owner_projection =
        reseal_v3_projection_with_exact_high_mirror(&input.physical, &physical)?;

    let accepted_operands = operands.clone();
    let exact = input.beginning_exact_surface_owner.advance_exact(
        &physical.ending_lse_state,
        input.physical.surface_configuration,
        &physical.ending_surface_owner,
        input.physical.transaction_id,
        input.physical.predecessor_transaction_id,
        input.physical.support_start_ns,
        input.physical.support_end_ns,
        &operands,
        accepted_operands,
    )?;
    let projection = SurfaceLiquidCompleteOwnerProjectionV4::new(
        input.physical.surface_configuration,
        &physical.complete_owner_projection,
        input.physical.beginning_lse_state,
        input.beginning_exact_surface_owner,
        &exact.ending_owner,
        &exact.receipt,
    )?;
    let bytes = projection.canonical_bytes(input.physical.surface_configuration)?;
    let replay = SurfaceLiquidCompleteOwnerProjectionV4::from_canonical_bytes(
        input.physical.surface_configuration,
        &bytes,
        input.physical.beginning_lse_state.0.state_sha256.as_str(),
    )?;
    if replay != projection {
        return Err(FrozenLitterV3RuntimeError::Closure(
            "complete-owner projection V4 replay",
        ));
    }
    Ok(AcceptedFrozenLitterV4RuntimeCandidate {
        physical,
        ending_exact_surface_owner: exact.ending_owner,
        exact_surface_receipt: exact.receipt,
        complete_owner_projection: projection,
    })
}
