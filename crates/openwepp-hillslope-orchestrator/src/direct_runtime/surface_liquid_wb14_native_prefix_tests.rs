#[test]
fn native_inactive_prefix_starts_first_physical_child_without_wb14_receipt() {
    let beginning = parent_fixture();
    let authority = beginning.authority;
    let cursor = beginning.beginning_cursor;
    let prefix_end_ns = authority.support_start_ns + 900_000_000_000;
    let proof = ValidatedNativeInactiveWb14PrefixV1::try_new(
        authority.coupled_parent_transaction_sha256,
        authority.parent_beginning_owner_sha256,
        [72; 32],
        authority.support_start_ns,
        prefix_end_ns,
        authority.support_end_ns,
        [71; 32],
    )
    .expect("complete coupled native inactive prefix");
    let proof_sha256 = proof.proof_sha256;
    let prefixed =
        DirectWb14ParentIntervalV1::begin_after_native_inactive_prefix(authority, cursor, proof)
            .expect("parent after inactive prefix");
    assert_eq!(prefixed.working().accepted_until_ns, prefix_end_ns);
    assert_eq!(prefixed.working().next_child_ordinal, 0);
    assert_eq!(
        prefixed.working().cumulative_supply_m.to_bits(),
        0.002_f64.to_bits()
    );
    assert_eq!(
        prefixed.working().cumulative_infiltration_m.to_bits(),
        0.001_f64.to_bits()
    );
    assert!(prefixed.receipts().is_empty());
    prefixed.validate().expect("prefix-backed parent replay");

    let restart_bytes = serde_json::to_vec(&prefixed).expect("split parent restart bytes");
    let restarted: DirectWb14ParentIntervalV1 =
        serde_json::from_slice(&restart_bytes).expect("split parent restart");
    restarted.validate().expect("restarted split parent");
    assert_eq!(
        serde_json::to_vec(&restarted).expect("restarted split parent bytes"),
        restart_bytes,
        "the complete split parent, including inactive proof and untouched cumulatives, restarts byte exactly"
    );

    let completed = accept_duration(&prefixed, 900, 0.000_12);
    let restarted_completed = accept_duration(&restarted, 900, 0.000_12);
    assert_eq!(completed, restarted_completed);
    assert_eq!(completed.receipts()[0].ordinal, 0);
    let finalized = completed.finalize().expect("physical suffix closes parent");
    assert_eq!(finalized.receipt.inactive_prefix_sha256, Some(proof_sha256));
}

#[test]
fn native_inactive_prefix_successor_taxonomy_distinguishes_support_from_identity() {
    let beginning = parent_fixture();
    let authority = beginning.authority;
    let prefix_end_ns = authority.support_start_ns + 900_000_000_000;
    let proof = ValidatedNativeInactiveWb14PrefixV1::try_new(
        authority.coupled_parent_transaction_sha256,
        authority.parent_beginning_owner_sha256,
        [72; 32],
        authority.support_start_ns,
        prefix_end_ns,
        authority.support_end_ns,
        [71; 32],
    )
    .expect("complete coupled native inactive prefix");
    let binding = crate::direct_runtime::DirectWb14CoupledChildBindingV1 {
        proposed_upper_bound_s_bits: 900.0_f64.to_bits(),
        coupled_parent_transaction_sha256: authority.coupled_parent_transaction_sha256,
        accepted_slab_sha256: [73; 32],
        parent_beginning_complete_owner_set_sha256: [72; 32],
        parent_support_start_ns: authority.support_start_ns,
        parent_support_end_ns: authority.support_end_ns,
        child_support_start_ns: prefix_end_ns,
        child_support_end_ns: authority.support_end_ns,
    };

    let support_error = proof
        .validate_successor_binding(crate::direct_runtime::DirectWb14CoupledChildBindingV1 {
            child_support_start_ns: prefix_end_ns + 1,
            ..binding
        })
        .expect_err("support gap");
    assert_eq!(support_error, DirectWb14ParentIntervalErrorV1::ChildSupport);
    assert_eq!(
        support_error.canonical_surface_liquid_error_code(),
        crate::DirectSurfaceLiquidErrorCode::E008
    );

    for poisoned in [
        crate::direct_runtime::DirectWb14CoupledChildBindingV1 {
            coupled_parent_transaction_sha256: [74; 32],
            ..binding
        },
        crate::direct_runtime::DirectWb14CoupledChildBindingV1 {
            parent_beginning_complete_owner_set_sha256: [75; 32],
            ..binding
        },
        crate::direct_runtime::DirectWb14CoupledChildBindingV1 {
            accepted_slab_sha256: [0; 32],
            ..binding
        },
    ] {
        let identity_error = proof
            .validate_successor_binding(poisoned)
            .expect_err("identity poison");
        assert_eq!(identity_error, DirectWb14ParentIntervalErrorV1::ChildIdentity);
        assert_eq!(
            identity_error.canonical_surface_liquid_error_code(),
            crate::DirectSurfaceLiquidErrorCode::E002
        );
    }
}

#[test]
fn native_inactive_prefix_does_not_admit_the_prefix_beginning_as_successor_owner() {
    let beginning = parent_fixture();
    let authority = beginning.authority;
    let prefix_end_ns = authority.support_start_ns + 900_000_000_000;
    let proof = ValidatedNativeInactiveWb14PrefixV1::try_new(
        authority.coupled_parent_transaction_sha256,
        authority.parent_beginning_owner_sha256,
        [72; 32],
        authority.support_start_ns,
        prefix_end_ns,
        authority.support_end_ns,
        [71; 32],
    )
    .expect("complete coupled native inactive prefix");
    let binding = crate::direct_runtime::DirectWb14CoupledChildBindingV1 {
        proposed_upper_bound_s_bits: 900.0_f64.to_bits(),
        coupled_parent_transaction_sha256: authority.coupled_parent_transaction_sha256,
        accepted_slab_sha256: [73; 32],
        parent_beginning_complete_owner_set_sha256: authority.parent_beginning_owner_sha256,
        parent_support_start_ns: authority.support_start_ns,
        parent_support_end_ns: authority.support_end_ns,
        child_support_start_ns: prefix_end_ns,
        child_support_end_ns: authority.support_end_ns,
    };
    assert!(
        proof.validate_successor_binding(binding).is_err(),
        "the first physical child must begin at the prefix ending owner, not its beginning"
    );
    let ending_bound = crate::direct_runtime::DirectWb14CoupledChildBindingV1 {
        parent_beginning_complete_owner_set_sha256: [72; 32],
        ..binding
    };
    proof
        .validate_successor_binding(ending_bound)
        .expect("first physical child begins at exact prefix ending owner");
}

#[test]
fn native_inactive_prefix_duplicate_first_child_rejects_without_mutation() {
    let beginning = parent_fixture();
    let authority = beginning.authority;
    let prefix_end_ns = authority.support_start_ns + 900_000_000_000;
    let proof = ValidatedNativeInactiveWb14PrefixV1::try_new(
        authority.coupled_parent_transaction_sha256,
        authority.parent_beginning_owner_sha256,
        [72; 32],
        authority.support_start_ns,
        prefix_end_ns,
        authority.support_end_ns,
        [71; 32],
    )
    .expect("complete coupled native inactive prefix");
    let prefixed = DirectWb14ParentIntervalV1::begin_after_native_inactive_prefix(
        authority,
        beginning.beginning_cursor,
        proof,
    )
    .expect("prefix-backed parent");
    let accepted = accept_duration(&prefixed, 900, 0.000_12);
    let before_id = accepted.authority.parent_id;
    let before_working = accepted.working();
    let before_receipts = accepted.receipts().clone();
    assert!(
        accepted
            .accept_child(
                0,
                prefix_end_ns,
                authority.support_end_ns,
                prefixed.working().receipt_chain_sha256,
                900.0,
                child_inputs(&prefixed, 900.0, 0.000_12),
            )
            .is_err(),
        "duplicate prefix successor was accepted"
    );
    assert_eq!(accepted.authority.parent_id, before_id);
    assert_eq!(accepted.working(), before_working);
    assert_eq!(accepted.receipts(), &before_receipts);
}

#[test]
fn native_inactive_prefix_refuses_advanced_cursor_and_nonzero_first_ordinal() {
    let beginning = parent_fixture();
    let authority = beginning.authority;
    let prefix_end_ns = authority.support_start_ns + 900_000_000_000;
    let proof = ValidatedNativeInactiveWb14PrefixV1::try_new(
        authority.coupled_parent_transaction_sha256,
        authority.parent_beginning_owner_sha256,
        [72; 32],
        authority.support_start_ns,
        prefix_end_ns,
        authority.support_end_ns,
        [71; 32],
    )
    .expect("complete coupled native inactive prefix");

    for poisoned_cursor in [
        DirectWb14PersistentCursorV1 {
            cumulative_supply_m: f64::from_bits(
                beginning.beginning_cursor.cumulative_supply_m.to_bits() + 1,
            ),
            ..beginning.beginning_cursor
        },
        DirectWb14PersistentCursorV1 {
            next_interval_index: beginning.beginning_cursor.next_interval_index + 1,
            ..beginning.beginning_cursor
        },
    ] {
        assert_eq!(
            DirectWb14ParentIntervalV1::begin_after_native_inactive_prefix(
                authority,
                poisoned_cursor,
                proof,
            ),
            Err(DirectWb14ParentIntervalErrorV1::CursorIdentity),
            "inactive support must not authorize cumulative or persistent-cursor advancement"
        );
    }

    let prefixed = DirectWb14ParentIntervalV1::begin_after_native_inactive_prefix(
        authority,
        beginning.beginning_cursor,
        proof,
    )
    .expect("prefix-backed parent");
    let before = serde_json::to_vec(&prefixed).expect("pre-attempt parent bytes");
    assert_eq!(
        prefixed.accept_child(
            1,
            prefix_end_ns,
            authority.support_end_ns,
            prefixed.working().receipt_chain_sha256,
            900.0,
            child_inputs(&prefixed, 900.0, 0.000_12),
        ),
        Err(DirectWb14ParentIntervalErrorV1::ChildOrdinal),
    );
    assert_eq!(
        serde_json::to_vec(&prefixed).expect("post-refusal parent bytes"),
        before,
        "nonzero first-child refusal must preserve the full split-parent state"
    );
}

#[test]
fn ordinary_parent_replay_remains_prefix_free() {
    let beginning = parent_fixture();
    let beginning_bytes = serde_json::to_vec(&beginning).expect("ordinary parent bytes");
    let replayed: DirectWb14ParentIntervalV1 =
        serde_json::from_slice(&beginning_bytes).expect("ordinary parent replay");
    assert_eq!(
        serde_json::to_vec(&replayed).expect("ordinary replay bytes"),
        beginning_bytes
    );
    assert_eq!(replayed, beginning);
    assert!(beginning.authority.inactive_prefix.is_none());
    assert_eq!(
        beginning.working().accepted_until_ns,
        beginning.authority.support_start_ns
    );
    assert_eq!(beginning.working().next_child_ordinal, 0);
    let completed = accept_duration(&beginning, 1_800, 0.000_12);
    assert_eq!(completed.receipts()[0].ordinal, 0);
    let finalized = completed.finalize().expect("ordinary parent finalization");
    assert_eq!(finalized.receipt.inactive_prefix_sha256, None);
}

#[test]
fn ordinary_parent_replay_refuses_advanced_working_cursor_without_prefix_proof() {
    let beginning = parent_fixture();
    let canonical = serde_json::to_vec(&beginning).expect("ordinary parent canonical bytes");
    let mut wire = serde_json::to_value(&beginning).expect("ordinary parent JSON value");
    wire["working"]["accepted_until_ns"] = serde_json::json!(
        beginning.authority.support_start_ns + 60_000_000_000_u128
    );
    let poisoned: DirectWb14ParentIntervalV1 =
        serde_json::from_value(wire).expect("structurally parseable advanced parent");
    assert!(poisoned.authority.inactive_prefix.is_none());
    assert_eq!(
        poisoned.validate(),
        Err(DirectWb14ParentIntervalErrorV1::ReceiptValidation),
        "an advanced parent-local cursor requires authenticated inactive-prefix or physical receipt custody"
    );
    assert_eq!(
        serde_json::to_vec(&beginning).expect("ordinary parent bytes after poison"),
        canonical,
        "replay poison validation must not mutate the ordinary parent"
    );
}
