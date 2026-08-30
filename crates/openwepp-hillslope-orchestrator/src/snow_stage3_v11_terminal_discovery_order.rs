fn replay_preterminal_microsteps_before_terminal_localization_v1(
    event_occurred: bool,
    microstep_count: usize,
    forcing_snowfall_m: f64,
    sealed_positive_solid_parcel: bool,
) -> Result<bool, DirectSnowStage3V11AttachmentError> {
    if !forcing_snowfall_m.is_finite()
        || forcing_snowfall_m < 0.0
        || (forcing_snowfall_m > 0.0) != sealed_positive_solid_parcel
    {
        return Err(DirectSnowStage3V11AttachmentError::Identity(
            "adaptive preterminal solid forcing/parcel join",
        ));
    }
    // Persistent Stage-3 installs an enclosing support's solid source before
    // terminal enthalpy integration. Its inner trace states therefore cannot
    // be resealed as shorter children when that source is positive: each
    // child owns only its projected solid parcel. Fall through to the exact
    // support-consistent terminal/reappearance path instead.
    Ok(!event_occurred && microstep_count != 0 && !sealed_positive_solid_parcel)
}

fn localized_terminal_candidate_offsets_v1(
    terminal_entry_offset_seconds: f64,
    evaluated_seconds: f64,
    hour_offset_seconds: f64,
    bracket_lower_seconds: f64,
    bracket_upper_seconds: f64,
) -> Option<[f64; 3]> {
    let values = [
        terminal_entry_offset_seconds,
        evaluated_seconds,
        hour_offset_seconds,
        bracket_lower_seconds,
        bracket_upper_seconds,
    ];
    if values
        .iter()
        .any(|value| !value.is_finite() || *value < 0.0)
        || bracket_lower_seconds > bracket_upper_seconds
        || hour_offset_seconds.to_bits()
            != (terminal_entry_offset_seconds + evaluated_seconds).to_bits()
    {
        return None;
    }
    let bracket_lower = terminal_entry_offset_seconds + bracket_lower_seconds;
    let bracket_upper = terminal_entry_offset_seconds + bracket_upper_seconds;
    if bracket_lower > hour_offset_seconds || hour_offset_seconds > bracket_upper {
        return None;
    }
    Some([hour_offset_seconds, bracket_lower, bracket_upper])
}
