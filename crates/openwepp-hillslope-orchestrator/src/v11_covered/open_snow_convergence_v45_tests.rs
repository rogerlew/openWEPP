fn v45_residual(
    coordinates: [f64; 3],
    residuals: [f64; 3],
    scaled_merit: f64,
    derived_r_z_m: Option<f64>,
) -> CoveredPhaseConsistentResidualEvaluationV1 {
    let derived_thickness_closures = derived_r_z_m
        .map(|r_z_m| CoveredDerivedThicknessClosureV1 {
            proposed_z_m: 0.01 + r_z_m,
            physical_z_m: 0.01,
            r_z_m,
            scaled_merit: r_z_m.abs(),
        })
        .into_iter()
        .collect::<Vec<_>>();
    CoveredPhaseConsistentResidualEvaluationV1 {
        coordinates: coordinates.to_vec(),
        residuals: residuals.to_vec(),
        absolute_tolerances: vec![1.0; 3],
        r_w_kg_m2: vec![residuals[0]],
        r_h_j_m2: vec![residuals[1]],
        r_rho_kg_m3: vec![residuals[2]],
        r_q_cn_j_m2: Vec::new(),
        physical_q_cn_j_m2: Vec::new(),
        derived_thickness_closures,
        r_e_j_m2: Vec::new(),
        r_t_k: Vec::new(),
        scaled_merit,
        derived_constraints_scaled_merit: derived_r_z_m.map_or(0.0, f64::abs),
        algebraic_side_constraints_satisfied: true,
    }
}

fn v45_bundle(
    coordinates: [f64; 3],
    residuals: [f64; 3],
    scaled_merit: f64,
    derived_r_z_m: Option<f64>,
    marker: u128,
    physical_evaluation_ordinal: usize,
) -> CoveredPhaseConsistentPhysicalEvaluationV1 {
    let artifacts = v35_authentic_receipt_stabilization_vectors::artifact(marker);
    let phase = covered_canonical_phase_predicate_v1(coordinates[0], coordinates[1])
        .expect("canonical phase");
    CoveredPhaseConsistentPhysicalEvaluationV1 {
        residual: v45_residual(coordinates, residuals, scaled_merit, derived_r_z_m),
        finalization_inputs: CoveredFinalizationEquivalentReplayInputsV1 {
            proposed_stage3: artifacts.stage3_candidate.clone(),
            proposed_soil: artifacts.soil_candidate.clone(),
            input_covered_boundaries: BTreeMap::new(),
            input_open_boundaries: BTreeMap::new(),
            destination_receipts: BTreeMap::new(),
        },
        artifacts,
        branch_identity: CoveredPhaseConsistentPhysicalBranchIdentityV1 {
            phase_branch: vec![phase],
            density_model_branch: vec![7],
        },
        coordinate_posture: CoveredPhaseConsistentCoordinatePostureV1::EnthalpyPrimaryWithCnHeat,
        physical_evaluation_ordinal,
    }
}

fn v45_target_residual(coordinates: &[f64]) -> [f64; 3] {
    [coordinates[0], coordinates[1] + 1.5, coordinates[2] - 99.5]
}

fn v45_merit(residuals: [f64; 3]) -> f64 {
    residuals.into_iter().map(f64::abs).fold(0.0, f64::max)
}

#[test]
fn v45_private_solve_preserves_polish_probe_and_replay_capacity() {
    let initial = vec![0.5, -1.0, 100.0];
    let mut budget = CoveredPhysicalEvaluationBudgetV1::new(92).expect("shared budget");
    let solve = phase_consistent_coupled_physical_solve_v1(
        initial.clone(),
        &mut budget,
        |coordinates, budget| {
            covered_physical_evaluation_budget_charge_v1(budget)?;
            Ok(v45_bundle(
                [coordinates[0], coordinates[1], coordinates[2]],
                [0.0; 3],
                0.0,
                None,
                1,
                budget.used,
            ))
        },
    )
    .expect("one private root map leaves three charges");
    assert_eq!(budget.used, 93);
    assert_eq!(solve.evaluations_used, 1);
    assert!(!solve.publication_eligible);

    let mut refused_budget = CoveredPhysicalEvaluationBudgetV1::new(93).expect("shared budget");
    let refused = phase_consistent_coupled_physical_solve_v1(
        initial,
        &mut refused_budget,
        |coordinates, budget| {
            covered_physical_evaluation_budget_charge_v1(budget)?;
            Ok(v45_bundle(
                [coordinates[0], coordinates[1], coordinates[2]],
                [0.0; 3],
                0.0,
                None,
                2,
                budget.used,
            ))
        },
    );
    assert_eq!(
        refused,
        Err(PhaseConsistentCoupledSolveErrorV1::EvaluationBudget)
    );
    assert_eq!(refused_budget.used, 93);

    let enlarged = CoveredPhysicalEvaluationBudgetV1 {
        used: 0,
        maximum: COVERED_PHYSICAL_EVALUATION_LIMIT_V1 + 1,
    };
    assert_eq!(
        covered_physical_evaluation_budget_preserve_v1(
            &enlarged,
            COVERED_PRIVATE_SOLVE_POST_ROOT_RESERVE_V1,
        ),
        Err(PhaseConsistentCoupledSolveErrorV1::EvaluationBudget)
    );
}

#[test]
fn v45_root_polish_carries_best_bundle_across_subtolerance_non_descent() {
    let coordinates = [0.5, -1.0, 100.0];
    let initial = v45_bundle(coordinates, [0.5; 3], 0.5, None, 1, 0);
    let initial_artifacts = initial.artifacts.clone();
    let initial_finalization = initial.finalization_inputs.clone();
    let mut budget = CoveredPhysicalEvaluationBudgetV1::new(0).expect("shared budget");
    let mut calls = 0_u128;
    let polished = phase_consistent_coupled_root_polish_v1(
        initial,
        0.25,
        &mut budget,
        |coordinates, budget| {
            covered_physical_evaluation_budget_charge_v1(budget)?;
            calls += 1;
            let residuals = if calls <= 3 {
                v45_target_residual(coordinates)
            } else {
                [0.5; 3]
            };
            Ok(v45_bundle(
                [coordinates[0], coordinates[1], coordinates[2]],
                residuals,
                v45_merit(residuals),
                None,
                calls + 1,
                budget.used,
            ))
        },
    )
    .expect("sub-tolerance representational floor");
    assert_eq!(
        polished.stop,
        CoveredCoupledPolishStopV1::SubToleranceNonDescent
    );
    assert_eq!(polished.evaluation.artifacts, initial_artifacts);
    assert_eq!(
        polished.evaluation.finalization_inputs,
        initial_finalization
    );
    assert_eq!(polished.evaluation.physical_evaluation_ordinal, 0);
    assert!(!polished.publication_eligible);
    assert_eq!(calls, 15, "three probes plus twelve rejected trust trials");
}

#[test]
fn v45_root_polish_admits_strict_descent_and_carries_solver_trust_state() {
    let coordinates = [0.5, -1.0, 100.0];
    let mut budget = CoveredPhysicalEvaluationBudgetV1::new(0).expect("shared budget");
    let mut calls = 0_u128;
    let polished = phase_consistent_coupled_root_polish_v1(
        v45_bundle(coordinates, [0.5; 3], 0.5, None, 10, 0),
        0.25,
        &mut budget,
        |coordinates, budget| {
            covered_physical_evaluation_budget_charge_v1(budget)?;
            calls += 1;
            let residuals = v45_target_residual(coordinates);
            Ok(v45_bundle(
                [coordinates[0], coordinates[1], coordinates[2]],
                residuals,
                v45_merit(residuals),
                None,
                10 + calls,
                budget.used,
            ))
        },
    )
    .expect("strict residual descent");
    assert_eq!(
        polished.stop,
        CoveredCoupledPolishStopV1::ExactResidualVector
    );
    assert_eq!(polished.evaluation.residual.residuals, vec![0.0; 3]);
    assert_eq!(polished.evaluation.physical_evaluation_ordinal, 8);
    assert_eq!(
        calls, 8,
        "the carried quarter-radius requires two shared safeguarded steps"
    );
}

#[test]
fn v45_root_polish_refuses_above_tolerance_stagnation_and_side_poison() {
    let initial = vec![10.0, -10.0, 100.0];
    let mut budget = CoveredPhysicalEvaluationBudgetV1::new(0).expect("shared budget");
    let mut calls = 0_u128;
    let above =
        phase_consistent_coupled_physical_solve_v1(initial, &mut budget, |coordinates, budget| {
            covered_physical_evaluation_budget_charge_v1(budget)?;
            calls += 1;
            let residuals = if calls <= 4 {
                [
                    coordinates[0] - 8.0,
                    coordinates[1] + 12.0,
                    coordinates[2] - 98.0,
                ]
            } else {
                [2.0; 3]
            };
            Ok(v45_bundle(
                [coordinates[0], coordinates[1], coordinates[2]],
                residuals,
                v45_merit(residuals),
                None,
                calls,
                budget.used,
            ))
        });
    assert_eq!(above, Err(PhaseConsistentCoupledSolveErrorV1::NonDescent));

    let mut budget = CoveredPhysicalEvaluationBudgetV1::new(0).expect("shared budget");
    let mut side = v45_bundle([0.5, -1.0, 100.0], [0.5; 3], 0.5, None, 20, 0);
    side.residual.algebraic_side_constraints_satisfied = false;
    let side_poison = phase_consistent_coupled_root_polish_v1(side, 1.0, &mut budget, |_, _| {
        unreachable!("invalid carried root must refuse before evaluation")
    });
    assert_eq!(
        side_poison,
        Err(PhaseConsistentCoupledSolveErrorV1::SideConstraint)
    );
}

#[test]
fn v45_polish_refuses_branch_stale_bundle_singular_and_nonfinite_poisons() {
    let coordinates = [0.5, -1.0, 100.0];
    let initial = v45_bundle(coordinates, [0.5; 3], 0.5, None, 30, 0);

    let mut budget = CoveredPhysicalEvaluationBudgetV1::new(0).expect("shared budget");
    let branch = phase_consistent_coupled_root_polish_v1(
        initial.clone(),
        1.0,
        &mut budget,
        |coordinates, budget| {
            covered_physical_evaluation_budget_charge_v1(budget)?;
            if coordinates[0] < 0.0 {
                return Err(PhaseConsistentCoupledSolveErrorV1::SideConstraint);
            }
            let residuals = v45_target_residual(coordinates);
            let mut trial = v45_bundle(
                [coordinates[0], coordinates[1], coordinates[2]],
                residuals,
                v45_merit(residuals),
                None,
                31,
                budget.used,
            );
            trial.branch_identity.phase_branch[0] = 1;
            Ok(trial)
        },
    );
    assert_eq!(
        branch,
        Err(PhaseConsistentCoupledSolveErrorV1::SideConstraint)
    );

    let mut budget = CoveredPhysicalEvaluationBudgetV1::new(0).expect("shared budget");
    let stale = phase_consistent_coupled_root_polish_v1(
        initial.clone(),
        1.0,
        &mut budget,
        |coordinates, budget| {
            covered_physical_evaluation_budget_charge_v1(budget)?;
            let residuals = v45_target_residual(coordinates);
            Ok(v45_bundle(
                [coordinates[0], coordinates[1], coordinates[2]],
                residuals,
                v45_merit(residuals),
                None,
                32,
                0,
            ))
        },
    );
    assert_eq!(stale, Err(PhaseConsistentCoupledSolveErrorV1::Structure));

    let mut budget = CoveredPhysicalEvaluationBudgetV1::new(0).expect("shared budget");
    let singular = phase_consistent_coupled_root_polish_v1(
        initial,
        1.0,
        &mut budget,
        |coordinates, budget| {
            covered_physical_evaluation_budget_charge_v1(budget)?;
            Ok(v45_bundle(
                [coordinates[0], coordinates[1], coordinates[2]],
                [0.5; 3],
                0.5,
                None,
                33,
                budget.used,
            ))
        },
    );
    assert_eq!(
        singular,
        Err(PhaseConsistentCoupledSolveErrorV1::SingularGeneralizedSystem)
    );

    let mut budget = CoveredPhysicalEvaluationBudgetV1::new(0).expect("shared budget");
    let nonfinite = phase_consistent_coupled_root_polish_v1(
        v45_bundle(coordinates, [f64::NAN, 0.0, 0.0], 0.5, None, 34, 0),
        1.0,
        &mut budget,
        |_, _| unreachable!("nonfinite carried root must refuse before evaluation"),
    );
    assert_eq!(
        nonfinite,
        Err(PhaseConsistentCoupledSolveErrorV1::NonFinite)
    );
}

#[test]
fn v45_polish_stagnation_and_receipt_entry_reserve_are_private_stops() {
    let coordinates = [0.5, -1.0, 100.0];
    let initial = v45_bundle(coordinates, [0.0; 3], 0.5, Some(0.5), 40, 0);
    let mut budget = CoveredPhysicalEvaluationBudgetV1::new(0).expect("shared budget");
    let stagnated = phase_consistent_coupled_root_polish_v1(
        initial,
        1.0,
        &mut budget,
        |coordinates, budget| {
            covered_physical_evaluation_budget_charge_v1(budget)?;
            let residuals = [
                coordinates[0] - 0.5,
                coordinates[1] + 1.0,
                coordinates[2] - 100.0,
            ];
            Ok(v45_bundle(
                [coordinates[0], coordinates[1], coordinates[2]],
                residuals,
                v45_merit(residuals).max(0.5),
                Some(0.5),
                41,
                budget.used,
            ))
        },
    )
    .expect("sub-tolerance zero-direction stop");
    assert_eq!(
        stagnated.stop,
        CoveredCoupledPolishStopV1::SubToleranceStagnation
    );
    assert!(!stagnated.publication_eligible);

    let mut budget = CoveredPhysicalEvaluationBudgetV1::new(93).expect("shared budget");
    let mut calls = 0_usize;
    let reserved = phase_consistent_coupled_root_polish_v1(
        v45_bundle(coordinates, [0.5; 3], 0.5, None, 42, 93),
        1.0,
        &mut budget,
        |coordinates, budget| {
            covered_physical_evaluation_budget_charge_v1(budget)?;
            calls += 1;
            let residuals = v45_target_residual(coordinates);
            Ok(v45_bundle(
                [coordinates[0], coordinates[1], coordinates[2]],
                residuals,
                v45_merit(residuals),
                None,
                42 + calls as u128,
                budget.used,
            ))
        },
    )
    .expect("polish preserves receipt probe and replay");
    assert_eq!(
        reserved.stop,
        CoveredCoupledPolishStopV1::ReceiptEntryReserve
    );
    assert_eq!(budget.used, 93);
    assert_eq!(
        calls, 0,
        "version 46 refuses an incomplete safeguarded step"
    );
}

#[test]
fn v45_nonstable_probe_cannot_consume_protected_replay_slot() {
    let r0 = v35_authentic_receipt_stabilization_vectors::receipt_set(273.0);
    let r1 = v35_authentic_receipt_stabilization_vectors::receipt_set(273.125);
    let mut budget = CoveredPhysicalEvaluationBudgetV1::new(94).expect("shared budget");
    let result =
        covered_authentic_receipt_stabilize_v1(r0.clone(), &mut budget, |_, input, budget| {
            let output = if covered_snow_soil_receipt_sets_exact_v1(input, &r0) {
                r1.clone()
            } else {
                input.clone()
            };
            v35_authentic_receipt_stabilization_vectors::charged_result(
                budget,
                v35_authentic_receipt_stabilization_vectors::residual(0.25),
                v35_authentic_receipt_stabilization_vectors::artifact(45),
                output,
            )
        });
    assert_eq!(
        result,
        Err(PhaseConsistentCoupledSolveErrorV1::EvaluationBudget)
    );
    assert_eq!(budget.used, 95, "the independent replay slot is untouched");
}

#[test]
fn v45_exact_receipt_stabilization_replays_at_shared_budget_96() {
    let receipts = v35_authentic_receipt_stabilization_vectors::receipt_set(273.0);
    let artifact = v35_authentic_receipt_stabilization_vectors::artifact(46);
    let mut budget = CoveredPhysicalEvaluationBudgetV1::new(94).expect("shared budget");
    let stabilized = covered_authentic_receipt_stabilize_v1(
        receipts.clone(),
        &mut budget,
        |_, input, budget| {
            assert!(covered_snow_soil_receipt_sets_exact_v1(input, &receipts));
            v35_authentic_receipt_stabilization_vectors::charged_result(
                budget,
                v35_authentic_receipt_stabilization_vectors::residual(0.25),
                artifact.clone(),
                receipts.clone(),
            )
        },
    )
    .expect("stable probe and protected same-input replay");
    assert_eq!(budget.used, COVERED_PHYSICAL_EVALUATION_LIMIT_V1);
    assert_eq!(stabilized.stabilization_probe_count, 1);
    assert_eq!(stabilized.independent_replay_count, 1);
}

#[test]
fn v45_receipt_replay_refuses_finalization_input_substitution() {
    let receipts = v35_authentic_receipt_stabilization_vectors::receipt_set(273.0);
    let artifact = v35_authentic_receipt_stabilization_vectors::artifact(50);
    let residual = v35_authentic_receipt_stabilization_vectors::residual(0.25);
    let mut budget = CoveredPhysicalEvaluationBudgetV1::new(0).expect("shared budget");
    let mut calls = 0_usize;
    let replay =
        covered_authentic_receipt_stabilize_v1(receipts.clone(), &mut budget, |_, _, budget| {
            covered_physical_evaluation_budget_charge_v1(budget)?;
            calls += 1;
            let mut inputs = CoveredFinalizationEquivalentReplayInputsV1 {
                proposed_stage3: artifact.stage3_candidate.clone(),
                proposed_soil: artifact.soil_candidate.clone(),
                input_covered_boundaries: BTreeMap::new(),
                input_open_boundaries: BTreeMap::new(),
                destination_receipts: BTreeMap::new(),
            };
            if calls == 2 {
                inputs.destination_receipts.insert(
                    (
                        OfeId::try_new("v45-ofe").expect("OFE"),
                        TileId::try_new("v45-tile").expect("tile"),
                    ),
                    Digest32::from_bytes([5; 32]),
                );
            }
            Ok((residual.clone(), artifact.clone(), inputs, receipts.clone()))
        });
    assert_eq!(
        replay,
        Err(PhaseConsistentCoupledSolveErrorV1::ReplayMismatch)
    );
}

#[test]
fn v45_polished_private_root_cannot_admit_or_publish() {
    let coordinates = [0.5, -1.0, 100.0];
    let mut budget = CoveredPhysicalEvaluationBudgetV1::new(0).expect("shared budget");
    let polished = phase_consistent_coupled_root_polish_v1(
        v45_bundle(coordinates, [0.0; 3], 0.0, None, 47, 0),
        1.0,
        &mut budget,
        |_, _| unreachable!("exact carried root needs no additional physical map"),
    )
    .expect("exact private residual root");
    assert_eq!(
        polished.stop,
        CoveredCoupledPolishStopV1::ExactResidualVector
    );
    assert_eq!(budget.used, 0, "the carried root is not reevaluated");
    assert!(!polished.publication_eligible);
    assert!(!CoveredConvergenceAdmissionV1::CoupledAuthentic.admits(false, true, false));
}
