use openwepp_kernel_contract::{ResourceOwnerId, TileId, TransactionId};
use sha2::{Digest, Sha256};

use crate::{
    LitterPhaseTransactionIdentity, LitterPhaseTransactionInput, OfeId, Sha256Digest,
    V3_MODEL_DEFINITION_SHA256, V3PhaseFreeSurfaceEnergyLedger,
    canonical_litter_phase_receipt_sha256, execute_litter_phase_v3, litter_phase_receipt_from_json,
    litter_phase_receipt_json, validate_litter_phase_receipt,
};

fn configuration() -> LitterPhaseConfiguration {
    LitterPhaseConfiguration {
        litter_depth_m: 0.04,
        dry_heat_capacity_j_m2_k: 3_235.68,
        liquid_capacity_kg_m2_tile: 6.0,
        ice_capacity_kg_m2_tile: 34.0,
    }
}

fn beginning(liquid: f64, ice: f64, temperature: f64) -> BeginningLitterPhaseState {
    let capacity = configuration().dry_heat_capacity_j_m2_k
        + liquid * WATER_HEAT_CAPACITY_J_KG_K
        + ice * LITTER_ICE_HEAT_CAPACITY_J_KG_K;
    BeginningLitterPhaseState {
        liquid_kg_m2_tile: liquid,
        ice_kg_m2_tile: ice,
        sensible_energy_j_m2_tile: capacity * (temperature - REFERENCE_TEMPERATURE_K),
        temperature_k: temperature,
    }
}

fn environment(temperature: f64, humidity: f64) -> LitterVaporEnvironment {
    LitterVaporEnvironment {
        accepted_phase_free_temperature_k: temperature,
        air_density_kg_m3: 1.1,
        air_pressure_pa: 93_000.0,
        recipient_specific_humidity_kg_kg: humidity,
        litter_to_canopy_resistance_s_m: 80.0,
    }
}

fn digest(byte: u8) -> Sha256Digest {
    Sha256Digest::try_new(format!("{byte:02x}").repeat(32)).expect("test digest")
}

fn transaction_input(
    liquid: f64,
    ice: f64,
    state_temperature: f64,
    phase_temperature: f64,
    humidity: f64,
    interval_s: u64,
) -> LitterPhaseTransactionInput {
    let state = beginning(liquid, ice, state_temperature);
    let vapor_environment = environment(phase_temperature, humidity);
    let raw =
        evaluate_raw_litter_vapor(configuration(), state, vapor_environment).expect("raw vapor");
    let interval_seconds = std::time::Duration::from_secs(interval_s).as_secs_f64();
    let finalized = FinalizedLitterVapor {
        liquid_signed_rate_kg_m2_s: if raw.raw_liquid_signed_rate_kg_m2_s >= 0.0 {
            raw.raw_liquid_signed_rate_kg_m2_s
                .min(liquid / interval_seconds)
        } else {
            raw.raw_liquid_signed_rate_kg_m2_s
        },
        ice_signed_rate_kg_m2_s: if raw.raw_ice_signed_rate_kg_m2_s >= 0.0 {
            raw.raw_ice_signed_rate_kg_m2_s.min(ice / interval_seconds)
        } else {
            raw.raw_ice_signed_rate_kg_m2_s
        },
    };
    let vapor = finalize_litter_vapor(raw, finalized, state, phase_temperature, interval_seconds)
        .expect("final vapor");
    let post_vapor = install_finalized_vapor(configuration(), state, phase_temperature, vapor)
        .expect("post vapor");
    let storage =
        (post_vapor.sensible_energy_j_m2_tile - state.sensible_energy_j_m2_tile) / interval_seconds;
    let liquid_vapor_energy = vapor.liquid_signed_energy_j_m2 / interval_seconds;
    let ice_vapor_energy = vapor.ice_signed_energy_j_m2 / interval_seconds;
    let phase_free_surface_energy = V3PhaseFreeSurfaceEnergyLedger {
        beginning_sensible_energy_j_m2: state.sensible_energy_j_m2_tile,
        ending_sensible_energy_j_m2: post_vapor.sensible_energy_j_m2_tile,
        absorbed_shortwave_w_m2: storage + liquid_vapor_energy + ice_vapor_energy,
        net_longwave_w_m2: 0.0,
        sensible_to_canopy_air_w_m2: 0.0,
        liquid_vapor_energy_w_m2: liquid_vapor_energy,
        ice_vapor_energy_w_m2: ice_vapor_energy,
        ground_heat_w_m2: 0.0,
        storage_w_m2: storage,
        reconstructed_energy_residual_w_m2: 0.0,
    };
    LitterPhaseTransactionInput {
        identity: LitterPhaseTransactionIdentity {
            lse_configuration_sha256: digest(0x11),
            transaction_id: TransactionId(7),
            ofe_id: OfeId::try_new("ofe-1").expect("ofe"),
            tile_id: TileId::try_new("forest").expect("tile"),
            surface_owner_id: ResourceOwnerId::try_new("surface-liquid-v2").expect("owner"),
            beginning_surface_owner_sha256: digest(0x22),
            candidate_surface_owner_sha256: digest(0x33),
            support_start_ns: 1_000_000_000_000,
            support_end_ns: 1_000_000_000_000 + u128::from(interval_s) * 1_000_000_000,
        },
        configuration: configuration(),
        beginning: state,
        vapor_environment,
        finalized_vapor: finalized,
        phase_free_surface_energy,
    }
}

#[test]
fn model_definition_bytes_match_the_compiled_identity() {
    let bytes = include_bytes!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/artifacts/openwepp_snow_free_lse_v3_definition.json"
    ));
    assert_eq!(
        format!("{:x}", Sha256::digest(bytes)),
        V3_MODEL_DEFINITION_SHA256
    );
    let definition: serde_json::Value = serde_json::from_slice(bytes).expect("definition JSON");
    assert_eq!(definition["constants"]["ice_timescale_s"], 3_300.0);
    assert_eq!(
        definition["constants"]["physical_fallback_floor_ns"],
        60_000_000_000u64
    );
}

#[test]
fn empty_pool_is_exact_zero_without_regularization() {
    let raw = evaluate_raw_litter_vapor(
        configuration(),
        beginning(0.0, 0.0, REFERENCE_TEMPERATURE_K),
        environment(REFERENCE_TEMPERATURE_K, 0.001),
    )
    .expect("empty vapor");
    assert_eq!(raw.frozen_fraction.to_bits(), 0.0f64.to_bits());
    assert_eq!(
        raw.raw_liquid_signed_rate_kg_m2_s.to_bits(),
        0.0f64.to_bits()
    );
    assert_eq!(raw.raw_ice_signed_rate_kg_m2_s.to_bits(), 0.0f64.to_bits());
}

#[test]
fn exact_floor_and_larger_supports_apply_one_bounded_phase_operation() {
    for interval in [60, 3_300, 7_200] {
        let candidate =
            execute_litter_phase_v3(&transaction_input(4.0, 0.0, 268.15, 268.15, 0.05, interval))
                .expect("cold candidate");
        assert!(candidate.receipt.transfer.freeze_kg_m2 > 0.0);
        assert_eq!(candidate.receipt.transfer.melt_kg_m2, 0.0);
        assert_eq!(candidate.receipt.same_support_resolve_count, 0);
        assert!(candidate.ending.liquid_kg_m2_tile >= 0.0);
        assert!(candidate.ending.ice_kg_m2_tile <= 34.0);
        validate_litter_phase_receipt(&candidate.receipt).expect("sealed receipt");
    }
}

#[test]
fn nonempty_exact_reference_temperature_has_zero_phase_transfer() {
    let saturation =
        saturation_specific_humidity(REFERENCE_TEMPERATURE_K, 93_000.0).expect("saturation");
    let candidate = execute_litter_phase_v3(&transaction_input(
        2.0,
        1.0,
        REFERENCE_TEMPERATURE_K,
        REFERENCE_TEMPERATURE_K,
        saturation,
        3_300,
    ))
    .expect("reference candidate");
    assert_eq!(
        candidate.receipt.transfer.freeze_kg_m2.to_bits(),
        0.0f64.to_bits()
    );
    assert_eq!(
        candidate.receipt.transfer.melt_kg_m2.to_bits(),
        0.0f64.to_bits()
    );
    assert_eq!(
        candidate.ending.liquid_kg_m2_tile.to_bits(),
        2.0f64.to_bits()
    );
    assert_eq!(candidate.ending.ice_kg_m2_tile.to_bits(), 1.0f64.to_bits());
}

#[test]
fn warm_all_ice_melts_with_conservation_resolved_sign() {
    let saturation = saturation_specific_humidity(278.15, 93_000.0).expect("saturation");
    let candidate = execute_litter_phase_v3(&transaction_input(
        0.0, 2.0, 278.15, 278.15, saturation, 6_600,
    ))
    .expect("warm candidate");
    assert_eq!(candidate.receipt.transfer.freeze_kg_m2, 0.0);
    assert_eq!(candidate.receipt.transfer.melt_kg_m2, 2.0);
    assert_eq!(candidate.receipt.transfer.signed_phase_kg_m2, -2.0);
    assert_eq!(candidate.receipt.transfer.fusion_energy_j_m2, -667_400.0);
    assert_eq!(candidate.ending.ice_kg_m2_tile, 0.0);
    assert_eq!(candidate.ending.liquid_kg_m2_tile, 2.0);
}

#[test]
fn mixed_outbound_vapor_uses_separate_pools_and_ice_enthalpy() {
    let input = transaction_input(2.0, 1.0, 275.15, 275.15, 0.001, 600);
    let candidate = execute_litter_phase_v3(&input).expect("mixed candidate");
    let vapor = candidate.receipt.vapor;
    assert!(vapor.finalized.liquid_signed_rate_kg_m2_s > 0.0);
    assert!(vapor.finalized.ice_signed_rate_kg_m2_s > 0.0);
    assert_ne!(
        vapor.liquid_specific_enthalpy_j_kg.to_bits(),
        vapor.ice_specific_enthalpy_j_kg.to_bits()
    );
    assert!(vapor.liquid_signed_mass_kg_m2 <= 2.0);
    assert!(vapor.ice_signed_mass_kg_m2 <= 1.0);
    assert!(
        candidate
            .receipt
            .closure
            .liquid_vapor_energy_residual_j_m2
            .abs()
            <= 1.0e-7
    );
    assert!(
        candidate
            .receipt
            .closure
            .ice_vapor_energy_residual_j_m2
            .abs()
            <= 1.0e-7
    );
}

#[test]
fn condensation_and_deposition_credit_only_their_named_phases() {
    let input = transaction_input(2.0, 1.0, 270.15, 270.15, 0.05, 600);
    let candidate = execute_litter_phase_v3(&input).expect("inbound candidate");
    assert!(candidate.receipt.vapor.liquid_signed_mass_kg_m2 < 0.0);
    assert!(candidate.receipt.vapor.ice_signed_mass_kg_m2 < 0.0);
    assert!(candidate.receipt.post_vapor.liquid_kg_m2_tile > 2.0);
    assert!(candidate.receipt.post_vapor.ice_kg_m2_tile > 1.0);
}

#[test]
fn phase_closure_distinguishes_old_capacity_and_wrong_sign_mutants() {
    let candidate =
        execute_litter_phase_v3(&transaction_input(3.0, 1.0, 268.15, 268.15, 0.05, 3_300))
            .expect("freezing candidate");
    let receipt = &candidate.receipt;
    let old_capacity = receipt.configuration.dry_heat_capacity_j_m2_k
        + receipt.post_vapor.liquid_kg_m2_tile * WATER_HEAT_CAPACITY_J_KG_K
        + receipt.post_vapor.ice_kg_m2_tile * LITTER_ICE_HEAT_CAPACITY_J_KG_K;
    let old_capacity_temperature =
        REFERENCE_TEMPERATURE_K + receipt.ending.sensible_energy_j_m2_tile / old_capacity;
    assert_ne!(
        old_capacity_temperature.to_bits(),
        receipt.ending.temperature_k.to_bits()
    );
    let wrong_sign_energy =
        receipt.post_vapor.sensible_energy_j_m2_tile - receipt.transfer.fusion_energy_j_m2;
    assert_ne!(
        wrong_sign_energy.to_bits(),
        receipt.ending.sensible_energy_j_m2_tile.to_bits()
    );
    assert!(receipt.closure.total_phase_mass_residual_kg_m2.abs() <= 1.0e-7);
    assert!(receipt.closure.phase_enthalpy_residual_j_m2.abs() <= 1.0e-7);
}

#[test]
fn freezing_is_bounded_by_liquid_water_equivalent_ice_capacity() {
    let saturation = saturation_specific_humidity(250.0, 93_000.0).expect("saturation");
    let candidate = execute_litter_phase_v3(&transaction_input(
        3.0, 33.9, 250.0, 250.0, saturation, 86_400,
    ))
    .expect("capacity candidate");
    assert!(candidate.receipt.transfer.freeze_kg_m2 <= 0.100_000_000_000_01);
    assert!(candidate.ending.ice_kg_m2_tile <= configuration().ice_capacity_kg_m2_tile);
}

#[test]
fn total_pool_vapor_cap_and_failed_candidate_are_rejected_without_mutation() {
    let mut input = transaction_input(0.01, 5.0, 280.0, 280.0, 0.001, 600);
    input.finalized_vapor.liquid_signed_rate_kg_m2_s = 0.02 / 600.0;
    let before = input.clone();
    assert!(matches!(
        execute_litter_phase_v3(&input),
        Err(LandSurfaceEnergyError::FrozenLitterVapor(_))
    ));
    assert_eq!(input, before);
}

#[test]
fn receipt_round_trip_is_exact_and_tampering_fails_closed() {
    let candidate =
        execute_litter_phase_v3(&transaction_input(2.0, 1.0, 270.15, 270.15, 0.05, 600))
            .expect("candidate");
    let bytes = litter_phase_receipt_json(&candidate.receipt).expect("receipt bytes");
    let replay = litter_phase_receipt_from_json(&bytes).expect("receipt replay");
    assert_eq!(replay, candidate.receipt);
    assert_eq!(
        canonical_litter_phase_receipt_sha256(&replay).expect("digest"),
        replay.receipt_sha256
    );

    let mut poison = replay;
    poison.ending.ice_kg_m2_tile += 0.01;
    assert!(validate_litter_phase_receipt(&poison).is_err());

    let mut raw_operand_poison = candidate.receipt;
    raw_operand_poison
        .vapor
        .raw
        .environment
        .recipient_specific_humidity_kg_kg += 0.001;
    raw_operand_poison.receipt_sha256 =
        canonical_litter_phase_receipt_sha256(&raw_operand_poison).expect("resealed poison");
    assert!(matches!(
        validate_litter_phase_receipt(&raw_operand_poison),
        Err(LandSurfaceEnergyError::FrozenLitterVapor(_))
    ));
}

#[test]
fn phase_free_energy_closure_rejects_phase_alias_vapor_only_and_producer_mutants() {
    let candidate =
        execute_litter_phase_v3(&transaction_input(2.0, 1.0, 275.15, 275.15, 0.001, 600))
            .expect("candidate");
    assert!(
        candidate
            .receipt
            .closure
            .phase_free_storage_residual_w_m2
            .abs()
            <= 1.0e-7
    );
    assert!(
        candidate
            .receipt
            .closure
            .phase_free_surface_energy_residual_w_m2
            .abs()
            <= 1.0e-7
    );

    let mut ice_alias = candidate.receipt.clone();
    let old_ice = ice_alias.phase_free_surface_energy.ice_vapor_energy_w_m2;
    let liquid_alias = ice_alias.phase_free_surface_energy.liquid_vapor_energy_w_m2;
    ice_alias.phase_free_surface_energy.ice_vapor_energy_w_m2 = liquid_alias;
    ice_alias.phase_free_surface_energy.absorbed_shortwave_w_m2 += liquid_alias - old_ice;
    ice_alias.receipt_sha256 =
        canonical_litter_phase_receipt_sha256(&ice_alias).expect("resealed ice alias");
    assert!(matches!(
        validate_litter_phase_receipt(&ice_alias),
        Err(LandSurfaceEnergyError::FrozenLitterPhaseClosure(_))
    ));

    let mut vapor_only = candidate.receipt.clone();
    vapor_only.phase_free_surface_energy.absorbed_shortwave_w_m2 -=
        vapor_only.phase_free_surface_energy.storage_w_m2;
    vapor_only.receipt_sha256 =
        canonical_litter_phase_receipt_sha256(&vapor_only).expect("resealed vapor-only closure");
    assert!(matches!(
        validate_litter_phase_receipt(&vapor_only),
        Err(LandSurfaceEnergyError::FrozenLitterPhaseClosure(_))
    ));

    let mut producer = candidate.receipt;
    producer
        .phase_free_surface_energy
        .reconstructed_energy_residual_w_m2 += 1.0;
    producer.receipt_sha256 =
        canonical_litter_phase_receipt_sha256(&producer).expect("resealed producer residual");
    assert!(matches!(
        validate_litter_phase_receipt(&producer),
        Err(LandSurfaceEnergyError::FrozenLitterPhaseClosure(_))
    ));
}

#[test]
fn sub_floor_support_and_rho_i_capacity_fail_closed() {
    let mut input = transaction_input(1.0, 0.0, 270.15, 270.15, 0.05, 60);
    input.identity.support_end_ns -= 1;
    assert!(matches!(
        execute_litter_phase_v3(&input),
        Err(LandSurfaceEnergyError::SupportBelowMinimum { .. })
    ));

    let mut wrong_capacity = configuration();
    wrong_capacity.ice_capacity_kg_m2_tile =
        LITTER_ICE_VOLUMETRIC_CAPACITY * LITTER_ICE_DENSITY_KG_M3 * wrong_capacity.litter_depth_m;
    assert!(validate_litter_phase_configuration(wrong_capacity).is_err());
}

#[test]
fn vector_artifact_retains_independent_constants_and_mutants() {
    let vectors: serde_json::Value = serde_json::from_slice(include_bytes!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/artifacts/openwepp_snow_free_lse_v3_phase_vectors.json"
    )))
    .expect("phase vectors");
    assert_eq!(vectors["basis"]["ice_capacity_kg_m2_tile"], 34.0);
    assert_eq!(
        vectors["vectors"][1]["expected_freeze"],
        1.161_234_641_893_916_8
    );
    assert!(vectors["anti_mutants"].as_array().expect("mutants").len() >= 14);
}
