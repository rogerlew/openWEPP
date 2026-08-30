fn checked_cumulative_delta(
    beginning: f64,
    ending: f64,
    field: &'static str,
) -> Result<f64, DirectRuntimeError> {
    validate_finite(field, beginning)?;
    validate_finite(field, ending)?;
    let delta = ending - beginning;
    validate_nonnegative_direct_m(field, delta)?;
    Ok(delta)
}

fn add_nonnegative(target: &mut f64, value: f64) -> Result<(), DirectRuntimeError> {
    validate_nonnegative_direct_m("stage3_publication.accepted_amount_m", value)?;
    *target += value;
    validate_finite("stage3_publication.accepted_amount_sum_m", *target)
}

fn stage3_publication_guard(detail: &'static str) -> DirectRuntimeError {
    DirectRuntimeError::DirectKernelGuardFailure {
        phase: "stage3_committed_publication",
        detail: detail.into(),
    }
}
