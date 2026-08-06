use std::fs;

const CONTRACT: &str = "docs/specifications/science-contracts/contracts/SC-SNOWFREEZE-001.md";
const INDEX: &str = "docs/specifications/science-contracts/index.md";
const PACKAGE: &str =
    "docs/work-packages/20260806-snow-stage3-shadow-solver-extraction-and-observability-001/package.md";

fn read(path: &str) -> String {
    fs::read_to_string(path).unwrap_or_else(|error| panic!("read {path}: {error}"))
}

#[test]
fn v128_admits_only_bounded_typed_shadow_observability() {
    let contract = read(CONTRACT);

    for required in [
        "contract_version: 128",
        "REF-SNOWFREEZE-STAGE3-SHADOW-OBSERVABILITY",
        "INV-SNOWFREEZE-095",
        "OBL-SNOWFREEZE-P-068",
        "OBL-SNOWFREEZE-C-010",
        "absent-by-default typed request",
        "same_state_paired_carrier_v1",
        "sequential_resolved_shadow_v1",
        "stage3_carrier_pair_v1",
        "stage3_surface_energy_v1",
        "stage3_complete_carrier_v1",
        "bounded_response_experiment",
        "Enabled evaluation emits internal schema v5 only",
        "authoritative state/ledgers/outputs remain exact",
        "Internal conduction may not be relabeled snow-ground flux",
        "Turbulent primitive failures preserve their typed meteorology source",
        "move `runoff_reconciliation.rs` below 3,000 lines",
    ] {
        assert!(contract.contains(required), "{CONTRACT} missing {required}");
    }
}

#[test]
fn v128_binds_real_consumer_and_retains_campaign_holds() {
    let contract = read(CONTRACT);

    for required in [
        "real internal JSONL consumer must read every new operand",
        "independently reject production/adjacent aliases",
        "requested/evaluated support and coverage",
        "consumer may read schema-v5 evidence as state or authority",
        "cross-interval persistence",
        "complete same-substep phase/liquid chronology",
        "seasonal/terminal claims",
        "production consumers",
        "cutover remain held",
    ] {
        assert!(contract.contains(required), "{CONTRACT} missing {required}");
    }
}

#[test]
fn lifecycle_index_and_package_name_the_same_realization() {
    let index = read(INDEX);
    let package = read(PACKAGE);

    for required in [
        "SC-SNOWFREEZE-001",
        "v128",
        "schema-v5 observability",
    ] {
        assert!(index.contains(required), "{INDEX} missing {required}");
    }
    for required in [
        "Stage 3 Shadow Solver Extraction And Observability",
        "schema-v5",
        "CoE remains the sole current melt owner",
        "No equation, coefficient, threshold",
    ] {
        assert!(package.contains(required), "{PACKAGE} missing {required}");
    }
}
