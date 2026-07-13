use std::fs;

const CONTRACT: &str = "docs/specifications/science-contracts/contracts/SC-SNOWFREEZE-001.md";
const PACKAGE: &str =
    "docs/work-packages/20260626-snowdensity-05f-melt-closure-density-handoff-001/package.md";
const HANDOFF: &str = "docs/work-packages/20260626-snowdensity-05f-melt-closure-density-handoff-001/artifacts/worker-handoff.md";
const BUILDER: &str = "crates/openwepp-runner/src/hillslope/direct_publication/day_input_and_helpers/00c_day_input_builder_impl.rs";
const SNOW_FROST_IMPL: &str = "crates/openwepp-runner/src/hillslope/direct_publication/day_input_and_helpers/00a_snow_frost_authority_impl.rs";
const SNOWBENCH: &str = "crates/openwepp-runner/src/bin/openwepp-snowbench.rs";
const PRODUCTION_BINS: &[&str] = &[
    "crates/openwepp-runner/src/bin/open_wepp_runner.rs",
    "crates/openwepp-runner/src/bin/openwepp-cli-hill.rs",
    "crates/openwepp-runner/src/bin/openwepp-cli-totalwatsed3.rs",
    "crates/openwepp-runner/src/bin/openwepp-cli-watershed.rs",
];

#[test]
fn snowdensity05f_contract_closes_melt_without_default_activation() {
    let contract = read(CONTRACT);
    for marker in [
        "contract_version: 117",
        "INV-SNOWFREEZE-056",
        "SNOWDENSITY-05F melt closure / density handoff",
        "SNOWDENSITY-05F closes the melt-modernization ladder without default activation",
        "`legacy_coe` remains the default and rollback path",
        "`coe_shortwave_albedo_v1` remains opt-in only",
        "accepted only as a density-facing interface",
        "activation evidence baseline requires both diagnostic replay and H as-built context",
        "05E diagnostic replay deltas are regime-limited",
        "diagnostic harness used `cancov = 0.0` and PySnobal-bridge radiation",
        "configured coniferous forest winter canopy cover of about `0.9`",
        "configured coniferous forest winter `cancov` expected near `0.9`",
        "same-day future snowfall",
        "SNOWDENSITY-06 may consume the opt-in melt boundary without retuning melt",
        "Brock albedo constants",
        "OBL-SNOWFREEZE-P-031",
        "SNOWDENSITY-05F Melt Closure / Density Handoff Addendum",
    ] {
        assert_contains(&contract, marker, CONTRACT);
    }
}

#[test]
fn snowdensity05f_production_default_and_cli_remain_confined() {
    let builder = format!("{}\n{}", read(BUILDER), read(SNOW_FROST_IMPL));
    assert_contains(
        &builder,
        "snow_melt_model: self.snow_melt_model",
        "direct publication snow/frost sources",
    );
    assert_contains(
        &builder,
        "Ok(openwepp_hillslope_orchestrator::SnowMeltModel::LegacyCoe)",
        "direct publication snow/frost sources",
    );
    assert_not_contains(
        &builder,
        "SnowMeltModel::CoeShortwaveAlbedoV1",
        "direct publication snow/frost sources",
    );

    for path in PRODUCTION_BINS {
        let source = read(path);
        assert_not_contains(&source, "--model", path);
        assert_not_contains(&source, "coe_shortwave_albedo_v1", path);
        assert_not_contains(&source, "SnowMeltModel::CoeShortwaveAlbedoV1", path);
    }

    let snowbench = read(SNOWBENCH);
    assert_contains(&snowbench, "coe-melt", SNOWBENCH);
    assert_contains(&snowbench, "--model", SNOWBENCH);
    assert_contains(&snowbench, "coe_shortwave_albedo_v1", SNOWBENCH);
}

#[test]
fn snowdensity05f_package_closes_with_density_handoff() {
    let package = read(PACKAGE);
    for marker in [
        "Status: complete.",
        "Closure: COMPLETE-05F-MELT-CLOSURE-DENSITY-HANDOFF.",
        "No default activation, parser/runfile/CLI selector, output schema,",
        "Subagent authorization: not used.",
    ] {
        assert_contains(&package, marker, PACKAGE);
    }

    let handoff = read(HANDOFF);
    for marker in [
        "Next recommended package: `SNOWDENSITY-06 Density Compaction`",
        "Consume `coe_shortwave_albedo_v1` only as a fixed opt-in melt boundary.",
        "Do not retune melt, albedo, coefficients, or shared radiation for density.",
        "Report both 05E diagnostic replay and H as-built context before any default-candidate claim.",
        "Treat the 05E diagnostic replay deltas as regime-limited",
        "SNOWDENSITY-06 Entry Gate",
        "coniferous forest winter `cancov` near `0.9`",
        "native openWEPP shortwave or prove the PySnobal-bridge radiation",
        "legacy_coe` remains the default and rollback path",
    ] {
        assert_contains(&handoff, marker, HANDOFF);
    }
}

fn read(path: &str) -> String {
    fs::read_to_string(path).unwrap_or_else(|err| panic!("failed to read {path}: {err}"))
}

fn assert_contains(text: &str, marker: &str, path: &str) {
    assert!(
        text.contains(marker),
        "expected {path} to contain marker: {marker}"
    );
}

fn assert_not_contains(text: &str, marker: &str, path: &str) {
    assert!(
        !text.contains(marker),
        "expected {path} not to contain marker: {marker}"
    );
}
