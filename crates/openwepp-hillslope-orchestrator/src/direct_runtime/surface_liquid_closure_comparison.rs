//! Terminal comparisons required by `SC-SURFACELIQUID-001` independent closure.

use openwepp_kernel_contract::{ResourceOwnerId, TransactionId};

use super::{
    DirectSurfaceLiquidClosureUnit, DirectSurfaceLiquidConfigurationRecord,
    DirectSurfaceLiquidError, DirectSurfaceLiquidErrorCode, DirectSurfaceLiquidParcelReceipt,
    DirectSurfaceLiquidStoreKey, LIQUID_HEAT_CAPACITY_J_KG_K, REFERENCE_TEMPERATURE_K,
    checked_surface_liquid_close, checked_surface_liquid_mul, checked_surface_liquid_sub,
    contextual_closure_arithmetic_failure, contextual_comparison_failure,
};

pub(super) fn validate_receipt_enthalpy(
    owner_id: &ResourceOwnerId,
    receipt: &DirectSurfaceLiquidParcelReceipt,
) -> Result<(), DirectSurfaceLiquidError> {
    if !receipt.mass_kg_m2_basis_ofe_ground.is_finite()
        || receipt.mass_kg_m2_basis_ofe_ground < 0.0
        || !receipt.enthalpy_j_m2_basis_ofe_ground.is_finite()
        || !receipt.temperature_k.is_finite()
    {
        return Err(DirectSurfaceLiquidError::Closure(
            "nonfinite or negative parcel receipt",
        ));
    }
    if receipt.mass_kg_m2_basis_ofe_ground == 0.0 {
        if receipt.enthalpy_j_m2_basis_ofe_ground.to_bits() != 0.0_f64.to_bits() {
            return Err(DirectSurfaceLiquidError::Closure(
                "zero-mass parcel carries enthalpy",
            ));
        }
        return Ok(());
    }
    let expected = checked_surface_liquid_sub(receipt.temperature_k, REFERENCE_TEMPERATURE_K)
        .and_then(|delta| checked_surface_liquid_mul(LIQUID_HEAT_CAPACITY_J_KG_K, delta))
        .and_then(|specific| {
            checked_surface_liquid_mul(receipt.mass_kg_m2_basis_ofe_ground, specific)
        })
        .ok_or_else(|| {
            contextual_closure_arithmetic_failure(
                receipt.transaction_id,
                &receipt.origin_store_key,
                Some(receipt.parcel_id.clone()),
                "parcel temperature/enthalpy arithmetic is nonfinite or underflowed",
            )
        })?;
    require_close_enthalpy(
        receipt.enthalpy_j_m2_basis_ofe_ground,
        expected,
        receipt.transaction_id,
        owner_id,
        &receipt.origin_store_key,
        Some(receipt.parcel_id.clone()),
        "parcel temperature/enthalpy join",
    )
}

pub(super) fn water_key_matches_record(
    key: &openwepp_land_surface_energy::GroundWaterKey,
    record: &DirectSurfaceLiquidConfigurationRecord,
) -> bool {
    key.ofe_id == record.key.ofe_id
        && key.source_tile_id.as_ref() == Some(&record.key.tile_id)
        && key.surface_id.as_ref() == Some(&record.key.surface_id)
        && key.surface_class == Some(record.key.surface_class)
        && key.source_type == record.key.source_type
        && key.source_id == record.key.source_id
}

pub(super) fn require_close_mass(
    actual: f64,
    expected: f64,
    transaction_id: TransactionId,
    owner_id: &ResourceOwnerId,
    store_key: &DirectSurfaceLiquidStoreKey,
    parcel_id: Option<String>,
    detail: &'static str,
) -> Result<(), DirectSurfaceLiquidError> {
    match checked_surface_liquid_close(actual, expected, DirectSurfaceLiquidClosureUnit::MassKgM2) {
        Some(true) => Ok(()),
        Some(false) => Err(contextual_comparison_failure(
            DirectSurfaceLiquidErrorCode::E010,
            transaction_id,
            owner_id,
            store_key,
            parcel_id,
            detail,
        )),
        None => Err(contextual_comparison_failure(
            DirectSurfaceLiquidErrorCode::E003,
            transaction_id,
            owner_id,
            store_key,
            parcel_id,
            detail,
        )),
    }
}

fn require_close_enthalpy(
    actual: f64,
    expected: f64,
    transaction_id: TransactionId,
    owner_id: &ResourceOwnerId,
    store_key: &DirectSurfaceLiquidStoreKey,
    parcel_id: Option<String>,
    detail: &'static str,
) -> Result<(), DirectSurfaceLiquidError> {
    match checked_surface_liquid_close(
        actual,
        expected,
        DirectSurfaceLiquidClosureUnit::EnthalpyJM2,
    ) {
        Some(true) => Ok(()),
        Some(false) => Err(contextual_comparison_failure(
            DirectSurfaceLiquidErrorCode::E010,
            transaction_id,
            owner_id,
            store_key,
            parcel_id,
            detail,
        )),
        None => Err(contextual_comparison_failure(
            DirectSurfaceLiquidErrorCode::E003,
            transaction_id,
            owner_id,
            store_key,
            parcel_id,
            detail,
        )),
    }
}
