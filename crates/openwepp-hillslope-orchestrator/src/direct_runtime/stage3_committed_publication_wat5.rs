#[derive(Clone, Copy, Debug)]
struct AcceptedWat5RainSegmentV1 {
    start_s: f64,
    end_s: f64,
    depth_m: f64,
}

#[derive(Clone, Debug)]
struct AcceptedWat5ReceiptSourceV1 {
    support_receipt_sha256: Digest32,
    kind: DirectSurfaceLiquidParcelKind,
    source_parcel_id: String,
    transaction_id: openwepp_kernel_contract::TransactionId,
    start_s: f64,
    end_s: f64,
    depth_m: f64,
    infiltration_m: f64,
    retained_surface_m: f64,
    runoff_m: f64,
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
            | DirectSurfaceLiquidParcelKind::LitterPhaseOverflow
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

fn install_authenticated_accepted_wat5_source(
    day: &mut DirectDayFrame,
    publication_input: &DirectPublicationDayInput,
    authenticated_lane_d_source_required: bool,
    supports: &[Stage3AcceptedPublicationSupportV1],
    coupled_subslabs: &[crate::snow_stage3_v11_attachment::Stage3CoupledSubslabReceiptV1],
    binding: &DirectSurfaceLiquidOfeBinding,
    surface_configuration: &DirectSurfaceLiquidConfiguration,
) -> Result<Option<runoff::DirectWb14SubhourlyProfile>, DirectRuntimeError> {
    // WAT5 selection controls only publication of the optional five-minute
    // diagnostic. Active Lane D independently authenticates the accepted
    // WB14 producer lineage even though it consumes the exact committed
    // 24-bin `wb14_hourly_excess_m` owner below this seam.
    day.wat5_subhourly_requested = publication_input.wat5_subhourly_requested;
    if !publication_input.wat5_subhourly_requested && !authenticated_lane_d_source_required {
        return Ok(None);
    }
    if day.infiltration_depression_inputs.producer_inputs.is_some() {
        return Err(stage3_publication_guard(
            "accepted WAT5 producer source overwrite",
        ));
    }

    let parameters = accepted_wat5_parameters(supports, &binding.ofe_id)?;
    let receipt_sources =
        accepted_wat5_receipt_sources(supports, coupled_subslabs, &binding.ofe_id)?;
    let hyetograph = accepted_wat5_hyetograph(&receipt_sources)?;
    let additional_supply_segments =
        accepted_wat5_additional_supply_segments(&receipt_sources, &binding.ofe_id)?;
    let hourly_additional_supply_m = runoff::wat5_hourly_additional_supply_from_segments(
        &additional_supply_segments,
        Some(&binding.ofe_id),
    )?;
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

    let producer_inputs = DirectWb14InfiltrationProducerInputs {
        hyetograph,
        hourly_additional_supply_m,
        effective_conductivity_m_s: parameters.effective_conductivity_m_s,
        matric_potential_m: parameters.matric_potential_m,
        storage_capacity_m: parameters.infiltration_storage_capacity_m,
        depression_storage_capacity_m,
    };
    let accepted_profile = if publication_input.wat5_subhourly_requested {
        Some(accepted_wat5_receipt_profile(
            &producer_inputs,
            &additional_supply_segments,
            &receipt_sources,
            &binding.ofe_id,
        )?)
    } else {
        None
    };
    day.infiltration_depression_inputs.producer_inputs = Some(producer_inputs);
    Ok(accepted_profile)
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

fn accepted_wat5_receipt_sources(
    supports: &[Stage3AcceptedPublicationSupportV1],
    coupled_subslabs: &[crate::snow_stage3_v11_attachment::Stage3CoupledSubslabReceiptV1],
    ofe_id: &OfeId,
) -> Result<Vec<AcceptedWat5ReceiptSourceV1>, DirectRuntimeError> {
    let day_start_ns = supports
        .first()
        .ok_or(stage3_publication_guard("missing accepted WAT5 supports"))?
        .support()
        .start_ns()
        .get();
    let mut sources = Vec::<AcceptedWat5ReceiptSourceV1>::new();
    for support in supports {
        let ledger = support
            .ingress_ledgers()
            .iter()
            .find(|ledger| &ledger.ofe_id == ofe_id);
        if ledger.is_none() {
            let matching = coupled_subslabs
                .iter()
                .filter(|subslab| subslab.support == support.support())
                .collect::<Vec<_>>();
            let native_inactive = if matching.len() == 1 {
                let subslab = matching[0];
                subslab.validate().map_err(|_| {
                    stage3_publication_guard("accepted WAT5 native inactive subslab seal")
                })?;
                crate::direct_runtime::stage3_covered_native_inactive_child_custody_binding(
                    &subslab.wb14_child_replay_bytes,
                    &subslab.wb14_ofe_topology,
                )
                .map_err(|_| {
                    stage3_publication_guard("accepted WAT5 native inactive custody marker")
                })?
                .is_some_and(|binding| {
                    subslab.wb14_ofe_topology.contains(ofe_id)
                        && binding.child_support_start_ns == support.support().start_ns().get()
                        && binding.child_support_end_ns == support.support().end_ns().get()
                })
            } else {
                false
            };
            if !native_inactive
                || !support.ingress_ledgers().is_empty()
                || !support.ingress_receipts().is_empty()
            {
                return Err(stage3_publication_guard(
                    "accepted WAT5 OFE ingress ledger",
                ));
            }
        }
        let source_start = sources.len();
        for receipt in support
            .ingress_receipts()
            .iter()
            .filter(|receipt| &receipt.basis_ofe_id == ofe_id)
        {
            if receipt.kind == DirectSurfaceLiquidParcelKind::LitterPhaseOverflow {
                let prefix = format!("litter-phase-overflow:{}:", receipt.transaction_id.0);
                if !receipt.source_parcel_id.starts_with(&prefix)
                    || receipt.origin_store_key != receipt.recipient_store_key
                {
                    return Err(stage3_publication_guard(
                        "accepted WAT5 litter-overflow phase receipt",
                    ));
                }
            }
            let depth_m = receipt.mass_kg_m2_basis_ofe_ground / KG_M2_PER_M_WATER;
            validate_nonnegative_direct_m("stage3_publication.wat5_receipt_depth_m", depth_m)?;
            if depth_m == 0.0 {
                continue;
            }
            let (start_s, end_s) = accepted_wat5_day_support(
                day_start_ns,
                support.support(),
                receipt.start_s,
                receipt.end_s,
            )?;
            if let Some(source) = sources[source_start..].iter_mut().find(|source| {
                source.support_receipt_sha256 == support.receipt_sha256()
                    && source.kind == receipt.kind
                    && source.source_parcel_id == receipt.source_parcel_id
                    && source.transaction_id == receipt.transaction_id
                    && source.start_s.to_bits() == start_s.to_bits()
                    && source.end_s.to_bits() == end_s.to_bits()
            }) {
                add_nonnegative(&mut source.depth_m, depth_m)?;
                match receipt.disposition {
                    DirectSurfaceLiquidReceiptDisposition::Infiltration => {
                        add_nonnegative(&mut source.infiltration_m, depth_m)?;
                    }
                    DirectSurfaceLiquidReceiptDisposition::RetainedSurface => {
                        add_nonnegative(&mut source.retained_surface_m, depth_m)?;
                    }
                    DirectSurfaceLiquidReceiptDisposition::RoutedRunoff
                    | DirectSurfaceLiquidReceiptDisposition::OutletRunoff => {
                        add_nonnegative(&mut source.runoff_m, depth_m)?;
                    }
                }
            } else {
                let (infiltration_m, retained_surface_m, runoff_m) = match receipt.disposition {
                    DirectSurfaceLiquidReceiptDisposition::Infiltration => (depth_m, 0.0, 0.0),
                    DirectSurfaceLiquidReceiptDisposition::RetainedSurface => {
                        (0.0, depth_m, 0.0)
                    }
                    DirectSurfaceLiquidReceiptDisposition::RoutedRunoff
                    | DirectSurfaceLiquidReceiptDisposition::OutletRunoff => {
                        (0.0, 0.0, depth_m)
                    }
                };
                sources.push(AcceptedWat5ReceiptSourceV1 {
                    support_receipt_sha256: support.receipt_sha256(),
                    kind: receipt.kind,
                    source_parcel_id: receipt.source_parcel_id.clone(),
                    transaction_id: receipt.transaction_id,
                    start_s,
                    end_s,
                    depth_m,
                    infiltration_m,
                    retained_surface_m,
                    runoff_m,
                });
            }
        }
        let projected_m = sources[source_start..]
            .iter()
            .map(|source| source.depth_m)
            .sum::<f64>();
        let authoritative_m = ledger.map_or(0.0, |value| {
            value.ingress_mass_kg_m2_ofe_ground / KG_M2_PER_M_WATER
        });
        if (projected_m - authoritative_m).abs() > ACCEPTED_CLOSURE_TOLERANCE_M {
            return Err(stage3_publication_guard(
                "WAT5-E-001 accepted receipt-source/ingress-ledger closure",
            ));
        }
    }
    Ok(sources)
}

fn add_accepted_wat5_depth_to_bins(
    bins: &mut [f64; runoff::WAT5_INTERVALS_PER_DAY],
    start_s: f64,
    end_s: f64,
    depth_m: f64,
) -> Result<(), DirectRuntimeError> {
    validate_nonnegative_direct_m("stage3_publication.wat5_receipt_operand_depth_m", depth_m)?;
    if depth_m == 0.0 {
        return Ok(());
    }
    let duration_s = end_s - start_s;
    if !duration_s.is_finite() || duration_s <= 0.0 {
        return Err(stage3_publication_guard(
            "accepted WAT5 receipt operand support",
        ));
    }
    for (bin_index, bin_m) in bins.iter_mut().enumerate() {
        let bin_index_u32 = u32::try_from(bin_index)
            .map_err(|_| stage3_publication_guard("accepted WAT5 bin index"))?;
        let bin_start_s = f64::from(bin_index_u32) * runoff::WAT5_INTERVAL_SECONDS;
        let bin_end_s = bin_start_s + runoff::WAT5_INTERVAL_SECONDS;
        let overlap_s = (end_s.min(bin_end_s) - start_s.max(bin_start_s)).max(0.0);
        if overlap_s > 0.0 {
            add_nonnegative(bin_m, depth_m * overlap_s / duration_s)?;
        }
    }
    Ok(())
}

fn accepted_wat5_receipt_profile(
    inputs: &DirectWb14InfiltrationProducerInputs,
    segments: &[runoff::Wat5AdditionalSupplySegmentV1],
    sources: &[AcceptedWat5ReceiptSourceV1],
    ofe_id: &OfeId,
) -> Result<runoff::DirectWb14SubhourlyProfile, DirectRuntimeError> {
    let mut profile = runoff::wat5_source_profile_with_exact_segments(
        inputs,
        segments,
        Some(ofe_id),
    )?;
    for source in sources {
        let disposition_total_m = source.infiltration_m + source.retained_surface_m + source.runoff_m;
        validate_nonnegative_direct_m(
            "stage3_publication.wat5_receipt_disposition_total_m",
            disposition_total_m,
        )?;
        if (source.depth_m - disposition_total_m).abs() > ACCEPTED_CLOSURE_TOLERANCE_M {
            return Err(stage3_publication_guard(
                "WAT5-E-001 accepted receipt source/disposition closure",
            ));
        }
        add_accepted_wat5_depth_to_bins(
            &mut profile.infiltration_m,
            source.start_s,
            source.end_s,
            source.infiltration_m,
        )?;
        add_accepted_wat5_depth_to_bins(
            &mut profile.depression_storage_retention_m,
            source.start_s,
            source.end_s,
            source.retained_surface_m,
        )?;
        add_accepted_wat5_depth_to_bins(
            &mut profile.post_depression_excess_m,
            source.start_s,
            source.end_s,
            source.runoff_m,
        )?;
        add_nonnegative(
            &mut profile.depression_storage_delta_m,
            source.retained_surface_m,
        )?;
    }
    Ok(profile)
}

fn accepted_wat5_hyetograph(
    sources: &[AcceptedWat5ReceiptSourceV1],
) -> Result<Vec<DirectWb14HyetographInterval>, DirectRuntimeError> {
    let segments = sources
        .iter()
        .filter(|source| {
            matches!(
                source.kind,
                DirectSurfaceLiquidParcelKind::RawPrecipitation
                    | DirectSurfaceLiquidParcelKind::CanopyThroughfall
                    | DirectSurfaceLiquidParcelKind::CanopyInitialDrainage
                    | DirectSurfaceLiquidParcelKind::CanopySecondDrainage
                    | DirectSurfaceLiquidParcelKind::CanopyStemflow
            )
        })
        .map(|source| AcceptedWat5RainSegmentV1 {
            start_s: source.start_s,
            end_s: source.end_s,
            depth_m: source.depth_m,
        })
        .collect::<Vec<_>>();
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

fn wat5_digest_hex(digest: Digest32) -> String {
    const HEX: &[u8; 16] = b"0123456789abcdef";
    let mut text = String::with_capacity(64);
    for byte in digest.as_bytes() {
        text.push(char::from(HEX[usize::from(byte >> 4)]));
        text.push(char::from(HEX[usize::from(byte & 0x0f)]));
    }
    text
}

fn accepted_wat5_day_support(
    day_start_ns: u128,
    support: openwepp_coupled_time::TimeSupport,
    start_s: f64,
    end_s: f64,
) -> Result<(f64, f64), DirectRuntimeError> {
    let support_offset_ns = support
        .start_ns()
        .get()
        .checked_sub(day_start_ns)
        .ok_or(stage3_publication_guard("accepted WAT5 support chronology"))?;
    let support_offset_ns = u64::try_from(support_offset_ns)
        .map_err(|_| stage3_publication_guard("accepted WAT5 support offset representation"))?;
    let support_offset_s = std::time::Duration::from_nanos(support_offset_ns).as_secs_f64();
    let day_start_s = support_offset_s + start_s;
    let day_end_s = support_offset_s + end_s;
    if !day_start_s.is_finite()
        || !day_end_s.is_finite()
        || day_start_s < 0.0
        || day_end_s <= day_start_s
        || day_end_s > 86_400.0
    {
        return Err(stage3_publication_guard(
            "accepted WAT5 additional segment support",
        ));
    }
    Ok((day_start_s, day_end_s))
}

fn accepted_wat5_additional_supply_segments(
    sources: &[AcceptedWat5ReceiptSourceV1],
    ofe_id: &OfeId,
) -> Result<Vec<runoff::Wat5AdditionalSupplySegmentV1>, DirectRuntimeError> {
    let mut segments = Vec::new();
    for source in sources {
        let source_kind = match source.kind {
            DirectSurfaceLiquidParcelKind::UpstreamRunon => {
                runoff::Wat5AdditionalSupplySourceKindV1::RoutedRunon
            }
            DirectSurfaceLiquidParcelKind::TerminalReceiver => {
                runoff::Wat5AdditionalSupplySourceKindV1::SnowTerminalReceiver
            }
            DirectSurfaceLiquidParcelKind::LitterPhaseOverflow => {
                runoff::Wat5AdditionalSupplySourceKindV1::LitterPhaseOverflow
            }
            DirectSurfaceLiquidParcelKind::CondensationOverflow => {
                runoff::Wat5AdditionalSupplySourceKindV1::CondensationOverflow
            }
            DirectSurfaceLiquidParcelKind::RawPrecipitation
            | DirectSurfaceLiquidParcelKind::CanopyThroughfall
            | DirectSurfaceLiquidParcelKind::CanopyInitialDrainage
            | DirectSurfaceLiquidParcelKind::CanopySecondDrainage
            | DirectSurfaceLiquidParcelKind::CanopyStemflow => continue,
        };
        let source_identity = format!(
            "{}:{:?}:{}",
            wat5_digest_hex(source.support_receipt_sha256),
            source.kind,
            source.source_parcel_id,
        );
        let transaction_id = format!("{:032x}", source.transaction_id.0);
        let source_receipt_sha256 = runoff::wat5_additional_supply_source_receipt_sha256(
            source_kind,
            &source_identity,
            &transaction_id,
            ofe_id,
            source.start_s,
            source.end_s,
            source.depth_m,
        )?;
        segments.push(runoff::Wat5AdditionalSupplySegmentV1 {
            source_kind,
            source_identity,
            source_receipt_sha256,
            transaction_id,
            destination_ofe_id: ofe_id.clone(),
            start_s: source.start_s,
            end_s: source.end_s,
            depth_m_ofe_ground: source.depth_m,
        });
    }
    Ok(segments)
}
