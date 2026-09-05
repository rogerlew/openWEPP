fn two_lane_stage3_endpoint_fixture() -> EndpointFixture {
    let mut fixture = two_ofe_routed_endpoint_fixture();
    let lower_ofe = OfeId::try_new("ofe-2").expect("lower OFE");
    let lower_open = TileId::try_new("lower-open").expect("lower open tile");
    let mut surface_records = fixture.surface_configuration.records.clone();
    surface_records
        .retain(|record| record.key.ofe_id != lower_ofe || record.key.tile_id == lower_open);
    surface_records
        .iter_mut()
        .find(|record| record.key.ofe_id == lower_ofe)
        .expect("lower open surface record")
        .tile_fraction = 1.0;
    fixture.surface_configuration = DirectSurfaceLiquidConfiguration::new(
        fixture.surface_configuration.owner_id.clone(),
        fixture.surface_configuration.run_id,
        fixture.surface_configuration.ofe_topology.clone(),
        fixture.surface_configuration.ofe_bindings.clone(),
        surface_records,
    )
    .expect("two-lane Stage-3 surface configuration");
    let initial_surface = fixture
        .surface_configuration
        .records
        .iter()
        .map(|record| (record.key.clone(), 0.0))
        .collect();
    let surface_state = crate::DirectSurfaceLiquidOwnedState::new_initial(
        &fixture.surface_configuration,
        &initial_surface,
        0,
    )
    .expect("two-lane Stage-3 surface state");
    let mut frame = fixture.hydrology.beginning_frame().clone();
    frame
        .configure_surface_liquid_shadow(&fixture.surface_configuration, surface_state)
        .expect("install two-lane Stage-3 surface owner");
    fixture.hydrology =
        crate::vegetation_real_hydrology_shadow::RealHydrologyShadowAdapter::try_from_day_start(
            &frame,
            fixture.hydrology.day_index(),
            fixture.hydrology.transaction_id(),
            fixture.hydrology.interval_s(),
            fixture.hydrology.hydrology_owner_id().clone(),
            fixture.hydrology.layer_maps(),
        )
        .expect("two-lane Stage-3 hydrology owner");
    let lower_lse = fixture
        .lse_configuration
        .ofes
        .iter_mut()
        .find(|ofe| ofe.ofe_id == lower_ofe)
        .expect("lower LSE OFE");
    lower_lse.tiles.retain(|tile| tile.tile_id == lower_open);
    lower_lse.tiles[0].fraction_ofe_ground = 1.0;
    fixture.lse_configuration.configuration_sha256 = fixture
        .lse_configuration
        .canonical_sha256()
        .expect("two-lane LSE configuration digest");
    fixture
        .lse_configuration
        .validate()
        .expect("two-lane LSE configuration");
    fixture
        .lse_state
        .tiles
        .retain(|tile| tile.ofe_id != lower_ofe || tile.tile_id == lower_open);
    fixture
        .lse_state
        .configuration_sha256
        .clone_from(&fixture.lse_configuration.configuration_sha256);
    fixture.lse_state.state_sha256 = fixture
        .lse_state
        .canonical_sha256()
        .expect("two-lane LSE state digest");
    fixture
        .lse_state
        .validate(&fixture.lse_configuration)
        .expect("two-lane LSE state");
    fixture
}

#[allow(clippy::too_many_lines)]
fn exercise_complete_owner_two_ofe_child_routes_upstream_runoff_as_downstream_runon() {
    let (shadow, fixture) = v10_shadow_fixture_from(two_ofe_routed_endpoint_fixture());
    let shadow = open_only_complete_owner_shadow(shadow);
    let lower_ofe = OfeId::try_new("ofe-2").expect("lower OFE");
    let mut interval = day_input(&fixture).intervals.remove(0);
    interval.wb14_parameters[0].effective_conductivity_m_s = 1.0e-10;
    interval.wb14_parameters[0].infiltration_storage_capacity_m = 1.0e-8;
    interval.wb14_parameters.push(DirectOfeWb14Parameters {
        ofe_id: OfeId::try_new("ofe-2").expect("lower OFE"),
        effective_conductivity_m_s: 1.0e-10,
        matric_potential_m: 0.1,
        infiltration_storage_capacity_m: 1.0e-8,
    });
    interval
        .lse_forcing
        .precipitation_parcels
        .push(openwepp_land_surface_energy::LiquidParcel {
            parcel_kind: openwepp_land_surface_energy::LiquidParcelKind::Precipitation,
            parcel_id: openwepp_land_surface_energy::ParcelId::try_new("two-ofe-upper-rain")
                .expect("parcel"),
            source_owner_id: ResourceOwnerId::try_new("meteorology").expect("owner"),
            source_ofe_id: OfeId::try_new("ofe-1").expect("upper OFE"),
            source_tile_id: TileId::try_new("atmosphere").expect("source tile"),
            destination_ofe_id: OfeId::try_new("ofe-1").expect("upper OFE"),
            destination_tile_id: TileId::try_new("open").expect("upper tile"),
            start_s: 0.0,
            end_s: 1_800.0,
            amount_kg_m2_destination_tile_ground: 10.0,
            temperature_provider:
                openwepp_land_surface_energy::LiquidTemperatureProvider::HarderPomeroyHourly,
            temperature_k: Some(280.0),
            specific_liquid_enthalpy_j_kg: Some(4_218.0 * (280.0 - 273.15)),
            source_state_sha256: Some(
                openwepp_land_surface_energy::Sha256Digest::try_new("e".repeat(64))
                    .expect("source state"),
            ),
        });
    interval.lse_forcing.forcing_sha256 = interval
        .lse_forcing
        .canonical_sha256()
        .expect("two-OFE forcing");
    let migrated =
        migrate_v10_runtime_to_v11(&shadow.vegetation_configuration, &shadow.vegetation_state)
            .expect("open-only V11 migration");
    let owners = initial_v11_owners(&shadow, &migrated.state);
    let clock_owners = owners
        .values()
        .map(|owner| owner.to_owner_state().expect("clock owner"))
        .collect::<Vec<_>>();
    let (parent_id, slab) = accepted_v11_slab(&clock_owners, 1_800_000_000_000);
    let mut parent = V11ParentTransaction::new_with_complete_owners(
        &migrated.configuration,
        &migrated.state,
        parent_id,
        ModelTimeNs::new(0),
        owners,
    )
    .expect("two-OFE complete parent");
    let stack = DirectV11RealConsumerStack::new(&shadow, &interval, 0, 0);
    let mut executor = crate::v11_vegetation_consumer::DirectV11VegetationExecutor { stack };
    begin_accepted_publication_support_capability_audit_v1();
    let segment =
        execute_direct_v11_segment(&migrated.configuration, &parent, &slab, &mut executor)
            .expect("two-OFE complete-owner child");
    accept_direct_v11_segment(
        &mut parent,
        &migrated.configuration,
        segment,
        &executor.stack.beginning,
    )
    .expect("accept two-OFE child");
    let finalized = parent
        .finalize(&migrated.configuration)
        .expect("finalize parent");
    let hydrology = executor
        .stack
        .last_hydrology_candidate()
        .expect("retained complete child hydrology candidate")
        .clone();
    let ingress = hydrology.surface_ingress();
    let routed = ingress
        .receipts()
        .iter()
        .find(|receipt| {
            receipt.disposition
                == crate::direct_runtime::DirectSurfaceLiquidReceiptDisposition::RoutedRunoff
                && receipt.basis_ofe_id.as_str() == "ofe-1"
                && matches!(
                    &receipt.recipient,
                    crate::direct_runtime::DirectSurfaceLiquidReceiptRecipient::RoutedOfe {
                        destination_ofe_id,
                        ..
                    } if destination_ofe_id.as_str() == "ofe-2"
                )
        })
        .expect("upper-origin routed runoff receipt");
    let lower_ledger = ingress
        .ledgers()
        .iter()
        .find(|ledger| ledger.ofe_id.as_str() == "ofe-2")
        .expect("downstream ingress ledger");
    assert_eq!(
        lower_ledger.ingress_mass_kg_m2_ofe_ground.to_bits(),
        (routed.mass_kg_m2_basis_ofe_ground * 0.5).to_bits(),
        "100/200 square-metre routing mass basis",
    );
    assert_eq!(
        lower_ledger.ingress_enthalpy_j_m2_ofe_ground.to_bits(),
        (routed.enthalpy_j_m2_basis_ofe_ground * 0.5).to_bits(),
        "100/200 square-metre routing enthalpy basis",
    );
    assert!(
        ingress.receipts().iter().any(|receipt| {
            receipt.source_parcel_id == routed.source_parcel_id
                && receipt.basis_ofe_id.as_str() == "ofe-2"
                && receipt.kind
                    == crate::direct_runtime::DirectSurfaceLiquidParcelKind::UpstreamRunon
        }),
        "downstream disposition must retain upper parcel lineage"
    );
    let ending = executor
        .stack
        .commit_selected_publication_and_take_staged_ending()
        .expect("ending owners");
    let capability_audit = take_accepted_publication_support_capability_audit_v1();
    assert_eq!(capability_audit.full_validation_attempt_count, 1);
    assert_eq!(capability_audit.full_validation_success_count, 1);
    assert_eq!(capability_audit.operand_seal_count, 1);
    assert_eq!(capability_audit.receipt_seal_count, 1);
    assert_eq!(capability_audit.capability_mint_count, 1);
    assert_eq!(capability_audit.trusted_append_attempt_count, 1);
    assert_eq!(capability_audit.live_revision_join_count, 1);
    assert_eq!(capability_audit.chronology_owner_tail_join_count, 1);
    assert_eq!(capability_audit.successful_append_count, 1);
    assert_eq!(capability_audit.append_time_full_validation_count, 0);
    assert_eq!(capability_audit.append_time_operand_reconstruction_count, 0);
    assert_eq!(capability_audit.append_time_receipt_reconstruction_count, 0);
    assert_eq!(capability_audit.append_time_serialization_count, 0);
    assert_eq!(capability_audit.append_time_full_prefix_scan_count, 0);
    assert_eq!(capability_audit.support_payload_clone_count, 0);
    let accepted_supports = ending
        .accepted_publication_supports_for_day(0)
        .expect("retained accepted publication support");
    assert_eq!(accepted_supports.len(), 1);
    let accepted = accepted_supports[0];
    assert_eq!(accepted.interval_index(), 0);
    assert_eq!(accepted.parent_transaction_id(), parent_id);
    assert_eq!(accepted.support(), slab.support());
    assert_eq!(accepted.accepted_slab_sha256(), slab.slab_id().digest());
    assert_eq!(
        accepted.run_identity(),
        hydrology.beginning_frame().identity
    );
    assert_eq!(
        accepted.beginning_subsurface_layers(0),
        Some(
            hydrology.beginning_frame().lanes[0]
                .subsurface_layers
                .as_slice()
        )
    );
    assert_eq!(
        accepted.ingress_receipts(),
        hydrology.surface_ingress().receipts()
    );
    assert_eq!(accepted.lse_forcing(), &interval.lse_forcing);
    assert_eq!(accepted.vegetation_forcing(), &interval.vegetation_forcing);
    assert_eq!(accepted.wb14_parameters(), interval.wb14_parameters);
    let accepted_runon_m = accepted
        .ingress_receipts()
        .iter()
        .filter(|receipt| {
            receipt.basis_ofe_id == lower_ofe
                && receipt.kind
                    == crate::direct_runtime::DirectSurfaceLiquidParcelKind::UpstreamRunon
        })
        .map(|receipt| receipt.mass_kg_m2_basis_ofe_ground / 1_000.0)
        .sum::<f64>();
    let accepted_local_m = accepted
        .ingress_receipts()
        .iter()
        .filter(|receipt| {
            receipt.basis_ofe_id == lower_ofe
                && receipt.kind
                    != crate::direct_runtime::DirectSurfaceLiquidParcelKind::UpstreamRunon
        })
        .map(|receipt| receipt.mass_kg_m2_basis_ofe_ground / 1_000.0)
        .sum::<f64>();
    let sent_volume_m3 = accepted
        .ingress_receipts()
        .iter()
        .filter(|receipt| {
            receipt.disposition
                == crate::direct_runtime::DirectSurfaceLiquidReceiptDisposition::RoutedRunoff
                && matches!(
                    &receipt.recipient,
                    crate::direct_runtime::DirectSurfaceLiquidReceiptRecipient::RoutedOfe {
                        destination_ofe_id,
                        ..
                    } if destination_ofe_id == &lower_ofe
                )
        })
        .map(|receipt| receipt.mass_kg_m2_basis_ofe_ground / 1_000.0 * 100.0)
        .sum::<f64>();
    let received_volume_m3 = accepted_runon_m * 200.0;
    assert!(
        accepted_runon_m > 0.0,
        "real accepted child must retain nonzero destination-basis runon",
    );
    assert!(
        (sent_volume_m3 - received_volume_m3).abs() <= 1.0e-12,
        "sealed routed send and destination receipt must close independently",
    );
    assert_ne!(
        accepted_runon_m.to_bits(),
        accepted_local_m.to_bits(),
        "accepted local liquid is a rejected UpStrmQ alias",
    );
    let duplicate = accepted.clone();
    let shared_clone = ending.clone();
    let forced_deep_clone = {
        let _guard = force_deep_clone_accepted_publication_history_v1();
        ending.clone()
    };
    assert_eq!(
        shared_clone, forced_deep_clone,
        "shared and forced-deep publication histories must be value-identical",
    );
    assert_eq!(
        shared_clone
            .canonical_owner_state_bytes()
            .expect("shared-history complete-owner bytes"),
        forced_deep_clone
            .canonical_owner_state_bytes()
            .expect("forced-deep complete-owner bytes"),
        "allocation identity entered complete-owner authority",
    );
    let mut poisoned = ending.clone();
    assert!(
        poisoned
            .restore_accepted_publication_supports(vec![duplicate.clone(), duplicate])
            .is_err(),
        "duplicate support identity must fail before publication",
    );
    let mut cached_tail_poison = ending.clone();
    std::sync::Arc::make_mut(&mut cached_tail_poison.accepted_publication_history.inner)
        .tail_authority
        .support_count += 1;
    assert!(
        cached_tail_poison
            .accepted_publication_history
            .validate_cached_tail_against_full_scan()
            .is_err(),
        "cached publication tail corruption must fail full qualification",
    );
    let mut wb14_poison = ending.accepted_publication_supports();
    let mut poisoned_replay = wb14_poison[0].wb14_child_replay.materialize();
    poisoned_replay[0] ^= 1;
    wb14_poison[0].wb14_child_replay = PersistentCanonicalWb14ReplayV1::from_bytes(poisoned_replay);
    assert!(
        poisoned
            .restore_accepted_publication_supports(wb14_poison)
            .is_err(),
        "WB14 replay substitution must fail before publication",
    );
    #[cfg(any(
        feature = "restart-authority-evidence",
        feature = "persisted-restart-v1"
    ))]
    {
        let canonical = ending
            .restart_authority_accepted_publication_supports_canonical_bytes()
            .expect("accepted publication support canonical bytes");
        let mut restored = ending.clone();
        restored
            .restore_accepted_publication_supports(Vec::new())
            .expect("clear retained supports before restart restore");
        restored
            .restart_authority_restore_accepted_publication_supports_canonical_bytes(&canonical)
            .expect("restore accepted publication support bytes");
        assert_eq!(
            restored.accepted_publication_supports(),
            ending.accepted_publication_supports(),
            "cross-process projection must retain every accepted operand bit-for-bit",
        );
        let mut poisoned_bytes = canonical.clone();
        let last = poisoned_bytes
            .last_mut()
            .expect("nonempty accepted publication bytes");
        *last ^= 1;
        assert!(
            restored
                .restart_authority_restore_accepted_publication_supports_canonical_bytes(
                    &poisoned_bytes,
                )
                .is_err(),
            "wire corruption must fail before support installation",
        );
    }
    let surface = ending
        .inner
        .hydrology_frame
        .surface_liquid_shadow
        .as_deref()
        .expect("ending surface owner");
    let upper = surface
        .continuations
        .iter()
        .find(|row| row.ofe_id.as_str() == "ofe-1")
        .expect("upper continuation");
    let lower = surface
        .continuations
        .iter()
        .find(|row| row.ofe_id.as_str() == "ofe-2")
        .expect("lower continuation");
    assert!(upper.cumulative_supply_m > 0.0);
    assert!(lower.cumulative_supply_m > 0.0, "same-child routed runon");
    assert!(lower.cumulative_infiltration_m > 0.0);
    assert_eq!(finalized.ending_complete_owners.len(), 7);
    assert_eq!(
        ending.inner.accepted_interval_count(),
        shadow.inner.accepted_interval_count() + 1
    );
}

#[test]
fn complete_owner_two_ofe_child_routes_upstream_runoff_as_downstream_runon() {
    exercise_complete_owner_two_ofe_child_routes_upstream_runoff_as_downstream_runon();
}

#[test]
fn three_trial_accepted_publication_capture_stays_within_budget() {
    crate::snow_stage3_v11_attachment::begin_adaptive_controller_test_audit(
        crate::snow_stage3_v11_attachment::AdaptiveControllerTestPolicyV1::default(),
    );
    for _ in 0..3 {
        exercise_complete_owner_two_ofe_child_routes_upstream_runoff_as_downstream_runon();
    }
    let _ = crate::snow_stage3_v11_attachment::take_adaptive_controller_test_audit();
    let captures = crate::snow_stage3_v11_attachment::take_accepted_publication_capture_audit();
    assert_eq!(
        captures.len(),
        3,
        "each real complete-owner trial must perform one compact publication capture",
    );
    let aggregate = captures
        .iter()
        .map(|capture| capture.total_elapsed)
        .sum::<std::time::Duration>();
    eprintln!("three compact publication captures: aggregate={aggregate:?}, phases={captures:?}");
    assert!(
        aggregate < std::time::Duration::from_secs(10),
        "three compact publication captures took {aggregate:?}: {captures:?}",
    );
}

#[test]
fn snow_free_two_ofe_parent_executes_two_routed_900_second_children() {
    let (shadow, fixture) = v10_shadow_fixture_from(two_ofe_routed_endpoint_fixture());
    let mut shadow = open_only_complete_owner_shadow(shadow);
    let full_surface = shadow
        .inner
        .surface_configuration
        .records
        .iter()
        .map(|record| (record.key.clone(), record.capacity_kg_m2_tile))
        .collect::<BTreeMap<_, _>>();
    shadow.inner.hydrology_frame.surface_liquid_shadow = Some(Box::new(
        crate::DirectSurfaceLiquidOwnedState::new_initial(
            &shadow.inner.surface_configuration,
            &full_surface,
            0,
        )
        .expect("full short-parent surface state"),
    ));
    let mut parent = day_input(&fixture).intervals.remove(0);
    parent.wb14_parameters[0].effective_conductivity_m_s = 1.0e-10;
    parent.wb14_parameters[0].infiltration_storage_capacity_m = 1.0e-8;
    parent.wb14_parameters.push(DirectOfeWb14Parameters {
        ofe_id: OfeId::try_new("ofe-2").expect("lower OFE"),
        effective_conductivity_m_s: 1.0e-10,
        matric_potential_m: 0.1,
        infiltration_storage_capacity_m: 1.0e-8,
    });
    parent.lse_forcing.reference_wind_m_s = 1.0e-6;
    parent.vegetation_forcing.wind_m_s = 1.0e-6;
    parent.vegetation_forcing.soil_layers.clear();
    parent
        .lse_forcing
        .precipitation_parcels
        .push(openwepp_land_surface_energy::LiquidParcel {
            parcel_kind: openwepp_land_surface_energy::LiquidParcelKind::Precipitation,
            parcel_id: openwepp_land_surface_energy::ParcelId::try_new("short-parent-upper-rain")
                .expect("parcel"),
            source_owner_id: ResourceOwnerId::try_new("meteorology").expect("owner"),
            source_ofe_id: OfeId::try_new("ofe-1").expect("upper"),
            source_tile_id: TileId::try_new("atmosphere").expect("source tile"),
            destination_ofe_id: OfeId::try_new("ofe-1").expect("upper"),
            destination_tile_id: TileId::try_new("open").expect("upper tile"),
            start_s: 0.0,
            end_s: 1_800.0,
            amount_kg_m2_destination_tile_ground: 20.0,
            temperature_provider:
                openwepp_land_surface_energy::LiquidTemperatureProvider::HarderPomeroyHourly,
            temperature_k: Some(280.0),
            specific_liquid_enthalpy_j_kg: Some(4_218.0 * (280.0 - 273.15)),
            source_state_sha256: Some(Sha256Digest::try_new("e".repeat(64)).expect("source")),
        });
    parent.lse_forcing.forcing_sha256 = parent.lse_forcing.canonical_sha256().expect("forcing");
    let beginning_cursor = shadow
        .inner
        .hydrology_frame
        .surface_liquid_shadow
        .as_deref()
        .expect("beginning surface")
        .continuations
        .clone();
    let migrated =
        migrate_v10_runtime_to_v11(&shadow.vegetation_configuration, &shadow.vegetation_state)
            .expect("short-child V11 migration");
    let owners = initial_v11_owners(&shadow, &migrated.state);
    let clock_owners = owners
        .values()
        .map(|owner| owner.to_owner_state().expect("clock owner"))
        .collect::<Vec<_>>();
    let beginning_owner_digest =
        complete_owner_set_digest(&clock_owners).expect("beginning owner digest");
    let (parent_id, slabs) =
        accepted_v11_slabs(&clock_owners, &[900_000_000_000, 1_800_000_000_000]);
    let mut v11_parent = V11ParentTransaction::new_with_complete_owners(
        &migrated.configuration,
        &migrated.state,
        parent_id,
        ModelTimeNs::new(0),
        owners,
    )
    .expect("short-child complete parent");
    let mut child_receipts = Vec::new();
    for ordinal in 0..2_u128 {
        let child_transaction = shadow.inner.vegetation_state.0.last_transaction_id + 1;
        let mut input =
            segment_interval(&parent, 900_000_000_000, u128::from(child_transaction), 0.0);
        let mut rain = parent
            .lse_forcing
            .precipitation_parcels
            .last()
            .expect("upper parent rain")
            .clone();
        rain.parcel_id = openwepp_land_surface_energy::ParcelId::try_new(format!(
            "short-parent-upper-rain-{ordinal}"
        ))
        .expect("child parcel");
        rain.start_s = 0.0;
        rain.end_s = 900.0;
        rain.amount_kg_m2_destination_tile_ground = 10.0;
        input.lse_forcing.precipitation_parcels.push(rain);
        input.lse_forcing.forcing_sha256 =
            input.lse_forcing.canonical_sha256().expect("child forcing");
        let final_child = ordinal == 1;
        let slab = &slabs[usize::try_from(ordinal).expect("slab ordinal")];
        let binding = crate::direct_runtime::DirectWb14CoupledChildBindingV1 {
            proposed_upper_bound_s_bits: 900.0_f64.to_bits(),
            coupled_parent_transaction_sha256: *parent_id.digest().as_bytes(),
            accepted_slab_sha256: *slab.slab_id().digest().as_bytes(),
            parent_beginning_complete_owner_set_sha256: *beginning_owner_digest.as_bytes(),
            parent_support_start_ns: 0,
            parent_support_end_ns: 1_800_000_000_000,
            child_support_start_ns: ordinal * 900_000_000_000,
            child_support_end_ns: (ordinal + 1) * 900_000_000_000,
        };
        let stack = DirectV11RealConsumerStack::new_parent_child(
            &shadow,
            &input,
            0,
            0,
            final_child,
            binding,
        );
        let mut executor = crate::v11_vegetation_consumer::DirectV11VegetationExecutor { stack };
        let segment =
            execute_direct_v11_segment(&migrated.configuration, &v11_parent, slab, &mut executor)
                .unwrap_or_else(|error| {
                    panic!("snow-free complete-owner short child {ordinal}: {error:?}")
                });
        accept_direct_v11_segment(
            &mut v11_parent,
            &migrated.configuration,
            segment,
            &executor.stack.beginning,
        )
        .expect("accept short child");
        let ingress = executor
            .stack
            .last_hydrology_candidate()
            .expect("short-child hydrology candidate")
            .surface_ingress();
        assert!(ingress.receipts().iter().any(|receipt| {
                receipt.disposition == crate::direct_runtime::DirectSurfaceLiquidReceiptDisposition::RoutedRunoff
            }), "child {ordinal} routes upper excess downstream");
        let lower = ingress
            .ledgers()
            .iter()
            .find(|ledger| ledger.ofe_id.as_str() == "ofe-2")
            .expect("lower ledger");
        assert!(
            lower.ingress_mass_kg_m2_ofe_ground > 0.0,
            "same-child downstream runon"
        );
        child_receipts.push(ingress.wb14_child_replay_bytes().to_vec());
        assert_eq!(
            ingress.wb14_parent_receipt_set_sha256().is_some(),
            final_child
        );
        shadow = executor
            .stack
            .commit_selected_publication_and_take_staged_ending()
            .expect("seven-owner child ending");
        let cursor = &shadow
            .inner
            .hydrology_frame
            .surface_liquid_shadow
            .as_deref()
            .expect("surface owner")
            .continuations;
        if !final_child {
            assert_eq!(cursor, &beginning_cursor);
        }
    }
    let finalized = v11_parent
        .finalize(&migrated.configuration)
        .expect("finalize short parent");
    assert_eq!(finalized.ending_complete_owners.len(), 7);
    let ending = shadow
        .inner
        .hydrology_frame
        .surface_liquid_shadow
        .as_deref()
        .expect("ending surface");
    assert!(
        ending
            .continuations
            .iter()
            .all(|row| row.next_interval_index == 1)
    );
    for replay in child_receipts {
        let rows: serde_json::Value = serde_json::from_slice(&replay).expect("replay");
        assert_eq!(rows[0][0], "ofe-1");
        assert_eq!(rows[1][0], "ofe-2");
    }
}
