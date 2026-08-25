#[cfg(test)]
mod tests {
    use std::collections::{BTreeMap, BTreeSet};

    use openwepp_coupled_time::{
        ConstraintClass, CoupledClockStateV1, CoupledSlabCandidateV1, Digest32, LedgerEntryV1,
        ModelTimeNs, OwnerState, ParentAuthorityV1, ParentIntervalId, ParentTransactionId,
        SegmentId, StepConstraintV1, TimeSupport, accept_slab, complete_owner_set_digest,
        digest_bytes, reduce_constraints,
    };
    use openwepp_input_contract::parsers::climate::{ParserMode, parse_climate_from_str};
    use openwepp_kernel_contract::TileId;
    use openwepp_land_surface_energy::{
        OfeId, SoilThermalLayerCandidate, V2_MODEL_DEFINITION_SHA256, V2_MODEL_VERSION,
        V2_VEGETATION_MODEL_DEFINITION_SHA256, V2_VEGETATION_MODEL_VERSION,
    };
    use crate::snow_stage3_terminal_handoff::{
        CanopyLongwaveComponent, CarrierSurface, CompleteOwnerSet, ParticipantSupportReceipt,
        SealedCoveredCarrierForcing, SealedCoveredCarrierForcingInputs, SealedExposureReceipt,
        SegmentPhase, SharedCarrierInput, SnowCarrierLedgerInput,
        SnowFreeContinuationInput, SnowStage3HandoffRuntime, SnowStage3OwnerExecutionReceipt,
        SnowStage3TerminalHandoffRequest, TerminalEventInput,
        TerminalStateRates,
    };
    use crate::snow_stage3_open_boundary::{
        SealedOpenSnowExposureReceiptV1, SealedOpenSnowTileForcingInputsV1,
        SealedOpenSnowTileForcingV1, SealedStage3TileBoundaryForcingV1,
    };
    use crate::snow_stage3_v11_attachment::{
        DirectSnowStage3V11StaticContext, DirectSnowStage3V11TerminalParcelPosture,
        PreparedStage3V11DayV1,
        PreparedStage3V11SupportIdentityV1, PreparedStage3V11SupportV1,
        STAGE3_V11_PARENT_SUPPORT_NS, Stage3LaneLifecycleV1, Stage3V11FailureInjection,
        execute_covered_real_v11_parent, stage3_lane_lifecycle,
    };
    use crate::v9_real_consumer_shadow::{
        DirectV11SnowCoveredRealConsumerStack, DirectV11SnowCoveredSegmentInput,
        DirectV11SnowCoveredStackInputs,
    };
    use crate::hydrology::{
        DirectActiveSnowPartitionInputs, DirectSnowHourlyForcing, DirectSnowStage3SupportInput,
        DirectSnowSurfaceEnergyOptions, DirectSnowTerminalEventRequest,
        SnowDensityModel, SnowMeltModel, SnowStage3LiquidRoutingModel, SnowSurfaceLongwaveModel,
        Wb11HydrologyKernel, stage3_is_resolved_thermal_domain,
    };
    use crate::winter_column::DirectSnowLayerState;
    use openwepp_vegetation::v11::{
        V11_COMPLETE_OWNER_MANIFEST, V11ExecutionError, V11OwnerEnvelope, V11ParentCandidate,
        V11ParentTransaction, V11ResourceDebit, V11SharedResourceOwnerTransition,
        migrate_v10_runtime_to_v11,
        v11_vegetation_owner_envelope,
    };

    fn test_wb14_coupled_binding() -> crate::direct_runtime::DirectWb14CoupledChildBindingV1 {
        crate::direct_runtime::DirectWb14CoupledChildBindingV1 {
            proposed_upper_bound_s_bits: 1_800.0_f64.to_bits(),
            coupled_parent_transaction_sha256: [1; 32],
            accepted_slab_sha256: [2; 32],
            parent_beginning_complete_owner_set_sha256: [3; 32],
            parent_support_start_ns: 0,
            parent_support_end_ns: 1_800_000_000_000,
            child_support_start_ns: 0,
            child_support_end_ns: 1_800_000_000_000,
        }
    }
    use openwepp_vegetation::{
        V8CoupledOwnedState, V9_MODEL_SHA256, V9CoupledOwnedState, V10_MODEL_SHA256,
        V10CoupledOwnedState,
    };

    use super::*;
    use crate::land_surface_energy_shadow::{EndpointFixture, endpoint_fixture};
    use crate::land_surface_energy_shadow::strict_v8_endpoint::endpoint_rollback_tests::two_ofe_routed_endpoint_fixture;
    use crate::runtime_inputs::{
        PreparedSnowFreeGsiDayV1, SnowFreeHalfHourProviderCursor,
        SnowFreeHalfHourStaticConfiguration,
        build_hillslope_climate_runtime_request,
    };
    use crate::{
        DirectExecutorMode, DirectFrameExecutor, DirectLanedActiveConfig,
        DirectLanedActiveLaneConfig, DirectLanedActiveMeshPolicy, DirectPublicationCalendarDay,
        DirectPublicationDayInput, DirectPublicationRunMetadata,
    };

    /// Generated-wire projection of every V8 physical field. Only the released
    /// successor identity/chronology paths are removed; both source states stay
    /// immutable and every remaining serialized field compares exactly.
    #[derive(Debug, PartialEq)]
    struct V11NonIdentityPhysicalProjection(serde_json::Value);

    impl V11NonIdentityPhysicalProjection {
        fn from_v8(state: &V8CoupledOwnedState) -> Self {
            let mut value = serde_json::to_value(state).expect("serialize V8 physical ledger");
            let root = value.as_object_mut().expect("V8 state object");
            for identity in [
                "model_definition_sha256",
                "configuration_sha256",
                "state_sha256",
                "last_transaction_id",
            ] {
                assert!(
                    root.remove(identity).is_some(),
                    "missing V8 identity field {identity}"
                );
            }
            let strata = root
                .get_mut("strata")
                .and_then(serde_json::Value::as_object_mut)
                .expect("typed V8 stratum map");
            for state in strata.values_mut() {
                let state = state.as_object_mut().expect("typed V8 stratum entry");
                assert!(state.remove("last_transaction_id").is_some());
            }
            let occupancies = root
                .get_mut("occupancies")
                .and_then(serde_json::Value::as_array_mut)
                .expect("typed V8 occupancy map");
            for entry in occupancies {
                let state = entry
                    .as_object_mut()
                    .and_then(|row| row.get_mut("state"))
                    .and_then(serde_json::Value::as_object_mut)
                    .expect("typed V8 occupancy entry");
                assert!(state.remove("last_accepted_transaction_id").is_some());
            }
            Self(value)
        }
    }

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
        v10_shadow_fixture_from(endpoint_fixture())
    }

    #[allow(clippy::too_many_lines)]
    fn v10_shadow_fixture_from(
        mut fixture: EndpointFixture,
    ) -> (DirectV10RealConsumerShadow, EndpointFixture) {
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

    fn attachment_stage3_inputs() -> DirectActiveSnowPartitionInputs {
        let mut layer = DirectSnowLayerState::new(0.18, 0.40, 450.0, 12.0);
        layer.temperature_c = -8.0;
        layer.cold_content_j_m2 = 0.18 * 1_000.0 * 2_100.0 * 8.0;
        DirectActiveSnowPartitionInputs {
            hyetograph_rainfall_m: 0.0,
            rst_c: 0.0,
            newsnw_kg_m3: 100.0,
            ssd_kg_m3: 522.0,
            runtime_swe_m: 0.18,
            runtime_depth_m: 0.40,
            runtime_density_kg_m3: 450.0,
            runtime_settle_day_count: 12.0,
            liquid_water_retained_m: 0.0,
            tmax_c: -3.0,
            tmin_c: -7.0,
            canopy_cover_fraction: 0.45,
            wind_m_s: 3.0,
            dewpoint_c: -15.0,
            snow_melt_model: SnowMeltModel::CoeLiquidHoldingCapacityV1,
            snow_density_model: SnowDensityModel::PhysicsBulkDensityCompactionV1,
            stage3_liquid_routing_model: SnowStage3LiquidRoutingModel::LayeredThermalLiquidV1,
            surface_energy_options: DirectSnowSurfaceEnergyOptions::default(),
            sturm_climate_class: None,
            sturm_day_of_year: None,
            coe_boundary_depth_m: 0.40,
            coe_boundary_density_kg_m3: 450.0,
            coe_boundary_settle_day_count: 12.0,
            snow_albedo_model: None,
            snow_albedo_state: None,
            snow_layers: vec![layer],
            underlying_surface_albedo: 0.2,
            hourly: [DirectSnowHourlyForcing::zero(); 24],
        }
    }

    fn digest_from_receipt(value: &str) -> Digest32 {
        let mut bytes = [0_u8; 32];
        for (index, chunk) in value.as_bytes().chunks_exact(2).enumerate() {
            bytes[index] = u8::from_str_radix(
                std::str::from_utf8(chunk).expect("lower-hex receipt"),
                16,
            )
            .expect("lower-hex receipt digits");
        }
        Digest32::from_bytes(bytes)
    }

    fn attachment_supports(
        provider: &PreparedSnowFreeGsiDayV1,
        interval_template: &DirectV9ShadowIntervalInput,
        lane_id: u32,
        day_index: usize,
    ) -> Vec<PreparedStage3V11SupportV1> {
        attachment_supports_with_start_offset(
            provider,
            interval_template,
            lane_id,
            day_index,
            0,
        )
    }

    fn attachment_supports_with_start_offset(
        provider: &PreparedSnowFreeGsiDayV1,
        interval_template: &DirectV9ShadowIntervalInput,
        lane_id: u32,
        day_index: usize,
        start_offset_ns: u128,
    ) -> Vec<PreparedStage3V11SupportV1> {
        (0..INTERVALS_PER_DAY)
            .map(|interval_index| {
                let provider_interval = &provider.forcing_receipts().receipts()[0].intervals
                    [interval_index];
                let mut interval = interval_template.clone();
                interval.lse_forcing.air_temperature_k =
                    openwepp_meteorology::snow_free_forcing::celsius_to_kelvin(
                        provider_interval.air_temperature_c,
                    );
                interval.lse_forcing.air_specific_humidity_kg_kg =
                    provider_interval.specific_humidity_kg_kg;
                interval.lse_forcing.air_pressure_pa =
                    openwepp_meteorology::snow_free_forcing::kilopascals_to_pascals(
                        provider_interval.pressure_kpa,
                    );
                interval.lse_forcing.reference_wind_m_s = provider_interval.wind_m_s;
                interval.lse_forcing.direct_vis_w_m2 = provider_interval.direct_visible_w_m2;
                interval.lse_forcing.diffuse_vis_w_m2 = provider_interval.diffuse_visible_w_m2;
                interval.lse_forcing.direct_nir_w_m2 = provider_interval.direct_nir_w_m2;
                interval.lse_forcing.diffuse_nir_w_m2 = provider_interval.diffuse_nir_w_m2;
                interval.lse_forcing.atmospheric_downward_longwave_w_m2 =
                    provider_interval.downward_longwave_w_m2;
                interval.lse_forcing.forcing_sha256 = interval
                    .lse_forcing
                    .canonical_sha256()
                    .expect("provider-projected LSE digest");
                interval.vegetation_forcing.air_temperature_k =
                    interval.lse_forcing.air_temperature_k;
                interval.vegetation_forcing.pressure_pa = interval.lse_forcing.air_pressure_pa;
                interval.vegetation_forcing.wind_m_s = provider_interval.wind_m_s;
                interval.vegetation_forcing.specific_humidity =
                    provider_interval.specific_humidity_kg_kg;
                interval.vegetation_forcing.direct_par_w_m2 =
                    provider_interval.direct_visible_w_m2;
                interval.vegetation_forcing.diffuse_par_w_m2 =
                    provider_interval.diffuse_visible_w_m2;
                interval.vegetation_forcing.direct_nir_w_m2 = provider_interval.direct_nir_w_m2;
                interval.vegetation_forcing.diffuse_nir_w_m2 =
                    provider_interval.diffuse_nir_w_m2;
                interval.vegetation_forcing.longwave_down_w_m2 =
                    provider_interval.downward_longwave_w_m2;
                let mut snow_inputs = attachment_stage3_inputs();
                snow_inputs.wind_m_s = provider_interval.wind_m_s;
                snow_inputs.dewpoint_c = provider_interval.dew_point_c;
                snow_inputs.surface_energy_options.atmospheric_pressure_pa =
                    interval.lse_forcing.air_pressure_pa;
                let support_start = (day_index as u128) * 86_400_000_000_000
                    + interval_index as u128 * 1_800_000_000_000
                    + start_offset_ns;
                let support = TimeSupport::new(
                    ModelTimeNs::new(support_start),
                    ModelTimeNs::new(support_start + 1_800_000_000_000),
                )
                .expect("1,800-second support");
                let identities = provider
                    .forcing_receipts()
                    .receipts()
                    .iter()
                    .map(|day| {
                        let interval = &day.intervals[interval_index];
                        PreparedStage3V11SupportIdentityV1::new(
                            interval.ofe_id.clone(),
                            interval.tile_id.clone(),
                            interval.wb14_configuration_sha256.clone(),
                            Digest32::from_bytes([7_u8; 32]),
                            interval.precipitation_parcels.clone(),
                            digest_from_receipt(&interval.interval_receipt_sha256),
                        )
                    })
                    .collect::<Vec<_>>();
                PreparedStage3V11SupportV1::try_new(
                    support,
                    BTreeMap::from([(lane_id, snow_inputs.clone())]),
                    BTreeMap::from([(
                        lane_id,
                        DirectSnowStage3SupportInput {
                            forcing: DirectSnowHourlyForcing {
                                air_temperature_c: provider_interval.air_temperature_c,
                                ..DirectSnowHourlyForcing::zero()
                            },
                            duration_seconds: 1_800.0,
                        },
                    )]),
                    interval,
                    BTreeMap::from([(lane_id, identities)]),
                )
                .expect("runner-built attachment support")
            })
            .collect()
    }

    fn child2c_support(
        participant_id: &str,
        receipt_id: &str,
        minimum_support_ns: u128,
    ) -> ParticipantSupportReceipt {
        ParticipantSupportReceipt {
            participant_id: participant_id.to_owned(),
            support_receipt_id: receipt_id.to_owned(),
            minimum_support_ns: ModelTimeNs::new(minimum_support_ns),
        }
    }

    fn child2c_carrier() -> SharedCarrierInput {
        SharedCarrierInput {
            phase: SegmentPhase::SnowCovered,
            rho_air_kg_m3: 1.2,
            cp_air_j_kg_k: 1005.0,
            reference: CarrierSurface {
                temperature_k: 280.0,
                specific_humidity: 0.002,
                heat_conductance_m_s: 0.1,
                vapor_conductance_m_s: 0.1,
            },
            canopy: CarrierSurface {
                temperature_k: 285.0,
                specific_humidity: 0.004,
                heat_conductance_m_s: 0.05,
                vapor_conductance_m_s: 0.05,
            },
            snow: CarrierSurface {
                temperature_k: 270.0,
                specific_humidity: 0.001,
                heat_conductance_m_s: 0.05,
                vapor_conductance_m_s: 0.05,
            },
            canopy_longwave_components: vec![
                CanopyLongwaveComponent {
                    temperature_k: 285.0,
                    emissive_area_weight: 0.7,
                },
                CanopyLongwaveComponent {
                    temperature_k: 275.0,
                    emissive_area_weight: 0.3,
                },
            ],
            exposure: SealedExposureReceipt {
                receipt_id: "exposure-v1".to_owned(),
                provider: "sealed-stage3-exposure".to_owned(),
                provider_digest: "exposure-provider-digest".to_owned(),
                source: "sealed-exposure-v1".to_owned(),
                wind_m_s: 3.0,
                transfer_height_m: 5.0,
                roughness_m: 0.005,
            },
            active_participants: vec![
                "shared-carrier".to_owned(),
                "stage3-snow".to_owned(),
                "v11-canopy".to_owned(),
            ],
            support_receipts: vec![
                child2c_support("shared-carrier", "support-carrier-v1", 600_000_000),
                child2c_support("stage3-snow", "support-stage3-v1", 600_000_000),
                child2c_support("v11-canopy", "support-v11-v1", 600_000_000),
            ],
            atmospheric_longwave_w_m2: 280.0,
            effective_canopy_cover: 0.5,
            canopy_intercepted_snow: false,
            ledger: SnowCarrierLedgerInput {
                duration_s: 3600.0,
                snow_ice_start_kg_m2: 10.0,
                solid_precipitation_kg_m2: 0.1,
                melt_kg_m2: 0.03,
                sublimation_kg_m2: 0.02,
                deposition_kg_m2: 0.01,
                liquid_start_kg_m2: 0.5,
                rain_kg_m2: 0.2,
                refreeze_kg_m2: 0.01,
                liquid_runoff_kg_m2: 0.1,
                energy_start_j_m2: 1000.0,
                external_energy_j_m2: 5000.0,
                canopy_energy_j_m2: -1000.0,
                snow_energy_j_m2: 3000.0,
                energy_end_j_m2: 8000.0,
                canopy_snow_longwave_exchange_j_m2: -139_473.340_214_138_1,
                snow_canopy_longwave_exchange_j_m2: 139_473.340_214_138_1,
            },
        }
    }

    fn child2c_carrier_forcing() -> SealedCoveredCarrierForcing {
        SealedCoveredCarrierForcing::try_new(SealedCoveredCarrierForcingInputs {
            rho_air_kg_m3: 1.2,
            cp_air_j_kg_k: 1005.0,
            reference_temperature_k: 280.0,
            reference_specific_humidity: 0.002,
            atmospheric_longwave_w_m2: 280.0,
            effective_canopy_cover: 0.5,
            exposure: SealedExposureReceipt {
                receipt_id: "exposure-v1".to_owned(),
                provider: "sealed-stage3-exposure".to_owned(),
                provider_digest: "exposure-provider-digest".to_owned(),
                source: "sealed-exposure-v1".to_owned(),
                wind_m_s: 3.0,
                transfer_height_m: 5.0,
                roughness_m: 0.005,
            },
            active_participants: vec![
                "shared-carrier".to_owned(),
                "stage3-snow".to_owned(),
                "v11-canopy".to_owned(),
            ],
            support_receipts: vec![
                child2c_support("shared-carrier", "support-carrier-v1", 600_000_000),
                child2c_support("stage3-snow", "support-stage3-v1", 600_000_000),
                child2c_support("v11-canopy", "support-v11-v1", 600_000_000),
            ],
        })
        .expect("sealed covered carrier forcing")
    }

    fn child2c_event(parent_end_ns: u128) -> TerminalEventInput {
        TerminalEventInput {
            parent_identity: "parent-child2c-v11-test".to_string(),
            segment_identity: "segment-stage3-v11-test".to_string(),
            event_ordinal: 1,
            parent_start_tick: ModelTimeNs::new(0),
            parent_end_tick: ModelTimeNs::new(parent_end_ns),
            proposed_event_tick: ModelTimeNs::new(0),
            candidate_ticks: vec![ModelTimeNs::new(0)],
            pre_active_participants: vec![
                child2c_support("shared-carrier", "support-carrier-v1", 600_000_000),
                child2c_support("stage3-snow", "support-stage3-v1", 600_000_000),
                child2c_support("v11-canopy", "support-v11-v1", 600_000_000),
            ],
            post_active_participants: vec![child2c_support("v11", "v11-post", 600_000_000)],
            event_time_tolerance_ns: ModelTimeNs::new(0),
            snow_mass_tolerance_kg_m2: 0.0,
            liquid_mass_tolerance_kg_m2: 0.0,
            energy_tolerance_j_m2: 0.0,
            terminal_state: TerminalStateRates {
                snow_start_kg_m2: 10.0,
                snow_rate_kg_m2_s: 0.0,
                snow_target_kg_m2: 10.0,
                liquid_start_kg_m2: 0.5,
                liquid_rate_kg_m2_s: 0.0,
                liquid_target_kg_m2: 0.5,
                energy_start_j_m2: 0.0,
                energy_rate_j_m2_s: 0.0,
                energy_target_j_m2: 0.0,
            },
        }
    }

    fn digest(seed: u8) -> Digest32 {
        Digest32::from_bytes([seed; 32])
    }

    fn accepted_v11_slab(
        owners: &[OwnerState],
        end_ns: u128,
    ) -> (
        ParentTransactionId,
        openwepp_coupled_time::AcceptedSlabReceiptV1,
    ) {
        let (parent, mut receipts) = accepted_v11_slabs(owners, &[end_ns]);
        (parent, receipts.remove(0))
    }

    fn accepted_v11_slabs(
        owners: &[OwnerState],
        end_ticks: &[u128],
    ) -> (
        ParentTransactionId,
        Vec<openwepp_coupled_time::AcceptedSlabReceiptV1>,
    ) {
        let parent_end = *end_ticks.last().expect("at least one slab");
        let support = TimeSupport::new(ModelTimeNs::new(0), ModelTimeNs::new(parent_end))
            .expect("parent support");
        let beginning = complete_owner_set_digest(owners).expect("owner digest");
        let interval =
            ParentIntervalId::derive(digest(1), digest(2), digest(3), support).expect("interval");
        let parent =
            ParentTransactionId::derive(digest(1), 40, interval, beginning).expect("parent");
        let authority =
            ParentAuthorityV1::new(digest(1), digest(2), digest(3), 40, support, beginning)
                .expect("authority");
        let participants = owners
            .iter()
            .map(|owner| owner.owner_id().to_owned())
            .collect::<Vec<_>>();
        let mut clock = CoupledClockStateV1::new(
            authority,
            owners.to_vec(),
            "snow-free".to_owned(),
            participants.clone(),
            digest(4),
            Vec::new(),
        )
        .expect("clock");
        let mut participant_bytes = Vec::new();
        for id in &participants {
            participant_bytes.extend_from_slice(id.as_bytes());
            participant_bytes.push(0);
        }
        let mut start_ns = 0;
        let mut receipts = Vec::with_capacity(end_ticks.len());
        for &end_ns in end_ticks {
            let slab_support =
                TimeSupport::new(ModelTimeNs::new(start_ns), ModelTimeNs::new(end_ns))
                    .expect("slab support");
            let constraint = StepConstraintV1::new(
                parent,
                ModelTimeNs::new(start_ns),
                ModelTimeNs::new(end_ns),
                "vegetation".to_owned(),
                ConstraintClass::HardBoundary,
                digest(5),
                digest(2),
                digest(3),
            )
            .expect("constraint");
            let reduced = reduce_constraints(
                &[constraint],
                parent,
                ModelTimeNs::new(start_ns),
                ModelTimeNs::new(parent_end),
                None,
            )
            .expect("reduced");
            let segment = SegmentId::derive(
                parent,
                0,
                support,
                digest_bytes(b"snow-free"),
                digest_bytes(&participant_bytes),
            )
            .expect("segment");
            let joined = digest(6);
            let ledger = LedgerEntryV1::new(
                "vegetation".to_owned(),
                "owner".to_owned(),
                joined,
                joined,
                digest(7),
            )
            .expect("ledger");
            let slab = CoupledSlabCandidateV1::new(
                &clock,
                segment,
                slab_support,
                &reduced,
                owners.to_vec(),
                vec![ledger],
            )
            .expect("slab");
            receipts.push(accept_slab(&mut clock, slab).expect("accepted slab"));
            start_ns = end_ns;
        }
        (parent, receipts)
    }

    fn initial_v11_owners(
        shadow: &DirectV10RealConsumerShadow,
        state: &openwepp_vegetation::v11::V11CoupledOwnedState,
    ) -> BTreeMap<String, V11OwnerEnvelope> {
        V11_COMPLETE_OWNER_MANIFEST
            .iter()
            .map(|id| {
                let envelope = match *id {
                    "vegetation" => v11_vegetation_owner_envelope(state).expect("vegetation owner"),
                    "land_surface_energy" => V11OwnerEnvelope::try_new(
                        (*id).to_owned(),
                        serde_json::to_vec(&shadow.inner.lse_state).expect("LSE owner"),
                    )
                    .expect("LSE owner envelope"),
                    "soil_thermal" => V11OwnerEnvelope::try_new(
                        (*id).to_owned(),
                        serde_json::to_vec(&shadow.inner.soil_thermal).expect("soil owner"),
                    )
                    .expect("soil owner envelope"),
                    _ => V11OwnerEnvelope::try_new((*id).to_owned(), id.as_bytes().to_vec())
                        .expect("owner"),
                };
                ((*id).to_owned(), envelope)
            })
            .collect()
    }

    fn segment_interval(
        base: &DirectV9ShadowIntervalInput,
        duration_ns: u128,
        transaction_id: u128,
        air_temperature_delta_k: f64,
    ) -> DirectV9ShadowIntervalInput {
        let mut interval = base.clone();
        interval.lse_forcing.interval_s = f64::from_bits(
            TimeSupport::new(ModelTimeNs::new(0), ModelTimeNs::new(duration_ns))
                .expect("segment duration support")
                .duration_s_bits(),
        );
        interval.lse_forcing.transaction_id = TransactionId(transaction_id);
        interval.lse_forcing.air_temperature_k += air_temperature_delta_k;
        interval.lse_forcing.precipitation_parcels.clear();
        interval.lse_forcing.runon_parcels.clear();
        interval.vegetation_forcing.air_temperature_k += air_temperature_delta_k;
        interval.vegetation_forcing.rain_kg_m2 = 0.0;
        interval.lse_forcing.forcing_sha256 = interval
            .lse_forcing
            .canonical_sha256()
            .expect("segment forcing digest");
        interval
    }

    fn run_actual_v11_segments(
        shadow: &DirectV10RealConsumerShadow,
        base_interval: &DirectV9ShadowIntervalInput,
        durations_ns: &[u128],
        temperature_deltas_k: &[f64],
    ) -> V11ParentCandidate {
        assert_eq!(durations_ns.len(), temperature_deltas_k.len());
        let migrated =
            migrate_v10_runtime_to_v11(&shadow.vegetation_configuration, &shadow.vegetation_state)
                .expect("migration");
        let owners = initial_v11_owners(shadow, &migrated.state);
        let clock_owners = owners
            .values()
            .map(|owner| owner.to_owner_state().expect("clock owner"))
            .collect::<Vec<_>>();
        let mut cumulative = 0;
        let end_ticks = durations_ns
            .iter()
            .map(|duration| {
                cumulative += duration;
                cumulative
            })
            .collect::<Vec<_>>();
        let (parent_id, receipts) = accepted_v11_slabs(&clock_owners, &end_ticks);
        let mut parent = V11ParentTransaction::new_with_complete_owners(
            &migrated.configuration,
            &migrated.state,
            parent_id,
            ModelTimeNs::new(0),
            owners,
        )
        .expect("parent");
        let mut staged_shadow = shadow.clone();
        for (ordinal, ((receipt, duration_ns), temperature_delta_k)) in receipts
            .iter()
            .zip(durations_ns)
            .zip(temperature_deltas_k)
            .enumerate()
        {
            let interval = segment_interval(base_interval, *duration_ns, 41, *temperature_delta_k);
            let stack = DirectV11RealConsumerStack::new(&staged_shadow, &interval, 0, ordinal);
            let mut executor =
                crate::v11_vegetation_consumer::DirectV11VegetationExecutor { stack };
            let segment =
                execute_direct_v11_segment(&migrated.configuration, &parent, receipt, &mut executor)
                    .expect("actual segmented V11 execution");
            for transition in &segment.shared_resource_transitions {
                let owner = segment
                    .ending_resource_owners
                    .get(&transition.shared_resource_key.owner_id)
                    .expect("transition ending owner");
                assert_eq!(
                    transition.owner_candidate_sha256, owner.state_sha256,
                    "transition must bind the canonical ending owner"
                );
                if let Some(previous) = parent.accepted_segments().last().and_then(|accepted| {
                    accepted
                        .shared_resource_transitions
                        .iter()
                        .find(|candidate| {
                            candidate.shared_resource_key == transition.shared_resource_key
                        })
                }) {
                    assert_eq!(
                        transition.beginning_amount.to_bits(),
                        previous.ending_amount.to_bits(),
                        "next shared-owner beginning must be the prior staged ending"
                    );
                }
                if transition.shared_resource_key.owner_id == "bgc" {
                    let linked = segment
                        .resource_debits
                        .iter()
                        .filter(|debit| {
                            transition.debit_receipt_ids.contains(&debit.receipt_id)
                        })
                        .collect::<Vec<_>>();
                    assert_eq!(linked.len(), transition.debit_receipt_ids.len());
                    let used = linked.iter().fold(0.0_f64, |sum, debit| {
                        assert_eq!(debit.tile_id, "stratum_scoped");
                        assert!(migrated.configuration.imported_v10.strata.iter().any(
                            |stratum| stratum.stratum_id.as_str() == debit.occupancy_id
                        ));
                        sum + debit.final_use
                    });
                    assert_eq!(
                        (transition.beginning_amount - used).to_bits(),
                        transition.ending_amount.to_bits(),
                        "linked BGC debits must exactly reconstruct the mineral-pool delta"
                    );
                }
            }
            accept_direct_v11_segment(
                &mut parent,
                &migrated.configuration,
                segment,
                &executor.stack.beginning,
            )
            .expect("accept actual segmented V11 execution");
            let support_receipt = executor
                .stack
                .last_support_receipt()
                .expect("sealed LSE support receipt");
            assert_eq!(
                support_receipt.requested_support_ns,
                duration_ns.to_string()
            );
            assert_eq!(support_receipt.slab_ordinal, ordinal.to_string());
            assert_eq!(
                parent
                    .accepted_segments()
                    .last()
                    .expect("accepted segment")
                    .lse_support_receipt
                    .canonical_json,
                serde_json::to_vec(support_receipt).expect("canonical LSE support receipt"),
                "accepted segment must retain the exact sealed support receipt"
            );
            assert_eq!(
                support_receipt.beginning_soil_thermal_state_sha256,
                staged_shadow.inner.soil_thermal.state_sha256,
                "support receipt must bind the staged beginning soil owner"
            );
            staged_shadow = executor.stack.take_staged_ending().expect("staged ending");
        }
        parent.finalize(&migrated.configuration).expect("finalize")
    }

    #[test]
    fn v11_full_support_runs_actual_v10_stack_and_finalizes_once() {
        let (shadow, fixture) = v10_shadow_fixture();
        let interval = day_input(&fixture).intervals.remove(0);
        let migrated =
            migrate_v10_runtime_to_v11(&shadow.vegetation_configuration, &shadow.vegetation_state)
                .expect("migration");
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
        .expect("parent");
        let stack = DirectV11RealConsumerStack::new(&shadow, &interval, 0, 0);
        let mut executor = crate::v11_vegetation_consumer::DirectV11VegetationExecutor { stack };
        let segment = execute_direct_v11_segment(&migrated.configuration, &parent, &slab, &mut executor)
            .expect("actual V11 segment");
        accept_direct_v11_segment(
            &mut parent,
            &migrated.configuration,
            segment,
            &executor.stack.beginning,
        )
        .expect("accept segment");
        let candidate = parent.finalize(&migrated.configuration).expect("finalize");

        let mut expected = shadow.clone();
        expected
            .inner
            .execute_interval(0, 0, &interval)
            .expect("V10");
        expected.vegetation_state = project_v9_runtime_to_v10(
            expected.inner.vegetation_state(),
            &expected.vegetation_configuration,
        )
        .expect("V10 ending");
        assert_eq!(
            V11NonIdentityPhysicalProjection::from_v8(&candidate.ending_state.physical),
            V11NonIdentityPhysicalProjection::from_v8(&expected.vegetation_state.0),
            "every non-identity V11 physical field must exactly match V10"
        );
        assert_eq!(
            candidate.ending_state.last_parent_transaction_id,
            migrated.state.last_parent_transaction_id + 1
        );
        assert_eq!(candidate.ending_complete_owners.len(), 7);
        assert_eq!(candidate.accepted_segments.len(), 1);
        assert_eq!(
            candidate.accepted_segment_checkpoints[0].lse_support_receipt,
            candidate.accepted_segments[0].lse_support_receipt,
            "parent checkpoint must retain the accepted LSE support receipt"
        );
    }
    include!("v9_real_consumer_shadow_wb14_tests.rs");
    #[test]
    fn child2c_scheduler_commits_the_concrete_v11_lse_bgc_soil_owner_candidate() {
        let (shadow, fixture) = v10_shadow_fixture();
        let base_interval = day_input(&fixture).intervals.remove(0);
        let migrated =
            migrate_v10_runtime_to_v11(&shadow.vegetation_configuration, &shadow.vegetation_state)
                .expect("migration");
        let v11_owners = initial_v11_owners(&shadow, &migrated.state);
        let owner_set = CompleteOwnerSet::new(
            v11_owners
                .iter()
                .map(|(owner_id, owner)| (owner_id.clone(), owner.state_bytes.clone()))
                .collect(),
        )
        .expect("complete V11 owner set");
        let clock_owners = v11_owners
            .values()
            .map(|owner| owner.to_owner_state().expect("clock owner"))
            .collect::<Vec<_>>();
        let (parent_id, slab) = accepted_v11_slab(&clock_owners, 1_800_000_000_000);
        let parent = V11ParentTransaction::new_with_complete_owners(
            &migrated.configuration,
            &migrated.state,
            parent_id,
            ModelTimeNs::new(0),
            v11_owners,
        )
        .expect("V11 parent");
        let interval = segment_interval(&base_interval, 1_800_000_000_000, 41, 0.0);
        let stack = DirectV11RealConsumerStack::new(&shadow, &interval, 0, 0);
        let mut owner_executor =
            crate::v11_vegetation_consumer::DirectV11SnowStage3OwnerExecutor::new(
                migrated.configuration,
                parent,
                slab,
                stack,
            );
        let mut frame = shadow.hydrology_frame().clone();
        let mut runtime = SnowStage3HandoffRuntime::new(ModelTimeNs::new(0), owner_set.clone())
            .expect("Child 2C runtime");
        let request = SnowStage3TerminalHandoffRequest {
            carrier: child2c_carrier(),
            event: child2c_event(1_800_000_000_000),
            beginning_owners: owner_set.clone(),
            ending_owners: owner_set.clone(),
            owner_execution: SnowStage3OwnerExecutionReceipt::from_owner_set(
                "test-placeholder",
                owner_set,
            )
            .expect("placeholder owner receipt"),
            retained_liquid_kg_m2: 0.7,
            snow_support_rain_kg_m2: 0.2,
            terminal_melt_kg_m2: 0.5,
            terminal_refreeze_kg_m2: 0.1,
            continuation: SnowFreeContinuationInput {
                duration_ns: ModelTimeNs::new(1_800_000_000_000),
                terminal_liquid_kg_m2: 1.3,
                post_event_contains_snow_operands: false,
            },
        };
        let mut pending = Some(request);
        DirectFrameExecutor::new(DirectExecutorMode::ProductionDirect)
            .run_publication_stream_with_snow_stage3_terminal_handoff_and_owner_executor(
                &mut frame,
                DirectPublicationRunMetadata {
                    run_name: "child-2c-real-owner-test".to_owned(),
                    runtime_selection: "default-off-child-2c".to_owned(),
                    output_policy: "test".to_owned(),
                },
                |_frame, _day, _lane| Ok(production_day_input()),
                |_lane, _day, _input, _day_frame| Ok(pending.take()),
                |_row, _day_frame| Ok(()),
                &mut runtime,
                &mut owner_executor,
            )
            .expect("concrete Child 2C owner endpoint");

        assert_eq!(
            runtime.accepted_cursor_ns(),
            ModelTimeNs::new(1_800_000_000_000)
        );
        let committed = owner_executor
            .committed_shadow()
            .expect("concrete owner candidate committed");
        assert_eq!(committed.inner.lse_state.last_accepted_transaction_id,
            Some(TransactionId(40)));
        assert_eq!(committed.inner.soil_thermal.last_accepted_transaction_id,
            Some(TransactionId(40)));
        assert_eq!(committed.inner.biogeochemistry.last_transaction_id, 40);
    }

    #[test]
    fn v11_rejected_duration_attempt_leaves_parent_and_live_stack_unchanged() {
        let (shadow, fixture) = v10_shadow_fixture();
        let base_interval = day_input(&fixture).intervals.remove(0);
        let migrated =
            migrate_v10_runtime_to_v11(&shadow.vegetation_configuration, &shadow.vegetation_state)
                .expect("migration");
        let owners = initial_v11_owners(&shadow, &migrated.state);
        let clock_owners = owners
            .values()
            .map(|owner| owner.to_owner_state().expect("clock owner"))
            .collect::<Vec<_>>();
        let (parent_id, slab) = accepted_v11_slab(&clock_owners, 599_999_999);
        let parent = V11ParentTransaction::new_with_complete_owners(
            &migrated.configuration,
            &migrated.state,
            parent_id,
            ModelTimeNs::new(0),
            owners,
        )
        .expect("parent");
        let interval = segment_interval(&base_interval, 599_999_999, 41, 0.0);
        let stack = DirectV11RealConsumerStack::new(&shadow, &interval, 0, 0);
        let mut executor = crate::v11_vegetation_consumer::DirectV11VegetationExecutor { stack };
        let before = parent.staged_state().clone();
        let error = execute_direct_v11_segment(&migrated.configuration, &parent, &slab, &mut executor)
            .expect_err("one tick below the LSE minimum must be rejected");
        assert!(matches!(
            error,
            V11ExecutionError::Executor(DirectV11RealConsumerError::Runtime(
                DirectV10RealConsumerError::LandSurface(
                    LandSurfaceEnergyError::SupportBelowMinimum {
                        requested_ns: 599_999_999,
                        minimum_ns: 600_000_000,
                    }
                )
            ))
        ));
        assert_eq!(parent.staged_state(), &before);
        assert!(executor.stack.take_staged_ending().is_none());
        assert_eq!(executor.stack.beginning, shadow);
    }

    #[test]
    fn v11_actual_stack_accepts_sequential_unequal_supports_once_per_parent() {
        let (shadow, fixture) = v10_shadow_fixture();
        let interval = day_input(&fixture).intervals.remove(0);
        for durations in [
            vec![600_000_000_000, 1_200_000_000_000],
            vec![1_200_000_000_000, 600_000_000_000],
            vec![300_000_000_000, 500_000_000_000, 1_000_000_000_000],
        ] {
            let candidate = run_actual_v11_segments(
                &shadow,
                &interval,
                &durations,
                &vec![0.0; durations.len()],
            );
            assert_eq!(candidate.accepted_segments.len(), durations.len());
            assert_eq!(candidate.ending_complete_owners.len(), 7);
            assert_eq!(
                candidate.ending_state.last_parent_transaction_id,
                shadow.vegetation_state.0.last_transaction_id + 1
            );
        }
    }

    #[test]
    fn v11_open_first_vegetated_second_executes_complete_bgc_consumer_atomically() {
        let mut fixture = two_ofe_routed_endpoint_fixture();
        for tile in &mut fixture.lse_configuration.ofes[0].tiles {
            tile.vegetation_tile_id = openwepp_kernel_contract::TileId::try_new(format!(
                "upper-open-{}",
                tile.tile_id.as_str()
            ))
            .expect("upper open vegetation tile");
        }
        let mut surface_records = fixture.surface_configuration.records.clone();
        for record in surface_records
            .iter_mut()
            .filter(|record| record.key.ofe_id.as_str() == "ofe-1")
        {
            record.ground_ingress_mode =
                crate::direct_runtime::DirectGroundIngressMode::OpenRawPrecipitation;
        }
        fixture.surface_configuration =
            crate::direct_runtime::DirectSurfaceLiquidConfiguration::new(
                fixture.surface_configuration.owner_id.clone(),
                fixture.surface_configuration.run_id,
                fixture.surface_configuration.ofe_topology.clone(),
                fixture.surface_configuration.ofe_bindings.clone(),
                surface_records,
            )
            .expect("open-first surface configuration");
        let mut rebound_frame = fixture.hydrology.beginning_frame().clone();
        let surface = rebound_frame
            .surface_liquid_shadow
            .as_mut()
            .expect("surface owner");
        surface
            .configuration_sha256
            .clone_from(&fixture.surface_configuration.configuration_sha256);
        surface.state_sha256 = surface.recomputed_sha256().expect("surface digest");
        fixture.hydrology = crate::vegetation_real_hydrology_shadow::RealHydrologyShadowAdapter::try_from_day_start(
            &rebound_frame,
            fixture.hydrology.day_index(),
            fixture.hydrology.transaction_id(),
            fixture.hydrology.interval_s(),
            fixture.hydrology.hydrology_owner_id().clone(),
            fixture.hydrology.layer_maps(),
        )
        .expect("rebound open-first surface owner");
        let lower = fixture.lse_configuration.ofes[1]
            .tiles
            .iter()
            .find(|tile| tile.tile_id.as_str() == "lower-forest")
            .expect("lower vegetation tile");
        assert_ne!(lower.tile_id, lower.vegetation_tile_id);
        let (shadow, fixture) = v10_shadow_fixture_from(fixture);
        let mut interval = day_input(&fixture).intervals.remove(0);
        interval.wb14_parameters = ["ofe-1", "ofe-2"]
            .into_iter()
            .map(|ofe_id| DirectOfeWb14Parameters {
                ofe_id: OfeId::try_new(ofe_id).expect("OFE"),
                effective_conductivity_m_s: 1e-6,
                matric_potential_m: 0.1,
                infiltration_storage_capacity_m: 0.04,
            })
            .collect();
        let migrated =
            migrate_v10_runtime_to_v11(&shadow.vegetation_configuration, &shadow.vegetation_state)
                .expect("migration");
        let owners = initial_v11_owners(&shadow, &migrated.state);
        let owner_bytes = owners.clone();
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
        .expect("parent");
        let stack = DirectV11RealConsumerStack::new(&shadow, &interval, 0, 0);
        let mut executor = crate::v11_vegetation_consumer::DirectV11VegetationExecutor { stack };
        let injected = crate::v11_vegetation_consumer::execute_direct_v11_segment_with_post_bgc_fault(
            &migrated.configuration,
            &parent,
            &slab,
            &mut executor,
        );
        let injected_debug = format!("{injected:?}");
        assert!(matches!(
            injected,
            Err(V11ExecutionError::Executor(DirectV11RealConsumerError::Identity(
                "injected post-BGC-transition fault"
            )))
        ), "{injected_debug}");
        assert_eq!(parent.staged_resource_owners(), &owner_bytes);
        assert_eq!(parent.staged_state(), &migrated.state);
        assert_eq!(executor.stack.beginning, shadow);
        assert!(executor.stack.take_staged_ending().is_none());
        let stack = DirectV11RealConsumerStack::new(&shadow, &interval, 0, 0);
        let mut executor = crate::v11_vegetation_consumer::DirectV11VegetationExecutor { stack };
        let candidate = execute_direct_v11_segment(&migrated.configuration, &parent, &slab, &mut executor)
            .expect("open-first/vegetated-second real consumer");
        assert_eq!(candidate.ending_resource_owners.len(), 7);
        let bgc_debits = candidate
            .resource_debits
            .iter()
            .filter(|debit| debit.owner_id == "bgc" && debit.final_use > 0.0)
            .collect::<Vec<_>>();
        assert!(!bgc_debits.is_empty(), "fixture must exercise mineral-N use");
        assert!(bgc_debits.iter().all(|debit| {
            debit.ofe_id == "ofe-2"
                && debit.tile_id == "stratum_scoped"
                && debit.amount_basis == "kg_n_m2"
        }));
        for transition in candidate
            .shared_resource_transitions
            .iter()
            .filter(|transition| transition.shared_resource_key.owner_id == "bgc")
        {
            let ending_bgc: openwepp_biogeochemistry::BiogeochemistryState = serde_json::from_slice(
                &candidate.ending_resource_owners["bgc"].state_bytes,
            )
            .expect("decoded ending BGC owner");
            let beginning_layer = shadow
                .inner
                .biogeochemistry()
                .layers
                .get(&transition.shared_resource_key.layer_id)
                .expect("beginning BGC layer");
            let ending_layer = ending_bgc
                .layers
                .get(&transition.shared_resource_key.layer_id)
                .expect("ending BGC layer");
            let (beginning_pool, ending_pool) = match transition.shared_resource_key.resource {
                openwepp_vegetation::v11::V11SharedResourceKind::Ammonium => {
                    (beginning_layer.ammonium_n, ending_layer.ammonium_n)
                }
                openwepp_vegetation::v11::V11SharedResourceKind::Nitrate => {
                    (beginning_layer.nitrate_n, ending_layer.nitrate_n)
                }
                _ => panic!("BGC transition must be mineral nitrogen"),
            };
            let used = transition.debit_receipt_ids.iter().fold(0.0_f64, |sum, id| {
                sum + candidate
                    .resource_debits
                    .iter()
                    .find(|debit| debit.receipt_id == *id)
                    .expect("linked BGC debit")
                    .final_use
            });
            assert_eq!(
                (beginning_pool - used).to_bits(),
                ending_pool.to_bits()
            );
            assert_eq!(transition.beginning_amount.to_bits(), beginning_pool.to_bits());
            assert_eq!(transition.ending_amount.to_bits(), ending_pool.to_bits());
        }
        assert_eq!(parent.staged_resource_owners(), &owner_bytes);
        assert_eq!(parent.staged_state(), &migrated.state);
        assert_eq!(executor.stack.beginning, shadow);

        accept_direct_v11_segment(
            &mut parent,
            &migrated.configuration,
            candidate,
            &executor.stack.beginning,
        )
        .expect("accept endpoint candidate");
        let checkpoint = parent.checkpoint();
        let bgc_scope = v11_bgc_debit_scope(
            &migrated.configuration.imported_v10,
            &executor.stack.beginning.inner.lse_configuration,
        )
        .expect("checkpoint BGC scope");
        V11ParentTransaction::restore_with_bgc_scope(
            &migrated.configuration,
            checkpoint.clone(),
            Some(&bgc_scope),
        )
        .expect("positive scoped checkpoint restore");
        let assert_checkpoint_poison = |mutate: fn(&mut V11ResourceDebit)| {
            let mut poison = checkpoint.clone();
            let segment = &mut poison.accepted_segments[0];
            let debit = segment
                .resource_debits
                .iter_mut()
                .find(|debit| debit.owner_id == "bgc")
                .expect("checkpoint BGC debit");
            let old_id = debit.receipt_id;
            mutate(debit);
            *debit = V11ResourceDebit::new(debit.clone()).expect("resealed checkpoint debit");
            let new_id = debit.receipt_id;
            let transition = segment
                .shared_resource_transitions
                .iter_mut()
                .find(|transition| transition.debit_receipt_ids.contains(&old_id))
                .expect("checkpoint BGC transition");
            for id in &mut transition.debit_receipt_ids {
                if *id == old_id {
                    *id = new_id;
                }
            }
            *transition = V11SharedResourceOwnerTransition::new(transition.clone())
                .expect("resealed checkpoint transition");
            for candidate in &mut segment.complete_owner_candidates {
                for component in &mut candidate.components {
                    for id in &mut component.debit_receipt_ids {
                        if *id == old_id {
                            *id = new_id;
                        }
                    }
                }
            }
            let bytes = serde_json::to_vec(&poison).expect("serialized checkpoint poison");
            let decoded = serde_json::from_slice(&bytes).expect("decoded checkpoint poison");
            assert!(V11ParentTransaction::restore_with_bgc_scope(
                &migrated.configuration,
                decoded,
                Some(&bgc_scope),
            )
            .is_err());
        };
        assert_checkpoint_poison(|debit| debit.tile_id = "occupancy_scoped".into());
        assert_checkpoint_poison(|debit| debit.occupancy_id = "unknown-stratum".into());
        assert_checkpoint_poison(|debit| debit.source_id = "no3".into());
        assert_checkpoint_poison(|debit| debit.layer_id = "wrong-layer".into());
        assert_checkpoint_poison(|debit| debit.amount_basis = "kg_m2".into());
    }

    #[test]
    fn v11_actual_stack_is_forcing_order_observable() {
        let (shadow, fixture) = v10_shadow_fixture();
        let interval = day_input(&fixture).intervals.remove(0);
        let warm_then_cool = run_actual_v11_segments(
            &shadow,
            &interval,
            &[600_000_000_000, 1_200_000_000_000],
            &[4.0, -4.0],
        );
        let cool_then_warm = run_actual_v11_segments(
            &shadow,
            &interval,
            &[600_000_000_000, 1_200_000_000_000],
            &[-4.0, 4.0],
        );
        assert_ne!(
            warm_then_cool.ending_state.state_sha256,
            cool_then_warm.ending_state.state_sha256
        );
    }

    #[test]
    fn coupled_time_one_nanosecond_support_is_structurally_admitted() {
        let support =
            TimeSupport::new(ModelTimeNs::new(0), ModelTimeNs::new(1)).expect("one tick support");
        assert_eq!(support.duration_ns(), 1);
        assert_eq!(
            support.duration_s_bits(),
            f64::from_bits(support.duration_s_bits()).to_bits()
        );
    }

    #[test]
    fn v11_actual_stack_accepts_the_declared_lse_minimum_support() {
        let (shadow, fixture) = v10_shadow_fixture();
        let interval = day_input(&fixture).intervals.remove(0);
        let candidate = run_actual_v11_segments(&shadow, &interval, &[600_000_000], &[0.0]);
        assert_eq!(candidate.accepted_segments.len(), 1);
        assert_eq!(candidate.ending_complete_owners.len(), 7);
    }

    #[test]
    fn v11_actual_stack_rejects_one_tick_below_lse_minimum_before_newton() {
        let (shadow, fixture) = v10_shadow_fixture();
        let interval = day_input(&fixture).intervals.remove(0);
        let migrated =
            migrate_v10_runtime_to_v11(&shadow.vegetation_configuration, &shadow.vegetation_state)
                .expect("migration");
        let owners = initial_v11_owners(&shadow, &migrated.state);
        let clock_owners = owners
            .values()
            .map(|owner| owner.to_owner_state().expect("clock owner"))
            .collect::<Vec<_>>();
        let (parent_id, slab) = accepted_v11_slab(&clock_owners, 599_999_999);
        let parent = V11ParentTransaction::new_with_complete_owners(
            &migrated.configuration,
            &migrated.state,
            parent_id,
            ModelTimeNs::new(0),
            owners,
        )
        .expect("parent");
        let segmented = segment_interval(&interval, 599_999_999, 41, 0.0);
        let stack = DirectV11RealConsumerStack::new(&shadow, &segmented, 0, 0);
        let mut executor = crate::v11_vegetation_consumer::DirectV11VegetationExecutor { stack };
        let error = execute_direct_v11_segment(&migrated.configuration, &parent, &slab, &mut executor)
            .expect_err("one tick below the LSE minimum must be rejected");
        assert!(matches!(
            error,
            V11ExecutionError::Executor(DirectV11RealConsumerError::Runtime(
                DirectV10RealConsumerError::LandSurface(
                    LandSurfaceEnergyError::SupportBelowMinimum {
                        requested_ns: 599_999_999,
                        minimum_ns: 600_000_000,
                    }
                )
            ))
        ));
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
                std::slice::from_ref(&day_frame),
                std::slice::from_ref(&production_input),
                prepared,
                template,
            )
            .expect("real Child4 consumes repository-derived provider day");
        assert_eq!(shadow.inner.accepted_interval_count(), 48);
    }

    #[test]
    fn v10_ignores_caller_root_hydraulic_template_operands() {
        let (mut poisoned, fixture) = v10_shadow_fixture();
        let beginning = poisoned.clone();
        let mut canonical = beginning.clone();
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
        let canonical_prepared = prepare(&canonical, &template);
        let production = fixture.hydrology.beginning_frame().clone();
        let production_input = production_day_input();
        let day_frame = projected_day(&production, &production_input);
        poisoned
            .execute_prepared_gsi_day(
                &production,
                std::slice::from_ref(&day_frame),
                std::slice::from_ref(&production_input),
                poisoned_prepared,
                poisoned_template,
            )
            .expect("caller hydraulic template fields are non-authoritative");
        canonical
            .execute_prepared_gsi_day(
                &production,
                &[day_frame],
                &[production_input],
                canonical_prepared,
                template,
            )
            .expect("canonical live-owner execution");
        assert_eq!(poisoned.inner.accepted_interval_count(), 48);
        assert_eq!(poisoned, canonical);
    }

    #[test]
    fn v10_canonicalizes_dead_global_ground_scalars_for_heterogeneous_tiles() {
        let (shadow, fixture) = v10_shadow_fixture();
        let template = day_input(&fixture);
        let provider = &template.intervals[0].vegetation_forcing;
        let canonical = project_live_vegetation_forcing(
            provider,
            &fixture.hydrology,
            shadow.inner.soil_thermal(),
            shadow.inner.root_zone_hydraulic_configuration.as_ref(),
            &shadow.inner.surface_configuration,
            &shadow.inner.lse_configuration,
            &shadow.inner.vegetation_configuration,
            shadow.inner.vegetation_state(),
            project_v9_runtime_to_v8(
                &shadow.inner.vegetation_configuration,
                shadow.inner.vegetation_state(),
            )
            .unwrap()
            .0
            .configuration_sha256,
            Sha256Digest::try_new("11".repeat(32)).unwrap(),
            TransactionId(1),
            0,
            0,
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
            &shadow.inner.surface_configuration,
            &shadow.inner.lse_configuration,
            &shadow.inner.vegetation_configuration,
            shadow.inner.vegetation_state(),
            project_v9_runtime_to_v8(
                &shadow.inner.vegetation_configuration,
                shadow.inner.vegetation_state(),
            )
            .unwrap()
            .0
            .configuration_sha256,
            Sha256Digest::try_new("11".repeat(32)).unwrap(),
            TransactionId(1),
            0,
            0,
        )
        .expect("caller ground forcing is not authoritative");
        assert_eq!(
            projected.0.ground_albedo_vis.to_bits(),
            canonical.0.ground_albedo_vis.to_bits()
        );
        assert_eq!(
            projected.0.ground_albedo_nir.to_bits(),
            canonical.0.ground_albedo_nir.to_bits()
        );
        assert_eq!(
            projected.0.longwave_up_w_m2.to_bits(),
            canonical.0.longwave_up_w_m2.to_bits()
        );
        assert_eq!(canonical.0.ground_albedo_vis.to_bits(), 0.0_f64.to_bits());
        assert_eq!(canonical.0.ground_albedo_nir.to_bits(), 0.0_f64.to_bits());
        assert_eq!(canonical.0.longwave_up_w_m2.to_bits(), 0.0_f64.to_bits());
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
                "runon requires an accepted routing publication owner"
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

    fn top_boundary_credit(fixture: &EndpointFixture) -> SoilThermalTopBoundaryCreditV1 {
        SoilThermalTopBoundaryCreditV1 {
            lane_id: 7,
            ofe_id: fixture.thermal.ofes[0].ofe_id.clone(),
            first_layer_id: fixture.thermal.ofes[0].ordered_layers[0].layer_id.clone(),
            beginning_owner_id: fixture.thermal.owner_id.clone(),
            beginning_configuration_sha256: fixture.thermal.configuration_sha256.clone(),
            beginning_state_sha256: fixture.thermal.state_sha256.clone(),
            support_start_ns: 0,
            support_end_ns: 1_800_000_000_000,
            accepted_positive_downward_j_m2_ofe_ground: 125.0,
            soil_thermal_credit_j_m2_ofe_ground: 125.0,
            snow_soil_heat_receipt_sha256: Sha256Digest::try_new("a".repeat(64)).expect("receipt"),
        }
    }

    #[test]
    fn ofe_top_boundary_credit_is_applied_once_and_is_tile_order_independent() {
        let (_, fixture) = shadow_fixture();
        let candidates = soil_candidates(&fixture);
        let credit = top_boundary_credit(&fixture);
        let beginning = fixture.thermal.ofes[0].ordered_layers[0].enthalpy_j_m2_ofe_ground;
        let tile_credit = candidates.iter()
            .map(|candidate| candidate.layers[0].ground_heat_credit_j_m2_ofe_ground).sum::<f64>();
        let accepted = aggregate_soil_thermal_ending_with_top_boundary_credits(
            &fixture.thermal, &fixture.lse_configuration, TransactionId(41), &candidates,
            std::slice::from_ref(&credit),
        ).expect("OFE credit");
        assert_eq!(accepted.ending.ofes[0].ordered_layers[0].enthalpy_j_m2_ofe_ground.to_bits(),
            (beginning + tile_credit + 125.0).to_bits());
        let mut reversed = candidates;
        reversed.reverse();
        assert_eq!(aggregate_soil_thermal_ending_with_top_boundary_credits(
            &fixture.thermal, &fixture.lse_configuration, TransactionId(41), &reversed, &[credit],
        ).expect("tile-order-independent OFE credit"), accepted);
    }

    #[test]
    fn ofe_top_boundary_credit_rejects_identity_duplicate_sign_and_support_poisons() {
        let (_, fixture) = shadow_fixture();
        let candidates = soil_candidates(&fixture);
        let valid = top_boundary_credit(&fixture);
        let beginning = fixture.thermal.clone();
        let reject = |credits: &[SoilThermalTopBoundaryCreditV1]| {
            aggregate_soil_thermal_ending_with_top_boundary_credits(
                &fixture.thermal, &fixture.lse_configuration, TransactionId(41), &candidates, credits,
            ).is_err()
        };
        let mut wrong_ofe = valid.clone();
        wrong_ofe.ofe_id = OfeId::try_new("wrong-ofe").expect("OFE");
        assert!(reject(&[wrong_ofe]));
        let mut wrong_layer = valid.clone();
        wrong_layer.first_layer_id = SoilLayerId::try_new("wrong-layer").expect("layer");
        assert!(reject(&[wrong_layer]));
        let mut wrong_owner = valid.clone();
        wrong_owner.beginning_owner_id = ResourceOwnerId::try_new("wrong-owner").expect("owner");
        assert!(reject(&[wrong_owner]));
        let mut wrong_sign = valid.clone();
        wrong_sign.soil_thermal_credit_j_m2_ofe_ground = -125.0;
        assert!(reject(&[wrong_sign]));
        let mut wrong_support = valid.clone();
        wrong_support.support_end_ns = wrong_support.support_start_ns;
        assert!(reject(&[wrong_support]));
        assert!(reject(&[valid.clone(), valid]));
        assert_eq!(fixture.thermal, beginning);
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
    #[allow(clippy::too_many_lines)]
    fn prepared_provider_chain_accepts_two_days_and_rejects_sequence_poisons() {
        let (shadow, fixture) = v10_shadow_fixture();
        let template = day_input(&fixture);
        let source = "5.30\n1 0 0\nTEST STATION 1500\nDAY MON YEAR PRCP STMDUR TIMEP IP TMAX TMIN RAD VWIND WIND TDPT\n41.1 -120.0 1225.0 30 2000 1 CLIGEN 5.30 --seed 123\nMONTHLY MAX TEMP HEADER\n1 2 3 4 5 6 7 8 9 10 11 12\nMONTHLY MIN TEMP HEADER\n-5 -4 -3 -2 -1 0 1 2 3 4 5 6\nMONTHLY RAD HEADER\n100 101 102 103 104 105 106 107 108 109 110 111\nMONTHLY RAIN HEADER\n10 11 12 13 14 15 16 17 18 19 20 21\nDAILY HEADER\nDAILY UNITS\n20 6 2000 0.0 0.0 0.0 0.0 28.0 22.0 0.0 2.5 180.0 20.0\n21 6 2000 0.0 0.0 0.0 0.0 29.0 23.0 0.0 2.5 180.0 21.0\n22 6 2000 0.0 0.0 0.0 0.0 30.0 24.0 0.0 2.5 180.0 22.0\n";
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
        let initial_gsi = shadow.gsi_state().clone();
        let initial_cursor = shadow.provider_cursor().clone();
        let day_zero = request
            .prepare_snow_free_gsi_day_from_repository(
                0,
                &configuration,
                shadow.gsi_owner_configuration(),
                &initial_gsi,
                &initial_cursor,
            )
            .expect("day zero provider capability");
        let lane_id = fixture.hydrology.beginning_frame().lanes[0].lane_id;
        let bound_day_zero = PreparedStage3V11DayV1::bind_provider_day(
            &day_zero,
            0,
            attachment_supports(&day_zero, &template.intervals[0], lane_id, 0),
        )
        .expect("day zero Stage-3/V11 provider binding");
        for poison in [
            PreparedStage3V11SupportV1::poison_base_air_temperature,
            PreparedStage3V11SupportV1::poison_base_wind,
            |support: &mut PreparedStage3V11SupportV1| {
                support.poison_covered_atmosphere(false);
            },
            |support: &mut PreparedStage3V11SupportV1| {
                support.poison_covered_atmosphere(true);
            },
            PreparedStage3V11SupportV1::poison_stage3_pressure,
            PreparedStage3V11SupportV1::poison_stage3_dewpoint,
        ] {
            let mut poisoned = attachment_supports(&day_zero, &template.intervals[0], lane_id, 0);
            poison(&mut poisoned[0]);
            assert!(PreparedStage3V11DayV1::bind_provider_day(&day_zero, 0, poisoned).is_err());
        }
        let open_requests = attachment_supports(&day_zero, &template.intervals[0], lane_id, 0)
            .into_iter()
            .enumerate()
            .map(|(interval_index, support)| {
                let interval = &day_zero.forcing_receipts().receipts()[0].intervals
                    [interval_index];
                support
                    .with_provider_open_snow_destination((
                        OfeId::try_new(interval.ofe_id.clone()).expect("open request OFE"),
                        TileId::try_new(interval.tile_id.clone()).expect("open request tile"),
                    ))
                    .expect("provider-owned open request")
            })
            .collect();
        let bound_open =
            PreparedStage3V11DayV1::bind_provider_day(&day_zero, 0, open_requests)
                .expect("provider-owned open atmosphere seal");
        let first_open = bound_open.supports()[0]
            .snow_surface_forcing_by_destination()
            .values()
            .next()
            .expect("sealed open forcing");
        let SealedStage3TileBoundaryForcingV1::OpenSnow(first_open) = first_open else {
            panic!("provider request must seal open snow");
        };
        let first_provider = &day_zero.forcing_receipts().receipts()[0].intervals[0];
        assert_eq!(
            first_open.exposure.source_wind_provider_sha256,
            digest_from_receipt(&first_provider.provider_definition_sha256)
        );
        assert_eq!(
            first_open.exposure.projection_model_definition_sha256,
            digest_bytes(b"OPENWEPP_STAGE3_RAW_WIND_IDENTITY_PROJECTION_V1")
        );
        assert_eq!(
            first_open.exposure.raw_or_projected_wind_m_s.to_bits(),
            first_provider.wind_m_s.to_bits()
        );
        assert_eq!(
            bound_open.supports()[0]
                .atmospheric_receipt_by_destination()
                .values()
                .next()
                .expect("provider atmosphere")
                .raw_wind_m_s
                .to_bits(),
            first_provider.wind_m_s.to_bits()
        );
        let rainy_source = "5.30\n1 0 0\nTEST STATION 1500\nDAY MON YEAR PRCP STMDUR TIMEP IP TMAX TMIN RAD VWIND WIND TDPT\n41.1 -120.0 1225.0 30 2000 1 CLIGEN 5.30 --seed 123\nMONTHLY MAX TEMP HEADER\n1 2 3 4 5 6 7 8 9 10 11 12\nMONTHLY MIN TEMP HEADER\n-5 -4 -3 -2 -1 0 1 2 3 4 5 6\nMONTHLY RAD HEADER\n100 101 102 103 104 105 106 107 108 109 110 111\nMONTHLY RAIN HEADER\n10 11 12 13 14 15 16 17 18 19 20 21\nDAILY HEADER\nDAILY UNITS\n20 6 2000 10.0 1.0 0.5 1.0 28.0 22.0 0.0 2.5 180.0 20.0\n";
        let rainy_climate =
            parse_climate_from_str(rainy_source, ParserMode::Strict).expect("rainy climate");
        let rainy_request =
            build_hillslope_climate_runtime_request(&rainy_climate).expect("rainy request");
        let rainy_day = rainy_request
            .prepare_snow_free_gsi_day_from_repository(
                0,
                &configuration,
                shadow.gsi_owner_configuration(),
                &initial_gsi,
                &initial_cursor,
            )
            .expect("rainy provider capability");
        assert!(rainy_day
            .forcing_receipts()
            .receipts()
            .iter()
            .flat_map(|day| day.intervals.iter())
            .any(|interval| !interval.precipitation_parcels.is_empty()));
        let rainy_open_requests = attachment_supports(
            &rainy_day,
            &template.intervals[0],
            lane_id,
            0,
        )
        .into_iter()
        .enumerate()
        .map(|(interval_index, support)| {
            let interval = &rainy_day.forcing_receipts().receipts()[0].intervals
                [interval_index];
            support
                .with_provider_open_snow_destination((
                    OfeId::try_new(interval.ofe_id.clone()).expect("rain poison OFE"),
                    TileId::try_new(interval.tile_id.clone()).expect("rain poison tile"),
                ))
                .expect("rain poison open request")
        })
        .collect();
        let gsi_before_rain_poison = initial_gsi.clone();
        let cursor_before_rain_poison = initial_cursor.clone();
        assert!(PreparedStage3V11DayV1::bind_provider_day(
            &rainy_day,
            0,
            rainy_open_requests,
        )
        .is_err());
        assert_eq!(initial_gsi, gsi_before_rain_poison);
        assert_eq!(initial_cursor, cursor_before_rain_poison);
        assert_eq!(bound_day_zero.supports()[0].support().start_ns().get(), 0);
        assert_eq!(
            bound_day_zero.supports()[47].support().end_ns().get(),
            86_400_000_000_000
        );
        let day_zero_replay = day_zero.clone();
        let mut gsi_after_day_zero = initial_gsi.clone();
        let mut cursor_after_day_zero = initial_cursor.clone();
        day_zero
            .commit(&mut gsi_after_day_zero, &mut cursor_after_day_zero)
            .expect("day zero provider commit");

        let day_one = request
            .prepare_snow_free_gsi_day_from_repository(
                1,
                &configuration,
                shadow.gsi_owner_configuration(),
                &gsi_after_day_zero,
                &cursor_after_day_zero,
            )
            .expect("day one provider capability");
        let bound_day_one = PreparedStage3V11DayV1::bind_provider_day(
            &day_one,
            1,
            attachment_supports(&day_one, &template.intervals[0], lane_id, 1),
        )
        .expect("day one Stage-3/V11 provider binding");
        assert_eq!(
            bound_day_one.supports()[0].support().start_ns().get(),
            86_400_000_000_000
        );
        assert_eq!(
            bound_day_one.supports()[47].support().end_ns().get(),
            172_800_000_000_000
        );
        assert_ne!(
            bound_day_zero.accepted_gsi_receipt(),
            bound_day_one.accepted_gsi_receipt(),
            "sequential days must carry distinct GSI receipts"
        );
        assert!(PreparedStage3V11DayV1::bind_provider_day(
            &day_one,
            1,
            attachment_supports(&day_zero_replay, &template.intervals[0], lane_id, 0),
        )
        .is_err());
        assert!(PreparedStage3V11DayV1::bind_provider_day(
            &day_zero_replay,
            0,
            attachment_supports_with_start_offset(
                &day_zero_replay,
                &template.intervals[0],
                lane_id,
                0,
                1,
            ),
        )
        .is_err());
        let day_one_replay = day_one.clone();
        let mut gsi_after_day_one = gsi_after_day_zero.clone();
        let mut cursor_after_day_one = cursor_after_day_zero.clone();
        day_one
            .commit(&mut gsi_after_day_one, &mut cursor_after_day_one)
            .expect("day one provider commit");
        cursor_after_day_one
            .validate_for_configuration(&configuration, 2)
            .expect("provider cursor advances to day two");

        assert!(day_zero_replay
            .commit(&mut gsi_after_day_one.clone(), &mut cursor_after_day_one.clone())
            .is_err());
        assert!(request
            .prepare_snow_free_gsi_day_from_repository(
                2,
                &configuration,
                shadow.gsi_owner_configuration(),
                &initial_gsi,
                &initial_cursor,
            )
            .is_err());

        let mut substituted_gsi = initial_gsi;
        let mut correct_cursor = cursor_after_day_zero;
        assert!(day_one_replay
            .clone()
            .commit(&mut substituted_gsi, &mut correct_cursor)
            .is_err());

        let mut correct_gsi = gsi_after_day_zero;
        let mut rewound_cursor = initial_cursor;
        assert!(day_one_replay
            .commit(&mut correct_gsi, &mut rewound_cursor)
            .is_err());
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

include!("v9_real_consumer_shadow_tests_tail.rs");
}
