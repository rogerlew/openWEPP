use std::process::Command;

use sha2::{Digest, Sha256};

const PACKAGE: &str = "docs/work-packages/20260818-c3-nighttime-ci-hold-lift-001/artifacts";

fn sha256(bytes: &[u8]) -> String {
    format!("{:x}", Sha256::digest(bytes))
}

#[test]
fn definitions_bind_exact_v10_and_lse_v2_authority() {
    let vegetation_contract = include_bytes!(
        "../../docs/specifications/science-contracts/contracts/SC-VEGETATION-001.md"
    );
    let lse_contract = include_bytes!(
        "../../docs/specifications/science-contracts/contracts/SC-LANDSURFACEENERGY-001.md"
    );
    let calculator = include_bytes!(concat!(
        "../../",
        "docs/work-packages/20260818-c3-nighttime-ci-hold-lift-001/artifacts/reference_nighttime_ci.py"
    ));
    let vectors = include_bytes!(concat!(
        "../../",
        "docs/work-packages/20260818-c3-nighttime-ci-hold-lift-001/artifacts/nighttime-ci-vectors.json"
    ));
    let closure = include_bytes!(concat!(
        "../../",
        "docs/work-packages/20260818-c3-nighttime-ci-hold-lift-001/artifacts/coupled-oracle/potential-as-final-evaluation.json"
    ));
    let v10: serde_json::Value = serde_json::from_slice(include_bytes!(
        "../../crates/openwepp-vegetation/model-registry/openwepp_c3_woody_v10_definition.json"
    ))
    .expect("V10 definition JSON");
    let lse_v2: serde_json::Value = serde_json::from_slice(include_bytes!(concat!(
        "../../",
        "docs/work-packages/20260818-c3-nighttime-ci-hold-lift-001/artifacts/openwepp_snow_free_lse_v2_definition.json"
    )))
    .expect("LSE-V2 definition JSON");
    assert_eq!(
        v10["canonical_contract_sha256"],
        sha256(vegetation_contract)
    );
    assert_eq!(v10["oracle"]["calculator_sha256"], sha256(calculator));
    assert_eq!(v10["oracle"]["vectors_sha256"], sha256(vectors));
    assert_eq!(v10["closure_evidence_sha256"], sha256(closure));
    assert_eq!(lse_v2["canonical_contract_sha256"], sha256(lse_contract));
    assert_eq!(lse_v2["closure_evidence_sha256"], sha256(closure));
    assert_eq!(lse_v2["coupled_vegetation_model"], "OPENWEPP_C3_WOODY_V10");
}

#[test]
fn independent_nighttime_calculator_regenerates_frozen_vectors() {
    let output = Command::new(".venv/bin/python")
        .arg(format!("{PACKAGE}/reference_nighttime_ci.py"))
        .output()
        .expect("execute independent V10 calculator");
    assert!(output.status.success());
    assert_eq!(
        output.stdout,
        std::fs::read(format!("{PACKAGE}/nighttime-ci-vectors.json")).expect("frozen V10 vectors")
    );
    let value: serde_json::Value = serde_json::from_slice(&output.stdout).expect("V10 vector JSON");
    let cases = value["cases"].as_array().expect("V10 cases");
    assert_eq!(cases[0]["result"]["branch"], "exact_zero_analytic");
    assert_eq!(cases[1]["result"]["branch"], "exact_zero_analytic");
    assert_eq!(cases[0]["result"]["ci"], cases[1]["result"]["ci"]);
}

#[test]
fn fixed_final_closure_evidence_has_all_29_accepted_residuals() {
    let value: serde_json::Value = serde_json::from_slice(include_bytes!(concat!(
        "../../",
        "docs/work-packages/20260818-c3-nighttime-ci-hold-lift-001/artifacts/coupled-oracle/potential-as-final-evaluation.json"
    )))
    .expect("closure evidence JSON");
    let residuals = value["residuals"].as_array().expect("residual ledger");
    assert_eq!(residuals.len(), 29);
    assert!(residuals.iter().all(|row| {
        row["normalized"]["decimal"]
            .as_str()
            .and_then(|value| value.parse::<f64>().ok())
            .is_some_and(|normalized| normalized.abs() <= 1.0)
    }));
    assert_eq!(value["evaluated_pass"], "final_fixed_cap");
    assert_eq!(value["rebuild_source"], "immutable_phase_beginning");
}
