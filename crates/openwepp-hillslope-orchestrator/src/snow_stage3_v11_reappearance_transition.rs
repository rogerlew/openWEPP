struct SolidReappearanceTransitionV1 {
    lanes: BTreeSet<u32>,
    parent: V11ParentTransaction,
    consumer: DirectV10RealConsumerShadow,
    clock: CoupledClockStateV1,
    stage3: BTreeMap<u32, DirectSnowStage3PersistentState>,
    accepted_event: AcceptedEventReceiptV1,
}

fn validate_solid_reappearance_publication_posture_v1(
    at_open_parent_beginning: bool,
    transitioned_state_matches: bool,
    publication_retained: bool,
    publication_is_ordered_tail: bool,
) -> Result<(), DirectSnowStage3V11AttachmentError> {
    let valid = if at_open_parent_beginning {
        transitioned_state_matches && !publication_retained && !publication_is_ordered_tail
    } else {
        publication_retained && publication_is_ordered_tail
    };
    if !valid {
        return Err(DirectSnowStage3V11AttachmentError::Identity(
            "solid reappearance retained event/state join",
        ));
    }
    Ok(())
}
