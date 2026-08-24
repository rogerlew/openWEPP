#[test]
#[allow(clippy::too_many_lines)]
fn independent_projection_binds_persistent_stores_continuations_and_digest() {
    let configuration = three_ofe_configuration();
    let beginning = initial_state(&configuration, 0.0);
    let transaction_id = TransactionId(411);
    let resource = resource_candidate(&configuration, &beginning, transaction_id, None, &[]);
    let input = DirectSurfaceLiquidIngressInput {
        transaction_id,
        day_index: 3,
        interval_index: 0,
        interval_s: INTERVAL_S,
        tile_ingress: configuration
            .records
            .iter()
            .map(|record| open_ingress(record, 0.1))
            .collect(),
        wb14_parameters: parameters(&configuration),
    };
    let candidate = execute_surface_liquid_ingress(&configuration, &resource, &input)
        .expect("three-OFE candidate");
    super::super::surface_liquid_closure::validate_surface_liquid_closure_operands(
        &configuration,
        &resource,
        &candidate.closure_operands,
        &candidate.receipts,
        &candidate.ending_state,
    )
    .expect("persistent projection baseline");
    let store_keys = configuration
        .records
        .iter()
        .map(|row| row.key.clone())
        .collect::<Vec<_>>();
    let mut store_poisons = Vec::new();
    let mut wrong_store = candidate.ending_state.clone();
    wrong_store.records[0].liquid_kg_m2_tile += 0.001;
    store_poisons.push((
        "wrong ending store value".to_owned(),
        wrong_store,
        store_keys[0].clone(),
    ));
    for (index, expected_store) in store_keys.iter().enumerate() {
        let mut ending = candidate.ending_state.clone();
        ending.records.remove(index);
        store_poisons.push((
            format!("missing ending store {index}"),
            ending,
            expected_store.clone(),
        ));
        let mut ending = candidate.ending_state.clone();
        ending.records.push(ending.records[index].clone());
        store_poisons.push((
            format!("extra ending store {index}"),
            ending,
            expected_store.clone(),
        ));
        let mut ending = candidate.ending_state.clone();
        let forged = DirectSurfaceLiquidStoreKey {
            run_id: 91,
            ofe_id: ofe(&format!("forged-{index}")),
            tile_id: tile(&format!("forged-tile-{index}")),
            surface_id: surface(&format!("forged-surface-{index}")),
            surface_class: SurfaceClass::BareMineralSoil,
            source_type: openwepp_land_surface_energy::WaterSourceType::SurfaceLiquid,
            source_id: source(&format!("forged-source-{index}")),
        };
        ending.records[index].key = forged.clone();
        store_poisons.push((format!("replacement ending store {index}"), ending, forged));
    }
    for (left, right) in [(0, 1), (1, 2), (0, 2)] {
        let mut ending = candidate.ending_state.clone();
        ending.records.swap(left, right);
        store_poisons.push((
            format!("reordered ending stores {left}/{right}"),
            ending,
            store_keys[right].clone(),
        ));
    }
    for (label, mut ending, expected_store) in store_poisons {
        ending.state_sha256 = ending.recomputed_sha256().expect("poison digest");
        let attempted_sha256 = ending.recomputed_sha256().expect("attempted digest");
        let error = super::super::surface_liquid_closure::validate_surface_liquid_closure_operands(
            &configuration,
            &resource,
            &candidate.closure_operands,
            &candidate.receipts,
            &ending,
        )
        .expect_err(&label);
        assert_independent_store_e010(
            &error,
            transaction_id,
            &configuration,
            &expected_store,
            &resource.beginning_state().state_sha256,
            &attempted_sha256,
            &label,
        );
    }
    let mut poisons = Vec::new();
    let mut supply = candidate.ending_state.clone();
    supply.continuations[0].cumulative_supply_m += 1.0e-6;
    poisons.push(("cumulative supply".to_owned(), supply, ofe("upper")));
    let mut infiltration = candidate.ending_state.clone();
    infiltration.continuations[0].cumulative_infiltration_m += 1.0e-6;
    poisons.push((
        "cumulative infiltration".to_owned(),
        infiltration,
        ofe("upper"),
    ));
    let mut rollover_day = candidate.ending_state.clone();
    rollover_day.continuations[0].day_index += 1;
    poisons.push(("rollover day".to_owned(), rollover_day, ofe("upper")));
    let mut rollover_interval = candidate.ending_state.clone();
    rollover_interval.continuations[0].next_interval_index = 48;
    poisons.push((
        "rollover interval".to_owned(),
        rollover_interval,
        ofe("upper"),
    ));
    let mut stale_transaction = candidate.ending_state.clone();
    stale_transaction.continuations[0].last_accepted_transaction_id = Some(TransactionId(410));
    poisons.push((
        "continuation transaction".to_owned(),
        stale_transaction,
        ofe("upper"),
    ));
    for (index, expected_ofe) in configuration.ofe_topology.iter().cloned().enumerate() {
        let mut ending = candidate.ending_state.clone();
        ending.continuations.remove(index);
        poisons.push((
            format!("missing continuation {index}"),
            ending,
            expected_ofe.clone(),
        ));
        let mut ending = candidate.ending_state.clone();
        ending
            .continuations
            .push(ending.continuations[index].clone());
        poisons.push((format!("extra continuation {index}"), ending, expected_ofe));
        let mut ending = candidate.ending_state.clone();
        let forged = ofe(&format!("forged-{index}"));
        ending.continuations[index].ofe_id = forged.clone();
        poisons.push((format!("replacement continuation {index}"), ending, forged));
    }
    for (left, right) in [(0, 1), (1, 2), (0, 2)] {
        let mut ending = candidate.ending_state.clone();
        ending.continuations.swap(left, right);
        poisons.push((
            format!("reordered continuations {left}/{right}"),
            ending,
            configuration.ofe_topology[right].clone(),
        ));
    }
    for (label, mut ending, expected_ofe) in poisons {
        ending.state_sha256 = ending.recomputed_sha256().expect("poison digest");
        let attempted_sha256 = ending.recomputed_sha256().expect("attempted digest");
        let error = super::super::surface_liquid_closure::validate_surface_liquid_closure_operands(
            &configuration,
            &resource,
            &candidate.closure_operands,
            &candidate.receipts,
            &ending,
        )
        .expect_err(&label);
        assert_independent_continuation_e010(
            &error,
            transaction_id,
            &configuration,
            &expected_ofe,
            &resource.beginning_state().state_sha256,
            &attempted_sha256,
            &label,
        );
    }
    let mut forged = candidate.clone();
    forged
        .closure_operands
        .forge_first_store_retained_and_ending_for_test(0.001);
    forged.ending_state.records[0].liquid_kg_m2_tile += 0.001;
    forged.ending_state.state_sha256 = forged
        .ending_state
        .recomputed_sha256()
        .expect("forged digest");
    let error = super::super::surface_liquid_closure::validate_surface_liquid_closure_operands(
        &configuration,
        &resource,
        &forged.closure_operands,
        &forged.receipts,
        &forged.ending_state,
    )
    .expect_err("self-consistent producer store operands");
    assert_eq!(error.code(), DirectSurfaceLiquidErrorCode::E010);

    let mut digest = candidate.ending_state.clone();
    digest.state_sha256.push('0');
    let attempted_sha256 = digest.recomputed_sha256().expect("attempted digest");
    let error = super::super::surface_liquid_closure::validate_surface_liquid_closure_operands(
        &configuration,
        &resource,
        &candidate.closure_operands,
        &candidate.receipts,
        &digest,
    )
    .expect_err("ending digest mismatch after joins");
    assert_independent_aggregate_e010(
        &error,
        transaction_id,
        &configuration,
        &resource.beginning_state().state_sha256,
        &attempted_sha256,
        "ending digest mismatch",
    );

    let mut aggregate = candidate.ending_state.clone();
    aggregate.owner_id = owner("forged-owner");
    aggregate.state_sha256 = aggregate.recomputed_sha256().expect("aggregate digest");
    let attempted_sha256 = aggregate.recomputed_sha256().expect("attempted digest");
    let error = super::super::surface_liquid_closure::validate_surface_liquid_closure_operands(
        &configuration,
        &resource,
        &candidate.closure_operands,
        &candidate.receipts,
        &aggregate,
    )
    .expect_err("aggregate owner mismatch");
    assert_independent_aggregate_e010(
        &error,
        transaction_id,
        &configuration,
        &resource.beginning_state().state_sha256,
        &attempted_sha256,
        "aggregate owner mismatch",
    );

    let mut aggregate = candidate.ending_state.clone();
    aggregate.configuration_sha256.push('0');
    aggregate.state_sha256 = aggregate.recomputed_sha256().expect("aggregate digest");
    let attempted_sha256 = aggregate.recomputed_sha256().expect("attempted digest");
    let error = super::super::surface_liquid_closure::validate_surface_liquid_closure_operands(
        &configuration,
        &resource,
        &candidate.closure_operands,
        &candidate.receipts,
        &aggregate,
    )
    .expect_err("aggregate configuration mismatch");
    assert_independent_aggregate_e010(
        &error,
        transaction_id,
        &configuration,
        &resource.beginning_state().state_sha256,
        &attempted_sha256,
        "aggregate configuration mismatch",
    );
}

#[test]
#[allow(clippy::too_many_lines)]
fn partition_input_membership_is_e009_and_arithmetic_e003_outranks_identity() {
    let configuration = three_ofe_configuration();
    let beginning = initial_state(&configuration, 0.0);
    let transaction_id = TransactionId(412);
    let resource = resource_candidate(&configuration, &beginning, transaction_id, None, &[]);
    let input = DirectSurfaceLiquidIngressInput {
        transaction_id,
        day_index: 3,
        interval_index: 0,
        interval_s: INTERVAL_S,
        tile_ingress: configuration
            .records
            .iter()
            .map(|record| open_ingress(record, 0.1))
            .collect(),
        wb14_parameters: parameters(&configuration),
    };
    let candidate = execute_surface_liquid_ingress(&configuration, &resource, &input)
        .expect("three-OFE candidate");

    let mut membership_poisons = Vec::new();
    let mut missing = candidate.clone();
    let missing_id = missing.closure_operands.remove_partition_input_for_test(1);
    membership_poisons.push(("missing", missing, missing_id));
    let mut duplicate = candidate.clone();
    let duplicate_id = duplicate
        .closure_operands
        .duplicate_partition_input_for_test(0);
    membership_poisons.push(("duplicate", duplicate, duplicate_id));
    let mut reordered = candidate.clone();
    reordered
        .closure_operands
        .reorder_partition_inputs_for_test();
    membership_poisons.push(("reordered", reordered, ofe("middle")));
    let mut wrong = candidate.clone();
    wrong
        .closure_operands
        .rekey_partition_input_for_test(0, ofe("forged"));
    membership_poisons.push(("wrong OFE", wrong, ofe("upper")));

    for (label, poison, expected_ofe) in membership_poisons {
        let attempted = poison.ending_state.recomputed_sha256().expect("digest");
        assert_producer_e009(
            &poison
                .validate(&configuration, &resource, &input)
                .expect_err(label),
            transaction_id,
            &configuration,
            Some(&expected_ofe),
            None,
            None,
            &resource.beginning_state().state_sha256,
            &attempted,
        );
    }

    let mut arithmetic = candidate.clone();
    arithmetic
        .closure_operands
        .poison_partition_cumulative_bound_for_test(0);
    arithmetic
        .closure_operands
        .remove_partition_input_for_test(1);
    arithmetic.ending_state.records[0].liquid_kg_m2_tile += 0.001;
    arithmetic.ending_state.state_sha256 = arithmetic
        .ending_state
        .recomputed_sha256()
        .expect("combined attempted digest");
    let error = arithmetic
        .validate(&configuration, &resource, &input)
        .expect_err("E003 must outrank E009 and E010");
    let failure = error.failure().expect("typed arithmetic failure");
    assert_eq!(failure.code, DirectSurfaceLiquidErrorCode::E003);
    assert_eq!(failure.phase, DirectSurfaceLiquidPhase::IndependentClosure);
    assert_eq!(failure.context.ofe_id, Some(ofe("upper")));
    assert_eq!(
        failure.rollback.beginning_owner_sha256.as_deref(),
        Some(resource.beginning_state().state_sha256.as_str())
    );
    assert_eq!(
        failure.rollback.attempted_owner_sha256.as_deref(),
        Some(arithmetic.ending_state.state_sha256.as_str())
    );

    let mut capacity = candidate.clone();
    capacity
        .closure_operands
        .poison_partition_capacity_bound_for_test(0);
    let capacity_error = capacity
        .validate(&configuration, &resource, &input)
        .expect_err("cumulative infiltration above capacity but below supply");
    let capacity_failure = capacity_error.failure().expect("typed capacity failure");
    assert_eq!(capacity_failure.code, DirectSurfaceLiquidErrorCode::E003);
    assert_eq!(capacity_failure.context.ofe_id, Some(ofe("upper")));
    assert_eq!(
        capacity_failure.rollback.beginning_owner_sha256.as_deref(),
        Some(resource.beginning_state().state_sha256.as_str())
    );

    let mut identity_over_closure = candidate;
    identity_over_closure
        .closure_operands
        .remove_partition_input_for_test(1);
    identity_over_closure.ending_state.records[0].liquid_kg_m2_tile += 0.001;
    identity_over_closure.ending_state.state_sha256 = identity_over_closure
        .ending_state
        .recomputed_sha256()
        .expect("identity attempted digest");
    assert_eq!(
        identity_over_closure
            .validate(&configuration, &resource, &input)
            .expect_err("E009 must outrank E010")
            .code(),
        DirectSurfaceLiquidErrorCode::E009
    );
}

#[test]
fn producer_identity_helper_attributes_reorder_and_replacement_to_actual_rows() {
    let expected = vec![ofe("upper"), ofe("middle"), ofe("lower")];
    let reordered = vec![ofe("middle"), ofe("upper"), ofe("lower")];
    let replacement = vec![ofe("replacement"), ofe("middle"), ofe("lower")];
    assert_eq!(
        first_identity_aware_mismatch(&reordered, &expected, Clone::clone),
        Some(&reordered[0])
    );
    assert_eq!(
        first_identity_aware_mismatch(&replacement, &expected, Clone::clone),
        Some(&replacement[0])
    );

    let expected_map = expected
        .iter()
        .cloned()
        .map(|key| (key, 1))
        .collect::<BTreeMap<_, _>>();
    let mut replacement_map = expected_map.clone();
    replacement_map.remove(&ofe("upper"));
    replacement_map.insert(ofe("replacement"), 1);
    assert_eq!(
        first_map_identity_mismatch(&replacement_map, &expected_map),
        Some(&ofe("replacement"))
    );
}

#[allow(clippy::too_many_arguments)]
fn assert_producer_e009(
    error: &DirectSurfaceLiquidError,
    transaction_id: TransactionId,
    configuration: &DirectSurfaceLiquidConfiguration,
    ofe_id: Option<&OfeId>,
    tile_id: Option<&TileId>,
    parcel_id: Option<&str>,
    beginning_sha256: &str,
    attempted_sha256: &str,
) {
    let failure = error.failure().expect("canonical producer failure");
    assert_eq!(failure.code, DirectSurfaceLiquidErrorCode::E009);
    assert_eq!(failure.phase, DirectSurfaceLiquidPhase::IngressCandidate);
    assert_eq!(failure.context.transaction_id, Some(transaction_id));
    assert_eq!(
        failure.context.owner_id,
        Some(configuration.owner_id.clone())
    );
    assert_eq!(failure.context.ofe_id.as_ref(), ofe_id);
    assert_eq!(failure.context.tile_id.as_ref(), tile_id);
    assert_eq!(failure.context.parcel_id.as_deref(), parcel_id);
    assert_eq!(
        failure.rollback.beginning_owner_sha256.as_deref(),
        Some(beginning_sha256)
    );
    assert_eq!(
        failure.rollback.attempted_owner_sha256.as_deref(),
        Some(attempted_sha256)
    );
}

#[path = "surface_liquid_ingress_terminal_tests.rs"]
mod terminal_tests;
