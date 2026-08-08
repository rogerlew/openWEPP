use std::fs;

const CONTRACT: &str =
    "docs/specifications/science-contracts/contracts/SC-LANDSURFACEENERGY-001.md";
const INDEX: &str = "docs/specifications/science-contracts/index.md";

fn read(path: &str) -> String {
    fs::read_to_string(path).unwrap_or_else(|error| panic!("read {path}: {error}"))
}

fn row<'a>(contract: &'a str, key: &str) -> &'a str {
    contract
        .lines()
        .find(|line| line.starts_with(&format!("| `{key}` |")))
        .unwrap_or_else(|| panic!("{CONTRACT} missing row {key}"))
}

#[test]
fn contract_binds_control_volume_closure_and_exact_one_custody() {
    let contract = read(CONTRACT);
    for required in [
        "contract_id: SC-LANDSURFACEENERGY-001",
        "E_s,1 - E_s,0 = dt * (R_sw + R_lw + H + LE + Q_p + Q_runon - Q_inf - Q_runoff + G)",
        "M_l,1 - M_l,0 = m_p + m_runon - m_evap - m_inf - m_runoff",
        "LE * dt = -L_v(T_s) * m_evap",
        "INV-LANDSURFACEENERGY-010",
        "INV-LANDSURFACEENERGY-011",
        "INV-LANDSURFACEENERGY-012",
        "INV-LANDSURFACEENERGY-013",
        "INV-LANDSURFACEENERGY-014",
        "INV-LANDSURFACEENERGY-015",
        "| `A` | `m^2` |",
        "| `dt` | `s` |",
        "| `T_s` | `K` |",
        "| `R_sw`, `R_lw` | `W m^-2` |",
        "TOL-LANDSURFACEENERGY-001",
        "TOL-LANDSURFACEENERGY-002",
        "`epsilon_E`, `epsilon_M` | `J m^-2`, `kg m^-2`",
        "`rho_E`, `rho_M` | `dimensionless`",
        "|epsilon_E| <= max(a_E, rho_E*sum_abs_energy_operands)",
        "|epsilon_M| <=\n  max(a_M, rho_M*sum_abs_mass_operands)",
        "validate before mutation and commit energy and\n  water state atomically",
        "surface records\n   `G`, while the sole soil/frost consumer records `-G`",
        "runon to `Q_runon`, infiltration to `Q_inf`, and\n   runoff to `Q_runoff`",
    ] {
        assert!(contract.contains(required), "{CONTRACT} missing {required}");
    }
}

#[test]
fn contract_preserves_adjacent_owners_and_rejects_terminal_payload() {
    let contract = read(CONTRACT);
    for required in [
        "SC-CLIMATE-001",
        "SC-EVAP-001",
        "SC-WATBAL-001",
        "SC-RUNOFFPART-001",
        "SC-SOIL-001",
        "SC-SUBHYD-001",
        "INV-SNOWENERGY-034",
        "Branch priority is `snow_terminal` rejection, then `snow_present` delegation",
        "Schema-v8 terminal liquid, energy, and time are censored",
        "must not mutate ET,\nrunoff, infiltration, soil, or frost",
        "a real scheduler consumer must prove",
        "authoritative upstream state reports zero represented\nsnow at the interval start",
        "legacy `surtmp(hour)` / `Thra` | not an alias of `T_s` in v1",
        "future named `degC <-> K` conversion and atomic cutover required",
    ] {
        assert!(contract.contains(required), "{CONTRACT} missing {required}");
    }
}

#[test]
fn authority_gaps_block_surrogate_or_production_claims() {
    let contract = read(CONTRACT);
    for required in [
        "science_implementation_status = NOT_IMPLEMENTED",
        "calibration_evidence_status = NOT_APPLICABLE",
        "identifiability_status = NOT_APPLICABLE",
        "AUTHORITY_MISSING",
        "NON_PROMOTABLE",
        "GAP-LANDSURFACEENERGY-001",
        "GAP-LANDSURFACEENERGY-006",
        "No production implementation is promotable",
        "provisional, surrogate, heuristic, or comparator-targeted physics",
        "Real-consumer proof remains intentionally unsatisfied",
        "Exact future precondition",
        "Exact future\npostcondition",
    ] {
        assert!(contract.contains(required), "{CONTRACT} missing {required}");
    }
}

#[test]
fn typed_failures_state_surface_and_guard_map_are_complete() {
    let contract = read(CONTRACT);
    for required in [
        "Required future inputs are one state identity",
        "Required outputs are end state",
        "Mutated state is limited to `E_s` and `M_l`",
        "### Invariant Guard Map",
        "| Invariant ID | Enforcement path | Guard class | Failure behavior | Evidence artifact |",
        "LSEB-E-001",
        "LSEB-E-010",
        "LSEB-E-011",
        "LSEB-E-012",
        "LSEB-E-013",
        "LSEB-E-014",
        "LSEB-E-015",
        "LSEB-E-020",
        "LSEB-E-021",
        "`git show\ndac3c950d8b16cc73774bf5ce2e7e11f80baac70:<path>`",
    ] {
        assert!(contract.contains(required), "{CONTRACT} missing {required}");
    }

    let guard_map = contract
        .split("### Invariant Guard Map")
        .nth(1)
        .expect("guard map section")
        .split("## Producer Obligations")
        .next()
        .expect("guard map terminator");
    for id in [
        "001", "002", "010", "011", "012", "013", "014", "015", "020", "021", "022", "030", "031",
        "032", "040", "041",
    ] {
        let invariant = format!("`INV-LANDSURFACEENERGY-{id}`");
        assert!(
            guard_map.contains(&format!("| {invariant} |")),
            "guard map missing {invariant}"
        );
    }

    for (symbol, units) in [
        ("A", "`m^2`"),
        ("dt", "`s`"),
        ("T_s", "`K`"),
        ("E_s,0`, `E_s,1", "`J m^-2`"),
        ("M_l,0`, `M_l,1", "`kg m^-2`"),
        ("R_sw`, `R_lw", "`W m^-2`"),
        ("H", "`W m^-2`"),
        ("LE", "`W m^-2`"),
        ("Q_p`, `Q_runon", "`W m^-2`"),
        ("Q_inf`, `Q_runoff", "`W m^-2`"),
        ("G", "`W m^-2`"),
        ("m_p`, `m_runon", "`kg m^-2`"),
        ("m_evap", "`kg m^-2`"),
        ("m_inf`, `m_runoff", "`kg m^-2`"),
    ] {
        assert!(
            row(&contract, symbol).contains(units),
            "{symbol} units changed"
        );
    }

    for mapping in [
        "non-finite/unit/domain failure | reject before mutation | `LSEB-E-001`",
        "duplicate/missing component lineage | reject | `LSEB-E-010`",
        "energy or water closure exceeds tolerance | reject atomically | `LSEB-E-011` / `LSEB-E-012`",
        "latent mass-energy mismatch | reject | `LSEB-E-013`",
        "ground-flux dual ownership | reject | `LSEB-E-014`",
        "negative end storage beyond tolerance | reject; no clamp/default | `LSEB-E-015`",
        "snow present | delegate exclusively to snow owner; no LSE mutation | `LSEB-E-020` on attempted dual evaluation",
        "snow terminal/censored payload present | reject; there is no v1 recipient | `LSEB-E-021`",
        "future branch selector plus poison vectors | runtime | `LSEB-E-020/021`; currently `HOLD`",
    ] {
        assert!(
            contract.contains(mapping),
            "typed mapping changed: {mapping}"
        );
    }
}

#[test]
fn schema_sections_test_vectors_and_registry_are_bound() {
    let contract = read(CONTRACT);
    let index = read(INDEX);
    for heading in [
        "## Purpose",
        "## Scientific Scope and Explicit Out-of-Scope Boundaries",
        "## Authority Anchors with Top-Down Citations",
        "## Variables and Units Using Canonical Symbols First",
        "## Algorithm State Surfaces",
        "## Algorithm Specification with Step Sequence",
        "## Branch and Guard Table",
        "## Invariants and Invariant Guard Map",
        "## Producer Obligations and Consumer Obligations",
        "## Symbol Alias Map",
        "## Constants and Parameters with Provenance Anchors",
        "## Unit-Governance Map",
        "## Tolerance and Numeric Notes",
        "## Calibration and Identifiability",
        "## Test-Vector Obligations",
        "## Binding Exposure Index",
        "## Gap Register and Promotability Labels",
        "## Change Log",
    ] {
        assert!(contract.contains(heading), "{CONTRACT} missing {heading}");
    }
    for poison in [
        "omit and duplicate precipitation water and",
        "runon water and heat",
        "infiltration water and heat",
        "runoff water and heat",
        "latent",
        "sensible",
        "shortwave",
        "longwave",
        "ground heat",
        "storage change",
        "evaporation",
        "infiltration",
        "runoff",
    ] {
        assert!(contract.contains(poison), "{CONTRACT} missing {poison}");
    }
    assert!(index.contains("| `SC-LANDSURFACEENERGY-001` |"));

    for expected in [
        "| all-zero flux | unchanged energy and water state with positive `dt` | `INV-010/011` |",
        "| terminal schema-v8 payload | reject with no mutation | `INV-021`, `LSEB-E-021` |",
        "| ground sign reversal | surface `G` equals soil/frost `-G` | `INV-013`, `LSEB-E-014` |",
        "| all-distinct operands | independently reconstructed `epsilon_E/epsilon_M` pass | `INV-010/011` |",
    ] {
        assert!(
            contract.contains(expected),
            "vector mapping changed: {expected}"
        );
    }

    for gap in ["001", "002", "003", "004", "005", "006"] {
        let line = row(&contract, &format!("GAP-LANDSURFACEENERGY-{gap}"));
        assert!(
            line.contains("`NON_PROMOTABLE`"),
            "gap {gap} lost NON_PROMOTABLE"
        );
        if gap != "004" {
            assert!(
                line.contains("`AUTHORITY_MISSING`"),
                "gap {gap} lost AUTHORITY_MISSING"
            );
        } else {
            assert!(
                line.contains("`IMPLEMENTATION_MISSING`"),
                "gap 004 lost IMPLEMENTATION_MISSING"
            );
        }
    }
}
