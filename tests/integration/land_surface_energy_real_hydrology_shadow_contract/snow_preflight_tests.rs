use super::*;

fn set_snow_scalar(frame: &mut DirectRunFrame, field: usize, value: f64) {
    let snow = &mut frame.lanes[0].winter_column.snow;
    match field {
        0 => snow.runtime_swe_m = value,
        1 => snow.runtime_depth_m = value,
        2 => snow.runtime_density_kg_m3 = value,
        3 => snow.runtime_settle_day_count = value,
        4 => snow.coe_boundary_depth_m = value,
        5 => snow.coe_boundary_density_kg_m3 = value,
        6 => snow.coe_boundary_settle_day_count = value,
        7 => snow.liquid_water_retained_m = value,
        _ => unreachable!("complete DirectSnowLaneState scalar table"),
    }
}

fn set_snow_carry_scalar(frame: &mut DirectRunFrame, field: usize, value: f64) {
    let mut carry = openwepp_hillslope_orchestrator::DirectSnowRuntimeCarry::from(
        openwepp_hillslope_orchestrator::DirectSnowLaneState::zero(),
    );
    match field {
        0 => carry.runtime_swe_m = value,
        1 => carry.runtime_depth_m = value,
        2 => carry.runtime_density_kg_m3 = value,
        3 => carry.runtime_settle_day_count = value,
        4 => carry.coe_boundary_depth_m = value,
        5 => carry.coe_boundary_density_kg_m3 = value,
        6 => carry.coe_boundary_settle_day_count = value,
        7 => carry.liquid_water_retained_m = value,
        _ => unreachable!("complete DirectSnowRuntimeCarry scalar table"),
    }
    frame.lanes[0].snow_runtime_carry = Some(Box::new(carry));
}

fn execute_snow_poison(
    field: usize,
    value: f64,
    runtime_carry: bool,
) -> (LandSurfaceEnergyShadowError, Sha256Digest, bool) {
    execute_winter_mutation(|frame| {
        if runtime_carry {
            set_snow_carry_scalar(frame, field, value);
        } else {
            set_snow_scalar(frame, field, value);
        }
    })
}

fn execute_winter_mutation(
    mutate: impl FnOnce(&mut DirectRunFrame),
) -> (LandSurfaceEnergyShadowError, Sha256Digest, bool) {
    let (mut frame, configuration) = configured_surface_frame(
        SurfaceClass::BareMineralSoil,
        WaterSourceType::SurfaceLiquid,
        1.0,
    );
    mutate(&mut frame);
    let (owner, _) = owner(&frame);
    let adapter = LandSurfaceEnergyRealHydrologyAdapter::new(&owner);
    let snapshot = unified_beginning_hydrology_snapshot_sha256(&adapter, &configuration)
        .expect("snow scalar snapshot remains representable");
    let batch = surface_potential_batch(
        SurfaceClass::BareMineralSoil,
        WaterSourceType::SurfaceLiquid,
        configuration.records[0].key.source_id.clone(),
        1.0,
    );
    let mut callback_called = false;
    let error = execute_unified_real_hydrology_shadow(
        &adapter,
        &configuration,
        &receiver_expectations(1, snapshot.clone()),
        &batch,
        &BTreeMap::new(),
        &ingress_input(),
        |_| {
            callback_called = true;
            Err(LandSurfaceEnergyShadowError::Identity(
                "snow scalar poison reached callback",
            ))
        },
    )
    .expect_err("snow scalar poison must fail at entry");
    (error, snapshot, callback_called)
}

fn assert_snow_failure(
    error: LandSurfaceEnergyShadowError,
    snapshot: &Sha256Digest,
    expected_code: DirectSurfaceLiquidErrorCode,
) -> String {
    let LandSurfaceEnergyShadowError::SurfaceLiquid(error) = error else {
        panic!("snow scalar poison must be canonical");
    };
    let failure = error.failure().expect("canonical snow scalar failure");
    assert_eq!(failure.code, expected_code);
    assert_eq!(failure.phase, DirectSurfaceLiquidPhase::AtomicEnvelope);
    assert_eq!(failure.context.transaction_id, Some(TransactionId(41)));
    assert_eq!(
        failure
            .context
            .owner_id
            .as_ref()
            .map(ResourceOwnerId::as_str),
        Some("production-hydrology")
    );
    assert_eq!(
        failure.context.ofe_id.as_ref().map(OfeId::as_str),
        Some("ofe-1")
    );
    assert_eq!(
        failure.context.tile_id.as_ref().map(TileId::as_str),
        Some("open")
    );
    assert_eq!(
        failure.context.surface_id.as_ref().map(SurfaceId::as_str),
        Some("surface:ofe-1:open")
    );
    assert_eq!(
        failure.context.source_id.as_ref().map(SourceId::as_str),
        Some("surface-store:ofe-1:open")
    );
    assert_eq!(
        failure.rollback.beginning_owner_sha256.as_deref(),
        Some(snapshot.as_str())
    );
    failure
        .rollback
        .attempted_owner_sha256
        .clone()
        .expect("attempted request hash")
}

#[test]
fn snow_top_level_scalars_reject_invalid_before_unsupported_winter() {
    let mut attempted_sha256 = None;
    for runtime_carry in [false, true] {
        for field in 0..8 {
            for value in [-1.0, f64::NAN, f64::INFINITY, f64::NEG_INFINITY] {
                let (error, snapshot, callback_called) =
                    execute_snow_poison(field, value, runtime_carry);
                assert!(!callback_called, "field {field} reached callback");
                let attempted =
                    assert_snow_failure(error, &snapshot, DirectSurfaceLiquidErrorCode::E003);
                if let Some(expected) = &attempted_sha256 {
                    assert_eq!(&attempted, expected, "field {field} request hash");
                } else {
                    attempted_sha256 = Some(attempted);
                }
            }

            let (error, snapshot, callback_called) =
                execute_snow_poison(field, 0.001, runtime_carry);
            assert!(!callback_called, "positive field {field} reached callback");
            let attempted =
                assert_snow_failure(error, &snapshot, DirectSurfaceLiquidErrorCode::E004);
            assert_eq!(attempted_sha256.as_ref(), Some(&attempted));
        }
    }
}

fn valid_layered_snow() -> openwepp_hillslope_orchestrator::DirectSnowLaneState {
    let layer = openwepp_hillslope_orchestrator::DirectSnowLayerState::new(0.1, 0.2, 500.0, 1.0)
        .with_stage3_thermal_liquid_state(-5.0, 0.01, 1_000.0, 0.005);
    let mut snow = openwepp_hillslope_orchestrator::DirectSnowLaneState::from_runtime_values(
        0.1, 0.2, 500.0, 1.0,
    );
    snow.layers = vec![layer];
    snow
}

fn assert_mutation_code(
    mutate: impl FnOnce(&mut DirectRunFrame),
    expected_code: DirectSurfaceLiquidErrorCode,
) {
    let (error, snapshot, callback_called) = execute_winter_mutation(mutate);
    assert!(!callback_called);
    assert_snow_failure(error, &snapshot, expected_code);
}

#[test]
fn snow_nested_and_cross_field_domains_precede_e004() {
    for field in 0..8 {
        assert_mutation_code(
            move |frame| {
                let mut snow = valid_layered_snow();
                let layer = &mut snow.layers[0];
                match field {
                    0 => layer.mass_swe_m = f64::NAN,
                    1 => layer.thickness_m = f64::NAN,
                    2 => layer.density_kg_m3 = f64::NAN,
                    3 => layer.settle_day_count = f64::NAN,
                    4 => layer.temperature_c = f64::NAN,
                    5 => layer.liquid_water_m = f64::NAN,
                    6 => layer.cold_content_j_m2 = f64::NAN,
                    7 => layer.refrozen_liquid_m = f64::NAN,
                    _ => unreachable!(),
                }
                frame.lanes[0].winter_column.snow = snow;
            },
            DirectSurfaceLiquidErrorCode::E003,
        );
    }
    for poison in 0..7 {
        assert_mutation_code(
            move |frame| {
                let mut snow = valid_layered_snow();
                match poison {
                    0 => snow.layers[0].density_kg_m3 = 523.0,
                    1 => snow.layers[0].mass_swe_m = 0.09,
                    2 => snow.layers[0].thickness_m = 0.19,
                    3 => snow.layers[0].liquid_water_m = 0.11,
                    4 => snow.layers[0].refrozen_liquid_m = 0.11,
                    5 => snow.layers[0].density_kg_m3 = 400.0,
                    6 => snow.runtime_density_kg_m3 = 400.0,
                    _ => unreachable!(),
                }
                frame.lanes[0].winter_column.snow = snow;
            },
            DirectSurfaceLiquidErrorCode::E003,
        );
    }
    for poison in 0..3 {
        assert_mutation_code(
            move |frame| {
                let mut snow = if poison == 2 {
                    openwepp_hillslope_orchestrator::DirectSnowLaneState::zero()
                } else {
                    valid_layered_snow()
                };
                snow.snow_albedo_state = Some(openwepp_hillslope_orchestrator::SnowAlbedoState {
                    model:
                        openwepp_hillslope_orchestrator::SnowAlbedoModel::Brock2000TemperatureAgeV1,
                    albedo: if poison == 0 { f64::NAN } else { 0.8 },
                    accumulated_positive_temperature_c_day: if poison == 1 { -1.0 } else { 1.0 },
                });
                frame.lanes[0].winter_column.snow = snow;
            },
            DirectSurfaceLiquidErrorCode::E003,
        );
    }
    assert_mutation_code(
        |frame| frame.lanes[0].winter_column.snow = valid_layered_snow(),
        DirectSurfaceLiquidErrorCode::E004,
    );
}

#[test]
fn snow_runtime_carry_reuses_complete_nested_validation() {
    for poison in 0..3 {
        assert_mutation_code(
            move |frame| {
                let mut snow = valid_layered_snow();
                match poison {
                    0 => snow.layers[0].temperature_c = f64::INFINITY,
                    1 => snow.layers[0].cold_content_j_m2 = -1.0,
                    2 => {
                        snow.snow_albedo_state = Some(
                            openwepp_hillslope_orchestrator::SnowAlbedoState {
                                model: openwepp_hillslope_orchestrator::SnowAlbedoModel::Brock2000TemperatureAgeV1,
                                albedo: 2.0,
                                accumulated_positive_temperature_c_day: 1.0,
                            },
                        );
                    }
                    _ => unreachable!(),
                }
                frame.lanes[0].snow_runtime_carry = Some(Box::new(
                    openwepp_hillslope_orchestrator::DirectSnowRuntimeCarry::from(snow),
                ));
            },
            DirectSurfaceLiquidErrorCode::E003,
        );
    }
}

fn set_frost_scalar(
    frost: &mut openwepp_hillslope_orchestrator::DirectFrostLaneState,
    field: usize,
    value: f64,
) {
    match field {
        0 => frost.dfrost_m = value,
        1 => frost.dthaw_m = value,
        2 => frost.nft = value,
        3 => frost.ws_frz_m = value,
        4 => frost.infcap_frz_m_s = value,
        5 => frost.frwatc_soil_water_before_m = value,
        6 => frost.frwatc_soil_water_after_m = value,
        7 => frost.frwatc_frozen_water_before_m = value,
        8 => frost.frwatc_frozen_water_after_m = value,
        9 => frost.frwatc_freeze_debit_m = value,
        10 => frost.frwatc_thaw_credit_m = value,
        11 => frost.frwatc_net_liquid_delta_m = value,
        12 => frost.frdp_m = value,
        13 => frost.thdp_m = value,
        14 => frost.tfrdp_m = value,
        15 => frost.tthawd_m = value,
        16 => frost.fgthwd_flag = value,
        17 => frost.total_fine_layer_count = value,
        18 => frost.conductivity_tilled_w_m_k = value,
        19 => frost.conductivity_untilled_w_m_k = value,
        20 => frost.conductivity_residue_w_m_k = value,
        21 => frost.shadow_total_water_before_m = value,
        22 => frost.shadow_total_water_after_m = value,
        23 => frost.shadow_wb_delta_m = value,
        24 => frost.shadow_frwatc_residual_m = value,
        25 => frost.watpdg_m = value,
        26 => frost.watbtm_m = value,
        _ => unreachable!("complete DirectFrostLaneState scalar table"),
    }
}

fn frost_shadow(
    layer_index: usize,
) -> openwepp_hillslope_orchestrator::DirectFrostLayerShadowState {
    openwepp_hillslope_orchestrator::DirectFrostLayerShadowState {
        layer_index,
        st_m: 0.0,
        soil_water_m: 0.0,
        frozen_depth_m: 0.0,
        frozen_water_m: 0.0,
        soilf_m: 0.0,
        yst_m: 0.0,
        nwfrzz_m: 0.0,
    }
}

fn frost_fine(
    layer_index: usize,
    fine_index: usize,
) -> openwepp_hillslope_orchestrator::DirectFrostFineLayerState {
    openwepp_hillslope_orchestrator::DirectFrostFineLayerState {
        layer_index,
        fine_index,
        fgfrst: 0.0,
        slfsd_m: 0.0,
        slsic_m: 0.0,
        slsw_theta: 0.0,
        sltime_s: 0.0,
    }
}

fn valid_frost_container() -> openwepp_hillslope_orchestrator::DirectFrostLaneState {
    let mut frost = openwepp_hillslope_orchestrator::DirectFrostLaneState::zero();
    frost.total_fine_layer_count = 3.0;
    frost.layer_shadows = vec![frost_shadow(1), frost_shadow(2)];
    frost.fine_layers = vec![frost_fine(1, 1), frost_fine(1, 2), frost_fine(2, 1)];
    frost
}

fn poison_frost_structure(
    frost: &mut openwepp_hillslope_orchestrator::DirectFrostLaneState,
    poison: usize,
) {
    match poison {
        0 => frost.total_fine_layer_count = f64::NAN,
        1 => frost.total_fine_layer_count = -1.0,
        2 => frost.total_fine_layer_count = 2.5,
        3 => frost.total_fine_layer_count = 2.0,
        4 => frost.layer_shadows[1].layer_index = 1,
        5 => frost.layer_shadows.swap(0, 1),
        6 => frost.layer_shadows[0].layer_index = 0,
        7 => frost.fine_layers[1] = frost.fine_layers[0],
        8 => frost.fine_layers.swap(0, 2),
        9 => frost.fine_layers[1].fine_index = 3,
        10 => frost.fine_layers[2].layer_index = 3,
        11 => frost.fine_layers[0].fine_index = 2,
        12 => {
            frost.layer_shadows.pop();
        }
        _ => unreachable!("complete frost structure poison table"),
    }
}

#[test]
fn frost_container_structure_precedes_unsupported_state_and_carry() {
    for runtime_carry in [false, true] {
        for poison in 0..13 {
            assert_mutation_code(
                move |frame| {
                    let mut frost = valid_frost_container();
                    poison_frost_structure(&mut frost, poison);
                    if runtime_carry {
                        frame.lanes[0].frost_runtime_carry = Some(frost.into());
                    } else {
                        frame.lanes[0].winter_column.frost = frost;
                    }
                },
                DirectSurfaceLiquidErrorCode::E003,
            );
        }
        assert_mutation_code(
            move |frame| {
                let frost = valid_frost_container();
                if runtime_carry {
                    frame.lanes[0].frost_runtime_carry = Some(frost.into());
                } else {
                    frame.lanes[0].winter_column.frost = frost;
                }
            },
            DirectSurfaceLiquidErrorCode::E004,
        );
    }
}

#[test]
fn frost_lane_and_runtime_carry_scalar_domains_precede_e004() {
    for runtime_carry in [false, true] {
        for field in 0..27 {
            assert_mutation_code(
                move |frame| {
                    let mut frost = openwepp_hillslope_orchestrator::DirectFrostLaneState::zero();
                    set_frost_scalar(&mut frost, field, f64::NAN);
                    if runtime_carry {
                        frame.lanes[0].frost_runtime_carry = Some(frost.into());
                    } else {
                        frame.lanes[0].winter_column.frost = frost;
                    }
                },
                DirectSurfaceLiquidErrorCode::E003,
            );
        }
    }
    assert_mutation_code(
        |frame| frame.lanes[0].winter_column.frost.dfrost_m = 0.001,
        DirectSurfaceLiquidErrorCode::E004,
    );
}

#[test]
fn frost_nested_layer_and_fine_layer_domains_precede_e004() {
    for runtime_carry in [false, true] {
        for field in 0..7 {
            assert_mutation_code(
                move |frame| {
                    let mut frost = openwepp_hillslope_orchestrator::DirectFrostLaneState::zero();
                    let mut layer = openwepp_hillslope_orchestrator::DirectFrostLayerShadowState {
                        layer_index: 1,
                        st_m: 0.0,
                        soil_water_m: 0.0,
                        frozen_depth_m: 0.0,
                        frozen_water_m: 0.0,
                        soilf_m: 0.0,
                        yst_m: 0.0,
                        nwfrzz_m: 0.0,
                    };
                    match field {
                        0 => layer.st_m = f64::NAN,
                        1 => layer.soil_water_m = f64::NAN,
                        2 => layer.frozen_depth_m = f64::NAN,
                        3 => layer.frozen_water_m = f64::NAN,
                        4 => layer.soilf_m = f64::NAN,
                        5 => layer.yst_m = f64::NAN,
                        6 => layer.nwfrzz_m = f64::NAN,
                        _ => unreachable!(),
                    }
                    frost.layer_shadows = vec![layer];
                    if runtime_carry {
                        frame.lanes[0].frost_runtime_carry = Some(frost.into());
                    } else {
                        frame.lanes[0].winter_column.frost = frost;
                    }
                },
                DirectSurfaceLiquidErrorCode::E003,
            );
        }
        for field in 0..5 {
            assert_mutation_code(
                move |frame| {
                    let mut frost = openwepp_hillslope_orchestrator::DirectFrostLaneState::zero();
                    let mut fine = openwepp_hillslope_orchestrator::DirectFrostFineLayerState {
                        layer_index: 1,
                        fine_index: 1,
                        fgfrst: 0.0,
                        slfsd_m: 0.0,
                        slsic_m: 0.0,
                        slsw_theta: 0.0,
                        sltime_s: 0.0,
                    };
                    match field {
                        0 => fine.fgfrst = f64::NAN,
                        1 => fine.slfsd_m = f64::NAN,
                        2 => fine.slsic_m = f64::NAN,
                        3 => fine.slsw_theta = f64::NAN,
                        4 => fine.sltime_s = f64::NAN,
                        _ => unreachable!(),
                    }
                    frost.fine_layers = vec![fine];
                    if runtime_carry {
                        frame.lanes[0].frost_runtime_carry = Some(frost.into());
                    } else {
                        frame.lanes[0].winter_column.frost = frost;
                    }
                },
                DirectSurfaceLiquidErrorCode::E003,
            );
        }
    }
    for fine_layer in [false, true] {
        assert_mutation_code(
            move |frame| {
                let mut frost = openwepp_hillslope_orchestrator::DirectFrostLaneState::zero();
                if fine_layer {
                    frost.fine_layers =
                        vec![openwepp_hillslope_orchestrator::DirectFrostFineLayerState {
                            layer_index: 0,
                            fine_index: 1,
                            fgfrst: 0.0,
                            slfsd_m: 0.0,
                            slsic_m: 0.0,
                            slsw_theta: 0.0,
                            sltime_s: 0.0,
                        }];
                } else {
                    frost.layer_shadows = vec![
                        openwepp_hillslope_orchestrator::DirectFrostLayerShadowState {
                            layer_index: 0,
                            st_m: 0.0,
                            soil_water_m: 0.0,
                            frozen_depth_m: 0.0,
                            frozen_water_m: 0.0,
                            soilf_m: 0.0,
                            yst_m: 0.0,
                            nwfrzz_m: 0.0,
                        },
                    ];
                }
                frame.lanes[0].winter_column.frost = frost;
            },
            DirectSurfaceLiquidErrorCode::E003,
        );
    }
}
