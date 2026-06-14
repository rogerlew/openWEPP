use super::*;
use crate::hillslope::intake_lane_setup::build_static_per_ofe_lane_slices;
use openwepp_hillslope_output::hillslope_pass::HillslopePassRow;
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

#[test]
fn mofe01_me4_redo_internal_wb13_records_close_true_transfer_and_storage_identities() {
    let mut first_output = TransferOutput::zero_for_terminal_ofe(1);
    first_output.recipient_ofe_id = Some(2);
    first_output.surface_carry[0] = 0.002;
    first_output.lateral_carry[0] = 0.003;
    let second_input = first_output
        .as_downstream_input()
        .expect("first OFE output should become second OFE input");

    let collection = DailyInternalPerOfeWb13Collection::from_records(
        2,
        vec![
            me4_internal_record(
                1,
                40.0,
                me4_wb13_row(
                    1,
                    me4_row_spec()
                        .with_total_soil(41.0)
                        .with_rm(6.0 + 5.0e-13)
                        .with_q(5.0),
                ),
                TransferInput::zero_for_first_ofe(),
                first_output,
            ),
            me4_internal_record(
                2,
                43.0,
                me4_wb13_row(
                    2,
                    me4_row_spec()
                        .with_upstrmq(2.0)
                        .with_subrin(3.0)
                        .with_total_soil(44.0)
                        .with_rm(1.0)
                        .with_q(5.0),
                ),
                second_input,
                TransferOutput::zero_for_terminal_ofe(2),
            ),
        ],
    )
    .expect("matching per-OFE WB13 records should close real identities");

    let mut summary = PerOfeInternalWb13RunSummary::default();
    summary
        .observe_day(&collection)
        .expect("closed collection should summarize");

    assert_eq!(summary.day_count, 1);
    assert_eq!(summary.record_count, 2);
    assert_eq!(summary.expected_record_count, 2);
    assert!(summary.transfer_identity_max_abs_mm.abs() < 1.0e-12);
    assert!(summary.per_element_identity_max_abs_mm > 0.0);
    assert!(summary.per_element_identity_max_abs_mm < 1.0e-11);
    assert!(summary.aggregate_transfer_cancellation_max_abs_mm.abs() < 1.0e-12);
    assert!(summary.hillslope_total_identity_max_abs_mm > 0.0);
    assert!(summary.hillslope_total_identity_max_abs_mm < 1.0e-9);
}

#[test]
fn mofe01_mi_hillslope_total_identity_rejects_area_weight_mismatch() {
    let mut first_output = TransferOutput::zero_for_terminal_ofe(1);
    first_output.recipient_ofe_id = Some(2);
    first_output.surface_carry[0] = 0.002;
    first_output.lateral_carry[0] = 0.003;
    let second_input = first_output
        .as_downstream_input()
        .expect("first OFE output should become second OFE input");

    let error = DailyInternalPerOfeWb13Collection::from_records(
        2,
        vec![
            me4_internal_record_with_area(
                1,
                2.0,
                40.0,
                me4_wb13_row(
                    1,
                    me4_row_spec()
                        .with_total_soil(41.0)
                        .with_rm(6.0)
                        .with_q(5.0),
                ),
                TransferInput::zero_for_first_ofe(),
                first_output,
            ),
            me4_internal_record_with_area(
                2,
                1.0,
                43.0,
                me4_wb13_row(
                    2,
                    me4_row_spec()
                        .with_upstrmq(2.0)
                        .with_subrin(3.0)
                        .with_total_soil(44.0)
                        .with_rm(1.0)
                        .with_q(5.0),
                ),
                second_input,
                TransferOutput::zero_for_terminal_ofe(2),
            ),
        ],
    )
    .expect_err("hillslope-total identity must consume OFE areas independently");

    assert!(
        error
            .to_string()
            .contains("hillslope-total identity residual"),
        "unexpected error: {error}"
    );
}

#[test]
fn mofe01_me4_redo_internal_wb13_records_reject_storage_delta_mismatch() {
    let error = DailyInternalPerOfeWb13Collection::from_records(
        1,
        vec![me4_internal_record(
            1,
            40.0,
            me4_wb13_row(
                1,
                me4_row_spec()
                    .with_total_soil(42.0)
                    .with_rm(3.0)
                    .with_q(0.5),
            ),
            TransferInput::zero_for_first_ofe(),
            TransferOutput::zero_for_terminal_ofe(1),
        )],
    )
    .expect_err("storage delta must be measured against pre-day state, not row aliases");

    assert!(
        error.to_string().contains("per-element storage identity residual"),
        "unexpected error: {error}"
    );
}

#[test]
fn mofe01_me4_redo_internal_wb13_records_reject_cross_ofe_transfer_mismatch() {
    let mut first_output = TransferOutput::zero_for_terminal_ofe(1);
    first_output.recipient_ofe_id = Some(2);
    first_output.surface_carry[0] = 0.002;
    let mut second_input = first_output
        .as_downstream_input()
        .expect("first OFE output should become second OFE input");
    second_input.upstrmq = 0.0025;

    let error = DailyInternalPerOfeWb13Collection::from_records(
        2,
        vec![
            me4_internal_record(
                1,
                40.0,
                me4_wb13_row(1, me4_row_spec().with_total_soil(41.0).with_rm(1.0)),
                TransferInput::zero_for_first_ofe(),
                first_output,
            ),
            me4_internal_record(
                2,
                43.0,
                me4_wb13_row(
                    2,
                    me4_row_spec()
                        .with_upstrmq(2.5)
                        .with_total_soil(44.0)
                        .with_rm(3.5)
                        .with_q(5.0),
                ),
                second_input,
                TransferOutput::zero_for_terminal_ofe(2),
            ),
        ],
    )
    .expect_err("OFE 1 sent transfer must match OFE 2 received transfer");

    assert!(
        error.to_string().contains("transfer identity residual"),
        "unexpected error: {error}"
    );
}

#[test]
fn mofe01_me4_redo_internal_wb13_records_include_frost_storage_delta_per_ofe() {
    let collection = DailyInternalPerOfeWb13Collection::from_records(
        1,
        vec![me4_internal_record(
            1,
            11.0,
            me4_wb13_row(
                1,
                me4_row_spec()
                    .with_total_soil(10.0)
                    .with_frozwt(2.0)
                    .with_rm(1.0 + 5.0e-13),
            ),
            TransferInput::zero_for_first_ofe(),
            TransferOutput::zero_for_terminal_ofe(1),
        )],
    )
    .expect("frost-active storage delta must include Total-Soil + frozwt");

    let mut summary = PerOfeInternalWb13RunSummary::default();
    summary
        .observe_day(&collection)
        .expect("frost-active collection should summarize");

    assert!(summary.per_element_identity_max_abs_mm > 0.0);
    assert!(summary.per_element_identity_max_abs_mm < 1.0e-11);
}

#[test]
fn mofe01_tb2_redo_pass_runvol_uses_published_q_area_not_qofe_area() {
    let mut upstream_surface_carry = [0.0_f64; 24];
    upstream_surface_carry[0] = 0.002;
    let upstream_output = TransferOutput {
        source_ofe_id: 1,
        recipient_ofe_id: Some(2),
        surface_carry: upstream_surface_carry,
        lateral_carry: [0.0; 24],
        qofe: 0.002,
        lateral_export: 0.0,
    };
    let downstream_input = TransferInput {
        source_ofe_id: Some(1),
        recipient_ofe_id: 2,
        area_ratio: 1.0,
        surface_carry: upstream_surface_carry,
        lateral_carry: [0.0; 24],
        upstrmq: 0.002,
        subrin: 0.0,
    };
    let outlet_output = TransferOutput {
        source_ofe_id: 2,
        recipient_ofe_id: None,
        surface_carry: [0.0; 24],
        lateral_carry: [0.0; 24],
        qofe: 0.005,
        lateral_export: 0.0,
    };

    let collection = DailyInternalPerOfeWb13Collection::from_records(
        2,
        vec![
            me4_internal_record_with_area(
                1,
                100.0,
                0.0,
                me4_wb13_row(1, me4_row_spec().with_p(4.0).with_rm(2.0).with_q(2.0)),
                TransferInput::zero_for_first_ofe(),
                upstream_output,
            ),
            me4_internal_record_with_area(
                2,
                100.0,
                0.0,
                me4_wb13_row(
                    2,
                    me4_row_spec()
                        .with_area(200.0)
                        .with_p(4.0)
                        .with_upstrmq(2.0)
                        .with_rm(0.5)
                        .with_q(2.5)
                        .with_qofe(5.0),
                ),
                downstream_input,
                outlet_output,
            ),
        ],
    )
    .expect("fixture must close per-OFE and hillslope-total identities");

    let mut pass_rows = Vec::<HillslopePassRow>::new();
    collection
        .append_runoff_delivery_rows_to(17, 200.0, &mut pass_rows)
        .expect("runoff-delivery row should build from outlet record");

    assert_eq!(pass_rows.len(), 1);
    let row = &pass_rows[0];
    assert_eq!(row.wepp_id, 17);
    assert!((row.runvol_m3 - 0.5).abs() <= 1.0e-12);

    let wrong_qofe_publication_area_volume_m3 = 5.0_f64 * 200.0 / 1_000.0;
    assert!((wrong_qofe_publication_area_volume_m3 - 1.0_f64).abs() <= 1.0e-12);
    assert!(
        (row.runvol_m3 - wrong_qofe_publication_area_volume_m3).abs() > 1.0e-9,
        "T-B2-REDO runvol must use Q * publication area, not QOFE * publication area"
    );

    let wrong_q_area_mismatch_volume_m3 = 2.5_f64 * 100.0 / 1_000.0;
    assert!((wrong_q_area_mismatch_volume_m3 - 0.25_f64).abs() <= 1.0e-12);
    assert!(
        (row.runvol_m3 - wrong_q_area_mismatch_volume_m3).abs() > 1.0e-9,
        "T-B2-REDO runvol must use the published Q dual when record area is not an outlet footprint"
    );

    let annual_precip_volume_m3 = 4.0_f64 * 100.0 / 1_000.0 + 4.0 * 100.0 / 1_000.0;
    assert!(
        row.runvol_m3 <= annual_precip_volume_m3,
        "runvol must satisfy the independent annual precipitation bound"
    );

    let per_ofe_sum_volume_m3: f64 = 2.0 * 100.0 / 1_000.0 + 5.0 * 100.0 / 1_000.0;
    assert!((per_ofe_sum_volume_m3 - 0.7).abs() <= 1.0e-12);
    assert!(
        (row.runvol_m3 - per_ofe_sum_volume_m3).abs() > 1.0e-9,
        "T-B2-REDO runvol must be outlet delivery, not the per-OFE volume sum"
    );
}

fn me4_internal_record(
    ofe_id: usize,
    previous_storage_total_mm: f64,
    row: SimulationOwnedWb13Row,
    upstream_transfer_input: TransferInput,
    current_transfer_output: TransferOutput,
) -> InternalPerOfeWb13Record {
    me4_internal_record_with_area(
        ofe_id,
        1.0,
        previous_storage_total_mm,
        row,
        upstream_transfer_input,
        current_transfer_output,
    )
}

fn me4_internal_record_with_area(
    ofe_id: usize,
    area_m2: f64,
    previous_storage_total_mm: f64,
    row: SimulationOwnedWb13Row,
    upstream_transfer_input: TransferInput,
    current_transfer_output: TransferOutput,
) -> InternalPerOfeWb13Record {
    InternalPerOfeWb13Record {
        ofe_id,
        area_m2,
        previous_storage_total_mm,
        physical_surface_outflow_mm: row.wb13_row.q,
        frost_upper_overflow_mm: 0.0,
        frost_internal_adjustment_mm: 0.0,
        storage_reconciliation_detail: "test-fixture".to_string(),
        row,
        upstream_transfer_input,
        current_transfer_output,
    }
}

#[derive(Debug, Clone, Copy)]
struct Me4Wb13RowSpec {
    area: f64,
    p: f64,
    upstrmq: f64,
    subrin: f64,
    total_soil: f64,
    frozwt: f64,
    rm: f64,
    q: f64,
    qofe: f64,
}

impl Me4Wb13RowSpec {
    fn with_area(mut self, value: f64) -> Self {
        self.area = value;
        self
    }

    fn with_p(mut self, value: f64) -> Self {
        self.p = value;
        self
    }

    fn with_upstrmq(mut self, value: f64) -> Self {
        self.upstrmq = value;
        self
    }

    fn with_subrin(mut self, value: f64) -> Self {
        self.subrin = value;
        self
    }

    fn with_total_soil(mut self, value: f64) -> Self {
        self.total_soil = value;
        self
    }

    fn with_frozwt(mut self, value: f64) -> Self {
        self.frozwt = value;
        self
    }

    fn with_rm(mut self, value: f64) -> Self {
        self.rm = value;
        self
    }

    fn with_q(mut self, value: f64) -> Self {
        self.q = value;
        self.qofe = value;
        self
    }

    fn with_qofe(mut self, value: f64) -> Self {
        self.qofe = value;
        self
    }
}

fn me4_row_spec() -> Me4Wb13RowSpec {
    Me4Wb13RowSpec {
        area: 1.0,
        p: 0.0,
        upstrmq: 0.0,
        subrin: 0.0,
        total_soil: 0.0,
        frozwt: 0.0,
        rm: 0.0,
        q: 0.0,
        qofe: 0.0,
    }
}

fn me4_wb13_row(ofe: u16, spec: Me4Wb13RowSpec) -> SimulationOwnedWb13Row {
    SimulationOwnedWb13Row {
        wb13_row: Wb13DailyWaterBalanceRow {
            ofe,
            julian_day: 1,
            year: 2000,
            p: spec.p,
            rm: spec.rm,
            q: spec.q,
            ep: 0.0,
            es: 0.0,
            er: 0.0,
            dp: 0.0,
            upstrmq: spec.upstrmq,
            subrin: spec.subrin,
            latqcc: 0.0,
            total_soil: spec.total_soil,
            frozwt: spec.frozwt,
            snow_water: 0.0,
            qofe: spec.qofe,
            tile: 0.0,
            irr: 0.0,
            area: spec.area,
            soil_water_total: spec.total_soil,
            profile_depth: 1_000.0,
            profile_porosity_cap: 500.0,
            profile_fc_store: 300.0,
            profile_wp_store: 100.0,
        },
        interception_mm: 0.0,
        frdp_mm: 0.0,
        month: 1,
        day_of_month: 1,
        water_year: 2000,
        sim_day_index: 1,
    }
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
