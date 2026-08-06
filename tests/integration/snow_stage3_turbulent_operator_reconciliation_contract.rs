use std::fs;

const CONTRACT: &str = "docs/specifications/science-contracts/contracts/SC-SNOWFREEZE-001.md";
const INDEX: &str = "docs/specifications/science-contracts/index.md";
const PACKAGE: &str = "docs/work-packages/20260806-snow-stage3-turbulent-carrier-lineage-and-operator-reconciliation-001/package.md";
const PROTOCOL: &str = "docs/work-packages/20260806-snow-stage3-turbulent-carrier-lineage-and-operator-reconciliation-001/artifacts/protocol-freeze.json";

fn read(path: &str) -> String {
    fs::read_to_string(path).unwrap_or_else(|error| panic!("read {path}: {error}"))
}

#[test]
fn v129_admits_only_additive_default_off_operator_reconciliation() {
    let contract = read(CONTRACT);

    for required in [
        "contract_version: 129",
        "REF-SNOWFREEZE-STAGE3-OPERATOR-RECONCILIATION",
        "INV-SNOWFREEZE-096",
        "OBL-SNOWFREEZE-P-069",
        "OBL-SNOWFREEZE-C-011",
        "one boxed, enabled-only reconciliation companion",
        "internal schema-v6 row",
        "share one private Monin-Obukhov solver",
        "duplicated formulas, coefficients, convergence logic, or arithmetic order are prohibited",
        "Historical schema-v5 meaning remains unchanged",
        "disabled/default rows remain exact schema v4 with no companion allocation",
    ] {
        assert!(contract.contains(required), "{CONTRACT} missing {required}");
    }
}

#[test]
fn v129_binds_exact_tuple_lineage_and_independent_reconstruction() {
    let contract = read(CONTRACT);

    for required in [
        "ordered duration-tagged tuples",
        "typed applicability instead of numeric-zero N/A",
        "`z_T/z_q/z_u/z_0,aero`",
        "solver options/status, stability length/corrections/log factors",
        "explicit albedo source/model/state",
        "active/lower cold-content change",
        "mixed sequential complete = comparable external subset + active internal conduction",
        "`F-S` projection and `Q-F` evolution effects",
        "without importing producer calculation helpers",
        "No production or public consumer may read schema-v6 evidence as state or",
    ] {
        assert!(contract.contains(required), "{CONTRACT} missing {required}");
    }
}

#[test]
fn v129_retains_production_and_claim_holds() {
    let contract = read(CONTRACT);
    let index = read(INDEX);

    for required in [
        "no result may change equations, constants, thresholds, defaults, ownership, state, ledgers, fixtures, observations, or public outputs",
        "claim seasonal physical chronology, terminal closure, validation, promotion, CoE retirement, or cutover",
        "CoE ownership",
        "seasonal/terminal/cutover holds remain unchanged",
    ] {
        assert!(contract.contains(required), "{CONTRACT} missing {required}");
    }
    for required in [
        "v129",
        "schema-v6 turbulent/state lineage",
        "historical schema-v5 semantics",
        "CoE ownership",
    ] {
        assert!(index.contains(required), "{INDEX} missing {required}");
    }
}

#[test]
fn admitted_protocol_freezes_support_bridge_and_decision_rules() {
    let package = read(PACKAGE);
    let protocol = read(PROTOCOL);

    for required in [
        "result-blind protocol admitted PASS/PASS",
        "same_state_paired_carrier_v1",
        "sequential_resolved_shadow_v1",
        "frozen_active_projection_reference",
        "CoE remains the sole authoritative production melt",
    ] {
        assert!(package.contains(required), "{PACKAGE} missing {required}");
    }
    for required in [
        "frozen_result_blind_admitted_pass_pass",
        "common_support_integration",
        "legacy_sequential_bridge",
        "ordered_three_way_support",
        "delta_projection=F-S and delta_evolution=Q-F",
        "INITIAL_CONTROL_VOLUME_PROJECTION_RECONCILES_SIGN_CONTRADICTION",
        "STATE_EVOLUTION_RECONCILES_SIGN_CONTRADICTION",
        "not promotion or cutover",
        "zero_alias_prohibited",
    ] {
        assert!(protocol.contains(required), "{PROTOCOL} missing {required}");
    }
}
