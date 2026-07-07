# T3-I0 — Hybrid implicit KW stepping: scheme design

Status: **EXECUTED (design)**. Evidence mode: Static (derivations); the prize
measurement is `i0-prize-measurement.md`.

## 1. The implicit stepper (recession/homogeneous phases)

### 1.1 Discrete form

Kinematic wave: `∂h/∂t + ∂q/∂x = v`, `q = α(h) h^1.5` (α through the friction
fixed point; on smooth phases `v = 0` and often `q_up = 0`). Information
propagates strictly downstream (celerity `dq/dh > 0`), so BACKWARD-EULER +
first-order UPWIND is a lower-triangular system solvable by a single
downstream march — no matrix, one scalar nonlinear solve per cell:

For cells `i = 0..n-1` at step `t^n → t^{n+1} = t^n + Δt`:

```
F_i(h) = h − h_i^n + (Δt/Δx)·(q(h) − q_in,i^{n+1}) − v_i·Δt = 0
q_in,0^{n+1} = q_up^{n+1}   (prescribed boundary; 0 in pure recession)
q_in,i^{n+1} = q(h_{i−1}^{n+1})   (already solved — downstream march)
```

### 1.2 Existence, uniqueness, positivity (unconditional) — I1-corrected

WITHIN a friction branch, `q(h)` is continuous and strictly increasing, so
`F_i' ≥ 1`, the root is unique, `F_i(0) ≤ 0` gives existence, and the root
is `≥ 0` for ANY `Δt` — the scheme is **unconditionally
positivity-preserving** with a machine-exact ledger (no clamp class).
**I1 correction (measured + derived):** the `INV-OFEROUTE-002` regime
dispatch makes the FULL rating Z-SHAPED — in an overlap band both branch
equilibria exist and fixed-point basins split at `Q_c = 1000·ν`, so
"the converged equilibrium" is seed-dependent and the naive solve accepted
inconsistent `(h, q)` pairs (silent conservation leaks up to 0.24 m²/step).
Rev-28 draft rules: (i) deterministic basin-split seeding per branch;
(ii) LOW-branch preference with HIGH-branch fallback when the low rating's
jump swallows the root; (iii) NO exit path returns an unvalidated pair
(fail-closed step-residual guard). Evidence:
`i1-implicit-stepper-evidence.md`.

Solver: safeguarded Newton (bisection fallback on a maintained bracket
`[0, h_hi]`, `h_hi` grown geometrically until `F(h_hi) > 0`). Newton's
iteration uses `F' = 1 + (Δt/Δx)·c(h)` with the same true-celerity
evaluation the rev-26 CFL uses (numerical `dq/dh` through the friction fixed
point at first; the T3-I3 analytic celerity slots in later without changing
this design). Convergence tolerance: `|F| ≤ 1e-14·max(h_i^n, h) + 1e-18`
(residual-form, so the ledger identity below is exact to the tolerance).

### 1.3 Machine-exact ledger BY CONSTRUCTION

Multiply `F_i = 0` by `Δx` and sum over cells: the flux terms telescope, so

```
Σ_i (h_i^{n+1} − h_i^n)·Δx  =  q_up^{n+1}·Δt − q(h_{n−1}^{n+1})·Δt + Σ_i v_i·Δt·Δx
```

The discrete equation IS the mass balance: book `inflow = q_up^{n+1}·Δt`,
`outflow = q_out^{n+1}·Δt`, `rain = Σ v·Δt·Δx`, storage from the committed
state — booked-equals-actual holds to the Newton residual tolerance
(≤ 1e-14·h per cell), with NO clamp term and NO dissipation term. This is
*stronger* than the explicit scheme's rev-24 ledger (which needs the
telescoping-dissipation and stage-clamp analysis).

### 1.4 Accuracy posture and why it is acceptable on smooth phases only

Backward-Euler upwind is first-order in time and space and strongly
diffusive at fronts — unusable for the Case-4 shock class (this is exactly
why D10B's explicit TVD-MacCormack exists). On RECESSION/homogeneous phases
the solution is smooth and monotone (pure drainage: no characteristic
crossing, no front steepening — expansions only), the time scale is hours,
and the acceptance question is "does the drained-mass timing stay within
named tolerances", which the dt-refinement ladder against the
characteristics/upwind reference answers directly. Hourly bins (900 s
outlet sampling) are the consuming resolution; sub-bin shape error on a
smooth recession is bounded by the ladder evidence.

## 2. The hybrid switching rule (deterministic, forcing-derived)

**Rule (rev-28 draft):** a solver step is taken IMPLICITLY iff, for the
current step interval, (a) the seam source rate is ZERO on every cell of the
lane, AND (b) the upstream boundary series contributes zero mass
(`∫ q_up = 0` over the step). Otherwise the step is EXPLICIT
(TVD-MacCormack, unchanged). Both predicates are FORCING-derived (the hourly
seam series and the upstream conservative bin series), not state-derived —
the rule is deterministic, hysteresis-free, and knowable at step start; with
the D15A hourly breakpoints, regime boundaries coincide with step
boundaries, so no step straddles a regime change.

Rationale for the conservative rule: (i) it covers exactly the phases where
the implicit scheme's diffusion is harmless (no source, no incoming front —
only drainage of an existing monotone-decaying profile; any front already in
the domain has formed under explicit stepping and decays during recession);
(ii) it needs no tunable state threshold (a state-derived "smoothness"
switch would be a second numerics policy to adjudicate and could chatter);
(iii) it is where the cost lives (see `i0-prize-measurement.md`) — the 6 h
drain tail plus intra-window dry gaps step at CFL rates today
(`dt = 0.9·Δx/c`, seconds-to-tens-of-seconds while the mesh drains) for
water that changes on hour scales.

**Residual risk accepted at design time:** a lane with a *shock still
steepening* at the moment the source shuts off would see that front diffuse
under implicit recession. Mitigation: the switch happens at the source-off
boundary where Case-4-class fronts have already formed and peaked (the
Iwagaki cutoff is exactly this configuration — its post-cutoff phase is the
classic recession the characteristics oracle solves); the I2 acceptance runs
the FULL hybrid Case 4 against the ratified oracle tolerances, which is the
direct test of this risk. If it fails, the fallback design (recorded now) is
an explicit cool-down: remain explicit for K steps after source-off until
the homogeneous TV(q) transient is below the rev-25 bound, then go implicit.

### 2.1 Implicit-phase dt policy

`Δt_impl = min(next regime boundary, next sample-bin boundary, Δt_max^impl)`
with `Δt_max^impl = 900 s` (the outlet bin width — so the conservative bin
series stays exact per bin with no sub-bin apportioning error, and the
hydrograph export cadence is unchanged). No CFL constraint applies
(unconditional stability); accuracy at `Δt = 900 s` is adjudicated by the I1
refinement ladder, not assumed.

### 2.2 Switch-seam semantics

State is `(h_i, q_i)` in both schemes. Entering implicit: consume the
committed `h_i` only; `q` is re-derived as the equilibrium `q(h)` inside the
solve (the explicit scheme's committed `q_i = α_frozen·h_i^1.5` differs from
the converged equilibrium by the alpha-lag — first-order, the same lag the
rev-26 contract already adjudicates via Tseng). Exiting implicit (next
source-active step): commit `q_i = q(h_i^{n+1})` from the LAST implicit
solve's own friction evaluations so the explicit scheme's pre-step alpha
evaluation sees a consistent state (the same form it would see after any
explicit step). The seam is mass-neutral by construction (state is `h`;
`q` is diagnostic between steps). Recorded as a named seam; the I2 Case-4
run is its acceptance.

### 2.3 Ledger/evidence composition

Per-day books are unchanged (`inflow/rain/outflow/storage/clamp`): implicit
steps contribute clamp ≡ 0 and exact fluxes; the rev-27 active hard-fails
(supply, router-internal, seam cross-ledger, day identity) apply verbatim.
New surfaced counters: `solver_steps_implicit`, `implicit_newton_iters`,
`explicit_cooldown_steps` (0 unless the fallback rule activates).

## 3. Acceptance surfaces (I1/I2 gates)

1. **I1 recession oracle**: for a family of initial profiles (steady-state
   at several `v`, then source-off) on uniform and Case-4 material meshes,
   the implicit outlet series at `Δt ∈ {900, 300, 100, 30, 10} s` converges
   to the reference (the existing `iwagaki_oracle` upwind reference at
   fine dt, cross-validated by characteristics where applicable) with
   first-order rate; NAMED tolerances at `Δt = 900 s` proposed from the
   measured ladder (drained-mass-per-bin relative error and recession-curve
   time-shift), ratified at I2.
2. **I1 exactness**: per-step ledger closure ≤ 1e-12 relative (expected
   ~1e-15); zero clamps; positivity with no `max(0)` anywhere.
3. **I1 steady state**: an equilibrium profile under constant `v` is a
   fixed point of the implicit step to Newton tolerance (regression guard
   that the equilibrium friction law is identical to the explicit path's).
4. **I2 Case-4 hybrid**: the full hybrid run meets the rev-25/26 ratified
   oracle tolerances at every ladder rung (peak ≤ 5 %, `t_peak` ≤ 1.5 s,
   rise ≤ 2.0 s, non-diverging) — note Case 4's post-cutoff phase runs
   implicitly under the rule, so this directly stresses the switch seam.
5. **I2 H2637 active**: all rev-27 hard-fails green; conservation figures
   at current magnitudes; before/after endpoint + slot timing; a recorded
   hydrograph-surface fidelity delta vs the pre-change trajectory (hourly
   outlet bins, max/mean relative per-bin mass deltas on event days).

## 4. Rev-28 amendment draft (ratify at I2, not before)

Add to `SC-OFEROUTE-001`: (a) Algorithm item — the implicit
recession/homogeneous stepper (backward-Euler upwind, downstream-marching
safeguarded Newton, residual tolerance, exact-ledger property, unconditional
positivity); (b) Branch/guard row — the forcing-derived switching rule and
the explicit cool-down fallback; (c) Tolerance notes — the ratified
`Δt = 900 s` recession tolerances from the I1 ladder + the unchanged rev-27
day-closure hard-fails; (d) evidence note that bit-identity with the rev-27
trajectory is deliberately surrendered on active lanes under the NEW opt-in
numerics selector (`OPENWEPP_LANED_ACTIVE_IMPLICIT=1`, composing with the
active selector; default active behavior unchanged until a D16-class
promotion) — no shadow/default surface is touched.

## 5. What this does NOT do

No change to the explicit TVD-MacCormack scheme, the D-val oracle, the
friction menu, the seam sources, or any default/shadow surface. Tier-1
(analytic celerity/Newton-α/pow) and Tier-2 (resolution) remain separate
delegable increments (I3/I4) and compose multiplicatively with this tier.
