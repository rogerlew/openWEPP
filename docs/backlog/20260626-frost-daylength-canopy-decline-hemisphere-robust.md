# Frost / Daylength Canopy Phenology — Hemisphere-Robust Leaf-Off and Leaf-On

## Status

- `state`: **backlog (concept)** — not promotable before deciduous/mixed-forest
  canopy fidelity becomes load-bearing for the snow program (SNOWDENSITY is
  currently on conifer / high-cancov; this matters once the mixed-forest
  fixtures are active) and before a growth/canopy science contract is authored.
  Scope covers the **full deciduous/mixed canopy cycle** — autumn leaf-off
  (frost/daylength decline) **and** spring leaf-on (thermal-time green-up) —
  since a hemisphere-robust canopy needs both limbs physically driven. Scope
  **also covered the ground-side complement: seasonal surface residue / litter
  cover** (autumn leaf-drop → litter → soil thermal insulation → frost), added
  2026-06-29 from the frost Step-3 diagnosis. That residue-cover dimension has
  now been implemented by
  `docs/work-packages/20260629-frost-residue-cover-implementation-001/`; the
  remaining backlog scope is the canopy leaf-off/leaf-on cycle.
- `date`: 2026-06-26 (created, Claude Code)
- `relates`:
  [ADR-0011](../decisions/0011-architecture-first-top-down-science-contracts.md)
  (contract-first new physics),
  [ADR-0017](../decisions/0017-re-pin-operational-distrust-comparator-is-flag-not-target.md)
  (legacy is reference, not target),
  [ADR-0033](../decisions/0033-ofe-by-ofe-overland-flow-routing.md)
  (OFE-by-OFE routing, opt-in activation),
  `SC-SNOWFREEZE-001` / `GAP-SNOWFREEZE-002` (canopy → CoE melt attenuation),
  `SC-OFEROUTE-001` (Papanicolaou routing friction operands),
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

## Management-file authority — first-class `lanuse` modes

The same structural lesson applies to Lane D routing: legacy WEPP and WEPPcloud
often encode forests and rangeland as `lanuse=1` cropland records because the
legacy input grammar and branch coverage made that the least-bad workaround.
openWEPP should not perpetuate that workaround for new physics. For openWEPP
native managements, the **management file** must be the opt-in authority, and
`lanuse` must select the physical landuse mode. The `.run` file should point to
the management sidecar and remain reproducibility metadata; it should not carry
hidden physics selectors that are lost when only the input sidecars are archived.

Required direction:

- Define a first-class openWEPP forest `lanuse` mode instead of requiring forest
  canopies, litter, and hydraulic roughness to masquerade as cropland
  perennial records.
- Carry forest phenology parameters in that landuse record: evergreen floor,
  deciduous fraction, leaf-off frost/photoperiod controls, leaf-on thermal-time
  or chilling controls, and litter/residue-pool parameters.
- Carry OFE-routing roughness parameters in the landuse/OFE record when routing
  is active: `k_o`, optional form/wave operands (`C_d`, `D_r`, `lambda`), and
  optional hydraulic vegetation operands (`LAI`, `h_c`, vegetation `C_d`) under
  `SC-OFEROUTE-001`.
- Treat cropland-encoded forest/range fixtures as **compatibility inputs**. They
  may be migrated or interpreted by an explicit adapter, but they must not be
  the authority for new forest/rangeland physics.

Activation rule: opt-in routing/phenology behavior should come from a complete
and typed `lanuse` block in the management file. A runfile-level flag may be
useful for diagnostics or forced disabling, but it must not be the source of the
physical parameters or the only record that the run used the enhanced path.

This also resolves the Papanicolaou roughness gap. It is not sensible to infer
hydraulic vegetation or isolated-element drag from legacy cropland fields such
as row width, rill spacing, or `rrinit` without a bridge contract. Put the
Papanicolaou operands where they belong: first-class landuse/OFE parameters.

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

## Surface residue / litter cover — the ground-side complement (frost insulation)

The **same autumn leaf-drop** that thins the canopy (the decline above) deposits
that biomass on the ground as **surface residue / litter**. These are two
consequences of one event, with **opposite-direction** snow/frost effects:

- **Canopy side (above-ground, in scope above):** leaf-off lowers `cancov` → more
  shortwave reaches the snow → more CoE melt (`amelt = 0.0607·hrad·(1-cancov)`).
- **Residue side (ground-side, this dimension):** leaf-drop **raises surface
  litter depth** → more soil thermal insulation → frost **onset delayed** and
  **thaw advanced** as the litter decays through winter into spring.

This is a **live lever, not a hypothetical** — confirmed 2026-06-29 in the frost
Step-3 diagnosis: the frost solver's `frost_surface_heat_path`
(`crates/.../hydrology/support_helpers_mod/coupling/frost.rs`) adds
`residue_depth_m / residue_conductivity_w_m_k` directly into the surface thermal
resistance. A **static** `residue_depth_m` (the inert `Tah_*` "no senescence or
decomposition" plant the deciduous frost fixtures use) **under-insulates in fall
(→ early frost onset) and over-insulates in spring (→ late thaw)** — exactly the
Sleepers W9 early-onset + late-thaw timing signature. So a correct **seasonal**
litter trajectory is required for deciduous-site frost timing.

### The structural question (the same cropland/rangeland limitation)

The biomass ledger already routes leaf-off biomass to litter/residue (see the
Conservation constraint below). The open question is whether the **cropland**
residue model can produce a **physically correct seasonal forest litter cycle**:

- Crop residue **decomposition coefficients** are calibrated for crop straw and
  may **decay forest leaf litter too fast**, collapsing the insulating layer
  before spring (the leaf litter would vanish, not persist as a slowly-decaying
  multi-year forest floor).
- A **perennial** management (`imngmt==2`) may yield **no recurring annual
  leaf-drop** to the surface-residue pool — the same cropland/rangeland branch
  split that strands the canopy decline (`grow.for:751`) can strand the litter
  input.

### What frost Step 3 actually found (2026-06-29) — the gap is a missing coupling

Frost Step 3 (`docs/work-packages/20260629-frost-step3-residue-parameterization-001/`)
routed to **branch C**: the `Dec_*` entry-gate run reached the frost solver but
`residue_depth_m` was **perfectly flat** (`0.0230259 m` min = max = autumn = spring
mean across `32874` rows). The **root cause** (Claude review of the production wiring)
is more fundamental — and more contained — than "cropland can't represent forests":

- openWEPP **already computes a *dynamic* surface-residue mass** —
  `07_decomposition_equations.rs` (`sumsrm_next = sumsrm_seed · surface_decay`) and
  `direct_runtime/decomposition.rs` (`surface_residue_kg_m2` with decay factors).
- But `frost.runtime_residue_depth_m` (the symbol `frost_surface_heat_path` reads)
  is **seeded once** from the legacy initial condition in
  `runtime_inputs/01_management.rs` (`("resdep", seed…)`,
  `("frost.runtime_residue_depth", seed…)`, asserted in tests as "legacy
  init1/res_dp lineage") and **never updated from the dynamic mass.** The only
  writebacks are the two seed sites.
- So **residue depth is static for *every* landuse**, not just forests. The flat
  `Dec_*` trace is the symptom; the missing `mass → depth → frost` coupling is the
  disease.

**This reshapes the implementation scope.** It is a **wiring** task (connect the
existing dynamic surface-residue mass to `frost.runtime_residue_depth_m` via a
mass→depth conversion), **not** a from-scratch litter model — *provided* the
`Dec_*` surface-residue **mass** is itself seasonal. The **first implementation
task** is to determine that: does the dynamic `surface_residue_kg_m2` under `Dec_*`
already show autumn senescence input + winter/spring decay (→ pure wiring), or does
the senescence not deposit recurring annual leaf-drop (→ also build the litter
*input* limb)? The cropland decomposition-rate concern below applies in either case.

If, once wired, a cropland management **can** carry a realistic seasonal
`residue_depth_m`, the residue dimension is satisfied by the coupling + the existing
`Dec_*` parameterization. If it **cannot** (crop decay too fast, or no recurring
perennial leaf-drop), implement a **first-class, landuse-agnostic forest
residue-cover representation** here — the litter analogue of the landuse-agnostic
canopy decline this backlog already argues for.

FROST RESIDUE-COVER IMPLEMENTATION
(`docs/work-packages/20260629-frost-residue-cover-implementation-001/`) resolved
this residue-cover dimension. Phase 0 found the `Dec_*` surface-residue mass was
flat under zero-rate/no-input management, so the package added the missing
litter-input limb plus dynamic `mass -> depth -> frost` wiring. The post-fix
entry gate passed after review disposition with autumn mean `0.165028 m`, spring
mean `0.159910 m`, and max monthly mean in October. The Sleepers A-vs-B rerun
routed to branch A as a partial contributor: candidate-defect timing cells
shrank from 18 to 13. Residual cells remain for later frost attribution, and the
fall litter-drop window still uses the management fall date (`jdharv`) until the
physical frost/daylength phenology backlog replaces that anchor, but the
residue-cover backlog item is implemented.

### Residue-specific validation gates (add to the gates below)

- **Seasonal litter trajectory:** `residue_depth_m` at a deciduous site **peaks in
  autumn** (post-leaf-drop) and **declines through winter/spring** — not flat, and
  not collapsed to zero before snowmelt.
- **Frost-timing response:** running a deciduous site with the seasonal residue (vs
  the inert static residue) moves frost **onset later and thaw earlier** toward the
  observations (the Step-3 attribution direction).
- **Multi-year forest floor:** the litter layer persists across years (a forest
  floor is a standing pool), rather than fully mineralizing each summer like crop
  residue.

## Governing constraints

- **Contract-first (ADR-0011):** author/extend a growth–canopy science contract
  for the decline (state, invariants, the `dec`/`fhr`/`frst` abstraction,
  hemisphere invariant) before default adoption. Candidate home: the plant-growth
  canopy contract that governs `06_growth_state.rs` `cancov`. The **surface-residue
  dimension** extends the same contract on the ground side: seasonal litter depth
  from leaf-drop, a forest-appropriate decomposition rate, and the
  `residue_depth_m` → `frost_surface_heat_path` insulation coupling.
- **Landuse-agnostic:** the mechanism must be available to the
  forest/perennial path, not gated behind a rangeland branch — the explicit
  break from the legacy limitation.
- **Management-file opt-in:** new forest/rangeland behavior must be selected and
  parameterized by a first-class management `lanuse` mode. The runfile must not
  be the sole opt-in authority for canopy phenology, litter physics, or
  Papanicolaou routing operands.
- **First-class routing roughness:** Papanicolaou operands (`k_o`, `C_d`, `D_r`,
  `lambda`, `LAI`, `h_c`) belong in the management landuse/OFE record when
  routing is active. Do not infer them from legacy cropland row/ridge workarounds
  unless a separate bridge contract ratifies that mapping.
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
7. **Management-file provenance:** an activated forest/rangeland run is
   reproducible from its sidecars alone: the `.man`/landuse record declares the
   physical mode and the Papanicolaou/canopy/litter operands used by the run.

## Promotion criteria

- Mixed/deciduous-forest canopy fidelity is on the critical path for the snow
  program (the SNOWDENSITY mixed-forest fixtures are active and canopy is the
  limiting term), **and**
- a growth–canopy science contract surface is identified to host the decline
  invariant, **and**
- the openWEPP management parser/runtime has a first-class forest `lanuse` mode
  capable of carrying canopy phenology, litter, and OFE-routing roughness
  operands, **and**
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
- Exact native management-file shape: extend legacy `.man` syntax with an
  openWEPP datver/section, or introduce an openWEPP-native management sidecar
  that preserves `lanuse` semantics while avoiding legacy parser ambiguity.
- Migration policy for existing WEPPcloud forest/range inputs that are encoded
  as cropland: whether to require explicit conversion to the new `lanuse` mode
  for enhanced routing/phenology, or allow a temporary compatibility adapter
  that emits a manifest warning and refuses ambiguous Papanicolaou operands.
- Forest/rangeland Papanicolaou defaults: whether any default `k_o`, hydraulic
  vegetation height, or form/wave parameters are defensible by landuse class, or
  whether active routing should fail closed until the management file supplies
  them explicitly.

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
- `SC-OFEROUTE-001` and ADR-0033 — the opt-in OFE-by-OFE routing contract and
  Papanicolaou friction operands that should be carried by native landuse/OFE
  management records, not by a runfile-only selector.
- [snow-frost-fidelity-strategy](../planning/snow-frost-fidelity-strategy.md) —
  the consuming program (canopy → CoE melt attenuation).
