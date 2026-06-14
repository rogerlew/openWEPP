pub(super) use openwepp_hillslope_orchestrator::{
    HillslopePhase, HillslopePhaseScheduler, HillslopeWritebackSurface, Wb11HydrologyKernel,
};
pub(super) use openwepp_kernel_contract::{
    BoundarySymbol, BoundaryValue, HillslopeConsumerAdapter, HillslopeKernel,
    HillslopeKernelPhaseClass, HillslopeKernelRequest, KernelRunResponse,
};
pub(super) use openwepp_sim_contract::status::BoundaryClass;
pub(super) use openwepp_topology::{parse_topology_fixture_str, validate_pre_execution_topology};

pub(super) const VALID_TOPOLOGY: &str = r"
HILLSLOPES 3
CHANNELS 2
IMPOUNDMENTS 1
NODE CHANNEL 1 H 1 2 0 C 0 0 0 I 0 0 0
NODE CHANNEL 2 H 3 0 0 C 1 0 0 I 0 0 0
NODE IMPOUNDMENT 1 H 0 0 0 C 2 0 0 I 0 0 0
";

pub(super) const CLIM06_TEST_TOLERANCE: f64 = 1.0e-6;
pub(super) const EXPECTED_DTHAW: f64 = 0.0;
pub(super) const EXPECTED_NFT: f64 = 1.0;
pub(super) const DEFAULT_MONTHLY_TMAX_C: [f64; 12] = [
    5.0, 7.0, 11.0, 16.0, 21.0, 25.0, 27.0, 26.0, 22.0, 16.0, 10.0, 6.0,
];
pub(super) const DEFAULT_MONTHLY_TMIN_C: [f64; 12] = [
    -4.0, -2.0, 1.0, 5.0, 9.0, 13.0, 15.0, 14.0, 10.0, 5.0, 1.0, -3.0,
];

#[allow(clippy::too_many_lines)]
pub(super) fn seeded_clim06_surface(active_frost: bool) -> HillslopeWritebackSurface {
    let mut state_surface = std::collections::BTreeMap::new();

    state_surface.insert(BoundarySymbol::from("nsl"), BoundaryValue::scalar(2.0));
    state_surface.insert(BoundarySymbol::from("solthk"), BoundaryValue::scalar(0.3));
    state_surface.insert(BoundarySymbol::from("day"), BoundaryValue::scalar(15.0));
    state_surface.insert(BoundarySymbol::from("mon"), BoundaryValue::scalar(1.0));
    state_surface.insert(BoundarySymbol::from("year"), BoundaryValue::scalar(1.0));
    for (month_index, (obmaxt, obmint)) in DEFAULT_MONTHLY_TMAX_C
        .iter()
        .zip(DEFAULT_MONTHLY_TMIN_C.iter())
        .enumerate()
    {
        let month = month_index + 1;
        state_surface.insert(
            BoundarySymbol::from(format!("obmaxt_{month:04}")),
            BoundaryValue::scalar(*obmaxt),
        );
        state_surface.insert(
            BoundarySymbol::from(format!("obmint_{month:04}")),
            BoundaryValue::scalar(*obmint),
        );
    }
    state_surface.insert(
        BoundarySymbol::from("solwpv"),
        BoundaryValue::scalar(2006.0),
    );
    state_surface.insert(BoundarySymbol::from("dg"), BoundaryValue::scalar(2.0));
    state_surface.insert(BoundarySymbol::from("thetdr"), BoundaryValue::scalar(0.0));
    state_surface.insert(BoundarySymbol::from("thetfc"), BoundaryValue::scalar(1.0));
    state_surface.insert(BoundarySymbol::from("ssc"), BoundaryValue::scalar(0.5));
    state_surface.insert(BoundarySymbol::from("cancov"), BoundaryValue::scalar(0.0));
    state_surface.insert(BoundarySymbol::from("lai"), BoundaryValue::scalar(0.0));
    state_surface.insert(BoundarySymbol::from("vdmt"), BoundaryValue::scalar(0.0));
    state_surface.insert(BoundarySymbol::from("rtd"), BoundaryValue::scalar(0.0));
    state_surface.insert(BoundarySymbol::from("pltol"), BoundaryValue::scalar(0.25));

    state_surface.insert(
        BoundarySymbol::from("wb11_soil_water"),
        BoundaryValue::scalar(12.0),
    );
    state_surface.insert(
        BoundarySymbol::from("wb11_et_demand"),
        BoundaryValue::scalar(2.0),
    );
    state_surface.insert(
        BoundarySymbol::from("wb17_residue_interception"),
        BoundaryValue::scalar(0.0),
    );
    state_surface.insert(
        BoundarySymbol::from("wb11_field_capacity"),
        BoundaryValue::scalar(8.0),
    );
    state_surface.insert(
        BoundarySymbol::from("wb11_perc_fraction"),
        BoundaryValue::scalar(0.5),
    );
    // WB18 per-layer percolation inputs (WB11 compatibility lane).
    state_surface.insert(
        BoundarySymbol::from("wb18_perc_theta_0001"),
        BoundaryValue::scalar(5.0),
    );
    state_surface.insert(
        BoundarySymbol::from("wb18_perc_fc_0001"),
        BoundaryValue::scalar(5.0),
    );
    state_surface.insert(
        BoundarySymbol::from("thetfc_0001"),
        BoundaryValue::scalar(50.0),
    );
    state_surface.insert(
        BoundarySymbol::from("thetdr_0001"),
        BoundaryValue::scalar(0.0),
    );
    state_surface.insert(
        BoundarySymbol::from("wb18_perc_ul_0001"),
        BoundaryValue::scalar(8.0),
    );
    state_surface.insert(
        BoundarySymbol::from("wb18_perc_ssc_0001"),
        BoundaryValue::scalar(2.0e-6),
    );
    state_surface.insert(
        BoundarySymbol::from("wb18_perc_theta_0002"),
        BoundaryValue::scalar(5.0),
    );
    state_surface.insert(
        BoundarySymbol::from("wb18_perc_fc_0002"),
        BoundaryValue::scalar(4.0),
    );
    state_surface.insert(
        BoundarySymbol::from("thetfc_0002"),
        BoundaryValue::scalar(40.0),
    );
    state_surface.insert(
        BoundarySymbol::from("thetdr_0002"),
        BoundaryValue::scalar(0.0),
    );
    state_surface.insert(
        BoundarySymbol::from("wb18_perc_ul_0002"),
        BoundaryValue::scalar(8.0),
    );
    state_surface.insert(
        BoundarySymbol::from("wb18_perc_ssc_0002"),
        BoundaryValue::scalar(2.0e-5),
    );
    state_surface.insert(BoundarySymbol::from("dg_0001"), BoundaryValue::scalar(0.1));
    state_surface.insert(BoundarySymbol::from("dg_0002"), BoundaryValue::scalar(0.1));
    state_surface.insert(
        BoundarySymbol::from("wb19_bulk_density_kg_m3_0001"),
        BoundaryValue::scalar(1_300.0),
    );
    state_surface.insert(
        BoundarySymbol::from("wb19_bulk_density_kg_m3_0002"),
        BoundaryValue::scalar(1_300.0),
    );
    state_surface.insert(BoundarySymbol::from("por_0001"), BoundaryValue::scalar(0.8));
    state_surface.insert(BoundarySymbol::from("por_0002"), BoundaryValue::scalar(0.8));
    state_surface.insert(BoundarySymbol::from("cpm_0001"), BoundaryValue::scalar(1.0));
    state_surface.insert(
        BoundarySymbol::from("coca_0001"),
        BoundaryValue::scalar(1.0),
    );
    state_surface.insert(BoundarySymbol::from("cpm_0002"), BoundaryValue::scalar(1.0));
    state_surface.insert(
        BoundarySymbol::from("coca_0002"),
        BoundaryValue::scalar(1.0),
    );
    state_surface.insert(BoundarySymbol::from("avgslp"), BoundaryValue::scalar(0.1));
    state_surface.insert(BoundarySymbol::from("slplen"), BoundaryValue::scalar(1.0));
    state_surface.insert(BoundarySymbol::from("nslpts"), BoundaryValue::scalar(2.0));
    state_surface.insert(
        BoundarySymbol::from("xinput_0001"),
        BoundaryValue::scalar(0.0),
    );
    state_surface.insert(
        BoundarySymbol::from("slpinp_0001"),
        BoundaryValue::scalar(0.0),
    );
    state_surface.insert(
        BoundarySymbol::from("wb19_lateral_anisotropy_ratio"),
        BoundaryValue::scalar(39.653_865_297_983_295),
    );
    state_surface.insert(
        BoundarySymbol::from("wb19_drain_enabled"),
        BoundaryValue::scalar(1.0),
    );
    state_surface.insert(
        BoundarySymbol::from("wb19_drain_depth"),
        BoundaryValue::scalar(0.15),
    );
    state_surface.insert(
        BoundarySymbol::from("wb19_drain_spacing"),
        BoundaryValue::scalar(0.285),
    );
    state_surface.insert(
        BoundarySymbol::from("wb19_drain_diameter"),
        BoundaryValue::scalar(0.1),
    );
    state_surface.insert(
        BoundarySymbol::from("wb11_lateral_fraction"),
        BoundaryValue::scalar(0.25),
    );
    state_surface.insert(
        BoundarySymbol::from("wb11_drainage_fraction"),
        BoundaryValue::scalar(0.5),
    );
    state_surface.insert(
        BoundarySymbol::from("wb11_drainage_coefficient"),
        BoundaryValue::scalar(1.0),
    );
    state_surface.insert(
        BoundarySymbol::from("wb11_drainable_storage"),
        BoundaryValue::scalar(2.0),
    );

    state_surface.insert(
        BoundarySymbol::from("wb12_rainfall_input"),
        BoundaryValue::scalar(3.0),
    );
    state_surface.insert(
        BoundarySymbol::from("wb12_runon_input"),
        BoundaryValue::scalar(0.4),
    );
    state_surface.insert(
        BoundarySymbol::from("wb12_infiltration"),
        BoundaryValue::scalar(0.0),
    );
    state_surface.insert(
        BoundarySymbol::from("wb12_depression_storage_delta"),
        BoundaryValue::scalar(0.2),
    );
    state_surface.insert(
        BoundarySymbol::from("wb12_runoff_observed"),
        BoundaryValue::scalar(0.0),
    );
    state_surface.insert(
        BoundarySymbol::from("wb12_runoff_closure_tolerance"),
        BoundaryValue::scalar(20.0),
    );

    state_surface.insert(
        BoundarySymbol::from("wb12_storage_initial"),
        BoundaryValue::scalar(12.0),
    );
    state_surface.insert(
        BoundarySymbol::from("wb12_storage_observed"),
        BoundaryValue::scalar(0.0),
    );
    state_surface.insert(
        BoundarySymbol::from("wb12_storage_closure_tolerance"),
        BoundaryValue::scalar(20.0),
    );
    state_surface.insert(
        BoundarySymbol::from("wb12_precip_input"),
        BoundaryValue::scalar(3.0),
    );

    state_surface.insert(BoundarySymbol::from("ninten"), BoundaryValue::scalar(4.0));
    state_surface.insert(
        BoundarySymbol::from("timem_0001"),
        BoundaryValue::scalar(0.0),
    );
    state_surface.insert(
        BoundarySymbol::from("timem_0002"),
        BoundaryValue::scalar(1.0),
    );
    state_surface.insert(
        BoundarySymbol::from("timem_0003"),
        BoundaryValue::scalar(2.0),
    );
    state_surface.insert(
        BoundarySymbol::from("timem_0004"),
        BoundaryValue::scalar(3.0),
    );
    state_surface.insert(
        BoundarySymbol::from("intsty_0001"),
        BoundaryValue::scalar(1.0),
    );
    state_surface.insert(
        BoundarySymbol::from("intsty_0002"),
        BoundaryValue::scalar(1.0),
    );
    state_surface.insert(
        BoundarySymbol::from("intsty_0003"),
        BoundaryValue::scalar(1.0),
    );
    state_surface.insert(
        BoundarySymbol::from("intsty_0004"),
        BoundaryValue::scalar(0.0),
    );
    state_surface.insert(BoundarySymbol::from("timep"), BoundaryValue::scalar(0.25));
    state_surface.insert(BoundarySymbol::from("efflen"), BoundaryValue::scalar(2.0));
    state_surface.insert(BoundarySymbol::from("ealpha"), BoundaryValue::scalar(1.0));
    state_surface.insert(BoundarySymbol::from("m"), BoundaryValue::scalar(1.5));

    state_surface.insert(
        BoundarySymbol::from("frost.options.wintRed"),
        BoundaryValue::scalar(if active_frost { 1.0 } else { 0.0 }),
    );
    state_surface.insert(
        BoundarySymbol::from("frost.options.fineTop"),
        BoundaryValue::scalar(10.0),
    );
    state_surface.insert(
        BoundarySymbol::from("frost.options.fineBot"),
        BoundaryValue::scalar(10.0),
    );
    state_surface.insert(
        BoundarySymbol::from("frost.options.ksnowf"),
        BoundaryValue::scalar(1.0),
    );
    state_surface.insert(
        BoundarySymbol::from("frost.options.kresf"),
        BoundaryValue::scalar(1.0),
    );
    state_surface.insert(
        BoundarySymbol::from("frost.options.ksoilf"),
        BoundaryValue::scalar(1.0),
    );
    state_surface.insert(
        BoundarySymbol::from("frost.options.kfactor1"),
        BoundaryValue::scalar(0.2),
    );
    state_surface.insert(
        BoundarySymbol::from("frost.options.kfactor2"),
        BoundaryValue::scalar(0.4),
    );
    state_surface.insert(
        BoundarySymbol::from("frost.options.kfactor3"),
        BoundaryValue::scalar(0.5),
    );
    state_surface.insert(
        BoundarySymbol::from("frost.options.frost_file_present"),
        BoundaryValue::scalar(if active_frost { 1.0 } else { 0.0 }),
    );
    state_surface.insert(
        BoundarySymbol::from("snow.runtime_depth_m"),
        BoundaryValue::scalar(0.0),
    );
    state_surface.insert(
        BoundarySymbol::from("snow.runtime_swe"),
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
    state_surface.insert(
        BoundarySymbol::from("frost.runtime_residue_depth_m"),
        BoundaryValue::scalar(0.0),
    );
    state_surface.insert(BoundarySymbol::from("vwind"), BoundaryValue::scalar(2.0));
    state_surface.insert(BoundarySymbol::from("tdpt"), BoundaryValue::scalar(-8.0));
    state_surface.insert(BoundarySymbol::from("salb"), BoundaryValue::scalar(0.2));
    state_surface.insert(BoundarySymbol::from("canhgt"), BoundaryValue::scalar(0.0));
    state_surface.insert(BoundarySymbol::from("rrinit"), BoundaryValue::scalar(0.01));
    for hour in 1..=24 {
        state_surface.insert(
            BoundarySymbol::from(format!("winter.hourly.air_temp_c_{hour:04}")),
            BoundaryValue::scalar(-8.086),
        );
        state_surface.insert(
            BoundarySymbol::from(format!("winter.hourly.rad_mj_m2_{hour:04}")),
            BoundaryValue::scalar(0.0),
        );
        state_surface.insert(
            BoundarySymbol::from(format!("winter.hourly.cloud_fraction_{hour:04}")),
            BoundaryValue::scalar(1.0),
        );
    }
    state_surface.insert(BoundarySymbol::from("tmax"), BoundaryValue::scalar(-2.0));
    state_surface.insert(BoundarySymbol::from("tmin"), BoundaryValue::scalar(-10.0));

    HillslopeWritebackSurface {
        state_surface,
        flux_surface: std::collections::BTreeMap::new(),
    }
}

pub(super) fn execute_clim06_surface(
    surface: HillslopeWritebackSurface,
) -> openwepp_hillslope_orchestrator::HillslopeKernelExecutionReport {
    let graph = parse_topology_fixture_str(VALID_TOPOLOGY).expect("fixture should parse");
    let topology_report =
        validate_pre_execution_topology(&graph).expect("topology report should build");
    let scheduler = HillslopePhaseScheduler::canonical();
    let mut kernel = Wb11HydrologyKernel;
    scheduler
        .execute_with_kernel(&topology_report, &mut kernel, surface)
        .expect("CLIM06 execution should return typed report")
}

pub(super) fn execute_clim06_runoff_phase(
    surface: &HillslopeWritebackSurface,
) -> KernelRunResponse {
    let request = HillslopeKernelRequest::with_phase_context(
        "runoff_reconciliation",
        HillslopeKernelPhaseClass::HydrologyRunoffReconciliation,
        HillslopeConsumerAdapter::Runoff,
        None,
        &surface.state_surface,
        &surface.flux_surface,
    );
    let mut kernel = Wb11HydrologyKernel;
    kernel.run_hillslope_phase(&request)
}

pub(super) fn require_state_scalar(
    report: &openwepp_hillslope_orchestrator::HillslopeKernelExecutionReport,
    symbol: &str,
) -> f64 {
    report
        .writeback_surface
        .state_surface
        .get(&BoundarySymbol::from(symbol))
        .unwrap_or_else(|| panic!("missing expected state symbol {symbol}"))
        .as_f64()
}

pub(super) fn require_response_state_update(response: &KernelRunResponse, symbol: &str) -> f64 {
    response
        .writeback
        .state_updates
        .iter()
        .find(|field| field.symbol == BoundarySymbol::from(symbol))
        .unwrap_or_else(|| panic!("missing expected state update {symbol}"))
        .value
        .as_f64()
}

pub(super) fn insert_state_scalar(
    surface: &mut HillslopeWritebackSurface,
    symbol: &str,
    value: f64,
) {
    surface
        .state_surface
        .insert(BoundarySymbol::from(symbol), BoundaryValue::scalar(value));
}

pub(super) fn set_winter_hourly_forcing(
    surface: &mut HillslopeWritebackSurface,
    air_temp_c: f64,
    rad_mj_m2: f64,
    cloud_fraction: f64,
) {
    for hour in 1..=24 {
        insert_state_scalar(
            surface,
            &format!("winter.hourly.air_temp_c_{hour:04}"),
            air_temp_c,
        );
        insert_state_scalar(
            surface,
            &format!("winter.hourly.rad_mj_m2_{hour:04}"),
            rad_mj_m2,
        );
        insert_state_scalar(
            surface,
            &format!("winter.hourly.cloud_fraction_{hour:04}"),
            cloud_fraction,
        );
    }
}

pub(super) fn set_neutral_tmpadj_hourly_forcing(surface: &mut HillslopeWritebackSurface) {
    let atmospheric_emissivity = (1.0 - 0.84) * (1.0 - 0.261) + 0.84;
    let longwave_deficit_w_m2 = (1.0 - atmospheric_emissivity) * 5.6697e-8 * 273.16_f64.powi(4);
    let hourly_rad_mj_m2 = (longwave_deficit_w_m2 / 0.8) * 3_600.0 / 1.0e6;
    set_winter_hourly_forcing(surface, 0.0, hourly_rad_mj_m2, 1.0);
}

pub(super) fn override_monthly_temperatures(
    surface: &mut HillslopeWritebackSurface,
    monthly_mean_c: f64,
) {
    for month in 1..=12 {
        insert_state_scalar(surface, &format!("obmaxt_{month:04}"), monthly_mean_c);
        insert_state_scalar(surface, &format!("obmint_{month:04}"), monthly_mean_c);
    }
}

pub(super) fn remove_state_prefixes(surface: &mut HillslopeWritebackSurface, prefixes: &[&str]) {
    surface.state_surface.retain(|symbol, _| {
        !prefixes
            .iter()
            .any(|prefix| symbol.as_str().starts_with(prefix))
    });
}

pub(super) fn fine_frost_symbol(root: &str, layer_index: usize, fine_index: usize) -> String {
    format!("{root}_{layer_index:04}_{fine_index:04}")
}

#[allow(clippy::too_many_lines)]
pub(super) fn seed_increment_a_shadow_fine_state(
    surface: &mut HillslopeWritebackSurface,
    yst_offset_m: f64,
) {
    insert_state_scalar(surface, "wb11_et_demand", 0.0);
    insert_state_scalar(surface, "wb11_perc_fraction", 0.5);
    insert_state_scalar(surface, "wb11_field_capacity", 12.0);
    insert_state_scalar(surface, "wb18_perc_fc_0001", 10.0);
    insert_state_scalar(surface, "wb18_perc_fc_0002", 10.0);
    insert_state_scalar(surface, "wb18_perc_ul_0001", 20.0);
    insert_state_scalar(surface, "wb18_perc_ul_0002", 20.0);
    insert_state_scalar(surface, "wb19_drain_enabled", 0.0);
    insert_state_scalar(surface, "wb11_lateral_fraction", 0.0);
    insert_state_scalar(surface, "wb11_drainage_fraction", 0.0);
    insert_state_scalar(surface, "wb12_rainfall_input", 0.0);
    insert_state_scalar(surface, "wb12_runon_input", 0.0);
    insert_state_scalar(surface, "wb12_precip_input", 0.0);
    insert_state_scalar(surface, "frost.runtime_ws_frz", 0.012);
    insert_state_scalar(surface, "frost.runtime_dfrost", 0.030);
    insert_state_scalar(surface, "frost.runtime_frdp_m", 0.030);
    insert_state_scalar(surface, "wb18_perc_frozen_depth_0001", 0.030);
    insert_state_scalar(surface, "wb18_perc_frzw_0001", 0.012);
    insert_state_scalar(surface, "wb18_perc_frozen_depth_0002", 0.0);
    insert_state_scalar(surface, "wb18_perc_frzw_0002", 0.0);
    insert_state_scalar(surface, "frost.runtime_yst_m_0001", 5.0 - yst_offset_m);
    insert_state_scalar(surface, "frost.runtime_yst_m_0002", 5.0);
    insert_state_scalar(surface, "frost.runtime_nwfrzz_m_0001", 0.0);
    insert_state_scalar(surface, "frost.runtime_nwfrzz_m_0002", 0.0);

    for fine_index in 1..=10 {
        let layer_1_frozen = fine_index <= 3;
        insert_state_scalar(
            surface,
            &fine_frost_symbol("frost.runtime_fgfrst", 1, fine_index),
            if layer_1_frozen { 1.0 } else { 0.0 },
        );
        insert_state_scalar(
            surface,
            &fine_frost_symbol("frost.runtime_slfsd_m", 1, fine_index),
            if layer_1_frozen { 0.010 } else { 0.0 },
        );
        insert_state_scalar(
            surface,
            &fine_frost_symbol("frost.runtime_slsic_m", 1, fine_index),
            if layer_1_frozen { 0.004 } else { 0.0 },
        );
        insert_state_scalar(
            surface,
            &fine_frost_symbol("frost.runtime_slsw_theta", 1, fine_index),
            if layer_1_frozen { 0.0 } else { 0.2 },
        );
        insert_state_scalar(
            surface,
            &fine_frost_symbol("frost.runtime_sltime_s", 1, fine_index),
            0.0,
        );

        insert_state_scalar(
            surface,
            &fine_frost_symbol("frost.runtime_fgfrst", 2, fine_index),
            0.0,
        );
        insert_state_scalar(
            surface,
            &fine_frost_symbol("frost.runtime_slfsd_m", 2, fine_index),
            0.0,
        );
        insert_state_scalar(
            surface,
            &fine_frost_symbol("frost.runtime_slsic_m", 2, fine_index),
            0.0,
        );
        insert_state_scalar(
            surface,
            &fine_frost_symbol("frost.runtime_slsw_theta", 2, fine_index),
            0.15,
        );
        insert_state_scalar(
            surface,
            &fine_frost_symbol("frost.runtime_sltime_s", 2, fine_index),
            0.0,
        );
    }
}

pub(super) fn frozen_layer_frzw_sum(
    report: &openwepp_hillslope_orchestrator::HillslopeKernelExecutionReport,
) -> f64 {
    require_state_scalar(report, "wb18_perc_frzw_0001")
        + require_state_scalar(report, "wb18_perc_frzw_0002")
}

pub(super) fn response_fine_layer_sum(
    response: &KernelRunResponse,
    root: &str,
    layer_index: usize,
    fine_count: usize,
) -> f64 {
    (1..=fine_count)
        .map(|fine_index| {
            require_response_state_update(
                response,
                &fine_frost_symbol(root, layer_index, fine_index),
            )
        })
        .sum()
}

pub(super) fn response_fine_flag_count(response: &KernelRunResponse, expected_flag: f64) -> usize {
    (1..=2)
        .flat_map(|layer_index| {
            (1..=10).map(move |fine_index| {
                require_response_state_update(
                    response,
                    &fine_frost_symbol("frost.runtime_fgfrst", layer_index, fine_index),
                )
            })
        })
        .filter(|flag| (*flag - expected_flag).abs() <= CLIM06_TEST_TOLERANCE)
        .count()
}

pub(super) fn layer_frozen_depth_sum(
    report: &openwepp_hillslope_orchestrator::HillslopeKernelExecutionReport,
) -> f64 {
    require_state_scalar(report, "wb18_perc_frozen_depth_0001")
        + require_state_scalar(report, "wb18_perc_frozen_depth_0002")
}

pub(super) fn configure_fdhp01_deep_profile(surface: &mut HillslopeWritebackSurface) {
    insert_state_scalar(surface, "dg_0002", 0.20);
    insert_state_scalar(surface, "wb18_perc_fc_0002", 8.0);
    insert_state_scalar(surface, "wb19_drain_enabled", 0.0);
    insert_state_scalar(surface, "wb11_lateral_fraction", 0.0);
    insert_state_scalar(surface, "wb11_drainage_fraction", 0.0);
}

pub(super) fn configure_fdhp01_frost_only_no_flux(surface: &mut HillslopeWritebackSurface) {
    insert_state_scalar(surface, "wb11_et_demand", 0.0);
    insert_state_scalar(surface, "wb11_perc_fraction", 0.0);
    insert_state_scalar(surface, "wb19_drain_enabled", 0.0);
    insert_state_scalar(surface, "wb11_lateral_fraction", 0.0);
    insert_state_scalar(surface, "wb11_drainage_fraction", 0.0);
    insert_state_scalar(surface, "wb12_rainfall_input", 0.0);
    insert_state_scalar(surface, "wb12_runon_input", 0.0);
    insert_state_scalar(surface, "wb12_precip_input", 0.0);
    insert_state_scalar(surface, "wb12_runoff_closure_tolerance", 1000.0);
    insert_state_scalar(surface, "wb12_storage_closure_tolerance", 1000.0);
}

pub(super) fn seed_prior_layered_frost(
    surface: &mut HillslopeWritebackSurface,
    depth_m: f64,
    frzw_m: f64,
) {
    insert_state_scalar(surface, "frost.runtime_frdp_m", depth_m);
    insert_state_scalar(surface, "frost.runtime_dfrost", depth_m);
    insert_state_scalar(surface, "frost.runtime_ws_frz", frzw_m);
    insert_state_scalar(surface, "wb18_perc_frozen_depth_0001", depth_m.min(0.10));
    insert_state_scalar(surface, "wb18_perc_frzw_0001", frzw_m.min(0.10));
    insert_state_scalar(
        surface,
        "wb18_perc_frozen_depth_0002",
        (depth_m - 0.10).max(0.0),
    );
    insert_state_scalar(surface, "wb18_perc_frzw_0002", (frzw_m - 0.10).max(0.0));
}

pub(super) fn seed_db_thin_front_frost(surface: &mut HillslopeWritebackSurface) {
    configure_fdhp01_frost_only_no_flux(surface);
    let initial_depth_m = 0.0004;
    let initial_ice_m = 0.00008;
    let liquid_theta = 0.2;
    let top_liquid_m = liquid_theta * (0.100 - initial_depth_m);
    let bottom_liquid_m = liquid_theta * 0.100;
    insert_state_scalar(surface, "wb11_soil_water", top_liquid_m + bottom_liquid_m);
    insert_state_scalar(surface, "wb18_perc_theta_0001", top_liquid_m);
    insert_state_scalar(surface, "wb18_perc_theta_0002", bottom_liquid_m);
    insert_state_scalar(surface, "wb18_perc_ul_0001", 0.040);
    insert_state_scalar(surface, "wb18_perc_ul_0002", 0.040);
    insert_state_scalar(surface, "frost.runtime_ws_frz", initial_ice_m);
    insert_state_scalar(surface, "frost.runtime_dfrost", initial_depth_m);
    insert_state_scalar(surface, "frost.runtime_frdp_m", initial_depth_m);
    insert_state_scalar(surface, "wb18_perc_frozen_depth_0001", initial_depth_m);
    insert_state_scalar(surface, "wb18_perc_frzw_0001", initial_ice_m);
    insert_state_scalar(surface, "wb18_perc_frozen_depth_0002", 0.0);
    insert_state_scalar(surface, "wb18_perc_frzw_0002", 0.0);
    insert_state_scalar(surface, "frost.runtime_yst_m_0001", top_liquid_m);
    insert_state_scalar(surface, "frost.runtime_yst_m_0002", bottom_liquid_m);
    insert_state_scalar(surface, "frost.runtime_nwfrzz_m_0001", 0.0);
    insert_state_scalar(surface, "frost.runtime_nwfrzz_m_0002", 0.0);

    for fine_index in 1..=10 {
        insert_state_scalar(
            surface,
            &fine_frost_symbol("frost.runtime_fgfrst", 1, fine_index),
            if fine_index == 1 { 2.0 } else { 0.0 },
        );
        insert_state_scalar(
            surface,
            &fine_frost_symbol("frost.runtime_slfsd_m", 1, fine_index),
            if fine_index == 1 {
                initial_depth_m
            } else {
                0.0
            },
        );
        insert_state_scalar(
            surface,
            &fine_frost_symbol("frost.runtime_slsic_m", 1, fine_index),
            if fine_index == 1 { initial_ice_m } else { 0.0 },
        );
        insert_state_scalar(
            surface,
            &fine_frost_symbol("frost.runtime_slsw_theta", 1, fine_index),
            liquid_theta,
        );
        insert_state_scalar(
            surface,
            &fine_frost_symbol("frost.runtime_sltime_s", 1, fine_index),
            0.0,
        );

        insert_state_scalar(
            surface,
            &fine_frost_symbol("frost.runtime_fgfrst", 2, fine_index),
            0.0,
        );
        insert_state_scalar(
            surface,
            &fine_frost_symbol("frost.runtime_slfsd_m", 2, fine_index),
            0.0,
        );
        insert_state_scalar(
            surface,
            &fine_frost_symbol("frost.runtime_slsic_m", 2, fine_index),
            0.0,
        );
        insert_state_scalar(
            surface,
            &fine_frost_symbol("frost.runtime_slsw_theta", 2, fine_index),
            liquid_theta,
        );
        insert_state_scalar(
            surface,
            &fine_frost_symbol("frost.runtime_sltime_s", 2, fine_index),
            0.0,
        );
    }
}

pub(super) fn seed_c2_full_top_layer_frost(surface: &mut HillslopeWritebackSurface) {
    configure_fdhp01_frost_only_no_flux(surface);
    insert_state_scalar(surface, "wb11_soil_water", 0.020);
    insert_state_scalar(surface, "wb18_perc_theta_0001", 0.0);
    insert_state_scalar(surface, "wb18_perc_theta_0002", 0.020);
    insert_state_scalar(surface, "wb18_perc_ul_0001", 0.020);
    insert_state_scalar(surface, "wb18_perc_ul_0002", 0.020);
    insert_state_scalar(surface, "frost.runtime_frdp_m", 0.100);
    insert_state_scalar(surface, "frost.runtime_dfrost", 0.100);
    insert_state_scalar(surface, "frost.runtime_ws_frz", 0.020);
    insert_state_scalar(surface, "wb18_perc_frozen_depth_0001", 0.100);
    insert_state_scalar(surface, "wb18_perc_frzw_0001", 0.020);
    insert_state_scalar(surface, "wb18_perc_frozen_depth_0002", 0.0);
    insert_state_scalar(surface, "wb18_perc_frzw_0002", 0.0);
    insert_state_scalar(surface, "frost.runtime_yst_m_0001", 0.0);
    insert_state_scalar(surface, "frost.runtime_yst_m_0002", 0.020);
    insert_state_scalar(surface, "frost.runtime_nwfrzz_m_0001", 0.0);
    insert_state_scalar(surface, "frost.runtime_nwfrzz_m_0002", 0.0);

    for fine_index in 1..=10 {
        insert_state_scalar(
            surface,
            &fine_frost_symbol("frost.runtime_fgfrst", 1, fine_index),
            1.0,
        );
        insert_state_scalar(
            surface,
            &fine_frost_symbol("frost.runtime_slfsd_m", 1, fine_index),
            0.010,
        );
        insert_state_scalar(
            surface,
            &fine_frost_symbol("frost.runtime_slsic_m", 1, fine_index),
            0.002,
        );
        insert_state_scalar(
            surface,
            &fine_frost_symbol("frost.runtime_slsw_theta", 1, fine_index),
            0.0,
        );
        insert_state_scalar(
            surface,
            &fine_frost_symbol("frost.runtime_sltime_s", 1, fine_index),
            0.0,
        );

        insert_state_scalar(
            surface,
            &fine_frost_symbol("frost.runtime_fgfrst", 2, fine_index),
            0.0,
        );
        insert_state_scalar(
            surface,
            &fine_frost_symbol("frost.runtime_slfsd_m", 2, fine_index),
            0.0,
        );
        insert_state_scalar(
            surface,
            &fine_frost_symbol("frost.runtime_slsic_m", 2, fine_index),
            0.0,
        );
        insert_state_scalar(
            surface,
            &fine_frost_symbol("frost.runtime_slsw_theta", 2, fine_index),
            0.2,
        );
        insert_state_scalar(
            surface,
            &fine_frost_symbol("frost.runtime_sltime_s", 2, fine_index),
            0.0,
        );
    }
}

#[allow(clippy::too_many_lines)]
pub(super) fn seed_de_full_meter_lower_front_profile(
    surface: &mut HillslopeWritebackSurface,
    top_layer_frozen: bool,
) {
    configure_fdhp01_frost_only_no_flux(surface);
    let layer_count = 6usize;
    let layer_count_f64 = 6.0;
    let layer_thickness_m = 0.2;
    let liquid_theta = 0.25;
    let liquid_storage_m = liquid_theta * layer_thickness_m;
    let frozen_storage_m = if top_layer_frozen {
        liquid_storage_m
    } else {
        0.0
    };

    insert_state_scalar(surface, "nsl", layer_count_f64);
    insert_state_scalar(surface, "solthk", layer_count_f64 * layer_thickness_m);
    insert_state_scalar(surface, "wb11_soil_water", liquid_storage_m * 5.0);
    insert_state_scalar(surface, "wb11_field_capacity", 0.5);
    insert_state_scalar(surface, "wb11_drainable_storage", 0.5);
    insert_state_scalar(
        surface,
        "frost.runtime_frdp_m",
        if top_layer_frozen {
            layer_thickness_m
        } else {
            0.0
        },
    );
    insert_state_scalar(
        surface,
        "frost.runtime_dfrost",
        if top_layer_frozen {
            layer_thickness_m
        } else {
            0.0
        },
    );
    insert_state_scalar(surface, "frost.runtime_ws_frz", frozen_storage_m);

    for layer_index in 1..=layer_count {
        let is_frozen_layer = top_layer_frozen && layer_index == 1;
        let theta_m = if is_frozen_layer {
            0.0
        } else {
            liquid_storage_m
        };
        insert_state_scalar(surface, &format!("dg_{layer_index:04}"), layer_thickness_m);
        insert_state_scalar(
            surface,
            &format!("wb19_dg_{layer_index:04}"),
            layer_thickness_m,
        );
        insert_state_scalar(
            surface,
            &format!("wb18_perc_dg_{layer_index:04}"),
            layer_thickness_m,
        );
        insert_state_scalar(
            surface,
            &format!("wb18_perc_theta_{layer_index:04}"),
            theta_m,
        );
        insert_state_scalar(surface, &format!("wb18_perc_fc_{layer_index:04}"), 0.08);
        insert_state_scalar(surface, &format!("wb18_perc_ul_{layer_index:04}"), 0.12);
        insert_state_scalar(surface, &format!("wb18_perc_ssc_{layer_index:04}"), 2.0e-6);
        insert_state_scalar(surface, &format!("thetfc_{layer_index:04}"), 0.4);
        insert_state_scalar(surface, &format!("thetdr_{layer_index:04}"), 0.0);
        insert_state_scalar(surface, &format!("wb19_thetdr_{layer_index:04}"), 0.0);
        insert_state_scalar(surface, &format!("wb19_thetfc_{layer_index:04}"), 0.4);
        insert_state_scalar(surface, &format!("wb18_perc_thetfc_{layer_index:04}"), 0.4);
        insert_state_scalar(
            surface,
            &format!("wb19_bulk_density_kg_m3_{layer_index:04}"),
            1_300.0,
        );
        insert_state_scalar(surface, &format!("por_{layer_index:04}"), 0.8);
        insert_state_scalar(surface, &format!("wb19_por_{layer_index:04}"), 0.8);
        insert_state_scalar(surface, &format!("cpm_{layer_index:04}"), 1.0);
        insert_state_scalar(surface, &format!("coca_{layer_index:04}"), 1.0);
        insert_state_scalar(surface, &format!("wb19_coca_{layer_index:04}"), 1.0);
        insert_state_scalar(surface, &format!("ssc_{layer_index:04}"), 2.0e-6);
        insert_state_scalar(
            surface,
            &format!("wb19_lateral_ssh_{layer_index:04}"),
            2.0e-6,
        );
        insert_state_scalar(
            surface,
            &format!("wb18_perc_frozen_depth_{layer_index:04}"),
            if is_frozen_layer {
                layer_thickness_m
            } else {
                0.0
            },
        );
        insert_state_scalar(
            surface,
            &format!("wb18_perc_frzw_{layer_index:04}"),
            if is_frozen_layer {
                frozen_storage_m
            } else {
                0.0
            },
        );
        insert_state_scalar(
            surface,
            &format!("frost.runtime_yst_m_{layer_index:04}"),
            theta_m,
        );
        insert_state_scalar(
            surface,
            &format!("frost.runtime_nwfrzz_m_{layer_index:04}"),
            0.0,
        );

        for fine_index in 1..=10 {
            insert_state_scalar(
                surface,
                &fine_frost_symbol("frost.runtime_fgfrst", layer_index, fine_index),
                if is_frozen_layer { 1.0 } else { 0.0 },
            );
            insert_state_scalar(
                surface,
                &fine_frost_symbol("frost.runtime_slfsd_m", layer_index, fine_index),
                if is_frozen_layer { 0.020 } else { 0.0 },
            );
            insert_state_scalar(
                surface,
                &fine_frost_symbol("frost.runtime_slsic_m", layer_index, fine_index),
                if is_frozen_layer { 0.005 } else { 0.0 },
            );
            insert_state_scalar(
                surface,
                &fine_frost_symbol("frost.runtime_slsw_theta", layer_index, fine_index),
                if is_frozen_layer { 0.0 } else { liquid_theta },
            );
            insert_state_scalar(
                surface,
                &fine_frost_symbol("frost.runtime_sltime_s", layer_index, fine_index),
                0.0,
            );
        }
    }
}

pub(super) fn apply_response_state_updates(
    surface: &mut HillslopeWritebackSurface,
    response: &KernelRunResponse,
) {
    assert!(
        response.status.ok_flag(),
        "cannot apply failed response: {:?}",
        response.status
    );
    for field in &response.writeback.state_updates {
        surface
            .state_surface
            .insert(field.symbol.clone(), field.value);
    }
    for field in &response.writeback.flux_updates {
        surface
            .flux_surface
            .insert(field.symbol.clone(), field.value);
    }
}

pub(super) fn assert_close(actual: f64, expected: f64, context: &str) {
    assert!(
        (actual - expected).abs() <= CLIM06_TEST_TOLERANCE,
        "{context}: expected {expected}, got {actual}"
    );
}
