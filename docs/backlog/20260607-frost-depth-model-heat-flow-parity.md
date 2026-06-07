# Frost Depth Model — Heat-Flow Parity (Stage-2 physics-magnitude)

## Status

- `state`: **backlog (deferred)** — Stage-2 physics-magnitude tier; `default_path: not eligible`
- `date`: 2026-06-07 (created)
- `relates`: [ADR-0011](../decisions/0011-architecture-first-top-down-science-contracts.md),
  [ADR-0017](../decisions/0017-re-pin-operational-distrust-comparator-is-flag-not-target.md),
  [ROADMAP.md](../ROADMAP.md) (queue item 2, Stage-2)
- `sizing gate`: ✅ **COMPLETE** — comparator characterization WP
  `docs/work-packages/20260608-fdmc01-frost-depth-comparator-characterization-001/`
- `sizing verdict`: **materially off** → target (1) heat-flow parity selected (do not
  sanction the proxy by contract). Evidence: openWEPP depth capped 200 mm vs legacy
  240–503 mm (43/43 exceed cap), depth-series median correlation 0.13, **and** openWEPP
  frozen-water duration +258 days (~34% longer — the proxy ratchets and over-persists).
  `execution still deferred to Stage-2 (post-MOFE) per ROADMAP staging.`
- `sibling`: [snow physics-magnitude (Stage 2)](20260605-snow-code-deferred-science-review.md)

## Why this is deferred, not now

Frost **activation** and **conservation** are closed (FQ-4: `ksflag` frost engages on
the agricultural substrate, 43/43 prefixes, closure ~3.22e-11 mm). What remains is the
**fidelity of the frost depth model** — and depth fidelity is a *magnitude* question,
which the [closure-not-magnitude principle](../ROADMAP.md) defers to the Stage-2 tier,
judged last against an already-closed and routed balance so depth-model error is not
aliased with routing or other open magnitudes. Nothing earlier blocks on it: MOFE
(rung-3) needs frost to *conserve* (done), not to be depth-faithful.

## The gap

openWEPP computes frost **depth** with a **freeze-index proxy**, not legacy's
energy-balance heat-flow model:

| | openWEPP | Legacy (`wepp-forest_260430_baseline`) |
|---|---|---|
| Depth model | `frdp = 0.20 · clamp(−mean_temp / 6)` (daily mean-temp index) | layered energy-balance heat-flow (`frostn.for`): energy flow between frozen layers, Dun-2008 fine sublayers (1 cm tilled / 2 cm untilled) |
| Max frost depth | hard-capped at **0.20 m** (`WB14_FROST_MAX_DEPTH_M`) | **`frdp ≤ 1.0 m`** |
| `Qsrf`/`Quf`/`ksrf` heat flux | computed, but **downstream of** the proxy depth (telemetry, not the driver) | drives the depth |

Authority: `SC-SNOWFREEZE-001` `INV-SNOWFREEZE-006` (heat-flow `Qsrf`/`Quf` + harmonic
conductivity, Eq. [3.8.1]–[3.8.4]), `INV-SNOWFREEZE-012` (`winter → frostN → frzng →
frznw` chain), and `GAP-SNOWFREEZE-002` (frost process/parity explicitly open). Legacy
`frostn.for`/`frzng.for`/`frznw.for`/`frsoil.for` are the heat-flow reference
(corroborating per ADR-0017, not acceptance authority).

**The depth model is openWEPP-introduced** — this is the engine's own simplification,
not inherited from legacy.

## In scope / out of scope

In scope:
- The **standard `ksflag` frost depth model** on agricultural (`lanuse=1`) substrates —
  when frost forms, how deep it grows, the 0.20 m cap, the 6 °C freeze-index scale, and
  the consequent frozen-soil-conductivity-bite duration.

Out of scope (explicitly):
- **Frost activation** — closed (FQ-4).
- **kfactor conductivity magnitude** — **legacy-faithful**, not a defect. openWEPP uses
  the documented WEPP defaults (`kfactor1=kfactor2=1e-5`, `kfactor3=0.5`); annual crops
  correctly select the near-impermeable "concrete frost" coefficient. The strong
  conductivity bite is intended behavior, not the gap.
- **Forest `ksatadj` sat-fraction conductivity** — a *separate* model on a separate
  path (forest/`lanuse≠1`, e.g. `/wc1/runs/ar/arboreal-dendrite`, where `ksflag` is
  off). Not the standard-ksflag frost-depth question; do not conflate.
- **Snow magnitude** — separate Stage-2 item (sibling note).

## Minimal target abstraction (if promoted)

One of two, decided by the sizing gate:
1. **Implement the legacy energy-balance heat-flow frost depth** (`frostn` lineage) per
   `INV-SNOWFREEZE-006`/`-012` — the contract already specifies it; this would close
   `GAP-SNOWFREEZE-002`. This is a substantial, well-bounded physics re-implementation
   (SHAW/Flerchinger-class), warranting its own DC-ExecPlan with contract authority.
2. **Sanction the proxy** — if the characterization shows the freeze-index proxy is
   adequate for the engine's accuracy goals, amend `SC-SNOWFREEZE-001` to authorize it
   explicitly (provenance + bounds + the 0.20 m cap rationale), retiring the implicit
   `INV-SNOWFREEZE-006` divergence rather than leaving it open.

## Promotion path

1. **Sizing gate (now): comparator characterization WP** — size the depth/duration gap
   (legacy heat-flow vs proxy) on the frost-active `algebraic-radium` substrate. This is
   characterization only; no code or contract change.
2. **Promotion decision** (after the sizing gate + after MOFE closes, so materiality is
   judged on a routed balance):
   - proxy materially off **and** it changes downstream runoff/erosion magnitude →
     promote target (1), a heat-flow parity DC-ExecPlan;
   - proxy crude-but-close → promote target (2), a contract amendment sanctioning the
     proxy;
   - either way, route new/changed physics through top-down science-contract authoring
     before adoption.

## Open questions (falsifiable, for the sizing gate)

- Does the proxy over- or under-predict frost **depth** vs legacy on the cohort?
- Does the **0.20 m cap** bind materially (legacy reaches 1.0 m)?
- Does the proxy get frost **onset/thaw timing** and **frozen-days duration** right
  (this governs how long the conductivity bite is active, hence runoff)?
- Is the **6 °C freeze-index scale** defensible, or fitted?
- Does depth fidelity materially change **runoff/erosion magnitude** once MOFE routes —
  or is it second-order?
- Where does legacy emit frost depth (which output surface yields `frdp` for the
  comparison)?
