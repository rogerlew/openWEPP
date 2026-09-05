//! Real complete-owner parent chronology coverage for the adaptive Stage-3 path.

use super::*;

use std::sync::OnceLock;

use openwepp_kernel_contract::{ResourceOwnerId, TransactionId};
use openwepp_land_surface_energy::{
    LandSurfaceEnergyV2State, Sha256Digest, V2_MODEL_DEFINITION_SHA256, V2_MODEL_VERSION,
    V2_VEGETATION_MODEL_DEFINITION_SHA256, V2_VEGETATION_MODEL_VERSION,
};
use openwepp_plant_phenology::{GsiParameters, GsiState};
use openwepp_vegetation::{
    V9_MODEL_SHA256, V9CoupledOwnedState, V10_MODEL_SHA256, V10CoupledOwnedState,
    V11ParentTransaction, V11ParentTransactionCheckpoint, V11ValidatedHandoffAuditV1,
    VegetationConfiguration, VegetationConfigurationV11, begin_v11_validated_handoff_audit_v1,
    take_v11_validated_handoff_audit_v1,
};
use sha2::{Digest as _, Sha256};

use crate::DirectOfeWb14Parameters;
use crate::hydrology::{
    DirectSnowHourlyForcing, DirectSnowSurfaceEnergyOptions, SnowDensityModel, SnowMeltModel,
    SnowStage3LiquidRoutingModel, SnowSurfaceLongwaveModel, SnowSurfaceSublimationModel,
};
use crate::land_surface_energy_shadow::{EndpointFixture, endpoint_fixture};
use crate::runtime_inputs::{
    DirectGsiOwnerConfigurationV1, SnowFreeHalfHourDestination, SnowFreeHalfHourProviderCursor,
    SnowFreeHalfHourStaticConfiguration,
};
use crate::snow_stage3_open_boundary::{
    SealedOpenSnowExposureReceiptV1, SealedOpenSnowTileForcingInputsV1, SealedOpenSnowTileForcingV1,
};
use crate::snow_stage3_terminal_handoff::{
    ParticipantSupportReceipt, SealedCoveredCarrierForcingInputs, SealedExposureReceipt,
    Stage3TileBoundaryClassV1,
};
use crate::v9_real_consumer_shadow::{
    DirectRootZoneHydraulicConfiguration, DirectRootZoneLayerConfiguration,
    DirectRootZoneStratumGeometry,
};
use crate::vegetation_real_hydrology_shadow::RealHydrologySourceKey;
use crate::winter_column::DirectSnowLayerState;

const CHRONOLOGY_TEST_PARENT_NS: u128 = 60_000_000_000;
const PARENT_END_OPEN_SHORTWAVE_SLOPE_W_M2: f64 = 105.9084;
// Sealed physical forcing for the exact 60-second endpoint. The controller
// owns one complete temporal quantum here; no sub-floor event is admitted.
const PARENT_END_OPEN_SNOW_MULTIPLIER: f64 = 27.378_160_922_489_325;

fn digest(seed: u8) -> Digest32 {
    Digest32::from_bytes([seed; 32])
}

fn wb14_parameter_sha256(value: &DirectOfeWb14Parameters) -> String {
    let mut digest = Sha256::new();
    digest.update(value.ofe_id.as_str().as_bytes());
    for operand in [
        value.effective_conductivity_m_s,
        value.matric_potential_m,
        value.infiltration_storage_capacity_m,
    ] {
        digest.update(operand.to_bits().to_le_bytes());
    }
    format!("{:x}", digest.finalize())
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
    fixture.hydrology =
        crate::vegetation_real_hydrology_shadow::RealHydrologyShadowAdapter::try_from_day_start(
            &wet_frame,
            fixture.hydrology.day_index(),
            fixture.hydrology.transaction_id(),
            fixture.hydrology.interval_s(),
            fixture.hydrology.hydrology_owner_id().clone(),
            fixture.hydrology.layer_maps(),
        )
        .expect("wet chronology hydrology fixture");

    let (mut vegetation_configuration, v9_state) = v9_configuration_and_state(&fixture);
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
    lse_configuration.vegetation_configuration.model_version = V2_VEGETATION_MODEL_VERSION.into();
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
        "stage3-parent-chronology-gsi-owner".into(),
        GsiParameters::generalized(),
        41.1,
    )
    .expect("GSI owner configuration");
    let wb14_parameters = lse_configuration
        .ofes
        .iter()
        .map(|ofe| DirectOfeWb14Parameters {
            ofe_id: ofe.ofe_id.clone(),
            effective_conductivity_m_s: 1.0e-6,
            matric_potential_m: 0.1,
            infiltration_storage_capacity_m: 0.04,
        })
        .collect::<Vec<_>>();
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
                let wb14 = wb14_parameters
                    .iter()
                    .find(|value| value.ofe_id == ofe.ofe_id)
                    .expect("OFE WB14 parameters");
                ofe.tiles
                    .iter()
                    .map(move |tile| SnowFreeHalfHourDestination {
                        ofe_id: ofe.ofe_id.as_str().to_owned(),
                        tile_id: tile.tile_id.as_str().to_owned(),
                        wb14_configuration_sha256: wb14_parameter_sha256(wb14),
                    })
            })
            .collect(),
    };

    let layer_maps = fixture.hydrology.layer_maps().to_vec();
    let mut root_layers = Vec::new();
    for map in &layer_maps {
        let mut top_m = 0.0;
        for layer_id in &map.layer_ids {
            let key = RealHydrologySourceKey {
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
            let saturated_matric_potential_mm = (-2_200.0 + 1_000.0 * node_m) / retention_factor;
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
    .expect("V10/LSE-V2 chronology fixture");
    (shadow, fixture)
}

fn wb14_parameters(fixture: &EndpointFixture) -> Vec<DirectOfeWb14Parameters> {
    fixture
        .lse_configuration
        .ofes
        .iter()
        .map(|ofe| DirectOfeWb14Parameters {
            ofe_id: ofe.ofe_id.clone(),
            effective_conductivity_m_s: 1.0e-6,
            matric_potential_m: 0.1,
            infiltration_storage_capacity_m: 0.04,
        })
        .collect()
}

fn stage3_inputs(swe_m: f64, cold_delta_k: f64) -> DirectActiveSnowPartitionInputs {
    // SC-SNOWENERGY-001@22 INV-043: use the production layer constructor's
    // exact mass × rho-water ÷ density ordering for every cached depth.
    let depth_m = swe_m * 1_000.0 / 100.0;
    let mut layer = DirectSnowLayerState::new(swe_m, depth_m, 100.0, 0.0);
    layer.temperature_c = -cold_delta_k;
    layer.cold_content_j_m2 = swe_m * 1_000.0 * 2_100.0 * cold_delta_k;
    DirectActiveSnowPartitionInputs {
        hyetograph_rainfall_m: 0.0,
        rst_c: 0.0,
        newsnw_kg_m3: 100.0,
        ssd_kg_m3: 522.0,
        runtime_swe_m: swe_m,
        runtime_depth_m: depth_m,
        runtime_density_kg_m3: 100.0,
        runtime_settle_day_count: 0.0,
        liquid_water_retained_m: 0.0,
        tmax_c: 0.0,
        tmin_c: 0.0,
        canopy_cover_fraction: 0.45,
        wind_m_s: 3.0,
        dewpoint_c: -15.0,
        snow_melt_model: SnowMeltModel::AdaptiveCompositionalStage3V1,
        snow_density_model: SnowDensityModel::PhysicsBulkDensityCompactionV1,
        stage3_liquid_routing_model: SnowStage3LiquidRoutingModel::LayeredThermalLiquidV1,
        surface_energy_options: DirectSnowSurfaceEnergyOptions {
            longwave_model: SnowSurfaceLongwaveModel::DilleyUnsworthSubcanopyV1,
            sublimation_model: SnowSurfaceSublimationModel::NeutralBulkStage3V1,
            daily_solar_radiation_mj_m2: 0.0,
            daily_extraterrestrial_radiation_mj_m2: 0.0,
            daylight: false,
            ..DirectSnowSurfaceEnergyOptions::default()
        },
        sturm_climate_class: None,
        sturm_day_of_year: None,
        coe_boundary_depth_m: depth_m,
        coe_boundary_density_kg_m3: 100.0,
        coe_boundary_settle_day_count: 0.0,
        snow_albedo_model: None,
        snow_albedo_state: None,
        snow_layers: vec![layer],
        underlying_surface_albedo: 0.2,
        hourly: [DirectSnowHourlyForcing::zero(); 24],
    }
}

fn participant_support(participant_id: &str, receipt_id: &str) -> ParticipantSupportReceipt {
    ParticipantSupportReceipt {
        participant_id: participant_id.to_owned(),
        support_receipt_id: receipt_id.to_owned(),
        minimum_support_ns: ModelTimeNs::new(STAGE3_V11_ADAPTIVE_MINIMUM_SUPPORT_NS),
    }
}

fn covered_carrier_forcing(reference_specific_humidity: f64) -> SealedCoveredCarrierForcing {
    SealedCoveredCarrierForcing::try_new(SealedCoveredCarrierForcingInputs {
        rho_air_kg_m3: 1.2,
        cp_air_j_kg_k: 1_005.0,
        reference_temperature_k: 280.0,
        reference_specific_humidity,
        atmospheric_longwave_w_m2: 280.0,
        effective_canopy_cover: 0.5,
        exposure: SealedExposureReceipt {
            receipt_id: "chronology-exposure-v1".to_owned(),
            provider: "sealed-stage3-exposure".to_owned(),
            provider_digest: "chronology-exposure-provider".to_owned(),
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
            participant_support("shared-carrier", "chronology-carrier-v1"),
            participant_support("stage3-snow", "chronology-stage3-v1"),
            participant_support("v11-canopy", "chronology-v11-v1"),
        ],
    })
    .expect("sealed chronology carrier forcing")
}

#[derive(Clone, Copy)]
struct ChronologyCase {
    swe_m: f64,
    cold_delta_k: f64,
    radiation_mj_m2: f64,
    open_snow_shortwave_multiplier: f64,
    reference_specific_humidity: f64,
    snowfall_m: f64,
    rain_m: f64,
    terminal_event: bool,
    hard_boundary_ns: Option<u128>,
}

impl ChronologyCase {
    const fn deposition_meltout() -> Self {
        Self {
            swe_m: 0.000_6,
            cold_delta_k: 0.0,
            radiation_mj_m2: 1_000.0,
            open_snow_shortwave_multiplier: 1.0,
            reference_specific_humidity: 0.002,
            snowfall_m: 0.0,
            rain_m: 0.0,
            terminal_event: true,
            hard_boundary_ns: None,
        }
    }

    const fn short_deposition_meltout_diagnostic() -> Self {
        let mut value = Self::deposition_meltout();
        // Preserve the production represented snow mass and scale only the
        // parent-total energy to the 60 s support. This keeps the source flux
        // unchanged without compressing the event below the 60 s floor.
        value.radiation_mj_m2 /= 30.0;
        // Exercise a real complete-energy authority without driving the
        // nonlinear canopy solver outside its constitutive domain. All four
        // sealed open-snow bands are perturbed consistently.
        value.open_snow_shortwave_multiplier = 50.0;
        value
    }

    const fn short_parent_end_meltout_diagnostic() -> Self {
        let mut value = Self::short_deposition_meltout_diagnostic();
        value.open_snow_shortwave_multiplier = PARENT_END_OPEN_SNOW_MULTIPLIER;
        value
    }
}

struct RealParentFixture {
    attachment: DirectSnowStage3V11ShadowAttachment,
    prepared: DirectSnowStage3V11PreparedSupport,
}

fn real_parent_fixture(case: ChronologyCase, parent_duration_ns: u128) -> RealParentFixture {
    let (shadow, fixture) = v10_shadow_fixture();
    real_parent_fixture_from_shadow(case, 0, parent_duration_ns, shadow, fixture)
}

fn native_v2_real_parent_fixture(
    case: ChronologyCase,
    parent_duration_ns: u128,
) -> RealParentFixture {
    let (v1_shadow, fixture) = v10_shadow_fixture();
    let shadow = crate::v9_real_consumer_shadow::migrate_shadow_to_native_v2_for_parent_test(
        v1_shadow,
        parent_duration_ns * 2,
    );
    real_parent_fixture_from_shadow(
        case,
        parent_duration_ns,
        parent_duration_ns,
        shadow,
        fixture,
    )
}

fn real_parent_fixture_from_shadow(
    case: ChronologyCase,
    parent_start_ns: u128,
    parent_duration_ns: u128,
    shadow: DirectV10RealConsumerShadow,
    fixture: EndpointFixture,
) -> RealParentFixture {
    let support = TimeSupport::new(
        ModelTimeNs::new(parent_start_ns),
        ModelTimeNs::new(parent_start_ns + parent_duration_ns),
    )
    .expect("chronology parent support");
    let mut interval = DirectV9ShadowIntervalInput {
        lse_forcing: fixture.forcing.clone(),
        vegetation_forcing: fixture.receipt.forcing().clone(),
        wb14_parameters: wb14_parameters(&fixture),
    };
    let parent_duration_s = f64::from_bits(support.duration_s_bits());
    interval.lse_forcing.interval_s = parent_duration_s;
    interval.lse_forcing.transaction_id = TransactionId(41);
    interval.lse_forcing.precipitation_parcels.clear();
    interval.lse_forcing.runon_parcels.clear();
    interval.lse_forcing.snow_present_at_beginning = true;
    interval.lse_forcing.snow_present_at_end = true;
    interval.lse_forcing.snow_terminal_payload_present = false;
    interval.lse_forcing.forcing_sha256 = interval
        .lse_forcing
        .canonical_sha256()
        .expect("chronology LSE forcing digest");
    interval.vegetation_forcing.rain_kg_m2 = case.rain_m * 1_000.0;
    let covered_interval = DirectV11SnowCoveredSegmentInput::from_snow_free(&interval);

    let mut inputs = stage3_inputs(case.swe_m, case.cold_delta_k);
    let mut forcing = DirectSnowHourlyForcing::zero();
    forcing.radiation_mj_m2 = case.radiation_mj_m2;
    forcing.snowfall_m = case.snowfall_m;
    forcing.rain_m = case.rain_m;
    forcing.active_precipitation_m = case.rain_m + case.snowfall_m * 0.1;
    if case.snowfall_m > 0.0 {
        forcing.snow_fraction = 1.0;
        forcing.hydrometeor_temperature_c = Some(0.0);
    }
    if case.rain_m > 0.0 {
        forcing.rain_fraction = 1.0;
        forcing.hydrometeor_temperature_c = Some(1.0);
    }
    inputs.hourly[0] = forcing;
    let support_forcing = DirectSnowStage3SupportInput {
        forcing,
        duration_seconds: parent_duration_s,
    };
    let beginning_stage3 = if case.terminal_event {
        Wb11HydrologyKernel::initialize_stage3_persistent_state_with_terminal_event(
            1,
            inputs.snow_layers.clone(),
            DirectSnowTerminalEventRequest::ENTHALPY_EVENT_V1,
        )
    } else {
        Wb11HydrologyKernel::initialize_stage3_persistent_state(1, inputs.snow_layers.clone())
    }
    .expect("chronology Stage-3 beginning");

    let covered_tiles = fixture
        .vegetation_configuration
        .strata
        .iter()
        .flat_map(|stratum| stratum.tile_ids.iter().cloned())
        .collect::<BTreeSet<_>>();
    let covered_record = fixture
        .surface_configuration
        .records
        .iter()
        .find(|record| covered_tiles.contains(&record.key.tile_id))
        .expect("chronology covered destination");
    let open_record = fixture
        .surface_configuration
        .records
        .iter()
        .find(|record| !covered_tiles.contains(&record.key.tile_id))
        .expect("chronology open destination");
    let open_destination = (
        open_record.key.ofe_id.clone(),
        open_record.key.tile_id.clone(),
    );
    let open_exposure = SealedOpenSnowExposureReceiptV1::try_new(
        support,
        open_destination.clone(),
        digest(10),
        digest(11),
        interval.lse_forcing.reference_wind_m_s,
        digest(12),
    )
    .expect("chronology open exposure");
    let open_forcing = SealedOpenSnowTileForcingV1::try_new(SealedOpenSnowTileForcingInputsV1 {
        support,
        destination: open_destination.clone(),
        forcing_receipt_sha256: digest(10),
        exposure: open_exposure,
        reference_temperature_k: interval.lse_forcing.air_temperature_k,
        reference_specific_humidity_kg_kg: interval.lse_forcing.air_specific_humidity_kg_kg,
        air_pressure_pa: interval.lse_forcing.air_pressure_pa,
        atmospheric_downward_longwave_w_m2: interval.lse_forcing.atmospheric_downward_longwave_w_m2,
        direct_vis_w_m2: interval.lse_forcing.direct_vis_w_m2 * case.open_snow_shortwave_multiplier,
        diffuse_vis_w_m2: interval.lse_forcing.diffuse_vis_w_m2
            * case.open_snow_shortwave_multiplier,
        direct_nir_w_m2: interval.lse_forcing.direct_nir_w_m2 * case.open_snow_shortwave_multiplier,
        diffuse_nir_w_m2: interval.lse_forcing.diffuse_nir_w_m2
            * case.open_snow_shortwave_multiplier,
        rain_m: case.rain_m,
        snowfall_m: case.snowfall_m,
        precipitation_parcel_count: usize::from(case.rain_m > 0.0)
            + usize::from(case.snowfall_m > 0.0),
    })
    .expect("chronology open forcing");
    let snow_surface_forcing_by_destination = BTreeMap::from([
        (
            (
                covered_record.key.ofe_id.clone(),
                covered_record.key.tile_id.clone(),
            ),
            SealedStage3TileBoundaryForcingV1::V11CanopyCovered(covered_carrier_forcing(
                case.reference_specific_humidity,
            )),
        ),
        (
            open_destination,
            SealedStage3TileBoundaryForcingV1::OpenSnow(open_forcing),
        ),
    ]);
    let lane_binding = fixture
        .surface_configuration
        .ofe_bindings
        .iter()
        .find(|binding| binding.production_lane_id == 1)
        .expect("chronology lane binding");
    let identities = BTreeMap::from([(
        1,
        fixture
            .surface_configuration
            .records
            .iter()
            .filter(|record| record.key.ofe_id == lane_binding.ofe_id)
            .map(|record| {
                PreparedStage3V11SupportIdentityV1::new(
                    record.key.ofe_id.as_str().to_owned(),
                    record.key.tile_id.as_str().to_owned(),
                    "a".repeat(64),
                    digest(13),
                    Vec::new(),
                    digest(14),
                )
            })
            .collect(),
    )]);
    let prepared = if parent_duration_ns == STAGE3_V11_PARENT_SUPPORT_NS {
        PreparedStage3V11SupportV1::try_new(
            support,
            BTreeMap::from([(1, inputs)]),
            BTreeMap::from([(1, support_forcing)]),
            interval,
            identities,
        )
    } else {
        PreparedStage3V11SupportV1::try_new_for_short_production_test(
            support,
            BTreeMap::from([(1, inputs)]),
            BTreeMap::from([(1, support_forcing)]),
            interval,
            identities,
        )
    };
    let mut prepared = prepared
        .expect("chronology prepared support")
        .with_covered_v11_interval(covered_interval);
    if let Some(boundary_ns) = case.hard_boundary_ns {
        prepared = prepared
            .with_hard_boundaries(vec![ModelTimeNs::new(boundary_ns)])
            .expect("chronology hard boundary");
    }
    for (destination, forcing) in snow_surface_forcing_by_destination {
        prepared = match forcing {
            SealedStage3TileBoundaryForcingV1::V11CanopyCovered(value) => {
                prepared.with_covered_tile_forcing(destination, value)
            }
            SealedStage3TileBoundaryForcingV1::OpenSnow(value) => {
                prepared.with_sealed_open_tile_forcing(destination, value)
            }
        };
    }

    let configuration = DirectSnowStage3V11ProductionConfigurationV1 {
        run_identity: digest(1),
        topology_identity: digest(9),
        calendar_receipt: digest(2),
        controller_policy: digest(5),
        surface_liquid_configuration: fixture.surface_configuration.clone(),
        wb14_parameters: wb14_parameters(&fixture),
    };
    let mut attachment = DirectSnowStage3V11ShadowAttachment::new_production(
        configuration,
        BTreeMap::from([(1, beginning_stage3)]),
        shadow,
    )
    .expect("production chronology attachment");
    if parent_duration_ns == STAGE3_V11_PARENT_SUPPORT_NS {
        return RealParentFixture {
            attachment,
            prepared,
        };
    }
    let owner_envelopes = attachment
        .committed
        .v11_parent_state
        .staged_resource_owners()
        .clone();
    let owner_states = owner_envelopes
        .values()
        .map(V11OwnerEnvelope::to_owner_state)
        .collect::<Result<Vec<_>, _>>()
        .expect("short chronology owner states");
    let beginning_owner_digest =
        complete_owner_set_digest(&owner_states).expect("short chronology owner digest");
    let authority = ParentAuthorityV1::new(
        digest(1),
        digest(2),
        digest(3),
        0,
        support,
        beginning_owner_digest,
    )
    .expect("short chronology parent authority");
    let participants = owner_states
        .iter()
        .map(|owner| owner.owner_id().to_owned())
        .collect();
    let clock = CoupledClockStateV1::new(
        authority,
        owner_states,
        "snow-stage3-v11-chronology-test".to_owned(),
        participants,
        digest(5),
        Vec::new(),
    )
    .expect("short chronology clock");
    let parent = V11ParentTransaction::new_with_complete_owners(
        &attachment.static_context.vegetation_configuration,
        attachment.committed.v11_parent_state.beginning_state(),
        clock.parent_transaction_id(),
        support.start_ns(),
        owner_envelopes,
    )
    .expect("short chronology V11 parent");
    attachment.static_context.parent_duration_ns = parent_duration_ns;
    attachment.committed.v11_parent_state = parent;
    attachment.committed.coupled_clock = clock;
    RealParentFixture {
        attachment,
        prepared,
    }
}

#[test]
fn stage3_support_precipitation_custody_uses_geometric_snowfall_depth() {
    let mut case = ChronologyCase::deposition_meltout();
    case.snowfall_m = 0.04;
    case.terminal_event = false;
    let fixture = real_parent_fixture(case, CHRONOLOGY_TEST_PARENT_NS);
    let forcing = fixture
        .prepared
        .support_forcing_by_lane
        .get(&1)
        .expect("lane forcing")
        .forcing;

    assert_eq!(forcing.snowfall_m.to_bits(), case.snowfall_m.to_bits());
    assert_eq!(
        forcing.active_precipitation_m.to_bits(),
        (case.snowfall_m * 0.1).to_bits()
    );
    fixture
        .prepared
        .validate_precipitation_custody()
        .expect("geometric snowfall reconstructs active water depth");

    let mut poison = fixture.prepared;
    poison
        .support_forcing_by_lane
        .get_mut(&1)
        .expect("lane forcing")
        .forcing
        .active_precipitation_m = case.snowfall_m;
    assert!(matches!(
        poison.validate_precipitation_custody(),
        Err(DirectSnowStage3V11AttachmentError::Precipitation(
            "Stage-3 support precipitation phase closure"
        ))
    ));
}

type RealParentOutcome = (
    V11ParentTransaction,
    DirectV10RealConsumerShadow,
    CoupledClockStateV1,
    V11ParentCandidate,
    BTreeMap<u32, DirectSnowStage3PersistentState>,
    Vec<Stage3CoupledSubslabReceiptV1>,
    Vec<Stage3V11TerminalEventGroupV1>,
    Vec<DirectSnowStage3V11TerminalParcel>,
);

fn execute_real_parent(
    fixture: &RealParentFixture,
    injection: Option<Stage3V11FailureInjection>,
) -> Result<RealParentOutcome, DirectSnowStage3V11AttachmentError> {
    execute_covered_real_v11_parent(
        &fixture.attachment.static_context,
        &fixture.attachment.committed.v11_parent_state,
        &fixture.attachment.committed.real_consumer,
        &fixture.attachment.committed.coupled_clock,
        &fixture.prepared,
        0,
        0,
        digest(3),
        fixture.attachment.committed.stage3_by_lane.clone(),
        fixture.attachment.committed.terminal_parcels.clone(),
        injection,
    )
}

#[derive(Clone)]
struct ValidatedVegetationHandoffEvidenceV1 {
    configuration: VegetationConfigurationV11,
    checkpoint: V11ParentTransactionCheckpoint,
    beginning_checkpoint: V11ParentTransactionCheckpoint,
    audit: V11ValidatedHandoffAuditV1,
    accepted_segment_count: usize,
    ending_state: openwepp_vegetation::V11CoupledOwnedState,
    ending_vegetation_bytes: Vec<u8>,
}

fn validated_vegetation_handoff_evidence_v1() -> &'static ValidatedVegetationHandoffEvidenceV1 {
    static EVIDENCE: OnceLock<ValidatedVegetationHandoffEvidenceV1> = OnceLock::new();
    EVIDENCE.get_or_init(|| {
        std::thread::Builder::new()
            .name("validated-vegetation-handoff".to_owned())
            .stack_size(64 * 1024 * 1024)
            .spawn(|| {
                let _short_wb14_parent =
                    crate::direct_runtime::permit_short_wb14_parent_support_for_test(
                        CHRONOLOGY_TEST_PARENT_NS,
                    );
                let fixture = native_v2_real_parent_fixture(
                    ChronologyCase {
                        swe_m: 0.08,
                        cold_delta_k: 8.0,
                        radiation_mj_m2: 0.0,
                        open_snow_shortwave_multiplier: 1.0,
                        reference_specific_humidity: 0.002,
                        snowfall_m: 0.0,
                        rain_m: 0.0,
                        terminal_event: false,
                        hard_boundary_ns: None,
                    },
                    CHRONOLOGY_TEST_PARENT_NS,
                );
                let configuration = fixture
                    .attachment
                    .static_context
                    .vegetation_configuration
                    .clone();
                let beginning_checkpoint =
                    fixture.attachment.committed.v11_parent_state.checkpoint();
                begin_v11_validated_handoff_audit_v1();
                let outcome = execute_real_parent(&fixture, None)
                    .expect("validated vegetation handoff parent");
                let audit = take_v11_validated_handoff_audit_v1();
                let ending_vegetation_bytes = outcome
                    .3
                    .ending_complete_owners
                    .iter()
                    .find(|owner| owner.owner_id() == "vegetation")
                    .expect("ending vegetation owner")
                    .state_bytes()
                    .to_vec();
                ValidatedVegetationHandoffEvidenceV1 {
                    configuration,
                    checkpoint: outcome.0.checkpoint(),
                    beginning_checkpoint,
                    audit,
                    accepted_segment_count: outcome.3.accepted_segment_checkpoints.len(),
                    ending_state: outcome.3.ending_state,
                    ending_vegetation_bytes,
                }
            })
            .expect("spawn validated vegetation handoff fixture")
            .join()
            .expect("join validated vegetation handoff fixture")
    })
}

mod validated_handoff {
    use super::*;

    #[test]
    fn parent_finalization_reuses_one_validated_vegetation_image() {
        let evidence = validated_vegetation_handoff_evidence_v1();
        assert_eq!(evidence.audit.trusted_parent_handoff_reuses, 1);
        assert_eq!(evidence.audit.lineage_mutation_full_validations, 1);
        assert_eq!(
            evidence.ending_vegetation_bytes,
            serde_json::to_vec(&evidence.ending_state).expect("accepted vegetation bytes")
        );
    }

    #[test]
    fn untrusted_v11_executor_ending_is_independently_validated_once() {
        let evidence = validated_vegetation_handoff_evidence_v1();
        assert!(evidence.accepted_segment_count > 0);
        assert_eq!(evidence.audit.untrusted_executor_full_validations, 2);
        assert!(
            evidence.audit.untrusted_executor_full_validations as usize
                >= evidence.accepted_segment_count
        );
    }

    #[test]
    fn lineage_mutation_requires_new_digest_and_validation() {
        let evidence = validated_vegetation_handoff_evidence_v1();
        assert_eq!(evidence.audit.lineage_mutation_full_validations, 1);
        let mut poison = evidence.checkpoint.clone();
        poison.staged_state.last_parent_transaction_id += 1;
        poison.staged_state.physical.last_transaction_id += 1;
        poison.staged_state.physical.state_sha256 = poison.staged_state.physical.canonical_sha256();
        poison.staged_state.state_sha256 = poison
            .staged_state
            .canonical_sha256()
            .expect("mutated state digest");
        assert!(V11ParentTransaction::restore(&evidence.configuration, poison).is_err());
    }

    #[test]
    fn vegetation_restart_reparses_and_revalidates_complete_state() {
        let evidence = validated_vegetation_handoff_evidence_v1();
        let bytes = serde_json::to_vec(&evidence.beginning_checkpoint).expect("checkpoint bytes");
        let parsed: V11ParentTransactionCheckpoint =
            serde_json::from_slice(&bytes).expect("checkpoint parse");
        assert_eq!(
            serde_json::to_vec(&parsed).expect("checkpoint reserialize"),
            bytes
        );
        begin_v11_validated_handoff_audit_v1();
        let restored = V11ParentTransaction::restore(&evidence.configuration, parsed)
            .expect("restart validation");
        assert!(take_v11_validated_handoff_audit_v1().restart_full_validations >= 2);
        assert_eq!(restored.checkpoint(), evidence.beginning_checkpoint);
    }

    #[test]
    fn validated_vegetation_proof_is_not_transferable() {
        let evidence = validated_vegetation_handoff_evidence_v1();
        for poison in 0..4 {
            let mut checkpoint = evidence.checkpoint.clone();
            let mut configuration = evidence.configuration.clone();
            match poison {
                0 => checkpoint.staged_state.state_sha256.replace_range(..1, "f"),
                1 => configuration.configuration_sha256.replace_range(..1, "f"),
                2 => checkpoint.staged_state.last_parent_transaction_id += 1,
                _ => {
                    checkpoint
                        .accepted_segments
                        .last_mut()
                        .expect("accepted segment")
                        .duration_s_bits ^= 1
                }
            }
            assert!(V11ParentTransaction::restore(&configuration, checkpoint).is_err());
        }
        assert_eq!(
            evidence.beginning_checkpoint,
            serde_json::from_slice(
                &serde_json::to_vec(&evidence.beginning_checkpoint).expect("beginning checkpoint")
            )
            .expect("beginning checkpoint parse")
        );
    }
}

fn assert_parent_finalization_event(
    clock: &CoupledClockStateV1,
    finalized: &V11ParentCandidate,
    expected_tick: ModelTimeNs,
) {
    let receipt = clock
        .accepted_event_receipts()
        .last()
        .expect("retained V11 parent-finalization event");
    assert_eq!(receipt.tick(), expected_tick);
    assert_eq!(
        receipt.ending_owner_set_digest(),
        complete_owner_set_digest(clock.owners()).expect("finalized clock owner digest")
    );
    let clock_vegetation = clock
        .owners()
        .iter()
        .find(|owner| owner.owner_id() == "vegetation")
        .expect("clock vegetation owner");
    let finalized_vegetation = finalized
        .ending_complete_owners
        .iter()
        .find(|owner| owner.owner_id() == "vegetation")
        .expect("finalized vegetation owner");
    assert_eq!(clock_vegetation, finalized_vegetation);
    receipt.validate().expect("parent-finalization event seal");
}

#[test]
fn production_parent_chronology_fixture_constructs_live_owner() {
    let (shadow, fixture) = v10_shadow_fixture();
    assert_eq!(
        shadow.hydrology_frame().identity,
        fixture.hydrology.beginning_frame().identity
    );
}

#[test]
fn production_adaptive_deposition_meltout_reports_exact_event_tick() {
    std::thread::Builder::new()
        .name("stage3-v11-real-parent-chronology".to_owned())
        .stack_size(64 * 1024 * 1024)
        .spawn(production_adaptive_deposition_meltout_reports_exact_event_tick_on_large_stack)
        .expect("spawn real parent chronology fixture")
        .join()
        .expect("join real parent chronology fixture");
}

#[test]
fn v50_native_v2_real_finalizer_uses_validated_envelope_transition() {
    std::thread::Builder::new()
        .name("v50-native-v2-real-finalizer".to_owned())
        .stack_size(64 * 1024 * 1024)
        .spawn(|| {
            let _short_wb14_parent =
                crate::direct_runtime::permit_short_wb14_parent_support_for_test(
                    CHRONOLOGY_TEST_PARENT_NS,
                );
            let fixture = native_v2_real_parent_fixture(
                ChronologyCase {
                    swe_m: 0.08,
                    cold_delta_k: 8.0,
                    radiation_mj_m2: 0.0,
                    open_snow_shortwave_multiplier: 1.0,
                    reference_specific_humidity: 0.002,
                    snowfall_m: 0.0,
                    rain_m: 0.0,
                    terminal_event: false,
                    hard_boundary_ns: None,
                },
                CHRONOLOGY_TEST_PARENT_NS,
            );
            crate::v9_real_consumer_shadow::begin_v50_outer_owner_transition_evidence_v1();
            execute_real_parent(&fixture, None).expect("V50 native-V2 real finalizer");
            let rows =
                crate::v9_real_consumer_shadow::take_v50_outer_owner_transition_evidence_v1();
            assert!(!rows.is_empty(), "V50 real finalizer produced no proof");
            assert!(rows.iter().all(|row| {
                row.reconstructed_vegetation_transaction_id == row.envelope_transaction_id.0
                    && row.reconstructed_lse_transaction_id == Some(row.envelope_transaction_id)
                    && row.reconstructed_bgc_transaction_id == row.envelope_transaction_id.0
            }));
            assert!(rows.iter().any(|row| {
                row.beginning_vegetation_transaction_id != row.envelope_transaction_id.0
                    || row.beginning_lse_transaction_id != Some(row.envelope_transaction_id)
                    || row.beginning_bgc_transaction_id != row.envelope_transaction_id.0
                    || row.beginning_soil_transaction_id != Some(row.envelope_transaction_id)
            }));
        })
        .expect("spawn V50 native-V2 real finalizer")
        .join()
        .expect("join V50 native-V2 real finalizer");
}

fn production_adaptive_deposition_meltout_reports_exact_event_tick_on_large_stack() {
    let _short_wb14_parent =
        crate::direct_runtime::permit_short_wb14_parent_support_for_test(CHRONOLOGY_TEST_PARENT_NS);
    let fixture = real_parent_fixture(
        ChronologyCase::short_deposition_meltout_diagnostic(),
        CHRONOLOGY_TEST_PARENT_NS,
    );
    begin_adaptive_controller_test_audit(AdaptiveControllerTestPolicyV1::default());
    begin_adaptive_comparison_test_audit();
    let result = execute_real_parent(&fixture, None);
    let adaptive = take_adaptive_controller_test_audit();
    let comparisons = take_adaptive_comparison_test_audit();
    for (index, comparison) in comparisons.iter().take(8).enumerate() {
        eprintln!(
            "REAL_PARENT_COMPARISON[{index}] owner={:?} path={:?} authority={:?} direct={:?} composed={:?} denominator={:?} scaled={} discrete={:?}",
            comparison.maximum_owner_id,
            comparison.maximum_path,
            comparison.maximum_tolerance_authority,
            comparison.maximum_direct_value,
            comparison.maximum_composed_value,
            comparison.maximum_tolerance_denominator,
            comparison.maximum_scaled_error,
            comparison.first_discrete_surface_delta,
        );
    }
    if let Some(entry) = adaptive.last() {
        let diagnostics = entry
            .transient_diagnostics()
            .expect("transient adaptive diagnostics");
        eprintln!(
            "REAL_PARENT_TRIAL_COUNTS direct={} split={} accepted={} rejected={}",
            diagnostics.direct_trial_count,
            diagnostics.split_child_trial_count,
            diagnostics.accepted_microstep_count,
            diagnostics.rejected_candidate_count,
        );
    }
    if let Err(error) = &result {
        eprintln!("REAL_PARENT_TYPED_ERROR {error:?}");
    }
    let outcome = result.expect("real adaptive chronology");
    assert_parent_finalization_event(&outcome.2, &outcome.3, fixture.prepared.support.end_ns());
    let mut premature_clock = fixture.attachment.committed.coupled_clock.clone();
    let beginning_clock = premature_clock.clone();
    let mut premature_consumer = fixture.attachment.committed.real_consumer.clone();
    let mut premature_finalized = outcome.3.clone();
    assert!(matches!(
        install_v11_parent_finalization_owner_transition(
            &mut premature_clock,
            &mut premature_consumer,
            &fixture.attachment.static_context.vegetation_configuration,
            &mut premature_finalized,
        ),
        Err(DirectSnowStage3V11AttachmentError::Identity(
            "V11 parent finalization before accepted endpoint"
        ))
    ));
    assert_eq!(premature_clock, beginning_clock);
    assert_eq!(premature_finalized, outcome.3);
    for receipt in outcome.5.iter().take(4) {
        let lane = &receipt.lane_receipts[&1];
        eprintln!(
            "REAL_PARENT_BOUNDARY support={}..{} shortwave_w_m2={} longwave_w_m2={} sensible_w_m2={} vapor_kg_m2_s={} latent_j_m2={}",
            receipt.support.start_ns().get(),
            receipt.support.end_ns().get(),
            lane.aggregate_snow_absorbed_shortwave_w_m2,
            lane.aggregate_snow_net_longwave_w_m2,
            lane.aggregate_sensible_to_canopy_air_w_m2,
            lane.aggregate_vapor_to_canopy_air_kg_m2_s,
            lane.aggregate_latent_energy_to_canopy_air_j_m2,
        );
    }
    let ending_snow = &outcome.4[&1];
    eprintln!(
        "REAL_PARENT_PHASE_Q complete_energy_j_m2={} melt_kg_m2={} sublimation_kg_m2={} deposition_kg_m2={} ending_layers={}",
        ending_snow.cumulative_complete_energy_j_m2,
        ending_snow.cumulative_melt_kg_m2,
        ending_snow.cumulative_sublimation_kg_m2,
        ending_snow.cumulative_deposition_kg_m2,
        ending_snow.layers.len(),
    );
    assert_eq!(outcome.6.len(), 1);
    let group = &outcome.6[0];
    let accepted_event = group
        .accepted_event_receipt
        .as_ref()
        .expect("accepted terminal event");
    assert_eq!(accepted_event.tick(), group.tick);
    assert_eq!(group.candidates.len(), 1);
    let candidate = &group.candidates[0];
    assert_eq!(candidate.tick, group.tick);
    assert_eq!(outcome.7.len(), 1);
    let parcel = &outcome.7[0];
    assert_eq!(parcel.source_lane_id, candidate.lane_id);
    assert_eq!(parcel.support, candidate.support);
    assert_eq!(parcel.event_result_digest, candidate.event_result_digest);
    assert_eq!(parcel.event_ordinal, accepted_event.ordinal());
    assert_eq!(
        parcel.mass_kg_m2_tile_ground.to_bits(),
        candidate.event.terminal_liquid_kg_m2.to_bits()
    );
    let projected_specific_enthalpy =
        candidate.event.terminal_unallocated_energy_j_m2 / candidate.event.terminal_liquid_kg_m2;
    let projected_temperature_k = 273.15 + projected_specific_enthalpy / 4_218.0;
    assert_eq!(
        parcel.temperature_k.to_bits(),
        projected_temperature_k.to_bits()
    );
    assert_eq!(
        parcel.specific_liquid_enthalpy_j_kg.to_bits(),
        openwepp_land_surface_energy::liquid_enthalpy_j_kg(parcel.temperature_k).to_bits()
    );
    assert!(
        (parcel.specific_liquid_enthalpy_j_kg - projected_specific_enthalpy).abs()
            <= 1.0e-9_f64.max(1.0e-12 * projected_specific_enthalpy.abs())
    );
    assert_eq!(
        parcel.posture,
        DirectSnowStage3V11TerminalParcelPosture::Consumed
    );
    assert_eq!(parcel.receiver_destinations.len(), 2);
    assert!(parcel.receiver_destinations.windows(2).all(|pair| {
        (&pair[0].destination_ofe_id, &pair[0].destination_tile_id)
            < (&pair[1].destination_ofe_id, &pair[1].destination_tile_id)
    }));
    assert!(
        (parcel
            .receiver_destinations
            .iter()
            .map(|destination| destination.destination_fraction)
            .sum::<f64>()
            - 1.0)
            .abs()
            <= 1.0e-12
    );
    let publication_events = outcome.1.accepted_publication_event_handoffs();
    assert_eq!(publication_events.len(), 3);
    assert_eq!(&publication_events[0], accepted_event);
    assert!(
        publication_events
            .iter()
            .enumerate()
            .all(|(ordinal, event)| {
                event.tick() == group.tick && usize::try_from(event.ordinal()) == Ok(ordinal)
            })
    );
    assert_eq!(publication_events, outcome.2.accepted_event_receipts());
    assert_eq!(
        publication_events.last(),
        outcome.2.accepted_event_receipts().last()
    );
    let supports = outcome.1.accepted_publication_supports();
    let mut support_only = outcome.1.clone();
    support_only
        .restore_accepted_publication_supports(supports.clone())
        .expect("support-only test projection");
    assert_eq!(support_only.accepted_publication_supports(), supports);
    assert!(
        support_only
            .accepted_publication_event_handoffs()
            .is_empty()
    );
    let beginning_cursor = fixture.attachment.committed.stage3_by_lane[&1].next_interval_index;
    assert_eq!(ending_snow.next_interval_index, beginning_cursor + 1);
    assert_eq!(
        ending_snow.fingerprint,
        Wb11HydrologyKernel::stage3_persistent_state_fingerprint(ending_snow)
    );
    eprintln!(
        "REAL_PARENT_TERMINAL_PARCEL mass_kg_m2={} unallocated_energy_j_m2={} specific_enthalpy_j_kg={} temperature_k={} topology_count={} posture={:?} final_cursor={}",
        parcel.mass_kg_m2_tile_ground,
        candidate.event.terminal_unallocated_energy_j_m2,
        parcel.specific_liquid_enthalpy_j_kg,
        parcel.temperature_k,
        parcel.receiver_destinations.len(),
        parcel.posture,
        ending_snow.next_interval_index,
    );
    eprintln!("REAL_PARENT_EVENT_TICK={}", group.tick.get());
}

#[test]
fn production_parent_end_event_consumes_same_tick_without_successor_support() {
    std::thread::Builder::new()
        .name("stage3-v11-parent-end-terminal-receiver".to_owned())
        .stack_size(64 * 1024 * 1024)
        .spawn(|| {
            let _short_wb14_parent = crate::direct_runtime::permit_short_wb14_parent_support_for_test(
                CHRONOLOGY_TEST_PARENT_NS,
            );
            let fixture = real_parent_fixture(
                ChronologyCase::short_parent_end_meltout_diagnostic(),
                CHRONOLOGY_TEST_PARENT_NS,
            );
            begin_adaptive_controller_test_audit(AdaptiveControllerTestPolicyV1::default());
            begin_adaptive_performance_test_audit();
            begin_parent_end_receiver_transaction_audit();
            let outcome = execute_real_parent(&fixture, None).expect("parent-end terminal receiver");
            assert_parent_finalization_event(
                &outcome.2,
                &outcome.3,
                fixture.prepared.support.end_ns(),
            );
            let adaptive = take_adaptive_controller_test_audit();
            let performance = take_adaptive_performance_test_audit();
            let (receiver_beginning_transaction, receiver_ending_transaction) =
                take_parent_end_receiver_transaction_audit()
                    .expect("parent-end receiver transaction audit");
            let diagnostics = adaptive
                .last()
                .expect("parent-end adaptive audit")
                .transient_diagnostics()
                .expect("transient parent-end adaptive diagnostics");
            assert_eq!(outcome.6.len(), 1);
            let group = &outcome.6[0];
            let event = group.accepted_event_receipt.as_ref().expect("parent-end event");
            let terminal_custody = group
                .terminal_receiver_custody_v2()
                .expect("typed parent-end terminal-liquid custody");
            terminal_custody
                .validate(group)
                .expect("typed parent-end terminal-liquid custody closure");
            assert!(!terminal_custody.receiver_receipts.is_empty());
            assert_eq!(
                terminal_custody.receiver_event.parent_transaction_id(),
                event.parent_transaction_id()
            );
            assert_eq!(terminal_custody.receiver_event.ordinal(), event.ordinal() + 1);
            let mut receipt_substitution = group.clone();
            receipt_substitution
                .terminal_receiver_custody_v2
                .as_mut()
                .expect("typed parent-end custody poison target")
                .receiver_receipts[0]
                .output_set_sha256[0] ^= 1;
            assert!(receipt_substitution
                .validate_terminal_receiver_custody_v2()
                .is_err());
            assert_eq!(group.candidates.len(), 1);
            assert_eq!(group.produced_unconsumed_parcels.len(), 1);
            let retained = &group.produced_unconsumed_parcels[0];
            assert_eq!(
                group.produced_unconsumed_parcel_digests,
                vec![retained.parcel_digest]
            );
            assert_eq!(retained.parent_transaction_id, event.parent_transaction_id().digest());
            assert_eq!(retained.event_ordinal, event.ordinal());
            assert_eq!(retained.support.end_ns(), group.tick);
            assert_eq!(retained.receiver_destinations.len(), 2);
            assert_eq!(
                retained
                    .receiver_destinations
                    .iter()
                    .map(|destination| destination.destination_fraction)
                    .sum::<f64>()
                    .to_bits(),
                1.0_f64.to_bits()
            );
            let candidate = &group.candidates[0];
            assert_eq!(retained.source_lane_id, candidate.lane_id);
            assert_eq!(retained.event_result_digest, candidate.event_result_digest);
            assert_eq!(
                retained.mass_kg_m2_tile_ground.to_bits(),
                candidate.event.terminal_liquid_kg_m2.to_bits()
            );
            let accepted_supports = outcome.1.accepted_publication_supports();
            let accepted_output = accepted_supports
                .last()
                .expect("accepted parent-end support")
                .accepted_snow_liquid_outputs()
                .iter()
                .find(|output| output.lane_id == candidate.lane_id)
                .expect("sealed positive-support snow-liquid output");
            accepted_output
                .validate()
                .expect("positive-support snow-liquid output seal");
            assert_eq!(accepted_output.support, candidate.support);
            assert_eq!(
                accepted_output.mass_kg_m2_ofe_ground.to_bits(),
                candidate.event.terminal_liquid_kg_m2.to_bits()
            );
            let terminal_support_receipt = outcome
                .5
                .iter()
                .find(|receipt| receipt.terminal_events.contains_key(&candidate.lane_id))
                .expect("sealed accepted terminal support receipt");
            assert!(
                terminal_support_receipt
                    .post_support_liquid_receiver_event
                    .is_none(),
                "terminal parcel mode must not double-install positive-support receiver",
            );
            let invoke_receiver = |outputs: &[Stage3AcceptedSnowLiquidOutputV1]| {
                let receipts = outputs
                    .iter()
                    .map(Stage3AcceptedSnowLiquidOutputV1::receipt_sha256)
                    .collect::<Vec<_>>();
                let fields = receipts
                    .iter()
                    .map(|receipt| FramedField {
                        tag: "snow_liquid_output",
                        value: receipt.as_bytes(),
                    })
                    .collect::<Vec<_>>();
                let output_set = framed_sha256(
                    "stage3-v11-positive-support-liquid-output-set",
                    &fields,
                )
                .expect("output set");
                outcome.1.accept_zero_duration_stage3_support_liquid_receiver(
                    outputs,
                    output_set,
                    Digest32::from_bytes([31; 32]),
                    Digest32::from_bytes([32; 32]),
                    0,
                )
            };
            let (receiver_candidate, _, _, _, _) = invoke_receiver(&[
                accepted_output.clone(),
            ])
                .expect("real surface owner accepts sealed support-liquid output");
            let beginning_surface = outcome
                .1
                .hydrology_frame()
                .surface_liquid_shadow
                .as_deref()
                .expect("receiver beginning surface");
            let ending_surface = receiver_candidate
                .hydrology_frame()
                .surface_liquid_shadow
                .as_deref()
                .expect("receiver ending surface");
            let configuration = &fixture
                .attachment
                .static_context
                .surface_liquid_configuration;
            let weighted_storage = |state: &crate::DirectSurfaceLiquidOwnedState| {
                state
                    .records
                    .iter()
                    .filter(|record| record.key.ofe_id == accepted_output.ofe_id)
                    .map(|record| {
                        let configured = configuration
                            .records
                            .iter()
                            .find(|configured| configured.key == record.key)
                            .expect("receiver destination configuration");
                        record.liquid_kg_m2_tile * configured.tile_fraction
                    })
                    .sum::<f64>()
            };
            assert!(
                (weighted_storage(ending_surface)
                    - weighted_storage(beginning_surface)
                    - accepted_output.mass_kg_m2_ofe_ground)
                    .abs()
                    <= 1.0e-12,
                "real receiver surface storage must close to sealed output",
            );
            assert!(
                invoke_receiver(&[
                        accepted_output.clone(),
                        accepted_output.clone(),
                    ])
                    .is_err(),
                "duplicate output receipt must reject",
            );
            let mut output_substitution = accepted_output.clone();
            output_substitution.mass_kg_m2_ofe_ground += 1.0e-9;
            assert!(
                invoke_receiver(&[
                        output_substitution,
                    ])
                    .is_err(),
                "output mass substitution must reject",
            );
            validate_retained_terminal_receiver_custody_v1(group)
                .expect("retained parent-end receiver custody");
            let mut omission = group.clone();
            omission.produced_unconsumed_parcels.clear();
            assert!(validate_retained_terminal_receiver_custody_v1(&omission).is_err());
            let mut duplicate = group.clone();
            duplicate
                .produced_unconsumed_parcels
                .push(retained.clone());
            assert!(validate_retained_terminal_receiver_custody_v1(&duplicate).is_err());
            for poison in 0..5 {
                let mut substituted = group.clone();
                let custody = &mut substituted.produced_unconsumed_parcels[0];
                match poison {
                    0 => custody.mass_kg_m2_tile_ground += 1.0e-9,
                    1 => custody.temperature_k += 1.0,
                    2 => custody.receiver_topology_sha256 = digest(0x91),
                    3 => custody.receiver_destinations[0].destination_fraction += 1.0e-3,
                    _ => custody.receiver_destinations.swap(0, 1),
                }
                assert!(
                    validate_retained_terminal_receiver_custody_v1(&substituted).is_err(),
                    "retained receiver custody poison {poison}"
                );
            }
            let terminal_event = terminal_support_receipt
                .terminal_events
                .get(&candidate.lane_id)
                .expect("sealed accepted terminal event operand");
            assert_eq!(terminal_event, &candidate.event);
            let terminal_boundary = &terminal_support_receipt.lane_receipts[&candidate.lane_id];
            let open_boundary = terminal_boundary
                .ordered_destinations
                .iter()
                .find(|destination| {
                    destination.boundary_class == Stage3TileBoundaryClassV1::OpenSnow
                })
                .expect("sealed open-snow terminal boundary");
            let sealed_vis_w_m2 = 410.0 * PARENT_END_OPEN_SNOW_MULTIPLIER
                + 83.0 * PARENT_END_OPEN_SNOW_MULTIPLIER;
            let sealed_nir_w_m2 = 355.0 * PARENT_END_OPEN_SNOW_MULTIPLIER
                + 101.0 * PARENT_END_OPEN_SNOW_MULTIPLIER;
            let expected_open_shortwave_w_m2 = (sealed_vis_w_m2 + sealed_nir_w_m2) * (1.0 - 0.82);
            assert_eq!(
                open_boundary.snow_absorbed_shortwave_w_m2.to_bits(),
                expected_open_shortwave_w_m2.to_bits()
            );
            let reconstructed_slope = open_boundary.tile_fraction
                * open_boundary.snow_absorbed_shortwave_w_m2
                / PARENT_END_OPEN_SNOW_MULTIPLIER;
            assert!(
                (reconstructed_slope - PARENT_END_OPEN_SHORTWAVE_SLOPE_W_M2).abs() <= 1.0e-12
            );
            let duration_s = f64::from_bits(candidate.support.duration_s_bits());
            let vapor_into_snow_kg_m2 =
                -terminal_boundary.aggregate_vapor_to_canopy_air_kg_m2_s * duration_s;
            assert!(
                (vapor_into_snow_kg_m2
                    - (candidate.event.deposition_kg_m2 - candidate.event.sublimation_kg_m2))
                    .abs()
                    <= 1.0e-12
            );
            let phase_energy_j_m2 = crate::hydrology::STAGE3_LATENT_HEAT_FUSION_J_KG
                * (candidate.event.melt_kg_m2 - candidate.event.refrozen_kg_m2);
            assert!(
                (candidate.event.complete_energy_j_m2
                    - candidate.event.terminal_unallocated_energy_j_m2
                    - candidate.event.cold_energy_change_j_m2
                    - phase_energy_j_m2)
                    .abs()
                    <= 1.0e-6
            );
            eprintln!(
                "PARENT_END_CAPTURE multiplier={} tick={} support={}..{} duration_s={} complete_q={} shortwave_q={} longwave_q={} sensible_q={} latent_q={} advected_q={} snow_soil_q={} cold_change_q={} melt_kg_m2={} terminal_liquid_kg_m2={} terminal_u={} absorbed_shortwave_w_m2={} net_longwave_w_m2={} sensible_to_air_w_m2={} vapor_to_air_kg_m2_s={} latent_to_air_j_m2={}",
                PARENT_END_OPEN_SNOW_MULTIPLIER,
                group.tick.get(),
                candidate.support.start_ns().get(),
                candidate.support.end_ns().get(),
                candidate.event.evaluated_seconds,
                candidate.event.complete_energy_j_m2,
                candidate.event.shortwave_energy_j_m2,
                candidate.event.longwave_energy_j_m2,
                candidate.event.sensible_energy_j_m2,
                candidate.event.latent_energy_j_m2,
                candidate.event.advected_energy_j_m2,
                candidate.event.snow_soil_heat_energy_j_m2,
                candidate.event.cold_energy_change_j_m2,
                candidate.event.melt_kg_m2,
                candidate.event.terminal_liquid_kg_m2,
                candidate.event.terminal_unallocated_energy_j_m2,
                terminal_boundary.aggregate_snow_absorbed_shortwave_w_m2,
                terminal_boundary.aggregate_snow_net_longwave_w_m2,
                terminal_boundary.aggregate_sensible_to_canopy_air_w_m2,
                terminal_boundary.aggregate_vapor_to_canopy_air_kg_m2_s,
                terminal_boundary.aggregate_latent_energy_to_canopy_air_j_m2,
            );
            assert_eq!(group.tick, fixture.prepared.support.end_ns());
            assert_eq!(event.tick(), fixture.prepared.support.end_ns());
            assert_eq!(outcome.7.len(), 1);
            let parcel = &outcome.7[0];
            assert_eq!(parcel.source_lane_id, candidate.lane_id);
            assert_eq!(parcel.support, candidate.support);
            assert_eq!(parcel.event_result_digest, candidate.event_result_digest);
            assert_eq!(parcel.event_ordinal, event.ordinal());
            assert_eq!(
                parcel.mass_kg_m2_tile_ground.to_bits(),
                candidate.event.terminal_liquid_kg_m2.to_bits()
            );
            let projected_specific_enthalpy = candidate.event.terminal_unallocated_energy_j_m2
                / candidate.event.terminal_liquid_kg_m2;
            let projected_temperature_k = 273.15 + projected_specific_enthalpy / 4_218.0;
            assert_eq!(
                parcel.temperature_k.to_bits(),
                projected_temperature_k.to_bits()
            );
            assert_eq!(
                parcel.specific_liquid_enthalpy_j_kg.to_bits(),
                openwepp_land_surface_energy::liquid_enthalpy_j_kg(parcel.temperature_k).to_bits()
            );
            assert!(
                (parcel.specific_liquid_enthalpy_j_kg - projected_specific_enthalpy).abs()
                    <= 1.0e-9_f64.max(1.0e-12 * projected_specific_enthalpy.abs())
            );
            assert_eq!(
                parcel.posture,
                DirectSnowStage3V11TerminalParcelPosture::Consumed
            );
            assert_eq!(parcel.receiver_destinations.len(), 2);
            assert!(parcel.receiver_destinations.windows(2).all(|pair| {
                (&pair[0].destination_ofe_id, &pair[0].destination_tile_id)
                    < (&pair[1].destination_ofe_id, &pair[1].destination_tile_id)
            }));
            assert!(
                (parcel
                    .receiver_destinations
                    .iter()
                    .map(|destination| destination.destination_fraction)
                    .sum::<f64>()
                    - 1.0)
                    .abs()
                    <= 1.0e-12
            );
            assert_eq!(outcome.1.accepted_publication_event_handoffs().len(), 3);
            assert_eq!(outcome.1.accepted_publication_event_handoffs()[0], *event);
            let receiver = &outcome.1.accepted_publication_event_handoffs()[1];
            assert_eq!(receiver.tick(), event.tick());
            assert_eq!(receiver.ordinal(), event.ordinal() + 1);
            assert_eq!(
                outcome.1.accepted_publication_event_handoffs().last(),
                outcome.2.accepted_event_receipts().last()
            );
            assert_eq!(
                receiver.beginning_owner_set_digest(),
                event.ending_owner_set_digest()
            );
            assert_eq!(
                outcome.2.accepted_until(),
                fixture.prepared.support.end_ns()
            );
            assert!(outcome.5.iter().all(|receipt| receipt.support.duration_ns() > 0));
            assert!(outcome.5.iter().all(|receipt| {
                receipt.support.start_ns() < fixture.prepared.support.end_ns()
            }));
            assert!(
                performance
                    .iter()
                    .all(|span| span.support_duration_ns > 0)
            );
            let beginning_cursor =
                fixture.attachment.committed.stage3_by_lane[&1].next_interval_index;
            assert_eq!(outcome.4[&1].next_interval_index, beginning_cursor + 1);
            assert_eq!(
                outcome.4[&1].fingerprint,
                Wb11HydrologyKernel::stage3_persistent_state_fingerprint(&outcome.4[&1])
            );
            let ending_surface = outcome
                .1
                .hydrology_frame()
                .surface_liquid_shadow
                .as_deref()
                .expect("parent-end receiving surface owner");
            let beginning_surface = fixture
                .attachment
                .committed
                .real_consumer
                .hydrology_frame()
                .surface_liquid_shadow
                .as_deref()
                .expect("parent-end beginning surface owner");
            for destination in &parcel.receiver_destinations {
                let ending = ending_surface
                    .records
                    .iter()
                    .find(|record| {
                        record.key.ofe_id.as_str() == destination.destination_ofe_id
                            && record.key.tile_id.as_str() == destination.destination_tile_id
                    })
                    .expect("parent-end ending destination store");
                let beginning = beginning_surface
                    .records
                    .iter()
                    .find(|record| record.key == ending.key)
                    .expect("parent-end beginning destination store");
                assert_eq!(
                    ending.liquid_kg_m2_tile.to_bits(),
                    (beginning.liquid_kg_m2_tile + parcel.mass_kg_m2_tile_ground).to_bits()
                );
            }
            assert!(ending_surface
                .continuations
                .iter()
                .all(|continuation| continuation.next_interval_index == 1));
            let ending_surface_transaction = ending_surface.records[0]
                .last_accepted_transaction_id
                .expect("parent-end receiver surface transaction");
            assert_eq!(ending_surface_transaction, receiver_ending_transaction);
            assert_eq!(
                ending_surface_transaction.0,
                receiver_beginning_transaction
                    .map_or(0, |transaction| transaction.0)
                    .checked_add(1)
                    .expect("parent-end surface transaction successor")
            );
            assert!(ending_surface
                .records
                .iter()
                .all(|record| record.last_accepted_transaction_id
                    == Some(ending_surface_transaction)));
            for ending in &ending_surface.continuations {
                let beginning = beginning_surface
                    .continuations
                    .iter()
                    .find(|continuation| continuation.ofe_id == ending.ofe_id)
                    .expect("parent-end beginning WB14 continuation");
                assert_eq!(
                    ending.last_accepted_transaction_id,
                    Some(ending_surface_transaction)
                );
                assert_eq!(
                    ending.cumulative_supply_m.to_bits(),
                    beginning.cumulative_supply_m.to_bits()
                );
                assert_eq!(
                    ending.cumulative_infiltration_m.to_bits(),
                    beginning.cumulative_infiltration_m.to_bits()
                );
            }
            eprintln!(
                "PARENT_END_RECEIVER tick={} event_ordinal={} receiver_ordinal={} parcel_mass={} parcel_temperature={} cursor={} direct={} split={} accepted={} rejected={}",
                group.tick.get(),
                event.ordinal(),
                receiver.ordinal(),
                outcome.7[0].mass_kg_m2_tile_ground,
                outcome.7[0].temperature_k,
                outcome.4[&1].next_interval_index,
                diagnostics.direct_trial_count,
                diagnostics.split_child_trial_count,
                diagnostics.accepted_microstep_count,
                diagnostics.rejected_candidate_count,
            );
            let committed_before = fixture.attachment.committed.clone();
            let rollback = execute_real_parent(
                &fixture,
                Some(Stage3V11FailureInjection::ParentEndTerminalReceiverCompleted),
            );
            assert!(matches!(
                rollback,
                Err(DirectSnowStage3V11AttachmentError::Identity(
                    "injected parent-end terminal receiver rollback"
                ))
            ));
            assert_eq!(fixture.attachment.committed, committed_before);
        })
        .expect("spawn parent-end receiver fixture")
        .join()
        .expect("join parent-end receiver fixture");
}

#[test]
fn terminal_provisional_publication_deferral_matches_forced_current_double_and_rejects_poisons() {
    let run = |force_current_double| {
        std::thread::Builder::new()
            .name("stage3-v11-terminal-publication-deferral".to_owned())
            .stack_size(64 * 1024 * 1024)
            .spawn(move || {
                let _short_wb14_parent =
                    crate::direct_runtime::permit_short_wb14_parent_support_for_test(
                        CHRONOLOGY_TEST_PARENT_NS,
                    );
                let fixture = real_parent_fixture(
                    ChronologyCase::short_parent_end_meltout_diagnostic(),
                    CHRONOLOGY_TEST_PARENT_NS,
                );
                force_terminal_provisional_publication_double_for_test(force_current_double);
                let _full_publication_scan = force_current_double.then(|| {
                    crate::v9_real_consumer_shadow::force_full_scan_accepted_publication_history_v1(
                    )
                });
                begin_terminal_pre_event_package_audit();
                begin_adaptive_controller_test_audit(AdaptiveControllerTestPolicyV1::default());
                begin_adaptive_comparison_test_audit();
                let outcome = execute_real_parent(&fixture, None)
                    .expect("terminal provisional publication execution");
                let controller = take_adaptive_controller_test_audit();
                let comparisons = take_adaptive_comparison_test_audit();
                let packages = take_terminal_pre_event_package_audit();
                (outcome, controller, comparisons, packages)
            })
            .expect("spawn terminal publication deferral fixture")
            .join()
            .expect("join terminal publication deferral fixture")
    };

    let (deferred, deferred_controller, deferred_comparisons, deferred_packages) = run(false);
    let (current, current_controller, current_comparisons, current_packages) = run(true);
    assert_eq!(deferred.0, current.0, "final V11 parent differs");
    assert_eq!(
        deferred
            .1
            .canonical_owner_state_bytes()
            .expect("deferred owner bytes"),
        current
            .1
            .canonical_owner_state_bytes()
            .expect("current owner bytes"),
    );
    assert_eq!(deferred.2, current.2, "final coupled clock differs");
    assert_eq!(deferred.3, current.3, "final parent candidate differs");
    assert_eq!(deferred.4, current.4, "final Stage-3 owners differ");
    assert_eq!(deferred.5, current.5, "accepted receipts differ");
    assert_eq!(deferred.6, current.6, "terminal event groups differ");
    assert_eq!(deferred.7, current.7, "terminal parcels differ");
    assert_eq!(deferred_controller, current_controller);
    assert_eq!(deferred_comparisons, current_comparisons);
    assert!(!deferred_packages.is_empty());
    assert_eq!(deferred_packages.len(), current_packages.len());

    let package = deferred_packages
        .iter()
        .find(|package| !package.terminal_events.is_empty())
        .expect("sealed terminal pre-event package");
    let authority =
        crate::v9_real_consumer_shadow::precomputed_terminal_pre_event_authority_sha256_v1(package)
            .expect("recompute terminal pre-event authority");
    assert_eq!(authority, package.pre_event_authority_sha256);
    let terminal_lane = *package
        .terminal_events
        .keys()
        .next()
        .expect("terminal event lane");

    let assert_authority_changes =
        |name: &str,
         poison: &crate::v9_real_consumer_shadow::PrecomputedTerminalAcceptedEndpointV1| {
            assert_ne!(
                crate::v9_real_consumer_shadow::precomputed_terminal_pre_event_authority_sha256_v1(
                    poison
                )
                .expect("recompute poisoned terminal authority"),
                authority,
                "{name} substitution was not bound",
            );
        };
    let mut endpoint_poison = package.clone();
    let endpoint = endpoint_poison
        .ending_stage3_by_lane
        .get_mut(&terminal_lane)
        .expect("terminal endpoint state");
    endpoint.next_interval_index += 1;
    endpoint.fingerprint = Wb11HydrologyKernel::stage3_persistent_state_fingerprint(endpoint);
    assert_authority_changes("endpoint", &endpoint_poison);

    let mut trial_poison = package.clone();
    trial_poison
        .terminal_snow_soil_trial_receipts
        .get_mut(&terminal_lane)
        .expect("terminal trial receipt")
        .receipt_sha256 = digest(0x71);
    assert_authority_changes("trial", &trial_poison);

    let mut event_poison = package.clone();
    event_poison
        .terminal_events
        .values_mut()
        .next()
        .expect("terminal event")
        .terminal_liquid_kg_m2 += 1.0e-9;
    assert_authority_changes("event", &event_poison);

    let mut parcel_poison = package.clone();
    let mut parcel = deferred.7.first().expect("terminal parcel").clone();
    parcel.parcel_digest = digest(0x74);
    parcel_poison
        .beginning_pending_terminal_parcels
        .insert(digest(0x75), parcel);
    assert_authority_changes("parcel", &parcel_poison);

    let mut owner_poison = package.clone();
    owner_poison.beginning_owner_set_sha256 = digest(0x72);
    assert_authority_changes("owner", &owner_poison);

    let mut accepted_slab_poison = package.clone();
    accepted_slab_poison.accepted_slab_sha256 = digest(0x76);
    assert_authority_changes("accepted slab", &accepted_slab_poison);

    let mut wb14_child_poison = package.clone();
    wb14_child_poison.wb14_child_receipt_set_sha256 = digest(0x77);
    assert_authority_changes("WB14 child authorization", &wb14_child_poison);

    let mut wb14_parent_poison = package.clone();
    wb14_parent_poison.wb14_parent_receipt_set_sha256 = Some(digest(0x78));
    assert_authority_changes("WB14 parent authorization", &wb14_parent_poison);

    let mut authorization_poison = package.clone();
    authorization_poison.pre_event_authority_sha256 = digest(0x73);
    assert_ne!(
        crate::v9_real_consumer_shadow::precomputed_terminal_pre_event_authority_sha256_v1(
            &authorization_poison
        )
        .expect("recompute authorization substitution"),
        authorization_poison.pre_event_authority_sha256,
    );

    let lane = terminal_lane;
    let second_lane = lane + 1;
    let mut order_poison = package.clone();
    let mut second_event = order_poison.terminal_events[&lane];
    second_event.terminal_liquid_kg_m2 += 2.0e-9;
    order_poison
        .terminal_events
        .insert(second_lane, second_event);
    order_poison.ending_stage3_by_lane.insert(
        second_lane,
        order_poison.ending_stage3_by_lane[&lane].clone(),
    );
    order_poison.terminal_snow_soil_trial_receipts.insert(
        second_lane,
        order_poison.terminal_snow_soil_trial_receipts[&lane].clone(),
    );
    order_poison
        .terminal_snow_soil_trial_receipt_chains_by_lane
        .insert(
            second_lane,
            order_poison.terminal_snow_soil_trial_receipt_chains_by_lane[&lane].clone(),
        );
    order_poison
        .final_child_actual_vapor_to_canopy_air_kg_m2_by_lane
        .insert(
            second_lane,
            order_poison.final_child_actual_vapor_to_canopy_air_kg_m2_by_lane[&lane],
        );
    let ordered_authority =
        crate::v9_real_consumer_shadow::precomputed_terminal_pre_event_authority_sha256_v1(
            &order_poison,
        )
        .expect("ordered two-lane terminal authority");
    let first_event = order_poison.terminal_events[&lane];
    order_poison.terminal_events.insert(lane, second_event);
    order_poison
        .terminal_events
        .insert(second_lane, first_event);
    assert_ne!(
        crate::v9_real_consumer_shadow::precomputed_terminal_pre_event_authority_sha256_v1(
            &order_poison
        )
        .expect("reordered two-lane terminal authority"),
        ordered_authority,
    );
}

#[test]
#[ignore = "optimized 1,800-second production-parent qualification"]
fn production_constructor_deposition_meltout_1800s_qualification() {
    std::thread::Builder::new()
        .name("stage3-v11-production-parent-qualification".to_owned())
        .stack_size(64 * 1024 * 1024)
        .spawn(|| {
            let fixture = real_parent_fixture(
                ChronologyCase::deposition_meltout(),
                STAGE3_V11_PARENT_SUPPORT_NS,
            );
            let outcome = execute_real_parent(&fixture, None)
                .expect("1,800-second production parent chronology");
            assert_eq!(outcome.6.len(), 1);
            assert_eq!(
                outcome.6[0]
                    .accepted_event_receipt
                    .as_ref()
                    .expect("production terminal event")
                    .tick(),
                outcome.6[0].tick,
            );
            assert_eq!(outcome.7.len(), 1);
            assert_eq!(
                outcome.7[0].posture,
                DirectSnowStage3V11TerminalParcelPosture::Consumed,
            );
        })
        .expect("spawn production parent qualification")
        .join()
        .expect("join production parent qualification");
}
