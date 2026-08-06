use std::fs;

const CONTRACT: &str = "docs/specifications/science-contracts/contracts/SC-SNOWFREEZE-001.md";
const ROADMAP: &str = "docs/planning/snow-surface-energy-balance-roadmap.md";

fn read(path: &str) -> String {
    fs::read_to_string(path).unwrap_or_else(|error| panic!("read {path}: {error}"))
}

#[test]
fn v127_preserves_one_production_state_and_admits_only_two_shadow_operators() {
    let contract = read(CONTRACT);

    for required in [
        "contract_version: 127",
        "INV-SNOWFREEZE-091",
        "INV-SNOWFREEZE-094",
        "OBL-SNOWFREEZE-P-067",
        "OBL-SNOWFREEZE-C-009",
        "production snow must retain one authoritative calculation/state-mutation path",
        "undeclared or production-readable second mass state",
        "sole exception is an evaluation-only object satisfying every custody and claim guard",
        "same_state_paired_carrier_v1",
        "sequential_resolved_shadow_v1",
        "typed, default-off request",
        "evaluator has exclusive custody",
        "cannot seed another shadow or production interval",
    ] {
        assert!(contract.contains(required), "{CONTRACT} missing {required}");
    }
}

#[test]
fn authority_rejects_production_reachability_and_overbroad_claims() {
    let contract = read(CONTRACT);

    for required in [
        "must not alias, mutate, replace, route into, publish as, calibrate, select, or otherwise reach production state",
        "must reject `snow_evaluation_shadow_state`",
        "Producer-only",
        "cannot close negative consumer reachability",
        "Daily or independently reinitialized shadows cannot support accumulation-season",
        "Persistence and terminal/receiving-surface authority require separate amendments",
        "authorizes no runtime implementation or CoE ownership change",
    ] {
        assert!(contract.contains(required), "{CONTRACT} missing {required}");
    }
}

#[test]
fn roadmap_orders_observability_before_result_bearing_audit_and_persistence() {
    let roadmap = read(ROADMAP);
    let authority = roadmap
        .find("scope a declared evaluation shadow under `INV-SNOWFREEZE-091`")
        .expect("roadmap must name evaluation-shadow authority first");
    let observability = roadmap
        .find("mechanically extract the shadow solver")
        .expect("roadmap must name extraction and observability second");
    let audit = roadmap
        .find("run a frozen paired-window per-term audit")
        .expect("roadmap must name the four-site audit third");
    let persistence = roadmap
        .find("admit and run persistent accumulation-season shadow state only if that")
        .expect("roadmap must keep persistence conditional");

    assert!(authority < observability);
    assert!(observability < audit);
    assert!(audit < persistence);
}
