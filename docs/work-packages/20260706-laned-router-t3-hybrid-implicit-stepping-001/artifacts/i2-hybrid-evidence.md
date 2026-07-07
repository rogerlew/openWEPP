# T3-I2 — Hybrid integration: evidence

Status: **EXECUTED (strict rule; experimental opt-in)**. Evidence mode:
**Ran** (tests + H2637 runs this session) + Static.

## What landed

- `route_single_ofe_hybrid` (cascade.rs): bin-level hybrid driver — STRICT
  smooth bins (zero source AND zero upstream mass per 900 s bin) step
  implicitly at bin cadence; all other bins run the unchanged explicit
  scheme, span-stitched with exact state handoff (`set_state`/`depth_state`/
  `discharge_state`; entering explicit installs the implicit solve's own
  converged equilibrium discharges per the design §2.2 rule).
- Selector: `OPENWEPP_LANED_ACTIVE_IMPLICIT=1` composing with the active
  selector (runner env → `DirectLanedActiveConfig.hybrid_implicit`);
  manifest records `hybrid_implicit_stepping`; profile line records
  `solver_steps_implicit`.
- POST-REVIEW CORRECTION (rev 29, 2026-07-07): the Filippov filled-jump
  COMMIT described below was removed after the Codex code review (T3-H1) —
  the monotonicity argument in the rev-29 changelog proves a both-jump
  outcome is unreachable for genuine physics, so it now FAILS CLOSED as a
  solve failure; Steffensen is basin-locked (T3-H2); direct
  LOW-jump→HIGH-root and dust-accumulation vectors are retained tests. The
  H2637 hybrid evidence below was re-executed on the fixed tree (numbers
  updated in `review-disposition.md`).
- Solve hardening discovered by execution (contract-recorded at rev 28/29):
  Steffensen acceleration (deterministic, same fixed points — cold-seed
  fixed-point cost was the hybrid's first timing killer);
  bisection-interleaved outer solve (false position stalls on the convex
  `q~h³` rating); LOW-jump→HIGH-root recovery with fail-closed
  double-collapse (a genuine turbulent root can exist below the low-branch
  collapse — the steady-state regression caught the over-eager earlier
  filled-jump path);
  the `DRY_DEPTH·L` dust floor on the step-residual guard (femto-scale
  near-dry steps tripped the relative test with zero physical mass).

## Executed evidence (H2637, native-patched fixture, `taskset -c 4`)

| Case | User time (3 runs) | Books |
|---|---|---|
| plain active (rev 27) | `37.95 s` (sanity; parquet hash unchanged `21c54bf2…` — T3 code does not perturb the rev-27 path) | seam 5.0e-14 class |
| HYBRID strict | `37.02 / 37.15 / 37.22 s` | seam `1.7e-14`, cascade `6.4e-14`, identity `2.1e-13`; outlet `373,581 m³` vs plain `374,463 m³` (0.24 % end-state difference; both ledgers exact) |

All rev-27 day-closure hard-fails ran LIVE under the hybrid and passed on
all 610 routed days. Focused tests: hybrid bit-identity on all-explicit
windows (bin-for-bin `to_bits` equality), event-day ledger exactness +
fidelity bound, non-integral-window fail-closed, full implicit/hybrid/
cascade/kinematic/laned_active suites green; workspace clippy clean;
workspace full suite on the final I2 tree: **1417/1417 passed** (4 slow,
4 skipped, 583 s).

## Honest bottom line

- The STRICT rule (30 % step coverage) yields ~2 % endpoint improvement
  after Steffensen — the smooth phases it covers were partially max_dt-capped
  explicit stepping, so the I0 upper-bound arithmetic does not fully accrue.
- The real prize is the AGGRESSIVE rule (55.5 % coverage, the CFL-bound
  upstream-fed drain phases). It executed to lane 17 day 54 and failed
  closed on a REAL composition defect: a short explicit span between
  implicit bins cannot forward-redistribute a front-arrival terminal-bin
  deficit (`NegativeOutletBin`). The fix is a bounded solver-API extension
  (carry the deficit across span boundaries in the hybrid composition), NOT
  a mask change — named follow-on, first item in the worker handoff.
- Fidelity tolerances for implicit-phase stepping remain UNRATIFIED; the
  selector is contract-recorded as evidence-gathering (rev 28). The
  I1 ladder (dt=900 → ~0.43 per-bin L1 on pure recession; dt=300 halves it)
  plus the erosion tail-fold argument feed that ratification.
