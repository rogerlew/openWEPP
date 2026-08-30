use std::fs;
use std::path::PathBuf;

fn read(path: &str) -> String {
    fs::read_to_string(PathBuf::from(env!("CARGO_MANIFEST_DIR")).join(path))
        .unwrap_or_else(|error| panic!("failed to read {path}: {error}"))
}

const ENERGY: &str = "docs/specifications/science-contracts/contracts/SC-SNOWENERGY-001.md";
const SNOW: &str = "docs/specifications/science-contracts/contracts/SC-SNOWFREEZE-001.md";

#[test]
fn v11_v134_separate_source_adjusted_and_virtual_wind() {
    let energy = read(ENERGY);
    let snow = read(SNOW);

    for required in [
        "contract_version: 29",
        "Version 18 defines the persistent Stage 3 snow--soil conductive boundary",
        "INV-SNOWENERGY-033",
        "`z_u,source`",
        "nominal `10 m`",
        "`u_cli`",
        "`u_2,PMET`",
        "virtual `z_u=5 m`",
        "never a Stage 3 input",
        "AUTHORITY_MISSING",
        "nearest pre-build code statically reconstructs",
        "one-decimal formatting",
    ] {
        assert!(energy.contains(required), "{ENERGY} missing {required}");
    }
    for required in [
        "contract_version: 140",
        "| `2026-08-07` | `132` | `Codex` |",
        "INV-SNOWFREEZE-099",
        "`fwv_m_s` is local and cannot feed snow",
        "never measurement height",
        "authoritative linkage are still required before `APPLICABLE` or `INAPPLICABLE`",
        "Modeled evergreen forest with `cancov=0.9` establishes target model intent only",
    ] {
        assert!(snow.contains(required), "{SNOW} missing {required}");
    }
}

#[test]
fn custody_contract_rejects_height_and_exposure_aliases() {
    let energy = read(ENERGY);
    let snow = read(SNOW);

    for required in [
        "Neither values, residuals, a height conversion, nor a desired energy balance",
        "fit attenuation",
        "license a canopy operator",
        "no production correction",
    ] {
        assert!(energy.contains(required), "{ENERGY} missing {required}");
    }
    for required in [
        "Values, residuals, height conversion, and desired energy balance cannot infer exposure",
        "target canopy/snow class",
        "authoritative linkage",
        "otherwise emit `AUTHORITY_MISSING`",
    ] {
        assert!(snow.contains(required), "{SNOW} missing {required}");
    }
}

#[test]
fn actual_runtime_source_keeps_pmet_adjustment_out_of_stage3() {
    let parser = read("crates/openwepp-input-contract/src/parsers/climate.rs");
    let projection =
        read("crates/openwepp-hillslope-orchestrator/src/runtime_inputs/03_climate.rs");
    let snow = read(
        "crates/openwepp-runner/src/hillslope/direct_publication/day_input_and_helpers/00a_snow_frost_authority_impl.rs",
    );
    let pmet = read(
        "crates/openwepp-runner/src/hillslope/direct_publication/day_input_and_helpers/00d_authority_runtime_impl.rs",
    );

    assert!(parser.contains("let vwind = parse_f64(tokens[10]"));
    assert!(projection.contains("vwind_m_s: day.vwind"));
    assert!(snow.contains("wind_m_s: forcing.vwind_m_s"));
    assert!(pmet.contains("let fwv_m_s = forcing.vwind_m_s * 4.87"));
    assert!(!snow.contains("fwv_m_s"));
}
