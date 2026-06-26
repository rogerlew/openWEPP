# Frost / Daylength Canopy Decline — Hemisphere-Robust Senescence

## Status

- `state`: **backlog (concept)** — not promotable before deciduous/mixed-forest
  canopy fidelity becomes load-bearing for the snow program (SNOWDENSITY is
  currently on conifer / high-cancov; this matters once the mixed-forest
  fixtures are active) and before a growth/canopy science contract is authored.
- `date`: 2026-06-26 (created, Claude Code)
- `relates`:
  [ADR-0011](../decisions/0011-architecture-first-top-down-science-contracts.md)
  (contract-first new physics),
  [ADR-0017](../decisions/0017-re-pin-operational-distrust-comparator-is-flag-not-target.md)
  (legacy is reference, not target),
  `SC-SNOWFREEZE-001` / `GAP-SNOWFREEZE-002` (canopy → CoE melt attenuation),
  [snow-frost-fidelity-strategy](../planning/snow-frost-fidelity-strategy.md)
- `provenance (cross-repo)`: wepppy work-package
  `docs/work-packages/20260626_deciduous_mixed_forest_managements/`,
  `docs/adrs/ADR-0009-deciduous-mixed-forest-managements.md` (Alternative #5),
  and `artifacts/gdd-senescence-experiment.md` — the negative GDD investigation
  that surfaced this gap.

## Summary

Legacy WEPP contains the **physically correct, cold/photoperiod-driven canopy
decline** for autumn leaf-off, but gates it to the **rangeland** growth path,
so cropland-mode perennial forests cannot use it. openWEPP — a clean,
contract-first reimplementation not bound by the legacy cropland/rangeland
branch split — can implement this decline as a **first-class, landuse-agnostic,
hemisphere-robust** mechanism, superseding the fixed-Northern-Hemisphere
calendar-date workaround that legacy WEPP and WEPPcloud are forced into.

## Origin (the structural finding)

The WEPPcloud deciduous/mixed-forest managements need a seasonal leaf-off so
deciduous winter canopy cover is low (it drives the CoE snowmelt attenuation
`amelt = 0.0607·hrad·(1-cancov)`). A 2026-06-26 experiment screened WEPP's
heat-unit senescence (`gddmax>0` + `dlai`) across a 240-point grid and two
climates and found **zero** climate-correct candidates. The reason is
structural, not tuning:

- WEPP heat-unit senescence fires when `fphu = sumgdd/gddmax >= dlai`. A
  **warmer** site accumulates growing-degree-days faster, so it senesces
  **earlier** — correct for *crop maturity* but the **reverse** of autumn
  leaf-off, which is driven by shortening photoperiod and frost and should make
  **colder** sites drop first.
- WEPP *does* implement the correct cold-driven mechanism, but only in the
  **rangeland** branch. Cropland-perennial forests (`lanuse==1 .and.
  imngmt==2`, `grow.for:751`) never reach it.

Forced to neither correct mechanism, WEPPcloud fell back to a **fixed Julian
senescence date** (`jdharv=286`, ~Oct 13). That is **Northern-Hemisphere-only**
(day 286 is SH mid-spring) and **not climate-adaptive** (one leaf-off day for
every site, whereas real leaf-off spans late-September to early-November across
CONUS). See wepppy ADR-0009 for the full record.

## Mechanism (legacy reference, minimal abstraction)

The legacy rangeland decline (`grow.for` ~804-850; `dec` reduces live above-
ground biomass `vdmt`, which in turn lowers canopy cover):

```
fhr = 0.35 - (1.0 - daylen / (daymin + 1))          # daylength reduction factor
frst = f(Tmin; x5, x6)                               # frost-damage factor from min temp + sensitivity
dec  = 0.5 * vdmt * (1 - fphu) * max(fhr, frst)      # biomass decrement during decline phase
```

- `daylen` = current photoperiod (h); `daymin` = minimum annual photoperiod at
  the site — both functions of **latitude and day-of-year** (solar declination).
- `frst` rises as the daily minimum temperature crosses the plant's frost-
  sensitivity points (`x5`, `x6`); `tmpmin` already exists in the plant block.
- Canopy chain: in openWEPP the growth state derives canopy from biomass
  (`06_growth_state.rs`: `cancov = 1 - exp(-bb·canopy_biomass)`, with an existing
  `canopy_decline` factor). A decrement to live biomass therefore lowers `cancov`.

This is reference physics under ADR-0017 — to be re-derived into an openWEPP
science contract, not ported verbatim.

## Why openWEPP can make it hemisphere-robust

`daylen`/`daymin` and `frst` depend only on **latitude + day-of-year + local
temperature** — quantities that are inherently hemisphere-symmetric:

- In the Southern Hemisphere the photoperiod cycle is phase-shifted six months,
  so `daylen` shortens toward the **June** (SH-winter) solstice — i.e. SH
  autumn — which is exactly when SH leaf-off should occur. A daylength factor
  computed from **signed latitude** gets SH timing right automatically.
- Frost (`Tmin`) arrives in the local autumn in either hemisphere.

The design rule that delivers robustness: **no fixed Julian dates anywhere** —
leaf-off timing must emerge from latitude-driven photoperiod and local
temperature. This is what makes openWEPP's version superior to the legacy
rangeland code (which still assumes NH winter bookkeeping elsewhere) and to the
WEPPcloud fixed-date fallback.

## Governing constraints

- **Contract-first (ADR-0011):** author/extend a growth–canopy science contract
  for the decline (state, invariants, the `dec`/`fhr`/`frst` abstraction,
  hemisphere invariant) before default adoption. Candidate home: the plant-growth
  canopy contract that governs `06_growth_state.rs` `cancov`.
- **Landuse-agnostic:** the mechanism must be available to the
  forest/perennial path, not gated behind a rangeland branch — the explicit
  break from the legacy limitation.
- **Conservation:** the biomass decrement must respect the growth/biomass
  ledger (decline removes live biomass to litter/residue; no canopy created or
  destroyed without a matching state change).
- **ADR-0017:** legacy `grow.for` is the reference for the formulation, not a
  parity target; the contract's invariant (cold-and-short-day → decline) is the
  acceptance authority.

## Falsifiable validation gates

1. **NH direction:** at two NH sites of contrasting climate (e.g. a cold/high
   site vs a warm/low one), the **cold site leaf-off precedes the warm site** —
   the direction the GDD route got backwards.
2. **SH correctness:** at a Southern-Hemisphere site, leaf-off occurs in the
   **SH autumn (Apr–Jun)**, not on an NH-autumn date — no calendar inversion.
   (Requires an SH climate source; see open questions.)
3. **Magnitude/ordering:** deciduous winter `cancov` low, mixed intermediate,
   evergreen high (the snow-relevant ordering), with canopy recovering at
   spring leaf-out.
4. **Conservation:** biomass/canopy ledger balances across the decline.

## Promotion criteria

- Mixed/deciduous-forest canopy fidelity is on the critical path for the snow
  program (the SNOWDENSITY mixed-forest fixtures are active and canopy is the
  limiting term), **and**
- a growth–canopy science contract surface is identified to host the decline
  invariant, **and**
- an SH validation climate is available (or the SH gate is explicitly deferred
  with NH-only interim scope).

When all hold, spin up a dated work-package under `docs/work-packages/` and
route the new physics through top-down contract authoring.

## Open questions

- What currently drives openWEPP's `canopy_decline` in `06_growth_state.rs`, and
  does adding the frost/daylength decrement compose with it or replace it?
- Source of `daylen`/`daymin` in openWEPP (existing solar/radiation code already
  computes declination for the energy balance — reuse it; confirm signed-latitude
  handling).
- Frost-sensitivity parameters (`x5`,`x6`/`tmpmin`) — carry from the plant block
  or re-derive defaults for deciduous vs mixed canopies.
- SH climate forcing: wepppy's stack (DAYMET/GRIDMET/CLIGEN/PRISM) is CONUS-only,
  so SH validation needs a different climate source — scope this before the SH
  gate is required.
- Interaction with snow interception: leafless deciduous still retains branch/
  stem area (~0.1–0.3 effective cover) that the live-canopy `cancov` misses
  (noted in wepppy ADR-0009) — decide whether a bare-canopy floor belongs here.

## References

- `grow.for` (wepp-forest): rangeland decline `dec = 0.5·vdmt·(1-fphu)·
  max(fhr,frst)` (~804-850); cropland-perennial branch (`:751`); daylength
  factor `fhr = 0.35 - (1.0 - daylen/(daymin+1))` (~813).
- wepppy `ADR-0009-deciduous-mixed-forest-managements.md` (Alternative #5) and
  `gdd-senescence-experiment.md` — the negative investigation establishing that
  GDD senescence is the wrong mechanism and the correct one is rangeland-only.
- `crates/openwepp-hillslope-orchestrator/src/hydrology/06_growth_state.rs` —
  the openWEPP canopy state (`cancov`, `canopy_decline`) this would extend.
- [snow-frost-fidelity-strategy](../planning/snow-frost-fidelity-strategy.md) —
  the consuming program (canopy → CoE melt attenuation).
