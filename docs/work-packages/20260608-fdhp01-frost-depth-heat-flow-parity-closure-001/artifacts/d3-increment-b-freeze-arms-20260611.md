# D3 Increment B Freeze-Arm Evidence

Status: executed-hold

Evidence mode: Static + Ran

Date: 2026-06-11

## Scope

Increment B rebounded frost-depth authority from the prior scalar target
projection to the persisted fine-sublayer state. This increment ports the
freeze-side structure needed before the thaw-arm pass:

- `SC-SNOWFREEZE-001` is now v59. It adds `frost.hourly.frzflg_####`,
  declares `fgfrst`/`slfsd`/`slsic`/`slsw`/`nwfrzz` as the active freeze-arm
  state, requires `watdst`-style derived depth from fine flags, and retires
  scalar `apply_layered_frost_target` projection as production authority. It
  also ratifies threshold-bounded exchange-debit limiting at the
  available-liquid handoff boundary; larger overruns remain hard domain
  violations.
- `compute_active_frost_coupling` now mutates the fine state for freeze-active
  hours: `frznw`-equivalent refreeze consumes `nwfrzz` into `slsic` before
  ordinary front advance, and front advance consumes hourly freezing energy
  against fine-layer water into `slfsd`/`slsic`.
- Runtime `frdp`/`thdp`/`tfrdp`/`tthawd` are derived from a fine-layer scan.
  Per-layer `wb18_perc_frozen_depth_####` and `wb18_perc_frzw_####` are
  aggregated from the same state after hourly mutation.
- Increment B intentionally does not complete `mlttp`/`mltbtm` sandwich and
  thaw-through behavior. The surviving thaw behavior is a bounded bottom-retreat
  carry-over sufficient to preserve existing warm-thaw exchange tests; Increment
  C owns top/bottom thaw arms, sandwich geometry, `fgthwd`, and final D3
  depth/duration acceptance.

## Red-Test Evidence

Ran:

| Command | Result |
|---|---|
| `cargo test --test clim06_frost_frozen_soil_kernel_contract fdhp01_frostn_dispatch_arms_match_inv_snowfreeze_012 -- --nocapture` before production edits | Failed as expected: missing `frost.hourly.frzflg_0001` |

## Local Gates

Ran:

| Command | Result |
|---|---|
| `cargo fmt --check` | Pass |
| `cargo test --test clim06_frost_frozen_soil_kernel_contract -- --nocapture` | Pass, 26 tests; includes Increment B dispatch, fine-front energy, `frznw`, and `watdst` vectors |
| `cargo test --test hphys0319_fixed_baseline_stmtim_observe_contract -- --nocapture` | Pass, 5 tests after `SC-SNOWFREEZE-001` v59 |
| `cargo test --test hphys0320_stmtim_start_time_source_line_contract -- --nocapture` | Pass, 3 tests after `SC-SNOWFREEZE-001` v59 |
| `cargo test -p openwepp --test pl14s_tier_a_candidate_emission_and_replay_contract -- --nocapture` | Pass, 8 tests after the threshold-bounded exchange-debit boundary fix |
| `cargo clippy --workspace --all-targets -- -D warnings` | Pass after renaming the local depth-summary fields to satisfy `clippy::struct_field_names` |

## Heavy Gates

Ran via the required comparator-suite runner:

| Command / Gate | Result |
|---|---|
| `cargo clippy --workspace --all-targets -- -D warnings` | Pass |
| `cargo test --workspace` | Pass |
| `cargo deny check` | Pass |
| `bash tools/release/check_authority_suite_antievasion.sh` | Pass |
| `cargo test --test auth11_required_suite_obligation_guards_contract -- --nocapture` | Pass, 2 tests |
| 43-prefix `algebraic-radium` frost-on cohort | Pass, `43/43` clean exits |

The copied FROSTVAL01 wrappers carry absolute `[outputs]` paths, so the clean
cohort wrote WAT/HBP/loss outputs under
`/tmp/frostval01_rerun_20260611T020951Z/outputs`; the CLI manifest root was
`/tmp/fdhp01_increment_b_final_20260611T193423Z/outputs`.

Generated reports copied into this package:

- `fdhp01_increment_b_execution_summary_20260611.json`
- `fdhp01_increment_b_run_status_20260611.tsv`
- `fdhp01_increment_b_annual_closure_residuals_20260611.csv`
- `fdhp01_increment_b_depth_metrics_20260611.csv`
- `fdhp01_increment_b_frozwt_frdp_ratio_20260611.csv`

Increment B gate metrics:

- Years 2-6 `Total-Soil + frozwt` max abs residual:
  `3.0880187296133954e-11 mm`; mean abs residual:
  `1.2662284657486707e-11 mm`.
- Year 7 boundary residual remains tiny: max abs
  `1.2683569483584733e-07 mm`. Year 1 has an initialization residual
  (`1.0505061950707386 mm` max) and is outside the staged gate.
- Profile-bound pinning is removed for this gate: `0/43` prefixes pin at
  `ProfileDepth`; minimum margin to profile bound is
  `16.63152804088827 mm`.
- Depth magnitude remains a D3 hold item despite de-pinning: mean max depth
  `1782.265765656973 mm`, median max depth `1782.454753408546 mm`.
- `frozwt/frdp` is materially below the scalar signature: `36064`
  frost-active rows, per-prefix correlation min/median/max
  `0.8210678396408894` / `0.9635362793734238` /
  `0.9861968090242198`, versus the rejected `0.9987` scalar signature.

## Disposition

Increment B passes its staged gates. It removes the scalar projection
anti-pattern from the freeze side, makes depth a fine-state derivative, and
clears the required de-pinning/correlation checks without regressing D2
conservation. It is not D3 closure: depth still rides near the physical profile
bound, and D3 remains `executed-hold` until Increment C ports the top/bottom
thaw arms, sandwich geometry, and thaw-through state machine and passes the
package's depth/duration acceptance gate.
