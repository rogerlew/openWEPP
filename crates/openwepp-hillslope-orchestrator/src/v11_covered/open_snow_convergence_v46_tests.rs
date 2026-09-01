fn v46_linear_bundle(
    coordinates: &[f64],
    residuals: &[f64],
    marker: u128,
    ordinal: usize,
) -> CoveredPhaseConsistentPhysicalEvaluationV1 {
    assert_eq!(coordinates.len(), 3);
    assert_eq!(residuals.len(), 3);
    v45_bundle(
        [coordinates[0], coordinates[1], coordinates[2]],
        [residuals[0], residuals[1], residuals[2]],
        residuals.iter().map(|value| value.abs()).fold(0.0, f64::max),
        None,
        marker,
        ordinal,
    )
}

#[test]
fn v46_polish_preflight_refuses_before_partial_jacobian_charge() {
    let coordinates = [0.5, -1.0, 100.0];
    let initial = v45_bundle(coordinates, [0.5; 3], 0.5, None, 460, 91);
    let retained_artifacts = initial.artifacts.clone();
    let retained_finalization = initial.finalization_inputs.clone();
    let mut budget = CoveredPhysicalEvaluationBudgetV1::new(91).expect("shared budget");
    let mut calls = 0_usize;
    let polished = phase_consistent_coupled_root_polish_v1(
        initial,
        1.0,
        &mut budget,
        |_, _| {
            calls += 1;
            unreachable!("incomplete three-column step must not charge")
        },
    )
    .expect("sub-tolerance complete-step reserve stop");
    assert_eq!(polished.stop, CoveredCoupledPolishStopV1::ReceiptEntryReserve);
    assert_eq!(budget.used, 91);
    assert_eq!(calls, 0);
    assert_eq!(polished.evaluation.artifacts, retained_artifacts);
    assert_eq!(polished.evaluation.finalization_inputs, retained_finalization);
    assert!(!polished.publication_eligible);
}

#[test]
fn v46_above_tolerance_preflight_is_typed_budget_failure() {
    let current = v45_bundle(
        [0.5, -1.0, 100.0],
        [2.0; 3],
        2.0,
        None,
        461,
        90,
    );
    let mut budget = CoveredPhysicalEvaluationBudgetV1::new(90).expect("shared budget");
    let mut calls = 0_usize;
    let step = phase_consistent_coupled_safeguarded_step_v1(
        &current,
        1.0,
        &mut budget,
        Some(COVERED_PRIVATE_SOLVE_POST_ROOT_RESERVE_V1),
        false,
        None,
        &mut |_, _| {
            calls += 1;
            unreachable!("above-tolerance incomplete step must not charge")
        },
    );
    assert_eq!(step, Err(PhaseConsistentCoupledSolveErrorV1::EvaluationBudget));
    assert_eq!(budget.used, 90);
    assert_eq!(calls, 0);
}

#[test]
fn v46_exact_complete_step_boundary_runs_unchanged_jacobian_and_trust() {
    let current = v45_bundle(
        [0.5, -1.0, 100.0],
        [0.5; 3],
        0.5,
        None,
        462,
        89,
    );
    let mut budget = CoveredPhysicalEvaluationBudgetV1::new(89).expect("shared budget");
    let mut calls = 0_u128;
    let step = phase_consistent_coupled_safeguarded_step_v1(
        &current,
        1.0,
        &mut budget,
        Some(COVERED_PRIVATE_SOLVE_POST_ROOT_RESERVE_V1),
        false,
        None,
        &mut |coordinates, budget| {
            covered_physical_evaluation_budget_charge_v1(budget)?;
            calls += 1;
            let residuals = v45_target_residual(coordinates);
            Ok(v46_linear_bundle(
                coordinates,
                &residuals,
                462 + calls,
                budget.used,
            ))
        },
    )
    .expect("exact minimum complete-step capacity");
    let CoveredSafeguardedStepV1::Admitted { evaluation, .. } = step else {
        panic!("minimum complete step must admit its strict descent trial");
    };
    assert_eq!(calls, 4, "three columns plus one physical trust trial");
    assert_eq!(budget.used, 93);
    assert_eq!(evaluation.residual.residuals, vec![0.0; 3]);
    assert_eq!(
        budget.maximum - budget.used,
        COVERED_PRIVATE_SOLVE_POST_ROOT_RESERVE_V1
    );
}

#[test]
fn v46_reverse_and_backtrack_maps_retain_per_call_reserve() {
    let current = v45_bundle([2.0, -1.0, 100.0], [0.5; 3], 0.5, None, 463, 87);
    let mut reverse_budget = CoveredPhysicalEvaluationBudgetV1::new(87).expect("shared budget");
    let mut reverse_calls = 0_usize;
    let reverse = phase_consistent_coupled_safeguarded_step_v1(
        &current,
        1.0,
        &mut reverse_budget,
        Some(COVERED_ROOT_POLISH_RECEIPT_RESERVE_V1),
        true,
        None,
        &mut |coordinates, budget| {
            covered_physical_evaluation_budget_charge_v1(budget)?;
            reverse_calls += 1;
            if reverse_calls == 1 {
                return Err(PhaseConsistentCoupledSolveErrorV1::SideConstraint);
            }
            let residuals = [
                coordinates[0] - 1.5,
                coordinates[1] + 1.5,
                coordinates[2] - 99.5,
            ];
            Ok(v46_linear_bundle(
                coordinates,
                &residuals,
                463 + reverse_calls as u128,
                budget.used,
            ))
        },
    )
    .expect("reverse probe keeps per-map receipt reserve");
    assert!(matches!(reverse, CoveredSafeguardedStepV1::Admitted { .. }));
    assert_eq!(reverse_calls, 5, "one failed forward, three usable columns, one trust trial");

    let mut backtrack_budget = CoveredPhysicalEvaluationBudgetV1::new(87).expect("shared budget");
    let mut backtrack_calls = 0_usize;
    let backtrack = phase_consistent_coupled_safeguarded_step_v1(
        &current,
        1.0,
        &mut backtrack_budget,
        Some(COVERED_ROOT_POLISH_RECEIPT_RESERVE_V1),
        true,
        None,
        &mut |coordinates, budget| {
            covered_physical_evaluation_budget_charge_v1(budget)?;
            backtrack_calls += 1;
            if backtrack_calls >= 4 {
                return Err(PhaseConsistentCoupledSolveErrorV1::SideConstraint);
            }
            let residuals = [
                coordinates[0] - 1.5,
                coordinates[1] + 1.5,
                coordinates[2] - 99.5,
            ];
            Ok(v46_linear_bundle(
                coordinates,
                &residuals,
                470 + backtrack_calls as u128,
                budget.used,
            ))
        },
    )
    .expect("rejected trust maps preserve receipt entry capacity");
    assert_eq!(backtrack, CoveredSafeguardedStepV1::ReceiptEntryReserve);
    assert_eq!(backtrack_budget.used, 94);
    assert_eq!(backtrack_calls, 7);
}

#[test]
fn v46_recovered_capacity_runs_evolving_receipt_chain_and_replay_at_96() {
    let receipts = [
        273.0, 273.125, 273.25, 273.375, 273.5, 273.625,
    ]
    .map(v35_authentic_receipt_stabilization_vectors::receipt_set);
    let stable_artifact = v35_authentic_receipt_stabilization_vectors::artifact(480);
    let stable_residual = v35_authentic_receipt_stabilization_vectors::residual(0.25);
    let mut budget = CoveredPhysicalEvaluationBudgetV1::new(89).expect("recovered capacity");
    let mut calls = 0_usize;
    let stabilized = covered_authentic_receipt_stabilize_v1(
        receipts[0].clone(),
        &mut budget,
        |_, input, budget| {
            covered_physical_evaluation_budget_charge_v1(budget)?;
            calls += 1;
            let index = receipts
                .iter()
                .position(|candidate| covered_snow_soil_receipt_sets_exact_v1(candidate, input))
                .expect("canonical receipt in exact chain");
            let output = receipts[(index + 1).min(receipts.len() - 1)].clone();
            let artifact = if index == receipts.len() - 1 {
                stable_artifact.clone()
            } else {
                v35_authentic_receipt_stabilization_vectors::artifact(480 + index as u128)
            };
            Ok((
                stable_residual.clone(),
                artifact.clone(),
                CoveredFinalizationEquivalentReplayInputsV1 {
                    proposed_stage3: artifact.stage3_candidate.clone(),
                    proposed_soil: artifact.soil_candidate.clone(),
                    input_covered_boundaries: BTreeMap::new(),
                    input_open_boundaries: BTreeMap::new(),
                    destination_receipts: BTreeMap::new(),
                },
                output,
            ))
        },
    )
    .expect("five evolving CN endpoints, exact probe, and replay");
    assert_eq!(calls, 7);
    assert_eq!(stabilized.stabilization_probe_count, 6);
    assert_eq!(stabilized.independent_replay_count, 1);
    assert_eq!(budget.used, COVERED_PHYSICAL_EVALUATION_LIMIT_V1);
    assert!(covered_snow_soil_receipt_sets_exact_v1(
        &stabilized.stabilized_receipts,
        &receipts[5]
    ));
    assert!(!stabilized.publication_eligible);
}

#[test]
fn v46_preflight_refuses_malformed_overflow_reset_and_enlarged_budgets() {
    let budget = CoveredPhysicalEvaluationBudgetV1::new(0).expect("shared budget");
    assert_eq!(
        covered_safeguarded_complete_step_capacity_v1(0, &budget, 2),
        Err(PhaseConsistentCoupledSolveErrorV1::Structure)
    );
    assert_eq!(
        covered_safeguarded_complete_step_capacity_v1(usize::MAX, &budget, 2),
        Err(PhaseConsistentCoupledSolveErrorV1::Structure)
    );
    assert_eq!(
        covered_safeguarded_complete_step_capacity_v1(1, &budget, usize::MAX),
        Err(PhaseConsistentCoupledSolveErrorV1::Structure)
    );
    let enlarged = CoveredPhysicalEvaluationBudgetV1 {
        used: 0,
        maximum: COVERED_PHYSICAL_EVALUATION_LIMIT_V1 + 1,
    };
    assert_eq!(
        covered_safeguarded_complete_step_capacity_v1(3, &enlarged, 2),
        Err(PhaseConsistentCoupledSolveErrorV1::EvaluationBudget)
    );
    let reset = CoveredPhysicalEvaluationBudgetV1 {
        used: COVERED_PHYSICAL_EVALUATION_LIMIT_V1 + 1,
        maximum: COVERED_PHYSICAL_EVALUATION_LIMIT_V1,
    };
    assert_eq!(
        covered_safeguarded_complete_step_capacity_v1(3, &reset, 2),
        Err(PhaseConsistentCoupledSolveErrorV1::EvaluationBudget)
    );
}

#[test]
fn v46_dimension_one_five_and_multi_exact_fit_and_one_short_vectors() {
    for (dimension, reserve_after_step) in [(1, 1), (5, 2), (5, 3), (8, 2)] {
        let required = dimension + 1 + reserve_after_step;
        let exact_used = COVERED_PHYSICAL_EVALUATION_LIMIT_V1 - required;
        let exact = CoveredPhysicalEvaluationBudgetV1::new(exact_used).expect("shared budget");
        assert_eq!(
            covered_safeguarded_complete_step_capacity_v1(
                dimension,
                &exact,
                reserve_after_step,
            ),
            Ok(CoveredSafeguardedCompleteStepCapacityV1 {
                dimension,
                minimum_physical_evaluations: dimension + 1,
                reserve_after_step,
            }),
            "exact dimension/reserve boundary must fund one complete step"
        );

        let one_short =
            CoveredPhysicalEvaluationBudgetV1::new(exact_used + 1).expect("shared budget");
        assert_eq!(
            covered_safeguarded_complete_step_capacity_v1(
                dimension,
                &one_short,
                reserve_after_step,
            ),
            Err(PhaseConsistentCoupledSolveErrorV1::EvaluationBudget),
            "one-short capacity must refuse before any physical map"
        );
        assert_eq!(one_short.used, exact_used + 1);
    }

    let canonical_exact = CoveredPhysicalEvaluationBudgetV1::new(88).expect("shared budget");
    assert!(covered_safeguarded_complete_step_capacity_v1(5, &canonical_exact, 2).is_ok());
    let canonical_one_short =
        CoveredPhysicalEvaluationBudgetV1::new(89).expect("shared budget");
    assert_eq!(
        covered_safeguarded_complete_step_capacity_v1(5, &canonical_one_short, 2),
        Err(PhaseConsistentCoupledSolveErrorV1::EvaluationBudget)
    );
    assert_eq!(
        COVERED_PHYSICAL_EVALUATION_LIMIT_V1 - canonical_one_short.used,
        7,
        "the canonical stop preserves six receipt probes plus replay"
    );
}

#[test]
fn v46_partial_or_rejected_artifacts_cannot_admit_or_publish() {
    let current = v45_bundle([0.5, -1.0, 100.0], [0.5; 3], 0.5, None, 490, 91);
    let retained = current.artifacts.clone();
    let mut budget = CoveredPhysicalEvaluationBudgetV1::new(91).expect("shared budget");
    let step = phase_consistent_coupled_safeguarded_step_v1(
        &current,
        1.0,
        &mut budget,
        Some(COVERED_ROOT_POLISH_RECEIPT_RESERVE_V1),
        true,
        Some(&current.branch_identity),
        &mut |_, _| unreachable!("preflight must retain the current bundle"),
    )
    .expect("private reserve transition");
    assert_eq!(step, CoveredSafeguardedStepV1::ReceiptEntryReserve);
    assert_eq!(budget.used, 91);
    assert_eq!(current.artifacts, retained);
    assert!(!CoveredConvergenceAdmissionV1::CoupledAuthentic.admits(false, true, false));
}
