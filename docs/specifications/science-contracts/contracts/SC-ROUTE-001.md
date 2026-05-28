---
contract_id: SC-ROUTE-001
title: Watershed Routing and Channel Process Contract
status: in_review
maturity: draft
owner: openWEPP maintainers + hydrology reviewer
contract_version: 34
producer_scope:
  - Channel runon/runoff volume routing and transmission-loss accounting surfaces
  - Channel peak-discharge and duration routing surfaces at inlet/outlet boundaries
  - Channel sediment continuity and detachment/deposition boundary surfaces
consumer_scope:
  - Watershed downstream channel and outlet routing consumers
  - Impoundment and watershed-node consumers requiring channel flux/state payloads
  - Comparator/replay surfaces using watershed confidence-tier signals
evidence_level: static
last_reviewed: 2026-05-28
supersedes: []
superseded_by: []
---

# SC-ROUTE-001 Watershed Routing and Channel Process Contract

Status: `in_review`
Maturity: `draft`
Evidence mode: `Static`

## Purpose

Define top-down scientific authority for watershed channel routing hydrology and
channel erosion behavior, including hillslope/impoundment/channel handoff
semantics required for openWEPP watershed assembly.

## Scientific Scope

In scope:
- Channel runon-runoff accounting, transmission-loss handling, and channel-event
  duration semantics. `[DIRECT][Static]`
- Channel inlet/outlet peak-runoff estimation and time-of-concentration routing
  semantics. `[DIRECT][Static]`
- Channel sediment continuity, detachment/transport/deposition, and shear-based
  erosion boundary behavior. `[DIRECT][Static]`
- Required boundary obligations for hillslope-to-channel and
  channel-to-downstream routing payloads. `[DIRECT][Static] + [INFERENCE][Static]`

Out of scope:
- Kernel implementation details and Rust API naming/layout. `[INFERENCE][Static]`
- Classical gully headcutting, bank sloughing, and perennial stream mechanics
  not represented by WEPP channel routines. `[DIRECT][Static]`
- Hillslope-only runoff partition internals owned by `SC-RUNOFFPART-001`,
  hillslope erosion internals owned by `SC-SED-001` (including baseline
  `CONTIN -> ROUTE` hillslope segment routing), and impoundment internals
  owned by `SC-IMPOUND-001` except explicit coupling boundaries. `[INFERENCE][Static]`

## Authority Anchors

| Anchor ID | Source anchor | Contract use | Evidence |
|---|---|---|---|
| REF-ROUTE-CH13-RUNON | `references/50201000/chap13.pdf` §13.2 Eq. [13.2.1]-[13.2.3] | Channel runon decomposition, runon-depth conversion, and event-duration selection. | `[DIRECT][Static]` |
| REF-ROUTE-CH13-TLOSS | `chap13.pdf` §13.2 Eq. [13.2.4]-[13.2.6] + Case I-IV text | Transmission-loss accounting and runoff-case branch semantics for `qci`, `qcf`, and `tl`. | `[DIRECT][Static]` |
| REF-ROUTE-CH13-PEAKIN | `chap13.pdf` §13.4.1 Eq. [13.4.1]-[13.4.2] | Triangular synthetic hydrograph inlet-peak superposition method for multi-source inflow. | `[DIRECT][Static]` |
| REF-ROUTE-CH13-RAT | `chap13.pdf` §13.4.2.1 Eq. [13.4.3]-[13.4.24] | Modified Rational outlet-peak method, travel-time decomposition, and alpha selection rules. | `[DIRECT][Static]` |
| REF-ROUTE-CH13-CREAMS | `chap13.pdf` §13.4.2.2 Eq. [13.4.25] | CREAMS statistical outlet-peak method. | `[DIRECT][Static]` |
| REF-ROUTE-WSHCQI-RUNON | `/workdir/wepp-forest_260430_baseline/src/wshcqi.for` (`dac3c950d8b16cc73774bf5ce2e7e11f80baac70`) | Legacy-equivalent channel runon assembly (`rvolat`, `rvotop`, `rvolon`) and channel-duration max rule (`watdur = max(...)`) authority used for WS11 migration closure. | `[DIRECT][Static]` |
| REF-ROUTE-WSHDRV-ORDER | `/workdir/wepp-forest_260430_baseline/src/wshdrv.for` (`dac3c950d8b16cc73774bf5ce2e7e11f80baac70`) | Channel execution-order authority for WS11: `wshcqi -> wshirs -> wshrun/wshpek`, plus direct `wshchr` routing path when `ipeak > 2` and local channel runoff is absent. | `[DIRECT][Static]` |
| REF-ROUTE-WSHPEK-IPEAK | `/workdir/wepp-forest_260430_baseline/src/wshpek.for` (`dac3c950d8b16cc73774bf5ce2e7e11f80baac70`) | `ipeak` method-selection authority (`1` modified Rational, `2` CREAMS, `>=3` wave-routing via `wshchr`) and peak/duration post-processing semantics. | `[DIRECT][Static]` |
| REF-ROUTE-WSHCHR-WAVE | `/workdir/wepp-forest_260430_baseline/src/wshchr.for` (`dac3c950d8b16cc73774bf5ce2e7e11f80baac70`) | Legacy-equivalent channel wave-routing authority (linear kinematic wave and Muskingum-Cunge branch equations, storage closure, routed `peakot`/`runvol`/`rundur` outputs). | `[DIRECT][Static]` |
| REF-ROUTE-CHRQIN-WAVE | `/workdir/wepp-forest_260430_baseline/src/chrqin.for` (`dac3c950d8b16cc73774bf5ce2e7e11f80baac70`) | Legacy-equivalent channel wave-routing inflow-state assembly authority for `ipeak > 2` (`q1`, `qin`, `qlat`, and segment-coefficient preparation surfaces). | `[DIRECT][Static]` |
| REF-ROUTE-CH13-DUR | `chap13.pdf` §13.4.3 Eq. [13.4.26] | Effective runoff-duration computation from volume and outlet peak. | `[DIRECT][Static]` |
| REF-ROUTE-CH13-SVF | `chap13.pdf` §13.5.2 Eq. [13.5.1]-[13.5.5] | Spatially-varied flow and friction-slope relationships used by channel erosion routines. | `[DIRECT][Static]` |
| REF-ROUTE-CH13-EFFLEN | `chap13.pdf` §13.5.3 Eq. [13.5.6]-[13.5.12] | Effective channel-length and discharge-distribution semantics for segment routing. | `[DIRECT][Static]` |
| REF-ROUTE-CH13-SHEAR | `chap13.pdf` §13.5.4 Eq. [13.5.13]-[13.5.16] | Shear stress partition between soil and vegetation and detachment-driving stress terms. | `[DIRECT][Static]` |
| REF-ROUTE-CH13-CONT | `chap13.pdf` §13.5.5 Eq. [13.5.17]-[13.5.18] | Quasi-steady sediment continuity and inlet/lateral sediment load assembly semantics. | `[DIRECT][Static]` |
| REF-ROUTE-CH13-DETDEP | `chap13.pdf` §13.5.6 Eq. [13.5.19]-[13.5.29] | Detachment-capacity, deposition, and transport-capacity branch logic for segment updates. | `[DIRECT][Static]` |
| REF-ROUTE-CH13-LIMIT | `chap13.pdf` §13.6 summary limitations | Applicability bounds: intended small agricultural watersheds and explicit limitations (no partial-area response, no headcutting, no bank sloughing, no perennial streams). | `[DIRECT][Static]` |
| REF-ROUTE-CH4-COUPLING | `references/50201000/chap4.pdf` Eq. [4.2.1]-[4.2.9], [4.3.1]-[4.3.5], [4.4.27]-[4.4.29], [4.5.4], [4.5.6] | Channel hydrology uses hillslope infiltration/rainfall-excess and recession-infiltration relationships by explicit Chapter-13 linkage. | `[DIRECT][Static] + [INFERENCE][Static]` |
| REF-ROUTE-CH5-COUPLING | `references/50201000/chap5.pdf` §5.1-§5.4 and `chap13.pdf` §13.3 | Channel water-balance/percolation routines are stated as identical to hillslope routines. | `[DIRECT][Static]` |
| REF-ROUTE-HBP-FORMAT | `/workdir/wepp-forest/docs/contracts/hillslope-binary-pass-format.md` (`EVENT Payload`) | Canonical binary pass serialization field names and units consumed at routing boundary (`total_detachment_kg`, `total_deposition_kg`, `sediment_concentration_kg_m3[npart]`, `particle_diameter_m[npart]`, `particle_flow_fraction[npart]`). | `[DIRECT][Static]` |
| REF-ROUTE-HBP-READER | `/workdir/wepp-forest/docs/contracts/watershed-hillslope-pass-reader-contract.md` (`Read Contract`, `Required Invariants`) | Reader/index fail-closed semantics for malformed/missing hillslope payload fields and no-text-fallback posture. | `[DIRECT][Static] + [INFERENCE][Static]` |
| REF-ROUTE-LEGACY-HSROUTE-BOUNDARY | `/workdir/wepp-forest_260430_baseline/src/contin.for` + `/workdir/wepp-forest_260430_baseline/src/route.for` (`dac3c950d8b16cc73774bf5ce2e7e11f80baac70`) | Scope-boundary provenance anchor: legacy `call route` from `CONTIN` is hillslope sediment routing authority governed by `SC-SED-001`, not WS10 watershed/channel routing authority in this contract. | `[DIRECT][Static]` |
| REF-ROUTE-LEGACY-RTPART-BOUNDARY | `/workdir/wepp-forest_260430_baseline/src/rtpart.for` + `/workdir/wepp-forest_260430_baseline/src/grow.for` (`dac3c950d8b16cc73774bf5ce2e7e11f80baac70`) | Provenance correction anchor: `rtpart.for` belongs to plant root-mass partitioning lineage and is out of routing-contract scope. | `[DIRECT][Static]` |
| REF-ROUTE-PHYS-BOUNDS | Physical/common-sense invariant class | Non-negative area/volume/duration domains and explicit branch handling for no-flow states. | `[INFERENCE][Static]` |

## Variables and Units (Externally Relevant)

| Symbol | Units | Meaning | Producer | Consumer |
|---|---|---|---|---|
| `rol`, `roi`, `rov` | `m^3` | Lateral, inlet, and total channel runon volumes (Eq. [13.2.1]). | upstream hillslope/impoundment/channel inflow assembly | channel runon-runoff solver |
| `Ach`, `rod` | `m^2`, `m` | Channel area and converted runon depth (Eq. [13.2.2]). | channel geometry + runon assembly | channel runoff-case branch logic |
| `durc`, `durrunon`, `durchan`, `durirrig` | `s` | Channel storm duration and candidate maxima (Eq. [13.2.3]). | channel event-duration selector | runoff + peak-routing logic |
| `qci`, `qcf` | `m` | Initial and final channel runoff depth after branch/correction logic. | channel rainfall-excess/transmission-loss solver | channel volume and peak calculations |
| `rofc`, `roff`, `tl` | `m^3` | Channel runoff pre-adjustment, final runoff, and transmission-loss volume. | channel runoff-case solver | peak-duration and balance reporting |
| `fc`, `fp` | `m^3` | Entering water volume and potential infiltration capacity for Case III logic. | transmission-loss branch solver | `qcf` and `tl` branch outcomes |
| `tb`, `tp` | `min` | Synthetic hydrograph base time and time-to-peak for contributing elements (Eq. [13.4.1]-[13.4.2]). | inlet-peak superposition routine | channel inlet peak estimate |
| `qpi`, `qpo` | `m^3 s^-1` | Peak runoff rate of contributing element and channel/watershed outlet peak discharge. | inlet/outlet peak-routing routines | downstream routing and duration computation |
| `tc`, `tcc`, `tcs`, `tci` | `h` | Outlet time of concentration and channel/overland/impoundment components (Eq. [13.4.4]). | modified Rational routing-time routines | outlet-peak equation |
| `alpha`, `alphah`, `alphac`, `alphai` | `fraction` | Rational-equation alpha and candidate source alphas (Eq. [13.4.21]-[13.4.24]). | source routing components + outlet alpha selector | modified Rational outlet peak |
| `durrof` | `s` | Effective runoff duration (Eq. [13.4.26]). | outlet volume/peak post-processor | channel erosion event forcing |
| `lc`, `leff`, `ltop` | `ft`, `ft`, `ft` | Physical/effective/top extension lengths for segment routing (Eq. [13.5.8]-[13.5.9]). | channel effective-length routine | segment discharge distribution |
| `qt`, `qlat`, `qlat_eff`, `qu`, `ql` | `ft^3 s^-1` | Top, lateral, effective lateral, upper, and lower segment discharges (Eq. [13.5.6]-[13.5.12]). | discharge-distribution routine | spatially-varied flow and erosion solver |
| `Sf`, `Sstar` | `ft ft^-1`, `fraction` | Friction slope and dimensionless slope from spatially-varied flow relations. | spatially-varied flow routine | shear and erosion equations |
| `tau`, `taucov`, `taucr` | `lb ft^-2` | Soil shear stress, cover shear stress, and critical shear stress thresholds. | shear partition routine + soil/cover parameters | detachment/deposition branch logic |
| `D`, `DF`, `DL` | `lb ft^-2 s^-1`, `lb ft^-2 s^-1`, `lb ft^-2 s^-1` | Detachment/deposition rates and lateral sediment inflow rate in continuity equation. | channel erosion segment solver | sediment load update |
| `qsed`, `qsed_top`, `qsed_lat`, `Tc` | `lb ft^-1 s^-1`, `lb s^-1`, `lb s^-1 ft^-1`, `lb ft^-1 s^-1` | Segment sediment load, inlet/lateral sediment fluxes, and transport capacity. | sediment continuity + transport-capacity solver | downstream segment and outlet sediment yield |
| `Kch`, `wc`, `Ech` | `s^-1`, `ft`, `lb ft^-1 s^-1` | Channel erodibility, active width, and per-length soil loss for active-channel branch (Eq. [13.5.20]). | detachment-capacity routine | erosion-width update and sediment load integration |

## Invariants

| Invariant ID | Statement | Severity | Authority | Evidence |
|---|---|---|---|---|
| INV-ROUTE-001 | Runon decomposition invariant: channel runon assembly must satisfy Eq. [13.2.1] (`rov = rol + roi`) and Eq. [13.2.2] (`rod = rov / Ach`) with explicit positive-area requirement (`Ach > 0`). | hard-fail | REF-ROUTE-CH13-RUNON, REF-ROUTE-PHYS-BOUNDS | `[DIRECT][Static] + [INFERENCE][Static]` |
| INV-ROUTE-002 | Duration-selection invariant: channel event duration must be selected by Eq. [13.2.3] (`durc = max(durrunon, durchan, durirrig)`) with declared units and no implicit duration fallback. | hard-fail | REF-ROUTE-CH13-RUNON | `[DIRECT][Static] + [INFERENCE][Static]` |
| INV-ROUTE-003 | Runoff-case invariant: Case I-IV branching from §13.2 must be explicit for (`qci`, `rod`) combinations, including Case IV zero-flow branch (`qcf = 0`, `roff = 0`) and Case III branch using Eq. [13.2.5]-[13.2.6]. | hard-fail | REF-ROUTE-CH13-TLOSS, REF-ROUTE-PHYS-BOUNDS | `[DIRECT][Static] + [INFERENCE][Static]` |
| INV-ROUTE-004 | Transmission-loss closure invariant: for Case I/II, transmission losses must satisfy Eq. [13.2.4]; for Case III, losses must satisfy Eq. [13.2.6], and computed losses cannot imply runoff volume greater than entering water volume. | hard-fail | REF-ROUTE-CH13-TLOSS, REF-ROUTE-PHYS-BOUNDS | `[DIRECT][Static] + [INFERENCE][Static]` |
| INV-ROUTE-005 | Inlet-peak superposition invariant: when multiple watershed elements contribute to channel inlet flow, triangular hydrograph procedure with Eq. [13.4.1]-[13.4.2] must be used and combined hydrograph peak must be the maximum discharge on the superimposed hydrograph. | hard-fail | REF-ROUTE-CH13-PEAKIN | `[DIRECT][Static]` |
| INV-ROUTE-006 | Outlet-method branch invariant: channel outlet routing must execute exactly one branch selected by `ipeak` (`1` = modified Rational Eq. [13.4.3]-[13.4.24], `2` = CREAMS Eq. [13.4.25], `3` = linear kinematic-wave channel routing, `>=4` = Muskingum-Cunge channel routing); implicit fallback/mixing is invalid and selected-branch inputs/outputs must be finite and unit-consistent. | hard-fail | REF-ROUTE-CH13-RAT, REF-ROUTE-CH13-CREAMS, REF-ROUTE-WSHPEK-IPEAK, REF-ROUTE-WSHCHR-WAVE, REF-ROUTE-PHYS-BOUNDS | `[DIRECT][Static] + [INFERENCE][Static]` |
| INV-ROUTE-007 | Peak-duration closure invariant: for `ipeak <= 2`, if `roff <= 0.001 m^3`, peak runoff and runoff duration are both zero per §13.4.1; for `ipeak >= 3`, routed channel flow may still be evaluated from incoming hydrograph when local channel runoff is zero, but emitted outputs must obey non-negative finite closure (`roff = qpo * durrof` for `qpo > 0`, and `durrof = 0` when `qpo <= 1e-12`). | hard-fail | REF-ROUTE-CH13-DUR, REF-ROUTE-CH13-RAT, REF-ROUTE-CH13-CREAMS, REF-ROUTE-WSHDRV-ORDER, REF-ROUTE-WSHCHR-WAVE, REF-ROUTE-PHYS-BOUNDS | `[DIRECT][Static] + [INFERENCE][Static]` |
| INV-ROUTE-008 | Spatially-varied flow/shear invariant: channel erosion solver must use consistent spatially-varied flow outputs (`Sf`, `Sstar`, `leff`, `q`) to compute shear terms, and soil shear relation Eq. [13.5.13]-[13.5.16] must preserve finite physically valid domains. | hard-fail | REF-ROUTE-CH13-SVF, REF-ROUTE-CH13-SHEAR | `[DIRECT][Static] + [INFERENCE][Static]` |
| INV-ROUTE-009 | Sediment continuity invariant: quasi-steady sediment continuity Eq. [13.5.17]-[13.5.18] must be conserved across segments and particle classes with explicit inlet (`qsed_top`) and lateral (`qsed_lat`) source accounting. | hard-fail | REF-ROUTE-CH13-CONT | `[DIRECT][Static]` |
| INV-ROUTE-010 | Detachment/deposition branch invariant: detachment capacity Eq. [13.5.19]/[13.5.20], deposition Eq. [13.5.21]-[13.5.22], and transport-capacity branch iteration semantics from §13.5.6 must be explicit; silent branch collapse is invalid. | hard-fail | REF-ROUTE-CH13-DETDEP | `[DIRECT][Static] + [INFERENCE][Static]` |
| INV-ROUTE-011 | Coupling completeness invariant: required hillslope/impoundment/channel handoff payloads (runon volumes, durations, peak flow, and HBP sediment payload family `total_detachment_kg`, `total_deposition_kg`, `particle_class_count`, `sediment_concentration_kg_m3_i`, `particle_diameter_m_i`, `particle_flow_fraction_i`) must be present and parseable before routing calculations proceed. | hard-fail | REF-ROUTE-CH13-RUNON, REF-ROUTE-CH13-CONT, REF-ROUTE-CH4-COUPLING, REF-ROUTE-CH5-COUPLING, REF-ROUTE-HBP-FORMAT | `[DIRECT][Static] + [INFERENCE][Static]` |
| INV-ROUTE-012 | Governance invariant: channel-routing outputs are watershed-integrated Tier-B surfaces; unresolved major discrepancies must route to investigation/disposition and cannot be silently promoted as Tier-A-equivalent confidence. | governance-fail | REF-ROUTE-CH13-RUNON, REF-ROUTE-CH13-DETDEP, REF-ROUTE-PHYS-BOUNDS | `[INFERENCE][Static]` |
| INV-ROUTE-013 | Applicability-bound invariant: authoritative scope is limited to small agricultural watersheds (Chapter-13 summary intent) with explicit exclusions (`no partial area response`, `no headcutting`, `no bank sloughing`, `no perennial streams`); use outside these limits requires explicit governance disposition. | governance-fail | REF-ROUTE-CH13-LIMIT | `[DIRECT][Static] + [INFERENCE][Static]` |

## Invariant Guard Map

| Invariant ID | Guard class | Enforcement path | Failure behavior | Gate impact | Evidence |
|---|---|---|---|---|---|
| `INV-ROUTE-001` | runtime | Runon assembler (`rov`, `rod`, `Ach`) | Typed hard error on algebra/domain violation | Tier-B investigation gate | `[DIRECT][Static] + [INFERENCE][Static]` |
| `INV-ROUTE-002` | runtime | Duration selector | Typed hard error on invalid duration selection/unit mismatch | Tier-B investigation gate | `[DIRECT][Static] + [INFERENCE][Static]` |
| `INV-ROUTE-003` | runtime | Runoff-case branch controller | Typed hard error on missing/invalid Case I-IV branch behavior | Tier-B investigation gate | `[DIRECT][Static] + [INFERENCE][Static]` |
| `INV-ROUTE-004` | runtime | Transmission-loss calculator | Typed hard error on closure violation or non-physical loss outcome | Tier-B investigation gate | `[DIRECT][Static] + [INFERENCE][Static]` |
| `INV-ROUTE-005` | runtime | Inlet hydrograph superposition routine | Typed hard error on invalid hydrograph-base/time-to-peak calculation path | Tier-B investigation gate | `[DIRECT][Static]` |
| `INV-ROUTE-006` | runtime | `ipeak` method selector + branch executor (Rational/CREAMS/KW/MC) | Typed hard error on mixed/implicit branch use, missing selected-branch inputs, or non-finite branch outputs | Tier-B investigation gate | `[DIRECT][Static] + [INFERENCE][Static]` |
| `INV-ROUTE-007` | runtime | Threshold/duration closure post-processor | Typed hard error on threshold-branch violation (`ipeak <= 2`) or invalid routed closure (`ipeak >= 3`) between `roff`/`qpo`/`durrof` | Tier-B investigation gate | `[DIRECT][Static] + [INFERENCE][Static]` |
| `INV-ROUTE-008` | runtime | Spatially-varied flow + shear partition pipeline | Typed hard error on invalid friction/shear domains | Tier-B investigation gate | `[DIRECT][Static] + [INFERENCE][Static]` |
| `INV-ROUTE-009` | runtime | Segment sediment continuity solver | Typed hard error on continuity violation across segment boundaries | Tier-B investigation gate | `[DIRECT][Static]` |
| `INV-ROUTE-010` | runtime | Detachment/deposition branch evaluator | Typed hard error on branch rule violation or unresolved Tc iteration failure | Tier-B investigation gate | `[DIRECT][Static] + [INFERENCE][Static]` |
| `INV-ROUTE-011` | runtime | Watershed handoff payload validator | Typed hard error on missing/unparseable boundary payload fields | Tier-B investigation gate | `[DIRECT][Static] + [INFERENCE][Static]` |
| `INV-ROUTE-012` | governance | Review/disposition/verification + comparator policy gate | Promotion `HOLD` when Tier-B discrepancies are undispositioned or misclassified | Governance gate | `[INFERENCE][Static]` |
| `INV-ROUTE-013` | governance | Contract scope review + promotion checklist | Promotion `HOLD` for workloads outside §13.6 applicability limits unless explicit risk disposition exists | Governance gate | `[DIRECT][Static] + [INFERENCE][Static]` |

## Symbol Alias Map

Canonical symbols in this contract follow Chapter-13 WEPP notation. EROD11
ratifies Wave-0 erosion-lane alias ownership for required routing-coupled
boundaries while preserving canonical identity aliases for not-yet-implemented
channel erosion internals.

| Canonical symbol | Boundary/API name | Scope | Units check | Evidence |
|---|---|---|---|---|
| `rol`, `roi`, `rov`, `rod` | identity names | runon/runoff volume-depth boundaries | `m^3` and `m` preserved | `[DIRECT][Static]` |
| `qci`, `qcf`, `rofc`, `roff`, `tl` | identity names | channel runoff and loss boundaries | chapter-declared units preserved | `[DIRECT][Static]` |
| `durc`, `durrunon`, `durchan`, `durirrig`, `durrof` | identity names | event-duration boundaries | `s` preserved | `[DIRECT][Static]` |
| `tb`, `tp`, `qpi`, `qpo` | identity names | hydrograph peak-routing boundaries | chapter-declared units preserved | `[DIRECT][Static]` |
| `qpo`, `durrof`, `roff` (WS10 typed state/flux projection) | `WatershedProductionStateSymbol::ChannelNode{field=Qpo|Durrof}`; `WatershedProductionFluxSymbol::ChannelNode{field=Roff}` | node-scoped production routing outputs | `m^3 s^-1`, `s`, `m^3` preserved | `[DIRECT][Static] + [INFERENCE][Static]` |
| `tc`, `tcc`, `tcs`, `tci` | identity names | time-of-concentration boundaries | `h` preserved | `[DIRECT][Static]` |
| `alpha`, `alphah`, `alphac`, `alphai` | identity names | outlet-peak method-selection boundaries | `fraction` preserved | `[DIRECT][Static]` |
| `lc`, `leff`, `ltop`, `qt`, `qlat`, `qlat_eff`, `qu`, `ql` | identity names | segment discharge-routing boundaries | chapter-declared units preserved | `[DIRECT][Static]` |
| `Sf`, `Sstar`, `tau`, `taucov`, `taucr` | identity names | friction/shear boundaries | chapter-declared units preserved | `[DIRECT][Static]` |
| `D`, `DF`, `DL`, `qsed`, `qsed_top`, `qsed_lat`, `Tc` | identity names | sediment continuity/detachment boundaries | chapter-declared units preserved | `[DIRECT][Static]` |
| `Kch`, `wc`, `Ech` | identity names | active-channel detachment boundaries | chapter-declared units preserved | `[DIRECT][Static]` |
| `hs{ID}_peakro`, `hs{ID}_watdur` | `WatershedProductionStateSymbol::{HillslopeContributorPeak,HillslopeContributorDuration}` | hillslope contributor forcing aliases consumed at WS10 channel ingress | contributor-scoped peak/duration semantics preserved | `[DIRECT][Static] + [INFERENCE][Static]` |
| `hs{ID}_total_detachment_kg`, `hs{ID}_total_deposition_kg` | `WatershedProductionStateSymbol::{HillslopeContributorTotalDetachmentKg,HillslopeContributorTotalDepositionKg}` | hillslope contributor sediment-total payload aliases consumed at WS10 channel ingress | contributor-scoped sediment-total semantics preserved | `[DIRECT][Static] + [INFERENCE][Static]` |
| `hs{ID}_particle_class_count` | `WatershedProductionStateSymbol::HillslopeContributorParticleClassCount` | hillslope contributor class-count payload alias consumed by WS10 sediment payload validator | contributor-scoped class-cardinality semantics preserved | `[DIRECT][Static] + [INFERENCE][Static]` |
| `hs{ID}_sediment_concentration_kg_m3_{class:04}` | `WatershedProductionStateSymbol::HillslopeContributorSedimentConcentrationKgM3` | hillslope contributor per-class sediment concentration payload aliases consumed at WS10 channel ingress | contributor-scoped concentration semantics preserved | `[DIRECT][Static] + [INFERENCE][Static]` |
| `hs{ID}_particle_diameter_m_{class:04}` | `WatershedProductionStateSymbol::HillslopeContributorParticleDiameterMeters` | hillslope contributor per-class particle-diameter payload aliases consumed at WS10 channel ingress for sediment transport-capacity lineage inputs | contributor-scoped particle-diameter semantics preserved | `[DIRECT][Static] + [INFERENCE][Static]` |
| `hs{ID}_particle_flow_fraction_{class:04}` | `WatershedProductionStateSymbol::HillslopeContributorParticleFlowFraction` | hillslope contributor per-class particle-flow-fraction payload aliases consumed at WS10 channel ingress | contributor-scoped fraction semantics preserved | `[DIRECT][Static] + [INFERENCE][Static]` |

## EROD11 Alias Ownership Register

| Boundary ID | Canonical symbols | Runtime alias surface | Producer ownership | Consumer ownership | Evidence |
|---|---|---|---|---|---|
| `EROD-BND-001` | `hs{ID}_peakro`, `hs{ID}_watdur` | `WatershedProductionStateSymbol::{HillslopeContributorPeak,HillslopeContributorDuration}` | `SC-RUNOFFPART-001` + `SC-WATBAL-001` via WB16 coupling | `SC-ROUTE-001` WS10 intake guards (`INV-ROUTE-011`) | `[DIRECT][Static] + [INFERENCE][Static]` |
| `EROD-BND-003` | `total_detachment_kg`, `total_deposition_kg`, `particle_class_count`, `sediment_concentration_kg_m3_i`, `particle_diameter_m_i`, `particle_flow_fraction_i` | `WatershedProductionStateSymbol::{HillslopeContributorTotalDetachmentKg,HillslopeContributorTotalDepositionKg,HillslopeContributorParticleClassCount,HillslopeContributorSedimentConcentrationKgM3,HillslopeContributorParticleDiameterMeters,HillslopeContributorParticleFlowFraction}` | `SC-SED-001` | `SC-ROUTE-001` segment/channel consumers | `[DIRECT][Static] + [INFERENCE][Static]` |
| `EROD-BND-004` | `ws10_channel_{id}_qpo`, `ws10_channel_{id}_durrof`, `ws10_channel_{id}_roff` | `WatershedProductionStateSymbol::ChannelNode`; `WatershedProductionFluxSymbol::ChannelNode` | `SC-ROUTE-001` | downstream channel/impoundment/watershed consumers | `[DIRECT][Static] + [INFERENCE][Static]` |

## EROD12 Cross-Domain Ownership and Guard Closure Addendum

| Cross-domain lane | Producer ownership | Consumer guard ownership | Closure posture | Evidence |
|---|---|---|---|---|
| Hillslope contributor intake (`hs{ID}_peakro`, `hs{ID}_watdur`) | `SC-RUNOFFPART-001` + `SC-WATBAL-001` via WB16 | `SC-ROUTE-001` (`INV-ROUTE-011`) | Required Wave-0 intake ownership and guard semantics are canonicalized. | `[DIRECT][Static] + [INFERENCE][Static]` |
| Sediment boundary intake (`total_detachment_kg`, `total_deposition_kg`, `particle_class_count`, `sediment_concentration_kg_m3_i`, `particle_diameter_m_i`, `particle_flow_fraction_i`) | `SC-SED-001` (`INV-SED-010`) | `SC-ROUTE-001` (`INV-ROUTE-011`) | Routing consumer guard ownership for sediment payload completeness is explicit. | `[DIRECT][Static] + [INFERENCE][Static]` |
| WS10 routing publication (`ws10_channel_{id}_qpo`, `ws10_channel_{id}_durrof`, `ws10_channel_{id}_roff`) | `SC-ROUTE-001` | `SC-SYSTEM-001` (`INV-SYSTEM-001`..`006`) + `SC-IMPOUND-001` | Cross-domain publication ownership and downstream guard owners are explicit. | `[DIRECT][Static] + [INFERENCE][Static]` |

## Allowed Degenerate States

| Degenerate state | Allowed condition | Rationale | Evidence |
|---|---|---|---|
| No channel flow event | Case IV (`qci = 0`, `rod = 0`) with `qcf = 0` and `roff = 0`. | Explicit §13.2 case definition. | `[DIRECT][Static]` |
| Runon-only event with infiltration dominance | Case III where `qci = 0`, `rod > 0`, and `fc <= fp` yields `qcf = 0`. | Explicit Eq. [13.2.5] branch condition. | `[DIRECT][Static]` |
| No lateral inflow routing | `qlat = 0` leading to `qu = qpo` and `qlat_eff = 0`. | Explicit Eq. [13.5.10]-[13.5.11] branch semantics. | `[DIRECT][Static]` |
| Channel event below peak-routing threshold (`ipeak <= 2`) | `roff <= 0.001 m^3` yields zero peak runoff and zero runoff duration. | Explicit §13.4.1 threshold branch. | `[DIRECT][Static]` |
| Net deposition segment | Segment state where `qsed > Tc` and Eq. [13.5.21] governs deposition. | Explicit §13.5.6 branch semantics. | `[DIRECT][Static]` |

## Invalid States

- `Ach <= 0` used in Eq. [13.2.2] or negative runon-volume terms (`rol`, `roi`, `rov`) outside declared tolerance. `[DIRECT][Static] + [INFERENCE][Static]`
- `durc` not equal to `max(durrunon, durchan, durirrig)` for the emitted channel event. `[DIRECT][Static]`
- Missing or contradictory Case I-IV branch resolution for (`qci`, `rod`) combinations. `[DIRECT][Static] + [INFERENCE][Static]`
- Transmission-loss algebra implies `roff > (rov + rofc)` or negative physically invalid final runoff/loss outcomes. `[DIRECT][Static] + [INFERENCE][Static]`
- Outlet peak/discharge products emitted with undefined selected-method inputs, mixed-method fallback behavior, or non-finite `qpo`, `tc`, or `durrof` values. `[DIRECT][Static] + [INFERENCE][Static]`
- Threshold/closure violation where `ipeak <= 2` and `roff <= 0.001 m^3` still emits positive `qpo`/`durrof`, or where `ipeak >= 3` emits `roff`/`qpo`/`durrof` values violating declared routed-closure semantics. `[DIRECT][Static] + [INFERENCE][Static]`
- Shear/transport calculations with invalid domains (`Sf`, `tau`, `taucr`, `Tc`, or segment discharge terms non-finite or non-physical). `[DIRECT][Static] + [INFERENCE][Static]`
- Sediment continuity violation where segment-to-segment `qsed` updates break Eq. [13.5.17] semantics. `[DIRECT][Static]`
- Missing mandatory handoff payload fields (runon, duration, peak, and HBP sediment payload family fields) before routing/erosion calculations. `[DIRECT][Static] + [INFERENCE][Static]`
- Applying this contract to conditions outside §13.6 scope without explicit governance disposition. `[DIRECT][Static] + [INFERENCE][Static]`

## Producer Obligations

- OBL-ROUTE-P-001: Emit runon/runoff/loss terms (`rol`, `roi`, `rov`, `rod`, `qci`, `qcf`, `rofc`, `roff`, `tl`) with chapter-declared units and case-branch provenance. `[DIRECT][Static] + [INFERENCE][Static]`
- OBL-ROUTE-P-002: Emit peak-routing terms (`qpo`, `tc`, selected method context, and `durrof`) with explicit method-selection metadata and unit-consistent inputs. `[DIRECT][Static] + [INFERENCE][Static]`
- OBL-ROUTE-P-003: Emit sediment-routing terms (`qsed*`, `Tc`, detachment/deposition branch outcomes) with particle-class continuity semantics preserved. `[DIRECT][Static] + [INFERENCE][Static]`
- OBL-ROUTE-P-004: Enforce invariant failures as typed errors; no silent branch-defaulting or numeric clamping on invalid routing states. `[INFERENCE][Static]`

## Consumer Obligations

- OBL-ROUTE-C-001: Downstream channel/watershed consumers must preserve runon/runoff and transmission-loss semantics without hidden reinterpretation of signs or units. `[DIRECT][Static] + [INFERENCE][Static]`
- OBL-ROUTE-C-002: Impoundment/watershed-node consumers must preserve peak-flow and duration boundary semantics (`qpo`, `durrof`, method context). `[DIRECT][Static] + [INFERENCE][Static]`
- OBL-ROUTE-C-003: Sediment consumers must preserve particle-class continuity semantics and reject malformed `qsed`/`Tc` payloads. `[DIRECT][Static] + [INFERENCE][Static]`
- OBL-ROUTE-C-004: All consumers must fail explicitly on invariant-violating payloads and propagate invariant IDs in error context. `[INFERENCE][Static]`

## Boundary Disposition

| Invariant family | Detection point | Disposition | Comparator tier impact | Evidence |
|---|---|---|---|---|
| Runon decomposition and duration (`INV-ROUTE-001/002`) | runon assembler + duration selector | Hard error on algebra/domain/selection failure | Tier-B investigation gate | `[DIRECT][Static] + [INFERENCE][Static]` |
| Runoff cases and transmission losses (`INV-ROUTE-003/004`) | runoff-case branch + loss calculator | Hard error on branch or closure failure | Tier-B investigation gate | `[DIRECT][Static] + [INFERENCE][Static]` |
| Peak and duration routing (`INV-ROUTE-005/006/007`) | inlet/outlet peak routines + duration post-processor | Hard error on method/domain/continuity failure | Tier-B investigation gate | `[DIRECT][Static] + [INFERENCE][Static]` |
| Spatially-varied flow and erosion physics (`INV-ROUTE-008/009/010`) | segment hydrodynamics + erosion solver | Hard error on invalid flow/shear/transport branch behavior | Tier-B investigation gate | `[DIRECT][Static] + [INFERENCE][Static]` |
| Coupling completeness (`INV-ROUTE-011`) | watershed boundary payload validation | Hard error on missing/unparseable required handoff fields | Tier-B investigation gate | `[DIRECT][Static] + [INFERENCE][Static]` |
| Comparator-confidence governance (`INV-ROUTE-012`) | review/disposition/verification gate | Governance `HOLD` until Tier-B discrepancies are explicitly dispositioned | Governance gate | `[INFERENCE][Static]` |
| Applicability limits (`INV-ROUTE-013`) | scope review + promotion checklist | Governance `HOLD` for use beyond Chapter-13 stated applicability limits without explicit risk acceptance | Governance gate | `[DIRECT][Static] + [INFERENCE][Static]` |

## Tolerance and Numeric Notes

This contract follows `docs/numerics/README.md` (semantic parity, not
bit-for-bit parity). Contract-specific tolerances:

| Tolerance ID | Definition | Value | Notes | Evidence |
|---|---|---|---|---|
| TOL-ROUTE-001 | Runon decomposition residual `abs(rov - (rol + roi))` | `<= 1e-9 m^3` | Closure check for Eq. [13.2.1]. | `[DIRECT][Static] + [INFERENCE][Static]` |
| TOL-ROUTE-002 | Transmission-loss residual for Eq. [13.2.4]/[13.2.6] | `<= 1e-9 m^3` | Branch-specific closure check; runtime still hard-fails on material mismatch. | `[DIRECT][Static] + [INFERENCE][Static]` |
| TOL-ROUTE-003 | Non-negative-domain comparator tolerance for runon/runoff/loss volumes | lower bound `>= -1e-12 m^3` | Comparator-noise allowance only; runtime does not silently clamp. | `[INFERENCE][Static]` |
| TOL-ROUTE-004 | Outlet peak and duration positivity tolerance (`qpo`, `durrof`) | lower bound `>= -1e-12` in declared units | Required only for floating-noise interpretation; physical domain remains non-negative. | `[DIRECT][Static] + [INFERENCE][Static]` |
| TOL-ROUTE-005 | Sediment continuity residual per segment/class | `<= 1e-9 lb ft^-1 s^-1` | Continuity residual for Eq. [13.5.17] diagnostics. | `[DIRECT][Static] + [INFERENCE][Static]` |

## WB16 Hillslope Peak-Flow Intake Addendum

### WB16 Required Upstream Surfaces

| Surface | Symbols |
|---|---|
| Hillslope peak-flow payload | `peakro`, `watdur` |
| Hillslope runoff coupling payload | `Q` |
| WB16 method trace payload | `wb16_peak_method_branch`, `wb16_tstar`, `wb16_qpstar`, `wb16_vstar` |

### WB16 Coupling Rules

1. Watershed routing intake must accept WB16 peak-flow payload as the
   authoritative hillslope peak/duration source for downstream assembly.
2. Intake validity requires finite/non-negative `peakro`, `watdur`, and `Q`
   with continuity `watdur = Q/peakro` (within tolerance).
3. Missing WB16 method-trace symbols is a typed boundary failure for
   observability and replay diagnostics.
4. Routing consumers must not silently synthesize replacement peak values when
   WB16 payloads are missing or malformed.

### WB16 Typed Guard Codes

| Condition | Code |
|---|---|
| Missing required symbol | `HKERNEL-WB16-PEAK-E-001` |
| Non-finite required symbol | `HKERNEL-WB16-PEAK-E-002` |
| Domain/closure violation | `HKERNEL-WB16-PEAK-E-003` |

## WS11 Channel-Routing Physics Equivalence Addendum

### WS11 Runtime Boundary Symbols

| Surface | Symbols |
|---|---|
| Channel global routing controls | `dtchr`, `nchnum`, `cbase`, `ipeak` |
| Channel per-node controls | `ws10_channel_{id}_chnn`, `ws10_channel_{id}_ctlslp`, `ws10_channel_{id}_chnk` |
| Channel per-node segment/hydraulic scaffold controls | `ws10_channel_{id}_nslpts`, `ws10_channel_{id}_x_{point:04}`, `ws10_channel_{id}_slope_{point:04}`, `ws10_channel_{id}_depa_{point:04}`, `ws10_channel_{id}_depb_{point:04}`, `ws10_channel_{id}_wida_{point:04}`, `ws10_channel_{id}_widb_{point:04}` |
| Contributor peak payloads | `hs{ID}_peakro`, `hs{ID}_watdur` |
| Contributor sediment payloads | `hs{ID}_total_detachment_kg`, `hs{ID}_total_deposition_kg`, `hs{ID}_particle_class_count`, `hs{ID}_sediment_concentration_kg_m3_{class:04}`, `hs{ID}_particle_diameter_m_{class:04}`, `hs{ID}_particle_flow_fraction_{class:04}` |
| Upstream node payloads | `ws10_channel_{id}_qpo`, `ws10_channel_{id}_durrof`, `ws10_impoundment_{id}_qo`, `ws10_impoundment_{id}_durout` |
| Channel published outputs | `ws10_channel_{id}_qpo`, `ws10_channel_{id}_durrof`, `ws10_channel_{id}_roff` |
| Canonical wave-routing internals | `q1(it)`, `qin(it)`, `qlat(it)`, `qref`, `ckref`, `c0`, `c1`, `c2`, `c3`, `c4`, `chvol` |

### WS11 Legacy-Equivalent Routing Steps

1. Assemble channel runon volume and duration using legacy-equivalent runon
   assembly (`rvolat`, `rvotop`, `rvolon`) and max-duration selection semantics
   (`watdur = max(...)`) before outlet method selection.
2. Apply legacy-equivalent channel runoff/transmission-loss case logic (Cases
   I-IV) before routing branch execution, retaining explicit branch identity.
3. Select exactly one outlet branch via `ipeak`:
   - `ipeak = 1`: modified Rational branch.
   - `ipeak = 2`: modified CREAMS branch.
   - `ipeak = 3`: linear kinematic-wave channel routing with segment update
     relation `qs(is,it) = (dtdx*qs(is-1,it) + cqa*qs(is,it-1) + dtchr*qlavg) / (dtdx + cqa)`.
   - `ipeak >= 4`: Muskingum-Cunge routing with segment update relation
     `qs(is,it) = c1*qs(is-1,it) + c2*qs(is-1,it-1) + c3*qs(is,it-1) + c4`.
4. Preserve legacy-equivalent non-local-runoff routing behavior: when local
   channel runoff is absent but `ipeak > 2`, route incoming hydrograph through
   the wave-routing branch rather than forcing zero outputs.
5. Publish routed outputs with explicit closure:
   - `qpo = peakot`,
   - `roff = runvol`,
   - `durrof = roff / qpo` when `qpo > 1e-12`, else `durrof = 0`.

### WS11 Coupling Rules

1. WS11 channel production execution consumes parser-projected controls,
   contributor payloads, and upstream node payloads; missing required symbols
   are typed hard failures.
2. WS11 routing authority must not reduce outlet routing to the pre-WS11
   gain-factor surrogate `(1 + ctlslp) / (1 + chnn)` or any equivalent
   single-gain reduction.
3. `ipeak` branch selection is mandatory and explicit; method mixing or implicit
   fallback between Rational/CREAMS/wave-routing branches is invalid.
4. Published channel outputs (`qpo`, `durrof`, `roff`) are deterministic,
   finite, and non-negative with declared closure semantics.
5. Downstream consumers must use upstream node payload symbols explicitly and
   fail hard if dependency payloads are missing, non-finite, or out-of-domain.

### WS11 Typed Guard Codes

| Condition | Code |
|---|---|
| Missing required symbol | `WKERNEL-WS10-CHANNEL-E-001` |
| Non-finite symbol | `WKERNEL-WS10-CHANNEL-E-002` |
| Domain/dependency violation | `WKERNEL-WS10-CHANNEL-E-003` |

### WS11 Contract-Derived Test Vectors

Minimum WS11 routing conformance vectors:
1. `ipeak = 1` route path executes modified Rational branch and emits finite
   non-negative `ws10_channel_{id}_qpo`, `ws10_channel_{id}_durrof`,
   `ws10_channel_{id}_roff`.
2. `ipeak = 2` route path executes CREAMS branch with finite/non-negative
   outputs and method-identity traceability.
3. `ipeak = 3` route path executes linear kinematic-wave routing and preserves
   routed closure (`roff = qpo * durrof` within tolerance).
4. `ipeak >= 4` route path executes Muskingum-Cunge routing (including the
   `ipeak = 5` variable-parameter branch when configured) and preserves routed
   closure with finite/non-negative outputs.
5. Missing/non-finite/domain-dependency violations fail with preserved guard
   family codes `WKERNEL-WS10-CHANNEL-E-001..003`.

## ARCH22 Typed Production-Surface Addendum

### Typed Runtime Surface Authority

1. Covered production watershed routing interfaces must use typed ARCH22 symbol
   surfaces (`WatershedProductionStateSymbol`,
   `WatershedProductionFluxSymbol`) for boundary resolution.
2. Covered production guard/accessor helper signatures must not accept raw
   `&str` symbol identifiers where typed ARCH22 symbols exist.
3. Node-scoped channel/impoundment dependency symbol families must be resolved
   through typed field builders and explicit node/hillslope identifiers.
4. Typed migration must preserve WS10 routing guard families
   (`WKERNEL-WS10-CHANNEL-E-001..003`) and failure behavior.

### Contract-Derived Migration Vectors

1. Static migration proof: covered routing production accessors use typed
   symbol families, not stringly `&str` parameters.
2. Nominal migration vector: channel routing execution preserves deterministic
   output/state publication under typed symbol resolution.
3. Failure migration vectors: missing/non-finite/domain/dependency violations
   preserve existing typed boundary classes and WS10 guard IDs.

## EROD14 Wave-2 Active Consumer-Coupling Addendum

1. Routing consumers of hillslope sediment exports must preserve Wave-2 class
   enrichment payload semantics for:
   - `sed_frac_*` class-fraction exports,
   - class-wise load closure surfaces (`erod14_gend_*`, `erod14_sedmax_*`),
   - enrichment ratio surface `ER`.
2. Producer ownership for Wave-2 class enrichment remains in `SC-SED-001`;
   routing consumer guard ownership remains `SC-ROUTE-001` under
   `INV-ROUTE-011` coupling completeness.
3. Missing/non-finite/domain-invalid Wave-2 enrichment payloads must hard-fail;
   consumer-side fallback defaults, truncation, or implicit renormalization are
   prohibited.
4. Wave-2 coupling continuity does not alter WS10 channel guard-family IDs
   (`WKERNEL-WS10-CHANNEL-E-001..003`) for routing boundary failures.

## EROD15 Wave-3 HBP Contributor-Payload Coupling Addendum

1. WS10 production routing intake must validate the full contributor-scoped HBP
   sediment payload family before channel or impoundment routing proceeds:
   - `hs{ID}_total_detachment_kg`,
   - `hs{ID}_total_deposition_kg`,
   - `hs{ID}_particle_class_count`,
   - `hs{ID}_sediment_concentration_kg_m3_{class:04}`,
   - `hs{ID}_particle_flow_fraction_{class:04}`.
2. `hs{ID}_particle_class_count` must be finite, integer-valued, and strictly
   positive; class-indexed concentration/fraction fields are required for every
   class in `1..particle_class_count`.
3. Concentration/fraction payload fields must be finite and non-negative; all
   missing/non-finite/out-of-domain payloads are hard-fail routing boundary
   states under `INV-ROUTE-011`.
4. Wave-3 coupling continuity preserves WS10 guard-family IDs
   (`WKERNEL-WS10-CHANNEL-E-001..003`, `WKERNEL-WS10-IMPOUNDMENT-E-001..003`)
   for routing boundary failures.
5. Routing consumers must not synthesize fallback sediment payload values when
   contributor payload fields are absent or invalid.

## EROD16 Hillslope ROUTE Scope-Partition Addendum

1. Baseline `route.for` provenance from `CONTIN` is explicitly classified as
   hillslope sediment-routing authority under `SC-SED-001`
   (`REF-ROUTE-LEGACY-HSROUTE-BOUNDARY`).
2. This routing contract (`SC-ROUTE-001`) remains authoritative for
   watershed/channel routing branches (`wshpek`, `wshchr`, WS10 symbol
   families) and for consumer-side validation of hillslope contributor payload
   completeness under `INV-ROUTE-011`.
3. `rtpart.for` is explicitly excluded from routing provenance in this domain;
   it remains a plant/root partitioning routine (`REF-ROUTE-LEGACY-RTPART-BOUNDARY`).
4. Contributor-payload alias continuity for WS10 intake remains unchanged and
   authoritative in this contract:
   - `hs{ID}_total_detachment_kg`
   - `hs{ID}_total_deposition_kg`
   - `hs{ID}_particle_class_count`
   - `hs{ID}_sediment_concentration_kg_m3_{class:04}`
   - `hs{ID}_particle_flow_fraction_{class:04}`
5. Any implementation package claiming watershed-routing closure must not use
   hillslope `route.for` presence/absence as a WS10 branch-conformance signal;
   hillslope branch parity is governed by `SC-SED-001` migration gaps.

## Gap Register

| Gap ID | Statement | Impact | Promotability | Evidence |
|---|---|---|---|---|
| GAP-ROUTE-001 | Per-invariant comparator vectors for watershed/channel Tier-B invariant families remain uncurated, and this residual automation limitation is explicitly risk-accepted for current governance progression. | Automated per-invariant acceptance remains limited; manual comparator interpretation is required where vectors are absent. | closed | `[DIRECT][Static]` |
| GAP-ROUTE-002 | Wave-0 erosion-lane alias-ownership ambiguity for required routing boundary symbols is explicitly dispositioned by canonical EROD11 alias ownership registers. | Alias-ownership ambiguity closure is complete for required boundary symbols; production erosion physics remains separately `HOLD`-gated by non-promotable companion/process gaps. | closed | `[DIRECT][Static] + [Ran]` |
| GAP-ROUTE-003 | EROD12 ratifies cross-domain ownership and guard semantics for required erosion-lane routing boundaries across `SC-HYDRAULICS-001`, `SC-SED-001`, `SC-ROUTE-001`, and `SC-SYSTEM-001`; downstream WS10/impoundment ownership paths remain explicitly guarded by their companion contracts. | Required Wave-0 ownership ambiguity is closed for routing-coupled erosion boundaries; non-Wave-0 scope/applicability holds remain governed by other gap rows. | closed | `[DIRECT][Static] + [Ran]` |
| GAP-ROUTE-004 | Chapter-13 mixed-unit and regression-derived formulation caveats remain and are explicitly retained as documented limitations with governance risk acceptance. | Unit-conversion and regression-lineage interpretation risk remains and requires explicit review in sensitive analyses; this is accepted as a model-governance limitation. | closed | `[DIRECT][Static] + [INFERENCE][Static]` |
| GAP-ROUTE-005 | Runtime workload guards for Chapter-13 applicability limits (small watershed intent and excluded process classes) are not yet bound to a concrete input-contract validator surface. | Applicability enforcement is governance-only until companion system/input contracts add explicit runtime selectors/guards. | non-promotable | `[DIRECT][Static] + [INFERENCE][Static]` |
| GAP-ROUTE-006 | WS11 wave-routing branch authority is anchored to pinned legacy static-code provenance (`wshcqi`, `wshdrv`, `wshpek`, `wshchr`) pending companion documentation that cross-indexes non-chapter method-lineage references in one canonical note. | Migration authority is executable and explicit, but review burden for non-chapter lineage remains elevated until companion documentation lands. | promotable-with-risk | `[DIRECT][Static] + [INFERENCE][Static]` |
| GAP-ROUTE-007 | Legacy provenance confusion between watershed routing and hillslope `CONTIN -> ROUTE` branch logic required explicit scope partitioning; EROD16 closes the documentation ambiguity but downstream hillslope runtime parity remains governed by `SC-SED-001` queue stages. | Prevents false attribution of hillslope branch parity status to WS10 routing closure decisions. | closed | `[DIRECT][Static] + [INFERENCE][Static]` |
| GAP-ROUTE-008 | WS11 runtime branch closure is still incomplete in openWEPP watershed execution surfaces: legacy-equivalent channel runon/runoff routine families (`wshcqi/wshirs/wshrun`) are not yet migrated as production kernel behavior. WSHED05 has closed the required `ipeak > 2` wave-routing state-family publication requirement (`q1/qin/qlat/c0..c4`) for WS10 channel outputs. | Contract authority is explicit and wave-state publication closure is active, but full runtime parity claims for channel hydrology/routing remain blocked pending runon/runoff routine-chain migration and downstream end-to-end validation closure. | non-promotable | `[DIRECT][Static] + [INFERENCE][Static]` |
| GAP-ROUTE-009 | Full watershed channel sediment routing process families (`chnero/chnrt/detach`) are not yet migrated into openWEPP watershed production kernels. WSHED06 closed WS11 publication-family symbols (`ws10_channel_{id}_qsed`, `ws10_channel_{id}_tc`), WSHEDIMPL15 added required WS15 runtime control projection + baseline conversion scaffold publication (`crsh/depmid/depsid`) with fail-closed guard continuity, WSHEDIMPL16 added fail-closed contributor `particle_diameter_m` payload projection for transport-capacity lineage ingress, WSHEDIMPL17 added fail-closed WS17 segment/hydraulic scaffold projection/guard closure (`nslpts`, segment `x/slope/depa/depb/wida/widb`), WSHEDIMPL18 migrated baseline `shield`/`trncap` transport-capacity authority into WS10 channel sediment publication so `tc` no longer collapses to surrogate identity (`tc = qsed`), WSHEDIMPL19 added fail-closed channel sediment branch payload export (`particle_class_count`, `particle_flow_fraction_{class:04}`, `particle_diameter_m_{class:04}`) plus upstream channel-dependency payload ingress for WS10 class-aware aggregation, WSHEDIMPL20 added opt-in WS20 segment-loop `case12` routing scaffolding with explicit unresolved-detachment diagnostics publication, WSHEDIMPL21 added WS21 case34/enddet diagnostics scaffolding behind opt-in controls, WSHEDIMPL22 replaced WS21 opt-in unresolved fallback behavior with baseline-lineage `dcap` + `case34/enddet` execution plus required fail-closed `crfrac` projection gating (`ws10_channel_{id}_crfrac_{class:04}`), WSHEDIMPL23 migrated baseline-authoritative `detach.for` iterative closure behavior for WS21 `case4` rows (`nt < cnpart`) so residual WS21 unresolved-detachment diagnostics are no longer required for that branch, WSHEDIMPL24 migrated baseline-authoritative `case12.for` deposition-to-detachment transition continuation (`xdemax < x(i)` into `detach.for`) with explicit WS24 transition diagnostics publication (`ws24_case2_detach_segment_count`), WSHEDIMPL25 closed residual WS20 opt-in unresolved-detachment fallback behavior by auto-activating WS21 migration lanes under WS20 opt-in with fail-closed `crfrac` seam enforcement in WS20-only opt-in lanes, WSHEDIMPL26 migrated baseline-authoritative `dcap(flagm=2)` max-detachment limiter semantics for WS23 iterative detach closure lanes, WSHEDIMPL27 migrated baseline-authoritative `enddet.for` bracket progression semantics (`xdbig/xdsmal`) for WS21 case4 enddet closure lanes, WSHEDIMPL28 migrated baseline-authoritative `chnrt.for` segment boundary-width semantics (`widb(i-1)` upper boundary, `wida(i)` lower boundary) in WS20 segment-loop routing lanes, WSHEDIMPL29 migrated rectangular-channel width-mutation semantics by projecting `dcap` eroded-width outcomes (`werb`) into runtime `widb(i-1)` updates and node-state writeback symbols (`ws10_channel_{id}_widb_{point:04}`), WSHEDIMPL30 migrated baseline-authoritative erodible-lane shape-transition continuity by activating `ishape=3` routing pathways plus `depa/depb`-driven rectangular fallback mapping for WS20/WS21 hydraulic and detach-capacity calls, WSHEDIMPL31 migrated baseline-authoritative lower-boundary width-mutation semantics by projecting `detach.for` eroded-width outcomes (`wera`) into rectangular-lane `wida(i)` updates (`flagc=2`, `wera>wfl`) and state-symbol writeback (`ws10_channel_{id}_wida_{point:04}`), and WSHEDIMPL32 reconciled parser/runtime naturally eroded shape-class lineage by aligning watershed channel parser projection and WS10 runtime consumption on explicit `ishape=3` mapping semantics (strict domain `1..=3`, compatibility `ishape>3 -> 3`). Remaining baseline-authoritative closure for full `chnero/chnrt` parity semantics remains open. | Channel sediment continuity/detachment/deposition parity and downstream sediment publication claims remain blocked pending completion of remaining `chnero/chnrt/detach` segment-process migration families and end-to-end validation closure. | non-promotable | `[DIRECT][Static] + [INFERENCE][Static]` |

## Revision History

| Date UTC | Version | Author | Change |
|---|---|---|---|
| `2026-05-20` | `0` | `Codex` | Initial canonical stub created by SCI-15 work-package prep. |
| `2026-05-20` | `1` | `Codex` | Full draft authored with Chapter-13 authority anchors, invariants, guard map, alias map, obligations, tolerances, and gap register for SCI-15 review cycle. |
| `2026-05-20` | `2` | `Codex` | Post-review amendment pass: made outlet method selection explicitly exclusive, promoted `roff <= 0.001 m^3` peak-threshold gating into invariant/guard tables, added `durrof` alias coverage, and added Chapter-13 applicability-bound governance controls. |
| `2026-05-23` | `3` | `Codex` | WB16 amendment: added hillslope WB16 peak-flow intake authority (`peakro`, `watdur`) plus typed guard and traceability requirements for watershed routing coupling readiness. |
| `2026-05-23` | `4` | `Codex` | WS10 amendment: added watershed production-kernel runtime alias surfaces (`ws10_channel_*`, dependency payloads), typed WS10 routing guard family (`WKERNEL-WS10-CHANNEL-E-001..003`), and contract-derived WS10 routing test-vector obligations. |
| `2026-05-23` | `5` | `Codex` | ARCH22 amendment: added typed production-surface authority requiring covered WS10 routing interfaces to consume boundary symbols via ARCH22 typed symbol families (including node-scoped builders) while preserving WS10 guard-family continuity. |
| `2026-05-23` | `6` | `Codex` | EROD11 amendment: ratified Wave-0 alias ownership across WB16 contributor intake and WS10 routing outputs, added explicit cross-contract ownership register, and downgraded `GAP-ROUTE-002` from non-promotable to promotable-with-risk pending `EROD15` internal alias expansion. |
| `2026-05-23` | `7` | `Codex` | EROD11 closure amendment: dispositioned alias-ownership ambiguity row `GAP-ROUTE-002` to `closed` for required boundary symbols and made explicit that erosion-physics implementation remains separately governed by non-promotable holds. |
| `2026-05-23` | `8` | `Codex` | EROD11 risk-acceptance amendment: dispositioned `GAP-ROUTE-001` and `GAP-ROUTE-004` from promotable-with-risk to `closed` via explicit governance risk acceptance while preserving non-promotable erosion-physics HOLD posture. |
| `2026-05-23` | `9` | `Codex` | EROD12 amendment: added cross-domain ownership/guard closure addendum and dispositioned `GAP-ROUTE-003` to `closed` for required erosion-lane routing boundaries while retaining non-Wave-0 applicability holds. |
| `2026-05-24` | `10` | `Codex` | WS11 amendment: replaced WS10 gain-factor surrogate routing authority with legacy-equivalent channel-routing branch authority (`ipeak`-selected Rational/CREAMS/KW/MC), added pinned baseline provenance anchors, expanded routing closure invariants, and published WS11 contract-derived vector obligations while preserving existing channel guard-family IDs. |
| `2026-05-25` | `11` | `Codex` | EROD14 amendment: added active Wave-2 consumer-coupling authority for hillslope enrichment payload continuity (`sed_frac_*`, `ER`, class-wise closure surfaces) with explicit hard-fail posture for malformed boundary payloads. |
| `2026-05-25` | `12` | `Codex` | EROD15 amendment: added Wave-3 HBP contributor-payload intake authority (`hs{ID}_total_detachment_kg`, `hs{ID}_total_deposition_kg`, class-counted concentration/fraction arrays) with explicit WS10 guard continuity under `INV-ROUTE-011`. |
| `2026-05-26` | `13` | `Codex` | EROD16 amendment: added explicit scope partitioning between watershed routing authority and hillslope `CONTIN -> ROUTE` sediment-branch authority, corrected `rtpart.for` provenance classification, and ratified boundary continuity requirements for WS10 contributor-payload aliases. |
| `2026-05-27` | `14` | `Codex` | WSHEDIMPL01 amendment: added explicit `chrqin.for` wave-routing lineage anchor, normalized watershed open-gap rows for unresolved WS11 runtime branch migration (`wshcqi/wshirs/wshrun`, `ipeak>2` state families), and opened explicit channel sediment runtime closure dependency (`chnero/chnrt/detach`) for WSHED05/WSHED06 sequencing. |
| `2026-05-27` | `15` | `Codex` | WSHEDIMPL05 amendment: ratified closure of WS11 `ipeak>2` wave-routing state-family publication (`q1/qin/qlat/c0..c4`) in production WS10 channel outputs and narrowed `GAP-ROUTE-008` to remaining `wshcqi/wshirs/wshrun` routine-chain migration plus downstream validation closure. |
| `2026-05-27` | `16` | `Codex` | WSHEDIMPL06 amendment: ratified WS11 channel sediment publication-family closure (`ws10_channel_{id}_qsed`, `ws10_channel_{id}_tc`) with typed guard continuity and narrowed `GAP-ROUTE-009` to remaining full `chnero/chnrt/detach` process-parity migration and validation closure. |
| `2026-05-27` | `17` | `Codex` | WSHEDIMPL15 amendment: ratified WS15 channel-sediment control projection and baseline conversion scaffold publication (`ws10_channel_{id}_{crsh,depmid,depsid}` plus `chz/nbarch`) with fail-closed guard continuity while preserving non-promotable `GAP-ROUTE-009` posture until full `chnero/chnrt/detach` process-parity migration closes. |
| `2026-05-27` | `18` | `Codex` | WSHEDIMPL16 amendment: ratified contributor `particle_diameter_m` payload ingress projection (`hs{ID}_particle_diameter_m_{class:04}`) with fail-closed WS10 guard continuity, and narrowed `GAP-ROUTE-009` to remaining full `chnero/chnrt/detach` segment-process migration closure scope. |
| `2026-05-27` | `19` | `Codex` | WSHEDIMPL17 amendment: ratified WS17 segment/hydraulic scaffold projection/guard closure (`ws10_channel_{id}_nslpts` + segment `x/slope/depa/depb/wida/widb` families) and narrowed `GAP-ROUTE-009` to remaining full `chnero/chnrt/detach` kernel process-family migration scope. |
| `2026-05-27` | `20` | `Codex` | WSHEDIMPL18 amendment: migrated baseline `shield`/`trncap` transport-capacity authority into WS10 channel sediment publication (`tc`) using class-aware contributor payload aggregation and removed surrogate `tc=qsed` identity coupling, while preserving non-promotable `GAP-ROUTE-009` posture for unresolved segment-loop detachment/deposition families (`case12/case34/detach/dcap/enddet`) and full `chnero/chnrt` parity closure. |
| `2026-05-27` | `21` | `Codex` | WSHEDIMPL19 amendment: ratified fail-closed WS10 channel sediment branch payload export (`particle_class_count`, `particle_flow_fraction_{class:04}`, `particle_diameter_m_{class:04}`) and upstream channel-dependency payload ingress for class-aware aggregation continuity, while preserving non-promotable `GAP-ROUTE-009` posture for unresolved segment-loop detachment/deposition families (`case12/case34/detach/dcap/enddet`) and full `chnero/chnrt` inflow-partition parity closure. |
| `2026-05-27` | `22` | `Codex` | WSHEDIMPL20 amendment: added opt-in WS20 segment-loop `case12` routing scaffolding and explicit unresolved-detachment diagnostics publication (`ws20_case1_segment_count`, `ws20_case2_segment_count`, `ws20_detachment_unmigrated_segment_count`) while preserving non-promotable `GAP-ROUTE-009` posture for remaining baseline-authoritative detachment/deposition families (`case34/detach/dcap/enddet`) and full `chnero/chnrt` parity closure. |
| `2026-05-27` | `23` | `Codex` | WSHEDIMPL21 amendment: added WS10 opt-in WS21 case34/enddet diagnostics scaffolding (`ws21_case3_segment_count`, `ws21_case4_segment_count`, `ws21_enddet_segment_count`, `ws21_detach_unmigrated_segment_count`) while preserving non-promotable `GAP-ROUTE-009` posture for remaining baseline-authoritative `detach/dcap` migration and full `chnero/chnrt` parity closure. |
| `2026-05-27` | `24` | `Codex` | WSHEDIMPL22 amendment: replaced WS21 opt-in unresolved fallback with baseline-lineage `dcap` + `case34/enddet` execution and required fail-closed `crfrac` projection gating (`ws10_channel_{id}_crfrac_{class:04}`), while preserving non-promotable `GAP-ROUTE-009` posture for residual WS21 `case4 -> detach` iterative closure (`nt < cnpart`) and remaining full `chnero/chnrt` parity closure. |
| `2026-05-27` | `25` | `Codex` | WSHEDIMPL23 amendment: migrated baseline-authoritative `detach.for` iterative closure behavior for WS21 `case4` rows (`nt < cnpart`) and removed residual WS21 unresolved-detachment fallback requirement for that branch, while preserving non-promotable `GAP-ROUTE-009` posture for remaining full `chnero/chnrt` parity closure families. |
| `2026-05-27` | `26` | `Codex` | WSHEDIMPL24 amendment: migrated baseline-authoritative `case12.for` deposition-to-detachment transition continuation (`xdemax < x(i)` into `detach.for`) in WS20 segment-loop routing and added explicit transition diagnostics publication (`ws24_case2_detach_segment_count`), while preserving non-promotable `GAP-ROUTE-009` posture for remaining full `chnero/chnrt` parity closure families. |
| `2026-05-27` | `27` | `Codex` | WSHEDIMPL25 amendment: closed residual WS20 opt-in unresolved-detachment fallback behavior by auto-activating WS21 migration lanes under WS20 opt-in and enforcing fail-closed `crfrac` requirements for WS20-only opt-in lanes, while preserving non-promotable `GAP-ROUTE-009` posture for remaining full `chnero/chnrt` parity closure families. |
| `2026-05-27` | `28` | `Codex` | WSHEDIMPL26 amendment: migrated baseline-authoritative `dcap(flagm=2)` max-detachment limiter semantics for WS23 iterative detach closure lanes and added explicit residual-branch mapping continuity while preserving non-promotable `GAP-ROUTE-009` posture for remaining full `chnero/chnrt` parity closure families. |
| `2026-05-27` | `29` | `Codex` | WSHEDIMPL27 amendment: migrated baseline-authoritative `enddet.for` bracket progression semantics (`xdbig/xdsmal`) for WS21 case4 enddet closure lanes and preserved non-promotable `GAP-ROUTE-009` posture for remaining full `chnero/chnrt` parity closure families. |
| `2026-05-27` | `30` | `Codex` | WSHEDIMPL28 amendment: migrated baseline-authoritative `chnrt.for` segment boundary-width semantics (`widb(i-1)` upper boundary, `wida(i)` lower boundary) in WS20 segment-loop routing lanes and preserved non-promotable `GAP-ROUTE-009` posture for remaining full `chnero/chnrt` parity closure families. |
| `2026-05-27` | `31` | `Codex` | WSHEDIMPL29 amendment: migrated baseline-authoritative rectangular-channel width-mutation semantics by projecting `dcap` eroded-width outcomes (`werb`) into WS20 `widb(i-1)` updates and state-symbol writeback (`ws10_channel_{id}_widb_{point:04}`), while preserving non-promotable `GAP-ROUTE-009` posture for remaining full `chnero/chnrt` parity closure families. |
| `2026-05-28` | `32` | `Codex` | WSHEDIMPL30 amendment: migrated baseline-authoritative erodible-lane shape-transition continuity by activating `ishape=3` routing pathways and applying `depa/depb`-driven rectangular fallback mapping for WS20/WS21 hydraulic and detach-capacity calls, while preserving non-promotable `GAP-ROUTE-009` posture for remaining full `chnero/chnrt` parity closure families. |
| `2026-05-28` | `33` | `Codex` | WSHEDIMPL31 amendment: migrated baseline-authoritative lower-boundary width-mutation continuity (`flagc=2`, `wera>wfl`) by projecting detach eroded-width outcomes (`wera`) into WS20 rectangular-lane `wida(i)` updates and state-symbol writeback (`ws10_channel_{id}_wida_{point:04}`), while preserving non-promotable `GAP-ROUTE-009` posture for remaining full `chnero/chnrt` parity closure families. |
| `2026-05-28` | `34` | `Codex` | WSHEDIMPL32 amendment: reconciled parser/runtime naturally eroded shape-class lineage by aligning watershed channel parser projection and WS10 runtime consumption on explicit `ishape=3` mapping semantics (strict domain `1..=3`, compatibility `ishape>3 -> 3`), while preserving non-promotable `GAP-ROUTE-009` posture for remaining full `chnero/chnrt` parity closure families. |
