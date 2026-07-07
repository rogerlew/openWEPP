# LANED-T3-AGG — Aggressive-rule deficit-carry composition fix (router numerics Tier 3 follow-on)

Status: **EXECUTED-PRIZE-NOT-REALIZED** (2026-07-07; scaffolded 2026-07-06,
operator-directed: "scaffold and execute work-package for aggressive-rule
deficit-carry fix"). Defect-closure shape (diagnose AND correct in one pass —
the diagnosis was already complete and contract-recorded at `SC-OFEROUTE-001`
rev 28). OUTCOME: the composition defect is CLOSED (rev 30) — the deficit
carry landed, the aggressive rule is live behind the experimental selector,
H2637 runs green through the former failure coordinates with machine-exact
closures, and the carry path demonstrably fired 6×/yr. The ~1.9x TIMING
prize did NOT materialize: the explicit-work cut matched the I0 coverage
exactly, but the implicit cell-solve cost consumes it (endpoint `38.0-38.3 s`
vs `37.9 s` plain) — the named follow-on lever is implicit solve-cost
reduction (see `artifacts/fix-evidence.md`). DUAL REVIEW COMPLETE
(2026-07-07): both Codex lanes GO-WITH-AMENDMENTS, no High findings; all
findings accepted and fixed same-day (C-M1 hour-partition guard, C-L1
bounded-drop pin, QA-M1 canonical re-gates `1426/1426`, QA-M2 contract-table
promotion, QA-M3 recipe promotion + AGENTS.md remainder delegated to Codex,
QA-L1/L2) — `artifacts/review-disposition.md`; post-fix H2637 books
bit-identical.

Parent: `20260706-laned-router-t3-hybrid-implicit-stepping-001/` (worker
handoff item 1 — the ~1.9x prize). Backlog authority:
[docs/backlog/20260706-laned-router-numerics-performance-tiers.md](../../backlog/20260706-laned-router-numerics-performance-tiers.md).
Contract focus: `SC-OFEROUTE-001` (rev-30 amendment, contract-first).

Base intake: clean `main` at `ef4172d5` (rev 29; D15A + T3 rev-28/29
landed). No branch creation.

## Defect (recorded, reproduced)

The hybrid composition (`route_single_ofe_hybrid`) partitions the day window
into implicit/explicit spans. The explicit solver's `BinRecorder` forward
redistribution (rev 24, Review-B M2) zeroes transiently-negative
front-arrival outlet bins and carries the deficit into LATER bins — but a
SHORT explicit span has no later bins, so `finish()` reports a material
terminal deficit and `run_with_options` fails closed
(`RoutingError::NegativeOutletBin`, `kinematic_wave.rs`). Under the
AGGRESSIVE switching rule (zero-source-only; upstream-fed bins step
implicitly) short explicit spans sandwiched between implicit bins are
common; the failure was observed at H2637 lane 17 day 54. The strict rule
(rev 28) avoided the defect by never sandwiching upstream-fed spans; the
prize (55.5 % implicit-eligible coverage vs 30 % strict; I0 estimate ~1.9x
active endpoint) is blocked on exactly this composition seam.

## Fix shape (per the T3 handoff + Codex adversarial confirmation)

1. **Solver-API extension** (`kinematic_wave.rs`): a deficit-carry variant
   of the windowed run that RETURNS the recorder's material terminal deficit
   to the caller instead of failing closed. `run_with_options` becomes a
   wrapper that keeps the exact fail-closed posture (plain path
   bit-identical; default path untouched).
2. **Cross-span absorption** (`cascade.rs`): `route_single_ofe_hybrid`
   CONTINUES the recorder's forward-redistribution rule across span
   boundaries — the carried deficit is absorbed by subsequent composed
   global bins under the same exact-total, non-negative rule. A material
   deficit remaining at the day-window END still fails closed
   (`NegativeOutletBin`); a sub-noise remainder folds into the last covered
   bin (the recorder's own noise rule, at the composed level).
3. **Mask flip**: the per-bin smoothness predicate drops the
   zero-upstream-mass condition (zero-source-only). The implicit step
   already books the interval-mean upstream inflow exactly.

## Acceptance

- Contract rev-30 amendment ratified BEFORE the behavior change: the
  aggressive rule as the selector's semantics, the cross-span deficit-carry
  composition rule (exact total, non-negative bins, fail-closed material
  end-of-window deficit), and the composition-scoped solver-API extension.
- Unit vectors: (a) a forced terminal-deficit explicit span composes
  exactly (Σ composed bins == booked outflow at machine precision; all bins
  non-negative); (b) an end-of-window material deficit FAILS CLOSED through
  the hybrid; (c) the deficit-carry wrapper preserves `run_with_options`
  fail-closed behavior; existing hybrid vectors (all-explicit bit-identity,
  event-day ledger/fidelity, non-integral-window rejection) stay green
  unchanged.
- H2637 executed evidence (aggressive rule): full-year active hybrid run
  green on ALL rev-27 day-closure hard-fails (supply / router-internal /
  seam cross-ledger / day identity) including the former failure
  coordinates (lane 17 day 54); implicit step share recorded
  (`solver_steps_implicit`); endpoint timing (3 runs) vs the recorded
  plain-active `37.9 s` and strict-hybrid `37.0-37.2 s` baselines.
- Plain-path invariance: `OPENWEPP_LANED_ACTIVE=1` (no hybrid selector)
  parquet hash unchanged (`21c54bf2…`); default path untouched by
  construction (workspace suite).
- Gates: `cargo fmt --check`, workspace clippy, `cargo deny check`, full
  workspace nextest, focused `ofe_routing` suites. Any FAIL/BLOCKED gate
  holds the package.
- EXPERIMENTAL posture unchanged: the selector remains evidence-gathering;
  fidelity tolerances remain UNRATIFIED (ratification is the parent
  package's open gate, not this one's scope).

## Included / Excluded

Included: `crates/openwepp-hillslope-orchestrator/src/ofe_routing/{kinematic_wave,cascade}.rs`,
`SC-OFEROUTE-001` rev-30 amendment, package artifacts, focused tests, H2637
evidence runs. Excluded: default-path changes (byte identity stands),
fidelity-tolerance ratification, Case-4 hybrid ladder, Tier-1/Tier-2,
D16/default promotion, watershed scope, f32 anywhere.

## Required artifacts

`artifacts/fix-evidence.md` (design deltas + unit-vector + H2637 evidence),
`artifacts/gate-results.md`, review/disposition set per
`docs/work-packages/AGENTS.md` (dual review dispatched post-execution, same
lane structure as the parent package), updates to the parent
`worker-handoff.md` (item 1 disposition).

Subagent authorization: this package authorizes read-only review, gate
execution, and timing-run subagents; expected outputs are findings, metrics,
and package-local artifact text.
