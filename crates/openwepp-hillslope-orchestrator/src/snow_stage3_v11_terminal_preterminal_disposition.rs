struct AcceptedPreterminalNonEventDispositionV1<'a> {
    support: TimeSupport,
    event_occurred: bool,
    terminal_entry_offset_seconds: f64,
    requested_seconds: f64,
    evaluated_seconds: f64,
    unevaluated_seconds: f64,
    hour_offset_seconds: f64,
    ending_is_supported_snow_domain: bool,
    microstep_supports: &'a [TimeSupport],
    microstep_states_are_exact: bool,
}

fn accepted_preterminal_non_event_disposition_v1(
    evidence: &AcceptedPreterminalNonEventDispositionV1<'_>,
) -> bool {
    let support_seconds = f64::from_bits(evidence.support.duration_s_bits());
    if evidence.event_occurred
        || evidence.terminal_entry_offset_seconds.to_bits() != 0.0_f64.to_bits()
        || evidence.requested_seconds.to_bits() != support_seconds.to_bits()
        || evidence.evaluated_seconds.to_bits() != support_seconds.to_bits()
        || evidence.unevaluated_seconds.abs() > 1.0e-6
        || evidence.hour_offset_seconds.to_bits() != support_seconds.to_bits()
        || !evidence.ending_is_supported_snow_domain
        || !evidence.microstep_states_are_exact
        || evidence.microstep_supports.is_empty()
    {
        return false;
    }
    let mut cursor = evidence.support.start_ns();
    for microstep in evidence.microstep_supports {
        if microstep.start_ns() != cursor || microstep.end_ns() > evidence.support.end_ns() {
            return false;
        }
        cursor = microstep.end_ns();
    }
    cursor == evidence.support.end_ns()
}
