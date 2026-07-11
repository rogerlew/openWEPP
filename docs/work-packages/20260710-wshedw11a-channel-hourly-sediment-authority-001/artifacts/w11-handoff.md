# W11 Implementation Handoff

Status: `EXECUTED`

Evidence mode: `Static` (this handoff names contract authority; the cited
rows exist in SC-ROUTE-001 **v53** — v51 as verified by the package's dual
verification, plus the v52/v53 amendments from the two Codex post-hoc
cycles).

`WSHED-W11-HOLD-001` stands lifted (Codex `RATIFIED`, cycle-2
re-confirmation, 2026-07-10). Canonical authority for time-resolved
channel sediment routing now exists in `SC-ROUTE-001` v53. W11 resumes at
its contract-first phase (Phase B) with **no executor science choices
remaining** — every item below is a contract citation, not a decision.

## Resume point

Resume `20260710-wshedw11-channel-network-hourly-water-sediment-routing-001`
at Phase B (contract-derived tests before production code, per the
authoring procedure's contract-first sequencing). The water-series
authority (`ipeak` 3-5 `wshchr` grid/state/dependency semantics) was
already established by W11 Phase A and is unchanged.

## Exact authority map

| Implementation concern | Binding authority |
|---|---|
| When the interval lane runs | `INV-ROUTE-015` biconditional (mandatory when the predicate holds); Activation section of the W11A addendum (topological evaluation; impoundment exclusion; downstream mixed-authority fail-closed) |
| Network dependency authority | `INV-ROUTE-005(a)` dependency-authority definition (active-interval-lane channel egress is the only non-hourly form) |
| Solve per interval | `INV-ROUTE-016`: the WSHEDIMPL18-41 migrated segment-solve lanes (WS20/WS21 runtime families) invoked per `dtchr` interval at interval operands; event-scalar operand substitution is a typed hard failure |
| Interval operands | W11A addendum operand table + projection formula (00:00-anchored exact interval overlap; day-level class-fraction blend per `SC-SED-001#GAP-SED-008` — do not treat the uniform split as enriched timing) |
| Hydraulic profile (v52/v53) | `INV-ROUTE-016` unique operand map: `qe(it) := q1(it)`, `qt(it) := qin(it)`; the published wave total `qlat(it)` (`m^3/s`) builds `leff(it)` ONLY; the solve's lateral operand is the derived per-unit-length `qlat_eff(it) := qe(it)/leff(it)` (never the raw total, never total/`lc`); event-peak fractions/event-duration rates invalid; storage deliberately unreconciled (`qt + qlat - q1`, totals); vector 11 is the anti-alias gate |
| Erosion clock/normalization (v52) | `t_exp(it)` fills every legacy `timsh` slot; `t_norm(it) := dtchr` fills every legacy `tb` denominator slot (the factor 2 retires); `d_i` = baseline `di` (`dcap.for:166`); `rho_soil` = in-place bulk mass density (`wtdsoi` provenance, lbm) |
| Unit handling | Addendum unit-bridge declaration: SI operands cross at the migrated `chnrt` conversion lineage sites; TOL closures evaluated on the SI side; **no new conversion constants** |
| Geometry state | `INV-ROUTE-017`: monotonic carry, in-time-order application, run-start + primary-tillage-only reseeds, non-narrowing/non-refilling |
| Widening clock | `INV-ROUTE-018`: **WEPP-adapted lineage realization** (linear rate, `1.0176`-modified exponential, fitted `f(x_b)`; CREAMS equations are structural provenance only); `timpot`/`timex` budget partition at layer contact; detachment gates on average soil shear, widening on boundary shear; triangular surrogate banned on this lane |
| Mass closure | `INV-ROUTE-019` + `TOL-ROUTE-006/007/008` (incl. the zero-mass carve-out); detached mass is the constructive geometry derivation |
| Degenerate states | `INV-ROUTE-020`: zero-flow interval deposition at the reused `1e-12 m^3 s^-1` floor; storage-attributed sediment zero-by-construction; geometry-only cross-day state; non-covering grid hard failure |
| Guards | `WKERNEL-WS10-CHANNEL-E-001..003` family throughout (guard-map rows 015-020) |
| Tests | The eleven contract-derived vector obligations (v52) are W11's implementation gates; vector 1's comparator construction is fully pinned (profile, exposure, and normalization operands); vectors 10(b)/(c) gate the GAP-ROUTE-014 terminal corrections; vector 11 is the hydraulic anti-alias |
| Comparator posture | Known-divergence note (Widening Clock section): interval-vs-surrogate deltas are Investigation-tier (ADR-0017/ADR-0036 D5), never acceptance gates |

## Do-not-do list (unchanged or newly bound)

- Do not start a water-only runtime or weaken the dependency guard (carried
  from the W11 hold handoff).
- Do not reconstruct channel hourly output from event scalars.
- Do not run the event-scalar sediment solve on an activated channel, or
  the interval solve on a non-activated one (`INV-ROUTE-015`).
- Do not introduce a sediment-specific flow threshold, a re-erodible bed
  store, a suspended cross-interval/cross-day pool, or any new unit
  conversion constant.
- Do not amend `SC-INFILE-HBP-001` — no schema change is authorized; the
  per-class-hourly channel remains a future additive extension
  (`GAP-SED-008`).
- **Do not reuse the two GAP-ROUTE-014 migrated terminals uncorrected**:
  the capped-widening and post-contact subcritical-boundary-shear terminals
  in `01_ws22_ws23_ws26_detachment.rs` (and their lock-in tests) must be
  corrected to pinned `dcap.for` behavior (`dcap.for:238-261` and
  `:210-215, 173-190`) before the interval lane activates; vectors
  10(b)/(c) are the correction gates.

## Kernel-profile note

W11's implementation package changes production kernel behavior and must
satisfy `docs/specifications/science-contracts/kernel-process-contract-profile.md`
at its own gate time (this authority package did not trigger it: docs-only).
