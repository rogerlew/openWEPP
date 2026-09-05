#[cfg(test)]
mod tests {
    use super::*;
    use openwepp_coupled_time::{
        ConstraintClass, CoupledClockStateV1, CoupledSlabCandidateV1, LedgerEntryV1,
        ParentAuthorityV1, ParentIntervalId, StepConstraintV1, accept_slab,
        complete_owner_set_digest, digest_bytes, reduce_constraints,
    };

    #[test]
    fn shared_inventory_regrouping_bound_is_exactly_one_ulp() {
        let value = 1.0_f64;
        assert!(nonnegative_finite_values_within_one_ulp(value, value));
        assert!(nonnegative_finite_values_within_one_ulp(
            value,
            f64::from_bits(value.to_bits() + 1),
        ));
        assert!(!nonnegative_finite_values_within_one_ulp(
            value,
            f64::from_bits(value.to_bits() + 2),
        ));
        assert!(!nonnegative_finite_values_within_one_ulp(f64::NAN, value,));
        assert!(!nonnegative_finite_values_within_one_ulp(-value, value));
    }

    #[test]
    fn imported_executor_error_display_retains_nested_typed_cause() {
        let error = V11ExecutionError::Executor("nested-direct-v10-cause");
        assert_eq!(
            error.to_string(),
            "VEG-E-123: imported V10 segment execution failed: nested-direct-v10-cause"
        );
    }

    fn v10_fixture() -> (VegetationConfiguration, V10CoupledOwnedState) {
        let (v8_config, v8_state) = crate::v8_state::v8_test_fixture();
        let mut configuration = v8_config;
        configuration.model_definition_sha256 = V10_MODEL_SHA256.into();
        configuration.configuration_sha256 = configuration.canonical_sha256().expect("config");
        let mut physical = v8_state;
        physical.model_definition_sha256 = V10_MODEL_SHA256.into();
        physical.configuration_sha256 = configuration.configuration_sha256.clone();
        physical.state_sha256 = physical.canonical_sha256();
        configuration.initial_state_sha256 = physical.state_sha256.clone();
        (configuration, V10CoupledOwnedState(physical))
    }

    fn digest(byte: u8) -> Digest32 {
        Digest32::from_bytes([byte; 32])
    }

    fn complete_owners(state: &V11CoupledOwnedState) -> BTreeMap<String, V11OwnerEnvelope> {
        V11_COMPLETE_OWNER_MANIFEST
            .into_iter()
            .map(|id| {
                let envelope = if id == "vegetation" {
                    v11_vegetation_owner_envelope(state).expect("vegetation owner")
                } else if id == "land_surface_energy" {
                    V11OwnerEnvelope::try_new(
                        id.into(),
                        serde_json::to_vec(&serde_json::json!({
                            "configuration_sha256": "a".repeat(64),
                            "state_sha256": "b".repeat(64),
                        }))
                        .expect("LSE state"),
                    )
                    .expect("owner")
                } else if id == "soil_thermal" {
                    V11OwnerEnvelope::try_new(
                        id.into(),
                        serde_json::to_vec(&serde_json::json!({
                            "state_sha256": "c".repeat(64),
                        }))
                        .expect("soil state"),
                    )
                    .expect("owner")
                } else {
                    V11OwnerEnvelope::try_new(id.into(), format!("{id}-state").into_bytes())
                        .expect("owner")
                };
                (id.into(), envelope)
            })
            .collect()
    }

    fn accepted_receipts(
        owners: &BTreeMap<String, V11OwnerEnvelope>,
        ends: &[u128],
    ) -> (ParentTransactionId, Vec<AcceptedSlabReceiptV1>) {
        let mut clock_owners = owners
            .values()
            .map(V11OwnerEnvelope::to_owner_state)
            .collect::<Result<Vec<_>, _>>()
            .expect("owner states");
        clock_owners.sort_by(|a, b| a.owner_id().cmp(b.owner_id()));
        let participants = clock_owners
            .iter()
            .map(|owner| owner.owner_id().to_owned())
            .collect::<Vec<_>>();
        let support = TimeSupport::new(ModelTimeNs::new(0), ModelTimeNs::new(1_800_000_000_000))
            .expect("parent support");
        let interval =
            ParentIntervalId::derive(digest(1), digest(2), digest(3), support).expect("interval");
        let owner_digest = complete_owner_set_digest(&clock_owners).expect("owner digest");
        let parent =
            ParentTransactionId::derive(digest(1), 0, interval, owner_digest).expect("parent");
        let authority =
            ParentAuthorityV1::new(digest(1), digest(2), digest(3), 0, support, owner_digest)
                .expect("authority");
        let mut clock = CoupledClockStateV1::new(
            authority,
            clock_owners.clone(),
            "vegetation".into(),
            participants.clone(),
            digest(4),
            vec![],
        )
        .expect("clock");
        let mut participant_bytes = Vec::new();
        for participant in &participants {
            participant_bytes.extend_from_slice(participant.as_bytes());
            participant_bytes.push(0);
        }
        let segment = SegmentId::derive(
            parent,
            0,
            support,
            digest_bytes(b"vegetation"),
            digest_bytes(&participant_bytes),
        )
        .expect("segment");
        let mut receipts = Vec::new();
        for &end in ends {
            let constraint = StepConstraintV1::new(
                parent,
                clock.accepted_until(),
                ModelTimeNs::new(end),
                "vegetation".into(),
                ConstraintClass::HardBoundary,
                digest(5),
                digest(6),
                digest(7),
            )
            .expect("constraint");
            let reduction = reduce_constraints(
                &[constraint],
                parent,
                clock.accepted_until(),
                ModelTimeNs::new(1_800_000_000_000),
                None,
            )
            .expect("reduction");
            let candidate = CoupledSlabCandidateV1::new(
                &clock,
                segment,
                TimeSupport::new(clock.accepted_until(), ModelTimeNs::new(end)).expect("slab"),
                &reduction,
                clock_owners.clone(),
                vec![
                    LedgerEntryV1::new(
                        "vegetation_test".into(),
                        "kg".into(),
                        digest(8),
                        digest(8),
                        digest(9),
                    )
                    .expect("ledger"),
                ],
            )
            .expect("candidate");
            receipts.push(accept_slab(&mut clock, candidate).expect("accept"));
        }
        (parent, receipts)
    }

    fn staged_candidate(
        parent: &V11ParentTransaction,
        receipt: AcceptedSlabReceiptV1,
        debit: Option<V11ResourceDebit>,
    ) -> V11AcceptedSegmentCandidate {
        staged_candidate_with_debits(parent, receipt, debit.into_iter().collect())
    }

    fn staged_candidate_with_debits(
        parent: &V11ParentTransaction,
        receipt: AcceptedSlabReceiptV1,
        resource_debits: Vec<V11ResourceDebit>,
    ) -> V11AcceptedSegmentCandidate {
        let complete_owner_candidates =
            build_complete_owner_candidates(&receipt, &parent.staged_resource_owners, &[])
                .expect("candidates");
        V11AcceptedSegmentCandidate {
            lse_support_receipt: test_lse_support_receipt(&receipt),
            accepted_slab_receipt: receipt,
            beginning_state_sha256: parent.staged_state.state_sha256.clone(),
            ending_state: parent.staged_state.clone(),
            resource_debits,
            admitted_resource_fluxes: vec![],
            shared_resource_transitions: vec![],
            complete_owner_candidates,
            material_transfers: vec![],
            ending_resource_owners: parent.staged_resource_owners.clone(),
        }
    }

    fn test_lse_support_receipt(receipt: &AcceptedSlabReceiptV1) -> V11LseSupportReceiptEnvelope {
        let support = receipt.support();
        let mut value = LseSupportReceiptWire {
            parent_transaction_id: digest_hex(receipt.parent_transaction_id().digest()),
            segment_id: digest_hex(receipt.segment_id().digest()),
            accepted_slab_id: digest_hex(receipt.slab_id().digest()),
            slab_ordinal: receipt.slab_ordinal().to_string(),
            support_start_ns: support.start_ns().get().to_string(),
            support_end_ns: support.end_ns().get().to_string(),
            model_version: "OPENWEPP_SNOW_FREE_LSE_V1".into(),
            model_definition_sha256:
                "e1736b8c77d13d6fb12fb97a6f747e54eea877edf237817b6c6e8954cff8332f".into(),
            configuration_sha256: "a".repeat(64),
            beginning_state_sha256: "b".repeat(64),
            beginning_soil_thermal_state_sha256: "c".repeat(64),
            tolerance_policy_sha256: format!(
                "{:x}",
                Sha256::digest(b"energy_absolute=1e-6;energy_relative=1e-10")
            ),
            numerical_policy_sha256: format!(
                "{:x}",
                Sha256::digest(b"iterations=50;backtracking=0..20;strict-decrease")
            ),
            requested_support_ns: support.duration_ns().to_string(),
            duration_s_bits: format!("{:016x}", support.duration_s_bits()),
            minimum_support_ns: "60000000000".into(),
            receipt_sha256: String::new(),
        };
        let mut preimage = b"OPENWEPP_LSE_SUPPORT_ADMISSION_V1\0".to_vec();
        preimage.extend(serde_json::to_vec(&value).expect("blank receipt"));
        value.receipt_sha256 = format!("{:x}", Sha256::digest(preimage));
        V11LseSupportReceiptEnvelope::from_canonical_json(
            serde_json::to_vec(&value).expect("receipt json"),
        )
        .expect("receipt envelope")
    }

    fn reframe_test_lse_receipt(
        envelope: &V11LseSupportReceiptEnvelope,
        lse_state: Option<String>,
        soil_state: Option<String>,
    ) -> V11LseSupportReceiptEnvelope {
        let mut wire: LseSupportReceiptWire =
            serde_json::from_slice(&envelope.canonical_json).expect("receipt wire");
        if let Some(value) = lse_state {
            wire.beginning_state_sha256 = value;
        }
        if let Some(value) = soil_state {
            wire.beginning_soil_thermal_state_sha256 = value;
        }
        wire.receipt_sha256.clear();
        let mut preimage = b"OPENWEPP_LSE_SUPPORT_ADMISSION_V1\0".to_vec();
        preimage.extend(serde_json::to_vec(&wire).expect("blank wire"));
        wire.receipt_sha256 = format!("{:x}", Sha256::digest(preimage));
        V11LseSupportReceiptEnvelope::from_canonical_json(
            serde_json::to_vec(&wire).expect("sealed wire"),
        )
        .expect("sealed receipt")
    }

    fn native_v2_soil_owner_bytes(nonzero_carry: bool) -> (Vec<u8>, String) {
        let mut state = SoilThermalOwnedStateV2Wire {
            owner_id: "soil-thermal".to_owned(),
            configuration_sha256: "6".repeat(64),
            state_sha256: "0".repeat(64),
            last_accepted_transaction_id: Some(40),
            ofes: vec![SoilThermalOfeStateV2Wire {
                ofe_id: "ofe-1".to_owned(),
                ordered_layers: vec![SoilThermalLayerStateV2Wire {
                    layer_id: "soil-1".to_owned(),
                    temperature_k: 273.15,
                    enthalpy_hi_j_m2_ofe_ground: 1.0,
                    enthalpy_carry: if nonzero_carry {
                        ExactDyadicEnthalpyWire {
                            sign: 1,
                            coefficient_hex: "1".to_owned(),
                            exponent2: -100,
                        }
                    } else {
                        ExactDyadicEnthalpyWire {
                            sign: 0,
                            coefficient_hex: "0".to_owned(),
                            exponent2: 0,
                        }
                    },
                    last_accepted_transaction_id: Some(40),
                }],
            }],
        };
        let body = SoilThermalStateDigestBodyV2 {
            owner_tag: SOIL_THERMAL_OWNER_V2_TAG,
            schema_sha256: SOIL_THERMAL_OWNER_V2_SCHEMA_SHA256,
            exact_carry_definition_sha256: EXACT_DYADIC_ENTHALPY_V1_DEFINITION_SHA256,
            owner_id: &state.owner_id,
            configuration_sha256: &state.configuration_sha256,
            last_accepted_transaction_id: state.last_accepted_transaction_id,
            ofes: &state.ofes,
        };
        state.state_sha256 = format!(
            "{:x}",
            Sha256::digest(cpython_json_exponents(
                &serde_json::to_vec(&body).expect("V2 digest body")
            ))
        );
        let digest = state.state_sha256.clone();
        let resident = DirectV10SoilThermalResidentV2Wire {
            schema: DIRECT_V10_SOIL_THERMAL_RESIDENT_V2_SCHEMA.to_owned(),
            owner: SoilThermalOwnerEnvelopeV2Wire {
                owner_tag: SOIL_THERMAL_OWNER_V2_TAG.to_owned(),
                schema_sha256: SOIL_THERMAL_OWNER_V2_SCHEMA_SHA256.to_owned(),
                exact_carry_definition_sha256: EXACT_DYADIC_ENTHALPY_V1_DEFINITION_SHA256
                    .to_owned(),
                parent_v1_state_sha256: "7".repeat(64),
                contract_version: 15,
                model_version: "OPENWEPP_SOIL_THERMAL_V1".to_owned(),
                model_definition_sha256: "8".repeat(64),
                run_id: "native-v2-receipt".to_owned(),
                transaction_id: 41,
                expected_predecessor_transaction_id: Some(40),
                support_start_ns: 0,
                support_end_ns: 60_000_000_000,
                receipt_chain_sha256: "9".repeat(64),
                state,
            },
            latest_credit_receipt_sha256: None,
            expected_operand_set_sha256: None,
            orchestrator_seal_sha256: None,
            receipt_free_seal_sha256: Some("d".repeat(64)),
        };
        (
            serde_json::to_vec(&resident).expect("native V2 soil owner"),
            digest,
        )
    }

    #[test]
    fn lse_support_receipt_v1_owner_join_bytes_remain_golden() {
        let (configuration, state) = v10_fixture();
        let migrated = migrate_v10_runtime_to_v11(&configuration, &state).expect("migration");
        let owners = complete_owners(&migrated.state);
        let (_, receipts) = accepted_receipts(&owners, &[60_000_000_000]);
        let receipt = test_lse_support_receipt(&receipts[0]);
        let before = receipt.canonical_json.clone();

        receipt
            .validate_beginning_owners(&owners)
            .expect("frozen V1 owner join");
        assert_eq!(
            V11LseSupportReceiptEnvelope::from_canonical_json(before.clone())
                .expect("frozen receipt bytes")
                .canonical_json,
            before
        );
    }

    #[test]
    fn native_v2_lse_support_receipt_binds_state_carry_owner_and_support() {
        let (configuration, state) = v10_fixture();
        let migrated = migrate_v10_runtime_to_v11(&configuration, &state).expect("migration");
        let mut owners = complete_owners(&migrated.state);
        let (soil_bytes, soil_state_sha256) = native_v2_soil_owner_bytes(false);
        let (nonzero_bytes, nonzero_state_sha256) = native_v2_soil_owner_bytes(true);
        assert_eq!(
            validated_soil_beginning_state_sha256(&nonzero_bytes)
                .expect("nonzero exact-carry V2 owner"),
            nonzero_state_sha256
        );
        owners.insert(
            "soil_thermal".to_owned(),
            V11OwnerEnvelope::try_new("soil_thermal".to_owned(), soil_bytes.clone())
                .expect("native V2 owner envelope"),
        );
        let (_, receipts) = accepted_receipts(&owners, &[60_000_000_000, 120_000_000_000]);
        let receipt = reframe_test_lse_receipt(
            &test_lse_support_receipt(&receipts[0]),
            None,
            Some(soil_state_sha256.clone()),
        );
        receipt
            .validate_beginning_owners(&owners)
            .expect("native V2 owner join");
        assert!(
            receipt.validate_join(&receipts[1]).is_err(),
            "stale support"
        );

        let stale_state = test_lse_support_receipt(&receipts[0]);
        assert!(
            stale_state.validate_beginning_owners(&owners).is_err(),
            "stale soil state"
        );

        let poison = |mut value: serde_json::Value| {
            serde_json::to_vec(&value.take()).expect("poisoned V2 owner bytes")
        };
        let mut carry: serde_json::Value =
            serde_json::from_slice(&soil_bytes).expect("native V2 JSON");
        carry["owner"]["state"]["ofes"][0]["ordered_layers"][0]["enthalpy_carry"] =
            serde_json::json!({"sign": 1, "coefficient_hex": "1", "exponent2": -100});
        assert!(
            validated_soil_beginning_state_sha256(&poison(carry)).is_err(),
            "same state digest cannot admit a carry mutation"
        );

        let mut owner: serde_json::Value =
            serde_json::from_slice(&soil_bytes).expect("native V2 JSON");
        owner["owner"]["state"]["owner_id"] = serde_json::json!("substituted-soil-owner");
        assert!(validated_soil_beginning_state_sha256(&poison(owner)).is_err());

        let mut tag: serde_json::Value =
            serde_json::from_slice(&soil_bytes).expect("native V2 JSON");
        tag["owner"]["owner_tag"] = serde_json::json!("OPENWEPP_SOIL_THERMAL_OWNER_V1");
        assert!(validated_soil_beginning_state_sha256(&poison(tag)).is_err());

        let mut schema: serde_json::Value =
            serde_json::from_slice(&soil_bytes).expect("native V2 JSON");
        schema["schema"] = serde_json::json!("OPENWEPP_DIRECT_V10_SOIL_THERMAL_RESIDENT_V3");
        assert!(validated_soil_beginning_state_sha256(&poison(schema)).is_err());

        let mut mixed: serde_json::Value =
            serde_json::from_slice(&soil_bytes).expect("native V2 JSON");
        mixed["state_sha256"] = serde_json::json!(soil_state_sha256);
        assert!(validated_soil_beginning_state_sha256(&poison(mixed)).is_err());
    }

    #[test]
    fn embedded_v11_definition_is_identity_distinct() {
        assert_ne!(v11_model_sha256(), V10_MODEL_SHA256);
        assert_eq!(
            load_v11_model_definition().expect("model").sha256,
            V11_MODEL_SHA256
        );
        let value: serde_json::Value = serde_json::from_slice(V11_MODEL_BYTES).expect("definition");
        assert_eq!(value["model_version"], V11_MODEL_VERSION);
    }

    #[test]
    fn migration_preserves_every_physical_payload_bit() {
        let (configuration, state) = v10_fixture();
        let migrated = migrate_v10_runtime_to_v11(&configuration, &state).expect("migration");
        assert_eq!(migrated.configuration.nominal_cadence_ns, 1_800_000_000_000);
        assert_eq!(migrated.state.physical.strata, state.0.strata);
        assert_eq!(migrated.state.physical.occupancies, state.0.occupancies);
        assert_eq!(
            migrated.state.last_parent_transaction_id,
            state.0.last_transaction_id
        );
        migrated
            .state
            .validate(&migrated.configuration)
            .expect("valid V11 state");
    }

    #[test]
    fn complete_owner_manifest_is_exact_and_digest_bound() {
        let owners = V11_COMPLETE_OWNER_MANIFEST
            .into_iter()
            .map(|id| {
                (
                    id.to_owned(),
                    V11OwnerEnvelope::try_new(id.to_owned(), id.as_bytes().to_vec())
                        .expect("owner"),
                )
            })
            .collect::<BTreeMap<_, _>>();
        validate_complete_owners(&owners).expect("complete owners");
        let mut poison = owners;
        poison.get_mut("hydrology").expect("hydrology").state_bytes[0] ^= 1;
        assert!(matches!(
            validate_complete_owners(&poison),
            Err(V11Error::ResourceOwnerCandidate)
        ));
    }

    #[test]
    fn segment_local_lineage_is_normalized_to_parent_chronology() {
        let (_, mut physical) = crate::v8_state::v8_test_fixture();
        physical.last_transaction_id = 9;
        for stratum in physical.strata.values_mut() {
            stratum.last_transaction_id = 9;
        }
        for occupancy in physical.occupancies.values_mut() {
            occupancy.last_accepted_transaction_id = Some(9);
        }
        normalize_parent_transaction_lineage(&mut physical, 8);
        assert_eq!(physical.last_transaction_id, 8);
        assert!(
            physical
                .strata
                .values()
                .all(|value| value.last_transaction_id == 8)
        );
        assert!(
            physical
                .occupancies
                .values()
                .all(|value| value.last_accepted_transaction_id == Some(8))
        );
    }

    #[test]
    fn unequal_segments_chain_and_finalize_one_complete_owner_successor() {
        let (v10_configuration, v10_state) = v10_fixture();
        let migrated = migrate_v10_runtime_to_v11(&v10_configuration, &v10_state).expect("migrate");
        let owners = complete_owners(&migrated.state);
        let (parent_id, receipts) =
            accepted_receipts(&owners, &[600_000_000_000, 1_800_000_000_000]);
        assert_eq!(receipts[0].duration_s_bits(), 600.0_f64.to_bits());
        assert_eq!(receipts[1].duration_s_bits(), 1_200.0_f64.to_bits());
        let mut parent = V11ParentTransaction::new_with_complete_owners(
            &migrated.configuration,
            &migrated.state,
            parent_id,
            ModelTimeNs::new(0),
            owners,
        )
        .expect("parent");
        for receipt in receipts {
            let candidate = staged_candidate(&parent, receipt, None);
            parent
                .accept_segment(&migrated.configuration, candidate)
                .expect("stage");
        }
        let beginning_transaction = migrated.state.last_parent_transaction_id;
        let candidate = parent.finalize(&migrated.configuration).expect("finalize");
        assert_eq!(
            candidate.ending_state.last_parent_transaction_id,
            beginning_transaction + 1
        );
        assert_eq!(
            candidate.ending_complete_owners[0].state_bytes(),
            serde_json::to_vec(&candidate.ending_state)
                .expect("ending state")
                .as_slice()
        );
        assert_eq!(
            candidate
                .ending_complete_owners
                .iter()
                .map(OwnerState::owner_id)
                .collect::<Vec<_>>(),
            V11_COMPLETE_OWNER_MANIFEST
        );
    }

    fn zero_duration_owner_transition_fixture() -> (
        V10ToV11Migration,
        BTreeMap<String, V11OwnerEnvelope>,
        ParentTransactionId,
        BTreeMap<String, V11OwnerEnvelope>,
    ) {
        let (v10_configuration, v10_state) = v10_fixture();
        let migrated = migrate_v10_runtime_to_v11(&v10_configuration, &v10_state).expect("migrate");
        let beginning_owners = complete_owners(&migrated.state);
        let (parent_id, _) = accepted_receipts(&beginning_owners, &[1_800_000_000_000]);
        let mut ending_owners = beginning_owners.clone();
        let mut ending_snow_bytes = ending_owners.get("snow").expect("snow").state_bytes.clone();
        ending_snow_bytes.push(1);
        ending_owners.insert(
            "snow".to_owned(),
            V11OwnerEnvelope::try_new("snow".to_owned(), ending_snow_bytes).expect("ending snow"),
        );
        (migrated, beginning_owners, parent_id, ending_owners)
    }

    fn zero_duration_owner_transition_parent(
        migrated: &V10ToV11Migration,
        beginning_owners: &BTreeMap<String, V11OwnerEnvelope>,
        parent_id: ParentTransactionId,
    ) -> V11ParentTransaction {
        V11ParentTransaction::new_with_complete_owners(
            &migrated.configuration,
            &migrated.state,
            parent_id,
            ModelTimeNs::new(0),
            beginning_owners.clone(),
        )
        .expect("parent")
    }

    #[test]
    fn zero_duration_owner_transition_requires_exact_mutation_set() {
        let (migrated, beginning_owners, parent_id, ending_owners) =
            zero_duration_owner_transition_fixture();
        let make_parent =
            || zero_duration_owner_transition_parent(&migrated, &beginning_owners, parent_id);

        let mut extra_member = make_parent();
        let before = extra_member.checkpoint();
        assert!(matches!(
            extra_member.accept_zero_duration_owner_transition(
                &migrated.configuration,
                ModelTimeNs::new(0),
                ending_owners.clone(),
                &["nonexistent".to_owned(), "snow".to_owned()],
            ),
            Err(V11Error::ResourceOwnerCandidate)
        ));
        assert_eq!(extra_member.checkpoint(), before);

        let mut omitted_member = make_parent();
        assert!(matches!(
            omitted_member.accept_zero_duration_owner_transition(
                &migrated.configuration,
                ModelTimeNs::new(0),
                ending_owners.clone(),
                &["soil_thermal".to_owned()],
            ),
            Err(V11Error::ResourceOwnerCandidate)
        ));

        let mut exact = make_parent();
        exact
            .accept_zero_duration_owner_transition(
                &migrated.configuration,
                ModelTimeNs::new(0),
                ending_owners.clone(),
                &["snow".to_owned()],
            )
            .expect("exact snow mutation");
        assert_eq!(exact.staged_resource_owners(), &ending_owners);

        let mut ordinary_empty = make_parent();
        assert!(
            ordinary_empty
                .accept_zero_duration_owner_transition(
                    &migrated.configuration,
                    ModelTimeNs::new(0),
                    beginning_owners.clone(),
                    &[],
                )
                .is_err()
        );
        let mut missing_receipt = make_parent();
        assert!(
            missing_receipt
                .accept_zero_duration_custody_noop(
                    &migrated.configuration,
                    ModelTimeNs::new(0),
                    beginning_owners.clone(),
                    Digest32::zero(),
                )
                .is_err()
        );
        let mut false_noop = make_parent();
        assert!(
            false_noop
                .accept_zero_duration_custody_noop(
                    &migrated.configuration,
                    ModelTimeNs::new(0),
                    ending_owners,
                    Digest32::from_bytes([0x51; 32]),
                )
                .is_err()
        );
        let mut receipt_noop = make_parent();
        receipt_noop
            .accept_zero_duration_custody_noop(
                &migrated.configuration,
                ModelTimeNs::new(0),
                beginning_owners,
                Digest32::from_bytes([0x52; 32]),
            )
            .expect("receipt-bearing exact no-op");
        let checkpoint = receipt_noop.checkpoint();
        assert_eq!(
            checkpoint.accepted_zero_duration_owner_transitions[0].custody_receipt_sha256,
            Some(Digest32::from_bytes([0x52; 32])),
        );
        let restored = V11ParentTransaction::restore(&migrated.configuration, checkpoint.clone())
            .expect("restore receipt-bearing no-op");
        assert_eq!(restored.checkpoint(), checkpoint);
        let mut omitted = checkpoint.clone();
        omitted.accepted_zero_duration_owner_transitions[0].custody_receipt_sha256 = None;
        assert!(V11ParentTransaction::restore(&migrated.configuration, omitted).is_err());
        let mut substituted = checkpoint;
        substituted.accepted_zero_duration_owner_transitions[0].custody_receipt_sha256 =
            Some(Digest32::zero());
        assert!(V11ParentTransaction::restore(&migrated.configuration, substituted).is_err());
    }

    #[test]
    fn checkpoint_restores_before_first_segment_and_rejects_state_substitution() {
        let (v10_configuration, v10_state) = v10_fixture();
        let migrated = migrate_v10_runtime_to_v11(&v10_configuration, &v10_state).expect("migrate");
        let owners = complete_owners(&migrated.state);
        let (parent_id, _) = accepted_receipts(&owners, &[1_800_000_000_000]);
        let parent = V11ParentTransaction::new_with_complete_owners(
            &migrated.configuration,
            &migrated.state,
            parent_id,
            ModelTimeNs::new(0),
            owners,
        )
        .expect("parent");
        let checkpoint = parent.checkpoint();
        let restored = V11ParentTransaction::restore(&migrated.configuration, checkpoint.clone())
            .expect("restore initial parent");
        assert_eq!(restored.checkpoint(), checkpoint);

        let mut state_substitution = checkpoint;
        state_substitution.staged_state.last_parent_transaction_id += 1;
        assert!(
            V11ParentTransaction::restore(&migrated.configuration, state_substitution).is_err()
        );
    }

    #[test]
    fn parent_candidate_checkpoint_authority_ignores_only_transient_segment_cache() {
        let (v10_configuration, v10_state) = v10_fixture();
        let migrated = migrate_v10_runtime_to_v11(&v10_configuration, &v10_state)
            .expect("migrate V11 fixture");
        let owners = complete_owners(&migrated.state);
        let (parent_id, receipts) = accepted_receipts(&owners, &[1_800_000_000_000]);
        let mut live = V11ParentTransaction::new_with_complete_owners(
            &migrated.configuration,
            &migrated.state,
            parent_id,
            ModelTimeNs::new(0),
            owners,
        )
        .expect("live parent");
        let segment = staged_candidate(&live, receipts[0].clone(), None);
        live.accept_segment(&migrated.configuration, segment)
            .expect("accepted segment");
        let checkpoint = live.checkpoint();
        let restored = V11ParentTransaction::restore(&migrated.configuration, checkpoint.clone())
            .expect("restored parent");
        assert_eq!(restored.checkpoint(), checkpoint);

        let live_candidate = live
            .finalize(&migrated.configuration)
            .expect("live candidate");
        let restored_candidate = restored
            .finalize(&migrated.configuration)
            .expect("restored candidate");
        assert_eq!(live_candidate.accepted_segments.len(), 1);
        assert!(restored_candidate.accepted_segments.is_empty());
        assert!(live_candidate.has_same_checkpoint_authority(&restored_candidate));

        let mut checkpoint_poison = restored_candidate.clone();
        checkpoint_poison.accepted_segment_checkpoints[0]
            .beginning_state_sha256
            .replace_range(..1, "f");
        assert!(
            !live_candidate.has_same_checkpoint_authority(&checkpoint_poison),
            "changed accepted checkpoint must reject",
        );
        let mut ending_owner_poison = restored_candidate;
        let ending_owner = &ending_owner_poison.ending_complete_owners[0];
        let mut ending_owner_bytes = ending_owner.state_bytes().to_vec();
        ending_owner_bytes.push(0);
        ending_owner_poison.ending_complete_owners[0] =
            OwnerState::new(ending_owner.owner_id().to_owned(), ending_owner_bytes)
                .expect("poison owner");
        assert!(
            !live_candidate.has_same_checkpoint_authority(&ending_owner_poison),
            "changed ending owner must reject",
        );
    }

    #[test]
    fn checkpoint_restores_ordered_same_tick_owner_transitions_and_rejects_poisons() {
        let (v10_configuration, v10_state) = v10_fixture();
        let migrated = migrate_v10_runtime_to_v11(&v10_configuration, &v10_state).expect("migrate");
        let owners = complete_owners(&migrated.state);
        let (parent_id, receipts) = accepted_receipts(&owners, &[1_800_000_000_000]);
        let mut parent = V11ParentTransaction::new_with_complete_owners(
            &migrated.configuration,
            &migrated.state,
            parent_id,
            ModelTimeNs::new(0),
            owners,
        )
        .expect("parent");
        let candidate = staged_candidate(&parent, receipts[0].clone(), None);
        parent
            .accept_segment(&migrated.configuration, candidate)
            .expect("stage");

        let mutate_owner =
            |owners: &BTreeMap<String, V11OwnerEnvelope>, owner_id: &str, marker: u8| {
                let mut ending = owners.clone();
                let mut bytes = ending
                    .get(owner_id)
                    .expect("mutated owner")
                    .state_bytes
                    .clone();
                bytes.push(marker);
                ending.insert(
                    owner_id.to_owned(),
                    V11OwnerEnvelope::try_new(owner_id.to_owned(), bytes).expect("ending owner"),
                );
                ending
            };
        let snow_ending = mutate_owner(parent.staged_resource_owners(), "snow", 1);
        parent
            .accept_zero_duration_owner_transition(
                &migrated.configuration,
                ModelTimeNs::new(1_800_000_000_000),
                snow_ending,
                &["snow".to_owned()],
            )
            .expect("snow transition");
        let lse_ending = mutate_owner(parent.staged_resource_owners(), "land_surface_energy", 2);
        parent
            .accept_zero_duration_owner_transition(
                &migrated.configuration,
                ModelTimeNs::new(1_800_000_000_000),
                lse_ending,
                &["land_surface_energy".to_owned()],
            )
            .expect("LSE transition");

        let checkpoint = parent.checkpoint();
        let restored = V11ParentTransaction::restore(&migrated.configuration, checkpoint.clone())
            .expect("restore transitions");
        assert_eq!(restored.checkpoint(), checkpoint);

        let mut omission = checkpoint.clone();
        omission.accepted_zero_duration_owner_transitions.remove(0);
        assert!(V11ParentTransaction::restore(&migrated.configuration, omission).is_err());

        let mut substitution = checkpoint.clone();
        substitution.accepted_zero_duration_owner_transitions[0].accepted_segment_count = 0;
        assert!(V11ParentTransaction::restore(&migrated.configuration, substitution).is_err());

        let mut order = checkpoint.clone();
        order.accepted_zero_duration_owner_transitions.reverse();
        assert!(V11ParentTransaction::restore(&migrated.configuration, order).is_err());

        let mut same_tick_duplicate = checkpoint;
        let duplicate = same_tick_duplicate.accepted_zero_duration_owner_transitions[0].clone();
        same_tick_duplicate
            .accepted_zero_duration_owner_transitions
            .insert(1, duplicate);
        assert!(
            V11ParentTransaction::restore(&migrated.configuration, same_tick_duplicate).is_err()
        );
    }

    #[test]
    fn checkpoint_restore_rejects_broken_predecessor_and_terminal_owner() {
        let (v10_configuration, v10_state) = v10_fixture();
        let migrated = migrate_v10_runtime_to_v11(&v10_configuration, &v10_state).expect("migrate");
        let owners = complete_owners(&migrated.state);
        let (parent_id, receipts) = accepted_receipts(&owners, &[1_800_000_000_000]);
        let mut parent = V11ParentTransaction::new_with_complete_owners(
            &migrated.configuration,
            &migrated.state,
            parent_id,
            ModelTimeNs::new(0),
            owners,
        )
        .expect("parent");
        let candidate = staged_candidate(&parent, receipts[0].clone(), None);
        parent
            .accept_segment(&migrated.configuration, candidate)
            .expect("stage");
        let checkpoint = parent.checkpoint();
        let restored = V11ParentTransaction::restore(&migrated.configuration, checkpoint.clone())
            .expect("restore");
        assert_eq!(restored.checkpoint(), checkpoint);

        let mut predecessor_poison = checkpoint.clone();
        predecessor_poison.accepted_segments[0].beginning_state_sha256 = "0".repeat(64);
        assert!(
            V11ParentTransaction::restore(&migrated.configuration, predecessor_poison).is_err()
        );

        let mut receipt_poison = checkpoint.clone();
        receipt_poison.accepted_segments[0]
            .lse_support_receipt
            .canonical_json[0] ^= 1;
        assert!(V11ParentTransaction::restore(&migrated.configuration, receipt_poison).is_err());

        let mut lse_join_poison = checkpoint.clone();
        lse_join_poison.accepted_segments[0].lse_support_receipt = reframe_test_lse_receipt(
            &lse_join_poison.accepted_segments[0].lse_support_receipt,
            Some("9".repeat(64)),
            None,
        );
        assert!(V11ParentTransaction::restore(&migrated.configuration, lse_join_poison).is_err());

        let mut soil_join_poison = checkpoint.clone();
        soil_join_poison.accepted_segments[0].lse_support_receipt = reframe_test_lse_receipt(
            &soil_join_poison.accepted_segments[0].lse_support_receipt,
            None,
            Some("8".repeat(64)),
        );
        assert!(V11ParentTransaction::restore(&migrated.configuration, soil_join_poison).is_err());

        let mut owner_poison = checkpoint;
        owner_poison.accepted_segments[0]
            .ending_resource_owners
            .get_mut("snow")
            .expect("snow")
            .state_bytes
            .push(0);
        assert!(V11ParentTransaction::restore(&migrated.configuration, owner_poison).is_err());
    }

    #[test]
    fn lse_support_receipt_replay_rejects_without_parent_mutation() {
        let (v10_configuration, v10_state) = v10_fixture();
        let migrated = migrate_v10_runtime_to_v11(&v10_configuration, &v10_state).expect("migrate");
        let owners = complete_owners(&migrated.state);
        let (parent_id, receipts) =
            accepted_receipts(&owners, &[600_000_000_000, 1_800_000_000_000]);
        let mut parent = V11ParentTransaction::new_with_complete_owners(
            &migrated.configuration,
            &migrated.state,
            parent_id,
            ModelTimeNs::new(0),
            owners,
        )
        .expect("parent");
        let first = staged_candidate(&parent, receipts[0].clone(), None);
        let replay = first.lse_support_receipt.clone();
        parent
            .accept_segment(&migrated.configuration, first)
            .expect("first segment");
        let before = parent.checkpoint();
        let mut second = staged_candidate(&parent, receipts[1].clone(), None);
        second.lse_support_receipt = replay;
        assert!(matches!(
            parent.accept_segment(&migrated.configuration, second),
            Err(V11Error::LseSupportReceipt | V11Error::SupportPredecessor)
        ));
        assert_eq!(parent.checkpoint(), before);
    }

    #[cfg(any())]
    #[test]
    #[allow(clippy::too_many_lines)]
    fn nonassociative_resource_custody_uses_sequential_endings_for_water_nh4_no3() {
        use openwepp_kernel_contract::{
            MineralNitrogenSpecies, OccupancyId, SoilLayerId, StratumId, TileId,
        };

        let (v10_configuration, v10_state) = v10_fixture();
        let migrated = migrate_v10_runtime_to_v11(&v10_configuration, &v10_state).expect("migrate");
        let owners = complete_owners(&migrated.state);
        let (parent_id, receipts) = accepted_receipts(
            &owners,
            &[600_000_000_000, 1_200_000_000_000, 1_800_000_000_000],
        );
        let mut parent = V11ParentTransaction::new_with_complete_owners(
            &migrated.configuration,
            &migrated.state,
            parent_id,
            ModelTimeNs::new(0),
            owners,
        )
        .expect("parent");
        let layer = migrated.configuration.imported_v10.strata[0].root_layers[0]
            .layer_id
            .clone();
        let keys = [
            V11ResourceKey::Water(WaterResourceKey {
                occupancy_id: OccupancyId {
                    stratum_id: StratumId::try_new("s1").expect("stratum"),
                    tile_id: TileId::try_new("t1").expect("tile"),
                },
                layer_id: layer.clone(),
            }),
            V11ResourceKey::MineralNitrogen(MineralNitrogenKey {
                layer_id: layer.clone(),
                species: MineralNitrogenSpecies::Ammonium,
            }),
            V11ResourceKey::MineralNitrogen(MineralNitrogenKey {
                layer_id: layer,
                species: MineralNitrogenSpecies::Nitrate,
            }),
        ];
        let beginnings = [
            497_355_953.965_941_8,
            0.497_355_953_965_941_84,
            497.355_953_965_941_75,
        ];
        let amounts = [
            [
                108_987_197.969_511_36,
                119_731_815.493_540_45,
                27_340_159.710_375_622,
            ],
            [
                0.108_987_197_969_511_37,
                0.119_731_815_493_540_46,
                0.027_340_159_710_375_622,
            ],
            [
                108.987_197_969_511_36,
                119.731_815_493_540_45,
                27.340_159_710_375_62,
            ],
        ];
        let mut staged = beginnings;
        for (ordinal, receipt) in receipts.into_iter().enumerate() {
            let debits: Vec<V11ResourceDebit> = keys
                .iter()
                .enumerate()
                .map(|(resource, key)| {
                    let beginning = staged[resource];
                    let amount = amounts[resource][ordinal];
                    let ending = beginning - amount;
                    staged[resource] = ending;
                    V11ResourceDebit {
                        owner_id: if resource == 0 { "hydrology" } else { "bgc" }.into(),
                        resource_key: key.clone(),
                        beginning_amount: beginning,
                        amount,
                        ending_amount: ending,
                    }
                })
                .collect();
            if ordinal == 2 {
                let mut regrouped = debits.clone();
                let cumulative = amounts[0]
                    .iter()
                    .fold(0.0_f64, |total, amount| total + amount);
                regrouped[0].ending_amount = beginnings[0] - cumulative;
                assert_ne!(
                    regrouped[0].ending_amount.to_bits(),
                    debits[0].ending_amount.to_bits()
                );
                let before = parent.checkpoint();
                let poison = staged_candidate_with_debits(&parent, receipt.clone(), regrouped);
                assert!(matches!(
                    parent.accept_segment(&migrated.configuration, poison),
                    Err(V11Error::ResourceDebit)
                ));
                assert_eq!(parent.checkpoint(), before);
            }
            let candidate = staged_candidate_with_debits(&parent, receipt, debits);
            parent
                .accept_segment(&migrated.configuration, candidate)
                .expect("sequential custody");
        }
        assert_eq!(staged[0].to_bits(), 241_296_780.792_514_32_f64.to_bits());
        assert_eq!(staged[1].to_bits(), 0.241_296_780_792_514_43_f64.to_bits());
        assert_eq!(staged[2].to_bits(), 241.296_780_792_514_34_f64.to_bits());
        for (resource, key) in keys.iter().enumerate() {
            let owner = if resource == 0 { "hydrology" } else { "bgc" };
            let cumulative = parent
                .cumulative_debits
                .get(&(owner.into(), key.clone()))
                .expect("cumulative diagnostic");
            assert_ne!(
                (beginnings[resource] - cumulative).to_bits(),
                staged[resource].to_bits()
            );
        }
        let checkpoint = parent.checkpoint();
        V11ParentTransaction::restore(&migrated.configuration, checkpoint)
            .expect("nonassociative checkpoint");
    }

    #[cfg(any())]
    #[test]
    fn wrong_segment_ending_rejects_atomically() {
        let (v10_configuration, v10_state) = v10_fixture();
        let migrated = migrate_v10_runtime_to_v11(&v10_configuration, &v10_state).expect("migrate");
        let owners = complete_owners(&migrated.state);
        let (parent_id, receipts) = accepted_receipts(&owners, &[1_800_000_000_000]);
        let mut parent = V11ParentTransaction::new_with_complete_owners(
            &migrated.configuration,
            &migrated.state,
            parent_id,
            ModelTimeNs::new(0),
            owners,
        )
        .expect("parent");
        let before = parent.checkpoint();
        let poison = V11ResourceDebit {
            owner_id: "hydrology".into(),
            resource_key: V11ResourceKey::Water(WaterResourceKey {
                occupancy_id: openwepp_kernel_contract::OccupancyId {
                    stratum_id: openwepp_kernel_contract::StratumId::try_new("s1")
                        .expect("stratum"),
                    tile_id: openwepp_kernel_contract::TileId::try_new("t1").expect("tile"),
                },
                layer_id: openwepp_kernel_contract::SoilLayerId::try_new("l1").expect("layer"),
            }),
            beginning_amount: 497_355_953.965_941_8,
            amount: 256_059_173.173_427_4,
            ending_amount: 241_296_780.792_514_32,
        };
        let candidate = staged_candidate(&parent, receipts[0].clone(), Some(poison));
        assert!(matches!(
            parent.accept_segment(&migrated.configuration, candidate),
            Err(V11Error::ResourceDebit)
        ));
        assert_eq!(parent.checkpoint(), before);
    }
    #[test]
    fn validated_v10_segment_ending_binds_state_config_transaction_and_support() {
        let (configuration, beginning) = v10_fixture();
        let support = TimeSupport::new(ModelTimeNs::new(0), ModelTimeNs::new(60_000_000_000))
            .expect("support");
        let mut ending = beginning.clone();
        normalize_parent_transaction_lineage(&mut ending.0, beginning.0.last_transaction_id + 1);
        ending.0.state_sha256 = ending.0.canonical_sha256();
        let validated = ValidatedV10SegmentEndingV1::validate_untrusted_executor_return(
            ending.clone(),
            &configuration,
            &beginning,
            support,
        )
        .expect("validated ending");
        assert_eq!(
            validated
                .clone()
                .into_ending(&configuration, &beginning, support)
                .expect("same revision"),
            ending,
        );

        let mut foreign_configuration = configuration.clone();
        foreign_configuration
            .configuration_sha256
            .replace_range(..1, "f");
        assert!(
            validated
                .clone()
                .into_ending(&foreign_configuration, &beginning, support)
                .is_err()
        );
        let mut foreign_beginning = beginning.clone();
        foreign_beginning.0.state_sha256.replace_range(..1, "f");
        assert!(
            validated
                .clone()
                .into_ending(&configuration, &foreign_beginning, support)
                .is_err()
        );
        let shifted_support =
            TimeSupport::new(ModelTimeNs::new(1), ModelTimeNs::new(60_000_000_001))
                .expect("shifted support");
        assert!(
            validated
                .into_ending(&configuration, &beginning, shifted_support)
                .is_err()
        );
    }

    #[test]
    fn validated_parent_handoff_reuses_once_then_lineage_mutation_revalidates() {
        let (v10_configuration, v10_state) = v10_fixture();
        let migrated = migrate_v10_runtime_to_v11(&v10_configuration, &v10_state)
            .expect("migrate V11 fixture");
        let owners = complete_owners(&migrated.state);
        let (parent_id, receipts) = accepted_receipts(&owners, &[1_800_000_000_000]);
        let mut parent = V11ParentTransaction::new_with_complete_owners(
            &migrated.configuration,
            &migrated.state,
            parent_id,
            ModelTimeNs::new(0),
            owners,
        )
        .expect("parent");
        let segment = staged_candidate(&parent, receipts[0].clone(), None);
        parent
            .accept_segment(&migrated.configuration, segment)
            .expect("accepted segment");
        begin_v11_validated_handoff_audit_v1();
        let finalized = parent
            .finalize(&migrated.configuration)
            .expect("finalized parent");
        let audit = take_v11_validated_handoff_audit_v1();
        assert_eq!(audit.trusted_parent_handoff_reuses, 1);
        assert_eq!(audit.lineage_mutation_full_validations, 1);
        assert_eq!(audit.untrusted_executor_full_validations, 0);
        let vegetation = finalized
            .ending_complete_owners
            .iter()
            .find(|owner| owner.owner_id() == "vegetation")
            .expect("vegetation owner");
        assert_eq!(
            vegetation.state_bytes(),
            serde_json::to_vec(&finalized.ending_state)
                .expect("canonical accepted state")
                .as_slice(),
        );
    }

    #[test]
    fn parent_finalization_projection_handoff_binds_exact_config_beginning_and_ending() {
        let make = || {
            let (v10_configuration, v10_state) = v10_fixture();
            let migrated = migrate_v10_runtime_to_v11(&v10_configuration, &v10_state)
                .expect("migrate V11 fixture");
            let owners = complete_owners(&migrated.state);
            let (parent_id, receipts) = accepted_receipts(&owners, &[1_800_000_000_000]);
            let mut parent = V11ParentTransaction::new_with_complete_owners(
                &migrated.configuration,
                &migrated.state,
                parent_id,
                ModelTimeNs::new(0),
                owners,
            )
            .expect("parent");
            let segment = staged_candidate(&parent, receipts[0].clone(), None);
            parent
                .accept_segment(&migrated.configuration, segment)
                .expect("accepted segment");
            let (candidate, handoff) = parent
                .finalize_with_validated_handoff(&migrated.configuration)
                .expect("validated finalization");
            (migrated.configuration, candidate, handoff)
        };

        let (configuration, candidate, handoff) = make();
        let beginning = handoff.beginning_v10.clone();
        let expected_v10 = project_v11_parent_finalization_to_v10(
            &configuration,
            &beginning,
            &candidate.ending_state,
        )
        .expect("independent projection");
        let expected_v9 = crate::project_v10_runtime_to_v9(&expected_v10.0, &expected_v10.1)
            .expect("independent V9 projection");
        let projected = handoff
            .project_to_v10(&configuration, &beginning, &candidate.ending_state)
            .expect("trusted exact projection");
        assert_eq!((&projected.0, &projected.1), (&expected_v10.0, &expected_v10.1));
        assert_eq!((&projected.2, &projected.3), (&expected_v9.0, &expected_v9.1));

        let (configuration, candidate, handoff) = make();
        let beginning = handoff.beginning_v10.clone();
        let mut foreign_configuration = configuration.clone();
        foreign_configuration.configuration_sha256.push('0');
        assert!(
            handoff
                .project_to_v10(
                    &foreign_configuration,
                    &beginning,
                    &candidate.ending_state,
                )
                .is_err()
        );

        let (configuration, candidate, handoff) = make();
        let mut foreign_beginning = handoff.beginning_v10.clone();
        foreign_beginning.0.last_transaction_id += 1;
        assert!(
            handoff
                .project_to_v10(&configuration, &foreign_beginning, &candidate.ending_state)
                .is_err()
        );

        let (configuration, mut candidate, handoff) = make();
        let beginning = handoff.beginning_v10.clone();
        candidate.ending_state.state_sha256.push('0');
        assert!(
            handoff
                .project_to_v10(&configuration, &beginning, &candidate.ending_state)
                .is_err()
        );
    }

    #[test]
    fn restart_revalidates_and_does_not_serialize_validated_handoff() {
        let (v10_configuration, v10_state) = v10_fixture();
        let migrated = migrate_v10_runtime_to_v11(&v10_configuration, &v10_state)
            .expect("migrate V11 fixture");
        let owners = complete_owners(&migrated.state);
        let (parent_id, _) = accepted_receipts(&owners, &[1_800_000_000_000]);
        let parent = V11ParentTransaction::new_with_complete_owners(
            &migrated.configuration,
            &migrated.state,
            parent_id,
            ModelTimeNs::new(0),
            owners,
        )
        .expect("parent");
        let bytes = serde_json::to_vec(&parent.checkpoint()).expect("checkpoint bytes");
        let checkpoint: V11ParentTransactionCheckpoint =
            serde_json::from_slice(&bytes).expect("checkpoint parse");
        assert_eq!(serde_json::to_vec(&checkpoint).expect("reserialize"), bytes);
        begin_v11_validated_handoff_audit_v1();
        let restored = V11ParentTransaction::restore(&migrated.configuration, checkpoint)
            .expect("restored checkpoint");
        assert_eq!(
            take_v11_validated_handoff_audit_v1().restart_full_validations,
            2
        );
        assert_eq!(restored.checkpoint(), parent.checkpoint());
        assert!(!String::from_utf8_lossy(&bytes).contains("ValidatedV10SegmentEndingV1"));
    }

    mod bgc_tests {
        include!("v11/tests/v11_bgc_tests.rs");
    }
    mod custody_tests {
        include!("v11/tests/v11_custody_tests.rs");
    }
}
