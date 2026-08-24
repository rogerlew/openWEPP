
    #[test]
    #[allow(clippy::too_many_lines)]
    fn mixed_open_covered_stack_executes_complete_ofe_ground_boundary() {
        exercise_complete_wb14_cadence(0.005, 8.0, true, None, false);
    }

    #[test]
    #[allow(clippy::too_many_lines)]
    fn two_900_second_complete_owner_children_publish_one_parent() {
        exercise_complete_wb14_cadence(0.02, 8.0, false, None, false);
    }

    #[test]
    #[allow(clippy::too_many_lines)]
    fn one_1800_second_child_matches_complete_historical_candidate() {
        exercise_complete_wb14_cadence(0.08, 8.0, false, None, false);
    }

    #[test]
    #[allow(clippy::too_many_lines)]
    fn coupled_hard_boundary_truncates_selected_900_second_child() {
        exercise_complete_wb14_cadence(0.02, 8.0, false, Some(60_000_000_000), false);
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
        );
    }

    fn exercise_complete_wb14_cadence(
        runtime_swe_m: f64,
        initial_cold_delta_k: f64,
        include_child_17: bool,
        hard_boundary_ns: Option<u128>,
        expect_dynamic_proposal: bool,
    ) {
        let (shadow, fixture) = v10_shadow_fixture();
        let base_interval = day_input(&fixture).intervals.remove(0);
        let interval = segment_interval(&base_interval, 1_800_000_000_000, 41, 0.0);
        let mut interval = interval;
        interval.lse_forcing.snow_present_at_beginning = true;
        interval.lse_forcing.snow_present_at_end = true;
        interval.lse_forcing.forcing_sha256 = interval
            .lse_forcing
            .canonical_sha256()
            .expect("covered forcing digest");
        let covered_interval = DirectV11SnowCoveredSegmentInput::from_snow_free(&interval);

        let migrated =
            migrate_v10_runtime_to_v11(&shadow.vegetation_configuration, &shadow.vegetation_state)
                .expect("migration");
        let owners = initial_v11_owners(&shadow, &migrated.state);
        let clock_owners = owners
            .values()
            .map(|owner| owner.to_owner_state().expect("clock owner"))
            .collect::<Vec<_>>();
        let (parent_id, slab) = accepted_v11_slab(&clock_owners, 1_800_000_000_000);
        let parent = V11ParentTransaction::new_with_complete_owners(
            &migrated.configuration,
            &migrated.state,
            parent_id,
            ModelTimeNs::new(0),
            owners,
        )
        .expect("covered parent");

        let mut stage3_inputs = attachment_stage3_inputs();
        stage3_inputs.surface_energy_options.longwave_model =
            SnowSurfaceLongwaveModel::DilleyUnsworthSubcanopyV1;
        stage3_inputs
            .surface_energy_options
            .daily_solar_radiation_mj_m2 = 0.0;
        stage3_inputs
            .surface_energy_options
            .daily_extraterrestrial_radiation_mj_m2 = 0.0;
        stage3_inputs.surface_energy_options.daylight = false;
        // Exercise the live small-mass Stage-3 proposal through thirty
        // 60-second complete-owner/WB14 children, not only the scalar oracle.
        stage3_inputs.runtime_swe_m = runtime_swe_m;
        stage3_inputs.runtime_depth_m = runtime_swe_m * 10.0;
        stage3_inputs.runtime_density_kg_m3 = 100.0;
        stage3_inputs.snow_layers[0].mass_swe_m = runtime_swe_m;
        stage3_inputs.snow_layers[0].thickness_m = runtime_swe_m * 10.0;
        stage3_inputs.snow_layers[0].density_kg_m3 = 100.0;
        stage3_inputs.snow_layers[0].cold_content_j_m2 =
            runtime_swe_m * 1_000.0 * 2_100.0 * initial_cold_delta_k;
        let stage3_beginning = Wb11HydrologyKernel::initialize_stage3_persistent_state(
            1,
            stage3_inputs.snow_layers.clone(),
        )
        .expect("persistent Stage-3 beginning");
        let stage3_forcing = DirectSnowStage3SupportInput {
            forcing: DirectSnowHourlyForcing::zero(),
            duration_seconds: 1_800.0,
        };
        let stage3_inputs_by_lane = BTreeMap::from([(1, stage3_inputs)]);
        let stage3_forcing_by_lane = BTreeMap::from([(1, stage3_forcing)]);
        let carrier_forcing_by_lane = BTreeMap::from([(1, child2c_carrier_forcing())]);
        let covered_tiles = shadow
            .inner
            .vegetation_configuration
            .strata
            .iter()
            .flat_map(|stratum| stratum.tile_ids.iter().cloned())
            .collect::<std::collections::BTreeSet<_>>();
        let covered_record = shadow
            .inner
            .surface_configuration
            .records
            .iter()
            .find(|record| covered_tiles.contains(&record.key.tile_id))
            .expect("mixed fixture covered tile");
        let covered_only_snow_surface_forcing = BTreeMap::from([(
            (
                covered_record.key.ofe_id.clone(),
                covered_record.key.tile_id.clone(),
            ),
            SealedStage3TileBoundaryForcingV1::V11CanopyCovered(
                carrier_forcing_by_lane[&1].clone(),
            ),
        )]);
        let mut missing_open_executor = crate::v11_vegetation_consumer::DirectV11VegetationExecutor {
            stack: DirectV11SnowCoveredRealConsumerStack::new(
                &shadow,
                DirectV11SnowCoveredStackInputs {
                    interval: &covered_interval,
                    stage3_inputs_by_lane: &stage3_inputs_by_lane,
                    stage3_forcing_by_lane: &stage3_forcing_by_lane,
                    snow_surface_forcing_by_destination: &covered_only_snow_surface_forcing,
                    stage3_beginning_by_lane: BTreeMap::from([(1, stage3_beginning.clone())]),
                    day_index: 0,
                    interval_index: 0,
                    finalize_wb14_parent_interval: true,
                    wb14_coupled_child_binding: test_wb14_coupled_binding(),
                },
            ),
        };
        let missing_error = execute_v11_segment(
            &migrated.configuration,
            &parent,
            &slab,
            &mut missing_open_executor,
        )
        .expect_err("mixed OFE without its open-snow boundary must reject");
        assert!(matches!(
            missing_error,
            V11ExecutionError::Executor(DirectV11RealConsumerError::Identity(
                "covered Stage-3 lane is missing a snow-surface contribution"
            ))
        ));
        assert!(missing_open_executor.stack.take_staged_stage3().is_none());
        assert!(missing_open_executor.stack.take_staged_ending().is_none());
        let open_record = shadow
            .inner
            .surface_configuration
            .records
            .iter()
            .find(|record| !covered_tiles.contains(&record.key.tile_id))
            .expect("mixed fixture open tile");
        let support = TimeSupport::new(
            ModelTimeNs::new(0),
            ModelTimeNs::new(1_800_000_000_000),
        )
        .expect("open-snow support");
        let exposure = SealedOpenSnowExposureReceiptV1::try_new(
            support,
            (open_record.key.ofe_id.clone(), open_record.key.tile_id.clone()),
            Digest32::from_bytes([10; 32]),
            Digest32::from_bytes([11; 32]),
            covered_interval.lse_forcing.reference_wind_m_s,
            Digest32::from_bytes([12; 32]),
        )
        .expect("open-snow exposure");
        let open_forcing = SealedOpenSnowTileForcingV1::try_new(
            SealedOpenSnowTileForcingInputsV1 {
                support,
                destination: (open_record.key.ofe_id.clone(), open_record.key.tile_id.clone()),
                forcing_receipt_sha256: Digest32::from_bytes([10; 32]),
                exposure,
                reference_temperature_k: covered_interval.lse_forcing.air_temperature_k,
                reference_specific_humidity_kg_kg: covered_interval
                    .lse_forcing
                    .air_specific_humidity_kg_kg,
                air_pressure_pa: covered_interval.lse_forcing.air_pressure_pa,
                atmospheric_downward_longwave_w_m2: covered_interval
                    .lse_forcing
                    .atmospheric_downward_longwave_w_m2,
                direct_vis_w_m2: covered_interval.lse_forcing.direct_vis_w_m2,
                diffuse_vis_w_m2: covered_interval.lse_forcing.diffuse_vis_w_m2,
                direct_nir_w_m2: covered_interval.lse_forcing.direct_nir_w_m2,
                diffuse_nir_w_m2: covered_interval.lse_forcing.diffuse_nir_w_m2,
                rain_m: 0.0,
                snowfall_m: 0.0,
                precipitation_parcel_count: 0,
            },
        )
        .expect("open-snow forcing");
        let snow_surface_forcing_by_destination = BTreeMap::from([
            (
                (
                    covered_record.key.ofe_id.clone(),
                    covered_record.key.tile_id.clone(),
                ),
                SealedStage3TileBoundaryForcingV1::V11CanopyCovered(
                    carrier_forcing_by_lane[&1].clone(),
                ),
            ),
            (
                open_forcing.destination.clone(),
                SealedStage3TileBoundaryForcingV1::OpenSnow(open_forcing),
            ),
        ]);
        let stage3_beginning_by_lane = BTreeMap::from([(1, stage3_beginning.clone())]);
        let stack = DirectV11SnowCoveredRealConsumerStack::new(
            &shadow,
            DirectV11SnowCoveredStackInputs {
                interval: &covered_interval,
                stage3_inputs_by_lane: &stage3_inputs_by_lane,
                stage3_forcing_by_lane: &stage3_forcing_by_lane,
                snow_surface_forcing_by_destination: &snow_surface_forcing_by_destination,
                stage3_beginning_by_lane,
                day_index: 0,
                interval_index: 0,
                finalize_wb14_parent_interval: true,
                wb14_coupled_child_binding: test_wb14_coupled_binding(),
            },
        );
        let mut executor = crate::v11_vegetation_consumer::DirectV11VegetationExecutor { stack };
        execute_v11_segment(&migrated.configuration, &parent, &slab, &mut executor)
            .expect("real mixed covered/open OFE execution");
        let lane_receipt = executor
            .stack
            .last_lane_boundary_receipts()
            .and_then(|receipts| receipts.get(&1))
            .expect("mixed OFE final lane receipt")
            .clone();
        assert_eq!(lane_receipt.ordered_destinations.len(), 2);
        assert!(lane_receipt
            .ordered_destinations
            .iter()
            .any(|value| value.boundary_class == crate::snow_stage3_terminal_handoff::Stage3TileBoundaryClassV1::OpenSnow));
        assert!(lane_receipt
            .ordered_destinations
            .iter()
            .any(|value| value.boundary_class == crate::snow_stage3_terminal_handoff::Stage3TileBoundaryClassV1::V11CanopyCovered));
        let historical_complete_candidate = executor
            .stack
            .take_staged_ending()
            .expect("historical one-child complete candidate");

        let identities = shadow
            .inner
            .surface_configuration
            .records
            .iter()
            .map(|record| {
                PreparedStage3V11SupportIdentityV1::new(
                    record.key.ofe_id.as_str().to_owned(),
                    record.key.tile_id.as_str().to_owned(),
                    "a".repeat(64),
                    Digest32::from_bytes([13; 32]),
                    Vec::new(),
                    Digest32::from_bytes([14; 32]),
                )
            })
            .collect::<Vec<_>>();
        let mut snow_free_parent_interval = base_interval.clone();
        snow_free_parent_interval.lse_forcing.snow_present_at_beginning = false;
        snow_free_parent_interval.lse_forcing.snow_present_at_end = false;
        snow_free_parent_interval.lse_forcing.forcing_sha256 = snow_free_parent_interval
            .lse_forcing
            .canonical_sha256()
            .expect("snow-free parent forcing digest");
        let mut prepared = PreparedStage3V11SupportV1::try_new(
            support,
            stage3_inputs_by_lane.clone(),
            stage3_forcing_by_lane.clone(),
            snow_free_parent_interval,
            BTreeMap::from([(1, identities)]),
        )
        .expect("coupled cadence prepared support")
        .with_covered_v11_interval(covered_interval.clone());
        for (destination, forcing) in &snow_surface_forcing_by_destination {
            prepared = match forcing {
                SealedStage3TileBoundaryForcingV1::V11CanopyCovered(value) => prepared
                    .with_covered_tile_forcing(destination.clone(), value.clone()),
                SealedStage3TileBoundaryForcingV1::OpenSnow(value) => prepared
                    .with_sealed_open_tile_forcing(destination.clone(), value.clone()),
            };
        }
        if let Some(boundary_ns) = hard_boundary_ns {
            prepared = prepared
                .with_hard_boundaries(vec![ModelTimeNs::new(boundary_ns)])
                .expect("accepted coupled hard boundary");
        }
        let beginning_owners = initial_v11_owners(&shadow, &migrated.state);
        let beginning_owner_states = beginning_owners
            .values()
            .map(|owner| owner.to_owner_state().expect("beginning clock owner"))
            .collect::<Vec<_>>();
        let beginning_owner_digest =
            complete_owner_set_digest(&beginning_owner_states).expect("beginning owner digest");
        let authority = ParentAuthorityV1::new(
            digest(1),
            digest(2),
            digest(3),
            40,
            support,
            beginning_owner_digest,
        )
        .expect("coupled parent authority");
        let participants = beginning_owner_states
            .iter()
            .map(|owner| owner.owner_id().to_owned())
            .collect::<Vec<_>>();
        let beginning_clock = CoupledClockStateV1::new(
            authority,
            beginning_owner_states,
            "snow-covered".to_owned(),
            participants,
            digest(4),
            Vec::new(),
        )
        .expect("covered beginning clock");
        let context = DirectSnowStage3V11StaticContext {
            run_identity: digest(1),
            topology_identity: digest(9),
            parent_duration_ns: STAGE3_V11_PARENT_SUPPORT_NS,
            minimum_support_ns: 60_000_000_000,
            calendar_receipt: digest(2),
            controller_policy: digest(5),
            parent_sequence: 40,
            lane_ids: vec![1],
            vegetation_configuration: migrated.configuration.clone(),
            surface_liquid_configuration: shadow.inner.surface_configuration.clone(),
            wb14_parameters: covered_interval.wb14_parameters.clone(),
        };
        let selected_seconds =
            Wb11HydrologyKernel::project_stage3_surface_state_v1(&stage3_beginning)
                .expect("coupled cadence projection")
                .selected_substep_seconds;
        let rollback_parent = parent.clone();
        let rollback_consumer = shadow.clone();
        let rollback_clock = beginning_clock.clone();
        let rollback_stage3 = stage3_beginning.clone();
        let mut injections = vec![Stage3V11FailureInjection::AfterSubslab(1)];
        if include_child_17 {
            injections.push(Stage3V11FailureInjection::AfterSubslab(17));
        }
        injections.push(Stage3V11FailureInjection::AfterFinalOwnerJoin);
        for injection in injections {
            assert!(execute_covered_real_v11_parent(
                &context,
                &parent,
                &shadow,
                &beginning_clock,
                &prepared,
                0,
                0,
                digest(3),
                BTreeMap::from([(1, stage3_beginning.clone())]),
                true,
                Some(injection),
            )
            .is_err());
            assert_eq!(parent, rollback_parent);
            assert_eq!(shadow, rollback_consumer);
            assert_eq!(beginning_clock, rollback_clock);
            assert_eq!(stage3_beginning, rollback_stage3);
        }
        let (_, ending_consumer, ending_clock, finalized_parent, _, subslabs) =
            execute_covered_real_v11_parent(
                &context,
                &parent,
                &shadow,
                &beginning_clock,
                &prepared,
                0,
                0,
                digest(3),
                BTreeMap::from([(1, stage3_beginning.clone())]),
                true,
                None,
            )
            .expect("synchronized covered parent cadence");
        assert_eq!(ending_clock.accepted_until(), support.end_ns());
        let expected_children = if expect_dynamic_proposal {
            subslabs.len()
        } else if hard_boundary_ns.is_some() {
            3
        } else {
            (1_800.0 / selected_seconds) as usize
        };
        assert_eq!(subslabs.len(), expected_children);
        if hard_boundary_ns.is_some() {
            assert_eq!(subslabs[0].selected_upper_bound_s_bits, 900.0_f64.to_bits());
            assert_eq!(subslabs[0].support.duration_s_bits(), 60.0_f64.to_bits());
        }
        if expect_dynamic_proposal {
            assert!(subslabs.iter().skip(1).any(|receipt| {
                receipt.selected_upper_bound_s_bits == 60.0_f64.to_bits()
            }), "latest accepted Stage-3 state must change the next proposal");
        }
        assert_eq!(finalized_parent.accepted_segments.len(), subslabs.len());
        assert_eq!(
            ending_consumer.inner.accepted_interval_count(),
            shadow.inner.accepted_interval_count() + 1,
            "thirty coupled slabs publish exactly one persistent parent interval",
        );
        if selected_seconds.to_bits() == 1_800.0_f64.to_bits() {
            assert_eq!(
                ending_consumer, historical_complete_candidate,
                "one-child coordinator must be bit-identical to the complete historical candidate",
            );
        }
        assert!(subslabs.iter().all(|receipt| {
            receipt.validate().is_ok()
                && digest_bytes(&receipt.wb14_child_replay_bytes)
                    == receipt.wb14_child_receipt_set_sha256
                &&
            receipt.wb14_child_receipt_set_sha256 != Digest32::zero()
                && receipt.owner_join.wb14_child_receipt_set_sha256
                    == receipt.wb14_child_receipt_set_sha256
        }));
        assert!(subslabs[..subslabs.len() - 1]
            .iter()
            .all(|receipt| receipt.wb14_parent_receipt_set_sha256.is_none()));
        assert!(subslabs
            .last()
            .and_then(|receipt| receipt.wb14_parent_receipt_set_sha256)
            .is_some());
        let mut poisoned = subslabs[0].clone();
        poisoned.selected_upper_bound_s_bits = if selected_seconds.to_bits() == 900.0_f64.to_bits()
        {
            60.0_f64.to_bits()
        } else {
            900.0_f64.to_bits()
        };
        assert!(poisoned.validate().is_err(), "proposal substitution must reject");
        let mut poisoned = subslabs[0].clone();
        poisoned.wb14_child_replay_bytes[0] ^= 1;
        assert!(poisoned.validate().is_err(), "replay payload substitution must reject");
        let mut poisoned = subslabs[0].clone();
        poisoned.accepted_slab_sha256 = digest(99);
        assert!(poisoned.validate().is_err(), "accepted-slab substitution must reject");
        for pair in subslabs.windows(2) {
            assert_eq!(pair[0].support.end_ns(), pair[1].support.start_ns());
        }
        let reconstructed_sensible = lane_receipt
            .ordered_destinations
            .iter()
            .map(|value| value.tile_fraction * value.sensible_to_canopy_air_w_m2)
            .sum::<f64>();
        assert_eq!(
            lane_receipt.aggregate_sensible_to_canopy_air_w_m2.to_bits(),
            reconstructed_sensible.to_bits(),
            "mixed OFE flux is the unnormalized sum of tile-fraction contributions",
        );
        assert!(executor.stack.take_staged_stage3().is_some());

        let mut open_shadow = shadow.clone();
        for record in &mut open_shadow.inner.surface_configuration.records {
            record.ground_ingress_mode = crate::DirectGroundIngressMode::OpenRawPrecipitation;
        }
        open_shadow.inner.surface_configuration = DirectSurfaceLiquidConfiguration::new(
            open_shadow.inner.surface_configuration.owner_id.clone(),
            open_shadow.inner.surface_configuration.run_id,
            open_shadow.inner.surface_configuration.ofe_topology.clone(),
            open_shadow.inner.surface_configuration.ofe_bindings.clone(),
            open_shadow.inner.surface_configuration.records.clone(),
        )
        .expect("open-only surface configuration");
        let open_liquid = open_shadow
            .inner
            .hydrology_frame
            .surface_liquid_shadow
            .as_deref()
            .expect("open-only beginning surface owner")
            .records
            .iter()
            .map(|record| (record.key.clone(), record.liquid_kg_m2_tile))
            .collect::<BTreeMap<_, _>>();
        open_shadow.inner.hydrology_frame.surface_liquid_shadow = Some(Box::new(
            crate::DirectSurfaceLiquidOwnedState::new_initial(
                &open_shadow.inner.surface_configuration,
                &open_liquid,
                0,
            )
            .expect("open-only beginning surface state"),
        ));
        open_shadow.vegetation_configuration.strata.clear();
        open_shadow.vegetation_configuration.configuration_sha256 = open_shadow
            .vegetation_configuration
            .canonical_sha256()
            .expect("open-only V10 configuration digest");
        open_shadow.vegetation_state.0.occupancies.clear();
        open_shadow.vegetation_state.0.strata.clear();
        open_shadow.vegetation_state.0.tile_canopy_air.clear();
        open_shadow.vegetation_state.0.configuration_sha256 =
            open_shadow.vegetation_configuration.configuration_sha256.clone();
        open_shadow.vegetation_state.0.state_sha256 =
            open_shadow.vegetation_state.0.canonical_sha256();
        open_shadow
            .lse_configuration
            .vegetation_configuration
            .configuration_sha256 = openwepp_land_surface_energy::Sha256Digest::try_new(
            open_shadow
                .vegetation_configuration
                .configuration_sha256
                .clone(),
        )
        .expect("open-only LSE-V2 vegetation configuration receipt");
        open_shadow.lse_configuration.configuration_sha256 = open_shadow
            .lse_configuration
            .canonical_sha256()
            .expect("open-only LSE-V2 configuration digest");
        open_shadow.lse_state.0.configuration_sha256 =
            open_shadow.lse_configuration.configuration_sha256.clone();
        open_shadow.lse_state.0.state_sha256 = open_shadow
            .lse_state
            .0
            .canonical_sha256()
            .expect("open-only LSE-V2 state digest");
        open_shadow.inner.vegetation_configuration.strata.clear();
        open_shadow.inner.vegetation_configuration.configuration_sha256 = open_shadow
            .inner
            .vegetation_configuration
            .canonical_sha256()
            .expect("open-only V9 configuration digest");
        open_shadow.inner.vegetation_state.0.occupancies.clear();
        open_shadow.inner.vegetation_state.0.strata.clear();
        open_shadow.inner.vegetation_state.0.tile_canopy_air.clear();
        open_shadow.inner.vegetation_state.0.configuration_sha256 = open_shadow
            .inner
            .vegetation_configuration
            .configuration_sha256
            .clone();
        open_shadow.inner.vegetation_state.0.state_sha256 =
            open_shadow.inner.vegetation_state.0.canonical_sha256();
        let (open_v8_configuration, _) = project_v9_runtime_to_v8(
            &open_shadow.inner.vegetation_configuration,
            &open_shadow.inner.vegetation_state,
        )
        .expect("open-only V8 projection");
        open_shadow
            .inner
            .lse_configuration
            .vegetation_configuration
            .configuration_sha256 = openwepp_land_surface_energy::Sha256Digest::try_new(
            open_v8_configuration.configuration_sha256,
        )
        .expect("open-only LSE vegetation configuration receipt");
        open_shadow.inner.lse_configuration.configuration_sha256 = open_shadow
            .inner
            .lse_configuration
            .canonical_sha256()
            .expect("open-only LSE configuration digest");
        open_shadow.inner.lse_state.configuration_sha256 =
            open_shadow.inner.lse_configuration.configuration_sha256.clone();
        open_shadow.inner.lse_state.state_sha256 = open_shadow
            .inner
            .lse_state
            .canonical_sha256()
            .expect("open-only LSE state digest");
        let open_migrated = migrate_v10_runtime_to_v11(
            &open_shadow.vegetation_configuration,
            &open_shadow.vegetation_state,
        )
        .expect("open-only migration");
        let open_owners = initial_v11_owners(&open_shadow, &open_migrated.state);
        let open_clock_owners = open_owners
            .values()
            .map(|owner| owner.to_owner_state().expect("open-only clock owner"))
            .collect::<Vec<_>>();
        let (open_parent_id, open_slab) =
            accepted_v11_slab(&open_clock_owners, 1_800_000_000_000);
        let open_parent = V11ParentTransaction::new_with_complete_owners(
            &open_migrated.configuration,
            &open_migrated.state,
            open_parent_id,
            ModelTimeNs::new(0),
            open_owners,
        )
        .expect("open-only parent");
        let open_only_forcing = open_shadow
            .inner
            .surface_configuration
            .records
            .iter()
            .enumerate()
            .map(|(index, record)| {
                let destination = (record.key.ofe_id.clone(), record.key.tile_id.clone());
                let forcing_receipt = Digest32::from_bytes([
                    20 + u8::try_from(index).expect("open-only tile ordinal");
                    32
                ]);
                let exposure = SealedOpenSnowExposureReceiptV1::try_new(
                    support,
                    destination.clone(),
                    forcing_receipt,
                    Digest32::from_bytes([30; 32]),
                    covered_interval.lse_forcing.reference_wind_m_s,
                    Digest32::from_bytes([31; 32]),
                )
                .expect("open-only exposure");
                let forcing = SealedOpenSnowTileForcingV1::try_new(
                    SealedOpenSnowTileForcingInputsV1 {
                        support,
                        destination: destination.clone(),
                        forcing_receipt_sha256: forcing_receipt,
                        exposure,
                        reference_temperature_k: covered_interval
                            .lse_forcing
                            .air_temperature_k,
                        reference_specific_humidity_kg_kg: covered_interval
                            .lse_forcing
                            .air_specific_humidity_kg_kg,
                        air_pressure_pa: covered_interval.lse_forcing.air_pressure_pa,
                        atmospheric_downward_longwave_w_m2: covered_interval
                            .lse_forcing
                            .atmospheric_downward_longwave_w_m2,
                        direct_vis_w_m2: covered_interval.lse_forcing.direct_vis_w_m2,
                        diffuse_vis_w_m2: covered_interval.lse_forcing.diffuse_vis_w_m2,
                        direct_nir_w_m2: covered_interval.lse_forcing.direct_nir_w_m2,
                        diffuse_nir_w_m2: covered_interval.lse_forcing.diffuse_nir_w_m2,
                        rain_m: 0.0,
                        snowfall_m: 0.0,
                        precipitation_parcel_count: 0,
                    },
                )
                .expect("open-only forcing");
                (
                    destination,
                    SealedStage3TileBoundaryForcingV1::OpenSnow(forcing),
                )
            })
            .collect::<BTreeMap<_, _>>();
        let mut open_only_executor =
            crate::v11_vegetation_consumer::DirectV11VegetationExecutor {
                stack: DirectV11SnowCoveredRealConsumerStack::new(
                    &open_shadow,
                    DirectV11SnowCoveredStackInputs {
                        interval: &covered_interval,
                        stage3_inputs_by_lane: &stage3_inputs_by_lane,
                        stage3_forcing_by_lane: &stage3_forcing_by_lane,
                        snow_surface_forcing_by_destination: &open_only_forcing,
                        stage3_beginning_by_lane: BTreeMap::from([(1, stage3_beginning.clone())]),
                        day_index: 0,
                        interval_index: 0,
                        finalize_wb14_parent_interval: true,
                        wb14_coupled_child_binding: test_wb14_coupled_binding(),
                    },
                ),
            };
        execute_v11_segment(
            &open_migrated.configuration,
            &open_parent,
            &open_slab,
            &mut open_only_executor,
        )
        .expect("real open-only Stage-3 parent execution");
        let open_only_receipt = open_only_executor
            .stack
            .last_lane_boundary_receipts()
            .and_then(|receipts| receipts.get(&1))
            .expect("open-only lane receipt");
        assert!(open_only_receipt.ordered_destinations.iter().all(|receipt| {
            receipt.boundary_class
                == crate::snow_stage3_terminal_handoff::Stage3TileBoundaryClassV1::OpenSnow
        }));
        assert!(open_only_executor.stack.take_staged_stage3().is_some());
        let open_only_ending = open_only_executor
            .stack
            .take_staged_ending()
            .expect("open-only staged owners");
        assert_eq!(
            open_only_ending.inner.lse_state.tiles,
            open_shadow.inner.lse_state.tiles,
            "open-only execution changes receipt chronology but not LSE tile physics",
        );
        assert_eq!(
            open_only_ending.inner.soil_thermal.ofes,
            open_shadow.inner.soil_thermal.ofes,
            "open-only execution changes receipt chronology but not soil thermal physics",
        );

        let (_, original_vegetation_state) = project_v9_runtime_to_v8(
            &executor.stack.beginning.inner.vegetation_configuration,
            &executor.stack.beginning.inner.vegetation_state,
        )
        .expect("original V8 vegetation state");
        let original_carrier_receipt = executor
            .stack
            .derive_live_carrier_input(
                        1,
                        &stage3_beginning,
                        &original_vegetation_state,
                        stage3_forcing,
                        &carrier_forcing_by_lane[&1],
                        None,
                        1_800.0,
            )
            .expect("original Stage-3 carrier guess")
            .diagnostic_sha256;
        let mut changed_layers = attachment_stage3_inputs().snow_layers;
        changed_layers[0].temperature_c -= 1.0;
        changed_layers[0].cold_content_j_m2 +=
            changed_layers[0].mass_swe_m * 1_000.0 * 2_100.0;
        let changed_stage3 = Wb11HydrologyKernel::initialize_stage3_persistent_state(
            1,
            changed_layers,
        )
        .expect("changed Stage-3 beginning");
        let changed_stack = DirectV11SnowCoveredRealConsumerStack::new(
            &shadow,
            DirectV11SnowCoveredStackInputs {
                interval: &covered_interval,
                stage3_inputs_by_lane: &stage3_inputs_by_lane,
                stage3_forcing_by_lane: &stage3_forcing_by_lane,
                snow_surface_forcing_by_destination: &snow_surface_forcing_by_destination,
                stage3_beginning_by_lane: BTreeMap::from([(1, changed_stage3.clone())]),
                day_index: 0,
                interval_index: 0,
                finalize_wb14_parent_interval: true,
                wb14_coupled_child_binding: test_wb14_coupled_binding(),
            },
        );
        let (_, changed_vegetation_state) = project_v9_runtime_to_v8(
            &changed_stack.beginning.inner.vegetation_configuration,
            &changed_stack.beginning.inner.vegetation_state,
        )
        .expect("changed V8 vegetation state");
        let changed_carrier_receipt = changed_stack
            .derive_live_carrier_input(
                    1,
                    &changed_stage3,
                    &changed_vegetation_state,
                    stage3_forcing,
                    &carrier_forcing_by_lane[&1],
                    None,
                    1_800.0,
            )
            .expect("changed Stage-3 carrier guess")
            .diagnostic_sha256;
        assert_ne!(
            changed_carrier_receipt,
            original_carrier_receipt,
            "carrier identity must depend on committed Stage-3 state"
        );
        let mut changed_canopy_state = changed_vegetation_state.clone();
        changed_canopy_state
            .tile_canopy_air
            .values_mut()
            .next()
            .expect("canopy-air tile")
            .canopy_air_temperature_k += 1.0;
        let changed_canopy_receipt = changed_stack
            .derive_live_carrier_input(
                    1,
                    &changed_stage3,
                    &changed_canopy_state,
                    stage3_forcing,
                    &carrier_forcing_by_lane[&1],
                    None,
                    1_800.0,
            )
            .expect("changed canopy carrier guess")
            .diagnostic_sha256;
        assert_ne!(
            changed_canopy_receipt, changed_carrier_receipt,
            "carrier identity must depend on candidate canopy-air state"
        );

        let mut poisoned_surface_forcing = snow_surface_forcing_by_destination.clone();
        let SealedStage3TileBoundaryForcingV1::V11CanopyCovered(poisoned_forcing) =
            poisoned_surface_forcing
                .get_mut(&(
                    covered_record.key.ofe_id.clone(),
                    covered_record.key.tile_id.clone(),
                ))
                .expect("covered destination forcing")
        else {
            panic!("covered destination class");
        };
        poisoned_forcing.exposure.wind_m_s = 0.0;
        let mut poisoned = crate::v11_vegetation_consumer::DirectV11VegetationExecutor {
            stack: DirectV11SnowCoveredRealConsumerStack::new(
                &shadow,
                DirectV11SnowCoveredStackInputs {
                    interval: &covered_interval,
                    stage3_inputs_by_lane: &stage3_inputs_by_lane,
                    stage3_forcing_by_lane: &stage3_forcing_by_lane,
                    snow_surface_forcing_by_destination: &poisoned_surface_forcing,
                    stage3_beginning_by_lane: BTreeMap::from([(
                        1,
                        Wb11HydrologyKernel::initialize_stage3_persistent_state(
                            1,
                            attachment_stage3_inputs().snow_layers,
                        )
                        .expect("rollback Stage-3 beginning"),
                    )]),
                    day_index: 0,
                    interval_index: 0,
                    finalize_wb14_parent_interval: true,
                    wb14_coupled_child_binding: test_wb14_coupled_binding(),
                },
            ),
        };
        assert!(execute_v11_segment(&migrated.configuration, &parent, &slab, &mut poisoned).is_err());
        assert!(poisoned.stack.take_staged_ending().is_none());
    }


    #[test]
    #[allow(clippy::too_many_lines)]
    fn complete_owner_two_ofe_child_routes_upstream_runoff_as_downstream_runon() {
        let (shadow, fixture) =
            v10_shadow_fixture_from(two_ofe_routed_endpoint_fixture());
        let shadow = open_only_complete_owner_shadow(shadow);
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
        let segment = execute_v11_segment(&migrated.configuration, &parent, &slab, &mut executor)
            .expect("two-OFE complete-owner child");
        parent
            .accept_segment(&migrated.configuration, segment)
            .expect("accept two-OFE child");
        let finalized = parent.finalize(&migrated.configuration).expect("finalize parent");
        let hydrology = executor
            .stack
            .last_hydrology_candidate()
            .expect("retained complete child hydrology candidate");
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
        assert!(ingress.receipts().iter().any(|receipt| {
            receipt.source_parcel_id == routed.source_parcel_id
                && receipt.basis_ofe_id.as_str() == "ofe-2"
                && receipt.kind
                    == crate::direct_runtime::DirectSurfaceLiquidParcelKind::UpstreamRunon
        }), "downstream disposition must retain upper parcel lineage");
        let ending = executor.stack.take_staged_ending().expect("ending owners");
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
        assert_eq!(ending.inner.accepted_interval_count(), shadow.inner.accepted_interval_count() + 1);
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
        parent.lse_forcing.precipitation_parcels.push(openwepp_land_surface_energy::LiquidParcel {
            parcel_kind: openwepp_land_surface_energy::LiquidParcelKind::Precipitation,
            parcel_id: openwepp_land_surface_energy::ParcelId::try_new("short-parent-upper-rain").expect("parcel"),
            source_owner_id: ResourceOwnerId::try_new("meteorology").expect("owner"),
            source_ofe_id: OfeId::try_new("ofe-1").expect("upper"),
            source_tile_id: TileId::try_new("atmosphere").expect("source tile"),
            destination_ofe_id: OfeId::try_new("ofe-1").expect("upper"),
            destination_tile_id: TileId::try_new("open").expect("upper tile"),
            start_s: 0.0, end_s: 1_800.0,
            amount_kg_m2_destination_tile_ground: 20.0,
            temperature_provider: openwepp_land_surface_energy::LiquidTemperatureProvider::HarderPomeroyHourly,
            temperature_k: Some(280.0),
            specific_liquid_enthalpy_j_kg: Some(4_218.0 * (280.0 - 273.15)),
            source_state_sha256: Some(Sha256Digest::try_new("e".repeat(64)).expect("source")),
        });
        parent.lse_forcing.forcing_sha256 = parent.lse_forcing.canonical_sha256().expect("forcing");
        let beginning_cursor = shadow.inner.hydrology_frame.surface_liquid_shadow.as_deref()
            .expect("beginning surface").continuations.clone();
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
            let mut input = segment_interval(
                &parent,
                900_000_000_000,
                u128::from(child_transaction),
                0.0,
            );
            let mut rain = parent.lse_forcing.precipitation_parcels.last()
                .expect("upper parent rain").clone();
            rain.parcel_id = openwepp_land_surface_energy::ParcelId::try_new(format!(
                "short-parent-upper-rain-{ordinal}"
            ))
            .expect("child parcel");
            rain.start_s = 0.0;
            rain.end_s = 900.0;
            rain.amount_kg_m2_destination_tile_ground = 10.0;
            input.lse_forcing.precipitation_parcels.push(rain);
            input.lse_forcing.forcing_sha256 = input.lse_forcing.canonical_sha256().expect("child forcing");
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
            let segment = execute_v11_segment(
                &migrated.configuration,
                &v11_parent,
                slab,
                &mut executor,
            )
            .unwrap_or_else(|error| panic!("snow-free complete-owner short child {ordinal}: {error:?}"));
            v11_parent
                .accept_segment(&migrated.configuration, segment)
                .expect("accept short child");
            let ingress = executor
                .stack
                .last_hydrology_candidate()
                .expect("short-child hydrology candidate")
                .surface_ingress();
            assert!(ingress.receipts().iter().any(|receipt| {
                receipt.disposition == crate::direct_runtime::DirectSurfaceLiquidReceiptDisposition::RoutedRunoff
            }), "child {ordinal} routes upper excess downstream");
            let lower = ingress.ledgers().iter().find(|ledger| ledger.ofe_id.as_str() == "ofe-2")
                .expect("lower ledger");
            assert!(lower.ingress_mass_kg_m2_ofe_ground > 0.0, "same-child downstream runon");
            child_receipts.push(ingress.wb14_child_replay_bytes().to_vec());
            assert_eq!(ingress.wb14_parent_receipt_set_sha256().is_some(), final_child);
            shadow = executor.stack.take_staged_ending().expect("seven-owner child ending");
            let cursor = &shadow.inner.hydrology_frame.surface_liquid_shadow.as_deref()
                .expect("surface owner").continuations;
            if !final_child { assert_eq!(cursor, &beginning_cursor); }
        }
        let finalized = v11_parent.finalize(&migrated.configuration).expect("finalize short parent");
        assert_eq!(finalized.ending_complete_owners.len(), 7);
        let ending = shadow.inner.hydrology_frame.surface_liquid_shadow.as_deref().expect("ending surface");
        assert!(ending.continuations.iter().all(|row| row.next_interval_index == 1));
        for replay in child_receipts {
            let rows: serde_json::Value = serde_json::from_slice(&replay).expect("replay");
            assert_eq!(rows[0][0], "ofe-1");
            assert_eq!(rows[1][0], "ofe-2");
        }
    }
    fn open_only_complete_owner_shadow(
        mut shadow: DirectV10RealConsumerShadow,
    ) -> DirectV10RealConsumerShadow {
        for record in &mut shadow.inner.surface_configuration.records {
            record.ground_ingress_mode = crate::DirectGroundIngressMode::OpenRawPrecipitation;
        }
        shadow.inner.surface_configuration = DirectSurfaceLiquidConfiguration::new(
            shadow.inner.surface_configuration.owner_id.clone(),
            shadow.inner.surface_configuration.run_id,
            shadow.inner.surface_configuration.ofe_topology.clone(),
            shadow.inner.surface_configuration.ofe_bindings.clone(),
            shadow.inner.surface_configuration.records.clone(),
        )
        .expect("open-only surface configuration");
        let initial = shadow
            .inner
            .surface_configuration
            .records
            .iter()
            .map(|record| (record.key.clone(), 0.0))
            .collect::<BTreeMap<_, _>>();
        shadow.inner.hydrology_frame.surface_liquid_shadow = Some(Box::new(
            crate::DirectSurfaceLiquidOwnedState::new_initial(
                &shadow.inner.surface_configuration,
                &initial,
                0,
            )
            .expect("open-only surface state"),
        ));
        shadow.vegetation_configuration.strata.clear();
        shadow.vegetation_configuration.configuration_sha256 = shadow
            .vegetation_configuration
            .canonical_sha256()
            .expect("open-only V10 configuration");
        shadow.vegetation_state.0.occupancies.clear();
        shadow.vegetation_state.0.strata.clear();
        shadow.vegetation_state.0.tile_canopy_air.clear();
        shadow.vegetation_state.0.configuration_sha256 =
            shadow.vegetation_configuration.configuration_sha256.clone();
        shadow.vegetation_state.0.state_sha256 = shadow.vegetation_state.0.canonical_sha256();
        shadow.lse_configuration.vegetation_configuration.configuration_sha256 =
            openwepp_land_surface_energy::Sha256Digest::try_new(
                shadow.vegetation_configuration.configuration_sha256.clone(),
            )
            .expect("open-only V10 LSE vegetation receipt");
        shadow.lse_configuration.configuration_sha256 = shadow
            .lse_configuration
            .canonical_sha256()
            .expect("open-only V2 LSE configuration");
        shadow.lse_state.0.configuration_sha256 =
            shadow.lse_configuration.configuration_sha256.clone();
        shadow.lse_state.0.state_sha256 = shadow
            .lse_state
            .0
            .canonical_sha256()
            .expect("open-only V2 LSE state");
        shadow.inner.vegetation_configuration.strata.clear();
        shadow.inner.vegetation_configuration.configuration_sha256 = shadow
            .inner
            .vegetation_configuration
            .canonical_sha256()
            .expect("open-only V9 configuration");
        shadow.inner.vegetation_state.0.occupancies.clear();
        shadow.inner.vegetation_state.0.strata.clear();
        shadow.inner.vegetation_state.0.tile_canopy_air.clear();
        shadow.inner.vegetation_state.0.configuration_sha256 = shadow
            .inner
            .vegetation_configuration
            .configuration_sha256
            .clone();
        shadow.inner.vegetation_state.0.state_sha256 =
            shadow.inner.vegetation_state.0.canonical_sha256();
        let (v8_configuration, _) = project_v9_runtime_to_v8(
            &shadow.inner.vegetation_configuration,
            &shadow.inner.vegetation_state,
        )
        .expect("open-only V8 projection");
        shadow
            .inner
            .lse_configuration
            .vegetation_configuration
            .configuration_sha256 = openwepp_land_surface_energy::Sha256Digest::try_new(
            v8_configuration.configuration_sha256,
        )
        .expect("open-only V8 LSE vegetation receipt");
        shadow.inner.lse_configuration.configuration_sha256 = shadow
            .inner
            .lse_configuration
            .canonical_sha256()
            .expect("open-only V1 LSE configuration");
        shadow.inner.lse_state.configuration_sha256 =
            shadow.inner.lse_configuration.configuration_sha256.clone();
        shadow.inner.lse_state.state_sha256 = shadow
            .inner
            .lse_state
            .canonical_sha256()
            .expect("open-only V1 LSE state");
        shadow
    }
