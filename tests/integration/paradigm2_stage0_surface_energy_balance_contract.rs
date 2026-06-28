use std::fs;
use std::path::{Path, PathBuf};

const PACKAGE: &str =
    "docs/work-packages/20260628-paradigm-2-stage-0-surface-energy-balance-001/package.md";
const PROVENANCE: &str = "docs/work-packages/20260628-paradigm-2-stage-0-surface-energy-balance-001/artifacts/clean-room-provenance.md";
const NO_WIRING: &str = "docs/work-packages/20260628-paradigm-2-stage-0-surface-energy-balance-001/artifacts/no-production-wiring-scan.md";
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
    "surface_energy",
    "net_all_wave_radiation",
    "turbulent_fluxes_monin_obukhov",
    "conductive_heat_flux",
    "precipitation_advected_heat_flux",
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
fn production_runtime_sources_do_not_wire_stage0_flux_primitives() {
    let no_wiring = repo_text(NO_WIRING);
    assert!(
        no_wiring.contains("No production runtime source references"),
        "no-production-wiring artifact must record the scan result"
    );

    let mut rust_files = Vec::new();
    for source_dir in RUNTIME_SOURCE_DIRS {
        collect_rust_files(&repo_path(source_dir), &mut rust_files);
    }

    let mut violations = Vec::new();
    for path in rust_files {
        let text = fs::read_to_string(&path)
            .unwrap_or_else(|err| panic!("failed to read {}: {err}", path.display()));
        for token in SURFACE_ENERGY_RUNTIME_TOKENS {
            if text.contains(token) {
                violations.push(format!("{} contains {token}", path.display()));
            }
        }
    }

    assert!(
        violations.is_empty(),
        "Stage 0 surface-energy primitives must remain unwired:\n{}",
        violations.join("\n")
    );
}
