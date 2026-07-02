# MOFEFID-D4 — Single-OFE Kinematic-Wave Solver (TVD-MacCormack)

Status: **EXECUTED — REVIEW-READY** (2026-07-02)
Campaign: [MOFEFID](../../planning/mofe-fidelity-campaign-strategy.md) Lane D.
Contract: `SC-OFEROUTE-001` (ratified). ADR: `ADR-0033` (ratified, narrowed).
Owner: Claude Code. Worktree: `mofefid-d4`. Activation: **opt-in / shadow-first**.

## What landed

`crates/openwepp-hillslope-orchestrator/src/ofe_routing/kinematic_wave.rs`:
a complete single-OFE 1-D kinematic-wave overland-flow solver —

- **TVD-MacCormack predictor/corrector** (SC-OFEROUTE-001 eqs. 8-14) with the
  dissipative flux-limiter term (`phi`/`Gr`/`Cf(Cr)`), a proper kinematic
  downstream outflow ghost, and upstream-inflow boundary conditions.
- **Space/time-variant friction** coupling: `alpha = C S_o^0.5` from the D3
  friction menu (eqs. 2-7), resolved through the implicit `Re(q)`/`Fr(q)`
  dependence by a seeded fixed-point (bootstraps from a dry start).
- **CFL-adaptive sub-timestep** (eq. 12): `Delta t` chosen so `Cr <= 1` at
  every cell; a residual CFL excursion fails closed.
- **Mass-balance ledger** with an *independent* outlet-flux measure and an
  explicit positivity-clamp accumulator, plus a Nash-Sutcliffe helper.

Shadow-first: **not wired into any production phase span** (grep-verified);
the default hillslope runtime does not reference it (`INV-OFEROUTE-010`;
default path byte-flat by construction).

## Validation (13 committed tests, no copyrighted data)

Contract-anchored to `INV-OFEROUTE-005/006/007` and the D3 kernels:

- **Conservation (INV-006):** the scheme conserves; the independent
  outlet-flux ledger residual is ~0.3% at 30 cells and the positivity clamp
  is exactly 0, so the residual is pure discretization — and it **converges
  with resolution** (2.83e-3 -> 1.09e-3 at 4x cells / 4x smaller dt), proven
  by a dedicated test.
- **CFL (INV-007):** `Cr <= 1` held on a steep+intense case despite a large
  `max_dt` cap (adaptive stepping); degenerate configs fail closed.
- **Steady state (Case 1 bare, INV-005):** outlet unit discharge -> `v * L`
  within 2%; a small (~6%) MacCormack rising-limb overshoot, TVD-damped.
- **Shock capture (Case 4 Iwagaki geometry):** a delayed discharge front
  reaches the outlet, positive/finite, oscillation-free, mass-conserving on a
  pure lateral pulse (no rain) over the 3-section decreasing-slope flume.
- **Boundary routing:** constant upstream inflow conveys to the outlet and
  conserves; **Nash-Sutcliffe** helper verified (Ef=1 perfect, Ef=0 mean).

## D-val (INV-OFEROUTE-011) scope — honest boundary

Formal Ef-vs-observed for the four Papanicolaou cases is **not** completed in
D4, and deliberately so:

- **Cases 1-3 need infiltration** (Green-Ampt rainfall -> rainfall-excess),
  which is `SC-RUNOFFPART-001`'s domain, not the routing solver's. That
  coupling is D5/integration scope.
- **Case 4 (Iwagaki)** is the pure-routing case (impermeable, lateral inflow,
  no rain), but the in-cache `Figure_4.xlsx` carries the paper's *model*
  series (Enhanced/Original WEPP), not cleanly-labeled digitized *observed*
  data; a faithful Ef needs the observed series from the original reference.

The Nash-Sutcliffe harness is implemented and tested; the D-val Ef run is the
D5/integration acceptance, using the exact per-section Case 4 lateral supply
and the digitized observed series. This package validates the routing solver
on the physics it owns (conservation, CFL, steady-state, shock, convergence)
and does not manufacture an Ef it cannot faithfully compute.

## Gates

- `ofe_routing` 13/13; full orchestrator suite green; fmt/clippy `-D warnings`
  clean; solver shadow-first (default path byte-flat).

## Next

D5: OFE-by-OFE cascade (per-OFE hydrograph handoff over the
`INV-RUNOFFPART-029` seam) + the `GAP-OFEROUTE-003` runon re-infiltration
reconciliation with DC01; then D-val Ef with infiltration coupling and
observed series.
