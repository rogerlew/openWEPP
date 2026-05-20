# 1.5D Top-Slope Trapezoidal Abstraction (Concept Backlog)

## Status
- `state`: backlog
- `maturity`: concept / planning only
- `default_path`: not eligible
- `date`: 2026-05-12

## Why this exists
Current hillslope abstractions with constant plan width can under-represent
concentration behavior for convergent headwater topography. A 1.5D geometry
model can encode this effect while keeping a 1D conservative routing kernel.

## Scope
Propose an optional hillslope hydrology kernel that:
- represents plan-view convergence with a prescribed width function `W(x)`;
- solves first-principles kinematic-wave continuity in conservative form;
- remains numerically 1D (no 2D mesh, no unstructured solver).

## Non-goals
- No claim of default replacement for baseline production kernels.
- No immediate sediment-kernel rewrite in this phase.
- No ABI/contract break in existing hillslope or watershed CLIs.

## Governing constraints (openWEPP policy)
- openWEPP default kernels are governed by openWEPP science contracts.
- This concept is new physics abstraction and must remain experimental until:
  - top-down contractization exists, and
  - parity/validation gates are explicitly defined and accepted.

## First-principles formulation
Coordinate:
- `x in [0, L]`, `x=0` upslope ridge, `x=L` channel outlet.

Linear converging width:
- `W(x) = W_top - ((W_top - W_bottom) / L) * x`

Conserved state:
- `A(x,t) = h(x,t) * W(x)`

Continuity with rainfall-excess source:
- `dA/dt + dQ/dx = r_e(x,t) * W(x)`

Kinematic closure (wide-sheet Manning family):
- unit discharge: `q = alpha * h^m` (nominal `m=5/3`)
- total discharge: `Q = q * W = alpha * A^m * W^(1-m)`

Rectangular-limit invariant:
- when `W_top = W_bottom`, equations reduce to constant-width 1D form.

## Minimal parameterization
Required:
- `L` (flow length)
- `W_top`
- `W_bottom`
- slope and roughness terms already required by kinematic closure

Recommended invariant form for setup UX:
- `L`, `area_plan`, `convergence_ratio = W_top / W_bottom`
- derive `W_top`, `W_bottom` from area + ratio to avoid accidental area drift.

## Numerical plan (phased)
Phase A: hydrology-only prototype
- conservative finite-volume discretization on 1D cells;
- monotone upwind flux for `Q(A,W)` with CFL timestep control;
- mass-balance accounting with explicit residual tracking.

Phase B: accuracy and robustness
- rectangular-limit equivalence tests;
- convergence sweep (`ratio = 1..Rmax`) with fixed area;
- stability sweep over storm intensity, slope, and roughness;
- optional higher-order reconstruction only after baseline stability.

Phase C: system integration assessment
- evaluate handoff fidelity to watershed routing interfaces;
- determine if event summary fields are sufficient or if optional richer
  hydrograph metadata is required for experimental mode.

## Acceptance criteria to promote from backlog
1. Closure:
   - per-event and per-run mass residual is bounded by explicit tolerance.
2. Limit consistency:
   - `W_top = W_bottom` reproduces baseline rectangular kernel within tolerance.
3. Response monotonicity:
   - increasing convergence ratio (fixed area/storm) yields earlier/higher peak
     without artificial volume gain.
4. Contract safety:
   - experimental mode does not change default ABI or baseline outputs.
5. Governance:
   - promotion path defined with explicit openWEPP contract ownership.

## Key risks
- parameter confounding between geometry, roughness, and infiltration;
- hydrograph-shape loss if interface compression is too coarse;
- sediment-coupling drift if hydraulic changes are introduced without matching
  erosion/deposition model updates.

## Open questions
1. Should experimental output include optional hydrograph ordinates or moments
   beyond `(volume, duration, peak)`?
2. What convergence-ratio bounds are physically defensible for target domains?
3. Should `W(x)` remain linear only, or allow constrained piecewise-linear
   profiles in a later phase?
4. What comparator evidence strategy is acceptable before contract promotion?
