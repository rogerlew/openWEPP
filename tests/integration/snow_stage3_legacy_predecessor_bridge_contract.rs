use std::fs;

const CONTRACT: &str = "docs/specifications/science-contracts/contracts/SC-SNOWFREEZE-001.md";
const INDEX: &str = "docs/specifications/science-contracts/index.md";
const PACKAGE: &str = "docs/work-packages/20260806-snow-stage3-legacy-predecessor-bridge-reconciliation-001/package.md";
const PROTOCOL: &str = "docs/work-packages/20260806-snow-stage3-legacy-predecessor-bridge-reconciliation-001/artifacts/protocol-freeze.json";

fn read(path: &str) -> String {
    fs::read_to_string(path).unwrap_or_else(|error| panic!("read {path}: {error}"))
}

#[test]
fn v130_binds_forcing_matched_predecessor_reproduction() {
    let contract = read(CONTRACT);

    for required in [
        "contract_version: 130",
        "REF-SNOWFREEZE-STAGE3-PREDECESSOR-BRIDGE",
        "INV-SNOWFREEZE-097",
        "OBL-SNOWFREEZE-P-070",
        "OBL-SNOWFREEZE-C-012",
        "TOL-SNOWFREEZE-020",
        "parameterized by the exact forcing SHA-256 of the `S/F/Q` estimand",
        "Development-forcing `S/F/Q` therefore requires current/development E11 versus old/development E01 closure",
        "current/canonical E10 versus old/canonical E00 cannot substitute",
        "every paired water-year difference",
        "Python `statistics.median`",
    ] {
        assert!(contract.contains(required), "{CONTRACT} missing {required}");
    }
}

#[test]
fn v130_limits_schema_v4_to_aggregate_custody() {
    let contract = read(CONTRACT);

    for required in [
        "Schema-v4 arms prove aggregate custody only",
        "stage3_shadow_hourly_complete_energy_j_m2",
        "stage3_shadow_complete_energy_j_m2",
        "cannot establish primitive terms, support, mass/cold closure",
        "active/lower conduction cancellation",
        "causal source transition",
        "legacy_sequential_complete_j_m2",
        "complete_external_flux_w_m2 * duration_seconds",
        "internal_active_lower_conduction_j_m2",
        "Cross-schema factorial decomposition is aggregate custody accounting, not mechanistic explanation",
    ] {
        assert!(contract.contains(required), "{CONTRACT} missing {required}");
    }
}

#[test]
fn v130_freezes_factorial_and_causal_boundaries() {
    let contract = read(CONTRACT);
    let protocol = read(PROTOCOL);

    for required in [
        "source_canonical=E10-E00",
        "source_development=E11-E01",
        "interaction=(E11-E10)-(E01-E00)",
        "FIRST_DIVERGENCE_DAILY_RESET",
        "FIRST_DIVERGENCE_TRANSITION_PREPARATION_CADENCE",
        "DAILY_RESET_CAUSAL",
        "TRANSITION_PREPARATION_CADENCE_CAUSAL",
        "MULTIFACTOR_OR_UNOBSERVED_PREDECESSOR_BOUNDARY",
        "SOURCE_BY_FORCING_INTERACTION_DESCRIPTIVE",
    ] {
        assert!(contract.contains(required), "{CONTRACT} missing {required}");
    }
    for required in [
        "source_canonical",
        "E10-E00",
        "source_development",
        "E11-E01",
        "interaction",
        "(E11-E10)-(E01-E00)",
        "FIRST_DIVERGENCE_DAILY_RESET",
        "FIRST_DIVERGENCE_TRANSITION_PREPARATION_CADENCE",
        "DAILY_RESET_CAUSAL",
        "TRANSITION_PREPARATION_CADENCE_CAUSAL",
        "MULTIFACTOR_OR_UNOBSERVED_PREDECESSOR_BOUNDARY",
        "SOURCE_BY_FORCING_INTERACTION_DESCRIPTIVE",
    ] {
        assert!(protocol.contains(required), "{PROTOCOL} missing {required}");
    }

    for required in [
        "behavior-neutrality against both uninstrumented endpoints",
        "exclusion of every alternative",
        "Zero or multiple closing substitutions",
    ] {
        assert!(contract.contains(required), "{CONTRACT} missing {required}");
    }
}

#[test]
fn v130_protocol_and_registry_retain_claim_limits() {
    let contract = read(CONTRACT);
    let index = read(INDEX);
    let package = read(PACKAGE);
    let protocol = read(PROTOCOL);

    for required in [
        "v130",
        "exact S/F/Q forcing hash",
        "schema-v4 evidence to aggregate custody",
        "CoE ownership",
        "seasonal/terminal/cutover holds remain unchanged",
    ] {
        assert!(index.contains(required), "{INDEX} missing {required}");
    }
    for required in [
        "CoE remains the sole production melt owner",
        "canonical S/F/Q class",
        "separately produces canonical S, F, and Q",
        "Missing package tools/tests, incomplete checkpointing",
        "instrumentation cannot justify HOLD",
    ] {
        assert!(package.contains(required), "{PACKAGE} missing {required}");
    }
    for required in [
        "prospectively_frozen_before_result_execution",
        "v4_aggregate_custody_only",
        "current/development and current/canonical each prove legacy selector equals explicit sequential_resolved_shadow_v1",
        "HISTORICAL_BINARY_NOT_RETAINED",
    ] {
        assert!(protocol.contains(required), "{PROTOCOL} missing {required}");
    }
    assert!(contract.contains("This correction changes no equation, constant, selector, default, state, output, ownership, or promotion status"));
}
