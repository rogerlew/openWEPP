//! Strict, default-off `OPENWEPP_SNOW_FREE_LSE_V1` runtime surfaces.
//!
//! This crate owns land-surface-energy identities and candidates. Hydrology
//! remains the exclusive owner of water mass and soil thermal remains the
//! exclusive owner of soil temperatures and enthalpies.

pub mod closure;
pub mod config;
pub mod diagnostics;
pub mod error;
pub mod forcing;
pub mod identity;
pub mod owner_envelope;
pub mod physics;
pub mod solver;
pub mod state;
pub mod transaction;
pub mod water;

pub use closure::*;
pub use config::*;
pub use diagnostics::*;
pub use error::*;
pub use forcing::*;
pub use identity::*;
pub use owner_envelope::*;
pub use physics::*;
pub use solver::*;
pub use state::*;
pub use transaction::*;
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

        let transaction_id = water.transaction_id;
        let receipt_digest = water.beginning_snapshot_sha256.clone();
        let receipt = |kind, owner: &str| CandidateOwnerReceipt {
            transaction_id,
            owner_kind: kind,
            owner_id: ResourceOwnerId::try_new(owner).expect("receipt owner"),
            beginning_state_sha256: receipt_digest.clone(),
            candidate_state_sha256: receipt_digest.clone(),
        };
        let envelope = OwnerEnvelopeIdentity {
            transaction_id,
            lse_configuration_sha256: water.beginning_snapshot_sha256.clone(),
            water_protocol: water,
            candidate_owner_receipts: CandidateReceiptSet {
                vegetation: receipt(CandidateOwnerKind::Vegetation, "vegetation"),
                hydrology: receipt(CandidateOwnerKind::Hydrology, "hydrology"),
                land_surface_energy: receipt(
                    CandidateOwnerKind::LandSurfaceEnergy,
                    "land-surface-energy",
                ),
                soil_thermal: receipt(CandidateOwnerKind::SoilThermal, "soil-thermal"),
                biogeochemistry: receipt(CandidateOwnerKind::Biogeochemistry, "biogeochemistry"),
            },
        };
        assert_eq!(
            envelope
                .validate()
                .expect_err("owner envelope protocol poison")
                .class(),
            LandSurfaceEnergyErrorClass::Identity,
        );
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
