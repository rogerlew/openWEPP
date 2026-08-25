use std::fs;

const CONTRACT: &str = "docs/specifications/science-contracts/contracts/SC-SNOWFREEZE-001.md";
const INDEX: &str = "docs/specifications/science-contracts/index.md";
const PACKAGE: &str =
    "docs/work-packages/20260807-snow-stage3-persistent-accumulation-shadow-001/package.md";

fn read(path: &str) -> String {
    fs::read_to_string(path).unwrap_or_else(|error| panic!("read {path}: {error}"))
}

#[test]
fn v134_admits_exactly_one_mechanics_only_persistent_operator() {
    let contract = read(CONTRACT);
    for required in [
        "contract_version: 137",
        "INV-SNOWFREEZE-100",
        "OBL-SNOWFREEZE-P-072",
        "OBL-SNOWFREEZE-C-014",
        "TOL-SNOWFREEZE-021",
        "persistent_accumulation_shadow_v1",
        "persistent state-continuity experiment",
        "an error commits nothing",
        "Complete disappearance commits a dormant empty state",
        "exact uninterrupted-versus-restore equivalence",
        "Enabled-only internal schema v7",
        "schema v4 disabled rows and historical v5/v6 meanings remain exact",
    ] {
        assert!(contract.contains(required), "{CONTRACT} missing {required}");
    }
    assert!(
        read(INDEX).contains("v136 admits a separate default-off terminal receiver chronology")
    );
    assert!(read(PACKAGE).contains("Wind\nexposure"));
}

#[test]
fn legacy_bounded_operators_and_physical_claim_holds_remain_explicit() {
    let contract = read(CONTRACT);
    for required in [
        "Absent the sole named exception in `INV-SNOWFREEZE-100`",
        "the two bounded `INV-SNOWFREEZE-094` operators",
        "Physically interpreted persistence requires every canonical-site",
        "may separately admit isolated synthetic/internal continuity mechanics",
        "Liquid or terminal energy without a resolved recipient is explicit censored diagnostic output",
        "prohibit physical accumulation-season, peak-SWE/date, meltout-efficacy, validation, promotion, ownership, or cutover claims",
    ] {
        assert!(contract.contains(required), "{CONTRACT} missing {required}");
    }
}

#[test]
fn custody_is_atomic_ordered_and_production_isolated() {
    let contract = read(CONTRACT);
    for required in [
        "one private state per direct-runner lane",
        "strictly increasing interval chronology",
        "later intervals start exclusively from the prior committed shadow state",
        "Typed hourly snowfall is credited exactly once",
        "External liquid is credited exactly once to the liquid ledger and never to ice",
        "reject unknown fields, duplicate serialized fields, nonfinite values, fingerprint mismatch, wrong lane, and caller-declared out-of-order restore",
        "cannot reach production state, CoE, frost, routing, defaults, fixtures, observations, calibration, WAT/HBP/PASS, or public output",
    ] {
        assert!(contract.contains(required), "{CONTRACT} missing {required}");
    }
}
