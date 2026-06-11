# D3 Increment C1 Capacity/Redistribution Attempt

Status: executed-hold; backed out after failed gate
Evidence mode: Ran
Date: 2026-06-11

## Scope Executed

Increment C1 was attempted without the comparator subagent because the user
reported the GPT-5.3-Codex-Spark weekly quota was exhausted. The parent ran the
local focused tests, release build, 43-prefix cohort, and targeted p43 smoke
checks.

The attempted implementation added:

- Fine-layer ice capacity guard from `frznw.for:123-135`
  (`ul/dg * slfsd - slsic`).
- `watpdg`/`watbtm` runtime surfaces.
- Downward fine-layer liquid redistribution and `watbtm` overflow routing.
- Aggregate WB18 upper-limit capping as a follow-up localization step.

## Full Cohort Gate

Ran: release `openwepp-cli-hill` over the 43-prefix `algebraic-radium` cohort
without subagent delegation.

- Run root: `/tmp/fdhp01_increment_c1_capacity_fix_20260611T224555Z`
- Reports copied into this artifact directory:
  - `fdhp01_increment_c1_run_status_20260611.tsv`
  - `fdhp01_increment_c1_run_summary_20260611.json`
  - `fdhp01_increment_c1_execution_summary_20260611.json`
  - `fdhp01_increment_c1_annual_closure_residuals_20260611.csv`
  - `fdhp01_increment_c1_depth_metrics_20260611.csv`
  - `fdhp01_increment_c1_frozwt_frdp_ratio_20260611.csv`
  - `fdhp01_increment_c1_activation_summary_20260611.csv`

Result:

- `43/43` clean exits, `43/43` WAT outputs.
- Years 2-6 `Total-Soil + frozwt` max abs residual:
  `16628.157022818832 mm` (fail; D2 hard stop).
- Years 2-6 mean abs residual: `6238.80817440851 mm`.
- `frozwt/frdp` correlation max: `0.9919091097937477` (fail vs Increment B
  max `0.9861968090242198`).
- Profile-bound pinning did not regress (`0` pinned prefixes).

The full cohort showed the first redistribution attempt stopped the earlier
geometric blow-up but still allowed `Total-Soil` to exceed the profile storage
cap by metres on affected prefixes.

## Localization Smoke

Ran: p43-only smoke after adding an aggregate WB18 upper-limit cap.

- Run root: `/tmp/fdhp01_increment_c1_p43_aggregate_cap_20260611T225644Z`
- Summary artifact:
  `fdhp01_increment_c1_p43_aggregate_cap_smoke_20260611.json`

Result:

- The aggregate cap collapsed p43 storage overfill to the published
  `ProfilePorosityCap` (`max_storage_mm = 809.0776779996984`,
  `ProfilePorosityCap = 809.0776779996982`).
- Annual closure still failed by `15.881486434365115` to
  `200.39845415539014 mm`.

This proves the capacity cap is necessary but not sufficient. The remaining C1
defect is not just oversized fine-layer storage; the frost-side
redistribution/overflow accounting still does not reconcile with the WAT
balance identity.

## Disposition

Increment C1 did not meet its D2 hard stop and was backed out. The production,
contract, and test edits from the attempted implementation must not be carried
forward.

Next C1 dispatch should start from the committed Increment B boundary and first
map the exact accounting target for `watpdg`/`watbtm` against WB18/WB19/WAT
outputs before reintroducing capacity code. A valid solution must satisfy both:

- per-fine-layer and aggregate WB18 capacity invariants, and
- years 2-6 `Total-Soil + frozwt` cohort closure at the Increment B noise
  floor.
