fn v52_one_lane_base_residual() -> CoveredPhaseConsistentResidualEvaluationV1 {
    let water_kg_m2 = 0.327_290_935_6;
    let enthalpy_j_m2 = -1_000.0;
    let density_kg_m3 = 100.0;
    covered_phase_consistent_residual_assemble_v1(CoveredPhaseConsistentResidualInputsV1 {
        coordinates: vec![water_kg_m2, enthalpy_j_m2, density_kg_m3, 500.0, 263.2],
        beginning_snow_water_kg_m2: vec![water_kg_m2],
        beginning_snow_enthalpy_j_m2: vec![enthalpy_j_m2],
        physical_delta_water_kg_m2: vec![0.0],
        physical_complete_energy_j_m2: vec![0.0],
        physical_ice_kg_m2: vec![water_kg_m2],
        physical_density_kg_m3: vec![density_kg_m3],
        physical_thickness_m: vec![water_kg_m2 / density_kg_m3],
        exact_density_settling_branch_satisfied: vec![true],
        beginning_soil_enthalpy_j_m2: vec![500.0],
        physical_soil_delta_energy_j_m2: vec![0.0],
        owner_soil_temperature_k: vec![263.2],
        absolute_tolerances: vec![1.0e-9, 1.0e-6, 1.0e-6, 1.0e-6, 1.0e-9],
        algebraic_side_constraints_satisfied: true,
    })
    .expect("valid V52 base residual")
}

fn v52_one_lane_q_residual(
    coordinate_q_j_m2: f64,
    physical_q_j_m2: f64,
) -> Result<CoveredPhaseConsistentResidualEvaluationV1, PhaseConsistentCoupledSolveErrorV1> {
    covered_cn_heat_coordinate_residual_evaluate_v1(
        v52_one_lane_base_residual(),
        vec![
            0.327_290_935_6,
            -1_000.0,
            100.0,
            coordinate_q_j_m2,
            500.0,
            263.2,
        ],
        &[coordinate_q_j_m2],
        &[physical_q_j_m2],
        vec![1.0e-9, 1.0e-6, 1.0e-6, 1.0e-6, 1.0e-6, 1.0e-9],
    )
}

fn v52_two_lane_base_residual() -> CoveredPhaseConsistentResidualEvaluationV1 {
    CoveredPhaseConsistentResidualEvaluationV1 {
        coordinates: vec![0.3, -1_000.0, 100.0, 0.4, -2_000.0, 120.0, 500.0, 263.2],
        residuals: vec![0.0; 8],
        absolute_tolerances: vec![1.0; 8],
        r_w_kg_m2: vec![0.0; 2],
        r_h_j_m2: vec![0.0; 2],
        r_rho_kg_m3: vec![0.0; 2],
        r_q_cn_j_m2: Vec::new(),
        physical_q_cn_j_m2: Vec::new(),
        derived_thickness_closures: vec![
            CoveredDerivedThicknessClosureV1 {
                proposed_z_m: 0.003,
                physical_z_m: 0.003,
                r_z_m: 0.0,
                scaled_merit: 0.0,
            },
            CoveredDerivedThicknessClosureV1 {
                proposed_z_m: 0.4 / 120.0,
                physical_z_m: 0.4 / 120.0,
                r_z_m: 0.0,
                scaled_merit: 0.0,
            },
        ],
        r_e_j_m2: vec![0.0],
        r_t_k: vec![0.0],
        scaled_merit: 0.0,
        derived_constraints_scaled_merit: 0.0,
        algebraic_side_constraints_satisfied: true,
    }
}

fn v52_charged_finalization_equivalent_result(
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
    let finalization_inputs = CoveredFinalizationEquivalentReplayInputsV1 {
        proposed_stage3: artifacts.stage3_candidate.clone(),
        proposed_soil: artifacts.soil_candidate.clone(),
        input_covered_boundaries: BTreeMap::new(),
        input_open_boundaries: BTreeMap::new(),
        destination_receipts: BTreeMap::new(),
    };
    let charged = covered_phase_consistent_finalization_equivalent_map_v1(budget, || {
        Ok((residual, artifacts, finalization_inputs, receipts))
    })?;
    charged.validate()?;
    Ok(charged.value)
}

#[test]
fn v52_cn_heat_coordinate_closes_r134_receipt_two_cycle() {
    let r134_left = 5_340.494_294_593_449;
    let r134_right = 5_340.494_294_593_433;
    let open = v52_one_lane_q_residual(r134_left, r134_right).expect("one-ULP-class Q residual");
    assert_ne!(open.r_q_cn_j_m2[0].to_bits(), 0.0_f64.to_bits());
    assert_eq!(open.residuals[3].to_bits(), open.r_q_cn_j_m2[0].to_bits());
    assert!(open.scaled_merit > 0.0);

    let closed = v52_one_lane_q_residual(r134_right, r134_right).expect("exact Q closure");
    assert_eq!(closed.r_q_cn_j_m2, vec![0.0]);
    assert!(covered_phase_consistent_residual_is_exact_zero_v1(&closed));
    assert!(!covered_phase_consistent_residual_exact_v1(&open, &closed));
}

#[test]
fn v52_cn_heat_coordinate_preserves_zero_heat_and_order() {
    let evaluation = covered_cn_heat_coordinate_residual_evaluate_v1(
        v52_two_lane_base_residual(),
        vec![
            0.3, -1_000.0, 100.0, 0.0, 0.4, -2_000.0, 120.0, 20.0, 500.0, 263.2,
        ],
        &[0.0, 20.0],
        &[0.0, 15.0],
        vec![1.0; 10],
    )
    .expect("ordered two-lane V52 residual");
    assert_eq!(evaluation.r_q_cn_j_m2, vec![0.0, 5.0]);
    assert_eq!(evaluation.residuals[3].to_bits(), 0.0_f64.to_bits());
    assert_eq!(evaluation.residuals[7].to_bits(), 5.0_f64.to_bits());
    assert_eq!(evaluation.coordinates[8..], [500.0, 263.2]);

    let reordered = covered_cn_heat_coordinate_residual_evaluate_v1(
        v52_two_lane_base_residual(),
        vec![
            0.3, -1_000.0, 100.0, 20.0, 0.4, -2_000.0, 120.0, 0.0, 500.0, 263.2,
        ],
        &[20.0, 0.0],
        &[0.0, 20.0],
        vec![1.0; 10],
    )
    .expect("finite reordered lane Q poison");
    assert_eq!(reordered.r_q_cn_j_m2, vec![20.0, -20.0]);
    assert_eq!(
        covered_cn_heat_coordinate_residual_evaluate_v1(
            v52_two_lane_base_residual(),
            vec![0.3, -1_000.0, 100.0, 0.0, 500.0, 263.2],
            &[0.0],
            &[0.0],
            vec![1.0; 6],
        ),
        Err(PhaseConsistentCoupledSolveErrorV1::Structure),
    );
}

#[test]
fn v52_cn_heat_coordinate_refuses_omission_sign_and_static_geometry_poison() {
    let base = v52_one_lane_base_residual();
    assert_eq!(
        covered_cn_heat_coordinate_residual_evaluate_v1(
            base,
            vec![0.327_290_935_6, -1_000.0, 100.0, 10.0, 500.0, 263.2],
            &[],
            &[10.0],
            vec![1.0; 6],
        ),
        Err(PhaseConsistentCoupledSolveErrorV1::Structure),
    );
    let sign_poison = v52_one_lane_q_residual(10.0, -10.0).expect("finite sign poison");
    assert_eq!(sign_poison.r_q_cn_j_m2, vec![20.0]);
    assert!(sign_poison.scaled_merit > 1.0);
    assert_eq!(
        v52_one_lane_q_residual(-0.0, 0.0),
        Err(PhaseConsistentCoupledSolveErrorV1::SideConstraint),
    );
    assert_eq!(
        v52_one_lane_q_residual(f64::NAN, 0.0),
        Err(PhaseConsistentCoupledSolveErrorV1::NonFinite),
    );
    let one_application = v52_one_lane_q_residual(10.0, 10.0).expect("one Q application");
    let wrong_sign = v52_one_lane_q_residual(-10.0, 10.0).expect("wrong-sign Q poison");
    let duration_double = v52_one_lane_q_residual(20.0, 10.0).expect("double Q poison");
    assert_eq!(one_application.r_q_cn_j_m2, vec![0.0]);
    assert_eq!(wrong_sign.r_q_cn_j_m2, vec![-20.0]);
    assert_eq!(duration_double.r_q_cn_j_m2, vec![10.0]);

    let receipts = v35_authentic_receipt_stabilization_vectors::receipt_set(273.0);
    let receipt = receipts.get(&7).expect("sealed receipt");
    let posture = CoveredPhaseConsistentProjectedSoilConsumptionV1::SnowSoilCnOnly;
    let coordinate = vec![0.3, -1_000.0, 100.0, 10.0, 500.0, 263.2];
    let trial = posture
        .cn_heat_coordinate_trial_operand(&coordinate, 0, receipt)
        .expect("exact static receipt joins");
    assert_eq!(trial.snow_candidate_heat_j_m2_ofe_ground, 10.0);
    let mut foreign_geometry = receipt.clone();
    foreign_geometry.topology_identity_sha256 = Digest32::from_bytes([0x52; 32]);
    assert_eq!(
        posture.cn_heat_coordinate_trial_operand(&coordinate, 0, &foreign_geometry),
        Err(PhaseConsistentCoupledSolveErrorV1::SideConstraint),
    );
}

#[test]
fn v52_cn_heat_coordinate_retains_shared_budget_exact_receipt_replay_and_rollback() {
    let receipts = v35_authentic_receipt_stabilization_vectors::receipt_set(273.0);
    let artifact = v35_authentic_receipt_stabilization_vectors::artifact(52);
    let artifact_before = artifact.clone();
    let residual = v52_one_lane_q_residual(10.0, 10.0).expect("closed Q residual");
    let mut budget = CoveredPhysicalEvaluationBudgetV1::new(94).expect("shared budget");
    let stabilized = covered_authentic_receipt_stabilize_v1(
        receipts.clone(),
        &mut budget,
        |kind, input, budget| {
            assert!(matches!(
                kind,
                CoveredPhaseConsistentPhysicalEvaluationKindV1::ReceiptStabilizationProbe
                    | CoveredPhaseConsistentPhysicalEvaluationKindV1::ReceiptStabilizationReplay
            ));
            assert!(covered_snow_soil_receipt_sets_exact_v1(input, &receipts));
            v52_charged_finalization_equivalent_result(
                budget,
                residual.clone(),
                artifact.clone(),
                receipts.clone(),
            )
        },
    )
    .expect("exact receipt probe and protected replay");
    assert_eq!(budget.used, COVERED_PHYSICAL_EVALUATION_LIMIT_V1);
    assert_eq!(stabilized.stabilization_probe_count, 1);
    assert_eq!(stabilized.independent_replay_count, 1);
    assert!(!stabilized.publication_eligible);
    assert_eq!(artifact, artifact_before);

    for dimension in [6, 10] {
        let exact_fit_used = COVERED_PHYSICAL_EVALUATION_LIMIT_V1 - (dimension + 1 + 2);
        let exact_fit = CoveredPhysicalEvaluationBudgetV1::new(exact_fit_used)
            .expect("exact-fit shared budget");
        let capacity = covered_safeguarded_complete_step_capacity_v1(dimension, &exact_fit, 2)
            .expect("complete V52 Jacobian plus trust trial and receipt reserve");
        assert_eq!(capacity.dimension, dimension);
        assert_eq!(capacity.minimum_physical_evaluations, dimension + 1);
        let one_short = CoveredPhysicalEvaluationBudgetV1::new(exact_fit_used + 1)
            .expect("one-short shared budget");
        assert_eq!(
            covered_safeguarded_complete_step_capacity_v1(dimension, &one_short, 2),
            Err(PhaseConsistentCoupledSolveErrorV1::EvaluationBudget),
        );
    }
}

#[test]
fn v52_cn_heat_coordinate_never_repairs_or_publishes_receipts() {
    let receipts = v35_authentic_receipt_stabilization_vectors::receipt_set(273.0);
    let receipt = receipts.get(&7).expect("sealed authentic receipt");
    let receipt_before = receipt.clone();
    let coordinate = vec![0.3, -1_000.0, 100.0, 99.0, 500.0, 263.2];
    let coordinate_trial = CoveredPhaseConsistentProjectedSoilConsumptionV1::SnowSoilCnOnly
        .cn_heat_coordinate_trial_operand(&coordinate, 0, receipt)
        .expect("private coordinate trial");
    let private_consumption = covered_phase_consistent_cn_consumption_v1(
        CoveredPhaseConsistentPhysicalEvaluationKindV1::PrivateTrial,
        Some(&coordinate_trial),
        Some(receipt),
    )
    .expect("PrivateTrial consumes coordinate Q");
    assert_eq!(
        private_consumption
            .snow_candidate_heat_j_m2_ofe_ground
            .to_bits(),
        99.0_f64.to_bits(),
    );
    assert_eq!(
        private_consumption
            .soil_candidate_heat_j_m2_ofe_ground
            .to_bits(),
        (-99.0_f64).to_bits(),
    );
    for authentic_kind in [
        CoveredPhaseConsistentPhysicalEvaluationKindV1::ReceiptStabilizationProbe,
        CoveredPhaseConsistentPhysicalEvaluationKindV1::ReceiptStabilizationReplay,
    ] {
        let authentic = covered_phase_consistent_cn_consumption_v1(
            authentic_kind,
            Some(&coordinate_trial),
            Some(receipt),
        )
        .expect("authentic posture consumes sealed receipt Q");
        assert_eq!(
            authentic.snow_candidate_heat_j_m2_ofe_ground.to_bits(),
            receipt.snow_candidate_heat_j_m2_ofe_ground.to_bits(),
        );
        assert_eq!(
            authentic.soil_candidate_heat_j_m2_ofe_ground.to_bits(),
            receipt.soil_candidate_heat_j_m2_ofe_ground.to_bits(),
        );
    }
    assert_ne!(
        coordinate_trial
            .snow_candidate_heat_j_m2_ofe_ground
            .to_bits(),
        receipt.snow_candidate_heat_j_m2_ofe_ground.to_bits(),
    );
    assert_eq!(
        receipt.snow_candidate_heat_j_m2_ofe_ground.to_bits(),
        receipt_before.snow_candidate_heat_j_m2_ofe_ground.to_bits(),
        "authentic probe/replay input remains the sealed receipt Q",
    );
    assert_eq!(
        receipt, &receipt_before,
        "coordinate construction cannot reseal input"
    );
    assert_eq!(
        covered_phase_consistent_cn_consumption_v1(
            CoveredPhaseConsistentPhysicalEvaluationKindV1::ReceiptStabilizationProbe,
            None,
            Some(receipt),
        ),
        Err(PhaseConsistentCoupledSolveErrorV1::Structure),
    );
    let mut foreign_static_receipt = receipt.clone();
    foreign_static_receipt.lane_id += 1;
    foreign_static_receipt = foreign_static_receipt
        .seal()
        .expect("independently sealed foreign lane receipt");
    assert_eq!(
        covered_phase_consistent_cn_consumption_v1(
            CoveredPhaseConsistentPhysicalEvaluationKindV1::ReceiptStabilizationReplay,
            Some(&coordinate_trial),
            Some(&foreign_static_receipt),
        ),
        Err(PhaseConsistentCoupledSolveErrorV1::SideConstraint),
    );
    assert!(
        CoveredPhaseConsistentPhysicalEvaluationKindV1::ReceiptStabilizationProbe
            .requires_authentic_receipts()
    );
    assert!(
        CoveredPhaseConsistentPhysicalEvaluationKindV1::ReceiptStabilizationReplay
            .requires_authentic_receipts()
    );
    assert!(!covered_phase_consistent_carrier_closure_posture_v1(
        CoveredPhaseConsistentPhysicalEvaluationKindV1::PrivateTrial,
    )
    .requires_strict_weighted_ofe_closure());
    assert!(!CoveredConvergenceAdmissionV1::CoupledAuthentic.admits(false, true, false));

    let next_temperature = f64::from_bits(273.0_f64.to_bits() + 1);
    let replay_temperature = f64::from_bits(273.0_f64.to_bits() + 2);
    let next = v35_authentic_receipt_stabilization_vectors::receipt_set(next_temperature);
    let replay_substitution =
        v35_authentic_receipt_stabilization_vectors::receipt_set(replay_temperature);
    assert_ne!(
        next[&7].receipt_sha256, replay_substitution[&7].receipt_sha256,
        "one-ULP endpoint changes the sealed receipt digest",
    );
    let artifact = v35_authentic_receipt_stabilization_vectors::artifact(53);
    let residual = v52_one_lane_q_residual(10.0, 10.0).expect("closed Q residual");
    let mut calls = 0usize;
    let mut budget = CoveredPhysicalEvaluationBudgetV1::new(90).expect("shared budget");
    let mismatch = covered_authentic_receipt_stabilize_v1(
        receipts.clone(),
        &mut budget,
        |kind, input, budget| {
            let output = match calls {
                0 => {
                    assert_eq!(
                        kind,
                        CoveredPhaseConsistentPhysicalEvaluationKindV1::ReceiptStabilizationProbe
                    );
                    assert!(covered_snow_soil_receipt_sets_exact_v1(input, &receipts));
                    next.clone()
                }
                1 => {
                    assert_eq!(
                        kind,
                        CoveredPhaseConsistentPhysicalEvaluationKindV1::ReceiptStabilizationProbe
                    );
                    assert!(covered_snow_soil_receipt_sets_exact_v1(input, &next));
                    next.clone()
                }
                _ => {
                    assert_eq!(
                        kind,
                        CoveredPhaseConsistentPhysicalEvaluationKindV1::ReceiptStabilizationReplay
                    );
                    assert!(covered_snow_soil_receipt_sets_exact_v1(input, &next));
                    replay_substitution.clone()
                }
            };
            calls += 1;
            v52_charged_finalization_equivalent_result(
                budget,
                residual.clone(),
                artifact.clone(),
                output,
            )
        },
    );
    assert_eq!(
        mismatch,
        Err(PhaseConsistentCoupledSolveErrorV1::ReplayMismatch)
    );
    assert_eq!(calls, 3);
    assert_eq!(
        receipt, &receipt_before,
        "failed replay rolls authentic input back"
    );

    let persisted_restart_source = include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../openwepp-persisted-restart-v1/src/snow_stage3_v11.rs"
    ));
    assert!(!persisted_restart_source.contains("r_q_cn_j_m2"));
    assert!(!persisted_restart_source.contains("snow_candidate_cn_heat_j_m2"));
}

include!("open_snow_convergence_v53_tests.rs");
