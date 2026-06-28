# Paradigm 2 — Multilayer Snow Physics: Implementation Requirements Specification

- Status: **investigation / specification** (informs the defer-vs-commit decision; no code)
- Author: Claude Code, 2026-06-28
- Relates: [snow-frost-fidelity-strategy](snow-frost-fidelity-strategy.md) §10.3 step 9 (paradigm
  fork); [paradigm assessment WP](../work-packages/20260628-snow-density-paradigm-assessment-001/);
  ADR-0025 (array-native runtime), ADR-0026 (winter-column sub-solver), ADR-0028
  (observed-data admission), ADR-0011 (contract-first), ADR-0019 (output surface);
  backlog [stream water temperature](../backlog/20260627-stream-water-temperature-surface-energy-balance.md),
  [canopy snow interception](../backlog/20260627-canopy-snow-interception-sublimation.md)

## 1. Why scope this before deferring

The paradigm assessment recommended deferring Paradigm 2 (multilayer) as
"disproportionate for a 15-cell density residual." That calculus changes once the
multilayer snowpack is recognized as the **shared structural foundation for three
program goals**, not a density-residual fix:

1. **Frost-depth fidelity** (the active goal). The snow pack is the frost
   insulation boundary; depth and density are the insulation drivers. The residual
   that bulk/climate-class could not fix sits in *exactly* the depth/density
   dimensions that drive frost.
2. **Winter water temperature** (operator goal). Per-layer cold content + a surface
   energy balance produce meltwater temperature and timing — the snowmelt *source
   temperature* the stream-water-temperature backlog needs.
3. **Runoff dynamics** (operator goal). Melt-front propagation + liquid
   percolation/refreezing through layers give meltwater output timing and magnitude
   (winter melt, spring pulse, rain-on-snow) far better than a bulk pack.

So the question is not "is multilayer worth it for the density residual" but "is the
multilayer snowpack the right foundation for frost + water-temperature + runoff." A
defer decision should be made against that scope; this document specifies it.

## 2. The cross-program payoff (and the current limitation each removes)

| Goal | Current (bulk) limitation | What multilayer provides |
|---|---|---|
| Frost insulation | snow→frost coupling passes only **bulk** `snow_depth_m` + `snow_density_kg_m3` (`DirectFrostThermalInputs`, `03_kernel_support_00_support_helpers.rs:299-310`; handoff `00a_snow_frost_authority_impl.rs:389-394`) | a **depth-weighted thermal-conductivity / insulation integral** over the layer stack — the physically correct insulation boundary condition |
| Winter water temperature | no per-layer thermal state; only melt *volume* is routed | per-layer cold content + surface energy balance → **meltwater temperature** and refreeze timing |
| Runoff dynamics | bulk melt release; no internal liquid routing | **melt-front + percolation/refreeze** per layer → realistic meltwater output timing/magnitude and rain-on-snow |
| Density (the residual) | one density under the **total** overburden (POC in `09_snow_density.rs` applies total SWE to one density) | per-layer densification under **local** overburden → the base-denser-than-top profile bulk cannot represent (the split-sign fix) |

## 3. Architectural landing — the key de-risking finding

**The hardest obstacle is already solved in the same sub-solver.** A multilayer pack
is variable-length (layers add/merge/collapse), which conflicts with the
array-native fixed-width frame (ADR-0025). But **frost already carries
variable-length layer Vecs** — `DirectFrostLaneState` holds `layer_shadows: Vec<…>`
and `fine_layers: Vec<…>` under the ADR-0026 winter-column exception, persistent at
lane level inside the boxed `DirectWinterColumnState` (`winter_column.rs:13-16`;
`DirectSnowLaneState` is `Copy` today, the column is `Clone` because frost owns heap
state). So:

- **Home:** the **winter-column sub-solver** (ADR-0026) is the natural and already-
  ratified home — snow and frost are co-located (`DirectWinterColumnState { snow,
  frost }`) and it already runs internal hourly loops over mutable lane-persistent
  state.
- **Pattern:** extend `DirectSnowLaneState` with a persistent `Vec<DirectSnowLayerState>`,
  mutated in place per day, carried via the existing `DirectSnowRuntimeCarry`
  mechanism — the **same pattern frost uses**, not a novel one.
- **Known hazard (documented):** ADR-0026's R7G failure was "unsafe coarse-layer
  projection" — mapping dynamic fine layers onto fixed coarse slots. Multilayer snow
  must avoid the same trap (own the layers, don't project onto fixed slots), exactly
  as frost learned to.

This materially lowers the architectural risk the paradigm assessment feared: the
variable-layer-in-array-native problem is precedented and ratified, not open.

## 4. Requirements

1. **Per-layer state.** `DirectSnowLayerState { thickness_m, mass_swe_m, density,
   temperature_c, cold_content, liquid_water_m, age/grain markers }`; `DirectSnowLaneState.layers:
   Vec<DirectSnowLayerState>`; layer management (add new-snow top layer; merge
   thin/similar layers; collapse to bulk and shed sub-threshold remnant — the
   n-layer analogue of SNOBAL `_calc_layers`/`_adj_layers`).
2. **Per-layer densification.** Apply the existing Anderson POC/PTM/wet physics
   (`09_snow_density.rs`) **per layer with the local overburden** (Σ overlying mass)
   — the change that produces the deep-pack base densification the bulk model
   cannot (the split-sign fix). Coefficients stay the Anderson authority.
3. **Per-layer thermal solve.** Inter-layer heat conduction + a surface energy
   balance; cold content per layer; melt/refreeze per layer; melt-front propagation.
   Requires the meteorology gap (req. 6).
4. **Liquid routing.** Percolation through layers (per-layer holding capacity +
   drainage, reusing the 10.3.8 holding-capacity authority) + refreezing; output
   meltwater (timing/magnitude) to the runoff phase; carry **meltwater temperature**.
5. **Snow→frost coupling upgrade.** Replace the bulk `snow_depth_m`/`snow_density_kg_m3`
   handoff with a **depth-weighted thermal-conductivity / insulation integral** over
   the layer stack into `DirectFrostThermalInputs`; preserve the prior-day timing
   decoupling (the existing guard).
6. **Meteorology surface energy balance (a real gap).** `openwepp-meteorology` has
   psychrometrics + Harder-Pomeroy phase but **not** the surface energy balance (net
   radiation, turbulent sensible/latent fluxes). Add surface-agnostic flux functions
   (Rn, H, L_vE, G) — **shared with the stream-water-temperature goal** (that
   backlog's "scope meteorology as the shared surface energy balance" decision).
7. **HBP / output.** Keep the **aggregate** SWE/depth/density as the public WAT/parquet
   columns (no schema churn); expose the per-layer profile as **diagnostic-only**
   first. A per-layer public schema is a separate ADR-0019 decision, deferred.
8. **Conservation.** Per-layer and whole-pack mass/energy ledgers; closure is the
   hard gate (independent of the rubric), per the standing discipline.
9. **Performance (ADR-0025).** n-layer × hourly × per-OFE-day is materially more
   expensive than bulk. The perf gate is real; frost's Vec exception proves it is
   feasible within budget, but layer count must be bounded by physics (merge
   discipline), and the hot loop kept array-native within the winter column.

## 5. Reference implementations (port-ready)

- **libsnobal (CC0, USDA-ARS)** — now license-clear and portable. Provides the
  surface energy balance, the 2-layer thermal scheme, melt/refreeze, and liquid
  routing. **But it is only 2-layer** — and 10.3.20 Stage B already showed the
  2-layer surface structure does **not** fix the density split-sign. So libsnobal is
  the energy-balance + liquid-routing + thermal *engine* to port; the **n-layer**
  density profile (the split-sign fix) must extend beyond it.
- **Crocus (Vionnet 2012, R-40, in-repo) / SNOWPACK / SNTHERM** — full n-layer with
  grain metamorphism; the reference for the n-layer densification/metamorphism and
  layer management. More complex; adopt selectively under ADR-0028.
- **Port strategy:** port libsnobal's energy balance + liquid routing (CC0,
  fast win, shared with water-temp), extend layering 2→n using Crocus as the
  n-layer densification/metamorphism reference, keep Anderson coefficients per layer.

## 6. Staging (each opt-in, contract-first, conservation-gated, identity-shadowed)

- **Stage 0 — meteorology surface energy balance.** Add Rn/H/L_vE/G to
  `openwepp-meteorology` (surface-agnostic). Prerequisite for Stages 2–3; immediately
  reusable by the water-temperature program. Pure crate, no runtime change.
- **Stage 1 — n-layer state + per-layer densification.** The density split-sign fix.
  Gate: cross-SNOTEL forcing-robust rubric, bidirectional densification flip,
  conservation, perf. (This is the lever 10.3.22 could not provide at bulk
  resolution.)
- **Stage 2 — per-layer thermal + snow→frost insulation-profile coupling.** The
  frost-insulation improvement. Gate: frost-depth fidelity against the frost
  observation corpus (the original goal).
- **Stage 3 — per-layer liquid routing + meltwater temperature.** Runoff dynamics +
  the winter water-temperature source. Gate: runoff/melt-timing evidence + the
  water-temperature validation corpus (USGS NWIS, per the stream-temp backlog).

Stages can be reordered to the priority goal (e.g. frost-first = Stage 0→2 with a
minimal thermal layering before the full density-profile Stage 1), but Stage 0 gates
2–3.

## 7. Cost / risk

- **Architecture risk: LOWER than feared.** The variable-layer-in-array-native
  problem is precedented and ratified (frost / ADR-0026); the winter-column home
  exists; the carry mechanism exists. The documented hazard (coarse-layer
  projection) has a known avoidance.
- **Real costs:** per-layer physics correctness (layer management, metamorphism,
  melt-front); the meteorology energy-balance addition (Stage 0); the performance
  budget (n-layer hourly within ADR-0025); the carry/projection discipline frost
  navigated; per-layer + whole-pack conservation.
- **Effort shape:** a staged multi-package program (Stages 0–3), not a single WP —
  but each stage is independently gated and independently valuable, and Stage 0 +
  the energy balance pay off for water temperature regardless of the snow outcome.

## 8. The defer-vs-commit decision inputs

- **Commit** if frost insulation fidelity, winter water temperature, and runoff
  dynamics are all on the roadmap — because the multilayer snowpack is the *shared
  enabling structure* for all three, so the cost is amortized across the program, not
  charged against the 15-cell density residual. The operator has stated all three are
  goals.
- **Defer** only if the density residual were the sole motivation (then
  disproportionate) — which is no longer the case.
- **Recommended lean: COMMIT, staged, frost-first.** Start with Stage 0 (meteorology
  energy balance — reusable, low-risk, unblocks water-temp too), then the thermal
  layering + snow→frost insulation-profile coupling (Stage 2) for the frost goal,
  then the density-profile (Stage 1) and liquid routing (Stage 3) as the rubric and
  runoff/water-temp programs demand. Each opt-in and gated, so the program can stop
  at any stage that proves disproportionate — the staging *is* the risk control.

## 9. Open questions

- Layer-count bound and merge thresholds (physics-driven, not fixed slots — avoid
  the ADR-0026 projection hazard). Fixed-max-`[T; N]` vs persistent `Vec`?
- Does Stage 2 need the full n-layer density profile, or does a coarse thermal
  layering (e.g. SNOBAL 2-layer + a base layer) already improve frost insulation
  enough? (Cheaper frost-first path.)
- Meltwater-temperature representation in the HBP boundary (intensive temperature on
  the flux — the stream-temp backlog's "carry temperature as typed flux state"
  decision; settle it here before the crate hardens).
- Performance envelope: measured n-layer hourly cost vs the ADR-0025 gate on H2637.
- Conservation ledger granularity (per-layer vs whole-pack) and the identity-shadow
  strategy for the variable-layer carry.
