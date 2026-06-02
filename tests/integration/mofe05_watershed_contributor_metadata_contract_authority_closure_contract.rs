use std::fs;

#[test]
fn mofe05_addenda_are_present_in_required_contracts() {
    let repo_root = env!("CARGO_MANIFEST_DIR");

    let system_contract = fs::read_to_string(format!(
        "{repo_root}/docs/specifications/science-contracts/contracts/SC-SYSTEM-001.md"
    ))
    .expect("SC-SYSTEM-001 should be readable");
    assert!(
        system_contract
            .contains("## MOFE05 Watershed Contributor Metadata Intake Validation Addendum"),
        "SC-SYSTEM-001 must include MOFE05 watershed contributor metadata intake addendum"
    );
    assert!(
        system_contract.contains("contributor_ofe_count` must equal contributor `hbp.nofe")
            && system_contract.contains("typed hard-fail boundary errors")
            && system_contract.contains("missing/malformed required"),
        "SC-SYSTEM-001 must encode MOFE05 contributor metadata consistency and typed fail-closed test vectors"
    );
    assert!(
        system_contract.contains("INV-SYSTEM-028")
            && system_contract.contains("mofe_hourly_carry")
            && system_contract.contains("substep_count = 24")
            && system_contract.contains("ui_SUrunf")
            && system_contract.contains("ui_LfCrf"),
        "SC-SYSTEM-001 must encode HPHYS0241 MOFE hourly carry metadata intake validation"
    );

    let watershed_runfile_contract = fs::read_to_string(format!(
        "{repo_root}/docs/contracts/openwepp-watershed-runfile-contract.md"
    ))
    .expect("openwepp-watershed-runfile-contract.md should be readable");
    assert!(
        watershed_runfile_contract.contains("manifest_file")
            && watershed_runfile_contract.contains("openwepp-hillslope-run-manifest-v1")
            && watershed_runfile_contract.contains("publication_ofe_policy")
            && watershed_runfile_contract.contains("contributor_ofe_count"),
        "watershed runfile contract must encode MOFE05 manifest-file metadata intake requirements"
    );

    let route_contract = fs::read_to_string(format!(
        "{repo_root}/docs/specifications/science-contracts/contracts/SC-ROUTE-001.md"
    ))
    .expect("SC-ROUTE-001 should be readable");
    assert!(
        route_contract.contains("INV-ROUTE-014")
            && route_contract.contains("mofe_hourly_carry")
            && route_contract.contains("baseline-wathour-24-slot-copy-forward"),
        "SC-ROUTE-001 must encode HPHYS0241 routing-continuity metadata gate"
    );
}
