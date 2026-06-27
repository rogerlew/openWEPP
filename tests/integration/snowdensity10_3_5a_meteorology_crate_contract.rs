use std::fs;
use std::path::Path;

const CONTRACT: &str = "docs/specifications/science-contracts/contracts/SC-SNOWFREEZE-001.md";
const ROOT_CARGO: &str = "Cargo.toml";

const UNAUTHORIZED_METEOROLOGY_DEPENDENCY_MANIFESTS: &[&str] = &[
    "crates/openwepp-climate-runtime-adapter/Cargo.toml",
    "crates/openwepp-hillslope-output/Cargo.toml",
    "crates/openwepp-watershed-output/Cargo.toml",
    "crates/openwepp-input-contract/Cargo.toml",
    "crates/openwepp-sim-contract/Cargo.toml",
    "crates/openwepp-kernel-contract/Cargo.toml",
    "crates/openwepp-topology/Cargo.toml",
    "crates/openwepp-watershed-orchestrator/Cargo.toml",
    "crates/openwepp-legacy-bridge/Cargo.toml",
    "crates/openwepp-summary-accumulator/Cargo.toml",
    "crates/openwepp-comparator-metadata/Cargo.toml",
];

fn repo_text(relative_path: &str) -> String {
    let path = Path::new(env!("CARGO_MANIFEST_DIR")).join(relative_path);
    fs::read_to_string(&path)
        .unwrap_or_else(|err| panic!("failed to read {}: {err}", path.display()))
}

#[test]
fn contract_declares_candidate_meteorology_scope_without_rst_cutover() {
    let contract = repo_text(CONTRACT);

    for marker in [
        "contract_version: 95",
        "REF-SNOWFREEZE-HARDER-POMEROY-2013",
        "hydrometeor_temperature",
        "harder_pomeroy_rain_fraction",
        "harder_pomeroy_snow_fraction",
        "INV-SNOWFREEZE-064",
        "OBL-SNOWFREEZE-P-039",
        "SNOWDENSITY-10.3.5a Harder-Pomeroy Meteorology Crate Addendum",
        "does not authorize replacement of production `RST`",
        "must not change `RST`, `stmtim`, daily/hourly WEPP",
        "Jennings/observed-phase corpus validation",
    ] {
        assert!(
            contract.contains(marker),
            "SC-SNOWFREEZE-001 missing 10.3.5a marker: {marker}"
        );
    }
}

#[test]
fn meteorology_crate_is_workspace_member_and_limited_to_authorized_followup_seam() {
    let root_cargo = repo_text(ROOT_CARGO);
    assert!(
        root_cargo.contains("\"crates/openwepp-meteorology\""),
        "new meteorology crate must be a workspace member"
    );

    for manifest in UNAUTHORIZED_METEOROLOGY_DEPENDENCY_MANIFESTS {
        let text = repo_text(manifest);
        assert!(
            !text.contains("openwepp-meteorology"),
            "{manifest} is outside the 10.3.5b authorized hourly partition/validation seam"
        );
    }

    let contract = repo_text(CONTRACT);
    for marker in [
        "INV-SNOWFREEZE-065",
        "Production crates must not depend on or call `openwepp-meteorology` until a later contract amendment",
        "production direct-runtime hourly winter partition may depend on `openwepp-meteorology` only behind an explicit package-bound selector",
    ] {
        assert!(
            contract.contains(marker),
            "SC-SNOWFREEZE-001 missing 10.3.5b dependency-boundary marker: {marker}"
        );
    }
}
