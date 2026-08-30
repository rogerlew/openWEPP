#[cfg(test)]
thread_local! {
    static PARENT_END_RECEIVER_TRANSACTION_AUDIT: std::cell::RefCell<Option<(Option<openwepp_kernel_contract::TransactionId>, openwepp_kernel_contract::TransactionId)>> =
        const { std::cell::RefCell::new(None) };
}

#[cfg(test)]
pub(crate) fn begin_parent_end_receiver_transaction_audit() {
    PARENT_END_RECEIVER_TRANSACTION_AUDIT.with(|audit| *audit.borrow_mut() = None);
}

#[cfg(test)]
pub(crate) fn take_parent_end_receiver_transaction_audit() -> Option<(
    Option<openwepp_kernel_contract::TransactionId>,
    openwepp_kernel_contract::TransactionId,
)> {
    PARENT_END_RECEIVER_TRANSACTION_AUDIT.with(|audit| audit.borrow_mut().take())
}

#[cfg(test)]
fn audit_parent_end_receiver_transaction(
    beginning: Option<openwepp_kernel_contract::TransactionId>,
    ending: openwepp_kernel_contract::TransactionId,
) {
    PARENT_END_RECEIVER_TRANSACTION_AUDIT.with(|audit| {
        *audit.borrow_mut() = Some((beginning, ending));
    });
}

#[cfg(not(test))]
#[inline(always)]
fn audit_parent_end_receiver_transaction(
    _: Option<openwepp_kernel_contract::TransactionId>,
    _: openwepp_kernel_contract::TransactionId,
) {
}

fn terminal_destination_output_custody_v1<'a>(
    destinations: impl IntoIterator<Item = (&'a str, &'a str, f64)>,
    mass_kg_m2_tile_ground: f64,
    specific_liquid_enthalpy_j_kg: f64,
) -> Result<(f64, f64), DirectSnowStage3V11AttachmentError> {
    let common_enthalpy = crate::direct_runtime::checked_surface_liquid_mul(
        mass_kg_m2_tile_ground,
        specific_liquid_enthalpy_j_kg,
    )
    .ok_or_else(|| support_liquid_transition_error("terminal parcel enthalpy"))?;
    let mut destinations = destinations.into_iter().collect::<Vec<_>>();
    destinations.sort_by(|left, right| (left.0, left.1).cmp(&(right.0, right.1)));
    let mut mass = 0.0;
    let mut enthalpy = 0.0;
    for (_, _, destination_fraction) in destinations {
        mass = crate::direct_runtime::checked_surface_liquid_add(
            mass,
            crate::direct_runtime::checked_surface_liquid_mul(
                destination_fraction,
                mass_kg_m2_tile_ground,
            )
            .ok_or_else(|| support_liquid_transition_error("terminal parcel destination mass"))?,
        )
        .ok_or_else(|| support_liquid_transition_error("terminal parcel mass sum"))?;
        enthalpy = crate::direct_runtime::checked_surface_liquid_add(
            enthalpy,
            crate::direct_runtime::checked_surface_liquid_mul(
                destination_fraction,
                common_enthalpy,
            )
            .ok_or_else(|| {
                support_liquid_transition_error("terminal parcel destination enthalpy")
            })?,
        )
        .ok_or_else(|| support_liquid_transition_error("terminal parcel enthalpy sum"))?;
    }
    if mass <= 0.0 || enthalpy < 0.0 {
        return Err(support_liquid_transition_error(
            "terminal parcel output custody domain",
        ));
    }
    Ok((mass, enthalpy))
}

fn terminal_parcel_output_custody_v1(
    parcel: &DirectSnowStage3V11TerminalParcel,
) -> Result<(f64, f64), DirectSnowStage3V11AttachmentError> {
    terminal_destination_output_custody_v1(
        parcel.receiver_destinations.iter().map(|destination| {
            (
                destination.destination_ofe_id.as_str(),
                destination.destination_tile_id.as_str(),
                destination.destination_fraction,
            )
        }),
        parcel.mass_kg_m2_tile_ground,
        parcel.specific_liquid_enthalpy_j_kg,
    )
}

#[allow(clippy::too_many_arguments)]
fn consume_parent_end_terminal_parcels_v1(
    context: &DirectSnowStage3V11StaticContext,
    parent: &mut V11ParentTransaction,
    consumer: &mut DirectV10RealConsumerShadow,
    clock: &mut CoupledClockStateV1,
    stage3: &BTreeMap<u32, DirectSnowStage3PersistentState>,
    terminal_parcels: &mut [DirectSnowStage3V11TerminalParcel],
    pending: &mut BTreeMap<Digest32, DirectSnowStage3V11TerminalParcel>,
    endpoint: &Stage3CoupledSubslabReceiptV1,
    terminal_group: &mut Stage3V11TerminalEventGroupV1,
) -> Result<AcceptedEventReceiptV1, DirectSnowStage3V11AttachmentError> {
    let predecessor_event = terminal_group
        .accepted_event_receipt
        .as_ref()
        .ok_or(DirectSnowStage3V11AttachmentError::Identity(
            "parent-end terminal receiver accepted predecessor",
        ))?
        .clone();
    if clock.accepted_until() != clock.parent_support().end_ns()
        || predecessor_event.tick() != clock.accepted_until()
        || predecessor_event.ending_owner_set_digest() != complete_owner_set_digest(clock.owners())?
        || pending.is_empty()
        || terminal_group.terminal_receiver_custody_v2().is_some()
    {
        return Err(DirectSnowStage3V11AttachmentError::Identity(
            "parent-end terminal receiver predecessor",
        ));
    }
    let produced = pending.values().cloned().collect::<Vec<_>>();
    let mut parcel_fields = Vec::new();
    let mut expected_output_custody = BTreeMap::new();
    for parcel in &produced {
        parcel_fields.push(FramedField {
            tag: "parcel",
            value: parcel.parcel_digest.as_bytes(),
        });
        if expected_output_custody
            .insert(
                *parcel.parcel_digest.as_bytes(),
                terminal_parcel_output_custody_v1(parcel)?,
            )
            .is_some()
        {
            return Err(support_liquid_transition_error(
                "terminal parcel output custody duplicate",
            ));
        }
    }
    let mut mass_kg_m2 = 0.0_f64;
    let mut enthalpy_j_m2 = 0.0_f64;
    for (mass, enthalpy) in expected_output_custody.values() {
        mass_kg_m2 = crate::direct_runtime::checked_surface_liquid_add(mass_kg_m2, *mass)
            .ok_or_else(|| support_liquid_transition_error("terminal output mass sum"))?;
        enthalpy_j_m2 =
            crate::direct_runtime::checked_surface_liquid_add(enthalpy_j_m2, *enthalpy)
                .ok_or_else(|| support_liquid_transition_error("terminal output enthalpy sum"))?;
    }
    if !mass_kg_m2.is_finite()
        || mass_kg_m2 <= 0.0
        || !enthalpy_j_m2.is_finite()
        || enthalpy_j_m2 < 0.0
    {
        return Err(DirectSnowStage3V11AttachmentError::Terminal(
            "parent-end terminal receiver custody domain",
        ));
    }
    let parcel_set = framed_sha256("stage3-v11-terminal-receiver-parcel-set", &parcel_fields)?;
    let context_sha256 = framed_sha256(
        "stage3-v11-parent-end-terminal-receiver",
        &[
            FramedField {
                tag: "parent_transaction",
                value: clock.parent_transaction_id().digest().as_bytes(),
            },
            FramedField {
                tag: "tick",
                value: &clock.accepted_until().get().to_be_bytes(),
            },
            FramedField {
                tag: "predecessor_event",
                value: predecessor_event.id().digest().as_bytes(),
            },
            FramedField {
                tag: "parcel_set",
                value: parcel_set.as_bytes(),
            },
            FramedField {
                tag: "mass_kg_m2",
                value: &mass_kg_m2.to_bits().to_be_bytes(),
            },
            FramedField {
                tag: "enthalpy_j_m2",
                value: &enthalpy_j_m2.to_bits().to_be_bytes(),
            },
        ],
    )?;
    let surface_beginning_state = consumer
        .effective_surface_liquid_state_for_zero_duration_receiver()
        .map_err(DirectSnowStage3V11AttachmentError::Owner)?;
    let lse_beginning_state = consumer
        .physical_lse_state_for_zero_duration_receiver()
        .clone();
    let beginning_surface_transaction = consumer
        .hydrology_frame()
        .surface_liquid_shadow
        .as_deref()
        .and_then(|surface| surface.records.first())
        .and_then(|record| record.last_accepted_transaction_id);
    let (candidate_consumer, surface_bytes, lse_bytes, receiver_receipt_set, receiver_receipts) =
        consumer
        .accept_zero_duration_terminal_receiver(
            &produced,
            parcel_set,
            predecessor_event.ending_owner_set_digest(),
            context_sha256,
            clock.event_ordinal(),
        )
        .map_err(DirectSnowStage3V11AttachmentError::Owner)?;
    let surface_ending_state = candidate_consumer
        .effective_surface_liquid_state_for_zero_duration_receiver()
        .map_err(DirectSnowStage3V11AttachmentError::Owner)?;
    let lse_ending_state = candidate_consumer
        .physical_lse_state_for_zero_duration_receiver()
        .clone();
    validate_support_liquid_surface_delta_v1(
        &surface_beginning_state,
        &surface_ending_state,
        &receiver_receipts,
    )?;
    validate_support_liquid_lse_delta_v1(
        &lse_beginning_state,
        &lse_ending_state,
        &receiver_receipts,
    )?;
    let ending_surface_transaction = candidate_consumer
        .hydrology_frame()
        .surface_liquid_shadow
        .as_deref()
        .and_then(|surface| surface.records.first())
        .and_then(|record| record.last_accepted_transaction_id)
        .ok_or(DirectSnowStage3V11AttachmentError::Identity(
            "parent-end terminal receiver surface transaction",
        ))?;
    let expected_surface_transaction = beginning_surface_transaction
        .map_or(0, |transaction| transaction.0)
        .checked_add(1)
        .ok_or(DirectSnowStage3V11AttachmentError::Identity(
            "parent-end terminal receiver surface transaction overflow",
        ))?;
    if ending_surface_transaction.0 != expected_surface_transaction {
        return Err(DirectSnowStage3V11AttachmentError::Identity(
            "parent-end terminal receiver surface transaction successor",
        ));
    }
    let ending_snow_bytes = canonical_stage3_snow_owner_bytes_with_pending_and_receipts(
        stage3,
        &BTreeMap::new(),
        &endpoint.lane_receipts,
        &endpoint.destination_receipts,
    )?;
    let ending_owners = clock
        .owners()
        .iter()
        .map(|owner| match owner.owner_id() {
            "snow" => OwnerState::new("snow".to_owned(), ending_snow_bytes.clone()),
            "surface_liquid" => OwnerState::new("surface_liquid".to_owned(), surface_bytes.clone()),
            "land_surface_energy" => {
                OwnerState::new("land_surface_energy".to_owned(), lse_bytes.clone())
            }
            _ => Ok(owner.clone()),
        })
        .collect::<Result<Vec<_>, _>>()?;
    let receiver_mutation_set = clock
        .owners()
        .iter()
        .zip(&ending_owners)
        .filter_map(|(before, after)| {
            (before != after && before.owner_id() != "snow")
                .then(|| before.owner_id().to_owned())
        })
        .collect::<Vec<_>>();
    validate_support_liquid_receiver_mutation_semantics_v1(
        &receiver_mutation_set,
        &receiver_receipts,
        &expected_output_custody,
        mass_kg_m2,
        enthalpy_j_m2,
    )?;
    let mutation_set = clock
        .owners()
        .iter()
        .zip(&ending_owners)
        .filter_map(|(before, after)| (before != after).then(|| before.owner_id().to_owned()))
        .collect::<Vec<_>>();
    if !mutation_set.iter().any(|owner| owner == "snow") {
        return Err(DirectSnowStage3V11AttachmentError::Identity(
            "parent-end terminal receiver snow mutation",
        ));
    }
    let receiver_receipt_set_sha256 = Digest32::from_bytes(receiver_receipt_set);
    let ledger = LedgerEntryV1::new(
        "terminal-liquid-receiver".to_owned(),
        "kg-m-2-and-j-m-2-ofe-ground".to_owned(),
        parcel_set,
        parcel_set,
        receiver_receipt_set_sha256,
    )?;
    let event = EventProposalV1::new(
        EventClass::OwnershipTransfer,
        "snow".to_owned(),
        context_sha256,
        ending_owners.clone(),
        mutation_set.clone(),
        "snow-free".to_owned(),
        clock.active_participants().to_vec(),
        vec![ledger],
    )?;
    let mut candidate_clock = clock.clone();
    let mut queue = EventQueueV1::new(candidate_clock.accepted_until(), vec![event])?;
    let accepted = queue
        .apply_next(&mut candidate_clock)?
        .ok_or(DirectSnowStage3V11AttachmentError::Terminal(
            "parent-end terminal receiver event application",
        ))?;
    if queue.apply_next(&mut candidate_clock)?.is_some()
        || accepted.beginning_owner_set_digest() != predecessor_event.ending_owner_set_digest()
        || accepted.ordinal()
            != predecessor_event.ordinal().checked_add(1).ok_or(
                DirectSnowStage3V11AttachmentError::Identity(
                    "parent-end terminal receiver event ordinal overflow",
                ),
            )?
    {
        return Err(DirectSnowStage3V11AttachmentError::Identity(
            "parent-end terminal receiver event chain",
        ));
    }
    let mut candidate_parent = parent.clone();
    candidate_parent.accept_zero_duration_owner_transition(
        &context.vegetation_configuration,
        candidate_clock.accepted_until(),
        owner_envelopes_from_states(&ending_owners)?,
        &mutation_set,
    )?;
    let mut candidate_consumer = candidate_consumer;
    candidate_consumer.retain_accepted_publication_zero_duration_event(&accepted)?;
    let custody = Stage3TerminalLiquidCustodyV2::seal(
        terminal_group,
        accepted.clone(),
        parcel_set,
        surface_beginning_state,
        surface_ending_state,
        lse_beginning_state,
        lse_ending_state,
        receiver_receipt_set_sha256,
        receiver_receipts,
    )?;
    let mut candidate_group = terminal_group.clone();
    candidate_group.install_terminal_receiver_custody_v2(custody)?;
    let pending_digests = pending.keys().copied().collect::<BTreeSet<_>>();
    for parcel in terminal_parcels {
        if pending_digests.contains(&parcel.parcel_digest) {
            if parcel.posture != DirectSnowStage3V11TerminalParcelPosture::ProducedUnconsumed {
                return Err(DirectSnowStage3V11AttachmentError::Terminal(
                    "parent-end terminal parcel exact-once posture",
                ));
            }
            parcel.posture = DirectSnowStage3V11TerminalParcelPosture::Consumed;
        }
    }
    pending.clear();
    audit_parent_end_receiver_transaction(
        beginning_surface_transaction,
        ending_surface_transaction,
    );
    *parent = candidate_parent;
    *clock = candidate_clock;
    *consumer = candidate_consumer;
    *terminal_group = candidate_group;
    Ok(accepted)
}

#[allow(clippy::too_many_arguments)]
struct PositiveSupportLiquidReceiverV1 {
    event: AcceptedEventReceiptV1,
    output_set_sha256: Digest32,
    mass_kg_m2_bits: u64,
    enthalpy_j_m2_bits: u64,
    surface_beginning_state: crate::DirectSurfaceLiquidOwnedState,
    surface_ending_state: crate::DirectSurfaceLiquidOwnedState,
    lse_beginning_state: openwepp_land_surface_energy::LandSurfaceEnergyState,
    lse_ending_state: openwepp_land_surface_energy::LandSurfaceEnergyState,
    receiver_receipt_set_sha256: Digest32,
    receiver_receipts: Vec<
        crate::direct_runtime::DirectZeroDurationSnowLiquidReceiptV1,
    >,
}

fn support_liquid_transition_error(
    detail: &'static str,
) -> DirectSnowStage3V11AttachmentError {
    DirectSnowStage3V11AttachmentError::Owner(
        DirectV11RealConsumerError::ZeroDurationSnowLiquid(detail.to_owned()),
    )
}

fn validate_support_liquid_surface_delta_v1(
    beginning: &crate::DirectSurfaceLiquidOwnedState,
    ending: &crate::DirectSurfaceLiquidOwnedState,
    receipts: &[crate::DirectZeroDurationSnowLiquidReceiptV1],
) -> Result<(), DirectSnowStage3V11AttachmentError> {
    crate::direct_runtime::validate_zero_duration_snow_liquid_surface_delta_v1(
        beginning, ending, receipts,
    )
    .map_err(|error| {
        DirectSnowStage3V11AttachmentError::Owner(
            DirectV11RealConsumerError::ZeroDurationSnowLiquid(error.to_string()),
        )
    })
}

fn validate_support_liquid_lse_delta_v1(
    beginning: &openwepp_land_surface_energy::LandSurfaceEnergyState,
    ending: &openwepp_land_surface_energy::LandSurfaceEnergyState,
    receipts: &[crate::DirectZeroDurationSnowLiquidReceiptV1],
) -> Result<(), DirectSnowStage3V11AttachmentError> {
    let mut expected = beginning.clone();
    for receipt in receipts {
        if receipt.disposition
            != crate::DirectZeroDurationSnowLiquidDispositionV1::RetainedSurface
        {
            continue;
        }
        let tile_id = receipt
            .recipient_tile_id
            .as_ref()
            .ok_or_else(|| support_liquid_transition_error("LSE delta retained tile"))?;
        let credit = receipt
            .credited_enthalpy_j_m2_recipient_tile_ground
            .ok_or_else(|| support_liquid_transition_error("LSE delta retained credit"))?;
        let matches = expected
            .tiles
            .iter_mut()
            .filter(|tile| {
                tile.ofe_id == receipt.recipient_ofe_id && &tile.tile_id == tile_id
            })
            .collect::<Vec<_>>();
        if matches.len() != 1 {
            return Err(support_liquid_transition_error("LSE delta topology"));
        }
        let tile = matches
            .into_iter()
            .next()
            .ok_or_else(|| support_liquid_transition_error("LSE delta tile"))?;
        tile.surface_enthalpy_j_m2_tile_ground = crate::direct_runtime::checked_surface_liquid_add(
            tile.surface_enthalpy_j_m2_tile_ground,
            credit,
        )
        .ok_or_else(|| support_liquid_transition_error("LSE delta enthalpy addition"))?;
    }
    expected.state_sha256 = expected
        .canonical_sha256()
        .map_err(|_| support_liquid_transition_error("LSE delta state seal"))?;
    if &expected != ending {
        return Err(support_liquid_transition_error("LSE delta substitution"));
    }
    Ok(())
}

fn validate_support_liquid_receiver_mutation_summary_v1(
    mutation_set: &[String],
    output_mass_kg_m2: f64,
    output_enthalpy_j_m2: f64,
    first_hop_mass_kg_m2: f64,
    first_hop_enthalpy_j_m2: f64,
    retained_mass_kg_m2: f64,
    retained_enthalpy_j_m2: f64,
    runoff_count: usize,
) -> bool {
    let exact_first_hop = first_hop_mass_kg_m2.is_finite()
        && first_hop_enthalpy_j_m2.is_finite()
        && first_hop_mass_kg_m2.to_bits() == output_mass_kg_m2.to_bits()
        && first_hop_enthalpy_j_m2.to_bits() == output_enthalpy_j_m2.to_bits();
    let exact_semantics = if mutation_set.is_empty() {
        retained_mass_kg_m2.to_bits() == 0.0_f64.to_bits()
            && retained_enthalpy_j_m2.to_bits() == 0.0_f64.to_bits()
            && runoff_count > 0
    } else if mutation_set == ["surface_liquid".to_owned()] {
        retained_enthalpy_j_m2.to_bits() == 0.0_f64.to_bits()
            && (retained_mass_kg_m2 > 0.0
                || (retained_mass_kg_m2.to_bits() == 0.0_f64.to_bits()
                    && runoff_count > 0))
    } else if mutation_set
        == [
            "land_surface_energy".to_owned(),
            "surface_liquid".to_owned(),
        ]
    {
        retained_mass_kg_m2 > 0.0 && retained_enthalpy_j_m2 > 0.0
    } else {
        false
    };
    exact_first_hop && exact_semantics
}

fn reconstruct_first_hop_output_custody_v1(
    receipts: &[crate::DirectZeroDurationSnowLiquidReceiptV1],
) -> Result<BTreeMap<[u8; 32], (f64, f64)>, DirectSnowStage3V11AttachmentError> {
    let mut first_hop_by_tile = BTreeMap::new();
    for receipt in receipts
        .iter()
        .filter(|receipt| receipt.basis_ofe_id == receipt.origin_ofe_id)
    {
        let tile = first_hop_by_tile
            .entry((
                receipt.output_receipt_sha256,
                receipt.origin_ofe_id.clone(),
                receipt.origin_tile_id.clone(),
            ))
            .or_insert((0.0, 0.0));
        tile.0 = crate::direct_runtime::checked_surface_liquid_add(
            tile.0,
            receipt.mass_kg_m2_basis_ofe_ground,
        )
        .ok_or_else(|| support_liquid_transition_error("first-hop tile mass sum"))?;
        tile.1 = crate::direct_runtime::checked_surface_liquid_add(
            tile.1,
            receipt.sensible_enthalpy_j_m2_basis_ofe_ground,
        )
        .ok_or_else(|| support_liquid_transition_error("first-hop tile enthalpy sum"))?;
    }
    let mut first_hop_by_output = BTreeMap::<[u8; 32], (f64, f64)>::new();
    for ((output_receipt, _, _), (mass, enthalpy)) in first_hop_by_tile {
        let output = first_hop_by_output
            .entry(output_receipt)
            .or_insert((0.0, 0.0));
        output.0 = crate::direct_runtime::checked_surface_liquid_add(output.0, mass)
            .ok_or_else(|| support_liquid_transition_error("first-hop output mass sum"))?;
        output.1 = crate::direct_runtime::checked_surface_liquid_add(output.1, enthalpy)
            .ok_or_else(|| support_liquid_transition_error("first-hop output enthalpy sum"))?;
    }
    Ok(first_hop_by_output)
}

fn exact_first_hop_output_custody_matches_v1(
    actual: &BTreeMap<[u8; 32], (f64, f64)>,
    expected: &BTreeMap<[u8; 32], (f64, f64)>,
) -> bool {
    actual.len() == expected.len()
        && actual.iter().all(|(receipt, actual)| {
            expected.get(receipt).is_some_and(|expected| {
                actual.0.to_bits() == expected.0.to_bits()
                    && actual.1.to_bits() == expected.1.to_bits()
            })
        })
}

fn validate_support_liquid_receiver_mutation_semantics_v1(
    mutation_set: &[String],
    receipts: &[crate::DirectZeroDurationSnowLiquidReceiptV1],
    expected_output_custody: &BTreeMap<[u8; 32], (f64, f64)>,
    output_mass_kg_m2: f64,
    output_enthalpy_j_m2: f64,
) -> Result<(), DirectSnowStage3V11AttachmentError> {
    if receipts.is_empty()
        || expected_output_custody.is_empty()
        || expected_output_custody.iter().any(|(receipt, (mass, enthalpy))| {
            *receipt == [0; 32]
                || !mass.is_finite()
                || *mass <= 0.0
                || !enthalpy.is_finite()
                || *enthalpy < 0.0
        })
        || output_mass_kg_m2 <= 0.0
        || !output_enthalpy_j_m2.is_finite()
    {
        return Err(DirectSnowStage3V11AttachmentError::Owner(
            DirectV11RealConsumerError::ZeroDurationSnowLiquid(
                "empty/domain receiver custody".to_owned(),
            ),
        ));
    }
    let mut first_hop_mass = 0.0_f64;
    let mut first_hop_enthalpy = 0.0_f64;
    let mut retained_mass = 0.0_f64;
    let mut retained_enthalpy = 0.0_f64;
    let mut runoff_count = 0_usize;
    for receipt in receipts {
        receipt.validate_seal().map_err(|error| {
            DirectSnowStage3V11AttachmentError::Owner(
                DirectV11RealConsumerError::ZeroDurationSnowLiquid(error.to_string()),
            )
        })?;
        match receipt.disposition {
            crate::DirectZeroDurationSnowLiquidDispositionV1::RetainedSurface => {
                retained_mass += receipt.mass_kg_m2_basis_ofe_ground;
                retained_enthalpy += receipt.sensible_enthalpy_j_m2_basis_ofe_ground;
            }
            crate::DirectZeroDurationSnowLiquidDispositionV1::RoutedRunoff
            | crate::DirectZeroDurationSnowLiquidDispositionV1::OutletRunoff => {
                runoff_count += 1;
            }
        }
    }
    let first_hop_by_output = reconstruct_first_hop_output_custody_v1(receipts)?;
    if !exact_first_hop_output_custody_matches_v1(
        &first_hop_by_output,
        expected_output_custody,
    ) {
        return Err(support_liquid_transition_error(
            "exact first-hop output mass/enthalpy custody",
        ));
    }
    for (mass, enthalpy) in first_hop_by_output.values() {
        first_hop_mass = crate::direct_runtime::checked_surface_liquid_add(first_hop_mass, *mass)
            .ok_or_else(|| support_liquid_transition_error("first-hop mass sum"))?;
        first_hop_enthalpy = crate::direct_runtime::checked_surface_liquid_add(
            first_hop_enthalpy,
            *enthalpy,
        )
        .ok_or_else(|| support_liquid_transition_error("first-hop enthalpy sum"))?;
    }
    if !validate_support_liquid_receiver_mutation_summary_v1(
        mutation_set,
        output_mass_kg_m2,
        output_enthalpy_j_m2,
        first_hop_mass,
        first_hop_enthalpy,
        retained_mass,
        retained_enthalpy,
        runoff_count,
    ) {
        return Err(DirectSnowStage3V11AttachmentError::Owner(
            DirectV11RealConsumerError::ZeroDurationSnowLiquid(format!(
                "exact owner mutation/custody semantics: {mutation_set:?}",
            )),
        ));
    }
    Ok(())
}

#[cfg(test)]
mod support_liquid_receiver_mutation_tests {
    use super::*;

    fn split_receipt(
        tile: &str,
        disposition: crate::DirectZeroDurationSnowLiquidDispositionV1,
        mass: f64,
        enthalpy: f64,
    ) -> crate::DirectZeroDurationSnowLiquidReceiptV1 {
        let ofe_id = OfeId::try_new("ofe-1").expect("OFE");
        let tile_id = TileId::try_new(tile).expect("tile");
        crate::DirectZeroDurationSnowLiquidReceiptV1 {
            output_receipt_sha256: [7; 32],
            output_set_sha256: [8; 32],
            predecessor_owner_set_sha256: [9; 32],
            receiver_context_sha256: [10; 32],
            support_start_ns: 1,
            support_end_ns: 2,
            receiver_ordinal: 3,
            transaction_id: openwepp_kernel_contract::TransactionId(4),
            origin_ofe_id: ofe_id.clone(),
            origin_tile_id: tile_id.clone(),
            basis_ofe_id: ofe_id.clone(),
            recipient_ofe_id: ofe_id,
            recipient_tile_id: matches!(
                disposition,
                crate::DirectZeroDurationSnowLiquidDispositionV1::RetainedSurface
            )
            .then_some(tile_id),
            recipient_tile_fraction: matches!(
                disposition,
                crate::DirectZeroDurationSnowLiquidDispositionV1::RetainedSurface
            )
            .then_some(0.5),
            disposition,
            mass_kg_m2_basis_ofe_ground: mass,
            sensible_enthalpy_j_m2_basis_ofe_ground: enthalpy,
            credited_mass_kg_m2_recipient_tile_ground: None,
            credited_enthalpy_j_m2_recipient_tile_ground: None,
            predecessor_receipt_sha256: [0; 32],
            receipt_sha256: [11; 32],
        }
    }

    #[test]
    fn per_output_first_hop_reconstruction_is_exact_and_rejects_split_receipt_poisons() {
        use crate::DirectZeroDurationSnowLiquidDispositionV1::{
            OutletRunoff, RetainedSurface,
        };

        let receipts = vec![
            split_receipt("a", RetainedSurface, 0.4, 3922.228364712222),
            split_receipt("a", OutletRunoff, 0.6, 770.7509739995221),
            split_receipt("b", RetainedSurface, 0.3, 613.3927148468006),
            split_receipt("b", OutletRunoff, 0.7, 803.9811112535149),
        ];
        let actual = reconstruct_first_hop_output_custody_v1(&receipts)
            .expect("canonical first-hop reconstruction");
        let output = actual[&[7; 32]];
        let flat_enthalpy = receipts
            .iter()
            .map(|receipt| receipt.sensible_enthalpy_j_m2_basis_ofe_ground)
            .sum::<f64>();
        assert_ne!(flat_enthalpy.to_bits(), output.1.to_bits());
        let expected = BTreeMap::from([([7; 32], output)]);
        assert!(exact_first_hop_output_custody_matches_v1(
            &actual, &expected
        ));

        let omitted = BTreeMap::new();
        assert!(!exact_first_hop_output_custody_matches_v1(
            &actual, &omitted
        ));
        let mut substituted = expected.clone();
        substituted.get_mut(&[7; 32]).expect("output").1 =
            f64::from_bits(output.1.to_bits() ^ 1);
        assert!(!exact_first_hop_output_custody_matches_v1(
            &actual,
            &substituted
        ));
        let mut wrong_lineage = receipts;
        wrong_lineage[0].output_receipt_sha256 = [12; 32];
        let wrong_lineage = reconstruct_first_hop_output_custody_v1(&wrong_lineage)
            .expect("wrong-lineage reconstruction");
        assert!(!exact_first_hop_output_custody_matches_v1(
            &wrong_lineage,
            &expected
        ));
    }

    #[test]
    fn receipt_bearing_full_runoff_noop_is_exact_and_false_noops_reject() {
        assert!(validate_support_liquid_receiver_mutation_summary_v1(
            &[], 2.0, 20.0, 2.0, 20.0, 0.0, 0.0, 1,
        ));
        assert!(!validate_support_liquid_receiver_mutation_summary_v1(
            &[], 2.0, 20.0, 2.0, 20.0, 0.0, 0.0, 0,
        ));
        assert!(!validate_support_liquid_receiver_mutation_summary_v1(
            &[], 2.0, 20.0, 1.0, 10.0, 0.0, 0.0, 1,
        ));
        assert!(!validate_support_liquid_receiver_mutation_summary_v1(
            &[], 2.0, 20.0, 2.0, 20.0, 0.1, 1.0, 1,
        ));
        assert!(validate_support_liquid_receiver_mutation_summary_v1(
            &["surface_liquid".to_owned()],
            2.0,
            20.0,
            2.0,
            20.0,
            0.0,
            0.0,
            1,
        ));
    }

    #[test]
    fn retained_mass_requires_exact_surface_and_enthalpy_mutations() {
        assert!(validate_support_liquid_receiver_mutation_summary_v1(
            &["surface_liquid".to_owned()],
            2.0,
            0.0,
            2.0,
            0.0,
            2.0,
            0.0,
            0,
        ));
        assert!(validate_support_liquid_receiver_mutation_summary_v1(
            &[
                "land_surface_energy".to_owned(),
                "surface_liquid".to_owned(),
            ],
            2.0,
            20.0,
            2.0,
            20.0,
            2.0,
            20.0,
            0,
        ));
        assert!(!validate_support_liquid_receiver_mutation_summary_v1(
            &["surface_liquid".to_owned()],
            2.0,
            20.0,
            2.0,
            20.0,
            2.0,
            20.0,
            0,
        ));
    }
}

#[allow(clippy::too_many_arguments)]
fn consume_positive_support_snow_liquid_v1(
    context: &DirectSnowStage3V11StaticContext,
    parent: &mut V11ParentTransaction,
    consumer: &mut DirectV10RealConsumerShadow,
    clock: &mut CoupledClockStateV1,
    support: TimeSupport,
    support_ending_owner: Digest32,
    terminal_lanes: &BTreeSet<u32>,
) -> Result<Option<PositiveSupportLiquidReceiverV1>, DirectSnowStage3V11AttachmentError> {
    if clock.accepted_until() != support.end_ns()
        || complete_owner_set_digest(clock.owners())? != support_ending_owner
    {
        return Err(DirectSnowStage3V11AttachmentError::Identity(
            "positive-support liquid receiver predecessor",
        ));
    }
    let outputs = consumer
        .accepted_snow_liquid_outputs_for_support(support)
        .map_err(DirectSnowStage3V11AttachmentError::Owner)?
        .into_iter()
        .filter(|output| !terminal_lanes.contains(&output.lane_id()))
        .collect::<Vec<_>>();
    if outputs.is_empty() {
        return Ok(None);
    }
    let surface_beginning_state = consumer
        .effective_surface_liquid_state_for_zero_duration_receiver()
        .map_err(DirectSnowStage3V11AttachmentError::Owner)?;
    let lse_beginning_state = consumer
        .physical_lse_state_for_zero_duration_receiver()
        .clone();
    let surface_beginning_bytes = surface_beginning_state
        .canonical_bytes(&context.surface_liquid_configuration)
        .map_err(|_| support_liquid_transition_error("surface predecessor bytes"))?;
    let lse_beginning_bytes = serde_json::to_vec(&lse_beginning_state)
        .map_err(|_| support_liquid_transition_error("LSE predecessor bytes"))?;
    if clock
        .owners()
        .iter()
        .find(|owner| owner.owner_id() == "surface_liquid")
        .is_none_or(|owner| owner.state_bytes() != surface_beginning_bytes)
        || clock
            .owners()
            .iter()
            .find(|owner| owner.owner_id() == "land_surface_energy")
            .is_none_or(|owner| owner.state_bytes() != lse_beginning_bytes)
    {
        return Err(support_liquid_transition_error(
            "receiver predecessor owner bytes",
        ));
    }
    let output_receipts = outputs
        .iter()
        .map(Stage3AcceptedSnowLiquidOutputV1::receipt_sha256)
        .collect::<Vec<_>>();
    let receipt_fields = output_receipts
        .iter()
        .map(|receipt| FramedField {
            tag: "snow_liquid_output",
            value: receipt.as_bytes(),
        })
        .collect::<Vec<_>>();
    let mut expected_output_custody = BTreeMap::new();
    for output in &outputs {
        if expected_output_custody
            .insert(
                *output.receipt_sha256().as_bytes(),
                (
                    output.mass_kg_m2_ofe_ground(),
                    output.sensible_enthalpy_j_m2_ofe_ground(),
                ),
            )
            .is_some()
        {
            return Err(support_liquid_transition_error(
                "positive-support output custody duplicate",
            ));
        }
    }
    let mut mass_kg_m2 = 0.0_f64;
    let mut enthalpy_j_m2 = 0.0_f64;
    for (mass, enthalpy) in expected_output_custody.values() {
        mass_kg_m2 = crate::direct_runtime::checked_surface_liquid_add(mass_kg_m2, *mass)
            .ok_or_else(|| support_liquid_transition_error("positive-support output mass sum"))?;
        enthalpy_j_m2 =
            crate::direct_runtime::checked_surface_liquid_add(enthalpy_j_m2, *enthalpy)
                .ok_or_else(|| {
                    support_liquid_transition_error("positive-support output enthalpy sum")
                })?;
    }
    if !mass_kg_m2.is_finite()
        || mass_kg_m2 <= 0.0
        || !enthalpy_j_m2.is_finite()
    {
        return Err(DirectSnowStage3V11AttachmentError::Terminal(
            "positive-support liquid receiver custody domain",
        ));
    }
    let output_set = framed_sha256(
        "stage3-v11-positive-support-liquid-output-set",
        &receipt_fields,
    )?;
    let context_sha256 = framed_sha256(
        "stage3-v11-positive-support-liquid-receiver",
        &[
            FramedField {
                tag: "parent_transaction",
                value: clock.parent_transaction_id().digest().as_bytes(),
            },
            FramedField {
                tag: "support_start",
                value: &support.start_ns().get().to_be_bytes(),
            },
            FramedField {
                tag: "support_end",
                value: &support.end_ns().get().to_be_bytes(),
            },
            FramedField {
                tag: "support_ending_owner",
                value: support_ending_owner.as_bytes(),
            },
            FramedField {
                tag: "output_set",
                value: output_set.as_bytes(),
            },
            FramedField {
                tag: "mass_kg_m2",
                value: &mass_kg_m2.to_bits().to_be_bytes(),
            },
            FramedField {
                tag: "enthalpy_j_m2",
                value: &enthalpy_j_m2.to_bits().to_be_bytes(),
            },
        ],
    )?;
    let receiver_ordinal = clock.event_ordinal();
    let (
        candidate_consumer,
        surface_bytes,
        lse_bytes,
        receiver_receipt_set,
        receiver_receipts,
    ) = consumer
        .accept_zero_duration_stage3_support_liquid_receiver(
            &outputs,
            output_set,
            support_ending_owner,
            context_sha256,
            receiver_ordinal,
        )
        .map_err(DirectSnowStage3V11AttachmentError::Owner)?;
    let surface_ending_state = candidate_consumer
        .effective_surface_liquid_state_for_zero_duration_receiver()
        .map_err(DirectSnowStage3V11AttachmentError::Owner)?;
    let lse_ending_state = candidate_consumer
        .physical_lse_state_for_zero_duration_receiver()
        .clone();
    validate_support_liquid_surface_delta_v1(
        &surface_beginning_state,
        &surface_ending_state,
        &receiver_receipts,
    )?;
    validate_support_liquid_lse_delta_v1(
        &lse_beginning_state,
        &lse_ending_state,
        &receiver_receipts,
    )?;
    let ending_owners = clock
        .owners()
        .iter()
        .map(|owner| match owner.owner_id() {
            "surface_liquid" => {
                OwnerState::new("surface_liquid".to_owned(), surface_bytes.clone())
            }
            "land_surface_energy" => {
                OwnerState::new("land_surface_energy".to_owned(), lse_bytes.clone())
            }
            _ => Ok(owner.clone()),
        })
        .collect::<Result<Vec<_>, _>>()?;
    let receiver_mutation_set = clock
        .owners()
        .iter()
        .zip(&ending_owners)
        .filter_map(|(before, after)| {
            (before != after).then(|| before.owner_id().to_owned())
        })
        .collect::<Vec<_>>();
    validate_support_liquid_receiver_mutation_semantics_v1(
        &receiver_mutation_set,
        &receiver_receipts,
        &expected_output_custody,
        mass_kg_m2,
        enthalpy_j_m2,
    )?;
    let receiver_receipt_set_sha256 = Digest32::from_bytes(receiver_receipt_set);
    let ledger = LedgerEntryV1::new(
        "positive-support-snow-liquid-receiver".to_owned(),
        "kg-m-2-and-j-m-2-ofe-ground".to_owned(),
        output_set,
        output_set,
        receiver_receipt_set_sha256,
    )?;
    let event = EventProposalV1::new(
        EventClass::OwnershipTransfer,
        "snow".to_owned(),
        context_sha256,
        ending_owners.clone(),
        receiver_mutation_set.clone(),
        "snow-stage3-v11".to_owned(),
        clock.active_participants().to_vec(),
        vec![ledger],
    )?;
    let mut queue = EventQueueV1::new(support.end_ns(), vec![event])?;
    let accepted = queue
        .apply_next(clock)?
        .ok_or(DirectSnowStage3V11AttachmentError::Terminal(
            "positive-support liquid receiver event application",
        ))?;
    if queue.apply_next(clock)?.is_some()
        || accepted.beginning_owner_set_digest() != support_ending_owner
    {
        return Err(DirectSnowStage3V11AttachmentError::Identity(
            "positive-support liquid receiver event chain",
        ));
    }
    let ending_owner_envelopes = owner_envelopes_from_states(&ending_owners)?;
    if receiver_mutation_set.is_empty() {
        parent.accept_zero_duration_custody_noop(
            &context.vegetation_configuration,
            support.end_ns(),
            ending_owner_envelopes,
            receiver_receipt_set_sha256,
        )?;
    } else {
        parent.accept_zero_duration_owner_transition(
            &context.vegetation_configuration,
            support.end_ns(),
            ending_owner_envelopes,
            &receiver_mutation_set,
        )?;
    }
    let mut candidate_consumer = candidate_consumer;
    candidate_consumer.retain_accepted_publication_zero_duration_event(&accepted)?;
    *consumer = candidate_consumer;
    Ok(Some(PositiveSupportLiquidReceiverV1 {
        event: accepted,
        output_set_sha256: output_set,
        mass_kg_m2_bits: mass_kg_m2.to_bits(),
        enthalpy_j_m2_bits: enthalpy_j_m2.to_bits(),
        surface_beginning_state,
        surface_ending_state,
        lse_beginning_state,
        lse_ending_state,
        receiver_receipt_set_sha256,
        receiver_receipts,
    }))
}
