use super::*;

#[test]
fn terminal_output_classification_is_exact_lane_support_and_mass_independent() {
    let support = openwepp_coupled_time::TimeSupport::new(
        openwepp_coupled_time::ModelTimeNs::new(60_000_000_000),
        openwepp_coupled_time::ModelTimeNs::new(120_000_000_000),
    )
    .expect("terminal support");
    let following_support = openwepp_coupled_time::TimeSupport::new(
        openwepp_coupled_time::ModelTimeNs::new(120_000_000_000),
        openwepp_coupled_time::ModelTimeNs::new(180_000_000_000),
    )
    .expect("following support");
    let terminal_mass = 1.0_f64;
    let accepted_output_mass = f64::from_bits(terminal_mass.to_bits() - 1);

    assert_ne!(terminal_mass.to_bits(), accepted_output_mass.to_bits());
    assert!(terminal_lane_support_matches(7, support, 7, support));
    assert!(!terminal_lane_support_matches(7, support, 8, support));
    assert!(!terminal_lane_support_matches(
        7,
        support,
        7,
        following_support
    ));
}

fn profile_authority_fixture() -> (DirectRunFrame, DirectPublicationDayInput) {
    let identity = DirectRunIdentity::new(707, 808, 1, 1).expect("profile identity");
    let mut lane = DirectLaneConstructorInputs::from_topology_with_dynamic_day_inputs(0, 1)
        .expect("profile lane topology");
    lane.area_m2 = 10.0;
    lane.subsurface_layers = vec![DirectSubsurfaceLayerState::neutral()];
    let frame = DirectRunFrame::from_constructor_inputs(DirectRunConstructorInputs::new(
        identity,
        vec![lane],
    ))
    .expect("profile frame");
    let retained = &frame.lanes[0];
    let mut input = DirectPublicationDayInput::calendar_only(DirectPublicationCalendarDay {
        year: 2000,
        julian_day: 1,
        month: 1,
        day_of_month: 1,
        water_year: 2000,
    });
    let mut subsurface = DirectSubsurfaceComputeInputs::neutral();
    subsurface.layers = retained
        .subsurface_layers
        .iter()
        .cloned()
        .map(Into::into)
        .collect();
    input.subsurface_compute_inputs = Some(subsurface);
    input.hydrology_projection_inputs = Some(DirectHydrologyProjectionInputs {
        aggregate_storage_tolerance_m: 1.0e-9,
        profile_depth_m: Some(1.0),
        profile_porosity_cap_m: Some(1.0),
        profile_field_capacity_m: Some(0.5),
        profile_wilting_point_m: Some(0.0),
        ..DirectHydrologyProjectionInputs::zero()
    });
    (frame, input)
}

#[test]
fn parsed_soil_profile_authority_installs_exact_values_and_rejects_poisons() {
    let (frame, input) = profile_authority_fixture();
    let lane = &frame.lanes[0];
    let authority = Stage3PublicationProfileAuthorityV1::try_from_day_input(0, 0, lane, &input)
        .expect("exact parsed-soil profile authority");
    let mut day = DirectDayFrame::seed(frame.identity, 0, 0).expect("profile day");
    authority
        .install(&mut day, lane)
        .expect("install exact profile authority");
    let installed = day.hydrology_projection_inputs;
    assert_eq!(installed.profile_depth_m, Some(1.0));
    assert_eq!(installed.profile_porosity_cap_m, Some(1.0));
    assert_eq!(installed.profile_field_capacity_m, Some(0.5));
    assert_eq!(installed.profile_wilting_point_m, Some(0.0));
    assert_eq!(
        installed.aggregate_storage_tolerance_m.to_bits(),
        1.0e-9_f64.to_bits()
    );

    let mut omitted = input.clone();
    omitted.hydrology_projection_inputs = None;
    assert!(Stage3PublicationProfileAuthorityV1::try_from_day_input(0, 0, lane, &omitted).is_err());

    let mut partial = input.clone();
    partial
        .hydrology_projection_inputs
        .as_mut()
        .expect("profile inputs")
        .profile_field_capacity_m = None;
    assert!(Stage3PublicationProfileAuthorityV1::try_from_day_input(0, 0, lane, &partial).is_err());

    let mut value_substitution = authority.clone();
    value_substitution.profile_depth_m += 0.001;
    assert!(value_substitution.validate(lane).is_err());

    let mut tolerance_substitution = authority.clone();
    tolerance_substitution.aggregate_storage_tolerance_m =
        f64::from_bits(authority.aggregate_storage_tolerance_m.to_bits() + 1);
    assert!(tolerance_substitution.validate(lane).is_err());

    let mut invalid_tolerance = input.clone();
    invalid_tolerance
        .hydrology_projection_inputs
        .as_mut()
        .expect("projection inputs")
        .aggregate_storage_tolerance_m = f64::NAN;
    assert!(
        Stage3PublicationProfileAuthorityV1::try_from_day_input(0, 0, lane, &invalid_tolerance,)
            .is_err()
    );

    let mut lane_substitution = lane.clone();
    lane_substitution.subsurface_layers[0].porosity -= 0.001;
    assert!(authority.validate(&lane_substitution).is_err());

    let mut overwrite = DirectDayFrame::seed(frame.identity, 0, 0).expect("profile day");
    overwrite.hydrology_projection_inputs.profile_depth_m = Some(0.5);
    assert!(authority.install(&mut overwrite, lane).is_err());

    let mut tolerance_overwrite = DirectDayFrame::seed(frame.identity, 0, 0).expect("profile day");
    tolerance_overwrite
        .hydrology_projection_inputs
        .aggregate_storage_tolerance_m = 1.0e-10;
    assert!(authority.install(&mut tolerance_overwrite, lane).is_err());
}

fn accepted_headers() -> (DirectRunIdentity, Vec<AcceptedSupportHeader>) {
    let identity = DirectRunIdentity::new(707, 808, 1, 1).expect("support test identity");
    let parent_count = crate::snow_stage3_v11_attachment::STAGE3_V11_PARENT_SUPPORT_COUNT;
    let parent_ns = crate::snow_stage3_v11_attachment::STAGE3_V11_PARENT_SUPPORT_NS;
    let child_ns = parent_ns / 2;
    let accepted_support_count = parent_count * 2;
    let owners = (0..=accepted_support_count)
        .map(|index| digest_bytes(format!("owner-{index}").as_bytes()))
        .collect::<Vec<_>>();
    let mut headers = Vec::with_capacity(accepted_support_count);
    for interval_index in 0..parent_count {
        let parent_start = interval_index as u128 * parent_ns;
        let parent_transaction =
            digest_bytes(format!("accepted-parent-{interval_index}").as_bytes());
        for child_index in 0..2 {
            let accepted_index = interval_index * 2 + child_index;
            let support_start_ns = parent_start + child_index as u128 * child_ns;
            headers.push(AcceptedSupportHeader {
                day_index: 0,
                interval_index,
                parent_transaction_sha256: parent_transaction,
                support_start_ns,
                support_end_ns: support_start_ns + child_ns,
                accepted_slab_sha256: digest_bytes(
                    format!("accepted-slab-{accepted_index}").as_bytes(),
                ),
                beginning_complete_owner_set_sha256: owners[accepted_index],
                ending_complete_owner_set_sha256: owners[accepted_index + 1],
                receipt_sha256: digest_bytes(
                    format!("accepted-receipt-{accepted_index}").as_bytes(),
                ),
                run_identity: identity,
                accepted_infiltration_is_installed: true,
            });
        }
    }
    (identity, headers)
}

fn accepted_headers_with_event_bridge() -> (
    DirectRunIdentity,
    Vec<AcceptedSupportHeader>,
    Vec<AcceptedEventHandoffHeader>,
) {
    let (identity, mut headers) = accepted_headers();
    let preceding_support_index = 10;
    let following_support_index = preceding_support_index + 1;
    let parent_transaction_sha256 = headers[preceding_support_index].parent_transaction_sha256;
    let tick_ns = headers[preceding_support_index].support_end_ns;
    let first_event_ending_owner = digest_bytes(b"accepted-event-bridge-owner-0");
    let second_event_ending_owner = digest_bytes(b"accepted-event-bridge-owner-1");
    let events = vec![
        AcceptedEventHandoffHeader {
            receipt_id_sha256: digest_bytes(b"accepted-event-bridge-0"),
            parent_transaction_sha256,
            tick_ns,
            ordinal: 0,
            beginning_complete_owner_set_sha256: headers[preceding_support_index]
                .ending_complete_owner_set_sha256,
            ending_complete_owner_set_sha256: first_event_ending_owner,
            seal_is_valid: true,
        },
        AcceptedEventHandoffHeader {
            receipt_id_sha256: digest_bytes(b"accepted-event-bridge-1"),
            parent_transaction_sha256,
            tick_ns,
            ordinal: 1,
            beginning_complete_owner_set_sha256: first_event_ending_owner,
            ending_complete_owner_set_sha256: second_event_ending_owner,
            seal_is_valid: true,
        },
    ];
    headers[following_support_index].beginning_complete_owner_set_sha256 =
        second_event_ending_owner;
    (identity, headers, events)
}

fn sealed_day() -> Stage3AcceptedPublicationDayV1 {
    let identity = DirectRunIdentity::new(77, 88, 1, 1).expect("test run identity");
    Stage3AcceptedPublicationDayV1 {
        day_index: 0,
        beginning_complete_owner_set_sha256: digest_bytes(b"beginning-owner"),
        ending_complete_owner_set_sha256: digest_bytes(b"ending-owner"),
        ordered_support_receipt_set_sha256: digest_bytes(b"support-set"),
        lane_frames: vec![DirectDayFrame::seed(identity, 0, 0).expect("test day frame")],
        stage3_surface_temperature_c_by_lane: vec![None],
        laned_active_summary: None,
        receipt_sha256: digest_bytes(b"committed-publication-receipt"),
    }
}

#[test]
fn committed_capability_rejects_missing_incomplete_and_poisoned_frames() {
    let value = sealed_day();
    value
        .validate_for_install(0, 1, value.ending_complete_owner_set_sha256)
        .expect("complete sealed capability");

    let mut missing_lane = value.clone();
    missing_lane.lane_frames.clear();
    assert!(
        missing_lane
            .validate_for_install(0, 1, value.ending_complete_owner_set_sha256)
            .is_err()
    );

    let mut incomplete_day = value.clone();
    incomplete_day.lane_frames[0].day_index = 1;
    assert!(
        incomplete_day
            .validate_for_install(0, 1, value.ending_complete_owner_set_sha256)
            .is_err()
    );

    let mut poisoned_receipt = value.clone();
    poisoned_receipt.receipt_sha256 = Digest32::zero();
    assert!(
        poisoned_receipt
            .validate_for_install(0, 1, value.ending_complete_owner_set_sha256)
            .is_err()
    );
}

#[test]
fn frame_layer_substitution_cannot_replace_sealed_accepted_ending_layers() {
    let mut accepted_layer = DirectSubsurfaceLayerState::neutral();
    accepted_layer.depth_m = 0.2;
    accepted_layer.theta_m = 0.03;
    accepted_layer.residual_theta = 0.12;
    let accepted_layers = vec![accepted_layer];
    let mut staged_frame_layers = accepted_layers.clone();

    validate_accepted_ending_layer_identity(&accepted_layers, &staged_frame_layers)
        .expect("matching staged frame must retain the sealed ending layers");
    let accepted_soil_m =
        accepted_ending_soil_water_m(&accepted_layers).expect("sealed ending soil water");

    staged_frame_layers[0].theta_m += 0.01;
    assert!(
        validate_accepted_ending_layer_identity(&accepted_layers, &staged_frame_layers).is_err()
    );
    assert_eq!(
        accepted_ending_soil_water_m(&accepted_layers)
            .expect("sealed ending soil remains authoritative")
            .to_bits(),
        accepted_soil_m.to_bits()
    );
}

#[test]
fn adaptive_day_support_gate_accepts_exact_48_parent_groups() {
    let (identity, headers) = accepted_headers();
    validate_complete_support_headers(0, 0, DAY_NS, identity, &headers, &[])
        .expect("exact sealed adaptive supports across 48 parent groups");

    let interval_indices = headers
        .iter()
        .map(|header| header.interval_index)
        .collect::<BTreeSet<_>>();
    assert_eq!(interval_indices, (0..48).collect());
    assert_eq!(
        headers.first().map(|header| header.support_start_ns),
        Some(0)
    );
    assert_eq!(
        headers.last().map(|header| header.support_end_ns),
        Some(DAY_NS)
    );
}

#[test]
fn adaptive_day_support_gate_traverses_authenticated_event_owner_bridges() {
    let (identity, headers, events) = accepted_headers_with_event_bridge();
    validate_complete_support_headers(0, 0, DAY_NS, identity, &headers, &events)
        .expect("ordered zero-duration event handoffs bridge adjacent accepted supports");

    assert!(
        validate_complete_support_headers(0, 0, DAY_NS, identity, &headers, &[]).is_err(),
        "omitting the required event bridge must reject"
    );

    let mut owner_substitution = events.clone();
    owner_substitution[1].beginning_complete_owner_set_sha256 =
        digest_bytes(b"substituted-event-beginning");
    assert!(
        validate_complete_support_headers(0, 0, DAY_NS, identity, &headers, &owner_substitution,)
            .is_err()
    );

    let mut reordered = events.clone();
    reordered.swap(0, 1);
    assert!(
        validate_complete_support_headers(0, 0, DAY_NS, identity, &headers, &reordered).is_err()
    );

    let mut cross_parent = events.clone();
    cross_parent[0].parent_transaction_sha256 = digest_bytes(b"cross-parent-event");
    assert!(
        validate_complete_support_headers(0, 0, DAY_NS, identity, &headers, &cross_parent,)
            .is_err()
    );

    let mut tick_substitution = events.clone();
    tick_substitution[0].tick_ns +=
        crate::snow_stage3_v11_attachment::STAGE3_V11_ADAPTIVE_MINIMUM_SUPPORT_NS;
    assert!(
        validate_complete_support_headers(0, 0, DAY_NS, identity, &headers, &tick_substitution,)
            .is_err()
    );

    let mut ordinal_substitution = events;
    ordinal_substitution[1].ordinal = 3;
    assert!(
        validate_complete_support_headers(0, 0, DAY_NS, identity, &headers, &ordinal_substitution,)
            .is_err()
    );
}

#[test]
fn adaptive_day_support_gate_rejects_omission_duplicate_and_reorder() {
    let (identity, headers) = accepted_headers();

    let mut omitted = headers.clone();
    omitted.remove(35);
    assert!(validate_complete_support_headers(0, 0, DAY_NS, identity, &omitted, &[]).is_err());

    let mut duplicated = headers.clone();
    duplicated.insert(37, duplicated[36].clone());
    assert!(validate_complete_support_headers(0, 0, DAY_NS, identity, &duplicated, &[]).is_err());

    let mut reordered = headers;
    reordered.swap(42, 43);
    assert!(validate_complete_support_headers(0, 0, DAY_NS, identity, &reordered, &[]).is_err());
}

#[test]
fn adaptive_day_support_gate_rejects_parent_and_interval_substitution() {
    let (identity, headers) = accepted_headers();

    let mut cross_parent = headers.clone();
    cross_parent[47].parent_transaction_sha256 = cross_parent[48].parent_transaction_sha256;
    assert!(validate_complete_support_headers(0, 0, DAY_NS, identity, &cross_parent, &[]).is_err());

    let mut interval_substitution = headers;
    interval_substitution[47].interval_index = 24;
    assert!(
        validate_complete_support_headers(0, 0, DAY_NS, identity, &interval_substitution, &[],)
            .is_err()
    );
}

#[test]
fn adaptive_day_support_gate_rejects_subfloor_and_off_grid_microsteps() {
    let (identity, headers) = accepted_headers();
    let minimum_ns = crate::snow_stage3_v11_attachment::STAGE3_V11_ADAPTIVE_MINIMUM_SUPPORT_NS;

    let mut subfloor = headers.clone();
    subfloor[12].support_end_ns = subfloor[12].support_start_ns + minimum_ns - 1;
    assert!(validate_complete_support_headers(0, 0, DAY_NS, identity, &subfloor, &[]).is_err());

    let mut off_grid = headers;
    off_grid[12].support_end_ns += 1;
    assert!(validate_complete_support_headers(0, 0, DAY_NS, identity, &off_grid, &[]).is_err());
}

#[test]
fn accepted_terminal_liquid_requires_exact_sealed_receiver_custody() {
    let terminal_liquid_m = 0.012_345_678_9;
    validate_accepted_terminal_liquid_receiver_custody(terminal_liquid_m, terminal_liquid_m)
        .expect("cumulative terminal liquid is routed by the sealed receiver parcel");

    assert!(
        validate_accepted_terminal_liquid_receiver_custody(terminal_liquid_m, 0.0).is_err(),
        "omitting the sealed receiver parcel must reject"
    );
    assert!(
        validate_accepted_terminal_liquid_receiver_custody(
            terminal_liquid_m,
            terminal_liquid_m + 2.0 * ACCEPTED_CLOSURE_TOLERANCE_M,
        )
        .is_err(),
        "substituting the sealed receiver amount must reject"
    );
    assert!(
        validate_accepted_terminal_liquid_receiver_custody(f64::NAN, terminal_liquid_m).is_err(),
        "nonfinite cumulative owner state must reject"
    );
}

#[test]
fn compact_accepted_validation_is_bounded_and_trial_count_independent() {
    let (identity, headers) = accepted_headers();
    let started = std::time::Instant::now();
    for _ in 0..256 {
        validate_complete_support_headers(0, 0, DAY_NS, identity, &headers, &[])
            .expect("compact accepted header validation");
    }
    assert!(
        started.elapsed() < std::time::Duration::from_secs(5),
        "256 compact adaptive-day validations exceeded five seconds"
    );
    let source = include_str!("stage3_committed_publication.rs");
    assert!(!source.contains(concat!("UnifiedRealHydrology", "Candidate")));
    assert!(!source.contains(concat!("rejected_", "trial")));
}

#[test]
fn committed_producer_is_the_real_row_consumer_and_never_overwritten() {
    let executor = include_str!("03_executor.rs");
    assert!(executor.contains(".committed_snow_stage3_publication_day(day_index)?"));
    assert!(executor.contains(".validate_publication_exogenous_input(lane_index, day_input)?"));
    assert!(executor.contains("DirectPublicationDayRow::from_day_frame"));
    assert!(
        !executor.contains("day_frame.normalization.precipitation_m = day_input.precipitation_m")
    );
}

#[test]
fn accepted_precipitation_identity_rejects_substitution_without_conflating_temperature_means() {
    let mut day_input = DirectPublicationDayInput::calendar_only(DirectPublicationCalendarDay {
        year: 2000,
        julian_day: 1,
        month: 1,
        day_of_month: 1,
        water_year: 2000,
    });
    day_input.precipitation_m = 0.01;
    // CLIGEN's daily midpoint is intentionally not required to equal the
    // accepted microstep-weighted atmospheric temperature.
    day_input.effective_temperature_c = 7.0;
    validate_publication_exogenous_climate_identity(0, 0.01, &day_input)
        .expect("exact accepted precipitation identity");

    let mut precipitation_poison = day_input.clone();
    precipitation_poison.precipitation_m = f64::from_bits(0.01_f64.to_bits() + 1_000_000_000);
    let error = validate_publication_exogenous_climate_identity(0, 0.01, &precipitation_poison)
        .expect_err("accepted precipitation substitution must reject");
    assert!(matches!(
        error,
        DirectRuntimeError::DirectKernelGuardFailure { phase, detail }
            if phase == "stage3_committed_publication"
                && detail.contains("accepted support/day-input precipitation identity")
                && detail.contains("lane=0")
    ));

    day_input.effective_temperature_c = 7.006;
    validate_publication_exogenous_climate_identity(0, 0.01, &day_input)
        .expect("distinct accepted temporal mean is not a daily-input identity poison");
}

#[test]
fn accepted_publication_source_contains_no_forbidden_operand_fabrication() {
    let source = include_str!("stage3_committed_publication.rs");
    for forbidden in [
        concat!("DirectRunonCarryInputs", "::zero()"),
        concat!("normalization.effective_temperature_c", " = 0.0"),
        concat!("depression_storage_delta_handoff_m:", " 0.0"),
        concat!("layer_uptake_potential_m: vec!", "[0.0;"),
        concat!("layer_uptake_actual_m: vec!", "[0.0;"),
        concat!("[0.0;", " 24]"),
    ] {
        assert!(
            !source.contains(forbidden),
            "forbidden fabricated accepted operand: {forbidden}"
        );
    }
    assert!(!source.contains(concat!(".hydrology", "()")));
    assert!(!source.contains(concat!("UnifiedRealHydrology", "Candidate")));
}

fn accepted_wb14_partition_fixture() -> AcceptedLaneDay {
    let mut accepted = AcceptedLaneDay {
        ingress_m: 0.014,
        local_liquid_m: 0.010,
        runon_m: 0.004,
        infiltration_m: 0.008,
        retained_surface_liquid_m: 0.001,
        runoff_m: 0.005,
        ..AcceptedLaneDay::default()
    };
    accepted.hourly_runoff_m[2] = 0.002;
    accepted.hourly_runoff_m[9] = 0.003;
    accepted
}

fn accepted_wb14_partition_day() -> DirectDayFrame {
    let identity = DirectRunIdentity::new(91, 92, 1, 1).expect("WB14 identity");
    let mut day = DirectDayFrame::seed(identity, 0, 0).expect("WB14 day");
    day.runon_carry = DirectRunonCarryState {
        runon_input_m: 0.004,
        subsurface_carry_m: 0.007,
    };
    day.liquid_input_shadow_projection = Some(DirectLiquidInputShadowProjection {
        lane_index: 0,
        day_index: 0,
        liquid_input_m: 0.010,
    });
    day.runon_carry_shadow_projection = Some(DirectRunonCarryShadowProjection {
        lane_index: 0,
        day_index: 0,
        runon_input_m: 0.004,
        subsurface_carry_m: 0.007,
    });
    day.infiltration_depression_shadow_projection =
        Some(DirectInfiltrationDepressionShadowProjection {
            lane_index: 0,
            day_index: 0,
            cumulative_infiltration_m: 0.008,
            depression_storage_delta_m: 0.001,
        });
    day.saturation_addback_shadow_projection = Some(DirectSaturationAddbackShadowProjection {
        lane_index: 0,
        day_index: 0,
        surface_saturation_runoff_m: 0.0,
    });
    day.subsurface_compute_shadow_projection = Some(DirectSubsurfaceComputeShadowProjection {
        lane_index: 0,
        day_index: 0,
        soil_water_before_m: 0.0,
        soil_water_after_m: 0.0,
        lateral_flow_m: 0.0,
        tile_drainage_m: 0.0,
        subsurface_loss_m: 0.0,
        lateral_target_m: 0.0,
        drainage_target_m: 0.0,
        lateral_capacity_m: 0.0,
        hourly_lateral_carry_m: [0.0; 24],
        hourly_saturation_carry_m: [0.0; 24],
        layer_state_after: Vec::new(),
        lateral_layer_withdrawal_m: Vec::new(),
    });
    day
}

#[test]
fn accepted_wb14_partition_preserves_exact_bins_and_excludes_subsurface_carry() {
    let accepted = accepted_wb14_partition_fixture();
    let mut day = accepted_wb14_partition_day();

    install_accepted_wb14_partition_inputs(&mut day, &accepted)
        .expect("sealed accepted WB14 partition source");

    assert_eq!(day.wb14_hourly_excess_m, accepted.hourly_runoff_m);
    assert_eq!(
        day.runoff_partition_inputs.runon_input_m.to_bits(),
        accepted.runon_m.to_bits(),
        "the distinct subsurface carry must not become surface runon",
    );
    assert_eq!(
        day.runon_carry.subsurface_carry_m.to_bits(),
        0.007_f64.to_bits(),
        "the carry remains available to subsurface/storage accounting",
    );
    day.run_r4a_runoff_partition_span()
        .expect("accepted partition");
    day.run_r7d6_peak_runoff_span()
        .expect("accepted WB14 peak timing");
    assert!((day.runoff_partition.q_runoff_m - accepted.runoff_m).abs() < 1.0e-15);
    assert_eq!(day.peak_runoff.peak_hour_index, Some(9));
    assert!((day.peak_runoff.peak_runoff_rate_m_s - 0.003 / 3_600.0).abs() < 1.0e-18);
}

#[test]
fn accepted_wb14_partition_rejects_subsurface_carry_as_a_surface_bin() {
    let mut accepted = accepted_wb14_partition_fixture();
    accepted.hourly_runoff_m[7] = 0.007;
    let mut day = accepted_wb14_partition_day();
    let before_inputs = day.runoff_partition_inputs;
    let before_hourly = day.wb14_hourly_excess_m;

    let error = install_accepted_wb14_partition_inputs(&mut day, &accepted)
        .expect_err("subsurface carry cannot be appended to accepted WB14 runoff");

    assert!(
        error
            .to_string()
            .contains("accepted WB14 hourly/daily runoff closure")
    );
    assert_eq!(day.runoff_partition_inputs, before_inputs);
    assert_eq!(day.wb14_hourly_excess_m, before_hourly);
}

#[test]
fn accepted_wb14_partition_poison_rolls_back_without_mutation() {
    let mut accepted = accepted_wb14_partition_fixture();
    accepted.hourly_runoff_m[2] = -0.002;
    let mut day = accepted_wb14_partition_day();
    day.runoff_partition_inputs.liquid_input_m = 0.123;
    day.wb14_hourly_excess_m[23] = 0.456;
    let before_inputs = day.runoff_partition_inputs;
    let before_hourly = day.wb14_hourly_excess_m;

    install_accepted_wb14_partition_inputs(&mut day, &accepted)
        .expect_err("negative accepted timing must reject");

    assert_eq!(day.runoff_partition_inputs, before_inputs);
    assert_eq!(day.wb14_hourly_excess_m, before_hourly);
}

fn replace_test_r4a_partition_scalar(day: &mut DirectDayFrame, partition_runoff_m: f64) {
    let mut state = day.runoff_partition;
    state.partition_runoff_m = partition_runoff_m;
    state.q_runoff_m = partition_runoff_m + state.surface_saturation_runoff_m;
    state.closure_residual_m = -partition_runoff_m;
    let downstream = DirectRunoffDownstreamOperands::from(state);
    day.runoff_partition = state;
    day.runoff_downstream_operands = downstream;
    day.runoff_shadow_projection = Some(DirectRunoffShadowProjection {
        lane_index: day.lane_index,
        day_index: day.day_index,
        liquid_input_m: downstream.liquid_input_m,
        runon_input_m: downstream.runon_input_m,
        cumulative_infiltration_m: downstream.cumulative_infiltration_m,
        depression_storage_delta_m: downstream.depression_storage_delta_m,
        surface_saturation_runoff_m: downstream.surface_saturation_runoff_m,
        partition_runoff_m: downstream.partition_runoff_m,
        q_runoff_m: downstream.q_runoff_m,
        closure_residual_m: downstream.closure_residual_m,
    });
    day.water.runoff_m = state.q_runoff_m;
}

fn accepted_zero_wb14_partition_day() -> (AcceptedLaneDay, DirectDayFrame) {
    let accepted = AcceptedLaneDay::default();
    let mut day = accepted_wb14_partition_day();
    install_accepted_wb14_partition_inputs(&mut day, &accepted).expect("zero accepted WB14");
    day.run_r4a_runoff_partition_span()
        .expect("zero accepted partition");
    (accepted, day)
}

#[test]
fn accepted_wb14_scalar_reconciliation_accepts_below_and_at_contract_bound() {
    let tolerance_m = ACCEPTED_WB14_INTERVAL_CLOSURE_TOLERANCE_M * 24.0;
    for residual_m in [tolerance_m * 0.5, tolerance_m] {
        let (accepted, mut day) = accepted_zero_wb14_partition_day();
        replace_test_r4a_partition_scalar(&mut day, residual_m);

        reconcile_accepted_wb14_partition_scalar(&mut day, &accepted)
            .expect("TOL-WATBAL-009 boundary is accepted");

        assert_eq!(day.runoff_partition.partition_runoff_m.to_bits(), 0);
        assert_eq!(day.runoff_partition.q_runoff_m.to_bits(), 0);
        assert_eq!(
            day.runoff_shadow_projection
                .expect("reconciled R4A shadow")
                .q_runoff_m
                .to_bits(),
            0,
        );
    }
}

#[test]
fn accepted_wb14_scalar_reconciliation_preserves_subthreshold_positive_source() {
    let mut accepted = AcceptedLaneDay {
        ingress_m: 1.0e-15,
        local_liquid_m: 1.0e-15,
        runoff_m: 1.0e-15,
        ..AcceptedLaneDay::default()
    };
    accepted.hourly_runoff_m[17] = 1.0e-15;
    let mut day = accepted_wb14_partition_day();
    install_accepted_wb14_partition_inputs(&mut day, &accepted)
        .expect("positive subthreshold accepted WB14");
    day.run_r4a_runoff_partition_span()
        .expect("positive subthreshold partition");
    replace_test_r4a_partition_scalar(&mut day, 0.0);

    reconcile_accepted_wb14_partition_scalar(&mut day, &accepted)
        .expect("positive hourly source is preserved");

    assert_eq!(
        day.runoff_partition.partition_runoff_m.to_bits(),
        1.0e-15_f64.to_bits(),
    );
    assert_eq!(
        day.wb14_hourly_excess_m[17].to_bits(),
        1.0e-15_f64.to_bits()
    );
}

#[test]
fn accepted_wb14_scalar_material_mismatch_fails_without_mutation() {
    let tolerance_m = ACCEPTED_WB14_INTERVAL_CLOSURE_TOLERANCE_M * 24.0;
    let above_tolerance_m = f64::from_bits(tolerance_m.to_bits() + 1);
    let (accepted, mut day) = accepted_zero_wb14_partition_day();
    replace_test_r4a_partition_scalar(&mut day, above_tolerance_m);
    let before_state = day.runoff_partition;
    let before_downstream = day.runoff_downstream_operands;
    let before_shadow = day.runoff_shadow_projection;
    let before_water = day.water.clone();
    let before_hourly = day.wb14_hourly_excess_m;

    let error = reconcile_accepted_wb14_partition_scalar(&mut day, &accepted)
        .expect_err("material R4A/WB14 mismatch must reject");

    assert!(matches!(
        error,
        DirectRuntimeError::DirectClosureToleranceExceeded {
            field: "stage3_publication.accepted_wb14_partition_runoff_m"
        }
    ));
    assert_eq!(day.runoff_partition, before_state);
    assert_eq!(day.runoff_downstream_operands, before_downstream);
    assert_eq!(day.runoff_shadow_projection, before_shadow);
    assert_eq!(day.water, before_water);
    assert_eq!(day.wb14_hourly_excess_m, before_hourly);
}

include!("stage3_committed_publication_tests_tail.rs");
