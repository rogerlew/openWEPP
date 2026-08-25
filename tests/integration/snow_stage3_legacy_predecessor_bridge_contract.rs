use std::fs;

const CONTRACT: &str = "docs/specifications/science-contracts/contracts/SC-SNOWFREEZE-001.md";
const INDEX: &str = "docs/specifications/science-contracts/index.md";
const PACKAGE: &str = "docs/work-packages/20260806-snow-stage3-legacy-predecessor-bridge-reconciliation-001/package.md";
const PROTOCOL: &str = "docs/work-packages/20260806-snow-stage3-legacy-predecessor-bridge-reconciliation-001/artifacts/protocol-freeze.json";

fn read(path: &str) -> String {
    fs::read_to_string(path).unwrap_or_else(|error| panic!("read {path}: {error}"))
}

fn section<'a>(document: &'a str, start: &str, end: &str) -> &'a str {
    let start_index = document
        .find(start)
        .unwrap_or_else(|| panic!("missing section start {start}"));
    let remainder = &document[start_index..];
    let end_index = remainder
        .find(end)
        .unwrap_or_else(|| panic!("missing section end {end}"));
    &remainder[..end_index]
}

#[test]
fn v130_binds_forcing_matched_predecessor_reproduction() {
    let contract = read(CONTRACT);

    for required in [
        "contract_version: 137",
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

    assert!(index.contains("v136"), "{INDEX} missing v136");
    for required in [
        "exact S/F/Q forcing hash",
        "schema-v4 evidence to aggregate custody",
        "CoE ownership",
        "seasonal/terminal/cutover holds remain unchanged",
    ] {
        assert!(contract.contains(required), "{CONTRACT} missing {required}");
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

#[test]
fn v130_binding_surfaces_are_section_scoped_and_complete() {
    let contract = read(CONTRACT);
    let protocol = read(PROTOCOL);
    let invariant = section(&contract, "| INV-SNOWFREEZE-097", "### HPHYS0298");
    let guard = section(&contract, "## Invariant Guard Map", "## Symbol Alias Map");
    let producer = section(
        &contract,
        "## Producer Obligations",
        "## Consumer Obligations",
    );
    let consumer = section(
        &contract,
        "## Consumer Obligations",
        "## Boundary Disposition",
    );
    let boundary = section(
        &contract,
        "## Boundary Disposition",
        "## Tolerance and Numeric Notes",
    );
    let tolerance = section(
        &contract,
        "## Tolerance and Numeric Notes",
        "## Stage 3 Evaluation Shadow Authority Addendum",
    );
    let addendum = section(
        &contract,
        "## Stage 3 Forcing-Matched Predecessor Bridge Correction Addendum",
        "## Wet-Compaction Operand Authority And Duplicate-Alias Closure Addendum",
    );
    let exposure = section(&contract, "## Binding Exposure Index", "## Known Gaps");

    for (name, scoped, required) in [
        ("invariant", invariant, "INV-SNOWFREEZE-097"),
        ("guard", guard, "`INV-SNOWFREEZE-097`"),
        ("producer", producer, "OBL-SNOWFREEZE-P-070"),
        ("consumer", consumer, "OBL-SNOWFREEZE-C-012"),
        ("boundary", boundary, "INV-SNOWFREEZE-097"),
        ("tolerance", tolerance, "TOL-SNOWFREEZE-020"),
        ("exposure", exposure, "SNOWFREEZE-STAGE3-PREDECESSOR-BRIDGE"),
    ] {
        assert!(scoped.contains(required), "{name} missing {required}");
    }

    for required in [
        "exact forcing-hash join",
        "schema-v4 aggregate-only limit",
        "same-forcing all-35-WY-plus-median aggregate reconstruction",
        "paired disabled controls",
        "single-axis substitution closure",
        "Evidence hard-fail",
        "governance `HOLD`",
    ] {
        assert!(
            guard.contains(required) || boundary.contains(required),
            "guard/boundary missing {required}"
        );
    }

    for required in [
        "any paired water-year source",
        "paired-difference median fails",
        "TOL-SNOWFREEZE-020",
    ] {
        assert!(producer.contains(required), "producer missing {required}");
    }
    for required in [
        "canonical lane executes iff any WY E10-E00 exceeds",
        "paired-difference median exceeds 1e-7 MJ m^-2",
        "development lane uses the same WY-or-median rule for E11-E01",
        "execute both when both trigger",
    ] {
        assert!(protocol.contains(required), "protocol missing {required}");
    }
    for required in [
        "schema-v4 24-hour sum versus daily aggregate",
        "max(1e-6 J m^-2, 1e-12 * sum_abs_operands)",
        "1e-7 MJ m^-2",
    ] {
        assert!(tolerance.contains(required), "tolerance missing {required}");
    }

    for required in [
        "FIRST_DIVERGENCE_DAILY_RESET",
        "FIRST_DIVERGENCE_INITIAL_CONTROL_VOLUME_PROJECTION",
        "FIRST_DIVERGENCE_SURFACE_TERMS",
        "FIRST_DIVERGENCE_ACTIVE_INTERNAL_CONDUCTION",
        "FIRST_DIVERGENCE_STATE_APPLICATION_REMOVAL",
        "FIRST_DIVERGENCE_TRANSITION_PREPARATION_CADENCE",
        "FIRST_DIVERGENCE_MULTIPLE_OR_UNOBSERVED",
        "DAILY_RESET_CAUSAL",
        "INITIAL_CONTROL_VOLUME_PROJECTION_CAUSAL",
        "SURFACE_TERMS_CAUSAL",
        "ACTIVE_INTERNAL_CONDUCTION_CAUSAL",
        "STATE_APPLICATION_REMOVAL_CAUSAL",
        "TRANSITION_PREPARATION_CADENCE_CAUSAL",
        "Zero or multiple closing substitutions",
        "different first intervals/axes across triggering forcing lanes",
        "SOURCE_BY_FORCING_INTERACTION_DESCRIPTIVE",
    ] {
        assert!(addendum.contains(required), "addendum missing {required}");
    }
}
