use openwepp_comparator_metadata::{
    COMPMETA_HIGH_CONFIDENCE_SINGLE_OFE_DAILY_MESSAGE_ID, ComparatorConfidenceTier,
    ComparatorSurfaceClass, ComparatorTierRoutingRequest, route_comparator_tier_metadata,
};

const PL14R_H5_WAT_COMPARATOR_SCHEMA_ALIGNED_JSON: &str = include_str!(
    "../../docs/work-packages/20260523-pl14r-tier-a-candidate-emission-and-replay-rerun-001/artifacts/h5_wat_comparator_schema_aligned.json",
);
const PL14R_H5_PLOT_COMPARATOR_SCHEMA_ALIGNED_JSON: &str = include_str!(
    "../../docs/work-packages/20260523-pl14r-tier-a-candidate-emission-and-replay-rerun-001/artifacts/h5_plot_comparator_schema_aligned.json",
);
const PL14R_H5_WAT_DAY_BY_DAY_SCHEMA_ALIGNED_JSON: &str = include_str!(
    "../../docs/work-packages/20260523-pl14r-tier-a-candidate-emission-and-replay-rerun-001/artifacts/h5_wat_day_by_day_schema_aligned.json",
);

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Pl15rCloseoutVerdict {
    Lift,
    LiftWithRiskAcceptance,
    RetainHold,
}

fn pl15r_closeout_verdict(
    unresolved_tier_a_blockers: usize,
    risk_acceptance_approval_reference: Option<&str>,
) -> Pl15rCloseoutVerdict {
    if unresolved_tier_a_blockers == 0 {
        return Pl15rCloseoutVerdict::Lift;
    }

    match risk_acceptance_approval_reference {
        Some(reference) if !reference.trim().is_empty() => {
            Pl15rCloseoutVerdict::LiftWithRiskAcceptance
        }
        _ => Pl15rCloseoutVerdict::RetainHold,
    }
}

fn contains_all(haystack: &str, needles: &[&str]) -> bool {
    needles.iter().all(|needle| haystack.contains(needle))
}

#[test]
fn pl15r_contract_conformance_routes_tier_a_surface_as_higher_confidence() {
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
fn pl15r_contract_conformance_requires_schema_aligned_wat_strict_pass() {
    assert!(contains_all(
        PL14R_H5_WAT_COMPARATOR_SCHEMA_ALIGNED_JSON,
        &["\"strict_pass\": true", "\"identical\": 1"]
    ));
}

#[test]
fn pl15r_contract_conformance_requires_schema_aligned_plot_strict_pass() {
    assert!(contains_all(
        PL14R_H5_PLOT_COMPARATOR_SCHEMA_ALIGNED_JSON,
        &["\"strict_pass\": true", "\"identical\": 1"]
    ));
}

#[test]
fn pl15r_contract_conformance_requires_day_by_day_25_measure_parity() {
    assert!(contains_all(
        PL14R_H5_WAT_DAY_BY_DAY_SCHEMA_ALIGNED_JSON,
        &[
            "\"all_columns_exact\": true",
            "\"common_row_count\": 1095",
            "\"upcast_numeric_row_width\": 25"
        ]
    ));
}

#[test]
fn pl15r_contract_conformance_requires_explicit_risk_acceptance_only_when_blockers_remain() {
    let no_blockers = pl15r_closeout_verdict(0, None);
    assert_eq!(no_blockers, Pl15rCloseoutVerdict::Lift);

    let blockers_without_reference = pl15r_closeout_verdict(1, None);
    assert_eq!(blockers_without_reference, Pl15rCloseoutVerdict::RetainHold);

    let blockers_with_reference = pl15r_closeout_verdict(
        1,
        Some("pl15r-risk-acceptance-approval-reference.md#RA-PL15R-001"),
    );
    assert_eq!(
        blockers_with_reference,
        Pl15rCloseoutVerdict::LiftWithRiskAcceptance
    );
}
