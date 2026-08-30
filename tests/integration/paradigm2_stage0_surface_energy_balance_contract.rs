use std::fs;
use std::path::{Path, PathBuf};

const PACKAGE: &str =
    "docs/work-packages/20260628-paradigm-2-stage-0-surface-energy-balance-001/package.md";
const PROVENANCE: &str = "docs/work-packages/20260628-paradigm-2-stage-0-surface-energy-balance-001/artifacts/clean-room-provenance.md";
const NO_WIRING: &str = "docs/work-packages/20260628-paradigm-2-stage-0-surface-energy-balance-001/artifacts/no-production-wiring-scan.md";
const STAGE3_PACKAGE: &str = "docs/work-packages/20260629-paradigm-2-stage-3-liquid-routing-meltwater-temperature-001/package.md";
const EB03_PACKAGE: &str = "docs/work-packages/20260730-snow-surface-eb-03-shared-thermal-energy-composition-001/package.md";
const SNOWFREEZE_CONTRACT: &str =
    "docs/specifications/science-contracts/contracts/SC-SNOWFREEZE-001.md";
const METEOROLOGY_LIB: &str = "crates/openwepp-meteorology/src/lib.rs";
const SURFACE_ENERGY: &str = "crates/openwepp-meteorology/src/surface_energy.rs";

const RUNTIME_SOURCE_DIRS: &[&str] = &[
    "crates/openwepp-hillslope-orchestrator/src",
    "crates/openwepp-runner/src",
    "crates/openwepp-watershed-orchestrator/src",
    "crates/openwepp-climate-runtime-adapter/src",
    "crates/openwepp-legacy-bridge/src",
];

const SURFACE_ENERGY_RUNTIME_TOKENS: &[&str] = &[
    // Bind the Stage-0 meteorology module specifically. The separately admitted
    // `openwepp_land_surface_energy` crate shares an English name but is not a
    // wiring path to these retained meteorology primitives.
    "openwepp_meteorology::surface_energy",
    "net_all_wave_radiation",
    "turbulent_fluxes_monin_obukhov",
    "conductive_heat_flux",
    "precipitation_advected_heat_flux",
];

const STAGE3_ALLOWED_RUNTIME_FILES: &[&str] = &[
    "crates/openwepp-hillslope-orchestrator/src/hydrology/03_kernel_support_00_support_helpers.rs",
    "crates/openwepp-hillslope-orchestrator/src/hydrology/support_helpers_mod/infiltration_reconciliation.rs",
    "crates/openwepp-hillslope-orchestrator/src/hydrology/support_helpers_mod/runoff_reconciliation.rs",
    "crates/openwepp-hillslope-orchestrator/src/hydrology/support_helpers_mod/runoff_reconciliation/stage3_solver.rs",
    "crates/openwepp-hillslope-orchestrator/src/hydrology/support_helpers_mod/runoff_reconciliation/stage3_solver/evaluation.rs",
    "crates/openwepp-hillslope-orchestrator/src/hydrology/support_helpers_mod/runoff_reconciliation/stage3_solver/stage3_evaluation_validation_tests/persistent_tests.rs",
    "crates/openwepp-hillslope-orchestrator/src/snow_stage3_open_boundary.rs",
    "crates/openwepp-hillslope-orchestrator/src/v11_covered/carrier_phase.rs",
    "crates/openwepp-hillslope-orchestrator/src/v11_covered/execution.rs",
    "crates/openwepp-hillslope-orchestrator/src/v11_covered/physical_outcome_ledger.rs",
    "crates/openwepp-hillslope-orchestrator/src/v9_real_consumer_shadow_wb14_tests.rs",
    "crates/openwepp-runner/src/hillslope/direct_publication/day_input_and_helpers/00a_snow_frost_authority_impl.rs",
    "crates/openwepp-runner/src/hillslope/direct_publication/day_input_and_helpers/00c_day_input_builder_impl.rs",
    "crates/openwepp-runner/src/hillslope/direct_publication/day_input_and_helpers/00h_snow_stage3_evaluation_trace.rs",
    "crates/openwepp-runner/src/hillslope/snowbench_coe_melt.rs",
    "crates/openwepp-runner/src/hillslope/tests03/stage3_evaluation_publication_parity.rs",
];

fn repo_path(relative_path: &str) -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join(relative_path)
}

fn repo_text(relative_path: &str) -> String {
    let path = repo_path(relative_path);
    fs::read_to_string(&path)
        .unwrap_or_else(|err| panic!("failed to read {}: {err}", path.display()))
}

fn collect_rust_files(root: &Path, files: &mut Vec<PathBuf>) {
    for entry in fs::read_dir(root)
        .unwrap_or_else(|err| panic!("failed to read source directory {}: {err}", root.display()))
    {
        let entry = entry.expect("directory entry is readable");
        let path = entry.path();
        if path.is_dir() {
            collect_rust_files(&path, files);
        } else if path.extension().is_some_and(|extension| extension == "rs") {
            files.push(path);
        }
    }
}

fn is_test_only_rust_source(relative_path: &str) -> bool {
    relative_path.ends_with("_tests.rs")
        || relative_path
            .split('/')
            .any(|component| component == "tests")
}

#[test]
fn stage0_package_declares_pure_surface_energy_boundary() {
    let package = repo_text(PACKAGE);

    for marker in [
        "PARADIGM-2 STAGE 0",
        "surface-energy-balance primitives",
        "pure crate",
        "no runtime wiring",
        "no production default/schema/fixture/frost change",
        "No SC amendment",
    ] {
        assert!(
            package.contains(marker),
            "Stage 0 package missing boundary marker: {marker}"
        );
    }
}

#[test]
fn clean_room_provenance_records_cc0_and_equation_sources() {
    let provenance = repo_text(PROVENANCE);

    for marker in [
        "bf8b41c71e3e54ae654ae04005ddf72566c47ee6",
        "license=\"CC0 1.0\"",
        "_net_rad.c",
        "hle1.c",
        "g_snow.c",
        "g_soil.c",
        "_advec.c",
        "Marks et al. 1999",
        "no fixture fitting",
    ] {
        assert!(
            provenance.contains(marker),
            "Stage 0 provenance missing marker: {marker}"
        );
    }
}

#[test]
fn meteorology_surface_energy_module_is_public_and_complete() {
    let lib = repo_text(METEOROLOGY_LIB);
    let surface_energy = repo_text(SURFACE_ENERGY);

    assert!(
        lib.contains("pub mod surface_energy;"),
        "surface energy module must be public from openwepp-meteorology"
    );

    for marker in [
        "net_all_wave_radiation",
        "turbulent_fluxes_monin_obukhov",
        "conductive_heat_flux",
        "precipitation_advected_heat_flux",
        "mass_flux_from_latent_heat_flux",
        "surface_energy_balance",
    ] {
        assert!(
            surface_energy.contains(marker),
            "surface energy module missing primitive: {marker}"
        );
    }
}

#[test]
fn production_runtime_sources_only_wire_stage0_flux_primitives_through_stage3_opt_in() {
    let no_wiring = repo_text(NO_WIRING);
    assert!(
        no_wiring.contains("No production runtime source references"),
        "no-production-wiring artifact must record the scan result"
    );
    let stage3_package = repo_text(STAGE3_PACKAGE);
    assert!(
        stage3_package.contains("OPENWEPP_PARADIGM2_STAGE3_LIQUID_MODEL=layered_thermal_liquid_v1"),
        "Stage 3 package must bind the thermal-provider opt-in use of Stage 0 primitives"
    );
    let eb03_package = repo_text(EB03_PACKAGE);
    assert!(
        eb03_package.contains("Dilley-Unsworth atmospheric longwave")
            && eb03_package.contains("existing Stage 3 hourly carrier")
            && eb03_package.contains("absent/empty selectors remain disabled"),
        "EB-03 package must bind the only additional runtime use of Stage 0 primitives"
    );
    let snowfreeze_contract = repo_text(SNOWFREEZE_CONTRACT);
    assert!(
        snowfreeze_contract.contains("INV-SNOWFREEZE-080")
            && snowfreeze_contract.contains("Stage 0 surface-energy primitives"),
        "SC-SNOWFREEZE-001 must authorize the Stage 3 opt-in use of Stage 0 primitives"
    );

    let mut rust_files = Vec::new();
    for source_dir in RUNTIME_SOURCE_DIRS {
        collect_rust_files(&repo_path(source_dir), &mut rust_files);
    }

    let mut violations = Vec::new();
    for path in rust_files {
        let text = fs::read_to_string(&path)
            .unwrap_or_else(|err| panic!("failed to read {}: {err}", path.display()));
        let relative_path = path
            .strip_prefix(repo_path(""))
            .unwrap_or(path.as_path())
            .to_string_lossy();
        if is_test_only_rust_source(&relative_path) {
            continue;
        }
        for token in SURFACE_ENERGY_RUNTIME_TOKENS {
            if text.contains(token)
                && !STAGE3_ALLOWED_RUNTIME_FILES
                    .iter()
                    .any(|allowed| relative_path == *allowed)
            {
                violations.push(format!("{} contains {token}", path.display()));
            }
        }
    }

    assert!(
        violations.is_empty(),
        "Stage 0 surface-energy primitives must remain unwired outside the Stage 3 opt-in boundary:\n{}",
        violations.join("\n")
    );
}
