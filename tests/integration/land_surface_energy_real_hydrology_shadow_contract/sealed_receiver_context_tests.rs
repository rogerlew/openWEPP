//! Sealed receiving-owner hash and exact public-context poisons.

use super::*;

#[test]
fn sealed_receiver_hash_covers_all_thermal_fields_and_numeric_preflight_precedes_topology() {
    let ground_key = key("ground", "thermal-1");
    let protocol = WaterProtocol {
        transaction_id: TransactionId(41),
        hydrology_owner_id: ResourceOwnerId::try_new("production-hydrology").expect("owner"),
        beginning_snapshot_sha256: digest('3'),
        requests: vec![
            openwepp_hillslope_orchestrator::land_surface_energy_shadow::WaterAmount {
                key: ground_key.clone(),
                amount_kg_m2_stand_ground: 0.0,
            },
        ],
        authorizations: vec![WaterAuthorization {
            key: ground_key.clone(),
            amount_kg_m2_stand_ground: 0.0,
            reason: WaterAuthorizationReason::ZeroSupply,
        }],
        finalized_uses: vec![
            openwepp_hillslope_orchestrator::land_surface_energy_shadow::WaterAmount {
                key: ground_key,
                amount_kg_m2_stand_ground: 0.0,
            },
        ],
        condensation_credits: Vec::new(),
    };
    let baseline = unified_finalization(protocol);
    let baseline_hash = baseline.receiver_sets_sha256();
    for field in 0..3 {
        let mut thermal = baseline.soil_thermal_candidates().to_vec();
        let layer = &mut thermal[0].layers[0];
        match field {
            0 => layer.ground_heat_credit_j_m2_ofe_ground += 1.0,
            1 => layer.infiltration_enthalpy_credit_j_m2_ofe_ground += 1.0,
            2 => layer.ending_temperature_k += 1.0,
            _ => unreachable!("bounded field table"),
        }
        let mutated = UnifiedLseFinalization::try_new(
            baseline.water_protocol().clone(),
            baseline.ending_tile_states_pre_ingress().to_vec(),
            thermal,
            baseline.rollback_hashes().to_vec(),
        )
        .expect("finite receiver mutation remains structurally valid");
        assert_ne!(
            baseline_hash,
            mutated.receiver_sets_sha256(),
            "field {field}"
        );
    }

    assert_lse_numeric_context(&baseline);
    assert_thermal_numeric_context(&baseline);
    assert_topology_poison_context(&baseline);
}

#[test]
fn sealed_finalization_requires_one_exact_rollback_row_per_owner() {
    let ground_key = key("ground", "thermal-1");
    let protocol = WaterProtocol {
        transaction_id: TransactionId(41),
        hydrology_owner_id: ResourceOwnerId::try_new("production-hydrology").expect("owner"),
        beginning_snapshot_sha256: digest('3'),
        requests: vec![
            openwepp_hillslope_orchestrator::land_surface_energy_shadow::WaterAmount {
                key: ground_key.clone(),
                amount_kg_m2_stand_ground: 0.0,
            },
        ],
        authorizations: vec![WaterAuthorization {
            key: ground_key.clone(),
            amount_kg_m2_stand_ground: 0.0,
            reason: WaterAuthorizationReason::ZeroSupply,
        }],
        finalized_uses: vec![
            openwepp_hillslope_orchestrator::land_surface_energy_shadow::WaterAmount {
                key: ground_key,
                amount_kg_m2_stand_ground: 0.0,
            },
        ],
        condensation_credits: Vec::new(),
    };
    let baseline = unified_finalization(protocol);
    assert_eq!(baseline.rollback_hashes().len(), 3);

    assert_sealed_rollback_failure(&baseline, Vec::new(), Some("land-surface-energy-v1"), None);

    let mut missing_thermal = baseline.rollback_hashes().to_vec();
    missing_thermal.pop();
    assert_sealed_rollback_failure(&baseline, missing_thermal, Some("soil-thermal"), None);

    let mut substituted = baseline.rollback_hashes().to_vec();
    substituted[2].owner_id = "substituted-thermal".into();
    assert_sealed_rollback_failure(
        &baseline,
        substituted,
        Some("substituted-thermal"),
        Some(&digest('4')),
    );

    let mut duplicate = baseline.rollback_hashes().to_vec();
    duplicate.push(duplicate[2].clone());
    assert_sealed_rollback_failure(&baseline, duplicate, Some("soil-thermal"), None);

    let mut unexpected = baseline.rollback_hashes().to_vec();
    unexpected.push(OwnerRollbackHash {
        owner_kind: OwnerKind::Vegetation,
        owner_id: "unexpected-vegetation".into(),
        before_sha256: digest('7'),
        after_sha256: digest('7'),
    });
    assert_sealed_rollback_failure(
        &baseline,
        unexpected,
        Some("unexpected-vegetation"),
        Some(&digest('7')),
    );

    let mut hydrology_changed = baseline.rollback_hashes().to_vec();
    hydrology_changed[1].after_sha256 = digest('8');
    assert_sealed_rollback_failure(
        &baseline,
        hydrology_changed,
        Some("production-hydrology"),
        Some(&digest('3')),
    );
}

#[test]
fn missing_sealed_thermal_receiver_never_borrows_lse_ownership() {
    let ground_key = key("ground", "thermal-1");
    let protocol = WaterProtocol {
        transaction_id: TransactionId(41),
        hydrology_owner_id: ResourceOwnerId::try_new("production-hydrology").expect("owner"),
        beginning_snapshot_sha256: digest('3'),
        requests: vec![
            openwepp_hillslope_orchestrator::land_surface_energy_shadow::WaterAmount {
                key: ground_key.clone(),
                amount_kg_m2_stand_ground: 0.0,
            },
        ],
        authorizations: vec![WaterAuthorization {
            key: ground_key.clone(),
            amount_kg_m2_stand_ground: 0.0,
            reason: WaterAuthorizationReason::ZeroSupply,
        }],
        finalized_uses: vec![
            openwepp_hillslope_orchestrator::land_surface_energy_shadow::WaterAmount {
                key: ground_key,
                amount_kg_m2_stand_ground: 0.0,
            },
        ],
        condensation_credits: Vec::new(),
    };
    let baseline = unified_finalization(protocol);
    let attempted = UnifiedLseFinalization::candidate_receiver_sets_sha256(
        baseline.ending_tile_states_pre_ingress(),
        &[],
        baseline.rollback_hashes(),
    );
    let LandSurfaceEnergyShadowError::SurfaceLiquid(error) = UnifiedLseFinalization::try_new(
        baseline.water_protocol().clone(),
        baseline.ending_tile_states_pre_ingress().to_vec(),
        Vec::new(),
        baseline.rollback_hashes().to_vec(),
    )
    .expect_err("missing thermal receiver must fail closed") else {
        panic!("missing thermal receiver must retain canonical failure");
    };
    let failure = error.failure().expect("canonical receiver failure");
    assert_eq!(failure.code, DirectSurfaceLiquidErrorCode::E011);
    assert_eq!(failure.phase, DirectSurfaceLiquidPhase::AtomicEnvelope);
    assert_eq!(
        failure
            .context
            .owner_id
            .as_ref()
            .map(ResourceOwnerId::as_str),
        Some("soil-thermal")
    );
    assert_exact_hashes(failure, &digest('4'), &attempted);
}

fn assert_sealed_rollback_failure(
    baseline: &UnifiedLseFinalization,
    rollback_hashes: Vec<OwnerRollbackHash>,
    expected_owner_id: Option<&str>,
    expected_beginning: Option<&Sha256Digest>,
) {
    let attempted = UnifiedLseFinalization::candidate_receiver_sets_sha256(
        baseline.ending_tile_states_pre_ingress(),
        baseline.soil_thermal_candidates(),
        &rollback_hashes,
    );
    let LandSurfaceEnergyShadowError::SurfaceLiquid(error) = UnifiedLseFinalization::try_new(
        baseline.water_protocol().clone(),
        baseline.ending_tile_states_pre_ingress().to_vec(),
        baseline.soil_thermal_candidates().to_vec(),
        rollback_hashes,
    )
    .expect_err("malformed sealed rollback envelope must fail closed") else {
        panic!("rollback envelope must retain canonical failure");
    };
    let failure = error.failure().expect("canonical rollback failure");
    assert_eq!(failure.code, DirectSurfaceLiquidErrorCode::E011);
    assert_eq!(failure.phase, DirectSurfaceLiquidPhase::AtomicEnvelope);
    assert_eq!(
        failure
            .context
            .owner_id
            .as_ref()
            .map(ResourceOwnerId::as_str),
        expected_owner_id
    );
    assert_eq!(
        failure.rollback.beginning_owner_sha256.as_deref(),
        expected_beginning.map(Sha256Digest::as_str)
    );
    assert_eq!(
        failure.rollback.attempted_owner_sha256.as_deref(),
        Some(attempted.as_str())
    );
}

fn assert_lse_numeric_context(baseline: &UnifiedLseFinalization) {
    let mut tiles = baseline.ending_tile_states_pre_ingress().to_vec();
    tiles[0].surface_enthalpy_j_m2_tile_ground = f64::NAN;
    let attempted = UnifiedLseFinalization::candidate_receiver_sets_sha256(
        &tiles,
        baseline.soil_thermal_candidates(),
        baseline.rollback_hashes(),
    );
    let LandSurfaceEnergyShadowError::SurfaceLiquid(error) = UnifiedLseFinalization::try_new(
        baseline.water_protocol().clone(),
        tiles,
        baseline.soil_thermal_candidates().to_vec(),
        baseline.rollback_hashes().to_vec(),
    )
    .expect_err("nonfinite LSE receiver must retain exact configured context") else {
        panic!("sealed numeric preflight must retain canonical failure");
    };
    let failure = error.failure().expect("canonical LSE failure");
    assert_eq!(
        failure
            .context
            .owner_id
            .as_ref()
            .map(ResourceOwnerId::as_str),
        Some("land-surface-energy-v1")
    );
    assert_eq!(failure.context.transaction_id, Some(TransactionId(41)));
    assert_eq!(
        failure.context.ofe_id.as_ref().map(OfeId::as_str),
        Some("ofe-1")
    );
    assert_eq!(
        failure.context.tile_id.as_ref().map(TileId::as_str),
        Some("open")
    );
    assert_eq!(
        failure.context.surface_id.as_ref().map(SurfaceId::as_str),
        Some("surface:ofe-1:open")
    );
    assert_eq!(
        failure.context.source_id.as_ref().map(SourceId::as_str),
        Some("soil:ofe-1:thermal-1")
    );
    assert_exact_hashes(failure, &digest('2'), &attempted);
}

fn assert_thermal_numeric_context(baseline: &UnifiedLseFinalization) {
    let mut thermal = baseline.soil_thermal_candidates().to_vec();
    thermal[0].layers[0].ending_temperature_k = f64::NAN;
    let attempted = UnifiedLseFinalization::candidate_receiver_sets_sha256(
        baseline.ending_tile_states_pre_ingress(),
        &thermal,
        baseline.rollback_hashes(),
    );
    let LandSurfaceEnergyShadowError::SurfaceLiquid(error) = UnifiedLseFinalization::try_new(
        baseline.water_protocol().clone(),
        baseline.ending_tile_states_pre_ingress().to_vec(),
        thermal,
        baseline.rollback_hashes().to_vec(),
    )
    .expect_err("nonfinite thermal receiver must retain thermal ownership") else {
        panic!("sealed thermal preflight must retain canonical failure");
    };
    let failure = error.failure().expect("canonical thermal failure");
    assert_eq!(
        failure
            .context
            .owner_id
            .as_ref()
            .map(ResourceOwnerId::as_str),
        Some("soil-thermal")
    );
    assert_exact_hashes(failure, &digest('4'), &attempted);
}

fn assert_topology_poison_context(baseline: &UnifiedLseFinalization) {
    let mut tiles = baseline.ending_tile_states_pre_ingress().to_vec();
    tiles[0].ofe_id = OfeId::try_new("wrong-ofe").expect("wrong OFE");
    tiles[0].surface_enthalpy_j_m2_tile_ground = f64::NAN;
    let attempted = UnifiedLseFinalization::candidate_receiver_sets_sha256(
        &tiles,
        baseline.soil_thermal_candidates(),
        baseline.rollback_hashes(),
    );
    let LandSurfaceEnergyShadowError::SurfaceLiquid(error) = UnifiedLseFinalization::try_new(
        baseline.water_protocol().clone(),
        tiles,
        baseline.soil_thermal_candidates().to_vec(),
        baseline.rollback_hashes().to_vec(),
    )
    .expect_err("nonfinite receiver must precede topology failure") else {
        panic!("sealed numeric preflight must retain canonical failure");
    };
    let failure = error.failure().expect("canonical sealed receiver failure");
    assert_eq!(failure.code, DirectSurfaceLiquidErrorCode::E003);
    assert_eq!(failure.phase, DirectSurfaceLiquidPhase::IndependentClosure);
    assert_eq!(
        failure
            .context
            .owner_id
            .as_ref()
            .map(ResourceOwnerId::as_str),
        Some("land-surface-energy-v1")
    );
    assert_exact_hashes(failure, &digest('2'), &attempted);
}

fn assert_exact_hashes(
    failure: &openwepp_hillslope_orchestrator::DirectSurfaceLiquidFailure,
    beginning: &Sha256Digest,
    attempted: &str,
) {
    assert_eq!(
        failure.rollback.beginning_owner_sha256.as_deref(),
        Some(beginning.as_str())
    );
    assert_eq!(
        failure.rollback.attempted_owner_sha256.as_deref(),
        Some(attempted)
    );
}
