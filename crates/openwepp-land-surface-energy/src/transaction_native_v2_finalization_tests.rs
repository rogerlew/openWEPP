mod native_v2_finalization_tests {
    use openwepp_kernel_contract::{ResourceOwnerId, SoilLayerId, TileId, TransactionId};

    use super::*;
    use crate::{
        BandDirectionalFluxes, BareSoilParameters, OpenNeutralGeometry, SoilThermalLayerSnapshot,
        SoilThermalNodeOperands, SoilThermalOfeSnapshot, SoilThermalV2MigrationIdentity,
        SurfaceStorageBranch, migrate_soil_thermal_v1_to_v2, prepare_soil_thermal_support_v2,
    };

    fn digest(fill: char) -> Sha256Digest {
        Sha256Digest::try_new(fill.to_string().repeat(64)).expect("digest")
    }

    fn owner(value: &str) -> ResourceOwnerId {
        ResourceOwnerId::try_new(value).expect("owner")
    }

    fn layer(value: &str) -> SoilLayerId {
        SoilLayerId::try_new(value).expect("layer")
    }

    fn identity() -> RuntimeTileIdentity {
        RuntimeTileIdentity {
            transaction_id: TransactionId(41),
            soil_thermal_transaction_id: TransactionId(41),
            lse_owner_id: owner("lse"),
            hydrology_owner_id: owner("hydrology"),
            soil_thermal_owner_id: owner("soil-thermal"),
            vegetation_owner_id: owner("vegetation"),
            biogeochemistry_owner_id: owner("bgc"),
            configuration_sha256: digest('a'),
            beginning_lse_state_sha256: digest('b'),
            beginning_hydrology_snapshot_sha256: digest('c'),
            beginning_soil_thermal_state_sha256: digest('d'),
            beginning_vegetation_state_sha256: digest('e'),
            beginning_biogeochemistry_state_sha256: digest('f'),
            ofe_id: OfeId::try_new("ofe-1").expect("OFE"),
            tile_id: TileId::try_new("tile-open").expect("tile"),
            surface_id: SurfaceId::try_new("surface-open").expect("surface"),
            surface_class: SurfaceClass::BareMineralSoil,
            ground_source_type: WaterSourceType::SoilLayerLiquid,
            ground_source_id: SourceId::try_new("soil-layer-1").expect("source"),
            ground_source_tile_id: None,
            ground_soil_layer_id: Some(layer("soil-layer-1")),
            tile_fraction: 1.0,
            interval_s: 1_800.0,
        }
    }

    fn problem() -> OpenSurfaceProblem {
        OpenSurfaceProblem {
            interval_s: 1_800.0,
            tile_fraction: 1.0,
            class: SurfaceClassKind::BareMineralSoil,
            storage_branch: SurfaceStorageBranch::FiniteCapacity,
            terminal_shortwave_w_m2_tile: BandDirectionalFluxes {
                direct_vis: 91.0,
                diffuse_vis: 31.0,
                direct_nir: 117.0,
                diffuse_nir: 39.0,
            },
            surface_vis_albedo: 0.18,
            surface_nir_albedo: 0.31,
            surface_emissivity: 1.0,
            surface_depth_m: 0.02,
            surface_conductivity_w_m_k: 0.75,
            surface_dry_heat_capacity_j_m2_k: 42_000.0,
            litter_capacity_kg_m2_tile: None,
            open_geometry: OpenNeutralGeometry {
                reference_height_m: 20.0,
                roughness_momentum_m: 0.12,
                roughness_heat_m: 0.015,
                roughness_vapor_m: 0.010,
            },
            air_temperature_k: 294.0,
            air_specific_humidity_kg_kg: 0.0095,
            air_pressure_pa: 93_000.0,
            reference_wind_m_s: 2.4,
            atmospheric_downward_longwave_w_m2: 335.0,
            surface_liquid_kg_m2_tile: 0.0,
            surface_enthalpy_j_m2_tile: 42_000.0 * (295.0 - crate::REFERENCE_TEMPERATURE_K),
            surface_temperature_warm_start_k: 295.0,
            bare_soil: Some(BareSoilParameters {
                top_layer_liquid_kg_m2: 26.0,
                top_layer_ice_kg_m2: 0.0,
                porosity: 0.46,
                saturated_matric_potential_mm: -120.0,
                clapp_hornberger_b: 4.05,
                theta_initial: 0.22,
            }),
            soil_nodes: (0..4)
                .map(|index| SoilThermalNodeOperands {
                    layer_id: format!("thermal-{}", index + 1),
                    depth_m: 0.08 + 0.05 * f64::from(index),
                    conductivity_w_m_k: 1.1 + 0.12 * f64::from(index),
                    heat_capacity_j_m2_k: 120_000.0 + 35_000.0 * f64::from(index),
                    beginning_temperature_k: 291.5 - 1.1 * f64::from(index),
                })
                .collect(),
        }
    }

    fn snapshot() -> SoilThermalSnapshot {
        SoilThermalSnapshot {
            owner_id: owner("soil-thermal"),
            configuration_sha256: digest('e'),
            state_sha256: digest('d'),
            snapshot_sha256: digest('f'),
            last_accepted_transaction_id: Some(TransactionId(40)),
            ofes: vec![SoilThermalOfeSnapshot {
                ofe_id: OfeId::try_new("ofe-1").expect("OFE"),
                ordered_layers: (0..4)
                    .map(|index| SoilThermalLayerSnapshot {
                        layer_id: layer(&format!("thermal-{}", index + 1)),
                        temperature_k: 291.5 - 1.1 * f64::from(index),
                        enthalpy_j_m2_ofe_ground: 1.0e6 * f64::from(index + 1),
                    })
                    .collect(),
            }],
        }
    }

    fn migrated() -> SoilThermalOwnerEnvelopeV2 {
        migrate_soil_thermal_v1_to_v2(
            &snapshot(),
            SoilThermalV2MigrationIdentity {
                model_version: "OPENWEPP_SOIL_THERMAL_EXACT_CARRY_V2".to_owned(),
                model_definition_sha256: digest('4'),
                run_id: "native-finalization".to_owned(),
                transaction_id: TransactionId(41),
                support_start_ns: 0,
                support_end_ns: 1_800_000_000_000,
                receipt_chain_sha256: digest('5'),
            },
        )
        .expect("migration")
    }

    fn authorization(phase: &OpenPotentialPhase) -> WaterAuthorization {
        WaterAuthorization {
            key: phase.request_batch.requests[0].key.clone(),
            amount_kg_m2_stand_ground: 0.000_053_040_160_893_323_02 * 1_800.0,
            reason: crate::WaterAuthorizationReason::ProportionalSupply,
        }
    }

    fn phase_for(owner: &SoilThermalOwnerEnvelopeV2) -> OpenPotentialPhase {
        let mut runtime = identity();
        runtime.beginning_soil_thermal_state_sha256 = owner.state.state_sha256.clone();
        solve_open_potential_phase(runtime, &problem(), None).expect("potential")
    }

    #[test]
    fn v1_wrapper_is_bitwise_identical_to_typed_v1_finalization() {
        let phase = solve_open_potential_phase(identity(), &problem(), None).expect("potential");
        let authorization = authorization(&phase);
        let beginning = snapshot();
        let legacy = finalize_open_phase(&phase, &digest('b'), &authorization, None, &beginning)
            .expect("legacy wrapper");
        let typed = finalize_open_phase_with_soil_thermal_beginning(
            &phase,
            &digest('b'),
            &authorization,
            None,
            SoilThermalFinalizationBeginning::V1(&beginning),
        )
        .expect("typed V1");
        assert_eq!(legacy, typed);
        assert!(matches!(
            legacy.soil_thermal.beginning_identity,
            SoilThermalCandidateBeginningIdentity::V1 { .. }
        ));
    }

    #[test]
    fn native_v2_finalization_retains_zero_and_nonzero_exact_carry_without_mutation() {
        let zero_owner = migrated();
        let zero_phase = phase_for(&zero_owner);
        let zero_prepared =
            prepare_soil_thermal_support_v2(&zero_owner, TransactionId(41), 0, 1_800_000_000_000)
                .expect("zero prepared");
        let zero = finalize_open_phase_with_soil_thermal_beginning(
            &zero_phase,
            &digest('b'),
            &authorization(&zero_phase),
            None,
            SoilThermalFinalizationBeginning::V2(zero_prepared.physical_read_view()),
        )
        .expect("zero-carry V2 finalization");
        assert_eq!(
            zero.soil_thermal.layers[0].beginning_enthalpy_carry,
            ExactDyadicEnthalpy::zero()
        );

        let mut carry_owner = migrated();
        let carry = ExactDyadicEnthalpy::from_f64(f64::from_bits(1)).expect("minimum subnormal");
        carry_owner.state.ofes[0].ordered_layers[0].enthalpy_carry = carry.clone();
        carry_owner.state.reseal().expect("reseal carry owner");
        carry_owner.validate().expect("canonical carry owner");
        let before = carry_owner.clone();
        let phase = phase_for(&carry_owner);
        let prepared =
            prepare_soil_thermal_support_v2(&carry_owner, TransactionId(41), 0, 1_800_000_000_000)
                .expect("carry prepared");
        let candidate = finalize_open_phase_with_soil_thermal_beginning(
            &phase,
            &digest('b'),
            &authorization(&phase),
            None,
            SoilThermalFinalizationBeginning::V2(prepared.physical_read_view()),
        )
        .expect("nonzero-carry V2 finalization");
        assert_eq!(carry_owner, before, "finalization is read-only");
        assert_eq!(
            candidate.soil_thermal.layers[0].beginning_enthalpy_carry,
            carry
        );
        assert_eq!(
            candidate.soil_thermal.layers[0]
                .beginning_enthalpy_j_m2_ofe_ground
                .to_bits(),
            zero.soil_thermal.layers[0]
                .beginning_enthalpy_j_m2_ofe_ground
                .to_bits()
        );
        assert_eq!(
            candidate.soil_thermal.layers[0]
                .ending_temperature_k
                .to_bits(),
            zero.soil_thermal.layers[0].ending_temperature_k.to_bits()
        );
        assert!(matches!(
            candidate.soil_thermal.beginning_identity,
            SoilThermalCandidateBeginningIdentity::V2 { .. }
        ));

        let pass_through = build_soil_thermal_passthrough_candidate(
            &phase.identity,
            SoilThermalFinalizationBeginning::V2(prepared.physical_read_view()),
        )
        .expect("native pass-through");
        assert_eq!(pass_through.layers[0].beginning_enthalpy_carry, carry);
        assert_eq!(
            pass_through.layers[0]
                .ground_heat_credit_j_m2_ofe_ground
                .to_bits(),
            0.0_f64.to_bits()
        );
        assert_eq!(
            pass_through.layers[0]
                .beginning_enthalpy_j_m2_ofe_ground
                .to_bits(),
            pass_through.layers[0]
                .ending_enthalpy_j_m2_ofe_ground
                .to_bits()
        );
    }

    #[test]
    fn native_v2_finalization_separates_outer_and_second_child_soil_transactions() {
        let mut first_child_accepted = migrated();
        first_child_accepted.transaction_id = TransactionId(42);
        first_child_accepted.expected_predecessor_transaction_id = Some(TransactionId(41));
        first_child_accepted.state.last_accepted_transaction_id = Some(TransactionId(42));
        for ofe in &mut first_child_accepted.state.ofes {
            for layer in &mut ofe.ordered_layers {
                layer.last_accepted_transaction_id = Some(TransactionId(42));
            }
        }
        first_child_accepted
            .state
            .reseal()
            .expect("first-child accepted state");
        first_child_accepted
            .validate()
            .expect("first-child accepted owner");
        let second_child = prepare_soil_thermal_support_v2(
            &first_child_accepted,
            TransactionId(43),
            0,
            1_800_000_000_000,
        )
        .expect("second-child support");

        let mut split_identity = identity();
        split_identity.transaction_id = TransactionId(42);
        split_identity.soil_thermal_transaction_id = TransactionId(43);
        split_identity.beginning_soil_thermal_state_sha256 =
            first_child_accepted.state.state_sha256.clone();
        let phase = solve_open_potential_phase(split_identity, &problem(), None)
            .expect("outer-42/soil-43 potential");
        assert_eq!(phase.identity.transaction_id, TransactionId(42));
        assert_eq!(
            phase.identity.soil_thermal_transaction_id,
            TransactionId(43)
        );
        assert_eq!(phase.request_batch.transaction_id, TransactionId(42));
        let candidate = finalize_open_phase_with_soil_thermal_beginning(
            &phase,
            &digest('b'),
            &authorization(&phase),
            None,
            SoilThermalFinalizationBeginning::V2(second_child.physical_read_view()),
        )
        .expect("outer-42/soil-43 finalization");
        assert!(matches!(
            candidate.soil_thermal.beginning_identity,
            SoilThermalCandidateBeginningIdentity::V2 {
                transaction_id: TransactionId(43),
                expected_predecessor_transaction_id: Some(TransactionId(42)),
                ..
            }
        ));
        build_soil_thermal_passthrough_candidate(
            &phase.identity,
            SoilThermalFinalizationBeginning::V2(second_child.physical_read_view()),
        )
        .expect("outer-42/soil-43 pass-through");

        for poison in [TransactionId(42), TransactionId(44)] {
            let mut poisoned = phase.clone();
            poisoned.identity.soil_thermal_transaction_id = poison;
            assert!(
                finalize_open_phase_with_soil_thermal_beginning(
                    &poisoned,
                    &digest('b'),
                    &authorization(&poisoned),
                    None,
                    SoilThermalFinalizationBeginning::V2(second_child.physical_read_view()),
                )
                .is_err(),
                "stale or out-of-order soil transaction must fail closed"
            );
        }
        let mut missing = phase.identity.clone();
        missing.soil_thermal_transaction_id = TransactionId(0);
        assert!(
            solve_open_potential_phase(missing, &problem(), None).is_err(),
            "missing soil transaction must fail before physics"
        );
    }

    #[test]
    fn native_v2_finalization_rejects_stale_reordered_duplicate_and_physical_poison() {
        let owner = migrated();

        let mut stale_identity = identity();
        stale_identity.beginning_soil_thermal_state_sha256 = owner.state.state_sha256.clone();
        stale_identity.soil_thermal_transaction_id = TransactionId(42);
        let stale_phase =
            solve_open_potential_phase(stale_identity, &problem(), None).expect("stale potential");
        let prepared =
            prepare_soil_thermal_support_v2(&owner, TransactionId(41), 0, 1_800_000_000_000)
                .expect("prepared");
        assert!(
            finalize_open_phase_with_soil_thermal_beginning(
                &stale_phase,
                &digest('b'),
                &authorization(&stale_phase),
                None,
                SoilThermalFinalizationBeginning::V2(prepared.physical_read_view()),
            )
            .is_err()
        );

        let mut reordered = migrated();
        reordered.state.ofes[0].ordered_layers.swap(0, 1);
        reordered.state.reseal().expect("reordered digest");
        let reordered_phase = phase_for(&reordered);
        let reordered_prepared =
            prepare_soil_thermal_support_v2(&reordered, TransactionId(41), 0, 1_800_000_000_000)
                .expect("reordered prepared");
        assert!(
            finalize_open_phase_with_soil_thermal_beginning(
                &reordered_phase,
                &digest('b'),
                &authorization(&reordered_phase),
                None,
                SoilThermalFinalizationBeginning::V2(reordered_prepared.physical_read_view(),),
            )
            .is_err()
        );

        let mut wrong_temperature = migrated();
        wrong_temperature.state.ofes[0].ordered_layers[0].temperature_k = 292.0;
        wrong_temperature
            .state
            .reseal()
            .expect("temperature digest");
        let wrong_phase = phase_for(&wrong_temperature);
        let wrong_prepared = prepare_soil_thermal_support_v2(
            &wrong_temperature,
            TransactionId(41),
            0,
            1_800_000_000_000,
        )
        .expect("temperature prepared");
        let before = wrong_temperature.clone();
        assert!(
            finalize_open_phase_with_soil_thermal_beginning(
                &wrong_phase,
                &digest('b'),
                &authorization(&wrong_phase),
                None,
                SoilThermalFinalizationBeginning::V2(wrong_prepared.physical_read_view()),
            )
            .is_err()
        );
        assert_eq!(wrong_temperature, before, "refusal is clone-only");

        let mut duplicate = migrated();
        let repeated_layer = duplicate.state.ofes[0].ordered_layers[0].clone();
        duplicate.state.ofes[0].ordered_layers.push(repeated_layer);
        duplicate.state.reseal().expect("duplicate digest");
        assert!(
            prepare_soil_thermal_support_v2(&duplicate, TransactionId(41), 0, 1_800_000_000_000,)
                .is_err()
        );

        let mut bad_carry = migrated();
        bad_carry.state.ofes[0].ordered_layers[0].enthalpy_carry =
            ExactDyadicEnthalpy::from_f64(1.0).expect("finite carry poison");
        bad_carry.state.reseal().expect("carry digest");
        assert!(
            prepare_soil_thermal_support_v2(&bad_carry, TransactionId(41), 0, 1_800_000_000_000,)
                .is_err()
        );
    }
}
