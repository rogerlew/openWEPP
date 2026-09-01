fn snow_soil_receipt_reseal_roundoff_within_bound_v1(
    residual_j_m2: f64,
    temperature_residual_k: f64,
) -> bool {
    residual_j_m2.is_finite()
        && residual_j_m2 >= 0.0
        && residual_j_m2
            <= crate::snow_stage3_v11_attachment::STAGE3_V11_SNOW_SOIL_RECEIPT_RESEAL_ROUNDOFF_J_M2
        && temperature_residual_k.is_finite()
        && temperature_residual_k >= 0.0
        && temperature_residual_k
            <= crate::snow_stage3_v11_attachment::STAGE3_V11_SNOW_SOIL_RECEIPT_RESEAL_ROUNDOFF_TEMPERATURE_K
}

/// Select the receipt-replayed soil trial without weakening physical ending
/// identity. V2 trial custody is expected to change when the final debit
/// receipt digest is installed into its layer credits, but its complete
/// ending owner state must remain byte-for-byte equivalent.
fn select_identity_replayed_soil_candidate_v1(
    installed: &DirectSoilThermalCandidate,
    identity_replayed: DirectSoilThermalCandidate,
) -> Result<DirectSoilThermalCandidate, DirectV11RealConsumerError> {
    let physical_ending_matches = match (installed, &identity_replayed) {
        (DirectSoilThermalCandidate::V1(installed), DirectSoilThermalCandidate::V1(replayed)) => {
            installed == replayed
        }
        (DirectSoilThermalCandidate::V2(installed), DirectSoilThermalCandidate::V2(replayed)) => {
            installed.ending_state() == replayed.ending_state()
        }
        _ => false,
    };
    if !physical_ending_matches {
        return Err(DirectV11RealConsumerError::Identity(
            "snow-soil identity-only receipt reseal physical ending",
        ));
    }
    Ok(identity_replayed)
}

#[cfg(test)]
mod snow_soil_receipt_reseal_roundoff_tests {
    use super::*;

    fn digest(fill: char) -> openwepp_land_surface_energy::Sha256Digest {
        openwepp_land_surface_energy::Sha256Digest::try_new(fill.to_string().repeat(64))
            .expect("digest")
    }

    fn v2_trial(
        debit_digest: char,
        energy_j_m2: f64,
        layer_id: &str,
    ) -> DirectSoilThermalCandidate {
        let beginning = openwepp_land_surface_energy::migrate_soil_thermal_v1_to_v2(
            &openwepp_land_surface_energy::SoilThermalSnapshot {
                owner_id: openwepp_kernel_contract::ResourceOwnerId::try_new("soil-owner")
                    .expect("owner"),
                configuration_sha256: digest('1'),
                state_sha256: digest('2'),
                snapshot_sha256: digest('3'),
                last_accepted_transaction_id: None,
                ofes: vec![openwepp_land_surface_energy::SoilThermalOfeSnapshot {
                    ofe_id: openwepp_land_surface_energy::OfeId::try_new("ofe-1").expect("OFE"),
                    ordered_layers: vec![openwepp_land_surface_energy::SoilThermalLayerSnapshot {
                        layer_id: openwepp_kernel_contract::SoilLayerId::try_new(layer_id)
                            .expect("layer"),
                        temperature_k: 273.15,
                        enthalpy_j_m2_ofe_ground: 0.0,
                    }],
                }],
            },
            openwepp_land_surface_energy::SoilThermalV2MigrationIdentity {
                model_version: "OPENWEPP_SOIL_THERMAL_EXACT_CARRY_V2".to_owned(),
                model_definition_sha256: digest('4'),
                run_id: "receipt-reseal-selection-test".to_owned(),
                transaction_id: openwepp_kernel_contract::TransactionId(7),
                support_start_ns: 0,
                support_end_ns: 60_000_000_000,
                receipt_chain_sha256: digest('5'),
            },
        )
        .expect("migration");
        let prepared = openwepp_land_surface_energy::prepare_soil_thermal_support_v2(
            &beginning,
            beginning.transaction_id,
            beginning.support_start_ns,
            beginning.support_end_ns,
        )
        .expect("prepared support");
        let ofe_id = beginning.state.ofes[0].ofe_id.clone();
        let layer_id = beginning.state.ofes[0].ordered_layers[0].layer_id.clone();
        let operand = openwepp_land_surface_energy::SoilThermalAcceptedEnergyOperandV2 {
            ofe_id: ofe_id.clone(),
            layer_id: layer_id.clone(),
            source_kind: openwepp_land_surface_energy::SoilThermalEnergyOperandKindV2::TopBoundary,
            source_owner_id: openwepp_kernel_contract::ResourceOwnerId::try_new("snow-owner")
                .expect("source owner"),
            debit_credit_identity_sha256: digest(debit_digest),
            ordinal: 0,
            units: "J m^-2 OFE-ground".to_owned(),
            basis: "ofe_ground".to_owned(),
            energy_j_m2_ofe_ground: energy_j_m2,
        };
        let projection = openwepp_land_surface_energy::SoilThermalTemperatureProjectionV2 {
            ofe_id,
            layer_id,
            heat_capacity_j_m2_k: 2_000.0,
            ending_temperature_k: 273.65,
        };
        let trial = openwepp_land_surface_energy::advance_soil_thermal_trial_v2(
            &prepared,
            std::slice::from_ref(&operand),
            std::slice::from_ref(&projection),
        )
        .expect("unpublished trial");
        DirectSoilThermalCandidate::from_v2(trial).expect("typed candidate")
    }

    #[test]
    fn exact_threshold_sides_are_fail_closed() {
        let energy_threshold =
            crate::snow_stage3_v11_attachment::STAGE3_V11_SNOW_SOIL_RECEIPT_RESEAL_ROUNDOFF_J_M2;
        let temperature_threshold = crate::snow_stage3_v11_attachment::STAGE3_V11_SNOW_SOIL_RECEIPT_RESEAL_ROUNDOFF_TEMPERATURE_K;
        assert!(snow_soil_receipt_reseal_roundoff_within_bound_v1(0.0, 0.0));
        assert!(snow_soil_receipt_reseal_roundoff_within_bound_v1(
            f64::from_bits(energy_threshold.to_bits() - 1),
            f64::from_bits(temperature_threshold.to_bits() - 1),
        ));
        assert!(snow_soil_receipt_reseal_roundoff_within_bound_v1(
            energy_threshold,
            temperature_threshold,
        ));
        assert!(!snow_soil_receipt_reseal_roundoff_within_bound_v1(
            f64::from_bits(energy_threshold.to_bits() + 1),
            temperature_threshold,
        ));
        assert!(!snow_soil_receipt_reseal_roundoff_within_bound_v1(
            energy_threshold,
            f64::from_bits(temperature_threshold.to_bits() + 1),
        ));
        assert!(!snow_soil_receipt_reseal_roundoff_within_bound_v1(
            -energy_threshold,
            temperature_threshold,
        ));
        assert!(!snow_soil_receipt_reseal_roundoff_within_bound_v1(
            energy_threshold,
            -temperature_threshold,
        ));
        assert!(!snow_soil_receipt_reseal_roundoff_within_bound_v1(
            f64::NAN,
            temperature_threshold,
        ));
        assert!(!snow_soil_receipt_reseal_roundoff_within_bound_v1(
            energy_threshold,
            f64::NAN,
        ));
    }

    #[test]
    fn v2_replay_selects_final_receipt_custody_when_physical_ending_is_exact() {
        let installed = v2_trial('6', 1_000.0, "layer-1");
        let replayed = v2_trial('7', 1_000.0, "layer-1");
        let (
            DirectSoilThermalCandidate::V2(installed_trial),
            DirectSoilThermalCandidate::V2(replayed_trial),
        ) = (&installed, &replayed)
        else {
            panic!("native V2 fixtures")
        };
        assert_eq!(
            installed_trial.ending_state(),
            replayed_trial.ending_state()
        );
        assert_ne!(
            installed_trial.unpublished_trial_sha256(),
            replayed_trial.unpublished_trial_sha256(),
            "the complete trial seal must bind the changed receipt custody"
        );
        assert_ne!(
            installed_trial.layer_credits(),
            replayed_trial.layer_credits()
        );

        let selected =
            select_identity_replayed_soil_candidate_v1(&installed, replayed).expect("exact ending");
        let DirectSoilThermalCandidate::V2(selected) = selected else {
            panic!("selected native V2 trial")
        };
        assert_eq!(
            selected.layer_credits()[0].accepted_operands[0].debit_credit_identity_sha256,
            digest('7'),
            "the selected credit must bind the final resealed receipt digest"
        );
    }

    #[test]
    fn v2_replay_refuses_one_ulp_physical_ending_change() {
        let installed = v2_trial('6', 1_000.0, "layer-1");
        let replayed = v2_trial('7', f64::from_bits(1_000.0_f64.to_bits() + 1), "layer-1");
        assert!(matches!(
            select_identity_replayed_soil_candidate_v1(&installed, replayed),
            Err(DirectV11RealConsumerError::Identity(
                "snow-soil identity-only receipt reseal physical ending"
            ))
        ));
    }

    #[test]
    fn v2_replay_refuses_topology_change() {
        let installed = v2_trial('6', 1_000.0, "layer-1");
        let replayed = v2_trial('7', 1_000.0, "layer-other");
        assert!(matches!(
            select_identity_replayed_soil_candidate_v1(&installed, replayed),
            Err(DirectV11RealConsumerError::Identity(
                "snow-soil identity-only receipt reseal physical ending"
            ))
        ));
    }
}
