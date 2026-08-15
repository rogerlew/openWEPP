use std::collections::BTreeMap;

use crate::vegetation_real_hydrology_shadow::RealHydrologyShadowAdapter;

use super::{
    DirectRunFrame, DirectSurfaceLiquidConfiguration, DirectSurfaceLiquidErrorCode,
    DirectSurfaceLiquidIngressCandidate, DirectSurfaceLiquidParcelReceipt,
    DirectSurfaceLiquidReceiptDisposition, LandSurfaceEnergyShadowError, OfeId,
    ReceiverFailureScope, ResourceOwnerId, Sha256Digest, SoilThermalTileCandidate, TileId,
    TileState, UnifiedReceiverExpectations, WATER_DENSITY_KG_M3, apply_production_infiltration,
    apply_receiver_receipt, checked_surface_liquid_add, checked_surface_liquid_div,
};

#[allow(clippy::too_many_arguments)]
pub(super) fn preflight_receiver_derived_arithmetic(
    owner: &RealHydrologyShadowAdapter,
    configuration: &DirectSurfaceLiquidConfiguration,
    receiver_expectations: &UnifiedReceiverExpectations,
    ingress: &DirectSurfaceLiquidIngressCandidate,
    ending_frame: &DirectRunFrame,
    lse_tiles: &[TileState],
    soil_thermal: &[SoilThermalTileCandidate],
    beginning_hydrology_snapshot_sha256: &Sha256Digest,
    receiver_attempt_sha256: &str,
) -> Result<(), LandSurfaceEnergyShadowError> {
    let scope = ReceiverFailureScope {
        transaction_id: ingress.transaction_id(),
        configuration,
        expectations: receiver_expectations,
        beginning_sha256: beginning_hydrology_snapshot_sha256.as_str(),
        attempted_sha256: receiver_attempt_sha256,
    };
    preflight_receipt_aggregations(&scope, ingress)?;

    let mut trial_lse = lse_tiles.to_vec();
    let mut trial_thermal = soil_thermal.to_vec();
    let mut infiltration_m_by_lane =
        BTreeMap::<usize, (f64, &DirectSurfaceLiquidParcelReceipt)>::new();
    for receipt in ingress.receipts() {
        match apply_receiver_receipt(&scope, receipt, &mut trial_lse, &mut trial_thermal) {
            Ok(Some((lane_index, infiltration_m))) => {
                let accumulated = infiltration_m_by_lane
                    .entry(lane_index)
                    .or_insert((0.0, receipt));
                accumulated.0 = checked_surface_liquid_add(accumulated.0, infiltration_m)
                    .ok_or_else(|| {
                        scope.failure(
                            DirectSurfaceLiquidErrorCode::E003,
                            owner.hydrology_owner_id(),
                            receipt,
                            "infiltration lane accumulation is nonfinite or underflowed",
                        )
                    })?;
                accumulated.1 = receipt;
            }
            Err(error) if is_receiver_arithmetic_error(&error) => return Err(error),
            Ok(None) | Err(_) => {}
        }
    }
    let mut trial_frame = ending_frame.clone();
    for (lane_index, accumulated) in infiltration_m_by_lane {
        let one_lane = BTreeMap::from([(lane_index, accumulated)]);
        if let Err(error) = apply_production_infiltration(owner, &scope, &mut trial_frame, one_lane)
        {
            if is_receiver_arithmetic_error(&error) {
                return Err(error);
            }
        }
    }
    Ok(())
}

fn preflight_receipt_aggregations(
    scope: &ReceiverFailureScope<'_>,
    ingress: &DirectSurfaceLiquidIngressCandidate,
) -> Result<(), LandSurfaceEnergyShadowError> {
    let mut infiltration_m_by_ofe = BTreeMap::<OfeId, f64>::new();
    let mut infiltration_enthalpy_by_tile = BTreeMap::<(OfeId, TileId), f64>::new();
    let mut retained_enthalpy_by_tile = BTreeMap::<(OfeId, TileId), f64>::new();
    for receipt in ingress.receipts() {
        let tile_key = (
            receipt.recipient_store_key.ofe_id.clone(),
            receipt.recipient_store_key.tile_id.clone(),
        );
        match receipt.disposition {
            DirectSurfaceLiquidReceiptDisposition::Infiltration => {
                let infiltration_m = checked_surface_liquid_div(
                    receipt.mass_kg_m2_basis_ofe_ground,
                    WATER_DENSITY_KG_M3,
                )
                .ok_or_else(|| {
                    scope.failure(
                        DirectSurfaceLiquidErrorCode::E003,
                        &scope.expectations.soil_thermal_owner_id,
                        receipt,
                        "infiltration receipt mass-to-depth arithmetic",
                    )
                })?;
                accumulate_receiver_preflight(
                    scope,
                    receipt,
                    &scope.configuration.owner_id,
                    infiltration_m_by_ofe
                        .entry(receipt.recipient_store_key.ofe_id.clone())
                        .or_default(),
                    infiltration_m,
                    "infiltration OFE accumulation is nonfinite or underflowed",
                )?;
                accumulate_receiver_preflight(
                    scope,
                    receipt,
                    &scope.expectations.soil_thermal_owner_id,
                    infiltration_enthalpy_by_tile.entry(tile_key).or_default(),
                    receipt.enthalpy_j_m2_basis_ofe_ground,
                    "infiltration enthalpy accumulation is nonfinite or underflowed",
                )?;
            }
            DirectSurfaceLiquidReceiptDisposition::RetainedSurface => {
                accumulate_receiver_preflight(
                    scope,
                    receipt,
                    &scope.expectations.lse_owner_id,
                    retained_enthalpy_by_tile.entry(tile_key).or_default(),
                    receipt.enthalpy_j_m2_basis_ofe_ground,
                    "retained enthalpy accumulation is nonfinite or underflowed",
                )?;
            }
            DirectSurfaceLiquidReceiptDisposition::RoutedRunoff
            | DirectSurfaceLiquidReceiptDisposition::OutletRunoff => {}
        }
    }
    Ok(())
}

fn accumulate_receiver_preflight(
    scope: &ReceiverFailureScope<'_>,
    receipt: &DirectSurfaceLiquidParcelReceipt,
    owner_id: &ResourceOwnerId,
    accumulated: &mut f64,
    amount: f64,
    detail: &'static str,
) -> Result<(), LandSurfaceEnergyShadowError> {
    *accumulated = checked_surface_liquid_add(*accumulated, amount).ok_or_else(|| {
        scope.failure(
            DirectSurfaceLiquidErrorCode::E003,
            owner_id,
            receipt,
            detail,
        )
    })?;
    Ok(())
}

fn is_receiver_arithmetic_error(error: &LandSurfaceEnergyShadowError) -> bool {
    matches!(
        error,
        LandSurfaceEnergyShadowError::SurfaceLiquid(error)
            if error.code() == DirectSurfaceLiquidErrorCode::E003
    )
}
