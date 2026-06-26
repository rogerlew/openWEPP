use std::fs;
use std::path::{Path, PathBuf};

use openwepp_runner::{PhysicsBulkRequest, PhysicsBulkVariant, run_physics_bulk_snowbench};

#[test]
fn physics_bulk_snowbench_runs_offline_for_snotel_fixture() {
    let output_dir = PathBuf::from("target/snowdensity03_contract/site1");
    if output_dir.exists() {
        fs::remove_dir_all(&output_dir).expect("test output cleanup should succeed");
    }

    let report = run_physics_bulk_snowbench(&PhysicsBulkRequest {
        run_dir: PathBuf::from("tests/fixtures/snotel_observed/snotel_mica_creek_st_joe_id"),
        run_file: None,
        output_dir: output_dir.clone(),
        variant: PhysicsBulkVariant::CandidateV1,
    })
    .expect("offline physics_bulk snowbench should run");

    assert_eq!(report.schema, "snowdensity03-physics-bulk-snowbench-v1");
    assert_eq!(report.model_id, "physics_bulk_candidate_v1");
    assert_eq!(report.variant, "candidate_v1");
    assert!(report.no_site_constants);
    assert_eq!(
        report.runtime_coupling,
        "none; offline snowbench candidate only"
    );
    assert!(report.day_count > 365);
    assert!(report.positive_snow_hours > 0);
    assert!(report.summary.max_abs_mass_balance_residual_kg_m2 < 1.0e-8);
    assert!(report.summary.max_abs_cold_content_residual_j_m2 < 1.0e-5);

    let csv = fs::read_to_string(output_dir.join("physics_bulk_snow.csv"))
        .expect("physics_bulk daily CSV should exist");
    assert!(csv.contains("date,snow_water_m,snow_depth_m,snow_density_kg_m3"));
    assert!(output_dir.join("physics_bulk_summary.json").is_file());
    assert!(output_dir.join("physics_bulk_summary.md").is_file());
}

#[test]
fn physics_bulk_runtime_mentions_are_confined_to_authorized_opt_in_surfaces() {
    let allowed = [
        "crates/openwepp-hillslope-orchestrator/src/hydrology/09_snow_density.rs",
        "crates/openwepp-runner/src/hillslope/snowbench_coe_density.rs",
        "crates/openwepp-runner/src/hillslope/snowbench_physics_bulk.rs",
        "crates/openwepp-runner/src/hillslope/mod.rs",
        "crates/openwepp-runner/src/lib.rs",
        "crates/openwepp-runner/src/hillslope/direct_publication/day_input_and_helpers/00_builders_and_authority.rs",
        "crates/openwepp-runner/src/bin/openwepp-snowbench.rs",
        "tests/integration/snowdensity02_contract_adr_guard.rs",
        "tests/integration/snowdensity03_physics_bulk_offline_contract.rs",
        "tests/integration/snowdensity06_density_compaction.rs",
        "tests/integration/snowdensity06b_coe_bound_density_replay.rs",
        "tests/integration/snowdensity07_runtime_opt_in.rs",
        "tests/integration/snowdensity08_gate_rerun.rs",
        "tests/integration/snowdensity09_coupled_wat_rerun.rs",
        "tools/snowfreeze_observed/coe_bound_density_adjudication.py",
        "tools/snowfreeze_observed/physics_bulk_adjudication.py",
        "tools/snowfreeze_observed/physics_bulk_snotel_profile.py",
        "tools/snowfreeze_observed/snowdensity08_gate_rerun.py",
        "tools/snowfreeze_observed/snowdensity09_coupled_wat_rerun.py",
    ];
    let contract =
        fs::read_to_string("docs/specifications/science-contracts/contracts/SC-SNOWFREEZE-001.md")
            .expect("SC-SNOWFREEZE-001 should be readable");
    assert!(contract.contains("INV-SNOWFREEZE-060"));
    assert!(contract.contains("INV-SNOWFREEZE-061"));
    assert!(contract.contains("INV-SNOWFREEZE-062"));
    assert!(contract.contains("physics_bulk_density_compaction_v1"));

    let mut unexpected = Vec::new();
    collect_physics_bulk_hits(Path::new("crates"), &allowed, &mut unexpected);
    collect_physics_bulk_hits(Path::new("tests/integration"), &allowed, &mut unexpected);
    collect_physics_bulk_hits(
        Path::new("tools/snowfreeze_observed"),
        &allowed,
        &mut unexpected,
    );

    assert!(
        unexpected.is_empty(),
        "physics_bulk must stay confined to diagnostic or authorized typed opt-in surfaces: {unexpected:?}"
    );
}

fn collect_physics_bulk_hits(path: &Path, allowed: &[&str], unexpected: &mut Vec<String>) {
    if path.is_dir() {
        for entry in fs::read_dir(path).expect("directory should be readable") {
            let entry = entry.expect("directory entry should be readable");
            collect_physics_bulk_hits(&entry.path(), allowed, unexpected);
        }
        return;
    }
    if path.extension().and_then(|value| value.to_str()) != Some("rs")
        && path.extension().and_then(|value| value.to_str()) != Some("py")
    {
        return;
    }
    let normalized = path.to_string_lossy().replace('\\', "/");
    let text = fs::read_to_string(path).expect("source file should be readable");
    if text.contains("physics_bulk")
        && !allowed
            .iter()
            .any(|allowed_path| normalized.ends_with(allowed_path))
    {
        unexpected.push(normalized);
    }
}
