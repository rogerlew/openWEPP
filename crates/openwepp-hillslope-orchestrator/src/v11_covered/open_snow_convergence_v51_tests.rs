fn captured_post_crossing_contraction_windows() -> Vec<CoveredParityMonotoneActiveSetResetV1> {
    let mut windows = one_way_phase_boundary_windows(
        [0.327_290_935_567_678_8; 5],
        [
            -1_801.311_838_185_816_7,
            3_268.648_762_015_633,
            6_234.617_433_731_793,
            5_237.053_780_882_328,
            5_577.484_365_103_41,
        ],
    );
    for window in &mut windows {
        window.physical_evaluation_ordinal += 1;
    }
    windows
}

fn captured_multilane_post_crossing_contraction_windows(
) -> Vec<CoveredParityMonotoneActiveSetResetV1> {
    let second_lane_enthalpy = [10.0_f64, 20.0, 30.0, 40.0, 50.0];
    let mut windows = captured_post_crossing_contraction_windows();
    for (index, window) in windows.iter_mut().enumerate() {
        window.reset.root_coordinates_bits.extend([
            0.2_f64.to_bits(),
            second_lane_enthalpy[index].to_bits(),
        ]);
        window.reset.reset_coordinates_bits.extend([
            0.2_f64.to_bits(),
            second_lane_enthalpy[index + 1].to_bits(),
        ]);
        window.reset.root_branch_predicates.push(1);
        window.reset.reset_branch_predicates.push(1);
    }
    windows
}

#[test]
fn v51_post_crossing_contraction_accepts_exact_r132_chain() {
    let windows = captured_post_crossing_contraction_windows();
    let budget = CoveredPhysicalEvaluationBudgetV1::new(10).expect("shared budget");
    assert_eq!(
        windows
            .iter()
            .map(|window| window.physical_evaluation_ordinal)
            .collect::<Vec<_>>(),
        vec![4, 6, 8, 10],
    );
    let enthalpy_points = std::iter::once(f64::from_bits(
        windows[0].reset.root_coordinates_bits[1],
    ))
    .chain(
        windows
            .iter()
            .map(|window| f64::from_bits(window.reset.reset_coordinates_bits[1])),
    )
    .collect::<Vec<_>>();
    assert_eq!(
        enthalpy_points,
        vec![
            -1_801.311_838_185_816_7,
            3_268.648_762_015_633,
            6_234.617_433_731_793,
            5_237.053_780_882_328,
            5_577.484_365_103_41,
        ],
    );
    assert_eq!(
        std::iter::once(windows[0].reset.root_branch_predicates[0])
            .chain(
                windows
                    .iter()
                    .map(|window| window.reset.reset_branch_predicates[0]),
            )
            .collect::<Vec<_>>(),
        vec![0, 1, 1, 1, 1],
    );
    assert_eq!(
        enthalpy_points
            .windows(2)
            .map(|pair| pair[1] - pair[0])
            .collect::<Vec<_>>(),
        vec![
            5_069.960_600_201_45,
            2_965.968_671_716_160_3,
            -997.563_652_849_465_1,
            340.430_584_221_081_5,
        ],
    );
    assert_eq!(
        covered_one_way_phase_boundary_eligibility_v1(&windows, &budget, 9),
        Err(PhaseConsistentCoupledSolveErrorV1::NonDescent),
    );
    let eligibility =
        covered_one_way_post_crossing_contraction_eligibility_v1(&windows, &budget, 9)
            .expect("exact r132 post-crossing contraction");
    assert_eq!(eligibility.reset_windows_observed, 4);
    assert_eq!(eligibility.canonical_boundary_crossings, 1);
    assert_eq!(
        eligibility.seed_coordinates,
        vec![0.327_290_935_567_678_8, 5_577.484_365_103_41],
    );
    assert!(!eligibility.publication_eligible);
}

#[test]
fn v51_post_crossing_contraction_refuses_pre_crossing_reversal_or_predicate_recross() {
    let budget = CoveredPhysicalEvaluationBudgetV1::new(10).expect("shared budget");
    for enthalpy in [
        [-1_000.0, -2_000.0, 1_000.0, 500.0, 750.0],
        [-1_000.0, 1_000.0, -500.0, 250.0, 125.0],
        [-1_000.0, 40_000.0, 20_000.0, 10_000.0, 15_000.0],
        [-1_000.0, -900.0, -800.0, -700.0, -600.0],
    ] {
        let windows = one_way_phase_boundary_windows([0.1; 5], enthalpy);
        assert!(
            covered_one_way_post_crossing_contraction_eligibility_v1(&windows, &budget, 9)
                .is_err(),
        );
    }
}

#[test]
fn v51_post_crossing_contraction_refuses_noncontracting_equal_nonalternating_or_nonfinite() {
    let budget = CoveredPhysicalEvaluationBudgetV1::new(10).expect("shared budget");
    for enthalpy in [
        [-1_000.0, 1_000.0, 1_900.0, 1_000.0, 2_000.0],
        [-1_000.0, 1_000.0, 1_900.0, 1_000.0, 1_900.0],
        [-1_000.0, 1_000.0, 1_900.0, 1_000.0, 500.0],
        [-1_000.0, 1_000.0, 1_500.0, 900.0, 1_600.0],
        [-1_000.0, 1_000.0, 1_900.0, 1_000.0, 1_000.0],
        [-1_000.0, 1_000.0, 1_500.0, 1_200.0, 1_200.0],
    ] {
        let windows = one_way_phase_boundary_windows([0.1; 5], enthalpy);
        assert!(
            covered_one_way_post_crossing_contraction_eligibility_v1(&windows, &budget, 9)
                .is_err(),
        );
    }
    let mut nonfinite = captured_post_crossing_contraction_windows();
    nonfinite[3].reset.reset_coordinates_bits[1] = f64::NAN.to_bits();
    assert!(
        covered_one_way_post_crossing_contraction_eligibility_v1(&nonfinite, &budget, 9).is_err(),
    );
    let mut raw_owner_nonfinite = captured_post_crossing_contraction_windows();
    raw_owner_nonfinite[2].reset.raw_authentic_continuous_owner_bits[0] = f64::NAN.to_bits();
    assert!(
        covered_one_way_post_crossing_contraction_eligibility_v1(
            &raw_owner_nonfinite,
            &budget,
            9,
        )
        .is_err(),
    );
}

#[test]
fn v51_post_crossing_contraction_retains_water_static_cadence_side_raw_owner_budget_guards() {
    let budget = CoveredPhysicalEvaluationBudgetV1::new(10).expect("shared budget");
    for poison in 0..6 {
        let mut windows = captured_post_crossing_contraction_windows();
        match poison {
            0 => windows[1].reset.reset_coordinates_bits[0] ^= 1,
            1 => windows[2].reset.reset_join_fingerprints[0] ^= 1,
            2 => windows[2].physical_evaluation_ordinal += 1,
            3 => windows[2].reset.opposite_raw_vapor_sides[0] = 1,
            4 => {
                windows[2].reset.raw_authentic_continuous_owner_bits =
                    windows[0].reset.raw_authentic_continuous_owner_bits.clone();
            }
            _ => windows[3].publication_eligible = true,
        }
        assert!(
            covered_one_way_post_crossing_contraction_eligibility_v1(&windows, &budget, 9)
                .is_err(),
            "poison {poison} must refuse",
        );
    }
    let multilane = captured_multilane_post_crossing_contraction_windows();
    assert!(
        covered_one_way_post_crossing_contraction_eligibility_v1(&multilane, &budget, 9).is_ok()
    );
    let mut later_lane_water = multilane.clone();
    later_lane_water[2].reset.reset_coordinates_bits[2] ^= 1;
    assert!(
        covered_one_way_post_crossing_contraction_eligibility_v1(
            &later_lane_water,
            &budget,
            9,
        )
        .is_err(),
    );
    let mut later_lane_predicate = multilane;
    later_lane_predicate[2].reset.reset_branch_predicates[1] = 0;
    assert!(
        covered_one_way_post_crossing_contraction_eligibility_v1(
            &later_lane_predicate,
            &budget,
            9,
        )
        .is_err(),
    );
    let late = CoveredPhysicalEvaluationBudgetV1::new(88).expect("late budget");
    assert_eq!(
        covered_one_way_post_crossing_contraction_eligibility_v1(
            &captured_post_crossing_contraction_windows(),
            &late,
            9,
        ),
        Err(PhaseConsistentCoupledSolveErrorV1::EvaluationBudget),
    );
}

#[test]
fn v51_post_crossing_contraction_dispatch_retains_unchanged_authentic_solver() {
    let budget = CoveredPhysicalEvaluationBudgetV1::new(10).expect("shared budget");
    let eligibility = covered_one_way_post_crossing_contraction_eligibility_v1(
        &captured_post_crossing_contraction_windows(),
        &budget,
        9,
    )
    .expect("V51 trigger eligibility");
    assert!(!eligibility.publication_eligible);
    assert!(!CoveredConvergenceAdmissionV1::Picard.admits(false, true, true));
    assert!(CoveredConvergenceAdmissionV1::CoupledAuthentic.admits(false, true, true));
    assert!(!CoveredConvergenceAdmissionV1::CoupledAuthentic.admits(false, false, true));
}

include!("open_snow_convergence_v52_tests.rs");
