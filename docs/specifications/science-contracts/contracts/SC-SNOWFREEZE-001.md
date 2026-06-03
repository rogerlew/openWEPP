---
contract_id: SC-SNOWFREEZE-001
title: Snow and Freeze Process Contract
status: in_review
maturity: draft
owner: openWEPP maintainers + hydrology reviewer
contract_version: 15
producer_scope:
  - Winter precipitation phase partition surfaces (rain vs snow)
  - Snowpack depth/density/water-equivalent state surfaces
  - Melt and freeze-thaw transition surfaces
consumer_scope:
  - Daily water-balance accounting consumers
  - Infiltration/runoff partition consumers affected by frozen-soil state
  - Soil/erosion coupling consumers requiring freeze-thaw context
evidence_level: static
last_reviewed: 2026-06-03
supersedes: []
superseded_by: []
---

# SC-SNOWFREEZE-001 Snow and Freeze Process Contract

Status: `in_review`
Maturity: `draft`
Evidence mode: `static`

## Purpose

Define top-down scientific authority for snow accumulation/melt and freeze-thaw
process behavior, including downstream coupling boundaries in openWEPP.

## Scientific Scope

In scope:
- Snowpack accumulation, density, and melt boundary behavior. `[DIRECT][Static]`
- Frozen-soil and thaw transition boundary behavior relevant to hydrology. `[DIRECT][Static] + [INFERENCE][Static]`
- Required producer/consumer boundary semantics for winter-process handoff. `[INFERENCE][Static]`
- Hourly winter-process forcing transformations derived from daily climate input. `[DIRECT][Static]`

Out of scope:
- Kernel implementation details. `[INFERENCE][Static]`
- Non-snow/freeze domains except required coupling boundaries. `[INFERENCE][Static]`
- Standalone activation of snow drifting process equations not active in the
  August 1995 WEPP release lineage. `[DIRECT][Static]`

## Authority Anchors

| Anchor ID | Source anchor | Contract use | Evidence |
|---|---|---|---|
| REF-SNOWFREEZE-CH3-INTRO | `references/50201000/chap3.pdf` §3.1 | Winter routine scope, activation conditions, and declared outputs/processes (hourly snow accumulation/melt/frost-thaw). | `[DIRECT][Static]` |
| REF-SNOWFREEZE-CH3-HRPRECIP | `chap3.pdf` §3.2 | Hourly precipitation derivation and disaggregation/start-time semantics for winter routine inputs. | `[DIRECT][Static]` |
| REF-SNOWFREEZE-CH3-HRTEMP | `chap3.pdf` §3.3, Eq. [3.3.1]-[3.3.3] | Hourly air/surface temperature derivation semantics used by melt/frost routines. | `[DIRECT][Static]` |
| REF-SNOWFREEZE-CH3-HRRAD | `chap3.pdf` §3.5, Eq. [3.5.1]-[3.5.7] | Hourly radiation derivation used by snowmelt energy terms. | `[DIRECT][Static]` |
| REF-SNOWFREEZE-CH3-MELT | `chap3.pdf` §3.6, Eq. [3.6.1]-[3.6.6] | Melt equation structure and component terms (`amelt`, `bmelt`, `cmelt`, `dmelt`). | `[DIRECT][Static]` |
| REF-SNOWFREEZE-CH3-MELT-ASSUMP | `chap3.pdf` §3.6 assumptions list | Melt gating assumptions (`Tmax` thresholds, density threshold, bounded melt, albedo assumption). | `[DIRECT][Static]` |
| REF-SNOWFREEZE-CH3-SNOWDENS | `chap3.pdf` §3.7, Eq. [3.7.1]-[3.7.5] | Snow depth/density update rules under snowfall, settling, melt, and mixed melt+snowfall. | `[DIRECT][Static]` |
| REF-SNOWFREEZE-CH3-SNOWDENS-LIM | `chap3.pdf` §3.7 terminal paragraph | Explicit upper density limit (`522 kg m^-3`). | `[DIRECT][Static]` |
| REF-SNOWFREEZE-CH3-FROST | `chap3.pdf` §3.8, Eq. [3.8.1]-[3.8.4] | Frost/thaw heat-flow relations, layered thermal conductivity, and hourly bookkeeping outputs. | `[DIRECT][Static]` |
| REF-SNOWFREEZE-CH3-DRIFT-INACTIVE | `chap3.pdf` §3.9 intro paragraph | Snow drifting equations are described but not currently active in the August 1995 WEPP release. | `[DIRECT][Static]` |
| REF-SNOWFREEZE-CH4-COUPLING | `references/50201000/chap4.pdf` §4.1 | Infiltration/runoff components consume rainfall-excess timing/intensity and infiltrated water surfaces. | `[DIRECT][Static]` |
| REF-SNOWFREEZE-CH5-COUPLING | `references/50201000/chap5.pdf` §5.1, Eq. [5.1.1] | Daily water balance includes snow-water term and treats melted snow as rainfall for runoff/percolation estimation. | `[DIRECT][Static]` |
| REF-SNOWFREEZE-LEGACY-WINTER-NEGMLT | `/workdir/wepp-forest_260430_baseline/src/winter.for` lines 420-464 and `/workdir/wepp-forest_260430_baseline/src/melt.for` lines 275-301 | Pinned baseline signed-hourly-melt source and bug-compatible comparator context; its daily negative-melt redistribution branch is superseded for target implementation by `REF-SNOWFREEZE-WEPPFOREST-WINTER-NEGMLT-FIX`. | `[DIRECT][Static]` |
| REF-SNOWFREEZE-WEPPFOREST-WINTER-NEGMLT-FIX | `/workdir/wepp-forest/src/winter.for` lines 441-460 at commit `03fee4558456535138592630b5dedc4d81ce8d06` (`winter: apply ngtvML/pstvML math fix and close P4 parity lane`) | Corrected daily negative-melt authority: compare net daily melt with `pstvML + ngtvML <= 0.0`; when net melt is positive, reduce positive hourly melt by scaling with `1 + ngtvML/pstvML`. This supersedes the pinned baseline sign/branch bug and is the openWEPP target behavior. | `[DIRECT][Static]` |
| REF-SNOWFREEZE-LEGACY-SNOWD-RAINSTORE | `/workdir/wepp-forest_260430_baseline/src/snowd.for` lines 240-279 | Pinned baseline rain-on-snow holding-capacity branch consumes hourly rain into snowpack density until `350 kg m^-3`, leaving only residual rain as liquid runoff/infiltration forcing. | `[DIRECT][Static]` |
| REF-SNOWFREEZE-PHYS-BOUNDS | Physical/common-sense invariant class | Non-negative snow depth/water and bounded densities are required for physical validity. | `[INFERENCE][Static]` |

## Variables and Units

| Symbol | Units | Meaning | Producer | Consumer |
|---|---|---|---|---|
| `Dsold` | `m` | Snow depth before hourly update. | winter snow routine | snow density/melt update logic |
| `Dsnew` | `m` | Snow depth after hourly update. | winter snow routine | melt routing, frost conductivity terms |
| `Dsavail` | `m` | Available pre-hour snow depth state used by Eq. [3.6.1] melt upper-bound branch (legacy text refers to preceding-hour snow depth state). | winter snow routine | melt bound branch semantics |
| `ρsold` | `kg m^-3` | Snow density before update. | winter snow routine | density transition logic |
| `ρsnew` | `kg m^-3` | Snow density after update. | winter snow routine | melt gating and density cap checks |
| `hrsnow` | `m` | Hourly snowfall depth increment. | hourly precip partition | snowpack accumulation update |
| `faldr` | `m` | Falling drift contribution used by legacy drift equations. | legacy drift formulation (inactive in target lineage) | governance/provenance only |
| `grdri` | `m` | Ground drift contribution used by legacy drift equations. | legacy drift formulation (inactive in target lineage) | governance/provenance only |
| `hrmelt` | `m` | Hourly melt water from snowpack. | melt routine | DISAG/infiltration-runoff coupling |
| `hrrain` | `m` | Hourly rainfall amount. | hourly precip partition | melt term and runoff/infiltration forcing |
| `hrrain_store` | `m` | Hourly rain retained in sub-`350 kg m^-3` snowpack holding capacity. | snow density update | daily `S`, runtime SWE, liquid-forcing reduction |
| `hrmelt_raw` | `m` | Signed hourly energy-balance melt before daily negative-melt redistribution. | melt routine | daily redistribution and diagnostics |
| `Thr` | `degC` | Hourly air temperature. | hourly temperature routine | melt/frost branch logic |
| `Thra` | `degC` | Hourly adjusted surface temperature. | surface energy balance routine | frost routine driver |
| `Tmax` | `degC` | Daily maximum air temperature. | climate forcing | rain/snow partition and melt gating |
| `Tmin` | `degC` | Daily minimum air temperature. | climate forcing | rain/snow partition and hourly temperature |
| `hrrad` | `MJ m^-2` | Hourly radiation on sloping surface. | SUNMAP routine | melt radiation term |
| `cancov` | `fraction` | Canopy cover fraction (`0..1`). | plant/canopy state | melt attenuation term |
| `clouds` | `fraction` | Cloud-cover fraction (`0..1`). | hourly radiation/cloud routine | melt and surface-temperature terms |
| `Qsrf` | `W m^-2` | Heat flux through snow-residue-frozen-soil layered system. | frost routine | freeze/thaw depth update bookkeeping |
| `Quf` | `W m^-2` | Heat flow from unfrozen soil below freezing front. | frost routine | freeze/thaw depth update bookkeeping |
| `Ksrf` | `W m^-1 degC^-1` | Harmonic-mean layered thermal conductivity for snow-residue-frozen system. | frost routine | heat-flux computation |
| `Snowd` | `m` | Snow layer depth in layered frost conductivity equation. | snow routine | frost layered conductivity equation |
| `Resd` | `m` | Residue thickness in layered conductivity equation. | residue/management surfaces | frost layered conductivity equation |
| `Tilld` | `m` | Frozen tilled-layer depth in conductivity equation. | soil/frost routine | frost layered conductivity equation |
| `Utilld` | `m` | Frozen untilled-layer depth in conductivity equation. | soil/frost routine | frost layered conductivity equation |
| `Dfrost` | `m` | Frost depth output from hourly frost bookkeeping. | frost routine | soil-state and winter coupling consumers |
| `Dthaw` | `m` | Thaw depth output from hourly frost bookkeeping. | frost routine | soil-state and winter coupling consumers |
| `S` | `m` | Daily snow-water storage term in Eq. [5.1.1] (`+` melt, `-` accumulation). | winter routine | daily water-balance closure |
| `Ws_frz` | `m` | Water accumulated in frozen soil (hourly bookkeeping output). | frost routine | infiltration-capacity adjustment / reporting |
| `Nft` | `count` | Number of freeze-thaw cycles over winter bookkeeping. | frost routine | soil-state/infiltration-capacity coupling |
| `InfCap_frz` | `m s^-1` | Infiltration capacity of tilled layer/top `0.20 m` (untilled case) under frost routine output; non-SI internal units must be converted at boundary publish. | frost routine | infiltration/runoff component |

## Invariants

| Invariant ID | Statement | Severity | Authority | Evidence |
|---|---|---|---|---|
| INV-SNOWFREEZE-001 | Melt bound and non-negativity branch semantics: post-branch exported melt satisfies `0 <= hrmelt <= Dsavail`, where `Dsavail` is the pre-hour available snow-depth state used by Eq. [3.6.1] branch logic. | hard-fail | REF-SNOWFREEZE-CH3-MELT, REF-SNOWFREEZE-CH3-MELT-ASSUMP, REF-SNOWFREEZE-PHYS-BOUNDS | `[DIRECT][Static] + [INFERENCE][Static]` |
| INV-SNOWFREEZE-002 | Snow-density melt gate: liquid melt export to infiltration/runoff is not allowed until post-update snow density reaches `ρsnew >= 350 kg m^-3`; below this threshold melt remains in-pack and increases density. | hard-fail | REF-SNOWFREEZE-CH3-MELT, REF-SNOWFREEZE-CH3-SNOWDENS | `[DIRECT][Static]` |
| INV-SNOWFREEZE-003 | Snow depth-density domain bounds: `Dsold >= 0`, `Dsnew >= 0`, `ρsold >= 0`, `ρsnew >= 0`, and `ρsnew <= 522 kg m^-3`; when `Dsnew = 0`, `ρsnew = 0`. | hard-fail | REF-SNOWFREEZE-CH3-SNOWDENS, REF-SNOWFREEZE-CH3-SNOWDENS-LIM, REF-SNOWFREEZE-PHYS-BOUNDS | `[DIRECT][Static] + [INFERENCE][Static]` |
| INV-SNOWFREEZE-004 | Active snow-update branch consistency: fresh snowfall contribution uses `100 kg m^-3` density and active depth/density updates follow Eq. [3.7.1]-[3.7.5] for settling, snowfall, melt, and melt+snowfall cases; drift-term equations remain governance-only while drift is inactive. | hard-fail | REF-SNOWFREEZE-CH3-SNOWDENS, REF-SNOWFREEZE-CH3-DRIFT-INACTIVE | `[DIRECT][Static] + [INFERENCE][Static]` |
| INV-SNOWFREEZE-005 | Rain/snow partition consistency: precipitation phase partition follows daily temperature logic (`Tmax < 0` => all snow; `Tmin > 0` => all rain; mixed day uses hourly occurrence/diurnal temperature function). | hard-fail | REF-SNOWFREEZE-CH3-INTRO, REF-SNOWFREEZE-CH3-HRPRECIP, REF-SNOWFREEZE-CH5-COUPLING | `[DIRECT][Static]` |
| INV-SNOWFREEZE-006 | Frost heat-flow formulation consistency: frost/thaw bookkeeping uses explicit layered heat-flow relations (`Qsrf`, `Quf`) and harmonic-mean layered thermal conductivity per Eq. [3.8.1]-[3.8.4]. | hard-fail | REF-SNOWFREEZE-CH3-FROST | `[DIRECT][Static]` |
| INV-SNOWFREEZE-007 | Winter coupling payload completeness: hourly winter outputs required for downstream consumers are emitted with valid units/domains, including `hrmelt`, `Dfrost`, `Dthaw`, `Nft`, `Ws_frz`, and `InfCap_frz`, and daily snow-water term `S` is consistently reflected in water balance semantics. | hard-fail | REF-SNOWFREEZE-CH3-INTRO, REF-SNOWFREEZE-CH3-MELT, REF-SNOWFREEZE-CH3-FROST, REF-SNOWFREEZE-CH4-COUPLING, REF-SNOWFREEZE-CH5-COUPLING | `[DIRECT][Static] + [INFERENCE][Static]` |
| INV-SNOWFREEZE-008 | Snow drifting governance invariant: process claims requiring active drift transport equations are non-promotable until authority confirms an active drift implementation path for the target lineage. | governance-fail | REF-SNOWFREEZE-CH3-DRIFT-INACTIVE | `[DIRECT][Static] + [INFERENCE][Static]` |
| INV-SNOWFREEZE-009 | Winter-routine activation branch is explicit: winter hourly processing is invoked when at least one trigger condition is true (existing snowpack, existing soil frost layer, or average daily temperature below `0 degC`), with no silent bypass. Activation depends on runtime state/forcing triggers, not snow-sidecar presence alone; parsed default snow controls are valid controls when the missing-file branch has explicitly set `defaults_applied=true`. | hard-fail | REF-SNOWFREEZE-CH3-INTRO | `[DIRECT][Static] + [INFERENCE][Static]` |
| INV-SNOWFREEZE-010 | CLIM05 parsed snow-control coupling invariant: when parsed `snow.options.*` controls are projected to runtime, coupling must enforce finite/valid control domains (`newsnw > 0`, `ssd > 0`, `newsnw <= ssd`), publish signed `S = melt - accumulation`, and maintain non-negative `snow.runtime_swe` without silent fallback/defaulting. | hard-fail | REF-SNOWFREEZE-CH3-INTRO, REF-SNOWFREEZE-CH3-MELT, REF-SNOWFREEZE-CH5-COUPLING, REF-SNOWFREEZE-PHYS-BOUNDS | `[DIRECT][Static] + [INFERENCE][Static]` |
| INV-SNOWFREEZE-011 | SIMIMPL18 day-key partition/publication closure: for active snow coupling and precipitation days where `Tmax <= rst`, liquid runoff-coupling input from direct rainfall/melt is zero for that day key (`RM = 0`), snow storage update remains explicit (`snow.runtime_swe(new) = snow.runtime_swe(old) + accumulation - melt`), and downstream published `Snow-Water`/hydout-equivalent snow storage values derive from runtime SWE state rather than static sidecar control `snow.options.ssd`. | hard-fail | REF-SNOWFREEZE-CH3-INTRO, REF-SNOWFREEZE-CH3-HRPRECIP, REF-SNOWFREEZE-CH5-COUPLING, REF-SNOWFREEZE-PHYS-BOUNDS | `[DIRECT][Static] + [INFERENCE][Static]` |
| INV-SNOWFREEZE-012 | Frost routine-chain dispatch and handoff closure: active winter-hourly frost triggers dispatch `winter -> frostN`, `frostN` performs water-state handoff with `frwatc(1)` at hourly entry and `frwatc(0)` at day-end/thaw-complete exit, and freeze-active branches execute `frzng -> frznw` lineage without silent bypass. | hard-fail | REF-SNOWFREEZE-CH3-INTRO, REF-SNOWFREEZE-CH3-FROST | `[DIRECT][Static] + [INFERENCE][Static]` |
| INV-SNOWFREEZE-013 | Frozen-soil conductivity authority closure: when frost is present, saturated-conductivity coupling follows `frsoil` fine-layer aggregation with `getFreezeCond` land-use-dependent `kfactor` selection and remains explicitly bounded/typed at the runtime seam (`frost.runtime_infcap_frz`). | hard-fail | REF-SNOWFREEZE-CH3-FROST, REF-SNOWFREEZE-CH4-COUPLING, REF-SNOWFREEZE-PHYS-BOUNDS | `[DIRECT][Static] + [INFERENCE][Static]` |
| INV-SNOWFREEZE-014 | HPHYS0268 spring snowpack lineage closure: material H1/H7/H39 seasonal `Ep` divergence claims must expose baseline-authoritative `winter -> snowd -> melt` lineage for runtime SWE/depth/density/settle carry state, hourly rain/snow/melt sums, signed `S`, WB13 `RM`, and WB13 `Snow-Water` before returning residual ownership to WB17 `Ep`. Active snowpack execution is governed by runtime snow/frost/thermal triggers and parsed/default snow controls; `snow.options.snow_file_present` may only select parsed-vs-default control provenance and must not gate whether snow processing runs. Non-agricultural HPHYS parity keeps frost disabled while snow remains active. | governance-hold | INV-SNOWFREEZE-009, INV-SNOWFREEZE-010, INV-SNOWFREEZE-011, REF-SNOWFREEZE-CH3-INTRO, REF-SNOWFREEZE-CH3-MELT, REF-SNOWFREEZE-CH5-COUPLING | `[DIRECT][Static] + [INFERENCE][Static]` |
| INV-SNOWFREEZE-015 | HPHYS0269 baseline winter melt/snowpack invariant: openWEPP snowpack migration must preserve corrected `wepp-forest` daily negative-melt redistribution authority while preserving pinned baseline `snowd.for`/`melt.for` signed hourly melt and rain-on-snow holding-capacity lineage. `melt.for` may emit negative hourly `hrmelt_raw`; only positive raw melt is bounded to available snow during the hourly melt branch. Corrected `winter.for` daily post-processing compares net daily melt (`pstvML + ngtvML`) and, when positive, scales positive hourly melt by `1 + ngtvML/pstvML` before daily routed melt is summed; the pinned baseline `pstvML <= ngtvML` and `1 - ngtvML/pstvML` branch is rejected as bug-compatible comparator behavior, not target physics. `snowd.for` rain-on-snow storage consumes hourly rain into snowpack density while `ρsnew < 350 kg m^-3`; retained rain increases runtime SWE and contributes negative daily `S` just like snowfall accumulation. Liquid runoff forcing may consume only residual direct rain plus redistributed melt. | hard-fail | REF-SNOWFREEZE-WEPPFOREST-WINTER-NEGMLT-FIX, REF-SNOWFREEZE-LEGACY-WINTER-NEGMLT, REF-SNOWFREEZE-LEGACY-SNOWD-RAINSTORE, REF-SNOWFREEZE-CH3-MELT, REF-SNOWFREEZE-CH3-SNOWDENS, REF-SNOWFREEZE-CH5-COUPLING | `[DIRECT][Static] + [INFERENCE][Static]` |
| INV-SNOWFREEZE-016 | HPHYS0270 daily snowpack carry-state invariant: H1/H7/H39 spring snowpack residual claims must expose same-day pre-update and post-update runtime SWE, snow depth, snow density, and settle-day-count state, plus their daily deltas, before assigning residual ownership to WB17 `Ep`, aggregate storage, WB13 publication, or a new snowpack production defect. Final-hour state alone is insufficient for closure because `winter -> snowd -> melt` mutates carry state across the whole day and WB13 `RM`/`Snow-Water` publication consumes the day-begin SWE lineage. | governance-hold | INV-SNOWFREEZE-014, INV-SNOWFREEZE-015, REF-SNOWFREEZE-CH3-SNOWDENS, REF-SNOWFREEZE-CH5-COUPLING, REF-SNOWFREEZE-WEPPFOREST-WINTER-NEGMLT-FIX | `[DIRECT][Static] + [INFERENCE][Static]` |
| INV-SNOWFREEZE-017 | HPHYS0271 day-36 melt-forcing lineage invariant: residual claims for the H1 sim-day 36 spurious early-February melt event must expose `melt.for` term-level hourly evidence (`amelt`, `bmelt`, `cmelt`, `dmelt`, signed `wmelt`) and the hourly forcing/branch inputs that produce them, including air temperature, dewpoint/temperature-for-rain term, radiation, cloud fraction, wind, rain, snowfall, canopy cover, wind adjustment, warm-branch activation, and pre/post snowpack state. Evidence that treats day-36 as broad accumulation, WB17 `Ep`, WB13 publication, aggregate storage, or negative-melt redistribution is invalid unless term-level melt evidence proves that ownership. | governance-hold | INV-SNOWFREEZE-015, INV-SNOWFREEZE-016, REF-SNOWFREEZE-CH3-MELT, REF-SNOWFREEZE-LEGACY-WINTER-NEGMLT, REF-SNOWFREEZE-WEPPFOREST-WINTER-NEGMLT-FIX | `[DIRECT][Static] + [INFERENCE][Static]` |

## Invariant Guard Map

| Invariant ID | Guard class | Enforcement path | Failure behavior | Gate impact | Evidence |
|---|---|---|---|---|---|
| `INV-SNOWFREEZE-001` | runtime | Melt branch validator and exporter (`hrmelt` bounded to `[0, Dsavail]`) | Explicit branch applies authoritative bounds; typed hard error if post-branch export remains out-of-domain | Tier-A/B gate | `[DIRECT][Static] + [INFERENCE][Static]` |
| `INV-SNOWFREEZE-002` | runtime | Melt-density gate before routing `hrmelt` to DISAG/infiltration | Typed hard error if liquid melt is exported while `ρsnew < 350 kg m^-3` | Tier-A gate | `[DIRECT][Static]` |
| `INV-SNOWFREEZE-003` | runtime | Snow state domain validator after each hourly update | Typed hard error on negative depths/densities, violated zero-depth/zero-density rule, or density cap breach | Tier-A/B gate | `[DIRECT][Static] + [INFERENCE][Static]` |
| `INV-SNOWFREEZE-004` | runtime | Branch-specific snow-density/depth equation checks for active snowfall/settling/melt/mixed branches | Typed hard error on inconsistent active-branch closure | Tier-A/B gate | `[DIRECT][Static] + [INFERENCE][Static]` |
| `INV-SNOWFREEZE-005` | runtime | Daily/hourly precipitation phase-partition branch validator | Typed hard error on partition logic mismatch | Tier-A/B gate | `[DIRECT][Static]` |
| `INV-SNOWFREEZE-006` | runtime | Frost routine heat-flow equation and layered conductivity checks | Typed hard error on invalid heat-flow domain or layered conductivity setup | Tier-B investigation gate | `[DIRECT][Static]` |
| `INV-SNOWFREEZE-007` | runtime | Winter payload boundary validator (hourly + daily coupling fields) | Typed hard error on missing/invalid required payload fields | Tier-A/B gate | `[DIRECT][Static] + [INFERENCE][Static]` |
| `INV-SNOWFREEZE-008` | governance | Review/disposition/verification promotion check | Promotion `HOLD` when drift-active claims appear without authority-backed implementation status update | Governance gate | `[DIRECT][Static] + [INFERENCE][Static]` |
| `INV-SNOWFREEZE-009` | runtime | Winter-routine trigger-condition branch gate | Typed hard error on silent skip when trigger condition is true | Tier-A/B gate | `[DIRECT][Static] + [INFERENCE][Static]` |
| `INV-SNOWFREEZE-010` | runtime | CLIM05 snow-control adapter + hydrology snow-coupling branch | Typed hard error on missing/non-finite/domain-invalid `snow.options.*` controls or invalid `S`/`snow.runtime_swe` closure | Tier-A gate | `[DIRECT][Static] + [INFERENCE][Static]` |
| `INV-SNOWFREEZE-011` | runtime | CLIM05 partition branch + WB13/hydout publication mapper | Typed hard error on cold-day liquid-partition mismatch (`RM > 0` when `Tmax <= rst`) or publication of `Snow-Water` from static `snow.options.ssd` instead of runtime SWE state | Tier-A gate | `[DIRECT][Static] + [INFERENCE][Static]` |
| `INV-SNOWFREEZE-012` | runtime | Winter/frost dispatch gate and routine-chain sequencing validator (`winter`, `frostN`, `frwatc`, `frzng`, `frznw`) | Typed hard error on active-branch dispatch mismatch, missing handoff call sequencing, or silent bypass | Tier-A/B gate | `[DIRECT][Static] + [INFERENCE][Static]` |
| `INV-SNOWFREEZE-013` | runtime | Frozen-soil conductivity coupling validator (`frsoil` + `getFreezeCond`) | Typed hard error on missing land-use coefficient selection, invalid frozen-zone conductivity aggregation, or non-finite seam export | Tier-A/B gate | `[DIRECT][Static] + [INFERENCE][Static]` |
| `INV-SNOWFREEZE-014` | governance | H1/H7/H39 material spring snowpack classifier spanning runtime snow carry state, hourly rain/snow/melt totals, signed `S`, WB13 `RM`, and WB13 `Snow-Water` | Explicit `HOLD` when spring `Ep` residual ownership is asserted without snowpack lineage evidence; no WB17 `Ep` compensation or sidecar-presence gate substitution | HPHYS0268 spring snowpack gate | `[DIRECT][Static] + [INFERENCE][Static]` |
| `INV-SNOWFREEZE-015` | runtime + governance | Winter melt/snowpack kernel and HPHYS0269 classifier spanning signed `hrmelt_raw`, redistributed hourly melt, rain retained in snowpack, runtime SWE/depth/density, signed `S`, residual liquid rain, WB13 `RM`, and WB13 `Snow-Water` | Typed hard error on missing/non-finite/domain-invalid retained-rain or snowpack state; explicit `HOLD` when full snowpack migration evidence is incomplete; no empirical melt or `Ep` compensation edits | HPHYS0269 winter melt/snowpack baselining gate | `[DIRECT][Static] + [INFERENCE][Static]` |
| `INV-SNOWFREEZE-016` | governance | Daily carry-state classifier spanning pre-day/post-day SWE, depth, density, settle count, deltas, signed `S`, WB13 `RM`, and WB13 `Snow-Water` | Explicit `HOLD` when residual ownership is asserted without daily carry-state evidence or when final-hour state is substituted for day-begin publication state | HPHYS0270 daily snowpack state gate | `[DIRECT][Static] + [INFERENCE][Static]` |
| `INV-SNOWFREEZE-017` | governance | H1 day-36 melt-term/hourly-forcing classifier spanning `amelt/bmelt/cmelt/dmelt`, raw/redistributed melt, forcing inputs, and warm-branch flags | Explicit `HOLD` when day-36 residual ownership is asserted without melt-term and hourly-forcing evidence; no WB17/storage/WB13/negative-melt compensation edits | HPHYS0271 day-36 melt-forcing gate | `[DIRECT][Static] + [INFERENCE][Static]` |

## Symbol Alias Map

Canonical symbols in this contract follow WEPP Chapter-3 notation and lineage
names by default. SIMIMPL27 ratifies concrete openWEPP boundary/API aliases for
winter snow/freeze migration scope. Existing typed aliases use
`HillslopeProductionStateSymbol` / `HillslopeProductionFluxSymbol` mappings;
hourly-internal aliases that are not yet produced on runtime writeback are
reserved under explicit `snow.hourly.*` / `winter.hourly.*` / `frost.hourly.*`
namespaces for staged SIMIMPL28/SIMIMPL29/SIMIMPL32 implementation.

| Canonical symbol | Boundary/API name | Scope | Units check | Evidence |
|---|---|---|---|---|
| `Dsold` | `snow.hourly.depth_before_m` | hourly snow-depth state surface | `m` -> `m` | `[DIRECT][Static] + [INFERENCE][Static]` |
| `Dsnew` | `snow.hourly.depth_after_m` | hourly snow-depth state surface | `m` -> `m` | `[DIRECT][Static] + [INFERENCE][Static]` |
| `Dsavail` | `snow.hourly.depth_available_m` | melt upper-bound branch pre-hour state | `m` -> `m` | `[DIRECT][Static] + [INFERENCE][Static]` |
| `ρsold` | `snow.hourly.density_before_kg_m3` | hourly snow-density state surface | `kg m^-3` -> `kg m^-3` | `[DIRECT][Static] + [INFERENCE][Static]` |
| `ρsnew` | `snow.hourly.density_after_kg_m3` | hourly snow-density state surface | `kg m^-3` -> `kg m^-3` | `[DIRECT][Static] + [INFERENCE][Static]` |
| `hrsnow` | `snow.hourly.snowfall_m` | hourly snowfall input | `m` -> `m` | `[DIRECT][Static] + [INFERENCE][Static]` |
| `faldr`, `grdri` | `snow.drift.faldr_m`, `snow.drift.grdri_m` (inactive/governance-only) | drift formulation provenance only while drift is inactive | `m` -> `m` | `[DIRECT][Static]` |
| `hrmelt` | `snow.hourly.melt_m` | hourly redistributed melt forcing and trace family after daily winter post-processing | `m` -> `m`; final routed values are finite/non-negative and daily routed melt is the summed coupling quantity | `[DIRECT][Static] + [INFERENCE][Static]` |
| `hrmelt_raw` | `snow.hourly.melt_raw_m` | signed hourly melt before daily redistribution | `m` -> `m`; negative values are valid diagnostics before daily post-processing | `[DIRECT][Static] + [INFERENCE][Static]` |
| `hrrain` | `snow.hourly.rain_m` | hourly rainfall forcing before snowpack holding-capacity retention | `m` -> `m` | `[DIRECT][Static] + [INFERENCE][Static]` |
| `hrrain_store` | `snow.hourly.rain_retained_m` | hourly rain retained in snowpack holding capacity | `m` -> `m`, finite and non-negative | `[DIRECT][Static] + [INFERENCE][Static]` |
| `Thr` | `winter.hourly.air_temp_c` | hourly thermal forcing surface | `degC` -> `degC` | `[DIRECT][Static] + [INFERENCE][Static]` |
| `Thra` | `winter.hourly.surface_temp_c` | hourly adjusted thermal forcing surface | `degC` -> `degC` | `[DIRECT][Static] + [INFERENCE][Static]` |
| `Tmax`, `Tmin` | `tmax`, `tmin` (`HillslopeProductionStateSymbol::{Wb14Tmax,Wb14Tmin}`) | daily thermal forcing surface | `degC` -> `degC` | `[DIRECT][Static] + [INFERENCE][Static]` |
| `hrrad` | `winter.hourly.rad_mj_m2` | hourly radiation surface | `MJ m^-2` -> `MJ m^-2` | `[DIRECT][Static] + [INFERENCE][Static]` |
| `cancov`, `clouds` | `cancov` (`HillslopeProductionStateSymbol::Wb15PlantCancov`), `winter.hourly.cloud_fraction` | melt and surface-temperature modifiers | `fraction` -> `fraction` | `[DIRECT][Static] + [INFERENCE][Static]` |
| `Qsrf`, `Quf`, `Ksrf` | `frost.hourly.qsrf_w_m2`, `frost.hourly.quf_w_m2`, `frost.hourly.ksrf_w_m_k` | frost heat-flow bookkeeping surface | `W m^-2` / `W m^-1 degC^-1` unchanged | `[DIRECT][Static] + [INFERENCE][Static]` |
| `Snowd`, `Resd`, `Tilld`, `Utilld` | `frost.hourly.snow_depth_m`, `frost.hourly.residue_depth_m`, `frost.hourly.tilled_frozen_depth_m`, `frost.hourly.untilled_frozen_depth_m` | layered conductivity state inputs | `m` -> `m` | `[DIRECT][Static] + [INFERENCE][Static]` |
| `Dfrost`, `Dthaw` | `frost.runtime_dfrost`, `frost.runtime_dthaw` (`HillslopeProductionStateSymbol::{Wb14FrostRuntimeDfrost,Wb14FrostRuntimeDthaw}`) | hourly frost/thaw depth boundary outputs | `m` -> `m` | `[DIRECT][Static] + [INFERENCE][Static]` |
| `S` | `S` (`HillslopeProductionFluxSymbol::Wb12SnowCouplingS`) | daily snow-water term in water-balance closure | `m` -> `m` | `[DIRECT][Static] + [INFERENCE][Static]` |
| `Snow-Water` (WB13/hydout publication surface) | derived alias from `snow.runtime_swe` at publication boundary | replay/output storage-state publication | runtime SWE (`m`) is converted to published snow-water storage units at boundary without sidecar-control substitution | `[DIRECT][Static] + [INFERENCE][Static]` |
| `snow.options.rst`, `snow.options.newsnw`, `snow.options.ssd`, `snow.options.snow_file_present` | identity (`HillslopeProductionStateSymbol::{Wb14SnowRst,Wb14SnowNewsnw,Wb14SnowSsd,Wb14SnowFilePresent}`) | parsed snow sidecar controls projected to runtime seam | scalar controls preserved; `snow_file_present` in `{0,1}` | `[DIRECT][Static] + [INFERENCE][Static]` |
| `snow.runtime_swe` | identity (`HillslopeProductionStateSymbol::Wb14SnowRuntimeSwe`) | runtime snow-water-equivalent storage state | `m` -> `m` | `[DIRECT][Static] + [INFERENCE][Static]` |
| HPHYS0268/HPHYS0269 snowpack diagnostics | `snow_runtime_swe_m`, `snow_runtime_depth_m`, `snow_runtime_density_kg_m3`, `snow_runtime_settle_day_count`, `snow_s_m`, `snow_hourly_rain_sum_m`, `snow_hourly_rain_retained_sum_m`, `snow_hourly_snowfall_water_equiv_sum_m`, `snow_hourly_melt_raw_sum_m`, `snow_hourly_melt_sum_m`, `snow_runtime_swe_closure_error_m`, `wb13_rm_mm`, `wb13_snow_water_mm` | Opt-in run-trace evidence for classifying H1/H7/H39 spring snowpack/SWE/`RM` lineage and winter melt/rain-retention migration before assigning material `Ep` residual ownership | runtime state `m`/`kg m^-3`/count, daily coupling `m`, WB13 publication `mm` | `[DIRECT][Static] + [INFERENCE][Static]` |
| HPHYS0270 daily snowpack state diagnostics | `snow_runtime_swe_before_m`, `snow_runtime_depth_before_m`, `snow_runtime_density_before_kg_m3`, `snow_runtime_settle_day_count_before`, `snow_runtime_swe_m`, `snow_runtime_depth_m`, `snow_runtime_density_kg_m3`, `snow_runtime_settle_day_count`, `snow_runtime_swe_delta_m`, `snow_runtime_depth_delta_m`, `snow_runtime_density_delta_kg_m3`, `snow_runtime_settle_day_count_delta` | Opt-in run-trace evidence for classifying daily snowpack carry-state residuals before assigning H1/H7/H39 spring divergence ownership to WB17 `Ep`, storage, WB13 publication, or another snowpack production seam | runtime state `m`/`kg m^-3`/count and daily deltas in same units | `[DIRECT][Static] + [INFERENCE][Static]` |
| HPHYS0271 melt-forcing diagnostics | `snow.hourly.melt_raw_m_####`, `snow.hourly.melt_m_####`, `snow.hourly.melt_amelt_in_####`, `snow.hourly.melt_bmelt_in_####`, `snow.hourly.melt_cmelt_in_####`, `snow.hourly.melt_dmelt_in_####`, `snow.hourly.melt_hrtef_f_####`, `snow.hourly.melt_hrdtf_f_####`, `snow.hourly.melt_vwmph_####`, `snow.hourly.melt_rainin_####`, `snow.hourly.melt_wind_adjustment_####`, `snow.hourly.melt_branch_active_####`, `winter.hourly.dewpoint_c_####`, `winter.hourly.wind_m_s_####` | Opt-in run-trace evidence for classifying H1 day-36 spurious melt against `melt.for` term-level lineage and hourly forcing before changing production physics | melt depths `m`; melt terms in inch-equivalent pre-`0.0254` conversion, temperatures `degF`/`degC`, wind `mph`/`m s^-1`, rain `in`, flags `0/1` | `[DIRECT][Static] + [INFERENCE][Static]` |
| `Ws_frz`, `Nft` | `frost.runtime_ws_frz`, `frost.runtime_nft` (`HillslopeProductionStateSymbol::{Wb14FrostRuntimeWsFrz,Wb14FrostRuntimeNft}`) | frozen-soil coupling boundary outputs | units preserved as declared | `[DIRECT][Static] + [INFERENCE][Static]` |
| `InfCap_frz` | `frost.runtime_infcap_frz` (`HillslopeProductionStateSymbol::Wb14FrostRuntimeInfcapFrz`) | frozen-soil infiltration-capacity boundary output | `m s^-1` required at exported boundary | `[DIRECT][Static] + [INFERENCE][Static]` |

## Allowed Degenerate States

| Degenerate state | Allowed condition | Rationale |
|---|---|---|
| Snow-free hour/day | `Dsnew = 0`, `ρsnew = 0`, `hrmelt = 0` | No snowpack is a valid boundary condition. `[DIRECT][Static] + [INFERENCE][Static]` |
| Cold non-melt day | `Tmax < -3 degC` with snowpack present and `hrmelt = 0` | Explicit melt assumption in Section 3.6. `[DIRECT][Static]` |
| Density-buildup pre-melt state | `ρsnew < 350 kg m^-3` and computed melt retained in pack (no liquid export) | Explicit density gate before liquid melt reaches soil. `[DIRECT][Static]` |
| All-snow precipitation day | `Tmax < 0 degC`, all precipitation routed as snowfall | Explicit partition rule. `[DIRECT][Static]` |
| All-rain precipitation day | `Tmin > 0 degC`, all precipitation routed as rainfall | Explicit partition rule. `[DIRECT][Static]` |

## Invalid States

- Negative snow depth or snow density (`Ds* < 0`, `ρs* < 0`) beyond tolerance. `[DIRECT][Static] + [INFERENCE][Static]`
- Melt water export where `ρsnew < 350 kg m^-3`. `[DIRECT][Static]`
- Post-branch positive `hrmelt_raw > Dsavail`; negative `hrmelt_raw` is valid only as signed `melt.for`/corrected daily redistribution lineage and must not be silently clamped before the `winter.for` daily post-processing branch. `[DIRECT][Static]`
- Negative retained rain (`hrrain_store < 0`) or residual direct rain below zero after rain-on-snow holding-capacity accounting. `[DIRECT][Static]`
- `ρsnew > 522 kg m^-3`. `[DIRECT][Static]`
- `Dsnew = 0` while `ρsnew > 0`. `[DIRECT][Static] + [INFERENCE][Static]`
- Missing required winter boundary payload fields for runoff/infiltration/water-balance coupling. `[DIRECT][Static] + [INFERENCE][Static]`
- Active CLIM05 coupling with missing/non-finite/out-of-domain `snow.options.*` controls or negative `snow.runtime_swe`. `[DIRECT][Static] + [INFERENCE][Static]`
- Published `Snow-Water` or hydout-equivalent snow storage value sourced from static sidecar control `snow.options.ssd` instead of runtime `snow.runtime_swe`. `[DIRECT][Static] + [INFERENCE][Static]`
- Drift-active process claims in promotion evidence without updated active-lineage authority. `[DIRECT][Static] + [INFERENCE][Static]`
- Active frost branch execution that omits required routine-chain handoff semantics (`frwatc(1)` at active-hour ingress and `frwatc(0)` at day-end/thaw-complete egress). `[DIRECT][Static] + [INFERENCE][Static]`
- Frost-active conductivity coupling that bypasses `frsoil`/`getFreezeCond` authority mapping or exports non-finite `frost.runtime_infcap_frz`. `[DIRECT][Static] + [INFERENCE][Static]`

## Producer Obligations

- OBL-SNOWFREEZE-P-001: Emit hourly winter state/update surfaces (`Ds*`, `ρs*`, `hrmelt`, `Dfrost`, `Dthaw`, `Nft`, `Ws_frz`, `InfCap_frz`) and daily `S` with declared units. `[DIRECT][Static] + [INFERENCE][Static]`
- OBL-SNOWFREEZE-P-002: Enforce all `INV-SNOWFREEZE-*` runtime guards before publishing boundary payloads. `[INFERENCE][Static]`
- OBL-SNOWFREEZE-P-003: Route meltwater to downstream runoff/infiltration only when density-gate conditions are satisfied. `[DIRECT][Static]`
- OBL-SNOWFREEZE-P-004: Surface typed errors for violated melt bounds, snow-state domains, and frost-branch inconsistencies; no silent fallback values. `[DIRECT][Static] + [INFERENCE][Static]`
- OBL-SNOWFREEZE-P-005: Keep drift-activation assumptions explicit; do not imply active drift transport without authority-backed contract amendment. `[DIRECT][Static] + [INFERENCE][Static]`
- OBL-SNOWFREEZE-P-006: When parsed snow controls are projected, publish `S` and `snow.runtime_swe` as explicit coupled boundary/state outputs and hard-fail on active-coupling symbol/domain violations. `[DIRECT][Static] + [INFERENCE][Static]`
- OBL-SNOWFREEZE-P-007: Publish day-key `Snow-Water` storage outputs from runtime SWE state mapping (not static sidecar controls) and preserve cold-day partition closure (`Tmax <= rst` implies no direct liquid `RM` contribution). `[DIRECT][Static] + [INFERENCE][Static]`
- OBL-SNOWFREEZE-P-008: Preserve baseline-authoritative frost routine-chain dispatch semantics in active branches (`winter -> frostN -> {frzng/mlt*}` with required `frwatc` ingress/egress handoff) and hard-fail on sequencing violations. `[DIRECT][Static] + [INFERENCE][Static]`
- OBL-SNOWFREEZE-P-009: Apply frozen-soil conductivity authority via `frsoil` + `getFreezeCond` lineage when frost is present and publish bounded, finite `frost.runtime_infcap_frz` seam outputs. `[DIRECT][Static] + [INFERENCE][Static]`
- OBL-SNOWFREEZE-P-010: Preserve signed hourly `melt.for` output through the corrected daily `winter.for` negative-melt redistribution branch before publishing daily melt/SWE closure; do not clamp negative hourly melt at the equation boundary and do not reproduce the pinned baseline sign/branch bug. `[DIRECT][Static]`
- OBL-SNOWFREEZE-P-011: Apply `snowd.for` rain-on-snow holding-capacity accounting before runoff/infiltration liquid forcing: retained rain increases snowpack density/SWE, residual rain remains liquid, and daily `S` includes retained rain as snowpack storage gain. `[DIRECT][Static]`

## Consumer Obligations

- OBL-SNOWFREEZE-C-001: Infiltration/runoff consumers treat `hrmelt` as event forcing with the same rigor as rainfall forcing where coupling specifies breakpoint-like handling. `[DIRECT][Static] + [INFERENCE][Static]`
- OBL-SNOWFREEZE-C-002: Daily water-balance consumer treats melted snow as rainfall contribution in Eq. [5.1.1] accounting semantics. `[DIRECT][Static]`
- OBL-SNOWFREEZE-C-003: Soil/erosion-related consumers receiving frost outputs (`Dfrost`, `Dthaw`, `Nft`, `Ws_frz`, `InfCap_frz`) must fail explicitly on missing or invalid winter payloads. `[DIRECT][Static] + [INFERENCE][Static]`
- OBL-SNOWFREEZE-C-004: Consumers propagate invariant-violation context as typed errors without silent clamping/defaulting. `[INFERENCE][Static]`
- OBL-SNOWFREEZE-C-005: Runoff and storage reconciliation consumers must apply signed `S` coupling semantics and reject active-coupling payloads missing required `snow.options.*` controls. `[DIRECT][Static] + [INFERENCE][Static]`
- OBL-SNOWFREEZE-C-006: Output/publication consumers must reject static-control substitution where snow-storage publication aliases (`Snow-Water`, hydout-equivalent snow-water surfaces) fail runtime SWE derivation checks. `[DIRECT][Static] + [INFERENCE][Static]`

## Boundary Disposition

| Invariant family | Detection point | Disposition | Comparator tier impact | Evidence |
|---|---|---|---|---|
| Melt bounds and density gate (`INV-SNOWFREEZE-001/002`) | melt post-processing and pre-routing checks | Explicit branch applies melt bounds; hard error only if post-branch state remains invalid or if density gate is violated | Tier-A gate | `[DIRECT][Static] + [INFERENCE][Static]` |
| Snow depth/density domain (`INV-SNOWFREEZE-003/004`) | hourly snowpack state update | Hard error on domain/branch inconsistency | Tier-A/B gate | `[DIRECT][Static] + [INFERENCE][Static]` |
| Partition and activation branches (`INV-SNOWFREEZE-005/009`) | daily/hourly branch selection | Hard error on branch mismatch or silent bypass | Tier-A/B gate | `[DIRECT][Static] + [INFERENCE][Static]` |
| Frost heat-flow semantics (`INV-SNOWFREEZE-006`) | frost routine bookkeeping | Hard error on invalid equation domain; investigate hourly-heavy deltas per ADR confidence tiers | Tier-B investigation gate | `[DIRECT][Static]` |
| Coupling completeness (`INV-SNOWFREEZE-007`) | winter payload boundary handoff | Hard error on missing/invalid field or unit mismatch | Tier-A/B gate | `[DIRECT][Static] + [INFERENCE][Static]` |
| CLIM05 parsed snow-control coupling (`INV-SNOWFREEZE-010`) | runtime snow-control adaptation and hydrology coupling branch | Hard error on missing/non-finite/out-of-domain controls or invalid `S`/`snow.runtime_swe` closure | Tier-A gate | `[DIRECT][Static] + [INFERENCE][Static]` |
| Frost routine-chain dispatch/handoff (`INV-SNOWFREEZE-012`) | active winter-frost branch execution path | Hard error on routine-chain dispatch mismatch or missing `frwatc` handoff sequencing | Tier-A/B gate | `[DIRECT][Static] + [INFERENCE][Static]` |
| Frozen-soil conductivity closure (`INV-SNOWFREEZE-013`) | frost-active conductivity/infiltration-capacity coupling path | Hard error on non-authoritative conductivity path or non-finite/invalid `frost.runtime_infcap_frz` export | Tier-A/B gate | `[DIRECT][Static] + [INFERENCE][Static]` |
| Spring snowpack lineage closure (`INV-SNOWFREEZE-014`) | HPHYS0268 targeted/full-suite evidence gate | Governance `HOLD` until material spring `Ep` divergence evidence includes baseline snowpack/SWE/`RM` lineage and rules in a production defect before production physics edits | HPHYS0268 gate | `[DIRECT][Static] + [INFERENCE][Static]` |
| Winter melt/rain-retention closure (`INV-SNOWFREEZE-015`) | HPHYS0269 targeted/full-suite evidence gate and active snowpack kernel | Hard error for invalid retained-rain/signed-melt state; governance `HOLD` until baseline `winter.for`/`snowd.for`/`melt.for` migration evidence is complete | HPHYS0269 gate | `[DIRECT][Static] + [INFERENCE][Static]` |
| Daily snowpack carry-state closure (`INV-SNOWFREEZE-016`) | HPHYS0270 targeted/full-suite evidence gate | Governance `HOLD` until pre-day/post-day SWE, depth, density, and settle-count lineage is explicit enough to localize H1/H7/H39 spring residual ownership | HPHYS0270 gate | `[DIRECT][Static] + [INFERENCE][Static]` |
| Day-36 melt-forcing closure (`INV-SNOWFREEZE-017`) | HPHYS0271 targeted/full-suite evidence gate | Governance `HOLD` until H1 day-36 `melt.for` term-level and hourly-forcing lineage is explicit enough to localize or correct the spurious melt event | HPHYS0271 gate | `[DIRECT][Static] + [INFERENCE][Static]` |
| Drift activation governance (`INV-SNOWFREEZE-008`) | review/disposition/promotion gate | Governance `HOLD` until active-implementation authority is explicit | Governance gate | `[DIRECT][Static] + [INFERENCE][Static]` |

## Tolerance and Numeric Notes

This contract follows `docs/numerics/README.md` (semantic parity, not bitwise
parity). Contract-specific interpretation tolerances:

| Tolerance ID | Definition | Value | Notes |
|---|---|---|---|
| TOL-SNOWFREEZE-001 | Melt bound residual tolerance for post-branch `0 <= hrmelt <= Dsavail` | `1e-12 m` | Comparator interpretation only; runtime branch/hard-fail behavior remains explicit for material violation. |
| TOL-SNOWFREEZE-002 | Snow depth/density non-negative comparator tolerance | lower bound `>= -1e-12` | Runtime hard-fail still required for values materially below zero. |
| TOL-SNOWFREEZE-003 | Density threshold gate proximity tolerance around `350 kg m^-3` | `1e-9 kg m^-3` | Prevents floating-point noise from flipping density-gate branch semantics. |
| TOL-SNOWFREEZE-004 | Zero-depth/zero-density closure tolerance | If `Dsnew <= 1e-12 m`, require `ρsnew <= 1e-9 kg m^-3` | Prevents false closure from product-only checks that can mask invalid nonzero density. |
| TOL-SNOWFREEZE-005 | Frost heat-flow equation residual tolerance for iterative closure diagnostics | `<= 1e-8` in routine-native flux units | Diagnostic/comparator aid; not a silent runtime correction mechanism. |

## CLIM05 Parsed Snow-Control Runtime Coupling Addendum

## HPHYS0247 Winter Activation Trigger Addendum

1. Winter hourly forcing must trigger from runtime state and climate forcing:
   - existing `snow.runtime_swe > 0`,
   - active frost runtime state,
   - or daily average temperature below `0 degC`.
2. Active snow coupling must trigger from snow/cold runtime state, not sidecar
   presence: existing `snow.runtime_swe > 0` or daily average temperature below
   `0 degC` with projected snow controls/defaults present. Frost-active
   processing can invoke winter/frost hourly forcing without forcing the
   snow-coupling branch when there is no existing snowpack and no projected
   snow-control payload.
3. `snow.options.snow_file_present` records sidecar discoverability/override
   provenance only. It must not suppress winter hourly processing when a
   runtime/thermal trigger is true and parsed snow controls are present from
   the explicit missing-file defaults branch.
4. Contract-derived vectors must prove a cold precipitation day without a snow
   sidecar emits hourly snowfall inputs, mutates `snow.runtime_swe`, and
   publishes `RM` from liquid rain plus melt rather than raw precipitation
   passthrough.

### CLIM05 Required Surfaces

| Surface | Symbols |
|---|---|
| Parsed sidecar controls | `snow.options.rst`, `snow.options.newsnw`, `snow.options.ssd`, `snow.options.snow_file_present` |
| Climate drivers | `Tmax`, `Tmin`, hyetograph precipitation depth |
| Coupled runtime state/output | `snow.runtime_swe`, `S` |

### CLIM05 Deterministic Rules

1. Active coupling is explicit when `snow.runtime_swe > 0` or when the day is
   thermally snow-active and projected snow controls/defaults are present on
   the runtime state surface. `snow.options.snow_file_present` is validated as
   binary sidecar discoverability/override provenance only and must not
   activate coupling by itself.
2. Partition logic uses `rst` threshold:
   - `Tmax <= rst`: all precipitation accumulates as snow;
   - `Tmin >= rst`: precipitation is rain pathway;
   - mixed-day branch: deterministic partition driven by `Tmin/Tmax/rst`.
3. Signed snow-water term is `S = melt - accumulation`; snow storage update is
   `snow.runtime_swe(new) = snow.runtime_swe(old) + accumulation - melt` with
   non-negative closure.
4. Parsed control domains must remain finite and physically valid:
   `newsnw > 0`, `ssd > 0`, and `newsnw <= ssd`.
5. Active-coupling missing/non-finite/domain-invalid controls or invalid
   `S`/`snow.runtime_swe` closure are hard-fail states.
6. Publication mapping closure is required: downstream day-key `Snow-Water`
   aliases must derive from runtime SWE state and must not publish static
   `snow.options.ssd` control values as dynamic storage state.

### CLIM05 Contract-Test Vectors

1. Active coupling nominal vector produces deterministic `S`, updated
   `snow.runtime_swe`, and valid hydrology-coupled closure.
2. Missing active-coupling required parsed control symbol hard-fails with typed
   missing-input posture.
3. Non-finite or out-of-domain active-coupling control/state hard-fails with
   typed non-finite/domain posture and no fallback.

## CLIM06 Frozen-Soil Runtime Coupling Addendum

### CLIM06 Required Surfaces

| Surface | Symbols |
|---|---|
| Parsed frost sidecar controls | `frost.options.wintRed`, `frost.options.fineTop`, `frost.options.fineBot`, `frost.options.ksnowf`, `frost.options.kresf`, `frost.options.ksoilf`, `frost.options.kfactor1`, `frost.options.kfactor2`, `frost.options.kfactor3`, `frost.options.frost_file_present` |
| Climate freeze/thaw drivers | `Tmax`, `Tmin`, hyetograph precipitation depth |
| Coupled runtime state/output | `frost.runtime_dfrost`, `frost.runtime_dthaw`, `frost.runtime_nft`, `frost.runtime_ws_frz`, `frost.runtime_infcap_frz` |

### CLIM06 Deterministic Rules

1. Active CLIM06 coupling is explicit when
   `frost.options.frost_file_present = 1` and `frost.options.wintRed = 1`.
2. Freeze/thaw branch selection is climate-driven and explicit:
   - `Tmin <= 0 degC`: freeze-active branch updates `Dfrost` and
     infiltration-capacity reduction state;
   - `Tmin > 0 degC`: thaw/inactive branch reduces frozen-depth influence.
3. CLIM06 infiltration-capacity coupling uses a bounded frozen-soil reduction
   envelope:
   - `kfactor_floor = min(kfactor1, kfactor2, kfactor3)`
   - `freeze_fraction = clamp(Dfrost / 0.20, 0, 1)`
   - `InfCap_frz = Ke * (1 - freeze_fraction + freeze_fraction * kfactor_floor)`
4. Active-coupling missing/non-finite/out-of-domain controls or derived frost
   runtime surfaces are hard-fail states; no silent fallback/default branch is
   allowed.

### CLIM06 Contract-Test Vectors

1. Active CLIM06 vector emits deterministic
   `Dfrost`/`Dthaw`/`Nft`/`Ws_frz`/`InfCap_frz` and reduced infiltration
   coupling with valid runoff closure.
2. Missing active-coupling frost control symbol hard-fails with typed
   missing-input posture.
3. Non-finite active-coupling frost control symbol hard-fails with typed
   non-finite posture.
4. Out-of-domain active-coupling frost control/state hard-fails with typed
   domain posture and no fallback.

## SIMIMPL27 Boundary/API Closure and Contract-Test Requirements Addendum

### Boundary/API Closure Scope

SIMIMPL27 closes the authority-side boundary/API ambiguity for hourly snow and
freeze migration scope by ratifying concrete alias names in this contract.

Cross-contract consumer ownership for the ratified winter payload is explicit:

| Boundary payload family | Producer authority | Consumer authority |
|---|---|---|
| `S`, `snow.runtime_swe`, day-key `Snow-Water` publication lineage | `SC-SNOWFREEZE-001`, `SC-WATBAL-001` | `SC-RUNOFFPART-001`, `SC-SYSTEM-001` |
| `frost.runtime_dfrost`, `frost.runtime_dthaw`, `frost.runtime_nft`, `frost.runtime_ws_frz`, `frost.runtime_infcap_frz` | `SC-SNOWFREEZE-001` | `SC-SOIL-001`, `SC-WATBAL-001`, `SC-SYSTEM-001` |
| Hourly reserved aliases under `snow.hourly.*`, `winter.hourly.*`, `frost.hourly.*` | `SC-SNOWFREEZE-001` | SIMIMPL28/SIMIMPL29/SIMIMPL32 runtime implementation and downstream coupling tests |

### Contract-Derived Test Requirements (Downstream)

SIMIMPL28/SIMIMPL29/SIMIMPL32 must implement contract-derived tests covering:

1. Alias projection closure:
   existing typed runtime symbols map exactly to the contract alias names for
   `snow.options.*`, `snow.runtime_swe`, `frost.runtime_*`, `tmax`, `tmin`, and
   flux `S`.
2. SIMIMPL28 reserved hourly forcing payload completeness:
   when hourly winter forcing synthesis is active, required climate-driven
   families (`winter.hourly.rad_mj_m2_####`, `winter.hourly.air_temp_c_####`,
   `winter.hourly.cloud_fraction_####`, `snow.hourly.rain_m_####`,
   `snow.hourly.snowfall_m_####`) must be present with valid units/domains
   before boundary publication.
3. SIMIMPL29 reserved hourly snow-kernel payload completeness:
   migration-complete hourly snow families (`snow.hourly.depth_*`,
   `snow.hourly.density_*`, `snow.hourly.depth_available_m`,
   `snow.hourly.melt_m`) must be present with valid units/domains before
   boundary publication.
4. `Dsavail` timing closure:
   melt upper-bound branch uses pre-hour available snow-depth state
   (`snow.hourly.depth_available_m`) with no off-by-one timing drift.
5. SIMIMPL32 frost-hourly payload and routine-chain completeness:
   required `frost.hourly.*` families plus active routine-chain dispatch/
   handoff semantics (`winter -> frostN`, `frwatc` ingress/egress, `frzng`/
   `frznw` freeze lineage, `frsoil`/`getFreezeCond` conductivity lineage) are
   present and validated with typed failures.
6. Typed failure posture:
   missing/non-finite/out-of-domain winter payload symbols in active hourly
   branches hard-fail with typed error paths and no silent fallback.

### SIMIMPL28 Forcing-Emission Scope Clarification

SIMIMPL28 closure scope is restricted to baseline-authoritative hourly forcing
families generated from climate + slope + active snow controls (`sunmap`,
`radcur`, `hr_tmp`, `stmtim` lineage). Frost heat-flow hourly families remain
SIMIMPL32 closure scope, while hourly snow depth/density/melt state families
remain SIMIMPL29 closure scope. Promotion claims for deferred families must
remain explicit `HOLD` ownership until corresponding evidence exists.

### SIMIMPL29 Snow Kernel Port and Hourly State Closure

SIMIMPL29 ports baseline-authoritative snow update and melt branch structure
(`snowd` + `melt` lineage) into openWEPP hydrology runtime coupling for active
snow control execution. Required closure surface for this wave:

1. Hourly snow kernel state publication:
   - `snow.hourly.depth_before_m_####`
   - `snow.hourly.depth_available_m_####`
   - `snow.hourly.density_before_kg_m3_####`
   - `snow.hourly.depth_after_m_####`
   - `snow.hourly.density_after_kg_m3_####`
   - `snow.hourly.melt_m_####`
2. Runtime carry-state publication for day-to-day continuity:
   - `snow.runtime_swe`
   - `snow.runtime_depth_m`
   - `snow.runtime_density_kg_m3`
   - `snow.runtime_settle_day_count`
3. Active branch inputs are required and typed:
   - `snow.hourly.{rain,snowfall}_####`
   - `winter.hourly.{rad_mj_m2,air_temp_c,cloud_fraction}_####`
   Missing or non-finite active-branch symbols are hard-fail conditions.
4. Snow coupling closure remains `S = melt - accumulation` with accumulation
   derived from hourly snowfall water-equivalent partition and non-negative
   runtime snow-state closure.

SIMIMPL29 does not claim full baseline frost energy-balance migration closure.
`frost.hourly.*` family closure remains explicit follow-on ownership.

### HPHYS0268 Spring Snowpack Re-Anchor Addendum

HPHYS0268 narrows the H1/H7/H39 continuation from generic seasonal `Ep`
residuals to the first material spring-snowmelt divergences (`|Ep diff| >
1 mm`). Closure evidence must compare the runtime snow carry state and WB13
publication lineage to pinned baseline `winter.for`, `snowd.for`, and
`melt.for` behavior before changing `Ep`/WB17 production code.

Required HPHYS0268 evidence:

1. Material-divergence anchor:
   H1/H7/H39 reports must identify first `|Ep diff| > 1 mm` dates and same-day
   WAT context for `Ep`, `RM`, `Snow-Water`, storage, runoff, lateral flow, and
   percolation.
2. Runtime snowpack lineage:
   diagnostics must expose `snow.runtime_swe`, `snow.runtime_depth_m`,
   `snow.runtime_density_kg_m3`, `snow.runtime_settle_day_count`, hourly
   rainfall, hourly snowfall water equivalent, hourly melt, and signed `S`.
3. Publication lineage:
   WB13 `Snow-Water` must derive from runtime SWE and WB13 `RM` must reconcile
   to precipitation plus snowpack change plus irrigation under contract units.
4. Trigger semantics:
   `snow.options.snow_file_present` is provenance for parsed-vs-default
   controls only. Runtime snowpack processing remains active when snowpack,
   frost, snowfall/rainfall partition, or cold thermal triggers require winter
   processing. HPHYS non-agricultural parity disables frost through legacy
   `ksflag`/land-use semantics, not snow.
5. Disposition rule:
   if the snowpack trace is incomplete or reconciles internally while semantic
   parity still diverges, disposition remains `HOLD` with narrowed ownership.
   Production changes require a baseline-authoritative defect proof and must not
   compensate through WB17 `Ep` or WB13 publication edits.

### HPHYS0269 Winter Melt/Snowpack Baselining Addendum

HPHYS0269 implements the HPHYS0268 continuation at the architecture seam rather
than tuning residuals. The required baseline source scope is
`/workdir/wepp-forest_260430_baseline/src/winter.for`, `snowd.for`, and
`melt.for` at commit `dac3c950d8b16cc73774bf5ce2e7e11f80baac70`.

Required HPHYS0269 migration authority:

1. `melt.for` computes signed hourly melt at lines 275-301. Negative values are
   not clamped at the equation boundary; positive values are bounded by the
   current snow-depth/density water equivalent.
2. `winter.for` lines 420-464 perform daily signed-melt post-processing after
   the hourly loop and before daily `wmelt` publication. Runtime closure
   evidence must distinguish raw signed melt from redistributed daily melt.
3. `snowd.for` lines 240-279 retain rain in snowpack while density remains
   below `350 kg m^-3`; retained rain raises density/SWE and reduces direct
   liquid forcing.
4. Daily signed `S` must equal redistributed melt minus snowfall water
   equivalent minus retained rain, and WB13 `RM` must therefore represent
   residual direct rain plus redistributed melt, not raw precipitation
   passthrough.
5. If any part of the source scope remains unmigrated, package disposition must
   remain `HOLD` with the unmigrated line range and state transition recorded.

## SIMIMPL31 Frost Routine-Chain Authority and Contract-Test Requirements Addendum

### Baseline Routine-Chain Authority Map

SIMIMPL31 closes migration-scope authority ambiguity for the baseline frost
routine chain by ratifying routine-to-alias/invariant ownership in canonical
contract text.

| Baseline routine (source) | Call lineage authority | Canonical responsibility | Contract boundary aliases |
|---|---|---|---|
| `winter` (`winter.for`) | Active hourly trigger dispatches `frostN(hour)` when frost trigger conditions are present; frost path is explicitly skipped only when winter coupling is disabled. | Winter-hourly orchestration entry point for frost processing within daily winter loop. | `winter.hourly.*`, `snow.hourly.*`, downstream `frost.runtime_*` payload handoff |
| `frostN` (`frostn.for`) | Main hourly frost driver; performs ingress `frwatc(1)` handoff at active-hour initialization, branch-specific freeze/thaw process dispatch, and egress `frwatc(0)` handoff at hour-24 or thaw-complete closure. | Freeze/thaw branch routing, heat-flow bookkeeping, and daily handoff closure. | `frost.hourly.*`, `frost.runtime_dfrost`, `frost.runtime_dthaw`, `frost.runtime_nft`, `frost.runtime_ws_frz`, `frost.runtime_infcap_frz` |
| `frzng` (`frzng.for`) | Freeze-active branch extension path called by `frostN`; invokes `frznw` when infiltrated/frozen-zone liquid-water freezing is required. | Energy-limited frost-front extension and latent-heat bookkeeping. | `frost.hourly.qsrf_w_m2`, `frost.hourly.quf_w_m2`, `frost.runtime_dfrost`, `frost.runtime_ws_frz`, `frost.runtime_nft` |
| `frznw` (`frznw.for`) | Layer-local freezing helper called by `frzng`. | Fine-layer freezing time/energy closure and frozen/liquid partition updates. | `frost.runtime_ws_frz`, `frost.runtime_dfrost` (through parent update lineage) |
| `frwatc` (`frwatc.for`) | Bidirectional handoff routine: `wbtofs=1` maps water-balance state to frost fine-layer state; `wbtofs=0` maps frost-updated fine-layer state back to coarse soil/water-balance state. | Water-state exchange seam between frost routines and water-balance/soil consumers. | `frost.runtime_ws_frz`, soil-water lineage surfaces consumed by `SC-SOIL-001` / `SC-WATBAL-001` |
| `frsoil` (`frsoil.for`) + `getFreezeCond` (`getfreezecond.for`) | Soil conductivity adjustment path for frost-active conditions; `getFreezeCond` selects frozen-soil coefficient from land-use/plant class; `frsoil` aggregates fine-layer conductivity to layer exports. | Frozen-soil conductivity and infiltration-capacity coupling authority under frost-active state. | `frost.runtime_infcap_frz`, `frost.options.kfactor{1,2,3}`, `frost.options.wintRed` |
| `winthd` (`winthd.for`) | Winter report/output helper routine. | Output/reporting surface for winter diagnostics; does not replace runtime boundary authority. | governance/reporting only |

### SIMIMPL32 Contract-Derived Test Requirements (Frost Scope)

SIMIMPL32 must implement contract-derived tests that demonstrate:

1. Dispatch trigger closure:
   active winter-frost trigger conditions dispatch frost processing and
   explicit disabled conditions skip processing without ambiguous side effects.
2. Handoff direction closure:
   `frwatc(1)` and `frwatc(0)` direction semantics are preserved at runtime
   seam boundaries with typed failures on missing/invalid handoff state.
3. Freeze lineage closure:
   `frzng`/`frznw` branch execution preserves finite/non-negative
   freeze-depth/water-state updates with explicit failure posture on invalid
   energy-time domains.
4. Conductivity lineage closure:
   `frsoil` + `getFreezeCond` land-use coefficient selection remains explicit
   and drives frost-active infiltration-capacity coupling exports.
5. Cross-contract seam closure:
   frost runtime payloads consumed by `SC-SOIL-001`, `SC-RUNOFFPART-001`,
   `SC-WATBAL-001`, and `SC-SYSTEM-001` remain complete, finite, and typed.

## Known Gaps

| Gap ID | Statement | Impact | Promotability | Evidence |
|---|---|---|---|---|
| GAP-SNOWFREEZE-001 | Per-invariant comparator vectors for hourly winter outputs (`hrmelt`, frost depth/thaw depth, freeze-thaw cycles) are not yet curated. | Limits immediate automated regression depth on hourly-heavy winter internals. | promotable-with-risk | `[DIRECT][Static]` |
| GAP-SNOWFREEZE-002 | SIMIMPL31 closes frost routine-chain authority mapping, but `frost.hourly.*` runtime family implementation and contract-derived frost execution tests remain open. | Authority ambiguity is removed, but executable frost hourly/process parity evidence is still pending. | promotable-with-risk | `[DIRECT][Static] + [INFERENCE][Static]` |
| GAP-SNOWFREEZE-003 | Snow drifting equations are documented in Chapter 3 but explicitly inactive in the August 1995 lineage; active-path authority for openWEPP is unresolved. | Drift-related claims cannot be promoted as active behavior yet. | non-promotable | `[DIRECT][Static]` |
| GAP-SNOWFREEZE-004 | Cross-contract boundary ownership with `SC-SOIL-001` and `SC-RUNOFFPART-001` is explicit, but executable cross-contract comparator vectors for frost-hourly internals are still incomplete. | Promotable contract authority exists; evidence depth for coupled frost vectors remains limited pending SIMIMPL32 and SIMIMPL35. | promotable-with-risk | `[DIRECT][Static] + [INFERENCE][Static]` |
| GAP-SNOWFREEZE-005 | `Dsavail` alias is fixed (`snow.hourly.depth_available_m`) and SIMIMPL29 emits the hourly family, but comparator-tier depth/density/melt vector breadth remains limited for broad climate regimes. | Residual risk is evidence-depth, not missing alias/state publication. | promotable-with-risk | `[DIRECT][Static] + [INFERENCE][Static]` |

## Revision History

| Date UTC | Version | Author | Change |
|---|---|---|---|
| `2026-06-03` | `15` | `Codex` | HPHYS0271 amendment: added day-36 melt-forcing lineage authority (`INV-SNOWFREEZE-017`) requiring `melt.for` term-level and hourly-forcing evidence before assigning H1 spurious melt residual ownership. |
| `2026-06-03` | `14` | `Codex` | HPHYS0270 amendment: added daily snowpack carry-state closure authority (`INV-SNOWFREEZE-016`) requiring pre-day/post-day SWE, depth, density, settle-count, and delta evidence before assigning H1/H7/H39 spring residual ownership. |
| `2026-06-03` | `13` | `Codex` | HPHYS0269 follow-up amendment: accepted corrected `/workdir/wepp-forest` commit `03fee455` as superseding authority for the daily negative-melt redistribution branch, rejecting pinned-baseline sign/branch bug compatibility as target behavior. |
| `2026-06-03` | `12` | `Codex` | HPHYS0269 amendment: added pinned-baseline `winter.for`/`snowd.for`/`melt.for` authority for signed hourly melt, daily negative-melt redistribution, and rain-on-snow retention (`INV-SNOWFREEZE-015`) with trace aliases and producer obligations. |
| `2026-06-03` | `11` | `Codex` | HPHYS0268 amendment: added spring snowpack lineage invariant requiring runtime SWE/depth/density/settle, hourly rain/snow/melt, signed `S`, WB13 `RM`, and WB13 `Snow-Water` evidence before returning material H1/H7/H39 `Ep` residual ownership to WB17. |
| `2026-06-02` | `10` | `Codex` | HPHYS0247 amendment: clarified `INV-SNOWFREEZE-009` so winter activation is driven by runtime snow/frost/thermal triggers rather than snow-sidecar presence alone when parsed default snow controls are explicitly available. |
| `2026-05-20` | `0` | `Codex` | Initial canonical stub created by SCI-05 work-package prep. |
| `2026-05-20` | `1` | `Codex` | Full draft authored with authority anchors, invariants, guard map, alias map, obligations, boundary disposition, tolerances, and gap register. |
| `2026-05-20` | `2` | `Codex` | Post-review amendment pass: resolved drift runtime/governance conflict, added missing frost/thaw symbols, fixed `InfCap_frz` unit declaration, clarified melt bound timing semantics, and tightened zero-depth/zero-density tolerance rule. |
| `2026-05-23` | `3` | `Codex` | CLIM05 amendment: added parsed snow-control runtime coupling authority (`snow.options.*`), signed `S` and `snow.runtime_swe` closure requirements, and active-coupling typed guard posture. |
| `2026-05-23` | `4` | `Codex` | CLIM06 amendment: added parsed frost-control runtime coupling authority (`frost.options.*`), explicit frozen-soil infiltration-capacity reduction envelope, and active-coupling typed guard posture for derived frost runtime surfaces. |
| `2026-05-25` | `5` | `Codex` | SIMIMPL18 amendment: added day-key cold-partition/publication closure (`INV-SNOWFREEZE-011`), explicit runtime-SWE-to-`Snow-Water` publication authority, and obligations preventing static sidecar (`snow.options.ssd`) leakage into dynamic storage outputs. |
| `2026-05-25` | `6` | `Codex` | SIMIMPL27 amendment: finalized concrete snow/freeze boundary alias mapping (typed + reserved hourly namespaces), added downstream contract-derived test requirements for hourly migration waves, and reclassified boundary/API gap posture from non-promotable ambiguity to implementation-queued promotable-with-risk. |
| `2026-05-25` | `7` | `Codex` | SIMIMPL28 amendment: split hourly reserved-family obligations into SIMIMPL28 forcing-emission scope vs SIMIMPL29 kernel-state closure scope, clarified deferred frost/depth-density family ownership, and updated gap posture to reflect staged hourly alias closure. |
| `2026-05-25` | `8` | `Codex` | SIMIMPL29 amendment: added baseline-authoritative snow kernel (`snowd`/`melt`) hourly state closure authority, runtime carry-state publication requirements, active hourly symbol hard-fail posture, and updated gap posture to reflect snow-family closure with frost-hourly follow-on ownership. |
| `2026-05-26` | `9` | `Codex` | SIMIMPL31 amendment: ratified baseline frost routine-chain authority (`winter`/`frostN`/`frwatc`/`frzng`/`frznw`/`frsoil`/`getFreezeCond`/`winthd`), added frost dispatch/conductivity closure invariants (`INV-SNOWFREEZE-012/013`), and defined SIMIMPL32 contract-derived frost test obligations with updated gap ownership. |
