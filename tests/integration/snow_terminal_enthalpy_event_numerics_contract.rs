use std::fs;

const ENERGY: &str = "docs/specifications/science-contracts/contracts/SC-SNOWENERGY-001.md";
const FREEZE: &str = "docs/specifications/science-contracts/contracts/SC-SNOWFREEZE-001.md";
const INDEX: &str = "docs/specifications/science-contracts/index.md";
const FIXED_POINT: &str = "crates/openwepp-hillslope-orchestrator/src/v11_covered/fixed_point.rs";
const OPEN_SNOW: &str = "crates/openwepp-hillslope-orchestrator/src/v11_covered/open_snow.rs";
const OPEN_SNOW_TESTS: &str =
    "crates/openwepp-hillslope-orchestrator/src/v11_covered/open_snow_convergence_tests.rs";
const PACKAGE: &str =
    "docs/work-packages/20260807-snow-terminal-enthalpy-event-numerics-001/package.md";

fn read(path: &str) -> String {
    fs::read_to_string(path).unwrap_or_else(|error| panic!("read {path}: {error}"))
}

#[test]
fn contracts_admit_only_event_local_terminal_snow_numerics() {
    let energy = read(ENERGY);
    let freeze = read(FREEZE);
    for required in [
        "contract_version: 32",
        "Version 18 defines the persistent Stage 3 snow--soil conductive boundary",
        "INV-SNOWENERGY-034",
        "OBL-SNOWENERGY-P-008",
        "OBL-SNOWENERGY-C-015",
        "TOL-SNOWENERGY-001",
        "H=-Q_cc+L_f m_l",
        "g(tau)=m_i,start+m_refrozen(tau)+m_deposition(tau)-m_sublimation(tau)-m_melt(tau)",
        "h_min=1e-9 s",
        "safeguarded bisection",
        "unevaluated_seconds=requested-t_event",
        "neither is a land-surface recipient",
    ] {
        assert!(energy.contains(required), "{ENERGY} missing {required}");
    }
    for required in [
        "contract_version: 140",
        "| `2026-08-07` | `135` | `Codex` |",
        "INV-SNOWFREEZE-101",
        "OBL-SNOWFREEZE-P-073",
        "OBL-SNOWFREEZE-C-015",
        "TOL-SNOWFREEZE-022",
        "Enabled internal schema v8",
        "Ordinary persistent evaluation remains exact schema v7",
    ] {
        assert!(freeze.contains(required), "{FREEZE} missing {required}");
    }
}

#[test]
fn terminal_endpoint_identity_includes_deposition_and_refreeze() {
    let energy = read(ENERGY);
    for required in [
        "deposition/refreeze cannot retroactively enlarge same-trial melt availability",
        "no event while deposited or refrozen solid remains",
        "complete solid identity—not a debit clamp—must establish zero ice",
        "Q_terminal_unallocated=Q_complete+Q_refreeze-Delta H_cc-L_f m_melt >= 0",
        "No snow-domain state receives energy and no snow flux is evaluated after the event",
    ] {
        assert!(energy.contains(required), "{ENERGY} missing {required}");
    }
}

#[test]
fn package_and_index_preserve_receiving_surface_and_production_boundaries() {
    let package = read(PACKAGE);
    let energy = read(ENERGY);
    let freeze = read(FREEZE);
    let index = read(INDEX);
    assert!(package.contains("No land-surface, soil, frost, infiltration, runoff"));
    assert!(package.contains("No assignment of terminal unallocated energy"));
    assert!(package.contains("No physical seasonal efficacy"));
    assert!(index.contains("v32 retains v31 same-disposition `W/H`"));
    assert!(
        index.contains("v140 owner amendment selects an exact 60-second adaptive Stage-3 floor")
    );
    assert!(
        energy.contains("Version 18 defines the persistent Stage 3 snow--soil conductive boundary")
    );
    assert!(
        freeze.contains("Version 140 supersedes the active mechanics of `INV-SNOWFREEZE-101/102`")
    );
    assert!(freeze.contains("The old receiver evidence remains\nhistorical evidence"));
}

#[test]
fn v32_contract_binds_pure_opposite_sign_vapor_active_set_authority() {
    let energy = read(ENERGY);
    let index = read(INDEX);
    for required in [
        "REF-SNOWENERGY-WGHL-V32",
        "INV-SNOWENERGY-056",
        "OBL-SNOWENERGY-C-024",
        "alpha_v=-V_0/(V_1-V_0)",
        "V=D=S=Q_v=+0",
        "alpha_v=0.04393657257739406",
        "+45.77845449909091 J m^-2",
        "w_p=max(0.25,min(0.5,120 s/h))",
        "same-disposition endpoints",
        "any exact covered support at or above the unchanged 60-second floor",
        "synthetic numerical states",
        "only a later fresh authentic image",
        "No diagnostic from this numerical path is persisted",
    ] {
        assert!(energy.contains(required), "{ENERGY} missing {required}");
    }
    assert!(index.contains("pure opposite-sign vapor-root interface"));
    assert!(index.contains("exact supports at or above the floor"));
    assert!(index.contains("affine latent interpolation is not authority"));
}

#[test]
fn v32_captured_root_rejects_affine_latent_interpolation() {
    let current_vapor_kg_m2 = 2.121_596_912_395_713_5e-4_f64;
    let authentic_vapor_kg_m2 = -4.616_612_304_251_271e-3_f64;
    let current_latent_j_m2 = 649.057_936_925_198_f64;
    let authentic_latent_j_m2 = -13_081.632_625_326_402_f64;

    let alpha = current_vapor_kg_m2 / (current_vapor_kg_m2 - authentic_vapor_kg_m2);
    let localized_vapor =
        current_vapor_kg_m2 + alpha * (authentic_vapor_kg_m2 - current_vapor_kg_m2);
    let forbidden_affine_latent =
        current_latent_j_m2 + alpha * (authentic_latent_j_m2 - current_latent_j_m2);

    assert_eq!(alpha.to_bits(), 0.043_936_572_577_394_06_f64.to_bits());
    assert_eq!(localized_vapor.to_bits(), 0.0_f64.to_bits());
    assert_eq!(
        forbidden_affine_latent.to_bits(),
        45.778_454_499_090_91_f64.to_bits()
    );
    assert_ne!(forbidden_affine_latent.to_bits(), 0.0_f64.to_bits());
}

#[test]
fn v32_production_symbols_and_larger_direct_support_behavior_are_required() {
    let fixed_point = read(FIXED_POINT);
    let open_snow = read(OPEN_SNOW);
    let open_snow_tests = read(OPEN_SNOW_TESTS);

    for required in [
        "CoveredVaporActiveSetInterfaceV1",
        "covered_vapor_active_set_interface_v1",
        "covered_vapor_active_set_branch_entry_v1",
        "COVERED_VAPOR_ACTIVE_SET_MIN_SUPPORT_NS",
    ] {
        assert!(
            fixed_point.contains(required),
            "{FIXED_POINT} missing version-32 production symbol {required}"
        );
    }
    assert!(
        open_snow.contains("covered_vapor_active_set_interface_v1"),
        "{OPEN_SNOW} must dispatch the version-32 private transition"
    );
    for required in [
        "v32_vapor_active_set_accepts_direct_support_above_exact_floor",
        "v32_vapor_active_set_retains_same_sign_v31_dispatch",
        "v32_vapor_active_set_synthetic_images_cannot_publish",
    ] {
        assert!(
            open_snow_tests.contains(required),
            "{OPEN_SNOW_TESTS} missing production behavior obligation {required}"
        );
    }
}
