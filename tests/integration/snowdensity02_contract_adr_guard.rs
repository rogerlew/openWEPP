use std::fs;

const CONTRACT: &str = "docs/specifications/science-contracts/contracts/SC-SNOWFREEZE-001.md";
const ADR: &str = "docs/decisions/0027-opt-in-physics-bulk-snow-model.md";
const ADR_INDEX: &str = "docs/decisions/README.md";
const PACKAGE: &str = "docs/work-packages/20260625-snowdensity-02-contract-adr-001/package.md";
const HANDOFF: &str =
    "docs/work-packages/20260625-snowdensity-02-contract-adr-001/artifacts/worker-handoff.md";

#[test]
fn snowdensity02_contract_ratifies_opt_in_physics_bulk_envelope() {
    let contract = read(CONTRACT);
    for marker in [
        "contract_version: 115",
        "INV-SNOWFREEZE-051",
        "SNOWDENSITY-02 opt-in `physics_bulk` snow-model envelope",
        "snow_model = legacy_wepp | physics_bulk",
        "`legacy_wepp` remaining the default",
        "Site-specific calibration is prohibited",
        "`ssd` remains a legacy control-state/provenance input",
        "Anderson-1976/SNOBAL-style metamorphism",
        "not an authorized production formula",
        "OBL-SNOWFREEZE-P-026",
        "Snow-Density Physics-Bulk Candidate Envelope Addendum",
    ] {
        assert_contains(&contract, marker, CONTRACT);
    }
}

#[test]
fn snowdensity02_adr_records_deliberate_legacy_divergence_without_default_activation() {
    let adr = read(ADR);
    for marker in [
        "# ADR-0027: Opt-in physics-bulk snow model for snow-density remediation",
        "**Status:** Accepted",
        "Authorize a deliberate-legacy-divergence snow model lane named",
        "snow_model = legacy_wepp | physics_bulk",
        "`legacy_wepp` remains the default production behavior",
        "`physics_bulk` is opt-in candidate scope only",
        "must not use site-specific tuning",
        "first implementation target is an offline Rust snowbench physics core",
        "does not select exact equations/constants",
    ] {
        assert_contains(&adr, marker, ADR);
    }

    let index = read(ADR_INDEX);
    assert_contains(
        &index,
        "[0027](0027-opt-in-physics-bulk-snow-model.md)",
        ADR_INDEX,
    );
}

#[test]
fn snowdensity02_package_closes_as_contract_only_with_follow_on_runtime_hold() {
    let package = read(PACKAGE);
    for marker in [
        "Status: complete.",
        "Package type: contract / ADR governance.",
        "No production runtime snow physics, constants, model options, parser surfaces, output schemas, or default behavior changes.",
        "Contract guard test passes and would fail if the opt-in/no-site-tuning language is removed.",
        "Subagent authorization: none.",
    ] {
        assert_contains(&package, marker, PACKAGE);
    }

    let handoff = read(HANDOFF);
    for marker in [
        "Next recommended package: `SNOWDENSITY-03 Offline Physics Core`",
        "Implement `physics_bulk` in Rust snowbench only.",
        "Do not couple `physics_bulk` into production runtime yet.",
        "No per-site constants.",
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
