//! Public canonical-context poisons for the surface-liquid ingress boundary.

use openwepp_kernel_contract::{TileId, TransactionId};
use openwepp_land_surface_energy::{SourceId, SurfaceId};

use super::tests::{
    initial_state, one_tile_configuration, open_ingress, parameters, resource_candidate,
};
use super::{
    DirectGroundIngressMode, DirectOfeWb14Parameters, DirectSurfaceLiquidConfiguration,
    DirectSurfaceLiquidErrorCode, DirectSurfaceLiquidIngressInput, DirectSurfaceLiquidPhase,
    DirectSurfaceLiquidReceiptDisposition, DirectTileGroundIngress, INTERVAL_S,
    WATER_DENSITY_KG_M3, execute_surface_liquid_ingress, liquid_specific_enthalpy,
};

fn five_window_configuration() -> DirectSurfaceLiquidConfiguration {
    let base = one_tile_configuration(DirectGroundIngressMode::OpenRawPrecipitation);
    let records = (0..5)
        .map(|index| {
            let mut record = base.records[0].clone();
            record.key.tile_id = TileId::try_new(format!("tile-{index}")).expect("tile");
            record.key.surface_id =
                SurfaceId::try_new(format!("surface-tile-{index}")).expect("surface");
            record.key.source_id =
                SourceId::try_new(format!("source-tile-{index}")).expect("source");
            record.tile_fraction = 0.2;
            record.capacity_kg_m2_tile = 1.0;
            record
        })
        .collect();
    DirectSurfaceLiquidConfiguration::new(
        base.owner_id,
        base.run_id,
        base.ofe_topology,
        base.ofe_bindings,
        records,
    )
    .expect("five-window configuration")
}

#[test]
fn candidate_validation_preflights_public_schema_and_identity_before_closure_arithmetic() {
    let configuration = one_tile_configuration(DirectGroundIngressMode::OpenRawPrecipitation);
    let beginning = initial_state(&configuration, 0.0);
    let transaction_id = TransactionId(4_140);
    let resource = resource_candidate(&configuration, &beginning, transaction_id, None, &[]);
    let input = DirectSurfaceLiquidIngressInput {
        transaction_id,
        day_index: 3,
        interval_index: 0,
        interval_s: INTERVAL_S,
        tile_ingress: vec![open_ingress(&configuration.records[0], 0.1)],
        wb14_parameters: parameters(&configuration),
    };
    let mut candidate =
        execute_surface_liquid_ingress(&configuration, &resource, &input).expect("valid candidate");
    candidate
        .closure_operands
        .poison_first_store_arithmetic_overflow_for_test();

    let mut wrong_transaction = input.clone();
    wrong_transaction.transaction_id = TransactionId(4_141);
    let error = candidate
        .validate(&configuration, &resource, &wrong_transaction)
        .expect_err("identity preflight must precede closure arithmetic");
    let failure = error.failure().expect("canonical identity failure");
    assert_eq!(failure.code, DirectSurfaceLiquidErrorCode::E002);
    assert_eq!(failure.phase, DirectSurfaceLiquidPhase::IngressCandidate);
    assert!(failure.rollback.beginning_owner_sha256.is_some());
    assert!(failure.rollback.attempted_owner_sha256.is_some());

    let mut empty_configuration = configuration.clone();
    empty_configuration.records.clear();
    let error = candidate
        .validate(&empty_configuration, &resource, &input)
        .expect_err("schema preflight must precede closure arithmetic");
    assert_eq!(error.code(), DirectSurfaceLiquidErrorCode::E001);
}

#[test]
fn five_window_temporal_mass_uses_exact_parent_remainder_and_rejects_one_ulp_loss() {
    let configuration = five_window_configuration();
    let beginning = initial_state(&configuration, 0.0);
    let transaction_id = TransactionId(4_142);
    let resource = resource_candidate(&configuration, &beginning, transaction_id, None, &[]);
    let mut ingress = configuration
        .records
        .iter()
        .enumerate()
        .map(|(index, record)| open_ingress(record, if index == 0 { 0.5 } else { 0.0 }))
        .collect::<Vec<_>>();
    let support_windows = [
        (360.0, 720.0),
        (720.0, 1_080.0),
        (1_080.0, 1_440.0),
        (1_440.0, 1_800.0),
    ];
    for (row, (start_s, end_s)) in ingress.iter_mut().skip(1).zip(support_windows) {
        let DirectTileGroundIngress::OpenRawPrecipitation {
            raw_precipitation, ..
        } = row
        else {
            panic!("open ingress fixture");
        };
        raw_precipitation.start_s = start_s;
        raw_precipitation.end_s = end_s;
    }
    let input = DirectSurfaceLiquidIngressInput {
        transaction_id,
        day_index: 3,
        interval_index: 0,
        interval_s: INTERVAL_S,
        tile_ingress: ingress,
        wb14_parameters: parameters(&configuration),
    };
    let candidate = execute_surface_liquid_ingress(&configuration, &resource, &input)
        .expect("five-window candidate");
    candidate
        .validate(&configuration, &resource, &input)
        .expect("exact temporal mass closure");

    let parent = candidate
        .closure_operands()
        .source_parcels()
        .iter()
        .find(|row| row.mass_kg_m2_basis_ofe_ground().to_bits() == 0.1_f64.to_bits())
        .expect("0.1 kg/m2 parent");
    let attributed = candidate
        .receipts()
        .iter()
        .filter(|row| row.source_parcel_id == parent.source_parcel_id())
        .map(|row| row.mass_kg_m2_basis_ofe_ground)
        .sum::<f64>();
    assert_eq!(
        attributed.to_bits(),
        parent.mass_kg_m2_basis_ofe_ground().to_bits()
    );
    let all_ratio = (0..5).map(|_| 0.1 * (360.0 / 1_800.0)).sum::<f64>();
    assert_ne!(all_ratio.to_bits(), 0.1_f64.to_bits());

    let mut poison = candidate.clone();
    let receipt = poison
        .receipts
        .iter_mut()
        .find(|row| {
            row.source_parcel_id == parent.source_parcel_id()
                && row.mass_kg_m2_basis_ofe_ground > 0.0
        })
        .expect("positive temporal child");
    receipt.mass_kg_m2_basis_ofe_ground =
        f64::from_bits(receipt.mass_kg_m2_basis_ofe_ground.to_bits() - 1);
    let error = super::super::surface_liquid_closure::validate_surface_liquid_closure_operands(
        &configuration,
        &resource,
        &poison.closure_operands,
        &poison.receipts,
        &poison.ending_state,
    )
    .expect_err("one-ULP temporal mass loss must fail exact closure");
    assert_eq!(error.code(), DirectSurfaceLiquidErrorCode::E010);
}

#[test]
fn cadence_failure_is_e008_with_exact_transaction_and_attempt_hash() {
    let configuration = one_tile_configuration(DirectGroundIngressMode::OpenRawPrecipitation);
    let beginning = initial_state(&configuration, 0.0);
    let transaction_id = TransactionId(411);
    let resource = resource_candidate(&configuration, &beginning, transaction_id, None, &[]);
    let input = DirectSurfaceLiquidIngressInput {
        transaction_id,
        day_index: 3,
        interval_index: 0,
        interval_s: INTERVAL_S + 1.0,
        tile_ingress: vec![open_ingress(&configuration.records[0], 0.1)],
        wb14_parameters: parameters(&configuration),
    };
    let error = execute_surface_liquid_ingress(&configuration, &resource, &input)
        .expect_err("wrong cadence");
    let failure = error.failure().expect("canonical failure");
    assert_eq!(failure.code, DirectSurfaceLiquidErrorCode::E008);
    assert_eq!(failure.phase, DirectSurfaceLiquidPhase::IngressCandidate);
    assert_eq!(failure.context.transaction_id, Some(transaction_id));
    assert!(failure.rollback.attempted_owner_sha256.is_some());
}

#[test]
fn nonfinite_interval_is_e003_before_cadence() {
    let configuration = one_tile_configuration(DirectGroundIngressMode::OpenRawPrecipitation);
    let beginning = initial_state(&configuration, 0.0);
    let transaction_id = TransactionId(4_111);
    let resource = resource_candidate(&configuration, &beginning, transaction_id, None, &[]);
    let input = DirectSurfaceLiquidIngressInput {
        transaction_id,
        day_index: 3,
        interval_index: 0,
        interval_s: f64::NAN,
        tile_ingress: vec![open_ingress(&configuration.records[0], 0.1)],
        wb14_parameters: parameters(&configuration),
    };
    let error = execute_surface_liquid_ingress(&configuration, &resource, &input)
        .expect_err("nonfinite interval");
    let failure = error.failure().expect("canonical interval failure");
    assert_eq!(failure.code, DirectSurfaceLiquidErrorCode::E003);
    assert_eq!(failure.phase, DirectSurfaceLiquidPhase::IngressCandidate);
    assert_eq!(failure.context.transaction_id, Some(transaction_id));
    assert!(failure.rollback.beginning_owner_sha256.is_some());
    assert!(failure.rollback.attempted_owner_sha256.is_some());
}

#[test]
fn unknown_ingress_identity_precedes_cardinality() {
    let configuration = one_tile_configuration(DirectGroundIngressMode::OpenRawPrecipitation);
    let beginning = initial_state(&configuration, 0.0);
    let transaction_id = TransactionId(4_112);
    let resource = resource_candidate(&configuration, &beginning, transaction_id, None, &[]);
    let mut unknown = open_ingress(&configuration.records[0], 0.1);
    let DirectTileGroundIngress::OpenRawPrecipitation { tile_id, .. } = &mut unknown else {
        panic!("open ingress fixture");
    };
    *tile_id = openwepp_kernel_contract::TileId::try_new("unknown").expect("unknown tile");
    let input = DirectSurfaceLiquidIngressInput {
        transaction_id,
        day_index: 3,
        interval_index: 0,
        interval_s: INTERVAL_S,
        tile_ingress: vec![open_ingress(&configuration.records[0], 0.1), unknown],
        wb14_parameters: parameters(&configuration),
    };
    let error = execute_surface_liquid_ingress(&configuration, &resource, &input)
        .expect_err("unknown identity plus cardinality");
    let failure = error.failure().expect("canonical ingress identity failure");
    assert_eq!(failure.code, DirectSurfaceLiquidErrorCode::E002);
    assert_eq!(failure.context.transaction_id, Some(transaction_id));
    assert_eq!(
        failure
            .context
            .tile_id
            .as_ref()
            .map(openwepp_kernel_contract::TileId::as_str),
        Some("unknown")
    );
    assert!(failure.rollback.beginning_owner_sha256.is_some());
    assert!(failure.rollback.attempted_owner_sha256.is_some());
}

#[test]
fn public_ingress_failures_retain_available_owner_and_exact_offender_context() {
    let configuration = one_tile_configuration(DirectGroundIngressMode::OpenRawPrecipitation);
    let beginning = initial_state(&configuration, 0.0);
    let resource_transaction = TransactionId(4_120);
    let resource = resource_candidate(&configuration, &beginning, resource_transaction, None, &[]);
    let mut input = DirectSurfaceLiquidIngressInput {
        transaction_id: TransactionId(4_121),
        day_index: 3,
        interval_index: 0,
        interval_s: INTERVAL_S,
        tile_ingress: vec![open_ingress(&configuration.records[0], 0.1)],
        wb14_parameters: parameters(&configuration),
    };

    let mismatch = execute_surface_liquid_ingress(&configuration, &resource, &input)
        .expect_err("transaction mismatch");
    let failure = mismatch.failure().expect("canonical transaction failure");
    assert_eq!(failure.code, DirectSurfaceLiquidErrorCode::E002);
    assert_eq!(failure.context.transaction_id, Some(input.transaction_id));
    assert_eq!(
        failure.context.owner_id.as_ref(),
        Some(&configuration.owner_id)
    );
    assert_eq!(failure.context.ofe_id, None);
    assert_eq!(failure.context.tile_id, None);

    input.transaction_id = resource_transaction;
    let DirectTileGroundIngress::OpenRawPrecipitation {
        raw_precipitation, ..
    } = &mut input.tile_ingress[0]
    else {
        panic!("open ingress fixture");
    };
    raw_precipitation.mass_kg_m2_tile_ground = f64::NAN;
    let invalid = execute_surface_liquid_ingress(&configuration, &resource, &input)
        .expect_err("invalid direct ingress amount");
    let failure = invalid.failure().expect("canonical direct-ingress failure");
    let key = &configuration.records[0].key;
    assert_eq!(failure.code, DirectSurfaceLiquidErrorCode::E003);
    assert_eq!(failure.context.transaction_id, Some(resource_transaction));
    assert_eq!(
        failure.context.owner_id.as_ref(),
        Some(&configuration.owner_id)
    );
    assert_eq!(failure.context.ofe_id.as_ref(), Some(&key.ofe_id));
    assert_eq!(failure.context.tile_id.as_ref(), Some(&key.tile_id));
    assert_eq!(failure.context.surface_id.as_ref(), Some(&key.surface_id));
    assert_eq!(failure.context.source_id.as_ref(), Some(&key.source_id));
}

#[test]
fn full_infiltration_preserves_exact_source_mass_and_never_debits_surface_store() {
    // 0x1.f9e1df20c7aa4p-6: dividing by 1000 and multiplying back rounds one
    // ULP upward, so this ordinary finite value exercises the exact WB14
    // full-infiltration identity branch.
    let source_mass = f64::from_bits(0x3f9f_9e1d_f20c_7aa4);
    assert!(source_mass / WATER_DENSITY_KG_M3 * WATER_DENSITY_KG_M3 > source_mass);

    for (case_index, beginning_fraction) in [0.0, 0.5].into_iter().enumerate() {
        let configuration = one_tile_configuration(DirectGroundIngressMode::OpenRawPrecipitation);
        let beginning = initial_state(&configuration, beginning_fraction);
        let transaction_id = TransactionId(410 + case_index as u128);
        let resource = resource_candidate(&configuration, &beginning, transaction_id, None, &[]);
        let input = DirectSurfaceLiquidIngressInput {
            transaction_id,
            day_index: 3,
            interval_index: 0,
            interval_s: INTERVAL_S,
            tile_ingress: vec![open_ingress(&configuration.records[0], source_mass)],
            wb14_parameters: vec![DirectOfeWb14Parameters {
                ofe_id: configuration.ofe_topology[0].clone(),
                effective_conductivity_m_s: 1.0e-6,
                matric_potential_m: 0.1,
                infiltration_storage_capacity_m: 1.0,
            }],
        };
        let candidate = execute_surface_liquid_ingress(&configuration, &resource, &input)
            .expect("exact full-infiltration candidate");
        let infiltration = candidate
            .receipts
            .iter()
            .find(|receipt| {
                receipt.disposition == DirectSurfaceLiquidReceiptDisposition::Infiltration
            })
            .expect("full-infiltration receipt");
        assert_eq!(
            infiltration.mass_kg_m2_basis_ofe_ground.to_bits(),
            source_mass.to_bits()
        );
        let ledger = &candidate.ledgers[0];
        assert_eq!(
            ledger.ingress_mass_kg_m2_ofe_ground.to_bits(),
            source_mass.to_bits()
        );
        assert_eq!(
            ledger.infiltration_mass_kg_m2_ofe_ground.to_bits(),
            source_mass.to_bits()
        );
        assert_eq!(
            ledger.retained_mass_kg_m2_ofe_ground.to_bits(),
            0.0_f64.to_bits()
        );
        assert_eq!(
            ledger.runoff_mass_kg_m2_ofe_ground.to_bits(),
            0.0_f64.to_bits()
        );
        let attributed = ledger.infiltration_mass_kg_m2_ofe_ground
            + ledger.retained_mass_kg_m2_ofe_ground
            + ledger.runoff_mass_kg_m2_ofe_ground;
        assert_eq!(
            attributed.to_bits(),
            source_mass.to_bits(),
            "I + E = X exactly"
        );
        assert!(candidate.receipts.iter().all(|receipt| {
            receipt.mass_kg_m2_basis_ofe_ground.is_finite()
                && receipt.mass_kg_m2_basis_ofe_ground >= 0.0
        }));
        assert_eq!(
            candidate.ending_state.records[0]
                .liquid_kg_m2_tile
                .to_bits(),
            beginning.records[0].liquid_kg_m2_tile.to_bits(),
            "full infiltration must not debit the beginning surface store"
        );

        let mut poison = candidate.clone();
        let poisoned = poison
            .receipts
            .iter_mut()
            .find(|receipt| {
                receipt.disposition == DirectSurfaceLiquidReceiptDisposition::Infiltration
            })
            .expect("poisoned infiltration receipt");
        poisoned.mass_kg_m2_basis_ofe_ground *= 0.5;
        poisoned.enthalpy_j_m2_basis_ofe_ground =
            poisoned.mass_kg_m2_basis_ofe_ground * liquid_specific_enthalpy(poisoned.temperature_k);
        let error = super::super::surface_liquid_closure::validate_surface_liquid_closure_operands(
            &configuration,
            &resource,
            &poison.closure_operands,
            &poison.receipts,
            &poison.ending_state,
        )
        .expect_err("raw source/OFE mass closure poison");
        assert_eq!(error.code(), DirectSurfaceLiquidErrorCode::E010);
    }
}

#[test]
fn canonical_last_enthalpy_remainders_close_exactly_and_reject_one_ulp_loss() {
    let configuration = one_tile_configuration(DirectGroundIngressMode::OpenRawPrecipitation);
    let beginning = initial_state(&configuration, 0.0);
    let transaction_id = TransactionId(414);
    let resource = resource_candidate(&configuration, &beginning, transaction_id, None, &[]);
    let input = DirectSurfaceLiquidIngressInput {
        transaction_id,
        day_index: 3,
        interval_index: 0,
        interval_s: INTERVAL_S,
        tile_ingress: vec![open_ingress(&configuration.records[0], 0.3)],
        wb14_parameters: vec![DirectOfeWb14Parameters {
            ofe_id: configuration.ofe_topology[0].clone(),
            effective_conductivity_m_s: 1.0e-6,
            matric_potential_m: 0.1,
            infiltration_storage_capacity_m: 3.0e-5,
        }],
    };
    let candidate = execute_surface_liquid_ingress(&configuration, &resource, &input)
        .expect("partial-infiltration enthalpy fixture");
    let receipt = |disposition| {
        candidate
            .receipts
            .iter()
            .find(|row| row.disposition == disposition)
            .expect("required split receipt")
    };
    let infiltration = receipt(DirectSurfaceLiquidReceiptDisposition::Infiltration);
    let retained = receipt(DirectSurfaceLiquidReceiptDisposition::RetainedSurface);
    let runoff = receipt(DirectSurfaceLiquidReceiptDisposition::OutletRunoff);
    assert!(
        (infiltration.mass_kg_m2_basis_ofe_ground - 0.03).abs() <= 8.0 * f64::EPSILON,
        "fixture must exercise I≈0.03 kg/m²"
    );
    let parent_q = candidate.closure_operands.source_parcels()[0].enthalpy_j_m2_basis_ofe_ground();
    let excess_q = retained.enthalpy_j_m2_basis_ofe_ground + runoff.enthalpy_j_m2_basis_ofe_ground;
    assert_eq!(
        excess_q.to_bits(),
        (parent_q - infiltration.enthalpy_j_m2_basis_ofe_ground).to_bits(),
        "retention + runoff must exactly reconstruct excess Q"
    );
    assert_eq!(
        (infiltration.enthalpy_j_m2_basis_ofe_ground + excess_q).to_bits(),
        parent_q.to_bits(),
        "infiltration + excess must exactly reconstruct parent Q"
    );

    let mut poison = candidate.clone();
    let poisoned = poison
        .receipts
        .iter_mut()
        .find(|row| row.disposition == DirectSurfaceLiquidReceiptDisposition::OutletRunoff)
        .expect("canonical-last runoff receipt");
    poisoned.enthalpy_j_m2_basis_ofe_ground =
        f64::from_bits(poisoned.enthalpy_j_m2_basis_ofe_ground.to_bits() - 1);
    let error = super::super::surface_liquid_closure::validate_surface_liquid_closure_operands(
        &configuration,
        &resource,
        &poison.closure_operands,
        &poison.receipts,
        &poison.ending_state,
    )
    .expect_err("one-ULP canonical-last energy loss must fail exact closure");
    assert_eq!(error.code(), DirectSurfaceLiquidErrorCode::E010);
}
