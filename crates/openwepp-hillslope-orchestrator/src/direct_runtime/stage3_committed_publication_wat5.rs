#[derive(Clone, Copy, Debug)]
struct AcceptedWat5RainSegmentV1 {
    start_s: f64,
    end_s: f64,
    depth_m: f64,
}

#[derive(Clone, Copy, Debug, Default, PartialEq)]
struct AcceptedRunonReceiptComponentsV1 {
    local_liquid_m: f64,
    upstream_runon_m: f64,
    sent_volume_m3: f64,
    received_volume_m3: f64,
}

fn accepted_runon_receipt_components(
    receipts: &[DirectSurfaceLiquidParcelReceipt],
    ofe_id: &OfeId,
    surface_configuration: &DirectSurfaceLiquidConfiguration,
) -> Result<AcceptedRunonReceiptComponentsV1, DirectRuntimeError> {
    let mut out = AcceptedRunonReceiptComponentsV1::default();
    let mut received_sources = BTreeSet::new();
    for receipt in receipts
        .iter()
        .filter(|receipt| &receipt.basis_ofe_id == ofe_id)
    {
        let amount_m = receipt.mass_kg_m2_basis_ofe_ground / KG_M2_PER_M_WATER;
        match receipt.kind {
            DirectSurfaceLiquidParcelKind::UpstreamRunon => {
                add_nonnegative(&mut out.upstream_runon_m, amount_m)?;
                received_sources.insert(receipt.source_parcel_id.as_str());
            }
            DirectSurfaceLiquidParcelKind::RawPrecipitation
            | DirectSurfaceLiquidParcelKind::CanopyThroughfall
            | DirectSurfaceLiquidParcelKind::CanopyInitialDrainage
            | DirectSurfaceLiquidParcelKind::CanopySecondDrainage
            | DirectSurfaceLiquidParcelKind::CanopyStemflow
            | DirectSurfaceLiquidParcelKind::CondensationOverflow
            | DirectSurfaceLiquidParcelKind::TerminalReceiver => {
                add_nonnegative(&mut out.local_liquid_m, amount_m)?;
            }
        }
    }

    let destination_area_m2 = accepted_publication_ofe_area_m2(surface_configuration, ofe_id)?;
    out.received_volume_m3 = out.upstream_runon_m * destination_area_m2;
    validate_nonnegative_direct_m(
        "stage3_publication.accepted_runon_received_volume_m3",
        out.received_volume_m3,
    )?;
    let mut sent_sources = BTreeSet::new();
    for receipt in receipts.iter().filter(|receipt| {
        receipt.disposition == DirectSurfaceLiquidReceiptDisposition::RoutedRunoff
            && matches!(
                &receipt.recipient,
                DirectSurfaceLiquidReceiptRecipient::RoutedOfe {
                    destination_ofe_id,
                    ..
                } if destination_ofe_id == ofe_id
            )
    }) {
        let source_area_m2 =
            accepted_publication_ofe_area_m2(surface_configuration, &receipt.basis_ofe_id)?;
        let volume_m3 = receipt.mass_kg_m2_basis_ofe_ground / KG_M2_PER_M_WATER * source_area_m2;
        add_nonnegative(&mut out.sent_volume_m3, volume_m3)?;
        sent_sources.insert(receipt.source_parcel_id.as_str());
    }
    if sent_sources != received_sources {
        return Err(stage3_publication_guard(
            "accepted routed-runon source identity closure",
        ));
    }
    let volume_tolerance_m3 = ACCEPTED_CLOSURE_TOLERANCE_M * destination_area_m2.max(1.0);
    if (out.sent_volume_m3 - out.received_volume_m3).abs() > volume_tolerance_m3 {
        return Err(stage3_publication_guard(
            "accepted routed-runon sent/received closure",
        ));
    }
    Ok(out)
}

fn accepted_publication_ofe_area_m2(
    surface_configuration: &DirectSurfaceLiquidConfiguration,
    ofe_id: &OfeId,
) -> Result<f64, DirectRuntimeError> {
    let mut area_m2 = None;
    for record in surface_configuration
        .records
        .iter()
        .filter(|record| &record.key.ofe_id == ofe_id)
    {
        validate_nonnegative_direct_m(
            "stage3_publication.accepted_runon_ofe_area_m2",
            record.ofe_area_m2,
        )?;
        if area_m2.is_some_and(|accepted: f64| accepted.to_bits() != record.ofe_area_m2.to_bits()) {
            return Err(stage3_publication_guard(
                "accepted routed-runon OFE area identity",
            ));
        }
        area_m2 = Some(record.ofe_area_m2);
    }
    let area_m2 = area_m2.ok_or(stage3_publication_guard(
        "accepted routed-runon OFE area omission",
    ))?;
    if area_m2 <= 0.0 {
        return Err(stage3_publication_guard(
            "accepted routed-runon positive OFE area",
        ));
    }
    Ok(area_m2)
}

fn install_requested_accepted_wat5_source(
    day: &mut DirectDayFrame,
    publication_input: &DirectPublicationDayInput,
    supports: &[Stage3AcceptedPublicationSupportV1],
    accepted: &AcceptedLaneDay,
    binding: &DirectSurfaceLiquidOfeBinding,
    surface_configuration: &DirectSurfaceLiquidConfiguration,
) -> Result<(), DirectRuntimeError> {
    day.wat5_subhourly_requested = publication_input.wat5_subhourly_requested;
    if !publication_input.wat5_subhourly_requested {
        return Ok(());
    }
    if day.infiltration_depression_inputs.producer_inputs.is_some() {
        return Err(stage3_publication_guard(
            "accepted WAT5 producer source overwrite",
        ));
    }

    let parameters = accepted_wat5_parameters(supports, &binding.ofe_id)?;
    let hyetograph = accepted_wat5_hyetograph(supports, &binding.ofe_id, surface_configuration)?;
    let hourly_additional_supply_m =
        accepted_wat5_additional_supply(supports, accepted, &binding.ofe_id)?;
    let depression_storage_capacity_m = surface_configuration
        .records
        .iter()
        .filter(|record| record.key.ofe_id == binding.ofe_id)
        .try_fold(0.0, |capacity_m, record| {
            let record_capacity_m =
                record.capacity_kg_m2_tile * record.tile_fraction / KG_M2_PER_M_WATER;
            validate_nonnegative_direct_m(
                "stage3_publication.wat5_depression_storage_capacity_m",
                record_capacity_m,
            )?;
            let capacity_m = capacity_m + record_capacity_m;
            validate_nonnegative_direct_m(
                "stage3_publication.wat5_depression_storage_capacity_m",
                capacity_m,
            )?;
            Ok(capacity_m)
        })?;

    day.infiltration_depression_inputs.producer_inputs =
        Some(DirectWb14InfiltrationProducerInputs {
            hyetograph,
            hourly_additional_supply_m,
            effective_conductivity_m_s: parameters.effective_conductivity_m_s,
            matric_potential_m: parameters.matric_potential_m,
            storage_capacity_m: parameters.infiltration_storage_capacity_m,
            depression_storage_capacity_m,
        });
    Ok(())
}

fn accepted_wat5_parameters(
    supports: &[Stage3AcceptedPublicationSupportV1],
    ofe_id: &OfeId,
) -> Result<DirectOfeWb14Parameters, DirectRuntimeError> {
    let mut accepted_parameters = None;
    for support in supports {
        let matches = support
            .wb14_parameters()
            .iter()
            .filter(|parameters| &parameters.ofe_id == ofe_id)
            .collect::<Vec<_>>();
        let [parameters] = matches.as_slice() else {
            return Err(stage3_publication_guard(
                "accepted WAT5 WB14 parameter cardinality",
            ));
        };
        if accepted_parameters
            .as_ref()
            .is_some_and(|accepted| accepted != *parameters)
        {
            return Err(stage3_publication_guard(
                "accepted WAT5 WB14 parameter chronology",
            ));
        }
        accepted_parameters = Some((*parameters).clone());
    }
    accepted_parameters.ok_or(stage3_publication_guard(
        "missing accepted WAT5 WB14 parameters",
    ))
}

fn accepted_wat5_hyetograph(
    supports: &[Stage3AcceptedPublicationSupportV1],
    ofe_id: &OfeId,
    surface_configuration: &DirectSurfaceLiquidConfiguration,
) -> Result<Vec<DirectWb14HyetographInterval>, DirectRuntimeError> {
    let day_start_ns = supports
        .first()
        .ok_or(stage3_publication_guard("missing accepted WAT5 supports"))?
        .support()
        .start_ns()
        .get();
    let mut segments = Vec::new();
    for support in supports {
        let support_offset_ns = support
            .support()
            .start_ns()
            .get()
            .checked_sub(day_start_ns)
            .ok_or(stage3_publication_guard("accepted WAT5 support chronology"))?;
        let support_offset_ns = u64::try_from(support_offset_ns)
            .map_err(|_| stage3_publication_guard("accepted WAT5 support offset representation"))?;
        let support_offset_s = std::time::Duration::from_nanos(support_offset_ns).as_secs_f64();
        for parcel in support
            .lse_forcing()
            .precipitation_parcels
            .iter()
            .filter(|parcel| &parcel.destination_ofe_id == ofe_id)
        {
            if parcel.parcel_kind != openwepp_land_surface_energy::LiquidParcelKind::Precipitation {
                return Err(stage3_publication_guard(
                    "accepted WAT5 precipitation source kind",
                ));
            }
            let record = surface_configuration
                .records
                .iter()
                .find(|record| {
                    record.key.ofe_id == parcel.destination_ofe_id
                        && record.key.tile_id == parcel.destination_tile_id
                })
                .ok_or(stage3_publication_guard(
                    "accepted WAT5 precipitation destination configuration",
                ))?;
            let depth_m = parcel.amount_kg_m2_destination_tile_ground * record.tile_fraction
                / KG_M2_PER_M_WATER;
            validate_nonnegative_direct_m("stage3_publication.wat5_rainfall_depth_m", depth_m)?;
            let start_s = support_offset_s + parcel.start_s;
            let end_s = support_offset_s + parcel.end_s;
            if !start_s.is_finite()
                || !end_s.is_finite()
                || start_s < 0.0
                || end_s <= start_s
                || end_s > 86_400.0
            {
                return Err(stage3_publication_guard(
                    "accepted WAT5 precipitation support domain",
                ));
            }
            if depth_m > 0.0 {
                segments.push(AcceptedWat5RainSegmentV1 {
                    start_s,
                    end_s,
                    depth_m,
                });
            }
        }
    }
    accepted_wat5_piecewise_hyetograph(&segments)
}

fn accepted_wat5_piecewise_hyetograph(
    segments: &[AcceptedWat5RainSegmentV1],
) -> Result<Vec<DirectWb14HyetographInterval>, DirectRuntimeError> {
    if segments.is_empty() {
        return Ok(vec![DirectWb14HyetographInterval {
            start_s: 0.0,
            end_s: 0.0,
            intensity_m_s: 0.0,
        }]);
    }
    let mut boundaries = segments
        .iter()
        .flat_map(|segment| [segment.start_s, segment.end_s])
        .collect::<Vec<_>>();
    boundaries.sort_by(f64::total_cmp);
    boundaries.dedup_by(|left, right| left.to_bits() == right.to_bits());
    let mut hyetograph = Vec::new();
    for bounds in boundaries.windows(2) {
        let start_s = bounds[0];
        let end_s = bounds[1];
        if end_s <= start_s {
            continue;
        }
        let intensity_m_s = segments
            .iter()
            .filter(|segment| segment.start_s <= start_s && segment.end_s >= end_s)
            .try_fold(0.0, |intensity_m_s, segment| {
                let rate_m_s = segment.depth_m / (segment.end_s - segment.start_s);
                validate_nonnegative_direct_m(
                    "stage3_publication.wat5_rainfall_intensity_m_s",
                    rate_m_s,
                )?;
                let intensity_m_s = intensity_m_s + rate_m_s;
                validate_nonnegative_direct_m(
                    "stage3_publication.wat5_rainfall_intensity_m_s",
                    intensity_m_s,
                )?;
                Ok(intensity_m_s)
            })?;
        hyetograph.push(DirectWb14HyetographInterval {
            start_s,
            end_s,
            intensity_m_s,
        });
    }
    if hyetograph.is_empty() {
        return Err(stage3_publication_guard(
            "accepted WAT5 rainfall chronology",
        ));
    }
    Ok(hyetograph)
}

fn accepted_wat5_additional_supply(
    supports: &[Stage3AcceptedPublicationSupportV1],
    accepted: &AcceptedLaneDay,
    ofe_id: &OfeId,
) -> Result<[f64; 24], DirectRuntimeError> {
    let day_start_ns = supports
        .first()
        .ok_or(stage3_publication_guard("missing accepted WAT5 supports"))?
        .support()
        .start_ns()
        .get();
    let mut hourly = accepted.hourly_snow_terminal_liquid_m;
    for support in supports {
        for receipt in support.ingress_receipts().iter().filter(|receipt| {
            &receipt.basis_ofe_id == ofe_id
                && receipt.kind == DirectSurfaceLiquidParcelKind::UpstreamRunon
        }) {
            let amount_m = receipt.mass_kg_m2_basis_ofe_ground / KG_M2_PER_M_WATER;
            distribute_receipt_to_hours(
                &mut hourly,
                day_start_ns,
                support.support(),
                receipt.start_s,
                receipt.end_s,
                amount_m,
            )?;
        }
    }
    Ok(hourly)
}
