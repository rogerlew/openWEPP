const SNOW_TERMINAL_ENTHALPY_EVENT_ENV: &str = "OPENWEPP_SNOW_TERMINAL_ENTHALPY_EVENT";

fn snow_terminal_enthalpy_event_requested() -> Result<bool, HillslopeCliError> {
    match std::env::var(SNOW_TERMINAL_ENTHALPY_EVENT_ENV) {
        Ok(value) => match value.trim() {
            "" | "0" | "false" | "disabled" => Ok(false),
            "1" | "true" | "enthalpy_event_v1" => Ok(true),
            observed => Err(HillslopeCliError::RuntimeSurfaceFailure {
                surface: "direct_production_snow_terminal_enthalpy_event",
                detail: format!(
                    "{SIMOUT_GUARD_ID} {SNOW_TERMINAL_ENTHALPY_EVENT_ENV} must be disabled, enthalpy_event_v1, true, false, 1, 0, or empty default, observed {observed}"
                ),
            }),
        },
        Err(std::env::VarError::NotPresent) => Ok(false),
        Err(std::env::VarError::NotUnicode(_)) => Err(HillslopeCliError::RuntimeSurfaceFailure {
            surface: "direct_production_snow_terminal_enthalpy_event",
            detail: format!("{SIMOUT_GUARD_ID} {SNOW_TERMINAL_ENTHALPY_EVENT_ENV} must be UTF-8"),
        }),
    }
}

fn snow_terminal_state_fingerprint(value: &serde_json::Value) -> Result<u64, &'static str> {
    fn add(mut hash: u64, value: u64) -> u64 {
        for byte in value.to_le_bytes() {
            hash ^= u64::from(byte);
            hash = hash.wrapping_mul(0x0000_0100_0000_01b3);
        }
        hash
    }
    let mut hash = 0xcbf2_9ce4_8422_2325;
    for field in ["schema_version", "lane_id", "next_interval_index"] {
        hash = add(hash, value[field].as_u64().ok_or("state integer")?);
    }
    if value["terminal_event_model"] == "enthalpy_event_v1" {
        hash = add(hash, 1);
    }
    for field in [
        "cumulative_snowfall_kg_m2", "cumulative_external_liquid_kg_m2",
        "cumulative_deposition_kg_m2", "cumulative_sublimation_kg_m2",
        "cumulative_melt_kg_m2", "cumulative_unresolved_liquid_kg_m2",
        "initial_ice_kg_m2", "initial_retained_liquid_kg_m2",
        "detached_retained_liquid_kg_m2", "cumulative_complete_energy_j_m2",
        "cumulative_cold_energy_change_j_m2",
        "cumulative_terminal_unallocated_energy_j_m2",
    ] {
        let number = value[field].as_f64().ok_or("state number")?;
        hash = add(hash, if number == 0.0 { 0 } else { number.to_bits() });
    }
    for layer in value["layers"].as_array().ok_or("state layers")? {
        for field in [
            "mass_swe_m", "thickness_m", "density_kg_m3", "settle_day_count",
            "temperature_c", "liquid_water_m", "cold_content_j_m2", "refrozen_liquid_m",
        ] {
            let number = layer[field].as_f64().ok_or("state layer number")?;
            hash = add(hash, if number == 0.0 { 0 } else { number.to_bits() });
        }
    }
    Ok(hash)
}

#[allow(clippy::too_many_lines)]
fn validate_terminal_interval(interval: &serde_json::Value) -> Result<(), &'static str> {
    let number = |field: &str| interval[field].as_f64().ok_or("terminal interval number");
    for field in [
        "start_ice_kg_m2",
        "start_liquid_kg_m2",
        "start_cold_content_j_m2",
        "end_ice_kg_m2",
        "terminal_liquid_kg_m2",
        "end_cold_content_j_m2",
        "external_liquid_kg_m2",
        "refrozen_kg_m2",
        "deposition_kg_m2",
        "sublimation_kg_m2",
        "melt_kg_m2",
        "terminal_unallocated_energy_j_m2",
        "terminal_entry_offset_seconds",
        "requested_seconds",
        "hour_offset_seconds",
        "evaluated_seconds",
        "unevaluated_seconds",
        "event_bracket_width_seconds",
        "event_bracket_lower_seconds",
        "event_bracket_upper_seconds",
        "event_bracket_lower_solid_kg_m2",
        "event_bracket_upper_solid_kg_m2",
        "lte_coarse_ice_kg_m2",
        "lte_fine_ice_kg_m2",
        "lte_coarse_liquid_kg_m2",
        "lte_fine_liquid_kg_m2",
        "lte_coarse_cold_content_j_m2",
        "lte_fine_cold_content_j_m2",
    ] {
        if number(field)? < 0.0 {
            return Err("terminal interval domain");
        }
    }
    let start_ice = number("start_ice_kg_m2")?;
    let end_ice = number("end_ice_kg_m2")?;
    let refrozen = number("refrozen_kg_m2")?;
    let deposition = number("deposition_kg_m2")?;
    let sublimation = number("sublimation_kg_m2")?;
    let melt = number("melt_kg_m2")?;
    let solid = start_ice + refrozen + deposition - sublimation - melt - end_ice;
    let liquid = number("start_liquid_kg_m2")? + number("external_liquid_kg_m2")? + melt
        - refrozen
        - number("terminal_liquid_kg_m2")?;
    let cold_change = number("start_cold_content_j_m2")?
        - number("end_cold_content_j_m2")?;
    let energy = number("complete_energy_j_m2")?
        - number("cold_energy_change_j_m2")?
        - 333_600.0 * melt
        + 333_600.0 * refrozen
        - number("terminal_unallocated_energy_j_m2")?;
    let components = number("shortwave_energy_j_m2")?
        + number("longwave_energy_j_m2")?
        + number("sensible_energy_j_m2")?
        + number("latent_energy_j_m2")?
        + number("advected_energy_j_m2")?;
    let scale = start_ice + end_ice + refrozen + deposition + sublimation + melt;
    let scaled = |coarse: f64, fine: f64, absolute: f64| {
        (coarse - fine).abs() / (absolute + 1.0e-8 * coarse.abs().max(fine.abs()))
    };
    let reconstructed_lte = scaled(
        number("lte_coarse_ice_kg_m2")?,
        number("lte_fine_ice_kg_m2")?,
        1.0e-9,
    )
    .max(scaled(
        number("lte_coarse_liquid_kg_m2")?,
        number("lte_fine_liquid_kg_m2")?,
        1.0e-9,
    ))
    .max(scaled(
        number("lte_coarse_cold_content_j_m2")?,
        number("lte_fine_cold_content_j_m2")?,
        1.0e-6,
    ))
    .max(scaled(
        number("lte_coarse_complete_energy_j_m2")?,
        number("lte_fine_complete_energy_j_m2")?,
        1.0e-6,
    ))
    .max(scaled(
        number("lte_coarse_unallocated_energy_j_m2")?,
        number("lte_fine_unallocated_energy_j_m2")?,
        1.0e-6,
    ));
    let bracket_lower = number("event_bracket_lower_seconds")?;
    let bracket_upper = number("event_bracket_upper_seconds")?;
    let bracket_width = number("event_bracket_width_seconds")?;
    let event_occurred = interval["event_occurred"]
        .as_bool()
        .ok_or("terminal interval event flag")?;
    if interval["model"] != "EnthalpyEventV1"
        || interval["hour_index"].as_u64().is_none_or(|hour| hour >= 24)
        || interval["accepted_trials"].as_u64().is_none_or(|count| count == 0)
        || interval["rejected_trials"].as_u64().is_none()
        || start_ice <= 0.0
        || start_ice > 1.0
        || solid.abs() > 1.0e-12_f64.max(1.0e-12 * scale)
        || liquid.abs() > 1.0e-12_f64.max(1.0e-12 * scale)
        || number("solid_mass_closure_residual_kg_m2")?.abs()
            > 1.0e-12_f64.max(1.0e-12 * scale)
        || number("liquid_mass_closure_residual_kg_m2")?.abs()
            > 1.0e-12_f64.max(1.0e-12 * scale)
        || (cold_change - number("cold_energy_change_j_m2")?).abs() > 1.0e-6
        || (components - number("complete_energy_j_m2")?).abs()
            > 1.0e-6_f64.max(1.0e-12 * components.abs())
        || energy.abs() > 1.0e-6_f64.max(1.0e-12 * number("complete_energy_j_m2")?.abs())
        || number("energy_closure_residual_j_m2")?.abs()
            > 1.0e-6_f64.max(1.0e-12 * number("complete_energy_j_m2")?.abs())
        || (number("requested_seconds")?
            - number("evaluated_seconds")?
            - number("unevaluated_seconds")?)
        .abs()
            > 1.0e-6
        || (number("terminal_entry_offset_seconds")? + number("evaluated_seconds")?
            - number("hour_offset_seconds")?)
        .abs()
            > 1.0e-6
        || reconstructed_lte > 1.0
        || (reconstructed_lte - number("maximum_scaled_error")?).abs() > 1.0e-12
        || bracket_lower > bracket_upper
        || (bracket_upper - bracket_lower - bracket_width).abs() > 1.0e-12
        || bracket_width > 1.0e-6
        || (!event_occurred && number("unevaluated_seconds")? != 0.0)
        || (event_occurred
            && (number("event_bracket_lower_solid_kg_m2")? <= 0.0
                || number("event_bracket_upper_solid_kg_m2")? != 0.0
                || end_ice != 0.0))
    {
        return Err("terminal interval reconstruction");
    }
    Ok(())
}

fn validate_snow_terminal_event_trace_consumer(
    persistent: &openwepp_hillslope_orchestrator::DirectSnowStage3PersistentDayResult,
) -> Result<(), &'static str> {
    const LATENT_HEAT_FUSION_J_KG: f64 = 333_600.0;
    if persistent.state.schema_version != 2 || persistent.start_state.schema_version != 2 {
        return Err("terminal event state schema mismatch");
    }
    let Some(event) = persistent.terminal_event else {
        return Ok(());
    };
    if event.model
        != openwepp_hillslope_orchestrator::DirectSnowTerminalEventModel::EnthalpyEventV1
        || event.hour_index >= 24
        || event.evaluated_seconds <= 0.0
        || event.unevaluated_seconds < 0.0
        || event.event_bracket_width_seconds > 1.0e-6
        || (event.event_occurred && event.end_ice_kg_m2 != 0.0)
    {
        return Err("terminal event identity or time domain");
    }
    let mass_scale = event.start_ice_kg_m2
        + event.refrozen_kg_m2
        + event.deposition_kg_m2
        + event.sublimation_kg_m2
        + event.melt_kg_m2;
    let mass_tolerance = 1.0e-12_f64.max(1.0e-12 * mass_scale);
    let solid = event.start_ice_kg_m2 + event.refrozen_kg_m2 + event.deposition_kg_m2
        - event.sublimation_kg_m2
        - event.melt_kg_m2
        - event.end_ice_kg_m2;
    let liquid = event.start_liquid_kg_m2 + event.external_liquid_kg_m2 + event.melt_kg_m2
        - event.refrozen_kg_m2
        - event.terminal_liquid_kg_m2;
    if solid.abs() > mass_tolerance
        || liquid.abs() > mass_tolerance
        || event.solid_mass_closure_residual_kg_m2.abs() > mass_tolerance
        || event.liquid_mass_closure_residual_kg_m2.abs() > mass_tolerance
    {
        return Err("terminal event mass reconstruction");
    }
    let components = event.shortwave_energy_j_m2
        + event.longwave_energy_j_m2
        + event.sensible_energy_j_m2
        + event.latent_energy_j_m2
        + event.advected_energy_j_m2;
    let energy = event.complete_energy_j_m2
        - event.cold_energy_change_j_m2
        - LATENT_HEAT_FUSION_J_KG * event.melt_kg_m2
        + LATENT_HEAT_FUSION_J_KG * event.refrozen_kg_m2
        - event.terminal_unallocated_energy_j_m2;
    let energy_tolerance = 1.0e-6_f64.max(
        1.0e-12
            * (event.complete_energy_j_m2.abs()
                + event.cold_energy_change_j_m2.abs()
                + (LATENT_HEAT_FUSION_J_KG * (event.melt_kg_m2 + event.refrozen_kg_m2))
                    .abs()
                + event.terminal_unallocated_energy_j_m2.abs()),
    );
    if (components - event.complete_energy_j_m2).abs() > energy_tolerance
        || energy.abs() > energy_tolerance
        || event.energy_closure_residual_j_m2.abs() > energy_tolerance
    {
        return Err("terminal event energy reconstruction");
    }
    for hour in persistent.evaluation.hourly.iter().skip(event.hour_index + 1) {
        if hour.evaluated_seconds != 0.0
            || hour.complete_energy_j_m2 != 0.0
            || hour.melt_kg_m2 != 0.0
            || hour.sublimation_kg_m2 != 0.0
            || hour.vapor_mass_exchange_kg_m2 != 0.0
        {
            return Err("post-terminal snow flux");
        }
    }
    Ok(())
}

#[allow(clippy::items_after_statements)]
#[allow(clippy::too_many_lines)]
fn validate_snow_terminal_event_trace_row(line: &str) -> Result<(), &'static str> {
    const LF: f64 = 333_600.0;
    let row: serde_json::Value = serde_json::from_str(line).map_err(|_| "invalid schema-v8 JSON")?;
    if row["schema"] != "openwepp-r7h-direct-production-snow-trace-v8"
        || row["stage3_terminal_event_model"] != "enthalpy_event_v1"
        || row["stage3_persistent_start_state"]["schema_version"] != 2
        || row["stage3_persistent_end_state"]["schema_version"] != 2
        || row["stage3_persistent_start_state"]["terminal_event_model"]
            != "enthalpy_event_v1"
        || row["stage3_persistent_end_state"]["terminal_event_model"]
            != "enthalpy_event_v1"
    {
        return Err("schema-v8 request/state binding");
    }
    fn contains_recipient_claim(value: &serde_json::Value) -> bool {
        match value {
            serde_json::Value::Object(object) => object.iter().any(|(key, value)| {
                key.contains("recipient")
                    || key.contains("receiving")
                    || key.contains("handoff")
                    || contains_recipient_claim(value)
            }),
            serde_json::Value::Array(values) => values.iter().any(contains_recipient_claim),
            _ => false,
        }
    }
    if contains_recipient_claim(&row) {
        return Err("schema-v8 unsupported receiving-surface claim");
    }
    for (state_field, alias_field) in [
        ("stage3_persistent_start_state", "stage3_persistent_start_state_fingerprint"),
        ("stage3_persistent_end_state", "stage3_persistent_end_state_fingerprint"),
    ] {
        let state = &row[state_field];
        if format!("{:016x}", snow_terminal_state_fingerprint(state)?)
            != row[alias_field].as_str().ok_or("state fingerprint alias")?
            || state["fingerprint"] != row[alias_field]
        {
            return Err("schema-v8 state fingerprint reconstruction");
        }
    }
    if row["stage3_terminal_event"].is_null() {
        return Ok(());
    }
    let event = row["stage3_terminal_event"]
        .as_object()
        .ok_or("schema-v8 terminal evidence malformed")?;
    const ALLOWED: [&str; 31] = [
        "event_occurred", "hour_index", "hour_offset_seconds", "evaluated_seconds",
        "unevaluated_seconds", "start_ice_kg_m2", "start_liquid_kg_m2",
        "start_cold_content_j_m2", "end_ice_kg_m2", "terminal_liquid_kg_m2",
        "end_cold_content_j_m2", "complete_energy_j_m2", "shortwave_energy_j_m2",
        "longwave_energy_j_m2", "sensible_energy_j_m2", "latent_energy_j_m2",
        "advected_energy_j_m2", "external_liquid_kg_m2", "cold_energy_change_j_m2",
        "refrozen_kg_m2", "deposition_kg_m2", "sublimation_kg_m2", "melt_kg_m2",
        "terminal_unallocated_energy_j_m2", "solid_mass_closure_residual_kg_m2",
        "liquid_mass_closure_residual_kg_m2", "energy_closure_residual_j_m2",
        "event_bracket_width_seconds", "accepted_trials", "rejected_trials",
        "maximum_scaled_error",
    ];
    let event_occurred = event["event_occurred"]
        .as_bool()
        .ok_or("schema-v8 event flag")?;
    if event.keys().any(|key| !ALLOWED.contains(&key.as_str())) {
        return Err("schema-v8 unknown or nonterminal evidence");
    }
    let number = |field: &str| event[field].as_f64().ok_or("schema-v8 numeric field");
    let start_ice = number("start_ice_kg_m2")?;
    let end_ice = number("end_ice_kg_m2")?;
    let refrozen = number("refrozen_kg_m2")?;
    let deposition = number("deposition_kg_m2")?;
    let sublimation = number("sublimation_kg_m2")?;
    let melt = number("melt_kg_m2")?;
    let start_liquid = number("start_liquid_kg_m2")?;
    let external = number("external_liquid_kg_m2")?;
    let terminal_liquid = number("terminal_liquid_kg_m2")?;
    let mass_scale = start_ice + refrozen + deposition + sublimation + melt + end_ice;
    let mass_tolerance = 1.0e-12_f64.max(1.0e-12 * mass_scale);
    let solid = start_ice + refrozen + deposition - sublimation - melt - end_ice;
    let liquid = start_liquid + external + melt - refrozen - terminal_liquid;
    if solid.abs() > mass_tolerance
        || liquid.abs() > mass_tolerance
        || number("solid_mass_closure_residual_kg_m2")?.abs() > mass_tolerance
        || number("liquid_mass_closure_residual_kg_m2")?.abs() > mass_tolerance
    {
        return Err("schema-v8 mass reconstruction");
    }
    let complete = number("complete_energy_j_m2")?;
    let components = number("shortwave_energy_j_m2")?
        + number("longwave_energy_j_m2")?
        + number("sensible_energy_j_m2")?
        + number("latent_energy_j_m2")?
        + number("advected_energy_j_m2")?;
    let phase = complete - number("cold_energy_change_j_m2")? - LF * melt
        + LF * refrozen
        - number("terminal_unallocated_energy_j_m2")?;
    let energy_tolerance = 1.0e-6_f64.max(1.0e-12 * complete.abs().max(components.abs()));
    if (complete - components).abs() > energy_tolerance
        || phase.abs() > energy_tolerance
        || number("energy_closure_residual_j_m2")?.abs() > energy_tolerance
    {
        return Err("schema-v8 energy reconstruction");
    }
    let evaluated = number("evaluated_seconds")?;
    let unevaluated = number("unevaluated_seconds")?;
    let offset = number("hour_offset_seconds")?;
    if evaluated <= 0.0
        || unevaluated < 0.0
        || offset < evaluated
        || offset > 3_600.0
        || evaluated + unevaluated > 3_600.0 + 1.0e-6
        || number("event_bracket_width_seconds")? > 1.0e-6
        || number("maximum_scaled_error")? > 1.0
        || (event_occurred && end_ice != 0.0)
    {
        return Err("schema-v8 time/root/LTE evidence");
    }
    let witness = row["stage3_terminal_event_solver_witness"]
        .as_object()
        .ok_or("schema-v8 solver witness missing")?;
    let intervals = row["stage3_terminal_intervals"]
        .as_array()
        .ok_or("schema-v8 terminal interval sequence missing")?;
    let tuples = row["stage3_operator_reconciliation"]["tuples"]
        .as_array()
        .ok_or("schema-v8 reconciliation tuples")?;
    let transitions = row["stage3_terminal_transition_witnesses"]
        .as_array()
        .ok_or("schema-v8 transition witnesses")?;
    if transitions.len() != tuples.len() {
        return Err("schema-v8 transition witness count");
    }
    let event_hour = event["hour_index"]
        .as_u64()
        .ok_or("schema-v8 event hour")?;
    let event_absolute = f64::from(
        u32::try_from(event_hour).map_err(|_| "schema-v8 event hour")?,
    ) * 3_600.0
        + offset;
    for (tuple, transition) in tuples.iter().zip(transitions) {
        let tuple_hour = tuple["hour_index"]
            .as_u64()
            .ok_or("schema-v8 tuple hour")?;
        let tuple_start = tuple["elapsed_start_seconds"]
            .as_f64()
            .ok_or("schema-v8 tuple start")?;
        let tuple_duration = tuple["duration_seconds"]
            .as_f64()
            .ok_or("schema-v8 tuple duration")?;
        let tuple_end = f64::from(
            u32::try_from(tuple_hour).map_err(|_| "schema-v8 tuple hour")?,
        ) * 3_600.0
            + tuple_start
            + tuple_duration;
        if !(0.0..=3_600.0).contains(&tuple_start)
            || tuple_duration <= 0.0
            || tuple_start + tuple_duration > 3_600.0 + 1.0e-6
            || transition["hour_index"] != tuple["hour_index"]
            || transition["elapsed_start_seconds"] != tuple["elapsed_start_seconds"]
            || transition["duration_seconds"] != tuple["duration_seconds"]
            || transition["total_retained_liquid_after_kg_m2"]
                .as_f64()
                .is_none_or(|liquid| liquid < 0.0)
            || (event_occurred && tuple_end > event_absolute + 1.0e-6)
        {
            return Err("schema-v8 tuple chronology");
        }
    }
    if intervals.is_empty() || intervals.last() != Some(&row["stage3_terminal_event_solver_witness"])
    {
        return Err("schema-v8 terminal interval chronology");
    }
    for key in ALLOWED {
        if event[key] != witness[key] {
            return Err("schema-v8 event/witness alias mismatch");
        }
    }
    for (index, interval) in intervals.iter().enumerate() {
        validate_terminal_interval(interval)?;
        if interval["event_occurred"].as_bool().ok_or("interval event flag")?
            != (index + 1 == intervals.len() && event_occurred)
        {
            return Err("schema-v8 terminal event finality");
        }
        if let Some(previous) = index.checked_sub(1).map(|prior| &intervals[prior]) {
            let previous_hour = previous["hour_index"].as_u64().ok_or("interval hour")?;
            let hour = interval["hour_index"].as_u64().ok_or("interval hour")?;
            let previous_offset = previous["hour_offset_seconds"].as_f64().ok_or("interval offset")?;
            let entry_offset = interval["terminal_entry_offset_seconds"].as_f64().ok_or("interval entry offset")?;
            let previous_hour_f64 = f64::from(
                u32::try_from(previous_hour).map_err(|_| "interval transition hour")?,
            );
            let hour_f64 =
                f64::from(u32::try_from(hour).map_err(|_| "interval transition hour")?);
            let previous_end = previous_hour_f64 * 3_600.0 + previous_offset;
            let current_entry = hour_f64 * 3_600.0 + entry_offset;
            let mut last_transition = None;
            let mut last_transition_liquid = None;
            let mut covered_until = previous_end;
            for (tuple, transition) in tuples.iter().zip(transitions) {
                let tuple_hour = tuple["hour_index"]
                    .as_u64()
                    .ok_or("interval transition hour")?;
                let tuple_start = tuple["elapsed_start_seconds"]
                    .as_f64()
                    .ok_or("interval transition start")?;
                let tuple_duration = tuple["duration_seconds"]
                    .as_f64()
                    .ok_or("interval transition duration")?;
                let tuple_hour_f64 = f64::from(
                    u32::try_from(tuple_hour).map_err(|_| "interval transition hour")?,
                );
                let tuple_absolute_start = tuple_hour_f64 * 3_600.0 + tuple_start;
                let tuple_end = tuple_absolute_start + tuple_duration;
                if !(0.0..=3_600.0).contains(&tuple_start)
                    || tuple_duration <= 0.0
                    || tuple_start + tuple_duration > 3_600.0 + 1.0e-6
                    || transition["hour_index"] != tuple["hour_index"]
                    || transition["elapsed_start_seconds"] != tuple["elapsed_start_seconds"]
                    || transition["duration_seconds"] != tuple["duration_seconds"]
                {
                    return Err("interval transition time domain");
                }
                if tuple_end > previous_end && tuple_end <= current_entry + 1.0e-6 {
                    if (tuple_absolute_start - covered_until).abs() > 1.0e-6 {
                        return Err("interval transition time gap");
                    }
                    covered_until = tuple_end;
                    last_transition = Some(tuple);
                    last_transition_liquid = Some(
                        transition["total_retained_liquid_after_kg_m2"]
                            .as_f64()
                            .ok_or("interval transition liquid")?,
                    );
                }
            }
            let expected_ice = if let Some(tuple) = last_transition {
                tuple["total_ice_mass_after_kg_m2"]
                    .as_f64()
                    .ok_or("interval transition ice")?
            } else {
                previous["end_ice_kg_m2"].as_f64().ok_or("interval ice")?
                    + interval["entry_solid_precipitation_kg_m2"]
                        .as_f64()
                        .ok_or("interval precipitation")?
            };
            let expected_cold = if let Some(tuple) = last_transition {
                tuple["total_cold_after_j_m2"]
                    .as_f64()
                    .ok_or("interval transition cold")?
            } else {
                previous["end_cold_content_j_m2"]
                    .as_f64()
                    .ok_or("interval cold")?
            };
            let expected_liquid = last_transition_liquid.unwrap_or(
                previous["terminal_liquid_kg_m2"]
                    .as_f64()
                    .ok_or("interval liquid")?,
            );
            if hour < previous_hour
                || (hour == previous_hour && entry_offset < previous_offset)
                || (covered_until - current_entry).abs() > 1.0e-6
                || (expected_ice
                    - interval["start_ice_kg_m2"].as_f64().ok_or("interval ice")?)
                .abs()
                    > 1.0e-12
                || (expected_cold
                    - interval["start_cold_content_j_m2"]
                        .as_f64()
                        .ok_or("interval cold")?)
                .abs()
                    > 1.0e-6
                || (expected_liquid
                    - interval["start_liquid_kg_m2"]
                        .as_f64()
                        .ok_or("interval liquid")?)
                .abs()
                    > 1.0e-12
            {
                return Err("schema-v8 terminal interval continuity");
            }
        }
    }
    let sum = |field: &str| {
        intervals
            .iter()
            .map(|interval| interval[field].as_f64().ok_or("schema-v8 interval operand"))
            .try_fold(0.0, |total, value| value.map(|value| total + value))
    };
    for (interval_field, day_field) in [
        ("deposition_kg_m2", "stage3_terminal_deposition_kg_m2"),
        ("sublimation_kg_m2", "stage3_terminal_sublimation_kg_m2"),
        ("melt_kg_m2", "stage3_terminal_melt_kg_m2"),
        (
            "terminal_unallocated_energy_j_m2",
            "stage3_terminal_unallocated_energy_j_m2",
        ),
        ("complete_energy_j_m2", "stage3_terminal_complete_energy_j_m2"),
        ("cold_energy_change_j_m2", "stage3_terminal_cold_energy_change_j_m2"),
        ("external_liquid_kg_m2", "stage3_terminal_external_liquid_kg_m2"),
        ("evaluated_seconds", "stage3_terminal_evaluated_seconds"),
    ] {
        let tolerance = if interval_field.ends_with("kg_m2") {
            1.0e-12
        } else {
            1.0e-6
        };
        if (sum(interval_field)? - row[day_field].as_f64().ok_or("schema-v8 day operand")?)
            .abs()
            > tolerance
        {
            return Err("schema-v8 terminal interval aggregate");
        }
    }
    if (sum("refrozen_kg_m2")?
        - row["stage3_terminal_refrozen_kg_m2"]
            .as_f64()
            .ok_or("schema-v8 refreeze aggregate")?)
    .abs()
        > 1.0e-12
    {
        return Err("schema-v8 terminal refreeze aggregate");
    }
    let witness_number = |field: &str| {
        witness[field]
            .as_f64()
            .ok_or("schema-v8 solver witness numeric field")
    };
    let witness_requested = witness_number("requested_seconds")?;
    let witness_entry = witness_number("terminal_entry_offset_seconds")?;
    if (witness_requested - evaluated - unevaluated).abs() > 1.0e-6
        || (witness_entry + evaluated - offset).abs() > 1.0e-6
        || start_ice <= 0.0
        || start_ice > 1.0
        || event["hour_index"].as_u64().is_none_or(|hour| hour >= 24)
        || event["accepted_trials"].as_u64().is_none_or(|count| count == 0)
        || event["rejected_trials"].as_u64().is_none()
    {
        return Err("schema-v8 support/domain evidence");
    }
    let hour_index = usize::try_from(
        event["hour_index"].as_u64().ok_or("schema-v8 hour index")?,
    )
    .map_err(|_| "schema-v8 hour index")?;
    let hourly = |field: &str| {
        row[field]
            .as_array()
            .ok_or("schema-v8 hourly array")?
            .iter()
            .map(|value| value.as_f64().ok_or("schema-v8 hourly operand"))
            .collect::<Result<Vec<_>, _>>()
    };
    let hourly_evaluated = hourly("stage3_evaluation_hourly_evaluated_seconds")?;
    if hourly_evaluated.len() != 24
        || (event_occurred && (hourly_evaluated[hour_index] - offset).abs() > 1.0e-6)
        || (event_occurred
            && hourly_evaluated
                .iter()
                .skip(hour_index + 1)
                .any(|value| *value != 0.0))
    {
        return Err("schema-v8 event-hour support alias");
    }
    for field in [
        "stage3_evaluation_hourly_complete_energy_j_m2",
        "stage3_evaluation_hourly_vapor_mass_exchange_kg_m2",
        "stage3_evaluation_hourly_sublimation_kg_m2",
        "stage3_evaluation_hourly_melt_kg_m2",
        "stage3_evaluation_hourly_terminal_unallocated_j_m2",
    ] {
        let values = hourly(field)?;
        if values.len() != 24
            || (event_occurred
                && values
                    .iter()
                    .skip(hour_index + 1)
                    .any(|value| *value != 0.0))
        {
            return Err("schema-v8 post-event snow flux");
        }
    }
    let hour_u64 = u64::try_from(hour_index).map_err(|_| "schema-v8 hour index")?;
    let tuple_sum = |field: &str| -> Result<f64, &'static str> {
        tuples
            .iter()
            .filter(|tuple| tuple["hour_index"].as_u64() == Some(hour_u64))
            .map(|tuple| tuple[field].as_f64().ok_or("schema-v8 tuple operand"))
            .try_fold(0.0, |total, value| value.map(|value| total + value))
    };
    let interval_hour_sum = |field: &str| -> Result<f64, &'static str> {
        intervals
            .iter()
            .filter(|interval| interval["hour_index"].as_u64() == Some(hour_u64))
            .map(|interval| interval[field].as_f64().ok_or("schema-v8 interval operand"))
            .try_fold(0.0, |total, value| value.map(|value| total + value))
    };
    for (hourly_field, tuple_field, interval_field, tolerance) in [
        (
            "stage3_evaluation_hourly_complete_energy_j_m2",
            "legacy_sequential_complete_j_m2",
            "complete_energy_j_m2",
            1.0e-6,
        ),
        (
            "stage3_evaluation_hourly_melt_kg_m2",
            "melt_kg_m2",
            "melt_kg_m2",
            1.0e-12,
        ),
        (
            "stage3_evaluation_hourly_sublimation_kg_m2",
            "sublimation_kg_m2",
            "sublimation_kg_m2",
            1.0e-12,
        ),
    ] {
        let values = hourly(hourly_field)?;
        if (values[hour_index] - tuple_sum(tuple_field)? - interval_hour_sum(interval_field)?).abs()
            > tolerance
        {
            return Err("schema-v8 event-hour full-step alias");
        }
    }
    let hourly_vapor = hourly("stage3_evaluation_hourly_vapor_mass_exchange_kg_m2")?;
    if (hourly_vapor[hour_index]
        - tuple_sum("vapor_mass_exchange_kg_m2")?
        - interval_hour_sum("deposition_kg_m2")?
        + interval_hour_sum("sublimation_kg_m2")?)
    .abs()
        > 1.0e-12
    {
        return Err("schema-v8 event-hour vapor alias");
    }
    let bracket_lower = witness_number("event_bracket_lower_seconds")?;
    let bracket_upper = witness_number("event_bracket_upper_seconds")?;
    if bracket_lower > bracket_upper
        || (number("event_bracket_width_seconds")?
            - witness_number("event_bracket_width_seconds")?)
        .abs()
            > 1.0e-12
        || (number("maximum_scaled_error")? - witness_number("maximum_scaled_error")?).abs()
            > 1.0e-12
        || (bracket_upper - bracket_lower
            - witness_number("event_bracket_width_seconds")?)
        .abs()
            > 1.0e-12
        || (event_occurred
            && (witness_number("event_bracket_lower_solid_kg_m2")? <= 0.0
                || witness_number("event_bracket_upper_solid_kg_m2")? != 0.0))
    {
        return Err("schema-v8 bracket reconstruction");
    }
    let scaled = |coarse: f64, fine: f64, absolute: f64| {
        (coarse - fine).abs() / (absolute + 1.0e-8 * coarse.abs().max(fine.abs()))
    };
    let reconstructed_lte = scaled(
        witness_number("lte_coarse_ice_kg_m2")?,
        witness_number("lte_fine_ice_kg_m2")?,
        1.0e-9,
    )
    .max(scaled(
        witness_number("lte_coarse_liquid_kg_m2")?,
        witness_number("lte_fine_liquid_kg_m2")?,
        1.0e-9,
    ))
    .max(scaled(
        witness_number("lte_coarse_cold_content_j_m2")?,
        witness_number("lte_fine_cold_content_j_m2")?,
        1.0e-6,
    ))
    .max(scaled(
        witness_number("lte_coarse_complete_energy_j_m2")?,
        witness_number("lte_fine_complete_energy_j_m2")?,
        1.0e-6,
    ))
    .max(scaled(
        witness_number("lte_coarse_unallocated_energy_j_m2")?,
        witness_number("lte_fine_unallocated_energy_j_m2")?,
        1.0e-6,
    ));
    if (reconstructed_lte - witness_number("maximum_scaled_error")?).abs() > 1.0e-12
        || reconstructed_lte > 1.0
    {
        return Err("schema-v8 LTE reconstruction");
    }
    Ok(())
}

#[cfg(test)]
mod snow_terminal_event_trace_tests {
    use super::*;

    #[allow(clippy::too_many_lines)]
    fn valid_row() -> serde_json::Value {
        let mut row = serde_json::json!({
            "schema": "openwepp-r7h-direct-production-snow-trace-v8",
            "stage3_terminal_event_model": "enthalpy_event_v1",
            "stage3_persistent_start_state": {
                "schema_version": 2, "terminal_event_model": "enthalpy_event_v1"
            },
            "stage3_persistent_end_state": {
                "schema_version": 2, "terminal_event_model": "enthalpy_event_v1"
            },
            "stage3_terminal_event": {
                "event_occurred": true, "hour_index": 4,
                "hour_offset_seconds": 600.0, "evaluated_seconds": 600.0,
                "unevaluated_seconds": 3000.0,
                "start_ice_kg_m2": 0.6, "start_liquid_kg_m2": 0.1,
                "start_cold_content_j_m2": 0.0, "end_ice_kg_m2": 0.0,
                "terminal_liquid_kg_m2": 0.7, "end_cold_content_j_m2": 0.0,
                "complete_energy_j_m2": 200_160.0,
                "shortwave_energy_j_m2": 120_000.0,
                "longwave_energy_j_m2": 10000.0,
                "sensible_energy_j_m2": 30000.0,
                "latent_energy_j_m2": 20160.0,
                "advected_energy_j_m2": 20000.0,
                "external_liquid_kg_m2": 0.0,
                "cold_energy_change_j_m2": 0.0, "refrozen_kg_m2": 0.0,
                "deposition_kg_m2": 0.0, "sublimation_kg_m2": 0.0,
                "melt_kg_m2": 0.6, "terminal_unallocated_energy_j_m2": 0.0,
                "solid_mass_closure_residual_kg_m2": 0.0,
                "liquid_mass_closure_residual_kg_m2": 0.0,
                "energy_closure_residual_j_m2": 0.0,
                "event_bracket_width_seconds": 0.000_001,
                "accepted_trials": 10, "rejected_trials": 1,
                "maximum_scaled_error": 0.0
            },
            "stage3_terminal_event_solver_witness": {
                "terminal_entry_offset_seconds": 0.0,
                "requested_seconds": 3600.0,
                "entry_solid_precipitation_kg_m2": 0.0,
                "event_bracket_lower_seconds": 599.999_999,
                "event_bracket_upper_seconds": 600.0,
                "event_bracket_width_seconds": 0.000_001,
                "event_bracket_lower_solid_kg_m2": 0.000_000_001,
                "event_bracket_upper_solid_kg_m2": 0.0,
                "lte_coarse_ice_kg_m2": 0.3, "lte_fine_ice_kg_m2": 0.3,
                "lte_coarse_liquid_kg_m2": 0.3, "lte_fine_liquid_kg_m2": 0.3,
                "lte_coarse_cold_content_j_m2": 0.0,
                "lte_fine_cold_content_j_m2": 0.0,
                "lte_coarse_complete_energy_j_m2": 100_080.0,
                "lte_fine_complete_energy_j_m2": 100_080.0,
                "lte_coarse_unallocated_energy_j_m2": 0.0,
                "lte_fine_unallocated_energy_j_m2": 0.0,
                "deposition_kg_m2": 0.0, "sublimation_kg_m2": 0.0,
                "melt_kg_m2": 0.6, "refrozen_kg_m2": 0.0,
                "terminal_unallocated_energy_j_m2": 0.0,
                "maximum_scaled_error": 0.0
            }
        });
        let state = |next_interval_index: u64| serde_json::json!({
            "schema_version": 2, "terminal_event_model": "enthalpy_event_v1",
            "fingerprint": "", "lane_id": 0, "next_interval_index": next_interval_index,
            "layers": [], "detached_retained_liquid_kg_m2": 0.0,
            "initial_ice_kg_m2": 0.6, "initial_retained_liquid_kg_m2": 0.1,
            "cumulative_snowfall_kg_m2": 0.0, "cumulative_external_liquid_kg_m2": 0.0,
            "cumulative_deposition_kg_m2": 0.0, "cumulative_sublimation_kg_m2": 0.0,
            "cumulative_melt_kg_m2": 0.0, "cumulative_unresolved_liquid_kg_m2": 0.0,
            "cumulative_complete_energy_j_m2": 0.0,
            "cumulative_cold_energy_change_j_m2": 0.0,
            "cumulative_terminal_unallocated_energy_j_m2": 0.0
        });
        for (state_field, alias_field, next) in [
            ("stage3_persistent_start_state", "stage3_persistent_start_state_fingerprint", 0),
            ("stage3_persistent_end_state", "stage3_persistent_end_state_fingerprint", 1),
        ] {
            row[state_field] = state(next);
            let fingerprint = format!("{:016x}", snow_terminal_state_fingerprint(&row[state_field]).unwrap());
            row[state_field]["fingerprint"] = serde_json::json!(fingerprint);
            row[alias_field] = serde_json::json!(fingerprint);
        }
        let mut interval = row["stage3_terminal_event"].clone();
        for (key, value) in row["stage3_terminal_event_solver_witness"]
            .as_object()
            .unwrap()
        {
            interval[key] = value.clone();
        }
        interval["model"] = serde_json::json!("EnthalpyEventV1");
        row["stage3_terminal_event_solver_witness"] = interval.clone();
        row["stage3_terminal_intervals"] = serde_json::json!([interval]);
        row["stage3_terminal_deposition_kg_m2"] = serde_json::json!(0.0);
        row["stage3_terminal_sublimation_kg_m2"] = serde_json::json!(0.0);
        row["stage3_terminal_melt_kg_m2"] = serde_json::json!(0.6);
        row["stage3_terminal_unallocated_energy_j_m2"] = serde_json::json!(0.0);
        row["stage3_terminal_refrozen_kg_m2"] = serde_json::json!(0.0);
        row["stage3_terminal_complete_energy_j_m2"] = serde_json::json!(200_160.0);
        row["stage3_terminal_cold_energy_change_j_m2"] = serde_json::json!(0.0);
        row["stage3_terminal_external_liquid_kg_m2"] = serde_json::json!(0.0);
        row["stage3_terminal_evaluated_seconds"] = serde_json::json!(600.0);
        let mut evaluated = vec![0.0; 24];
        evaluated[4] = 600.0;
        let mut complete = vec![0.0; 24];
        complete[4] = 200_160.0;
        let mut melt = vec![0.0; 24];
        melt[4] = 0.6;
        row["stage3_evaluation_hourly_evaluated_seconds"] = serde_json::json!(evaluated);
        row["stage3_evaluation_hourly_complete_energy_j_m2"] = serde_json::json!(complete);
        row["stage3_evaluation_hourly_vapor_mass_exchange_kg_m2"] =
            serde_json::json!(vec![0.0; 24]);
        row["stage3_evaluation_hourly_sublimation_kg_m2"] =
            serde_json::json!(vec![0.0; 24]);
        row["stage3_evaluation_hourly_melt_kg_m2"] = serde_json::json!(melt);
        row["stage3_evaluation_hourly_terminal_unallocated_j_m2"] =
            serde_json::json!(vec![0.0; 24]);
        row["stage3_operator_reconciliation"] = serde_json::json!({"tuples": []});
        row["stage3_terminal_transition_witnesses"] = serde_json::json!([]);
        row
    }

    #[test]
    fn parsed_schema_v8_consumer_reconstructs_and_rejects_poison() {
        let row = valid_row();
        validate_snow_terminal_event_trace_row(&row.to_string()).unwrap();
        for (field, value) in [
            ("melt_kg_m2", 0.59),
            ("energy_closure_residual_j_m2", 2.0),
            ("maximum_scaled_error", 1.1),
        ] {
            let mut poisoned = row.clone();
            poisoned["stage3_terminal_event"][field] = serde_json::json!(value);
            assert!(validate_snow_terminal_event_trace_row(&poisoned.to_string()).is_err());
        }
        let mut recipient = row;
        recipient["stage3_terminal_event"]["receiving_ground_energy_j_m2"] =
            serde_json::json!(1.0);
        assert!(validate_snow_terminal_event_trace_row(&recipient.to_string()).is_err());
    }

    #[test]
    fn parsed_schema_v8_consumer_rejects_adaptive_alias_and_scope_poisons() {
        let row = valid_row();
        let mut adaptive = row["stage3_terminal_intervals"][0].clone();
        adaptive["maximum_scaled_error"] = serde_json::json!(1.1);
        assert!(validate_terminal_interval(&adaptive).is_err());

        let mut alias = row.clone();
        alias["stage3_terminal_event"]["end_ice_kg_m2"] = serde_json::json!(0.1);
        alias["stage3_terminal_event"]["start_ice_kg_m2"] = serde_json::json!(0.7);
        assert!(validate_snow_terminal_event_trace_row(&alias.to_string()).is_err());

        let mut negative_liquid = row.clone();
        for field in [
            "stage3_terminal_event",
            "stage3_terminal_event_solver_witness",
        ] {
            negative_liquid[field]["start_liquid_kg_m2"] = serde_json::json!(-0.1);
            negative_liquid[field]["terminal_liquid_kg_m2"] = serde_json::json!(0.5);
        }
        negative_liquid["stage3_terminal_intervals"][0]["start_liquid_kg_m2"] =
            serde_json::json!(-0.1);
        negative_liquid["stage3_terminal_intervals"][0]["terminal_liquid_kg_m2"] =
            serde_json::json!(0.5);
        assert!(validate_snow_terminal_event_trace_row(&negative_liquid.to_string()).is_err());

        let mut malformed_tuple = row.clone();
        malformed_tuple["stage3_operator_reconciliation"]["tuples"] =
            serde_json::json!([{"hour_index": 4}]);
        assert!(validate_snow_terminal_event_trace_row(&malformed_tuple.to_string()).is_err());

        let mut post_event_tuple = row.clone();
        post_event_tuple["stage3_operator_reconciliation"]["tuples"] = serde_json::json!([{
            "hour_index": 5, "elapsed_start_seconds": 0.0, "duration_seconds": 60.0
        }]);
        post_event_tuple["stage3_terminal_transition_witnesses"] = serde_json::json!([{
            "hour_index": 5, "elapsed_start_seconds": 0.0, "duration_seconds": 60.0,
            "total_retained_liquid_after_kg_m2": 0.7
        }]);
        assert!(validate_snow_terminal_event_trace_row(&post_event_tuple.to_string()).is_err());

        let mut recipient = row;
        recipient["receiving_ground_energy_j_m2"] = serde_json::json!(1.0);
        assert!(validate_snow_terminal_event_trace_row(&recipient.to_string()).is_err());
    }
}
