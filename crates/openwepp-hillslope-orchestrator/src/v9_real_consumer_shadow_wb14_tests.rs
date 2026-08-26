// Contract binding: SC-SURFACELIQUID-001@8 INV-SURFACELIQUID-012..014.
#[test]
#[allow(clippy::too_many_lines)]
fn mixed_open_covered_stack_executes_complete_ofe_ground_boundary() {
    exercise_complete_wb14_cadence(0.005, 8.0, true, None, false, None, false, false);
}

#[test]
#[allow(clippy::too_many_lines)]
fn two_900_second_complete_owner_children_publish_one_parent() {
    exercise_complete_wb14_cadence(0.02, 8.0, false, None, false, None, false, false);
}

#[test]
#[allow(clippy::too_many_lines)]
fn one_1800_second_child_matches_complete_historical_candidate() {
    exercise_complete_wb14_cadence(0.08, 8.0, false, None, false, None, false, false);
}

#[test]
#[allow(clippy::too_many_lines)]
fn coupled_hard_boundary_truncates_selected_900_second_child() {
    exercise_complete_wb14_cadence(0.02, 8.0, false, Some(60_000_000_000), false, None, false, false);
}

#[test]
#[allow(clippy::too_many_lines)]
fn latest_accepted_stage3_state_changes_next_wb14_proposal() {
    exercise_complete_wb14_cadence(0.010_000_001, 0.0, false, Some(60_000_000_000), true, None, false, false);
}

#[test]
#[allow(clippy::too_many_lines)]
fn resolved_snow_and_snow_free_lanes_publish_one_atomic_parent() {
    exercise_complete_wb14_cadence(0.08, 8.0, false, None, false, Some(0.0), false, false);
}

#[test]
#[allow(clippy::too_many_lines)]
fn two_resolved_snow_lanes_choose_common_earliest_cadence() {
    exercise_complete_wb14_cadence(0.08, 8.0, true, None, false, Some(0.005), false, false);
}

#[test]
#[allow(clippy::too_many_lines)]
fn interior_terminal_event_runs_covered_event_and_snow_free_remainder() {
    exercise_complete_wb14_cadence(0.000_6, 0.0, false, None, false, None, true, false);
}

#[test]
fn interior_terminal_event_capture_reproduces_below_carrier_domain() {
    exercise_complete_wb14_cadence(0.000_6, 0.0, false, None, false, None, true, true);
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
) {
    let positive_covered_rain = runtime_swe_m.to_bits() == 0.005_f64.to_bits()
        && include_child_17
        && second_lane_swe_m.is_none();
    let (shadow, fixture) = if second_lane_swe_m.is_some() {
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
    let support_end_ns = 1_800_000_000_000;
    let support_seconds = 1_800.0;
    let base_interval = day_input(&fixture).intervals.remove(0);
    let interval = segment_interval(&base_interval, support_end_ns, 41, 0.0);
    let mut interval = interval;
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
    interval.lse_forcing.snow_present_at_beginning = true;
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
    stage3_inputs.runtime_depth_m = runtime_swe_m * 10.0;
    stage3_inputs.runtime_density_kg_m3 = 100.0;
    stage3_inputs.snow_layers[0].mass_swe_m = runtime_swe_m;
    stage3_inputs.snow_layers[0].thickness_m = runtime_swe_m * 10.0;
    stage3_inputs.snow_layers[0].density_kg_m3 = 100.0;
    let stage3_cold_delta_k = initial_cold_delta_k;
    stage3_inputs.snow_layers[0].temperature_c = -stage3_cold_delta_k;
    stage3_inputs.snow_layers[0].cold_content_j_m2 =
        runtime_swe_m * 1_000.0 * 2_100.0 * stage3_cold_delta_k;
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
    let stage3_forcing = DirectSnowStage3SupportInput {
        forcing: stage3_hourly,
        duration_seconds: support_seconds,
    };
    let mut stage3_inputs_by_lane = BTreeMap::from([(1, stage3_inputs.clone())]);
    let mut stage3_forcing_by_lane = BTreeMap::from([(1, stage3_forcing)]);
    let carrier_forcing_by_lane = BTreeMap::from([(1, child2c_carrier_forcing())]);
    let mut stage3_beginning_by_lane = BTreeMap::from([(1, stage3_beginning.clone())]);
    let preliminary_stage3_inputs_by_lane = if terminal_event {
        let mut value = stage3_inputs.clone();
        value.runtime_swe_m = 0.005;
        value.runtime_depth_m = 0.05;
        value.snow_layers[0].mass_swe_m = 0.005;
        value.snow_layers[0].thickness_m = 0.05;
        BTreeMap::from([(1, value)])
    } else {
        stage3_inputs_by_lane.clone()
    };
    let preliminary_stage3_beginning_by_lane = if terminal_event {
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
        second_inputs.runtime_depth_m = second_swe * 10.0;
        second_inputs.runtime_density_kg_m3 = if second_swe == 0.0 { 0.0 } else { 100.0 };
        second_inputs.snow_layers = if second_swe == 0.0 {
            Vec::new()
        } else {
            let mut layer = second_inputs.snow_layers[0];
            layer.mass_swe_m = second_swe;
            layer.thickness_m = second_swe * 10.0;
            layer.density_kg_m3 = 100.0;
            layer.cold_content_j_m2 = second_swe * 1_000.0 * 2_100.0 * 8.0;
            vec![layer]
        };
        let second_state = Wb11HydrologyKernel::initialize_stage3_persistent_state(
            2,
            second_inputs.snow_layers.clone(),
        )
        .expect("second persistent Stage-3 beginning");
        stage3_inputs_by_lane.insert(2, second_inputs);
        stage3_forcing_by_lane.insert(2, stage3_forcing);
        stage3_beginning_by_lane.insert(2, second_state);
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
    let missing_error = execute_direct_v11_segment(
        &migrated.configuration,
        &parent,
        &slab,
        &mut missing_open_executor,
    )
    .expect_err("mixed OFE without its open-snow boundary must reject");
    if second_lane_swe_m.is_none() && !positive_covered_rain && !terminal_event {
        assert!(matches!(
            missing_error,
            V11ExecutionError::Executor(DirectV11RealConsumerError::Identity(
                "covered Stage-3 lane is missing a snow-surface contribution"
            ))
        ));
    }
    assert!(missing_open_executor.stack.take_staged_stage3().is_none());
    assert!(missing_open_executor.stack.take_staged_ending().is_none());
    let open_record = shadow
        .inner
        .surface_configuration
        .records
        .iter()
        .find(|record| !covered_tiles.contains(&record.key.tile_id))
        .expect("mixed fixture open tile");
    let support = TimeSupport::new(ModelTimeNs::new(0), ModelTimeNs::new(support_end_ns))
        .expect("open-snow support");
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
        reference_specific_humidity_kg_kg: covered_interval.lse_forcing.air_specific_humidity_kg_kg,
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
        let (parcel_mass, _) = reconstruct_precipitation_mass_and_advected_heat(precipitation_set)
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
                validate_precipitation_producer_manifest(&resealed_omission, &producer_manifest),
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
        assert!(crate::snow_stage3_v11_attachment::validate_precipitation_phase_parcel_set(
            &precipitation_seal_poison
        )
        .is_err());
        let snow_soil_receipt = executor
            .stack
            .last_snow_soil_heat_receipts()
            .and_then(|receipts| receipts.get(&1))
            .expect("installed rainy snow-soil heat receipt");
        crate::snow_stage3_v11_attachment::validate_snow_soil_heat_receipt(snow_soil_receipt)
            .expect("installed rainy snow-soil receipt validates");
        let mut snow_soil_poison = snow_soil_receipt.clone();
        snow_soil_poison.soil_candidate_heat_j_m2_ofe_ground += 1.0;
        assert!(crate::snow_stage3_v11_attachment::validate_snow_soil_heat_receipt(
            &snow_soil_poison
        )
        .is_err());
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
                        PreparedStage3V11SupportIdentityV1::new(
                            record.key.ofe_id.as_str().to_owned(),
                            record.key.tile_id.as_str().to_owned(),
                            "a".repeat(64),
                            Digest32::from_bytes([13; 32]),
                            Vec::new(),
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
        identities,
    )
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
        .expect("active Stage-3 cadence");
    let rollback_parent = parent.clone();
    let rollback_consumer = shadow.clone();
    let rollback_clock = beginning_clock.clone();
    let rollback_stage3 = stage3_beginning.clone();
    let mut injections = vec![
        Stage3V11FailureInjection::OutcomeLedgerBuilt(1),
        Stage3V11FailureInjection::PrecipitationReceiptRejected(1),
        Stage3V11FailureInjection::SnowSoilHeatReceiptRejected(1),
        Stage3V11FailureInjection::SubslabAccepted(1),
    ];
    if include_child_17 {
        injections.push(Stage3V11FailureInjection::SubslabAccepted(17));
    }
    injections.push(Stage3V11FailureInjection::FinalOwnerJoinCompleted);
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
    if positive_covered_rain {
        return;
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
        let (result, evidence) = crate::snow_stage3_v11_attachment::execute_covered_real_v11_parent_capture(
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
        assert!(matches!(
            no_evidence_result,
            Err(DirectSnowStage3V11AttachmentError::Stage3(
                DirectSnowStage3EvaluationError::TerminalNumerics(
                    crate::SnowTerminalNumericsFailure::BelowCarrierDomain
                )
            ))
        ));
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
        assert!(pair.rejected);
        assert_eq!(pair.components[3].0.to_bits(), 0x4094_9afb_c192_8120);
        assert_eq!(pair.components[3].1.to_bits(), 0x4094_2e21_8363_bae1);
        assert_eq!(pair.components[3].2.to_bits(), 0xc03b_368f_8bb1_8fc0);
        let trials = &evidence.selected_trials[evidence.selected_trials.len() - 3..];
        // Typed Stage-3/hydrology supply proof: terminal liquid is absent from
        // every selected trial's live external-liquid operand.
        assert!(trials.iter().all(|trial| {
            trial.ledger.external_liquid_kg_m2.to_bits() == 0.0_f64.to_bits()
        }));
        let admission = evidence.admissions.last().expect("floor admission");
        assert_eq!(admission.0.to_bits(), 0.9375_f64.to_bits());
        assert_eq!(admission.1.to_bits(), 0.46875_f64.to_bits());
        assert_eq!(admission.2.to_bits(), 0.6_f64.to_bits());
        assert_eq!(admission.3, crate::SnowTerminalNumericsFailure::BelowCarrierDomain);
        assert_eq!(admission.4, admission.5);
        assert!(!evidence.provider_calls.is_empty());
        assert_eq!(evidence.provider_calls.len(), evidence.coupling_iterations.len());
        assert!(evidence.provider_calls.iter().enumerate().all(|(ordinal, call)|
            call.ordinal == ordinal as u64
                && matches!(call.outcome, crate::hydrology::CapturedProviderOutcome::Success(_))));
        assert!(evidence.coupling_iterations.iter().all(|iteration| {
            let request = &iteration.hook.request;
            let comparison_shape = if request.coupling_iteration == 0 {
                iteration.hook.comparisons.is_none() && request.ending_snow_hint.is_none()
            } else {
                iteration.hook.comparisons.is_some() && request.ending_snow_hint.is_some()
            };
            comparison_shape
                && evidence.provider_calls.iter().filter(|call| {
                    call.request.support == request.support
                        && call.request.role == request.role
                        && call.request.attempt_ordinal == request.attempt_ordinal
                        && call.request.coupling_iteration == request.coupling_iteration
                        && call.request.lane_id == request.lane_id
                        && call.request.beginning_joint.receipt_sha256()
                            == request.beginning_joint.receipt_sha256()
                }).count() == 1
        }));
        assert!(evidence.coupling_selections.iter().all(|selection| {
            selection.reason == crate::hydrology::TerminalCouplingSelectionReason::FourComponentConvergenceBreak
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
            assert!(second_iteration
                .hook
                .comparisons
                .expect("second-iteration comparisons")
                .iter()
                .all(|comparison| comparison.2.to_bits() == 0.0_f64.to_bits()));
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
            assert_eq!(first.transition.beginning_joint, second.transition.beginning_joint);
            assert_eq!(first.transition.ending_joint, second.transition.ending_joint);
            assert_eq!(
                first.transition.probe_child_identity,
                second.transition.probe_child_identity
            );
            assert_eq!(first.precipitation_sets, second.precipitation_sets);
            assert_eq!(first.complete_lower_boundaries, second.complete_lower_boundaries);
            assert_eq!(first.carrier_source_receipts, second.carrier_source_receipts);
            assert_eq!(first.covered_lse_states, second.covered_lse_states);
            assert_eq!(first.soil_candidate, second.soil_candidate);
            assert_eq!(first.soil_top_boundary_credit, second.soil_top_boundary_credit);
            assert_eq!(
                first.wb14_child_receipt_set_sha256,
                second.wb14_child_receipt_set_sha256
            );
            assert_eq!(first.wb14_child_replay_bytes, second.wb14_child_replay_bytes);
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
                    == pair.components.iter().fold(0.0_f64, |maximum, component| maximum.max(component.4)).to_bits()
                && pair.rejected
                    == (pair.maximum_scaled > 1.0 && pair.components[0].1 > 0.0)
        }));
        assert_eq!(pair.proposed_next_duration_s.to_bits(), admission.0.to_bits());
        assert!(evidence.provider_calls.iter().all(|call| {
            call.request.support.end_ns().get() - call.request.support.start_ns().get()
                >= 600_000_000
        }));
        // Typed WB14 authorization/credit proof.
        assert!(evidence.provider_calls.iter().all(|call| match &call.outcome {
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
        }));
        // Typed input surface-liquid ingress proof, independent of WB14 receipts.
        assert!(evidence.provider_calls.iter().all(|call| match &call.outcome {
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
        }));
        macro_rules! rejects_poison {
            ($label:literal, $mutate:expr) => {{
                let mut poisoned = evidence.clone();
                ($mutate)(&mut poisoned);
                assert!(poisoned.validate().is_err(), $label);
            }};
        }
        rejects_poison!("missing provider", |poisoned: &mut crate::hydrology::CaptureState| {
            poisoned.provider_calls.remove(0);
        });
        rejects_poison!("duplicate provider", |poisoned: &mut crate::hydrology::CaptureState| {
            poisoned.provider_calls.insert(0, poisoned.provider_calls[0].clone());
        });
        rejects_poison!("reordered provider", |poisoned: &mut crate::hydrology::CaptureState| {
            poisoned.provider_calls.swap(0, 1);
        });
        rejects_poison!("substituted provider key", |poisoned: &mut crate::hydrology::CaptureState| {
            poisoned.provider_calls[0].request.attempt_ordinal ^= 1;
        });
        rejects_poison!("missing coupling iteration", |poisoned: &mut crate::hydrology::CaptureState| {
            poisoned.coupling_iterations.remove(0);
        });
        rejects_poison!("duplicate coupling iteration", |poisoned: &mut crate::hydrology::CaptureState| {
            poisoned.coupling_iterations.insert(0, poisoned.coupling_iterations[0].clone());
        });
        rejects_poison!("reordered coupling iteration", |poisoned: &mut crate::hydrology::CaptureState| {
            poisoned.coupling_iterations.swap(0, 1);
        });
        rejects_poison!("substituted coupling key", |poisoned: &mut crate::hydrology::CaptureState| {
            poisoned.coupling_iterations[0].hook.request.attempt_ordinal ^= 1;
        });
        rejects_poison!("coupling comparison", |poisoned: &mut crate::hydrology::CaptureState| {
            let iteration = poisoned
                .coupling_iterations
                .iter_mut()
                .find(|iteration| iteration.hook.comparisons.is_some())
                .expect("comparison iteration");
            iteration.hook.comparisons.as_mut().expect("comparisons")[0].2 += 1.0;
        });
        rejects_poison!("missing coupling selection", |poisoned: &mut crate::hydrology::CaptureState| {
            poisoned.coupling_selections.remove(0);
        });
        rejects_poison!("duplicate coupling selection", |poisoned: &mut crate::hydrology::CaptureState| {
            poisoned.coupling_selections.insert(0, poisoned.coupling_selections[0].clone());
        });
        rejects_poison!("reordered coupling selection", |poisoned: &mut crate::hydrology::CaptureState| {
            poisoned.coupling_selections.swap(0, 1);
        });
        rejects_poison!("substituted coupling selection", |poisoned: &mut crate::hydrology::CaptureState| {
            poisoned.coupling_selections[0].request.attempt_ordinal ^= 1;
        });
        rejects_poison!("selected convergence reason", |poisoned: &mut crate::hydrology::CaptureState| {
            poisoned.coupling_selections[0].reason =
                crate::hydrology::TerminalCouplingSelectionReason::IterationLoopExhausted;
        });
        rejects_poison!("selected live convergence", |poisoned: &mut crate::hydrology::CaptureState| {
            let request = poisoned.coupling_selections[0].request.clone();
            poisoned
                .coupling_iterations
                .iter_mut()
                .find(|iteration| {
                    iteration.hook.request.support == request.support
                        && iteration.hook.request.role == request.role
                        && iteration.hook.request.attempt_ordinal == request.attempt_ordinal
                        && iteration.hook.request.coupling_iteration == request.coupling_iteration
                })
                .expect("selected iteration")
                .hook
                .converged = false;
        });
        rejects_poison!("selected trial order", |poisoned: &mut crate::hydrology::CaptureState| {
            poisoned.selected_trials.swap(0, 1);
        });
        rejects_poison!("missing selected trial", |poisoned: &mut crate::hydrology::CaptureState| {
            poisoned.selected_trials.remove(0);
        });
        rejects_poison!("duplicate selected trial", |poisoned: &mut crate::hydrology::CaptureState| {
            poisoned.selected_trials.insert(0, poisoned.selected_trials[0].clone());
        });
        rejects_poison!("substituted selected trial", |poisoned: &mut crate::hydrology::CaptureState| {
            poisoned.selected_trials[0].duration_s += 1.0;
        });
        rejects_poison!("selected joint join", |poisoned: &mut crate::hydrology::CaptureState| {
            poisoned.selected_trials[2].beginning_joint = None;
        });
        rejects_poison!("maximum-scaled conjunct", |poisoned: &mut crate::hydrology::CaptureState| {
            poisoned.pairs.last_mut().expect("pair").maximum_scaled = 1.0;
        });
        rejects_poison!("refined-ice conjunct", |poisoned: &mut crate::hydrology::CaptureState| {
            poisoned.pairs.last_mut().expect("pair").components[0].1 = 0.0;
        });
        rejects_poison!("decision delta", |poisoned: &mut crate::hydrology::CaptureState| {
            poisoned.pairs[0].components[0].2 += 1.0;
        });
        rejects_poison!("decision denominator", |poisoned: &mut crate::hydrology::CaptureState| {
            poisoned.pairs[0].components[0].3 += 1.0;
        });
        rejects_poison!("decision scaled", |poisoned: &mut crate::hydrology::CaptureState| {
            poisoned.pairs[0].components[0].4 += 1.0;
        });
        rejects_poison!("missing pair", |poisoned: &mut crate::hydrology::CaptureState| {
            poisoned.pairs.remove(0);
        });
        rejects_poison!("duplicate pair", |poisoned: &mut crate::hydrology::CaptureState| {
            poisoned.pairs.insert(0, poisoned.pairs[0].clone());
        });
        rejects_poison!("reordered pair", |poisoned: &mut crate::hydrology::CaptureState| {
            poisoned.pairs.swap(0, 1);
        });
        rejects_poison!("substituted pair", |poisoned: &mut crate::hydrology::CaptureState| {
            poisoned.pairs[0].duration_s += 1.0;
        });
        rejects_poison!("missing floor", |poisoned: &mut crate::hydrology::CaptureState| {
            poisoned.admissions.clear();
        });
        rejects_poison!("duplicate floor", |poisoned: &mut crate::hydrology::CaptureState| {
            poisoned.admissions.push(poisoned.admissions[0].clone());
        });
        rejects_poison!("floor outcome", |poisoned: &mut crate::hydrology::CaptureState| {
            poisoned.admissions[0].3 = crate::SnowTerminalNumericsFailure::DomainOrNonFinite;
        });
        let validated = evidence.validate().expect("raw terminal evidence validates");
        assert_eq!(validated.call_count_through_final_pair, validated.call_count_at_floor);
        assert_eq!(validated.pairs.last().unwrap().trials.len(), 3);
        assert!(validated.pairs.last().unwrap().decision.rejected);
        assert_eq!(validated.floor.outcome, crate::SnowTerminalNumericsFailure::BelowCarrierDomain);
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
    ) =
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
            None,
        )
        .expect("synchronized covered parent cadence");
    if terminal_event {
        assert_eq!(event_groups.len(), 1);
        assert!(!terminal_parcels.is_empty());
        assert!(terminal_parcels.iter().all(|parcel| {
            parcel.posture == DirectSnowStage3V11TerminalParcelPosture::ProducedUnconsumed
        }));
        assert_eq!(stage3_lane_lifecycle(&ending_stage3[&1], 0.0), Stage3LaneLifecycleV1::SnowFree);
        return;
    }
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
        assert!(
            subslabs
                .iter()
                .skip(1)
                .any(|receipt| { receipt.selected_upper_bound_s_bits == 60.0_f64.to_bits() }),
            "latest accepted Stage-3 state must change the next proposal"
        );
    }
    assert_eq!(finalized_parent.accepted_segments.len(), subslabs.len());
    assert_eq!(
        ending_consumer.inner.accepted_interval_count(),
        shadow.inner.accepted_interval_count() + 1,
        "thirty coupled slabs publish exactly one persistent parent interval",
    );
    if selected_seconds.to_bits() == 1_800.0_f64.to_bits() && second_lane_swe_m.is_none() {
        assert_eq!(
            ending_consumer, historical_complete_candidate,
            "one-child coordinator must be bit-identical to the complete historical candidate",
        );
    }
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
    poisoned.selected_upper_bound_s_bits = if selected_seconds.to_bits() == 900.0_f64.to_bits() {
        60.0_f64.to_bits()
    } else {
        900.0_f64.to_bits()
    };
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
    assert!(open_only_executor.stack.take_staged_stage3().is_some());
    let open_only_ending = open_only_executor
        .stack
        .take_staged_ending()
        .expect("open-only staged owners");
    assert_eq!(
        open_only_ending.inner.lse_state.tiles, open_shadow.inner.lse_state.tiles,
        "open-only execution changes receipt chronology but not LSE tile physics",
    );
    let accepted_soil = &open_only_ending.inner.soil_thermal.ofes[0].ordered_layers;
    let beginning_soil = &open_shadow.inner.soil_thermal.ofes[0].ordered_layers;
    assert_ne!(
        accepted_soil[0].temperature_k.to_bits(),
        beginning_soil[0].temperature_k.to_bits(),
        "persistent snow installs its equal-and-opposite top-soil heat credit",
    );
    assert_eq!(
        &accepted_soil[1..],
        &beginning_soil[1..],
        "the OFE-ground lower boundary mutates only the first ordered soil node",
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

#[test]
#[allow(clippy::too_many_lines)]
fn complete_owner_two_ofe_child_routes_upstream_runoff_as_downstream_runon() {
    let (shadow, fixture) = v10_shadow_fixture_from(two_ofe_routed_endpoint_fixture());
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
    assert!(
        ingress.receipts().iter().any(|receipt| {
            receipt.source_parcel_id == routed.source_parcel_id
                && receipt.basis_ofe_id.as_str() == "ofe-2"
                && receipt.kind
                    == crate::direct_runtime::DirectSurfaceLiquidParcelKind::UpstreamRunon
        }),
        "downstream disposition must retain upper parcel lineage"
    );
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
    assert_eq!(
        ending.inner.accepted_interval_count(),
        shadow.inner.accepted_interval_count() + 1
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
            .take_staged_ending()
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
