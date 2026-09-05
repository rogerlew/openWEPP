#[allow(dead_code)]
pub(super) fn validate_surface_liquid_closure_operands(
    configuration: &DirectSurfaceLiquidConfiguration,
    resource: &DirectSurfaceLiquidResourceCandidate,
    operands: &DirectSurfaceLiquidClosureOperands,
    receipts: &[DirectSurfaceLiquidParcelReceipt],
    ending: &DirectSurfaceLiquidOwnedState,
) -> Result<(), DirectSurfaceLiquidError> {
    validate_surface_liquid_closure_operands_inner(
        configuration,
        resource,
        None,
        operands,
        receipts,
        ending,
        &[],
    )
}

#[allow(dead_code)]
pub(super) fn validate_surface_liquid_closure_operands_with_input(
    configuration: &DirectSurfaceLiquidConfiguration,
    resource: &DirectSurfaceLiquidResourceCandidate,
    input: &DirectSurfaceLiquidIngressInput,
    operands: &DirectSurfaceLiquidClosureOperands,
    receipts: &[DirectSurfaceLiquidParcelReceipt],
    ending: &DirectSurfaceLiquidOwnedState,
) -> Result<(), DirectSurfaceLiquidError> {
    validate_surface_liquid_closure_operands_with_input_and_phase_capacity_spills(
        configuration,
        resource,
        input,
        operands,
        receipts,
        ending,
        &[],
    )
}

#[allow(clippy::too_many_arguments)]
pub(super) fn validate_surface_liquid_closure_operands_with_input_and_phase_capacity_spills(
    configuration: &DirectSurfaceLiquidConfiguration,
    resource: &DirectSurfaceLiquidResourceCandidate,
    input: &DirectSurfaceLiquidIngressInput,
    operands: &DirectSurfaceLiquidClosureOperands,
    receipts: &[DirectSurfaceLiquidParcelReceipt],
    ending: &DirectSurfaceLiquidOwnedState,
    phase_capacity_spills: &[LitterPhaseCapacitySpillV1],
) -> Result<(), DirectSurfaceLiquidError> {
    validate_surface_liquid_closure_operands_inner(
        configuration,
        resource,
        Some(input),
        operands,
        receipts,
        ending,
        phase_capacity_spills,
    )
}

fn validate_surface_liquid_closure_operands_inner(
    configuration: &DirectSurfaceLiquidConfiguration,
    resource: &DirectSurfaceLiquidResourceCandidate,
    input: Option<&DirectSurfaceLiquidIngressInput>,
    operands: &DirectSurfaceLiquidClosureOperands,
    receipts: &[DirectSurfaceLiquidParcelReceipt],
    ending: &DirectSurfaceLiquidOwnedState,
    phase_capacity_spills: &[LitterPhaseCapacitySpillV1],
) -> Result<(), DirectSurfaceLiquidError> {
    let result = (|| {
        if operands.transaction_id != resource.transaction_id() {
            return Err(DirectSurfaceLiquidError::Closure(
                "independent closure transaction mismatch",
            ));
        }
        // Store arithmetic owns the first closure-class comparison. Running it
        // before parcel projection preserves E010 for a finite but substituted
        // frozen store while still returning E003 for non-projectable store
        // arithmetic. Parcel preflight must not reclassify that substitution
        // merely because it changes the independently replayed capacity.
        validate_store_equations(configuration, resource, operands)?;
        preflight_surface_liquid_closure_arithmetic(configuration, resource, operands, receipts)?;
        arithmetic_preflight::validate_partition_input_identities(configuration, operands)?;
        validate_frozen_source_identities(
            configuration,
            resource,
            input,
            operands,
            phase_capacity_spills,
        )?;
        validate_parcel_joins(configuration, operands, receipts, ending)
    })();
    result.map_err(|error| {
        let code = error.code();
        error.complete_context(
            code,
            DirectSurfaceLiquidPhase::IndependentClosure,
            DirectSurfaceLiquidErrorContext {
                transaction_id: Some(operands.transaction_id),
                owner_id: Some(configuration.owner_id.clone()),
                ..DirectSurfaceLiquidErrorContext::default()
            },
            Some(resource.beginning_state().state_sha256.clone()),
            ending.recomputed_sha256().ok(),
        )
    })
}

fn validate_frozen_source_identities(
    configuration: &DirectSurfaceLiquidConfiguration,
    resource: &DirectSurfaceLiquidResourceCandidate,
    input: Option<&DirectSurfaceLiquidIngressInput>,
    operands: &DirectSurfaceLiquidClosureOperands,
    phase_capacity_spills: &[LitterPhaseCapacitySpillV1],
) -> Result<(), DirectSurfaceLiquidError> {
    let mut expected = Vec::new();
    for record in &configuration.records {
        expected.extend(frozen_identities_for_record(record, input)?);
    }
    for overflow in resource.condensation_overflow() {
        expected.push(FrozenSourceIdentity {
            source_parcel_id: canonical_surface_liquid_source_id(
                CanonicalSurfaceLiquidSource::Condensation {
                    transaction_id: operands.transaction_id,
                    store_key: &overflow.store_key,
                },
            ),
            kind: DirectSurfaceLiquidParcelKind::CondensationOverflow,
            origin_store_key: overflow.store_key.clone(),
            basis_ofe_id: overflow.store_key.ofe_id.clone(),
            start_s_bits: 0.0_f64.to_bits(),
            end_s_bits: operands.interval_s.to_bits(),
        });
    }
    for spill in phase_capacity_spills {
        let configured = configuration
            .records
            .iter()
            .find(|record| record.key.ofe_id == spill.ofe_id && record.key.tile_id == spill.tile_id)
            .ok_or(DirectSurfaceLiquidError::Identity(
                "phase-spill frozen source key",
            ))?;
        expected.push(FrozenSourceIdentity {
            source_parcel_id: canonical_surface_liquid_source_id(
                CanonicalSurfaceLiquidSource::LitterPhaseOverflow {
                    transaction_id: operands.transaction_id,
                    store_key: &configured.key,
                    phase_receipt_sha256: spill.phase_receipt_sha256.as_str(),
                },
            ),
            kind: DirectSurfaceLiquidParcelKind::LitterPhaseOverflow,
            origin_store_key: configured.key.clone(),
            basis_ofe_id: configured.key.ofe_id.clone(),
            start_s_bits: 0.0_f64.to_bits(),
            end_s_bits: operands.interval_s.to_bits(),
        });
    }
    if input.is_none() {
        align_unfrozen_support(&mut expected, &operands.source_parcels);
    }
    expected.sort_by(frozen_source_identity_order);
    let actual = operands
        .source_parcels
        .iter()
        .map(FrozenSourceIdentity::from)
        .collect::<Vec<_>>();
    if actual == expected {
        return Ok(());
    }
    let offending = frozen_identity_mismatch(&actual, &expected)?;
    Err(contextual_closure_failure(
        operands.transaction_id,
        &offending.origin_store_key,
        Some(offending.source_parcel_id.clone()),
        "frozen source parcel identity mismatch",
    ))
}

fn frozen_identities_for_record(
    record: &DirectSurfaceLiquidConfigurationRecord,
    input: Option<&DirectSurfaceLiquidIngressInput>,
) -> Result<Vec<FrozenSourceIdentity>, DirectSurfaceLiquidError> {
    if let Some(input) = input {
        let ingress = input
            .tile_ingress
            .iter()
            .find(|ingress| {
                ingress.identity()
                    == (
                        &record.key.ofe_id,
                        &record.key.tile_id,
                        &record.key.surface_id,
                    )
            })
            .ok_or(DirectSurfaceLiquidError::Closure(
                "missing ingress for frozen source identity",
            ))?;
        match ingress {
            DirectTileGroundIngress::OpenRawPrecipitation {
                raw_precipitation, ..
            } => {
                return Ok(vec![local_frozen_identity(
                    record,
                    DirectSurfaceLiquidParcelKind::RawPrecipitation,
                    raw_precipitation.start_s,
                    raw_precipitation.end_s,
                )]);
            }
            DirectTileGroundIngress::OpenLiquidParcels { parcels, .. } => {
                return Ok(parcels
                    .iter()
                    .map(|parcel| FrozenSourceIdentity {
                        source_parcel_id: parcel.parcel_id.to_string(),
                        kind: parcel.kind,
                        origin_store_key: record.key.clone(),
                        basis_ofe_id: record.key.ofe_id.clone(),
                        start_s_bits: parcel.amount.start_s.to_bits(),
                        end_s_bits: parcel.amount.end_s.to_bits(),
                    })
                    .collect());
            }
            DirectTileGroundIngress::CoveredCanopyRelease { .. } => {}
            DirectTileGroundIngress::CoveredCanopyReleaseAndRunon { runon_parcels, .. } => {
                let mut identities = covered_canonical_frozen_identities(record, input.interval_s);
                identities.extend(runon_parcels.iter().map(|parcel| FrozenSourceIdentity {
                    source_parcel_id: parcel.parcel_id.to_string(),
                    kind: parcel.kind,
                    origin_store_key: record.key.clone(),
                    basis_ofe_id: record.key.ofe_id.clone(),
                    start_s_bits: parcel.amount.start_s.to_bits(),
                    end_s_bits: parcel.amount.end_s.to_bits(),
                }));
                return Ok(identities);
            }
        }
    }
    let kinds: &[DirectSurfaceLiquidParcelKind] = match record.ground_ingress_mode {
        super::surface_liquid_owner::DirectGroundIngressMode::OpenRawPrecipitation => {
            &[DirectSurfaceLiquidParcelKind::RawPrecipitation]
        }
        super::surface_liquid_owner::DirectGroundIngressMode::CoveredCanopyRelease => &[
            DirectSurfaceLiquidParcelKind::CanopyThroughfall,
            DirectSurfaceLiquidParcelKind::CanopyInitialDrainage,
            DirectSurfaceLiquidParcelKind::CanopySecondDrainage,
            DirectSurfaceLiquidParcelKind::CanopyStemflow,
        ],
    };
    let interval_s = input.map_or(INTERVAL_S, |value| value.interval_s);
    Ok(kinds
        .iter()
        .map(|kind| local_frozen_identity(record, *kind, 0.0, interval_s))
        .collect())
}

fn covered_canonical_frozen_identities(
    record: &DirectSurfaceLiquidConfigurationRecord,
    interval_s: f64,
) -> Vec<FrozenSourceIdentity> {
    [
        DirectSurfaceLiquidParcelKind::CanopyThroughfall,
        DirectSurfaceLiquidParcelKind::CanopyInitialDrainage,
        DirectSurfaceLiquidParcelKind::CanopySecondDrainage,
        DirectSurfaceLiquidParcelKind::CanopyStemflow,
    ]
    .into_iter()
    .map(|kind| local_frozen_identity(record, kind, 0.0, interval_s))
    .collect()
}

fn local_frozen_identity(
    record: &DirectSurfaceLiquidConfigurationRecord,
    kind: DirectSurfaceLiquidParcelKind,
    start_s: f64,
    end_s: f64,
) -> FrozenSourceIdentity {
    FrozenSourceIdentity {
        source_parcel_id: canonical_surface_liquid_source_id(CanonicalSurfaceLiquidSource::Local {
            store_key: &record.key,
            kind,
        }),
        kind,
        origin_store_key: record.key.clone(),
        basis_ofe_id: record.key.ofe_id.clone(),
        start_s_bits: start_s.to_bits(),
        end_s_bits: end_s.to_bits(),
    }
}

fn align_unfrozen_support(
    expected: &mut [FrozenSourceIdentity],
    actual: &[DirectSurfaceLiquidParcelClosureOperands],
) {
    for expected_row in expected {
        if let Some(actual_row) = actual
            .iter()
            .find(|row| row.source_parcel_id == expected_row.source_parcel_id)
        {
            expected_row.start_s_bits = actual_row.start_s.to_bits();
            expected_row.end_s_bits = actual_row.end_s.to_bits();
        }
    }
}

fn frozen_identity_mismatch<'a>(
    actual: &'a [FrozenSourceIdentity],
    expected: &'a [FrozenSourceIdentity],
) -> Result<&'a FrozenSourceIdentity, DirectSurfaceLiquidError> {
    let actual_set = actual.iter().cloned().collect::<BTreeSet<_>>();
    let mismatch = if actual.len() < expected.len() {
        expected
            .iter()
            .find(|row| !actual_set.contains(*row))
            .or_else(|| actual.first())
    } else {
        actual
            .iter()
            .zip(expected)
            .find(|(actual_row, expected_row)| actual_row != expected_row)
            .map(|(actual_row, _)| actual_row)
            .or_else(|| actual.get(expected.len()))
            .or_else(|| expected.first())
    };
    mismatch.ok_or(DirectSurfaceLiquidError::Closure(
        "empty frozen source identity mismatch",
    ))
}

#[allow(clippy::too_many_lines)]
fn parcel_join_key(
    owner_id: &ResourceOwnerId,
    segment: &RawParcelSegment,
    recipient: DirectSurfaceLiquidReceiptRecipient,
    disposition: DirectSurfaceLiquidReceiptDisposition,
    start_s: f64,
    end_s: f64,
) -> ParcelJoinKey {
    ParcelJoinKey {
        owner_id: owner_id.clone(),
        source_parcel_id: segment.source_parcel_id.clone(),
        origin_store_key: segment.origin_store_key.clone(),
        recipient_store_key: segment.recipient_store_key.clone(),
        recipient,
        basis_ofe_id: segment.basis_ofe_id.clone(),
        kind: segment.kind,
        start_s_bits: start_s.to_bits(),
        end_s_bits: end_s.to_bits(),
        disposition: Some(disposition),
    }
}

fn receipt_join_key(
    owner_id: &ResourceOwnerId,
    receipt: &DirectSurfaceLiquidParcelReceipt,
) -> ParcelJoinKey {
    ParcelJoinKey {
        owner_id: owner_id.clone(),
        source_parcel_id: receipt.source_parcel_id.clone(),
        origin_store_key: receipt.origin_store_key.clone(),
        recipient_store_key: receipt.recipient_store_key.clone(),
        recipient: receipt.recipient.clone(),
        basis_ofe_id: receipt.basis_ofe_id.clone(),
        kind: receipt.kind,
        start_s_bits: receipt.start_s.to_bits(),
        end_s_bits: receipt.end_s.to_bits(),
        disposition: Some(receipt.disposition),
    }
}

fn add_expected_partition(
    expected: &mut BTreeMap<ParcelJoinKey, AmountPair>,
    key: ParcelJoinKey,
    mass: f64,
    enthalpy: f64,
) -> Option<()> {
    if !mass.is_finite() || mass < 0.0 || !enthalpy.is_finite() {
        return None;
    }
    expected.entry(key).or_default().checked_add(mass, enthalpy)
}

fn add_expected_temperature(
    expected: &mut BTreeMap<ParcelJoinKey, f64>,
    key: ParcelJoinKey,
    temperature_k: f64,
) -> Option<()> {
    if !temperature_k.is_finite() || !(200.0..=350.0).contains(&temperature_k) {
        return None;
    }
    match expected.entry(key) {
        std::collections::btree_map::Entry::Vacant(entry) => {
            entry.insert(temperature_k);
            Some(())
        }
        std::collections::btree_map::Entry::Occupied(entry) => {
            (entry.get().to_bits() == temperature_k.to_bits()).then_some(())
        }
    }
}

fn project_actual_receipt_arithmetic(
    configuration: &DirectSurfaceLiquidConfiguration,
    operands: &DirectSurfaceLiquidClosureOperands,
    receipts: &[DirectSurfaceLiquidParcelReceipt],
) -> Result<BTreeMap<ParcelJoinKey, AmountPair>, DirectSurfaceLiquidError> {
    let mut actual = BTreeMap::<ParcelJoinKey, AmountPair>::new();
    for receipt in receipts {
        actual
            .entry(receipt_join_key(&configuration.owner_id, receipt))
            .or_default()
            .checked_add(
                receipt.mass_kg_m2_basis_ofe_ground,
                receipt.enthalpy_j_m2_basis_ofe_ground,
            )
            .ok_or_else(|| {
                contextual_comparison_failure(
                    DirectSurfaceLiquidErrorCode::E003,
                    operands.transaction_id,
                    &configuration.owner_id,
                    &receipt.recipient_store_key,
                    Some(receipt.parcel_id.clone()),
                    "receipt aggregate arithmetic",
                )
            })?;
    }
    Ok(actual)
}

#[allow(clippy::too_many_lines)]
fn project_parcel_arithmetic(
    configuration: &DirectSurfaceLiquidConfiguration,
    operands: &DirectSurfaceLiquidClosureOperands,
    receipts: &[DirectSurfaceLiquidParcelReceipt],
) -> Result<ParcelArithmeticProjection, DirectSurfaceLiquidError> {
    let actual = project_actual_receipt_arithmetic(configuration, operands, receipts)?;

    let mut raw_segments = operands
        .source_parcels
        .iter()
        .map(|parcel| RawParcelSegment {
            source_parcel_id: parcel.source_parcel_id.clone(),
            basis_ofe_id: parcel.basis_ofe_id.clone(),
            origin_store_key: parcel.origin_store_key.clone(),
            recipient_store_key: parcel.origin_store_key.clone(),
            kind: parcel.kind,
            start_s: parcel.start_s,
            end_s: parcel.end_s,
            mass: parcel.mass_kg_m2_basis_ofe_ground,
            enthalpy: parcel.enthalpy_j_m2_basis_ofe_ground,
        })
        .collect::<Vec<_>>();
    let mut store_liquid = operands
        .stores
        .iter()
        .map(|store| {
            project_store_arithmetic(store)
                .map(|projected| {
                    (
                        store.store_key.clone(),
                        projected.pre_ingress_liquid_kg_m2_tile,
                    )
                })
                .ok_or_else(|| {
                    contextual_comparison_failure(
                        DirectSurfaceLiquidErrorCode::E003,
                        operands.transaction_id,
                        &configuration.owner_id,
                        &store.store_key,
                        None,
                        "partition beginning-store arithmetic",
                    )
                })
        })
        .collect::<Result<BTreeMap<_, _>, _>>()?;

    let mut expected = BTreeMap::<ParcelJoinKey, AmountPair>::new();
    let mut expected_temperature_k = BTreeMap::<ParcelJoinKey, f64>::new();
    let (raw_ofe_mass, raw_source_mass) =
        raw_parent_reconstruction::reconstruct_raw_parent_mass(configuration, operands)?;
    let mut replayed_ofe_enthalpy = BTreeMap::<OfeId, f64>::new();
    let mut expected_continuations = BTreeMap::<OfeId, DirectProjectedContinuation>::new();
    for ofe_id in &configuration.ofe_topology {
        let partition = operands
            .partition_inputs
            .iter()
            .find(|row| &row.ofe_id == ofe_id)
            .ok_or_else(|| {
                contextual_ofe_comparison_failure(
                    DirectSurfaceLiquidErrorCode::E003,
                    operands.transaction_id,
                    &configuration.owner_id,
                    ofe_id,
                    "frozen partition inputs missing",
                )
            })?;
        let binding = configuration
            .ofe_bindings
            .iter()
            .find(|row| &row.ofe_id == ofe_id)
            .ok_or(DirectSurfaceLiquidError::Closure(
                "partition infiltration binding missing",
            ))?;
        let route = configuration
            .records
            .iter()
            .find(|row| &row.key.ofe_id == ofe_id)
            .ok_or(DirectSurfaceLiquidError::Closure(
                "partition route source missing",
            ))?;
        let segments = raw_segments
            .iter()
            .filter(|segment| &segment.basis_ofe_id == ofe_id)
            .cloned()
            .collect::<Vec<_>>();
        let mut boundaries = segments
            .iter()
            .flat_map(|segment| [segment.start_s, segment.end_s])
            .collect::<Vec<_>>();
        boundaries.extend([0.0, operands.interval_s]);
        boundaries.sort_by(f64::total_cmp);
        boundaries.dedup_by(|left, right| left.to_bits() == right.to_bits());

        let mut cumulative_supply_m = partition.beginning_cumulative_supply_m;
        let mut cumulative_infiltration_m = partition.beginning_cumulative_infiltration_m;
        let mut routed_segments = Vec::new();
        let mut allocated_temporal_mass = vec![0.0; segments.len()];
        let mut allocated_temporal_enthalpy = vec![0.0; segments.len()];
        for window in boundaries.windows(2) {
            let start_s = window[0];
            let end_s = window[1];
            if end_s <= start_s {
                continue;
            }
            let mut contributions = segments
                .iter()
                .enumerate()
                .filter(|(_, segment)| segment.start_s <= start_s && segment.end_s >= end_s)
                .map(|(segment_index, segment)| {
                    let fraction = checked_surface_liquid_div(
                        end_s - start_s,
                        segment.end_s - segment.start_s,
                    )?;
                    // Replay the producer's temporal child identity separately
                    // so receipt bits remain independently checked. Raw mass
                    // custody is reconstructed above only from frozen parents,
                    // never from these replayed children.
                    let is_last = end_s.to_bits() == segment.end_s.to_bits();
                    let (mass, allocated) = enthalpy_reconstruction::allocate_ordered_child(
                        segment.mass,
                        allocated_temporal_mass[segment_index],
                        checked_surface_liquid_mul(segment.mass, fraction),
                        is_last,
                    )?;
                    if mass < 0.0 {
                        return None;
                    }
                    allocated_temporal_mass[segment_index] = allocated;
                    let (enthalpy, allocated) = enthalpy_reconstruction::allocate_ordered_child(
                        segment.enthalpy,
                        allocated_temporal_enthalpy[segment_index],
                        enthalpy_reconstruction::proportional_q(
                            segment.enthalpy,
                            end_s - start_s,
                            segment.end_s - segment.start_s,
                        ),
                        is_last,
                    )?;
                    allocated_temporal_enthalpy[segment_index] = allocated;
                    Some((segment.clone(), mass, enthalpy))
                })
                .collect::<Option<Vec<_>>>()
                .ok_or_else(|| {
                    contextual_ofe_comparison_failure(
                        DirectSurfaceLiquidErrorCode::E003,
                        operands.transaction_id,
                        &configuration.owner_id,
                        ofe_id,
                        "partition window projection arithmetic",
                    )
                })?;
            contributions.retain(|row| row.1 > 0.0);
            contributions.sort_by(|left, right| projected_parcel_order(&left.0, &right.0));
            let supply_mass = checked_surface_liquid_sum(contributions.iter().map(|row| row.1))
                .ok_or_else(|| {
                    contextual_ofe_comparison_failure(
                        DirectSurfaceLiquidErrorCode::E003,
                        operands.transaction_id,
                        &configuration.owner_id,
                        ofe_id,
                        "partition supply mass arithmetic",
                    )
                })?;
            let supply_enthalpy = checked_surface_liquid_sum(contributions.iter().map(|row| row.2))
                .ok_or_else(|| {
                    contextual_ofe_comparison_failure(
                        DirectSurfaceLiquidErrorCode::E003,
                        operands.transaction_id,
                        &configuration.owner_id,
                        ofe_id,
                        "partition supply enthalpy arithmetic",
                    )
                })?;
            if supply_mass == 0.0 {
                continue;
            }
            let raw_enthalpy = replayed_ofe_enthalpy.entry(ofe_id.clone()).or_default();
            *raw_enthalpy =
                checked_surface_liquid_add(*raw_enthalpy, supply_enthalpy).ok_or_else(|| {
                    contextual_ofe_comparison_failure(
                        DirectSurfaceLiquidErrorCode::E003,
                        operands.transaction_id,
                        &configuration.owner_id,
                        ofe_id,
                        "partition raw enthalpy aggregate arithmetic",
                    )
                })?;
            let duration_s = end_s - start_s;
            let interval_supply_m = checked_surface_liquid_div(supply_mass, WATER_DENSITY_KG_M3)
                .ok_or_else(|| {
                    contextual_ofe_comparison_failure(
                        DirectSurfaceLiquidErrorCode::E003,
                        operands.transaction_id,
                        &configuration.owner_id,
                        ofe_id,
                        "partition supply depth arithmetic",
                    )
                })?;
            let outcome =
                advance_wb14_continuation_interval(DirectWb14ContinuationIntervalInputs {
                    cumulative_supply_m,
                    cumulative_infiltration_m,
                    interval_supply_m,
                    interval_duration_s: duration_s,
                    effective_conductivity_m_s: partition.effective_conductivity_m_s,
                    matric_potential_m: partition.matric_potential_m,
                    storage_capacity_m: partition.infiltration_storage_capacity_m,
                })
                .map_err(|_| {
                    contextual_ofe_comparison_failure(
                        DirectSurfaceLiquidErrorCode::E003,
                        operands.transaction_id,
                        &configuration.owner_id,
                        ofe_id,
                        "independent WB14 partition rejected",
                    )
                })?;
            cumulative_supply_m = outcome.cumulative_supply_m;
            cumulative_infiltration_m = outcome.cumulative_infiltration_m;
            let full_infiltration =
                outcome.interval_infiltration_m.to_bits() == interval_supply_m.to_bits();
            let total_infiltration = if full_infiltration {
                supply_mass
            } else {
                checked_surface_liquid_mul(outcome.interval_infiltration_m, WATER_DENSITY_KG_M3)
                    .ok_or_else(|| {
                        contextual_ofe_comparison_failure(
                            DirectSurfaceLiquidErrorCode::E003,
                            operands.transaction_id,
                            &configuration.owner_id,
                            ofe_id,
                            "partition infiltration mass arithmetic",
                        )
                    })?
            };
            let h_mix =
                checked_surface_liquid_div(supply_enthalpy, supply_mass).ok_or_else(|| {
                    contextual_ofe_comparison_failure(
                        DirectSurfaceLiquidErrorCode::E003,
                        operands.transaction_id,
                        &configuration.owner_id,
                        ofe_id,
                        "partition mixture enthalpy arithmetic",
                    )
                })?;
            let temperature_k = checked_surface_liquid_div(h_mix, LIQUID_HEAT_CAPACITY_J_KG_K)
                .and_then(|offset| checked_surface_liquid_add(REFERENCE_TEMPERATURE_K, offset))
                .ok_or_else(|| {
                    contextual_ofe_comparison_failure(
                        DirectSurfaceLiquidErrorCode::E003,
                        operands.transaction_id,
                        &configuration.owner_id,
                        ofe_id,
                        "partition mixture temperature arithmetic",
                    )
                })?;
            let mut allocated_infiltration = 0.0;
            let mut allocated_excess = 0.0;
            let mut allocated_mixed_enthalpy = 0.0;
            let count = contributions.len();
            let use_excess_authority =
                enthalpy_reconstruction::direct_infiltration_requires_excess_authority(
                    total_infiltration,
                    supply_mass,
                    contributions.iter().map(|row| row.1),
                )
                .ok_or_else(|| {
                    contextual_ofe_comparison_failure(
                        DirectSurfaceLiquidErrorCode::E003,
                        operands.transaction_id,
                        &configuration.owner_id,
                        ofe_id,
                        "partition infiltration authority arithmetic",
                    )
                })?;
            let mut excess_by_store =
                BTreeMap::<DirectSurfaceLiquidStoreKey, Vec<(RawParcelSegment, f64, f64)>>::new();
            for (index, (segment, mass, _)) in contributions.into_iter().enumerate() {
                let (mixed_part_q, allocated) = enthalpy_reconstruction::allocate_ordered_child(
                    supply_enthalpy,
                    allocated_mixed_enthalpy,
                    enthalpy_reconstruction::proportional_q(supply_enthalpy, mass, supply_mass),
                    index + 1 == count,
                )
                .ok_or_else(|| {
                    contextual_ofe_comparison_failure(
                        DirectSurfaceLiquidErrorCode::E003,
                        operands.transaction_id,
                        &configuration.owner_id,
                        ofe_id,
                        "partition mixed enthalpy allocation arithmetic",
                    )
                })?;
                allocated_mixed_enthalpy = allocated;
                let (infiltrated, excess) =
                    enthalpy_reconstruction::allocate_infiltration_and_excess(
                        total_infiltration,
                        supply_mass,
                        allocated_infiltration,
                        allocated_excess,
                        mass,
                        index + 1 == count,
                        use_excess_authority,
                    )
                    .ok_or_else(|| {
                        contextual_ofe_comparison_failure(
                            DirectSurfaceLiquidErrorCode::E003,
                            operands.transaction_id,
                            &configuration.owner_id,
                            ofe_id,
                            "partition infiltration allocation arithmetic",
                        )
                    })?;
                allocated_infiltration =
                    checked_surface_liquid_add(allocated_infiltration, infiltrated).ok_or_else(
                        || {
                            contextual_ofe_comparison_failure(
                                DirectSurfaceLiquidErrorCode::E003,
                                operands.transaction_id,
                                &configuration.owner_id,
                                ofe_id,
                                "partition infiltration accumulation arithmetic",
                            )
                        },
                    )?;
                allocated_excess = checked_surface_liquid_add(allocated_excess, excess)
                    .ok_or_else(|| {
                        contextual_ofe_comparison_failure(
                            DirectSurfaceLiquidErrorCode::E003,
                            operands.transaction_id,
                            &configuration.owner_id,
                            ofe_id,
                            "partition excess accumulation arithmetic",
                        )
                    })?;
                let infiltration_recipient =
                    DirectSurfaceLiquidReceiptRecipient::SoilInfiltration {
                        ofe_id: binding.ofe_id.clone(),
                        production_lane_index: binding.production_lane_index,
                        production_lane_id: binding.production_lane_id,
                        ordered_soil_layer_ids: binding.ordered_soil_layer_ids.clone(),
                        soil_thermal_layer_id: binding.infiltration_soil_thermal_layer_id.clone(),
                    };
                let key = parcel_join_key(
                    &configuration.owner_id,
                    &segment,
                    infiltration_recipient,
                    DirectSurfaceLiquidReceiptDisposition::Infiltration,
                    start_s,
                    end_s,
                );
                let (infiltration_q, excess_q) =
                    enthalpy_reconstruction::split_first_then_remainder(
                        mixed_part_q,
                        mass,
                        infiltrated,
                        excess,
                    )
                    .ok_or_else(|| {
                        contextual_ofe_comparison_failure(
                            DirectSurfaceLiquidErrorCode::E003,
                            operands.transaction_id,
                            &configuration.owner_id,
                            ofe_id,
                            "partition infiltration enthalpy arithmetic",
                        )
                    })?;
                add_expected_temperature(&mut expected_temperature_k, key.clone(), temperature_k)
                    .and_then(|()| {
                        add_expected_partition(&mut expected, key, infiltrated, infiltration_q)
                    })
                    .ok_or_else(|| {
                        contextual_ofe_comparison_failure(
                            DirectSurfaceLiquidErrorCode::E003,
                            operands.transaction_id,
                            &configuration.owner_id,
                            ofe_id,
                            "partition infiltration enthalpy arithmetic",
                        )
                    })?;
                excess_by_store
                    .entry(segment.recipient_store_key.clone())
                    .or_default()
                    .push((segment, excess, excess_q));
            }

            for (store_key, mut parts) in excess_by_store {
                parts.sort_by(|left, right| projected_parcel_order(&left.0, &right.0));
                let configured = configuration
                    .records
                    .iter()
                    .find(|row| row.key == store_key)
                    .ok_or(DirectSurfaceLiquidError::Closure(
                        "partition recipient store missing",
                    ))?;
                let current_liquid = store_liquid.get(&store_key).copied().ok_or(
                    DirectSurfaceLiquidError::Closure("partition store state missing"),
                )?;
                let available_tile =
                    checked_surface_liquid_sub(configured.capacity_kg_m2_tile, current_liquid)
                        .ok_or_else(|| {
                            contextual_comparison_failure(
                                DirectSurfaceLiquidErrorCode::E003,
                                operands.transaction_id,
                                &configuration.owner_id,
                                &store_key,
                                None,
                                "partition retention capacity arithmetic",
                            )
                        })?;
                let available =
                    checked_surface_liquid_mul(configured.tile_fraction, available_tile)
                        .ok_or_else(|| {
                            contextual_comparison_failure(
                                DirectSurfaceLiquidErrorCode::E003,
                                operands.transaction_id,
                                &configuration.owner_id,
                                &store_key,
                                None,
                                "partition retention area arithmetic",
                            )
                        })?;
                let total_excess = checked_surface_liquid_sum(parts.iter().map(|row| row.1))
                    .ok_or_else(|| {
                        contextual_comparison_failure(
                            DirectSurfaceLiquidErrorCode::E003,
                            operands.transaction_id,
                            &configuration.owner_id,
                            &store_key,
                            None,
                            "partition excess aggregate arithmetic",
                        )
                    })?;
                let capacity_ofe = checked_surface_liquid_mul(
                    configured.tile_fraction,
                    configured.capacity_kg_m2_tile,
                )
                .ok_or_else(|| {
                    contextual_comparison_failure(
                        DirectSurfaceLiquidErrorCode::E003,
                        operands.transaction_id,
                        &configuration.owner_id,
                        &store_key,
                        None,
                        "partition capacity envelope arithmetic",
                    )
                })?;
                let stored_ofe =
                    checked_surface_liquid_mul(configured.tile_fraction, current_liquid)
                        .ok_or_else(|| {
                            contextual_comparison_failure(
                                DirectSurfaceLiquidErrorCode::E003,
                                operands.transaction_id,
                                &configuration.owner_id,
                                &store_key,
                                None,
                                "partition storage envelope arithmetic",
                            )
                        })?;
                let raw_retained = total_excess.min(available);
                let total_retained = independently_project_effective_retained_mass(
                    raw_retained,
                    capacity_ofe,
                    stored_ofe,
                    total_excess,
                )
                .ok_or_else(|| {
                    contextual_comparison_failure(
                        DirectSurfaceLiquidErrorCode::E003,
                        operands.transaction_id,
                        &configuration.owner_id,
                        &store_key,
                        None,
                        "partition representational-credit envelope arithmetic",
                    )
                })?;
                let mut allocated_retained = 0.0;
                let part_count = parts.len();
                for (index, (segment, excess, excess_q)) in parts.into_iter().enumerate() {
                    let retained = enthalpy_reconstruction::allocate_retained_mass(
                        total_retained,
                        total_excess,
                        allocated_retained,
                        excess,
                        index + 1 == part_count,
                    )
                    .ok_or_else(|| {
                        contextual_comparison_failure(
                            DirectSurfaceLiquidErrorCode::E003,
                            operands.transaction_id,
                            &configuration.owner_id,
                            &store_key,
                            Some(segment.source_parcel_id.clone()),
                            "partition retained allocation arithmetic",
                        )
                    })?;
                    allocated_retained = checked_surface_liquid_add(allocated_retained, retained)
                        .ok_or_else(|| {
                        contextual_comparison_failure(
                            DirectSurfaceLiquidErrorCode::E003,
                            operands.transaction_id,
                            &configuration.owner_id,
                            &store_key,
                            Some(segment.source_parcel_id.clone()),
                            "partition retained accumulation arithmetic",
                        )
                    })?;
                    let runoff = checked_surface_liquid_sub(excess, retained).ok_or_else(|| {
                        contextual_comparison_failure(
                            DirectSurfaceLiquidErrorCode::E003,
                            operands.transaction_id,
                            &configuration.owner_id,
                            &store_key,
                            Some(segment.source_parcel_id.clone()),
                            "partition runoff arithmetic",
                        )
                    })?;
                    let (retained_q, runoff_q) =
                        enthalpy_reconstruction::split_first_then_remainder(
                            excess_q, excess, retained, runoff,
                        )
                        .ok_or_else(|| {
                            contextual_comparison_failure(
                                DirectSurfaceLiquidErrorCode::E003,
                                operands.transaction_id,
                                &configuration.owner_id,
                                &store_key,
                                Some(segment.source_parcel_id.clone()),
                                "partition retained enthalpy arithmetic",
                            )
                        })?;
                    if retained > 0.0 {
                        let key = parcel_join_key(
                            &configuration.owner_id,
                            &segment,
                            DirectSurfaceLiquidReceiptRecipient::SurfaceStore {
                                store_key: store_key.clone(),
                            },
                            DirectSurfaceLiquidReceiptDisposition::RetainedSurface,
                            start_s,
                            end_s,
                        );
                        add_expected_temperature(
                            &mut expected_temperature_k,
                            key.clone(),
                            temperature_k,
                        )
                        .and_then(|()| {
                            add_expected_partition(&mut expected, key, retained, retained_q)
                        })
                        .ok_or_else(|| {
                            contextual_comparison_failure(
                                DirectSurfaceLiquidErrorCode::E003,
                                operands.transaction_id,
                                &configuration.owner_id,
                                &store_key,
                                Some(segment.source_parcel_id.clone()),
                                "partition retained enthalpy arithmetic",
                            )
                        })?;
                    }
                    if runoff == 0.0 {
                        continue;
                    }
                    let (disposition, recipient) = if let (
                        Some(destination_ofe),
                        Some(destination_tile),
                    ) = (
                        route.runon_destination_ofe_id.as_ref(),
                        route.runon_destination_tile_id.as_ref(),
                    ) {
                        let destination = configuration
                            .records
                            .iter()
                            .find(|row| {
                                &row.key.ofe_id == destination_ofe
                                    && &row.key.tile_id == destination_tile
                            })
                            .ok_or(DirectSurfaceLiquidError::Closure(
                                "partition route destination missing",
                            ))?;
                        let area_ratio =
                            checked_surface_liquid_div(route.ofe_area_m2, destination.ofe_area_m2)
                                .ok_or_else(|| {
                                    contextual_comparison_failure(
                                        DirectSurfaceLiquidErrorCode::E003,
                                        operands.transaction_id,
                                        &configuration.owner_id,
                                        &store_key,
                                        Some(segment.source_parcel_id.clone()),
                                        "partition route area arithmetic",
                                    )
                                })?;
                        let routed_mass = checked_surface_liquid_mul(runoff, area_ratio)
                            .ok_or_else(|| {
                                contextual_comparison_failure(
                                    DirectSurfaceLiquidErrorCode::E003,
                                    operands.transaction_id,
                                    &configuration.owner_id,
                                    &store_key,
                                    Some(segment.source_parcel_id.clone()),
                                    "partition routed mass arithmetic",
                                )
                            })?;
                        let routed_enthalpy = checked_surface_liquid_mul(runoff_q, area_ratio)
                            .ok_or_else(|| {
                                contextual_comparison_failure(
                                    DirectSurfaceLiquidErrorCode::E003,
                                    operands.transaction_id,
                                    &configuration.owner_id,
                                    &store_key,
                                    Some(segment.source_parcel_id.clone()),
                                    "partition routed enthalpy arithmetic",
                                )
                            })?;
                        routed_segments.push(RawParcelSegment {
                            source_parcel_id: segment.source_parcel_id.clone(),
                            basis_ofe_id: destination_ofe.clone(),
                            origin_store_key: segment.origin_store_key.clone(),
                            recipient_store_key: destination.key.clone(),
                            kind: DirectSurfaceLiquidParcelKind::UpstreamRunon,
                            start_s,
                            end_s,
                            mass: routed_mass,
                            enthalpy: routed_enthalpy,
                        });
                        (
                            DirectSurfaceLiquidReceiptDisposition::RoutedRunoff,
                            DirectSurfaceLiquidReceiptRecipient::RoutedOfe {
                                source_ofe_id: segment.basis_ofe_id.clone(),
                                destination_ofe_id: destination_ofe.clone(),
                                destination_store_key: destination.key.clone(),
                            },
                        )
                    } else {
                        (
                            DirectSurfaceLiquidReceiptDisposition::OutletRunoff,
                            DirectSurfaceLiquidReceiptRecipient::Outlet {
                                ofe_id: segment.basis_ofe_id.clone(),
                            },
                        )
                    };
                    let key = parcel_join_key(
                        &configuration.owner_id,
                        &segment,
                        recipient,
                        disposition,
                        start_s,
                        end_s,
                    );
                    add_expected_temperature(
                        &mut expected_temperature_k,
                        key.clone(),
                        temperature_k,
                    )
                    .and_then(|()| add_expected_partition(&mut expected, key, runoff, runoff_q))
                    .ok_or_else(|| {
                        contextual_comparison_failure(
                            DirectSurfaceLiquidErrorCode::E003,
                            operands.transaction_id,
                            &configuration.owner_id,
                            &store_key,
                            Some(segment.source_parcel_id.clone()),
                            "partition runoff enthalpy arithmetic",
                        )
                    })?;
                }
                let ending_liquid = independently_project_ending_store(
                    current_liquid,
                    configured.capacity_kg_m2_tile,
                    configured.tile_fraction,
                    available_tile,
                    available,
                    total_retained,
                )
                .ok_or_else(|| {
                    contextual_comparison_failure(
                        DirectSurfaceLiquidErrorCode::E003,
                        operands.transaction_id,
                        &configuration.owner_id,
                        &store_key,
                        None,
                        "partition retained store update arithmetic",
                    )
                })?;
                store_liquid.insert(store_key, ending_liquid);
            }
        }
        expected_continuations.insert(
            ofe_id.clone(),
            DirectProjectedContinuation {
                day_index: partition.ending_day_index,
                next_interval_index: partition.ending_next_interval_index,
                cumulative_supply_m,
                cumulative_infiltration_m,
                transaction_id: operands.transaction_id,
            },
        );
        raw_segments.extend(routed_segments);
    }

    let mut expected_source_mass = BTreeMap::<(OfeId, String), f64>::new();
    let mut expected_ofe_mass = BTreeMap::<OfeId, f64>::new();
    let mut expected_ofe_enthalpy = BTreeMap::<OfeId, f64>::new();
    for (key, amount) in &expected {
        let source_mass = expected_source_mass
            .entry((key.basis_ofe_id.clone(), key.source_parcel_id.clone()))
            .or_default();
        *source_mass = checked_surface_liquid_add(*source_mass, amount.mass).ok_or_else(|| {
            contextual_comparison_failure(
                DirectSurfaceLiquidErrorCode::E003,
                operands.transaction_id,
                &configuration.owner_id,
                &key.recipient_store_key,
                Some(key.source_parcel_id.clone()),
                "expected source mass aggregate arithmetic",
            )
        })?;
        let ofe_mass = expected_ofe_mass
            .entry(key.basis_ofe_id.clone())
            .or_default();
        *ofe_mass = checked_surface_liquid_add(*ofe_mass, amount.mass).ok_or_else(|| {
            contextual_comparison_failure(
                DirectSurfaceLiquidErrorCode::E003,
                operands.transaction_id,
                &configuration.owner_id,
                &key.recipient_store_key,
                Some(key.source_parcel_id.clone()),
                "expected OFE mass aggregate arithmetic",
            )
        })?;
        let accumulated = expected_ofe_enthalpy
            .entry(key.basis_ofe_id.clone())
            .or_default();
        *accumulated =
            checked_surface_liquid_add(*accumulated, amount.enthalpy).ok_or_else(|| {
                contextual_comparison_failure(
                    DirectSurfaceLiquidErrorCode::E003,
                    operands.transaction_id,
                    &configuration.owner_id,
                    &key.recipient_store_key,
                    Some(key.source_parcel_id.clone()),
                    "expected OFE enthalpy aggregate arithmetic",
                )
            })?;
    }
    let mut actual_source_mass = BTreeMap::<(OfeId, String), f64>::new();
    let mut actual_ofe_mass = BTreeMap::<OfeId, f64>::new();
    let mut actual_ofe_enthalpy = BTreeMap::<OfeId, f64>::new();
    for (key, amount) in &actual {
        let source_mass = actual_source_mass
            .entry((key.basis_ofe_id.clone(), key.source_parcel_id.clone()))
            .or_default();
        *source_mass = checked_surface_liquid_add(*source_mass, amount.mass).ok_or_else(|| {
            contextual_comparison_failure(
                DirectSurfaceLiquidErrorCode::E003,
                operands.transaction_id,
                &configuration.owner_id,
                &key.recipient_store_key,
                Some(key.source_parcel_id.clone()),
                "actual source mass aggregate arithmetic",
            )
        })?;
        let ofe_mass = actual_ofe_mass.entry(key.basis_ofe_id.clone()).or_default();
        *ofe_mass = checked_surface_liquid_add(*ofe_mass, amount.mass).ok_or_else(|| {
            contextual_comparison_failure(
                DirectSurfaceLiquidErrorCode::E003,
                operands.transaction_id,
                &configuration.owner_id,
                &key.recipient_store_key,
                Some(key.source_parcel_id.clone()),
                "actual OFE mass aggregate arithmetic",
            )
        })?;
        let accumulated = actual_ofe_enthalpy
            .entry(key.basis_ofe_id.clone())
            .or_default();
        *accumulated =
            checked_surface_liquid_add(*accumulated, amount.enthalpy).ok_or_else(|| {
                contextual_comparison_failure(
                    DirectSurfaceLiquidErrorCode::E003,
                    operands.transaction_id,
                    &configuration.owner_id,
                    &key.recipient_store_key,
                    Some(key.source_parcel_id.clone()),
                    "actual OFE enthalpy aggregate arithmetic",
                )
            })?;
    }
    Ok(ParcelArithmeticProjection {
        expected,
        expected_temperature_k,
        actual,
        expected_source_mass,
        actual_source_mass,
        raw_source_mass,
        expected_ofe_mass,
        actual_ofe_mass,
        raw_ofe_mass,
        expected_ofe_enthalpy,
        actual_ofe_enthalpy,
        raw_ofe_enthalpy: replayed_ofe_enthalpy,
        expected_store_liquid: store_liquid,
        expected_continuations,
    })
}
