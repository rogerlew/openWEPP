use openwepp_kernel_contract::{ResourceOwnerId, TileId};
use openwepp_land_surface_energy::{
    CondensationCredit, SourceId, StandGroundWaterAmountBasis, SurfaceClass,
};

use super::*;
use crate::direct_runtime::{
    DirectSurfaceLiquidOfeBinding, apply_surface_liquid_resource_phase,
    authorize_surface_liquid_withdrawals,
};

fn owner(value: &str) -> ResourceOwnerId {
    ResourceOwnerId::try_new(value).expect("owner")
}

fn ofe(value: &str) -> OfeId {
    OfeId::try_new(value).expect("OFE")
}

fn tile(value: &str) -> TileId {
    TileId::try_new(value).expect("tile")
}

fn surface(value: &str) -> SurfaceId {
    SurfaceId::try_new(value).expect("surface")
}

fn source(value: &str) -> SourceId {
    SourceId::try_new(value).expect("source")
}

fn binding(ofe_name: &str, lane_index: usize) -> DirectSurfaceLiquidOfeBinding {
    let top_layer = SoilLayerId::try_new(format!("soil-{ofe_name}-top")).expect("soil layer");
    DirectSurfaceLiquidOfeBinding {
        ofe_id: ofe(ofe_name),
        production_lane_index: lane_index,
        production_lane_id: u32::try_from(lane_index + 1).expect("lane id"),
        ordered_soil_layer_ids: vec![top_layer.clone()],
        infiltration_soil_thermal_layer_id: top_layer,
    }
}

fn config_record(
    ofe_name: &str,
    tile_name: &str,
    area: f64,
    capacity: f64,
    mode: DirectGroundIngressMode,
    destination: Option<(&str, &str)>,
) -> DirectSurfaceLiquidConfigurationRecord {
    DirectSurfaceLiquidConfigurationRecord {
        key: DirectSurfaceLiquidStoreKey {
            run_id: 91,
            ofe_id: ofe(ofe_name),
            tile_id: tile(tile_name),
            surface_id: surface(&format!("surface-{tile_name}")),
            surface_class: SurfaceClass::BareMineralSoil,
            source_type: openwepp_land_surface_energy::WaterSourceType::SurfaceLiquid,
            source_id: source(&format!("source-{tile_name}")),
        },
        tile_fraction: 1.0,
        capacity_kg_m2_tile: capacity,
        ofe_area_m2: area,
        ground_ingress_mode: mode,
        runon_destination_ofe_id: destination.map(|row| ofe(row.0)),
        runon_destination_tile_id: destination.map(|row| tile(row.1)),
    }
}

fn routed_configuration() -> DirectSurfaceLiquidConfiguration {
    DirectSurfaceLiquidConfiguration::new(
        owner("surface-water"),
        91,
        vec![ofe("upper"), ofe("lower")],
        vec![binding("upper", 0), binding("lower", 1)],
        vec![
            config_record(
                "upper",
                "upper-tile",
                100.0,
                0.1,
                DirectGroundIngressMode::OpenRawPrecipitation,
                Some(("lower", "lower-tile")),
            ),
            config_record(
                "lower",
                "lower-tile",
                200.0,
                0.1,
                DirectGroundIngressMode::OpenRawPrecipitation,
                None,
            ),
        ],
    )
    .expect("configuration")
}

fn mixed_kind_routed_configuration() -> DirectSurfaceLiquidConfiguration {
    DirectSurfaceLiquidConfiguration::new(
        owner("surface-water"),
        91,
        vec![ofe("upper"), ofe("lower")],
        vec![binding("upper", 0), binding("lower", 1)],
        vec![
            config_record(
                "upper",
                "upper-tile",
                100.0,
                0.1,
                DirectGroundIngressMode::CoveredCanopyRelease,
                Some(("lower", "lower-tile")),
            ),
            config_record(
                "lower",
                "lower-tile",
                250.0,
                0.1,
                DirectGroundIngressMode::OpenRawPrecipitation,
                None,
            ),
        ],
    )
    .expect("mixed-kind routed configuration")
}

fn three_ofe_configuration() -> DirectSurfaceLiquidConfiguration {
    DirectSurfaceLiquidConfiguration::new(
        owner("surface-water"),
        91,
        vec![ofe("upper"), ofe("middle"), ofe("lower")],
        vec![
            binding("upper", 0),
            binding("middle", 1),
            binding("lower", 2),
        ],
        vec![
            config_record(
                "upper",
                "upper-tile",
                100.0,
                0.1,
                DirectGroundIngressMode::OpenRawPrecipitation,
                Some(("middle", "middle-tile")),
            ),
            config_record(
                "middle",
                "middle-tile",
                100.0,
                0.1,
                DirectGroundIngressMode::OpenRawPrecipitation,
                Some(("lower", "lower-tile")),
            ),
            config_record(
                "lower",
                "lower-tile",
                100.0,
                0.1,
                DirectGroundIngressMode::OpenRawPrecipitation,
                None,
            ),
        ],
    )
    .expect("three-OFE configuration")
}

fn multi_tile_one_ofe_configuration() -> DirectSurfaceLiquidConfiguration {
    let mut first = config_record(
        "only",
        "tile-a",
        100.0,
        0.1,
        DirectGroundIngressMode::OpenRawPrecipitation,
        None,
    );
    first.tile_fraction = 0.5;
    let mut second = config_record(
        "only",
        "tile-b",
        100.0,
        0.1,
        DirectGroundIngressMode::OpenRawPrecipitation,
        None,
    );
    second.tile_fraction = 0.5;
    DirectSurfaceLiquidConfiguration::new(
        owner("surface-water"),
        91,
        vec![ofe("only")],
        vec![binding("only", 0)],
        vec![first, second],
    )
    .expect("multi-tile configuration")
}

pub(super) fn one_tile_configuration(
    mode: DirectGroundIngressMode,
) -> DirectSurfaceLiquidConfiguration {
    DirectSurfaceLiquidConfiguration::new(
        owner("surface-water"),
        91,
        vec![ofe("only")],
        vec![binding("only", 0)],
        vec![config_record("only", "tile", 100.0, 0.1, mode, None)],
    )
    .expect("configuration")
}

pub(super) fn initial_state(
    configuration: &DirectSurfaceLiquidConfiguration,
    fraction_of_capacity: f64,
) -> DirectSurfaceLiquidOwnedState {
    let liquid = configuration
        .records
        .iter()
        .map(|record| {
            (
                record.key.clone(),
                fraction_of_capacity * record.capacity_kg_m2_tile,
            )
        })
        .collect();
    DirectSurfaceLiquidOwnedState::new_initial(configuration, &liquid, 3).expect("initial state")
}

pub(super) fn resource_candidate(
    configuration: &DirectSurfaceLiquidConfiguration,
    beginning: &DirectSurfaceLiquidOwnedState,
    transaction_id: TransactionId,
    predecessor: Option<TransactionId>,
    condensation: &[CondensationCredit],
) -> DirectSurfaceLiquidResourceCandidate {
    let arbitration = authorize_surface_liquid_withdrawals(
        configuration,
        beginning,
        transaction_id,
        predecessor,
        &[],
    )
    .expect("empty authorization");
    apply_surface_liquid_resource_phase(configuration, &arbitration, &[], condensation)
        .expect("resource candidate")
}

fn amount(mass: f64, temperature_k: f64, start_s: f64, end_s: f64) -> DirectIngressAmount {
    DirectIngressAmount {
        mass_kg_m2_tile_ground: mass,
        temperature_k,
        specific_liquid_enthalpy_j_kg: liquid_specific_enthalpy(temperature_k),
        start_s,
        end_s,
    }
}

pub(super) fn open_ingress(
    record: &DirectSurfaceLiquidConfigurationRecord,
    mass: f64,
) -> DirectTileGroundIngress {
    DirectTileGroundIngress::OpenRawPrecipitation {
        ofe_id: record.key.ofe_id.clone(),
        tile_id: record.key.tile_id.clone(),
        surface_id: record.key.surface_id.clone(),
        raw_precipitation: amount(mass, 285.0, 0.0, INTERVAL_S),
    }
}

pub(super) fn parameters(
    configuration: &DirectSurfaceLiquidConfiguration,
) -> Vec<DirectOfeWb14Parameters> {
    configuration
        .ofe_topology
        .iter()
        .map(|ofe_id| DirectOfeWb14Parameters {
            ofe_id: ofe_id.clone(),
            effective_conductivity_m_s: 1.0e-6,
            matric_potential_m: 0.1,
            infiltration_storage_capacity_m: 0.0,
        })
        .collect()
}

#[test]
fn unequal_area_runoff_routes_once_and_preserves_mass_and_enthalpy() {
    let configuration = routed_configuration();
    let beginning = initial_state(&configuration, 1.0);
    let transaction_id = TransactionId(201);
    let resource = resource_candidate(&configuration, &beginning, transaction_id, None, &[]);
    let input = DirectSurfaceLiquidIngressInput {
        transaction_id,
        day_index: 3,
        interval_index: 0,
        interval_s: INTERVAL_S,
        tile_ingress: vec![
            open_ingress(&configuration.records[0], 1.0),
            open_ingress(&configuration.records[1], 0.0),
        ],
        wb14_parameters: parameters(&configuration),
    };
    let candidate = execute_surface_liquid_ingress(&configuration, &resource, &input)
        .expect("routed candidate");
    assert_eq!(candidate.wb14_calls_by_ofe.values().copied().sum::<u8>(), 2);
    assert!(
        candidate
            .ending_state
            .records
            .iter()
            .zip(&beginning.records)
            .all(|(ending, start)| ending.liquid_kg_m2_tile.to_bits()
                == start.liquid_kg_m2_tile.to_bits())
    );
    assert_eq!(
        candidate.ending_state.continuations[0].next_interval_index,
        1
    );
    assert_eq!(
        candidate.ending_state.continuations[1].next_interval_index,
        1
    );
    let routed = candidate
        .receipts
        .iter()
        .find(|row| row.disposition == DirectSurfaceLiquidReceiptDisposition::RoutedRunoff)
        .expect("routed receipt");
    let outlet = candidate
        .receipts
        .iter()
        .find(|row| row.disposition == DirectSurfaceLiquidReceiptDisposition::OutletRunoff)
        .expect("outlet receipt");
    assert_eq!(routed.transaction_id, transaction_id);
    assert_eq!(
        routed.mass_kg_m2_basis_ofe_ground.to_bits(),
        1.0_f64.to_bits()
    );
    assert_eq!(
        outlet.mass_kg_m2_basis_ofe_ground.to_bits(),
        0.5_f64.to_bits()
    );
    assert_eq!(outlet.basis_ofe_id, ofe("lower"));
    assert_eq!(
        outlet.enthalpy_j_m2_basis_ofe_ground.to_bits(),
        (routed.enthalpy_j_m2_basis_ofe_ground * 0.5).to_bits()
    );
    candidate
        .ending_state
        .validate(&configuration)
        .expect("ending state");
}

#[test]
#[allow(clippy::too_many_lines)]
fn mixed_kinds_unequal_area_and_downstream_local_overlap_preserve_canonical_mix() {
    let configuration = mixed_kind_routed_configuration();
    let beginning = initial_state(&configuration, 0.5);
    let transaction_id = TransactionId(413);
    let resource = resource_candidate(&configuration, &beginning, transaction_id, None, &[]);
    let zero = amount(0.0, 285.0, 0.0, INTERVAL_S);
    let mut input = DirectSurfaceLiquidIngressInput {
        transaction_id,
        day_index: 3,
        interval_index: 0,
        interval_s: INTERVAL_S,
        tile_ingress: vec![
            DirectTileGroundIngress::CoveredCanopyRelease {
                ofe_id: ofe("upper"),
                tile_id: tile("upper-tile"),
                surface_id: surface("surface-upper-tile"),
                release: DirectCanopyLiquidRelease {
                    throughfall: amount(0.4, 280.0, 0.0, INTERVAL_S),
                    initial_drainage: amount(0.2, 305.0, 0.0, INTERVAL_S),
                    second_drainage: zero.clone(),
                    stemflow: zero,
                },
            },
            open_ingress(&configuration.records[1], 0.3),
        ],
        wb14_parameters: parameters(&configuration)
            .into_iter()
            .map(|mut row| {
                row.infiltration_storage_capacity_m = 5.0e-5;
                row
            })
            .collect(),
    };
    let DirectTileGroundIngress::OpenRawPrecipitation {
        raw_precipitation, ..
    } = &mut input.tile_ingress[1]
    else {
        panic!("open lower input");
    };
    *raw_precipitation = amount(0.3, 292.0, 600.0, 1_500.0);

    let candidate = execute_surface_liquid_ingress(&configuration, &resource, &input)
        .expect("mixed-kind routed candidate");
    candidate
        .validate(&configuration, &resource, &input)
        .expect("independent mixed-kind closure");
    let actual = candidate
        .receipts
        .iter()
        .map(|receipt| {
            (
                receipt.source_parcel_id.clone(),
                receipt.basis_ofe_id.clone(),
                receipt.kind,
                receipt.disposition,
                receipt.recipient.clone(),
                receipt.start_s.to_bits(),
                receipt.end_s.to_bits(),
                receipt.mass_kg_m2_basis_ofe_ground.to_bits(),
                receipt.temperature_k.to_bits(),
                receipt.enthalpy_j_m2_basis_ofe_ground.to_bits(),
            )
        })
        .collect::<Vec<_>>();
    let upper_key = configuration.records[0].key.clone();
    let lower_key = configuration.records[1].key.clone();
    let upper_binding = binding("upper", 0);
    let lower_binding = binding("lower", 1);
    let upper_soil = DirectSurfaceLiquidReceiptRecipient::SoilInfiltration {
        ofe_id: ofe("upper"),
        production_lane_index: upper_binding.production_lane_index,
        production_lane_id: upper_binding.production_lane_id,
        ordered_soil_layer_ids: upper_binding.ordered_soil_layer_ids,
        soil_thermal_layer_id: upper_binding.infiltration_soil_thermal_layer_id,
    };
    let lower_soil = DirectSurfaceLiquidReceiptRecipient::SoilInfiltration {
        ofe_id: ofe("lower"),
        production_lane_index: lower_binding.production_lane_index,
        production_lane_id: lower_binding.production_lane_id,
        ordered_soil_layer_ids: lower_binding.ordered_soil_layer_ids,
        soil_thermal_layer_id: lower_binding.infiltration_soil_thermal_layer_id,
    };
    let upper_store = DirectSurfaceLiquidReceiptRecipient::SurfaceStore {
        store_key: upper_key.clone(),
    };
    let lower_store = DirectSurfaceLiquidReceiptRecipient::SurfaceStore {
        store_key: lower_key.clone(),
    };
    let routed = DirectSurfaceLiquidReceiptRecipient::RoutedOfe {
        source_ofe_id: ofe("upper"),
        destination_ofe_id: ofe("lower"),
        destination_store_key: lower_key.clone(),
    };
    let outlet = DirectSurfaceLiquidReceiptRecipient::Outlet {
        ofe_id: ofe("lower"),
    };
    let throughfall = "local:OfeId(\"upper\"):TileId(\"upper-tile\"):CanopyThroughfall";
    let drainage = "local:OfeId(\"upper\"):TileId(\"upper-tile\"):CanopyInitialDrainage";
    let precipitation = "local:OfeId(\"lower\"):TileId(\"lower-tile\"):RawPrecipitation";
    macro_rules! expected_row {
        ($source:expr, $basis:literal, $kind:ident, $disposition:ident, $recipient:expr,
         $start:expr, $end:expr, $mass:expr, $temperature:expr, $enthalpy:expr) => {
            (
                $source.to_owned(),
                ofe($basis),
                DirectSurfaceLiquidParcelKind::$kind,
                DirectSurfaceLiquidReceiptDisposition::$disposition,
                $recipient.clone(),
                $start,
                $end,
                $mass,
                $temperature,
                $enthalpy,
            )
        };
    }
    let expected = vec![
        expected_row!(
            throughfall,
            "upper",
            CanopyThroughfall,
            Infiltration,
            upper_soil,
            0,
            4_655_631_299_166_339_072,
            4_584_964_660_638_322_963,
            4_643_780_029_834_417_493,
            4_656_912_838_609_132_157
        ),
        expected_row!(
            drainage,
            "upper",
            CanopyInitialDrainage,
            Infiltration,
            upper_soil,
            0,
            4_655_631_299_166_339_072,
            4_580_461_061_010_952_468,
            4_643_780_029_834_417_493,
            4_652_409_238_981_761_662
        ),
        expected_row!(
            throughfall,
            "upper",
            CanopyThroughfall,
            RetainedSurface,
            upper_store,
            0,
            4_655_631_299_166_339_072,
            4_584_964_660_638_322_962,
            4_643_780_029_834_417_493,
            4_656_912_838_609_132_156
        ),
        expected_row!(
            drainage,
            "upper",
            CanopyInitialDrainage,
            RetainedSurface,
            upper_store,
            0,
            4_655_631_299_166_339_072,
            4_580_461_061_010_952_464,
            4_643_780_029_834_417_493,
            4_652_409_238_981_761_658
        ),
        expected_row!(
            throughfall,
            "upper",
            CanopyThroughfall,
            RoutedRunoff,
            routed,
            0,
            4_655_631_299_166_339_072,
            4_599_676_419_421_066_582,
            4_643_780_029_834_417_493,
            4_671_597_243_375_096_090
        ),
        expected_row!(
            drainage,
            "upper",
            CanopyInitialDrainage,
            RoutedRunoff,
            routed,
            0,
            4_655_631_299_166_339_072,
            4_595_172_819_793_696_086,
            4_643_780_029_834_417_493,
            4_667_093_643_747_725_594
        ),
        expected_row!(
            drainage,
            "lower",
            UpstreamRunon,
            Infiltration,
            lower_soil,
            0,
            4_648_488_871_632_306_176,
            4_580_461_061_010_952_465,
            4_643_780_029_834_417_493,
            4_652_409_238_981_761_657
        ),
        expected_row!(
            throughfall,
            "lower",
            UpstreamRunon,
            Infiltration,
            lower_soil,
            0,
            4_648_488_871_632_306_176,
            4_584_964_660_638_322_962,
            4_643_780_029_834_417_493,
            4_656_912_838_609_132_154
        ),
        expected_row!(
            drainage,
            "lower",
            UpstreamRunon,
            RetainedSurface,
            lower_store,
            0,
            4_648_488_871_632_306_176,
            4_573_055_141_623_720_988,
            4_643_780_029_834_417_493,
            4_644_966_847_572_157_265
        ),
        expected_row!(
            throughfall,
            "lower",
            UpstreamRunon,
            RetainedSurface,
            lower_store,
            0,
            4_648_488_871_632_306_176,
            4_577_558_741_251_091_480,
            4_643_780_029_834_417_493,
            4_649_470_447_199_527_757
        ),
        expected_row!(
            drainage,
            "lower",
            UpstreamRunon,
            Infiltration,
            lower_soil,
            4_648_488_871_632_306_176,
            4_654_311_885_213_007_872,
            0,
            4_643_828_408_346_039_637,
            0
        ),
        expected_row!(
            throughfall,
            "lower",
            UpstreamRunon,
            Infiltration,
            lower_soil,
            4_648_488_871_632_306_176,
            4_654_311_885_213_007_872,
            0,
            4_643_828_408_346_039_637,
            0
        ),
        expected_row!(
            precipitation,
            "lower",
            RawPrecipitation,
            Infiltration,
            lower_soil,
            4_648_488_871_632_306_176,
            4_654_311_885_213_007_872,
            0,
            4_643_828_408_346_039_637,
            0
        ),
        expected_row!(
            drainage,
            "lower",
            UpstreamRunon,
            RetainedSurface,
            lower_store,
            4_648_488_871_632_306_176,
            4_654_311_885_213_007_872,
            4_568_551_541_996_350_486,
            4_643_828_408_346_039_637,
            4_641_596_917_733_799_006
        ),
        expected_row!(
            throughfall,
            "lower",
            UpstreamRunon,
            RetainedSurface,
            lower_store,
            4_648_488_871_632_306_176,
            4_654_311_885_213_007_872,
            4_573_055_141_623_720_982,
            4_643_828_408_346_039_637,
            4_646_100_517_361_169_502
        ),
        expected_row!(
            precipitation,
            "lower",
            RawPrecipitation,
            RetainedSurface,
            lower_store,
            4_648_488_871_632_306_176,
            4_654_311_885_213_007_872,
            4_582_862_980_812_216_728,
            4_643_828_408_346_039_637,
            4_656_031_829_262_105_321
        ),
        expected_row!(
            drainage,
            "lower",
            UpstreamRunon,
            Infiltration,
            lower_soil,
            4_654_311_885_213_007_872,
            4_655_631_299_166_339_072,
            0,
            4_643_780_029_834_417_493,
            0
        ),
        expected_row!(
            throughfall,
            "lower",
            UpstreamRunon,
            Infiltration,
            lower_soil,
            4_654_311_885_213_007_872,
            4_655_631_299_166_339_072,
            0,
            4_643_780_029_834_417_493,
            0
        ),
        expected_row!(
            drainage,
            "lower",
            UpstreamRunon,
            OutletRunoff,
            outlet,
            4_648_488_871_632_306_176,
            4_654_311_885_213_007_872,
            4_584_464_260_679_726_241,
            4_643_828_408_346_039_637,
            4_657_301_034_627_793_922
        ),
        expected_row!(
            throughfall,
            "lower",
            UpstreamRunon,
            OutletRunoff,
            outlet,
            4_648_488_871_632_306_176,
            4_654_311_885_213_007_872,
            4_588_967_860_307_096_737,
            4_643_828_408_346_039_637,
            4_661_804_634_255_164_418
        ),
        expected_row!(
            precipitation,
            "lower",
            RawPrecipitation,
            OutletRunoff,
            outlet,
            4_648_488_871_632_306_176,
            4_654_311_885_213_007_872,
            4_598_625_579_508_013_466,
            4_643_828_408_346_039_637,
            4_671_447_160_954_164_354
        ),
        expected_row!(
            drainage,
            "lower",
            UpstreamRunon,
            OutletRunoff,
            outlet,
            4_654_311_885_213_007_872,
            4_655_631_299_166_339_072,
            4_577_558_741_251_091_480,
            4_643_780_029_834_417_493,
            4_649_470_447_199_527_757
        ),
        expected_row!(
            throughfall,
            "lower",
            UpstreamRunon,
            OutletRunoff,
            outlet,
            4_654_311_885_213_007_872,
            4_655_631_299_166_339_072,
            4_582_062_340_878_461_976,
            4_643_780_029_834_417_493,
            4_653_974_046_826_898_253
        ),
    ];
    assert_eq!(actual, expected);
    assert_eq!(
        candidate
            .ending_state
            .records
            .iter()
            .map(|row| (
                row.key.clone(),
                row.liquid_kg_m2_tile.to_bits(),
                row.last_accepted_transaction_id,
            ))
            .collect::<Vec<_>>(),
        vec![
            (upper_key, 0.1_f64.to_bits(), Some(transaction_id)),
            (lower_key, 0.1_f64.to_bits(), Some(transaction_id)),
        ]
    );
    assert_eq!(
        candidate
            .ending_state
            .continuations
            .iter()
            .map(|row| (
                row.ofe_id.clone(),
                row.day_index,
                row.next_interval_index,
                row.cumulative_supply_m.to_bits(),
                row.cumulative_infiltration_m.to_bits(),
                row.last_accepted_transaction_id,
            ))
            .collect::<Vec<_>>(),
        vec![
            (
                ofe("upper"),
                3,
                1,
                4_558_673_246_493_684_322,
                4_542_503_522_391_573_293,
                Some(transaction_id),
            ),
            (
                ofe("lower"),
                3,
                1,
                4_557_750_909_289_998_844,
                4_542_503_522_391_573_293,
                Some(transaction_id),
            ),
        ]
    );

    input.tile_ingress.reverse();
    let reordered = execute_surface_liquid_ingress(&configuration, &resource, &input)
        .expect("caller-reordered mixed-kind candidate");
    assert_eq!(candidate.receipts, reordered.receipts);
    assert_eq!(candidate.ending_state, reordered.ending_state);
    assert_eq!(candidate.closure_operands, reordered.closure_operands);
}

#[test]
#[allow(clippy::too_many_lines)]
fn partial_support_routes_independently_across_multiple_hops() {
    let configuration = three_ofe_configuration();
    let beginning = initial_state(&configuration, 1.0);
    let transaction_id = TransactionId(408);
    let resource = resource_candidate(&configuration, &beginning, transaction_id, None, &[]);
    let mut input = DirectSurfaceLiquidIngressInput {
        transaction_id,
        day_index: 3,
        interval_index: 0,
        interval_s: INTERVAL_S,
        tile_ingress: vec![
            open_ingress(&configuration.records[0], 1.0),
            open_ingress(&configuration.records[1], 0.0),
            open_ingress(&configuration.records[2], 0.0),
        ],
        wb14_parameters: parameters(&configuration),
    };
    let DirectTileGroundIngress::OpenRawPrecipitation {
        raw_precipitation, ..
    } = &mut input.tile_ingress[0]
    else {
        panic!("open input");
    };
    *raw_precipitation = amount(1.0, 287.0, 300.0, 1_500.0);

    let candidate = execute_surface_liquid_ingress(&configuration, &resource, &input)
        .expect("partial-support multi-hop candidate");
    candidate
        .validate(&configuration, &resource, &input)
        .expect("independently projected multi-hop route");
    let source_id = candidate
        .closure_operands
        .source_parcels()
        .iter()
        .find(|source| source.basis_ofe_id() == &ofe("upper"))
        .expect("upper source")
        .source_parcel_id()
        .to_owned();
    let routed = candidate
        .receipts
        .iter()
        .filter(|receipt| {
            receipt.source_parcel_id == source_id
                && receipt.disposition == DirectSurfaceLiquidReceiptDisposition::RoutedRunoff
        })
        .collect::<Vec<_>>();
    assert_eq!(routed.len(), 2);
    assert!(routed.iter().all(|receipt| {
        receipt.start_s.to_bits() == 300.0_f64.to_bits()
            && receipt.end_s.to_bits() == 1_500.0_f64.to_bits()
    }));
    assert!(candidate.receipts.iter().any(|receipt| {
        receipt.source_parcel_id == source_id
            && receipt.basis_ofe_id == ofe("lower")
            && receipt.disposition == DirectSurfaceLiquidReceiptDisposition::OutletRunoff
    }));

    let mut disposition_drift = candidate.clone();
    disposition_drift
        .receipts
        .iter_mut()
        .find(|receipt| {
            receipt.source_parcel_id == source_id
                && receipt.basis_ofe_id == ofe("middle")
                && receipt.disposition == DirectSurfaceLiquidReceiptDisposition::RoutedRunoff
        })
        .expect("first routed receipt")
        .disposition = DirectSurfaceLiquidReceiptDisposition::RetainedSurface;
    let disposition_error =
        super::super::surface_liquid_closure::validate_surface_liquid_closure_operands(
            &configuration,
            &resource,
            &disposition_drift.closure_operands,
            &disposition_drift.receipts,
            &disposition_drift.ending_state,
        )
        .expect_err("actual routed disposition drift");
    assert_eq!(disposition_error.code(), DirectSurfaceLiquidErrorCode::E010);

    let mut kind_drift = candidate.clone();
    kind_drift
        .receipts
        .iter_mut()
        .find(|receipt| {
            receipt.source_parcel_id == source_id
                && receipt.basis_ofe_id == ofe("middle")
                && receipt.kind == DirectSurfaceLiquidParcelKind::UpstreamRunon
        })
        .expect("downstream runon receipt")
        .kind = DirectSurfaceLiquidParcelKind::RawPrecipitation;
    let kind_error =
        super::super::surface_liquid_closure::validate_surface_liquid_closure_operands(
            &configuration,
            &resource,
            &kind_drift.closure_operands,
            &kind_drift.receipts,
            &kind_drift.ending_state,
        )
        .expect_err("routed descendant kind drift");
    assert_eq!(kind_error.code(), DirectSurfaceLiquidErrorCode::E010);

    let mut drift = candidate;
    let routed_receipt = drift
        .receipts
        .iter_mut()
        .find(|receipt| {
            receipt.source_parcel_id == source_id
                && receipt.basis_ofe_id == ofe("middle")
                && receipt.disposition == DirectSurfaceLiquidReceiptDisposition::RoutedRunoff
        })
        .expect("first routed receipt");
    routed_receipt.mass_kg_m2_basis_ofe_ground *= 0.9;
    routed_receipt.enthalpy_j_m2_basis_ofe_ground *= 0.9;
    let error = super::super::surface_liquid_closure::validate_surface_liquid_closure_operands(
        &configuration,
        &resource,
        &drift.closure_operands,
        &drift.receipts,
        &drift.ending_state,
    )
    .expect_err("actual routed receipt drift");
    assert_eq!(error.code(), DirectSurfaceLiquidErrorCode::E010);
}

#[test]
fn open_and_covered_ingress_are_structurally_exclusive() {
    let configuration = one_tile_configuration(DirectGroundIngressMode::OpenRawPrecipitation);
    let beginning = initial_state(&configuration, 0.0);
    let transaction_id = TransactionId(202);
    let resource = resource_candidate(&configuration, &beginning, transaction_id, None, &[]);
    let zero = amount(0.0, 280.0, 0.0, INTERVAL_S);
    let input = DirectSurfaceLiquidIngressInput {
        transaction_id,
        day_index: 3,
        interval_index: 0,
        interval_s: INTERVAL_S,
        tile_ingress: vec![DirectTileGroundIngress::CoveredCanopyRelease {
            ofe_id: configuration.records[0].key.ofe_id.clone(),
            tile_id: configuration.records[0].key.tile_id.clone(),
            surface_id: configuration.records[0].key.surface_id.clone(),
            release: DirectCanopyLiquidRelease {
                throughfall: zero.clone(),
                initial_drainage: zero.clone(),
                second_drainage: zero.clone(),
                stemflow: zero,
            },
        }],
        wb14_parameters: parameters(&configuration),
    };
    let before = resource.clone();
    let error = execute_surface_liquid_ingress(&configuration, &resource, &input)
        .expect_err("wrong ingress mode");
    let failure = error.failure().expect("canonical failure");
    assert_eq!(failure.code, DirectSurfaceLiquidErrorCode::E002);
    assert_eq!(failure.phase, DirectSurfaceLiquidPhase::IngressCandidate);
    assert_eq!(resource, before);
    assert_eq!(&beginning, resource.beginning_state());
}

#[test]
fn condensation_overflow_becomes_timed_outlet_parcel() {
    let configuration = one_tile_configuration(DirectGroundIngressMode::OpenRawPrecipitation);
    let beginning = initial_state(&configuration, 1.0);
    let transaction_id = TransactionId(203);
    let temperature_k = 286.0;
    let condensation = CondensationCredit {
        transaction_id,
        hydrology_owner_id: configuration.owner_id.clone(),
        ofe_id: configuration.records[0].key.ofe_id.clone(),
        tile_id: configuration.records[0].key.tile_id.clone(),
        surface_id: configuration.records[0].key.surface_id.clone(),
        amount_kg_m2_stand_ground: 0.2,
        amount_basis: StandGroundWaterAmountBasis::KgH2oM2StandGroundInterval,
        temperature_k,
        specific_liquid_enthalpy_j_kg: liquid_specific_enthalpy(temperature_k),
    };
    let resource = resource_candidate(
        &configuration,
        &beginning,
        transaction_id,
        None,
        &[condensation],
    );
    assert_eq!(resource.condensation_overflow().len(), 1);
    let input = DirectSurfaceLiquidIngressInput {
        transaction_id,
        day_index: 3,
        interval_index: 0,
        interval_s: INTERVAL_S,
        tile_ingress: vec![open_ingress(&configuration.records[0], 0.0)],
        wb14_parameters: parameters(&configuration),
    };
    let candidate = execute_surface_liquid_ingress(&configuration, &resource, &input)
        .expect("condensation ingress");
    let outlet = candidate
        .receipts
        .iter()
        .find(|row| row.disposition == DirectSurfaceLiquidReceiptDisposition::OutletRunoff)
        .expect("overflow outlet");
    assert_eq!(
        outlet.kind,
        DirectSurfaceLiquidParcelKind::CondensationOverflow
    );
    assert_eq!(outlet.start_s.to_bits(), 0.0_f64.to_bits());
    assert_eq!(outlet.end_s.to_bits(), INTERVAL_S.to_bits());
    assert!(
        (outlet.mass_kg_m2_basis_ofe_ground - 0.2).abs()
            <= mass_tolerance(outlet.mass_kg_m2_basis_ofe_ground.abs() + 0.2)
    );
    assert_eq!(
        candidate.ending_state.records[0]
            .liquid_kg_m2_tile
            .to_bits(),
        0.1_f64.to_bits()
    );
}

#[test]
fn simultaneous_canopy_sources_share_retention_proportionally() {
    let configuration = one_tile_configuration(DirectGroundIngressMode::CoveredCanopyRelease);
    let beginning = initial_state(&configuration, 0.5);
    let transaction_id = TransactionId(204);
    let resource = resource_candidate(&configuration, &beginning, transaction_id, None, &[]);
    let zero = amount(0.0, 284.0, 0.0, INTERVAL_S);
    let input = DirectSurfaceLiquidIngressInput {
        transaction_id,
        day_index: 3,
        interval_index: 0,
        interval_s: INTERVAL_S,
        tile_ingress: vec![DirectTileGroundIngress::CoveredCanopyRelease {
            ofe_id: configuration.records[0].key.ofe_id.clone(),
            tile_id: configuration.records[0].key.tile_id.clone(),
            surface_id: configuration.records[0].key.surface_id.clone(),
            release: DirectCanopyLiquidRelease {
                throughfall: amount(0.2, 280.0, 0.0, INTERVAL_S),
                initial_drainage: amount(0.3, 290.0, 0.0, INTERVAL_S),
                second_drainage: zero.clone(),
                stemflow: zero,
            },
        }],
        wb14_parameters: parameters(&configuration),
    };
    let candidate =
        execute_surface_liquid_ingress(&configuration, &resource, &input).expect("covered ingress");
    let retained = candidate
        .receipts
        .iter()
        .filter(|row| row.disposition == DirectSurfaceLiquidReceiptDisposition::RetainedSurface)
        .map(|row| (row.kind, row.mass_kg_m2_basis_ofe_ground))
        .collect::<BTreeMap<_, _>>();
    assert!((retained[&DirectSurfaceLiquidParcelKind::CanopyThroughfall] - 0.02).abs() < 1e-14);
    assert!((retained[&DirectSurfaceLiquidParcelKind::CanopyInitialDrainage] - 0.03).abs() < 1e-14);
    assert!((candidate.ending_state.records[0].liquid_kg_m2_tile - 0.1).abs() < 1e-14);
    let expected_h_mix =
        (0.2 * liquid_specific_enthalpy(280.0) + 0.3 * liquid_specific_enthalpy(290.0)) / 0.5;
    let expected_temperature =
        REFERENCE_TEMPERATURE_K + expected_h_mix / LIQUID_HEAT_CAPACITY_J_KG_K;
    assert!(candidate.receipts.iter().all(|receipt| {
        (receipt.temperature_k - expected_temperature).abs() <= f64::EPSILON * 512.0
    }));
}

#[test]
fn continuation_advances_48_intervals_then_requires_new_day_zero() {
    let configuration = one_tile_configuration(DirectGroundIngressMode::OpenRawPrecipitation);
    let mut state = initial_state(&configuration, 0.0);
    let mut predecessor = None;
    for interval in 0_u8..48 {
        let transaction_id = TransactionId(300 + u128::from(interval));
        let resource = resource_candidate(&configuration, &state, transaction_id, predecessor, &[]);
        let input = DirectSurfaceLiquidIngressInput {
            transaction_id,
            day_index: 3,
            interval_index: interval,
            interval_s: INTERVAL_S,
            tile_ingress: vec![open_ingress(&configuration.records[0], 0.0)],
            wb14_parameters: parameters(&configuration),
        };
        let candidate = execute_surface_liquid_ingress(&configuration, &resource, &input)
            .expect("daily continuation");
        state = candidate.ending_state;
        predecessor = Some(transaction_id);
    }
    assert_eq!(state.continuations[0].next_interval_index, 48);
    let transaction_id = TransactionId(400);
    let resource = resource_candidate(&configuration, &state, transaction_id, predecessor, &[]);
    let stale_day = DirectSurfaceLiquidIngressInput {
        transaction_id,
        day_index: 3,
        interval_index: 0,
        interval_s: INTERVAL_S,
        tile_ingress: vec![open_ingress(&configuration.records[0], 0.0)],
        wb14_parameters: parameters(&configuration),
    };
    assert!(execute_surface_liquid_ingress(&configuration, &resource, &stale_day).is_err());
    let next_day = DirectSurfaceLiquidIngressInput {
        day_index: 4,
        ..stale_day
    };
    let candidate = execute_surface_liquid_ingress(&configuration, &resource, &next_day)
        .expect("new-day continuation");
    assert_eq!(candidate.ending_state.continuations[0].day_index, 4);
    assert_eq!(
        candidate.ending_state.continuations[0].next_interval_index,
        1
    );
    assert_eq!(
        candidate.ending_state.continuations[0]
            .cumulative_supply_m
            .to_bits(),
        0.0_f64.to_bits()
    );
    assert_eq!(
        candidate.ending_state.continuations[0]
            .cumulative_infiltration_m
            .to_bits(),
        0.0_f64.to_bits()
    );
}

#[test]
fn independent_closure_rejects_wrong_infiltration_recipient() {
    let configuration = one_tile_configuration(DirectGroundIngressMode::OpenRawPrecipitation);
    let beginning = initial_state(&configuration, 0.0);
    let transaction_id = TransactionId(390);
    let resource = resource_candidate(&configuration, &beginning, transaction_id, None, &[]);
    let input = DirectSurfaceLiquidIngressInput {
        transaction_id,
        day_index: 3,
        interval_index: 0,
        interval_s: INTERVAL_S,
        tile_ingress: vec![open_ingress(&configuration.records[0], 0.1)],
        wb14_parameters: parameters(&configuration),
    };
    let mut candidate =
        execute_surface_liquid_ingress(&configuration, &resource, &input).expect("valid candidate");
    let infiltration = candidate
        .receipts
        .iter_mut()
        .find(|receipt| receipt.disposition == DirectSurfaceLiquidReceiptDisposition::Infiltration)
        .expect("infiltration receipt");
    if let DirectSurfaceLiquidReceiptRecipient::SoilInfiltration {
        production_lane_id, ..
    } = &mut infiltration.recipient
    {
        *production_lane_id += 100;
    } else {
        panic!("wrong receipt variant");
    }
    let offending = candidate
        .receipts
        .iter()
        .find(|receipt| receipt.disposition == DirectSurfaceLiquidReceiptDisposition::Infiltration)
        .expect("offending infiltration receipt")
        .clone();
    let expected_attempted = candidate
        .ending_state
        .recomputed_sha256()
        .expect("attempted digest");
    let error = candidate
        .validate(&configuration, &resource, &input)
        .expect_err("wrong infiltration recipient");
    let failure = error.failure().expect("canonical candidate failure");
    assert_eq!(failure.code, DirectSurfaceLiquidErrorCode::E009);
    assert_eq!(failure.phase, DirectSurfaceLiquidPhase::IngressCandidate);
    assert_eq!(failure.context.transaction_id, Some(transaction_id));
    assert_eq!(
        failure.context.owner_id,
        Some(configuration.owner_id.clone())
    );
    assert_eq!(
        failure.context.ofe_id,
        Some(offending.recipient_store_key.ofe_id.clone())
    );
    assert_eq!(
        failure.context.tile_id,
        Some(offending.recipient_store_key.tile_id.clone())
    );
    assert_eq!(failure.context.parcel_id, Some(offending.parcel_id));
    assert_eq!(
        failure.rollback.beginning_owner_sha256.as_deref(),
        Some(resource.beginning_state().state_sha256.as_str())
    );
    assert_eq!(
        failure.rollback.attempted_owner_sha256.as_deref(),
        Some(expected_attempted.as_str())
    );
}

#[test]
fn independent_closure_rejects_poisoned_source_operand() {
    let configuration = one_tile_configuration(DirectGroundIngressMode::OpenRawPrecipitation);
    let beginning = initial_state(&configuration, 0.0);
    let transaction_id = TransactionId(391);
    let resource = resource_candidate(&configuration, &beginning, transaction_id, None, &[]);
    let input = DirectSurfaceLiquidIngressInput {
        transaction_id,
        day_index: 3,
        interval_index: 0,
        interval_s: INTERVAL_S,
        tile_ingress: vec![open_ingress(&configuration.records[0], 0.1)],
        wb14_parameters: parameters(&configuration),
    };
    let candidate =
        execute_surface_liquid_ingress(&configuration, &resource, &input).expect("valid candidate");
    let mut poisoned = candidate.closure_operands().clone();
    poisoned.poison_first_beginning_for_test();
    let error = super::super::surface_liquid_closure::validate_surface_liquid_closure_operands(
        &configuration,
        &resource,
        &poisoned,
        &candidate.receipts,
        &candidate.ending_state,
    )
    .expect_err("ordinary closure mismatch");
    assert_eq!(error.code(), DirectSurfaceLiquidErrorCode::E010);

    let mut producer_poison = candidate;
    producer_poison.closure_operands = poisoned;
    let expected_attempted = producer_poison
        .ending_state
        .recomputed_sha256()
        .expect("attempted digest");
    let error = producer_poison
        .validate(&configuration, &resource, &input)
        .expect_err("public producer mismatch");
    let failure = error.failure().expect("canonical public closure failure");
    assert_eq!(failure.code, DirectSurfaceLiquidErrorCode::E010);
    assert_eq!(failure.phase, DirectSurfaceLiquidPhase::IndependentClosure);
    assert_eq!(failure.context.transaction_id, Some(transaction_id));
    assert_eq!(
        failure.context.owner_id,
        Some(configuration.owner_id.clone())
    );
    assert_eq!(
        failure.context.ofe_id,
        Some(configuration.records[0].key.ofe_id.clone())
    );
    assert_eq!(
        failure.context.tile_id,
        Some(configuration.records[0].key.tile_id.clone())
    );
    assert_eq!(
        failure.rollback.beginning_owner_sha256.as_deref(),
        Some(resource.beginning_state().state_sha256.as_str())
    );
    assert_eq!(
        failure.rollback.attempted_owner_sha256.as_deref(),
        Some(expected_attempted.as_str())
    );
}

#[test]
fn independent_closure_rejects_large_finite_store_arithmetic_overflow() {
    let configuration = one_tile_configuration(DirectGroundIngressMode::OpenRawPrecipitation);
    let beginning = initial_state(&configuration, 0.0);
    let transaction_id = TransactionId(392);
    let resource = resource_candidate(&configuration, &beginning, transaction_id, None, &[]);
    let input = DirectSurfaceLiquidIngressInput {
        transaction_id,
        day_index: 3,
        interval_index: 0,
        interval_s: INTERVAL_S,
        tile_ingress: vec![open_ingress(&configuration.records[0], 0.1)],
        wb14_parameters: parameters(&configuration),
    };
    let candidate =
        execute_surface_liquid_ingress(&configuration, &resource, &input).expect("valid candidate");
    let mut poisoned = candidate.closure_operands().clone();
    poisoned.poison_first_store_arithmetic_overflow_for_test();
    let error = super::super::surface_liquid_closure::validate_surface_liquid_closure_operands(
        &configuration,
        &resource,
        &poisoned,
        &candidate.receipts,
        &candidate.ending_state,
    )
    .expect_err("large finite closure arithmetic must fail closed");
    assert_eq!(error.code(), DirectSurfaceLiquidErrorCode::E003);

    let mut producer_poison = candidate.clone();
    producer_poison
        .closure_operands
        .poison_first_store_arithmetic_overflow_for_test();
    let expected_attempted = producer_poison
        .ending_state
        .recomputed_sha256()
        .expect("attempted digest");
    let error = producer_poison
        .validate(&configuration, &resource, &input)
        .expect_err("public producer closure must retain arithmetic failure");
    let failure = error.failure().expect("canonical public closure failure");
    assert_eq!(failure.code, DirectSurfaceLiquidErrorCode::E003);
    assert_eq!(failure.context.transaction_id, Some(transaction_id));
    assert_eq!(
        failure.context.owner_id,
        Some(configuration.owner_id.clone())
    );
    assert_eq!(
        failure.context.ofe_id,
        Some(configuration.records[0].key.ofe_id.clone())
    );
    assert_eq!(
        failure.context.tile_id,
        Some(configuration.records[0].key.tile_id.clone())
    );
    assert_eq!(
        failure.rollback.beginning_owner_sha256.as_deref(),
        Some(resource.beginning_state().state_sha256.as_str())
    );
    assert_eq!(
        failure.rollback.attempted_owner_sha256.as_deref(),
        Some(expected_attempted.as_str())
    );
}

#[test]
fn arithmetic_preflight_finds_later_store_e003_after_earlier_finite_mismatch() {
    let configuration = routed_configuration();
    let beginning = initial_state(&configuration, 0.0);
    let transaction_id = TransactionId(395);
    let resource = resource_candidate(&configuration, &beginning, transaction_id, None, &[]);
    let input = DirectSurfaceLiquidIngressInput {
        transaction_id,
        day_index: 3,
        interval_index: 0,
        interval_s: INTERVAL_S,
        tile_ingress: configuration
            .records
            .iter()
            .map(|record| open_ingress(record, 0.1))
            .collect(),
        wb14_parameters: parameters(&configuration),
    };
    let mut candidate = execute_surface_liquid_ingress(&configuration, &resource, &input)
        .expect("two-store candidate");
    candidate
        .closure_operands
        .poison_first_finite_and_last_arithmetic_for_test();
    let attempted = candidate
        .ending_state
        .recomputed_sha256()
        .expect("attempted digest");
    let error = candidate
        .validate(&configuration, &resource, &input)
        .expect_err("later arithmetic failure must outrank earlier finite mismatch");
    let failure = error.failure().expect("canonical arithmetic failure");
    let last = configuration.records.last().expect("last store");
    assert_eq!(failure.code, DirectSurfaceLiquidErrorCode::E003);
    assert_eq!(failure.phase, DirectSurfaceLiquidPhase::IndependentClosure);
    assert_eq!(failure.context.transaction_id, Some(transaction_id));
    assert_eq!(
        failure.context.owner_id,
        Some(configuration.owner_id.clone())
    );
    assert_eq!(failure.context.ofe_id, Some(last.key.ofe_id.clone()));
    assert_eq!(failure.context.tile_id, Some(last.key.tile_id.clone()));
    assert_eq!(failure.context.parcel_id, None);
    assert_eq!(
        failure.rollback.beginning_owner_sha256.as_deref(),
        Some(resource.beginning_state().state_sha256.as_str())
    );
    assert_eq!(
        failure.rollback.attempted_owner_sha256.as_deref(),
        Some(attempted.as_str())
    );
}

#[test]
fn arithmetic_preflight_finds_aggregate_e003_before_producer_and_finite_closure_errors() {
    let configuration = one_tile_configuration(DirectGroundIngressMode::CoveredCanopyRelease);
    let beginning = initial_state(&configuration, 0.0);
    let transaction_id = TransactionId(398);
    let resource = resource_candidate(&configuration, &beginning, transaction_id, None, &[]);
    let zero = amount(0.0, 284.0, 0.0, INTERVAL_S);
    let input = DirectSurfaceLiquidIngressInput {
        transaction_id,
        day_index: 3,
        interval_index: 0,
        interval_s: INTERVAL_S,
        tile_ingress: vec![DirectTileGroundIngress::CoveredCanopyRelease {
            ofe_id: configuration.records[0].key.ofe_id.clone(),
            tile_id: configuration.records[0].key.tile_id.clone(),
            surface_id: configuration.records[0].key.surface_id.clone(),
            release: DirectCanopyLiquidRelease {
                throughfall: amount(0.1, 280.0, 0.0, INTERVAL_S),
                initial_drainage: amount(0.1, 290.0, 0.0, INTERVAL_S),
                second_drainage: zero.clone(),
                stemflow: zero,
            },
        }],
        wb14_parameters: parameters(&configuration),
    };
    let mut candidate = execute_surface_liquid_ingress(&configuration, &resource, &input)
        .expect("covered candidate");
    let source_ids = candidate
        .closure_operands
        .poison_finite_store_and_two_parcel_aggregate_for_test();
    assert_eq!(source_ids.len(), 2);
    let expected_parcel = source_ids[0].clone();
    for source_id in source_ids {
        let mut first = true;
        for receipt in candidate
            .receipts
            .iter_mut()
            .filter(|receipt| receipt.source_parcel_id == source_id)
        {
            receipt.enthalpy_j_m2_basis_ofe_ground = if first {
                first = false;
                f64::MAX * 0.3
            } else {
                0.0
            };
        }
        assert!(!first, "source parcel must retain at least one receipt");
    }

    let attempted = candidate
        .ending_state
        .recomputed_sha256()
        .expect("attempted digest");
    let error = candidate
        .validate(&configuration, &resource, &input)
        .expect_err("aggregate arithmetic failure must outrank producer and closure mismatches");
    let failure = error.failure().expect("canonical arithmetic failure");
    let record = &configuration.records[0];
    assert_eq!(failure.code, DirectSurfaceLiquidErrorCode::E003);
    assert_eq!(failure.phase, DirectSurfaceLiquidPhase::IndependentClosure);
    assert_eq!(failure.context.transaction_id, Some(transaction_id));
    assert_eq!(
        failure.context.owner_id,
        Some(configuration.owner_id.clone())
    );
    assert_eq!(failure.context.ofe_id, Some(record.key.ofe_id.clone()));
    assert_eq!(failure.context.tile_id, Some(record.key.tile_id.clone()));
    assert_eq!(
        failure.context.surface_id,
        Some(record.key.surface_id.clone())
    );
    assert_eq!(
        failure.context.source_id,
        Some(record.key.source_id.clone())
    );
    assert_eq!(failure.context.parcel_id, Some(expected_parcel));
    assert_eq!(
        failure.rollback.beginning_owner_sha256.as_deref(),
        Some(resource.beginning_state().state_sha256.as_str())
    );
    assert_eq!(
        failure.rollback.attempted_owner_sha256.as_deref(),
        Some(attempted.as_str())
    );
}

#[test]
fn same_ofe_input_order_preserves_mixture_but_raw_enthalpy_swap_is_e003() {
    let configuration = one_tile_configuration(DirectGroundIngressMode::CoveredCanopyRelease);
    let beginning = initial_state(&configuration, 0.0);
    let transaction_id = TransactionId(399);
    let resource = resource_candidate(&configuration, &beginning, transaction_id, None, &[]);
    let zero = amount(0.0, 284.0, 0.0, INTERVAL_S);
    let input = DirectSurfaceLiquidIngressInput {
        transaction_id,
        day_index: 3,
        interval_index: 0,
        interval_s: INTERVAL_S,
        tile_ingress: vec![DirectTileGroundIngress::CoveredCanopyRelease {
            ofe_id: ofe("only"),
            tile_id: tile("tile"),
            surface_id: surface("surface-tile"),
            release: DirectCanopyLiquidRelease {
                throughfall: amount(0.1, 280.0, 0.0, INTERVAL_S),
                initial_drainage: amount(0.1, 290.0, 0.0, INTERVAL_S),
                second_drainage: zero.clone(),
                stemflow: zero,
            },
        }],
        wb14_parameters: parameters(&configuration),
    };
    let mut candidate = execute_surface_liquid_ingress(&configuration, &resource, &input)
        .expect("covered candidate");
    let mut reversed_input = input.clone();
    let DirectTileGroundIngress::CoveredCanopyRelease { release, .. } =
        &mut reversed_input.tile_ingress[0]
    else {
        panic!("covered input");
    };
    std::mem::swap(&mut release.throughfall, &mut release.initial_drainage);
    let reversed = execute_surface_liquid_ingress(&configuration, &resource, &reversed_input)
        .expect("reversed-temperature candidate");
    assert_eq!(candidate.ledgers, reversed.ledgers);
    assert_eq!(
        candidate
            .receipts
            .iter()
            .map(|receipt| receipt.temperature_k.to_bits())
            .collect::<Vec<_>>(),
        reversed
            .receipts
            .iter()
            .map(|receipt| receipt.temperature_k.to_bits())
            .collect::<Vec<_>>()
    );
    let _ = candidate
        .closure_operands
        .swap_first_two_source_enthalpies_for_test();
    let error = candidate
        .validate(&configuration, &resource, &input)
        .expect_err("raw Q must remain bound to its own source operands");
    assert_eq!(error.code(), DirectSurfaceLiquidErrorCode::E003);
}

#[test]
fn frozen_source_identity_rejects_zero_row_deletion_rekey_duplicate_and_kind_swap() {
    let configuration = one_tile_configuration(DirectGroundIngressMode::CoveredCanopyRelease);
    let beginning = initial_state(&configuration, 0.0);
    let transaction_id = TransactionId(402);
    let resource = resource_candidate(&configuration, &beginning, transaction_id, None, &[]);
    let zero = amount(0.0, 284.0, 0.0, INTERVAL_S);
    let input = DirectSurfaceLiquidIngressInput {
        transaction_id,
        day_index: 3,
        interval_index: 0,
        interval_s: INTERVAL_S,
        tile_ingress: vec![DirectTileGroundIngress::CoveredCanopyRelease {
            ofe_id: ofe("only"),
            tile_id: tile("tile"),
            surface_id: surface("surface-tile"),
            release: DirectCanopyLiquidRelease {
                throughfall: amount(0.1, 280.0, 0.0, INTERVAL_S),
                initial_drainage: zero.clone(),
                second_drainage: zero.clone(),
                stemflow: zero,
            },
        }],
        wb14_parameters: parameters(&configuration),
    };
    let candidate = execute_surface_liquid_ingress(&configuration, &resource, &input)
        .expect("covered candidate");
    let key = &configuration.records[0].key;
    for (poison, parcel_id) in [
        {
            let mut poison = candidate.clone();
            let id = poison.closure_operands.remove_source_for_test(2);
            (poison, id)
        },
        {
            let mut poison = candidate.clone();
            let id = poison.closure_operands.rekey_first_source_for_test();
            (poison, id)
        },
        {
            let mut poison = candidate.clone();
            let id = poison.closure_operands.duplicate_first_source_for_test();
            (poison, id)
        },
        {
            let mut poison = candidate.clone();
            let id = poison
                .closure_operands
                .swap_first_two_source_kinds_for_test();
            (poison, id)
        },
    ] {
        let attempted = poison.ending_state.recomputed_sha256().expect("digest");
        assert_producer_e009(
            &poison
                .validate(&configuration, &resource, &input)
                .expect_err("frozen identity poison"),
            transaction_id,
            &configuration,
            Some(&key.ofe_id),
            Some(&key.tile_id),
            Some(&parcel_id),
            &resource.beginning_state().state_sha256,
            &attempted,
        );
    }
}

#[test]
fn source_specific_output_temperature_is_not_the_interval_mixture() {
    let configuration = one_tile_configuration(DirectGroundIngressMode::CoveredCanopyRelease);
    let beginning = initial_state(&configuration, 0.0);
    let transaction_id = TransactionId(403);
    let resource = resource_candidate(&configuration, &beginning, transaction_id, None, &[]);
    let zero = amount(0.0, 284.0, 0.0, INTERVAL_S);
    let input = DirectSurfaceLiquidIngressInput {
        transaction_id,
        day_index: 3,
        interval_index: 0,
        interval_s: INTERVAL_S,
        tile_ingress: vec![DirectTileGroundIngress::CoveredCanopyRelease {
            ofe_id: ofe("only"),
            tile_id: tile("tile"),
            surface_id: surface("surface-tile"),
            release: DirectCanopyLiquidRelease {
                throughfall: amount(0.1, 280.0, 0.0, INTERVAL_S),
                initial_drainage: amount(0.1, 290.0, 0.0, INTERVAL_S),
                second_drainage: zero.clone(),
                stemflow: zero,
            },
        }],
        wb14_parameters: parameters(&configuration),
    };
    let mut candidate =
        execute_surface_liquid_ingress(&configuration, &resource, &input).expect("mixed candidate");
    let mut changed_mass = 0.0;
    for receipt in candidate
        .receipts
        .iter_mut()
        .filter(|receipt| receipt.kind == DirectSurfaceLiquidParcelKind::CanopyThroughfall)
    {
        assert!((receipt.temperature_k - 285.0).abs() <= f64::EPSILON * 512.0);
        receipt.temperature_k = 280.0;
        receipt.enthalpy_j_m2_basis_ofe_ground =
            receipt.mass_kg_m2_basis_ofe_ground * liquid_specific_enthalpy(280.0);
        changed_mass += receipt.mass_kg_m2_basis_ofe_ground;
    }
    assert!(changed_mass > 0.0);
    let error = super::super::surface_liquid_closure::validate_surface_liquid_closure_operands(
        &configuration,
        &resource,
        &candidate.closure_operands,
        &candidate.receipts,
        &candidate.ending_state,
    )
    .expect_err("source-specific output temperature");
    assert_eq!(error.code(), DirectSurfaceLiquidErrorCode::E010);
    let failure = error.failure().expect("typed closure failure");
    assert_eq!(failure.context.ofe_id, Some(ofe("only")));
    assert_eq!(failure.context.tile_id, Some(tile("tile")));
}

#[test]
fn chronological_partial_overlap_closure_and_support_identity() {
    let configuration = multi_tile_one_ofe_configuration();
    let beginning = initial_state(&configuration, 0.0);
    let transaction_id = TransactionId(405);
    let resource = resource_candidate(&configuration, &beginning, transaction_id, None, &[]);
    let input = DirectSurfaceLiquidIngressInput {
        transaction_id,
        day_index: 3,
        interval_index: 0,
        interval_s: INTERVAL_S,
        tile_ingress: vec![
            open_ingress(&configuration.records[0], 0.2),
            open_ingress(&configuration.records[1], 0.2),
        ],
        wb14_parameters: parameters(&configuration),
    };
    let mut input = input;
    let DirectTileGroundIngress::OpenRawPrecipitation {
        raw_precipitation, ..
    } = &mut input.tile_ingress[0]
    else {
        panic!("open input");
    };
    *raw_precipitation = amount(0.2, 280.0, 0.0, 1_200.0);
    let DirectTileGroundIngress::OpenRawPrecipitation {
        raw_precipitation, ..
    } = &mut input.tile_ingress[1]
    else {
        panic!("open input");
    };
    *raw_precipitation = amount(0.2, 300.0, 600.0, INTERVAL_S);
    let candidate = execute_surface_liquid_ingress(&configuration, &resource, &input)
        .expect("multi-boundary candidate");
    candidate
        .validate(&configuration, &resource, &input)
        .expect("chronological independent closure");

    let mut window_swap = candidate.clone();
    let indices = window_swap
        .receipts
        .iter()
        .enumerate()
        .filter(|(_, receipt)| receipt.mass_kg_m2_basis_ofe_ground > 0.0)
        .map(|(index, _)| index)
        .take(2)
        .collect::<Vec<_>>();
    assert_eq!(indices.len(), 2);
    let (left, right) = window_swap.receipts.split_at_mut(indices[1]);
    std::mem::swap(
        &mut left[indices[0]].temperature_k,
        &mut right[0].temperature_k,
    );
    std::mem::swap(
        &mut left[indices[0]].enthalpy_j_m2_basis_ofe_ground,
        &mut right[0].enthalpy_j_m2_basis_ofe_ground,
    );
    let swap_error =
        super::super::surface_liquid_closure::validate_surface_liquid_closure_operands(
            &configuration,
            &resource,
            &window_swap.closure_operands,
            &window_swap.receipts,
            &window_swap.ending_state,
        )
        .expect_err("cross-window Q/T swap");
    assert_eq!(swap_error.code(), DirectSurfaceLiquidErrorCode::E010);

    let mut poison = candidate;
    let parcel_id = poison
        .closure_operands
        .poison_first_source_support_for_test();
    let attempted = poison.ending_state.recomputed_sha256().expect("digest");
    assert_producer_e009(
        &poison
            .validate(&configuration, &resource, &input)
            .expect_err("wrong frozen support"),
        transaction_id,
        &configuration,
        Some(&ofe("only")),
        Some(&configuration.records[0].key.tile_id),
        Some(&parcel_id),
        &resource.beginning_state().state_sha256,
        &attempted,
    );
}

#[test]
fn independent_partition_rejects_owner_and_cross_tile_receipt_swaps() {
    let configuration = one_tile_configuration(DirectGroundIngressMode::OpenRawPrecipitation);
    let beginning = initial_state(&configuration, 0.0);
    let transaction_id = TransactionId(409);
    let resource = resource_candidate(&configuration, &beginning, transaction_id, None, &[]);
    let input = DirectSurfaceLiquidIngressInput {
        transaction_id,
        day_index: 3,
        interval_index: 0,
        interval_s: INTERVAL_S,
        tile_ingress: vec![open_ingress(&configuration.records[0], 1.0)],
        wb14_parameters: parameters(&configuration),
    };
    let candidate =
        execute_surface_liquid_ingress(&configuration, &resource, &input).expect("partitioned");
    let mut owner_swap = candidate.clone();
    let infiltration = owner_swap
        .receipts
        .iter()
        .position(|row| row.disposition == DirectSurfaceLiquidReceiptDisposition::Infiltration)
        .expect("infiltration receipt");
    let retained = owner_swap
        .receipts
        .iter()
        .position(|row| row.disposition == DirectSurfaceLiquidReceiptDisposition::RetainedSurface)
        .expect("retention receipt");
    let (left, right) = owner_swap.receipts.split_at_mut(retained);
    std::mem::swap(&mut left[infiltration].recipient, &mut right[0].recipient);
    std::mem::swap(
        &mut left[infiltration].recipient_store_key,
        &mut right[0].recipient_store_key,
    );
    std::mem::swap(
        &mut left[infiltration].disposition,
        &mut right[0].disposition,
    );
    let error = super::super::surface_liquid_closure::validate_surface_liquid_closure_operands(
        &configuration,
        &resource,
        &owner_swap.closure_operands,
        &owner_swap.receipts,
        &owner_swap.ending_state,
    )
    .expect_err("coordinated infiltration-retention owner swap");
    assert_eq!(error.code(), DirectSurfaceLiquidErrorCode::E010);

    let configuration = multi_tile_one_ofe_configuration();
    let beginning = initial_state(&configuration, 0.0);
    let transaction_id = TransactionId(410);
    let resource = resource_candidate(&configuration, &beginning, transaction_id, None, &[]);
    let input = DirectSurfaceLiquidIngressInput {
        transaction_id,
        day_index: 3,
        interval_index: 0,
        interval_s: INTERVAL_S,
        tile_ingress: configuration
            .records
            .iter()
            .map(|record| open_ingress(record, 0.2))
            .collect(),
        wb14_parameters: parameters(&configuration),
    };
    let mut cross_tile =
        execute_surface_liquid_ingress(&configuration, &resource, &input).expect("multi-tile");
    let destination = configuration.records[1].key.clone();
    let receipt = cross_tile
        .receipts
        .iter_mut()
        .find(|row| {
            row.disposition == DirectSurfaceLiquidReceiptDisposition::RetainedSurface
                && row.recipient_store_key == configuration.records[0].key
        })
        .expect("first tile retention");
    receipt.recipient_store_key = destination.clone();
    receipt.recipient = DirectSurfaceLiquidReceiptRecipient::SurfaceStore {
        store_key: destination,
    };
    let error = super::super::surface_liquid_closure::validate_surface_liquid_closure_operands(
        &configuration,
        &resource,
        &cross_tile.closure_operands,
        &cross_tile.receipts,
        &cross_tile.ending_state,
    )
    .expect_err("cross-tile retention");
    assert_eq!(error.code(), DirectSurfaceLiquidErrorCode::E010);
}

#[test]
fn caller_tile_ingress_order_does_not_change_canonical_sources_or_results() {
    let configuration = multi_tile_one_ofe_configuration();
    let beginning = initial_state(&configuration, 0.0);
    let transaction_id = TransactionId(406);
    let resource = resource_candidate(&configuration, &beginning, transaction_id, None, &[]);
    let mut input = DirectSurfaceLiquidIngressInput {
        transaction_id,
        day_index: 3,
        interval_index: 0,
        interval_s: INTERVAL_S,
        tile_ingress: configuration
            .records
            .iter()
            .map(|record| open_ingress(record, 0.1))
            .collect(),
        wb14_parameters: parameters(&configuration),
    };
    let ordered =
        execute_surface_liquid_ingress(&configuration, &resource, &input).expect("ordered ingress");
    input.tile_ingress.reverse();
    let reversed = execute_surface_liquid_ingress(&configuration, &resource, &input)
        .expect("reversed ingress");
    assert_eq!(ordered.ending_state, reversed.ending_state);
    assert_eq!(ordered.receipts, reversed.receipts);
    assert_eq!(ordered.ledgers, reversed.ledgers);
    assert_eq!(ordered.closure_operands, reversed.closure_operands);
}

#[test]
fn exhaustive_source_and_receipt_domains_fail_e003_before_producer_comparison() {
    let configuration = one_tile_configuration(DirectGroundIngressMode::OpenRawPrecipitation);
    let beginning = initial_state(&configuration, 0.0);
    let transaction_id = TransactionId(407);
    let resource = resource_candidate(&configuration, &beginning, transaction_id, None, &[]);
    let input = DirectSurfaceLiquidIngressInput {
        transaction_id,
        day_index: 3,
        interval_index: 0,
        interval_s: INTERVAL_S,
        tile_ingress: vec![open_ingress(&configuration.records[0], 0.1)],
        wb14_parameters: parameters(&configuration),
    };
    let candidate =
        execute_surface_liquid_ingress(&configuration, &resource, &input).expect("candidate");

    let mut poisons = Vec::new();
    let mut frozen_nan = candidate.clone();
    let parcel_id = frozen_nan
        .closure_operands
        .poison_first_source_nan_support_for_test();
    poisons.push((frozen_nan, parcel_id));
    let mut raw_q = candidate.clone();
    let parcel_id = raw_q
        .closure_operands
        .poison_first_source_raw_enthalpy_for_test();
    poisons.push((raw_q, parcel_id));
    let mut reversed = candidate.clone();
    reversed.receipts[0].end_s = reversed.receipts[0].start_s;
    poisons.push((reversed.clone(), reversed.receipts[0].parcel_id.clone()));
    let mut out_of_range = candidate.clone();
    out_of_range.receipts[0].end_s = INTERVAL_S + 1.0;
    poisons.push((
        out_of_range.clone(),
        out_of_range.receipts[0].parcel_id.clone(),
    ));
    let mut receipt_nan = candidate.clone();
    receipt_nan.receipts[0].start_s = f64::NAN;
    poisons.push((
        receipt_nan.clone(),
        receipt_nan.receipts[0].parcel_id.clone(),
    ));
    let mut negative = candidate.clone();
    negative.receipts[0].mass_kg_m2_basis_ofe_ground = -1.0;
    poisons.push((negative.clone(), negative.receipts[0].parcel_id.clone()));
    let mut hot = candidate;
    hot.receipts[0].temperature_k = 350.000_1;
    poisons.push((hot.clone(), hot.receipts[0].parcel_id.clone()));

    for (poison, parcel_id) in poisons {
        let attempted = poison.ending_state.recomputed_sha256().expect("digest");
        let error = poison
            .validate(&configuration, &resource, &input)
            .expect_err("domain poison");
        let failure = error.failure().expect("typed E003");
        assert_eq!(failure.code, DirectSurfaceLiquidErrorCode::E003);
        assert_eq!(failure.phase, DirectSurfaceLiquidPhase::IndependentClosure);
        assert_eq!(failure.context.transaction_id, Some(transaction_id));
        assert_eq!(
            failure.context.owner_id,
            Some(configuration.owner_id.clone())
        );
        assert_eq!(failure.context.ofe_id, Some(ofe("only")));
        assert_eq!(failure.context.tile_id, Some(tile("tile")));
        assert_eq!(
            failure.context.parcel_id.as_deref(),
            Some(parcel_id.as_str())
        );
        assert_eq!(
            failure.rollback.beginning_owner_sha256.as_deref(),
            Some(resource.beginning_state().state_sha256.as_str())
        );
        assert_eq!(
            failure.rollback.attempted_owner_sha256.as_deref(),
            Some(attempted.as_str())
        );
    }
}

#[test]
fn per_source_comparison_scale_overflow_is_e003() {
    let configuration = one_tile_configuration(DirectGroundIngressMode::OpenRawPrecipitation);
    let beginning = initial_state(&configuration, 0.0);
    let transaction_id = TransactionId(400);
    let resource = resource_candidate(&configuration, &beginning, transaction_id, None, &[]);
    let input = DirectSurfaceLiquidIngressInput {
        transaction_id,
        day_index: 3,
        interval_index: 0,
        interval_s: INTERVAL_S,
        tile_ingress: vec![open_ingress(&configuration.records[0], 0.1)],
        wb14_parameters: parameters(&configuration),
    };
    let mut candidate =
        execute_surface_liquid_ingress(&configuration, &resource, &input).expect("open candidate");
    let source_id = candidate
        .closure_operands
        .poison_first_source_comparison_scale_for_test();
    let mut first = true;
    for receipt in candidate
        .receipts
        .iter_mut()
        .filter(|receipt| receipt.source_parcel_id == source_id)
    {
        receipt.enthalpy_j_m2_basis_ofe_ground = if first {
            first = false;
            f64::MAX * 0.6
        } else {
            0.0
        };
    }
    assert!(!first);
    let attempted = candidate.ending_state.recomputed_sha256().expect("digest");
    let failure = candidate
        .validate(&configuration, &resource, &input)
        .expect_err("mixture arithmetic")
        .failure()
        .expect("failure")
        .clone();
    assert_eq!(failure.code, DirectSurfaceLiquidErrorCode::E003);
    assert_eq!(failure.context.ofe_id, Some(ofe("only")));
    assert_eq!(failure.context.tile_id, Some(tile("tile")));
    assert_eq!(failure.context.surface_id, Some(surface("surface-tile")));
    assert_eq!(failure.context.source_id, Some(source("source-tile")));
    assert_eq!(failure.context.parcel_id, Some(source_id));
    assert_eq!(
        failure.rollback.beginning_owner_sha256.as_deref(),
        Some(resource.beginning_state().state_sha256.as_str())
    );
    assert_eq!(
        failure.rollback.attempted_owner_sha256.as_deref(),
        Some(attempted.as_str())
    );
}

#[test]
fn multi_tile_ofe_aggregate_failure_does_not_invent_a_tile_identity() {
    let configuration = multi_tile_one_ofe_configuration();
    let beginning = initial_state(&configuration, 0.0);
    let transaction_id = TransactionId(404);
    let resource = resource_candidate(&configuration, &beginning, transaction_id, None, &[]);
    let input = DirectSurfaceLiquidIngressInput {
        transaction_id,
        day_index: 3,
        interval_index: 0,
        interval_s: INTERVAL_S,
        tile_ingress: configuration
            .records
            .iter()
            .map(|record| open_ingress(record, 0.1))
            .collect(),
        wb14_parameters: parameters(&configuration),
    };
    let mut candidate = execute_surface_liquid_ingress(&configuration, &resource, &input)
        .expect("multi-tile candidate");
    let source_ids = candidate
        .closure_operands
        .poison_finite_store_and_two_parcel_aggregate_for_test();
    assert_eq!(source_ids.len(), 2);
    let expected_parcel = source_ids[0].clone();
    let failure = candidate
        .validate(&configuration, &resource, &input)
        .expect_err("multi-tile OFE aggregate arithmetic")
        .failure()
        .expect("typed failure")
        .clone();
    assert_eq!(failure.code, DirectSurfaceLiquidErrorCode::E003);
    assert_eq!(
        failure.context.owner_id,
        Some(configuration.owner_id.clone())
    );
    assert_eq!(failure.context.ofe_id, Some(ofe("only")));
    assert_eq!(failure.context.tile_id, Some(tile("tile-a")));
    assert_eq!(failure.context.surface_id, Some(surface("surface-tile-a")));
    assert_eq!(failure.context.source_id, Some(source("source-tile-a")));
    assert_eq!(failure.context.parcel_id, Some(expected_parcel));
}

#[test]
fn producer_second_store_ledger_and_wb14_mismatches_report_exact_identity() {
    let configuration = routed_configuration();
    let beginning = initial_state(&configuration, 0.0);
    let transaction_id = TransactionId(396);
    let resource = resource_candidate(&configuration, &beginning, transaction_id, None, &[]);
    let input = DirectSurfaceLiquidIngressInput {
        transaction_id,
        day_index: 3,
        interval_index: 0,
        interval_s: INTERVAL_S,
        tile_ingress: configuration
            .records
            .iter()
            .map(|record| open_ingress(record, 0.1))
            .collect(),
        wb14_parameters: parameters(&configuration),
    };
    let candidate = execute_surface_liquid_ingress(&configuration, &resource, &input)
        .expect("two-store candidate");
    let second = &configuration.records[1];

    let mut ending_poison = candidate.clone();
    ending_poison.ending_state.records[1].liquid_kg_m2_tile += 0.01;
    let ending_attempted = ending_poison
        .ending_state
        .recomputed_sha256()
        .expect("ending attempted digest");
    assert_producer_e009(
        &ending_poison
            .validate(&configuration, &resource, &input)
            .expect_err("second ending store mismatch"),
        transaction_id,
        &configuration,
        Some(&second.key.ofe_id),
        Some(&second.key.tile_id),
        None,
        &resource.beginning_state().state_sha256,
        &ending_attempted,
    );

    let mut ledger_poison = candidate.clone();
    ledger_poison.ledgers[1].runoff_mass_kg_m2_ofe_ground += 0.01;
    let ledger_attempted = ledger_poison
        .ending_state
        .recomputed_sha256()
        .expect("ledger attempted digest");
    assert_producer_e009(
        &ledger_poison
            .validate(&configuration, &resource, &input)
            .expect_err("second ledger mismatch"),
        transaction_id,
        &configuration,
        Some(&second.key.ofe_id),
        None,
        None,
        &resource.beginning_state().state_sha256,
        &ledger_attempted,
    );

    let mut wb14_poison = candidate;
    *wb14_poison
        .wb14_calls_by_ofe
        .get_mut(&second.key.ofe_id)
        .expect("second WB14 counter") += 1;
    let wb14_attempted = wb14_poison
        .ending_state
        .recomputed_sha256()
        .expect("WB14 attempted digest");
    assert_producer_e009(
        &wb14_poison
            .validate(&configuration, &resource, &input)
            .expect_err("second WB14 mismatch"),
        transaction_id,
        &configuration,
        Some(&second.key.ofe_id),
        None,
        None,
        &resource.beginning_state().state_sha256,
        &wb14_attempted,
    );
}

#[test]
#[allow(clippy::too_many_lines)]
fn producer_upper_and_middle_deletions_report_the_missing_identity() {
    let configuration = three_ofe_configuration();
    let beginning = initial_state(&configuration, 0.0);
    let transaction_id = TransactionId(397);
    let resource = resource_candidate(&configuration, &beginning, transaction_id, None, &[]);
    let input = DirectSurfaceLiquidIngressInput {
        transaction_id,
        day_index: 3,
        interval_index: 0,
        interval_s: INTERVAL_S,
        tile_ingress: configuration
            .records
            .iter()
            .map(|record| open_ingress(record, 0.1))
            .collect(),
        wb14_parameters: parameters(&configuration),
    };
    let candidate = execute_surface_liquid_ingress(&configuration, &resource, &input)
        .expect("three-OFE candidate");

    for index in [0, 1] {
        let expected = configuration.records[index].key.clone();
        let mut poison = candidate.clone();
        poison.ending_state.records.remove(index);
        let attempted = poison
            .ending_state
            .recomputed_sha256()
            .expect("state-deletion digest");
        assert_producer_e009(
            &poison
                .validate(&configuration, &resource, &input)
                .expect_err("ending-state deletion"),
            transaction_id,
            &configuration,
            Some(&expected.ofe_id),
            Some(&expected.tile_id),
            None,
            &resource.beginning_state().state_sha256,
            &attempted,
        );

        let expected_continuation = candidate.ending_state.continuations[index].ofe_id.clone();
        let mut poison = candidate.clone();
        poison.ending_state.continuations.remove(index);
        let attempted = poison
            .ending_state
            .recomputed_sha256()
            .expect("continuation-deletion digest");
        assert_producer_e009(
            &poison
                .validate(&configuration, &resource, &input)
                .expect_err("continuation deletion"),
            transaction_id,
            &configuration,
            Some(&expected_continuation),
            None,
            None,
            &resource.beginning_state().state_sha256,
            &attempted,
        );

        let expected_receipt = candidate.receipts[index].clone();
        let mut poison = candidate.clone();
        poison.receipts.remove(index);
        let attempted = poison
            .ending_state
            .recomputed_sha256()
            .expect("receipt-deletion digest");
        assert_producer_e009(
            &poison
                .validate(&configuration, &resource, &input)
                .expect_err("receipt deletion"),
            transaction_id,
            &configuration,
            Some(&expected_receipt.recipient_store_key.ofe_id),
            Some(&expected_receipt.recipient_store_key.tile_id),
            Some(&expected_receipt.parcel_id),
            &resource.beginning_state().state_sha256,
            &attempted,
        );

        let expected_ledger = candidate.ledgers[index].ofe_id.clone();
        let mut poison = candidate.clone();
        poison.ledgers.remove(index);
        let attempted = poison
            .ending_state
            .recomputed_sha256()
            .expect("ledger-deletion digest");
        assert_producer_e009(
            &poison
                .validate(&configuration, &resource, &input)
                .expect_err("ledger deletion"),
            transaction_id,
            &configuration,
            Some(&expected_ledger),
            None,
            None,
            &resource.beginning_state().state_sha256,
            &attempted,
        );

        let expected_wb14 = configuration.ofe_topology[index].clone();
        let mut poison = candidate.clone();
        poison.wb14_calls_by_ofe.remove(&expected_wb14);
        let attempted = poison
            .ending_state
            .recomputed_sha256()
            .expect("WB14-deletion digest");
        assert_producer_e009(
            &poison
                .validate(&configuration, &resource, &input)
                .expect_err("WB14 deletion"),
            transaction_id,
            &configuration,
            Some(&expected_wb14),
            None,
            None,
            &resource.beginning_state().state_sha256,
            &attempted,
        );
    }
}

fn assert_independent_store_e010(
    error: &DirectSurfaceLiquidError,
    transaction_id: TransactionId,
    configuration: &DirectSurfaceLiquidConfiguration,
    expected: &DirectSurfaceLiquidStoreKey,
    beginning_sha256: &str,
    attempted_sha256: &str,
    label: &str,
) {
    let failure = error.failure().expect("typed store failure");
    assert_eq!(failure.code, DirectSurfaceLiquidErrorCode::E010, "{label}");
    assert_eq!(
        failure.phase,
        DirectSurfaceLiquidPhase::IndependentClosure,
        "{label}"
    );
    assert_eq!(
        failure.context.transaction_id,
        Some(transaction_id),
        "{label}"
    );
    assert_eq!(
        failure.context.owner_id,
        Some(configuration.owner_id.clone()),
        "{label}"
    );
    assert_eq!(
        (
            failure.context.ofe_id.as_ref(),
            failure.context.tile_id.as_ref(),
            failure.context.surface_id.as_ref(),
            failure.context.source_id.as_ref(),
            failure.context.parcel_id.as_ref(),
        ),
        (
            Some(&expected.ofe_id),
            Some(&expected.tile_id),
            Some(&expected.surface_id),
            Some(&expected.source_id),
            None,
        ),
        "{label}"
    );
    assert_eq!(
        (
            failure.rollback.beginning_owner_sha256.as_deref(),
            failure.rollback.attempted_owner_sha256.as_deref(),
        ),
        (Some(beginning_sha256), Some(attempted_sha256)),
        "{label}"
    );
}

fn assert_independent_continuation_e010(
    error: &DirectSurfaceLiquidError,
    transaction_id: TransactionId,
    configuration: &DirectSurfaceLiquidConfiguration,
    expected_ofe: &OfeId,
    beginning_sha256: &str,
    attempted_sha256: &str,
    label: &str,
) {
    let failure = error.failure().expect("typed continuation failure");
    assert_eq!(failure.code, DirectSurfaceLiquidErrorCode::E010, "{label}");
    assert_eq!(
        failure.phase,
        DirectSurfaceLiquidPhase::IndependentClosure,
        "{label}"
    );
    assert_eq!(
        failure.context.transaction_id,
        Some(transaction_id),
        "{label}"
    );
    assert_eq!(
        failure.context.owner_id,
        Some(configuration.owner_id.clone()),
        "{label}"
    );
    assert_eq!(
        (
            failure.context.ofe_id.as_ref(),
            failure.context.tile_id.as_ref(),
            failure.context.surface_id.as_ref(),
            failure.context.source_id.as_ref(),
            failure.context.parcel_id.as_ref(),
        ),
        (Some(expected_ofe), None, None, None, None),
        "{label}"
    );
    assert_eq!(
        (
            failure.rollback.beginning_owner_sha256.as_deref(),
            failure.rollback.attempted_owner_sha256.as_deref(),
        ),
        (Some(beginning_sha256), Some(attempted_sha256)),
        "{label}"
    );
}

fn assert_independent_aggregate_e010(
    error: &DirectSurfaceLiquidError,
    transaction_id: TransactionId,
    configuration: &DirectSurfaceLiquidConfiguration,
    beginning_sha256: &str,
    attempted_sha256: &str,
    label: &str,
) {
    let failure = error.failure().expect("typed aggregate failure");
    assert_eq!(failure.code, DirectSurfaceLiquidErrorCode::E010, "{label}");
    assert_eq!(
        failure.phase,
        DirectSurfaceLiquidPhase::IndependentClosure,
        "{label}"
    );
    assert_eq!(
        failure.context.transaction_id,
        Some(transaction_id),
        "{label}"
    );
    assert_eq!(
        failure.context.owner_id,
        Some(configuration.owner_id.clone()),
        "{label}"
    );
    assert_eq!(
        (
            failure.context.ofe_id.as_ref(),
            failure.context.tile_id.as_ref(),
            failure.context.surface_id.as_ref(),
            failure.context.source_id.as_ref(),
            failure.context.parcel_id.as_ref(),
        ),
        (None, None, None, None, None),
        "{label}"
    );
    assert_eq!(
        (
            failure.rollback.beginning_owner_sha256.as_deref(),
            failure.rollback.attempted_owner_sha256.as_deref(),
        ),
        (Some(beginning_sha256), Some(attempted_sha256)),
        "{label}"
    );
}

#[test]
#[allow(clippy::too_many_lines)]
fn independent_projection_binds_persistent_stores_continuations_and_digest() {
    let configuration = three_ofe_configuration();
    let beginning = initial_state(&configuration, 0.0);
    let transaction_id = TransactionId(411);
    let resource = resource_candidate(&configuration, &beginning, transaction_id, None, &[]);
    let input = DirectSurfaceLiquidIngressInput {
        transaction_id,
        day_index: 3,
        interval_index: 0,
        interval_s: INTERVAL_S,
        tile_ingress: configuration
            .records
            .iter()
            .map(|record| open_ingress(record, 0.1))
            .collect(),
        wb14_parameters: parameters(&configuration),
    };
    let candidate = execute_surface_liquid_ingress(&configuration, &resource, &input)
        .expect("three-OFE candidate");
    super::super::surface_liquid_closure::validate_surface_liquid_closure_operands(
        &configuration,
        &resource,
        &candidate.closure_operands,
        &candidate.receipts,
        &candidate.ending_state,
    )
    .expect("persistent projection baseline");
    let store_keys = configuration
        .records
        .iter()
        .map(|row| row.key.clone())
        .collect::<Vec<_>>();
    let mut store_poisons = Vec::new();
    let mut wrong_store = candidate.ending_state.clone();
    wrong_store.records[0].liquid_kg_m2_tile += 0.001;
    store_poisons.push((
        "wrong ending store value".to_owned(),
        wrong_store,
        store_keys[0].clone(),
    ));
    for (index, expected_store) in store_keys.iter().enumerate() {
        let mut ending = candidate.ending_state.clone();
        ending.records.remove(index);
        store_poisons.push((
            format!("missing ending store {index}"),
            ending,
            expected_store.clone(),
        ));
        let mut ending = candidate.ending_state.clone();
        ending.records.push(ending.records[index].clone());
        store_poisons.push((
            format!("extra ending store {index}"),
            ending,
            expected_store.clone(),
        ));
        let mut ending = candidate.ending_state.clone();
        let forged = DirectSurfaceLiquidStoreKey {
            run_id: 91,
            ofe_id: ofe(&format!("forged-{index}")),
            tile_id: tile(&format!("forged-tile-{index}")),
            surface_id: surface(&format!("forged-surface-{index}")),
            surface_class: SurfaceClass::BareMineralSoil,
            source_type: openwepp_land_surface_energy::WaterSourceType::SurfaceLiquid,
            source_id: source(&format!("forged-source-{index}")),
        };
        ending.records[index].key = forged.clone();
        store_poisons.push((format!("replacement ending store {index}"), ending, forged));
    }
    for (left, right) in [(0, 1), (1, 2), (0, 2)] {
        let mut ending = candidate.ending_state.clone();
        ending.records.swap(left, right);
        store_poisons.push((
            format!("reordered ending stores {left}/{right}"),
            ending,
            store_keys[right].clone(),
        ));
    }
    for (label, mut ending, expected_store) in store_poisons {
        ending.state_sha256 = ending.recomputed_sha256().expect("poison digest");
        let attempted_sha256 = ending.recomputed_sha256().expect("attempted digest");
        let error = super::super::surface_liquid_closure::validate_surface_liquid_closure_operands(
            &configuration,
            &resource,
            &candidate.closure_operands,
            &candidate.receipts,
            &ending,
        )
        .expect_err(&label);
        assert_independent_store_e010(
            &error,
            transaction_id,
            &configuration,
            &expected_store,
            &resource.beginning_state().state_sha256,
            &attempted_sha256,
            &label,
        );
    }
    let mut poisons = Vec::new();
    let mut supply = candidate.ending_state.clone();
    supply.continuations[0].cumulative_supply_m += 1.0e-6;
    poisons.push(("cumulative supply".to_owned(), supply, ofe("upper")));
    let mut infiltration = candidate.ending_state.clone();
    infiltration.continuations[0].cumulative_infiltration_m += 1.0e-6;
    poisons.push((
        "cumulative infiltration".to_owned(),
        infiltration,
        ofe("upper"),
    ));
    let mut rollover_day = candidate.ending_state.clone();
    rollover_day.continuations[0].day_index += 1;
    poisons.push(("rollover day".to_owned(), rollover_day, ofe("upper")));
    let mut rollover_interval = candidate.ending_state.clone();
    rollover_interval.continuations[0].next_interval_index = 48;
    poisons.push((
        "rollover interval".to_owned(),
        rollover_interval,
        ofe("upper"),
    ));
    let mut stale_transaction = candidate.ending_state.clone();
    stale_transaction.continuations[0].last_accepted_transaction_id = Some(TransactionId(410));
    poisons.push((
        "continuation transaction".to_owned(),
        stale_transaction,
        ofe("upper"),
    ));
    for (index, expected_ofe) in configuration.ofe_topology.iter().cloned().enumerate() {
        let mut ending = candidate.ending_state.clone();
        ending.continuations.remove(index);
        poisons.push((
            format!("missing continuation {index}"),
            ending,
            expected_ofe.clone(),
        ));
        let mut ending = candidate.ending_state.clone();
        ending
            .continuations
            .push(ending.continuations[index].clone());
        poisons.push((format!("extra continuation {index}"), ending, expected_ofe));
        let mut ending = candidate.ending_state.clone();
        let forged = ofe(&format!("forged-{index}"));
        ending.continuations[index].ofe_id = forged.clone();
        poisons.push((format!("replacement continuation {index}"), ending, forged));
    }
    for (left, right) in [(0, 1), (1, 2), (0, 2)] {
        let mut ending = candidate.ending_state.clone();
        ending.continuations.swap(left, right);
        poisons.push((
            format!("reordered continuations {left}/{right}"),
            ending,
            configuration.ofe_topology[right].clone(),
        ));
    }
    for (label, mut ending, expected_ofe) in poisons {
        ending.state_sha256 = ending.recomputed_sha256().expect("poison digest");
        let attempted_sha256 = ending.recomputed_sha256().expect("attempted digest");
        let error = super::super::surface_liquid_closure::validate_surface_liquid_closure_operands(
            &configuration,
            &resource,
            &candidate.closure_operands,
            &candidate.receipts,
            &ending,
        )
        .expect_err(&label);
        assert_independent_continuation_e010(
            &error,
            transaction_id,
            &configuration,
            &expected_ofe,
            &resource.beginning_state().state_sha256,
            &attempted_sha256,
            &label,
        );
    }
    let mut forged = candidate.clone();
    forged
        .closure_operands
        .forge_first_store_retained_and_ending_for_test(0.001);
    forged.ending_state.records[0].liquid_kg_m2_tile += 0.001;
    forged.ending_state.state_sha256 = forged
        .ending_state
        .recomputed_sha256()
        .expect("forged digest");
    let error = super::super::surface_liquid_closure::validate_surface_liquid_closure_operands(
        &configuration,
        &resource,
        &forged.closure_operands,
        &forged.receipts,
        &forged.ending_state,
    )
    .expect_err("self-consistent producer store operands");
    assert_eq!(error.code(), DirectSurfaceLiquidErrorCode::E010);

    let mut digest = candidate.ending_state.clone();
    digest.state_sha256.push('0');
    let attempted_sha256 = digest.recomputed_sha256().expect("attempted digest");
    let error = super::super::surface_liquid_closure::validate_surface_liquid_closure_operands(
        &configuration,
        &resource,
        &candidate.closure_operands,
        &candidate.receipts,
        &digest,
    )
    .expect_err("ending digest mismatch after joins");
    assert_independent_aggregate_e010(
        &error,
        transaction_id,
        &configuration,
        &resource.beginning_state().state_sha256,
        &attempted_sha256,
        "ending digest mismatch",
    );

    let mut aggregate = candidate.ending_state.clone();
    aggregate.owner_id = owner("forged-owner");
    aggregate.state_sha256 = aggregate.recomputed_sha256().expect("aggregate digest");
    let attempted_sha256 = aggregate.recomputed_sha256().expect("attempted digest");
    let error = super::super::surface_liquid_closure::validate_surface_liquid_closure_operands(
        &configuration,
        &resource,
        &candidate.closure_operands,
        &candidate.receipts,
        &aggregate,
    )
    .expect_err("aggregate owner mismatch");
    assert_independent_aggregate_e010(
        &error,
        transaction_id,
        &configuration,
        &resource.beginning_state().state_sha256,
        &attempted_sha256,
        "aggregate owner mismatch",
    );

    let mut aggregate = candidate.ending_state.clone();
    aggregate.configuration_sha256.push('0');
    aggregate.state_sha256 = aggregate.recomputed_sha256().expect("aggregate digest");
    let attempted_sha256 = aggregate.recomputed_sha256().expect("attempted digest");
    let error = super::super::surface_liquid_closure::validate_surface_liquid_closure_operands(
        &configuration,
        &resource,
        &candidate.closure_operands,
        &candidate.receipts,
        &aggregate,
    )
    .expect_err("aggregate configuration mismatch");
    assert_independent_aggregate_e010(
        &error,
        transaction_id,
        &configuration,
        &resource.beginning_state().state_sha256,
        &attempted_sha256,
        "aggregate configuration mismatch",
    );
}

#[test]
#[allow(clippy::too_many_lines)]
fn partition_input_membership_is_e009_and_arithmetic_e003_outranks_identity() {
    let configuration = three_ofe_configuration();
    let beginning = initial_state(&configuration, 0.0);
    let transaction_id = TransactionId(412);
    let resource = resource_candidate(&configuration, &beginning, transaction_id, None, &[]);
    let input = DirectSurfaceLiquidIngressInput {
        transaction_id,
        day_index: 3,
        interval_index: 0,
        interval_s: INTERVAL_S,
        tile_ingress: configuration
            .records
            .iter()
            .map(|record| open_ingress(record, 0.1))
            .collect(),
        wb14_parameters: parameters(&configuration),
    };
    let candidate = execute_surface_liquid_ingress(&configuration, &resource, &input)
        .expect("three-OFE candidate");

    let mut membership_poisons = Vec::new();
    let mut missing = candidate.clone();
    let missing_id = missing.closure_operands.remove_partition_input_for_test(1);
    membership_poisons.push(("missing", missing, missing_id));
    let mut duplicate = candidate.clone();
    let duplicate_id = duplicate
        .closure_operands
        .duplicate_partition_input_for_test(0);
    membership_poisons.push(("duplicate", duplicate, duplicate_id));
    let mut reordered = candidate.clone();
    reordered
        .closure_operands
        .reorder_partition_inputs_for_test();
    membership_poisons.push(("reordered", reordered, ofe("middle")));
    let mut wrong = candidate.clone();
    wrong
        .closure_operands
        .rekey_partition_input_for_test(0, ofe("forged"));
    membership_poisons.push(("wrong OFE", wrong, ofe("upper")));

    for (label, poison, expected_ofe) in membership_poisons {
        let attempted = poison.ending_state.recomputed_sha256().expect("digest");
        assert_producer_e009(
            &poison
                .validate(&configuration, &resource, &input)
                .expect_err(label),
            transaction_id,
            &configuration,
            Some(&expected_ofe),
            None,
            None,
            &resource.beginning_state().state_sha256,
            &attempted,
        );
    }

    let mut arithmetic = candidate.clone();
    arithmetic
        .closure_operands
        .poison_partition_cumulative_bound_for_test(0);
    arithmetic
        .closure_operands
        .remove_partition_input_for_test(1);
    arithmetic.ending_state.records[0].liquid_kg_m2_tile += 0.001;
    arithmetic.ending_state.state_sha256 = arithmetic
        .ending_state
        .recomputed_sha256()
        .expect("combined attempted digest");
    let error = arithmetic
        .validate(&configuration, &resource, &input)
        .expect_err("E003 must outrank E009 and E010");
    let failure = error.failure().expect("typed arithmetic failure");
    assert_eq!(failure.code, DirectSurfaceLiquidErrorCode::E003);
    assert_eq!(failure.phase, DirectSurfaceLiquidPhase::IndependentClosure);
    assert_eq!(failure.context.ofe_id, Some(ofe("upper")));
    assert_eq!(
        failure.rollback.beginning_owner_sha256.as_deref(),
        Some(resource.beginning_state().state_sha256.as_str())
    );
    assert_eq!(
        failure.rollback.attempted_owner_sha256.as_deref(),
        Some(arithmetic.ending_state.state_sha256.as_str())
    );

    let mut capacity = candidate.clone();
    capacity
        .closure_operands
        .poison_partition_capacity_bound_for_test(0);
    let capacity_error = capacity
        .validate(&configuration, &resource, &input)
        .expect_err("cumulative infiltration above capacity but below supply");
    let capacity_failure = capacity_error.failure().expect("typed capacity failure");
    assert_eq!(capacity_failure.code, DirectSurfaceLiquidErrorCode::E003);
    assert_eq!(capacity_failure.context.ofe_id, Some(ofe("upper")));
    assert_eq!(
        capacity_failure.rollback.beginning_owner_sha256.as_deref(),
        Some(resource.beginning_state().state_sha256.as_str())
    );

    let mut identity_over_closure = candidate;
    identity_over_closure
        .closure_operands
        .remove_partition_input_for_test(1);
    identity_over_closure.ending_state.records[0].liquid_kg_m2_tile += 0.001;
    identity_over_closure.ending_state.state_sha256 = identity_over_closure
        .ending_state
        .recomputed_sha256()
        .expect("identity attempted digest");
    assert_eq!(
        identity_over_closure
            .validate(&configuration, &resource, &input)
            .expect_err("E009 must outrank E010")
            .code(),
        DirectSurfaceLiquidErrorCode::E009
    );
}

#[test]
fn producer_identity_helper_attributes_reorder_and_replacement_to_actual_rows() {
    let expected = vec![ofe("upper"), ofe("middle"), ofe("lower")];
    let reordered = vec![ofe("middle"), ofe("upper"), ofe("lower")];
    let replacement = vec![ofe("replacement"), ofe("middle"), ofe("lower")];
    assert_eq!(
        first_identity_aware_mismatch(&reordered, &expected, Clone::clone),
        Some(&reordered[0])
    );
    assert_eq!(
        first_identity_aware_mismatch(&replacement, &expected, Clone::clone),
        Some(&replacement[0])
    );

    let expected_map = expected
        .iter()
        .cloned()
        .map(|key| (key, 1))
        .collect::<BTreeMap<_, _>>();
    let mut replacement_map = expected_map.clone();
    replacement_map.remove(&ofe("upper"));
    replacement_map.insert(ofe("replacement"), 1);
    assert_eq!(
        first_map_identity_mismatch(&replacement_map, &expected_map),
        Some(&ofe("replacement"))
    );
}

#[allow(clippy::too_many_arguments)]
fn assert_producer_e009(
    error: &DirectSurfaceLiquidError,
    transaction_id: TransactionId,
    configuration: &DirectSurfaceLiquidConfiguration,
    ofe_id: Option<&OfeId>,
    tile_id: Option<&TileId>,
    parcel_id: Option<&str>,
    beginning_sha256: &str,
    attempted_sha256: &str,
) {
    let failure = error.failure().expect("canonical producer failure");
    assert_eq!(failure.code, DirectSurfaceLiquidErrorCode::E009);
    assert_eq!(failure.phase, DirectSurfaceLiquidPhase::IngressCandidate);
    assert_eq!(failure.context.transaction_id, Some(transaction_id));
    assert_eq!(
        failure.context.owner_id,
        Some(configuration.owner_id.clone())
    );
    assert_eq!(failure.context.ofe_id.as_ref(), ofe_id);
    assert_eq!(failure.context.tile_id.as_ref(), tile_id);
    assert_eq!(failure.context.parcel_id.as_deref(), parcel_id);
    assert_eq!(
        failure.rollback.beginning_owner_sha256.as_deref(),
        Some(beginning_sha256)
    );
    assert_eq!(
        failure.rollback.attempted_owner_sha256.as_deref(),
        Some(attempted_sha256)
    );
}

#[test]
fn finite_ingress_enthalpy_overflow_fails_before_candidate() {
    let configuration = one_tile_configuration(DirectGroundIngressMode::OpenRawPrecipitation);
    let beginning = initial_state(&configuration, 0.0);
    let transaction_id = TransactionId(393);
    let resource = resource_candidate(&configuration, &beginning, transaction_id, None, &[]);
    let input = DirectSurfaceLiquidIngressInput {
        transaction_id,
        day_index: 3,
        interval_index: 0,
        interval_s: INTERVAL_S,
        tile_ingress: vec![open_ingress(&configuration.records[0], f64::MAX / 2.0)],
        wb14_parameters: parameters(&configuration),
    };
    let error = execute_surface_liquid_ingress(&configuration, &resource, &input)
        .expect_err("finite parcel enthalpy overflow must fail closed");
    assert_eq!(error.code(), DirectSurfaceLiquidErrorCode::E003);
}

#[test]
fn finite_routing_area_underflow_fails_before_receipt() {
    let mut configuration = routed_configuration();
    configuration.records[0].ofe_area_m2 = f64::MIN_POSITIVE;
    configuration.records[1].ofe_area_m2 = f64::MAX;
    configuration.configuration_sha256 = configuration.recomputed_sha256().expect("digest");
    configuration.validate().expect("finite extreme areas");
    let source = &configuration.records[0];
    let parcel = TimedParcel {
        parcel_id: "underflow-route".to_owned(),
        origin_store_key: source.key.clone(),
        recipient_store_key: source.key.clone(),
        basis_ofe_id: source.key.ofe_id.clone(),
        kind: DirectSurfaceLiquidParcelKind::RawPrecipitation,
        start_s: 0.0,
        end_s: INTERVAL_S,
        mass_kg_m2_basis_ofe_ground: 1.0,
        enthalpy_j_m2_basis_ofe_ground: 1.0,
    };
    let mut pending = BTreeMap::new();
    let mut receipts = Vec::new();
    let error = route_runoff(
        &configuration,
        &source.key.ofe_id,
        vec![parcel],
        &mut pending,
        &mut receipts,
        TransactionId(394),
    )
    .expect_err("finite area-ratio underflow must fail closed");
    assert_eq!(error.code(), DirectSurfaceLiquidErrorCode::E003);
    assert!(pending.is_empty());
    assert!(receipts.is_empty());
}

#[test]
fn wb14_failure_preserves_every_resource_candidate_byte() {
    let configuration = one_tile_configuration(DirectGroundIngressMode::OpenRawPrecipitation);
    let beginning = initial_state(&configuration, 0.0);
    let first_transaction_id = TransactionId(401);
    let first_resource =
        resource_candidate(&configuration, &beginning, first_transaction_id, None, &[]);
    let first = execute_surface_liquid_ingress(
        &configuration,
        &first_resource,
        &DirectSurfaceLiquidIngressInput {
            transaction_id: first_transaction_id,
            day_index: 3,
            interval_index: 0,
            interval_s: INTERVAL_S,
            tile_ingress: vec![open_ingress(&configuration.records[0], 0.1)],
            wb14_parameters: vec![DirectOfeWb14Parameters {
                ofe_id: configuration.ofe_topology[0].clone(),
                effective_conductivity_m_s: 1.0e-6,
                matric_potential_m: 0.1,
                infiltration_storage_capacity_m: 1.0,
            }],
        },
    )
    .expect("first continuation");
    assert!(first.ending_state.continuations[0].cumulative_infiltration_m > 0.0);
    let transaction_id = TransactionId(402);
    let resource = resource_candidate(
        &configuration,
        &first.ending_state,
        transaction_id,
        Some(first_transaction_id),
        &[],
    );
    let input = DirectSurfaceLiquidIngressInput {
        transaction_id,
        day_index: 3,
        interval_index: 1,
        interval_s: INTERVAL_S,
        tile_ingress: vec![open_ingress(&configuration.records[0], 0.1)],
        wb14_parameters: vec![DirectOfeWb14Parameters {
            ofe_id: configuration.ofe_topology[0].clone(),
            effective_conductivity_m_s: 1.0e-6,
            matric_potential_m: 0.1,
            infiltration_storage_capacity_m: 0.0,
        }],
    };
    let before_candidate = resource.clone();
    let before = (
        resource
            .beginning_state()
            .canonical_bytes(&configuration)
            .expect("beginning bytes before"),
        resource
            .working_state()
            .canonical_bytes(&configuration)
            .expect("working bytes before"),
    );
    let error = execute_surface_liquid_ingress(&configuration, &resource, &input)
        .expect_err("invalid continuation bound");
    let failure = error.failure().expect("canonical failure");
    assert_eq!(failure.code, DirectSurfaceLiquidErrorCode::E008);
    assert_eq!(failure.phase, DirectSurfaceLiquidPhase::IngressCandidate);
    assert_eq!(failure.context.transaction_id, Some(transaction_id));
    assert_eq!(
        failure.rollback.beginning_owner_sha256.as_deref(),
        Some(resource.beginning_state().state_sha256.as_str())
    );
    assert!(failure.rollback.attempted_owner_sha256.is_some());
    let after = (
        resource
            .beginning_state()
            .canonical_bytes(&configuration)
            .expect("beginning bytes after"),
        resource
            .working_state()
            .canonical_bytes(&configuration)
            .expect("working bytes after"),
    );
    assert_eq!(after, before);
    assert_eq!(resource, before_candidate);
}

#[test]
fn sealed_ingress_candidate_reconstructs_and_rejects_forgery() {
    let configuration = one_tile_configuration(DirectGroundIngressMode::OpenRawPrecipitation);
    let beginning = initial_state(&configuration, 0.0);
    let transaction_id = TransactionId(412);
    let resource = resource_candidate(&configuration, &beginning, transaction_id, None, &[]);
    let input = DirectSurfaceLiquidIngressInput {
        transaction_id,
        day_index: 3,
        interval_index: 0,
        interval_s: INTERVAL_S,
        tile_ingress: vec![open_ingress(&configuration.records[0], 0.1)],
        wb14_parameters: parameters(&configuration),
    };
    let mut candidate =
        execute_surface_liquid_ingress(&configuration, &resource, &input).expect("valid candidate");
    candidate
        .validate(&configuration, &resource, &input)
        .expect("candidate reconstruction");
    candidate.ending_state.records[0].liquid_kg_m2_tile += 0.25;
    let error = candidate
        .validate(&configuration, &resource, &input)
        .expect_err("forged ending state");
    let failure = error.failure().expect("canonical failure");
    assert_eq!(failure.code, DirectSurfaceLiquidErrorCode::E009);
    assert_eq!(failure.context.transaction_id, Some(transaction_id));
    assert!(failure.rollback.beginning_owner_sha256.is_some());
    assert!(failure.rollback.attempted_owner_sha256.is_some());
}
