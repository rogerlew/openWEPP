---
contract_id: SC-ROUTE-001
title: Watershed Routing and Channel Process Contract
status: in_review
maturity: draft
owner: openWEPP maintainers + hydrology reviewer
contract_version: 8
producer_scope:
  - Channel runon/runoff volume routing and transmission-loss accounting surfaces
  - Channel peak-discharge and duration routing surfaces at inlet/outlet boundaries
  - Channel sediment continuity and detachment/deposition boundary surfaces
consumer_scope:
  - Watershed downstream channel and outlet routing consumers
  - Impoundment and watershed-node consumers requiring channel flux/state payloads
  - Comparator/replay surfaces using watershed confidence-tier signals
evidence_level: static
last_reviewed: 2026-05-23
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
  hillslope erosion internals owned by `SC-SED-001`, and impoundment internals
  owned by `SC-IMPOUND-001` except explicit coupling boundaries. `[INFERENCE][Static]`

## Authority Anchors

| Anchor ID | Source anchor | Contract use | Evidence |
|---|---|---|---|
| REF-ROUTE-CH13-RUNON | `references/50201000/chap13.pdf` §13.2 Eq. [13.2.1]-[13.2.3] | Channel runon decomposition, runon-depth conversion, and event-duration selection. | `[DIRECT][Static]` |
| REF-ROUTE-CH13-TLOSS | `chap13.pdf` §13.2 Eq. [13.2.4]-[13.2.6] + Case I-IV text | Transmission-loss accounting and runoff-case branch semantics for `qci`, `qcf`, and `tl`. | `[DIRECT][Static]` |
| REF-ROUTE-CH13-PEAKIN | `chap13.pdf` §13.4.1 Eq. [13.4.1]-[13.4.2] | Triangular synthetic hydrograph inlet-peak superposition method for multi-source inflow. | `[DIRECT][Static]` |
| REF-ROUTE-CH13-RAT | `chap13.pdf` §13.4.2.1 Eq. [13.4.3]-[13.4.24] | Modified Rational outlet-peak method, travel-time decomposition, and alpha selection rules. | `[DIRECT][Static]` |
| REF-ROUTE-CH13-CREAMS | `chap13.pdf` §13.4.2.2 Eq. [13.4.25] | CREAMS statistical outlet-peak method. | `[DIRECT][Static]` |
| REF-ROUTE-CH13-DUR | `chap13.pdf` §13.4.3 Eq. [13.4.26] | Effective runoff-duration computation from volume and outlet peak. | `[DIRECT][Static]` |
| REF-ROUTE-CH13-SVF | `chap13.pdf` §13.5.2 Eq. [13.5.1]-[13.5.5] | Spatially-varied flow and friction-slope relationships used by channel erosion routines. | `[DIRECT][Static]` |
| REF-ROUTE-CH13-EFFLEN | `chap13.pdf` §13.5.3 Eq. [13.5.6]-[13.5.12] | Effective channel-length and discharge-distribution semantics for segment routing. | `[DIRECT][Static]` |
| REF-ROUTE-CH13-SHEAR | `chap13.pdf` §13.5.4 Eq. [13.5.13]-[13.5.16] | Shear stress partition between soil and vegetation and detachment-driving stress terms. | `[DIRECT][Static]` |
| REF-ROUTE-CH13-CONT | `chap13.pdf` §13.5.5 Eq. [13.5.17]-[13.5.18] | Quasi-steady sediment continuity and inlet/lateral sediment load assembly semantics. | `[DIRECT][Static]` |
| REF-ROUTE-CH13-DETDEP | `chap13.pdf` §13.5.6 Eq. [13.5.19]-[13.5.29] | Detachment-capacity, deposition, and transport-capacity branch logic for segment updates. | `[DIRECT][Static]` |
| REF-ROUTE-CH13-LIMIT | `chap13.pdf` §13.6 summary limitations | Applicability bounds: intended small agricultural watersheds and explicit limitations (no partial-area response, no headcutting, no bank sloughing, no perennial streams). | `[DIRECT][Static]` |
| REF-ROUTE-CH4-COUPLING | `references/50201000/chap4.pdf` Eq. [4.2.1]-[4.2.9], [4.3.1]-[4.3.5], [4.4.27]-[4.4.29], [4.5.4], [4.5.6] | Channel hydrology uses hillslope infiltration/rainfall-excess and recession-infiltration relationships by explicit Chapter-13 linkage. | `[DIRECT][Static] + [INFERENCE][Static]` |
| REF-ROUTE-CH5-COUPLING | `references/50201000/chap5.pdf` §5.1-§5.4 and `chap13.pdf` §13.3 | Channel water-balance/percolation routines are stated as identical to hillslope routines. | `[DIRECT][Static]` |
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
| INV-ROUTE-006 | Outlet-peak method invariant: channel outlet peak discharge `qpo` must be computed by one explicitly selected outlet method (modified Rational Eq. [13.4.3]-[13.4.24] or CREAMS Eq. [13.4.25]); method mixing or silent fallback between methods is invalid, and all selected-method inputs must be finite and unit-consistent. | hard-fail | REF-ROUTE-CH13-RAT, REF-ROUTE-CH13-CREAMS | `[DIRECT][Static] + [INFERENCE][Static]` |
| INV-ROUTE-007 | Peak-threshold/duration invariant: if `roff <= 0.001 m^3`, then peak runoff and runoff duration are both zero per §13.4.1; otherwise `qpo` must be strictly positive and effective runoff duration must satisfy Eq. [13.4.26] with positive-domain consistency between `roff`, `qpo`, and `durrof`. | hard-fail | REF-ROUTE-CH13-DUR, REF-ROUTE-CH13-RAT, REF-ROUTE-CH13-CREAMS, REF-ROUTE-PHYS-BOUNDS | `[DIRECT][Static] + [INFERENCE][Static]` |
| INV-ROUTE-008 | Spatially-varied flow/shear invariant: channel erosion solver must use consistent spatially-varied flow outputs (`Sf`, `Sstar`, `leff`, `q`) to compute shear terms, and soil shear relation Eq. [13.5.13]-[13.5.16] must preserve finite physically valid domains. | hard-fail | REF-ROUTE-CH13-SVF, REF-ROUTE-CH13-SHEAR | `[DIRECT][Static] + [INFERENCE][Static]` |
| INV-ROUTE-009 | Sediment continuity invariant: quasi-steady sediment continuity Eq. [13.5.17]-[13.5.18] must be conserved across segments and particle classes with explicit inlet (`qsed_top`) and lateral (`qsed_lat`) source accounting. | hard-fail | REF-ROUTE-CH13-CONT | `[DIRECT][Static]` |
| INV-ROUTE-010 | Detachment/deposition branch invariant: detachment capacity Eq. [13.5.19]/[13.5.20], deposition Eq. [13.5.21]-[13.5.22], and transport-capacity branch iteration semantics from §13.5.6 must be explicit; silent branch collapse is invalid. | hard-fail | REF-ROUTE-CH13-DETDEP | `[DIRECT][Static] + [INFERENCE][Static]` |
| INV-ROUTE-011 | Coupling completeness invariant: required hillslope/impoundment/channel handoff payloads (runon volumes, durations, peak flow, sediment class fluxes) must be present and parseable before routing calculations proceed. | hard-fail | REF-ROUTE-CH13-RUNON, REF-ROUTE-CH13-CONT, REF-ROUTE-CH4-COUPLING, REF-ROUTE-CH5-COUPLING | `[DIRECT][Static] + [INFERENCE][Static]` |
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
| `INV-ROUTE-006` | runtime | Outlet-peak method selector/calculator | Typed hard error on mixed/implicit method selection, missing selected-method inputs, or non-finite peak output | Tier-B investigation gate | `[DIRECT][Static] + [INFERENCE][Static]` |
| `INV-ROUTE-007` | runtime | Peak-threshold + duration post-processor | Typed hard error on threshold-branch violation or invalid `roff`/`qpo`/`durrof` relation | Tier-B investigation gate | `[DIRECT][Static] + [INFERENCE][Static]` |
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

## EROD11 Alias Ownership Register

| Boundary ID | Canonical symbols | Runtime alias surface | Producer ownership | Consumer ownership | Evidence |
|---|---|---|---|---|---|
| `EROD-BND-001` | `hs{ID}_peakro`, `hs{ID}_watdur` | `WatershedProductionStateSymbol::{HillslopeContributorPeak,HillslopeContributorDuration}` | `SC-RUNOFFPART-001` + `SC-WATBAL-001` via WB16 coupling | `SC-ROUTE-001` WS10 intake guards (`INV-ROUTE-011`) | `[DIRECT][Static] + [INFERENCE][Static]` |
| `EROD-BND-003` | `sed_det_total`, `sed_dep_total`, `sed_conc_i`, `sed_frac_i` | canonical identity boundary symbols (runtime projection owner deferred under erosion-physics `HOLD`) | `SC-SED-001` | `SC-ROUTE-001` segment/channel consumers | `[DIRECT][Static] + [INFERENCE][Static]` |
| `EROD-BND-004` | `ws10_channel_{id}_qpo`, `ws10_channel_{id}_durrof`, `ws10_channel_{id}_roff` | `WatershedProductionStateSymbol::ChannelNode`; `WatershedProductionFluxSymbol::ChannelNode` | `SC-ROUTE-001` | downstream channel/impoundment/watershed consumers | `[DIRECT][Static] + [INFERENCE][Static]` |

## Allowed Degenerate States

| Degenerate state | Allowed condition | Rationale | Evidence |
|---|---|---|---|
| No channel flow event | Case IV (`qci = 0`, `rod = 0`) with `qcf = 0` and `roff = 0`. | Explicit §13.2 case definition. | `[DIRECT][Static]` |
| Runon-only event with infiltration dominance | Case III where `qci = 0`, `rod > 0`, and `fc <= fp` yields `qcf = 0`. | Explicit Eq. [13.2.5] branch condition. | `[DIRECT][Static]` |
| No lateral inflow routing | `qlat = 0` leading to `qu = qpo` and `qlat_eff = 0`. | Explicit Eq. [13.5.10]-[13.5.11] branch semantics. | `[DIRECT][Static]` |
| Channel event below peak-routing threshold | `roff <= 0.001 m^3` yields zero peak runoff and zero runoff duration. | Explicit §13.4.1 threshold branch. | `[DIRECT][Static]` |
| Net deposition segment | Segment state where `qsed > Tc` and Eq. [13.5.21] governs deposition. | Explicit §13.5.6 branch semantics. | `[DIRECT][Static]` |

## Invalid States

- `Ach <= 0` used in Eq. [13.2.2] or negative runon-volume terms (`rol`, `roi`, `rov`) outside declared tolerance. `[DIRECT][Static] + [INFERENCE][Static]`
- `durc` not equal to `max(durrunon, durchan, durirrig)` for the emitted channel event. `[DIRECT][Static]`
- Missing or contradictory Case I-IV branch resolution for (`qci`, `rod`) combinations. `[DIRECT][Static] + [INFERENCE][Static]`
- Transmission-loss algebra implies `roff > (rov + rofc)` or negative physically invalid final runoff/loss outcomes. `[DIRECT][Static] + [INFERENCE][Static]`
- Outlet peak/discharge products emitted with undefined selected-method inputs, mixed-method fallback behavior, or non-finite `qpo`, `tc`, or `durrof` values. `[DIRECT][Static] + [INFERENCE][Static]`
- Threshold branch violation where `roff <= 0.001 m^3` still emits positive `qpo`/`durrof`, or `roff > 0.001 m^3` emits zero peak without explicit authority. `[DIRECT][Static] + [INFERENCE][Static]`
- Shear/transport calculations with invalid domains (`Sf`, `tau`, `taucr`, `Tc`, or segment discharge terms non-finite or non-physical). `[DIRECT][Static] + [INFERENCE][Static]`
- Sediment continuity violation where segment-to-segment `qsed` updates break Eq. [13.5.17] semantics. `[DIRECT][Static]`
- Missing mandatory handoff payload fields (runon, duration, peak, sediment class flux) before routing/erosion calculations. `[DIRECT][Static] + [INFERENCE][Static]`
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

## WS10 Watershed Production-Kernel Addendum

### WS10 Runtime Boundary Symbols

| Surface | Symbols |
|---|---|
| Channel global routing controls | `dtchr`, `nchnum`, `cbase` |
| Channel per-node controls | `ws10_channel_{id}_chnn`, `ws10_channel_{id}_ctlslp`, `ws10_channel_{id}_chnk` |
| Contributor peak payloads | `hs{ID}_peakro`, `hs{ID}_watdur` |
| Upstream node payloads | `ws10_channel_{id}_qpo`, `ws10_channel_{id}_durrof`, `ws10_impoundment_{id}_qo`, `ws10_impoundment_{id}_durout` |
| Channel published outputs | `ws10_channel_{id}_qpo`, `ws10_channel_{id}_durrof`, `ws10_channel_{id}_roff` |

### WS10 Coupling Rules

1. WS10 channel production execution consumes parser-projected per-channel
   controls plus contributor peak payloads and upstream node payloads; missing
   required symbols are typed hard failures.
2. Channel routing execution must keep explicit branch behavior for no-runoff
   domain (`incoming_peak <= 0`) and positive-runoff domain
   (`incoming_peak > 0`) without silent default substitution.
3. Published channel outputs (`qpo`, `durrof`, `roff`) are deterministic and
   finite, and must remain non-negative.
4. Downstream consumers must use upstream node payload symbols explicitly and
   fail hard if dependency payloads are missing, non-finite, or out-of-domain.

### WS10 Typed Guard Codes

| Condition | Code |
|---|---|
| Missing required symbol | `WKERNEL-WS10-CHANNEL-E-001` |
| Non-finite symbol | `WKERNEL-WS10-CHANNEL-E-002` |
| Domain/dependency violation | `WKERNEL-WS10-CHANNEL-E-003` |

### WS10 Contract-Derived Test Vectors

Minimum WS10 routing conformance vectors:
1. Nominal channel execution with hillslope contributor payloads and finite
   parser-projected controls emits finite non-negative
   `ws10_channel_{id}_qpo`, `ws10_channel_{id}_durrof`, `ws10_channel_{id}_roff`.
2. Missing required channel control symbol fails with
   `WKERNEL-WS10-CHANNEL-E-001`.
3. Non-finite required contributor/control symbol fails with
   `WKERNEL-WS10-CHANNEL-E-002`.
4. Domain/dependency violation (e.g., invalid positive-domain parameter or
   unresolved upstream payload) fails with `WKERNEL-WS10-CHANNEL-E-003`.

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

## Gap Register

| Gap ID | Statement | Impact | Promotability | Evidence |
|---|---|---|---|---|
| GAP-ROUTE-001 | Per-invariant comparator vectors for watershed/channel Tier-B invariant families remain uncurated, and this residual automation limitation is explicitly risk-accepted for current governance progression. | Automated per-invariant acceptance remains limited; manual comparator interpretation is required where vectors are absent. | closed | `[DIRECT][Static]` |
| GAP-ROUTE-002 | Wave-0 erosion-lane alias-ownership ambiguity for required routing boundary symbols is explicitly dispositioned by canonical EROD11 alias ownership registers. | Alias-ownership ambiguity closure is complete for required boundary symbols; production erosion physics remains separately `HOLD`-gated by non-promotable companion/process gaps. | closed | `[DIRECT][Static] + [Ran]` |
| GAP-ROUTE-003 | Companion contracts (`SC-HYDRAULICS-001`, `SC-SED-001`, `SC-IMPOUND-001`, `SC-SYSTEM-001`) are not fully authored, so cross-domain ownership boundaries remain provisional. | Promotion-readiness depends on downstream contract completion and consistency. | non-promotable | `[DIRECT][Static]` |
| GAP-ROUTE-004 | Chapter-13 mixed-unit and regression-derived formulation caveats remain and are explicitly retained as documented limitations with governance risk acceptance. | Unit-conversion and regression-lineage interpretation risk remains and requires explicit review in sensitive analyses; this is accepted as a model-governance limitation. | closed | `[DIRECT][Static] + [INFERENCE][Static]` |
| GAP-ROUTE-005 | Runtime workload guards for Chapter-13 applicability limits (small watershed intent and excluded process classes) are not yet bound to a concrete input-contract validator surface. | Applicability enforcement is governance-only until companion system/input contracts add explicit runtime selectors/guards. | non-promotable | `[DIRECT][Static] + [INFERENCE][Static]` |

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
