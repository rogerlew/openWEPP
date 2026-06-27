# Frost / Daylength Canopy Phenology — Hemisphere-Robust Leaf-Off and Leaf-On

## Status

- `state`: **backlog (concept)** — not promotable before deciduous/mixed-forest
  canopy fidelity becomes load-bearing for the snow program (SNOWDENSITY is
  currently on conifer / high-cancov; this matters once the mixed-forest
  fixtures are active) and before a growth/canopy science contract is authored.
  Scope covers the **full deciduous/mixed canopy cycle** — autumn leaf-off
  (frost/daylength decline) **and** spring leaf-on (thermal-time green-up) —
  since a hemisphere-robust canopy needs both limbs physically driven.
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

Deciduous and mixed canopies are a **full annual cycle**, not a one-way decline:
the canopy must also **rebuild in spring (leaf-on / green-up)** from its winter
minimum to the summer maximum. WEPPcloud's managements disable WEPP's heat-unit
growth (`gddmax=0`) and fall back to fixed management dates for **both** limbs,
so spring leaf-out is as climate-blind and Northern-Hemisphere-bound as autumn
leaf-off. This item therefore covers the **whole phenology**: a hemisphere-robust
canopy needs both the cold/short-day **leaf-off** and the thermal-time
**leaf-on** driven by physical forcing, with **no fixed Julian dates** in either
limb.

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

## Leaf-on (spring green-up) — the complementary limb

Canopy phenology is a full annual cycle: deciduous and mixed forests not only
drop their canopy in autumn (the decline above) but **rebuild it in spring** from
the winter minimum to the summer maximum during **budburst → leaf expansion**.
Because WEPPcloud disables heat-unit growth (`gddmax=0`) and leans on fixed
management dates, spring leaf-out is just as **climate-blind, fixed-date, and
NH-bound** as the autumn leaf-off. A hemisphere-robust canopy needs **both**
limbs driven by physical forcing.

### The GDD asymmetry (one mechanism, two directions)

The 2026-06-26 negative experiment found heat-unit accumulation is the **wrong**
mechanism for **autumn senescence** (a warmer site accumulates GDD faster, so it
senesces *earlier* — the reverse of reality). The same physics is the **right**
mechanism for **spring leaf-out**: budburst is governed by spring thermal-time
accumulation, and a **warmer spring genuinely does leaf out earlier**. So GDD is
not "wrong for forests" wholesale — it is **correct for the green-up limb and
backwards for the senescence limb**. This resolves the apparent paradox in the
negative experiment: GDD failed because it was asked to drive the limb it has
backwards, not because it is unusable.

The clean design that follows:

- **Spring leaf-on:** a spring thermal-time (growing-degree-day above a base
  temperature) threshold, optionally gated by a **chilling requirement** (winter
  cold accumulation, which blocks premature budburst during a mid-winter thaw)
  and/or photoperiod. Warm-first ordering is correct here — keep GDD on this limb.
- **Autumn leaf-off:** the frost/daylength decline above (cold-and-short-day
  first). Warm-first ordering is wrong here — GDD must **not** drive this limb.

### Hemisphere robustness (same rule)

Spring green-up is SH-spring (≈Sep–Nov) in the Southern Hemisphere. The same
design rule applies: **no fixed Julian dates** — leaf-out must emerge from spring
temperature accumulation (plus chilling/photoperiod) computed from local forcing
and **signed latitude**, so SH timing falls out automatically. A unified
continuous phenology index driven by minimum temperature, photoperiod, and
humidity (e.g. the Jolly et al. 2005 GSI) is one candidate formulation that
delivers **both** limbs from the same hemisphere-symmetric quantities openWEPP's
energy balance already computes.

## Scoping note — leaf-on is not the current spring-melt residual

A SNOWDENSITY-10.3.8 seasonal analysis (2026-06-27) localized the residual
maritime over-accumulation to the **spring melt season (Feb–May)** but showed it
is **canopy-independent**: bare open surfaces (`harvard_open`,
`sleepers_south_field`) over-accumulate as much as or more than the paired
forested surfaces (`harvard_hardwood`, `sleepers_w9_hardwood`), with nearly
identical seasonal SWE profiles. The residual is spring melt/ablation
realization, **not** canopy attenuation. Leaf-on timing therefore changes the
**forested-surface canopy ordering** and late-spring melt attenuation under
deciduous/mixed canopy, but it is **not** the lever for the present spring
over-accumulation — keep the two distinct.

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

1. **Leaf-off NH direction:** at two NH sites of contrasting climate (e.g. a
   cold/high site vs a warm/low one), the **cold site leaf-off precedes the warm
   site** — the direction the GDD route got backwards.
2. **Leaf-on NH direction:** the **warm-spring site leafs out earlier** than the
   cold site — the symmetric complement of gate 1, and the direction GDD gets
   *right* (so the two limbs are validated to move oppositely with warmth).
3. **SH correctness (both limbs):** at a Southern-Hemisphere site, leaf-off
   occurs in **SH autumn (Apr–Jun)** and leaf-on in **SH spring (Sep–Nov)** —
   no calendar inversion of either limb. (Requires an SH climate source; see
   open questions.)
4. **Chilling guard (if implemented):** a warm mid-winter thaw does **not**
   trigger premature budburst — leaf-on waits for the chilling requirement.
5. **Magnitude/ordering:** deciduous winter `cancov` low, mixed intermediate,
   evergreen high (the snow-relevant ordering); the canopy rebuilds to its
   summer maximum at spring leaf-out and returns to the same seasonal envelope
   year-over-year (no inter-annual drift).
6. **Conservation:** the biomass/canopy ledger balances across **both** the
   decline and the green-up (no canopy created or destroyed without a matching
   biomass state change).

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
- **Leaf-on driver:** pure spring-GDD threshold vs GDD + chilling vs GDD +
  photoperiod (the sequential/parallel/alternating phenology-model families),
  and base-temperature/threshold defaults for deciduous vs mixed canopies. Is a
  simple spring-GDD threshold defensible at first pass, or is a chilling term
  required to suppress mid-winter-thaw budburst in the target climates?
- **One index or two laws:** drive leaf-on and leaf-off from a single unified
  continuous index (e.g. GSI from `Tmin` + photoperiod + VPD) vs two explicit
  limb-specific laws (spring thermal-time, autumn frost/daylength). The unified
  index is more parsimonious and hemisphere-robust by construction; the two-law
  form maps more directly onto the legacy `dec` abstraction.
- **Reuse the growth GDD machinery?** Spring green-up could reuse the existing
  `gddmax`/`sumgdd`/`fphu` growth phase rather than a dedicated canopy-phenology
  state — but the WEPPcloud managements set `gddmax=0`, so this interacts with
  how the deciduous/mixed managements are parameterized.
- **Mixed-forest evergreen floor:** mixed canopy = evergreen component + a
  deciduous overstory fraction; only the deciduous fraction leafs on/off, so
  mixed `cancov` never goes to zero. Define the evergreen floor (and reconcile
  with the bare-canopy branch/stem floor noted below).
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
  factor `fhr = 0.35 - (1.0 - daylen/(daymin+1))` (~813); the heat-unit growth
  phase (`fphu = sumgdd/gddmax`) is the spring-limb analogue to extend.
- wepppy `ADR-0009-deciduous-mixed-forest-managements.md` (Alternative #5) and
  `gdd-senescence-experiment.md` — the negative investigation establishing that
  GDD senescence is the wrong mechanism for autumn leaf-off (it is, however, the
  right-direction mechanism for spring leaf-on — the asymmetry above).
- **Jolly, W. M., Nemani, R., Running, S. W. (2005).** *A generalized,
  bioclimatic index to predict foliar phenology in response to climate.* Global
  Change Biology 11:619–632. A continuous 0–1 phenology index from minimum
  temperature, photoperiod, and VPD — globally validated and hemisphere-robust
  by construction; a candidate to drive **both** limbs from one law.
- **Chuine, I. (2000).** *A unified model for budburst of trees.* J. Theor. Biol.
  207:337–347 — the chilling+forcing budburst formulation for the leaf-on limb.
- **White, M. A., Thornton, P. E., Running, S. W. (1997).** *A continental
  phenology model for monitoring vegetation responses to interannual climatic
  variability.* Global Biogeochem. Cycles 11:217–234 — leaf-on/leaf-off thermal
  and photoperiod thresholds in an ecosystem-model context.
- `crates/openwepp-hillslope-orchestrator/src/hydrology/06_growth_state.rs` —
  the openWEPP canopy state (`cancov`, `canopy_decline`) this would extend.
- [snow-frost-fidelity-strategy](../planning/snow-frost-fidelity-strategy.md) —
  the consuming program (canopy → CoE melt attenuation).
