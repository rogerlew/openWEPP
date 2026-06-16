use super::fixtures::*;
use super::*;

fn assert_erod19_xcrit_case(inputs: (f64, f64, f64, f64, f64, f64), expected: (f64, f64, f64)) {
    let (a, b, c, tauc, xb, xe) = inputs;
    let (observed_mshear, observed_xc1, observed_xc2) =
        Wb11HydrologyKernel::erod19_xcrit_classification(a, b, c, tauc, xb, xe);
    let (expected_mshear, expected_xc1, expected_xc2) = expected;

    assert!(
        (observed_mshear - expected_mshear).abs() <= 1.0e-12,
        "mshear mismatch for inputs {inputs:?}: observed {observed_mshear}, expected {expected_mshear}"
    );
    assert!(
        (observed_xc1 - expected_xc1).abs() <= 1.0e-12,
        "xc1 mismatch for inputs {inputs:?}: observed {observed_xc1}, expected {expected_xc1}"
    );
    assert!(
        (observed_xc2 - expected_xc2).abs() <= 1.0e-12,
        "xc2 mismatch for inputs {inputs:?}: observed {observed_xc2}, expected {expected_xc2}"
    );
}

#[test]
fn cqr17_erod19_xcrit_classification_preserves_branch_vectors() {
    let cases = [
        (
            "linear increasing critical point inside segment",
            (0.0, 4.0, 1.0, 2.0, 0.0, 1.0),
            (3.0, 0.457_106_781_186_547_6, 1.0),
        ),
        (
            "linear decreasing critical point inside segment",
            (0.0, -4.0, 5.0, 2.0, -1.0, 1.0),
            (4.0, -0.0, 1.0),
        ),
        (
            "convex rising all above critical shear",
            (1.0, 0.0, 4.0, 2.0, 0.0, 1.0),
            (2.0, 0.0, 1.0),
        ),
        (
            "convex rising all below critical shear",
            (1.0, 0.0, 0.0, 3.0, 0.0, 1.0),
            (1.0, 0.0, 1.0),
        ),
        (
            "convex rising crosses critical shear",
            (1.0, 0.0, 0.0, 0.5, 0.0, 1.0),
            (3.0, 0.594_603_557_501_360_5, 1.0),
        ),
        (
            "curved segment remains above critical shear",
            (-1.0, 0.0, 9.0, 2.0, 0.0, 1.0),
            (2.0, 0.0, 1.0),
        ),
        (
            "curved segment has no real critical crossing",
            (-1.0, 0.0, 0.0, 1.0, 0.0, 1.0),
            (1.0, 0.0, 1.0),
        ),
        (
            "curved segment crosses from below to above critical shear",
            (-1.0, 3.0, 0.0, 1.0, 0.0, 1.0),
            (3.0, 0.381_966_011_250_105_1, 1.0),
        ),
        (
            "curved segment crosses from above to below critical shear",
            (-0.1, -1.0, 3.0, 2.0, 0.0, 2.0),
            (4.0, 0.0, 2.0),
        ),
        (
            "curved segment has two critical crossings",
            (-4.0, 4.0, 0.0, 0.5, 0.0, 1.0),
            (5.0, 0.097_990_482_262_320_6, 0.902_009_517_737_679_5),
        ),
    ];

    for (label, inputs, expected) in cases {
        assert_erod19_xcrit_case(inputs, expected);
        eprintln!("covered EROD19 xcrit vector: {label}");
    }
}

fn cqr23_route_state_surface() -> BTreeMap<BoundarySymbol, BoundaryValue> {
    let mut state_surface = BTreeMap::new();
    for (symbol, value) in [
        ("erod14_wave2_enabled", 1.0),
        ("nslpts", 2.0),
        ("xu_0002", 0.2),
        ("xl_0002", 0.5),
        ("ainf_0002", 0.4),
        ("binf_0002", 0.3),
        ("cinf_0002", 0.2),
        ("ainftc_0002", 0.4),
        ("binftc_0002", 0.3),
        ("cinftc_0002", 0.2),
        ("qostar", 0.2),
        ("xdetst", 0.1),
        ("lddend", 0.3),
        ("erod14_ktrato", 1.1),
        ("theta", 0.2),
        ("phi", 0.5),
        ("taucn", 0.2),
        ("G", 0.2),
    ] {
        state_surface.insert(BoundarySymbol::from(symbol), BoundaryValue::scalar(value));
    }
    state_surface
}

fn cqr23_run_route_segment(
    state_surface: &BTreeMap<BoundarySymbol, BoundaryValue>,
    erod13_state_updates: &[WritebackField],
) -> Vec<WritebackField> {
    let flux_surface = BTreeMap::new();
    let request = HillslopeKernelRequest::with_phase_context(
        "peak_runoff",
        HillslopeKernelPhaseClass::HydrologyPeakRunoff,
        HillslopeConsumerAdapter::Runoff,
        None,
        state_surface,
        &flux_surface,
    );

    Wb11HydrologyKernel::run_erod19_route_segment_migration(&request, erod13_state_updates)
        .expect("EROD19 route segment migration should succeed")
}

fn assert_cqr23_route_field(
    updates: &[WritebackField],
    index: usize,
    symbol: &str,
    value: f64,
    minimum: Option<f64>,
    maximum: Option<f64>,
) {
    let field = updates
        .get(index)
        .unwrap_or_else(|| panic!("missing CQR23 route field at index {index}"));
    assert_eq!(field.symbol, BoundarySymbol::from(symbol));
    assert!(
        (field.value.as_f64() - value).abs() <= 1.0e-12,
        "{symbol} value mismatch: observed {}, expected {value}",
        field.value.as_f64()
    );
    assert_eq!(field.minimum, minimum, "{symbol} minimum mismatch");
    assert_eq!(field.maximum, maximum, "{symbol} maximum mismatch");
}

#[test]
fn cqr23_erod19_route_segment_characterizes_wave_gate() {
    let state_surface = BTreeMap::new();
    let updates = cqr23_run_route_segment(&state_surface, &[]);

    assert!(
        updates.is_empty(),
        "disabled EROD14 wave2 route must publish no EROD19 updates"
    );
}

#[test]
fn cqr23_erod19_route_segment_characterizes_publication_family() {
    let state_surface = cqr23_route_state_surface();
    let updates = cqr23_run_route_segment(&state_surface, &[]);

    assert_eq!(updates.len(), 21);
    assert_cqr23_route_field(&updates, 0, "nslpts", 2.0, Some(2.0), None);
    assert_cqr23_route_field(&updates, 1, "xu_0002", 0.2, Some(0.0), None);
    assert_cqr23_route_field(&updates, 2, "xl_0002", 0.5, Some(0.2), None);
    assert_cqr23_route_field(&updates, 3, "ainf_0002", 0.4, None, None);
    assert_cqr23_route_field(&updates, 4, "binf_0002", 0.3, None, None);
    assert_cqr23_route_field(&updates, 5, "cinf_0002", 0.2, None, None);
    assert_cqr23_route_field(&updates, 6, "ainftc_0002", 0.4, None, None);
    assert_cqr23_route_field(&updates, 7, "binftc_0002", 0.3, None, None);
    assert_cqr23_route_field(&updates, 8, "cinftc_0002", 0.2, None, None);
    assert_cqr23_route_field(&updates, 9, "qostar", 0.2, None, None);
    assert_cqr23_route_field(&updates, 10, "xdbeg", 0.0, Some(0.0), None);
    assert_cqr23_route_field(&updates, 11, "xdend", 0.5, Some(0.2), Some(0.5));
    assert_cqr23_route_field(&updates, 12, "xdetst", 0.1, Some(0.0), Some(0.5));
    assert_cqr23_route_field(&updates, 13, "ldlast", 0.2, Some(0.0), None);
    assert_cqr23_route_field(&updates, 14, "lddend", 0.2, Some(0.0), None);
    assert_cqr23_route_field(&updates, 15, "du", 0.0, None, None);
    assert_cqr23_route_field(&updates, 16, "dl", 0.0, None, None);
    assert_cqr23_route_field(&updates, 17, "ndep", 0.0, Some(0.0), Some(1.0));
    assert_cqr23_route_field(&updates, 18, "mshear", 2.0, Some(1.0), Some(5.0));
    assert_cqr23_route_field(&updates, 19, "xc1", 0.2, Some(0.2), Some(0.5));
    assert_cqr23_route_field(&updates, 20, "xc2", 0.5, Some(0.2), Some(0.5));
}

#[test]
fn cqr23_erod19_route_segment_characterizes_erod13_update_precedence() {
    let mut state_surface = cqr23_route_state_surface();
    state_surface.insert(BoundarySymbol::from("theta"), BoundaryValue::scalar(0.8));
    state_surface.insert(BoundarySymbol::from("phi"), BoundaryValue::scalar(0.9));
    state_surface.insert(BoundarySymbol::from("taucn"), BoundaryValue::scalar(0.9));

    let erod13_state_updates = [
        WritebackField::bounded("theta", 0.2, Some(0.0), None),
        WritebackField::bounded("phi", 0.5, Some(0.0), None),
        WritebackField::bounded("taucn", 0.2, Some(0.0), None),
    ];
    let stale_state_updates = cqr23_run_route_segment(&state_surface, &[]);
    let override_updates = cqr23_run_route_segment(&state_surface, &erod13_state_updates);

    let stale_mshear =
        state_update_scalar(&stale_state_updates, "mshear").expect("mshear should publish");
    let override_mshear =
        state_update_scalar(&override_updates, "mshear").expect("mshear should publish");
    assert!((stale_mshear - 1.0).abs() <= 1.0e-12);
    assert!((override_mshear - 2.0).abs() <= 1.0e-12);
}

#[test]
fn cqr23_erod19_route_segment_characterizes_legacy_input_fallbacks() {
    let mut state_surface = cqr23_route_state_surface();
    state_surface.remove(&BoundarySymbol::from("theta"));
    state_surface.remove(&BoundarySymbol::from("phi"));
    state_surface.remove(&BoundarySymbol::from("taucn"));
    state_surface.remove(&BoundarySymbol::from("G"));

    for (symbol, value) in [
        ("cntlen", 2.0),
        ("detinr", 0.15),
        ("tcend", 1.5),
        ("effdrr", 0.8),
        ("effdrn", 1.0),
        ("beta", 0.2),
        ("veleff", 0.5),
        ("pkro", 0.25),
        ("tcadjf", 0.5),
        ("shcrit", 1.0),
        ("shrsol", 1.0),
    ] {
        state_surface.insert(BoundarySymbol::from(symbol), BoundaryValue::scalar(value));
    }

    let updates = cqr23_run_route_segment(&state_surface, &[]);

    let du = state_update_scalar(&updates, "du").expect("du should publish");
    let mshear = state_update_scalar(&updates, "mshear").expect("mshear should publish");
    let xc1 = state_update_scalar(&updates, "xc1").expect("xc1 should publish");
    let xc2 = state_update_scalar(&updates, "xc2").expect("xc2 should publish");
    assert!((du - -0.16).abs() <= 1.0e-12);
    assert!((mshear - 3.0).abs() <= 1.0e-12);
    assert!((xc1 - 0.349_229_574_432_848_2).abs() <= 1.0e-12);
    assert!((xc2 - 0.5).abs() <= 1.0e-12);
}

#[test]
fn hphys0246_wb18_percolation_preserves_residual_storage_in_aggregate_soil_water() {
    let state_surface = hphys0246_wb18_aggregate_state_surface();
    let flux_surface = BTreeMap::new();
    let request = HillslopeKernelRequest::with_phase_context(
        "percolation_deep_seepage",
        HillslopeKernelPhaseClass::HydrologyPercolationDeepSeepage,
        HillslopeConsumerAdapter::Perc,
        None,
        &state_surface,
        &flux_surface,
    );

    let mut kernel = Wb11HydrologyKernel;
    let response = kernel.run_hillslope_phase(&request);

    assert_eq!(response.status.message_id(), "HKERNEL-WB11-PERC-OK-001");
    let soil_water_after =
        state_update_scalar(&response.writeback.state_updates, "wb11_soil_water")
            .expect("WB18 should publish wb11_soil_water");
    let theta_after =
        state_update_scalar(&response.writeback.state_updates, "wb18_perc_theta_0001")
            .expect("WB18 should publish layer 1 theta")
            + state_update_scalar(&response.writeback.state_updates, "wb18_perc_theta_0002")
                .expect("WB18 should publish layer 2 theta");
    let expected_soilw = theta_after + (0.05 * 0.30) + (0.07 * 0.40);

    assert!(
        (soil_water_after - expected_soilw).abs() < 1.0e-12,
        "WB18 aggregate soil water must follow baseline soilw=sum(st+thetdr*dg), observed {soil_water_after} expected {expected_soilw}"
    );
    assert!(
        (soil_water_after - theta_after).abs() > 1.0e-6,
        "test vector must detect the old sigma-theta-only writeback"
    );
}

#[test]
fn hphys0246_wb18_percolation_requires_residual_storage_symbols_for_aggregate_writeback() {
    let mut state_surface = hphys0246_wb18_aggregate_state_surface();
    state_surface.remove(&BoundarySymbol::from("thetdr_0002"));
    let flux_surface = BTreeMap::new();
    let request = HillslopeKernelRequest::with_phase_context(
        "percolation_deep_seepage",
        HillslopeKernelPhaseClass::HydrologyPercolationDeepSeepage,
        HillslopeConsumerAdapter::Perc,
        None,
        &state_surface,
        &flux_surface,
    );

    let mut kernel = Wb11HydrologyKernel;
    let response = kernel.run_hillslope_phase(&request);

    assert_eq!(
        response.status.message_id(),
        "HKERNEL-WB11-PERC-E-001",
        "WB18 must fail closed instead of silently defaulting missing residual storage"
    );
    assert!(
        response.writeback.state_updates.is_empty(),
        "failed WB18 guard must not publish partial state updates"
    );
}

#[test]
fn wbval05_wb18_percolation_rejects_invalid_projected_snow_state_before_zero_infiltration() {
    let mut state_surface = hphys0246_wb18_aggregate_state_surface();
    state_surface.insert(
        BoundarySymbol::from("management.initial.params.tillay2_m"),
        BoundaryValue::scalar(0.0),
    );
    state_surface.insert(
        BoundarySymbol::from("wb12_infiltration"),
        BoundaryValue::scalar(0.0),
    );

    state_surface.insert(
        BoundarySymbol::from("wb12_rainfall_input"),
        BoundaryValue::scalar(0.0),
    );
    state_surface.insert(BoundarySymbol::from("ssc"), BoundaryValue::scalar(1.0e-6));
    state_surface.insert(BoundarySymbol::from("dg"), BoundaryValue::scalar(0.10));
    state_surface.insert(BoundarySymbol::from("thetdr"), BoundaryValue::scalar(0.05));
    state_surface.insert(BoundarySymbol::from("thetfc"), BoundaryValue::scalar(0.10));
    state_surface.insert(BoundarySymbol::from("ninten"), BoundaryValue::scalar(0.0));
    state_surface.insert(
        BoundarySymbol::from("snow.runtime_swe"),
        BoundaryValue::scalar(-0.006_171_157_610_042_402),
    );

    let flux_surface = BTreeMap::new();
    let request = HillslopeKernelRequest::with_phase_context(
        "percolation_deep_seepage",
        HillslopeKernelPhaseClass::HydrologyPercolationDeepSeepage,
        HillslopeConsumerAdapter::Perc,
        None,
        &state_surface,
        &flux_surface,
    );

    let mut kernel = Wb11HydrologyKernel;
    let response = kernel.run_hillslope_phase(&request);

    assert_eq!(
        response.status.message_id(),
        "HKERNEL-WB11-PERC-E-003",
        "WB18 must fail closed on invalid projected snow state before consuming compatibility infiltration"
    );
    assert!(response.writeback.state_updates.is_empty());
}

#[test]
fn fdhp01_c1b_wb18_consumes_published_infiltration_without_replaying_wb14_frost() {
    let mut state_surface = hphys0246_wb18_aggregate_state_surface();
    state_surface.insert(
        BoundarySymbol::from("management.initial.params.tillay2_m"),
        BoundaryValue::scalar(0.0),
    );
    state_surface.insert(
        BoundarySymbol::from("wb12_infiltration"),
        BoundaryValue::scalar(0.003),
    );
    state_surface.insert(
        BoundarySymbol::from("wb12_rainfall_input"),
        BoundaryValue::scalar(0.025),
    );
    state_surface.insert(
        BoundarySymbol::from("snow.runtime_swe"),
        BoundaryValue::scalar(0.0),
    );
    state_surface.insert(
        BoundarySymbol::from("snow.runtime_depth_m"),
        BoundaryValue::scalar(0.0),
    );
    state_surface.insert(
        BoundarySymbol::from("snow.runtime_density_kg_m3"),
        BoundaryValue::scalar(0.0),
    );
    state_surface.insert(
        BoundarySymbol::from("snow.runtime_settle_day_count"),
        BoundaryValue::scalar(0.0),
    );

    let flux_surface = BTreeMap::new();
    let request = HillslopeKernelRequest::with_phase_context(
        "percolation_deep_seepage",
        HillslopeKernelPhaseClass::HydrologyPercolationDeepSeepage,
        HillslopeConsumerAdapter::Perc,
        None,
        &state_surface,
        &flux_surface,
    );

    let mut kernel = Wb11HydrologyKernel;
    let response = kernel.run_hillslope_phase(&request);

    assert_eq!(
        response.status.message_id(),
        "HKERNEL-WB11-PERC-OK-001",
        "WB18 must consume already-published same-pass infiltration instead of replaying WB14 frost/runoff coupling"
    );
    let infiltration = state_update_scalar(&response.writeback.state_updates, "wb12_infiltration")
        .expect("WB18 must carry published infiltration through writeback");
    assert!(
        (infiltration - 0.003).abs() < 1.0e-12,
        "published infiltration must be preserved, observed {infiltration}"
    );
}

#[test]
fn fdhp01_c1b_wb18_canonicalizes_roundoff_deep_loss_before_storage_debit() {
    let bottom_theta = 0.250_000_000_006;
    let initial_soil_water = 0.10 + bottom_theta + (0.05 * 0.30) + (0.07 * 0.40);
    let mut state_surface = hphys0246_wb18_aggregate_state_surface();
    state_surface.insert(
        BoundarySymbol::from("wb11_soil_water"),
        BoundaryValue::scalar(initial_soil_water),
    );
    state_surface.insert(
        BoundarySymbol::from("wb18_perc_theta_0002"),
        BoundaryValue::scalar(bottom_theta),
    );

    let flux_surface = BTreeMap::new();
    let request = HillslopeKernelRequest::with_phase_context(
        "percolation_deep_seepage",
        HillslopeKernelPhaseClass::HydrologyPercolationDeepSeepage,
        HillslopeConsumerAdapter::Perc,
        None,
        &state_surface,
        &flux_surface,
    );

    let mut kernel = Wb11HydrologyKernel;
    let response = kernel.run_hillslope_phase(&request);

    assert_eq!(response.status.message_id(), "HKERNEL-WB11-PERC-OK-001");
    let soil_water_after =
        state_update_scalar(&response.writeback.state_updates, "wb11_soil_water")
            .expect("WB18 must publish wb11_soil_water");
    let bottom_theta_after =
        state_update_scalar(&response.writeback.state_updates, "wb18_perc_theta_0002")
            .expect("WB18 must publish layer 2 theta");
    let deep_loss = flux_update_scalar(&response.writeback.flux_updates, "D")
        .expect("WB18 must publish deep percolation");
    let recharge = flux_update_scalar(&response.writeback.flux_updates, "Pe")
        .expect("WB18 must publish recharge");

    assert!(
        (soil_water_after - initial_soil_water).abs() < 1.0e-15,
        "sub-WB13-threshold deep-loss dust must not debit storage, observed {soil_water_after} expected {initial_soil_water}"
    );
    assert!(
        (bottom_theta_after - bottom_theta).abs() < 1.0e-15,
        "roundoff deep loss must be restored to the bottom layer, observed {bottom_theta_after} expected {bottom_theta}"
    );
    assert!(deep_loss.abs() < 1.0e-15);
    assert!(recharge.abs() < 1.0e-15);
}

#[test]
fn fdhp01_c1b_wb18_positive_deep_loss_uses_scalar_ledger_and_rebalances_layers() {
    let bottom_theta = 0.250_000_000_012;
    let aggregate_soil_water = 0.10 + bottom_theta + (0.05 * 0.30) + (0.07 * 0.40);
    let incoming_soil_water = aggregate_soil_water - 1.5e-12;
    let mut state_surface = hphys0246_wb18_aggregate_state_surface();
    state_surface.insert(
        BoundarySymbol::from("wb11_soil_water"),
        BoundaryValue::scalar(incoming_soil_water),
    );
    state_surface.insert(
        BoundarySymbol::from("wb18_perc_theta_0002"),
        BoundaryValue::scalar(bottom_theta),
    );

    let flux_surface = BTreeMap::new();
    let request = HillslopeKernelRequest::with_phase_context(
        "percolation_deep_seepage",
        HillslopeKernelPhaseClass::HydrologyPercolationDeepSeepage,
        HillslopeConsumerAdapter::Perc,
        None,
        &state_surface,
        &flux_surface,
    );

    let mut kernel = Wb11HydrologyKernel;
    let response = kernel.run_hillslope_phase(&request);

    assert_eq!(response.status.message_id(), "HKERNEL-WB11-PERC-OK-001");
    let soil_water_after =
        state_update_scalar(&response.writeback.state_updates, "wb11_soil_water")
            .expect("WB18 must publish wb11_soil_water");
    let theta_after =
        state_update_scalar(&response.writeback.state_updates, "wb18_perc_theta_0001")
            .expect("WB18 must publish layer 1 theta")
            + state_update_scalar(&response.writeback.state_updates, "wb18_perc_theta_0002")
                .expect("WB18 must publish layer 2 theta");
    let aggregate_after = theta_after + (0.05 * 0.30) + (0.07 * 0.40);
    let deep_loss = flux_update_scalar(&response.writeback.flux_updates, "D")
        .expect("WB18 must publish deep percolation");
    let expected_soil_water_after = incoming_soil_water - deep_loss;

    assert!(
        deep_loss > 1.0e-11,
        "test vector must remain above the deep-percolation dust threshold"
    );
    assert!(
        (soil_water_after - expected_soil_water_after).abs() < 1.0e-15,
        "positive-D WB18 must debit the scalar ledger exactly, observed {soil_water_after} expected {expected_soil_water_after}"
    );
    assert!(
        (aggregate_after - expected_soil_water_after).abs() < 1.0e-15,
        "positive-D WB18 must rebalance layer roundoff to the scalar ledger, observed {aggregate_after} expected {expected_soil_water_after}"
    );
}

#[test]
fn fdhp01_c1b_wb18_no_flux_preserves_scalar_and_rebalances_layer_roundoff() {
    let aggregate_soil_water = 0.10 + 0.20 + (0.05 * 0.30) + (0.07 * 0.40);
    let incoming_soil_water = aggregate_soil_water - 1.2e-11;
    let mut state_surface = hphys0246_wb18_aggregate_state_surface();
    state_surface.insert(
        BoundarySymbol::from("wb11_soil_water"),
        BoundaryValue::scalar(incoming_soil_water),
    );

    let flux_surface = BTreeMap::new();
    let request = HillslopeKernelRequest::with_phase_context(
        "percolation_deep_seepage",
        HillslopeKernelPhaseClass::HydrologyPercolationDeepSeepage,
        HillslopeConsumerAdapter::Perc,
        None,
        &state_surface,
        &flux_surface,
    );

    let mut kernel = Wb11HydrologyKernel;
    let response = kernel.run_hillslope_phase(&request);

    assert_eq!(response.status.message_id(), "HKERNEL-WB11-PERC-OK-001");
    let soil_water_after =
        state_update_scalar(&response.writeback.state_updates, "wb11_soil_water")
            .expect("WB18 must publish wb11_soil_water");
    let theta_after =
        state_update_scalar(&response.writeback.state_updates, "wb18_perc_theta_0001")
            .expect("WB18 must publish layer 1 theta")
            + state_update_scalar(&response.writeback.state_updates, "wb18_perc_theta_0002")
                .expect("WB18 must publish layer 2 theta");
    let aggregate_after = theta_after + (0.05 * 0.30) + (0.07 * 0.40);
    let deep_loss = flux_update_scalar(&response.writeback.flux_updates, "D")
        .expect("WB18 must publish deep percolation");

    assert!(deep_loss.abs() < 1.0e-15);
    assert!(
        (soil_water_after - incoming_soil_water).abs() < 1.0e-15,
        "no-flux WB18 must preserve incoming scalar, observed {soil_water_after} expected {incoming_soil_water}"
    );
    assert!(
        (aggregate_after - incoming_soil_water).abs() < 1.0e-15,
        "no-flux WB18 must rebalance layer roundoff with the preserved scalar, observed {aggregate_after} expected {incoming_soil_water}"
    );
    assert!(
        (aggregate_after - aggregate_soil_water).abs() > 1.0e-12,
        "test vector must expose the pre-existing scalar/layer mismatch"
    );
}

#[test]
fn fdhp01_c1b_wb17_zero_uptake_preserves_incoming_soil_water_scalar() {
    let aggregate_soil_water = 0.10 + 0.20 + (0.05 * 0.30) + (0.07 * 0.40);
    let incoming_soil_water = aggregate_soil_water - 1.5e-12;
    let mut state_surface = hphys0246_wb18_aggregate_state_surface();
    state_surface.insert(
        BoundarySymbol::from("wb11_soil_water"),
        BoundaryValue::scalar(incoming_soil_water),
    );
    state_surface.insert(BoundarySymbol::from("rtd"), BoundaryValue::scalar(0.30));
    state_surface.insert(BoundarySymbol::from("pltol"), BoundaryValue::scalar(0.25));

    let mut flux_surface = BTreeMap::new();
    flux_surface.insert(BoundarySymbol::from("ET"), BoundaryValue::scalar(0.0));
    flux_surface.insert(BoundarySymbol::from("Etp"), BoundaryValue::scalar(0.0));

    let request = HillslopeKernelRequest::with_phase_context(
        "plant_root_uptake",
        HillslopeKernelPhaseClass::HydrologyPlantRootUptake,
        HillslopeConsumerAdapter::Watbal,
        None,
        &state_surface,
        &flux_surface,
    );

    let mut kernel = Wb11HydrologyKernel;
    let response = kernel.run_hillslope_phase(&request);

    assert_eq!(response.status.message_id(), "HKERNEL-WB17-SWU-OK-001");
    let soil_water_after =
        state_update_scalar(&response.writeback.state_updates, "wb11_soil_water")
            .expect("WB17 must publish wb11_soil_water");
    let ui = flux_update_scalar(&response.writeback.flux_updates, "Ep")
        .expect("WB17 must publish actual plant uptake");

    assert!(ui.abs() < 1.0e-15);
    assert!(
        (soil_water_after - incoming_soil_water).abs() < 1.0e-15,
        "zero-uptake WB17 must not convert aggregate recompute dust into storage, observed {soil_water_after} expected {incoming_soil_water}"
    );
    assert!(
        (soil_water_after - aggregate_soil_water).abs() > 1.0e-12,
        "test vector must expose the scalar/layer mismatch"
    );
}

#[test]
fn hphys0264_pmet_evapotranspiration_consumes_evappm_components_without_pt_repartition() {
    let mut state_surface = BTreeMap::new();
    state_surface.insert(
        BoundarySymbol::from("wb11_soil_water"),
        BoundaryValue::scalar(0.222),
    );
    state_surface.insert(
        BoundarySymbol::from("wb11_et_demand"),
        BoundaryValue::scalar(0.004),
    );
    state_surface.insert(BoundarySymbol::from("lai"), BoundaryValue::scalar(1.2));
    state_surface.insert(BoundarySymbol::from("cancov"), BoundaryValue::scalar(0.72));
    state_surface.insert(
        BoundarySymbol::from("wb17_residue_interception"),
        BoundaryValue::scalar(0.000_2),
    );
    state_surface.insert(
        BoundarySymbol::from("wb11_et_seed_branch_evappm"),
        BoundaryValue::scalar(1.0),
    );
    state_surface.insert(
        BoundarySymbol::from("pmet.es_m"),
        BoundaryValue::scalar(0.001_1),
    );
    state_surface.insert(
        BoundarySymbol::from("pmet.ep_m"),
        BoundaryValue::scalar(0.003_4),
    );
    state_surface.insert(BoundarySymbol::from("nsl"), BoundaryValue::scalar(2.0));
    state_surface.insert(
        BoundarySymbol::from("wb18_perc_theta_0001"),
        BoundaryValue::scalar(0.050),
    );
    state_surface.insert(
        BoundarySymbol::from("wb18_perc_theta_0002"),
        BoundaryValue::scalar(0.100),
    );
    state_surface.insert(
        BoundarySymbol::from("wb18_perc_ul_0001"),
        BoundaryValue::scalar(1.0),
    );
    state_surface.insert(
        BoundarySymbol::from("wb18_perc_ul_0002"),
        BoundaryValue::scalar(1.0),
    );
    state_surface.insert(BoundarySymbol::from("dg_0001"), BoundaryValue::scalar(0.05));
    state_surface.insert(BoundarySymbol::from("dg_0002"), BoundaryValue::scalar(0.20));
    state_surface.insert(
        BoundarySymbol::from("thetdr_0001"),
        BoundaryValue::scalar(0.04),
    );
    state_surface.insert(
        BoundarySymbol::from("thetdr_0002"),
        BoundaryValue::scalar(0.05),
    );
    let flux_surface = BTreeMap::new();
    let request = HillslopeKernelRequest::with_phase_context(
        "evapotranspiration",
        HillslopeKernelPhaseClass::HydrologyEvapotranspiration,
        HillslopeConsumerAdapter::Watbal,
        None,
        &state_surface,
        &flux_surface,
    );

    let mut kernel = Wb11HydrologyKernel;
    let response = kernel.run_hillslope_phase(&request);

    assert_eq!(response.status.message_id(), "HKERNEL-WB11-ET-OK-001");
    let etp = flux_update_scalar(&response.writeback.flux_updates, "Etp")
        .expect("PMET seam must publish Etp");
    let es = flux_update_scalar(&response.writeback.flux_updates, "Es")
        .expect("PMET seam must publish Es");
    let er = flux_update_scalar(&response.writeback.flux_updates, "Er")
        .expect("PMET seam must publish Er");

    assert!(
        (etp - 0.003_4).abs() < 1.0e-12,
        "PMET mode must pass pmet.ep_m to SWU as Etp, observed {etp}"
    );
    assert!(
        (es + er - 0.001_1).abs() < 1.0e-12,
        "PMET mode must derive Es+Er from pmet.es_m, observed Es={es} Er={er}"
    );
    assert!(
        (etp - (1.2 * 0.004 / 3.0)).abs() > 1.0e-6,
        "test vector must detect the old Priestley-Taylor LAI repartition"
    );
}

#[test]
fn hphys0264_pmet_evapotranspiration_rejects_material_negative_soil_evaporation() {
    let mut state_surface = BTreeMap::new();
    state_surface.insert(
        BoundarySymbol::from("wb11_soil_water"),
        BoundaryValue::scalar(0.010),
    );
    state_surface.insert(
        BoundarySymbol::from("wb11_et_demand"),
        BoundaryValue::scalar(0.001),
    );
    state_surface.insert(BoundarySymbol::from("lai"), BoundaryValue::scalar(2.4));
    state_surface.insert(BoundarySymbol::from("cancov"), BoundaryValue::scalar(0.10));
    state_surface.insert(
        BoundarySymbol::from("wb17_residue_interception"),
        BoundaryValue::scalar(0.0),
    );
    state_surface.insert(
        BoundarySymbol::from("wb11_et_seed_branch_evappm"),
        BoundaryValue::scalar(1.0),
    );
    state_surface.insert(
        BoundarySymbol::from("pmet.es_m"),
        BoundaryValue::scalar(-0.000_575_419_020_248_203_2),
    );
    state_surface.insert(
        BoundarySymbol::from("pmet.ep_m"),
        BoundaryValue::scalar(0.001),
    );
    state_surface.insert(BoundarySymbol::from("nsl"), BoundaryValue::scalar(1.0));
    state_surface.insert(
        BoundarySymbol::from("wb18_perc_theta_0001"),
        BoundaryValue::scalar(0.010),
    );
    state_surface.insert(
        BoundarySymbol::from("wb18_perc_ul_0001"),
        BoundaryValue::scalar(1.0),
    );
    state_surface.insert(BoundarySymbol::from("dg_0001"), BoundaryValue::scalar(0.20));
    state_surface.insert(
        BoundarySymbol::from("thetdr_0001"),
        BoundaryValue::scalar(0.0),
    );
    let flux_surface = BTreeMap::new();
    let request = HillslopeKernelRequest::with_phase_context(
        "evapotranspiration",
        HillslopeKernelPhaseClass::HydrologyEvapotranspiration,
        HillslopeConsumerAdapter::Watbal,
        None,
        &state_surface,
        &flux_surface,
    );

    let mut kernel = Wb11HydrologyKernel;
    let response = kernel.run_hillslope_phase(&request);

    assert_eq!(
        response.status.message_id(),
        "HKERNEL-WB11-ET-E-003",
        "material negative PMET Es must fail closed instead of publishing signed Es"
    );
    assert!(response.writeback.flux_updates.is_empty());
    assert!(response.writeback.state_updates.is_empty());
}

#[test]
fn hphys0264_pmet_evapotranspiration_snaps_roundoff_negative_soil_evaporation() {
    let mut state_surface = BTreeMap::new();
    state_surface.insert(
        BoundarySymbol::from("wb11_soil_water"),
        BoundaryValue::scalar(0.010),
    );
    state_surface.insert(
        BoundarySymbol::from("wb11_et_demand"),
        BoundaryValue::scalar(0.001),
    );
    state_surface.insert(BoundarySymbol::from("lai"), BoundaryValue::scalar(2.4));
    state_surface.insert(BoundarySymbol::from("cancov"), BoundaryValue::scalar(0.10));
    state_surface.insert(
        BoundarySymbol::from("wb17_residue_interception"),
        BoundaryValue::scalar(0.0),
    );
    state_surface.insert(
        BoundarySymbol::from("wb11_et_seed_branch_evappm"),
        BoundaryValue::scalar(1.0),
    );
    state_surface.insert(
        BoundarySymbol::from("pmet.es_m"),
        BoundaryValue::scalar(-1.0e-13),
    );
    state_surface.insert(
        BoundarySymbol::from("pmet.ep_m"),
        BoundaryValue::scalar(0.001),
    );
    state_surface.insert(BoundarySymbol::from("nsl"), BoundaryValue::scalar(1.0));
    state_surface.insert(
        BoundarySymbol::from("wb18_perc_theta_0001"),
        BoundaryValue::scalar(0.010),
    );
    state_surface.insert(
        BoundarySymbol::from("wb18_perc_ul_0001"),
        BoundaryValue::scalar(1.0),
    );
    state_surface.insert(BoundarySymbol::from("dg_0001"), BoundaryValue::scalar(0.20));
    state_surface.insert(
        BoundarySymbol::from("thetdr_0001"),
        BoundaryValue::scalar(0.0),
    );
    let flux_surface = BTreeMap::new();
    let request = HillslopeKernelRequest::with_phase_context(
        "evapotranspiration",
        HillslopeKernelPhaseClass::HydrologyEvapotranspiration,
        HillslopeConsumerAdapter::Watbal,
        None,
        &state_surface,
        &flux_surface,
    );

    let mut kernel = Wb11HydrologyKernel;
    let response = kernel.run_hillslope_phase(&request);

    assert_eq!(response.status.message_id(), "HKERNEL-WB11-ET-OK-001");
    let es = flux_update_scalar(&response.writeback.flux_updates, "Es")
        .expect("PMET seam must publish Es");
    assert!(
        es.abs() < f64::EPSILON,
        "near-zero negative PMET Es roundoff must canonicalize to zero"
    );
}

#[test]
fn hphys0281_pmet_evapotranspiration_applies_condensation_storage_return() {
    let mut state_surface = BTreeMap::new();
    state_surface.insert(
        BoundarySymbol::from("wb11_soil_water"),
        BoundaryValue::scalar(0.162),
    );
    state_surface.insert(
        BoundarySymbol::from("wb11_et_demand"),
        BoundaryValue::scalar(0.001),
    );
    state_surface.insert(BoundarySymbol::from("lai"), BoundaryValue::scalar(2.4));
    state_surface.insert(BoundarySymbol::from("cancov"), BoundaryValue::scalar(0.10));
    state_surface.insert(
        BoundarySymbol::from("wb17_residue_interception"),
        BoundaryValue::scalar(0.000_2),
    );
    state_surface.insert(
        BoundarySymbol::from("wb11_et_seed_branch_evappm"),
        BoundaryValue::scalar(1.0),
    );
    state_surface.insert(
        BoundarySymbol::from("pmet.es_m"),
        BoundaryValue::scalar(0.0),
    );
    state_surface.insert(
        BoundarySymbol::from("pmet.es_storage_return_m"),
        BoundaryValue::scalar(0.000_3),
    );
    state_surface.insert(
        BoundarySymbol::from("pmet.ep_m"),
        BoundaryValue::scalar(0.001),
    );
    state_surface.insert(BoundarySymbol::from("nsl"), BoundaryValue::scalar(2.0));
    state_surface.insert(
        BoundarySymbol::from("wb18_perc_theta_0001"),
        BoundaryValue::scalar(0.050),
    );
    state_surface.insert(
        BoundarySymbol::from("wb18_perc_theta_0002"),
        BoundaryValue::scalar(0.100),
    );
    state_surface.insert(
        BoundarySymbol::from("wb18_perc_ul_0001"),
        BoundaryValue::scalar(1.0),
    );
    state_surface.insert(
        BoundarySymbol::from("wb18_perc_ul_0002"),
        BoundaryValue::scalar(1.0),
    );
    state_surface.insert(BoundarySymbol::from("dg_0001"), BoundaryValue::scalar(0.05));
    state_surface.insert(BoundarySymbol::from("dg_0002"), BoundaryValue::scalar(0.20));
    state_surface.insert(
        BoundarySymbol::from("thetdr_0001"),
        BoundaryValue::scalar(0.04),
    );
    state_surface.insert(
        BoundarySymbol::from("thetdr_0002"),
        BoundaryValue::scalar(0.05),
    );
    let flux_surface = BTreeMap::new();
    let request = HillslopeKernelRequest::with_phase_context(
        "evapotranspiration",
        HillslopeKernelPhaseClass::HydrologyEvapotranspiration,
        HillslopeConsumerAdapter::Watbal,
        None,
        &state_surface,
        &flux_surface,
    );

    let mut kernel = Wb11HydrologyKernel;
    let response = kernel.run_hillslope_phase(&request);

    assert_eq!(response.status.message_id(), "HKERNEL-WB11-ET-OK-001");
    let theta_0001 = state_update_scalar(&response.writeback.state_updates, "wb18_perc_theta_0001")
        .expect("PMET condensation return must update top-layer storage");
    let es = flux_update_scalar(&response.writeback.flux_updates, "Es")
        .expect("PMET seam must publish Es");
    let er = flux_update_scalar(&response.writeback.flux_updates, "Er")
        .expect("PMET seam must publish Er");

    assert!(
        (theta_0001 - 0.050_5).abs() < 1.0e-12,
        "top-layer storage must include explicit condensation return plus residue return"
    );
    assert!(
        es.abs() < f64::EPSILON,
        "zero PMET Es must not trigger soil extraction"
    );
    assert!(
        er.abs() < f64::EPSILON,
        "zero PMET Es under residue interception must return residue to storage instead of evaporating it"
    );
}

#[test]
fn hphys0250_wb11_growth_transition_publishes_state_after_for_ep_lineage() {
    let state_surface = BTreeMap::new();
    let flux_surface = BTreeMap::new();
    let state_after = HillslopeGrowthStateSurface {
        sumgdd: 42.0,
        vdmt: 1.25,
        cancov: 0.45,
        lai: 1.8,
        rtmass: 0.75,
        rtd: 0.62,
        hia: 0.2,
    };
    let context =
        HillslopeGrowthKernelContext::new(HillslopeGrowthManagementClass::Perennial, 1.0, 1.0)
            .with_transition_payload(HillslopeGrowthTransitionPayload {
                active_slot_index: 1,
                active_crop_slot_index: 1,
                runtime_day_of_year: 150,
                state_before: HillslopeGrowthStateSurface {
                    sumgdd: 40.0,
                    vdmt: 1.0,
                    cancov: 0.3,
                    lai: 1.0,
                    rtmass: 0.5,
                    rtd: 0.25,
                    hia: 0.1,
                },
                state_after,
                control: HillslopeGrowthTransitionControl::Perennial(
                    HillslopePerennialGrowthControl {
                        jdharv: 0,
                        jdplt: 0,
                        jdstop: 0,
                        mgtopt: 3,
                        rw: 1.0,
                        active_action: HillslopePerennialGrowthAction::None,
                    },
                ),
            });
    let request = HillslopeKernelRequest::with_phase_context(
        "perennial_growth_transition",
        HillslopeKernelPhaseClass::GrowthPerennialTransition,
        HillslopeConsumerAdapter::Growth,
        Some(context),
        &state_surface,
        &flux_surface,
    );

    let mut kernel = Wb11HydrologyKernel;
    let response = kernel.run_hillslope_phase(&request);

    assert_eq!(response.status.message_id(), "HKERNEL-WB11-GROWTH-OK-001");
    assert_eq!(
        state_update_scalar(&response.writeback.state_updates, "sumgdd"),
        Some(state_after.sumgdd)
    );
    assert_eq!(
        state_update_scalar(&response.writeback.state_updates, "vdmt"),
        Some(state_after.vdmt)
    );
    assert_eq!(
        state_update_scalar(&response.writeback.state_updates, "cancov"),
        Some(state_after.cancov)
    );
    assert_eq!(
        state_update_scalar(&response.writeback.state_updates, "lai"),
        Some(state_after.lai)
    );
    assert_eq!(
        state_update_scalar(&response.writeback.state_updates, "rtmass"),
        Some(state_after.rtmass)
    );
    assert_eq!(
        state_update_scalar(&response.writeback.state_updates, "rtd"),
        Some(state_after.rtd)
    );
    assert_eq!(
        state_update_scalar(&response.writeback.state_updates, "hia"),
        Some(state_after.hia)
    );
    assert!(
        !response.writeback.state_updates.is_empty(),
        "growth transition must not NOP after scheduler computes state_after"
    );
}

#[test]
fn hphys0250_wb11_decomposition_transition_publishes_seed_surface() {
    let state_surface = BTreeMap::new();
    let flux_surface = BTreeMap::new();
    let context = HillslopeDecompositionKernelContext::new(
        HillslopeDecompositionManagementClass::Perennial,
        1.0,
        1.0,
    )
    .with_transition_payload(HillslopeDecompositionTransitionPayload {
        active_slot_index: 1,
        active_crop_slot_index: 1,
        runtime_day_of_year: 150,
        iresd_seed: 3.0,
        sumrtm_seed: 2.25,
        sumsrm_seed: 1.75,
        control: HillslopeDecompositionTransitionControl::Perennial(
            HillslopePerennialDecompositionControl {
                mgtopt: 3,
                ncut: 0,
                ncycle: 0,
                active_action: HillslopePerennialDecompositionAction::None,
                active_grazing_cycle: None,
            },
        ),
    });
    let request = HillslopeKernelRequest::with_transition_context(
        "decomposition_transition",
        HillslopeKernelPhaseClass::DecompositionTransition,
        HillslopeConsumerAdapter::Decomposition,
        Some(context),
        None,
        &state_surface,
        &flux_surface,
    );

    let mut kernel = Wb11HydrologyKernel;
    let response = kernel.run_hillslope_phase(&request);

    assert_eq!(response.status.message_id(), "HKERNEL-WB11-DECOMP-OK-001");
    assert_eq!(
        state_update_scalar(&response.writeback.state_updates, "iresd_seed"),
        Some(3.0)
    );
    assert_eq!(
        state_update_scalar(&response.writeback.state_updates, "sumrtm_seed"),
        Some(2.25)
    );
    assert_eq!(
        state_update_scalar(&response.writeback.state_updates, "sumsrm_seed"),
        Some(1.75)
    );
    assert!(
        !response.writeback.state_updates.is_empty(),
        "decomposition transition must not NOP after scheduler computes seed surface"
    );
}
