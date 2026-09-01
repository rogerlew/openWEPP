fn v54_receipts(ending_temperature_k: f64) -> BTreeMap<u32, SnowSoilHeatReceiptV1> {
    v35_authentic_receipt_stabilization_vectors::receipt_set(ending_temperature_k)
}

fn v54_probe(
    input_receipts: BTreeMap<u32, SnowSoilHeatReceiptV1>,
    reconstructed_receipts: BTreeMap<u32, SnowSoilHeatReceiptV1>,
    marker: u128,
) -> CoveredAuthenticReceiptStabilizationProbeV1 {
    let artifacts = v35_authentic_receipt_stabilization_vectors::artifact(marker);
    CoveredAuthenticReceiptStabilizationProbeV1 {
        input_receipts,
        residual: v52_one_lane_q_residual(
            reconstructed_receipts[&7].snow_candidate_heat_j_m2_ofe_ground,
            reconstructed_receipts[&7].snow_candidate_heat_j_m2_ofe_ground,
        )
        .expect("closed cycle-member Q residual"),
        finalization_inputs: CoveredFinalizationEquivalentReplayInputsV1 {
            proposed_stage3: artifacts.stage3_candidate.clone(),
            proposed_soil: artifacts.soil_candidate.clone(),
            input_covered_boundaries: BTreeMap::new(),
            input_open_boundaries: BTreeMap::new(),
            destination_receipts: BTreeMap::new(),
        },
        artifacts,
        reconstructed_receipts,
    }
}

fn v54_v2_probe(
    input_receipts: BTreeMap<u32, SnowSoilHeatReceiptV1>,
    reconstructed_receipts: BTreeMap<u32, SnowSoilHeatReceiptV1>,
    marker: u128,
) -> CoveredAuthenticReceiptStabilizationProbeV1 {
    let mut probe = v54_probe(input_receipts, reconstructed_receipts, marker);
    let v1 = probe
        .artifacts
        .soil_candidate
        .v1()
        .expect("V35 physical soil fixture")
        .clone();
    let migrated = openwepp_land_surface_energy::migrate_soil_thermal_v1_to_v2(
        &v1,
        openwepp_land_surface_energy::SoilThermalV2MigrationIdentity {
            model_version: "v54-model".to_owned(),
            model_definition_sha256: Sha256Digest::try_new("a".repeat(64)).expect("digest"),
            run_id: "v54-cycle-projection".to_owned(),
            transaction_id: TransactionId(90),
            support_start_ns: 0,
            support_end_ns: 60_000_000_000,
            receipt_chain_sha256: Sha256Digest::try_new("b".repeat(64)).expect("digest"),
        },
    )
    .expect("checked V54 V1-to-V2 migration");
    let prepared = openwepp_land_surface_energy::prepare_soil_thermal_support_v2(
        &migrated,
        TransactionId(91),
        60_000_000_000,
        120_000_000_000,
    )
    .expect("prepared V54 V2 support");
    let top = &prepared.beginning_owner().state.ofes[0].ordered_layers[0];
    let operands = vec![
        openwepp_land_surface_energy::SoilThermalAcceptedEnergyOperandV2 {
            ofe_id: prepared.beginning_owner().state.ofes[0].ofe_id.clone(),
            layer_id: top.layer_id.clone(),
            source_kind: openwepp_land_surface_energy::SoilThermalEnergyOperandKindV2::SoilInternal,
            source_owner_id: ResourceOwnerId::try_new("v54-cycle-energy").expect("owner"),
            debit_credit_identity_sha256: Sha256Digest::try_new("c".repeat(64)).expect("digest"),
            ordinal: 0,
            units: "J m^-2 OFE-ground".to_owned(),
            basis: "ofe_ground".to_owned(),
            energy_j_m2_ofe_ground: f64::from_bits(1),
        },
    ];
    let projections = prepared
        .beginning_owner()
        .state
        .ofes
        .iter()
        .flat_map(|ofe| {
            ofe.ordered_layers.iter().map(|layer| {
                openwepp_land_surface_energy::SoilThermalTemperatureProjectionV2 {
                    ofe_id: ofe.ofe_id.clone(),
                    layer_id: layer.layer_id.clone(),
                    heat_capacity_j_m2_k: 1.0,
                    ending_temperature_k: layer.temperature_k,
                }
            })
        })
        .collect::<Vec<_>>();
    let trial = openwepp_land_surface_energy::advance_soil_thermal_trial_v2(
        &prepared,
        &operands,
        &projections,
    )
    .expect("nonzero-carry V54 physical trial");
    assert_ne!(
        trial.ending_state().ofes[0].ordered_layers[0].enthalpy_carry,
        openwepp_land_surface_energy::ExactDyadicEnthalpy::zero(),
        "sub-ULP physical energy is retained in the exact carry",
    );
    probe.artifacts.soil_candidate =
        DirectSoilThermalCandidate::from_v2(trial).expect("V54 V2 candidate");
    probe.finalization_inputs.proposed_soil = probe.artifacts.soil_candidate.clone();
    probe
}

fn v54_branch() -> CoveredPhaseConsistentPhysicalBranchIdentityV1 {
    CoveredPhaseConsistentPhysicalBranchIdentityV1 {
        phase_branch: vec![0],
        density_model_branch: vec![7],
    }
}

fn v54_evaluation(
    coordinates: &[f64],
    ordinal: usize,
    marker: u128,
    output_q_j_m2: f64,
) -> CoveredPhaseConsistentPhysicalEvaluationV1 {
    let artifacts = v35_authentic_receipt_stabilization_vectors::artifact(marker);
    let residual = v52_one_lane_q_residual(coordinates[3], output_q_j_m2)
        .expect("finite V54 physical residual");
    CoveredPhaseConsistentPhysicalEvaluationV1 {
        residual,
        finalization_inputs: CoveredFinalizationEquivalentReplayInputsV1 {
            proposed_stage3: artifacts.stage3_candidate.clone(),
            proposed_soil: artifacts.soil_candidate.clone(),
            input_covered_boundaries: BTreeMap::new(),
            input_open_boundaries: BTreeMap::new(),
            destination_receipts: BTreeMap::new(),
        },
        artifacts,
        branch_identity: v54_branch(),
        coordinate_posture: CoveredPhaseConsistentCoordinatePostureV1::EnthalpyPrimaryWithCnHeat,
        physical_evaluation_ordinal: ordinal,
    }
}

#[test]
fn v54_exact_cycle_chronology_retains_first_seen_members() {
    let r0 = v54_receipts(263.0);
    let r1 = v54_receipts(263.125);
    let r2 = v54_receipts(263.25);
    let mut budget = CoveredPhysicalEvaluationBudgetV1::new(80).expect("shared budget");
    let mut observed = Vec::new();
    let outcome = covered_authentic_receipt_stabilize_or_cycle_v1(
        r0.clone(),
        &mut budget,
        |kind, input, budget| {
            observed.push((kind, input.clone()));
            let (marker, next) = if covered_snow_soil_receipt_sets_exact_v1(input, &r0) {
                (1, r1.clone())
            } else if covered_snow_soil_receipt_sets_exact_v1(input, &r1) {
                (2, r2.clone())
            } else {
                (3, r1.clone())
            };
            v52_charged_finalization_equivalent_result(
                budget,
                v52_one_lane_q_residual(
                    next[&7].snow_candidate_heat_j_m2_ofe_ground,
                    next[&7].snow_candidate_heat_j_m2_ofe_ground,
                )
                .expect("closed probe residual"),
                v35_authentic_receipt_stabilization_vectors::artifact(marker),
                next,
            )
        },
    )
    .expect("exact two-member receipt cycle is retained, not admitted");
    let CoveredAuthenticReceiptStabilizationOutcomeV1::ExactCycle(cycle) = outcome else {
        panic!("expected an exact receipt cycle");
    };
    assert_eq!(cycle.discovery_probe_count, 3);
    assert_eq!(cycle.members.len(), 2);
    assert!(!cycle.publication_eligible);
    assert!(covered_snow_soil_receipt_sets_exact_v1(
        &cycle.members[0].reconstructed_receipts,
        &r1,
    ));
    assert!(covered_snow_soil_receipt_sets_exact_v1(
        &cycle.members[1].reconstructed_receipts,
        &r2,
    ));
    assert_eq!(cycle.members[0].artifacts.transaction_id, TransactionId(1));
    assert_eq!(cycle.members[1].artifacts.transaction_id, TransactionId(2));
    assert_eq!(
        cycle.members[0].finalization_inputs.proposed_soil,
        cycle.members[0].artifacts.soil_candidate,
    );
    assert_eq!(
        cycle.members[1].finalization_inputs.proposed_soil,
        cycle.members[1].artifacts.soil_candidate,
    );
    assert!(covered_snow_soil_receipt_sets_exact_v1(&observed[0].1, &r0));
    assert!(covered_snow_soil_receipt_sets_exact_v1(&observed[1].1, &r1));
    assert!(covered_snow_soil_receipt_sets_exact_v1(&observed[2].1, &r2));

    let r3 = v54_receipts(263.375);
    let mut overlong_budget = CoveredPhysicalEvaluationBudgetV1::new(70).expect("shared budget");
    let overlong = covered_authentic_receipt_stabilize_or_cycle_v1(
        r0.clone(),
        &mut overlong_budget,
        |_, input, budget| {
            let next = if covered_snow_soil_receipt_sets_exact_v1(input, &r0) {
                r1.clone()
            } else if covered_snow_soil_receipt_sets_exact_v1(input, &r1) {
                r2.clone()
            } else if covered_snow_soil_receipt_sets_exact_v1(input, &r2) {
                r3.clone()
            } else {
                r0.clone()
            };
            v52_charged_finalization_equivalent_result(
                budget,
                v35_authentic_receipt_stabilization_vectors::residual(0.5),
                v35_authentic_receipt_stabilization_vectors::artifact(4),
                next,
            )
        },
    );
    assert_eq!(
        overlong,
        Err(PhaseConsistentCoupledSolveErrorV1::ReceiptOscillation),
        "the detector refuses an exact four-member cycle",
    );
    assert_eq!(overlong_budget.used, 74);
}

#[test]
fn v54_endpoint_projection_uses_each_members_own_artifacts_and_receipt() {
    let mut receipts = v54_receipts(263.125);
    let mut second = v54_receipts(263.25)
        .remove(&7)
        .expect("distinct second physical receipt");
    second.lane_id = 8;
    second.receipt_sha256 = Digest32::zero();
    receipts.insert(8, second.seal().expect("resealed distinct second receipt"));
    let stage = BTreeMap::from([(7, (0.3, -10.0, 100.0)), (8, (0.4, -20.0, 120.0))]);
    let soil = vec![(500.0, 263.2)];
    let coordinates =
        covered_receipt_cycle_endpoint_coordinates_assemble_v1(&[7, 8], &stage, &receipts, &soil)
            .expect("complete member-owned endpoint projection");
    assert_eq!(coordinates.len(), 10);
    assert_eq!(coordinates[..3], [0.3, -10.0, 100.0]);
    assert_eq!(
        coordinates[3].to_bits(),
        receipts[&7].snow_candidate_heat_j_m2_ofe_ground.to_bits()
    );
    assert_eq!(coordinates[4..7], [0.4, -20.0, 120.0]);
    assert_eq!(
        coordinates[7].to_bits(),
        receipts[&8].snow_candidate_heat_j_m2_ofe_ground.to_bits()
    );
    assert_eq!(coordinates[8..], [500.0, 263.2]);
    assert_ne!(coordinates[3].to_bits(), coordinates[7].to_bits());

    let real_member = v54_v2_probe(v54_receipts(263.25), v54_receipts(263.125), 54);
    let real_member_before = real_member.clone();
    let real_coordinates = covered_receipt_cycle_endpoint_coordinates_v1(&real_member, &[7])
        .expect("real V2 exact-carry member projection");
    assert_eq!(real_coordinates.len(), 6);
    assert_eq!(
        real_coordinates[3].to_bits(),
        real_member.reconstructed_receipts[&7]
            .snow_candidate_heat_j_m2_ofe_ground
            .to_bits(),
    );
    let (_, _, exact_soil) =
        stable_monotone_v2_carry_coordinates_v1(&real_member.artifacts.soil_candidate)
            .expect("independent exact high-plus-carry reconstruction");
    assert_eq!(real_coordinates[4].to_bits(), exact_soil[0].to_bits());
    assert_eq!(
        covered_receipt_cycle_endpoint_coordinates_v1(&real_member, &[8]),
        Err(PhaseConsistentCoupledSolveErrorV1::Structure),
        "lane-order substitution is refused by the real projector",
    );
    let mut v1_carry_poison = real_member.clone();
    v1_carry_poison.artifacts.soil_candidate =
        v35_authentic_receipt_stabilization_vectors::artifact(55).soil_candidate;
    assert_eq!(
        covered_receipt_cycle_endpoint_coordinates_v1(&v1_carry_poison, &[7]),
        Err(PhaseConsistentCoupledSolveErrorV1::SideConstraint),
        "the exact-carry projector refuses a V1 soil substitution",
    );
    assert_eq!(real_member, real_member_before, "projection is clone-only");

    assert_eq!(
        covered_receipt_cycle_endpoint_coordinates_assemble_v1(&[8, 7], &stage, &receipts, &soil,),
        Err(PhaseConsistentCoupledSolveErrorV1::Structure),
    );
    assert_eq!(
        covered_receipt_cycle_endpoint_coordinates_assemble_v1(
            &[7, 8],
            &stage,
            &receipts,
            &[(f64::NAN, 263.2)],
        ),
        Err(PhaseConsistentCoupledSolveErrorV1::NonFinite),
    );
    let production = include_str!("phase_consistent_coupled_solve.rs");
    let start = production
        .find("fn covered_receipt_cycle_endpoint_coordinates_v1")
        .expect("production endpoint projection");
    let end = production[start..]
        .find("fn covered_authentic_receipt_cycle_endpoint_witness_v1")
        .map(|offset| start + offset)
        .expect("projection boundary");
    let projection = &production[start..end];
    assert!(projection.contains("stable_monotone_stage_coordinates_v1"));
    assert!(projection.contains("stable_monotone_v2_carry_coordinates_v1"));
    assert!(projection.contains("member.reconstructed_receipts"));
}

#[test]
fn v54_q_only_variation_cannot_change_authentic_physical_map() {
    let receipts = v54_receipts(263.204_229_777_162_2);
    let receipt = &receipts[&7];
    let low_coordinates: Vec<f64> = vec![0.3, -1_000.0, 100.0, 5_340.494_294_593_43, 500.0, 263.2];
    let mut high_coordinates = low_coordinates.clone();
    high_coordinates[3] = f64::from_bits(low_coordinates[3].to_bits() + 21);
    let low_trial = CoveredPhaseConsistentProjectedSoilConsumptionV1::SnowSoilCnOnly
        .cn_heat_coordinate_trial_operand(&low_coordinates, 0, receipt)
        .expect("low-Q coordinate trial");
    let high_trial = CoveredPhaseConsistentProjectedSoilConsumptionV1::SnowSoilCnOnly
        .cn_heat_coordinate_trial_operand(&high_coordinates, 0, receipt)
        .expect("high-Q coordinate trial");
    assert_ne!(
        low_trial.snow_candidate_heat_j_m2_ofe_ground.to_bits(),
        high_trial.snow_candidate_heat_j_m2_ofe_ground.to_bits(),
    );
    for kind in [
        CoveredPhaseConsistentPhysicalEvaluationKindV1::ReceiptStabilizationProbe,
        CoveredPhaseConsistentPhysicalEvaluationKindV1::ReceiptStabilizationReplay,
    ] {
        let low = covered_phase_consistent_cn_consumption_v1(kind, Some(&low_trial), Some(receipt))
            .expect("sealed receipt authentic low-Q consumption");
        let high =
            covered_phase_consistent_cn_consumption_v1(kind, Some(&high_trial), Some(receipt))
                .expect("sealed receipt authentic high-Q consumption");
        assert_eq!(low, high);
        assert_eq!(
            low.snow_candidate_heat_j_m2_ofe_ground.to_bits(),
            receipt.snow_candidate_heat_j_m2_ofe_ground.to_bits()
        );
    }
    let low_residual = v52_one_lane_q_residual(
        low_coordinates[3],
        receipt.snow_candidate_heat_j_m2_ofe_ground,
    )
    .expect("low-Q residual");
    let high_residual = v52_one_lane_q_residual(
        high_coordinates[3],
        receipt.snow_candidate_heat_j_m2_ofe_ground,
    )
    .expect("high-Q residual");
    assert_ne!(
        low_residual.r_q_cn_j_m2[0].to_bits(),
        high_residual.r_q_cn_j_m2[0].to_bits()
    );
}

fn v54_member_coordinates(member: &CoveredAuthenticReceiptStabilizationProbeV1) -> Vec<f64> {
    vec![
        0.327_290_935_6,
        -1_000.0,
        100.0,
        member.reconstructed_receipts[&7].snow_candidate_heat_j_m2_ofe_ground,
        500.0,
        263.2,
    ]
}

fn v54_two_member_budget_vectors() {
    let temperature = 263.204_229_777_162_2_f64;
    let r1 = v54_receipts(temperature);
    let r2 = v54_receipts(f64::from_bits(temperature.to_bits() + 1));
    let cycle = CoveredAuthenticReceiptExactCycleV1 {
        members: vec![
            v54_probe(r2.clone(), r1.clone(), 61),
            v54_probe(r1.clone(), r2.clone(), 62),
        ],
        discovery_probe_count: 3,
        publication_eligible: false,
    };
    let mut refused_budget = CoveredPhysicalEvaluationBudgetV1::new(94).expect("one short");
    let mut refused_calls = 0usize;
    assert_eq!(
        covered_authentic_receipt_cycle_endpoint_witness_v1(
            &cycle,
            &mut refused_budget,
            &v54_branch(),
            |member| Ok(v54_member_coordinates(member)),
            |_, _, _, _| {
                refused_calls += 1;
                unreachable!("atomic preflight must precede every physical map")
            },
        ),
        Err(PhaseConsistentCoupledSolveErrorV1::EvaluationBudget),
    );
    assert_eq!(refused_calls, 0);
    assert_eq!(refused_budget.used, 94);

    let mut budget = CoveredPhysicalEvaluationBudgetV1::new(93).expect("exact fit");
    let mut calls = 0usize;
    let stabilized = covered_authentic_receipt_cycle_endpoint_witness_v1(
        &cycle,
        &mut budget,
        &v54_branch(),
        |member| Ok(v54_member_coordinates(member)),
        |kind, coordinates, input, budget| {
            covered_physical_evaluation_budget_charge_v1(budget)?;
            calls += 1;
            let (marker, output) = match calls {
                1 => (70, r2.clone()),
                2 | 3 => (71, input.clone()),
                _ => unreachable!("two probes plus replay only"),
            };
            if calls == 3 {
                assert_eq!(
                    kind,
                    CoveredPhaseConsistentPhysicalEvaluationKindV1::ReceiptStabilizationReplay
                );
            }
            Ok((
                v54_evaluation(
                    coordinates,
                    budget.used,
                    marker,
                    output[&7].snow_candidate_heat_j_m2_ofe_ground,
                ),
                output,
            ))
        },
    )
    .expect("second own-artifact endpoint is an exact representable witness");
    assert_eq!(budget.used, 96);
    assert_eq!(calls, 3);
    assert_eq!(stabilized.independent_replay_count, 1);
    assert!(!stabilized.publication_eligible);
}

fn v54_three_member_budget_vector() {
    let temperature = 263.204_229_777_162_2_f64;
    let r1 = v54_receipts(temperature);
    let r2 = v54_receipts(f64::from_bits(temperature.to_bits() + 1));
    let r3 = v54_receipts(f64::from_bits(temperature.to_bits() + 2));
    let max_cycle = CoveredAuthenticReceiptExactCycleV1 {
        members: vec![
            v54_probe(r3.clone(), r1.clone(), 72),
            v54_probe(r1.clone(), r2.clone(), 73),
            v54_probe(r2.clone(), r3.clone(), 74),
        ],
        discovery_probe_count: 4,
        publication_eligible: false,
    };
    let mut max_budget = CoveredPhysicalEvaluationBudgetV1::new(92).expect("exact max fit");
    let mut max_calls = 0usize;
    let max_result = covered_authentic_receipt_cycle_endpoint_witness_v1(
        &max_cycle,
        &mut max_budget,
        &v54_branch(),
        |member| Ok(v54_member_coordinates(member)),
        |_, coordinates, input, budget| {
            covered_physical_evaluation_budget_charge_v1(budget)?;
            max_calls += 1;
            let output = match max_calls {
                1 => r2.clone(),
                2 => r3.clone(),
                3 | 4 => input.clone(),
                _ => unreachable!("three members plus replay only"),
            };
            Ok((
                v54_evaluation(
                    coordinates,
                    budget.used,
                    75,
                    output[&7].snow_candidate_heat_j_m2_ofe_ground,
                ),
                output,
            ))
        },
    )
    .expect("three-member maximum plus replay fits exactly");
    assert_eq!(max_budget.used, 96);
    assert_eq!(max_calls, 4);
    assert_eq!(max_result.independent_replay_count, 1);
    assert!(!max_result.publication_eligible);
}

#[test]
fn v54_cycle_witness_preflights_all_members_plus_replay() {
    v54_two_member_budget_vectors();
    v54_three_member_budget_vector();
}

fn v54_preflight_poison_vectors() {
    let r1 = v54_receipts(263.125);
    let r2 = v54_receipts(263.25);
    let member = v54_probe(r2.clone(), r1.clone(), 80);
    let artifact_before = member.artifacts.clone();
    let poisoned = CoveredAuthenticReceiptExactCycleV1 {
        members: vec![member],
        discovery_probe_count: 2,
        publication_eligible: true,
    };
    let mut budget = CoveredPhysicalEvaluationBudgetV1::new(90).expect("shared budget");
    assert_eq!(
        covered_authentic_receipt_cycle_endpoint_witness_v1(
            &poisoned,
            &mut budget,
            &v54_branch(),
            |_| unreachable!("publishable cycle is refused before projection"),
            |_, _, _, _| unreachable!("publishable cycle is refused before evaluation"),
        ),
        Err(PhaseConsistentCoupledSolveErrorV1::SideConstraint),
    );
    assert_eq!(budget.used, 90);
    assert_eq!(
        artifact_before,
        v35_authentic_receipt_stabilization_vectors::artifact(80)
    );

    let overlong = CoveredAuthenticReceiptExactCycleV1 {
        members: vec![
            v54_probe(r2.clone(), r1.clone(), 83),
            v54_probe(r1.clone(), r2.clone(), 84),
            v54_probe(r2.clone(), r1.clone(), 85),
            v54_probe(r1.clone(), r2.clone(), 86),
        ],
        discovery_probe_count: 5,
        publication_eligible: false,
    };
    let mut overlong_budget =
        CoveredPhysicalEvaluationBudgetV1::new(80).expect("ample shared budget");
    assert_eq!(
        covered_authentic_receipt_cycle_endpoint_witness_v1(
            &overlong,
            &mut overlong_budget,
            &v54_branch(),
            |_| unreachable!("overlong cycle is refused before projection"),
            |_, _, _, _| unreachable!("overlong cycle is refused before evaluation"),
        ),
        Err(PhaseConsistentCoupledSolveErrorV1::SideConstraint),
    );
    assert_eq!(overlong_budget.used, 80);
}

fn v54_no_witness_and_replay_vectors() {
    let temperature = 263.204_229_777_162_2_f64;
    let cycle_left = v54_receipts(temperature);
    let cycle_right = v54_receipts(f64::from_bits(temperature.to_bits() + 1));
    let no_witness = CoveredAuthenticReceiptExactCycleV1 {
        members: vec![
            v54_probe(cycle_right.clone(), cycle_left.clone(), 87),
            v54_probe(cycle_left.clone(), cycle_right.clone(), 88),
        ],
        discovery_probe_count: 3,
        publication_eligible: false,
    };
    let mut no_witness_budget = CoveredPhysicalEvaluationBudgetV1::new(90).expect("shared budget");
    let mut no_witness_calls = 0usize;
    let no_witness_result = covered_authentic_receipt_cycle_endpoint_witness_v1(
        &no_witness,
        &mut no_witness_budget,
        &v54_branch(),
        |member| Ok(v54_member_coordinates(member)),
        |_, coordinates, input, budget| {
            covered_physical_evaluation_budget_charge_v1(budget)?;
            no_witness_calls += 1;
            let output = if covered_snow_soil_receipt_sets_exact_v1(input, &cycle_left) {
                cycle_right.clone()
            } else {
                cycle_left.clone()
            };
            Ok((
                v54_evaluation(
                    coordinates,
                    budget.used,
                    89,
                    output[&7].snow_candidate_heat_j_m2_ofe_ground,
                ),
                output,
            ))
        },
    );
    assert_eq!(
        no_witness_result,
        Err(PhaseConsistentCoupledSolveErrorV1::ReceiptOscillation),
    );
    assert_eq!(no_witness_calls, 2);
    assert_eq!(no_witness_budget.used, 92);

    let replay_cycle = CoveredAuthenticReceiptExactCycleV1 {
        members: vec![v54_probe(cycle_right.clone(), cycle_left.clone(), 90)],
        discovery_probe_count: 2,
        publication_eligible: false,
    };
    let mut replay_budget = CoveredPhysicalEvaluationBudgetV1::new(90).expect("shared budget");
    let mut replay_calls = 0usize;
    let replay_mismatch = covered_authentic_receipt_cycle_endpoint_witness_v1(
        &replay_cycle,
        &mut replay_budget,
        &v54_branch(),
        |member| Ok(v54_member_coordinates(member)),
        |_, coordinates, input, budget| {
            covered_physical_evaluation_budget_charge_v1(budget)?;
            replay_calls += 1;
            Ok((
                v54_evaluation(
                    coordinates,
                    budget.used,
                    90 + replay_calls as u128,
                    input[&7].snow_candidate_heat_j_m2_ofe_ground,
                ),
                input.clone(),
            ))
        },
    );
    assert_eq!(
        replay_mismatch,
        Err(PhaseConsistentCoupledSolveErrorV1::ReplayMismatch),
        "a charged replay artifact substitution fails closed",
    );
    assert_eq!(replay_calls, 2);
    assert_eq!(replay_budget.used, 92);
}

fn v54_branch_poison_vector() {
    let r1 = v54_receipts(263.125);
    let r2 = v54_receipts(263.25);
    let cycle = CoveredAuthenticReceiptExactCycleV1 {
        members: vec![v54_probe(r2.clone(), r1.clone(), 81)],
        discovery_probe_count: 2,
        publication_eligible: false,
    };
    let coordinates = vec![
        0.327_290_935_6,
        -1_000.0,
        100.0,
        r1[&7].snow_candidate_heat_j_m2_ofe_ground,
        500.0,
        263.2,
    ];
    let mut budget = CoveredPhysicalEvaluationBudgetV1::new(90).expect("shared budget");
    let wrong_branch = CoveredPhaseConsistentPhysicalBranchIdentityV1 {
        phase_branch: vec![1],
        density_model_branch: vec![7],
    };
    let result = covered_authentic_receipt_cycle_endpoint_witness_v1(
        &cycle,
        &mut budget,
        &wrong_branch,
        |_| Ok(coordinates.clone()),
        |_, coordinates, _, budget| {
            covered_physical_evaluation_budget_charge_v1(budget)?;
            Ok((
                v54_evaluation(
                    coordinates,
                    budget.used,
                    82,
                    r1[&7].snow_candidate_heat_j_m2_ofe_ground,
                ),
                r1.clone(),
            ))
        },
    );
    assert_eq!(
        result,
        Err(PhaseConsistentCoupledSolveErrorV1::SideConstraint)
    );
    assert_eq!(budget.used, 91);
    assert!(!CoveredConvergenceAdmissionV1::CoupledAuthentic.admits(false, true, false));
}

#[test]
fn v54_cycle_witness_refuses_poison_and_rolls_back_without_publication() {
    v54_preflight_poison_vectors();
    v54_no_witness_and_replay_vectors();
    v54_branch_poison_vector();
}

include!("open_snow_convergence_v55_tests.rs");
