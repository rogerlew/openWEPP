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
        index.contains("SC-RUNOFFPART-001") && index.contains("SC-WATBAL-001"),
        "science contract index must expose the M-B contract authority rows"
    );
}

#[test]
fn mofe01_mg_erosion_qin_boundary_contract_authority_is_present() {
    let repo_root = env!("CARGO_MANIFEST_DIR");

    let runoffpart = fs::read_to_string(format!(
        "{repo_root}/docs/specifications/science-contracts/contracts/SC-RUNOFFPART-001.md"
    ))
    .expect("SC-RUNOFFPART-001 should be readable");
    assert!(
        runoffpart.contains("INV-RUNOFFPART-030")
            && runoffpart.contains("MOFE01 M-G Erosion `qin` Producer-Boundary Addendum")
            && runoffpart.contains("water-transfer-only-mofe01-mg-sediment-coupling-follow-on")
            && runoffpart.contains(
                "cannot by themselves prove accepted sediment-coupled EROD14 downstream `qin`"
            ),
        "SC-RUNOFFPART-001 must carry M-G producer-boundary authority"
    );

    let watbal = fs::read_to_string(format!(
        "{repo_root}/docs/specifications/science-contracts/contracts/SC-WATBAL-001.md"
    ))
    .expect("SC-WATBAL-001 should be readable");
    assert!(
        watbal.contains("INV-WATBAL-099")
            && watbal.contains("MOFE01 M-G Erosion `qin` Water-Balance Boundary Addendum")
            && watbal.contains("erod14_qin_sediment_coupled = false")
            && watbal.contains("prior-OFE erosion `qout`"),
        "SC-WATBAL-001 must carry M-G water-balance boundary authority"
    );

    let sed = fs::read_to_string(format!(
        "{repo_root}/docs/specifications/science-contracts/contracts/SC-SED-001.md"
    ))
    .expect("SC-SED-001 should be readable");
    assert!(
        sed.contains("INV-SED-012")
            && sed.contains("MOFE01 M-G Downstream `qin` Handoff Boundary")
            && sed.contains("/workdir/wepp-forest_260430_baseline/src/xinflo.for:130-151")
            && sed.contains("/workdir/wepp-forest_260430_baseline/src/route.for:139-154")
            && sed.contains("Water-transfer-only `erod14_qin` seeding"),
        "SC-SED-001 must carry M-G downstream qin/sediment-handoff authority"
    );

    let system = fs::read_to_string(format!(
        "{repo_root}/docs/specifications/science-contracts/contracts/SC-SYSTEM-001.md"
    ))
    .expect("SC-SYSTEM-001 should be readable");
    assert!(
        system.contains("INV-SYSTEM-032")
            && system.contains("MOFE01 M-G EROD14 `qin` Manifest Boundary Addendum")
            && system.contains("erod14_qin_source_policy")
            && system.contains("erod14_qin_sediment_coupled"),
        "SC-SYSTEM-001 must carry M-G manifest-boundary authority"
    );

    let runner = [
        "crates/openwepp-runner/src/hillslope/00_runner_intake_and_lane_setup.rs",
        "crates/openwepp-runner/src/hillslope/05_runner_execution_and_outputs.rs",
    ]
    .iter()
    .map(|path| {
        fs::read_to_string(format!("{repo_root}/{path}"))
            .unwrap_or_else(|err| panic!("runner manifest source {path} should be readable: {err}"))
    })
    .collect::<Vec<_>>()
    .join("\n");
    assert!(
        runner.contains("EROD14_QIN_POLICY_WATER_TRANSFER_ONLY")
            && runner.contains("MOFE01-MG-W-001")
            && runner.contains("erod14_qin_source_policy")
            && runner.contains("erod14_qin_sediment_coupled"),
        "runner manifest provenance must expose M-G qin policy fields"
    );
}

#[test]
fn mofe01_mi_hillslope_total_closure_contract_authority_is_present() {
    let repo_root = env!("CARGO_MANIFEST_DIR");

    let watbal = fs::read_to_string(format!(
        "{repo_root}/docs/specifications/science-contracts/contracts/SC-WATBAL-001.md"
    ))
    .expect("SC-WATBAL-001 should be readable");
    assert!(
        watbal.contains("INV-WATBAL-100")
            && watbal.contains("MOFE01 M-I Hillslope-Total Closure Addendum")
            && watbal.contains("TOL-WATBAL-008")
            && watbal.contains("<= 1e-9 mm")
            && watbal.contains("hillslope_total_identity_max_abs_mm")
            && watbal.contains("Published WAT rows may audit the result")
            && watbal.contains("non-outlet routed surface runoff")
            && watbal.contains("surface/lateral hillslope exports")
            && watbal.contains("upstream-area/current-area")
            && watbal.contains("scheduler lifecycle may remain the single-OFE specialization")
            && watbal.contains("compute and discard an aggregate"),
        "SC-WATBAL-001 must carry M-I hillslope-total and scheduler-lifecycle closure authority"
    );
}
