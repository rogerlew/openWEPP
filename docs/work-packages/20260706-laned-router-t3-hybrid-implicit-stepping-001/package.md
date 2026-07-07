# LANED-T3 — Hybrid implicit kinematic-wave stepping (router numerics Tier 3)

Status: **EXECUTED-HOLD-REV28-RATIFICATION** (2026-07-07, post-dual-review;
originally 2026-07-06, operator-directed same-session start after D15A
closure: "prioritize the tiers to maximize your remaining availability and
do the most complex work first — partition the kinematic wave stepping
first"). I0/I1 complete. I2 is landed as an EXPERIMENTAL evidence-gathering
opt-in only: the run-level gates that exist are green
(`artifacts/gate-results.md`), but the I2 CLOSURE acceptance below (the
full hybrid Case-4 oracle ladder and the fidelity-tolerance ratification)
is NOT met and rev 28 is NOT settled — the Codex dual review (NO-GO for
settling rev 28 / GO-WITH-AMENDMENTS as experimental evidence) and the
finding fixes are recorded in `artifacts/review-disposition.md`. Remaining
acceptance + the aggressive-rule composition fix are the recorded next
actions (`artifacts/worker-handoff.md`). Backlog authority:
[docs/backlog/20260706-laned-router-numerics-performance-tiers.md](../../backlog/20260706-laned-router-numerics-performance-tiers.md).
Contract focus: `SC-OFEROUTE-001` (rev-28 amendment, contract-first per
increment).

Base intake: the D15A working tree (opt-in active owner + rev-27 closure
hard-fails green). No branch creation.

## Objective

Land the highest-complexity piece of the router-numerics performance program
first: an IMPLICIT kinematic-wave stepper for smooth phases and the HYBRID
switching rule that keeps the explicit TVD-MacCormack scheme on
shock/source-active phases — partitioned so the design- and numerics-hard
increments (I0-I2) are executed now and the mechanical tiers (Tier 1/Tier 2)
are delegable follow-ons.

## Partition (operator-requested)

| Inc | Scope | Complexity | Owner |
|---|---|---|---|
| T3-I0 | Prize measurement + scheme design + contract amendment draft: instrument the share of solver steps/cost in ZERO-SOURCE HOMOGENEOUS phases on H2637 active; specify the implicit downstream-marching scalar-Newton stepper, its exact ledger, its positivity proof, the hybrid switching rule, and the acceptance surfaces. | design-hard | Fable (now) |
| T3-I1 | The implicit stepper at solver tier (`ofe_routing::implicit_recession`): per-step machine-exact ledger, unconditional positivity, unit physics vectors (steady state; recession vs the characteristics oracle; dt-refinement ladder). NO production wiring. | numerics-hard | Fable (now) |
| T3-I2 | Hybrid integration behind an explicit opt-in NUMERICS selector on the ACTIVE path only: forcing-derived deterministic switching (implicit only on zero-source, zero-upstream-inflow spans), rev-28 contract amendment ratified, Case-4 oracle ladder + H2637 active evidence + rev-27 closure hard-fails + timing. | integration-hard | Fable (if session budget holds; else first delegable) |
| T3-I3 | Tier 1 (analytic celerity; Newton-α; `h·sqrt(h)`/pow) — fully specified in the backlog note; mechanical relative to I0-I2. | moderate | delegable (Codex/other) |
| T3-I4 | Tier 2 production mesh-resolution adjudication via the existing oracle ladder. | moderate | delegable |

Each increment carries its own gates; a later increment never carries an
earlier one's acceptance (gate non-deferral). I0/I1 make NO production
behavior change (solver-tier only); the first behavior-affecting increment is
I2, which is opt-in-selector-gated and contract-ratified before code.

## Included / Excluded

Included: `crates/openwepp-hillslope-orchestrator/src/ofe_routing/**` (new
implicit module; instrumentation counters), `docs/specifications/…/SC-OFEROUTE-001.md`
(rev-28 draft/amendment), package artifacts, focused tests, H2637 evidence
runs. Excluded: default-path changes (byte identity stands), D16 policy, the
shadow's recorded behavior, watershed scope, Tier-1/Tier-2 implementation
(delegated increments), f32 anywhere.

## Acceptance (per the backlog promotion criteria)

- I1: implicit stepper conservation EXACT per step (booked = actual by
  construction); positivity unconditional (no clamps); recession solution
  converges to the characteristics/upwind reference under dt-refinement with
  named tolerances; steady state reproduced exactly.
- I2: full hybrid Case-4 within the rev-25/26 ratified oracle tolerances at
  every ladder rung; H2637 active run green on ALL rev-27 hard-fails
  (supply/router/seam/identity); recorded before/after endpoint + slot
  timing; named fidelity delta vs the pre-change trajectory at the
  hydrograph surface (bit-identity is deliberately surrendered — that is the
  point of the tier).
- Any FAIL/BLOCKED gate holds the increment, not the earlier ones.

## Required artifacts

`artifacts/i0-prize-measurement.md`, `artifacts/i0-scheme-design.md`
(includes the rev-28 amendment draft), `artifacts/i1-implicit-stepper-evidence.md`,
`artifacts/i2-hybrid-evidence.md` (if reached), `artifacts/gate-results.md`,
review/verification/disposition set per `docs/work-packages/AGENTS.md`,
`artifacts/worker-handoff.md` (must make I3/I4 delegable with exact specs).

Subagent authorization: this package explicitly authorizes
spawning/delegating to `rust_code_reviewer`, `rust_qa_reviewer`, `explorer`,
and `timing_comparator` subagents for read-only review, oracle/gate
execution, and timing runs; expected outputs are findings, metrics, and
package-local artifact text; write access is read-only.
