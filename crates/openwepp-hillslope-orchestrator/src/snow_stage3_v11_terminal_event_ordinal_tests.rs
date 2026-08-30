use super::*;

fn parent(byte: u8) -> ParentTransactionId {
    ParentTransactionId::from_digest(Digest32::from_bytes([byte; 32]))
}

fn handoff(
    id: u8,
    parent_transaction_id: ParentTransactionId,
    ordinal: u32,
) -> AcceptedOwnerHandoffChronologyV1 {
    AcceptedOwnerHandoffChronologyV1 {
        id: openwepp_coupled_time::ReceiptId::from_digest(Digest32::from_bytes([id; 32])),
        parent_transaction_id,
        tick: ModelTimeNs::new(u128::from(ordinal)),
        ordinal,
        beginning_owner_sha256: Digest32::from_bytes([id.wrapping_add(1); 32]),
        ending_owner_sha256: Digest32::from_bytes([id.wrapping_add(2); 32]),
    }
}

#[test]
fn coupled_receipt_ordinals_are_parent_local_and_contiguous() {
    let mut next = BTreeMap::new();
    assert!(advance_canonical_terminal_event_ordinal(&mut next, parent(1), 0).is_ok());
    assert!(advance_canonical_terminal_event_ordinal(&mut next, parent(1), 1).is_ok());
    assert!(advance_canonical_terminal_event_ordinal(&mut next, parent(2), 0).is_ok());
}

#[test]
fn coupled_receipt_ordinal_duplicate_gap_and_nonzero_start_fail() {
    let mut next = BTreeMap::new();
    let beginning = next.clone();
    assert!(advance_canonical_terminal_event_ordinal(&mut next, parent(1), 1).is_err());
    assert_eq!(next, beginning, "failed first receipt must retain no state");
    assert!(advance_canonical_terminal_event_ordinal(&mut next, parent(2), 0).is_ok());
    let accepted = next.clone();
    assert!(advance_canonical_terminal_event_ordinal(&mut next, parent(2), 0).is_err());
    assert_eq!(next, accepted, "duplicate rejection must roll back exactly");
    assert!(advance_canonical_terminal_event_ordinal(&mut next, parent(2), 2).is_err());
    assert_eq!(next, accepted, "gap rejection must roll back exactly");
}

#[test]
fn terminal_then_post_support_receiver_ordinals_are_one_exact_sequence() {
    let mut next = BTreeMap::new();
    let transaction = parent(7);
    advance_canonical_terminal_event_ordinal(&mut next, transaction, 0)
        .expect("terminal event ordinal");
    advance_canonical_terminal_event_ordinal(&mut next, transaction, 1)
        .expect("post-support receiver ordinal");
    advance_canonical_terminal_event_ordinal(&mut next, transaction, 2)
        .expect("following terminal ordinal");
    let accepted = next.clone();
    assert!(advance_canonical_terminal_event_ordinal(&mut next, transaction, 2).is_err());
    assert_eq!(next, accepted, "duplicate must retain accepted sequence");

    let mut reordered = BTreeMap::new();
    assert!(advance_canonical_terminal_event_ordinal(&mut reordered, transaction, 1).is_err());
    assert!(reordered.is_empty(), "reorder must install no ordinal");
}

#[test]
fn terminal_group_uses_exact_post_support_clock_ordinal_and_rejects_skips() {
    assert_eq!(
        terminal_group_ordinal_after_physical_support_v1(2, 2)
            .expect("physical support without event"),
        2,
    );
    assert_eq!(
        terminal_group_ordinal_after_physical_support_v1(2, 3)
            .expect("same-tick support-liquid event precedes terminal group"),
        3,
    );
    assert!(
        terminal_group_ordinal_after_physical_support_v1(3, 2).is_err(),
        "rollback or stale post-support clock must fail closed",
    );
    assert!(
        terminal_group_ordinal_after_physical_support_v1(2, 4).is_err(),
        "unsealed event-ordinal skip must fail closed",
    );
    assert!(
        terminal_group_ordinal_after_physical_support_v1(u64::MAX, u32::MAX).is_err(),
        "ordinal overflow must fail closed",
    );
}

#[test]
fn complete_event_index_admits_terminal_subset_after_reappearance_and_rejects_poisons() {
    let transaction = parent(9);
    let reappearance = handoff(20, transaction, 0);
    let terminal = handoff(21, transaction, 1);
    let receiver = handoff(22, transaction, 2);
    let complete = canonical_accepted_event_index_v1(&[reappearance, terminal, receiver])
        .expect("complete reappearance/terminal/receiver chronology");
    assert_eq!(complete.get(&(transaction, 1)), Some(&terminal.id));
    assert_eq!(complete.get(&(transaction, 2)), Some(&receiver.id));

    assert!(
        canonical_accepted_event_index_v1(&[terminal, receiver]).is_err(),
        "omitted reappearance must not renumber the terminal subset",
    );
    assert!(
        canonical_accepted_event_index_v1(&[reappearance, receiver, terminal]).is_err(),
        "reordered terminal/receiver chronology",
    );
    let duplicate_id = AcceptedOwnerHandoffChronologyV1 {
        id: terminal.id,
        ..receiver
    };
    assert!(
        canonical_accepted_event_index_v1(&[reappearance, terminal, duplicate_id]).is_err(),
        "duplicate event receipt identity",
    );
}
