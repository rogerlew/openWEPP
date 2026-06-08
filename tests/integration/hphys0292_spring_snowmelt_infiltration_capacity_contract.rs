use std::collections::BTreeMap;
use std::fs;

use openwepp_hillslope_orchestrator::{HillslopeWritebackSurface, Wb11HydrologyKernel};
use openwepp_kernel_contract::{
    BoundarySymbol, BoundaryValue, HillslopeConsumerAdapter, HillslopeKernel,
    HillslopeKernelPhaseClass, HillslopeKernelRequest, KernelRunResponse,
};

const KERNEL_HELPER_SOURCES: [&str; 2] = [
    "crates/openwepp-hillslope-orchestrator/src/hydrology/03_kernel_support_00_support_helpers.rs",
    "crates/openwepp-hillslope-orchestrator/src/hydrology/support_helpers_mod/runoff_reconciliation.rs",
];
const KERNEL_PHASE_SOURCE: &str =
    "crates/openwepp-hillslope-orchestrator/src/hydrology/03_kernel_support_01_kernel_phases.rs";
const SC_SNOWFREEZE: &str = "docs/specifications/science-contracts/contracts/SC-SNOWFREEZE-001.md";
const SC_RUNOFFPART: &str = "docs/specifications/science-contracts/contracts/SC-RUNOFFPART-001.md";
const SC_WATBAL: &str = "docs/specifications/science-contracts/contracts/SC-WATBAL-001.md";
const TOL: f64 = 1.0e-9;

fn insert_state(surface: &mut HillslopeWritebackSurface, symbol: &str, value: f64) {
    surface
        .state_surface
        .insert(BoundarySymbol::from(symbol), BoundaryValue::scalar(value));
}

#[allow(clippy::too_many_lines)]
fn high_capacity_active_snowmelt_surface() -> HillslopeWritebackSurface {
    let mut surface = HillslopeWritebackSurface {
        state_surface: BTreeMap::new(),
        flux_surface: BTreeMap::new(),
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

fn realistic_capacity_full_day_snowmelt_surface() -> HillslopeWritebackSurface {
    let mut surface = high_capacity_active_snowmelt_surface();
    insert_state(&mut surface, "ssc", 40.0 / 3_600_000.0);
    for hour in 1..=24 {
        insert_state(
            &mut surface,
            &format!("winter.hourly.rad_mj_m2_{hour:04}"),
            0.329_489,
        );
        insert_state(
            &mut surface,
            &format!("winter.hourly.air_temp_c_{hour:04}"),
            5.0,
        );
    }
    surface
}

fn run_runoff_response(surface: HillslopeWritebackSurface) -> KernelRunResponse {
    let state_surface = Box::leak(Box::new(surface.state_surface));
    let flux_surface = Box::leak(Box::new(surface.flux_surface));
    let request = HillslopeKernelRequest::with_transition_context(
        "runoff_reconciliation",
        HillslopeKernelPhaseClass::HydrologyRunoffReconciliation,
        HillslopeConsumerAdapter::Runoff,
        None,
        None,
        state_surface,
        flux_surface,
    );
    let mut kernel = Wb11HydrologyKernel;
    kernel.run_hillslope_phase(&request)
}

fn writeback_flux_value(response: &KernelRunResponse, symbol: &str) -> f64 {
    response
        .writeback
        .flux_updates
        .iter()
        .find(|field| field.symbol == BoundarySymbol::from(symbol))
        .unwrap_or_else(|| panic!("missing flux writeback symbol {symbol}"))
        .value
        .as_f64()
}

fn writeback_state_value(response: &KernelRunResponse, symbol: &str) -> f64 {
    response
        .writeback
        .state_updates
        .iter()
        .find(|field| field.symbol == BoundarySymbol::from(symbol))
        .unwrap_or_else(|| panic!("missing state writeback symbol {symbol}"))
        .value
        .as_f64()
}

#[test]
fn hphys0292_contracts_define_spring_snowmelt_capacity_localization() {
    let snow = fs::read_to_string(SC_SNOWFREEZE).expect("snow contract should be readable");
    let runoff = fs::read_to_string(SC_RUNOFFPART).expect("runoff contract should be readable");
    let watbal = fs::read_to_string(SC_WATBAL).expect("watbal contract should be readable");

    assert!(
        snow.contains("INV-SNOWFREEZE-025")
            && snow.contains("spring snowmelt producer-partition localization invariant"),
        "SC-SNOWFREEZE must define HPHYS0292 producer-partition authority"
    );
    assert!(
        runoff.contains("INV-RUNOFFPART-022")
            && runoff.contains("spring snowmelt infiltration-capacity localization invariant"),
        "SC-RUNOFFPART must define HPHYS0292 WB12 capacity authority"
    );
    assert!(
        watbal.contains("INV-WATBAL-067")
            && watbal.contains("spring snowmelt/infiltration capacity lineage invariant"),
        "SC-WATBAL must define HPHYS0292 water-balance localization authority"
    );
}

#[test]
fn hphys0292_source_preserves_wmelt_infiltration_before_residual_q() {
    let helpers = KERNEL_HELPER_SOURCES
        .iter()
        .map(|path| {
            fs::read_to_string(path)
                .unwrap_or_else(|_| panic!("kernel helper source {path} should be readable"))
        })
        .collect::<String>();
    let phases =
        fs::read_to_string(KERNEL_PHASE_SOURCE).expect("kernel phase source should be readable");

    assert!(
        helpers.contains("fn compute_coupled_infiltration_depth")
            && helpers.contains("snowmelt_hourly_state: &[SnowHourlyState]")
            && helpers.contains("snowmelt_shape_scale")
            && helpers.contains("bounded_interval_overlap_duration")
            && helpers.contains("BoundarySymbol::from(\"snow.routed_melt_m\")"),
        "coupled infiltration helper must include routed snowmelt as event liquid"
    );
    assert!(
        phases.contains("liquid_after_interception + runoff_snow_term")
            && phases.contains("Self::compute_runoff_after_interception(")
            && phases.contains("runoff_snow_term,")
            && phases.contains("wb14_effective_conductivity_m_s")
            && phases.contains("wb14_matric_potential_m"),
        "runoff residual must be computed after routed melt has been offered to infiltration"
    );
}

#[test]
fn hphys0292_active_snowmelt_infiltrates_before_residual_runoff() {
    let response = run_runoff_response(high_capacity_active_snowmelt_surface());
    assert_eq!(response.status.message_id(), "HKERNEL-WB14-RUNOFF-OK-001");

    let routed_melt = writeback_flux_value(&response, "snow.routed_melt_m");
    let post_winter_rain = writeback_flux_value(&response, "snow.post_winter_rain_m");
    let infiltration = writeback_state_value(&response, "wb12_infiltration");
    let q = writeback_flux_value(&response, "Q");

    assert!(
        routed_melt > 0.0,
        "test vector must generate active routed melt, observed {routed_melt}"
    );
    assert!(
        post_winter_rain.abs() <= TOL,
        "snowmelt-only vector must not publish direct post-winter rain, observed {post_winter_rain}"
    );
    assert!(
        (infiltration - routed_melt).abs() <= TOL,
        "high-capacity WB12 path must offer routed melt to infiltration before Q; infiltration={infiltration}, routed_melt={routed_melt}"
    );
    assert!(
        q.abs() <= TOL,
        "high-capacity active snowmelt vector should leave no residual Q, observed {q}"
    );
}

#[test]
fn hphys0292_snowmelt_only_capacity_uses_positive_hour_duration() {
    let response = run_runoff_response(realistic_capacity_full_day_snowmelt_surface());
    assert_eq!(response.status.message_id(), "HKERNEL-WB14-RUNOFF-OK-001");

    let routed_melt = writeback_flux_value(&response, "snow.routed_melt_m");
    let infiltration = writeback_state_value(&response, "wb12_infiltration");
    let q = writeback_flux_value(&response, "Q");

    assert!(
        routed_melt > 0.020,
        "test vector must generate material full-day routed melt, observed {routed_melt}"
    );
    assert!(
        (infiltration - routed_melt).abs() <= TOL,
        "40 mm/h effective capacity over positive melt hours should absorb snowmelt-only forcing; infiltration={infiltration}, routed_melt={routed_melt}"
    );
    assert!(
        q.abs() <= TOL,
        "snowmelt-only full-day capacity vector should leave no residual Q, observed {q}"
    );
}
