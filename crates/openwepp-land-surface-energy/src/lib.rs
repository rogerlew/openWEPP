//! Strict, default-off `OPENWEPP_SNOW_FREE_LSE_V1` runtime surfaces.
//!
//! This crate owns land-surface-energy identities and candidates. Hydrology
//! remains the exclusive owner of water mass and soil thermal remains the
//! exclusive owner of soil temperatures and enthalpies.

pub mod closure;
pub mod config;
mod covered_liquid;
mod covered_output;
pub mod diagnostics;
pub mod error;
pub mod exact_dyadic_enthalpy;
pub mod forcing;
pub mod identity;
pub mod litter_phase;
pub mod litter_phase_closure;
pub mod litter_phase_output;
mod numerics;
pub mod owner_envelope;
pub mod physics;
pub mod solver;
pub mod solver_litter_phase;
pub mod state;
pub mod support;
pub mod transaction;
pub mod transaction_v3;
pub mod v2_state;
pub mod v3_state;
pub mod water;

pub use closure::*;
pub use config::*;
pub use covered_liquid::{CoveredLiquidPass, CoveredOccupancyLiquidLedger};
pub use covered_output::*;
pub use diagnostics::*;
pub use error::*;
pub use exact_dyadic_enthalpy::*;
pub use forcing::*;
pub use identity::*;
pub use litter_phase::*;
pub use litter_phase_closure::*;
pub use litter_phase_output::*;
pub use owner_envelope::*;
pub use physics::*;
pub use solver::*;
pub use solver_litter_phase::*;
pub use state::*;
pub use support::*;
pub use transaction::*;
pub use transaction_v3::*;
pub use v2_state::*;
pub use v3_state::*;
pub use water::*;

/// Immutable model name admitted by `SC-LANDSURFACEENERGY-001@3`.
pub const MODEL_VERSION: &str = "OPENWEPP_SNOW_FREE_LSE_V1";
/// SHA-256 of the canonical compact V1 model-definition artifact.
pub const MODEL_DEFINITION_SHA256: &str =
    "e1736b8c77d13d6fb12fb97a6f747e54eea877edf237817b6c6e8954cff8332f";
/// Required coupled vegetation identity.
pub const VEGETATION_MODEL_VERSION: &str = "OPENWEPP_C3_WOODY_V8";
/// Required coupled vegetation model-definition digest.
pub const VEGETATION_MODEL_DEFINITION_SHA256: &str =
    "622bc900a08bd4c70e67c09e1fa113a9de24c48afce3b145a494bb76f6dcbe9b";
/// Prospective exact-zero-PAR coupled successor identity.
pub const V2_MODEL_VERSION: &str = "OPENWEPP_SNOW_FREE_LSE_V2";
pub const V2_MODEL_DEFINITION_SHA256: &str =
    "67d1681bf47c2b8b87d6195433209990b4021b7896bc50df973ac9246bfd6c19";
pub const V2_VEGETATION_MODEL_VERSION: &str = "OPENWEPP_C3_WOODY_V10";
pub const V2_VEGETATION_MODEL_DEFINITION_SHA256: &str =
    "0c42b025b6f9282d85afd5c8819ec9cc60d66a2b79ac6d5922bfdcc8026dd182";
/// Immutable snow-free forest-litter liquid/ice successor identity.
pub const V3_MODEL_VERSION: &str = "OPENWEPP_SNOW_FREE_LSE_V3";
/// SHA-256 of `artifacts/openwepp_snow_free_lse_v3_definition.json`.
pub const V3_MODEL_DEFINITION_SHA256: &str =
    "b8d8886d640f6993e7b6a9f22cc49a5a6d9871caf61a2f82a4041157231117fb";
/// Immutable sealed phase-receipt identity.
pub const V3_PHASE_RECEIPT_VERSION: &str = "OPENWEPP_FOREST_LITTER_PHASE_RECEIPT_V1";
pub const V3_IDENTITY_ERROR_CODE: &str = "LSEB-E-045";
pub const V3_VAPOR_ERROR_CODE: &str = "LSEB-E-046";
pub const V3_PHASE_CLOSURE_ERROR_CODE: &str = "LSEB-E-047";
pub const V3_TRANSACTION_ERROR_CODE: &str = "LSEB-E-048";

#[cfg(test)]
mod covered_oracle_conformance_tests;

#[cfg(test)]
mod authority_schema_tests {
    use openwepp_kernel_contract::ResourceOwnerId;
    use serde_json::Value;

    use super::*;

    fn frozen_vectors() -> Value {
        serde_json::from_str(include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/../../docs/work-packages/20260814-snow-free-land-surface-energy-authority-001/artifacts/openwepp_snow_free_lse_v1_vectors.json"
        )))
        .expect("frozen LSE authority vectors parse")
    }

    fn instance(name: &str) -> Value {
        frozen_vectors()["strict_schema_instances"][name].clone()
    }

    fn owner_envelope(water_protocol: WaterProtocol) -> OwnerEnvelopeIdentity {
        let transaction_id = water_protocol.transaction_id;
        let receipt_digest = water_protocol.beginning_snapshot_sha256.clone();
        let hydrology_owner = water_protocol.hydrology_owner_id.clone();
        let receipt = |kind, owner_id| CandidateOwnerReceipt {
            transaction_id,
            owner_kind: kind,
            owner_id,
            beginning_state_sha256: receipt_digest.clone(),
            candidate_state_sha256: receipt_digest.clone(),
        };
        OwnerEnvelopeIdentity {
            transaction_id,
            lse_configuration_sha256: water_protocol.beginning_snapshot_sha256.clone(),
            water_protocol,
            candidate_owner_receipts: CandidateReceiptSet {
                vegetation: receipt(
                    CandidateOwnerKind::Vegetation,
                    ResourceOwnerId::try_new("vegetation").expect("vegetation owner"),
                ),
                hydrology: receipt(CandidateOwnerKind::Hydrology, hydrology_owner),
                land_surface_energy: receipt(
                    CandidateOwnerKind::LandSurfaceEnergy,
                    ResourceOwnerId::try_new("land-surface-energy").expect("LSE owner"),
                ),
                soil_thermal: receipt(
                    CandidateOwnerKind::SoilThermal,
                    ResourceOwnerId::try_new("soil-thermal").expect("soil owner"),
                ),
                biogeochemistry: receipt(
                    CandidateOwnerKind::Biogeochemistry,
                    ResourceOwnerId::try_new("biogeochemistry").expect("BGC owner"),
                ),
            },
        }
    }

    #[test]
    fn frozen_configuration_and_state_validate_with_exact_canonical_digests() {
        let configuration: LandSurfaceEnergyConfiguration =
            serde_json::from_value(instance("configuration")).expect("configuration schema");
        configuration.validate().expect("configuration authority");
        assert_eq!(
            configuration
                .canonical_sha256()
                .expect("configuration digest"),
            configuration.configuration_sha256
        );

        let state: LandSurfaceEnergyState =
            serde_json::from_value(instance("state")).expect("state schema");
        state.validate_schema().expect("state authority");
        assert_eq!(
            state.canonical_sha256().expect("state digest").as_str(),
            "6ff22f0d72b6c4fdad3c0d8a0b2947571191e48213635609af8f3b951c07abf1"
        );
    }

    #[test]
    fn frozen_forcing_water_and_diagnostics_validate() {
        let forcing: LandSurfaceForcing =
            serde_json::from_value(instance("forcing")).expect("forcing schema");
        forcing
            .validate(forcing.transaction_id)
            .expect("forcing authority");

        let water: WaterProtocol =
            serde_json::from_value(instance("water_protocol")).expect("water schema");
        water.validate().expect("water protocol authority");

        let diagnostics: NumericalDiagnostics =
            serde_json::from_value(instance("diagnostics")).expect("diagnostics schema");
        diagnostics.validate().expect("diagnostics authority");
    }

    #[test]
    fn protocol_identity_stage_precedes_earlier_row_domain_in_direct_and_owner_envelope_paths() {
        let mut water: WaterProtocol =
            serde_json::from_value(instance("water_protocol")).expect("water schema");
        water.requests[0].amount_kg_m2_stand_ground = f64::NAN;
        water.authorizations[1].key.transaction_id =
            openwepp_kernel_contract::TransactionId(water.transaction_id.0 + 1);

        let violation = water
            .validate_identity_stage()
            .expect_err("later-row identity poison");
        assert_eq!(violation.row, WaterProtocolRow::Authorization(1));
        assert_eq!(
            violation.error.class(),
            LandSurfaceEnergyErrorClass::Identity
        );
        assert_eq!(
            water
                .validate()
                .expect_err("canonical protocol poison")
                .class(),
            LandSurfaceEnergyErrorClass::Identity,
        );

        let envelope = owner_envelope(water);
        assert_eq!(
            envelope
                .validate()
                .expect_err("owner envelope protocol poison")
                .class(),
            LandSurfaceEnergyErrorClass::Identity,
        );
    }

    #[test]
    fn owner_envelope_identity_set_precedes_every_protocol_numeric_stage() {
        for identity_poison in 0..9 {
            for protocol_poison in 0..5 {
                let water: WaterProtocol =
                    serde_json::from_value(instance("water_protocol")).expect("water schema");
                let mut envelope = owner_envelope(water);
                let expected_configuration = envelope.lse_configuration_sha256.clone();
                match identity_poison {
                    0 => envelope.transaction_id = openwepp_kernel_contract::TransactionId(0),
                    1 => {
                        envelope.transaction_id =
                            openwepp_kernel_contract::TransactionId(envelope.transaction_id.0 + 1);
                    }
                    2 => {
                        envelope.lse_configuration_sha256 =
                            Sha256Digest::try_new("e".repeat(64)).expect("wrong config digest");
                    }
                    3 => {
                        envelope
                            .candidate_owner_receipts
                            .vegetation
                            .transaction_id
                            .0 += 1;
                    }
                    4 => envelope.candidate_owner_receipts.hydrology.transaction_id.0 += 1,
                    5 => {
                        envelope
                            .candidate_owner_receipts
                            .land_surface_energy
                            .transaction_id
                            .0 += 1;
                    }
                    6 => {
                        envelope
                            .candidate_owner_receipts
                            .soil_thermal
                            .transaction_id
                            .0 += 1;
                    }
                    7 => {
                        envelope
                            .candidate_owner_receipts
                            .biogeochemistry
                            .transaction_id
                            .0 += 1;
                    }
                    8 => {
                        envelope.candidate_owner_receipts.hydrology.owner_id =
                            ResourceOwnerId::try_new("wrong-hydrology-owner")
                                .expect("wrong hydrology owner");
                    }
                    _ => unreachable!("bounded identity poison table"),
                }
                match protocol_poison {
                    0 => envelope.water_protocol.requests[0].amount_kg_m2_stand_ground = f64::NAN,
                    1 => envelope
                        .water_protocol
                        .requests
                        .push(envelope.water_protocol.requests[0].clone()),
                    2 => envelope.water_protocol.requests[0].amount_kg_m2_stand_ground = -1.0,
                    3 => envelope.water_protocol.authorizations[0].amount_kg_m2_stand_ground = -1.0,
                    4 => envelope.water_protocol.finalized_uses[0].amount_kg_m2_stand_ground = -1.0,
                    _ => unreachable!("bounded protocol poison table"),
                }
                assert_eq!(
                    validate_five_owner_envelope(&envelope, &expected_configuration)
                        .expect_err("owner identity must precede protocol numeric poison")
                        .class(),
                    LandSurfaceEnergyErrorClass::Identity,
                    "identity poison {identity_poison}, protocol poison {protocol_poison}",
                );
            }
        }
    }

    #[test]
    fn genuine_receipt_owner_set_failures_remain_owner_envelope_errors() {
        let water: WaterProtocol =
            serde_json::from_value(instance("water_protocol")).expect("water schema");
        for poison in 0..2 {
            let mut envelope = owner_envelope(water.clone());
            match poison {
                0 => {
                    envelope.candidate_owner_receipts.vegetation.owner_kind =
                        CandidateOwnerKind::Hydrology;
                }
                1 => {
                    envelope.candidate_owner_receipts.vegetation.owner_id = envelope
                        .candidate_owner_receipts
                        .land_surface_energy
                        .owner_id
                        .clone();
                }
                _ => unreachable!("bounded owner-set poison table"),
            }
            assert_eq!(
                envelope
                    .validate()
                    .expect_err("genuine owner-set poison")
                    .class(),
                LandSurfaceEnergyErrorClass::OwnerEnvelope,
            );
        }
    }

    #[test]
    fn unknown_fields_and_digest_mutations_fail_closed() {
        let mut configuration = instance("configuration");
        configuration["unknown_scientific_default"] = Value::from(1);
        assert!(serde_json::from_value::<LandSurfaceEnergyConfiguration>(configuration).is_err());

        let mut configuration: LandSurfaceEnergyConfiguration =
            serde_json::from_value(instance("configuration")).expect("configuration schema");
        configuration.ofes[0].tiles[0].surface_vis_albedo =
            f64::from_bits(configuration.ofes[0].tiles[0].surface_vis_albedo.to_bits() + 1);
        assert!(matches!(
            configuration.validate(),
            Err(LandSurfaceEnergyError::Identity {
                field: "configuration_sha256",
                ..
            })
        ));
    }
}
