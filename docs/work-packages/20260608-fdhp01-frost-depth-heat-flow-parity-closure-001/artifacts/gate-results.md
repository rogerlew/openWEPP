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
| `cargo test --test clim06_frost_frozen_soil_kernel_contract -- --nocapture` | Pass, 19 tests after layered-store diagnostics |
| `cargo test --test cli04_runner_wat_parquet_contract_derived_tests -- --nocapture` | Pass, 2 tests |
| `cargo test --test sim_contract_boundary_unit_registry -- --nocapture` | Pass, 14 tests |
| `cargo test -p openwepp --test pl14s_tier_a_candidate_emission_and_replay_contract -- --nocapture` | Pass, 8 tests |
| `cargo test -p openwepp-runner --lib fdhp01_wb13 -- --nocapture` | Pass, 3 tests |
| `cargo test -p openwepp-runner --lib hphys0203_wb13_soil_water_total_preserves_watcon_alias -- --nocapture` | Pass, 1 test |
| `cargo test -p openwepp-hillslope-output schema_includes_required_dataset_metadata_keys -- --nocapture` | Pass, 1 test |
| `cargo test --test hphys0319_fixed_baseline_stmtim_observe_contract -- --nocapture` | Pass, 5 tests after `SC-SNOWFREEZE-001` v56 / `SC-WATBAL-001` v152 updates |
| `cargo test --test hphys0320_stmtim_start_time_source_line_contract -- --nocapture` | Pass, 3 tests after `SC-SNOWFREEZE-001` v56 / `SC-WATBAL-001` v152 updates |

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

## D2 Exchange Diagnostic Gates

| Command / Gate | Result |
|---|---|
| `cargo fmt` | Pass, formatting applied |
| `cargo fmt --check` | Pass |
| `cargo clippy --workspace --all-targets -- -D warnings` | Pass |
| `cargo test --workspace` | Pass |
| `cargo deny check` | Pass |
| `cargo test --test clim06_frost_frozen_soil_kernel_contract -- --nocapture` | Pass, 17 tests; freeze-onset and warm-thaw diagnostics reconcile liquid/frozen exchange |
| `cargo test --test hphys0319_fixed_baseline_stmtim_observe_contract -- --nocapture` | Pass, 5 tests after `SC-WATBAL-001` v150 update |
| `cargo test --test hphys0320_stmtim_start_time_source_line_contract -- --nocapture` | Pass, 3 tests after `SC-WATBAL-001` v150 update |
| `wctl doc-lint --path docs` | Pass, `1220 files validated, 0 errors, 0 warnings` |

## D2 Frozwt Publication Source Gates

| Command / Gate | Result |
|---|---|
| `cargo fmt --check` | Pass |
| `cargo clippy --workspace --all-targets -- -D warnings` | Pass |
| `cargo test --workspace` | Pass |
| `cargo deny check` | Pass |
| `cargo test -p openwepp-runner --lib fdhp01_wb13 -- --nocapture` | Pass, 3 tests; WAT `frozwt` requires `frost.runtime_frwatc_frozen_water_after_m` and rejects a missing exchange-store symbol |
| `cargo test -p openwepp-runner --lib hphys0203_wb13_soil_water_total_preserves_watcon_alias -- --nocapture` | Pass, proves WAT `frozwt` follows the exchange diagnostic instead of `runtime_ws_frz` |
| `cargo test --test hphys0319_fixed_baseline_stmtim_observe_contract -- --nocapture` | Pass, 5 tests after `SC-WATBAL-001` v151 update |
| `cargo test --test hphys0320_stmtim_start_time_source_line_contract -- --nocapture` | Pass, 3 tests after `SC-WATBAL-001` v151 update |
| `cargo test --test clim06_frost_frozen_soil_kernel_contract -- --nocapture` | Pass, 17 tests |
| `cargo build --release -p openwepp-runner --bin openwepp-cli-hill` | Pass |
| `wctl doc-lint --path docs` | Pass, `1220 files validated, 0 errors, 0 warnings` |
| 43-prefix `algebraic-radium` frost-on cohort, `/tmp/fdhp01_frozwt_publication_20260611T070334Z` | Fail, `42/43` clean exits; `p2` failed before WAT publication at `HKERNEL-WB11-PERC-E-003` |
| Annual closure on emitted prefixes | Fail, max abs residual `2.4798612273409617 mm`; unchanged from the post-D1 floor |
| `frozwt/frdp` cohort ratio audit | Fail, `35297` frost-active days; min correlation `0.9999999999999994`, median ratio median `0.15199999999999997`, max ratio standard deviation `3.2273877788806054e-17` |

## Layered Frozen-Store Gates

| Command / Gate | Result |
|---|---|
| `cargo fmt --check` | Pass |
| `cargo clippy --workspace --all-targets -- -D warnings` | Pass |
| `cargo test --workspace` | Pass |
| `cargo deny check` | Pass |
| `wctl doc-lint --path docs` | Pass, `1220 files validated, 0 errors, 0 warnings` |
| `cargo test --test clim06_frost_frozen_soil_kernel_contract -- --nocapture` | Pass, 19 tests; layered store rejects scalar `frdp * theta` equivalence and updates per-layer `frzw` |
| `cargo test --test hphys0319_fixed_baseline_stmtim_observe_contract -- --nocapture` | Pass, 5 tests after `SC-SNOWFREEZE-001` v56 / `SC-WATBAL-001` v152 |
| `cargo test --test hphys0320_stmtim_start_time_source_line_contract -- --nocapture` | Pass, 3 tests after `SC-SNOWFREEZE-001` v56 / `SC-WATBAL-001` v152 |
| `cargo test -p openwepp-runner --lib fdhp01_wb13 -- --nocapture` | Pass, 3 tests |
| `cargo build --release -p openwepp-runner --bin openwepp-cli-hill` | Pass, release binary SHA `b4cb18a728d1556c2ba50f28d3fe7671f306735a82aa5153682a40ad72c69c8e` |
| 43-prefix `algebraic-radium` frost-on cohort, `/tmp/fdhp01_layered_store_20260611T080722Z` | Pass for execution, `43/43` clean exits |
| Annual `Total-Soil + frozwt` closure | Pass, max abs residual `1.2683574368566042e-07 mm`, mean abs residual `2.1277404919798806e-09 mm` |
| Soil-only annual identity | Expected fail under v152 additive storage term, max abs residual `119.04111532237937 mm` |
| `frozwt/frdp` cohort ratio audit | Pass, no exact scalar ratio: `36064` frost-active days, min correlation `0.8210678396408895`, median correlation `0.963536279373424`, max ratio standard deviation `0.0700106996666242` |
| FDMC01 depth/duration movement | Fail, max-depth mean `1782.0379909380451 mm` versus legacy `414.22093023255815 mm`; median depth correlation `-0.27756218032931956`; frozen-day delta `-518.5348837209302` |

Disposition: Rust gates and D2 storage closure pass, but package acceptance
still fails on D3 frost-depth parity. FDHP01 is executed-hold.
