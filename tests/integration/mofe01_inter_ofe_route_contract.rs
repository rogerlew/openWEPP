use std::fs;

#[test]
fn mofe01_mb_inter_ofe_route_contract_authority_is_present() {
    let repo_root = env!("CARGO_MANIFEST_DIR");

    let runoffpart = fs::read_to_string(format!(
        "{repo_root}/docs/specifications/science-contracts/contracts/SC-RUNOFFPART-001.md"
    ))
    .expect("SC-RUNOFFPART-001 should be readable");
    assert!(
        runoffpart.contains("INV-RUNOFFPART-028")
            && runoffpart.contains("MOFE01 M-B Inter-OFE Runoff/Lateral Transfer Addendum")
            && runoffpart.contains("runon_input = UpStrmQ + SubRIn")
            && runoffpart.contains("stale pre-seeded aggregate carry must be purged"),
        "SC-RUNOFFPART-001 must carry MOFE01 M-B component-preserving transfer authority"
    );

    let watbal = fs::read_to_string(format!(
        "{repo_root}/docs/specifications/science-contracts/contracts/SC-WATBAL-001.md"
    ))
    .expect("SC-WATBAL-001 should be readable");
    assert!(
        watbal.contains("INV-WATBAL-096")
            && watbal.contains("MOFE01 M-B Inter-OFE Conservation Addendum")
            && watbal.contains("Per-element closure")
            && watbal.contains("Transfer closure")
            && watbal.contains("Hillslope-total closure")
            && watbal.contains("stale aggregate carry must be purged"),
        "SC-WATBAL-001 must carry MOFE01 M-B conservation identities"
    );

    let index = fs::read_to_string(format!(
        "{repo_root}/docs/specifications/science-contracts/index.md"
    ))
    .expect("science contract index should be readable");
    assert!(
        index.contains("SC-RUNOFFPART-001") && index.contains("2026-06-12"),
        "science contract index must expose the M-B review date"
    );
}
