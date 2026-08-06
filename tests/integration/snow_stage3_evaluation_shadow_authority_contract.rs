use std::fs;

const CONTRACT: &str = "docs/specifications/science-contracts/contracts/SC-SNOWFREEZE-001.md";
const ROADMAP: &str = "docs/planning/snow-surface-energy-balance-roadmap.md";
const REPORT: &str = "assurance/v2/reports/snow-and-frozen-soil-process-evaluation/report.yaml";
const REVIEW_LOCK: &str =
    "assurance/v2/reports/snow-and-frozen-soil-process-evaluation/review.lock.json";

fn read(path: &str) -> String {
    fs::read_to_string(path).unwrap_or_else(|error| panic!("read {path}: {error}"))
}

fn table_row<'a>(contract: &'a str, symbol: &str) -> &'a str {
    contract
        .lines()
        .find(|line| line.starts_with(&format!("| `{symbol}` |")))
        .unwrap_or_else(|| panic!("{CONTRACT} missing table row for {symbol}"))
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
        "exactly two stable named/versioned carrier arms",
        "geometry including `z_T/z_q/z_u/z_0`",
        "independent initial-plus-flux-to-final mass, enthalpy, and liquid ledger closure",
        "typed, default-off request",
        "evaluator has exclusive custody",
        "cannot seed another shadow or production interval",
    ] {
        assert!(contract.contains(required), "{CONTRACT} missing {required}");
    }

    let operator_row = table_row(&contract, "snow_evaluation_operator");
    assert_eq!(operator_row.matches("_v1`").count(), 2);
    assert_eq!(
        operator_row.matches("same_state_paired_carrier_v1").count(),
        1
    );
    assert_eq!(
        operator_row
            .matches("sequential_resolved_shadow_v1")
            .count(),
        1
    );
    assert_eq!(
        contract
            .matches("sole exception is an evaluation-only object satisfying every custody and claim guard")
            .count(),
        1
    );
    for forbidden in ["generic diagnostic state", "arbitrary diagnostic state"] {
        assert!(!contract.to_lowercase().contains(forbidden));
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
        "third/generic-operator rejection",
        "otherwise the result is only a bounded response experiment",
        "Daily or independently reinitialized shadows cannot support accumulation-season",
        "Persistence and terminal/receiving-surface authority require separate amendments",
        "authorizes no runtime implementation or CoE ownership change",
    ] {
        assert!(contract.contains(required), "{CONTRACT} missing {required}");
    }
}

#[test]
fn draft_assurance_subject_tracks_v127_without_review_or_approval_authority() {
    let report = read(REPORT);
    let review_lock = read(REVIEW_LOCK);

    for required in [
        "lifecycle: DRAFT",
        "openwepp:SC-SNOWFREEZE-001:v127",
        "v7/v127",
    ] {
        assert!(report.contains(required), "{REPORT} missing {required}");
    }
    for required in [
        r#""lifecycle": "DRAFT""#,
        r#""event_ids": []"#,
        r#""pre_steward_approval_root": null"#,
        r#""approval_lock_root": null"#,
        r#""realization_root": null"#,
        r#""release_transfer_root": null"#,
    ] {
        assert!(
            review_lock.contains(required),
            "{REVIEW_LOCK} missing {required}"
        );
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
