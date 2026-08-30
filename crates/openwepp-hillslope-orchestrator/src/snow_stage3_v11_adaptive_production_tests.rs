use crate::STAGE3_V11_ADAPTIVE_MINIMUM_SUPPORT_NS;
use crate::snow_stage3_v11_attachment::{
    AcceptedPublicationCaptureAuditV1, AcceptedPublicationCaptureRegimeV1,
    AdaptiveControllerTestPolicyV1, Stage3AdaptiveStepDecisionV1,
    begin_adaptive_comparison_test_audit, begin_adaptive_controller_test_audit,
    begin_adaptive_performance_test_audit, begin_covered_fixed_point_iteration_audit_v1,
    begin_terminal_batch_production_audit, take_accepted_publication_capture_audit,
    take_adaptive_comparison_test_audit, take_adaptive_controller_test_audit,
    take_adaptive_performance_test_audit, take_covered_fixed_point_iteration_audit_v1,
    take_terminal_batch_production_audit,
};

const MINIMUM_STEP_NS: u128 = STAGE3_V11_ADAPTIVE_MINIMUM_SUPPORT_NS;

#[test]
fn adaptive_comparison_audit_excludes_lineage_but_retains_receipt_ordering() {
    use crate::v9_real_consumer_shadow::AdaptiveDiscreteSurfaceKindV1::{
        ReceiptLineage, ReceiptOrdering,
    };

    assert!(
        !crate::snow_stage3_v11_attachment::adaptive_comparison_audit_includes_discrete_surface_kind_v1(
            ReceiptLineage,
        ),
        "transaction-local receipt lineage is not a cross-path physical mismatch",
    );
    assert!(
        crate::snow_stage3_v11_attachment::adaptive_comparison_audit_includes_discrete_surface_kind_v1(
            ReceiptOrdering,
        ),
        "exact receipt ordering remains observable in cross-path audit telemetry",
    );
}

std::thread_local! {
    static SHORT_SUPPORT_REQUESTED: std::cell::Cell<bool> = const { std::cell::Cell::new(false) };
    static SHORT_SUPPORT_DURATION_NS: std::cell::Cell<u128> = const { std::cell::Cell::new(60_000_000_000) };
    static EQUILIBRIUM_FIXTURE_REQUESTED: std::cell::Cell<bool> = const { std::cell::Cell::new(false) };
    static FINAL_PHYSICAL_COMPARISON: std::cell::RefCell<Option<crate::v9_real_consumer_shadow::AdaptiveCompleteOwnerComparisonV1>> = const { std::cell::RefCell::new(None) };
}

pub(super) fn short_support_requested() -> bool {
    SHORT_SUPPORT_REQUESTED.with(std::cell::Cell::get)
}

pub(super) fn short_support_duration_ns() -> u128 {
    SHORT_SUPPORT_DURATION_NS.with(std::cell::Cell::get)
}

pub(super) fn equilibrium_fixture_requested() -> bool {
    EQUILIBRIUM_FIXTURE_REQUESTED.with(std::cell::Cell::get)
}

pub(super) fn record_final_physical_comparison(
    comparison: crate::v9_real_consumer_shadow::AdaptiveCompleteOwnerComparisonV1,
) {
    FINAL_PHYSICAL_COMPARISON.with(|value| *value.borrow_mut() = Some(comparison));
}

fn take_final_physical_comparison()
-> crate::v9_real_consumer_shadow::AdaptiveCompleteOwnerComparisonV1 {
    FINAL_PHYSICAL_COMPARISON
        .with(|value| value.borrow_mut().take())
        .expect("final complete-owner physical comparison")
}

fn run_complete_owner(
    runtime_swe_m: f64,
    hard_boundary_ns: Option<u128>,
    second_lane_swe_m: Option<f64>,
    terminal_event: bool,
    policy: AdaptiveControllerTestPolicyV1,
) -> (
    crate::snow_stage3_v11_attachment::Stage3AdaptiveSupportReceiptV1,
    Vec<openwepp_coupled_time::TimeSupport>,
    Vec<crate::snow_stage3_v11_attachment::TerminalBatchProductionAuditV2>,
    Vec<crate::snow_stage3_v11_attachment::AcceptedPublicationCaptureAuditV1>,
    crate::v9_real_consumer_shadow::AdaptiveCompleteOwnerComparisonV1,
    Vec<crate::snow_stage3_v11_attachment::AdaptivePerformanceSpanV1>,
    Vec<crate::snow_stage3_v11_attachment::AdaptiveComparisonAuditV1>,
) {
    std::thread::Builder::new()
        .name("adaptive-production-path-coverage".to_owned())
        .stack_size(64 * 1024 * 1024)
        .spawn(move || {
            run_complete_owner_on_large_stack(
                runtime_swe_m,
                hard_boundary_ns,
                second_lane_swe_m,
                terminal_event,
                policy,
                true,
            )
        })
        .expect("spawn adaptive production coverage")
        .join()
        .expect("join adaptive production coverage")
}

fn run_equilibrium_complete_owner() -> (
    crate::snow_stage3_v11_attachment::Stage3AdaptiveSupportReceiptV1,
    Vec<openwepp_coupled_time::TimeSupport>,
    Vec<crate::snow_stage3_v11_attachment::TerminalBatchProductionAuditV2>,
    Vec<crate::snow_stage3_v11_attachment::AcceptedPublicationCaptureAuditV1>,
    crate::v9_real_consumer_shadow::AdaptiveCompleteOwnerComparisonV1,
    Vec<crate::snow_stage3_v11_attachment::AdaptivePerformanceSpanV1>,
    Vec<crate::snow_stage3_v11_attachment::AdaptiveComparisonAuditV1>,
) {
    run_equilibrium_complete_owner_with_policy(AdaptiveControllerTestPolicyV1::default())
}

fn run_equilibrium_complete_owner_with_policy(
    policy: AdaptiveControllerTestPolicyV1,
) -> (
    crate::snow_stage3_v11_attachment::Stage3AdaptiveSupportReceiptV1,
    Vec<openwepp_coupled_time::TimeSupport>,
    Vec<crate::snow_stage3_v11_attachment::TerminalBatchProductionAuditV2>,
    Vec<crate::snow_stage3_v11_attachment::AcceptedPublicationCaptureAuditV1>,
    crate::v9_real_consumer_shadow::AdaptiveCompleteOwnerComparisonV1,
    Vec<crate::snow_stage3_v11_attachment::AdaptivePerformanceSpanV1>,
    Vec<crate::snow_stage3_v11_attachment::AdaptiveComparisonAuditV1>,
) {
    run_equilibrium_complete_owner_for_duration(60_000_000_000, policy)
}

type AdaptiveProductionOutcomeV1 = (
    crate::snow_stage3_v11_attachment::Stage3AdaptiveSupportReceiptV1,
    Vec<openwepp_coupled_time::TimeSupport>,
    Vec<crate::snow_stage3_v11_attachment::TerminalBatchProductionAuditV2>,
    Vec<crate::snow_stage3_v11_attachment::AcceptedPublicationCaptureAuditV1>,
    crate::v9_real_consumer_shadow::AdaptiveCompleteOwnerComparisonV1,
    Vec<crate::snow_stage3_v11_attachment::AdaptivePerformanceSpanV1>,
    Vec<crate::snow_stage3_v11_attachment::AdaptiveComparisonAuditV1>,
);

fn run_equilibrium_complete_owner_for_duration(
    duration_ns: u128,
    policy: AdaptiveControllerTestPolicyV1,
) -> (
    crate::snow_stage3_v11_attachment::Stage3AdaptiveSupportReceiptV1,
    Vec<openwepp_coupled_time::TimeSupport>,
    Vec<crate::snow_stage3_v11_attachment::TerminalBatchProductionAuditV2>,
    Vec<crate::snow_stage3_v11_attachment::AcceptedPublicationCaptureAuditV1>,
    crate::v9_real_consumer_shadow::AdaptiveCompleteOwnerComparisonV1,
    Vec<crate::snow_stage3_v11_attachment::AdaptivePerformanceSpanV1>,
    Vec<crate::snow_stage3_v11_attachment::AdaptiveComparisonAuditV1>,
) {
    run_equilibrium_complete_owner_for_duration_with_reuse_mode(duration_ns, policy, false, false)
}

fn run_equilibrium_with_provisional_physical_audit(
    force_full_provisional_envelope: bool,
) -> (
    AdaptiveProductionOutcomeV1,
    Vec<crate::v9_real_consumer_shadow::CoveredProvisionalPhysicalAuditV1>,
) {
    std::thread::Builder::new()
        .name("adaptive-provisional-physical-audit".to_owned())
        .stack_size(64 * 1024 * 1024)
        .spawn(move || {
            EQUILIBRIUM_FIXTURE_REQUESTED.with(|value| value.set(true));
            SHORT_SUPPORT_DURATION_NS.with(|value| value.set(60_000_000_000));
            let _audit =
                crate::v9_real_consumer_shadow::begin_covered_provisional_physical_audit_v1();
            let _full = force_full_provisional_envelope.then(
                crate::v9_real_consumer_shadow::force_covered_full_provisional_envelope_for_test,
            );
            let outcome = run_complete_owner_on_large_stack(
                0.08,
                None,
                None,
                false,
                AdaptiveControllerTestPolicyV1::default(),
                true,
            );
            let audit =
                crate::v9_real_consumer_shadow::take_covered_provisional_physical_audit_v1();
            (outcome, audit)
        })
        .expect("spawn provisional physical audit")
        .join()
        .expect("join provisional physical audit")
}

fn run_equilibrium_complete_owner_for_duration_with_reuse_mode(
    duration_ns: u128,
    policy: AdaptiveControllerTestPolicyV1,
    force_double_physical_evaluation: bool,
    force_full_provisional_envelope: bool,
) -> (
    crate::snow_stage3_v11_attachment::Stage3AdaptiveSupportReceiptV1,
    Vec<openwepp_coupled_time::TimeSupport>,
    Vec<crate::snow_stage3_v11_attachment::TerminalBatchProductionAuditV2>,
    Vec<crate::snow_stage3_v11_attachment::AcceptedPublicationCaptureAuditV1>,
    crate::v9_real_consumer_shadow::AdaptiveCompleteOwnerComparisonV1,
    Vec<crate::snow_stage3_v11_attachment::AdaptivePerformanceSpanV1>,
    Vec<crate::snow_stage3_v11_attachment::AdaptiveComparisonAuditV1>,
) {
    std::thread::Builder::new()
        .name("adaptive-equilibrium-production-path".to_owned())
        .stack_size(64 * 1024 * 1024)
        .spawn(move || {
            EQUILIBRIUM_FIXTURE_REQUESTED.with(|value| value.set(true));
            SHORT_SUPPORT_DURATION_NS.with(|value| value.set(duration_ns));
            crate::snow_stage3_v11_attachment::force_covered_physical_double_evaluation_for_test(
                force_double_physical_evaluation,
            );
            let _full_provisional = force_full_provisional_envelope.then(
                crate::v9_real_consumer_shadow::force_covered_full_provisional_envelope_for_test,
            );
            run_complete_owner_on_large_stack(0.08, None, None, false, policy, true)
        })
        .expect("spawn adaptive equilibrium production coverage")
        .join()
        .expect("join adaptive equilibrium production coverage")
}

fn run_transient_complete_owner_for_duration(
    duration_ns: u128,
    policy: AdaptiveControllerTestPolicyV1,
) -> (
    crate::snow_stage3_v11_attachment::Stage3AdaptiveSupportReceiptV1,
    Vec<openwepp_coupled_time::TimeSupport>,
    Vec<crate::snow_stage3_v11_attachment::TerminalBatchProductionAuditV2>,
    Vec<crate::snow_stage3_v11_attachment::AcceptedPublicationCaptureAuditV1>,
    crate::v9_real_consumer_shadow::AdaptiveCompleteOwnerComparisonV1,
    Vec<crate::snow_stage3_v11_attachment::AdaptivePerformanceSpanV1>,
    Vec<crate::snow_stage3_v11_attachment::AdaptiveComparisonAuditV1>,
) {
    run_transient_complete_owner_for_duration_with_memo_mode(duration_ns, policy, false)
}

fn run_transient_complete_owner_for_duration_with_memo_mode(
    duration_ns: u128,
    policy: AdaptiveControllerTestPolicyV1,
    force_memo_disabled: bool,
) -> (
    crate::snow_stage3_v11_attachment::Stage3AdaptiveSupportReceiptV1,
    Vec<openwepp_coupled_time::TimeSupport>,
    Vec<crate::snow_stage3_v11_attachment::TerminalBatchProductionAuditV2>,
    Vec<crate::snow_stage3_v11_attachment::AcceptedPublicationCaptureAuditV1>,
    crate::v9_real_consumer_shadow::AdaptiveCompleteOwnerComparisonV1,
    Vec<crate::snow_stage3_v11_attachment::AdaptivePerformanceSpanV1>,
    Vec<crate::snow_stage3_v11_attachment::AdaptiveComparisonAuditV1>,
) {
    std::thread::Builder::new()
        .name("adaptive-transient-production-path".to_owned())
        .stack_size(64 * 1024 * 1024)
        .spawn(move || {
            SHORT_SUPPORT_DURATION_NS.with(|value| value.set(duration_ns));
            let _memo_disabled = force_memo_disabled.then(
                crate::snow_stage3_v11_attachment::
                    force_adaptive_covered_child_memo_disabled_for_test,
            );
            run_complete_owner_on_large_stack(0.08, None, None, false, policy, true)
        })
        .expect("spawn adaptive transient production coverage")
        .join()
        .expect("join adaptive transient production coverage")
}

fn run_full_duration_complete_owner_qualification()
-> crate::snow_stage3_v11_attachment::Stage3AdaptiveSupportReceiptV1 {
    std::thread::Builder::new()
        .name("adaptive-full-parent-qualification".to_owned())
        .stack_size(64 * 1024 * 1024)
        .spawn(|| {
            EQUILIBRIUM_FIXTURE_REQUESTED.with(|value| value.set(true));
            run_complete_owner_on_large_stack(
                0.08,
                None,
                None,
                false,
                AdaptiveControllerTestPolicyV1::default(),
                false,
            )
            .0
        })
        .expect("spawn full-parent adaptive qualification")
        .join()
        .expect("join full-parent adaptive qualification")
}

fn run_complete_owner_on_large_stack(
    runtime_swe_m: f64,
    hard_boundary_ns: Option<u128>,
    second_lane_swe_m: Option<f64>,
    terminal_event: bool,
    policy: AdaptiveControllerTestPolicyV1,
    short_support: bool,
) -> (
    crate::snow_stage3_v11_attachment::Stage3AdaptiveSupportReceiptV1,
    Vec<openwepp_coupled_time::TimeSupport>,
    Vec<crate::snow_stage3_v11_attachment::TerminalBatchProductionAuditV2>,
    Vec<crate::snow_stage3_v11_attachment::AcceptedPublicationCaptureAuditV1>,
    crate::v9_real_consumer_shadow::AdaptiveCompleteOwnerComparisonV1,
    Vec<crate::snow_stage3_v11_attachment::AdaptivePerformanceSpanV1>,
    Vec<crate::snow_stage3_v11_attachment::AdaptiveComparisonAuditV1>,
) {
    SHORT_SUPPORT_REQUESTED.with(|value| value.set(short_support));
    let short_duration_ns = short_support_duration_ns();
    let _short_wb14_permit = short_support.then(|| {
        crate::direct_runtime::permit_short_wb14_parent_support_for_test(short_duration_ns)
    });
    begin_adaptive_controller_test_audit(policy);
    begin_adaptive_performance_test_audit();
    begin_adaptive_comparison_test_audit();
    crate::v9_real_consumer_shadow::v11_covered::begin_covered_carrier_support_audit();
    crate::v9_real_consumer_shadow::v11_covered::begin_covered_carrier_live_consumption_audit();
    begin_terminal_batch_production_audit();
    super::exercise_complete_wb14_cadence(
        runtime_swe_m,
        8.0,
        false,
        hard_boundary_ns,
        false,
        second_lane_swe_m,
        terminal_event,
        false,
        false,
        true,
    );
    let receipts = take_adaptive_controller_test_audit();
    let carrier_supports =
        crate::v9_real_consumer_shadow::v11_covered::take_covered_carrier_support_audit();
    let batches = take_terminal_batch_production_audit();
    let publication_captures = take_accepted_publication_capture_audit();
    let performance_spans = take_adaptive_performance_test_audit();
    let comparison_audits = take_adaptive_comparison_test_audit();
    let live_carrier_audit =
        crate::v9_real_consumer_shadow::v11_covered::take_covered_carrier_live_consumption_audit();
    if equilibrium_fixture_requested() {
        let expected_q_bits =
            super::equilibrium_stage3_reference_specific_humidity(101_325.0).to_bits();
        assert!(
            !live_carrier_audit.carrier_rows.is_empty(),
            "equilibrium production path consumed no live carrier rows",
        );
        for row in &live_carrier_audit.carrier_rows {
            assert_ne!(row.forcing_sha256, openwepp_coupled_time::Digest32::zero());
            assert_eq!(row.reference_specific_humidity_bits, expected_q_bits);
            assert_eq!(row.snow_specific_humidity_bits, expected_q_bits);
            assert_eq!(row.shared_specific_humidity_bits, expected_q_bits);
            assert_eq!(
                row.snow_vapor_into_surface_bits,
                0.0_f64.to_bits(),
                "equilibrium live carrier emitted a snow vapor credit",
            );
        }
        assert!(
            !live_carrier_audit.open_snow_rows.is_empty(),
            "equilibrium production path consumed no open-snow rows",
        );
        for row in &live_carrier_audit.open_snow_rows {
            assert_ne!(row.forcing_sha256, openwepp_coupled_time::Digest32::zero());
            assert_eq!(row.reference_specific_humidity_bits, expected_q_bits);
            assert_eq!(row.snow_specific_humidity_bits, expected_q_bits);
            assert_eq!(
                row.vapor_outward_bits,
                0.0_f64.to_bits(),
                "equilibrium open-snow destination emitted vapor",
            );
        }
        assert!(
            live_carrier_audit.condensation_credits.is_empty(),
            "equilibrium carrier produced condensation credits: {:?}",
            live_carrier_audit.condensation_credits,
        );
    }
    let receipt = receipts
        .last()
        .cloned()
        .expect("complete-owner adaptive production receipt");
    (
        receipt,
        carrier_supports,
        batches,
        publication_captures,
        take_final_physical_comparison(),
        performance_spans,
        comparison_audits,
    )
}

#[test]
fn equilibrium_carrier_humidity_is_exact_stage3_saturation_authority() {
    let pressure_pa = 101_325.0;
    let carrier_q = super::equilibrium_stage3_reference_specific_humidity(pressure_pa);
    let reconstructed_pressure_pa = carrier_q * pressure_pa / (0.622 + 0.378 * carrier_q);
    let stage3_pressure_pa =
        openwepp_meteorology::surface_energy::saturation_vapor_pressure_snobal_pa(
            openwepp_unit_boundary::TemperatureCelsius::try_new(0.0)
                .expect("Stage-3 equilibrium temperature"),
        )
        .expect("Stage-3 equilibrium saturation pressure")
        .as_pascals();
    assert_eq!(
        reconstructed_pressure_pa.to_bits(),
        stage3_pressure_pa.to_bits(),
        "carrier q must invert to the exact Stage-3 SNOBAL saturation pressure",
    );
    assert_ne!(
        carrier_q.to_bits(),
        super::equilibrium_canopy_specific_humidity(pressure_pa).to_bits(),
        "substituting the vegetation Table-5.2 dark-gas saturation law is rejected",
    );
}

#[test]
fn stable_minimum_production_support_accepts_one_direct_trial() {
    let (
        receipt,
        carrier_supports,
        batches,
        publication_captures,
        _,
        performance_spans,
        comparison_audits,
    ) = run_equilibrium_complete_owner();
    assert!(comparison_audits.is_empty());
    assert!(
        performance_spans
            .iter()
            .any(|span| span.label == "complete_owner_direct_trial")
    );
    assert!(
        !performance_spans
            .iter()
            .any(|span| span.label == "complete_owner_composed_trial")
    );
    receipt
        .validate()
        .expect("stable adaptive receipt validates");
    assert_eq!(receipt.parent_requests.len(), 1);
    assert_eq!(receipt.direct_trials.len(), 1);
    assert!(receipt.split_child_trials.is_empty());
    assert_eq!(receipt.comparisons.len(), 1);
    assert_eq!(
        receipt.comparisons[0].decision,
        Stage3AdaptiveStepDecisionV1::FloorAccepted
    );
    assert_eq!(receipt.accepted_microsteps.len(), 1);
    let diagnostics = receipt
        .transient_diagnostics()
        .expect("transient floor diagnostics");
    assert_eq!(diagnostics.direct_trial_count, 1);
    assert_eq!(diagnostics.split_child_trial_count, 0);
    assert_eq!(diagnostics.accepted_microstep_count, 1);
    assert_eq!(diagnostics.rejected_candidate_count, 0);
    assert_eq!(publication_captures.len(), 2);
    let captures_by_support = publication_captures.iter().fold(
        std::collections::BTreeMap::<(u128, u128), Vec<_>>::new(),
        |mut grouped, capture| {
            grouped
                .entry((
                    capture.support.start_ns().get(),
                    capture.support.end_ns().get(),
                ))
                .or_default()
                .push(capture.regime);
            grouped
        },
    );
    assert_eq!(captures_by_support.len(), 1);
    assert!(captures_by_support.iter().all(|(support, regimes)| {
        support.1 > support.0
            && regimes
                .iter()
                .copied()
                .collect::<std::collections::BTreeSet<_>>()
                == std::collections::BTreeSet::from([
                    AcceptedPublicationCaptureRegimeV1::ProvisionalClock,
                    AcceptedPublicationCaptureRegimeV1::FinalClock,
                ])
            && regimes.len() == 2
    }));
    let aggregate_capture_elapsed = publication_captures
        .iter()
        .map(|capture| capture.total_elapsed)
        .sum::<std::time::Duration>();
    assert!(
        aggregate_capture_elapsed < std::time::Duration::from_secs(10),
        "two compact provisional/final publication captures took {aggregate_capture_elapsed:?}: {publication_captures:?}",
    );
    assert_eq!(
        receipt.parent_requests[0].proposed_step_quanta,
        receipt.parent_support.duration_ns() / MINIMUM_STEP_NS
    );
    let parent_start = receipt.parent_support.start_ns().get();
    let parent_end = receipt.parent_support.end_ns().get();
    assert_eq!(
        carrier_supports
            .iter()
            .map(|support| (support.start_ns().get(), support.end_ns().get()))
            .collect::<Vec<_>>(),
        vec![(parent_start, parent_end)],
    );
    assert!(batches.is_empty());
}

#[test]
fn covered_fixed_point_exhaustion_at_96_is_fail_closed() {
    let (panicked, audit) = std::thread::Builder::new()
        .name("covered-fixed-point-exhaustion".to_owned())
        .stack_size(64 * 1024 * 1024)
        .spawn(|| {
            let _force = crate::v9_real_consumer_shadow::v11_covered::
                force_covered_fixed_point_exhaustion_for_test();
            let _audit_guard = begin_covered_fixed_point_iteration_audit_v1();
            EQUILIBRIUM_FIXTURE_REQUESTED.with(|value| value.set(true));
            SHORT_SUPPORT_DURATION_NS.with(|value| value.set(MINIMUM_STEP_NS));
            let panicked = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
                run_complete_owner_on_large_stack(
                    0.08,
                    None,
                    None,
                    false,
                    AdaptiveControllerTestPolicyV1::default(),
                    true,
                )
            }))
            .is_err();
            (panicked, take_covered_fixed_point_iteration_audit_v1())
        })
        .expect("spawn fixed-point exhaustion proof")
        .join()
        .expect("join fixed-point exhaustion proof");
    assert!(panicked, "96 nonconvergent iterations must fail closed");
    assert!(audit.iter().any(|entry| {
        !entry.converged
            && entry.completed_iterations == 96
            && entry.support.duration_ns() == MINIMUM_STEP_NS
    }));
}

#[test]
fn receipt_reseal_one_ulp_density_mapping_reenters_and_converges_before_install() {
    let (outcome, audit) = std::thread::Builder::new()
        .name("covered-receipt-reseal-one-ulp".to_owned())
        .stack_size(64 * 1024 * 1024)
        .spawn(|| {
            EQUILIBRIUM_FIXTURE_REQUESTED.with(|value| value.set(true));
            SHORT_SUPPORT_DURATION_NS.with(|value| value.set(MINIMUM_STEP_NS));
            let _reconstruction = crate::v9_real_consumer_shadow::v11_covered::
                force_covered_receipt_reseal_density_ulp_once_for_test();
            let _audit_guard = begin_covered_fixed_point_iteration_audit_v1();
            let outcome = run_complete_owner_on_large_stack(
                0.08,
                None,
                None,
                false,
                AdaptiveControllerTestPolicyV1::default(),
                true,
            );
            (outcome, take_covered_fixed_point_iteration_audit_v1())
        })
        .expect("spawn receipt-reseal convergence proof")
        .join()
        .expect("join receipt-reseal convergence proof");
    outcome
        .0
        .validate()
        .expect("receipt-reseal accepted adaptive receipt");
    assert!(audit.iter().any(|entry| {
        entry.converged
            && entry.completed_iterations > 1
            && entry.completed_iterations <= 64
            && entry.support.duration_ns() == MINIMUM_STEP_NS
    }));
}

#[test]
fn noncontracting_receipt_reseal_density_mapping_exhausts_96_fail_closed() {
    let (panicked, audit) = std::thread::Builder::new()
        .name("covered-receipt-reseal-noncontraction".to_owned())
        .stack_size(64 * 1024 * 1024)
        .spawn(|| {
            EQUILIBRIUM_FIXTURE_REQUESTED.with(|value| value.set(true));
            SHORT_SUPPORT_DURATION_NS.with(|value| value.set(MINIMUM_STEP_NS));
            let _reconstruction = crate::v9_real_consumer_shadow::v11_covered::
                force_covered_receipt_reseal_density_noncontraction_for_test();
            let _audit_guard = begin_covered_fixed_point_iteration_audit_v1();
            let panicked = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
                run_complete_owner_on_large_stack(
                    0.08,
                    None,
                    None,
                    false,
                    AdaptiveControllerTestPolicyV1::default(),
                    true,
                )
            }))
            .is_err();
            (panicked, take_covered_fixed_point_iteration_audit_v1())
        })
        .expect("spawn receipt-reseal noncontraction proof")
        .join()
        .expect("join receipt-reseal noncontraction proof");
    assert!(panicked, "noncontracting receipt reseal must fail closed");
    assert!(audit.iter().any(|entry| {
        !entry.converged
            && entry.completed_iterations == 96
            && entry.support.duration_ns() == MINIMUM_STEP_NS
    }));
}

#[test]
fn ordinary_physical_reuse_is_byte_identical_to_forced_double_evaluation() {
    let reuse = run_equilibrium_complete_owner_for_duration_with_reuse_mode(
        60_000_000_000,
        AdaptiveControllerTestPolicyV1::default(),
        false,
        false,
    );
    let forced = {
        let _deep_history =
            crate::v9_real_consumer_shadow::force_deep_clone_accepted_publication_history_v1();
        let _full_publication_validation =
            crate::v9_real_consumer_shadow::force_full_scan_accepted_publication_history_v1();
        let _deep_wb14 = crate::direct_runtime::force_deep_clone_direct_wb14_receipt_history_v1();
        run_equilibrium_complete_owner_for_duration_with_reuse_mode(
            60_000_000_000,
            AdaptiveControllerTestPolicyV1::default(),
            true,
            false,
        )
    };
    assert_eq!(
        serde_json::to_vec(&reuse.0).expect("reuse adaptive receipt bytes"),
        serde_json::to_vec(&forced.0).expect("forced-double adaptive receipt bytes"),
        "reuse changed canonical adaptive receipt/controller bytes",
    );
    assert_eq!(reuse.1, forced.1, "reuse changed physical support calls");
    assert_eq!(reuse.2, forced.2, "reuse changed terminal batch outcomes");
    assert_eq!(reuse.4, forced.4, "reuse changed complete-owner bytes");
    assert_eq!(reuse.6, forced.6, "reuse changed comparison outcomes");
    let publication_topology = |captures: &[AcceptedPublicationCaptureAuditV1]| {
        captures
            .iter()
            .map(|capture| (capture.support, capture.regime))
            .collect::<Vec<_>>()
    };
    assert_eq!(
        publication_topology(&reuse.3),
        publication_topology(&forced.3),
        "reuse changed accepted-publication support/regime topology",
    );
    let evaluated_supports =
        |spans: &[crate::snow_stage3_v11_attachment::AdaptivePerformanceSpanV1]| {
            spans
                .iter()
                .map(|span| (span.label, span.support_duration_ns))
                .collect::<Vec<_>>()
        };
    assert_eq!(
        evaluated_supports(&reuse.5),
        evaluated_supports(&forced.5),
        "reuse changed controller evaluation topology",
    );
}

#[test]
fn covered_physical_only_provisional_is_canonical_to_forced_full_envelope() {
    let (physical_only, physical_audit) = run_equilibrium_with_provisional_physical_audit(false);
    let (forced_full, forced_audit) = run_equilibrium_with_provisional_physical_audit(true);
    assert!(
        !physical_audit.is_empty(),
        "missing physical-only iteration audit"
    );
    assert_eq!(
        physical_audit, forced_audit,
        "physical-only changed exact LSE/soil/precipitation iteration evidence",
    );
    let mut omitted_precipitation = physical_audit.clone();
    omitted_precipitation[0].precipitation_sets.pop_first();
    assert_ne!(
        omitted_precipitation, forced_audit,
        "precipitation omission"
    );
    let mut substituted_boundary = physical_audit.clone();
    let boundary = substituted_boundary[0]
        .corrected_boundaries
        .values_mut()
        .next()
        .expect("covered boundary evidence");
    boundary.snow_temperature_k = f64::from_bits(boundary.snow_temperature_k.to_bits() ^ 1);
    assert_ne!(substituted_boundary, forced_audit, "boundary substitution");
    let mut omitted_soil = physical_audit.clone();
    omitted_soil[0].soil_candidates.pop();
    assert_ne!(omitted_soil, forced_audit, "soil omission");
    let mut substituted_lse = physical_audit.clone();
    let lse = substituted_lse[0]
        .lse_states
        .values_mut()
        .next()
        .expect("covered LSE evidence");
    lse.canopy_air_temperature_k = f64::from_bits(lse.canopy_air_temperature_k.to_bits() ^ 1);
    assert_ne!(substituted_lse, forced_audit, "LSE substitution");
    assert_eq!(
        serde_json::to_vec(&physical_only.0).expect("physical-only adaptive receipt bytes"),
        serde_json::to_vec(&forced_full.0).expect("full-envelope adaptive receipt bytes"),
        "physical-only provisional evaluation changed controller/receipt authority",
    );
    assert_eq!(physical_only.1, forced_full.1, "physical support calls");
    assert_eq!(physical_only.2, forced_full.2, "terminal batch outcomes");
    let publication_topology = |captures: &[AcceptedPublicationCaptureAuditV1]| {
        captures
            .iter()
            .map(|capture| (capture.support, capture.regime))
            .collect::<Vec<_>>()
    };
    assert_eq!(
        publication_topology(&physical_only.3),
        publication_topology(&forced_full.3),
        "accepted publication topology",
    );
    assert_eq!(physical_only.4, forced_full.4, "complete owner bytes");
    assert_eq!(physical_only.6, forced_full.6, "comparison outputs");
}

#[test]
#[ignore = "optimized 1,800-second production qualification; run explicitly after per-trial carrier cost is bounded"]
fn full_1800_second_production_constructor_large_step_qualification() {
    let receipt = run_full_duration_complete_owner_qualification();
    receipt
        .validate()
        .expect("full-parent adaptive receipt validates");
    assert_eq!(receipt.parent_support.duration_ns(), 1_800_000_000_000);
    let diagnostics = receipt
        .transient_diagnostics()
        .expect("transient full-parent diagnostics");
    assert_eq!(diagnostics.direct_trial_count, 1);
    assert_eq!(diagnostics.split_child_trial_count, 2);
    assert_eq!(diagnostics.accepted_microstep_count, 1);
    assert_eq!(diagnostics.rejected_candidate_count, 0);
}

#[test]
fn odd_quanta_tile_exactly_and_never_call_carrier_below_floor() {
    let (receipt, carrier_supports, _, _, _, _, _) = run_equilibrium_complete_owner_for_duration(
        7 * MINIMUM_STEP_NS,
        AdaptiveControllerTestPolicyV1 {
            initial_proposal_quanta: Some(3),
            growth_factor: 3,
        },
    );
    receipt
        .validate()
        .expect("odd-quanta adaptive receipt validates");
    assert_eq!(receipt.parent_requests[0].proposed_step_quanta, 3);
    assert_eq!(receipt.parent_support.duration_ns(), 7 * MINIMUM_STEP_NS);
    assert!(receipt.parent_requests.iter().all(|request| {
        request.context.step_support.duration_ns() >= MINIMUM_STEP_NS
            && request.context.step_support.duration_ns() % MINIMUM_STEP_NS == 0
    }));
    assert!(carrier_supports.iter().all(|support| {
        support.duration_ns() >= MINIMUM_STEP_NS && support.duration_ns() % MINIMUM_STEP_NS == 0
    }));
    let accepted_ns = receipt
        .accepted_microsteps
        .iter()
        .map(|accepted| accepted.context.step_support.duration_ns())
        .sum::<u128>();
    assert_eq!(accepted_ns, receipt.parent_support.duration_ns());
}

#[test]
fn cold_transient_rejects_coarse_trial_then_accepts_at_floor_on_three_quanta_parent() {
    let (receipt, carrier_supports, _, _, _, _, comparison_audits) =
        run_transient_complete_owner_for_duration(
            3 * MINIMUM_STEP_NS,
            AdaptiveControllerTestPolicyV1 {
                initial_proposal_quanta: Some(3),
                growth_factor: 2,
            },
        );
    receipt
        .validate()
        .expect("bounded cold-transient adaptive receipt validates");
    assert_eq!(receipt.parent_support.duration_ns(), 3 * MINIMUM_STEP_NS);
    assert!(
        receipt
            .transient_diagnostics()
            .expect("transient cold diagnostics")
            .rejected_candidate_count
            >= 1
    );
    assert!(
        receipt.accepted_microsteps.iter().any(|accepted| accepted
            .context
            .step_support
            .duration_ns()
            == MINIMUM_STEP_NS)
    );
    assert!(
        carrier_supports
            .iter()
            .all(|support| support.duration_ns() >= MINIMUM_STEP_NS)
    );
    assert!(
        comparison_audits
            .iter()
            .any(|audit| audit.maximum_scaled_error > 1.0)
    );
}

#[test]
fn reject_child_memo_is_canonical_byte_identical_to_forced_recomputation() {
    let policy = AdaptiveControllerTestPolicyV1 {
        initial_proposal_quanta: Some(3),
        growth_factor: 2,
    };
    let memoized = run_transient_complete_owner_for_duration_with_memo_mode(
        3 * MINIMUM_STEP_NS,
        policy,
        false,
    );
    let forced =
        run_transient_complete_owner_for_duration_with_memo_mode(3 * MINIMUM_STEP_NS, policy, true);
    assert_eq!(
        serde_json::to_vec(&memoized.0).expect("memoized adaptive receipt bytes"),
        serde_json::to_vec(&forced.0).expect("forced adaptive receipt bytes"),
        "child memo changed adaptive controller or receipt authority",
    );
    assert_eq!(memoized.2, forced.2, "child memo changed terminal batches");
    assert_eq!(memoized.4, forced.4, "child memo changed ending owners");
    assert_eq!(memoized.6, forced.6, "child memo changed comparisons");
    let memoized_physical_evaluations = memoized
        .5
        .iter()
        .filter(|span| span.label == "covered_complete_owner_subslab")
        .count();
    let forced_physical_evaluations = forced
        .5
        .iter()
        .filter(|span| span.label == "covered_complete_owner_subslab")
        .count();
    assert!(
        memoized_physical_evaluations < forced_physical_evaluations,
        "fixture did not consume a composed child: memoized={memoized_physical_evaluations} forced={forced_physical_evaluations}",
    );
}

#[test]
fn initial_proposal_and_growth_policy_preserve_final_physical_owner_state() {
    let (default_receipt, _, _, _, default_physical, _, _) =
        run_equilibrium_complete_owner_for_duration(
            7 * MINIMUM_STEP_NS,
            AdaptiveControllerTestPolicyV1::default(),
        );
    let (alternate_receipt, _, _, _, alternate_physical, _, _) =
        run_equilibrium_complete_owner_for_duration(
            7 * MINIMUM_STEP_NS,
            AdaptiveControllerTestPolicyV1 {
                initial_proposal_quanta: Some(5),
                growth_factor: 3,
            },
        );
    assert_ne!(
        default_receipt.parent_request_set_sha256, alternate_receipt.parent_request_set_sha256,
        "the test must exercise distinct controller histories"
    );
    let (maximum_scaled_error, _) = default_physical
        .scaled_error(&alternate_physical)
        .expect("final physical scalar comparison");
    let maximum_physical_scalar = default_physical
        .scalars
        .iter()
        .zip(&alternate_physical.scalars)
        .map(|(left, right)| {
            let denominator = left.absolute_tolerance
                + left.relative_tolerance * left.value.abs().max(right.value.abs());
            (
                (right.value - left.value).abs() / denominator,
                left.owner_id.as_str(),
                left.path.as_str(),
                left.value,
                right.value,
            )
        })
        .max_by(|left, right| left.0.total_cmp(&right.0))
        .expect("nonempty complete-owner scalar vector");
    let controller_trace =
        |receipt: &crate::snow_stage3_v11_attachment::Stage3AdaptiveSupportReceiptV1| {
            receipt
                .parent_requests
                .iter()
                .zip(&receipt.comparisons)
                .map(|(request, comparison)| {
                    (
                        request.context.step_ordinal,
                        request.context.attempt_ordinal,
                        request.proposed_step_quanta,
                        comparison.decision,
                        comparison.maximum_scaled_error,
                    )
                })
                .collect::<Vec<_>>()
        };
    let default_trace = controller_trace(&default_receipt);
    let alternate_trace = controller_trace(&alternate_receipt);
    assert_ne!(default_trace, alternate_trace);
    eprintln!(
        "ADAPTIVE_POLICY_INVARIANCE default_trace={default_trace:?} alternate_trace={alternate_trace:?} final_max={:?}",
        maximum_physical_scalar,
    );
    assert!(
        maximum_scaled_error <= 1.0,
        "controller history changed final physical scalars: {maximum_scaled_error}"
    );
    let persistent_discrete =
        |comparison: &crate::v9_real_consumer_shadow::AdaptiveCompleteOwnerComparisonV1| {
            comparison
            .exact_discrete_surfaces
            .iter()
            .filter(|surface| {
                !matches!(
                    surface.kind,
                    crate::v9_real_consumer_shadow::AdaptiveDiscreteSurfaceKindV1::ReceiptOrdering
                        | crate::v9_real_consumer_shadow::AdaptiveDiscreteSurfaceKindV1::ReceiptLineage
                )
            })
            .cloned()
            .collect::<Vec<_>>()
        };
    assert_eq!(
        persistent_discrete(&default_physical),
        persistent_discrete(&alternate_physical),
        "controller history changed final physical discrete surfaces"
    );
}

#[test]
fn adaptive_trace_replay_is_exact_and_attempt_ordinals_are_contiguous() {
    let (first, _, _, _, first_physical, _, _) =
        run_equilibrium_complete_owner_with_policy(AdaptiveControllerTestPolicyV1::default());
    let (replay, _, _, _, replay_physical, _, _) =
        run_equilibrium_complete_owner_with_policy(AdaptiveControllerTestPolicyV1::default());
    assert_eq!(first, replay);
    assert_eq!(first_physical, replay_physical);
    assert_eq!(first.parent_requests[0].context.attempt_ordinal, 0);
    assert!(first.parent_requests.windows(2).all(|pair| {
        if pair[0].context.step_ordinal == pair[1].context.step_ordinal {
            pair[1].context.attempt_ordinal == pair[0].context.attempt_ordinal + 1
        } else {
            pair[1].context.step_ordinal == pair[0].context.step_ordinal + 1
                && pair[1].context.attempt_ordinal == 0
        }
    }));
    let diagnostics = first
        .transient_diagnostics()
        .expect("transient deterministic diagnostics");
    assert_eq!(
        diagnostics.direct_trial_count,
        diagnostics.accepted_microstep_count + diagnostics.rejected_candidate_count
    );
}

#[test]
fn real_two_active_lane_terminal_and_survivor_use_one_joint_batch_decision() {
    let (_receipt, carrier_supports, batches, _, _, _, _) = run_complete_owner(
        0.000_6,
        None,
        Some(0.005),
        true,
        AdaptiveControllerTestPolicyV1::default(),
    );
    assert!(
        !batches.is_empty(),
        "real terminal-plus-survivor batch path"
    );
    assert!(batches.iter().all(|batch| {
        batch.lane_ids == vec![1, 2]
            && batch.provider_call_count == 1
            && batch.join_call_count == 1
            && batch.beginning_joint_sha256 != batch.ending_joint_sha256
    }));
    assert!(batches.iter().any(|batch| {
        batch.ending_terminal_lanes == vec![1] && batch.ending_surviving_lanes == vec![2]
    }));
    assert!(
        carrier_supports
            .iter()
            .all(|support| support.duration_ns() >= MINIMUM_STEP_NS)
    );
}
