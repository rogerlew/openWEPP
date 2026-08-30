use super::*;
use crate::land_surface_energy_shadow::endpoint_fixture;
use openwepp_land_surface_energy::{
    EXACT_DYADIC_ENTHALPY_V1_DEFINITION_SHA256, SoilThermalAcceptedEnergyOperandV2,
    SoilThermalEnergyOperandKindV2, SoilThermalV2MigrationIdentity, migrate_soil_thermal_v1_to_v2,
};

fn digest(byte: char) -> Sha256Digest {
    Sha256Digest::try_new(byte.to_string().repeat(64)).expect("digest")
}

fn beginning_and_configuration() -> (
    openwepp_land_surface_energy::SoilThermalOwnerEnvelopeV2,
    LandSurfaceEnergyConfiguration,
) {
    let mut fixture = endpoint_fixture();
    fixture.thermal.ofes[0].ordered_layers[0].temperature_k = 273.15;
    fixture.thermal.ofes[0].ordered_layers[0].enthalpy_j_m2_ofe_ground = -34_315.421_541_136_02;
    let configuration = fixture.lse_configuration;
    let owner = migrate_soil_thermal_v1_to_v2(
        &fixture.thermal,
        SoilThermalV2MigrationIdentity {
            model_version: configuration
                .soil_thermal_configuration
                .model_version
                .clone(),
            model_definition_sha256: configuration
                .soil_thermal_configuration
                .model_definition_sha256
                .clone(),
            run_id: "wat5-v2-receiver".to_owned(),
            transaction_id: TransactionId(41),
            support_start_ns: 0,
            support_end_ns: 1_800_000_000_000,
            receipt_chain_sha256: digest('a'),
        },
    )
    .expect("V2 migration");
    (owner, configuration)
}

fn operand(
    beginning: &openwepp_land_surface_energy::SoilThermalOwnerEnvelopeV2,
    kind: SoilThermalEnergyOperandKindV2,
    ordinal: u32,
    energy: f64,
    digest_byte: char,
) -> SoilThermalAcceptedEnergyOperandV2 {
    SoilThermalAcceptedEnergyOperandV2 {
        ofe_id: beginning.state.ofes[0].ofe_id.clone(),
        layer_id: beginning.state.ofes[0].ordered_layers[0].layer_id.clone(),
        source_kind: kind,
        source_owner_id: ResourceOwnerId::try_new(format!("source-{digest_byte}"))
            .expect("source owner"),
        debit_credit_identity_sha256: digest(digest_byte),
        ordinal,
        units: "J m^-2 OFE-ground".to_owned(),
        basis: "ofe_ground".to_owned(),
        energy_j_m2_ofe_ground: energy,
    }
}

#[test]
fn wat5_credit_retains_exact_carry_and_seals_restart_checkpoint() {
    let (beginning, configuration) = beginning_and_configuration();
    let beginning_bytes = serde_json::to_vec(&beginning).expect("beginning bytes");
    let expected = SoilThermalExpectedAcceptedOperandSetV2::try_new(
        &beginning,
        &configuration,
        vec![operand(
            &beginning,
            SoilThermalEnergyOperandKindV2::Infiltration,
            0,
            -8.067_033_983_233_015e-19,
            'b',
        )],
    )
    .expect("external expected operands");
    let accepted = aggregate_soil_thermal_ending_v2(&beginning, &configuration, &expected)
        .expect("accepted exact credit");
    let ending = &accepted.ending_owner.state.ofes[0].ordered_layers[0];
    assert_eq!(
        ending.enthalpy_hi_j_m2_ofe_ground.to_bits(),
        (-34_315.421_541_136_02_f64).to_bits()
    );
    assert_eq!(ending.enthalpy_carry.sign, -1);
    assert_eq!(ending.enthalpy_carry.coefficient_hex, "1dc319224e55f");
    assert_eq!(ending.enthalpy_carry.exponent2, -109);
    assert_eq!(
        ending.temperature_k.to_bits(),
        beginning.state.ofes[0].ordered_layers[0]
            .temperature_k
            .to_bits()
    );
    let seals = seal_soil_thermal_accepted_candidate_v2(&beginning, &accepted)
        .expect("restart/checkpoint seals");
    validate_soil_thermal_orchestrator_seals_v2(&beginning, &accepted, &seals)
        .expect("seal replay");
    let bytes = canonical_soil_thermal_v2_bundle_bytes(&beginning, &accepted, &seals)
        .expect("canonical bundle");
    assert!(!bytes.is_empty());
    assert_eq!(
        serde_json::to_vec(&beginning).expect("unchanged beginning bytes"),
        beginning_bytes
    );
    assert_eq!(
        accepted.ending_owner.exact_carry_definition_sha256.as_str(),
        EXACT_DYADIC_ENTHALPY_V1_DEFINITION_SHA256
    );
}

#[test]
fn canonical_multi_operand_cancellation_is_order_bound_and_exact() {
    let (beginning, configuration) = beginning_and_configuration();
    let positive = operand(
        &beginning,
        SoilThermalEnergyOperandKindV2::SoilInternal,
        0,
        f64::from_bits(1),
        'c',
    );
    let negative = operand(
        &beginning,
        SoilThermalEnergyOperandKindV2::SoilInternal,
        1,
        -f64::from_bits(1),
        'd',
    );
    let expected = SoilThermalExpectedAcceptedOperandSetV2::try_new(
        &beginning,
        &configuration,
        vec![positive.clone(), negative.clone()],
    )
    .expect("canonical cancellation");
    let accepted = aggregate_soil_thermal_ending_v2(&beginning, &configuration, &expected)
        .expect("exact cancellation");
    let ending = &accepted.ending_owner.state.ofes[0].ordered_layers[0];
    let initial = &beginning.state.ofes[0].ordered_layers[0];
    assert_eq!(
        ending.enthalpy_hi_j_m2_ofe_ground.to_bits(),
        initial.enthalpy_hi_j_m2_ofe_ground.to_bits()
    );
    assert_eq!(ending.enthalpy_carry, initial.enthalpy_carry);

    let before = serde_json::to_vec(&beginning).expect("before bytes");
    assert!(
        SoilThermalExpectedAcceptedOperandSetV2::try_new(
            &beginning,
            &configuration,
            vec![negative, positive],
        )
        .is_err()
    );
    assert_eq!(
        serde_json::to_vec(&beginning).expect("rollback bytes"),
        before
    );
}

#[test]
fn external_expected_set_and_all_seals_reject_substitution_without_mutation() {
    let (beginning, configuration) = beginning_and_configuration();
    let expected = SoilThermalExpectedAcceptedOperandSetV2::try_new(
        &beginning,
        &configuration,
        vec![operand(
            &beginning,
            SoilThermalEnergyOperandKindV2::TopBoundary,
            0,
            0.25,
            'e',
        )],
    )
    .expect("expected set");
    let accepted =
        aggregate_soil_thermal_ending_v2(&beginning, &configuration, &expected).expect("accepted");
    let seals = seal_soil_thermal_accepted_candidate_v2(&beginning, &accepted).expect("seals");
    let before = serde_json::to_vec(&beginning).expect("before");

    let mut receipt_poison = accepted.clone();
    receipt_poison.credit_receipt.layer_credits[0]
        .accepted_operands
        .clear();
    receipt_poison
        .credit_receipt
        .reseal()
        .expect("reseal poison");
    receipt_poison.ending_owner.receipt_chain_sha256 =
        receipt_poison.credit_receipt.receipt_sha256.clone();
    assert!(
        validate_soil_thermal_orchestrator_seals_v2(&beginning, &receipt_poison, &seals,).is_err()
    );

    let mut seal_poison = seals.clone();
    seal_poison.expected_operand_set_sha256 = digest('f');
    assert!(
        validate_soil_thermal_orchestrator_seals_v2(&beginning, &accepted, &seal_poison).is_err()
    );
    assert!(canonical_soil_thermal_v2_bundle_bytes(&beginning, &accepted, &seal_poison).is_err());
    assert_eq!(serde_json::to_vec(&beginning).expect("after"), before);
}

#[test]
fn typed_top_boundary_credit_binds_support_owner_layer_and_receipt() {
    let (beginning, configuration) = beginning_and_configuration();
    let source_owner = ResourceOwnerId::try_new("snow-owner").expect("source owner");
    let credit = SoilThermalTopBoundaryCreditV1 {
        lane_id: 7,
        ofe_id: beginning.state.ofes[0].ofe_id.clone(),
        first_layer_id: beginning.state.ofes[0].ordered_layers[0].layer_id.clone(),
        beginning_owner_id: beginning.state.owner_id.clone(),
        beginning_configuration_sha256: beginning.state.configuration_sha256.clone(),
        beginning_state_sha256: beginning.state.state_sha256.clone(),
        support_start_ns: i64::try_from(beginning.support_start_ns).expect("start"),
        support_end_ns: i64::try_from(beginning.support_end_ns).expect("end"),
        accepted_positive_downward_j_m2_ofe_ground: 0.125,
        soil_thermal_credit_j_m2_ofe_ground: 0.125,
        snow_soil_heat_receipt_sha256: digest('7'),
    };
    let operands = soil_thermal_top_boundary_operands_v2(
        &beginning,
        std::slice::from_ref(&credit),
        &source_owner,
    )
    .expect("typed top-boundary operand");
    let expected =
        SoilThermalExpectedAcceptedOperandSetV2::try_new(&beginning, &configuration, operands)
            .expect("top-boundary expected set");
    aggregate_soil_thermal_ending_v2(&beginning, &configuration, &expected)
        .expect("top-boundary exact receiver");

    let mut wrong_support = credit;
    wrong_support.support_end_ns -= 1;
    assert!(
        soil_thermal_top_boundary_operands_v2(&beginning, &[wrong_support], &source_owner,)
            .is_err()
    );
}

#[test]
fn v1_bytes_are_frozen_across_checked_zero_carry_migration() {
    let fixture = endpoint_fixture();
    let v1_before = serde_json::to_vec(&fixture.thermal).expect("V1 bytes");
    let _ = migrate_soil_thermal_v1_to_v2(
        &fixture.thermal,
        SoilThermalV2MigrationIdentity {
            model_version: fixture
                .lse_configuration
                .soil_thermal_configuration
                .model_version
                .clone(),
            model_definition_sha256: fixture
                .lse_configuration
                .soil_thermal_configuration
                .model_definition_sha256
                .clone(),
            run_id: "migration-byte-lock".to_owned(),
            transaction_id: TransactionId(41),
            support_start_ns: 0,
            support_end_ns: 60_000_000_000,
            receipt_chain_sha256: digest('9'),
        },
    )
    .expect("checked migration");
    assert_eq!(
        serde_json::to_vec(&fixture.thermal).expect("unchanged V1 bytes"),
        v1_before
    );
}

#[test]
fn authoritative_lse_v2_configuration_is_admitted_without_weakening_v1() {
    let (beginning, mut configuration) = beginning_and_configuration();
    configuration.model_version = openwepp_land_surface_energy::V2_MODEL_VERSION.to_owned();
    configuration.model_definition_sha256 =
        Sha256Digest::try_new(openwepp_land_surface_energy::V2_MODEL_DEFINITION_SHA256)
            .expect("V2 model digest");
    configuration.vegetation_configuration.model_version =
        openwepp_land_surface_energy::V2_VEGETATION_MODEL_VERSION.to_owned();
    configuration
        .vegetation_configuration
        .model_definition_sha256 =
        Sha256Digest::try_new(openwepp_land_surface_energy::V2_VEGETATION_MODEL_DEFINITION_SHA256)
            .expect("V2 vegetation digest");
    configuration.configuration_sha256 = configuration.canonical_sha256().expect("V2 config seal");
    configuration
        .validate_v2()
        .expect("authoritative V2 config");

    let expected = SoilThermalExpectedAcceptedOperandSetV2::try_new(
        &beginning,
        &configuration,
        vec![operand(
            &beginning,
            SoilThermalEnergyOperandKindV2::SoilInternal,
            0,
            0.0,
            '8',
        )],
    )
    .expect("V2 configuration accepted");
    aggregate_soil_thermal_ending_v2(&beginning, &configuration, &expected)
        .expect("V2 configuration exact receiver");

    configuration.model_version = "OPENWEPP_SNOW_FREE_LSE_UNAUTHORIZED".to_owned();
    configuration.configuration_sha256 = configuration
        .canonical_sha256()
        .expect("unknown config reseal");
    assert!(
        SoilThermalExpectedAcceptedOperandSetV2::try_new(&beginning, &configuration, Vec::new(),)
            .is_err()
    );
}
