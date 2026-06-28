# ADR-0029: Commit to Paradigm 2 — staged multilayer snow physics

**Status:** Accepted (ratified 2026-06-28 by Codex package `20260628-adr0029-paradigm-2-ratification-001`)
**Date:** 2026-06-28
**Deciders:** Roger Lew, Codex
**Author of draft:** Claude Code (drafted at decider Roger Lew's direction)
**Ratification provenance:** Accepted by Roger Lew and Codex via
`docs/work-packages/20260628-adr0029-paradigm-2-ratification-001/` after
ratifying ADR-0028 and verifying the Paradigm 2 de-risking, bulk-floor,
Paradigm-1 refutation, Stage 0, and ADR-consistency claims.
**Supersedes:** the "climate-class (Paradigm 1) first" recommendation in the
[snow-density paradigm assessment](../work-packages/20260628-snow-density-paradigm-assessment-001/)
(a WP-local ADR-candidate), which SNOWDENSITY-10.3.22 refuted
**Admission basis:** [ADR-0028](0028-observed-data-admission-authority.md) (observed-data admission)
**Builds on:** [ADR-0026](0026-stateful-winter-column-sub-solver.md) (winter-column home + variable-layer precedent), [ADR-0025](0025-array-native-hillslope-day-frame.md) (array-native constraint), [ADR-0011](0011-architecture-first-top-down-science-contracts.md) (contract-first)
**Specification:** [paradigm2-multilayer-snow-specification](../planning/paradigm2-multilayer-snow-specification.md)

## Context

The snow-density program converged at a defensible **bulk floor** — the
`coe_liquid_holding_capacity_v1 + physics_bulk_density_compaction_v1 +
harder_pomeroy` default scores 15 fail / 179 on the cross-SNOTEL forcing-robust
rubric, **above legacy** (16 / 176; the Harder-Pomeroy partition was the decisive
cross-climate lever). The remaining residual is a **structural densification-trajectory
split-sign**: the bulk model over-densifies humid/continental-forest packs and
under-densifies deep mountain packs.

Both tractable paradigms have failed to resolve it:

- The **bulk SNOBAL/CoE/Anderson family** (SNOWDENSITY-10.3.16–10.3.20: sublimation,
  composition, shallow-pack guard, two-layer Stage B) — none beat the default.
- **Climate-class parameter specialization (Paradigm 1)** (10.3.22, source-verified
  Sturm 1995/2010) — our validation corpus maps to the indistinct high-ρmax
  {alpine, maritime, prairie} cluster (all ρmax ≈ 595), while Sturm's strong class
  divergence lives in tundra/taiga (arctic/boreal regimes absent from the corpus).
  The split-sign is a finer distinction than Sturm's six classes resolve.

The diagnosis is therefore **structural, not parametric**: a single bulk density
under the *total* overburden cannot represent the base-denser-than-top profile of a
deep pack (the overburden physics is correct but applied at bulk resolution). Only
**per-layer resolution** can. This was scoped in the specification, which found two
decision-changing facts:

1. **Multilayer snow is the shared structural foundation for three program goals**,
   not a density-residual fix: frost insulation (the depth-weighted
   thermal-conductivity profile the bulk snow→frost handoff cannot supply — and
   depth/density is exactly where the residual sits), **winter water temperature**
   (per-layer cold content + the surface energy balance → meltwater temperature),
   and **runoff dynamics** (melt-front + per-layer percolation/refreeze).
2. **The architecture is de-risked.** The variable-length-layer-in-array-native
   obstacle (ADR-0025) is already solved for frost under ADR-0026 — the winter-column
   sub-solver carries persistent per-layer Vecs (`DirectFrostLaneState`
   `layer_shadows`/`fine_layers`). Multilayer snow adopts the same precedented
   pattern in the same ratified home, with the documented R7G coarse-layer-projection
   hazard to avoid.

## Decision

**Commit to Paradigm 2 (multilayer snow physics) as the snow-density-structure
path**, built **opt-in and staged**, each stage independently gated and
independently valuable, with the **bulk default remaining the default and
rollback**. The specification is the requirements authority. Admission is under
ADR-0028 (defensible physics + cross-SNOTEL forcing-robust rubric improvement + no
fixture fitting + conservation); the model is authored contract-first (ADR-0011) in
the winter-column sub-solver (ADR-0026), respecting the array-native constraint
(ADR-0025) via the frost-precedent ownership pattern (persistent lane-level layer
state, no coarse-layer projection).

The staged program (see the specification for requirements):

- **Stage 0** — surface energy balance in `openwepp-meteorology` (pure primitives;
  prerequisite for Stages 2–3; shared with the stream-water-temperature program).
- **Stage 1** — n-layer snow state + per-layer densification under *local*
  overburden (the density split-sign fix).
- **Stage 2** — per-layer thermal solve + the snow→frost insulation-profile coupling
  (the frost goal; replaces the bulk depth/density handoff).
- **Stage 3** — per-layer liquid routing + meltwater temperature (runoff dynamics +
  the winter water-temperature source).

**The staging is the risk control:** any stage may close `HOLD`/non-promotion on its
own gate (rubric / conservation / ADR-0025 perf / frost or runoff/water-temp
evidence), and the program may stop at any stage that proves disproportionate. This
decision supersedes the assessment's Paradigm-1-first recommendation.

## Scope

- This ADR commits to the **program, its staged shape, and the
  admission/home/constraint framing** — it does **not** pre-decide each stage's
  physics (contract-first per stage) and does **not** activate any default. The bulk
  default stands until a stage gates and a later activation decision is made under
  the existing Policy B (ADR-style activation gates).
- Reference implementations: libsnobal (CC0) for the energy balance + 2-layer thermal
  + liquid routing; Crocus (R-40) / SNOWPACK / SNTHERM for the n-layer
  densification/metamorphism. The 2-layer form alone is insufficient for the density
  split-sign (10.3.20 Stage B).

## Consequences

**Positive**

- Targets the only remaining density lever **and** lays the foundation for frost
  insulation, winter water temperature, and runoff dynamics — three program goals on
  one structure, so the cost amortizes across the program rather than the 15-cell
  residual.
- De-risked architecture (frost/ADR-0026 precedent + ratified home); staged + opt-in
  + gated; bulk default + rollback preserved.
- Authority-grounded and license-clean (CC0 libsnobal + Crocus), no fixture fitting.

**Negative / risks (with mitigations)**

- *Per-layer hourly cost vs the ADR-0025 perf gate* → bounded layer count (merge
  discipline), array-native hot loop within the winter column; frost's Vec exception
  demonstrates feasibility within budget.
- *Per-layer physics correctness + layer management* → staged delivery, conservation
  as a hard gate independent of the rubric, reference implementations.
- *Carry/projection discipline* → follow the ADR-0026 frost pattern; avoid the R7G
  coarse-layer-projection hazard explicitly.
- *Multi-package program effort* → each stage independently valuable; the program can
  stop at any stage.

## First increment

PARADIGM-2 Stage 0 (surface energy balance in `openwepp-meteorology`) is complete as
a pure, unwired, CC0-provenanced crate addition — the foundation for Stages 2–3 and
the stream-water-temperature program.
