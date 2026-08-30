#[test]
fn qualification_adaptive_publication_crossjoin_rejects_child_poisons() {
    let expected = [
        qualification_adaptive_support(0, 5, 11),
        qualification_adaptive_support(5, 10, 12),
    ];
    let subslabs = [
        qualification_subslab(0, 5, 1, 11),
        qualification_subslab(5, 10, 2, 12),
    ];
    let publication = [
        qualification_retained_publication(0, 5, 1, 11),
        qualification_retained_publication(5, 10, 2, 12),
    ];
    validate_qualification_adaptive_publication_crossjoin_v1(
        &expected,
        &subslabs,
        &publication,
        &[],
    )
    .expect("sealed composed children in exact order");

    for poison in [
        vec![subslabs[0]],
        vec![subslabs[0], subslabs[0]],
        vec![subslabs[1], subslabs[0]],
        vec![subslabs[0], qualification_subslab(5, 10, 2, 13)],
        vec![
            qualification_subslab(0, 10, 3, 12),
            subslabs[0],
            subslabs[1],
        ],
    ] {
        assert!(
            validate_qualification_adaptive_publication_crossjoin_v1(
                &expected,
                &poison,
                &publication,
                &[],
            )
            .is_err(),
            "subslab omission, duplication, ordering, owner substitution, or rejected direct trial",
        );
    }
    for poison in [
        vec![publication[0].clone()],
        vec![publication[0].clone(), publication[0].clone()],
        vec![publication[1].clone(), publication[0].clone()],
        vec![
            publication[0].clone(),
            qualification_retained_publication(5, 10, 3, 12),
        ],
        vec![
            qualification_retained_publication(0, 10, 3, 12),
            publication[0].clone(),
            publication[1].clone(),
        ],
    ] {
        assert!(
            validate_qualification_adaptive_publication_crossjoin_v1(
                &expected,
                &subslabs,
                &poison,
                &[],
            )
            .is_err(),
            "publication omission, duplication, ordering, owner substitution, or rejected direct trial",
        );
    }
}

#[test]
fn qualification_rejects_successor_crossing_the_sealed_terminal_child() {
    const SECOND_NS: u128 = 1_000_000_000;
    let expected = [qualification_adaptive_support_with_posture(
        420 * SECOND_NS,
        900 * SECOND_NS,
        12,
        Stage3AdaptiveEventPostureV1::TerminalEvent,
    )];
    let subslabs = [qualification_subslab_with_terminal(
        420 * SECOND_NS,
        540 * SECOND_NS,
        2,
        12,
        true,
    )];
    let publication = [qualification_retained_publication(
        420 * SECOND_NS,
        540 * SECOND_NS,
        2,
        12,
    )];
    let crossing = [
        qualification_snow_free_successor(540 * SECOND_NS, 600 * SECOND_NS),
        qualification_snow_free_successor(600 * SECOND_NS, 1_800 * SECOND_NS),
    ];
    assert!(
        validate_qualification_adaptive_publication_crossjoin_v1(
            &expected,
            &subslabs,
            &publication,
            &crossing,
        )
        .is_err(),
        "a successor crossing the sealed 900-second child end remains fail-closed",
    );
}
