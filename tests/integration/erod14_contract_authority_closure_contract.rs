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

    // E.3 stage 2e: the EROD14/Wave-2 runtime arm is DELETED — every
    // addendum must remain (historical specification) AND carry the
    // deleted-historical disposition banner so the canonical authority
    // is consistent with the runtime.
    for (path, section) in required_sections {
        let content =
            fs::read_to_string(format!("{repo_root}/{path}")).expect("contract should be readable");
        assert!(
            content.contains(section),
            "{path} must retain the historical EROD14 section: {section}"
        );
        let section_start = content.find(section).expect("section located");
        let banner_window = &content[section_start..(section_start + 1200).min(content.len())];
        assert!(
            banner_window.contains("DELETED — E.3 stage 2e"),
            "{path} section {section} must carry the E.3 stage-2e deleted-historical banner"
        );
        // Codex 2e round-2: registry/index rows must agree with the body
        // banners — a binding-exposure row for the deleted arm may not
        // claim `active` status or live obligations.
        for line in content.lines() {
            let is_registry_row = line.starts_with('|')
                && (line.contains("EROD14-WAVE-2") || line.contains("EROD15-WAVE-3"));
            if is_registry_row {
                assert!(
                    !line.contains("| `active` |"),
                    "{path} registry row for the deleted EROD14/EROD15 arm must not be `active`: {line}"
                );
                assert!(
                    line.contains("deleted-historical"),
                    "{path} registry row for the deleted EROD14/EROD15 arm must carry the deleted-historical status: {line}"
                );
            }
        }
    }

    let sed_contract = fs::read_to_string(format!(
        "{repo_root}/docs/specifications/science-contracts/contracts/SC-SED-001.md"
    ))
    .expect("SC-SED-001 should be readable");
    assert!(
        sed_contract.contains("Production hillslope runner activation policy is explicit"),
        "SC-SED-001 must encode MOFE03 production activation policy authority"
    );

    let system_contract = fs::read_to_string(format!(
        "{repo_root}/docs/specifications/science-contracts/contracts/SC-SYSTEM-001.md"
    ))
    .expect("SC-SYSTEM-001 should be readable");
    assert!(
        system_contract.contains("must deterministically carry")
            && system_contract.contains("erod14_wave2_enabled"),
        "SC-SYSTEM-001 must encode MOFE03 boundary-carry authority for Wave-2 activation surfaces"
    );
}

#[test]
fn erod14_contract_changelog_references_wave2_authority() {
    let repo_root = env!("CARGO_MANIFEST_DIR");
    let contract = fs::read_to_string(format!(
        "{repo_root}/docs/specifications/science-contracts/contracts/SC-SED-001.md"
    ))
    .expect("SC-SED-001 should be readable");

    assert!(
        contract.contains("HKERNEL-EROD14-WAVE2-E-001")
            && contract.contains("HKERNEL-EROD14-WAVE2-E-002")
            && contract.contains("HKERNEL-EROD14-WAVE2-E-003"),
        "SC-SED-001 must reference EROD14 guard family continuity"
    );
}
