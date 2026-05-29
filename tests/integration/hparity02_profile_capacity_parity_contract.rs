use std::fs;
use std::path::Path;

fn repo_file(path: &str) -> String {
    let repo_root = env!("CARGO_MANIFEST_DIR");
    let full_path = Path::new(repo_root).join(path);
    fs::read_to_string(&full_path)
        .unwrap_or_else(|error| panic!("expected readable file {}: {error}", full_path.display()))
}

#[test]
fn hparity02_runner_profile_capacity_publication_uses_authoritative_lineage_symbols() {
    let runner_mod = repo_file("crates/openwepp-runner/src/hillslope/mod.rs");

    assert!(
        runner_mod.contains("wb13_profile_depth_mm")
            && runner_mod.contains("wb13_profile_porosity_cap_mm")
            && runner_mod.contains("wb13_profile_fc_store_mm")
            && runner_mod.contains("wb13_profile_wp_store_mm"),
        "WB13 publication must consume explicit profile-lineage runtime symbols when available"
    );
    assert!(
        !runner_mod.contains("profile_fc_store_mm.max(profile_wp_store_mm) + 20.0"),
        "HPARITY02 must remove synthesized placeholder ProfilePorosityCap logic"
    );
}

#[test]
fn hparity02_package_and_contract_authority_sections_exist() {
    let package = repo_file(
        "docs/work-packages/20260529-hparity02-profile-capacity-storage-lineage-closure-001/package.md",
    );
    let watbal = repo_file("docs/specifications/science-contracts/contracts/SC-WATBAL-001.md");

    assert!(
        package.contains("MEASURE-HP02-001")
            && package.contains("MEASURE-HP02-004")
            && package.contains("Mandatory Contract-First Sequence"),
        "HPARITY02 package must preserve closure measures and contract-first sequencing"
    );
    assert!(
        watbal.contains("### HPARITY02 Profile-Capacity Publication Lineage Closure"),
        "SC-WATBAL-001 must include HPARITY02 profile-capacity authority section"
    );
}
