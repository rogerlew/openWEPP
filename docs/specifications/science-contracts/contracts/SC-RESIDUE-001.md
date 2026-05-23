---
contract_id: SC-RESIDUE-001
title: Residue Management Process Contract
status: in_review
maturity: draft
owner: openWEPP maintainers + hydrology reviewer
contract_version: 4
producer_scope:
  - Cropland residue and root decomposition state/flux surfaces (standing, flat, buried, root)
  - Cropland management-operation residue transitions (tillage, cutting/shredding, burning, removal)
  - Rangeland residue/litter and root decomposition state surfaces
  - Residue cover boundary surfaces consumed by ET, runoff/erosion, and coupled domains
consumer_scope:
  - ET and water-balance consumers using residue mass/cover attenuation signals
  - Soil and erosion consumers using residue placement/cover effects on erodibility and transport
  - Plant-management and snow/freeze consumers requiring residue boundary continuity
evidence_level: Static
last_reviewed: 2026-05-23
supersedes: []
superseded_by: []
---

# SC-RESIDUE-001 Residue Management Process Contract

Status: `in_review`
Maturity: `draft`
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

## Algorithm State Surfaces (PL12 Decomposition/Resup Transition Execution)

### Required Inputs

| Surface | Symbols |
|---|---|
| Active-slot dispatch controls | `day`, `year`, `pl_schedule_slot_*`, `pl_growth_slot_*_imngmt` |
| Decomposition seed state | `iresd_seed`, `sumrtm_seed`, `sumsrm_seed` |
| Annual transition controls | `resmgt`, `jdherb`, `jdburn`, `jdslge`, `jdcut`, `jdmove`, `fbrnag`, `fbrnog`, `frcut`, `frmove` |
| Perennial transition controls | `mgtopt`, `ncut`, `ncycle`, `cutday[*]`, `gday[*]`, `gend[*]`, `animal[*]`, `bodywt[*]`, `area[*]`, `digest[*]` |
| Ordering constraints | `pl_order_decomp_before_soil`, `pl_order_growth_after_decomp` |

### Required Outputs

| Surface | Output |
|---|---|
| Typed decomposition context | management class (`annual/fallow` or `perennial`), active slot/crop identity, runtime day, seed state, transition-control payload, and active day transition selector |
| Scheduler failure surface | typed hard-fail status when required transition-control inputs are missing/non-finite/out-of-domain/non-contiguous |

### Mutated State Surfaces

At scheduler/decomposition-dispatch boundary, mutation authority is limited to
typed transition-context assembly and typed failure reporting; direct residue
mass mutation is delegated to decomposition kernel handlers consuming that
context.

## Algorithm Specification (PL12 Scheduler Decomposition Transition Authority)

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

## Branch and Guard Table (PL12 Decomposition Transition Controls)

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

## Producer Obligations

- OBL-RESIDUE-P-001: Emit cropland and rangeland residue state surfaces using canonical symbols and declared units. `[DIRECT][Static] + [INFERENCE][Static]`
- OBL-RESIDUE-P-002: Apply Chapter-9 equation branches explicitly for decomposition, cover, and management operations; no implicit fallbacks. `[DIRECT][Static] + [INFERENCE][Static]`
- OBL-RESIDUE-P-003: Propagate invariant failures as typed errors; do not silently clamp materially invalid mass/cover states. `[INFERENCE][Static]`
- OBL-RESIDUE-P-004: Publish ET/erosion-consumable residue payloads (`Cr`, cover fractions, residue placement context) with explicit units and timing semantics. `[DIRECT][Static] + [INFERENCE][Static]`

## Consumer Obligations

- OBL-RESIDUE-C-001: ET consumers must apply residue attenuation using declared residue mass/cover semantics (Eq. [5.2.13]) and reject malformed residue payloads. `[DIRECT][Static] + [INFERENCE][Static]`
- OBL-RESIDUE-C-002: Soil/erosion consumers must preserve residue cover and placement semantics that drive erodibility/shear adjustments and interrill/rill behavior. `[DIRECT][Static] + [INFERENCE][Static]`
- OBL-RESIDUE-C-003: Plant-management consumers must provide deterministic handoff of biomass-to-residue transfers and event controls used by Chapter-9 branches. `[DIRECT][Static] + [INFERENCE][Static]`
- OBL-RESIDUE-C-004: Snow/freeze and hydrology consumers must preserve residue boundary fields needed for coupled thermal/evaporation and cover pathways. `[DIRECT][Static] + [INFERENCE][Static]`

## Boundary Disposition

| Invariant family | Detection point | Disposition | Comparator tier impact | Evidence |
|---|---|---|---|---|
| Harvest/decomposition mass updates (`INV-RESIDUE-001/002/003/004/005`) | residue daily update core | Hard error on invalid equations, factors, or mass domains | Tier-A gate | `[DIRECT][Static] + [INFERENCE][Static]` |
| Cover and ridge-furrow semantics (`INV-RESIDUE-006/007`) | residue cover and transfer stage | Hard error on malformed cover fractions or residue-transfer imbalance | Tier-A/B gate | `[DIRECT][Static] + [INFERENCE][Static]` |
| Management-event semantics (`INV-RESIDUE-008/009`) | management event handler | Hard error on invalid event-factor/date behavior or burial/removal mismatch | Tier-A gate | `[DIRECT][Static] + [INFERENCE][Static]` |
| Rangeland residue decomposition (`INV-RESIDUE-010`) | rangeland residue update stage | Hard error on invalid antecedent-moisture/domain or negative outputs | Tier-A/B gate | `[DIRECT][Static] + [INFERENCE][Static]` |
| Coupling payload and update order (`INV-RESIDUE-011/012`) | residue boundary publish and workflow validator | Hard error on missing payloads or sequencing divergence | Tier-A gate | `[DIRECT][Static] + [INFERENCE][Static]` |
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
| `2026-05-20` | `0` | `Codex` | Initial canonical stub created by SCI-11 work-package prep. |
| `2026-05-20` | `1` | `Codex` | Full draft authored with Chapter-9 residue authority anchors, invariant/guard map coverage, alias map, obligations, tolerances, and gap register for SCI-11 review cycle. |
| `2026-05-20` | `2` | `Codex` | Post-review amendment pass: added explicit `Cr` variable coverage, replaced mixed-unit rows with explicit unit declarations, and mapped canonical symbols to legacy Chapter-9 variable tokens. |
| `2026-05-20` | `3` | `Codex` | Verification-fix amendment: completed alias-map coverage for `P` stubble-population state and `Wn` rangeland woody biomass symbol. |
| `2026-05-23` | `4` | `Codex` | PL12 amendment: added scheduler decomposition-transition algorithm authority, branch/guard table for PL11 payload consumption, constants table, and PL12 test-vector obligations. |
