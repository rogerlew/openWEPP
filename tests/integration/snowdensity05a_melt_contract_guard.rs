use std::fs;

const CONTRACT: &str = "docs/specifications/science-contracts/contracts/SC-SNOWFREEZE-001.md";
const CLIMATE_CONTRACT: &str = "docs/specifications/science-contracts/contracts/SC-CLIMATE-001.md";
const PACKAGE: &str =
    "docs/work-packages/20260625-snowdensity-05-melt-modernization-contract-first-001/package.md";
const HANDOFF: &str = "docs/work-packages/20260625-snowdensity-05-melt-modernization-contract-first-001/artifacts/worker-handoff.md";

#[test]
fn snowdensity05a_contract_ratifies_melt_modernization_envelope() {
    let contract = read(CONTRACT);
    for marker in [
        "contract_version: 89",
        "INV-SNOWFREEZE-052",
        "SNOWDENSITY-05A CoE melt-modernization contract and sign-convention envelope",
        "snow_melt_model = legacy_coe | coe_shortwave_albedo_v1",
        "`legacy_coe` remains the default",
        "`coe_shortwave_albedo_v1` is opt-in only",
        "`dense_slow_melt_v1` remains a negative benchmark",
        "Shared radiation forcing must not be tuned",
        "SC-CLIMATE-001#INV-CLIMATE-013",
        "OBL-SNOWFREEZE-P-027",
        "SNOWDENSITY-05A CoE Melt Modernization Contract Addendum",
    ] {
        assert_contains(&contract, marker, CONTRACT);
    }

    let climate_contract = read(CLIMATE_CONTRACT);
    assert_contains(&climate_contract, "INV-CLIMATE-013", CLIMATE_CONTRACT);
    assert_contains(
        &climate_contract,
        "winter.hourly.rad_mj_m2_####",
        CLIMATE_CONTRACT,
    );
}

#[test]
fn snowdensity05a_contract_reconciles_bmelt_signed_trace_convention() {
    let contract = read(CONTRACT);
    for marker in [
        "`melt_bmelt_in` stores the signed `bmelt` contribution",
        "`hrmelt_raw = 0.0254 * (amelt + melt_bmelt_in + cmelt + dmelt)`",
        "WEPP Chapter 3 prose writes this term as `- bmelt`",
        "silent sign flip or double subtraction",
        "tests/integration/clim05_snow_runtime_kernel_contract.rs",
    ] {
        assert_contains(&contract, marker, CONTRACT);
    }
}

#[test]
fn snowdensity05a_package_closes_as_contract_only_with_follow_on_handoff() {
    let package = read(PACKAGE);
    for marker in [
        "Status: complete.",
        "Package type: contract/sign-reconciliation package.",
        "No production runtime code, constants, parser surfaces, output schemas, or",
        "defaults are changed by SNOWDENSITY-05A.",
        "Subagent authorization: not used.",
        "Closure: COMPLETE-05A-CONTRACT-SIGN-GATE",
    ] {
        assert_contains(&package, marker, PACKAGE);
    }

    let handoff = read(HANDOFF);
    for marker in [
        "Next recommended package: `SNOWDENSITY-05B Shortwave Source Binding`",
        "Do not implement `coe_shortwave_albedo_v1` production melt until 05B and 05C are complete.",
        "Preserve the signed `melt_bmelt_in` convention.",
        "Do not promote `dense_slow_melt_v1`.",
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
