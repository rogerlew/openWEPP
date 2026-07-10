---
contract_id: SC-ROUTE-001
title: Watershed Routing and Channel Process Contract
status: in_review
maturity: draft
owner: openWEPP maintainers + hydrology reviewer
contract_version: 51
producer_scope:
  - Channel runon/runoff volume routing and transmission-loss accounting surfaces
  - Channel peak-discharge and duration routing surfaces at inlet/outlet boundaries
  - Channel sediment continuity and detachment/deposition boundary surfaces
consumer_scope:
  - Watershed downstream channel and outlet routing consumers
  - Impoundment and watershed-node consumers requiring channel flux/state payloads
  - Comparator/replay surfaces using watershed confidence-tier signals
evidence_level: static
last_reviewed: 2026-07-10
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
| REF-ROUTE-CH13-PEAKIN | `chap13.pdf` §13.4.1 Eq. [13.4.1]-[13.4.2] | Triangular synthetic hydrograph inlet-peak superposition method for multi-source inflow (the INV-ROUTE-005 fallback basis when no contributor carries any hourly surface authority). | `[DIRECT][Static]` |
| REF-ROUTE-ADR0036-HOURLY | [`ADR-0036`](../../../decisions/0036-hydrograph-resolved-sediment-transport-and-routing.md) + `SC-INFILE-HBP-001` §3a | Hour-resolved inlet superposition authority: paired minor-1 `hourly_runoff_volume_m3[24]` (m³) / `hourly_sediment_mass_kg[24]` (kg) EVENT surfaces on a shared 24-slot time base; routing consumes the serialized modeled hydrograph and its sediment timing rather than reconstructing either from event aggregates. | `[DIRECT][Static]` |
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
| REF-ROUTE-CH13-GEOMCARRY | `chap13.pdf` §13.5.1 (p. 13.10: "The ephemeral gully cross-sectional geometry is updated after each precipitation event that causes detachment in order to calculate channel hydraulics for subsequent events.") | Lineage authority that eroded channel geometry state carries forward between solves rather than resetting. | `[DIRECT][Static]` |
| REF-ROUTE-CREAMS-CH3-QS | `references/vendorable/creams/312-ch3.pdf` (+ `312-ch3.md` conversion) Eq. [I-56] and introduction | Primary-source quasi-steady rationale: "The assumption of quasisteady state allows deletion of time terms"; the event-scalar collapse is documented as a compute-cost reduction ("excessive use of computer time practically prohibits simulating 20 to 30 years of record ... a single time step for models which simulate over the entire runoff event"), not a physics claim. Also the `Leff/10` segment-discretization lineage. | `[DIRECT][Static]` |
| REF-ROUTE-CREAMS-CH3-WIDEN | `references/vendorable/creams/312-ch3.pdf` Eq. [I-128]-[I-140] (widening law verified against the rendered scan pp. 54-55, 2026-07-10) | Concentrated-flow detachment (`[I-128]`), incision rate `d_ch = e_m / rho_soil` (`[I-131]`), active-channel loss (`[I-132]`), and the post-nonerodible-layer widening time-evolution law: `omega = 1 - exp(-t_star)` with `omega = (W - W_i)/(W_f - W_i)`, `t_star = (t - t_i)(dW/dt)_i/(W_f - W_i)` (`[I-133]-[I-135]`), initial widening rate (`[I-136]-[I-138]`), and flow-dependent final width `W_f(Q)` (`[I-139]-[I-140]`). | `[DIRECT][Static]` |
| REF-ROUTE-ARS77-SAMEGRID | `references/vendorable/kineros/703.pdf` (+ `703.md` conversion; Woolhiser, Smith & Goodrich 1990, USDA-ARS ARS-77) | External canonical same-grid coupling authority: the sediment mass-balance equation (Bennett 1974 lineage, restated with citation) is solved on the same time/space grid as the kinematic water solution; channels with zero inlet transport capacity deposit incoming lateral sediment. Also the recorded unsteady-advection fallback form. | `[DIRECT][Static]` |
| REF-ROUTE-HECRAS-QUS | `references/vendorable/HEC_RAS_1D_Sediment_Transport_UserManual_20260710.pdf` (USACE web capture; user-manual text carrying the Technical Reference Manual's quasi-unsteady semantics — treated as **class corroboration** co-anchored with REF-ROUTE-ARS77-SAMEGRID and REF-ROUTE-CREAMS-CH3-QS; the formal TRM citation remains pinned for acquisition, bibliography R-107) | External canonical quasi-steady-sequence class authority: a flow hydrograph approximated by a series of steady profiles; the computational increment is the hydraulic and sediment-transport time step; bed geometry updates after each increment and carries to the next, justified when per-increment bed change does not alter hydrodynamics appreciably. | `[DIRECT][Static]` |
| REF-ROUTE-CH14-TIMESTEP | `references/50201000/chap14.pdf` §14.1-§14.2 | Internal lineage precedent that time-resolved sediment routing exists inside the WEPP document family: WEPPSIE impoundment sediment runs per adaptive time step ("the amount of sediment deposited and the outflow concentration for each time step"). | `[DIRECT][Static]` |
| REF-ROUTE-GULLY-STATE | `/workdir/wepp-forest_260430_baseline/src/cgully.inc` + `chncon.for` + `chnrt.for` + `dcap.for` + `detach.for` + `wshdrv.for` (`dac3c950d8b16cc73774bf5ce2e7e11f80baac70`) | Secondary static-code provenance for geometry state carry: `/gully/` COMMON arrays (`depa/depb/wida/widb/wera/werb`) initialized once (`chncon`), mutated in place during the event solve (`dcap`/`detach`/`chnrt`), never event-reset, reseeded only by primary tillage on `ishape=3` channels (`wshdrv.for:1179-1189`); the event-scalar solve basis (`qe = peakot`, `tb = 2*rundur` triangular shear-time surrogate, `gstu = gpart/rundur`). | `[DIRECT][Static]` |
| REF-ROUTE-JIMF2023-CARRY | `/workdir/wepp-forest/docs/jimf-wepp-2023-diff-audit.md` (r1305 rows) | Maintainer-intent evidence (graded corroboration, not physics authority): the 2023 upstream revision added a "channel sediment initialization fix to prevent carryover to following storms" — cross-event sediment-mass carry treated as a defect while geometry carry remains design. | `[DIRECT][Static]` |
| REF-ROUTE-CH13-LIMIT | `chap13.pdf` §13.6 summary limitations | Applicability bounds: intended small agricultural watersheds and explicit limitations (no partial-area response, no headcutting, no bank sloughing, no perennial streams). | `[DIRECT][Static]` |
| REF-ROUTE-RUNFILE-APPLICABILITY | `docs/contracts/openwepp-watershed-runfile-contract.md` (`inputs.applicability` selectors) + `crates/openwepp-runner/src/bin/openwepp-cli-watershed.rs` (`CLIWAT-E-040`) | Concrete input-validator binding for Chapter-13 applicability declarations (`chapter13_small_watershed_intent=true`, excluded-process selectors=false) with typed fail-closed runtime behavior. | `[DIRECT][Static] + [INFERENCE][Static]` |
| REF-ROUTE-CH4-COUPLING | `references/50201000/chap4.pdf` Eq. [4.2.1]-[4.2.9], [4.3.1]-[4.3.5], [4.4.27]-[4.4.29], [4.5.4], [4.5.6] | Channel hydrology uses hillslope infiltration/rainfall-excess and recession-infiltration relationships by explicit Chapter-13 linkage. | `[DIRECT][Static] + [INFERENCE][Static]` |
| REF-ROUTE-CH5-COUPLING | `references/50201000/chap5.pdf` §5.1-§5.4 and `chap13.pdf` §13.3 | Channel water-balance/percolation routines are stated as identical to hillslope routines. | `[DIRECT][Static]` |
| REF-ROUTE-MOFE-HOURLY-CARRY | `SC-WATBAL-001#INV-WATBAL-033`, `SC-RUNOFFPART-001#INV-RUNOFFPART-013`, and `SC-SYSTEM-001#INV-SYSTEM-028` | Routing coupling completeness for multi-OFE hourly hillslope contributors requires explicit carry-array provenance before watershed dispatch. | `[DIRECT][Static] + [INFERENCE][Static]` |
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
| `mofe_hourly_carry` | manifest metadata | Contributor-level evidence that multi-OFE hourly hillslope pass payloads were produced with explicit 24-slot carry arrays rather than aggregate-only runon substitution. | hillslope runner manifest | watershed routing admission validator |
| `V_h`, `S_h` | `m^3`, `kg` | Hour-integrated runoff volume and exported sediment mass (minor-1 HBP EVENT surfaces; ADR-0036 D2), consumed on the interval lane as projection sources only. | hillslope HBP producer (`SC-SED-001`/`SC-INFILE-HBP-001`) | interval-projection assembler (INV-ROUTE-015) |
| `q1(it)`, `ntchr`, `dtchr` | `m^3 s^-1`, count, `s` | Routed interval discharge series, interval count, and interval length on the normalized water grid (`ntchr * dtchr = 86400 s`). | WS11 wave-routing branch | channel-interval sediment lane |
| `W`, `W_i`, `W_f`, `wera`, `werb` | `ft` | Current/anchor/final eroded channel widths and eroded-width state at segment boundaries (CREAMS widening family, lineage realization). | widening-clock evaluator + geometry carrier | subsequent-interval hydraulics and geometry publication |
| `omega`, `t_star` | dimensionless | Nondimensional width and time of the lineage-modified exponential widening law (`wstar = (1 - exp(-1.0176*t_star))/1.0176`). | widening-clock evaluator | widening-state advance |
| `(dW/dt)_i`, `e_m`, `d_ch` | `ft s^-1`, `lb ft^-2 s^-1`, `ft s^-1` | Widening rate basis, maximum-shear erosion rate, and incision rate (`d_ch = e_m / rho_soil`, Eq. [I-131] lineage). | detachment-capacity/widening routines | geometry mutation and detached-mass derivation |
| `rho_soil` | `lb ft^-3` | In-place soil weight density (baseline `wtdsoi` basis, from the channel soil input); the [I-131]/[I-136] denominator. | channel soil-input projection | incision/widening rates and detached-mass derivation |
| `timpot`, `timex` | `s` | Erosion-time budget partition at nonerodible-layer contact: incision-completion time and residual widening time within the contact interval (`dcap` lineage). | widening-clock evaluator | INV-ROUTE-018 budget accounting |

## Invariants

| Invariant ID | Statement | Severity | Authority | Evidence |
|---|---|---|---|---|
| INV-ROUTE-001 | Runon decomposition invariant: channel runon assembly must satisfy Eq. [13.2.1] (`rov = rol + roi`) and Eq. [13.2.2] (`rod = rov / Ach`) with explicit positive-area requirement (`Ach > 0`). | hard-fail | REF-ROUTE-CH13-RUNON, REF-ROUTE-PHYS-BOUNDS | `[DIRECT][Static] + [INFERENCE][Static]` |
| INV-ROUTE-002 | Duration-selection invariant: channel event duration must be selected by Eq. [13.2.3] (`durc = max(durrunon, durchan, durirrig)`) with declared units and no implicit duration fallback. | hard-fail | REF-ROUTE-CH13-RUNON | `[DIRECT][Static] + [INFERENCE][Static]` |
| INV-ROUTE-003 | Runoff-case invariant: Case I-IV branching from §13.2 must be explicit for (`qci`, `rod`) combinations, including Case IV zero-flow branch (`qcf = 0`, `roff = 0`) and Case III branch using Eq. [13.2.5]-[13.2.6]. | hard-fail | REF-ROUTE-CH13-TLOSS, REF-ROUTE-PHYS-BOUNDS | `[DIRECT][Static] + [INFERENCE][Static]` |
| INV-ROUTE-004 | Transmission-loss closure invariant: for Case I/II, transmission losses must satisfy Eq. [13.2.4]; for Case III, losses must satisfy Eq. [13.2.6], and computed losses cannot imply runoff volume greater than entering water volume. | hard-fail | REF-ROUTE-CH13-TLOSS, REF-ROUTE-PHYS-BOUNDS | `[DIRECT][Static] + [INFERENCE][Static]` |
| INV-ROUTE-005 | Inlet-peak superposition invariant (conditional per ADR-0036 D3): (a) when **every** contributing element's payload carries the minor-1 paired hourly surfaces (`hourly_runoff_volume_m3[24]`, `hourly_sediment_mass_kg[24]`) and no upstream dependency element lacks channel-hourly surface authority (dependency-authority definition, W11A: an upstream channel node on the **active interval lane** (INV-ROUTE-015), publishing same-grid per-interval per-class egress, carries channel-hourly surface authority for this clause and clause (c); it is the only non-hourly dependency form that does; impoundment dependency nodes carry no such authority), inlet superposition must be **hour-resolved on the shared time base** — per-hour water and sediment inflows summed across contributors, combined inlet peak = the maximum hour-mean discharge (`max_h(Σ V_h / 3600 s)`), inlet volumes = `Σ_h Σ_contributors V_h`, and inlet sediment timing taken from `S_h` (never reconstructed from event aggregates); (b) when **no** contributing element carries either hourly surface, the triangular hydrograph procedure of Eq. [13.4.1]-[13.4.2] applies to the **entire** contributor set for that inlet, and the combined peak must be the maximum discharge on the superimposed hydrograph; (c) partial, malformed, or mixed hourly authority is invalid: any contributor with only one hourly surface, a non-24-slot surface, a mixture of hourly and non-hourly contributor payloads at one inlet, or an hourly hillslope contributor feeding an inlet with dependency nodes that do not yet carry channel-hourly surfaces must fail closed rather than silently collapsing to the triangular daily-scalar branch. (d) Sediment **mass authority is per-contribution**: any contribution carrying the serialized `hourly_sediment_mass_kg` surface contributes `Σ S_h` as its sediment mass on BOTH authorized branches — never the `total_detachment − total_deposition` reconstruction. (e) **Labeled single-rate reduction (conditional scope limit, W11A):** when the channel-interval sediment lane (INV-ROUTE-015..INV-ROUTE-020) is **inactive**, the channel sediment solve remains quasi-steady per event; on branch (a) its sediment-rate time base must be the superposed `S_h` **active-hour span** (the serialized sediment timing), and the per-hour inlet sediment array must be carried on the routed-inlet state. When the channel-interval sediment lane is **active**, the per-interval sequencing authority of INV-ROUTE-015..INV-ROUTE-020 governs the channel sediment solve instead, consuming that carried per-hour inlet array as its interval-projection source. Reducing sediment timing to the water/event duration on an hourly-resolved inlet is invalid on both lanes. | hard-fail | REF-ROUTE-CH13-PEAKIN, REF-ROUTE-ADR0036-HOURLY | `[DIRECT][Static]` |
| INV-ROUTE-006 | Outlet-method branch invariant: channel outlet routing must execute exactly one branch selected by `ipeak` (`1` = modified Rational Eq. [13.4.3]-[13.4.24], `2` = CREAMS Eq. [13.4.25], `3` = linear kinematic-wave channel routing, `>=4` = Muskingum-Cunge channel routing); implicit fallback/mixing is invalid and selected-branch inputs/outputs must be finite and unit-consistent. | hard-fail | REF-ROUTE-CH13-RAT, REF-ROUTE-CH13-CREAMS, REF-ROUTE-WSHPEK-IPEAK, REF-ROUTE-WSHCHR-WAVE, REF-ROUTE-PHYS-BOUNDS | `[DIRECT][Static] + [INFERENCE][Static]` |
| INV-ROUTE-007 | Peak-duration closure invariant: for `ipeak <= 2`, if `roff <= 0.001 m^3`, peak runoff and runoff duration are both zero per §13.4.1; for `ipeak >= 3`, routed channel flow may still be evaluated from incoming hydrograph when local channel runoff is zero, but emitted outputs must obey non-negative finite closure (`roff = qpo * durrof` for `qpo > 0`, and `durrof = 0` when `qpo <= 1e-12`). | hard-fail | REF-ROUTE-CH13-DUR, REF-ROUTE-CH13-RAT, REF-ROUTE-CH13-CREAMS, REF-ROUTE-WSHDRV-ORDER, REF-ROUTE-WSHCHR-WAVE, REF-ROUTE-PHYS-BOUNDS | `[DIRECT][Static] + [INFERENCE][Static]` |
| INV-ROUTE-008 | Spatially-varied flow/shear invariant: channel erosion solver must use consistent spatially-varied flow outputs (`Sf`, `Sstar`, `leff`, `q`) to compute shear terms, and soil shear relation Eq. [13.5.13]-[13.5.16] must preserve finite physically valid domains. | hard-fail | REF-ROUTE-CH13-SVF, REF-ROUTE-CH13-SHEAR | `[DIRECT][Static] + [INFERENCE][Static]` |
| INV-ROUTE-009 | Sediment continuity invariant: quasi-steady sediment continuity Eq. [13.5.17]-[13.5.18] must be conserved across segments and particle classes with explicit inlet (`qsed_top`) and lateral (`qsed_lat`) source accounting. | hard-fail | REF-ROUTE-CH13-CONT | `[DIRECT][Static]` |
| INV-ROUTE-010 | Detachment/deposition branch invariant: detachment capacity Eq. [13.5.19]/[13.5.20], deposition Eq. [13.5.21]-[13.5.22], and transport-capacity branch iteration semantics from §13.5.6 must be explicit; silent branch collapse is invalid. | hard-fail | REF-ROUTE-CH13-DETDEP | `[DIRECT][Static] + [INFERENCE][Static]` |
| INV-ROUTE-011 | Coupling completeness invariant: required hillslope/impoundment/channel handoff payloads (runon volumes, durations, peak flow, and HBP sediment payload family `total_detachment_kg`, `total_deposition_kg`, `particle_class_count`, `sediment_concentration_kg_m3_i`, `particle_diameter_m_i`, `particle_flow_fraction_i`; plus, on minor-1 shards, the paired `hourly_runoff_volume_m3[24]` / `hourly_sediment_mass_kg[24]` surfaces with their `SC-INFILE-HBP-001` Section 8.5 integral-closure intake validation) must be present and parseable before routing calculations proceed. | hard-fail | REF-ROUTE-CH13-RUNON, REF-ROUTE-CH13-CONT, REF-ROUTE-CH4-COUPLING, REF-ROUTE-CH5-COUPLING, REF-ROUTE-HBP-FORMAT, REF-ROUTE-ADR0036-HOURLY | `[DIRECT][Static] + [INFERENCE][Static]` |
| INV-ROUTE-012 | Governance invariant: channel-routing outputs are watershed-integrated Tier-B surfaces; unresolved major discrepancies must route to investigation/disposition and cannot be silently promoted as Tier-A-equivalent confidence. | governance-fail | REF-ROUTE-CH13-RUNON, REF-ROUTE-CH13-DETDEP, REF-ROUTE-PHYS-BOUNDS | `[INFERENCE][Static]` |
| INV-ROUTE-013 | Applicability-bound invariant: authoritative scope is limited to small agricultural watersheds (Chapter-13 summary intent) with explicit exclusions (`no partial area response`, `no headcutting`, `no bank sloughing`, `no perennial streams`); watershed runfile intake must declare these selectors explicitly and fail closed when declarations are absent or violated. | hard-fail | REF-ROUTE-CH13-LIMIT, REF-ROUTE-RUNFILE-APPLICABILITY | `[DIRECT][Static] + [INFERENCE][Static]` |
| INV-ROUTE-014 | HPHYS0241 MOFE hourly carry routing-continuity invariant: watershed routing admission for multi-OFE hourly hillslope contributors must validate `mofe_hourly_carry` manifest provenance before consuming HBP pass payloads; aggregate-only carry metadata, inactive carry metadata, non-24-slot metadata, or malformed carry totals are coupling-incomplete and must hard-fail before channel routing. | hard-fail | REF-ROUTE-MOFE-HOURLY-CARRY, REF-ROUTE-CH13-RUNON, REF-ROUTE-HBP-FORMAT, REF-ROUTE-HBP-READER | `[DIRECT][Static] + [INFERENCE][Static]` |
| INV-ROUTE-015 | Channel sediment temporal-quantum and lane-activation invariant (W11A): **lane activation is biconditional and mandatory** — the interval lane is active for a channel if and only if (i) the channel executes a wave-routing branch (`ipeak >= 3`) producing the routed interval series `q1(it)` on the normalized `dtchr` grid, and (ii) the channel's inlet satisfies INV-ROUTE-005(a) authority (every hillslope contribution carries the paired hourly surfaces; every upstream channel dependency is itself interval-lane active per the (a) dependency-authority definition; any impoundment dependency precludes activation). When the predicate holds, executing the event-scalar sediment solve is invalid; when it does not hold, the INV-ROUTE-005(e) event-scalar lane governs. On the active lane the channel sediment solve executes once per `dtchr` interval on the routed water grid — the same normalized `ntchr`-interval grid the water routing produces (`q1(it)` basis). The hourly `V_h`/`S_h` HBP surfaces enter only as boundary conditions projected onto that grid by exact interval overlap (hour-uniform within each hour, per the ADR-0036 D2 hour-integrated definition). A sediment solve grid coarser than the water grid, an independent sediment grid, or direct consumption of hourly surfaces as the solve quantum is invalid. | hard-fail | REF-ROUTE-ARS77-SAMEGRID, REF-ROUTE-HECRAS-QUS, REF-ROUTE-ADR0036-HOURLY | `[DIRECT][Static] + [INFERENCE][Static]` |
| INV-ROUTE-016 | Per-interval quasi-steady solve-form invariant (W11A): each hydraulically active interval executes the complete Chapter-13 quasi-steady spatially-varied segment solve (the REF-ROUTE-CH13-SVF/EFFLEN/SHEAR/CONT/DETDEP machinery in the WSHEDIMPL18-41 migrated segment-solve lanes — the WS20/WS21 runtime families) at that interval's operands: segment discharges derived from the routed interval discharge and interval-projected lateral inflow; inlet sediment flux = interval-projected inlet class mass / interval duration (replacing the event-scalar `qsed_top = qsed_tot / durrof` reduction of Eq. [13.5.18]); upstream-channel sediment ingress = the upstream channel's same-interval egress on the shared grid. Published daily channel sediment surfaces are the interval sums. This quasi-steady-sequence form is a labeled refinement beyond legacy event-scalar source-intent (REF-ROUTE-CREAMS-CH3-QS documents the time-term deletion as a compute-cost reduction); the recorded fallback if per-interval quasi-steady proves untenable is the unsteady advection continuity form (REF-ROUTE-ARS77-SAMEGRID lineage) — never a return to the single event-peak solve. | hard-fail | REF-ROUTE-HECRAS-QUS, REF-ROUTE-CREAMS-CH3-QS, REF-ROUTE-CH13-CONT, REF-ROUTE-CH13-DETDEP, REF-ROUTE-CH14-TIMESTEP | `[DIRECT][Static] + [INFERENCE][Static]` |
| INV-ROUTE-017 | Channel geometry carry invariant (W11A): channel geometry state (depth-to-nonerodible-layer `depa/depb`, bottom widths `wida/widb`, eroded widths `wera/werb`) advances monotonically through the interval sequence in time order and carries across events, days, and the whole simulation. Interval, event, day, or calendar resets are invalid. The only authorized reseeds are run-start initialization (`chncon` lineage from `chnedm`/`chneds`/`chnwid` inputs) and the primary-tillage reseed for `ishape=3` channels (`wshdrv.for:1179-1189` lineage). Geometry is non-narrowing and non-refilling: eroded width never decreases and eroded depth never refills; deposition does not create a re-erodible bed store (GAP-ROUTE-012). | hard-fail | REF-ROUTE-CH13-GEOMCARRY, REF-ROUTE-GULLY-STATE, REF-ROUTE-HECRAS-QUS | `[DIRECT][Static] + [INFERENCE][Static]` |
| INV-ROUTE-018 | Widening-clock invariant (W11A): post-nonerodible-layer channel widening follows the **WEPP-adapted lineage realization** of the CREAMS widening law — the linear widening rate (the `dwdti = excess * 2 * Kch * (tau_b - taucr) / rho_soil` lineage form, with the CREAMS `^1.05` exponent dropped exactly as the lineage detachment path drops the [I-128] `1.35`/`^1.05` factors), the lineage-modified exponential (`wstar = (1 - exp(-1.0176*t_star))/1.0176`), and the fitted shear-distribution `f(x_b)` (`shdist` lineage) — as implemented in the WSHEDIMPL18-41 migrated segment-solve lanes; CREAMS Eq. [I-133]-[I-140] is cited as **structural provenance** for the law's form, not as a literal override of the lineage realization. Evaluation is per interval with carried state: each active interval computes its own final width `W_f(Q_interval)` and rate basis from that interval's hydraulics, advances the lineage exponential by the interval's widening-time budget with `W_i := W_current`, and holds geometry unchanged when `W_f(Q_interval) <= W_current` (the Chapter-13 "flow is too shallow to cause detachment" branch). Gate operands per lineage: detachment gates on average soil shear (`tau`, the `effsh` lineage); widening gates on boundary shear (`tau_b = tau * f(x_b)`). Within a layer-contact interval the erosion-time budget partitions per the lineage `timpot`/`timex` semantics: incision consumes `timpot = depmid * rho_soil / d_i` and only the residual budget drives widening. Per-interval re-anchoring is the interval-ization of the lineage's own per-event re-anchoring (carried eroded width plus each event's own discharge) — a **labeled refinement** with the same recorded-fallback posture as INV-ROUTE-016; no persistent widening state exists beyond the carried geometry. The event-scalar triangular shear-time surrogate (`tb = 2*rundur`, `timsh = tb*(1 - taucr/tau)` `dcap` lineage) is invalid on the interval lane: time-above-critical-shear is resolved directly by the interval series (an interval's erosion-time budget is `dtchr` when its gate shear exceeds `taucr`, else zero). | hard-fail | REF-ROUTE-CREAMS-CH3-WIDEN, REF-ROUTE-GULLY-STATE, REF-ROUTE-CH13-DETDEP | `[DIRECT][Static] + [INFERENCE][Static]` |
| INV-ROUTE-019 | Per-interval class mass-closure invariant (W11A): per particle class per interval, inlet ingress + lateral ingress + flow detachment = egress + deposition (TOL-ROUTE-006); per class per day, the interval sums equal the published daily class masses (TOL-ROUTE-007); interval projection is exact — the interval-projected inlet/lateral class masses sum to the source hourly masses (`Σ_intervals = Σ_h S_h` per contribution, TOL-ROUTE-008). No suspended sediment mass pool carries between intervals, events, or days: the quasi-steady interval solve carries no storage term, so each interval closes exactly and no sediment mass is attributable to end-of-grid routed water storage on this lane (INV-ROUTE-020(c)). Boundary-detached mass is **defined constructively** as eroded geometry volume * `rho_soil` (`d_ch = e_m / rho_soil`, Eq. [I-131] lineage; `rho_soil` = in-place soil weight density, baseline `wtdsoi` basis) — a derivation rule, not a separately checked residual. Per-class time resolution of ingress is the day-level class-fraction blend applied uniformly to projected interval masses; treating that uniform split as enriched timing is invalid (`SC-SED-001#GAP-SED-008` interchange scope — the serialized `S_h` is total-mass). | hard-fail | REF-ROUTE-CH13-CONT, REF-ROUTE-CREAMS-CH3-WIDEN, REF-ROUTE-ADR0036-HOURLY, REF-ROUTE-JIMF2023-CARRY | `[DIRECT][Static] + [INFERENCE][Static]` |
| INV-ROUTE-020 | Channel-interval degenerate-state invariant (W11A): (a) zero-flow interval — when the routed interval discharge is at or below the routed-closure constant of INV-ROUTE-007 (the `qpo` floor, `1e-12 m^3 s^-1`, applied here per interval), no detachment/transport solve executes and any interval-projected incoming sediment mass deposits in the reach (zero inlet transport capacity rule); geometry is unchanged; (b) the zero-flow floor is that existing routed-closure constant — introducing a separate sediment-specific flow threshold is invalid; (c) end-of-grid storage disposition — on the quasi-steady interval lane each interval closes without a suspended-storage term, so the sediment mass attributable to end-of-grid routed water storage is **zero by construction** and day closure is unaffected by nonzero water storage; should the recorded unsteady fallback lane (INV-ROUTE-016) ever activate, its grid-end suspended concentration state deposits in the reach at grid end (deposit-at-grid-end; GAP-ROUTE-013 records the labeled decision); carrying a suspended pool across midnight is invalid on both lanes; (d) cross-midnight — the `dtchr` grid covers exactly 86400 s (water-routing normalization lineage; a non-covering grid handed to the sediment lane is a typed hard failure) and the only cross-day channel sediment state is geometry (INV-ROUTE-017). | hard-fail | REF-ROUTE-ARS77-SAMEGRID, REF-ROUTE-JIMF2023-CARRY, REF-ROUTE-PHYS-BOUNDS | `[DIRECT][Static] + [INFERENCE][Static]` |

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
| `INV-ROUTE-013` | runtime + governance | Watershed runfile applicability validator (`inputs.applicability.*`) + promotion checklist | Typed hard error (`CLIWAT-E-040`) on missing/invalid applicability declarations; governance `HOLD` still required for any intentional out-of-scope workload claims | Governance + runtime gate | `[DIRECT][Static] + [INFERENCE][Static]` |
| `INV-ROUTE-014` | runtime + governance | Watershed contributor manifest validator before HBP routing dispatch | Typed hard error (`CLIWAT-E-037`) when multi-OFE hourly contributors lack active 24-slot MOFE carry-array provenance or publish malformed carry totals | HPHYS MOFE routing-continuity gate | `[DIRECT][Static] + [INFERENCE][Static]` |
| `INV-ROUTE-015` | runtime | Channel-interval sediment grid selector | Typed hard error (`WKERNEL-WS10-CHANNEL-E-003` family) on non-water-grid sediment quantum or unprojected hourly-surface consumption | Tier-B investigation gate | `[DIRECT][Static]` |
| `INV-ROUTE-016` | runtime | Per-interval segment-solve dispatcher (WSHEDIMPL18-41 lanes; `WKERNEL-WS10-CHANNEL-E-001..003` family) | Typed hard error on missing interval operands, event-scalar operand substitution, or skipped active-interval solves | Tier-B investigation gate | `[DIRECT][Static] + [INFERENCE][Static]` |
| `INV-ROUTE-017` | runtime | Channel geometry state carrier (`WKERNEL-WS10-CHANNEL-E-001..003` family) | Typed hard error on out-of-order interval application, unauthorized geometry reset, width decrease, or depth refill | Tier-B investigation gate | `[DIRECT][Static] + [INFERENCE][Static]` |
| `INV-ROUTE-018` | runtime | Widening-clock evaluator (`WKERNEL-WS10-CHANNEL-E-001..003` family) | Typed hard error on triangular shear-time surrogate use on the interval lane, widening-state advance under `W_f(Q) <= W_current`, or budget double-count at layer contact | Tier-B investigation gate | `[DIRECT][Static] + [INFERENCE][Static]` |
| `INV-ROUTE-019` | runtime | Per-interval class mass-closure checker (`WKERNEL-WS10-CHANNEL-E-001..003` family) | Typed hard error on interval/day closure residual beyond TOL-ROUTE-006/007, projection inexactness beyond TOL-ROUTE-008, or suspended-pool carry | Tier-B investigation gate | `[DIRECT][Static] + [INFERENCE][Static]` |
| `INV-ROUTE-020` | runtime | Channel-interval degenerate-state handler (`WKERNEL-WS10-CHANNEL-E-001..003` family) | Typed hard error on zero-flow-interval detachment, sediment-specific flow thresholds, cross-midnight suspended carry, or non-covering grids | Tier-B investigation gate | `[DIRECT][Static] + [INFERENCE][Static]` |

## Binding Exposure Index

| Entry ID | Source | Status | Binding classification | Canonical binding IDs | Gate | Notes |
|---|---|---|---|---|---|---|
| `BEI-ROUTE-001` | Chapter-13 runon, duration, transmission-loss, and runoff-case equations. | `active` | `maps-to-existing-INV` | `INV-ROUTE-001, INV-ROUTE-002, INV-ROUTE-003, INV-ROUTE-004` | `runtime-guard` | Existing WS10 runon/duration/loss guards carry these bindings. |
| `BEI-ROUTE-002` | Triangular and ADR-0036 hourly inlet superposition authority. | `active` | `maps-to-existing-INV` | `INV-ROUTE-005, INV-ROUTE-011` | `runtime-guard` | M-T3 tightens all-hourly/no-hourly authority without adding a new invariant family. |
| `BEI-ROUTE-003` | Outlet peak, duration, and WS11 wave-routing branch authority. | `active` | `maps-to-existing-INV` | `INV-ROUTE-006, INV-ROUTE-007` | `runtime-guard` | Branch selection and routed closure remain explicit WS10/WS11 guards. |
| `BEI-ROUTE-004` | Channel sediment continuity, shear, detachment, deposition, and transport capacity. | `active` | `maps-to-existing-INV` | `INV-ROUTE-008, INV-ROUTE-009, INV-ROUTE-010` | `runtime-guard` | Segment sediment process bindings are exposed through existing channel sediment guards. |
| `BEI-ROUTE-005` | Comparator confidence and Chapter-13 applicability declarations. | `active` | `maps-to-existing-INV` | `INV-ROUTE-012, INV-ROUTE-013` | `governance-runtime-guard` | Runtime admission and Tier-B promotion posture are both binding surfaces. |
| `BEI-ROUTE-006` | MOFE hourly carry routing-continuity metadata. | `active` | `maps-to-existing-INV` | `INV-ROUTE-014` | `runtime-guard` | Admission remains fail-closed before HBP routing dispatch. |
| `BEI-ROUTE-007` | W11A channel-interval sediment sequencing addendum (quantum, solve form, geometry carry, widening clock, closure, degenerate states). | `active` | `maps-to-existing-INV` | `INV-ROUTE-015, INV-ROUTE-016, INV-ROUTE-017, INV-ROUTE-018, INV-ROUTE-019, INV-ROUTE-020` | `runtime-guard` | Addendum narrative binds only through these invariant rows; INV-ROUTE-015 defines the biconditional activation predicate and INV-ROUTE-005(e) states which authority governs each side of it. |

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
| `mofe_hourly_carry` | `openwepp-hillslope-run-manifest-v1.mofe_hourly_carry` | multi-OFE hourly hillslope contributor routing-admission metadata | 24-slot carry-array provenance preserved | `[DIRECT][Static] + [INFERENCE][Static]` |
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
| Zero-flow channel interval (interval lane) | `q1(it) <= 1e-12 m^3 s^-1` with no detachment/transport solve; interval-projected incoming sediment deposits in the reach; geometry unchanged. | INV-ROUTE-020(a) zero-transport-capacity rule (REF-ROUTE-ARS77-SAMEGRID, extended by inference from the flowing-channel boundary condition to the dry interval). | `[DIRECT][Static] + [INFERENCE][Static]` |
| Non-widening active interval (interval lane) | `W_f(Q_interval) <= W_current` after nonerodible-layer contact: solve proceeds, geometry unchanged. | INV-ROUTE-018 widening-clock hold branch. | `[DIRECT][Static]` |
| End-of-grid residual channel storage (interval lane) | Nonzero routed water storage at grid end; sediment day closure unaffected (storage-associated suspended mass is zero by construction on the quasi-steady lane). | INV-ROUTE-020(c) (GAP-ROUTE-013 records the fallback-lane deposit-at-grid-end rule). | `[DIRECT][Static] + [INFERENCE][Static]` |

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
- Interval-lane channel sediment computed on a grid other than the routed water grid, or hourly `V_h`/`S_h` surfaces consumed as the solve quantum without interval projection. `[DIRECT][Static]`
- Channel geometry state reset at interval/event/day boundaries, applied out of time order, narrowed, or refilled outside the run-start and primary-tillage reseeds. `[DIRECT][Static]`
- Suspended channel sediment mass carried between intervals, events, or days; end-of-grid storage sediment left unclosed. `[DIRECT][Static] + [INFERENCE][Static]`
- Triangular shear-time surrogate (`tb = 2*rundur` lineage) used to derive erosion time on the interval-resolved lane. `[DIRECT][Static]`

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
| Applicability limits (`INV-ROUTE-013`) | watershed runfile applicability validation + scope review | Typed hard error on missing/violating applicability selectors; governance `HOLD` for intentional out-of-scope workloads without explicit risk acceptance | Governance + runtime gate | `[DIRECT][Static] + [INFERENCE][Static]` |

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
| TOL-ROUTE-006 | Per-interval per-class mass-closure residual (interval lane) | `<= 1e-9 kg` | INV-ROUTE-019 interval closure: ingress + detachment vs egress + deposition per class per `dtchr` interval. | `[INFERENCE][Static]` |
| TOL-ROUTE-007 | Daily class-mass sum residual (interval lane) | `<= 1e-9 kg` | Published daily class masses vs interval sums; structural f64 summation, tolerance covers rounding only. | `[INFERENCE][Static]` |
| TOL-ROUTE-008 | Hourly-to-interval projection exactness (interval lane) | `<= 1e-12` relative per contribution; contributions with `Σ_h S_h = 0` require exactly zero projected mass (absolute) | `Σ_intervals` of projected masses vs `Σ_h S_h`; exact interval-overlap projection, tolerance covers f64 rounding only. | `[INFERENCE][Static]` |

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
| Channel global routing controls | `dtchr`, `cbase`, `ipeak`; `nchnum` is a `chan.inp` channel-output selection count and may be zero |
| Channel per-node controls | `ws10_channel_{id}_chnn`, `ws10_channel_{id}_ctlslp`, `ws10_channel_{id}_chnk`, `ws10_channel_{id}_icntrl`, `ws10_channel_{id}_flgout`, `ws10_channel_{id}_rccoef`, `ws10_channel_{id}_rcexp`, `ws10_channel_{id}_rcoset` (`icntrl==4` lanes only) |
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
6. Preserve `chan.inp` output-gate semantics: `nchnum = 0` disables selected
   channel-detail output records but is not a channel-routing domain violation
   and must not be used as a positive routing contributor.

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
6. `ipeak >= 4` Muskingum-Cunge state publication preserves finite coefficient
   surfaces without non-physical coefficient clamping (`c1/c2/c3` are allowed
   to be signed), and branch output responds to prior wave-state memory
   (`ws10_channel_{id}_q1`, `ws10_channel_{id}_qin`) when provided.
7. `ipeak = 5` vectors execute variable-parameter Muskingum-Cunge
   dynamic-coefficient refresh semantics for the single-segment WS10 lane by
   recomputing `c0..c4` from dynamic reference-flow lineage each execution step
   (`qref = (qin + qin_previous + q1_previous) / 3` in the reduced lane),
   preserving routed closure and finite coefficient publication continuity.

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

> **DELETED — E.3 stage 2e (2026-07-04, SC-SED-001 rev 45).** The
> EROD14/Wave-2 runtime arm is removed from the codebase; the Wave-1
> chain (`SC-SED-001#INV-SED-016`) is the sole multi-OFE erosion engine
> and its intake carry the sole inter-OFE erosion coupling. This
> addendum is retained as the historical specification of the deleted
> arm; its obligations bind nothing at runtime. Manifest lineage:
> `erod14_wave2_enabled` publishes `false` permanently;
> `erod14_wave2_kernel_status_seen` is replaced by
> `multi_ofe_wave1_chained` (true only when the run is multi-OFE AND the
> Wave-1 seed enables — the no-tillage scope).

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
6. Zero-sediment contributor payloads are valid when the complete class-indexed
   payload is present, `max(total_detachment_kg - total_deposition_kg, 0)` is
   zero, and all class concentrations and particle-flow fractions are zero.
   Routing consumers must route those contributors as zero sediment load rather
   than requiring a positive class-fraction support. Positive net-sediment
   payloads still require positive particle-flow-fraction support and remain
   hard-fail states when the support is absent.

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

## WSHEDIMPL39 Chapter-13 Applicability Validator Addendum

1. Watershed runfile intake is the canonical runtime validator surface for
   Chapter-13 applicability declarations under this contract.
2. Runtime intake must require explicit selectors in
   `inputs.applicability`:
   - `chapter13_small_watershed_intent = true`,
   - `allow_partial_area_response = false`,
   - `allow_headcutting = false`,
   - `allow_bank_sloughing = false`,
   - `allow_perennial_streams = false`.
3. Missing selectors or disallowed selector values are typed hard-fail routing
   admission errors (`CLIWAT-E-040`); no silent defaults/coercion are allowed.
4. Applicability selector closure does not authorize out-of-scope workloads; it
   provides explicit runtime binding for declared constraints and retains
   governance risk-disposition requirements for exceptional use.

## WSHEDIMPL40 Muskingum-Cunge Baseline-Parity Addendum

1. WS11 Muskingum-Cunge (`ipeak >= 4`) runtime must ingest prior wave-state
   memory from published channel state symbols when present:
   - `ws10_channel_{id}_qin` -> prior inflow state,
   - `ws10_channel_{id}_q1` -> prior outflow state.
2. Initial-condition fallback for missing prior wave-state symbols must be
   explicit and deterministic; no implicit synthetic defaults are allowed.
3. WS11 Muskingum-Cunge lateral term publication must follow baseline lineage
   `c4 = 2 * qlat * dtchr * c0` for the current single-segment WS10 routing
   lane (`wshchr.for` `c4 = 2*qlavg*dxchr*dtchr*c0`, reduced with
   `dxchr == chnl1` and `qlavg = qlat/chnl1` in the single-segment reduction).
4. WS11 Muskingum-Cunge coefficient surfaces (`c1`, `c2`, `c3`) are finite
   real coefficients and must not be forced non-negative by publication-time
   clamps; invalidity is non-finite or undefined denominator, not sign alone.
5. WS11 Muskingum-Cunge publication still requires non-negative finite routed
   outflow (`q1`) and routed closure publication (`qpo`, `roff`, `durrof`)
   under `INV-ROUTE-006/007`.

## WSHEDIMPL41 MVPMC3 Dynamic-Coefficient Refresh Addendum

1. WS11 `ipeak = 5` branch must execute variable-parameter
   Muskingum-Cunge dynamic-coefficient refresh lineage from
   `wshchr.for` (`MVPMC3`) rather than reusing static `ipeak = 4`
   coefficients.
2. In the current single-segment WS10 routing lane, dynamic reference flow
   lineage is reduced from baseline segment-state terms as:
   - `qs(is-1,it)` -> `qin`,
   - `qs(is-1,it-1)` -> `qin_previous`,
   - `qs(is,it-1)` -> `q1_previous`,
   - `qref = (qin + qin_previous + q1_previous) / 3`.
3. Dynamic `ipeak = 5` coefficient refresh must recompute `c0..c4` from
   refreshed hydraulic terms each execution step and must not silently
   substitute static `ipeak = 4` coefficients when dynamic refresh inputs are
   valid.
4. Dynamic refresh validity is finite/domain bounded input closure; domain
   violations are typed hard-fail channel guard outcomes, not silent fallback.

## HPHYS0241 MOFE Hourly Carry Routing-Continuity Addendum

1. Watershed routing admission must treat multi-OFE hourly hillslope
   contributors as coupling-incomplete unless their declared hillslope manifest
   includes active `mofe_hourly_carry` provenance.
2. Required routing-admission metadata is:
   - `policy = baseline-wathour-24-slot-copy-forward`,
   - `substep_count = 24`,
   - required arrays exactly covering `ui_SUrunf`, `ui_SCrunf`,
     `ui_LfUrf`, and `ui_LfCrf`,
   - finite non-negative aggregate upstream/current carry totals.
3. This metadata gate is an admission guard before HBP pass payload consumption;
   it does not replace HBP field validation or channel runon/runoff equations.
4. Missing, inactive, malformed, non-24-slot, or aggregate-only carry metadata
   must hard-fail with watershed intake context. Channel routing may not infer
   array completeness from canonicalized WB13 row ids or daily aggregate
   `wb12_runoff_carryover` alone.

## W11A Channel-Interval Sediment Sequencing Addendum

Authority disposition of `20260710-wshedw11a-channel-hourly-sediment-authority-001`
(lifting `WSHED-W11-HOLD-001`). Binding residue is exposed through
`INV-ROUTE-015..020` (`BEI-ROUTE-007`); this addendum records the executable
sequencing for W11 implementation.

### Activation

Lane activation is the INV-ROUTE-015 biconditional, evaluated per channel:
the interval lane is active if and only if the channel runs a wave-routing
branch (`ipeak >= 3`) producing `q1(it)` on the normalized `dtchr` grid AND
the inlet satisfies INV-ROUTE-005(a) authority, where an upstream channel
dependency satisfies (a) exactly when it is itself interval-lane active
(publishing same-grid per-interval per-class egress — the INV-ROUTE-005(a)
dependency-authority definition). Activation is therefore evaluated in
topological order and propagates down an all-hourly network. When the
predicate holds, running the event-scalar sediment solve is invalid; when it
does not hold, the channel remains on the INV-ROUTE-005(e) event-scalar lane
or its existing fail-closed branches.

Inlets with impoundment dependency nodes do not activate: impoundments carry
no hourly or interval surface authority (impoundment sediment routing is out
of scope), so such inlets remain on the existing INV-ROUTE-005 branches.
An interval-lane-active channel feeding an inlet whose own predicate cannot
hold (for example, additional non-hourly contributors join downstream) is
the INV-ROUTE-005(c) mixed-authority state and fails closed.

On an activated channel, partial execution is invalid: consuming interval
water with event-scalar sediment operands, or event-scalar water with
interval sediment operands, on the same channel-day is a typed hard failure.
(Non-activated `ipeak >= 3` channels routing interval water with the
event-scalar sediment solve are the ordinary INV-ROUTE-005(e) lane, not
partial execution.)

### Interval Operand Assembly

| Operand | Basis |
|---|---|
| Interval discharge | routed `q1(it)` at the reach outlet on the shared grid; upstream dependency inflow already superposed per the wave-routing branch (`qin(it)`) |
| Interval inlet sediment mass per class | carried per-hour inlet sediment array (INV-ROUTE-005(e)) projected to the interval by exact interval overlap, split per class by the day-level class-fraction blend (`SC-SED-001#GAP-SED-008` interchange scope) |
| Interval lateral sediment mass per class | lateral contributor `S_h` surfaces projected and class-split identically |
| Interval inlet sediment flux | interval inlet class mass / `dtchr` (replaces event `qsed_top = qsed_tot / durrof`) |
| Upstream-channel sediment ingress | upstream channel's same-interval per-class egress on the shared grid |
| Erosion-time budget | `dtchr` when the interval's gate shear exceeds `taucr` (detachment gate: average soil shear `tau`; widening gate: boundary shear `tau_b = tau * f(x_b)`), else zero; partitioned per `timpot`/`timex` within a layer-contact interval (replaces the `tb = 2*rundur` triangular surrogate; INV-ROUTE-018) |

**Projection formula (INV-ROUTE-015):** for interval `i` and contribution
with hourly masses `S_h` and day-level class fractions `f_k`:
`mass_i(k) = f_k * Σ_h S_h * overlap(interval_i, hour_h) / 3600 s`, with the
grid anchored at 00:00 of the simulation day (interval `i` spans
`[(i-1)*dtchr, i*dtchr)`). The analogous formula projects `V_h` where the
water lane has not already projected it.

**Unit bridge (unit-governance declaration):** interval-lane external
operands are SI (`q1(it)` in `m^3 s^-1`, projected masses in `kg`, `dtchr`
in `s`); the migrated segment solve operates in the Chapter-13 English-unit
system (`ft^3 s^-1`, `lb ft^-1 s^-1`). The lane crosses SI to English at the
same named conversion boundary the WSHEDIMPL18-41 migrated lanes already
implement (baseline `chnrt` conversion lineage — the discharge and
volume conversion sites `chnrt.for:166`/`chnrt.for:846` preserved by
migration), and the TOL-ROUTE-006/007/008 closures are evaluated on the SI
(kg) side after the inverse conversion. No new conversion constants are
introduced by this lane.

### Sequencing Steps (per day, per channel, in interval order)

1. For each interval `it = 1..ntchr` in time order:
   a. If `q1(it) <= 1e-12 m^3 s^-1`: deposit all interval-projected incoming
      sediment in the reach (INV-ROUTE-020(a)); geometry unchanged; publish
      zero egress for the interval; continue.
   b. Otherwise run the Chapter-13 quasi-steady segment solve (effective
      length, spatially-varied flow, shear partition, transport capacity,
      the §13.5.6 detachment-deposition Cases I-IV — the case12/case34
      branch families of the WSHEDIMPL18-41 migrated lanes; distinct from
      the §13.2 runoff Cases I-IV of INV-ROUTE-003) at the interval
      operands, against the CURRENT carried geometry.
   c. Apply geometry mutations produced by the solve (incision toward the
      nonerodible layer; post-contact widening per the INV-ROUTE-018
      widening clock) to the carried state before the next interval.
   d. Close the interval per class (INV-ROUTE-019, TOL-ROUTE-006) and hand
      the interval egress to downstream dependents on the shared grid.
2. At grid end: close the day per class (TOL-ROUTE-007) and against the
   projection sources (TOL-ROUTE-008); storage-associated suspended mass is
   zero by construction on this lane (INV-ROUTE-020(c)), so nonzero residual
   water storage does not enter the sediment closure.
3. Publish daily channel sediment surfaces as the interval sums; geometry
   carries to the next day unmodified (INV-ROUTE-017).

### Widening Clock (per INV-ROUTE-018)

Realization: the **WEPP-adapted lineage forms** govern (linear rate,
`1.0176`-modified exponential, fitted `f(x_b)` — the WSHEDIMPL18-41 lane
implementations); the CREAMS equation numbers below cite structural
provenance, not literal forms. For a reach section with carried geometry:

1. If the section has not reached the nonerodible layer: incision proceeds
   at the lineage rate (`d_ch` from [I-131] structure) over the interval's
   erosion-time budget. If the layer is reached mid-interval, the budget
   partitions per the lineage `timpot`/`timex` semantics: incision consumes
   `timpot = depmid * rho_soil / d_i`; only `timex = budget - timpot`
   drives widening in that interval.
2. At/after layer contact: compute `W_f(Q_interval)` ([I-139]-[I-140]
   structure) and the lineage rate basis ([I-136]-[I-138] structure) at the
   interval hydraulics; if `W_f(Q_interval) > W_current` and the widening
   gate shear (`tau_b`) exceeds `taucr`, advance the lineage exponential
   with the interval's widening-time budget from `W_i := W_current`.
3. Else: hold geometry (the erosion-rate-zero branch).
4. Width advances monotonically; state carries per INV-ROUTE-017; no
   persistent widening state exists beyond the carried geometry.

**Known-divergence note (comparator posture):** the interval lane replaces
{event-peak shear held over the reduced `timsh` window} with {interval
shear over the interval budget}; above-critical excursions shorter than the
hourly serialization quantum are truncated by the hour-mean water authority.
Comparator deltas on widening/detachment magnitude against the event-scalar
arm or legacy are Investigation-tier flags (ADR-0017, ADR-0036 D5), never
acceptance gates.

### Contract-Derived Test Vector Obligations (W11 implementation gates)

1. **Single-interval equivalence**: a day whose routed series has exactly one
   active interval (discharge `Q`, duration `dtchr`) reproduces the shared
   segment-solve core invoked once with pinned operands `durrof := dtchr`,
   `qsed_top := interval class mass / dtchr`, and erosion-time budget
   `:= dtchr` (i.e., the event-scalar path minus the triangular surrogate
   and minus interval sequencing) — per-class egress equal within
   TOL-ROUTE-006. This vector verifies sequencing/wiring identity, not
   operand-law differences (the surrogate retirement is a deliberate
   operand change covered by the known-divergence note).
2. **Interval-sum closure**: a multi-interval day closes per class per
   interval (TOL-ROUTE-006), per day (TOL-ROUTE-007), and against the
   projected `Σ_h S_h` sources (TOL-ROUTE-008).
3. **Geometry carry**: two consecutive active intervals with decreasing
   discharge — the second consumes the first's mutated geometry; width is
   non-decreasing; no reset.
4. **Widening clock**: an interval with `W_f(Q) > W_current` widens per the
   INV-ROUTE-018 lineage realization ([I-133]-[I-136] structure); a
   following smaller-flow interval with `W_f(Q) <= W_current` leaves width
   unchanged.
5. **Zero-flow deposition**: an interval at the `1e-12 m^3 s^-1` floor with
   positive projected inlet mass deposits the full mass, executes no
   detachment, and leaves geometry unchanged.
6. **End-of-grid storage**: a day ending with nonzero routed water storage
   closes per TOL-ROUTE-007 with zero storage-attributed sediment mass
   (INV-ROUTE-020(c) zero-by-construction), and no suspended surface
   crosses the day boundary.
7. **Cross-day carry**: geometry mutated on day N is the day N+1 starting
   state; no sediment mass surface carries across the day boundary.
8. **Tillage reseed**: primary tillage on an `ishape=3` channel reseeds
   geometry to input values; all other days never reseed.
9. **Fail-closed vectors**: non-water-grid sediment quantum, event-scalar
   operand substitution on an active interval lane, unauthorized geometry
   reset, suspended-pool carry, and triangular-surrogate erosion time each
   fail with the `WKERNEL-WS10-CHANNEL-E-001..003` guard family.
10. **Mid-interval layer contact**: an interval in which incision reaches
    the nonerodible layer partway through partitions its erosion-time
    budget per `timpot`/`timex` (no double-count: incision through the
    residual `depmid` plus widening only over `timex`), and the contact
    interval's detached mass matches the constructive geometry derivation
    of INV-ROUTE-019.

## Gap Register

| Gap ID | Statement | Impact | Promotability | Evidence |
|---|---|---|---|---|
| GAP-ROUTE-001 | Per-invariant comparator vectors for watershed/channel Tier-B invariant families remain uncurated, and this residual automation limitation is explicitly risk-accepted for current governance progression. | Automated per-invariant acceptance remains limited; manual comparator interpretation is required where vectors are absent. | closed | `[DIRECT][Static]` |
| GAP-ROUTE-002 | Wave-0 erosion-lane alias-ownership ambiguity for required routing boundary symbols is explicitly dispositioned by canonical EROD11 alias ownership registers. | Alias-ownership ambiguity closure is complete for required boundary symbols; production erosion physics remains separately `HOLD`-gated by non-promotable companion/process gaps. | closed | `[DIRECT][Static] + [Ran]` |
| GAP-ROUTE-003 | EROD12 ratifies cross-domain ownership and guard semantics for required erosion-lane routing boundaries across `SC-HYDRAULICS-001`, `SC-SED-001`, `SC-ROUTE-001`, and `SC-SYSTEM-001`; downstream WS10/impoundment ownership paths remain explicitly guarded by their companion contracts. | Required Wave-0 ownership ambiguity is closed for routing-coupled erosion boundaries; non-Wave-0 scope/applicability holds remain governed by other gap rows. | closed | `[DIRECT][Static] + [Ran]` |
| GAP-ROUTE-004 | Chapter-13 mixed-unit and regression-derived formulation caveats remain and are explicitly retained as documented limitations with governance risk acceptance. | Unit-conversion and regression-lineage interpretation risk remains and requires explicit review in sensitive analyses; this is accepted as a model-governance limitation. | closed | `[DIRECT][Static] + [INFERENCE][Static]` |
| GAP-ROUTE-005 | WSHEDIMPL39 bound Chapter-13 applicability limits to concrete watershed runfile validator selectors (`inputs.applicability.*`) with typed fail-closed intake errors (`CLIWAT-E-040`) for missing/invalid declarations. | Runtime applicability admission is now explicit and fail-closed; intentional out-of-scope workload claims still require governance risk disposition. | closed | `[DIRECT][Static] + [Ran]` |
| GAP-ROUTE-006 | WS11 wave-routing branch authority is anchored to pinned legacy static-code provenance (`wshcqi`, `wshdrv`, `wshpek`, `wshchr`) pending companion documentation that cross-indexes non-chapter method-lineage references in one canonical note. | Migration authority is executable and explicit, but review burden for non-chapter lineage remains elevated until companion documentation lands. | promotable-with-risk | `[DIRECT][Static] + [INFERENCE][Static]` |
| GAP-ROUTE-007 | Legacy provenance confusion between watershed routing and hillslope `CONTIN -> ROUTE` branch logic required explicit scope partitioning; EROD16 closes the documentation ambiguity but downstream hillslope runtime parity remains governed by `SC-SED-001` queue stages. | Prevents false attribution of hillslope branch parity status to WS10 routing closure decisions. | closed | `[DIRECT][Static] + [INFERENCE][Static]` |
| GAP-ROUTE-008 | WSHEDIMPL37 migrated baseline-authoritative WS11 runon/runoff routine-chain behavior (`wshcqi/wshirs/wshrun`) into production WS10 routing lanes, including runon-volume partition publication (`rvolat`, `rvotop`, `rvolon`), duration-max continuity (`durlat`, `durtop`, `durrunon`, `durchan`, `watdur`), runoff-case publication (`ws11_runoff_case`, `ws11_qci`, `ws11_qcf`, `ws11_runvol`, `tl`, `rofc`), and explicit `ipeak` threshold/wave-routing continuity vectors. Combined with WSHED05 `ipeak > 2` wave-state publication closure (`q1/qin/qlat/c0..c4`), WS11 route-chain parity for this gap scope is complete. | WS11 hydrology routine-chain parity closure is now explicit for watershed route-chain scope; residual routing HOLD posture is governed by remaining channel sediment process-parity blocker `GAP-ROUTE-009`. | closed | `[DIRECT][Static] + [Ran]` |
| GAP-ROUTE-009 | WSHEDIMPL38 closed the residual watershed channel sediment parity seam by retiring unresolved fallback diagnostics (`ws20_detachment_unmigrated_segment_count`, `ws21_detach_unmigrated_segment_count`) and converting residual invalid-segment fallback branches in WS20/WS21 routing to typed fail-closed domain guards (`ws20_case12_next_flux_{class:04}`, `ws21_case3_next_flux_{class:04}`, `ws21_case4_next_flux_{class:04}`), while preserving baseline-authoritative `chnero/chnrt/detach` execution lineage and migrated width/shape/transition semantics from WSHEDIMPL20-37. | Watershed channel sediment routing now executes without unresolved-detachment surrogate counters; residual numeric/domain violations surface as explicit typed guard failures instead of fallback continuation. | closed | `[DIRECT][Static] + [Ran]` |
| GAP-ROUTE-010 | WSHEDIMPL40 identified residual WS11 Muskingum-Cunge drift versus pinned baseline in prior-state memory ingestion and coefficient publication semantics (`c4` lateral term scaling and sign-permissive coefficient handling). | Without this closure, successive-event MC routing could ignore prior routed state and incorrectly force coefficient sign, reducing branch-equivalence confidence for `ipeak >= 4` vectors. | closed | `[DIRECT][Static] + [Ran]` |
| GAP-ROUTE-011 | WSHEDIMPL41 migrated WS11 `ipeak = 5` variable-parameter Muskingum-Cunge dynamic-coefficient refresh behavior into the current single-segment WS10 runtime lane by executing dynamic reference-flow lineage and per-step coefficient refresh semantics (`c0..c4`) under typed fail-closed guards. | `ipeak = 5` branch behavior no longer reuses static `ipeak = 4` coefficients when dynamic refresh inputs are valid; dynamic-coefficient parity closure is explicit for the current WS10 lane. | closed | `[DIRECT][Static] + [Ran]` |
| GAP-ROUTE-012 | No re-erodible deposited-bed store (W11A recorded limitation): interval-lane deposition leaves the active sediment accounting permanently, matching the WEPP/CREAMS lineage in which detachment draws only on channel-boundary soil down to the nonerodible layer. External unsteady models (KINEROS2, HEC-RAS) carry exchangeable bed layers; importing one would be new physics with no WEPP-lineage support and no acceptance driver at this tier. Retained deliberately with governance risk acceptance. | Deposited channel sediment cannot later re-entrain; long-duration aggradation/degradation cycling is out of authoritative scope (consistent with INV-ROUTE-013 applicability bounds). | closed | `[DIRECT][Static] + [INFERENCE][Static]` |
| GAP-ROUTE-013 | End-of-grid storage sediment disposition (W11A labeled decision): on the ratified quasi-steady interval lane the question is **moot by construction** — each interval solve closes without a suspended-storage term, so no sediment mass is attributable to end-of-grid routed water storage (INV-ROUTE-020(c)). The decision recorded here governs the recorded unsteady fallback lane only, where suspended concentration state exists: external precedent diverges (HEC-RAS carries suspended state continuously; the lineage record — event-closed solves and the 2023 upstream fix treating cross-storm sediment-mass carry as a defect, REF-ROUTE-JIMF2023-CARRY — closes it), and the ratified fallback-lane default is deposit-at-grid-end. Revisit only with an explicit acceptance driver for suspended carry (e.g. perennial-stream scope change, outside INV-ROUTE-013 bounds today). | Days always mass-close on both lanes; on the fallback lane, suspended sediment in end-of-day residual storage is recorded as reach deposition rather than carried concentration state. | closed | `[DIRECT][Static] + [INFERENCE][Static]` |

## Revision History

| Date UTC | Version | Author | Change |
|---|---|---|---|
| `2026-07-10` | `51` | `Claude Code` | WSHED-W11A channel-interval sediment sequencing authority (lifts `WSHED-W11-HOLD-001`): added anchors `REF-ROUTE-CH13-GEOMCARRY`, `REF-ROUTE-CREAMS-CH3-QS/WIDEN` (vendored CREAMS Ch. 3, widening law [I-133]-[I-140] verified against the rendered scan), `REF-ROUTE-ARS77-SAMEGRID`, `REF-ROUTE-HECRAS-QUS` (class-corroboration grade), `REF-ROUTE-CH14-TIMESTEP`, `REF-ROUTE-GULLY-STATE`, `REF-ROUTE-JIMF2023-CARRY`; added `INV-ROUTE-015..020` (biconditional mandatory lane activation + dtchr-grid sediment quantum; per-interval quasi-steady sequence on the WSHEDIMPL18-41 lanes with recorded unsteady fallback; monotonic geometry carry with tillage-only reseed; the widening clock on the **WEPP-adapted lineage realization** — linear rate, 1.0176-modified exponential, fitted `f(x_b)`, `timpot/timex` budget partition — with CREAMS as structural provenance and the per-interval re-anchoring labeled as a refinement; per-interval/day class mass closure with projection exactness and the constructive geometry-mass derivation; degenerate states incl. zero-flow deposition and the zero-by-construction storage disposition with the fallback-lane deposit-at-grid-end rule); extended `INV-ROUTE-005(a)` with the interval-lane dependency-authority definition and made `(e)` conditional on lane activation; added `BEI-ROUTE-007`, `TOL-ROUTE-006..008`, new Variables-and-Units rows with the SI/English unit-bridge declaration, the W11A sequencing addendum (activation topology incl. the impoundment exclusion, projection formula, comparator known-divergence note) with ten contract-derived test-vector obligations, and `GAP-ROUTE-012/013`. Dual review (W11A package review_agent_a/b, both GO-WITH-AMENDMENTS) dispositioned within this cycle: the widening-law realization adjudication (A-1), layer-contact budget partition (A-2), storage-closure reconciliation (A-3), activation biconditional and network dependency-authority (B-1/B-2), and unit-bridge/symbol-table completion (B-4) are incorporated above. |
| `2026-07-09` | `50` | `Codex` | M-T3 profile-only closure: added a Binding Exposure Index mapping active route/channel authority and obligations to existing `INV-ROUTE-*` / `OBL-ROUTE-*` bindings; no process-physics authority changed. |
| `2026-07-09` | `49` | `Codex` | M-T3 hourly watershed consumer amendment: tightened `INV-ROUTE-005` to an all-hourly or no-hourly inlet rule. Complete minor-1 contributor sets use the serialized `V_h`/`S_h` time base; all contributors without hourly authority retain the triangular fallback; partial, malformed, mixed hourly/non-hourly contributors or hourly contributors with dependency nodes lacking channel-hourly surfaces fail closed. |
| `2026-07-04` | `48` | `Claude Code` | E.3 stage 2e disposition: the EROD14 Wave-2 addendum is marked DELETED-historical (the runtime arm is removed; the Wave-1 chain `SC-SED-001#INV-SED-016` is the sole multi-OFE erosion engine); manifest lineage noted (`erod14_wave2_enabled` permanently false; kernel-status field replaced by `multi_ofe_wave1_chained`, true only on no-tillage multi-OFE runs). |
| `2026-07-04` | `47` | `Claude Code` | E.2 Codex round-1 High: `INV-ROUTE-005` extended with (c) per-contribution `Σ S_h` sediment-mass authority on both branches and (d) the labeled single-rate reduction — the quasi-steady channel sediment-rate time base on hourly-resolved inlets is the superposed `S_h` active-hour span, with the per-hour inlet array carried for the future channel-hourly extension. |
| `2026-07-04` | `46` | `Claude Code` | E.2/ADR-0036 amendment: `INV-ROUTE-005` made conditional (hour-resolved inlet superposition on the paired minor-1 `V_h`/`S_h` surfaces when all contributors carry them; Eq. [13.4.1]-[13.4.2] triangular procedure as the whole-inlet fallback, no mixed-basis superposition), added `REF-ROUTE-ADR0036-HOURLY`, and extended `INV-ROUTE-011` coupling completeness with the minor-1 hourly pair + intake integral-closure requirement. |
| `2026-06-14` | `45` | `Codex` | WSHED01 W-C amendment: clarified EROD15 routing-intake semantics for complete zero-sediment contributor payloads and pinned `chan.inp` `nchnum=0` as valid output-disabled routing input rather than a positive channel-routing operand. |
| `2026-06-01` | `44` | `Codex` | HPHYS0241 amendment: added `INV-ROUTE-014` and routing-admission authority requiring active 24-slot MOFE hourly carry-array manifest provenance for multi-OFE hourly hillslope contributors before watershed HBP routing dispatch. |
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
| `2026-05-28` | `35` | `Codex` | WSHEDIMPL33 amendment: reconciled parser/runtime channel `ienslp` lineage by aligning watershed channel parser projection and WS10 runtime seed validation on explicit `ienslp` domain semantics (`1..=2`, fail-closed out-of-domain), while preserving non-promotable `GAP-ROUTE-009` posture for remaining full `chnero/chnrt` parity closure families. |
| `2026-05-28` | `36` | `Codex` | WSHEDIMPL34 amendment: reconciled parser/runtime watershed-channel Manning relation lineage by aligning parser projection authority and WS10 runtime seed validation on explicit `chnn >= chnnbr` fail-closed semantics, while preserving non-promotable `GAP-ROUTE-009` posture for remaining full `chnero/chnrt` parity closure families. |
| `2026-05-28` | `37` | `Codex` | WSHEDIMPL35 amendment: reconciled parser/runtime channel control lineage by projecting `icntrl`/`flgout` into WS10 runtime seed surfaces with explicit fail-closed domain semantics (`icntrl in [0,4]`, `flgout in [0,1]`), while preserving non-promotable `GAP-ROUTE-009` posture for remaining full `chnero/chnrt` parity closure families. |
| `2026-05-28` | `38` | `Codex` | WSHEDIMPL36 amendment: reconciled parser/runtime rating-curve control lineage by projecting `ws10_channel_{id}_{rccoef,rcexp,rcoset}` for `icntrl==4` lanes into WS10 runtime seed surfaces with explicit fail-closed payload-shape/domain semantics (`rccoef>0`, `rcexp>0`, `rcoset>=0`), while preserving non-promotable `GAP-ROUTE-009` posture for remaining full `chnero/chnrt` parity closure families. |
| `2026-05-28` | `39` | `Codex` | WSHEDIMPL37 amendment: migrated baseline-authoritative WS11 runon/runoff route-chain behavior (`wshcqi/wshirs/wshrun`) into WS10 production runtime lanes with explicit runon partition, duration-max, runoff-case, and `ipeak` threshold/wave-routing continuity publication, dispositioning `GAP-ROUTE-008` to `closed` while preserving non-promotable `GAP-ROUTE-009` posture for remaining full `chnero/chnrt` parity closure families. |
| `2026-05-28` | `40` | `Codex` | WSHEDIMPL38 amendment: closed `GAP-ROUTE-009` by retiring unresolved-detachment diagnostics symbols and replacing residual WS20/WS21 invalid-segment fallback continuation with typed fail-closed domain guards (`ws20_case12_next_flux_{class:04}`, `ws21_case3_next_flux_{class:04}`, `ws21_case4_next_flux_{class:04}`) under canonical `chnero/chnrt/detach` migration authority. |
| `2026-05-28` | `41` | `Codex` | WSHEDIMPL39 amendment: bound Chapter-13 applicability limits to concrete watershed runfile selectors (`inputs.applicability.*`) with typed fail-closed intake error `CLIWAT-E-040`, added canonical runtime-validator authority anchor, and dispositioned `GAP-ROUTE-005` to `closed`. |
| `2026-05-28` | `42` | `Codex` | WSHEDIMPL40 amendment: ratified WS11 Muskingum-Cunge parity closure for prior wave-state memory ingestion (`ws10_channel_{id}_{qin,q1}`), single-segment baseline-lineage lateral term scaling (`c4 = 2*qlat*dtchr*c0`), and finite signed MC coefficient publication semantics (`c1/c2/c3`) without non-physical non-negative clamps (`GAP-ROUTE-010` closed); retained follow-on `ipeak=5` variable-parameter dynamic-coefficient parity gap (`GAP-ROUTE-011`) as promotable-with-risk. |
| `2026-05-28` | `43` | `Codex` | WSHEDIMPL41 amendment: migrated WS11 `ipeak=5` MVPMC3 dynamic-coefficient refresh lineage into the current single-segment WS10 routing lane by deriving dynamic reference-flow terms from reduced segment-state aliases and recomputing `c0..c4` per execution step under typed fail-closed guards, dispositioning `GAP-ROUTE-011` to `closed`. |
