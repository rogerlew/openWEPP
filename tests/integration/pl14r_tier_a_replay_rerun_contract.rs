use openwepp_comparator_metadata::{
    COMPMETA_ERROR_MISSING_OFE_COUNT_MESSAGE_ID,
    COMPMETA_HIGH_CONFIDENCE_SINGLE_OFE_DAILY_MESSAGE_ID, ComparatorConfidenceTier,
    ComparatorSurfaceClass, ComparatorTierRoutingError, ComparatorTierRoutingRequest,
    route_comparator_tier_metadata,
};
use openwepp_summary_accumulator::{
    SummaryScalarSurface, WB13_H5_WAT_COLUMNS, Wb13DailyWaterBalanceRow,
    Wb13DailyWaterBalanceSurface,
};

const PL14R_REQUIRED_REPLAY_INCLUDE_SURFACES: [&str; 2] = ["H5.wat.dat", "H5.plot.dat"];

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Pl14rReplayGateVerdict {
    Pass,
    Hold,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct ReplaySurfaceStatus {
    strict_pass: bool,
    present_in_candidate_lane: bool,
}

fn pl14r_replay_gate_verdict(
    h5_wat: ReplaySurfaceStatus,
    h5_plot: ReplaySurfaceStatus,
    provenance_hashes_complete: bool,
) -> Pl14rReplayGateVerdict {
    if !provenance_hashes_complete {
        return Pl14rReplayGateVerdict::Hold;
    }

    for surface in [h5_wat, h5_plot] {
        if !surface.present_in_candidate_lane || !surface.strict_pass {
            return Pl14rReplayGateVerdict::Hold;
        }
    }

    Pl14rReplayGateVerdict::Pass
}

fn seeded_wb13_surface() -> SummaryScalarSurface {
    SummaryScalarSurface::from_pairs([
        ("P", 4.40),
        ("RM", 0.00),
        ("Q", 0.0),
        ("Ep", 0.57),
        ("Es", 0.61),
        ("Er", 0.00),
        ("Dp", 0.02),
        ("UpStrmQ", 0.0),
        ("SubRIn", 0.00),
        ("latqcc", 0.80),
        ("Total-Soil", 71.30),
        ("frozwt", 0.00),
        ("Snow-Water", 4.40),
        ("QOFE", 0.0),
        ("Tile", 0.00),
        ("Irr", 0.00),
        ("Area", 9891.92),
        ("SoilWaterTotal", 71.30),
        ("ProfileDepth", 400.00),
        ("ProfilePorosityCap", 171.48),
        ("ProfileFCStore", 38.75),
        ("ProfileWPStore", 14.38),
    ])
    .expect("valid seeded WB13 scalar surface")
}

#[test]
fn pl14r_contract_conformance_routes_single_ofe_daily_lane_to_higher_confidence() {
    let metadata = route_comparator_tier_metadata(ComparatorTierRoutingRequest::new(
        ComparatorSurfaceClass::SingleOfeDailyWaterBalance,
        Some(1),
    ))
    .expect("single OFE replay lane should route");

    assert_eq!(
        metadata.surface_class,
        ComparatorSurfaceClass::SingleOfeDailyWaterBalance
    );
    assert_eq!(
        metadata.confidence_tier,
        ComparatorConfidenceTier::HigherConfidence
    );
    assert_eq!(
        metadata.message_id,
        COMPMETA_HIGH_CONFIDENCE_SINGLE_OFE_DAILY_MESSAGE_ID
    );
}

#[test]
fn pl14r_contract_conformance_rejects_missing_required_single_ofe_metadata() {
    let missing = route_comparator_tier_metadata(ComparatorTierRoutingRequest::new(
        ComparatorSurfaceClass::SingleOfeDailyWaterBalance,
        None,
    ))
    .expect_err("missing OFE count should fail");

    assert_eq!(
        missing,
        ComparatorTierRoutingError::MissingRequiredMetadata {
            field: "contributor_ofe_count",
            message_id: COMPMETA_ERROR_MISSING_OFE_COUNT_MESSAGE_ID,
        }
    );
}

#[test]
fn pl14r_contract_conformance_wb13_rows_remain_canonical_25_column_schema() {
    let row_1 = Wb13DailyWaterBalanceRow::from_surface(1, 1, 2008, &seeded_wb13_surface())
        .expect("first row should build");
    let row_2 = Wb13DailyWaterBalanceRow::from_surface(1, 2, 2008, &seeded_wb13_surface())
        .expect("second row should build");

    let mut surface = Wb13DailyWaterBalanceSurface::new();
    surface.append_row(row_1).expect("first row should append");
    surface.append_row(row_2).expect("second row should append");

    assert_eq!(
        Wb13DailyWaterBalanceSurface::column_headers(),
        &WB13_H5_WAT_COLUMNS
    );

    let rendered = surface.render_h5_wat_dat();
    let numeric_rows: Vec<&str> = rendered
        .lines()
        .filter(|line| {
            line.split_whitespace()
                .next()
                .is_some_and(|token| token.parse::<f64>().is_ok())
        })
        .collect();

    assert_eq!(numeric_rows.len(), 2);
    assert_eq!(numeric_rows[0].split_whitespace().count(), 25);
    assert_eq!(numeric_rows[1].split_whitespace().count(), 25);
}

#[test]
fn pl14r_contract_conformance_requires_h5_wat_and_h5_plot_include_surfaces() {
    assert_eq!(
        PL14R_REQUIRED_REPLAY_INCLUDE_SURFACES,
        ["H5.wat.dat", "H5.plot.dat"]
    );
}

#[test]
fn pl14r_contract_conformance_holds_when_required_surface_missing_or_strict_failure_present() {
    let missing_plot = pl14r_replay_gate_verdict(
        ReplaySurfaceStatus {
            strict_pass: false,
            present_in_candidate_lane: true,
        },
        ReplaySurfaceStatus {
            strict_pass: false,
            present_in_candidate_lane: false,
        },
        true,
    );
    assert_eq!(missing_plot, Pl14rReplayGateVerdict::Hold);

    let strict_failure = pl14r_replay_gate_verdict(
        ReplaySurfaceStatus {
            strict_pass: false,
            present_in_candidate_lane: true,
        },
        ReplaySurfaceStatus {
            strict_pass: true,
            present_in_candidate_lane: true,
        },
        true,
    );
    assert_eq!(strict_failure, Pl14rReplayGateVerdict::Hold);
}

#[test]
fn pl14r_contract_conformance_requires_complete_hash_provenance_for_pass() {
    let missing_hashes = pl14r_replay_gate_verdict(
        ReplaySurfaceStatus {
            strict_pass: true,
            present_in_candidate_lane: true,
        },
        ReplaySurfaceStatus {
            strict_pass: true,
            present_in_candidate_lane: true,
        },
        false,
    );
    assert_eq!(missing_hashes, Pl14rReplayGateVerdict::Hold);

    let complete = pl14r_replay_gate_verdict(
        ReplaySurfaceStatus {
            strict_pass: true,
            present_in_candidate_lane: true,
        },
        ReplaySurfaceStatus {
            strict_pass: true,
            present_in_candidate_lane: true,
        },
        true,
    );
    assert_eq!(complete, Pl14rReplayGateVerdict::Pass);
}
