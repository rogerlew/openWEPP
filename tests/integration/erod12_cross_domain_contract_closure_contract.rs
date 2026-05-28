use std::fs;

fn assert_gap_status(contract_text: &str, gap_id: &str, expected_status: &str) {
    let prefix = format!("| {gap_id} |");
    let row = contract_text
        .lines()
        .find(|line| line.trim_start().starts_with(&prefix))
        .unwrap_or_else(|| panic!("missing gap row for {gap_id}"));
    let columns: Vec<&str> = row.split('|').map(str::trim).collect();
    let status = columns
        .get(4)
        .copied()
        .unwrap_or_else(|| panic!("missing promotability column for {gap_id}"));

    assert_eq!(
        status, expected_status,
        "unexpected status for {gap_id}; row: {row}"
    );
}

#[test]
fn erod12_cross_domain_addenda_are_present() {
    let repo_root = env!("CARGO_MANIFEST_DIR");
    let contracts = [
        "docs/specifications/science-contracts/contracts/SC-SED-001.md",
        "docs/specifications/science-contracts/contracts/SC-HYDRAULICS-001.md",
        "docs/specifications/science-contracts/contracts/SC-ROUTE-001.md",
        "docs/specifications/science-contracts/contracts/SC-RUNOFFPART-001.md",
        "docs/specifications/science-contracts/contracts/SC-WATBAL-001.md",
        "docs/specifications/science-contracts/contracts/SC-SYSTEM-001.md",
    ];

    for relative in contracts {
        let content = fs::read_to_string(format!("{repo_root}/{relative}"))
            .expect("contract file should be readable");
        assert!(
            content.contains("## EROD12 Cross-Domain Ownership and Guard Closure Addendum"),
            "{relative} must include EROD12 cross-domain ownership/guard closure authority"
        );
    }
}

#[test]
fn erod12_wave0_cross_domain_blockers_are_closed() {
    let repo_root = env!("CARGO_MANIFEST_DIR");

    let sed = fs::read_to_string(format!(
        "{repo_root}/docs/specifications/science-contracts/contracts/SC-SED-001.md"
    ))
    .expect("SC-SED-001 must be readable");
    assert_gap_status(&sed, "GAP-SED-003", "closed");

    let hyd = fs::read_to_string(format!(
        "{repo_root}/docs/specifications/science-contracts/contracts/SC-HYDRAULICS-001.md"
    ))
    .expect("SC-HYDRAULICS-001 must be readable");
    assert_gap_status(&hyd, "GAP-HYD-003", "closed");

    let route = fs::read_to_string(format!(
        "{repo_root}/docs/specifications/science-contracts/contracts/SC-ROUTE-001.md"
    ))
    .expect("SC-ROUTE-001 must be readable");
    assert_gap_status(&route, "GAP-ROUTE-003", "closed");

    let runoffpart = fs::read_to_string(format!(
        "{repo_root}/docs/specifications/science-contracts/contracts/SC-RUNOFFPART-001.md"
    ))
    .expect("SC-RUNOFFPART-001 must be readable");
    assert_gap_status(&runoffpart, "GAP-RUNOFFPART-004", "closed");
}

#[test]
fn erod12_non_wave0_posture_remains_explicit() {
    let repo_root = env!("CARGO_MANIFEST_DIR");

    let route = fs::read_to_string(format!(
        "{repo_root}/docs/specifications/science-contracts/contracts/SC-ROUTE-001.md"
    ))
    .expect("SC-ROUTE-001 must be readable");
    assert_gap_status(&route, "GAP-ROUTE-005", "closed");

    let runoffpart = fs::read_to_string(format!(
        "{repo_root}/docs/specifications/science-contracts/contracts/SC-RUNOFFPART-001.md"
    ))
    .expect("SC-RUNOFFPART-001 must be readable");
    assert_gap_status(&runoffpart, "GAP-RUNOFFPART-003", "non-promotable");

    let watbal = fs::read_to_string(format!(
        "{repo_root}/docs/specifications/science-contracts/contracts/SC-WATBAL-001.md"
    ))
    .expect("SC-WATBAL-001 must be readable");
    assert_gap_status(&watbal, "GAP-WATBAL-002", "non-promotable");

    let system = fs::read_to_string(format!(
        "{repo_root}/docs/specifications/science-contracts/contracts/SC-SYSTEM-001.md"
    ))
    .expect("SC-SYSTEM-001 must be readable");
    assert_gap_status(&system, "GAP-SYSTEM-001", "promotable-with-risk");
}
