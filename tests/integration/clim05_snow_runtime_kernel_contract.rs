use openwepp_hillslope_orchestrator::{
    HillslopePhase, HillslopePhaseScheduler, HillslopeWritebackSurface, Wb11HydrologyKernel,
};
use openwepp_kernel_contract::{BoundarySymbol, BoundaryValue};
use openwepp_sim_contract::status::BoundaryClass;
use openwepp_topology::{parse_topology_fixture_str, validate_pre_execution_topology};

const VALID_TOPOLOGY: &str = r"
HILLSLOPES 3
CHANNELS 2
IMPOUNDMENTS 1
NODE CHANNEL 1 H 1 2 0 C 0 0 0 I 0 0 0
NODE CHANNEL 2 H 3 0 0 C 1 0 0 I 0 0 0
NODE IMPOUNDMENT 1 H 0 0 0 C 2 0 0 I 0 0 0
";

const EXPECTED_WB14_INFILTRATION: f64 = 2.849_436_367_164_438;
const EXPECTED_S: f64 = -0.1;
const EXPECTED_WB14_Q_WITH_S: f64 = 0.250_563_632_835_562;
const EXPECTED_WB12_STORAGE_WITH_S: f64 = 12.709_319_506_403_453;
const CLIM05_TEST_TOLERANCE: f64 = 1.0e-6;

#[allow(clippy::too_many_lines)]
fn seeded_clim05_surface() -> HillslopeWritebackSurface {
    let mut state_surface = std::collections::BTreeMap::new();

    state_surface.insert(BoundarySymbol::from("nsl"), BoundaryValue::scalar(2.0));
    state_surface.insert(BoundarySymbol::from("solthk"), BoundaryValue::scalar(0.3));
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
        BoundaryValue::scalar(EXPECTED_WB14_Q_WITH_S),
    );
    state_surface.insert(
        BoundarySymbol::from("wb12_runoff_closure_tolerance"),
        BoundaryValue::scalar(CLIM05_TEST_TOLERANCE),
    );

    state_surface.insert(
        BoundarySymbol::from("wb12_storage_initial"),
        BoundaryValue::scalar(12.0),
    );
    state_surface.insert(
        BoundarySymbol::from("wb12_storage_observed"),
        BoundaryValue::scalar(EXPECTED_WB12_STORAGE_WITH_S),
    );
    state_surface.insert(
        BoundarySymbol::from("wb12_storage_closure_tolerance"),
        BoundaryValue::scalar(CLIM05_TEST_TOLERANCE),
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
        BoundarySymbol::from("wb20_forward_solver_lane_enabled"),
        BoundaryValue::scalar(1.0),
    );

    state_surface.insert(
        BoundarySymbol::from("snow.options.snow_file_present"),
        BoundaryValue::scalar(1.0),
    );
    state_surface.insert(
        BoundarySymbol::from("snow.options.rst"),
        BoundaryValue::scalar(0.0),
    );
    state_surface.insert(
        BoundarySymbol::from("snow.options.newsnw"),
        BoundaryValue::scalar(100.0),
    );
    state_surface.insert(
        BoundarySymbol::from("snow.options.ssd"),
        BoundaryValue::scalar(200.0),
    );
    state_surface.insert(
        BoundarySymbol::from("snow.runtime_swe"),
        BoundaryValue::scalar(1.0),
    );
    state_surface.insert(
        BoundarySymbol::from("snow.runtime_depth_m"),
        BoundaryValue::scalar(10.0),
    );
    state_surface.insert(
        BoundarySymbol::from("snow.runtime_density_kg_m3"),
        BoundaryValue::scalar(100.0),
    );
    state_surface.insert(
        BoundarySymbol::from("snow.runtime_settle_day_count"),
        BoundaryValue::scalar(0.0),
    );
    state_surface.insert(BoundarySymbol::from("tmax"), BoundaryValue::scalar(2.0));
    state_surface.insert(BoundarySymbol::from("tmin"), BoundaryValue::scalar(1.0));
    state_surface.insert(BoundarySymbol::from("vwind"), BoundaryValue::scalar(3.0));
    state_surface.insert(BoundarySymbol::from("tdpt"), BoundaryValue::scalar(-1.0));

    for hour in 1..=24 {
        let snowfall = if hour == 1 { 1.0 } else { 0.0 };
        state_surface.insert(
            BoundarySymbol::from(format!("snow.hourly.rain_m_{hour:04}")),
            BoundaryValue::scalar(0.0),
        );
        state_surface.insert(
            BoundarySymbol::from(format!("snow.hourly.snowfall_m_{hour:04}")),
            BoundaryValue::scalar(snowfall),
        );
        state_surface.insert(
            BoundarySymbol::from(format!("winter.hourly.rad_mj_m2_{hour:04}")),
            BoundaryValue::scalar(0.0),
        );
        state_surface.insert(
            BoundarySymbol::from(format!("winter.hourly.air_temp_c_{hour:04}")),
            BoundaryValue::scalar(-2.0),
        );
        state_surface.insert(
            BoundarySymbol::from(format!("winter.hourly.cloud_fraction_{hour:04}")),
            BoundaryValue::scalar(0.5),
        );
    }

    HillslopeWritebackSurface {
        state_surface,
        flux_surface: std::collections::BTreeMap::new(),
    }
}

#[test]
#[allow(clippy::too_many_lines)]
fn clim05_contract_conformance_couples_snow_controls_into_hydrology_reconciliation() {
    let graph = parse_topology_fixture_str(VALID_TOPOLOGY).expect("fixture should parse");
    let topology_report =
        validate_pre_execution_topology(&graph).expect("topology report should build");
    let scheduler = HillslopePhaseScheduler::canonical();
    let mut kernel = Wb11HydrologyKernel;

    let report = scheduler
        .execute_with_kernel(&topology_report, &mut kernel, seeded_clim05_surface())
        .expect("clim05 execution should return typed report");
    let halted_status = report
        .phase_reports
        .iter()
        .find(|phase| Some(phase.phase) == report.scheduler_report.halted_phase)
        .map_or_else(
            || "none".to_string(),
            |phase| phase.decision_status.message_id().to_string(),
        );

    assert!(
        report.scheduler_report.is_success(),
        "scheduler halted at {:?} with status {}",
        report.scheduler_report.halted_phase,
        halted_status
    );

    for hour in 1..=24 {
        let typed_symbols = [
            (format!("snow.hourly.depth_before_m_{hour:04}"), "m"),
            (format!("snow.hourly.depth_available_m_{hour:04}"), "m"),
            (
                format!("snow.hourly.density_before_kg_m3_{hour:04}"),
                "kg m^-3",
            ),
            (format!("snow.hourly.depth_after_m_{hour:04}"), "m"),
            (
                format!("snow.hourly.density_after_kg_m3_{hour:04}"),
                "kg m^-3",
            ),
            (format!("snow.hourly.rain_retained_m_{hour:04}"), "m"),
            (format!("snow.hourly.melt_m_{hour:04}"), "m"),
            (
                format!("snow.hourly.melt_branch_active_{hour:04}"),
                "dimensionless",
            ),
            (format!("winter.hourly.dewpoint_c_{hour:04}"), "degC"),
            (format!("winter.hourly.wind_m_s_{hour:04}"), "m s^-1"),
        ];

        for (symbol, expected_unit) in typed_symbols {
            let value = report
                .writeback_surface
                .state_surface
                .get(&BoundarySymbol::from(symbol.clone()))
                .unwrap_or_else(|| panic!("missing {symbol}"));
            assert_eq!(value.unit_label(), expected_unit, "{symbol} unit label");
            assert_ne!(value.unit_label(), "scalar", "{symbol} must not be scalar");
        }
    }

    let infiltration = report
        .writeback_surface
        .state_surface
        .get(&BoundarySymbol::from("wb12_infiltration"))
        .expect("wb12_infiltration should be present")
        .as_f64();
    assert!(
        (infiltration - EXPECTED_WB14_INFILTRATION).abs() <= CLIM05_TEST_TOLERANCE,
        "infiltration={infiltration} expected={EXPECTED_WB14_INFILTRATION}"
    );

    let snow_flux = report
        .writeback_surface
        .flux_surface
        .get(&BoundarySymbol::from("S"))
        .expect("S should be present")
        .as_f64();
    assert!(
        (snow_flux - EXPECTED_S).abs() <= CLIM05_TEST_TOLERANCE,
        "snow_flux={snow_flux} expected={EXPECTED_S}"
    );

    let snow_runtime_swe = report
        .writeback_surface
        .state_surface
        .get(&BoundarySymbol::from("snow.runtime_swe"))
        .expect("snow.runtime_swe should be present")
        .to_owned();
    assert_eq!(snow_runtime_swe.unit_label(), "m");
    assert!(
        (snow_runtime_swe.as_f64() - (1.0 - EXPECTED_S)).abs() <= CLIM05_TEST_TOLERANCE,
        "runtime_swe={} expected={}",
        snow_runtime_swe.as_f64(),
        1.0 - EXPECTED_S
    );
    assert_eq!(
        report
            .writeback_surface
            .state_surface
            .get(&BoundarySymbol::from("snow.runtime_depth_m"))
            .expect("snow.runtime_depth_m should be present")
            .unit_label(),
        "m"
    );
    assert_eq!(
        report
            .writeback_surface
            .state_surface
            .get(&BoundarySymbol::from("snow.runtime_density_kg_m3"))
            .expect("snow.runtime_density_kg_m3 should be present")
            .unit_label(),
        "kg m^-3"
    );

    let q_runoff = report
        .writeback_surface
        .flux_surface
        .get(&BoundarySymbol::from("Q"))
        .expect("Q should be present")
        .as_f64();
    assert!(
        (q_runoff - EXPECTED_WB14_Q_WITH_S).abs() <= CLIM05_TEST_TOLERANCE,
        "q_runoff={q_runoff} expected={EXPECTED_WB14_Q_WITH_S}"
    );

    let storage_reconciled = report
        .writeback_surface
        .state_surface
        .get(&BoundarySymbol::from("wb12_storage_reconciled"))
        .expect("wb12_storage_reconciled should be present")
        .as_f64();
    assert!(
        (storage_reconciled - EXPECTED_WB12_STORAGE_WITH_S).abs() <= CLIM05_TEST_TOLERANCE,
        "storage_reconciled={storage_reconciled} expected={EXPECTED_WB12_STORAGE_WITH_S}"
    );
}

#[test]
fn clim05_contract_conformance_cold_trigger_runs_snow_without_sidecar_gate() {
    let graph = parse_topology_fixture_str(VALID_TOPOLOGY).expect("fixture should parse");
    let topology_report =
        validate_pre_execution_topology(&graph).expect("topology report should build");
    let scheduler = HillslopePhaseScheduler::canonical();
    let mut kernel = Wb11HydrologyKernel;

    let mut surface = seeded_clim05_surface();
    surface.state_surface.insert(
        BoundarySymbol::from("snow.options.snow_file_present"),
        BoundaryValue::scalar(0.0),
    );
    surface.state_surface.insert(
        BoundarySymbol::from("snow.runtime_swe"),
        BoundaryValue::scalar(0.0),
    );
    surface.state_surface.insert(
        BoundarySymbol::from("snow.runtime_depth_m"),
        BoundaryValue::scalar(0.0),
    );
    surface.state_surface.insert(
        BoundarySymbol::from("snow.runtime_density_kg_m3"),
        BoundaryValue::scalar(0.0),
    );
    surface
        .state_surface
        .insert(BoundarySymbol::from("tmax"), BoundaryValue::scalar(-1.0));
    surface
        .state_surface
        .insert(BoundarySymbol::from("tmin"), BoundaryValue::scalar(-3.0));

    let report = scheduler
        .execute_with_kernel(&topology_report, &mut kernel, surface)
        .expect("cold-trigger snow execution should return typed report");
    assert!(
        report.scheduler_report.is_success(),
        "scheduler halted at {:?}",
        report.scheduler_report.halted_phase
    );

    let snow_flux = report
        .writeback_surface
        .flux_surface
        .get(&BoundarySymbol::from("S"))
        .expect("S should be present under cold trigger")
        .as_f64();
    let runtime_swe = report
        .writeback_surface
        .state_surface
        .get(&BoundarySymbol::from("snow.runtime_swe"))
        .expect("snow.runtime_swe should be present under cold trigger")
        .as_f64();

    assert!((snow_flux + 0.1).abs() <= CLIM05_TEST_TOLERANCE);
    assert!((runtime_swe - 0.1).abs() <= CLIM05_TEST_TOLERANCE);
}

#[test]
fn hphys0269_contract_conformance_retains_rain_in_subthreshold_snowpack() {
    let graph = parse_topology_fixture_str(VALID_TOPOLOGY).expect("fixture should parse");
    let topology_report =
        validate_pre_execution_topology(&graph).expect("topology report should build");
    let scheduler = HillslopePhaseScheduler::canonical();
    let mut kernel = Wb11HydrologyKernel;

    let mut surface = seeded_clim05_surface();
    surface.state_surface.insert(
        BoundarySymbol::from("snow.runtime_swe"),
        BoundaryValue::scalar(0.200),
    );
    surface.state_surface.insert(
        BoundarySymbol::from("snow.runtime_depth_m"),
        BoundaryValue::scalar(1.0),
    );
    surface.state_surface.insert(
        BoundarySymbol::from("snow.runtime_density_kg_m3"),
        BoundaryValue::scalar(200.0),
    );
    surface
        .state_surface
        .insert(BoundarySymbol::from("tmax"), BoundaryValue::scalar(2.0));
    surface
        .state_surface
        .insert(BoundarySymbol::from("tmin"), BoundaryValue::scalar(1.0));
    surface
        .state_surface
        .insert(BoundarySymbol::from("vwind"), BoundaryValue::scalar(0.0));
    surface
        .state_surface
        .insert(BoundarySymbol::from("tdpt"), BoundaryValue::scalar(0.0));

    for hour in 1..=24 {
        surface.state_surface.insert(
            BoundarySymbol::from(format!("snow.hourly.rain_m_{hour:04}")),
            BoundaryValue::scalar(0.001),
        );
        surface.state_surface.insert(
            BoundarySymbol::from(format!("snow.hourly.snowfall_m_{hour:04}")),
            BoundaryValue::scalar(0.0),
        );
        surface.state_surface.insert(
            BoundarySymbol::from(format!("winter.hourly.rad_mj_m2_{hour:04}")),
            BoundaryValue::scalar(0.0),
        );
        surface.state_surface.insert(
            BoundarySymbol::from(format!("winter.hourly.air_temp_c_{hour:04}")),
            BoundaryValue::scalar(0.0),
        );
        surface.state_surface.insert(
            BoundarySymbol::from(format!("winter.hourly.cloud_fraction_{hour:04}")),
            BoundaryValue::scalar(1.0),
        );
    }

    let report = scheduler
        .execute_with_kernel(&topology_report, &mut kernel, surface)
        .expect("rain-retention snow execution should return typed report");
    assert!(
        report.scheduler_report.is_success(),
        "scheduler halted at {:?}",
        report.scheduler_report.halted_phase
    );

    let retained_rain = (1..=24)
        .map(|hour| {
            report
                .writeback_surface
                .state_surface
                .get(&BoundarySymbol::from(format!(
                    "snow.hourly.rain_retained_m_{hour:04}"
                )))
                .expect("rain-retained trace should be present")
                .as_f64()
        })
        .sum::<f64>();
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

    assert!((retained_rain - 0.024).abs() <= CLIM05_TEST_TOLERANCE);
    assert!((snow_flux + 0.024).abs() <= CLIM05_TEST_TOLERANCE);
    assert!((runtime_swe - 0.224).abs() <= CLIM05_TEST_TOLERANCE);
}

#[test]
fn hphys0269_contract_conformance_records_signed_raw_melt_and_redistributes_daily_melt() {
    let graph = parse_topology_fixture_str(VALID_TOPOLOGY).expect("fixture should parse");
    let topology_report =
        validate_pre_execution_topology(&graph).expect("topology report should build");
    let scheduler = HillslopePhaseScheduler::canonical();
    let mut kernel = Wb11HydrologyKernel;

    let mut surface = seeded_clim05_surface();
    surface.state_surface.insert(
        BoundarySymbol::from("snow.runtime_swe"),
        BoundaryValue::scalar(0.350),
    );
    surface.state_surface.insert(
        BoundarySymbol::from("snow.runtime_depth_m"),
        BoundaryValue::scalar(1.0),
    );
    surface.state_surface.insert(
        BoundarySymbol::from("snow.runtime_density_kg_m3"),
        BoundaryValue::scalar(350.0),
    );
    surface
        .state_surface
        .insert(BoundarySymbol::from("tmax"), BoundaryValue::scalar(2.0));
    surface
        .state_surface
        .insert(BoundarySymbol::from("tmin"), BoundaryValue::scalar(1.0));
    surface
        .state_surface
        .insert(BoundarySymbol::from("vwind"), BoundaryValue::scalar(0.0));
    surface
        .state_surface
        .insert(BoundarySymbol::from("tdpt"), BoundaryValue::scalar(-1.0));

    for hour in 1..=24 {
        surface.state_surface.insert(
            BoundarySymbol::from(format!("snow.hourly.rain_m_{hour:04}")),
            BoundaryValue::scalar(0.0),
        );
        surface.state_surface.insert(
            BoundarySymbol::from(format!("snow.hourly.snowfall_m_{hour:04}")),
            BoundaryValue::scalar(0.0),
        );
        surface.state_surface.insert(
            BoundarySymbol::from(format!("winter.hourly.rad_mj_m2_{hour:04}")),
            BoundaryValue::scalar(if hour <= 12 { 1.0 } else { 0.0 }),
        );
        surface.state_surface.insert(
            BoundarySymbol::from(format!("winter.hourly.air_temp_c_{hour:04}")),
            BoundaryValue::scalar(if hour <= 12 { 2.0 } else { 1.0 }),
        );
        surface.state_surface.insert(
            BoundarySymbol::from(format!("winter.hourly.cloud_fraction_{hour:04}")),
            BoundaryValue::scalar(0.5),
        );
    }

    let report = scheduler
        .execute_with_kernel(&topology_report, &mut kernel, surface)
        .expect("negative-melt snow execution should return typed report");
    assert!(
        report.scheduler_report.is_success(),
        "scheduler halted at {:?}",
        report.scheduler_report.halted_phase
    );

    let raw_melt_sum = (1..=24)
        .map(|hour| {
            report
                .writeback_surface
                .state_surface
                .get(&BoundarySymbol::from(format!(
                    "snow.hourly.melt_raw_m_{hour:04}"
                )))
                .expect("raw melt trace should be present")
                .as_f64()
        })
        .sum::<f64>();
    let redistributed_melt_sum = (1..=24)
        .map(|hour| {
            report
                .writeback_surface
                .state_surface
                .get(&BoundarySymbol::from(format!(
                    "snow.hourly.melt_m_{hour:04}"
                )))
                .expect("redistributed melt trace should be present")
                .as_f64()
        })
        .sum::<f64>();
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

    assert!(raw_melt_sum < redistributed_melt_sum + 1.0e-12);
    assert!((redistributed_melt_sum - snow_flux).abs() <= CLIM05_TEST_TOLERANCE);
    assert!((runtime_swe - (0.350 - redistributed_melt_sum)).abs() <= CLIM05_TEST_TOLERANCE);
}

#[test]
fn hphys0271_contract_conformance_records_melt_terms_and_hourly_forcing() {
    let graph = parse_topology_fixture_str(VALID_TOPOLOGY).expect("fixture should parse");
    let topology_report =
        validate_pre_execution_topology(&graph).expect("topology report should build");
    let scheduler = HillslopePhaseScheduler::canonical();
    let mut kernel = Wb11HydrologyKernel;

    let mut surface = seeded_clim05_surface();
    surface.state_surface.insert(
        BoundarySymbol::from("snow.runtime_swe"),
        BoundaryValue::scalar(0.350),
    );
    surface.state_surface.insert(
        BoundarySymbol::from("snow.runtime_depth_m"),
        BoundaryValue::scalar(1.0),
    );
    surface.state_surface.insert(
        BoundarySymbol::from("snow.runtime_density_kg_m3"),
        BoundaryValue::scalar(350.0),
    );
    surface
        .state_surface
        .insert(BoundarySymbol::from("tmax"), BoundaryValue::scalar(2.0));
    surface
        .state_surface
        .insert(BoundarySymbol::from("tmin"), BoundaryValue::scalar(1.0));
    surface
        .state_surface
        .insert(BoundarySymbol::from("vwind"), BoundaryValue::scalar(2.0));
    surface
        .state_surface
        .insert(BoundarySymbol::from("tdpt"), BoundaryValue::scalar(-1.0));

    for hour in 1..=24 {
        surface.state_surface.insert(
            BoundarySymbol::from(format!("snow.hourly.rain_m_{hour:04}")),
            BoundaryValue::scalar(0.0),
        );
        surface.state_surface.insert(
            BoundarySymbol::from(format!("snow.hourly.snowfall_m_{hour:04}")),
            BoundaryValue::scalar(0.0),
        );
        surface.state_surface.insert(
            BoundarySymbol::from(format!("winter.hourly.rad_mj_m2_{hour:04}")),
            BoundaryValue::scalar(if hour == 1 { 1.25 } else { 0.0 }),
        );
        surface.state_surface.insert(
            BoundarySymbol::from(format!("winter.hourly.air_temp_c_{hour:04}")),
            BoundaryValue::scalar(if hour == 1 { 2.0 } else { 1.0 }),
        );
        surface.state_surface.insert(
            BoundarySymbol::from(format!("winter.hourly.cloud_fraction_{hour:04}")),
            BoundaryValue::scalar(0.5),
        );
    }

    let report = scheduler
        .execute_with_kernel(&topology_report, &mut kernel, surface)
        .expect("melt-term snow execution should return typed report");
    assert!(
        report.scheduler_report.is_success(),
        "scheduler halted at {:?}",
        report.scheduler_report.halted_phase
    );

    let hour_value = |symbol: &str| {
        report
            .writeback_surface
            .state_surface
            .get(&BoundarySymbol::from(symbol))
            .unwrap_or_else(|| panic!("{symbol} should be present"))
            .as_f64()
    };

    let amelt = hour_value("snow.hourly.melt_amelt_in_0001");
    let bmelt = hour_value("snow.hourly.melt_bmelt_in_0001");
    let cmelt = hour_value("snow.hourly.melt_cmelt_in_0001");
    let dmelt = hour_value("snow.hourly.melt_dmelt_in_0001");
    let raw_melt = hour_value("snow.hourly.melt_raw_m_0001");

    assert!((raw_melt - (0.0254 * (amelt + bmelt + cmelt + dmelt))).abs() <= 1.0e-12);
    assert!((hour_value("snow.hourly.melt_branch_active_0001") - 1.0).abs() <= 1.0e-12);
    assert!((hour_value("winter.hourly.dewpoint_c_0001") + 1.0).abs() <= 1.0e-12);
    assert!((hour_value("winter.hourly.wind_m_s_0001") - 2.0).abs() <= 1.0e-12);
    assert!((hour_value("snow.hourly.melt_hrtef_f_0001") - 3.6).abs() <= 1.0e-12);
    assert!((hour_value("snow.hourly.melt_hrdtf_f_0001") + 1.8).abs() <= 1.0e-12);
    assert!(
        (hour_value("snow.hourly.melt_vwmph_0001") - ((2.0 * 3600.0) / 1609.0)).abs() <= 1.0e-12
    );
    assert!(hour_value("snow.hourly.melt_wind_adjustment_0001") > 0.0);
    assert!(hour_value("snow.hourly.melt_rainin_0001").abs() <= 1.0e-12);
}

#[test]
fn clim05_contract_conformance_rejects_missing_active_snow_control_symbol() {
    let graph = parse_topology_fixture_str(VALID_TOPOLOGY).expect("fixture should parse");
    let topology_report =
        validate_pre_execution_topology(&graph).expect("topology report should build");
    let scheduler = HillslopePhaseScheduler::canonical();
    let mut kernel = Wb11HydrologyKernel;

    let mut surface = seeded_clim05_surface();
    surface
        .state_surface
        .remove(&BoundarySymbol::from("snow.options.ssd"));

    let report = scheduler
        .execute_with_kernel(&topology_report, &mut kernel, surface)
        .expect("clim05 missing-symbol failure should return typed report");

    assert_eq!(
        report.scheduler_report.halted_phase,
        Some(HillslopePhase::RunoffReconciliation)
    );

    let runoff_phase = report
        .phase_reports
        .iter()
        .find(|phase| phase.phase == HillslopePhase::RunoffReconciliation)
        .expect("runoff phase report should exist");
    assert_eq!(
        runoff_phase.decision_status.message_id(),
        "HKERNEL-WB14-RUNOFF-E-001"
    );
    assert_eq!(
        runoff_phase.decision_status.boundary_class(),
        BoundaryClass::MissingRequiredInput
    );
}

#[test]
fn clim05_contract_conformance_rejects_non_finite_active_snow_control_symbol() {
    let graph = parse_topology_fixture_str(VALID_TOPOLOGY).expect("fixture should parse");
    let topology_report =
        validate_pre_execution_topology(&graph).expect("topology report should build");
    let scheduler = HillslopePhaseScheduler::canonical();
    let mut kernel = Wb11HydrologyKernel;

    let mut surface = seeded_clim05_surface();
    surface.state_surface.insert(
        BoundarySymbol::from("snow.options.newsnw"),
        BoundaryValue::scalar(f64::NAN),
    );

    let report = scheduler
        .execute_with_kernel(&topology_report, &mut kernel, surface)
        .expect("clim05 non-finite failure should return typed report");

    assert_eq!(
        report.scheduler_report.halted_phase,
        Some(HillslopePhase::RunoffReconciliation)
    );

    let runoff_phase = report
        .phase_reports
        .iter()
        .find(|phase| phase.phase == HillslopePhase::RunoffReconciliation)
        .expect("runoff phase report should exist");
    assert_eq!(
        runoff_phase.decision_status.message_id(),
        "HKERNEL-WB14-RUNOFF-E-002"
    );
    assert_eq!(
        runoff_phase.decision_status.boundary_class(),
        BoundaryClass::NonFinite
    );
}

#[test]
fn clim05_contract_conformance_rejects_invalid_active_snow_control_domain() {
    let graph = parse_topology_fixture_str(VALID_TOPOLOGY).expect("fixture should parse");
    let topology_report =
        validate_pre_execution_topology(&graph).expect("topology report should build");
    let scheduler = HillslopePhaseScheduler::canonical();
    let mut kernel = Wb11HydrologyKernel;

    let mut surface = seeded_clim05_surface();
    surface.state_surface.insert(
        BoundarySymbol::from("snow.options.newsnw"),
        BoundaryValue::scalar(250.0),
    );

    let report = scheduler
        .execute_with_kernel(&topology_report, &mut kernel, surface)
        .expect("clim05 domain failure should return typed report");

    assert_eq!(
        report.scheduler_report.halted_phase,
        Some(HillslopePhase::RunoffReconciliation)
    );

    let runoff_phase = report
        .phase_reports
        .iter()
        .find(|phase| phase.phase == HillslopePhase::RunoffReconciliation)
        .expect("runoff phase report should exist");
    assert_eq!(
        runoff_phase.decision_status.message_id(),
        "HKERNEL-WB14-RUNOFF-E-003"
    );
    assert_eq!(
        runoff_phase.decision_status.boundary_class(),
        BoundaryClass::DomainViolation
    );
}

#[test]
fn simimpl29_contract_conformance_rejects_missing_hourly_snow_kernel_symbol() {
    let graph = parse_topology_fixture_str(VALID_TOPOLOGY).expect("fixture should parse");
    let topology_report =
        validate_pre_execution_topology(&graph).expect("topology report should build");
    let scheduler = HillslopePhaseScheduler::canonical();
    let mut kernel = Wb11HydrologyKernel;

    let mut surface = seeded_clim05_surface();
    surface
        .state_surface
        .remove(&BoundarySymbol::from("snow.hourly.rain_m_0001"));

    let report = scheduler
        .execute_with_kernel(&topology_report, &mut kernel, surface)
        .expect("missing hourly snow symbol should return typed report");

    assert_eq!(
        report.scheduler_report.halted_phase,
        Some(HillslopePhase::RunoffReconciliation)
    );

    let runoff_phase = report
        .phase_reports
        .iter()
        .find(|phase| phase.phase == HillslopePhase::RunoffReconciliation)
        .expect("runoff phase report should exist");
    assert_eq!(
        runoff_phase.decision_status.message_id(),
        "HKERNEL-WB14-RUNOFF-E-001"
    );
    assert_eq!(
        runoff_phase.decision_status.boundary_class(),
        BoundaryClass::MissingRequiredInput
    );
}
