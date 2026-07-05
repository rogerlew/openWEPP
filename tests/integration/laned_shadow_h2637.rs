//! Lane D activation increment — the REAL-H2637 executed vector
//! (`SC-OFEROUTE-001#INV-OFEROUTE-012`): the opt-in seam shadow routes
//! the subsurface-dominated MOFE stress hillslope (19 OFEs, steep-wet
//! forest class) through the actual `ofe_routing` cascade from LIVE
//! published surfaces, while every protected output stays
//! byte-identical (`INV-OFEROUTE-010`). The shadow's conservation
//! figure is a DIAGNOSTIC bound (the GAP-OFEROUTE-005
//! resolution-sensitivity class), NOT a physics acceptance; the
//! supply-reconstruction law and byte-identity are hard.

use std::fs;
use std::path::{Path, PathBuf};

use openwepp_runner::{HillslopeRunRequest, SidecarPolicy, execute_hillslope_run};

fn fixture_dir() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("tests/fixtures/laned_shadow_h2637")
}

fn copy_fixture_to_temp(tag: &str) -> PathBuf {
    let destination =
        std::env::temp_dir().join(format!("laned_shadow_h2637_{tag}_{}", std::process::id()));
    if destination.exists() {
        fs::remove_dir_all(&destination).expect("stale run dir removable");
    }
    fs::create_dir_all(&destination).expect("run dir");
    for entry in fs::read_dir(fixture_dir()).expect("fixture dir") {
        let entry = entry.expect("fixture entry");
        fs::copy(entry.path(), destination.join(entry.file_name())).expect("fixture copy");
    }
    destination
}

fn run_h2637(tag: &str, shadow: bool) -> (PathBuf, serde_json::Value, Vec<u8>, Vec<u8>) {
    let run_dir = copy_fixture_to_temp(tag);
    let output_dir = run_dir.join("output");
    let manifest_path = run_dir.join("manifest.json");
    // nextest runs each test in its own process, and both runs execute
    // serially inside this one test — the env mutation cannot leak.
    if shadow {
        // SAFETY: single-threaded test setup before any runner threads.
        unsafe { std::env::set_var("OPENWEPP_LANED_SHADOW", "1") };
    } else {
        // SAFETY: as above.
        unsafe { std::env::remove_var("OPENWEPP_LANED_SHADOW") };
    }
    let report = execute_hillslope_run(
        &HillslopeRunRequest {
            run_dir: run_dir.clone(),
            run_file: PathBuf::from("p2637.run.toml"),
            output_dir,
            sidecar_policy: SidecarPolicy::Compat,
            legacy_sidecar_discovery: false,
            manifest_path: Some(manifest_path.clone()),
        },
        &["openwepp-cli-hill".to_string()],
    )
    .expect("H2637 must run end-to-end");
    let manifest: serde_json::Value =
        serde_json::from_str(&fs::read_to_string(&manifest_path).expect("manifest readable"))
            .expect("manifest parses");
    let pass_bytes = fs::read(&report.output_pass).expect("HBP bytes");
    let parquet_bytes = fs::read(report.output_pass.with_file_name("H2637.pass.parquet"))
        .expect("pass parquet bytes");
    (run_dir, manifest, pass_bytes, parquet_bytes)
}

fn find_key<'a>(value: &'a serde_json::Value, key: &str) -> Option<&'a serde_json::Value> {
    match value {
        serde_json::Value::Object(map) => map
            .get(key)
            .or_else(|| map.values().find_map(|nested| find_key(nested, key))),
        serde_json::Value::Array(items) => items.iter().find_map(|item| find_key(item, key)),
        _ => None,
    }
}

#[test]
fn h2637_executed_vector_shadow_on_off() {
    let (_dir_off, manifest_off, pass_off, parquet_off) = run_h2637("off", false);
    assert!(
        find_key(&manifest_off, "laned_shadow").is_none(),
        "no shadow keys when the shadow is off"
    );

    let (_dir_on, manifest_on, pass_on, parquet_on) = run_h2637("on", true);
    // SAFETY: restore the process env for any later in-process work.
    unsafe { std::env::remove_var("OPENWEPP_LANED_SHADOW") };

    // INV-OFEROUTE-010: protected outputs byte-identical with the
    // shadow on — the routed subsystem produces diagnostics only, and
    // the latqcc/baseflow export path is untouched (the D3 bypass).
    assert_eq!(pass_off, pass_on, "HBP must be byte-identical");
    assert_eq!(
        parquet_off, parquet_on,
        "pass parquet must be byte-identical"
    );

    let shadow =
        find_key(&manifest_on, "laned_shadow").expect("shadow manifest block present when enabled");
    let days_seen = shadow["days_seen"].as_u64().expect("days_seen");
    let days_routed = shadow["days_routed"].as_u64().expect("days_routed");
    assert_eq!(days_seen, 731, "two climate years");
    assert!(
        days_routed > 300,
        "the wet H2637 record must route hundreds of event days (got {days_routed})"
    );

    // HARD LAW: the weights x qofe reconstruction resums the published
    // supply exactly (the seam consumes the two D1 limbs faithfully).
    let supply_rel = shadow["max_supply_reconstruction_rel"]
        .as_f64()
        .expect("supply rel");
    assert!(
        supply_rel < 1.0e-9,
        "supply reconstruction must be exact (got {supply_rel})"
    );

    // DIAGNOSTIC bound (GAP-OFEROUTE-005 resolution-sensitivity class,
    // recorded sweep: 6.0% at the shipped constants): the aggregate
    // router conservation residual stays under 15%.
    let aggregate_rel = shadow["aggregate_router_conservation_rel"]
        .as_f64()
        .expect("aggregate rel");
    assert!(
        aggregate_rel < 0.15,
        "aggregate router conservation diagnostic (got {aggregate_rel})"
    );

    // The routed toe receives the dominant share of the injected
    // surface supply over the run (day-window tail storage is small).
    let source_m3 = shadow["total_source_m3"].as_f64().expect("source");
    let outlet_m3 = shadow["total_routed_outlet_m3"].as_f64().expect("outlet");
    assert!(source_m3 > 0.0);
    assert!(
        outlet_m3 / source_m3 > 0.9,
        "toe delivery fraction (got {})",
        outlet_m3 / source_m3
    );

    // The melt-limb coverage finding stays visible: a small class of
    // runoff days carries no D1 hourly shape (uniform fallback).
    let uniform = shadow["days_uniform_shape"].as_u64().expect("uniform");
    assert!(
        uniform > 0 && uniform < 50,
        "the lump-only day class is present and small (got {uniform})"
    );
}
