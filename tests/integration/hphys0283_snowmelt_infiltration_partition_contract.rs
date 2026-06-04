use openwepp_hillslope_orchestrator::{
    HillslopePhaseScheduler, HillslopeWritebackSurface, Wb11HydrologyKernel,
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
fn snowmelt_only_surface() -> HillslopeWritebackSurface {
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
        ("snow.runtime_swe", 0.050),
        ("snow.runtime_depth_m", 0.100),
        ("snow.runtime_density_kg_m3", 500.0),
        ("snow.runtime_settle_day_count", 1.0),
        ("tmax", 5.0),
        ("tmin", 1.0),
        ("vwind", 0.0),
        ("tdpt", 0.0),
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
            if hour == 12 { 0.329_489 } else { 0.0 },
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

#[test]
fn hphys0283_contract_snowmelt_is_offered_to_infiltration_before_runoff() {
    let graph = parse_topology_fixture_str(VALID_TOPOLOGY).expect("fixture should parse");
    let topology_report =
        validate_pre_execution_topology(&graph).expect("topology report should build");
    let scheduler = HillslopePhaseScheduler::canonical();
    let mut kernel = Wb11HydrologyKernel;

    let report = scheduler
        .execute_with_kernel(&topology_report, &mut kernel, snowmelt_only_surface())
        .expect("HPHYS0283 snowmelt vector should return typed report");

    assert!(
        report.scheduler_report.is_success(),
        "scheduler halted at {:?}",
        report.scheduler_report.halted_phase
    );

    let infiltration = report
        .writeback_surface
        .state_surface
        .get(&BoundarySymbol::from("wb12_infiltration"))
        .expect("wb12_infiltration should be present")
        .as_f64();
    let runoff = report
        .writeback_surface
        .flux_surface
        .get(&BoundarySymbol::from("Q"))
        .expect("Q should be present")
        .as_f64();
    let snow_melt = report
        .writeback_surface
        .state_surface
        .get(&BoundarySymbol::from("snow.hourly.melt_m_0012"))
        .expect("hour 12 melt should be present")
        .as_f64();
    let soil_water = report
        .writeback_surface
        .state_surface
        .get(&BoundarySymbol::from("wb11_soil_water"))
        .expect("wb11_soil_water should be present")
        .as_f64();
    let theta_1 = report
        .writeback_surface
        .state_surface
        .get(&BoundarySymbol::from("wb18_perc_theta_0001"))
        .expect("layer 1 storage should be present")
        .as_f64();
    let theta_2 = report
        .writeback_surface
        .state_surface
        .get(&BoundarySymbol::from("wb18_perc_theta_0002"))
        .expect("layer 2 storage should be present")
        .as_f64();

    assert!(
        (snow_melt - 0.000_508).abs() <= 1.0e-6,
        "test vector should produce controlled melt near 0.508 mm, observed {snow_melt}"
    );
    assert!(
        (infiltration - snow_melt).abs() <= 1.0e-6,
        "meltwater must infiltrate when conductivity can absorb it; infiltration={infiltration}, melt={snow_melt}"
    );
    assert!(
        runoff.abs() <= TOL,
        "meltwater should not become runoff when it all infiltrates, observed {runoff}"
    );
    assert!(
        soil_water > 10.0 + 0.5 * snow_melt,
        "infiltrated melt must materially update aggregate WB11 storage before downstream losses; soil_water={soil_water}, melt={snow_melt}"
    );
    assert!(
        (theta_1 + theta_2) > 10.0 + 0.5 * snow_melt,
        "infiltrated melt must materially update WB18 layer storage before downstream losses; theta_sum={}, melt={snow_melt}",
        theta_1 + theta_2
    );
}
