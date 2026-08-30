fn validate_event_handoff_header(
    event: &AcceptedEventHandoffHeader,
    expected_parent_transaction_sha256: Digest32,
    expected_beginning_owner_sha256: Digest32,
    event_ids: &mut BTreeSet<Digest32>,
    last_event_ordinal_by_parent: &mut BTreeMap<Digest32, u32>,
) -> Result<(), DirectRuntimeError> {
    let ordinal_is_exact = last_event_ordinal_by_parent
        .get(&event.parent_transaction_sha256)
        .map_or(event.ordinal == 0, |previous| {
            previous.checked_add(1) == Some(event.ordinal)
        });
    if !event.seal_is_valid
        || event.receipt_id_sha256 == Digest32::zero()
        || event.parent_transaction_sha256 != expected_parent_transaction_sha256
        || event.beginning_complete_owner_set_sha256 != expected_beginning_owner_sha256
        || event.ending_complete_owner_set_sha256 == Digest32::zero()
        || !event_ids.insert(event.receipt_id_sha256)
        || !ordinal_is_exact
    {
        return Err(stage3_publication_guard(
            "accepted publication event handoff authority",
        ));
    }
    last_event_ordinal_by_parent.insert(event.parent_transaction_sha256, event.ordinal);
    Ok(())
}

fn terminal_lane_support_matches(
    candidate_lane_id: u32,
    candidate_support: openwepp_coupled_time::TimeSupport,
    output_lane_id: u32,
    output_support: openwepp_coupled_time::TimeSupport,
) -> bool {
    candidate_lane_id == output_lane_id && candidate_support == output_support
}
