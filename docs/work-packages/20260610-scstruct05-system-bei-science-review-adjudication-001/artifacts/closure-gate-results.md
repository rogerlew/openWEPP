# Closure Gate Results

Evidence: Ran
Date: 2026-06-10

## Gates

| Gate | Result | Evidence |
|---|---|---|
| Focused reconciled test | pass | `cargo test --test hphys0202_profile_fc_wp_lineage_contract hphys0202_package_and_contract_authority_sections_exist` passed. |
| `cargo fmt --check` | pass after format | First run failed on formatting in reconciled test; `cargo fmt` applied; rerun passed. |
| `cargo clippy --workspace --all-targets -- -D warnings` | fail unrelated | Failed in `tests/integration/owcmp_cli_contract.rs` on `clippy::similar_names` and `clippy::too_many_lines`; file not touched by SCSTRUCT05. |
| `cargo test --workspace` | fail unrelated | Reconciled HPHYS0202 test passed; run later failed in `tests/integration/hphys0298_paired_lineage_partition_contract.rs::hphys0298_harness_rejects_historical_hrsnow_water_equiv_pairing`. |
| `cargo deny check` | pass | `advisories ok, bans ok, licenses ok, sources ok`. |
| `git diff --check` | pass | No whitespace errors. |
| BEI lint default | pass-deferred | 27 rows, 11 follow-ons, exit `0`. |
| BEI lint strict | expected fail | 27 rows, 11 follow-ons, strict exit `1`. |

## Unrelated Failure Details

Clippy:

```console
error: binding's name is too similar to existing binding
tests/integration/owcmp_cli_contract.rs:708
error: this function has too many lines (135/100)
tests/integration/owcmp_cli_contract.rs:62
```

Workspace test:

```console
test hphys0298_harness_rejects_historical_hrsnow_water_equiv_pairing ... FAILED
unit guard stderr must point to the bad water-equivalent field and corrected HPHYS0299 depth surface
```

These failures are outside the SCSTRUCT05 write set.
