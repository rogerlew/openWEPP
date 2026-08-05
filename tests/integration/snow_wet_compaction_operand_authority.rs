use std::fs;
use std::path::{Path, PathBuf};

use openwepp_hillslope_orchestrator::{
    DirectActiveSnowPartitionInputs, DirectSnowHourlyForcing, DirectSnowLayerState,
    DirectSnowSurfaceEnergyOptions, SnowDensityModel, SnowMeltModel, SnowPhasePartitionModel,
    SnowStage3LiquidRoutingModel, Wb11HydrologyKernel,
};
use sha2::{Digest, Sha256};

const CANONICAL_SHA256: &str = "10c1ede130f697ccec01a4fb076d937213f0699e2f6c100492c7a4ef28ec11a7";
const DERIVED_SHA256: &str = "c673145ee7fd41e71e3f2e21c529fba2d12691abd5f0f055444e621fb0b80afb";

fn root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
}

fn read(path: impl AsRef<Path>) -> String {
    fs::read_to_string(path).expect("test authority file must be readable")
}

fn sha256_hex(bytes: &[u8]) -> String {
    format!("{:x}", Sha256::digest(bytes))
}

fn precipitation_tenths_mm(token: &str) -> i64 {
    let (whole, tenth) = token
        .split_once('.')
        .expect("CLIGEN precipitation must retain one decimal place");
    whole
        .parse::<i64>()
        .expect("CLIGEN whole precipitation must be integral")
        * 10
        + tenth
            .parse::<i64>()
            .expect("CLIGEN precipitation tenth must be integral")
}

fn daily_rows(text: &str) -> Vec<Vec<&str>> {
    let mut header_seen = false;
    text.lines()
        .filter_map(|line| {
            if line.trim_start().starts_with("da mo year") {
                header_seen = true;
                return None;
            }
            if !header_seen || line.trim_start().starts_with("(mm)") {
                return None;
            }
            Some(line.split_whitespace().collect::<Vec<_>>())
        })
        .collect()
}

#[test]
fn contract_binds_generated_melt_plus_snow_contact_rain_exactly_once() {
    let contract =
        read(root().join("docs/specifications/science-contracts/contracts/SC-SNOWFREEZE-001.md"));
    for marker in [
        "INV-SNOWFREEZE-092",
        "OBL-SNOWFREEZE-P-065",
        "TOL-SNOWFREEZE-017",
        "sum(max(snow.hourly.melt_raw_m, 0)) + rain_retained + rain_released",
        "snowpack_state_loss + routed_melt",
        "retained-liquid store level/change",
        "PySnobal 0.2.3",
    ] {
        assert!(contract.contains(marker), "contract missing {marker}");
    }
}

#[test]
fn production_source_rejects_the_retired_duplicate_alias() {
    let support = read(root().join(
        "crates/openwepp-hillslope-orchestrator/src/hydrology/03_kernel_support_00_support_helpers.rs",
    ));
    let finalization = read(root().join(
        "crates/openwepp-hillslope-orchestrator/src/hydrology/support_helpers_mod/infiltration_reconciliation.rs",
    ));
    let density_handoff = read(root().join(
        "crates/openwepp-hillslope-orchestrator/src/hydrology/support_helpers_mod/runoff_reconciliation.rs",
    ));
    let snowbench_melt =
        read(root().join("crates/openwepp-runner/src/hillslope/snowbench_coe_melt.rs"));
    let snowbench_density =
        read(root().join("crates/openwepp-runner/src/hillslope/snowbench_coe_density.rs"));

    assert!(support.contains("wet_compaction_liquid_input_m"));
    assert!(finalization.contains("hourly.melt_raw_m.max(0.0)"));
    assert!(finalization.contains("totals.rain_retained_m"));
    assert!(finalization.contains("totals.rain_released_m"));
    assert!(
        density_handoff
            .contains("liquid_for_compaction_m: snow_coupling.wet_compaction_liquid_input_m")
    );
    assert!(
        !density_handoff
            .contains("liquid_for_compaction_m: snow_coupling.snowpack_state_loss + routed_melt_m")
    );
    assert!(snowbench_melt.contains("gross_positive_generated_melt_m"));
    assert!(snowbench_density.contains("boundary_day.gross_positive_generated_melt_m"));
    assert!(snowbench_density.contains("boundary_day.rain_retained_m"));
    assert!(snowbench_density.contains("boundary_day.rain_released_m"));
    assert!(
        !snowbench_density
            .contains("boundary_day.snowpack_swe_loss_m + boundary_day.routed_melt_m")
    );
}

#[test]
fn materiality_tool_fails_closed_on_contract_acceptance_thresholds() {
    let tool = read(root().join(
        "docs/work-packages/20260804-snow-wet-compaction-operand-authority-duplicate-alias-closure-001/tools/run_materiality.py",
    ));
    for marker in [
        "OPERAND_RECONSTRUCTION_TOLERANCE_M = 1.0e-12",
        "MASS_CLOSURE_TOLERANCE_M = 1.0e-9",
        "DENSITY_CLOSURE_TOLERANCE_KG_M3 = 1.0e-9",
        "def require_materiality_acceptance(",
        "materiality acceptance failed",
        "canonical_driver_changed_day_count",
        "canonical_density_changed_day_count",
    ] {
        assert!(tool.contains(marker), "materiality tool missing {marker}");
    }
}

#[test]
fn snowbird_development_cli_is_an_exact_precipitation_only_derivative() {
    let fixture = root().join("tests/fixtures/snotel_observed/snotel_snowbird_ut");
    let canonical_path = fixture.join("p8.cli");
    let derived_dir = fixture.join("development/precip_x1p2155576");
    let derived_path = derived_dir.join("p8.cli");
    let canonical = fs::read(&canonical_path).expect("canonical Snowbird CLI must be readable");
    let derived = fs::read(&derived_path).expect("derived Snowbird CLI must be readable");
    assert_eq!(sha256_hex(&canonical), CANONICAL_SHA256);
    assert_eq!(sha256_hex(&derived), DERIVED_SHA256);

    let canonical_text = String::from_utf8(canonical).expect("canonical CLI must be ASCII");
    let derived_text = String::from_utf8(derived).expect("derived CLI must be ASCII");
    let canonical_rows = daily_rows(&canonical_text);
    let derived_rows = daily_rows(&derived_text);
    assert_eq!(canonical_rows.len(), 14_245);
    assert_eq!(canonical_rows.len(), derived_rows.len());

    let mut changed = 0;
    let mut canonical_total_tenths_mm = 0;
    let mut derived_total_tenths_mm = 0;
    for (canonical_row, derived_row) in canonical_rows.iter().zip(&derived_rows) {
        assert_eq!(canonical_row.len(), 13);
        assert_eq!(canonical_row.len(), derived_row.len());
        assert_eq!(&canonical_row[..3], &derived_row[..3]);
        assert_eq!(&canonical_row[4..], &derived_row[4..]);

        let source_tenths = precipitation_tenths_mm(canonical_row[3]);
        let actual_tenths = precipitation_tenths_mm(derived_row[3]);
        let expected_tenths = (source_tenths * 12_155_576 + 5_000_000) / 10_000_000;
        assert_eq!(actual_tenths, expected_tenths);
        changed += usize::from(source_tenths != actual_tenths);
        canonical_total_tenths_mm += source_tenths;
        derived_total_tenths_mm += actual_tenths;
    }
    assert_eq!(changed, 4_472);
    assert_eq!(canonical_total_tenths_mm, 464_918);
    assert_eq!(derived_total_tenths_mm, 565_191);

    let manifest: serde_json::Value =
        serde_json::from_str(&read(derived_dir.join("manifest.json")))
            .expect("derived fixture manifest must be valid JSON");
    assert_eq!(manifest["classification"], "DEVELOPMENT_ONLY");
    assert_eq!(manifest["source_sha256"], CANONICAL_SHA256);
    assert_eq!(manifest["derived_sha256"], DERIVED_SHA256);
    assert_eq!(
        manifest["transformation"]["factor_decimal_exact"],
        "1.2155576"
    );
    assert_eq!(manifest["transformation"]["rounding"], "ROUND_HALF_UP");
    assert_eq!(manifest["transformation"]["output_resolution_mm"], "0.1");
}

#[test]
fn anti_alias_vector_keeps_every_candidate_numerically_distinct() {
    let gross_positive_generated_melt_m = 0.011;
    let rain_retained_m = 0.003;
    let rain_released_m = 0.005;
    let snowpack_state_loss_m = 0.006;
    let routed_melt_m = snowpack_state_loss_m + rain_released_m;
    let retained_store_change_m = 0.002;
    let authoritative: f64 = gross_positive_generated_melt_m + rain_retained_m + rain_released_m;
    let candidates = [
        snowpack_state_loss_m + routed_melt_m,
        routed_melt_m,
        snowpack_state_loss_m + rain_retained_m + rain_released_m,
        retained_store_change_m,
    ];
    for candidate in candidates {
        assert!((authoritative - candidate).abs() > 1.0e-12);
    }
    for (index, candidate) in candidates.iter().enumerate() {
        assert!(
            candidates[index + 1..]
                .iter()
                .all(|other| (candidate - other).abs() > 1.0e-12)
        );
    }
}

fn wet_compaction_inputs() -> DirectActiveSnowPartitionInputs {
    let mut hourly = [DirectSnowHourlyForcing {
        cloud_fraction: 1.0,
        ..DirectSnowHourlyForcing::zero()
    }; 24];
    hourly[0] = DirectSnowHourlyForcing {
        active_precipitation_m: 0.020,
        rain_m: 0.020,
        radiation_mj_m2: 2.0,
        air_temperature_c: 2.0,
        cloud_fraction: 1.0,
        phase_model: SnowPhasePartitionModel::HarderPomeroyHourly,
        rain_fraction: 1.0,
        snow_fraction: 0.0,
        hydrometeor_temperature_c: Some(1.0),
        ..DirectSnowHourlyForcing::zero()
    };
    hourly[1] = DirectSnowHourlyForcing {
        air_temperature_c: -10.0,
        cloud_fraction: 1.0,
        ..DirectSnowHourlyForcing::zero()
    };

    DirectActiveSnowPartitionInputs {
        hyetograph_rainfall_m: 0.020,
        rst_c: 0.0,
        newsnw_kg_m3: 100.0,
        ssd_kg_m3: 522.0,
        runtime_swe_m: 0.2,
        runtime_depth_m: 1.0,
        runtime_density_kg_m3: 200.0,
        runtime_settle_day_count: 4.0,
        liquid_water_retained_m: 0.0,
        tmax_c: 5.0,
        tmin_c: 5.0,
        canopy_cover_fraction: 0.0,
        wind_m_s: 0.0,
        dewpoint_c: 0.0,
        snow_melt_model: SnowMeltModel::CoeLiquidHoldingCapacityV1,
        snow_density_model: SnowDensityModel::PhysicsBulkDensityCompactionV1,
        stage3_liquid_routing_model: SnowStage3LiquidRoutingModel::Disabled,
        surface_energy_options: DirectSnowSurfaceEnergyOptions::default(),
        sturm_climate_class: None,
        sturm_day_of_year: None,
        coe_boundary_depth_m: 1.0,
        coe_boundary_density_kg_m3: 200.0,
        coe_boundary_settle_day_count: 4.0,
        snow_albedo_model: None,
        snow_albedo_state: None,
        snow_layers: Vec::new(),
        underlying_surface_albedo: 0.2,
        hourly,
    }
}

#[test]
fn mixed_onset_excludes_raw_daily_rain_and_inactive_path_stays_zero() {
    let mut onset = wet_compaction_inputs();
    onset.runtime_swe_m = 0.0;
    onset.runtime_depth_m = 0.0;
    onset.runtime_density_kg_m3 = 0.0;
    onset.runtime_settle_day_count = 0.0;
    onset.coe_boundary_depth_m = 0.0;
    onset.coe_boundary_density_kg_m3 = 0.0;
    onset.coe_boundary_settle_day_count = 0.0;
    onset.hyetograph_rainfall_m = 0.030;
    onset.tmax_c = -1.0;
    onset.tmin_c = -5.0;
    onset.hourly = [DirectSnowHourlyForcing {
        air_temperature_c: -5.0,
        cloud_fraction: 1.0,
        ..DirectSnowHourlyForcing::zero()
    }; 24];
    onset.hourly[0] = DirectSnowHourlyForcing {
        active_precipitation_m: 0.030,
        rain_m: 0.020,
        snowfall_m: 0.100,
        air_temperature_c: -1.0,
        cloud_fraction: 1.0,
        phase_model: SnowPhasePartitionModel::HarderPomeroyHourly,
        rain_fraction: 2.0 / 3.0,
        snow_fraction: 1.0 / 3.0,
        hydrometeor_temperature_c: Some(-1.0),
        ..DirectSnowHourlyForcing::zero()
    };
    let onset_outcome =
        Wb11HydrologyKernel::compute_direct_snow_liquid_partition_from_typed(&onset)
            .expect("mixed onset vector must compute");
    assert!(onset_outcome.active_snow_coupling);
    assert!(onset_outcome.runtime_swe_after_m > 0.0);
    assert!((onset_outcome.post_winter_rain_m - 0.020).abs() <= 1.0e-12);
    assert!(onset_outcome.rain_retained_m.abs() <= 1.0e-12);
    assert!(onset_outcome.solid_to_liquid_ledger().rain_released_m.abs() <= 1.0e-12);
    assert!(
        onset_outcome
            .density_process_diagnostics
            .liquid_for_compaction_mass_kg_m2
            .abs()
            <= 1.0e-12
    );

    let mut inactive = onset;
    inactive.hyetograph_rainfall_m = 0.0;
    inactive.hourly = [DirectSnowHourlyForcing::zero(); 24];
    let inactive_outcome =
        Wb11HydrologyKernel::compute_direct_snow_liquid_partition_from_typed(&inactive)
            .expect("inactive vector must compute");
    assert!(!inactive_outcome.active_snow_coupling);
    assert!(!inactive_outcome.density_process_diagnostics.applicable);
    assert!(
        inactive_outcome
            .density_process_diagnostics
            .liquid_for_compaction_mass_kg_m2
            .abs()
            <= 1.0e-12
    );
}

#[test]
fn multilayer_real_consumer_receives_the_same_exact_operand() {
    let mut inputs = wet_compaction_inputs();
    inputs.snow_density_model = SnowDensityModel::PhysicsBulkMultilayerDensityV1;
    inputs.snow_layers = vec![
        DirectSnowLayerState::new(0.08, 0.40, 200.0, 4.0),
        DirectSnowLayerState::new(0.12, 0.60, 200.0, 4.0),
    ];
    let outcome = Wb11HydrologyKernel::compute_direct_snow_liquid_partition_from_typed(&inputs)
        .expect("multilayer wet-compaction vector must compute");
    let gross_positive_generated_melt_m = outcome
        .verbose_diagnostics
        .as_ref()
        .expect("multilayer proof requires verbose diagnostics")
        .accumulation_melt
        .hourly_melt
        .iter()
        .map(|hour| hour.coe_melt_applied_m.max(0.0))
        .sum::<f64>();
    let ledger = outcome.solid_to_liquid_ledger();
    let expected_m =
        gross_positive_generated_melt_m + outcome.rain_retained_m + ledger.rain_released_m;
    let actual_m = outcome
        .density_process_diagnostics
        .liquid_for_compaction_mass_kg_m2
        / 1_000.0;
    assert!((actual_m - expected_m).abs() <= 1.0e-12);
    assert!(outcome.snow_layers_after.len() >= 2);
    let layer_swe_m = outcome
        .snow_layers_after
        .iter()
        .map(|layer| layer.mass_swe_m)
        .sum::<f64>();
    let layer_depth_m = outcome
        .snow_layers_after
        .iter()
        .map(|layer| layer.thickness_m)
        .sum::<f64>();
    assert!((layer_swe_m - outcome.runtime_swe_after_m).abs() <= 1.0e-12);
    assert!((layer_depth_m - outcome.runtime_depth_after_m).abs() <= 1.0e-12);
}

#[test]
fn real_density_consumer_uses_generated_melt_and_contact_rain_once() {
    let inputs = wet_compaction_inputs();
    let outcome = Wb11HydrologyKernel::compute_direct_snow_liquid_partition_from_typed(&inputs)
        .expect("wet-compaction anti-alias vector must compute");
    let verbose = outcome
        .verbose_diagnostics
        .as_ref()
        .expect("real consumer proof requires selected verbose diagnostics");
    let gross_positive_generated_melt_m = verbose
        .accumulation_melt
        .hourly_melt
        .iter()
        .map(|hour| hour.coe_melt_applied_m.max(0.0))
        .sum::<f64>();
    let actual_m = outcome
        .density_process_diagnostics
        .liquid_for_compaction_mass_kg_m2
        / 1_000.0;
    let ledger = outcome.solid_to_liquid_ledger();
    let authoritative_m =
        gross_positive_generated_melt_m + outcome.rain_retained_m + ledger.rain_released_m;

    assert!(gross_positive_generated_melt_m > 0.0);
    assert!(outcome.rain_retained_m > 0.0);
    assert!(ledger.rain_released_m > 0.0);
    assert!((actual_m - authoritative_m).abs() <= 1.0e-12);

    let retained_store_change_m =
        outcome.liquid_water_retained_after_m - inputs.liquid_water_retained_m;
    for rejected in [
        ledger.snowpack_swe_loss_m + ledger.liquid_handoff_m,
        ledger.liquid_handoff_m,
        ledger.snowpack_swe_loss_m + outcome.rain_retained_m + ledger.rain_released_m,
        outcome.liquid_water_retained_after_m,
        retained_store_change_m,
    ] {
        assert!(
            (actual_m - rejected).abs() > 1.0e-12,
            "real consumer accepted adjacent alias {rejected:.15} for authoritative {actual_m:.15}"
        );
    }

    let storage_residual_m =
        inputs.runtime_swe_m + outcome.accumulation_m + outcome.rain_retained_m
            - ledger.snowpack_swe_loss_m
            - outcome.sublimation_m
            - outcome.runtime_swe_after_m;
    assert!(storage_residual_m.abs() <= 1.0e-9);
    assert!(
        (ledger.liquid_handoff_m - ledger.snowpack_swe_loss_m - ledger.rain_released_m).abs()
            <= 1.0e-9
    );
    assert!(
        outcome
            .density_process_diagnostics
            .closure_residual_kg_m3
            .abs()
            <= 1.0e-9
    );
}
