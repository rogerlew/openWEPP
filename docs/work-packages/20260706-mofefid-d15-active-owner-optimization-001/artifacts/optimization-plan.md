# Optimization Plan (D15A-S3)

Status: **EXECUTED** (plan authored before implementation; results in
`optimization-results.md`).

Evidence mode: Static (candidates derived from the Ran S2 profile); the
bit-identity claims below are proven by the S4/S5 preservation evidence.

Preservation bar (same as D14): protected outputs, the manifest `laned_shadow`
diagnostics block, the hydrograph/step counters, and every routed diagnostic
must be **bit-identical** pre/post. Any candidate that changes numerical
method, `dt` selection, source authority, closure tolerance, activation
semantics, or any output value is rejected.

Numbering continues D14's OPT-1..4 (OPT-4 = `h^1.5 → h·sqrt(h)` was REJECTED
there as not bit-identical and stays rejected).

## Selected candidates

| ID | Change | Why bit-identical | Expected effect |
|---|---|---|---|
| OPT-5 | `CellParameters::alpha` returns the pair `(alpha, q = alpha·h_pow)` it already computed (0-paths return `(0.0, 0.0)`); `prepare_step_alpha` reuses `q`/`q2` instead of recomputing `alpha·h.powf(1.5)` at the base and perturbed depths. | The fixed point's last iterate computes `q_new = alpha·h_pow` with the exact operands the caller re-derives; `0·h_pow = +0.0` for every zero-alpha path (finite positive `h_pow`), so the zero cases carry the same bits. | removes ~15.7 redundant `powf`/step (the S2 table's second row) |
| OPT-6 | Cache the pre-step per-cell `q` from `prepare_step_alpha` in the step scratch; commit the state BEFORE the homogeneous TV(q) diagnostic and compute `tv_before` from the cached pre-step `q`, `tv_after` from the just-committed `discharge_m2_s`. | `q_at(depth, i) = alpha[i]·depth[i].max(0).powf(1.5)` is exactly the cached pre-step `q` (depths are non-negative by construction, so `.max(0)` is the identity; dry/zero-alpha cells give `+0.0` on both forms), and `q_at(h_next, i)` is exactly the committed discharge expression. Reordering commit before the diagnostic changes internal error-path ordering only (`NonFiniteState` days abort the run either way; no output exists in that case). | removes up to 4(n−1)=36 `powf` per homogeneous step (drain-tail and dry-source steps) |
| OPT-7 | Hoist `self.slope.sqrt()` out of the alpha fixed-point loop (loop-invariant). | Same deterministic `sqrt` value every iteration. | ~3 `sqrt` per alpha call |
| OPT-8a | `equivalent_friction_with_rain_term`: compute the Froude number lazily inside the wave branch (`D_r > 0 && h < D_r`) instead of unconditionally. | `fr` is consumed ONLY by that branch; elsewhere its value is dead. | removes powi+sqrt+div per friction evaluation on fixtures with `D_r = 0` (all H2637 lanes) |
| OPT-8b | `vegetation_resistance_katul`: compute `L_c` first and return `0` before computing `beta` when `L_c <= 0`. | Both orderings return exactly `0.0` in that branch; `beta` is dead there. (With `LAI > 0` the `beta <= 0` guard is unreachable; it is kept.) | removes sqrt+div per friction evaluation when `C_d,veg = 0` (all H2637 lanes) |
| OPT-9 | Precompute the per-face material-interface flags (`cells[f] != cells[f+1]`) once per `run` (the mesh is immutable during a run) and index the flag array in the dissipation and TV loops. | Same comparison, hoisted; per-run mesh immutability is structural (`KinematicWaveSolver` owns the mesh; nothing mutates cells after construction). | removes 2×(n−1) 9-field struct compares per step |

## Rejected candidates

| Candidate | Rejection reason |
|---|---|
| Analytic celerity (`3 q/h` laminar, `(5/3) q/h` Manning) instead of the numerical perturbed fixed point | numerical-method change (rev-26 binds the numerically-evaluated true celerity; the analytic forms are the *rationale*, not the bound scheme) |
| Warmer fixed-point seeds (previous-step alpha; perturbed seeded by extrapolation) or a relaxed convergence tolerance / different iteration cap | changes the iterate path ⇒ different final bits |
| `h.powf(1.5) → h·h.sqrt()`, `powf(0.45)` polynomial approximations, fast-math | not bit-identical (D14 OPT-4 precedent) |
| Skipping the perturbed evaluation on "nearly-uniform" cells | changes celerity ⇒ changes `dt` selection ⇒ different trajectory |
| Shortening/skipping the 6 h drain tail or early-exiting drained windows | changes the routed window ⇒ changes routed diagnostics (outputs) |
| Reducing the TV(q) diagnostic's coverage (e.g. sampling) | weakens a contract evidence surface (`INV-OFEROUTE-006`-adjacent bound) |
| Caching alpha across steps for unchanged `(h, q, I)` | state changes every step except exact-dry cells, which already early-return; complexity for no measured win |
| Cursor-cached upstream bin-series integration | measured ≤ 2.7 % (cascade residual); not worth hot-path state |

## Step-count posture (recorded before implementation)

The +64 % step count is the rev-26 CFL trajectory itself. No behavior-preserving
optimization can reduce it, and no candidate above touches `dt` selection. The
S5 adjudication therefore evaluates: (optimized per-step cost) × (mandated step
count), against the D14 budget — with the explicit position that the D14
`29.9 s` budget was measured on the pre-rev-24 scheme that D10B proved
latently unstable (true Courant ~1.8) and under-dissipative, so the budget's
basis is superseded; the package adjudicates the corrected-scheme cost on its
own evidence rather than treating the defective scheme's cost as authority.
