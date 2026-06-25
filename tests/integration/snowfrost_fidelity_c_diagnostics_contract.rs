use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;

use serde_json::Value;

const TOOL_SOURCE: &str = include_str!("../../tools/snowfreeze_observed/frozen_k_diagnostics.py");
const PACKAGE: &str = include_str!(
    "../../docs/work-packages/20260625-snowfrost-fidelity-c-sfcc-frozen-k-diagnostics-001/package.md"
);

#[test]
fn snowfrost_c_cli_emits_diagnostic_only_sfcc_frozen_k_payload() {
    let repo = repo_root();
    let output_dir = repo.join("target/snowfrost_fidelity_c_contract");
    fs::create_dir_all(&output_dir).expect("diagnostic output dir should be creatable");
    let output_json = output_dir.join("diagnostics.json");
    let output_md = output_dir.join("diagnostics.md");

    let status = Command::new(repo.join(".venv/bin/python"))
        .args([
            "tools/snowfreeze_observed/frozen_k_diagnostics.py",
            "--output-json",
            output_json
                .to_str()
                .expect("diagnostic JSON path should be UTF-8"),
            "--output-md",
            output_md
                .to_str()
                .expect("diagnostic MD path should be UTF-8"),
        ])
        .current_dir(&repo)
        .status()
        .expect("diagnostic CLI should launch");
    assert!(status.success(), "diagnostic CLI failed with {status}");

    let payload = read_json(&output_json);
    assert_eq!(
        payload.get("schema").and_then(Value::as_str),
        Some("snowfrost-fidelity-c-frozen-k-diagnostics-v1")
    );
    assert_eq!(
        payload.get("promotion_status").and_then(Value::as_str),
        Some("diagnostic_only_not_runtime_authority")
    );
    assert_eq!(
        payload.get("runtime_coupling").and_then(Value::as_str),
        Some("none")
    );
    assert_eq!(
        payload.get("qwet_authority").and_then(Value::as_str),
        Some("not_authorized")
    );
    assert!(
        fs::read_to_string(&output_md)
            .expect("diagnostic markdown should be readable")
            .contains("diagnostic comparison surfaces only"),
        "markdown output must preserve non-promotion disposition"
    );
}

#[test]
fn snowfrost_c_curves_are_bounded_monotonic_and_impedance_ordered() {
    let payload = diagnostic_payload("bounded_monotonic");
    let temperatures = payload
        .get("temperatures_c")
        .and_then(Value::as_array)
        .expect("temperature grid should be present");
    assert_eq!(temperatures.len(), 7);

    for soil in payload
        .get("soils")
        .and_then(Value::as_array)
        .expect("soils should be present")
    {
        let soil_id = soil
            .get("soil_id")
            .and_then(Value::as_str)
            .expect("soil_id should be present");
        let samples = samples_for_soil(&payload, soil_id);
        assert_eq!(samples.len(), temperatures.len());
        let mut prior_liquid = f64::INFINITY;
        let mut prior_krel = f64::INFINITY;
        for sample in samples {
            let liquid = number(&sample, "liquid_water_m3_m3");
            let krel = number(&sample, "sfcc_mualem_k_rel");
            let impeded = number(&sample, "impedance_scaled_k_rel");
            let capillary = number(&sample, "watanabe_flury_capillary_bundle_screening_k_rel");
            assert!(
                (0.0..=1.0).contains(&krel),
                "SFCC-Mualem krel out of bounds for {soil_id}: {krel}"
            );
            assert!(
                (0.0..=1.0).contains(&impeded),
                "impedance-scaled krel out of bounds for {soil_id}: {impeded}"
            );
            assert!(
                (0.0..=1.0).contains(&capillary),
                "capillary screening ratio out of bounds for {soil_id}: {capillary}"
            );
            assert!(
                impeded <= krel + 1.0e-12,
                "impedance diagnostic must not increase conductivity for {soil_id}: impeded={impeded}, base={krel}"
            );
            assert!(
                liquid <= prior_liquid + 1.0e-12,
                "liquid water must not increase as the temperature grid gets colder for {soil_id}"
            );
            assert!(
                krel <= prior_krel + 1.0e-12,
                "frozen conductivity ratio must not increase as the temperature grid gets colder for {soil_id}"
            );
            prior_liquid = liquid;
            prior_krel = krel;
        }
    }
}

#[test]
fn snowfrost_c_records_salinity_sensitivity_without_production_promotion() {
    let payload = diagnostic_payload("salinity_sensitivity");
    let sensitivity = payload
        .get("salinity_sensitivity")
        .and_then(Value::as_array)
        .expect("salinity diagnostics should be present");
    assert_eq!(sensitivity.len(), 3);
    for row in sensitivity {
        let fresh = number(row, "fresh_liquid_water_m3_m3");
        let saline = number(row, "saline_liquid_water_m3_m3");
        assert!(
            saline > fresh,
            "freezing-point depression diagnostic should increase liquid water at fixed subzero temperature"
        );
        assert!(
            row.get("model_role")
                .and_then(Value::as_str)
                .unwrap_or_default()
                .contains("not production"),
            "salinity diagnostic must retain non-production label"
        );
    }
}

#[test]
fn snowfrost_c_sources_and_crates_preserve_diagnostic_boundary() {
    for marker in [
        "diagnostic-only",
        "diagnostic_fixture_not_texture_default",
        "Kurylyk and Watanabe 2013",
        "Watanabe and Flury 2008",
        "Azmatch et al. 2012",
        "Ming et al. 2020",
        "Cheng et al. 2023",
        "Amankwah et al. 2021",
        "Devoie et al. 2022",
        "Qwet",
    ] {
        assert!(
            TOOL_SOURCE.contains(marker) || PACKAGE.contains(marker),
            "diagnostic tool/package must retain marker {marker}"
        );
    }

    let mut hits = Vec::new();
    collect_crate_marker_hits(&repo_root().join("crates"), &mut hits);
    assert!(
        hits.is_empty(),
        "production crates must not reference C diagnostic markers: {hits:?}"
    );
}

fn diagnostic_payload(label: &str) -> Value {
    let repo = repo_root();
    let output_dir = repo.join(format!("target/snowfrost_fidelity_c_contract_{label}"));
    fs::create_dir_all(&output_dir).expect("diagnostic output dir should be creatable");
    let output_json = output_dir.join("diagnostics.json");
    let output_md = output_dir.join("diagnostics.md");
    let status = Command::new(repo.join(".venv/bin/python"))
        .args([
            "tools/snowfreeze_observed/frozen_k_diagnostics.py",
            "--output-json",
            output_json
                .to_str()
                .expect("diagnostic JSON path should be UTF-8"),
            "--output-md",
            output_md
                .to_str()
                .expect("diagnostic MD path should be UTF-8"),
        ])
        .current_dir(&repo)
        .status()
        .expect("diagnostic CLI should launch");
    assert!(status.success(), "diagnostic CLI failed with {status}");
    read_json(&output_json)
}

fn samples_for_soil(payload: &Value, soil_id: &str) -> Vec<Value> {
    let mut samples: Vec<Value> = payload
        .get("samples")
        .and_then(Value::as_array)
        .expect("samples should be present")
        .iter()
        .filter(|sample| sample.get("soil_id").and_then(Value::as_str) == Some(soil_id))
        .cloned()
        .collect();
    samples.sort_by(|left, right| {
        number(right, "temperature_c")
            .partial_cmp(&number(left, "temperature_c"))
            .expect("temperatures should be finite")
    });
    samples
}

fn number(value: &Value, key: &str) -> f64 {
    value
        .get(key)
        .and_then(Value::as_f64)
        .unwrap_or_else(|| panic!("missing numeric key {key}: {value}"))
}

fn read_json(path: &Path) -> Value {
    let text = fs::read_to_string(path)
        .unwrap_or_else(|error| panic!("JSON should be readable at {}: {error}", path.display()));
    serde_json::from_str(&text)
        .unwrap_or_else(|error| panic!("JSON should parse at {}: {error}", path.display()))
}

fn collect_crate_marker_hits(path: &Path, hits: &mut Vec<String>) {
    let entries = fs::read_dir(path)
        .unwrap_or_else(|error| panic!("crate scan path should be readable: {error}"));
    for entry in entries {
        let entry = entry.expect("crate scan entry should be readable");
        let path = entry.path();
        if path.is_dir() {
            collect_crate_marker_hits(&path, hits);
            continue;
        }
        let Some(extension) = path.extension().and_then(|extension| extension.to_str()) else {
            continue;
        };
        if !matches!(extension, "rs" | "toml") {
            continue;
        }
        let text = fs::read_to_string(&path)
            .unwrap_or_else(|error| panic!("crate scan file should be readable: {error}"));
        for marker in [
            "frozen_k_diagnostics",
            "sfcc_mualem",
            "clapeyron_unfrozen",
            "diagnostic_fixture",
        ] {
            if text.contains(marker) {
                hits.push(format!("{} contains {marker}", path.display()));
            }
        }
    }
}

fn repo_root() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).to_path_buf()
}
