fn apply_support_liquid_custody_v2_to_lane_day(
    out: &mut AcceptedLaneDay,
    subslab: &crate::snow_stage3_v11_attachment::Stage3CoupledSubslabReceiptV1,
    binding: &DirectSurfaceLiquidOfeBinding,
    surface_configuration: &DirectSurfaceLiquidConfiguration,
    day_start_ns: u128,
) -> Result<(), DirectRuntimeError> {
    subslab
        .validate_support_liquid_custody_v2()
        .map_err(|_| stage3_publication_guard("accepted support-liquid custody V2 seal"))?;
    let custody = subslab
        .support_liquid_custody_v2()
        .ok_or(stage3_publication_guard(
            "accepted support-liquid custody V2 omission",
        ))?;
    custody
        .surface_beginning_state
        .validate(surface_configuration)
        .map_err(|_| stage3_publication_guard("accepted support-liquid beginning surface seal"))?;
    custody
        .surface_ending_state
        .validate(surface_configuration)
        .map_err(|_| stage3_publication_guard("accepted support-liquid ending surface seal"))?;
    project_zero_duration_snow_liquid_receipts_to_lane_day(
        out,
        &custody.receiver_receipts,
        custody.receiver_receipt_set_sha256,
        binding,
        surface_configuration,
        &custody.surface_beginning_state,
        &custody.surface_ending_state,
        day_start_ns,
        custody.support,
        true,
        false,
    )
}

#[allow(clippy::too_many_arguments)]
fn project_zero_duration_snow_liquid_receipts_to_lane_day(
    out: &mut AcceptedLaneDay,
    receipts: &[crate::DirectZeroDurationSnowLiquidReceiptV1],
    expected_receipt_set_sha256: Digest32,
    binding: &DirectSurfaceLiquidOfeBinding,
    surface_configuration: &DirectSurfaceLiquidConfiguration,
    surface_beginning_state: &DirectSurfaceLiquidOwnedState,
    surface_ending_state: &DirectSurfaceLiquidOwnedState,
    day_start_ns: u128,
    support: openwepp_coupled_time::TimeSupport,
    track_support_liquid_runon: bool,
    runoff_at_support_end: bool,
) -> Result<(), DirectRuntimeError> {
    let receipt_set = crate::zero_duration_snow_liquid_receipt_set_sha256(receipts)
        .map(Digest32::from_bytes)
        .map_err(|_| stage3_publication_guard("accepted support-liquid typed receipt set"))?;
    if receipt_set != expected_receipt_set_sha256 {
        return Err(stage3_publication_guard(
            "accepted support-liquid receipt-set identity",
        ));
    }
    validate_support_liquid_routing_receipts(receipts, surface_configuration)?;

    let mut ingress_m = 0.0;
    let mut runon_m = 0.0;
    let mut retained_m = 0.0;
    let mut runoff_m = 0.0;
    let mut ingress_enthalpy_j_m2 = 0.0;
    let mut retained_enthalpy_j_m2 = 0.0;
    let mut runoff_enthalpy_j_m2 = 0.0;
    let mut has_basis_receipt = false;
    let support_seconds = support.duration_ns() as f64 / 1.0e9;
    for receipt in receipts
        .iter()
        .filter(|receipt| receipt.basis_ofe_id == binding.ofe_id)
    {
        has_basis_receipt = true;
        let amount_m = receipt.mass_kg_m2_basis_ofe_ground / KG_M2_PER_M_WATER;
        add_nonnegative(&mut ingress_m, amount_m)?;
        add_nonnegative(
            &mut ingress_enthalpy_j_m2,
            receipt.sensible_enthalpy_j_m2_basis_ofe_ground,
        )?;
        if receipt.origin_ofe_id != receipt.basis_ofe_id {
            add_nonnegative(&mut runon_m, amount_m)?;
        }
        match receipt.disposition {
            crate::direct_runtime::DirectZeroDurationSnowLiquidDispositionV1::RetainedSurface => {
                add_nonnegative(&mut retained_m, amount_m)?;
                add_nonnegative(
                    &mut retained_enthalpy_j_m2,
                    receipt.sensible_enthalpy_j_m2_basis_ofe_ground,
                )?;
            }
            crate::direct_runtime::DirectZeroDurationSnowLiquidDispositionV1::RoutedRunoff
            | crate::direct_runtime::DirectZeroDurationSnowLiquidDispositionV1::OutletRunoff => {
                add_nonnegative(&mut runoff_m, amount_m)?;
                add_nonnegative(
                    &mut runoff_enthalpy_j_m2,
                    receipt.sensible_enthalpy_j_m2_basis_ofe_ground,
                )?;
                if runoff_at_support_end {
                    let tick_offset = support.end_ns().get().checked_sub(day_start_ns).ok_or(
                        stage3_publication_guard(
                            "accepted zero-duration snow-liquid runoff day tick",
                        ),
                    )?;
                    let hour = usize::try_from(
                        tick_offset.saturating_sub(1) / 3_600_000_000_000,
                    )
                    .map_err(|_| {
                        stage3_publication_guard(
                            "accepted zero-duration snow-liquid runoff hour",
                        )
                    })?
                    .min(23);
                    add_nonnegative(&mut out.hourly_runoff_m[hour], amount_m)?;
                } else {
                    distribute_receipt_to_hours(
                        &mut out.hourly_runoff_m,
                        day_start_ns,
                        support,
                        0.0,
                        support_seconds,
                        amount_m,
                    )?;
                }
            }
        }
    }
    let beginning_storage_m = ofe_surface_storage_m(
        surface_beginning_state,
        surface_configuration,
        &binding.ofe_id,
    )?;
    let ending_storage_m =
        ofe_surface_storage_m(surface_ending_state, surface_configuration, &binding.ofe_id)?;
    if !has_basis_receipt {
        if beginning_storage_m.to_bits() != ending_storage_m.to_bits() {
            return Err(stage3_publication_guard(
                "accepted support-liquid unrelated OFE mutation",
            ));
        }
        return Ok(());
    }
    if ingress_m <= 0.0 || (retained_m + runoff_m - ingress_m).abs() > ACCEPTED_CLOSURE_TOLERANCE_M
    {
        return Err(stage3_publication_guard(
            "accepted support-liquid disposition closure",
        ));
    }
    if (retained_enthalpy_j_m2 + runoff_enthalpy_j_m2 - ingress_enthalpy_j_m2).abs() > 1.0e-9 {
        return Err(stage3_publication_guard(
            "accepted support-liquid enthalpy disposition closure",
        ));
    }
    if (ending_storage_m - beginning_storage_m - retained_m).abs() > ACCEPTED_CLOSURE_TOLERANCE_M {
        return Err(stage3_publication_guard(
            "accepted support-liquid retained surface closure",
        ));
    }
    add_nonnegative(&mut out.ingress_m, ingress_m)?;
    add_nonnegative(&mut out.runon_m, runon_m)?;
    if track_support_liquid_runon {
        add_nonnegative(&mut out.support_liquid_runon_m, runon_m)?;
    }
    add_nonnegative(&mut out.retained_surface_liquid_m, retained_m)?;
    add_nonnegative(&mut out.runoff_m, runoff_m)?;
    Ok(())
}

#[cfg(test)]
#[allow(clippy::too_many_arguments)]
fn project_support_liquid_receipts_to_lane_day(
    out: &mut AcceptedLaneDay,
    receipts: &[crate::DirectZeroDurationSnowLiquidReceiptV1],
    expected_receipt_set_sha256: Digest32,
    binding: &DirectSurfaceLiquidOfeBinding,
    surface_configuration: &DirectSurfaceLiquidConfiguration,
    surface_beginning_state: &DirectSurfaceLiquidOwnedState,
    surface_ending_state: &DirectSurfaceLiquidOwnedState,
    day_start_ns: u128,
    support: openwepp_coupled_time::TimeSupport,
) -> Result<(), DirectRuntimeError> {
    project_zero_duration_snow_liquid_receipts_to_lane_day(
        out,
        receipts,
        expected_receipt_set_sha256,
        binding,
        surface_configuration,
        surface_beginning_state,
        surface_ending_state,
        day_start_ns,
        support,
        true,
        false,
    )
}

fn validate_support_liquid_routing_receipts(
    receipts: &[crate::DirectZeroDurationSnowLiquidReceiptV1],
    surface_configuration: &DirectSurfaceLiquidConfiguration,
) -> Result<(), DirectRuntimeError> {
    type RouteKey = ([u8; 32], OfeId, OfeId);
    let mut routed = BTreeMap::<RouteKey, (f64, f64)>::new();
    let mut disposition_by_basis = BTreeMap::<([u8; 32], OfeId), (f64, f64)>::new();
    for receipt in receipts {
        let disposition = disposition_by_basis
            .entry((receipt.output_receipt_sha256, receipt.basis_ofe_id.clone()))
            .or_default();
        disposition.0 =
            checked_surface_liquid_add(disposition.0, receipt.mass_kg_m2_basis_ofe_ground)
                .filter(|value| value.is_finite())
                .ok_or(stage3_publication_guard(
                    "accepted support-liquid disposition mass aggregation",
                ))?;
        disposition.1 = checked_surface_liquid_add(
            disposition.1,
            receipt.sensible_enthalpy_j_m2_basis_ofe_ground,
        )
        .filter(|value| value.is_finite())
        .ok_or(stage3_publication_guard(
            "accepted support-liquid disposition enthalpy aggregation",
        ))?;
        if receipt.disposition
            == crate::direct_runtime::DirectZeroDurationSnowLiquidDispositionV1::RoutedRunoff
        {
            let transfer = routed
                .entry((
                    receipt.output_receipt_sha256,
                    receipt.basis_ofe_id.clone(),
                    receipt.recipient_ofe_id.clone(),
                ))
                .or_default();
            transfer.0 =
                checked_surface_liquid_add(transfer.0, receipt.mass_kg_m2_basis_ofe_ground)
                    .filter(|value| value.is_finite())
                    .ok_or(stage3_publication_guard(
                        "accepted support-liquid routed mass aggregation",
                    ))?;
            transfer.1 = checked_surface_liquid_add(
                transfer.1,
                receipt.sensible_enthalpy_j_m2_basis_ofe_ground,
            )
            .filter(|value| value.is_finite())
            .ok_or(stage3_publication_guard(
                "accepted support-liquid routed enthalpy aggregation",
            ))?;
        }
    }
    for ((output, source_ofe, destination_ofe), (source_mass, source_enthalpy)) in routed {
        let source_area = support_liquid_ofe_area_m2(surface_configuration, &source_ofe)?;
        let destination_area = support_liquid_ofe_area_m2(surface_configuration, &destination_ofe)?;
        let area_ratio = source_area / destination_area;
        if !area_ratio.is_finite() || area_ratio <= 0.0 {
            return Err(stage3_publication_guard(
                "accepted support-liquid routed area ratio",
            ));
        }
        let (destination_mass, destination_enthalpy) = disposition_by_basis
            .get(&(output, destination_ofe))
            .copied()
            .ok_or(stage3_publication_guard(
                "accepted support-liquid routed destination omission",
            ))?;
        if (destination_mass - source_mass * area_ratio).abs() > 1.0e-12
            || (destination_enthalpy - source_enthalpy * area_ratio).abs() > 1.0e-9
        {
            return Err(stage3_publication_guard(
                "accepted support-liquid routed mass/enthalpy area closure",
            ));
        }
    }
    Ok(())
}

fn support_liquid_ofe_area_m2(
    surface_configuration: &DirectSurfaceLiquidConfiguration,
    ofe_id: &OfeId,
) -> Result<f64, DirectRuntimeError> {
    let mut areas = surface_configuration
        .records
        .iter()
        .filter(|record| &record.key.ofe_id == ofe_id)
        .map(|record| record.ofe_area_m2);
    let area = areas.next().ok_or(stage3_publication_guard(
        "accepted support-liquid routed OFE area",
    ))?;
    if !area.is_finite()
        || area <= 0.0
        || areas.any(|candidate| candidate.to_bits() != area.to_bits())
    {
        return Err(stage3_publication_guard(
            "accepted support-liquid routed OFE area identity",
        ));
    }
    Ok(area)
}
