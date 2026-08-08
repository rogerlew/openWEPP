---
contract_id: SC-RESIDUE-001
title: Residue Management Process Contract
status: approved
maturity: active
owner: openWEPP maintainers + hydrology reviewer
contract_version: 17
producer_scope:
  - Cropland residue and root decomposition state/flux surfaces (standing, flat, buried, root)
  - Cropland management-operation residue transitions (tillage, cutting/shredding, burning, removal)
  - Rangeland residue/litter and root decomposition state surfaces
  - Forest litter/residue mass-to-depth boundary state consumed by frost surface resistance
  - Residue cover boundary surfaces consumed by ET, runoff/erosion, and coupled domains
consumer_scope:
  - ET and water-balance consumers using residue mass/cover attenuation signals
  - Soil and erosion consumers using residue placement/cover effects on erodibility and transport
  - Plant-management and snow/freeze consumers requiring residue boundary continuity
evidence_level: Static
last_reviewed: 2026-08-08
supersedes: []
superseded_by: []
---

# SC-RESIDUE-001 Residue Management Process Contract

Status: `approved`
Maturity: `active`
Evidence mode: `Static`

## Purpose

Define top-down scientific authority for residue decomposition and residue-management
semantics in openWEPP, including mass-partitioning rules, management-operation
transitions, and coupling payloads consumed by ET, soil/erosion, and adjacent process
domains.

## Scientific Scope

In scope:
- Cropland residue partitioning and daily decomposition of standing, flat, buried, and root pools. `[DIRECT][Static]`
- Environmental modifiers and management-operation equations that update residue mass/cover states. `[DIRECT][Static]`
- Rangeland residue/litter and root decomposition semantics and management-coupled state behavior. `[DIRECT][Static]`
- Coupled boundary obligations for ET attenuation and erosion-facing cover/erodibility adjustment pathways. `[DIRECT][Static] + [INFERENCE][Static]`
- Dynamic litter/residue depth boundary semantics for snow/freeze surface heat resistance, including mass-to-depth conversion and recurring senescence deposition into the surface-residue pool. `[DIRECT][Static] + [INFERENCE][Static]`

Out of scope:
- Rust implementation details, field names, and data-structure choices. `[INFERENCE][Static]`
- Channel/watershed routing internals outside residue-management boundary payload semantics. `[INFERENCE][Static]`
- Plant-growth physiological production equations except residue-transfer interfaces crossing into this domain. `[INFERENCE][Static]`

## Authority Anchors

| Anchor ID | Source anchor | Contract use | Evidence |
|---|---|---|---|
| REF-RESIDUE-CH9-INTRO | `references/50201000/chap9.pdf` §9.1 | Declares residue decomposition/management domain boundaries and cropland/rangeland split. | `[DIRECT][Static]` |
| REF-RESIDUE-CH9-CROP-DECOMP | `references/50201000/chap9.pdf` §9.2 Eq. [9.2.1]-[9.2.13], Table 9.2.1 | Cropland residue partitioning, decomposition-day formulation, environmental factors, and standing-to-flat conversion/stubble updates. | `[DIRECT][Static]` |
| REF-RESIDUE-CH9-COVER | `references/50201000/chap9.pdf` §9.3 Eq. [9.3.1]-[9.3.11] | Flat/standing/total/ridge-rill residue-cover and ground-cover equations. | `[DIRECT][Static]` |
| REF-RESIDUE-CH9-CROP-SUMMARY | `references/50201000/chap9.pdf` §9.4 | Normative cropland update sequencing and management-date branching summary. | `[DIRECT][Static]` |
| REF-RESIDUE-CH9-MGMT | `references/50201000/chap9.pdf` §9.5 Eq. [9.5.1]-[9.5.11], Table 9.5.1 | Tillage-intensity mixing, standing-to-flat conversion under tillage, burial update, shredding/cutting, burning, and residue-removal equations. | `[DIRECT][Static]` |
| REF-RESIDUE-CH9-RANGE | `references/50201000/chap9.pdf` §9.6-§9.7 Eq. [9.7.1]-[9.7.7] | Rangeland litter/root decomposition, antecedent moisture index caps, and nondecomposable woody biomass handling. | `[DIRECT][Static]` |
| REF-RESIDUE-CH8-COUPLING | `references/50201000/chap8.pdf` §8.2.3 Eq. [8.2.9]-[8.2.13], §8.6 inputs summary | Plant senescence/management transfers into residue pools and required initialization/coupling surfaces. | `[DIRECT][Static] + [INFERENCE][Static]` |
| REF-RESIDUE-CH5-ET | `references/50201000/chap5.pdf` §5.2 Eq. [5.2.13], §5.5 | Soil-evaporation attenuation uses plant residue mass (`Cr`) and daily coupling inputs include residue cover context. | `[DIRECT][Static] + [INFERENCE][Static]` |
| REF-RESIDUE-CH11-EROSION | `references/50201000/chap11.pdf` §11.3 Eq. [11.3.7]-[11.3.10], §11.6 | Erosion adjustments include residue effects via soil-parameter adjustments and surface-cover semantics from plant/residue routines. | `[DIRECT][Static] + [INFERENCE][Static]` |
| REF-RESIDUE-CH7-RIDGE-SOIL | `references/50201000/chap7.pdf` §7.6, §7.10, §7.11 | Ridge/furrow criteria and soil erodibility pathways coupled to residue placement/cover conditions. | `[DIRECT][Static] + [INFERENCE][Static]` |
| REF-RESIDUE-LEGACY-ORATE-DOMAIN | `/workdir/wepp-forest_260430_baseline/src/infile.for:539-541`, `/workdir/wepp-forest_260430_baseline/src/decomp.for:575-633` | Legacy decomposition-rate domain authority: `oratea/orater` are consumed directly by exponential decay equations; zero-valued constants yield no-decay factors (`exp(0)=1`). | `[DIRECT][Static]` |
| REF-RESIDUE-LEGACY-RESDEP-CONVERSION | `/workdir/wepp-forest_260430_baseline/src/winter.for:247-249`, `/workdir/wepp-forest_260430_baseline/src/res_dp.for:81-126`, commit `dac3c950d8b16cc73774bf5ce2e7e11f80baac70` | Legacy residue-depth conversion authority for frost surface resistance: current ground residue mass and residue type are converted to `resdep` before frost consumes residue thickness. | `[DIRECT][Static]` |
| REF-RESIDUE-FOREST-LITTER-DECAY | Olson first-order litter decay model and forest-litter decomposition evidence (`research.fs.usda.gov/download/treesearch/55092.pdf`, long-term temperate deciduous forest-floor persistence; broadleaf forest syntheses report roughly `43%` first-year turnover, corresponding to `k≈0.56 yr^-1`) | Forest-litter fallback decay-rate class when a seasonal litter crop has recurring senescence input but zero crop-straw `oratea`; authorizes a moderate `k=0.5 yr^-1` first-order surface-litter fallback, not fixture tuning. | `[DIRECT][Static] + [INFERENCE][Static]` |
| REF-RESIDUE-FROST-COVER-20260629 | `docs/work-packages/20260629-frost-residue-cover-implementation-001/` | FROST RESIDUE-COVER IMPLEMENTATION package: Phase-0 evidence showed seasonal `Dec_*` residue mass is flat under current inputs (`sumsrm_seed=0.5 kg m^-2`, `oratea=0`, no recurring leaf-drop input), requiring both senescence deposition into surface residue and dynamic mass-to-depth frost coupling. | `[DIRECT][Ran] + [INFERENCE][Static]` |
| REF-RESIDUE-PHYS-BOUNDS | Physical/common-sense invariant class | Non-negative biomass pools, bounded fractions, and mass-transfer closure across management/decomposition operations. | `[INFERENCE][Static]` |

## Variables and Units (Externally Relevant)

| Symbol | Units | Meaning | Producer | Consumer |
|---|---|---|---|---|
| `Mrt` | `kg m^-2` | Total above-ground residue mass at harvest (source for partitioning). | plant/residue handoff | cropland residue partitioner |
| `Ms`, `Mf`, `Mb`, `Mr` | `kg m^-2` | Standing, flat, buried, and dead-root residue masses for cropland pools. | residue state updater | ET attenuation + erosion cover coupling |
| `Cr` | `kg ha^-1` | Plant residue mass input for ET attenuation relation (Eq. [5.2.13]). | residue boundary publisher | ET soil-evaporation attenuation |
| `Fpc` | `fraction` | Harvest partition control for standing vs flat residue allocation. | plant+management inputs | residue partition updater |
| `Hcut`, `Hcm` | `m` | Harvest and canopy height controls used in partition factor Eq. [9.2.3]. | plant+management inputs | residue partition updater |
| `ORATEA`, `ORATER` | `kg m^-2 d^-1` | Optimum decomposition-rate constants for above-ground residues and roots. | residue parameter surface | daily decomposition updater |
| `ENVIND`, `WFCs`, `WFCf`, `WFCb`, `TFC` | `fraction` | Environmental decomposition factors and pool-specific moisture/temperature modifiers. | residue environment updater | decomposition equation pathway |
| `PRCP` | `m` | Daily precipitation input for standing-residue water factor branch. | climate forcing | residue decomposition factors |
| `θtill`, `θopt`, `φtill` | `fraction` | Soil moisture/porosity drivers for flat and buried residue water factors. | soil coupling | residue decomposition factors |
| `Tavg` | `degC` | Daily average air temperature used by decomposition temperature function. | climate forcing | residue decomposition factors |
| `Fct` | `fraction` | Standing-to-flat adjustment factor for wind/snow flattening. | residue state updater | standing/flat transition |
| `P`, `Pm` | `plants m^-2` | Stubble population state and harvest reference population. | residue state updater | management and cover pathway |
| `Crf`, `Crs`, `Crt`, `Crr`, `Crl`, `Cri`, `Cg`, `Ccf` | `fraction` | Flat/standing/total/ridge/rill/interrill/ground cover fractions used in coupling. | residue cover updater | ET + erosion + hydrology coupling |
| `frr`, `frl` | `fraction` | Ridge/furrow area fractions for weighted cover aggregation. | ridge-furrow residue pathway | erosion cover partition coupling |
| `Mrr`, `Mrl`, `ΔMw` | `kg m^-2` | Ridge/furrow residue masses and daily transferred mass under repositioning. | ridge-furrow residue pathway | erosion cover partition coupling |
| `Ti`, `Rmf` | `fraction`, `fraction` | Tillage intensity and derived residue-mixing factor for post-tillage cover. | management-event handler | residue mass/cover update pathway |
| `Fcut`, `Fbs`, `Fbf`, `Frm` | `fraction` | Event fractions for cutting/shredding, burning, and residue removal. | management-event handler | post-event residue pools |
| `Rg`, `Brt`, `Wn`, `Ra`, `Nd` | `kg m^-2` | Rangeland litter/root/nondecomposable woody/standing-dead state terms. | rangeland residue pathway | rangeland cover and management coupling |
| `Smi`, `Sr` | `m` | Antecedent moisture indices for rangeland litter/root decomposition. | rangeland environment pathway | rangeland residue/root decomposition |
| `Cn`, `αf`, `αr`, `ωL`, `χ`, `τ`, `ν` | `fraction` | Rangeland decomposition coefficients/factors and weighted-time terms. | rangeland environment pathway | rangeland residue/root decomposition |
| `FERIND`, `PSZIND` | `fraction` | Fertility and residue particle-size decomposition modifiers. | residue updater | cropland decomposition branch logic |
| `Bc` | `kg m^-2` | Daily disappearance of rangeland litter from insects/rodents. | rangeland updater | rangeland decomposition branch logic |
| `Mlit_in` | `kg m^-2 d^-1` | Daily above-ground senescence/litter input deposited into the surface-residue pool before residue depth is published for frost. | plant-growth/residue handoff | residue decomposition and frost-residue depth boundary |
| `residue_depth_m` | `m` | Dynamic residue/litter thickness derived from the current surface-residue mass through the authorized residue-depth conversion. | residue boundary publisher | snow/freeze frost surface heat resistance |

## Algorithm State Surfaces (PL12/PL16/PL17 Transition Execution)

### Required Inputs

| Surface | Symbols |
|---|---|
| Active-slot dispatch controls | `day`, `year`, `pl_schedule_slot_*`, `pl_growth_slot_*_imngmt` |
| Decomposition seed state | `iresd_seed`, `sumrtm_seed`, `sumsrm_seed` |
| Annual transition controls | `resmgt`, `jdherb`, `jdburn`, `jdslge`, `jdcut`, `jdmove`, `fbrnag`, `fbrnog`, `frcut`, `frmove` |
| Perennial transition controls | `mgtopt`, `ncut`, `ncycle`, `cutday[*]`, `gday[*]`, `gend[*]`, `animal[*]`, `bodywt[*]`, `area[*]`, `digest[*]` |
| Decomposition equation controls | `tmax`, `tmin`, `prcp`, `Ws`, `oratea`, `orater`, optional `surface_litter_input_kg_m2`, `residue_depth_conversion_m_per_kg_m2` |
| Growth transition controls | `jdplt`, `jdharv`, `jdstop`, `rw`, `mgtopt`, runtime day-window checks |
| Growth transition state surface | `sumgdd`, `vdmt`, `cancov`, `lai`, `rtmass`, `rtd`, `hia` |
| Ordering constraints | `pl_order_decomp_before_soil`, `pl_order_growth_after_decomp`, `pl_order_watbal_after_growth` |

### Required Outputs

| Surface | Output |
|---|---|
| Typed decomposition context | management class (`annual/fallow` or `perennial`), active slot/crop identity, runtime day, seed state, transition-control payload, active day transition selector, surface-litter input when produced by growth senescence, dynamic residue-depth boundary value, and equation-updated tracked seed-pool values |
| Typed growth context | management class (`annual/fallow` or `perennial`), active slot/crop identity, runtime day, growth transition-control payload, and pre/post transition state snapshot for key growth surfaces |
| Scheduler failure surface | typed hard-fail status when required transition-control inputs are missing/non-finite/out-of-domain/non-contiguous |

### Mutated State Surfaces

At scheduler transition-dispatch boundaries, mutation authority is limited to
typed transition-context assembly and typed failure reporting. PL17 extends this
authority to include equation-updated tracked decomposition seed-pool values
inside the typed payload, while direct global runtime-surface mutation remains
delegated to kernel handlers consuming the typed contexts.

## Algorithm Specification (PL12/PL16/PL17 Scheduler Transition Authority)

1. Resolve active PL slot/crop from schedule topology and runtime `day/year`
   controls.
2. Read `imngmt` and enforce supported management-class domain:
   - annual/fallow: `imngmt in {1,3}`;
   - perennial: `imngmt = 2`.
3. Enforce decomposition-order flags (`pl_order_decomp_before_soil`,
   `pl_order_growth_after_decomp`) and required seed-state finiteness
   (`iresd_seed`, `sumrtm_seed`, `sumsrm_seed`).
4. Annual/fallow branch:
   - read `resmgt` and branch-required annual control family;
   - enforce day/fraction domains and mutually exclusive event-family
     expectations;
   - compute deterministic same-day active annual transition selector
     (`none`, `herbicide`, `burn`, `silage`, `cut`, `remove`).
5. Perennial branch:
   - read `mgtopt`, `ncut`, `ncycle`;
   - enforce indexed-family closure (contiguous `1..N`, no holes, no overflow
     indices) for `cutday` and grazing payload arrays;
   - enforce grazing-window ordering (`gday[k] < gend[k]`) and positive payload
     domains (`animal`, `bodywt`, `area`, `digest`);
   - compute deterministic same-day active perennial selector:
     `none`, `cut(event_index)`, or `grazing(cycle_index, payload)`.
6. Impossible transfer/removal state domains (for example fractions outside
   `[0,1]`, non-positive grazing payload magnitudes, or invalid day windows)
   are typed hard failures and must not be silently clamped/defaulted.
7. Emit typed decomposition context for downstream kernel execution only after
   all guards pass.
8. Growth-transition dispatch consumes active slot/crop controls and runtime
   day, and validates key growth state surfaces:
   `sumgdd`, `vdmt`, `cancov`, `lai`, `rtmass`, `rtd`, `hia`.
9. Annual/fallow growth branch emits deterministic action signaling:
   - `planting_reset` on `day == jdplt`,
   - `harvest_reset` on `day == jdharv`,
   - `none` otherwise.
10. Perennial growth branch emits deterministic action signaling:
   - `planting_reset` on `day == jdplt`,
   - `stop_reset` on `day == jdstop` when `jdstop > 0`,
   - `none` otherwise.
11. For reset actions, scheduler emits typed growth transition payload with
    explicit post-transition zero state for:
    `sumgdd`, `vdmt`, `cancov`, `lai`, `rtmass`, `rtd`, `hia`.
12. On non-reset active-growth days, growth payload `state_after` must be
    equation-derived (GDD/stress/biomass/canopy/LAI/root updates) rather than
    pass-through/no-op or unconditional zero reset.
13. Growth transition payload domains are hard-fail validated (`cancov in
    [0,0.999]`, `hia in [0,1]`, remaining surfaces non-negative); silent
    clamping/default behavior is prohibited.
14. INT10 coupled replay closure requires that growth transition dispatch
    carries `order_watbal_after_growth = 1` and that scheduler progression into
    watbal/hydrology phases occurs only after successful decomposition and
    growth transition dispatch completion.

### PL17 Decomposition Runtime Update Addendum

1. Decomposition transition payload assembly must emit equation-updated tracked
   seed-pool values (`sumrtm_seed`, `sumsrm_seed`) with the active residue-type
   selector (`iresd_seed`).
2. On active decomposition days, compute legacy-aligned temperature and water
   modifiers:
   - `tave = (tmax + tmin) / 2`
   - `tmpfac = 0` outside `(-6.1, 49.2) degC`, else
     `tmpfac = t1 * (2*t2 - t1) / t2^2`, where `t1=(tave+6.1)^2` and
     `t2=1528.81`
   - `swatfc = 0` for `tave <= 0`, `swatfc = prcp/0.004` for
     `0 < prcp < 0.004`, else `swatfc = 1`
   - `fwatfc = clamp(Ws, 0, 1)` for transition payload update closure
3. Compute environmental indices and exponential decay factors:
   - `senvin = min(tmpfac, swatfc)`
   - `envinx = min(tmpfac, fwatfc)`
   - `surface_decay = exp(-envinx * oratea)`
   - `root_decay = exp(-envinx * orater)`
   - `oratea=0` and/or `orater=0` are valid and produce no-decay factors
     (`surface_decay=1` and/or `root_decay=1`) for the affected pool updates.
4. Apply decomposition kinetics to tracked pools:
   - `sumsrm_next = (sumsrm_prev + surface_litter_input) * surface_decay`
   - `sumrtm_next = sumrtm_prev * root_decay`
5. Apply same-day annual transition modifiers when active:
   - `burn`: `sumsrm_next *= (1 - fbrnog)`
   - `remove`: `sumsrm_next *= (1 - frmove)`
   - `cut`: explicit transition-pool transfer update
     `transfer = sumsrm_next * frcut`,
     `sumsrm_next -= transfer`, `sumrtm_next += transfer`
6. Apply same-day perennial transition modifiers when active:
   - `grazing(cycle)`: remove digest-dependent fraction from tracked surface
     pool, bounded to `[0,1]`.
7. Decomposition payload state domains are hard-fail validated:
   non-finite values, negative pool masses, invalid fraction domains, or
   non-finite exponential arguments are typed failures; silent
   clamp/default/fallback behavior is prohibited.
8. Growth and hydrology ordering obligations from PL16/INT10 remain unchanged:
   decomposition payload update completion is a prerequisite for downstream
   growth/watbal lane progression.
9. When the active plant-growth branch produces above-ground senescence or
   litterfall before the frost surface heat path is evaluated, the lost live
   biomass must be deposited into the tracked surface-residue pool as
   `surface_litter_input_kg_m2`. The transfer is a mass-conserving plant-to-
   residue handoff; it is not a fitted frost parameter and must not disappear
   from the residue ledger.
10. The dynamic residue-depth boundary is derived from the current
    `sumsrm_next` surface-residue mass using an authorized mass-to-depth
    conversion (`residue_depth_conversion_m_per_kg_m2`) established from the
    legacy `res_dp` lineage and the t0 residue type/packing state. The
    published `residue_depth_m` must be finite and non-negative and must track
    the current mass; the initial-condition seed is a t0 value, not a static
    season-long frost boundary once dynamic residue updates are active.

## Branch and Guard Table (PL12/PL16/PL17 Transition Controls)

| Branch ID | Trigger | Required symbols | Guard class | Failure posture |
|---|---|---|---|---|
| `BR-RES-PL12-ANNUAL` | `imngmt in {1,3}` | `resmgt` + annual transition controls | runtime | typed hard-fail on missing/non-finite/out-of-domain controls |
| `BR-RES-PL12-ANNUAL-BURN` | `resmgt=2` | `jdburn`, `fbrnag`, `fbrnog` | runtime | typed hard-fail on day/fraction domain violations |
| `BR-RES-PL12-ANNUAL-CUT` | `resmgt=4` | `jdcut`, `frcut` | runtime | typed hard-fail on day/fraction domain violations |
| `BR-RES-PL12-ANNUAL-REMOVE` | `resmgt=5` | `jdmove`, `frmove` | runtime | typed hard-fail on day/fraction domain violations |
| `BR-RES-PL12-PERENNIAL` | `imngmt=2` | `mgtopt`, `ncut`, `ncycle` | runtime | typed hard-fail on missing/non-integral/out-of-domain controls |
| `BR-RES-PL12-PERENNIAL-CUT` | `mgtopt=1` | `cutday[1..ncut]` | runtime | typed hard-fail on cardinality/index closure or day domain violation |
| `BR-RES-PL12-PERENNIAL-GRAZE` | `mgtopt=2` | `gday/gend/payload[1..ncycle]` | runtime | typed hard-fail on cardinality/index closure, day-window ordering, or payload domain violations |
| `BR-RES-PL12-PERENNIAL-DORMANT` | `mgtopt=3` | no indexed families | runtime | typed hard-fail if cut/graze payload symbols are present unexpectedly |
| `BR-RES-PL16-GROWTH-ANNUAL` | annual/fallow branch active | `jdplt`, `jdharv`, growth state surface, PL16 growth-physics symbol set | runtime | typed hard-fail on missing/non-finite/non-integral/out-of-domain growth controls/state |
| `BR-RES-PL16-GROWTH-PERENNIAL` | perennial branch active | `jdplt`, `jdharv`, `jdstop`, `mgtopt`, growth state surface, PL16 growth-physics symbol set | runtime | typed hard-fail on missing/non-finite/non-integral/out-of-domain growth controls/state |
| `BR-RES-PL16-GROWTH-RESET` | planting/harvest/stop action day | growth state surface | runtime | typed hard-fail if reset payload cannot be emitted from valid pre-state domain |
| `BR-RES-PL16-GROWTH-EQUATION` | active non-reset growth day | climate (`tmax/tmin/rad`), stress (`Ws`), projected crop-parameter symbols, growth state surface | runtime | typed hard-fail on missing/non-finite/out-of-domain equation symbols or non-equation fallback behavior |
| `BR-RES-PL17-DECOMP-EQUATION` | decomposition transition branch active | `sumrtm_seed`, `sumsrm_seed`, `tmax`, `tmin`, `prcp`, `Ws`, `oratea`, `orater` | runtime | typed hard-fail on missing/non-finite/out-of-domain decomposition equation inputs or non-equation fallback behavior |
| `BR-RES-PL17-DECOMP-EVENT-TRANSFER` | active annual/perennial decomposition management action day | `resmgt`/`mgtopt` action controls + event fractions/payloads | runtime | typed hard-fail on invalid event-domain/transfer behavior; no silent no-op fallback for covered event branches |
| `BR-RES-PL17-FROST-RESIDUE-DEPTH` | residue boundary feeds frost surface resistance | `sumsrm_seed`, `surface_litter_input_kg_m2`, `oratea`, `residue_depth_conversion_m_per_kg_m2` | runtime | typed hard-fail on non-finite/negative mass, negative litter input, invalid conversion, or static seed-depth reuse after dynamic residue mass changes |
| `BR-RES-INT10-ORDER` | coupled replay lane closure (`decomp -> growth -> watbal`) | `pl_order_decomp_before_soil`, `pl_order_growth_after_decomp`, `pl_order_watbal_after_growth` | runtime | typed hard-fail on missing/non-finite/out-of-domain ordering symbols; hydrology phase entry blocked |

## Invariants

| Invariant ID | Statement | Severity | Authority | Evidence |
|---|---|---|---|---|
| INV-RESIDUE-001 | Harvest partition closure invariant: at harvest, residue transfer from above-ground biomass must follow Eq. [9.2.1]-[9.2.3] so that standing (`Ms`) and flat (`Mf`) partitioning is explicitly controlled by `Fpc` and preserves signed transfer consistency. | hard-fail | REF-RESIDUE-CH9-CROP-DECOMP, REF-RESIDUE-CH8-COUPLING | `[DIRECT][Static] + [INFERENCE][Static]` |
| INV-RESIDUE-002 | Cropland decomposition invariant: daily pool updates for standing/flat/buried/root masses must follow Eq. [9.2.4] with explicit `ENVIND`, `ORATE`, `PSZIND`, and `FERIND` factors; emitted pool masses must be finite and non-negative within tolerance. | hard-fail | REF-RESIDUE-CH9-CROP-DECOMP, REF-RESIDUE-PHYS-BOUNDS | `[DIRECT][Static] + [INFERENCE][Static]` |
| INV-RESIDUE-003 | Environmental-factor invariant: `ENVIND = min(WFC, TFC)` per Eq. [9.2.5], with pool-specific WFC branches from Eq. [9.2.6]-[9.2.9], lower bound behavior, and temperature cutoffs applied explicitly. | hard-fail | REF-RESIDUE-CH9-CROP-DECOMP | `[DIRECT][Static]` |
| INV-RESIDUE-004 | Temperature-function invariant: `TFC` must follow Eq. [9.2.10] and branch rules (`TFC = 0` outside chapter-declared temperature bounds), with finite values bounded to valid factor range. | hard-fail | REF-RESIDUE-CH9-CROP-DECOMP, REF-RESIDUE-PHYS-BOUNDS | `[DIRECT][Static] + [INFERENCE][Static]` |
| INV-RESIDUE-005 | Standing-to-flat conversion invariant: conversion by wind/snow flattening must follow Eq. [9.2.11]-[9.2.12], and stubble-population updates must follow Eq. [9.2.13] with no silent standing-mass loss outside declared pathways. | hard-fail | REF-RESIDUE-CH9-CROP-DECOMP | `[DIRECT][Static] + [INFERENCE][Static]` |
| INV-RESIDUE-006 | Cover-mapping invariant: residue-cover computations for flat/standing/total/ridge/rill/ground cover must follow Eq. [9.3.1]-[9.3.11], preserving explicit area-weighted composition and bounded fraction semantics. | hard-fail | REF-RESIDUE-CH9-COVER, REF-RESIDUE-PHYS-BOUNDS | `[DIRECT][Static] + [INFERENCE][Static]` |
| INV-RESIDUE-007 | Ridge-furrow transfer invariant: wind/tillage repositioning between ridges and furrows must preserve signed residue-transfer bookkeeping (`ΔMw`, `Mrr`, `Mrl`) and use Eq. [9.3.5]-[9.3.10] / Eq. [9.5.1]-[9.5.5] branch semantics explicitly. | hard-fail | REF-RESIDUE-CH9-COVER, REF-RESIDUE-CH9-MGMT, REF-RESIDUE-CH7-RIDGE-SOIL | `[DIRECT][Static] + [INFERENCE][Static]` |
| INV-RESIDUE-008 | Tillage-management invariant: tillage-intensity updates (`Ti`, `Rmf`) and standing/flat/buried transitions must follow Eq. [9.5.1]-[9.5.5], including explicit handling of residue burial increments and no undeclared clamping of event effects. | hard-fail | REF-RESIDUE-CH9-MGMT | `[DIRECT][Static] + [INFERENCE][Static]` |
| INV-RESIDUE-009 | Event-management invariant: shredding/cutting, burning, and residue-removal operations must follow Eq. [9.5.6]-[9.5.11] with valid fraction domains and deterministic event-date application. | hard-fail | REF-RESIDUE-CH9-MGMT, REF-RESIDUE-CH9-CROP-SUMMARY | `[DIRECT][Static] + [INFERENCE][Static]` |
| INV-RESIDUE-010 | Rangeland decomposition invariant: litter/root update relations (Eq. [9.7.1]-[9.7.7]) must preserve documented antecedent-moisture behavior (including cap handling) and non-negative emitted state values. | hard-fail | REF-RESIDUE-CH9-RANGE, REF-RESIDUE-PHYS-BOUNDS | `[DIRECT][Static] + [INFERENCE][Static]` |
| INV-RESIDUE-011 | Coupling payload invariant: residue mass/cover outputs required by ET (`Cr` attenuation in Eq. [5.2.13]) and erosion-adjustment pathways must be present, unit-consistent, and temporally aligned with daily update sequencing. | hard-fail | REF-RESIDUE-CH5-ET, REF-RESIDUE-CH11-EROSION, REF-RESIDUE-CH9-CROP-SUMMARY | `[DIRECT][Static] + [INFERENCE][Static]` |
| INV-RESIDUE-012 | Update-order invariant: daily residue updates must preserve explicit sequence from §9.4 (decomposition, standing-to-flat conversion, cover update, management-day branch, and harvest repartition) without silent reordering. | hard-fail | REF-RESIDUE-CH9-CROP-SUMMARY | `[DIRECT][Static] + [INFERENCE][Static]` |
| INV-RESIDUE-013 | Governance-range invariant: when crop/operation parameters are applied outside chapter-declared tables or documented limits/assumptions, outputs are non-promotable unless labeled and dispositioned with explicit risk rationale. | governance-fail | REF-RESIDUE-CH9-CROP-DECOMP, REF-RESIDUE-CH9-MGMT | `[DIRECT][Static] + [INFERENCE][Static]` |
| INV-RESIDUE-014 | Growth transition state-domain invariant: scheduler growth transition payload assembly requires finite/non-negative growth state surfaces with bounded canopy and harvest-index domains (`cancov in [0,0.999]`, `hia in [0,1]`). | hard-fail | REF-RESIDUE-CH8-COUPLING, REF-RESIDUE-PHYS-BOUNDS | `[DIRECT][Static] + [INFERENCE][Static]` |
| INV-RESIDUE-015 | Growth transition reset invariant: reset-class actions (`planting`, `harvest`, `stop`) emit explicit zero-state payloads for `sumgdd`, `vdmt`, `cancov`, `lai`, `rtmass`, `rtd`, `hia` and do not rely on implicit defaults. | hard-fail | REF-RESIDUE-CH8-COUPLING, REF-RESIDUE-CH9-CROP-SUMMARY | `[DIRECT][Static] + [INFERENCE][Static]` |
| INV-RESIDUE-016 | INT10 coupled replay invariant: daily coupled transition/hydrology execution preserves `decomp -> growth -> watbal` ordering, with typed context and writeback state transfer observable by downstream hydrology phases; ordering-symbol violations are hard-fail with no silent fallback. | hard-fail | REF-RESIDUE-CH8-COUPLING, REF-RESIDUE-CH5-ET, REF-RESIDUE-PHYS-BOUNDS | `[DIRECT][Static] + [INFERENCE][Static]` |
| INV-RESIDUE-017 | PL17 decomposition equation-update invariant: active decomposition payload assembly updates tracked seed residue/root pools (`sumsrm_seed`, `sumrtm_seed`) using explicit equation-driven exponential decay factors derived from environmental indices and decomposition-rate constants; pass-through/no-op fallback is prohibited for covered branches. | hard-fail | REF-RESIDUE-CH9-CROP-DECOMP, REF-RESIDUE-CH9-MGMT, REF-RESIDUE-PHYS-BOUNDS | `[DIRECT][Static] + [INFERENCE][Static]` |
| INV-RESIDUE-018 | PL17 decomposition required-symbol guard invariant: decomposition equation and event-transfer inputs (`tmax`, `tmin`, `prcp`, `Ws`, `oratea`, `orater`, event fractions/payloads) must be present, finite, and domain-valid or runtime must hard-fail as typed boundary error; `oratea/orater` are non-negative with zero-valued no-decay constants allowed, while negative values are invalid. Silent defaults/clamps are prohibited. | hard-fail | REF-RESIDUE-CH9-CROP-DECOMP, REF-RESIDUE-CH9-MGMT, REF-RESIDUE-LEGACY-ORATE-DOMAIN, REF-RESIDUE-PHYS-BOUNDS | `[DIRECT][Static] + [INFERENCE][Static]` |
| INV-RESIDUE-019 | Dynamic frost-residue boundary invariant: when residue state is consumed by snow/freeze frost surface resistance, the published residue depth must be derived from the current surface-residue mass after same-day senescence/litter input and decomposition, using an authority-backed mass-to-depth conversion. Live above-ground biomass lost through deciduous/perennial senescence must be transferred to the surface-residue pool before the frost boundary is published; decomposition/removal/grazing are the only authorized losses. Compatibility perennial inputs without CP-GSI02 authority may retain the management-date pending bucket, but native `generalized_gsi_v1` forest litter is deposited on the physical GSI decline day and must never enter the `jdharv` window. The t0 `resdep` seed is conversion provenance, not a static later boundary. Zero `oratea` remains valid for inert/no-senescence residue; active recurring forest litter with zero `oratea` uses the authority-backed `k=0.5 yr^-1` fallback. | hard-fail | REF-RESIDUE-CH8-COUPLING, REF-RESIDUE-LEGACY-RESDEP-CONVERSION, REF-RESIDUE-FOREST-LITTER-DECAY, REF-RESIDUE-FROST-COVER-20260629, REF-RESIDUE-PHYS-BOUNDS, SC-PLANT-001#INV-PLANT-035 | `[DIRECT][Ran] + [INFERENCE][Static]` |
| INV-RESIDUE-020 | Ground-cover authority invariant (GAP-SED-009 closure, 2026-07-05): the interrill and rill ground-residue pools are runtime state — seeded at day 0 by the `init1.for:295-297` inverse from the management IC's DECLARED `inrcov`/`rilcov` and the residue plant's cover factor `cf` (`canopy_line[4]`; zero declared cover or zero `cf` seeds zero pools), evolved daily by the SAME decay factor as the surface pool with surface-litter input added to both (`decomp.for` applies the identical law to `rigrm`/`rilrm`; Burn/Remove/Grazing fractions apply to the ground pools; **Cut ADDS the cut material to both ground pools** (`decomp.for:689-693` — the cut standing-mat mass joins `rilrm`/`rigrm`/`rmogt`; our pool topology has no standing mat, so the cut-mass basis is the surface-pool transfer `surface·cut_transfer_fraction`, a labeled mapping with the source-true addition rule)). The erosion interrill/rill covers are RE-DERIVED from the pools each day by the `covcal.for:160-176` forward form `1 − exp(−cf·mass)` clamped `[0, 0.999]` — the seed/derive round trip reproduces the declared covers exactly. The standing-mat `strcov` term is 0 (the standing pool is not modeled; the term is additive-only, so its absence is conservative in the fail-direction of the closed defect — a labeled limitation). A no-decay, no-litter scenario holds the pools and covers constant (the forest no-decomp IC behavior legacy exhibits). The ground-pool seeds and `cf` fail closed at the decomposition input boundary (nonnegative-finite). The published composite `cover_fraction` is the `covcal.for:176` `rescov` area-weighted blend (`w·inrcov + (1−w)·rilcov`, `w = (rspace − width)/rspace`). | hard-fail | REF-RESIDUE-CH8-COUPLING, INV-RESIDUE-019, SC-SED-001#INV-SED-017 | `[DIRECT][Ran] + [INFERENCE][Static]` |
| INV-RESIDUE-021 | CP-GSI02 native leaf-off closure: after the no-transfer first realization, the residue-domain litter input for a native GSI forest equals the same day's `L_leaf` from `SC-PLANT-001#INV-PLANT-035`; it is added once to surface and ground litter pools before decomposition, residue-cover derivation, residue-depth conversion, and frost consumption. Cold-start aggregate-live-biomass loss, pending-date accumulation, duplicate transfer, and delayed publication are invalid. | hard-fail | REF-RESIDUE-CH8-COUPLING, REF-RESIDUE-PHYS-BOUNDS, SC-PLANT-001#INV-PLANT-035 | `[DIRECT][Static] + [INFERENCE][Static]` |
| INV-RESIDUE-022 | Authenticated external forest-litter closure: same-day `Q=L_leaf+N_ext+W_ext` is projected exactly once into each parallel surface/interrill/rill areal recurrence before common decay and actions; the states are never summed as independent global masses. Surface drives depth/frost, interrill/rill drive cover/erosion, and external `N_ext+W_ext` remains labeled open-system influx without canopy debit. | hard-fail | REF-RESIDUE-CH8-COUPLING, REF-RESIDUE-PHYS-BOUNDS, SC-PLANT-001#INV-PLANT-039 | `[DIRECT][Static] + [INFERENCE][Static]` |
| INV-RESIDUE-023 | Future vegetation retains live and standing-dead pools until one immutable accepted transfer crosses to residue/biogeochemistry. Donor and receiver independently reconstruct distinct interval-integrated dry-matter, carbon, and nitrogen operands exactly once; current CP-GSI02 and authenticated external litter authority remains active until atomic real-consumer cutover. | governance-hold | INV-RESIDUE-021, INV-RESIDUE-022, SC-VEGETATION-001#INV-VEGETATION-030, SC-VEGETATION-001#INV-VEGETATION-031 | `[DIRECT][Static] + [INFERENCE][Static]` |

## Invariant Guard Map

| Invariant ID | Guard class | Enforcement path | Failure behavior | Gate impact | Evidence |
|---|---|---|---|---|---|
| `INV-RESIDUE-001` | runtime | Harvest residue partition updater | Typed hard error on malformed partition factors or transfer mismatch | Tier-A gate | `[DIRECT][Static] + [INFERENCE][Static]` |
| `INV-RESIDUE-002` | runtime | Cropland decomposition updater | Typed hard error on non-finite/negative pool states or malformed decomposition factors | Tier-A gate | `[DIRECT][Static] + [INFERENCE][Static]` |
| `INV-RESIDUE-003` | runtime | Environmental-factor calculator | Typed hard error on invalid branch/domain behavior for WFC/ENVIND | Tier-A/B gate | `[DIRECT][Static]` |
| `INV-RESIDUE-004` | runtime | Temperature-function branch calculator | Typed hard error on invalid TFC domain/output | Tier-A/B gate | `[DIRECT][Static] + [INFERENCE][Static]` |
| `INV-RESIDUE-005` | runtime | Standing-to-flat/stubble updater | Typed hard error on undeclared standing-mass loss or invalid stubble update state | Tier-A gate | `[DIRECT][Static] + [INFERENCE][Static]` |
| `INV-RESIDUE-006` | runtime | Residue-cover calculator | Typed hard error on malformed cover fractions or area-weighting violations | Tier-A gate | `[DIRECT][Static] + [INFERENCE][Static]` |
| `INV-RESIDUE-007` | runtime | Ridge-furrow transfer updater | Typed hard error on residue-transfer imbalance or invalid ridge/rill branch state | Tier-A/B gate | `[DIRECT][Static] + [INFERENCE][Static]` |
| `INV-RESIDUE-008` | runtime | Tillage-management updater | Typed hard error on invalid `Ti` branch update or burial accounting mismatch | Tier-A gate | `[DIRECT][Static] + [INFERENCE][Static]` |
| `INV-RESIDUE-009` | runtime | Event-management handler | Typed hard error on invalid event fractions/date semantics or resulting invalid pool states | Tier-A gate | `[DIRECT][Static] + [INFERENCE][Static]` |
| `INV-RESIDUE-010` | runtime | Rangeland residue/root updater | Typed hard error on invalid antecedent-moisture/domain behavior or negative outputs | Tier-A/B gate | `[DIRECT][Static] + [INFERENCE][Static]` |
| `INV-RESIDUE-011` | runtime | Residue boundary payload validator | Typed hard error on missing/invalid ET or erosion coupling residue payloads | Tier-A gate | `[DIRECT][Static] + [INFERENCE][Static]` |
| `INV-RESIDUE-012` | runtime | Daily update-order validator | Typed hard error on silent sequencing divergence from §9.4 workflow | Tier-A/B gate | `[DIRECT][Static] + [INFERENCE][Static]` |
| `INV-RESIDUE-013` | governance | Review/disposition/promotion checklist | Promotion `HOLD` when out-of-range use lacks explicit labeling/rationale | Governance gate | `[DIRECT][Static] + [INFERENCE][Static]` |
| `INV-RESIDUE-014` | runtime | Growth transition payload state validator | Typed hard error on invalid growth state domain for transition payload assembly | Tier-A gate | `[DIRECT][Static] + [INFERENCE][Static]` |
| `INV-RESIDUE-015` | runtime | Growth transition reset payload assembler | Typed hard error when required reset-class state projection is missing or malformed | Tier-A gate | `[DIRECT][Static] + [INFERENCE][Static]` |
| `INV-RESIDUE-016` | runtime | Coupled replay transition/hydrology lane-order guard and state-transfer boundary checks | Typed hard error on missing/non-finite/invalid ordering symbols or failed transition preconditions before watbal lane | Tier-A gate for INT10 coupled replay | `[DIRECT][Static] + [INFERENCE][Static]` |
| `INV-RESIDUE-017` | runtime | Decomposition transition payload equation updater (`state_after`) for tracked residue/root seed pools | Typed hard error when covered decomposition branches emit pass-through/no-op state in place of equation update | Tier-A gate for PL17 decomposition physics closure | `[DIRECT][Static] + [INFERENCE][Static]` |
| `INV-RESIDUE-018` | runtime | Decomposition equation/event input validator before payload update execution | Typed hard error on missing/non-finite/negative required decomposition symbols; zero-valued `oratea/orater` are valid no-decay constants | Tier-A gate | `[DIRECT][Static] + [INFERENCE][Static]` |
| `INV-RESIDUE-019` | runtime | Dynamic residue-depth publisher consumed by snow/freeze frost surface resistance | Typed hard error on stale static-depth reuse after mass changes, invalid litter input, invalid mass-to-depth conversion, or non-conserved plant-to-residue transfer | Frost-residue coupling gate | `[DIRECT][Ran] + [INFERENCE][Static]` |
| `INV-RESIDUE-021` | runtime | Native phenology-to-litter handoff before decomposition | Typed hard error on non-finite/negative/duplicate/delayed transfer; package `HOLD` if the fixed-date bridge carries native litter | CP-GSI02 conservation/consumer gate | `[DIRECT][Static] + [INFERENCE][Static]` |
| `INV-RESIDUE-022` | runtime + integration | External needle/fine-wood source ledger and parallel residue recurrences | Typed hard error on missing/duplicate source projection, downstream re-addition, external/internal mislabeling, parallel-state summation, or stale depth/cover/frost/erosion consumer | CANOPY-LITTER-SOURCE-AUTHORITY-01 conservation gate | `[DIRECT][Static] + [INFERENCE][Static]` |
| `INV-RESIDUE-023` | governance + future integration | Vegetation donor/residue receiver dual-reconstruction and cutover gate | Explicit `HOLD` on missing/duplicate transfer, dry-matter/C/N aliasing, receiver mutation before accepted receipt, duplicate old/new source, or cutover without real-consumer proof | VEGETATION-BOUNDARY-AUTHORITY gate | `[DIRECT][Static] + [INFERENCE][Static]` |

## Symbol Alias Map

Canonical symbols in this contract use Chapter-9 notation. The alias map below
uses legacy WEPP variable tokens from §9.9 where available; openWEPP runtime
field names remain provisional and must preserve these mappings.

| Canonical symbol | Boundary/API name | Scope | Units check | Evidence |
|---|---|---|---|---|
| `Mrt`, `Ms`, `Mf`, `Mb`, `Mr` | `RESAMT`, `RMAG`, `RMOG`, `SMRM`, `RTM` | cropland residue mass pools | `kg m^-2` preserved | `[DIRECT][Static]` |
| `Fpc`, `Hcut`, `Hcm` | `PARTCF`, `CUTHGT`, `HMAX` | harvest partition controls | chapter-declared units preserved | `[DIRECT][Static]` |
| `ORATEA`, `ORATER`, `ENVIND`, `FERIND`, `PSZIND` | `ORATEA`, `ORATER`, `ENVIND`, `FERIND`, `PSZIND` | decomposition-rate controls | chapter-declared units preserved | `[DIRECT][Static]` |
| `PRCP`, `θtill`, `θopt`, `φtill`, `Tavg` | `PRCP`, `SUMWAT`, `OPTWAT`, `AVPOR`, `TMPAVE` | decomposition environmental drivers | chapter-declared units preserved | `[DIRECT][Static]` |
| `WFCs`, `WFCf`, `WFCb`, `TFC` | `WFC` (pool-specific), `TFC` | decomposition factor surfaces | fraction semantics preserved | `[DIRECT][Static]` |
| `Fct`, `Pm` | `FACT`, `POPMAT` | standing-flattening and stubble reference state | chapter-declared units preserved | `[DIRECT][Static]` |
| `P` | `P(t)` stubble-population state (Eq. [9.2.13]) | stubble-population runtime surface | `plants m^-2` preserved | `[DIRECT][Static] + [INFERENCE][Static]` |
| `Crf`, `Crs`, `Crt`, `Crr`, `Crl`, `Cri`, `Cg`, `Ccf` | `FLRCOV`, `STRCOV`, `RESCOV`, `RIGCOV`, `RILCOV`, `INRCOV`, `GCOVER`, `ROKCOV` | residue/ground cover coupling surfaces | fraction semantics preserved | `[DIRECT][Static]` |
| `frr`, `frl`, `Mrr`, `Mrl`, `ΔMw` | `1-WIDTH/RSPACE`, `WIDTH/RSPACE`, `RIGRM`, `RILRM`, `DELTRM` | ridge-furrow transfer surfaces | chapter-declared units preserved | `[DIRECT][Static]` |
| `Ti`, `Rmf`, `Fcut`, `Fbs`, `Fbf`, `Frm` | `MFO`, `RMF`, `FRCUT`, `FBRNAG`, `FBRNOG`, `FRMOVE` | management-event controls | fraction semantics preserved | `[DIRECT][Static]` |
| `Rg`, `Brt`, `Ra`, `Nd`, `Wn` | `RMOG`, `RTMASS`, `RMAG`, `WOOD`, `DECOMP` | rangeland residue/woody/root state | chapter-declared units preserved | `[DIRECT][Static]` |
| `Smi`, `Sr`, `Cn`, `αf`, `αr`, `ωL`, `χ`, `τ`, `ν` | `AMC`, `AMC2`, `CN`, `ACA`, `AR`, `SMRATI`, `RPATIO`, `TAU`, `TAU2` | rangeland decomposition drivers/factors | chapter-declared units preserved | `[DIRECT][Static]` |
| `Cr` | `Cr` (ET coupling symbol) | ET attenuation coupling residue mass (Eq. [5.2.13]) | `kg ha^-1` preserved at ET interface | `[DIRECT][Static]` |

## Allowed Degenerate States

| Degenerate state | Allowed condition | Rationale | Evidence |
|---|---|---|---|
| No effective decomposition day | `ENVIND = 0` due to moisture/temperature constraints. | Chapter equations allow zero decomposition fraction when environmental factors collapse. | `[DIRECT][Static]` |
| Standing residue saturation cap day | `WFCs = 1.0` when `PRCP >= 0.004 m`. | Standing-residue water factor is explicitly capped by rainfall threshold. | `[DIRECT][Static]` |
| Post-harvest no-management day | Daily updates run with no management branch if date does not match management schedule. | Section 9.4 step sequence uses explicit management-date check. | `[DIRECT][Static] + [INFERENCE][Static]` |
| Zero-change management fractions | `Fcut = 0`, `Fbs = 0`, `Fbf = 0`, or `Frm = 0` produce no event conversion/removal. | Event equations are multiplicative/additive and permit zero-effect fractions. | `[DIRECT][Static] + [INFERENCE][Static]` |
| Rangeland high-rainfall antecedent cap state | `Smi`/`Sr` are capped at 100 mm for decomposition weighting. | Chapter text explicitly caps antecedent moisture index to reduce decay-rate inflation. | `[DIRECT][Static]` |

## Invalid States

- Negative or non-finite residue pool masses (`Ms`, `Mf`, `Mb`, `Mr`, `Rg`, `Brt`) beyond tolerance. `[DIRECT][Static] + [INFERENCE][Static]`
- Invalid fraction domains for management parameters (`Fpc`, `Fcut`, `Fbs`, `Fbf`, `Frm`) or cover outputs outside bounded semantics. `[DIRECT][Static] + [INFERENCE][Static]`
- Silent residue mass creation/loss during standing-flat-buried transfers not explainable by declared equations/events. `[DIRECT][Static] + [INFERENCE][Static]`
- Missing residue coupling payloads required for ET attenuation (`Cr`) or erosion/cover adjustment pathways. `[DIRECT][Static] + [INFERENCE][Static]`
- Silent reordering of daily residue update stages relative to the §9.4 sequence. `[DIRECT][Static] + [INFERENCE][Static]`
- Negative decomposition-rate constants (`oratea`, `orater`) in decomposition payload surfaces. `[DIRECT][Static] + [INFERENCE][Static]`
- Static reuse of the initial frost residue depth after the tracked surface-residue mass or senescence/litter input has changed. `[DIRECT][Ran] + [INFERENCE][Static]`
- Negative, non-finite, or silently clipped surface-litter input or mass-to-depth conversion factors. `[INFERENCE][Static]`

## Producer Obligations

- OBL-RESIDUE-P-001: Emit cropland and rangeland residue state surfaces using canonical symbols and declared units. `[DIRECT][Static] + [INFERENCE][Static]`
- OBL-RESIDUE-P-002: Apply Chapter-9 equation branches explicitly for decomposition, cover, and management operations; no implicit fallbacks. `[DIRECT][Static] + [INFERENCE][Static]`
- OBL-RESIDUE-P-003: Propagate invariant failures as typed errors; do not silently clamp materially invalid mass/cover states. `[INFERENCE][Static]`
- OBL-RESIDUE-P-004: Publish ET/erosion-consumable residue payloads (`Cr`, cover fractions, residue placement context) with explicit units and timing semantics. `[DIRECT][Static] + [INFERENCE][Static]`
- OBL-RESIDUE-P-005: Publish frost-consumable dynamic `residue_depth_m` from current surface-residue mass, including same-day senescence/litter input where active, and preserve a residue mass/depth ledger showing deposition, decomposition, and removal terms. `[DIRECT][Ran] + [INFERENCE][Static]`

## Consumer Obligations

- OBL-RESIDUE-C-001: ET consumers must apply residue attenuation using declared residue mass/cover semantics (Eq. [5.2.13]) and reject malformed residue payloads. `[DIRECT][Static] + [INFERENCE][Static]`
- OBL-RESIDUE-C-002: Soil/erosion consumers must preserve residue cover and placement semantics that drive erodibility/shear adjustments and interrill/rill behavior. `[DIRECT][Static] + [INFERENCE][Static]`
- OBL-RESIDUE-C-003: Plant-management consumers must provide deterministic handoff of biomass-to-residue transfers and event controls used by Chapter-9 branches. `[DIRECT][Static] + [INFERENCE][Static]`
- OBL-RESIDUE-C-004: Snow/freeze and hydrology consumers must preserve residue boundary fields needed for coupled thermal/evaporation and cover pathways. `[DIRECT][Static] + [INFERENCE][Static]`
- OBL-RESIDUE-C-005: Snow/freeze consumers must reject missing, non-finite, or stale static residue-depth payloads when dynamic residue mass is available. `[DIRECT][Ran] + [INFERENCE][Static]`

## Boundary Disposition

| Invariant family | Detection point | Disposition | Comparator tier impact | Evidence |
|---|---|---|---|---|
| Harvest/decomposition mass updates (`INV-RESIDUE-001/002/003/004/005`) | residue daily update core | Hard error on invalid equations, factors, or mass domains | Tier-A gate | `[DIRECT][Static] + [INFERENCE][Static]` |
| Cover and ridge-furrow semantics (`INV-RESIDUE-006/007`) | residue cover and transfer stage | Hard error on malformed cover fractions or residue-transfer imbalance | Tier-A/B gate | `[DIRECT][Static] + [INFERENCE][Static]` |
| Management-event semantics (`INV-RESIDUE-008/009`) | management event handler | Hard error on invalid event-factor/date behavior or burial/removal mismatch | Tier-A gate | `[DIRECT][Static] + [INFERENCE][Static]` |
| Rangeland residue decomposition (`INV-RESIDUE-010`) | rangeland residue update stage | Hard error on invalid antecedent-moisture/domain or negative outputs | Tier-A/B gate | `[DIRECT][Static] + [INFERENCE][Static]` |
| Coupling payload and update order (`INV-RESIDUE-011/012`) | residue boundary publish and workflow validator | Hard error on missing payloads or sequencing divergence | Tier-A gate | `[DIRECT][Static] + [INFERENCE][Static]` |
| Dynamic frost-residue boundary (`INV-RESIDUE-019`) | residue-depth publish before frost thermal input construction | Hard error on static seed-depth reuse after dynamic mass changes, invalid conversion, or non-conserved litter input; otherwise frost consumes the dynamic depth as the current residue boundary | Frost residue-cover implementation gate | `[DIRECT][Ran] + [INFERENCE][Static]` |
| Parameter-range governance (`INV-RESIDUE-013`) | review/verification/promotion | Governance `HOLD` until explicit labeling/rationale is recorded | Governance gate | `[DIRECT][Static] + [INFERENCE][Static]` |

## Constants and Parameters Table

| Constant/parameter | Units | Domain | Contract use | Authority |
|---|---|---|---|---|
| `JULIAN_DAY_MIN` | day-of-year | `1` | lower bound for active transition days | REF-RESIDUE-CH9-CROP-SUMMARY |
| `JULIAN_DAY_MAX` | day-of-year | `366` | upper bound for active transition days | REF-RESIDUE-CH9-CROP-SUMMARY |
| `ZERO_SENTINEL` | day/count | `0` | inactive branch sentinel where explicitly authorized | REF-RESIDUE-CH9-CROP-SUMMARY, REF-RESIDUE-CH9-MGMT |
| `FRACTION_MIN` | fraction | `0.0` | lower bound for transition fractions (`Fcut`, `Fbs`, `Fbf`, `Frm`) | REF-RESIDUE-CH9-MGMT |
| `FRACTION_MAX` | fraction | `1.0` | upper bound for transition fractions (`Fcut`, `Fbs`, `Fbf`, `Frm`) | REF-RESIDUE-CH9-MGMT |
| `INDEX_ORIGIN` | index | `1` | first valid index for `cutday[*]` and grazing cycle families | REF-RESIDUE-CH9-CROP-SUMMARY |
| `FOREST_LITTER_FALLBACK_DECAY_RATE` | `d^-1` | `0.5 / 365.25` | First-order forest-litter turnover fallback when recurring seasonal litter input is active and `oratea=0`; not used for inert/no-senescence residue. | REF-RESIDUE-FOREST-LITTER-DECAY |
| `FOREST_LITTER_DROP_WINDOW_DAYS` | day | `45` | Width of the frost-visible fall litter-drop publication window ending on the management fall date for recurring perennial forest litter. | REF-RESIDUE-FROST-COVER-20260629 |

## Tolerance and Numeric Notes

This contract follows `docs/numerics/README.md` (semantic parity, not
bit-for-bit parity). `[DIRECT][Static]`

| Tolerance ID | Definition | Value | Notes | Evidence |
|---|---|---|---|---|
| TOL-RESIDUE-001 | Residue pool non-negativity tolerance (`Ms`, `Mf`, `Mb`, `Mr`, `Rg`, `Brt`) | lower bound `>= -1e-12 kg m^-2` | Hard-fail on material negatives; tolerance absorbs floating-noise only. | `[INFERENCE][Static]` |
| TOL-RESIDUE-002 | Cover fraction bounds tolerance (`Crf`, `Crs`, `Crt`, `Crr`, `Crl`, `Cri`, `Cg`) | `-1e-12 <= value <= 1 + 1e-12` | Applies to fraction surfaces at coupling boundaries. | `[DIRECT][Static] + [INFERENCE][Static]` |
| TOL-RESIDUE-003 | Environmental-factor bounds tolerance (`ENVIND`, `WFC*`, `TFC`) | `-1e-12 <= value <= 1 + 1e-12` | Branch formulas are bounded; out-of-range beyond tolerance is invalid. | `[DIRECT][Static] + [INFERENCE][Static]` |
| TOL-RESIDUE-004 | Exponential update argument finiteness (`Eq. 9.2.4`, Eq. [9.5.2]) | finite real required | Non-finite exponent inputs are invariant failures. | `[INFERENCE][Static]` |
| TOL-RESIDUE-005 | Management fraction domain tolerance (`Fpc`, `Fcut`, `Fbs`, `Fbf`, `Frm`) | `-1e-12 <= value <= 1 + 1e-12` | Parameters are fractions; out-of-domain beyond tolerance is invalid. | `[DIRECT][Static] + [INFERENCE][Static]` |

## Test-Vector Obligations

Minimum required scenario families for contract conformance:

1. Annual burn transition (`resmgt=2`) with same-day trigger and valid
   `fbrnag/fbrnog` fractions.
2. Annual cut transition (`resmgt=4`) with same-day trigger and valid `frcut`.
3. Annual removal transition (`resmgt=5`) with same-day trigger and valid
   `frmove`.
4. Perennial cutting transition (`mgtopt=1`) with contiguous `cutday[1..ncut]`
   and deterministic `cut(event_index)` selection on matching day.
5. Perennial grazing transition (`mgtopt=2`) with contiguous
   `gday/gend/payload[1..ncycle]`, valid `gday<gend`, and deterministic active
   grazing-cycle selection by day window.
6. Invalid-domain rejects for:
   - non-integral/out-of-range `resmgt`, `mgtopt`, `ncut`, `ncycle`;
   - missing indexed symbols within declared cardinality;
   - overflow indexed symbols above declared cardinality;
   - fraction-domain violations and non-positive grazing payloads;
   - invalid grazing window ordering (`gday>=gend`).
7. Failure-posture assertion: all invalid transition-control states surface
   typed hard failures with no silent clamp/default behavior.
8. Growth transition domain rejects for:
   - missing growth state symbols in transition payload assembly;
   - out-of-domain growth state values (`cancov > 0.999`, `hia > 1`,
     negative state surfaces);
   - non-integral runtime day and growth control day symbols.
9. Growth transition reset payload obligations:
   - annual `planting_reset` and `harvest_reset` emit explicit zero-state
     post-transition payloads;
   - perennial `stop_reset` emits explicit zero-state post-transition payload;
   - no implicit defaults are accepted for reset-class state zeroing.
10. PL16 growth-equation obligations:
   - active non-reset annual day (`jdplt < day < jdharv`) emits equation-derived
     `state_after` updates for `sumgdd`, `vdmt`, `cancov`, `lai`, `rtmass`,
     `rtd`, `hia`;
   - active non-reset perennial day (not `jdplt` and not `jdstop`) emits
     equation-derived `state_after` updates with perennial LAI/root-depth
     branch behavior;
   - missing/non-finite/out-of-domain required equation symbols
     (`tmax`, `tmin`, `rad`, `Ws`, projected crop parameters) are typed hard
     failures; no fallback reset/pass-through is accepted.
11. INT10 coupled replay obligations:
   - canonical annual replay preserves scheduler order
     `decomp -> growth -> watbal`;
   - decomposition/growth writeback state emitted in transition phases is
     observable by hydrology phases;
   - missing/non-finite ordering-symbol vectors fail with typed status and
     prevent watbal-lane completion.
12. PL17 decomposition equation/update obligations:
   - active annual non-reset decomposition day emits equation-updated
     `state_after.sumsrm_seed` and `state_after.sumrtm_seed` values;
   - active perennial non-reset decomposition day emits equation-updated
     `state_after.sumsrm_seed` and `state_after.sumrtm_seed` values;
   - covered event-action days (`burn`, `cut`, `remove`, grazing) apply explicit
     transfer/removal updates to tracked pools in addition to daily decay.
13. PL17 decomposition required-symbol guard obligations:
   - missing `pl_decomp_slot_*_oratea` or `..._orater` fails with typed status;
   - zero-valued `oratea`/`orater` are accepted and preserve no-decay behavior;
   - negative `oratea`/`orater` values fail with typed domain status;
   - non-finite `prcp` or out-of-domain `Ws` fails with typed status;
   - active branch does not silently default missing decomposition symbols.
14. Dynamic frost-residue boundary obligations:
   - a seasonal deciduous/perennial fixture shows recurring litter input into
     `sumsrm_seed` during senescence rather than a flat initial seed only;
   - `residue_depth_m` changes consistently with current surface-residue mass
     and remains finite/non-negative;
   - an inert/no-senescence fixture remains identity-stable against its t0
     residue-depth seed;
   - frost thermal inputs consume the dynamic `residue_depth_m` on the real
     downstream path, not a producer-only shadow counter.

## ARCH22 Typed Production-Surface Addendum

### Typed Runtime Surface Authority

1. Covered production kernel interfaces that consume residue-coupled boundary
   payloads must use typed ARCH22 symbol surfaces:
   `HillslopeProductionStateSymbol` and `HillslopeProductionFluxSymbol`.
2. Covered production guard/accessor helper signatures must not accept raw
   `&str` symbol identifiers where typed ARCH22 symbols exist.
3. Typed migration must preserve PL17 and INT10 typed hard-fail posture for
   missing/non-finite/domain-invalid residue-coupled payloads.

### Contract-Derived Migration Vectors

1. Static migration proof: covered residue-coupled production guard accessors
   use typed symbol families, not stringly `&str` symbol parameters.
2. Nominal migration vector: residue-coupled hydrology execution preserves
   deterministic state/flux publication semantics with typed symbols.
3. Failure migration vectors: missing/non-finite/domain-invalid residue-coupled
   symbols still emit typed hard-fail boundary classifications and IDs.

## Authenticated External Forest-litter Boundary Addendum

Let `L` be internally debited CP-GSI02 leaf-off and `N`/`W` authenticated
external needle/fine-woody ground deposition from
`SC-PLANT-001#INV-PLANT-039`. Define `Q=L+N+W`, all in
`kg dry mass m^-2 d^-1`.

For prior surface, interrill-ground, and rill-ground areal states `S`, `I`,
and `R`, same-day other authorized additions `O_*`, common surface decay
factor `f`, and typed management/action operators `A_*`:

```text
S_pre = S + Q + O_s       S_next = A_s(S_pre * f)
I_pre = I + Q + O_i       I_next = A_i(I_pre * f)
R_pre = R + Q + O_r       R_next = A_r(R_pre * f)
```

These are parallel per-unit-area representations. They are not summed as
three global source masses. `Q` appears exactly once inside each recurrence.
`S_next` drives mass-to-depth and frost; `I_next` and `R_next` drive
interrill/rill cover and erosion, with
`G_next=w*I_next+(1-w)*R_next`.

Independent reconstruction must recover each pre-state, decay/action loss,
post-state, weighted ground state, cover, depth, frost input, and erosion
input. Plant-plus-residue open-system closure cancels internal `-L/+L`;
`N+W` remains external influx. Missing or duplicate projection, downstream
re-addition, parallel-state summation, or portrayal of external input as
generated canopy mass hard-fails.

## Gap Register

| Gap ID | Statement | Impact | Promotability | Evidence |
|---|---|---|---|---|
| GAP-RESIDUE-001 | Comparator vectors for per-pool residue transitions and management-event delta checks are not yet curated as reusable fixtures. | Limits automated acceptance breadth for residue-specific invariant replay. | promotable-with-risk | `[DIRECT][Static]` |
| GAP-RESIDUE-002 | Legacy variable-token aliases are mapped, but concrete openWEPP runtime field aliases for residue boundary payloads are not fixed yet. | Symbol-continuity checks across implementation boundaries remain provisional until runtime naming is bound. | non-promotable | `[DIRECT][Static] + [INFERENCE][Static]` |
| GAP-RESIDUE-003 | Cross-contract closure remains provisional until `SC-SED-001`, `SC-HYDRAULICS-001`, and `SC-SUBHYD-001` reach draft completeness for residue-coupled obligations. | Full erosion/hydrology coupling governance cannot yet be treated as closed-loop. | non-promotable | `[DIRECT][Static] + [INFERENCE][Static]` |
| GAP-RESIDUE-004 | Rangeland management-option mechanics are delegated to Chapter 8 and not fully restated as residue-specific executable policy tests in this cycle. | Rangeland branch governance is partially documented but not yet comparator-ready. | promotable-with-risk | `[DIRECT][Static] + [INFERENCE][Static]` |

## Revision History

| Date UTC | Version | Author | Change |
|---|---|---|---|
| `2026-08-08` | `17` | `Codex` | VEGETATION-BOUNDARY-AUTHORITY amendment: established exact-once future vegetation dead-material custody with distinct dry-matter/C/N operands while retaining current litter owners until cutover. |
| `2026-07-20` | `15` | `Codex` | CP-GSI02 review amendment: the first native realization has no fabricated litter and aggregate PL live biomass cannot seed a cold-start leaf-off transfer. |
| `2026-07-28` | `16` | `Codex` | CANOPY-LITTER-SOURCE-AUTHORITY-01 amendment: added `INV-RESIDUE-022` for authenticated external needle/fine-wood influx, exact source-before-decay surface/interrill/rill parallel recurrences, open-system closure, and real depth/frost/cover/erosion consumers. |
| `2026-07-19` | `14` | `Codex` | CP-GSI02 amendment: native GSI leaf-off litter is a same-day exact plant-to-residue transfer before decomposition, cover, depth, and frost; the `jdharv` pending window remains compatibility-only. |
| `2026-07-05` | `13` | `Claude Code` | Codex round-1 corrections: the Cut ground-pool rule fixed to the source-true ADDITION (`decomp.for:689-693`; the rev-12 "Cut does not apply" was source-inaccurate — cut mass joins both ground pools; cut-mass basis labeled as the surface transfer in our standing-mat-free topology); ground-pool seeds + `cf` validated at the decomposition input boundary; the composite `cover_fraction` computed as the `rescov` blend (was still zero-hardcoded despite the package claim). |
| `2026-07-05` | `12` | `Claude Code` | GAP-SED-009 closure amendment: added `INV-RESIDUE-020` — interrill/rill ground-residue pools seeded from the DECLARED IC covers (`init1.for` inverse), evolved by the surface decay law + litter input (+ Burn/Remove/Grazing), covers re-derived daily via the `covcal.for` forward form; `strcov` labeled unmodeled (additive-only). Closes the pathway where erosion covers were hardcoded zero. |
| `2026-05-20` | `0` | `Codex` | Initial canonical stub created by SCI-11 work-package prep. |
| `2026-05-20` | `1` | `Codex` | Full draft authored with Chapter-9 residue authority anchors, invariant/guard map coverage, alias map, obligations, tolerances, and gap register for SCI-11 review cycle. |
| `2026-05-20` | `2` | `Codex` | Post-review amendment pass: added explicit `Cr` variable coverage, replaced mixed-unit rows with explicit unit declarations, and mapped canonical symbols to legacy Chapter-9 variable tokens. |
| `2026-05-20` | `3` | `Codex` | Verification-fix amendment: completed alias-map coverage for `P` stubble-population state and `Wn` rangeland woody biomass symbol. |
| `2026-05-23` | `4` | `Codex` | PL12 amendment: added scheduler decomposition-transition algorithm authority, branch/guard table for PL11 payload consumption, constants table, and PL12 test-vector obligations. |
| `2026-05-23` | `5` | `Codex` | PL13 amendment: added growth-transition branch/reset authority, growth state-domain invariants (`INV-RESIDUE-014/015`), and PL13 growth transition test-vector obligations. |
| `2026-05-23` | `6` | `Codex` | INT10 amendment: added coupled replay lane-order authority (`decomp -> growth -> watbal`), explicit branch/guard and invariant coverage (`INV-RESIDUE-016`), and INT10 ordering/state-transfer test-vector obligations. |
| `2026-05-23` | `7` | `Codex` | PL16 amendment: aligned growth transition authority to reset-only (`planting/harvest/stop`) plus equation-driven non-reset payload behavior, added explicit PL16 growth-equation guard branch, and updated PL16-oriented test-vector obligations/failure posture. |
| `2026-05-23` | `8` | `Codex` | PL17 amendment: added decomposition equation/update addendum with legacy-aligned environmental factors and decay forms, introduced decomposition payload equation-updated seed-pool authority and event-transfer update obligations, added `INV-RESIDUE-017/018` plus guard-map rows, and expanded PL17 test-vector obligations for decomposition kinetics and required-symbol failure posture. |
| `2026-05-23` | `9` | `Codex` | ARCH22 amendment: added typed production-surface authority requiring residue-coupled production interfaces to consume boundary symbols via ARCH22 typed symbol families while preserving PL17/INT10 typed failure semantics under migration. |
| `2026-05-25` | `10` | `Codex` | MOFE11 amendment: added legacy `oratea/orater` domain authority (`infile.for` direct read + `decomp.for` exponential usage), revised PL17 required-symbol domain semantics to allow zero-valued no-decay constants, and updated guard/test vectors to typed-reject negative decomposition-rate constants. |
| `2026-06-29` | `11` | `Codex` | FROST RESIDUE-COVER IMPLEMENTATION amendment: added dynamic frost-residue boundary authority (`INV-RESIDUE-019`), Phase-0 evidence that current `Dec_*` residue mass is flat under zero-rate/no-input fixtures, required senescence/litter input into the surface-residue pool, bound `residue_depth_m` to current surface-residue mass through legacy `res_dp`-lineage conversion, added the narrow `k=0.5 yr^-1` forest-litter turnover fallback plus fall litter-drop publication window ending on the management fall date for recurring seasonal litter with zero `oratea`, recorded the fixed management-date anchor as a known limitation until the physical frost/daylength phenology backlog lands, and required real frost consumers to read the dynamic depth rather than the t0 seed. |
