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
mod m1_liquid_routing;
#[cfg(all(test, feature = "m1-trust-region-controller-stage"))]
mod m1_trust_region_controller_interface;
mod numerics;
#[doc(hidden)]
#[must_use]
pub fn m1_fixed_sequence_controller_policy() -> (u32, u32) {
    numerics::m1_fixed_sequence_controller_policy()
}
pub mod owner_envelope;
pub mod physics;
pub mod solver;
pub mod solver_litter_open;
pub mod solver_litter_phase;
pub mod solver_mechanism_audit;
#[cfg(any(test, feature = "test-support"))]
pub mod solver_residual_corpus_capture;
#[cfg(test)]
mod solver_stem_jacobian_tests;
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
pub use m1_liquid_routing::*;
pub use owner_envelope::*;
pub use physics::*;
pub use solver::*;
pub use solver_litter_open::*;
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
mod m1_coupled_tests;
#[cfg(test)]
mod m1_trust_region_stage1;
// Stage 2 controls are deliberately a separate, feature-gated expected-red
// surface.  They name the controller observations that a later body must
// populate without making the experiment reachable from a normal test or
// production build.
#[cfg(all(test, feature = "m1-trust-region-controller-stage"))]
mod m1_trust_region_stage2_controls;
#[cfg(test)]
pub(crate) use m1_trust_region_stage1::{
    m1_trust_region_frozen_merit_for_test, m1_trust_region_jacobi_observation_for_test,
    m1_trust_region_solve_subproblem_for_test, m1_trust_region_validate_ball_endpoint_for_test,
};

#[cfg(all(test, feature = "m1-trust-region-controller-stage"))]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) enum M1TrustRegionAssemblyKind {
    NaturalPredictor,
    SelectedReplacement,
}

// These records are observations, never input-selected verdicts.  The body
// must append an entry before each owned operation and preserve the immutable
// base identity on every refusal.
#[cfg(all(test, feature = "m1-trust-region-controller-stage"))]
#[derive(Clone, Debug)]
pub(crate) struct M1TrustRegionControllerOperationEntry {
    pub(crate) operation: M1TrustRegionControllerOperation,
    pub(crate) core_entries: u16,
    pub(crate) hydraulic_blocks: u16,
    pub(crate) svd_face_factorizations: u16,
    pub(crate) subproblem_entries: u8,
    pub(crate) materialization_entries: u8,
    pub(crate) attempted_core_entries: u16,
    pub(crate) attempted_hydraulic_blocks: u16,
    pub(crate) attempted_svd_face_factorizations: u16,
    pub(crate) attempted_materialization_entries: u8,
    pub(crate) attempted_operation_is_recorded: bool,
}

#[cfg(all(test, feature = "m1-trust-region-controller-stage"))]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) enum M1TrustRegionControllerOperation {
    NaturalAssembly,
    SelectedReplacementAssembly,
    ProposalCore,
    ProposalHydraulicBlock,
    SubproblemFace,
    PostUpdateCurrentAdmission,
    TerminalCurrentAdmission,
    Materialization,
}

// Private, test-only contract boundary for COLD-CANOPY-M1-TR-SVD-BVLS-01
// Stage 1.  These data-only declarations deliberately contain no solver or
// fallback behavior.  The detached numerical body may be introduced only
// after independent review releases these controls.
#[cfg(test)]
#[derive(Clone, Debug)]
pub(crate) struct M1TrustRegionSubproblemInput {
    pub(crate) raw_residual: [f64; 21],
    pub(crate) raw_jacobian: [[f64; 21]; 21],
    pub(crate) normalizers: [f64; 21],
    pub(crate) scales: [f64; 21],
    pub(crate) lower: [f64; 21],
    pub(crate) upper: [f64; 21],
    pub(crate) initial_radius: f64,
    pub(crate) base: [f64; 21],
    pub(crate) test_work: M1TrustRegionTestWorkState,
}

#[cfg(test)]
#[derive(Clone, Copy, Debug, Default)]
pub(crate) struct M1TrustRegionTestWorkState {
    // A test-only, observed starting chronology.  The solver still computes
    // the next Jacobi/face event from the supplied matrix and box; it may not
    // substitute a requested outcome or alter any frozen cap.
    pub(crate) completed_jacobi_sweeps: u8,
    pub(crate) completed_face_pivots: u8,
}

#[cfg(test)]
#[derive(Clone, Copy, Debug)]
pub(crate) struct M1TrustRegionMeritInput {
    pub(crate) raw_residual: [f64; 2],
    pub(crate) raw_jacobian: [[f64; 2]; 2],
    pub(crate) frozen_normalizers: [f64; 2],
    pub(crate) coordinate_scales: [f64; 2],
    pub(crate) scaled_step: [f64; 2],
    pub(crate) candidate_raw_residual: [f64; 2],
    pub(crate) moving_candidate_normalizers: [f64; 2],
}

#[cfg(test)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) enum M1TrustRegionRefusalKind {
    SvdNonFinite,
    SvdNoConvergence,
    RankDeficient,
    LambdaBracket,
    BallAccuracy,
    FacePivotLimit,
    #[cfg(feature = "m1-trust-region-controller-stage")]
    RadiusExhausted,
    #[cfg(feature = "m1-trust-region-controller-stage")]
    WorkCap,
    DegenerateFace,
}

#[cfg(test)]
#[derive(Clone, Copy, Debug)]
pub(crate) struct M1TrustRegionBallEndpoint {
    pub(crate) a: f64,
    pub(crate) f: f64,
    pub(crate) radius: f64,
    pub(crate) endpoint_override: Option<[f64; 1]>,
}

// These observations are deliberately data-only.  Stage 1 controls need the
// implementation to expose what it computed, rather than an injected verdict
// or a requested work count.  The later private body owns their population.
#[cfg(test)]
#[derive(Clone, Debug)]
pub(crate) struct M1TrustRegionBallObservation {
    pub(crate) factorization_entered: bool,
    pub(crate) bracket_endpoint_evaluations: Vec<M1TrustRegionLambdaEvaluation>,
    pub(crate) bisection_evaluations: Vec<M1TrustRegionLambdaEvaluation>,
    pub(crate) bracket_initial: [f64; 2],
    pub(crate) bracket_final: [f64; 2],
    pub(crate) final_upper_lambda: f64,
    pub(crate) natural_upper_endpoint: f64,
    pub(crate) pre_hook_endpoint: f64,
    pub(crate) endpoint_hook_observed: Option<[f64; 1]>,
    pub(crate) post_hook_endpoint: [f64; 1],
    pub(crate) computed_norm: f64,
    pub(crate) computed_gap: f64,
    pub(crate) computed_tolerance: f64,
    pub(crate) accepted: bool,
    pub(crate) kind: Option<M1TrustRegionRefusalKind>,
}

#[cfg(test)]
#[derive(Clone, Copy, Debug)]
pub(crate) struct M1TrustRegionLambdaEvaluation {
    pub(crate) lambda: f64,
    pub(crate) norm: f64,
    pub(crate) feasible: bool,
}

#[cfg(test)]
#[derive(Clone, Copy, Debug)]
pub(crate) struct M1TrustRegionJacobiInput {
    pub(crate) matrix: [[f64; 3]; 3],
    pub(crate) free_ids: [usize; 3],
}

#[cfg(test)]
#[derive(Clone, Debug)]
pub(crate) struct M1TrustRegionJacobiObservation {
    pub(crate) completed_sweeps: u8,
    pub(crate) pair_visits: Vec<M1TrustRegionJacobiPair>,
    pub(crate) rotations: usize,
    pub(crate) b_columns: [[f64; 3]; 3],
    pub(crate) v_columns: [[f64; 3]; 3],
    pub(crate) unsorted_singular_values: [f64; 3],
    pub(crate) sorted_singular_values: [f64; 3],
    pub(crate) sorted_free_ids: [usize; 3],
    pub(crate) sorted_from_unsorted: [usize; 3],
    pub(crate) sorted_b_columns: [[f64; 3]; 3],
    pub(crate) sorted_v_columns: [[f64; 3]; 3],
}

#[cfg(test)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) struct M1TrustRegionJacobiPair {
    pub(crate) left: usize,
    pub(crate) right: usize,
    pub(crate) rotated: bool,
    pub(crate) sweep: u8,
}

#[cfg(test)]
#[derive(Clone, Debug)]
pub(crate) struct M1TrustRegionKktEvent {
    pub(crate) action: M1TrustRegionKktAction,
    pub(crate) selected_coordinate: usize,
    pub(crate) lower_mask: [bool; 21],
    pub(crate) upper_mask: [bool; 21],
    pub(crate) free_mask: [bool; 21],
    // Step 4 computes these only after a box-feasible face.  Crossing/active
    // events must record their masks but cannot fabricate KKT arithmetic.
    pub(crate) g: Option<[f64; 21]>,
    pub(crate) lambda_times_p: Option<[f64; 21]>,
    pub(crate) h: Option<[f64; 21]>,
}

#[cfg(test)]
#[derive(Clone, Copy, Debug)]
pub(crate) struct M1TrustRegionMeritObservation {
    pub(crate) predicted_reduction: f64,
    pub(crate) actual_reduction: f64,
    pub(crate) rho: f64,
    pub(crate) moving_weight_trial_merit: f64,
}

#[cfg(test)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) enum M1TrustRegionKktAction {
    ActivateLower,
    ActivateUpper,
    ReleaseLower,
    ReleaseUpper,
    Return,
}

#[cfg(test)]
#[derive(Clone, Debug)]
pub(crate) struct M1TrustRegionSubproblemObservation {
    pub(crate) initial_face_step: [f64; 21],
    pub(crate) base_coordinates: [f64; 21],
    pub(crate) candidate_coordinates: [f64; 21],
    pub(crate) initial_lower_mask: [bool; 21],
    pub(crate) initial_upper_mask: [bool; 21],
    pub(crate) initial_free_mask: [bool; 21],
    pub(crate) scaled_step: [f64; 21],
    pub(crate) final_residual: [f64; 21],
    pub(crate) final_g: [f64; 21],
    pub(crate) final_lambda_times_p: [f64; 21],
    pub(crate) final_h: [f64; 21],
    pub(crate) lambda: f64,
    pub(crate) event_trace: Vec<M1TrustRegionKktEvent>,
    pub(crate) entered_svd_factorizations: u8,
    pub(crate) completed_face_pivots: u8,
    pub(crate) raw_jacobian_columns: [[f64; 21]; 21],
    pub(crate) active_lower: [bool; 21],
    pub(crate) active_upper: [bool; 21],
}

#[cfg(test)]
#[derive(Clone, Debug)]
pub(crate) struct M1TrustRegionSubproblemRefusal {
    pub(crate) kind: M1TrustRegionRefusalKind,
    pub(crate) attempted_subproblems: u8,
    pub(crate) completed_jacobi_sweeps: u8,
    pub(crate) completed_face_pivots: u8,
    pub(crate) entered_svd_factorizations: u8,
    pub(crate) raw_residual_max: f64,
}
#[cfg(test)]
mod m1_zero_par_tests;

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

pub mod snow_accuracy_policy;

pub mod transaction_v3_open;
pub use transaction_v3_open::*;

mod transparent_canopy;
pub use transparent_canopy::{TransparentCanopyLiquidBoundaryV1, TransparentCanopyPassageProofV1};
