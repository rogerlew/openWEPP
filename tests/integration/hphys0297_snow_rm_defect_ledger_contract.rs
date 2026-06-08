use std::fs;

const SC_SNOWFREEZE: &str = "docs/specifications/science-contracts/contracts/SC-SNOWFREEZE-001.md";
const SC_RUNOFFPART: &str = "docs/specifications/science-contracts/contracts/SC-RUNOFFPART-001.md";
const SC_WATBAL: &str = "docs/specifications/science-contracts/contracts/SC-WATBAL-001.md";
const PACKAGE: &str = "docs/work-packages/20260605-hphys0297-snow-rm-defect-ledger-reconstruction-closure-001/package.md";
const KERNEL_HELPER_SOURCES: [&str; 2] = [
    "crates/openwepp-hillslope-orchestrator/src/hydrology/03_kernel_support_00_support_helpers.rs",
    "crates/openwepp-hillslope-orchestrator/src/hydrology/support_helpers_mod/runoff_reconciliation.rs",
];

#[test]
fn hphys0297_contracts_require_defect_ledger_reconstruction() {
    let snow = fs::read_to_string(SC_SNOWFREEZE).expect("snow contract should be readable");
    let runoff = fs::read_to_string(SC_RUNOFFPART).expect("runoff contract should be readable");
    let watbal = fs::read_to_string(SC_WATBAL).expect("watbal contract should be readable");

    assert!(
        snow.contains("INV-SNOWFREEZE-028")
            && snow.contains("defect-ledger reconstruction invariant")
            && snow.contains("/workdir/wepp-forest_260430_baseline/src/winter.for:434-448")
            && snow.contains("1 - ngtvML/pstvML")
            && snow.contains("03_kernel_support_00_support_helpers.rs:4231-4276")
            && snow.contains("reconstruction residual to a named tolerance")
            && snow.contains(
                "Material negative raw melt without reconstruction closure remains `UNRESOLVED`"
            ),
        "SC-SNOWFREEZE must encode HPHYS0297 per-window reconstruction authority"
    );
    assert!(
        runoff.contains("INV-RUNOFFPART-025")
            && runoff.contains("runoff-consumer defect-ledger invariant")
            && runoff.contains("Closed runoff identity cannot convert an unresolved snow producer residual into acceptance"),
        "SC-RUNOFFPART must prohibit runoff/storage compensation without the snow/RM reconstruction verdict"
    );
    assert!(
        watbal.contains("INV-WATBAL-072")
            && watbal.contains("defect-ledger water-balance invariant")
            && watbal.contains("reconstructed `/workdir/wepp-forest_260430_baseline` branch `RM`")
            && watbal.contains(
                "Rows with reconstruction residual outside tolerance remain failing/owned `HOLD`"
            ),
        "SC-WATBAL must keep failed reconstructions in semantic accounting"
    );
}

#[test]
fn hphys0297_package_is_defect_ledger_not_acceptance_bucket() {
    let package = fs::read_to_string(PACKAGE).expect("package should be readable");

    assert!(
        package.contains("auditable")
            && package.contains("defect ledger")
            && package.contains("LEGACY-DEFECTIVE")
            && package.contains("OPENWEPP-DEFECTIVE")
            && package.contains("UNRESOLVED")
            && package.contains("/workdir/wepp-forest_260430_baseline")
            && package.contains(
                "Do not reproduce the `/workdir/wepp-forest_260430_baseline` negative-melt"
            ),
        "HPHYS0297 package must frame the work as a defect ledger with explicit baseline path"
    );
}

#[test]
fn hphys0297_preserves_snowsci_single_source_openwepp_negative_melt_lineage() {
    let helpers = KERNEL_HELPER_SOURCES
        .iter()
        .map(|path| {
            fs::read_to_string(path)
                .unwrap_or_else(|_| panic!("kernel helper source {path} should be readable"))
        })
        .collect::<String>();

    assert!(
        helpers.contains("fn redistribute_daily_signed_snowmelt")
            && helpers.contains("routed_melt_total_m: positive_melt_total_m")
            && helpers.contains("snowpack_state_loss_m: positive_melt_total_m")
            && helpers.contains("SIMIMPL29_SNOWPACK_STATE_LOSS_OVERDRAW_TOLERANCE_M"),
        "HPHYS0297 must compare against corrected SNOWSCI-S1 OpenWEPP negative-melt lineage, not replace it"
    );
}
