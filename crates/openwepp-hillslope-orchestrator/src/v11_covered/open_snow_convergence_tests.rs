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
    changed.layers[0].density_kg_m3 =
        f64::from_bits(changed.layers[0].density_kg_m3.to_bits() + 1);
    reseal(&mut changed);
    assert!(!equal(original, changed));
}

#[test]
fn underrelaxation_retains_exact_candidate_density_while_damping_continuous_state() {
    let original = state();
    let mut candidate = original.clone();
    candidate.layers[0].density_kg_m3 =
        f64::from_bits(candidate.layers[0].density_kg_m3.to_bits() + 1);
    candidate.layers[0].thickness_m = candidate.layers[0].mass_swe_m * 1_000.0
        / candidate.layers[0].density_kg_m3;
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
        (relaxed[&7].layers[0].mass_swe_m * 1_000.0
            / relaxed[&7].layers[0].density_kg_m3)
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
    assert!(!covered_fixed_point_picard_accepts_convergence_v1(
        true, true, true,
    ));
    assert!(covered_fixed_point_picard_accepts_convergence_v1(
        true, false, true,
    ));
    assert!(covered_fixed_point_picard_accepts_convergence_v1(
        true, true, false,
    ));
    assert!(!covered_fixed_point_picard_accepts_convergence_v1(
        false, false, true,
    ));
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
    state_b.layers[0].density_kg_m3 =
        f64::from_bits(state_b.layers[0].density_kg_m3.to_bits() + 1);
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
        covered_fixed_point_relaxation_weight_v1(60_000_000_000, true)
            .map(f64::to_bits),
        Some(0.5_f64.to_bits()),
    );
}

#[test]
fn period_two_detector_rejects_event_and_topology_poisons() {
    let state_a = state();
    let mut state_b = state_a.clone();
    state_b.layers[0].density_kg_m3 =
        f64::from_bits(state_b.layers[0].density_kg_m3.to_bits() + 1);
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
    candidate.layers[0].thickness_m = candidate.layers[0].mass_swe_m * 1_000.0
        / candidate.layers[0].density_kg_m3;
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
    assert!(validate_destination_reconstruction_against_lane_aggregate(reconstructed, reconstructed).is_ok());
    for index in 0..reconstructed.len() {
        let mut substituted = reconstructed;
        substituted[index] += 2.0e-6;
        assert!(validate_destination_reconstruction_against_lane_aggregate(reconstructed, substituted).is_err());
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
    assert_eq!(difference.2, original[&8].layers[0].cold_content_j_m2.to_bits());
    assert_eq!(difference.3, changed_lane_8.layers[0].cold_content_j_m2.to_bits());
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
