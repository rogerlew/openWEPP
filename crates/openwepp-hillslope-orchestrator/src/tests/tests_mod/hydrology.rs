use super::fixtures::*;
use super::*;

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
