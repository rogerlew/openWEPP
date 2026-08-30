fn validate_destination_reconstruction_against_lane_aggregate(
    reconstructed: [f64; 5],
    aggregate: [f64; 5],
) -> Result<(), DirectV11RealConsumerError> {
    if reconstructed
        .iter()
        .zip(aggregate)
        .any(|(reconstructed, aggregate)| (reconstructed - aggregate).abs() > 1.0e-6)
    {
        return Err(DirectV11RealConsumerError::Identity(
            "physical ledger lane aggregate substitution",
        ));
    }
    Ok(())
}

fn reconstruct_interlayer_from_owner_states(
    lower_before: f64,
    lower_after: f64,
    reported_active: f64,
    reported_lower: f64,
) -> Result<(f64, f64), DirectV11RealConsumerError> {
    let reconstructed_lower = lower_before - lower_after;
    if (reported_lower - reconstructed_lower).abs() > 1.0e-9
        || (reported_active + reconstructed_lower).abs() > 1.0e-9
    {
        return Err(DirectV11RealConsumerError::Identity(
            "interlayer owner-state reconstruction",
        ));
    }
    Ok((-reconstructed_lower, reconstructed_lower))
}
