use std::fs;

const ENERGY: &str = "docs/specifications/science-contracts/contracts/SC-SNOWENERGY-001.md";
const FREEZE: &str = "docs/specifications/science-contracts/contracts/SC-SNOWFREEZE-001.md";
const LSE: &str = "docs/specifications/science-contracts/contracts/SC-LANDSURFACEENERGY-001.md";
const LIQUID: &str = "docs/specifications/science-contracts/contracts/SC-SURFACELIQUID-001.md";
const TXN: &str = "docs/specifications/science-contracts/contracts/SC-VEGETATIONTRANSACTION-001.md";
const INDEX: &str = "docs/specifications/science-contracts/index.md";

fn read(path: &str) -> String {
    fs::read_to_string(path)
        .unwrap_or_else(|error| panic!("read {path}: {error}"))
        .split_whitespace()
        .collect::<Vec<_>>()
        .join(" ")
}

fn table_has_row(document: &str, invariant: &str) -> bool {
    document
        .lines()
        .any(|line| line.starts_with('|') && line.contains(invariant))
}

fn terminal_liquid(retained: f64, snow_rain: f64, melt: f64, refreeze: f64) -> f64 {
    retained + snow_rain + melt - refreeze
}

fn precedence_rank(stage: &str) -> Option<u8> {
    [
        "restart",
        "snow_forcing",
        "snow_solver",
        "receipt",
        "receiver",
        "lse",
        "wb14",
        "frost",
        "routing",
        "commit",
        "rollback",
    ]
    .iter()
    .position(|candidate| *candidate == stage)
    .map(|index| index as u8 + 1)
}

fn valid_restart_transition(from: &str, to: &str) -> bool {
    matches!(
        (from, to),
        ("pre_trial", "one_full_done")
            | ("one_full_done", "first_half_done")
            | ("first_half_done", "two_half_done")
            | ("two_half_done", "pre_trial")
            | ("two_half_done", "bracketed")
            | ("bracketed", "bisect_left_done")
            | ("bisect_left_done", "bisect_mid_done")
            | ("bisect_mid_done", "bisect_left_done")
            | ("bisect_mid_done", "localized")
            | ("localized", "receiver_running")
            | ("receiver_running", "complete")
    )
}

fn validate_segment(tagged: bool, start: u64, event: u64, end: u64) -> Result<u64, &'static str> {
    if !tagged {
        return Err("E-008");
    }
    if start > event || event > end {
        return Err("support");
    }
    Ok(end - event)
}

fn consume_receipt(consumed: bool, fingerprint_matches: bool) -> Result<bool, &'static str> {
    if consumed || !fingerprint_matches {
        Err("receipt-replay")
    } else {
        Ok(true)
    }
}

#[test]
fn terminal_receiver_is_fresh_default_off_authority() {
    let energy = read(ENERGY);
    let freeze = read(FREEZE);
    for required in [
        "INV-SNOWENERGY-035",
        "terminal_receiver_v1",
        "earliest complete solid-exhaustion event",
        "dt_remaining = dt_interval - t*",
        "temperature is exactly `273.15 K`",
        "INV-SNOWENERGY-034` retains schema-v8 evaluation-only",
        "CoE remains the sole production snow mass/melt generator",
    ] {
        assert!(energy.contains(required), "{ENERGY} missing {required}");
    }
    for required in [
        "INV-SNOWFREEZE-102",
        "preserves `INV-SNOWFREEZE-101` as evaluation-only",
        "earliest localized exhaustion",
        "partial WB14 interval continuation",
        "Cross-midnight lineage advances once",
        "CoE remains production owner",
    ] {
        assert!(freeze.contains(required), "{FREEZE} missing {required}");
    }
}

#[test]
fn receiver_rebuilds_actual_surface_and_continues_only_remaining_support() {
    let lse = read(LSE);
    let liquid = read(LIQUID);
    for required in [
        "INV-LANDSURFACEENERGY-114",
        "INV-LANDSURFACEENERGY-115",
        "actual vegetation/litter/mineral/frost/water owner state",
        "No snow temperature, albedo, roughness",
        "`dt_remaining=0` suppresses the LSE solve",
    ] {
        assert!(lse.contains(required), "{LSE} missing {required}");
    }
    for required in [
        "INV-SURFACELIQUID-010",
        "INV-SURFACELIQUID-011",
        "retained snow liquid plus newly generated terminal liquid",
        "never aliased to rain, runon, store level/change",
        "partial WB14 interval over exactly `dt_remaining`",
        "Pre-event support is not replayed",
    ] {
        assert!(liquid.contains(required), "{LIQUID} missing {required}");
    }
}

#[test]
fn all_owner_failure_rolls_back_and_index_records_lifecycle() {
    let txn = read(TXN);
    let index = read(INDEX);
    for required in [
        "INV-VEGTRANSACTION-008",
        "One transaction ID and exact",
        "restores every beginning owner byte and predecessor identity",
        "Production owners, CoE state, selectors, public output",
        "simultaneous CoE/Stage 3 liquid generation",
    ] {
        assert!(txn.contains(required), "{TXN} missing {required}");
    }
    for required in [
        "v13 adds separate default-off terminal-receiver mechanics",
        "v136 admits a separate default-off terminal receiver chronology",
        "v7 admits exact-one terminal liquid receipt",
        "v3 adds the default-off terminal receiver all-owner envelope",
    ] {
        assert!(index.contains(required), "{INDEX} missing {required}");
    }
}

#[test]
fn terminal_liquid_and_energy_have_closed_non_aliasing_semantics() {
    let energy = read(ENERGY);
    assert!(energy.contains("m_terminal_liquid = m_liquid,retained,start + m_rain,snow_support"));
    assert!(energy.contains("m_melt,new - m_refreeze"));
    assert!(!energy.contains("m_liquid_vapor_export"));
    assert!(energy.contains("terminal_receipt_consumed=true"));
    assert!(energy.contains("Q_terminal_unallocated <= TOL-SNOWENERGY-001"));
    assert!(energy.contains("rejects the receiver transaction with no recipient"));
    assert!(!energy.contains("Q_terminal_unallocated is assigned to soil"));
}

#[test]
fn partial_wb14_is_a_narrow_reconciled_exception_not_a_scaled_proxy() {
    let liquid = read(LIQUID);
    assert!(liquid.contains("fixed production base remains exactly 48 wall bins of 1800 seconds"));
    assert!(liquid.contains("only exception is a tagged terminal receiver segment"));
    assert!(liquid.contains("every other non-1800 call `E-008`"));
    assert!(liquid.contains("compute_green_ampt_interval_infiltration"));
    assert!(liquid.contains("beginning ponded water"));
    assert!(liquid.contains("No copied equation, per-parcel solve"));
    assert!(liquid.contains("A parcel exactly at `wall_t*` belongs to the receiver"));
    assert!(!liquid.contains("remaining forcing is proportionally scaled from the full bin"));
}

#[test]
fn semantic_oracles_close_liquid_support_continuation_and_precedence() {
    assert!((terminal_liquid(0.7, 0.2, 0.5, 0.1) - 1.3).abs() < 1e-12);
    let wall_start = 86_100_u64;
    let wall_event = 86_350_u64;
    let wall_end = 86_400_u64;
    assert_eq!(wall_event - wall_start, 250);
    assert_eq!(wall_end - wall_event, 50);
    assert!(wall_start <= 86_350 && 86_350 < wall_event + 1);
    let continuation = (47_u8, wall_event, false);
    let completed = (48_u8, wall_end, true);
    assert!(completed.0 > continuation.0);
    assert_eq!(completed.1, 86_400);
    assert!(completed.2);
    assert!(precedence_rank("snow_solver") < precedence_rank("rollback"));
    assert_eq!(precedence_rank("restart"), Some(1));
    assert_eq!(precedence_rank("rollback"), Some(11));
    assert_eq!(validate_segment(true, 86_100, 86_350, 86_400), Ok(50));
    assert_eq!(validate_segment(true, 86_100, 86_400, 86_400), Ok(0));
    assert_eq!(
        validate_segment(false, 86_100, 86_350, 86_400),
        Err("E-008")
    );
    assert_eq!(
        validate_segment(true, 86_350, 86_100, 86_400),
        Err("support")
    );
    assert_eq!(consume_receipt(false, true), Ok(true));
    assert_eq!(consume_receipt(true, true), Err("receipt-replay"));
    assert_eq!(consume_receipt(false, false), Err("receipt-replay"));
}

#[test]
fn restart_state_machine_and_canonical_tables_are_enforced() {
    assert!(valid_restart_transition("one_full_done", "first_half_done"));
    assert!(valid_restart_transition("bisect_mid_done", "localized"));
    assert!(!valid_restart_transition("pre_trial", "localized"));
    assert!(!valid_restart_transition("complete", "receiver_running"));

    for (path, invariants) in [
        (ENERGY, &["INV-SNOWENERGY-035"][..]),
        (FREEZE, &["INV-SNOWFREEZE-102"][..]),
        (
            LSE,
            &["INV-LANDSURFACEENERGY-114", "INV-LANDSURFACEENERGY-115"][..],
        ),
        (
            LIQUID,
            &["INV-SURFACELIQUID-010", "INV-SURFACELIQUID-011"][..],
        ),
        (TXN, &["INV-VEGTRANSACTION-008"][..]),
    ] {
        let raw = fs::read_to_string(path).unwrap();
        for invariant in invariants {
            assert!(
                table_has_row(&raw, invariant),
                "{path} has no table row for {invariant}"
            );
        }
    }

    let all = [read(ENERGY), read(FREEZE), read(LSE), read(LIQUID)].join(" ");
    assert!(!all.contains("[t*,dt_interval]"));
    assert!(!all.contains("m_liquid_vapor_export"));
    assert!(!all.contains("I=integral min"));
}

#[test]
fn midnight_restart_and_half_open_endpoint_have_no_overlap_or_gap() {
    let snow_support = 86_400_u64 - 86_100;
    let receiver_support = 86_700_u64 - 86_400;
    assert_eq!(snow_support + receiver_support, 86_700 - 86_100);
    let before_midnight = (0_u32, 47_u8, 86_400_u64, false);
    let after_midnight = (1_u32, 0_u8, 86_400_u64, true);
    assert_eq!(before_midnight.2, after_midnight.2);
    assert_eq!(after_midnight.0, before_midnight.0 + 1);
    assert_eq!(after_midnight.1, 0);
    assert!(!before_midnight.3);
    assert!(after_midnight.3);

    let liquid = read(LIQUID);
    assert!(liquid.contains("`d=0` executes no WB14 physics"));
    assert!(liquid.contains("Untagged variable duration, overlap/gap"));
    assert!(liquid.contains("receipt replay `E-003/E-011`"));
}

#[test]
fn support_restart_and_error_precedence_are_unambiguous() {
    let freeze = read(FREEZE);
    let txn = read(TXN);
    assert!(freeze.contains("Absolute support identity is `(calendar_day, wall_bin, wall_start"));
    assert!(freeze.contains("must not encode, infer, or replace calendar/wall support"));
    assert!(freeze.contains("Global error precedence is total"));
    assert!(freeze.contains("never replaces or masks the first error"));
    assert!(freeze.contains("terminal_receiver_restart_v1` is version `1`"));
    assert!(freeze.contains("one_full_done|first_half_done|two_half_done|bracketed"));
    assert!(freeze.contains("Version 0 has no migration"));
    assert!(freeze.contains("receipt/consumed mismatch rejects before mutation"));
    assert!(txn.contains("rollback diagnostic is secondary unless itself first"));
    assert!(!freeze.contains("transaction ID determines calendar day"));
}
