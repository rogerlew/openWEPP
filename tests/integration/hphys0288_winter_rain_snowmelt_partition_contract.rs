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
const INITIAL_SNOW_DEPTH_M: f64 = 0.100;
const INITIAL_SNOW_DENSITY_KG_M3: f64 = 340.0;
const INITIAL_SWE_M: f64 = INITIAL_SNOW_DEPTH_M * INITIAL_SNOW_DENSITY_KG_M3 / 1000.0;
const HOURLY_RAIN_M: f64 = 0.003;
const RETAINED_RAIN_M: f64 = INITIAL_SNOW_DEPTH_M * (350.0 - INITIAL_SNOW_DENSITY_KG_M3) / 1000.0;
const RELEASED_RAIN_M: f64 = HOURLY_RAIN_M - RETAINED_RAIN_M;

fn insert_state(surface: &mut HillslopeWritebackSurface, symbol: &str, value: f64) {
    surface
        .state_surface
        .insert(BoundarySymbol::from(symbol), BoundaryValue::scalar(value));
}

#[allow(clippy::too_many_lines)]
fn partial_rain_on_snow_surface() -> HillslopeWritebackSurface {
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
        ("wb18_perc_ssc_0001", 2.0e-12),
        ("wb18_perc_theta_0002", 5.0),
        ("wb18_perc_fc_0002", 5.0),
        ("thetfc_0002", 50.0),
        ("thetdr_0002", 0.0),
        ("wb18_perc_ul_0002", 8.0),
        ("wb18_perc_ssc_0002", 2.0e-12),
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
        ("wb18_perc_lane_substeps", 24.0),
        ("wb12_rainfall_input", HOURLY_RAIN_M),
        ("wb12_runon_input", 0.0),
        ("wb12_infiltration", 0.0),
        ("wb12_depression_storage_delta", 0.0),
        ("wb12_runoff_observed", 0.0),
        ("wb12_runoff_closure_tolerance", TOL),
        ("wb12_storage_initial", 10.0),
        ("wb12_storage_observed", 10.0 + RELEASED_RAIN_M),
        ("wb12_storage_closure_tolerance", TOL),
        ("wb12_precip_input", HOURLY_RAIN_M),
        ("wb20_forward_solver_lane_enabled", 1.0),
        ("ninten", 2.0),
        ("timem_0001", 0.0),
        ("timem_0002", 1.0),
        ("intsty_0001", HOURLY_RAIN_M),
        ("intsty_0002", 0.0),
        ("timep", 0.25),
        ("efflen", 1.0),
        ("ealpha", 1.0),
        ("m", 1.5),
        ("snow.options.snow_file_present", 1.0),
        ("snow.options.rst", -1.0),
        ("snow.options.newsnw", 100.0),
        ("snow.options.ssd", 350.0),
        ("snow.runtime_swe", INITIAL_SWE_M),
        ("snow.runtime_depth_m", INITIAL_SNOW_DEPTH_M),
        ("snow.runtime_density_kg_m3", INITIAL_SNOW_DENSITY_KG_M3),
        ("snow.runtime_settle_day_count", 1.0),
        ("tmax", 2.0),
        ("tmin", 1.0),
        ("vwind", 0.0),
        ("tdpt", 0.0),
    ] {
        insert_state(&mut surface, symbol, value);
    }

    for hour in 1..=24 {
        insert_state(
            &mut surface,
            &format!("snow.hourly.rain_m_{hour:04}"),
            if hour == 1 { HOURLY_RAIN_M } else { 0.0 },
        );
        insert_state(
            &mut surface,
            &format!("snow.hourly.snowfall_m_{hour:04}"),
            0.0,
        );
        insert_state(
            &mut surface,
            &format!("winter.hourly.rad_mj_m2_{hour:04}"),
            0.0,
        );
        insert_state(
            &mut surface,
            &format!("winter.hourly.air_temp_c_{hour:04}"),
            0.0,
        );
        insert_state(
            &mut surface,
            &format!("winter.hourly.cloud_fraction_{hour:04}"),
            1.0,
        );
    }

    surface
}

fn execute_hphys0288_surface(surface: HillslopeWritebackSurface) -> HillslopeKernelExecutionReport {
    let graph = parse_topology_fixture_str(VALID_TOPOLOGY).expect("fixture should parse");
    let topology_report =
        validate_pre_execution_topology(&graph).expect("topology report should build");
    let scheduler = HillslopePhaseScheduler::canonical();
    let mut kernel = Wb11HydrologyKernel;

    let report = scheduler
        .execute_with_kernel(&topology_report, &mut kernel, surface)
        .expect("HPHYS0288 rain-on-snow vector should return typed report");

    assert!(
        report.scheduler_report.is_success(),
        "scheduler halted at {:?}",
        report.scheduler_report.halted_phase
    );

    report
}

fn state_value(report: &HillslopeKernelExecutionReport, symbol: &str) -> f64 {
    report
        .writeback_surface
        .state_surface
        .get(&BoundarySymbol::from(symbol))
        .unwrap_or_else(|| panic!("{symbol} should be present"))
        .as_f64()
}

#[test]
fn hphys0288_partial_rain_on_snow_release_is_routed_as_wmelt() {
    let report = execute_hphys0288_surface(partial_rain_on_snow_surface());

    let retained_rain = report
        .writeback_surface
        .state_surface
        .get(&BoundarySymbol::from("snow.hourly.rain_retained_m_0001"))
        .expect("retained rain trace should be present")
        .as_f64();
    let released_rain_as_melt = report
        .writeback_surface
        .state_surface
        .get(&BoundarySymbol::from("snow.hourly.melt_m_0001"))
        .expect("hour 1 routed melt trace should be present")
        .as_f64();
    let snow_flux = report
        .writeback_surface
        .flux_surface
        .get(&BoundarySymbol::from("S"))
        .expect("S should be present")
        .as_f64();
    let runtime_swe = report
        .writeback_surface
        .state_surface
        .get(&BoundarySymbol::from("snow.runtime_swe"))
        .expect("runtime SWE should be present")
        .as_f64();
    let infiltration = report
        .writeback_surface
        .state_surface
        .get(&BoundarySymbol::from("wb12_infiltration"))
        .expect("wb12_infiltration should be present")
        .as_f64();

    assert!((retained_rain - RETAINED_RAIN_M).abs() <= 1.0e-9);
    assert!(
        (released_rain_as_melt - RELEASED_RAIN_M).abs() <= 1.0e-9,
        "released rain-on-snow must be routed through final hrmlt/wmelt; observed {released_rain_as_melt}"
    );
    assert!((snow_flux + RETAINED_RAIN_M).abs() <= 1.0e-9);
    assert!((runtime_swe - (INITIAL_SWE_M + RETAINED_RAIN_M)).abs() <= 1.0e-9);
    assert!((infiltration - RELEASED_RAIN_M).abs() <= 1.0e-9);
}

#[test]
fn hphys0288_dense_snow_release_adds_to_positive_raw_melt() {
    let mut surface = partial_rain_on_snow_surface();
    insert_state(&mut surface, "snow.runtime_swe", 0.035);
    insert_state(&mut surface, "snow.runtime_density_kg_m3", 350.0);
    insert_state(&mut surface, "wb12_storage_observed", 10.01);
    insert_state(&mut surface, "wb12_storage_closure_tolerance", 0.02);
    insert_state(&mut surface, "winter.hourly.rad_mj_m2_0001", 1.25);
    insert_state(&mut surface, "winter.hourly.air_temp_c_0001", 2.0);
    insert_state(&mut surface, "winter.hourly.cloud_fraction_0001", 0.5);

    let report = execute_hphys0288_surface(surface);

    let raw_melt = state_value(&report, "snow.hourly.melt_raw_m_0001");
    let released_rain = state_value(&report, "snow.hourly.rain_released_m_0001");
    let routed_melt = state_value(&report, "snow.hourly.melt_m_0001");
    let retained_rain = state_value(&report, "snow.hourly.rain_retained_m_0001");

    assert!(raw_melt > 0.0);
    assert!(retained_rain.abs() <= 1.0e-12);
    assert!((released_rain - HOURLY_RAIN_M).abs() <= 1.0e-9);
    assert!((routed_melt - (raw_melt + released_rain)).abs() <= 1.0e-9);
}

#[test]
fn hphys0288_multi_hour_dense_release_keeps_direct_rain_out_of_snow_partition() {
    let mut surface = partial_rain_on_snow_surface();
    insert_state(&mut surface, "snow.runtime_swe", 0.035);
    insert_state(&mut surface, "snow.runtime_density_kg_m3", 350.0);
    insert_state(&mut surface, "snow.hourly.rain_m_0001", 0.001);
    insert_state(&mut surface, "snow.hourly.rain_m_0002", 0.002);
    insert_state(&mut surface, "wb12_storage_observed", 10.003);

    let report = execute_hphys0288_surface(surface);

    let released_hour_1 = state_value(&report, "snow.hourly.rain_released_m_0001");
    let released_hour_2 = state_value(&report, "snow.hourly.rain_released_m_0002");
    let routed_hour_1 = state_value(&report, "snow.hourly.melt_m_0001");
    let routed_hour_2 = state_value(&report, "snow.hourly.melt_m_0002");
    let infiltration = state_value(&report, "wb12_infiltration");

    assert!((released_hour_1 - 0.001).abs() <= 1.0e-9);
    assert!((released_hour_2 - 0.002).abs() <= 1.0e-9);
    assert!((routed_hour_1 - released_hour_1).abs() <= 1.0e-9);
    assert!((routed_hour_2 - released_hour_2).abs() <= 1.0e-9);
    assert!((infiltration - (released_hour_1 + released_hour_2)).abs() <= 1.0e-9);
}
