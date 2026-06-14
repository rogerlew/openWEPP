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
    assert!(
        watbal_contract.contains("INV-WATBAL-033")
            && watbal_contract.contains("ui_SUrunf")
            && watbal_contract.contains("ui_SCrunf")
            && watbal_contract.contains("ui_LfUrf")
            && watbal_contract.contains("ui_LfCrf"),
        "SC-WATBAL-001 must encode HPHYS0241 MOFE hourly carry-array publication authority"
    );
    assert!(
        watbal_contract.contains("INV-WATBAL-098")
            && watbal_contract.contains("MOFE01 M-F-REDO Per-OFE Publication Anti-Clone Addendum")
            && watbal_contract.contains("all-OFE-identical")
            && watbal_contract.contains("Q - UpStrmQ")
            && watbal_contract.contains("seeded/stale `wb12_infiltration`")
            && watbal_contract.contains("frost.runtime_frwatc_net_liquid_delta_m")
            && watbal_contract.contains("frost.runtime_watbtm_m")
            && watbal_contract.contains("frost.runtime_watpdg_m")
            && watbal_contract.contains("zero-on-zero transfer acceptance")
            && watbal_contract.contains("downstream `UpStrmQ` matches the upstream `QOFE`"),
        "SC-WATBAL-001 must encode M-F-REDO/M-F-REDO-CLONE anti-clone, local-runoff, frost-exchange, and active surface-handoff authority"
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
    assert!(
        system_contract.contains("mofe_hourly_carry")
            && system_contract.contains("baseline-wathour-24-slot-copy-forward")
            && system_contract.contains("required_arrays"),
        "SC-SYSTEM-001 must encode HPHYS0241 MOFE hourly carry manifest authority"
    );
    assert!(
        system_contract.contains("INV-SYSTEM-031")
            && system_contract.contains("MOFE01 M-F-REDO Per-OFE Publication Anti-Clone Addendum")
            && system_contract.contains("row cardinality")
            && system_contract.contains("nonzero adjacent surface handoff")
            && system_contract.contains("Q - UpStrmQ")
            && system_contract.contains("seeded/stale `wb12_infiltration`")
            && system_contract.contains("all-OFE-identical"),
        "SC-SYSTEM-001 must encode M-F-REDO/M-F-REDO-CLONE anti-clone manifest/consumer authority"
    );
}
