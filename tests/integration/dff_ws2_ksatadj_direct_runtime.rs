use std::fs;
use std::path::{Path, PathBuf};
use std::time::{Duration, Instant};

use openwepp_runner::{HillslopeRunRequest, SidecarPolicy};
use serde_json::Value;

mod common;

#[test]
fn dff_ws2_forest_high_severity_loam_runs_with_live_direct_ksatadj_effect() {
    let fixture = fixture_path();
    let ksatadj_on_dir = copy_fixture_to_temp(&fixture, "dff_ws2_ksatadj_p313_on");
    let _telemetry = openwepp_hillslope_orchestrator::snow_stage3_v11_attachment::begin_adaptive_parent_telemetry_v1(
        10_000,
        Duration::from_secs(3_600),
    )
    .expect("bounded test-only adaptive telemetry");
    let _fixed_point_audit = openwepp_hillslope_orchestrator::snow_stage3_v11_attachment::begin_covered_fixed_point_iteration_audit_v1();
    openwepp_hillslope_orchestrator::snow_stage3_v11_attachment::begin_stage3_physical_outcome_closure_audit_v1();
    let started = Instant::now();
    let run = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        run_fixture(&ksatadj_on_dir)
    }));
    let fixed_point_audit = openwepp_hillslope_orchestrator::snow_stage3_v11_attachment::take_covered_fixed_point_iteration_audit_v1();
    let limiter_audit = openwepp_hillslope_orchestrator::snow_stage3_v11_attachment::take_covered_fixed_point_limiter_audit_v1();
    emit_fixed_point_limiter_summary_v1(
        if run.is_ok() { "success" } else { "failure" },
        &limiter_audit,
    );
    let ksatadj_on_report = match run {
        Ok(report) => report,
        Err(panic) => {
            eprintln!(
                "DFF_V32_FIXED_POINT_FAILURE entries={} tail={:?}",
                fixed_point_audit.len(),
                fixed_point_audit.iter().rev().take(8).collect::<Vec<_>>()
            );
            std::panic::resume_unwind(panic);
        }
    };
    let runtime = started.elapsed();
    let adaptive = openwepp_hillslope_orchestrator::snow_stage3_v11_attachment::take_adaptive_parent_telemetry_v1();
    let closure = openwepp_hillslope_orchestrator::snow_stage3_v11_attachment::take_stage3_physical_outcome_closure_audit_v1();
    assert_v32_adaptive_and_closure_metrics(&adaptive, closure, runtime);
    assert_p313_outputs(&ksatadj_on_report);
    assert!(
        ksatadj_evaluation_count(&ksatadj_on_report) > 0,
        "DFF-WS2 p313 should invoke the direct ksatadj evaluator when ksatadj=1"
    );

    let ksatadj_off_dir = copy_fixture_to_temp(&fixture, "dff_ws2_ksatadj_p313_off");
    disable_ksatadj_in_soil(&ksatadj_off_dir.join("p313.sol"));
    let ksatadj_off_report = run_fixture(&ksatadj_off_dir);
    assert_p313_outputs(&ksatadj_off_report);
    assert_eq!(
        ksatadj_evaluation_count(&ksatadj_off_report),
        0,
        "DFF-WS2 p313 should not invoke the direct ksatadj evaluator when only soil ksatadj is disabled"
    );
}

fn emit_fixed_point_limiter_summary_v1(
    outcome: &str,
    audit: &openwepp_hillslope_orchestrator::snow_stage3_v11_attachment::CoveredFixedPointLimiterAuditV1,
) {
    let tail = audit
        .retained_tail
        .iter()
        .rev()
        .take(24)
        .map(|sample| {
            (
                sample.support,
                sample.iteration,
                sample.stage,
                (
                    sample.lse_converged,
                    sample.stage3_converged,
                    sample.soil_converged,
                    sample.boundary_converged,
                ),
                (
                    f64::from_bits(sample.lse_max_normalized_delta_bits),
                    f64::from_bits(sample.stage3_max_normalized_delta_bits),
                    f64::from_bits(sample.soil_enthalpy_max_normalized_delta_bits),
                    f64::from_bits(sample.soil_temperature_max_normalized_delta_bits),
                    f64::from_bits(sample.boundary_max_normalized_delta_bits),
                ),
            )
        })
        .collect::<Vec<_>>();
    eprintln!(
        "DFF_V32_PICARD_LIMITER outcome={} samples={} dropped={} peak_normalized=(lse={:.9e},stage3={:.9e},soil_enthalpy={:.9e},soil_temperature={:.9e},boundary={:.9e}) reverse_tail=(support,iteration,stage,converged[lse,stage3,soil,boundary],normalized_delta[lse,stage3,soil_enthalpy,soil_temperature,boundary])={:?}",
        outcome,
        audit.total_sample_count,
        audit.dropped_sample_count,
        f64::from_bits(audit.peak_lse_normalized_delta_bits),
        f64::from_bits(audit.peak_stage3_normalized_delta_bits),
        f64::from_bits(audit.peak_soil_enthalpy_normalized_delta_bits),
        f64::from_bits(audit.peak_soil_temperature_normalized_delta_bits),
        f64::from_bits(audit.peak_boundary_normalized_delta_bits),
        tail,
    );
}

fn assert_v32_adaptive_and_closure_metrics(
    adaptive: &[openwepp_hillslope_orchestrator::snow_stage3_v11_attachment::AdaptiveParentTelemetryV1],
    closure: openwepp_hillslope_orchestrator::snow_stage3_v11_attachment::Stage3PhysicalOutcomeClosureAuditV1,
    runtime: Duration,
) {
    let accepted = adaptive
        .iter()
        .map(|row| row.accepted_microstep_count)
        .sum::<u64>();
    let rejected = adaptive
        .iter()
        .map(|row| row.rejected_candidate_count)
        .sum::<u64>();
    let phase = adaptive
        .iter()
        .map(|row| row.phase_rejection_count)
        .sum::<u64>();
    let event = adaptive
        .iter()
        .map(|row| row.event_rejection_count)
        .sum::<u64>();
    let both = adaptive
        .iter()
        .map(|row| row.phase_and_event_rejection_count)
        .sum::<u64>();
    let other = adaptive
        .iter()
        .map(|row| row.other_rejection_count)
        .sum::<u64>();
    let mut widths = std::collections::BTreeMap::<u128, u64>::new();
    for row in adaptive {
        for (width_ns, count) in &row.accepted_width_histogram {
            *widths.entry(*width_ns).or_default() += count;
        }
    }
    assert!(
        accepted > 0,
        "canonical fixture must accept adaptive supports"
    );
    assert!(
        widths.keys().any(|width_ns| *width_ns > 60_000_000_000),
        "ordinary stable supports must accept above the 60-second fallback: {widths:?}"
    );
    assert!(closure.validated_ledger_count > 0);
    assert!(closure.maximum_abs_mass_residual_kg_m2 <= 1.0e-9);
    assert!(closure.maximum_abs_energy_residual_j_m2 <= 1.0e-6);
    eprintln!(
        "DFF_V32_METRICS runtime_s={:.6} parents={} accepted={} rejected={} widths_ns={:?} phase_rejections={} event_rejections={} phase_event_rejections={} other_rejections={} ledger_count={} max_abs_mass_including_vapor_residual_kg_m2={:.17e} max_abs_energy_residual_j_m2={:.17e}",
        runtime.as_secs_f64(),
        adaptive.len(),
        accepted,
        rejected,
        widths,
        phase,
        event,
        both,
        other,
        closure.validated_ledger_count,
        closure.maximum_abs_mass_residual_kg_m2,
        closure.maximum_abs_energy_residual_j_m2,
    );
}

fn run_fixture(temp_run_dir: &Path) -> openwepp_runner::HillslopeRunReport {
    common::execute_with_complete_stage3_owner_seed(
        &HillslopeRunRequest {
            run_dir: temp_run_dir.to_path_buf(),
            run_file: PathBuf::from("p313.run.toml"),
            output_dir: temp_run_dir.join("output"),
            sidecar_policy: SidecarPolicy::Compat,
            legacy_sidecar_discovery: false,
            manifest_path: None,
        },
        &["openwepp-cli-hill".to_string()],
    )
    .expect("DFF-WS2 disturbed-burn p313 fixture should run end-to-end")
}

fn assert_p313_outputs(report: &openwepp_runner::HillslopeRunReport) {
    assert!(report.output_pass.is_file());
    assert!(report.output_loss.is_file());
    assert!(
        report
            .optional_outputs
            .iter()
            .any(|path| path.ends_with("H313.wat.parquet")),
        "DFF-WS2 fixture should publish the optional WAT output"
    );
    let manifest = fs::read_to_string(&report.manifest_path).expect("manifest should read");
    assert!(manifest.contains("openwepp-hillslope-run-manifest-v1"));
    assert!(manifest.contains("H313.hbp"));
    assert!(manifest.contains("H313.loss.json"));
    assert!(manifest.contains("H313.wat.parquet"));
}

fn ksatadj_evaluation_count(report: &openwepp_runner::HillslopeRunReport) -> u64 {
    let manifest = fs::read_to_string(&report.manifest_path).expect("manifest should read");
    let manifest: Value = serde_json::from_str(&manifest).expect("manifest should parse");
    manifest
        .pointer("/direct_runtime_counters/ksatadj_effective_conductivity_evaluations")
        .and_then(Value::as_u64)
        .expect("manifest should carry the direct ksatadj evaluation counter")
}

fn disable_ksatadj_in_soil(soil_path: &Path) {
    let contents = fs::read_to_string(soil_path).expect("soil fixture should read");
    let mut changed = false;
    let updated = contents
        .lines()
        .map(|line| {
            if !changed && line.starts_with("1\t 'forest high sev fire'\t 'loam'") {
                changed = true;
                line.replacen("1\t", "0\t", 1)
            } else {
                line.to_string()
            }
        })
        .collect::<Vec<_>>()
        .join("\n");
    assert!(
        changed,
        "p313 disturbed-policy row should be present for ksatadj-off comparison"
    );
    fs::write(soil_path, format!("{updated}\n")).expect("soil fixture should be writable");
}

fn fixture_path() -> PathBuf {
    Path::new(file!())
        .parent()
        .expect("integration file parent exists")
        .parent()
        .expect("tests directory exists")
        .join("fixtures")
        .join("disturbed_burn")
        .join("forest_high_severity_loam")
}

fn copy_fixture_to_temp(source_dir: &Path, prefix: &str) -> PathBuf {
    let timestamp = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .expect("unix epoch should be before now")
        .as_nanos();
    let destination = std::env::temp_dir().join(format!("{prefix}_{timestamp}"));

    copy_dir_recursive(source_dir, &destination);
    destination
}

fn copy_dir_recursive(source: &Path, destination: &Path) {
    fs::create_dir_all(destination).expect("destination directory should be creatable");

    for entry in fs::read_dir(source).expect("source directory should be readable") {
        let entry = entry.expect("directory entry should be readable");
        let source_path = entry.path();
        let destination_path = destination.join(entry.file_name());

        if source_path.is_dir() {
            copy_dir_recursive(&source_path, &destination_path);
        } else {
            fs::copy(&source_path, &destination_path).expect("file copy should succeed");
        }
    }
}
