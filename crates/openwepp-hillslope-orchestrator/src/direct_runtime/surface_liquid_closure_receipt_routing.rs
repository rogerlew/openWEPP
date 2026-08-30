fn validate_projected_ending_state(
    configuration: &DirectSurfaceLiquidConfiguration,
    operands: &DirectSurfaceLiquidClosureOperands,
    ending: &DirectSurfaceLiquidOwnedState,
    projection: &ParcelArithmeticProjection,
) -> Result<(), DirectSurfaceLiquidError> {
    if ending.owner_id != configuration.owner_id
        || ending.configuration_sha256 != configuration.configuration_sha256
    {
        return Err(ending_aggregate_failure(
            operands.transaction_id,
            &configuration.owner_id,
            "projected ending-state owner/configuration",
        ));
    }

    let actual_store_keys = ending
        .records
        .iter()
        .map(|row| row.key.clone())
        .collect::<Vec<_>>();
    let expected_store_keys = configuration
        .records
        .iter()
        .map(|row| row.key.clone())
        .collect::<Vec<_>>();
    if let Some(offender) =
        first_membership_aware_mismatch(&actual_store_keys, &expected_store_keys)
    {
        return Err(contextual_comparison_failure(
            DirectSurfaceLiquidErrorCode::E010,
            operands.transaction_id,
            &configuration.owner_id,
            &offender,
            None,
            "projected ending store membership/order",
        ));
    }
    for (actual, configured) in ending.records.iter().zip(&configuration.records) {
        let expected_liquid = projection
            .expected_store_liquid
            .get(&configured.key)
            .copied()
            .ok_or(DirectSurfaceLiquidError::Closure(
                "projected ending store absent from independent projection",
            ))?;
        if actual.liquid_kg_m2_tile.to_bits() != expected_liquid.to_bits()
            || actual.last_accepted_transaction_id != Some(operands.transaction_id)
        {
            return Err(contextual_comparison_failure(
                DirectSurfaceLiquidErrorCode::E010,
                operands.transaction_id,
                &configuration.owner_id,
                &configured.key,
                None,
                "projected ending store join",
            ));
        }
    }

    let actual_continuation_ids = ending
        .continuations
        .iter()
        .map(|row| row.ofe_id.clone())
        .collect::<Vec<_>>();
    if let Some(offender) =
        first_membership_aware_mismatch(&actual_continuation_ids, &configuration.ofe_topology)
    {
        return Err(contextual_ofe_comparison_failure(
            DirectSurfaceLiquidErrorCode::E010,
            operands.transaction_id,
            &configuration.owner_id,
            &offender,
            "projected ending continuation membership/order",
        ));
    }
    for (actual, expected_ofe) in ending.continuations.iter().zip(&configuration.ofe_topology) {
        let expected = projection.expected_continuations.get(expected_ofe).ok_or(
            DirectSurfaceLiquidError::Closure(
                "projected ending continuation absent from independent projection",
            ),
        )?;
        if actual.day_index != expected.day_index
            || actual.next_interval_index != expected.next_interval_index
            || actual.cumulative_supply_m.to_bits() != expected.cumulative_supply_m.to_bits()
            || actual.cumulative_infiltration_m.to_bits()
                != expected.cumulative_infiltration_m.to_bits()
            || actual.last_accepted_transaction_id != Some(expected.transaction_id)
        {
            return Err(contextual_ofe_comparison_failure(
                DirectSurfaceLiquidErrorCode::E010,
                operands.transaction_id,
                &configuration.owner_id,
                expected_ofe,
                "projected ending continuation join",
            ));
        }
    }

    validate_projected_ending_digest(configuration, operands, ending)
}

fn validate_receipt_recipient(
    configuration: &DirectSurfaceLiquidConfiguration,
    binding: &super::surface_liquid_owner::DirectSurfaceLiquidOfeBinding,
    route_record: &DirectSurfaceLiquidConfigurationRecord,
    receipt: &DirectSurfaceLiquidParcelReceipt,
) -> Result<(), DirectSurfaceLiquidError> {
    let valid = match (&receipt.disposition, &receipt.recipient) {
        (
            DirectSurfaceLiquidReceiptDisposition::Infiltration,
            DirectSurfaceLiquidReceiptRecipient::SoilInfiltration {
                ofe_id,
                production_lane_index,
                production_lane_id,
                ordered_soil_layer_ids,
                soil_thermal_layer_id,
            },
        ) => {
            ofe_id == &binding.ofe_id
                && production_lane_index == &binding.production_lane_index
                && production_lane_id == &binding.production_lane_id
                && ordered_soil_layer_ids == &binding.ordered_soil_layer_ids
                && soil_thermal_layer_id == &binding.infiltration_soil_thermal_layer_id
        }
        (
            DirectSurfaceLiquidReceiptDisposition::RetainedSurface,
            DirectSurfaceLiquidReceiptRecipient::SurfaceStore { store_key },
        ) => store_key == &receipt.recipient_store_key,
        (
            DirectSurfaceLiquidReceiptDisposition::RoutedRunoff,
            DirectSurfaceLiquidReceiptRecipient::RoutedOfe {
                source_ofe_id,
                destination_ofe_id,
                destination_store_key,
            },
        ) => match route_destination(configuration, route_record) {
            Ok((expected_ofe, expected_record)) => {
                source_ofe_id == &receipt.basis_ofe_id
                    && destination_ofe_id == &expected_ofe
                    && destination_store_key == &expected_record.key
            }
            Err(_) => false,
        },
        (
            DirectSurfaceLiquidReceiptDisposition::OutletRunoff,
            DirectSurfaceLiquidReceiptRecipient::Outlet { ofe_id },
        ) => {
            ofe_id == &receipt.basis_ofe_id
                && route_record.runon_destination_ofe_id.is_none()
                && route_record.runon_destination_tile_id.is_none()
        }
        _ => false,
    };
    if valid {
        Ok(())
    } else {
        Err(DirectSurfaceLiquidError::Closure(
            "wrong typed parcel recipient",
        ))
    }
}

fn route_destination<'a>(
    configuration: &'a DirectSurfaceLiquidConfiguration,
    route_record: &DirectSurfaceLiquidConfigurationRecord,
) -> Result<(OfeId, &'a DirectSurfaceLiquidConfigurationRecord), DirectSurfaceLiquidError> {
    let destination_ofe =
        route_record
            .runon_destination_ofe_id
            .clone()
            .ok_or(DirectSurfaceLiquidError::Closure(
                "routed receipt on terminal OFE",
            ))?;
    let destination_tile = route_record.runon_destination_tile_id.as_ref().ok_or(
        DirectSurfaceLiquidError::Closure("routed receipt missing destination tile"),
    )?;
    let destination = configuration
        .records
        .iter()
        .find(|record| {
            record.key.ofe_id == destination_ofe && record.key.tile_id == *destination_tile
        })
        .ok_or(DirectSurfaceLiquidError::Closure(
            "routed receipt destination missing",
        ))?;
    Ok((destination_ofe, destination))
}
