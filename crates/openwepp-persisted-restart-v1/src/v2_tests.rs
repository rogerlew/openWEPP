//! Contract-derived persisted-restart V2 tests.

use serde::{Deserialize, Serialize};

use crate::{CanonicalNativeFrameV2, SoilThermalRestartV2Error};

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
struct FrameProbe {
    high_bits: u64,
    carry: String,
}

#[test]
fn canonical_native_frame_reencodes_exactly_and_refuses_wrong_type_or_bytes() {
    let probe = FrameProbe {
        high_bits: (-34315.42154113602_f64).to_bits(),
        carry: "-1dc319224e55f@-109".to_owned(),
    };
    let frame = CanonicalNativeFrameV2::encode("FrameProbe", &probe).expect("canonical frame");
    assert_eq!(
        frame.decode::<FrameProbe>("FrameProbe").expect("decode"),
        probe
    );
    assert_eq!(
        frame
            .decode::<FrameProbe>("WrongType")
            .expect_err("wrong native type"),
        SoilThermalRestartV2Error::NativeType
    );
    let mut poison = frame;
    poison.canonical_json_base64.push('A');
    assert!(poison.decode::<FrameProbe>("FrameProbe").is_err());
}

#[cfg(feature = "fixtures")]
mod fixtures {
    use openwepp_hillslope_orchestrator::v9_real_consumer_shadow::{
        SoilThermalExpectedAcceptedOperandSetV2, aggregate_soil_thermal_ending_v2,
        seal_soil_thermal_accepted_candidate_v2,
    };
    use openwepp_kernel_contract::TransactionId;
    use openwepp_land_surface_energy::{
        ExactDyadicEnthalpy, PreparedSoilThermalSupportV2, Sha256Digest,
        SoilThermalOwnerCheckpointV2, SoilThermalOwnerEnvelopeV2, SoilThermalOwnerRestartV2,
        SoilThermalReceiptFreeOwnerSealsV2, SoilThermalV2MigrationIdentity,
        prepare_soil_thermal_support_v2, seal_soil_thermal_receipt_free_owner_v2,
        validate_soil_thermal_receipt_free_owner_v2,
    };

    use crate::{
        AcceptedIntervalCount, DirectV10CheckpointPhaseV2, DirectV10ContinuationTemplateRestartV1,
        DirectV10NativeOwnerHostV2, DirectV10PreparedDayTransactionV2,
        DirectV10RealConsumerCheckpointV2, DirectV10RestartHostV2, ExpectedRestartStaticContextV2,
        IsolatedRestoredCheckpointV2, PreparedDayWireOwnersV2, RestartAdmissionFailureV2,
        Sha256Hex, SoilThermalNativeSealAuthorityV2, SoilThermalOwnerStateRestartV2,
        SoilThermalRestartV2Error, WireDayIndex, admit_and_install_checkpoint_v2,
        admit_checkpoint_v2, bootstrap_complete_owner_state_v1_to_v2,
        bootstrap_soil_thermal_restart_v1_to_v2, checkpoint_identities_v1,
        checkpoint_identities_v2, project_receipt_free_soil_thermal_owner_state_v2,
        refuse_soil_thermal_restart_v2_to_v1, restart_authority_owner_fixture,
        restart_authority_prepared_day_fixture, substitute_complete_soil_owner_v2,
        to_canonical_bytes,
    };

    fn digest(fill: char) -> Sha256Digest {
        Sha256Digest::try_new(fill.to_string().repeat(64)).expect("digest")
    }

    fn wire_digest(fill: char) -> Sha256Hex {
        Sha256Hex::try_new(fill.to_string().repeat(64)).expect("wire digest")
    }

    struct ReceiptFreeAuthority<'a> {
        prepared: &'a PreparedSoilThermalSupportV2,
        seals: &'a SoilThermalReceiptFreeOwnerSealsV2,
    }

    impl SoilThermalNativeSealAuthorityV2 for ReceiptFreeAuthority<'_> {
        fn validate_restart_seal(
            &self,
            envelope: &SoilThermalOwnerEnvelopeV2,
            seal: &SoilThermalOwnerRestartV2,
        ) -> Result<(), &'static str> {
            if envelope != self.prepared.beginning_owner() || seal != &self.seals.restart {
                return Err("receipt-free restart join");
            }
            validate_soil_thermal_receipt_free_owner_v2(self.prepared, self.seals)
                .map_err(|_| "receipt-free restart validation")
        }

        fn validate_checkpoint_seal(
            &self,
            envelope: &SoilThermalOwnerEnvelopeV2,
            seal: &SoilThermalOwnerCheckpointV2,
        ) -> Result<(), &'static str> {
            if envelope != self.prepared.beginning_owner() || seal != &self.seals.checkpoint {
                return Err("receipt-free checkpoint join");
            }
            validate_soil_thermal_receipt_free_owner_v2(self.prepared, self.seals)
                .map_err(|_| "receipt-free checkpoint validation")
        }
    }

    fn receipt_free_material(
        envelope: &SoilThermalOwnerEnvelopeV2,
    ) -> (
        PreparedSoilThermalSupportV2,
        SoilThermalReceiptFreeOwnerSealsV2,
    ) {
        let prepared = prepare_soil_thermal_support_v2(
            envelope,
            envelope.transaction_id,
            envelope.support_start_ns,
            envelope.support_end_ns,
        )
        .expect("prepare receipt-free owner");
        let seals =
            seal_soil_thermal_receipt_free_owner_v2(&prepared).expect("seal receipt-free owner");
        (prepared, seals)
    }

    fn migrated() -> (
        crate::RestartAuthorityOwnerFixture,
        SoilThermalOwnerStateRestartV2,
    ) {
        let fixture = restart_authority_owner_fixture();
        let parent = fixture.owners().scientific.soil_thermal.clone();
        let shadow = &fixture.runtime.shadow;
        let owner = shadow
            .restart_authority_soil_thermal()
            .expect("V1 fixture soil resident")
            .owner_id
            .clone();
        let configuration = shadow
            .restart_authority_soil_thermal()
            .expect("V1 fixture soil resident")
            .configuration_sha256
            .clone();
        let identity = SoilThermalV2MigrationIdentity {
            model_version: "OPENWEPP_SOIL_THERMAL_EXACT_CARRY_V2".to_owned(),
            model_definition_sha256: digest('c'),
            run_id: "persisted-restart-v2-test".to_owned(),
            transaction_id: TransactionId(99),
            support_start_ns: 0,
            support_end_ns: 60_000_000_000,
            receipt_chain_sha256: digest('d'),
        };
        let state =
            bootstrap_soil_thermal_restart_v1_to_v2(parent, &owner, &configuration, identity)
                .expect("checked migration");
        (fixture, state)
    }

    trait Owners {
        fn owners(&self) -> &crate::CompleteCommittedOwnerStateV1;
    }

    impl Owners for crate::RestartAuthorityOwnerFixture {
        fn owners(&self) -> &crate::CompleteCommittedOwnerStateV1 {
            &self.committed
        }
    }

    #[test]
    fn v1_bytes_are_frozen_and_migration_copies_high_bits_with_zero_carry() {
        let fixture = restart_authority_owner_fixture();
        let parent = fixture.committed.scientific.soil_thermal.clone();
        let before = to_canonical_bytes(&parent).expect("V1 bytes");
        let (_, state) = migrated();
        assert_eq!(
            before,
            to_canonical_bytes(&parent).expect("unchanged V1 bytes")
        );
        let native = state.decode_native().expect("native V2");
        for (v1_ofe, v2_ofe) in parent.ofes.iter().zip(&native.owner_envelope.state.ofes) {
            for (v1, v2) in v1_ofe.ordered_layers.iter().zip(&v2_ofe.ordered_layers) {
                assert_eq!(
                    v1.temperature_k.to_f64().to_bits(),
                    v2.temperature_k.to_bits()
                );
                assert_eq!(
                    v1.enthalpy_j_m2_ofe_ground.to_f64().to_bits(),
                    v2.enthalpy_hi_j_m2_ofe_ground.to_bits()
                );
                assert_eq!(v2.enthalpy_carry, ExactDyadicEnthalpy::zero());
            }
        }
        assert_eq!(
            refuse_soil_thermal_restart_v2_to_v1(&state),
            Err(SoilThermalRestartV2Error::DowngradeProhibited)
        );
    }

    #[test]
    fn wat5_nonzero_carry_round_trips_through_canonical_native_frames() {
        let (fixture, migrated) = migrated();
        let owner = fixture
            .runtime
            .shadow
            .restart_authority_soil_thermal()
            .expect("V1 fixture soil resident")
            .owner_id
            .clone();
        let configuration = fixture
            .runtime
            .shadow
            .restart_authority_soil_thermal()
            .expect("V1 fixture soil resident")
            .configuration_sha256
            .clone();
        let parent = migrated.parent_v1.clone();
        let mut envelope = migrated.decode_native().expect("native").owner_envelope;
        let layer = &mut envelope.state.ofes[0].ordered_layers[0];
        layer.enthalpy_hi_j_m2_ofe_ground = -34315.42154113602;
        layer.enthalpy_carry =
            ExactDyadicEnthalpy::try_new(-1, "1dc319224e55f", -109).expect("WAT5 carry");
        envelope.state.reseal().expect("reseal state");
        let prepared = prepare_soil_thermal_support_v2(
            &envelope,
            envelope.transaction_id,
            envelope.support_start_ns,
            envelope.support_end_ns,
        )
        .expect("prepare WAT5 owner");
        let seals = seal_soil_thermal_receipt_free_owner_v2(&prepared).expect("seal WAT5 owner");
        let state = project_receipt_free_soil_thermal_owner_state_v2(parent, &prepared, &seals)
            .expect("persist WAT5 carry");
        let bytes = to_canonical_bytes(&state).expect("canonical bytes");
        let decoded: SoilThermalOwnerStateRestartV2 =
            crate::from_canonical_bytes(&bytes).expect("canonical decode");
        let authority = ReceiptFreeAuthority {
            prepared: &prepared,
            seals: &seals,
        };
        let layer = &decoded
            .validate(&owner, &configuration, &authority)
            .expect("validate")
            .state
            .ofes[0]
            .ordered_layers[0];
        assert_eq!(
            layer.enthalpy_hi_j_m2_ofe_ground.to_bits(),
            (-34315.42154113602_f64).to_bits()
        );
        assert_eq!(
            layer.enthalpy_carry,
            ExactDyadicEnthalpy::try_new(-1, "1dc319224e55f", -109).expect("WAT5 carry")
        );
    }

    #[test]
    fn native_accepted_candidate_receipt_expected_set_and_seals_revalidate() {
        let (fixture, migrated) = migrated();
        let beginning = migrated
            .decode_native()
            .expect("native migration")
            .owner_envelope;
        let (receipt_free_prepared, receipt_free_seals) = receipt_free_material(&beginning);
        let configuration = fixture.runtime.shadow.restart_authority_lse_configuration();
        let expected =
            SoilThermalExpectedAcceptedOperandSetV2::try_new(&beginning, configuration, Vec::new())
                .expect("independent empty physical set");
        let candidate = aggregate_soil_thermal_ending_v2(&beginning, configuration, &expected)
            .expect("accepted no-op candidate");
        let seals =
            seal_soil_thermal_accepted_candidate_v2(&beginning, &candidate).expect("native seals");
        let state = SoilThermalOwnerStateRestartV2::from_accepted_candidate(
            migrated.parent_v1,
            beginning,
            candidate,
            seals,
            configuration,
        )
        .expect("persist accepted candidate");
        let bytes = to_canonical_bytes(&state).expect("persisted candidate bytes");
        let decoded: SoilThermalOwnerStateRestartV2 =
            crate::from_canonical_bytes(&bytes).expect("canonical candidate bytes");
        let receipt_free_authority = ReceiptFreeAuthority {
            prepared: &receipt_free_prepared,
            seals: &receipt_free_seals,
        };
        decoded
            .validate_with_configuration(
                &configuration.soil_thermal_configuration.owner_id,
                configuration,
                &receipt_free_authority,
            )
            .expect("native receipt and seal replay");
    }

    #[test]
    fn synthetic_split_before_and_after_credit_is_admitted_and_replay_refuses() {
        let fixture = restart_authority_prepared_day_fixture();
        let shadow = &fixture.owners.runtime.shadow;
        let configuration = shadow.restart_authority_lse_configuration();
        let committed = bootstrap_complete_owner_state_v1_to_v2(
            fixture.owners.committed.clone(),
            &shadow
                .restart_authority_soil_thermal()
                .expect("V1 fixture soil resident")
                .owner_id,
            &shadow
                .restart_authority_soil_thermal()
                .expect("V1 fixture soil resident")
                .configuration_sha256,
            SoilThermalV2MigrationIdentity {
                model_version: "OPENWEPP_SOIL_THERMAL_EXACT_CARRY_V2".to_owned(),
                model_definition_sha256: digest('c'),
                run_id: "persisted-restart-v2-split".to_owned(),
                transaction_id: TransactionId(101),
                support_start_ns: 0,
                support_end_ns: 60_000_000_000,
                receipt_chain_sha256: digest('d'),
            },
        )
        .expect("split migration");
        let beginning = committed
            .scientific
            .soil_thermal_v2
            .decode_native()
            .expect("beginning native")
            .owner_envelope;
        let prepared_native = prepare_soil_thermal_support_v2(
            &beginning,
            beginning.transaction_id,
            beginning.support_start_ns,
            beginning.support_end_ns,
        )
        .expect("prepare native beginning");
        let receipt_free_seals = seal_soil_thermal_receipt_free_owner_v2(&prepared_native)
            .expect("seal native beginning");
        let host = DirectV10NativeOwnerHostV2::from_receipt_free_native(
            committed.clone(),
            &prepared_native,
            &receipt_free_seals,
            configuration,
        )
        .expect("native owner host");
        let target_before_refusal = host.clone();
        let mut poisoned_seals = receipt_free_seals.clone();
        poisoned_seals.receipt_free_seal_sha256 = digest('f');
        assert!(
            DirectV10NativeOwnerHostV2::from_receipt_free_native(
                committed.clone(),
                &prepared_native,
                &poisoned_seals,
                configuration,
            )
            .is_err()
        );
        assert_eq!(host, target_before_refusal, "native host refusal is atomic");
        let expected =
            SoilThermalExpectedAcceptedOperandSetV2::try_new(&beginning, configuration, Vec::new())
                .expect("split expected set");
        let candidate = aggregate_soil_thermal_ending_v2(&beginning, configuration, &expected)
            .expect("split candidate");
        let seals =
            seal_soil_thermal_accepted_candidate_v2(&beginning, &candidate).expect("split seals");
        let (run, topology) =
            checkpoint_identities_v2(&committed, shadow.root_zone_hydraulic_configuration())
                .expect("split identities");
        let receipt_free_authority = ReceiptFreeAuthority {
            prepared: &prepared_native,
            seals: &receipt_free_seals,
        };
        let context = ExpectedRestartStaticContextV2 {
            run_identity_sha256: &run,
            topology_sha256: &topology,
            soil_thermal_owner_id: &shadow
                .restart_authority_soil_thermal()
                .expect("V1 fixture soil resident")
                .owner_id,
            lse_configuration: configuration,
            native_seal_authority: &receipt_free_authority,
        };
        let prepared = PreparedDayWireOwnersV2 {
            accepted_gsi_daily_receipt: fixture.gsi_receipt.clone(),
            staged_gsi_ending_state: fixture.ending_gsi_state.clone(),
            ending_provider_cursor: fixture.ending_cursor.clone(),
            validated_forcing_day_receipts: fixture.forcing_receipts.clone(),
            continuation_template: DirectV10ContinuationTemplateRestartV1::project(
                &fixture.template,
            ),
        };
        let mut transaction = DirectV10PreparedDayTransactionV2::prepare_from_native_host(
            &host,
            prepared,
            wire_digest('e'),
            run.clone(),
            topology.clone(),
            0,
            0,
        )
        .expect("prepare V2 split");
        let before = transaction.checkpoint().expect("before-credit checkpoint");
        admit_checkpoint_v2(&before, &context).expect("admit before-credit split");
        transaction
            .accept_native_soil_candidate(
                beginning.clone(),
                candidate.clone(),
                seals.clone(),
                configuration,
                &context,
            )
            .expect("accept exact credit");
        let after = transaction.checkpoint().expect("after-credit checkpoint");
        admit_checkpoint_v2(&after, &context).expect("admit after-credit split");
        let before_replay = transaction.checkpoint().expect("before replay");
        assert!(
            transaction
                .accept_native_soil_candidate(beginning, candidate, seals, configuration, &context,)
                .is_err(),
            "same credit cannot replay against its own ending owner"
        );
        assert_eq!(
            transaction.checkpoint().expect("after replay refusal"),
            before_replay,
            "replay refusal is atomic"
        );
    }

    fn between_days_checkpoint() -> (
        DirectV10RealConsumerCheckpointV2,
        crate::RestartAuthorityOwnerFixture,
    ) {
        let (fixture, soil) = migrated();
        let committed = substitute_complete_soil_owner_v2(fixture.committed.clone(), soil);
        let (run, topology) = checkpoint_identities_v1(
            &fixture.committed,
            fixture.runtime.shadow.root_zone_hydraulic_configuration(),
        )
        .expect("identities");
        let (run_v2, topology_v2) = checkpoint_identities_v2(
            &committed,
            fixture.runtime.shadow.root_zone_hydraulic_configuration(),
        )
        .expect("V2 identities");
        assert_eq!(run_v2, run);
        assert_eq!(topology_v2, topology);
        let mut checkpoint = DirectV10RealConsumerCheckpointV2 {
            schema: crate::DIRECT_V10_CHECKPOINT_V2_SCHEMA.to_owned(),
            version: 2,
            parent_v1_checkpoint_sha256: wire_digest('e'),
            run_identity_sha256: run,
            topology_sha256: topology,
            phase: DirectV10CheckpointPhaseV2::BetweenDays {
                next_day_index: WireDayIndex(0),
                accepted_interval_count: AcceptedIntervalCount::try_new(0).expect("zero accepted"),
                committed,
            },
            payload_sha256: wire_digest('0'),
        };
        checkpoint.seal().expect("checkpoint seal");
        (checkpoint, fixture)
    }

    #[test]
    fn checkpoint_admission_and_atomic_host_rollback_are_fail_closed() {
        let (checkpoint, fixture) = between_days_checkpoint();
        let bytes = to_canonical_bytes(&checkpoint).expect("checkpoint bytes");
        let envelope = match &checkpoint.phase {
            DirectV10CheckpointPhaseV2::BetweenDays { committed, .. } => {
                committed
                    .scientific
                    .soil_thermal_v2
                    .decode_native()
                    .expect("checkpoint soil owner")
                    .owner_envelope
            }
            DirectV10CheckpointPhaseV2::InProgressDay { .. } => panic!("between-days fixture"),
        };
        let (prepared, seals) = receipt_free_material(&envelope);
        let authority = ReceiptFreeAuthority {
            prepared: &prepared,
            seals: &seals,
        };
        let context = ExpectedRestartStaticContextV2 {
            run_identity_sha256: &checkpoint.run_identity_sha256,
            topology_sha256: &checkpoint.topology_sha256,
            soil_thermal_owner_id: &fixture
                .runtime
                .shadow
                .restart_authority_soil_thermal()
                .expect("V1 fixture soil resident")
                .owner_id,
            lse_configuration: fixture.runtime.shadow.restart_authority_lse_configuration(),
            native_seal_authority: &authority,
        };
        let admitted = admit_checkpoint_v2(&bytes, &context).expect("admit V2");
        let mut host = DirectV10RestartHostV2::from_isolated(admitted.clone());
        let before = host.clone();
        let mut poison = bytes;
        poison.push(b' ');
        assert_eq!(
            admit_and_install_checkpoint_v2(&mut host, &poison, &context),
            Err(RestartAdmissionFailureV2::NoncanonicalBytes)
        );
        assert_eq!(host, before);
        assert!(matches!(
            admitted,
            IsolatedRestoredCheckpointV2::BetweenDays { .. }
        ));
    }

    #[test]
    fn omission_reorder_cross_version_and_diagnostic_keys_refuse() {
        let (checkpoint, fixture) = between_days_checkpoint();
        let envelope = match &checkpoint.phase {
            DirectV10CheckpointPhaseV2::BetweenDays { committed, .. } => {
                committed
                    .scientific
                    .soil_thermal_v2
                    .decode_native()
                    .expect("checkpoint soil owner")
                    .owner_envelope
            }
            DirectV10CheckpointPhaseV2::InProgressDay { .. } => panic!("between-days fixture"),
        };
        let (prepared, seals) = receipt_free_material(&envelope);
        let authority = ReceiptFreeAuthority {
            prepared: &prepared,
            seals: &seals,
        };
        let context = ExpectedRestartStaticContextV2 {
            run_identity_sha256: &checkpoint.run_identity_sha256,
            topology_sha256: &checkpoint.topology_sha256,
            soil_thermal_owner_id: &fixture
                .runtime
                .shadow
                .restart_authority_soil_thermal()
                .expect("V1 fixture soil resident")
                .owner_id,
            lse_configuration: fixture.runtime.shadow.restart_authority_lse_configuration(),
            native_seal_authority: &authority,
        };
        let bytes = to_canonical_bytes(&checkpoint).expect("bytes");
        let text = std::str::from_utf8(&bytes).expect("UTF-8 checkpoint");
        for forbidden in [
            "microstep",
            "iteration",
            "solver",
            "diagnostic",
            "carry_diagnostic",
        ] {
            assert!(
                !text.contains(forbidden),
                "forbidden serialized key {forbidden}"
            );
        }

        let mut cross = checkpoint.clone();
        cross.schema = "OPENWEPP_DIRECT_V10_REAL_CONSUMER_CHECKPOINT_V1".to_owned();
        cross.seal().expect("reseal cross-version poison");
        assert_eq!(
            admit_checkpoint_v2(&to_canonical_bytes(&cross).expect("cross bytes"), &context),
            Err(RestartAdmissionFailureV2::Schema)
        );

        let mut value = serde_json::to_value(&checkpoint).expect("checkpoint value");
        value
            .as_object_mut()
            .expect("object")
            .remove("topology_sha256");
        assert_eq!(
            admit_checkpoint_v2(
                &serde_json::to_vec(&value).expect("omission bytes"),
                &context
            ),
            Err(RestartAdmissionFailureV2::NoncanonicalBytes)
        );
        let reordered = format!(
            "{{\"version\":2,\"schema\":{} }}",
            serde_json::to_string(crate::DIRECT_V10_CHECKPOINT_V2_SCHEMA).expect("schema")
        );
        assert_eq!(
            admit_checkpoint_v2(reordered.as_bytes(), &context),
            Err(RestartAdmissionFailureV2::NoncanonicalBytes)
        );
    }
}
