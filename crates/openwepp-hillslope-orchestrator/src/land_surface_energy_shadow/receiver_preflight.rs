use std::collections::BTreeMap;

use crate::vegetation_real_hydrology_shadow::RealHydrologyShadowAdapter;

use super::{
    DirectRunFrame, DirectSurfaceLiquidConfiguration, DirectSurfaceLiquidErrorCode,
    DirectSurfaceLiquidIngressCandidate, DirectSurfaceLiquidParcelReceipt,
    DirectSurfaceLiquidReceiptDisposition, LandSurfaceEnergyShadowError, OfeId,
    ReceiverFailureScope, Sha256Digest, SoilThermalTileCandidate, TileId, TileState,
    UnifiedReceiverExpectations, WATER_DENSITY_KG_M3, apply_production_infiltration,
    apply_receiver_receipt, checked_surface_liquid_add, checked_surface_liquid_div,
};

pub(super) type OfeAmountMap = BTreeMap<OfeId, f64>;
pub(super) type TileAmountMap = BTreeMap<(OfeId, TileId), f64>;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(super) enum ReceiptAggregationOwner {
    SurfaceLiquid,
    LandSurfaceEnergy,
    SoilThermal,
}

#[derive(Clone, Debug, PartialEq)]
pub(super) struct ReceiverReceiptAggregates {
    pub(super) infiltration_m_by_ofe: OfeAmountMap,
    pub(super) infiltration_enthalpy_by_tile: TileAmountMap,
    pub(super) retained_enthalpy_by_tile: TileAmountMap,
}

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
    aggregate_receiver_receipts(ingress.receipts(), |owner, receipt, detail| {
        let owner_id = match owner {
            ReceiptAggregationOwner::SurfaceLiquid => &scope.configuration.owner_id,
            ReceiptAggregationOwner::LandSurfaceEnergy => &scope.expectations.lse_owner_id,
            ReceiptAggregationOwner::SoilThermal => &scope.expectations.soil_thermal_owner_id,
        };
        scope.failure(
            DirectSurfaceLiquidErrorCode::E003,
            owner_id,
            receipt,
            detail,
        )
    })?;

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

pub(super) fn aggregate_receiver_receipts<'a, E>(
    receipts: impl IntoIterator<Item = &'a DirectSurfaceLiquidParcelReceipt>,
    mut failure: impl FnMut(
        ReceiptAggregationOwner,
        &DirectSurfaceLiquidParcelReceipt,
        &'static str,
    ) -> E,
) -> Result<ReceiverReceiptAggregates, E> {
    let mut infiltration_m_by_ofe = BTreeMap::<OfeId, f64>::new();
    let mut infiltration_enthalpy_by_tile = BTreeMap::<(OfeId, TileId), f64>::new();
    let mut retained_enthalpy_by_tile = BTreeMap::<(OfeId, TileId), f64>::new();
    for receipt in receipts {
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
                    failure(
                        ReceiptAggregationOwner::SoilThermal,
                        receipt,
                        "infiltration receipt mass-to-depth arithmetic",
                    )
                })?;
                accumulate_receiver_amount(
                    &mut failure,
                    ReceiptAggregationOwner::SurfaceLiquid,
                    receipt,
                    infiltration_m_by_ofe
                        .entry(receipt.recipient_store_key.ofe_id.clone())
                        .or_default(),
                    infiltration_m,
                    "infiltration OFE accumulation is nonfinite or underflowed",
                )?;
                accumulate_receiver_amount(
                    &mut failure,
                    ReceiptAggregationOwner::SoilThermal,
                    receipt,
                    infiltration_enthalpy_by_tile.entry(tile_key).or_default(),
                    receipt.enthalpy_j_m2_basis_ofe_ground,
                    "infiltration enthalpy accumulation is nonfinite or underflowed",
                )?;
            }
            DirectSurfaceLiquidReceiptDisposition::RetainedSurface => {
                accumulate_receiver_amount(
                    &mut failure,
                    ReceiptAggregationOwner::LandSurfaceEnergy,
                    receipt,
                    retained_enthalpy_by_tile.entry(tile_key).or_default(),
                    receipt.enthalpy_j_m2_basis_ofe_ground,
                    "retained enthalpy accumulation is nonfinite or underflowed",
                )?;
            }
            DirectSurfaceLiquidReceiptDisposition::RoutedRunoff
            | DirectSurfaceLiquidReceiptDisposition::OutletRunoff => {}
        }
    }
    Ok(ReceiverReceiptAggregates {
        infiltration_m_by_ofe,
        infiltration_enthalpy_by_tile,
        retained_enthalpy_by_tile,
    })
}

fn accumulate_receiver_amount<E>(
    failure: &mut impl FnMut(
        ReceiptAggregationOwner,
        &DirectSurfaceLiquidParcelReceipt,
        &'static str,
    ) -> E,
    owner: ReceiptAggregationOwner,
    receipt: &DirectSurfaceLiquidParcelReceipt,
    accumulated: &mut f64,
    amount: f64,
    detail: &'static str,
) -> Result<(), E> {
    *accumulated = checked_surface_liquid_add(*accumulated, amount)
        .ok_or_else(|| failure(owner, receipt, detail))?;
    Ok(())
}

fn is_receiver_arithmetic_error(error: &LandSurfaceEnergyShadowError) -> bool {
    matches!(
        error,
        LandSurfaceEnergyShadowError::SurfaceLiquid(error)
            if error.code() == DirectSurfaceLiquidErrorCode::E003
    )
}

#[cfg(test)]
mod tests {
    use openwepp_kernel_contract::{SoilLayerId, TransactionId};

    use crate::{
        DirectSurfaceLiquidParcelKind, DirectSurfaceLiquidReceiptRecipient,
        DirectSurfaceLiquidStoreKey,
    };

    use super::*;
    use crate::land_surface_energy_shadow::{SourceId, SurfaceClass, SurfaceId, WaterSourceType};

    fn receipt(
        index: usize,
        disposition: DirectSurfaceLiquidReceiptDisposition,
        mass: f64,
        enthalpy: f64,
    ) -> DirectSurfaceLiquidParcelReceipt {
        let ofe_id = OfeId::try_new("ofe-aggregate").expect("OFE");
        let tile_id = TileId::try_new("tile-aggregate").expect("tile");
        let store_key = DirectSurfaceLiquidStoreKey {
            run_id: 71,
            ofe_id: ofe_id.clone(),
            tile_id: tile_id.clone(),
            surface_id: SurfaceId::try_new("surface-aggregate").expect("surface"),
            surface_class: SurfaceClass::BareMineralSoil,
            source_type: WaterSourceType::SurfaceLiquid,
            source_id: SourceId::try_new("source-aggregate").expect("source"),
        };
        let recipient = if disposition == DirectSurfaceLiquidReceiptDisposition::Infiltration {
            DirectSurfaceLiquidReceiptRecipient::SoilInfiltration {
                ofe_id: ofe_id.clone(),
                production_lane_index: 0,
                production_lane_id: 1,
                ordered_soil_layer_ids: vec![
                    SoilLayerId::try_new("soil-aggregate").expect("soil layer"),
                ],
                soil_thermal_layer_id: SoilLayerId::try_new("soil-aggregate").expect("soil layer"),
            }
        } else {
            DirectSurfaceLiquidReceiptRecipient::SurfaceStore {
                store_key: store_key.clone(),
            }
        };
        DirectSurfaceLiquidParcelReceipt {
            parcel_id: format!("parcel-{index}"),
            source_parcel_id: format!("source-parcel-{index}"),
            transaction_id: TransactionId(71),
            origin_store_key: store_key.clone(),
            recipient_store_key: store_key,
            recipient,
            basis_ofe_id: ofe_id,
            kind: DirectSurfaceLiquidParcelKind::RawPrecipitation,
            disposition,
            start_s: 0.0,
            end_s: 1_800.0,
            mass_kg_m2_basis_ofe_ground: mass,
            temperature_k: 273.15,
            enthalpy_j_m2_basis_ofe_ground: enthalpy,
        }
    }

    #[test]
    fn shared_aggregation_is_context_independent_and_bit_exact() {
        let source_mass = f64::from_bits(0x3f9f_9e1d_f20c_7aa4);
        let receipts = vec![
            receipt(
                0,
                DirectSurfaceLiquidReceiptDisposition::Infiltration,
                source_mass,
                3.0,
            ),
            receipt(
                1,
                DirectSurfaceLiquidReceiptDisposition::Infiltration,
                0.125,
                5.0,
            ),
            receipt(
                2,
                DirectSurfaceLiquidReceiptDisposition::RetainedSurface,
                0.25,
                7.0,
            ),
        ];
        let preflight = aggregate_receiver_receipts(&receipts, |_, _, _| "preflight")
            .expect("preflight aggregation");
        let frozen =
            aggregate_receiver_receipts(&receipts, |_, _, _| "frozen").expect("frozen aggregation");
        assert_eq!(preflight, frozen, "failure context cannot drift arithmetic");

        let ofe_id = OfeId::try_new("ofe-aggregate").expect("OFE");
        let tile_id = TileId::try_new("tile-aggregate").expect("tile");
        let expected_depth = source_mass / WATER_DENSITY_KG_M3 + 0.125 / WATER_DENSITY_KG_M3;
        assert_eq!(
            preflight.infiltration_m_by_ofe[&ofe_id].to_bits(),
            expected_depth.to_bits()
        );
        assert_eq!(
            preflight.infiltration_enthalpy_by_tile[&(ofe_id.clone(), tile_id.clone())].to_bits(),
            8.0_f64.to_bits()
        );
        assert_eq!(
            preflight.retained_enthalpy_by_tile[&(ofe_id, tile_id)].to_bits(),
            7.0_f64.to_bits()
        );
    }

    #[test]
    fn shared_aggregation_poison_reports_first_e003_owner_and_receipt() {
        let receipts = vec![
            receipt(
                0,
                DirectSurfaceLiquidReceiptDisposition::Infiltration,
                f64::from_bits(1),
                0.0,
            ),
            receipt(
                1,
                DirectSurfaceLiquidReceiptDisposition::Infiltration,
                1.0,
                f64::MAX,
            ),
            receipt(
                2,
                DirectSurfaceLiquidReceiptDisposition::Infiltration,
                1.0,
                f64::MAX,
            ),
        ];
        let error = aggregate_receiver_receipts(&receipts, |owner, receipt, detail| {
            (
                DirectSurfaceLiquidErrorCode::E003,
                owner,
                receipt.parcel_id.clone(),
                detail,
            )
        })
        .expect_err("mass-to-depth underflow must outrank later enthalpy overflow");
        assert_eq!(error.0, DirectSurfaceLiquidErrorCode::E003);
        assert_eq!(error.1, ReceiptAggregationOwner::SoilThermal);
        assert_eq!(error.2, "parcel-0");
        assert_eq!(error.3, "infiltration receipt mass-to-depth arithmetic");
    }
}
