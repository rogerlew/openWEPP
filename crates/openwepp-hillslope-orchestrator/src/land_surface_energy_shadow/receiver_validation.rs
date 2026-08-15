use super::{
    Digest, DirectSurfaceLiquidClosureUnit, DirectSurfaceLiquidConfiguration,
    DirectSurfaceLiquidError, DirectSurfaceLiquidErrorCode, DirectSurfaceLiquidErrorContext,
    DirectSurfaceLiquidPhase, DirectSurfaceLiquidRollbackHashes, OfeId,
    ProductionSoilLayerReceiverOperands, ProductionSoilReceiverOperands,
    RealReceiverClosureOperands, ResourceOwnerId, Sha256, SoilLayerId, TileId,
    checked_surface_liquid_add, checked_surface_liquid_close, checked_surface_liquid_div,
    checked_surface_liquid_mul, checked_surface_liquid_sub, checked_surface_liquid_sum,
};

/// Independently reconstruct all real receiver ending equations from frozen operands.
pub fn validate_real_receiver_closure(
    operands: &RealReceiverClosureOperands,
) -> Result<(), DirectSurfaceLiquidError> {
    validate_receiver_envelope(operands)?;
    validate_production_soil_receiver_closure(operands)?;
    for thermal in &operands.soil_thermal {
        let expected_credit = checked_surface_liquid_add(
            thermal.beginning_infiltration_credit_j_m2_ofe_ground,
            thermal.infiltration_enthalpy_j_m2_ofe_ground,
        );
        let expected_ending = checked_surface_liquid_add(
            thermal.beginning_enthalpy_j_m2_ofe_ground,
            thermal.infiltration_enthalpy_j_m2_ofe_ground,
        );
        let (expected_credit, expected_ending) =
            expected_credit.zip(expected_ending).ok_or_else(|| {
                receiver_arithmetic_failure(
                    operands,
                    Some(&thermal.ofe_id),
                    Some(&thermal.tile_id),
                    "soil-thermal infiltration enthalpy arithmetic",
                )
            })?;
        require_receiver_close(
            operands,
            thermal.ending_infiltration_credit_j_m2_ofe_ground,
            expected_credit,
            DirectSurfaceLiquidClosureUnit::EnthalpyJM2,
            Some(&thermal.ofe_id),
            Some(&thermal.tile_id),
            "soil-thermal infiltration-credit ending equation",
        )?;
        require_receiver_close(
            operands,
            thermal.ending_enthalpy_j_m2_ofe_ground,
            expected_ending,
            DirectSurfaceLiquidClosureUnit::EnthalpyJM2,
            Some(&thermal.ofe_id),
            Some(&thermal.tile_id),
            "soil-thermal infiltration enthalpy ending equation",
        )?;
    }
    for tile in &operands.lse_tiles {
        if !tile.tile_fraction.is_finite() || tile.tile_fraction <= 0.0 {
            return Err(receiver_atomic_failure(
                operands,
                Some(&tile.ofe_id),
                Some(&tile.tile_id),
                "LSE retained tile fraction",
            ));
        }
        let expected =
            checked_surface_liquid_div(tile.retained_enthalpy_j_m2_ofe_ground, tile.tile_fraction)
                .and_then(|retained| {
                    checked_surface_liquid_add(tile.beginning_enthalpy_j_m2_tile_ground, retained)
                })
                .ok_or_else(|| {
                    receiver_arithmetic_failure(
                        operands,
                        Some(&tile.ofe_id),
                        Some(&tile.tile_id),
                        "LSE retained enthalpy arithmetic",
                    )
                })?;
        require_receiver_close(
            operands,
            tile.ending_enthalpy_j_m2_tile_ground,
            expected,
            DirectSurfaceLiquidClosureUnit::EnthalpyJM2,
            Some(&tile.ofe_id),
            Some(&tile.tile_id),
            "LSE retained enthalpy ending equation",
        )?;
    }
    Ok(())
}

#[allow(clippy::too_many_lines)]
fn validate_production_soil_receiver_closure(
    operands: &RealReceiverClosureOperands,
) -> Result<(), DirectSurfaceLiquidError> {
    for lane in &operands.production_soil {
        if lane.ordered_layers.is_empty()
            || !lane.infiltration_m.is_finite()
            || lane.infiltration_m < 0.0
            || !lane.tillage_depth_m.is_finite()
        {
            return Err(receiver_atomic_failure(
                operands,
                Some(&lane.ofe_id),
                None,
                "production soil receiver operand domain",
            ));
        }
        let expected = independently_reconstruct_infiltration(lane).ok_or_else(|| {
            receiver_atomic_failure(
                operands,
                Some(&lane.ofe_id),
                None,
                "production soil receiver reconstruction domain",
            )
        })?;
        for (layer, expected_ending) in lane.ordered_layers.iter().zip(expected) {
            require_receiver_close(
                operands,
                layer.ending_liquid_m,
                expected_ending,
                DirectSurfaceLiquidClosureUnit::MassM,
                Some(&lane.ofe_id),
                None,
                "ordered production soil-layer infiltration equation",
            )?;
        }
        let beginning_terms = lane
            .ordered_layers
            .iter()
            .map(|layer| checked_receiver_layer_total(layer.beginning_liquid_m, layer))
            .collect::<Option<Vec<_>>>();
        let ending_terms = lane
            .ordered_layers
            .iter()
            .map(|layer| checked_receiver_layer_total(layer.ending_liquid_m, layer))
            .collect::<Option<Vec<_>>>();
        let beginning_sum = beginning_terms
            .and_then(checked_surface_liquid_sum)
            .ok_or_else(|| {
                receiver_arithmetic_failure(
                    operands,
                    Some(&lane.ofe_id),
                    None,
                    "beginning aggregate soil-water arithmetic",
                )
            })?;
        let ending_sum = ending_terms
            .and_then(checked_surface_liquid_sum)
            .ok_or_else(|| {
                receiver_arithmetic_failure(
                    operands,
                    Some(&lane.ofe_id),
                    None,
                    "ending aggregate soil-water arithmetic",
                )
            })?;
        let expected_aggregate_ending =
            checked_surface_liquid_add(lane.beginning_aggregate_soil_water_m, lane.infiltration_m)
                .ok_or_else(|| {
                    receiver_arithmetic_failure(
                        operands,
                        Some(&lane.ofe_id),
                        None,
                        "aggregate soil-water ending arithmetic",
                    )
                })?;
        require_receiver_close(
            operands,
            lane.beginning_aggregate_soil_water_m,
            beginning_sum,
            DirectSurfaceLiquidClosureUnit::MassM,
            Some(&lane.ofe_id),
            None,
            "beginning aggregate production soil-water equation",
        )?;
        require_receiver_close(
            operands,
            lane.ending_aggregate_soil_water_m,
            ending_sum,
            DirectSurfaceLiquidClosureUnit::MassM,
            Some(&lane.ofe_id),
            None,
            "ending aggregate production soil-water equation",
        )?;
        require_receiver_close(
            operands,
            lane.ending_aggregate_soil_water_m,
            expected_aggregate_ending,
            DirectSurfaceLiquidClosureUnit::MassM,
            Some(&lane.ofe_id),
            None,
            "aggregate production soil-water ending equation",
        )?;
    }
    Ok(())
}

fn checked_receiver_layer_total(
    liquid_m: f64,
    layer: &ProductionSoilLayerReceiverOperands,
) -> Option<f64> {
    let unfrozen_depth = checked_surface_liquid_sub(layer.layer_depth_m, layer.frozen_depth_m)?;
    let residual = checked_surface_liquid_mul(layer.residual_theta, unfrozen_depth.max(0.0))?;
    checked_surface_liquid_add(liquid_m, residual)
}

fn independently_reconstruct_infiltration(
    lane: &ProductionSoilReceiverOperands,
) -> Option<Vec<f64>> {
    let first_depth = lane.ordered_layers.first()?.layer_depth_m;
    let resolved_tillage_depth_m = if lane.tillage_depth_m > 1.0e-12 {
        lane.tillage_depth_m
    } else {
        first_depth
    };
    if !resolved_tillage_depth_m.is_finite() || resolved_tillage_depth_m <= 0.0 {
        return None;
    }
    let mut remaining = lane.infiltration_m;
    let mut cumulative_depth_m = 0.0;
    let mut expected = lane
        .ordered_layers
        .iter()
        .map(|layer| layer.beginning_liquid_m)
        .collect::<Vec<_>>();
    for (layer, ending) in lane.ordered_layers.iter().zip(&mut expected) {
        if remaining <= 0.0 {
            break;
        }
        if !layer.layer_depth_m.is_finite()
            || layer.layer_depth_m <= 0.0
            || !layer.residual_theta.is_finite()
            || layer.residual_theta < 0.0
            || !layer.frozen_depth_m.is_finite()
            || layer.frozen_depth_m < 0.0
        {
            return None;
        }
        cumulative_depth_m = checked_surface_liquid_add(cumulative_depth_m, layer.layer_depth_m)?;
        let addition = if cumulative_depth_m < resolved_tillage_depth_m - 1.0e-12 {
            checked_surface_liquid_mul(remaining, layer.layer_depth_m)
                .and_then(|value| checked_surface_liquid_div(value, resolved_tillage_depth_m))?
        } else {
            remaining
        };
        *ending = checked_surface_liquid_add(*ending, addition.max(0.0))?;
        remaining = checked_surface_liquid_sub(remaining, addition)?;
    }
    if remaining > 0.0 {
        let last = expected.last_mut()?;
        *last = checked_surface_liquid_add(*last, remaining)?;
    }
    Some(expected)
}

#[allow(clippy::too_many_arguments)]
pub(super) fn require_receiver_close(
    operands: &RealReceiverClosureOperands,
    actual: f64,
    expected: f64,
    unit: DirectSurfaceLiquidClosureUnit,
    ofe_id: Option<&OfeId>,
    tile_id: Option<&TileId>,
    detail: &'static str,
) -> Result<(), DirectSurfaceLiquidError> {
    match checked_surface_liquid_close(actual, expected, unit) {
        Some(true) => Ok(()),
        Some(false) => Err(receiver_atomic_failure(operands, ofe_id, tile_id, detail)),
        None => Err(receiver_arithmetic_failure(
            operands, ofe_id, tile_id, detail,
        )),
    }
}

pub(super) fn receiver_atomic_failure(
    operands: &RealReceiverClosureOperands,
    ofe_id: Option<&OfeId>,
    tile_id: Option<&TileId>,
    detail: &'static str,
) -> DirectSurfaceLiquidError {
    DirectSurfaceLiquidError::atomic_envelope_failure(
        DirectSurfaceLiquidErrorContext {
            transaction_id: Some(operands.transaction_id),
            owner_id: Some(operands.hydrology_owner_id.clone()),
            ofe_id: ofe_id.cloned(),
            tile_id: tile_id.cloned(),
            ..DirectSurfaceLiquidErrorContext::default()
        },
        Some(operands.beginning_hydrology_snapshot_sha256.to_string()),
        Some(receiver_operands_sha256(operands)),
        detail,
    )
}

fn receiver_arithmetic_failure(
    operands: &RealReceiverClosureOperands,
    ofe_id: Option<&OfeId>,
    tile_id: Option<&TileId>,
    detail: &'static str,
) -> DirectSurfaceLiquidError {
    DirectSurfaceLiquidError::canonical_failure(
        DirectSurfaceLiquidErrorCode::E003,
        DirectSurfaceLiquidPhase::IndependentClosure,
        DirectSurfaceLiquidErrorContext {
            transaction_id: Some(operands.transaction_id),
            owner_id: Some(operands.hydrology_owner_id.clone()),
            ofe_id: ofe_id.cloned(),
            tile_id: tile_id.cloned(),
            ..DirectSurfaceLiquidErrorContext::default()
        },
        DirectSurfaceLiquidRollbackHashes {
            beginning_owner_sha256: Some(operands.beginning_hydrology_snapshot_sha256.to_string()),
            attempted_owner_sha256: Some(receiver_operands_sha256(operands)),
        },
        detail,
    )
}

type ProductionIdentity = (OfeId, usize, u32, Vec<SoilLayerId>);
type ThermalIdentity = (OfeId, TileId, SoilLayerId);
type TileIdentity = (OfeId, TileId);

#[allow(clippy::type_complexity)]
pub(super) fn expected_receiver_identities(
    configuration: &DirectSurfaceLiquidConfiguration,
) -> (
    Vec<ProductionIdentity>,
    Vec<ThermalIdentity>,
    Vec<TileIdentity>,
) {
    let production = configuration
        .ofe_bindings
        .iter()
        .map(|binding| {
            (
                binding.ofe_id.clone(),
                binding.production_lane_index,
                binding.production_lane_id,
                binding.ordered_soil_layer_ids.clone(),
            )
        })
        .collect();
    let thermal = configuration
        .records
        .iter()
        .filter_map(|record| {
            configuration
                .ofe_bindings
                .iter()
                .find(|binding| binding.ofe_id == record.key.ofe_id)
                .map(|binding| {
                    (
                        record.key.ofe_id.clone(),
                        record.key.tile_id.clone(),
                        binding.infiltration_soil_thermal_layer_id.clone(),
                    )
                })
        })
        .collect();
    let lse = configuration
        .records
        .iter()
        .map(|record| (record.key.ofe_id.clone(), record.key.tile_id.clone()))
        .collect();
    (production, thermal, lse)
}

pub(super) fn validate_receiver_envelope(
    operands: &RealReceiverClosureOperands,
) -> Result<(), DirectSurfaceLiquidError> {
    validate_numeric_domains(operands)?;
    let production = operands
        .production_soil
        .iter()
        .map(|lane| {
            (
                lane.ofe_id.clone(),
                lane.production_lane_index,
                lane.production_lane_id,
                lane.ordered_layers
                    .iter()
                    .map(|layer| layer.layer_id.clone())
                    .collect(),
            )
        })
        .collect::<Vec<_>>();
    if production != operands.expected_production_soil {
        let index = first_mismatch(&operands.expected_production_soil, &production);
        let ofe = production
            .get(index)
            .or_else(|| operands.expected_production_soil.get(index))
            .map(|identity| &identity.0);
        return Err(join_failure(
            operands,
            &operands.hydrology_owner_id,
            ofe,
            None,
            "production soil receiver topology mismatch",
        ));
    }
    let thermal = operands
        .soil_thermal
        .iter()
        .map(|row| {
            (
                row.ofe_id.clone(),
                row.tile_id.clone(),
                row.layer_id.clone(),
            )
        })
        .collect::<Vec<_>>();
    if thermal != operands.expected_soil_thermal {
        let index = first_mismatch(&operands.expected_soil_thermal, &thermal);
        let identity = thermal
            .get(index)
            .or_else(|| operands.expected_soil_thermal.get(index));
        return Err(join_failure(
            operands,
            &operands.soil_thermal_owner_id,
            identity.map(|row| &row.0),
            identity.map(|row| &row.1),
            "soil-thermal receiver topology mismatch",
        ));
    }
    let lse = operands
        .lse_tiles
        .iter()
        .map(|row| (row.ofe_id.clone(), row.tile_id.clone()))
        .collect::<Vec<_>>();
    if lse != operands.expected_lse_tiles {
        let index = first_mismatch(&operands.expected_lse_tiles, &lse);
        let identity = lse
            .get(index)
            .or_else(|| operands.expected_lse_tiles.get(index));
        return Err(join_failure(
            operands,
            &operands.lse_owner_id,
            identity.map(|row| &row.0),
            identity.map(|row| &row.1),
            "LSE tile receiver topology mismatch",
        ));
    }
    Ok(())
}

fn validate_numeric_domains(
    operands: &RealReceiverClosureOperands,
) -> Result<(), DirectSurfaceLiquidError> {
    for lane in &operands.production_soil {
        let scalars = [
            lane.tillage_depth_m,
            lane.infiltration_m,
            lane.beginning_aggregate_soil_water_m,
            lane.ending_aggregate_soil_water_m,
        ];
        let invalid_layer = lane.ordered_layers.iter().any(|layer| {
            ![
                layer.beginning_liquid_m,
                layer.ending_liquid_m,
                layer.layer_depth_m,
                layer.residual_theta,
                layer.frozen_depth_m,
            ]
            .iter()
            .all(|value| value.is_finite())
                || layer.beginning_liquid_m < 0.0
                || layer.ending_liquid_m < 0.0
                || layer.layer_depth_m <= 0.0
                || layer.residual_theta < 0.0
                || layer.frozen_depth_m < 0.0
                || layer.frozen_depth_m > layer.layer_depth_m
        });
        if !scalars.iter().all(|value| value.is_finite())
            || lane.tillage_depth_m < 0.0
            || lane.infiltration_m < 0.0
            || lane.beginning_aggregate_soil_water_m < 0.0
            || lane.ending_aggregate_soil_water_m < 0.0
            || invalid_layer
            || !production_arithmetic_is_defined(lane)
        {
            return Err(receiver_arithmetic_failure(
                operands,
                Some(&lane.ofe_id),
                None,
                "production soil receiver operand domain",
            ));
        }
    }
    for row in &operands.soil_thermal {
        if ![
            row.beginning_infiltration_credit_j_m2_ofe_ground,
            row.ending_infiltration_credit_j_m2_ofe_ground,
            row.beginning_enthalpy_j_m2_ofe_ground,
            row.infiltration_enthalpy_j_m2_ofe_ground,
            row.ending_enthalpy_j_m2_ofe_ground,
        ]
        .iter()
        .all(|value| value.is_finite())
            || checked_surface_liquid_add(
                row.beginning_infiltration_credit_j_m2_ofe_ground,
                row.infiltration_enthalpy_j_m2_ofe_ground,
            )
            .is_none()
            || checked_surface_liquid_add(
                row.beginning_enthalpy_j_m2_ofe_ground,
                row.infiltration_enthalpy_j_m2_ofe_ground,
            )
            .is_none()
        {
            return Err(receiver_arithmetic_failure(
                operands,
                Some(&row.ofe_id),
                Some(&row.tile_id),
                "soil-thermal receiver operand domain",
            ));
        }
    }
    for row in &operands.lse_tiles {
        if !row.tile_fraction.is_finite()
            || row.tile_fraction <= 0.0
            || row.tile_fraction > 1.0
            || ![
                row.beginning_enthalpy_j_m2_tile_ground,
                row.retained_enthalpy_j_m2_ofe_ground,
                row.ending_enthalpy_j_m2_tile_ground,
            ]
            .iter()
            .all(|value| value.is_finite())
            || checked_surface_liquid_div(row.retained_enthalpy_j_m2_ofe_ground, row.tile_fraction)
                .and_then(|retained| {
                    checked_surface_liquid_add(row.beginning_enthalpy_j_m2_tile_ground, retained)
                })
                .is_none()
        {
            return Err(receiver_arithmetic_failure(
                operands,
                Some(&row.ofe_id),
                Some(&row.tile_id),
                "LSE tile receiver operand domain",
            ));
        }
    }
    Ok(())
}

fn production_arithmetic_is_defined(lane: &ProductionSoilReceiverOperands) -> bool {
    let totals = |ending| {
        lane.ordered_layers
            .iter()
            .map(|layer| {
                checked_receiver_layer_total(
                    if ending {
                        layer.ending_liquid_m
                    } else {
                        layer.beginning_liquid_m
                    },
                    layer,
                )
            })
            .collect::<Option<Vec<_>>>()
            .and_then(checked_surface_liquid_sum)
    };
    independently_reconstruct_infiltration(lane).is_some()
        && totals(false).is_some()
        && totals(true).is_some()
        && checked_surface_liquid_add(lane.beginning_aggregate_soil_water_m, lane.infiltration_m)
            .is_some()
}

fn first_mismatch<T: PartialEq>(expected: &[T], actual: &[T]) -> usize {
    (0..expected.len().max(actual.len()))
        .find(|&index| expected.get(index) != actual.get(index))
        .unwrap_or(0)
}

fn join_failure(
    operands: &RealReceiverClosureOperands,
    owner_id: &ResourceOwnerId,
    ofe_id: Option<&OfeId>,
    tile_id: Option<&TileId>,
    detail: &'static str,
) -> DirectSurfaceLiquidError {
    DirectSurfaceLiquidError::canonical_failure(
        DirectSurfaceLiquidErrorCode::E010,
        DirectSurfaceLiquidPhase::IndependentClosure,
        DirectSurfaceLiquidErrorContext {
            transaction_id: Some(operands.transaction_id),
            owner_id: Some(owner_id.clone()),
            ofe_id: ofe_id.cloned(),
            tile_id: tile_id.cloned(),
            ..DirectSurfaceLiquidErrorContext::default()
        },
        DirectSurfaceLiquidRollbackHashes {
            beginning_owner_sha256: Some(operands.beginning_hydrology_snapshot_sha256.to_string()),
            attempted_owner_sha256: Some(receiver_operands_sha256(operands)),
        },
        detail,
    )
}

pub(super) fn receiver_operands_sha256(operands: &RealReceiverClosureOperands) -> String {
    let mut digest = Sha256::new();
    digest.update(b"openwepp-real-receiver-closure-operands-v2");
    digest.update(operands.transaction_id.0.to_be_bytes());
    for owner in [
        &operands.hydrology_owner_id,
        &operands.lse_owner_id,
        &operands.soil_thermal_owner_id,
    ] {
        digest.update(owner.as_str().as_bytes());
    }
    for (ofe, index, lane_id, layers) in &operands.expected_production_soil {
        digest.update(ofe.as_str().as_bytes());
        digest.update(index.to_be_bytes());
        digest.update(lane_id.to_be_bytes());
        for layer in layers {
            digest.update(layer.as_str().as_bytes());
        }
    }
    for (ofe, tile, layer) in &operands.expected_soil_thermal {
        digest.update(ofe.as_str().as_bytes());
        digest.update(tile.as_str().as_bytes());
        digest.update(layer.as_str().as_bytes());
    }
    for (ofe, tile) in &operands.expected_lse_tiles {
        digest.update(ofe.as_str().as_bytes());
        digest.update(tile.as_str().as_bytes());
    }
    for lane in &operands.production_soil {
        digest.update(lane.ofe_id.as_str().as_bytes());
        digest.update(lane.production_lane_index.to_be_bytes());
        digest.update(lane.production_lane_id.to_be_bytes());
        for value in [
            lane.tillage_depth_m,
            lane.infiltration_m,
            lane.beginning_aggregate_soil_water_m,
            lane.ending_aggregate_soil_water_m,
        ] {
            digest.update(value.to_bits().to_be_bytes());
        }
        for layer in &lane.ordered_layers {
            digest.update(layer.layer_id.as_str().as_bytes());
            for value in [
                layer.beginning_liquid_m,
                layer.ending_liquid_m,
                layer.layer_depth_m,
                layer.residual_theta,
                layer.frozen_depth_m,
            ] {
                digest.update(value.to_bits().to_be_bytes());
            }
        }
    }
    for thermal in &operands.soil_thermal {
        digest.update(thermal.ofe_id.as_str().as_bytes());
        digest.update(thermal.tile_id.as_str().as_bytes());
        digest.update(thermal.layer_id.as_str().as_bytes());
        for value in [
            thermal.beginning_infiltration_credit_j_m2_ofe_ground,
            thermal.ending_infiltration_credit_j_m2_ofe_ground,
            thermal.beginning_enthalpy_j_m2_ofe_ground,
            thermal.infiltration_enthalpy_j_m2_ofe_ground,
            thermal.ending_enthalpy_j_m2_ofe_ground,
        ] {
            digest.update(value.to_bits().to_be_bytes());
        }
    }
    for tile in &operands.lse_tiles {
        digest.update(tile.ofe_id.as_str().as_bytes());
        digest.update(tile.tile_id.as_str().as_bytes());
        for value in [
            tile.tile_fraction,
            tile.beginning_enthalpy_j_m2_tile_ground,
            tile.retained_enthalpy_j_m2_ofe_ground,
            tile.ending_enthalpy_j_m2_tile_ground,
        ] {
            digest.update(value.to_bits().to_be_bytes());
        }
    }
    format!("{:x}", digest.finalize())
}
