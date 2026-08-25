use super::*;

#[test]
fn persistent_operator_requires_stateful_api() {
    let inputs = reconciliation_inputs();
    assert!(
        Wb11HydrologyKernel::compute_direct_snow_liquid_partition_with_evaluation(
            &inputs,
            SnowStage3EvaluationOperator::PersistentAccumulationShadowV1,
        )
        .is_err()
    );
}

#[test]
fn persistent_shadow_accumulates_snow_and_censors_external_liquid() {
    let mut inputs = reconciliation_inputs();
    inputs.snow_layers.clear();
    inputs.runtime_swe_m = 0.0;
    inputs.runtime_depth_m = 0.0;
    inputs.runtime_density_kg_m3 = 0.0;
    inputs.hourly = [DirectSnowHourlyForcing {
        air_temperature_c: -5.0,
        ..DirectSnowHourlyForcing::zero()
    }; 24];
    inputs.hourly[0].snowfall_m = 0.02;
    inputs.hourly[0].hydrometeor_temperature_c = Some(-5.0);
    inputs.hourly[1].rain_m = 0.003;
    inputs.hourly[1].hydrometeor_temperature_c = Some(-5.0);
    let state = Wb11HydrologyKernel::initialize_stage3_persistent_state(7, Vec::new())
        .expect("valid empty carry");
    let mut dormant_inputs = inputs.clone();
    dormant_inputs.hourly[0].snowfall_m = 0.0;
    dormant_inputs.hourly[1].rain_m = 0.0;
    let dormant =
        Wb11HydrologyKernel::evaluate_stage3_persistent_day(&dormant_inputs, &state, 7, 0)
            .expect("dormant day");
    assert_eq!(dormant.lifecycle, "dormant");
    let day = Wb11HydrologyKernel::evaluate_stage3_persistent_day(&inputs, &dormant.state, 7, 1)
        .expect("persistent day");
    assert_eq!(day.lifecycle, "reappeared");
    assert_eq!(day.snowfall_kg_m2.to_bits(), 2.0_f64.to_bits());
    assert_eq!(day.external_liquid_kg_m2.to_bits(), 3.0_f64.to_bits());
    assert!(
        (day.external_liquid_kg_m2 + day.melt_kg_m2 + day.start_retained_liquid_kg_m2
            - day.refrozen_kg_m2
            - day.end_retained_liquid_kg_m2
            - day.unresolved_liquid_kg_m2)
            .abs()
            <= 1.0e-12
    );
    assert!(day.end_ice_kg_m2 > 0.0);
    assert!(day.ice_mass_closure_residual_kg_m2.abs() <= 1.0e-9);
    assert_eq!(day.state.next_interval_index, 2);
}

#[test]
fn persistent_shadow_restore_is_exact_and_order_is_fail_closed() {
    let inputs = reconciliation_inputs();
    let mut nonfinite_layer = inputs.snow_layers[0];
    nonfinite_layer.settle_day_count = f64::NAN;
    assert!(
        Wb11HydrologyKernel::initialize_stage3_persistent_state(3, vec![nonfinite_layer],).is_err()
    );
    let initial =
        Wb11HydrologyKernel::initialize_stage3_persistent_state(3, inputs.snow_layers.clone())
            .expect("valid initial carry");
    let first = Wb11HydrologyKernel::evaluate_stage3_persistent_day(&inputs, &initial, 3, 0)
        .expect("first day");
    let snapshot = Wb11HydrologyKernel::serialize_stage3_persistent_state(&first.state)
        .expect("serialize valid snapshot");
    let restored = Wb11HydrologyKernel::restore_stage3_persistent_state_json(&snapshot, 3, 1)
        .expect("valid fingerprint-bound restore");
    let uninterrupted =
        Wb11HydrologyKernel::evaluate_stage3_persistent_day(&inputs, &first.state, 3, 1)
            .expect("uninterrupted second day");
    let resumed = Wb11HydrologyKernel::evaluate_stage3_persistent_day(&inputs, &restored, 3, 1)
        .expect("restored second day");
    assert_eq!(uninterrupted, resumed);
    assert!(
        Wb11HydrologyKernel::evaluate_stage3_persistent_day(&inputs, &restored, 4, 1,).is_err()
    );
    assert!(
        Wb11HydrologyKernel::evaluate_stage3_persistent_day(&inputs, &restored, 3, 0,).is_err()
    );
    let mut corrupted = restored.clone();
    corrupted.cumulative_snowfall_kg_m2 += 1.0;
    assert!(Wb11HydrologyKernel::restore_stage3_persistent_state(corrupted, 3, 1,).is_err());
    assert_eq!(first.state, restored);
    let mut unknown = snapshot.clone();
    unknown.pop();
    unknown.extend_from_slice(b",\"unknown\":1}");
    assert!(Wb11HydrologyKernel::restore_stage3_persistent_state_json(&unknown, 3, 1,).is_err());
}

#[test]
fn persistent_shadow_disappears_dorms_and_reappears() {
    let mut inputs = reconciliation_inputs();
    inputs.snow_layers[0].mass_swe_m = 0.001_1;
    inputs.snow_layers[0].thickness_m = 0.002_2;
    inputs.snow_layers[0].cold_content_j_m2 = 0.0;
    inputs.hourly = [DirectSnowHourlyForcing {
        radiation_mj_m2: 1_000.0,
        air_temperature_c: 0.0,
        ..DirectSnowHourlyForcing::zero()
    }; 24];
    inputs.surface_energy_options.daily_solar_radiation_mj_m2 = 48.0;
    let initial =
        Wb11HydrologyKernel::initialize_stage3_persistent_state(9, inputs.snow_layers.clone())
            .unwrap();
    let disappeared =
        Wb11HydrologyKernel::evaluate_stage3_persistent_day(&inputs, &initial, 9, 0).unwrap();
    assert_eq!(disappeared.lifecycle, "disappeared");

    let mut dormant_inputs = inputs.clone();
    dormant_inputs.hourly = [DirectSnowHourlyForcing::zero(); 24];
    let dormant = Wb11HydrologyKernel::evaluate_stage3_persistent_day(
        &dormant_inputs,
        &disappeared.state,
        9,
        1,
    )
    .unwrap();
    assert_eq!(dormant.lifecycle, "dormant");

    dormant_inputs.hourly[0] = DirectSnowHourlyForcing {
        snowfall_m: 0.02,
        air_temperature_c: -5.0,
        hydrometeor_temperature_c: Some(-5.0),
        ..DirectSnowHourlyForcing::zero()
    };
    let reappeared =
        Wb11HydrologyKernel::evaluate_stage3_persistent_day(&dormant_inputs, &dormant.state, 9, 2)
            .unwrap();
    assert_eq!(reappeared.lifecycle, "reappeared");
}

#[test]
fn terminal_event_request_is_state_bound_and_censors_remaining_time() {
    let mut inputs = reconciliation_inputs();
    inputs.snow_layers.truncate(1);
    inputs.snow_layers[0].mass_swe_m = 0.000_6;
    inputs.snow_layers[0].thickness_m = 0.001_2;
    inputs.snow_layers[0].cold_content_j_m2 = 0.0;
    inputs.snow_layers[0].temperature_c = 0.0;
    inputs.hourly = [DirectSnowHourlyForcing {
        radiation_mj_m2: 1_000.0,
        air_temperature_c: 0.0,
        ..DirectSnowHourlyForcing::zero()
    }; 24];
    inputs.surface_energy_options.daily_solar_radiation_mj_m2 = 48.0;
    let ordinary =
        Wb11HydrologyKernel::initialize_stage3_persistent_state(12, inputs.snow_layers.clone())
            .unwrap();
    assert!(
        Wb11HydrologyKernel::evaluate_stage3_persistent_day_with_terminal_event(
            &inputs,
            &ordinary,
            12,
            0,
            DirectSnowTerminalEventRequest::ENTHALPY_EVENT_V1,
        )
        .is_err()
    );
    let terminal = Wb11HydrologyKernel::initialize_stage3_persistent_state_with_terminal_event(
        12,
        inputs.snow_layers.clone(),
        DirectSnowTerminalEventRequest::ENTHALPY_EVENT_V1,
    )
    .unwrap();
    assert!(
        Wb11HydrologyKernel::evaluate_stage3_persistent_day(&inputs, &terminal, 12, 0,).is_err()
    );
    let day = Wb11HydrologyKernel::evaluate_stage3_persistent_day_with_terminal_event(
        &inputs,
        &terminal,
        12,
        0,
        DirectSnowTerminalEventRequest::ENTHALPY_EVENT_V1,
    )
    .unwrap();
    let event = day.terminal_event.expect("localized exhaustion event");
    assert!(event.event_occurred);
    assert!(event.evaluated_seconds > 0.0);
    assert!(event.unevaluated_seconds > 0.0);
    assert!(day.end_ice_kg_m2.abs() <= f64::EPSILON);
    assert!(day.state.layers.is_empty());
    assert!(event.solid_mass_closure_residual_kg_m2.abs() <= 1.0e-9);
    assert!(event.liquid_mass_closure_residual_kg_m2.abs() <= 1.0e-9);
    assert!(event.energy_closure_residual_j_m2.abs() <= 1.0e-6);
}

#[test]
fn persistent_support_evaluator_runs_one_admitted_parent_support() {
    let mut inputs = reconciliation_inputs();
    inputs.snow_layers[0].mass_swe_m = 0.000_6;
    inputs.snow_layers[0].thickness_m = 0.001_2;
    inputs.snow_layers[0].cold_content_j_m2 = 0.0;
    inputs.snow_layers[0].temperature_c = 0.0;
    inputs.hourly = [DirectSnowHourlyForcing {
        radiation_mj_m2: 1_000.0,
        air_temperature_c: 0.0,
        ..DirectSnowHourlyForcing::zero()
    }; 24];
    inputs.surface_energy_options.daily_solar_radiation_mj_m2 = 48.0;
    let state = Wb11HydrologyKernel::initialize_stage3_persistent_state_with_terminal_event(
        21,
        inputs.snow_layers.clone(),
        DirectSnowTerminalEventRequest::ENTHALPY_EVENT_V1,
    )
    .expect("valid support state");
    let support = DirectSnowStage3SupportInput {
        forcing: inputs.hourly[0],
        duration_seconds: 1_800.0,
    };
    let result = Wb11HydrologyKernel::evaluate_stage3_persistent_support(
        &inputs,
        &state,
        21,
        0,
        support,
        DirectSnowTerminalEventRequest::ENTHALPY_EVENT_V1,
    )
    .expect("one parent support must use the actual terminal solver");
    let event = result.terminal_event.expect("terminal result is retained");
    assert!(event.event_occurred);
    assert_eq!(result.evaluation.requested_seconds, 1_800.0);
    assert_eq!(event.requested_seconds, 1_800.0);
    assert!(event.evaluated_seconds > 0.0);
    assert!(event.unevaluated_seconds > 0.0);
    assert_eq!(result.state.next_interval_index, 1);
}

#[test]
fn terminal_no_event_refreeze_closes_persistent_day() {
    let mut inputs = reconciliation_inputs();
    inputs.snow_layers.truncate(1);
    inputs.snow_layers[0].mass_swe_m = 0.000_6;
    inputs.snow_layers[0].thickness_m = 0.001_2;
    inputs.snow_layers[0].liquid_water_m = 0.000_1;
    inputs.snow_layers[0].cold_content_j_m2 = 50_000.0;
    inputs.snow_layers[0].temperature_c = -5.0;
    inputs.hourly = [DirectSnowHourlyForcing {
        air_temperature_c: -10.0,
        ..DirectSnowHourlyForcing::zero()
    }; 24];
    let initial = Wb11HydrologyKernel::initialize_stage3_persistent_state_with_terminal_event(
        14,
        inputs.snow_layers.clone(),
        DirectSnowTerminalEventRequest::ENTHALPY_EVENT_V1,
    )
    .unwrap();
    let day = Wb11HydrologyKernel::evaluate_stage3_persistent_day_with_terminal_event(
        &inputs,
        &initial,
        14,
        0,
        DirectSnowTerminalEventRequest::ENTHALPY_EVENT_V1,
    )
    .unwrap();
    assert!(day.refrozen_kg_m2 > 0.0);
    assert!(day.ice_mass_closure_residual_kg_m2.abs() <= 1.0e-12);
    assert!(day.total_water_closure_residual_kg_m2.abs() <= 1.0e-12);
    assert!(!day.terminal_intervals.is_empty());
    assert!(!day.terminal_intervals.last().unwrap().event_occurred);
}

#[test]
fn persistent_cold_rain_on_snow_refreezes_and_closes_linked_ledgers() {
    let mut inputs = reconciliation_inputs();
    inputs.snow_layers.truncate(1);
    inputs.snow_layers[0].mass_swe_m = 0.05;
    inputs.snow_layers[0].thickness_m = 0.10;
    inputs.snow_layers[0].cold_content_j_m2 = 1_000_000.0;
    inputs.snow_layers[0].temperature_c = -10.0;
    inputs.hourly = [DirectSnowHourlyForcing {
        air_temperature_c: -10.0,
        ..DirectSnowHourlyForcing::zero()
    }; 24];
    inputs.hourly[0].rain_m = 0.001;
    inputs.hourly[0].hydrometeor_temperature_c = Some(-5.0);
    let initial =
        Wb11HydrologyKernel::initialize_stage3_persistent_state(31, inputs.snow_layers.clone())
            .expect("valid cold persistent snow");

    let day = Wb11HydrologyKernel::evaluate_stage3_persistent_day(&inputs, &initial, 31, 0)
        .expect("cold rain-on-snow must use the persistent liquid disposition");

    assert_eq!(day.external_liquid_kg_m2.to_bits(), 1.0_f64.to_bits());
    assert!(day.refrozen_kg_m2 > 0.0, "{day:?}");
    assert!(day.end_ice_kg_m2 > day.start_ice_kg_m2);
    assert!(day.ice_mass_closure_residual_kg_m2.abs() <= 1.0e-9);
    assert!(day.total_water_closure_residual_kg_m2.abs() <= 1.0e-9);
}

#[test]
fn persistent_warm_rain_on_isothermal_snow_does_not_refreeze() {
    let mut layers = vec![DirectSnowLayerState {
        mass_swe_m: 0.05,
        thickness_m: 0.10,
        density_kg_m3: 500.0,
        settle_day_count: 1.0,
        temperature_c: 0.0,
        liquid_water_m: 0.0,
        cold_content_j_m2: 0.0,
        refrozen_liquid_m: 0.0,
    }];
    let mut cold_content = vec![0.0];
    let beginning_ice_kg_m2 = layers[0].mass_swe_m * STAGE3_RHO_WATER_KG_M3;
    let (routed, retained, refrozen) =
        Wb11HydrologyKernel::route_stage3_persistent_liquid_through_layers(
            HillslopeKernelPhaseClass::HydrologyRunoffReconciliation,
            1.0,
            &mut layers,
            &mut cold_content,
        )
        .expect("isothermal snow must disposition warm rain without refreezing");

    assert_eq!(refrozen.to_bits(), 0.0_f64.to_bits());
    assert_eq!(
        (layers[0].mass_swe_m * STAGE3_RHO_WATER_KG_M3).to_bits(),
        beginning_ice_kg_m2.to_bits()
    );
    assert!((1.0 - routed - retained).abs() <= 1.0e-12);
}

#[test]
fn persistent_rain_disposition_failure_preserves_beginning_owner() {
    let mut inputs = reconciliation_inputs();
    inputs.hourly[0].rain_m = f64::NAN;
    let beginning =
        Wb11HydrologyKernel::initialize_stage3_persistent_state(33, inputs.snow_layers.clone())
            .expect("valid beginning owner");
    let immutable_snapshot = beginning.clone();

    assert!(
        Wb11HydrologyKernel::evaluate_stage3_persistent_day(&inputs, &beginning, 33, 0).is_err()
    );
    assert_eq!(beginning, immutable_snapshot);
}
