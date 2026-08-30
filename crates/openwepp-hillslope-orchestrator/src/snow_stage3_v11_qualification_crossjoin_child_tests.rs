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
