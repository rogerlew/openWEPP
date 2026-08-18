#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        BandDirectionalFluxes, BareSoilParameters, MODEL_DEFINITION_SHA256, OpenNeutralGeometry,
        SoilThermalLayerSnapshot, SoilThermalNodeOperands, SoilThermalOfeSnapshot,
        SurfaceStorageBranch,
    };
    use serde_json::Value;

    fn digest(byte: char) -> Sha256Digest {
        Sha256Digest::try_new(byte.to_string().repeat(64)).expect("test digest")
    }

    fn owner(value: &str) -> ResourceOwnerId {
        ResourceOwnerId::try_new(value).expect("test owner")
    }

    fn layer(value: &str) -> SoilLayerId {
        SoilLayerId::try_new(value).expect("test layer")
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

    fn identity() -> RuntimeTileIdentity {
        RuntimeTileIdentity {
            transaction_id: TransactionId(41),
            lse_owner_id: owner("lse"),
            hydrology_owner_id: owner("hydrology"),
            soil_thermal_owner_id: owner("soil-thermal"),
            vegetation_owner_id: owner("vegetation-real-owner"),
            biogeochemistry_owner_id: owner("bgc-real-owner"),
            configuration_sha256: digest('a'),
            beginning_lse_state_sha256: digest('b'),
            beginning_hydrology_snapshot_sha256: digest('c'),
            beginning_soil_thermal_state_sha256: digest('d'),
            beginning_vegetation_state_sha256: digest('e'),
            beginning_biogeochemistry_state_sha256: digest('f'),
            ofe_id: OfeId::try_new("ofe-1").expect("ofe"),
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

    fn soil_snapshot() -> SoilThermalSnapshot {
        SoilThermalSnapshot {
            owner_id: owner("soil-thermal"),
            configuration_sha256: digest('e'),
            state_sha256: digest('d'),
            snapshot_sha256: digest('f'),
            last_accepted_transaction_id: Some(TransactionId(40)),
            ofes: vec![SoilThermalOfeSnapshot {
                ofe_id: OfeId::try_new("ofe-1").expect("ofe"),
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

    fn vectors() -> Value {
        serde_json::from_str(include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/../../docs/work-packages/20260814-snow-free-land-surface-energy-authority-001/artifacts/openwepp_snow_free_lse_v1_vectors.json"
        )))
        .expect("authority vectors")
    }

    #[test]
    fn open_transaction_binds_frozen_vector_and_rebuilds_from_beginning() {
        let beginning = problem();
        let phase = solve_open_potential_phase(identity(), &beginning, None).expect("potential");
        assert_eq!(beginning, problem());
        let request_before = phase.request_batch.requests.clone();
        let authorization = WaterAuthorization {
            key: request_before[0].key.clone(),
            amount_kg_m2_stand_ground: 0.000_053_040_160_893_323_02 * 1_800.0,
            reason: crate::WaterAuthorizationReason::ProportionalSupply,
        };
        let candidate =
            finalize_open_phase(&phase, &digest('b'), &authorization, None, &soil_snapshot())
                .expect("final");
        assert_eq!(candidate.water_protocol.requests, request_before);
        assert_eq!(
            candidate.water_protocol.finalized_uses[0]
                .amount_kg_m2_stand_ground
                .to_bits(),
            authorization.amount_kg_m2_stand_ground.to_bits()
        );
        assert_eq!(candidate.rollback_hashes.len(), 5);
        assert!(
            candidate
                .rollback_hashes
                .iter()
                .all(|row| row.before_sha256 == row.after_sha256)
        );
        let expected = &vectors()["exact_model_reductions"]["open_bare_soil_four_layer"]["fixed_cap_rebuilt_from_beginning"]
            ["solution"];
        for (actual, frozen) in candidate
            .final_solver_candidate
            .solution
            .iter()
            .zip(expected.as_array().expect("solution array"))
        {
            assert!((actual - frozen.as_f64().expect("number")).abs() < 2.0e-10);
        }
        candidate
            .energy_operands
            .validate()
            .expect("independent operands");
    }

    #[test]
    fn rejected_open_cap_diagnostics_distinguish_active_inactive_and_tie() {
        let phase = solve_open_potential_phase(identity(), &problem(), None).expect("potential");
        let law = evaluate_open_surface(&phase.beginning, &phase.accepted.solution, None, None)
            .expect("uncapped failed iterate")
            .water
            .law_kg_m2_tile_s;
        assert!(law > 0.0);
        let failure = NumericalFailure {
            kind: NumericalFailureKind::IterationLimit,
            iterations: 50,
            normalized_residuals: vec![2.0],
            ordered_residuals: vec![NormalizedResidual {
                identity: "ground_surface_energy".into(),
                raw: 2.0,
                scale: 2.0,
                tolerance: 1.0,
                normalized: 2.0,
                unit: crate::ResidualUnit::WattsPerSquareMeter,
            }],
            failed_solution: phase.accepted.solution.clone(),
            occupancy_id: None,
            active_bounds: Vec::new(),
            backtracking_count: 0,
            step_norms: StepNorms {
                temperature_k: Some(1.0),
                humidity_kg_kg: None,
                ci_pa: None,
                hydraulic_mm: None,
                beta: None,
            },
            pivot_magnitude: Some(1.0),
            matrix_norm: Some(1.0),
        };
        let key = phase.request_batch.requests[0].key.clone();
        assert_eq!(
            rejected_open_active_caps(&phase, &failure, law * 0.5).unwrap(),
            vec![key.clone()]
        );
        assert_eq!(
            rejected_open_active_caps(&phase, &failure, law).unwrap(),
            vec![key]
        );
        assert!(
            rejected_open_active_caps(&phase, &failure, law * 2.0)
                .unwrap()
                .is_empty()
        );
    }

    #[test]
    fn rejected_covered_caps_distinguish_active_inactive_tie_and_request_order() {
        assert!(cap_is_active_or_tie(10.0, 9.0));
        assert!(cap_is_active_or_tie(10.0, 10.0));
        assert!(!cap_is_active_or_tie(10.0, 11.0));
        let mut active_key = identity().ground_key();
        active_key.source_id = SourceId::try_new("active-source").unwrap();
        let mut inactive_key = identity().ground_key();
        inactive_key.source_id = SourceId::try_new("inactive-source").unwrap();
        let mut tie_key = identity().ground_key();
        tie_key.source_id = SourceId::try_new("tie-source").unwrap();
        let requests = vec![
            WaterAmount {
                key: tie_key.clone(),
                amount_kg_m2_stand_ground: 10.0,
            },
            WaterAmount {
                key: inactive_key,
                amount_kg_m2_stand_ground: 11.0,
            },
            WaterAmount {
                key: active_key.clone(),
                amount_kg_m2_stand_ground: 9.0,
            },
        ];
        let active = BTreeSet::from([active_key.clone(), tie_key.clone()]);
        assert_eq!(
            ordered_active_cap_keys(&requests, &active),
            vec![tie_key, active_key]
        );
    }

    #[test]
    fn v10_full_supply_requires_exact_amount_reason_and_identity() {
        let mut key = identity().ground_key();
        key.requesting_component = crate::RequestingComponent::VegetationRoot;
        key.occupancy_id = Some(crate::ComponentId::try_new("occupancy-1").expect("occupancy"));
        key.surface_id = None;
        key.surface_class = None;
        key.source_type = crate::WaterSourceType::SoilLayerLiquid;
        key.source_tile_id = None;
        key.soil_layer_id = Some(layer("soil-layer-1"));
        let request = WaterAmount {
            key: key.clone(),
            amount_kg_m2_stand_ground: 0.25,
        };
        let batch = PotentialWaterRequestBatch::try_new(
            TransactionId(41),
            digest('b'),
            vec![request.clone()],
        )
        .expect("request batch");
        let exact = WaterAuthorization {
            key: key.clone(),
            amount_kg_m2_stand_ground: request.amount_kg_m2_stand_ground,
            reason: crate::WaterAuthorizationReason::FullSupply,
        };
        assert!(v10_exact_full_supply(
            &batch,
            &BTreeMap::from([(key.clone(), exact.clone())])
        ));

        let mut one_bit = exact.clone();
        one_bit.amount_kg_m2_stand_ground =
            f64::from_bits(one_bit.amount_kg_m2_stand_ground.to_bits() - 1);
        one_bit.reason = crate::WaterAuthorizationReason::ProportionalSupply;
        assert!(!v10_exact_full_supply(
            &batch,
            &BTreeMap::from([(key.clone(), one_bit)])
        ));

        let partial = WaterAuthorization {
            key: key.clone(),
            amount_kg_m2_stand_ground: 0.125,
            reason: crate::WaterAuthorizationReason::ProportionalSupply,
        };
        assert_eq!(
            require_v10_exact_full_supply(true, &batch, &BTreeMap::from([(key.clone(), partial)])),
            Err(LandSurfaceEnergyError::UnsupportedDomain(
                "V10 nonpositive-assimilation partial root authorization"
            ))
        );

        let ground_key = identity().ground_key();
        let ground_request = WaterAmount {
            key: ground_key.clone(),
            amount_kg_m2_stand_ground: 0.25,
        };
        let ground_batch = PotentialWaterRequestBatch::try_new(
            TransactionId(41),
            digest('b'),
            vec![ground_request],
        )
        .expect("ground request batch");
        let partial_ground = WaterAuthorization {
            key: ground_key.clone(),
            amount_kg_m2_stand_ground: 0.125,
            reason: crate::WaterAuthorizationReason::ProportionalSupply,
        };
        assert_eq!(
            require_v10_exact_full_supply(
                true,
                &ground_batch,
                &BTreeMap::from([(ground_key, partial_ground)])
            ),
            Ok(())
        );

        let mut wrong_key = key.clone();
        wrong_key.source_id = SourceId::try_new("wrong-source").expect("source");
        assert!(!v10_exact_full_supply(
            &batch,
            &BTreeMap::from([(wrong_key, exact)])
        ));
    }

    #[test]
    fn v10_finalization_rejects_wrong_sealed_gas_branch_receipt() {
        use crate::V10LeafGasBranch::{ExactZeroPar, RespirationDominated};

        assert!(matches!(
            validate_sealed_gas_branches(
                &[[ExactZeroPar, RespirationDominated]],
                &[[ExactZeroPar, ExactZeroPar]],
            ),
            Err(LandSurfaceEnergyError::WaterIdentityOrBound {
                class: crate::WaterErrorClass::Identity,
                ..
            })
        ));
    }

    #[test]
    fn covered_phase_lineage_rejects_resealed_batch_identity() {
        let identity = identity();
        let request = WaterAmount {
            key: identity.ground_key(),
            amount_kg_m2_stand_ground: 0.25,
        };
        let wrong_transaction = PotentialWaterRequestBatch::try_new(
            TransactionId(42),
            identity.beginning_lse_state_sha256.clone(),
            vec![WaterAmount {
                key: GroundWaterKey {
                    transaction_id: TransactionId(42),
                    ..request.key.clone()
                },
                ..request.clone()
            }],
        )
        .expect("self-consistent wrong transaction");
        assert!(matches!(
            validate_covered_phase_lineage(&identity, &wrong_transaction),
            Err(LandSurfaceEnergyError::WaterIdentityOrBound {
                class: crate::WaterErrorClass::Identity,
                ..
            })
        ));

        let wrong_beginning = PotentialWaterRequestBatch::try_new(
            identity.transaction_id,
            digest('9'),
            vec![request],
        )
        .expect("self-consistent wrong beginning");
        assert!(matches!(
            validate_covered_phase_lineage(&identity, &wrong_beginning),
            Err(LandSurfaceEnergyError::WaterIdentityOrBound {
                class: crate::WaterErrorClass::Identity,
                ..
            })
        ));
    }

    #[test]
    fn stale_potential_and_producer_residual_evasion_fail_closed() {
        let phase = solve_open_potential_phase(identity(), &problem(), None).expect("potential");
        let mut altered_batch = phase.request_batch.clone();
        altered_batch.requests[0].amount_kg_m2_stand_ground = f64::from_bits(
            altered_batch.requests[0]
                .amount_kg_m2_stand_ground
                .to_bits()
                + 1,
        );
        assert!(matches!(
            altered_batch.validate(),
            Err(LandSurfaceEnergyError::Identity {
                field: "potential request batch digest",
                ..
            })
        ));
        let authorization = WaterAuthorization {
            key: phase.request_batch.requests[0].key.clone(),
            amount_kg_m2_stand_ground: phase.request_batch.requests[0].amount_kg_m2_stand_ground,
            reason: crate::WaterAuthorizationReason::FullSupply,
        };
        assert!(matches!(
            finalize_open_phase(&phase, &digest('9'), &authorization, None, &soil_snapshot()),
            Err(LandSurfaceEnergyError::StateLineage(_))
        ));
        let mut candidate =
            finalize_open_phase(&phase, &digest('b'), &authorization, None, &soil_snapshot())
                .expect("final");
        candidate.energy_operands.surface.storage_w_m2 += 1.0;
        assert!(matches!(
            candidate.energy_operands.validate(),
            Err(LandSurfaceEnergyError::ControlVolumeClosure(_))
        ));
    }

    #[test]
    fn post_ingress_carries_exact_enthalpy_to_surface_and_soil() {
        let phase = solve_open_potential_phase(identity(), &problem(), None).expect("potential");
        let authorization = WaterAuthorization {
            key: phase.request_batch.requests[0].key.clone(),
            amount_kg_m2_stand_ground: phase.request_batch.requests[0].amount_kg_m2_stand_ground,
            reason: crate::WaterAuthorizationReason::FullSupply,
        };
        let mut candidate =
            finalize_open_phase(&phase, &digest('b'), &authorization, None, &soil_snapshot())
                .expect("final");
        let surface_before = candidate
            .ending_tile_state_pre_ingress
            .surface_enthalpy_j_m2_tile_ground;
        let soil_before = candidate.soil_thermal.layers[0].ending_enthalpy_j_m2_ofe_ground;
        let records = [
            AdvectedLiquidRecord {
                parcel_id: "rain-retained".into(),
                disposition: PostIngressDisposition::RetainedSurface,
                mass_kg_m2_tile_ground: 0.2,
                temperature_k: Some(285.0),
                enthalpy_j_m2_tile_ground: 0.2 * liquid_enthalpy_j_kg(285.0),
            },
            AdvectedLiquidRecord {
                parcel_id: "rain-infiltration".into(),
                disposition: PostIngressDisposition::Infiltration,
                mass_kg_m2_tile_ground: 0.3,
                temperature_k: Some(286.0),
                enthalpy_j_m2_tile_ground: 0.3 * liquid_enthalpy_j_kg(286.0),
            },
        ];
        apply_post_ingress(&mut candidate, &records).expect("post ingress");
        assert_eq!(
            candidate
                .ending_tile_state_pre_ingress
                .surface_enthalpy_j_m2_tile_ground
                .to_bits(),
            (surface_before + records[0].enthalpy_j_m2_tile_ground).to_bits()
        );
        assert_eq!(
            candidate.soil_thermal.layers[0]
                .ending_enthalpy_j_m2_ofe_ground
                .to_bits(),
            (soil_before + records[1].enthalpy_j_m2_tile_ground).to_bits()
        );
    }

    #[test]
    fn ending_state_digest_binds_transaction_and_every_tile() {
        let beginning = LandSurfaceEnergyState {
            model_definition_sha256: Sha256Digest::try_new(MODEL_DEFINITION_SHA256)
                .expect("model digest"),
            configuration_sha256: digest('a'),
            state_sha256: digest('b'),
            owner_id: owner("lse"),
            last_accepted_transaction_id: Some(TransactionId(40)),
            tiles: vec![TileState {
                ofe_id: OfeId::try_new("ofe-1").expect("ofe"),
                tile_id: TileId::try_new("tile-open").expect("tile"),
                surface_enthalpy_j_m2_tile_ground: 1.0,
                surface_temperature_warm_start_k: 290.0,
            }],
        };
        let ending = build_lse_ending_state(
            &beginning,
            TransactionId(41),
            vec![TileState {
                ofe_id: OfeId::try_new("ofe-1").expect("ofe"),
                tile_id: TileId::try_new("tile-open").expect("tile"),
                surface_enthalpy_j_m2_tile_ground: 2.0,
                surface_temperature_warm_start_k: 291.0,
            }],
        )
        .expect("ending state");
        assert_eq!(ending.last_accepted_transaction_id, Some(TransactionId(41)));
        assert_ne!(ending.state_sha256, beginning.state_sha256);
    }

    #[test]
    fn numerical_failure_errors_preserve_kind_diagnostics_and_rollback_lineage() {
        let identity = identity();
        for (kind, diagnostic_kind) in [
            (
                NumericalFailureKind::SingularPivot,
                DiagnosticFailureKind::SingularPivot,
            ),
            (
                NumericalFailureKind::BacktrackingLimit,
                DiagnosticFailureKind::BacktrackingLimit,
            ),
            (
                NumericalFailureKind::IterationLimit,
                DiagnosticFailureKind::IterationLimit,
            ),
        ] {
            let failure = NumericalFailure {
                kind,
                iterations: 7,
                normalized_residuals: vec![2.0, -3.0],
                ordered_residuals: vec![
                    NormalizedResidual {
                        identity: "ground_surface_energy".into(),
                        raw: 4.0,
                        scale: 4.0,
                        tolerance: 2.0,
                        normalized: 2.0,
                        unit: crate::ResidualUnit::WattsPerSquareMeter,
                    },
                    NormalizedResidual {
                        identity: "soil_thermal:thermal-1".into(),
                        raw: -6.0,
                        scale: 6.0,
                        tolerance: 2.0,
                        normalized: -3.0,
                        unit: crate::ResidualUnit::WattsPerSquareMeter,
                    },
                ],
                failed_solution: vec![295.0, 291.0],
                occupancy_id: None,
                active_bounds: vec!["surface_liquid_store_cap".into()],
                backtracking_count: 20,
                step_norms: StepNorms {
                    temperature_k: Some(1.0e-5),
                    humidity_kg_kg: Some(2.0e-9),
                    ci_pa: Some(3.0e-5),
                    hydraulic_mm: Some(4.0e-6),
                    beta: Some(5.0e-10),
                },
                pivot_magnitude: Some(2.0e-9),
                matrix_norm: Some(4.0),
            };
            let error = numerical_failure_error(
                &identity,
                SolvePass::Potential,
                SolveIdentity::SurfaceEnergy,
                &failure,
                Vec::new(),
            )
            .expect("typed numerical error");
            match (&kind, &error) {
                (
                    NumericalFailureKind::SingularPivot,
                    LandSurfaceEnergyError::NumericalSingular { .. },
                )
                | (
                    NumericalFailureKind::BacktrackingLimit,
                    LandSurfaceEnergyError::NumericalBacktrackingLimit { .. },
                )
                | (
                    NumericalFailureKind::IterationLimit,
                    LandSurfaceEnergyError::NumericalIterationLimit { .. },
                ) => {}
                _ => panic!("wrong public numerical error variant: {error:?}"),
            }
            let diagnostics = error
                .numerical_diagnostics()
                .expect("public error carries diagnostics");
            assert!(!diagnostics.accepted);
            assert_eq!(diagnostics.pass, SolvePass::Potential);
            assert_eq!(diagnostics.failure_kind, Some(diagnostic_kind));
            assert_eq!(diagnostics.step_norms, failure.step_norms);
            assert_eq!(diagnostics.ordered_residuals, failure.ordered_residuals);
            assert!(diagnostics.active_water_caps.is_empty());
            assert_truthful_rollback_hashes(diagnostics, &identity);
        }
    }

    fn assert_truthful_rollback_hashes(
        diagnostics: &NumericalDiagnostics,
        identity: &RuntimeTileIdentity,
    ) {
        assert_eq!(diagnostics.owner_rollback_hashes.len(), 5);
        assert!(
            diagnostics
                .owner_rollback_hashes
                .iter()
                .all(|row| row.before_sha256 == row.after_sha256)
        );
        let vegetation = diagnostics
            .owner_rollback_hashes
            .iter()
            .find(|row| row.owner_kind == OwnerKind::Vegetation)
            .unwrap();
        assert_eq!(vegetation.owner_id, identity.vegetation_owner_id.as_str());
        assert_eq!(
            vegetation.before_sha256,
            identity.beginning_vegetation_state_sha256
        );
        let bgc = diagnostics
            .owner_rollback_hashes
            .iter()
            .find(|row| row.owner_kind == OwnerKind::Biogeochemistry)
            .unwrap();
        assert_eq!(bgc.owner_id, identity.biogeochemistry_owner_id.as_str());
        assert_eq!(
            bgc.before_sha256,
            identity.beginning_biogeochemistry_state_sha256
        );
    }

    #[test]
    fn condensation_receipt_binds_exact_surface_identity_temperature_and_enthalpy() {
        let mut identity = identity();
        identity.ground_source_type = WaterSourceType::SurfaceLiquid;
        identity.ground_source_tile_id = Some(identity.tile_id.clone());
        identity.ground_soil_layer_id = None;
        let credits = condensation_credits(&identity, 0.0125, 281.0).expect("credit");
        assert_eq!(credits.len(), 1);
        let credit = &credits[0];
        assert_eq!(credit.transaction_id, identity.transaction_id);
        assert_eq!(credit.tile_id, identity.tile_id);
        assert_eq!(credit.surface_id, identity.surface_id);
        assert!((credit.amount_kg_m2_stand_ground - 0.0125).abs() < f64::EPSILON);
        assert_eq!(
            credit.specific_liquid_enthalpy_j_kg.to_bits(),
            liquid_enthalpy_j_kg(281.0).to_bits()
        );
    }

    fn accepted_vegetation_fixture() -> AcceptedCoveredVegetationOperands {
        let tile = identity();
        let occupancy_id = ComponentId::try_new("stratum-a@tile-open").expect("occupancy");
        let runtime = RootRuntimeIdentity {
            solver_occupancy_id: "canopy-rank-0".into(),
            requesting_owner_id: owner("vegetation-v8"),
            occupancy_id: occupancy_id.clone(),
            layer_id: layer("soil-layer-1"),
            source_id: SourceId::try_new("soil-layer-1").expect("source"),
        };
        let mut result = AcceptedCoveredVegetationOperands {
            pass: CoveredVegetationOperandPass::FixedAuthorizationFinal,
            transaction_id: tile.transaction_id,
            vegetation_model_version: VEGETATION_MODEL_VERSION,
            vegetation_model_definition_sha256: VEGETATION_MODEL_DEFINITION_SHA256,
            lse_configuration_sha256: tile.configuration_sha256.clone(),
            beginning_lse_state_sha256: tile.beginning_lse_state_sha256.clone(),
            vegetation_owner_id: runtime.requesting_owner_id.clone(),
            ofe_id: tile.ofe_id.clone(),
            tile_id: tile.tile_id.clone(),
            tile_fraction: tile.tile_fraction,
            interval_s: tile.interval_s,
            canopy_air_temperature_k: 295.0,
            canopy_air_specific_humidity_kg_kg: 0.01,
            top_rain_kg_m2_tile_ground: 0.0,
            ground_canopy_release_kg_m2_tile_ground: 0.0,
            ground_stemflow_kg_m2_tile_ground: 0.0,
            occupancies: vec![AcceptedCoveredOccupancyOperands {
                occupancy_id,
                sun_leaf_area_m2_m2_tile_ground: 1.2,
                shade_leaf_area_m2_m2_tile_ground: 0.8,
                sun_leaf_potential_mm: -5_900.0,
                shade_leaf_potential_mm: -5_500.0,
                stem_potential_mm: -4_300.0,
                root_node_potential_mm: -2_850.0,
                beta_sun: 0.5,
                beta_shade: 0.25,
                sun_emax_kg_m2_tile_s: 3.0,
                shade_emax_kg_m2_tile_s: 1.0,
                beta_hyd: 0.4375,
                sun_leaf_temperature_k: 296.0,
                shade_leaf_temperature_k: 295.5,
                wet_surface_temperature_k: 295.2,
                dry_stem_temperature_k: 294.8,
                sun_ci_pa: 28.0,
                shade_ci_pa: 30.0,
                sun_gross_assimilation_umol_co2_m2_leaf_s: 12.0,
                shade_gross_assimilation_umol_co2_m2_leaf_s: 6.0,
                sun_net_assimilation_umol_co2_m2_leaf_s: 11.0,
                shade_net_assimilation_umol_co2_m2_leaf_s: 5.5,
                sun_dark_respiration_umol_co2_m2_leaf_s: 1.0,
                shade_dark_respiration_umol_co2_m2_leaf_s: 0.5,
                signed_wet_phase_change_kg_m2_tile_ground: 0.01,
                wet_phase_branch: crate::WaterBranch::ConstitutiveLaw,
                liquid: CoveredOccupancyLiquidLedger {
                    pass: crate::CoveredLiquidPass::FixedAuthorizationFinal,
                    beginning_store_kg_m2_tile: 0.02,
                    incident_rain_kg_m2_tile: 0.0,
                    ending_store_kg_m2_tile: 0.01,
                    evaporation_kg_m2_tile: 0.01,
                    condensation_kg_m2_tile: 0.0,
                    throughfall_kg_m2_tile: 0.0,
                    stemflow_kg_m2_tile: 0.0,
                    initial_drainage_kg_m2_tile: 0.0,
                    second_drainage_kg_m2_tile: 0.0,
                    wet_fraction: 0.5,
                    wet_surface_temperature_k: 295.2,
                    wet_surface_specific_enthalpy_j_kg: crate::WATER_HEAT_CAPACITY_J_KG_K
                        * (295.2 - crate::REFERENCE_TEMPERATURE_K),
                },
                root_water: vec![AcceptedRootWaterOperands {
                    key: root_key(&tile, &runtime),
                    request_kg_m2_stand_ground: 0.3,
                    authorization_kg_m2_stand_ground: 0.2,
                    finalized_use_kg_m2_stand_ground: 0.15,
                }],
            }],
            payload_sha256: tile.beginning_lse_state_sha256.clone(),
            seal: SealedCoveredVegetationOperands::FixedAuthorizationFinal,
        };
        result.payload_sha256 = canonical_digest(&result).expect("accepted payload digest");
        result
    }

    #[test]
    fn accepted_v8_payload_validates_identity_state_carbon_and_root_daf() {
        let accepted = accepted_vegetation_fixture();
        accepted.validate().expect("accepted V8 operands");
        assert_eq!(
            accepted.occupancies[0]
                .sun_net_assimilation_umol_co2_m2_leaf_s
                .to_bits(),
            (accepted.occupancies[0].sun_gross_assimilation_umol_co2_m2_leaf_s
                - accepted.occupancies[0].sun_dark_respiration_umol_co2_m2_leaf_s)
                .to_bits()
        );
        assert_eq!(
            accepted.occupancies[0].root_water[0]
                .key
                .requesting_owner_id,
            accepted.vegetation_owner_id
        );
    }

    #[test]
    fn accepted_v8_payload_rejects_pass_derived_and_identity_poisons() {
        let mut wrong_pass = accepted_vegetation_fixture();
        wrong_pass.pass = CoveredVegetationOperandPass::Potential;
        assert!(matches!(
            wrong_pass.validate(),
            Err(LandSurfaceEnergyError::StateLineage(_))
        ));

        let mut wrong_carbon = accepted_vegetation_fixture();
        wrong_carbon.occupancies[0].sun_net_assimilation_umol_co2_m2_leaf_s = 12.0;
        assert!(matches!(
            wrong_carbon.validate(),
            Err(LandSurfaceEnergyError::Identity {
                field: "accepted vegetation operand digest",
                ..
            })
        ));

        let mut wrong_owner = accepted_vegetation_fixture();
        wrong_owner.occupancies[0].root_water[0]
            .key
            .requesting_owner_id = owner("other-vegetation");
        assert!(matches!(
            wrong_owner.validate(),
            Err(LandSurfaceEnergyError::Identity {
                field: "accepted vegetation operand digest",
                ..
            })
        ));

        let mut wrong_daf = accepted_vegetation_fixture();
        wrong_daf.occupancies[0].root_water[0].finalized_use_kg_m2_stand_ground = 0.21;
        assert!(matches!(
            wrong_daf.validate(),
            Err(LandSurfaceEnergyError::Identity {
                field: "accepted vegetation operand digest",
                ..
            })
        ));

        let mut stale_potential_liquid = accepted_vegetation_fixture();
        stale_potential_liquid.occupancies[0]
            .liquid
            .ending_store_kg_m2_tile = 0.02;
        stale_potential_liquid.occupancies[0]
            .liquid
            .evaporation_kg_m2_tile = 0.0;
        stale_potential_liquid.occupancies[0].signed_wet_phase_change_kg_m2_tile_ground = 0.0;
        assert!(matches!(
            stale_potential_liquid.validate(),
            Err(LandSurfaceEnergyError::Identity {
                field: "accepted vegetation operand digest",
                ..
            })
        ));

        // Module-local adversarial tests can recompute the payload seal. The
        // independent final-pass and D/A/F validators must still reject
        // producer-consistent but noncanonical payloads.
        let mut sealed_stale_potential_liquid = accepted_vegetation_fixture();
        sealed_stale_potential_liquid.occupancies[0].liquid.pass =
            crate::CoveredLiquidPass::Potential;
        sealed_stale_potential_liquid.payload_sha256 =
            canonical_digest(&sealed_stale_potential_liquid)
                .expect("reseal stale-potential poison");
        assert!(matches!(
            sealed_stale_potential_liquid.validate(),
            Err(LandSurfaceEnergyError::StateLineage(
                "accepted vegetation liquid pass"
            ))
        ));

        let mut sealed_finalized_above_authorization = accepted_vegetation_fixture();
        sealed_finalized_above_authorization.occupancies[0].root_water[0]
            .finalized_use_kg_m2_stand_ground = 0.21;
        sealed_finalized_above_authorization.payload_sha256 =
            canonical_digest(&sealed_finalized_above_authorization).expect("reseal D/A/F poison");
        assert!(matches!(
            sealed_finalized_above_authorization.validate(),
            Err(LandSurfaceEnergyError::WaterIdentityOrBound {
                class: crate::WaterErrorClass::Bound,
                ..
            })
        ));
    }
}
