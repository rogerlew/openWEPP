---
contract_id: SC-PLANT-001
title: Plant Growth Process Contract
status: in_review
maturity: draft
owner: openWEPP maintainers + hydrology reviewer
contract_version: 4
producer_scope:
  - Plant state evolution for cropland and rangeland growth submodels
  - Plant to water-balance coupling surfaces (LAI, root depth, plant biomass/residue descriptors)
  - Plant to erosion and residue-component coupling surfaces
  - PL transition-control runtime projection authority for annual/perennial management event payloads
consumer_scope:
  - Water balance and evapotranspiration surfaces consuming plant state descriptors
  - Erosion surfaces consuming canopy and cover descriptors
  - Residue decomposition and management surfaces consuming plant-to-residue transfers
  - Scheduler and PL kernel boundaries consuming projected management transition controls
evidence_level: static
last_reviewed: 2026-05-23
supersedes: []
superseded_by: []
---

# SC-PLANT-001 Plant Growth Process Contract

Status: `in_review`
Maturity: `draft`
Evidence mode: `static`

## Purpose

Define top-down scientific authority for plant growth behavior, plant-driven
coupling surfaces, and PL transition-control projection semantics used by
openWEPP hydrology, erosion, residue, and scheduler dispatch domains.

## Scientific Scope

In scope:
- Cropland and rangeland plant-state evolution used by WEPP plant growth
  component semantics.
- Plant biomass, canopy, root, and yield state/flux invariants.
- Required producer/consumer boundaries between plant growth and Chapter 5
  (water balance), Chapter 9 (residue), and Chapter 11 (erosion) domains.
- Runtime projection authority for annual/perennial transition-control payloads
  required by PL branch selection and event sequencing.

Out of scope:
- Kernel implementation details and data-structure layout.
- Nutrient, pest, and aeration stress process modeling not implemented by WEPP.
- Non-plant domains except boundary definitions required for coupling safety.

## Authority Anchors

| Anchor ID | Source anchor | Contract use | Evidence |
|---|---|---|---|
| REF-PLANT-CH8-INTRO | `references/50201000/chap8.pdf` §8.1 | Declares plant outputs and cross-domain coupling to Chapters 5, 9, 11. | `[DIRECT][Static]` |
| REF-PLANT-CH8-PHENO | `chap8.pdf` §8.2, Eq. [8.2.1]-[8.2.2] | Cropland heat-unit and maturity-index semantics. | `[DIRECT][Static]` |
| REF-PLANT-CH8-GROWTH | `chap8.pdf` §8.2.1, Eq. [8.2.3]-[8.2.5] | Potential biomass and daily biomass accumulation semantics. | `[DIRECT][Static]` |
| REF-PLANT-CH8-STRESS | `chap8.pdf` §8.2.4, Eq. [8.2.14]-[8.2.16] | Water/temperature stress boundedness and growth regulation. | `[DIRECT][Static]` |
| REF-PLANT-CH8-SENESCENCE | `chap8.pdf` §8.2.3, Eq. [8.2.9]-[8.2.13] | Canopy decline and live-biomass to flat-residue transfer semantics. | `[DIRECT][Static]` |
| REF-PLANT-CH8-ROOT | `chap8.pdf` §8.2.7, Eq. [8.2.20]-[8.2.25] | Root biomass partitioning and root-depth upper bounds. | `[DIRECT][Static]` |
| REF-PLANT-CH8-MGMT | `chap8.pdf` §8.3-§8.5 | Management conversion/removal constraints (harvest, grazing, dormancy, burning). | `[DIRECT][Static]` |
| REF-PLANT-CH8-RANGE | `chap8.pdf` §8.4-§8.5 | Rangeland growth-curve (`gi`) and dormancy/stress transfer semantics. | `[DIRECT][Static]` |
| REF-PLANT-CH5-COUPLING | `references/50201000/chap5.pdf` §5.5 | ET/water-balance receives daily LAI, root depth, biomass, residue cover; returns plant water-stress factor. | `[DIRECT][Static]` |
| REF-PLANT-CH9-COUPLING | `references/50201000/chap9.pdf` §9.2, §9.4 | Residue domain consumes standing/flat/root biomass transfers and management outcomes. | `[DIRECT][Static]` |
| REF-PLANT-CH11-COUPLING | `references/50201000/chap11.pdf` §11.6 | Erosion adjustments depend on canopy/surface cover and residue surfaces from plant/residue routines. | `[DIRECT][Static]` |
| REF-PLANT-LEGACY-INFILE | `/workdir/wepp-forest_260430_baseline/src/infile.for:1115-1220` | Yearly-scenario decoding of annual extension controls and perennial event/cycle payload arrays. | `[DIRECT][Static]` |
| REF-PLANT-LEGACY-TILAGE | `/workdir/wepp-forest_260430_baseline/src/tilage.for:234-417` | Runtime schedule expansion and branch-specific assignment of annual/perennial transition controls. | `[DIRECT][Static]` |
| REF-PLANT-LEGACY-CUTGRZ | `/workdir/wepp-forest_260430_baseline/src/cutgrz.for:18-41` | Perennial harvest-date progression semantics through cut and grazing cycles. | `[DIRECT][Static]` |
| REF-PLANT-LEGACY-PTGRP | `/workdir/wepp-forest_260430_baseline/src/ptgrp.for:351-375` | Grazing day-window and `ncycle`-bounded cycle progression semantics. | `[DIRECT][Static]` |
| REF-PLANT-LEGACY-PTGRA | `/workdir/wepp-forest_260430_baseline/src/ptgra.for:188-291` | Annual event-day trigger precedence and event-day reset behavior for growth state. | `[DIRECT][Static]` |
| REF-PLANT-LEGACY-DECOMP | `/workdir/wepp-forest_260430_baseline/src/decomp.for:666-714` | Residue/decomposition event handling for annual extension controls (`jdburn`, `jdcut`, `jdmove`). | `[DIRECT][Static]` |
| REF-PLANT-LEGACY-INIDAT | `/workdir/wepp-forest_260430_baseline/src/inidat.for:613-647` | Zero-sentinel initialization and default domains for management schedule arrays. | `[DIRECT][Static]` |
| REF-PLANT-INFILE-CONTRACT | `docs/specifications/science-contracts/contracts/SC-INFILE-MANAGEMENT-001.md` §3, §8 | Canonical field domains for `jd*`, `ncut`, `cutday`, `ncycle`, `gday`, `gend`, payload arrays. | `[DIRECT][Static]` |
| REF-PLANT-PHYS-BOUNDS | Physical/common-sense invariant class | Non-negative mass/depth and bounded fractions are required for physically valid state. | `[INFERENCE][Static]` |

## Variables and Units (Externally Relevant)

| Symbol | Units | Meaning | Producer | Consumer |
|---|---|---|---|---|
| `HU` | `degC day` | Daily heat-unit increment. | plant growth | phenology logic |
| `HUI` | `fraction` | Heat-unit index (`0` at planting, `1` at physiological maturity). | plant growth | growth/senescence/yield logic |
| `Bm` | `kg m^-2` | Live above-ground biomass. | plant growth | yield, senescence, management, ET coupling |
| `Brt` | `kg m^-2` | Total live root biomass. | plant growth | root partitioning, coupling checks |
| `Rd` | `m` | Root depth. | plant growth | ET root-zone distribution (Chapter 5) |
| `LAI` | `m^2 m^-2` | Leaf area index. | plant growth | ET and interception components |
| `Cc` | `fraction` | Canopy cover (`0..1`). | plant growth | erosion and interception coupling |
| `Hc` | `m` | Canopy height. | plant growth | erosion/interception coupling |
| `YLD` | `kg m^-2` | Economic yield. | plant growth | output/reporting, management evaluation |
| `Mf` | `kg m^-2` | Flat residue mass. | plant growth / residue mgmt | residue decomposition and erosion cover |
| `Ms` | `kg m^-2` | Standing residue/standing dead mass. | plant growth / residue mgmt | residue decomposition and cover |
| `WS` | `fraction` | Water-stress factor (`0..1`). | ET/water-balance coupling | plant growth regulation |
| `TS` | `fraction` | Temperature-stress factor (`0..1`). | plant growth | plant growth regulation |
| `REG` | `fraction` | Growth regulation factor `min(WS, TS)`. | plant growth | daily biomass update |
| `EP` / `Etp` | `m d^-1` | Potential plant transpiration demand surfaces. | ET component | plant stress/uptake logic |
| `u_l` | `mm` | Layer water use for plant uptake by soil layer `l`. | ET component | water-stress computation |
| `DeltaBp`, `DeltaBi` | `kg m^-2 d^-1` | Potential and stress-adjusted daily biomass increment. | plant growth | daily biomass update invariants |
| `Rdx` | `m` | Crop/community maximum root depth parameter. | plant parameterization | root-depth envelope invariant |
| `CRITVM` | `kg m^-2` | Critical lower biomass floor under heavy grazing (where defined). | plant parameterization | grazing management invariant |
| `gi` | `fraction` | Rangeland growth-curve increment (`0..1` progression). | rangeland growth submodel | rangeland growth/dormancy gating |
| `RGCMIN` | `fraction` | Minimum live-biomass growth-curve floor for evergreen behavior. | rangeland growth submodel | allowed evergreen degenerate behavior |
| `jdherb`, `jdburn`, `jdslge`, `jdcut`, `jdmove`, `jdstop` | day-of-year (`integer`) | Annual/perennial transition-control day triggers. `0` is sentinel only where explicitly allowed. | management schedule projection | growth/decomp event selectors |
| `fbrnag`, `fbrnog`, `frcut`, `frmove` | `fraction` | Annual extension residue-management fractions coupled to event-day triggers. | management schedule projection | decomposition residue-update branches |
| `ncut`, `ncycle` | `count` | Perennial branch cardinality controls for cut and grazing cycles. | management schedule projection | growth/decomp branch selectors |
| `cutday[k]` | day-of-year (`integer`) | Cut-event day array for perennial cutting branch, `k=1..ncut`. | management schedule projection | perennial harvest transition control |
| `gday[k]`, `gend[k]` | day-of-year (`integer`) | Grazing cycle start/end day arrays, `k=1..ncycle`. | management schedule projection | perennial grazing active-window control |
| `animal[k]`, `bodywt[k]`, `area[k]`, `digest[k]` | count, kg, m^2, fraction | Grazing-cycle payload arrays aligned with each `gday/gend` cycle entry. | management schedule projection | grazing intake/removal controls |

## Algorithm State Surfaces (PL Transition-Control Runtime Projection)

### Required Inputs

| Surface | Symbols |
|---|---|
| Slot topology and yearly references | `slot_count`, `slot_ofe_index`, `slot_rotation_index`, `slot_year_in_rotation`, `crop_slots`, `yearly_ref` |
| Branch selectors | `lanuse`, `imngmt`, `resmgt`, `mgtopt` |
| Date controls | `jdplt`, `jdharv`, `jdstop`, `jdherb`, `jdburn`, `jdslge`, `jdcut`, `jdmove`, `ncut`, `cutday[*]`, `ncycle`, `gday[*]`, `gend[*]` |
| Event payload controls | `fbrnag`, `fbrnog`, `frcut`, `frmove`, `animal[*]`, `bodywt[*]`, `area[*]`, `digest[*]` |

### Required Outputs

| Surface | Required projected symbol families |
|---|---|
| Schedule surface | `pl_schedule_slot_{slot:04}_crop_{crop:04}_{root}` for `lanuse`, `imngmt`, `jdplt`, `jdharv`, `jdstop`, `resmgt`, `mgtopt`, `ncut`, `ncycle` |
| Growth surface | `pl_growth_slot_{slot:04}_crop_{crop:04}_{root}` for branch selectors and date controls used by growth transition gating |
| Decomposition surface | `pl_decomp_slot_{slot:04}_crop_{crop:04}_{root}` for residue-management controls and perennial cycle controls |
| Annual extension controls | `..._jdherb`, `..._jdburn`, `..._jdslge`, `..._jdcut`, `..._jdmove`, `..._fbrnag`, `..._fbrnog`, `..._frcut`, `..._frmove` |
| Perennial cut arrays | `..._cutday_{event:04}` for `event=1..ncut` |
| Perennial grazing arrays | `..._gday_{cycle:04}`, `..._gend_{cycle:04}`, `..._animal_{cycle:04}`, `..._bodywt_{cycle:04}`, `..._area_{cycle:04}`, `..._digest_{cycle:04}` for `cycle=1..ncycle` |

### Mutated State Surfaces

This projection algorithm is pure with respect to plant-process state:
- no biomass/cover/residue pool mutation is allowed;
- output is deterministic symbol projection and validation only.

## Algorithm Specification (PL10b Transition-Control Authority)

1. For each resolved `(slot, crop)` schedule entry, read branch selectors
   (`lanuse`, `imngmt`) and validate supported domain.
2. Apply branch partition:
   - annual/fallow branch when `imngmt in {1,3}`;
   - perennial branch when `imngmt = 2`.
3. For annual/fallow branch:
   - emit `jdplt`, `jdharv`, `resmgt` and branch-required controls;
   - for each `resmgt` option, emit exactly one active annual extension event
     control family with corresponding fractions (`herb`, `burn`, `silage`,
     `cut`, or legacy `move`) and zero-sentinel for inactive families.
4. For perennial branch:
   - emit `jdplt`, `jdharv`, `jdstop`, `mgtopt`;
   - when `mgtopt=1`, emit `ncut` and full `cutday[k]` array for
     `k=1..ncut`, and set initial harvest progression seed to `cutday[1]`;
   - when `mgtopt=2`, emit `ncycle` and full grazing arrays
     (`gday[k]`, `gend[k]`, `animal[k]`, `bodywt[k]`, `area[k]`, `digest[k]`)
     for `k=1..ncycle`, and set initial harvest progression seed to `gend[1]`.
5. Cardinality closure rules:
   - every projected per-index symbol family must have exactly one scalar value;
   - per-index families must be contiguous from `1..N` with no holes;
   - no payload value may exist for index `k > N`.
6. Event-window ordering rules:
   - each cycle requires `gday[k] < gend[k]`;
   - day values must satisfy integer Julian domain policy (`1..366`, with
     explicit sentinel exceptions only where authorized by
     `SC-INFILE-MANAGEMENT-001`).
7. Annual event-day precedence and day-trigger behavior are preserved from
   legacy control flow:
   - silage/herbicide/harvest/burn events are date-triggered branch controls;
   - decomp-side `jdburn/jdcut/jdmove` branches preserve same-day trigger
     semantics from legacy routines.
8. Invalid branch-domain, cardinality, indexing, or date-window states are
   hard-fail typed errors. Silent clamp/default behavior is prohibited.

## Branch and Guard Table (Transition Controls)

| Branch ID | Trigger | Required symbols | Guard class | Failure posture |
|---|---|---|---|---|
| `BR-PL-TC-ANNUAL` | `imngmt in {1,3}` | `jdplt`, `jdharv`, `resmgt`, annual extension fields | runtime | typed hard-fail on missing or out-of-domain symbols |
| `BR-PL-TC-ANNUAL-RESMGT-1` | `resmgt=1` | `jdherb` | runtime | typed hard-fail on invalid day domain |
| `BR-PL-TC-ANNUAL-RESMGT-2` | `resmgt=2` | `jdburn`, `fbrnag`, `fbrnog` | runtime | typed hard-fail on invalid day/fraction domain |
| `BR-PL-TC-ANNUAL-RESMGT-3` | `resmgt=3` | `jdslge` | runtime | typed hard-fail on invalid day domain |
| `BR-PL-TC-ANNUAL-RESMGT-4` | `resmgt=4` | `jdcut`, `frcut` | runtime | typed hard-fail on invalid day/fraction domain |
| `BR-PL-TC-ANNUAL-RESMGT-5` | `resmgt=5` (legacy mode) | `jdmove`, `frmove` | runtime | typed hard-fail on invalid day/fraction domain |
| `BR-PL-TC-PERENNIAL` | `imngmt=2` | `jdplt`, `jdharv`, `jdstop`, `mgtopt` | runtime | typed hard-fail on missing branch controls |
| `BR-PL-TC-PERENNIAL-CUT` | `mgtopt=1` | `ncut`, `cutday[1..ncut]` | runtime | typed hard-fail on cardinality/index closure violation |
| `BR-PL-TC-PERENNIAL-GRAZE` | `mgtopt=2` | `ncycle`, `gday/gend/payload[1..ncycle]` | runtime | typed hard-fail on cardinality/index or day-window violation |
| `BR-PL-TC-LANUSE` | unsupported `lanuse` branch | `lanuse` | governance + runtime | explicit unsupported typed failure; no fallback projection |

## Invariants

| Invariant ID | Statement | Severity | Authority | Evidence |
|---|---|---|---|---|
| INV-PLANT-001 | State-domain boundedness: `Bm >= 0`, `Brt >= 0`, `Mf >= 0`, `Ms >= 0`, `Rd >= 0`, `Hc >= 0`, `LAI >= 0`, `YLD >= 0`, and `0 <= Cc <= 1`. | hard-fail | REF-PLANT-CH8-INTRO, REF-PLANT-PHYS-BOUNDS | `[DIRECT][Static] + [INFERENCE][Static]` |
| INV-PLANT-002 | Cropland phenology bounds and gating: for cropland submodel paths only, `0 <= HUI <= 1`; annual growth does not proceed at/under base temperature and stops when maturity (`HUI = 1`) is reached. | hard-fail | REF-PLANT-CH8-PHENO, REF-PLANT-CH8-MGMT (model summary) | `[DIRECT][Static]` |
| INV-PLANT-003 | Stress boundedness: `0 <= WS <= 1`, `0 <= TS <= 1`, and `REG = min(WS, TS)` with `0 <= REG <= 1`; adjusted biomass update must use `DeltaBi = DeltaBp * REG`. | hard-fail | REF-PLANT-CH8-STRESS | `[DIRECT][Static]` |
| INV-PLANT-004 | Senescence transfer closure: daily reduction in live above-ground biomass attributable to senescence is added to flat residue mass in the same step (signed conservation for the transfer pair). | hard-fail | REF-PLANT-CH8-SENESCENCE, REF-PLANT-CH9-COUPLING | `[DIRECT][Static] + [INFERENCE][Static]` |
| INV-PLANT-005 | Root-depth envelope: computed root depth cannot exceed crop maximum root depth (`Rdx`) or configured maximum soil depth; root-mass partitioning follows declared depth-zone logic. | hard-fail | REF-PLANT-CH8-ROOT | `[DIRECT][Static]` |
| INV-PLANT-006 | Management-removal bound: harvest, grazing, herbicide, and burn operations may convert/remove biomass only from available pools; no operation may produce negative residual pool mass; grazing floor (`CRITVM`) is respected where defined. | hard-fail | REF-PLANT-CH8-MGMT | `[DIRECT][Static] + [INFERENCE][Static]` |
| INV-PLANT-007 | Coupling payload completeness: plant component emits required state surfaces to water-balance/ET (`LAI`, `Rd`, biomass/residue descriptors, stress linkage), erosion (canopy cover/height and cover context), and residue components (senescence/management transfers). | hard-fail | REF-PLANT-CH8-INTRO, REF-PLANT-CH5-COUPLING, REF-PLANT-CH9-COUPLING, REF-PLANT-CH11-COUPLING | `[DIRECT][Static] + [INFERENCE][Static]` |
| INV-PLANT-008 | Rangeland stress-transfer caps: drought-stress-driven daily conversion from standing live to standing dead is bounded by chapter-defined daily limits (3% for old standing live; 5% for old standing dead transfer/depletion constraint). | hard-fail | REF-PLANT-CH8-RANGE | `[DIRECT][Static]` |
| INV-PLANT-009 | Explicit model-limit invariant: nutrient/pest/aeration stress is not natively simulated by plant routines; any such effects must be represented through explicit parameterization/inputs, not hidden default factors. | governance-fail | REF-PLANT-CH8-MGMT (yield-adjustment and model-summary limitations) | `[DIRECT][Static] + [INFERENCE][Static]` |
| INV-PLANT-010 | Rangeland growth-curve bounds and gating: for rangeland submodel paths, `0 <= gi <= 1`; growth period initiation occurs when `gi > 0.001`, and growth for the period stops once `gi` reaches `1.0`. | hard-fail | REF-PLANT-CH8-RANGE | `[DIRECT][Static]` |
| INV-PLANT-011 | Annual-extension projection completeness: annual branch emits option-correct transition controls (`jdherb/jdburn/jdslge/jdcut/jdmove` + corresponding fractions) with mutually exclusive branch activation and explicit zero-sentinel for inactive option families. | hard-fail | REF-PLANT-LEGACY-INFILE, REF-PLANT-LEGACY-TILAGE, REF-PLANT-INFILE-CONTRACT | `[DIRECT][Static] + [INFERENCE][Static]` |
| INV-PLANT-012 | Perennial cardinality and payload closure: `ncut` and `ncycle` cardinalities bound exactly the indexed projection families (`cutday`, `gday`, `gend`, and grazing payload arrays) with no sparse indices or overflow indices. | hard-fail | REF-PLANT-LEGACY-INFILE, REF-PLANT-LEGACY-TILAGE, REF-PLANT-LEGACY-CUTGRZ | `[DIRECT][Static] + [INFERENCE][Static]` |
| INV-PLANT-013 | Grazing-window ordering: for each cycle `k`, `gday[k] < gend[k]`; cycle progression is bounded by `ncycle`, and harvest progression follows cycle-end day ordering semantics. | hard-fail | REF-PLANT-LEGACY-PTGRP, REF-PLANT-LEGACY-CUTGRZ, REF-PLANT-INFILE-CONTRACT | `[DIRECT][Static] + [INFERENCE][Static]` |
| INV-PLANT-014 | Event-day domain validity: all projected day controls are integer Julian values in contract domain (`1..366`), with `0` allowed only for explicitly documented sentinel fields/branches. | hard-fail | REF-PLANT-INFILE-CONTRACT, REF-PLANT-LEGACY-INIDAT | `[DIRECT][Static] + [INFERENCE][Static]` |
| INV-PLANT-015 | Projection failure posture: invalid transition-control runtime projection domains must hard-fail as typed errors; silent defaults or clamps are prohibited. | hard-fail | REF-PLANT-LEGACY-TILAGE, REF-PLANT-INFILE-CONTRACT | `[INFERENCE][Static]` |

## Allowed Degenerate States

| Degenerate state | Allowed condition | Rationale |
|---|---|---|
| Bare/fallow surface | `Bm = 0`, `Brt = 0`, `Cc = 0`, `Hc = 0`, `LAI = 0` | No active crop is a valid simulation state. |
| Dormant perennial | Above-ground live state near zero while root state remains positive | Chapter 8 dormancy transitions permit this behavior. |
| Senescence completion | Live biomass reduced while flat/standing residue pools increase | Expected transfer from plant to residue domain. |
| Full water stress day | `WS = 0` and `REG = 0` | Growth can halt under severe water stress without violating physics. |
| Evergreen floor behavior | `gi` lower-bounded by `RGCMIN` for evergreen communities | Chapter 8 rangeland formulation permits non-zero baseline live biomass. |
| Inactive annual extension controls | Non-selected annual event controls represented as explicit `0` sentinel fields | Legacy branch assignment writes non-selected annual controls to zero. |

## Invalid States

- Any negative biomass or residue mass (`Bm`, `Brt`, `Mf`, `Ms`) beyond numeric tolerance. `[DIRECT][Static] + [INFERENCE][Static]`
- `Cc`, `WS`, `TS`, `REG`, `HUI`, or `gi` outside `[0,1]` on applicable submodel paths. `[DIRECT][Static] + [INFERENCE][Static]`
- `Rd` greater than `min(Rdx, configured max soil depth)`. `[DIRECT][Static]`
- Management conversion/removal larger than available biomass pool. `[DIRECT][Static] + [INFERENCE][Static]`
- Missing required plant coupling outputs at daily boundary handoff. `[DIRECT][Static]`
- Hidden nutrient/pest/aeration stress multiplier not declared in inputs/contract. `[DIRECT][Static] + [INFERENCE][Static]`
- Annual extension branch with conflicting active controls (for example non-zero `jdburn` and `jdherb` under same branch) without explicit policy allowance. `[DIRECT][Static] + [INFERENCE][Static]`
- `ncut`/`ncycle` cardinality mismatch with indexed payload families. `[DIRECT][Static] + [INFERENCE][Static]`
- Grazing cycle where `gday[k] >= gend[k]`. `[DIRECT][Static] + [INFERENCE][Static]`
- Transition-control projection domain violations handled through silent default or clamp. `[INFERENCE][Static]`

## Producer Obligations

- OBL-PLANT-P-001: Emit daily plant state surfaces (`Bm`, `Brt`, `Rd`, `LAI`, `Cc`, `Hc`, residue-transfer quantities) with declared units. `[DIRECT][Static] + [INFERENCE][Static]`
- OBL-PLANT-P-002: Enforce all `INV-PLANT-*` bounds before publishing boundary payloads. `[INFERENCE][Static]`
- OBL-PLANT-P-003: Apply management events as explicit state transitions with non-negative residual pools. `[DIRECT][Static] + [INFERENCE][Static]`
- OBL-PLANT-P-004: Surface typed boundary errors when invalid plant state or missing coupling payload occurs. `[INFERENCE][Static]`
- OBL-PLANT-P-005: Emit full annual extension payload controls when annual/fallow branch is active, with explicit zero-sentinel for non-selected option families. `[DIRECT][Static] + [INFERENCE][Static]`
- OBL-PLANT-P-006: Emit full indexed perennial payload families for `cutday` and grazing cycles exactly through declared cardinalities. `[DIRECT][Static] + [INFERENCE][Static]`
- OBL-PLANT-P-007: Reject invalid transition-control runtime projection domains with typed failures; no silent coercion. `[INFERENCE][Static]`

## Consumer Obligations

- OBL-PLANT-C-001: Water-balance/ET consumer must treat plant-provided units exactly as declared and must return stress-linked surfaces consistently. `[DIRECT][Static] + [INFERENCE][Static]`
- OBL-PLANT-C-002: Erosion consumer must not assume canopy/cover inputs outside declared domains and must fail explicitly on invalid payloads. `[DIRECT][Static] + [INFERENCE][Static]`
- OBL-PLANT-C-003: Residue consumer must preserve mass-accounting semantics for plant-to-residue transfers. `[DIRECT][Static] + [INFERENCE][Static]`
- OBL-PLANT-C-004: All consumers must propagate invariant-violation context without silent clamping/defaulting. `[INFERENCE][Static]`
- OBL-PLANT-C-005: Scheduler/growth/decomp consumers must treat transition-control symbol families as deterministic indexed sets; missing/extra indices are hard failures. `[INFERENCE][Static]`

## Boundary Disposition

| Invariant family | Detection point | Disposition | Comparator tier impact | Evidence |
|---|---|---|---|---|
| State-domain bounds (`INV-PLANT-001/002/003/005/010`) | plant daily update before publish | Hard error; boundary payload rejected; violation logged with invariant ID | Tier-A/B gate | `[DIRECT][Static] + [INFERENCE][Static]` |
| Transfer closure (`INV-PLANT-004`) | senescence/management transfer step | Hard error if closure residual exceeds tolerance; require fix before promotion | Tier-A gate | `[DIRECT][Static] + [INFERENCE][Static]` |
| Management removal bounds (`INV-PLANT-006/008`) | management event application | Hard error and event rejection on impossible removal/conversion | Tier-A gate | `[DIRECT][Static] + [INFERENCE][Static]` |
| Coupling completeness (`INV-PLANT-007`) | plant->consumer handoff | Hard error on missing/invalid field; no fallback payload synthesis | Tier-A/B gate | `[DIRECT][Static] + [INFERENCE][Static]` |
| Transition-control projection closure (`INV-PLANT-011/012/013/014/015`) | parser->runtime projection boundary and scheduler pre-dispatch | Hard error on missing/incoherent/index-invalid/out-of-domain control surface | Tier-A gate for PL transition execution; Tier-B investigation otherwise | `[DIRECT][Static] + [INFERENCE][Static]` |
| Model-limit governance (`INV-PLANT-009`) | review/verification and runtime config audit | Governance failure; requires explicit contract amendment before promotion | Governance gate | `[DIRECT][Static] + [INFERENCE][Static]` |

## Guard Map

| Invariant ID | Guard class | Enforcement path | Failure behavior | Gate impact | Evidence |
|---|---|---|---|---|---|
| `INV-PLANT-001` | runtime | Plant-state validation before boundary publish | Typed hard error; payload rejected | Tier-A/B gate | `[DIRECT][Static] + [INFERENCE][Static]` |
| `INV-PLANT-002` | runtime | Cropland phenology update path (`HUI`/temperature gate checks) | Typed hard error; daily step invalidated | Tier-A/B gate | `[DIRECT][Static] + [INFERENCE][Static]` |
| `INV-PLANT-003` | runtime | Stress-regulation computation before biomass increment update | Typed hard error on out-of-domain stress/regulator | Tier-A/B gate | `[DIRECT][Static] + [INFERENCE][Static]` |
| `INV-PLANT-004` | runtime | Senescence transfer-accounting step | Typed hard error on residual > tolerance | Tier-A gate | `[DIRECT][Static] + [INFERENCE][Static]` |
| `INV-PLANT-005` | runtime | Root-depth computation and envelope check | Typed hard error when depth exceeds envelope | Tier-A/B gate | `[DIRECT][Static] + [INFERENCE][Static]` |
| `INV-PLANT-006` | runtime | Management-event biomass conversion/removal handlers | Typed hard error; operation rejected | Tier-A gate | `[DIRECT][Static] + [INFERENCE][Static]` |
| `INV-PLANT-007` | runtime | Plant-to-consumer boundary payload validator | Typed hard error on missing/invalid required field | Tier-A/B gate | `[DIRECT][Static] + [INFERENCE][Static]` |
| `INV-PLANT-008` | runtime | Rangeland drought-stress transfer update | Typed hard error on cap violation | Tier-A gate | `[DIRECT][Static] + [INFERENCE][Static]` |
| `INV-PLANT-009` | governance | Review + verification + promotion checklist | Promotion `HOLD` until explicit contract amendment resolves mismatch | Governance gate | `[DIRECT][Static] + [INFERENCE][Static]` |
| `INV-PLANT-010` | runtime | Rangeland growth-curve/dormancy transition checks | Typed hard error on growth-domain violation | Tier-A/B gate | `[DIRECT][Static] + [INFERENCE][Static]` |
| `INV-PLANT-011` | runtime | Annual branch projection validator (`resmgt`-scoped fields and exclusivity) | Typed hard error; reject annual control payload | Tier-A gate for PL transition execution | `[DIRECT][Static] + [INFERENCE][Static]` |
| `INV-PLANT-012` | runtime | Indexed projection validator for `ncut`/`ncycle` families | Typed hard error; reject projection payload | Tier-A gate for PL transition execution | `[DIRECT][Static] + [INFERENCE][Static]` |
| `INV-PLANT-013` | runtime | Grazing window validator before cycle dispatch | Typed hard error; reject invalid cycle | Tier-A gate for PL transition execution | `[DIRECT][Static] + [INFERENCE][Static]` |
| `INV-PLANT-014` | runtime | Date-domain validator for transition-control symbols | Typed hard error; reject out-of-domain date | Tier-A gate | `[DIRECT][Static] + [INFERENCE][Static]` |
| `INV-PLANT-015` | runtime | Projection error policy guard | Typed hard error only; silent default/clamp is forbidden | Tier-A gate | `[INFERENCE][Static]` |

## Symbol Alias Map

Canonical symbols in this contract follow WEPP chapter notation and legacy
lineage names by default. openWEPP boundary/API naming may diverge; this table
states required deterministic alias mapping for transition-control projections.

| Canonical symbol | Boundary/API name | Scope | Units check | Evidence |
|---|---|---|---|---|
| `HU` | `HU` (identity) | plant daily phenology surface | `degC day` -> `degC day` | `[DIRECT][Static]` |
| `HUI` | `HUI` (identity) | plant daily phenology surface | `fraction` -> `fraction` | `[DIRECT][Static]` |
| `Bm` | `Bm` (identity) | plant state export surface | `kg m^-2` -> `kg m^-2` | `[DIRECT][Static]` |
| `Brt` | `Brt` (identity) | plant state export surface | `kg m^-2` -> `kg m^-2` | `[DIRECT][Static]` |
| `Rd` | `Rd` (identity) | plant->ET coupling surface | `m` -> `m` | `[DIRECT][Static]` |
| `LAI` | `LAI` (identity) | plant->ET/erosion coupling surface | `m^2 m^-2` -> `m^2 m^-2` | `[DIRECT][Static]` |
| `Cc` | `Cc` (identity) | plant->erosion coupling surface | `fraction` -> `fraction` | `[DIRECT][Static]` |
| `Hc` | `Hc` (identity) | plant->erosion coupling surface | `m` -> `m` | `[DIRECT][Static]` |
| `YLD` | `YLD` (identity) | output/reporting surface | `kg m^-2` -> `kg m^-2` | `[DIRECT][Static]` |
| `Mf` | `Mf` (identity) | plant->residue coupling surface | `kg m^-2` -> `kg m^-2` | `[DIRECT][Static]` |
| `Ms` | `Ms` (identity) | plant->residue coupling surface | `kg m^-2` -> `kg m^-2` | `[DIRECT][Static]` |
| `WS` | `WS` (identity) | ET stress return surface | `fraction` -> `fraction` | `[DIRECT][Static]` |
| `TS` | `TS` (identity) | plant stress surface | `fraction` -> `fraction` | `[DIRECT][Static]` |
| `REG` | `REG` (identity) | plant growth regulator surface | `fraction` -> `fraction` | `[DIRECT][Static]` |
| `EP` / `Etp` | `EP` / `Etp` (identity) | ET demand surface | `m d^-1` -> `m d^-1` | `[DIRECT][Static]` |
| `u_l` | `u_l` (identity) | layered uptake surface | `mm` -> `mm` | `[DIRECT][Static]` |
| `DeltaBp`, `DeltaBi` | `DeltaBp`, `DeltaBi` (identity) | biomass increment surfaces | `kg m^-2 d^-1` -> `kg m^-2 d^-1` | `[DIRECT][Static]` |
| `Rdx` | `Rdx` (identity) | root-envelope parameter surface | `m` -> `m` | `[DIRECT][Static]` |
| `CRITVM` | `CRITVM` (identity) | grazing-floor parameter surface | `kg m^-2` -> `kg m^-2` | `[DIRECT][Static]` |
| `gi` | `gi` (identity) | rangeland growth-curve surface | `fraction` -> `fraction` | `[DIRECT][Static]` |
| `RGCMIN` | `RGCMIN` (identity) | evergreen floor parameter surface | `fraction` -> `fraction` | `[DIRECT][Static]` |
| `jdherb` | `pl_growth_slot_{slot:04}_crop_{crop:04}_jdherb` | annual extension trigger surface | day-of-year integer -> day-of-year integer | `[DIRECT][Static] + [INFERENCE][Static]` |
| `jdburn` | `pl_decomp_slot_{slot:04}_crop_{crop:04}_jdburn` | annual extension trigger surface | day-of-year integer -> day-of-year integer | `[DIRECT][Static] + [INFERENCE][Static]` |
| `jdslge` | `pl_growth_slot_{slot:04}_crop_{crop:04}_jdslge` | annual extension trigger surface | day-of-year integer -> day-of-year integer | `[DIRECT][Static] + [INFERENCE][Static]` |
| `jdcut` | `pl_decomp_slot_{slot:04}_crop_{crop:04}_jdcut` | annual extension trigger surface | day-of-year integer -> day-of-year integer | `[DIRECT][Static] + [INFERENCE][Static]` |
| `jdmove` | `pl_decomp_slot_{slot:04}_crop_{crop:04}_jdmove` | annual extension trigger surface | day-of-year integer -> day-of-year integer | `[DIRECT][Static] + [INFERENCE][Static]` |
| `fbrnag` / `fbrnog` | `pl_decomp_slot_{slot:04}_crop_{crop:04}_fbrnag` / `..._fbrnog` | annual extension fraction controls | fraction -> fraction | `[DIRECT][Static] + [INFERENCE][Static]` |
| `frcut` / `frmove` | `pl_decomp_slot_{slot:04}_crop_{crop:04}_frcut` / `..._frmove` | annual extension fraction controls | fraction -> fraction | `[DIRECT][Static] + [INFERENCE][Static]` |
| `ncut` / `cutday[k]` | `..._ncut`, `..._cutday_{event:04}` | perennial cut control surface | count/day -> count/day | `[DIRECT][Static] + [INFERENCE][Static]` |
| `ncycle` / `gday[k]` / `gend[k]` | `..._ncycle`, `..._gday_{cycle:04}`, `..._gend_{cycle:04}` | perennial grazing control surface | count/day -> count/day | `[DIRECT][Static] + [INFERENCE][Static]` |
| `animal[k]` / `bodywt[k]` / `area[k]` / `digest[k]` | `..._animal_{cycle:04}`, `..._bodywt_{cycle:04}`, `..._area_{cycle:04}`, `..._digest_{cycle:04}` | perennial grazing payload surface | count/kg/m^2/fraction -> same | `[DIRECT][Static] + [INFERENCE][Static]` |

## Constants and Parameters Table

| Constant/parameter | Units | Domain | Contract use | Authority |
|---|---|---|---|---|
| `JULIAN_DAY_MIN` | day-of-year | `1` | lower bound for active day controls | REF-PLANT-INFILE-CONTRACT |
| `JULIAN_DAY_MAX` | day-of-year | `366` | upper bound for active day controls | REF-PLANT-INFILE-CONTRACT |
| `ZERO_SENTINEL` | day/count | `0` | inactive/unspecified optional controls only where explicitly allowed | REF-PLANT-LEGACY-INIDAT, REF-PLANT-INFILE-CONTRACT |
| `CUT_INDEX_ORIGIN` | index | `1` | first valid cut event index | REF-PLANT-LEGACY-TILAGE |
| `CYCLE_INDEX_ORIGIN` | index | `1` | first valid grazing cycle index | REF-PLANT-LEGACY-TILAGE |

## Tolerance and Numeric Notes

This contract follows `docs/numerics/README.md` (semantic parity, not bitwise
parity). Contract-specific tolerances used for comparator interpretation:

| Tolerance ID | Definition | Value | Notes |
|---|---|---|---|
| TOL-PLANT-001 | Senescence transfer closure residual: `abs((Bm(i-1)-Bm(i)) - (Mf(i)-Mf(i-1)))` | `<= 1e-10 kg m^-2` | Applies only to the Eq. [8.2.13] transfer pair. |
| TOL-PLANT-002 | Fraction-domain tolerance for `{Cc, WS, TS, REG, HUI, gi}` | `abs(bound violation) <= 1e-12` allowed for comparator noise only | Runtime must not silently clamp; typed error if materially out-of-domain. |
| TOL-PLANT-003 | Non-negative-domain tolerance for biomass/depth states | lower bound `>= -1e-12` for comparator interpretation | Runtime violation remains explicit when negative beyond tolerance. |
| TOL-PLANT-004 | Transition-control day-domain tolerance | none (`integer exact`) | Date controls are integral; non-integral representation is invalid projection state. |

## Test-Vector Obligations

Minimum required scenario families for contract conformance:

1. Annual herbicide branch: `resmgt=1` projects `jdherb` and keeps non-selected
   annual extension families at explicit zero sentinel.
2. Annual burn branch: `resmgt=2` projects `jdburn`, `fbrnag`, `fbrnog`.
3. Annual cutting/removal branches: `resmgt=4` projects `jdcut`, `frcut`;
   legacy removal branch projects `jdmove`, `frmove` where supported.
4. Perennial cutting branch: `mgtopt=1` projects `ncut` and complete indexed
   `cutday` family with contiguous `1..ncut` coverage.
5. Perennial grazing branch: `mgtopt=2` projects `ncycle`, complete
   `gday/gend` arrays, and aligned grazing payload arrays (`animal`, `bodywt`,
   `area`, `digest`) through `1..ncycle`.
6. Invalid domain rejects:
   - `ncut` mismatch vs `cutday` length;
   - `ncycle` mismatch vs grazing array lengths;
   - `gday >= gend` for any cycle;
   - invalid day domain (non-integral, `<0`, `>366`).
7. Failure posture assertion: invalid projection states surface typed failures
   and never silent defaults/clamps.

## Gap Register

| Gap ID | Statement | Impact | Promotability | Evidence |
|---|---|---|---|---|
| GAP-PLANT-001 | Per-equation comparator vectors for `INV-PLANT-*` are not yet curated in this package. | Limits immediate regression-gate automation for each invariant family. | promotable-with-risk | `[DIRECT][Static]` |
| GAP-PLANT-002 | Nutrient/pest/aeration coupling is outside current WEPP plant routines and remains parameterization-only. | Reduces causal fidelity for yield stress attribution without external calibration workflow. | promotable-with-risk | `[DIRECT][Static]` |
| GAP-PLANT-003 | Legacy routine provenance is mapped at domain level (`grow.for`, `growop.for`, `range.for`) but not yet per-invariant line anchor. | Traceability for implementation-level acceptance is partial. | promotable-with-risk | `[INFERENCE][Static]` |
| GAP-PLANT-004 | Boundary contract IDs for plant consumers (`SC-WATBAL-001`, `SC-RESIDUE-001`, `SC-SED-001`) are not yet fully authored. | Cross-contract closure is provisional until downstream contracts reach draft status. | non-promotable | `[DIRECT][Static]` |
| GAP-PLANT-005 | Concrete openWEPP runtime symbol implementations for new transition-control payload families are not yet proven by conformance evidence in this contract cycle. | Prevents closing PL event-projection parity until implementation/package follow-on closes. | non-promotable | `[DIRECT][Static] + [INFERENCE][Static]` |
| GAP-PLANT-006 | Contract-level typed error taxonomy for projection-domain failures is defined, but final one-to-one mapping to runtime error codes remains open pending PL11 implementation closure. | Risk of temporary mismatch between contract and runtime failure labels. | promotable-with-risk | `[INFERENCE][Static]` |

## Revision History

| Date UTC | Version | Author | Change |
|---|---|---|---|
| `2026-05-20` | `0` | `Codex` | Initial canonical stub created by SCI-02 package prep. |
| `2026-05-20` | `1` | `Codex` | Full draft authored with invariant set, boundary obligations, and citation anchors per SCI-02 kickoff prompt. |
| `2026-05-20` | `2` | `Codex` | Post-review amendment pass: scoped cropland/rangeland invariants, added missing symbols/anchors, added claim-level evidence tags, and labeled gap promotability. |
| `2026-05-20` | `3` | `Codex` | Reopen delta for procedure update: added required invariant guard map and symbol alias map; added alias-governance gap entry. |
| `2026-05-23` | `4` | `Codex` | PL10b blind-authority amendment: added transition-control runtime-projection algorithm authority, branch/guard table, invariant family `INV-PLANT-011..015`, constants table, test-vector obligations, and explicit symbol-family alias mappings for annual/perennial payload controls. |
