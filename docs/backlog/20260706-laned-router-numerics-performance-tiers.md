# Lane D router numerics — tiered performance revision

Status: concept (backlog). Operator-requested 2026-07-06 after D15A closed
opt-in activation ("shippable as-is; what are options for revising the
numerical methods for performance … this will likely run millions of
hillslopes a year").

## Why this exists

D15A exhausted the bit-identity optimization headroom (`optimization-results.md`:
remaining ≤ ~5 %) and adjudicated the corrected-scheme cost: H2637 active
endpoint `37.4 s` user (2-year, 19 OFE) ⇒ a 100-year climate is ~31 CPU-min
per hillslope. At fleet scale (10⁶ hillslopes/year) every 2x is thousands of
CPU-days. All remaining gains require NUMERICAL-METHOD changes — bits change,
so each tier is a contract-first numerics decision validated by the D10B
acceptance harness (Iwagaki-primary oracle convergence ladder, exact
conservation gates, TV bound, H2637-class sweep, and now the rev-27 active
day-closure hard-fails). That harness is what makes these adjudicable.

Cost structure today (D15A S2 profile): ~65-70 % alpha/friction fixed-point
work (302M calls; the rev-26 true celerity costs a SECOND full fixed-point at
a perturbed depth ≈ 44 % of alpha calls; the fixed point's laminar contraction
~0.5 means the 4-iteration cap binds without meeting its own 1e-12 tolerance);
~27 % libm `pow` (`h^1.5` ×3 sites per cell-step; `Re^0.45` turbulent limb);
step count (16.9M shadow / CFL-bound) is fixed by the trajectory.

## Tier 1 — local numerics (est. combined ~2.5-4x on the router; low risk)

1. **Analytic celerity** (drop the perturbed-depth second fixed-point):
   closed forms per limb — Manning `(5/3)q/h`, laminar `3q/h`, Hirsch
   turbulent `≈1.94 q/h`; the additive menu's `dq/dh` follows from implicit
   differentiation of `q = α(q,h)h^1.5`. MORE accurate than the current
   finite difference (removes the `dh` heuristic). Est. 1.3-1.5x.
2. **Newton for α instead of the capped fixed point**: quadratic convergence
   reaches tolerance in 1-2 iterations where today 4 iterations still fall
   short — cheaper AND more accurate. Est. 1.4-1.7x (overlaps 1).
3. **`h^1.5 → h·h.sqrt()`** + bounded-minimax/vector `Re^0.45` (D14's
   rejected-for-bit-identity OPT-4): ~1 ulp deltas, 2-5x cheaper than libm
   `powf`. Est. 1.2-1.35x.

Ship as ONE package (same ~60 lines, one validation run). Projected active
H2637: ~37 s → ~12-16 s.

## Tier 2 — production mesh-resolution adjudication (est. ~3-4x; pure fidelity call)

Cost ∝ n² in cells/OFE (dt ∝ dx, work/step ∝ n). 10 cells is the D-val
WORKING resolution, not a proven production requirement; the oracle ladder
directly quantifies peak/timing error at 5 cells and conservation is exact at
any resolution. If 5 cells meets a NAMED fidelity tolerance for the
production regime, this is a config + contract adjudication, not a scheme
change. Projected: → ~4-6 s, i.e. router ≈ parity with the rest of the
hillslope run (~2.5 s).

## Tier 3 — scheme-level (est. 5-20x on covered phases; real design risk)

4. **Hybrid implicit stepping**: KW is downstream-marching, so an implicit
   first-order scheme needs one scalar Newton per cell per step (O(n), no
   matrix) and is unconditionally stable (dt → sample resolution). Numerical
   diffusion will likely fail Case-4 shock tolerances at large dt, so the
   practical shape is HYBRID: explicit TVD-MacCormack during source-active /
   shock hours, implicit during recession and the 6 h drain tail (smooth,
   homogeneous — a large share of steps on event days). Method-switching
   criteria become contract text; full D-val re-adjudication required.
5. **SIMD-batched friction evaluations** (vector libm across cells):
   ~1.2-1.4x, orthogonal; bits change only via vector-libm ulp.

Rejected: f32 (the 1e-9..1e-12 conservation ledgers preclude it); silently
folding Tier-2 resolution into Tier-1 tuning (keep each gain attributed to
its own adjudicated decision or the fidelity story becomes unauditable).

## Promotion criteria (per tier)

- Contract-first amendment to `SC-OFEROUTE-001` naming the method change and
  its acceptance tolerances BEFORE code.
- Iwagaki-oracle ladder within the rev-25/26 ratified tolerances at every
  rung, non-diverging; exact booked-ledger conservation; TV(q) transient
  bound; 19-OFE class-fixture sweep exact.
- H2637 active endpoint + rev-27 day-closure hard-fails green (supply, router
  books, SEAM cross-ledger, day identity).
- Recorded before/after timing with the D14/D15A two-instrument protocol
  (persistent slots + perf), and a named fidelity delta vs the pre-change
  trajectory (peak/timing at the hydrograph surface), since bit-identity is
  deliberately given up.
- Sequencing: after D16 default-promotion adjudication (or alongside it if
  fleet economics demand), Tier 1 → Tier 2 → reassess before Tier 3.
