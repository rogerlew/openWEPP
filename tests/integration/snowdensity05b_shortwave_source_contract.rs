use std::fs;

const CONTRACT: &str = "docs/specifications/science-contracts/contracts/SC-SNOWFREEZE-001.md";
const CLIMATE_CONTRACT: &str = "docs/specifications/science-contracts/contracts/SC-CLIMATE-001.md";
const EVAP_CONTRACT: &str = "docs/specifications/science-contracts/contracts/SC-EVAP-001.md";
const PACKAGE: &str =
    "docs/work-packages/20260626-snowdensity-05b-shortwave-source-binding-001/package.md";
const HANDOFF: &str = "docs/work-packages/20260626-snowdensity-05b-shortwave-source-binding-001/artifacts/worker-handoff.md";
const LEDGER: &str = "docs/work-packages/20260626-snowdensity-05b-shortwave-source-binding-001/artifacts/source-provenance-ledger.md";

#[test]
fn snowdensity05b_contract_binds_shortwave_source_authority() {
    let contract = read(CONTRACT);
    for marker in [
        "contract_version: 109",
        "INV-SNOWFREEZE-053",
        "SNOWDENSITY-05B shortwave source/provenance binding",
        "canonical openWEPP shortwave source is the daily climate `rad`/`radly` field",
        "`Ly d^-1`",
        "`radmj = radly * 0.04184`",
        "`sunmap`",
        "`radcur`",
        "`hr_tmp`",
        "`winter.hourly.rad_mj_m2_####`",
        "`SC-CLIMATE-001#INV-CLIMATE-013`",
        "openWEPP must not fetch, select, spatialize, or tune gridded shortwave products",
        "snow-only radiation scalar",
        "ET and snowmelt consume the same daily radiation authority",
        "OBL-SNOWFREEZE-P-028",
        "SNOWDENSITY-05B Shortwave Source Binding Addendum",
    ] {
        assert_contains(&contract, marker, CONTRACT);
    }
}

#[test]
fn snowdensity05b_contract_preserves_climate_and_et_shared_authority() {
    let climate_contract = read(CLIMATE_CONTRACT);
    for marker in [
        "INV-CLIMATE-013",
        "`radly`",
        "`radmj = radly * 0.04184`",
        "`sunmap`",
        "`radcur`",
        "`winter.hourly.rad_mj_m2_####`",
        "must not clip, cap, renormalize, or compensate",
    ] {
        assert_contains(&climate_contract, marker, CLIMATE_CONTRACT);
    }

    let evap_contract = read(EVAP_CONTRACT);
    for marker in ["INV-EVAP-021", "`RA`", "`radpot`", "daily solar radiation"] {
        assert_contains(&evap_contract, marker, EVAP_CONTRACT);
    }
}

#[test]
fn snowdensity05b_package_closes_source_binding_without_runtime_changes() {
    let package = read(PACKAGE);
    for marker in [
        "Status: complete.",
        "Package type: contract/source-binding package.",
        "Closure: COMPLETE-05B-SHORTWAVE-SOURCE-BINDING.",
        "No production runtime code, constants, parser surfaces, output schemas, or",
        "defaults are changed by SNOWDENSITY-05B.",
        "Upstream gridded product selection and spatialization remain outside",
        "Subagent authorization: not used.",
    ] {
        assert_contains(&package, marker, PACKAGE);
    }

    let ledger = read(LEDGER);
    for marker in [
        "Canonical openWEPP acceptance point",
        "daily climate `rad`/`radly`",
        "`Ly d^-1`",
        "`radmj = radly * 0.04184`",
        "`sunmap`",
        "`radcur`/`hr_tmp`",
        "ET shared authority",
        "snow-only radiation scalar",
    ] {
        assert_contains(&ledger, marker, LEDGER);
    }

    let handoff = read(HANDOFF);
    for marker in [
        "Next recommended package: `SNOWDENSITY-05C Albedo State Core`",
        "Do not implement `coe_shortwave_albedo_v1` production melt until 05C is complete.",
        "Preserve the 05B radiation-source binding.",
        "Do not tune, rescale, clip, or reinterpret shared radiation forcing for snowmelt.",
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
