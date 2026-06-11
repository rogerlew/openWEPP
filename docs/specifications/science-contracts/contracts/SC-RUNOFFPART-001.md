---
contract_id: SC-RUNOFFPART-001
title: Surface Runoff Partition Process Contract
status: in_review
maturity: draft
owner: openWEPP maintainers + hydrology reviewer
contract_version: 40
producer_scope:
  - Event-scale infiltration accounting and rainfall-excess partition surfaces
  - Depression-storage satisfaction/release and runoff onset transition surfaces
  - OFE-to-OFE runoff/runon and outlet runoff-volume/peak-rate aggregation surfaces
consumer_scope:
  - Daily water-balance consumers requiring surface-runoff depth (`Q`)
  - Erosion/hydraulics consumers requiring runoff duration, volume, and peak discharge
  - Comparator/replay surfaces using Tier-A single-OFE runoff acceptance signals
evidence_level: static
last_reviewed: 2026-06-07
supersedes: []
superseded_by: []
---

# SC-RUNOFFPART-001 Surface Runoff Partition Process Contract

Status: `in_review`
Maturity: `draft`
Evidence mode: `static`

## Purpose

Define top-down scientific authority for event-scale runoff partition behavior:
infiltration, rainfall excess generation, depression storage handling, peak
runoff estimation, and multiple-OFE runoff/runon approximation boundaries.

## Scientific Scope

In scope:
- Event-scale infiltration and rainfall-excess partition semantics.
- Depression storage satisfaction and runoff-onset boundary behavior.
- Peak discharge and effective runoff duration surfaces required by erosion
  coupling.
- Multiple-OFE approximation boundaries (case logic, averaged infiltration
  parameters, equivalent-plane aggregate coefficient).

Out of scope:
- Kernel implementation details and Rust API naming.
- Daily root-zone closure accounting owned by `SC-WATBAL-001`.
- Sediment continuity and detachment/transport kernels owned by
  `SC-SED-001`.
- Watershed channel/impoundment routing surfaces.

## Authority Anchors

| Anchor ID | Source anchor | Contract use | Evidence |
|---|---|---|---|
| REF-RUNOFFPART-CH4-INTRO | `references/50201000/chap4.pdf` §4.1 | Declares surface-hydrology sequence and primary outputs (rainfall-excess duration/intensity, runoff volume, peak discharge, infiltration handoff). | `[DIRECT][Static]` |
| REF-RUNOFFPART-CH4-INFIL | `chap4.pdf` §4.2, Eq. [4.2.1]-[4.2.9] | GAML infiltration and ponding/cease-ponding indicator semantics. | `[DIRECT][Static]` |
| REF-RUNOFFPART-CH4-RAINEX | `chap4.pdf` §4.3, Eq. [4.3.1], [4.3.2], [4.3.5] | Rainfall-excess generation from rainfall/infiltration/storage conditions and interval-rate assembly. | `[DIRECT][Static]` |
| REF-RUNOFFPART-CH4-DEPSTOR | `chap4.pdf` §4.3, Eq. [4.3.3], [4.3.4] + storage-condition text | Depression-storage reduction, onset sequencing, and hiatus infiltration behavior. | `[DIRECT][Static]` |
| REF-RUNOFFPART-CH4-KWAVE | `chap4.pdf` §4.4.1, Eq. [4.4.1]-[4.4.3], [4.4.10] | Kinematic-wave routing basis and time-to-equilibrium behavior for single OFE. | `[DIRECT][Static]` |
| REF-RUNOFFPART-CH4-PEAK | `chap4.pdf` §4.4.2, Eq. [4.4.17]-[4.4.26] | Approximate peak-discharge method for variable rainfall excess in continuous mode. | `[DIRECT][Static]` |
| REF-RUNOFFPART-CH4-RECESS | `chap4.pdf` §4.4.3, Eq. [4.4.27]-[4.4.29] | Recession-infiltration runoff-volume reduction semantics for partial-equilibrium events. | `[DIRECT][Static]` |
| REF-RUNOFFPART-CH4-DURATION | `chap4.pdf` §4.4.4, Eq. [4.4.30] | Effective runoff-duration continuity (`De = Qv/qp`) for erosion coupling. | `[DIRECT][Static]` |
| REF-RUNOFFPART-CH4-MULTIOFE | `chap4.pdf` §4.5, Eq. [4.5.1]-[4.5.15] | Multiple-OFE case classification, averaged infiltration/depression parameters, runon/runoff branching, and equivalent-plane coefficient construction. | `[DIRECT][Static]` |
| REF-RUNOFFPART-CH4-LIMITS | `chap4.pdf` §4.6 | Domain limitations (Hortonian-flow framing, no explicit variable-source-area/return-flow treatment, recession approximation limits). | `[DIRECT][Static]` |
| REF-RUNOFFPART-CH5-COUPLING | `references/50201000/chap5.pdf` §5.1 Eq. [5.1.1] | Daily water-balance consumer uses runoff depth `Q` as a closure term with signed conventions preserved. | `[DIRECT][Static]` |
| REF-RUNOFFPART-LEGACY-HOURLY-CARRY | `/workdir/wepp-forest_260430_baseline/src/wathour.inc:26-44` and `/workdir/wepp-forest_260430_baseline/src/watbal_hourly.for:438-471,843-902` (`dac3c950d8b16cc73774bf5ce2e7e11f80baac70`) | MOFE hourly upstream saturation/lateral carry arrays are part of the runoff/runon source term consumed before current-OFE runoff partition closure, and current-OFE `ui_SCrunf(ii)` surface-saturation excess is added to daily `runoff`. | `[DIRECT][Static]` |
| REF-RUNOFFPART-LEGACY-WINTER-RAINRELEASE | `/workdir/wepp-forest_260430_baseline/src/snowd.for` lines 240-279 and `/workdir/wepp-forest_260430_baseline/src/winter.for` lines 456-459, commit `dac3c950d8b16cc73774bf5ce2e7e11f80baac70` | Rain-on-snow holding-capacity accounting retains part or all of `hrrain` in snowpack; any positive residual `hrrain` is added into `hrmlt` before daily `wmelt` is summed. | `[DIRECT][Static]` |
| REF-RUNOFFPART-LEGACY-WMELT-INFIL | `/workdir/wepp-forest_260430_baseline/src/watbal_hourly.for` lines 342-345 and `/workdir/wepp-forest_260430_baseline/src/grna.for` lines 267-269, commit `dac3c950d8b16cc73774bf5ce2e7e11f80baac70` | Snowmelt (`wmelt`) is part of the same event water supply used by infiltration and runoff partition: `fin` includes `wmelt`, and Green-Ampt snowmelt forcing derives `smrate = wmelt / dur`. | `[DIRECT][Static]` |
| REF-RUNOFFPART-LEGACY-WB13-RM | `/workdir/wepp-forest_260430_baseline/src/contin.for:847-880`, `/workdir/wepp-forest_260430_baseline/src/watbalprint.for:84-106`, and `/workdir/wepp-forest_260430_baseline/src/watbal_hourly.for:1082-1142`, commit `dac3c950d8b16cc73774bf5ce2e7e11f80baac70` | WB13 `RM` consumes post-winter `rain(iplane)` and daily routed `wmelt(iplane)` from the same runoff/infiltration lineage. Winter-active days clear `rain(iplane)` except the warm-rain/no-snow restoration branch, so publication must not reconstruct `RM` from raw precipitation when routed `wmelt` is available. | `[DIRECT][Static]` |
| REF-RUNOFFPART-LEGACY-FIN-INFIL | `/workdir/wepp-forest_260430_baseline/src/watbal_hourly.for:342-345,471-479,494-516,520-524`, commit `dac3c950d8b16cc73774bf5ce2e7e11f80baac70` | Same-pass local `fin/xfin` liquid supply includes direct rain after interception, routed snowmelt, and irrigation before infiltration/runoff closure and layer-storage ingress; MOFE carry/runon arrays are governed by `REF-RUNOFFPART-LEGACY-HOURLY-CARRY` and require separate promotion for storage-ingress closure. | `[DIRECT][Static]` |
| REF-RUNOFFPART-LEGACY-IDAT-INTERCEPT | `/workdir/wepp-forest_260430_baseline/src/idat.for:286-291`, commit `dac3c950d8b16cc73774bf5ce2e7e11f80baac70` | Baseline canopy/residue interception equation-input bound: live/dead mass is limited to `8000 kg ha^-1` before evaluating the second-order interception equation; the plant biomass state itself is not rejected by this cap. | `[DIRECT][Static]` |
| REF-RUNOFFPART-CH11-COUPLING | `references/50201000/chap11.pdf` chapter context + `chap4.pdf` §4.4.4 | Erosion continuity uses peak runoff and effective duration surfaces from runoff partition domain. | `[DIRECT][Static] + [INFERENCE][Static]` |
| REF-RUNOFFPART-PHYS-BOUNDS | Physical/common-sense invariant class | Non-negative depth/volume/rate magnitudes (except explicitly signed intermediary comparisons) and bounded branch domains. | `[INFERENCE][Static]` |

## Variables and Units (Externally Relevant)

| Symbol | Units | Meaning | Producer | Consumer |
|---|---|---|---|---|
| `R`, `Ri` | `m` | Event/interval cumulative rainfall depth. | climate forcing interface | infiltration/rainfall-excess partition |
| `r`, `ri` | `m s^-1` | Event/interval rainfall rate. | climate forcing interface | infiltration/rainfall-excess partition |
| `F`, `Fi` | `m` | Event/interval cumulative infiltration depth. | infiltration component | rainfall-excess and water-balance coupling |
| `fi` | `m s^-1` | Interval-average infiltration rate (Eq. [4.2.1]). | infiltration component | rainfall-excess branch logic |
| `V`, `Vi` | `m` | Event/interval cumulative rainfall-excess depth after declared adjustments. | rainfall-excess component | runoff routing and water-balance `Q` coupling |
| `vi` | `m s^-1` | Interval-average rainfall-excess rate (Eq. [4.3.5]). | rainfall-excess component | runoff routing and peak approximation |
| `Ke` | `m s^-1` | Effective saturated hydraulic conductivity in GAML relations. | soil/hydrology parameterization | infiltration and multi-OFE averaging |
| `Ψ`, `θd` | `m`, `m m^-1` | Capillary potential and soil-moisture deficit terms in infiltration relations. | soil/hydrology parameterization | infiltration and multi-OFE averaging |
| `Sp` | `m` | Upper-limit storage threshold in top two layers for rainfall-excess branch selection (Eq. [4.3.2]). | infiltration/rainfall-excess coupling | rainfall-excess branch logic |
| `Sd` | `m` | Maximum depression-storage depth (Eq. [4.3.4]). | depression-storage subcomponent | runoff-onset and rainfall-excess reduction |
| `rr`, `So` | `m`, `m m^-1` | Random roughness and plane slope used for depression storage. | topography/soil parameterization | depression-storage computation |
| `Qv` | `m` | Routed/adjusted runoff depth used in duration coupling. | runoff routing component | erosion coupling and event summaries |
| `qp` | `m^2 s^-1` | Peak runoff rate per unit width. | peak-discharge component | erosion continuity coupling |
| `De` | `s` | Effective runoff duration (`Qv/qp`). | runoff partition component | erosion computations |
| `Qj-1`, `Vj`, `Qj` | `m` | Runon from upper OFE, rainfall excess on current OFE, runoff from current OFE in multi-OFE case logic. | OFE cascade routing | downstream OFE branch logic |
| `ui_SUrunf(ii)`, `ui_LfUrf(ii)` | `m` per hourly substep | Hourly upstream saturation-runoff and lateral-flow carry entering current-OFE runoff/runon closure for `ii=1..24`. | upstream OFE hourly carry state | current-OFE WB12/WB14 runoff partition closure |
| `ui_SCrunf(ii)`, `ui_LfCrf(ii)` | `m` per hourly substep | Hourly current-OFE saturation-runoff and lateral-flow carry published for downstream copy-forward. | current-OFE runoff/lateral runtime | downstream OFE runon/carry closure |
| `surdra` | `m` | Daily sum of current-OFE hourly surface-saturation excess, `Σui_SCrunf(ii)`, added to published runoff. | current-OFE WB19/WB12 runtime | WB12/WB14 runoff and water-balance closure |
| `Ka`, `(Ψθd)a`, `Sa` | `m s^-1`, `m`, `m` | Weighted-average infiltration/depression parameters for applicable OFE range (Eq. [4.5.1]-[4.5.3]). | multi-OFE aggregator | case classification and branch calculations |
| `Fp`, `Fh` | `m^3 m^-1` | Potential infiltration capacity per unit width and incoming unit-width water volume (Eq. [4.5.4]-[4.5.7]). | multi-OFE runon/runoff logic | case-three/case-four branch decision |
| `α`, `αe`, `m` | coeff, coeff, exponent | Depth-discharge coefficients/exponent for routing and equivalent-plane transformation (Eq. [4.4.2], [4.5.15]). | routing/aggregation components | peak/runoff hydrograph approximation |

## Event Closure Term Definition

For `INV-RUNOFFPART-001`, event accounting must publish an explicit closure
relation at the declared boundary:

`Rtot + Qin = Ftot + Qv + ΔSdep + εevt`

where:
- `Rtot` is event rainfall depth contribution;
- `Qin` is runon contribution entering the declared boundary (zero for isolated
  single-OFE events);
- `Ftot` is cumulative infiltration depth at event end;
- `Qv` is final routed runoff depth after declared depression-storage and
  recession-infiltration adjustments;
- `ΔSdep` is net depression-storage retention at event end (allowed `>= 0`);
- `εevt` is the explicit closure residual constrained by `TOL-RUNOFFPART-001`.

This relation is an accounting identity for contract enforcement and does not
replace the Chapter-4 process equations. `[DIRECT][Static] + [INFERENCE][Static]`

## Invariants

| Invariant ID | Statement | Severity | Authority | Evidence |
|---|---|---|---|---|
| INV-RUNOFFPART-001 | Event water-partition closure invariant: each modeled event must expose an explicit accounting relation where rainfall inputs are partitioned into infiltration, retained depression-storage effects, and routed runoff outputs (`Qv`), with any residual explicitly computed and bounded by tolerance. | hard-fail | REF-RUNOFFPART-CH4-INTRO, REF-RUNOFFPART-CH4-INFIL, REF-RUNOFFPART-CH4-RAINEX, REF-RUNOFFPART-CH4-DEPSTOR, REF-RUNOFFPART-PHYS-BOUNDS | `[DIRECT][Static] + [INFERENCE][Static]` |
| INV-RUNOFFPART-002 | Rainfall-excess generation invariant: Eq. [4.3.1] branch semantics must be applied consistently (`ri > fi` with storage condition yields rainfall excess; `ri <= fi` does not increase excess; exceeded storage limit branch is explicit). | hard-fail | REF-RUNOFFPART-CH4-RAINEX | `[DIRECT][Static]` |
| INV-RUNOFFPART-003 | Depression-storage onset invariant: runoff routing cannot begin until declared depression-storage condition is satisfied, and depression-storage reduction follows Eq. [4.3.3]-[4.3.4]. | hard-fail | REF-RUNOFFPART-CH4-DEPSTOR, REF-RUNOFFPART-PHYS-BOUNDS | `[DIRECT][Static] + [INFERENCE][Static]` |
| INV-RUNOFFPART-004 | Infiltration-domain invariant: GAML state variables (`Ke`, `Ψ`, `θd`, `F`, `R`, `V`) must remain physically valid (`Ke > 0`, finite states, non-negative cumulative depths), and ponding/no-ponding transitions use Eq. [4.2.2]-[4.2.9] branch logic explicitly. | hard-fail | REF-RUNOFFPART-CH4-INFIL, REF-RUNOFFPART-PHYS-BOUNDS | `[DIRECT][Static] + [INFERENCE][Static]` |
| INV-RUNOFFPART-005 | Peak-discharge invariant: emitted peak runoff (`qp`) and derived effective duration (`De`) must be consistent with the selected routing pathway and continuity relation Eq. [4.4.30], with `qp > 0` whenever `Qv > 0`. | hard-fail | REF-RUNOFFPART-CH4-KWAVE, REF-RUNOFFPART-CH4-PEAK, REF-RUNOFFPART-CH4-DURATION, REF-RUNOFFPART-CH11-COUPLING | `[DIRECT][Static] + [INFERENCE][Static]` |
| INV-RUNOFFPART-006 | Partial-equilibrium adjustment invariant: when recession-infiltration correction is triggered, runoff-depth reduction must follow Eq. [4.4.27]-[4.4.29], and adjusted runoff cannot exceed pre-adjustment rainfall-excess volume. | hard-fail | REF-RUNOFFPART-CH4-RECESS, REF-RUNOFFPART-PHYS-BOUNDS | `[DIRECT][Static] + [INFERENCE][Static]` |
| INV-RUNOFFPART-007 | Multiple-OFE case invariant: four-case logic in §4.5.1 must classify each downstream OFE state using `Qj-1` and `Vj`, with explicit branch outcomes and no silent fall-through between case-three and case-four conditions. | hard-fail | REF-RUNOFFPART-CH4-MULTIOFE | `[DIRECT][Static]` |
| INV-RUNOFFPART-008 | Multiple-OFE aggregation invariant: weighted averages and equivalent-plane transformations (Eq. [4.5.1]-[4.5.3], [4.5.15]) must preserve non-negative domains and declared geometric weighting. | hard-fail | REF-RUNOFFPART-CH4-MULTIOFE, REF-RUNOFFPART-PHYS-BOUNDS | `[DIRECT][Static] + [INFERENCE][Static]` |
| INV-RUNOFFPART-009 | Coupling invariant: runoff depth `Q` exported to daily water balance and runoff surfaces exported to erosion consumers (`Qv`, `qp`, `De`) must be unit-consistent and sign-consistent with downstream contract assumptions. | hard-fail | REF-RUNOFFPART-CH5-COUPLING, REF-RUNOFFPART-CH11-COUPLING | `[DIRECT][Static] + [INFERENCE][Static]` |
| INV-RUNOFFPART-010 | Governance limitation invariant: model outputs must remain labeled as Hortonian-flow-scope surfaces; scenarios needing explicit variable-source-area or return-flow physics are out of contract scope and block promotion if unlabeled. | governance-fail | REF-RUNOFFPART-CH4-LIMITS | `[DIRECT][Static] + [INFERENCE][Static]` |
| INV-RUNOFFPART-011 | WB20 forward-solver lane invariant: when `wb20_forward_solver_lane_enabled = 1`, WB12 runoff closure-delta acceptance must be solver-residual-derived and must not consume `wb12_runoff_observed` as an acceptance-driving target. | hard-fail | REF-RUNOFFPART-CH4-RAINEX, REF-RUNOFFPART-CH5-COUPLING, REF-RUNOFFPART-PHYS-BOUNDS | `[DIRECT][Static] + [INFERENCE][Static]` |
| INV-RUNOFFPART-012 | HPHYS0240 runoff-carryover authority invariant: WB12/WB14 runoff partitioning must use same-pass `wb12_runoff_carryover` flux when present, use `wb12_runon_input` only as finite non-negative compatibility input when the flux is absent, and publish the resolved carryover flux into the runoff closure/output boundary. Malformed present carryover fluxes hard-fail and cannot be silently replaced by stale state. | hard-fail | REF-RUNOFFPART-CH4-RAINEX, REF-RUNOFFPART-CH5-COUPLING, REF-RUNOFFPART-PHYS-BOUNDS | `[DIRECT][Static] + [INFERENCE][Static]` |
| INV-RUNOFFPART-013 | HPHYS0241 MOFE hourly carry-array runoff invariant: when MOFE hourly carry arrays are enabled, WB12/WB14 runoff partitioning must resolve runon from the explicit 24-slot upstream array sum `Σ(ui_SUrunf(ii) + ui_LfUrf(ii))` with required area-scaling provenance when non-zero upstream carry is present. Present aggregate `wb12_runoff_carryover` must match the array-derived value or hard-fail; missing/malformed arrays cannot fall back to daily aggregate state. | hard-fail | REF-RUNOFFPART-LEGACY-HOURLY-CARRY, REF-RUNOFFPART-CH4-MULTIOFE, REF-RUNOFFPART-CH5-COUPLING, REF-RUNOFFPART-PHYS-BOUNDS | `[DIRECT][Static] + [INFERENCE][Static]` |
| INV-RUNOFFPART-014 | HPHYS0242 surface-saturation runoff addback invariant: when MOFE hourly carry arrays are enabled, WB12/WB14 runoff publication must add `surdra = Σui_SCrunf(ii)` to partition-derived runoff `Q`, use that same addback in closure residuals, and reject missing, malformed, non-finite, negative, or aggregate-only current saturation payloads. Positive top-layer saturation excess cannot remain hidden in storage or be replaced by `wb12_runoff_carryover`. | hard-fail | REF-RUNOFFPART-LEGACY-HOURLY-CARRY, REF-RUNOFFPART-CH4-RAINEX, REF-RUNOFFPART-CH5-COUPLING, REF-RUNOFFPART-PHYS-BOUNDS, SC-WATBAL-001#INV-WATBAL-034 | `[DIRECT][Static] + [INFERENCE][Static]` |
| INV-RUNOFFPART-015 | HPHYS0283 snowmelt event-forcing invariant: WB12 runoff partition must include routed snowmelt in the event liquid supply available to Green-Ampt infiltration before residual runoff is computed, and the resulting infiltration must remain available to WB18 same-pass layer ingress. A closure that adds melt to `Q`/runoff residuals while omitting it from infiltration forcing or downstream layer storage violates baseline `wmelt -> smrate/fin/xfin` semantics. | hard-fail | REF-RUNOFFPART-LEGACY-WMELT-INFIL, REF-RUNOFFPART-CH4-INFIL, REF-RUNOFFPART-CH4-RAINEX, SC-SNOWFREEZE-001#INV-SNOWFREEZE-018, SC-PERC-001#INV-PERC-016 | `[DIRECT][Static] + [INFERENCE][Static]` |
| INV-RUNOFFPART-016 | HPHYS0285 local-liquid infiltration handoff invariant: WB12/WB14 runoff partition must treat positive same-pass infiltration from direct rain, routed snowmelt, or irrigation as both a runoff residual term and a WB18 layer-storage ingress obligation. Active-snow state may affect snowmelt supply, but it cannot gate whether non-snow positive local infiltration remains available to WB18 `fin/xfin` ingress. MOFE carry/runon infiltration remains a follow-up closure item under the existing carry-array invariants. | hard-fail | REF-RUNOFFPART-LEGACY-FIN-INFIL, REF-RUNOFFPART-CH4-INFIL, REF-RUNOFFPART-CH4-RAINEX, SC-PERC-001#INV-PERC-017, SC-WATBAL-001#INV-WATBAL-060 | `[DIRECT][Static] + [INFERENCE][Static]` |
| INV-RUNOFFPART-017 | HPHYS0287 snow liquid partition guard invariant: WB12/WB14 runoff partition must validate projected runtime snow-state domains before deciding that snow coupling is inactive and before assembling direct-rain, retained-rain, routed-melt, irrigation, infiltration, and residual runoff terms. When any snow option/control/runtime state is projected, missing runtime snow vector members must hard-fail rather than defaulting to zero. Material negative or non-finite `snow.runtime_swe`/depth/density/settle state must hard-fail instead of being canonicalized to zero by the inactive branch. This guard protects the liquid-partition lineage without allowing invalid snow state to suppress valid non-snow infiltration. | hard-fail | SC-SNOWFREEZE-001#INV-SNOWFREEZE-020, INV-RUNOFFPART-015, INV-RUNOFFPART-016, REF-RUNOFFPART-CH4-INFIL, REF-RUNOFFPART-PHYS-BOUNDS | `[DIRECT][Static] + [INFERENCE][Static]` |
| INV-RUNOFFPART-018 | HPHYS0288 rain-on-snow routed-melt partition invariant: WB12/WB14 liquid forcing must treat positive rain-on-snow residual released by `snowd.for` holding-capacity accounting as part of routed `wmelt` after `winter.for` daily post-processing. Direct-rain liquid supplied to the hyetograph path must be reduced by both retained rain and residual rain-on-snow that has been promoted into `wmelt`; leaving that residual on the direct-rain-only path, omitting it from `wmelt`, or double counting it across direct rain and routed melt violates the baseline `hrrain -> hrmlt -> wmelt -> fin/smrate` lineage. | hard-fail | REF-RUNOFFPART-LEGACY-WINTER-RAINRELEASE, REF-RUNOFFPART-LEGACY-WMELT-INFIL, SC-SNOWFREEZE-001#INV-SNOWFREEZE-021, SC-WATBAL-001#INV-WATBAL-063 | `[DIRECT][Static] + [INFERENCE][Static]` |
| INV-RUNOFFPART-019 | HPHYS0289 routed-melt publication invariant: WB12/WB14 must publish an explicit daily routed-melt depth equivalent to baseline `wmelt(iplane)` after winter redistribution and rain-on-snow release so WB13 can compute `RM = post-winter rain + wmelt + irrigation`. The publication surface must be finite, non-negative, and identical to the snowmelt liquid term used by infiltration/runoff closure; deriving WB13 `RM` from raw precipitation and SWE delta while the kernel used a different routed-melt term is invalid. | hard-fail | REF-RUNOFFPART-LEGACY-WMELT-INFIL, REF-RUNOFFPART-LEGACY-WB13-RM, SC-SNOWFREEZE-001#INV-SNOWFREEZE-022, SC-WATBAL-001#INV-WATBAL-064 | `[DIRECT][Static] + [INFERENCE][Static]` |
| INV-RUNOFFPART-020 | HPHYS0290 post-winter rain publication invariant: WB12/WB14 must publish the post-winter direct-rain depth equivalent to baseline `rain(iplane)` after winter clearing/restoration, snowpack retention, and rain-on-snow promotion to `wmelt`. The openWEPP surface is `snow.post_winter_rain_m`; it must be finite, non-negative, and sourced from the same liquid partition used for direct-rain hyetograph/runoff forcing. WB13 may not reconstruct this term from raw precipitation, snow-active flags, SWE deltas, or routed melt. | hard-fail | REF-RUNOFFPART-LEGACY-WB13-RM, REF-RUNOFFPART-LEGACY-WINTER-RAINRELEASE, INV-RUNOFFPART-018, INV-RUNOFFPART-019, SC-SNOWFREEZE-001#INV-SNOWFREEZE-023, SC-WATBAL-001#INV-WATBAL-065 | `[DIRECT][Static] + [INFERENCE][Static]` |
| INV-RUNOFFPART-021 | HPHYS0291 snow publication lifecycle invariant: runoff reconciliation owns same-day publication of `snow.post_winter_rain_m` and `snow.routed_melt_m` into the flux surface before WB13 publication. The publisher must emit finite non-negative values on every daily execution, including explicit zeroes when no routed melt or post-winter direct rain exists, and WB13 may not accept state/default substitutes for absent producer fluxes. | hard-fail | INV-RUNOFFPART-019, INV-RUNOFFPART-020, SC-SNOWFREEZE-001#INV-SNOWFREEZE-024, SC-WATBAL-001#INV-WATBAL-066, REF-RUNOFFPART-LEGACY-WB13-RM | `[DIRECT][Static] + [INFERENCE][Static]` |
| INV-RUNOFFPART-022 | HPHYS0292 spring snowmelt infiltration-capacity localization invariant: WB12/WB14 spring diagnostics must prove whether residual `Q` is caused by routed-melt magnitude/timing or by infiltration-capacity partitioning before downstream storage/ET ownership is assigned. The event liquid supply must include direct liquid after interception, routed `wmelt`, irrigation, and runon before residual `Q` is calculated. Routed snowmelt timing must use the producer hourly melt shape while conserving the daily routed-melt scalar; uniformly spreading active melt across non-melt hours is invalid. Where conductivity and matric-potential capacity can absorb active routed melt, `wb12_infiltration` must consume the routed melt and `Q` must remain zero within tolerance; where `Q` remains positive, evidence must record the capacity inputs and cumulative infiltration that produced the residual. | hard-fail | REF-RUNOFFPART-LEGACY-WMELT-INFIL, REF-RUNOFFPART-LEGACY-WB13-RM, INV-RUNOFFPART-015, INV-RUNOFFPART-019, INV-RUNOFFPART-021, SC-SNOWFREEZE-001#INV-SNOWFREEZE-025, SC-WATBAL-001#INV-WATBAL-067 | `[DIRECT][Static] + [INFERENCE][Static]` |
| INV-RUNOFFPART-023 | HPHYS0293 post-WB14 runoff exclusion invariant: once HPHYS0292 target rows prove WB12 capacity consumes routed melt and residual `Q` is zero, subsequent snowpack/`RM` residual attribution must not reassign ownership back to runoff partitioning unless new evidence shows positive residual `Q`, capacity under-publication, or event-liquid closure failure. Snow producer magnitude/timing residuals must remain separated from WB14 capacity evidence. | governance-hold | INV-RUNOFFPART-022, SC-SNOWFREEZE-001#INV-SNOWFREEZE-026, SC-WATBAL-001#INV-WATBAL-068 | `[DIRECT][Static] + [INFERENCE][Static]` |
| INV-RUNOFFPART-024 | HPHYS0296 snow/`RM` runoff-consumer acceptance invariant: `RM` acceptance must prove that WB12/WB14 consumed the same producer-owned `snow.routed_melt_m` and `snow.post_winter_rain_m` surfaces published to WB13, and that residual `Q` remains closed, but closed runoff-consumer identity is necessary evidence only and is not sufficient to accept or remove a snow/`RM` residual from the failing set. Any window re-tiering must satisfy `SC-SNOWFREEZE-001#INV-SNOWFREEZE-027`: mechanistic `file:line` root cause in both openWEPP and `/workdir/wepp-forest_260430_baseline`, reconstruction controlled experiment to named tolerance, independent correctness adjudication, and explicit `LEGACY-DEFECTIVE`/`OPENWEPP-DEFECTIVE`/`UNRESOLVED` disposition. If candidate `RM` publication identity closes while candidate/baseline `RM` magnitude remains open without that defective-model verdict, runoff partitioning remains excluded and the package must hold on snow/winter producer authority rather than patching infiltration, runoff, or downstream storage. | governance-hold | INV-RUNOFFPART-021, INV-RUNOFFPART-022, INV-RUNOFFPART-023, SC-SNOWFREEZE-001#INV-SNOWFREEZE-027, SC-WATBAL-001#INV-WATBAL-071 | `[DIRECT][Static] + [INFERENCE][Static]` |
| INV-RUNOFFPART-025 | HPHYS0297 runoff-consumer defect-ledger invariant: runoff/infiltration may be excluded from a snow/`RM` verdict only when the ledger preserves closed `Q`, closed WB12/WB14-to-WB13 producer-consumer identity, and the `SC-SNOWFREEZE-001#INV-SNOWFREEZE-028` reconstruction verdict. Closed runoff identity cannot convert an unresolved snow producer residual into acceptance, and absent reconstruction keeps the window in `HOLD` rather than authorizing WB12/WB14/WB13 compensation. | governance-hold | INV-RUNOFFPART-024, SC-SNOWFREEZE-001#INV-SNOWFREEZE-028, SC-WATBAL-001#INV-WATBAL-072 | `[DIRECT][Static] + [INFERENCE][Static]` |
| INV-RUNOFFPART-026 | HPHYS0298 runoff-consumer paired-partition invariant: closed `Q`, closed WB12/WB14 liquid forcing, or closed WB13 `RM` identity may exclude runoff as first-divergent cut-point only when the HPHYS0298 partition ledger proves that identity on the same target window and ties it to `SC-SNOWFREEZE-001#INV-SNOWFREEZE-029`. The ledger must keep the ordered cut-point context and cannot convert downstream closure into snow/`RM` acceptance, WB12/WB14/WB13 compensation, or removal from semantic accounting when the first divergence is upstream or unresolved. | governance-hold | INV-RUNOFFPART-025, SC-SNOWFREEZE-001#INV-SNOWFREEZE-029, SC-WATBAL-001#INV-WATBAL-073 | `[DIRECT][Static] + [INFERENCE][Static]` |
| INV-RUNOFFPART-027 | FQ3-DC top-two-layer storage-limit invariant: WB12/WB14 infiltration-excess partition must apply the Eq. [4.3.2] upper-storage condition before publishing same-pass infiltration. When the declared top-two-layer storage surface is at or above its upper limit, current event liquid cannot be absorbed into same-pass infiltration merely because interval rainfall rate is below `Ke`; the storage-limited residual must remain in rainfall excess/runoff accounting subject to depression-storage and routing terms. If WB18/percolation has already published same-pass `wb12_infiltration`, WB14 runoff partition must consume that producer value rather than recomputing infiltration from a later storage state. | hard-fail | REF-RUNOFFPART-CH4-RAINEX, REF-RUNOFFPART-CH4-INFIL, REF-RUNOFFPART-PHYS-BOUNDS | `[DIRECT][Static] + [INFERENCE][Static]` |

## Invariant Guard Map

| Invariant ID | Guard class | Enforcement path | Failure behavior | Gate impact | Evidence |
|---|---|---|---|---|---|
| `INV-RUNOFFPART-001` | runtime | Event partition closure assembler | Typed hard error on residual above tolerance | Tier-A gate | `[DIRECT][Static] + [INFERENCE][Static]` |
| `INV-RUNOFFPART-002` | runtime | Rainfall-excess branch evaluator | Typed hard error on branch-condition violation or inconsistent branch output | Tier-A gate | `[DIRECT][Static]` |
| `INV-RUNOFFPART-003` | runtime | Depression-storage adjustment and runoff-start validator | Typed hard error when routing starts before required storage condition or storage adjustment violates Eq. [4.3.3]-[4.3.4] | Tier-A gate | `[DIRECT][Static] + [INFERENCE][Static]` |
| `INV-RUNOFFPART-004` | runtime | Infiltration domain/transition validator | Typed hard error on invalid domains or implicit ponding branch transitions | Tier-A gate | `[DIRECT][Static] + [INFERENCE][Static]` |
| `INV-RUNOFFPART-005` | runtime | Peak/duration coupling validator | Typed hard error on inconsistent `Qv`, `qp`, `De` continuity or non-physical `qp` domain | Tier-A/B gate | `[DIRECT][Static] + [INFERENCE][Static]` |
| `INV-RUNOFFPART-006` | runtime | Recession-infiltration adjustment calculator | Typed hard error if adjusted runoff exceeds allowed bounds or branch equations are misapplied | Tier-B investigation gate | `[DIRECT][Static] + [INFERENCE][Static]` |
| `INV-RUNOFFPART-007` | runtime | OFE case classifier | Typed hard error on ambiguous/missing case classification | Tier-A/B gate | `[DIRECT][Static]` |
| `INV-RUNOFFPART-008` | runtime | OFE weighted-parameter/equivalent-plane calculator | Typed hard error on invalid weighting geometry or negative transformed coefficients | Tier-B investigation gate | `[DIRECT][Static] + [INFERENCE][Static]` |
| `INV-RUNOFFPART-009` | runtime | Boundary payload validator for water-balance/erosion consumers | Typed hard error on missing required surfaces or units/sign mismatch | Tier-A gate | `[DIRECT][Static] + [INFERENCE][Static]` |
| `INV-RUNOFFPART-010` | governance | Contract review + promotion checklist | Promotion `HOLD` if Hortonian-only scope is not explicitly carried into downstream interpretation | Governance gate | `[DIRECT][Static] + [INFERENCE][Static]` |
| `INV-RUNOFFPART-011` | runtime | WB12 runoff closure-delta lane selector and assembler | Typed hard error when forward lane consumes observed target in acceptance logic or emits non-residual closure delta | Tier-A parity lane gate | `[DIRECT][Static] + [INFERENCE][Static]` |
| `INV-RUNOFFPART-012` | runtime + governance | WB12/WB14 runoff-carryover resolver and output-boundary validator | Typed hard error / explicit `HOLD` when same-pass `wb12_runoff_carryover` is ignored, malformed, or replaced by stale `wb12_runon_input` | HPHYS hourly carryover closure gate | `[DIRECT][Static] + [INFERENCE][Static]` |
| `INV-RUNOFFPART-013` | runtime + governance | MOFE hourly carry-array resolver, aggregate anti-shadow check, and malformed-array rejection vectors | Typed hard error / explicit `HOLD` when array-derived carry is absent/malformed, aggregate carry conflicts with arrays, or area-scaling provenance is missing for non-zero upstream carry | HPHYS MOFE hourly carry-array closure gate | `[DIRECT][Static] + [INFERENCE][Static]` |
| `INV-RUNOFFPART-014` | runtime + governance | WB19 current-saturation array validator plus WB12/WB14 runoff assembler | Typed hard error / explicit `HOLD` when `ui_SCrunf` is missing/malformed, current saturation addback is omitted from `Q`, or closure residuals use a different runoff value than the published flux | HPHYS cadence/order closure gate | `[DIRECT][Static] + [INFERENCE][Static]` |
| `INV-RUNOFFPART-015` | runtime + governance | WB12 meltwater event-forcing assembler and infiltration/runoff residual closure | Typed hard error / explicit `HOLD` when routed melt is excluded from infiltration forcing or is double-counted in runoff closure | HPHYS0283 spring partition gate | `[DIRECT][Static] + [INFERENCE][Static]` |
| `INV-RUNOFFPART-016` | runtime + governance | WB12 local-liquid infiltration handoff assembler and WB18 ingress availability validator | Typed hard error / explicit `HOLD` when positive same-pass infiltration is published as runoff closure only, active-snow-gated before WB18, or unavailable to layer storage | HPHYS0285 spring soil-retention gate | `[DIRECT][Static] + [INFERENCE][Static]` |
| `INV-RUNOFFPART-017` | runtime + governance | WB12/WB14 runtime snow-state validator before inactive snow fallback and liquid-partition assembly | Typed hard error / explicit `HOLD` when material negative snow runtime state is silently zeroed before runoff/infiltration partition | HPHYS0287 snow liquid partition gate | `[DIRECT][Static] + [INFERENCE][Static]` |
| `INV-RUNOFFPART-018` | runtime + governance | WB12/WB14 rain-on-snow direct-rain vs routed-melt assembler | Typed hard error / explicit `HOLD` when residual rain-on-snow is omitted from routed melt, left exclusively as direct rain, or double counted | HPHYS0288 rain-on-snow partition gate | `[DIRECT][Static] + [INFERENCE][Static]` |
| `INV-RUNOFFPART-019` | runtime + governance | WB12/WB14 daily routed-`wmelt` publisher and WB13 consumer seam | Typed hard error / explicit `HOLD` when routed melt is missing, negative, non-finite, or differs from the liquid term used by runoff/infiltration closure | HPHYS0289 WB13 publication gate | `[DIRECT][Static] + [INFERENCE][Static]` |
| `INV-RUNOFFPART-020` | runtime + governance | WB12/WB14 daily post-winter direct-rain publisher and WB13 consumer seam | Typed hard error / explicit `HOLD` when post-winter rain is missing, negative, non-finite, not tied to direct-rain forcing, or reconstructed downstream | HPHYS0290 WB13 post-winter-rain publication gate | `[DIRECT][Static] + [INFERENCE][Static]` |
| `INV-RUNOFFPART-021` | runtime + governance | WB12/WB14 same-day snow publication flux lifecycle before WB13 | Typed hard error / explicit `HOLD` when required fluxes are absent, stale-state shadowed, or defaulted instead of produced | HPHYS0291 snow publication lifecycle gate | `[DIRECT][Static] + [INFERENCE][Static]` |
| `INV-RUNOFFPART-022` | runtime + governance | WB12/WB14 spring routed-melt capacity classifier spanning event liquid supply, cumulative infiltration, and residual `Q` | Typed hard error / explicit `HOLD` when routed melt bypasses capacity evaluation or when `Q` ownership is asserted without capacity evidence | HPHYS0292 spring snowmelt/infiltration capacity gate | `[DIRECT][Static] + [INFERENCE][Static]` |
| `INV-RUNOFFPART-023` | governance | Post-WB14 exclusion classifier for HPHYS0293 snow residuals | Explicit `HOLD` if snowpack/`RM` residuals are reassigned to WB14 without positive `Q` or capacity-closure evidence | HPHYS0293 winter melt magnitude/timing gate | `[DIRECT][Static] + [INFERENCE][Static]` |
| `INV-RUNOFFPART-024` | governance | HPHYS0296 runoff-consumer classifier tying WB12/WB14 liquid forcing, residual `Q`, and WB13 `RM` publication identity to the same snow producer surfaces plus the snow/`RM` defective-model verdict gate | Explicit `HOLD` when `RM` magnitude residuals are reassigned to runoff/storage despite closed `Q`, closed producer-consumer identity, and absent per-window defective-model verdict | HPHYS0296 snow/`RM` acceptance gate | `[DIRECT][Static] + [INFERENCE][Static]` |
| `INV-RUNOFFPART-025` | governance | HPHYS0297 runoff exclusion ledger tying closed `Q` and producer-consumer identity to snow/`RM` reconstruction verdicts | Explicit `HOLD` if runoff/storage compensation is asserted without the HPHYS0297 reconstruction verdict | HPHYS0297 snow/`RM` defect-ledger gate | `[DIRECT][Static] + [INFERENCE][Static]` |
| `INV-RUNOFFPART-026` | governance | HPHYS0298 runoff-consumer partition ledger tying closed `Q` and WB12/WB14/WB13 identity to the ordered first-divergent cut-point verdict | Explicit `HOLD` if closed downstream identity is used as acceptance or compensation without paired first-divergence evidence | HPHYS0298 paired snow/`RM` lineage partition gate | `[DIRECT][Static] + [INFERENCE][Static]` |
| `INV-RUNOFFPART-027` | runtime | WB12/WB14 same-pass infiltration cap using top-two-layer `wb18_perc_theta` vs `wb18_perc_ul` storage availability | Typed hard error on malformed storage symbols; rainfall-excess residual instead of over-infiltration when storage availability is exhausted | FQ3-DC runoff underproduction closure gate | `[DIRECT][Static] + [INFERENCE][Static]` |

## Symbol Alias Map

Canonical symbols in this contract use Chapter-4 WEPP notation. EROD11
ratifies Wave-0 erosion-lane alias ownership for required runoff/peak-duration
surfaces while preserving canonical identity aliases for not-yet-implemented
runoff partition internals.

| Canonical symbol | Boundary/API name | Scope | Units check | Evidence |
|---|---|---|---|---|
| `R`, `Ri`, `r`, `ri` | identity names | climate-to-runoff forcing surfaces | `m` / `m s^-1` preserved | `[DIRECT][Static]` |
| `F`, `Fi`, `fi` | identity names | infiltration and partition surfaces | `m` / `m s^-1` preserved | `[DIRECT][Static]` |
| `V`, `Vi`, `vi` | identity names | rainfall-excess/routing surfaces | `m` / `m s^-1` preserved | `[DIRECT][Static]` |
| `Ke`, `Ψ`, `θd`, `Sp` | identity names | infiltration branch parameter/state surfaces | chapter-declared units preserved | `[DIRECT][Static]` |
| `Sd`, `rr`, `So` | identity names | depression-storage surfaces | chapter-declared units preserved | `[DIRECT][Static]` |
| `Qv`, `qp` | identity names | routed-runoff and peak-runoff outputs | `m` and `m^2 s^-1` preserved | `[DIRECT][Static]` |
| `Q` | `HillslopeProductionFluxSymbol::Wb12RunoffQ -> Q` | runoff-depth handoff to daily water-balance and erosion consumers | `m` preserved | `[DIRECT][Static] + [INFERENCE][Static]` |
| `wb20_forward_solver_lane_enabled` | `wb20_forward_solver_lane_enabled` | WB20 runoff closure lane selector (`1` forward-solver, `0`/absent compatibility) for WB12 closure-delta semantics | scalar in `{0,1}` preserved | `[INFERENCE][Static]` |
| `peakro`, `watdur` | `HillslopeProductionStateSymbol::{Wb16Peakro,Wb16Watdur}` | WB16 peak-runoff/duration state aliases exported for erosion and routing intake | `m^3 s^-1`, `s` preserved | `[DIRECT][Static] + [INFERENCE][Static]` |
| `wb16_peak_method_branch`, `wb16_tstar`, `wb16_qpstar`, `wb16_vstar` | `HillslopeProductionStateSymbol::{Wb16MethodBranch,Wb16Tstar,Wb16Qpstar,Wb16Vstar}` | WB16 branch-traceability surfaces required by downstream contract diagnostics | branch metadata + scalar continuity diagnostics preserved | `[DIRECT][Static] + [INFERENCE][Static]` |
| `De` | identity name | effective-runoff-duration coupling surface | `s` preserved | `[DIRECT][Static]` |
| `Qj-1`, `Vj`, `Qj` | identity names | multi-OFE case-classification surfaces | `m` preserved | `[DIRECT][Static]` |
| `ui_SUrunf(ii)` | `ui_SUrunf_{hour:04}` state symbol | MOFE hourly upstream saturation-runoff carry into current-OFE runon closure | `m` preserved; `hour=1..24` | `[DIRECT][Static] + [INFERENCE][Static]` |
| `ui_LfUrf(ii)` | `ui_LfUrf_{hour:04}` state symbol | MOFE hourly upstream lateral-flow carry into current-OFE runon closure | `m` preserved; `hour=1..24` | `[DIRECT][Static] + [INFERENCE][Static]` |
| `ui_SCrunf(ii)` | `ui_SCrunf_{hour:04}` state symbol | MOFE hourly current saturation-runoff carry output for downstream copy-forward | `m` preserved; `hour=1..24` | `[DIRECT][Static] + [INFERENCE][Static]` |
| `surdra = Σui_SCrunf(ii)` | `Σ ui_SCrunf_{hour:04}` plus `Q` addback | current-OFE hourly saturation excess that must be included in daily WB12/WB14 runoff publication | `m` preserved; 24-hour sum contributes to daily `Q` | `[DIRECT][Static] + [INFERENCE][Static]` |
| `ui_LfCrf(ii)` | `ui_LfCrf_{hour:04}` state symbol | MOFE hourly current lateral-flow carry output for downstream copy-forward | `m` preserved; `hour=1..24` | `[DIRECT][Static] + [INFERENCE][Static]` |
| `Ka`, `(Ψθd)a`, `Sa`, `Fp`, `Fh` | identity names | multi-OFE averaging and runon/runoff branch surfaces | chapter-declared units preserved | `[DIRECT][Static]` |
| `α`, `αe`, `m` | identity names | routing/equivalent-plane coefficient surfaces | coefficient/exponent semantics preserved | `[DIRECT][Static]` |
| `Q` (legacy canonical alias continuity) | identity name | canonical WEPP symbol continuity row retained for contract lineage | `m` preserved | `[DIRECT][Static]` |

## EROD11 Alias Ownership Register

| Boundary ID | Canonical symbols | Runtime alias surface | Producer ownership | Consumer ownership | Evidence |
|---|---|---|---|---|---|
| `EROD-BND-001` | `Q`, `peakro`, `watdur`, `wb16_peak_method_branch`, `wb16_tstar`, `wb16_qpstar`, `wb16_vstar` | `HillslopeProductionFluxSymbol::Wb12RunoffQ`; `HillslopeProductionStateSymbol::{Wb16Peakro,Wb16Watdur,Wb16MethodBranch,Wb16Tstar,Wb16Qpstar,Wb16Vstar}` | `SC-RUNOFFPART-001` via WB12/WB16 outputs | `SC-SED-001`, `SC-HYDRAULICS-001`, `SC-ROUTE-001`, `SC-WATBAL-001` | `[DIRECT][Static] + [INFERENCE][Static]` |
| `EROD-BND-006` | `Qj-1`, `Vj`, `Qj`, `Ka`, `(Ψθd)a`, `Sa`, `Fp`, `Fh` | canonical identity boundary symbols (runtime projection owner deferred under erosion-physics `HOLD`) | `SC-RUNOFFPART-001` | downstream OFE cascade and erosion-coupled routing lanes | `[DIRECT][Static] + [INFERENCE][Static]` |

## EROD12 Cross-Domain Ownership and Guard Closure Addendum

| Cross-domain lane | Producer ownership | Consumer guard ownership | Closure posture | Evidence |
|---|---|---|---|---|
| Runoff and peak-duration export (`Q`, `peakro`, `watdur`, `wb16_*`) | `SC-RUNOFFPART-001` (`INV-RUNOFFPART-009`, `INV-RUNOFFPART-011`) + `SC-WATBAL-001` (`INV-WATBAL-007`, `INV-WATBAL-016`) | `SC-SED-001` (`INV-SED-004`), `SC-HYDRAULICS-001` (`INV-HYDRAULICS-008`, `INV-HYDRAULICS-011`), `SC-ROUTE-001` (`INV-ROUTE-011`) | Required Wave-0 producer/consumer guard ownership is explicit and canonicalized. | `[DIRECT][Static] + [INFERENCE][Static]` |
| Multi-OFE downstream handoff (`Qj-1`, `Vj`, `Qj`, `Ka`, `(Ψθd)a`, `Sa`, `Fp`, `Fh`) | `SC-RUNOFFPART-001` (`INV-RUNOFFPART-007`..`009`) | downstream erosion/routing consumers (`SC-SED-001`, `SC-ROUTE-001`) | Cross-domain ownership semantics are explicit for required erosion-lane OFE boundary surfaces. | `[DIRECT][Static] + [INFERENCE][Static]` |

## Allowed Degenerate States

| Degenerate state | Allowed condition | Rationale |
|---|---|---|
| No-ponding interval | `Cu < 0` with continuing infiltration updates and no rainfall-excess increase for the interval. | Explicit §4.2 branch behavior. |
| Ponding then cessation | `Cp` transitions from `>0` to `<0` within event intervals with explicit branch update. | Explicit §4.2 ponding-cease logic. |
| Small event with zero runoff | Rainfall excess remains below potential depression storage so final runoff equals zero. | Explicit §4.3 depression-storage behavior. |
| Partial-equilibrium hydrograph | Routed runoff adjusted for recession infiltration by Eq. [4.4.27]-[4.4.29]. | Explicit §4.4.3 correction behavior. |
| Multi-OFE case four | `Fh - Fp <= 0` and downstream OFE emits zero runoff despite runon. | Explicit §4.5 branch case. |

## Multi-OFE Normative Branch Outcomes

| Case | Condition | Required outcome |
|---|---|---|
| Case 1 | `Qj-1 = 0`, `Vj = 0` | `Qj = 0` |
| Case 2 | `Qj-1 > 0`, `Vj > 0` | `Qj > 0` and rainfall-excess/depression adjustments proceed using averaged-parameter branch semantics. |
| Case 3 | `Qj-1 > 0`, `Vj = 0`, `Fh - Fp > 0` | `Qj > 0` with runoff depth assembled from Eq. [4.5.8]-[4.5.9]. |
| Case 4 | `Qj-1 > 0`, `Vj = 0`, `Fh - Fp <= 0` | `Qj = 0` with explicit case-four classification retained. |

## Invalid States

- Event partition residual exceeding declared tolerance without typed failure. `[DIRECT][Static] + [INFERENCE][Static]`
- Negative cumulative rainfall/infiltration/rainfall-excess/runoff depths beyond tolerance allowances. `[DIRECT][Static] + [INFERENCE][Static]`
- Runoff routing emitted before depression-storage branch condition allows onset. `[DIRECT][Static] + [INFERENCE][Static]`
- Invalid/implicit ponding transition handling (missing explicit `Cu`/`Cp` branch semantics). `[DIRECT][Static] + [INFERENCE][Static]`
- `Qv > 0` with non-positive/undefined `qp` or undefined `De` continuity relation. `[DIRECT][Static] + [INFERENCE][Static]`
- Multi-OFE state with unresolved case classification or inconsistent case-three/case-four decision. `[DIRECT][Static]`
- Boundary payload missing required `Q`/`Qv`/`qp` surfaces or unit/sign contract mismatch. `[DIRECT][Static] + [INFERENCE][Static]`

## Producer Obligations

- OBL-RUNOFFPART-P-001: Emit infiltration, rainfall-excess, runoff-volume, and peak-rate surfaces with units declared in this contract. `[DIRECT][Static] + [INFERENCE][Static]`
- OBL-RUNOFFPART-P-002: Evaluate and retain event partition residuals and enforce hard-fail behavior on tolerance violation. `[INFERENCE][Static]`
- OBL-RUNOFFPART-P-003: Apply explicit branch logic for Eq. [4.3.1], depression-storage adjustments, and multi-OFE case classification. `[DIRECT][Static] + [INFERENCE][Static]`
- OBL-RUNOFFPART-P-004: Propagate invariant violations as typed errors; no silent defaulting/clamping of hydrologic terms. `[INFERENCE][Static]`
- OBL-RUNOFFPART-P-005: Include routed snowmelt in the event liquid supply before evaluating infiltration capacity and runoff residuals; publish `wb12_infiltration`, `Q`, and closure diagnostics from the same supply so WB18 can apply the same infiltrated liquid to layer storage. `[DIRECT][Static] + [INFERENCE][Static]`
- OBL-RUNOFFPART-P-006: Validate runtime snow-state domains before inactive snow fallback in WB12/WB14 partition paths; domain-invalid snow state must not be converted to zero-state continuation. `[DIRECT][Static] + [INFERENCE][Static]`
- OBL-RUNOFFPART-P-007: Route residual rain-on-snow released from snowpack holding-capacity accounting through the `wmelt` event-forcing path and remove it from direct-rain hyetograph forcing to avoid omission or double counting. `[DIRECT][Static] + [INFERENCE][Static]`
- OBL-RUNOFFPART-P-008: For spring H1/H7/H39 diagnostics, publish enough WB12/WB14 capacity evidence to distinguish routed-melt supply from infiltration limitation: effective conductivity, matric-potential proxy inputs, routed melt, cumulative infiltration, and residual `Q`. `[DIRECT][Static] + [INFERENCE][Static]`
- OBL-RUNOFFPART-P-009: Preserve HPHYS0292 capacity evidence in HPHYS0293 traces so snow producer residuals can be excluded from WB14 only when `wb12_infiltration`, event-liquid supply, and residual `Q` remain closed. `[DIRECT][Static] + [INFERENCE][Static]`
- OBL-RUNOFFPART-P-010: For HPHYS0296 acceptance, preserve `RM` producer-consumer identity evidence showing that WB12/WB14 liquid forcing and WB13 `RM` publication consume the same `snow.routed_melt_m` and `snow.post_winter_rain_m` surfaces, with residual `Q` closed, and require the `SC-SNOWFREEZE-001#INV-SNOWFREEZE-027` per-window defective-model verdict before any snow/`RM` residual leaves the failing set; closed runoff identity alone only excludes runoff/storage compensation. `[DIRECT][Static] + [INFERENCE][Static]`
- OBL-RUNOFFPART-P-011: For HPHYS0297, preserve closed residual `Q`, WB12/WB14 liquid forcing, and WB13 `RM` publication identity alongside the snow/`RM` reconstruction ledger so runoff remains excluded only as evidence, never as a downstream compensation path. `[DIRECT][Static] + [INFERENCE][Static]`
- OBL-RUNOFFPART-P-012: For HPHYS0298, publish runoff-consumer identity evidence for each target window in the paired partition ledger and preserve `Q`/WB12/WB14/WB13 closure only as first-divergence context, not as snow/`RM` acceptance or downstream compensation authority. `[DIRECT][Static] + [INFERENCE][Static]`

## Consumer Obligations

- OBL-RUNOFFPART-C-001: Water-balance consumers must ingest runoff depth `Q` with Chapter-5 sign/unit semantics unchanged. `[DIRECT][Static] + [INFERENCE][Static]`
- OBL-RUNOFFPART-C-002: Erosion/hydraulics consumers must treat `Qv`, `qp`, and `De` as coupled outputs and reject malformed/undefined combinations. `[DIRECT][Static] + [INFERENCE][Static]`
- OBL-RUNOFFPART-C-003: OFE-downstream consumers must preserve declared case classifications and avoid implicit fallback between runon/runoff cases. `[DIRECT][Static] + [INFERENCE][Static]`
- OBL-RUNOFFPART-C-004: All consumers must fail explicitly on invariant-violating payloads and carry invariant IDs in error context. `[INFERENCE][Static]`

## Boundary Disposition

| Invariant family | Detection point | Disposition | Comparator tier impact | Evidence |
|---|---|---|---|---|
| Event closure and rainfall-excess branch semantics (`INV-RUNOFFPART-001/002`) | event partition assembly | Hard error; reject event output | Tier-A gate | `[DIRECT][Static] + [INFERENCE][Static]` |
| Depression-storage and infiltration transition semantics (`INV-RUNOFFPART-003/004`) | post-infiltration/pre-routing stage | Hard error on onset/transition violation | Tier-A gate | `[DIRECT][Static] + [INFERENCE][Static]` |
| Peak/duration and recession adjustment semantics (`INV-RUNOFFPART-005/006`) | peak/routing post-processing | Hard error on continuity/domain failure | Tier-B investigation gate | `[DIRECT][Static] + [INFERENCE][Static]` |
| Multi-OFE branching and aggregation semantics (`INV-RUNOFFPART-007/008`) | OFE cascade evaluator | Hard error on branch/aggregation domain violation | Tier-A/B gate | `[DIRECT][Static] + [INFERENCE][Static]` |
| Boundary coupling completeness (`INV-RUNOFFPART-009`) | cross-domain handoff | Hard error on missing malformed boundary field | Tier-A gate | `[DIRECT][Static] + [INFERENCE][Static]` |
| Hortonian-scope governance (`INV-RUNOFFPART-010`) | review/verification/promotion | Governance `HOLD` until scope label is explicit | Governance gate | `[DIRECT][Static] + [INFERENCE][Static]` |
| WB20 forward-solver lane closure semantics (`INV-RUNOFFPART-011`) | WB12 runoff closure-delta lane boundary | Hard error when forward lane consumes observed target for acceptance or emits non-residual closure delta | Tier-A parity lane gate | `[DIRECT][Static] + [INFERENCE][Static]` |

## Tolerance and Numeric Notes

This contract follows `docs/numerics/README.md` (semantic parity, not
bit-for-bit parity). Contract-level tolerance declarations:

| Tolerance ID | Definition | Value | Notes |
|---|---|---|---|
| TOL-RUNOFFPART-001 | Event closure residual tolerance for `INV-RUNOFFPART-001` | `<= 1e-9 m` | Residual is explicitly computed per event accounting boundary. |
| TOL-RUNOFFPART-002 | Non-negative-domain tolerance for depth terms (`R`, `F`, `V`, `Qv`) | lower bound `>= -1e-12 m` | Comparator-noise allowance only; runtime still hard-fails on material negatives. |
| TOL-RUNOFFPART-003 | Non-negative-domain tolerance for infiltration/rainfall-excess rates (`fi`, `vi`) | lower bound `>= -1e-12 m s^-1` | Comparator-noise allowance only; runtime still hard-fails on material negatives. |
| TOL-RUNOFFPART-004 | Non-negative-domain tolerance for peak runoff rate (`qp`) | lower bound `>= -1e-12 m^2 s^-1` | No silent clamping in runtime path. |
| TOL-RUNOFFPART-005 | Multi-OFE branch threshold tolerance around `Fh - Fp` case boundary | `abs(Fh - Fp) <= 1e-12 m^3 m^-1` treated as case-four boundary | Prevents numerical jitter from toggling case-three/case-four branch outcomes. |
| TOL-RUNOFFPART-006 | WB12/WB14 reconciled runoff near-zero canonicalization tolerance (`Q`, `wb12_runoff_reconciled`) | normalize to `0` when `-1e-12 m <= value < 0` before writeback/publication; `value < -1e-12 m` is domain-invalid | Explicit roundoff canonicalization only; not a fallback for material negative runoff. |
| TOL-RUNOFFPART-007 | WB14 interval infiltration upper-bound roundoff tolerance | `ΔFj <= Rj + 1e-9 m`; accepted values are normalized to `min(ΔFj, Rj)` before accumulation | Bounded floating-point allowance for Green-Ampt interval solves at the ponding/no-ponding boundary; material over-infiltration remains invalid. |

## WB12 Runoff Reconciliation Addendum

### WB12 Reconciliation Inputs/Outputs

| Surface | Symbols |
|---|---|
| WB12 runoff reconciliation required/carryover inputs | `wb12_rainfall_input`, `wb12_runon_input`, same-pass flux `wb12_runoff_carryover` when published, `wb12_infiltration`, `wb12_depression_storage_delta`, `wb12_runoff_closure_tolerance` |
| WB20 lane selector | `wb20_forward_solver_lane_enabled` (`1` forward-solver lane, `0` or absent compatibility lane) |
| Compatibility-lane observed target | `wb12_runoff_observed` |
| WB12 runoff reconciliation outputs | `Q`, `wb12_runoff_carryover`, `wb12_runoff_reconciled`, `wb12_runoff_closure_delta` |

### WB12 Reconciliation Rule

Runoff reconciliation publishes:
- `runoff_carryover = wb12_runoff_carryover` when the same-pass flux is
  present and finite/non-negative; otherwise `runoff_carryover =
  wb12_runon_input` when the state surface is finite/non-negative.
- `Q = wb12_rainfall_input + runoff_carryover - wb12_infiltration - wb12_depression_storage_delta`
- apply explicit near-zero canonicalization before writeback/closure-delta
  publication: if `Q` is in `[-1e-12, 0)`, set `Q = 0` and
  `wb12_runoff_reconciled = 0`; `Q < -1e-12` is a domain violation.
- publish resolved `runoff_carryover` as same-pass flux
  `wb12_runoff_carryover`.
- forward-solver lane (`wb20_forward_solver_lane_enabled = 1`):
  - `wb12_runoff_closure_delta = (wb12_rainfall_input + runoff_carryover - wb12_infiltration - wb12_depression_storage_delta) - Q`
  - observed targets are excluded from acceptance-driving inputs.
- compatibility lane (`wb20_forward_solver_lane_enabled = 0` or symbol absent):
  - `wb12_runoff_closure_delta = Q - wb12_runoff_observed`

Closure delta beyond `wb12_runoff_closure_tolerance` is an invalid closure state.

### WB12 Typed Guard Codes

| Condition | Code |
|---|---|
| Missing required symbol | `HKERNEL-WB12-RUNOFF-E-001` |
| Non-finite required symbol | `HKERNEL-WB12-RUNOFF-E-002` |
| Domain/closure violation | `HKERNEL-WB12-RUNOFF-E-003` |

### WB12 Contract-Test Vectors

1. Valid WB12 runoff inputs emit deterministic `Q` and reconciliation diagnostics.
2. Non-finite WB12 runoff input hard-fails with `HKERNEL-WB12-RUNOFF-E-002`.
3. Forward-solver lane vectors with perturbed `wb12_runoff_observed` still emit solver-residual closure deltas and remain acceptance-valid when required inputs are valid.
4. Compatibility-lane closure-delta overflow beyond tolerance hard-fails with `HKERNEL-WB12-RUNOFF-E-003` and no writeback mutation.

## WB13 Daily Output Coupling Addendum

### WB13 Runoff/Runon Output Symbols

| WB13 column | Runoff-partition coupling surface | Units |
|---|---|---|
| `Q` | Daily runoff depth exported to Chapter-5 closure boundary | `mm` |
| `QOFE` | Single-OFE runoff output alias (`QOFE = Q`) | `mm` |
| `UpStrmQ` | Upstream runon contribution added to OFE | `mm` |
| `RM` | Rainfall + irrigation + snowmelt daily input depth for runoff accounting | `mm` |
| `P` | Daily precipitation contribution included in runoff/accounting surfaces | `mm` |

### WB13 Coupling Requirements

1. WB13 daily rows must carry finite, non-negative runoff/runon symbols.
2. `QOFE` must be equal to `Q` for single-OFE WB13 rows.
3. Missing/non-finite/out-of-domain runoff/runon symbols are invalid WB13
   output states and must hard-fail with WB13 typed guard posture.

## WB14 Infiltration and Subdaily Hyetograph Kernel Authority Addendum

### WB14 Required Surfaces

| Surface | Symbols |
|---|---|
| Hyetograph forcing inputs | `ninten` or `nbrkpt`; `timem_####`; `intsty_####` |
| Soil-derived infiltration inputs | `ssc`, `dg`, `thetdr`, `thetfc` |
| Disturbed-soil conductivity-adjustment inputs | `solwpv`, `ksatadj`, `ksatfac`, `ksatrec`, `lkeff`, `wb18_perc_theta_####`, `wb18_perc_fc_####`, `wb18_perc_ul_####`, `dg_####` |
| Runoff reconciliation inputs | `wb12_rainfall_input`, `wb12_runon_input`, same-pass flux `wb12_runoff_carryover` when published, `wb12_depression_storage_delta`, `wb12_runoff_closure_tolerance`, `wb20_forward_solver_lane_enabled` (`1` forward-solver lane, `0` or absent compatibility lane) |
| Compatibility-lane observed target input | `wb12_runoff_observed` |
| Runoff reconciliation outputs | `wb12_infiltration`, `Q`, `wb12_runoff_carryover`, `wb12_runoff_closure_delta`, `wb12_runoff_reconciled` |

### WB14 Deterministic Infiltration and Runoff Rules

1. Build event subdaily intervals from `timem_####` and `intsty_####` with
   strict time monotonicity and non-negative interval intensities.
2. Derive baseline-authoritative effective conductivity `Ke`:
   - default path: `Ke = ssc`;
   - disturbed path is gated by `ksatadj = 1`;
   - disturbed-path saturation fraction uses first-two-layer WB18 stores:
     `sat_frac = min((theta_1 + theta_2)/(ul_1 + ul_2), 1.0)`;
   - `solwpv = 9001`:
     - `keffu = ssc * 3.6e6`
     - `keffl = keffu / ksatfac`
     - `keff = ((keffu-keffl)/(exp(1/ksatrec)-1))*(exp(sat_frac/ksatrec)-1)+keffl`
     - `Ke = keff / 3.6e6`;
   - `solwpv >= 9002`:
     - top-two-layer weighted volumetric terms:
       - `avthetafc = (fc_1 + fc_2) / (dg_1 + dg_2)`
       - `avthetadr = ((ul_1-fc_1) + (ul_2-fc_2)) / (dg_1 + dg_2)`
     - `psi = ln(1500/33) / ln(avthetafc/avthetadr)`
     - `lambda = 1/psi`
     - `keff = (ssc * 3.6e6) * sat_frac^(2*lambda + 3)`
     - `solwpv = 9003` applies burn-severity floor when `lkeff > 0`:
       `keff = max(keff, lkeff)`
     - `Ke = keff / 3.6e6`.
3. Active disturbed-path domain violations (missing/non-finite/non-positive
   required regime symbols or invalid logarithmic-domain terms) are typed
   hard-fail states; no silent defaults/clamping are allowed.
4. Derive Green-Ampt lineage moisture parameters from runtime symbols:
   - `θd = thetfc - thetdr`
   - `Sm = dg * θd`
5. For each interval `j`, compute rainfall depth
   `Rj = intsty_j * (timem_{j+1} - timem_j)` and cumulative infiltration `F`:
   - if `intsty_j <= Ke`, interval infiltration increment is `Rj`;
   - if `intsty_j > Ke`, evaluate ponding threshold
     `Fp = (Ke * Sm) / (intsty_j - Ke)` and apply explicit branch:
     - no-ponding branch: rainfall-controlled increment when interval end
       cumulative infiltration remains below `Fp`;
     - ponded branch: solve the Green-Ampt implicit cumulative relation
       `(F - Fp) - Sm * ln((F + Sm)/(Fp + Sm)) = Ke * Δtp`
       for the ponded sub-interval duration `Δtp`.
6. Interval rainfall excess is `max(Rj - ΔFj, 0)` where `ΔFj` is interval
   infiltration increment.
7. Event totals are:
   - `wb14_hyetograph_rainfall = Σ Rj`
   - `wb12_infiltration = Σ ΔFj`
   - interval infiltration values may exceed `Rj` only within
     `TOL-RUNOFFPART-007` and are normalized to `Rj` before accumulation;
     larger over-infiltration remains a typed domain failure.
8. `wb14_hyetograph_rainfall` and `wb12_rainfall_input` must agree within
   `wb12_runoff_closure_tolerance`; mismatch is an invalid state.
9. Reconciled runoff is:
   - `runoff_carryover = wb12_runoff_carryover` when the same-pass flux is
     present and finite/non-negative; otherwise `runoff_carryover =
     wb12_runon_input` when the state surface is finite/non-negative.
   - `Q = wb14_hyetograph_rainfall + runoff_carryover - wb12_infiltration - wb12_depression_storage_delta`
   - apply explicit near-zero canonicalization before writeback/closure-delta
     publication: if `Q` is in `[-1e-12, 0)`, set `Q = 0` and
     `wb12_runoff_reconciled = 0`; `Q < -1e-12` is a domain violation.
   - publish resolved `runoff_carryover` as same-pass flux
     `wb12_runoff_carryover`.
   - forward-solver lane (`wb20_forward_solver_lane_enabled = 1`):
     - `wb12_runoff_closure_delta = (wb14_hyetograph_rainfall + runoff_carryover - wb12_infiltration - wb12_depression_storage_delta) - Q`
   - compatibility lane (`wb20_forward_solver_lane_enabled = 0` or symbol absent):
     - `wb12_runoff_closure_delta = Q - wb12_runoff_observed`
10. Missing/non-finite/out-of-domain symbols and closure violations hard-fail;
   no silent clamping/defaulting of hyetograph or infiltration terms is
   allowed.

### WB14 Typed Guard Codes

| Condition | Code |
|---|---|
| Missing required symbol | `HKERNEL-WB14-RUNOFF-E-001` |
| Non-finite required symbol | `HKERNEL-WB14-RUNOFF-E-002` |
| Domain/closure violation | `HKERNEL-WB14-RUNOFF-E-003` |

### WB14 Contract-Test Vectors

1. Valid hyetograph + soil infiltration symbols produce deterministic
   `wb12_infiltration`, `Q`, and closure diagnostics.
2. Missing required hyetograph or infiltration symbols hard-fails with
   `HKERNEL-WB14-RUNOFF-E-001`.
3. Non-finite hyetograph/infiltration/reconciliation symbols hard-fail with
   `HKERNEL-WB14-RUNOFF-E-002`.
4. Non-monotone hyetograph time, negative intensity, rainfall-mismatch, or
   runoff closure overflow hard-fail with `HKERNEL-WB14-RUNOFF-E-003`.
5. Active `ksatadj` regime vectors (`solwpv=9001/9002/9003`) must produce
   deterministic conductivity-adjusted infiltration behavior; active-regime
   domain violations hard-fail with typed `HKERNEL-WB14-RUNOFF-E-00x` posture.
6. Within-tolerance negative reconciled runoff (`-1e-12 <= Q < 0`) is
   canonicalized to zero at writeback/publication boundary; values below
   tolerance remain typed domain failures.
7. Same-pass `wb12_runoff_carryover` flux overrides stale
   `wb12_runon_input` in WB14 reconciliation and malformed present carryover
   fluxes hard-fail with typed non-finite/domain posture.
8. Material negative projected runtime snow state hard-fails before inactive
   snow fallback or liquid-partition assembly; bounded near-zero values may
   only be normalized after the non-negative-domain guard accepts tolerance.

## WB15 Canopy Interception Runtime Coupling Addendum

### WB15 Required Surfaces

| Surface | Symbols |
|---|---|
| Plant runtime interception inputs | `cancov`, `lai`, `vdmt` |
| Hyetograph forcing inputs | `ninten` or `nbrkpt`; `timem_####`; `intsty_####` |
| Runoff reconciliation outputs | `I`, `wb12_infiltration`, `Q`, `wb12_runoff_closure_delta`, `wb12_runoff_reconciled` |

### WB15 Deterministic Runoff Rule

1. Compute hyetograph rainfall first (`wb14_hyetograph_rainfall`) from subdaily
   forcing.
2. Compute canopy interception from plant runtime state using Eq. [5.1.2]
   lineage (Chapter-5 coupling) with biomass proxy
   `VE_raw = vdmt * 10000`, capped to `VE = min(VE_raw, 8000)` for
   interception-equation evaluation only:
   - `Ipot = cancov * ((0.000627 * VE - 3.73349e-8 * VE^2) / 1000)`
3. Apply interception-before-infiltration coupling:
   - `I = min(Ipot, wb14_hyetograph_rainfall)` for `lai > 0` and `cancov > 0`
   - `I = 0` for `lai <= 0` or `cancov <= 0`
4. Reconcile with intercepted liquid depth:
   - `wb14_hyetograph_liquid_after_interception = wb14_hyetograph_rainfall - I`
   - `Q = wb14_hyetograph_liquid_after_interception + S + runoff_carryover - wb12_infiltration - wb12_depression_storage_delta`
5. Canopy-state domain policy is hard-fail:
   - `0 <= cancov <= 0.999`
   - `lai >= 0`
   - `vdmt >= 0` (`kg m^-2`) and finite so `VE = vdmt * 10000` remains
     finite and non-negative
6. Missing/non-finite/out-of-domain canopy symbols are invalid runoff states.
   Silent defaults/clamps are prohibited.

### WB15 Typed Guard Codes

| Condition | Code |
|---|---|
| Missing required symbol | `HKERNEL-WB14-RUNOFF-E-001` |
| Non-finite required symbol | `HKERNEL-WB14-RUNOFF-E-002` |
| Domain/closure violation | `HKERNEL-WB14-RUNOFF-E-003` |

### WB15 Contract-Test Vectors

1. Nominal canopy-coupled vector emits finite `I` and deterministic coupled
   `wb12_infiltration` + `Q`.
2. Missing canopy interception symbol (`cancov`, `lai`, `vdmt`) hard-fails
   with `HKERNEL-WB14-RUNOFF-E-001`.
3. Non-finite canopy interception symbol hard-fails with
   `HKERNEL-WB14-RUNOFF-E-002`.
4. Out-of-domain canopy interception symbol (`cancov`, `lai`, negative/non-finite
   `vdmt`) or coupled runoff-closure overflow
   hard-fails with `HKERNEL-WB14-RUNOFF-E-003`.

## IRRIG10 Irrigation Runtime Coupling Addendum

### IRRIG10 Required Surfaces

| Surface | Symbols |
|---|---|
| Runtime irrigation scheduling traces | `irrigation.runtime_schedule_source`, `irrigation.runtime_depth_m`, `irrigation.runtime_duration_s`, `irrigation.runtime_rate_m_per_s` |
| Parser-projected irrigation schedules | `irrigation.depletion.*`, `irrigation.fixeddate.*` |
| Coupled runoff outputs | `Irr`, `Q`, `wb12_runoff_closure_delta`, `wb12_runoff_reconciled` |

### IRRIG10 Deterministic Runoff Rules

1. Runoff reconciliation consumes irrigation additions as explicit forcing
   depth term: `wb12_rainfall_input = wb14_hyetograph_rainfall + irrigation.runtime_depth_m`.
2. Irrigation forcing is applied through explicit schedule-source resolution
   (fixed-date priority, then depletion) and emitted as `Irr`.
3. Coupled runoff equation remains explicit under irrigation:
   - `Q = wb14_hyetograph_liquid_after_interception + S + runoff_carryover - wb12_infiltration - wb12_depression_storage_delta`
4. Missing/non-finite/out-of-domain irrigation scheduling payloads are invalid
   runoff states; no fallback/default branch is allowed.

### IRRIG10 Typed Guard Codes

| Condition | Code |
|---|---|
| Missing required symbol | `HKERNEL-WB14-RUNOFF-E-001` |
| Non-finite required symbol | `HKERNEL-WB14-RUNOFF-E-002` |
| Domain/closure violation | `HKERNEL-WB14-RUNOFF-E-003` |

### IRRIG10 Contract-Test Vectors

1. Matching fixed-date sprinkler event emits positive `Irr` and deterministic
   runoff closure outputs.
2. Matching depletion sprinkler period emits positive `Irr` with deterministic
   runoff closure outputs.
3. Missing irrigation scheduling key symbols hard-fail with
   `HKERNEL-WB14-RUNOFF-E-001`.
4. Non-finite/out-of-domain irrigation scheduling payloads hard-fail with
   `HKERNEL-WB14-RUNOFF-E-002/003`.

## CLIM05 Snow Runtime Coupling Addendum

### CLIM05 Required Surfaces

| Surface | Symbols |
|---|---|
| Parsed snow controls | `snow.options.rst`, `snow.options.newsnw`, `snow.options.ssd`, `snow.options.snow_file_present` |
| Runtime snow state/output | `snow.runtime_swe`, `S` |
| Climate partition drivers | `Tmax`, `Tmin`, `timem_####`, `intsty_####` |
| Runoff reconciliation outputs | `Q`, `wb12_infiltration`, `wb12_runoff_closure_delta`, `wb12_runoff_reconciled` |

### CLIM05 Deterministic Runoff Rule

1. When active snow coupling controls are projected (`snow.options.snow_file_present`),
   runoff reconciliation uses signed snow term `S = melt - accumulation`.
2. Snow-coupled liquid input depth is:
   - `wb14_liquid_input = wb14_hyetograph_rainfall + S`
3. Reconciled runoff becomes:
   - `Q = wb14_liquid_input + runoff_carryover - wb12_infiltration - wb12_depression_storage_delta`
4. Active-coupling missing/non-finite/domain-invalid `snow.options.*` controls,
   `S`, or `snow.runtime_swe` are hard-fail runoff states; no fallback/default
   branch is allowed.

### CLIM05 Typed Guard Codes

| Condition | Code |
|---|---|
| Missing required symbol | `HKERNEL-WB14-RUNOFF-E-001` |
| Non-finite required symbol | `HKERNEL-WB14-RUNOFF-E-002` |
| Domain/closure violation | `HKERNEL-WB14-RUNOFF-E-003` |

### CLIM05 Contract-Test Vectors

1. Active-coupling nominal vector changes reconciled `Q` according to signed
   `S` while preserving typed closure diagnostics.
2. Missing required active-coupling snow control symbol hard-fails with
   `HKERNEL-WB14-RUNOFF-E-001`.
3. Non-finite/out-of-domain active-coupling snow control/state hard-fails with
   `HKERNEL-WB14-RUNOFF-E-002/003`.

## CLIM06 Frozen-Soil Runtime Coupling Addendum

### CLIM06 Required Surfaces

| Surface | Symbols |
|---|---|
| Parsed frost controls | `frost.options.wintRed`, `frost.options.fineTop`, `frost.options.fineBot`, `frost.options.ksnowf`, `frost.options.kresf`, `frost.options.ksoilf`, `frost.options.kfactor1`, `frost.options.kfactor2`, `frost.options.kfactor3`, `frost.options.frost_file_present` |
| Frozen-state runtime outputs | `frost.runtime_dfrost`, `frost.runtime_dthaw`, `frost.runtime_nft`, `frost.runtime_ws_frz`, `frost.runtime_infcap_frz` |
| WB14 runoff reconciliation surfaces | `wb12_infiltration`, `Q`, `wb12_runoff_closure_delta`, `wb12_runoff_reconciled` |

### CLIM06 Deterministic Runoff Rule

1. Active CLIM06 coupling is explicit when
   `frost.options.frost_file_present = 1` and `frost.options.wintRed = 1`.
2. Runoff reconciliation must consume frozen-soil effective infiltration
   capacity from `frost.runtime_infcap_frz` when active CLIM06 coupling is
   enabled.
3. CLIM06 frozen-state domains are bounded and non-negative:
   - `0 <= frost.runtime_dfrost <= 0.20`
   - `0 <= frost.runtime_dthaw <= 0.20`
   - `frost.runtime_nft >= 0`
   - `frost.runtime_ws_frz >= 0`
   - `0 <= frost.runtime_infcap_frz <= ssc`
4. Active-coupling missing/non-finite/out-of-domain frost controls or
   frozen-state runtime symbols are hard-fail runoff states; no fallback/default
   branch is allowed.

### CLIM06 Typed Guard Codes

| Condition | Code |
|---|---|
| Missing required symbol | `HKERNEL-WB14-RUNOFF-E-001` |
| Non-finite required symbol | `HKERNEL-WB14-RUNOFF-E-002` |
| Domain/closure violation | `HKERNEL-WB14-RUNOFF-E-003` |

### CLIM06 Contract-Test Vectors

1. Active-coupling nominal vector updates `frost.runtime_*` symbols,
   reduces infiltration-capacity with `frost.runtime_infcap_frz`, and emits
   deterministic runoff reconciliation closure.
2. Missing required active-coupling frost symbol hard-fails with
   `HKERNEL-WB14-RUNOFF-E-001`.
3. Non-finite active-coupling frost symbol hard-fails with
   `HKERNEL-WB14-RUNOFF-E-002`.
4. Out-of-domain active-coupling frost control/state hard-fails with
   `HKERNEL-WB14-RUNOFF-E-003`.

## WB16 Peak-Runoff Kernel Addendum

### WB16 Required Surfaces

| Surface | Symbols |
|---|---|
| Closure-diagnostics runoff inputs | `Q`, `timem_####`, `intsty_####`, `Irr`, `I` |
| Peak-branch parameters | `efflen`, `ealpha`, `m` |
| WB16 outputs | `peakro`, `watdur`, `wb16_peak_method_branch`, `wb16_tstar`, `wb16_qpstar`, `wb16_vstar` |
| WB16 provenance outputs | `wb16_ealpha_compatibility_seed_used`, `wb16_ealpha_seed_policy` |

### WB16 Deterministic Peak-Runoff Rule

1. WB16 peak runoff consumes accepted WB14/WB15/IRRIG10-coupled runoff depth
   `Q` at closure diagnostics.
2. Baseline-authoritative near-zero runoff branch from
   `/workdir/wepp-forest_260430_baseline/src/appmth.for` applies first:
   - if `Q < 1.0e-8`, emit `peakro_raw = 0`, then canonicalize
     `peakro = 3.63e-8` and `watdur = 0`.
3. Event duration is derived from hyetograph elapsed time:
   - `effdrr = timem_last - timem_first`.
4. Mean runoff and maximum-rate terms are:
   - `vave = Q / effdrr`
   - `remax = max(intsty_####) + irrigation.runtime_rate_m_per_s`
   - `vstar = vave / remax`
5. Time-ratio branch terms use Chapter-4 lineage:
   - `te = (efflen / (ealpha * vave^(m-1)))^(1/m)`
   - `tstar = te / effdrr`
   - if `vstar < 1`,
     `tc = (1 - sqrt(1 - 2.4 * (1 - vstar) * vstar)) / (1.2 * (1 - vstar))`
6. Branch authority is deterministic:
   - `tstar >= 1`: `qpstar = 1 / tstar^m`
   - `vstar < 1` and `tc < tstar < 1`: `qpstar = 1 / tstar`
   - `vstar < 1` and `0 < tstar <= tc`:
     `qpstar = 1/vstar - 0.6 * ((1 - vstar) / vstar) * tstar`
   - `vstar >= 1` and `tstar < 1`: `qpstar = 1`
7. Peak/runoff-duration outputs are:
   - `peakro_raw = vave * qpstar`
   - `peakro = max(peakro_raw, 3.63e-8)`
   - `watdur = min(Q / peakro, 86400)`
8. Missing/non-finite/out-of-domain WB16 symbol/intermediate states are
   hard-fail and must not silently default/branch-repair.
9. Domain-invalid means non-finite values or non-positive required branch
   denominators (`effdrr <= 0`, `remax <= 0`, `vave <= 0`, `vstar <= 0`,
   `m <= 0`, `ealpha <= 0`, `efflen <= 0`); positive near-zero magnitudes are
   valid and must not hard-fail solely due epsilon thresholds.
10. `m` producer authority is baseline-canonical and constant:
    `/workdir/wepp-forest_260430_baseline/src/rdat.for` assigns `m = 1.5`
    (Chezy depth-discharge exponent) and runtime producers must preserve that
    canonical value unless a future canonical contract amendment supersedes it.
11. `ealpha` producer authority is baseline-canonical as a chain:
    `frcfac -> rdat(alpha) -> alphay -> eplane(optional multi-OFE projection)`
    (`/workdir/wepp-forest_260430_baseline/src/frcfac.for`,
    `rdat.for`, `irs.for`, `eplane.for`).
12. Runtime lanes with complete producer inputs must publish baseline-lineage
    `ealpha` from the authoritative producer chain with explicit provenance:
    - `wb16_ealpha_compatibility_seed_used = false`
    - `wb16_ealpha_seed_policy = "runtime_provided"`
13. Compatibility seeding (`ealpha = 1.0`) is allowed only as a typed
    degradation branch when required producer inputs are unavailable, and only
    when runtime emits explicit provenance:
    - `wb16_ealpha_compatibility_seed_used = true`
    - `wb16_ealpha_seed_policy = "compatibility_seed_1p0"`
    - warning text containing `SIMPIPE-W-003`
    Compatibility-seed runs are non-promotable for full WB16 input-provenance
    parity closure.

### WB16 Typed Guard Codes

| Condition | Code |
|---|---|
| Missing required symbol | `HKERNEL-WB16-PEAK-E-001` |
| Non-finite required symbol | `HKERNEL-WB16-PEAK-E-002` |
| Domain/closure violation | `HKERNEL-WB16-PEAK-E-003` |

### WB16 Contract-Test Vectors

1. Nominal WB16 vector emits finite `peakro` and `watdur` with continuity
   `watdur = Q/peakro`.
2. Method-branch vectors trigger each WB16 branch (`tstar >= 1`,
   `tc < tstar < 1`, `0 < tstar <= tc`) deterministically.
3. Missing WB16 required symbol hard-fails with `HKERNEL-WB16-PEAK-E-001`.
4. Non-finite WB16 required symbol hard-fails with `HKERNEL-WB16-PEAK-E-002`.
5. Domain-invalid WB16 symbol/intermediate hard-fails with
   `HKERNEL-WB16-PEAK-E-003`.
6. Near-zero positive runoff vector (`0 < Q < 1.0e-8`) executes the
   baseline-authoritative branch, emits `peakro = 3.63e-8`, `watdur = 0`,
   and does not hard-fail.
7. Runtime-producer provenance vector: when required producer symbols are
   available, runtime emits
   `wb16_ealpha_compatibility_seed_used = false`,
   `wb16_ealpha_seed_policy = "runtime_provided"`, and no `SIMPIPE-W-003`
   warning.
8. Compatibility-seed provenance vector: when `ealpha` is not runtime-produced
   and compatibility seeding is invoked, runtime emits
   `wb16_ealpha_compatibility_seed_used = true`,
   `wb16_ealpha_seed_policy = "compatibility_seed_1p0"`, and warning id
   `SIMPIPE-W-003`.

## ARCH22 Typed Production-Surface Addendum

### Typed Runtime Surface Authority

1. Covered production runoff-partition interfaces must use typed ARCH22 symbol
   surfaces (`HillslopeProductionStateSymbol`, `HillslopeProductionFluxSymbol`)
   for boundary-state and boundary-flux resolution.
2. Covered production guard/accessor helper signatures must not accept raw
   `&str` symbol identifiers where typed ARCH22 symbols exist.
3. Typed migration must preserve WB14/WB15/WB16 runoff guard families and
   failure behavior for missing/non-finite/domain-invalid payloads.

### Contract-Derived Migration Vectors

1. Static migration proof: covered runoff-partition production accessors use
   typed symbol families, not stringly `&str` parameters.
2. Nominal migration vector: runoff reconciliation + peak-runoff lanes preserve
   deterministic outputs under typed symbol resolution.
3. Failure migration vectors: existing typed hard-fail boundary classes and
   guard IDs remain unchanged.

## EROD13 Wave-1 Active Producer-Coupling Addendum

1. When `erod13_core_enabled = 1`, runoff producer surfaces
   (`Q`, `peakro`, `watdur`, `wb16_peak_method_branch`, `wb16_tstar`,
   `wb16_qpstar`, `wb16_vstar`) are mandatory for Wave-1 erosion-core
   consumer execution.
2. Producer ownership remains in `SC-RUNOFFPART-001` with continuity across
   WB12/WB16 authorities; consumer-side typed guard ownership is enforced in
   `SC-SED-001` through `HKERNEL-EROD13-CORE-E-001..003`.
3. Missing/non-finite/out-of-domain runoff coupling symbols on the enabled
   Wave-1 path must hard-fail; fallback synthesis of peak/duration or runoff
   inputs is prohibited.

## EROD14 Wave-2 Active Producer-Coupling Addendum

1. When `erod14_wave2_enabled = 1`, runoff-partition producer surfaces needed
   for multi-OFE routing-classification and enrichment transitions are
   mandatory:
   - `erod14_Qj_minus_1`, `erod14_Vj`, `erod14_Qj`, `erod14_Fh`,
     `erod14_Fp`, `erod14_case`,
   - `erod14_qout`, `erod14_qin`, `erod14_qostar`, `erod14_slplen`.
2. Producer ownership remains in `SC-RUNOFFPART-001` with continuity from
   `INV-RUNOFFPART-007..009`; consumer guard ownership is enforced in
   `SC-SED-001` through `HKERNEL-EROD14-WAVE2-E-001..003`.
3. Enabled-path missing/non-finite/domain-invalid multi-OFE producer symbols
   must hard-fail; implicit case fallback (including case-three/case-four
   collapse) is prohibited.
4. Wave-2 activation does not weaken existing Wave-1 producer-coupling guard
   obligations.

## HPHYS0240 Hourly Runoff Carryover Addendum

1. WB12/WB14 runoff reconciliation resolves incoming carryover from same-pass
   `wb12_runoff_carryover` flux when present and uses `wb12_runon_input` only
   as a finite non-negative compatibility surface when the flux is absent.
2. The resolved carryover participates in `Q` and closure-delta equations and
   must be republished as `wb12_runoff_carryover`.
3. Present carryover flux values are authoritative over stale state copies;
   malformed present fluxes are typed hard-fail states and must not be masked by
   compatibility fallback.
4. Contract-derived vectors must probe flux-over-state anti-shadow behavior and
   non-finite/out-of-domain carryover rejection.

## HPHYS0241 MOFE Hourly Carry-Array Runoff Addendum

1. In MOFE hourly lanes, upstream runon for current-OFE runoff closure is
   array-authoritative: `Qin_hourly = Σ(ui_SUrunf(ii) + ui_LfUrf(ii))` for
   `ii = 1..24`, with baseline upstream/current area-ratio scaling required
   when non-zero upstream carry is consumed.
2. `wb12_runoff_carryover` remains an aggregate publication of the explicit
   hourly array payload. If present during array-enabled execution, it must
   match the array-derived carry within declared numeric tolerance or hard-fail.
3. `wb12_runon_input` is compatibility-only and is non-authoritative whenever
   MOFE hourly carry arrays are enabled.
4. Missing array entries, non-finite entries, negative entries, or omitted
   non-zero upstream area-ratio provenance are typed hard-fail states. No
   daily aggregate can be used to synthesize omitted hourly array entries.

## HPHYS0242 Surface-Saturation Runoff Addback Addendum

1. In hourly MOFE lanes, current-OFE surface saturation runoff is
   array-authoritative: `surdra = Σui_SCrunf(ii)` for `ii = 1..24`.
2. WB12/WB14 `Q` publication must use
   `Q = Q_partition + surdra`, and the WB12 closure residual must consume the
   same `Q` value.
3. Positive top-layer saturation excess must be produced by the same-pass WB19
   hourly tail and cannot be omitted, inferred from daily aggregate carryover,
   or left in `wb18_perc_theta_0001`.
4. Contract-derived vectors must include a positive `ui_SCrunf` addback case,
   malformed-current-array rejection, and a stale-state conflict where same-pass
   `Q` remains authoritative for storage closure.

## Binding Exposure Index

Status: `scstruct08-triage-deferred`
Evidence mode: `Static`

This index conserves binding residue during SC-RUNOFFPART context reduction.
Rows mapped to existing IDs have a same-section `INV-RUNOFFPART-*` reference and
are resolved only for mechanical exposure. Rows routed to
`science-review-follow-on` remain in the binding core and are not sidecar-
eligible until SCSTRUCT09 maps, promotes, or explicitly retains them with a
narrower HOLD.

| Entry ID | Source | Status | Binding classification | Canonical binding IDs | Review gate | Notes |
|---|---|---|---|---|---|---|
| `EROD12-CROSS-DOMAIN-OWNERSHIP-AND-GUARD-CLOSURE-ADDENDUM` | `SC-RUNOFFPART-001.md#erod12-cross-domain-ownership-and-guard-closure-addendum` | `active` | `maps-to-existing-INV` | `INV-RUNOFFPART-007, INV-RUNOFFPART-008, INV-RUNOFFPART-009, INV-RUNOFFPART-011` | `none` | SCSTRUCT08 mechanical map: same-section table names RUNOFFPART producer ownership for runoff/peak-duration and multi-OFE handoff lanes through the cited invariants; cross-contract consumer details remain core-resident pending SCSTRUCT09 semantic review. |
| `WB12-RUNOFF-RECONCILIATION-ADDENDUM` | `SC-RUNOFFPART-001.md#wb12-runoff-reconciliation-addendum` | `active` | `unpromoted-binding` | `none` | `science-review-follow-on` | SCSTRUCT08 conservative triage: runoff reconciliation equations, carryover precedence, lane-specific closure deltas, typed guards, and vectors have no same-section RUNOFFPART binding ID. |
| `WB13-DAILY-OUTPUT-COUPLING-ADDENDUM` | `SC-RUNOFFPART-001.md#wb13-daily-output-coupling-addendum` | `active` | `unpromoted-binding` | `none` | `science-review-follow-on` | SCSTRUCT08 conservative triage: WB13 `Q`/`QOFE`/`UpStrmQ`/`RM`/`P` output and hard-fail requirements have no same-section RUNOFFPART binding ID. |
| `WB14-INFILTRATION-AND-SUBDAILY-HYETOGRAPH-KERNEL-AUTHORITY-ADDENDUM` | `SC-RUNOFFPART-001.md#wb14-infiltration-and-subdaily-hyetograph-kernel-authority-addendum` | `active` | `unpromoted-binding` | `none` | `science-review-follow-on` | SCSTRUCT08 conservative triage: hyetograph, disturbed-conductivity, Green-Ampt, runoff reconciliation, tolerance, guard, and vector obligations have no same-section RUNOFFPART binding ID. |
| `WB15-CANOPY-INTERCEPTION-RUNTIME-COUPLING-ADDENDUM` | `SC-RUNOFFPART-001.md#wb15-canopy-interception-runtime-coupling-addendum` | `active` | `unpromoted-binding` | `none` | `science-review-follow-on` | SCSTRUCT08 conservative triage: interception ordering, biomass equation-input cap, coupled runoff closure, guards, and vectors have no same-section RUNOFFPART binding ID. |
| `IRRIG10-IRRIGATION-RUNTIME-COUPLING-ADDENDUM` | `SC-RUNOFFPART-001.md#irrig10-irrigation-runtime-coupling-addendum` | `active` | `unpromoted-binding` | `none` | `science-review-follow-on` | SCSTRUCT08 conservative triage: irrigation schedule resolution, forcing-depth coupling, runoff equation, guard, and vector obligations have no same-section RUNOFFPART binding ID. |
| `CLIM05-SNOW-RUNTIME-COUPLING-ADDENDUM` | `SC-RUNOFFPART-001.md#clim05-snow-runtime-coupling-addendum` | `active` | `unpromoted-binding` | `none` | `science-review-follow-on` | SCSTRUCT08 conservative triage: snow control/state requirements, signed `S` liquid input coupling, runoff reconciliation, and guard/vector obligations have no same-section RUNOFFPART binding ID. |
| `CLIM06-FROZEN-SOIL-RUNTIME-COUPLING-ADDENDUM` | `SC-RUNOFFPART-001.md#clim06-frozen-soil-runtime-coupling-addendum` | `active` | `unpromoted-binding` | `none` | `science-review-follow-on` | SCSTRUCT08 conservative triage: frost controls, frozen-state domains, infiltration-capacity consumption, guard, and vector obligations have no same-section RUNOFFPART binding ID. |
| `WB16-PEAK-RUNOFF-KERNEL-ADDENDUM` | `SC-RUNOFFPART-001.md#wb16-peak-runoff-kernel-addendum` | `active` | `unpromoted-binding` | `none` | `science-review-follow-on` | SCSTRUCT08 conservative triage: peak-runoff branch equations, baseline near-zero behavior, `m`/`ealpha` producer authority, provenance policy, guards, and vectors have no same-section RUNOFFPART binding ID. |
| `ARCH22-TYPED-PRODUCTION-SURFACE-ADDENDUM` | `SC-RUNOFFPART-001.md#arch22-typed-production-surface-addendum` | `active` | `unpromoted-binding` | `none` | `science-review-follow-on` | SCSTRUCT08 conservative triage: typed symbol/accessor migration obligations and guard-preservation vectors have no same-section RUNOFFPART binding ID. |
| `EROD13-WAVE-1-ACTIVE-PRODUCER-COUPLING-ADDENDUM` | `SC-RUNOFFPART-001.md#erod13-wave-1-active-producer-coupling-addendum` | `active` | `unpromoted-binding` | `none` | `science-review-follow-on` | SCSTRUCT08 conservative triage: Wave-1 enabled-path mandatory producer surfaces and fallback prohibitions cite RUNOFFPART ownership but no same-section RUNOFFPART binding ID. |
| `EROD14-WAVE-2-ACTIVE-PRODUCER-COUPLING-ADDENDUM` | `SC-RUNOFFPART-001.md#erod14-wave-2-active-producer-coupling-addendum` | `active` | `maps-to-existing-INV` | `INV-RUNOFFPART-007, INV-RUNOFFPART-008, INV-RUNOFFPART-009` | `none` | SCSTRUCT08 mechanical map: same-section text preserves continuity from `INV-RUNOFFPART-007..009` for multi-OFE routing-classification and producer-coupling surfaces; semantic completeness remains for SCSTRUCT09 before any relocation. |
| `HPHYS0240-HOURLY-RUNOFF-CARRYOVER-ADDENDUM` | `SC-RUNOFFPART-001.md#hphys0240-hourly-runoff-carryover-addendum` | `active` | `unpromoted-binding` | `none` | `science-review-follow-on` | SCSTRUCT08 conservative triage: same-pass carryover precedence, republished carryover, anti-shadow behavior, and malformed flux rejection have no same-section RUNOFFPART binding ID. |
| `HPHYS0241-MOFE-HOURLY-CARRY-ARRAY-RUNOFF-ADDENDUM` | `SC-RUNOFFPART-001.md#hphys0241-mofe-hourly-carry-array-runoff-addendum` | `active` | `unpromoted-binding` | `none` | `science-review-follow-on` | SCSTRUCT08 conservative triage: hourly array-authoritative upstream runon, aggregate anti-shadow, area-scaling provenance, and malformed-array rejection have no same-section RUNOFFPART binding ID. |
| `HPHYS0242-SURFACE-SATURATION-RUNOFF-ADDBACK-ADDENDUM` | `SC-RUNOFFPART-001.md#hphys0242-surface-saturation-runoff-addback-addendum` | `active` | `unpromoted-binding` | `none` | `science-review-follow-on` | SCSTRUCT08 conservative triage: `surdra` addback, same-pass `Q` closure, hidden-storage prohibition, and vector obligations have no same-section RUNOFFPART binding ID. |

## Gap Register

| Gap ID | Statement | Impact | Promotability | Evidence |
|---|---|---|---|---|
| GAP-RUNOFFPART-001 | Full per-invariant comparator vectors for multi-OFE invariant families remain uncurated, and this residual automation limitation is explicitly risk-accepted for current governance progression. | Automated per-invariant acceptance remains limited; manual comparator interpretation is required where vectors are absent. | closed | `[DIRECT][Static]` |
| GAP-RUNOFFPART-002 | Wave-0 erosion-lane alias-ownership ambiguity for required runoff/peak-duration boundary symbols is explicitly dispositioned by canonical EROD11 alias ownership registers. | Alias-ownership ambiguity closure is complete for required boundary symbols; production erosion physics remains separately `HOLD`-gated by non-promotable companion/process gaps. | closed | `[DIRECT][Static] + [Ran]` |
| GAP-RUNOFFPART-003 | Chapter-4 limitations explicitly note Hortonian-flow framing and reduced recession interaction outside partial-equilibrium correction; companion contracts for variable-source-area/return-flow behavior are not authored. | Scope caveat must remain explicit to avoid over-claiming runoff applicability. | non-promotable | `[DIRECT][Static] + [INFERENCE][Static]` |
| GAP-RUNOFFPART-004 | EROD12 ratifies cross-domain ownership and guard semantics for required erosion-lane runoff boundary surfaces using canonical companion-contract addenda and row-scoped invariant ownership. | Required Wave-0 cross-domain ownership ambiguity is closed for erosion-lane runoff boundaries; broader hydrology-scope limits remain governed by `GAP-RUNOFFPART-003`. | closed | `[DIRECT][Static] + [Ran]` |
| GAP-RUNOFFPART-005 | WB16 baseline-authoritative `ealpha` producer chain (`frcfac -> rdat(alpha) -> alphay -> eplane`) is now implemented in production runtime surfaces for runtime-projection-complete lanes, with explicit runtime/compatibility provenance policy. | Producer-chain migration closure is complete for scoped runtime lanes; compatibility branch remains explicitly non-promotable and warning-gated when required producer symbols are absent. | closed | `[DIRECT][Static] + [Ran]` |

## Revision History

| Date UTC | Version | Author | Change |
|---|---|---|---|
| `2026-06-07` | `40` | `Codex` | FQ3-DC ET Corn amendment: aligned WB15 interception biomass-domain guard with plant authority; finite non-negative `vdmt` is valid even above the prior 0.8 kg m^-2 consumer cap, while the interception equation uses the pinned-baseline `8000 kg ha^-1` mass-input cap. |
| `2026-06-05` | `38` | `Codex` | HPHYS0298 amendment: added runoff-consumer paired-partition authority requiring closed downstream identities to remain tied to ordered first-divergent cut-point evidence and forbidding compensation when snow/`RM` ownership remains upstream or unresolved. |
| `2026-06-05` | `37` | `Codex` | HPHYS0297 amendment: added runoff-consumer defect-ledger authority requiring closed `Q` and producer-consumer identity to remain tied to snow/`RM` reconstruction verdicts without authorizing WB12/WB14/WB13 compensation. |
| `2026-06-07` | `38` | `Codex` | SNOWSCI-S1 review disposition: added `TOL-RUNOFFPART-007`, a bounded WB14 interval-infiltration roundoff tolerance for Green-Ampt boundary solves, with material over-infiltration still fail-closed. |
| `2026-06-05` | `36` | `Codex` | HPHYS0296 review disposition: tightened runoff-consumer acceptance so closed `Q` and closed `RM` producer-consumer identity are necessary but not sufficient; snow/`RM` residual re-tiering now requires the per-window defective-model verdict gate. |
| `2026-06-05` | `35` | `Codex` | HPHYS0296 amendment: added runoff-consumer acceptance classifier requiring closed `RM` producer-consumer identity and closed `Q` before routing residuals to snow/winter producer acceptance or hold. |
| `2026-06-05` | `34` | `Codex` | HPHYS0291 amendment: required runoff reconciliation to publish same-day snow publication fluxes before WB13 and prohibited state/default substitutes for absent producer fluxes. |
| `2026-06-05` | `33` | `Codex` | HPHYS0290 amendment: required WB12/WB14 to publish explicit post-winter direct rain as `snow.post_winter_rain_m` for WB13 `RM` instead of allowing downstream inference. |
| `2026-06-04` | `32` | `Codex` | HPHYS0289 amendment: required WB12/WB14 to publish explicit daily routed `wmelt` matching the infiltration/runoff liquid term so WB13 `RM` can consume baseline `rain + wmelt + irrigation`. |
| `2026-06-04` | `31` | `Codex` | HPHYS0288 amendment: added WB12/WB14 residual rain-on-snow routed-melt partition authority requiring released rain to follow `hrrain -> hrmlt -> wmelt -> fin/smrate` rather than direct-rain-only forcing. |
| `2026-06-04` | `30` | `Codex` | HPHYS0287 amendment: added WB12/WB14 fail-closed runtime snow-state guard before inactive fallback and liquid partition assembly. |
| `2026-06-04` | `29` | `Codex` | HPHYS0285 amendment: added local-liquid WB12/WB14 infiltration handoff authority requiring positive direct-rain, melt, and irrigation infiltration to remain available for WB18 `fin/xfin` storage ingress without active-snow gating; MOFE carry/runon storage-ingress promotion is deferred. |
| `2026-06-04` | `28` | `Codex` | HPHYS0283 amendment: linked WB12 snowmelt infiltration partition to WB18 same-pass layer ingress (`SC-PERC-001#INV-PERC-016`) so reduced runoff without storage mutation remains invalid. |
| `2026-06-04` | `27` | `Codex` | HPHYS0283 amendment: added baseline-authoritative snowmelt event-forcing authority for WB12 infiltration/runoff partition and forbade melt-only runoff closure shortcuts. |
| `2026-06-01` | `26` | `Codex` | HPHYS0242 amendment: added `INV-RUNOFFPART-014` and addendum requiring `Σui_SCrunf(ii)` current surface-saturation addback in WB12/WB14 `Q`, shared closure residual lineage, and malformed/aggregate-only current-array rejection. |
| `2026-06-01` | `25` | `Codex` | HPHYS0241 amendment: added `INV-RUNOFFPART-013` and runoff/runon authority for explicit 24-slot MOFE hourly carry arrays, aggregate anti-shadow validation against `wb12_runoff_carryover`, compatibility-only `wb12_runon_input` posture, and fail-closed malformed-array vectors. |
| `2026-06-01` | `24` | `Codex` | HPHYS0240 amendment: added `INV-RUNOFFPART-012` and addendum codifying same-pass `wb12_runoff_carryover` authority, compatibility-only `wb12_runon_input` fallback, carryover flux publication, and malformed-flux hard-fail vectors. |
| `2026-05-29` | `23` | `Codex` | HILLSTAB08 amendment: landed baseline-authoritative WB16 `ealpha` producer-chain runtime migration (`frcfac -> rdat(alpha) -> alphay -> eplane`), added runtime-producer provenance vector (`runtime_provided`), retained explicit compatibility degradation policy (`SIMPIPE-W-003`), and dispositioned `GAP-RUNOFFPART-005` to `closed`. |
| `2026-05-29` | `22` | `Codex` | HILLSTAB07 amendment: added explicit WB16 input-provenance authority for canonical `m=1.5`, baseline `ealpha` producer-chain lineage, compatibility-seed provenance surfaces/warning obligations (`wb16_ealpha_compatibility_seed_used`, `wb16_ealpha_seed_policy`, `SIMPIPE-W-003`), and non-promotable gap row `GAP-RUNOFFPART-005` for full producer migration closure. |
| `2026-05-29` | `21` | `Codex` | HILLSTAB06 amendment: aligned WB16 authority to baseline `appmth` near-zero runoff branch (`Q < 1.0e-8`) and explicit positivity-domain semantics so positive near-zero WB16 intermediates do not fail pre-floor. |
| `2026-05-26` | `20` | `Codex` | SIMIMPL36 amendment: added explicit WB12/WB14 near-zero reconciled-runoff canonicalization authority (`TOL-RUNOFFPART-006`) requiring `Q`/`wb12_runoff_reconciled` normalization to zero only within `[-1e-12, 0)` before writeback/publication while preserving hard-fail posture for material negatives. |
| `2026-05-25` | `19` | `Codex` | MOFE13 amendment: added baseline-authoritative WB14 `ksatadj` three-regime conductivity authority (`9001` exponential recovery, `9002` Saxton-Rawls Brooks-Corey, `9003` burn-severity floor) with explicit active-path guard posture and WB18 layer-state coupling inputs. |
| `2026-05-20` | `0` | `Codex` | Initial canonical stub created by SCI-06 work-package prep. |
| `2026-05-20` | `1` | `Codex` | Full draft authored with Chapter-4 authority anchors, invariants, guard map, alias map, obligations, boundary dispositions, tolerances, and gap register. |
| `2026-05-20` | `2` | `Codex` | Post-review amendment pass: added explicit event-closure term identity, added normative multi-OFE case outcome table, added alias row for `De`, and split rate tolerances for clearer unit-specific governance. |
| `2026-05-23` | `3` | `Codex` | WB12 amendment: added runoff reconciliation kernel authority with deterministic closure diagnostics, typed WB12 runoff guard codes, and WB12 contract-derived vectors. |
| `2026-05-23` | `4` | `Codex` | WB13 amendment: added canonical daily output coupling authority for runoff/runon symbols (`Q`, `QOFE`, `UpStrmQ`, `RM`, `P`) with explicit WB13 malformed-output hard-fail posture. |
| `2026-05-23` | `5` | `Codex` | WB14 amendment: added production infiltration + subdaily hyetograph kernel authority with Green-Ampt lineage branch rules, typed WB14 runoff guards, and WB14 contract-derived vectors. |
| `2026-05-23` | `6` | `Codex` | CLIM05 amendment: added active snow-control runoff coupling authority via signed `S`, required `snow.options.*`/`snow.runtime_swe` surfaces, and typed hard-fail guard posture for active-coupling symbol/domain violations. |
| `2026-05-23` | `7` | `Codex` | CLIM06 amendment: added active frost/frozen-soil runoff coupling authority, required `frost.options.*` and `frost.runtime_*` surfaces, and typed hard-fail posture for active-coupling symbol/domain violations in WB14 reconciliation. |
| `2026-05-23` | `8` | `Codex` | WB15 amendment: added canopy interception runtime coupling authority using plant-state surfaces (`cancov`, `lai`, `vdmt`) with interception-before-infiltration reconciliation and explicit `I` runoff coupling output under typed guard posture. |
| `2026-05-23` | `9` | `Codex` | IRRIG10 amendment: added runtime irrigation schedule-source coupling authority (`irrigation.depletion.*`, `irrigation.fixeddate.*`, `irrigation.runtime_*`) and explicit `Irr` runoff-forcing closure posture with typed WB14 guard requirements. |
| `2026-05-23` | `10` | `Codex` | WB16 amendment: added closure-diagnostics peak-runoff authority (`peakro`, `watdur`) with deterministic `tstar` branch rules, explicit minimum-flow/duration-limit posture, and typed WB16 guard/test-vector requirements. |
| `2026-05-23` | `11` | `Codex` | ARCH22 amendment: added typed production-surface authority requiring covered runoff-partition interfaces to consume boundary symbols via ARCH22 typed symbol families while preserving WB14/WB15/WB16 failure-class/message continuity. |
| `2026-05-23` | `12` | `Codex` | EROD11 amendment: ratified Wave-0 alias ownership for runoff/peak-duration coupling surfaces, added explicit cross-contract ownership register, and downgraded `GAP-RUNOFFPART-002` from non-promotable to promotable-with-risk pending `EROD14` internal alias expansion. |
| `2026-05-23` | `13` | `Codex` | EROD11 closure amendment: dispositioned alias-ownership ambiguity row `GAP-RUNOFFPART-002` to `closed` for required boundary symbols and made explicit that erosion-physics implementation remains separately governed by non-promotable holds. |
| `2026-05-23` | `14` | `Codex` | EROD11 risk-acceptance amendment: dispositioned `GAP-RUNOFFPART-001` from promotable-with-risk to `closed` via explicit governance risk acceptance while preserving non-promotable erosion-physics HOLD posture. |
| `2026-05-23` | `15` | `Codex` | WB20 amendment: added forward-solver lane selector authority (`wb20_forward_solver_lane_enabled`) and lane-scoped WB12 runoff closure semantics so parity-lane acceptance is solver-residual-derived and excludes observed target substitution. |
| `2026-05-23` | `16` | `Codex` | EROD12 amendment: added cross-domain ownership/guard closure addendum and dispositioned `GAP-RUNOFFPART-004` to `closed` for required erosion-lane runoff boundaries while retaining Hortonian-scope governance hold row `GAP-RUNOFFPART-003`. |
| `2026-05-25` | `17` | `Codex` | EROD13 amendment: activated Wave-1 runoff producer coupling semantics for erosion-core execution (`erod13_core_enabled` path), requiring explicit WB12/WB16 runoff and peak-duration surfaces with typed hard-fail continuity (`HKERNEL-EROD13-CORE-E-001..003`). |
| `2026-05-25` | `18` | `Codex` | EROD14 amendment: added active Wave-2 runoff producer-coupling authority for multi-OFE case symbols and runon/runoff transition surfaces with typed hard-fail continuity (`HKERNEL-EROD14-WAVE2-E-001..003`). |
