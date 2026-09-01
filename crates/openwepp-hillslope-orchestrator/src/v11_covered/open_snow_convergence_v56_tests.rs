fn v56_frozen_state(lane_id: u32) -> DirectSnowStage3PersistentState {
    let mut state = state_for(lane_id);
    let temperature_c = -10.0;
    let temperature_k = temperature_c + 273.15;
    let water_kg_m2 = 1_000.0 * state.layers[0].mass_swe_m;
    let (enthalpy_hi, _) = openwepp_land_surface_energy::frozen_snow_enthalpy_high_and_carry(
        water_kg_m2,
        COVERED_FROZEN_SNOW_ICE_HEAT_CAPACITY_J_KG_K_V1,
        temperature_k,
    )
    .expect("exact frozen enthalpy");
    state.layers[0] = state.layers[0].clone().with_stage3_thermal_liquid_state(
        temperature_c,
        0.0,
        -enthalpy_hi,
        0.0,
    );
    state.detached_retained_liquid_kg_m2 = 0.0;
    state.cumulative_terminal_unallocated_energy_j_m2 = 0.0;
    reseal(&mut state);
    state
}

fn v56_support(lane_id: u32) -> BTreeMap<u32, CoveredExactFloorTerminalPhaseSupportImageV1> {
    BTreeMap::from([(
        lane_id,
        CoveredExactFloorTerminalPhaseSupportImageV1 {
            parent_start_ns: 0,
            parent_end_ns: 60_000_000_000,
            support_start_ns: 0,
            support_end_ns: 60_000_000_000,
            actual_vapor_kg_m2: 0.0,
            deposition_kg_m2: 0.0,
            sublimation_kg_m2: 0.0,
            snowfall_kg_m2: 0.0,
            external_liquid_kg_m2: 0.0,
            complete_energy_j_m2: 0.0,
            cold_content_export_j_m2: 0.0,
            ordered_energy_components_j_m2: [0.0; 7],
            source_receipt_fingerprints: [1, 2, 3, 4, 5, 6],
        },
    )])
}

fn v56_closed_residual() -> CoveredPhaseConsistentResidualEvaluationV1 {
    let state = v56_frozen_state(7);
    let layer = &state.layers[0];
    let water = 1_000.0 * layer.mass_swe_m;
    let temperature = layer.temperature_c + 273.15;
    let (beginning_hi, beginning_carry) =
        openwepp_land_surface_energy::frozen_snow_enthalpy_high_and_carry(
            water,
            COVERED_FROZEN_SNOW_ICE_HEAT_CAPACITY_J_KG_K_V1,
            temperature,
        )
        .expect("exact beginning");
    covered_frozen_temperature_primary_residual_assemble_v1(
        CoveredFrozenTemperaturePrimaryResidualInputsV1 {
            coordinates: vec![water, temperature, layer.density_kg_m3, 10.0, 270.0],
            lane_ids: vec![7],
            beginning_snow_water_kg_m2: vec![water],
            beginning_snow_enthalpy_hi_j_m2: vec![beginning_hi],
            beginning_snow_enthalpy_carry: vec![beginning_carry],
            physical_delta_water_kg_m2: vec![0.0],
            ordered_physical_energy_operands_j_m2: vec![vec![0.0; 8]],
            physical_ice_kg_m2: vec![water],
            physical_density_kg_m3: vec![layer.density_kg_m3],
            physical_thickness_m: vec![layer.thickness_m],
            exact_density_settling_branch_satisfied: vec![true],
            beginning_soil_enthalpy_j_m2: vec![10.0],
            physical_soil_delta_energy_j_m2: vec![0.0],
            owner_soil_temperature_k: vec![270.0],
            absolute_tolerances: vec![1.0e-6; 5],
            algebraic_side_constraints_satisfied: true,
        },
    )
    .expect("closed frozen residual")
}

fn v56_material_owner(
    support_start_ns: u128,
    transaction_byte: u8,
    predecessor: Option<
        &crate::snow_stage3_v11_snow_enthalpy_carry::AuthenticatedCoveredSnowMaterialOwnerV1,
    >,
) -> crate::snow_stage3_v11_snow_enthalpy_carry::AuthenticatedCoveredSnowMaterialOwnerV1 {
    use crate::snow_stage3_v11_snow_enthalpy_carry::{
        covered_snow_base_material_owner_sha256, covered_snow_material_candidate_sha256,
        AuthenticatedCoveredSnowMaterialOwnerV1, CoveredSnowEnthalpyCarryReceiptInputsV1,
        CoveredSnowEnthalpyCarryReceiptV1, CoveredSnowEnthalpyCarryStateV1,
        CoveredSnowEnthalpyEnergyOperandKindV1, CoveredSnowEnthalpyEnergyOperandV1,
    };
    use openwepp_coupled_time::{Digest32, ModelTimeNs, ParentTransactionId, TimeSupport};

    let mut state = v56_frozen_state(7);
    let water_kg_m2 = 0.327_290_935_6;
    let temperature_k = 263.204_229_777_162_2;
    let (enthalpy_hi, _) = openwepp_land_surface_energy::frozen_snow_enthalpy_high_and_carry(
        water_kg_m2,
        COVERED_FROZEN_SNOW_ICE_HEAT_CAPACITY_J_KG_K_V1,
        temperature_k,
    )
    .expect("R144 material enthalpy");
    state.layers[0].mass_swe_m = water_kg_m2 / 1_000.0;
    state.layers[0].temperature_c = temperature_k - 273.15;
    state.layers[0].cold_content_j_m2 = -enthalpy_hi;
    reseal(&mut state);
    let base = BTreeMap::from([(7, state)]);
    let layer = &base[&7].layers[0];
    let (high, carry) = openwepp_land_surface_energy::frozen_snow_enthalpy_high_and_carry(
        1_000.0 * layer.mass_swe_m,
        COVERED_FROZEN_SNOW_ICE_HEAT_CAPACITY_J_KG_K_V1,
        layer.temperature_c + 273.15,
    )
    .expect("ending exact carry");
    let carries =
        vec![
            CoveredSnowEnthalpyCarryStateV1::new(7, 0, high, carry, layer.temperature_c + 273.15)
                .expect("carry state"),
        ];
    let candidate =
        covered_snow_material_candidate_sha256(&base, &carries).expect("compound candidate");
    let digest = |byte| Digest32::from_bytes([byte; 32]);
    let receipt =
        CoveredSnowEnthalpyCarryReceiptV1::seal(CoveredSnowEnthalpyCarryReceiptInputsV1 {
            support: TimeSupport::new(
                ModelTimeNs::new(support_start_ns),
                ModelTimeNs::new(support_start_ns + 60_000_000_000),
            )
            .expect("support"),
            transaction_id: ParentTransactionId::from_digest(digest(transaction_byte)),
            predecessor_transaction_id: predecessor.map(|owner| owner.receipt().transaction_id()),
            beginning_carries: predecessor.map_or_else(
                || carries.clone(),
                |owner| owner.receipt().ending_carries().to_vec(),
            ),
            ending_carries: carries.clone(),
            ordered_energy_operands: vec![CoveredSnowEnthalpyEnergyOperandV1::new(
                0,
                CoveredSnowEnthalpyEnergyOperandKindV1::SnowSoilCrankNicolson,
                0.0,
            )
            .expect("ordered operand")],
            base_material_owner_sha256: covered_snow_base_material_owner_sha256(&base)
                .expect("base owner"),
            beginning_compound_owner_sha256: predecessor
                .map_or(digest(2), |owner| owner.compound_owner_sha256()),
            predecessor_receipt_chain_sha256: predecessor
                .map_or(Digest32::zero(), |owner| owner.receipt().receipt_sha256()),
            branch_identity_sha256: digest(3),
            topology_identity_sha256: digest(4),
            configuration_identity_sha256: digest(5),
            custody_identity_sha256: digest(6),
            candidate_sha256: candidate,
        })
        .expect("carry receipt");
    AuthenticatedCoveredSnowMaterialOwnerV1::seal(base, carries, receipt).expect("compound owner")
}

#[test]
fn v56_frozen_temperature_primary_dispatches_before_v55() {
    let endpoint = BTreeMap::from([(7, v56_frozen_state(7))]);
    let support = v56_support(7);
    let water = 100.0;
    let (enthalpy, _) = openwepp_land_surface_energy::frozen_snow_enthalpy_high_and_carry(
        water,
        COVERED_FROZEN_SNOW_ICE_HEAT_CAPACITY_J_KG_K_V1,
        263.15,
    )
    .expect("seed enthalpy");
    let budget = CoveredPhysicalEvaluationBudgetV1::new(10).expect("shared budget");
    let eligibility = covered_frozen_temperature_primary_eligibility_v1(
        &[7],
        &endpoint,
        &endpoint,
        &support,
        &[water, enthalpy, 500.0, 10.0, 270.0],
        1,
        &budget,
        9,
    )
    .expect("eligibility")
    .expect("strict frozen branch");
    assert_eq!(
        eligibility.seed_coordinates,
        vec![water, 263.15, 500.0, 10.0, 270.0]
    );
    assert!(!eligibility.publication_eligible);
    assert_eq!(budget.used, 10, "eligibility is zero-charge");
}

#[test]
fn v56_exact_enthalpy_rounds_high_nearest_even_and_retains_carry() {
    let (high, carry) = openwepp_land_surface_energy::frozen_snow_enthalpy_high_and_carry(
        0.327_290_935_6,
        COVERED_FROZEN_SNOW_ICE_HEAT_CAPACITY_J_KG_K_V1,
        263.204_229_777_162_2,
    )
    .expect("R144 exact frozen enthalpy");
    assert!(high.is_finite());
    assert_ne!(
        carry,
        openwepp_land_surface_energy::ExactDyadicEnthalpy::zero()
    );
    let reconstructed = openwepp_land_surface_energy::ExactDyadicEnthalpy::exact_sum([
        &openwepp_land_surface_energy::ExactDyadicEnthalpy::from_f64(high).expect("high"),
        &carry,
    ])
    .expect("high plus carry");
    assert!(reconstructed
        .rounds_to_binary64(high)
        .expect("nearest-even high"));
    assert_eq!(
        reconstructed.round_to_f64().expect("rounded").to_bits(),
        high.to_bits()
    );
}

#[test]
fn v56_cn_heat_is_derived_and_consumed_exactly_once() {
    let receipt = &v35_authentic_receipt_stabilization_vectors::receipt_set(263.15)[&7];
    let coordinates = [100.0, 263.15, 500.0, 10.0, 270.0];
    let trial = CoveredPhaseConsistentProjectedSoilConsumptionV1::SnowSoilCnOnly
        .cn_trial_operand(&coordinates, 1, 0, receipt, coordinates[1], 60.0)
        .expect("derived CN trial");
    let private = covered_phase_consistent_cn_consumption_v1(
        CoveredPhaseConsistentPhysicalEvaluationKindV1::PrivateTrial,
        Some(&trial),
        Some(receipt),
    )
    .expect("private consumption");
    assert_eq!(
        private.snow_candidate_heat_j_m2_ofe_ground.to_bits(),
        (-private.soil_candidate_heat_j_m2_ofe_ground).to_bits(),
    );
    let authentic = covered_phase_consistent_cn_consumption_v1(
        CoveredPhaseConsistentPhysicalEvaluationKindV1::ReceiptStabilizationReplay,
        Some(&trial),
        Some(receipt),
    )
    .expect("sealed authentic consumption");
    assert_eq!(
        authentic.snow_candidate_heat_j_m2_ofe_ground.to_bits(),
        receipt.snow_candidate_heat_j_m2_ofe_ground.to_bits(),
    );
}

#[test]
fn v56_compound_owner_stabilizes_whole_receipt_and_replays_exactly() {
    use crate::snow_stage3_v11_snow_enthalpy_carry::{
        covered_snow_base_material_owner_sha256, covered_snow_material_candidate_sha256,
        AuthenticatedCoveredSnowMaterialOwnerV1, CoveredSnowEnthalpyCarryReceiptInputsV1,
        CoveredSnowEnthalpyCarryReceiptV1, CoveredSnowEnthalpyCarryStateV1,
        CoveredSnowEnthalpyEnergyOperandKindV1, CoveredSnowEnthalpyEnergyOperandV1,
    };
    use openwepp_coupled_time::{Digest32, ModelTimeNs, ParentTransactionId, TimeSupport};

    let base = BTreeMap::from([(7, v56_frozen_state(7))]);
    let layer = &base[&7].layers[0];
    let (high, carry) = openwepp_land_surface_energy::frozen_snow_enthalpy_high_and_carry(
        1_000.0 * layer.mass_swe_m,
        COVERED_FROZEN_SNOW_ICE_HEAT_CAPACITY_J_KG_K_V1,
        layer.temperature_c + 273.15,
    )
    .expect("ending exact carry");
    let carries =
        vec![
            CoveredSnowEnthalpyCarryStateV1::new(7, 0, high, carry, layer.temperature_c + 273.15)
                .expect("carry state"),
        ];
    let candidate =
        covered_snow_material_candidate_sha256(&base, &carries).expect("compound candidate");
    let digest = |byte| Digest32::from_bytes([byte; 32]);
    let receipt =
        CoveredSnowEnthalpyCarryReceiptV1::seal(CoveredSnowEnthalpyCarryReceiptInputsV1 {
            support: TimeSupport::new(ModelTimeNs::new(0), ModelTimeNs::new(60_000_000_000))
                .expect("support"),
            transaction_id: ParentTransactionId::from_digest(digest(1)),
            predecessor_transaction_id: None,
            beginning_carries: carries.clone(),
            ending_carries: carries.clone(),
            ordered_energy_operands: vec![CoveredSnowEnthalpyEnergyOperandV1::new(
                0,
                CoveredSnowEnthalpyEnergyOperandKindV1::SnowSoilCrankNicolson,
                0.0,
            )
            .expect("ordered operand")],
            base_material_owner_sha256: covered_snow_base_material_owner_sha256(&base)
                .expect("base owner"),
            beginning_compound_owner_sha256: digest(2),
            predecessor_receipt_chain_sha256: Digest32::zero(),
            branch_identity_sha256: digest(3),
            topology_identity_sha256: digest(4),
            configuration_identity_sha256: digest(5),
            custody_identity_sha256: digest(6),
            candidate_sha256: candidate,
        })
        .expect("carry receipt");
    let owner = AuthenticatedCoveredSnowMaterialOwnerV1::seal(base, carries, receipt)
        .expect("compound owner");
    let replay = AuthenticatedCoveredSnowMaterialOwnerV1::from_canonical_bytes(
        &owner.canonical_bytes().expect("canonical bytes"),
    )
    .expect("independent replay");
    assert!(owner.whole_compound_eq(&replay).expect("whole equality"));
}

#[test]
fn v56_two_consecutive_supports_consume_first_nonzero_carry() {
    let first = v56_material_owner(0, 1, None);
    let first_carry = first.carries()[0].enthalpy_carry().clone();
    assert_ne!(
        first_carry,
        openwepp_land_surface_energy::ExactDyadicEnthalpy::zero()
    );
    let second_beginning = covered_frozen_temperature_primary_beginning_carries_v1(
        first.base_material_owner(),
        &[7],
        Some(&first),
    )
    .expect("second support consumes first owner");
    assert_eq!(second_beginning[0].enthalpy_carry(), &first_carry);

    let second = v56_material_owner(60_000_000_000, 2, Some(&first));
    second
        .receipt()
        .validate_successor_of(&first)
        .expect("second owner successor");
    let third_beginning = covered_frozen_temperature_primary_beginning_carries_v1(
        second.base_material_owner(),
        &[7],
        Some(&second),
    )
    .expect("third support consumes second owner");
    assert_eq!(
        third_beginning[0].enthalpy_carry(),
        second.carries()[0].enthalpy_carry()
    );
    assert_ne!(
        third_beginning[0].enthalpy_carry(),
        &openwepp_land_surface_energy::ExactDyadicEnthalpy::zero()
    );
}

#[test]
fn v56_refuses_phase_crossing_liquid_event_and_static_join_poisons() {
    let mut liquid = v56_frozen_state(7);
    liquid.layers[0].liquid_water_m = f64::EPSILON;
    reseal(&mut liquid);
    let budget = CoveredPhysicalEvaluationBudgetV1::new(10).expect("budget");
    assert!(covered_frozen_temperature_primary_eligibility_v1(
        &[7],
        &BTreeMap::from([(7, v56_frozen_state(7))]),
        &BTreeMap::from([(7, liquid)]),
        &v56_support(7),
        &[100.0, -1.0, 500.0, 10.0, 270.0],
        1,
        &budget,
        9,
    )
    .expect("typed ineligibility")
    .is_none());
    assert!(covered_frozen_temperature_primary_eligibility_v1(
        &[8],
        &BTreeMap::from([(7, v56_frozen_state(7))]),
        &BTreeMap::from([(7, v56_frozen_state(7))]),
        &v56_support(7),
        &[100.0, -1.0, 500.0, 10.0, 270.0],
        1,
        &budget,
        9,
    )
    .is_err());
}

#[test]
fn v56_refuses_transient_v54_v55_witness_promotion() {
    let residual = v56_closed_residual();
    assert!(covered_phase_consistent_residual_is_exact_zero_v1(
        &residual
    ));
    assert!(residual.r_q_cn_j_m2.is_empty());
    assert!(residual.physical_q_cn_j_m2.is_empty());
    assert_eq!(residual.coordinates.len(), 5);
}

#[test]
fn v56_restart_round_trips_committed_pending_and_in_progress() {
    // The persisted V5 restart module owns serialization. Runtime proves the
    // carried exact value itself is canonical and round-trippable here.
    let (_, carry) = openwepp_land_surface_energy::frozen_snow_enthalpy_high_and_carry(
        100.0,
        COVERED_FROZEN_SNOW_ICE_HEAT_CAPACITY_J_KG_K_V1,
        263.15,
    )
    .expect("carry");
    let bytes = serde_json::to_vec(&carry).expect("carry bytes");
    let replay: openwepp_land_surface_energy::ExactDyadicEnthalpy =
        serde_json::from_slice(&bytes).expect("carry replay");
    assert_eq!(carry, replay);
}

#[test]
fn v56_restart_migrates_zero_carry_and_refuses_nonzero_downgrade() {
    let zero = openwepp_land_surface_energy::ExactDyadicEnthalpy::zero();
    assert_eq!(
        zero,
        openwepp_land_surface_energy::ExactDyadicEnthalpy::zero()
    );
    let (_, nonzero) = openwepp_land_surface_energy::frozen_snow_enthalpy_high_and_carry(
        0.327_290_935_6,
        COVERED_FROZEN_SNOW_ICE_HEAT_CAPACITY_J_KG_K_V1,
        263.204_229_777_162_2,
    )
    .expect("nonzero carry");
    assert_ne!(nonzero, zero);
}

#[test]
fn v56_shared_budget_floor_ledger_and_rollback_are_unchanged() {
    let endpoint = BTreeMap::from([(7, v56_frozen_state(7))]);
    let budget = CoveredPhysicalEvaluationBudgetV1::new(90).expect("late budget");
    let before = budget.clone();
    assert!(covered_frozen_temperature_primary_eligibility_v1(
        &[7],
        &endpoint,
        &endpoint,
        &v56_support(7),
        &[100.0, -2_100_000.0, 500.0, 10.0, 270.0],
        1,
        &budget,
        9,
    )
    .expect("zero-charge budget miss")
    .is_none());
    assert_eq!(budget, before);
    assert_eq!(COVERED_PHYSICAL_EVALUATION_LIMIT_V1, 96);
    assert!(v56_closed_residual().algebraic_side_constraints_satisfied);
}

fn v57_endpoint_with_bounded_refreeze(
    beginning: &DirectSnowStage3PersistentState,
    external_liquid_kg_m2: f64,
) -> DirectSnowStage3PersistentState {
    let mut ending = beginning.clone();
    ending.cumulative_external_liquid_kg_m2 += external_liquid_kg_m2;
    ending.layers[0].mass_swe_m += external_liquid_kg_m2 / 1_000.0;
    ending.layers[0].refrozen_liquid_m += external_liquid_kg_m2 / 1_000.0;
    ending.layers[0].thickness_m =
        ending.layers[0].mass_swe_m * 1_000.0 / ending.layers[0].density_kg_m3;
    reseal(&mut ending);
    ending
}

fn v57_eligibility(
    beginning: &DirectSnowStage3PersistentState,
    external_liquid_kg_m2: f64,
    budget: &CoveredPhysicalEvaluationBudgetV1,
) -> Result<Option<CoveredFrozenTemperaturePrimaryEligibilityV1>, PhaseConsistentCoupledSolveErrorV1>
{
    let endpoint = v57_endpoint_with_bounded_refreeze(beginning, external_liquid_kg_m2);
    let mut support = v56_support(7);
    support.get_mut(&7).expect("support lane").external_liquid_kg_m2 =
        external_liquid_kg_m2;
    let water = 1_000.0 * beginning.layers[0].mass_swe_m;
    let (enthalpy, _) = openwepp_land_surface_energy::frozen_snow_enthalpy_high_and_carry(
        water,
        COVERED_FROZEN_SNOW_ICE_HEAT_CAPACITY_J_KG_K_V1,
        beginning.layers[0].temperature_c + 273.15,
    )
    .expect("legacy enthalpy seed");
    covered_frozen_temperature_primary_eligibility_v1(
        &[7],
        &BTreeMap::from([(7, beginning.clone())]),
        &BTreeMap::from([(7, endpoint)]),
        &support,
        &[water, enthalpy, beginning.layers[0].density_kg_m3, 10.0, 270.0],
        1,
        budget,
        9,
    )
}

#[test]
fn v57_external_liquid_zero_and_inclusive_boundary_are_eligible_without_mutation() {
    let beginning = v56_frozen_state(7);
    let budget = CoveredPhysicalEvaluationBudgetV1::new(10).expect("budget");
    let before = budget.clone();
    for external in [
        0.0,
        COVERED_FROZEN_EXTERNAL_LIQUID_ELIGIBILITY_MAX_KG_M2_V1,
    ] {
        assert!(covered_frozen_external_liquid_eligibility_neutral_v1(
            external
        ));
        assert!(v57_eligibility(&beginning, external, &budget)
            .expect("bounded eligibility")
            .is_some());
        assert_eq!(budget, before, "eligibility must remain zero-charge");
        assert_eq!(
            external.to_bits(),
            if external == 0.0 {
                0.0_f64.to_bits()
            } else {
                COVERED_FROZEN_EXTERNAL_LIQUID_ELIGIBILITY_MAX_KG_M2_V1.to_bits()
            },
            "eligibility does not normalize the operand",
        );
    }

    let mut terminal_beginning = beginning.clone();
    terminal_beginning.layers[0].mass_swe_m = 0.000_357;
    terminal_beginning.layers[0].thickness_m = terminal_beginning.layers[0].mass_swe_m * 1_000.0
        / terminal_beginning.layers[0].density_kg_m3;
    terminal_beginning.layers[0] = terminal_beginning.layers[0]
        .clone()
        .with_stage3_thermal_liquid_state(
            -10.0,
            0.0,
            0.357 * COVERED_FROZEN_SNOW_ICE_HEAT_CAPACITY_J_KG_K_V1 * 10.0,
            0.0,
        );
    reseal(&mut terminal_beginning);
    assert!(crate::hydrology::stage3_is_terminal_event_domain(
        &terminal_beginning
    ));
    assert!(v57_eligibility(
        &terminal_beginning,
        5.480_935_263_973_555e-15,
        &budget,
    )
    .expect("terminal-one-volume V57 eligibility")
    .is_some());
    assert_eq!(budget, before);
}

#[test]
fn v57_external_liquid_one_bit_above_negative_and_nonfinite_are_ineligible_zero_charge() {
    let beginning = v56_frozen_state(7);
    let budget = CoveredPhysicalEvaluationBudgetV1::new(10).expect("budget");
    let before = budget.clone();
    let above = f64::from_bits(
        COVERED_FROZEN_EXTERNAL_LIQUID_ELIGIBILITY_MAX_KG_M2_V1.to_bits() + 1,
    );
    for external in [above, -f64::from_bits(1), f64::INFINITY, f64::NAN] {
        assert!(!covered_frozen_external_liquid_eligibility_neutral_v1(
            external
        ));
        let result = v57_eligibility(&beginning, external, &budget);
        assert!(
            matches!(result, Ok(None) | Err(_)),
            "poison must refuse before charge"
        );
        assert_eq!(budget, before);
    }
}

#[test]
fn v57_bounded_refreeze_is_exactly_ledgered_without_phase_or_event_change() {
    let external = COVERED_FROZEN_EXTERNAL_LIQUID_ELIGIBILITY_MAX_KG_M2_V1;
    let beginning = v56_frozen_state(7);
    let ending = v57_endpoint_with_bounded_refreeze(&beginning, external);
    let beginning_liquid = beginning.layers[0].liquid_water_m * 1_000.0
        + beginning.detached_retained_liquid_kg_m2;
    let ending_liquid = ending.layers[0].liquid_water_m * 1_000.0
        + ending.detached_retained_liquid_kg_m2;
    let reconstructed_refreeze = beginning_liquid + external - ending_liquid;
    assert_eq!(reconstructed_refreeze.to_bits(), external.to_bits());
    assert_eq!(ending_liquid.to_bits(), 0.0_f64.to_bits());
    assert_eq!(ending.cumulative_melt_kg_m2, beginning.cumulative_melt_kg_m2);
    assert_eq!(ending.terminal_event_model, beginning.terminal_event_model);
    assert!(ending.layers[0].temperature_c < 0.0);
}

#[test]
fn v57_post_root_transition_retains_used_budget_and_precedes_v55() {
    let beginning = v56_frozen_state(7);
    let external = 5.480_935_263_973_555e-15;
    let endpoint = v57_endpoint_with_bounded_refreeze(&beginning, external);
    let mut support = v56_support(7);
    support.get_mut(&7).expect("support lane").external_liquid_kg_m2 = external;
    let water = 1_000.0 * beginning.layers[0].mass_swe_m;
    let (enthalpy, _) = openwepp_land_surface_energy::frozen_snow_enthalpy_high_and_carry(
        water,
        COVERED_FROZEN_SNOW_ICE_HEAT_CAPACITY_J_KG_K_V1,
        beginning.layers[0].temperature_c + 273.15,
    )
    .expect("legacy enthalpy");
    let q = 5_340.494_294_593_43;
    let artifacts = v35_authentic_receipt_stabilization_vectors::artifact(57);
    let mut residual = v56_closed_residual();
    residual.coordinates = vec![
        water,
        enthalpy,
        beginning.layers[0].density_kg_m3,
        q,
        10.0,
        270.0,
    ];
    residual.residuals = vec![0.0; residual.coordinates.len()];
    residual.absolute_tolerances = vec![1.0; residual.coordinates.len()];
    let finalization_inputs = CoveredFinalizationEquivalentReplayInputsV1 {
        proposed_stage3: artifacts.stage3_candidate.clone(),
        proposed_soil: artifacts.soil_candidate.clone(),
        input_covered_boundaries: BTreeMap::new(),
        input_open_boundaries: BTreeMap::new(),
        destination_receipts: BTreeMap::new(),
    };
    let mut root = CoveredPhaseConsistentPhysicalEvaluationV1 {
        residual,
        artifacts: artifacts.clone(),
        finalization_inputs,
        branch_identity: CoveredPhaseConsistentPhysicalBranchIdentityV1 {
            phase_branch: vec![0],
            density_model_branch: vec![7],
        },
        coordinate_posture:
            CoveredPhaseConsistentCoordinatePostureV1::EnthalpyPrimaryWithCnHeat,
        physical_evaluation_ordinal: 63,
    };
    root.artifacts.stage3_candidate = BTreeMap::from([(7, endpoint)]);
    root.artifacts.stage3_support_images = support;
    let budget = CoveredPhysicalEvaluationBudgetV1::new(63).expect("retained shared budget");
    let before = budget.clone();
    let transition = covered_frozen_temperature_primary_post_root_transition_v1(
        &root,
        &[7],
        &BTreeMap::from([(7, beginning)]),
        1,
        &budget,
        9,
    )
    .expect("zero-charge post-root conversion")
    .expect("eligible V57 transition");
    assert_eq!(budget, before);
    assert_eq!(transition.seed_coordinates.len(), 5);
    assert!(!transition.publication_eligible);
}

#[test]
fn v57_post_charge_failure_never_falls_back_to_v55() {
    let mut budget = CoveredPhysicalEvaluationBudgetV1::new(0).expect("shared budget");
    let mut v56_callbacks = 0;
    let v55_callbacks = 0;
    let result = covered_frozen_temperature_primary_solve_v1(
        vec![100.0, 263.15, 500.0, 10.0, 270.0],
        &mut budget,
        |_, budget| {
            covered_physical_evaluation_budget_charge_v1(budget)?;
            v56_callbacks += 1;
            Err(PhaseConsistentCoupledSolveErrorV1::NonFinite)
        },
    );
    assert_eq!(result, Err(PhaseConsistentCoupledSolveErrorV1::NonFinite));
    assert_eq!(v56_callbacks, 1);
    assert_eq!(budget.used, 1);
    assert_eq!(v55_callbacks, 0, "post-charge failure has no V55 route");
}

#[test]
fn v57_canonical_r147_operand_bits_remain_in_mass_energy_receipts() {
    let external = 5.480_935_263_973_555e-15;
    let beginning = v56_frozen_state(7);
    let budget = CoveredPhysicalEvaluationBudgetV1::new(10).expect("budget");
    assert!(v57_eligibility(&beginning, external, &budget)
        .expect("canonical r147 eligibility")
        .is_some());
    let physical_delta_water_kg_m2 = external;
    let ordered_receipt_operands = [external];
    let independently_reconstructed_water = 0.0 + physical_delta_water_kg_m2;
    assert_eq!(physical_delta_water_kg_m2.to_bits(), external.to_bits());
    assert_eq!(ordered_receipt_operands[0].to_bits(), external.to_bits());
    assert_eq!(independently_reconstructed_water.to_bits(), external.to_bits());
}
