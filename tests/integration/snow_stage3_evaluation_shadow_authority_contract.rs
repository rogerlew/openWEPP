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
        .find(|line| {
            line.starts_with(&format!("| `{symbol}` |"))
                || line.starts_with(&format!("| {symbol} |"))
        })
        .unwrap_or_else(|| panic!("{CONTRACT} missing table row for {symbol}"))
}

#[test]
fn v134_preserves_one_production_state_and_scopes_persistent_exception() {
    let contract = read(CONTRACT);

    for required in [
        "contract_id: SC-SNOWFREEZE-001",
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
        "geometry including `z_T/z_q/z_u/z_0,aero`",
        "independent initial-plus-flux-to-final mass, enthalpy, and liquid ledger closure",
        "typed, default-off request",
        "evaluator has exclusive custody",
        "Absent the sole named exception in `INV-SNOWFREEZE-100`",
    ] {
        assert!(contract.contains(required), "{CONTRACT} missing {required}");
    }

    let operator_row = table_row(&contract, "snow_evaluation_operator");
    assert_eq!(
        operator_row,
        "| `snow_evaluation_operator` | `enum` | Evaluation-only operator identity: `same_state_paired_carrier_v1`, `sequential_resolved_shadow_v1`, or `persistent_accumulation_shadow_v1`. Absence means no evaluation shadow. Unsupported or ambiguous values fail the evaluation request without changing production. | typed evaluation request | evaluation runner and evidence consumer only |"
    );

    let inv091_row = table_row(&contract, "INV-SNOWFREEZE-091");
    let exact_exception = "The sole exception is an evaluation-only object satisfying every custody and claim guard in `INV-SNOWFREEZE-094`; that exception does not weaken the one-authoritative-production-state rule.";
    assert_eq!(
        inv091_row.matches("exception").count(),
        2,
        "INV-SNOWFREEZE-091 must expose one sole exception and its non-weakening restatement"
    );
    assert!(inv091_row.contains(exact_exception));
    assert!(inv091_row.contains(&format!("{exact_exception} | hard-fail |")));
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
        "terminal/receiving-surface and physically interpreted persistence remain held",
        "by itself authorizes no runtime implementation or CoE ownership change",
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
fn roadmap_orders_observability_before_result_bearing_audit_and_persistent_mechanics() {
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
        .find("implemented the default-off persistent continuity experiment")
        .expect("roadmap must record completed persistent mechanics");

    assert!(authority < observability);
    assert!(observability < audit);
    assert!(audit < persistence);
}
