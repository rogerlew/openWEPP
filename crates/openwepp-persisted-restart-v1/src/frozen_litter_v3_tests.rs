//! Focused frozen-litter V3 persisted-restart contract vectors.

use crate::{
    DIRECT_FROZEN_LITTER_CHECKPOINT_V3_SCHEMA, FROZEN_LITTER_PUBLICATION_AUTHORITY_V3_SCHEMA,
    FROZEN_LITTER_SCIENTIFIC_OWNER_V3_SCHEMA,
};

#[test]
fn v3_schema_tags_are_additive_and_version_specific() {
    assert_eq!(
        DIRECT_FROZEN_LITTER_CHECKPOINT_V3_SCHEMA,
        "OPENWEPP_DIRECT_FROZEN_LITTER_CHECKPOINT_V3"
    );
    assert_eq!(
        FROZEN_LITTER_PUBLICATION_AUTHORITY_V3_SCHEMA,
        "OPENWEPP_FROZEN_LITTER_PUBLICATION_AUTHORITY_V3"
    );
    assert_eq!(
        FROZEN_LITTER_SCIENTIFIC_OWNER_V3_SCHEMA,
        "OPENWEPP_FROZEN_LITTER_SCIENTIFIC_OWNER_V3"
    );
}

#[cfg(feature = "fixtures")]
mod fixtures {
    use std::collections::BTreeMap;

    use openwepp_hillslope_orchestrator::{
        SurfaceLiquidConfigurationV2, SurfaceLiquidOwnedStateV2, SurfaceLiquidOwnerEnvelopeV2,
        SurfaceLiquidOwnerModelDefinitionV2,
        land_surface_energy_shadow::{
            AcceptedNonzeroCarrySplitV4EvidenceV1, AcceptedNonzeroCarryV4SupportEvidenceV1,
            execute_nonzero_carry_successor_after_reload_v1,
        },
        v9_real_consumer_shadow::{FrozenLitterV3Resident, FrozenLitterV4Resident},
    };
    use openwepp_kernel_contract::ResourceOwnerId;
    use openwepp_kernel_contract::TransactionId;
    use openwepp_land_surface_energy::{
        LandSurfaceEnergyConfiguration, LandSurfaceEnergyV3State, PreparedSoilThermalSupportV2,
        Sha256Digest, SoilThermalOwnerCheckpointV2, SoilThermalOwnerEnvelopeV2,
        SoilThermalOwnerRestartV2, SoilThermalReceiptFreeOwnerSealsV2,
        SoilThermalV2MigrationIdentity, V2_MODEL_DEFINITION_SHA256, V2_MODEL_VERSION,
        V2_VEGETATION_MODEL_DEFINITION_SHA256, V2_VEGETATION_MODEL_VERSION,
        migrate_soil_thermal_v1_to_v2, migrate_v2_configuration_to_v3, migrate_v2_state_to_v3,
        prepare_soil_thermal_support_v2, project_validated_v1_runtime_to_v2,
        seal_soil_thermal_receipt_free_owner_v2, validate_soil_thermal_receipt_free_owner_v2,
    };

    use crate::{
        DirectFrozenLitterCheckpointV3, DirectFrozenLitterExactEnthalpyCheckpointV4,
        DirectFrozenLitterExactEnthalpyRestartHostV4, DirectFrozenLitterRestartHostV3,
        ExpectedFrozenLitterCheckpointContextV3, ExpectedFrozenLitterExactEnthalpyContextV4,
        FrozenLitterExpectedScientificContextV3, FrozenLitterProjectionRestartError,
        FrozenLitterProjectionSealAuthorityV3, FrozenLitterPublicationAuthorityV3,
        IsolatedRestoredFrozenLitterCheckpointV3, NativeFrozenLitterProjectionAuthorityV3,
        Sha256Hex, SoilThermalNativeBundleV2, SoilThermalNativeSealAuthorityV2,
        SoilThermalOwnerStateRestartV2, ValidatedFrozenLitterProjectionV3,
        admit_and_install_frozen_litter_checkpoint_v3,
        admit_and_install_frozen_litter_exact_enthalpy_checkpoint_v4,
        admit_frozen_litter_checkpoint_v3, admit_frozen_litter_exact_enthalpy_checkpoint_v4,
        advance_frozen_litter_exact_enthalpy_checkpoint_v4,
        project_frozen_litter_scientific_owner_v3,
        project_receipt_free_soil_thermal_owner_state_v2, restart_authority_owner_fixture,
        to_canonical_bytes,
    };

    const TX: TransactionId = TransactionId(99);
    const SUPPORT_END_NS: u128 = 60_000_000_000;

    fn digest(fill: char) -> String {
        fill.to_string().repeat(64)
    }

    fn wire_digest(fill: char) -> Sha256Hex {
        Sha256Hex::try_new(digest(fill)).expect("wire digest")
    }

    fn typed_digest(fill: char) -> Sha256Digest {
        Sha256Digest::try_new(digest(fill)).expect("typed digest")
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
                return Err("restart join");
            }
            validate_soil_thermal_receipt_free_owner_v2(self.prepared, self.seals)
                .map_err(|_| "restart seal")
        }

        fn validate_checkpoint_seal(
            &self,
            envelope: &SoilThermalOwnerEnvelopeV2,
            seal: &SoilThermalOwnerCheckpointV2,
        ) -> Result<(), &'static str> {
            if envelope != self.prepared.beginning_owner() || seal != &self.seals.checkpoint {
                return Err("checkpoint join");
            }
            validate_soil_thermal_receipt_free_owner_v2(self.prepared, self.seals)
                .map_err(|_| "checkpoint seal")
        }
    }

    struct CarriedSoilAuthority<'a> {
        owner: &'a SoilThermalOwnerEnvelopeV2,
        restart: &'a SoilThermalOwnerRestartV2,
        checkpoint: &'a SoilThermalOwnerCheckpointV2,
    }

    impl SoilThermalNativeSealAuthorityV2 for CarriedSoilAuthority<'_> {
        fn validate_restart_seal(
            &self,
            envelope: &SoilThermalOwnerEnvelopeV2,
            seal: &SoilThermalOwnerRestartV2,
        ) -> Result<(), &'static str> {
            (envelope == self.owner && seal == self.restart)
                .then_some(())
                .ok_or("carried restart join")
        }

        fn validate_checkpoint_seal(
            &self,
            envelope: &SoilThermalOwnerEnvelopeV2,
            seal: &SoilThermalOwnerCheckpointV2,
        ) -> Result<(), &'static str> {
            (envelope == self.owner && seal == self.checkpoint)
                .then_some(())
                .ok_or("carried checkpoint join")
        }
    }

    fn carried_soil_checkpoint(
        support: &AcceptedNonzeroCarryV4SupportEvidenceV1,
    ) -> SoilThermalOwnerCheckpointV2 {
        SoilThermalOwnerCheckpointV2 {
            owner_tag: support.soil_thermal_owner.owner_tag.clone(),
            schema_sha256: support.soil_thermal_owner.schema_sha256.clone(),
            exact_carry_definition_sha256: support
                .soil_thermal_owner
                .exact_carry_definition_sha256
                .clone(),
            parent_v1_state_sha256: support.soil_thermal_owner.parent_v1_state_sha256.clone(),
            owner_state_sha256: support.soil_thermal_owner.state.state_sha256.clone(),
            last_accepted_transaction_id: support
                .soil_thermal_owner
                .state
                .last_accepted_transaction_id,
            receipt_chain_sha256: support.soil_thermal_owner.receipt_chain_sha256.clone(),
            checkpoint_sha256: support.soil_thermal_restart.restart_sha256.clone(),
        }
    }

    #[allow(clippy::too_many_arguments)]
    fn exact_parent_checkpoint(
        evidence: &AcceptedNonzeroCarrySplitV4EvidenceV1,
        support: &AcceptedNonzeroCarryV4SupportEvidenceV1,
        soil_checkpoint: &SoilThermalOwnerCheckpointV2,
        soil_authority: &dyn SoilThermalNativeSealAuthorityV2,
        projection_authority: &dyn FrozenLitterProjectionSealAuthorityV3,
        parent_v2: Sha256Hex,
        run: Sha256Hex,
        topology: Sha256Hex,
    ) -> DirectFrozenLitterCheckpointV3 {
        let mut parent_v1 = restart_authority_owner_fixture()
            .committed
            .scientific
            .soil_thermal
            .clone();
        parent_v1.state_sha256 = Sha256Hex::try_new(
            support
                .soil_thermal_owner
                .parent_v1_state_sha256
                .as_str()
                .to_owned(),
        )
        .expect("support parent V1 state digest");
        parent_v1
            .seal_restart_payload()
            .expect("support parent V1 restart seal");
        let persisted_soil = SoilThermalOwnerStateRestartV2::from_native(
            parent_v1,
            SoilThermalNativeBundleV2 {
                owner_envelope: support.soil_thermal_owner.clone(),
                restart_seal: support.soil_thermal_restart.clone(),
                checkpoint_seal: soil_checkpoint.clone(),
                credit_beginning_owner_envelope: None,
                latest_credit_receipt: None,
                expected_accepted_operands: Vec::new(),
                expected_temperature_projections: Vec::new(),
                native_expected_source_set: None,
                native_orchestrator_seals: None,
            },
            &evidence
                .lse_configuration
                .soil_thermal_configuration
                .owner_id,
            &evidence
                .lse_configuration
                .soil_thermal_configuration
                .configuration_sha256,
            soil_authority,
        )
        .expect("persist support soil owner");
        let projection_bytes = support
            .projection_v3
            .canonical_bytes(&evidence.surface_configuration)
            .expect("support projection V3 bytes");
        let publication =
            FrozenLitterPublicationAuthorityV3::from_projection(&support.projection_v3)
                .expect("support publication authority");
        let validated = projection_authority
            .validate_projection(
                &evidence.surface_configuration,
                &projection_bytes,
                &publication,
            )
            .expect("support projection validation");
        let context = FrozenLitterExpectedScientificContextV3 {
            lse_configuration: &evidence.lse_configuration,
            surface_liquid_configuration: &evidence.surface_configuration,
            soil_thermal_owner_id: &evidence
                .lse_configuration
                .soil_thermal_configuration
                .owner_id,
            soil_thermal_seal_authority: soil_authority,
            projection_seal_authority: projection_authority,
        };
        let scientific = project_frozen_litter_scientific_owner_v3(
            &evidence.lse_configuration,
            &support.ending_lse_state,
            &evidence.surface_configuration,
            &support.ending_surface_owner,
            validated.wb14_parent_working_state_bytes,
            persisted_soil,
            projection_bytes,
            publication,
            &context,
        )
        .expect("support scientific owner");
        DirectFrozenLitterCheckpointV3::new(parent_v2, run, topology, scientific)
            .expect("support parent V3 checkpoint")
    }

    #[derive(Clone)]
    struct RetainedProjectionAuthority {
        expected_bytes: Vec<u8>,
        validated: ValidatedFrozenLitterProjectionV3,
    }

    impl FrozenLitterProjectionSealAuthorityV3 for RetainedProjectionAuthority {
        fn validate_projection(
            &self,
            _configuration: &SurfaceLiquidConfigurationV2,
            bytes: &[u8],
            authority: &FrozenLitterPublicationAuthorityV3,
        ) -> Result<ValidatedFrozenLitterProjectionV3, FrozenLitterProjectionRestartError> {
            if bytes != self.expected_bytes || authority != &self.validated.authority {
                return Err(FrozenLitterProjectionRestartError::Identity);
            }
            Ok(self.validated.clone())
        }
    }

    struct Fixture {
        checkpoint: DirectFrozenLitterCheckpointV3,
        lse_configuration: LandSurfaceEnergyConfiguration,
        surface_configuration: SurfaceLiquidConfigurationV2,
        soil_owner_id: openwepp_kernel_contract::ResourceOwnerId,
        prepared: PreparedSoilThermalSupportV2,
        seals: SoilThermalReceiptFreeOwnerSealsV2,
        projection_authority: RetainedProjectionAuthority,
        parent: Sha256Hex,
        run: Sha256Hex,
        topology: Sha256Hex,
    }

    fn lse_v3_fixture() -> (LandSurfaceEnergyConfiguration, LandSurfaceEnergyV3State) {
        let fixture = restart_authority_owner_fixture();
        let endpoint = &fixture.runtime.endpoint;
        let mut v2_configuration = endpoint.lse_configuration.clone();
        v2_configuration.model_version = V2_MODEL_VERSION.into();
        v2_configuration.model_definition_sha256 =
            Sha256Digest::try_new(V2_MODEL_DEFINITION_SHA256).expect("V2 digest");
        v2_configuration.vegetation_configuration.model_version =
            V2_VEGETATION_MODEL_VERSION.into();
        v2_configuration
            .vegetation_configuration
            .model_definition_sha256 = Sha256Digest::try_new(V2_VEGETATION_MODEL_DEFINITION_SHA256)
            .expect("vegetation V2 digest");
        v2_configuration.configuration_sha256 = v2_configuration
            .canonical_sha256()
            .expect("V2 configuration digest");
        let v2_state = project_validated_v1_runtime_to_v2(
            &endpoint.lse_configuration,
            &endpoint.lse_state,
            &v2_configuration,
            &v2_configuration
                .vegetation_configuration
                .configuration_sha256,
        )
        .expect("V2 state");
        let configuration =
            migrate_v2_configuration_to_v3(&v2_configuration).expect("V3 configuration");
        let mut state =
            migrate_v2_state_to_v3(&v2_configuration, &v2_state, &configuration).expect("V3 state");
        state.0.last_accepted_transaction_id = Some(TX);
        state.0.state_sha256 = state.canonical_sha256().expect("V3 state digest");
        state.validate(&configuration).expect("V3 validation");
        (configuration, state)
    }

    fn surface_v2_fixture(
        lse: &LandSurfaceEnergyV3State,
    ) -> (SurfaceLiquidConfigurationV2, SurfaceLiquidOwnerEnvelopeV2) {
        let parent = restart_authority_owner_fixture()
            .runtime
            .endpoint
            .surface_configuration;
        let depths = parent
            .records
            .iter()
            .filter(|record| record.key.tile_id.as_str() == "forest")
            .map(|record| (record.key.clone(), 0.04))
            .collect();
        let model = SurfaceLiquidOwnerModelDefinitionV2::new(digest('1'), digest('2'), digest('3'))
            .expect("surface model");
        let configuration =
            SurfaceLiquidConfigurationV2::new(parent, model, &depths).expect("surface config");
        let liquid = configuration
            .parent()
            .records
            .iter()
            .map(|record| (record.key.clone(), 0.0))
            .collect::<BTreeMap<_, _>>();
        let ice = liquid.clone();
        let enthalpy = configuration
            .parent()
            .records
            .iter()
            .map(|record| {
                let value = lse
                    .0
                    .tiles
                    .iter()
                    .find(|tile| {
                        tile.ofe_id == record.key.ofe_id && tile.tile_id == record.key.tile_id
                    })
                    .map_or(0.0, |tile| tile.surface_enthalpy_j_m2_tile_ground);
                (record.key.clone(), value)
            })
            .collect();
        let state =
            SurfaceLiquidOwnedStateV2::new_initial(&configuration, &liquid, &ice, &enthalpy, 0)
                .expect("surface state");
        let records = state
            .records()
            .iter()
            .cloned()
            .map(|mut record| {
                record.last_accepted_transaction_id = Some(TX);
                record
            })
            .collect();
        let continuations = state
            .continuations()
            .iter()
            .cloned()
            .map(|mut continuation| {
                continuation.next_interval_index = 1;
                continuation.last_accepted_transaction_id = Some(TX);
                continuation
            })
            .collect();
        let state = SurfaceLiquidOwnedStateV2::try_new(&configuration, records, continuations)
            .expect("accepted surface state");
        let owner =
            SurfaceLiquidOwnerEnvelopeV2::wrap_v2(&configuration, state).expect("surface owner");
        (configuration, owner)
    }

    fn fixture() -> Fixture {
        let owners = restart_authority_owner_fixture();
        let (lse_configuration, lse_state) = lse_v3_fixture();
        let (surface_configuration, surface_owner) = surface_v2_fixture(&lse_state);
        let parent_soil = owners.committed.scientific.soil_thermal.clone();
        let snapshot = parent_soil
            .restore(
                &lse_configuration.soil_thermal_configuration.owner_id,
                &lse_configuration
                    .soil_thermal_configuration
                    .configuration_sha256,
            )
            .expect("V1 soil snapshot");
        let owner = migrate_soil_thermal_v1_to_v2(
            &snapshot,
            SoilThermalV2MigrationIdentity {
                model_version: "OPENWEPP_SOIL_THERMAL_EXACT_CARRY_V2".into(),
                model_definition_sha256: typed_digest('c'),
                run_id: surface_configuration.parent().run_id.to_string(),
                transaction_id: TX,
                support_start_ns: 0,
                support_end_ns: SUPPORT_END_NS,
                receipt_chain_sha256: typed_digest('d'),
            },
        )
        .expect("soil V2 migration");
        let prepared = prepare_soil_thermal_support_v2(&owner, TX, 0, SUPPORT_END_NS)
            .expect("prepared receipt-free support");
        let seals = seal_soil_thermal_receipt_free_owner_v2(&prepared).expect("soil seals");
        let authority = ReceiptFreeAuthority {
            prepared: &prepared,
            seals: &seals,
        };
        let soil_restart =
            project_receipt_free_soil_thermal_owner_state_v2(parent_soil, &prepared, &seals)
                .expect("persisted soil owner");
        let surface_bytes = surface_owner
            .canonical_bytes(surface_configuration.parent(), Some(&surface_configuration))
            .expect("surface bytes");
        let owner_bytes = serde_json::to_vec(&owner).expect("soil owner bytes");
        let restart_bytes = serde_json::to_vec(&seals.restart).expect("soil restart bytes");
        let mut publication = FrozenLitterPublicationAuthorityV3 {
            schema: crate::FROZEN_LITTER_PUBLICATION_AUTHORITY_V3_SCHEMA.into(),
            version: 3,
            run_id: surface_configuration.parent().run_id,
            transaction_id: TX,
            predecessor_transaction_id: None,
            parent_support_start_ns: 0,
            parent_support_end_ns: SUPPORT_END_NS,
            support_start_ns: 0,
            support_end_ns: SUPPORT_END_NS,
            predecessor_receipt_chain_sha256: wire_digest('0'),
            receipt_chain_sha256: wire_digest('e'),
            complete_projection_sha256: wire_digest('f'),
            publication_authority_sha256: wire_digest('0'),
        };
        publication.publication_authority_sha256 =
            publication.compute_digest().expect("publication digest");
        let projection_bytes = b"retained-complete-projection-v3".to_vec();
        let projection_authority = RetainedProjectionAuthority {
            expected_bytes: projection_bytes.clone(),
            validated: ValidatedFrozenLitterProjectionV3 {
                authority: publication.clone(),
                ending_surface_owner_bytes: surface_bytes,
                wb14_parent_working_state_bytes: b"open-wb14-parent-v2".to_vec(),
                soil_thermal_owner_envelope_bytes: owner_bytes,
                soil_thermal_restart_identity_bytes: restart_bytes,
            },
        };
        let context = FrozenLitterExpectedScientificContextV3 {
            lse_configuration: &lse_configuration,
            surface_liquid_configuration: &surface_configuration,
            soil_thermal_owner_id: &lse_configuration.soil_thermal_configuration.owner_id,
            soil_thermal_seal_authority: &authority,
            projection_seal_authority: &projection_authority,
        };
        let scientific = project_frozen_litter_scientific_owner_v3(
            &lse_configuration,
            &lse_state,
            &surface_configuration,
            &surface_owner,
            b"open-wb14-parent-v2".to_vec(),
            soil_restart,
            projection_bytes,
            publication,
            &context,
        )
        .expect("scientific owner");
        let run = wire_digest('a');
        let topology = wire_digest('b');
        let parent = wire_digest('9');
        let checkpoint = DirectFrozenLitterCheckpointV3::new(
            parent.clone(),
            run.clone(),
            topology.clone(),
            scientific,
        )
        .expect("checkpoint");
        Fixture {
            checkpoint,
            lse_configuration,
            surface_configuration,
            soil_owner_id: owner.state.owner_id.clone(),
            prepared,
            seals,
            projection_authority,
            parent,
            run,
            topology,
        }
    }

    fn admit(
        fixture: &Fixture,
        checkpoint: &DirectFrozenLitterCheckpointV3,
    ) -> Result<
        IsolatedRestoredFrozenLitterCheckpointV3,
        crate::FrozenLitterCheckpointAdmissionErrorV3,
    > {
        let authority = ReceiptFreeAuthority {
            prepared: &fixture.prepared,
            seals: &fixture.seals,
        };
        let context = ExpectedFrozenLitterCheckpointContextV3 {
            parent_v2_checkpoint_sha256: &fixture.parent,
            run_identity_sha256: &fixture.run,
            topology_sha256: &fixture.topology,
            scientific: FrozenLitterExpectedScientificContextV3 {
                lse_configuration: &fixture.lse_configuration,
                surface_liquid_configuration: &fixture.surface_configuration,
                soil_thermal_owner_id: &fixture.soil_owner_id,
                soil_thermal_seal_authority: &authority,
                projection_seal_authority: &fixture.projection_authority,
            },
        };
        admit_frozen_litter_checkpoint_v3(
            &to_canonical_bytes(checkpoint).expect("checkpoint bytes"),
            &context,
        )
    }

    #[test]
    fn canonical_roundtrip_restores_every_v3_v2_owner_join() {
        let fixture = fixture();
        let before = to_canonical_bytes(&fixture.checkpoint).expect("canonical checkpoint");
        let parent_v1_before =
            to_canonical_bytes(&fixture.checkpoint.scientific.soil_thermal_v2.parent_v1)
                .expect("parent V1 bytes");
        let soil_v2_before = to_canonical_bytes(&fixture.checkpoint.scientific.soil_thermal_v2)
            .expect("soil V2 bytes");
        let restored = admit(&fixture, &fixture.checkpoint).expect("admission");
        assert_eq!(
            to_canonical_bytes(&restored.persisted).expect("restored bytes"),
            before
        );
        assert_eq!(
            restored.scientific.lse_v3.0.last_accepted_transaction_id,
            Some(TX)
        );
        assert!(restored.scientific.surface_liquid_v2.v2_state().is_some());
        assert_eq!(
            restored.scientific.soil_thermal_v2,
            fixture.prepared.beginning_owner().clone()
        );
        assert_eq!(
            to_canonical_bytes(&restored.persisted.scientific.soil_thermal_v2.parent_v1,)
                .expect("restored parent V1 bytes"),
            parent_v1_before
        );
        assert_eq!(
            to_canonical_bytes(&restored.persisted.scientific.soil_thermal_v2)
                .expect("restored soil V2 bytes"),
            soil_v2_before
        );
        let text = String::from_utf8(before).expect("JSON text");
        for forbidden in ["microstep", "iteration", "residual", "diagnostic"] {
            assert!(
                !text.contains(forbidden),
                "persisted diagnostics: {forbidden}"
            );
        }
    }

    #[test]
    fn receipt_free_v4_checkpoint_rejects_accepted_owner_masquerade() {
        let fixture = fixture();
        let authority = ReceiptFreeAuthority {
            prepared: &fixture.prepared,
            seals: &fixture.seals,
        };
        let parent_context = ExpectedFrozenLitterCheckpointContextV3 {
            parent_v2_checkpoint_sha256: &fixture.parent,
            run_identity_sha256: &fixture.run,
            topology_sha256: &fixture.topology,
            scientific: FrozenLitterExpectedScientificContextV3 {
                lse_configuration: &fixture.lse_configuration,
                surface_liquid_configuration: &fixture.surface_configuration,
                soil_thermal_owner_id: &fixture.soil_owner_id,
                soil_thermal_seal_authority: &authority,
                projection_seal_authority: &fixture.projection_authority,
            },
        };
        let exact_owner_id =
            ResourceOwnerId::try_new("lse-surface-enthalpy-exact").expect("exact owner ID");
        let nested_lse = openwepp_land_surface_energy::LandSurfaceEnergyV3State::from_json(
            &fixture.checkpoint.scientific.lse_v3_state_json,
            &fixture.lse_configuration,
        )
        .expect("nested V3 state");
        let nested_surface =
            openwepp_hillslope_orchestrator::SurfaceLiquidOwnerEnvelopeV2::from_canonical_bytes(
                fixture.surface_configuration.parent(),
                Some(&fixture.surface_configuration),
                &fixture
                    .checkpoint
                    .scientific
                    .surface_liquid_v2_envelope_bytes,
            )
            .expect("nested surface V2 owner");
        let exact_owner =
            openwepp_hillslope_orchestrator::LseSurfaceEnthalpyOwnerEnvelopeV1::adopt_from_frozen_v2_v3(
                exact_owner_id.clone(),
                &fixture.lse_configuration,
                &nested_lse,
                &fixture.surface_configuration,
                &nested_surface,
            )
            .expect("exact owner adoption");
        let context = ExpectedFrozenLitterExactEnthalpyContextV4 {
            parent_v3: parent_context,
            exact_surface_owner_id: &exact_owner_id,
            accepted_support_beginning_lse_v3_state_sha256: &nested_lse.0.state_sha256,
            publication_history_beginning_lse_v3_state_sha256: &nested_lse.0.state_sha256,
        };
        assert!(
            DirectFrozenLitterExactEnthalpyCheckpointV4::receipt_free(
                fixture.checkpoint.clone(),
                exact_owner,
                &context,
            )
            .is_err(),
            "an owner retaining accepted transaction lineage cannot use receipt-free posture",
        );
    }

    #[test]
    fn accepted_negative_zero_v4_checkpoint_reloads_exact_projection_and_rolls_back_poison() {
        let evidence = openwepp_hillslope_orchestrator::land_surface_energy_shadow::
            accepted_negative_zero_v4_evidence_v1();
        let mut parent_v1 = restart_authority_owner_fixture()
            .committed
            .scientific
            .soil_thermal
            .clone();
        parent_v1.state_sha256 = wire_digest('4');
        parent_v1
            .seal_restart_payload()
            .expect("evidence V1 parent restart seal");
        let soil_checkpoint = SoilThermalOwnerCheckpointV2 {
            owner_tag: evidence.soil_thermal_owner.owner_tag.clone(),
            schema_sha256: evidence.soil_thermal_owner.schema_sha256.clone(),
            exact_carry_definition_sha256: evidence
                .soil_thermal_owner
                .exact_carry_definition_sha256
                .clone(),
            parent_v1_state_sha256: evidence.soil_thermal_owner.parent_v1_state_sha256.clone(),
            owner_state_sha256: evidence.soil_thermal_owner.state.state_sha256.clone(),
            last_accepted_transaction_id: evidence
                .soil_thermal_owner
                .state
                .last_accepted_transaction_id,
            receipt_chain_sha256: evidence.soil_thermal_owner.receipt_chain_sha256.clone(),
            checkpoint_sha256: typed_digest('c'),
        };
        let soil_authority = CarriedSoilAuthority {
            owner: &evidence.soil_thermal_owner,
            restart: &evidence.soil_thermal_restart,
            checkpoint: &soil_checkpoint,
        };
        let persisted_soil = SoilThermalOwnerStateRestartV2::from_native(
            parent_v1,
            SoilThermalNativeBundleV2 {
                owner_envelope: evidence.soil_thermal_owner.clone(),
                restart_seal: evidence.soil_thermal_restart.clone(),
                checkpoint_seal: soil_checkpoint.clone(),
                credit_beginning_owner_envelope: None,
                latest_credit_receipt: None,
                expected_accepted_operands: Vec::new(),
                expected_temperature_projections: Vec::new(),
                native_expected_source_set: None,
                native_orchestrator_seals: None,
            },
            &evidence
                .lse_configuration
                .soil_thermal_configuration
                .owner_id,
            &evidence
                .lse_configuration
                .soil_thermal_configuration
                .configuration_sha256,
            &soil_authority,
        )
        .expect("persist accepted evidence soil owner");
        let projection_v3_bytes = evidence
            .projection_v3
            .canonical_bytes(&evidence.surface_configuration)
            .expect("canonical accepted V3 projection");
        let publication =
            FrozenLitterPublicationAuthorityV3::from_projection(&evidence.projection_v3)
                .expect("accepted publication authority");
        let projection_authority = NativeFrozenLitterProjectionAuthorityV3;
        let validated_projection = projection_authority
            .validate_projection(
                &evidence.surface_configuration,
                &projection_v3_bytes,
                &publication,
            )
            .expect("native accepted projection validation");
        let mut physical_resident = FrozenLitterV3Resident::try_new(
            evidence.lse_configuration.clone(),
            evidence.ending_lse_state.clone(),
            evidence.surface_configuration.clone(),
            evidence.ending_surface_owner.clone(),
        )
        .expect("accepted physical V3 resident");
        physical_resident
            .restore_restart_authority(
                std::slice::from_ref(&evidence.physical_v3_publication_bytes),
                (!validated_projection
                    .wb14_parent_working_state_bytes
                    .is_empty())
                .then_some(
                    validated_projection
                        .wb14_parent_working_state_bytes
                        .as_slice(),
                ),
                &evidence.projection_v3.identity().receipt_chain_sha256,
            )
            .expect("authentic accepted V3 publication restore");
        assert_eq!(
            physical_resident
                .accepted_publication_supports_canonical_bytes()
                .expect("restored V3 publication bytes"),
            vec![evidence.physical_v3_publication_bytes.clone()],
        );
        assert_eq!(
            validated_projection.ending_surface_owner_bytes,
            evidence
                .ending_surface_owner
                .canonical_bytes(
                    evidence.surface_configuration.parent(),
                    Some(&evidence.surface_configuration),
                )
                .expect("accepted surface bytes"),
        );
        assert_eq!(
            validated_projection.soil_thermal_owner_envelope_bytes,
            serde_json::to_vec(&evidence.soil_thermal_owner).expect("accepted soil bytes"),
        );
        assert_eq!(
            validated_projection.soil_thermal_restart_identity_bytes,
            serde_json::to_vec(&evidence.soil_thermal_restart).expect("accepted soil restart"),
        );
        let surface_state = evidence
            .ending_surface_owner
            .v2_state()
            .expect("accepted V2 surface state");
        assert_eq!(
            evidence.ending_lse_state.0.tiles.len(),
            surface_state.records().len(),
        );
        for (lse, surface) in evidence
            .ending_lse_state
            .0
            .tiles
            .iter()
            .zip(surface_state.records())
        {
            assert_eq!(
                (&lse.ofe_id, &lse.tile_id),
                (&surface.key.ofe_id, &surface.key.tile_id)
            );
            assert_eq!(
                lse.surface_enthalpy_j_m2_tile_ground.to_bits(),
                surface.surface_enthalpy_j_m2_tile.to_bits(),
            );
            assert_eq!(
                surface.last_accepted_transaction_id,
                Some(evidence.transaction_id)
            );
        }
        assert_eq!(
            evidence.ending_lse_state.0.last_accepted_transaction_id,
            Some(evidence.transaction_id),
        );
        assert_eq!(
            evidence.soil_thermal_owner.transaction_id,
            evidence.transaction_id,
        );
        assert_eq!(
            evidence.soil_thermal_owner.support_start_ns,
            evidence.support_start_ns,
        );
        assert_eq!(
            evidence.soil_thermal_owner.support_end_ns,
            evidence.support_end_ns,
        );
        assert_eq!(
            evidence.surface_configuration.parent().run_id,
            publication.run_id,
        );
        assert_eq!(
            evidence
                .lse_configuration
                .ofes
                .iter()
                .map(|ofe| &ofe.ofe_id)
                .collect::<Vec<_>>(),
            evidence
                .soil_thermal_owner
                .state
                .ofes
                .iter()
                .map(|ofe| &ofe.ofe_id)
                .collect::<Vec<_>>(),
        );
        let scientific_context = FrozenLitterExpectedScientificContextV3 {
            lse_configuration: &evidence.lse_configuration,
            surface_liquid_configuration: &evidence.surface_configuration,
            soil_thermal_owner_id: &evidence
                .lse_configuration
                .soil_thermal_configuration
                .owner_id,
            soil_thermal_seal_authority: &soil_authority,
            projection_seal_authority: &projection_authority,
        };
        let scientific = project_frozen_litter_scientific_owner_v3(
            &evidence.lse_configuration,
            &evidence.ending_lse_state,
            &evidence.surface_configuration,
            &evidence.ending_surface_owner,
            evidence
                .projection_v3
                .wb14_parent_working_state_bytes()
                .to_vec(),
            persisted_soil,
            projection_v3_bytes.clone(),
            publication,
            &scientific_context,
        )
        .expect("accepted V3 scientific owner");
        let parent = wire_digest('9');
        let run = wire_digest('a');
        let topology = wire_digest('b');
        let parent_v3 = DirectFrozenLitterCheckpointV3::new(
            parent.clone(),
            run.clone(),
            topology.clone(),
            scientific,
        )
        .expect("accepted V3 checkpoint");
        let parent_context = ExpectedFrozenLitterCheckpointContextV3 {
            parent_v2_checkpoint_sha256: &parent,
            run_identity_sha256: &run,
            topology_sha256: &topology,
            scientific: scientific_context,
        };
        let exact_owner_id = evidence.beginning_exact_surface_owner.owner_id.clone();
        let context = ExpectedFrozenLitterExactEnthalpyContextV4 {
            parent_v3: parent_context,
            exact_surface_owner_id: &exact_owner_id,
            accepted_support_beginning_lse_v3_state_sha256: &evidence
                .beginning_lse_state
                .0
                .state_sha256,
            publication_history_beginning_lse_v3_state_sha256: &evidence
                .beginning_lse_state
                .0
                .state_sha256,
        };
        let checkpoint = DirectFrozenLitterExactEnthalpyCheckpointV4::accepted_credit(
            parent_v3,
            evidence.beginning_exact_surface_owner.clone(),
            evidence.ending_exact_surface_restart.clone(),
            evidence.ending_exact_surface_checkpoint.clone(),
            evidence.projection_v4.clone(),
            &context,
        )
        .expect("accepted exact-enthalpy checkpoint");
        let bytes = to_canonical_bytes(&checkpoint).expect("canonical accepted V4 checkpoint");
        let restored = admit_frozen_litter_exact_enthalpy_checkpoint_v4(&bytes, &context)
            .expect("accepted V4 admission");

        let forest = restored
            .exact_surface_owner
            .records()
            .iter()
            .find(|record| record.surface_key.tile_id.as_str() == "forest")
            .expect("restored forest exact owner");
        assert_eq!(forest.enthalpy_hi_j_m2_tile.to_bits(), (-0.0_f64).to_bits());
        assert_eq!(
            restored.beginning_exact_surface_owner.as_ref(),
            Some(&evidence.beginning_exact_surface_owner),
        );
        assert_eq!(
            restored.exact_surface_owner,
            evidence.ending_exact_surface_owner
        );
        assert_eq!(
            restored.exact_surface_checkpoint.receipt.as_ref(),
            Some(&evidence.exact_surface_receipt),
        );
        let restored_projection_v4 = restored
            .complete_owner_projection_v4
            .as_ref()
            .expect("accepted V4 projection");
        assert_eq!(
            restored_projection_v4
                .projection_v3(&evidence.surface_configuration)
                .expect("nested V3 projection")
                .canonical_bytes(&evidence.surface_configuration)
                .expect("nested V3 bytes"),
            projection_v3_bytes,
        );
        assert_eq!(
            restored_projection_v4
                .exact_surface_receipt()
                .expect("nested exact source receipt"),
            evidence.exact_surface_receipt,
        );
        assert_eq!(
            restored_projection_v4
                .beginning_exact_surface_owner()
                .expect("nested beginning exact owner"),
            evidence.beginning_exact_surface_owner,
        );
        assert_eq!(
            restored_projection_v4
                .exact_surface_owner()
                .expect("nested ending exact owner"),
            restored.exact_surface_owner,
        );

        let mut host = DirectFrozenLitterExactEnthalpyRestartHostV4::from_isolated(restored);
        let before_poison = host.clone();
        let mut poison = checkpoint;
        poison
            .complete_owner_projection_v4
            .as_mut()
            .expect("accepted projection frame")
            .canonical_json
            .push(b' ');
        poison.seal().expect("reseal poisoned outer checkpoint");
        let poison_bytes =
            to_canonical_bytes(&poison).expect("canonical poisoned outer checkpoint");
        assert!(
            admit_and_install_frozen_litter_exact_enthalpy_checkpoint_v4(
                &mut host,
                &poison_bytes,
                &context,
            )
            .is_err(),
            "poisoned nested source/projection frame must fail closed",
        );
        assert_eq!(host, before_poison, "failed admission must not publish");
    }

    #[test]
    #[allow(clippy::too_many_lines)]
    fn nonzero_carry_split_reload_advances_exact_owner_and_publication_history() {
        let evidence = openwepp_hillslope_orchestrator::land_surface_energy_shadow::
            accepted_nonzero_carry_split_v4_evidence_v1();
        assert!(
            evidence
                .first
                .ending_exact_surface_owner
                .records()
                .iter()
                .any(|record| {
                    record.enthalpy_carry
                        != openwepp_land_surface_energy::ExactDyadicEnthalpy::zero()
                }),
            "the checkpoint boundary must retain a genuine nonzero carry",
        );

        let first_soil_checkpoint = carried_soil_checkpoint(&evidence.first);
        let second_soil_checkpoint = carried_soil_checkpoint(&evidence.second);
        let first_soil_authority = CarriedSoilAuthority {
            owner: &evidence.first.soil_thermal_owner,
            restart: &evidence.first.soil_thermal_restart,
            checkpoint: &first_soil_checkpoint,
        };
        let second_soil_authority = CarriedSoilAuthority {
            owner: &evidence.second.soil_thermal_owner,
            restart: &evidence.second.soil_thermal_restart,
            checkpoint: &second_soil_checkpoint,
        };
        let projection_authority = NativeFrozenLitterProjectionAuthorityV3;
        let first_parent = wire_digest('1');
        let first_run = wire_digest('2');
        let first_topology = wire_digest('3');
        let second_parent = wire_digest('5');
        let second_run = wire_digest('6');
        let second_topology = wire_digest('7');
        let first_parent_checkpoint = exact_parent_checkpoint(
            &evidence,
            &evidence.first,
            &first_soil_checkpoint,
            &first_soil_authority,
            &projection_authority,
            first_parent.clone(),
            first_run.clone(),
            first_topology.clone(),
        );
        let second_parent_checkpoint = exact_parent_checkpoint(
            &evidence,
            &evidence.second,
            &second_soil_checkpoint,
            &second_soil_authority,
            &projection_authority,
            second_parent.clone(),
            second_run.clone(),
            second_topology.clone(),
        );
        let first_scientific_context = FrozenLitterExpectedScientificContextV3 {
            lse_configuration: &evidence.lse_configuration,
            surface_liquid_configuration: &evidence.surface_configuration,
            soil_thermal_owner_id: &evidence
                .lse_configuration
                .soil_thermal_configuration
                .owner_id,
            soil_thermal_seal_authority: &first_soil_authority,
            projection_seal_authority: &projection_authority,
        };
        let second_scientific_context = FrozenLitterExpectedScientificContextV3 {
            lse_configuration: &evidence.lse_configuration,
            surface_liquid_configuration: &evidence.surface_configuration,
            soil_thermal_owner_id: &evidence
                .lse_configuration
                .soil_thermal_configuration
                .owner_id,
            soil_thermal_seal_authority: &second_soil_authority,
            projection_seal_authority: &projection_authority,
        };
        let first_parent_context = ExpectedFrozenLitterCheckpointContextV3 {
            parent_v2_checkpoint_sha256: &first_parent,
            run_identity_sha256: &first_run,
            topology_sha256: &first_topology,
            scientific: first_scientific_context,
        };
        let second_parent_context = ExpectedFrozenLitterCheckpointContextV3 {
            parent_v2_checkpoint_sha256: &second_parent,
            run_identity_sha256: &second_run,
            topology_sha256: &second_topology,
            scientific: second_scientific_context,
        };
        let exact_owner_id = evidence
            .first
            .beginning_exact_surface_owner
            .owner_id
            .clone();
        let first_context = ExpectedFrozenLitterExactEnthalpyContextV4 {
            parent_v3: first_parent_context,
            exact_surface_owner_id: &exact_owner_id,
            accepted_support_beginning_lse_v3_state_sha256: &evidence
                .first
                .beginning_lse_state
                .0
                .state_sha256,
            publication_history_beginning_lse_v3_state_sha256: &evidence
                .first
                .beginning_lse_state
                .0
                .state_sha256,
        };
        let second_context = ExpectedFrozenLitterExactEnthalpyContextV4 {
            parent_v3: second_parent_context,
            exact_surface_owner_id: &exact_owner_id,
            accepted_support_beginning_lse_v3_state_sha256: &evidence
                .second
                .beginning_lse_state
                .0
                .state_sha256,
            publication_history_beginning_lse_v3_state_sha256: &evidence
                .first
                .beginning_lse_state
                .0
                .state_sha256,
        };
        let first_checkpoint = DirectFrozenLitterExactEnthalpyCheckpointV4::accepted_credit(
            first_parent_checkpoint.clone(),
            evidence.first.beginning_exact_surface_owner.clone(),
            evidence.first.ending_exact_surface_restart.clone(),
            evidence.first.ending_exact_surface_checkpoint.clone(),
            evidence.first.projection_v4.clone(),
            &first_context,
        )
        .expect("first accepted exact checkpoint");
        assert!(
            DirectFrozenLitterExactEnthalpyCheckpointV4::receipt_free(
                first_parent_checkpoint,
                evidence.first.ending_exact_surface_owner.clone(),
                &first_context,
            )
            .is_err(),
            "V1/receipt-free omission must refuse an accepted nonzero carry",
        );
        let first_bytes =
            to_canonical_bytes(&first_checkpoint).expect("first exact checkpoint bytes");
        let restored_first =
            admit_frozen_litter_exact_enthalpy_checkpoint_v4(&first_bytes, &first_context)
                .expect("reload first exact checkpoint");
        let mut split_host =
            DirectFrozenLitterExactEnthalpyRestartHostV4::from_isolated(restored_first);
        assert_eq!(
            split_host.admitted().exact_surface_owner,
            evidence.first.ending_exact_surface_owner,
        );
        assert_eq!(
            split_host.admitted().complete_owner_projection_v4.as_ref(),
            Some(&evidence.first.projection_v4),
        );

        let restored_predecessor_projection = split_host
            .admitted()
            .complete_owner_projection_v4
            .as_ref()
            .expect("reloaded exact projection")
            .projection_v3(&evidence.surface_configuration)
            .expect("reloaded physical V3 projection");
        let restored_second = execute_nonzero_carry_successor_after_reload_v1(
            &evidence.lse_configuration,
            &evidence.surface_configuration,
            &split_host.admitted().parent_v3.scientific.lse_v3,
            &split_host.admitted().parent_v3.scientific.surface_liquid_v2,
            &split_host.admitted().exact_surface_owner,
            &restored_predecessor_projection,
            &split_host.admitted().parent_v3.scientific.soil_thermal_v2,
        );
        assert_eq!(
            restored_second, evidence.second,
            "the real successor executor must be split/reload identical",
        );

        let second_checkpoint = DirectFrozenLitterExactEnthalpyCheckpointV4::accepted_credit(
            second_parent_checkpoint,
            restored_second.beginning_exact_surface_owner.clone(),
            restored_second.ending_exact_surface_restart.clone(),
            restored_second.ending_exact_surface_checkpoint.clone(),
            restored_second.projection_v4.clone(),
            &second_context,
        )
        .expect("successor exact checkpoint");
        let mut poison = second_checkpoint.clone();
        poison
            .beginning_exact_surface_owner
            .as_mut()
            .expect("successor beginning frame")
            .canonical_json
            .push(b' ');
        poison.seal().expect("reseal poisoned successor");
        let poison_bytes = to_canonical_bytes(&poison).expect("poisoned successor bytes");
        let before_poison = split_host.clone();
        assert!(
            advance_frozen_litter_exact_enthalpy_checkpoint_v4(
                &mut split_host,
                &poison_bytes,
                &second_context,
            )
            .is_err()
        );
        assert_eq!(
            split_host, before_poison,
            "failed successor admission must be byte-atomic",
        );

        let second_bytes =
            to_canonical_bytes(&second_checkpoint).expect("successor checkpoint bytes");
        advance_frozen_litter_exact_enthalpy_checkpoint_v4(
            &mut split_host,
            &second_bytes,
            &second_context,
        )
        .expect("advance reloaded host through lawful successor");
        let uninterrupted =
            admit_frozen_litter_exact_enthalpy_checkpoint_v4(&second_bytes, &second_context)
                .expect("uninterrupted successor admission");
        assert_eq!(split_host.admitted(), &uninterrupted);
        assert_eq!(
            split_host.admitted().exact_surface_owner.canonical_bytes(),
            restored_second.ending_exact_surface_owner.canonical_bytes(),
        );
        assert_eq!(
            split_host.admitted().exact_surface_restart,
            restored_second.ending_exact_surface_restart,
        );
        assert_eq!(
            split_host.admitted().exact_surface_checkpoint,
            restored_second.ending_exact_surface_checkpoint,
        );

        let physical_publications = vec![
            evidence.first.physical_v3_publication_bytes.clone(),
            evidence.second.physical_v3_publication_bytes.clone(),
        ];
        let mut physical = FrozenLitterV3Resident::try_new(
            evidence.lse_configuration.clone(),
            evidence.second.ending_lse_state.clone(),
            evidence.surface_configuration.clone(),
            evidence.second.ending_surface_owner.clone(),
        )
        .expect("final physical resident");
        physical
            .restore_restart_authority(
                &physical_publications,
                None,
                &evidence
                    .second
                    .projection_v3
                    .identity()
                    .receipt_chain_sha256,
            )
            .expect("reload chained physical V3 publication history");
        assert_eq!(
            physical
                .accepted_publication_supports_canonical_bytes()
                .expect("physical publication bytes"),
            physical_publications,
        );
        let exact_publications = vec![
            evidence
                .first
                .projection_v4
                .canonical_bytes(&evidence.surface_configuration)
                .expect("first exact publication"),
            evidence
                .second
                .projection_v4
                .canonical_bytes(&evidence.surface_configuration)
                .expect("second exact publication"),
        ];
        let exact_resident = FrozenLitterV4Resident::try_restore(
            &physical,
            split_host.admitted().exact_surface_owner.clone(),
            &exact_publications,
            &evidence.first.beginning_lse_state.0.state_sha256,
        )
        .expect("reload chained exact V4 publication history");
        assert_eq!(
            exact_resident.accepted_publication_supports_canonical_bytes(),
            exact_publications,
        );
        assert_eq!(
            exact_resident.exact_surface_owner().canonical_bytes(),
            evidence.second.ending_exact_surface_owner.canonical_bytes(),
        );
    }

    #[test]
    fn cross_version_identity_and_receipt_chain_poisons_fail_closed() {
        let fixture = fixture();
        let mut cross_version = fixture.checkpoint.clone();
        cross_version.version = 2;
        cross_version.seal().expect("outer reseal");
        assert!(admit(&fixture, &cross_version).is_err());

        let mut nested_version = fixture.checkpoint.clone();
        nested_version.scientific.version = 2;
        nested_version.scientific.scientific_owner_sha256 = nested_version
            .scientific
            .compute_digest()
            .expect("scientific reseal");
        nested_version.seal().expect("outer reseal");
        assert!(admit(&fixture, &nested_version).is_err());

        let v1_state = restart_authority_owner_fixture()
            .runtime
            .endpoint
            .frame
            .surface_liquid_shadow
            .as_deref()
            .expect("V1 surface state")
            .clone();
        let v1_envelope = SurfaceLiquidOwnerEnvelopeV2::wrap_v1(
            fixture.surface_configuration.parent(),
            v1_state,
            digest('3'),
        )
        .expect("V1 envelope");
        let mut nested_surface_version = fixture.checkpoint.clone();
        nested_surface_version
            .scientific
            .surface_liquid_v2_envelope_bytes = v1_envelope
            .canonical_bytes(fixture.surface_configuration.parent(), None)
            .expect("V1 envelope bytes");
        nested_surface_version.scientific.scientific_owner_sha256 = nested_surface_version
            .scientific
            .compute_digest()
            .expect("scientific reseal");
        nested_surface_version.seal().expect("outer reseal");
        assert!(admit(&fixture, &nested_surface_version).is_err());

        let mut run = fixture.checkpoint.clone();
        run.run_identity_sha256 = wire_digest('7');
        run.seal().expect("outer reseal");
        assert!(admit(&fixture, &run).is_err());

        let mut parent = fixture.checkpoint.clone();
        parent.parent_v2_checkpoint_sha256 = wire_digest('5');
        parent.seal().expect("outer reseal");
        assert!(admit(&fixture, &parent).is_err());

        let mut topology = fixture.checkpoint.clone();
        topology.topology_sha256 = wire_digest('8');
        topology.seal().expect("outer reseal");
        assert!(admit(&fixture, &topology).is_err());

        let mut enthalpy = fixture.checkpoint.clone();
        let mut mismatched_lse = LandSurfaceEnergyV3State::from_json(
            &enthalpy.scientific.lse_v3_state_json,
            &fixture.lse_configuration,
        )
        .expect("LSE V3 replay");
        mismatched_lse.0.tiles[0].surface_enthalpy_j_m2_tile_ground += 1.0;
        mismatched_lse.0.state_sha256 = mismatched_lse
            .canonical_sha256()
            .expect("mismatched LSE digest");
        enthalpy.scientific.lse_v3_state_json =
            mismatched_lse.to_json().expect("mismatched LSE bytes");
        enthalpy.scientific.scientific_owner_sha256 = enthalpy
            .scientific
            .compute_digest()
            .expect("scientific reseal");
        enthalpy.seal().expect("outer reseal");
        assert!(admit(&fixture, &enthalpy).is_err());

        let mut wb14 = fixture.checkpoint.clone();
        wb14.scientific.wb14_v2_parent_working_state_bytes = b"stale-wb14-parent-v2".to_vec();
        wb14.scientific.scientific_owner_sha256 =
            wb14.scientific.compute_digest().expect("scientific reseal");
        wb14.seal().expect("outer reseal");
        assert!(admit(&fixture, &wb14).is_err());

        let mut chain = fixture.checkpoint.clone();
        chain.scientific.publication_authority.receipt_chain_sha256 = wire_digest('6');
        chain
            .scientific
            .publication_authority
            .publication_authority_sha256 = chain
            .scientific
            .publication_authority
            .compute_digest()
            .expect("authority reseal");
        chain.scientific.scientific_owner_sha256 = chain
            .scientific
            .compute_digest()
            .expect("scientific reseal");
        chain.seal().expect("outer reseal");
        assert!(admit(&fixture, &chain).is_err());

        let canonical = to_canonical_bytes(&fixture.checkpoint).expect("canonical bytes");
        let mut noncanonical = b" ".to_vec();
        noncanonical.extend(canonical);
        let authority = ReceiptFreeAuthority {
            prepared: &fixture.prepared,
            seals: &fixture.seals,
        };
        let context = ExpectedFrozenLitterCheckpointContextV3 {
            parent_v2_checkpoint_sha256: &fixture.parent,
            run_identity_sha256: &fixture.run,
            topology_sha256: &fixture.topology,
            scientific: FrozenLitterExpectedScientificContextV3 {
                lse_configuration: &fixture.lse_configuration,
                surface_liquid_configuration: &fixture.surface_configuration,
                soil_thermal_owner_id: &fixture.soil_owner_id,
                soil_thermal_seal_authority: &authority,
                projection_seal_authority: &fixture.projection_authority,
            },
        };
        assert!(admit_frozen_litter_checkpoint_v3(&noncanonical, &context).is_err());
    }

    #[test]
    fn failed_atomic_install_preserves_exact_prior_host() {
        let fixture = fixture();
        let admitted = admit(&fixture, &fixture.checkpoint).expect("initial admission");
        let mut host = DirectFrozenLitterRestartHostV3::from_isolated(admitted);
        let before = host.clone();
        let mut poison = fixture.checkpoint.clone();
        poison.payload_sha256 = wire_digest('1');
        let authority = ReceiptFreeAuthority {
            prepared: &fixture.prepared,
            seals: &fixture.seals,
        };
        let context = ExpectedFrozenLitterCheckpointContextV3 {
            parent_v2_checkpoint_sha256: &fixture.parent,
            run_identity_sha256: &fixture.run,
            topology_sha256: &fixture.topology,
            scientific: FrozenLitterExpectedScientificContextV3 {
                lse_configuration: &fixture.lse_configuration,
                surface_liquid_configuration: &fixture.surface_configuration,
                soil_thermal_owner_id: &fixture.soil_owner_id,
                soil_thermal_seal_authority: &authority,
                projection_seal_authority: &fixture.projection_authority,
            },
        };
        let result = admit_and_install_frozen_litter_checkpoint_v3(
            &mut host,
            &to_canonical_bytes(&poison).expect("poison bytes"),
            &context,
        );
        assert!(result.is_err());
        assert_eq!(host, before);
    }
}
