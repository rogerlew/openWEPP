#[test]
fn qualification_adaptive_subset_excludes_same_parent_snow_free_successors_and_rejects_poisons() {
    let expected = [
        qualification_adaptive_support(0, 5, 11),
        qualification_adaptive_support(5, 10, 12),
    ];
    let subslabs = [
        qualification_subslab(0, 5, 1, 11),
        qualification_subslab(5, 10, 2, 12),
    ];
    let first = qualification_retained_publication(0, 5, 1, 11);
    let second = qualification_retained_publication(5, 10, 2, 12);
    let successor = qualification_retained_publication(10, 20, 3, 3);
    let exact = qualification_exact_adaptive_publication_subset(
        &subslabs,
        &[first.clone(), second.clone(), successor],
    );
    assert_eq!(exact, [first.clone(), second.clone()]);
    validate_qualification_adaptive_publication_crossjoin_v1(&expected, &subslabs, &exact, &[])
        .expect("same-parent snow-free successor is outside the adaptive temporal subset");
    let mut with_later_downstream_event = exact.clone();
    with_later_downstream_event[1]
        .ordered_owner_chain_sha256s
        .push(digest(14));
    validate_qualification_adaptive_publication_crossjoin_v1(
        &expected,
        &subslabs,
        &with_later_downstream_event,
        &[],
    )
    .expect("adaptive ending is an authenticated intermediate event-chain checkpoint");

    for poison in [
        vec![first.clone()],
        vec![first.clone(), first.clone(), second.clone()],
        vec![second.clone(), first.clone()],
        vec![
            first.clone(),
            qualification_retained_publication(5, 10, 4, 12),
        ],
        vec![
            first.clone(),
            qualification_retained_publication(5, 10, 2, 13),
        ],
    ] {
        let poison = qualification_exact_adaptive_publication_subset(&subslabs, &poison);
        assert!(
            validate_qualification_adaptive_publication_crossjoin_v1(
                &expected,
                &subslabs,
                &poison,
                &[],
            )
            .is_err(),
            "omission, duplication, ordering, physical-owner substitution, and event-tail substitution remain fail-closed",
        );
    }
}

#[test]
fn qualification_adaptive_envelopes_bind_ordered_exact_replay_partitions() {
    let expected = [
        qualification_adaptive_support(0, 900, 11),
        qualification_adaptive_support(900, 1_800, 12),
    ];
    let subslabs = [
        qualification_subslab(0, 60, 1, 2),
        qualification_subslab(60, 900, 3, 11),
        qualification_subslab(900, 960, 4, 5),
        qualification_subslab(960, 1_800, 6, 12),
    ];
    let publication = [
        qualification_retained_publication(0, 60, 1, 2),
        qualification_retained_publication(60, 900, 3, 11),
        qualification_retained_publication(900, 960, 4, 5),
        qualification_retained_publication(960, 1_800, 6, 12),
    ];
    validate_qualification_adaptive_publication_crossjoin_v1(
        &expected,
        &subslabs,
        &publication,
        &[],
    )
    .expect("unpublished discovery may yield ordered exact replay partitions inside envelopes");

    for poison in [
        subslabs[..3].to_vec(),
        vec![subslabs[0], subslabs[2], subslabs[1], subslabs[3]],
        vec![
            subslabs[0],
            subslabs[1],
            subslabs[1],
            subslabs[2],
            subslabs[3],
        ],
    ] {
        let poison_publication = qualification_exact_adaptive_publication_subset(
            &poison,
            &publication,
        );
        assert!(
            validate_qualification_adaptive_publication_crossjoin_v1(
                &expected,
                &poison,
                &poison_publication,
                &[],
            )
            .is_err(),
            "omitted, reordered, and duplicated replay partitions fail closed",
        );
    }
}

#[test]
fn qualification_terminal_child_requires_exact_physical_and_successor_partition() {
    const SECOND_NS: u128 = 1_000_000_000;
    let expected = [
        qualification_adaptive_support(0, 420 * SECOND_NS, 11),
        qualification_adaptive_support_with_posture(
            420 * SECOND_NS,
            900 * SECOND_NS,
            12,
            Stage3AdaptiveEventPostureV1::TerminalEvent,
        ),
    ];
    let subslabs = [
        qualification_subslab(0, 420 * SECOND_NS, 1, 11),
        qualification_subslab_with_terminal(420 * SECOND_NS, 840 * SECOND_NS, 2, 12, true),
    ];
    let publication = [
        qualification_retained_publication(0, 420 * SECOND_NS, 1, 11),
        qualification_retained_publication(420 * SECOND_NS, 840 * SECOND_NS, 2, 12),
    ];
    let successor = [qualification_snow_free_successor(
        840 * SECOND_NS,
        900 * SECOND_NS,
    )];
    validate_qualification_adaptive_publication_crossjoin_v1(
        &expected,
        &subslabs,
        &publication,
        &successor,
    )
    .expect("terminal physical prefix plus sealed snow-free tail exactly tiles child envelope");
    let same_parent_tail = [
        successor[0],
        qualification_snow_free_successor(900 * SECOND_NS, 1_800 * SECOND_NS),
    ];
    validate_qualification_adaptive_publication_crossjoin_v1(
        &expected,
        &subslabs,
        &publication,
        &same_parent_tail,
    )
    .expect("same-parent successor beginning at the envelope boundary belongs to the parent tail");

    let mut no_terminal = subslabs;
    no_terminal[1].terminal_event_at_support_end = false;
    assert!(
        validate_qualification_adaptive_publication_crossjoin_v1(
            &expected,
            &no_terminal,
            &publication,
            &successor,
        )
        .is_err(),
        "a truncated physical prefix requires a terminal event at its ending tick",
    );

    let mut no_event_expected = expected;
    no_event_expected[1].event_posture = Stage3AdaptiveEventPostureV1::NoEvent;
    assert!(
        validate_qualification_adaptive_publication_crossjoin_v1(
            &no_event_expected,
            &subslabs,
            &publication,
            &successor,
        )
        .is_err(),
        "non-event adaptive children retain exact physical-support equality",
    );

    let mut wrong_parent = successor[0];
    wrong_parent.parent_transaction_sha256 = digest(91);
    for poison in [
        Vec::new(),
        vec![qualification_snow_free_successor(
            839 * SECOND_NS,
            900 * SECOND_NS,
        )],
        vec![qualification_snow_free_successor(
            841 * SECOND_NS,
            900 * SECOND_NS,
        )],
        vec![qualification_snow_free_successor(
            840 * SECOND_NS,
            899 * SECOND_NS,
        )],
        vec![qualification_snow_free_successor(
            840 * SECOND_NS,
            901 * SECOND_NS,
        )],
        vec![wrong_parent],
        vec![successor[0], successor[0]],
    ] {
        assert!(
            validate_qualification_adaptive_publication_crossjoin_v1(
                &expected,
                &subslabs,
                &publication,
                &poison,
            )
            .is_err(),
            "missing, overlapping, gapped, short, long, wrong-parent, and duplicate tails fail closed",
        );
    }
}
