fn v55_q(bits: u64) -> f64 {
    f64::from_bits(bits)
}

fn v55_root(
    q_bits: u64,
    q_out_bits: u64,
    ordinal: usize,
) -> CoveredPhaseConsistentPhysicalEvaluationV1 {
    v54_evaluation(
        &[
            0.327_290_935_6,
            -1_000.0,
            100.0,
            v55_q(q_bits),
            500.0,
            263.2,
        ],
        ordinal,
        5500,
        v55_q(q_out_bits),
    )
}

#[test]
fn v55_private_q_lattice_closes_r140_before_polish() {
    let root_bits = 5_340.494_294_593_43_f64.to_bits();
    let output_bits = root_bits + 3;
    let root = v55_root(root_bits, output_bits, 63);
    let unchanged = root.residual.coordinates.clone();
    let mut budget = CoveredPhysicalEvaluationBudgetV1::new(63).expect("shared budget");
    let mut visited = Vec::new();
    let mut charged_ordinals = Vec::new();
    let polished =
        phase_consistent_coupled_root_polish_v1(root, 1.0, &mut budget, |coordinates, budget| {
            covered_physical_evaluation_budget_charge_v1(budget)?;
            visited.push(coordinates.to_vec());
            charged_ordinals.push(budget.used);
            let bits = coordinates[3].to_bits();
            let physical = if bits == root_bits + 2 {
                bits
            } else {
                bits + 1
            };
            Ok(v54_evaluation(
                coordinates,
                budget.used,
                5500 + u128::from(bits - root_bits),
                v55_q(physical),
            ))
        })
        .expect("first exact representable private Q witness");

    assert_eq!(
        polished.stop,
        CoveredCoupledPolishStopV1::PrivateQLatticeExactWitness
    );
    assert_eq!(
        polished.evaluation.residual.coordinates[3].to_bits(),
        root_bits + 2
    );
    assert_eq!(
        polished.evaluation.residual.r_q_cn_j_m2[0].to_bits(),
        0.0_f64.to_bits()
    );
    assert_eq!(polished.evaluations_used, 3);
    assert_eq!(budget.used, 66);
    assert_eq!(
        visited.len(),
        3,
        "members after the first witness are still evaluated"
    );
    assert_eq!(charged_ordinals, vec![64, 65, 66]);
    assert_eq!(
        polished.evaluation.artifacts.transaction_id,
        TransactionId(5502),
        "retained first witness artifacts belong to its exact coordinate",
    );
    for (ordinal, coordinates) in visited.iter().enumerate() {
        assert_eq!(coordinates[3].to_bits(), root_bits + ordinal as u64 + 1);
        for index in [0, 1, 2, 4, 5] {
            assert_eq!(coordinates[index].to_bits(), unchanged[index].to_bits());
        }
    }
    assert!(!polished.publication_eligible);

    let receipts = v54_receipts(263.2);
    let witness_artifacts = polished.evaluation.artifacts.clone();
    let witness_finalization = polished.evaluation.finalization_inputs.clone();
    let witness_residual = polished.evaluation.residual.clone();
    let stabilized = covered_authentic_receipt_stabilize_v1(
        receipts.clone(),
        &mut budget,
        |kind, input, budget| {
            assert!(kind.requires_authentic_receipts());
            assert!(covered_snow_soil_receipt_sets_exact_v1(input, &receipts));
            v35_authentic_receipt_stabilization_vectors::charged_result(
                budget,
                witness_residual.clone(),
                witness_artifacts.clone(),
                receipts.clone(),
            )
        },
    )
    .expect("exact whole-receipt probe and independent replay");
    assert_eq!(stabilized.stabilization_probe_count, 1);
    assert_eq!(stabilized.independent_replay_count, 1);
    assert_eq!(stabilized.artifacts, witness_artifacts);
    assert_eq!(stabilized.finalization_inputs, witness_finalization);
    assert_eq!(budget.used, 68);
    assert!(!stabilized.publication_eligible);
}

#[test]
fn v55_private_q_lattice_requires_exactly_one_charge_per_candidate() {
    let root_bits = 5_340.494_294_593_43_f64.to_bits();
    let root = v55_root(root_bits, root_bits + 2, 70);
    let mut budget = CoveredPhysicalEvaluationBudgetV1::new(70).expect("shared budget");
    let missing_charge = covered_private_q_lattice_witness_v1(
        &root,
        &mut budget,
        &v54_branch(),
        |coordinates, budget| {
            Ok(v54_evaluation(
                coordinates,
                budget.used,
                5510,
                coordinates[3],
            ))
        },
    );
    assert_eq!(
        missing_charge,
        Err(PhaseConsistentCoupledSolveErrorV1::EvaluationBudget)
    );
    assert_eq!(budget.used, 70);

    let mut budget = CoveredPhysicalEvaluationBudgetV1::new(70).expect("shared budget");
    let double_charge = covered_private_q_lattice_witness_v1(
        &root,
        &mut budget,
        &v54_branch(),
        |coordinates, budget| {
            covered_physical_evaluation_budget_charge_v1(budget)?;
            covered_physical_evaluation_budget_charge_v1(budget)?;
            Ok(v54_evaluation(
                coordinates,
                budget.used,
                5511,
                coordinates[3],
            ))
        },
    );
    assert_eq!(
        double_charge,
        Err(PhaseConsistentCoupledSolveErrorV1::EvaluationBudget)
    );
    assert_eq!(budget.used, 72);
}

#[test]
fn v55_private_q_lattice_descends_deterministically_and_fails_closed_without_witness() {
    let root_bits = 5_340.494_294_593_50_f64.to_bits();
    let output_bits = root_bits - 3;
    let root = v55_root(root_bits, output_bits, 80);
    let root_before = root.clone();
    let mut budget = CoveredPhysicalEvaluationBudgetV1::new(80).expect("shared budget");
    let mut visited = Vec::new();
    let result = covered_private_q_lattice_witness_v1(
        &root,
        &mut budget,
        &v54_branch(),
        |coordinates, budget| {
            covered_physical_evaluation_budget_charge_v1(budget)?;
            visited.push(coordinates[3].to_bits());
            Ok(v54_evaluation(
                coordinates,
                budget.used,
                5502,
                v55_q(coordinates[3].to_bits() - 1),
            ))
        },
    );
    assert_eq!(
        result,
        Err(PhaseConsistentCoupledSolveErrorV1::PrivateQLatticeNoWitness)
    );
    assert_eq!(visited, vec![root_bits - 1, root_bits - 2, root_bits - 3]);
    assert_eq!(budget.used, 83);
    assert_eq!(root_before.residual.coordinates[3].to_bits(), root_bits);
}

#[test]
fn v55_private_q_lattice_preflight_is_atomic_exact_fit_and_one_short() {
    let root_bits = 5_340.494_294_593_43_f64.to_bits();
    let root = v55_root(root_bits, root_bits + 3, 91);
    let mut exact = CoveredPhysicalEvaluationBudgetV1::new(91).expect("shared budget");
    let mut exact_calls = 0usize;
    let result = covered_private_q_lattice_witness_v1(
        &root,
        &mut exact,
        &v54_branch(),
        |coordinates, budget| {
            covered_physical_evaluation_budget_charge_v1(budget)?;
            exact_calls += 1;
            Ok(v54_evaluation(
                coordinates,
                budget.used,
                5503,
                v55_q(coordinates[3].to_bits() + 1),
            ))
        },
    );
    assert_eq!(
        result,
        Err(PhaseConsistentCoupledSolveErrorV1::PrivateQLatticeNoWitness)
    );
    assert_eq!(exact_calls, 3);
    assert_eq!(exact.used, 94, "two authentic charges remain protected");

    let mut short = CoveredPhysicalEvaluationBudgetV1::new(92).expect("shared budget");
    let mut short_calls = 0usize;
    let result = covered_private_q_lattice_witness_v1(&root, &mut short, &v54_branch(), |_, _| {
        short_calls += 1;
        unreachable!("atomic preflight refuses before a physical map")
    });
    assert_eq!(result, Ok(None));
    assert_eq!(short_calls, 0);
    assert_eq!(short.used, 92);
}

#[test]
fn v55_private_q_lattice_refuses_lineage_interval_and_coordinate_poisons() {
    let root_bits = 5_340.494_294_593_43_f64.to_bits();
    let mut stale = v55_root(root_bits, root_bits + 2, 70);
    stale.residual.physical_q_cn_j_m2[0] = v55_q(root_bits + 3);
    let mut budget = CoveredPhysicalEvaluationBudgetV1::new(70).expect("shared budget");
    assert_eq!(
        covered_private_q_lattice_witness_v1(
            &stale,
            &mut budget,
            &v54_branch(),
            |_, _| unreachable!()
        ),
        Err(PhaseConsistentCoupledSolveErrorV1::PrivateQLatticeInterval),
        "canonical endpoint Q must cross-check the already assembled residual",
    );
    assert_eq!(budget.used, 70);

    for poison in [f64::NAN, f64::INFINITY] {
        let mut root = v55_root(root_bits, root_bits + 2, 70);
        root.residual.physical_q_cn_j_m2[0] = poison;
        root.residual.r_q_cn_j_m2[0] = root.residual.coordinates[3] - poison;
        let mut poison_budget = CoveredPhysicalEvaluationBudgetV1::new(70).expect("shared budget");
        assert_eq!(
            covered_private_q_lattice_witness_v1(
                &root,
                &mut poison_budget,
                &v54_branch(),
                |_, _| unreachable!()
            ),
            Err(PhaseConsistentCoupledSolveErrorV1::NonFinite)
        );
        assert_eq!(poison_budget.used, 70);
    }
    for miss in [-0.0, 0.0, -1.0] {
        let mut root = v55_root(root_bits, root_bits + 2, 70);
        root.residual.physical_q_cn_j_m2[0] = miss;
        root.residual.r_q_cn_j_m2[0] = root.residual.coordinates[3] - miss;
        let mut miss_budget = CoveredPhysicalEvaluationBudgetV1::new(70).expect("shared budget");
        assert_eq!(
            covered_private_q_lattice_witness_v1(
                &root,
                &mut miss_budget,
                &v54_branch(),
                |_, _| unreachable!()
            ),
            Ok(None)
        );
        assert_eq!(miss_budget.used, 70);
    }

    let root = v55_root(root_bits, root_bits + 2, 70);
    let mut branch_budget = CoveredPhysicalEvaluationBudgetV1::new(70).expect("shared budget");
    let result = covered_private_q_lattice_witness_v1(
        &root,
        &mut branch_budget,
        &v54_branch(),
        |coordinates, budget| {
            covered_physical_evaluation_budget_charge_v1(budget)?;
            let mut candidate = v54_evaluation(coordinates, budget.used, 5504, coordinates[3]);
            candidate.branch_identity.phase_branch[0] = 1;
            Ok(candidate)
        },
    );
    assert_eq!(
        result,
        Err(PhaseConsistentCoupledSolveErrorV1::SideConstraint)
    );
    assert_eq!(branch_budget.used, 71);
}

#[test]
fn v55_private_q_lattice_refuses_multi_q_and_preserves_authentic_receipt_q() {
    let root_bits = 5_340.494_294_593_43_f64.to_bits();
    let mut multi = v55_root(root_bits, root_bits + 2, 70);
    multi.residual.coordinates = vec![
        0.327_290_935_6,
        -1_000.0,
        100.0,
        v55_q(root_bits),
        0.25,
        -500.0,
        120.0,
        v55_q(root_bits + 10),
        500.0,
        263.2,
    ];
    multi
        .residual
        .r_q_cn_j_m2
        .push(v55_q(root_bits + 10) - v55_q(root_bits + 12));
    multi
        .residual
        .physical_q_cn_j_m2
        .push(v55_q(root_bits + 12));
    let mut budget = CoveredPhysicalEvaluationBudgetV1::new(70).expect("shared budget");
    assert_eq!(
        covered_private_q_lattice_witness_v1(
            &multi,
            &mut budget,
            &v54_branch(),
            |_, _| unreachable!()
        ),
        Ok(None)
    );
    assert_eq!(
        budget.used, 70,
        "multi-Q specialization miss is zero-charge"
    );

    let receipt = &v54_receipts(263.2)[&7];
    let coordinate_q = f64::from_bits(receipt.snow_candidate_heat_j_m2_ofe_ground.to_bits() + 1);
    let trial = CoveredPhaseConsistentCnTrialOperandV1::from_sealed_receipt(receipt, coordinate_q)
        .expect("private coordinate operand");
    let private = covered_phase_consistent_cn_consumption_v1(
        CoveredPhaseConsistentPhysicalEvaluationKindV1::PrivateTrial,
        Some(&trial),
        Some(receipt),
    )
    .expect("private Q consumption");
    assert_eq!(
        private.snow_candidate_heat_j_m2_ofe_ground.to_bits(),
        coordinate_q.to_bits()
    );
    for kind in [
        CoveredPhaseConsistentPhysicalEvaluationKindV1::ReceiptStabilizationProbe,
        CoveredPhaseConsistentPhysicalEvaluationKindV1::ReceiptStabilizationReplay,
    ] {
        let authentic =
            covered_phase_consistent_cn_consumption_v1(kind, Some(&trial), Some(receipt))
                .expect("authentic sealed receipt consumption");
        assert_eq!(
            authentic.snow_candidate_heat_j_m2_ofe_ground.to_bits(),
            receipt.snow_candidate_heat_j_m2_ofe_ground.to_bits(),
        );
        assert_ne!(
            authentic.snow_candidate_heat_j_m2_ofe_ground.to_bits(),
            coordinate_q.to_bits()
        );
    }
}

#[test]
fn v55_r142_overcapacity_is_zero_charge_then_v45_polish_continues() {
    let root_bits = 5_340.494_294_593_43_f64.to_bits();
    let root = v55_root(root_bits, root_bits + 1_394, 30);
    let mut preflight_budget = CoveredPhysicalEvaluationBudgetV1::new(30).expect("shared budget");
    let mut lattice_calls = 0usize;
    assert_eq!(
        covered_private_q_lattice_witness_v1(
            &root,
            &mut preflight_budget,
            &v54_branch(),
            |_, _| {
                lattice_calls += 1;
                unreachable!("R142 interval cannot commit")
            }
        ),
        Ok(None)
    );
    assert_eq!(lattice_calls, 0);
    assert_eq!(preflight_budget.used, 30);

    let mut polish_budget = CoveredPhysicalEvaluationBudgetV1::new(30).expect("shared budget");
    let mut v45_calls = 0usize;
    let result =
        phase_consistent_coupled_root_polish_v1(root, 1.0, &mut polish_budget, |_, budget| {
            covered_physical_evaluation_budget_charge_v1(budget)?;
            v45_calls += 1;
            Err(PhaseConsistentCoupledSolveErrorV1::SideConstraint)
        });
    assert_eq!(
        result,
        Err(PhaseConsistentCoupledSolveErrorV1::SideConstraint)
    );
    assert_eq!(
        v45_calls, 2,
        "unchanged V45 forward/reverse evaluator receives control"
    );
    assert_eq!(polish_budget.used, 32);
}

#[test]
fn v55_hard_shape_and_post_commit_failures_never_fall_back() {
    let root_bits = 5_340.494_294_593_43_f64.to_bits();
    let mut malformed = v55_root(root_bits, root_bits + 2, 70);
    malformed.residual.physical_q_cn_j_m2.clear();
    let mut malformed_budget = CoveredPhysicalEvaluationBudgetV1::new(70).expect("shared budget");
    assert_eq!(
        covered_private_q_lattice_witness_v1(
            &malformed,
            &mut malformed_budget,
            &v54_branch(),
            |_, _| unreachable!()
        ),
        Err(PhaseConsistentCoupledSolveErrorV1::Structure)
    );
    assert_eq!(malformed_budget.used, 70);

    let root = v55_root(root_bits, root_bits + 2, 92);
    let mut committed_budget = CoveredPhysicalEvaluationBudgetV1::new(92).expect("shared budget");
    let mut calls = 0usize;
    let result =
        phase_consistent_coupled_root_polish_v1(root, 1.0, &mut committed_budget, |_, budget| {
            covered_physical_evaluation_budget_charge_v1(budget)?;
            calls += 1;
            Err(PhaseConsistentCoupledSolveErrorV1::SideConstraint)
        });
    assert_eq!(
        result,
        Err(PhaseConsistentCoupledSolveErrorV1::SideConstraint)
    );
    assert_eq!(calls, 1, "post-commit error cannot enter V45 fallback");
    assert_eq!(committed_budget.used, 93);
}

#[test]
fn v55_private_q_lattice_refuses_merit_z_side_and_artifact_custody_poisons() {
    let root_bits = 5_340.494_294_593_43_f64.to_bits();
    let root = v55_root(root_bits, root_bits + 2, 70);
    for poison in 0..4 {
        let mut budget = CoveredPhysicalEvaluationBudgetV1::new(70).expect("shared budget");
        let result = covered_private_q_lattice_witness_v1(
            &root,
            &mut budget,
            &v54_branch(),
            |coordinates, budget| {
                covered_physical_evaluation_budget_charge_v1(budget)?;
                let mut candidate = v54_evaluation(coordinates, budget.used, 5520, coordinates[3]);
                match poison {
                    0 => candidate.residual.scaled_merit = 2.0,
                    1 => {
                        candidate.residual.derived_thickness_closures[0].r_z_m = 2.0e-9;
                        candidate.residual.derived_thickness_closures[0].scaled_merit = 2.0;
                        candidate.residual.derived_constraints_scaled_merit = 2.0;
                        candidate.residual.scaled_merit = 2.0;
                    }
                    2 => candidate.residual.algebraic_side_constraints_satisfied = false,
                    3 => candidate.finalization_inputs.proposed_stage3.clear(),
                    _ => unreachable!(),
                }
                Ok(candidate)
            },
        );
        let expected = if poison == 3 {
            PhaseConsistentCoupledSolveErrorV1::Structure
        } else {
            PhaseConsistentCoupledSolveErrorV1::SideConstraint
        };
        assert_eq!(result, Err(expected));
        assert_eq!(budget.used, 71);
    }
}

#[test]
fn v55_private_q_lattice_contract_and_source_forbid_authentic_repair() {
    let contract = include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../docs/specifications/science-contracts/contracts/SC-SNOWENERGY-001.md"
    ));
    let source = format!(
        "{}\n{}",
        include_str!("phase_consistent_coupled_solve.rs"),
        include_str!("phase_consistent_private_q_lattice.rs")
    );
    for seam in [
        "INV-SNOWENERGY-079",
        "OBL-SNOWENERGY-C-047",
        "canonical reconstructed endpoint receipt",
        "PrivateTrial",
        "Authentic probes/replay consume their supplied sealed receipt Q unchanged",
    ] {
        assert!(contract.contains(seam), "missing V55 authority seam {seam}");
    }
    for seam in [
        "covered_private_q_lattice_witness_v1",
        "physical_q_cn_j_m2",
        "PrivateQLatticeExactWitness",
        "COVERED_ROOT_POLISH_RECEIPT_RESERVE_V1",
    ] {
        assert!(source.contains(seam), "missing V55 production seam {seam}");
    }
    for forbidden in ["nextafter", "receipt_distance", "digest_distance"] {
        assert!(
            !source.contains(forbidden),
            "forbidden V55 repair seam {forbidden}"
        );
    }
    assert!(!CoveredConvergenceAdmissionV1::CoupledAuthentic.admits(false, true, false));
}
