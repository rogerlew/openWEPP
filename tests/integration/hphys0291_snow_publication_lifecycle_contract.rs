use std::collections::BTreeMap;
use std::fs;
use std::path::{Path, PathBuf};

use openwepp_hillslope_orchestrator::{HillslopeWritebackSurface, Wb11HydrologyKernel};
use openwepp_kernel_contract::{
    BoundarySymbol, BoundaryValue, HillslopeConsumerAdapter, HillslopeKernel,
    HillslopeKernelPhaseClass, HillslopeKernelRequest, KernelRunResponse,
};

const KERNEL_PHASE_SOURCE: &str =
    "crates/openwepp-hillslope-orchestrator/src/hydrology/03_kernel_support_01_kernel_phases.rs";
const SC_SNOWFREEZE: &str = "docs/specifications/science-contracts/contracts/SC-SNOWFREEZE-001.md";
const SC_RUNOFFPART: &str = "docs/specifications/science-contracts/contracts/SC-RUNOFFPART-001.md";
const SC_WATBAL: &str = "docs/specifications/science-contracts/contracts/SC-WATBAL-001.md";
const TOL: f64 = 1.0e-9;

fn collect_runner_source_files(dir: &Path, files: &mut Vec<PathBuf>) {
    for entry in fs::read_dir(dir).expect("runner hillslope source entry should be readable") {
        let entry = entry.expect("runner hillslope source entry should be readable");
        let path = entry.path();
        if path.is_dir() {
            collect_runner_source_files(&path, files);
        } else if path.extension().and_then(|ext| ext.to_str()) == Some("rs") {
            files.push(path);
        }
    }
}

fn read_runner_hillslope_sources() -> String {
    let runner_dir =
        Path::new(env!("CARGO_MANIFEST_DIR")).join("crates/openwepp-runner/src/hillslope");
    let mut files: Vec<PathBuf> = Vec::new();
    collect_runner_source_files(&runner_dir, &mut files);
    files.sort();

    files
        .into_iter()
        .map(|path| {
            fs::read_to_string(&path).unwrap_or_else(|error| {
                panic!(
                    "runner source {} should be readable: {error}",
                    path.display()
                )
            })
        })
        .collect::<Vec<_>>()
        .join("\n")
}

fn insert_state(surface: &mut HillslopeWritebackSurface, symbol: &str, value: f64) {
    surface
        .state_surface
        .insert(BoundarySymbol::from(symbol), BoundaryValue::scalar(value));
}

#[allow(clippy::too_many_lines)]
fn snowmelt_runoff_surface() -> HillslopeWritebackSurface {
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

fn dry_no_snow_runoff_surface() -> HillslopeWritebackSurface {
    let mut surface = snowmelt_runoff_surface();
    for (symbol, value) in [
        ("snow.options.snow_file_present", 0.0),
        ("snow.runtime_swe", 0.0),
        ("snow.runtime_depth_m", 0.0),
        ("snow.runtime_density_kg_m3", 0.0),
        ("snow.runtime_settle_day_count", 0.0),
        ("tmax", 0.0),
        ("tmin", 0.0),
    ] {
        insert_state(&mut surface, symbol, value);
    }
    for hour in 1..=24 {
        insert_state(
            &mut surface,
            &format!("winter.hourly.rad_mj_m2_{hour:04}"),
            0.0,
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

#[test]
fn hphys0291_contracts_define_same_day_snow_publication_lifecycle() {
    let snow = fs::read_to_string(SC_SNOWFREEZE).expect("snow contract should be readable");
    let runoff = fs::read_to_string(SC_RUNOFFPART).expect("runoff contract should be readable");
    let watbal = fs::read_to_string(SC_WATBAL).expect("watbal contract should be readable");

    assert!(
        snow.contains("INV-SNOWFREEZE-024")
            && snow.contains("same-day snow publication lifecycle invariant"),
        "SC-SNOWFREEZE must define HPHYS0291 same-day lifecycle authority"
    );
    assert!(
        runoff.contains("INV-RUNOFFPART-021")
            && runoff.contains("runoff reconciliation owns same-day publication"),
        "SC-RUNOFFPART must define HPHYS0291 producer lifecycle authority"
    );
    assert!(
        watbal.contains("INV-WATBAL-066")
            && watbal.contains("downstream of a same-day producer flux lifecycle"),
        "SC-WATBAL must define HPHYS0291 WB13 consumer lifecycle authority"
    );
}

#[test]
fn hphys0291_kernel_source_uses_named_snow_publication_helper() {
    let kernel =
        fs::read_to_string(KERNEL_PHASE_SOURCE).expect("kernel phase source should be readable");

    assert!(
        kernel.contains("fn publish_same_day_snow_publication_fluxes"),
        "runoff reconciliation must use a named same-day snow publication helper"
    );
    assert!(
        kernel.contains("BoundarySymbol::from(\"snow.post_winter_rain_m\")"),
        "helper must publish snow.post_winter_rain_m"
    );
    assert!(
        kernel.contains("BoundarySymbol::from(\"snow.routed_melt_m\")"),
        "helper must publish snow.routed_melt_m"
    );
    assert!(
        kernel.contains("HPHYS0291 same-day snow publication lifecycle"),
        "producer helper must carry explicit HPHYS0291 lifecycle provenance"
    );
}

#[test]
fn hphys0291_kernel_publishes_required_snow_fluxes_on_runoff_reconciliation() {
    let dry_response = run_runoff_response(dry_no_snow_runoff_surface());
    assert_eq!(
        dry_response.status.message_id(),
        "HKERNEL-WB14-RUNOFF-OK-001"
    );
    assert!(
        writeback_flux_value(&dry_response, "snow.routed_melt_m").abs() <= TOL,
        "dry/no-snow run must publish explicit zero routed melt"
    );
    assert!(
        writeback_flux_value(&dry_response, "snow.post_winter_rain_m").abs() <= TOL,
        "dry/no-snow run must publish explicit zero post-winter rain"
    );

    let active_response = run_runoff_response(snowmelt_runoff_surface());
    assert_eq!(
        active_response.status.message_id(),
        "HKERNEL-WB14-RUNOFF-OK-001"
    );
    let routed_melt_m = writeback_flux_value(&active_response, "snow.routed_melt_m");
    let post_winter_rain_m = writeback_flux_value(&active_response, "snow.post_winter_rain_m");
    assert!(
        routed_melt_m > 0.0,
        "active snow vector must publish positive routed melt, observed {routed_melt_m}"
    );
    assert!(
        post_winter_rain_m.abs() <= TOL,
        "snowmelt-only vector must publish explicit zero post-winter rain, observed {post_winter_rain_m}"
    );
}

#[test]
fn hphys0291_wb13_requires_same_day_fluxes_without_state_default_masking() {
    let runner = read_runner_hillslope_sources();

    assert!(
        runner.contains(
            "require_runtime_flux_surface_scalar(runtime_surface, \"snow.post_winter_rain_m\")"
        ),
        "WB13 must require post-winter rain from the same-day flux surface"
    );
    assert!(
        runner.contains("hphys0291_wb13_rm_publication_rejects_state_only_routed_melt"),
        "WB13 tests must reject state-only routed melt when lifecycle authority is active"
    );
    assert!(
        !runner.contains("reset_daily_snow_publication_fluxes"),
        "daily lifecycle must not satisfy required snow publication by reset/default state"
    );
}

#[test]
fn hphys0291_trace_preserves_snow_publication_lifecycle_surfaces() {
    let runner = read_runner_hillslope_sources();

    assert!(
        runner.contains("snow_routed_melt_m")
            && runner.contains("snow_post_winter_rain_m")
            && runner.contains(
                "runtime_surface_flux_symbol_value(runtime_surface, \"snow.routed_melt_m\")"
            )
            && runner.contains(
                "runtime_surface_flux_symbol_value(runtime_surface, \"snow.post_winter_rain_m\")"
            )
            && runner.contains("openwepp-hphys0245-wb11-wb18-wb19-wb17-evappm-branch-trace-v17"),
        "trace schema must expose flux-only routed melt and post-winter rain lifecycle surfaces"
    );
}
