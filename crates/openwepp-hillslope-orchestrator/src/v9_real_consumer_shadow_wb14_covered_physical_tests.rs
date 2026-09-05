// Contract binding: SC-SURFACELIQUID-001@8 INV-SURFACELIQUID-012..014.
#[test]
#[allow(clippy::too_many_lines)]
fn mixed_open_covered_stack_executes_complete_ofe_ground_boundary() {
    std::thread::Builder::new()
        .name("mixed-covered-open-boundary".to_owned())
        .stack_size(64 * 1024 * 1024)
        .spawn(|| {
            exercise_complete_wb14_cadence(
                0.005, 8.0, true, None, false, None, false, false, false, false,
            );
        })
        .expect("spawn mixed covered/open boundary fixture")
        .join()
        .expect("join mixed covered/open boundary fixture");
}

#[test]
#[allow(clippy::too_many_lines)]
fn two_900_second_complete_owner_children_publish_one_parent() {
    exercise_complete_wb14_cadence(
        0.02, 8.0, false, None, false, None, false, false, false, false,
    );
}

#[test]
#[allow(clippy::too_many_lines)]
fn one_1800_second_child_matches_complete_historical_candidate() {
    exercise_complete_wb14_cadence(
        0.08, 8.0, false, None, false, None, false, false, false, false,
    );
}

fn snow_free_reuse_live_clock_v1(
    beginning_owners: &[OwnerState],
    end_ns: u128,
) -> (ParentTransactionId, CoupledClockStateV1) {
    let support = TimeSupport::new(ModelTimeNs::new(0), ModelTimeNs::new(end_ns))
        .expect("snow-free reuse support");
    let beginning_digest =
        complete_owner_set_digest(beginning_owners).expect("snow-free reuse beginning owners");
    let interval = ParentIntervalId::derive(digest(1), digest(2), digest(3), support)
        .expect("snow-free reuse interval");
    let parent = ParentTransactionId::derive(digest(1), 40, interval, beginning_digest)
        .expect("snow-free reuse parent");
    let authority = ParentAuthorityV1::new(
        digest(1),
        digest(2),
        digest(3),
        40,
        support,
        beginning_digest,
    )
    .expect("snow-free reuse authority");
    let participants = beginning_owners
        .iter()
        .map(|owner| owner.owner_id().to_owned())
        .collect::<Vec<_>>();
    let clock = CoupledClockStateV1::new(
        authority,
        beginning_owners.to_vec(),
        "snow-free".to_owned(),
        participants,
        digest(4),
        Vec::new(),
    )
    .expect("snow-free reuse clock");
    (parent, clock)
}

fn accepted_snow_free_receipt_from_live_clock_v1(
    live_clock: &CoupledClockStateV1,
    ending_owners: Vec<OwnerState>,
    end_ns: u128,
    constraint_authority: Digest32,
    ledger_lineage: Digest32,
) -> openwepp_coupled_time::AcceptedSlabReceiptV1 {
    let support = TimeSupport::new(ModelTimeNs::new(0), ModelTimeNs::new(end_ns))
        .expect("snow-free reuse support");
    let mut clock = live_clock.clone();
    let parent = clock.parent_transaction_id();
    let constraint = StepConstraintV1::new(
        parent,
        ModelTimeNs::new(0),
        ModelTimeNs::new(end_ns),
        "vegetation".to_owned(),
        ConstraintClass::HardBoundary,
        constraint_authority,
        digest(2),
        digest(3),
    )
    .expect("snow-free reuse constraint");
    let reduction = reduce_constraints(
        &[constraint],
        parent,
        ModelTimeNs::new(0),
        ModelTimeNs::new(end_ns),
        None,
    )
    .expect("snow-free reuse reduction");
    let segment = clock.active_segment_id();
    let joined = digest(6);
    let ledger = LedgerEntryV1::new(
        "vegetation".to_owned(),
        "owner".to_owned(),
        joined,
        joined,
        ledger_lineage,
    )
    .expect("snow-free reuse ledger");
    let slab = CoupledSlabCandidateV1::new(
        &clock,
        segment,
        support,
        &reduction,
        ending_owners,
        vec![ledger],
    )
    .expect("snow-free reuse slab");
    accept_slab(&mut clock, slab).expect("snow-free reuse receipt")
}

fn snow_free_reuse_binding_v1(
    parent: ParentTransactionId,
    receipt: &openwepp_coupled_time::AcceptedSlabReceiptV1,
    beginning_owner_sha256: Digest32,
    end_ns: u128,
) -> crate::direct_runtime::DirectWb14CoupledChildBindingV1 {
    crate::direct_runtime::DirectWb14CoupledChildBindingV1 {
        proposed_upper_bound_s_bits: 1_800.0_f64.to_bits(),
        coupled_parent_transaction_sha256: *parent.digest().as_bytes(),
        accepted_slab_sha256: *receipt.slab_id().digest().as_bytes(),
        parent_beginning_complete_owner_set_sha256: *beginning_owner_sha256.as_bytes(),
        parent_support_start_ns: 0,
        // This focused reuse vector deliberately leaves a live partial WB14
        // parent so final identity reseal must rebind its staged child.
        parent_support_end_ns: end_ns * 2,
        child_support_start_ns: 0,
        child_support_end_ns: end_ns,
    }
}

#[test]
#[allow(clippy::too_many_lines)]
fn snow_free_publication_reseal_mints_one_fresh_validated_support_capability() {
    std::thread::Builder::new()
        .name("direct-snow-free-identity-reseal".to_owned())
        .stack_size(64 * 1024 * 1024)
        .spawn(|| {
            const END_NS: u128 = 900_000_000_000;
            let (shadow, fixture) = v10_nonredistributing_shadow_fixture();
            let base_interval = day_input(&fixture).intervals.remove(0);
            let interval = segment_interval(&base_interval, END_NS, 41, 0.0);
            let migrated = migrate_v10_runtime_to_v11(
                &shadow.vegetation_configuration,
                &shadow.vegetation_state,
            )
            .expect("snow-free reuse migration");
            let owners = initial_v11_owners(&shadow, &migrated.state);
            let beginning_owner_states = owners
                .values()
                .map(|owner| owner.to_owner_state().expect("snow-free beginning owner"))
                .collect::<Vec<_>>();
            let beginning_owner_sha256 = complete_owner_set_digest(&beginning_owner_states)
                .expect("snow-free beginning digest");
            let (parent_id, live_clock) =
                snow_free_reuse_live_clock_v1(&beginning_owner_states, END_NS);
            let provisional_receipt = accepted_snow_free_receipt_from_live_clock_v1(
                &live_clock,
                beginning_owner_states.clone(),
                END_NS,
                digest(5),
                digest(7),
            );
            let parent = V11ParentTransaction::new_with_complete_owners(
                &migrated.configuration,
                &migrated.state,
                parent_id,
                ModelTimeNs::new(0),
                owners,
            )
            .expect("snow-free reuse V11 parent");

            crate::v9_real_consumer_shadow::snow_free_physical_reuse::
                begin_snow_free_physical_reuse_audit_v1();
            crate::v9_real_consumer_shadow::begin_accepted_publication_support_capability_audit_v1(
            );
            let provisional_binding = snow_free_reuse_binding_v1(
                parent_id,
                &provisional_receipt,
                beginning_owner_sha256,
                END_NS,
            );
            let mut provisional_executor =
                crate::v11_vegetation_consumer::DirectV11VegetationExecutor {
                    stack: DirectV11RealConsumerStack::new_parent_child(
                        &shadow,
                        &interval,
                        0,
                        0,
                        false,
                        provisional_binding,
                    ),
                };
            let provisional = execute_direct_v11_segment(
                &migrated.configuration,
                &parent,
                &provisional_receipt,
                &mut provisional_executor,
            )
            .expect("snow-free provisional physical execution");
            let final_owner_states = provisional
                .ending_resource_owners
                .values()
                .map(|owner| owner.to_owner_state().expect("snow-free ending owner"))
                .collect::<Vec<_>>();
            let final_receipt = accepted_snow_free_receipt_from_live_clock_v1(
                &live_clock,
                final_owner_states.clone(),
                END_NS,
                digest(5),
                digest(7),
            );
            assert!(
                provisional_receipt.shares_live_beginning_revision_with(&final_receipt),
                "provisional/final receipts must fork the same live clock revision",
            );
            let final_binding = snow_free_reuse_binding_v1(
                parent_id,
                &final_receipt,
                beginning_owner_sha256,
                END_NS,
            );
            let armed = crate::v9_real_consumer_shadow::prepare_snow_free_physical_reuse(
                provisional_executor.stack,
                final_binding,
            )
            .expect("arm snow-free final identity reseal");

            let refused_clone = armed.clone();
            let mut refused_clone_executor =
                crate::v11_vegetation_consumer::DirectV11VegetationExecutor {
                    stack: refused_clone,
                };
            assert!(matches!(
                execute_direct_v11_segment(
                    &migrated.configuration,
                    &parent,
                    &final_receipt,
                    &mut refused_clone_executor,
                ),
                Err(V11ExecutionError::Executor(
                    DirectV11RealConsumerError::Identity("snow-free physical reuse single use")
                ))
            ));
            let mut reuse_executor =
                crate::v11_vegetation_consumer::DirectV11VegetationExecutor { stack: armed };
            let reused = execute_direct_v11_segment(
                &migrated.configuration,
                &parent,
                &final_receipt,
                &mut reuse_executor,
            )
            .expect("snow-free final identity reseal");
            let mut accepted_parent = parent.clone();
            accept_direct_v11_segment(
                &mut accepted_parent,
                &migrated.configuration,
                reused.clone(),
                &shadow,
            )
            .expect("accept snow-free identity-resealed candidate into the real V11 parent");
            let reused_shadow = reuse_executor
                .stack
                .commit_selected_publication_and_take_staged_ending()
                .expect("snow-free reused ending");
            let capability_audit = crate::v9_real_consumer_shadow::
                take_accepted_publication_support_capability_audit_v1();
            assert_eq!(
                capability_audit,
                crate::v9_real_consumer_shadow::AcceptedPublicationSupportCapabilityAuditV1 {
                    full_validation_attempt_count: 1,
                    full_validation_success_count: 1,
                    operand_seal_count: 1,
                    receipt_seal_count: 1,
                    capability_mint_count: 1,
                    trusted_append_attempt_count: 1,
                    live_revision_join_count: 1,
                    chronology_owner_tail_join_count: 1,
                    successful_append_count: 1,
                    append_time_full_validation_count: 0,
                    append_time_operand_reconstruction_count: 0,
                    append_time_receipt_reconstruction_count: 0,
                    append_time_serialization_count: 0,
                    append_time_full_prefix_scan_count: 0,
                    support_payload_clone_count: 0,
                },
            );
            let reuse_audit = crate::v9_real_consumer_shadow::snow_free_physical_reuse::
                take_snow_free_physical_reuse_audit_v1();
            assert_eq!(reuse_audit.physical_execution_count, 1);
            assert_eq!(reuse_audit.identity_reseal_count, 1);
            assert_eq!(reuse_audit.final_publication_append_count, 1);
            assert_eq!(reuse_audit.outer_accepted_publication_count, 1);
            assert_eq!(reuse_audit.provider_projection_count, 1);
            assert_eq!(reuse_audit.vapor_operation_count, 1);
            assert_eq!(reuse_audit.phase_operation_count, 1);
            assert_eq!(reuse_audit.ingress_operation_count, 1);
            assert_eq!(reuse_audit.wb14_operation_count, 1);
            assert_eq!(reuse_audit.routing_operation_count, 1);

            let accepted_receipt_before_duplicate =
                reuse_executor.stack.last_support_receipt().cloned();
            assert!(reuse_executor.stack.ending.is_none());
            assert!(matches!(
                execute_direct_v11_segment(
                    &migrated.configuration,
                    &parent,
                    &final_receipt,
                    &mut reuse_executor,
                ),
                Err(V11ExecutionError::Executor(
                    DirectV11RealConsumerError::Identity("snow-free physical reuse single use")
                ))
            ));
            assert!(reuse_executor.stack.ending.is_none());
            assert_eq!(
                reuse_executor.stack.last_support_receipt(),
                accepted_receipt_before_duplicate.as_ref()
            );

            crate::v9_real_consumer_shadow::snow_free_physical_reuse::
                begin_snow_free_physical_reuse_audit_v1();
            let mut forced_executor = crate::v11_vegetation_consumer::DirectV11VegetationExecutor {
                // Explicit fresh/restart-equivalent execution owns no proof;
                // it is the full-physics oracle, never an armed-stack clone.
                stack: DirectV11RealConsumerStack::new_parent_child(
                    &shadow,
                    &interval,
                    0,
                    0,
                    false,
                    final_binding,
                ),
            };
            let forced = execute_direct_v11_segment(
                &migrated.configuration,
                &parent,
                &final_receipt,
                &mut forced_executor,
            )
            .expect("fresh full final execution after proof transfer");
            let mut forced_parent = parent.clone();
            accept_direct_v11_segment(
                &mut forced_parent,
                &migrated.configuration,
                forced.clone(),
                &shadow,
            )
            .expect("accept fresh full final oracle into the real V11 parent");
            let forced_shadow = forced_executor
                .stack
                .commit_selected_publication_and_take_staged_ending()
                .expect("forced final ending");
            let fresh_audit = crate::v9_real_consumer_shadow::snow_free_physical_reuse::
                take_snow_free_physical_reuse_audit_v1();
            assert_eq!(fresh_audit.physical_execution_count, 1);
            assert_eq!(fresh_audit.identity_reseal_count, 0);
            assert_eq!(fresh_audit.final_publication_append_count, 0);
            assert_eq!(fresh_audit.outer_accepted_publication_count, 1);
            assert_eq!(fresh_audit.provider_projection_count, 1);
            assert_eq!(fresh_audit.vapor_operation_count, 1);
            assert_eq!(fresh_audit.phase_operation_count, 1);
            assert_eq!(fresh_audit.ingress_operation_count, 1);
            assert_eq!(fresh_audit.wb14_operation_count, 1);
            assert_eq!(fresh_audit.routing_operation_count, 1);

            assert_eq!(reused, forced, "final V11 candidate bytes/receipts");
            assert_eq!(
                reused.ending_resource_owners, provisional.ending_resource_owners,
                "identity reseal changed validated physical owner envelopes",
            );
            assert!(
                reused_shadow.inner.authority == forced_shadow.inner.authority,
                "inner authority differs"
            );
            assert!(
                reused_shadow.inner.provider_gsi_receipt_sha256
                    == forced_shadow.inner.provider_gsi_receipt_sha256,
                "inner GSI receipt differs"
            );
            assert!(
                reused_shadow.inner.vegetation_configuration
                    == forced_shadow.inner.vegetation_configuration,
                "inner vegetation configuration differs"
            );
            assert!(
                reused_shadow.inner.vegetation_state == forced_shadow.inner.vegetation_state,
                "inner vegetation state differs"
            );
            assert!(
                reused_shadow.inner.vegetation_owner_id == forced_shadow.inner.vegetation_owner_id,
                "inner vegetation owner differs"
            );
            assert!(
                reused_shadow.inner.lse_configuration == forced_shadow.inner.lse_configuration,
                "inner LSE configuration differs"
            );
            assert!(
                reused_shadow.inner.lse_state == forced_shadow.inner.lse_state,
                "inner LSE state differs"
            );
            assert!(
                reused_shadow.inner.surface_configuration
                    == forced_shadow.inner.surface_configuration,
                "inner surface configuration differs"
            );
            assert!(
                reused_shadow.inner.layer_maps == forced_shadow.inner.layer_maps,
                "inner layer maps differ"
            );
            assert!(
                reused_shadow.inner.soil_thermal == forced_shadow.inner.soil_thermal,
                "inner soil thermal differs"
            );
            assert!(
                reused_shadow.inner.biogeochemistry == forced_shadow.inner.biogeochemistry,
                "inner BGC differs"
            );
            assert!(
                reused_shadow.inner.hydrology_frame == forced_shadow.inner.hydrology_frame,
                "inner hydrology differs"
            );
            assert!(
                reused_shadow.inner.next_day_index == forced_shadow.inner.next_day_index,
                "inner next day differs"
            );
            assert!(
                reused_shadow.inner.accepted_interval_count
                    == forced_shadow.inner.accepted_interval_count,
                "inner accepted interval count differs"
            );
            assert!(
                reused_shadow.inner.wb14_parent_working_state
                    == forced_shadow.inner.wb14_parent_working_state,
                "inner WB14 parent differs"
            );
            assert!(
                reused_shadow.inner.root_zone_hydraulic_configuration
                    == forced_shadow.inner.root_zone_hydraulic_configuration,
                "inner root-zone configuration differs"
            );
            assert!(reused_shadow.inner == forced_shadow.inner, "inner differs");
            assert!(
                reused_shadow.vegetation_configuration == forced_shadow.vegetation_configuration,
                "vegetation configuration differs"
            );
            assert!(
                reused_shadow.vegetation_state == forced_shadow.vegetation_state,
                "vegetation state differs"
            );
            assert!(
                reused_shadow.lse_configuration == forced_shadow.lse_configuration,
                "LSE configuration differs"
            );
            assert!(
                reused_shadow.lse_state == forced_shadow.lse_state,
                "LSE state differs"
            );
            assert!(
                reused_shadow.gsi_owner_configuration == forced_shadow.gsi_owner_configuration,
                "GSI owner configuration differs"
            );
            assert!(
                reused_shadow.gsi_state == forced_shadow.gsi_state,
                "GSI state differs"
            );
            assert!(
                reused_shadow.provider_static_configuration
                    == forced_shadow.provider_static_configuration,
                "provider static configuration differs"
            );
            assert!(
                reused_shadow.provider_cursor == forced_shadow.provider_cursor,
                "provider cursor differs"
            );
            assert!(
                reused_shadow.root_zone_hydraulic_configuration
                    == forced_shadow.root_zone_hydraulic_configuration,
                "root-zone configuration differs"
            );
            assert!(
                reused_shadow.accepted_publication_history
                    == forced_shadow.accepted_publication_history,
                "accepted publication history differs"
            );
            assert!(
                reused_shadow.frozen_litter_v3 == forced_shadow.frozen_litter_v3,
                "frozen litter V3 differs"
            );
            assert!(
                reused_shadow.frozen_litter_v4 == forced_shadow.frozen_litter_v4,
                "frozen litter V4 differs"
            );
            assert_eq!(reused_shadow, forced_shadow, "final staged owner bytes");
            assert_eq!(
                reuse_executor.stack.last_support_receipt(),
                forced_executor.stack.last_support_receipt(),
                "final support receipt",
            );

            let make_armed = || {
                let mut executor = crate::v11_vegetation_consumer::DirectV11VegetationExecutor {
                    stack: DirectV11RealConsumerStack::new_parent_child(
                        &shadow,
                        &interval,
                        0,
                        0,
                        false,
                        provisional_binding,
                    ),
                };
                execute_direct_v11_segment(
                    &migrated.configuration,
                    &parent,
                    &provisional_receipt,
                    &mut executor,
                )
                .expect("poison-vector provisional physical execution");
                crate::v9_real_consumer_shadow::prepare_snow_free_physical_reuse(
                    executor.stack,
                    final_binding,
                )
                .expect("poison-vector arm")
            };
            let reject_without_mutation = |stack: DirectV11RealConsumerStack<'_>| {
                let staged_before = stack
                    .ending
                    .clone()
                    .expect("poison-vector staged physical ending");
                let receipt_before = stack.last_support_receipt().cloned();
                let mut executor =
                    crate::v11_vegetation_consumer::DirectV11VegetationExecutor { stack };
                assert!(matches!(
                    execute_direct_v11_segment(
                        &migrated.configuration,
                        &parent,
                        &final_receipt,
                        &mut executor,
                    ),
                    Err(V11ExecutionError::Executor(
                        DirectV11RealConsumerError::Identity("snow-free physical reuse identity")
                    ))
                ));
                assert_eq!(executor.stack.ending.as_ref(), Some(&staged_before));
                assert_eq!(
                    executor.stack.last_support_receipt(),
                    receipt_before.as_ref()
                );
            };

            // The retained physical ending is part of the sealed authority,
            // including its provider cursor, daily state, and owner state.
            crate::v9_real_consumer_shadow::snow_free_physical_reuse::
                begin_snow_free_physical_reuse_audit_v1();
            let mut cursor_poison = make_armed();
            let ending = cursor_poison
                .ending
                .as_mut()
                .expect("provider-cursor poison ending");
            let poisoned_next_day = ending.inner.next_day_index + 1;
            let poisoned_cursor_bytes = serde_json::to_vec(&serde_json::json!({
                "next_day_index": poisoned_next_day,
                "configuration_sha256": ending.provider_static_configuration.configuration_sha256(),
                "pending_carry": [],
                "pending_solid_carry": [],
            }))
            .expect("provider-cursor poison bytes");
            ending.provider_cursor = SnowFreeHalfHourProviderCursor::restore_json(
                &poisoned_cursor_bytes,
                &ending.provider_static_configuration,
                poisoned_next_day,
            )
            .expect("different valid provider cursor");
            reject_without_mutation(cursor_poison);

            let mut state_poison = make_armed();
            let ending = state_poison
                .ending
                .as_mut()
                .expect("provider-state poison ending");
            ending.gsi_state = if ending.gsi_state.sample_count() == 0 {
                GsiState::try_from_history(
                    &[0.5],
                    Some(openwepp_plant_phenology::GsiDate {
                        year: 2020,
                        ordinal_day: 1,
                    }),
                )
                .expect("different valid GSI state")
            } else {
                GsiState::new()
            };
            reject_without_mutation(state_poison);

            let mut owner_poison = make_armed();
            owner_poison
                .ending
                .as_mut()
                .expect("owner poison ending")
                .inner
                .biogeochemistry
                .last_transaction_id += 1;
            reject_without_mutation(owner_poison);

            let mut binding_poison = make_armed();
            binding_poison
                .wb14_coupled_child_binding
                .as_mut()
                .expect("binding poison")
                .parent_support_end_ns += 1;
            reject_without_mutation(binding_poison);

            let ending_poison_audit =
                crate::v9_real_consumer_shadow::snow_free_physical_reuse::
                    take_snow_free_physical_reuse_audit_v1();
            assert_eq!(ending_poison_audit.physical_execution_count, 4);
            assert_eq!(ending_poison_audit.identity_reseal_count, 0);
            assert_eq!(ending_poison_audit.final_publication_append_count, 0);
            assert_eq!(ending_poison_audit.outer_accepted_publication_count, 0);
            assert_eq!(ending_poison_audit.provider_projection_count, 4);
            assert_eq!(ending_poison_audit.vapor_operation_count, 4);
            assert_eq!(ending_poison_audit.phase_operation_count, 4);
            assert_eq!(ending_poison_audit.ingress_operation_count, 4);
            assert_eq!(ending_poison_audit.wb14_operation_count, 4);
            assert_eq!(ending_poison_audit.routing_operation_count, 4);

            // Exhaust the remaining non-slab physical identity coordinates
            // through the real outer consumer. Each validly framed mutation
            // consumes the proof before reseal and preserves staged custody.
            crate::v9_real_consumer_shadow::snow_free_physical_reuse::
                begin_snow_free_physical_reuse_audit_v1();
            let mut parent_poison = make_armed();
            parent_poison
                .wb14_coupled_child_binding
                .as_mut()
                .expect("parent poison binding")
                .coupled_parent_transaction_sha256[0] ^= 1;
            reject_without_mutation(parent_poison);

            let mut support_poison = make_armed();
            support_poison
                .wb14_coupled_child_binding
                .as_mut()
                .expect("support poison binding")
                .child_support_end_ns += 1;
            reject_without_mutation(support_poison);

            let mut configuration_poison = make_armed();
            configuration_poison
                .beginning
                .vegetation_configuration
                .configuration_sha256 = "0".repeat(64);
            reject_without_mutation(configuration_poison);

            let mut topology_poison = make_armed();
            topology_poison
                .beginning
                .root_zone_hydraulic_configuration
                .ordered_layers
                .pop()
                .expect("topology poison layer");
            reject_without_mutation(topology_poison);

            let mut forcing_poison_interval = interval.clone();
            forcing_poison_interval.lse_forcing.air_temperature_k += 1.0;
            forcing_poison_interval.lse_forcing.forcing_sha256 = forcing_poison_interval
                .lse_forcing
                .canonical_sha256()
                .expect("valid forcing poison digest");
            let mut forcing_poison = make_armed();
            forcing_poison.interval = &forcing_poison_interval;
            reject_without_mutation(forcing_poison);

            let mut beginning_poison = make_armed();
            beginning_poison
                .beginning
                .vegetation_state
                .0
                .last_transaction_id += 1;
            reject_without_mutation(beginning_poison);

            let identity_poison_audit =
                crate::v9_real_consumer_shadow::snow_free_physical_reuse::
                    take_snow_free_physical_reuse_audit_v1();
            assert_eq!(identity_poison_audit.physical_execution_count, 6);
            assert_eq!(identity_poison_audit.identity_reseal_count, 0);
            assert_eq!(identity_poison_audit.final_publication_append_count, 0);
            assert_eq!(identity_poison_audit.outer_accepted_publication_count, 0);
            assert_eq!(identity_poison_audit.provider_projection_count, 6);
            assert_eq!(identity_poison_audit.vapor_operation_count, 6);
            assert_eq!(identity_poison_audit.phase_operation_count, 6);
            assert_eq!(identity_poison_audit.ingress_operation_count, 6);
            assert_eq!(identity_poison_audit.wb14_operation_count, 6);
            assert_eq!(identity_poison_audit.routing_operation_count, 6);

            // A final receipt from the same parent/support/owner set is still
            // foreign when either accepted constraint or ledger authority
            // differs from the receipt that authenticated the physical run.
            let foreign_constraint_receipt = accepted_snow_free_receipt_from_live_clock_v1(
                &live_clock,
                final_owner_states.clone(),
                END_NS,
                digest(15),
                digest(7),
            );
            let foreign_ledger_receipt = accepted_snow_free_receipt_from_live_clock_v1(
                &live_clock,
                final_owner_states.clone(),
                END_NS,
                digest(5),
                digest(16),
            );
            assert!(
                provisional_receipt
                    .shares_live_beginning_revision_with(&foreign_constraint_receipt)
            );
            assert!(
                provisional_receipt.shares_live_beginning_revision_with(&foreign_ledger_receipt)
            );
            crate::v9_real_consumer_shadow::snow_free_physical_reuse::
                begin_snow_free_physical_reuse_audit_v1();
            for (label, foreign_receipt) in [
                ("constraint", &foreign_constraint_receipt),
                ("ledger", &foreign_ledger_receipt),
            ] {
                let mut provisional_executor =
                    crate::v11_vegetation_consumer::DirectV11VegetationExecutor {
                        stack: DirectV11RealConsumerStack::new_parent_child(
                            &shadow,
                            &interval,
                            0,
                            0,
                            false,
                            provisional_binding,
                        ),
                    };
                execute_direct_v11_segment(
                    &migrated.configuration,
                    &parent,
                    &provisional_receipt,
                    &mut provisional_executor,
                )
                .unwrap_or_else(|error| panic!("{label} provisional physical execution: {error}"));
                let foreign_binding = snow_free_reuse_binding_v1(
                    parent_id,
                    foreign_receipt,
                    beginning_owner_sha256,
                    END_NS,
                );
                let armed = crate::v9_real_consumer_shadow::prepare_snow_free_physical_reuse(
                    provisional_executor.stack,
                    foreign_binding,
                )
                .unwrap_or_else(|error| panic!("{label} receipt arm: {error}"));
                let staged_before = armed
                    .ending
                    .clone()
                    .unwrap_or_else(|| panic!("{label} staged physical ending"));
                let support_before = armed.last_support_receipt().cloned();
                let mut foreign_executor =
                    crate::v11_vegetation_consumer::DirectV11VegetationExecutor { stack: armed };
                assert!(
                    matches!(
                        execute_direct_v11_segment(
                            &migrated.configuration,
                            &parent,
                            foreign_receipt,
                            &mut foreign_executor,
                        ),
                        Err(V11ExecutionError::Executor(
                            DirectV11RealConsumerError::Identity(
                                "snow-free physical reuse identity"
                            )
                        ))
                    ),
                    "foreign {label} receipt must typed-refuse",
                );
                assert_eq!(
                    foreign_executor.stack.ending.as_ref(),
                    Some(&staged_before),
                    "foreign {label} receipt must preserve staged ending bytes",
                );
                assert_eq!(
                    foreign_executor.stack.last_support_receipt(),
                    support_before.as_ref(),
                    "foreign {label} receipt must preserve staged support",
                );
            }
            let foreign_receipt_audit =
                crate::v9_real_consumer_shadow::snow_free_physical_reuse::
                    take_snow_free_physical_reuse_audit_v1();
            assert_eq!(foreign_receipt_audit.physical_execution_count, 2);
            assert_eq!(foreign_receipt_audit.identity_reseal_count, 0);
            assert_eq!(foreign_receipt_audit.final_publication_append_count, 0);
            assert_eq!(foreign_receipt_audit.outer_accepted_publication_count, 0);
            assert_eq!(foreign_receipt_audit.provider_projection_count, 2);
            assert_eq!(foreign_receipt_audit.vapor_operation_count, 2);
            assert_eq!(foreign_receipt_audit.phase_operation_count, 2);
            assert_eq!(foreign_receipt_audit.ingress_operation_count, 2);
            assert_eq!(foreign_receipt_audit.wb14_operation_count, 2);
            assert_eq!(foreign_receipt_audit.routing_operation_count, 2);

            // A same-live fork with identical non-ending authority cannot
            // relabel the retained physical ending as a different complete
            // owner candidate.
            let mut forked_ending_owner_states = final_owner_states.clone();
            let forked_owner = forked_ending_owner_states
                .first()
                .expect("forked ending owner")
                .clone();
            let mut forked_owner_bytes = forked_owner.state_bytes().to_vec();
            forked_owner_bytes.push(0);
            forked_ending_owner_states[0] = OwnerState::new(
                forked_owner.owner_id().to_owned(),
                forked_owner_bytes,
            )
            .expect("valid different ending owner");
            let forked_ending_receipt = accepted_snow_free_receipt_from_live_clock_v1(
                &live_clock,
                forked_ending_owner_states,
                END_NS,
                digest(5),
                digest(7),
            );
            assert!(
                provisional_receipt
                    .shares_live_beginning_revision_with(&forked_ending_receipt)
            );
            assert!(
                provisional_receipt.shares_nonending_context_with(&forked_ending_receipt)
            );
            assert!(
                !forked_ending_receipt
                    .authenticates_complete_ending_owners(&final_owner_states)
                    .expect("forked ending owner relation"),
            );
            crate::v9_real_consumer_shadow::snow_free_physical_reuse::
                begin_snow_free_physical_reuse_audit_v1();
            let mut forked_provisional_executor =
                crate::v11_vegetation_consumer::DirectV11VegetationExecutor {
                    stack: DirectV11RealConsumerStack::new_parent_child(
                        &shadow,
                        &interval,
                        0,
                        0,
                        false,
                        provisional_binding,
                    ),
                };
            execute_direct_v11_segment(
                &migrated.configuration,
                &parent,
                &provisional_receipt,
                &mut forked_provisional_executor,
            )
            .expect("forked-ending provisional physical execution");
            let forked_binding = snow_free_reuse_binding_v1(
                parent_id,
                &forked_ending_receipt,
                beginning_owner_sha256,
                END_NS,
            );
            let forked_armed = crate::v9_real_consumer_shadow::prepare_snow_free_physical_reuse(
                forked_provisional_executor.stack,
                forked_binding,
            )
            .expect("arm forked-ending refusal");
            let forked_ending_before = forked_armed
                .ending
                .clone()
                .expect("forked-ending staged physical ending");
            let forked_support_before = forked_armed.last_support_receipt().cloned();
            let mut forked_executor =
                crate::v11_vegetation_consumer::DirectV11VegetationExecutor {
                    stack: forked_armed,
                };
            assert!(matches!(
                execute_direct_v11_segment(
                    &migrated.configuration,
                    &parent,
                    &forked_ending_receipt,
                    &mut forked_executor,
                ),
                Err(V11ExecutionError::Executor(
                    DirectV11RealConsumerError::Identity(
                        "snow-free physical reuse final receipt ending owners"
                    )
                ))
            ));
            assert_eq!(
                forked_executor.stack.ending.as_ref(),
                Some(&forked_ending_before),
            );
            assert_eq!(
                forked_executor.stack.last_support_receipt(),
                forked_support_before.as_ref(),
            );
            assert!(matches!(
                execute_direct_v11_segment(
                    &migrated.configuration,
                    &parent,
                    &forked_ending_receipt,
                    &mut forked_executor,
                ),
                Err(V11ExecutionError::Executor(
                    DirectV11RealConsumerError::Identity("snow-free physical reuse single use")
                ))
            ));
            assert_eq!(
                forked_executor.stack.ending.as_ref(),
                Some(&forked_ending_before),
            );
            assert_eq!(
                forked_executor.stack.last_support_receipt(),
                forked_support_before.as_ref(),
            );
            let forked_ending_audit =
                crate::v9_real_consumer_shadow::snow_free_physical_reuse::
                    take_snow_free_physical_reuse_audit_v1();
            assert_eq!(forked_ending_audit.physical_execution_count, 1);
            assert_eq!(forked_ending_audit.identity_reseal_count, 0);
            assert_eq!(forked_ending_audit.final_publication_append_count, 0);
            assert_eq!(forked_ending_audit.outer_accepted_publication_count, 0);

            // Durable receipt bytes from an independently constructed clock
            // are insufficient: only the same process-local live revision
            // can authorize the physical result.
            let (independent_parent, independent_clock) =
                snow_free_reuse_live_clock_v1(&beginning_owner_states, END_NS);
            assert_eq!(independent_parent, parent_id);
            let independent_receipt = accepted_snow_free_receipt_from_live_clock_v1(
                &independent_clock,
                final_owner_states.clone(),
                END_NS,
                digest(5),
                digest(7),
            );
            assert_eq!(
                serde_json::to_vec(&independent_receipt).expect("independent receipt bytes"),
                serde_json::to_vec(&final_receipt).expect("live receipt bytes"),
                "independent receipt must differ only in non-wire live authority",
            );
            assert!(
                !provisional_receipt.shares_live_beginning_revision_with(&independent_receipt),
                "independent clock incarnation must not share live authority",
            );
            crate::v9_real_consumer_shadow::snow_free_physical_reuse::
                begin_snow_free_physical_reuse_audit_v1();
            let mut independent_provisional_executor =
                crate::v11_vegetation_consumer::DirectV11VegetationExecutor {
                    stack: DirectV11RealConsumerStack::new_parent_child(
                        &shadow,
                        &interval,
                        0,
                        0,
                        false,
                        provisional_binding,
                    ),
                };
            execute_direct_v11_segment(
                &migrated.configuration,
                &parent,
                &provisional_receipt,
                &mut independent_provisional_executor,
            )
            .expect("independent-incarnation provisional physical execution");
            let independent_binding = snow_free_reuse_binding_v1(
                parent_id,
                &independent_receipt,
                beginning_owner_sha256,
                END_NS,
            );
            let independent_armed =
                crate::v9_real_consumer_shadow::prepare_snow_free_physical_reuse(
                    independent_provisional_executor.stack,
                    independent_binding,
                )
                .expect("arm independent-incarnation refusal");
            let independent_ending_before = independent_armed
                .ending
                .clone()
                .expect("independent-incarnation staged ending");
            let independent_support_before = independent_armed.last_support_receipt().cloned();
            let mut independent_executor =
                crate::v11_vegetation_consumer::DirectV11VegetationExecutor {
                    stack: independent_armed,
                };
            assert!(matches!(
                execute_direct_v11_segment(
                    &migrated.configuration,
                    &parent,
                    &independent_receipt,
                    &mut independent_executor,
                ),
                Err(V11ExecutionError::Executor(
                    DirectV11RealConsumerError::Identity("snow-free physical reuse identity")
                ))
            ));
            assert_eq!(
                independent_executor.stack.ending.as_ref(),
                Some(&independent_ending_before),
            );
            assert_eq!(
                independent_executor.stack.last_support_receipt(),
                independent_support_before.as_ref(),
            );
            let independent_audit =
                crate::v9_real_consumer_shadow::snow_free_physical_reuse::
                    take_snow_free_physical_reuse_audit_v1();
            assert_eq!(independent_audit.physical_execution_count, 1);
            assert_eq!(independent_audit.identity_reseal_count, 0);
            assert_eq!(independent_audit.final_publication_append_count, 0);
            assert_eq!(independent_audit.outer_accepted_publication_count, 0);
            assert_eq!(independent_audit.provider_projection_count, 1);
            assert_eq!(independent_audit.vapor_operation_count, 1);
            assert_eq!(independent_audit.phase_operation_count, 1);
            assert_eq!(independent_audit.ingress_operation_count, 1);
            assert_eq!(independent_audit.wb14_operation_count, 1);
            assert_eq!(independent_audit.routing_operation_count, 1);

            // A provisional outer-auth failure restores the exact
            // pre-execution stack and installs a refusal tombstone before the
            // fallible mint. Retry must not enter the physical producer.
            crate::v9_real_consumer_shadow::snow_free_physical_reuse::
                begin_snow_free_physical_reuse_audit_v1();
            let mut provisional_fault_executor =
                crate::v11_vegetation_consumer::DirectV11VegetationExecutor {
                    stack: DirectV11RealConsumerStack::new_parent_child(
                        &shadow,
                        &interval,
                        0,
                        0,
                        false,
                        provisional_binding,
                    ),
                };
            let provisional_fault_ending_before = provisional_fault_executor.stack.ending.clone();
            let provisional_fault_support_before = provisional_fault_executor
                .stack
                .last_support_receipt()
                .cloned();
            crate::v9_real_consumer_shadow::snow_free_physical_reuse::
                inject_snow_free_outer_auth_failure_v1();
            assert!(matches!(
                execute_direct_v11_segment(
                    &migrated.configuration,
                    &parent,
                    &provisional_receipt,
                    &mut provisional_fault_executor,
                ),
                Err(V11ExecutionError::Executor(
                    DirectV11RealConsumerError::Identity(
                        "injected snow-free outer authentication failure"
                    )
                ))
            ));
            assert_eq!(
                provisional_fault_executor.stack.ending,
                provisional_fault_ending_before,
            );
            assert_eq!(
                provisional_fault_executor.stack.last_support_receipt(),
                provisional_fault_support_before.as_ref(),
            );
            assert!(matches!(
                execute_direct_v11_segment(
                    &migrated.configuration,
                    &parent,
                    &provisional_receipt,
                    &mut provisional_fault_executor,
                ),
                Err(V11ExecutionError::Executor(
                    DirectV11RealConsumerError::Identity("snow-free physical reuse single use")
                ))
            ));
            assert_eq!(
                provisional_fault_executor.stack.ending,
                provisional_fault_ending_before,
            );
            assert_eq!(
                provisional_fault_executor.stack.last_support_receipt(),
                provisional_fault_support_before.as_ref(),
            );
            let provisional_fault_audit =
                crate::v9_real_consumer_shadow::snow_free_physical_reuse::
                    take_snow_free_physical_reuse_audit_v1();
            assert_eq!(provisional_fault_audit.physical_execution_count, 1);
            assert_eq!(provisional_fault_audit.identity_reseal_count, 0);
            assert_eq!(provisional_fault_audit.final_publication_append_count, 0);
            assert_eq!(provisional_fault_audit.outer_accepted_publication_count, 0);
            assert_eq!(provisional_fault_audit.provider_projection_count, 1);
            assert_eq!(provisional_fault_audit.vapor_operation_count, 1);
            assert_eq!(provisional_fault_audit.phase_operation_count, 1);
            assert_eq!(provisional_fault_audit.ingress_operation_count, 1);
            assert_eq!(provisional_fault_audit.wb14_operation_count, 1);
            assert_eq!(provisional_fault_audit.routing_operation_count, 1);

            // Failure after identity reseal but before parent acceptance must
            // restore the provisional staged owner/support exactly and leave
            // a permanent refusal tombstone. A retry cannot replay physics.
            crate::v9_real_consumer_shadow::snow_free_physical_reuse::
                begin_snow_free_physical_reuse_audit_v1();
            let outer_fault_armed = make_armed();
            let outer_fault_ending_before = outer_fault_armed
                .ending
                .clone()
                .expect("outer-fault staged physical ending");
            let outer_fault_support_before = outer_fault_armed.last_support_receipt().cloned();
            let mut outer_fault_executor =
                crate::v11_vegetation_consumer::DirectV11VegetationExecutor {
                    stack: outer_fault_armed,
                };
            crate::v9_real_consumer_shadow::snow_free_physical_reuse::
                inject_snow_free_outer_auth_failure_v1();
            assert!(matches!(
                execute_direct_v11_segment(
                    &migrated.configuration,
                    &parent,
                    &final_receipt,
                    &mut outer_fault_executor,
                ),
                Err(V11ExecutionError::Executor(
                    DirectV11RealConsumerError::Identity(
                        "injected snow-free outer authentication failure"
                    )
                ))
            ));
            assert_eq!(
                outer_fault_executor.stack.ending.as_ref(),
                Some(&outer_fault_ending_before),
                "outer-auth failure must restore staged ending bytes",
            );
            assert_eq!(
                outer_fault_executor.stack.last_support_receipt(),
                outer_fault_support_before.as_ref(),
                "outer-auth failure must restore staged support",
            );
            assert!(matches!(
                execute_direct_v11_segment(
                    &migrated.configuration,
                    &parent,
                    &final_receipt,
                    &mut outer_fault_executor,
                ),
                Err(V11ExecutionError::Executor(
                    DirectV11RealConsumerError::Identity("snow-free physical reuse single use")
                ))
            ));
            assert_eq!(
                outer_fault_executor.stack.ending.as_ref(),
                Some(&outer_fault_ending_before),
            );
            assert_eq!(
                outer_fault_executor.stack.last_support_receipt(),
                outer_fault_support_before.as_ref(),
            );
            let outer_fault_audit =
                crate::v9_real_consumer_shadow::snow_free_physical_reuse::
                    take_snow_free_physical_reuse_audit_v1();
            assert_eq!(outer_fault_audit.physical_execution_count, 1);
            assert_eq!(outer_fault_audit.identity_reseal_count, 1);
            assert_eq!(outer_fault_audit.final_publication_append_count, 0);
            assert_eq!(outer_fault_audit.outer_accepted_publication_count, 0);
            assert_eq!(outer_fault_audit.provider_projection_count, 1);
            assert_eq!(outer_fault_audit.vapor_operation_count, 1);
            assert_eq!(outer_fault_audit.phase_operation_count, 1);
            assert_eq!(outer_fault_audit.ingress_operation_count, 1);
            assert_eq!(outer_fault_audit.wb14_operation_count, 1);
            assert_eq!(outer_fault_audit.routing_operation_count, 1);

            // Any error after custody is moved into the identity-only reseal
            // restores the exact pre-final staged ending and support while
            // leaving a permanent refusal tombstone.
            crate::v9_real_consumer_shadow::snow_free_physical_reuse::
                begin_snow_free_physical_reuse_audit_v1();
            let post_take_armed = make_armed();
            let post_take_ending_before = post_take_armed
                .ending
                .clone()
                .expect("post-take staged physical ending");
            let post_take_support_before = post_take_armed.last_support_receipt().cloned();
            let mut post_take_executor =
                crate::v11_vegetation_consumer::DirectV11VegetationExecutor {
                    stack: post_take_armed,
                };
            crate::v9_real_consumer_shadow::snow_free_physical_reuse::
                inject_snow_free_post_take_failure_v1();
            assert!(matches!(
                execute_direct_v11_segment(
                    &migrated.configuration,
                    &parent,
                    &final_receipt,
                    &mut post_take_executor,
                ),
                Err(V11ExecutionError::Executor(
                    DirectV11RealConsumerError::Identity(
                        "injected snow-free post-take failure"
                    )
                ))
            ));
            assert_eq!(
                post_take_executor.stack.ending.as_ref(),
                Some(&post_take_ending_before),
            );
            assert_eq!(
                post_take_executor.stack.last_support_receipt(),
                post_take_support_before.as_ref(),
            );
            assert!(matches!(
                execute_direct_v11_segment(
                    &migrated.configuration,
                    &parent,
                    &final_receipt,
                    &mut post_take_executor,
                ),
                Err(V11ExecutionError::Executor(
                    DirectV11RealConsumerError::Identity("snow-free physical reuse single use")
                ))
            ));
            assert_eq!(
                post_take_executor.stack.ending.as_ref(),
                Some(&post_take_ending_before),
            );
            assert_eq!(
                post_take_executor.stack.last_support_receipt(),
                post_take_support_before.as_ref(),
            );
            let post_take_audit =
                crate::v9_real_consumer_shadow::snow_free_physical_reuse::
                    take_snow_free_physical_reuse_audit_v1();
            assert_eq!(post_take_audit.physical_execution_count, 1);
            assert_eq!(post_take_audit.identity_reseal_count, 0);
            assert_eq!(post_take_audit.final_publication_append_count, 0);
            assert_eq!(post_take_audit.outer_accepted_publication_count, 0);
            assert_eq!(post_take_audit.provider_projection_count, 1);
            assert_eq!(post_take_audit.vapor_operation_count, 1);
            assert_eq!(post_take_audit.phase_operation_count, 1);
            assert_eq!(post_take_audit.ingress_operation_count, 1);
            assert_eq!(post_take_audit.wb14_operation_count, 1);
            assert_eq!(post_take_audit.routing_operation_count, 1);

            // Moving the staged ending invalidates the capability even if a
            // caller puts the identical bytes back into the stack.
            let mut move_restore = make_armed();
            let moved = move_restore
                .take_staged_ending()
                .expect("move-out staged ending");
            move_restore.ending = Some(moved);
            let mut move_restore_executor =
                crate::v11_vegetation_consumer::DirectV11VegetationExecutor {
                    stack: move_restore,
                };
            assert!(matches!(
                execute_direct_v11_segment(
                    &migrated.configuration,
                    &parent,
                    &final_receipt,
                    &mut move_restore_executor,
                ),
                Err(V11ExecutionError::Executor(
                    DirectV11RealConsumerError::Identity("snow-free physical reuse single use")
                ))
            ));

            let stale = crate::v9_real_consumer_shadow::prepare_snow_free_physical_reuse(
                crate::v11_vegetation_consumer::DirectV11VegetationExecutor {
                    stack: DirectV11RealConsumerStack::new_parent_child(
                        &shadow,
                        &interval,
                        0,
                        0,
                        false,
                        provisional_binding,
                    ),
                }
                .stack,
                final_binding,
            );
            assert!(
                stale.is_err(),
                "a stack without a minted proof must reject reuse"
            );

            // A mutated execution context consumes the move-only proof but
            // cannot alter the already-staged physical owner set or append a
            // publication. Recovery therefore requires a fresh physical run.
            crate::v9_real_consumer_shadow::snow_free_physical_reuse::
                begin_snow_free_physical_reuse_audit_v1();
            let mut mutated_provisional_executor =
                crate::v11_vegetation_consumer::DirectV11VegetationExecutor {
                    stack: DirectV11RealConsumerStack::new_parent_child(
                        &shadow,
                        &interval,
                        0,
                        0,
                        false,
                        provisional_binding,
                    ),
                };
            execute_direct_v11_segment(
                &migrated.configuration,
                &parent,
                &provisional_receipt,
                &mut mutated_provisional_executor,
            )
            .expect("mutated-context provisional physical execution");
            let mut mutated = crate::v9_real_consumer_shadow::prepare_snow_free_physical_reuse(
                mutated_provisional_executor.stack,
                final_binding,
            )
            .expect("arm mutated-context identity reseal");
            let staged_before_rejection = mutated
                .ending
                .clone()
                .expect("staged physical owner before rejection");
            let receipt_before_rejection = mutated.last_support_receipt().cloned();
            mutated.day_index = 1;
            let mut mutated_executor =
                crate::v11_vegetation_consumer::DirectV11VegetationExecutor { stack: mutated };
            assert!(matches!(
                execute_direct_v11_segment(
                    &migrated.configuration,
                    &parent,
                    &final_receipt,
                    &mut mutated_executor,
                ),
                Err(V11ExecutionError::Executor(
                    DirectV11RealConsumerError::Identity("snow-free physical reuse identity")
                ))
            ));
            assert_eq!(
                mutated_executor.stack.ending.as_ref(),
                Some(&staged_before_rejection),
                "failed reseal must roll back staged physical owners byte-for-byte",
            );
            assert_eq!(
                mutated_executor.stack.last_support_receipt(),
                receipt_before_rejection.as_ref(),
                "failed reseal must not replace publication support",
            );
            let rejected_audit = crate::v9_real_consumer_shadow::snow_free_physical_reuse::
                take_snow_free_physical_reuse_audit_v1();
            assert_eq!(rejected_audit.physical_execution_count, 1);
            assert_eq!(rejected_audit.identity_reseal_count, 0);
            assert_eq!(rejected_audit.final_publication_append_count, 0);
            assert_eq!(rejected_audit.outer_accepted_publication_count, 0);
            assert_eq!(rejected_audit.provider_projection_count, 1);
            assert_eq!(rejected_audit.vapor_operation_count, 1);
            assert_eq!(rejected_audit.phase_operation_count, 1);
            assert_eq!(rejected_audit.ingress_operation_count, 1);
            assert_eq!(rejected_audit.wb14_operation_count, 1);
            assert_eq!(rejected_audit.routing_operation_count, 1);
        })
        .expect("spawn direct snow-free identity reseal")
        .join()
        .expect("join direct snow-free identity reseal");
}

#[test]
#[allow(clippy::too_many_lines)]
fn coupled_hard_boundary_truncates_selected_900_second_child() {
    exercise_complete_wb14_cadence(
        0.02,
        8.0,
        false,
        Some(60_000_000_000),
        false,
        None,
        false,
        false,
        false,
        false,
    );
}

#[test]
#[allow(clippy::too_many_lines)]
fn latest_accepted_stage3_state_changes_next_wb14_proposal() {
    exercise_complete_wb14_cadence(
        0.010_000_001,
        0.0,
        false,
        Some(60_000_000_000),
        true,
        None,
        false,
        false,
        false,
        false,
    );
}

#[test]
#[allow(clippy::too_many_lines)]
fn resolved_snow_and_snow_free_lanes_publish_one_atomic_parent() {
    std::thread::Builder::new()
        .name("resolved-and-snow-free-lanes".to_owned())
        .stack_size(64 * 1024 * 1024)
        .spawn(|| {
            exercise_complete_wb14_cadence(
                0.08,
                8.0,
                false,
                None,
                false,
                Some(0.0),
                false,
                false,
                false,
                false,
            );
        })
        .expect("spawn resolved/snow-free lane fixture")
        .join()
        .expect("join resolved/snow-free lane fixture");
}

#[test]
#[allow(clippy::too_many_lines)]
fn two_resolved_snow_lanes_choose_common_earliest_cadence() {
    std::thread::Builder::new()
        .name("two-resolved-snow-lanes".to_owned())
        .stack_size(64 * 1024 * 1024)
        .spawn(|| {
            exercise_complete_wb14_cadence(
                0.08,
                8.0,
                true,
                None,
                false,
                Some(0.005),
                false,
                false,
                false,
                false,
            );
        })
        .expect("spawn two resolved snow-lane fixture")
        .join()
        .expect("join two resolved snow-lane fixture");
}

#[test]
#[allow(clippy::too_many_lines)]
fn interior_terminal_event_runs_covered_event_and_snow_free_remainder() {
    std::thread::Builder::new()
        .name("interior-terminal-event-owner-path".to_owned())
        .stack_size(64 * 1024 * 1024)
        .spawn(|| {
            exercise_complete_wb14_cadence(
                0.000_6, 0.0, false, None, false, None, true, false, false, false,
            );
        })
        .expect("spawn interior terminal-event fixture")
        .join()
        .expect("join interior terminal-event fixture");
}

#[test]
fn solid_precipitation_reappears_through_the_adaptive_owner_path() {
    std::thread::Builder::new()
        .name("solid-reappearance-owner-path".to_owned())
        .stack_size(64 * 1024 * 1024)
        .spawn(|| {
            exercise_complete_wb14_cadence(
                0.0, 0.0, false, None, false, None, false, false, true, false,
            );
        })
        .expect("spawn solid-reappearance fixture")
        .join()
        .expect("join solid-reappearance fixture");
}

pub(crate) mod adaptive_production_path_coverage {
    include!("snow_stage3_v11_adaptive_production_tests.rs");
}

#[test]
fn interior_terminal_event_capture_reproduces_below_carrier_domain() {
    std::thread::Builder::new()
        .name("child1-real-discrete-fixture".to_owned())
        .stack_size(64 * 1024 * 1024)
        .spawn(|| {
            exercise_complete_wb14_cadence(
                0.000_6, 0.0, false, None, false, None, true, true, false, false,
            );
        })
        .expect("spawn Child-1 real discrete fixture")
        .join()
        .expect("join Child-1 real discrete fixture");
}

#[allow(
    clippy::cast_possible_truncation,
    clippy::cast_sign_loss,
    clippy::too_many_lines,
    reason = "test-only exact event ticks are checked finite, positive, integral, and parent-bounded before conversion"
)]
#[inline(never)]
fn run_real_discrete_endpoint_probes(
    shadow: &DirectV10RealConsumerShadow,
    beginning_clock: &CoupledClockStateV1,
    prepared: &PreparedStage3V11SupportV1,
    stage3_beginning_by_lane: &BTreeMap<u32, DirectSnowStage3PersistentState>,
    selected_seconds: f64,
) {
    const TERMINAL_ENERGY_COMPARISON_TOLERANCE_J_M2: f64 = 1.0e-6;
    let beginning_terminal_parcels = BTreeMap::new();
    let classify =
        |endpoint: &crate::snow_stage3_v11_attachment::RealDiscreteCompleteEndpointEvidenceV1| {
            use crate::discrete_terminal_support_root::EndpointTerminalClass;
            let duration_s = f64::from_bits(endpoint.support.duration_s_bits());
            let evaluated_s = f64::from_bits(endpoint.event_evaluated_seconds_bits);
            let event_offset_s = f64::from_bits(endpoint.event_hour_offset_seconds_bits);
            let unevaluated_s = f64::from_bits(endpoint.event_unevaluated_seconds_bits);
            let admissible_terminal_ledger =
                f64::from_bits(endpoint.terminal_unallocated_energy_bits)
                    <= TERMINAL_ENERGY_COMPARISON_TOLERANCE_J_M2;
            if endpoint.event_occurred
                && evaluated_s.to_bits() == duration_s.to_bits()
                && event_offset_s.to_bits() == duration_s.to_bits()
                && unevaluated_s <= 1.0e-6
                && endpoint.end_ice_bits == 0.0_f64.to_bits()
                && admissible_terminal_ledger
            {
                EndpointTerminalClass::TerminalAtEndpoint
            } else if endpoint.event_occurred
                && event_offset_s > 0.0
                && event_offset_s < duration_s
                && endpoint.end_ice_bits == 0.0_f64.to_bits()
                && admissible_terminal_ledger
            {
                let event_ns = event_offset_s * 1_000_000_000.0;
                if event_ns.is_finite() && event_ns.fract() == 0.0 {
                    EndpointTerminalClass::CrossedTerminal {
                        event_tick: ModelTimeNs::new(
                            endpoint.support.start_ns().get() + event_ns as u128,
                        ),
                    }
                } else {
                    EndpointTerminalClass::Invalid
                }
            } else if !endpoint.event_occurred
                && f64::from_bits(endpoint.terminal_unallocated_energy_bits)
                    <= TERMINAL_ENERGY_COMPARISON_TOLERANCE_J_M2
                && f64::from_bits(endpoint.end_ice_bits) > 0.0
            {
                EndpointTerminalClass::PreTerminal
            } else {
                EndpointTerminalClass::Invalid
            }
        };
    let mut evaluate = |endpoint_tick: u128| {
        let endpoint =
            crate::snow_stage3_v11_attachment::evaluate_real_discrete_complete_endpoint_v1(
                shadow,
                beginning_clock,
                prepared,
                0,
                0,
                stage3_beginning_by_lane,
                &beginning_terminal_parcels,
                selected_seconds,
                1,
                1,
                ModelTimeNs::new(endpoint_tick),
            )
            .expect("real discrete complete endpoint probe");
        eprintln!(
            "CHILD1_REAL_DISCRETE_EVALUATED tick={} class={:?} event={} ice_bits={:#018x} liquid_bits={:#018x} deposition_bits={:#018x} melt_bits={:#018x} unallocated_bits={:#018x} energy_closure_bits={:#018x} ice_closure_bits={:#018x} water_closure_bits={:#018x} owner_count={}",
            endpoint_tick,
            classify(&endpoint),
            endpoint.event_occurred,
            endpoint.end_ice_bits,
            endpoint.end_liquid_bits,
            endpoint.deposition_bits,
            endpoint.melt_bits,
            endpoint.terminal_unallocated_energy_bits,
            endpoint.energy_closure_residual_bits,
            endpoint.ice_closure_residual_bits,
            endpoint.water_closure_residual_bits,
            endpoint.owner_count,
        );
        endpoint
    };
    let find_first_non_preterminal = |mut lower: u128,
                                      mut upper: u128,
                                      evaluate: &mut dyn FnMut(u128) -> crate::snow_stage3_v11_attachment::RealDiscreteCompleteEndpointEvidenceV1| {
        assert_eq!(classify(&evaluate(lower)), crate::discrete_terminal_support_root::EndpointTerminalClass::PreTerminal);
        assert_eq!(classify(&evaluate(upper)), crate::discrete_terminal_support_root::EndpointTerminalClass::Invalid);
        while lower + 1 < upper {
            let middle = lower + (upper - lower) / 2;
            if classify(&evaluate(middle))
                == crate::discrete_terminal_support_root::EndpointTerminalClass::PreTerminal
            {
                lower = middle;
            } else {
                upper = middle;
            }
        }
        upper
    };
    let first = find_first_non_preterminal(60_000_000_000, 900_000_000_000, &mut evaluate);
    let second = find_first_non_preterminal(93_750_000_000, 1_799_999_999_999, &mut evaluate);
    assert_eq!(
        first, second,
        "material comparison must be bracket-independent"
    );
    let mut typed_batch_endpoint = |tick: ModelTimeNs| {
        let candidate = evaluate(tick.get());
        Ok(
            crate::discrete_terminal_support_root::BatchEndpointEvaluation {
                tick,
                lane_classes: BTreeMap::from([(1, classify(&candidate))]),
                candidate: Some(candidate),
            },
        )
    };
    assert_eq!(
        crate::discrete_terminal_support_root::integer_bisection(
            ModelTimeNs::new(0),
            beginning_clock.parent_support().end_ns(),
            ModelTimeNs::new(60_000_000_000),
            ModelTimeNs::new(900_000_000_000),
            None,
            &mut typed_batch_endpoint,
        ),
        Err(crate::discrete_terminal_support_root::DiscreteRootError::InvalidEndpoint),
        "the batch-shaped real endpoint must return a typed failure rather than a root"
    );
    let selected = first;
    let mut boundary_candidates = Vec::new();
    for tick in [selected - 1, selected, selected + 1] {
        let endpoint = evaluate(tick);
        assert_ne!(
            classify(&endpoint),
            crate::discrete_terminal_support_root::EndpointTerminalClass::TerminalAtEndpoint
        );
        assert!(!endpoint.event_occurred);
        assert!(f64::from_bits(endpoint.end_ice_bits) > 0.0);
        boundary_candidates.push((tick, endpoint));
    }
    let previous = &boundary_candidates[0].1;
    let candidate = &boundary_candidates[1].1;
    let next = &boundary_candidates[2].1;
    assert_eq!(
        classify(previous),
        crate::discrete_terminal_support_root::EndpointTerminalClass::PreTerminal
    );
    assert_eq!(
        classify(candidate),
        crate::discrete_terminal_support_root::EndpointTerminalClass::Invalid
    );
    assert_eq!(
        classify(next),
        crate::discrete_terminal_support_root::EndpointTerminalClass::Invalid
    );
    assert!(
        f64::from_bits(candidate.terminal_unallocated_energy_bits)
            > TERMINAL_ENERGY_COMPARISON_TOLERANCE_J_M2
    );
    assert_eq!(candidate.end_ice_bits, candidate.deposition_bits);
    assert_eq!(candidate.melt_bits, 0.6_f64.to_bits());
    let competition_inputs =
        crate::snow_terminal_phase_competition::inputs_from_real_endpoint(candidate);
    let complementarity =
        crate::snow_terminal_phase_competition::simultaneous_complementarity(competition_inputs)
            .expect("real complementarity allocation");
    let residual_frost =
        crate::snow_terminal_phase_competition::residual_surface_frost(competition_inputs)
            .expect("real residual-frost allocation");
    assert!(
        complementarity.ending_pack_ice_kg_m2 <= 1.0e-9
            || complementarity.unallocated_energy_j_m2 <= 1.0e-6
    );
    assert_eq!(
        residual_frost.ending_pack_ice_kg_m2.to_bits(),
        0.0_f64.to_bits()
    );
    assert_eq!(
        residual_frost.ending_surface_frost_kg_m2.to_bits(),
        candidate.deposition_bits
    );
    let evaluate_complete_complementarity = |tick: u128| {
        crate::snow_stage3_v11_attachment::evaluate_real_phase_complementarity_endpoint_v1(
            shadow,
            beginning_clock,
            prepared,
            0,
            0,
            stage3_beginning_by_lane,
            &beginning_terminal_parcels,
            selected_seconds,
            1,
            1,
            ModelTimeNs::new(tick),
        )
        .expect("real complete-owner phase-complementarity endpoint")
    };
    let complete_complementarity_boundary = evaluate_complete_complementarity(selected);
    assert_eq!(
        complete_complementarity_boundary.end_ice_bits,
        complementarity.ending_pack_ice_kg_m2.to_bits()
    );
    assert_eq!(
        complete_complementarity_boundary.end_liquid_bits,
        complementarity.ending_liquid_kg_m2.to_bits()
    );
    assert_eq!(
        complete_complementarity_boundary.terminal_unallocated_energy_bits,
        0.0_f64.to_bits()
    );
    let trajectory_beginning = crate::snow_terminal_phase_trajectory::TrajectoryState {
        pack_ice_kg_m2: f64::from_bits(candidate.start_ice_bits),
        surface_frost_kg_m2: 0.0,
        liquid_kg_m2: f64::from_bits(candidate.start_liquid_bits),
        cold_content_j_m2: f64::from_bits(candidate.start_cold_content_bits),
    };
    let trajectory_segment =
        crate::snow_terminal_phase_trajectory::segment_from_real_endpoint(candidate);
    let released_trajectory = crate::snow_terminal_phase_trajectory::released_ordered_trajectory(
        trajectory_beginning,
        &[trajectory_segment],
    )
    .expect("real released-order trajectory");
    let frost_hybrid = crate::snow_terminal_phase_trajectory::event_driven_frost_hybrid(
        trajectory_beginning,
        &[trajectory_segment],
    )
    .expect("real frost-hybrid trajectory");
    let resolved_complementarity =
        crate::snow_terminal_phase_trajectory::time_resolved_complementarity(
            trajectory_beginning,
            &[trajectory_segment],
        )
        .expect("real time-resolved complementarity");
    let (tagged_frost, tagged_envelope) =
        crate::snow_terminal_phase_trajectory::existing_snow_frost_subtype(
            trajectory_beginning,
            &[trajectory_segment],
        )
        .expect("real tagged-frost trajectory");
    assert_eq!(released_trajectory.events.len(), 2);
    assert_eq!(
        released_trajectory.ending.pack_ice_kg_m2.to_bits(),
        candidate.deposition_bits
    );
    assert_eq!(
        frost_hybrid.ending.surface_frost_kg_m2.to_bits(),
        0.0_f64.to_bits()
    );
    assert_eq!(
        frost_hybrid.ending.pack_ice_kg_m2.to_bits(),
        complementarity.ending_pack_ice_kg_m2.to_bits()
    );
    assert_eq!(
        resolved_complementarity.ending.pack_ice_kg_m2.to_bits(),
        complementarity.ending_pack_ice_kg_m2.to_bits()
    );
    assert_eq!(tagged_frost.ending, frost_hybrid.ending);
    assert_eq!(
        crate::snow_terminal_phase_trajectory::TaggedSnowOwnerEnvelope::restore(
            &tagged_envelope.canonical_bytes(),
        ),
        Ok(tagged_envelope)
    );
    let expected_latent_heat =
        openwepp_meteorology::surface_energy::latent_heat_for_surface_temperature(
            openwepp_unit_boundary::TemperatureCelsius::try_new(0.0)
                .expect("terminal surface temperature"),
        )
        .expect("terminal latent heat")
        .as_joules_per_kilogram();
    assert!(
        (trajectory_segment.latent_heat_j_kg - expected_latent_heat).abs()
            <= 1.0e-9 * expected_latent_heat
    );
    assert_eq!(
        crate::snow_terminal_phase_trajectory::released_ordered_trajectory(
            trajectory_beginning,
            &[trajectory_segment],
        )
        .expect("real trajectory replay"),
        released_trajectory
    );
    for energy_delta_j_m2 in [-1.0e-3, -1.0e-6, 0.0, 1.0e-6, 1.0e-3] {
        let perturbed = crate::snow_terminal_phase_trajectory::ForcingSegment {
            complete_energy_j_m2: trajectory_segment.complete_energy_j_m2 + energy_delta_j_m2,
            ..trajectory_segment
        };
        crate::snow_terminal_phase_trajectory::released_ordered_trajectory(
            trajectory_beginning,
            &[perturbed],
        )
        .expect("nearby released-order forcing");
        crate::snow_terminal_phase_trajectory::event_driven_frost_hybrid(
            trajectory_beginning,
            &[perturbed],
        )
        .expect("nearby frost-hybrid forcing");
        crate::snow_terminal_phase_trajectory::time_resolved_complementarity(
            trajectory_beginning,
            &[perturbed],
        )
        .expect("nearby resolved-complementarity forcing");
    }
    for energy_delta_j_m2 in [-1.0e-3, -1.0e-6, 0.0, 1.0e-6, 1.0e-3] {
        let perturbed = crate::snow_terminal_phase_competition::TerminalPhaseInputs {
            non_vapor_energy_j_m2: competition_inputs.non_vapor_energy_j_m2 + energy_delta_j_m2,
            complete_energy_j_m2: competition_inputs.complete_energy_j_m2 + energy_delta_j_m2,
            ..competition_inputs
        };
        crate::snow_terminal_phase_competition::simultaneous_complementarity(perturbed)
            .expect("nearby real-energy perturbation");
    }
    let parent_endpoint = evaluate(900_000_000_000);
    let parent_complementarity =
        crate::snow_terminal_phase_competition::simultaneous_complementarity(
            crate::snow_terminal_phase_competition::inputs_from_real_endpoint(&parent_endpoint),
        )
        .expect("real parent-end complementarity allocation");
    assert_eq!(
        parent_complementarity.event,
        crate::snow_terminal_phase_competition::TerminalEventChronology::Interior
    );
    assert!(parent_complementarity.ending_pack_ice_kg_m2 <= 1.0e-9);
    let complete_complementarity_parent = evaluate_complete_complementarity(900_000_000_000);
    assert!(complete_complementarity_parent.event_occurred);
    assert_eq!(
        f64::from_bits(complete_complementarity_parent.end_ice_bits),
        0.0
    );
    assert_eq!(
        complete_complementarity_parent.end_liquid_bits,
        parent_complementarity.ending_liquid_kg_m2.to_bits()
    );
    assert_eq!(
        evaluate(selected),
        *candidate,
        "exact endpoint replay must be byte-identical"
    );
    eprintln!(
        "CHILD1_TERMINAL_PHASE_COMPETITION tick={selected} complementarity={complementarity:?} residual_frost={residual_frost:?} released_trajectory={released_trajectory:?} frost_hybrid={frost_hybrid:?} resolved_complementarity={resolved_complementarity:?} tagged_frost={tagged_frost:?} parent_end={parent_complementarity:?}",
    );
}
