# D3 Increment C1a Seam Accounting Diagnostic

Status: executed-hold; diagnostic complete
Evidence mode: Ran + Static
Date: 2026-06-11

## Scope Executed

Increment C1a was run without the comparator subagent because the user reported
the GPT-5.3-Codex-Spark weekly quota was exhausted. A temporary env-gated
ledger hook was applied to the current B-boundary source and removed before
commit. No production physics edits are retained by this artifact.

Artifacts:

- `fdhp01_increment_c1a_seam_accounting_summary_20260611.json`
- `fdhp01_increment_c1a_seam_ledger_excerpt_20260611.csv`

Diagnostic run root:
`/tmp/fdhp01_increment_c1a_diagnostic_20260611T231749Z`.

## Result

The C1a ledger found a pre-C1b hard boundary, matching the capacity-watch item:
the freeze path can already write illegal ice state before redistribution or
overflow code is reintroduced.

- `p43` failed with `HKERNEL-WB14-RUNOFF-E-003` at simulation day 94
  (`1990-04-04`). The illegal state was written on day 93
  (`1990-04-03`): aggregate `frzw=50.58972525883585 m` against aggregate
  `ul=0.543517677999698 m`, with `50.049070656902806 m` over the upper-limit
  bound.
- `p1` failed with the same guard and day. Day 93 wrote aggregate
  `frzw=51.18301848887181 m` against the same aggregate
  `ul=0.543517677999698 m`, with `50.644102740198335 m` over the bound.
- Fine-layer ice exceeded the legacy `frznw` capacity form immediately on the
  freeze path, before aggregate `frzw` exceeded `ul`: day 1 p43 had
  `0.041949772970434 m` fine-ice excess and p1 had
  `0.042174177930601 m`.
- The largest shadow `frwatc(1)` residual before the day-93 re-freeze was
  `33.4009943366675 m` on p43 and `33.79382883453257 m` on p1, showing that
  the fine/coarse handoff is already desynchronized before the capacity guard
  rejects the next day.

The prior C1 p43 aggregate-cap smoke remains relevant: the cap collapsed
published storage to `ProfilePorosityCap=809.0776779996982 mm`, but annual
closure still missed by up to `200.39845415539014 mm`. That proves a downstream
aggregate clamp is not a valid accounting fix.

## Attribution

Static code points:

- `apply_shadow_frwatc_ingress` applies `st - yst` to the fine state:
  `crates/openwepp-hillslope-orchestrator/src/hydrology/support_helpers_mod/coupling.rs:301`.
- `aggregate_shadow_layer` recomputes coarse shadow state and sets `yst`:
  `crates/openwepp-hillslope-orchestrator/src/hydrology/support_helpers_mod/coupling.rs:361`.
- The read side rejects `frzw > ul`:
  `crates/openwepp-hillslope-orchestrator/src/hydrology/support_helpers_mod/coupling.rs:1228`.
- `freeze_fine_front` freezes `slsw_theta` into `slsic` without the remaining
  pore-capacity limit used by legacy `frznw`:
  `crates/openwepp-hillslope-orchestrator/src/hydrology/support_helpers_mod/coupling.rs:672`.
- Runoff writeback publishes aggregate `frzw` without an upper-limit bound and
  writes the fine shadow state separately:
  `crates/openwepp-hillslope-orchestrator/src/hydrology/kernel_phases_mod/hydrology_phase_runoff_reconciliation.rs:949`
  and `:1006`.

The first C1a failure is therefore not an overflow-routing choice downstream
of a valid freeze step. It is the combination of:

1. fine-state ingress accumulating a large `st - yst` handoff while coarse and
   fine representations are still both active,
2. a freeze-front step that converts that liquid to ice without the per-fine
   capacity bound, and
3. aggregate/fine writeback that lets the illegal state re-enter the next day,
   where the existing read-side `frzw <= ul` guard correctly rejects it.

## C1b Requirement

C1b must implement to the C1a accounting specification appended to
`d3-fine-sublayer-port-scope.md`. In practical terms, the next code pass must
first make the daily frost/WB handoff single-owner and capacity-limited:

- apply `st - yst` once at the day ingress point;
- let fine state own liquid and ice until egress;
- bound all new ice by `ul/dg * slfsd - slsic`;
- recompute coarse `theta`, `frzw`, `frozen_depth`, `soil_water`, `st`, `yst`,
  and `nwfrzz` wholesale at egress; and
- route any `watpdg`/`watbtm` overflow through named flux/storage surfaces that
  are part of the WAT identity.

Acceptance for the next implementation increment starts with passing the day-94
capacity boundary on p43 and p1 with zero aggregate `frzw > ul` rows and
shadow `frwatc` residuals at numerical noise, before re-running the full
years-2-6 additive-identity cohort gate.
