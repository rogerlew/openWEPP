use std::fs;

#[test]
fn mofe04_addenda_are_present_in_required_contracts() {
    let repo_root = env!("CARGO_MANIFEST_DIR");

    let watbal_contract = fs::read_to_string(format!(
        "{repo_root}/docs/specifications/science-contracts/contracts/SC-WATBAL-001.md"
    ))
    .expect("SC-WATBAL-001 should be readable");
    assert!(
        watbal_contract.contains("## MOFE04 Multi-OFE WB13/WAT Publication Policy Addendum"),
        "SC-WATBAL-001 must include MOFE04 WB13/WAT publication-policy authority addendum"
    );
    assert!(
        watbal_contract.contains("publication_ofe_policy")
            && watbal_contract.contains("sum-ofe-geometry-area")
            && watbal_contract.contains("canonical row id"),
        "SC-WATBAL-001 must encode explicit MOFE04 publication-policy/provenance semantics"
    );

    let system_contract = fs::read_to_string(format!(
        "{repo_root}/docs/specifications/science-contracts/contracts/SC-SYSTEM-001.md"
    ))
    .expect("SC-SYSTEM-001 should be readable");
    assert!(
        system_contract
            .contains("## MOFE04 Multi-OFE WB13/WAT Publication Boundary-Carry Addendum"),
        "SC-SYSTEM-001 must include MOFE04 system-boundary carry authority addendum"
    );
    assert!(
        system_contract.contains("contributor_ofe_count")
            && system_contract.contains("OFE = 1")
            && system_contract.contains("fail closed"),
        "SC-SYSTEM-001 must encode MOFE04 boundary-carry and fail-closed interpretation semantics"
    );
}
