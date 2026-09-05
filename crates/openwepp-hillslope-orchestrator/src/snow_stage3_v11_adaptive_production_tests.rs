use crate::snow_stage3_v11_attachment::{
    begin_adaptive_comparison_test_audit, begin_adaptive_controller_test_audit,
    begin_adaptive_performance_test_audit, begin_terminal_batch_production_audit,
    take_accepted_publication_capture_audit, take_adaptive_comparison_test_audit,
    take_adaptive_controller_test_audit, take_adaptive_performance_test_audit,
    take_terminal_batch_production_audit, AcceptedPublicationCaptureAuditV1,
    AcceptedPublicationCaptureRegimeV1, AdaptiveControllerTestPolicyV1,
    Stage3AdaptiveStepDecisionV1,
};
use crate::STAGE3_V11_ADAPTIVE_MINIMUM_SUPPORT_NS;

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
    static CANONICAL_PARITY_CASE: std::cell::Cell<Option<crate::v9_real_consumer_shadow::v11_covered::CanonicalCoveredPhysicalParityCaseV1>> = const { std::cell::Cell::new(None) };
}

pub(super) fn canonical_parity_case_requested(
) -> Option<crate::v9_real_consumer_shadow::v11_covered::CanonicalCoveredPhysicalParityCaseV1> {
    CANONICAL_PARITY_CASE.with(std::cell::Cell::get)
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

fn take_final_physical_comparison(
) -> crate::v9_real_consumer_shadow::AdaptiveCompleteOwnerComparisonV1 {
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
) -> AdaptiveProductionOutcomeV1 {
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

fn run_equilibrium_complete_owner() -> AdaptiveProductionOutcomeV1 {
    run_equilibrium_complete_owner_with_policy(AdaptiveControllerTestPolicyV1::default())
}

fn run_equilibrium_complete_owner_with_policy(
    policy: AdaptiveControllerTestPolicyV1,
) -> AdaptiveProductionOutcomeV1 {
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
) -> AdaptiveProductionOutcomeV1 {
    run_equilibrium_complete_owner_for_duration_with_reuse_mode(duration_ns, policy, false, false)
}

fn run_equilibrium_with_provisional_physical_audit(
    force_full_provisional_envelope: bool,
    case: Option<crate::v9_real_consumer_shadow::v11_covered::CanonicalCoveredPhysicalParityCaseV1>,
) -> (
    AdaptiveProductionOutcomeV1,
    Vec<crate::v9_real_consumer_shadow::CoveredProvisionalPhysicalAuditV1>,
    crate::v9_real_consumer_shadow::CoveredNativePhysicalPathAuditV1,
    crate::v9_real_consumer_shadow::v11_covered::CanonicalCoveredMapRoleAuditV1,
) {
    std::thread::Builder::new()
        .name("adaptive-provisional-physical-audit".to_owned())
        .stack_size(64 * 1024 * 1024)
        .spawn(move || {
            use crate::v9_real_consumer_shadow::v11_covered::
                CanonicalCoveredPhysicalParityCaseV1 as Case;

            CANONICAL_PARITY_CASE.with(|slot| slot.set(case));
            EQUILIBRIUM_FIXTURE_REQUESTED.with(|value| {
                value.set(matches!(
                    case,
                    Some(
                        crate::v9_real_consumer_shadow::v11_covered::
                            CanonicalCoveredPhysicalParityCaseV1::Ordinary
                            | crate::v9_real_consumer_shadow::v11_covered::
                                CanonicalCoveredPhysicalParityCaseV1::NativeThawRefreeze
                    )
                ));
            });
            SHORT_SUPPORT_DURATION_NS.with(|value| value.set(60_000_000_000));
            let _audit =
                crate::v9_real_consumer_shadow::begin_covered_provisional_physical_audit_v1();
            let _native_audit =
                crate::v9_real_consumer_shadow::begin_covered_native_physical_path_audit_v1();
            let _map_audit = crate::v9_real_consumer_shadow::v11_covered::
                begin_canonical_covered_map_role_audit_v1();
            let _full = force_full_provisional_envelope.then(
                crate::v9_real_consumer_shadow::force_covered_full_provisional_envelope_for_test,
            );
            let (runtime_swe_m, second_lane_swe_m) = match case {
                Some(Case::NativeMixedPhase) => (0.02, None),
                Some(Case::NativeThawRefreeze | Case::NativeWetCanopy) => (0.005, None),
                Some(Case::NativeMultiOfe) => (0.08, Some(0.08)),
                Some(Case::Ordinary | Case::NativeFrozen) | None => (0.08, None),
            };
            let outcome = run_complete_owner_on_large_stack(
                runtime_swe_m,
                None,
                second_lane_swe_m,
                false,
                AdaptiveControllerTestPolicyV1::default(),
                true,
            );
            let audit =
                crate::v9_real_consumer_shadow::take_covered_provisional_physical_audit_v1();
            let native_audit =
                crate::v9_real_consumer_shadow::take_covered_native_physical_path_audit_v1();
            let map_audit = crate::v9_real_consumer_shadow::v11_covered::
                take_canonical_covered_map_role_audit_v1();
            (outcome, audit, native_audit, map_audit)
        })
        .expect("spawn provisional physical audit")
        .join()
        .expect("join provisional physical audit")
}

fn run_canonical_covered_physical_prefix_capture(
    force_full_provisional_envelope: bool,
    case: crate::v9_real_consumer_shadow::v11_covered::CanonicalCoveredPhysicalParityCaseV1,
) -> (
    Vec<crate::v9_real_consumer_shadow::CoveredProvisionalPhysicalAuditV1>,
    crate::v9_real_consumer_shadow::CoveredNativePhysicalPathAuditV1,
    crate::v9_real_consumer_shadow::v11_covered::CanonicalCoveredMapRoleAuditV1,
) {
    std::thread::Builder::new()
        .name("canonical-covered-physical-prefix-capture".to_owned())
        .stack_size(64 * 1024 * 1024)
        .spawn(move || {
            use crate::v9_real_consumer_shadow::v11_covered::{
                CanonicalCoveredErrorClassV1 as ErrorClass,
                CanonicalCoveredPhysicalParityCaseV1 as Case,
                CanonicalCoveredRejectionStageV1 as Stage,
            };

            CANONICAL_PARITY_CASE.with(|slot| slot.set(Some(case)));
            EQUILIBRIUM_FIXTURE_REQUESTED.with(|value| {
                value.set(matches!(case, Case::Ordinary | Case::NativeThawRefreeze));
            });
            SHORT_SUPPORT_DURATION_NS.with(|value| value.set(60_000_000_000));
            let _audit =
                crate::v9_real_consumer_shadow::begin_covered_provisional_physical_audit_v1();
            let _native_audit =
                crate::v9_real_consumer_shadow::begin_covered_native_physical_path_audit_v1();
            let _map_audit = crate::v9_real_consumer_shadow::v11_covered::
                begin_canonical_covered_map_role_audit_v1();
            let _full = force_full_provisional_envelope.then(
                crate::v9_real_consumer_shadow::force_covered_full_provisional_envelope_for_test,
            );
            let (runtime_swe_m, second_lane_swe_m) = match case {
                Case::NativeMixedPhase => (0.02, None),
                Case::NativeThawRefreeze | Case::NativeWetCanopy => (0.005, None),
                Case::NativeMultiOfe => (0.08, Some(0.08)),
                Case::Ordinary | Case::NativeFrozen => (0.08, None),
            };
            let execution = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
                run_complete_owner_on_large_stack(
                    runtime_swe_m,
                    None,
                    second_lane_swe_m,
                    false,
                    AdaptiveControllerTestPolicyV1::default(),
                    true,
                )
            }));
            let audit =
                crate::v9_real_consumer_shadow::take_covered_provisional_physical_audit_v1();
            let native_audit =
                crate::v9_real_consumer_shadow::take_covered_native_physical_path_audit_v1();
            let map_audit = crate::v9_real_consumer_shadow::v11_covered::
                take_canonical_covered_map_role_audit_v1();
            let rejection = crate::v9_real_consumer_shadow::v11_covered::
                take_canonical_covered_parity_rejection_for_test();
            match execution {
                Ok(_) => assert_eq!(rejection, None, "successful {case:?} prefix capture"),
                Err(payload) => {
                    let panic_message = payload
                        .downcast_ref::<String>()
                        .map(String::as_str)
                        .or_else(|| payload.downcast_ref::<&str>().copied());
                    assert!(
                        panic_message.is_some_and(|message| {
                            message.contains(
                            "AdaptiveRefinement(\"canonical covered dependent-output instability\")"
                        )
                        }),
                        "{case:?} rejected outside the expected adaptive response",
                    );
                    assert!(matches!(
                        rejection,
                        Some((
                            ErrorClass::AdaptiveRefinement,
                            Stage::AdjudicationConvergence,
                            _
                        ))
                    ));
                    let solve = map_audit
                        .solves
                        .last()
                        .unwrap_or_else(|| panic!("{case:?} dependent-rejection solve audit"));
                    assert_eq!(
                        solve.validated_physical_endpoint_count,
                        solve.charged_map_attempt_count,
                    );
                    assert!(solve.validated_pending_adjudication_count >= 1);
                    assert_eq!(solve.dependent_rejection_disposition_count, 1);
                    assert_eq!(solve.final_constructor_attempt_count, 0);
                    assert_eq!(solve.completed_final_envelope_count, 0);
                    assert_eq!(solve.final_disposition_count, 0);
                    assert_eq!(map_audit.map_local_publication_attempt_count, 0);
                    assert_eq!(map_audit.accepted_parent_publication_count, 0);
                    let rejected_solves = map_audit
                        .solves
                        .iter()
                        .filter(|solve| solve.dependent_rejection_disposition_count == 1)
                        .collect::<Vec<_>>();
                    assert!(!rejected_solves.is_empty());
                    assert!(rejected_solves.iter().all(|solve| {
                        solve.terminated
                            && !solve.completed
                            && solve.dependent_rejection_disposition_count == 1
                            && solve.final_constructor_attempt_count == 0
                            && solve.final_disposition_count == 0
                            && solve.completed_final_envelope_count == 0
                    }));
                    if case == Case::NativeMultiOfe {
                        assert!(
                            rejected_solves.len() >= 2,
                            "each adaptive retry must occupy a separate solve audit record",
                        );
                    }
                }
            }
            (audit, native_audit, map_audit)
        })
        .expect("spawn canonical covered physical-prefix capture")
        .join()
        .expect("join canonical covered physical-prefix capture")
}

pub(crate) fn canonical_covered_physical_parity_fixture_v1(
    case: crate::v9_real_consumer_shadow::v11_covered::CanonicalCoveredPhysicalParityCaseV1,
) -> (
    Vec<crate::v9_real_consumer_shadow::CoveredProvisionalPhysicalAuditV1>,
    Vec<crate::v9_real_consumer_shadow::CoveredProvisionalPhysicalAuditV1>,
    crate::v9_real_consumer_shadow::CoveredNativePhysicalPathAuditV1,
    crate::v9_real_consumer_shadow::CoveredNativePhysicalPathAuditV1,
    crate::v9_real_consumer_shadow::v11_covered::CanonicalCoveredMapRoleAuditV1,
    crate::v9_real_consumer_shadow::v11_covered::CanonicalCoveredMapRoleAuditV1,
) {
    let (physical_only, physical_only_native, physical_only_map) =
        run_canonical_covered_physical_prefix_capture(false, case);
    let (forced_complete, forced_complete_native, forced_complete_map) =
        run_canonical_covered_physical_prefix_capture(true, case);
    (
        physical_only,
        forced_complete,
        physical_only_native,
        forced_complete_native,
        physical_only_map,
        forced_complete_map,
    )
}

pub(crate) fn canonical_covered_physical_poison_fixture_v1(
    case: crate::v9_real_consumer_shadow::v11_covered::CanonicalCoveredPhysicalParityCaseV1,
    poison: crate::v9_real_consumer_shadow::v11_covered::CanonicalCoveredPhysicalParityPoisonV1,
) -> (
    crate::v9_real_consumer_shadow::v11_covered::CanonicalCoveredMapRoleAuditV1,
    Option<(
        crate::v9_real_consumer_shadow::v11_covered::CanonicalCoveredErrorClassV1,
        crate::v9_real_consumer_shadow::v11_covered::CanonicalCoveredRejectionStageV1,
        bool,
    )>,
) {
    canonical_covered_physical_poison_fixture_at_optional_map_v1(case, poison, None, None)
}

pub(crate) fn canonical_covered_physical_poison_fixture_at_map_v1(
    case: crate::v9_real_consumer_shadow::v11_covered::CanonicalCoveredPhysicalParityCaseV1,
    poison: crate::v9_real_consumer_shadow::v11_covered::CanonicalCoveredPhysicalParityPoisonV1,
    target: crate::v9_real_consumer_shadow::v11_covered::CanonicalCoveredPoisonTargetV1,
) -> (
    crate::v9_real_consumer_shadow::v11_covered::CanonicalCoveredMapRoleAuditV1,
    Option<(
        crate::v9_real_consumer_shadow::v11_covered::CanonicalCoveredErrorClassV1,
        crate::v9_real_consumer_shadow::v11_covered::CanonicalCoveredRejectionStageV1,
        bool,
    )>,
) {
    canonical_covered_physical_poison_fixture_at_optional_map_v1(case, poison, Some(target), None)
}

pub(crate) fn canonical_covered_convergence_poison_fixture_v1(
    case: crate::v9_real_consumer_shadow::v11_covered::CanonicalCoveredPhysicalParityCaseV1,
    convergence_poison: crate::v9_real_consumer_shadow::v11_covered::CanonicalCoveredFinalConvergencePoisonV1,
) -> (
    crate::v9_real_consumer_shadow::v11_covered::CanonicalCoveredMapRoleAuditV1,
    Option<(
        crate::v9_real_consumer_shadow::v11_covered::CanonicalCoveredErrorClassV1,
        crate::v9_real_consumer_shadow::v11_covered::CanonicalCoveredRejectionStageV1,
        bool,
    )>,
) {
    canonical_covered_physical_poison_fixture_at_optional_map_v1(
        case,
        crate::v9_real_consumer_shadow::v11_covered::CanonicalCoveredPhysicalParityPoisonV1::Precipitation,
        Some(crate::v9_real_consumer_shadow::v11_covered::CanonicalCoveredPoisonTargetV1 {
            role: crate::v9_real_consumer_shadow::v11_covered::CanonicalCoveredAuditRoleV1::UnchargedSentinel,
            ordinal: u32::MAX,
        }),
        Some(convergence_poison),
    )
}

fn canonical_covered_physical_poison_fixture_at_optional_map_v1(
    case: crate::v9_real_consumer_shadow::v11_covered::CanonicalCoveredPhysicalParityCaseV1,
    poison: crate::v9_real_consumer_shadow::v11_covered::CanonicalCoveredPhysicalParityPoisonV1,
    target: Option<crate::v9_real_consumer_shadow::v11_covered::CanonicalCoveredPoisonTargetV1>,
    convergence_poison: Option<
        crate::v9_real_consumer_shadow::v11_covered::CanonicalCoveredFinalConvergencePoisonV1,
    >,
) -> (
    crate::v9_real_consumer_shadow::v11_covered::CanonicalCoveredMapRoleAuditV1,
    Option<(
        crate::v9_real_consumer_shadow::v11_covered::CanonicalCoveredErrorClassV1,
        crate::v9_real_consumer_shadow::v11_covered::CanonicalCoveredRejectionStageV1,
        bool,
    )>,
) {
    std::thread::Builder::new()
        .name("canonical-covered-physical-poison".to_owned())
        .stack_size(64 * 1024 * 1024)
        .spawn(move || {
            use crate::v9_real_consumer_shadow::v11_covered::{
                CanonicalCoveredPhysicalParityCaseV1 as Case,
                CanonicalCoveredPhysicalParityPoisonV1 as Poison,
            };

            CANONICAL_PARITY_CASE.with(|slot| slot.set(Some(case)));
            EQUILIBRIUM_FIXTURE_REQUESTED.with(|value| value.set(matches!(
                case,
                crate::v9_real_consumer_shadow::v11_covered::
                    CanonicalCoveredPhysicalParityCaseV1::Ordinary
            )));
            SHORT_SUPPORT_DURATION_NS.with(|value| value.set(60_000_000_000));
            let _map_audit = crate::v9_real_consumer_shadow::v11_covered::
                begin_canonical_covered_map_role_audit_v1();
            let _convergence_poison = convergence_poison.map(|poison| {
                crate::v9_real_consumer_shadow::v11_covered::
                    force_canonical_covered_final_convergence_poison_for_test(poison)
            });
            crate::v9_real_consumer_shadow::v11_covered::
                begin_covered_carrier_endpoint_owner_projection_audit_v1();
            let _poison = if let Some(target) = target {
                crate::v9_real_consumer_shadow::v11_covered::
                    force_canonical_covered_targeted_parity_poison_for_test(poison, target)
            } else {
                crate::v9_real_consumer_shadow::v11_covered::
                    force_canonical_covered_parity_poison_for_test(poison)
            };
            let (runtime_swe_m, second_lane_swe_m) = match case {
                Case::NativeMixedPhase => (0.02, None),
                Case::NativeThawRefreeze | Case::NativeWetCanopy => (0.005, None),
                Case::NativeMultiOfe => (0.08, Some(0.08)),
                Case::Ordinary | Case::NativeFrozen => (0.08, None),
            };
            SHORT_SUPPORT_REQUESTED.with(|value| value.set(true));
            let _short_wb14_permit =
                crate::direct_runtime::permit_short_wb14_parent_support_for_test(
                    60_000_000_000,
                );
            let initial_cold_delta_k = match case {
                Case::NativeMixedPhase | Case::NativeThawRefreeze | Case::NativeWetCanopy => 0.0,
                Case::Ordinary | Case::NativeFrozen | Case::NativeMultiOfe => 8.0,
            };
            super::exercise_complete_wb14_cadence(
                runtime_swe_m,
                initial_cold_delta_k,
                matches!(case, Case::NativeWetCanopy),
                None,
                false,
                second_lane_swe_m,
                false,
                false,
                false,
                true,
            );
            let audit = crate::v9_real_consumer_shadow::v11_covered::
                take_canonical_covered_map_role_audit_v1();
            let endpoint_owner_projection_count = crate::v9_real_consumer_shadow::v11_covered::
                take_covered_carrier_endpoint_owner_projection_audit_v1();
            let charged_map_attempt_count = audit.solves.iter().fold(0_u32, |count, solve| {
                count.saturating_add(solve.charged_map_attempt_count)
            });
            let final_constructor_attempt_count = audit.solves.iter().fold(0_u32, |count, solve| {
                count.saturating_add(solve.final_constructor_attempt_count)
            });
            assert!(
                endpoint_owner_projection_count <= final_constructor_attempt_count,
                "a poisoned complete-owner endpoint projected its owner map more than once",
            );
            assert!(
                endpoint_owner_projection_count <= charged_map_attempt_count,
                "an uncharged covered map projected a complete owner endpoint",
            );
            let expected_endpoint_owner_projection_count = u32::from(matches!(
                poison,
                Poison::EndingJoint
                    | Poison::CompleteOwnerSet
                    | Poison::PublicationSupport
            ));
            assert_eq!(
                endpoint_owner_projection_count, expected_endpoint_owner_projection_count,
                "rejected physical/nonfinal maps must not project complete owner bytes, and a final map reaching the joint seam must project once",
            );
            let rejection = crate::v9_real_consumer_shadow::v11_covered::
                take_canonical_covered_parity_rejection_for_test();
            (audit, rejection)
        })
        .expect("spawn covered physical poison fixture")
        .join()
        .expect("join covered physical poison fixture")
}

fn run_equilibrium_complete_owner_for_duration_with_reuse_mode(
    duration_ns: u128,
    policy: AdaptiveControllerTestPolicyV1,
    force_double_physical_evaluation: bool,
    force_full_provisional_envelope: bool,
) -> AdaptiveProductionOutcomeV1 {
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
) -> AdaptiveProductionOutcomeV1 {
    run_transient_complete_owner_for_duration_with_memo_mode(duration_ns, policy, false)
}

fn run_transient_complete_owner_for_duration_with_memo_mode(
    duration_ns: u128,
    policy: AdaptiveControllerTestPolicyV1,
    force_memo_disabled: bool,
) -> AdaptiveProductionOutcomeV1 {
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

fn run_full_duration_complete_owner_qualification(
) -> crate::snow_stage3_v11_attachment::Stage3AdaptiveSupportReceiptV1 {
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
) -> AdaptiveProductionOutcomeV1 {
    use crate::v9_real_consumer_shadow::v11_covered::CanonicalCoveredPhysicalParityCaseV1 as Case;

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
    let parity_case = canonical_parity_case_requested();
    let initial_cold_delta_k = match parity_case {
        Some(Case::NativeMixedPhase | Case::NativeThawRefreeze | Case::NativeWetCanopy) => 0.0,
        Some(Case::Ordinary | Case::NativeFrozen | Case::NativeMultiOfe) | None => 8.0,
    };
    let include_child_17 = matches!(parity_case, Some(Case::NativeWetCanopy));
    super::exercise_complete_wb14_cadence(
        runtime_swe_m,
        initial_cold_delta_k,
        include_child_17,
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
    if equilibrium_fixture_requested()
        && !matches!(
            canonical_parity_case_requested(),
            Some(
                crate::v9_real_consumer_shadow::v11_covered::
                    CanonicalCoveredPhysicalParityCaseV1::NativeThawRefreeze
            )
        )
    {
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
    assert!(performance_spans
        .iter()
        .any(|span| span.label == "complete_owner_direct_trial"));
    assert!(!performance_spans
        .iter()
        .any(|span| span.label == "complete_owner_composed_trial"));
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
    let (physical_only, physical_audit, _, _) =
        run_equilibrium_with_provisional_physical_audit(false, None);
    let (forced_full, forced_audit, _, _) =
        run_equilibrium_with_provisional_physical_audit(true, None);
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

pub(crate) fn run_equilibrium_with_canonical_covered_role_audit() -> (
    bool,
    crate::v9_real_consumer_shadow::v11_covered::CanonicalCoveredMapRoleAuditV1,
) {
    std::thread::Builder::new()
        .name("canonical-covered-role-audit".to_owned())
        .stack_size(64 * 1024 * 1024)
        .spawn(move || {
            EQUILIBRIUM_FIXTURE_REQUESTED.with(|value| value.set(true));
            SHORT_SUPPORT_DURATION_NS.with(|value| value.set(MINIMUM_STEP_NS));
            let _audit = crate::v9_real_consumer_shadow::v11_covered::
                begin_canonical_covered_map_role_audit_v1();
            crate::v9_real_consumer_shadow::v11_covered::
                begin_covered_carrier_endpoint_owner_projection_audit_v1();
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
            let audit = crate::v9_real_consumer_shadow::v11_covered::
                take_canonical_covered_map_role_audit_v1();
            let endpoint_owner_projection_count = crate::v9_real_consumer_shadow::v11_covered::
                take_covered_carrier_endpoint_owner_projection_audit_v1();
            let completed_final_envelope_count = audit.solves.iter().fold(0_u32, |count, solve| {
                count.saturating_add(solve.completed_final_envelope_count)
            });
            assert_eq!(
                endpoint_owner_projection_count, completed_final_envelope_count,
                "each real successful complete-owner map must project canonical owner bytes once",
            );
            (panicked, audit)
        })
        .expect("spawn canonical covered role audit")
        .join()
        .expect("join canonical covered role audit")
}

pub(crate) fn run_transient_with_canonical_covered_role_audit(
    convergence_poison: Option<
        crate::v9_real_consumer_shadow::v11_covered::CanonicalCoveredFinalConvergencePoisonV1,
    >,
) -> (
    bool,
    crate::v9_real_consumer_shadow::v11_covered::CanonicalCoveredMapRoleAuditV1,
) {
    let (panicked, audit, _) =
        run_transient_with_canonical_covered_role_and_proposal_audit(convergence_poison, None);
    (panicked, audit)
}

pub(crate) fn run_transient_with_canonical_covered_multisecant_poison_audit(
    convergence_poison: crate::v9_real_consumer_shadow::v11_covered::CanonicalCoveredFinalConvergencePoisonV1,
    proposal_poison: crate::v9_real_consumer_shadow::v11_covered::CanonicalCoveredMultisecantProposalPoisonV1,
) -> (
    bool,
    crate::v9_real_consumer_shadow::v11_covered::CanonicalCoveredMapRoleAuditV1,
    Option<(
        crate::v9_real_consumer_shadow::v11_covered::CanonicalCoveredErrorClassV1,
        crate::v9_real_consumer_shadow::v11_covered::CanonicalCoveredRejectionStageV1,
        bool,
    )>,
) {
    run_transient_with_canonical_covered_role_and_proposal_audit(
        Some(convergence_poison),
        Some(proposal_poison),
    )
}

fn run_transient_with_canonical_covered_role_and_proposal_audit(
    convergence_poison: Option<
        crate::v9_real_consumer_shadow::v11_covered::CanonicalCoveredFinalConvergencePoisonV1,
    >,
    proposal_poison: Option<
        crate::v9_real_consumer_shadow::v11_covered::CanonicalCoveredMultisecantProposalPoisonV1,
    >,
) -> (
    bool,
    crate::v9_real_consumer_shadow::v11_covered::CanonicalCoveredMapRoleAuditV1,
    Option<(
        crate::v9_real_consumer_shadow::v11_covered::CanonicalCoveredErrorClassV1,
        crate::v9_real_consumer_shadow::v11_covered::CanonicalCoveredRejectionStageV1,
        bool,
    )>,
) {
    std::thread::Builder::new()
        .name("canonical-covered-transient-role-audit".to_owned())
        .stack_size(64 * 1024 * 1024)
        .spawn(move || {
            SHORT_SUPPORT_DURATION_NS.with(|value| value.set(MINIMUM_STEP_NS));
            let _audit = crate::v9_real_consumer_shadow::v11_covered::
                begin_canonical_covered_map_role_audit_v1();
            let _convergence_poison = convergence_poison.map(|poison| {
                crate::v9_real_consumer_shadow::v11_covered::
                    force_canonical_covered_final_convergence_poison_for_test(poison)
            });
            let _proposal_poison = proposal_poison.map(|poison| {
                crate::v9_real_consumer_shadow::v11_covered::
                    force_canonical_covered_multisecant_proposal_poison_for_test(poison)
            });
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
            let audit = crate::v9_real_consumer_shadow::v11_covered::
                take_canonical_covered_map_role_audit_v1();
            let rejection = crate::v9_real_consumer_shadow::v11_covered::
                take_canonical_covered_parity_rejection_for_test();
            (panicked, audit, rejection)
        })
        .expect("spawn canonical covered transient role audit")
        .join()
        .expect("join canonical covered transient role audit")
}

#[test]
fn canonical_covered_success_has_m_minus_one_private_maps_and_one_private_final_envelope() {
    use crate::v9_real_consumer_shadow::v11_covered::CanonicalCoveredAuditRoleV1 as Role;

    let (panicked, audit) = run_equilibrium_with_canonical_covered_role_audit();
    assert!(!panicked, "canonical covered success fixture failed");
    assert!(!audit.solves.is_empty(), "missing covered solve audit");
    for solve in &audit.solves {
        assert!(solve.completed);
        assert!((2..=7).contains(&solve.charged_map_attempt_count));
        assert_eq!(
            solve.charged_roles.len(),
            usize::try_from(solve.charged_map_attempt_count).expect("charged role count"),
            "charged-role audit omitted or duplicated a map"
        );
        assert_eq!(
            solve.validated_physical_endpoint_count,
            solve.charged_map_attempt_count
        );
        assert_eq!(
            solve.validated_pending_adjudication_count + 1,
            solve.charged_map_attempt_count
        );
        assert_eq!(
            solve.history_disposition_count + 2,
            solve.charged_map_attempt_count
        );
        assert_eq!(solve.dependent_rejection_disposition_count, 0);
        assert_eq!(solve.final_disposition_count, 1);
        assert_eq!(solve.final_constructor_attempt_count, 1);
        assert_eq!(solve.completed_final_envelope_count, 1);
        assert_eq!(solve.v8_receipt_constructor_attempt_count, 1);
        assert_eq!(solve.vegetation_persistent_constructor_attempt_count, 1);
        assert_eq!(solve.vegetation_material_constructor_attempt_count, 1);
        assert_eq!(solve.biogeochemistry_constructor_attempt_count, 1);
        assert_eq!(solve.ending_joint_constructor_attempt_count, 1);
        assert_eq!(solve.complete_owner_set_constructor_attempt_count, 1);
        assert_eq!(
            solve.restart_owner_constructor_attempt_count, 0,
            "a private map-local final envelope has no restart/history owner"
        );
        assert_eq!(solve.charged_roles.first(), Some(&(Role::Initial, 0)));
        if solve.charged_map_attempt_count == 2 {
            assert_eq!(
                solve.charged_roles,
                vec![(Role::Initial, 0), (Role::FixedPointAdjudication, 1)],
                "stable support must consume the fixed-point adjudication map as final",
            );
        } else {
            assert_eq!(
                solve.charged_roles.get(1),
                Some(&(Role::FixedPointAdjudication, 1)),
                "iterative support must begin with one fixed-point adjudication",
            );
            for (offset, role) in solve.charged_roles[2..].iter().enumerate() {
                assert_eq!(
                    role,
                    &(
                        Role::MultisecantAdjudication(
                            u8::try_from(offset + 1).expect("trial index"),
                        ),
                        u32::try_from(offset + 2).expect("trial ordinal"),
                    )
                );
            }
        }
    }
    assert_eq!(
        audit.accepted_parent_publication_count, 0,
        "the adaptive Complete result remains private until attachment installation"
    );
    assert_eq!(
        audit.successful_history_append_count, 1,
        "the selected composed parent stages exactly one validated support history append"
    );
    assert_eq!(audit.map_local_publication_attempt_count, 0);
    let source = include_str!("v11_covered/canonical_covered_solver.rs");
    let pending = source
        .split_once("struct CanonicalCoveredPendingAdjudicationMapV1")
        .expect("private pending-adjudication typestate source")
        .1
        .split_once("struct CanonicalCoveredFinalMapV1")
        .expect("private final-map boundary")
        .0;
    // The private pending result has no coupled-time publication API, so
    // ERR-CT-018 is unreachable here by construction rather than by a
    // behavioral counter whose zero value could never observe an attempt.
    for forbidden in [
        "finalize_v11_imported_segment",
        "complete_owner_set_digest",
        "accept_slab(",
        "accept_direct_v11_segment",
        "fallback",
        "promote",
    ] {
        assert!(
            !pending.contains(forbidden),
            "private pending map structurally exposes forbidden {forbidden} path"
        );
    }
    assert!(
        !pending.contains("derive(Clone") && !pending.contains("impl Clone"),
        "pending adjudication typestate must remain non-Clone",
    );
    let final_disposition = source
        .split_once("fn consume_canonical_covered_pending_as_final_v1")
        .expect("same-map final disposition source")
        .1
        .split_once("fn execute_canonical_covered_production_v1")
        .expect("canonical production boundary")
        .0;
    assert!(final_disposition.contains("complete_covered_carrier_physical_phase_v1"));
    for replay in [
        "canonical_covered_map_endpoint_v1(",
        "execute_covered_carrier_physical_phase_v1(",
        "canonical_covered_audit_charge_v1(",
    ] {
        assert!(
            !final_disposition.contains(replay),
            "final disposition must not replay or recharge `{replay}`",
        );
    }
}

#[test]
fn canonical_covered_failure_matrix_never_completes_or_publishes_a_failed_envelope() {
    use crate::v9_real_consumer_shadow::v11_covered::{
        CanonicalCoveredAuditRoleV1 as Role, CanonicalCoveredErrorClassV1 as ErrorClass,
        CanonicalCoveredPhysicalParityCaseV1 as Case,
        CanonicalCoveredPhysicalParityPoisonV1 as Poison, CanonicalCoveredPoisonTargetV1,
        CanonicalCoveredRejectionStageV1 as Stage,
    };

    for (
        case,
        poison,
        target,
        expected_error,
        expected_stage,
        expected_counts,
        expected_dispositions,
    ) in [
        (
            Case::NativeMixedPhase,
            Poison::Precipitation,
            CanonicalCoveredPoisonTargetV1 {
                role: Role::FixedPointAdjudication,
                ordinal: 1,
            },
            ErrorClass::Stage3PrecipitationCustody,
            Stage::Physical,
            (2, 1, 0),
            (0, 0, 0, 0),
        ),
        (
            Case::Ordinary,
            Poison::Precipitation,
            CanonicalCoveredPoisonTargetV1 {
                role: Role::FixedPointAdjudication,
                ordinal: 1,
            },
            ErrorClass::Stage3PrecipitationCustody,
            Stage::Physical,
            (2, 1, 0),
            (0, 0, 0, 0),
        ),
        (
            Case::Ordinary,
            Poison::V8Persistent,
            CanonicalCoveredPoisonTargetV1 {
                role: Role::FixedPointAdjudication,
                ordinal: 1,
            },
            ErrorClass::V8Vegetation,
            Stage::V8Persistent,
            (2, 2, 1),
            (1, 0, 0, 1),
        ),
    ] {
        let rejection = crate::v9_real_consumer_shadow::v11_covered::
            canonical_covered_targeted_failure_for_test(case, poison, target);
        assert_eq!(
            rejection.typed_error, expected_error,
            "{poison:?}/{target:?}"
        );
        assert_eq!(
            rejection.rejection_stage, expected_stage,
            "{poison:?}/{target:?}"
        );
        assert_eq!(
            (
                rejection.charged_map_attempt_count,
                rejection.validated_physical_endpoint_count,
                rejection.final_constructor_attempt_count,
            ),
            expected_counts,
            "{poison:?}/{target:?}",
        );
        assert_eq!(
            (
                rejection.validated_pending_adjudication_count,
                rejection.history_disposition_count,
                rejection.dependent_rejection_disposition_count,
                rejection.final_disposition_count,
            ),
            expected_dispositions,
            "{poison:?}/{target:?}",
        );
        assert_eq!(
            rejection.charged_roles,
            vec![(Role::Initial, 0), (Role::FixedPointAdjudication, 1)],
            "failed adjudication must not charge a replay or successor",
        );
        assert_eq!(rejection.completed_final_envelope_count, 0);
        assert_eq!(rejection.map_local_publication_attempt_count, 0);
        assert_eq!(rejection.accepted_parent_publication_count, 0);
        assert!(rejection.rollback_byte_identical, "{poison:?}/{target:?}");
    }

    // Defense in depth: the executable matrix above is primary evidence;
    // this scan only keeps obsolete fabricated injection vocabulary deleted.
    let solver = include_str!("v11_covered/canonical_covered_solver.rs");
    let audit = include_str!("v11_covered/canonical_covered_solver_test_audit.rs");
    let v8 = include_str!("land_surface_energy_shadow/covered_v8_owner.rs");
    let carrier = include_str!("v11_covered/carrier_phase.rs");
    let finalizer = include_str!("v11_covered/owner_finalization.rs");
    for source in [solver, audit, v8, carrier, finalizer] {
        for fabricated in [
            "forced nonfinal covered physical failure",
            "forced final covered physical failure",
            "forced nonfinal covered physical isolation failure",
            "forced final V8 receipt boundary",
            "forced final covered owner boundary",
            "forced final covered ending-joint boundary",
            "forced final complete-owner boundary",
            "CanonicalCoveredFailureInjectionV1",
        ] {
            assert!(
                !source.contains(fabricated),
                "covered failure evidence still fabricates {fabricated}"
            );
        }
    }
}

#[test]
fn canonical_covered_physical_prefix_matches_forced_complete_regime_matrix() {
    use crate::v9_real_consumer_shadow::v11_covered::CanonicalCoveredPhysicalParityCaseV1 as Case;

    for case in [
        Case::Ordinary,
        Case::NativeFrozen,
        Case::NativeMixedPhase,
        Case::NativeThawRefreeze,
        Case::NativeWetCanopy,
        Case::NativeMultiOfe,
    ] {
        let parity = crate::v9_real_consumer_shadow::v11_covered::
            canonical_covered_physical_parity_for_test(case)
            .unwrap_or_else(|error| {
                panic!("execute {case:?} physical-only and forced-complete parity paths: {error:?}")
            });
        assert_eq!(
            parity.physical_only, parity.forced_complete,
            "{case:?} physical prefix differs"
        );
        assert!(parity.physical_only.includes_stage3_endings);
        assert!(parity.physical_only.includes_surface_wb14_custody);
        assert!(parity.physical_only.realizes_named_regime, "{case:?}");
        if !matches!(case, Case::Ordinary) {
            assert_eq!(
                parity.native_snow_free_litter_physics_call_count, 0,
                "{case:?}"
            );
            assert_eq!(
                parity.native_snow_free_surface_physics_call_count, 0,
                "{case:?}"
            );
            assert_eq!(
                parity.native_snow_free_wb14_physics_call_count, 0,
                "{case:?}"
            );
            assert!(
                parity
                    .physical_only
                    .retains_inactive_native_litter_wb14_bytes
            );
        }
    }
}

#[test]
#[allow(
    clippy::too_many_lines,
    reason = "one real cadence fixture drives the complete C-025 poison matrix"
)]
fn native_inactive_prefix_validator_consumes_real_ordered_terminal_chronology() {
    std::thread::Builder::new()
        .name("canonical-native-prefix-validation".to_owned())
        .stack_size(64 * 1024 * 1024)
        .spawn(|| {
            use crate::v9_real_consumer_shadow::v11_covered::
                CanonicalCoveredPhysicalParityCaseV1 as Case;

            CANONICAL_PARITY_CASE.with(|slot| slot.set(Some(Case::NativeMixedPhase)));
            SHORT_SUPPORT_DURATION_NS.with(|value| value.set(1_800_000_000_000));
            super::begin_real_native_inactive_prefix_capture_v1();
            let _ = run_complete_owner_on_large_stack(
                0.000_6,
                Some(60_000_000_000),
                None,
                true,
                AdaptiveControllerTestPolicyV1::default(),
                false,
            );
            let fixture = super::take_real_native_inactive_prefix_capture_v1()
                .expect("real native inactive prefix capture");
            let terminal_end = fixture
                .terminal_groups
                .last()
                .expect("accepted native terminal group")
                .tick;
            let prefix = fixture
                .receipts
                .iter()
                .filter(|receipt| receipt.support.end_ns() <= terminal_end)
                .cloned()
                .collect::<Vec<_>>();
            assert!(
                prefix.len() >= 2,
                "fixture must authenticate at least two inactive children: {:?}",
                fixture
                    .receipts
                    .iter()
                    .map(|receipt| (
                        receipt.support.start_ns().get(),
                        receipt.support.end_ns().get()
                    ))
                    .collect::<Vec<_>>()
            );
            let validated = crate::direct_runtime::validate_native_inactive_wb14_prefix_v1(
                &prefix,
                &fixture.terminal_groups,
                fixture.parent_support,
                &fixture.surface_configuration,
            )
            .expect("real native inactive prefix");
            let restart_bytes = serde_json::to_vec(&validated).expect("prefix restart bytes");
            let replayed: crate::direct_runtime::ValidatedNativeInactiveWb14PrefixV1 =
                serde_json::from_slice(&restart_bytes).expect("prefix restart replay");
            assert_eq!(validated, replayed, "prefix restart changed canonical authority");

            let terminal_receipt = fixture
                .terminal_groups
                .last()
                .and_then(|group| group.accepted_event_receipt.as_ref())
                .expect("accepted terminal receipt");
            let successor_binding = crate::direct_runtime::DirectWb14CoupledChildBindingV1 {
                proposed_upper_bound_s_bits: 60.0_f64.to_bits(),
                coupled_parent_transaction_sha256: *prefix[0]
                    .owner_join
                    .parent_transaction_sha256
                    .as_bytes(),
                accepted_slab_sha256: [7; 32],
                parent_beginning_complete_owner_set_sha256: *terminal_receipt
                    .ending_owner_set_digest()
                    .as_bytes(),
                parent_support_start_ns: fixture.parent_support.start_ns().get(),
                parent_support_end_ns: fixture.parent_support.end_ns().get(),
                child_support_start_ns: terminal_end.get(),
                child_support_end_ns: terminal_end.get() + 60_000_000_000,
            };
            replayed
                .validate_successor_binding(successor_binding)
                .expect("replayed prefix authorizes exact first physical successor");
            for (name, poisoned) in [
                (
                    "wrong prefix end",
                    crate::direct_runtime::DirectWb14CoupledChildBindingV1 {
                        child_support_start_ns: successor_binding.child_support_start_ns + 1,
                        ..successor_binding
                    },
                ),
                (
                    "foreign parent",
                    crate::direct_runtime::DirectWb14CoupledChildBindingV1 {
                        coupled_parent_transaction_sha256: [8; 32],
                        ..successor_binding
                    },
                ),
                (
                    "foreign prefix-ending owner",
                    crate::direct_runtime::DirectWb14CoupledChildBindingV1 {
                        parent_beginning_complete_owner_set_sha256: [9; 32],
                        ..successor_binding
                    },
                ),
            ] {
                assert!(
                    replayed.validate_successor_binding(poisoned).is_err(),
                    "{name} successor binding was admitted"
                );
            }

            let assert_rejected = |name: &str,
                                   receipts: &[crate::snow_stage3_v11_attachment::
                                        Stage3CoupledSubslabReceiptV1],
                                   groups: &[crate::snow_stage3_v11_attachment::
                                        Stage3V11TerminalEventGroupV1],
                                   surface_configuration: &crate::DirectSurfaceLiquidConfiguration| {
                assert!(
                    crate::direct_runtime::validate_native_inactive_wb14_prefix_v1(
                        receipts,
                        groups,
                        fixture.parent_support,
                        surface_configuration,
                    )
                    .is_err(),
                    "{name} prefix poison was admitted"
                );
            };

            assert_rejected(
                "omitted child",
                &prefix[1..],
                &fixture.terminal_groups,
                &fixture.surface_configuration,
            );
            let mut reordered = prefix.clone();
            reordered.swap(0, 1);
            assert_rejected(
                "reordered children",
                &reordered,
                &fixture.terminal_groups,
                &fixture.surface_configuration,
            );
            let mut extra = prefix.clone();
            extra.push(prefix[0].clone());
            let overlap_error = crate::direct_runtime::validate_native_inactive_wb14_prefix_v1(
                &extra,
                &fixture.terminal_groups,
                fixture.parent_support,
                &fixture.surface_configuration,
            )
            .expect_err("overlapping child");
            assert_eq!(
                overlap_error.canonical_surface_liquid_error_code(),
                crate::DirectSurfaceLiquidErrorCode::E008
            );
            let mut discontinuous_owner = prefix.clone();
            discontinuous_owner[1]
                .owner_join
                .beginning_complete_owner_set_sha256 = openwepp_coupled_time::Digest32::from_bytes([250; 32]);
            let owner_error = crate::direct_runtime::validate_native_inactive_wb14_prefix_v1(
                &discontinuous_owner,
                &fixture.terminal_groups,
                fixture.parent_support,
                &fixture.surface_configuration,
            )
            .expect_err("discontinuous owner chain");
            assert_eq!(
                owner_error.canonical_surface_liquid_error_code(),
                crate::DirectSurfaceLiquidErrorCode::E002
            );
            let mut owner = prefix.clone();
            owner[0].owner_join.parent_transaction_sha256 =
                openwepp_coupled_time::Digest32::from_bytes([251; 32]);
            assert_rejected(
                "foreign owner",
                &owner,
                &fixture.terminal_groups,
                &fixture.surface_configuration,
            );
            let mut wrong_slab = prefix.clone();
            wrong_slab[0].accepted_slab_sha256 =
                openwepp_coupled_time::Digest32::from_bytes([252; 32]);
            assert_rejected(
                "wrong accepted-slab custody",
                &wrong_slab,
                &fixture.terminal_groups,
                &fixture.surface_configuration,
            );
            let mut marker = prefix.clone();
            marker[0].wb14_child_replay_bytes[0] ^= 1;
            assert_rejected(
                "inactive marker",
                &marker,
                &fixture.terminal_groups,
                &fixture.surface_configuration,
            );
            let mut foreign_configuration = fixture.surface_configuration.clone();
            foreign_configuration.ofe_topology.push(
                openwepp_land_surface_energy::OfeId::try_new("foreign-prefix-ofe")
                    .expect("foreign OFE"),
            );
            assert_rejected(
                "foreign topology",
                &prefix,
                &fixture.terminal_groups,
                &foreign_configuration,
            );
            let mut gap = prefix.clone();
            gap.remove(1);
            assert_rejected(
                "support gap",
                &gap,
                &fixture.terminal_groups,
                &fixture.surface_configuration,
            );
            let mut terminal = fixture.terminal_groups.clone();
            terminal[0].ordinal = terminal[0].ordinal.saturating_add(1);
            let ordinal_error = crate::direct_runtime::validate_native_inactive_wb14_prefix_v1(
                &prefix,
                &terminal,
                fixture.parent_support,
                &fixture.surface_configuration,
            )
            .expect_err("terminal ordinal");
            assert_eq!(
                ordinal_error.canonical_surface_liquid_error_code(),
                crate::DirectSurfaceLiquidErrorCode::E008
            );
            let mut terminal = fixture.terminal_groups.clone();
            terminal[0].discovery_receipt_sha256 =
                openwepp_coupled_time::Digest32::from_bytes([10; 32]);
            assert_rejected(
                "terminal discovery receipt",
                &prefix,
                &terminal,
                &fixture.surface_configuration,
            );
            let mut terminal = fixture.terminal_groups.clone();
            terminal[0].receipt_sha256 = openwepp_coupled_time::Digest32::from_bytes([11; 32]);
            assert_rejected(
                "terminal preaccept receipt",
                &prefix,
                &terminal,
                &fixture.surface_configuration,
            );
            let mut terminal = fixture.terminal_groups.clone();
            terminal[0].accepted_group_receipt_sha256 =
                Some(openwepp_coupled_time::Digest32::from_bytes([12; 32]));
            assert_rejected(
                "accepted terminal group receipt",
                &prefix,
                &terminal,
                &fixture.surface_configuration,
            );
        })
        .expect("spawn native prefix validation")
        .join()
        .expect("join native prefix validation");
}

#[test]
fn native_terminal_batch_dispatch_accepts_exact_physical_marker_topology() {
    use crate::v9_real_consumer_shadow::v11_covered::CanonicalCoveredPhysicalParityCaseV1 as Case;

    let (_, evidence, _, _) =
        run_equilibrium_with_provisional_physical_audit(false, Some(Case::NativeMultiOfe));
    assert!(!evidence.is_empty(), "missing native physical endpoint");
    for endpoint in evidence {
        assert!(
            endpoint.stage3_covered_native,
            "ordinary endpoint in native fixture"
        );
        let topology = endpoint.surface_ofe_topology.clone();
        let lower_boundary_ofes = endpoint
            .corrected_boundaries
            .keys()
            .map(|(ofe_id, _)| ofe_id)
            .collect::<std::collections::BTreeSet<_>>();
        assert!(!lower_boundary_ofes.is_empty());
        assert!(lower_boundary_ofes
            .iter()
            .all(|ofe_id| topology.contains(ofe_id)));
        let binding = crate::direct_runtime::stage3_covered_native_inactive_child_custody_binding(
            &endpoint.wb14_child_replay_bytes,
            &topology,
        )
        .unwrap_or_else(|error| panic!("exact native marker/topology rejected: {error:?}"));
        assert!(
            binding.is_some(),
            "native endpoint carried ordinary WB14 replay bytes"
        );
    }

    let topology = ["z-upstream", "a-downstream"]
        .into_iter()
        .map(|value| openwepp_land_surface_energy::OfeId::try_new(value).expect("opaque OFE"))
        .collect::<Vec<_>>();
    let marker = crate::direct_runtime::stage3_covered_native_inactive_child_custody_bytes(
        crate::direct_runtime::DirectWb14CoupledChildBindingV1 {
            proposed_upper_bound_s_bits: 900.0_f64.to_bits(),
            coupled_parent_transaction_sha256: [1; 32],
            accepted_slab_sha256: [2; 32],
            parent_beginning_complete_owner_set_sha256: [3; 32],
            parent_support_start_ns: 0,
            parent_support_end_ns: 20,
            child_support_start_ns: 10,
            child_support_end_ns: 20,
        },
        &topology,
    )
    .expect("nonlexical native marker");
    assert!(
        crate::direct_runtime::stage3_covered_native_inactive_child_custody_binding(
            &marker, &topology,
        )
        .expect("authenticated nonlexical topology")
        .is_some()
    );
    let mut lexically_sorted = topology.clone();
    lexically_sorted.sort();
    assert!(
        crate::direct_runtime::stage3_covered_native_inactive_child_custody_binding(
            &marker,
            &lexically_sorted,
        )
        .is_err(),
        "lexical OFE sorting changed opaque topology authority",
    );
}

#[test]
#[allow(clippy::too_many_lines)] // One table preserves the real failure chronology and counts.
fn canonical_covered_physical_prefix_poisons_reject_with_exact_rollback() {
    use crate::v9_real_consumer_shadow::v11_covered::{
        CanonicalCoveredErrorClassV1 as ErrorClass, CanonicalCoveredPhysicalParityCaseV1 as Case,
        CanonicalCoveredPhysicalParityPoisonV1 as Poison,
        CanonicalCoveredRejectionStageV1 as Stage,
    };

    for case in [
        Case::Ordinary,
        Case::NativeFrozen,
        Case::NativeMixedPhase,
        Case::NativeThawRefreeze,
        Case::NativeWetCanopy,
        Case::NativeMultiOfe,
    ] {
        for poison in [
            Poison::RoleOrdinal,
            Poison::Support,
            Poison::Transaction,
            Poison::Topology,
            Poison::LowerBoundary,
            Poison::Precipitation,
            Poison::SoilCandidate,
            Poison::BeginningOwner,
            Poison::NativeOrdinarySubstitution,
            Poison::HalfNativeCustody,
            Poison::PhysicalOneUlp,
            Poison::RoleOrdinalAndPhysicalOneUlp,
            Poison::SupportAndPhysicalOneUlp,
            Poison::LowerBoundaryAndV8Persistent,
        ] {
            let rejection = crate::v9_real_consumer_shadow::v11_covered::
                canonical_covered_physical_parity_poison_for_test(case, poison);
            let (expected_error, expected_stage) = match poison {
                Poison::RoleOrdinal | Poison::RoleOrdinalAndPhysicalOneUlp => {
                    (ErrorClass::AdaptiveRefinement, Stage::RoleOrdinal)
                }
                Poison::Support
                | Poison::Transaction
                | Poison::Topology
                | Poison::BeginningOwner
                | Poison::NativeOrdinarySubstitution
                | Poison::HalfNativeCustody
                | Poison::SupportAndPhysicalOneUlp => {
                    (ErrorClass::Identity, Stage::PreflightIdentity)
                }
                Poison::LowerBoundary | Poison::LowerBoundaryAndV8Persistent => {
                    (ErrorClass::CoveredBoundary, Stage::Physical)
                }
                Poison::Precipitation => (ErrorClass::Stage3PrecipitationCustody, Stage::Physical),
                Poison::SoilCandidate => (ErrorClass::Stage3SnowSoilHeatCustody, Stage::Physical),
                Poison::PhysicalOneUlp => (ErrorClass::Stage3, Stage::PhysicalValidation),
                Poison::V8Persistent
                | Poison::V8VegetationCandidate
                | Poison::V8Biogeochemistry
                | Poison::V8EnvelopeValidation
                | Poison::EndingJoint
                | Poison::CompleteOwnerSet
                | Poison::PublicationSupport => {
                    unreachable!("constructor poisons are exercised below")
                }
            };
            assert_eq!(rejection.typed_error, expected_error, "{case:?}/{poison:?}");
            assert_eq!(
                rejection.rejection_stage, expected_stage,
                "{case:?}/{poison:?}"
            );
            assert!(
                rejection.rollback_byte_identical,
                "{case:?}/{poison:?} changed live owners"
            );
            assert_eq!(rejection.completed_final_envelope_count, 0);
            assert_eq!(rejection.accepted_parent_publication_count, 0);
        }
    }

    for (
        poison,
        expected_error,
        expected_stage,
        completed_envelopes,
        expected_constructor_counts,
    ) in [
        (
            Poison::V8Persistent,
            ErrorClass::V8Vegetation,
            Stage::V8Persistent,
            0,
            [1, 1, 0, 0, 0, 0, 0, 0],
        ),
        (
            Poison::V8VegetationCandidate,
            ErrorClass::V8Vegetation,
            Stage::V8VegetationCandidate,
            0,
            [1, 1, 1, 0, 0, 0, 0, 0],
        ),
        (
            Poison::V8Biogeochemistry,
            ErrorClass::V8Biogeochemistry,
            Stage::V8Biogeochemistry,
            0,
            [1, 1, 1, 1, 0, 0, 0, 0],
        ),
        (
            Poison::V8EnvelopeValidation,
            ErrorClass::V8OwnerIdentity,
            Stage::V8EnvelopeValidation,
            0,
            [1, 1, 1, 1, 0, 0, 0, 0],
        ),
        (
            Poison::EndingJoint,
            ErrorClass::Stage3,
            Stage::EndingJoint,
            0,
            [1, 1, 1, 1, 1, 0, 0, 0],
        ),
        (
            Poison::CompleteOwnerSet,
            ErrorClass::CoupledTime,
            Stage::CompleteOwnerSet,
            0,
            [1, 1, 1, 1, 1, 1, 0, 0],
        ),
        (
            Poison::PublicationSupport,
            ErrorClass::Identity,
            Stage::PublicationSupport,
            1,
            [1, 1, 1, 1, 1, 1, 0, 1],
        ),
    ] {
        let rejection = crate::v9_real_consumer_shadow::v11_covered::
            canonical_covered_physical_parity_poison_for_test(Case::Ordinary, poison);
        assert_eq!(rejection.typed_error, expected_error, "{poison:?}");
        assert_eq!(rejection.rejection_stage, expected_stage, "{poison:?}");
        assert!(rejection.rollback_byte_identical, "{poison:?}");
        assert_eq!(rejection.charged_map_attempt_count, 2, "{poison:?}");
        assert_eq!(rejection.validated_physical_endpoint_count, 2, "{poison:?}");
        assert_eq!(
            rejection.validated_iteration_endpoint_count, 1,
            "{poison:?}"
        );
        assert_eq!(
            rejection.validated_final_physical_endpoint_count, 1,
            "{poison:?}"
        );
        assert_eq!(
            rejection.validated_pending_adjudication_count, 1,
            "{poison:?}"
        );
        assert_eq!(rejection.history_disposition_count, 0, "{poison:?}");
        assert_eq!(
            rejection.dependent_rejection_disposition_count, 0,
            "{poison:?}"
        );
        assert_eq!(rejection.final_disposition_count, 1, "{poison:?}");
        assert_eq!(rejection.final_constructor_attempt_count, 1, "{poison:?}");
        assert_eq!(
            [
                rejection.v8_receipt_constructor_attempt_count,
                rejection.vegetation_persistent_constructor_attempt_count,
                rejection.vegetation_material_constructor_attempt_count,
                rejection.biogeochemistry_constructor_attempt_count,
                rejection.ending_joint_constructor_attempt_count,
                rejection.complete_owner_set_constructor_attempt_count,
                rejection.restart_owner_constructor_attempt_count,
                rejection.publication_support_constructor_attempt_count,
            ],
            expected_constructor_counts,
            "{poison:?}",
        );
        assert_eq!(
            rejection.completed_final_envelope_count, completed_envelopes,
            "{poison:?}"
        );
        assert_eq!(rejection.accepted_parent_publication_count, 0, "{poison:?}");
    }
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
    assert!(receipt.accepted_microsteps.iter().any(|accepted| accepted
        .context
        .step_support
        .duration_ns()
        == MINIMUM_STEP_NS));
    assert!(carrier_supports
        .iter()
        .all(|support| support.duration_ns() >= MINIMUM_STEP_NS));
    assert!(comparison_audits
        .iter()
        .any(|audit| audit.maximum_scaled_error > 1.0));
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
        "ADAPTIVE_POLICY_INVARIANCE default_trace={default_trace:?} alternate_trace={alternate_trace:?} final_max={maximum_physical_scalar:?}",
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
        0.000_8,
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
    assert!(carrier_supports
        .iter()
        .all(|support| support.duration_ns() >= MINIMUM_STEP_NS));
}

#[test]
#[cfg(any())]
#[ignore = "contract-first expected red: retained 52-map terminal-parent validation audit is not implemented"]
fn carrier_parent_static_and_same_map_validation_once_has_authentic_1_52_52_counts() {
    use crate::v9_real_consumer_shadow::v11_covered::{
        begin_carrier_parent_validation_once_audit_v1,
        take_carrier_parent_validation_once_audit_v1, CarrierValidationCallStageV1 as Stage,
    };

    let (outcome, audit) = std::thread::Builder::new()
        .name("carrier-validation-once-52-map-parent".to_owned())
        .stack_size(64 * 1024 * 1024)
        .spawn(|| {
            let _audit = begin_carrier_parent_validation_once_audit_v1();
            EQUILIBRIUM_FIXTURE_REQUESTED.with(|value| value.set(false));
            SHORT_SUPPORT_REQUESTED.with(|value| value.set(false));
            let outcome = run_complete_owner_on_large_stack(
                0.000_8,
                None,
                Some(0.005),
                true,
                AdaptiveControllerTestPolicyV1::default(),
                false,
            );
            (outcome, take_carrier_parent_validation_once_audit_v1())
        })
        .expect("spawn authentic 52-map parent validation audit")
        .join()
        .expect("join authentic 52-map parent validation audit");

    outcome
        .0
        .validate()
        .expect("authentic terminal-parent receipt validates");
    assert_eq!(audit.real_carrier_parent_count, 1);
    assert_eq!(audit.parent_static_validation_count, 1);
    assert_eq!(audit.normalized_forcing_validation_count, 52);
    assert_eq!(audit.dynamic_map_validation_count, 52);
    assert_eq!(audit.map_records.len(), 52);
    assert_eq!(audit.fallback_validation_count, 0);
    assert!(audit.map_records.iter().all(|map| {
        let expected_native_calls = usize::from(map.native_regime);
        map.real_provider_call_count == 1
            && map.real_v8_projection_call_count == 1
            && map.real_native_v3_consumer_count == expected_native_calls
            && map.resident_proof_mint_count == expected_native_calls
            && map.resident_proof_consume_count == expected_native_calls
            && map.normalized_forcing_validation_count == 1
            && map.forcing_proof_mint_count == 1
            && map.forcing_proof_consume_count == 1
            && map.dynamic_map_validation_count == 1
            && map.call_order
                == vec![
                    Stage::CarrierChildAndJointGuards,
                    Stage::Support,
                    Stage::Duration,
                    Stage::Transaction,
                    Stage::NormalizedForcing,
                    Stage::ParentStaticPlanJoinAtV8StructuralValidation,
                    Stage::V8LseAndSurfaceValidation,
                    Stage::V8ForcingProofConsumption,
                    Stage::IngressSchedule,
                    Stage::NativeResidentRevisionJoinIfApplicable,
                    Stage::NativeResidentProofConsumptionIfApplicable,
                    Stage::DynamicProcessValidation,
                    Stage::PhysicalEvaluation,
                    Stage::OutputValidation,
                ]
    }));
}

#[test]
#[cfg(any())]
#[ignore = "contract-first expected red: forced-full versus admitted validation oracle is not implemented"]
fn carrier_validation_once_is_bitwise_equal_for_every_role_path_and_regime() {
    use crate::v9_real_consumer_shadow::v11_covered::{
        carrier_parent_validation_once_parity_for_test,
        CanonicalCoveredPhysicalParityCaseV1 as Case, CarrierValidationMapPathV1 as Path,
        CarrierValidationRoleV1 as Role,
    };

    for case in [
        Case::Ordinary,
        Case::NativeFrozen,
        Case::NativeMixedPhase,
        Case::NativeThawRefreeze,
        Case::NativeWetCanopy,
        Case::NativeMultiOfe,
    ] {
        let parity = carrier_parent_validation_once_parity_for_test(case);
        let native = case != Case::Ordinary;
        assert!(parity.used_real_carrier, "{case:?}");
        assert!(parity.used_real_v8_projection, "{case:?}");
        assert_eq!(parity.used_real_native_v3_consumer, native, "{case:?}");
        assert_eq!(parity.resident_proof_mint_count > 0, native, "{case:?}");
        assert_eq!(parity.resident_proof_consume_count > 0, native, "{case:?}");
        assert_eq!(
            parity.resident_proof_mint_count, parity.resident_proof_consume_count,
            "{case:?}",
        );
        assert_eq!(parity.real_native_physical_count > 0, native, "{case:?}");
        assert_eq!(parity.fallback_validation_count, 0, "{case:?}");
        assert_eq!(
            parity.full_validation_receipt_bytes, parity.admitted_receipt_bytes,
            "{case:?} receipt parity",
        );
        assert_eq!(
            parity.full_validation_final_owner_bytes, parity.admitted_final_owner_bytes,
            "{case:?} final-owner parity",
        );
        assert_eq!(
            parity.full_validation_maps.len(),
            parity.admitted_maps.len()
        );
        let required = parity
            .required_role_paths
            .iter()
            .copied()
            .collect::<std::collections::BTreeSet<_>>();
        let mut observed = std::collections::BTreeSet::new();
        for (full, admitted) in parity
            .full_validation_maps
            .iter()
            .zip(&parity.admitted_maps)
        {
            assert_eq!(full.role, admitted.role, "{case:?}");
            assert_eq!(full.path, admitted.path, "{case:?}");
            assert_eq!(full.call_order, admitted.call_order, "{case:?}");
            assert_eq!(
                full.physical_result_bytes, admitted.physical_result_bytes,
                "{case:?} physical map parity",
            );
            assert_eq!(
                full.final_owner_bytes, admitted.final_owner_bytes,
                "{case:?} per-map owner parity",
            );
            assert!(
                observed.insert((admitted.role, admitted.path)),
                "duplicate authentic {case:?} {:?}/{:?}",
                admitted.role,
                admitted.path,
            );
        }
        assert_eq!(observed, required, "{case:?} applicability matrix");
        for role in [Role::Initial, Role::History, Role::FinalAccepted] {
            assert!(
                required.iter().any(|(seen_role, _)| *seen_role == role),
                "missing required {case:?}/{role:?} coverage",
            );
        }
        assert!(
            required.iter().any(|(_, path)| *path == Path::Direct),
            "missing required {case:?}/Direct coverage",
        );
        if native {
            for path in [Path::Half1, Path::Half2] {
                assert!(
                    required.iter().any(|(_, seen_path)| *seen_path == path),
                    "missing required {case:?}/{path:?} coverage",
                );
            }
        }
    }
}

#[test]
#[cfg(any())]
#[ignore = "contract-first expected red: validation-once poison injection is not implemented"]
fn carrier_validation_once_poisons_keep_first_error_no_fallback_and_rollback() {
    use crate::v9_real_consumer_shadow::v11_covered::{
        carrier_parent_validation_once_competing_poisons_for_test,
        carrier_parent_validation_once_poison_for_test,
        CarrierParentValidationOncePoisonV1 as Poison,
    };

    for poison in [
        Poison::ParentGeneration,
        Poison::StructuralLseConfiguration,
        Poison::StructuralLseState,
        Poison::StructuralSurfaceConfiguration,
        Poison::StructuralSurfaceOwner,
        Poison::NativeResidentLseConfiguration,
        Poison::NativeResidentLseState,
        Poison::NativeResidentSurfaceConfiguration,
        Poison::NativeResidentSurfaceOwner,
        Poison::Topology,
        Poison::StaticIndex,
        Poison::Support,
        Poison::Duration,
        Poison::Transaction,
        Poison::ForcingPointer,
        Poison::SameDigestDifferentForcingAllocation,
        Poison::CompleteJoint,
        Poison::IngressSchedule,
        Poison::ResidentRevision,
        Poison::ProofSecondUse,
        Poison::ProofCrossMapTransfer,
        Poison::ProofCrossParentTransfer,
        Poison::RestartRestoredProof,
        Poison::DynamicVegetationState,
        Poison::DynamicSurfaceState,
        Poison::DynamicSoilHydrologyState,
        Poison::NativeSolverResidual,
        Poison::OutputValidation,
    ] {
        let rejection = carrier_parent_validation_once_poison_for_test(poison);
        assert!(rejection.used_real_carrier_callsite, "{poison:?}");
        assert!(rejection.used_real_v8_or_native_v3_callsite, "{poison:?}");
        assert_eq!(
            rejection.returned_error, rejection.full_validation_error,
            "{poison:?}"
        );
        assert_eq!(
            rejection.returned_error_ordinal, rejection.full_validation_error_ordinal,
            "{poison:?} first-error precedence",
        );
        assert_eq!(rejection.fallback_validation_count, 0, "{poison:?}");
        assert_eq!(rejection.publication_count, 0, "{poison:?}");
        assert!(rejection.rollback_byte_identical, "{poison:?}");
        if poison == Poison::SameDigestDifferentForcingAllocation {
            assert!(rejection.semantic_digest_equal, "same-digest poison setup");
            assert!(!rejection.pointer_identity_equal, "pointer poison setup");
        }
        if poison == Poison::RestartRestoredProof {
            assert!(rejection.restart_ran_full_validation, "restart boundary");
            assert!(
                !rejection.private_proof_restored,
                "private proof is non-wire"
            );
        }
    }

    for (earlier, later) in [
        (Poison::Support, Poison::ParentGeneration),
        (Poison::Duration, Poison::StructuralLseConfiguration),
        (Poison::Transaction, Poison::StaticIndex),
        (
            Poison::CompleteJoint,
            Poison::StructuralSurfaceConfiguration,
        ),
        (Poison::ForcingPointer, Poison::StructuralLseState),
        (Poison::StructuralLseState, Poison::IngressSchedule),
        (Poison::IngressSchedule, Poison::NativeResidentLseState),
        (Poison::NativeResidentSurfaceOwner, Poison::ProofSecondUse),
        (
            Poison::NativeResidentSurfaceOwner,
            Poison::DynamicVegetationState,
        ),
        (Poison::DynamicVegetationState, Poison::DynamicSurfaceState),
        (
            Poison::DynamicSurfaceState,
            Poison::DynamicSoilHydrologyState,
        ),
        (
            Poison::DynamicSoilHydrologyState,
            Poison::NativeSolverResidual,
        ),
        (Poison::NativeSolverResidual, Poison::OutputValidation),
    ] {
        let rejection = carrier_parent_validation_once_competing_poisons_for_test(earlier, later);
        assert!(
            rejection.used_real_carrier_callsite,
            "{earlier:?}/{later:?}"
        );
        assert_eq!(
            rejection.returned_error, rejection.full_validation_error,
            "{earlier:?}/{later:?}",
        );
        assert_eq!(
            rejection.returned_error_ordinal, rejection.full_validation_error_ordinal,
            "{earlier:?}/{later:?} first-error precedence",
        );
        assert_eq!(rejection.triggered_first_poison, earlier);
        assert!(!rejection.triggered_second_poison);
        assert_eq!(rejection.fallback_validation_count, 0);
        assert_eq!(rejection.publication_count, 0);
        assert!(rejection.rollback_byte_identical);
    }
}

#[test]
#[cfg(any())]
fn carrier_validation_once_source_surface_forbids_dynamic_or_wire_caches() {
    let source = [
        include_str!("snow_stage3_v11_terminal_execution.rs"),
        include_str!("snow_stage3_v11_adaptive_execution.rs"),
        include_str!("v11_covered/execution.rs"),
        include_str!("v11_covered/carrier_phase.rs"),
        include_str!("land_surface_energy_shadow/covered_parent_structural_admission.rs"),
        include_str!("land_surface_energy_shadow/strict_v8_endpoint.rs"),
        include_str!("land_surface_energy_shadow/v8_input_projection.rs"),
        include_str!("land_surface_energy_shadow/v3_multitile_adoption.rs"),
        include_str!("v9_real_consumer_shadow/frozen_litter_v3_adoption.rs"),
    ]
    .join("\n");

    for required_private_surface in [
        "CarrierParentStaticValidationPlanV1",
        "ValidatedCarrierForcingV1",
        "ValidatedFrozenLitterResidentMapProofV1",
        "ValidatedFrozenLitterV3ResidentRevisionV1",
        "begin_carrier_parent_validation_once_audit_v1",
        "take_carrier_parent_validation_once_audit_v1",
    ] {
        assert!(
            source.contains(required_private_surface),
            "missing real validation-once surface {required_private_surface}",
        );
    }
    for forbidden in [
        "Arc<DirectV10",
        "Arc < DirectV10",
        "CachedValidatedV8RuntimeInputProjection",
        "CachedProjectedCoveredColumn",
        "CachedSolverReadyTile",
        "CachedHydrologySnapshot",
        "CachedCarrierPhysicalResult",
        "CachedNativeResidentProjection",
        "CachedFrozenLitterResidentValidation",
    ] {
        assert!(
            !source.contains(forbidden),
            "forbidden validation-once cache/wire surface {forbidden}",
        );
    }

    for ephemeral_type in [
        "CarrierParentStaticValidationPlanV1",
        "ValidatedCarrierForcingV1",
        "ValidatedFrozenLitterResidentMapProofV1",
    ] {
        let declaration = format!("struct {ephemeral_type}");
        let declaration_at = source
            .find(&declaration)
            .unwrap_or_else(|| panic!("missing declaration for {ephemeral_type}"));
        let prefix = &source[..declaration_at];
        let prior_type_at = [
            prefix.rfind("\nstruct "),
            prefix.rfind("\npub struct "),
            prefix.rfind("\npub(crate) struct "),
            prefix.rfind("\npub(super) struct "),
        ]
        .into_iter()
        .flatten()
        .max()
        .unwrap_or(0);
        if let Some(derive_at) = prefix.rfind("#[derive(") {
            if derive_at > prior_type_at {
                let derive = &prefix[derive_at..];
                for forbidden_trait in ["Clone", "Serialize", "Deserialize"] {
                    assert!(
                        !derive.contains(forbidden_trait),
                        "{ephemeral_type} must not derive {forbidden_trait}",
                    );
                }
            }
        }
        for forbidden_impl in ["Clone", "serde::Serialize", "serde::Deserialize"] {
            assert!(
                !source.contains(&format!("impl {forbidden_impl} for {ephemeral_type}")),
                "{ephemeral_type} must not implement {forbidden_impl}",
            );
        }
    }
}
