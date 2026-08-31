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

mod adaptive_production_path_coverage {
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

#[allow(
    clippy::cast_possible_truncation,
    clippy::cast_sign_loss,
    clippy::too_many_lines
)]
fn exercise_complete_wb14_cadence(
    runtime_swe_m: f64,
    initial_cold_delta_k: f64,
    include_child_17: bool,
    hard_boundary_ns: Option<u128>,
    expect_dynamic_proposal: bool,
    second_lane_swe_m: Option<f64>,
    terminal_event: bool,
    capture_terminal_failure: bool,
    solid_reappearance: bool,
    production_only: bool,
) {
    let equilibrium_fixture = adaptive_production_path_coverage::equilibrium_fixture_requested();
    let positive_covered_rain = runtime_swe_m.to_bits() == 0.005_f64.to_bits()
        && include_child_17
        && second_lane_swe_m.is_none();
    let (shadow, fixture) = if equilibrium_fixture {
        let mut fixture = endpoint_fixture();
        let temperature_k: f64 = 273.15;
        let humidity = equilibrium_canopy_specific_humidity(101_325.0);
        for shared in fixture.vegetation_state.strata.values_mut() {
            // SC-VEGETATION-001@29 INV-118: this is the typed dormant,
            // zero-area class. Clear every plant material source that could
            // redeploy leaf area during the trial; standing-dead and external
            // soil/BGC owners remain present and sealed.
            for pool in shared.tissues.values_mut() {
                *pool = openwepp_vegetation::carbon_nitrogen::TissuePool::default();
            }
            shared.retranslocation_n = 0.0;
            shared.nsc_c = 0.0;
            shared.xs_c = 0.0;
            shared.phase = openwepp_vegetation::transaction::PhenologyPhase::Dormant;
            shared.onset_remaining_s = 0.0;
            shared.offset_remaining_s = 0.0;
            shared.previous_gsi = 1.0;
            shared.pending_transfers.clear();
            shared.leaf_area = 0.0;
            shared.stem_area = 0.0;
            shared.root_area = 0.0;
        }
        for occupancy in fixture.vegetation_state.occupancies.values_mut() {
            occupancy.sun_leaf_temperature_k = temperature_k;
            occupancy.shade_leaf_temperature_k = temperature_k;
            occupancy.dry_stem_temperature_k = temperature_k;
            occupancy.wet_surface_temperature_k = temperature_k;
            occupancy.canopy_liquid_kg_h2o_m2_tile_ground = 0.0;
        }
        for (occupancy_id, occupancy) in &mut fixture.vegetation_state.occupancies {
            fixture
                .vegetation_configuration
                .strata
                .iter()
                .find(|stratum| stratum.stratum_id == occupancy_id.stratum_id)
                .expect("equilibrium occupancy stratum");
            occupancy.root_node_potential_mm = 0.0;
            occupancy.stem_potential_mm = 0.0;
            occupancy.sun_leaf_potential_mm = 0.0;
            occupancy.shade_leaf_potential_mm = 0.0;
            occupancy.beta_hyd = 1.0;
        }
        for canopy_air in fixture.vegetation_state.tile_canopy_air.values_mut() {
            canopy_air.canopy_air_temperature_k = temperature_k;
            canopy_air.canopy_air_specific_humidity_kg_kg = humidity;
        }
        fixture.vegetation_state.state_sha256 = fixture.vegetation_state.canonical_sha256();
        for tile in &mut fixture.lse_state.tiles {
            tile.surface_enthalpy_j_m2_tile_ground = 0.0;
            tile.surface_temperature_warm_start_k = temperature_k;
        }
        fixture.lse_state.state_sha256 = fixture
            .lse_state
            .canonical_sha256()
            .expect("equilibrium LSE state digest");
        for ofe in &mut fixture.thermal.ofes {
            for layer in &mut ofe.ordered_layers {
                layer.temperature_k = temperature_k;
                layer.enthalpy_j_m2_ofe_ground = 0.0;
            }
        }
        // The strict endpoint fixture predates explicit soil-thermal lineage.
        // Bind this equilibrium-only reseal to the exact beginning owner
        // transaction already carried jointly by vegetation and BGC; the
        // hydrology/forcing transaction is the successor (41), not the
        // beginning identity used by this persistent snapshot.
        assert_eq!(
            fixture.vegetation_state.last_transaction_id,
            fixture.biogeochemistry.last_transaction_id,
            "equilibrium beginning-owner lineage"
        );
        let thermal_transaction = TransactionId(fixture.vegetation_state.last_transaction_id);
        fixture.thermal.last_accepted_transaction_id = Some(thermal_transaction);
        fixture.thermal.state_sha256 = digest_soil_state(
            &fixture.thermal.owner_id,
            thermal_transaction,
            &fixture.thermal.ofes,
        )
        .expect("equilibrium soil-thermal state digest");
        fixture.thermal.snapshot_sha256 = digest_soil_snapshot(
            &fixture.thermal.owner_id,
            &fixture.thermal.configuration_sha256,
            &fixture.thermal.state_sha256,
            thermal_transaction,
            &fixture.thermal.ofes,
        )
        .expect("equilibrium soil-thermal snapshot digest");
        fixture
            .thermal
            .validate()
            .expect("equilibrium soil-thermal snapshot shape");
        assert_eq!(
            fixture.thermal.state_sha256,
            digest_soil_state(
                &fixture.thermal.owner_id,
                thermal_transaction,
                &fixture.thermal.ofes,
            )
            .expect("validate equilibrium soil-thermal state digest"),
            "equilibrium nested soil state digest"
        );
        assert_eq!(
            fixture.thermal.snapshot_sha256,
            digest_soil_snapshot(
                &fixture.thermal.owner_id,
                &fixture.thermal.configuration_sha256,
                &fixture.thermal.state_sha256,
                thermal_transaction,
                &fixture.thermal.ofes,
            )
            .expect("validate equilibrium soil-thermal snapshot digest"),
            "equilibrium outer soil snapshot digest"
        );
        let hydrology_adapter =
            crate::land_surface_energy_shadow::LandSurfaceEnergyRealHydrologyAdapter::new(
                &fixture.hydrology,
            );
        let snow_forcing = fixture.receipt.forcing().clone();
        fixture.receipt = V8CanopyForcingReceipt::try_new(
            fixture.vegetation_configuration.configuration_sha256.clone(),
            fixture.vegetation_state.state_sha256.clone(),
            fixture.lse_configuration.configuration_sha256.clone(),
            fixture.forcing.forcing_sha256.clone(),
            unified_beginning_hydrology_snapshot_sha256(
                &hydrology_adapter,
                &fixture.surface_configuration,
            )
            .expect("equilibrium hydrology snapshot"),
            fixture.thermal.snapshot_sha256.clone(),
            TransactionId(41),
            snow_forcing,
        )
        .expect("equilibrium canopy forcing receipt");
        v10_shadow_fixture_from(fixture)
    } else if second_lane_swe_m.is_some() {
        v10_shadow_fixture_from(two_lane_stage3_endpoint_fixture())
    } else if positive_covered_rain {
        let mut fixture = endpoint_fixture();
        for stratum in &mut fixture.vegetation_configuration.strata {
            stratum.p_liq_kg_m2_plant = 0.20;
            stratum.wet_surface_dimension_m = 100.0;
            stratum.stemflow_fraction = 0.25;
        }
        fixture.vegetation_configuration.configuration_sha256 = fixture
            .vegetation_configuration
            .canonical_sha256()
            .expect("wet-canopy fixture configuration digest");
        fixture
            .vegetation_state
            .configuration_sha256
            .clone_from(&fixture.vegetation_configuration.configuration_sha256);
        fixture.vegetation_state.state_sha256 = fixture.vegetation_state.canonical_sha256();
        v10_shadow_fixture_from(fixture)
    } else {
        v10_shadow_fixture()
    };
    let short_support = adaptive_production_path_coverage::short_support_requested();
    let support_end_ns = if short_support {
        adaptive_production_path_coverage::short_support_duration_ns()
    } else {
        1_800_000_000_000
    };
    let support_seconds = support_end_ns as f64 / 1_000_000_000.0;
    let base_interval = day_input(&fixture).intervals.remove(0);
    let interval = segment_interval(&base_interval, support_end_ns, 41, 0.0);
    let mut interval = interval;
    if equilibrium_fixture {
        let temperature_k: f64 = 273.15;
        let humidity = equilibrium_canopy_specific_humidity(interval.lse_forcing.air_pressure_pa);
        let emitted_longwave_w_m2 = 5.670_374_419e-8 * temperature_k.powi(4);
        interval.lse_forcing.air_temperature_k = temperature_k;
        interval.lse_forcing.air_specific_humidity_kg_kg = humidity;
        interval.lse_forcing.direct_vis_w_m2 = 0.0;
        interval.lse_forcing.diffuse_vis_w_m2 = 0.0;
        interval.lse_forcing.direct_nir_w_m2 = 0.0;
        interval.lse_forcing.diffuse_nir_w_m2 = 0.0;
        interval.lse_forcing.atmospheric_downward_longwave_w_m2 = emitted_longwave_w_m2;
        interval.vegetation_forcing.air_temperature_k = temperature_k;
        interval.vegetation_forcing.specific_humidity = humidity;
        interval.vegetation_forcing.direct_par_w_m2 = 0.0;
        interval.vegetation_forcing.diffuse_par_w_m2 = 0.0;
        interval.vegetation_forcing.direct_nir_w_m2 = 0.0;
        interval.vegetation_forcing.diffuse_nir_w_m2 = 0.0;
        interval.vegetation_forcing.solar_zenith_cosine = 0.0;
        interval.vegetation_forcing.longwave_down_w_m2 = emitted_longwave_w_m2;
        interval.vegetation_forcing.longwave_up_w_m2 = emitted_longwave_w_m2;
    }
    if second_lane_swe_m.is_some() {
        interval.wb14_parameters = shadow
            .inner
            .lse_configuration
            .ofes
            .iter()
            .map(|ofe| DirectOfeWb14Parameters {
                ofe_id: ofe.ofe_id.clone(),
                effective_conductivity_m_s: 1e-6,
                matric_potential_m: 0.1,
                infiltration_storage_capacity_m: 0.04,
            })
            .collect();
    }
    interval.lse_forcing.snow_present_at_beginning = !solid_reappearance;
    interval.lse_forcing.snow_present_at_end = true;
    if positive_covered_rain {
        interval.vegetation_forcing.rain_kg_m2 = 0.3;
        let covered_tiles = shadow
            .inner
            .vegetation_configuration
            .strata
            .iter()
            .flat_map(|stratum| stratum.tile_ids.iter())
            .collect::<std::collections::BTreeSet<_>>();
        let covered = shadow
            .inner
            .surface_configuration
            .records
            .iter()
            .find(|record| covered_tiles.contains(&record.key.tile_id))
            .expect("positive-rain covered destination");
        interval.lse_forcing.precipitation_parcels.push(
            openwepp_land_surface_energy::LiquidParcel {
                parcel_kind: openwepp_land_surface_energy::LiquidParcelKind::Precipitation,
                parcel_id: openwepp_land_surface_energy::ParcelId::try_new("covered-stage3-rain")
                    .expect("parcel"),
                source_owner_id: ResourceOwnerId::try_new("meteorology").expect("owner"),
                source_ofe_id: covered.key.ofe_id.clone(),
                source_tile_id: TileId::try_new("atmosphere").expect("source tile"),
                destination_ofe_id: covered.key.ofe_id.clone(),
                destination_tile_id: covered.key.tile_id.clone(),
                start_s: 0.0,
                end_s: support_seconds,
                amount_kg_m2_destination_tile_ground: 0.3,
                temperature_provider:
                    openwepp_land_surface_energy::LiquidTemperatureProvider::HarderPomeroyHourly,
                temperature_k: Some(272.15),
                specific_liquid_enthalpy_j_kg: Some(4_218.0 * (272.15 - 273.15)),
                source_state_sha256: Some(
                    Sha256Digest::try_new("d".repeat(64)).expect("source state"),
                ),
            },
        );
        let open = shadow
            .inner
            .surface_configuration
            .records
            .iter()
            .find(|record| !covered_tiles.contains(&record.key.tile_id))
            .expect("rain-on-snow open destination");
        interval.lse_forcing.precipitation_parcels.push(
            openwepp_land_surface_energy::LiquidParcel {
                parcel_kind: openwepp_land_surface_energy::LiquidParcelKind::Precipitation,
                parcel_id: openwepp_land_surface_energy::ParcelId::try_new("open-rain-on-snow")
                    .expect("parcel"),
                source_owner_id: ResourceOwnerId::try_new("meteorology").expect("owner"),
                source_ofe_id: open.key.ofe_id.clone(),
                source_tile_id: TileId::try_new("atmosphere").expect("source tile"),
                destination_ofe_id: open.key.ofe_id.clone(),
                destination_tile_id: open.key.tile_id.clone(),
                start_s: 0.0,
                end_s: support_seconds,
                amount_kg_m2_destination_tile_ground: 0.05,
                temperature_provider:
                    openwepp_land_surface_energy::LiquidTemperatureProvider::HarderPomeroyHourly,
                temperature_k: Some(272.15),
                specific_liquid_enthalpy_j_kg: Some(4_218.0 * (272.15 - 273.15)),
                source_state_sha256: Some(
                    Sha256Digest::try_new("e".repeat(64)).expect("source state"),
                ),
            },
        );
    }
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
    let (parent_id, slab) = accepted_v11_slab(&clock_owners, support_end_ns);
    let mut wb14_binding = test_wb14_coupled_binding();
    wb14_binding.proposed_upper_bound_s_bits = support_seconds.to_bits();
    wb14_binding.parent_support_end_ns = support_end_ns;
    wb14_binding.child_support_end_ns = support_end_ns;
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
    stage3_inputs.runtime_depth_m = runtime_swe_m * 1_000.0 / 100.0;
    stage3_inputs.runtime_density_kg_m3 = if solid_reappearance { 0.0 } else { 100.0 };
    if solid_reappearance {
        stage3_inputs
            .surface_energy_options
            .daily_solar_radiation_mj_m2 = 5.0;
        stage3_inputs
            .surface_energy_options
            .daily_extraterrestrial_radiation_mj_m2 = 10.0;
        stage3_inputs.surface_energy_options.daylight = true;
        stage3_inputs.snow_layers.clear();
    } else {
        stage3_inputs.snow_layers[0].mass_swe_m = runtime_swe_m;
        stage3_inputs.snow_layers[0].thickness_m = runtime_swe_m * 1_000.0 / 100.0;
        stage3_inputs.snow_layers[0].density_kg_m3 = 100.0;
    }
    let stage3_cold_delta_k = if equilibrium_fixture {
        0.0
    } else {
        initial_cold_delta_k
    };
    if let Some(layer) = stage3_inputs.snow_layers.first_mut() {
        layer.temperature_c = -stage3_cold_delta_k;
        layer.cold_content_j_m2 = runtime_swe_m * 1_000.0 * 2_100.0 * stage3_cold_delta_k;
    }
    let stage3_beginning = if terminal_event {
        Wb11HydrologyKernel::initialize_stage3_persistent_state_with_terminal_event(
            1,
            stage3_inputs.snow_layers.clone(),
            DirectSnowTerminalEventRequest::ENTHALPY_EVENT_V1,
        )
    } else {
        Wb11HydrologyKernel::initialize_stage3_persistent_state(
            1,
            stage3_inputs.snow_layers.clone(),
        )
    }
    .expect("persistent Stage-3 beginning");
    let mut stage3_hourly = DirectSnowHourlyForcing::zero();
    if positive_covered_rain {
        stage3_hourly.active_precipitation_m = 0.000_4;
        stage3_hourly.rain_m = 0.000_3;
        stage3_hourly.snowfall_m = 0.000_1;
        stage3_hourly.rain_fraction = 1.0;
        stage3_hourly.hydrometeor_temperature_c = Some(-1.0);
    }
    if terminal_event {
        stage3_hourly.radiation_mj_m2 = 1_000.0;
    }
    if solid_reappearance {
        // The legacy snow-depth operand is ten times its liquid-water
        // equivalent. Keep active precipitation on the water-equivalent basis
        // used by the phase-debit closure.
        stage3_hourly.active_precipitation_m = 0.05;
        stage3_hourly.snowfall_m = 0.5;
        stage3_hourly.snow_fraction = 1.0;
        stage3_hourly.hydrometeor_temperature_c = Some(0.0);
    }
    let stage3_forcing = DirectSnowStage3SupportInput {
        forcing: stage3_hourly,
        duration_seconds: support_seconds,
    };
    let mut stage3_inputs_by_lane = BTreeMap::from([(1, stage3_inputs.clone())]);
    let mut stage3_forcing_by_lane = BTreeMap::from([(1, stage3_forcing)]);
    let carrier_forcing_by_lane = BTreeMap::from([(
        1,
        if equilibrium_fixture {
            equilibrium_child2c_carrier_forcing()
        } else {
            child2c_carrier_forcing()
        },
    )]);
    let mut stage3_beginning_by_lane = BTreeMap::from([(1, stage3_beginning.clone())]);
    let mut preliminary_stage3_inputs_by_lane = if terminal_event {
        let mut value = stage3_inputs.clone();
        value.runtime_swe_m = 0.005;
        value.runtime_depth_m = 0.05;
        value.snow_layers[0].mass_swe_m = 0.005;
        value.snow_layers[0].thickness_m = 0.05;
        BTreeMap::from([(1, value)])
    } else {
        stage3_inputs_by_lane.clone()
    };
    let mut preliminary_stage3_beginning_by_lane = if terminal_event {
        BTreeMap::from([(
            1,
            Wb11HydrologyKernel::initialize_stage3_persistent_state(
                1,
                preliminary_stage3_inputs_by_lane[&1].snow_layers.clone(),
            )
            .expect("ordinary preliminary Stage-3 beginning"),
        )])
    } else {
        stage3_beginning_by_lane.clone()
    };
    if let Some(second_swe) = second_lane_swe_m {
        let mut second_inputs = stage3_inputs;
        second_inputs.runtime_swe_m = second_swe;
        second_inputs.runtime_depth_m = second_swe * 1_000.0 / 100.0;
        second_inputs.runtime_density_kg_m3 = if second_swe == 0.0 { 0.0 } else { 100.0 };
        second_inputs.snow_layers = if second_swe == 0.0 {
            Vec::new()
        } else {
            let mut layer = second_inputs.snow_layers[0];
            layer.mass_swe_m = second_swe;
            layer.thickness_m = second_swe * 1_000.0 / 100.0;
            layer.density_kg_m3 = 100.0;
            layer.cold_content_j_m2 = second_swe * 1_000.0 * 2_100.0 * 8.0;
            vec![layer]
        };
        let second_state = Wb11HydrologyKernel::initialize_stage3_persistent_state(
            2,
            second_inputs.snow_layers.clone(),
        )
        .expect("second persistent Stage-3 beginning");
        stage3_inputs_by_lane.insert(2, second_inputs.clone());
        stage3_forcing_by_lane.insert(2, stage3_forcing);
        stage3_beginning_by_lane.insert(2, second_state.clone());
        preliminary_stage3_inputs_by_lane.insert(2, second_inputs);
        preliminary_stage3_beginning_by_lane.insert(2, second_state);
    }
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
        SealedStage3TileBoundaryForcingV1::V11CanopyCovered(carrier_forcing_by_lane[&1].clone()),
    )]);
    let mut missing_open_executor = crate::v11_vegetation_consumer::DirectV11VegetationExecutor {
        stack: DirectV11SnowCoveredRealConsumerStack::new(
            &shadow,
            DirectV11SnowCoveredStackInputs {
                interval: &covered_interval,
                stage3_inputs_by_lane: &preliminary_stage3_inputs_by_lane,
                stage3_forcing_by_lane: &stage3_forcing_by_lane,
                snow_surface_forcing_by_destination: &covered_only_snow_surface_forcing,
                stage3_beginning_by_lane: preliminary_stage3_beginning_by_lane.clone(),
                pending_terminal_parcels: BTreeMap::new(),
                day_index: 0,
                interval_index: 0,
                finalize_wb14_parent_interval: true,
                wb14_coupled_child_binding: wb14_binding,
            },
        ),
    };
    if !production_only {
        let missing_error = execute_direct_v11_segment(
            &migrated.configuration,
            &parent,
            &slab,
            &mut missing_open_executor,
        )
        .expect_err("mixed OFE without its open-snow boundary must reject");
        if second_lane_swe_m.is_none()
            && !positive_covered_rain
            && !terminal_event
            && !solid_reappearance
        {
            assert!(
                matches!(
                    &missing_error,
                    V11ExecutionError::Executor(DirectV11RealConsumerError::Identity(
                        "covered Stage-3 lane is missing a snow-surface contribution"
                    ))
                ),
                "unexpected missing-boundary poison: {missing_error:?}"
            );
        }
        assert!(missing_open_executor.stack.take_staged_stage3().is_none());
        assert!(missing_open_executor.stack.take_staged_ending().is_none());
    }
    let open_record = shadow
        .inner
        .surface_configuration
        .records
        .iter()
        .find(|record| !covered_tiles.contains(&record.key.tile_id))
        .expect("mixed fixture open tile");
    let support = TimeSupport::new(ModelTimeNs::new(0), ModelTimeNs::new(support_end_ns))
        .expect("open-snow support");
    let snow_surface_reference_humidity = if equilibrium_fixture {
        equilibrium_stage3_reference_specific_humidity(
            covered_interval.lse_forcing.air_pressure_pa,
        )
    } else {
        covered_interval.lse_forcing.air_specific_humidity_kg_kg
    };
    let exposure = SealedOpenSnowExposureReceiptV1::try_new(
        support,
        (
            open_record.key.ofe_id.clone(),
            open_record.key.tile_id.clone(),
        ),
        Digest32::from_bytes([10; 32]),
        Digest32::from_bytes([11; 32]),
        covered_interval.lse_forcing.reference_wind_m_s,
        Digest32::from_bytes([12; 32]),
    )
    .expect("open-snow exposure");
    let open_forcing = SealedOpenSnowTileForcingV1::try_new(SealedOpenSnowTileForcingInputsV1 {
        support,
        destination: (
            open_record.key.ofe_id.clone(),
            open_record.key.tile_id.clone(),
        ),
        forcing_receipt_sha256: Digest32::from_bytes([10; 32]),
        exposure,
        reference_temperature_k: covered_interval.lse_forcing.air_temperature_k,
        reference_specific_humidity_kg_kg: snow_surface_reference_humidity,
        air_pressure_pa: covered_interval.lse_forcing.air_pressure_pa,
        atmospheric_downward_longwave_w_m2: covered_interval
            .lse_forcing
            .atmospheric_downward_longwave_w_m2,
        direct_vis_w_m2: covered_interval.lse_forcing.direct_vis_w_m2,
        diffuse_vis_w_m2: covered_interval.lse_forcing.diffuse_vis_w_m2,
        direct_nir_w_m2: covered_interval.lse_forcing.direct_nir_w_m2,
        diffuse_nir_w_m2: covered_interval.lse_forcing.diffuse_nir_w_m2,
        rain_m: 0.0,
        snowfall_m: if solid_reappearance { 0.5 } else { 0.0 },
        precipitation_parcel_count: usize::from(solid_reappearance),
    })
    .expect("open-snow forcing");
    let mut snow_surface_forcing_by_destination = BTreeMap::from([
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
    for binding in &shadow.inner.surface_configuration.ofe_bindings {
        if !stage3_beginning_by_lane
            .get(&binding.production_lane_id)
            .is_some_and(stage3_is_resolved_thermal_domain)
            && !stage3_forcing_by_lane
                .get(&binding.production_lane_id)
                .is_some_and(|forcing| forcing.forcing.snowfall_m > 0.0)
        {
            continue;
        }
        for record in shadow
            .inner
            .surface_configuration
            .records
            .iter()
            .filter(|record| record.key.ofe_id == binding.ofe_id)
        {
            let destination = (record.key.ofe_id.clone(), record.key.tile_id.clone());
            if snow_surface_forcing_by_destination.contains_key(&destination) {
                continue;
            }
            let exposure = SealedOpenSnowExposureReceiptV1::try_new(
                support,
                destination.clone(),
                Digest32::from_bytes([10; 32]),
                Digest32::from_bytes([11; 32]),
                covered_interval.lse_forcing.reference_wind_m_s,
                Digest32::from_bytes([12; 32]),
            )
            .expect("additional open-snow exposure");
            let forcing = SealedOpenSnowTileForcingV1::try_new(SealedOpenSnowTileForcingInputsV1 {
                support,
                destination: destination.clone(),
                forcing_receipt_sha256: Digest32::from_bytes([10; 32]),
                exposure,
                reference_temperature_k: covered_interval.lse_forcing.air_temperature_k,
                reference_specific_humidity_kg_kg: snow_surface_reference_humidity,
                air_pressure_pa: covered_interval.lse_forcing.air_pressure_pa,
                atmospheric_downward_longwave_w_m2: covered_interval
                    .lse_forcing
                    .atmospheric_downward_longwave_w_m2,
                direct_vis_w_m2: covered_interval.lse_forcing.direct_vis_w_m2,
                diffuse_vis_w_m2: covered_interval.lse_forcing.diffuse_vis_w_m2,
                direct_nir_w_m2: covered_interval.lse_forcing.direct_nir_w_m2,
                diffuse_nir_w_m2: covered_interval.lse_forcing.diffuse_nir_w_m2,
                rain_m: 0.0,
                snowfall_m: if solid_reappearance { 0.5 } else { 0.0 },
                precipitation_parcel_count: usize::from(solid_reappearance),
            })
            .expect("additional open-snow forcing");
            snow_surface_forcing_by_destination.insert(
                destination,
                SealedStage3TileBoundaryForcingV1::OpenSnow(forcing),
            );
        }
    }
    let stack = DirectV11SnowCoveredRealConsumerStack::new(
        &shadow,
        DirectV11SnowCoveredStackInputs {
            interval: &covered_interval,
            stage3_inputs_by_lane: &preliminary_stage3_inputs_by_lane,
            stage3_forcing_by_lane: &stage3_forcing_by_lane,
            snow_surface_forcing_by_destination: &snow_surface_forcing_by_destination,
            stage3_beginning_by_lane: preliminary_stage3_beginning_by_lane.clone(),
            pending_terminal_parcels: BTreeMap::new(),
            day_index: 0,
            interval_index: 0,
            finalize_wb14_parent_interval: true,
            wb14_coupled_child_binding: wb14_binding,
        },
    );
    let mut executor = crate::v11_vegetation_consumer::DirectV11VegetationExecutor { stack };
    if !production_only {
        execute_direct_v11_segment(&migrated.configuration, &parent, &slab, &mut executor)
            .expect("real mixed covered/open OFE execution");
        let lane_receipt = executor
            .stack
            .last_lane_boundary_receipts()
            .and_then(|receipts| receipts.get(&1))
            .expect("mixed OFE final lane receipt")
            .clone();
        assert_eq!(lane_receipt.ordered_destinations.len(), 2);
        assert!(
            lane_receipt
                .ordered_destinations
                .iter()
                .any(|value| value.boundary_class
                    == crate::snow_stage3_terminal_handoff::Stage3TileBoundaryClassV1::OpenSnow)
        );
        assert!(lane_receipt
            .ordered_destinations
            .iter()
            .any(|value| value.boundary_class == crate::snow_stage3_terminal_handoff::Stage3TileBoundaryClassV1::V11CanopyCovered));
        let historical_complete_candidate = executor
            .stack
            .take_staged_ending()
            .expect("historical one-child complete candidate");
        if positive_covered_rain {
            assert!(lane_receipt.precipitation_parcel_set_sha256 != Digest32::zero());
            let ending_stage3 = executor
                .stack
                .take_staged_stage3()
                .expect("positive rain Stage-3 candidate");
            let precipitation_set = executor
                .stack
                .last_precipitation_parcel_sets()
                .and_then(|sets| sets.get(&1))
                .expect("installed precipitation parcel set");
            for source in [
                Stage3PrecipitationSourceV1::VegetationTerminalThroughfall,
                Stage3PrecipitationSourceV1::VegetationTerminalStemflow,
            ] {
                assert!(precipitation_set.parcels.iter().any(|parcel| {
                    parcel.source == source && parcel.mass_kg_m2_tile_ground > 0.0
                }));
            }
            assert!(!precipitation_set.parcels.iter().any(|parcel| {
                parcel.destination_tile_id == covered_record.key.tile_id
                    && parcel.source == Stage3PrecipitationSourceV1::OpenRawRain
            }));
            assert!(precipitation_set.parcels.iter().any(|parcel| {
                parcel.destination_tile_id == open_record.key.tile_id
                    && parcel.source == Stage3PrecipitationSourceV1::OpenRawRain
                    && parcel.mass_kg_m2_tile_ground > 0.0
            }));
            let (parcel_mass, _) =
                reconstruct_precipitation_mass_and_advected_heat(precipitation_set)
                    .expect("installed precipitation reconstruction");
            let parcel_advection = precipitation_set.parcels.iter().fold(0.0, |sum, parcel| {
                let fraction = precipitation_set.destinations
                    [parcel.destination_topology_index as usize]
                    .fraction_of_ofe;
                let specific_enthalpy = match parcel.enthalpy_provider {
                    Stage3PrecipitationEnthalpyProviderV1::Temperature {
                        temperature_k,
                        reference_temperature_k,
                        specific_heat_j_kg_k,
                        ..
                    } => specific_heat_j_kg_k * (temperature_k - reference_temperature_k),
                    Stage3PrecipitationEnthalpyProviderV1::SpecificEnthalpy {
                        specific_enthalpy_j_kg,
                        ..
                    } => specific_enthalpy_j_kg,
                };
                sum + fraction * parcel.mass_kg_m2_tile_ground * specific_enthalpy
            });
            assert!(parcel_mass > 0.0);
            assert_ne!(parcel_advection.to_bits(), 0.0_f64.to_bits());
            let covered_release_kg_m2_tile = precipitation_set
                .parcels
                .iter()
                .filter(|parcel| {
                    parcel.destination_tile_id == covered_record.key.tile_id
                        && parcel.phase == Stage3PrecipitationPhaseV1::Liquid
                })
                .fold(0.0, |sum, parcel| sum + parcel.mass_kg_m2_tile_ground);
            assert!(covered_release_kg_m2_tile > 0.0);
            let (parcel_liquid, parcel_solid) =
                precipitation_set
                    .parcels
                    .iter()
                    .fold((0.0, 0.0), |(liquid, solid), parcel| {
                        let fraction = precipitation_set.destinations
                            [parcel.destination_topology_index as usize]
                            .fraction_of_ofe;
                        match parcel.phase {
                            Stage3PrecipitationPhaseV1::Liquid => {
                                (liquid + fraction * parcel.mass_kg_m2_tile_ground, solid)
                            }
                            Stage3PrecipitationPhaseV1::Solid => {
                                (liquid, solid + fraction * parcel.mass_kg_m2_tile_ground)
                            }
                        }
                    });
            assert_eq!(
                ending_stage3[&1].cumulative_external_liquid_kg_m2.to_bits(),
                parcel_liquid.to_bits()
            );
            assert_eq!(
                ending_stage3[&1].cumulative_snowfall_kg_m2.to_bits(),
                parcel_solid.to_bits()
            );
            assert!(
                historical_complete_candidate
                    .vegetation_state
                    .0
                    .occupancies
                    .values()
                    .any(|state| state.canopy_liquid_kg_h2o_m2_tile_ground > 0.0)
            );
            let physical = executor
                .stack
                .last_physical_outcome_ledgers()
                .and_then(|values| values.get(&1))
                .expect("rain-on-snow physical outcome ledger");
            assert!(physical.refreeze_kg_m2 > 0.0);
            assert_eq!(
                physical.liquid_precipitation_kg_m2.to_bits(),
                parcel_liquid.to_bits()
            );
            assert_eq!(
                physical.precipitation_advection_j_m2.to_bits(),
                parcel_advection.to_bits()
            );
            let producer_manifest = precipitation_set
                .parcels
                .iter()
                .map(|parcel| PrecipitationProducerManifestRowV1 {
                    destination_topology_index: parcel.destination_topology_index,
                    source: parcel.source,
                    semantic_receipt_ordinal: parcel.semantic_receipt_ordinal,
                    mass_kg_m2_tile_ground: parcel.mass_kg_m2_tile_ground,
                    enthalpy_provider: parcel.enthalpy_provider.clone(),
                    source_identity_sha256: parcel.source_identity_sha256,
                    producer_beginning_state_sha256: parcel.producer_beginning_state_sha256,
                })
                .collect::<Vec<_>>();
            for source in [
                Stage3PrecipitationSourceV1::AtmosphericGroundSnow,
                Stage3PrecipitationSourceV1::OpenRawRain,
                Stage3PrecipitationSourceV1::VegetationTerminalThroughfall,
            ] {
                let omitted = producer_manifest
                    .iter()
                    .find(|row| row.source == source && row.mass_kg_m2_tile_ground > 0.0)
                    .expect("positive producer route");
                let mut resealed_omission = precipitation_set.clone();
                resealed_omission.parcels.retain(|parcel| {
                    parcel.destination_topology_index != omitted.destination_topology_index
                        || parcel.source != omitted.source
                        || parcel.semantic_receipt_ordinal != omitted.semantic_receipt_ordinal
                });
                resealed_omission.receipt_sha256 = Digest32::zero();
                let resealed_omission = resealed_omission.seal().expect("resealed omission poison");
                assert!(matches!(
                    validate_precipitation_producer_manifest(
                        &resealed_omission,
                        &producer_manifest
                    ),
                    Err(DirectV11RealConsumerError::Identity(
                        "precipitation producer route parcel cardinality"
                    ))
                ));
            }
            for source in [
                Stage3PrecipitationSourceV1::AtmosphericGroundSnow,
                Stage3PrecipitationSourceV1::OpenRawRain,
                Stage3PrecipitationSourceV1::VegetationTerminalThroughfall,
            ] {
                let original = precipitation_set
                    .parcels
                    .iter()
                    .find(|parcel| parcel.source == source && parcel.mass_kg_m2_tile_ground > 0.0)
                    .expect("positive producer route");
                let mut extra = original.clone();
                extra.semantic_receipt_ordinal = precipitation_set
                    .parcels
                    .iter()
                    .filter(|parcel| {
                        parcel.destination_topology_index == original.destination_topology_index
                            && parcel.source == source
                    })
                    .map(|parcel| parcel.semantic_receipt_ordinal)
                    .max()
                    .expect("existing producer ordinal")
                    + 1;
                extra.receipt_sha256 = Digest32::zero();
                let extra = extra.seal().expect("resealed extra parcel poison");
                let mut resealed_extra = precipitation_set.clone();
                resealed_extra.parcels.push(extra);
                resealed_extra.parcels.sort_by_key(|parcel| {
                    (
                        parcel.lane_id,
                        parcel.destination_topology_index,
                        parcel.phase,
                        parcel.source,
                        parcel.semantic_receipt_ordinal,
                    )
                });
                resealed_extra.receipt_sha256 = Digest32::zero();
                let resealed_extra = resealed_extra.seal().expect("resealed extra set poison");
                assert!(matches!(
                    validate_precipitation_producer_manifest(&resealed_extra, &producer_manifest),
                    Err(DirectV11RealConsumerError::Identity(
                        "precipitation producer route parcel cardinality"
                    ))
                ));
            }
            let mut precipitation_seal_poison = precipitation_set.clone();
            precipitation_seal_poison.parcels[0].mass_kg_m2_tile_ground += 1.0;
            assert!(
                crate::snow_stage3_v11_attachment::validate_precipitation_phase_parcel_set(
                    &precipitation_seal_poison
                )
                .is_err()
            );
            let snow_soil_receipt = executor
                .stack
                .last_snow_soil_heat_receipts()
                .and_then(|receipts| receipts.get(&1))
                .expect("installed rainy snow-soil heat receipt");
            crate::snow_stage3_v11_attachment::validate_snow_soil_heat_receipt(snow_soil_receipt)
                .expect("installed rainy snow-soil receipt validates");
            let mut snow_soil_poison = snow_soil_receipt.clone();
            snow_soil_poison.soil_candidate_heat_j_m2_ofe_ground += 1.0;
            assert!(
                crate::snow_stage3_v11_attachment::validate_snow_soil_heat_receipt(
                    &snow_soil_poison
                )
                .is_err()
            );
        }
    }

    let identities = shadow
        .inner
        .surface_configuration
        .ofe_bindings
        .iter()
        .map(|binding| {
            (
                binding.production_lane_id,
                shadow
                    .inner
                    .surface_configuration
                    .records
                    .iter()
                    .filter(|record| record.key.ofe_id == binding.ofe_id)
                    .map(|record| {
                        let solid_parcels = solid_reappearance.then(|| {
                            vec![crate::runtime_inputs::SnowFreeSolidPrecipitationParcelReceipt {
                                parcel_id: format!(
                                    "solid-reappearance:{}:{}",
                                    record.key.ofe_id.as_str(),
                                    record.key.tile_id.as_str()
                                ),
                                source_owner_id: "climate-solid-fixture".to_owned(),
                                destination_ofe_id: record.key.ofe_id.as_str().to_owned(),
                                destination_tile_id: record.key.tile_id.as_str().to_owned(),
                                start_s: 0.0,
                                end_s: support_seconds,
                                // Snow-free provider solid parcels use the
                                // legacy 100 kg m-2 per metre-of-snow basis.
                                // Every destination carries the same depth;
                                // the production transition applies the sealed
                                // tile fractions when reconstructing OFE mass.
                                mass_kg_m2: 0.5 * 100.0,
                                temperature_k: 273.15,
                                enthalpy_j_m2: 0.0,
                            }]
                        });
                        PreparedStage3V11SupportIdentityV1::new_with_phase_parcels(
                            record.key.ofe_id.as_str().to_owned(),
                            record.key.tile_id.as_str().to_owned(),
                            "a".repeat(64),
                            Digest32::from_bytes([13; 32]),
                            Vec::new(),
                            solid_parcels.unwrap_or_default(),
                            Digest32::from_bytes([14; 32]),
                        )
                    })
                    .collect::<Vec<_>>(),
            )
        })
        .collect::<BTreeMap<_, _>>();
    let mut snow_free_parent_interval = base_interval.clone();
    snow_free_parent_interval
        .lse_forcing
        .snow_present_at_beginning = false;
    snow_free_parent_interval.lse_forcing.snow_present_at_end = solid_reappearance;
    snow_free_parent_interval.lse_forcing.forcing_sha256 = snow_free_parent_interval
        .lse_forcing
        .canonical_sha256()
        .expect("snow-free parent forcing digest");
    let prepared_result = if short_support {
        PreparedStage3V11SupportV1::try_new_for_short_production_test(
            support,
            stage3_inputs_by_lane.clone(),
            stage3_forcing_by_lane.clone(),
            snow_free_parent_interval,
            identities,
        )
    } else {
        PreparedStage3V11SupportV1::try_new(
            support,
            stage3_inputs_by_lane.clone(),
            stage3_forcing_by_lane.clone(),
            snow_free_parent_interval,
            identities,
        )
    };
    let mut prepared = prepared_result
        .expect("coupled cadence prepared support")
        .with_covered_v11_interval(covered_interval.clone());
    for (destination, forcing) in &snow_surface_forcing_by_destination {
        prepared = match forcing {
            SealedStage3TileBoundaryForcingV1::V11CanopyCovered(value) => {
                prepared.with_covered_tile_forcing(destination.clone(), value.clone())
            }
            SealedStage3TileBoundaryForcingV1::OpenSnow(value) => {
                prepared.with_sealed_open_tile_forcing(destination.clone(), value.clone())
            }
        };
    }
    if let Some(boundary_ns) = hard_boundary_ns {
        prepared = prepared
            .with_hard_boundaries(vec![ModelTimeNs::new(boundary_ns)])
            .expect("accepted coupled hard boundary");
    }
    let mut beginning_owners = initial_v11_owners(&shadow, &migrated.state);
    if solid_reappearance {
        let beginning_snow_owner_bytes = crate::v9_real_consumer_shadow::v11_covered::
            canonical_stage3_snow_owner_bytes_v11(&stage3_beginning_by_lane)
            .expect("solid-reappearance beginning snow owner bytes");
        beginning_owners.insert(
            "snow".to_owned(),
            V11OwnerEnvelope::try_new("snow".to_owned(), beginning_snow_owner_bytes)
                .expect("solid-reappearance beginning snow owner"),
        );
    }
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
    // The solid-reappearance event is a zero-duration mutation of the snow
    // owner at the parent start. Its parent and coupled clock must therefore
    // begin from the same canonical empty-snow lane bytes, not the generic
    // placeholder used by older WB14-only fixtures. Derive the parent identity
    // from that corrected owner set as the coupled-time authority does.
    let parent = V11ParentTransaction::new_with_complete_owners(
        &migrated.configuration,
        &migrated.state,
        beginning_clock.parent_transaction_id(),
        ModelTimeNs::new(0),
        beginning_owners.clone(),
    )
    .expect("adaptive covered parent");
    let context = DirectSnowStage3V11StaticContext {
        run_identity: digest(1),
        topology_identity: digest(9),
        parent_duration_ns: support_end_ns,
        minimum_support_ns: 60_000_000_000,
        calendar_receipt: digest(2),
        controller_policy: digest(5),
        parent_sequence: 40,
        lane_ids: stage3_beginning_by_lane.keys().copied().collect(),
        vegetation_configuration: migrated.configuration.clone(),
        surface_liquid_configuration: shadow.inner.surface_configuration.clone(),
        wb14_parameters: covered_interval.wb14_parameters.clone(),
    };
    let selected_seconds = stage3_beginning_by_lane
        .values()
        .filter(|state| {
            stage3_is_resolved_thermal_domain(state)
                || crate::hydrology::stage3_is_terminal_event_domain(state)
        })
        .map(|state| {
            if crate::hydrology::stage3_is_terminal_event_domain(state) {
                Wb11HydrologyKernel::project_stage3_terminal_surface_state_v1(state)
            } else {
                Wb11HydrologyKernel::project_stage3_surface_state_v1(state)
            }
            .expect("coupled cadence projection")
            .selected_substep_seconds
        })
        .reduce(f64::min)
        .unwrap_or(support_seconds);
    let rollback_parent = parent.clone();
    let rollback_consumer = shadow.clone();
    let rollback_clock = beginning_clock.clone();
    let rollback_stage3 = stage3_beginning.clone();
    let injections = if include_child_17 {
        vec![
            Stage3V11FailureInjection::OutcomeLedgerBuilt(1),
            Stage3V11FailureInjection::PrecipitationReceiptRejected(1),
            Stage3V11FailureInjection::SnowSoilHeatReceiptRejected(1),
            Stage3V11FailureInjection::SubslabAccepted(1),
            Stage3V11FailureInjection::FinalOwnerJoinCompleted,
        ]
    } else {
        Vec::new()
    };
    for injection in injections {
        assert!(
            execute_covered_real_v11_parent(
                &context,
                &parent,
                &shadow,
                &beginning_clock,
                &prepared,
                0,
                0,
                digest(3),
                stage3_beginning_by_lane.clone(),
                BTreeMap::new(),
                Some(injection),
            )
            .is_err()
        );
        assert_eq!(parent, rollback_parent);
        assert_eq!(shadow, rollback_consumer);
        assert_eq!(beginning_clock, rollback_clock);
        assert_eq!(stage3_beginning, rollback_stage3);
    }
    if capture_terminal_failure {
        crate::snow_stage3_v11_attachment::begin_terminal_provider_support_audit();
        let no_evidence_result = execute_covered_real_v11_parent(
            &context,
            &parent,
            &shadow,
            &beginning_clock,
            &prepared,
            0,
            0,
            digest(3),
            stage3_beginning_by_lane.clone(),
            BTreeMap::new(),
            None,
        );
        let no_evidence_supports =
            crate::snow_stage3_v11_attachment::take_terminal_provider_support_audit();
        crate::snow_stage3_v11_attachment::begin_terminal_provider_support_audit();
        let (result, evidence) =
            crate::snow_stage3_v11_attachment::execute_covered_real_v11_parent_capture(
                &context,
                &parent,
                &shadow,
                &beginning_clock,
                &prepared,
                0,
                0,
                digest(3),
                stage3_beginning_by_lane.clone(),
                BTreeMap::new(),
                None,
            );
        let capture_supports =
            crate::snow_stage3_v11_attachment::take_terminal_provider_support_audit();
        assert_eq!(no_evidence_supports, capture_supports);
        assert_eq!(
            capture_supports,
            evidence
                .provider_calls
                .iter()
                .map(|call| call.request.support)
                .collect::<Vec<_>>()
        );
        if let (Ok(_), Ok(captured)) = (&no_evidence_result, &result) {
            let (_, _, ending_clock, _, _, _, event_groups, terminal_parcels) = captured;
            assert_eq!(ending_clock.accepted_until(), ModelTimeNs::new(support_end_ns));
            assert_eq!(event_groups.len(), 1);
            let group = &event_groups[0];
            assert_eq!(group.tick, ModelTimeNs::new(540_000_000_000));
            let accepted_event = group
                .accepted_event_receipt
                .as_ref()
                .expect("accepted terminal event evidence");
            assert_eq!(accepted_event.tick(), group.tick);
            assert_eq!(terminal_parcels.len(), 1);
            let parcel = &terminal_parcels[0];
            assert_eq!(
                parcel.posture,
                crate::snow_stage3_v11_attachment::DirectSnowStage3V11TerminalParcelPosture::Consumed,
            );
            assert!(
                group
                    .produced_unconsumed_parcel_digests
                    .contains(&parcel.parcel_digest)
            );
            assert!(!evidence.provider_calls.is_empty());
            assert!(evidence
                .provider_calls
                .iter()
                .all(|call| call.request.support.duration_ns() >= 60_000_000_000));
            assert_eq!(shadow, rollback_consumer);
            assert_eq!(beginning_clock, rollback_clock);
            assert_eq!(stage3_beginning, rollback_stage3);
            return;
        }
        fn is_refinement_source(error: &DirectSnowStage3V11AttachmentError) -> bool {
            match error {
                DirectSnowStage3V11AttachmentError::AdaptiveTrial { source, .. } => {
                    is_refinement_source(source)
                }
                DirectSnowStage3V11AttachmentError::AdaptiveRefinement(_) => true,
                DirectSnowStage3V11AttachmentError::Owner(
                    crate::v9_real_consumer_shadow::DirectV11RealConsumerError::AdaptiveRefinement(
                        _,
                    ),
                ) => true,
                DirectSnowStage3V11AttachmentError::V11(V11ExecutionError::Executor(
                    crate::v9_real_consumer_shadow::DirectV11RealConsumerError::AdaptiveRefinement(
                        _,
                    ),
                )) => true,
                _ => false,
            }
        }
        let is_current_adaptive_refinement =
            |result: &Result<_, DirectSnowStage3V11AttachmentError>| match result {
                Err(error) => is_refinement_source(error),
                Ok(_) => false,
            };
        if is_current_adaptive_refinement(&no_evidence_result)
            && is_current_adaptive_refinement(&result)
        {
            assert!(!evidence.provider_calls.is_empty());
            assert!(
                evidence
                    .provider_calls
                    .iter()
                    .all(|call| call.request.support.duration_ns() >= 60_000_000_000)
            );
            assert_eq!(shadow, rollback_consumer);
            assert_eq!(beginning_clock, rollback_clock);
            assert_eq!(stage3_beginning, rollback_stage3);
            return;
        }
        assert!(matches!(
            &no_evidence_result,
            Err(DirectSnowStage3V11AttachmentError::Stage3(
                DirectSnowStage3EvaluationError::TerminalNumerics(
                    crate::SnowTerminalNumericsFailure::BelowCarrierDomain
                )
            ))
        ), "unexpected uncaptured result: {no_evidence_result:?}; captured result: {result:?}");
        assert!(matches!(
            result,
            Err(DirectSnowStage3V11AttachmentError::Stage3(
                DirectSnowStage3EvaluationError::TerminalNumerics(
                    crate::SnowTerminalNumericsFailure::BelowCarrierDomain
                )
            ))
        ));
        let pair = evidence
            .pairs
            .iter()
            .find(|pair| pair.duration_s.to_bits() == 1.875_f64.to_bits())
            .expect("selected 1.875-second rejected pair");
        let real_layout = evidence
            .provider_calls
            .iter()
            .find_map(|call| match &call.outcome {
                crate::hydrology::CapturedProviderOutcome::Success(result)
                    if f64::from_bits(call.request.support.duration_s_bits()).to_bits()
                        == pair.duration_s.to_bits() =>
                {
                    Some(result.candidate_layout_counts_v1())
                }
                _ => None,
            })
            .expect("real candidate layout");
        assert_eq!(real_layout.owner_count, 7);
        assert_eq!(real_layout.snow_lane_count, 1);
        assert_eq!(real_layout.soil_layer_count, 6);
        assert_eq!(real_layout.covered_destination_count, 1);
        assert_eq!(real_layout.lse_component_surface_count, 8);
        assert_eq!(real_layout.lower_boundary_count, 2);
        assert_eq!(real_layout.precipitation_lane_count, 1);
        eprintln!(
            "CHILD1_REAL_CANDIDATE_LAYOUT={}",
            serde_json::to_string(&real_layout).expect("layout JSON")
        );
        assert!(pair.rejected);
        assert_eq!(pair.components[3].0.to_bits(), 0x4094_9afb_c192_8120);
        assert_eq!(pair.components[3].1.to_bits(), 0x4094_2e21_8363_bae1);
        assert_eq!(pair.components[3].2.to_bits(), 0xc03b_368f_8bb1_8fc0);
        let trials = &evidence.selected_trials[evidence.selected_trials.len() - 3..];
        for (position, trial) in ["coarse", "fine-1", "fine-2"].into_iter().zip(trials) {
            eprintln!(
                "CHILD1_DISCRETE_ENDPOINT position={position} duration_bits={:#018x} ice_bits={:#018x} liquid_bits={:#018x} cold_bits={:#018x} unallocated_bits={:#018x} complete_energy_bits={:#018x}",
                trial.duration_s.to_bits(),
                trial.ending.ice_kg_m2.to_bits(),
                trial.ending.liquid_kg_m2.to_bits(),
                trial.ending.cold_content_j_m2.to_bits(),
                trial.ledger.unallocated_energy_j_m2.to_bits(),
                trial.ledger.complete_energy_j_m2.to_bits(),
            );
        }
        // Typed Stage-3/hydrology supply proof: terminal liquid is absent from
        // every selected trial's live external-liquid operand.
        assert!(
            trials
                .iter()
                .all(|trial| { trial.ledger.external_liquid_kg_m2.to_bits() == 0.0_f64.to_bits() })
        );
        let admission = evidence.admissions.last().expect("floor admission");
        assert_eq!(admission.0.to_bits(), 0.6_f64.to_bits());
        assert_eq!(admission.1.to_bits(), 0.0_f64.to_bits());
        assert_eq!(admission.2.to_bits(), 0.6_f64.to_bits());
        assert_eq!(
            admission.3,
            crate::hydrology::TerminalFloorDecision::Accepted
        );
        assert_eq!(admission.4, admission.5);
        run_real_discrete_endpoint_probes(
            &shadow,
            &beginning_clock,
            &prepared,
            &stage3_beginning_by_lane,
            selected_seconds,
        );
        assert_eq!(shadow, rollback_consumer);
        assert_eq!(beginning_clock, rollback_clock);
        assert_eq!(stage3_beginning, rollback_stage3);
        assert!(!evidence.provider_calls.is_empty());
        assert_eq!(
            evidence.provider_calls.len(),
            evidence.coupling_iterations.len()
        );
        assert!(
            evidence
                .provider_calls
                .iter()
                .enumerate()
                .all(|(ordinal, call)| call.ordinal == ordinal as u64
                    && matches!(
                        call.outcome,
                        crate::hydrology::CapturedProviderOutcome::Success(_)
                    ))
        );
        assert!(evidence.coupling_iterations.iter().all(|iteration| {
            let request = &iteration.hook.request;
            let comparison_shape = if request.coupling_iteration == 0 {
                iteration.hook.comparisons.is_none() && request.ending_snow_hint.is_none()
            } else {
                iteration.hook.comparisons.is_some() && request.ending_snow_hint.is_some()
            };
            comparison_shape
                && evidence
                    .provider_calls
                    .iter()
                    .filter(|call| {
                        call.request.support == request.support
                            && call.request.role == request.role
                            && call.request.attempt_ordinal == request.attempt_ordinal
                            && call.request.coupling_iteration == request.coupling_iteration
                            && call.request.lane_id == request.lane_id
                            && call.request.beginning_joint.receipt_sha256()
                                == request.beginning_joint.receipt_sha256()
                    })
                    .count()
                    == 1
        }));
        assert!(evidence.coupling_selections.iter().all(|selection| {
            selection.reason
                == crate::hydrology::TerminalCouplingSelectionReason::FourComponentConvergenceBreak
                && selection.post_loop_three_component_check
        }));
        assert!(evidence.coupling_selections.iter().all(|selection| {
            let calls = evidence
                .provider_calls
                .iter()
                .filter(|call| {
                    call.request.lane_id == selection.request.lane_id
                        && call.request.support == selection.request.support
                        && call.request.role == selection.request.role
                        && call.request.attempt_ordinal == selection.request.attempt_ordinal
                        && call.request.beginning_joint.receipt_sha256()
                            == selection.request.beginning_joint.receipt_sha256()
                })
                .collect::<Vec<_>>();
            assert_eq!(calls.len(), 2, "real carrier coupling group cardinality");
            assert_eq!(calls[0].request.coupling_iteration, 0);
            assert!(calls[0].request.ending_snow_hint.is_none());
            assert_eq!(calls[1].request.coupling_iteration, 1);
            assert!(calls[1].request.ending_snow_hint.is_some());
            let second_iteration = evidence
                .coupling_iterations
                .iter()
                .find(|iteration| {
                    iteration.hook.request.lane_id == calls[1].request.lane_id
                        && iteration.hook.request.support == calls[1].request.support
                        && iteration.hook.request.role == calls[1].request.role
                        && iteration.hook.request.attempt_ordinal
                            == calls[1].request.attempt_ordinal
                        && iteration.hook.request.coupling_iteration
                            == calls[1].request.coupling_iteration
                        && iteration.hook.request.beginning_joint.receipt_sha256()
                            == calls[1].request.beginning_joint.receipt_sha256()
                })
                .expect("second real coupling iteration");
            assert!(
                second_iteration
                    .hook
                    .comparisons
                    .expect("second-iteration comparisons")
                    .iter()
                    .all(|comparison| comparison.2.to_bits() == 0.0_f64.to_bits())
            );
            let (
                crate::hydrology::CapturedProviderOutcome::Success(first),
                crate::hydrology::CapturedProviderOutcome::Success(second),
            ) = (&calls[0].outcome, &calls[1].outcome)
            else {
                panic!("real coupling group must contain two successes");
            };
            // Only `ending_snow_hint` and `coupling_iteration` differ. The
            // genuine carrier transition, ending joint, and retained evidence
            // projection are invariant to those generic-loop fields.
            assert_eq!(first.transition.boundary, second.transition.boundary);
            assert_eq!(
                first.transition.beginning_joint,
                second.transition.beginning_joint
            );
            assert_eq!(
                first.transition.ending_joint,
                second.transition.ending_joint
            );
            assert_eq!(
                first.transition.probe_child_identity,
                second.transition.probe_child_identity
            );
            assert_eq!(first.precipitation_sets, second.precipitation_sets);
            assert_eq!(
                first.complete_lower_boundaries,
                second.complete_lower_boundaries
            );
            assert_eq!(
                first.carrier_source_receipts,
                second.carrier_source_receipts
            );
            assert_eq!(first.covered_lse_states, second.covered_lse_states);
            assert_eq!(first.soil_candidate, second.soil_candidate);
            assert_eq!(
                first.soil_top_boundary_credit,
                second.soil_top_boundary_credit
            );
            assert_eq!(
                first.wb14_child_receipt_set_sha256,
                second.wb14_child_receipt_set_sha256
            );
            assert_eq!(
                first.wb14_child_replay_bytes,
                second.wb14_child_replay_bytes
            );
            true
        }));
        assert!(!evidence.coupling_selections.iter().any(|selection| {
            selection.reason
                == crate::hydrology::TerminalCouplingSelectionReason::IterationLoopExhausted
        }));
        assert_eq!(evidence.selected_trials.len(), evidence.pairs.len() * 3);
        assert!(evidence.selected_trials.chunks_exact(3).all(|trials| {
            trials[0].position == crate::hydrology::TerminalPairPosition::Coarse
                && trials[1].position == crate::hydrology::TerminalPairPosition::Fine1
                && trials[2].position == crate::hydrology::TerminalPairPosition::Fine2
                && trials[1].role == crate::hydrology::CoveredTerminalTrialRoleV1::Half1
                && trials[2].role == crate::hydrology::CoveredTerminalTrialRoleV1::Half2
                && trials[2].beginning == trials[1].ending
                && trials[2].beginning_joint == trials[1].hydrology_ending_joint
        }));
        assert!(evidence.pairs.iter().all(|pair| {
            pair.components.len() == 5
                && pair.maximum_scaled.to_bits()
                    == pair
                        .components
                        .iter()
                        .fold(0.0_f64, |maximum, component| maximum.max(component.4))
                        .to_bits()
                && pair.rejected == (pair.maximum_scaled > 1.0 && pair.components[0].1 > 0.0)
        }));
        assert_eq!(
            pair.proposed_next_duration_s.to_bits(),
            admission.0.to_bits()
        );
        assert!(evidence.provider_calls.iter().all(|call| {
            call.request.support.end_ns().get() - call.request.support.start_ns().get()
                >= 60_000_000_000
        }));
        // Typed WB14 authorization/credit proof.
        assert!(evidence.provider_calls.iter().all(|call| {
            match &call.outcome {
                crate::hydrology::CapturedProviderOutcome::Success(result) => !result
                    .carrier_envelope
                    .hydrology()
                    .surface_ingress()
                    .receipts()
                    .iter()
                    .any(|receipt| {
                        receipt.kind
                        == crate::direct_runtime::DirectSurfaceLiquidParcelKind::TerminalReceiver
                    }),
                crate::hydrology::CapturedProviderOutcome::Failure(_) => true,
            }
        }));
        // Typed input surface-liquid ingress proof, independent of WB14 receipts.
        assert!(evidence.provider_calls.iter().all(|call| {
            match &call.outcome {
                crate::hydrology::CapturedProviderOutcome::Success(result) => !result
                    .carrier_envelope
                    .hydrology()
                    .surface_ingress()
                    .open_ingress_parcels()
                    .iter()
                    .any(|parcel| {
                        parcel.kind
                        == crate::direct_runtime::DirectSurfaceLiquidParcelKind::TerminalReceiver
                    }),
                crate::hydrology::CapturedProviderOutcome::Failure(_) => true,
            }
        }));
        macro_rules! rejects_poison {
            ($label:literal, $mutate:expr) => {{
                let mut poisoned = evidence.clone();
                ($mutate)(&mut poisoned);
                assert!(poisoned.validate().is_err(), $label);
            }};
        }
        rejects_poison!(
            "missing provider",
            |poisoned: &mut crate::hydrology::CaptureState| {
                poisoned.provider_calls.remove(0);
            }
        );
        rejects_poison!(
            "duplicate provider",
            |poisoned: &mut crate::hydrology::CaptureState| {
                poisoned
                    .provider_calls
                    .insert(0, poisoned.provider_calls[0].clone());
            }
        );
        rejects_poison!(
            "reordered provider",
            |poisoned: &mut crate::hydrology::CaptureState| {
                poisoned.provider_calls.swap(0, 1);
            }
        );
        rejects_poison!(
            "substituted provider key",
            |poisoned: &mut crate::hydrology::CaptureState| {
                poisoned.provider_calls[0].request.attempt_ordinal ^= 1;
            }
        );
        rejects_poison!(
            "missing coupling iteration",
            |poisoned: &mut crate::hydrology::CaptureState| {
                poisoned.coupling_iterations.remove(0);
            }
        );
        rejects_poison!(
            "duplicate coupling iteration",
            |poisoned: &mut crate::hydrology::CaptureState| {
                poisoned
                    .coupling_iterations
                    .insert(0, poisoned.coupling_iterations[0].clone());
            }
        );
        rejects_poison!(
            "reordered coupling iteration",
            |poisoned: &mut crate::hydrology::CaptureState| {
                poisoned.coupling_iterations.swap(0, 1);
            }
        );
        rejects_poison!(
            "substituted coupling key",
            |poisoned: &mut crate::hydrology::CaptureState| {
                poisoned.coupling_iterations[0].hook.request.attempt_ordinal ^= 1;
            }
        );
        rejects_poison!(
            "coupling comparison",
            |poisoned: &mut crate::hydrology::CaptureState| {
                let iteration = poisoned
                    .coupling_iterations
                    .iter_mut()
                    .find(|iteration| iteration.hook.comparisons.is_some())
                    .expect("comparison iteration");
                iteration.hook.comparisons.as_mut().expect("comparisons")[0].2 += 1.0;
            }
        );
        rejects_poison!(
            "missing coupling selection",
            |poisoned: &mut crate::hydrology::CaptureState| {
                poisoned.coupling_selections.remove(0);
            }
        );
        rejects_poison!(
            "duplicate coupling selection",
            |poisoned: &mut crate::hydrology::CaptureState| {
                poisoned
                    .coupling_selections
                    .insert(0, poisoned.coupling_selections[0].clone());
            }
        );
        rejects_poison!(
            "reordered coupling selection",
            |poisoned: &mut crate::hydrology::CaptureState| {
                poisoned.coupling_selections.swap(0, 1);
            }
        );
        rejects_poison!(
            "substituted coupling selection",
            |poisoned: &mut crate::hydrology::CaptureState| {
                poisoned.coupling_selections[0].request.attempt_ordinal ^= 1;
            }
        );
        rejects_poison!(
            "selected convergence reason",
            |poisoned: &mut crate::hydrology::CaptureState| {
                poisoned.coupling_selections[0].reason =
                    crate::hydrology::TerminalCouplingSelectionReason::IterationLoopExhausted;
            }
        );
        rejects_poison!(
            "selected live convergence",
            |poisoned: &mut crate::hydrology::CaptureState| {
                let request = poisoned.coupling_selections[0].request.clone();
                poisoned
                    .coupling_iterations
                    .iter_mut()
                    .find(|iteration| {
                        iteration.hook.request.support == request.support
                            && iteration.hook.request.role == request.role
                            && iteration.hook.request.attempt_ordinal == request.attempt_ordinal
                            && iteration.hook.request.coupling_iteration
                                == request.coupling_iteration
                    })
                    .expect("selected iteration")
                    .hook
                    .converged = false;
            }
        );
        rejects_poison!(
            "selected trial order",
            |poisoned: &mut crate::hydrology::CaptureState| {
                poisoned.selected_trials.swap(0, 1);
            }
        );
        rejects_poison!(
            "missing selected trial",
            |poisoned: &mut crate::hydrology::CaptureState| {
                poisoned.selected_trials.remove(0);
            }
        );
        rejects_poison!(
            "duplicate selected trial",
            |poisoned: &mut crate::hydrology::CaptureState| {
                poisoned
                    .selected_trials
                    .insert(0, poisoned.selected_trials[0].clone());
            }
        );
        rejects_poison!(
            "substituted selected trial",
            |poisoned: &mut crate::hydrology::CaptureState| {
                poisoned.selected_trials[0].duration_s += 1.0;
            }
        );
        rejects_poison!(
            "selected joint join",
            |poisoned: &mut crate::hydrology::CaptureState| {
                poisoned.selected_trials[2].beginning_joint = None;
            }
        );
        rejects_poison!(
            "maximum-scaled conjunct",
            |poisoned: &mut crate::hydrology::CaptureState| {
                poisoned.pairs.last_mut().expect("pair").maximum_scaled = 1.0;
            }
        );
        rejects_poison!(
            "refined-ice conjunct",
            |poisoned: &mut crate::hydrology::CaptureState| {
                poisoned.pairs.last_mut().expect("pair").components[0].1 = 0.0;
            }
        );
        rejects_poison!(
            "decision delta",
            |poisoned: &mut crate::hydrology::CaptureState| {
                poisoned.pairs[0].components[0].2 += 1.0;
            }
        );
        rejects_poison!(
            "decision denominator",
            |poisoned: &mut crate::hydrology::CaptureState| {
                poisoned.pairs[0].components[0].3 += 1.0;
            }
        );
        rejects_poison!(
            "decision scaled",
            |poisoned: &mut crate::hydrology::CaptureState| {
                poisoned.pairs[0].components[0].4 += 1.0;
            }
        );
        rejects_poison!(
            "missing pair",
            |poisoned: &mut crate::hydrology::CaptureState| {
                poisoned.pairs.remove(0);
            }
        );
        rejects_poison!(
            "duplicate pair",
            |poisoned: &mut crate::hydrology::CaptureState| {
                poisoned.pairs.insert(0, poisoned.pairs[0].clone());
            }
        );
        rejects_poison!(
            "reordered pair",
            |poisoned: &mut crate::hydrology::CaptureState| {
                poisoned.pairs.swap(0, 1);
            }
        );
        rejects_poison!(
            "substituted pair",
            |poisoned: &mut crate::hydrology::CaptureState| {
                poisoned.pairs[0].duration_s += 1.0;
            }
        );
        rejects_poison!(
            "missing floor",
            |poisoned: &mut crate::hydrology::CaptureState| {
                poisoned.admissions.clear();
            }
        );
        rejects_poison!(
            "duplicate floor",
            |poisoned: &mut crate::hydrology::CaptureState| {
                poisoned.admissions.push(poisoned.admissions[0].clone());
            }
        );
        rejects_poison!(
            "floor outcome",
            |poisoned: &mut crate::hydrology::CaptureState| {
                poisoned.admissions[0].3 = crate::hydrology::TerminalFloorDecision::Rejected;
            }
        );
        let validated = evidence
            .validate()
            .expect("raw terminal evidence validates");
        assert!(validated.call_count_through_final_pair < validated.call_count_at_floor);
        assert_eq!(validated.pairs.last().unwrap().trials.len(), 3);
        assert!(validated.pairs.last().unwrap().decision.rejected);
        assert_eq!(
            validated.floor.decision,
            crate::hydrology::TerminalFloorDecision::Accepted
        );
        assert_eq!(parent, rollback_parent);
        assert_eq!(shadow, rollback_consumer);
        assert_eq!(beginning_clock, rollback_clock);
        assert_eq!(stage3_beginning, rollback_stage3);
        return;
    }
    let (
        _,
        ending_consumer,
        ending_clock,
        finalized_parent,
        ending_stage3,
        subslabs,
        event_groups,
        terminal_parcels,
    ) = execute_covered_real_v11_parent(
        &context,
        &parent,
        &shadow,
        &beginning_clock,
        &prepared,
        0,
        0,
        digest(3),
        stage3_beginning_by_lane.clone(),
        BTreeMap::new(),
        None,
    )
    .expect("synchronized covered parent cadence");
    if production_only {
        let comparison = ending_consumer
            .adaptive_complete_owner_comparison_v1(
                crate::v9_real_consumer_shadow::v11_covered::canonical_stage3_snow_owner_bytes_v11(
                    &ending_stage3,
                )
                .expect("final adaptive snow owner bytes"),
            )
            .expect("final complete-owner physical comparison");
        adaptive_production_path_coverage::record_final_physical_comparison(comparison);
    }
    if terminal_event {
        if second_lane_swe_m.is_none() {
            // The parent returns retained custody evidence, not pending
            // custody. One terminal event occurs at 540 seconds and its
            // produced parcel is consumed by the immediately following
            // receiver support.
            assert_eq!(event_groups.len(), 1);
            let group = &event_groups[0];
            assert_eq!(group.tick, ModelTimeNs::new(540_000_000_000));
            let accepted_event = group
                .accepted_event_receipt
                .as_ref()
                .expect("accepted terminal event evidence");
            assert_eq!(accepted_event.tick(), group.tick);
            assert_eq!(terminal_parcels.len(), 1);
            let parcel = &terminal_parcels[0];
            assert_eq!(
                parcel.posture,
                crate::snow_stage3_v11_attachment::DirectSnowStage3V11TerminalParcelPosture::Consumed,
            );
            assert!(
                group
                    .produced_unconsumed_parcel_digests
                    .contains(&parcel.parcel_digest)
            );
        }
        return;
    }
    assert_eq!(ending_clock.accepted_until(), support.end_ns());
    assert!(!subslabs.is_empty());
    assert!(
        subslabs.len() <= 256,
        "stable adaptive parent exceeded its performance budget"
    );
    let mut adaptive_cursor = support.start_ns();
    for receipt in &subslabs {
        assert_eq!(receipt.support.start_ns(), adaptive_cursor);
        assert!(receipt.support.duration_ns() >= 60_000_000_000);
        assert_eq!(receipt.support.duration_ns() % 60_000_000_000, 0);
        assert!(receipt.support.end_ns() <= support.end_ns());
        adaptive_cursor = receipt.support.end_ns();
    }
    assert_eq!(adaptive_cursor, support.end_ns());
    if let Some(boundary_ns) = hard_boundary_ns {
        let boundary = ModelTimeNs::new(boundary_ns);
        assert!(
            subslabs
                .iter()
                .any(|receipt| receipt.support.end_ns() == boundary)
        );
        assert!(
            subslabs
                .iter()
                .any(|receipt| receipt.support.start_ns() == boundary)
        );
    }
    let _ = expect_dynamic_proposal;
    assert_eq!(finalized_parent.accepted_segments.len(), subslabs.len());
    assert_eq!(
        ending_consumer.inner.accepted_interval_count(),
        shadow.inner.accepted_interval_count() + 1,
        "thirty coupled slabs publish exactly one persistent parent interval",
    );
    // The historical one-child endpoint remains a fixture oracle for local
    // carrier checks above, but adaptive production installs the resolved
    // composed path and therefore must not be endpoint-byte-equal to it.
    assert!(subslabs.iter().all(|receipt| {
        receipt.validate().is_ok()
            && digest_bytes(&receipt.wb14_child_replay_bytes)
                == receipt.wb14_child_receipt_set_sha256
            && receipt.wb14_child_receipt_set_sha256 != Digest32::zero()
            && receipt.owner_join.wb14_child_receipt_set_sha256
                == receipt.wb14_child_receipt_set_sha256
    }));
    if let Some(second_swe) = second_lane_swe_m {
        let expected_active_lanes = if second_swe == 0.0 {
            BTreeSet::from([1])
        } else {
            BTreeSet::from([1, 2])
        };
        assert!(subslabs.iter().all(|receipt| {
            receipt
                .lane_receipts
                .keys()
                .copied()
                .collect::<BTreeSet<_>>()
                == expected_active_lanes
        }));
        assert_eq!(ending_stage3.len(), 2);
        if second_swe == 0.0 {
            assert_eq!(ending_stage3[&2], stage3_beginning_by_lane[&2]);
        } else {
            assert!(subslabs.iter().all(|receipt| {
                receipt.selected_upper_bound_s_bits == 60.0_f64.to_bits()
                    && receipt.lane_receipts[&1].lane_id == 1
                    && receipt.lane_receipts[&2].lane_id == 2
            }));
        }
    }
    assert!(
        subslabs[..subslabs.len() - 1]
            .iter()
            .all(|receipt| receipt.wb14_parent_receipt_set_sha256.is_none())
    );
    assert!(
        subslabs
            .last()
            .and_then(|receipt| receipt.wb14_parent_receipt_set_sha256)
            .is_some()
    );
    let mut poisoned = subslabs[0].clone();
    poisoned.selected_upper_bound_s_bits ^= 1;
    assert!(
        poisoned.validate().is_err(),
        "proposal substitution must reject"
    );
    let mut poisoned = subslabs[0].clone();
    poisoned.wb14_child_replay_bytes[0] ^= 1;
    assert!(
        poisoned.validate().is_err(),
        "replay payload substitution must reject"
    );
    let mut poisoned = subslabs[0].clone();
    poisoned.accepted_slab_sha256 = digest(99);
    assert!(
        poisoned.validate().is_err(),
        "accepted-slab substitution must reject"
    );
    for pair in subslabs.windows(2) {
        assert_eq!(pair[0].support.end_ns(), pair[1].support.start_ns());
    }
    if solid_reappearance {
        let beginning = stage3_beginning_by_lane
            .get(&1)
            .expect("solid-reappearance beginning lane");
        let ending = ending_stage3
            .get(&1)
            .expect("solid-reappearance ending lane");
        assert!(!crate::hydrology::stage3_has_represented_ice(beginning));
        assert!(crate::hydrology::stage3_has_represented_ice(ending));
        assert_eq!(ending.cumulative_snowfall_kg_m2.to_bits(), 50.0_f64.to_bits());
        assert_eq!(
            ending_clock.accepted_event_receipts().len(),
            beginning_clock.accepted_event_receipts().len() + subslabs.len() + 1,
            "one source-custody transition precedes the receipt-bearing support events",
        );
        // The legacy direct/open-only appendix below starts from a snow owner
        // placeholder and does not execute the canonical zero-duration
        // reappearance event. The adaptive assertions above are the real owner
        // path for this fixture.
        return;
    }
    if production_only {
        return;
    }
    let lane_receipt = executor
        .stack
        .last_lane_boundary_receipts()
        .and_then(|receipts| receipts.get(&1))
        .expect("mixed OFE final lane receipt");
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
    if !positive_covered_rain {
        assert!(executor.stack.take_staged_stage3().is_some());
    }

    if second_lane_swe_m.is_some() {
        return;
    }

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
    open_shadow.vegetation_state.0.configuration_sha256 = open_shadow
        .vegetation_configuration
        .configuration_sha256
        .clone();
    open_shadow.vegetation_state.0.state_sha256 = open_shadow.vegetation_state.0.canonical_sha256();
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
    open_shadow
        .inner
        .vegetation_configuration
        .configuration_sha256 = open_shadow
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
    open_shadow.inner.lse_state.configuration_sha256 = open_shadow
        .inner
        .lse_configuration
        .configuration_sha256
        .clone();
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
    let (open_parent_id, open_slab) = accepted_v11_slab(&open_clock_owners, 1_800_000_000_000);
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
            let forcing_receipt = Digest32::from_bytes(
                [20 + u8::try_from(index).expect("open-only tile ordinal"); 32],
            );
            let exposure = SealedOpenSnowExposureReceiptV1::try_new(
                support,
                destination.clone(),
                forcing_receipt,
                Digest32::from_bytes([30; 32]),
                covered_interval.lse_forcing.reference_wind_m_s,
                Digest32::from_bytes([31; 32]),
            )
            .expect("open-only exposure");
            let forcing = SealedOpenSnowTileForcingV1::try_new(SealedOpenSnowTileForcingInputsV1 {
                support,
                destination: destination.clone(),
                forcing_receipt_sha256: forcing_receipt,
                exposure,
                reference_temperature_k: covered_interval.lse_forcing.air_temperature_k,
                reference_specific_humidity_kg_kg: if equilibrium_fixture {
                    equilibrium_stage3_reference_specific_humidity(
                        covered_interval.lse_forcing.air_pressure_pa,
                    )
                } else {
                    covered_interval.lse_forcing.air_specific_humidity_kg_kg
                },
                air_pressure_pa: covered_interval.lse_forcing.air_pressure_pa,
                atmospheric_downward_longwave_w_m2: covered_interval
                    .lse_forcing
                    .atmospheric_downward_longwave_w_m2,
                direct_vis_w_m2: covered_interval.lse_forcing.direct_vis_w_m2,
                diffuse_vis_w_m2: covered_interval.lse_forcing.diffuse_vis_w_m2,
                direct_nir_w_m2: covered_interval.lse_forcing.direct_nir_w_m2,
                diffuse_nir_w_m2: covered_interval.lse_forcing.diffuse_nir_w_m2,
                rain_m: 0.0,
                snowfall_m: if solid_reappearance { 0.5 } else { 0.0 },
                precipitation_parcel_count: usize::from(solid_reappearance),
            })
            .expect("open-only forcing");
            (
                destination,
                SealedStage3TileBoundaryForcingV1::OpenSnow(forcing),
            )
        })
        .collect::<BTreeMap<_, _>>();
    let mut open_only_executor = crate::v11_vegetation_consumer::DirectV11VegetationExecutor {
        stack: DirectV11SnowCoveredRealConsumerStack::new(
            &open_shadow,
            DirectV11SnowCoveredStackInputs {
                interval: &covered_interval,
                stage3_inputs_by_lane: &stage3_inputs_by_lane,
                stage3_forcing_by_lane: &stage3_forcing_by_lane,
                snow_surface_forcing_by_destination: &open_only_forcing,
                stage3_beginning_by_lane: BTreeMap::from([(1, stage3_beginning.clone())]),
                pending_terminal_parcels: BTreeMap::new(),
                day_index: 0,
                interval_index: 0,
                finalize_wb14_parent_interval: true,
                wb14_coupled_child_binding: test_wb14_coupled_binding(),
            },
        ),
    };
    execute_direct_v11_segment(
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
    assert!(
        open_only_receipt
            .ordered_destinations
            .iter()
            .all(|receipt| {
                receipt.boundary_class
                    == crate::snow_stage3_terminal_handoff::Stage3TileBoundaryClassV1::OpenSnow
            })
    );
    let open_only_stage3 = open_only_executor
        .stack
        .take_staged_stage3()
        .expect("open-only staged Stage-3 owners");
    if solid_reappearance {
        let ending = open_only_stage3.get(&1).expect("reappeared open-only lane");
        assert!(crate::hydrology::stage3_has_represented_ice(ending));
        assert!(ending.cumulative_snowfall_kg_m2 > 0.0);
        assert!(!ending.layers.is_empty());
    }
    let open_only_ending = open_only_executor
        .stack
        .take_staged_ending()
        .expect("open-only staged owners");
    assert_eq!(
        open_only_ending.inner.lse_state.tiles, open_shadow.inner.lse_state.tiles,
        "open-only execution changes receipt chronology but not LSE tile physics",
    );
    let accepted_soil = &open_only_ending
        .inner
        .soil_thermal
        .v1()
        .expect("V1 soil resident")
        .ofes[0]
        .ordered_layers;
    let beginning_soil = &open_shadow
        .inner
        .soil_thermal
        .v1()
        .expect("V1 soil resident")
        .ofes[0]
        .ordered_layers;
    assert_ne!(
        accepted_soil[0].temperature_k.to_bits(),
        beginning_soil[0].temperature_k.to_bits(),
        "persistent snow installs its equal-and-opposite top-soil heat credit",
    );
    if solid_reappearance {
        assert!(
            accepted_soil
                .iter()
                .zip(beginning_soil)
                .any(|(ending, beginning)| ending != beginning),
            "reappearance support must advance the coupled soil owner",
        );
    } else {
        assert_eq!(
            &accepted_soil[1..],
            &beginning_soil[1..],
            "the OFE-ground lower boundary mutates only the first ordered soil node",
        );
    }

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
    changed_layers[0].cold_content_j_m2 += changed_layers[0].mass_swe_m * 1_000.0 * 2_100.0;
    let changed_stage3 = Wb11HydrologyKernel::initialize_stage3_persistent_state(1, changed_layers)
        .expect("changed Stage-3 beginning");
    let changed_stack = DirectV11SnowCoveredRealConsumerStack::new(
        &shadow,
        DirectV11SnowCoveredStackInputs {
            interval: &covered_interval,
            stage3_inputs_by_lane: &stage3_inputs_by_lane,
            stage3_forcing_by_lane: &stage3_forcing_by_lane,
            snow_surface_forcing_by_destination: &snow_surface_forcing_by_destination,
            stage3_beginning_by_lane: BTreeMap::from([(1, changed_stage3.clone())]),
            pending_terminal_parcels: BTreeMap::new(),
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
        changed_carrier_receipt, original_carrier_receipt,
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
                pending_terminal_parcels: BTreeMap::new(),
                day_index: 0,
                interval_index: 0,
                finalize_wb14_parent_interval: true,
                wb14_coupled_child_binding: test_wb14_coupled_binding(),
            },
        ),
    };
    assert!(
        execute_direct_v11_segment(&migrated.configuration, &parent, &slab, &mut poisoned).is_err()
    );
    assert!(poisoned.stack.take_staged_ending().is_none());
}

include!("v9_real_consumer_shadow_wb14_routing_tests.rs");
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
    shadow
        .lse_configuration
        .vegetation_configuration
        .configuration_sha256 = openwepp_land_surface_energy::Sha256Digest::try_new(
        shadow.vegetation_configuration.configuration_sha256.clone(),
    )
    .expect("open-only V10 LSE vegetation receipt");
    shadow.lse_configuration.configuration_sha256 = shadow
        .lse_configuration
        .canonical_sha256()
        .expect("open-only V2 LSE configuration");
    shadow.lse_state.0.configuration_sha256 = shadow.lse_configuration.configuration_sha256.clone();
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
        .configuration_sha256 =
        openwepp_land_surface_energy::Sha256Digest::try_new(v8_configuration.configuration_sha256)
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
