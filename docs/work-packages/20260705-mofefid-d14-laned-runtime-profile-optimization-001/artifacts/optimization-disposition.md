# Optimization Disposition

Status: **D14-S3 SELECTED** — dispositions assigned from the measured
D14-S2 profile (`slot-timing-evidence.md`); implementation evidence added at
D14-S4.

Behavior-preservation bar for this package: a candidate is admissible only if
it is **bit-identical** on (a) default/off protected outputs, (b) the
shadow-on protected outputs, and (c) the shadow `laned_shadow` manifest
diagnostic values recorded in `baseline-timing.md`. Timing-only surfaces
(new opt-in profiling diagnostics) are exempt from (c) by construction when
disabled by default.

## Candidate register (Static pre-registration, 2026-07-05)

| ID | Candidate | Mechanism | Bit-identity argument | Status |
|---|---|---|---|---|
| OPT-1 | Single alpha evaluation per cell per step | `KinematicWaveSolver::run` computes `CellParameters::alpha` for the same `(h, q, I)` state three times per cell per step: in `cfl_dt`, in the CFL-evidence loop, and again inside `step`. Compute the per-cell alpha vector once per step and reuse for dt selection, Courant evidence, and the step math. | `alpha(h, q, I)` is a pure function of pre-step state; all three call sites see identical inputs, so one evaluation reused yields identical f64 values at every consumer. Validation/failure ordering is preserved. | **ACCEPTED** — measured basis: `solver_cfl` slot 34.77 s (51.4 %) is redundant re-evaluation; 24.1 alpha evals/step where ~10 suffice; perf shows three inlined copies of the alpha fixed-point loop. |
| OPT-2 | Per-step workspace buffers | `step()` allocates 8 `Vec<f64>` per call (`alpha`, `v`, `h_pred`, `q_pred`, `h_corr`, `averaged`, `gr`, `h_next`) — 10.3 M steps → ~83 M allocations per H2637 run. Hoist into reusable solver workspace allocated once per solver. | Allocation location does not change any computed value; loop bodies and operation order unchanged. | **ACCEPTED** — measured basis: malloc/free ~2 % in perf; small but real, and the workspace restructure is required by OPT-1 anyway. |
| OPT-3 | Hoist skin rain term per step | `skin_resistance_shen_li` computes `3393.0 * I.powf(0.407)` on every friction evaluation (≈ 4 per alpha eval × 249 M evals); `I` is constant within a step (single fetch at step start). Hoist the term once per step and pass through. | Same operations on the same operand in the same order (`powf`, `mul`, `add`, `div`); reuse of an identical intermediate f64 is bit-identical. Unit test asserts bit-equality against the canonical `skin_resistance` over a value grid. | **ACCEPTED** — measured basis: libm `pow` is 35.6 % of the run; the skin `I^0.407` is the highest-multiplicity pow call site. |
| OPT-4 | Replace `h.powf(1.5)` with `h * h.sqrt()` | Cheaper math for the depth-discharge power. | **REJECTED (pre-registered):** not bit-identical (differs in ULPs), would move shadow conservation diagnostics — output-affecting without contract authority. | rejected |
| OPT-5 | Avoid `seg.mesh.clone()` in `run_cascade` | Mesh is cloned once per OFE per routed day. | Behavior-preserving but measured negligible. | **REJECTED — profile-unjustified:** `solver_setup` slot measured 0.004 s (~0 %). |
| OPT-6 | Collapse double dyn-closure dispatch in cascade forcing | Two boxed-closure indirections per forcing fetch per cell per step. | Behavior-preserving; accept only if the profile shows call overhead is material. | **REJECTED — profile-unjustified:** forcing-fetch dispatch is inside the ~0.78 s cascade residual (~1 %); the seam-rate closure symbol is 1.28 % in perf. Not worth the structural churn this package. |
| OPT-7 | Numerical-method / tolerance / resolution changes (cells, `sample_dt`, `max_dt`, CFL target, limiter, fixed-point iteration count) | — | **REJECTED (pre-registered):** these are D10 `GAP-OFEROUTE-005` shock-numerics surfaces or physics-affecting controls; excluded by package scope regardless of timing benefit. | rejected |

## Measured profile basis

Recorded in `slot-timing-evidence.md` (D14-S2 pre-opt + §4a post-opt).

## Final dispositions (D14-S4, Ran)

All three accepted candidates landed together (they share the solver
restructure):

- **OPT-1 (landed).** `cfl_dt` and the CFL-evidence alpha recomputations are
  fused into `KinematicWaveSolver::prepare_step_alpha`: one alpha evaluation
  per cell per step into the step scratch, reused by dt selection, Courant
  evidence, and `step()`. Intensity is fetched and validated once per step,
  before any consumer.
- **OPT-2 (landed).** `StepScratch` (8 reusable per-solver vectors) replaces
  the 8 per-step `Vec` allocations; every slot is written before read within
  the same step.
- **OPT-3 (landed).** `friction::skin_rain_term` +
  `friction::skin_resistance_with_rain_term` hoist `3393 I^0.407` to once
  per step. `CellParameters::equivalent_friction` delegates to the
  rain-term form (one friction-menu implementation); the canonical
  `skin_resistance`/`skin_resistance_shen_li` kernels keep their standalone
  equation-as-stated forms, so the regime dispatch exists in two textual
  places — the bit-identity grid test below binds them together.

Tests and parity evidence:

- `friction::tests::skin_rain_term_dispatch_is_bit_identical` — bit-level
  equality of the hoisted dispatch against canonical `skin_resistance` over
  a 6×5×9 operand grid including the negative-intensity NaN path and both
  Reynolds regimes (Ran, passing).
- Full `ofe_routing` + `laned_shadow` focused suites pass unchanged
  (64 tests) — including the D4 conservation/CFL/steady-state/shock physics
  vectors, cascade conservation/handoff vectors, seam gate fixtures A/B, and
  the D14 profiling tests.
- H2637 parity: protected outputs and the `laned_shadow` manifest block are
  bit-identical pre/post optimization; the solver trajectory is
  step-count-identical (`protected-output-evidence.md`).
- Endpoint effect: shadow-on 67.6 s → 29.8 s wall (2.27×); overhead
  65.3 s → 27.5 s (−58 %) (`baseline-timing.md`).

Explicitly not landed (besides the rejected rows above): stateful
upstream-interpolation cursors and celerity caching — both plausibly
bit-identical but individually worth ≲ 1–3 % on the measured profile; left
for a future package if D15 needs more headroom.
