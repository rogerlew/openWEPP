const HILLSLOPE_ORCHESTRATOR_MANIFEST: &str =
    include_str!("../../crates/openwepp-hillslope-orchestrator/Cargo.toml");
const WATERSHED_ORCHESTRATOR_MANIFEST: &str =
    include_str!("../../crates/openwepp-watershed-orchestrator/Cargo.toml");
const ROOT_CRATE_LIB: &str = include_str!("../../src/lib.rs");

#[test]
fn orchestrators_declare_direct_parser_contract_dependency() {
    assert!(
        HILLSLOPE_ORCHESTRATOR_MANIFEST
            .contains("openwepp-input-contract = { path = \"../openwepp-input-contract\""),
        "hillslope orchestrator must own parser seam dependency directly in crate manifest"
    );
    assert!(
        WATERSHED_ORCHESTRATOR_MANIFEST
            .contains("openwepp-input-contract = { path = \"../openwepp-input-contract\""),
        "watershed orchestrator must own parser seam dependency directly in crate manifest"
    );
}

#[test]
fn root_crate_stays_non_reexport_aggregator() {
    assert!(
        !ROOT_CRATE_LIB.contains("pub use openwepp_"),
        "root crate must not mask integration ownership through re-export wiring"
    );
}
