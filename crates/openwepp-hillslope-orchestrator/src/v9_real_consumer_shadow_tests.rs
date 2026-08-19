#[cfg(test)]
mod tests {
    use openwepp_input_contract::parsers::climate::{ParserMode, parse_climate_from_str};
    use openwepp_kernel_contract::TileId;
    use openwepp_land_surface_energy::{
        OfeId, SoilThermalLayerCandidate, V2_MODEL_DEFINITION_SHA256, V2_MODEL_VERSION,
        V2_VEGETATION_MODEL_DEFINITION_SHA256, V2_VEGETATION_MODEL_VERSION,
    };
    use openwepp_vegetation::{
        V9_MODEL_SHA256, V9CoupledOwnedState, V10_MODEL_SHA256, V10CoupledOwnedState,
    };

    use super::*;
    use crate::land_surface_energy_shadow::{EndpointFixture, endpoint_fixture};
    use crate::runtime_inputs::{
        SnowFreeHalfHourProviderCursor, SnowFreeHalfHourStaticConfiguration,
        build_hillslope_climate_runtime_request,
    };
    use crate::{
        DirectExecutorMode, DirectFrameExecutor, DirectLanedActiveConfig,
        DirectLanedActiveLaneConfig, DirectLanedActiveMeshPolicy, DirectPublicationCalendarDay,
        DirectPublicationDayInput, DirectPublicationRunMetadata,
    };

    fn v9_configuration_and_state(
        fixture: &EndpointFixture,
    ) -> (VegetationConfiguration, V9CoupledOwnedState) {
        let mut configuration = fixture.vegetation_configuration.clone();
        configuration.model_definition_sha256 = V9_MODEL_SHA256.into();
        configuration.configuration_sha256 = configuration
            .canonical_sha256()
            .expect("V9 configuration digest");
        let mut state = fixture.vegetation_state.clone();
        state.model_definition_sha256 = V9_MODEL_SHA256.into();
        state
            .configuration_sha256
            .clone_from(&configuration.configuration_sha256);
        state.state_sha256 = state.canonical_sha256();
        let state = V9CoupledOwnedState(state);
        state.validate(&configuration).expect("V9 fixture state");
        (configuration, state)
    }

    fn shadow_fixture() -> (DirectV9RealConsumerShadow, EndpointFixture) {
        let fixture = endpoint_fixture();
        let (configuration, state) = v9_configuration_and_state(&fixture);
        let shadow = DirectV9RealConsumerShadow::try_new(
            configuration,
            state,
            ResourceOwnerId::try_new("vegetation-v8").expect("owner"),
            fixture.lse_configuration.clone(),
            fixture.lse_state.clone(),
            fixture.surface_configuration.clone(),
            fixture.hydrology.layer_maps().to_vec(),
            fixture.thermal.clone(),
            fixture.biogeochemistry.clone(),
            fixture.hydrology.beginning_frame().clone(),
            0,
        )
        .expect("shadow fixture");
        (shadow, fixture)
    }

    #[allow(clippy::too_many_lines)]
    fn v10_shadow_fixture() -> (DirectV10RealConsumerShadow, EndpointFixture) {
        let mut fixture = endpoint_fixture();
        let mut wet_frame = fixture.hydrology.beginning_frame().clone();
        for lane in &mut wet_frame.lanes {
            for layer in &mut lane.subsurface_layers {
                layer.theta_m = 0.95 * layer.porosity * layer.depth_m;
                layer.conductivity_m_s = 1.0e-10;
            }
            lane.water.soil_water_m = lane
                .subsurface_layers
                .iter()
                .map(|layer| layer.theta_m)
                .sum();
        }
        fixture.hydrology = crate::vegetation_real_hydrology_shadow::RealHydrologyShadowAdapter::try_from_day_start(
            &wet_frame,
            fixture.hydrology.day_index(),
            fixture.hydrology.transaction_id(),
            fixture.hydrology.interval_s(),
            fixture.hydrology.hydrology_owner_id().clone(),
            fixture.hydrology.layer_maps(),
        )
        .expect("wet V10 hydrology fixture");
        let (v9_configuration, v9_state) = v9_configuration_and_state(&fixture);
        let mut vegetation_configuration = v9_configuration;
        vegetation_configuration.model_definition_sha256 = V10_MODEL_SHA256.into();
        vegetation_configuration.configuration_sha256 = vegetation_configuration
            .canonical_sha256()
            .expect("V10 configuration digest");
        let mut vegetation_payload = v9_state.0;
        vegetation_payload.model_definition_sha256 = V10_MODEL_SHA256.into();
        vegetation_payload
            .configuration_sha256
            .clone_from(&vegetation_configuration.configuration_sha256);
        for (occupancy_id, occupancy) in &mut vegetation_payload.occupancies {
            let height_m = vegetation_configuration
                .strata
                .iter()
                .find(|stratum| stratum.stratum_id == occupancy_id.stratum_id)
                .expect("occupancy stratum")
                .height_m;
            occupancy.root_node_potential_mm = -1_900.0;
            occupancy.stem_potential_mm = -1_900.0 - 1_000.0 * height_m;
            occupancy.sun_leaf_potential_mm = occupancy.stem_potential_mm - 100.0;
            occupancy.shade_leaf_potential_mm = occupancy.stem_potential_mm - 100.0;
        }
        vegetation_payload.state_sha256 = vegetation_payload.canonical_sha256();
        let vegetation_state = V10CoupledOwnedState(vegetation_payload);

        let mut lse_configuration = fixture.lse_configuration.clone();
        lse_configuration.model_version = V2_MODEL_VERSION.into();
        lse_configuration.model_definition_sha256 =
            Sha256Digest::try_new(V2_MODEL_DEFINITION_SHA256).expect("LSE-V2 digest");
        lse_configuration.vegetation_configuration.model_version =
            V2_VEGETATION_MODEL_VERSION.into();
        lse_configuration
            .vegetation_configuration
            .model_definition_sha256 = Sha256Digest::try_new(V2_VEGETATION_MODEL_DEFINITION_SHA256)
            .expect("V10 vegetation digest");
        lse_configuration
            .vegetation_configuration
            .configuration_sha256 =
            Sha256Digest::try_new(vegetation_configuration.configuration_sha256.clone())
                .expect("V10 configuration receipt");
        lse_configuration.configuration_sha256 = lse_configuration
            .canonical_sha256()
            .expect("LSE-V2 configuration digest");
        let mut lse_payload = fixture.lse_state.clone();
        lse_payload.model_definition_sha256 =
            Sha256Digest::try_new(V2_MODEL_DEFINITION_SHA256).expect("LSE-V2 state identity");
        lse_payload
            .configuration_sha256
            .clone_from(&lse_configuration.configuration_sha256);
        lse_payload.state_sha256 = lse_payload.canonical_sha256().expect("LSE-V2 state digest");
        let lse_state = LandSurfaceEnergyV2State(lse_payload);
        let gsi_owner_configuration = DirectGsiOwnerConfigurationV1::try_new(
            "v10-test-gsi-owner".into(),
            GsiParameters::generalized(),
            41.1,
        )
        .expect("GSI owner configuration");
        let provider_static_configuration = SnowFreeHalfHourStaticConfiguration {
            run_id: fixture
                .hydrology
                .beginning_frame()
                .identity
                .run_id
                .to_string(),
            co2_pa: fixture.receipt.forcing().co2_pa,
            reference_height_m: fixture.receipt.forcing().reference_height_m,
            gsi_owner_configuration_sha256: gsi_owner_configuration.configuration_sha256.clone(),
            destinations: lse_configuration
                .ofes
                .iter()
                .flat_map(|ofe| {
                    let wb14 = DirectOfeWb14Parameters {
                        ofe_id: ofe.ofe_id.clone(),
                        effective_conductivity_m_s: 1e-6,
                        matric_potential_m: 0.1,
                        infiltration_storage_capacity_m: 0.04,
                    };
                    ofe.tiles
                        .iter()
                        .map(move |tile| SnowFreeHalfHourDestination {
                            ofe_id: ofe.ofe_id.as_str().to_string(),
                            tile_id: tile.tile_id.as_str().to_string(),
                            wb14_configuration_sha256: wb14_parameter_sha256(&wb14),
                        })
                })
                .collect(),
        };

        let layer_maps = fixture.hydrology.layer_maps().to_vec();
        let mut root_layers = Vec::new();
        for map in &layer_maps {
            let mut top_m = 0.0;
            for layer_id in &map.layer_ids {
                let key = crate::vegetation_real_hydrology_shadow::RealHydrologySourceKey {
                    ofe_lane: map.ofe_lane,
                    layer_id: layer_id.clone(),
                };
                let fact = fixture
                    .hydrology
                    .layer_facts()
                    .get(&key)
                    .expect("root layer fact");
                let saturation = fact.liquid_water_depth_m / fact.layer_thickness_m / fact.porosity;
                let retention_factor = libm::pow(saturation.max(0.01), -4.05);
                let node_m = top_m + 0.5 * fact.layer_thickness_m;
                let saturated_matric_potential_mm =
                    (-2_200.0 + 1_000.0 * node_m) / retention_factor;
                root_layers.push(
                    DirectRootZoneLayerConfiguration::try_new(
                        map.ofe_lane.lane_index,
                        map.ofe_lane.lane_id,
                        layer_id.clone(),
                        saturated_matric_potential_mm,
                        4.05,
                    )
                    .expect("root layer"),
                );
                top_m += fact.layer_thickness_m;
            }
        }
        let root_zone = DirectRootZoneHydraulicConfiguration::try_new(
            root_layers,
            vegetation_configuration
                .strata
                .iter()
                .map(|stratum| {
                    DirectRootZoneStratumGeometry::try_new(stratum.stratum_id.clone(), 0.2)
                        .expect("root path")
                })
                .collect(),
        )
        .expect("root-zone configuration");
        let shadow = DirectV10RealConsumerShadow::try_new(
            vegetation_configuration,
            vegetation_state,
            ResourceOwnerId::try_new("vegetation-v8").expect("owner"),
            lse_configuration,
            lse_state,
            fixture.surface_configuration.clone(),
            layer_maps,
            fixture.thermal.clone(),
            fixture.biogeochemistry.clone(),
            fixture.hydrology.beginning_frame().clone(),
            0,
            gsi_owner_configuration,
            GsiState::new(),
            provider_static_configuration,
            SnowFreeHalfHourProviderCursor::default(),
            root_zone,
        )
        .expect("V10/LSE-V2 shadow fixture");
        (shadow, fixture)
    }

    fn day_input(fixture: &EndpointFixture) -> DirectV9ShadowDayInput {
        let base_vegetation = fixture.receipt.forcing().clone();
        let intervals = (0..INTERVALS_PER_DAY)
            .map(|index| {
                let mut forcing = fixture.forcing.clone();
                forcing.transaction_id = TransactionId(41 + index as u128);
                forcing.forcing_sha256 = forcing.canonical_sha256().expect("forcing digest");
                DirectV9ShadowIntervalInput {
                    lse_forcing: forcing,
                    vegetation_forcing: base_vegetation.clone(),
                    wb14_parameters: vec![DirectOfeWb14Parameters {
                        ofe_id: OfeId::try_new("ofe-1").expect("OFE"),
                        effective_conductivity_m_s: 1e-6,
                        matric_potential_m: 0.1,
                        infiltration_storage_capacity_m: 0.04,
                    }],
                }
            })
            .collect();
        DirectV9ShadowDayInput::try_new(0, intervals).expect("shadow day input")
    }

    fn production_day_input() -> DirectPublicationDayInput {
        let mut input = DirectPublicationDayInput::calendar_only(DirectPublicationCalendarDay {
            year: 2026,
            julian_day: 1,
            month: 1,
            day_of_month: 1,
            water_year: 2026,
        });
        input.precipitation_m = 0.0;
        input.effective_temperature_c = 7.5;
        input
    }

    #[test]
    fn sealed_repository_receipts_project_into_real_child4_forcing_types() {
        let (mut shadow, fixture) = v10_shadow_fixture();
        let template = day_input(&fixture);
        let source = "5.30\n1 0 0\nTEST STATION 1500\nDAY MON YEAR PRCP STMDUR TIMEP IP TMAX TMIN RAD VWIND WIND TDPT\n41.1 -120.0 1225.0 30 2000 1 CLIGEN 5.30 --seed 123\nMONTHLY MAX TEMP HEADER\n1 2 3 4 5 6 7 8 9 10 11 12\nMONTHLY MIN TEMP HEADER\n-5 -4 -3 -2 -1 0 1 2 3 4 5 6\nMONTHLY RAD HEADER\n100 101 102 103 104 105 106 107 108 109 110 111\nMONTHLY RAIN HEADER\n10 11 12 13 14 15 16 17 18 19 20 21\nDAILY HEADER\nDAILY UNITS\n20 6 2000 0.0 0.0 0.0 0.0 28.0 22.0 0.0 2.5 180.0 20.0\n";
        let climate = parse_climate_from_str(source, ParserMode::Strict).expect("strict climate");
        let request = build_hillslope_climate_runtime_request(&climate).expect("climate request");
        let legacy_configuration = shadow
            .snow_free_provider_configuration(&template)
            .expect("owner-derived provider configuration");
        let configuration = SnowFreeHalfHourStaticConfiguration {
            run_id: legacy_configuration.run_id,
            co2_pa: legacy_configuration.co2_pa,
            reference_height_m: legacy_configuration.reference_height_m,
            gsi_owner_configuration_sha256: shadow
                .gsi_owner_configuration()
                .configuration_sha256
                .clone(),
            destinations: legacy_configuration.destinations,
        };
        let prepared = request
            .prepare_snow_free_gsi_day_from_repository(
                0,
                &configuration,
                shadow.gsi_owner_configuration(),
                shadow.gsi_state(),
                shadow.provider_cursor(),
            )
            .expect("staged GSI/provider owners");
        let receipts = prepared.forcing_receipts();
        assert!(celsius_to_kelvin(receipts[0].intervals[0].air_temperature_c).is_finite());
        let production = fixture.hydrology.beginning_frame().clone();
        let production_input = production_day_input();
        let day_frame = projected_day(&production, &production_input);
        shadow
            .execute_prepared_gsi_day(
                &production,
                &[day_frame],
                &[production_input],
                prepared,
                template,
            )
            .expect("real Child4 consumes repository-derived provider day");
        assert_eq!(shadow.inner.accepted_interval_count(), 48);
    }

    #[test]
    fn v10_rejects_caller_root_hydraulic_template_operands_atomically() {
        let (mut poisoned, fixture) = v10_shadow_fixture();
        let beginning = poisoned.clone();
        let template = day_input(&fixture);
        let mut poisoned_template = template.clone();
        for interval in &mut poisoned_template.intervals {
            for layer in &mut interval.vegetation_forcing.soil_layers {
                layer.matric_potential_mm = -9_999_999.0;
                layer.hydraulic_conductivity_mm_s = 0.75;
                layer.root_path_length_mm = 9_999.0;
                layer.gravity_root_mm = -9_999.0;
                layer.accessible = false;
                layer.frozen = true;
            }
        }
        let source = "5.30\n1 0 0\nTEST STATION 1500\nDAY MON YEAR PRCP STMDUR TIMEP IP TMAX TMIN RAD VWIND WIND TDPT\n41.1 -120.0 1225.0 30 2000 1 CLIGEN 5.30 --seed 123\nMONTHLY MAX TEMP HEADER\n1 2 3 4 5 6 7 8 9 10 11 12\nMONTHLY MIN TEMP HEADER\n-5 -4 -3 -2 -1 0 1 2 3 4 5 6\nMONTHLY RAD HEADER\n100 101 102 103 104 105 106 107 108 109 110 111\nMONTHLY RAIN HEADER\n10 11 12 13 14 15 16 17 18 19 20 21\nDAILY HEADER\nDAILY UNITS\n20 6 2000 0.0 0.0 0.0 0.0 28.0 22.0 0.0 2.5 180.0 20.0\n";
        let climate = parse_climate_from_str(source, ParserMode::Strict).expect("strict climate");
        let request = build_hillslope_climate_runtime_request(&climate).expect("climate request");
        let prepare = |shadow: &DirectV10RealConsumerShadow, template: &DirectV10ShadowDayInput| {
            let legacy = shadow
                .snow_free_provider_configuration(template)
                .expect("provider configuration");
            let configuration = SnowFreeHalfHourStaticConfiguration {
                run_id: legacy.run_id,
                co2_pa: legacy.co2_pa,
                reference_height_m: legacy.reference_height_m,
                gsi_owner_configuration_sha256: shadow
                    .gsi_owner_configuration()
                    .configuration_sha256
                    .clone(),
                destinations: legacy.destinations,
            };
            request
                .prepare_snow_free_gsi_day_from_repository(
                    0,
                    &configuration,
                    shadow.gsi_owner_configuration(),
                    shadow.gsi_state(),
                    shadow.provider_cursor(),
                )
                .expect("prepared provider day")
        };
        let poisoned_prepared = prepare(&poisoned, &poisoned_template);
        let production = fixture.hydrology.beginning_frame().clone();
        let production_input = production_day_input();
        let day_frame = projected_day(&production, &production_input);
        let error = poisoned
            .execute_prepared_gsi_day(
                &production,
                &[day_frame],
                &[production_input],
                poisoned_prepared,
                poisoned_template,
            )
            .expect_err("caller hydraulic template poison must reject");
        assert!(matches!(
            error,
            DirectV10RealConsumerError::Runtime(DirectV9RealConsumerError::Physical(
                ExecuteV8LseRuntimeShadowError::Projection(V8InputProjectionError::Identity(
                    "hydraulic owner operand join"
                ))
            ))
        ));
        assert_eq!(poisoned, beginning);
    }

    #[test]
    fn v10_reprojects_ground_optics_and_upward_longwave_from_lse_owners() {
        let (shadow, fixture) = v10_shadow_fixture();
        let template = day_input(&fixture);
        let provider = &template.intervals[0].vegetation_forcing;
        let canonical = project_live_vegetation_forcing(
            provider,
            &fixture.hydrology,
            shadow.inner.soil_thermal(),
            shadow.inner.root_zone_hydraulic_configuration.as_ref(),
            &shadow.inner.vegetation_configuration,
            shadow.inner.vegetation_state(),
            &shadow.inner.lse_configuration,
            shadow.inner.lse_state(),
        )
        .expect("owner-derived ground forcing");
        let mut poisoned = provider.clone();
        poisoned.ground_albedo_vis = 0.99;
        poisoned.ground_albedo_nir = 0.01;
        poisoned.longwave_up_w_m2 = 1.0;
        let projected = project_live_vegetation_forcing(
            &poisoned,
            &fixture.hydrology,
            shadow.inner.soil_thermal(),
            shadow.inner.root_zone_hydraulic_configuration.as_ref(),
            &shadow.inner.vegetation_configuration,
            shadow.inner.vegetation_state(),
            &shadow.inner.lse_configuration,
            shadow.inner.lse_state(),
        )
        .expect("caller ground forcing is not authoritative");
        assert_eq!(
            projected.ground_albedo_vis.to_bits(),
            canonical.ground_albedo_vis.to_bits()
        );
        assert_eq!(
            projected.ground_albedo_nir.to_bits(),
            canonical.ground_albedo_nir.to_bits()
        );
        assert_eq!(
            projected.longwave_up_w_m2.to_bits(),
            canonical.longwave_up_w_m2.to_bits()
        );
    }

    #[test]
    fn v10_rejects_unowned_external_runon_without_mutation() {
        let (mut shadow, fixture) = v10_shadow_fixture();
        let beginning = shadow.clone();
        let mut interval = day_input(&fixture).intervals.remove(0);
        interval.lse_forcing.runon_parcels.push(openwepp_land_surface_energy::LiquidParcel {
            parcel_kind: openwepp_land_surface_energy::LiquidParcelKind::RoutedRunon,
            parcel_id: openwepp_land_surface_energy::ParcelId::try_new("unowned-runon")
                .expect("parcel id"),
            source_owner_id: ResourceOwnerId::try_new("external-runon").expect("owner"),
            source_ofe_id: OfeId::try_new("ofe-upstream").expect("source OFE"),
            source_tile_id: TileId::try_new("forest").expect("source tile"),
            destination_ofe_id: OfeId::try_new("ofe-1").expect("destination OFE"),
            destination_tile_id: TileId::try_new("forest").expect("destination tile"),
            start_s: 0.0,
            end_s: INTERVAL_S,
            amount_kg_m2_destination_tile_ground: 0.01,
            temperature_provider: openwepp_land_surface_energy::LiquidTemperatureProvider::AcceptedUpstreamOutletParcel,
            temperature_k: Some(285.0),
            specific_liquid_enthalpy_j_kg: None,
            source_state_sha256: None,
        });
        let error = shadow
            .inner
            .execute_interval(0, 0, &interval)
            .expect_err("unowned runon must reject");
        assert_eq!(
            error,
            DirectV9RealConsumerError::Unsupported(
                "external runon lacks an accepted routing owner"
            )
        );
        assert_eq!(shadow, beginning);
    }

    #[test]
    fn v10_zero_radiation_provider_day_executes_all_48_intervals() {
        let (mut shadow, fixture) = v10_shadow_fixture();
        let template = day_input(&fixture);
        let source = "5.30\n1 0 0\nTEST STATION 1500\nDAY MON YEAR PRCP STMDUR TIMEP IP TMAX TMIN RAD VWIND WIND TDPT\n41.1 -120.0 1225.0 30 2000 1 CLIGEN 5.30 --seed 123\nMONTHLY MAX TEMP HEADER\n1 2 3 4 5 6 7 8 9 10 11 12\nMONTHLY MIN TEMP HEADER\n-5 -4 -3 -2 -1 0 1 2 3 4 5 6\nMONTHLY RAD HEADER\n100 101 102 103 104 105 106 107 108 109 110 111\nMONTHLY RAIN HEADER\n10 11 12 13 14 15 16 17 18 19 20 21\nDAILY HEADER\nDAILY UNITS\n20 6 2000 0.0 0.0 0.0 0.0 28.0 22.0 0.0 2.5 180.0 20.0\n";
        let climate = parse_climate_from_str(source, ParserMode::Strict).expect("strict climate");
        let request = build_hillslope_climate_runtime_request(&climate).expect("climate request");
        let legacy_configuration = shadow
            .snow_free_provider_configuration(&template)
            .expect("owner-derived provider configuration");
        let configuration = SnowFreeHalfHourStaticConfiguration {
            run_id: legacy_configuration.run_id,
            co2_pa: legacy_configuration.co2_pa,
            reference_height_m: legacy_configuration.reference_height_m,
            gsi_owner_configuration_sha256: shadow
                .gsi_owner_configuration()
                .configuration_sha256
                .clone(),
            destinations: legacy_configuration.destinations,
        };
        let prepared = request
            .prepare_snow_free_gsi_day_from_repository(
                0,
                &configuration,
                shadow.gsi_owner_configuration(),
                shadow.gsi_state(),
                shadow.provider_cursor(),
            )
            .expect("staged GSI/provider owners");
        let production = fixture.hydrology.beginning_frame().clone();
        let production_input = production_day_input();
        let day_frame = projected_day(&production, &production_input);
        let receipt = shadow
            .execute_prepared_gsi_day(
                &production,
                &[day_frame],
                &[production_input],
                prepared,
                template,
            )
            .expect("complete zero-radiation provider day");
        assert_eq!(receipt.accepted_interval_count, 48);
        assert_eq!(shadow.vegetation_state.0.last_transaction_id, 88);
        assert_eq!(
            shadow.lse_state.0.last_accepted_transaction_id,
            Some(TransactionId(88))
        );
        assert_eq!(shadow.gsi_state().sample_count(), 1);
        assert_ne!(
            shadow.provider_cursor(),
            &SnowFreeHalfHourProviderCursor::default()
        );
    }

    #[test]
    fn prepared_gsi_provider_day_rolls_back_every_owner_on_downstream_failure() {
        let (mut shadow, fixture) = v10_shadow_fixture();
        let beginning = shadow.clone();
        let mut template = day_input(&fixture);
        template.intervals[47].lse_forcing.snow_present_at_beginning = true;
        let source = "5.30\n1 0 0\nTEST STATION 1500\nDAY MON YEAR PRCP STMDUR TIMEP IP TMAX TMIN RAD VWIND WIND TDPT\n41.1 -120.0 1225.0 30 2000 1 CLIGEN 5.30 --seed 123\nMONTHLY MAX TEMP HEADER\n1 2 3 4 5 6 7 8 9 10 11 12\nMONTHLY MIN TEMP HEADER\n-5 -4 -3 -2 -1 0 1 2 3 4 5 6\nMONTHLY RAD HEADER\n100 101 102 103 104 105 106 107 108 109 110 111\nMONTHLY RAIN HEADER\n10 11 12 13 14 15 16 17 18 19 20 21\nDAILY HEADER\nDAILY UNITS\n20 6 2000 0.0 0.0 0.0 0.0 28.0 22.0 0.0 2.5 180.0 20.0\n";
        let climate = parse_climate_from_str(source, ParserMode::Strict).expect("strict climate");
        let request = build_hillslope_climate_runtime_request(&climate).expect("climate request");
        let legacy = shadow
            .snow_free_provider_configuration(&template)
            .expect("owner-derived provider configuration");
        let configuration = SnowFreeHalfHourStaticConfiguration {
            run_id: legacy.run_id,
            co2_pa: legacy.co2_pa,
            reference_height_m: legacy.reference_height_m,
            gsi_owner_configuration_sha256: shadow
                .gsi_owner_configuration()
                .configuration_sha256
                .clone(),
            destinations: legacy.destinations,
        };
        let prepared = request
            .prepare_snow_free_gsi_day_from_repository(
                0,
                &configuration,
                shadow.gsi_owner_configuration(),
                shadow.gsi_state(),
                shadow.provider_cursor(),
            )
            .expect("staged GSI/provider owners");
        let production = fixture.hydrology.beginning_frame().clone();
        let production_input = production_day_input();
        let day_frame = projected_day(&production, &production_input);
        assert!(
            shadow
                .execute_prepared_gsi_day(
                    &production,
                    &[day_frame],
                    &[production_input],
                    prepared,
                    template,
                )
                .is_err()
        );
        assert_eq!(shadow, beginning);
    }

    #[test]
    fn v10_positive_radiation_provider_day_executes_low_light_and_daylight() {
        let (mut shadow, fixture) = v10_shadow_fixture();
        let template = day_input(&fixture);
        let source = "5.30\n1 0 0\nTEST STATION 1500\nDAY MON YEAR PRCP STMDUR TIMEP IP TMAX TMIN RAD VWIND WIND TDPT\n41.1 -120.0 1225.0 30 2000 1 CLIGEN 5.30 --seed 123\nMONTHLY MAX TEMP HEADER\n1 2 3 4 5 6 7 8 9 10 11 12\nMONTHLY MIN TEMP HEADER\n-5 -4 -3 -2 -1 0 1 2 3 4 5 6\nMONTHLY RAD HEADER\n100 101 102 103 104 105 106 107 108 109 110 111\nMONTHLY RAIN HEADER\n10 11 12 13 14 15 16 17 18 19 20 21\nDAILY HEADER\nDAILY UNITS\n20 6 2000 0.0 0.0 0.0 0.0 28.0 22.0 20.0 2.5 180.0 20.0\n";
        let source = source.replace("22.0 20.0 2.5", "22.0 500.0 2.5");
        let climate = parse_climate_from_str(&source, ParserMode::Strict).expect("strict climate");
        let request = build_hillslope_climate_runtime_request(&climate).expect("climate request");
        let legacy_configuration = shadow
            .snow_free_provider_configuration(&template)
            .expect("owner-derived provider configuration");
        let configuration = SnowFreeHalfHourStaticConfiguration {
            run_id: legacy_configuration.run_id,
            co2_pa: legacy_configuration.co2_pa,
            reference_height_m: legacy_configuration.reference_height_m,
            gsi_owner_configuration_sha256: shadow
                .gsi_owner_configuration()
                .configuration_sha256
                .clone(),
            destinations: legacy_configuration.destinations,
        };
        let prepared = request
            .prepare_snow_free_gsi_day_from_repository(
                0,
                &configuration,
                shadow.gsi_owner_configuration(),
                shadow.gsi_state(),
                shadow.provider_cursor(),
            )
            .expect("sealed provider receipts");
        let receipts = prepared.forcing_receipts();
        assert_eq!(
            receipts[0].intervals[0]
                .global_horizontal_shortwave_w_m2
                .to_bits(),
            0.0_f64.to_bits()
        );
        assert!(
            receipts[0]
                .intervals
                .iter()
                .any(|interval| interval.global_horizontal_shortwave_w_m2 > 0.0)
        );
        let production = fixture.hydrology.beginning_frame().clone();
        let production_input = production_day_input();
        let day_frame = projected_day(&production, &production_input);
        let receipt = shadow
            .execute_prepared_gsi_day(
                &production,
                &[day_frame],
                &[production_input],
                prepared,
                template,
            )
            .expect("complete realistic positive-radiation provider day");
        assert_eq!(receipt.accepted_interval_count, 48);
        assert_eq!(shadow.vegetation_state.0.last_transaction_id, 88);
        assert_eq!(
            shadow.lse_state.0.last_accepted_transaction_id,
            Some(TransactionId(88))
        );
    }

    #[test]
    fn v10_interval_15_failure_rolls_back_every_shadow_owner_exactly() {
        let (mut shadow, fixture) = v10_shadow_fixture();
        let beginning = shadow.clone();
        let mut input = day_input(&fixture);
        input.intervals[15].lse_forcing.snow_present_at_beginning = true;
        assert!(shadow.execute_intervals_for_test(&input, 15).is_err());
        assert_eq!(shadow, beginning);
    }

    #[test]
    fn v10_midnight_failure_rolls_back_every_shadow_owner_exactly() {
        let (mut shadow, fixture) = v10_shadow_fixture();
        let beginning = shadow.clone();
        let mut input = day_input(&fixture);
        input.intervals[0].lse_forcing.snow_present_at_beginning = true;
        assert!(matches!(
            shadow.execute_first_interval_for_test(&input),
            Err(DirectV10RealConsumerError::Runtime(
                DirectV9RealConsumerError::Unsupported(
                    "forcing transaction, cadence, or snow domain"
                )
            ))
        ));
        assert_eq!(shadow, beginning);
    }

    #[test]
    fn v10_constructor_rejects_independently_valid_wrong_vegetation_receipt() {
        let (shadow, _) = v10_shadow_fixture();
        let mut lse_configuration = shadow.lse_configuration.clone();
        lse_configuration
            .vegetation_configuration
            .configuration_sha256 = Sha256Digest::try_new("9".repeat(64)).expect("wrong receipt");
        lse_configuration.configuration_sha256 = lse_configuration
            .canonical_sha256()
            .expect("altered LSE configuration");
        let mut lse_state = shadow.lse_state.clone();
        lse_state
            .0
            .configuration_sha256
            .clone_from(&lse_configuration.configuration_sha256);
        lse_state.0.state_sha256 = lse_state.0.canonical_sha256().expect("altered LSE state");
        assert!(matches!(
            DirectV10RealConsumerShadow::try_new(
                shadow.vegetation_configuration.clone(),
                shadow.vegetation_state.clone(),
                shadow.inner.vegetation_owner_id.clone(),
                lse_configuration,
                lse_state,
                shadow.inner.surface_configuration.clone(),
                shadow.inner.layer_maps.clone(),
                shadow.inner.soil_thermal.clone(),
                shadow.inner.biogeochemistry.clone(),
                shadow.inner.hydrology_frame.clone(),
                shadow.inner.next_day_index,
                shadow.gsi_owner_configuration.clone(),
                shadow.gsi_state.clone(),
                shadow.provider_static_configuration.clone(),
                shadow.provider_cursor.clone(),
                shadow.root_zone_hydraulic_configuration.clone(),
            ),
            Err(DirectV10RealConsumerError::LseV2(
                LseV2StateError::VegetationIdentity
            ))
        ));
    }

    fn projected_day(
        production: &DirectRunFrame,
        input: &DirectPublicationDayInput,
    ) -> DirectDayFrame {
        let mut day = production.seed_day_frame(0, 0).expect("repository day");
        day.forcing.precipitation_m = input.precipitation_m;
        day.forcing.effective_temperature_c = input.effective_temperature_c;
        day
    }

    fn soil_candidates(fixture: &EndpointFixture) -> Vec<SoilThermalTileCandidate> {
        fixture
            .lse_configuration
            .ofes
            .iter()
            .flat_map(|ofe| {
                let beginning = fixture
                    .thermal
                    .ofes
                    .iter()
                    .find(|value| value.ofe_id == ofe.ofe_id)
                    .expect("beginning OFE");
                ofe.tiles.iter().enumerate().map(move |(tile_index, tile)| {
                    SoilThermalTileCandidate {
                        owner_id: fixture.thermal.owner_id.clone(),
                        beginning_state_sha256: fixture.thermal.state_sha256.clone(),
                        ofe_id: ofe.ofe_id.clone(),
                        tile_id: tile.tile_id.clone(),
                        layers: beginning
                            .ordered_layers
                            .iter()
                            .enumerate()
                            .map(|(layer_index, layer)| {
                                let credit = if layer_index == 0 {
                                    if tile_index == 0 { 10.0 } else { 20.0 }
                                } else {
                                    0.0
                                };
                                SoilThermalLayerCandidate {
                                    layer_id: layer.layer_id.clone(),
                                    beginning_enthalpy_j_m2_ofe_ground: layer
                                        .enthalpy_j_m2_ofe_ground,
                                    ground_heat_credit_j_m2_ofe_ground: credit,
                                    infiltration_enthalpy_credit_j_m2_ofe_ground: 0.0,
                                    ending_enthalpy_j_m2_ofe_ground: layer.enthalpy_j_m2_ofe_ground
                                        + credit,
                                    ending_temperature_k: layer.temperature_k,
                                }
                            })
                            .collect(),
                    }
                })
            })
            .collect()
    }

    #[test]
    fn forty_eight_interval_day_replaces_only_complete_shadow_state() {
        let (mut shadow, fixture) = shadow_fixture();
        let production = fixture.hydrology.beginning_frame().clone();
        let production_before = production.clone();
        let input = day_input(&fixture);
        let production_input = production_day_input();
        let projected = projected_day(&production, &production_input);
        let receipt = shadow
            .execute_day(&production, &[projected], &[production_input], &input)
            .expect("complete shadow day");
        assert_eq!(receipt.accepted_interval_count, 48);
        assert_eq!(receipt.first_transaction_id, TransactionId(41));
        assert_eq!(receipt.last_transaction_id, TransactionId(88));
        assert_eq!(shadow.accepted_interval_count(), 48);
        assert_eq!(shadow.vegetation_state().0.last_transaction_id, 88);
        assert_eq!(production, production_before);
        assert_ne!(
            receipt.beginning_shadow_diagnostic_fingerprint,
            receipt.ending_shadow_diagnostic_fingerprint
        );
    }

    #[test]
    fn failed_late_interval_rolls_back_every_shadow_and_production_byte() {
        let (mut shadow, fixture) = shadow_fixture();
        let production = fixture.hydrology.beginning_frame().clone();
        let production_before = production.clone();
        let shadow_before = shadow.clone();
        let mut input = day_input(&fixture);
        input.intervals[47].lse_forcing.snow_present_at_end = true;
        let production_input = production_day_input();
        let projected = projected_day(&production, &production_input);
        assert!(matches!(
            shadow.execute_day(&production, &[projected], &[production_input], &input),
            Err(DirectV9RealConsumerError::Unsupported(_))
        ));
        assert_eq!(shadow, shadow_before);
        assert_eq!(production, production_before);
    }

    #[test]
    fn retained_half_day_restart_is_byte_identical_to_uninterrupted_day() {
        let (mut uninterrupted, fixture) = shadow_fixture();
        let input = day_input(&fixture);
        for (index, interval) in input.intervals.iter().enumerate() {
            uninterrupted
                .execute_interval(0, index, interval)
                .expect("uninterrupted interval");
        }
        let (mut first_half, _) = shadow_fixture();
        for (index, interval) in input.intervals[..24].iter().enumerate() {
            first_half
                .execute_interval(0, index, interval)
                .expect("first restart half");
        }
        let vegetation: V9CoupledOwnedState = serde_json::from_slice(
            &serde_json::to_vec(&first_half.vegetation_state).expect("vegetation checkpoint"),
        )
        .expect("vegetation reload");
        let lse: LandSurfaceEnergyState = serde_json::from_slice(
            &serde_json::to_vec(&first_half.lse_state).expect("LSE checkpoint"),
        )
        .expect("LSE reload");
        let soil: SoilThermalSnapshot = serde_json::from_slice(
            &serde_json::to_vec(&first_half.soil_thermal).expect("soil checkpoint"),
        )
        .expect("soil reload");
        let bgc: BiogeochemistryState = serde_json::from_slice(
            &serde_json::to_vec(&first_half.biogeochemistry).expect("BGC checkpoint"),
        )
        .expect("BGC reload");
        let mut checkpoint = first_half.checkpoint();
        checkpoint.shadow.vegetation_state = vegetation;
        checkpoint.shadow.lse_state = lse;
        checkpoint.shadow.soil_thermal = soil;
        checkpoint.shadow.biogeochemistry = bgc;
        let mut restarted = DirectV9RealConsumerShadow::restore(checkpoint)
            .expect("complete typed restart owner reload");
        for (index, interval) in input.intervals[24..].iter().enumerate() {
            restarted
                .execute_interval(0, index + 24, interval)
                .expect("second restart half");
        }
        assert_eq!(restarted, uninterrupted);
        assert_eq!(
            restarted
                .diagnostic_fingerprint()
                .expect("restarted fingerprint"),
            uninterrupted
                .diagnostic_fingerprint()
                .expect("uninterrupted fingerprint")
        );
    }

    #[test]
    fn shared_soil_thermal_aggregation_is_ordered_complete_and_owner_bound() {
        let (_, fixture) = shadow_fixture();
        let candidates = soil_candidates(&fixture);
        let ending = aggregate_soil_thermal_ending(
            &fixture.thermal,
            &fixture.lse_configuration,
            TransactionId(41),
            &candidates,
        )
        .expect("complete shared aggregate");
        let expected_credit = candidates
            .iter()
            .map(|candidate| candidate.layers[0].ground_heat_credit_j_m2_ofe_ground)
            .sum::<f64>();
        assert_eq!(
            ending.ofes[0].ordered_layers[0]
                .enthalpy_j_m2_ofe_ground
                .to_bits(),
            (fixture.thermal.ofes[0].ordered_layers[0].enthalpy_j_m2_ofe_ground + expected_credit)
                .to_bits()
        );
        let mut reversed = candidates.clone();
        reversed.reverse();
        assert_eq!(
            aggregate_soil_thermal_ending(
                &fixture.thermal,
                &fixture.lse_configuration,
                TransactionId(41),
                &reversed,
            )
            .expect("canonical tile order"),
            ending
        );
        let mut omitted = candidates.clone();
        omitted.pop();
        assert!(
            aggregate_soil_thermal_ending(
                &fixture.thermal,
                &fixture.lse_configuration,
                TransactionId(41),
                &omitted,
            )
            .is_err()
        );
        let mut duplicate = candidates.clone();
        duplicate.push(candidates[0].clone());
        assert!(
            aggregate_soil_thermal_ending(
                &fixture.thermal,
                &fixture.lse_configuration,
                TransactionId(41),
                &duplicate,
            )
            .is_err()
        );
        let mut wrong_owner = candidates;
        wrong_owner[0].owner_id = ResourceOwnerId::try_new("wrong-soil-owner").expect("owner");
        assert!(
            aggregate_soil_thermal_ending(
                &fixture.thermal,
                &fixture.lse_configuration,
                TransactionId(41),
                &wrong_owner,
            )
            .is_err()
        );
        let mut extra_tile = wrong_owner;
        extra_tile[0].owner_id = fixture.thermal.owner_id.clone();
        extra_tile[0].tile_id = TileId::try_new("nonexistent-extra-tile").expect("tile");
        assert!(
            aggregate_soil_thermal_ending(
                &fixture.thermal,
                &fixture.lse_configuration,
                TransactionId(41),
                &extra_tile,
            )
            .is_err()
        );
        let mut extra_layer = soil_candidates(&fixture);
        let repeated_layer = extra_layer[0].layers[0].clone();
        extra_layer[0].layers.push(repeated_layer);
        assert!(
            aggregate_soil_thermal_ending(
                &fixture.thermal,
                &fixture.lse_configuration,
                TransactionId(41),
                &extra_layer,
            )
            .is_err()
        );
    }

    #[test]
    fn mixed_complete_owner_lineage_is_rejected_before_execution() {
        let (mut shadow, _) = shadow_fixture();
        shadow.lse_state.last_accepted_transaction_id = Some(TransactionId(39));
        assert!(shadow.validate_complete_owner_set().is_err());
        let (mut shadow, _) = shadow_fixture();
        shadow.soil_thermal.last_accepted_transaction_id = Some(TransactionId(39));
        assert!(shadow.validate_complete_owner_set().is_err());
        let (mut shadow, _) = shadow_fixture();
        shadow.layer_maps[0].ofe_lane.lane_id = u32::MAX;
        assert!(shadow.validate_complete_owner_set().is_err());
    }

    #[test]
    fn explicit_scheduler_consumer_advances_shadow_without_changing_production() {
        let (mut shadow, fixture) = shadow_fixture();
        let mut baseline = fixture.hydrology.beginning_frame().clone();
        let mut observed = baseline.clone();
        let shadow_input = day_input(&fixture);
        let production_input = production_day_input();
        let metadata = DirectPublicationRunMetadata {
            run_name: "v9-real-consumer-shadow".into(),
            runtime_selection: "direct-default-off-shadow-test".into(),
            output_policy: "test-only".into(),
        };
        let executor = DirectFrameExecutor::new(DirectExecutorMode::ShadowOnly);
        let mut baseline_rows = Vec::new();
        let baseline_report = executor
            .run_publication_stream_with_interleaved_day_inputs_and_day_frames(
                &mut baseline,
                metadata.clone(),
                |_, _, _| Ok(production_input.clone()),
                |row, _| {
                    baseline_rows.push(row.clone());
                    Ok(())
                },
            )
            .expect("baseline production run");
        let mut observed_rows = Vec::new();
        let observed_report = executor
            .run_publication_stream_with_v9_real_consumer_shadow(
                &mut observed,
                metadata,
                |_, _, _| Ok(production_input.clone()),
                |_, _, _| Ok(shadow_input.clone()),
                |row, _| {
                    observed_rows.push(row.clone());
                    Ok(())
                },
                &mut shadow,
            )
            .expect("explicit default-off shadow run");
        assert_eq!(observed, baseline);
        assert_eq!(observed_rows, baseline_rows);
        assert_eq!(observed_report, baseline_report);
        assert_eq!(shadow.accepted_interval_count(), INTERVALS_PER_DAY as u64);
    }

    #[test]
    fn scheduler_consumes_repository_prepared_v10_day_without_changing_production() {
        let (mut shadow, fixture) = v10_shadow_fixture();
        let mut production = fixture.hydrology.beginning_frame().clone();
        let mut baseline = production.clone();
        let template = day_input(&fixture);
        let production_input = production_day_input();
        let source = "5.30\n1 0 0\nTEST STATION 1500\nDAY MON YEAR PRCP STMDUR TIMEP IP TMAX TMIN RAD VWIND WIND TDPT\n41.1 -120.0 1225.0 30 2000 1 CLIGEN 5.30 --seed 123\nMONTHLY MAX TEMP HEADER\n1 2 3 4 5 6 7 8 9 10 11 12\nMONTHLY MIN TEMP HEADER\n-5 -4 -3 -2 -1 0 1 2 3 4 5 6\nMONTHLY RAD HEADER\n100 101 102 103 104 105 106 107 108 109 110 111\nMONTHLY RAIN HEADER\n10 11 12 13 14 15 16 17 18 19 20 21\nDAILY HEADER\nDAILY UNITS\n20 6 2000 0.0 0.0 0.0 0.0 28.0 22.0 0.0 2.5 180.0 20.0\n";
        let climate = parse_climate_from_str(source, ParserMode::Strict).expect("strict climate");
        let request = build_hillslope_climate_runtime_request(&climate).expect("climate request");
        let executor = DirectFrameExecutor::new(DirectExecutorMode::ShadowOnly);
        executor
            .run_publication_stream_with_interleaved_day_inputs_and_day_frames(
                &mut baseline,
                DirectPublicationRunMetadata {
                    run_name: "v10-repository-scheduler-baseline".into(),
                    runtime_selection: "direct-baseline".into(),
                    output_policy: "test-only".into(),
                },
                |_, _, _| Ok(production_input.clone()),
                |_, _| Ok(()),
            )
            .expect("baseline scheduler day");
        executor
            .run_publication_stream_with_v10_prepared_shadow(
                &mut production,
                DirectPublicationRunMetadata {
                    run_name: "v10-repository-scheduler-shadow".into(),
                    runtime_selection: "direct-default-off-shadow-test".into(),
                    output_policy: "test-only".into(),
                },
                |_, _, _| Ok(production_input.clone()),
                |day_index, _, _, candidate| {
                    let prepared = request
                        .prepare_snow_free_gsi_day_from_repository(
                            day_index,
                            candidate.provider_static_configuration(),
                            candidate.gsi_owner_configuration(),
                            candidate.gsi_state(),
                            candidate.provider_cursor(),
                        )
                        .map_err(|error| {
                            crate::DirectRuntimeError::V9RealConsumerShadowFailure {
                                category: "forcing_provider",
                                detail: error.to_string(),
                            }
                        })?;
                    Ok((prepared, template.clone()))
                },
                |_, _| Ok(()),
                &mut shadow,
            )
            .expect("scheduler consumes sealed repository day");
        assert_eq!(production, baseline);
        assert_eq!(shadow.inner.accepted_interval_count(), 48);
        shadow
            .provider_cursor()
            .validate_for_configuration(shadow.provider_static_configuration(), 1)
            .expect("provider cursor advances exactly once");
    }

    #[test]
    fn downstream_scheduler_failure_discards_production_and_complete_shadow_candidate() {
        let (mut shadow, fixture) = shadow_fixture();
        let mut production = fixture.hydrology.beginning_frame().clone();
        let production_before = production.clone();
        let shadow_before = shadow.clone();
        let shadow_input = day_input(&fixture);
        let production_input = production_day_input();
        let error = DirectFrameExecutor::new(DirectExecutorMode::ShadowOnly)
            .run_publication_stream_with_v9_real_consumer_shadow(
                &mut production,
                DirectPublicationRunMetadata {
                    run_name: "v9-shadow-rollback".into(),
                    runtime_selection: "direct-default-off-shadow-test".into(),
                    output_policy: "test-only".into(),
                },
                |_, _, _| Ok(production_input.clone()),
                |_, _, _| Ok(shadow_input.clone()),
                |_, _| {
                    Err(crate::DirectRuntimeError::PublicationSinkFailure {
                        detail: "injected after shadow day".into(),
                    })
                },
                &mut shadow,
            )
            .expect_err("injected downstream failure");
        assert!(matches!(
            error,
            crate::DirectRuntimeError::PublicationSinkFailure { .. }
        ));
        assert_eq!(production, production_before);
        assert_eq!(shadow, shadow_before);
    }

    #[test]
    fn active_routing_is_typed_unsupported_before_any_shadow_or_production_change() {
        let (mut shadow, fixture) = shadow_fixture();
        let mut production = fixture.hydrology.beginning_frame().clone();
        production.laned_active = Some(Box::new(DirectLanedActiveConfig {
            lanes: vec![DirectLanedActiveLaneConfig {
                slplen_m: 10.0,
                width_m: 10.0,
                mean_gradient: 0.01,
                skin_friction_coefficient_ko: 500.0,
                form_drag_coefficient: 0.0,
                roughness_element_height_m: 0.0,
                roughness_concentration: 0.0,
                vegetation_drag_coefficient: 0.0,
                canopy_height_m: None,
            }],
            mesh_policy: DirectLanedActiveMeshPolicy::FixedCells { cells: 10 },
            max_dt_s: 300.0,
            trace_enabled: false,
            trace_detail_filter: None,
            step_trace_enabled: false,
        }));
        let production_before = production.clone();
        let shadow_before = shadow.clone();
        let shadow_input = day_input(&fixture);
        let production_input = production_day_input();
        let error = DirectFrameExecutor::new(DirectExecutorMode::ShadowOnly)
            .run_publication_stream_with_v9_real_consumer_shadow(
                &mut production,
                DirectPublicationRunMetadata {
                    run_name: "v9-active-unsupported".into(),
                    runtime_selection: "direct-default-off-shadow-test".into(),
                    output_policy: "test-only".into(),
                },
                |_, _, _| Ok(production_input.clone()),
                |_, _, _| Ok(shadow_input.clone()),
                |_, _| Ok(()),
                &mut shadow,
            )
            .expect_err("active routing must reject");
        assert!(matches!(
            error,
            crate::DirectRuntimeError::DirectDomainViolation {
                field: "v9_shadow.laned_active_unsupported"
            }
        ));
        assert_eq!(production, production_before);
        assert_eq!(shadow, shadow_before);
    }

    #[test]
    fn repository_day_receipt_mismatch_discards_both_candidates() {
        let (mut shadow, fixture) = shadow_fixture();
        let mut production = fixture.hydrology.beginning_frame().clone();
        let production_before = production.clone();
        let shadow_before = shadow.clone();
        let shadow_input = day_input(&fixture);
        let mut actual_input = production_day_input();
        actual_input.precipitation_m = f64::from_bits(actual_input.precipitation_m.to_bits() ^ 1);
        let mut published_row_count = 0_usize;
        let error = DirectFrameExecutor::new(DirectExecutorMode::ShadowOnly)
            .run_publication_stream_with_v9_real_consumer_shadow(
                &mut production,
                DirectPublicationRunMetadata {
                    run_name: "v9-provider-poison".into(),
                    runtime_selection: "direct-default-off-shadow-test".into(),
                    output_policy: "test-only".into(),
                },
                |_, _, _| Ok(actual_input.clone()),
                |_, _, _| Ok(shadow_input.clone()),
                |_, _| {
                    published_row_count += 1;
                    Ok(())
                },
                &mut shadow,
            )
            .expect_err("repository receipt mismatch");
        assert!(matches!(
            error,
            crate::DirectRuntimeError::V9RealConsumerShadowFailure {
                category: "identity",
                ..
            }
        ));
        assert_eq!(published_row_count, 0);
        assert_eq!(production, production_before);
        assert_eq!(shadow, shadow_before);
    }
}
