# D3 Increment C Thaw-Arm Attempt Evidence

Status: executed-hold, backed out

Evidence mode: Static + Ran

Date: 2026-06-11

## Scope

Increment C attempted to port the thaw side of the staged fine-sublayer frost
state machine: top thaw (`mlttp`), bottom thaw (`mltbtm`), sandwich geometry,
and `fgthwd` thaw-through behavior.

The production, contract, and test edits from this attempt were backed out
because the increment violated the staged plan's D2 hard stop. The committed
tree remains at the Increment B state (`SC-SNOWFREEZE-001` v59).

## Local Evidence

Ran:

| Command / Gate | Result |
|---|---|
| `cargo test --test clim06_frost_frozen_soil_kernel_contract fdhp01_fine_sublayer_mlttp_top_thaw_sets_sandwich_and_fgthwd -- --nocapture` before production edits | Failed as expected: partial top-thaw vector still reported `fgfrst=2` instead of `3` |
| `cargo test --test clim06_frost_frozen_soil_kernel_contract -- --nocapture` after edits | Pass, 29 tests before the pore-cap regression; 30 tests after the temporary pore-cap regression |
| `cargo test --workspace` | Pass before the cohort failure was discovered |
| `cargo fmt --check` / `git diff --check` / `cargo clippy --workspace --all-targets -- -D warnings` | Pass after a test-helper `clone_on_copy` cleanup |
| `cargo deny check` | Pass |
| `bash tools/release/check_authority_suite_antievasion.sh` | Pass |
| `cargo test --test auth11_required_suite_obligation_guards_contract -- --nocapture` | Pass, 2 tests |

The required `comparator_suite_runner` subagent could not execute: the spawned
subagent errored due a GPT-5.3-Codex-Spark usage limit. Heavy gates were
therefore run locally in the parent session and recorded in
`fdhp01_increment_c_execution_summary_20260611.json`.

## Cohort Evidence

Run root:
`/tmp/fdhp01_increment_c_thaw_20260611T215005Z`.

Generated reports copied into this package:

- `fdhp01_increment_c_execution_summary_20260611.json`
- `fdhp01_increment_c_run_status_20260611.tsv`
- `fdhp01_increment_c_annual_closure_residuals_20260611.csv`
- `fdhp01_increment_c_depth_metrics_20260611.csv`
- `fdhp01_increment_c_frozwt_frdp_ratio_20260611.csv`
- `fdhp01_increment_c_activation_summary_20260611.csv`

The first cohort attempt failed on `p1` at 1990 day 45 with
`HKERNEL-WB11-PERC-E-003`. A temporary local diagnostic showed the actual guard:
`wb18_perc_frzw_0001=0.06135293352005228` exceeded
`wb18_perc_ul_0001=0.05875247947169813`. A source cap on freeze-front ice
formation removed that runtime failure, but the full cohort then revealed the
larger defect below.

Post-cap cohort metrics:

- Clean execution: `43/43` prefixes.
- Years 2-6 `Total-Soil + frozwt` max abs residual:
  `2325832826960980.0 mm`; mean abs residual:
  `863664411656061.6 mm`.
- Year 7 boundary residual also explodes: max abs
  `2203549546983243.5 mm`.
- `Total-Soil` grows catastrophically in frost/thaw cycles; for `p1`, year 1
  reaches `1.327023e+14 mm`, and year 4 reaches `1.558719e+35 mm`.
- Profile-bound pinning remains directionally unpinned (`0/43` pinned), but
  depth is still near the profile bound: mean max depth
  `1794.0628184427708 mm`, minimum margin `5.350358610292005 mm`.
- Depth correlation regresses: median `-0.4265170275507577`.
- Frozen duration overshoots legacy instead of converging: median
  open-minus-legacy frozen-day delta `+382` days.

## Disposition

Increment C failed before package acceptance because it broke the D2 additive
storage identity. The failed implementation demonstrated that thaw arms cannot
be retained without the missing `watdst` redistribution and `watpdg`/`watbtm`
overflow paths. Simply melting `slsic` into the thawed fine-layer liquid state
lets fine-layer water content amplify across freeze/thaw cycles and eventually
poisons WB18/WAT storage by astronomical magnitudes.

All Increment C production, contract, and test edits were backed out. FDHP01
remains `executed-hold` at the Increment B boundary. The next Increment C pass
must first implement/verify capacity-aware `watdst` redistribution and thaw
overflow handling, then reintroduce `mlttp`/`mltbtm` against the same D2
hard-stop gate.
