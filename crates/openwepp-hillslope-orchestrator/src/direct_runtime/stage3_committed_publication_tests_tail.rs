#[test]
fn terminal_liquid_canonicalizes_only_the_exact_one_ulp_phase_reference() {
    let reference = 273.15_f64;
    let next_up = f64::from_bits(reference.to_bits() + 1);
    let two_up = f64::from_bits(reference.to_bits() + 2);
    assert_eq!(
        canonical_stage3_terminal_liquid_temperature_k(reference).to_bits(),
        reference.to_bits()
    );
    assert_eq!(
        canonical_stage3_terminal_liquid_temperature_k(next_up).to_bits(),
        reference.to_bits()
    );
    assert_eq!(
        canonical_stage3_terminal_liquid_temperature_k(two_up).to_bits(),
        two_up.to_bits()
    );
    assert_eq!(
        canonical_stage3_terminal_liquid_temperature_k(f64::from_bits(reference.to_bits() - 1))
            .to_bits(),
        reference.to_bits() - 1
    );
}

#[test]
fn mixed_terminal_custody_temperature_does_not_replace_meltwater_phase_reference() {
    let routed_liquid_m = 1.00650874615698643e-2;
    let mixed_temperature_k = 273.16522205099076_f64;
    let temperature_mass = routed_liquid_m * mixed_temperature_k;

    let published = accepted_meltwater_phase_reference_c(routed_liquid_m, temperature_mass)
        .expect("valid mixed terminal custody")
        .expect("active routed-liquid phase reference");

    assert_eq!(published.as_celsius().to_bits(), 0.0_f64.to_bits());
    assert_eq!(
        (temperature_mass / routed_liquid_m).to_bits(),
        mixed_temperature_k.to_bits(),
        "publication must not mutate or discard the independently retained bulk custody temperature",
    );
}

#[test]
fn meltwater_phase_reference_rejects_omitted_or_substituted_custody_temperature() {
    let active = crate::constants::WB11_ZERO_THRESHOLD * 2_000.0;
    assert!(accepted_meltwater_phase_reference_c(active, 0.0).is_err());
    assert!(accepted_meltwater_phase_reference_c(active, f64::NAN).is_err());
    assert!(accepted_meltwater_phase_reference_c(active, active * 199.0).is_err());
    assert!(accepted_meltwater_phase_reference_c(active, active * 351.0).is_err());
    assert!(accepted_meltwater_phase_reference_c(0.0, 273.15).is_err());
}

#[test]
fn meltwater_phase_reference_preserves_wb11_activity_threshold() {
    let subthreshold = crate::constants::WB11_ZERO_THRESHOLD / 4.0;
    assert_eq!(
        accepted_meltwater_phase_reference_c(subthreshold, subthreshold * 273.16)
            .expect("valid subthreshold custody"),
        None,
    );

    let active = f64::from_bits(crate::constants::WB11_ZERO_THRESHOLD.to_bits() + 1);
    let published = accepted_meltwater_phase_reference_c(active, active * 273.16)
        .expect("valid active custody")
        .expect("active phase reference");
    assert_eq!(published.as_celsius().to_bits(), 0.0_f64.to_bits());
}

#[test]
fn wb11_outcome_threshold_does_not_alias_exact_subthreshold_receiver_custody() {
    let subthreshold = crate::constants::WB11_ZERO_THRESHOLD / 4.0;
    let solid = DirectSnowSolidToLiquidLedger {
        raw_signed_melt_m: subthreshold,
        redistributed_positive_melt_m: subthreshold,
        snowpack_swe_loss_m: subthreshold,
        rain_released_m: 0.0,
        liquid_handoff_m: subthreshold,
    };
    let disposition = DirectSnowLiquidDispositionLedger {
        incoming_liquid_m: subthreshold,
        routed_liquid_m: subthreshold,
        retained_liquid_delta_m: 0.0,
        refrozen_liquid_m: 0.0,
        liquid_closure_residual_m: 0.0,
    };
    DirectSnowMassTransitionLedgers::try_from_parts(
        solid,
        disposition,
        DirectSnowStage3Outcome {
            enabled: true,
            meltwater_temperature_c: None,
            sublimation_m: 0.0,
        },
    )
    .expect("exact subthreshold custody is inactive only on the WB11 outcome surface");

    let temperature = TemperatureCelsius::try_new(0.0).expect("freezing temperature");
    assert!(
        DirectSnowMassTransitionLedgers::try_from_parts(
            solid,
            disposition,
            DirectSnowStage3Outcome {
                enabled: true,
                meltwater_temperature_c: Some(temperature),
                sublimation_m: 0.0,
            },
        )
        .is_err(),
        "subthreshold receiver custody cannot be aliased to an active WB11 outcome",
    );

    let active = crate::constants::WB11_ZERO_THRESHOLD * 2_000.0;
    let active_solid = DirectSnowSolidToLiquidLedger {
        raw_signed_melt_m: active,
        redistributed_positive_melt_m: active,
        snowpack_swe_loss_m: active,
        rain_released_m: 0.0,
        liquid_handoff_m: active,
    };
    let active_disposition = DirectSnowLiquidDispositionLedger {
        incoming_liquid_m: active,
        routed_liquid_m: active,
        retained_liquid_delta_m: 0.0,
        refrozen_liquid_m: 0.0,
        liquid_closure_residual_m: 0.0,
    };
    assert!(
        DirectSnowMassTransitionLedgers::try_from_parts(
            active_solid,
            active_disposition,
            DirectSnowStage3Outcome {
                enabled: true,
                meltwater_temperature_c: None,
                sublimation_m: 0.0,
            },
        )
        .is_err(),
        "active routed liquid cannot omit its WB11 outcome temperature",
    );

    let mut double_count = active_disposition;
    double_count.routed_liquid_m *= 2.0;
    assert!(
        DirectSnowMassTransitionLedgers::try_from_parts(
            active_solid,
            double_count,
            DirectSnowStage3Outcome {
                enabled: true,
                meltwater_temperature_c: Some(temperature),
                sublimation_m: 0.0,
            },
        )
        .is_err(),
        "receiver custody cannot be counted twice",
    );
}

#[test]
fn support_liquid_publication_uses_exact_capacity_dispositions_and_rejects_poisons() {
    use crate::direct_runtime::{
        DirectZeroDurationSnowLiquidDispositionV1, DirectZeroDurationSnowLiquidInputV1,
    };
    use openwepp_coupled_time::{ModelTimeNs, TimeSupport};

    let configuration = crate::direct_runtime::surface_liquid_owner::tests::configuration();
    let beginning =
        crate::direct_runtime::surface_liquid_owner::tests::accepted_state(&configuration);
    let output_receipt_sha256 = [11; 32];
    let output_set_sha256 = [12; 32];
    let predecessor_owner_set_sha256 = [13; 32];
    let receiver_context_sha256 = [14; 32];
    let support =
        TimeSupport::new(ModelTimeNs::new(0), ModelTimeNs::new(60_000_000_000)).expect("support");
    let inputs = configuration
        .records
        .iter()
        .map(|record| DirectZeroDurationSnowLiquidInputV1 {
            output_receipt_sha256,
            output_set_sha256,
            predecessor_owner_set_sha256,
            receiver_context_sha256,
            support_start_ns: support.start_ns().get(),
            support_end_ns: support.end_ns().get(),
            receiver_ordinal: 0,
            ofe_id: record.key.ofe_id.clone(),
            tile_id: record.key.tile_id.clone(),
            tile_fraction: record.tile_fraction,
            // Beginning storage is 1 kg/m2 tile and capacity is 2, so this
            // exact input retains half and routes half to the outlet.
            mass_kg_m2_tile_ground: 2.0,
            sensible_enthalpy_j_m2_tile_ground: 200.0,
        })
        .collect::<Vec<_>>();
    let outcome = beginning
        .accept_zero_duration_snow_liquid_outputs_v1(&configuration, &inputs, false)
        .expect("capacity-limited receiver transaction");
    assert!(outcome.receipts.iter().any(|receipt| {
        receipt.disposition == DirectZeroDurationSnowLiquidDispositionV1::RetainedSurface
    }));
    assert!(outcome.receipts.iter().any(|receipt| {
        receipt.disposition == DirectZeroDurationSnowLiquidDispositionV1::OutletRunoff
    }));

    let mut accepted = AcceptedLaneDay::default();
    project_support_liquid_receipts_to_lane_day(
        &mut accepted,
        &outcome.receipts,
        Digest32::from_bytes(outcome.receipt_set_sha256),
        &configuration.ofe_bindings[0],
        &configuration,
        &beginning,
        &outcome.ending_state,
        0,
        support,
    )
    .expect("typed capacity dispositions publish");
    assert!((accepted.ingress_m - 0.002).abs() < 1.0e-15);
    assert!((accepted.retained_surface_liquid_m - 0.001).abs() < 1.0e-15);
    assert!((accepted.runoff_m - 0.001).abs() < 1.0e-15);
    assert_eq!(accepted.runon_m.to_bits(), 0.0_f64.to_bits());
    assert!((accepted.hourly_runoff_m.iter().sum::<f64>() - 0.001).abs() < 1.0e-15);

    let mut omission = outcome.receipts.clone();
    omission.pop();
    assert!(
        project_support_liquid_receipts_to_lane_day(
            &mut AcceptedLaneDay::default(),
            &omission,
            Digest32::from_bytes(outcome.receipt_set_sha256),
            &configuration.ofe_bindings[0],
            &configuration,
            &beginning,
            &outcome.ending_state,
            0,
            support,
        )
        .is_err(),
        "receipt omission must reject",
    );

    let mut order = outcome.receipts.clone();
    order.swap(0, 1);
    assert!(
        project_support_liquid_receipts_to_lane_day(
            &mut AcceptedLaneDay::default(),
            &order,
            Digest32::from_bytes(outcome.receipt_set_sha256),
            &configuration.ofe_bindings[0],
            &configuration,
            &beginning,
            &outcome.ending_state,
            0,
            support,
        )
        .is_err(),
        "receipt order substitution must reject",
    );

    let mut disposition = outcome.receipts.clone();
    disposition[0].disposition = DirectZeroDurationSnowLiquidDispositionV1::OutletRunoff;
    assert!(
        project_support_liquid_receipts_to_lane_day(
            &mut AcceptedLaneDay::default(),
            &disposition,
            Digest32::from_bytes(outcome.receipt_set_sha256),
            &configuration.ofe_bindings[0],
            &configuration,
            &beginning,
            &outcome.ending_state,
            0,
            support,
        )
        .is_err(),
        "disposition substitution must reject",
    );

    assert!(
        project_support_liquid_receipts_to_lane_day(
            &mut AcceptedLaneDay::default(),
            &outcome.receipts,
            Digest32::from_bytes([99; 32]),
            &configuration.ofe_bindings[0],
            &configuration,
            &beginning,
            &outcome.ending_state,
            0,
            support,
        )
        .is_err(),
        "receipt-set substitution must reject",
    );

    assert!(
        project_support_liquid_receipts_to_lane_day(
            &mut AcceptedLaneDay::default(),
            &outcome.receipts,
            Digest32::from_bytes(outcome.receipt_set_sha256),
            &configuration.ofe_bindings[0],
            &configuration,
            &beginning,
            &beginning,
            0,
            support,
        )
        .is_err(),
        "ending surface substitution must reject",
    );
}

#[test]
fn support_liquid_publication_routes_source_runoff_and_destination_runon_once() {
    use crate::direct_runtime::{
        DirectGroundIngressMode, DirectSurfaceLiquidConfiguration,
        DirectSurfaceLiquidConfigurationRecord, DirectSurfaceLiquidOfeBinding,
        DirectSurfaceLiquidStoreKey, DirectZeroDurationSnowLiquidDispositionV1,
        DirectZeroDurationSnowLiquidInputV1,
    };
    use openwepp_coupled_time::{ModelTimeNs, TimeSupport};
    use openwepp_kernel_contract::{ResourceOwnerId, SoilLayerId, TileId};
    use openwepp_land_surface_energy::{SourceId, SurfaceId};

    let make_binding = |name: &str, lane_index: usize| {
        let top = SoilLayerId::try_new(format!("{name}-top")).expect("soil layer");
        DirectSurfaceLiquidOfeBinding {
            ofe_id: OfeId::try_new(name).expect("OFE"),
            production_lane_index: lane_index,
            production_lane_id: u32::try_from(lane_index + 1).expect("lane"),
            ordered_soil_layer_ids: vec![
                top.clone(),
                SoilLayerId::try_new(format!("{name}-bottom")).expect("soil layer"),
            ],
            infiltration_soil_thermal_layer_id: top,
        }
    };
    let make_record = |name: &str, area: f64, capacity: f64, route: Option<(&str, &str)>| {
        DirectSurfaceLiquidConfigurationRecord {
            key: DirectSurfaceLiquidStoreKey {
                run_id: 72,
                ofe_id: OfeId::try_new(name).expect("OFE"),
                tile_id: TileId::try_new(format!("{name}-tile")).expect("tile"),
                surface_id: SurfaceId::try_new(format!("{name}-surface")).expect("surface"),
                surface_class: SurfaceClass::BareMineralSoil,
                source_type: WaterSourceType::SurfaceLiquid,
                source_id: SourceId::try_new(format!("{name}-source")).expect("source"),
            },
            tile_fraction: 1.0,
            capacity_kg_m2_tile: capacity,
            ofe_area_m2: area,
            ground_ingress_mode: DirectGroundIngressMode::OpenRawPrecipitation,
            runon_destination_ofe_id: route.map(|(ofe, _)| OfeId::try_new(ofe).expect("route OFE")),
            runon_destination_tile_id: route
                .map(|(_, tile)| TileId::try_new(tile).expect("route tile")),
        }
    };
    let configuration = DirectSurfaceLiquidConfiguration::new(
        ResourceOwnerId::try_new("hydrology").expect("owner"),
        72,
        vec![
            OfeId::try_new("upper").expect("OFE"),
            OfeId::try_new("lower").expect("OFE"),
        ],
        vec![make_binding("upper", 0), make_binding("lower", 1)],
        vec![
            make_record("upper", 100.0, 1.0, Some(("lower", "lower-tile"))),
            make_record("lower", 200.0, 10.0, None),
        ],
    )
    .expect("routed configuration");
    let upper = &configuration.ofe_bindings[0];
    let lower = &configuration.ofe_bindings[1];
    let initial = configuration
        .records
        .iter()
        .map(|record| {
            let liquid = if record.key.ofe_id == upper.ofe_id {
                record.capacity_kg_m2_tile
            } else {
                0.0
            };
            (record.key.clone(), liquid)
        })
        .collect::<BTreeMap<_, _>>();
    let beginning = DirectSurfaceLiquidOwnedState::new_initial(&configuration, &initial, 0)
        .expect("capacity-pressure routed beginning");
    let support =
        TimeSupport::new(ModelTimeNs::new(0), ModelTimeNs::new(60_000_000_000)).expect("support");
    let inputs = configuration
        .records
        .iter()
        .filter(|record| record.key.ofe_id == upper.ofe_id)
        .map(|record| DirectZeroDurationSnowLiquidInputV1 {
            output_receipt_sha256: [21; 32],
            output_set_sha256: [22; 32],
            predecessor_owner_set_sha256: [23; 32],
            receiver_context_sha256: [24; 32],
            support_start_ns: support.start_ns().get(),
            support_end_ns: support.end_ns().get(),
            receiver_ordinal: 0,
            ofe_id: record.key.ofe_id.clone(),
            tile_id: record.key.tile_id.clone(),
            tile_fraction: record.tile_fraction,
            mass_kg_m2_tile_ground: 1.0,
            sensible_enthalpy_j_m2_tile_ground: 100.0,
        })
        .collect::<Vec<_>>();
    let outcome = beginning
        .accept_zero_duration_snow_liquid_outputs_v1(&configuration, &inputs, false)
        .expect("routed receiver transaction");
    assert!(outcome.receipts.iter().any(|receipt| {
        receipt.basis_ofe_id == upper.ofe_id
            && receipt.disposition == DirectZeroDurationSnowLiquidDispositionV1::RoutedRunoff
    }));
    assert!(outcome.receipts.iter().any(|receipt| {
        receipt.basis_ofe_id == lower.ofe_id
            && receipt.origin_ofe_id == upper.ofe_id
            && matches!(
                receipt.disposition,
                DirectZeroDurationSnowLiquidDispositionV1::RetainedSurface
                    | DirectZeroDurationSnowLiquidDispositionV1::OutletRunoff
            )
    }));

    let receipt_set = Digest32::from_bytes(outcome.receipt_set_sha256);
    let mut upper_day = AcceptedLaneDay::default();
    project_support_liquid_receipts_to_lane_day(
        &mut upper_day,
        &outcome.receipts,
        receipt_set,
        upper,
        &configuration,
        &beginning,
        &outcome.ending_state,
        0,
        support,
    )
    .expect("upper routed publication");
    assert!((upper_day.ingress_m - 0.001).abs() < 1.0e-15);
    assert!((upper_day.runoff_m - 0.001).abs() < 1.0e-15);
    assert_eq!(upper_day.runon_m.to_bits(), 0.0_f64.to_bits());

    let mut lower_day = AcceptedLaneDay::default();
    project_support_liquid_receipts_to_lane_day(
        &mut lower_day,
        &outcome.receipts,
        receipt_set,
        lower,
        &configuration,
        &beginning,
        &outcome.ending_state,
        0,
        support,
    )
    .expect("lower routed publication");
    // The routed fixture has a 100/200 m2 upper/lower area ratio.
    assert!((lower_day.ingress_m - 0.0005).abs() < 1.0e-15);
    assert!((lower_day.runon_m - 0.0005).abs() < 1.0e-15);
    assert!((lower_day.support_liquid_runon_m - 0.0005).abs() < 1.0e-15);
    assert!(
        (lower_day.retained_surface_liquid_m + lower_day.runoff_m - lower_day.ingress_m).abs()
            < 1.0e-15
    );

    let source_enthalpy = outcome
        .receipts
        .iter()
        .filter(|receipt| {
            receipt.basis_ofe_id == upper.ofe_id
                && receipt.disposition == DirectZeroDurationSnowLiquidDispositionV1::RoutedRunoff
        })
        .map(|receipt| receipt.sensible_enthalpy_j_m2_basis_ofe_ground)
        .sum::<f64>();
    let destination_enthalpy = outcome
        .receipts
        .iter()
        .filter(|receipt| receipt.basis_ofe_id == lower.ofe_id)
        .map(|receipt| receipt.sensible_enthalpy_j_m2_basis_ofe_ground)
        .sum::<f64>();
    assert!((destination_enthalpy - source_enthalpy * 0.5).abs() < 1.0e-12);

    let mut enthalpy_substitution = outcome.receipts.clone();
    enthalpy_substitution
        .iter_mut()
        .find(|receipt| receipt.basis_ofe_id == lower.ofe_id)
        .expect("destination receipt")
        .sensible_enthalpy_j_m2_basis_ofe_ground += 1.0;
    assert!(
        project_support_liquid_receipts_to_lane_day(
            &mut AcceptedLaneDay::default(),
            &enthalpy_substitution,
            receipt_set,
            lower,
            &configuration,
            &beginning,
            &outcome.ending_state,
            0,
            support,
        )
        .is_err(),
        "destination enthalpy substitution must reject",
    );
}
