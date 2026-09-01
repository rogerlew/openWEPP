use super::*;
use crate::DirectSnowLayerState;

fn state_for(lane_id: u32) -> DirectSnowStage3PersistentState {
    Wb11HydrologyKernel::initialize_stage3_persistent_state(
        lane_id,
        vec![DirectSnowLayerState::new(0.1, 0.2, 500.0, 3.0)],
    )
    .expect("persistent state")
}

fn state() -> DirectSnowStage3PersistentState {
    state_for(7)
}

fn equal(left: DirectSnowStage3PersistentState, right: DirectSnowStage3PersistentState) -> bool {
    covered_fixed_point_stage3_states_equal(
        &BTreeMap::from([(7, left)]),
        &BTreeMap::from([(7, right)]),
    )
}

fn reseal(state: &mut DirectSnowStage3PersistentState) {
    state.fingerprint = Wb11HydrologyKernel::stage3_persistent_state_fingerprint(state);
}

#[cfg(any())]
mod superseded_v33_coordinate_map_vectors {
    use super::*;

    fn point(x: &[f64], mapped: &[f64], tolerance: &[f64]) -> PhaseConsistentCoupledPointV1 {
        phase_consistent_coupled_point_v1(x.to_vec(), mapped.to_vec(), tolerance.to_vec())
            .expect("finite physical residual point")
    }

    fn scalar_root(previous: (f64, f64), current: (f64, f64)) -> f64 {
        let previous = point(&[previous.0], &[previous.1], &[1.0e-12]);
        let current = point(&[current.0], &[current.1], &[1.0e-12]);
        phase_consistent_coupled_solve_v1(&previous, &current, 94)
            .expect("safeguarded scalar semismooth step")
            .trial_coordinates[0]
    }

    fn project(water: f64, enthalpy: f64) -> (f64, f64, f64, f64) {
        let fusion = crate::hydrology::STAGE3_LATENT_HEAT_FUSION_J_KG;
        if enthalpy < 0.0 {
            (water, 0.0, -enthalpy, 0.0)
        } else if enthalpy < fusion * water {
            let liquid = enthalpy / fusion;
            (water - liquid, liquid, 0.0, 0.0)
        } else {
            (0.0, water, 0.0, (enthalpy - fusion * water).max(0.0))
        }
    }

    fn support(duration_ns: u128, vapor: f64) -> CoveredExactFloorTerminalPhaseSupportImageV1 {
        CoveredExactFloorTerminalPhaseSupportImageV1 {
            parent_start_ns: 0,
            parent_end_ns: duration_ns,
            support_start_ns: 0,
            support_end_ns: duration_ns,
            actual_vapor_kg_m2: vapor,
            deposition_kg_m2: vapor.max(0.0),
            sublimation_kg_m2: (-vapor).max(0.0),
            snowfall_kg_m2: 0.0,
            external_liquid_kg_m2: 0.0,
            complete_energy_j_m2: vapor * 2_834_000.0,
            cold_content_export_j_m2: 0.0,
            ordered_energy_components_j_m2: [0.0, 0.0, 0.0, vapor * 2_834_000.0, 0.0, 0.0, 0.0],
            source_receipt_fingerprints: [1, 2, 3, 4, 5, 6],
        }
    }

    #[test]
    fn v33_exact_60_120_authentic_period_two_invokes_reduced_solve() {
        let a = BTreeMap::from([(7, state())]);
        let mut b_state = state();
        b_state.layers[0].cold_content_j_m2 = 10.0;
        b_state.layers[0].temperature_c =
            Wb11HydrologyKernel::stage3_temperature_from_cold_content_values(
                b_state.layers[0].mass_swe_m,
                b_state.layers[0].cold_content_j_m2,
            );
        reseal(&mut b_state);
        let b = BTreeMap::from([(7, b_state)]);
        for duration in [60_000_000_000, 120_000_000_000] {
            let support_a = BTreeMap::from([(7, support(duration, 1.0e-4))]);
            let support_b = BTreeMap::from([(7, support(duration, -2.0e-4))]);
            assert!(phase_consistent_coupled_exact_authentic_cycle_v1(
                Some(&a),
                Some(&b),
                &a,
                Some(&support_a),
                Some(&support_b),
                &support_a,
            ));
        }
        let root = scalar_root((0.0, 2.0), (2.0, 1.0));
        assert!((root - 4.0 / 3.0).abs() <= 2.0 * f64::EPSILON);
    }

    #[test]
    fn v33_known_root_cold_branch_closes() {
        let root = scalar_root((-2_000.0, 1_000.0), (1_000.0, -500.0));
        let phase = project(0.31, root);
        assert_eq!(root.to_bits(), 0.0_f64.to_bits());
        assert_eq!(phase, (0.31, 0.0, 0.0, 0.0));
        assert_eq!(project(0.31, -1_234.0), (0.31, 0.0, 1_234.0, 0.0));
    }

    #[test]
    fn v33_known_root_mixed_phase_branch_closes() {
        let fusion = crate::hydrology::STAGE3_LATENT_HEAT_FUSION_J_KG;
        let wanted = fusion * 0.12;
        let root = scalar_root((wanted - 2.0, wanted + 2.0), (wanted + 1.0, wanted - 1.0));
        let phase = project(0.31, root);
        assert!((root - wanted).abs() <= 1.0e-9);
        assert!((phase.0 - 0.19).abs() <= f64::EPSILON);
        assert!((phase.1 - 0.12).abs() <= f64::EPSILON);
    }

    #[test]
    fn v33_known_root_fusion_boundary_closes() {
        let wanted = crate::hydrology::STAGE3_LATENT_HEAT_FUSION_J_KG * 0.31;
        let root = scalar_root((wanted - 2.0, wanted + 2.0), (wanted + 1.0, wanted - 1.0));
        assert!((root - wanted).abs() <= 1.0e-9);
        assert_eq!(project(0.31, root), (0.0, 0.31, 0.0, 0.0));
    }

    #[test]
    fn v33_root_is_distinct_from_v31_v32_affine_states() {
        let root = scalar_root((0.0, 2.0), (2.0, 1.0));
        assert_ne!(root.to_bits(), 1.0_f64.to_bits());
        assert_ne!(root.to_bits(), 0.0_f64.to_bits());
    }

    #[test]
    fn v33_coupled_authentic_final_replay_reseals() {
        let root = point(&[0.31, -1_234.0], &[0.31, -1_234.0], &[1.0e-6, 1.0e-6]);
        phase_consistent_coupled_authentic_final_evaluation_v1(&root, &root)
            .expect("fresh authentic evaluation");
        phase_consistent_coupled_authentic_final_replay_reseal_v1(&root, &root)
            .expect("exact independent replay/reseal");
    }

    #[test]
    fn v33_reduced_solve_refuses_poisoned_cycles_and_rolls_back() {
        assert_eq!(
            phase_consistent_coupled_point_v1(vec![f64::NAN], vec![0.0], vec![1.0e-6]),
            Err(PhaseConsistentCoupledSolveErrorV1::Structure),
        );
        let singular_a = point(&[1.0], &[2.0], &[1.0e-6]);
        let singular_b = point(&[1.0], &[3.0], &[1.0e-6]);
        assert_eq!(
            phase_consistent_coupled_solve_v1(&singular_a, &singular_b, 1),
            Err(PhaseConsistentCoupledSolveErrorV1::SingularGeneralizedSystem),
        );
        let expected = point(&[1.0], &[1.0], &[1.0e-6]);
        let poisoned = point(&[1.0], &[1.1], &[1.0e-6]);
        assert_eq!(
            phase_consistent_coupled_authentic_final_evaluation_v1(&expected, &poisoned),
            Err(PhaseConsistentCoupledSolveErrorV1::ReplayMismatch),
        );
    }

    #[test]
    fn v33_reduced_solve_honors_single_96_evaluation_budget() {
        let previous = point(&[0.0], &[2.0], &[1.0e-6]);
        let current = point(&[2.0], &[1.0], &[1.0e-6]);
        assert_eq!(
            phase_consistent_coupled_solve_v1(&previous, &current, 0),
            Err(PhaseConsistentCoupledSolveErrorV1::EvaluationBudget),
        );
        let solve = phase_consistent_coupled_solve_v1(&previous, &current, 94)
            .expect("one trial within cumulative cap");
        assert_eq!(solve.evaluations_used, 1);
        assert!(!solve.publication_eligible);
    }
}

mod v33_phase_consistent_coupled_vectors {
    use super::*;

    fn transition_trace() -> CoveredPhaseConsistentTransitionResetV1 {
        CoveredPhaseConsistentTransitionResetV1 {
            root_join_fingerprints: vec![17, 29, 31, 43],
            reset_join_fingerprints: vec![17, 29, 31, 43],
            root_coordinates_bits: vec![0.31_f64.to_bits(), 0.0_f64.to_bits()],
            reset_coordinates_bits: vec![0.31_f64.to_bits(), 0.0_f64.to_bits()],
            root_branch_predicates: vec![1, 0, 1],
            reset_branch_predicates: vec![1, 0, 1],
            branch_entry_vapor_sides: vec![1],
            opposite_raw_vapor_sides: vec![-1],
            raw_authentic_continuous_owner_bits: vec![
                1.168_754_927_98e-4_f64.to_bits(),
                1.168_754_927_99e-4_f64.to_bits(),
            ],
        }
    }

    fn parity_windows() -> Vec<CoveredParityMonotoneActiveSetResetV1> {
        let roots = [1.0_f64, 1.4, 1.6, 1.7];
        let resets = [1.4_f64, 1.6, 1.7, 1.75];
        roots
            .into_iter()
            .zip(resets)
            .enumerate()
            .map(|(index, (root, reset))| {
                let mut trace = transition_trace();
                trace.root_coordinates_bits = vec![root.to_bits(), 0.0_f64.to_bits()];
                trace.reset_coordinates_bits = vec![reset.to_bits(), 0.0_f64.to_bits()];
                trace.raw_authentic_continuous_owner_bits = vec![
                    (0.2 + index as f64 * 0.01).to_bits(),
                    (10.0 - index as f64 * 0.25).to_bits(),
                ];
                CoveredParityMonotoneActiveSetResetV1 {
                    support_start_ns: 1_860_000_000_000,
                    support_end_ns: 1_920_000_000_000,
                    reset: trace,
                    physical_evaluation_ordinal: 3 + 2 * index,
                    publication_eligible: false,
                }
            })
            .collect()
    }

    fn one_way_phase_boundary_windows(
        water_points: [f64; 5],
        enthalpy_points: [f64; 5],
    ) -> Vec<CoveredParityMonotoneActiveSetResetV1> {
        parity_windows()
            .into_iter()
            .enumerate()
            .map(|(index, mut window)| {
                window.reset.root_coordinates_bits = vec![
                    water_points[index].to_bits(),
                    enthalpy_points[index].to_bits(),
                ];
                window.reset.reset_coordinates_bits = vec![
                    water_points[index + 1].to_bits(),
                    enthalpy_points[index + 1].to_bits(),
                ];
                window.reset.root_branch_predicates = vec![covered_canonical_phase_predicate_v1(
                    water_points[index],
                    enthalpy_points[index],
                )
                .expect("canonical root predicate")];
                window.reset.reset_branch_predicates = vec![covered_canonical_phase_predicate_v1(
                    water_points[index + 1],
                    enthalpy_points[index + 1],
                )
                .expect("canonical reset predicate")];
                window
            })
            .collect()
    }

    fn captured_one_way_phase_boundary_windows() -> Vec<CoveredParityMonotoneActiveSetResetV1> {
        one_way_phase_boundary_windows(
            [0.316_811_3; 5],
            [-3_327.0, -2_445.0, -957.0, 1_454.0, 2_782.0],
        )
    }

    fn residual_inputs(
        coordinates: Vec<f64>,
        physical_delta_water_kg_m2: Vec<f64>,
        physical_complete_energy_j_m2: Vec<f64>,
        physical_soil_delta_energy_j_m2: Vec<f64>,
        owner_soil_temperature_k: Vec<f64>,
    ) -> CoveredPhaseConsistentResidualInputsV1 {
        let physical_ice_kg_m2 = if coordinates[1] <= 0.0 {
            coordinates[0]
        } else {
            coordinates[0] - coordinates[1] / crate::hydrology::STAGE3_LATENT_HEAT_FUSION_J_KG
        };
        let physical_thickness_m = physical_ice_kg_m2 / 120.0;
        CoveredPhaseConsistentResidualInputsV1 {
            coordinates,
            beginning_snow_water_kg_m2: vec![0.25],
            beginning_snow_enthalpy_j_m2: vec![-10.0],
            physical_delta_water_kg_m2,
            physical_complete_energy_j_m2,
            physical_ice_kg_m2: vec![physical_ice_kg_m2],
            physical_density_kg_m3: vec![120.0],
            physical_thickness_m: vec![physical_thickness_m],
            exact_density_settling_branch_satisfied: vec![true],
            beginning_soil_enthalpy_j_m2: vec![100.0],
            physical_soil_delta_energy_j_m2,
            owner_soil_temperature_k,
            absolute_tolerances: vec![1.0e-9, 1.0e-6, 1.0e-6, 1.0e-6, 1.0e-9],
            algebraic_side_constraints_satisfied: true,
        }
    }

    fn coupled_evaluate(
        coordinates: &[f64],
        budget: &mut CoveredPhysicalEvaluationBudgetV1,
    ) -> Result<CoveredPhaseConsistentResidualEvaluationV1, PhaseConsistentCoupledSolveErrorV1>
    {
        let target = [0.5, 20.0, 120.0, 130.0, 270.0];
        covered_phase_consistent_residual_evaluate_v1(
            residual_inputs(
                coordinates.to_vec(),
                vec![target[0] - 0.25 - 0.01 * (coordinates[1] - target[1])],
                vec![target[1] + 10.0 - 2.0 * (coordinates[0] - target[0])],
                vec![target[3] - 100.0 - 0.5 * (coordinates[4] - target[4])],
                vec![target[4] - 0.001 * (coordinates[3] - target[3])],
            ),
            budget,
        )
    }

    #[test]
    fn v33_transition_reset_allows_asymptotically_changing_authentic_owner() {
        let trace = transition_trace();
        assert_ne!(
            trace.raw_authentic_continuous_owner_bits[0],
            trace.raw_authentic_continuous_owner_bits[1]
        );
        assert!(phase_consistent_coupled_active_set_transition_reset_v1(
            &trace
        ));
    }

    #[test]
    fn v33_transition_reset_refuses_join_or_reset_mutation() {
        let mut changed_join = transition_trace();
        changed_join.reset_join_fingerprints[3] ^= 1;
        assert!(!phase_consistent_coupled_active_set_transition_reset_v1(
            &changed_join
        ));
        let mut changed_reset = transition_trace();
        changed_reset.reset_coordinates_bits[1] = (-0.0_f64).to_bits();
        assert!(!phase_consistent_coupled_active_set_transition_reset_v1(
            &changed_reset
        ));
        let mut changed_predicate = transition_trace();
        changed_predicate.reset_branch_predicates[0] = 0;
        assert!(!phase_consistent_coupled_active_set_transition_reset_v1(
            &changed_predicate
        ));
        let mut same_side = transition_trace();
        same_side.opposite_raw_vapor_sides[0] = 1;
        assert!(!phase_consistent_coupled_active_set_transition_reset_v1(
            &same_side
        ));
    }

    #[test]
    fn v33_transition_window_rearms_stale_root_then_dispatches_exact_reset() {
        let mut root_anchor = Some(11_u64);
        let mut branch_entry_seen = true;
        let mut branch_entry_vapor_sides = Some(vec![1]);
        let mut stale_reset = transition_trace();
        stale_reset.reset_coordinates_bits[0] ^= 1;
        assert!(!phase_consistent_coupled_active_set_transition_window_v1(
            &mut root_anchor,
            &mut branch_entry_seen,
            &mut branch_entry_vapor_sides,
            &22,
            &stale_reset,
        ));
        assert_eq!(root_anchor, Some(22));
        assert!(!branch_entry_seen);
        assert_eq!(branch_entry_vapor_sides, None);

        branch_entry_seen = true;
        branch_entry_vapor_sides = Some(vec![-1]);
        assert!(phase_consistent_coupled_active_set_transition_window_v1(
            &mut root_anchor,
            &mut branch_entry_seen,
            &mut branch_entry_vapor_sides,
            &33,
            &transition_trace(),
        ));
        assert_eq!(root_anchor, Some(22));
        assert!(branch_entry_seen);
        assert_eq!(branch_entry_vapor_sides, Some(vec![-1]));
    }

    #[test]
    fn v33_transition_window_never_dispatches_nonexact_reset() {
        for (index, poison) in [0_u8, 1, 2, 3].into_iter().enumerate() {
            let mut trace = transition_trace();
            match poison {
                0 => trace.reset_join_fingerprints[0] ^= 1,
                1 => trace.reset_coordinates_bits[0] ^= 1,
                2 => trace.reset_branch_predicates[0] ^= 1,
                _ => trace.opposite_raw_vapor_sides[0] = 1,
            }
            let mut root_anchor = Some(index as u64);
            let mut branch_entry_seen = true;
            let mut branch_entry_vapor_sides = Some(vec![1]);
            let promoted = 100 + index as u64;
            assert!(!phase_consistent_coupled_active_set_transition_window_v1(
                &mut root_anchor,
                &mut branch_entry_seen,
                &mut branch_entry_vapor_sides,
                &promoted,
                &trace,
            ));
            assert_eq!(root_anchor, Some(promoted));
            assert!(!branch_entry_seen);
            assert_eq!(branch_entry_vapor_sides, None);
        }
    }

    #[test]
    fn v40_parity_monotone_accepts_four_exact_static_decreasing_windows() {
        let windows = parity_windows();
        let mut observed = Vec::new();
        for (index, window) in windows.iter().cloned().enumerate() {
            let budget = CoveredPhysicalEvaluationBudgetV1::new(window.physical_evaluation_ordinal)
                .expect("shared observation budget");
            let eligibility =
                covered_parity_monotone_active_set_observe_v1(&mut observed, window, &budget, 9)
                    .expect("valid rolling observation");
            assert_eq!(eligibility.is_some(), index == 3);
        }
        let budget = CoveredPhysicalEvaluationBudgetV1::new(9).expect("shared budget");
        let eligible = covered_parity_monotone_active_set_eligibility_v1(&windows, &budget, 9)
            .expect("four descending exact-static windows");
        assert_eq!(eligible.reset_windows_observed, 4);
        assert_eq!(eligible.minimum_solver_reserve, 9);
        assert_eq!(eligible.seed_coordinates, vec![1.75, 0.0]);
        assert!(!eligible.publication_eligible);
        let drifts = windows
            .iter()
            .map(|window| {
                covered_parity_monotone_active_set_root_drift_v1(&window.reset)
                    .expect("finite positive drift")
            })
            .collect::<Vec<_>>();
        assert!(drifts.windows(2).all(|pair| pair[1] < pair[0]));
    }

    #[test]
    fn v40_parity_monotone_refuses_nonfinite_stagnation_or_reversal() {
        let budget = CoveredPhysicalEvaluationBudgetV1::new(9).expect("shared budget");
        let mut nonfinite = parity_windows();
        nonfinite[2].reset.reset_coordinates_bits[0] = f64::NAN.to_bits();
        assert!(covered_parity_monotone_active_set_eligibility_v1(&nonfinite, &budget, 9).is_err());
        let mut stagnant = parity_windows();
        stagnant[3].reset.reset_coordinates_bits = stagnant[3].reset.root_coordinates_bits.clone();
        assert!(covered_parity_monotone_active_set_eligibility_v1(&stagnant, &budget, 9).is_err());
        let mut reversed = parity_windows();
        reversed[3].reset.reset_coordinates_bits[0] = 2.2_f64.to_bits();
        assert_eq!(
            covered_parity_monotone_active_set_eligibility_v1(&reversed, &budget, 9),
            Err(PhaseConsistentCoupledSolveErrorV1::NonDescent)
        );
    }

    #[test]
    fn v40_parity_monotone_refuses_static_chain_phase_side_or_cadence_poison() {
        let budget = CoveredPhysicalEvaluationBudgetV1::new(9).expect("shared budget");
        for poison in 0..6 {
            let mut windows = parity_windows();
            match poison {
                0 => windows[2].reset.reset_join_fingerprints[0] ^= 1,
                1 => windows[2].reset.root_coordinates_bits[0] ^= 1,
                2 => windows[2].reset.reset_branch_predicates[0] ^= 1,
                3 => windows[2].reset.opposite_raw_vapor_sides[0] = 1,
                4 => windows[2].physical_evaluation_ordinal += 1,
                _ => {
                    windows[2].reset.raw_authentic_continuous_owner_bits =
                        windows[0].reset.raw_authentic_continuous_owner_bits.clone();
                }
            }
            assert!(
                covered_parity_monotone_active_set_eligibility_v1(&windows, &budget, 9).is_err(),
                "poison {poison} must fail closed"
            );
        }
    }

    #[test]
    fn v40_parity_monotone_requires_shared_budget_reserve_without_publication() {
        let windows = parity_windows();
        let late_budget = CoveredPhysicalEvaluationBudgetV1::new(88).expect("late budget");
        assert_eq!(
            covered_parity_monotone_active_set_eligibility_v1(&windows, &late_budget, 9),
            Err(PhaseConsistentCoupledSolveErrorV1::EvaluationBudget)
        );
        let mut published = windows;
        published[3].publication_eligible = true;
        let budget = CoveredPhysicalEvaluationBudgetV1::new(9).expect("shared budget");
        assert!(covered_parity_monotone_active_set_eligibility_v1(&published, &budget, 9).is_err());
    }

    #[test]
    fn v40_parity_monotone_dispatch_retains_authentic_only_acceptance() {
        let budget = CoveredPhysicalEvaluationBudgetV1::new(9).expect("shared budget");
        let eligible =
            covered_parity_monotone_active_set_eligibility_v1(&parity_windows(), &budget, 9)
                .expect("trigger eligibility");
        assert!(!eligible.publication_eligible);
        assert!(!CoveredConvergenceAdmissionV1::Picard.admits(false, true, true));
        assert!(CoveredConvergenceAdmissionV1::CoupledAuthentic.admits(false, true, true));
        assert!(!CoveredConvergenceAdmissionV1::CoupledAuthentic.admits(false, false, true));
    }

    #[test]
    fn v41_one_way_phase_boundary_accepts_exact_monotone_single_crossing() {
        let windows = captured_one_way_phase_boundary_windows();
        let budget = CoveredPhysicalEvaluationBudgetV1::new(9).expect("shared budget");
        let eligibility = covered_one_way_phase_boundary_eligibility_v1(&windows, &budget, 9)
            .expect("captured one-way canonical boundary bracket");
        assert_eq!(eligibility.reset_windows_observed, 4);
        assert_eq!(eligibility.canonical_boundary_crossings, 1);
        assert_eq!(eligibility.minimum_solver_reserve, 9);
        assert_eq!(eligibility.seed_coordinates, vec![0.316_811_3, 2_782.0]);
        assert!(!eligibility.publication_eligible);
        assert!(
            covered_parity_monotone_active_set_eligibility_v1(&windows, &budget, 9).is_err(),
            "V41 is distinct from the V40 fixed-phase/drift route"
        );
    }

    #[test]
    fn v41_one_way_phase_boundary_refuses_reversal_stagnation_or_multiple_crossing() {
        let budget = CoveredPhysicalEvaluationBudgetV1::new(9).expect("shared budget");
        for enthalpy_points in [
            [-3_327.0, -2_445.0, -957.0, 1_454.0, 500.0],
            [-3_327.0, -2_445.0, -957.0, 1_454.0, 1_454.0],
            [-100.0, 100.0, 40_000.0, 41_000.0, 42_000.0],
            [-5_000.0, -4_000.0, -3_000.0, -2_000.0, -1_000.0],
        ] {
            let windows = one_way_phase_boundary_windows([0.1; 5], enthalpy_points);
            assert!(covered_one_way_phase_boundary_eligibility_v1(&windows, &budget, 9).is_err());
        }
    }

    #[test]
    fn v41_one_way_phase_boundary_refuses_water_drift_join_side_or_cadence_poison() {
        let budget = CoveredPhysicalEvaluationBudgetV1::new(9).expect("shared budget");
        let water = 0.316_811_3_f64;
        let changed_water = f64::from_bits(water.to_bits() + 1);
        let water_drift = one_way_phase_boundary_windows(
            [water, water, changed_water, changed_water, changed_water],
            [-3_327.0, -2_445.0, -957.0, 1_454.0, 2_782.0],
        );
        assert!(covered_one_way_phase_boundary_eligibility_v1(&water_drift, &budget, 9).is_err());
        for poison in 0..4 {
            let mut windows = captured_one_way_phase_boundary_windows();
            match poison {
                0 => windows[2].reset.reset_join_fingerprints[0] ^= 1,
                1 => windows[2].reset.opposite_raw_vapor_sides[0] = 1,
                2 => windows[2].physical_evaluation_ordinal += 1,
                _ => windows[2].reset.root_coordinates_bits[1] ^= 1,
            }
            assert!(
                covered_one_way_phase_boundary_eligibility_v1(&windows, &budget, 9).is_err(),
                "poison {poison} must fail closed"
            );
        }
    }

    #[test]
    fn v41_one_way_phase_boundary_requires_shared_budget_reserve_without_publication() {
        let windows = captured_one_way_phase_boundary_windows();
        let late_budget = CoveredPhysicalEvaluationBudgetV1::new(88).expect("late budget");
        assert_eq!(
            covered_one_way_phase_boundary_eligibility_v1(&windows, &late_budget, 9),
            Err(PhaseConsistentCoupledSolveErrorV1::EvaluationBudget)
        );
        let mut published = windows;
        published[3].publication_eligible = true;
        let budget = CoveredPhysicalEvaluationBudgetV1::new(9).expect("shared budget");
        assert!(covered_one_way_phase_boundary_eligibility_v1(&published, &budget, 9).is_err());
    }

    #[test]
    fn v41_one_way_phase_boundary_dispatch_retains_unchanged_authentic_solver() {
        let budget = CoveredPhysicalEvaluationBudgetV1::new(9).expect("shared budget");
        let eligibility = covered_one_way_phase_boundary_eligibility_v1(
            &captured_one_way_phase_boundary_windows(),
            &budget,
            9,
        )
        .expect("one-way boundary eligibility");
        assert!(!eligibility.publication_eligible);
        assert!(!CoveredConvergenceAdmissionV1::Picard.admits(false, true, true));
        assert!(CoveredConvergenceAdmissionV1::CoupledAuthentic.admits(false, true, true));
        assert!(!CoveredConvergenceAdmissionV1::CoupledAuthentic.admits(false, false, true));
    }

    include!("open_snow_convergence_v51_tests.rs");

    #[test]
    fn v33_physical_residual_evaluator_reconstructs_r_w_r_h_r_e_r_t() {
        let mut budget = CoveredPhysicalEvaluationBudgetV1::new(0).expect("budget");
        let evaluated = covered_phase_consistent_residual_evaluate_v1(
            residual_inputs(
                vec![0.55, 22.0, 120.0, 135.0, 271.0],
                vec![0.20],
                vec![30.0],
                vec![30.0],
                vec![270.0],
            ),
            &mut budget,
        )
        .expect("sealed physical residual");
        assert!((evaluated.r_w_kg_m2[0] - 0.10).abs() <= f64::EPSILON);
        assert_eq!(evaluated.r_h_j_m2, vec![2.0]);
        assert_eq!(evaluated.r_rho_kg_m3, vec![0.0]);
        assert_eq!(evaluated.r_e_j_m2, vec![5.0]);
        assert_eq!(evaluated.r_t_k, vec![1.0]);
        assert_eq!(budget.used, 1);
    }

    #[test]
    fn v33_one_budget_spans_jacobian_rejections_fresh_and_replay() {
        let mut budget = CoveredPhysicalEvaluationBudgetV1::new(0).expect("budget");
        let solve = phase_consistent_coupled_solve_v1(
            vec![0.55, 22.0, 120.0, 135.0, 271.0],
            &mut budget,
            coupled_evaluate,
        )
        .expect("dense safeguarded solve");
        assert!(solve.root.scaled_merit <= 1.0);
        assert!(!solve.publication_eligible);
        let after_solve = budget.used;
        let fresh = phase_consistent_coupled_authentic_final_evaluation_v1(
            &solve.root,
            &mut budget,
            coupled_evaluate,
        )
        .expect("fresh physical root");
        assert_eq!(budget.used, after_solve + 1);
        phase_consistent_coupled_authentic_final_replay_reseal_v1(
            &fresh,
            &mut budget,
            coupled_evaluate,
        )
        .expect("independent replay/reseal");
        assert_eq!(budget.used, after_solve + 2);
        assert_eq!(solve.evaluations_used, after_solve);
    }

    #[test]
    fn v33_coupled_authentic_bypasses_only_picard_equality() {
        assert!(!CoveredConvergenceAdmissionV1::Picard.admits(false, true, true));
        assert!(CoveredConvergenceAdmissionV1::Picard.admits(true, false, false));
        assert!(CoveredConvergenceAdmissionV1::CoupledAuthentic.admits(false, true, true));
        assert!(!CoveredConvergenceAdmissionV1::CoupledAuthentic.admits(false, false, true));
        assert!(!CoveredConvergenceAdmissionV1::CoupledAuthentic.admits(false, true, false));
    }
}

mod v34_stable_monotone_vectors {
    use super::*;

    fn static_joins() -> CoveredStableMonotoneStaticJoinsV1 {
        CoveredStableMonotoneStaticJoinsV1 {
            support_start_ns: 1_800_000_000_000,
            support_end_ns: 1_860_000_000_000,
            source_event_topology_custody: vec![1, 2, 3, 4, 5],
            static_receipt_joins: vec![vec![6, 7, 8], vec![9, 10]],
            phase_branch: vec![1],
            density_model_branch: vec![1, 0, 1, 0],
            carry_authority_and_representation: vec![11, 12, 13, 14],
        }
    }

    fn raw_map(ordinal: usize, merit: f64) -> CoveredStableMonotoneRawAuthenticMapV1 {
        CoveredStableMonotoneRawAuthenticMapV1 {
            static_joins: static_joins(),
            physical_receipt_digests: vec![Digest32::from_bytes(
                [u8::try_from(ordinal).expect("small ordinal"); 32],
            )],
            evolving_carry_coordinate_bits: vec![(
                (1.0e6 + ordinal as f64).to_bits(),
                1,
                format!("{:x}", ordinal + 1),
                -20,
            )],
            residual: CoveredPhaseConsistentResidualEvaluationV1 {
                coordinates: vec![
                    0.3 + ordinal as f64 * 1.0e-4,
                    -20.0,
                    100.0 + ordinal as f64 * 1.0e-3,
                    100.0,
                    270.0,
                ],
                residuals: vec![merit * 1.0e-9, 0.0, 0.0, 0.0, 0.0],
                absolute_tolerances: vec![1.0e-9, 1.0e-6, 1.0e-6, 1.0e-6, 1.0e-9],
                r_w_kg_m2: vec![merit * 1.0e-9],
                r_h_j_m2: vec![0.0],
                r_rho_kg_m3: vec![0.0],
                r_q_cn_j_m2: Vec::new(),
                physical_q_cn_j_m2: Vec::new(),
                derived_thickness_closures: vec![CoveredDerivedThicknessClosureV1 {
                    proposed_z_m: 0.003,
                    physical_z_m: 0.003,
                    r_z_m: 0.0,
                    scaled_merit: 0.0,
                }],
                r_e_j_m2: vec![0.0],
                r_t_k: vec![0.0],
                scaled_merit: merit,
                derived_constraints_scaled_merit: 0.0,
                algebraic_side_constraints_satisfied: true,
            },
            authentic_seed_coordinates: vec![
                0.3 + (ordinal + 1) as f64 * 1.0e-4,
                -19.0,
                100.0 + (ordinal + 1) as f64 * 1.0e-3,
                101.0,
                270.1,
            ],
            physical_evaluation_ordinal: ordinal,
            event_free_terminal_one_volume: true,
            exact_carry_reconstruction_satisfied: true,
            active_set_transition: false,
            finalization_restart: false,
            publication_eligible: false,
        }
    }

    fn charged_trace() -> (
        Vec<CoveredStableMonotoneRawAuthenticMapV1>,
        CoveredPhysicalEvaluationBudgetV1,
    ) {
        let mut budget = CoveredPhysicalEvaluationBudgetV1::new(0).expect("shared budget");
        let mut maps = Vec::new();
        for ordinal in 1..=COVERED_STABLE_MONOTONE_RAW_MAPS_REQUIRED {
            covered_physical_evaluation_budget_charge_v1(&mut budget).expect("raw map charge");
            maps.push(raw_map(ordinal, 20.0 / ordinal as f64));
        }
        (maps, budget)
    }

    #[test]
    fn v34_stable_monotone_eligibility_accepts_exactly_eight_raw_authentic_maps() {
        let (maps, budget) = charged_trace();
        assert!(covered_stable_monotone_solve_eligibility_v1(&maps[..7], &budget).is_err());
        let eligibility = covered_stable_monotone_solve_eligibility_v1(&maps, &budget)
            .expect("exact eight-map eligibility");
        assert_eq!(eligibility.raw_maps_charged, 8);
        assert_eq!(
            eligibility.seed_coordinates,
            maps[7].authentic_seed_coordinates
        );
        assert!(!eligibility.publication_eligible);
    }

    #[test]
    fn v34_stable_monotone_eligibility_allows_physical_receipt_and_carry_coordinate_evolution() {
        let (maps, budget) = charged_trace();
        assert_ne!(
            maps[0].physical_receipt_digests,
            maps[7].physical_receipt_digests
        );
        assert_ne!(
            maps[0].evolving_carry_coordinate_bits,
            maps[7].evolving_carry_coordinate_bits
        );
        assert_eq!(maps[0].static_joins, maps[7].static_joins);
        assert!(maps
            .iter()
            .all(|map| map.exact_carry_reconstruction_satisfied));
        assert!(covered_stable_monotone_solve_eligibility_v1(&maps, &budget).is_ok());
    }

    #[test]
    fn v34_stable_monotone_eligibility_refuses_static_join_phase_or_merit_change() {
        let (maps, budget) = charged_trace();
        for poison in [0_u8, 1, 2] {
            let mut poisoned = maps.clone();
            match poison {
                0 => poisoned[7].static_joins.static_receipt_joins[0][0] ^= 1,
                1 => poisoned[7].static_joins.phase_branch[0] ^= 1,
                _ => poisoned[7].residual.scaled_merit = poisoned[6].residual.scaled_merit,
            }
            assert!(covered_stable_monotone_solve_eligibility_v1(&poisoned, &budget).is_err());
        }
        let mut carry_poison = maps.clone();
        carry_poison[7].exact_carry_reconstruction_satisfied = false;
        assert!(covered_stable_monotone_solve_eligibility_v1(&carry_poison, &budget).is_err());
    }

    #[test]
    fn v34_pre_root_refusal_discards_private_trials_and_resumes_raw_picard() {
        let (maps, budget) = charged_trace();
        let mut trace = Vec::new();
        for map in maps.iter().take(7).cloned() {
            assert!(
                covered_stable_monotone_observe_raw_authentic_map_v1(&mut trace, map, &budget)
                    .is_none()
            );
        }
        let mut refused = maps[7].clone();
        refused.residual.scaled_merit = maps[6].residual.scaled_merit;
        assert!(
            covered_stable_monotone_observe_raw_authentic_map_v1(&mut trace, refused, &budget)
                .is_none()
        );
        assert_eq!(trace.len(), 1, "only the resumed raw map remains");
        let mut disabled = false;
        covered_stable_monotone_disable_after_pre_root_refusal_v1(&mut trace, &mut disabled);
        assert!(trace.is_empty());
        assert!(disabled, "the same support generation cannot re-arm");
        covered_stable_monotone_clear_on_finalization_restart_v1(&mut trace, &mut disabled);
        assert!(
            disabled,
            "a finalization restart disables stable eligibility for this support"
        );
        assert_eq!(budget.used, 8, "fallback cannot reset the shared account");
    }

    #[test]
    fn v34_stable_monotone_uses_existing_shared_physical_evaluation_budget() {
        let (maps, mut budget) = charged_trace();
        let eligibility = covered_stable_monotone_solve_eligibility_v1(&maps, &budget)
            .expect("charged eligibility");
        assert_eq!(eligibility.raw_maps_charged, 8);
        let before = budget.used;
        covered_physical_evaluation_budget_charge_v1(&mut budget).expect("solver evaluation");
        assert_eq!(before, 8);
        assert_eq!(budget.used, 9);
        let mut uncharged = budget.clone();
        uncharged.used = 7;
        assert_eq!(
            covered_stable_monotone_solve_eligibility_v1(&maps, &uncharged),
            Err(PhaseConsistentCoupledSolveErrorV1::EvaluationBudget)
        );
    }

    #[test]
    fn v34_stable_monotone_private_trial_cannot_accept_or_publish() {
        let (maps, budget) = charged_trace();
        let eligibility = covered_stable_monotone_solve_eligibility_v1(&maps, &budget)
            .expect("stable monotone eligibility");
        assert!(!eligibility.publication_eligible);
        assert!(!CoveredConvergenceAdmissionV1::CoupledAuthentic.admits(false, true, false));
        assert!(!CoveredConvergenceAdmissionV1::CoupledAuthentic.admits(false, false, true));
    }
}

#[cfg(test)]
mod v35_authentic_receipt_stabilization_vectors {
    use super::*;

    fn receipt(ending_bottom_snow_temperature_k: f64) -> SnowSoilHeatReceiptV1 {
        let beginning_bottom_snow_temperature_k = 274.0;
        let beginning_top_soil_temperature_k = 272.0;
        let ending_top_soil_temperature_k = 272.0;
        let (beginning, ending, accepted) =
            crate::snow_stage3_v11_attachment::snow_soil_heat_w_m2_ofe_ground(
                0.1,
                0.2,
                0.2,
                0.4,
                beginning_bottom_snow_temperature_k,
                beginning_top_soil_temperature_k,
                ending_bottom_snow_temperature_k,
                ending_top_soil_temperature_k,
            )
            .expect("finite snow-soil heat");
        SnowSoilHeatReceiptV1 {
            schema_version: 1,
            model_identity_sha256: digest_bytes(b"v35-snow-soil-model"),
            support: TimeSupport::new(
                openwepp_coupled_time::ModelTimeNs::new(0),
                openwepp_coupled_time::ModelTimeNs::new(1_800_000_000_000),
            )
            .expect("support"),
            support_duration_ns: 1_800_000_000_000,
            lane_id: 7,
            ofe_id: OfeId::try_new("v35-ofe").expect("OFE"),
            ofe_ground_basis: true,
            topology_identity_sha256: digest_bytes(b"v35-topology"),
            configuration_identity_sha256: digest_bytes(b"v35-configuration"),
            beginning_snow_owner_identity_sha256: digest_bytes(b"v35-beginning-snow"),
            beginning_soil_owner_identity_sha256: digest_bytes(b"v35-beginning-soil"),
            bottom_snow_layer_id: 1,
            first_soil_layer_id: SoilLayerId::try_new("v35-soil-layer").expect("soil layer"),
            bottom_snow_half_thickness_m: 0.1,
            bottom_snow_conductivity_w_m_k: 0.2,
            top_soil_half_thickness_m: 0.2,
            top_soil_conductivity_w_m_k: 0.4,
            beginning_bottom_snow_temperature_k,
            beginning_top_soil_temperature_k,
            ending_bottom_snow_temperature_k,
            ending_top_soil_temperature_k,
            beginning_heat_flux_w_m2_ofe_ground: beginning,
            ending_heat_flux_w_m2_ofe_ground: ending,
            accepted_heat_flux_w_m2_ofe_ground: accepted,
            accepted_heat_j_m2_ofe_ground: accepted * 1_800.0,
            snow_candidate_heat_j_m2_ofe_ground: -accepted * 1_800.0,
            soil_candidate_heat_j_m2_ofe_ground: accepted * 1_800.0,
            snow_candidate_ending_identity_sha256: digest_bytes(b"v35-ending-snow"),
            soil_candidate_ending_identity_sha256: digest_bytes(b"v35-ending-soil"),
            receipt_sha256: Digest32::zero(),
        }
        .seal()
        .expect("sealed receipt")
    }

    pub(super) fn receipt_set(
        ending_bottom_snow_temperature_k: f64,
    ) -> BTreeMap<u32, SnowSoilHeatReceiptV1> {
        BTreeMap::from([(7, receipt(ending_bottom_snow_temperature_k))])
    }

    pub(super) fn residual(merit: f64) -> CoveredPhaseConsistentResidualEvaluationV1 {
        CoveredPhaseConsistentResidualEvaluationV1 {
            coordinates: vec![0.3, -20.0, 100.0, 100.0, 270.0],
            residuals: vec![merit * 1.0e-9, 0.0, 0.0, 0.0, 0.0],
            absolute_tolerances: vec![1.0e-9, 1.0e-6, 1.0e-6, 1.0e-6, 1.0e-9],
            r_w_kg_m2: vec![merit * 1.0e-9],
            r_h_j_m2: vec![0.0],
            r_rho_kg_m3: vec![0.0],
            r_q_cn_j_m2: Vec::new(),
            physical_q_cn_j_m2: Vec::new(),
            derived_thickness_closures: vec![CoveredDerivedThicknessClosureV1 {
                proposed_z_m: 0.003,
                physical_z_m: 0.003,
                r_z_m: 0.0,
                scaled_merit: 0.0,
            }],
            r_e_j_m2: vec![0.0],
            r_t_k: vec![0.0],
            scaled_merit: merit,
            derived_constraints_scaled_merit: 0.0,
            algebraic_side_constraints_satisfied: true,
        }
    }

    pub(super) fn artifact(marker: u128) -> CoveredPhaseConsistentPhysicalArtifactsV1 {
        let soil_candidate = DirectSoilThermalCandidate::V1(SoilThermalSnapshot {
            owner_id: ResourceOwnerId::try_new("v35-soil-owner").expect("owner"),
            configuration_sha256: Sha256Digest::try_new("5".repeat(64)).expect("digest"),
            state_sha256: Sha256Digest::try_new("4".repeat(64)).expect("digest"),
            snapshot_sha256: Sha256Digest::try_new("6".repeat(64)).expect("digest"),
            last_accepted_transaction_id: None,
            ofes: vec![SoilThermalOfeSnapshot {
                ofe_id: OfeId::try_new("v35-ofe").expect("OFE"),
                ordered_layers: vec![SoilThermalLayerSnapshot {
                    layer_id: SoilLayerId::try_new("v35-soil-layer").expect("layer"),
                    temperature_k: 272.0,
                    enthalpy_j_m2_ofe_ground: 1.0e6,
                }],
            }],
        });
        CoveredPhaseConsistentPhysicalArtifactsV1 {
            stage3_candidate: BTreeMap::from([(7, state())]),
            stage3_support_images: BTreeMap::new(),
            corrected_boundaries: BTreeMap::new(),
            lse_states: BTreeMap::new(),
            precipitation_sets: BTreeMap::new(),
            transaction_id: TransactionId(marker),
            soil_candidates: Vec::new(),
            soil_candidate,
            cn_trial_operands: BTreeMap::new(),
            snow_enthalpy_material_owner: None,
        }
    }

    pub(super) fn charged_result(
        budget: &mut CoveredPhysicalEvaluationBudgetV1,
        residual: CoveredPhaseConsistentResidualEvaluationV1,
        artifacts: CoveredPhaseConsistentPhysicalArtifactsV1,
        receipts: BTreeMap<u32, SnowSoilHeatReceiptV1>,
    ) -> Result<
        (
            CoveredPhaseConsistentResidualEvaluationV1,
            CoveredPhaseConsistentPhysicalArtifactsV1,
            CoveredFinalizationEquivalentReplayInputsV1,
            BTreeMap<u32, SnowSoilHeatReceiptV1>,
        ),
        PhaseConsistentCoupledSolveErrorV1,
    > {
        covered_physical_evaluation_budget_charge_v1(budget)?;
        let finalization_inputs = CoveredFinalizationEquivalentReplayInputsV1 {
            proposed_stage3: artifacts.stage3_candidate.clone(),
            proposed_soil: artifacts.soil_candidate.clone(),
            input_covered_boundaries: BTreeMap::new(),
            input_open_boundaries: BTreeMap::new(),
            destination_receipts: BTreeMap::new(),
        };
        Ok((residual, artifacts, finalization_inputs, receipts))
    }

    #[test]
    fn v35_receipt_stabilization_feeds_reconstructed_output_as_next_immutable_input() {
        let r0 = receipt_set(273.0);
        let r1 = receipt_set(273.125);
        let mut observed = Vec::new();
        let mut budget = CoveredPhysicalEvaluationBudgetV1::new(8).expect("shared budget");
        let stabilized = covered_authentic_receipt_stabilize_v1(
            r0.clone(),
            &mut budget,
            |kind, input, budget| {
                observed.push((kind, input.clone()));
                if covered_snow_soil_receipt_sets_exact_v1(input, &r0) {
                    charged_result(budget, residual(0.5), artifact(1), r1.clone())
                } else {
                    charged_result(budget, residual(0.25), artifact(2), r1.clone())
                }
            },
        )
        .expect("exact receipt stabilization and replay");
        assert_eq!(observed.len(), 3);
        assert_eq!(
            observed[0].0,
            CoveredPhaseConsistentPhysicalEvaluationKindV1::ReceiptStabilizationProbe
        );
        assert!(covered_snow_soil_receipt_sets_exact_v1(&observed[0].1, &r0));
        assert!(covered_snow_soil_receipt_sets_exact_v1(&observed[1].1, &r1));
        assert_eq!(
            observed[2].0,
            CoveredPhaseConsistentPhysicalEvaluationKindV1::ReceiptStabilizationReplay
        );
        assert!(covered_snow_soil_receipt_sets_exact_v1(&observed[2].1, &r1));
        assert_eq!(stabilized.stabilization_probe_count, 2);
        assert_eq!(stabilized.independent_replay_count, 1);
        assert_eq!(budget.used, 11);
        assert!(!stabilized.publication_eligible);
    }

    #[test]
    fn v35_first_root_reseal_is_probe_not_cross_input_replay() {
        let r0 = receipt_set(273.0);
        let r1 = receipt_set(273.25);
        let mut calls = 0usize;
        let mut budget = CoveredPhysicalEvaluationBudgetV1::new(8).expect("shared budget");
        let stabilized = covered_authentic_receipt_stabilize_v1(
            r0,
            &mut budget,
            |kind, _input, budget| {
                calls += 1;
                match calls {
                    1 => {
                        assert_eq!(kind, CoveredPhaseConsistentPhysicalEvaluationKindV1::ReceiptStabilizationProbe);
                        charged_result(budget, residual(0.75), artifact(41), r1.clone())
                    }
                    _ => charged_result(budget, residual(0.25), artifact(42), r1.clone()),
                }
            },
        )
        .expect("first cross-input image is not replay-compared");
        assert_eq!(calls, 3);
        assert_eq!(stabilized.artifacts.transaction_id, TransactionId(42));
    }

    #[test]
    fn v35_receipt_stabilization_requires_exact_input_output_receipts() {
        let r0 = receipt_set(273.0);
        let r1 = receipt_set(f64::from_bits(273.0_f64.to_bits() + 1));
        assert!(!covered_snow_soil_receipt_sets_exact_v1(&r0, &r1));
        let mut inputs = Vec::new();
        let mut budget = CoveredPhysicalEvaluationBudgetV1::new(8).expect("shared budget");
        covered_authentic_receipt_stabilize_v1(r0.clone(), &mut budget, |_, input, budget| {
            inputs.push(input.clone());
            charged_result(budget, residual(0.25), artifact(1), r1.clone())
        })
        .expect("one-bit receipt evolution stabilizes only on the next input");
        assert_eq!(inputs.len(), 3);
        assert!(covered_snow_soil_receipt_sets_exact_v1(&inputs[0], &r0));
        assert!(covered_snow_soil_receipt_sets_exact_v1(&inputs[1], &r1));
    }

    #[test]
    fn v35_same_input_replay_requires_exact_residual_artifact_and_receipt_equality() {
        for poison in 0_u8..3 {
            let receipts = receipt_set(273.0);
            let other_receipts = receipt_set(273.125);
            let mut calls = 0usize;
            let mut budget = CoveredPhysicalEvaluationBudgetV1::new(8).expect("shared budget");
            let result = covered_authentic_receipt_stabilize_v1(
                receipts.clone(),
                &mut budget,
                |kind, _, budget| {
                    calls += 1;
                    let is_replay = kind
                        == CoveredPhaseConsistentPhysicalEvaluationKindV1::ReceiptStabilizationReplay;
                    let replay_residual = if is_replay && poison == 0 {
                        residual(f64::from_bits(0.25_f64.to_bits() + 1))
                    } else {
                        residual(0.25)
                    };
                    let replay_artifact = artifact(if is_replay && poison == 1 { 2 } else { 1 });
                    let replay_receipts = if is_replay && poison == 2 {
                        other_receipts.clone()
                    } else {
                        receipts.clone()
                    };
                    charged_result(budget, replay_residual, replay_artifact, replay_receipts)
                },
            );
            assert_eq!(
                result,
                Err(PhaseConsistentCoupledSolveErrorV1::ReplayMismatch)
            );
            assert_eq!(calls, 2, "one stable probe plus exactly one replay");
        }
    }

    #[test]
    fn v35_receipt_oscillation_nonfinite_constraint_budget_discards_probe_artifacts() {
        let r0 = receipt_set(273.0);
        let r1 = receipt_set(273.125);
        let mut calls = 0usize;
        let mut budget = CoveredPhysicalEvaluationBudgetV1::new(8).expect("shared budget");
        let oscillation =
            covered_authentic_receipt_stabilize_v1(r0.clone(), &mut budget, |_, _, budget| {
                calls += 1;
                let output = if calls == 1 { r1.clone() } else { r0.clone() };
                charged_result(budget, residual(0.25), artifact(calls as u128), output)
            });
        assert_eq!(
            oscillation,
            Err(PhaseConsistentCoupledSolveErrorV1::ReceiptOscillation)
        );

        let mut budget = CoveredPhysicalEvaluationBudgetV1::new(8).expect("shared budget");
        let nonfinite =
            covered_authentic_receipt_stabilize_v1(r0.clone(), &mut budget, |_, _, budget| {
                charged_result(budget, residual(f64::NAN), artifact(1), r0.clone())
            });
        assert_eq!(
            nonfinite,
            Err(PhaseConsistentCoupledSolveErrorV1::NonFinite)
        );

        let mut budget = CoveredPhysicalEvaluationBudgetV1::new(8).expect("shared budget");
        let side_constraint =
            covered_authentic_receipt_stabilize_v1(r0.clone(), &mut budget, |_, _, budget| {
                let mut constrained = residual(0.25);
                constrained.algebraic_side_constraints_satisfied = false;
                charged_result(budget, constrained, artifact(1), r0.clone())
            });
        assert_eq!(
            side_constraint,
            Err(PhaseConsistentCoupledSolveErrorV1::SideConstraint)
        );

        let mut budget =
            CoveredPhysicalEvaluationBudgetV1::new(COVERED_PHYSICAL_EVALUATION_LIMIT_V1)
                .expect("exhausted shared budget");
        let exhausted =
            covered_authentic_receipt_stabilize_v1(r0.clone(), &mut budget, |_, _, budget| {
                charged_result(budget, residual(0.25), artifact(1), r0.clone())
            });
        assert_eq!(
            exhausted,
            Err(PhaseConsistentCoupledSolveErrorV1::EvaluationBudget)
        );
    }
}

mod v36_geometry_complete_vectors {
    use super::*;

    fn geometry(
        water_kg_m2: f64,
        enthalpy_j_m2: f64,
        density_kg_m3: f64,
    ) -> CoveredTerminalDensityGeometryCoordinateV1 {
        let phase = phase_consistent_canonical_phase_projection_v1(
            water_kg_m2,
            enthalpy_j_m2,
            density_kg_m3,
        )
        .expect("canonical phase");
        CoveredTerminalDensityGeometryCoordinateV1::from_canonical_phase(&phase)
            .expect("canonical geometry")
    }

    fn raw_map(
        ordinal: usize,
        merit: f64,
        density_kg_m3: f64,
    ) -> CoveredStableMonotoneRawAuthenticMapV1 {
        CoveredStableMonotoneRawAuthenticMapV1 {
            static_joins: CoveredStableMonotoneStaticJoinsV1 {
                support_start_ns: 1_800_000_000_000,
                support_end_ns: 1_860_000_000_000,
                source_event_topology_custody: vec![1, 2, 3],
                static_receipt_joins: vec![vec![4, 5]],
                phase_branch: vec![1, 0, 0, 0, 0, 0, 0, 240],
                density_model_branch: vec![1, 0, 1, 0],
                carry_authority_and_representation: vec![6, 7],
            },
            physical_receipt_digests: vec![Digest32::from_bytes([ordinal as u8; 32])],
            evolving_carry_coordinate_bits: vec![(
                (100.0 + ordinal as f64).to_bits(),
                1,
                format!("{:x}", ordinal + 1),
                -10,
            )],
            residual: CoveredPhaseConsistentResidualEvaluationV1 {
                coordinates: vec![0.31, -1.0, density_kg_m3],
                residuals: vec![merit, 0.0, 0.0],
                absolute_tolerances: vec![1.0, 1.0, 1.0],
                r_w_kg_m2: vec![merit],
                r_h_j_m2: vec![0.0],
                r_rho_kg_m3: vec![0.0],
                r_q_cn_j_m2: Vec::new(),
                physical_q_cn_j_m2: Vec::new(),
                derived_thickness_closures: vec![CoveredDerivedThicknessClosureV1 {
                    proposed_z_m: 0.31 / density_kg_m3,
                    physical_z_m: 0.31 / density_kg_m3,
                    r_z_m: 0.0,
                    scaled_merit: 0.0,
                }],
                r_e_j_m2: vec![],
                r_t_k: vec![],
                scaled_merit: merit,
                derived_constraints_scaled_merit: 0.0,
                algebraic_side_constraints_satisfied: true,
            },
            authentic_seed_coordinates: vec![0.31, -1.0, density_kg_m3 + 0.01],
            physical_evaluation_ordinal: ordinal,
            event_free_terminal_one_volume: true,
            exact_carry_reconstruction_satisfied: true,
            active_set_transition: false,
            finalization_restart: false,
            publication_eligible: false,
        }
    }

    #[test]
    fn v36_geometry_complete_solver_reconstructs_thickness_from_mass_and_density() {
        let captured_left_z = f64::from_bits(4_569_208_177_783_694_401);
        let captured_right_z = f64::from_bits(4_569_208_162_027_237_604);
        let ice_1_kg_m2 = captured_right_z * 100.0;
        let initial_rho = ice_1_kg_m2 / captured_left_z;
        let physical_rho = ice_1_kg_m2 / captured_right_z;
        let mut budget = CoveredPhysicalEvaluationBudgetV1::new(8).expect("shared budget");
        let solve = phase_consistent_coupled_solve_v1(
            vec![ice_1_kg_m2, -1.0, initial_rho],
            &mut budget,
            |coordinates, budget| {
                covered_phase_consistent_residual_evaluate_v1(
                    CoveredPhaseConsistentResidualInputsV1 {
                        coordinates: coordinates.to_vec(),
                        beginning_snow_water_kg_m2: vec![ice_1_kg_m2],
                        beginning_snow_enthalpy_j_m2: vec![-1.0],
                        physical_delta_water_kg_m2: vec![0.0],
                        physical_complete_energy_j_m2: vec![0.0],
                        physical_ice_kg_m2: vec![ice_1_kg_m2],
                        physical_density_kg_m3: vec![physical_rho],
                        physical_thickness_m: vec![captured_right_z],
                        exact_density_settling_branch_satisfied: vec![true],
                        beginning_soil_enthalpy_j_m2: vec![],
                        physical_soil_delta_energy_j_m2: vec![],
                        owner_soil_temperature_k: vec![],
                        absolute_tolerances: vec![1.0e-6, 1.0e-6, 1.0e-8],
                        algebraic_side_constraints_satisfied: true,
                    },
                    budget,
                )
            },
        )
        .expect("geometry-complete generalized physical solve");
        let initial = geometry(ice_1_kg_m2, -1.0, initial_rho);
        let ending = geometry(
            solve.root.coordinates[0],
            solve.root.coordinates[1],
            solve.root.coordinates[2],
        );
        let physical = geometry(ice_1_kg_m2, -1.0, physical_rho);
        assert_eq!(initial.z_1_m.to_bits(), captured_left_z.to_bits());
        assert_eq!(physical.z_1_m.to_bits(), captured_right_z.to_bits());
        assert!((ending.z_1_m - physical.z_1_m).abs() <= COVERED_FIXED_POINT_POLICY.depth_abs_m);
        assert!(((captured_left_z - captured_right_z) - 6.833_273_876e-9).abs() <= 1.0e-18);
        assert!(solve.root.scaled_merit <= 1.0);
        assert!(!solve.publication_eligible);
    }

    #[test]
    fn v36_geometry_complete_solver_evaluates_physical_r_rho_not_map_difference() {
        let coordinate = geometry(0.31, -1.0, 120.0);
        let physical_density = 119.875;
        let r_rho_kg_m3 = covered_terminal_density_geometry_residual_evaluate_v1(
            coordinate,
            physical_density,
            true,
        )
        .expect("physical Stage-3 density residual");
        assert_eq!(
            r_rho_kg_m3.to_bits(),
            (120.0_f64 - physical_density).to_bits()
        );
    }

    #[test]
    fn v36_stable_eligibility_preserves_density_branch_while_rho_and_z_evolve() {
        let maps = (1..=COVERED_STABLE_MONOTONE_RAW_MAPS_REQUIRED)
            .map(|ordinal| raw_map(ordinal, 9.0 - ordinal as f64, 100.0 + ordinal as f64))
            .collect::<Vec<_>>();
        let mut budget = CoveredPhysicalEvaluationBudgetV1::new(0).expect("shared budget");
        for _ in &maps {
            covered_physical_evaluation_budget_charge_v1(&mut budget).expect("charged raw map");
        }
        let eligibility = covered_stable_monotone_solve_eligibility_v1(&maps, &budget)
            .expect("stable density branch");
        assert_eq!(eligibility.seed_coordinates[2], 108.01);
        assert_ne!(
            geometry(0.31, -1.0, 101.0).z_1_m.to_bits(),
            geometry(0.31, -1.0, 108.0).z_1_m.to_bits()
        );
        let mut poisoned = maps;
        poisoned[7].static_joins.density_model_branch[3] ^= 1;
        assert_eq!(
            covered_stable_monotone_solve_eligibility_v1(&poisoned, &budget),
            Err(PhaseConsistentCoupledSolveErrorV1::SideConstraint)
        );
    }

    #[test]
    fn v36_geometry_physics_charges_shared_budget_and_retains_v35_stabilization() {
        let mut budget = CoveredPhysicalEvaluationBudgetV1::new(8).expect("shared budget");
        let input = CoveredPhaseConsistentResidualInputsV1 {
            coordinates: vec![0.31, -1.0, 100.0],
            beginning_snow_water_kg_m2: vec![0.30],
            beginning_snow_enthalpy_j_m2: vec![-2.0],
            physical_delta_water_kg_m2: vec![0.01],
            physical_complete_energy_j_m2: vec![1.0],
            physical_ice_kg_m2: vec![0.31],
            physical_density_kg_m3: vec![100.0],
            physical_thickness_m: vec![0.0031],
            exact_density_settling_branch_satisfied: vec![true],
            beginning_soil_enthalpy_j_m2: vec![],
            physical_soil_delta_energy_j_m2: vec![],
            owner_soil_temperature_k: vec![],
            absolute_tolerances: vec![1.0e-6, 1.0e-6, 1.0e-6],
            algebraic_side_constraints_satisfied: true,
        };
        let evaluated = covered_phase_consistent_residual_evaluate_v1(input, &mut budget)
            .expect("charged geometry physics");
        assert_eq!(budget.used, 9);
        assert_eq!(evaluated.r_rho_kg_m3, vec![0.0]);
        let receipts = super::v35_authentic_receipt_stabilization_vectors::receipt_set(273.0);
        let stabilized = covered_authentic_receipt_stabilize_v1(
            receipts.clone(),
            &mut budget,
            |_, _, budget| {
                super::v35_authentic_receipt_stabilization_vectors::charged_result(
                    budget,
                    super::v35_authentic_receipt_stabilization_vectors::residual(0.25),
                    super::v35_authentic_receipt_stabilization_vectors::artifact(1),
                    receipts.clone(),
                )
            },
        )
        .expect("v35 exact probe plus same-input replay");
        assert_eq!(budget.used, 11);
        assert_eq!(stabilized.stabilization_probe_count, 1);
        assert_eq!(stabilized.independent_replay_count, 1);
        assert!(!stabilized.publication_eligible);
    }

    #[test]
    fn v36_geometry_solver_refuses_branch_poison_interpolation_repair_or_bypass() {
        for density in [0.0, -1.0, f64::NAN, f64::INFINITY] {
            assert!(phase_consistent_canonical_phase_projection_v1(0.31, -1.0, density).is_err());
        }
        let coordinate = geometry(0.31, -1.0, 100.0);
        assert_eq!(
            covered_terminal_density_geometry_residual_evaluate_v1(coordinate, 101.0, false),
            Err(PhaseConsistentCoupledSolveErrorV1::SideConstraint)
        );
        let repaired = CoveredTerminalDensityGeometryCoordinateV1 {
            z_1_m: f64::from_bits(coordinate.z_1_m.to_bits() + 1),
            ..coordinate
        };
        assert_eq!(
            covered_terminal_density_geometry_residual_evaluate_v1(repaired, 101.0, true),
            Err(PhaseConsistentCoupledSolveErrorV1::SideConstraint)
        );
        let mut one_volume = Wb11HydrologyKernel::initialize_stage3_persistent_state(7, Vec::new())
            .expect("empty state");
        one_volume.layers = vec![DirectSnowLayerState::new(0.001, 0.01, 100.0, 2.0)];
        assert!(covered_terminal_density_physical_layer_v1(&one_volume, 3.0).is_err());
        let mut multilayer = one_volume.clone();
        multilayer
            .layers
            .push(DirectSnowLayerState::new(0.001, 0.01, 100.0, 2.0));
        assert!(covered_terminal_density_physical_layer_v1(&multilayer, 2.0).is_err());
        let mut exhausted =
            CoveredPhysicalEvaluationBudgetV1::new(COVERED_PHYSICAL_EVALUATION_LIMIT_V1)
                .expect("exhausted budget");
        assert_eq!(
            covered_physical_evaluation_budget_charge_v1(&mut exhausted),
            Err(PhaseConsistentCoupledSolveErrorV1::EvaluationBudget)
        );
        assert!(!CoveredConvergenceAdmissionV1::CoupledAuthentic.admits(false, true, false));
    }

    fn v37_low_density_inputs(proposed_water_kg_m2: f64) -> CoveredPhaseConsistentResidualInputsV1 {
        CoveredPhaseConsistentResidualInputsV1 {
            coordinates: vec![proposed_water_kg_m2, -1.0, 100.0],
            beginning_snow_water_kg_m2: vec![0.30],
            beginning_snow_enthalpy_j_m2: vec![-2.0],
            physical_delta_water_kg_m2: vec![0.01],
            physical_complete_energy_j_m2: vec![1.0],
            physical_ice_kg_m2: vec![0.31],
            physical_density_kg_m3: vec![100.0],
            physical_thickness_m: vec![0.0031],
            exact_density_settling_branch_satisfied: vec![true],
            beginning_soil_enthalpy_j_m2: vec![],
            physical_soil_delta_energy_j_m2: vec![],
            owner_soil_temperature_k: vec![],
            absolute_tolerances: vec![1.0e-6, 1.0e-6, 1.0e-6],
            algebraic_side_constraints_satisfied: true,
        }
    }

    #[test]
    fn v37_derived_thickness_closure_blocks_low_density_amplified_water_error() {
        let evaluation =
            covered_phase_consistent_residual_assemble_v1(v37_low_density_inputs(0.310_000_8))
                .expect("finite low-density physical image");
        assert!(evaluation.r_w_kg_m2[0].abs() <= 1.0e-6);
        assert!(evaluation.derived_thickness_closures[0].r_z_m.abs() > 1.0e-9);
        assert!(evaluation.derived_constraints_scaled_merit > 1.0);
        assert_eq!(
            evaluation.scaled_merit.to_bits(),
            evaluation.derived_constraints_scaled_merit.to_bits()
        );
    }

    #[test]
    fn v37_derived_thickness_closure_uses_same_charged_physical_image() {
        let mut budget = CoveredPhysicalEvaluationBudgetV1::new(0).expect("shared budget");
        let evaluation = covered_phase_consistent_residual_evaluate_v1(
            v37_low_density_inputs(0.31),
            &mut budget,
        )
        .expect("one charged physical image");
        assert_eq!(budget.used, 1);
        assert!(evaluation.r_w_kg_m2[0].abs() <= f64::EPSILON);
        assert_eq!(evaluation.derived_thickness_closures[0].r_z_m, 0.0);

        let mut poisoned = v37_low_density_inputs(0.31);
        poisoned.physical_thickness_m[0] =
            f64::from_bits(poisoned.physical_thickness_m[0].to_bits() + 1);
        assert_eq!(
            covered_phase_consistent_residual_evaluate_v1(poisoned, &mut budget),
            Err(PhaseConsistentCoupledSolveErrorV1::SideConstraint)
        );
        assert_eq!(budget.used, 2);
    }

    #[test]
    fn v37_root_admission_requires_residual_and_derived_thickness_merit() {
        let initial =
            covered_phase_consistent_residual_assemble_v1(v37_low_density_inputs(0.310_000_8))
                .expect("initial physical image");
        assert!(initial.r_w_kg_m2[0].abs() <= 1.0e-6);
        assert!(initial.scaled_merit > 1.0);

        let mut budget = CoveredPhysicalEvaluationBudgetV1::new(0).expect("shared budget");
        let solve = phase_consistent_coupled_solve_v1(
            initial.coordinates.clone(),
            &mut budget,
            |coordinates, budget| {
                let mut inputs = v37_low_density_inputs(coordinates[0]);
                inputs.coordinates.clone_from_slice(coordinates);
                covered_phase_consistent_residual_evaluate_v1(inputs, budget)
            },
        )
        .expect("derived depth drives the existing water solve");
        assert!(solve.root.scaled_merit <= 1.0);
        assert!(solve.root.derived_constraints_scaled_merit <= 1.0);
        assert!(solve.root.r_w_kg_m2[0].abs() < initial.r_w_kg_m2[0].abs());
        assert_eq!(solve.root.coordinates.len(), 3);
        assert_eq!(solve.root.residuals.len(), 3);
    }

    #[test]
    fn v37_receipt_replay_and_finalization_retain_derived_thickness_closure() {
        let captured_root_z = f64::from_bits(4_569_208_177_783_694_401);
        let captured_final_z = f64::from_bits(4_569_208_162_027_237_604);
        let density_kg_m3 = 100.0;
        let root_ice_kg_m2 = captured_root_z * density_kg_m3;
        let physical_ice_kg_m2 = captured_final_z * density_kg_m3;
        let evaluate = |coordinates: &[f64], budget: &mut CoveredPhysicalEvaluationBudgetV1| {
            covered_phase_consistent_residual_evaluate_v1(
                CoveredPhaseConsistentResidualInputsV1 {
                    coordinates: coordinates.to_vec(),
                    beginning_snow_water_kg_m2: vec![physical_ice_kg_m2],
                    beginning_snow_enthalpy_j_m2: vec![-1.0],
                    physical_delta_water_kg_m2: vec![0.0],
                    physical_complete_energy_j_m2: vec![0.0],
                    physical_ice_kg_m2: vec![physical_ice_kg_m2],
                    physical_density_kg_m3: vec![density_kg_m3],
                    physical_thickness_m: vec![captured_final_z],
                    exact_density_settling_branch_satisfied: vec![true],
                    beginning_soil_enthalpy_j_m2: vec![],
                    physical_soil_delta_energy_j_m2: vec![],
                    owner_soil_temperature_k: vec![],
                    absolute_tolerances: vec![1.0e-6, 1.0e-6, 1.0e-6],
                    algebraic_side_constraints_satisfied: true,
                },
                budget,
            )
        };
        let initial =
            covered_phase_consistent_residual_assemble_v1(CoveredPhaseConsistentResidualInputsV1 {
                coordinates: vec![root_ice_kg_m2, -1.0, density_kg_m3],
                beginning_snow_water_kg_m2: vec![physical_ice_kg_m2],
                beginning_snow_enthalpy_j_m2: vec![-1.0],
                physical_delta_water_kg_m2: vec![0.0],
                physical_complete_energy_j_m2: vec![0.0],
                physical_ice_kg_m2: vec![physical_ice_kg_m2],
                physical_density_kg_m3: vec![density_kg_m3],
                physical_thickness_m: vec![captured_final_z],
                exact_density_settling_branch_satisfied: vec![true],
                beginning_soil_enthalpy_j_m2: vec![],
                physical_soil_delta_energy_j_m2: vec![],
                owner_soil_temperature_k: vec![],
                absolute_tolerances: vec![1.0e-6, 1.0e-6, 1.0e-6],
                algebraic_side_constraints_satisfied: true,
            })
            .expect("captured r93 root image");
        assert!(initial.r_w_kg_m2[0].abs() <= 1.0e-6);
        assert_eq!(
            initial.derived_thickness_closures[0].proposed_z_m.to_bits(),
            captured_root_z.to_bits()
        );
        assert_eq!(
            initial.derived_thickness_closures[0].physical_z_m.to_bits(),
            captured_final_z.to_bits()
        );
        assert!(initial.derived_constraints_scaled_merit > 1.0);

        let mut budget = CoveredPhysicalEvaluationBudgetV1::new(0).expect("shared budget");
        let solve =
            phase_consistent_coupled_solve_v1(initial.coordinates.clone(), &mut budget, evaluate)
                .expect("captured root closes derived depth");
        assert!(solve.root.derived_constraints_scaled_merit <= 1.0);

        let receipts = super::v35_authentic_receipt_stabilization_vectors::receipt_set(273.0);
        let artifact = super::v35_authentic_receipt_stabilization_vectors::artifact(77);
        let stabilized = covered_authentic_receipt_stabilize_v1(
            receipts.clone(),
            &mut budget,
            |_, _, budget| {
                super::v35_authentic_receipt_stabilization_vectors::charged_result(
                    budget,
                    solve.root.clone(),
                    artifact.clone(),
                    receipts.clone(),
                )
            },
        )
        .expect("same-input receipt replay retains derived closure");
        assert!(stabilized.residual.derived_constraints_scaled_merit <= 1.0);
        assert!(CoveredConvergenceAdmissionV1::CoupledAuthentic.admits(
            false,
            stabilized.residual.scaled_merit <= 1.0,
            stabilized.independent_replay_count == 1,
        ));

        let root_geometry = geometry(
            solve.root.coordinates[0],
            solve.root.coordinates[1],
            solve.root.coordinates[2],
        );
        let mut root_state = Wb11HydrologyKernel::initialize_stage3_persistent_state(7, Vec::new())
            .expect("root finalization state");
        root_state.layers = vec![DirectSnowLayerState::new(
            root_geometry.ice_1_kg_m2 / 1_000.0,
            root_geometry.z_1_m,
            root_geometry.rho_1_kg_m3,
            2.0,
        )];
        root_state.fingerprint =
            Wb11HydrologyKernel::stage3_persistent_state_fingerprint(&root_state);
        let mut final_state =
            Wb11HydrologyKernel::initialize_stage3_persistent_state(7, Vec::new())
                .expect("authentic finalization state");
        final_state.layers = vec![DirectSnowLayerState::new(
            physical_ice_kg_m2 / 1_000.0,
            captured_final_z,
            density_kg_m3,
            2.0,
        )];
        final_state.fingerprint =
            Wb11HydrologyKernel::stage3_persistent_state_fingerprint(&final_state);
        assert!(covered_fixed_point_stage3_states_equal(
            &BTreeMap::from([(7, root_state)]),
            &BTreeMap::from([(7, final_state)])
        ));
    }

    #[test]
    fn v37_refuses_independent_z_omission_interpolation_repair_or_bypass() {
        let coordinate = geometry(0.31, -1.0, 100.0);
        let poisoned_coordinate = CoveredTerminalDensityGeometryCoordinateV1 {
            z_1_m: f64::from_bits(coordinate.z_1_m.to_bits() + 1),
            ..coordinate
        };
        assert_eq!(
            covered_derived_thickness_closure_evaluate_v1(poisoned_coordinate, 0.31, 100.0, 0.0031,),
            Err(PhaseConsistentCoupledSolveErrorV1::SideConstraint)
        );
        assert_eq!(
            covered_derived_thickness_closure_evaluate_v1(
                coordinate,
                0.31,
                100.0,
                f64::from_bits(0.0031_f64.to_bits() + 1),
            ),
            Err(PhaseConsistentCoupledSolveErrorV1::SideConstraint)
        );
        for invalid in [0.0, -1.0, f64::NAN, f64::INFINITY] {
            assert!(covered_derived_thickness_closure_evaluate_v1(
                coordinate, 0.31, 100.0, invalid,
            )
            .is_err());
        }
        let blocked =
            covered_phase_consistent_residual_assemble_v1(v37_low_density_inputs(0.310_000_8))
                .expect("finite but depth-open image");
        assert!(!CoveredConvergenceAdmissionV1::CoupledAuthentic.admits(
            false,
            blocked.scaled_merit <= 1.0,
            true,
        ));
    }

    #[test]
    fn v38_charged_evaluation_uses_finalization_equivalent_endpoint_map() {
        let mut budget = CoveredPhysicalEvaluationBudgetV1::new(8).expect("shared budget");
        let calls = std::cell::Cell::new(0_usize);
        let map = covered_phase_consistent_finalization_equivalent_map_v1(&mut budget, || {
            calls.set(calls.get() + 1);
            Ok(41_u32)
        })
        .expect("one finalization-equivalent map");
        map.validate().expect("map posture");
        assert_eq!(budget.used, 9);
        assert_eq!(calls.get(), 1);
        assert_eq!(map.stage3_physical_map_count, 1);
        assert_eq!(map.value, 41);

        let mut lane_7 = state_for(7);
        lane_7.layers[0].refrozen_liquid_m = 1.0e-5;
        reseal(&mut lane_7);
        let baseline = BTreeMap::from([(7, lane_7), (8, state_for(8))]);
        let coordinates = [0.31, -1_000.0, 100.0, 0.42, -2_000.0, 140.0];
        let (projected, phases) = covered_phase_consistent_project_stage3_coordinates_v1(
            &baseline,
            &[7, 8],
            &coordinates,
        )
        .expect("canonical endpoint Stage-3 projection");
        for (lane_index, lane_id) in [7_u32, 8].iter().enumerate() {
            let state = &projected[lane_id];
            let layer = &state.layers[0];
            let phase = &phases[lane_id];
            assert_eq!(state.layers.len(), 1);
            assert_eq!(
                layer.mass_swe_m.to_bits(),
                (phase.ice_kg_m2 / 1_000.0).to_bits()
            );
            assert_eq!(layer.thickness_m.to_bits(), phase.depth_m.to_bits());
            assert_eq!(
                layer.density_kg_m3.to_bits(),
                coordinates[3 * lane_index + 2].to_bits()
            );
            assert_eq!(layer.settle_day_count.to_bits(), 3.0_f64.to_bits());
            assert_eq!(
                state.fingerprint,
                Wb11HydrologyKernel::stage3_persistent_state_fingerprint(state)
            );
        }
        assert_eq!(
            projected[&7].layers[0].refrozen_liquid_m.to_bits(),
            1.0e-5_f64.to_bits()
        );
    }

    #[test]
    fn v38_provisional_map_closure_cannot_admit_root() {
        for stage3_physical_map_count in [0, 2] {
            let poisoned = CoveredFinalizationEquivalentPhysicalMapV1 {
                posture: CoveredPhaseConsistentPhysicalMapPostureV1::FinalizationEquivalent,
                stage3_physical_map_count,
                physical_evaluation_ordinal: 1,
                value: (),
            };
            assert_eq!(
                poisoned.validate(),
                Err(PhaseConsistentCoupledSolveErrorV1::SideConstraint)
            );
        }
        assert!(!CoveredConvergenceAdmissionV1::CoupledAuthentic.admits(false, true, false));
    }

    #[test]
    fn v38_finalization_equivalent_map_is_receipt_replay_stable() {
        let receipts = super::v35_authentic_receipt_stabilization_vectors::receipt_set(273.0);
        let artifact = super::v35_authentic_receipt_stabilization_vectors::artifact(88);
        let residual = covered_phase_consistent_residual_assemble_v1(v37_low_density_inputs(0.31))
            .expect("closed residual");
        let mut budget = CoveredPhysicalEvaluationBudgetV1::new(0).expect("shared budget");
        let stabilized = covered_authentic_receipt_stabilize_v1(
            receipts.clone(),
            &mut budget,
            |_, _, budget| {
                let map = covered_phase_consistent_finalization_equivalent_map_v1(budget, || {
                    Ok((
                        residual.clone(),
                        artifact.clone(),
                        CoveredFinalizationEquivalentReplayInputsV1 {
                            proposed_stage3: artifact.stage3_candidate.clone(),
                            proposed_soil: artifact.soil_candidate.clone(),
                            input_covered_boundaries: BTreeMap::new(),
                            input_open_boundaries: BTreeMap::new(),
                            destination_receipts: BTreeMap::new(),
                        },
                        receipts.clone(),
                    ))
                })?;
                map.validate()?;
                Ok(map.value)
            },
        )
        .expect("exact receipt replay over the finalization-equivalent map");
        assert_eq!(budget.used, 2);
        assert_eq!(stabilized.independent_replay_count, 1);
        assert_eq!(stabilized.artifacts, artifact);
        assert_eq!(stabilized.stabilized_receipts, receipts);
    }

    #[test]
    fn v38_finalization_independent_replay_requires_exact_same_map_image() {
        let artifact = super::v35_authentic_receipt_stabilization_vectors::artifact(99);
        let mut budget = CoveredPhysicalEvaluationBudgetV1::new(0).expect("shared budget");
        let first = covered_phase_consistent_finalization_equivalent_map_v1(&mut budget, || {
            Ok(artifact.clone())
        })
        .expect("first endpoint image");
        let replay = covered_phase_consistent_finalization_equivalent_map_v1(&mut budget, || {
            Ok(artifact.clone())
        })
        .expect("independent endpoint image");
        first.validate().expect("first posture");
        replay.validate().expect("replay posture");
        assert!(covered_phase_consistent_artifacts_exact_v1(
            &first.value,
            &replay.value,
        ));
        let mut poisoned = replay.value;
        poisoned.transaction_id = TransactionId(poisoned.transaction_id.0 + 1);
        assert!(!covered_phase_consistent_artifacts_exact_v1(
            &first.value,
            &poisoned,
        ));
    }

    #[test]
    fn v38_refuses_extra_physics_map_repair_bypass_or_publication() {
        let mut budget = CoveredPhysicalEvaluationBudgetV1::new(0).expect("shared budget");
        let refused = covered_phase_consistent_finalization_equivalent_map_v1(
            &mut budget,
            || -> Result<(), PhaseConsistentCoupledSolveErrorV1> {
                Err(PhaseConsistentCoupledSolveErrorV1::SideConstraint)
            },
        );
        assert_eq!(
            refused,
            Err(PhaseConsistentCoupledSolveErrorV1::SideConstraint)
        );
        assert_eq!(budget.used, 1);

        let mut exhausted =
            CoveredPhysicalEvaluationBudgetV1::new(COVERED_PHYSICAL_EVALUATION_LIMIT_V1)
                .expect("exhausted budget");
        let called = std::cell::Cell::new(false);
        assert_eq!(
            covered_phase_consistent_finalization_equivalent_map_v1(&mut exhausted, || {
                called.set(true);
                Ok(())
            }),
            Err(PhaseConsistentCoupledSolveErrorV1::EvaluationBudget)
        );
        assert!(!called.get());
        assert!(!CoveredConvergenceAdmissionV1::Picard.admits(false, true, true));
        assert!(!CoveredConvergenceAdmissionV1::CoupledAuthentic.admits(true, true, false));

        let one_volume = BTreeMap::from([(7, state_for(7))]);
        for coordinates in [
            vec![0.31, -1.0, 0.0],
            vec![0.31, -1.0, -100.0],
            vec![0.31, -1.0, f64::NAN],
            vec![0.31, -1.0, f64::INFINITY],
        ] {
            assert!(covered_phase_consistent_project_stage3_coordinates_v1(
                &one_volume,
                &[7],
                &coordinates,
            )
            .is_err());
        }
        assert!(covered_phase_consistent_project_stage3_coordinates_v1(
            &one_volume,
            &[8],
            &[0.31, -1.0, 100.0],
        )
        .is_err());
        let mut multi_layer = state_for(7);
        multi_layer
            .layers
            .push(DirectSnowLayerState::new(0.1, 0.2, 500.0, 3.0));
        reseal(&mut multi_layer);
        assert!(covered_phase_consistent_project_stage3_coordinates_v1(
            &BTreeMap::from([(7, multi_layer)]),
            &[7],
            &[0.31, -1.0, 100.0],
        )
        .is_err());
    }
}

// Contract-first vectors were retained after the recorded expected-red gate and
// are active with the authorized production seam.
#[cfg(test)]
mod v31_preimplementation_contract_vectors {
    use super::*;

    const V31_FUSION_J_KG: f64 = 333_600.0;
    const V31_CAPTURE_START_NS: u128 = 1_860_000_000_000;
    const V31_CAPTURE_END_NS: u128 = 1_980_000_000_000;
    const V31_EXACT_FLOOR_START_NS: u128 = 1_920_000_000_000;

    fn v31_terminal_beginning() -> DirectSnowStage3PersistentState {
        Wb11HydrologyKernel::initialize_stage3_persistent_state(7, Vec::new())
            .expect("captured-support immutable snow-free beginning")
    }

    fn v31_support_image(
        actual_vapor_kg_m2: f64,
        complete_energy_j_m2: f64,
    ) -> CoveredExactFloorTerminalPhaseSupportImageV1 {
        CoveredExactFloorTerminalPhaseSupportImageV1 {
            parent_start_ns: V31_CAPTURE_START_NS,
            parent_end_ns: V31_CAPTURE_END_NS,
            support_start_ns: V31_EXACT_FLOOR_START_NS,
            support_end_ns: V31_CAPTURE_END_NS,
            actual_vapor_kg_m2,
            deposition_kg_m2: actual_vapor_kg_m2.max(0.0),
            sublimation_kg_m2: (-actual_vapor_kg_m2).max(0.0),
            snowfall_kg_m2: 0.32,
            external_liquid_kg_m2: 0.0,
            complete_energy_j_m2,
            cold_content_export_j_m2: 0.0,
            ordered_energy_components_j_m2: [complete_energy_j_m2, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0],
            source_receipt_fingerprints: [11, 12, 13, 14, 15, 16],
        }
    }

    fn v31_phase_oracle(
        beginning: &DirectSnowStage3PersistentState,
        support: &CoveredExactFloorTerminalPhaseSupportImageV1,
    ) -> DirectSnowStage3PersistentState {
        assert!(beginning.layers.is_empty());
        let beginning_ice = 0.0;
        let beginning_liquid = 0.0;
        let beginning_cold = 0.0;
        let deposition = support.actual_vapor_kg_m2.max(0.0);
        let sublimation = (-support.actual_vapor_kg_m2)
            .max(0.0)
            .min(beginning_ice + support.snowfall_kg_m2 + deposition);
        let water = beginning_ice + beginning_liquid + support.snowfall_kg_m2 + deposition
            - sublimation
            + support.external_liquid_kg_m2;
        let enthalpy = -beginning_cold
            + V31_FUSION_J_KG * (beginning_liquid + support.external_liquid_kg_m2)
            + support.complete_energy_j_m2
            + support.cold_content_export_j_m2;
        let fusion_capacity = V31_FUSION_J_KG * water;
        let (ice, liquid, cold, unallocated) = if enthalpy < 0.0 {
            (water, 0.0, -enthalpy, 0.0)
        } else if enthalpy < fusion_capacity {
            let liquid = enthalpy / V31_FUSION_J_KG;
            (water - liquid, liquid, 0.0, 0.0)
        } else {
            (0.0, water, 0.0, (enthalpy - fusion_capacity).max(0.0))
        };
        let liquid_pre = beginning_liquid + support.external_liquid_kg_m2;
        let melt = (liquid - liquid_pre).max(0.0);
        let refreeze = (liquid_pre - liquid).max(0.0);

        let mut state = beginning.clone();
        state.next_interval_index = beginning.next_interval_index;
        state.layers = vec![DirectSnowLayerState::new(0.0, 0.0, 100.0, 0.0)];
        let layer = &mut state.layers[0];
        layer.mass_swe_m = ice / 1_000.0;
        layer.liquid_water_m = liquid / 1_000.0;
        layer.cold_content_j_m2 = cold;
        layer.refrozen_liquid_m = refreeze / 1_000.0;
        layer.thickness_m = layer.mass_swe_m * 1_000.0 / layer.density_kg_m3;
        layer.temperature_c = Wb11HydrologyKernel::stage3_temperature_from_cold_content_values(
            layer.mass_swe_m,
            layer.cold_content_j_m2,
        );
        state.cumulative_snowfall_kg_m2 += support.snowfall_kg_m2;
        state.cumulative_external_liquid_kg_m2 += support.external_liquid_kg_m2;
        state.cumulative_deposition_kg_m2 += deposition;
        state.cumulative_sublimation_kg_m2 += sublimation;
        state.cumulative_melt_kg_m2 += melt;
        state.cumulative_complete_energy_j_m2 += support.complete_energy_j_m2;
        state.cumulative_cold_energy_change_j_m2 +=
            beginning_cold - cold - V31_FUSION_J_KG * refreeze - support.cold_content_export_j_m2;
        state.cumulative_terminal_unallocated_energy_j_m2 += unallocated;
        reseal(&mut state);
        state
    }

    #[test]
    fn v31_retained_midpoint_oracle_reconstructs_exact_canonical_w_h_without_publication() {
        let beginning = v31_terminal_beginning();
        let mut current = v31_support_image(0.125, -128.0);
        let mut authentic = v31_support_image(0.375, 128.0);
        current.snowfall_kg_m2 = 0.25;
        authentic.snowfall_kg_m2 = 0.25;
        let retained = covered_exact_floor_terminal_phase_iterate_v1(
            &beginning,
            &current,
            &authentic,
            DirectSnowLayerState::new(0.0, 0.0, 100.0, 0.0),
            beginning.next_interval_index,
        )
        .expect("retained exact-floor canonical midpoint oracle");
        assert_eq!(
            retained.support_image.actual_vapor_kg_m2.to_bits(),
            0.25_f64.to_bits()
        );
        assert_eq!(
            retained.support_image.complete_energy_j_m2.to_bits(),
            0.0_f64.to_bits()
        );
        assert_eq!(retained.raw_authentic_support_image, authentic);
        assert!(!retained.publication_eligible);
        let layer = &retained.iterate.layers[0];
        assert_eq!(layer.mass_swe_m.to_bits(), 0.0005_f64.to_bits());
        assert_eq!(layer.liquid_water_m.to_bits(), 0.0_f64.to_bits());
        assert_eq!(layer.cold_content_j_m2.to_bits(), 0.0_f64.to_bits());
        Wb11HydrologyKernel::validate_stage3_persistent_cumulative_closure(&retained.iterate)
            .expect("retained midpoint oracle closes exactly");
    }

    fn assert_v31_close(left: f64, right: f64, tolerance: f64) {
        assert!(
            (left - right).abs() <= tolerance,
            "left={left:.17e} right={right:.17e} tolerance={tolerance:.3e}",
        );
    }

    fn v32_captured_support(
        duration_ns: u128,
        vapor: f64,
        latent: f64,
    ) -> CoveredExactFloorTerminalPhaseSupportImageV1 {
        let mut image = v31_support_image(vapor, 0.0);
        image.support_start_ns = V31_CAPTURE_START_NS;
        image.support_end_ns = V31_CAPTURE_START_NS + duration_ns;
        image.parent_start_ns = image.support_start_ns;
        image.parent_end_ns = image.support_end_ns;
        image.external_liquid_kg_m2 = if vapor > 0.0 { 0.01 } else { 0.03 };
        image.ordered_energy_components_j_m2 = [100.0, -50.0, 25.0, latent, 10.0, -5.0, 2.0];
        image.complete_energy_j_m2 = image.ordered_energy_components_j_m2.iter().sum();
        image
    }

    #[test]
    fn v32_vapor_active_set_accepts_direct_support_above_exact_floor() {
        let current_v = 2.121_596_912_395_713_5e-4;
        let authentic_v = -4.616_612_304_251_271e-3;
        let current_qv = 6.490_579_369_251_98e2;
        let authentic_qv = -1.308_163_262_532_640_2e4;
        for duration_ns in [COVERED_VAPOR_ACTIVE_SET_MIN_SUPPORT_NS, 120_000_000_000] {
            let current = v32_captured_support(duration_ns, current_v, current_qv);
            let authentic = v32_captured_support(duration_ns, authentic_v, authentic_qv);
            let interface = covered_vapor_active_set_interface_v1(&current, &authentic)
                .expect("captured opposite-sign operands have one exact interface");
            assert_eq!(
                interface.alpha_v.to_bits(),
                0.043_936_572_577_394_06_f64.to_bits()
            );
            assert!(!interface.publication_eligible);
            assert_eq!(interface.raw_authentic_support_image, authentic);
            for value in [
                interface.support_image.actual_vapor_kg_m2,
                interface.support_image.deposition_kg_m2,
                interface.support_image.sublimation_kg_m2,
                interface.support_image.ordered_energy_components_j_m2[3],
            ] {
                assert_eq!(value.to_bits(), 0.0_f64.to_bits());
            }
            let affine_latent = current_qv + interface.alpha_v * (authentic_qv - current_qv);
            assert_v31_close(affine_latent, 45.778_454_499_090_91, 1.0e-12);
            assert_ne!(affine_latent.to_bits(), 0.0_f64.to_bits());
            let expected_nonlatent = current.ordered_energy_components_j_m2[0]
                + interface.alpha_v
                    * (authentic.ordered_energy_components_j_m2[0]
                        - current.ordered_energy_components_j_m2[0]);
            assert_eq!(
                interface.support_image.ordered_energy_components_j_m2[0].to_bits(),
                expected_nonlatent.to_bits(),
            );
            assert_eq!(
                interface.support_image.complete_energy_j_m2.to_bits(),
                interface
                    .support_image
                    .ordered_energy_components_j_m2
                    .iter()
                    .sum::<f64>()
                    .to_bits(),
            );
        }
    }

    #[test]
    fn v32_vapor_active_set_retains_same_sign_v31_dispatch() {
        let beginning = v31_terminal_beginning();
        let current = v31_support_image(0.125, -128.0);
        let authentic = v31_support_image(0.375, 128.0);
        assert!(covered_vapor_active_set_interface_v1(&current, &authentic).is_err());
        let retained = covered_exact_floor_terminal_phase_iterate_v1(
            &beginning,
            &current,
            &authentic,
            DirectSnowLayerState::new(0.0, 0.0, 100.0, 0.0),
            beginning.next_interval_index,
        )
        .expect("same-sign phase crossing retains the V31 canonical W/H dispatch");
        assert_eq!(retained.raw_authentic_support_image, authentic);
        assert!(!retained.publication_eligible);
        Wb11HydrologyKernel::validate_stage3_persistent_cumulative_closure(&retained.iterate)
            .expect("retained same-sign V31 dispatch closes exactly");
    }

    #[test]
    fn vapor_active_set_branch_entry_uses_support_weight_and_authentic_specific_latent_heat() {
        let mut current = v32_captured_support(
            COVERED_VAPOR_ACTIVE_SET_MIN_SUPPORT_NS,
            2.121_596_912_395_713_5e-4,
            6.490_579_369_251_98e2,
        );
        let mut authentic = v32_captured_support(
            COVERED_VAPOR_ACTIVE_SET_MIN_SUPPORT_NS,
            -4.616_612_304_251_271e-3,
            -1.308_163_262_532_640_2e4,
        );
        current.external_liquid_kg_m2 = 0.0;
        authentic.external_liquid_kg_m2 = 0.0;
        let interface = covered_vapor_active_set_interface_v1(&current, &authentic)
            .expect("captured interface");
        let entry = covered_vapor_active_set_branch_entry_v1(&interface.support_image, &authentic)
            .expect("zero-to-sublimation branch entry");
        assert_eq!(entry.alpha_v.to_bits(), 0.5_f64.to_bits());
        assert_eq!(
            entry.support_image.actual_vapor_kg_m2.to_bits(),
            (0.5 * authentic.actual_vapor_kg_m2).to_bits(),
        );
        assert_eq!(
            entry.support_image.deposition_kg_m2.to_bits(),
            0.0_f64.to_bits()
        );
        assert_eq!(
            entry.support_image.sublimation_kg_m2.to_bits(),
            (-entry.support_image.actual_vapor_kg_m2).to_bits(),
        );
        let authentic_specific_latent =
            authentic.ordered_energy_components_j_m2[3] / authentic.actual_vapor_kg_m2;
        assert_eq!(
            entry.support_image.ordered_energy_components_j_m2[3].to_bits(),
            (entry.support_image.actual_vapor_kg_m2 * authentic_specific_latent).to_bits(),
        );
        assert!(!entry.publication_eligible);

        let mut long_interface = interface.support_image.clone();
        let mut long_authentic = authentic.clone();
        long_interface.support_end_ns += 420_000_000_000;
        long_interface.parent_end_ns = long_interface.support_end_ns;
        long_authentic.support_end_ns = long_interface.support_end_ns;
        long_authentic.parent_end_ns = long_interface.parent_end_ns;
        let long_entry = covered_vapor_active_set_branch_entry_v1(&long_interface, &long_authentic)
            .expect("480-second direct support entry");
        assert_eq!(long_entry.alpha_v.to_bits(), 0.25_f64.to_bits());
    }

    #[test]
    fn v32_vapor_active_set_synthetic_images_cannot_publish() {
        let beginning = v31_terminal_beginning();
        let mut current = v32_captured_support(
            COVERED_VAPOR_ACTIVE_SET_MIN_SUPPORT_NS,
            2.121_596_912_395_713_5e-4,
            6.490_579_369_251_98e2,
        );
        let mut authentic = v32_captured_support(
            COVERED_VAPOR_ACTIVE_SET_MIN_SUPPORT_NS,
            -4.616_612_304_251_271e-3,
            -1.308_163_262_532_640_2e4,
        );
        current.external_liquid_kg_m2 = 0.0;
        authentic.external_liquid_kg_m2 = 0.0;
        let current_state = v31_phase_oracle(&beginning, &current);
        let authentic_state = v31_phase_oracle(&beginning, &authentic);
        let outcome = covered_vapor_active_set_iterate_v1(
            &BTreeMap::from([(7, current_state)]),
            &BTreeMap::from([(7, authentic_state.clone())]),
            &BTreeMap::from([(7, beginning.clone())]),
            &BTreeMap::from([(7, current)]),
            &BTreeMap::from([(7, authentic.clone())]),
            CoveredVaporActiveSetTransitionV1::Interface,
        )
        .expect("captured root projects through canonical W/H");
        assert_eq!(outcome.raw_authentic_candidate[&7], authentic_state);
        assert_eq!(
            outcome.transition,
            CoveredVaporActiveSetTransitionV1::Interface
        );
        assert!(!outcome.publication_eligible);
        let interface_support = &outcome.support_images[&7];
        let expected = v31_phase_oracle(&beginning, interface_support);
        assert_eq!(outcome.iterate[&7], expected);
        let state = &outcome.iterate[&7];
        let water = (state.layers[0].mass_swe_m + state.layers[0].liquid_water_m) * 1_000.0;
        let expected_water =
            interface_support.snowfall_kg_m2 + interface_support.external_liquid_kg_m2;
        assert_v31_close(water, expected_water, 1.0e-12);
        let enthalpy = V31_FUSION_J_KG * state.layers[0].liquid_water_m * 1_000.0
            - state.layers[0].cold_content_j_m2
            + state.cumulative_terminal_unallocated_energy_j_m2;
        let expected_enthalpy = V31_FUSION_J_KG * interface_support.external_liquid_kg_m2
            + interface_support.complete_energy_j_m2;
        assert_v31_close(enthalpy, expected_enthalpy, 1.0e-9);
        Wb11HydrologyKernel::validate_stage3_persistent_cumulative_closure(state)
            .expect("active-set interface closure");
    }

    #[test]
    fn vapor_active_set_refuses_zero_mixed_latent_component_and_identity_poisons() {
        let current = v32_captured_support(COVERED_VAPOR_ACTIVE_SET_MIN_SUPPORT_NS, 0.01, 30_000.0);
        let authentic =
            v32_captured_support(COVERED_VAPOR_ACTIVE_SET_MIN_SUPPORT_NS, -0.02, -60_000.0);
        let mut poison = authentic.clone();
        poison.actual_vapor_kg_m2 = 0.0;
        poison.sublimation_kg_m2 = 0.01;
        poison.deposition_kg_m2 = 0.01;
        assert_eq!(
            covered_vapor_active_set_interface_v1(&current, &poison),
            Err(CoveredExactFloorTerminalPhaseErrorV1::VaporMixedDisposition),
        );
        let mut zero = current.clone();
        zero.actual_vapor_kg_m2 = 0.0;
        zero.deposition_kg_m2 = 0.0;
        zero.ordered_energy_components_j_m2[3] = 0.0;
        zero.complete_energy_j_m2 = zero.ordered_energy_components_j_m2.iter().sum();
        assert!(covered_vapor_active_set_interface_v1(&zero, &authentic).is_err());
        let mut identity = authentic.clone();
        identity.source_receipt_fingerprints[2] ^= 1;
        assert_eq!(
            covered_vapor_active_set_interface_v1(&current, &identity),
            Err(CoveredExactFloorTerminalPhaseErrorV1::SourceIdentity),
        );
        let interface =
            covered_vapor_active_set_interface_v1(&current, &authentic).expect("valid interface");
        let mut latent = authentic.clone();
        latent.ordered_energy_components_j_m2[3] = 1.0;
        latent.complete_energy_j_m2 = latent.ordered_energy_components_j_m2.iter().sum();
        assert_eq!(
            covered_vapor_active_set_branch_entry_v1(&interface.support_image, &latent),
            Err(CoveredExactFloorTerminalPhaseErrorV1::VaporLatent),
        );
        let mut nonfinite = authentic;
        nonfinite.ordered_energy_components_j_m2[0] = f64::NAN;
        assert!(covered_vapor_active_set_interface_v1(&current, &nonfinite).is_err());
    }

    fn v42_captured_reappearance_endpoint() -> (
        DirectSnowStage3PersistentState,
        CoveredExactFloorTerminalPhaseSupportImageV1,
        DirectSnowStage3PersistentState,
    ) {
        let beginning_mass_swe_m = 0.001_834_695_025_909_846_2;
        let beginning_cold_j_m2 = 2_759.688_012_590_927;
        let beginning_layer = DirectSnowLayerState {
            mass_swe_m: beginning_mass_swe_m,
            thickness_m: 0.018_346_950_259_098_462,
            density_kg_m3: 100.0,
            settle_day_count: 0.0,
            temperature_c: Wb11HydrologyKernel::stage3_temperature_from_cold_content_values(
                beginning_mass_swe_m,
                beginning_cold_j_m2,
            ),
            liquid_water_m: 0.0,
            cold_content_j_m2: beginning_cold_j_m2,
            refrozen_liquid_m: 0.0,
        };
        let beginning =
            Wb11HydrologyKernel::initialize_stage3_persistent_state(7, vec![beginning_layer])
                .expect("captured reappearance beginning");
        let complete_energy_j_m2 = -30_981.514_410_700_682;
        let captured_ending_enthalpy_j_m2 = -33_738.881_304_185_81;
        let actual_vapor_kg_m2 = -2.269_113_572_450_036e-4;
        let mut support = v31_support_image(actual_vapor_kg_m2, complete_energy_j_m2);
        support.parent_start_ns = 72_000_000_000_000;
        support.parent_end_ns = 72_060_000_000_000;
        support.support_start_ns = support.parent_start_ns;
        support.support_end_ns = support.parent_end_ns;
        support.snowfall_kg_m2 = 0.0;
        support.external_liquid_kg_m2 = 0.0;
        support.ordered_energy_components_j_m2 = [
            0.0,
            -21_595.764_918_106_684,
            1_261.056_782_398_769,
            -643.483_504_985_346_3,
            0.0,
            -10_003.322_770_007_439,
            0.0,
        ];
        support.complete_energy_j_m2 = support.ordered_energy_components_j_m2.iter().sum();
        support.cold_content_export_j_m2 =
            captured_ending_enthalpy_j_m2 - (-beginning_cold_j_m2 + support.complete_energy_j_m2);

        let mut ending = beginning.clone();
        ending.layers[0].mass_swe_m = 0.001_834_468_114_552_603_2;
        ending.layers[0].thickness_m = 0.018_344_681_145_526_03;
        ending.layers[0].cold_content_j_m2 = -captured_ending_enthalpy_j_m2;
        ending.layers[0].temperature_c =
            Wb11HydrologyKernel::stage3_temperature_from_cold_content_values(
                ending.layers[0].mass_swe_m,
                ending.layers[0].cold_content_j_m2,
            );
        ending.cumulative_sublimation_kg_m2 += support.sublimation_kg_m2;
        ending.cumulative_complete_energy_j_m2 += support.complete_energy_j_m2;
        ending.cumulative_cold_energy_change_j_m2 += support.complete_energy_j_m2;
        reseal(&mut ending);
        Wb11HydrologyKernel::validate_stage3_persistent_cumulative_closure(&ending)
            .expect("captured authentic endpoint cumulative closure");
        (beginning, support, ending)
    }

    #[test]
    fn v42_cold_content_export_closes_captured_snow_reappearance_endpoint() {
        let (beginning, support, ending) = v42_captured_reappearance_endpoint();
        let (_, enthalpy) = phase_consistent_support_coordinates_v1(&beginning, &support)
            .expect("captured support coordinates");
        assert_eq!(
            enthalpy.to_bits(),
            (-ending.layers[0].cold_content_j_m2).to_bits()
        );
        assert!(covered_vapor_active_set_endpoint_coordinates_close_v1(
            &beginning, &support, &ending,
        )
        .expect("captured endpoint comparison"));
    }

    #[test]
    fn v42_zero_export_retains_v31_v32_coordinate_bits() {
        let beginning = v31_terminal_beginning();
        let support = v32_captured_support(
            COVERED_VAPOR_ACTIVE_SET_MIN_SUPPORT_NS,
            2.121_596_912_395_713_5e-4,
            6.490_579_369_251_98e2,
        );
        assert_eq!(
            support.cold_content_export_j_m2.to_bits(),
            0.0_f64.to_bits()
        );
        let coordinates = phase_consistent_support_coordinates_v1(&beginning, &support)
            .expect("zero-export support coordinates");
        let old_water = support.snowfall_kg_m2 + support.deposition_kg_m2
            - support.sublimation_kg_m2
            + support.external_liquid_kg_m2;
        let old_enthalpy =
            V31_FUSION_J_KG * support.external_liquid_kg_m2 + support.complete_energy_j_m2;
        assert_eq!(coordinates.0.to_bits(), old_water.to_bits());
        assert_eq!(coordinates.1.to_bits(), old_enthalpy.to_bits());
    }

    #[test]
    fn v42_private_active_set_contracts_export_with_exact_shared_weight() {
        let mut current = v32_captured_support(
            COVERED_VAPOR_ACTIVE_SET_MIN_SUPPORT_NS,
            2.121_596_912_395_713_5e-4,
            6.490_579_369_251_98e2,
        );
        let mut authentic = v32_captured_support(
            COVERED_VAPOR_ACTIVE_SET_MIN_SUPPORT_NS,
            -4.616_612_304_251_271e-3,
            -1.308_163_262_532_640_2e4,
        );
        current.cold_content_export_j_m2 = 2.0;
        authentic.cold_content_export_j_m2 = 6.0;
        let interface = covered_vapor_active_set_interface_v1(&current, &authentic)
            .expect("export-complete private interface");
        assert_eq!(
            interface.support_image.cold_content_export_j_m2.to_bits(),
            (current.cold_content_export_j_m2
                + interface.alpha_v
                    * (authentic.cold_content_export_j_m2 - current.cold_content_export_j_m2))
                .to_bits(),
        );
        let entry = covered_vapor_active_set_branch_entry_v1(&interface.support_image, &authentic)
            .expect("export-complete private branch entry");
        assert_eq!(entry.alpha_v.to_bits(), 0.5_f64.to_bits());
        assert_eq!(
            entry.support_image.cold_content_export_j_m2.to_bits(),
            (interface.support_image.cold_content_export_j_m2
                + entry.alpha_v
                    * (authentic.cold_content_export_j_m2
                        - interface.support_image.cold_content_export_j_m2))
                .to_bits(),
        );
        assert!(!interface.publication_eligible && !entry.publication_eligible);
    }

    #[test]
    fn v42_cold_content_export_refuses_nonfinite_negative_or_substituted_endpoint() {
        let (beginning, support, ending) = v42_captured_reappearance_endpoint();
        for value in [f64::NAN, f64::INFINITY, -1.0] {
            let mut poisoned = support.clone();
            poisoned.cold_content_export_j_m2 = value;
            assert!(poisoned.validate().is_err());
        }
        let mut substituted = support;
        substituted.cold_content_export_j_m2 += 1.0e-3;
        assert!(!covered_vapor_active_set_endpoint_coordinates_close_v1(
            &beginning,
            &substituted,
            &ending,
        )
        .expect("finite substituted endpoint comparison"));
    }

    #[test]
    fn v42_cold_content_export_refuses_omission_order_weight_or_closure_poison() {
        let (beginning, support, ending) = v42_captured_reappearance_endpoint();
        let mut omitted = support.clone();
        omitted.cold_content_export_j_m2 = 0.0;
        assert!(!covered_vapor_active_set_endpoint_coordinates_close_v1(
            &beginning, &omitted, &ending,
        )
        .expect("omitted export endpoint comparison"));

        let mut reordered = support.clone();
        reordered.source_receipt_fingerprints.swap(0, 1);
        assert_eq!(
            support.validate_active_set_identity(&reordered),
            Err(CoveredExactFloorTerminalPhaseErrorV1::SourceIdentity),
        );

        let mut weighted = support.clone();
        weighted.cold_content_export_j_m2 *= 0.5;
        assert!(!covered_vapor_active_set_endpoint_coordinates_close_v1(
            &beginning, &weighted, &ending,
        )
        .expect("independently weighted endpoint comparison"));

        let mut closure = ending;
        closure.layers[0].cold_content_j_m2 =
            f64::from_bits(closure.layers[0].cold_content_j_m2.to_bits() + 1_000_000);
        reseal(&mut closure);
        assert!(!covered_vapor_active_set_endpoint_coordinates_close_v1(
            &beginning, &support, &closure,
        )
        .expect("closure poison endpoint comparison"));
    }
}

#[test]
fn structural_fingerprint_and_count_fields_are_exact() {
    let original = state();
    let mut changed = original.clone();
    changed.fingerprint ^= 1;
    assert!(!equal(original.clone(), changed));
    let mut changed = original.clone();
    changed.layers[0].settle_day_count =
        f64::from_bits(changed.layers[0].settle_day_count.to_bits() + 1);
    reseal(&mut changed);
    assert!(!equal(original, changed));
}

#[test]
fn unit_specific_state_tolerances_do_not_share_one_scale() {
    let original = state();
    let mut within = original.clone();
    within.layers[0].mass_swe_m += 0.5e-9;
    within.layers[0].temperature_c += 0.5e-8;
    within.layers[0].cold_content_j_m2 += 0.5e-6;
    reseal(&mut within);
    assert!(equal(original.clone(), within));
    let mut outside = original.clone();
    outside.layers[0].cold_content_j_m2 += 2.0e-6;
    reseal(&mut outside);
    assert!(!equal(original, outside));
}

#[test]
fn density_is_exact_after_each_state_fingerprint_is_reconstructed() {
    let original = state();
    let mut changed = original.clone();
    changed.layers[0].density_kg_m3 = f64::from_bits(changed.layers[0].density_kg_m3.to_bits() + 1);
    reseal(&mut changed);
    assert!(!equal(original, changed));
}

#[test]
fn underrelaxation_retains_exact_candidate_density_while_damping_continuous_state() {
    let original = state();
    let mut candidate = original.clone();
    candidate.layers[0].density_kg_m3 =
        f64::from_bits(candidate.layers[0].density_kg_m3.to_bits() + 1);
    candidate.layers[0].thickness_m =
        candidate.layers[0].mass_swe_m * 1_000.0 / candidate.layers[0].density_kg_m3;
    reseal(&mut candidate);
    let relaxed = covered_fixed_point_stage3_underrelaxed_iterate_v1(
        &BTreeMap::from([(7, original.clone())]),
        &BTreeMap::from([(7, candidate.clone())]),
        0.25,
    )
    .expect("exact candidate density does not disable continuous damping");
    assert_eq!(
        relaxed[&7].layers[0].density_kg_m3.to_bits(),
        candidate.layers[0].density_kg_m3.to_bits(),
    );
    assert_eq!(
        relaxed[&7].layers[0].thickness_m.to_bits(),
        (relaxed[&7].layers[0].mass_swe_m * 1_000.0 / relaxed[&7].layers[0].density_kg_m3)
            .to_bits(),
    );
    assert_eq!(
        relaxed[&7].fingerprint,
        Wb11HydrologyKernel::stage3_persistent_state_fingerprint(&relaxed[&7]),
    );
    Wb11HydrologyKernel::validate_stage3_persistent_state(&relaxed[&7])
        .expect("relaxed exact-density state remains domain-valid");
    Wb11HydrologyKernel::validate_stage3_persistent_cumulative_closure(&relaxed[&7])
        .expect("relaxed exact-density state retains cumulative closure");
    assert_eq!(
        relaxed[&7].terminal_event_model,
        candidate.terminal_event_model,
    );
    assert!(!covered_fixed_point_stage3_states_equal(
        &BTreeMap::from([(7, original)]),
        &relaxed,
    ));
}

#[test]
fn relaxation_weight_keeps_raw_exact_floor_until_period_two_is_detected() {
    assert_eq!(
        covered_fixed_point_relaxation_weight_v1(59_999_999_999, true),
        None,
    );
    assert_eq!(
        covered_fixed_point_relaxation_weight_v1(60_000_000_000, false),
        None,
    );
    assert_eq!(
        covered_fixed_point_relaxation_weight_v1(60_000_000_000, true).map(f64::to_bits),
        Some(0.5_f64.to_bits()),
    );
    assert_eq!(
        covered_fixed_point_relaxation_weight_v1(120_000_000_000, false).map(f64::to_bits),
        Some(0.5_f64.to_bits()),
    );
    assert_eq!(
        covered_fixed_point_relaxation_weight_v1(480_000_000_000, false).map(f64::to_bits),
        Some(0.25_f64.to_bits()),
    );
    assert_eq!(
        covered_fixed_point_relaxation_weight_v1(1_800_000_000_000, false).map(f64::to_bits),
        Some(0.25_f64.to_bits()),
    );
}

#[test]
fn finalization_restart_reuses_guarded_support_scaled_stage3_contraction() {
    let mut original = state();
    original.layers[0].cold_content_j_m2 = 100.0;
    original.layers[0].temperature_c =
        Wb11HydrologyKernel::stage3_temperature_from_cold_content_values(
            original.layers[0].mass_swe_m,
            original.layers[0].cold_content_j_m2,
        );
    reseal(&mut original);
    let mut candidate = original.clone();
    candidate.layers[0].cold_content_j_m2 += 14.0e-6;
    candidate.layers[0].temperature_c =
        Wb11HydrologyKernel::stage3_temperature_from_cold_content_values(
            candidate.layers[0].mass_swe_m,
            candidate.layers[0].cold_content_j_m2,
        );
    reseal(&mut candidate);
    let original = BTreeMap::from([(7, original)]);
    let candidate = BTreeMap::from([(7, candidate)]);

    let restarted = covered_fixed_point_finalization_stage3_iterate_v1(
        &original,
        &candidate,
        420_000_000_000,
        false,
    );
    let expected = original[&7].layers[0].cold_content_j_m2
        + (2.0 / 7.0)
            * (candidate[&7].layers[0].cold_content_j_m2
                - original[&7].layers[0].cold_content_j_m2);
    assert_eq!(
        restarted[&7].layers[0].cold_content_j_m2.to_bits(),
        expected.to_bits(),
    );
    assert_eq!(
        restarted[&7].fingerprint,
        Wb11HydrologyKernel::stage3_persistent_state_fingerprint(&restarted[&7]),
    );
    Wb11HydrologyKernel::validate_stage3_persistent_state(&restarted[&7])
        .expect("finalization iterate remains domain-valid");
    Wb11HydrologyKernel::validate_stage3_persistent_cumulative_closure(&restarted[&7])
        .expect("finalization iterate retains cumulative closure");

    let mut density_candidate = candidate[&7].clone();
    density_candidate.layers[0].density_kg_m3 =
        f64::from_bits(density_candidate.layers[0].density_kg_m3.to_bits() + 1);
    density_candidate.layers[0].thickness_m = density_candidate.layers[0].mass_swe_m * 1_000.0
        / density_candidate.layers[0].density_kg_m3;
    reseal(&mut density_candidate);
    let density_candidate = BTreeMap::from([(7, density_candidate)]);
    let density_restarted = covered_fixed_point_finalization_stage3_iterate_v1(
        &original,
        &density_candidate,
        420_000_000_000,
        false,
    );
    assert_eq!(
        density_restarted[&7].layers[0].density_kg_m3.to_bits(),
        density_candidate[&7].layers[0].density_kg_m3.to_bits(),
        "authentic candidate density remains exact and is never interpolated",
    );
    assert!(!covered_fixed_point_stage3_states_equal(
        &original,
        &density_restarted,
    ));

    assert_eq!(
        covered_fixed_point_finalization_stage3_iterate_v1(
            &original,
            &candidate,
            60_000_000_000,
            false,
        ),
        candidate,
        "the exact floor remains raw before authentic period-two detection",
    );
}

#[test]
fn finalization_restart_declines_contraction_across_discrete_event_change() {
    let original = state();
    let mut candidate = original.clone();
    candidate.terminal_event_model = None;
    reseal(&mut candidate);
    let original = BTreeMap::from([(7, original)]);
    let candidate = BTreeMap::from([(7, candidate)]);

    assert_eq!(
        covered_fixed_point_finalization_stage3_iterate_v1(
            &original,
            &candidate,
            420_000_000_000,
            false,
        ),
        candidate,
        "a discrete change must retain the raw authentic candidate",
    );
}

#[test]
fn finalization_restart_requires_one_relaxed_picard_stabilization_crossing() {
    let mut stabilization = CoveredFinalizationStabilizationV1::default();
    stabilization.observe_restart(true);
    assert!(!stabilization.picard_accepts_convergence(false, true));
    assert!(
        stabilization.pending,
        "nonconvergence retains the pending seam"
    );
    assert!(!stabilization.picard_accepts_convergence(true, true));
    assert!(
        !stabilization.pending,
        "the first otherwise-converged relaxed crossing is consumed exactly once",
    );
    assert!(stabilization.picard_accepts_convergence(true, true));

    stabilization.observe_restart(false);
    assert!(stabilization.picard_accepts_convergence(true, false));
    assert!(
        !stabilization.pending,
        "a raw exact-floor restart cannot create a stabilization crossing",
    );
}

#[test]
fn raw_convergent_exact_floor_does_not_enable_period_two_relaxation() {
    let converged = BTreeMap::from([(7, state())]);
    assert!(!covered_fixed_point_exact_floor_period_two_detected_v1(
        Some(&converged),
        Some(&converged),
        &converged,
    ));
    assert_eq!(
        covered_fixed_point_relaxation_weight_v1(60_000_000_000, false),
        None,
    );
}

#[test]
fn authentic_density_period_two_enables_exact_floor_relaxation() {
    let state_a = state();
    let mut state_b = state_a.clone();
    state_b.layers[0].density_kg_m3 = f64::from_bits(state_b.layers[0].density_kg_m3.to_bits() + 1);
    state_b.layers[0].thickness_m =
        state_b.layers[0].mass_swe_m * 1_000.0 / state_b.layers[0].density_kg_m3;
    reseal(&mut state_b);
    let state_a = BTreeMap::from([(7, state_a)]);
    let state_b = BTreeMap::from([(7, state_b)]);

    assert!(covered_fixed_point_exact_floor_period_two_detected_v1(
        Some(&state_a),
        Some(&state_b),
        &state_a,
    ));
    assert_eq!(
        covered_fixed_point_relaxation_weight_v1(60_000_000_000, true).map(f64::to_bits),
        Some(0.5_f64.to_bits()),
    );
}

#[test]
fn period_two_detector_rejects_event_and_topology_poisons() {
    let state_a = state();
    let mut state_b = state_a.clone();
    state_b.layers[0].density_kg_m3 = f64::from_bits(state_b.layers[0].density_kg_m3.to_bits() + 1);
    state_b.layers[0].thickness_m =
        state_b.layers[0].mass_swe_m * 1_000.0 / state_b.layers[0].density_kg_m3;
    reseal(&mut state_b);
    let mut event_poison = state_a.clone();
    event_poison.terminal_event_model = None;
    reseal(&mut event_poison);
    let state_a = BTreeMap::from([(7, state_a)]);
    let state_b = BTreeMap::from([(7, state_b)]);

    assert!(!covered_fixed_point_exact_floor_period_two_detected_v1(
        Some(&state_a),
        Some(&state_b),
        &BTreeMap::from([(7, event_poison)]),
    ));
    assert!(!covered_fixed_point_exact_floor_period_two_detected_v1(
        Some(&state_a),
        Some(&state_b),
        &BTreeMap::from([(8, state_for(8))]),
    ));
}

#[test]
fn exact_floor_period_two_contraction_rejects_discrete_event_changes() {
    let original = state();
    let mut candidate = original.clone();
    candidate.layers[0].density_kg_m3 =
        f64::from_bits(candidate.layers[0].density_kg_m3.to_bits() + 1);
    candidate.layers[0].thickness_m =
        candidate.layers[0].mass_swe_m * 1_000.0 / candidate.layers[0].density_kg_m3;
    candidate.terminal_event_model = None;
    reseal(&mut candidate);

    assert!(covered_fixed_point_stage3_underrelaxed_iterate_v1(
        &BTreeMap::from([(7, original)]),
        &BTreeMap::from([(7, candidate)]),
        covered_fixed_point_relaxation_weight_v1(60_000_000_000, true)
            .expect("the exact fallback support has bounded contraction"),
    )
    .is_none());
}

#[test]
fn cumulative_mass_uses_its_area_mass_tolerance() {
    let original = state();
    let mut within = original.clone();
    within.cumulative_snowfall_kg_m2 += 0.5e-6;
    reseal(&mut within);
    assert!(equal(original.clone(), within));
    let mut outside = original.clone();
    outside.cumulative_snowfall_kg_m2 += 2.0e-6;
    reseal(&mut outside);
    assert!(!equal(original, outside));
}

#[test]
fn immutable_initial_mass_lineage_is_bitwise_exact() {
    let original = state();
    let mutations: [fn(&mut DirectSnowStage3PersistentState); 2] = [
        |state| state.initial_ice_kg_m2 = f64::from_bits(state.initial_ice_kg_m2.to_bits() + 1),
        |state| {
            state.initial_retained_liquid_kg_m2 =
                f64::from_bits(state.initial_retained_liquid_kg_m2.to_bits() + 1);
        },
    ];
    for mutate in mutations {
        let mut changed = original.clone();
        mutate(&mut changed);
        reseal(&mut changed);
        assert!(!equal(original.clone(), changed));
    }
}

#[test]
fn coherently_resealed_lane_aggregate_cannot_replace_destination_fold() {
    let reconstructed = [1.0, 2.0, 3.0, 4.0, 5.0];
    assert!(validate_destination_reconstruction_against_lane_aggregate(
        reconstructed,
        reconstructed
    )
    .is_ok());
    for index in 0..reconstructed.len() {
        let mut substituted = reconstructed;
        substituted[index] += 2.0e-6;
        assert!(validate_destination_reconstruction_against_lane_aggregate(
            reconstructed,
            substituted
        )
        .is_err());
    }
}

#[test]
fn coherent_interlayer_pair_substitution_fails_owner_state_reconstruction() {
    let accepted = reconstruct_interlayer_from_owner_states(40.0, 42.0, 2.0, -2.0)
        .expect("owner-state reconstructed interlayer transfer");
    assert_eq!(accepted, (2.0, -2.0));
    assert!(reconstruct_interlayer_from_owner_states(40.0, 42.0, 3.0, -3.0).is_err());
}

#[test]
fn lower_layer_refreeze_after_conduction_does_not_change_internal_transfer() {
    assert_eq!(
        reconstruct_interlayer_from_owner_states(40.0, 42.0, 2.0, -2.0)
            .expect("pre/post-conduction lower material snapshots"),
        (2.0, -2.0),
    );
}

#[test]
fn receipt_reconstruction_discriminator_reports_first_tolerance_failure_across_all_lanes() {
    let lane_7 = state_for(7);
    let lane_8 = state_for(8);
    let original = BTreeMap::from([(7, lane_7.clone()), (8, lane_8.clone())]);
    assert_eq!(
        covered_stage3_state_first_difference_v1(&original, &original),
        None
    );

    let mut changed_lane_8 = lane_8;
    changed_lane_8.layers[0].cold_content_j_m2 += 2.0e-6;
    reseal(&mut changed_lane_8);
    let changed = BTreeMap::from([(7, lane_7), (8, changed_lane_8.clone())]);
    let difference = covered_stage3_state_first_difference_v1(&original, &changed)
        .expect("second-lane physical substitution must be discriminated");
    assert_eq!(difference.0, 8);
    assert_eq!(difference.1, "layer.cold_content_j_m2");
    assert_eq!(
        difference.2,
        original[&8].layers[0].cold_content_j_m2.to_bits()
    );
    assert_eq!(
        difference.3,
        changed_lane_8.layers[0].cold_content_j_m2.to_bits()
    );
    assert_eq!(difference.4, original[&8].fingerprint);
    assert_eq!(difference.5, changed_lane_8.fingerprint);
}

#[test]
fn receipt_reconstruction_discriminator_does_not_alias_fingerprint_substitution() {
    let original = BTreeMap::from([(7, state())]);
    let mut changed = original.clone();
    changed.get_mut(&7).expect("lane").fingerprint ^= 1;
    let difference = covered_stage3_state_first_difference_v1(&original, &changed)
        .expect("fingerprint substitution must be discriminated");
    assert_eq!(difference.0, 7);
    assert_eq!(difference.1, "fingerprint");
    assert_ne!(difference.2, difference.3);
}

include!("open_snow_convergence_v44_tests.rs");
include!("open_snow_convergence_v45_tests.rs");
include!("open_snow_convergence_v46_tests.rs");
include!("open_snow_convergence_v56_tests.rs");

#[test]
fn v44_corrected_exchange_enters_single_stage3_map() {
    let interval_s = 60.0;
    let stale_forest = openwepp_land_surface_energy::WeightedTileEnergyOperands {
        tile_fraction: 0.38,
        local_input_j_m2_tile: 14_805.881_368_502_804,
        local_output_j_m2_tile: 14_382.380_685_603_006,
        local_storage_change_j_m2_tile: 0.0,
        local_sum_abs_integrated_components_j_m2_tile: 29_188.262_054_105_81,
    };
    let open = openwepp_land_surface_energy::WeightedTileEnergyOperands {
        tile_fraction: 0.62,
        local_input_j_m2_tile: 0.0,
        local_output_j_m2_tile: 0.0,
        local_storage_change_j_m2_tile: 0.0,
        local_sum_abs_integrated_components_j_m2_tile: 0.0,
    };
    assert!(openwepp_land_surface_energy::validate_weighted_ofe_energy(
        interval_s,
        &[stale_forest, open],
    )
    .is_err());
    let corrected_forest = openwepp_land_surface_energy::WeightedTileEnergyOperands {
        local_output_j_m2_tile: stale_forest.local_input_j_m2_tile,
        ..stale_forest
    };
    let strict_closure = openwepp_land_surface_energy::validate_weighted_ofe_energy(
        interval_s,
        &[corrected_forest, open],
    )
    .expect("unchanged strict weighted-OFE closure after corrected exchange");
    assert_eq!(
        strict_closure.reconstructed_residual.to_bits(),
        0.0_f64.to_bits()
    );

    let mut budget = CoveredPhysicalEvaluationBudgetV1::new(10).expect("shared budget");
    let stage3_calls = std::cell::Cell::new(0_usize);
    let result = covered_phase_consistent_finalization_equivalent_map_v1(&mut budget, || {
        assert_eq!(
            strict_closure.reconstructed_residual.to_bits(),
            0.0_f64.to_bits()
        );
        stage3_calls.set(stage3_calls.get() + 1);
        Ok(77_u32)
    })
    .expect("charged corrected-exchange map");
    result.validate().expect("one Stage 3 map posture");
    assert_eq!(result.value, 77);
    assert_eq!(stage3_calls.get(), 1);
    assert_eq!(budget.used, 11);
}

#[test]
fn v44_projected_soil_coordinate_is_consumed_once_by_cn_not_v8() {
    let use_posture = CoveredPhaseConsistentProjectedSoilConsumptionV1::SnowSoilCnOnly;
    let coordinates = [0.31, -1.0, 100.0, 8_000.0, 269.75];
    let receipts = v35_authentic_receipt_stabilization_vectors::receipt_set(273.0);
    let receipt = &receipts[&7];
    let retained_v8_artifacts = v35_authentic_receipt_stabilization_vectors::artifact(44);
    let retained_v8_before = retained_v8_artifacts.clone();
    let cn_trial = use_posture
        .cn_trial_operand(&coordinates, 1, 0, receipt, 271.25, 60.0)
        .expect("sealed projected-coordinate CN operand");
    cn_trial
        .validate_against(receipt)
        .expect("CN trial retains authentic receipt custody");
    let (_, _, expected_heat_w_m2) =
        crate::snow_stage3_v11_attachment::snow_soil_heat_w_m2_ofe_ground(
            receipt.bottom_snow_half_thickness_m,
            receipt.bottom_snow_conductivity_w_m_k,
            receipt.top_soil_half_thickness_m,
            receipt.top_soil_conductivity_w_m_k,
            receipt.beginning_bottom_snow_temperature_k,
            receipt.beginning_top_soil_temperature_k,
            271.25,
            coordinates[4],
        )
        .expect("independent CN reconstruction");
    assert_eq!(
        cn_trial.snow_candidate_heat_j_m2_ofe_ground.to_bits(),
        (-expected_heat_w_m2 * 60.0).to_bits()
    );
    covered_phase_consistent_projected_soil_exact_once_v1(&[7], &[7], &[])
        .expect("one CN use and no Stage3-covered V8 soil-energy use");
    assert_eq!(
        covered_phase_consistent_projected_soil_exact_once_v1(&[7], &[], &[]),
        Err(PhaseConsistentCoupledSolveErrorV1::SideConstraint)
    );
    assert_eq!(
        covered_phase_consistent_projected_soil_exact_once_v1(&[7], &[7], &[7]),
        Err(PhaseConsistentCoupledSolveErrorV1::SideConstraint)
    );
    assert_eq!(
        covered_phase_consistent_projected_soil_exact_once_v1(&[7, 8], &[8, 7], &[]),
        Err(PhaseConsistentCoupledSolveErrorV1::SideConstraint)
    );
    assert_eq!(retained_v8_artifacts, retained_v8_before);
    assert_eq!(
        use_posture
            .cn_temperature_coordinate(&coordinates, 1, 0)
            .expect("top-soil CN coordinate")
            .to_bits(),
        269.75_f64.to_bits()
    );
    let mut changed = coordinates;
    changed[4] = f64::from_bits(changed[4].to_bits() + 1);
    let changed_trial = use_posture
        .cn_trial_operand(&changed, 1, 0, receipt, 271.25, 60.0)
        .expect("changed projected-coordinate CN operand");
    assert_ne!(
        cn_trial.snow_candidate_heat_j_m2_ofe_ground.to_bits(),
        changed_trial.snow_candidate_heat_j_m2_ofe_ground.to_bits()
    );
    changed[4] = f64::NAN;
    assert_eq!(
        use_posture.cn_trial_operand(&changed, 1, 0, receipt, 271.25, 60.0),
        Err(PhaseConsistentCoupledSolveErrorV1::NonFinite)
    );
    assert_eq!(
        use_posture.cn_trial_operand(&coordinates[..4], 1, 0, receipt, 271.25, 60.0),
        Err(PhaseConsistentCoupledSolveErrorV1::Structure)
    );
}

#[test]
fn v44_deferred_private_closure_cannot_admit_or_publish() {
    let private = covered_phase_consistent_carrier_closure_posture_v1(
        CoveredPhaseConsistentPhysicalEvaluationKindV1::PrivateTrial,
    );
    assert!(!private.requires_strict_weighted_ofe_closure());
    assert!(!CoveredConvergenceAdmissionV1::CoupledAuthentic.admits(false, true, false,));
    assert!(!CoveredConvergenceAdmissionV1::Picard.admits(false, true, true,));
    let strict = covered_phase_consistent_carrier_closure_posture_v1(
        CoveredPhaseConsistentPhysicalEvaluationKindV1::ReceiptStabilizationReplay,
    );
    assert!(strict.requires_strict_weighted_ofe_closure());
    assert!(CoveredConvergenceAdmissionV1::CoupledAuthentic.admits(false, true, true,));

    let receipts = v35_authentic_receipt_stabilization_vectors::receipt_set(273.0);
    let retained_artifact = v35_authentic_receipt_stabilization_vectors::artifact(440);
    let retained_before = retained_artifact.clone();
    let mut budget = CoveredPhysicalEvaluationBudgetV1::new(8).expect("shared budget");
    let stabilized = covered_authentic_receipt_stabilize_v1(
        receipts.clone(),
        &mut budget,
        |kind, input, budget| {
            assert!(covered_phase_consistent_carrier_closure_posture_v1(kind)
                .requires_strict_weighted_ofe_closure());
            assert!(covered_snow_soil_receipt_sets_exact_v1(input, &receipts));
            v35_authentic_receipt_stabilization_vectors::charged_result(
                budget,
                v35_authentic_receipt_stabilization_vectors::residual(0.25),
                retained_artifact.clone(),
                receipts.clone(),
            )
        },
    )
    .expect("strict stable receipt probe and independent replay");
    assert_eq!(stabilized.independent_replay_count, 1);
    assert!(!stabilized.publication_eligible);
    assert_eq!(stabilized.artifacts, retained_before);

    let rollback_artifact = v35_authentic_receipt_stabilization_vectors::artifact(441);
    let rollback_before = rollback_artifact.clone();
    let mut budget = CoveredPhysicalEvaluationBudgetV1::new(8).expect("shared budget");
    let refused =
        covered_authentic_receipt_stabilize_v1(receipts.clone(), &mut budget, |_, _, budget| {
            let mut residual = v35_authentic_receipt_stabilization_vectors::residual(0.25);
            residual.algebraic_side_constraints_satisfied = false;
            v35_authentic_receipt_stabilization_vectors::charged_result(
                budget,
                residual,
                rollback_artifact.clone(),
                receipts.clone(),
            )
        });
    assert_eq!(
        refused,
        Err(PhaseConsistentCoupledSolveErrorV1::SideConstraint)
    );
    assert_eq!(rollback_artifact, rollback_before);
}
