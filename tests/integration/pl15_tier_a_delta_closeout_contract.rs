use openwepp_comparator_metadata::{
    COMPMETA_HIGH_CONFIDENCE_SINGLE_OFE_DAILY_MESSAGE_ID, ComparatorConfidenceTier,
    ComparatorSurfaceClass, ComparatorTierRoutingRequest, route_comparator_tier_metadata,
};

const PL14_H5_WAT_COMPARATOR_JSON: &str = include_str!(
    "../../docs/work-packages/20260523-pl14-tier-a-candidate-emission-and-replay-001/artifacts/h5_wat_comparator.json",
);
const PL14_H5_PLOT_COMPARATOR_JSON: &str = include_str!(
    "../../docs/work-packages/20260523-pl14-tier-a-candidate-emission-and-replay-001/artifacts/h5_plot_comparator.json",
);

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Pl15CloseoutVerdict {
    Lift,
    LiftWithRiskAcceptance,
    RetainHold,
}

fn pl15_closeout_verdict(
    unresolved_tier_a_blockers: usize,
    risk_acceptance_approval_reference: Option<&str>,
) -> Pl15CloseoutVerdict {
    if unresolved_tier_a_blockers == 0 {
        return Pl15CloseoutVerdict::Lift;
    }

    match risk_acceptance_approval_reference {
        Some(reference) if !reference.trim().is_empty() => {
            Pl15CloseoutVerdict::LiftWithRiskAcceptance
        }
        _ => Pl15CloseoutVerdict::RetainHold,
    }
}

fn contains_all(haystack: &str, needles: &[&str]) -> bool {
    needles.iter().all(|needle| haystack.contains(needle))
}

#[test]
fn pl15_contract_conformance_routes_tier_a_surface_as_higher_confidence() {
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
fn pl15_contract_conformance_flags_wat_structure_delta_from_pl14_replay() {
    assert!(contains_all(
        PL14_H5_WAT_COMPARATOR_JSON,
        &[
            "\"strict_pass\": false",
            "\"status\": \"structure_diff\"",
            "\"line_count_baseline\": 1123",
            "\"line_count_candidate\": 5",
            "\"numeric_values_compared\": 0"
        ]
    ));
}

#[test]
fn pl15_contract_conformance_flags_plot_artifact_absence_from_pl14_replay() {
    assert!(contains_all(
        PL14_H5_PLOT_COMPARATOR_JSON,
        &[
            "\"strict_pass\": false",
            "\"only_baseline_count\": 1",
            "\"H5.plot.dat\""
        ]
    ));
}

#[test]
fn pl15_contract_conformance_requires_explicit_risk_acceptance_reference() {
    let unresolved_blockers = 2;

    let without_reference = pl15_closeout_verdict(unresolved_blockers, None);
    assert_eq!(without_reference, Pl15CloseoutVerdict::RetainHold);

    let with_reference = pl15_closeout_verdict(
        unresolved_blockers,
        Some("pl15-risk-acceptance-approval-reference.md#RA-PL15-001"),
    );
    assert_eq!(with_reference, Pl15CloseoutVerdict::LiftWithRiskAcceptance);
}
