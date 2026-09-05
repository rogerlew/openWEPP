use std::fs;
use std::path::PathBuf;

const RECOVERY_PACKAGE: &str =
    "docs/work-packages/20260901-stage3-native-vegetation-laned-watershed-throughput-recovery-001";

fn root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
}

fn text(path: &str) -> String {
    fs::read_to_string(root().join(path))
        .unwrap_or_else(|error| panic!("expected readable {path}: {error}"))
}

fn normalized(path: &str) -> String {
    text(path).split_whitespace().collect::<Vec<_>>().join(" ")
}

#[test]
fn canonical_standard_prohibits_accretive_production_solver_dispatch() {
    let standard = normalized("docs/standards/numerical-solver-architecture.md");
    for required in [
        "Production kernels must not use accretive solver dispatch",
        "Each physically defined regime has one canonical solver",
        "It may not invoke an older solver",
        "Source and release history are the rollback mechanism",
        "A regime selector may not test whether another solver converged",
        "Bit-exact continuous-state or receipt fixed points require separate explicit science/numerical authority",
        "Current noncompliance quarantine",
    ] {
        assert!(standard.contains(required), "standard missing: {required}");
    }

    for required in [
        "one shared evaluation and runtime budget covers the complete algorithm",
        "every regime has exactly one production solver",
        "failure injection cannot reach an older/alternate solver",
        "no V58 or other successor/fallback may be added",
    ] {
        assert!(
            standard.contains(required),
            "closure rule missing: {required}"
        );
    }
}

#[test]
fn adr0044_is_accepted_registered_and_not_a_grandfather_clause() {
    let adr = normalized("docs/decisions/0044-prohibit-accretive-production-solver-dispatch.md");
    for required in [
        "**Status:** Accepted",
        "Each authoritative physical regime has exactly one canonical production solver",
        "A successor replaces and deletes the superseded production solver",
        "known noncompliance and is not grandfathered",
        "No V58 or equivalent successor is permitted",
    ] {
        assert!(adr.contains(required), "ADR-0044 missing: {required}");
    }

    let registry = text("docs/decisions/README.md");
    assert!(registry.contains("0044-prohibit-accretive-production-solver-dispatch.md"));
    assert!(registry.contains("ADR-0044 prohibits live historical-version"));
}

#[test]
fn repository_authoring_instructions_bind_solver_changes_to_the_standard() {
    let bindings = [
        ("AGENTS.md", "Production numerical solvers must not accrete"),
        (
            "crates/AGENTS.md",
            "docs/standards/numerical-solver-architecture.md",
        ),
        (
            "docs/work-packages/AGENTS.md",
            "docs/standards/numerical-solver-architecture.md",
        ),
        (
            "docs/specifications/science-contracts/AGENTS.md",
            "docs/standards/numerical-solver-architecture.md",
        ),
    ];

    for (path, required) in bindings {
        assert!(text(path).contains(required), "{path} missing: {required}");
    }
}

#[test]
fn stage3_chain_is_quarantined_for_replacement_not_accepted_as_architecture() {
    let package = normalized(&format!("{RECOVERY_PACKAGE}/package.md"));
    for required in [
        "Binding architecture rule: no accretive solvers",
        "This package applies accepted ADR-0044",
        "The package must replace the v33--v57 accretive solver architecture",
        "No V58",
    ] {
        assert!(
            package.contains(required),
            "recovery package missing: {required}"
        );
    }

    let kickoff = normalized(&format!("{RECOVERY_PACKAGE}/prompts/active/kickoff.md"));
    assert!(kickoff.contains("No accretive solver dispatch"));
    assert!(kickoff.contains("do not create V58"));

    let roadmap = text("docs/ROADMAP.md");
    assert!(roadmap.contains("0044-prohibit-accretive-production-solver-dispatch.md"));
    assert!(roadmap.contains("replace—not extend—the accretive v33–v57 solver dispatch"));
}

#[test]
fn stage3_canonical_replacement_authority_is_nonversioned_and_anti_accretive() {
    let contract =
        normalized("docs/specifications/science-contracts/contracts/SC-SNOWENERGY-001.md");
    for required in [
        "ADR-0044 Canonical Covered-Solver Amendment",
        "It is not version 58",
        "INV-SNOWENERGY-082",
        "TOL-SNOWENERGY-007",
        "at most eight authentic physical maps",
        "Coordinate finite differences",
        "same solver",
        "OBL-SNOWENERGY-C-050",
        "historical and superseded",
    ] {
        assert!(contract.contains(required), "contract missing: {required}");
    }

    for (path, required) in [
        (
            "docs/specifications/science-contracts/contracts/SC-LANDSURFACEENERGY-001.md",
            "INV-LANDSURFACEENERGY-152",
        ),
        (
            "docs/specifications/science-contracts/contracts/SC-COUPLEDTIME-001.md",
            "INV-COUPLEDTIME-027",
        ),
        (
            "docs/specifications/science-contracts/contracts/SC-SURFACELIQUID-001.md",
            "INV-SURFACELIQUID-032",
        ),
        (
            "docs/specifications/science-contracts/contracts/SC-RUNOFFPART-001.md",
            "INV-RUNOFFPART-034",
        ),
        (
            "docs/specifications/science-contracts/contracts/SC-OFEROUTE-001.md",
            "INV-OFEROUTE-015",
        ),
        (
            "docs/specifications/science-contracts/contracts/SC-WATBAL-001.md",
            "INV-WATBAL-105",
        ),
    ] {
        assert!(text(path).contains(required), "{path} missing {required}");
    }
}
