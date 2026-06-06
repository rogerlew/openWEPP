use std::fs;
use std::path::Path;

const PACKAGE: &str =
    "docs/work-packages/20260606-hphys0318-stmtim-control-surface-instrumentation-001";

fn read(path: &str) -> String {
    fs::read_to_string(path).unwrap_or_else(|error| panic!("failed to read {path}: {error}"))
}

fn assert_contains(content: &str, token: &str) {
    assert!(content.contains(token), "missing required token: {token}");
}

#[test]
fn hphys0318_contract_authority_is_registered() {
    let climate = read("docs/specifications/science-contracts/contracts/SC-CLIMATE-001.md");
    assert_contains(&climate, "contract_version:");
    assert_contains(&climate, "INV-CLIMATE-016");
    assert_contains(
        &climate,
        "HPHYS0318 SIMIMPL28 `stmtim` control-surface instrumentation invariant",
    );
    assert_contains(&climate, "OBL-CLIMATE-P-011");
    assert_contains(&climate, "snow.hourly.stmtim.rain_m_####");
    assert_contains(&climate, "SC-SNOWFREEZE-001#INV-SNOWFREEZE-044");
    assert_contains(&climate, "SC-WATBAL-001#INV-WATBAL-092");

    let snowfreeze = read("docs/specifications/science-contracts/contracts/SC-SNOWFREEZE-001.md");
    assert_contains(&snowfreeze, "contract_version:");
    assert_contains(&snowfreeze, "INV-SNOWFREEZE-044");
    assert_contains(
        &snowfreeze,
        "HPHYS0318 `stmtim` control-surface trace invariant",
    );
    assert_contains(&snowfreeze, "OBL-SNOWFREEZE-P-023");
    assert_contains(&snowfreeze, "snow.hourly.stmtim.*_####");
    assert_contains(&snowfreeze, "paired-fixed-baseline-stmtim-observe");
    assert_contains(&snowfreeze, "SC-CLIMATE-001#INV-CLIMATE-016");
    assert_contains(&snowfreeze, "SC-WATBAL-001#INV-WATBAL-092");

    let watbal = read("docs/specifications/science-contracts/contracts/SC-WATBAL-001.md");
    assert_contains(&watbal, "contract_version:");
    assert_contains(&watbal, "INV-WATBAL-092");
    assert_contains(
        &watbal,
        "HPHYS0318 `stmtim` control-surface water-balance gate",
    );
    assert_contains(&watbal, "OBL-WATBAL-P-028");
    assert_contains(&watbal, "snow_hourly_stmtim_rain_m");
    assert_contains(&watbal, "SC-SNOWFREEZE-001#INV-SNOWFREEZE-044");
    assert_contains(&watbal, "SC-CLIMATE-001#INV-CLIMATE-016");

    let index = read("docs/specifications/science-contracts/index.md");
    assert_contains(&index, "SC-CLIMATE-001#INV-CLIMATE-016");
    assert_contains(&index, "SC-SNOWFREEZE-001#INV-SNOWFREEZE-044");
    assert_contains(&index, "SC-WATBAL-001#INV-WATBAL-092");
}

#[test]
fn hphys0318_package_is_autonomous_and_scoped_to_instrumentation() {
    let package = read(&format!("{PACKAGE}/package.md"));
    assert_contains(&package, "Status: executed-hold");
    assert_contains(&package, "Contract-First Sequence");
    assert_contains(
        &package,
        "Production code edits limited to trace/diagnostic instrumentation",
    );
    assert_contains(&package, "No production precipitation-phase physics edit");
    assert_contains(
        &package,
        "No snow producer, drift, melt-term, branch-predicate",
    );
    assert_contains(&package, "Dual review findings");

    let prompt = read(&format!(
        "{PACKAGE}/prompts/active/20260606-hphys0318-stmtim-control-surface-instrumentation-001_kickoff_agent_prompt.md"
    ));
    assert_contains(&prompt, "Execution mode: package-end-to-end");
    assert_contains(&prompt, "Required reading");
    assert_contains(&prompt, "Autonomy:");
    assert_contains(&prompt, "no external connectivity");
    assert_contains(&prompt, "no production precipitation-phase physics edit");
}

#[test]
fn hphys0318_runtime_and_trace_symbols_are_registered() {
    let units = read("crates/openwepp-sim-contract/src/units.rs");
    for token in [
        "snow_hourly_stmtim_rain",
        "snow.hourly.stmtim.rain_m_{idx4}",
        "snow_hourly_stmtim_stmdur",
        "snow.hourly.stmtim.stmdur_s_{idx4}",
        "snow_hourly_stmtim_wntdur",
        "snow.hourly.stmtim.wntdur_h_{idx4}",
        "snow_hourly_stmtim_wnttim",
        "snow.hourly.stmtim.wnttim_h_{idx4}",
        "snow_hourly_stmtim_hrtemp",
        "snow.hourly.stmtim.hrtemp_c_{idx4}",
        "snow_hourly_stmtim_rst",
        "snow.hourly.stmtim.rst_c_{idx4}",
        "snow_hourly_stmtim_hrrain",
        "snow.hourly.stmtim.hrrain_m_{idx4}",
        "snow_hourly_stmtim_hrsnow",
        "snow.hourly.stmtim.hrsnow_m_{idx4}",
        "snow_hourly_stmtim_active_interval",
        "snow.hourly.stmtim.active_interval_{idx4}",
        "snow_hourly_stmtim_rain_branch",
        "snow.hourly.stmtim.rain_branch_{idx4}",
        "snow_hourly_stmtim_snow_branch",
        "snow.hourly.stmtim.snow_branch_{idx4}",
        "SC-CLIMATE-001#INV-CLIMATE-016",
    ] {
        assert_contains(&units, token);
    }

    let runtime = read(
        "crates/openwepp-hillslope-orchestrator/src/runtime_inputs/06_simimpl28_hourly_forcing.rs",
    );
    assert_contains(&runtime, "Simimpl28StmtimHourlyPartition");
    for token in [
        "snow.hourly.stmtim.rain_m",
        "snow.hourly.stmtim.stmdur_s",
        "snow.hourly.stmtim.wntdur_h",
        "snow.hourly.stmtim.wnttim_h",
        "snow.hourly.stmtim.hrtemp_c",
        "snow.hourly.stmtim.rst_c",
        "snow.hourly.stmtim.hrrain_m",
        "snow.hourly.stmtim.hrsnow_m",
        "snow.hourly.stmtim.active_interval",
        "snow.hourly.stmtim.rain_branch",
        "snow.hourly.stmtim.snow_branch",
    ] {
        assert_contains(&runtime, token);
    }

    let runner = read("crates/openwepp-runner/src/hillslope/mod.rs");
    assert_contains(
        &runner,
        "openwepp-hphys0245-wb11-wb18-wb19-wb17-evappm-branch-trace-v17",
    );
    for token in [
        "snow_hourly_stmtim_rain_m",
        "snow_hourly_stmtim_stmdur_s",
        "snow_hourly_stmtim_wntdur_h",
        "snow_hourly_stmtim_wnttim_h",
        "snow_hourly_stmtim_hrtemp_c",
        "snow_hourly_stmtim_rst_c",
        "snow_hourly_stmtim_hrrain_m",
        "snow_hourly_stmtim_hrsnow_m",
        "snow_hourly_stmtim_active_interval",
        "snow_hourly_stmtim_rain_branch",
        "snow_hourly_stmtim_snow_branch",
    ] {
        assert_contains(&runner, token);
    }
}

#[test]
fn hphys0318_ledger_preserves_57_rows_and_records_remaining_baseline_gap() {
    let ledger_path = format!("{PACKAGE}/artifacts/paired-stmtim-control-surface-ledger.md");
    assert!(
        Path::new(&ledger_path).exists(),
        "required HPHYS0318 paired stmtim ledger artifact is missing"
    );
    let ledger = read(&ledger_path);
    assert!(!ledger.contains("Status: queued"));
    assert!(!ledger.contains("Evidence mode: not-run"));
    assert_contains(&ledger, "Evidence mode: Static");
    assert_contains(&ledger, "carried_rows_total: `57`");
    assert_contains(&ledger, "HPHYS0315_spring_2014_rows: `24`");
    assert_contains(&ledger, "HPHYS0316_spring_2016_rows: `33`");
    assert_contains(&ledger, "openwepp_stmtim_trace_implemented: `true`");
    assert_contains(&ledger, "fixed_baseline_stmtim_observe_available: `false`");
    assert_contains(&ledger, "production_physics_edit_authorized: `false`");
    assert_contains(&ledger, "paired-fixed-baseline-stmtim-observe-hold");
    for token in [
        "`snow.hourly.stmtim.rain_m_0011`",
        "`snow.hourly.stmtim.stmdur_s_0011`",
        "`snow.hourly.stmtim.wntdur_h_0011`",
        "`snow.hourly.stmtim.wnttim_h_0011`",
        "`snow.hourly.stmtim.hrtemp_c_0011`",
        "`snow.hourly.stmtim.rst_c_0011`",
        "`snow.hourly.stmtim.hrrain_m_0011`",
        "`snow.hourly.stmtim.hrsnow_m_0011`",
        "`snow.hourly.stmtim.active_interval_0011`",
        "`snow.hourly.stmtim.rain_branch_0011`",
        "`snow.hourly.stmtim.snow_branch_0011`",
    ] {
        assert_contains(&ledger, token);
    }
}

#[test]
fn hphys0318_artifacts_are_executed_and_gates_are_recorded() {
    for relative in [
        "artifacts/README.md",
        "artifacts/contract-implementation-evidence.md",
        "artifacts/contract-test-implementation-evidence.md",
        "artifacts/pre-implementation-contract-gate.md",
        "artifacts/implementation-test-evidence.md",
        "artifacts/stmtim-openwepp-trace-instrumentation.md",
        "artifacts/paired-stmtim-control-surface-ledger.md",
        "artifacts/baseline-observe-gap.md",
        "artifacts/kernel-profile-compliance-checklist.md",
        "artifacts/owned-file-manifest.md",
        "artifacts/gate-results.md",
        "artifacts/review-disposition.md",
        "artifacts/review_agent_a.md",
        "artifacts/review_agent_b.md",
        "artifacts/verification_agent_a.md",
        "artifacts/verification_agent_b.md",
        "artifacts/disposition.md",
        "artifacts/worker-handoff.md",
        "artifacts/full-39-suite-metrics.md",
        "prompts/README.md",
        "prompts/active/README.md",
        "prompts/archived/README.md",
    ] {
        let content = read(&format!("{PACKAGE}/{relative}"));
        assert!(
            !content.contains("Status: queued"),
            "{relative} still has queued scaffold status"
        );
        assert!(
            !content.contains("Evidence mode: not-run"),
            "{relative} still has not-run evidence mode"
        );
    }

    let metrics = read(&format!("{PACKAGE}/artifacts/full-39-suite-metrics.md"));
    assert_contains(&metrics, "H1..H39");
    assert_contains(&metrics, "0/39");
    assert_contains(&metrics, "instrumentation-only carry-forward");

    let gate_results = read(&format!("{PACKAGE}/artifacts/gate-results.md"));
    for token in [
        "cargo fmt --check",
        "cargo clippy --workspace --all-targets -- -D warnings",
        "cargo test --workspace",
        "cargo deny check",
        "markdown-doc lint",
        "check_authority_suite_antievasion.sh",
        "auth11_required_suite_obligation_guards_contract",
        "exit status was `0`",
    ] {
        assert_contains(&gate_results, token);
    }

    let disposition = read(&format!("{PACKAGE}/artifacts/disposition.md"));
    assert_contains(&disposition, "Final disposition: `executed-hold`");
    assert_contains(&disposition, "HPHYS0319");
    assert_contains(&disposition, "OpenWEPP-side SIMIMPL28 runtime symbols");

    let review_disposition = read(&format!("{PACKAGE}/artifacts/review-disposition.md"));
    assert_contains(&review_disposition, "A-001");
    assert_contains(&review_disposition, "`accepted`");
    assert_contains(&review_disposition, "B-001");
    assert_contains(&review_disposition, "No undispositioned findings remain");

    let verification_a = read(&format!("{PACKAGE}/artifacts/verification_agent_a.md"));
    assert_contains(&verification_a, "Final verification: PASS");
    let verification_b = read(&format!("{PACKAGE}/artifacts/verification_agent_b.md"));
    assert_contains(&verification_b, "Final verification: PASS");

    let handoff = read(&format!("{PACKAGE}/artifacts/worker-handoff.md"));
    assert_contains(&handoff, "Status: complete");
    assert_contains(&handoff, "Next package: `HPHYS0319`");
}
