#[test]
fn qualification_accumulator_stream_reconstruction_is_exact_and_bounded() {
    let mut days = Vec::new();
    let mut beginning = qualification_endpoint(0, 201, false);
    for day_index in 0..365 {
        let ending_identity = u8::try_from((day_index % 53) + 1).expect("bounded identity");
        let day = qualification_day_delta(day_index, beginning, ending_identity);
        beginning = day.ending_owner.clone();
        days.push(day);
    }
    let first = SnowStage3V11QualificationAccumulatorV1::reconstruct_from_days(&days[..1])
        .expect("one-day accumulator");
    let season = SnowStage3V11QualificationAccumulatorV1::reconstruct_from_days(&days)
        .expect("season accumulator");
    season
        .validate_stream_reconstruction(&days)
        .expect("archive stream reconstructs resident accumulator");
    assert_eq!(season.committed_day_count, 365);
    assert_eq!(season.accepted_support_receipts.record_count, 365 * 48);
    let first_bytes = serde_json::to_vec(&first).expect("serialize one-day accumulator");
    let season_bytes = serde_json::to_vec(&season).expect("serialize season accumulator");
    assert!(
        season_bytes.len() <= first_bytes.len() + 128,
        "resident qualification bytes must be independent of day/support count: one={} season={}",
        first_bytes.len(),
        season_bytes.len(),
    );
}

#[test]
fn qualification_accumulator_omission_substitution_and_order_poisons_fail_closed() {
    let day0 = qualification_day_delta(0, qualification_endpoint(0, 201, false), 1);
    let day1 = qualification_day_delta(1, day0.ending_owner.clone(), 2);
    let expected = SnowStage3V11QualificationAccumulatorV1::reconstruct_from_days([&day0, &day1])
        .expect("two-day accumulator");

    assert!(expected.validate_stream_reconstruction([&day0]).is_err());
    assert!(
        SnowStage3V11QualificationAccumulatorV1::reconstruct_from_days([&day1, &day0]).is_err()
    );

    let mut substituted = day1.clone();
    substituted.accepted_support_receipt_sha256s[7] = digest(250);
    substituted = substituted.seal().expect("seal substituted stream");
    assert!(
        expected
            .validate_stream_reconstruction([&day0, &substituted])
            .is_err()
    );

    let mut duplicate = day1.clone();
    duplicate.accepted_support_receipt_sha256s[7] =
        duplicate.accepted_support_receipt_sha256s[6];
    duplicate.receipt_sha256 = duplicate.reconstructed_digest().expect("reseal duplicate");
    assert!(duplicate.validate().is_err());
}

#[test]
fn qualification_accumulator_failed_fold_preserves_exact_beginning_bytes() {
    let day0 = qualification_day_delta(0, qualification_endpoint(0, 201, false), 1);
    let mut accumulator = SnowStage3V11QualificationAccumulatorV1::default();
    accumulator.fold_day(&day0).expect("first day");
    let before = serde_json::to_vec(&accumulator).expect("beginning bytes");
    let mut poison = qualification_day_delta(1, day0.ending_owner.clone(), 2);
    poison.beginning_owner.coupled_owner_set_sha256 = digest(99);
    poison.receipt_sha256 = poison.reconstructed_digest().expect("reseal poison");
    accumulator
        .fold_day(&poison)
        .expect_err("owner substitution must reject");
    assert_eq!(
        serde_json::to_vec(&accumulator).expect("ending bytes"),
        before,
    );
}
