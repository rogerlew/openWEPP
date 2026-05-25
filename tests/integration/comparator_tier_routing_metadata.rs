use openwepp_comparator_metadata::{
    COMPMETA_ERROR_MISSING_OFE_COUNT_MESSAGE_ID,
    COMPMETA_HIGH_CONFIDENCE_SINGLE_OFE_DAILY_MESSAGE_ID, COMPMETA_INVESTIGATION_HOURLY_MESSAGE_ID,
    COMPMETA_INVESTIGATION_WATERSHED_MESSAGE_ID, ComparatorConfidenceTier, ComparatorSurfaceClass,
    ComparatorTierRoutingError, ComparatorTierRoutingFailureClass, ComparatorTierRoutingRequest,
    route_comparator_tier_metadata,
};
use openwepp_summary_accumulator::{
    CalendarDay, SummaryAccumulator, SummaryAccumulatorError, SummaryScalarSurface, SummaryWindow,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum StrictLanePolicyMode {
    StrictRequired,
    StrictEquivalentRequired,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum CandidateSourceClass {
    NativeRuntimeDat,
    ConversionDerivedDat,
    NativeRuntimeParquet,
}

fn strict_lane_policy_for_extension(candidate_extension: &str) -> StrictLanePolicyMode {
    if candidate_extension.eq_ignore_ascii_case(".dat") {
        StrictLanePolicyMode::StrictRequired
    } else {
        StrictLanePolicyMode::StrictEquivalentRequired
    }
}

fn candidate_source_class_for_extension(
    candidate_extension: &str,
    conversion_derived_dat: bool,
) -> CandidateSourceClass {
    if candidate_extension.eq_ignore_ascii_case(".dat") {
        if conversion_derived_dat {
            CandidateSourceClass::ConversionDerivedDat
        } else {
            CandidateSourceClass::NativeRuntimeDat
        }
    } else {
        CandidateSourceClass::NativeRuntimeParquet
    }
}

#[test]
fn deterministic_tier_mapping_routes_single_ofe_daily_to_higher_confidence() {
    let metadata = route_comparator_tier_metadata(ComparatorTierRoutingRequest::new(
        ComparatorSurfaceClass::SingleOfeDailyWaterBalance,
        Some(1),
    ))
    .expect("single OFE daily route should succeed");

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
fn deterministic_tier_mapping_routes_hourly_and_watershed_to_investigation() {
    let hourly = route_comparator_tier_metadata(ComparatorTierRoutingRequest::new(
        ComparatorSurfaceClass::HourlyWaterBalance,
        Some(2),
    ))
    .expect("hourly route should succeed");
    assert_eq!(
        hourly.confidence_tier,
        ComparatorConfidenceTier::Investigation
    );
    assert_eq!(hourly.message_id, COMPMETA_INVESTIGATION_HOURLY_MESSAGE_ID);

    let watershed = route_comparator_tier_metadata(ComparatorTierRoutingRequest::new(
        ComparatorSurfaceClass::WatershedWaterBalance,
        None,
    ))
    .expect("watershed route should succeed");
    assert_eq!(
        watershed.confidence_tier,
        ComparatorConfidenceTier::Investigation
    );
    assert_eq!(
        watershed.message_id,
        COMPMETA_INVESTIGATION_WATERSHED_MESSAGE_ID
    );
}

#[test]
fn missing_required_single_ofe_metadata_is_typed_error() {
    let error = route_comparator_tier_metadata(ComparatorTierRoutingRequest::new(
        ComparatorSurfaceClass::SingleOfeDailyWaterBalance,
        None,
    ))
    .expect_err("missing OFE count should fail");

    assert_eq!(
        error,
        ComparatorTierRoutingError::MissingRequiredMetadata {
            field: "contributor_ofe_count",
            message_id: COMPMETA_ERROR_MISSING_OFE_COUNT_MESSAGE_ID,
        }
    );
    assert_eq!(
        error.failure_class(),
        ComparatorTierRoutingFailureClass::MissingRequiredMetadata
    );
    assert_eq!(
        error.message_id(),
        COMPMETA_ERROR_MISSING_OFE_COUNT_MESSAGE_ID
    );
}

#[test]
fn summary_rollups_propagate_routing_metadata() {
    let mut accumulator = SummaryAccumulator::new(ComparatorTierRoutingRequest::single_ofe_daily())
        .expect("routing should be valid");

    let day_1 = CalendarDay::new(2026, 3, 9).expect("valid day");
    let day_2 = CalendarDay::new(2026, 3, 10).expect("valid day");

    let delta_1 =
        SummaryScalarSurface::from_pairs([("runoff", 1.0), ("sed", 0.5)]).expect("surface");
    let delta_2 =
        SummaryScalarSurface::from_pairs([("runoff", 2.0), ("sed", 1.0)]).expect("surface");

    let first = accumulator
        .accumulate_day(day_1, delta_1)
        .expect("first day should accumulate");
    assert!(first.is_empty());

    let second = accumulator
        .accumulate_day(day_2, delta_2)
        .expect("second day should emit daily rollup");
    assert_eq!(second.emitted_rollups.len(), 1);

    let rollup = &second.emitted_rollups[0];
    assert_eq!(rollup.window, SummaryWindow::Daily);
    assert_eq!(
        rollup.comparator_metadata.confidence_tier,
        ComparatorConfidenceTier::HigherConfidence
    );
    assert_eq!(
        rollup.comparator_metadata.message_id,
        COMPMETA_HIGH_CONFIDENCE_SINGLE_OFE_DAILY_MESSAGE_ID
    );
}

#[test]
fn summary_constructor_rejects_invalid_routing_metadata() {
    let error = SummaryAccumulator::new(ComparatorTierRoutingRequest::new(
        ComparatorSurfaceClass::SingleOfeDailyWaterBalance,
        None,
    ))
    .expect_err("invalid routing metadata should fail");

    assert!(matches!(
        error,
        SummaryAccumulatorError::ComparatorMetadata(
            ComparatorTierRoutingError::MissingRequiredMetadata { .. }
        )
    ));
}

#[test]
fn strict_lane_policy_mode_is_deterministic_by_candidate_extension() {
    assert_eq!(
        strict_lane_policy_for_extension(".dat"),
        StrictLanePolicyMode::StrictRequired
    );
    assert_eq!(
        strict_lane_policy_for_extension(".DAT"),
        StrictLanePolicyMode::StrictRequired
    );
    assert_eq!(
        strict_lane_policy_for_extension(".parquet"),
        StrictLanePolicyMode::StrictEquivalentRequired
    );
}

#[test]
fn candidate_source_class_policy_distinguishes_native_vs_conversion_dat() {
    assert_eq!(
        candidate_source_class_for_extension(".dat", false),
        CandidateSourceClass::NativeRuntimeDat
    );
    assert_eq!(
        candidate_source_class_for_extension(".dat", true),
        CandidateSourceClass::ConversionDerivedDat
    );
    assert_eq!(
        candidate_source_class_for_extension(".parquet", false),
        CandidateSourceClass::NativeRuntimeParquet
    );
}
