use std::fs;

#[test]
fn erod14_wave2_addenda_are_present_in_required_contracts() {
    let repo_root = env!("CARGO_MANIFEST_DIR");
    let required_sections = [
        (
            "docs/specifications/science-contracts/contracts/SC-SED-001.md",
            "## EROD14 Wave-2 Multi-OFE and Enrichment Runtime Addendum",
        ),
        (
            "docs/specifications/science-contracts/contracts/SC-HYDRAULICS-001.md",
            "## EROD14 Wave-2 Active Companion-Coupling Addendum",
        ),
        (
            "docs/specifications/science-contracts/contracts/SC-RUNOFFPART-001.md",
            "## EROD14 Wave-2 Active Producer-Coupling Addendum",
        ),
        (
            "docs/specifications/science-contracts/contracts/SC-WATBAL-001.md",
            "## EROD14 Wave-2 Active Producer-Coupling Addendum",
        ),
        (
            "docs/specifications/science-contracts/contracts/SC-ROUTE-001.md",
            "## EROD14 Wave-2 Active Consumer-Coupling Addendum",
        ),
        (
            "docs/specifications/science-contracts/contracts/SC-SYSTEM-001.md",
            "## EROD14 Wave-2 Active Boundary-Carry Addendum",
        ),
    ];

    for (path, section) in required_sections {
        let content =
            fs::read_to_string(format!("{repo_root}/{path}")).expect("contract should be readable");
        assert!(
            content.contains(section),
            "{path} must include EROD14 authority section: {section}"
        );
    }
}

#[test]
fn erod14_registry_updates_reference_wave2_authority() {
    let repo_root = env!("CARGO_MANIFEST_DIR");
    let index = fs::read_to_string(format!(
        "{repo_root}/docs/specifications/science-contracts/index.md"
    ))
    .expect("science-contract index should be readable");

    assert!(
        index.contains("Last updated: 2026-05-25"),
        "index must retain 2026-05-25 update stamp for EROD14 authority"
    );
    assert!(
        index.contains("HKERNEL-EROD14-WAVE2-E-001..003"),
        "index must reference EROD14 guard family continuity"
    );
}
