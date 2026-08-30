fn qualification_surface_occurrence(
    accepted_support_receipt_sha256: Digest32,
    start_s: u128,
    end_s: u128,
    source_receipt_ordinal: usize,
    source_receipt_sha256: Digest32,
) -> SnowStage3V11QualificationSurfaceReceiptOccurrenceV1 {
    SnowStage3V11QualificationSurfaceReceiptOccurrenceV1::try_new(
        accepted_support_receipt_sha256,
        start_s * 1_000_000_000,
        end_s * 1_000_000_000,
        0,
        source_receipt_ordinal,
        source_receipt_sha256,
    )
    .expect("sealed surface receipt occurrence")
}

fn qualification_day_with_repeated_support_relative_surface_receipt()
-> SnowStage3V11QualificationDayDeltaV1 {
    let mut value = qualification_day_delta(0, qualification_endpoint(0, 201, false), 1);
    let repeated_raw_receipt = digest(81);
    value.surface_receipt_occurrences = vec![
        qualification_surface_occurrence(digest(71), 420, 480, 0, repeated_raw_receipt),
        qualification_surface_occurrence(digest(72), 480, 540, 0, repeated_raw_receipt),
    ];
    value.receipt_sha256 = Digest32::zero();
    value.seal().expect("seal real-shape repeated raw receipt")
}

#[test]
fn qualification_surface_identity_counts_repeated_parent_receipt_per_support_occurrence() {
    let value = qualification_day_with_repeated_support_relative_surface_receipt();
    let first = value.surface_receipt_occurrences[0];
    let second = value.surface_receipt_occurrences[1];
    assert_eq!(first.source_receipt_sha256, second.source_receipt_sha256);
    assert_ne!(
        first.accepted_support_receipt_sha256,
        second.accepted_support_receipt_sha256
    );
    assert_ne!(first.receipt_sha256, second.receipt_sha256);

    let bytes = serde_json::to_vec(&value).expect("serialize nonempty occurrence delta");
    let restored: SnowStage3V11QualificationDayDeltaV1 =
        serde_json::from_slice(&bytes).expect("deserialize nonempty occurrence delta");
    assert_eq!(restored, value);
    restored.validate().expect("roundtrip validates");

    let accumulator = SnowStage3V11QualificationAccumulatorV1::reconstruct_from_days([&value])
        .expect("fold repeated receipt occurrences");
    assert_eq!(accumulator.surface_receipts.record_count, 2);
    accumulator.validate().expect("bounded root remains sealed");
}

#[test]
fn qualification_surface_identity_zero_duplicate_order_and_substitution_fail_closed() {
    let value = qualification_day_with_repeated_support_relative_surface_receipt();

    let mut zero = value.clone();
    zero.surface_receipt_occurrences[0].source_receipt_sha256 = Digest32::zero();
    zero.receipt_sha256 = zero
        .reconstructed_digest()
        .expect("reseal zero day envelope");
    assert!(matches!(
        zero.validate(),
        Err(DirectSnowStage3V11AttachmentError::QualificationOrderedRecordIdentity {
            vector: "surface_receipt_occurrences",
            failure: "invalid occurrence",
            first_index: 0,
            source_receipt_sha256: Some(digest),
            ..
        }) if digest == Digest32::zero()
    ));

    let mut duplicate = value.clone();
    duplicate.surface_receipt_occurrences = vec![
        qualification_surface_occurrence(digest(71), 420, 480, 0, digest(81)),
        qualification_surface_occurrence(digest(71), 420, 480, 1, digest(81)),
    ];
    duplicate.receipt_sha256 = duplicate
        .reconstructed_digest()
        .expect("reseal duplicate day envelope");
    assert!(matches!(
        duplicate.validate(),
        Err(DirectSnowStage3V11AttachmentError::QualificationOrderedRecordIdentity {
            vector: "surface_receipt_occurrences",
            failure: "duplicate custody",
            first_index: 0,
            duplicate_index: Some(1),
            source_support_receipt_sha256: Some(support),
            source_receipt_sha256: Some(receipt),
            ..
        }) if support == digest(71) && receipt == digest(81)
    ));

    let mut reordered = value.clone();
    reordered.surface_receipt_occurrences.reverse();
    reordered.receipt_sha256 = reordered
        .reconstructed_digest()
        .expect("reseal reordered day envelope");
    assert!(matches!(
        reordered.validate(),
        Err(
            DirectSnowStage3V11AttachmentError::QualificationOrderedRecordIdentity {
                vector: "surface_receipt_occurrences",
                failure: "noncanonical order",
                first_index: 0,
                duplicate_index: Some(1),
                ..
            }
        )
    ));

    let mut substituted = value;
    substituted.surface_receipt_occurrences[1].accepted_support_receipt_sha256 = digest(99);
    substituted.receipt_sha256 = substituted
        .reconstructed_digest()
        .expect("reseal substituted day envelope");
    assert!(matches!(
        substituted.validate(),
        Err(DirectSnowStage3V11AttachmentError::QualificationOrderedRecordIdentity {
            vector: "surface_receipt_occurrences",
            failure: "invalid occurrence",
            first_index: 1,
            source_support_receipt_sha256: Some(support),
            source_receipt_sha256: Some(receipt),
            ..
        }) if support == digest(99) && receipt == digest(81)
    ));
}
