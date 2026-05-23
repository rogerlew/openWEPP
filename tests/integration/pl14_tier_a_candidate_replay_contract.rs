use openwepp_comparator_metadata::{
    COMPMETA_ERROR_MISSING_OFE_COUNT_MESSAGE_ID,
    COMPMETA_ERROR_SINGLE_OFE_COUNT_MISMATCH_MESSAGE_ID,
    COMPMETA_HIGH_CONFIDENCE_SINGLE_OFE_DAILY_MESSAGE_ID, ComparatorConfidenceTier,
    ComparatorSurfaceClass, ComparatorTierRoutingError, ComparatorTierRoutingRequest,
    route_comparator_tier_metadata,
};
use openwepp_summary_accumulator::{
    SummaryAccumulatorError, SummaryScalarSurface, WB13_H5_WAT_COLUMNS, Wb13DailyWaterBalanceRow,
    Wb13DailyWaterBalanceSurface,
};

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
fn pl14_contract_conformance_routes_single_ofe_daily_lane_to_higher_confidence() {
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
fn pl14_contract_conformance_rejects_missing_or_mismatched_single_ofe_metadata() {
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

    let mismatch = route_comparator_tier_metadata(ComparatorTierRoutingRequest::new(
        ComparatorSurfaceClass::SingleOfeDailyWaterBalance,
        Some(2),
    ))
    .expect_err("single OFE mismatch should fail");
    assert_eq!(
        mismatch,
        ComparatorTierRoutingError::SingleOfeCountMismatch {
            contributor_ofe_count: 2,
            message_id: COMPMETA_ERROR_SINGLE_OFE_COUNT_MISMATCH_MESSAGE_ID,
        }
    );
}

#[test]
fn pl14_contract_conformance_emits_replay_staging_rows_with_canonical_25_columns() {
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
fn pl14_contract_conformance_rejects_missing_required_replay_symbol() {
    let mut scalars = seeded_wb13_surface().as_map().clone();
    scalars.remove("ProfileDepth");
    let malformed = SummaryScalarSurface::from_map(scalars).expect("map remains non-empty");

    let error = Wb13DailyWaterBalanceRow::from_surface(1, 1, 2008, &malformed)
        .expect_err("missing replay symbol should fail");

    assert_eq!(
        error,
        SummaryAccumulatorError::MissingRequiredOutputSymbol {
            symbol: "ProfileDepth".to_string(),
        }
    );
}
