use super::*;

fn digest(byte: u8) -> Digest32 {
    Digest32::from_bytes([byte; 32])
}

fn discrete_surfaces(byte: u8) -> Vec<Stage3AdaptiveDiscreteSurfaceReceiptV1> {
    let mut surfaces = vec![
        Stage3AdaptiveDiscreteSurfaceReceiptV1 {
            owner_id: "complete_owner".to_owned(),
            path: "adaptive_scalars.cardinality".to_owned(),
            kind: "membership".to_owned(),
            exact_value: "0".to_owned(),
        },
        Stage3AdaptiveDiscreteSurfaceReceiptV1 {
            owner_id: "complete_owner".to_owned(),
            path: "adaptive_scalars.ordered_identity_set_sha256".to_owned(),
            kind: "ordering".to_owned(),
            exact_value: format!("0:{byte:064x}"),
        },
        Stage3AdaptiveDiscreteSurfaceReceiptV1 {
            owner_id: "snow".to_owned(),
            path: "lanes[0].posture".to_owned(),
            kind: "posture".to_owned(),
            exact_value: byte.to_string(),
        },
        Stage3AdaptiveDiscreteSurfaceReceiptV1 {
            owner_id: "snow".to_owned(),
            path: "pending_terminal_parcels.cardinality".to_owned(),
            kind: "membership".to_owned(),
            exact_value: "0".to_owned(),
        },
        Stage3AdaptiveDiscreteSurfaceReceiptV1 {
            owner_id: "snow".to_owned(),
            path: "pending_terminal_parcels.ordered_identity_set_sha256".to_owned(),
            kind: "ordering".to_owned(),
            exact_value: format!("0:{byte:064x}"),
        },
    ];
    surfaces.sort_by(|left, right| {
        (
            left.owner_id.as_str(),
            left.path.as_str(),
            left.kind.as_str(),
            left.exact_value.as_str(),
        )
            .cmp(&(
                right.owner_id.as_str(),
                right.path.as_str(),
                right.kind.as_str(),
                right.exact_value.as_str(),
            ))
    });
    surfaces
}

fn support(start: u128, end: u128) -> TimeSupport {
    TimeSupport::new(ModelTimeNs::new(start), ModelTimeNs::new(end))
        .expect("valid adaptive test support")
}

fn context(attempt_ordinal: u32, step_support: TimeSupport) -> Stage3AdaptiveReceiptContextV1 {
    Stage3AdaptiveReceiptContextV1 {
        parent_transaction_id: ParentTransactionId::from_digest(digest(1)),
        parent_support: support(0, 180_000_000_000),
        step_support,
        step_ordinal: 4,
        attempt_ordinal,
        beginning_complete_owner_set_sha256: digest(2),
        forcing_projection_sha256: digest(3),
        topology_sha256: digest(4),
        configuration_sha256: digest(5),
    }
}

fn direct(request: &Stage3AdaptiveParentRequestReceiptV1) -> Stage3AdaptiveDirectTrialReceiptV1 {
    Stage3AdaptiveDirectTrialReceiptV1::try_new(
        request,
        digest(6),
        digest(7),
        digest(8),
        Stage3AdaptiveEventPostureV1::NoEvent,
        Stage3AdaptiveTrialDispositionV1::Closed,
    )
    .expect("sealed direct trial")
}

fn composed_chain() -> (
    Stage3AdaptiveParentRequestReceiptV1,
    Stage3AdaptiveDirectTrialReceiptV1,
    Stage3AdaptiveSplitChildTrialReceiptV1,
    Stage3AdaptiveSplitChildTrialReceiptV1,
    Stage3AdaptiveStepComparisonReceiptV1,
) {
    let request =
        Stage3AdaptiveParentRequestReceiptV1::try_new(context(0, support(0, 180_000_000_000)), 3)
            .expect("sealed request");
    let direct = direct(&request);
    let child_1 = Stage3AdaptiveSplitChildTrialReceiptV1::try_child_1(
        &request,
        &direct,
        support(0, STAGE3_ADAPTIVE_MINIMUM_STEP_NS),
        digest(9),
        digest(10),
        digest(11),
        Stage3AdaptiveEventPostureV1::NoEvent,
        Stage3AdaptiveTrialDispositionV1::Closed,
    )
    .expect("sealed child 1");
    let child_2 = Stage3AdaptiveSplitChildTrialReceiptV1::try_child_2(
        &request,
        &child_1,
        support(STAGE3_ADAPTIVE_MINIMUM_STEP_NS, 180_000_000_000),
        digest(12),
        digest(13),
        digest(14),
        Stage3AdaptiveEventPostureV1::TerminalEvent,
        Stage3AdaptiveTrialDispositionV1::Closed,
    )
    .expect("sealed child 2");
    let comparison = Stage3AdaptiveStepComparisonReceiptV1::try_composed(
        &request,
        &direct,
        &child_1,
        &child_2,
        digest(15),
        digest(16),
        digest(17),
        discrete_surfaces(1),
        discrete_surfaces(2),
        0.25,
        false,
        true,
    )
    .expect("sealed comparison");
    (request, direct, child_1, child_2, comparison)
}

#[test]
fn composed_receipt_chain_is_acyclic_and_installs_the_split_endpoint() {
    let (request, direct, child_1, child_2, comparison) = composed_chain();
    let accepted = Stage3AdaptiveAcceptedMicrostepReceiptV1::try_new(&comparison)
        .expect("accepted composed step");

    request.validate().expect("request");
    direct.validate_against(&request).expect("direct");
    child_1.validate().expect("child 1");
    child_2.validate().expect("child 2");
    accepted
        .validate_against(&comparison)
        .expect("accepted lineage");
    assert_eq!(child_1.predecessor_receipt_sha256, direct.receipt_sha256);
    assert_eq!(child_2.predecessor_receipt_sha256, child_1.receipt_sha256);
    assert_eq!(
        comparison.predecessor_receipt_sha256,
        child_2.receipt_sha256
    );
    assert_eq!(
        accepted.comparison_receipt_sha256,
        comparison.receipt_sha256
    );
    assert_eq!(
        accepted.ending_complete_owner_set_sha256,
        child_2.ending_complete_owner_set_sha256
    );
    assert_ne!(
        accepted.ending_complete_owner_set_sha256, direct.ending_complete_owner_set_sha256,
        "accepted path must install the composed endpoint"
    );
}

#[test]
fn discrete_rejection_retains_a_finite_physical_error_and_rejects_infinity() {
    let (request, direct, child_1, child_2, _) = composed_chain();
    let rejected = Stage3AdaptiveStepComparisonReceiptV1::try_composed(
        &request,
        &direct,
        &child_1,
        &child_2,
        digest(15),
        digest(16),
        digest(17),
        discrete_surfaces(1),
        discrete_surfaces(2),
        0.25,
        true,
        false,
    )
    .expect("finite physical error plus discrete mismatch");
    rejected.validate().expect("sealed discrete rejection");
    assert_eq!(
        rejected.decision,
        Stage3AdaptiveStepDecisionV1::RefineRejected
    );
    assert!(rejected.maximum_scaled_error.is_finite());
    assert!(rejected.discrete_mismatch);

    assert!(
        Stage3AdaptiveStepComparisonReceiptV1::try_composed(
            &request,
            &direct,
            &child_1,
            &child_2,
            digest(15),
            digest(16),
            digest(17),
            discrete_surfaces(1),
            discrete_surfaces(2),
            f64::INFINITY,
            true,
            false,
        )
        .is_err(),
        "non-finite physical diagnostics must remain fail closed",
    );
}

#[test]
fn comparison_receipt_seals_pending_parcel_cardinality_identity_and_order() {
    let (_, _, _, _, comparison) = composed_chain();
    let add_pending = |mut surfaces: Vec<Stage3AdaptiveDiscreteSurfaceReceiptV1>, byte| {
        surfaces
            .iter_mut()
            .find(|surface| surface.path == "pending_terminal_parcels.cardinality")
            .expect("pending cardinality")
            .exact_value = "1".to_owned();
        surfaces
            .iter_mut()
            .find(|surface| surface.path == "pending_terminal_parcels.ordered_identity_set_sha256")
            .expect("pending ordered identity set")
            .exact_value = format!("1:{byte:064x}");
        surfaces.sort_by(|left, right| {
            (
                left.owner_id.as_str(),
                left.path.as_str(),
                left.kind.as_str(),
                left.exact_value.as_str(),
            )
                .cmp(&(
                    right.owner_id.as_str(),
                    right.path.as_str(),
                    right.kind.as_str(),
                    right.exact_value.as_str(),
                ))
        });
        surfaces
    };
    let mut sealed = comparison;
    sealed.direct_exact_discrete_surfaces = add_pending(sealed.direct_exact_discrete_surfaces, 1);
    sealed.composed_exact_discrete_surfaces =
        add_pending(sealed.composed_exact_discrete_surfaces, 2);
    sealed = Stage3AdaptiveStepComparisonReceiptV1::seal(sealed)
        .expect("explicit pending parcel authority");
    sealed.validate().expect("sealed pending parcel authority");

    let mut omission = sealed.clone();
    omission
        .direct_exact_discrete_surfaces
        .retain(|surface| surface.path != "pending_terminal_parcels.ordered_identity_set_sha256");
    assert!(omission.reconstructed_digest().is_err());

    let mut cardinality = sealed.clone();
    cardinality
        .direct_exact_discrete_surfaces
        .iter_mut()
        .find(|surface| surface.path == "pending_terminal_parcels.cardinality")
        .expect("pending cardinality")
        .exact_value = "2".to_owned();
    assert!(cardinality.reconstructed_digest().is_err());

    let mut identity_set = sealed.clone();
    identity_set
        .direct_exact_discrete_surfaces
        .iter_mut()
        .find(|surface| surface.path == "pending_terminal_parcels.ordered_identity_set_sha256")
        .expect("pending ordered identity set")
        .exact_value = "not-a-canonical-digest".to_owned();
    assert!(identity_set.reconstructed_digest().is_err());

    let mut order = sealed;
    order.direct_exact_discrete_surfaces.swap(0, 1);
    assert!(order.reconstructed_digest().is_err());
}

#[test]
fn receipt_chain_rejects_identity_support_owner_phase_event_and_order_poisons() {
    let (request, direct, child_1, child_2, comparison) = composed_chain();

    let mut forcing = request.clone();
    forcing.context.forcing_projection_sha256 = digest(90);
    assert!(forcing.validate().is_err());

    let mut topology = direct.clone();
    topology.context.topology_sha256 = digest(91);
    assert!(topology.validate_against(&request).is_err());

    let mut phase = child_1.clone();
    phase.phase_result_sha256 = digest(92);
    assert!(phase.validate().is_err());

    let mut event = child_2.clone();
    event.event_posture = Stage3AdaptiveEventPostureV1::PendingParcel;
    assert!(event.validate().is_err());

    let mut support_poison = child_2.clone();
    support_poison.child_support = support(120_000_000_000, 180_000_000_000);
    assert!(support_poison.validate().is_err());

    let mut owner_poison = child_2.clone();
    owner_poison.trial_beginning_complete_owner_set_sha256 = digest(93);
    owner_poison.receipt_sha256 = owner_poison
        .reconstructed_digest()
        .expect("resealed owner poison");
    assert!(
        Stage3AdaptiveStepComparisonReceiptV1::try_composed(
            &request,
            &direct,
            &child_1,
            &owner_poison,
            digest(15),
            digest(16),
            digest(17),
            discrete_surfaces(1),
            discrete_surfaces(2),
            0.25,
            false,
            true,
        )
        .is_err()
    );

    assert!(
        Stage3AdaptiveStepComparisonReceiptV1::try_composed(
            &request,
            &direct,
            &child_2,
            &child_1,
            digest(15),
            digest(16),
            digest(17),
            discrete_surfaces(1),
            discrete_surfaces(2),
            0.25,
            false,
            true,
        )
        .is_err()
    );

    let mut substitution = comparison.clone();
    substitution.selected_ending_complete_owner_set_sha256 =
        direct.ending_complete_owner_set_sha256;
    assert!(substitution.validate().is_err());

    let mut tolerance_substitution = comparison;
    tolerance_substitution.tolerance_policy_id = "SUBSTITUTED_POLICY".to_owned();
    assert!(tolerance_substitution.validate().is_err());
}

#[test]
fn floor_decisions_have_one_trial_and_cannot_fabricate_split_children() {
    let request = Stage3AdaptiveParentRequestReceiptV1::try_new(
        context(0, support(0, STAGE3_ADAPTIVE_MINIMUM_STEP_NS)),
        1,
    )
    .expect("floor request");
    let floor_direct = direct(&request);
    let accepted_comparison = Stage3AdaptiveStepComparisonReceiptV1::try_floor(
        &request,
        &floor_direct,
        digest(16),
        discrete_surfaces(1),
        true,
    )
    .expect("floor accepted");
    Stage3AdaptiveAcceptedMicrostepReceiptV1::try_new(&accepted_comparison)
        .expect("accepted floor microstep");

    let rejected_direct = Stage3AdaptiveDirectTrialReceiptV1::try_new(
        &request,
        digest(6),
        digest(7),
        digest(8),
        Stage3AdaptiveEventPostureV1::NoEvent,
        Stage3AdaptiveTrialDispositionV1::TypedRejected,
    )
    .expect("rejected floor trial");
    let rejected_comparison = Stage3AdaptiveStepComparisonReceiptV1::try_floor(
        &request,
        &rejected_direct,
        digest(16),
        discrete_surfaces(1),
        false,
    )
    .expect("floor rejected");
    assert!(Stage3AdaptiveAcceptedMicrostepReceiptV1::try_new(&rejected_comparison).is_err());

    let mut fabricated = accepted_comparison;
    fabricated.split_child_1_sha256 = Some(digest(94));
    assert!(fabricated.validate().is_err());

    let non_floor =
        Stage3AdaptiveParentRequestReceiptV1::try_new(context(0, support(0, 120_000_000_000)), 2)
            .expect("non-floor request");
    assert!(
        Stage3AdaptiveStepComparisonReceiptV1::try_floor(
            &non_floor,
            &direct(&non_floor),
            digest(16),
            discrete_surfaces(1),
            true,
        )
        .is_err()
    );
}

#[test]
fn attempt_identity_is_diagnostic_and_does_not_change_physical_endpoint() {
    let request_0 = Stage3AdaptiveParentRequestReceiptV1::try_new(
        context(0, support(0, STAGE3_ADAPTIVE_MINIMUM_STEP_NS)),
        1,
    )
    .expect("attempt 0");
    let request_7 = Stage3AdaptiveParentRequestReceiptV1::try_new(
        context(7, support(0, STAGE3_ADAPTIVE_MINIMUM_STEP_NS)),
        1,
    )
    .expect("attempt 7");
    let direct_0 = direct(&request_0);
    let direct_7 = direct(&request_7);
    assert_ne!(request_0.receipt_sha256, request_7.receipt_sha256);
    assert_ne!(direct_0.receipt_sha256, direct_7.receipt_sha256);
    assert_eq!(
        direct_0.ending_complete_owner_set_sha256,
        direct_7.ending_complete_owner_set_sha256
    );
    assert_eq!(
        direct_0.physical_ledger_sha256,
        direct_7.physical_ledger_sha256
    );
}

#[test]
fn canonical_receipt_sets_reject_duplicate_and_reordered_requests() {
    let request_0 = Stage3AdaptiveParentRequestReceiptV1::try_new(
        context(0, support(0, STAGE3_ADAPTIVE_MINIMUM_STEP_NS)),
        1,
    )
    .expect("attempt 0");
    let request_1 = Stage3AdaptiveParentRequestReceiptV1::try_new(
        context(1, support(0, STAGE3_ADAPTIVE_MINIMUM_STEP_NS)),
        1,
    )
    .expect("attempt 1");
    let complete =
        stage3_adaptive_parent_request_set_sha256_v1(&[request_0.clone(), request_1.clone()])
            .expect("ordered request set");
    let omitted = stage3_adaptive_parent_request_set_sha256_v1(&[request_0.clone()])
        .expect("short request set");
    assert_ne!(complete, omitted, "omission must change the set identity");
    assert!(
        stage3_adaptive_parent_request_set_sha256_v1(&[request_0.clone(), request_0.clone(),])
            .is_err()
    );
    assert!(stage3_adaptive_parent_request_set_sha256_v1(&[request_1, request_0]).is_err());

    let (_, _, _, _, comparison) = composed_chain();
    let accepted =
        Stage3AdaptiveAcceptedMicrostepReceiptV1::try_new(&comparison).expect("accepted receipt");
    assert_ne!(
        stage3_adaptive_accepted_microstep_set_sha256_v1(&[]).expect("empty accepted set"),
        stage3_adaptive_accepted_microstep_set_sha256_v1(&[accepted])
            .expect("one accepted receipt")
    );
}

fn support_receipt() -> Stage3AdaptiveSupportReceiptV1 {
    let (request, direct, child_1, child_2, comparison) = composed_chain();
    let accepted =
        Stage3AdaptiveAcceptedMicrostepReceiptV1::try_new(&comparison).expect("accepted receipt");
    let parent_request_set_sha256 =
        stage3_adaptive_parent_request_set_sha256_v1(std::slice::from_ref(&request))
            .expect("request set");
    let accepted_microstep_set_sha256 =
        stage3_adaptive_accepted_microstep_set_sha256_v1(std::slice::from_ref(&accepted))
            .expect("accepted set");
    Stage3AdaptiveSupportReceiptV1 {
        parent_transaction_id: request.context.parent_transaction_id,
        parent_support: request.context.parent_support,
        parent_requests: vec![request],
        direct_trials: vec![direct],
        split_child_trials: vec![child_1, child_2],
        comparisons: vec![comparison],
        accepted_microsteps: vec![accepted],
        parent_request_set_sha256,
        accepted_microstep_set_sha256,
    }
}

#[test]
fn controller_telemetry_is_derived_from_authoritative_receipt_vectors() {
    let receipt = support_receipt();
    receipt.validate().expect("support receipt");
    let telemetry = receipt
        .transient_diagnostics()
        .expect("derived controller telemetry");
    assert_eq!(telemetry.direct_trial_count, 1);
    assert_eq!(telemetry.split_child_trial_count, 2);
    assert_eq!(telemetry.accepted_microstep_count, 1);
    assert_eq!(telemetry.rejected_candidate_count, 0);
    assert_eq!(telemetry.minimum_accepted_step_ns, Some(180_000_000_000));
    assert_eq!(telemetry.maximum_accepted_step_ns, Some(180_000_000_000));
}

#[test]
fn support_receipt_serialization_omits_diagnostics_and_rejects_unknown_diagnostic_field() {
    let receipt = support_receipt();
    let mut value = serde_json::to_value(&receipt).expect("support receipt JSON");
    let object = value.as_object_mut().expect("support receipt object");
    for key in [
        "diagnostics",
        "direct_trial_count",
        "split_child_trial_count",
        "accepted_microstep_count",
        "rejected_candidate_count",
        "minimum_accepted_step_ns",
        "maximum_accepted_step_ns",
        "phase_refinement_count",
        "event_refinement_count",
        "owner_evaluation_counts",
    ] {
        assert!(!object.contains_key(key), "persisted diagnostic key: {key}");
    }

    object.insert("diagnostics".to_owned(), serde_json::json!({}));
    assert!(serde_json::from_value::<Stage3AdaptiveSupportReceiptV1>(value).is_err());
}
