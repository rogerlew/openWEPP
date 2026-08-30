use std::fs;

const ENERGY: &str = "docs/specifications/science-contracts/contracts/SC-SNOWENERGY-001.md";
const FREEZE: &str = "docs/specifications/science-contracts/contracts/SC-SNOWFREEZE-001.md";
const TIME: &str = "docs/specifications/science-contracts/contracts/SC-COUPLEDTIME-001.md";
const SURFACE_LIQUID: &str =
    "docs/specifications/science-contracts/contracts/SC-SURFACELIQUID-001.md";
const PACKAGE: &str =
    "docs/work-packages/20260826-snow-stage3-adaptive-compositional-microstepping-001/package.md";
const LF: f64 = 333_600.0;
const MIN_STEP_NS: u128 = 60_000_000_000;

fn read(path: &str) -> String {
    fs::read_to_string(path).unwrap_or_else(|error| panic!("read {path}: {error}"))
}

#[derive(Clone, Copy, Debug)]
struct Phase {
    ice: f64,
    liquid: f64,
    cold: f64,
    deposition: f64,
    sublimation: f64,
    melt: f64,
    refreeze: f64,
    unallocated: f64,
    solid_residual: f64,
    liquid_residual: f64,
    energy_residual: f64,
}

fn project(i0: f64, l0: f64, c0: f64, vapor: f64, lin: f64, q: f64) -> Phase {
    let deposition = vapor.max(0.0);
    let sublimation = (-vapor).max(0.0).min(i0);
    let water = i0 + l0 + deposition - sublimation + lin;
    let enthalpy = -c0 + LF * (l0 + lin) + q;
    let (ice, liquid, cold, unallocated) = if enthalpy < 0.0 {
        (water, 0.0, -enthalpy, 0.0)
    } else if enthalpy < LF * water {
        let liquid = enthalpy / LF;
        (water - liquid, liquid, 0.0, 0.0)
    } else {
        (0.0, water, 0.0, (enthalpy - LF * water).max(0.0))
    };
    let liquid_pre = l0 + lin;
    let melt = (liquid - liquid_pre).max(0.0);
    let refreeze = (liquid_pre - liquid).max(0.0);
    Phase {
        ice,
        liquid,
        cold,
        deposition,
        sublimation,
        melt,
        refreeze,
        unallocated,
        solid_residual: i0 + deposition + refreeze - sublimation - melt - ice,
        liquid_residual: l0 + lin + melt - refreeze - liquid,
        energy_residual: q - (c0 - cold) - LF * melt + LF * refreeze - unallocated,
    }
}

fn split_quanta(quanta: u128) -> Option<(u128, u128)> {
    if quanta < 2 {
        return None;
    }
    let first = quanta / 2;
    Some((first, quanta - first))
}

fn assert_closed(value: Phase) {
    assert!(value.solid_residual.abs() <= 1.0e-12, "{value:?}");
    assert!(value.liquid_residual.abs() <= 1.0e-12, "{value:?}");
    assert!(value.energy_residual.abs() <= 1.0e-6, "{value:?}");
    assert!(value.ice == 0.0 || value.unallocated == 0.0, "{value:?}");
    assert!(value.ice >= 0.0 && value.liquid >= 0.0 && value.cold >= 0.0);
}

#[test]
fn successor_contracts_bind_one_adaptive_model_and_preserve_rejected_history() {
    let energy = read(ENERGY);
    let freeze = read(FREEZE);
    let time = read(TIME);
    let surface_liquid = read(SURFACE_LIQUID);
    let package = read(PACKAGE);
    for required in [
        "contract_version: 26",
        "INV-SNOWENERGY-048",
        "INV-SNOWENERGY-049",
        "INV-SNOWENERGY-050",
        "TOL-SNOWENERGY-005",
        "W = I0 + L0 + D - S + Lin",
        "Versions 19 through 21 remain historical",
    ] {
        assert!(energy.contains(required), "{ENERGY} missing {required}");
    }
    for required in [
        "contract_version: 140",
        "INV-SNOWFREEZE-103",
        "INV-SNOWFREEZE-104",
        "INV-SNOWFREEZE-105",
        "INV-SNOWFREEZE-106",
        "Versions 137 through 139 are rejected historical candidates",
    ] {
        assert!(freeze.contains(required), "{FREEZE} missing {required}");
    }
    for required in [
        "contract_version: 9",
        "INV-COUPLEDTIME-021",
        "INV-COUPLEDTIME-022",
        "INV-COUPLEDTIME-023",
        "INV-COUPLEDTIME-024",
        "INV-COUPLEDTIME-026",
        "OBL-COUPLEDTIME-009",
        "versions 4-6 remain rejected historical candidates",
    ] {
        assert!(time.contains(required), "{TIME} missing {required}");
    }
    for forbidden in [
        "CoE fallback",
        "fixed-step fallback",
        "selectable legacy terminal",
    ] {
        assert!(!package.contains(&format!("add {forbidden}")));
    }
    for required in [
        "contract_version: 13",
        "SURFACELIQUID-V13-FACTORIZATION-LINEAGE",
        "SURFACELIQUID-V10-ADAPTIVE-GRID",
        "60_000_000_000 ns",
        "result-blind",
    ] {
        assert!(
            surface_liquid.contains(required),
            "{SURFACE_LIQUID} missing {required}"
        );
    }
}

#[test]
fn phase_projection_matrix_closes_and_deposition_participates_in_melt() {
    let cases = [
        project(0.6, 0.0, 10_000.0, 0.0, 0.0, 0.0),
        project(0.6, 0.0, 0.0, 0.0, 0.0, LF * 0.3),
        project(0.6, 0.0, 0.0, 0.0, 0.0, LF * 0.6),
        project(0.6, 0.0, 0.0, 0.0, 0.0, LF * 0.7),
        project(0.6, 0.1, 0.0, 0.002, 0.0, LF * 0.602),
        project(0.6, 0.1, 30_000.0, 0.0, 0.2, -50_000.0),
        project(0.6, 0.0, 0.0, -0.9, 0.0, 0.0),
    ];
    for value in cases {
        assert_closed(value);
    }

    let exact = project(0.6, 0.0, 0.0, 0.002, 0.0, LF * 0.602);
    assert_eq!(exact.ice.to_bits(), 0.0_f64.to_bits());
    assert_eq!(exact.unallocated.to_bits(), 0.0_f64.to_bits());
    assert!((exact.melt - 0.602).abs() <= 1.0e-12);
    assert!((exact.deposition - 0.002).abs() <= 1.0e-12);

    let excess = project(0.6, 0.0, 0.0, 0.002, 0.0, LF * 0.603);
    assert_eq!(excess.ice.to_bits(), 0.0_f64.to_bits());
    assert!(excess.unallocated > 0.0);
    assert_closed(excess);

    let bounded = project(0.6, 0.0, 0.0, -0.9, 0.0, 0.0);
    assert_eq!(bounded.sublimation.to_bits(), 0.6_f64.to_bits());
    assert_eq!(bounded.ice.to_bits(), 0.0_f64.to_bits());
    let refrozen = project(0.2, 0.1, 40_000.0, 0.0, 0.0, -10_000.0);
    assert!(refrozen.refreeze > 0.0);
    assert_closed(refrozen);
}

#[test]
fn exact_grid_split_is_positive_complete_and_unequal_for_odd_counts() {
    assert_eq!(split_quanta(1), None);
    assert_eq!(split_quanta(2), Some((1, 1)));
    assert_eq!(split_quanta(3), Some((1, 2)));
    assert_eq!(split_quanta(5), Some((2, 3)));
    for quanta in 2..=3_000_u128 {
        let (first, second) = split_quanta(quanta).unwrap();
        assert!(first >= 1 && second >= 1);
        assert_eq!(first + second, quanta);
        assert!(first * MIN_STEP_NS >= MIN_STEP_NS);
        assert!(second * MIN_STEP_NS >= MIN_STEP_NS);
    }
}

#[test]
fn bounded_vapor_requires_mass_energy_pairing() {
    fn bounded(raw_mass: f64, raw_energy: f64, available_ice: f64) -> (f64, f64) {
        if raw_mass >= 0.0 {
            return (raw_mass, raw_energy);
        }
        let actual = raw_mass.max(-available_ice);
        if raw_mass == 0.0 {
            (0.0, 0.0)
        } else {
            (actual, raw_energy * (actual / raw_mass))
        }
    }
    assert_eq!(bounded(0.0, 0.0, 0.6), (0.0, 0.0));
    assert_eq!(bounded(0.2, 560_000.0, 0.6), (0.2, 560_000.0));
    let (mass, energy) = bounded(-0.9, -2_520_000.0, 0.6);
    assert!((mass + 0.6).abs() <= 1.0e-12);
    assert!((energy + 1_680_000.0).abs() <= 1.0e-9);
    assert!((energy / mass - 2_800_000.0).abs() <= 1.0e-9);
}
