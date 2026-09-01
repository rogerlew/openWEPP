mod soil_thermal_v2_cutover_tests {
    use openwepp_kernel_contract::{ResourceOwnerId, SoilLayerId, TransactionId};

    use super::*;
    use crate::{
        OfeId, SoilThermalLayerSnapshot, SoilThermalLayerStateV2, SoilThermalOfeSnapshot,
        SoilThermalUnpublishedCoordinateV2, SoilThermalV2MigrationIdentity,
        advance_soil_thermal_composed_trial_v2,
        advance_soil_thermal_sequential_unpublished_trial_v2,
        compose_soil_thermal_accepted_from_unpublished_v2, migrate_soil_thermal_v1_to_v2,
        prepare_soil_thermal_support_v2, project_soil_thermal_unpublished_coordinates_v2,
        project_soil_thermal_unpublished_top_layer_coordinates_v2,
        seal_soil_thermal_receipt_free_owner_v2, validate_soil_thermal_receipt_free_owner_v2,
    };

    fn digest(fill: char) -> Sha256Digest {
        Sha256Digest::try_new(fill.to_string().repeat(64)).expect("digest")
    }

    fn beginning() -> SoilThermalOwnerEnvelopeV2 {
        migrate_soil_thermal_v1_to_v2(
            &SoilThermalSnapshot {
                owner_id: ResourceOwnerId::try_new("soil-owner").expect("owner"),
                configuration_sha256: digest('1'),
                state_sha256: digest('2'),
                snapshot_sha256: digest('3'),
                last_accepted_transaction_id: None,
                ofes: vec![SoilThermalOfeSnapshot {
                    ofe_id: OfeId::try_new("ofe-1").expect("OFE"),
                    ordered_layers: vec![SoilThermalLayerSnapshot {
                        layer_id: SoilLayerId::try_new("layer-1").expect("layer"),
                        temperature_k: 273.15,
                        enthalpy_j_m2_ofe_ground: 0.0,
                    }],
                }],
            },
            SoilThermalV2MigrationIdentity {
                model_version: "OPENWEPP_SOIL_THERMAL_EXACT_CARRY_V2".to_owned(),
                model_definition_sha256: digest('4'),
                run_id: "cutover-test".to_owned(),
                transaction_id: TransactionId(7),
                support_start_ns: 0,
                support_end_ns: 60_000_000_000,
                receipt_chain_sha256: digest('5'),
            },
        )
        .expect("migration")
    }

    fn multilayer_beginning() -> SoilThermalOwnerEnvelopeV2 {
        let mut owner = beginning();
        owner.support_end_ns = 180_000_000_000;
        let transaction = owner.state.last_accepted_transaction_id;
        owner.state.ofes[0]
            .ordered_layers
            .push(SoilThermalLayerStateV2 {
                layer_id: SoilLayerId::try_new("layer-2").expect("second layer"),
                temperature_k: 265.0,
                enthalpy_hi_j_m2_ofe_ground: 4.0,
                enthalpy_carry: ExactDyadicEnthalpy::try_new(1, "1", -53)
                    .expect("second-layer carry"),
                last_accepted_transaction_id: transaction,
            });
        let mut second = owner.state.ofes[0].clone();
        second.ofe_id = OfeId::try_new("ofe-2").expect("second OFE");
        second.ordered_layers[0].layer_id = SoilLayerId::try_new("layer-3").expect("third layer");
        second.ordered_layers[1].layer_id = SoilLayerId::try_new("layer-4").expect("fourth layer");
        second.ordered_layers[1].enthalpy_hi_j_m2_ofe_ground = 8.0;
        second.ordered_layers[1].enthalpy_carry =
            ExactDyadicEnthalpy::try_new(-1, "1", -53).expect("fourth-layer carry");
        owner.state.ofes.push(second);
        owner.state.reseal().expect("multilayer state seal");
        owner.validate().expect("multilayer beginning");
        owner
    }

    #[test]
    fn prepared_read_trial_and_receipt_free_seals_are_native_and_fail_closed() {
        let beginning = beginning();
        let prepared = prepare_soil_thermal_support_v2(
            &beginning,
            beginning.transaction_id,
            beginning.support_start_ns,
            beginning.support_end_ns,
        )
        .expect("prepared support");
        let read = prepared.physical_read_view();
        let layer = &beginning.state.ofes[0].ordered_layers[0];
        assert_eq!(
            read.exact_layer_enthalpy(&beginning.state.ofes[0].ofe_id, &layer.layer_id)
                .expect("exact read"),
            ExactDyadicEnthalpy::zero()
        );
        let seals = seal_soil_thermal_receipt_free_owner_v2(&prepared).expect("native seals");
        validate_soil_thermal_receipt_free_owner_v2(&prepared, &seals)
            .expect("native seal validation");
        let mut poison = seals.clone();
        poison.receipt_free_seal_sha256 = digest('f');
        assert!(validate_soil_thermal_receipt_free_owner_v2(&prepared, &poison).is_err());

        let operand = SoilThermalAcceptedEnergyOperandV2 {
            ofe_id: beginning.state.ofes[0].ofe_id.clone(),
            layer_id: layer.layer_id.clone(),
            source_kind: crate::SoilThermalEnergyOperandKindV2::TopBoundary,
            source_owner_id: ResourceOwnerId::try_new("snow-owner").expect("source owner"),
            debit_credit_identity_sha256: digest('6'),
            ordinal: 0,
            units: "J m^-2 OFE-ground".to_owned(),
            basis: "ofe_ground".to_owned(),
            energy_j_m2_ofe_ground: 1_000.0,
        };
        let projection = SoilThermalTemperatureProjectionV2 {
            ofe_id: operand.ofe_id.clone(),
            layer_id: operand.layer_id.clone(),
            heat_capacity_j_m2_k: 2_000.0,
            ending_temperature_k: 273.65,
        };
        let trial = advance_soil_thermal_trial_v2(
            &prepared,
            std::slice::from_ref(&operand),
            std::slice::from_ref(&projection),
        )
        .expect("unpublished trial");
        assert_eq!(trial.transaction_id(), beginning.transaction_id);
        assert_eq!(
            trial.predecessor_transaction_id(),
            beginning.expected_predecessor_transaction_id
        );
        assert_eq!(trial.support_start_ns(), beginning.support_start_ns);
        assert_eq!(trial.support_end_ns(), beginning.support_end_ns);
        assert_eq!(
            trial.beginning_state_sha256(),
            &beginning.state.state_sha256
        );
        assert_eq!(
            trial.accepted_predecessor_receipt_chain_sha256(),
            Some(&beginning.receipt_chain_sha256)
        );
        assert_eq!(
            trial.ending_state().ofes[0].ordered_layers[0]
                .enthalpy_hi_j_m2_ofe_ground
                .to_bits(),
            1_000.0_f64.to_bits()
        );
        assert_eq!(prepared.beginning_owner(), &beginning);

        let accepted = apply_soil_thermal_energy_credit_v2(&beginning, &[operand], &[projection])
            .expect("accepted candidate");
        assert_eq!(&accepted.ending_owner.state, trial.ending_state());
        let next = prepare_soil_thermal_support_v2(
            &accepted.ending_owner,
            TransactionId(8),
            60_000_000_000,
            120_000_000_000,
        )
        .expect("successor support");
        assert_eq!(
            next.beginning_owner().expected_predecessor_transaction_id,
            Some(TransactionId(7))
        );
        assert!(
            prepare_soil_thermal_support_v2(
                &accepted.ending_owner,
                TransactionId(9),
                120_000_000_000,
                179_999_999_999,
            )
            .is_err(),
            "one tick below the exact floor refuses"
        );
        prepare_soil_thermal_support_v2(
            &accepted.ending_owner,
            TransactionId(9),
            120_000_000_000,
            3_720_000_000_000,
        )
        .expect("stable one-hour support");
    }

    #[test]
    fn composed_trial_binds_child_support_but_recomputes_from_original_beginning() {
        let mut parent = beginning();
        parent.support_end_ns = 180_000_000_000;
        parent.validate().expect("parent owner");
        let prepared = prepare_soil_thermal_support_v2(
            &parent,
            parent.transaction_id,
            parent.support_start_ns,
            parent.support_end_ns,
        )
        .expect("parent prepared");
        let layer = &parent.state.ofes[0].ordered_layers[0];
        let operand = SoilThermalAcceptedEnergyOperandV2 {
            ofe_id: parent.state.ofes[0].ofe_id.clone(),
            layer_id: layer.layer_id.clone(),
            source_kind: crate::SoilThermalEnergyOperandKindV2::TopBoundary,
            source_owner_id: ResourceOwnerId::try_new("snow-owner").expect("source owner"),
            debit_credit_identity_sha256: digest('7'),
            ordinal: 0,
            units: "J m^-2 OFE-ground".to_owned(),
            basis: "ofe_ground".to_owned(),
            energy_j_m2_ofe_ground: 1_000.0,
        };
        let projection = SoilThermalTemperatureProjectionV2 {
            ofe_id: operand.ofe_id.clone(),
            layer_id: operand.layer_id.clone(),
            heat_capacity_j_m2_k: 2_000.0,
            ending_temperature_k: 273.65,
        };
        let trial = advance_soil_thermal_composed_trial_v2(
            &prepared,
            60_000_000_000,
            120_000_000_000,
            &[operand],
            &[projection],
        )
        .expect("composed child trial");
        assert_eq!(trial.support_start_ns(), 60_000_000_000);
        assert_eq!(trial.support_end_ns(), 120_000_000_000);
        assert_eq!(trial.beginning_state_sha256(), &parent.state.state_sha256);
        assert_eq!(
            trial.ending_state().last_accepted_transaction_id,
            Some(parent.transaction_id)
        );
        let second_operand = SoilThermalAcceptedEnergyOperandV2 {
            debit_credit_identity_sha256: digest('8'),
            energy_j_m2_ofe_ground: 1_000.0,
            ..trial.layer_credits()[0].accepted_operands[0].clone()
        };
        let second_projection = SoilThermalTemperatureProjectionV2 {
            ofe_id: trial.layer_credits()[0].ofe_id.clone(),
            layer_id: trial.layer_credits()[0].layer_id.clone(),
            heat_capacity_j_m2_k: trial.layer_credits()[0].heat_capacity_j_m2_k,
            ending_temperature_k: 274.15,
        };
        let sequential = advance_soil_thermal_sequential_unpublished_trial_v2(
            &trial,
            120_000_000_000,
            180_000_000_000,
            std::slice::from_ref(&second_operand),
            std::slice::from_ref(&second_projection),
        )
        .expect("private sequential child");
        assert_eq!(sequential.transaction_id(), TransactionId(8));
        assert_eq!(
            sequential.predecessor_transaction_id(),
            Some(TransactionId(7))
        );
        assert_eq!(
            sequential.beginning_state_sha256(),
            &trial.ending_state().state_sha256
        );
        assert_eq!(
            sequential.unpublished_predecessor_trial_sha256(),
            Some(trial.unpublished_trial_sha256())
        );
        assert!(
            advance_soil_thermal_composed_trial_v2(
                &prepared,
                60_000_000_001,
                120_000_000_000,
                &[],
                &[],
            )
            .is_err(),
            "sub-floor child support refuses before projection"
        );
    }

    #[test]
    #[allow(
        clippy::too_many_lines,
        reason = "one integrated vector proves private projection, exact coordinates, poisons, and rollback"
    )]
    fn numerical_coordinate_projection_is_exact_private_and_fail_closed() {
        let mut beginning = beginning();
        beginning.support_end_ns = 180_000_000_000;
        let layer = &mut beginning.state.ofes[0].ordered_layers[0];
        layer.enthalpy_hi_j_m2_ofe_ground = 1.0;
        layer.enthalpy_carry =
            ExactDyadicEnthalpy::try_new(1, "1", -53).expect("nonzero exact carry");
        beginning.state.reseal().expect("reseal carry beginning");
        beginning.validate().expect("carry beginning");
        let prepared = prepare_soil_thermal_support_v2(
            &beginning,
            beginning.transaction_id,
            beginning.support_start_ns,
            beginning.support_end_ns,
        )
        .expect("prepared coordinate beginning");
        let before = prepared.clone();
        let coordinate = SoilThermalUnpublishedCoordinateV2 {
            ofe_id: beginning.state.ofes[0].ofe_id.clone(),
            layer_id: beginning.state.ofes[0].ordered_layers[0].layer_id.clone(),
            proposed_total_enthalpy_j_m2_ofe_ground: 2.0,
            proposed_temperature_k: 280.0,
        };
        let projection = project_soil_thermal_unpublished_coordinates_v2(
            &prepared,
            std::slice::from_ref(&coordinate),
        )
        .expect("private coordinate projection");
        let trial = projection.trial();
        assert_eq!(trial.transaction_id(), beginning.transaction_id);
        assert_eq!(
            trial.predecessor_transaction_id(),
            beginning.expected_predecessor_transaction_id
        );
        assert_eq!(trial.support_start_ns(), beginning.support_start_ns);
        assert_eq!(trial.support_end_ns(), beginning.support_end_ns);
        assert_eq!(
            trial.beginning_state_sha256(),
            &beginning.state.state_sha256
        );
        assert_eq!(
            trial.accepted_predecessor_receipt_chain_sha256(),
            Some(&beginning.receipt_chain_sha256)
        );
        assert!(trial.numerical_coordinate_authority_sha256().is_some());
        assert!(trial.numerical_coordinate_set_sha256().is_some());
        assert!(trial.layer_credits().is_empty());
        let ending = &trial.ending_state().ofes[0].ordered_layers[0];
        assert_eq!(
            ending.enthalpy_hi_j_m2_ofe_ground.to_bits(),
            coordinate.proposed_total_enthalpy_j_m2_ofe_ground.to_bits()
        );
        assert_eq!(ending.enthalpy_carry, ExactDyadicEnthalpy::zero());
        assert_eq!(
            ending.temperature_k.to_bits(),
            coordinate.proposed_temperature_k.to_bits()
        );
        let sibling = project_soil_thermal_unpublished_coordinates_v2(
            &prepared,
            &[SoilThermalUnpublishedCoordinateV2 {
                proposed_total_enthalpy_j_m2_ofe_ground: 3.0,
                ..coordinate.clone()
            }],
        )
        .expect("sibling coordinate projection");
        assert_eq!(
            sibling.trial().beginning_state_sha256(),
            trial.beginning_state_sha256()
        );
        assert_ne!(
            sibling.trial().unpublished_trial_sha256(),
            trial.unpublished_trial_sha256()
        );
        assert!(
            advance_soil_thermal_sequential_unpublished_trial_v2(
                trial,
                180_000_000_000,
                240_000_000_000,
                &[],
                &[],
            )
            .is_err(),
            "coordinate projection cannot become sequential authority"
        );
        assert!(
            compose_soil_thermal_accepted_from_unpublished_v2(
                &prepared,
                trial,
                &[],
                &[vec![], vec![]],
            )
            .is_err(),
            "coordinate projection cannot become accepted authority"
        );
        for poison in [
            SoilThermalUnpublishedCoordinateV2 {
                proposed_total_enthalpy_j_m2_ofe_ground: f64::NAN,
                ..coordinate.clone()
            },
            SoilThermalUnpublishedCoordinateV2 {
                proposed_total_enthalpy_j_m2_ofe_ground: -0.0,
                ..coordinate.clone()
            },
            SoilThermalUnpublishedCoordinateV2 {
                proposed_temperature_k: f64::INFINITY,
                ..coordinate.clone()
            },
            SoilThermalUnpublishedCoordinateV2 {
                proposed_temperature_k: 199.999,
                ..coordinate.clone()
            },
            SoilThermalUnpublishedCoordinateV2 {
                ofe_id: OfeId::try_new("wrong-ofe").expect("wrong OFE"),
                ..coordinate.clone()
            },
        ] {
            assert!(project_soil_thermal_unpublished_coordinates_v2(&prepared, &[poison]).is_err());
            assert_eq!(
                prepared, before,
                "projection refusal must not mutate beginning"
            );
        }
        assert!(project_soil_thermal_unpublished_coordinates_v2(&prepared, &[]).is_err());
        assert_eq!(prepared, before);
    }

    #[test]
    fn top_layer_coordinate_projection_preserves_lower_layers_bit_exact() {
        let beginning = multilayer_beginning();
        let prepared = prepare_soil_thermal_support_v2(
            &beginning,
            beginning.transaction_id,
            beginning.support_start_ns,
            beginning.support_end_ns,
        )
        .expect("multilayer prepared beginning");
        let before = prepared.clone();
        let coordinates = beginning
            .state
            .ofes
            .iter()
            .enumerate()
            .map(|(index, ofe)| SoilThermalUnpublishedCoordinateV2 {
                ofe_id: ofe.ofe_id.clone(),
                layer_id: ofe.ordered_layers[0].layer_id.clone(),
                proposed_total_enthalpy_j_m2_ofe_ground: 20.0 + index as f64,
                proposed_temperature_k: 270.0 + index as f64,
            })
            .collect::<Vec<_>>();
        let projection =
            project_soil_thermal_unpublished_top_layer_coordinates_v2(&prepared, &coordinates)
                .expect("top-layer coordinate projection");
        for ((beginning_ofe, ending_ofe), coordinate) in beginning
            .state
            .ofes
            .iter()
            .zip(&projection.trial().ending_state().ofes)
            .zip(&coordinates)
        {
            let ending_top = &ending_ofe.ordered_layers[0];
            assert_eq!(
                ending_top.enthalpy_hi_j_m2_ofe_ground.to_bits(),
                coordinate.proposed_total_enthalpy_j_m2_ofe_ground.to_bits()
            );
            assert_eq!(ending_top.enthalpy_carry, ExactDyadicEnthalpy::zero());
            assert_eq!(
                ending_top.temperature_k.to_bits(),
                coordinate.proposed_temperature_k.to_bits()
            );
            assert_eq!(
                &ending_ofe.ordered_layers[1..],
                &beginning_ofe.ordered_layers[1..],
                "every lower-layer identity/high/carry/T bit must be retained"
            );
        }
        let mut reversed = coordinates.clone();
        reversed.reverse();
        let mut wrong_layer = coordinates.clone();
        wrong_layer[0].layer_id = beginning.state.ofes[0].ordered_layers[1].layer_id.clone();
        for poison in [reversed, coordinates[..1].to_vec(), wrong_layer] {
            assert!(
                project_soil_thermal_unpublished_top_layer_coordinates_v2(&prepared, &poison)
                    .is_err()
            );
            assert_eq!(
                prepared, before,
                "top-layer refusal must not mutate beginning"
            );
        }
        let mut duplicate = coordinates.clone();
        duplicate.push(coordinates[1].clone());
        assert!(
            project_soil_thermal_unpublished_top_layer_coordinates_v2(&prepared, &duplicate)
                .is_err()
        );
        assert_eq!(prepared, before);
    }

    #[test]
    fn receipt_free_serialization_has_no_adaptive_diagnostic_metadata() {
        let beginning = beginning();
        let prepared = prepare_soil_thermal_support_v2(
            &beginning,
            beginning.transaction_id,
            beginning.support_start_ns,
            beginning.support_end_ns,
        )
        .expect("prepared support");
        let seals = seal_soil_thermal_receipt_free_owner_v2(&prepared).expect("native seals");
        let text = serde_json::to_string(&(prepared.beginning_owner(), seals))
            .expect("canonical-compatible JSON");
        for forbidden in [
            "microstep",
            "iteration",
            "solver",
            "diagnostic",
            "carry_diagnostic",
        ] {
            assert!(!text.contains(forbidden), "forbidden key {forbidden}");
        }
    }
}
