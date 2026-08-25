fn owner_states_from_envelopes(
    owners: &BTreeMap<String, V11OwnerEnvelope>,
) -> Result<Vec<OwnerState>, DirectSnowStage3V11AttachmentError> {
    let values = owners
        .values()
        .map(V11OwnerEnvelope::to_owner_state)
        .collect::<Result<Vec<_>, _>>()?;
    Ok(values)
}
/// The Stage-3 persistent state is the sole authoritative snow owner. The
/// hydrology winter-column fields remain a checked compatibility projection;
/// they are intentionally absent from this canonical owner envelope.
fn canonical_stage3_snow_owner_bytes(
    states: &BTreeMap<u32, DirectSnowStage3PersistentState>,
) -> Result<Vec<u8>, DirectSnowStage3V11AttachmentError> {
    #[derive(Serialize)]
    struct CanonicalSnowOwner<'a> {
        schema: &'static str,
        lanes: Vec<(&'a u32, &'a DirectSnowStage3PersistentState)>,
    }
    serde_json::to_vec(&CanonicalSnowOwner {
        schema: "OPENWEPP_STAGE3_CANONICAL_SNOW_OWNER_V1",
        lanes: states.iter().collect(),
    })
    .map_err(|_| DirectSnowStage3V11AttachmentError::Identity("canonical Stage-3 snow bytes"))
}

fn canonical_stage3_snow_owner_bytes_with_pending(
    states: &BTreeMap<u32, DirectSnowStage3PersistentState>,
    pending_terminal_parcels: &BTreeMap<Digest32, DirectSnowStage3V11TerminalParcel>,
) -> Result<Vec<u8>, DirectSnowStage3V11AttachmentError> {
    if pending_terminal_parcels.is_empty() {
        return canonical_stage3_snow_owner_bytes(states);
    }
    crate::snow_owner_v4::canonical_stage3_snow_owner_v4_bytes(
        states,
        pending_terminal_parcels,
        &BTreeMap::new(),
        &BTreeMap::new(),
    )
    .map_err(|_| DirectSnowStage3V11AttachmentError::Identity("canonical Stage-3 snow bytes"))
}

fn parse_lower_hex_digest(value: &str) -> Result<Digest32, DirectSnowStage3V11AttachmentError> {
    if value.len() != 64
        || value
            .bytes()
            .any(|byte| !(byte.is_ascii_digit() || (b'a'..=b'f').contains(&byte)))
    {
        return Err(DirectSnowStage3V11AttachmentError::Support(
            "provider receipt digest encoding",
        ));
    }
    let mut bytes = [0_u8; 32];
    for (index, chunk) in value.as_bytes().chunks_exact(2).enumerate() {
        let text = std::str::from_utf8(chunk).map_err(|_| {
            DirectSnowStage3V11AttachmentError::Support("provider receipt digest encoding")
        })?;
        bytes[index] = u8::from_str_radix(text, 16).map_err(|_| {
            DirectSnowStage3V11AttachmentError::Support("provider receipt digest digits")
        })?;
    }
    Ok(Digest32::from_bytes(bytes))
}

fn validate_lane_destination_set(
    bound_ofe_id: &str,
    identities: &[PreparedStage3V11SupportIdentityV1],
    expected: &BTreeSet<(String, String)>,
) -> Result<(), DirectSnowStage3V11AttachmentError> {
    let actual = identities
        .iter()
        .map(|identity| {
            (
                identity.destination_ofe_id.clone(),
                identity.destination_tile_id.clone(),
            )
        })
        .collect::<BTreeSet<_>>();
    if identities
        .iter()
        .any(|identity| identity.destination_ofe_id != bound_ofe_id)
        || &actual != expected
    {
        return Err(DirectSnowStage3V11AttachmentError::Support(
            "support lane/OFE destination binding",
        ));
    }
    Ok(())
}

fn validate_parent_support_duration(
    duration_ns: u128,
) -> Result<(), DirectSnowStage3V11AttachmentError> {
    if duration_ns != STAGE3_V11_PARENT_SUPPORT_NS {
        return Err(DirectSnowStage3V11AttachmentError::Support(
            "support duration is not 1,800 seconds",
        ));
    }
    Ok(())
}

fn day_start_ns(day_index: usize) -> Result<u128, DirectSnowStage3V11AttachmentError> {
    u128::try_from(day_index)
        .map_err(|_| DirectSnowStage3V11AttachmentError::Support("day index width"))?
        .checked_mul(STAGE3_V11_DAY_NS)
        .ok_or(DirectSnowStage3V11AttachmentError::Support(
            "run-relative day start overflow",
        ))
}

fn owner_envelopes_from_states(
    owners: &[OwnerState],
) -> Result<BTreeMap<String, V11OwnerEnvelope>, DirectSnowStage3V11AttachmentError> {
    owners
        .iter()
        .map(|owner| {
            Ok((
                owner.owner_id().to_owned(),
                V11OwnerEnvelope::try_new(
                    owner.owner_id().to_owned(),
                    owner.state_bytes().to_vec(),
                )?,
            ))
        })
        .collect()
}

#[allow(
    clippy::cast_precision_loss,
    clippy::large_types_passed_by_value,
    clippy::too_many_arguments
)]
fn select_actual_terminal_candidate(
    inputs: &DirectActiveSnowPartitionInputs,
    state: &DirectSnowStage3PersistentState,
    lane_id: u32,
    interval_index: u64,
    support: &DirectSnowStage3V11PreparedSupport,
    _support_forcing: DirectSnowStage3SupportInput,
    full_result: DirectSnowTerminalEventResult,
    minimum_support_ns: u128,
) -> Result<
    (
        DirectSnowStage3V11TerminalReceipt,
        DirectSnowStage3PersistentDayResult,
    ),
    DirectSnowStage3V11AttachmentError,
> {
    let start = support.support.start_ns();
    let end = support.support.end_ns();
    let duration_s = support.support.duration_ns() as f64 / 1.0e9;
    let mut relative_seconds = vec![
        0.0,
        duration_s,
        full_result.hour_offset_seconds,
        full_result.event_bracket_lower_seconds,
        full_result.event_bracket_upper_seconds,
    ];
    relative_seconds
        .retain(|seconds| seconds.is_finite() && *seconds >= 0.0 && *seconds <= duration_s);
    let mut candidate_ticks = relative_seconds
        .into_iter()
        .map(|seconds| {
            quantize_seconds_to_tick(
                ModelTimeNs::new(0),
                ModelTimeNs::new(support.support.duration_ns()),
                seconds,
            )
        })
        .collect::<Result<Vec<_>, _>>()?;
    candidate_ticks.sort_unstable();
    candidate_ticks.dedup();
    let candidate_ticks = candidate_ticks
        .into_iter()
        .map(|tick| ModelTimeNs::new(start.get() + tick.get()))
        .collect::<Vec<_>>();
    let mut accepted = Vec::new();
    for tick in &candidate_ticks {
        let pre = tick.get() - start.get();
        let post = end.get() - tick.get();
        if pre != 0 && pre < minimum_support_ns || post != 0 && post < minimum_support_ns {
            continue;
        }
        if *tick == start {
            continue;
        }
        let trial_support = TimeSupport::new(start, *tick)?;
        let projected = support.coupled_subslab(trial_support, 0)?;
        let projected_forcing = projected
            .support_forcing_by_lane
            .get(&lane_id)
            .copied()
            .ok_or(DirectSnowStage3V11AttachmentError::Support(
                "terminal projected lane forcing",
            ))?;
        let trial = Wb11HydrologyKernel::evaluate_stage3_persistent_support(
            inputs,
            state,
            lane_id,
            interval_index,
            projected_forcing,
            DirectSnowTerminalEventRequest::ENTHALPY_EVENT_V1,
        )?;
        let Some(result) = trial.terminal_event else {
            continue;
        };
        let actual_offset = quantize_seconds_to_tick(
            ModelTimeNs::new(0),
            ModelTimeNs::new(support.support.duration_ns()),
            result.hour_offset_seconds,
        )?;
        let actual_tick = ModelTimeNs::new(start.get() + actual_offset.get());
        if result.event_occurred && actual_tick == *tick {
            accepted.push((*tick, trial, result));
        }
    }
    let (accepted_event_tick, result, terminal) = accepted
        .into_iter()
        .min_by_key(|(tick, _, _)| {
            tick.get().abs_diff(
                start.get()
                    + quantize_seconds_to_tick(
                        ModelTimeNs::new(0),
                        ModelTimeNs::new(support.support.duration_ns()),
                        full_result.hour_offset_seconds,
                    )
                    .map_or(0, ModelTimeNs::get),
            )
        })
        .ok_or(DirectSnowStage3V11AttachmentError::Terminal(
            "no actual terminal candidate satisfied coupled support",
        ))?;
    Ok((
        DirectSnowStage3V11TerminalReceipt {
            lane_id,
            support: support.support,
            result: terminal,
            candidate_ticks,
            accepted_event_tick,
        },
        result,
    ))
}

fn validate_receiver_topology(
    records: &[DirectSurfaceLiquidConfigurationRecord],
) -> Result<(), DirectSnowStage3V11AttachmentError> {
    let mut fractions = BTreeMap::<String, f64>::new();
    for record in records {
        if !record.tile_fraction.is_finite() || record.tile_fraction <= 0.0 {
            return Err(DirectSnowStage3V11AttachmentError::Identity(
                "receiver tile fraction",
            ));
        }
        let entry = fractions
            .entry(record.key.ofe_id.to_string())
            .or_insert(0.0);
        *entry += record.tile_fraction;
    }
    if fractions.values().any(|sum| (sum - 1.0).abs() > 1.0e-12) {
        return Err(DirectSnowStage3V11AttachmentError::Identity(
            "receiver area split",
        ));
    }
    Ok(())
}
