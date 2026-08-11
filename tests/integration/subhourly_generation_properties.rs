use std::fs;
use std::path::PathBuf;

fn read(path: &str) -> String {
    fs::read_to_string(PathBuf::from(env!("CARGO_MANIFEST_DIR")).join(path))
        .expect("source must be readable")
}

#[test]
fn five_minute_projection_names_closure_and_source_completeness_guards() {
    let source =
        read("crates/openwepp-hillslope-orchestrator/src/direct_runtime/subhourly_generation.rs");
    for required in [
        "WAT5_INTERVALS_PER_HOUR",
        "WAT5_INTERVALS_PER_DAY",
        "positive additional supply lacks 300-second timing",
        "positive authoritative WB14 hour has zero raw support",
        "hourly_closure_residual_m",
        "hourly_zero_order_hold",
        "first_active_subinterval",
        "last_active_subinterval",
        "depression_storage_retention_depth_m",
    ] {
        assert!(
            source.contains(required),
            "missing property guard marker: {required}"
        );
    }
    assert!(
        !source.contains("power_exponent: Some"),
        "rejected erosion exponent must not be populated"
    );
}

#[test]
fn production_output_does_not_modify_hbp_or_erosion_authority() {
    let output = read("crates/openwepp-hillslope-output/src/hillslope_wat_subhourly.rs");
    assert!(output.contains("HILLSLOPE_WAT_SUBHOURLY_SCHEMA_ID"));
    assert!(output.contains("openwepp-hillslope-wat-subhourly-v2.0"));
    assert!(output.contains("validate_rows"));
    assert!(!output.contains("Hbp"));
    assert!(!output.contains("sediment"));
}
