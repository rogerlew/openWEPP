use super::*;
use crate::hillslope::intake_lane_setup::build_static_per_ofe_lane_slices;
use openwepp_hillslope_orchestrator::{
    PerOfeDailyWaterBalanceCollection, PerOfeDailyWaterBalanceError,
    PerOfeDailyWaterBalanceRecord, TransferInput, TransferOutput,
};
use openwepp_input_contract::parsers::soil::{SoilDatver, SoilOfe, SoilProfile};

#[test]
fn mofe01_me1_single_ofe_collection_round_trips_legacy_aggregate_surface() {
    let mut aggregate = HillslopeWritebackSurface::default();
    aggregate
        .state_surface
        .insert(BoundarySymbol::from("soil_water_total"), BoundaryValue::scalar(42.0));
    aggregate
        .state_surface
        .insert(BoundarySymbol::from("ofe1_width"), BoundaryValue::scalar(30.0));
    aggregate
        .flux_surface
        .insert(BoundarySymbol::from("Q"), BoundaryValue::scalar(3.25));

    let mut collection = PerOfeDailyWaterBalanceCollection::new(1, 1)
        .expect("single-OFE collection should construct");
    collection
        .push_record(
            PerOfeDailyWaterBalanceRecord::from_legacy_single_ofe_aggregate_surface(
                1,
                1,
                aggregate.clone(),
            )
            .expect("single-OFE aggregate adapter should construct"),
        )
        .expect("single OFE record should append");

    let round_trip = collection
        .aggregate_for_legacy_outer_consumers()
        .expect("N=1 shadow collection must round-trip to scalar adapter");

    assert_eq!(collection.simulation_day_index(), 1);
    assert_eq!(collection.contributor_ofe_count(), 1);
    assert_eq!(collection.record_count(), 1);
    assert_eq!(round_trip.state_surface, aggregate.state_surface);
    assert_eq!(round_trip.flux_surface, aggregate.flux_surface);
}

#[test]
fn mofe01_me1_multi_ofe_collection_rejects_aggregate_adapter() {
    let mut collection = PerOfeDailyWaterBalanceCollection::new(1, 2)
        .expect("two-OFE collection should construct");
    collection
        .push_record(
            PerOfeDailyWaterBalanceRecord::from_legacy_single_ofe_aggregate_surface(
                1,
                1,
                HillslopeWritebackSurface::default(),
            )
            .expect("single-OFE aggregate adapter should construct"),
        )
        .expect_err("terminal aggregate adapter must not satisfy a multi-OFE collection");

    let aggregate_error = collection
        .aggregate_for_legacy_outer_consumers()
        .expect_err("multi-OFE collection must not derive aggregate state in M-E1");

    assert_eq!(
        aggregate_error,
        PerOfeDailyWaterBalanceError::MultiOfeAggregateNotImplemented {
            contributor_ofe_count: 2,
        }
    );
}

#[test]
fn mofe01_me1_transfer_output_uses_recorded_adjacent_recipient() {
    let mut output = TransferOutput::zero_for_terminal_ofe(1);
    output.recipient_ofe_id = Some(2);
    output.surface_carry[0] = 0.25;
    output.lateral_carry[3] = 0.5;

    let downstream = output
        .as_downstream_input()
        .expect("adjacent nonterminal output should become downstream input");

    assert_eq!(downstream.source_ofe_id, Some(1));
    assert_eq!(downstream.recipient_ofe_id, 2);
    assert!((downstream.upstrmq - 0.25).abs() < 1.0e-12);
    assert!((downstream.subrin - 0.5).abs() < 1.0e-12);

    let terminal_error = TransferOutput::zero_for_terminal_ofe(1)
        .as_downstream_input()
        .expect_err("terminal output must not synthesize a downstream input");
    assert_eq!(
        terminal_error,
        PerOfeDailyWaterBalanceError::TransferOutputRecipientMismatch {
            source_ofe_id: 1,
            expected_recipient_ofe_id: Some(2),
            observed_recipient_ofe_id: None,
        }
    );
}

#[test]
fn mofe01_me1_collection_rejects_mismatched_transfer_identity() {
    let mut collection = PerOfeDailyWaterBalanceCollection::new(1, 2)
        .expect("two-OFE collection should construct");

    let mut output = TransferOutput::zero_for_terminal_ofe(2);
    output.source_ofe_id = 99;
    output.recipient_ofe_id = Some(2);

    let error = collection
        .push_record(
            PerOfeDailyWaterBalanceRecord::new(
                1,
                1,
                1,
                HillslopeWritebackSurface::default(),
                HillslopeWritebackSurface::default(),
                TransferInput::zero_for_first_ofe(),
                output,
            )
            .expect("explicit record constructor should construct"),
        )
        .expect_err("record transfer source must match record OFE");

    assert_eq!(
        error,
        PerOfeDailyWaterBalanceError::TransferOutputSourceMismatch {
            ofe_id: 1,
            source_ofe_id: 99,
        }
    );
}

#[test]
fn mofe01_me1_collection_accepts_two_ofe_transfer_chain() {
    let mut collection = PerOfeDailyWaterBalanceCollection::new(1, 2)
        .expect("two-OFE collection should construct");

    let mut first_output = TransferOutput::zero_for_terminal_ofe(1);
    first_output.recipient_ofe_id = Some(2);
    first_output.surface_carry[0] = 0.25;
    first_output.lateral_carry[0] = 0.5;
    let second_input = first_output
        .as_downstream_input()
        .expect("first OFE output should become second OFE input");

    collection
        .push_record(
            PerOfeDailyWaterBalanceRecord::new(
                1,
                1,
                1,
                HillslopeWritebackSurface::default(),
                HillslopeWritebackSurface::default(),
                TransferInput::zero_for_first_ofe(),
                first_output,
            )
            .expect("first explicit record should construct"),
        )
        .expect("first OFE transfer record should append");
    collection
        .push_record(
            PerOfeDailyWaterBalanceRecord::new(
                2,
                1,
                1,
                HillslopeWritebackSurface::default(),
                HillslopeWritebackSurface::default(),
                second_input,
                TransferOutput::zero_for_terminal_ofe(2),
            )
            .expect("second explicit record should construct"),
        )
        .expect("second OFE transfer record should append");

    assert_eq!(collection.record_count(), 2);
}

#[test]
fn mofe01_me1_static_slices_have_exact_cardinality_and_unique_ofe_ids() {
    let slope = SlopeProfile {
        datver: 2023.3,
        datver_source: DatverSource::Header,
        ofe_count: 3,
        ofes: vec![
            slope_ofe(0, 30.0, 60.0),
            slope_ofe(1, 35.0, 40.0),
            slope_ofe(2, 25.0, 50.0),
        ],
    };
    let soil = SoilProfile {
        datver: SoilDatver::V9002,
        datver_raw: 9002.0,
        datver_alias_applied: false,
        comment: "fixture".to_string(),
        ntemp: 3,
        ksflag: true,
        ofes: vec![soil_ofe("A"), soil_ofe("B"), soil_ofe("C")],
        restrictive_layer: None,
    };

    let slices = build_static_per_ofe_lane_slices(&slope, &soil, 3)
        .expect("matched slope/soil/management topology should slice");
    let ids: std::collections::BTreeSet<usize> =
        slices.iter().map(|slice| slice.ofe_id).collect();

    assert_eq!(slices.len(), 3);
    assert_eq!(ids.len(), 3);
    assert_eq!(ids.into_iter().collect::<Vec<_>>(), vec![1, 2, 3]);
    assert_eq!(slices[0].slope_ofe_index, 0);
    assert_eq!(slices[1].soil_ofe_index, 1);
    assert_eq!(slices[2].management_ofe_index, 2);
    assert!((slices[0].area_m2 - 1_800.0).abs() < 1.0e-12);
    assert!((slices[1].area_m2 - 1_400.0).abs() < 1.0e-12);
    assert!((slices[2].area_m2 - 1_250.0).abs() < 1.0e-12);
}

#[test]
fn mofe01_me1_static_slices_reject_invalid_topology_and_geometry() {
    let mut slope = SlopeProfile {
        datver: 2023.3,
        datver_source: DatverSource::Header,
        ofe_count: 2,
        ofes: vec![slope_ofe(0, 30.0, 60.0), slope_ofe(1, 0.0, 40.0)],
    };
    let mut soil = SoilProfile {
        datver: SoilDatver::V9002,
        datver_raw: 9002.0,
        datver_alias_applied: false,
        comment: "fixture".to_string(),
        ntemp: 2,
        ksflag: true,
        ofes: vec![soil_ofe("A"), soil_ofe("B")],
        restrictive_layer: None,
    };

    let topology_error = build_static_per_ofe_lane_slices(&slope, &soil, 1)
        .expect_err("management topology mismatch should fail");
    assert!(topology_error.to_string().contains("CLIHILL-E-019"));

    let geometry_error = build_static_per_ofe_lane_slices(&slope, &soil, 2)
        .expect_err("zero width should fail");
    assert!(geometry_error.to_string().contains("finite and > 0.0"));

    slope.ofe_count = 3;
    soil.ntemp = 3;
    let cardinality_error = build_static_per_ofe_lane_slices(&slope, &soil, 3)
        .expect_err("declared OFE count must match slope vector");
    assert!(cardinality_error
        .to_string()
        .contains("slope OFE vector length"));
}

fn slope_ofe(index: usize, width_m: f64, length_m: f64) -> SlopeOfe {
    SlopeOfe {
        index,
        azm: 180.0,
        fwidth: width_m,
        elevation: None,
        nslpts: 2,
        slplen: length_m,
        distance_mode: DistanceMode::Normalized,
        points: vec![
            SlopePoint {
                xinput: 0.0,
                slpinp: 0.02,
            },
            SlopePoint {
                xinput: 1.0,
                slpinp: 0.04,
            },
        ],
    }
}

fn soil_ofe(id: &str) -> SoilOfe {
    SoilOfe {
        slid: id.to_string(),
        texid: "silt_loam".to_string(),
        nsl: 0,
        salb: 0.2,
        sat: 0.55,
        ki: 900_000.0,
        kr: 0.005,
        shcrit: 4.2,
        avke: 10.5,
        policy: None,
        layers: Vec::new(),
    }
}
