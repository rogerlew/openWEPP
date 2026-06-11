# Gate Results

Status: executed-hold

Evidence mode: Ran

Date: 2026-06-11

## Required Gates

| Gate | Result |
|---|---|
| `cargo fmt --check` | Pass |
| `cargo clippy --workspace --all-targets -- -D warnings` | Pass |
| `cargo test --workspace` | Pass |
| `cargo deny check` | Pass |

## Focused Regression Gates

| Command | Result |
|---|---|
| `cargo test --test clim06_frost_frozen_soil_kernel_contract -- --nocapture` | Pass, 16 tests |
| `cargo test --test cli04_runner_wat_parquet_contract_derived_tests -- --nocapture` | Pass, 2 tests |
| `cargo test --test sim_contract_boundary_unit_registry -- --nocapture` | Pass, 14 tests |
| `cargo test -p openwepp --test pl14s_tier_a_candidate_emission_and_replay_contract -- --nocapture` | Pass, 8 tests |
| `cargo test -p openwepp-runner --lib fdhp01_wb13_publication -- --nocapture` | Pass, 2 tests |
| `cargo test -p openwepp-hillslope-output schema_includes_required_dataset_metadata_keys -- --nocapture` | Pass, 1 test |
| `cargo test --test hphys0319_fixed_baseline_stmtim_observe_contract -- --nocapture` | Pass, 5 tests after `SC-SNOWFREEZE-001` v55 update |
| `cargo test --test hphys0320_stmtim_start_time_source_line_contract -- --nocapture` | Pass, 3 tests after `SC-SNOWFREEZE-001` v55 update |

## Dependency / Authority Guards

`cargo deny check` reported:

```text
advisories ok, bans ok, licenses ok, sources ok
```

No source-level anti-evasion guard was run for this package because FDHP01 did
not edit external-authority suite posture, cohort fixture bindings, or
required-case bindings. The auth11 guard test was included in the successful
workspace test run.

## Post-Review Cohort Gate

| Command / Gate | Result |
|---|---|
| `cargo build --release -p openwepp-runner --bin openwepp-cli-hill` | Pass |
| D1 focused gates: `cargo fmt --check`; WB13 runner unit; summary accumulator lib; HPHYS0203; HPHYS0208; CLIM06 frost suite | Pass |
| 43-prefix `algebraic-radium` frost-on cohort, `/tmp/fdhp01_closure_after_d1_restored_20260611T053545Z` | Fail, `42/43` clean exits; `p2` failed before WAT publication at `HKERNEL-WB11-PERC-E-003` |
| Annual closure on emitted prefixes | Fail, max abs residual `2.4798612273409617 mm` after D1; pre-D1 post-review residual was `75.43917280313423 mm` |
| FDMC01 depth/duration movement on emitted prefixes | Fail, max-depth mean `1782.2670980346527 mm`, median correlation `-0.10301692862035305`; duration delta diagnostic only because closure failed |

Disposition: Rust gates pass, but the package acceptance gate fails. FDHP01 is
executed-hold.
