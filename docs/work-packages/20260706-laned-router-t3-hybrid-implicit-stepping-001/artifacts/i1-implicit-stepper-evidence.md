# T3-I1 — Implicit recession stepper: evidence

Status: **EXECUTED (solver tier)**. Evidence mode: **Ran** (test suite +
diagnostic ladder this session) + Static (derivations).

## What landed

`ofe_routing::implicit_recession` — backward-Euler + first-order-upwind KW
stepper: downstream-marching scalar safeguarded secant/bisection per cell,
deterministic basin-split equilibrium rating, fail-closed residual guard
(exact-ledger-by-construction means a material step residual IS a solve
failure), typed `RoutingError::ImplicitSolveNonConvergence`. Supporting:
`CellParameters::equilibrium_discharge_converged` (converged equilibrium
with seed control) and `KinematicWaveSolver::depth_state` (the hybrid
handoff seam accessor). ZERO production consumers in this increment; the
explicit scheme is untouched (D10B oracle suite re-run green).

## Discovery 1 — the regime-dispatched rating is Z-SHAPED (bistable)

`INV-OFEROUTE-002`'s skin dispatch is discontinuous at `Re = 1000` (`f`
drops laminar→turbulent, e.g. 0.5 → 0.142 at `k_o = 500`), so in an overlap
depth band BOTH a laminar and a turbulent equilibrium exist; the fixed-point
basins split exactly at `Q_c = 1000·ν`. Consequences: (a) a warm-seeded
"converged equilibrium" is HISTORY-DEPENDENT — the first implementation
produced silent per-step conservation leaks up to 0.24 m² (drained mass
EXCEEDING initial storage) via internally-inconsistent `(h, q)` pairs
accepted at bracket collapse; (b) the EXPLICIT scheme's frozen 4-iteration
cap rides over this structure without resolving it (a recorded property,
now understood, not a defect there). Resolution (rev-28 draft rule):
deterministic basin-split seeding (`Q_c·1e-3` / `Q_c·1e3`) makes each
branch a pure function of depth; the cell solve prefers the LOW (laminar)
branch and falls back to the HIGH branch when the low rating's jump
swallows the root; NO exit path may return an unvalidated pair.

## Discovery 2 — the validated-pair rule is load-bearing

The fail-closed residual guard (|residual| > 1e-9·scale ⇒ typed error)
converted the silent leak class into visible failures during development —
the same guard-catches-real-defect pattern as D15A's seam check. It stays
in production posture.

## Acceptance ladder (Ran; diagnostic test retained as `#[ignore]`)

Recession from the explicit solver's own committed post-spin-up state
(identical initial conditions via `depth_state`), 6 h window, zero source;
reference = explicit TVD-MacCormack at `max_dt = 2 s`; per-900-s-bin L1
error normalized by reference drained mass:

| cells | S0 (m²) | explicit drained | implicit dt=900 | 300 | 100 | 30 | 10 |
|---|---|---|---|---|---|---|---|
| 10 | 0.3299 | 0.3070 | L1 0.427 | 0.213 | 0.113 | 0.073 | **0.061** |
| 20 | 0.3269 | 0.3043 | 0.417 | 0.197 | 0.094 | 0.051 | **0.038** |
| 40 | 0.3303 | 0.3078 | 0.404 | 0.184 | 0.080 | 0.037 | **0.023** |

- Per-step ledger residual ≤ 1.7e-14 m² at EVERY rung (machine-exact, no
  clamp term — the scheme is unconditionally positivity-preserving).
- Monotone first-order dt-convergence at every mesh; the dt→0 limit
  approaches the explicit reference as the mesh refines (0.061 → 0.038 →
  0.023 ≈ O(1/n)) — cross-scheme CONSISTENCY, the correct acceptance frame
  (the two schemes' spatial operators differ at fixed n; dt-refinement
  cannot close that gap, which the first acceptance draft got wrong).
- Drained totals agree with the explicit reference to 1.6 % (n=10) → 0.4 %
  (n=40).
- Outer-solve cost ≈ 2.9 iterations/cell/step at dt=900 (7,017 iterations
  for 24 steps × 10 cells) — each iteration one converged equilibrium; on
  covered phases this replaces hundreds of CFL-bound explicit steps
  (~10-20x cheaper per the I0 prize arithmetic).
- Unit vectors green: exact ledger + positivity; upstream-inflow booking
  exactness (aggressive-rule readiness); steady-state fixed point
  (equilibrium profile preserved to 1e-6, outlet = v·L).

## Tolerance posture handed to I2

The dt=900 per-bin redistribution (~0.43 of drained mass; backward-Euler
front-loads recession) is the measured price of hour-scale implicit
stepping on this fixture; dt=300 halves it (~0.21). I2 must ratify the
implicit-phase dt and tolerances against the Case-4 hybrid oracle run and
the H2637 hydrograph surface — candidates: dt=300 default, or dt=900 with
the erosion tail-fold argument (recession-tail mass beyond hour 24 folds
into one weight anyway). Not pre-decided here.
