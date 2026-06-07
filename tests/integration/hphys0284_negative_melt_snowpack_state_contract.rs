use openwepp_hillslope_orchestrator::{
    HillslopeKernelExecutionReport, HillslopePhaseScheduler, HillslopeWritebackSurface,
    Wb11HydrologyKernel,
};
use openwepp_kernel_contract::{BoundarySymbol, BoundaryValue};
use openwepp_topology::{parse_topology_fixture_str, validate_pre_execution_topology};

const VALID_TOPOLOGY: &str = r"
HILLSLOPES 3
CHANNELS 2
IMPOUNDMENTS 1
NODE CHANNEL 1 H 1 2 0 C 0 0 0 I 0 0 0
NODE CHANNEL 2 H 3 0 0 C 1 0 0 I 0 0 0
NODE IMPOUNDMENT 1 H 0 0 0 C 2 0 0 I 0 0 0
";

const TOL: f64 = 1.0e-9;

fn insert_state(surface: &mut HillslopeWritebackSurface, symbol: &str, value: f64) {
    surface
        .state_surface
        .insert(BoundarySymbol::from(symbol), BoundaryValue::scalar(value));
}

#[allow(clippy::too_many_lines)]
fn mixed_positive_negative_melt_surface(positive_hour_count: usize) -> HillslopeWritebackSurface {
    let mut surface = HillslopeWritebackSurface {
        state_surface: std::collections::BTreeMap::new(),
        flux_surface: std::collections::BTreeMap::new(),
    };

    for (symbol, value) in [
        ("nsl", 2.0),
        ("solthk", 0.3),
        ("solwpv", 2006.0),
        ("dg", 0.1),
        ("thetdr", 0.1),
        ("thetfc", 0.3),
        ("ssc", 0.10),
        ("cancov", 0.0),
        ("lai", 0.0),
        ("rtd", 0.0),
        ("pltol", 0.25),
        ("vdmt", 0.0),
        ("management.initial.params.tillay2_m", 0.0),
        ("wb11_soil_water", 10.0),
        ("wb11_et_demand", 0.0),
        ("wb17_residue_interception", 0.0),
        ("wb11_field_capacity", 8.0),
        ("wb11_perc_fraction", 0.0),
        ("wb18_perc_theta_0001", 5.0),
        ("wb18_perc_fc_0001", 5.0),
        ("thetfc_0001", 50.0),
        ("thetdr_0001", 0.0),
        ("wb18_perc_ul_0001", 8.0),
        ("wb18_perc_ssc_0001", 2.0e-6),
        ("wb18_perc_theta_0002", 5.0),
        ("wb18_perc_fc_0002", 5.0),
        ("thetfc_0002", 50.0),
        ("thetdr_0002", 0.0),
        ("wb18_perc_ul_0002", 8.0),
        ("wb18_perc_ssc_0002", 2.0e-5),
        ("dg_0001", 0.1),
        ("dg_0002", 0.1),
        ("por_0001", 0.55),
        ("por_0002", 0.55),
        ("cpm_0001", 1.0),
        ("coca_0001", 1.0),
        ("cpm_0002", 1.0),
        ("coca_0002", 1.0),
        ("avgslp", 0.1),
        ("slplen", 1.0),
        ("nslpts", 2.0),
        ("xinput_0001", 0.0),
        ("slpinp_0001", 0.0),
        ("wb19_lateral_anisotropy_ratio", 1.0),
        ("wb19_drain_enabled", 0.0),
        ("wb11_lateral_fraction", 0.0),
        ("wb11_drainage_fraction", 0.0),
        ("wb11_drainage_coefficient", 0.0),
        ("wb11_drainable_storage", 0.0),
        ("wb12_rainfall_input", 0.0),
        ("wb12_runon_input", 0.0),
        ("wb12_infiltration", 0.0),
        ("wb12_depression_storage_delta", 0.0),
        ("wb12_runoff_observed", 0.0),
        ("wb12_runoff_closure_tolerance", TOL),
        ("wb12_storage_initial", 10.0),
        ("wb12_storage_observed", 10.0),
        ("wb12_storage_closure_tolerance", TOL),
        ("wb12_precip_input", 0.0),
        ("wb20_forward_solver_lane_enabled", 1.0),
        ("ninten", 2.0),
        ("timem_0001", 0.0),
        ("timem_0002", 1.0),
        ("intsty_0001", 0.0),
        ("intsty_0002", 0.0),
        ("timep", 0.25),
        ("efflen", 1.0),
        ("ealpha", 1.0),
        ("m", 1.5),
        ("snow.options.snow_file_present", 1.0),
        ("snow.options.rst", -1.0),
        ("snow.options.newsnw", 100.0),
        ("snow.options.ssd", 350.0),
        ("snow.runtime_swe", 0.350),
        ("snow.runtime_depth_m", 1.0),
        ("snow.runtime_density_kg_m3", 350.0),
        ("snow.runtime_settle_day_count", 1.0),
        ("tmax", 2.0),
        ("tmin", 1.0),
        ("vwind", 0.0),
        ("tdpt", -1.0),
    ] {
        insert_state(&mut surface, symbol, value);
    }

    for hour in 1..=24 {
        insert_state(&mut surface, &format!("snow.hourly.rain_m_{hour:04}"), 0.0);
        insert_state(
            &mut surface,
            &format!("snow.hourly.snowfall_m_{hour:04}"),
            0.0,
        );
        insert_state(
            &mut surface,
            &format!("winter.hourly.rad_mj_m2_{hour:04}"),
            if hour <= positive_hour_count {
                1.0
            } else {
                0.0
            },
        );
        insert_state(
            &mut surface,
            &format!("winter.hourly.air_temp_c_{hour:04}"),
            if hour <= positive_hour_count {
                2.0
            } else {
                1.0
            },
        );
        insert_state(
            &mut surface,
            &format!("winter.hourly.cloud_fraction_{hour:04}"),
            0.5,
        );
    }

    surface
}

fn state_value(report: &HillslopeKernelExecutionReport, symbol: impl Into<BoundarySymbol>) -> f64 {
    report
        .writeback_surface
        .state_surface
        .get(&symbol.into())
        .expect("state symbol should be present")
        .as_f64()
}

fn melt_totals(report: &HillslopeKernelExecutionReport) -> (f64, f64, f64) {
    let raw_positive_melt = (1..=24)
        .map(|hour| state_value(report, format!("snow.hourly.melt_raw_m_{hour:04}")).max(0.0))
        .sum::<f64>();
    let raw_negative_melt = (1..=24)
        .map(|hour| state_value(report, format!("snow.hourly.melt_raw_m_{hour:04}")).min(0.0))
        .sum::<f64>();
    let routed_melt = (1..=24)
        .map(|hour| state_value(report, format!("snow.hourly.melt_m_{hour:04}")))
        .sum::<f64>();

    (raw_positive_melt, raw_negative_melt, routed_melt)
}

fn execute_surface(surface: HillslopeWritebackSurface) -> HillslopeKernelExecutionReport {
    let graph = parse_topology_fixture_str(VALID_TOPOLOGY).expect("fixture should parse");
    let topology_report =
        validate_pre_execution_topology(&graph).expect("topology report should build");
    let scheduler = HillslopePhaseScheduler::canonical();
    let mut kernel = Wb11HydrologyKernel;

    let report = scheduler
        .execute_with_kernel(&topology_report, &mut kernel, surface)
        .expect("mixed positive/negative snowmelt vector should return typed report");

    assert!(
        report.scheduler_report.is_success(),
        "scheduler halted at {:?}",
        report.scheduler_report.halted_phase
    );

    report
}

#[test]
fn hphys0284_large_negative_melt_state_overdraw_fails_closed() {
    let graph = parse_topology_fixture_str(VALID_TOPOLOGY).expect("fixture should parse");
    let topology_report =
        validate_pre_execution_topology(&graph).expect("topology report should build");
    let scheduler = HillslopePhaseScheduler::canonical();
    let mut kernel = Wb11HydrologyKernel;
    let mut surface = mixed_positive_negative_melt_surface(12);
    insert_state(&mut surface, "snow.runtime_swe", 0.001);
    insert_state(&mut surface, "snow.runtime_depth_m", 1.0);
    insert_state(&mut surface, "snow.runtime_density_kg_m3", 350.0);

    let report = scheduler
        .execute_with_kernel(&topology_report, &mut kernel, surface)
        .expect("typed kernel guard should return a failure report");
    let status = &report.scheduler_report.scheduler_status;

    assert!(
        !report.scheduler_report.is_success()
            && status.finite_ok()
            && !status.domain_ok()
            && status.message_id() == "HKERNEL-WB11-PERC-E-003",
        "expected typed runtime SWE domain failure report, got {:?}",
        report.scheduler_report
    );
}

#[test]
fn hphys0284_negative_melt_state_uses_single_source_storage_loss() {
    let report = execute_surface(mixed_positive_negative_melt_surface(12));
    let (raw_positive_melt, raw_negative_melt, routed_melt) = melt_totals(&report);
    let snow_flux = report
        .writeback_surface
        .flux_surface
        .get(&BoundarySymbol::from("S"))
        .expect("S should be present")
        .as_f64();
    let runtime_swe = state_value(&report, "snow.runtime_swe");
    let runtime_depth = state_value(&report, "snow.runtime_depth_m");
    let runtime_density = state_value(&report, "snow.runtime_density_kg_m3");

    assert!(raw_positive_melt > 0.0);
    assert!(raw_negative_melt < 0.0);
    assert!(raw_positive_melt + raw_negative_melt > 0.0);

    let expected_routed_melt = raw_positive_melt;
    let expected_state_loss = raw_positive_melt;
    let expected_runtime_swe = 0.350 - expected_state_loss;

    assert!((routed_melt - expected_routed_melt).abs() <= TOL);
    assert!((snow_flux - expected_routed_melt).abs() <= TOL);
    assert!((runtime_depth - (expected_runtime_swe * 1000.0 / 350.0)).abs() <= TOL);
    assert!((runtime_density - 350.0).abs() <= TOL);
    assert!(
        (runtime_swe - expected_runtime_swe).abs() <= TOL,
        "runtime SWE must follow the authoritative depth/density storage loss without a second negative-melt debit; runtime_swe={runtime_swe}, expected={expected_runtime_swe}, routed_melt={routed_melt}, raw_positive={raw_positive_melt}, raw_negative={raw_negative_melt}"
    );
}

#[test]
fn hphys0284_negative_melt_state_does_not_undo_positive_loss_when_daily_net_is_nonpositive() {
    let report = execute_surface(mixed_positive_negative_melt_surface(1));
    let (raw_positive_melt, raw_negative_melt, routed_melt) = melt_totals(&report);
    let snow_flux = report
        .writeback_surface
        .flux_surface
        .get(&BoundarySymbol::from("S"))
        .expect("S should be present")
        .as_f64();
    let runtime_swe = state_value(&report, "snow.runtime_swe");
    let runtime_depth = state_value(&report, "snow.runtime_depth_m");
    let runtime_density = state_value(&report, "snow.runtime_density_kg_m3");

    assert!(raw_positive_melt > 0.0);
    assert!(raw_negative_melt < 0.0);
    assert!(raw_positive_melt + raw_negative_melt <= TOL);
    assert!((routed_melt - raw_positive_melt).abs() <= TOL);
    assert!((snow_flux - raw_positive_melt).abs() <= TOL);
    assert!((runtime_swe - (0.350 - raw_positive_melt)).abs() <= TOL);
    assert!((runtime_depth - ((0.350 - raw_positive_melt) * 1000.0 / 350.0)).abs() <= TOL);
    assert!((runtime_density - 350.0).abs() <= TOL);
}
