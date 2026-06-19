use super::*;

fn insert_state_scalar(
    state: &mut BTreeMap<BoundarySymbol, BoundaryValue>,
    symbol: impl Into<BoundarySymbol>,
    value: f64,
) {
    state.insert(symbol.into(), BoundaryValue::scalar(value));
}

fn perfdeep01_h2637_like_warm_rain_surface() -> (
    BTreeMap<BoundarySymbol, BoundaryValue>,
    BTreeMap<BoundarySymbol, BoundaryValue>,
) {
    let mut state = BTreeMap::new();
    let mut flux = BTreeMap::new();

    insert_state_scalar(
        &mut state,
        HillslopeProductionStateSymbol::Wb12RainfallInput,
        0.042,
    );
    insert_state_scalar(
        &mut state,
        HillslopeProductionStateSymbol::Wb12RunonInput,
        0.0065,
    );
    insert_state_scalar(
        &mut state,
        HillslopeProductionStateSymbol::Wb12RunoffClosureTolerance,
        1.0e-9,
    );
    insert_state_scalar(
        &mut state,
        HillslopeProductionStateSymbol::Wb14SoilConductivity,
        3.25e-7,
    );
    insert_state_scalar(
        &mut state,
        HillslopeProductionStateSymbol::Wb14SoilLayerDepth,
        0.20,
    );
    insert_state_scalar(
        &mut state,
        HillslopeProductionStateSymbol::Wb14SoilThetaResidual,
        0.08,
    );
    insert_state_scalar(
        &mut state,
        HillslopeProductionStateSymbol::Wb14SoilThetaFieldCapacity,
        0.34,
    );
    insert_state_scalar(
        &mut state,
        HillslopeProductionStateSymbol::Wb14HyetographNinten,
        2.0,
    );
    insert_state_scalar(
        &mut state,
        HillslopeProductionStateSymbol::Wb14HyetographNbrkpt,
        2.0,
    );
    insert_state_scalar(&mut state, "timem_0001", 0.0);
    insert_state_scalar(&mut state, "timem_0002", 86_400.0);
    insert_state_scalar(&mut state, "intsty_0001", 0.042 / 86_400.0);
    insert_state_scalar(&mut state, "intsty_0002", 0.0);
    insert_state_scalar(
        &mut state,
        HillslopeProductionStateSymbol::Wb15PlantCancov,
        0.71,
    );
    insert_state_scalar(
        &mut state,
        HillslopeProductionStateSymbol::Wb15PlantLai,
        2.2,
    );
    insert_state_scalar(
        &mut state,
        HillslopeProductionStateSymbol::Wb15PlantVdmt,
        1.9,
    );
    insert_state_scalar(
        &mut state,
        HillslopeProductionStateSymbol::Wb12DepressionStorageDelta,
        0.00125,
    );
    insert_state_scalar(&mut state, "day", 44.0);
    insert_state_scalar(&mut state, "year", 1987.0);
    insert_state_scalar(&mut state, "peakro", 0.0042);
    insert_state_scalar(&mut state, "watdur", 1800.0);
    insert_state_scalar(&mut state, "total_detachment_kg", 12.4);
    insert_state_scalar(&mut state, "total_deposition_kg", 3.1);
    insert_state_scalar(&mut state, "sediment_concentration_kg_m3_0001", 0.00042);

    // Seed a subset of MOFE hourly carry families to validate array capture.
    insert_state_scalar(&mut state, "ui_SUrunf_0001", 1.0e-5);
    insert_state_scalar(&mut state, "ui_SUrunf_0002", 2.0e-5);
    insert_state_scalar(&mut state, "ui_SCrunf_0001", 3.0e-5);
    insert_state_scalar(&mut state, "ui_LfUrf_0001", 4.0e-5);
    insert_state_scalar(&mut state, "ui_LfCrf_0001", 5.0e-5);

    flux.insert(
        BoundarySymbol::from("wb12_runoff_carryover"),
        BoundaryValue::scalar(0.0065),
    );

    (state, flux)
}

#[test]
fn perfdeep01_frame_seed_flush_roundtrip_is_bit_identical() {
    let (state, flux) = perfdeep01_h2637_like_warm_rain_surface();
    let registry = SymbolRegistry::from_surfaces(&state, &flux)
        .expect("registry should build from the seeded warm-rain fixture");

    let frame = HillslopeDayFrame::seed_from_surfaces(&state, &flux, &registry, None)
        .expect("frame seed should succeed");
    let report = frame
        .assert_shadow_roundtrip_bits(&state, &flux)
        .expect("seed/flush shadow roundtrip should remain bit-identical");

    assert!(report.is_bit_identical());
    assert_eq!(report.state_symbol_count, state.len());
    assert_eq!(report.flux_symbol_count, flux.len());
    assert_eq!(report.state_mismatch_count, 0);
    assert_eq!(report.flux_mismatch_count, 0);

    let flushed = frame
        .flush_to_writeback_surface()
        .expect("flush should reconstruct a logical writeback surface");
    assert_eq!(flushed.state_surface, state);
    assert_eq!(flushed.flux_surface, flux);
}

#[test]
fn perfdeep01_frame_captures_io_edge_scalars_and_mofe_arrays() {
    let (state, flux) = perfdeep01_h2637_like_warm_rain_surface();
    let registry = SymbolRegistry::from_surfaces(&state, &flux)
        .expect("registry should build from seeded fixture");

    let frame = HillslopeDayFrame::seed_from_surfaces(&state, &flux, &registry, None)
        .expect("frame seed should succeed");

    let io = frame.io_edge_scalars();
    assert_eq!(io.peakro, Some(BoundaryValue::scalar(0.0042)));
    assert_eq!(io.watdur, Some(BoundaryValue::scalar(1800.0)));
    assert_eq!(io.total_detachment_kg, Some(BoundaryValue::scalar(12.4)));
    assert_eq!(io.total_deposition_kg, Some(BoundaryValue::scalar(3.1)));
    assert_eq!(
        io.sediment_concentration_kg_m3_0001,
        Some(BoundaryValue::scalar(0.00042))
    );
    assert_eq!(io.runtime_day, Some(BoundaryValue::scalar(44.0)));
    assert_eq!(io.runtime_year, Some(BoundaryValue::scalar(1987.0)));

    assert_eq!(
        frame.mofe_hourly_upstream_saturation_runoff[0],
        Some(BoundaryValue::scalar(1.0e-5))
    );
    assert_eq!(
        frame.mofe_hourly_upstream_saturation_runoff[1],
        Some(BoundaryValue::scalar(2.0e-5))
    );
    assert_eq!(
        frame.mofe_hourly_current_saturation_runoff[0],
        Some(BoundaryValue::scalar(3.0e-5))
    );
    assert_eq!(
        frame.mofe_hourly_upstream_lateral_runoff[0],
        Some(BoundaryValue::scalar(4.0e-5))
    );
    assert_eq!(
        frame.mofe_hourly_current_lateral_runoff[0],
        Some(BoundaryValue::scalar(5.0e-5))
    );
}

#[test]
fn perfdeep01_frame_borrows_climate_forcing_series_without_copy() {
    let (state, flux) = perfdeep01_h2637_like_warm_rain_surface();
    let registry = SymbolRegistry::from_surfaces(&state, &flux)
        .expect("registry should build from seeded fixture");
    let forcing = vec![1.0, 2.0, 3.0, 4.0];

    let frame = HillslopeDayFrame::seed_from_surfaces(&state, &flux, &registry, Some(&forcing))
        .expect("frame seed should succeed with borrowed climate forcing");

    let borrowed = frame
        .climate_forcing_series()
        .expect("climate forcing should remain borrowed on the frame");
    assert_eq!(borrowed, forcing.as_slice());
    assert!(std::ptr::eq(borrowed.as_ptr(), forcing.as_ptr()));
}
