mod soil_thermal_v2_cutover_tests {
    use openwepp_kernel_contract::{ResourceOwnerId, SoilLayerId, TransactionId};

    use super::*;
    use crate::{
        OfeId, SoilThermalLayerSnapshot, SoilThermalOfeSnapshot, SoilThermalV2MigrationIdentity,
        migrate_soil_thermal_v1_to_v2, prepare_soil_thermal_support_v2,
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
            trial.beginning_state_sha256(),
            &beginning.state.state_sha256
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
