use std::fs;

use openwepp_sim_contract::symbols::SymbolAliasRegistry;

#[test]
fn erod15_wave3_addenda_are_present_in_required_contracts() {
    let repo_root = env!("CARGO_MANIFEST_DIR");
    let required_sections = [
        (
            "docs/specifications/science-contracts/contracts/SC-SED-001.md",
            "## EROD15 Wave-3 HBP Routing-Boundary Export Addendum",
        ),
        (
            "docs/specifications/science-contracts/contracts/SC-ROUTE-001.md",
            "## EROD15 Wave-3 HBP Contributor-Payload Coupling Addendum",
        ),
        (
            "docs/specifications/science-contracts/contracts/SC-SYSTEM-001.md",
            "## EROD15 Wave-3 HBP Boundary-Carry Addendum",
        ),
    ];

    for (path, section) in required_sections {
        let content =
            fs::read_to_string(format!("{repo_root}/{path}")).expect("contract should be readable");
        assert!(
            content.contains(section),
            "{path} must include EROD15 authority section: {section}"
        );
    }
}

#[test]
fn erod15_contract_references_wave3_authority() {
    let repo_root = env!("CARGO_MANIFEST_DIR");
    let contract = fs::read_to_string(format!(
        "{repo_root}/docs/specifications/science-contracts/contracts/SC-SED-001.md"
    ))
    .expect("SC-SED-001 should be readable");

    assert!(
        contract.contains("total_detachment_kg")
            && contract.contains("particle_flow_fraction")
            && contract.contains("hillslope-binary-pass-format.md"),
        "SC-SED-001 must reference EROD15 Wave-3 HBP routing-boundary authority"
    );
}

#[test]
fn erod15_alias_registry_contains_hbp_payload_templates() {
    let registry = SymbolAliasRegistry::canonical_wepp_registry()
        .expect("canonical WEPP alias registry should construct");
    let detachment_aliases = registry
        .aliases_for_canonical("total_detachment_kg")
        .expect("detachment aliases should resolve");

    assert_eq!(detachment_aliases.len(), 2);
    assert!(
        detachment_aliases.contains(&"total_detachment_kg".to_string())
            && detachment_aliases.contains(&"hs{ofe}_total_detachment_kg".to_string()),
        "detachment aliases must include direct and contributor-scoped forms"
    );
    let particle_fraction_aliases = registry
        .aliases_for_canonical("particle_flow_fraction")
        .expect("particle-flow-fraction aliases should resolve");

    assert_eq!(particle_fraction_aliases.len(), 2);
    assert!(
        particle_fraction_aliases.contains(&"particle_flow_fraction_{idx4}".to_string())
            && particle_fraction_aliases
                .contains(&"hs{ofe}_particle_flow_fraction_{idx4}".to_string()),
        "particle_flow_fraction aliases must include direct and contributor-scoped forms"
    );
    let particle_diameter_aliases = registry
        .aliases_for_canonical("particle_diameter_m")
        .expect("particle-diameter aliases should resolve");

    assert_eq!(particle_diameter_aliases.len(), 2);
    assert!(
        particle_diameter_aliases.contains(&"particle_diameter_m_{idx4}".to_string())
            && particle_diameter_aliases
                .contains(&"hs{ofe}_particle_diameter_m_{idx4}".to_string()),
        "particle_diameter_m aliases must include direct and contributor-scoped forms"
    );
    assert_eq!(
        registry
            .canonical_for_boundary_alias("hs7_sediment_concentration_kg_m3_0003")
            .expect("concentration alias should resolve"),
        "sediment_concentration_kg_m3"
    );
}
