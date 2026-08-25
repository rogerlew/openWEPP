use std::fs;

const ENERGY: &str = "docs/specifications/science-contracts/contracts/SC-SNOWENERGY-001.md";
const FREEZE: &str = "docs/specifications/science-contracts/contracts/SC-SNOWFREEZE-001.md";
const INDEX: &str = "docs/specifications/science-contracts/index.md";
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
        "contract_version: 18",
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
        "contract_version: 137",
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
    let index = read(INDEX);
    assert!(package.contains("No land-surface, soil, frost, infiltration, runoff"));
    assert!(package.contains("No assignment of terminal unallocated energy"));
    assert!(package.contains("No physical seasonal efficacy"));
    assert!(index.contains("v18 admits OFE/lane snow--soil boundary custody"));
    assert!(index.contains("preserves v17 precipitation custody, v16 convergence"));
    assert!(index.contains("INV-101 remains evaluation-only"));
}
