# Implementation

Status: EXECUTED
Evidence mode: Static.

## Contract Authority

`SC-OFEROUTE-001` is amended to rev 40.

Rev 40 adds an active-publication guard to the active day-window/inter-day-reset
surface: a routed active day fails closed when positivity-clamp injection
exceeds the active router's external injected source mass for that day. This is
not a tolerance relaxation and not a bounded clamp-ratio residual class.

Explicit non-claims:

- No target-`dx` production promotion.
- No source/coefficient tuning.
- No hydrology, crop, climate, soil, management, or disturbed-data change.
- No D10B TVD-MacCormack solver/oracle semantic change.
- No accepted positivity-preserving solver correction.

## Code Change

`crates/openwepp-hillslope-orchestrator/src/direct_runtime/laned_active.rs`
adds:

- `LANED_ACTIVE_CLAMP_INPUT_REL_CAP = 1.0`.
- A day-closure guard before the ordinary rev-27 cascade residual:
  `books.clamp_m3 > books.injected_m3 + closure_slack` returns
  `DirectKernelGuardFailure` at phase
  `laned_active_clamp_exceeds_source`.
- Focused unit vectors in
  `day_closure_enforces_cascade_and_identity_tolerances` proving a day with
  algebraically exact clamp-adjusted router books still fails when clamp mass
  exceeds source mass, while clamp equal to source remains allowed and
  zero-source/nonzero-clamp fails.

`crates/openwepp-hillslope-orchestrator/src/direct_runtime/03_executor.rs`
is reordered so active routing for every lane in a day populates local
`day_frames` and route books first. `laned_active_enforce_day_closure` now runs
before active rows are built, consumed, dynamically published, or committed.
Only after the guard passes does the executor run erosion/ledger and row
publication in lane order, preserving downstream erosion-inflow semantics.

The guard is intentionally active-publication scoped. The underlying solver can
still be used by D10B/Case-4 evidence exactly as before; active publication now
prevents material clamp amplification from reaching row consumers merely
because the clamp-adjusted ledger is algebraically exact.

## Result

The package closes the silent-publication class for WA by converting it into a
typed active-mode failure. It does not make WA active routing acceptable for
target-`dx` promotion; the first follow-on remains a true positivity-preserving
solver correction or a replacement explicit active solver policy.
