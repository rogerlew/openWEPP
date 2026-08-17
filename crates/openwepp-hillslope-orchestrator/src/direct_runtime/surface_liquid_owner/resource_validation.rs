use std::collections::{BTreeMap, BTreeSet};

use openwepp_kernel_contract::TransactionId;
use openwepp_land_surface_energy::{CondensationCredit, StandGroundWaterAmountBasis, WaterAmount};

use super::{
    DirectSurfaceLiquidArbitration, DirectSurfaceLiquidConfiguration, DirectSurfaceLiquidError,
    DirectSurfaceLiquidErrorCode, DirectSurfaceLiquidErrorContext, DirectSurfaceLiquidPhase,
    DirectSurfaceLiquidRollbackHashes, DirectSurfaceLiquidStoreKey, water_protocol_failure,
};

pub(super) fn preflight_resource_phase_inputs(
    configuration: &DirectSurfaceLiquidConfiguration,
    arbitration: &DirectSurfaceLiquidArbitration,
    finalized_uses: &[WaterAmount],
    condensation_credits: &[CondensationCredit],
) -> Result<(), DirectSurfaceLiquidError> {
    validate_identities(
        configuration,
        arbitration,
        finalized_uses,
        condensation_credits,
    )?;
    validate_domains(
        configuration,
        arbitration,
        finalized_uses,
        condensation_credits,
    )?;
    validate_protocol_cardinality(
        configuration,
        arbitration,
        finalized_uses,
        condensation_credits,
    )?;
    validate_bounds(
        configuration,
        arbitration,
        finalized_uses,
        condensation_credits,
    )?;
    Ok(())
}

fn validate_identities(
    configuration: &DirectSurfaceLiquidConfiguration,
    arbitration: &DirectSurfaceLiquidArbitration,
    finalized_uses: &[WaterAmount],
    condensation_credits: &[CondensationCredit],
) -> Result<(), DirectSurfaceLiquidError> {
    let transaction_id = arbitration.transaction_id;
    for request in &arbitration.requests {
        request.key.validate(transaction_id).map_err(|_| {
            water_protocol_failure(
                DirectSurfaceLiquidErrorCode::E002,
                DirectSurfaceLiquidPhase::ResourceCandidate,
                transaction_id,
                &request.key,
                "invalid retained request key",
            )
        })?;
        configuration
            .store_key_for_water(&request.key)
            .map_err(|error| {
                water_protocol_failure(
                    DirectSurfaceLiquidErrorCode::E002,
                    DirectSurfaceLiquidPhase::ResourceCandidate,
                    transaction_id,
                    &request.key,
                    error.to_string(),
                )
            })?;
    }
    for authorization in &arbitration.authorizations {
        authorization.key.validate(transaction_id).map_err(|_| {
            water_protocol_failure(
                DirectSurfaceLiquidErrorCode::E002,
                DirectSurfaceLiquidPhase::ResourceCandidate,
                transaction_id,
                &authorization.key,
                "invalid retained authorization key",
            )
        })?;
        configuration
            .store_key_for_water(&authorization.key)
            .map_err(|error| {
                water_protocol_failure(
                    DirectSurfaceLiquidErrorCode::E002,
                    DirectSurfaceLiquidPhase::ResourceCandidate,
                    transaction_id,
                    &authorization.key,
                    error.to_string(),
                )
            })?;
    }
    for finalized in finalized_uses {
        finalized.key.validate(transaction_id).map_err(|_| {
            water_protocol_failure(
                DirectSurfaceLiquidErrorCode::E002,
                DirectSurfaceLiquidPhase::ResourceCandidate,
                transaction_id,
                &finalized.key,
                "invalid finalized-use key",
            )
        })?;
        configuration
            .store_key_for_water(&finalized.key)
            .map_err(|error| {
                water_protocol_failure(
                    DirectSurfaceLiquidErrorCode::E002,
                    DirectSurfaceLiquidPhase::ResourceCandidate,
                    transaction_id,
                    &finalized.key,
                    error.to_string(),
                )
            })?;
    }
    for credit in condensation_credits {
        if credit.transaction_id != transaction_id
            || credit.hydrology_owner_id != configuration.owner_id
        {
            return Err(condensation_failure(
                configuration,
                transaction_id,
                credit,
                DirectSurfaceLiquidErrorCode::E002,
                "condensation transaction or owner mismatch",
            ));
        }
        condensation_store_key(configuration, credit).ok_or_else(|| {
            condensation_failure(
                configuration,
                transaction_id,
                credit,
                DirectSurfaceLiquidErrorCode::E002,
                "condensation store missing",
            )
        })?;
    }
    Ok(())
}

fn validate_domains(
    configuration: &DirectSurfaceLiquidConfiguration,
    arbitration: &DirectSurfaceLiquidArbitration,
    finalized_uses: &[WaterAmount],
    condensation_credits: &[CondensationCredit],
) -> Result<(), DirectSurfaceLiquidError> {
    let transaction_id = arbitration.transaction_id;
    for request in &arbitration.requests {
        if !request.amount_kg_m2_stand_ground.is_finite() {
            return Err(water_protocol_failure(
                DirectSurfaceLiquidErrorCode::E003,
                DirectSurfaceLiquidPhase::ResourceCandidate,
                transaction_id,
                &request.key,
                "retained request is nonfinite",
            ));
        }
    }
    for authorization in &arbitration.authorizations {
        if !authorization.amount_kg_m2_stand_ground.is_finite() {
            return Err(water_protocol_failure(
                DirectSurfaceLiquidErrorCode::E003,
                DirectSurfaceLiquidPhase::ResourceCandidate,
                transaction_id,
                &authorization.key,
                "retained authorization is nonfinite",
            ));
        }
    }
    for finalized in finalized_uses {
        if !finalized.amount_kg_m2_stand_ground.is_finite() {
            return Err(water_protocol_failure(
                DirectSurfaceLiquidErrorCode::E003,
                DirectSurfaceLiquidPhase::ResourceCandidate,
                transaction_id,
                &finalized.key,
                "finalized use is nonfinite",
            ));
        }
    }
    for credit in condensation_credits {
        if !credit.amount_kg_m2_stand_ground.is_finite()
            || !credit.temperature_k.is_finite()
            || !credit.specific_liquid_enthalpy_j_kg.is_finite()
            || !(200.0..=350.0).contains(&credit.temperature_k)
        {
            return Err(condensation_failure(
                configuration,
                transaction_id,
                credit,
                DirectSurfaceLiquidErrorCode::E003,
                "condensation amount, temperature, or enthalpy is nonfinite or out of domain",
            ));
        }
    }
    Ok(())
}

fn validate_protocol_cardinality(
    configuration: &DirectSurfaceLiquidConfiguration,
    arbitration: &DirectSurfaceLiquidArbitration,
    finalized_uses: &[WaterAmount],
    condensation_credits: &[CondensationCredit],
) -> Result<(), DirectSurfaceLiquidError> {
    let transaction_id = arbitration.transaction_id;
    if arbitration.authorizations.len() != arbitration.requests.len()
        || arbitration.request_store_keys.len() != arbitration.requests.len()
    {
        return Err(incomplete_protocol_failure(configuration, transaction_id));
    }
    let mut retained_request_keys = BTreeSet::new();
    for (request, authorization) in arbitration.requests.iter().zip(&arbitration.authorizations) {
        if !retained_request_keys.insert(request.key.clone()) || authorization.key != request.key {
            return Err(water_protocol_failure(
                DirectSurfaceLiquidErrorCode::E005,
                DirectSurfaceLiquidPhase::ResourceCandidate,
                transaction_id,
                &request.key,
                "duplicate request or mismatched authorization identity",
            ));
        }
    }
    let request_keys = arbitration
        .requests
        .iter()
        .map(|request| request.key.clone())
        .collect::<BTreeSet<_>>();
    let mut finalized_keys = BTreeSet::new();
    for finalized in finalized_uses {
        if !finalized_keys.insert(finalized.key.clone()) {
            return Err(water_protocol_failure(
                DirectSurfaceLiquidErrorCode::E005,
                DirectSurfaceLiquidPhase::ResourceCandidate,
                transaction_id,
                &finalized.key,
                "duplicate finalized use",
            ));
        }
        if !request_keys.contains(&finalized.key) {
            return Err(water_protocol_failure(
                DirectSurfaceLiquidErrorCode::E005,
                DirectSurfaceLiquidPhase::ResourceCandidate,
                transaction_id,
                &finalized.key,
                "use without request",
            ));
        }
    }
    if finalized_uses.len() != arbitration.requests.len() {
        return Err(incomplete_protocol_failure(configuration, transaction_id));
    }
    let mut condensation_keys = BTreeSet::new();
    for credit in condensation_credits {
        let store = condensation_store_key(configuration, credit).ok_or_else(|| {
            condensation_failure(
                configuration,
                transaction_id,
                credit,
                DirectSurfaceLiquidErrorCode::E002,
                "condensation store missing",
            )
        })?;
        if !condensation_keys.insert(store) {
            return Err(condensation_failure(
                configuration,
                transaction_id,
                credit,
                DirectSurfaceLiquidErrorCode::E005,
                "duplicate condensation credit",
            ));
        }
    }
    Ok(())
}

fn validate_bounds(
    configuration: &DirectSurfaceLiquidConfiguration,
    arbitration: &DirectSurfaceLiquidArbitration,
    finalized_uses: &[WaterAmount],
    condensation_credits: &[CondensationCredit],
) -> Result<(), DirectSurfaceLiquidError> {
    let transaction_id = arbitration.transaction_id;
    let request_map = arbitration
        .requests
        .iter()
        .enumerate()
        .map(|(index, request)| (request.key.clone(), index))
        .collect::<BTreeMap<_, _>>();
    for (request, authorization) in arbitration.requests.iter().zip(&arbitration.authorizations) {
        if request.amount_kg_m2_stand_ground < 0.0
            || authorization.amount_kg_m2_stand_ground < 0.0
            || authorization.amount_kg_m2_stand_ground > request.amount_kg_m2_stand_ground
        {
            return Err(water_protocol_failure(
                DirectSurfaceLiquidErrorCode::E006,
                DirectSurfaceLiquidPhase::ResourceCandidate,
                transaction_id,
                &request.key,
                "A <= D and nonnegative retained amounts",
            ));
        }
    }
    for finalized in finalized_uses {
        let index = request_map[&finalized.key];
        if finalized.amount_kg_m2_stand_ground < 0.0
            || finalized.amount_kg_m2_stand_ground
                > arbitration.authorizations[index].amount_kg_m2_stand_ground
        {
            return Err(water_protocol_failure(
                DirectSurfaceLiquidErrorCode::E006,
                DirectSurfaceLiquidPhase::ResourceCandidate,
                transaction_id,
                &finalized.key,
                "F <= A <= D",
            ));
        }
    }
    for credit in condensation_credits {
        if credit.amount_kg_m2_stand_ground <= 0.0
            || credit.amount_basis != StandGroundWaterAmountBasis::KgH2oM2StandGroundInterval
        {
            return Err(condensation_failure(
                configuration,
                transaction_id,
                credit,
                DirectSurfaceLiquidErrorCode::E006,
                "nonpositive condensation amount or wrong basis",
            ));
        }
    }
    Ok(())
}

fn incomplete_protocol_failure(
    configuration: &DirectSurfaceLiquidConfiguration,
    transaction_id: TransactionId,
) -> DirectSurfaceLiquidError {
    DirectSurfaceLiquidError::canonical_failure(
        DirectSurfaceLiquidErrorCode::E005,
        DirectSurfaceLiquidPhase::ResourceCandidate,
        DirectSurfaceLiquidErrorContext {
            transaction_id: Some(transaction_id),
            owner_id: Some(configuration.owner_id.clone()),
            ..DirectSurfaceLiquidErrorContext::default()
        },
        DirectSurfaceLiquidRollbackHashes {
            beginning_owner_sha256: None,
            attempted_owner_sha256: None,
        },
        "incomplete retained request/authorization/finalized-use protocol",
    )
}

fn condensation_store_key(
    configuration: &DirectSurfaceLiquidConfiguration,
    credit: &CondensationCredit,
) -> Option<DirectSurfaceLiquidStoreKey> {
    configuration
        .records
        .iter()
        .find(|record| {
            record.key.ofe_id == credit.ofe_id
                && record.key.tile_id == credit.tile_id
                && record.key.surface_id == credit.surface_id
        })
        .map(|record| record.key.clone())
}

fn condensation_failure(
    configuration: &DirectSurfaceLiquidConfiguration,
    transaction_id: TransactionId,
    credit: &CondensationCredit,
    code: DirectSurfaceLiquidErrorCode,
    detail: &'static str,
) -> DirectSurfaceLiquidError {
    DirectSurfaceLiquidError::canonical_failure(
        code,
        DirectSurfaceLiquidPhase::ResourceCandidate,
        DirectSurfaceLiquidErrorContext {
            transaction_id: Some(transaction_id),
            owner_id: Some(configuration.owner_id.clone()),
            ofe_id: Some(credit.ofe_id.clone()),
            tile_id: Some(credit.tile_id.clone()),
            surface_id: Some(credit.surface_id.clone()),
            source_id: condensation_store_key(configuration, credit).map(|store| store.source_id),
            parcel_id: None,
        },
        DirectSurfaceLiquidRollbackHashes {
            beginning_owner_sha256: None,
            attempted_owner_sha256: None,
        },
        detail,
    )
}
