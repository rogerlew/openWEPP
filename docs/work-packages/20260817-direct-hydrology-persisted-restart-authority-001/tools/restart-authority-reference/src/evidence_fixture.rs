use crate::{
    AcceptedIntervalCount, BiogeochemistryStateRestartV1, CompleteCommittedOwnerStateV1,
    DirectGsiDailyReceiptRestartV1, DirectGsiOwnerConfigurationRestartV1,
    DirectGsiOwnerStateRestartV1, DirectHydrologyRestartV1,
    DirectSurfaceLiquidConfigurationRestartV1, DirectV10CheckpointPhaseV1,
    DirectV10RealConsumerCheckpointV1, LseV2StateRestartV1, ScientificOwnerStateSetV1, Sha256Hex,
    SnowFreeHalfHourProviderCursorRestartV1, SnowFreeHalfHourStaticConfigurationRestartV1,
    SoilThermalStateRestartV1, VegetationV10StateRestartV1, WireDayIndex, canonical_sha256,
};
use openwepp_hillslope_orchestrator::DirectOfeWb14Parameters;
use openwepp_hillslope_orchestrator::land_surface_energy_shadow::{
    EndpointFixture, endpoint_fixture,
};
use openwepp_hillslope_orchestrator::runtime_inputs::{
    DirectGsiOwnerConfigurationV1, PreparedSnowFreeGsiDayV1, SnowFreeHalfHourDestination,
    SnowFreeHalfHourProviderCursor, SnowFreeHalfHourStaticConfiguration,
    build_hillslope_climate_runtime_request,
};
use openwepp_hillslope_orchestrator::v9_real_consumer_shadow::{
    DirectV9ShadowIntervalInput, DirectV10RealConsumerShadow, DirectV10ShadowDayInput,
    restart_authority_wb14_parameter_sha256,
};
use openwepp_input_contract::parsers::climate::{ParserMode, parse_climate_from_str};
use openwepp_kernel_contract::{ResourceOwnerId, TransactionId};
use openwepp_land_surface_energy::{
    LandSurfaceEnergyV2State, OfeId, Sha256Digest, V2_MODEL_DEFINITION_SHA256, V2_MODEL_VERSION,
    V2_VEGETATION_MODEL_DEFINITION_SHA256, V2_VEGETATION_MODEL_VERSION,
};
use openwepp_plant_phenology::{GsiParameters, GsiState};
use openwepp_vegetation::{
    V9_MODEL_SHA256, V9CoupledOwnedState, V10_MODEL_SHA256, V10CoupledOwnedState,
};
use sha2::{Digest, Sha256};

pub struct RestartAuthorityEvidenceFixture {
    pub shadow: DirectV10RealConsumerShadow,
    pub endpoint: EndpointFixture,
}

pub struct RestartAuthorityOwnerFixture {
    pub runtime: RestartAuthorityEvidenceFixture,
    pub committed: CompleteCommittedOwnerStateV1,
    pub phase_plan_sha256: Sha256Hex,
    pub day_input_digests: Vec<Sha256Hex>,
    pub day_inputs: Vec<Vec<openwepp_hillslope_orchestrator::DirectDayConstructorInputs>>,
}

pub struct RestartAuthorityPreparedDayFixture {
    pub owners: RestartAuthorityOwnerFixture,
    pub gsi_receipt: DirectGsiDailyReceiptRestartV1,
    pub ending_gsi_state: DirectGsiOwnerStateRestartV1,
    pub ending_cursor: SnowFreeHalfHourProviderCursorRestartV1,
    pub forcing_receipts: Vec<crate::SnowFreeHalfHourDayReceiptRestartV1>,
    pub prepared: PreparedSnowFreeGsiDayV1,
    pub template: DirectV10ShadowDayInput,
}

pub fn restart_authority_identities(
    committed: &CompleteCommittedOwnerStateV1,
) -> (Sha256Hex, Sha256Hex) {
    let hydrology = &committed.scientific.direct_hydrology;
    let run = Sha256Hex::try_new(
        canonical_sha256(&(
            hydrology.run_id,
            hydrology.hillslope_id,
            hydrology.lane_count,
            hydrology.day_count,
        ))
        .unwrap(),
    )
    .unwrap();
    let topology = serde_json::json!({
        "ordered_lanes": hydrology.lanes.iter().map(|lane| serde_json::json!({
            "lane_id": lane.lane_id,
            "upstream_lane_id": lane.upstream_lane_id,
            "downstream_lane_id": lane.downstream_lane_id,
            "soil_layer_count": lane.subsurface_layers.len(),
        })).collect::<Vec<_>>(),
        "ordered_ofe_tiles": committed.static_forcing_configuration.destinations.iter().map(|destination| (
            &destination.ofe_id,
            &destination.tile_id,
            &destination.wb14_configuration_sha256,
        )).collect::<Vec<_>>(),
        "lse_tiles": committed.scientific.lse_v2.tiles.iter().map(|tile| (&tile.ofe_id, &tile.tile_id)).collect::<Vec<_>>(),
        "soil_thermal_layer_maps": committed.scientific.soil_thermal.ofes.iter().map(|ofe| (
            &ofe.ofe_id,
            ofe.ordered_layers.iter().map(|layer| &layer.layer_id).collect::<Vec<_>>(),
        )).collect::<Vec<_>>(),
    });
    (
        run,
        Sha256Hex::try_new(canonical_sha256(&topology).unwrap()).unwrap(),
    )
}

pub fn project_evidence_scientific_owners(
    shadow: &DirectV10RealConsumerShadow,
    phase_plan_sha256: &Sha256Hex,
    day_input_digests: &[Sha256Hex],
) -> ScientificOwnerStateSetV1 {
    ScientificOwnerStateSetV1 {
        vegetation_v10: VegetationV10StateRestartV1::project(
            shadow.vegetation_state(),
            shadow.restart_authority_vegetation_configuration(),
        )
        .unwrap(),
        lse_v2: LseV2StateRestartV1::project(
            shadow.lse_state(),
            shadow.restart_authority_lse_configuration(),
        )
        .unwrap(),
        direct_hydrology: DirectHydrologyRestartV1::project(
            shadow.restart_authority_hydrology_frame(),
            phase_plan_sha256.clone(),
            day_input_digests,
        )
        .unwrap(),
        soil_thermal: SoilThermalStateRestartV1::project(shadow.restart_authority_soil_thermal())
            .unwrap(),
        biogeochemistry: BiogeochemistryStateRestartV1::project(
            shadow.restart_authority_biogeochemistry(),
        )
        .unwrap(),
    }
}

pub fn project_evidence_complete_live_owners(
    shadow: &DirectV10RealConsumerShadow,
    phase_plan_sha256: &Sha256Hex,
    day_input_digests: &[Sha256Hex],
    expected_next_day_index: usize,
) -> CompleteCommittedOwnerStateV1 {
    let native_gsi_state =
        openwepp_hillslope_orchestrator::runtime_inputs::restart_authority_project_gsi_state(
            shadow.gsi_state(),
        )
        .unwrap();
    CompleteCommittedOwnerStateV1 {
        gsi_configuration: DirectGsiOwnerConfigurationRestartV1::project(
            shadow.gsi_owner_configuration(),
        )
        .unwrap(),
        gsi_state: DirectGsiOwnerStateRestartV1::project(&native_gsi_state).unwrap(),
        static_forcing_configuration: SnowFreeHalfHourStaticConfigurationRestartV1::project(
            shadow.provider_static_configuration(),
        )
        .unwrap(),
        provider_cursor: SnowFreeHalfHourProviderCursorRestartV1::project(
            shadow.provider_cursor(),
            shadow.provider_static_configuration(),
            expected_next_day_index,
        )
        .unwrap(),
        surface_liquid_configuration: DirectSurfaceLiquidConfigurationRestartV1::project(
            shadow.restart_authority_surface_configuration(),
        )
        .unwrap(),
        scientific: project_evidence_scientific_owners(
            shadow,
            phase_plan_sha256,
            day_input_digests,
        ),
    }
}

pub fn restart_authority_in_progress_checkpoint_fixture(
    through: u8,
) -> (
    RestartAuthorityPreparedDayFixture,
    DirectV10RealConsumerCheckpointV1,
    Sha256Hex,
    Sha256Hex,
) {
    let mut fixture = restart_authority_prepared_day_fixture();
    fixture
        .owners
        .runtime
        .shadow
        .restart_authority_advance_staged_intervals(
            &fixture.prepared,
            fixture.template.clone(),
            0,
            usize::from(through),
        )
        .unwrap();
    let staged = project_evidence_scientific_owners(
        &fixture.owners.runtime.shadow,
        &fixture.owners.phase_plan_sha256,
        &fixture.owners.day_input_digests,
    );
    let (run, topology) = restart_authority_identities(&fixture.owners.committed);
    let mut checkpoint = DirectV10RealConsumerCheckpointV1 {
        schema: "OPENWEPP_DIRECT_V10_REAL_CONSUMER_CHECKPOINT_V1".into(),
        version: 1,
        run_identity_sha256: run.clone(),
        topology_sha256: topology.clone(),
        phase: DirectV10CheckpointPhaseV1::InProgressDay {
            day_index: WireDayIndex(0),
            next_interval_index: crate::InProgressIntervalIndex::try_new(through).unwrap(),
            accepted_interval_count: AcceptedIntervalCount::try_new(u64::from(through)).unwrap(),
            committed_day_beginning: fixture.owners.committed.clone(),
            staged_scientific: staged,
            accepted_gsi_daily_receipt: fixture.gsi_receipt.clone(),
            staged_gsi_ending_state: fixture.ending_gsi_state.clone(),
            ending_provider_cursor: fixture.ending_cursor.clone(),
            validated_forcing_day_receipts: fixture.forcing_receipts.clone(),
        },
        payload_sha256: Sha256Hex::try_new("0".repeat(64)).unwrap(),
    };
    checkpoint.seal().unwrap();
    (fixture, checkpoint, run, topology)
}

pub fn restart_authority_prepared_day_fixture() -> RestartAuthorityPreparedDayFixture {
    let source = "5.30\n1 0 0\nTEST STATION 1500\nDAY MON YEAR PRCP STMDUR TIMEP IP TMAX TMIN RAD VWIND WIND TDPT\n41.1 -120.0 1225.0 30 2000 1 CLIGEN 5.30 --seed 123\nMONTHLY MAX TEMP HEADER\n1 2 3 4 5 6 7 8 9 10 11 12\nMONTHLY MIN TEMP HEADER\n-5 -4 -3 -2 -1 0 1 2 3 4 5 6\nMONTHLY RAD HEADER\n100 101 102 103 104 105 106 107 108 109 110 111\nMONTHLY RAIN HEADER\n10 11 12 13 14 15 16 17 18 19 20 21\nDAILY HEADER\nDAILY UNITS\n20 6 2000 0.0 0.0 0.0 0.0 28.0 22.0 0.0 2.5 180.0 20.0\n";
    restart_authority_prepared_day_fixture_from_source(source)
}

pub fn restart_authority_cross_midnight_carry_fixture() -> RestartAuthorityPreparedDayFixture {
    let source = "5.30\n1 1 0\nTEST STATION 1500\nDAY MON YEAR NBRKPT TMAX TMIN RAD VWIND WIND TDPT\n41.1 -120.0 1225.0 30 2000 1\nMONTHLY MAX TEMP HEADER\n1 2 3 4 5 6 7 8 9 10 11 12\nMONTHLY MIN TEMP HEADER\n-5 -4 -3 -2 -1 0 1 2 3 4 5 6\nMONTHLY RAD HEADER\n100 101 102 103 104 105 106 107 108 109 110 111\nMONTHLY RAIN HEADER\n10 11 12 13 14 15 16 17 18 19 20 21\nDAILY HEADER\nDAILY UNITS\n20 6 2000 3 28.0 22.0 420.0 2.5 180.0 20.0\n23.50 0.0\n24.00 3.6\n24.50 7.2\n";
    restart_authority_prepared_day_fixture_from_source(source)
}

fn restart_authority_prepared_day_fixture_from_source(
    source: &str,
) -> RestartAuthorityPreparedDayFixture {
    let mut owners = restart_authority_owner_fixture();
    let climate = parse_climate_from_str(source, ParserMode::SnowFreeHalfHourProvider).unwrap();
    let request = build_hillslope_climate_runtime_request(&climate).unwrap();
    let shadow = &owners.runtime.shadow;
    let prepared = request
        .prepare_snow_free_gsi_day_from_repository(
            0,
            shadow.provider_static_configuration(),
            shadow.gsi_owner_configuration(),
            shadow.gsi_state(),
            shadow.provider_cursor(),
        )
        .unwrap();
    let gsi_receipt = DirectGsiDailyReceiptRestartV1::project(prepared.gsi_receipt()).unwrap();
    owners.committed.gsi_state = gsi_receipt.beginning_state.clone();
    let ending_gsi_state = gsi_receipt.ending_state.clone();
    let forcing_receipts = prepared
        .forcing_receipts()
        .receipts()
        .iter()
        .map(crate::SnowFreeHalfHourDayReceiptRestartV1::project)
        .collect::<Result<Vec<_>, _>>()
        .unwrap();
    let mut gsi_state = shadow.gsi_state().clone();
    let mut cursor = shadow.provider_cursor().clone();
    prepared
        .clone()
        .commit(&mut gsi_state, &mut cursor)
        .unwrap();
    let ending_cursor = SnowFreeHalfHourProviderCursorRestartV1::project(
        &cursor,
        shadow.provider_static_configuration(),
        1,
    )
    .unwrap();
    let base_vegetation = owners.runtime.endpoint.receipt.forcing().clone();
    let intervals = (0..48)
        .map(|index| {
            let mut forcing = owners.runtime.endpoint.forcing.clone();
            forcing.transaction_id = TransactionId(41 + index as u128);
            forcing.forcing_sha256 = forcing.canonical_sha256().unwrap();
            DirectV9ShadowIntervalInput {
                lse_forcing: forcing,
                vegetation_forcing: base_vegetation.clone(),
                wb14_parameters: vec![DirectOfeWb14Parameters {
                    ofe_id: OfeId::try_new("ofe-1").unwrap(),
                    effective_conductivity_m_s: 1e-6,
                    matric_potential_m: 0.1,
                    infiltration_storage_capacity_m: 0.04,
                }],
            }
        })
        .collect();
    let template = DirectV10ShadowDayInput::try_new(0, intervals).unwrap();
    RestartAuthorityPreparedDayFixture {
        owners,
        gsi_receipt,
        ending_gsi_state,
        ending_cursor,
        forcing_receipts,
        prepared,
        template,
    }
}

pub fn restart_authority_owner_fixture() -> RestartAuthorityOwnerFixture {
    let runtime = restart_authority_evidence_fixture();
    let shadow = &runtime.shadow;
    let mut hydrology_frame = shadow.restart_authority_hydrology_frame().clone();
    hydrology_frame.lane_transfer_ledger = hydrology_frame
        .lanes
        .iter()
        .map(
            |lane| openwepp_hillslope_orchestrator::DirectLaneTransferLedger {
                lane_id: lane.lane_id,
                upstream_lane_id: lane.upstream_lane_id,
                downstream_lane_id: lane.downstream_lane_id,
                upstream_area_ratio: lane.upstream_area_ratio,
                area_m2: lane.area_m2,
                outgoing_surface_m: 0.0,
                outgoing_lateral_m: 0.0,
                received_surface_m: 0.0,
                received_lateral_m: 0.0,
                net_transfer_m: 0.0,
            },
        )
        .collect();
    let mut native_gsi_state =
        openwepp_hillslope_orchestrator::runtime_inputs::DirectGsiOwnerStateV1 {
            history_oldest_first: vec![],
            last_date: None,
            state_sha256: String::new(),
        };
    let mut gsi_bytes =
        serde_json::to_vec(&serde_json::to_value(&native_gsi_state).unwrap()).unwrap();
    gsi_bytes.push(b'\n');
    native_gsi_state.state_sha256 = format!("{:x}", Sha256::digest(gsi_bytes));
    let phase_plan_sha256 = Sha256Hex::try_new(
        canonical_sha256(&format!(
            "{:?}",
            shadow.restart_authority_hydrology_frame().phase_plan
        ))
        .unwrap(),
    )
    .unwrap();
    let day_inputs = shadow
        .restart_authority_hydrology_frame()
        .lanes
        .iter()
        .map(|lane| lane.day_inputs.clone())
        .collect::<Vec<_>>();
    let day_input_digests = day_inputs
        .iter()
        .enumerate()
        .map(|(index, _)| {
            Sha256Hex::try_new(canonical_sha256(&("DIRECT_DAY_INPUTS_V1", index)).unwrap()).unwrap()
        })
        .collect::<Vec<_>>();
    let lineage = shadow.vegetation_state().0.last_transaction_id;
    let mut lse_state = shadow.lse_state().clone();
    lse_state.0.last_accepted_transaction_id =
        Some(openwepp_kernel_contract::TransactionId(lineage));
    lse_state.0.state_sha256 = lse_state.0.canonical_sha256().unwrap();
    let mut soil_thermal = shadow.restart_authority_soil_thermal().clone();
    soil_thermal.last_accepted_transaction_id =
        Some(openwepp_kernel_contract::TransactionId(lineage));
    let scientific = ScientificOwnerStateSetV1 {
        vegetation_v10: VegetationV10StateRestartV1::project(
            shadow.vegetation_state(),
            shadow.restart_authority_vegetation_configuration(),
        )
        .unwrap(),
        lse_v2: LseV2StateRestartV1::project(
            &lse_state,
            shadow.restart_authority_lse_configuration(),
        )
        .unwrap(),
        direct_hydrology: DirectHydrologyRestartV1::project(
            &hydrology_frame,
            phase_plan_sha256.clone(),
            &day_input_digests,
        )
        .unwrap(),
        soil_thermal: SoilThermalStateRestartV1::project(&soil_thermal).unwrap(),
        biogeochemistry: BiogeochemistryStateRestartV1::project(
            shadow.restart_authority_biogeochemistry(),
        )
        .unwrap(),
    };
    let committed = CompleteCommittedOwnerStateV1 {
        gsi_configuration: DirectGsiOwnerConfigurationRestartV1::project(
            shadow.gsi_owner_configuration(),
        )
        .unwrap(),
        gsi_state: DirectGsiOwnerStateRestartV1::project(&native_gsi_state).unwrap(),
        static_forcing_configuration: SnowFreeHalfHourStaticConfigurationRestartV1::project(
            shadow.provider_static_configuration(),
        )
        .unwrap(),
        provider_cursor: SnowFreeHalfHourProviderCursorRestartV1::project(
            shadow.provider_cursor(),
            shadow.provider_static_configuration(),
            0,
        )
        .unwrap(),
        surface_liquid_configuration: DirectSurfaceLiquidConfigurationRestartV1::project(
            shadow.restart_authority_surface_configuration(),
        )
        .unwrap(),
        scientific,
    };
    RestartAuthorityOwnerFixture {
        runtime,
        committed,
        phase_plan_sha256,
        day_input_digests,
        day_inputs,
    }
}

pub fn restart_authority_evidence_fixture() -> RestartAuthorityEvidenceFixture {
    let endpoint = endpoint_fixture();
    let mut v9_configuration = endpoint.vegetation_configuration.clone();
    v9_configuration.model_definition_sha256 = V9_MODEL_SHA256.into();
    v9_configuration.configuration_sha256 = v9_configuration.canonical_sha256().unwrap();
    let mut v9_payload = endpoint.vegetation_state.clone();
    v9_payload.model_definition_sha256 = V9_MODEL_SHA256.into();
    v9_payload
        .configuration_sha256
        .clone_from(&v9_configuration.configuration_sha256);
    v9_payload.state_sha256 = v9_payload.canonical_sha256();
    let v9_state = V9CoupledOwnedState(v9_payload);
    let mut vegetation_configuration = v9_configuration;
    vegetation_configuration.model_definition_sha256 = V10_MODEL_SHA256.into();
    vegetation_configuration.configuration_sha256 =
        vegetation_configuration.canonical_sha256().unwrap();
    let mut vegetation_payload = v9_state.0;
    vegetation_payload.model_definition_sha256 = V10_MODEL_SHA256.into();
    vegetation_payload
        .configuration_sha256
        .clone_from(&vegetation_configuration.configuration_sha256);
    vegetation_payload.state_sha256 = vegetation_payload.canonical_sha256();
    vegetation_configuration
        .initial_state_sha256
        .clone_from(&vegetation_payload.state_sha256);
    let vegetation_state = V10CoupledOwnedState(vegetation_payload);
    let mut lse_configuration = endpoint.lse_configuration.clone();
    lse_configuration.model_version = V2_MODEL_VERSION.into();
    lse_configuration.model_definition_sha256 =
        Sha256Digest::try_new(V2_MODEL_DEFINITION_SHA256).unwrap();
    lse_configuration.vegetation_configuration.model_version = V2_VEGETATION_MODEL_VERSION.into();
    lse_configuration
        .vegetation_configuration
        .model_definition_sha256 =
        Sha256Digest::try_new(V2_VEGETATION_MODEL_DEFINITION_SHA256).unwrap();
    lse_configuration
        .vegetation_configuration
        .configuration_sha256 =
        Sha256Digest::try_new(vegetation_configuration.configuration_sha256.clone()).unwrap();
    lse_configuration.configuration_sha256 = lse_configuration.canonical_sha256().unwrap();
    let mut lse_payload = endpoint.lse_state.clone();
    lse_payload.model_definition_sha256 =
        Sha256Digest::try_new(V2_MODEL_DEFINITION_SHA256).unwrap();
    lse_payload
        .configuration_sha256
        .clone_from(&lse_configuration.configuration_sha256);
    lse_payload.state_sha256 = lse_payload.canonical_sha256().unwrap();
    let lse_state = LandSurfaceEnergyV2State(lse_payload);
    let gsi = DirectGsiOwnerConfigurationV1::try_new(
        "restart-authority-gsi".into(),
        GsiParameters::generalized(),
        41.1,
    )
    .unwrap();
    let wb14 = DirectOfeWb14Parameters {
        ofe_id: OfeId::try_new("ofe-1").unwrap(),
        effective_conductivity_m_s: 1e-6,
        matric_potential_m: 0.1,
        infiltration_storage_capacity_m: 0.04,
    };
    let forcing = SnowFreeHalfHourStaticConfiguration {
        run_id: endpoint
            .hydrology
            .beginning_frame()
            .identity
            .run_id
            .to_string(),
        co2_pa: endpoint.receipt.forcing().co2_pa,
        reference_height_m: endpoint.receipt.forcing().reference_height_m,
        gsi_owner_configuration_sha256: gsi.configuration_sha256.clone(),
        destinations: ["forest", "open"]
            .into_iter()
            .map(|tile| SnowFreeHalfHourDestination {
                ofe_id: "ofe-1".into(),
                tile_id: tile.into(),
                wb14_configuration_sha256: restart_authority_wb14_parameter_sha256(&wb14),
            })
            .collect(),
    };
    let mut hydrology_frame = endpoint.hydrology.beginning_frame().clone();
    hydrology_frame.lane_transfer_ledger = hydrology_frame
        .lanes
        .iter()
        .map(
            |lane| openwepp_hillslope_orchestrator::DirectLaneTransferLedger {
                lane_id: lane.lane_id,
                upstream_lane_id: lane.upstream_lane_id,
                downstream_lane_id: lane.downstream_lane_id,
                upstream_area_ratio: lane.upstream_area_ratio,
                area_m2: lane.area_m2,
                outgoing_surface_m: 0.0,
                outgoing_lateral_m: 0.0,
                received_surface_m: 0.0,
                received_lateral_m: 0.0,
                net_transfer_m: 0.0,
            },
        )
        .collect();
    let shadow = DirectV10RealConsumerShadow::try_new(
        vegetation_configuration,
        vegetation_state,
        ResourceOwnerId::try_new("vegetation-v10").unwrap(),
        lse_configuration,
        lse_state,
        endpoint.surface_configuration.clone(),
        endpoint.hydrology.restart_authority_layer_maps().to_vec(),
        endpoint.thermal.clone(),
        endpoint.biogeochemistry.clone(),
        hydrology_frame,
        0,
        gsi,
        GsiState::new(),
        forcing,
        SnowFreeHalfHourProviderCursor::default(),
    )
    .unwrap();
    RestartAuthorityEvidenceFixture { shadow, endpoint }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn cross_midnight_fixture_has_real_outgoing_and_cursor_carry() {
        let fixture = restart_authority_cross_midnight_carry_fixture();
        let outgoing = fixture
            .forcing_receipts
            .iter()
            .flat_map(|receipt| &receipt.next_day_precipitation_carry)
            .collect::<Vec<_>>();
        assert!(!outgoing.is_empty());
        assert_eq!(outgoing.len(), fixture.ending_cursor.pending_carry.len());
        assert_eq!(outgoing, fixture.ending_cursor.pending_carry.iter().collect::<Vec<_>>());
    }
    use crate::{
        AcceptedIntervalCount, DirectV10CheckpointPhaseV1, DirectV10RealConsumerCheckpointV1,
        ExpectedRestartStaticContext, WireDayIndex, admit_checkpoint_v1, to_canonical_bytes,
    };
    #[test]
    fn repository_backed_v10_lse_v2_fixture_is_valid() {
        let fixture = restart_authority_evidence_fixture();
        fixture
            .shadow
            .vegetation_state()
            .validate(fixture.shadow.restart_authority_vegetation_configuration())
            .unwrap();
        fixture
            .shadow
            .lse_state()
            .validate(fixture.shadow.restart_authority_lse_configuration())
            .unwrap();
    }

    #[test]
    fn complete_repository_owner_set_is_admitted_into_fresh_objects() {
        let fixture = restart_authority_owner_fixture();
        let (run, topology) = restart_authority_identities(&fixture.committed);
        let mut checkpoint = DirectV10RealConsumerCheckpointV1 {
            schema: "OPENWEPP_DIRECT_V10_REAL_CONSUMER_CHECKPOINT_V1".into(),
            version: 1,
            run_identity_sha256: run.clone(),
            topology_sha256: topology.clone(),
            phase: DirectV10CheckpointPhaseV1::BetweenDays {
                next_day_index: WireDayIndex(0),
                accepted_interval_count: AcceptedIntervalCount::try_new(0).unwrap(),
                committed: fixture.committed.clone(),
            },
            payload_sha256: Sha256Hex::try_new("0".repeat(64)).unwrap(),
        };
        checkpoint.seal().unwrap();
        assert_eq!(
            to_canonical_bytes(&checkpoint.abort_to_day_beginning()).unwrap(),
            to_canonical_bytes(&fixture.committed).unwrap()
        );
        let context = ExpectedRestartStaticContext {
            run_identity_sha256: &run,
            topology_sha256: &topology,
            vegetation_configuration: fixture
                .runtime
                .shadow
                .restart_authority_vegetation_configuration(),
            lse_configuration: fixture.runtime.shadow.restart_authority_lse_configuration(),
            surface_liquid_configuration: fixture
                .runtime
                .shadow
                .restart_authority_surface_configuration(),
            gsi_configuration: fixture.runtime.shadow.gsi_owner_configuration(),
            forcing_static_configuration: fixture.runtime.shadow.provider_static_configuration(),
            phase_plan: &fixture
                .runtime
                .shadow
                .restart_authority_hydrology_frame()
                .phase_plan,
            phase_plan_sha256: &fixture.phase_plan_sha256,
            day_inputs: &fixture.day_inputs,
            day_input_digests: &fixture.day_input_digests,
        };
        admit_checkpoint_v1(&to_canonical_bytes(&checkpoint).unwrap(), &context).unwrap();
    }

    #[test]
    fn repository_prepared_day_has_every_validated_interval() {
        let mut fixture = restart_authority_prepared_day_fixture();
        assert_eq!(fixture.forcing_receipts.len(), 2);
        assert_eq!(fixture.forcing_receipts[0].intervals.len(), 48);
        assert_eq!(fixture.ending_cursor.next_day_index.0, 1);
        fixture
            .owners
            .runtime
            .shadow
            .restart_authority_advance_staged_intervals(
                &fixture.prepared,
                fixture.template.clone(),
                0,
                24,
            )
            .unwrap();
        assert_eq!(
            fixture
                .owners
                .runtime
                .shadow
                .vegetation_state()
                .0
                .last_transaction_id,
            64
        );
        let staged = project_evidence_scientific_owners(
            &fixture.owners.runtime.shadow,
            &fixture.owners.phase_plan_sha256,
            &fixture.owners.day_input_digests,
        );
        let (run, topology) = restart_authority_identities(&fixture.owners.committed);
        let mut checkpoint = DirectV10RealConsumerCheckpointV1 {
            schema: "OPENWEPP_DIRECT_V10_REAL_CONSUMER_CHECKPOINT_V1".into(),
            version: 1,
            run_identity_sha256: run.clone(),
            topology_sha256: topology.clone(),
            phase: DirectV10CheckpointPhaseV1::InProgressDay {
                day_index: WireDayIndex(0),
                next_interval_index: crate::InProgressIntervalIndex::try_new(24).unwrap(),
                accepted_interval_count: AcceptedIntervalCount::try_new(24).unwrap(),
                committed_day_beginning: fixture.owners.committed.clone(),
                staged_scientific: staged,
                accepted_gsi_daily_receipt: fixture.gsi_receipt.clone(),
                staged_gsi_ending_state: fixture.ending_gsi_state.clone(),
                ending_provider_cursor: fixture.ending_cursor.clone(),
                validated_forcing_day_receipts: fixture.forcing_receipts.clone(),
            },
            payload_sha256: Sha256Hex::try_new("0".repeat(64)).unwrap(),
        };
        checkpoint.seal().unwrap();
        assert_eq!(
            to_canonical_bytes(&checkpoint.abort_to_day_beginning()).unwrap(),
            to_canonical_bytes(&fixture.owners.committed).unwrap()
        );
        let context = ExpectedRestartStaticContext {
            run_identity_sha256: &run,
            topology_sha256: &topology,
            vegetation_configuration: fixture
                .owners
                .runtime
                .shadow
                .restart_authority_vegetation_configuration(),
            lse_configuration: fixture
                .owners
                .runtime
                .shadow
                .restart_authority_lse_configuration(),
            surface_liquid_configuration: fixture
                .owners
                .runtime
                .shadow
                .restart_authority_surface_configuration(),
            gsi_configuration: fixture.owners.runtime.shadow.gsi_owner_configuration(),
            forcing_static_configuration: fixture
                .owners
                .runtime
                .shadow
                .provider_static_configuration(),
            phase_plan: &fixture
                .owners
                .runtime
                .shadow
                .restart_authority_hydrology_frame()
                .phase_plan,
            phase_plan_sha256: &fixture.owners.phase_plan_sha256,
            day_inputs: &fixture.owners.day_inputs,
            day_input_digests: &fixture.owners.day_input_digests,
        };
        let hydro_context = crate::ExpectedDirectHydrologyRestartContext {
            phase_plan: context.phase_plan,
            phase_plan_sha256: context.phase_plan_sha256,
            day_inputs: context.day_inputs,
            day_input_digests: context.day_input_digests,
            surface_liquid_configuration: context.surface_liquid_configuration,
        };
        if let DirectV10CheckpointPhaseV1::InProgressDay {
            staged_scientific, ..
        } = &checkpoint.phase
        {
            staged_scientific
                .direct_hydrology
                .restore(&hydro_context)
                .unwrap();
            staged_scientific
                .vegetation_v10
                .restore(context.vegetation_configuration)
                .unwrap();
            staged_scientific
                .lse_v2
                .restore(context.lse_configuration)
                .unwrap();
            staged_scientific.soil_thermal.restore().unwrap();
            staged_scientific.biogeochemistry.restore().unwrap();
        }
        fixture.owners.committed.gsi_state.restore().unwrap();
        let native_receipt = fixture.gsi_receipt.restore().unwrap();
        assert_eq!(
            native_receipt.run_id,
            context.forcing_static_configuration.run_id
        );
        assert_eq!(native_receipt.day_index, 0);
        assert_eq!(
            fixture.gsi_receipt.beginning_state,
            fixture.owners.committed.gsi_state
        );
        assert_eq!(fixture.gsi_receipt.ending_state, fixture.ending_gsi_state);
        assert_eq!(
            native_receipt.configuration_sha256,
            context.gsi_configuration.configuration_sha256
        );
        fixture
            .owners
            .committed
            .scientific
            .vegetation_v10
            .restore(context.vegetation_configuration)
            .unwrap();
        fixture
            .owners
            .committed
            .scientific
            .lse_v2
            .restore(context.lse_configuration)
            .unwrap();
        fixture
            .owners
            .committed
            .scientific
            .direct_hydrology
            .restore(&hydro_context)
            .unwrap();
        fixture
            .owners
            .committed
            .scientific
            .soil_thermal
            .restore()
            .unwrap();
        fixture
            .owners
            .committed
            .scientific
            .biogeochemistry
            .restore()
            .unwrap();
        let admitted =
            admit_checkpoint_v1(&to_canonical_bytes(&checkpoint).unwrap(), &context).unwrap();
        let crate::IsolatedRestoredCheckpointV1::InProgressDay {
            committed_day_beginning,
            staged_scientific,
            staged_gsi_ending_state,
            accepted_gsi_daily_receipt,
            validated_forcing_day_receipts,
            ending_provider_cursor,
            ..
        } = admitted
        else {
            unreachable!()
        };
        let staged_gsi_ending_state =
            openwepp_hillslope_orchestrator::runtime_inputs::restart_authority_restore_gsi_state(
                &staged_gsi_ending_state,
            )
            .unwrap();
        let expected_ending_provider_cursor = ending_provider_cursor.clone();
        let committed_provider_cursor = committed_day_beginning.provider_cursor.clone();
        let committed_gsi_state =
            openwepp_hillslope_orchestrator::runtime_inputs::restart_authority_restore_gsi_state(
                &committed_day_beginning.gsi_state,
            )
            .unwrap();
        let restored_prepared = openwepp_hillslope_orchestrator::runtime_inputs::restart_authority_prepare_from_restored_receipts(
            accepted_gsi_daily_receipt,
            staged_gsi_ending_state.clone(),
            validated_forcing_day_receipts,
            committed_provider_cursor.clone(),
            ending_provider_cursor.clone(),
        )
        .unwrap();
        let mut resumed = DirectV10RealConsumerShadow::try_new(
            context.vegetation_configuration.clone(),
            staged_scientific.vegetation_v10,
            ResourceOwnerId::try_new("vegetation-v10").unwrap(),
            context.lse_configuration.clone(),
            staged_scientific.lse_v2,
            context.surface_liquid_configuration.clone(),
            fixture
                .owners
                .runtime
                .endpoint
                .hydrology
                .restart_authority_layer_maps()
                .to_vec(),
            staged_scientific.soil_thermal,
            staged_scientific.biogeochemistry,
            staged_scientific.direct_hydrology,
            0,
            context.gsi_configuration.clone(),
            committed_gsi_state,
            context.forcing_static_configuration.clone(),
            committed_provider_cursor,
        )
        .unwrap();
        resumed
            .restart_authority_install_staged_daily_owners(
                staged_gsi_ending_state,
                ending_provider_cursor,
                1,
            )
            .unwrap();
        resumed
            .restart_authority_advance_staged_intervals(
                &restored_prepared,
                fixture.template.clone(),
                24,
                48,
            )
            .unwrap();
        let mut continuous = restart_authority_prepared_day_fixture();
        continuous
            .owners
            .runtime
            .shadow
            .restart_authority_advance_staged_intervals(
                &continuous.prepared,
                continuous.template.clone(),
                0,
                48,
            )
            .unwrap();
        let resumed_projection = project_evidence_scientific_owners(
            &resumed,
            &fixture.owners.phase_plan_sha256,
            &fixture.owners.day_input_digests,
        );
        let continuous_projection = project_evidence_scientific_owners(
            &continuous.owners.runtime.shadow,
            &continuous.owners.phase_plan_sha256,
            &continuous.owners.day_input_digests,
        );
        assert_eq!(resumed_projection, continuous_projection);
        assert_eq!(
            resumed.gsi_state(),
            &openwepp_hillslope_orchestrator::runtime_inputs::restart_authority_restore_gsi_state(
                &fixture.gsi_receipt.restore().unwrap().ending_state,
            )
            .unwrap()
        );
        assert_eq!(resumed.provider_cursor(), &expected_ending_provider_cursor);
    }
}
