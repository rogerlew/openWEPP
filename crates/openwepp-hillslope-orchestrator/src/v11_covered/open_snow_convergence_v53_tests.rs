fn v53_receipts(ending_temperature_k: f64) -> BTreeMap<u32, SnowSoilHeatReceiptV1> {
    v35_authentic_receipt_stabilization_vectors::receipt_set(ending_temperature_k)
}

fn v53_legacy_seed() -> Vec<f64> {
    vec![0.316_811_296_4, 2_782.434_5, 100.0, -139_348.515_7, 271.988_76]
}

fn v53_two_lane_receipts() -> BTreeMap<u32, SnowSoilHeatReceiptV1> {
    let mut single = v53_receipts(273.125);
    let first = single.remove(&7).expect("lane 7 receipt");
    let mut second = first.clone();
    second.lane_id = 8;
    second.receipt_sha256 = Digest32::zero();
    let second = second.seal().expect("resealed lane 8 receipt");
    BTreeMap::from([(7, first), (8, second)])
}

#[test]
fn v53_same_map_cn_heat_seed_uses_endpoint_receipts() {
    let captured_input_q = f64::from_bits(0x40c8_60a4_672a_c030);
    let captured_latest_coordinate_q = f64::from_bits(0x40bf_c5d3_06d4_1e50);
    let captured_latest_r_q = f64::from_bits(0xbeaf_908c_0000_0000);
    let captured_latest_output_q = f64::from_bits(0x40bf_c5d3_06e3_e696);
    assert_eq!(captured_input_q, 12_481.284_398_406_831);
    assert_eq!(captured_latest_coordinate_q, 8_133.824_322_945_977);
    assert_eq!(captured_latest_r_q, -9.406_994_649_907_574e-7);
    assert_eq!(captured_latest_output_q, 8_133.824_323_886_676);
    assert_eq!(
        (captured_latest_coordinate_q - captured_latest_r_q).to_bits(),
        captured_latest_output_q.to_bits()
    );
    assert_ne!(captured_input_q.to_bits(), captured_latest_output_q.to_bits());

    let retained = v53_receipts(273.0);
    let endpoint = v53_receipts(273.125);
    let retained_q = retained[&7].snow_candidate_heat_j_m2_ofe_ground;
    let endpoint_q = endpoint[&7].snow_candidate_heat_j_m2_ofe_ground;
    assert_ne!(retained_q.to_bits(), endpoint_q.to_bits());

    let assembled = covered_phase_consistent_same_map_cn_heat_seed_v1(
        &v53_legacy_seed(),
        &[7],
        1,
        &endpoint,
    )
    .expect("same-map endpoint Q seed");
    assert_eq!(assembled.len(), 6);
    assert_eq!(assembled[3].to_bits(), endpoint_q.to_bits());
    assert_ne!(assembled[3].to_bits(), retained_q.to_bits());
}

#[test]
fn v53_same_map_cn_heat_seed_rebinds_legacy_seed() {
    let endpoint = v53_receipts(273.125);
    let legacy = v53_legacy_seed();
    let assembled = covered_phase_consistent_same_map_cn_heat_seed_v1(
        &legacy,
        &[7],
        1,
        &endpoint,
    )
    .expect("legacy seed augmented by same-map Q");
    assert_eq!(&assembled[..3], &legacy[..3]);
    assert_eq!(
        assembled[3].to_bits(),
        endpoint[&7]
            .snow_candidate_heat_j_m2_ofe_ground
            .to_bits()
    );
    assert_eq!(&assembled[4..], &legacy[3..]);
}

#[test]
fn v53_same_map_cn_heat_seed_refuses_cross_map_receipt_substitution() {
    let source = include_str!("open_snow.rs");
    let start = source
        .find("let endpoint_seed_snow_soil_receipts")
        .expect("endpoint receipt reconstruction seam");
    let end = source[start..]
        .find("let authentic_replay_input_exchange")
        .map(|offset| start + offset)
        .expect("seed assembly boundary");
    let seed_source = &source[start..end];
    assert!(seed_source.contains("&endpoint_seed_stage3"));
    assert!(seed_source.contains("&endpoint_seed_soil"));
    assert!(seed_source.contains("&endpoint_seed_snow_soil_receipts"));
    assert!(seed_source.contains("if legacy_seed_coordinates.is_empty()"));
    assert!(seed_source.contains("legacy_initial_coordinates = legacy_seed_coordinates"));
    assert!(!seed_source.contains("&accepted_snow_soil_receipts"));

    let retained = v53_receipts(273.0);
    let endpoint = v53_receipts(273.125);
    let assembled = covered_phase_consistent_same_map_cn_heat_seed_v1(
        &v53_legacy_seed(),
        &[7],
        1,
        &endpoint,
    )
    .expect("only locally reconstructed endpoint receipt is supplied");
    assert_ne!(
        assembled[3].to_bits(),
        retained[&7]
            .snow_candidate_heat_j_m2_ofe_ground
            .to_bits()
    );
}

#[test]
fn v53_same_map_cn_heat_seed_refuses_lane_and_finite_poison() {
    let endpoint = v53_receipts(273.125);
    assert_eq!(
        covered_phase_consistent_same_map_cn_heat_seed_v1(
            &v53_legacy_seed(),
            &[8],
            1,
            &endpoint,
        ),
        Err(PhaseConsistentCoupledSolveErrorV1::Structure)
    );
    let two_lane = v53_two_lane_receipts();
    let two_lane_legacy = vec![0.3, -10.0, 100.0, 0.4, -20.0, 120.0, 500.0, 263.2];
    assert_eq!(
        covered_phase_consistent_same_map_cn_heat_seed_v1(
            &two_lane_legacy,
            &[8, 7],
            1,
            &two_lane,
        ),
        Err(PhaseConsistentCoupledSolveErrorV1::Structure),
        "a complete two-lane map in non-owner order is refused"
    );
    let ordered = covered_phase_consistent_same_map_cn_heat_seed_v1(
        &two_lane_legacy,
        &[7, 8],
        1,
        &two_lane,
    )
    .expect("two-lane owner order");
    assert_eq!(ordered.len(), 10);
    assert_eq!(
        ordered[3].to_bits(),
        two_lane[&7]
            .snow_candidate_heat_j_m2_ofe_ground
            .to_bits()
    );
    assert_eq!(
        ordered[7].to_bits(),
        two_lane[&8]
            .snow_candidate_heat_j_m2_ofe_ground
            .to_bits()
    );
    assert_eq!(
        covered_phase_consistent_same_map_cn_heat_seed_v1(
            &v53_legacy_seed(),
            &[7, 8],
            1,
            &endpoint,
        ),
        Err(PhaseConsistentCoupledSolveErrorV1::Structure)
    );
    let mut nonfinite = v53_legacy_seed();
    nonfinite[1] = f64::NAN;
    assert_eq!(
        covered_phase_consistent_same_map_cn_heat_seed_v1(&nonfinite, &[7], 1, &endpoint),
        Err(PhaseConsistentCoupledSolveErrorV1::Structure)
    );
    let mut unsealed = endpoint.clone();
    unsealed
        .get_mut(&7)
        .expect("receipt")
        .snow_candidate_heat_j_m2_ofe_ground = f64::INFINITY;
    assert_eq!(
        covered_phase_consistent_same_map_cn_heat_seed_v1(
            &v53_legacy_seed(),
            &[7],
            1,
            &unsealed,
        ),
        Err(PhaseConsistentCoupledSolveErrorV1::SideConstraint)
    );
}

#[test]
fn v53_same_map_cn_heat_seed_is_uncharged_and_retains_admission_guards() {
    let endpoint = v53_receipts(273.125);
    let assembled = covered_phase_consistent_same_map_cn_heat_seed_v1(
        &v53_legacy_seed(),
        &[7],
        1,
        &endpoint,
    )
    .expect("uncharged representational assembly");
    assert_eq!(assembled.len(), 4 * 1 + 2 * 1);
    let production = include_str!("phase_consistent_coupled_solve.rs");
    let start = production
        .find("fn covered_phase_consistent_same_map_cn_heat_seed_v1")
        .expect("same-map seed helper");
    let end = production[start..]
        .find("fn covered_phase_consistent_carrier_closure_posture_v1")
        .map(|offset| start + offset)
        .expect("next production helper");
    let helper = &production[start..end];
    assert!(!helper.contains("CoveredPhysicalEvaluationBudgetV1"));
    assert!(!helper.contains("covered_physical_evaluation_budget_charge_v1"));
    assert!(!helper.contains("finalization_equivalent_map"));
    assert!(!CoveredConvergenceAdmissionV1::Picard.admits(false, true, true));
    assert!(CoveredConvergenceAdmissionV1::CoupledAuthentic.admits(false, true, true));
    assert!(!CoveredConvergenceAdmissionV1::CoupledAuthentic.admits(false, false, true));
}

include!("open_snow_convergence_v54_tests.rs");
