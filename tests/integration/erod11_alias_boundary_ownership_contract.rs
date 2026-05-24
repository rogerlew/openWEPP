use openwepp_kernel_contract::{
    BoundarySymbol, HillslopeProductionFluxSymbol, HillslopeProductionStateSymbol,
    WatershedChannelFluxField, WatershedChannelStateField, WatershedProductionFluxSymbol,
    WatershedProductionStateSymbol,
};
use openwepp_sim_contract::symbols::SymbolAliasRegistry;
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
fn erod11_wave0_hillslope_alias_projection_matches_authority() {
    assert_eq!(
        BoundarySymbol::from(HillslopeProductionFluxSymbol::Wb12RunoffQ).as_str(),
        "Q"
    );
    assert_eq!(
        BoundarySymbol::from(HillslopeProductionStateSymbol::Wb16Peakro).as_str(),
        "peakro"
    );
    assert_eq!(
        BoundarySymbol::from(HillslopeProductionStateSymbol::Wb16Watdur).as_str(),
        "watdur"
    );
    assert_eq!(
        BoundarySymbol::from(HillslopeProductionStateSymbol::Wb16MethodBranch).as_str(),
        "wb16_peak_method_branch"
    );
    assert_eq!(
        BoundarySymbol::from(HillslopeProductionStateSymbol::Wb16Tstar).as_str(),
        "wb16_tstar"
    );
    assert_eq!(
        BoundarySymbol::from(HillslopeProductionStateSymbol::Wb16Qpstar).as_str(),
        "wb16_qpstar"
    );
    assert_eq!(
        BoundarySymbol::from(HillslopeProductionStateSymbol::Wb16Vstar).as_str(),
        "wb16_vstar"
    );
}

#[test]
fn erod11_wave0_watershed_alias_projection_matches_authority() {
    let hillslope_peak =
        BoundarySymbol::from(WatershedProductionStateSymbol::HillslopeContributorPeak {
            hillslope_id: 17,
        });
    assert_eq!(hillslope_peak.as_str(), "hs17_peakro");

    let hillslope_duration = BoundarySymbol::from(
        WatershedProductionStateSymbol::HillslopeContributorDuration { hillslope_id: 17 },
    );
    assert_eq!(hillslope_duration.as_str(), "hs17_watdur");

    let channel_qpo = BoundarySymbol::from(WatershedProductionStateSymbol::ChannelNode {
        node_id: 4,
        field: WatershedChannelStateField::Qpo,
    });
    assert_eq!(channel_qpo.as_str(), "ws10_channel_4_qpo");

    let channel_durrof = BoundarySymbol::from(WatershedProductionStateSymbol::ChannelNode {
        node_id: 4,
        field: WatershedChannelStateField::Durrof,
    });
    assert_eq!(channel_durrof.as_str(), "ws10_channel_4_durrof");

    let channel_roff = BoundarySymbol::from(WatershedProductionFluxSymbol::ChannelNode {
        node_id: 4,
        field: WatershedChannelFluxField::Roff,
    });
    assert_eq!(channel_roff.as_str(), "ws10_channel_4_roff");
}

#[test]
fn erod11_peak_duration_alias_registry_entries_exist() {
    let registry = SymbolAliasRegistry::canonical_wepp_registry()
        .expect("canonical WEPP alias registry should construct");

    assert_eq!(
        registry
            .aliases_for_canonical("peakro")
            .expect("peakro aliases should resolve"),
        ["peak_runoff_rate_m3s"]
    );
    assert_eq!(
        registry
            .aliases_for_canonical("watdur")
            .expect("watdur aliases should resolve"),
        ["runoff_duration_s"]
    );
}

#[test]
fn erod11_contract_addenda_and_gap_posture_are_present() {
    let repo_root = env!("CARGO_MANIFEST_DIR");
    let contracts = [
        "docs/specifications/science-contracts/contracts/SC-SED-001.md",
        "docs/specifications/science-contracts/contracts/SC-HYDRAULICS-001.md",
        "docs/specifications/science-contracts/contracts/SC-ROUTE-001.md",
        "docs/specifications/science-contracts/contracts/SC-WATBAL-001.md",
        "docs/specifications/science-contracts/contracts/SC-RUNOFFPART-001.md",
    ];

    for relative in contracts {
        let content = fs::read_to_string(format!("{repo_root}/{relative}"))
            .expect("contract file should be readable");
        assert!(
            content.contains("## EROD11 Alias Ownership Register"),
            "{relative} must include EROD11 alias ownership authority"
        );
    }

    let sed = fs::read_to_string(format!(
        "{repo_root}/docs/specifications/science-contracts/contracts/SC-SED-001.md"
    ))
    .expect("SC-SED-001 must be readable");
    assert_gap_status(&sed, "GAP-SED-001", "closed");
    assert_gap_status(&sed, "GAP-SED-002", "closed");
    // EROD12 closed the Wave-0 cross-domain ownership blocker row.
    assert_gap_status(&sed, "GAP-SED-003", "closed");
    assert_gap_status(&sed, "GAP-SED-004", "closed");

    let hyd = fs::read_to_string(format!(
        "{repo_root}/docs/specifications/science-contracts/contracts/SC-HYDRAULICS-001.md"
    ))
    .expect("SC-HYDRAULICS-001 must be readable");
    assert_gap_status(&hyd, "GAP-HYD-001", "closed");
    assert_gap_status(&hyd, "GAP-HYD-002", "closed");
    // EROD12 closed the Wave-0 cross-domain ownership blocker row.
    assert_gap_status(&hyd, "GAP-HYD-003", "closed");
    assert_gap_status(&hyd, "GAP-HYD-004", "closed");

    let route = fs::read_to_string(format!(
        "{repo_root}/docs/specifications/science-contracts/contracts/SC-ROUTE-001.md"
    ))
    .expect("SC-ROUTE-001 must be readable");
    assert_gap_status(&route, "GAP-ROUTE-001", "closed");
    assert_gap_status(&route, "GAP-ROUTE-002", "closed");
    // EROD12 closed the Wave-0 cross-domain ownership blocker row.
    assert_gap_status(&route, "GAP-ROUTE-003", "closed");
    assert_gap_status(&route, "GAP-ROUTE-004", "closed");
    assert_gap_status(&route, "GAP-ROUTE-005", "non-promotable");

    let watbal = fs::read_to_string(format!(
        "{repo_root}/docs/specifications/science-contracts/contracts/SC-WATBAL-001.md"
    ))
    .expect("SC-WATBAL-001 must be readable");
    assert_gap_status(&watbal, "GAP-WATBAL-001", "closed");
    assert_gap_status(&watbal, "GAP-WATBAL-002", "non-promotable");
    assert_gap_status(&watbal, "GAP-WATBAL-003", "closed");
    assert_gap_status(&watbal, "GAP-WATBAL-004", "closed");

    let runoffpart = fs::read_to_string(format!(
        "{repo_root}/docs/specifications/science-contracts/contracts/SC-RUNOFFPART-001.md"
    ))
    .expect("SC-RUNOFFPART-001 must be readable");
    assert_gap_status(&runoffpart, "GAP-RUNOFFPART-001", "closed");
    assert_gap_status(&runoffpart, "GAP-RUNOFFPART-002", "closed");
    assert_gap_status(&runoffpart, "GAP-RUNOFFPART-003", "non-promotable");
    // EROD12 closed the Wave-0 cross-domain ownership blocker row.
    assert_gap_status(&runoffpart, "GAP-RUNOFFPART-004", "closed");
}
