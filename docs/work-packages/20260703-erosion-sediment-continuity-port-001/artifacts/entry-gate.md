# Increment-0 — Entry Gate + Design (erosion sediment-continuity port)

Evidence: **Static** — legacy baseline read (`dac3c950`,
`/workdir/wepp-forest_260430_baseline/src/`), `SC-SED-001` invariants, current
direct-runtime erosion, and the git-recoverable prior Rust port (`a381702b^`).
Author: Claude Code, 2026-07-03.

## Design decision — RESOLVED: legacy hybrid (RK4 detachment + analytic deposition), normalized space

The march-vs-analytic question is **not a free choice; the math dictates it**:

- **Detachment region:** `dG/dx = Dc·(1 − G/Tc) + θ`, with `Dc = η·(τf − τc)` and
  `Tc`, `τf = (a x² + b x + c)^{2/3}` both varying nonlinearly in `x`. The `^{2/3}`
  shear nonlinearity has **no closed form** → the legacy integrates it with
  **RK4** (`runge.for`, confirmed dG/dx form at four sites). We port RK4.
- **Deposition region:** `dG/dx = φ·(Tc − G)` integrates to the **closed form**
  `G = Tc − D(x)·(x + q*)/φ` with `D(x)` analytic (`depeqs.for`,
  `depos.for:110`). We port the analytic solution.

So the port reproduces the legacy **hybrid**, per **ADR-0024 source-intent
authority** — not a re-derivation. The whole solve is in the legacy's
**nondimensional** space: `x ∈ [0,1]` (101 points `xinput(i)=(i−1)·0.01`), load
`G` normalized by `tcend`, parameters `η/τc/θ/φ` and shear/transport polynomials
`(a,b,c)`/`(atc,btc,ctc)`. Dimensional sediment is recovered at the OFE exit via
`tcend`. (Note: the existing `direct_erod13_fluxes` is a *dimensional* pointwise
kernel and is **not** the driver's inner loop — the driver is normalized-space,
matching legacy.)

## Algorithm (source-intent, condensed; full map in the extraction)

Per OFE, `route` marches slope segments `k = 2..nslpts`; per segment:
1. **Classify** shear vs critical with `xcrit` → `mshear ∈ {1..5}` (below-crit /
   above-crit / rising-cross / falling-cross / double-cross), crossing points
   `xc1,xc2` from `root(a,b,tauchk)`, `tauchk = τc^{1.5} − c`.
2. **Upper-boundary deposition rate** `dl` at segment top (`route.for:169-173`).
3. **If `du < 0` (deposition at top):** `depc` (constant `cdep`) → `depend`
   (`xdend`, Newton) → `depos`/`depeqs` (write `G`,`Tc`,`detach` profile via
   `G = Tc − D·(x+q*)/φ`). If deposition ends inside the segment, dispatch the
   remaining detachment sub-intervals by `mshear`.
4. **Else (detachment at top):** dispatch `erod` sub-intervals by `mshear`
   (splitting at `xc1/xc2`; the rill-erodibility arg `η` is passed only on
   above-critical sub-intervals, `0.0` below-critical so only interrill `θ`
   acts). `erod` runs the **RK4 101-point march**; if load reaches `Tc` mid-march
   (`ndep`), it secant-solves the onset `xdbeg` (`cross`) and hands to `depc`/
   `depos` for the deposition tail.
5. **Enrichment** (`enrich`) — deferred to Increment-3.

Fidelity details the port MUST reproduce (from source-intent):
- RK4 with `k3` reusing `k2`'s `Dc,Tc` (same `x+dx/2`); `ldnew` interrill floor
  `ldnew ≥ ldold + θ·dx` (`runge.for:219`); the `/detcom/` `Dc` memo (recompute
  only when the shear term or `η/τc` changes).
- `xcrit` `tauchk` is **un-clamped** in the pinned baseline (the `≥0` clamp is
  commented out, `xcrit.for:82`); the prior Rust port `erod19` *added* a `tauchk`
  and `xc1/xc2` clamp — **reconcile to un-clamped** (verify no-op on fixtures).

## Operand lineage (what the normalized solve needs → source)

The normalization (`param.for` / `xinflo.for`) needs, per OFE-day:
- `η = cntlen·kr·kradjf·shrsol/tcend`, `τc = tcadjf·shcrit/shrsol`,
  `θ = cntlen·detinr/tcend·(effdrr/effdrn)`, `φ = β·Vf/pkro` (β=0.5 rain/1.0 dry,
  `Vf = falvel(spgeff,diaeff)`, `pkro=(qout−qin)/slplen`), `ktrato = kt2/kt`,
  `tcend = kt·shrsol^{1.5}`, `strldn` (inflow load), `qostar`.
- These derive from: **runoff** (`qout/qin/pkro/peakro/watdur/Ie/te`) — already in
  the frame via `peak_runoff_shadow_projection` (`r7d8_erosion_inputs_with_runoff_authority`
  already wires `q_runoff/peakro/watdur`); **soil erodibility** (`kr/kradjf/kt/
  tcadjf/shcrit/detinr`) — from the soil/Chapter-7 projection; **slope/geometry**
  (`slplen/efflen/cntlen/shrsol` shear) — from the slope projection + WB16 shear.
  Increment-1 maps each to a concrete direct-frame operand; gaps become typed
  fail-closed requirements (no defaults).

The `DirectErod13Inputs` already carries most raw operands (`kr_s_m`, `kradjf`,
`tcadjf`, `shrsol_pa`, `tcend_kg_s_m`, `shcrit_pa`, `detinr_kg_s_m2`, `cntlen_m`,
`effdrr_m`, `effdrn_m`, `beta`, `vf_m_s`, `pkro_m3_s`, `q_m2_s`, `peakro`,
`watdur`) — the projection exists; the **driver** consuming them across `x` does
not.

## Recovery plan (reuse the prior port)
`git show a381702b^:…/hydrology_phase_erod19.rs` ports **`xcrit`/`mshear`,
`depc`, `depend`, `shear`, `root`, and the `param.for` `η/τc/θ/φ` derivations** —
recover as the starting scaffold (reconcile the clamp discrepancy above).
**Add** (the genuinely missing pieces): `erod` (RK4 101-pt march driver),
`runge` (RK4 step + interrill floor + `/detcom` memo), `depos`/`depeqs` (the
analytic deposition-profile writer), and the `route` segment control flow.

## Increment-1 scope + gate
- Single-OFE only; normalized route→erod/runge→xcrit→depc/depend/depos solve
  behind a **shadow projection** (`erosion_shadow_projection` exists); enable
  `wave1_enabled` for single-OFE.
- **Hard gate:** mass conservation `Σ detachment − Σ deposition = exported
  sediment` (denormalized), plus the SC-SED-001 pointwise invariants
  (INV-SED-001/002/003/006/007), on a **runoff+sediment-generating fixture**
  (McKenzie Bridge class — NOT MORAN-WY p313, which is erosion-inert).
- Contract mapping: driver continuity → INV-SED-001; detach branch →
  INV-SED-002; deposition branch/analytic → INV-SED-003; Tc → INV-SED-006;
  `η/τc/θ/φ` → INV-SED-007; payload → INV-SED-010 (Increment-1 emits totals,
  per-class deferred to Increment-3). INV-SED-008 (strip routing) + INV-SED-012
  (MOFE qin handoff) → Increment-2.

## Entry-gate status: RESOLVED
Design decided (hybrid, normalized, source-intent), algorithm mapped to source
lines, operand lineage traced to existing frame projections, recovery scaffold
identified. Increment-1 implementation may begin.
